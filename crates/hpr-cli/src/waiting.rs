//! What a fetch says while it waits on the network: the product system's `cli.md` (*Output*) asks
//! a command to print something within 100 ms and to show progress on standard error for anything
//! longer, hidden when standard error isn't a terminal (#411).
//!
//! [`Waiting`] wraps the HTTP transport. A fetch still going after [`Wait::after`] draws one line
//! over itself on standard error, saying which host it waits on, the bytes received so far and
//! the seconds gone, and takes it off the screen when the answer comes. [`crate::run_with`]
//! installs a [`Wait`] only when [`Console::progress`] says so: standard error a terminal that
//! redraws a line, and text output, so never on a pipe or with `--json`. A fetch answered from
//! the cache never reaches the transport and draws nothing.

use std::cell::RefCell;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use hpr::hpr_net::{Http, Transport};

use crate::console::Console;

/// How often the waiting line is drawn again.
const TICK: Duration = Duration::from_millis(100);

/// Where and after how long a fetch draws its waiting line.
pub(crate) struct Wait {
    /// How long a fetch goes before its line is drawn.
    pub(crate) after: Duration,
    /// Standard error.
    pub(crate) to: Box<dyn Write>,
}

impl Wait {
    /// The wait for a command run on `console`, writing to `to` (standard error): `None` when
    /// [`Console::progress`] says a run draws no progress, as on a pipe or with `--json`.
    pub(crate) fn on(console: Console, json: bool, to: Box<dyn Write>) -> Option<Self> {
        console.progress(json).map(|after| Self { after, to })
    }
}

thread_local! {
    /// The wait [`with_wait`] installed for this thread's command, if any.
    static WAIT: RefCell<Option<Wait>> = const { RefCell::new(None) };
}

/// Runs `f` with `wait` installed for every fetch on this thread; the wait before is put back
/// after, whether `f` returns or panics.
pub(crate) fn with_wait<R>(wait: Option<Wait>, f: impl FnOnce() -> R) -> R {
    /// Puts the wait before back when dropped.
    struct Restore(Option<Wait>);
    impl Drop for Restore {
        fn drop(&mut self) {
            let before = self.0.take();
            WAIT.with(|slot| *slot.borrow_mut() = before);
        }
    }
    let _restore = Restore(WAIT.with(|slot| slot.replace(wait)));
    f()
}

/// A transport that says what it waits for when a fetch goes on past the installed [`Wait`].
#[derive(Debug, Clone)]
pub(crate) struct Waiting<T> {
    /// The transport that fetches.
    inner: T,
    /// The body bytes `inner`'s fetch has read so far, when it counts them.
    received: Option<Arc<AtomicU64>>,
}

impl Waiting<Http> {
    /// `http`, showing the bytes it has received ([`Http::received`]).
    pub(crate) fn http(http: Http) -> Self {
        let received = Some(http.received());
        Self {
            inner: http,
            received,
        }
    }
}

impl<T: Transport + Sync> Transport for Waiting<T> {
    fn get(&self, url: &str) -> Result<Vec<u8>, String> {
        WAIT.with(|slot| match slot.borrow_mut().as_mut() {
            Some(wait) => waited(
                || self.inner.get(url),
                &host(url),
                self.received.as_deref(),
                wait,
            ),
            None => self.inner.get(url),
        })
    }
}

/// Runs `fetch` on a thread of its own and, once it has gone on for `wait.after`, draws on
/// `wait.to` what it waits for, again each [`TICK`] until it ends; then takes the line off the
/// screen. A failure to write standard error leaves the fetch to finish quietly.
fn waited<R: Send>(
    fetch: impl FnOnce() -> R + Send,
    host: &str,
    received: Option<&AtomicU64>,
    wait: &mut Wait,
) -> R {
    let started = Instant::now();
    std::thread::scope(|scope| {
        let (done, finished) = mpsc::channel();
        let worker = scope.spawn(move || {
            let answer = fetch();
            // The receiver waits until this is sent; it can't be gone.
            let _ = done.send(());
            answer
        });
        let mut shown: usize = 0;
        loop {
            let elapsed = started.elapsed();
            if elapsed >= wait.after {
                let line = line(
                    host,
                    received.map(|count| count.load(Ordering::Relaxed)),
                    elapsed,
                );
                let length = line.chars().count();
                let rest = " ".repeat(shown.saturating_sub(length));
                if write!(wait.to, "\r{line}{rest}")
                    .and_then(|()| wait.to.flush())
                    .is_ok()
                {
                    shown = length;
                }
            }
            match finished.recv_timeout(TICK) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
        if shown > 0 {
            let blank = " ".repeat(shown);
            let _ = write!(wait.to, "\r{blank}\r").and_then(|()| wait.to.flush());
        }
        // A panic on the fetch's thread is passed on, as it would be without the wait.
        worker
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    })
}

/// The waiting line: the host, the bytes received when they are counted and some have come, and
/// the whole seconds gone.
fn line(host: &str, received: Option<u64>, elapsed: Duration) -> String {
    let seconds = elapsed.as_secs();
    match received {
        Some(bytes) if bytes > 0 => {
            format!("fetching from {host}: {} so far, {seconds} s", size(bytes))
        }
        _ => format!("waiting for {host}, {seconds} s"),
    }
}

/// `bytes` in decimal units, as a download is usually counted: `512 bytes`, `48.2 kB`, `3.1 MB`.
fn size(bytes: u64) -> String {
    match bytes {
        0..1_000 => format!("{bytes} bytes"),
        1_000..1_000_000 => format!("{:.1} kB", bytes as f64 / 1e3),
        _ => format!("{:.1} MB", bytes as f64 / 1e6),
    }
}

/// The host `url` names, without its scheme, any user name and password, path or query, and with
/// each control character shown as `?`; the URL as it is when it names none.
fn host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or(rest);
    let host = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    crate::printable(if host.is_empty() { url } else { host })
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    /// A standard error the test reads back.
    #[derive(Clone, Default)]
    struct Shared(Rc<RefCell<Vec<u8>>>);

    impl Write for Shared {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Shared {
        fn text(&self) -> String {
            String::from_utf8(self.0.borrow().clone()).unwrap()
        }
    }

    /// A transport that answers after `delay`, having "received" `bytes`.
    struct Slow {
        delay: Duration,
        bytes: u64,
        received: Arc<AtomicU64>,
    }

    impl Transport for Slow {
        fn get(&self, _url: &str) -> Result<Vec<u8>, String> {
            self.received.store(self.bytes, Ordering::Relaxed);
            std::thread::sleep(self.delay);
            Ok(vec![1, 2, 3])
        }
    }

    fn slow(delay_ms: u64, bytes: u64) -> Waiting<Slow> {
        let received = Arc::new(AtomicU64::new(0));
        Waiting {
            inner: Slow {
                delay: Duration::from_millis(delay_ms),
                bytes,
                received: Arc::clone(&received),
            },
            received: Some(received),
        }
    }

    const URL: &str = "https://user:secret@nomads.ncep.noaa.gov:443/cgi-bin/filter_gfs_0p25.pl?x=1";

    fn terminal() -> Console {
        Console {
            stderr_terminal: true,
            ..Console::PIPED
        }
    }

    /// A fetch past 100 ms names the host and the bytes so far, then clears its line; the answer
    /// is the transport's.
    #[test]
    fn a_slow_fetch_says_what_it_waits_for() {
        let err = Shared::default();
        let wait = Wait::on(terminal(), false, Box::new(err.clone()));
        let answer = with_wait(wait, || slow(450, 48_213).get(URL));
        assert_eq!(answer, Ok(vec![1, 2, 3]));
        let text = err.text();
        assert!(
            text.starts_with("\rfetching from nomads.ncep.noaa.gov:443: 48.2 kB so far, "),
            "{text:?}"
        );
        assert!(!text.contains("secret"), "{text:?}");
        let last = text.rsplit('\r').nth(1).unwrap();
        assert!(last.trim().is_empty(), "the line is cleared: {text:?}");
        assert!(text.ends_with('\r'), "{text:?}");
    }

    /// A fetch answered before its wait is up draws nothing. The wait is longer than a
    /// terminal's 100 ms, so a slow machine starting the thread can't draw it.
    #[test]
    fn a_quick_fetch_says_nothing() {
        let err = Shared::default();
        let wait = Wait {
            after: Duration::from_secs(5),
            to: Box::new(err.clone()),
        };
        assert_eq!(
            with_wait(Some(wait), || slow(0, 10).get(URL)),
            Ok(vec![1, 2, 3])
        );
        assert_eq!(err.text(), "");
    }

    /// Nothing is installed for standard error on a pipe, with `--json`, or on a `dumb`
    /// terminal, so even a slow fetch draws nothing; and the wait before is put back after.
    #[test]
    fn nothing_is_drawn_on_a_pipe_or_with_json() {
        let dumb = Console {
            dumb_terminal: true,
            ..terminal()
        };
        for (console, json) in [(Console::PIPED, false), (terminal(), true), (dumb, false)] {
            let err = Shared::default();
            let wait = Wait::on(console, json, Box::new(err.clone()));
            assert!(wait.is_none(), "{console:?}, json {json}");
            assert_eq!(
                with_wait(wait, || slow(250, 10).get(URL)),
                Ok(vec![1, 2, 3])
            );
            assert_eq!(err.text(), "", "{console:?}, json {json}");
        }
        assert!(WAIT.with(|slot| slot.borrow().is_none()));
    }

    /// Before any byte comes, the line says it waits; sizes read in decimal units.
    #[test]
    fn the_line_reads_as_it_should() {
        let second = Duration::from_millis(1_400);
        assert_eq!(line("a.org", Some(0), second), "waiting for a.org, 1 s");
        assert_eq!(line("a.org", None, second), "waiting for a.org, 1 s");
        assert_eq!(
            line("a.org", Some(3_100_000), second),
            "fetching from a.org: 3.1 MB so far, 1 s"
        );
        assert_eq!(size(999), "999 bytes");
        assert_eq!(size(1_000), "1.0 kB");
        assert_eq!(host("http://a.org"), "a.org");
        assert_eq!(host("a.org/x\u{1b}"), "a.org");
        assert_eq!(host("https://x@\u{1b}b/"), "?b");
    }
}
