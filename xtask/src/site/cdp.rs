//! A small client for Chrome's DevTools protocol (<https://chromedevtools.github.io/devtools-protocol/>),
//! enough for the vitals measurement ([`super::vitals`]): JSON commands over a WebSocket to a
//! headless browser started with `--remote-debugging-port=0`, each answered by the message that
//! carries its `id`. Events the browser sends in between are read and dropped, so they never pile
//! up; the measurement asks the page for what it needs instead of following events.

use std::fs;
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tungstenite::{Message, WebSocket};

/// The file Chrome writes in its profile once it listens: the port it took, then the browser's
/// WebSocket path.
const ACTIVE_PORT: &str = "DevToolsActivePort";

/// How long the browser has to write [`ACTIVE_PORT`] after it starts.
const START_TIMEOUT: Duration = Duration::from_secs(60);

/// How long one read of the socket may block. A command's own deadline is checked between reads,
/// so a browser that stops answering fails the command instead of hanging the check.
const READ_TIMEOUT: Duration = Duration::from_secs(5);

/// A connection to one browser, and the id of its next command.
pub(super) struct Cdp {
    socket: WebSocket<TcpStream>,
    next: u64,
}

impl Cdp {
    /// Connects to the browser whose profile is `profile`, once it has written [`ACTIVE_PORT`].
    /// `exited` says how the browser exited, if it has, so a browser that fails to start fails
    /// here at once instead of after [`START_TIMEOUT`].
    pub(super) fn connect(
        profile: &Path,
        mut exited: impl FnMut() -> Option<String>,
    ) -> Result<Cdp, String> {
        let file = profile.join(ACTIVE_PORT);
        let started = Instant::now();
        let (port, path) = loop {
            if let Some(port) = fs::read_to_string(&file)
                .ok()
                .and_then(|text| active_port(&text))
            {
                break port;
            }
            if let Some(status) = exited() {
                return Err(format!("the browser exited before it listened ({status})"));
            }
            if started.elapsed() > START_TIMEOUT {
                return Err(format!(
                    "the browser wrote no {} within {} s",
                    file.display(),
                    START_TIMEOUT.as_secs()
                ));
            }
            thread::sleep(Duration::from_millis(50));
        };
        let stream = TcpStream::connect(("127.0.0.1", port))
            .map_err(|err| format!("could not reach the browser on port {port}: {err}"))?;
        stream
            .set_read_timeout(Some(READ_TIMEOUT))
            .and_then(|()| stream.set_write_timeout(Some(READ_TIMEOUT)))
            .map_err(|err| format!("could not set the browser connection's timeouts: {err}"))?;
        let url = format!("ws://127.0.0.1:{port}{path}");
        let (socket, _) = tungstenite::client::client(url.as_str(), stream)
            .map_err(|err| format!("could not open the browser's WebSocket at {url}: {err}"))?;
        Ok(Cdp { socket, next: 1 })
    }

    /// Sends `method` with `params`, to the page attached as `session` or to the browser itself,
    /// and returns its result, failing on the protocol's error or after `timeout`.
    pub(super) fn call(
        &mut self,
        session: Option<&str>,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, String> {
        let id = self.next;
        self.next += 1;
        let mut message = json!({ "id": id, "method": method, "params": params });
        if let Some(session) = session {
            message["sessionId"] = Value::from(session);
        }
        self.socket
            .send(Message::text(message.to_string()))
            .map_err(|err| format!("{method}: could not send: {err}"))?;
        let deadline = Instant::now() + timeout;
        loop {
            if Instant::now() > deadline {
                return Err(format!(
                    "{method}: no answer within {} s",
                    timeout.as_secs_f64()
                ));
            }
            let text = match self.socket.read() {
                Ok(Message::Text(text)) => text,
                Ok(Message::Close(_)) => {
                    return Err(format!("{method}: the browser closed the connection"));
                }
                // Pings are answered by the library; nothing else carries an answer.
                Ok(_) => continue,
                Err(tungstenite::Error::Io(err))
                    if matches!(
                        err.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) =>
                {
                    continue;
                }
                Err(err) => return Err(format!("{method}: could not read: {err}")),
            };
            let answer: Value = serde_json::from_str(text.as_str())
                .map_err(|err| format!("{method}: the browser sent something not JSON: {err}"))?;
            if answer.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if let Some(error) = answer.get("error") {
                let why = error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("no message");
                return Err(format!("{method}: {why}"));
            }
            return Ok(answer.get("result").cloned().unwrap_or(Value::Null));
        }
    }
}

/// The port and WebSocket path [`ACTIVE_PORT`] names, once the browser has written both lines.
fn active_port(text: &str) -> Option<(u16, String)> {
    let mut lines = text.lines();
    let port = lines.next()?.trim().parse().ok()?;
    let path = lines.next()?.trim();
    path.starts_with("/devtools/browser/")
        .then(|| (port, path.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_active_port_file_is_read_once_both_lines_are_there() {
        assert_eq!(
            active_port("9222\n/devtools/browser/abc-123\n"),
            Some((9222, "/devtools/browser/abc-123".to_owned()))
        );
        // Chrome writes the file in one go, but a read can land before the second line.
        assert_eq!(active_port("9222\n"), None);
        assert_eq!(active_port(""), None);
        assert_eq!(active_port("port\n/devtools/browser/x"), None);
        assert_eq!(active_port("9222\n/devtools/page/x"), None);
    }
}
