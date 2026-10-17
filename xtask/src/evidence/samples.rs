//! `cargo xtask evidence --samples`: each real sample's file is under `refs/`, its SHA-256 is the
//! register's, and the test that reads it runs and passes without skipping. The gate's `refs`
//! step runs it; CI can't, as `refs/` is gitignored.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use sha2::{Digest, Sha256};

use super::{Kind, REGISTER, Register, Sample, decisions, read, structure};

/// What running a sample's test printed, its output and its errors together, and whether cargo
/// finished cleanly.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TestRun {
    /// Whether `cargo test` exited with success.
    pub success: bool,
    /// What it printed, standard output then standard error.
    pub output: String,
}

/// Runs the check on the repository at `root`: the register first held to its rules, then each
/// sample read and its test run.
pub(crate) fn run(root: &Path) -> Result<(), String> {
    let register = Register::parse(&read(root, REGISTER)?)?;
    let broken = structure(&register, &decisions(root)?);
    if !broken.is_empty() {
        return Err(format!(
            "{REGISTER} breaks its rules, so no sample is checked:\n{}",
            broken.join("\n")
        ));
    }
    let count = (register.row.iter())
        .filter(|row| row.kind == Kind::Real)
        .map(|row| row.sample.len())
        .sum::<usize>();
    if count == 0 {
        println!("evidence --samples: no row is real, so there is no sample to check");
        return Ok(());
    }
    let read_file = |path: &str| match fs::read(root.join(path)) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("reading {path}: {error}")),
    };
    let problems = problems(&register, &read_file, &|sample| run_test(root, sample));
    if problems.is_empty() {
        println!(
            "evidence --samples: each of the {count} samples is there, with its hash, and its \
             test ran and passed"
        );
        Ok(())
    } else {
        Err(format!(
            "evidence --samples: {} problems:\n{}",
            problems.len(),
            problems.join("\n")
        ))
    }
}

/// Reads a file from its path under the repository: its bytes, `None` if it isn't there, or why
/// it can't be read.
pub(crate) type ReadFile<'a> = &'a dyn Fn(&str) -> Result<Option<Vec<u8>>, String>;

/// Runs a sample's test, or says why it can't.
pub(crate) type RunTest<'a> = &'a dyn Fn(&Sample) -> Result<TestRun, String>;

/// Each sample's problems: missing, its hash differing, or its test not run, skipped or failed.
/// `read` gives a file's bytes from its path under the repository (`None` if it isn't there),
/// and `run` runs a sample's test. A sample whose file is missing or differs runs no test.
pub(crate) fn problems(register: &Register, read: ReadFile, run: RunTest) -> Vec<String> {
    let mut problems = Vec::new();
    for row in register.row.iter().filter(|row| row.kind == Kind::Real) {
        for sample in &row.sample {
            let at =
                |what: String| format!("{REGISTER}: {}: sample {}: {what}", row.id, sample.path);
            let bytes = match read(&sample.path) {
                Ok(Some(bytes)) => bytes,
                Ok(None) => {
                    problems.push(at("missing".to_owned()));
                    continue;
                }
                Err(error) => {
                    problems.push(at(error));
                    continue;
                }
            };
            let actual: String = (Sha256::digest(&bytes).iter())
                .map(|b| format!("{b:02x}"))
                .collect();
            if actual != sample.sha256 {
                problems.push(at(format!("hash differs: {actual}")));
                continue;
            }
            match run(sample) {
                Ok(test) => {
                    if let Err(what) = outcome(&test) {
                        problems.push(at(format!("{what} ({})", sample.test)));
                    }
                }
                Err(error) => problems.push(at(format!("its test could not be run: {error}"))),
            }
        }
    }
    problems
}

/// Whether a test's run counts as evidence: exactly one test ran and passed, none was ignored,
/// and nothing printed a line starting `skipped:`, which the repository's tests print when the
/// file they read under `refs/` isn't there.
pub(crate) fn outcome(test: &TestRun) -> Result<(), &'static str> {
    let results: Vec<&str> = (test.output.lines())
        .filter(|line| line.trim_start().starts_with("test result: "))
        .collect();
    if !test.success || results.iter().any(|line| line.contains("FAILED")) {
        return Err("its test failed");
    }
    let one = matches!(results[..], [line] if line.contains("test result: ok. 1 passed;")
        && line.contains("; 0 ignored;"));
    if !one {
        return Err("its test did not run: the output shows no single test run and passed");
    }
    if (test.output.lines()).any(|line| line.trim_start().starts_with("skipped:")) {
        return Err("its test skipped");
    }
    Ok(())
}

/// Runs a sample's test with cargo, as `cargo test --locked -p <package> --lib -- --exact <test>
/// --nocapture` (`--test <target>` for an integration test), in the repository at `root`.
fn run_test(root: &Path, sample: &Sample) -> Result<TestRun, String> {
    let mut command = Command::new(crate::workspace::cargo());
    command
        .current_dir(root)
        .args(["test", "--locked", "-p", &sample.package]);
    if sample.target == "lib" {
        command.arg("--lib");
    } else {
        command.args(["--test", &sample.target]);
    }
    command.args(["--", "--exact", &sample.test, "--nocapture"]);
    let output = command
        .output()
        .map_err(|error| format!("running cargo test: {error}"))?;
    Ok(TestRun {
        success: output.status.success(),
        output: format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    })
}
