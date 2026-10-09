//! `hpr mc`'s text form: the spreads a flyer checks first, the apogee's and the landing's, then
//! the landing ellipses, what flew, from where, what was scattered, and what failed.
//!
//! Heights and distances are SI first, then in feet: the spreads' table gives each quantity a
//! row in meters and a row in feet, and the landing's lines put feet in brackets
//! ([`crate::units`]; ADR-164 §6, ADR-210).
//!
//! The run goes to standard output; its `warning:`, `note:` and `help:` lines go to standard
//! error ([`Diagnostics`]).

use std::io::{self, Write};

use crate::console::{Diagnostics, Level};
use crate::output::{EllipseKind, FlagKind, IssueKind, McDispersion, McRun, McSpread};
use crate::sim_text::{
    design_lines, flag_help, flag_prefix, launch_lines, motor_lines, note_lines,
};
use crate::units::{FOOT_M, fixed, meters};

/// The width of a figure's name in the tables.
const NAME: usize = 22;
/// The width of an ellipse's semi-axis, `10000.0 m (32808 ft)` with two spaces before it.
const AXIS: usize = 22;

/// The text form of `run`: the run on `out`, its warnings, notes and hints on `diagnostics`.
pub(crate) fn print(
    run: &McRun,
    out: &mut dyn Write,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    design_lines(&run.design, out, diagnostics)?;
    writeln!(out)?;
    let failed = if run.failed.count == 1 {
        "1 failed".to_owned()
    } else {
        format!("{} failed", run.failed.count)
    };
    writeln!(out, "{} flights, seed {}: {failed}", run.runs, run.seed)?;
    writeln!(
        out,
        "{:<NAME$}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}",
        "", "nominal", "mean", "std dev", "5%", "median", "95%"
    )?;
    row(out, "apogee", " AGL", run.nominal.apogee_m, &run.apogee_m)?;
    row(
        out,
        "landing distance",
        "",
        run.nominal.landing_distance_m,
        &run.landing_distance_m,
    )?;
    writeln!(out)?;
    landing(run, out)?;
    writeln!(out)?;
    motor_lines(&run.motors, out)?;
    launch_lines(&run.launch, out, diagnostics)?;
    let scattered = scattered(&run.dispersion);
    if !scattered.is_empty() {
        writeln!(
            out,
            "scattered, one standard deviation each: {}",
            scattered.join(", ")
        )?;
    }
    note_lines(&run.notes, &run.warnings, diagnostics)?;
    for failure in &run.failed.reasons {
        let flights = if failure.count == 1 {
            format!("flight {} failed", failure.first_index)
        } else {
            format!(
                "{} flights failed, the first flight {}",
                failure.count, failure.first_index
            )
        };
        diagnostics.line(
            Level::Warning,
            &format!("{flights}: {}", crate::printable(&failure.reason)),
        )?;
    }
    for flag in &run.flags {
        diagnostics.line(
            Level::Warning,
            &format!(
                "{}: {} in {} of {} flights",
                flag_prefix(flag.flag),
                flag_words(flag.flag),
                flag.flights,
                run.runs
            ),
        )?;
    }
    let kinds: Vec<FlagKind> = run.flags.iter().map(|flag| flag.flag).collect();
    flag_help(&kinds, diagnostics)?;
    for issue in &run.issues {
        let kind = match issue.kind {
            IssueKind::Drag => "drag",
            IssueKind::Stability => "stability",
            IssueKind::Flight => "flight",
        };
        diagnostics.line(
            Level::Warning,
            &format!("{kind}, the nominal flight: {}", issue.message),
        )?;
    }
    if let Some(export) = &run.export {
        writeln!(
            out,
            "wrote {} ({} flights){}",
            export.path,
            export.rows,
            export
                .meta
                .as_ref()
                .map_or_else(String::new, |meta| format!(" and {meta}"))
        )?;
    }
    Ok(())
}

/// Two rows of the spreads' table for a height or a distance `name`: the nominal value, then the
/// spread's, in meters to a tenth, then in feet, whole. `datum` follows the unit in the label,
/// ` AGL` for a height above the site (`apogee (m AGL)`, as `data.md` names a height's datum).
fn row(
    out: &mut dyn Write,
    name: &str,
    datum: &str,
    nominal: Option<f64>,
    spread: &McSpread,
) -> io::Result<()> {
    let cells = [
        nominal,
        spread.mean,
        spread.standard_deviation,
        spread.p05,
        spread.p50,
        spread.p95,
    ];
    for (unit, per_m, decimals) in [("m", 1.0, 1), ("ft", 1.0 / FOOT_M, 0)] {
        let label = format!("{name} ({unit}{datum})");
        write!(out, "{label:<NAME$}")?;
        for cell in cells {
            let text = cell.map_or_else(|| "-".to_owned(), |v| fixed(v * per_m, decimals));
            write!(out, "{text:>9}")?;
        }
        writeln!(out)?;
    }
    Ok(())
}

/// Where the flights landed, and the ellipses about the landings.
fn landing(run: &McRun, out: &mut dyn Write) -> io::Result<()> {
    let landing = &run.landing;
    let (Some(east), Some(north)) = (landing.center_east_m, landing.center_north_m) else {
        return writeln!(out, "no flight landed");
    };
    let (north, north_or_south) = if north < 0.0 {
        (-north, "south")
    } else {
        (north, "north")
    };
    let (east, east_or_west) = if east < 0.0 {
        (-east, "west")
    } else {
        (east, "east")
    };
    writeln!(
        out,
        "{} of {} flights landed, centered {} {east_or_west} and {} {north_or_south} of the pad",
        landing.count,
        run.runs,
        meters(east),
        meters(north),
    )?;
    if landing.ellipses.is_empty() {
        return writeln!(
            out,
            "too few landings for an ellipse: it takes two, and three for the next flight's"
        );
    }
    writeln!(
        out,
        "{:<NAME$}{:>AXIS$}{:>AXIS$}{:>9}{:>17}",
        "landing ellipse", "semi-major", "semi-minor", "heading", "flights inside"
    )?;
    for ellipse in &landing.ellipses {
        let name = match ellipse.kind {
            EllipseKind::Scatter => format!("{:.0}%", 100.0 * ellipse.level),
            EllipseKind::NextFlight => format!("{:.0}%, the next flight", 100.0 * ellipse.level),
        };
        writeln!(
            out,
            "{name:<NAME$}{:>AXIS$}{:>AXIS$}{:>8.0}°{:>16.1}%",
            meters(ellipse.semi_major_m),
            meters(ellipse.semi_minor_m),
            ellipse.major_heading_deg,
            100.0 * ellipse.inside
        )?;
    }
    Ok(())
}

/// The options that scattered an input, with their values, as given.
fn scattered(given: &McDispersion) -> Vec<String> {
    [
        ("--mass-sd", given.mass_sd_fraction),
        ("--cg-sd", given.cg_sd_m),
        ("--drag-sd", given.drag_sd_fraction),
        ("--impulse-sd", given.impulse_sd_fraction),
        ("--burn-time-sd", given.burn_time_sd_fraction),
        ("--delay-sd", given.delay_sd_s),
        ("--wind-sd", given.wind_sd_fraction),
        ("--wind-from-sd", given.wind_from_sd_deg),
        ("--inclination-sd", given.inclination_sd_deg),
        ("--heading-sd", given.heading_sd_deg),
        ("--deployment-lag-sd", given.deployment_lag_sd_s),
    ]
    .into_iter()
    .filter(|(_, value)| *value > 0.0)
    .map(|(option, value)| format!("{option} {value}"))
    .collect()
}

/// A flag, in words.
fn flag_words(flag: FlagKind) -> &'static str {
    match flag {
        FlagKind::UnstableUnderPower => "a static margin below zero under power",
        FlagKind::UnstableWithoutMargin => {
            "a pitching moment that turns it away under power, with no margin"
        }
        FlagKind::BeyondValidatedRange => {
            "faster than any public flight HPR Sim has been checked against"
        }
        FlagKind::HighAngleOfAttack => "above 15° angle of attack after the rail",
        FlagKind::OutsideCoreBand => "past Mach 2.5, the core band's top",
        FlagKind::BeyondEnvelope => "past Mach 3.5, the envelope's top",
    }
}
