//! HPR Sim's tip-off phase held to RocketPy's (M1.14b9), written by `cargo xtask tip-off-model`
//! to `validation/reports/tip-off-model.json` and `.md`.
//!
//! - **The question.** HPR Sim's tip-off phase ([`hpr_sim::Simulation::with_tip_off`]) lets a
//!   rocket pivot about its last guide once the guide ahead of it has left the rail. RocketPy's
//!   development branch has the same phase. This report flies both on the same thirteen designs,
//!   in still air and in a crosswind, and sets the two beside each other.
//! - **The reference** is `validation/reports/tip-off.json`, which `cargo xtask tip-off` writes
//!   from RocketPy's flights: for each rocket and wind, RocketPy guided to its lower button and
//!   with its phase on, and everything the flights carried.
//! - **HPR Sim's flights** are [`hpr_validate::tip_off::fly`]'s: the public design built from
//!   the same example, on that record's rail, site, wind, parachutes and drag coefficient,
//!   guided as HPR Sim flies by default and with its phase. Each is flown twice: on RocketPy's
//!   normal force, which the record holds, and on HPR Sim's own.
//! - **The bars** are the tip-off decision record's ([ADR-357][adr-357], decision 4): where the
//!   last guide leaves, the turning rate within 10% of RocketPy's or 0.1 deg/s and the angle off
//!   the rail within 10% or 0.005°, each as a vector of its nose-down and nose-right parts; the
//!   two codes' apogee moves within 0.2% of RocketPy's guided apogee; and their landing-point
//!   moves, as vectors, within 1% of it.
//! - **Which flights are held to the bars** is the next record's ([ADR-358][adr-358]): all 26 on
//!   RocketPy's normal force, where the phase meets one drag and one normal force in both
//!   codes, and the 13 in still
//!   air on HPR Sim's own. The 13 crosswind flights on HPR Sim's own normal force are past the
//!   bars: they stay in the report with every figure past a bar marked, and are an open
//!   increment of the roadmap.
//!
//! [adr-357]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
//! [adr-358]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_validate::tip_off::{Flown, NormalForce, fly};
use serde_json::{Value, json};

use crate::tip_off::{fixed, list, number, signed, text};

pub const USAGE: &str = "\
  tip-off-model [--check]  Fly HPR Sim's tip-off phase on the thirteen designs of
                           validation/reports/tip-off.json, on RocketPy's normal force and on
                           its own, and write the comparison with RocketPy's phase to
                           validation/reports/tip-off-model.json and .md. --check fails if the
                           committed report is not this run's instead of writing.";

const ORACLE_REPORT: &str = "validation/reports/tip-off.json";
const REPORT: &str = "validation/reports/tip-off-model.json";
const REPORT_MD: &str = "validation/reports/tip-off-model.md";

/// The turning rate where the last guide leaves may differ from RocketPy's, as a vector, by this
/// share of RocketPy's rate,
pub(crate) const RATE_FRACTION: f64 = 0.10;
/// or by this, deg/s, whichever is larger.
pub(crate) const RATE_FLOOR_DEG_S: f64 = 0.1;
/// The angle off the rail's axis may differ, as a vector, by this share of RocketPy's angle,
pub(crate) const ANGLE_FRACTION: f64 = 0.10;
/// or by this, degrees, whichever is larger.
pub(crate) const ANGLE_FLOOR_DEG: f64 = 0.005;
/// The two codes' apogee moves may differ by this, percent of RocketPy's guided apogee.
pub(crate) const APOGEE_MOVE_BAR_PERCENT: f64 = 0.2;
/// Their landing-point moves, as vectors, by this, percent of RocketPy's guided apogee.
pub(crate) const LANDING_MOVE_BAR_PERCENT: f64 = 1.0;
/// A landing point that the phase moves toward or away from the pad by less than this, m, is
/// counted as landing the same distance from it: the vertical rail in still air moves 6 mm.
pub(crate) const PAD_DEAD_BAND_M: f64 = 0.1;

/// The two normal forces each flight is flown on, by the report's key.
const FORCES: [(&str, NormalForce); 2] = [
    ("recorded_normal_force", NormalForce::Recorded),
    ("own_normal_force", NormalForce::Own),
];
/// The figures held to a bar, by the report's key and the words for them.
const FIGURES: [(&str, &str); 4] = [
    ("pitch_rate", "turning rate"),
    ("angle", "angle off the rail"),
    ("apogee_move", "apogee move"),
    ("landing_move", "landing-point move"),
];

fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// RocketPy's side of one rocket and wind: its guided apogee, the state where its lower button
/// leaves with the phase on, and what the phase moves from its guided flight.
fn rocketpy_row(wind: &Value) -> Result<Value, String> {
    let (on, guided) = (&wind["phase_on"], &wind["guided_to_lower_button"]);
    let release = &on["lower_button_leaves"];
    let (east, north) = (
        number(on, "landing_east_m")?,
        number(on, "landing_north_m")?,
    );
    let (guided_east, guided_north) = (
        number(guided, "landing_east_m")?,
        number(guided, "landing_north_m")?,
    );
    let mut state = serde_json::Map::new();
    for key in [
        "time_s",
        "speed_m_s",
        "nose_down_deg",
        "nose_right_deg",
        "pitch_rate_nose_down_deg_s",
        "pitch_rate_nose_right_deg_s",
    ] {
        state.insert(key.to_owned(), json!(number(release, key)?));
    }
    Ok(json!({
        "guided_apogee_agl_m": number(guided, "apogee_agl_m")?,
        "last_guide_leaves": state,
        "moved": {
            "apogee_m": number(on, "apogee_agl_m")? - number(guided, "apogee_agl_m")?,
            "landing_east_m": east - guided_east,
            "landing_north_m": north - guided_north,
            "landing_range_m": east.hypot(north) - guided_east.hypot(guided_north),
        },
    }))
}

/// What a flight with the phase moves from the guided one.
fn moved(guided: &Flown, tipped: &Flown) -> Value {
    json!({
        "apogee_m": tipped.apogee_agl_m - guided.apogee_agl_m,
        "landing_east_m": tipped.landing_east_m - guided.landing_east_m,
        "landing_north_m": tipped.landing_north_m - guided.landing_north_m,
        "landing_range_m": tipped.landing_east_m.hypot(tipped.landing_north_m)
            - guided.landing_east_m.hypot(guided.landing_north_m),
    })
}

/// How far HPR Sim's figures are from RocketPy's, each beside its bar, and which are past it.
/// `release` is HPR Sim's state where the last guide leaves and `moved` what its phase moves;
/// `rocketpy` is [`rocketpy_row`]'s.
fn differences(release: &Value, moved: &Value, rocketpy: &Value) -> Result<Value, String> {
    let theirs = &rocketpy["last_guide_leaves"];
    let apart = |down: &str, right: &str| -> Result<(f64, f64), String> {
        let (d, r) = (number(theirs, down)?, number(theirs, right)?);
        Ok((
            (number(release, down)? - d).hypot(number(release, right)? - r),
            d.hypot(r),
        ))
    };
    let (rate, their_rate) = apart("pitch_rate_nose_down_deg_s", "pitch_rate_nose_right_deg_s")?;
    let (angle, their_angle) = apart("nose_down_deg", "nose_right_deg")?;
    let apogee_m = number(rocketpy, "guided_apogee_agl_m")?;
    let their_move = &rocketpy["moved"];
    let apogee_move =
        100.0 * (number(moved, "apogee_m")? - number(their_move, "apogee_m")?).abs() / apogee_m;
    let landing_move = 100.0
        * (number(moved, "landing_east_m")? - number(their_move, "landing_east_m")?)
            .hypot(number(moved, "landing_north_m")? - number(their_move, "landing_north_m")?)
        / apogee_m;
    let figures = [
        (
            rate,
            (RATE_FRACTION * their_rate).max(RATE_FLOOR_DEG_S),
            "deg_s",
        ),
        (
            angle,
            (ANGLE_FRACTION * their_angle).max(ANGLE_FLOOR_DEG),
            "deg",
        ),
        (apogee_move, APOGEE_MOVE_BAR_PERCENT, "percent_of_apogee"),
        (landing_move, LANDING_MOVE_BAR_PERCENT, "percent_of_apogee"),
    ];
    let mut out = serde_json::Map::new();
    let mut past = Vec::new();
    for ((key, _), (apart, bar, unit)) in FIGURES.iter().zip(figures) {
        out.insert(format!("{key}_{unit}"), json!(apart));
        out.insert(format!("{key}_bar_{unit}"), json!(bar));
        // A figure that is not a number is past its bar.
        if apart.is_nan() || apart > bar {
            past.push(json!(key));
        }
    }
    out.insert("past".to_owned(), Value::Array(past));
    Ok(Value::Object(out))
}

/// HPR Sim's side of one rocket and wind on one normal force: the guided flight, the flight with
/// the phase, what the phase moves, and how far that is from RocketPy's.
fn hpr_row(
    root: &Path,
    (case, wind, declared_cd0): (&Value, &Value, &Value),
    force: NormalForce,
    rocketpy: &Value,
) -> Result<Value, String> {
    let guided = fly(root, case, wind, declared_cd0, (force, false)).map_err(|e| e.to_string())?;
    let tipped = fly(root, case, wind, declared_cd0, (force, true)).map_err(|e| e.to_string())?;
    let value = |flown: &Flown| serde_json::to_value(flown).map_err(|e| e.to_string());
    let (guided_value, tipped_value) = (value(&guided)?, value(&tipped)?);
    let moved = moved(&guided, &tipped);
    let differences = differences(&tipped_value["last_guide_leaves"], &moved, rocketpy)?;
    Ok(json!({
        "guided": guided_value,
        "tip_off": tipped_value,
        "moved": moved,
        "differences": differences,
    }))
}

/// The unit a figure's difference is recorded in.
fn unit(figure: &str) -> &'static str {
    match figure {
        "pitch_rate" => "deg_s",
        "angle" => "deg",
        _ => "percent_of_apogee",
    }
}

/// How the landing points moved against the pad, with [`PAD_DEAD_BAND_M`]: how many flights land
/// farther from it with the phase, nearer, and the same distance.
fn pad_counts<'a>(moves: impl Iterator<Item = &'a Value>) -> Result<Value, String> {
    let (mut farther, mut nearer, mut same) = (0, 0, 0);
    for moved in moves {
        let range = number(moved, "landing_range_m")?;
        if range > PAD_DEAD_BAND_M {
            farther += 1;
        } else if range < -PAD_DEAD_BAND_M {
            nearer += 1;
        } else {
            same += 1;
        }
    }
    Ok(json!({ "farther": farther, "nearer": nearer, "same": same }))
}

/// The reading of `flights` on one normal force (`key`): how many flights are inside every bar,
/// in all and by wind, each figure past a bar, the largest difference of each figure, and the
/// landing points against the pad.
fn reading_of(flights: &[Value], key: &str) -> Result<Value, String> {
    let mut past = serde_json::Map::new();
    let mut largest = serde_json::Map::new();
    for (figure, _) in FIGURES {
        let unit = unit(figure);
        let entry = |flight: &Value| -> Result<Value, String> {
            let differences = &flight[key]["differences"];
            Ok(json!({
                "case": text(flight, "case")?,
                "wind": text(flight, "wind")?,
                "apart": number(differences, &format!("{figure}_{unit}"))?,
                "bar": number(differences, &format!("{figure}_bar_{unit}"))?,
            }))
        };
        let mut rows = Vec::new();
        let mut worst: Option<Value> = None;
        for flight in flights {
            let row = entry(flight)?;
            if list(&flight[key]["differences"], "past")?
                .iter()
                .any(|name| name == figure)
            {
                rows.push(row.clone());
            }
            let apart = number(&row, "apart")?;
            if worst
                .as_ref()
                .is_none_or(|w| w["apart"].as_f64().is_none_or(|x| apart > x))
            {
                worst = Some(row);
            }
        }
        past.insert(figure.to_owned(), Value::Array(rows));
        largest.insert(figure.to_owned(), worst.unwrap_or(Value::Null));
    }
    let inside = |wind: Option<&str>| -> Result<Value, String> {
        let mut flown = 0;
        let mut inside = 0;
        for flight in flights {
            if wind.is_some_and(|wind| flight["wind"] != wind) {
                continue;
            }
            flown += 1;
            if list(&flight[key]["differences"], "past")?.is_empty() {
                inside += 1;
            }
        }
        Ok(json!({ "flights": flown, "inside_every_bar": inside }))
    };
    Ok(json!({
        "all": inside(None)?,
        "still": inside(Some("still"))?,
        "crosswind": inside(Some("crosswind"))?,
        "past": past,
        "largest": largest,
        "landing_against_the_pad": pad_counts(flights.iter().map(|flight| &flight[key]["moved"]))?,
    }))
}

/// How far apart the two normal forces are where the last guide leaves in the crosswind, and what
/// that does to the turn: the least and greatest ratio of HPR Sim's own normal-force coefficient
/// to RocketPy's, and how much farther nose-right (downwind) the rocket turns on HPR Sim's own
/// normal force than on RocketPy's, both flown by HPR Sim, so the phase is the same in both.
fn crosswind_cause(flights: &[Value]) -> Result<Value, String> {
    let (recorded, own) = (FORCES[0].0, FORCES[1].0);
    let (mut ratios, mut rates) = (Vec::new(), Vec::new());
    for flight in flights
        .iter()
        .filter(|flight| flight["wind"] == "crosswind")
    {
        let release = |key: &str| &flight[key]["tip_off"]["last_guide_leaves"];
        ratios.push(
            number(release(own), "normal_force_coefficient")?
                / number(release(recorded), "normal_force_coefficient")?,
        );
        rates.push(
            number(release(own), "pitch_rate_nose_right_deg_s")?
                - number(release(recorded), "pitch_rate_nose_right_deg_s")?,
        );
    }
    let least = |values: &[f64]| values.iter().copied().fold(f64::INFINITY, f64::min);
    let greatest = |values: &[f64]| values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if ratios.is_empty() || ratios.iter().chain(&rates).any(|x| !x.is_finite()) {
        return Ok(Value::Null);
    }
    Ok(json!({
        "flights": ratios.len(),
        "coefficient_ratio_least": least(&ratios),
        "coefficient_ratio_greatest": greatest(&ratios),
        "farther_nose_right": rates.iter().filter(|&&rate| rate > 0.0).count(),
        "nose_right_rate_more_least_deg_s": least(&rates),
        "nose_right_rate_more_greatest_deg_s": greatest(&rates),
    }))
}

/// The whole reading: each normal force's, RocketPy's landing points against the pad, and the
/// two things ADR-358 holds: every figure of every flight on RocketPy's normal force inside its
/// bar, and every figure of every still-air flight on HPR Sim's own.
fn reading(flights: &[Value]) -> Result<Value, String> {
    let (recorded, own) = (
        reading_of(flights, FORCES[0].0)?,
        reading_of(flights, FORCES[1].0)?,
    );
    // A set with no flights in it holds nothing.
    let all_inside =
        |part: &Value| part["inside_every_bar"] == part["flights"] && part["flights"] != 0;
    let (held, held_still) = (all_inside(&recorded["all"]), all_inside(&own["still"]));
    Ok(json!({
        FORCES[0].0: recorded,
        FORCES[1].0: own,
        "rocketpy_landing_against_the_pad":
            pad_counts(flights.iter().map(|flight| &flight["rocketpy"]["moved"]))?,
        "held_on_the_recorded_normal_force": held,
        "held_in_still_air_on_its_own_normal_force": held_still,
        "crosswind_normal_force": crosswind_cause(flights)?,
    }))
}

/// Flies every flight and builds the report. Needs only committed files.
pub fn generate(root: &Path) -> Result<Value, String> {
    let bytes = fs::read(root.join(ORACLE_REPORT)).map_err(|e| format!("{ORACLE_REPORT}: {e}"))?;
    let oracle: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("{ORACLE_REPORT}: {e}"))?;
    let flown = &oracle["flown"];
    let declared_cd0 = &flown["drag"]["declared_cd0"];
    let mut flights = Vec::new();
    for case in list(flown, "cases")? {
        for wind in list(case, "winds")? {
            let rocketpy = rocketpy_row(wind)?;
            let mut flight = json!({
                "case": text(case, "name")?,
                "wind": text(wind, "wind")?,
                "rail_inclination_deg": number(&case["rail"], "inclination_deg")?,
                "rocketpy": rocketpy,
            });
            for (key, force) in FORCES {
                flight[key] = hpr_row(root, (case, wind, declared_cd0), force, &rocketpy)?;
            }
            flights.push(flight);
        }
    }
    let reading = reading(&flights)?;
    Ok(json!({
        "generator": "cargo xtask tip-off-model",
        "evaluation": "HPR Sim's tip-off phase against RocketPy's: each rocket and wind of the \
                       reference flown in HPR Sim guided to its last guide and with the phase, \
                       on RocketPy's normal force as the reference records it and on HPR Sim's \
                       own; moved is the flight with the phase less the guided one, and \
                       differences are HPR Sim's figures less RocketPy's, each beside its bar",
        "reference": {
            "report": ORACLE_REPORT,
            "sha256": sha256(&bytes),
            "oracle": text(flown, "oracle")?,
            "equations": text(flown, "equations")?,
        },
        "bars": {
            "pitch_rate_fraction": RATE_FRACTION,
            "pitch_rate_floor_deg_s": RATE_FLOOR_DEG_S,
            "angle_fraction": ANGLE_FRACTION,
            "angle_floor_deg": ANGLE_FLOOR_DEG,
            "apogee_move_percent_of_apogee": APOGEE_MOVE_BAR_PERCENT,
            "landing_move_percent_of_apogee": LANDING_MOVE_BAR_PERCENT,
            "judged_on": FORCES[0].0,
        },
        "pad_dead_band_m": PAD_DEAD_BAND_M,
        "flights": flights,
        "reading": reading,
    }))
}

/// `(down, right)` as a pair of signed figures.
fn pair(value: &Value, down: &str, right: &str, decimals: usize) -> Result<String, String> {
    Ok(format!(
        "{}, {}",
        signed(number(value, down)?, decimals),
        signed(number(value, right)?, decimals)
    ))
}

/// A difference beside its bar, in bold when it is past it.
fn against_bar(differences: &Value, figure: &str, decimals: usize) -> Result<String, String> {
    let unit = unit(figure);
    let apart = fixed(number(differences, &format!("{figure}_{unit}"))?, decimals);
    let bar = fixed(
        number(differences, &format!("{figure}_bar_{unit}"))?,
        decimals,
    );
    let past = list(differences, "past")?.iter().any(|name| name == figure);
    Ok(if past {
        format!("**{apart}** of {bar}")
    } else {
        format!("{apart} of {bar}")
    })
}

/// One normal force's line of the reading.
fn reading_line(part: &Value) -> Result<String, String> {
    let count = |wind: &str| -> Result<String, String> {
        Ok(format!(
            "{} of {}",
            number(&part[wind], "inside_every_bar")?,
            number(&part[wind], "flights")?
        ))
    };
    let mut line = format!(
        "{} flights inside every bar ({} in still air, {} in the crosswind).",
        count("all")?,
        count("still")?,
        count("crosswind")?
    );
    for (figure, words) in FIGURES {
        let rows = list(&part["past"], figure)?;
        let worst = &part["largest"][figure];
        let decimals = if figure == "angle" { 4 } else { 3 };
        let symbol = match figure {
            "pitch_rate" => " deg/s",
            "angle" => "°",
            _ => "% of the apogee",
        };
        write!(
            line,
            " The {words}: {} past its bar; the largest difference is {}{symbol} against a bar \
             of {}{symbol} ({}, {}).",
            rows.len(),
            fixed(number(worst, "apart")?, decimals),
            fixed(number(worst, "bar")?, decimals),
            text(worst, "case")?,
            text(worst, "wind")?,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(line)
}

/// A landing count in words.
fn pad_line(counts: &Value) -> Result<String, String> {
    Ok(format!(
        "farther from the pad on {}, nearer on {} and the same distance on {}",
        number(counts, "farther")?,
        number(counts, "nearer")?,
        number(counts, "same")?
    ))
}

/// The two tables of one normal force: where the last guide leaves, and what the phase moves.
fn tables(out: &mut String, flights: &[Value], key: &str) -> Result<(), String> {
    out.push_str(
        "| Rocket | Wind | Turning rate, HPR Sim (down, right) | RocketPy | Apart, deg/s | Angle \
         off the rail, HPR Sim (down, right) | RocketPy | Apart, ° |\n\
         |---|---|--:|--:|--:|--:|--:|--:|\n",
    );
    for flight in flights {
        let ours = &flight[key];
        let (release, theirs) = (
            &ours["tip_off"]["last_guide_leaves"],
            &flight["rocketpy"]["last_guide_leaves"],
        );
        let (rate_down, rate_right) = ("pitch_rate_nose_down_deg_s", "pitch_rate_nose_right_deg_s");
        writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            text(flight, "case")?,
            text(flight, "wind")?,
            pair(release, rate_down, rate_right, 3)?,
            pair(theirs, rate_down, rate_right, 3)?,
            against_bar(&ours["differences"], "pitch_rate", 3)?,
            pair(release, "nose_down_deg", "nose_right_deg", 4)?,
            pair(theirs, "nose_down_deg", "nose_right_deg", 4)?,
            against_bar(&ours["differences"], "angle", 4)?,
        )
        .map_err(|e| e.to_string())?;
    }
    out.push_str(
        "\n| Rocket | Wind | Apogee moved, HPR Sim | RocketPy | Apart, % of the apogee | \
         Landing point moved, HPR Sim (east, north) | RocketPy | Apart, % of the apogee | \
         Farther from the pad, HPR Sim | RocketPy |\n\
         |---|---|--:|--:|--:|--:|--:|--:|--:|--:|\n",
    );
    for flight in flights {
        let ours = &flight[key];
        let (moved, theirs) = (&ours["moved"], &flight["rocketpy"]["moved"]);
        writeln!(
            out,
            "| {} | {} | {} m | {} m | {} | {} m | {} m | {} | {} m | {} m |",
            text(flight, "case")?,
            text(flight, "wind")?,
            signed(number(moved, "apogee_m")?, 2),
            signed(number(theirs, "apogee_m")?, 2),
            against_bar(&ours["differences"], "apogee_move", 3)?,
            pair(moved, "landing_east_m", "landing_north_m", 1)?,
            pair(theirs, "landing_east_m", "landing_north_m", 1)?,
            against_bar(&ours["differences"], "landing_move", 3)?,
            signed(number(moved, "landing_range_m")?, 1),
            signed(number(theirs, "landing_range_m")?, 1),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// The report as a page.
pub fn markdown(report: &Value) -> Result<String, String> {
    let flights = list(report, "flights")?;
    let reading = &report["reading"];
    let (recorded, own) = (FORCES[0].0, FORCES[1].0);
    let mut out = String::from("# HPR Sim's tip-off phase against RocketPy's\n\n");
    write!(
        out,
        "Written by `cargo xtask tip-off-model`; don't edit it by hand. What it measures and how \
         far to trust it: [rigid-body flight](../../docs/physics/flight.md#tip-off). It is one \
         code against another, on {} flights of thirteen designs; RocketPy's phase is in its \
         development branch, in no release, and no real flight checks either. RocketPy's side \
         is [its own report](tip-off.md) ({}; {}).\n\n",
        flights.len(),
        text(&report["reference"], "oracle")?,
        text(&report["reference"], "equations")?,
    )
    .map_err(|e| e.to_string())?;
    out.push_str(
        "How to read it:\n\n\
         - **Guided** is HPR Sim as it flies by default: held straight until its last guide \
         leaves the rail. **Tip-off** adds the phase: once the guide ahead of the last one is \
         off the rail, the rocket slides on, free to pitch and yaw about its last guide with \
         roll held. **Moved** is the flight with the phase less the guided one, in each code \
         against its own guided flight.\n\
         - **On RocketPy's normal force,** HPR Sim flies the normal force RocketPy flew, which \
         the reference records for each rocket: a slope against Mach number times the angle of \
         attack, at a center of pressure against Mach number. Both codes already fly one \
         declared drag coefficient. So the phase meets one drag and one normal force in both \
         codes; damping in pitch and yaw stays each code's own. All of these are held to the \
         bars.\n\
         - **On its own normal force,** HPR Sim flies its own aerodynamics. In the crosswind \
         the rocket meets the air at 9.7° to 25.6° as it leaves the rail, where HPR Sim's normal \
         force and RocketPy's are far apart, and the turn about the last guide follows the \
         force. The still-air flights are held to the bars. The crosswind flights are past \
         them and are not held: they are open work \
         ([the decision record](../../docs/decisions/0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md)). \
         Every figure past a bar is in bold.\n\
         - **Where the last guide leaves:** the rate at which the nose's direction moves and \
         the angle of the rocket's axis off the rail's, each as two parts: nose down, below \
         the rail's line in the rail's vertical plane, and nose right, to the right of the \
         rail's heading, which is downwind in the crosswind. **Apart** is the length of the \
         difference of the two codes' pairs, beside its bar.\n\
         - **A landing point's move** is east and north. **Farther from the pad** is how much \
         farther from the pad the rocket lands with the phase, negative when nearer.\n",
    );
    writeln!(
        out,
        "- **The bars:** the turning rate within {}% of RocketPy's or {} deg/s, whichever is \
         larger; the angle within {}% or {}°; the apogee moves within {}% of RocketPy's guided \
         apogee; the landing-point moves, as vectors, within {}% of it.",
        fixed(100.0 * RATE_FRACTION, 0),
        fixed(RATE_FLOOR_DEG_S, 1),
        fixed(100.0 * ANGLE_FRACTION, 0),
        fixed(ANGLE_FLOOR_DEG, 3),
        fixed(APOGEE_MOVE_BAR_PERCENT, 1),
        fixed(LANDING_MOVE_BAR_PERCENT, 0),
    )
    .map_err(|e| e.to_string())?;

    out.push_str("\n## The reading\n\n");
    writeln!(
        out,
        "- **On RocketPy's normal force:** {}",
        reading_line(&reading[recorded])?
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        out,
        "- **On HPR Sim's own normal force:** {}",
        reading_line(&reading[own])?
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        out,
        "- **Which way leaving the phase out errs,** counting a move of under {} m toward or \
         away from the pad as none: with the phase HPR Sim lands {} on RocketPy's normal force, \
         and {} on its own; RocketPy lands {}.",
        fixed(PAD_DEAD_BAND_M, 1),
        pad_line(&reading[recorded]["landing_against_the_pad"])?,
        pad_line(&reading[own]["landing_against_the_pad"])?,
        pad_line(&reading["rocketpy_landing_against_the_pad"])?,
    )
    .map_err(|e| e.to_string())?;
    writeln!(
        out,
        "- **Held:** {}; {}.",
        if reading["held_on_the_recorded_normal_force"] == true {
            "every figure on RocketPy's normal force is inside its bar"
        } else {
            "no: a figure on RocketPy's normal force is past its bar"
        },
        if reading["held_in_still_air_on_its_own_normal_force"] == true {
            "every still-air figure on HPR Sim's own normal force is inside its bar"
        } else {
            "a still-air figure on HPR Sim's own normal force is past its bar"
        }
    )
    .map_err(|e| e.to_string())?;

    out.push_str("\n## On RocketPy's normal force\n\n");
    tables(&mut out, flights, recorded)?;
    out.push_str("\n## On HPR Sim's own normal force\n\n");
    tables(&mut out, flights, own)?;

    out.push_str(
        "\n## The normal force where the last guide leaves, in the crosswind\n\n\
         The air's angle to the rocket's axis as the last guide leaves, the normal-force \
         coefficient each model gives there, and that force's moment about the last guide for \
         each unit of dynamic pressure and reference area (the coefficient times its arm ahead \
         of the guide, negative when the force acts behind it). The moment is what turns the \
         nose downwind, and the nose-right turning rate follows it.\n\n\
         | Rocket | Speed | Angle of attack | Normal force, HPR Sim's own | RocketPy's | Moment \
         about the guide, HPR Sim's own | RocketPy's | Nose-right rate, on its own | on \
         RocketPy's | RocketPy |\n\
         |---|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n",
    );
    for flight in flights
        .iter()
        .filter(|flight| flight["wind"] == "crosswind")
    {
        let release = |key: &str| &flight[key]["tip_off"]["last_guide_leaves"];
        let rate = "pitch_rate_nose_right_deg_s";
        writeln!(
            out,
            "| {} | {} m/s | {}° | {} | {} | {} m | {} m | {} deg/s | {} deg/s | {} deg/s |",
            text(flight, "case")?,
            fixed(number(release(own), "speed_m_s")?, 1),
            fixed(number(release(own), "angle_of_attack_deg")?, 1),
            fixed(number(release(own), "normal_force_coefficient")?, 2),
            fixed(number(release(recorded), "normal_force_coefficient")?, 2),
            signed(number(release(own), "pivot_moment_m")?, 2),
            signed(number(release(recorded), "pivot_moment_m")?, 2),
            signed(number(release(own), rate)?, 3),
            signed(number(release(recorded), rate)?, 3),
            signed(number(&flight["rocketpy"]["last_guide_leaves"], rate)?, 3),
        )
        .map_err(|e| e.to_string())?;
    }
    let cause = &reading["crosswind_normal_force"];
    if !cause.is_null() {
        writeln!(
            out,
            "\nOn these {} flights HPR Sim's own normal-force coefficient is {} to {} times \
             RocketPy's. Flown by HPR Sim on each, the rocket leaves its last guide turning \
             farther nose-right on HPR Sim's own normal force than on RocketPy's in {} of the \
             {}, by {} to {} deg/s. Which of the two forces is nearer a real rocket's at these \
             angles, no reference here says.",
            cause["flights"],
            fixed(number(cause, "coefficient_ratio_least")?, 2),
            fixed(number(cause, "coefficient_ratio_greatest")?, 2),
            cause["farther_nose_right"],
            cause["flights"],
            fixed(number(cause, "nose_right_rate_more_least_deg_s")?, 3),
            fixed(number(cause, "nose_right_rate_more_greatest_deg_s")?, 3),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(out)
}

/// Whether a regenerated report is the committed one: every number within 2e-6, or 1e-7 of
/// itself, whichever is larger, and everything else the same. HPR Sim is bit-identical on one
/// platform, not across three, so this is the rule `cargo xtask validate --check` holds its own
/// report to. Returns where the two first differ.
fn differs(committed: &Value, fresh: &Value, path: &str) -> Option<String> {
    match (committed, fresh) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) if (x - y).abs() <= (2e-6_f64).max(1e-7 * y.abs()) => None,
            _ if a == b => None,
            _ => Some(format!("{path}: {a} committed, {b} flown")),
        },
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => a
            .iter()
            .zip(b)
            .enumerate()
            .find_map(|(index, (a, b))| differs(a, b, &format!("{path}[{index}]"))),
        (Value::Object(a), Value::Object(b)) if a.len() == b.len() => {
            a.iter().find_map(|(key, a)| match b.get(key) {
                Some(b) => differs(a, b, &format!("{path}.{key}")),
                None => Some(format!("{path}.{key}: committed, not flown")),
            })
        }
        _ if committed == fresh => None,
        _ => Some(format!("{path}: {committed} committed, {fresh} flown")),
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let fresh = generate(&root)?;
    let (path, page_path) = (root.join(REPORT), root.join(REPORT_MD));
    if check {
        let committed = fs::read_to_string(&path).map_err(|e| format!("{REPORT}: {e}"))?;
        let committed: Value =
            serde_json::from_str(&committed).map_err(|e| format!("{REPORT}: {e}"))?;
        if let Some(difference) = differs(&committed, &fresh, "") {
            return Err(format!(
                "{REPORT} is not this run's: run `cargo xtask tip-off-model` and commit it: \
                 {difference}"
            ));
        }
        let committed_page =
            fs::read_to_string(&page_path).map_err(|e| format!("{REPORT_MD}: {e}"))?;
        if committed_page != markdown(&committed)? {
            return Err(format!(
                "{REPORT_MD} is not {REPORT}'s page: run `cargo xtask tip-off-model` and commit it"
            ));
        }
        // The verdicts are read from the flights flown now, not from the committed words.
        let reading = reading(list(&fresh, "flights")?)?;
        if reading["held_on_the_recorded_normal_force"] != true {
            return Err(format!(
                "{REPORT}: a figure on RocketPy's normal force is past its bar"
            ));
        }
        if reading["held_in_still_air_on_its_own_normal_force"] != true {
            return Err(format!(
                "{REPORT}: a still-air figure on HPR Sim's own normal force is past its bar"
            ));
        }
        println!(
            "{REPORT} is current; every figure on RocketPy's normal force is inside its bar, \
             and every still-air figure on HPR Sim's own"
        );
        return Ok(());
    }
    let text = serde_json::to_string_pretty(&fresh).map_err(|e| e.to_string())? + "\n";
    fs::write(&path, text).map_err(|e| format!("{REPORT}: {e}"))?;
    fs::write(&page_path, markdown(&fresh)?).map_err(|e| format!("{REPORT_MD}: {e}"))?;
    println!("wrote {REPORT} and {REPORT_MD}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(path: &str) -> String {
        let root = crate::designs::root().unwrap();
        fs::read_to_string(root.join(path)).unwrap()
    }

    fn committed() -> Value {
        serde_json::from_str(&read(REPORT)).unwrap()
    }

    #[test]
    fn the_committed_page_is_the_reports() {
        assert_eq!(read(REPORT_MD), markdown(&committed()).unwrap());
    }

    #[test]
    fn the_committed_report_is_of_the_committed_reference() {
        // The reference's bytes are what the flights were flown from: a regenerated reference
        // with this report left as it was fails here, on every platform.
        let root = crate::designs::root().unwrap();
        let bytes = fs::read(root.join(ORACLE_REPORT)).unwrap();
        // Git may check the file out with other line ends on Windows; the hash is of the
        // committed bytes.
        let text = String::from_utf8(bytes).unwrap().replace("\r\n", "\n");
        assert_eq!(
            committed()["reference"]["sha256"],
            sha256(text.as_bytes()),
            "{REPORT} was flown from another {ORACLE_REPORT}: run `cargo xtask tip-off-model`"
        );
    }

    #[test]
    fn every_difference_and_the_reading_follow_from_the_committed_flights() {
        // Nothing in the report's verdicts is its own word: each flight's differences come
        // again from its two codes' figures, each `past` from the bars here, and the reading
        // from the flights.
        let report = committed();
        let flights = list(&report, "flights").unwrap();
        assert_eq!(flights.len(), 26);
        for flight in flights {
            let theirs = &flight["rocketpy"];
            for (key, _) in FORCES {
                let ours = &flight[key];
                let again = differences(
                    &ours["tip_off"]["last_guide_leaves"],
                    &ours["moved"],
                    theirs,
                )
                .unwrap();
                assert!(
                    differs(&ours["differences"], &again, "").is_none(),
                    "{} {} {key}: {:?}",
                    flight["case"],
                    flight["wind"],
                    differs(&ours["differences"], &again, "")
                );
                assert_eq!(ours["differences"]["past"], again["past"]);
                // What the phase moves is the two flights' difference.
                for (moved, flown) in [
                    ("apogee_m", "apogee_agl_m"),
                    ("landing_east_m", "landing_east_m"),
                    ("landing_north_m", "landing_north_m"),
                ] {
                    let difference = number(&ours["tip_off"], flown).unwrap()
                        - number(&ours["guided"], flown).unwrap();
                    assert!((number(&ours["moved"], moved).unwrap() - difference).abs() < 1e-9);
                }
                // Guided, HPR Sim leaves the rail straight, with no turning rate.
                let straight = &ours["guided"]["last_guide_leaves"];
                for figure in [
                    "nose_down_deg",
                    "nose_right_deg",
                    "pitch_rate_nose_down_deg_s",
                    "pitch_rate_nose_right_deg_s",
                ] {
                    assert!(number(straight, figure).unwrap().abs() < 1e-9);
                }
            }
        }
        assert_eq!(report["reading"], reading(flights).unwrap());
        let bars = &report["bars"];
        for (key, bar) in [
            ("pitch_rate_fraction", RATE_FRACTION),
            ("pitch_rate_floor_deg_s", RATE_FLOOR_DEG_S),
            ("angle_fraction", ANGLE_FRACTION),
            ("angle_floor_deg", ANGLE_FLOOR_DEG),
            ("apogee_move_percent_of_apogee", APOGEE_MOVE_BAR_PERCENT),
            ("landing_move_percent_of_apogee", LANDING_MOVE_BAR_PERCENT),
        ] {
            assert_eq!(bars[key], json!(bar));
        }
        assert_eq!(report["pad_dead_band_m"], json!(PAD_DEAD_BAND_M));
    }

    #[test]
    fn the_bars_are_the_decision_records() {
        // ADR-357, decision 4: 10% or 0.1 deg/s, 10% or 0.005°, 0.2% and 1% of the apogee.
        // Widening one here fails, as does judging the bars on other flights.
        assert_eq!(
            (
                RATE_FRACTION,
                RATE_FLOOR_DEG_S,
                ANGLE_FRACTION,
                ANGLE_FLOOR_DEG
            ),
            (0.10, 0.1, 0.10, 0.005)
        );
        assert_eq!(
            (APOGEE_MOVE_BAR_PERCENT, LANDING_MOVE_BAR_PERCENT),
            (0.2, 1.0)
        );
        let adr = read(
            "docs/decisions/0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md",
        );
        let words: String = adr.split_whitespace().collect::<Vec<_>>().join(" ");
        for clause in [
            "by at most 10% of RocketPy's rate or 0.1 deg/s, whichever is larger",
            "by at most 10% of RocketPy's angle or 0.005°, whichever is larger",
            "apogee moves differ by at most 0.2% of RocketPy's guided apogee",
            "as vectors, differ by at most 1% of RocketPy's guided apogee",
        ] {
            assert!(words.contains(clause), "ADR-357 no longer says: {clause}");
        }
    }

    #[test]
    fn the_phase_is_held_on_rocketpys_normal_force_and_in_still_air_on_its_own() {
        // What ADR-358 holds of ADR-357 decision 4's bars: all 26 flights on RocketPy's normal
        // force inside every bar, and the 13 still-air flights on HPR Sim's own. The crosswind
        // flights on its own normal force are past the turning rate's bar, every one: the
        // report keeps them, and they are the open increment's.
        let report = committed();
        let reading = &report["reading"];
        assert_eq!(reading["held_on_the_recorded_normal_force"], true);
        assert_eq!(reading["held_in_still_air_on_its_own_normal_force"], true);
        let inside = |force: &str, wind: &str| {
            (
                reading[force][wind]["inside_every_bar"].as_u64().unwrap(),
                reading[force][wind]["flights"].as_u64().unwrap(),
            )
        };
        assert_eq!(inside("recorded_normal_force", "all"), (26, 26));
        assert_eq!(inside("recorded_normal_force", "still"), (13, 13));
        assert_eq!(inside("recorded_normal_force", "crosswind"), (13, 13));
        assert_eq!(inside("own_normal_force", "still"), (13, 13));
        assert_eq!(inside("own_normal_force", "crosswind"), (0, 13));
        assert_eq!(
            list(&reading["own_normal_force"]["past"], "pitch_rate")
                .unwrap()
                .len(),
            13
        );
        // A figure past its bar is counted, whichever figure it is: one flight's landing-point
        // difference set past its bar turns the reading.
        let mut flights = list(&report, "flights").unwrap().clone();
        flights[3]["recorded_normal_force"]["differences"]["past"] = json!(["landing_move"]);
        let turned = super::reading(&flights).unwrap();
        assert_eq!(turned["held_on_the_recorded_normal_force"], false);
        assert_eq!(
            turned["recorded_normal_force"]["all"]["inside_every_bar"],
            25
        );
        assert_eq!(turned["held_in_still_air_on_its_own_normal_force"], true);
        // The same for a still-air flight on HPR Sim's own normal force, and for no flights.
        let mut flights = list(&report, "flights").unwrap().clone();
        let still = flights
            .iter()
            .position(|flight| flight["wind"] == "still")
            .unwrap();
        flights[still]["own_normal_force"]["differences"]["past"] = json!(["angle"]);
        let turned = super::reading(&flights).unwrap();
        assert_eq!(turned["held_in_still_air_on_its_own_normal_force"], false);
        assert_eq!(turned["held_on_the_recorded_normal_force"], true);
        // The cause's size is the committed flights': 13 crosswind flights, HPR Sim's own
        // normal force the larger on every one, the nose turning farther downwind on every one.
        let cause = &reading["crosswind_normal_force"];
        assert_eq!(cause["flights"], 13);
        assert_eq!(cause["farther_nose_right"], 13);
        assert!(cause["coefficient_ratio_least"].as_f64().unwrap() > 1.2);
        assert!(cause["coefficient_ratio_greatest"].as_f64().unwrap() < 2.8);
        let none = super::reading(&[]).unwrap();
        assert_eq!(none["crosswind_normal_force"], Value::Null);
        assert_eq!(none["held_on_the_recorded_normal_force"], false);
        assert_eq!(none["held_in_still_air_on_its_own_normal_force"], false);
    }

    #[test]
    fn a_figure_at_its_bar_is_inside_and_one_past_it_is_not() {
        let rocketpy = json!({
            "guided_apogee_agl_m": 1000.0,
            "last_guide_leaves": {
                "nose_down_deg": 0.06, "nose_right_deg": 0.08,
                "pitch_rate_nose_down_deg_s": 3.0, "pitch_rate_nose_right_deg_s": 4.0,
            },
            "moved": { "apogee_m": -5.0, "landing_east_m": 30.0, "landing_north_m": 40.0 },
        });
        // RocketPy's rate is 5 deg/s and its angle 0.1°: the bars are 0.5 deg/s and 0.01°. The
        // apogee's is 2 m and the landing point's 10 m.
        let ours = |rate_right: f64, angle_right: f64, apogee: f64, east: f64| {
            differences(
                &json!({
                    "nose_down_deg": 0.06, "nose_right_deg": angle_right,
                    "pitch_rate_nose_down_deg_s": 3.0, "pitch_rate_nose_right_deg_s": rate_right,
                }),
                &json!({ "apogee_m": apogee, "landing_east_m": east, "landing_north_m": 40.0 }),
                &rocketpy,
            )
            .unwrap()
        };
        let inside = ours(4.49, 0.0899, -6.99, 39.99);
        assert_eq!(inside["past"], json!([]));
        assert!((inside["pitch_rate_bar_deg_s"].as_f64().unwrap() - 0.5).abs() < 1e-12);
        assert!((inside["angle_bar_deg"].as_f64().unwrap() - 0.01).abs() < 1e-12);
        assert_eq!(ours(4.51, 0.08, -5.0, 30.0)["past"], json!(["pitch_rate"]));
        assert_eq!(ours(4.0, 0.0901, -5.0, 30.0)["past"], json!(["angle"]));
        assert_eq!(ours(4.0, 0.08, -7.01, 30.0)["past"], json!(["apogee_move"]));
        assert_eq!(
            ours(4.0, 0.08, -5.0, 40.01)["past"],
            json!(["landing_move"])
        );
        // A part turned the wrong way is past the bar: the difference is of vectors.
        assert_eq!(
            ours(-4.0, -0.08, -5.0, 30.0)["past"],
            json!(["pitch_rate", "angle"])
        );
        // Where RocketPy has next to nothing, the floors decide: 0.1 deg/s and 0.005°.
        let still = json!({
            "guided_apogee_agl_m": 1000.0,
            "last_guide_leaves": {
                "nose_down_deg": 0.0, "nose_right_deg": 0.0,
                "pitch_rate_nose_down_deg_s": 0.0, "pitch_rate_nose_right_deg_s": 0.0,
            },
            "moved": { "apogee_m": 0.0, "landing_east_m": 0.0, "landing_north_m": 0.0 },
        });
        let floors = |rate: f64, angle: f64| {
            differences(
                &json!({
                    "nose_down_deg": angle, "nose_right_deg": 0.0,
                    "pitch_rate_nose_down_deg_s": rate, "pitch_rate_nose_right_deg_s": 0.0,
                }),
                &still["moved"],
                &still,
            )
            .unwrap()
        };
        assert_eq!(floors(0.099, 0.0049)["past"], json!([]));
        assert_eq!(
            floors(0.101, 0.0051)["past"],
            json!(["pitch_rate", "angle"])
        );
        // A figure that is not a number is refused, never counted inside: JSON has no NaN, so
        // one reaches here as null.
        let broken = differences(
            &json!({
                "nose_down_deg": 0.0, "nose_right_deg": 0.0,
                "pitch_rate_nose_down_deg_s": f64::NAN, "pitch_rate_nose_right_deg_s": 0.0,
            }),
            &still["moved"],
            &still,
        );
        assert!(broken.unwrap_err().contains("pitch_rate_nose_down_deg_s"));
    }

    #[test]
    fn the_pad_count_has_a_dead_band() {
        let moves =
            [0.006, 0.1, 0.11, -0.1, -0.2, 30.0].map(|range| json!({ "landing_range_m": range }));
        assert_eq!(
            pad_counts(moves.iter()).unwrap(),
            json!({ "farther": 2, "nearer": 1, "same": 3 })
        );
    }

    #[test]
    fn a_report_reproduces_within_the_platforms_digits_and_no_further() {
        let committed = json!({ "a": [1.0, 2500.0], "b": "still", "c": { "past": [] } });
        let near = json!({ "a": [1.000_001, 2500.0002], "b": "still", "c": { "past": [] } });
        assert_eq!(differs(&committed, &near, ""), None);
        let far = json!({ "a": [1.000_003, 2500.0], "b": "still", "c": { "past": [] } });
        assert!(differs(&committed, &far, "").unwrap().starts_with(".a[0]"));
        let turned = json!({ "a": [1.0, 2500.0], "b": "still", "c": { "past": ["angle"] } });
        assert!(
            differs(&committed, &turned, "")
                .unwrap()
                .starts_with(".c.past")
        );
        let missing = json!({ "a": [1.0, 2500.0], "b": "still" });
        assert!(differs(&committed, &missing, "").is_some());
    }
}
