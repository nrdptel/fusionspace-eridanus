//! How the pages read: the voice check, its report, and the census of the commands' messages it
//! holds (M0.14a; ADR-284; the voice section of `docs/writing.md`).
//!
//! This crate reads the pages, lists them with `git ls-files` and writes the report; it needs none of
//! the simulator's crates, so it builds without them: a prose-only change
//! rewrites `validation/reports/voice.md` with `cargo voice` (this crate's binary) and tests it
//! with `cargo test -p xtask-voice`. `cargo xtask voice` runs the same code, and `cargo xtask site`
//! runs its check.

#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool reports on the terminal"
)]
#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "dev tooling runs git and uses the filesystem; it is not part of the pure core"
)]

use std::path::{Path, PathBuf};

pub mod markdown;
pub mod messages;
pub mod voice;

/// The workspace root: two directories above this crate's (`xtask/voice`).
pub fn root() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask-voice is not two directories under the workspace root".to_owned())
}
