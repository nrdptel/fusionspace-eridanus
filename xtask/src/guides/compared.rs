//! The simulator guide's *Compared with other simulators* (M0.8a3, ADR-209 §8, ADR-248).
//!
//! The page's table is written from `validation/features/simulator.toml` (`features.rs`); its
//! prose quotes what a committed report measured with HPR Sim and another tool side by side:
//! the private collection's flights from their report, and the steps to a first answer, the
//! round trips through OpenRocket's file and each tool's count of the table's capabilities from
//! the scoreboard, which reads them from theirs.

use super::{Quote, REAL, Reports, fixed, percent, set_count, set_number, signed_percent};
use crate::scoreboard::SCOREBOARD_JSON as SCOREBOARD;

/// The pointer of `tool`'s cell in the scoreboard's row `id`, found by reading the rows' and the
/// cells' ids, so a renamed row fails the quote instead of reading another.
fn cell(r: &Reports, id: &str, tool: &str) -> Result<String, String> {
    let rows = r.get(SCOREBOARD, "/rows")?.as_array().map_or(0, Vec::len);
    for row in 0..rows {
        if r.text(SCOREBOARD, &format!("/rows/{row}/id"))? != id {
            continue;
        }
        let cells =
            (r.get(SCOREBOARD, &format!("/rows/{row}/cells"))?.as_array()).map_or(0, Vec::len);
        for at in 0..cells {
            let pointer = format!("/rows/{row}/cells/{at}");
            if r.text(SCOREBOARD, &format!("{pointer}/tool"))? == tool {
                return Ok(pointer);
            }
        }
        return Err(format!("{SCOREBOARD}'s row {id} has no cell for {tool}"));
    }
    Err(format!("{SCOREBOARD} has no row {id}"))
}

/// The measured value of `tool`'s cell in the row `id`.
fn value(r: &Reports, id: &str, tool: &str) -> Result<f64, String> {
    r.number(SCOREBOARD, &format!("{}/value", cell(r, id, tool)?))
}

/// The count beside it: of how many capabilities, configurations or designs.
fn count(r: &Reports, id: &str, tool: &str) -> Result<usize, String> {
    r.count(SCOREBOARD, &format!("{}/count", cell(r, id, tool)?))
}

/// The page's quotes.
pub(super) fn quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "Of the table's {} capabilities, HPR Sim has {} in full, OpenRocket {} and \
                       RocketPy {}",
            holes: |r| {
                // One record gives every tool's count, so each counts of the same rows.
                let of = count(r, "features.count", "hpr")?;
                Ok(vec![
                    of.to_string(),
                    fixed(value(r, "features.count", "hpr")?, 0),
                    fixed(value(r, "features.count", "openrocket")?, 0),
                    fixed(value(r, "features.count", "rocketpy")?, 0),
                ])
            },
        },
        Quote {
            template: "On {} logged flights of fliers' own designs, flown from the same files, HPR \
                       Sim's apogee missed by {} on average without signs and OpenRocket's by {}",
            holes: |r| {
                Ok(vec![
                    set_count(r, "hpr", "n")?.to_string(),
                    percent(set_number(r, "hpr", "mean_absolute_percent")?, 2),
                    percent(set_number(r, "openrocket", "mean_absolute_percent")?, 2),
                ])
            },
        },
        Quote {
            template: "with their signs, HPR Sim's averaged {} and OpenRocket's {}",
            holes: |r| {
                Ok(vec![
                    signed_percent(set_number(r, "hpr", "mean_percent")?, 2),
                    signed_percent(set_number(r, "openrocket", "mean_percent")?, 2),
                ])
            },
        },
        Quote {
            template: "On the {} public flights HPR Sim missed by {} on average without signs",
            holes: |r| {
                Ok(vec![
                    r.count(REAL, "/summary/flights")?.to_string(),
                    percent(
                        r.number(REAL, "/summary/mean_absolute_apogee_error_percent")?,
                        2,
                    ),
                ])
            },
        },
        Quote {
            template: "HPR Sim takes {} commands at a command line, or {} lines of Python",
            holes: |r| {
                Ok(vec![
                    fixed(value(r, "steps.command-line", "hpr")?, 0),
                    fixed(value(r, "steps.script", "hpr")?, 0),
                ])
            },
        },
        Quote {
            template: "OpenRocket flies it in {} clicks in its window",
            holes: |r| {
                // Its cell is a "can't", since no release has a Monte Carlo, which states the
                // clicks to the last task it reaches first: "7 clicks to fly, then can't: …".
                let pointer = cell(r, "steps.window-clicks", "openrocket")?;
                let cannot = r.text(SCOREBOARD, &format!("{pointer}/cannot"))?;
                let clicks = cannot
                    .split_once(" clicks to fly")
                    .map(|(clicks, _)| clicks)
                    .filter(|clicks| {
                        !clicks.is_empty() && clicks.bytes().all(|b| b.is_ascii_digit())
                    })
                    .ok_or_else(|| {
                        format!("{SCOREBOARD}'s OpenRocket clicks state no count to fly")
                    })?;
                Ok(vec![clicks.to_owned()])
            },
        },
        Quote {
            template: "Of {} configurations, {} moved by more than {}",
            holes: |r| {
                Ok(vec![
                    count(r, "interop.changed", "openrocket")?.to_string(),
                    fixed(value(r, "interop.changed", "openrocket")?, 0),
                    percent(threshold(r)?, 1),
                ])
            },
        },
        Quote {
            template: "the largest change was {}; {} of {} designs didn't make the trip",
            holes: |r| {
                Ok(vec![
                    percent(value(r, "interop.largest", "openrocket")?, 6),
                    fixed(value(r, "interop.failed", "openrocket")?, 0),
                    count(r, "interop.failed", "openrocket")?.to_string(),
                ])
            },
        },
        super::tour::copernicus(),
    ]
}

/// The apogee change past which a round trip counts as changed, read from the scoreboard row's
/// label, which states it.
fn threshold(r: &Reports) -> Result<f64, String> {
    let rows = r.get(SCOREBOARD, "/rows")?.as_array().map_or(0, Vec::len);
    for row in 0..rows {
        if r.text(SCOREBOARD, &format!("/rows/{row}/id"))? == "interop.changed" {
            let label = r.text(SCOREBOARD, &format!("/rows/{row}/label"))?;
            return label
                .split_once("by more than ")
                .and_then(|(_, rest)| rest.split_once('%'))
                .and_then(|(number, _)| number.parse().ok())
                .ok_or_else(|| format!("{SCOREBOARD}'s interop.changed states no threshold"));
        }
    }
    Err(format!("{SCOREBOARD} has no row interop.changed"))
}
