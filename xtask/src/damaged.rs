//! Damaged input (M0.15): each reader of outside bytes fed damaged copies of real input. A damaged
//! file or a server's damaged answer gets an error, never a panic or a hang; `clippy` can't say so
//! (`indexing_slicing` flagged 1,524 places in the crates that read one), so the tests try it.
//!
//! [`probe`] runs one read on its own thread and says how it ended. The list of readers and the
//! tests that walk it are in `damaged/tests.rs`; `xtask logs --check`, `xtask ork-flights --library
//! --check` and `xtask fixture-flights --check` read [`PRIVATE_CUTS`] cuts of each private file
//! through [`Tally`].

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once, mpsc};
use std::thread;
use std::time::Duration;

/// How long one read may take before it counts as a hang (M0.15's *done when*).
pub const READ_LIMIT: Duration = Duration::from_secs(10);

/// How many cuts of each private log and design the corpus checks read.
pub const PRIVATE_CUTS: usize = 8;

/// The name of the threads a read runs on, whose panics the hook keeps out of the output.
const THREAD_NAME: &str = "damaged-read";

/// How one read of damaged bytes ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ended {
    /// It gave a value.
    Read,
    /// It gave an error.
    Refused,
    /// It panicked: where, and the panic's message.
    Panicked { at: String, message: String },
    /// It was still running at [`READ_LIMIT`]; its thread is left behind.
    Hung,
}

thread_local! {
    /// Where this thread's last panic happened, which the payload doesn't say.
    static PANICKED_AT: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Records where a read's panic happened and keeps it out of the output; a private file's bytes can
/// be in a panic's message. Any other thread's panic goes to the hook that was there.
fn quiet_reads_panics() {
    static HOOK: Once = Once::new();
    HOOK.call_once(|| {
        let earlier = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if thread::current().name() == Some(THREAD_NAME) {
                let at = info.location().map(ToString::to_string).unwrap_or_default();
                PANICKED_AT.with(|cell| *cell.borrow_mut() = at);
            } else {
                earlier(info);
            }
        }));
    });
}

/// Runs `read` over `bytes` on its own thread, for at most [`READ_LIMIT`]. `read` says whether it
/// gave a value. A stack overflow or a failed allocation ends the process, which fails the test.
pub fn probe(read: fn(&[u8]) -> bool, bytes: Arc<[u8]>) -> Ended {
    probe_within(read, bytes, READ_LIMIT)
}

/// [`probe`], its limit given.
fn probe_within(read: fn(&[u8]) -> bool, bytes: Arc<[u8]>, limit: Duration) -> Ended {
    quiet_reads_panics();
    let (send, receive) = mpsc::channel();
    let thread = thread::Builder::new()
        .name(THREAD_NAME.to_owned())
        .spawn(move || {
            let ended = match panic::catch_unwind(AssertUnwindSafe(|| read(&bytes))) {
                Ok(true) => Ended::Read,
                Ok(false) => Ended::Refused,
                Err(payload) => Ended::Panicked {
                    at: PANICKED_AT.with(|cell| cell.borrow().clone()),
                    message: payload
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                        .unwrap_or_default(),
                },
            };
            // The receiver is gone only once the read has already counted as a hang.
            let _ = send.send(ended);
        });
    if thread.is_err() {
        return Ended::Panicked {
            at: "the probe".to_owned(),
            message: "no thread for the read".to_owned(),
        };
    }
    receive.recv_timeout(limit).unwrap_or(Ended::Hung)
}

/// `count` places in `len` bytes spread evenly, from the first: `k·len/count`. They repeat only in
/// a sample shorter than `count`.
pub fn evenly(len: usize, count: usize) -> Vec<usize> {
    (0..count)
        .map(|k| {
            // A file's length fits in u128 times a small count, so this doesn't overflow.
            usize::try_from(k as u128 * len as u128 / count as u128).unwrap_or(len)
        })
        .collect()
}

/// What the cuts of a private corpus came to. Counts only: a panic's message or a file's name stays
/// out of what a public report could quote; the failures, with the names the caller gave, are for
/// its own console.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tally {
    /// Cuts that gave a value.
    pub read: usize,
    /// Cuts that gave an error.
    pub refused: usize,
    /// Cuts still reading at [`READ_LIMIT`]: a long private log's read in a debug build can be
    /// slow, so these are counted and named, and don't fail the check as a panic does.
    pub slow: Vec<String>,
    /// One line for each panic: the file's name as the caller gave it, the cut, where.
    pub panics: Vec<String>,
    /// Files that couldn't be loaded to be cut, which fail the check too.
    pub unloaded: Vec<String>,
}

impl Tally {
    /// Reads [`PRIVATE_CUTS`] cuts of `bytes`, none of them whole or empty.
    pub fn cuts_of(&mut self, name: &str, bytes: &[u8], read: fn(&[u8]) -> bool) {
        for at in evenly(bytes.len(), PRIVATE_CUTS + 1).into_iter().skip(1) {
            let cut: Arc<[u8]> = Arc::from(bytes.get(..at).unwrap_or(bytes));
            self.count(&format!("{name}: cut at byte {at}"), probe(read, cut));
        }
    }

    /// Counts how one read ended.
    fn count(&mut self, what: &str, ended: Ended) {
        match ended {
            Ended::Read => self.read += 1,
            Ended::Refused => self.refused += 1,
            Ended::Panicked { at, .. } => self.panics.push(format!("{what}: panicked at {at}")),
            Ended::Hung => self.slow.push(what.to_owned()),
        }
    }

    /// Adds another tally's counts to this one.
    fn add(&mut self, other: Self) {
        self.read += other.read;
        self.refused += other.refused;
        self.slow.extend(other.slow);
        self.panics.extend(other.panics);
        self.unloaded.extend(other.unloaded);
    }

    /// Loads a file, or says it couldn't be; `cut` gets its bytes.
    pub fn load(&mut self, name: &str, path: &Path, cut: impl FnOnce(&mut Self, &[u8])) {
        match fs::read(path) {
            Ok(bytes) => cut(self, &bytes),
            Err(error) => self.unloaded.push(format!("{name}: {error}")),
        }
    }

    /// Prints what the cuts came to; a panic or a file that couldn't be loaded fails the check.
    pub fn check(&self, what: &str) -> Result<(), String> {
        println!(
            "damaged: {PRIVATE_CUTS} cuts of each {what}: {} read, {} refused, {} past {} s, {} \
             panicked",
            self.read,
            self.refused,
            self.slow.len(),
            READ_LIMIT.as_secs(),
            self.panics.len()
        );
        for line in &self.slow {
            println!("damaged: past {} s: {line}", READ_LIMIT.as_secs());
        }
        if !self.unloaded.is_empty() {
            return Err(format!(
                "{} file(s) of the {what} check couldn't be loaded:\n{}",
                self.unloaded.len(),
                self.unloaded.join("\n")
            ));
        }
        if self.panics.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{} cut(s) of a {what} panicked:\n{}",
                self.panics.len(),
                self.panics.join("\n")
            ))
        }
    }
}

/// `cut` over each of `files` on every core, what they come to added up. A design is loaded only
/// as its turn comes, so a corpus isn't all held at once.
pub fn tally_cuts<T: Sync>(files: &[T], cut: impl Fn(&T, &mut Tally) + Sync) -> Tally {
    let next = AtomicUsize::new(0);
    let width = thread::available_parallelism().map_or(2, usize::from);
    let tallies: Vec<Tally> = thread::scope(|scope| {
        let workers: Vec<_> = (0..width)
            .map(|_| {
                scope.spawn(|| {
                    let mut tally = Tally::default();
                    while let Some(file) = files.get(next.fetch_add(1, Ordering::Relaxed)) {
                        cut(file, &mut tally);
                    }
                    tally
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| {
                worker.join().unwrap_or_else(|_| Tally {
                    panics: vec!["a worker of the cuts' tally panicked".to_owned()],
                    ..Tally::default()
                })
            })
            .collect()
    });
    let mut whole = Tally::default();
    for tally in tallies {
        whole.add(tally);
    }
    // In an order that doesn't hang on the workers'.
    whole.slow.sort();
    whole.panics.sort();
    whole.unloaded.sort();
    whole
}

/// A log as `hpr analyze` reads one: the file, then its readings.
pub fn read_log(bytes: &[u8]) -> bool {
    hpr_flightdata::import::read_one(bytes, None, None).is_ok_and(|log| {
        drop(hpr_flightdata::readings::read(&log));
        true
    })
}

/// A `.ork` file, then the design it holds.
fn read_design(bytes: &[u8]) -> bool {
    hpr_io::ork::read(bytes).is_ok_and(|read| {
        drop(hpr_io::ork::design(&read.value));
        true
    })
}

/// The XML inside a `.ork`.
fn read_design_xml(bytes: &[u8]) -> bool {
    hpr_io::ork::Document::parse(&String::from_utf8_lossy(bytes)).is_ok()
}

/// The cuts of one design: a cut of the zip can't get past its directory at the file's end, so the
/// XML inside is cut too. [`PRIVATE_CUTS`] of each.
pub fn cuts_of_design(tally: &mut Tally, name: &str, bytes: &[u8]) {
    tally.cuts_of(name, bytes, read_design);
    // A whole file's unpacking runs under a probe too, so its panic is counted, not printed.
    let unpacks = |bytes: &[u8]| hpr_io::ork::container::unpack(bytes).is_ok();
    match probe(unpacks, Arc::from(bytes)) {
        Ended::Read => {
            if let Ok(unpacked) = hpr_io::ork::container::unpack(bytes) {
                tally.cuts_of(
                    &format!("{name}, its XML"),
                    unpacked.value.design.as_bytes(),
                    read_design_xml,
                );
            }
        }
        Ended::Refused => {}
        ended => tally.count(&format!("{name}: unpacking"), ended),
    }
}

/// Every `.ork` under `dir`, its subfolders' too, each content once, by path: a collection holds a
/// design under several flights.
pub fn designs_under(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let (mut folders, mut seen, mut designs) = (vec![dir.to_owned()], BTreeSet::new(), Vec::new());
    while let Some(folder) = folders.pop() {
        let mut entries: Vec<PathBuf> = fs::read_dir(&folder)
            .map_err(|error| format!("{}: {error}", folder.display()))?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<_, _>>()
            .map_err(|error| format!("{}: {error}", folder.display()))?;
        entries.sort();
        for path in entries {
            if path.is_dir() {
                folders.push(path);
            } else if path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("ork"))
            {
                let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
                if seen.insert(crate::ork_supply::sha256(&bytes)) {
                    designs.push(path);
                }
            }
        }
    }
    Ok(designs)
}

#[cfg(test)]
mod tests;
