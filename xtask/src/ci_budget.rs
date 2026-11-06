//! `cargo xtask ci-budget`: the time budget for CI and the gate, checked before a pull request
//! opens (M0.16a3, [ADR-149] amended 2026-11-03 and 2026-11-04).
//!
//! The budget: a pull request's CI within 20 minutes, counted as its longest job's run, a wait
//! for a runner apart; a passing full `scripts/gate.sh` within 30 minutes on the Mac; and the
//! flight censuses whole each night on main. A pull request's own run hasn't finished when the
//! check runs, so three records stand in for it:
//!
//! 1. **The gate.** The newest passing full run in the gate's history (`gate-history.tsv` in the
//!    git directory, or `$HPR_GATE_HISTORY`, as `scripts/gate.sh` writes it) at the branch's
//!    head (`HEAD`, or `--head <commit>`) or a commit on it since it left `origin/main`. Over
//!    1,800 s fails. A branch with no such run yet is a note: the gate is asked for before a
//!    merge.
//! 2. **A pull request's jobs.** Main's last passing CI run that ran a pull request's jobs: one
//!    started by a push, or by hand without `full`, in which no `census` job ran (ADR-149 §2) and
//!    `types` did. A run that skipped `types` changed research notes alone, so most of its checks
//!    ran no step; the check reads the run before it. A job whose run, from `started_at` to
//!    `completed_at`, passes 1,200 s fails. The night's jobs are a full run's, not a pull
//!    request's; they are printed, not held.
//! 3. **The night.** A passing run on main, nightly or started by hand with `full`, in which every
//!    `census` job and `fit intervals` passed, made within the last 26 hours by its first
//!    attempt's time (a re-run of an old night's failed jobs is no new night). None fails.
//!
//! Reading GitHub's runs needs `gh` and the network; a failure to read them fails the check
//! and says why.
//!
//! [ADR-149]: ../../docs/decisions/0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

pub const USAGE: &str = "  ci-budget [--head <commit>]
                           Check the time budget before a pull request (ADR-149): the
                           newest passing full gate on the branch whose head is HEAD (or
                           <commit>) within 30 minutes, every job of main's last passing
                           CI run with a pull request's jobs (pushed, or by hand without
                           `full`) within 20, and a passing nightly (or hand-started full)
                           run on main within 26 hours. Reads the gate's history and
                           GitHub's runs with `gh`.";

/// The repository whose runs are read.
const REPOSITORY: &str = "nrdptel/fusionspace-eridanus";
/// The workflow whose runs are read.
const WORKFLOW: &str = "ci.yml";
/// A passing full gate's budget on the Mac, seconds (ADR-149 §1).
pub(crate) const GATE_BUDGET_S: i64 = 30 * 60;
/// A pull request's longest job's budget, seconds, a wait for a runner apart (ADR-149 §1).
pub(crate) const JOB_BUDGET_S: i64 = 20 * 60;
/// How old main's newest passing full run may be, seconds: a day and two hours' slack.
pub(crate) const NIGHT_WINDOW_S: i64 = 26 * 3600;
/// The job that a run skips when it changes research notes alone (`ci.yml`'s `scope`).
const NOTES_SKIP_JOB: &str = "types";
/// The census jobs' names start so, one a platform (`census (windows-latest)`).
const CENSUS_JOB_PREFIX: &str = "census (";
/// The job that checks the fit's whole report from its parts.
const FIT_JOB: &str = "fit intervals";
/// Push runs read before giving up on finding one that changed more than research notes.
const PUSH_RUNS: usize = 30;
/// Hand-started runs read, full or not.
const HAND_RUNS: usize = 10;
/// Nightly runs read.
const NIGHT_RUNS: usize = 5;
/// GitHub's largest page of jobs; a run with more is refused rather than read in part.
const JOBS_PAGE: usize = 100;

/// What one of the three checks found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Verdict {
    /// Within the budget.
    Ok(String),
    /// Nothing to hold yet; not a failure.
    Note(String),
    /// Over the budget, or the night missing.
    Fail(String),
}

impl Verdict {
    fn line(&self) -> String {
        match self {
            Verdict::Ok(text) => format!("ok    {text}"),
            Verdict::Note(text) => format!("note  {text}"),
            Verdict::Fail(text) => format!("FAIL  {text}"),
        }
    }
}

/// One workflow run as GitHub's runs API writes it.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Run {
    /// The run's id.
    pub(crate) id: u64,
    /// What started it: `push`, `schedule`, `workflow_dispatch`, `pull_request`.
    pub(crate) event: String,
    /// `success`, `failure`, ...; absent while the run is going.
    pub(crate) conclusion: Option<String>,
    /// When the run was made, UTC, `YYYY-MM-DDTHH:MM:SSZ`: its first attempt's time, which a
    /// re-run keeps (`run_started_at` moves to the re-run's).
    pub(crate) created_at: String,
    /// The commit it ran on.
    pub(crate) head_sha: String,
}

/// A page of runs.
#[derive(Debug, Deserialize)]
struct Runs {
    workflow_runs: Vec<Run>,
}

/// One job of a run as GitHub's jobs API writes it.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Job {
    /// The job's name as the run shows it (`test (windows-latest, 1/2)`).
    pub(crate) name: String,
    /// `success`, `skipped`, `failure`, ...; absent while it runs.
    pub(crate) conclusion: Option<String>,
    /// When a runner started it, UTC; absent for a job that never started.
    pub(crate) started_at: Option<String>,
    /// When it finished, UTC.
    pub(crate) completed_at: Option<String>,
}

/// A page of jobs, with the run's whole count.
#[derive(Debug, Deserialize)]
struct Jobs {
    total_count: usize,
    jobs: Vec<Job>,
}

/// Seconds since 1970-01-01T00:00:00Z of a UTC time written `YYYY-MM-DDTHH:MM:SSZ`, as GitHub
/// writes them, by the days-from-civil count of the proleptic Gregorian calendar. Refuses any
/// other form, and fields out of range.
pub(crate) fn unix_seconds(time: &str) -> Result<i64, String> {
    let bad = || format!("`{time}` is not a UTC time written YYYY-MM-DDTHH:MM:SSZ");
    let b = time.as_bytes();
    if b.len() != 20
        || b[4] != b'-'
        || b[7] != b'-'
        || b[10] != b'T'
        || b[13] != b':'
        || b[16] != b':'
        || b[19] != b'Z'
    {
        return Err(bad());
    }
    let field = |from: usize, to: usize| -> Result<i64, String> {
        let digits = &time[from..to];
        if !digits.bytes().all(|c| c.is_ascii_digit()) {
            return Err(bad());
        }
        digits.parse::<i64>().map_err(|_| bad())
    };
    let (year, month, day) = (field(0, 4)?, field(5, 7)?, field(8, 10)?);
    let (hour, minute, second) = (field(11, 13)?, field(14, 16)?, field(17, 19)?);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return Err(bad()),
    };
    if day < 1 || day > month_days || hour > 23 || minute > 59 || second > 60 {
        return Err(bad());
    }
    // Days from 1970-01-01 (H. Hinnant, "chrono-Compatible Low-Level Date Algorithms").
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Ok(days * 86_400 + hour * 3600 + minute * 60 + second)
}

/// A job's run in seconds, from a runner's start to its end; `None` if it never started or
/// hasn't ended.
pub(crate) fn job_seconds(job: &Job) -> Result<Option<i64>, String> {
    match (&job.started_at, &job.completed_at) {
        (Some(start), Some(end)) => Ok(Some(unix_seconds(end)? - unix_seconds(start)?)),
        _ => Ok(None),
    }
}

/// Minutes and seconds, `18m37s`.
fn minutes(seconds: i64) -> String {
    format!("{}m{:02}s", seconds / 60, seconds % 60)
}

/// The first eight characters of a commit's name, or all of it if shorter or not ASCII there.
fn short(sha: &str) -> &str {
    sha.get(..8).unwrap_or(sha)
}

/// One run of the gate, a line of its history.
struct GateRow<'a> {
    time: &'a str,
    head: &'a str,
    full: bool,
    passed: bool,
    seconds: i64,
    /// Each step's seconds, and whether it failed, in the order it ran.
    steps: Vec<(&'a str, i64, bool)>,
    /// The prose rule left validate out (`validate-left`).
    validate_left: bool,
    /// The page-prose rule left the site's layout check out (`site-left`).
    site_left: bool,
}

/// A line of the gate's history: UTC time, head, whether the tree had changes, whether every
/// default step ran (`1`), `PASS` or `FAIL`, the seconds, and each step's `name=seconds`
/// (`:FAIL` after a failed one), then `validate-left` and `site-left` when the prose rules left
/// validate or the site's layout check out.
fn gate_row(n: usize, line: &str) -> Result<GateRow<'_>, String> {
    let fields: Vec<&str> = line.split('\t').collect();
    if fields.len() < 6 {
        return Err(format!("gate history line {n}: fewer than six fields"));
    }
    let seconds = |text: &str| -> Result<i64, String> {
        text.parse()
            .map_err(|_| format!("gate history line {n}: `{text}` is no seconds"))
    };
    let mut steps = Vec::new();
    let mut validate_left = false;
    let mut site_left = false;
    for token in fields.get(6).copied().unwrap_or("").split_whitespace() {
        if token == "validate-left" {
            validate_left = true;
            continue;
        }
        if token == "site-left" {
            site_left = true;
            continue;
        }
        let (name, time) = token
            .split_once('=')
            .ok_or_else(|| format!("gate history line {n}: `{token}` is no step's time"))?;
        let (time, failed) = match time.strip_suffix(":FAIL") {
            Some(time) => (time, true),
            None => (time, false),
        };
        steps.push((name, seconds(time)?, failed));
    }
    Ok(GateRow {
        time: fields[0],
        head: fields[1],
        full: fields[3] == "1",
        passed: fields[4] == "PASS",
        seconds: seconds(fields[5])?,
        steps,
        validate_left,
        site_left,
    })
}

/// The gate's check, on the history's runs whose head is one of `commits` (the branch's head
/// and its commits since `origin/main`): the newest passing full run's seconds, each step's
/// replaced by the newest passing reading of that step in a later run, so a step run alone after
/// it (`scripts/gate.sh validate`) still counts. Over [`GATE_BUDGET_S`] fails.
pub(crate) fn gate_verdict(history: &str, commits: &[String]) -> Result<Verdict, String> {
    let mut rows = Vec::new();
    for (n, line) in history.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row = gate_row(n + 1, line)?;
        if commits.iter().any(|c| c == row.head) {
            rows.push(row);
        }
    }
    let Some(at) = rows.iter().rposition(|row| row.full && row.passed) else {
        return Ok(Verdict::Note(
            "gate: no passing full run on this branch yet (run `scripts/gate.sh`)".to_owned(),
        ));
    };
    let full = &rows[at];
    // The newest passing reading of each step after the full run, by name.
    let mut newer: Vec<(&str, i64, &str)> = Vec::new();
    for row in &rows[at + 1..] {
        for &(name, seconds, failed) in &row.steps {
            if failed {
                continue;
            }
            newer.retain(|(n, _, _)| *n != name);
            newer.push((name, seconds, row.head));
        }
    }
    let mut total = full.seconds;
    let mut changed = Vec::new();
    for &(name, seconds, head) in &newer {
        let old = full
            .steps
            .iter()
            .find(|(n, _, _)| *n == name)
            .map_or(0, |&(_, s, _)| s);
        if seconds != old {
            total += seconds - old;
            changed.push(format!("{name} {seconds} s at {}", short(head)));
        }
    }
    let mut what = format!(
        "gate: the newest passing full run on this branch, {} at {}, took {} s ({})",
        full.time,
        short(full.head),
        full.seconds,
        minutes(full.seconds)
    );
    if full.validate_left {
        what.push_str(", validate left out as the branch changed only prose");
    }
    if full.site_left {
        what.push_str(", the site left out as the branch changed only the words of pages");
    }
    if !changed.is_empty() {
        let _ = write!(
            what,
            "; with the newer steps ({}), {total} s ({})",
            changed.join(", "),
            minutes(total)
        );
    }
    Ok(if total > GATE_BUDGET_S {
        Verdict::Fail(format!(
            "{what}, over the {GATE_BUDGET_S} s budget: split what made it slow (ADR-149 §1)"
        ))
    } else {
        Verdict::Ok(format!("{what}, within {GATE_BUDGET_S} s"))
    })
}

/// Whether a run ran a pull request's jobs: its `types` job passed (a run that changed research
/// notes alone skips it, and most of its checks run no step), and no `census` job ran (only a
/// full run's do).
pub(crate) fn pull_request_jobs(jobs: &[Job]) -> bool {
    fn conclusion(job: &Job) -> Option<&str> {
        job.conclusion.as_deref()
    }
    jobs.iter()
        .any(|job| job.name == NOTES_SKIP_JOB && conclusion(job) == Some("success"))
        && jobs
            .iter()
            .filter(|job| job.name.starts_with(CENSUS_JOB_PREFIX))
            .all(|job| conclusion(job) == Some("skipped"))
}

/// The jobs' check on main's last passing push run that changed more than research notes: every
/// job within the budget, the longest named either way.
pub(crate) fn jobs_verdict(run: &Run, jobs: &[Job]) -> Result<Verdict, String> {
    let mut timed = Vec::new();
    for job in jobs {
        if let Some(seconds) = job_seconds(job)? {
            timed.push((seconds, job.name.as_str()));
        }
    }
    timed.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
    let Some(&(longest, name)) = timed.first() else {
        return Err(format!("run {}: no job with a start and an end", run.id));
    };
    let what = format!(
        "jobs: main's last passing run with a pull request's jobs, {} ({}) at {} ({} jobs), \
         longest `{name}` {longest} s ({})",
        run.id,
        run.event,
        short(&run.head_sha),
        timed.len(),
        minutes(longest)
    );
    let over: Vec<String> = timed
        .iter()
        .filter(|(seconds, _)| *seconds > JOB_BUDGET_S)
        .map(|(seconds, name)| format!("`{name}` {seconds} s"))
        .collect();
    Ok(if over.is_empty() {
        Verdict::Ok(format!("{what}, within {JOB_BUDGET_S} s"))
    } else {
        Verdict::Fail(format!(
            "{what}; over the {JOB_BUDGET_S} s budget: {}; split them (ADR-149 §1)",
            over.join(", ")
        ))
    })
}

/// Whether a run is a passing full one: it passed, every `census` job (at least one) passed,
/// and so did `fit intervals`.
pub(crate) fn full_pass(run: &Run, jobs: &[Job]) -> bool {
    let passed = |job: &Job| job.conclusion.as_deref() == Some("success");
    let census: Vec<&Job> = jobs
        .iter()
        .filter(|job| job.name.starts_with(CENSUS_JOB_PREFIX))
        .collect();
    run.conclusion.as_deref() == Some("success")
        && !census.is_empty()
        && census.iter().all(|job| passed(job))
        && jobs.iter().any(|job| job.name == FIT_JOB && passed(job))
}

/// The night's check, from main's passing nightly and hand-started runs, each with its jobs,
/// newest first: the newest passing full run must have been made within 26 hours of `now`, by
/// its first attempt's time, so a re-run of an old night's failed jobs passes for no new night.
pub(crate) fn night_verdict(runs: &[(Run, Vec<Job>)], now: i64) -> Result<Verdict, String> {
    for (run, jobs) in runs {
        if !full_pass(run, jobs) {
            continue;
        }
        let age = now - unix_seconds(&run.created_at)?;
        let what = format!(
            "night: main's newest passing full run, {} ({}) made at {}, {} ago",
            run.id,
            run.event,
            run.created_at,
            hours(age)
        );
        return Ok(if age > NIGHT_WINDOW_S {
            Verdict::Fail(format!(
                "{what}, over 26 h: find why the night didn't run or pass (ADR-149 §3)"
            ))
        } else {
            Verdict::Ok(format!("{what}, within 26 h"))
        });
    }
    Ok(Verdict::Fail(
        "night: no passing full run on main among the runs read (ADR-149 §2, §3)".to_owned(),
    ))
}

/// Hours and minutes, `25h04m`.
fn hours(seconds: i64) -> String {
    let minutes = seconds.max(0) / 60;
    format!("{}h{:02}m", minutes / 60, minutes % 60)
}

/// The night's longest jobs, printed for the record: they are a full run's, not a pull
/// request's, so the budget doesn't hold them.
fn night_jobs_line(jobs: &[Job]) -> Result<String, String> {
    let mut timed = Vec::new();
    for job in jobs {
        if let Some(seconds) = job_seconds(job)? {
            timed.push((seconds, job.name.as_str()));
        }
    }
    timed.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
    let longest: Vec<String> = timed
        .iter()
        .take(3)
        .map(|(seconds, name)| format!("`{name}` {}", minutes(*seconds)))
        .collect();
    Ok(format!(
        "note  night: its longest jobs, a full run's and not held to {JOB_BUDGET_S} s: {}",
        longest.join(", ")
    ))
}

/// `gh api <path>`'s output.
fn gh_api(path: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .args(["api", path])
        .output()
        .map_err(|error| format!("running `gh api {path}`: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "`gh api {path}` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Main's passing runs of CI started by `event`, newest first.
fn passing_runs(event: &str, count: usize) -> Result<Vec<Run>, String> {
    let path = format!(
        "repos/{REPOSITORY}/actions/workflows/{WORKFLOW}/runs?branch=main&status=success\
         &event={event}&per_page={count}"
    );
    let runs: Runs = serde_json::from_str(&gh_api(&path)?)
        .map_err(|error| format!("reading {event} runs: {error}"))?;
    Ok(runs.workflow_runs)
}

/// A run's jobs, its latest attempt's, refusing one with more than a page of them.
fn run_jobs(run: &Run) -> Result<Vec<Job>, String> {
    let path = format!(
        "repos/{REPOSITORY}/actions/runs/{}/jobs?filter=latest&per_page={JOBS_PAGE}",
        run.id
    );
    let jobs: Jobs = serde_json::from_str(&gh_api(&path)?)
        .map_err(|error| format!("reading run {}'s jobs: {error}", run.id))?;
    if jobs.total_count > jobs.jobs.len() {
        return Err(format!(
            "run {} has {} jobs, more than one page of {JOBS_PAGE}: read them all",
            run.id, jobs.total_count
        ));
    }
    Ok(jobs.jobs)
}

/// `git <args>`'s output, run in the current directory: the branch being checked.
fn git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| format!("running `git {}`: {error}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The gate's history file, where `scripts/gate.sh` writes it.
fn history_path() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("HPR_GATE_HISTORY").filter(|p| !p.is_empty()) {
        return Ok(PathBuf::from(path));
    }
    Ok(Path::new(&git(&["rev-parse", "--git-common-dir"])?).join("gate-history.tsv"))
}

/// The branch's head (`rev`, a commit) and its commits since it left `origin/main`.
fn branch_commits(rev: &str) -> Result<Vec<String>, String> {
    let commit = format!("{rev}^{{commit}}");
    let head = git(&["rev-parse", "--verify", "--end-of-options", &commit])?;
    let mut commits: Vec<String> = git(&["rev-list", &head, "--not", "origin/main"])?
        .lines()
        .map(str::to_owned)
        .collect();
    if !commits.contains(&head) {
        commits.push(head);
    }
    Ok(commits)
}

/// The command: print each check's line, and fail if any failed.
pub fn run(args: &[String]) -> Result<(), String> {
    let head = match args {
        [] => "HEAD",
        [flag, rev] if flag == "--head" => rev.as_str(),
        _ => return Err(format!("ci-budget takes only `--head <commit>`\n{USAGE}")),
    };
    let mut verdicts = Vec::new();

    let path = history_path()?;
    let history = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(format!("{}: {error}", path.display())),
    };
    verdicts.push(gate_verdict(&history, &branch_commits(head)?)?);

    let hand = passing_runs("workflow_dispatch", HAND_RUNS)?;
    let mut runs = passing_runs("push", PUSH_RUNS)?;
    runs.extend(hand.iter().cloned());
    runs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let mut read: HashMap<u64, Vec<Job>> = HashMap::new();
    let mut jobs_found = None;
    for run in &runs {
        let jobs = read.entry(run.id).or_insert(run_jobs(run)?);
        if pull_request_jobs(jobs) {
            jobs_found = Some(jobs_verdict(run, jobs)?);
            break;
        }
    }
    verdicts.push(jobs_found.unwrap_or_else(|| {
        Verdict::Fail(format!(
            "jobs: none of main's last {PUSH_RUNS} passing push runs and {HAND_RUNS} hand-started \
             ones ran a pull request's jobs; read further back"
        ))
    }));

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("the clock reads before 1970: {error}"))?
        .as_secs();
    let now = i64::try_from(now).map_err(|_| "the clock is past i64's seconds".to_owned())?;
    let mut nights = passing_runs("schedule", NIGHT_RUNS)?;
    nights.extend(hand);
    nights.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let mut with_jobs = Vec::new();
    let mut night_line = None;
    for run in nights {
        let jobs = match read.remove(&run.id) {
            Some(jobs) => jobs,
            None => run_jobs(&run)?,
        };
        let full = full_pass(&run, &jobs);
        if full {
            night_line = Some(night_jobs_line(&jobs)?);
        }
        with_jobs.push((run, jobs));
        if full {
            break;
        }
    }
    verdicts.push(night_verdict(&with_jobs, now)?);

    for verdict in &verdicts {
        println!("{}", verdict.line());
    }
    if let Some(line) = night_line {
        println!("{line}");
    }
    let failed = verdicts
        .iter()
        .filter(|v| matches!(v, Verdict::Fail(_)))
        .count();
    if failed > 0 {
        return Err(format!("ci-budget: {failed} over the budget (ADR-149)"));
    }
    println!("ci-budget: within the budget");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(name: &str, conclusion: &str, start: &str, end: &str) -> Job {
        Job {
            name: name.to_owned(),
            conclusion: Some(conclusion.to_owned()),
            started_at: Some(start.to_owned()),
            completed_at: Some(end.to_owned()),
        }
    }

    fn run(id: u64, event: &str, started: &str) -> Run {
        Run {
            id,
            event: event.to_owned(),
            conclusion: Some("success".to_owned()),
            created_at: started.to_owned(),
            head_sha: "0123456789abcdef".to_owned(),
        }
    }

    #[test]
    fn unix_seconds_matches_known_times() {
        assert_eq!(unix_seconds("1970-01-01T00:00:00Z"), Ok(0));
        // `date -u -d 2026-11-04T01:25:08Z +%s`
        assert_eq!(unix_seconds("2026-11-04T01:25:08Z"), Ok(1_793_755_508));
        assert_eq!(unix_seconds("2000-02-29T23:59:59Z"), Ok(951_868_799));
        assert_eq!(unix_seconds("2024-03-01T00:00:00Z"), Ok(1_709_251_200));
        for bad in [
            "2026-11-04 01:25:08Z",
            "2026-11-04T01:25:08",
            "2026-11-04T01:25:08+00:00",
            "2026-13-04T01:25:08Z",
            "2026-02-29T00:00:00Z",
            "2100-02-29T00:00:00Z",
            "2026-11-31T00:00:00Z",
            "2026-11-04T24:00:00Z",
            "2026-11-04T01:60:00Z",
            "2026-1a-04T01:25:08Z",
            "+026-11-04T01:25:08Z",
            "",
        ] {
            assert!(unix_seconds(bad).is_err(), "{bad}");
        }
    }

    const HISTORY: &str = "\
2026-11-04T10:00:00Z\taaaa\t0\t1\tPASS\t1700\tfmt=7
2026-11-04T11:00:00Z\tbbbb\t0\t1\tFAIL\t2400\tfmt=7 test=2000:FAIL
2026-11-04T12:00:00Z\tbbbb\t0\t0\tPASS\t2500\tfmt=7
2026-11-04T13:00:00Z\tcccc\t0\t1\tPASS\t1801\tfmt=7
";

    fn commits(list: &[&str]) -> Vec<String> {
        list.iter().map(|c| (*c).to_owned()).collect()
    }

    #[test]
    fn the_gate_reads_the_newest_passing_full_run_on_the_branch() {
        // On aaaa alone: 1,700 s passes.
        let v = gate_verdict(HISTORY, &commits(&["aaaa"])).unwrap();
        assert!(
            matches!(v, Verdict::Ok(ref t) if t.contains("1700 s")),
            "{v:?}"
        );
        // cccc's 1,801 s, newest, fails by one second.
        let v = gate_verdict(HISTORY, &commits(&["aaaa", "cccc"])).unwrap();
        assert!(
            matches!(v, Verdict::Fail(ref t) if t.contains("1801 s")),
            "{v:?}"
        );
        // bbbb's failed full run and passing partial run are neither counted: no run yet.
        let v = gate_verdict(HISTORY, &commits(&["bbbb"])).unwrap();
        assert!(matches!(v, Verdict::Note(_)), "{v:?}");
        // A head off the branch isn't counted.
        let v = gate_verdict(HISTORY, &commits(&["dddd"])).unwrap();
        assert!(matches!(v, Verdict::Note(_)), "{v:?}");
        // Exactly 30 minutes passes.
        let v = gate_verdict(
            "2026-11-04T10:00:00Z\teeee\t1\t1\tPASS\t1800\t\n",
            &commits(&["eeee"]),
        )
        .unwrap();
        assert!(matches!(v, Verdict::Ok(_)), "{v:?}");
        assert!(gate_verdict("x\ty\n", &commits(&["y"])).is_err());
        assert!(gate_verdict("t\taaaa\t0\t1\tPASS\tfast\n", &commits(&["aaaa"])).is_err());
        assert!(gate_verdict("t\taaaa\t0\t1\tPASS\t9\tfmt:7\n", &commits(&["aaaa"])).is_err());
    }

    /// A step run alone after the newest full run, at a later commit of the branch, replaces that
    /// step's time in it: a full run of 600 s and a later validate alone of 7,867 s make 8,267 s.
    /// A failed step's reading isn't one, and a step the prose rule left out is added.
    #[test]
    fn a_step_run_alone_after_the_full_run_counts() {
        let history = "\
t1\taaaa\t0\t1\tPASS\t600\tfmt=7 test=150 validate=200
t2\tbbbb\t0\t0\tPASS\t7870\tvalidate=7867
";
        let v = gate_verdict(history, &commits(&["aaaa", "bbbb"])).unwrap();
        let Verdict::Fail(text) = v else {
            panic!("{v:?}")
        };
        assert!(text.contains("took 600 s"), "{text}");
        assert!(text.contains("validate 7867 s at bbbb"), "{text}");
        assert!(text.contains("8267 s"), "{text}");
        // The later run off the branch isn't counted.
        let v = gate_verdict(history, &commits(&["aaaa"])).unwrap();
        assert!(
            matches!(v, Verdict::Ok(ref t) if !t.contains("newer")),
            "{v:?}"
        );
        // A failed step's time is no reading of it.
        let failed = "\
t1\taaaa\t0\t1\tPASS\t600\tfmt=7 test=150 validate=200
t2\tbbbb\t0\t0\tFAIL\t7870\tvalidate=7867:FAIL
";
        let v = gate_verdict(failed, &commits(&["aaaa", "bbbb"])).unwrap();
        assert!(matches!(v, Verdict::Ok(_)), "{v:?}");
        // The newest reading wins: validate 7,867 s, then 250 s.
        let again = format!("{history}t3\tbbbb\t0\t0\tPASS\t250\tvalidate=250\n");
        let v = gate_verdict(&again, &commits(&["aaaa", "bbbb"])).unwrap();
        assert!(
            matches!(v, Verdict::Ok(ref t) if t.contains("650 s")),
            "{v:?}"
        );
        // validate left out by the prose rule, then run: added, and the line says so.
        let left = "\
t1\taaaa\t0\t1\tPASS\t400\tfmt=7 test=150 validate-left
t2\tbbbb\t0\t0\tPASS\t1500\tvalidate=1500
";
        let v = gate_verdict(left, &commits(&["aaaa", "bbbb"])).unwrap();
        let Verdict::Fail(text) = v else {
            panic!("{v:?}")
        };
        assert!(text.contains("validate left out"), "{text}");
        assert!(text.contains("1900 s"), "{text}");
        // The site left out by the page-prose rule: read, not refused as a step with no time.
        let site = "t1\taaaa\t0\t1\tPASS\t300\tfmt=7 pages=80 site-left validate-left\n";
        let v = gate_verdict(site, &commits(&["aaaa"])).unwrap();
        let Verdict::Ok(text) = v else {
            panic!("{v:?}")
        };
        assert!(text.contains("the site left out"), "{text}");
        assert!(text.contains("validate left out"), "{text}");
        assert!(text.contains("300 s"), "{text}");
    }

    #[test]
    fn a_job_over_twenty_minutes_fails_and_a_wait_for_a_runner_is_apart() {
        let r = run(7, "push", "2026-11-04T13:00:00Z");
        // Queued an hour (not counted), then 20 minutes on its runner: passes.
        let mut jobs = vec![
            job(
                "site (2/3)",
                "success",
                "2026-11-04T14:00:00Z",
                "2026-11-04T14:20:00Z",
            ),
            job(
                "fmt",
                "success",
                "2026-11-04T13:00:00Z",
                "2026-11-04T13:01:00Z",
            ),
        ];
        let v = jobs_verdict(&r, &jobs).unwrap();
        assert!(
            matches!(v, Verdict::Ok(ref t) if t.contains("`site (2/3)` 1200 s")),
            "{v:?}"
        );
        jobs.push(job(
            "test (windows-latest, 1/2)",
            "success",
            "2026-11-04T13:00:00Z",
            "2026-11-04T13:20:01Z",
        ));
        let v = jobs_verdict(&r, &jobs).unwrap();
        let Verdict::Fail(text) = v else {
            panic!("{v:?}")
        };
        assert!(
            text.contains("`test (windows-latest, 1/2)` 1201 s"),
            "{text}"
        );
        assert!(!text.contains("budget: `site"), "{text}");
        // A job that never started is left out; a run with none started is refused.
        let unstarted = Job {
            name: "deploy".to_owned(),
            conclusion: Some("skipped".to_owned()),
            started_at: None,
            completed_at: None,
        };
        assert!(jobs_verdict(&r, &[unstarted]).is_err());
    }

    #[test]
    fn a_pull_requests_jobs_ran_types_and_no_census() {
        let t = "2026-11-04T13:00:00Z";
        let types = |c: &str| job("types", c, t, t);
        let census = |c: &str| job("census (windows-latest)", c, t, t);
        assert!(pull_request_jobs(&[types("success")]));
        assert!(pull_request_jobs(&[types("success"), census("skipped")]));
        // Research notes alone: `types` skipped.
        assert!(!pull_request_jobs(&[types("skipped"), census("skipped")]));
        // A full run: a census ran.
        assert!(!pull_request_jobs(&[types("success"), census("success")]));
        // No `types` job at all is no pull request's run.
        assert!(!pull_request_jobs(&[job("wasm-check", "success", t, t)]));
    }

    fn night_jobs(census: &str, fit: &str) -> Vec<Job> {
        let (t0, t1) = ("2026-11-04T08:20:00Z", "2026-11-04T08:50:00Z");
        vec![
            job("census (ubuntu-latest)", "success", t0, t1),
            job("census (windows-latest)", census, t0, t1),
            job("fit intervals (1/8)", "success", t0, t1),
            job("fit intervals", fit, t0, t1),
        ]
    }

    #[test]
    fn the_night_must_pass_whole_within_twenty_six_hours() {
        let now = unix_seconds("2026-11-05T10:17:00Z").unwrap();
        let night = run(1, "schedule", "2026-11-04T08:17:00Z");
        // 26 h exactly passes; a second past fails.
        let v = night_verdict(&[(night.clone(), night_jobs("success", "success"))], now).unwrap();
        assert!(matches!(v, Verdict::Ok(_)), "{v:?}");
        let v = night_verdict(
            &[(night.clone(), night_jobs("success", "success"))],
            now + 1,
        )
        .unwrap();
        assert!(matches!(v, Verdict::Fail(_)), "{v:?}");
        // A run whose census or fit check skipped is no full run: the older one is read.
        let older = run(0, "workflow_dispatch", "2026-11-03T08:17:00Z");
        let runs = [
            (night.clone(), night_jobs("skipped", "success")),
            (older.clone(), night_jobs("success", "success")),
        ];
        let v = night_verdict(&runs, now).unwrap();
        assert!(
            matches!(v, Verdict::Fail(ref t) if t.contains("run, 0 ")),
            "{v:?}"
        );
        let runs = [(night.clone(), night_jobs("success", "skipped"))];
        assert!(matches!(
            night_verdict(&runs, now).unwrap(),
            Verdict::Fail(_)
        ));
        // A run with no census job at all is no full run.
        let mut jobs = night_jobs("success", "success");
        jobs.retain(|job| !job.name.starts_with("census"));
        assert!(matches!(
            night_verdict(&[(night.clone(), jobs)], now).unwrap(),
            Verdict::Fail(_)
        ));
        // None read fails.
        assert!(matches!(night_verdict(&[], now).unwrap(), Verdict::Fail(_)));
    }

    #[test]
    fn the_api_fields_read() {
        let runs: Runs = serde_json::from_str(
            r#"{"total_count":1,"workflow_runs":[{"id":41753918203,"event":"workflow_dispatch",
            "conclusion":"success","created_at":"2026-11-04T01:25:08Z","head_sha":"abc",
            "run_started_at":"2026-11-05T01:25:08Z",
            "status":"completed"}]}"#,
        )
        .unwrap();
        assert_eq!(runs.workflow_runs[0].id, 41_753_918_203);
        // The first attempt's time, not a re-run's.
        assert_eq!(runs.workflow_runs[0].created_at, "2026-11-04T01:25:08Z");
        let jobs: Jobs = serde_json::from_str(
            r#"{"total_count":1,"jobs":[{"name":"deploy","conclusion":null,"started_at":null,
            "completed_at":null,"status":"queued"}]}"#,
        )
        .unwrap();
        assert_eq!(job_seconds(&jobs.jobs[0]), Ok(None));
    }

    /// The names the check reads are `ci.yml`'s: `types` skips on a change to research notes
    /// alone, a `census` job runs on every platform, and `fit intervals` checks the fit's parts.
    #[test]
    fn the_job_names_read_are_ci_yml_s() {
        let root = crate::designs::root().unwrap();
        let ci = std::fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap();
        let ci = ci.replace("\r\n", "\n");
        let types = &ci[ci.find("\n  types:\n").unwrap()..];
        let types = &types[..types[1..].find("\n  # ").unwrap_or(types.len() - 1)];
        assert!(types.contains("    name: types\n"), "{types}");
        assert!(
            types.contains("    if: needs.scope.outputs.notes != 'true'\n"),
            "{types}"
        );
        assert!(ci.contains("    name: census (${{ matrix.os }})\n"));
        assert!(ci.contains("    name: fit intervals\n"));
        assert!(ci.contains("  push:\n    branches: [main]\n"));
        assert!(ci.contains("  schedule:\n"));
    }
}
