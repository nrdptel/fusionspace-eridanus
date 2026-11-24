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

use crate::console::{Diagnostics, Level, Text};
use crate::output::{
    EllipseKind, FlagKind, IssueKind, McCountedQuantile, McDispersion, McRun, McSpread,
};
use crate::sim_text::{
    design_lines, flag_help, flag_prefix, issue_help, issue_text, launch_lines, motor_lines,
    note_lines,
};
use crate::units::{
    FOOT_M, STANDARD_GRAVITY_M_S2, bearing, fixed, meters, meters_bound, rounded_down, rounded_up,
};

/// The width of a landing ellipse's label in its table.
const NAME: usize = 22;
/// The width of an ellipse's semi-axis, `10000.0 m (32808 ft)` with two spaces before it.
const AXIS: usize = 22;

/// The text form of `run`: the run on `out`, its warnings, notes and hints on `diagnostics`.
pub(crate) fn print(
    run: &McRun,
    out: &mut Text<'_>,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    design_lines(&run.design, out, diagnostics)?;
    writeln!(out)?;
    let failed = format!(
        "{} failed, up to {} of flights at 95%",
        run.failed.count,
        percent_up(run.failed.up_to)
    );
    writeln!(
        out,
        "{} simulated flights, seed {}: {failed}",
        run.runs, run.seed
    )?;
    out.heading(&format!(
        "{:<SPREAD$}{:>9}{:>9}{:>9}{:>9}{:>9}{:>9}",
        "", "nominal", "mean", "std dev", "5%", "median", "95%"
    ))?;
    let figures = figures(run);
    for figure in &figures {
        row(out, figure)?;
    }
    counting_missing(run, &figures, out)?;
    upper_95_95(run, out)?;
    writeln!(out)?;
    landing(run, out)?;
    writeln!(out)?;
    motor_lines(&run.motors, out)?;
    launch_lines(&run.launch, out)?;
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
            &format!(
                "{flights}, up to {} of flights at 95%: {}",
                percent_up(failure.up_to),
                crate::printable(&crate::sim::rounded(&failure.reason))
            ),
        )?;
    }
    for flag in &run.flags {
        let failed = match flag.failed {
            0 => String::new(),
            failed => format!(
                ", or {} counting the {failed} that failed",
                flag.flights + failed
            ),
        };
        diagnostics.line(
            Level::Warning,
            &format!(
                "{}: {} in {} of {} flights{failed}; up to {} at 95%",
                flag_prefix(flag.flag),
                flag_words(flag.flag),
                flag.flights,
                run.runs,
                percent_up(flag.up_to)
            ),
        )?;
    }
    unraised(run, out)?;
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
            &format!("{kind}, the nominal flight: {}", issue_text(issue)),
        )?;
    }
    issue_help(&run.issues, diagnostics)?;
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
    crate::sim_text::trust_lines(&run.trust, out)
}

/// The width of a figure's name in the spreads' table: `opening airspeed (ft/s)` and a space.
const SPREAD: usize = 24;

/// A figure of the spreads' table: its name, its two units (SI first, then the US or the
/// gravities in which a flier reads it), each with the factor from SI and its decimals, its
/// nominal value and its spread, and what a flight with a value has, for the count of them.
struct Figure<'a> {
    name: &'static str,
    units: [(&'static str, f64, usize); 2],
    nominal: Option<f64>,
    spread: &'a McSpread,
    has: &'static str,
}

/// The figures the table gives: the apogee and the landing distance, then the opening shock and the
/// airspeed a device opened at, each when any flight, or the nominal one, has a value for it
/// (#1258).
fn figures(run: &McRun) -> Vec<Figure<'_>> {
    let length = |datum| [(datum, 1.0, 1), (feet_datum(datum), 1.0 / FOOT_M, 0)];
    let mut figures = vec![
        Figure {
            name: "apogee",
            units: length("m AGL"),
            nominal: run.nominal.apogee_m,
            spread: &run.apogee_m,
            has: "an apogee",
        },
        Figure {
            name: "landing distance",
            units: length("m"),
            nominal: run.nominal.landing_distance_m,
            spread: &run.landing_distance_m,
            has: "a landing",
        },
    ];
    if run.opening_shock_m_s2.count > 0 || run.nominal.opening_shock_m_s2.is_some() {
        figures.push(Figure {
            name: "opening shock",
            units: [("m/s²", 1.0, 1), ("g", 1.0 / STANDARD_GRAVITY_M_S2, 1)],
            nominal: run.nominal.opening_shock_m_s2,
            spread: &run.opening_shock_m_s2,
            has: "an opening shock",
        });
    }
    if run.deployment_airspeed_m_s.count > 0 || run.nominal.deployment_airspeed_m_s.is_some() {
        figures.push(Figure {
            name: "opening airspeed",
            units: [("m/s", 1.0, 1), ("ft/s", 1.0 / FOOT_M, 0)],
            nominal: run.nominal.deployment_airspeed_m_s,
            spread: &run.deployment_airspeed_m_s,
            has: "a device opened",
        });
    }
    figures
}

/// The US unit beside an SI length's: feet, with the datum kept (`m AGL` is `ft AGL`).
fn feet_datum(unit: &'static str) -> &'static str {
    match unit {
        "m AGL" => "ft AGL",
        _ => "ft",
    }
}

/// Two rows of the spreads' table for `figure`: the nominal value, then the spread's, in its
/// first unit, then in its second on the row under it. `apogee (m AGL)` names the height's
/// datum, as `data.md` does.
fn row(out: &mut dyn Write, figure: &Figure<'_>) -> io::Result<()> {
    let spread = figure.spread;
    let cells = [
        figure.nominal,
        spread.mean,
        spread.standard_deviation,
        spread.p05,
        spread.p50,
        spread.p95,
    ];
    for (unit, per_si, decimals) in figure.units {
        let label = format!("{} ({unit})", figure.name);
        write!(out, "{label:<SPREAD$}")?;
        for cell in cells {
            let text = cell.map_or_else(|| "-".to_owned(), |v| fixed(v * per_si, decimals));
            write!(out, "{text:>9}")?;
        }
        writeln!(out)?;
    }
    Ok(())
}

/// When a flight failed or gave no value, what the spreads' columns are taken over, and the 5%
/// and 95% with each such flight placed past that end: the worst case the missing flights allow
/// (#1200). Nothing when every flight gave every value.
fn counting_missing(run: &McRun, figures: &[Figure<'_>], out: &mut dyn Write) -> io::Result<()> {
    if figures
        .iter()
        .all(|figure| figure.spread.count == figure.spread.attempted)
    {
        return Ok(());
    }
    let counts: Vec<String> = figures
        .iter()
        .map(|figure| {
            format!(
                "{} with {} (the rest up to {} of flights at 95%)",
                figure.spread.count,
                figure.has,
                percent_up(figure.spread.missing_up_to)
            )
        })
        .collect();
    let said = format!(
        "the columns count only the flights with a value, of {}: {}; with each of the others \
         placed past that end, the 5% and 95% are at worst",
        run.runs,
        counts.join(", ")
    );
    for line in crate::trust::wrap(&said, 96) {
        writeln!(out, "{line}")?;
    }
    for figure in figures {
        let spread = figure.spread;
        if spread.count == spread.attempted {
            continue;
        }
        let cells = [spread.p05_counting_missing, spread.p95_counting_missing];
        for (unit, per_si, decimals) in figure.units {
            let label = format!("{} ({unit})", figure.name);
            write!(out, "{label:<SPREAD$}{:27}", "")?;
            for (i, cell) in cells.iter().enumerate() {
                let text = match cell {
                    Some(McCountedQuantile::Value(v)) => fixed(v * per_si, decimals),
                    Some(McCountedQuantile::Unbounded) => "unbounded".to_owned(),
                    None => "-".to_owned(),
                };
                if i == 0 {
                    write!(out, "{text:>9}{:9}", "")?;
                } else {
                    write!(out, "{text:>9}")?;
                }
            }
            writeln!(out)?;
        }
    }
    Ok(())
}

/// Where the flights landed, and the ellipses about the landings.
fn landing(run: &McRun, out: &mut Text<'_>) -> io::Result<()> {
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
    writeln!(
        out,
        "{} didn't land, up to {} of flights at 95%",
        run.runs
            .saturating_sub(u64::try_from(landing.count).unwrap_or(run.runs)),
        percent_up(landing.not_landed_up_to)
    )?;
    if landing.ellipses.is_empty() {
        return writeln!(
            out,
            "too few landings for an ellipse: it takes two, and three for the next flight's"
        );
    }
    out.heading(&format!(
        "{:<NAME$}{:>AXIS$}{:>AXIS$}{:>9}{:>18}",
        "landing ellipse", "semi-major", "semi-minor", "heading", "flights inside"
    ))?;
    for ellipse in &landing.ellipses {
        let name = match ellipse.kind {
            EllipseKind::Scatter => format!("{:.0}%", 100.0 * ellipse.level),
            EllipseKind::NextFlight => format!("{:.0}%, the next flight", 100.0 * ellipse.level),
        };
        let inside = format!(
            "{:.1}% (≥ {})",
            100.0 * ellipse.inside,
            percent_down(ellipse.inside_at_least)
        );
        writeln!(
            out,
            "{name:<NAME$}{:>AXIS$}{:>AXIS$}{:>9}{inside:>18}",
            meters(ellipse.semi_major_m),
            meters(ellipse.semi_minor_m),
            bearing(ellipse.major_heading_deg, 0),
        )?;
    }
    for line in crate::trust::wrap(
        "(≥ …) is a lower bound on the share of all the flights this scatter could give, at about \
         95% as the ellipses are fitted to these same flights, a failed flight counted outside",
        96,
    ) {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

/// The flags no flight raised, each with its upper bound at 95%: a run that raises none still
/// allows a few percent, and a failed flight may have raised any of them (#1260).
fn unraised(run: &McRun, out: &mut dyn Write) -> io::Result<()> {
    let Some(first) = run.unraised_flags.first() else {
        return Ok(());
    };
    let names: Vec<&str> = run
        .unraised_flags
        .iter()
        .map(|flag| flag_words(flag.flag))
        .collect();
    let failed = match first.failed {
        0 => String::new(),
        1 => ", counting the 1 that failed".to_owned(),
        failed => format!(", counting the {failed} that failed"),
    };
    let said = format!(
        "no flight raised these flags; each could still be up to {} of flights at 95%{failed}: {}",
        percent_up(first.up_to),
        names.join("; ")
    );
    for line in crate::trust::wrap(&said, 96) {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

/// The apogee's and the landing distance's 95/95 upper bounds (Wilks), beside the 95% column,
/// which is a sample percentile: the figures for a waiver's ceiling or a field's edge (#1260).
fn upper_95_95(run: &McRun, out: &mut dyn Write) -> io::Result<()> {
    let bound = |spread: &McSpread| match spread.upper_95_95 {
        Some(McCountedQuantile::Value(value)) => meters_bound(value),
        Some(McCountedQuantile::Unbounded) => "unbounded, too many flights failed".to_owned(),
        None => "none, it takes 59 flights".to_owned(),
    };
    let said = format!(
        "95/95 upper bounds (at least 95% of flights below, at 95% confidence; a failed flight \
         counted above): apogee {}; landing distance {}",
        bound(&run.apogee_m),
        bound(&run.landing_distance_m)
    );
    for line in crate::trust::wrap(&said, 96) {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

/// A share as a percentage rounded up, for an upper bound: to a tenth from 1%, and to two
/// significant figures below it, so that a small bound isn't printed as zero.
fn percent_up(share: f64) -> String {
    let percent = 100.0 * share;
    if percent <= 0.0 {
        return "0%".to_owned();
    }
    let decimals = if percent >= 1.0 {
        1
    } else {
        // Two significant figures: 0.0030% at 100,000 flights.
        1 - percent.log10().floor() as i32
    };
    format!(
        "{}%",
        fixed(rounded_up(percent, decimals), decimals.max(0) as usize)
    )
}

/// A share as a percentage rounded down to a tenth, for a lower bound.
fn percent_down(share: f64) -> String {
    format!("{}%", fixed(rounded_down(100.0 * share, 1), 1))
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
