//! What RocketPy's tip-off phase moves on its example rockets (M1.14b5), written by
//! `cargo xtask tip-off` to `validation/reports/tip-off.json` and `.md`.
//!
//! - **The question.** HPR Sim guides a rocket until its last guide leaves the rail and frees it
//!   with no turning rate. A real rocket pivots about its last guide once the one ahead is off
//!   the rail: tip-off. This report measures what that pivot is worth, with RocketPy's own
//!   tip-off phase, so the flight page can say what leaving it out costs.
//! - **The oracle** is `validation/oracles/rocketpy/tip_off.py`: RocketPy's `develop` at the
//!   merge of its pull request #920 (pinned in `validation/refs.lock.toml` as
//!   `rocketpy-tip-off`), flying each example rocket with the phase off and on, in still air and
//!   in a 20 mph crosswind, and a third time guided to where the lower button leaves, which is
//!   HPR Sim's release to within half a button's length (HPR Sim waits for the button's aft
//!   edge). Its docstring states what it declares.
//! - **The bars** are the plan's: a tip-off model is called for when the phase moves any apogee
//!   by more than 1%, or any landing point by more than 1% of the apogee
//!   ([`APOGEE_BAR_PERCENT`], [`LANDING_BAR_PERCENT_OF_APOGEE`]).
//! - **Two runs and a probe.** The first run flies the commit's equations with RocketPy's pull
//!   request #1196, as every whole flight of the suite does; the second flies them as pinned,
//!   to show the correction does not decide whether a model is called for. The report keeps
//!   the first whole and the second's largest moves and reading. The probe flies, with its
//!   parachute's lag, the one rocket and wind the runs fly without it, and the report keeps
//!   that it had not finished in the probe's wall time.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

pub const USAGE: &str = "\
  tip-off [--check]        Fly RocketPy's example rockets with its tip-off phase off and on
                           (validation/oracles/rocketpy/tip_off.py, on the checkout pinned as
                           rocketpy-tip-off under refs/) and write what the phase moves to
                           validation/reports/tip-off.json and .md. --check fails if the
                           committed report differs instead of writing.";

const ORACLE: &str = "validation/oracles/rocketpy/tip_off.py";
const REPORT: &str = "validation/reports/tip-off.json";
const REPORT_MD: &str = "validation/reports/tip-off.md";
/// The checkout's name in `validation/refs.lock.toml`.
const LOCK_NAME: &str = "rocketpy-tip-off";

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

fn number(value: &Value, key: &str) -> Result<f64, String> {
    value[key]
        .as_f64()
        .filter(|x| x.is_finite())
        .ok_or_else(|| format!("no number `{key}` in {value}"))
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("no text `{key}` in {value}"))
}

fn list<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
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

/// The report from the two runs' documents.
fn report(flown: Value, as_pinned: &Value, lag_probe: &Value) -> Result<Value, String> {
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
        "as_pinned": {
            "command": as_pinned["command"],
            "equations": as_pinned["equations"],
            "largest": as_pinned["largest"],
            "largest_from_guided": as_pinned["largest_from_guided"],
            "reading": reading(as_pinned)?,
        },
        "lag_probe": probes,
        "flown": flown,
    }))
}

/// Flies both runs and the probe. Needs the oracle environment and the pinned checkout under
/// `refs/`.
pub fn generate(root: &Path) -> Result<Value, String> {
    let flown = oracle(root, None)?;
    let as_pinned = oracle(root, Some("--as-pinned"))?;
    let lag_probe = oracle(root, Some("--lag-probe"))?;
    let pinned = crate::refs::pinned_commit(root, LOCK_NAME)?;
    for run in [&flown, &as_pinned] {
        if !text(run, "oracle")?.contains(&pinned) {
            return Err(format!(
                "{ORACLE} flew {}, not the pinned {pinned}",
                run["oracle"]
            ));
        }
    }
    report(flown, &as_pinned, &lag_probe)
}

/// `value` to `decimals` places, a value that rounds to zero without its sign.
fn fixed(value: f64, decimals: usize) -> String {
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
fn signed(value: f64, decimals: usize) -> String {
    let shown = fixed(value, decimals);
    if value > 0.0 && shown.chars().any(|c| ('1'..='9').contains(&c)) {
        format!("+{shown}")
    } else {
        shown
    }
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
    let as_pinned = &report["as_pinned"];
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
         release, and no flight checks it. HPR Sim frees a rocket with no turning rate where its last guide leaves the rail. This \
         report measures the pivot about that guide which it leaves out, with RocketPy's tip-off \
         phase: {}. Equations: {}. Each example rocket of \
         `validation/fixtures/design/rocketpy-rocket-mass.json` flies from its example's rail and \
         site on a declared constant drag coefficient, in still air and in a {} mph ({} m/s) wind \
         {}.\n",
        text(flown, "oracle")?,
        text(flown, "equations")?,
        fixed(number(crosswind, "speed_mph")?, 0),
        fixed(number(crosswind, "speed_m_s")?, 2),
        text(crosswind, "direction")?,
    );
    let _ = writeln!(w, "How to read it:\n");
    let _ = writeln!(
        w,
        "- **Phase off** is RocketPy as released: the rocket is free where its upper button \
         leaves the rail.\n\
         - **Phase on** adds the tip-off phase: from there the rocket slides on along the rail, \
         free to pitch and yaw about its lower button with roll held, until that button leaves \
         too.\n\
         - **Guided** is the phase off on a rail longer by the distance between the buttons: the \
         rocket is held straight until the lower button leaves the real rail, as HPR Sim holds \
         it.\n\
         - **Moved** is the phase on less the other flight. A landing point's move is the \
         distance between the two landing points, given in meters and as a share of the other \
         flight's apogee; **farther from the pad** is how much farther from the pad the rocket \
         lands with the phase on, negative when it lands nearer.\n\
         - **Where the lower button leaves:** the angle between the rocket's axis and the \
         rail's, its part below the rail's line in the rail's vertical plane (nose down), and \
         the turning rate across the rocket's axis (pitch and yaw together).\n\
         - **The bars** are the plan's: a tip-off model is called for when the phase moves any \
         apogee by more than {}%, or any landing point by more than {}% of the apogee.\n",
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
    let _ = writeln!(
        w,
        "- **The equations as pinned** (`{}`), on less off: {}; the phase moves {}. On less \
         guided: {}; the phase moves {}.",
        text(as_pinned, "command")?,
        past_line(&as_pinned["reading"]["moved"])?,
        largest_line(as_pinned, "largest")?,
        past_line(&as_pinned["reading"]["moved_from_guided"])?,
        largest_line(as_pinned, "largest_from_guided")?,
    );
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            if let Some(declared) = wind.get("declared") {
                let probe = list(report, "lag_probe")?
                    .iter()
                    .find(|probe| probe["case"] == case["name"] && probe["wind"] == wind["wind"])
                    .ok_or("a declared change with no probe")?;
                let _ = writeln!(
                    w,
                    "- **A declared change** ({}, {}): {}. That is the probe's result \
                     (`tip_off.py --lag-probe`, {} s of wall time): its integration had \
                     reached {} s, and the apogee is at {} s.",
                    text(case, "name")?,
                    text(wind, "wind")?,
                    declared.as_str().ok_or("`declared` is not text")?,
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
    Ok(page)
}

pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
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
    /// and the decision record's table and its sentence on the equations as pinned. Not held:
    /// the record's other figures.
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
                "in {} of the 26 flights on three rockets: Valetudo Juno III and Prometheus 2022",
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
                    "in still air) by {} mm ({} in)",
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
        let pinned = &report["as_pinned"];
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
            "the first run flies the corrected equations"
        );
        assert_eq!(
            report["as_pinned"]["equations"],
            "as the pinned commit has them"
        );

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
}
