//! Each command's `--help`, pinned: the text `hpr` prints is compared with the snapshot in
//! `tests/help/`, one file a command, named by its path (`hpr-motors-show.help`). A change to a
//! command's help fails here until its snapshot is written again:
//! `HPR_BLESS=1 cargo test -p fusionspace-hpr-cli --test help`. `cargo xtask voice` reads the
//! snapshots and holds each one to the writing guide's voice limits (M0.14d1).

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the tests run the built `hpr` and read and write the snapshots; not the pure core"
)]
#![allow(
    clippy::panic,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Where the snapshots are.
fn snapshots() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/help")
}

/// Every command's path from `hpr`, `hpr` itself first, less clap's own `help` command (its
/// `--help` is refused: it has none).
fn commands() -> Vec<Vec<String>> {
    fn walk(command: &clap::Command, path: &[String], found: &mut Vec<Vec<String>>) {
        found.push(path.to_vec());
        for sub in command.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            let mut next = path.to_vec();
            next.push(sub.get_name().to_owned());
            walk(sub, &next, found);
        }
    }
    let mut found = Vec::new();
    walk(&hpr_cli::command(), &[], &mut found);
    found
}

/// The snapshot's file name for a command's path: `hpr`, then each command, joined by hyphens.
fn file_name(path: &[String]) -> String {
    let mut name = String::from("hpr");
    for part in path {
        name.push('-');
        name.push_str(part);
    }
    name.push_str(".help");
    name
}

/// What `hpr <path> --help` prints, piped, so with no color whatever the environment says.
fn help(path: &[String]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_hpr"))
        .args(path)
        .arg("--help")
        .env("NO_COLOR", "1")
        .env_remove("FORCE_COLOR")
        .env_remove("CLICOLOR_FORCE")
        .output()
        .unwrap_or_else(|e| panic!("running hpr {path:?} --help: {e}"));
    assert_eq!(output.status.code(), Some(0), "{path:?}: {output:?}");
    assert!(output.stderr.is_empty(), "{path:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap_or_else(|e| panic!("{path:?}: {e}"))
}

#[test]
fn each_help_matches_its_snapshot() {
    let bless = std::env::var_os("HPR_BLESS").is_some();
    let mut stale = Vec::new();
    for path in commands() {
        let file = snapshots().join(file_name(&path));
        let printed = help(&path);
        if bless {
            fs::create_dir_all(snapshots()).unwrap_or_else(|e| panic!("{e}"));
            fs::write(&file, &printed).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
            continue;
        }
        // Git may check a snapshot out with CRLF line ends on Windows; the help has LF.
        let pinned = fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("{}: {e}", file.display()))
            .replace("\r\n", "\n");
        if pinned != printed {
            let at = pinned
                .lines()
                .zip(printed.lines())
                .position(|(a, b)| a != b)
                .unwrap_or_else(|| pinned.lines().count().min(printed.lines().count()));
            stale.push(format!(
                "{}: line {}\n  pinned:  {:?}\n  printed: {:?}",
                file_name(&path),
                at + 1,
                pinned.lines().nth(at).unwrap_or(""),
                printed.lines().nth(at).unwrap_or("")
            ));
        }
    }
    assert!(
        stale.is_empty(),
        "help that differs from its snapshot; if the change is meant, write the snapshots again \
         with `HPR_BLESS=1 cargo test -p fusionspace-hpr-cli --test help` and review the diff:\n{}",
        stale.join("\n")
    );
}

/// Each snapshot is a command's: one left behind by a removed or renamed command fails, so the
/// voice check doesn't hold help that `hpr` no longer prints.
#[test]
fn each_snapshot_is_a_commands() {
    let expected: Vec<String> = commands().iter().map(|path| file_name(path)).collect();
    let mut found: Vec<String> = fs::read_dir(snapshots())
        .unwrap_or_else(|e| panic!("{}: {e}", snapshots().display()))
        .map(|entry| {
            entry
                .unwrap_or_else(|e| panic!("{e}"))
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".help"))
        .collect();
    found.sort();
    let mut sorted = expected.clone();
    sorted.sort();
    assert_eq!(
        found, sorted,
        "the snapshots in tests/help/ and the commands"
    );
    // The walk reaches the nested commands too.
    assert!(expected.contains(&"hpr-motors-show.help".to_owned()));
    assert!(expected.contains(&"hpr-weather-era5.help".to_owned()));
}

/// The usage lines say `hpr` whatever the binary's file is called, as on Windows, where it's
/// `hpr.exe`: clap would otherwise take the name it was started by.
#[test]
fn the_usage_line_says_hpr_whatever_the_file_is_called() {
    for args in [
        &["C:\\tools\\hpr.exe", "--help"][..],
        &["/usr/local/bin/hpr.exe", "sim", "--help"],
        &["hpr-0.2", "motors", "show", "--help"],
    ] {
        let Err(error) = hpr_cli::command().try_get_matches_from(args) else {
            panic!("{args:?}: no help");
        };
        assert_eq!(
            error.kind(),
            clap::error::ErrorKind::DisplayHelp,
            "{args:?}"
        );
        let help = error.render().to_string();
        let usage = help
            .lines()
            .find(|line| line.starts_with("Usage: "))
            .unwrap_or_else(|| panic!("{args:?}\n{help}"));
        let words: Vec<&str> = usage.split(' ').collect();
        assert_eq!(words[1], "hpr", "{args:?}: {usage}");
    }
}
