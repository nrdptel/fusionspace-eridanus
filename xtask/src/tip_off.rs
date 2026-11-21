//! What RocketPy's tip-off phase moves on its example rockets (M1.14b5), written by
//! `cargo xtask tip-off` to `validation/reports/tip-off.json` and `.md`.
//!
//! - **The question.** HPR Sim guides a rocket until its last guide leaves the rail and frees it
//!   with no turning rate. A real rocket pivots about its last guide once the one ahead is off
//!   the rail: tip-off. This report measures what that pivot is worth, with RocketPy's own
//!   tip-off phase, so the flight page can say what leaving it out costs.
//! - **The oracle** is `validation/oracles/rocketpy/tip_off.py`: RocketPy's `develop` at the
//!   merge of its pull request #1217, which rewrote the phase #920 added (pinned in
//!   `validation/refs.lock.toml` as `rocketpy-tip-off`; M1.14b13, ADR-363), flying each example
//!   rocket with the phase off and on, in still air and
//!   in a 20 mph crosswind, and a third time guided to where the lower button leaves, which is
//!   HPR Sim's release to within half a button's length (HPR Sim waits for the button's aft
//!   edge). Its docstring states what it declares.
//! - **The bars** are the plan's: a tip-off model is called for when the phase moves any apogee
//!   by more than 1%, or any landing point by more than 1% of the apogee
//!   ([`APOGEE_BAR_PERCENT`], [`LANDING_BAR_PERCENT_OF_APOGEE`]).
//! - **One run and a probe.** The run flies the commit's equations as they are: the commit
//!   carries RocketPy's pull request #1196, which every whole flight of the suite flies. The
//!   probe flies, with its parachute's lag, the one rocket and wind the run flies without it,
//!   and the report keeps that it had not finished in the probe's wall time.
//! - **The earlier pin.** The report first flew the merge of #920, with #1196 applied here.
//!   That report is kept byte for byte ([`EARLIER`]), and this one says how far each flight's
//!   figures moved between the two pins ([`since_the_earlier_pin`]).

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub const USAGE: &str = "\
  tip-off [--check|--page] Fly RocketPy's example rockets with its tip-off phase off and on
                           (validation/oracles/rocketpy/tip_off.py, on the checkout pinned as
                           rocketpy-tip-off under refs/) and write what the phase moves to
                           validation/reports/tip-off.json and .md. --check fails if the
                           committed report differs instead of writing. --page writes the .md
                           from the committed .json alone, with no flight, for a change to
                           the page's words.";

const ORACLE: &str = "validation/oracles/rocketpy/tip_off.py";
const REPORT: &str = "validation/reports/tip-off.json";
const REPORT_MD: &str = "validation/reports/tip-off.md";
/// The checkout's name in `validation/refs.lock.toml`.
const LOCK_NAME: &str = "rocketpy-tip-off";

/// The report as the earlier pin gave it: RocketPy's `develop` at the merge of its pull request
/// #920 (`ebe1b775`), flown with pull request #1196's correction applied by the oracle. It is
/// this repository's `validation/reports/tip-off.json` at [`EARLIER_COMMIT`], byte for byte
/// ([`EARLIER_SHA256`]): `git show <commit>:validation/reports/tip-off.json` gives the same file.
const EARLIER: &str = "validation/fixtures/flight/rocketpy-tip-off-at-pull-920.json";
/// The last commit of this repository whose report flew the earlier pin as [`EARLIER`] has it.
const EARLIER_COMMIT: &str = "ee9a5af6a175a850242f79bda3e272e27c6fe3d3";
/// [`EARLIER`]'s SHA-256.
const EARLIER_SHA256: &str = "af8467df095acab240ef3cb2e19c4fa5154cace5cb09abae79a073521f3daa78";
/// The pull request that rewrote RocketPy's phase between the two pins.
const REWRITE: &str = "https://github.com/RocketPy-Team/RocketPy/pull/1217";

/// A tip-off model is called for when the phase moves any apogee by more than this, percent.
const APOGEE_BAR_PERCENT: f64 = 1.0;
/// Or any landing point by more than this, percent of the apogee.
const LANDING_BAR_PERCENT_OF_APOGEE: f64 = 1.0;

/// One run of the oracle, with `flag` if given, its printed document parsed.
fn oracle(root: &Path, flag: Option<&str>) -> Result<Value, String> {
    let mut command = crate::refs::oracle_python(root)?;
    command.current_dir(root).arg(ORACLE);
    if let Some(flag) = flag {
        command.arg(flag);
    }
    let output = command.output().map_err(|e| format!("{ORACLE}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "{ORACLE} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("{ORACLE}'s output: {e}"))
}

pub(crate) fn number(value: &Value, key: &str) -> Result<f64, String> {
    value[key]
        .as_f64()
        .filter(|x| x.is_finite())
        .ok_or_else(|| format!("no number `{key}` in {value}"))
}

pub(crate) fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("no text `{key}` in {value}"))
}

pub(crate) fn list<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    value[key]
        .as_array()
        .ok_or_else(|| format!("no list `{key}` in {value}"))
}

/// Each rocket and wind of a run with its moves under `key`: the rocket, the wind, the move.
fn rows<'a>(flown: &'a Value, key: &str) -> Result<Vec<(&'a str, &'a str, &'a Value)>, String> {
    let mut rows = Vec::new();
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            rows.push((text(case, "name")?, text(wind, "wind")?, &wind[key]));
        }
    }
    Ok(rows)
}

/// The rows of a run whose move under `key` is past a bar, and the designs they belong to. A
/// move exactly at a bar is not past it.
fn past_the_bars(flown: &Value, key: &str) -> Result<Value, String> {
    let (mut apogee, mut landing, mut rockets) = (Vec::new(), Vec::new(), Vec::<&str>::new());
    let (mut total, mut farther, mut nearer) = (0, 0, 0);
    for (name, wind, moved) in rows(flown, key)? {
        total += 1;
        let apogee_percent = number(moved, "apogee_percent")?;
        let mut past = false;
        if apogee_percent.abs() > APOGEE_BAR_PERCENT {
            apogee.push(json!({"case": name, "wind": wind, "apogee_percent": apogee_percent}));
            past = true;
        }
        let range = number(moved, "landing_range_m")?;
        farther += usize::from(range > 0.0);
        nearer += usize::from(range < 0.0);
        let percent = number(moved, "landing_point_percent_of_apogee")?;
        if percent > LANDING_BAR_PERCENT_OF_APOGEE {
            landing.push(json!({
                "case": name,
                "wind": wind,
                "landing_point_percent_of_apogee": percent,
                "landing_point_m": number(moved, "landing_point_m")?,
            }));
            past = true;
        }
        if past && !rockets.contains(&name) {
            rockets.push(name);
        }
    }
    Ok(json!({
        "rows": total,
        "landing_farther_from_the_pad": farther,
        "landing_nearer_the_pad": nearer,
        "apogee_past_the_bar": apogee,
        "landing_point_past_the_bar": landing,
        "designs_past_a_bar": rockets,
    }))
}

/// A run's reading against the bars: the phase on less off, which is the plan's rule, and on
/// less the flight guided to the lower button, which is what HPR Sim's release leaves out.
fn reading(flown: &Value) -> Result<Value, String> {
    let moved = past_the_bars(flown, "moved")?;
    let model_called_for = !list(&moved, "designs_past_a_bar")?.is_empty();
    Ok(json!({
        "model_called_for": model_called_for,
        "moved": moved,
        "moved_from_guided": past_the_bars(flown, "moved_from_guided")?,
    }))
}

/// The largest difference between two documents' numbers, which must sit in the same places:
/// an object's keys, a list's length and every value that is not a number have to be the same
/// in both.
fn largest_difference(a: &Value, b: &Value) -> Result<f64, String> {
    match (a, b) {
        (Value::Number(_), Value::Number(_)) => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Ok((x - y).abs()),
            _ => Err(format!("{a} or {b} is not a finite number")),
        },
        (Value::Object(x), Value::Object(y)) => {
            if x.len() != y.len() {
                return Err(format!("{a} and {b} differ in their keys"));
            }
            x.iter().try_fold(0.0_f64, |largest, (key, value)| {
                let other = y.get(key).ok_or_else(|| format!("no `{key}` in {b}"))?;
                Ok(largest.max(largest_difference(value, other)?))
            })
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                return Err(format!("{a} and {b} differ in length"));
            }
            x.iter()
                .zip(y)
                .try_fold(0.0_f64, |largest, (value, other)| {
                    Ok(largest.max(largest_difference(value, other)?))
                })
        }
        _ if a == b => Ok(0.0),
        _ => Err(format!("{a} is not {b}")),
    }
}

/// The three flights of a rocket and wind, as the report names them and as the oracle keys them.
const FLIGHTS: [(&str, &str); 3] = [
    ("phase off", "phase_off"),
    ("phase on", "phase_on"),
    ("guided", "guided_to_lower_button"),
];

/// How far one rocket and wind's figures moved from the earlier pin's (`old`) to this run's
/// (`new`), in the four figures M1.14b9's bars are on (ADR-357 decision 4) and in the flights
/// themselves.
///
/// - The turning rate and the angle off the rail's axis where the lower button leaves, phase
///   on: each the length of the difference of its nose-down and nose-right parts.
/// - What the phase moves from the guided flight: the apogee's move, and the landing point's as
///   a vector, each pin's taken against its own guided flight, their difference as a share of
///   this run's guided apogee.
/// - Each flight's own apogee, and the distance between its two landing points.
fn flight_since(name: &str, old: &Value, new: &Value) -> Result<Value, String> {
    let at = |run: &Value, key: &str| number(&run["phase_on"]["lower_button_leaves"], key);
    let part = |key: &str| Ok::<f64, String>(at(new, key)? - at(old, key)?);
    let rate = part("pitch_rate_nose_down_deg_s")?.hypot(part("pitch_rate_nose_right_deg_s")?);
    let angle = part("nose_down_deg")?.hypot(part("nose_right_deg")?);
    // The phase's move from the guided flight: up, east and north.
    let moves = |run: &Value| -> Result<[f64; 3], String> {
        let (on, guided) = (&run["phase_on"], &run["guided_to_lower_button"]);
        let by = |key: &str| Ok::<f64, String>(number(on, key)? - number(guided, key)?);
        Ok([
            by("apogee_agl_m")?,
            by("landing_east_m")?,
            by("landing_north_m")?,
        ])
    };
    let (was, is) = (moves(old)?, moves(new)?);
    let guided_apogee_m = number(&new["guided_to_lower_button"], "apogee_agl_m")?;
    let (mut apogee_m, mut landing_point_m) = (serde_json::Map::new(), serde_json::Map::new());
    for (label, key) in FLIGHTS {
        let by = |figure: &str| {
            Ok::<f64, String>(number(&new[key], figure)? - number(&old[key], figure)?)
        };
        apogee_m.insert(label.to_owned(), json!(by("apogee_agl_m")?));
        landing_point_m.insert(
            label.to_owned(),
            json!(by("landing_east_m")?.hypot(by("landing_north_m")?)),
        );
    }
    Ok(json!({
        "case": name,
        "wind": new["wind"],
        "turning_rate_deg_s": rate,
        "angle_deg": angle,
        "apogee_move_percent_of_apogee": 100.0 * (is[0] - was[0]) / guided_apogee_m,
        "landing_point_move_percent_of_apogee":
            100.0 * (is[1] - was[1]).hypot(is[2] - was[2]) / guided_apogee_m,
        "apogee_m": apogee_m,
        "landing_point_m": landing_point_m,
    }))
}

/// The rockets and winds past each bar in a run, without their figures: what has to stay the
/// same for two runs to read alike against the bars.
fn names_past_the_bars(flown: &Value) -> Result<Value, String> {
    let named = |part: &Value, key: &str| -> Result<Vec<Value>, String> {
        Ok(list(part, key)?
            .iter()
            .map(|row| json!([row["case"], row["wind"]]))
            .collect())
    };
    let mut names = serde_json::Map::new();
    for key in ["moved", "moved_from_guided"] {
        let part = past_the_bars(flown, key)?;
        names.insert(
            key.to_owned(),
            json!({
                "apogee_past_the_bar": named(&part, "apogee_past_the_bar")?,
                "landing_point_past_the_bar": named(&part, "landing_point_past_the_bar")?,
                "designs_past_a_bar": part["designs_past_a_bar"],
                "landing_farther_from_the_pad": part["landing_farther_from_the_pad"],
                "landing_nearer_the_pad": part["landing_nearer_the_pad"],
            }),
        );
    }
    Ok(Value::Object(names))
}

/// What moved between the earlier pin's report (`earlier`, [`EARLIER`]) and this run: each
/// flight's figures ([`flight_since`]), the largest of each, whether the same flights are past
/// the bars, and the largest difference in what the rockets carried. Fails unless both flew the
/// same rockets from the same rails and sites, in the same winds, on the same drag and solver
/// settings.
fn since_the_earlier_pin(earlier: &Value, flown: &Value) -> Result<Value, String> {
    let before = &earlier["flown"];
    for key in ["solver", "drag", "crosswind"] {
        if before[key] != flown[key] || before[key].is_null() {
            return Err(format!("the two pins' runs differ in `{key}`"));
        }
    }
    let (old_cases, new_cases) = (list(before, "cases")?, list(flown, "cases")?);
    if old_cases.len() != new_cases.len() {
        return Err("the two pins flew different numbers of rockets".to_owned());
    }
    let mut flights = Vec::new();
    let mut carried = 0.0_f64;
    for (old_case, new_case) in old_cases.iter().zip(new_cases) {
        let name = text(new_case, "name")?;
        for key in ["name", "site", "rail", "button_spacing_m", "parachutes"] {
            if old_case[key] != new_case[key] {
                return Err(format!("{name}: the two pins differ in `{key}`"));
            }
        }
        let (old_winds, new_winds) = (list(old_case, "winds")?, list(new_case, "winds")?);
        if old_winds.len() != new_winds.len() {
            return Err(format!("{name}: the two pins flew different winds"));
        }
        for (old, new) in old_winds.iter().zip(new_winds) {
            for key in ["wind", "declared", "wind_east_m_s", "wind_north_m_s"] {
                if old[key] != new[key] {
                    return Err(format!("{name}: the two pins differ in `{key}`"));
                }
            }
            carried = carried.max(
                largest_difference(&old["flown_with"], &new["flown_with"])
                    .map_err(|e| format!("{name}: what the flights carried: {e}"))?,
            );
            flights.push(flight_since(name, old, new)?);
        }
    }
    // The largest of a figure over the flights: its rocket, wind and value, and for a figure
    // kept by flight, which of the three flights.
    let largest = |figure: &str| -> Result<Value, String> {
        let mut top: Option<(f64, Value)> = None;
        for flight in &flights {
            let entries: Vec<(Option<&str>, f64)> = match &flight[figure] {
                Value::Object(by_flight) => FLIGHTS
                    .iter()
                    .map(|(label, _)| Ok((Some(*label), number(&flight[figure], label)?)))
                    .collect::<Result<_, String>>()
                    .and_then(|entries: Vec<_>| {
                        if entries.len() == by_flight.len() {
                            Ok(entries)
                        } else {
                            Err(format!("`{figure}` is not by flight in {flight}"))
                        }
                    })?,
                _ => vec![(None, number(flight, figure)?)],
            };
            for (label, value) in entries {
                if top
                    .as_ref()
                    .is_none_or(|(most, _)| value.abs() > most.abs())
                {
                    let mut entry = json!({
                        "case": flight["case"],
                        "wind": flight["wind"],
                        "value": value,
                    });
                    if let Some(label) = label {
                        entry["flight"] = json!(label);
                    }
                    top = Some((value, entry));
                }
            }
        }
        top.map(|(_, entry)| entry)
            .ok_or_else(|| "no flight to compare between the pins".to_owned())
    };
    let mut largest_of = serde_json::Map::new();
    for figure in [
        "turning_rate_deg_s",
        "angle_deg",
        "apogee_move_percent_of_apogee",
        "landing_point_move_percent_of_apogee",
        "apogee_m",
        "landing_point_m",
    ] {
        largest_of.insert(figure.to_owned(), largest(figure)?);
    }
    Ok(json!({
        "record": EARLIER,
        "commit": EARLIER_COMMIT,
        "sha256": EARLIER_SHA256,
        "oracle": before["oracle"],
        "equations": before["equations"],
        "same_flights_past_the_bars": names_past_the_bars(before)? == names_past_the_bars(flown)?,
        "carried_largest_difference": carried,
        "largest": largest_of,
        "flights": flights,
    }))
}

/// [`EARLIER`], after checking it is the file [`EARLIER_SHA256`] names.
fn earlier(root: &Path) -> Result<Value, String> {
    earlier_of(&fs::read(root.join(EARLIER)).map_err(|e| format!("{EARLIER}: {e}"))?)
}

/// The earlier pin's report from its bytes, refused unless they are the bytes
/// [`EARLIER_SHA256`] names: a kept report that was edited, or regenerated, is not the record.
fn earlier_of(bytes: &[u8]) -> Result<Value, String> {
    let digest: String = Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if digest != EARLIER_SHA256 {
        return Err(format!(
            "{EARLIER} is not the earlier pin's report: its SHA-256 is {digest}"
        ));
    }
    serde_json::from_slice(bytes).map_err(|e| format!("{EARLIER}: {e}"))
}

/// The report from the run's document, the probe's and the earlier pin's report.
fn report(flown: Value, lag_probe: &Value, earlier: &Value) -> Result<Value, String> {
    // The declared change stands on the probe: each flight flown without its parachute's lag
    // had not finished with it in the probe's wall time.
    let probes = list(lag_probe, "lag_probe")?;
    for probe in probes {
        if probe["finished"] != json!(false) {
            return Err(format!("the flight with its lag finished: {probe}"));
        }
        number(probe, "solver_time_s")?;
    }
    let declared: Vec<Value> = list(&flown, "cases")?
        .iter()
        .flat_map(|case| {
            case["winds"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|wind| wind.get("declared").is_some())
                .map(|wind| json!([case["name"], wind["wind"]]))
        })
        .collect();
    let probed: Vec<Value> = probes
        .iter()
        .map(|probe| json!([probe["case"], probe["wind"]]))
        .collect();
    if declared != probed {
        return Err(format!(
            "the flights with a declared change, {declared:?}, are not the probed ones, {probed:?}"
        ));
    }
    Ok(json!({
        "generator": "cargo xtask tip-off",
        "bars": {
            "apogee_percent": APOGEE_BAR_PERCENT,
            "landing_point_percent_of_apogee": LANDING_BAR_PERCENT_OF_APOGEE,
        },
        "reading": reading(&flown)?,
        "since_the_earlier_pin": since_the_earlier_pin(earlier, &flown)?,
        "lag_probe": probes,
        "flown": flown,
    }))
}

/// Flies the run and the probe. Needs the oracle environment and the pinned checkout under
/// `refs/`.
pub fn generate(root: &Path) -> Result<Value, String> {
    let flown = oracle(root, None)?;
    let lag_probe = oracle(root, Some("--lag-probe"))?;
    let pinned = crate::refs::pinned_commit(root, LOCK_NAME)?;
    if !text(&flown, "oracle")?.contains(&pinned) {
        return Err(format!(
            "{ORACLE} flew {}, not the pinned {pinned}",
            flown["oracle"]
        ));
    }
    report(flown, &lag_probe, &earlier(root)?)
}

/// The plan whose bars the report reads against: M1.14b's (ADR-351), from the report's folder.
const PLAN: &str =
    "../../docs/decisions/0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md";

/// `text` with each "pull request #N" linked to RocketPy's pull request N, since the oracle's
/// strings name RocketPy's pull requests bare.
fn linked_pulls(text: &str) -> String {
    const WORDS: &str = "pull request #";
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find(WORDS) {
        let after = &rest[at + WORDS.len()..];
        let digits = after.len() - after.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        out.push_str(&rest[..at]);
        if digits == 0 {
            out.push_str(WORDS);
        } else {
            let number = &after[..digits];
            let _ = write!(
                out,
                "[{WORDS}{number}](https://github.com/RocketPy-Team/RocketPy/pull/{number})"
            );
        }
        rest = &after[digits..];
    }
    out.push_str(rest);
    out
}

/// `value` to `decimals` places, a value that rounds to zero without its sign.
pub(crate) fn fixed(value: f64, decimals: usize) -> String {
    let shown = format!("{value:.decimals$}");
    if shown
        .trim_start_matches('-')
        .chars()
        .all(|c| c == '0' || c == '.')
    {
        shown.trim_start_matches('-').to_owned()
    } else {
        shown
    }
}

/// `value` to `decimals` places with its sign, zero bare.
pub(crate) fn signed(value: f64, decimals: usize) -> String {
    let shown = fixed(value, decimals);
    if value > 0.0 && shown.chars().any(|c| ('1'..='9').contains(&c)) {
        format!("+{shown}")
    } else {
        shown
    }
}

/// A bound's magnitude, rounded up at `decimals`, so "at most" and "no more than" stay true of
/// the figure before rounding.
pub(crate) fn at_most(value: f64, decimals: usize) -> String {
    let scale = (0..decimals).fold(1.0, |scale, _| scale * 10.0);
    fixed((value.abs() * scale).ceil() / scale, decimals)
}

/// A move's landing-point cells: the distance, its share of the apogee and how much farther from
/// the pad the rocket lands.
fn landing_cells(moved: &Value) -> Result<[String; 3], String> {
    Ok([
        format!("{} m", fixed(number(moved, "landing_point_m")?, 1)),
        format!(
            "{}%",
            fixed(number(moved, "landing_point_percent_of_apogee")?, 3)
        ),
        format!("{} m", signed(number(moved, "landing_range_m")?, 1)),
    ])
}

/// One line on a run's largest moves under `key` (`largest` or `largest_from_guided`).
fn largest_line(run: &Value, key: &str) -> Result<String, String> {
    let largest = &run[key];
    let (apogee, landing, rate) = (
        &largest["apogee"],
        &largest["landing_point"],
        &largest["pitch_rate"],
    );
    Ok(format!(
        "the apogee by at most {}% ({} m; {}, {}), the landing point by at most {}% of the apogee \
         ({} m; {}, {}), and the pitch rate where the lower button leaves by at most {} deg/s \
         ({}, {})",
        signed(number(apogee, "apogee_percent")?, 3),
        signed(number(apogee, "apogee_m")?, 1),
        text(apogee, "case")?,
        text(apogee, "wind")?,
        fixed(number(landing, "landing_point_percent_of_apogee")?, 3),
        fixed(number(landing, "landing_point_m")?, 1),
        text(landing, "case")?,
        text(landing, "wind")?,
        signed(number(rate, "pitch_rate_deg_s")?, 2),
        text(rate, "case")?,
        text(rate, "wind")?,
    ))
}

/// How many rows and designs of a reading's part are past the bars, in words.
fn past_line(part: &Value) -> Result<String, String> {
    Ok(format!(
        "{} of {} apogees past {}% and {} of {} landing points past {}% of the apogee, on {} of \
         the designs; with the phase on the rocket lands farther from the pad on {} of the {} \
         and nearer on {}",
        list(part, "apogee_past_the_bar")?.len(),
        number(part, "rows")?,
        fixed(APOGEE_BAR_PERCENT, 0),
        list(part, "landing_point_past_the_bar")?.len(),
        number(part, "rows")?,
        fixed(LANDING_BAR_PERCENT_OF_APOGEE, 0),
        list(part, "designs_past_a_bar")?.len(),
        number(part, "landing_farther_from_the_pad")?,
        number(part, "rows")?,
        number(part, "landing_nearer_the_pad")?,
    ))
}

/// The report as a page.
pub fn markdown(report: &Value) -> Result<String, String> {
    let flown = &report["flown"];
    let reading = &report["reading"];
    let since = &report["since_the_earlier_pin"];
    let crosswind = &flown["crosswind"];
    let mut page = String::new();
    let w = &mut page;
    // Writing to a String cannot fail.
    let _ = writeln!(w, "# What RocketPy's tip-off phase moves\n");
    let _ = writeln!(
        w,
        "Written by `cargo xtask tip-off`; don't edit it by hand. What it measures and how far to \
         trust it: [rigid-body flight](../../docs/physics/flight.md#what-leaving-tip-off-out-costs). \
         It is one code against another: the phase is in RocketPy's development branch, in no \
         release, and no flight checks it. By default HPR Sim frees a rocket with no turning rate \
         where its last guide leaves the rail ([its opt-in tip-off phase against this \
         report](tip-off-model.md)). This \
         report measures the pivot about that guide which that leaves out, with RocketPy's tip-off \
         phase: {}. Equations: {}. Each example rocket of \
         `validation/fixtures/design/rocketpy-rocket-mass.json` flies from its example's rail and \
         site on a declared constant drag coefficient, in still air and in a {} mph ({} m/s) wind \
         {}.\n",
        linked_pulls(text(flown, "oracle")?),
        linked_pulls(text(flown, "equations")?),
        fixed(number(crosswind, "speed_mph")?, 0),
        fixed(number(crosswind, "speed_m_s")?, 2),
        text(crosswind, "direction")?,
    );
    let _ = writeln!(w, "How to read it:\n");
    let _ = writeln!(
        w,
        "- **Phase off** is RocketPy as its releases fly: the rocket is free where its upper \
         button leaves the rail. The commit this report flies has no switch for the phase: it \
         flies it in every flight that uses its standard six-degree-of-freedom equations and \
         starts with both buttons on the rail, the lower one nearer the nozzle, so its rail \
         length is the longer. So these flights take {}.\n\
         - **Phase on** adds the tip-off phase: from there the rocket slides on along the rail, \
         free to pitch and yaw about its lower button with roll held, until that button leaves \
         too.\n\
         - **Guided** is the phase off on a rail longer by the distance between the buttons: the \
         rocket is held straight until the lower button leaves the real rail, as HPR Sim holds \
         it.\n\
         - **Rocket** is the design's id. The two Calisto ids are two versions of one rocket, \
         from RocketPy's getting-started notebook and from its tests. Each ends in the motor's \
         position in meters, in the rocket's own coordinates as its source sets it: \
         `minus-1.255` is −1.255 m. Each source measures from its own zero point, and the two \
         zero points are 0.118 m (4.6 in) apart. So in both the nozzle sits 2.533 m (8.31 ft) \
         behind the nose's tip. The fins differ, and the notebook's rail buttons and its center \
         of mass without the motor sit 0.118 m (4.6 in) farther back.\n\
         - **Rail** is the rail's length and its angle above the horizontal.\n\
         - **Moved** is the phase on less the other flight: the phase-on figure minus the \
         other's. *On less off* takes the phase off from the phase on, and *on less guided* takes \
         the guided flight. A landing point's move is the distance between the two landing \
         points, in meters and as a share of the other flight's apogee. **Farther from the pad** \
         is how much farther from the pad the rocket lands with the phase on, negative when it \
         lands nearer. **Apogee point moved** is the distance across the ground between the two \
         flights' apogee points; the change in height is in **Apogee moved**.\n\
         - **Where the lower button leaves:** the rocket's speed with the phase on, the angle \
         between the rocket's axis and the \
         rail's, its part below the rail's line in the rail's vertical plane (nose down), and \
         the turning rate across the rocket's axis (pitch and yaw together).\n\
         - **The bars** are [the plan's]({PLAN}): a tip-off model is called for when the phase \
         moves any apogee by more than {}%, or any landing point by more than {}% of the apogee.\n\
         - **A declared change** is a flight flown with one of its design's inputs changed, with \
         the change and its reason stated.\n",
        text(flown, "phase_off")?,
        fixed(APOGEE_BAR_PERCENT, 0),
        fixed(LANDING_BAR_PERCENT_OF_APOGEE, 0),
    );
    let _ = writeln!(w, "## The reading\n");
    let called = reading["model_called_for"]
        .as_bool()
        .ok_or("no `model_called_for`")?;
    let _ = writeln!(
        w,
        "- **On less off, the plan's rule:** {}. A tip-off model is {}.",
        past_line(&reading["moved"])?,
        if called {
            "called for"
        } else {
            "not called for"
        },
    );
    let _ = writeln!(w, "  The phase moves {}.", largest_line(flown, "largest")?);
    let _ = writeln!(
        w,
        "- **On less guided, what HPR Sim's release leaves out:** {}.",
        past_line(&reading["moved_from_guided"])?,
    );
    let _ = writeln!(
        w,
        "  The phase moves {}.",
        largest_line(flown, "largest_from_guided")?
    );
    // The largest of a figure between the pins: its value, and where.
    let most = |figure: &str| -> Result<(f64, String), String> {
        let entry = &since["largest"][figure];
        let flight = entry["flight"]
            .as_str()
            .map(|flight| format!(", {flight}"))
            .unwrap_or_default();
        Ok((
            number(entry, "value")?,
            format!("{}, {}{flight}", text(entry, "case")?, text(entry, "wind")?),
        ))
    };
    let same = since["same_flights_past_the_bars"]
        .as_bool()
        .ok_or("no `same_flights_past_the_bars`")?;
    let (rate, rate_at) = most("turning_rate_deg_s")?;
    let (angle, angle_at) = most("angle_deg")?;
    let (apogee_move, apogee_move_at) = most("apogee_move_percent_of_apogee")?;
    let (landing_move, landing_move_at) = most("landing_point_move_percent_of_apogee")?;
    let (apogee, apogee_at) = most("apogee_m")?;
    let (landing, landing_at) = most("landing_point_m")?;
    let carried = number(since, "carried_largest_difference")?;
    // The farthest any apogee or landing point moved among the flights flown these ways.
    let farthest_mm = |ways: &[&str]| -> Result<f64, String> {
        let mut most = 0.0_f64;
        for flight in list(since, "flights")? {
            for figure in ["apogee_m", "landing_point_m"] {
                for way in ways {
                    most = most.max(number(&flight[figure], way)?.abs());
                }
            }
        }
        Ok(most * 1000.0)
    };
    let without_mm = farthest_mm(&["phase off", "guided"])?;
    let with_mm = farthest_mm(&["phase on"])?;
    let _ = writeln!(
        w,
        "- **Since the earlier pin.** A pin is the one commit of RocketPy's source a report \
         flies. This report first flew {}. Its equations: {}. RocketPy then rewrote the phase \
         ([pull request #1217]({REWRITE})), and this report flies that pull request's merge. \
         The earlier report's data is kept as `{}`, a byte-for-byte copy of `{REPORT}` as commit \
         `{}` of this repository had it. Between the two pins, {}. \
         [The last table](#between-the-two-pins) gives each flight.\n  \
         - Where the lower button leaves with the phase on, the turning rate moved by at most \
         {} deg/s ({rate_at}) and the angle off the rail's axis by at most {}° ({angle_at}), \
         each as its nose-down and nose-right parts together.\n  \
         - What the phase moves from the guided flight changed by at most {}% of the apogee in \
         the apogee ({apogee_move_at}) and {}% of the apogee in the landing point \
         ({landing_move_at}).\n  \
         - No flight's apogee moved by more than {} mm ({apogee_at}), nor its landing point by \
         more than {} mm ({landing_at}).\n  \
         - These are differences between two commits, not the measured effect of one change. \
         By its description the rewrite simplified the phase's function (`udot_rail2`), changed \
         what RocketPy reports inside the phase and corrected how the phase ends. It also finds \
         every flight's rail exit a new way, by a root search on the solver's own interpolation \
         of the step. Other changes to RocketPy lie between the two pins as well. Over all the \
         flights without the phase (phase off and guided), an apogee or landing point moved by \
         up to {} mm; over those with it, by up to {} mm. {} The last two columns of the last \
         table give the guided flights' moves alone. This report does not test which change \
         made each.\n  \
         - {}",
        linked_pulls(text(since, "oracle")?),
        linked_pulls(text(since, "equations")?),
        text(since, "record")?,
        text(since, "commit")?
            .get(..8)
            .ok_or("the earlier pin's commit is shorter than 8 characters")?,
        if same {
            "the same flights are past each bar, and the rocket lands farther from the pad in \
             the same number of them"
        } else {
            "the flights past the bars are not the same"
        },
        at_most(rate, 6),
        at_most(angle, 7),
        at_most(apogee_move, 6),
        at_most(landing_move, 6),
        at_most(apogee * 1000.0, 2),
        at_most(landing * 1000.0, 2),
        at_most(without_mm, 2),
        at_most(with_mm, 2),
        if without_mm >= with_mm {
            "So the moves are not the phase's alone."
        } else {
            "The flights with the phase moved the farthest."
        },
        if carried == 0.0 {
            "Each rocket carried the same in both: its normal force at every Mach number, its \
             dry mass, its motor's total impulse, burn time and propellant mass, and its \
             parachutes."
                .to_owned()
        } else {
            format!(
                "What the rockets carried (normal force, dry mass, the motor's total impulse, \
                 burn time and propellant mass, parachutes) differs between the two by up to \
                 {carried:e}."
            )
        },
    );
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            if let Some(declared) = wind.get("declared") {
                let probe = list(report, "lag_probe")?
                    .iter()
                    .find(|probe| probe["case"] == case["name"] && probe["wind"] == wind["wind"])
                    .ok_or("a declared change with no probe")?;
                let declared = declared.as_str().ok_or("`declared` is not text")?;
                // The oracle states the change, then its reason, after a colon.
                let declared = declared.trim_end_matches('.');
                let (change, reason) = declared.split_once(": ").unwrap_or((declared, ""));
                let _ = writeln!(
                    w,
                    "- **A declared change** ({}, {}):\n  - The change: {}.",
                    text(case, "name")?,
                    text(wind, "wind")?,
                    change,
                );
                if !reason.is_empty() {
                    let _ = writeln!(w, "  - Why: {reason}.");
                }
                let _ = writeln!(
                    w,
                    "  - The probe (`tip_off.py --lag-probe`, {} s of wall time) had integrated \
                     to {} s. The apogee is at {} s.",
                    fixed(number(probe, "wall_s")?, 0),
                    fixed(number(probe, "solver_time_s")?, 1),
                    fixed(number(&wind["phase_off"], "apogee_time_s")?, 1),
                );
            }
        }
    }
    let _ = writeln!(w, "\n## What the phase moves: on less off\n");
    let _ = writeln!(
        w,
        "| Rocket | Wind | Rail | Between the buttons | Apogee, phase off | Apogee moved | \
         Landing point moved | Of the apogee | Farther from the pad | Apogee point moved |"
    );
    let _ = writeln!(w, "|---|---|--:|--:|--:|--:|--:|--:|--:|--:|");
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            let moved = &wind["moved"];
            let [meters, share, range] = landing_cells(moved)?;
            let _ = writeln!(
                w,
                "| {} | {} | {} m at {}° | {} m | {} m | {}% | {} | {} | {} | {} m |",
                text(case, "name")?,
                text(wind, "wind")?,
                fixed(number(&case["rail"], "rail_length_m")?, 2),
                fixed(number(&case["rail"], "inclination_deg")?, 1),
                fixed(number(case, "button_spacing_m")?, 3),
                fixed(number(&wind["phase_off"], "apogee_agl_m")?, 1),
                signed(number(moved, "apogee_percent")?, 3),
                meters,
                share,
                range,
                fixed(number(moved, "apogee_point_m")?, 1),
            );
        }
    }

    let _ = writeln!(w, "\n## Where the lower button leaves the rail\n");
    let _ = writeln!(
        w,
        "| Rocket | Wind | Speed, phase on | Off the rail's axis: off | on | guided | Nose down: \
         off | on | Pitch rate: off | on | guided |"
    );
    let _ = writeln!(w, "|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            let state = |key: &str| &wind[key]["lower_button_leaves"];
            let (off, on, guided) = (
                state("phase_off"),
                state("phase_on"),
                state("guided_to_lower_button"),
            );
            let _ = writeln!(
                w,
                "| {} | {} | {} m/s | {}° | {}° | {}° | {}° | {}° | {} deg/s | {} deg/s | {} deg/s |",
                text(case, "name")?,
                text(wind, "wind")?,
                fixed(number(on, "speed_m_s")?, 2),
                fixed(number(off, "off_rail_axis_deg")?, 3),
                fixed(number(on, "off_rail_axis_deg")?, 3),
                fixed(number(guided, "off_rail_axis_deg")?, 3),
                fixed(number(off, "nose_down_deg")?, 3),
                fixed(number(on, "nose_down_deg")?, 3),
                fixed(number(off, "pitch_rate_deg_s")?, 2),
                fixed(number(on, "pitch_rate_deg_s")?, 2),
                fixed(number(guided, "pitch_rate_deg_s")?, 2),
            );
        }
    }

    let _ = writeln!(
        w,
        "\n## What HPR Sim's release leaves out: on less guided\n"
    );
    let _ = writeln!(
        w,
        "| Rocket | Wind | Apogee, guided | Apogee moved | Landing point moved | Of the apogee | \
         Farther from the pad | Pitch rate moved |"
    );
    let _ = writeln!(w, "|---|---|--:|--:|--:|--:|--:|--:|");
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            let moved = &wind["moved_from_guided"];
            let [meters, share, range] = landing_cells(moved)?;
            let _ = writeln!(
                w,
                "| {} | {} | {} m | {}% | {} | {} | {} | {} deg/s |",
                text(case, "name")?,
                text(wind, "wind")?,
                fixed(number(&wind["guided_to_lower_button"], "apogee_agl_m")?, 1),
                signed(number(moved, "apogee_percent")?, 3),
                meters,
                share,
                range,
                signed(number(moved, "pitch_rate_deg_s")?, 2),
            );
        }
    }

    let _ = writeln!(w, "\n## Between the two pins\n");
    let _ = writeln!(
        w,
        "This pin's figure less the earlier pin's, for each flight: the difference between \
         the two commits of RocketPy, whatever in them made it. The first four are the \
         figures HPR Sim's own phase is held to RocketPy's on \
         ([the comparison](tip-off-model.md)), compared the same way: the turning rate and the \
         angle where the lower button leaves with the phase on, each as its nose-down and \
         nose-right parts together; and what the phase moves from the guided flight, in the \
         apogee and, as a vector, in the landing point, each as a share of the guided apogee. \
         The last two are how far the guided flight itself moved. Each figure here is rounded \
         to the nearest; the bounds under [the reading](#the-reading) are rounded up.\n"
    );
    let _ = writeln!(
        w,
        "| Rocket | Wind | Turning rate | Angle | Apogee move | Landing-point move | Apogee, \
         guided | Landing point, guided |"
    );
    let _ = writeln!(w, "|---|---|--:|--:|--:|--:|--:|--:|");
    for flight in list(since, "flights")? {
        let _ = writeln!(
            w,
            "| {} | {} | {} deg/s | {}° | {}% | {}% | {} mm | {} mm |",
            text(flight, "case")?,
            text(flight, "wind")?,
            fixed(number(flight, "turning_rate_deg_s")?, 6),
            fixed(number(flight, "angle_deg")?, 7),
            signed(number(flight, "apogee_move_percent_of_apogee")?, 6),
            fixed(number(flight, "landing_point_move_percent_of_apogee")?, 6),
            signed(number(&flight["apogee_m"], "guided")? * 1000.0, 2),
            fixed(number(&flight["landing_point_m"], "guided")? * 1000.0, 2),
        );
    }
    Ok(page)
}

/// `--page`: writes [`REPORT_MD`] from the committed [`REPORT`], with no oracle run, for a change
/// to the page's words alone. `--check` still holds the JSON to a fresh run.
fn rewrite_page() -> Result<(), String> {
    let root = crate::designs::root()?;
    let committed = fs::read_to_string(root.join(REPORT)).map_err(|e| format!("{REPORT}: {e}"))?;
    let committed: Value =
        serde_json::from_str(&committed).map_err(|e| format!("{REPORT}: {e}"))?;
    fs::write(root.join(REPORT_MD), markdown(&committed)?)
        .map_err(|e| format!("{REPORT_MD}: {e}"))?;
    println!("wrote {REPORT_MD} from {REPORT}");
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        [flag] if flag == "--page" => return rewrite_page(),
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let fresh = generate(&root)?;
    let page = markdown(&fresh)?;
    let (path, page_path) = (root.join(REPORT), root.join(REPORT_MD));
    if check {
        let committed = fs::read_to_string(&path).map_err(|e| format!("{REPORT}: {e}"))?;
        let committed: Value =
            serde_json::from_str(&committed).map_err(|e| format!("{REPORT}: {e}"))?;
        if !crate::designs::same(&committed, &fresh) {
            return Err(format!(
                "{REPORT} is stale: run `cargo xtask tip-off` and commit it: {}",
                crate::designs::difference(&committed, &fresh).unwrap_or_default()
            ));
        }
        let committed_page =
            fs::read_to_string(&page_path).map_err(|e| format!("{REPORT_MD}: {e}"))?;
        if committed_page != markdown(&committed)? {
            return Err(format!(
                "{REPORT_MD} is not {REPORT}'s page: run `cargo xtask tip-off` and commit it"
            ));
        }
        println!("{REPORT} is current");
        return Ok(());
    }
    let text = serde_json::to_string_pretty(&fresh).map_err(|e| e.to_string())? + "\n";
    fs::write(&path, text).map_err(|e| format!("{REPORT}: {e}"))?;
    fs::write(&page_path, page).map_err(|e| format!("{REPORT_MD}: {e}"))?;
    println!("wrote {REPORT} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A run of one rocket in one wind whose phase moved the apogee and the landing point by
    /// these shares.
    fn run_of(apogee_percent: f64, landing_percent: f64) -> Value {
        json!({"cases": [{"name": "a", "winds": [{
            "wind": "still",
            "moved": {
                "apogee_percent": apogee_percent,
                "landing_point_m": 10.0,
                "landing_range_m": -2.0,
                "landing_point_percent_of_apogee": landing_percent,
            },
            "moved_from_guided": {
                "apogee_percent": 0.0,
                "landing_point_m": 1.0,
                "landing_point_percent_of_apogee": 0.1,
                "landing_range_m": 0.5,
            },
        }]}]})
    }

    #[test]
    fn a_move_past_either_bar_calls_for_a_model_and_one_at_a_bar_does_not() {
        let called = |apogee, landing| {
            reading(&run_of(apogee, landing)).unwrap()["model_called_for"]
                .as_bool()
                .unwrap()
        };
        // At the bars, and under them on both sides of zero: no model.
        assert!(!called(1.0, 1.0));
        assert!(!called(-1.0, 0.0));
        assert!(!called(0.999, 0.999));
        // Past the apogee's bar either way, or past the landing point's.
        assert!(called(1.001, 0.0));
        assert!(called(-1.001, 0.0));
        assert!(called(0.0, 1.001));
        // Which way the landing point moved is counted on every row.
        let read = reading(&run_of(0.0, 0.0)).unwrap();
        assert_eq!(read["moved"]["rows"], 1);
        assert_eq!(read["moved"]["landing_nearer_the_pad"], 1);
        assert_eq!(read["moved"]["landing_farther_from_the_pad"], 0);
        assert_eq!(read["moved_from_guided"]["landing_farther_from_the_pad"], 1);
        // The reading is the plan's rule, on less off: a move from the guided flight alone does
        // not call for a model.
        let mut run = run_of(0.0, 0.0);
        run["cases"][0]["winds"][0]["moved_from_guided"]["landing_point_percent_of_apogee"] =
            json!(5.0);
        let read = reading(&run).unwrap();
        assert_eq!(read["model_called_for"], false);
        assert_eq!(
            read["moved_from_guided"]["designs_past_a_bar"],
            json!(["a"])
        );
    }

    #[test]
    fn figures_keep_their_places_and_zero_has_no_sign() {
        assert_eq!(fixed(-0.0004, 3), "0.000");
        assert_eq!(fixed(-0.0005001, 3), "-0.001");
        assert_eq!(fixed(113.72, 1), "113.7");
        assert_eq!(signed(0.3206, 3), "+0.321");
        assert_eq!(signed(-0.378, 3), "-0.378");
        assert_eq!(signed(0.0001, 3), "0.000");
        assert_eq!(signed(-0.0001, 3), "0.000");
    }

    fn committed() -> Value {
        let root = crate::designs::root().unwrap();
        serde_json::from_str(&fs::read_to_string(root.join(REPORT)).unwrap()).unwrap()
    }

    /// RocketPy's pull requests named in the oracle's strings become links, each to its own
    /// number; a "pull request #" with no number stays as written.
    #[test]
    fn pull_requests_are_linked_by_number() {
        assert_eq!(
            linked_pulls("develop (pull request #920's merge) with pull request #1196's fix"),
            "develop ([pull request #920](https://github.com/RocketPy-Team/RocketPy/pull/920)'s \
             merge) with [pull request #1196](https://github.com/RocketPy-Team/RocketPy/pull/1196)'s fix"
        );
        assert_eq!(
            linked_pulls("pull request #x and none"),
            "pull request #x and none"
        );
        assert_eq!(
            linked_pulls("ends at pull request #"),
            "ends at pull request #"
        );
    }

    /// The committed page states the declared change, its reason and the probe as three lines,
    /// links RocketPy's pull requests and the plan, and defines before its reading the terms a
    /// reader could not guess: Rocket, Rail, Apogee point moved, on less off and a declared change.
    #[test]
    fn the_page_splits_the_declared_change_and_defines_its_terms() {
        let page = markdown(&committed()).unwrap();
        assert!(
            page.contains(
                "  - The change: the parachute that opens at apogee opens with no lag.\n"
            )
        );
        assert!(page.contains("\n  - Why: on a vertical rail in still air"));
        assert!(page.contains("\n  - The probe (`tip_off.py --lag-probe`"));
        assert!(page.contains("(https://github.com/RocketPy-Team/RocketPy/pull/920)"));
        assert!(page.contains("(https://github.com/RocketPy-Team/RocketPy/pull/1196)"));
        assert!(page.contains("(https://github.com/RocketPy-Team/RocketPy/pull/1217)"));
        assert!(page.contains(&format!("[the plan's]({PLAN})")));
        for term in [
            "**Rocket**",
            "**Rail**",
            "**Apogee point moved**",
            "*On less off*",
            "**A declared change** is",
        ] {
            let defined = page.find(term).unwrap_or(usize::MAX);
            assert!(
                defined < page.find("## The reading").unwrap(),
                "{term} isn't defined first"
            );
        }
        let root = crate::designs::root().unwrap();
        assert!(
            root.join("validation/reports").join(PLAN).exists(),
            "{PLAN} doesn't resolve"
        );
    }

    /// The two Calisto designs are two versions of one rocket, not one rocket with its motor
    /// moved: their fins differ, and the nose, fin set, tail and motor positions differ by one
    /// offset, so the motor sits the same distance behind the nose in both. The rail buttons and
    /// the center of mass without the motor keep their coordinates, so in the notebook's they sit
    /// that offset farther back. The motor's position is its nozzle (`nozzle_position` 0) and a
    /// nose's is its tip, as RocketPy's `add_nose` documents. The report page and the flight
    /// page say so with the fixture's figures.
    #[test]
    fn the_two_calistos_differ_in_fins_and_origin_not_motor_place() {
        let root = crate::designs::root().unwrap();
        let fixture: Value = serde_json::from_str(
            &fs::read_to_string(root.join("validation/fixtures/design/rocketpy-rocket-mass.json"))
                .unwrap(),
        )
        .unwrap();
        let case = |name: &str| -> Value {
            fixture["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == name)
                .unwrap_or_else(|| panic!("{name} isn't in the fixture"))
                .clone()
        };
        let notebook = case("calisto-getting-started-motor-at-minus-1.255");
        let tests = case("calisto-tests-motor-at-minus-1.373");
        let at = |c: &Value, path: &[&str]| -> f64 {
            path.iter()
                .fold(c, |v, key| match key.parse::<usize>() {
                    Ok(i) => &v[i],
                    Err(_) => &v[*key],
                })
                .as_f64()
                .unwrap()
        };
        let motor = ["rocket", "motor_position"];
        let nose = ["geometry", "nose", "position"];
        let offset = at(&notebook, &motor) - at(&tests, &motor);
        assert!((at(&notebook, &nose) - at(&tests, &nose) - offset).abs() < 1e-9);
        // The notebook gives the fin and tail stations to more digits (−1.04956 and −1.194656
        // against −1.168 and −1.313), so they move with the origin to within half a millimeter.
        for path in [
            &["geometry", "fin_sets", "0", "position"][..],
            &["geometry", "tails", "0", "position"],
        ] {
            assert!(
                (at(&notebook, path) - at(&tests, path) - offset).abs() < 5e-4,
                "{path:?} doesn't move with the origin"
            );
        }
        let behind = at(&notebook, &nose) - at(&notebook, &motor);
        assert!((behind - (at(&tests, &nose) - at(&tests, &motor))).abs() < 1e-9);
        assert_ne!(
            at(&notebook, &["geometry", "fin_sets", "0", "span"]),
            at(&tests, &["geometry", "fin_sets", "0", "span"])
        );
        // The airframe moves with the zero point; the buttons and the dry center of mass don't.
        let back = |c: &Value, path: &[&str]| at(c, &nose) - at(c, path);
        let lower = ["geometry", "rail_buttons", "lower_button_position"];
        let mass = ["rocket", "center_of_mass_without_motor"];
        for path in [
            &lower[..],
            &["geometry", "rail_buttons", "upper_button_position"],
            &mass,
        ] {
            assert!(
                (back(&notebook, path) - back(&tests, path) - offset).abs() < 5e-4,
                "{path:?} doesn't keep its coordinate"
            );
        }
        let one_line = |text: String| text.split_whitespace().collect::<Vec<_>>().join(" ");
        let page = one_line(markdown(&committed()).unwrap());
        let flight = one_line(fs::read_to_string(root.join("docs/physics/flight.md")).unwrap());
        let inches = offset / 0.0254;
        for text in [&page, &flight] {
            for phrase in [
                format!("the two zero points are {offset:.3} m ({inches:.1} in) apart."),
                format!(
                    "So in both the nozzle sits {behind:.3} m ({:.2} ft) behind the nose's tip.",
                    behind / 0.3048
                ),
                format!(
                    "The fins differ, and the notebook's rail buttons and its center of mass \
                     without the motor sit {offset:.3} m ({inches:.1} in) farther back."
                ),
            ] {
                assert!(text.contains(&phrase), "missing: {phrase}");
            }
            assert!(!text.to_lowercase().contains("two motor positions"));
        }
    }

    /// The committed page is the committed report's, so the two can't drift apart.
    #[test]
    fn the_committed_page_is_the_reports() {
        let root = crate::designs::root().unwrap();
        let page = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(page, markdown(&committed()).unwrap());
    }

    /// The pages quote the committed report, which CI can't regenerate. Held to the report, at
    /// the report page's own precision: the measured figures of the flight page's section with
    /// their conversions to US units, its declared wind and drag, and its two short statements
    /// of the cost; the cost on the validation page, in the glossary and in the risks note;
    /// and, from the report kept from the earlier pin, the table of the decision record that
    /// measured on that pin and its sentence on the equations as pinned. Not held: that
    /// record's other figures.
    #[test]
    fn the_pages_quote_the_committed_report() {
        let root = crate::designs::root().unwrap();
        let report = committed();
        let flown = &report["flown"];
        // A page wraps its lines, groups digits in threes and writes a minus sign.
        let page = |path: &str| {
            let text = fs::read_to_string(root.join(path))
                .unwrap()
                .replace(',', "");
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        };
        let minus = |shown: String| shown.replace('-', "−");
        let num = |value: &Value, key: &str| number(value, key).unwrap();
        let count = |part: &Value, key: &str| part[key].as_array().unwrap().len();
        let wind_of = |name: &str, kind: &str| -> &Value {
            list(flown, "cases")
                .unwrap()
                .iter()
                .find(|case| case["name"] == name)
                .and_then(|case| {
                    list(case, "winds")
                        .unwrap()
                        .iter()
                        .find(|wind| wind["wind"] == kind)
                })
                .unwrap()
        };
        let at = |flight: &Value, key: &str| num(&flight["lower_button_leaves"], key);
        let every_wind = || {
            list(flown, "cases")
                .unwrap()
                .iter()
                .flat_map(|case| list(case, "winds").unwrap())
        };
        let largest = |of: &dyn Fn(&Value) -> f64| every_wind().map(of).fold(0.0, f64::max);

        let (off_reading, guided_reading) = (
            &report["reading"]["moved"],
            &report["reading"]["moved_from_guided"],
        );
        let (off, guided) = (&flown["largest"], &flown["largest_from_guided"]);
        let rows = num(guided_reading, "rows");
        assert_eq!(rows, 26.0);
        let guided_apogee = minus(signed(num(&guided["apogee"], "apogee_percent"), 3));
        let guided_landing = fixed(
            num(&guided["landing_point"], "landing_point_percent_of_apogee"),
            3,
        );
        let cost = format!("{guided_landing}% of the apogee");
        let apogee_cost = format!("{}%", guided_apogee.trim_start_matches('−'));

        // The three rockets past the bar against the guided flight, as the pages name them.
        assert_eq!(
            guided_reading["designs_past_a_bar"],
            json!(["valetudo", "juno-iii", "prometheus-2022-generic-motor"])
        );
        let juno = wind_of("juno-iii", "still");
        let bella = wind_of("bella-lui", "crosswind");
        let juno_case = list(flown, "cases")
            .unwrap()
            .iter()
            .find(|case| case["name"] == "juno-iii")
            .unwrap();
        let held =
            at(&juno["phase_on"], "time_s") - num(&juno["phase_on"], "upper_button_leaves_s");
        let flight_page = page("docs/physics/flight.md");
        // A length as the page gives it, with its feet at `feet` places.
        let with_feet = |meters: f64, places: usize, feet: usize| {
            format!(
                "{} m ({} ft)",
                fixed(meters, places),
                fixed(meters / 0.3048, feet)
            )
        };
        let apogee_m = num(&guided["apogee"], "apogee_m");
        let speed = at(&juno["phase_on"], "speed_m_s");
        let drag: Vec<f64> = list(&flown["drag"], "declared_cd0")
            .unwrap()
            .iter()
            .map(|point| point[1].as_f64().unwrap())
            .collect();
        assert!(drag.iter().all(|&cd| cd == drag[0]), "{drag:?}");
        assert_eq!(count(guided_reading, "apogee_past_the_bar"), 0);
        assert_eq!(report["bars"]["landing_point_percent_of_apogee"], 1.0);
        let expected = [
            "None of the 26 apogees moves by 1%".to_owned(),
            "past the plan's bar of 1% of the apogee".to_owned(),
            format!("constant drag coefficient of {}", drag[0]),
            format!(
                "a {} mph ({} m/s) wind",
                fixed(num(&flown["crosswind"], "speed_mph"), 0),
                fixed(num(&flown["crosswind"], "speed_m_s"), 2)
            ),
            format!("at most {guided_apogee}%: "),
            minus(format!(
                "{} m ({} ft)",
                signed(apogee_m, 1),
                signed(apogee_m / 0.3048, 0)
            )),
            format!(
                "in {} of the 26 flights. They are on three rockets: Valetudo Juno III and \
                 Prometheus 2022",
                count(guided_reading, "landing_point_past_the_bar")
            ),
            format!("The largest share is {cost}"),
            with_feet(num(&guided["landing_point"], "landing_point_m"), 1, 0),
            format!(
                "The largest distance is {}",
                with_feet(
                    largest(&|wind| num(&wind["moved_from_guided"], "landing_point_m")),
                    1,
                    0
                )
            ),
            format!(
                "farther from the pad in {} of the 26 flights",
                num(guided_reading, "landing_farther_from_the_pad")
            ),
            {
                // The one flight with nothing to turn it: the vertical rail's, in still air.
                let range_m = num(
                    &wind_of("ndrt-2020-nose-to-tail", "still")["moved_from_guided"],
                    "landing_range_m",
                );
                format!(
                    "the vertical rail's in still air moves by {} mm ({} in)",
                    fixed(range_m * 1000.0, 0),
                    fixed(range_m / 0.0254, 1)
                )
            },
            format!(
                "turning at up to {} deg/s",
                fixed(
                    largest(&|wind| at(&wind["phase_on"], "pitch_rate_deg_s")),
                    2
                )
            ),
            format!(
                "at most {}° off the rail's",
                fixed(
                    largest(&|wind| at(&wind["phase_on"], "off_rail_axis_deg")),
                    3
                )
            ),
            format!(
                "moves {} of the 26 landing points",
                count(off_reading, "landing_point_past_the_bar")
            ),
            format!(
                "by up to {}%",
                fixed(
                    num(&off["landing_point"], "landing_point_percent_of_apogee"),
                    3
                )
            ),
            format!(
                "one apogee by {}%",
                signed(num(&off["apogee"], "apogee_percent"), 3)
            ),
            format!(
                "at {} deg/s by the time",
                fixed(at(&bella["phase_off"], "pitch_rate_deg_s"), 2)
            ),
            format!(
                "it turns at {} deg/s",
                fixed(at(&bella["phase_on"], "pitch_rate_deg_s"), 2)
            ),
            format!(
                "leaves a {}",
                with_feet(num(&juno_case["rail"], "rail_length_m"), 2, 1)
            ),
            format!(
                "rail at {}°",
                fixed(num(&juno_case["rail"], "inclination_deg"), 0)
            ),
            format!(
                "its buttons {}",
                with_feet(num(juno_case, "button_spacing_m"), 3, 2)
            ),
            format!(
                "its last button leaves at {} m/s ({} ft/s)",
                fixed(speed, 2),
                fixed(speed / 0.3048, 1)
            ),
            format!("{} s after the forward one", fixed(held, 3)),
            format!(
                "nose-down at {} deg/s",
                fixed(at(&juno["phase_on"], "pitch_rate_deg_s"), 2)
            ),
            format!(
                "{}° off the rail's line",
                fixed(at(&juno["phase_on"], "nose_down_deg"), 3)
            ),
            format!(
                "Guided it reaches {}",
                with_feet(num(&juno["guided_to_lower_button"], "apogee_agl_m"), 1, 0)
            ),
            format!(
                "it reaches {}",
                with_feet(num(&juno["phase_on"], "apogee_agl_m"), 1, 0)
            ),
            format!(
                "lands {}",
                with_feet(num(&juno["moved_from_guided"], "landing_range_m"), 1, 0)
            ),
            {
                // The flights where the guided rocket lands farther from the pad: three, all in
                // the crosswind, by the range the page gives.
                let nearer: Vec<(f64, &str)> = list(flown, "cases")
                    .unwrap()
                    .iter()
                    .flat_map(|case| list(case, "winds").unwrap())
                    .map(|wind| {
                        (
                            num(&wind["moved_from_guided"], "landing_range_m"),
                            wind["wind"].as_str().unwrap(),
                        )
                    })
                    .filter(|(range, _)| *range < 0.0)
                    .collect();
                assert_eq!(nearer.len(), 3);
                assert!(nearer.iter().all(|(_, wind)| *wind == "crosswind"));
                let least = nearer.iter().map(|(r, _)| -r).fold(f64::INFINITY, f64::min);
                let most = nearer.iter().map(|(r, _)| -r).fold(0.0, f64::max);
                format!(
                    "In the other three all in the crosswind the guided rocket lands {} to {} m \
                     ({} to {} ft) farther from the pad",
                    fixed(least, 1),
                    fixed(most, 1),
                    fixed(least / 0.3048, 0),
                    fixed(most / 0.3048, 0)
                )
            },
            {
                let range_m = num(
                    &wind_of("ndrt-2020-nose-to-tail", "still")["moved_from_guided"],
                    "landing_range_m",
                );
                assert_eq!(fixed(range_m, 1), "0.0");
                "which the report's table rounds to zero".to_owned()
            },
            {
                let guided_rate = at(&juno["guided_to_lower_button"], "pitch_rate_deg_s");
                assert_eq!(fixed(guided_rate, 2), "0.00");
                "guided it isn't turning".to_owned()
            },
            format!(
                "unrounded the apogee moves down by {}",
                with_feet(-num(&juno["moved_from_guided"], "apogee_m"), 2, 1)
            ),
        ];
        for fragment in &expected {
            assert!(
                flight_page.contains(fragment),
                "the flight page lacks `{fragment}`"
            );
        }
        // Where the page says tip-off is left out, it gives the cost: in short, at the rail's
        // geometry and in the section itself.
        let both = format!("{cost} in the landing point and {apogee_cost} in the apogee");
        assert_eq!(flight_page.matches(&both).count(), 2, "{both}");
        let rail = format!("by up to {cost} and the apogee by up to {apogee_cost}");
        assert!(flight_page.contains(&rail), "{rail}");

        for path in [
            "docs/VALIDATION.md",
            "docs/glossary.md",
            "docs/research/known-risks.md",
        ] {
            assert!(page(path).contains(&cost), "{path} lacks `{cost}`");
        }
        assert!(page("docs/VALIDATION.md").contains(&format!("by up to {apogee_cost} from")));

        // The decision record's table and its reading of the equations as pinned.
        let record = fs::read_dir(root.join("docs/decisions"))
            .unwrap()
            .filter_map(|entry| entry.unwrap().file_name().into_string().ok())
            .find(|name| name.starts_with("0357-"))
            .unwrap();
        let record = page(&format!("docs/decisions/{record}"));
        // That record is the measurement on the earlier pin, so it is held to the report kept
        // from it, not to this pin's.
        let was = earlier(&root).unwrap();
        let (off_reading, guided_reading) = (
            &was["reading"]["moved"],
            &was["reading"]["moved_from_guided"],
        );
        let (off, guided) = (
            &was["flown"]["largest"],
            &was["flown"]["largest_from_guided"],
        );
        let guided_apogee = minus(signed(num(&guided["apogee"], "apogee_percent"), 3));
        let guided_landing = fixed(
            num(&guided["landing_point"], "landing_point_percent_of_apogee"),
            3,
        );
        let pinned = &was["as_pinned"];
        let of_13 = |part: &Value| count(part, "designs_past_a_bar");
        let expected = [
            format!(
                "| {} of 26 | {} of 26 |",
                count(off_reading, "apogee_past_the_bar"),
                count(guided_reading, "apogee_past_the_bar")
            ),
            format!(
                "| {} of 26 | {} of 26 |",
                count(off_reading, "landing_point_past_the_bar"),
                count(guided_reading, "landing_point_past_the_bar")
            ),
            format!(
                "| {} of 13 | {} of 13 |",
                of_13(off_reading),
                of_13(guided_reading)
            ),
            format!(
                "| {}% | {guided_apogee}% |",
                signed(num(&off["apogee"], "apogee_percent"), 3)
            ),
            format!(
                "| {}% | {guided_landing}% |",
                fixed(
                    num(&off["landing_point"], "landing_point_percent_of_apogee"),
                    3
                )
            ),
            format!(
                "| {} of 26 | {} of 26 |",
                num(off_reading, "landing_farther_from_the_pad"),
                num(guided_reading, "landing_farther_from_the_pad")
            ),
            format!(
                "one apogee and {} landing points past the bars",
                count(&pinned["reading"]["moved"], "landing_point_past_the_bar")
            ),
            format!(
                "no apogee and {} landing points the largest {}% of the apogee",
                count(
                    &pinned["reading"]["moved_from_guided"],
                    "landing_point_past_the_bar"
                ),
                fixed(
                    num(
                        &pinned["largest_from_guided"]["landing_point"],
                        "landing_point_percent_of_apogee"
                    ),
                    3
                )
            ),
        ];
        for fragment in &expected {
            assert!(
                record.contains(fragment),
                "the decision record lacks `{fragment}`"
            );
        }
        assert_eq!(count(&pinned["reading"]["moved"], "apogee_past_the_bar"), 1);
        assert_eq!(
            count(
                &pinned["reading"]["moved_from_guided"],
                "apogee_past_the_bar"
            ),
            0
        );
    }

    /// The committed report holds together without the oracle: it flew the pinned commit, every
    /// public design built from RocketPy's examples in both winds, each move is its two
    /// flights' difference, the reading is the rows', and so are the largest apogee and
    /// landing-point shares. The one declared change has its probe, which had not finished.
    #[test]
    fn the_committed_report_is_its_own_flights() {
        let root = crate::designs::root().unwrap();
        let report = committed();
        let flown = &report["flown"];
        let pinned = crate::refs::pinned_commit(&root, LOCK_NAME).unwrap();
        assert!(text(flown, "oracle").unwrap().contains(&pinned));
        assert!(
            text(flown, "equations").unwrap().contains("#1196"),
            "the run flies the corrected equations"
        );
        assert!(
            text(flown, "phase_off")
                .unwrap()
                .contains("free flight from where the upper button leaves"),
            "the run says how it flew without the phase"
        );
        assert!(report.get("as_pinned").is_none());

        // Every public design built from RocketPy's examples is a case, and no other.
        let mut designs: Vec<String> = fs::read_dir(root.join("validation/designs"))
            .unwrap()
            .filter_map(|entry| entry.unwrap().file_name().into_string().ok())
            .filter_map(|name| {
                Some(
                    name.strip_prefix("rocketpy-")?
                        .strip_suffix(".json")?
                        .to_owned(),
                )
            })
            .collect();
        designs.sort();
        let cases = list(flown, "cases").unwrap();
        let mut names: Vec<String> = cases
            .iter()
            .map(|case| text(case, "name").unwrap().to_owned())
            .collect();
        names.sort();
        assert_eq!(names, designs);
        assert_eq!(names.len(), 13);

        let close = |a: f64, b: f64| (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0);
        let mut declared = Vec::new();
        for case in cases {
            let winds = list(case, "winds").unwrap();
            let kinds: Vec<&str> = winds.iter().map(|w| text(w, "wind").unwrap()).collect();
            assert_eq!(kinds, ["still", "crosswind"]);
            let heading = number(&case["rail"], "heading_deg").unwrap().to_radians();
            for wind in winds {
                let name = format!("{}, {}", case["name"], wind["wind"]);
                if wind.get("declared").is_some() {
                    assert_eq!(number(&case["rail"], "inclination_deg").unwrap(), 90.0);
                    declared.push(name.clone());
                }
                // The declared wind: none, or 20 mph square to the heading, toward its right.
                let speed = if wind["wind"] == "still" {
                    0.0
                } else {
                    20.0 * 0.447_04
                };
                assert!(close(
                    number(wind, "wind_east_m_s").unwrap(),
                    speed * heading.cos()
                ));
                assert!(close(
                    number(wind, "wind_north_m_s").unwrap(),
                    -speed * heading.sin()
                ));
                let (off, on, guided) = (
                    &wind["phase_off"],
                    &wind["phase_on"],
                    &wind["guided_to_lower_button"],
                );
                for (key, base) in [("moved", off), ("moved_from_guided", guided)] {
                    let moved = &wind[key];
                    let apogee = number(base, "apogee_agl_m").unwrap();
                    let by = number(on, "apogee_agl_m").unwrap() - apogee;
                    assert!(close(number(moved, "apogee_m").unwrap(), by), "{name}");
                    assert!(
                        close(
                            number(moved, "apogee_percent").unwrap(),
                            100.0 * by / apogee
                        ),
                        "{name}"
                    );
                    let apart = |east: &str, north: &str| {
                        (number(on, east).unwrap() - number(base, east).unwrap())
                            .hypot(number(on, north).unwrap() - number(base, north).unwrap())
                    };
                    assert!(close(
                        number(moved, "apogee_point_m").unwrap(),
                        apart("apogee_east_m", "apogee_north_m")
                    ));
                    let landing = apart("landing_east_m", "landing_north_m");
                    assert!(close(number(moved, "landing_point_m").unwrap(), landing));
                    assert!(close(
                        number(moved, "landing_point_percent_of_apogee").unwrap(),
                        100.0 * landing / apogee
                    ));
                    let from_pad = |flight: &Value| {
                        number(flight, "landing_east_m")
                            .unwrap()
                            .hypot(number(flight, "landing_north_m").unwrap())
                    };
                    let range = number(moved, "landing_range_m").unwrap();
                    assert!((range - (from_pad(on) - from_pad(base))).abs() < 1e-9);
                    for part in ["off_rail_axis_deg", "nose_down_deg", "pitch_rate_deg_s"] {
                        let at =
                            |flight: &Value| number(&flight["lower_button_leaves"], part).unwrap();
                        assert!(close(number(moved, part).unwrap(), at(on) - at(base)));
                    }
                }
                // Guided, the rocket leaves straight and unturned; with the phase on it leaves
                // turning.
                let rate = |flight: &Value| {
                    number(&flight["lower_button_leaves"], "pitch_rate_deg_s").unwrap()
                };
                assert!(rate(guided) < 1e-6, "{name}: {}", rate(guided));
                assert!(rate(on) > rate(guided), "{name}");
            }
        }
        // The one declared change is the vertical rail's in still air, and its probe, flown
        // with the lag, had not finished: its integration was past the apogee and short of
        // the flight without the lag's end.
        assert_eq!(declared, ["\"ndrt-2020-nose-to-tail\", \"still\""]);
        let probes = list(&report, "lag_probe").unwrap();
        assert_eq!(probes.len(), 1);
        let probe = &probes[0];
        assert_eq!(probe["case"], "ndrt-2020-nose-to-tail");
        assert_eq!(probe["wind"], "still");
        assert_eq!(probe["finished"], false);
        let without_lag = list(flown, "cases")
            .unwrap()
            .iter()
            .find(|case| case["name"] == probe["case"])
            .map(|case| &case["winds"][0]["phase_off"])
            .unwrap();
        let reached = number(probe, "solver_time_s").unwrap();
        assert!(reached > number(without_lag, "apogee_time_s").unwrap());
        assert!(reached < number(without_lag, "flight_time_s").unwrap());

        // The reading and the largest moves are recomputed from the rows.
        assert_eq!(report["reading"], reading(flown).unwrap());
        for key in ["moved", "moved_from_guided"] {
            let all = rows(flown, key).unwrap();
            let largest = &flown[if key == "moved" {
                "largest"
            } else {
                "largest_from_guided"
            }];
            let apogee = all
                .iter()
                .map(|row| number(row.2, "apogee_percent").unwrap().abs())
                .fold(0.0, f64::max);
            assert_eq!(
                number(&largest["apogee"], "apogee_percent").unwrap().abs(),
                apogee
            );
            let landing = all
                .iter()
                .filter_map(|row| row.2["landing_point_percent_of_apogee"].as_f64())
                .fold(0.0, f64::max);
            assert_eq!(
                number(&largest["landing_point"], "landing_point_percent_of_apogee").unwrap(),
                landing
            );
        }
        assert_eq!(report["bars"]["apogee_percent"], APOGEE_BAR_PERCENT);
        assert_eq!(
            report["bars"]["landing_point_percent_of_apogee"],
            LANDING_BAR_PERCENT_OF_APOGEE
        );
    }

    /// One rocket in one wind as a run's document has it, for the comparison between pins:
    /// the three flights' apogees and landing points, and where the lower button leaves.
    fn pin_of(on: [f64; 3], guided: [f64; 3], rate: [f64; 2], angle: [f64; 2]) -> Value {
        let flight = |[apogee, east, north]: [f64; 3], rate: [f64; 2], angle: [f64; 2]| {
            json!({
                "apogee_agl_m": apogee,
                "landing_east_m": east,
                "landing_north_m": north,
                "lower_button_leaves": {
                    "pitch_rate_nose_down_deg_s": rate[0],
                    "pitch_rate_nose_right_deg_s": rate[1],
                    "nose_down_deg": angle[0],
                    "nose_right_deg": angle[1],
                },
            })
        };
        let none = [0.0, 0.0];
        let moved = json!({
            "apogee_percent": 0.0,
            "landing_point_m": 1.0,
            "landing_point_percent_of_apogee": 0.1,
            "landing_range_m": 0.5,
        });
        json!({
            "solver": {"rtol": 1e-9},
            "drag": {"declared_cd0": [[0.0, 0.5]]},
            "crosswind": {"speed_mph": 20.0},
            "cases": [{
                "name": "a",
                "site": {"elevation": 0.0},
                "rail": {"rail_length_m": 5.0},
                "button_spacing_m": 0.5,
                "parachutes": ["main"],
                "winds": [{
                    "wind": "still",
                    "wind_east_m_s": 0.0,
                    "wind_north_m_s": 0.0,
                    "flown_with": {"dry_mass_kg": 10.0, "normal_force": {"slope_per_rad": [2.0, 3.0]}},
                    "phase_off": flight([900.0, 5.0, 5.0], none, none),
                    "phase_on": flight(on, rate, angle),
                    "guided_to_lower_button": flight(guided, none, none),
                    "moved": moved,
                    "moved_from_guided": moved,
                }],
            }],
        })
    }

    /// Between two pins a flight's figures are compared as M1.14b9's bars compare two codes':
    /// the turning rate and the angle as vectors of their two parts, the phase's move from the
    /// guided flight in the apogee and, as a vector, in the landing point, each as a share of
    /// the later pin's guided apogee. A guided flight that moved with the phase-on one leaves
    /// the phase's move where it was.
    #[test]
    fn between_two_pins_the_figures_are_compared_as_the_bars_compare_them() {
        let old = pin_of(
            [1000.0, 10.0, 20.0],
            [1010.0, 0.0, 0.0],
            [1.0, 0.0],
            [0.01, 0.0],
        );
        // Nothing moved: every figure is zero and the reading is the same.
        let still = since_the_earlier_pin(&json!({"flown": old}), &old).unwrap();
        assert_eq!(still["same_flights_past_the_bars"], true);
        assert_eq!(still["carried_largest_difference"], 0.0);
        for figure in [
            "turning_rate_deg_s",
            "angle_deg",
            "apogee_move_percent_of_apogee",
            "landing_point_move_percent_of_apogee",
        ] {
            assert_eq!(still["flights"][0][figure], 0.0, "{figure}");
            assert_eq!(still["largest"][figure]["value"], 0.0, "{figure}");
        }

        // The phase-on flight moves 2 m up, 3 m east and 4 m north, the guided one 2 m up and
        // not across: the apogee's move is unchanged and the landing point's moves 5 m, which
        // is 0.5% of the 1,000 m the later guided flight reaches. The rate moves by 0.3 and
        // 0.4 deg/s, 0.5 as a vector; the angle by 0.003° and 0.004°, 0.005°.
        let new = pin_of(
            [1002.0, 13.0, 24.0],
            [1000.0, 0.0, 0.0],
            [1.3, 0.4],
            [0.013, 0.004],
        );
        let old = pin_of(
            [1000.0, 10.0, 20.0],
            [998.0, 0.0, 0.0],
            [1.0, 0.0],
            [0.01, 0.0],
        );
        let since = since_the_earlier_pin(&json!({"flown": old}), &new).unwrap();
        let flight = &since["flights"][0];
        let close = |key: &str, expected: f64| {
            let got = number(flight, key).unwrap();
            assert!((got - expected).abs() < 1e-12, "{key}: {got}");
        };
        close("turning_rate_deg_s", 0.5);
        close("angle_deg", 0.005);
        close("apogee_move_percent_of_apogee", 0.0);
        close("landing_point_move_percent_of_apogee", 0.5);
        assert_eq!(flight["apogee_m"]["phase on"], 2.0);
        assert_eq!(flight["apogee_m"]["guided"], 2.0);
        assert_eq!(flight["apogee_m"]["phase off"], 0.0);
        assert_eq!(flight["landing_point_m"]["phase on"], 5.0);
        assert_eq!(flight["landing_point_m"]["guided"], 0.0);
        // The largest of a figure kept by flight names its flight; the first of two equal
        // ones stands.
        assert_eq!(since["largest"]["landing_point_m"]["flight"], "phase on");
        assert_eq!(since["largest"]["landing_point_m"]["value"], 5.0);
        assert_eq!(since["largest"]["apogee_m"]["flight"], "phase on");
        assert_eq!(since["largest"]["turning_rate_deg_s"]["case"], "a");
        assert!(
            since["largest"]["turning_rate_deg_s"]
                .get("flight")
                .is_none()
        );

        // Only the apogee's move changes: the guided flight alone gains 3 m, so the phase's
        // move falls by 3 m, −0.3% of the later guided apogee of 1,001 m.
        let lifted = pin_of(
            [1000.0, 10.0, 20.0],
            [1001.0, 0.0, 0.0],
            [1.0, 0.0],
            [0.01, 0.0],
        );
        let since = since_the_earlier_pin(&json!({"flown": old}), &lifted).unwrap();
        let got = number(&since["flights"][0], "apogee_move_percent_of_apogee").unwrap();
        assert!((got + 100.0 * 3.0 / 1001.0).abs() < 1e-12, "{got}");
    }

    /// The comparison between pins refuses two runs that did not fly the same thing: other
    /// solver settings, another rail, another wind, or a rocket that carried a different part
    /// list. A number that differs in what a rocket carried is measured, not refused.
    #[test]
    fn two_pins_that_flew_different_things_are_not_compared() {
        let old = pin_of(
            [1000.0, 10.0, 20.0],
            [1010.0, 0.0, 0.0],
            [1.0, 0.0],
            [0.01, 0.0],
        );
        let earlier = json!({"flown": old});
        let changed = |edit: &dyn Fn(&mut Value)| {
            let mut new = old.clone();
            edit(&mut new);
            since_the_earlier_pin(&earlier, &new)
        };
        let refused = |edit: &dyn Fn(&mut Value), what: &str| {
            let error = changed(edit).unwrap_err();
            assert!(error.contains(what), "{error}");
        };
        refused(&|new| new["solver"]["rtol"] = json!(1e-6), "`solver`");
        refused(
            &|new| new["drag"] = json!({"declared_cd0": [[0.0, 0.6]]}),
            "`drag`",
        );
        refused(
            &|new| new["crosswind"]["speed_mph"] = json!(10.0),
            "`crosswind`",
        );
        refused(
            &|new| new["cases"][0]["rail"]["rail_length_m"] = json!(6.0),
            "`rail`",
        );
        refused(
            &|new| new["cases"][0]["winds"][0]["wind"] = json!("crosswind"),
            "`wind`",
        );
        refused(
            &|new| new["cases"][0]["winds"][0]["declared"] = json!("no lag"),
            "`declared`",
        );
        refused(
            &|new| {
                new["cases"][0]["winds"][0]["flown_with"]["normal_force"]["slope_per_rad"] =
                    json!([2.0])
            },
            "differ in length",
        );
        refused(
            &|new| new["cases"][0]["winds"][0]["flown_with"]["motor"] = json!("another"),
            "differ in their keys",
        );
        refused(
            &|new| new["cases"] = json!([]),
            "different numbers of rockets",
        );
        let carried = changed(&|new| {
            new["cases"][0]["winds"][0]["flown_with"]["normal_force"]["slope_per_rad"][1] =
                json!(3.25);
        })
        .unwrap();
        assert_eq!(carried["carried_largest_difference"], 0.25);

        assert_eq!(largest_difference(&json!("a"), &json!("a")), Ok(0.0));
        assert!(largest_difference(&json!("a"), &json!("b")).is_err());
        assert!(largest_difference(&json!(1.0), &json!("1.0")).is_err());
        assert!(largest_difference(&json!({"a": 1.0}), &json!({"b": 1.0})).is_err());
    }

    /// The kept report is read only as the bytes its hash names: one figure edited, or a byte
    /// added at the end, and it is refused.
    #[test]
    fn an_edited_copy_of_the_earlier_report_is_refused() {
        let root = crate::designs::root().unwrap();
        let bytes = fs::read(root.join(EARLIER)).unwrap();
        assert!(earlier_of(&bytes).is_ok());
        let text = String::from_utf8(bytes.clone()).unwrap();
        let figure = "\"apogee_percent\": ";
        let at = text.find(figure).unwrap() + figure.len();
        let mut edited = bytes.clone();
        edited[at + 2] = if edited[at + 2] == b'1' { b'2' } else { b'1' };
        assert!(serde_json::from_slice::<Value>(&edited).is_ok());
        let mut longer = bytes.clone();
        longer.push(b'\n');
        for changed in [edited, longer, Vec::new()] {
            let error = earlier_of(&changed).unwrap_err();
            assert!(error.contains("is not the earlier pin's report"), "{error}");
        }
    }

    /// The sentence after the two bounds credits the farthest moves to the phase only when a
    /// flight with it moved farther than every flight without it; a tie is not the phase's.
    #[test]
    fn the_farthest_moves_are_the_phases_only_when_its_flights_moved_farthest() {
        let report = committed();
        let alone = "So the moves are not the phase's alone.";
        let farthest = "The flights with the phase moved the farthest.";
        let page = markdown(&report).unwrap();
        assert!(page.contains(alone) && !page.contains(farthest));
        let without_m = list(&report["since_the_earlier_pin"], "flights")
            .unwrap()
            .iter()
            .flat_map(|flight| {
                ["apogee_m", "landing_point_m"].map(|figure| {
                    ["phase off", "guided"]
                        .map(|way| number(&flight[figure], way).unwrap().abs())
                        .into_iter()
                        .fold(0.0, f64::max)
                })
            })
            .fold(0.0, f64::max);
        let with_move = |move_m: f64| {
            let mut moved = report.clone();
            moved["since_the_earlier_pin"]["flights"][0]["landing_point_m"]["phase on"] =
                json!(move_m);
            markdown(&moved).unwrap()
        };
        let page = with_move(1.0);
        assert!(page.contains(farthest) && !page.contains(alone));
        assert!(
            page.contains("over those with it, by up to 1000.00 mm. The flights with the phase")
        );
        let page = with_move(-without_m);
        assert!(page.contains(alone) && !page.contains(farthest));
    }

    /// A bound is rounded away from zero, never toward it, and an exact figure is left alone.
    #[test]
    fn a_bound_is_rounded_up() {
        assert_eq!(at_most(1.9159e-5, 6), "0.000020");
        assert_eq!(at_most(-1.9159e-5, 6), "0.000020");
        assert_eq!(at_most(1.035_846, 2), "1.04");
        assert_eq!(at_most(0.5, 1), "0.5");
        assert_eq!(at_most(2.0, 2), "2.00");
        assert_eq!(at_most(0.0, 2), "0.00");
        // Nearest rounding would understate each of these.
        assert_eq!(fixed(1.9159e-5, 6), "0.000019");
        assert_eq!(fixed(0.0413, 2), "0.04");
        assert_eq!(at_most(0.0413, 2), "0.05");
    }

    /// The earlier pin's report is the file the hash names, it flew the earlier commit with
    /// pull request #1196 applied here, and the committed report's comparison with it is
    /// recomputed from the two. What moved between the pins is far inside every bar HPR Sim's
    /// phase is held to: under a hundredth of each, so the earlier comparison's reading stands.
    /// The pages quote each bound rounded up ([`at_most`]).
    #[test]
    fn the_committed_report_compares_itself_with_the_earlier_pins() {
        let root = crate::designs::root().unwrap();
        let was = earlier(&root).unwrap();
        let before = &was["flown"];
        assert!(
            text(before, "oracle")
                .unwrap()
                .contains("ebe1b775cd9270c93018dc6d7867313d6eed4e5c (pull request #920's merge)")
        );
        assert!(text(before, "equations").unwrap().contains("#1196"));
        let report = committed();
        let pinned = crate::refs::pinned_commit(&root, LOCK_NAME).unwrap();
        assert_ne!(pinned, "ebe1b775cd9270c93018dc6d7867313d6eed4e5c");
        assert!(
            text(&report["flown"], "oracle")
                .unwrap()
                .contains("(pull request #1217's merge)")
        );
        let since = since_the_earlier_pin(&was, &report["flown"]).unwrap();
        assert!(
            crate::designs::same(&report["since_the_earlier_pin"], &since),
            "{:?}",
            crate::designs::difference(&report["since_the_earlier_pin"], &since)
        );
        assert_eq!(list(&since, "flights").unwrap().len(), 26);
        assert_eq!(since["same_flights_past_the_bars"], true);
        assert_eq!(since["carried_largest_difference"], 0.0);
        let most = |figure: &str| number(&since["largest"][figure], "value").unwrap().abs();
        use crate::tip_off_model::{
            ANGLE_FLOOR_DEG, APOGEE_MOVE_BAR_PERCENT, LANDING_MOVE_BAR_PERCENT, RATE_FLOOR_DEG_S,
        };
        assert!(most("turning_rate_deg_s") < 0.01 * RATE_FLOOR_DEG_S);
        assert!(most("angle_deg") < 0.01 * ANGLE_FLOOR_DEG);
        assert!(most("apogee_move_percent_of_apogee") < 0.01 * APOGEE_MOVE_BAR_PERCENT);
        assert!(most("landing_point_move_percent_of_apogee") < 0.01 * LANDING_MOVE_BAR_PERCENT);
        // No flight's apogee or landing point moved by as much as 2 mm.
        assert!(most("apogee_m") < 0.002);
        assert!(most("landing_point_m") < 0.002);
        // The page gives the same reading.
        let page = markdown(&report).unwrap();
        assert!(page.contains("the same flights are past each bar"));
        assert!(page.contains("Each rocket carried the same in both"));
        // The moves are not credited to the phase: the flights without it moved as far.
        let farthest = |ways: &[&str]| {
            list(&since, "flights")
                .unwrap()
                .iter()
                .flat_map(|flight| {
                    ["apogee_m", "landing_point_m"].map(|figure| {
                        ways.iter()
                            .map(|way| number(&flight[figure], way).unwrap().abs())
                            .fold(0.0, f64::max)
                    })
                })
                .fold(0.0, f64::max)
                * 1000.0
        };
        let (without, with) = (farthest(&["phase off", "guided"]), farthest(&["phase on"]));
        assert!(without >= with, "{without} {with}");
        assert!(page.contains(&format!(
            "an apogee or landing point moved by up to {} mm; over those with it, by up to {} mm",
            at_most(without, 2),
            at_most(with, 2)
        )));
        assert!(page.contains("So the moves are not the phase's alone."));
        assert!(page.contains("This report does not test which change made each."));
        assert!(page.contains(&format!("`{EARLIER}`")));
        assert!(page.contains(&format!("commit `{}`", &EARLIER_COMMIT[..8])));

        // The flight page and the decision record give those figures as the report has them.
        let words = |path: &str| {
            let text = fs::read_to_string(root.join(path)).unwrap();
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        };
        let rate = at_most(most("turning_rate_deg_s"), 6);
        let (apogee_mm, landing_mm) = (most("apogee_m") * 1000.0, most("landing_point_m") * 1000.0);
        let farthest_mm = apogee_mm.max(landing_mm);
        let flight_page = words("docs/physics/flight.md");
        for fragment in [
            format!("moved by at most {rate} deg/s"),
            format!(
                "by more than {} mm ({} in)",
                at_most(farthest_mm, 2),
                at_most(farthest_mm / 25.4, 3)
            ),
            "No figure quoted on this page changed".to_owned(),
            "under a hundredth of the bar".to_owned(),
            format!("kept as `{EARLIER}`"),
            format!("(`{}`), flies the thirteen", &pinned[..8]),
        ] {
            assert!(
                flight_page.contains(&fragment),
                "the flight page lacks `{fragment}`"
            );
        }
        let record = words(
            "docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md",
        );
        for fragment in [
            format!("turning rate moved by at most {rate} deg/s"),
            format!("by at most {}°", at_most(most("angle_deg"), 7)),
            format!(
                "at most {}% of the apogee in the apogee",
                at_most(most("apogee_move_percent_of_apogee"), 6)
            ),
            format!(
                "{}% of the apogee in the landing point",
                at_most(most("landing_point_move_percent_of_apogee"), 6)
            ),
            format!("apogee moved by more than {} mm", at_most(apogee_mm, 2)),
            format!("landing point by more than {} mm", at_most(landing_mm, 2)),
            format!("`{pinned}`"),
            "The same flights are past the plan's bars".to_owned(),
        ] {
            assert!(
                record.contains(&fragment),
                "the decision record lacks `{fragment}`"
            );
        }
        // "No figure on this page changed": every figure the flight page quotes from the
        // report reads the same from the earlier pin's, at the places the page gives them.
        let shown = |report: &Value| {
            let flown = &report["flown"];
            let guided = &flown["largest_from_guided"];
            let off = &flown["largest"];
            [
                signed(number(&guided["apogee"], "apogee_percent").unwrap(), 3),
                signed(number(&guided["apogee"], "apogee_m").unwrap(), 1),
                fixed(
                    number(&guided["landing_point"], "landing_point_percent_of_apogee").unwrap(),
                    3,
                ),
                fixed(
                    number(&guided["landing_point"], "landing_point_m").unwrap(),
                    1,
                ),
                fixed(
                    number(&off["landing_point"], "landing_point_percent_of_apogee").unwrap(),
                    3,
                ),
                signed(number(&off["apogee"], "apogee_percent").unwrap(), 3),
            ]
        };
        assert_eq!(shown(&report), shown(&was));
    }
}
