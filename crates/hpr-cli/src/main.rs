//! The `hpr` command-line tool. The commands are in the `hpr_cli` library; this hands them the
//! process's arguments and exits with the status they return.

#![allow(
    clippy::disallowed_methods,
    reason = "the command-line tool reads its arguments; it is not part of the pure core"
)]

use std::process::ExitCode;

fn main() -> ExitCode {
    // Read before the streams are wrapped. `always` passes escape codes through, and on a Windows
    // console that can't take them it writes them as the console's own colors; `hpr` writes them
    // only where `console` and `--color` say a stream gets color.
    let console = hpr_cli::console::Console::process();
    let exit = hpr_cli::run_with(
        std::env::args_os(),
        &mut anstream::AutoStream::always(std::io::stdout().lock()),
        &mut anstream::AutoStream::always(std::io::stderr().lock()),
        console,
    );
    ExitCode::from(exit.code())
}
