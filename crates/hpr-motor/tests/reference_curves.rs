//! The motor files under the project's reference checkouts (`refs/`, not committed), read by the
//! `.eng` and `.rse` readers: none is refused as cut off ([`CUT_OFF_SHARE`]), and the test prints
//! how many curves end above zero thrust and the highest such end as a share of its peak, the
//! figures `CUT_OFF_SHARE`'s documentation quotes. Without the checkouts, as in CI, it reads none
//! and says so.

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the test reads files under refs/; not the pure core"
)]
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "a test stops at the failure (clippy.toml)"
)]

use std::fs;
use std::path::{Path, PathBuf};

use hpr_motor::curve::CUT_OFF_SHARE;
use hpr_motor::text::WarningKind;
use hpr_motor::{eng, rse};

/// Every `.eng` and `.rse` file under `dir`, sorted, not following links.
fn motor_files(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();
    for path in paths {
        let linked = fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink());
        if path.is_dir() && !linked {
            motor_files(&path, found);
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("eng") || ext.eq_ignore_ascii_case("rse"))
        {
            found.push(path);
        }
    }
}

#[test]
fn no_reference_curve_is_refused_as_cut_off() {
    let refs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../refs");
    let mut files = Vec::new();
    motor_files(&refs, &mut files);
    if files.is_empty() {
        println!("no motor files under refs/: nothing to read");
        return;
    }
    let (mut curves, mut above_zero, mut highest) = (0usize, 0usize, 0.0f64);
    let mut unread = 0usize;
    let mut cut = Vec::new();
    let mut tally = |thrusts: Vec<f64>| {
        curves += 1;
        let peak = thrusts.iter().copied().fold(0.0, f64::max);
        if let Some(&last) = thrusts.last()
            && last != 0.0
            && peak > 0.0
        {
            above_zero += 1;
            highest = highest.max(last / peak);
        }
    };
    for file in &files {
        let bytes = fs::read(file).unwrap();
        let text = String::from_utf8_lossy(&bytes);
        let rse = file
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rse"));
        let (read, warnings, error) = if rse {
            match rse::parse(&text) {
                Ok(parsed) => {
                    for engine in parsed.value.engines {
                        tally(engine.points.iter().map(|p| p.thrust_n).collect());
                    }
                    (true, parsed.warnings, None)
                }
                Err(error) => (false, Vec::new(), Some(error.to_string())),
            }
        } else {
            match eng::parse(&text) {
                Ok(parsed) => {
                    for entry in parsed.value.entries {
                        tally(entry.points.iter().map(|&(_, f)| f).collect());
                    }
                    (true, parsed.warnings, None)
                }
                Err(error) => (false, Vec::new(), Some(error.to_string())),
            }
        };
        if !read {
            unread += 1;
        }
        let skipped = warnings
            .iter()
            .filter(|w| w.kind == WarningKind::Skipped)
            .map(|w| w.message.clone());
        for message in skipped.chain(error) {
            if message.contains("looks cut off") {
                cut.push(format!("{}: {message}", file.display()));
            }
        }
    }
    println!(
        "{} motor files under refs/ ({unread} not read for other reasons): {curves} curves, \
         {above_zero} ending above zero thrust, the highest at {:.1}% of its peak; the cut-off \
         share is {:.0}%",
        files.len(),
        100.0 * highest,
        100.0 * CUT_OFF_SHARE
    );
    assert!(
        cut.is_empty(),
        "reference curves refused as cut off:\n{}",
        cut.join("\n")
    );
    assert!(highest <= CUT_OFF_SHARE);
}
