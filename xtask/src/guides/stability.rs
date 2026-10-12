//! The stability guide (M0.7a3): how the center of pressure moves with Mach number and angle of
//! attack.
//!
//! Its numbers come from two files. [`REPORT`] holds the guide rocket's center of pressure as HPR
//! Sim's aerodynamics give it, through Mach number straight into the wind and through angle of
//! attack at one low speed; `cargo xtask guides` writes it, and `--check` fails a committed copy
//! the design no longer gives. [`TUNNEL`] is the committed comparison of HPR Sim's center of
//! pressure with NASA's wind-tunnel tests of the Arcas Robin (`cargo xtask aero`, ADR-027), which
//! the guide's figure draws, HPR Sim's errors on the side that flatters a margin marked.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_aero::{AeroModel, Flow};
use serde_json::{Value, json};

use super::{
    CANVAS, CAUTION, INK, INK_MUTED, PLOT_BOTTOM, PLOT_LEFT, PLOT_RIGHT, PLOT_TOP, Quote, RULE,
    Reports, WIDTH, escaped, fixed,
};
pub(crate) use crate::aero_mach::FIXTURE as TUNNEL;

/// The guide rocket's center of pressure, as HPR Sim gives it.
pub(crate) const REPORT: &str = "validation/reports/guide-rocket-cp.json";
/// The guides' own rocket.
const DESIGN: &str = "validation/fixtures/ork/guides/level-1.ork";
/// The Mach numbers of the report's sweep straight into the wind.
const MACHS: [f64; 11] = [0.0, 0.3, 0.5, 0.8, 0.9, 1.0, 1.2, 1.5, 2.0, 2.5, 3.0];
/// The Mach number of the angle sweep, and its angles of attack, degrees.
const ANGLE_MACH: f64 = 0.3;
const ANGLES_DEG: [f64; 5] = [0.0, 2.0, 5.0, 10.0, 15.0];
/// The tunnel comparison the guide draws: the short model, the Arcas Robin itself.
const TUNNEL_MODEL: &str = "arcas-robin-short";
/// The tunnel's other model, the Arcas Robin with a longer body.
const TUNNEL_LONG: &str = "arcas-robin-long";
/// How far a regenerated report's number may sit from the committed one: the aerodynamics' last
/// bits move between platforms' maths libraries, far below the guide's two decimals.
const SAME: f64 = 1e-9;

/// The guide rocket's aerodynamic model, from its `.ork`.
fn model(root: &Path) -> Result<AeroModel, String> {
    let bytes = fs::read(root.join(DESIGN)).map_err(|error| format!("{DESIGN}: {error}"))?;
    let file = hpr_io::ork::read(&bytes).map_err(|error| format!("{DESIGN}: {error}"))?;
    let design = hpr_io::ork::design(&file.value).value;
    let layout = design
        .rocket
        .layout()
        .map_err(|error| format!("{DESIGN}: {error}"))?;
    AeroModel::new(&layout).map_err(|error| format!("{DESIGN}: {error}"))
}

/// The CP at `flow`, m aft of the nose tip, and `C_N/α`, per radian.
fn cp(model: &AeroModel, flow: &Flow) -> Result<(f64, f64), String> {
    let force = model
        .normal_force(flow)
        .map_err(|error| format!("{DESIGN}: {error}"))?;
    let cp = force
        .cp_station_m
        .ok_or(format!("{DESIGN}: no normal-force slope, so no CP"))?;
    Ok((cp, force.slope_per_rad))
}

/// The report: the CP through [`MACHS`] straight into the wind, and through [`ANGLES_DEG`] at
/// [`ANGLE_MACH`], each also in calibers (the reference diameter) aft of the nose tip.
pub(crate) fn generate(root: &Path) -> Result<Value, String> {
    let model = model(root)?;
    let diameter_m = model.reference_diameter_m();
    let fins_supersonic_mach = model
        .fin_sets()
        .iter()
        .map(|set| set.fin.supersonic_mach())
        .reduce(f64::min)
        .ok_or(format!("{DESIGN} has no fins"))?;
    let mut by_mach = Vec::new();
    for mach in MACHS {
        let (cp_m, slope) = cp(&model, &Flow::axial(mach))?;
        by_mach.push(json!({
            "mach": mach,
            "cp_m": cp_m,
            "cp_calibers": cp_m / diameter_m,
            "slope_per_rad": slope,
        }));
    }
    let (straight_m, _) = cp(&model, &Flow::axial(ANGLE_MACH))?;
    let mut by_angle = Vec::new();
    for alpha_deg in ANGLES_DEG {
        let (cp_m, slope) = cp(&model, &Flow::new(ANGLE_MACH, alpha_deg.to_radians(), 0.0))?;
        by_angle.push(json!({
            "alpha_deg": alpha_deg,
            "cp_m": cp_m,
            "cp_calibers": cp_m / diameter_m,
            "shift_calibers": (cp_m - straight_m) / diameter_m,
            "normal_force_over_alpha_per_rad": slope,
        }));
    }
    Ok(json!({
        "generator": "cargo xtask guides",
        "design": DESIGN,
        "note": "HPR Sim's center of pressure for the guides' rocket, from its aerodynamics alone: \
                 m aft of the nose tip, and in calibers, over the reference diameter. by_mach: \
                 straight into the wind (the small-angle CP). by_angle: at angle_mach, the CP at \
                 each angle of attack with the body's lift at that angle, and its shift from the \
                 small-angle CP at that speed, positive aft. fins_supersonic_mach: where supersonic \
                 linear theory starts for the fins. A model's output, not a measurement.",
        "reference_diameter_m": diameter_m,
        "fins_supersonic_mach": fins_supersonic_mach,
        "by_mach": by_mach,
        "angle_mach": ANGLE_MACH,
        "by_angle": by_angle,
    }))
}

/// The report as it is committed.
pub(crate) fn written(report: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|text| text + "\n")
        .map_err(|error| format!("{REPORT}: {error}"))
}

/// Whether two reports hold the same keys, texts and flags, and numbers within [`SAME`].
pub(super) fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) => (a - b).abs() <= SAME,
            _ => false,
        },
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, a)| b.get(key).is_some_and(|b| same(a, b)))
        }
        _ => a == b,
    }
}

/// A problem, if the committed report is not what the design gives now.
pub(crate) fn stale(root: &Path) -> Result<Option<String>, String> {
    let now = generate(root)?;
    let committed = fs::read_to_string(root.join(REPORT))
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok());
    Ok(match committed {
        Some(committed) if same(&committed, &now) => None,
        _ => Some(format!(
            "{REPORT} is not what {DESIGN} gives now; run `cargo xtask guides` and commit it"
        )),
    })
}

/// A Mach number as the guide writes it: to two places, without trailing zeros (`0.8`, `3`).
pub(super) fn mach(value: f64) -> String {
    let text = fixed(value, 2);
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        text
    }
}

/// Calibers to two places.
fn calibers(value: f64) -> String {
    fixed(value, 2)
}

/// A bound in calibers, "up to" or "within" it: rounded up to two places, so the page never
/// states a smaller error than the comparison holds.
fn bound(value: f64) -> String {
    calibers((value * 100.0).ceil() / 100.0)
}

/// The direction of a move along the rocket: aft for a positive one.
fn direction(value: f64) -> &'static str {
    if value > 0.0 { "aft" } else { "forward" }
}

/// One of the guide rocket's rows straight into the wind, its Mach number checked.
fn rocket_row(r: &Reports, at: usize, expected: f64) -> Result<String, String> {
    let pointer = format!("/by_mach/{at}");
    let value = r.number(REPORT, &format!("{pointer}/mach"))?;
    if value == expected {
        Ok(pointer)
    } else {
        Err(format!(
            "{REPORT}'s row {at} is at Mach {value}, not {expected}"
        ))
    }
}

/// The guide rocket's CP at `pointer`'s row, less its CP at low speed, in calibers.
fn rocket_move(r: &Reports, pointer: &str) -> Result<f64, String> {
    let low = rocket_row(r, 0, 0.0)?;
    Ok(r.number(REPORT, &format!("{pointer}/cp_calibers"))?
        - r.number(REPORT, &format!("{low}/cp_calibers"))?)
}

/// The holes for "By Mach {} it has moved {} calibers {}" at row `at`, its Mach number as the
/// report gives it.
fn rocket_move_holes(r: &Reports, at: usize) -> Result<Vec<String>, String> {
    let row = format!("/by_mach/{at}");
    let moved = rocket_move(r, &row)?;
    Ok(vec![
        mach(r.number(REPORT, &format!("{row}/mach"))?),
        calibers(moved.abs()),
        direction(moved).to_owned(),
    ])
}

/// One row of the tunnel comparison.
struct TunnelRow {
    mach: f64,
    band: String,
    measured_calibers: f64,
    hpr_calibers: f64,
    error_calibers: f64,
    uncertainty_calibers: f64,
}

impl Reports {
    /// The pointer of the comparison `id` in [`TUNNEL`]. Finding it reads the references' ids,
    /// a lookup, not a value a quote shows, so they aren't recorded.
    fn tunnel_model(&self, id: &str) -> Result<String, String> {
        let references = self
            .files
            .get(TUNNEL)
            .and_then(|value| value.pointer("/references"))
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{TUNNEL} has no references"))?;
        let at = references
            .iter()
            .position(|reference| reference["id"].as_str() == Some(id))
            .ok_or_else(|| format!("{TUNNEL} has no `{id}`"))?;
        Ok(format!("/references/{at}"))
    }

    /// The short model's rows, which the figure draws.
    fn tunnel_rows(&self) -> Result<Vec<TunnelRow>, String> {
        self.tunnel_rows_of(TUNNEL_MODEL)
    }

    /// The rows of the comparison `id`, each recorded as read, the CPs in calibers aft of the
    /// nose tip.
    fn tunnel_rows_of(&self, id: &str) -> Result<Vec<TunnelRow>, String> {
        let model = self.tunnel_model(id)?;
        let diameter = self.number(TUNNEL, &format!("{model}/reference_diameter_m"))?;
        let count = self
            .get(TUNNEL, &format!("{model}/rows"))?
            .as_array()
            .map(Vec::len)
            .ok_or_else(|| format!("{TUNNEL}'s {model} has no rows"))?;
        let rows = (0..count)
            .map(|at| {
                let row = format!("{model}/rows/{at}");
                let number = |key: &str| self.number(TUNNEL, &format!("{row}/{key}"));
                Ok(TunnelRow {
                    mach: number("mach")?,
                    band: self.text(TUNNEL, &format!("{row}/band"))?.to_owned(),
                    measured_calibers: number("reference_cp_m")? / diameter,
                    hpr_calibers: number("hpr_cp_m")? / diameter,
                    error_calibers: number("cp_error_calibers")?,
                    uncertainty_calibers: number("cp_uncertainty_calibers")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        // The quotes read the first and last rows as the slowest and fastest.
        if rows.windows(2).any(|pair| pair[0].mach >= pair[1].mach) {
            return Err(format!("{TUNNEL}'s {id} rows are not in rising Mach order"));
        }
        Ok(rows)
    }
}

/// The fastest row on HPR Sim's subsonic fin method, at the transonic band's lower edge.
fn last_subsonic(rows: &[TunnelRow]) -> Result<&TunnelRow, String> {
    rows.iter()
        .rfind(|row| row.band == "subsonic")
        .ok_or_else(|| "no subsonic row".to_owned())
}

/// The row where HPR Sim's CP sits furthest aft of the tunnel's, the flattering side.
fn furthest_aft(rows: &[TunnelRow]) -> Result<&TunnelRow, String> {
    rows.iter()
        .filter(|row| row.error_calibers > 0.0)
        .max_by(|a, b| a.error_calibers.total_cmp(&b.error_calibers))
        .ok_or_else(|| "HPR Sim's CP is aft of the tunnel's at no speed".to_owned())
}

/// The Mach numbers that bound the transonic band in the comparison: its last subsonic row and
/// its first supersonic one.
fn join_edges(rows: &[TunnelRow]) -> Result<(f64, f64), String> {
    let below = rows
        .iter()
        .filter(|row| row.band == "subsonic")
        .map(|row| row.mach)
        .reduce(f64::max)
        .ok_or("no subsonic row")?;
    let above = rows
        .iter()
        .filter(|row| row.band == "supersonic")
        .map(|row| row.mach)
        .reduce(f64::min)
        .ok_or("no supersonic row")?;
    Ok((below, above))
}

/// The tunnel's CP at its most aft, and its most forward before that: where it moves aft from
/// and to.
fn tunnel_peak(rows: &[TunnelRow]) -> Result<(&TunnelRow, &TunnelRow), String> {
    let peak = rows
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.measured_calibers.total_cmp(&b.1.measured_calibers))
        .ok_or("no rows")?;
    let before = rows[..peak.0]
        .iter()
        .min_by(|a, b| a.measured_calibers.total_cmp(&b.measured_calibers))
        .ok_or("the tunnel's CP is furthest aft at its first speed")?;
    Ok((before, peak.1))
}

/// How many of the rows HPR Sim's CP is within the comparison's CP target of the tunnel's.
fn within_target(r: &Reports, rows: &[TunnelRow]) -> Result<(usize, f64), String> {
    let target = r.number(TUNNEL, "/targets/cp_calibers")?;
    Ok((
        rows.iter()
            .filter(|row| row.error_calibers.abs() <= target)
            .count(),
        target,
    ))
}

/// Each angle of attack, degrees, with the CP's shift there, calibers, positive aft.
type Shifts = Vec<(f64, f64)>;

/// The guide rocket's shifts at each angle of attack above zero, and the direction of the shift
/// at the largest angle.
fn angle_shifts(r: &Reports) -> Result<(Shifts, &'static str), String> {
    let count = r
        .get(REPORT, "/by_angle")?
        .as_array()
        .map(Vec::len)
        .ok_or_else(|| format!("{REPORT} has no angles"))?;
    let mut shifts = Vec::new();
    for at in 0..count {
        let alpha = r.number(REPORT, &format!("/by_angle/{at}/alpha_deg"))?;
        if alpha > 0.0 {
            let shift = r.number(REPORT, &format!("/by_angle/{at}/shift_calibers"))?;
            shifts.push((alpha, shift));
        }
    }
    if shifts.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err(format!("{REPORT}'s angles are not in rising order"));
    }
    let way = direction(shifts.last().ok_or("no angle above zero")?.1);
    Ok((shifts, way))
}

/// The stability guide's quotes.
pub(super) fn quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "aft of the tunnel's by up to {} calibers on a longer model, and up to {} \
                       on the one the figure draws",
            holes: |r| {
                let long = r.tunnel_rows_of(TUNNEL_LONG)?;
                let short = r.tunnel_rows()?;
                Ok(vec![
                    bound(furthest_aft(&long)?.error_calibers),
                    bound(furthest_aft(&short)?.error_calibers),
                ])
            },
        },
        Quote {
            template: "On the longer model the tunnel also tested, it sat up to {} calibers aft, \
                       at Mach {}, and {} of its {} speeds are within the target",
            holes: |r| {
                let rows = r.tunnel_rows_of(TUNNEL_LONG)?;
                let worst = furthest_aft(&rows)?;
                let (within, _) = within_target(r, &rows)?;
                Ok(vec![
                    bound(worst.error_calibers),
                    mach(worst.mach),
                    within.to_string(),
                    rows.len().to_string(),
                ])
            },
        },
        Quote {
            template: "its center of pressure sits {} calibers behind its nose tip at low speed",
            holes: |r| {
                let low = rocket_row(r, 0, 0.0)?;
                Ok(vec![calibers(
                    r.number(REPORT, &format!("{low}/cp_calibers"))?,
                )])
            },
        },
        Quote {
            template: "By Mach {} it has moved {} calibers {}",
            holes: |r| rocket_move_holes(r, 3),
        },
        Quote {
            template: "by Mach {}, {} calibers {}",
            holes: |r| rocket_move_holes(r, 6),
        },
        Quote {
            template: "and by Mach {} it sits {} calibers {} of where it started",
            holes: |r| rocket_move_holes(r, 10),
        },
        Quote {
            template: "linear theory starts for this rocket's fins at Mach {}",
            holes: |r| Ok(vec![mach(r.number(REPORT, "/fins_supersonic_mach")?)]),
        },
        Quote {
            template: "from Mach {} to {}",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let first = rows.first().ok_or("no rows")?;
                let last = rows.last().ok_or("no rows")?;
                Ok(vec![mach(first.mach), mach(last.mach)])
            },
        },
        Quote {
            // The peak is the most aft reading, so the CP moves aft to it and forward after it.
            template: "From Mach {} to {} the tunnel's center of pressure moved aft by {} \
                       calibers, then forward by {} calibers by Mach {}",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let (before, peak) = tunnel_peak(&rows)?;
                let last = rows.last().ok_or("no rows")?;
                if last.mach <= peak.mach {
                    return Err("the tunnel's CP is furthest aft at its last speed".to_owned());
                }
                Ok(vec![
                    mach(before.mach),
                    mach(peak.mach),
                    calibers(peak.measured_calibers - before.measured_calibers),
                    calibers(peak.measured_calibers - last.measured_calibers),
                    mach(last.mach),
                ])
            },
        },
        Quote {
            template: "At Mach {}, HPR Sim's sat {} calibers {} of the tunnel's",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let first = rows.first().ok_or("no rows")?;
                Ok(vec![
                    mach(first.mach),
                    calibers(first.error_calibers.abs()),
                    direction(first.error_calibers).to_owned(),
                ])
            },
        },
        Quote {
            template: "from Mach {} up, it stayed within {} calibers of it",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let (_, above) = join_edges(&rows)?;
                let worst = rows
                    .iter()
                    .filter(|row| row.band == "supersonic")
                    .map(|row| row.error_calibers.abs())
                    .reduce(f64::max)
                    .ok_or("no supersonic row")?;
                Ok(vec![mach(above), bound(worst)])
            },
        },
        Quote {
            template: "between Mach {} and {}",
            holes: |r| {
                let (below, above) = join_edges(&r.tunnel_rows()?)?;
                Ok(vec![mach(below), mach(above)])
            },
        },
        Quote {
            template: "In the join it didn't follow either: it sat up to {} calibers aft, at Mach {}",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let join: Vec<TunnelRow> = rows
                    .into_iter()
                    .filter(|row| row.band == "transonic")
                    .collect();
                let worst = furthest_aft(&join)?;
                Ok(vec![bound(worst.error_calibers), mach(worst.mach)])
            },
        },
        Quote {
            template: "From Mach {} to {} the tunnel's center of pressure moved {} by {} \
                       calibers, while HPR Sim's moved {} by {}",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let first = rows.first().ok_or("no rows")?;
                let last = last_subsonic(&rows)?;
                let tunnel = last.measured_calibers - first.measured_calibers;
                let hpr = last.hpr_calibers - first.hpr_calibers;
                Ok(vec![
                    mach(first.mach),
                    mach(last.mach),
                    direction(tunnel).to_owned(),
                    calibers(tunnel.abs()),
                    direction(hpr).to_owned(),
                    calibers(hpr.abs()),
                ])
            },
        },
        Quote {
            template: "At Mach {}, still on its subsonic fin method, HPR Sim's sat {} calibers {} \
                       of the tunnel's",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let last = last_subsonic(&rows)?;
                Ok(vec![
                    mach(last.mach),
                    calibers(last.error_calibers.abs()),
                    direction(last.error_calibers).to_owned(),
                ])
            },
        },
        Quote {
            template: "read off the reports' plots to within {} to {} calibers either way",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let uncertainties = rows.iter().map(|row| row.uncertainty_calibers);
                let least = uncertainties.clone().reduce(f64::min).ok_or("no rows")?;
                let most = uncertainties.reduce(f64::max).ok_or("no rows")?;
                Ok(vec![calibers(least), calibers(most)])
            },
        },
        Quote {
            template: "{} of the {} speeds are within {} calibers",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let (within, target) = within_target(r, &rows)?;
                Ok(vec![
                    within.to_string(),
                    rows.len().to_string(),
                    calibers(target),
                ])
            },
        },
        Quote {
            template: "At {} of the {} speeds HPR Sim's center of pressure sat aft of the tunnel's",
            holes: |r| {
                let rows = r.tunnel_rows()?;
                let aft = rows.iter().filter(|row| row.error_calibers > 0.0).count();
                Ok(vec![aft.to_string(), rows.len().to_string()])
            },
        },
        Quote {
            template: "At Mach {}, the guides' rocket's center of pressure moves {} by {}",
            holes: |r| {
                let (shifts, way) = angle_shifts(r)?;
                let list: Vec<String> = shifts
                    .iter()
                    .map(|&(alpha, shift)| {
                        // A shift the other way than the largest angle's says so.
                        let other = if direction(shift) == way {
                            String::new()
                        } else {
                            format!(" {}", direction(shift))
                        };
                        format!(
                            "{} calibers{other} at {}°",
                            calibers(shift.abs()),
                            fixed(alpha, 0)
                        )
                    })
                    .collect();
                let list = match list.split_last() {
                    Some((last, rest)) if !rest.is_empty() => {
                        format!("{}, and {last}", rest.join(", "))
                    }
                    _ => list.join(""),
                };
                Ok(vec![
                    mach(r.number(REPORT, "/angle_mach")?),
                    way.to_owned(),
                    list,
                ])
            },
        },
    ]
}

/// The figure's Mach axis, from zero to this, in steps of one.
const MACH_TOP: f64 = 5.0;
/// The x of Mach `value`.
fn mach_x(value: f64) -> f64 {
    PLOT_LEFT + value / MACH_TOP * (PLOT_RIGHT - PLOT_LEFT)
}

/// The stability guide's figure: the tunnel's center of pressure on the Arcas Robin against
/// HPR Sim's through Mach number, each reading with its uncertainty as a band, the transonic band
/// shaded, and each speed where HPR Sim's sits aft of the tunnel's, the side that flatters a
/// margin, marked.
pub(super) fn draw_cp_against_mach(reports: &Reports) -> Result<String, String> {
    let rows = reports.tunnel_rows()?;
    let (below, above) = join_edges(&rows)?;
    let aft = rows.iter().filter(|row| row.error_calibers > 0.0).count();
    // The caliber axis: whole calibers either side of every reading and its band.
    let least = rows
        .iter()
        .map(|row| {
            row.hpr_calibers
                .min(row.measured_calibers - row.uncertainty_calibers)
        })
        .reduce(f64::min)
        .ok_or("no rows")?
        .floor();
    let most = rows
        .iter()
        .map(|row| {
            row.hpr_calibers
                .max(row.measured_calibers + row.uncertainty_calibers)
        })
        .reduce(f64::max)
        .ok_or("no rows")?
        .ceil();
    if rows.iter().any(|row| row.mach < 0.0 || row.mach > MACH_TOP) {
        return Err(format!(
            "a row past the figure's Mach axis, 0 to {MACH_TOP}"
        ));
    }
    let y = |calibers: f64| {
        PLOT_BOTTOM - (calibers - least) / (most - least) * (PLOT_BOTTOM - PLOT_TOP)
    };
    let height = PLOT_BOTTOM + 150.0;
    let title = "The Arcas Robin's center of pressure by Mach";
    let lines = [
        "NASA's wind tunnel (dots, each with its reading's uncertainty as a band) against"
            .to_owned(),
        "HPR Sim (line): the center of pressure, in calibers aft of the nose tip, by Mach number."
            .to_owned(),
        format!(
            "Shaded: Mach {} to {}, where HPR Sim joins its fin methods with a straight line.",
            mach(below),
            mach(above)
        ),
    ];
    let mut svg = String::new();
    let _ = writeln!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"{height}\" \
         viewBox=\"0 0 {WIDTH} {height}\" font-family=\"'Cascadia Mono', ui-monospace, Menlo, \
         Consolas, monospace\" font-size=\"12\" role=\"img\" aria-labelledby=\"title\" \
         aria-describedby=\"desc\">"
    );
    let _ = writeln!(svg, "<title id=\"title\">{}</title>", escaped(title));
    let _ = writeln!(
        svg,
        "<desc id=\"desc\">{}</desc>",
        escaped(&lines.join(" "))
    );
    let _ = writeln!(
        svg,
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{CANVAS}\"/>"
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"36\" font-size=\"18\" font-weight=\"600\" fill=\"{INK}\">{}</text>",
        escaped(title)
    );
    for (at, line) in lines.iter().enumerate() {
        let _ = writeln!(
            svg,
            "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">{}</text>",
            62 + 16 * at,
            escaped(line)
        );
    }
    // The transonic band, behind everything.
    let (from, to) = (mach_x(below), mach_x(above));
    let _ = writeln!(
        svg,
        "<rect x=\"{from:.1}\" y=\"{PLOT_TOP}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{RULE}\"/>",
        to - from,
        PLOT_BOTTOM - PLOT_TOP
    );
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">the join</text>",
        f64::midpoint(from, to),
        PLOT_TOP - 10.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">calibers aft of the nose tip</text>",
        PLOT_TOP - 30.0
    );
    // The caliber axis.
    let mut tick = least;
    while tick <= most {
        let at = y(tick);
        let _ = writeln!(
            svg,
            "<line x1=\"{:.1}\" y1=\"{at:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{at:.1}\" \
             stroke=\"{INK_MUTED}\" stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" \
             text-anchor=\"end\" fill=\"{INK_MUTED}\">{}</text>",
            PLOT_LEFT - 4.0,
            PLOT_LEFT - 8.0,
            at + 4.0,
            fixed(tick, 0)
        );
        tick += 1.0;
    }
    // Each speed's difference, HPR Sim's less the tunnel's: caution where HPR Sim's is aft.
    for row in &rows {
        let x = mach_x(row.mach);
        let (color, width) = if row.error_calibers > 0.0 {
            (CAUTION, 3.0)
        } else {
            (INK_MUTED, 1.0)
        };
        let _ = writeln!(
            svg,
            "<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{color}\" \
             stroke-width=\"{width}\"/>",
            y(row.measured_calibers),
            y(row.hpr_calibers)
        );
    }
    // The tunnel's readings, each with its band.
    for row in &rows {
        let x = mach_x(row.mach);
        let (top, bottom) = (
            y(row.measured_calibers + row.uncertainty_calibers),
            y(row.measured_calibers - row.uncertainty_calibers),
        );
        let _ = writeln!(
            svg,
            "<rect x=\"{:.1}\" y=\"{top:.1}\" width=\"10\" height=\"{:.1}\" fill=\"{RULE}\" \
             stroke=\"{INK_MUTED}\" stroke-width=\"1\"/><circle cx=\"{x:.1}\" cy=\"{:.1}\" \
             r=\"3\" fill=\"{INK}\"/>",
            x - 5.0,
            bottom - top,
            y(row.measured_calibers)
        );
    }
    // HPR Sim's line through its rows.
    let path: Vec<String> = rows
        .iter()
        .enumerate()
        .map(|(at, row)| {
            format!(
                "{} {:.1} {:.1}",
                if at == 0 { "M" } else { "L" },
                mach_x(row.mach),
                y(row.hpr_calibers)
            )
        })
        .collect();
    let _ = writeln!(
        svg,
        "<path d=\"{}\" fill=\"none\" stroke=\"{PREDICTED}\" stroke-width=\"2\"/>",
        path.join(" ")
    );
    // The axes.
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_TOP}\" x2=\"{PLOT_LEFT}\" y2=\"{PLOT_BOTTOM}\" \
         stroke=\"{INK}\" stroke-width=\"1\"/><line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_BOTTOM}\" \
         x2=\"{PLOT_RIGHT}\" y2=\"{PLOT_BOTTOM}\" stroke=\"{INK}\" stroke-width=\"1\"/>"
    );
    let mut tick = 0.0;
    while tick <= MACH_TOP {
        let _ = writeln!(
            svg,
            "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">{}</text>",
            mach_x(tick),
            PLOT_BOTTOM + 18.0,
            fixed(tick, 0)
        );
        tick += 1.0;
    }
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">Mach \
         number</text>",
        f64::midpoint(PLOT_LEFT, PLOT_RIGHT),
        PLOT_BOTTOM + 40.0
    );
    // The key: the flattering side first.
    let key = PLOT_BOTTOM + 64.0;
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{:.1}\" stroke=\"{CAUTION}\" \
         stroke-width=\"3\"/><text x=\"{:.1}\" y=\"{key:.1}\" font-weight=\"600\" \
         fill=\"{CAUTION}\">flatters a margin: HPR Sim's aft of the tunnel's, at {aft} of {} \
         speeds</text>",
        key - 10.0,
        key + 2.0,
        PLOT_LEFT + 10.0,
        rows.len()
    );
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{:.1}\" stroke=\"{INK_MUTED}\" \
         stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">costs margin: \
         HPR Sim's forward of the tunnel's</text>",
        key + 10.0,
        key + 22.0,
        PLOT_LEFT + 10.0,
        key + 20.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Drawn by cargo xtask guides from \
         {TUNNEL}.</text>",
        height - 32.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Measured: NASA TN D-4013 and D-4014, \
         half-scale models of the Arcas Robin.</text>",
        height - 16.0
    );
    svg.push_str("</svg>\n");
    Ok(svg)
}

/// HPR Sim's line: the system's color for a prediction (`figures::drawing::TOKENS`).
const PREDICTED: &str = "#A22488";

#[cfg(test)]
mod tests {
    use super::*;

    fn reports() -> Reports {
        Reports::load(&crate::designs::root().unwrap()).unwrap()
    }

    /// The committed report is what the guide rocket's design gives now, and a moved number or
    /// a missing row fails it.
    #[test]
    fn the_report_is_current() {
        let root = crate::designs::root().unwrap();
        assert_eq!(stale(&root).unwrap(), None);
        let now = generate(&root).unwrap();
        let mut moved = now.clone();
        moved["by_angle"][4]["shift_calibers"] =
            json!(now["by_angle"][4]["shift_calibers"].as_f64().unwrap() + 10.0 * SAME);
        assert!(!same(&now, &moved));
        let mut shorter = now.clone();
        shorter["by_mach"].as_array_mut().unwrap().pop();
        assert!(!same(&now, &shorter));
        assert!(same(&now, &now.clone()));
    }

    /// The figure's caution marks are the speeds where HPR Sim's CP sits aft of the tunnel's, a
    /// positive `cp_error_calibers`: the side `aero_mach`'s
    /// `a_cp_aft_of_the_reference_reads_a_higher_margin` pins as a margin that reads high.
    #[test]
    fn the_flattering_marks_are_the_speeds_aft_of_the_tunnel() {
        let reports = reports();
        let figure = draw_cp_against_mach(&reports).unwrap();
        let rows = reports.tunnel_rows().unwrap();
        let marks = figure
            .lines()
            .filter(|line| {
                line.starts_with("<line") && line.contains(CAUTION) && !line.contains("<text")
            })
            .count();
        let aft = rows.iter().filter(|row| row.error_calibers > 0.0).count();
        assert!(aft > 0);
        assert_eq!(marks, aft);
        for row in &rows {
            let x = format!("x1=\"{:.1}\"", mach_x(row.mach));
            let line = figure
                .lines()
                .find(|line| line.starts_with(&format!("<line {x}")))
                .unwrap();
            assert_eq!(line.contains(CAUTION), row.error_calibers > 0.0, "{line}");
            assert!(row.error_calibers > 0.0 || row.hpr_calibers <= row.measured_calibers);
            assert!(row.error_calibers <= 0.0 || row.hpr_calibers > row.measured_calibers);
        }
    }

    /// The figure passes the figure checks and draws what the guide's description says.
    #[test]
    fn the_figure_draws_its_description() {
        let figure = draw_cp_against_mach(&reports()).unwrap();
        assert!(crate::figures::check(&figure).is_empty());
        assert!(crate::figures::drawing::check(&figure).is_empty());
        assert!(figure.contains(">the join<"));
        assert!(figure.contains("flatters a margin"));
    }

    /// The figure's shaded band is where HPR Sim joins its fin methods on the Arcas Robin: from
    /// the subsonic method's last Mach number to where linear theory starts for its fins.
    #[test]
    fn the_join_edges_are_the_arcas_fins() {
        let root = crate::designs::root().unwrap();
        let (_, model) =
            crate::aero_mach::load_design(&root, "wind-tunnel-arcas-robin-short.json").unwrap();
        let start = model
            .fin_sets()
            .iter()
            .map(|set| set.fin.supersonic_mach())
            .reduce(f64::min)
            .unwrap();
        let (below, above) = join_edges(&reports().tunnel_rows().unwrap()).unwrap();
        assert_eq!(below, hpr_aero::fins::TRANSONIC_START_MACH);
        assert!((above - start).abs() < 1e-9, "{above} against {start}");
    }

    /// A bound rounds up, so it never states less than the error.
    #[test]
    fn bounds_round_up() {
        assert_eq!(bound(0.3446), "0.35");
        assert_eq!(bound(2.3633), "2.37");
        assert_eq!(bound(0.5), "0.50");
    }

    /// Mach numbers are written to two places, without trailing zeros.
    #[test]
    fn mach_numbers_drop_trailing_zeros() {
        assert_eq!(mach(0.8), "0.8");
        assert_eq!(mach(3.0), "3");
        assert_eq!(mach(4.63), "4.63");
        assert_eq!(mach(1.3678), "1.37");
        assert_eq!(mach(0.0), "0");
    }
}
