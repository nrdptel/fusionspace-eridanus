//! `cargo voice`: the voice check and its report, built without the simulator (#1459). It takes
//! what `cargo xtask voice` takes.

#![allow(
    clippy::print_stderr,
    reason = "a command-line tool reports on the terminal"
)]
#![allow(
    clippy::disallowed_methods,
    reason = "dev tooling reads its arguments; it is not part of the pure core"
)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match xtask_voice::voice::run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
