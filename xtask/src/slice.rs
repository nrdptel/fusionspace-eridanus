//! A flight census's slice (M0.16a1, ADR-149 amended): a fixed part of its flights, each written
//! down whole, so that a pull request and `scripts/gate.sh` can check that part exactly while the
//! whole census runs each night.
//!
//! A census's slice is every flight of each of its sets whose index is a multiple of the census's
//! `every`: flights 0, `every`, 2·`every` and on. The command that writes the census writes the
//! slice beside it, as `<report>.slice.json`: for each flight in it, one line for each thing the
//! census makes of that flight, its numbers to the digits the platforms share (the census's three
//! significant figures; an error as a share of its bound, or of its standard deviation, to
//! three decimals). A census's `--slice` flies only those flights, writes their lines again and
//! compares them with the committed file, naming each flight that differs; it holds each flight to
//! the rules the census holds every flight to (a reading within its bound, say), and leaves to the
//! nightly run what only the whole set can show (a coverage test, a range). The whole census's
//! `--check` makes the slice too and compares it, so the committed slice is always the whole
//! run's.
//!
//! A flight is the same however many flights came before it on its thread (each census's tests),
//! so its lines in the slice are its lines in the whole run.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// One flight of a slice and what the census makes of it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flight {
    /// Its set's name.
    pub set: String,
    /// Its index in the set.
    pub index: u64,
    /// One line for each thing the census makes of it, in the census's order.
    pub made: Vec<String>,
}

/// A census's slice as it is committed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slice {
    /// What it is.
    pub about: String,
    /// The slice holds each set's flights whose index is a multiple of this.
    pub every: u64,
    /// Its flights, set by set in the census's order, each set's in index order.
    pub flights: Vec<Flight>,
}

/// The indices of the first `count` flights that a slice taking every `every`th holds.
pub fn indices(count: u64, every: u64) -> Vec<u64> {
    // `every` is a constant of each census, never zero (its tests).
    (0..count).step_by(every.max(1) as usize).collect()
}

/// The threads to share flights among, as every census does.
pub fn threads() -> usize {
    std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16)
}

/// Runs `each` on every job, each thread taking every `n`th job with a state of its own made by
/// `state`, and gives the results in the jobs' order. A job that panics is an error naming it,
/// and its thread starts a new state, as a panic may leave the old one half-built.
pub fn on_cores<J, S, T>(
    jobs: &[J],
    name: impl Fn(&J) -> String + Sync,
    state: impl Fn() -> S + Sync,
    each: impl Fn(&mut S, &J) -> Result<T, String> + Sync,
) -> Result<Vec<T>, String>
where
    J: Sync,
    T: Send,
{
    let threads = threads();
    let (name, state, each) = (&name, &state, &each);
    let mut done: Vec<(usize, Result<T, String>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|start| {
                scope.spawn(move || {
                    let mut held = state();
                    let mut done = Vec::new();
                    for at in (start..jobs.len()).step_by(threads) {
                        let job = &jobs[at];
                        let made = panic::catch_unwind(AssertUnwindSafe(|| each(&mut held, job)))
                            .unwrap_or_else(|payload| {
                                held = state();
                                Err(format!("it panicked: {}", panic_message(&*payload)))
                            })
                            .map_err(|error| format!("{}: {error}", name(job)));
                        done.push((at, made));
                    }
                    done
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| {
                handle.join().unwrap_or_else(|_| {
                    // Every job's panic is caught above; this is a panic in the loop itself.
                    vec![(usize::MAX, Err("a thread panicked".to_owned()))]
                })
            })
            .collect()
    });
    done.sort_by_key(|(at, _)| *at);
    let mut problems = Vec::new();
    let mut made = Vec::new();
    for (_, result) in done {
        match result {
            Ok(value) => made.push(value),
            Err(problem) => problems.push(problem),
        }
    }
    if problems.is_empty() && made.len() == jobs.len() {
        Ok(made)
    } else {
        problems.sort();
        Err(problems.join("\n"))
    }
}

/// A panic's message.
pub fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no message".to_owned())
}

/// `x` to three significant figures, as the censuses keep it, with no sign on a zero.
pub fn figures(x: f64) -> String {
    let rounded = hpr_forensics::synthetic::intervals::three_figures(x);
    if rounded == 0.0 {
        "0".to_owned()
    } else {
        format!("{rounded}")
    }
}

/// A share (an error over its bound or its standard deviation) to three decimals, with no sign
/// on a zero. A share is kept to a fixed step, not to significant figures, so that an error near
/// zero isn't written to digits that are only its rounding.
pub fn share(x: f64) -> String {
    if !x.is_finite() {
        return format!("{x}");
    }
    let rounded = (x * 1_000.0).round() / 1_000.0;
    if rounded == 0.0 {
        "0".to_owned()
    } else {
        format!("{rounded:.3}")
    }
}

/// The slice's JSON as it is written, with its last newline.
pub fn json(slice: &Slice) -> Result<String, String> {
    serde_json::to_string_pretty(slice)
        .map(|json| json + "\n")
        .map_err(|error| error.to_string())
}

/// The most flights a failed comparison names: the rest are counted.
const FLIGHTS_SHOWN: usize = 20;

/// How a slice is held to the committed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// Byte for byte: on the build that writes the reports, the Mac's everyday build, which
    /// `scripts/gate.sh` runs before each merge.
    Exact,
    /// Every word of every line, its numbers aside: on any other platform or build. A flight's
    /// numbers can move there: set A's flight 399, whose simulated apogee moves 1.2e-6 of itself
    /// between the everyday and the optimized build on one Mac, has its smoother's shares move by
    /// up to 0.087 and a count of its unweighed GNSS fixes by 69 (ADR-149, amended 2026-11-04). Its
    /// words didn't, in the builds and platforms compared: the side each truth falls on, what was
    /// withheld and why, the smoother's flags, the design, the logger and the model. Those are what
    /// the census counts as outcomes; the nightly run holds the report's tallies to the digits the
    /// platforms share, and the slice's flights in words.
    Words,
}

/// `line` with each number in it written as `#`: what [`Held::Words`] compares. NaN and infinity
/// stay words: a share that stops being a number is a change, not a build's rounding.
pub fn words(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut at = 0;
    let starts_word = |at: usize| at == 0 || !chars[at - 1].is_alphanumeric();
    // A digit after a hyphen or underscore joined to a name is part of the name: `imu-500hz`.
    let in_name = |at: usize| {
        at >= 2 && matches!(chars[at - 1], '-' | '_') && chars[at - 2].is_alphanumeric()
    };
    while at < chars.len() {
        let c = chars[at];
        let signed = c == '-' && starts_word(at);
        let digit_at = |i: usize| chars.get(i).is_some_and(char::is_ascii_digit);
        if (c.is_ascii_digit() && starts_word(at) && !in_name(at)) || (signed && digit_at(at + 1)) {
            let start = at;
            at += 1;
            while at < chars.len() {
                let next = chars[at];
                let exponent = (next == 'e' || next == 'E')
                    && (digit_at(at + 1)
                        || (matches!(chars.get(at + 1), Some('-' | '+')) && digit_at(at + 2)));
                if next.is_ascii_digit() || (next == '.' && digit_at(at + 1)) {
                    at += 1;
                } else if exponent {
                    at += if digit_at(at + 1) { 2 } else { 3 };
                } else {
                    break;
                }
            }
            // Digits run into letters are a name too: `500hz`.
            if chars.get(at).is_some_and(|c| c.is_alphanumeric()) {
                out.extend(&chars[start..at]);
            } else {
                out.push('#');
            }
        } else {
            out.push(c);
            at += 1;
        }
    }
    out
}

/// How a passing comparison was held, for its message.
pub fn held_text(held: Held) -> &'static str {
    match held {
        Held::Exact => "byte for byte",
        Held::Words => "in every word (its numbers aside)",
    }
}

/// `slice` with every line as [`words`] gives it.
fn in_words(slice: &Slice) -> Slice {
    Slice {
        about: slice.about.clone(),
        every: slice.every,
        flights: slice
            .flights
            .iter()
            .map(|f| Flight {
                set: f.set.clone(),
                index: f.index,
                made: f.made.iter().map(|line| words(line)).collect(),
            })
            .collect(),
    }
}

/// What differs between the committed slice at `path` and `made`, held as `held` says: each
/// flight that differs, with its first differing line, a flight in one and not the other, or a
/// different `every`. None when the committed file is what `made` writes, byte for byte or in
/// its words.
pub fn compare(path: &Path, made: &Slice, held: Held) -> Option<String> {
    let made_json = match json(made) {
        Ok(json) => json,
        Err(error) => return Some(format!("{}: {error}", path.display())),
    };
    let text = match fs::read_to_string(path) {
        Ok(text) => text.replace("\r\n", "\n"),
        Err(error) => return Some(format!("{}: {error}", path.display())),
    };
    if text == made_json {
        return None;
    }
    let committed: Slice = match serde_json::from_str(&text) {
        Ok(committed) => committed,
        Err(error) => return Some(format!("{}: {error}", path.display())),
    };
    match held {
        Held::Exact => Some(differences(path, &committed, made)),
        Held::Words => {
            // Its layout is held by the exact check on the build that writes it.
            let (committed, made) = (in_words(&committed), in_words(made));
            (committed != made).then(|| differences(path, &committed, &made))
        }
    }
}

/// [`compare`] for a part of a slice, in words: `made` against the committed flights whose set
/// and index are in `places`. A place with no committed flight, a committed or made flight off
/// the places, or a flight that differs in its words, is named.
pub fn compare_part(path: &Path, made: &Slice, places: &BTreeSet<(String, u64)>) -> Option<String> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text.replace("\r\n", "\n"),
        Err(error) => return Some(format!("{}: {error}", path.display())),
    };
    let mut committed: Slice = match serde_json::from_str(&text) {
        Ok(committed) => committed,
        Err(error) => return Some(format!("{}: {error}", path.display())),
    };
    committed
        .flights
        .retain(|f| places.contains(&(f.set.clone(), f.index)));
    let (committed, made) = (in_words(&committed), in_words(made));
    let off: Vec<String> = made
        .flights
        .iter()
        .filter(|f| !places.contains(&(f.set.clone(), f.index)))
        .map(|f| format!("{} flight {}: made, but not in this part", f.set, f.index))
        .collect();
    if !off.is_empty() {
        return Some(format!("{}: {}", path.display(), off.join("; ")));
    }
    (committed != made).then(|| differences(path, &committed, &made))
}

/// The differences between two slices, as [`compare`] names them.
fn differences(path: &Path, committed: &Slice, made: &Slice) -> String {
    let mut problems = Vec::new();
    if committed.every != made.every {
        problems.push(format!(
            "it steps {} flights at a time, the census {}",
            committed.every, made.every
        ));
    }
    if committed.about != made.about {
        problems.push("its `about` differs".to_owned());
    }
    let key = |f: &Flight| (f.set.clone(), f.index);
    for flight in &made.flights {
        match committed.flights.iter().find(|c| key(c) == key(flight)) {
            None => problems.push(format!("{} flight {}: not in it", flight.set, flight.index)),
            Some(old) if old.made != flight.made => {
                let at = old
                    .made
                    .iter()
                    .zip(&flight.made)
                    .position(|(a, b)| a != b)
                    .unwrap_or(old.made.len().min(flight.made.len()));
                problems.push(format!(
                    "{} flight {} differs: committed `{}`, made `{}`",
                    flight.set,
                    flight.index,
                    old.made.get(at).map_or("(nothing)", String::as_str),
                    flight.made.get(at).map_or("(nothing)", String::as_str)
                ));
            }
            Some(_) => {}
        }
    }
    for flight in &committed.flights {
        if !made.flights.iter().any(|m| key(m) == key(flight)) {
            problems.push(format!(
                "{} flight {}: committed, but not in the slice",
                flight.set, flight.index
            ));
        }
    }
    if problems.is_empty() {
        // Every flight agrees, so the order or the layout differs.
        problems.push("its flights are in another order, or laid out otherwise".to_owned());
    }
    let total = problems.len();
    let mut text = format!(
        "{} isn't what the slice's flights make ({total} differences):",
        path.display()
    );
    for problem in problems.iter().take(FLIGHTS_SHOWN) {
        let _ = write!(text, "\n  {problem}");
    }
    if total > FLIGHTS_SHOWN {
        let _ = write!(text, "\n  and {} more", total - FLIGHTS_SHOWN);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slice() -> Slice {
        Slice {
            about: "a slice".to_owned(),
            every: 20,
            flights: vec![
                Flight {
                    set: "a".to_owned(),
                    index: 0,
                    made: vec![
                        "census: apogee 1230 m".to_owned(),
                        "smoother: 0.5".to_owned(),
                    ],
                },
                Flight {
                    set: "a".to_owned(),
                    index: 20,
                    made: vec![
                        "census: apogee 2100 m".to_owned(),
                        "smoother: -0.1".to_owned(),
                    ],
                },
            ],
        }
    }

    fn written(slice: &Slice) -> tempfile_path::Path {
        tempfile_path::Path::with(&json(slice).unwrap())
    }

    /// A file of its own under the target directory, removed when dropped.
    mod tempfile_path {
        pub struct Path(pub std::path::PathBuf);
        impl Path {
            pub fn with(text: &str) -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static NEXT: AtomicU64 = AtomicU64::new(0);
                let dir = std::env::temp_dir().join(format!(
                    "hpr-slice-test-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                std::fs::create_dir_all(&dir).unwrap();
                let path = dir.join("slice.json");
                std::fs::write(&path, text).unwrap();
                Self(path)
            }
        }
        impl Drop for Path {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(self.0.parent().unwrap());
            }
        }
    }

    #[test]
    fn a_slice_takes_every_nth_flight_from_the_first() {
        assert_eq!(indices(100, 20), vec![0, 20, 40, 60, 80]);
        assert_eq!(indices(101, 20), vec![0, 20, 40, 60, 80, 100]);
        assert_eq!(indices(0, 20), Vec::<u64>::new());
    }

    #[test]
    fn the_same_slice_compares_equal_and_crlf_is_read_as_lf() {
        let made = slice();
        assert_eq!(compare(&written(&made).0, &made, Held::Exact), None);
        let crlf = tempfile_path::Path::with(&json(&made).unwrap().replace('\n', "\r\n"));
        assert_eq!(compare(&crlf.0, &made, Held::Exact), None);
    }

    #[test]
    fn one_changed_flight_fails_and_is_named() {
        let committed = slice();
        let file = written(&committed);
        let mut made = committed.clone();
        made.flights[1].made[1] = "smoother: -0.2".to_owned();
        let problem = compare(&file.0, &made, Held::Exact).unwrap();
        assert!(problem.contains("(1 differences)"), "{problem}");
        assert!(
            problem
                .contains("a flight 20 differs: committed `smoother: -0.1`, made `smoother: -0.2`"),
            "{problem}"
        );
        assert!(!problem.contains("flight 0"), "{problem}");
    }

    #[test]
    fn a_flight_missing_added_or_moved_fails() {
        let committed = slice();
        let file = written(&committed);
        let mut fewer = committed.clone();
        fewer.flights.pop();
        assert!(
            compare(&file.0, &fewer, Held::Exact)
                .unwrap()
                .contains("a flight 20: committed, but not in the slice")
        );
        let mut more = committed.clone();
        more.flights.push(Flight {
            set: "b".to_owned(),
            index: 0,
            made: vec![],
        });
        assert!(
            compare(&file.0, &more, Held::Exact)
                .unwrap()
                .contains("b flight 0: not in it")
        );
        let mut moved = committed.clone();
        moved.flights.reverse();
        assert!(
            compare(&file.0, &moved, Held::Exact)
                .unwrap()
                .contains("another order")
        );
        let mut every = committed.clone();
        every.every = 10;
        assert!(
            compare(&file.0, &every, Held::Exact)
                .unwrap()
                .contains("it steps 20 flights at a time, the census 10")
        );
    }

    #[test]
    fn a_line_added_to_a_flight_fails() {
        let committed = slice();
        let file = written(&committed);
        let mut made = committed.clone();
        made.flights[0].made.push("intervals: inside".to_owned());
        let problem = compare(&file.0, &made, Held::Exact).unwrap();
        assert!(
            problem.contains("a flight 0 differs: committed `(nothing)`, made `intervals: inside`"),
            "{problem}"
        );
    }

    #[test]
    fn a_missing_or_damaged_file_fails() {
        let made = slice();
        let missing = std::env::temp_dir().join("hpr-slice-test-none/slice.json");
        assert!(compare(&missing, &made, Held::Exact).is_some());
        let damaged = tempfile_path::Path::with("{ not json");
        assert!(compare(&damaged.0, &made, Held::Exact).is_some());
    }

    #[test]
    fn numbers_keep_the_digits_the_platforms_share() {
        assert_eq!(figures(1234.5), "1230");
        assert_eq!(figures(0.012_345), "0.0123");
        assert_eq!(figures(-0.0), "0");
        assert_eq!(share(0.123_49), "0.123");
        assert_eq!(share(-0.000_4), "0");
        assert_eq!(share(1e-17), "0");
        assert_eq!(share(-1.5), "-1.500");
        assert_eq!(share(f64::NAN), "NaN");
    }

    #[test]
    fn jobs_come_back_in_order_and_a_panic_is_named() {
        let jobs: Vec<u64> = (0..50).collect();
        let made = on_cores(
            &jobs,
            |j| format!("job {j}"),
            || 0u64,
            |n, j| {
                *n += 1;
                Ok(j * 2)
            },
        )
        .unwrap();
        assert_eq!(made, (0..50).map(|j| j * 2).collect::<Vec<_>>());
        let error = on_cores(
            &jobs,
            |j| format!("job {j}"),
            || (),
            |(), j| {
                assert!(*j != 7, "seven");
                if *j == 9 {
                    Err("nine".to_owned())
                } else {
                    Ok(*j)
                }
            },
        )
        .unwrap_err();
        assert!(error.contains("job 7: it panicked: seven"), "{error}");
        assert!(error.contains("job 9: nine"), "{error}");
    }

    #[test]
    fn words_masks_every_number_and_keeps_every_word() {
        assert_eq!(
            words("smoother: altos, gnss yes; apogee height -0.530 inside; burnout 1.2e-5 above"),
            "smoother: altos, gnss yes; apogee height # inside; burnout # above"
        );
        assert_eq!(
            words("rrc3: 4585 samples, highest 3970 m"),
            "rrc3: # samples, highest # m"
        );
        // NaN and infinity aren't a number that moved, so they stay.
        assert_eq!(
            words("share NaN, -inf and inf; inside"),
            "share NaN, -inf and inf; inside"
        );
        // A number inside a name stays: a logger, a design, a reason.
        assert_eq!(
            words("imu-500hz, rocketpy-ndrt-2020"),
            "imu-500hz, rocketpy-ndrt-2020"
        );
        assert_eq!(words("20 Hz in 10 Pa steps"), "# Hz in # Pa steps");
    }

    #[test]
    fn held_in_words_a_number_may_move_but_no_word() {
        let committed = slice();
        let file = written(&committed);
        let mut moved = committed.clone();
        moved.flights[1].made[1] = "smoother: -0.103".to_owned();
        moved.flights[0].made[0] = "census: apogee 1240 m".to_owned();
        assert_eq!(compare(&file.0, &moved, Held::Words), None);
        assert!(compare(&file.0, &moved, Held::Exact).is_some());
        let mut side = committed.clone();
        side.flights[1].made[1] = "smoother: -0.1 above".to_owned();
        let problem = compare(&file.0, &side, Held::Words).unwrap();
        assert!(problem.contains("a flight 20 differs"), "{problem}");
        let mut fewer = committed.clone();
        fewer.flights.pop();
        assert!(compare(&file.0, &fewer, Held::Words).is_some());
        let mut every = committed.clone();
        every.every = 21;
        assert!(compare(&file.0, &every, Held::Words).is_some());
        // A number that turns NaN or infinite fails in words too.
        for gone in ["NaN", "inf", "-inf"] {
            let mut lost = committed.clone();
            lost.flights[0].made[0] = format!("census: apogee {gone} m");
            let problem = compare(&file.0, &lost, Held::Words).unwrap();
            assert!(problem.contains("a flight 0 differs"), "{gone}: {problem}");
        }
    }

    #[test]
    fn an_unknown_field_fails() {
        let made = slice();
        let text = json(&made)
            .unwrap()
            .replacen("\"every\"", "\"extra\": 1,\n  \"every\"", 1);
        let file = tempfile_path::Path::with(&text);
        let problem = compare(&file.0, &made, Held::Exact).unwrap();
        assert!(problem.contains("unknown field"), "{problem}");
    }
}
