//! Roll against Mach (M1.8c): hpr's roll forcing against the Arcas Robin's measured roll
//! effectiveness, and its roll damping against the Basic Finner's, written by `cargo xtask aero`
//! to `validation/fixtures/aero/roll-vs-mach.json`.
//!
//! - **The Arcas Robin wind-tunnel models** (NASA TN D-4014 Fig. 14): `C_lδ`, the rolling moment
//!   per degree of fin cant on the body's cross-section and diameter, read into
//!   `validation/fixtures/aero/arcas-robin-wind-tunnel.json` at `α` = −4°, 0° and +4°; compared at
//!   0°, where hpr's model applies.
//! - **The Basic Finner** (Barrowman 1967 Figs. 5-6 and 5-7): its roll damping `C_lp` measured
//!   in a wind tunnel from Mach 1.5 to 3, and Barrowman's computed value at Mach 0.07, read into
//!   `validation/fixtures/aero/basic-finner-roll-damping.json`. Fig. 5-7's points sit a flat 1.27
//!   times the range data (#763), so the scored reference is NOL's spin-decay tunnel, NAVORD
//!   Report 6652 Table 2 (Mach 1.53 to 2.76), converted to `C_lp` per `p d/(2V)` on `π d²/4`.
//!
//! The roadmap sets no target for either (M1.8c: "compared"); each error is reported as it is.

use std::f64::consts::PI;
use std::fs;
use std::path::Path;

use hpr_aero::{AeroModel, FinAero, roll_damping_interference};
use hpr_design::{FinPlanform, Rocket};
use serde_json::{Value, json};

pub const FIXTURE: &str = "validation/fixtures/aero/roll-vs-mach.json";
pub const BASIC_FINNER: &str = "validation/fixtures/aero/basic-finner-roll-damping.json";
/// DREV-TM-9703 Table IX's range fits, cached under `refs/`; only error statistics are committed.
pub const DREV_CACHE: &str =
    "refs/research/finner-reference-data/drev-tm-9703-table-ix-supersonic.csv";
/// HPR Sim's error statistics against DREV, by Mach band, written by `cargo xtask roll-drev`.
pub const DREV_STATISTICS: &str = "validation/fixtures/aero/basic-finner-roll-damping-drev.json";
/// The top of the extended Mach band (ADR-143): DREV rows above it aren't scored.
const ENVELOPE_TOP_MACH: f64 = 3.5;

/// `cargo xtask roll-drev`'s line in the command list.
pub const USAGE: &str = "\
  roll-drev [--check]      Fly the Basic Finner's roll damping against DREV-TM-9703 Table IX's
                           range fits, read from the cached copy under refs/, and write only
                           hpr's error statistics (mean and largest, signed, by Mach band) to
                           validation/fixtures/aero/basic-finner-roll-damping-drev.json.
                           --check fails if the committed statistics differ instead of writing.";

fn read(root: &Path, name: &str) -> Result<Value, String> {
    let text = fs::read_to_string(root.join(name)).map_err(|e| format!("{name}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{name}: {e}"))
}

fn number(value: &Value, what: &str) -> Result<f64, String> {
    value
        .as_f64()
        .ok_or_else(|| format!("{what} is not a number"))
}

/// The fixture, from the committed designs and the two committed references.
pub fn generate(root: &Path) -> Result<Value, String> {
    Ok(json!({
        "generator": "cargo xtask aero",
        "note": "hpr's roll forcing C_l_delta (per degree of cant, the rolling moment the cant \
                 drives, positive, on the body's cross-section and diameter) against the Arcas \
                 Robin's measured roll effectiveness at an angle of attack of 0, and hpr's roll \
                 damping C_lp (per unit of p d/(2V)) against the Basic Finner's. error is hpr's \
                 over the reference's, minus 1. The roadmap sets no target (M1.8c).",
        "arcas_robin": arcas_robin(root)?,
        "basic_finner": basic_finner(root)?,
    }))
}

/// Each Arcas Robin configuration's `C_lδ` at `α = 0` against hpr's.
fn arcas_robin(root: &Path) -> Result<Vec<Value>, String> {
    let tunnel = read(root, crate::aero_mach::WIND_TUNNEL)?;
    let configurations = tunnel["configurations"]
        .as_array()
        .ok_or("the wind tunnel has no `configurations`")?;
    let mut out = Vec::new();
    for configuration in configurations {
        let id = configuration["id"]
            .as_str()
            .ok_or("a configuration without `id`")?;
        let design = configuration["design"]
            .as_str()
            .ok_or("a configuration without `design`")?;
        let rocket: Rocket =
            serde_json::from_value(read(root, &format!("validation/designs/{design}"))?)
                .map_err(|e| format!("{design}: {e}"))?;
        let layout = rocket.layout().map_err(|e| format!("{design}: {e}"))?;
        let model = AeroModel::new(&layout).map_err(|e| format!("{design}: {e}"))?;
        let [set] = model.fin_sets() else {
            return Err(format!("{design}: one fin set expected"));
        };
        let d = model.reference_diameter_m();
        let mut rows = Vec::new();
        let curves = configuration["roll_effectiveness"]
            .as_array()
            .ok_or_else(|| format!("{id}: no `roll_effectiveness`"))?;
        for curve in curves {
            let mach = number(&curve["mach"], "a roll curve's `mach`")?;
            let points = curve["alpha_deg_c_l_delta_per_deg"]
                .as_array()
                .ok_or_else(|| format!("{id} at Mach {mach}: no points"))?;
            let at_zero = points
                .iter()
                .find(|p| p[0].as_f64().is_some_and(|a| a.abs() < 0.5))
                .ok_or_else(|| format!("{id} at Mach {mach}: no reading at 0 deg"))?;
            let measured = number(&at_zero[1], "a roll reading")?;
            let fin = set
                .fin
                .roll(mach, set.body_radius_m, d)
                .map_err(|e| format!("{id} at Mach {mach}: {e}"))?;
            let per_rad =
                f64::from(set.count) * fin.forcing_per_rad * set.roll_forcing_interference;
            let hpr = per_rad * PI / 180.0;
            let error = hpr / measured - 1.0;
            println!(
                "{id:<20} Mach {mach:<5} C_l_delta {hpr:.4}/deg against {measured:.4} ({:+.1}%)",
                100.0 * error
            );
            rows.push(json!({
                "mach": mach,
                "band": crate::aero_mach::band(mach),
                "alpha_deg": at_zero[0],
                "reference_c_l_delta_per_deg": measured,
                "hpr_c_l_delta_per_deg": hpr,
                "hpr_without_body_interference_per_deg": hpr / set.roll_forcing_interference,
                "error": error,
            }));
        }
        out.push(json!({
            "id": id,
            "design": design,
            "reference": "TN D-4014 Fig. 14",
            "fin_count": set.count,
            "roll_forcing_interference": set.roll_forcing_interference,
            "roll_damping_interference": set.roll_damping_interference,
            "rows": rows,
        }));
    }
    Ok(out)
}

/// hpr's Basic Finner: four square fins one diameter in chord and span on a body one diameter
/// across, so any diameter gives the same coefficients.
struct Finner {
    fin: FinAero,
    count: f64,
    body_radius: f64,
    /// The body's roll-damping interference factor `k_R(B)`.
    k_r: f64,
}

impl Finner {
    const DIAMETER: f64 = 1.0;

    /// hpr's `C_lp` at a Mach number: `N C_lp k_R(B)`, per unit of `p d/(2V)` on `π d²/4`.
    fn c_lp(&self, mach: f64) -> Result<f64, String> {
        let roll = self
            .fin
            .roll(mach, self.body_radius, Self::DIAMETER)
            .map_err(|e| e.to_string())?;
        Ok(self.count * roll.damping * self.k_r)
    }
}

/// The Basic Finner of the reference file's `geometry`.
fn finner_model(reference: &Value) -> Result<Finner, String> {
    let geometry = &reference["geometry"];
    let count = number(&geometry["fin_count"], "fin_count")?;
    let (c_r, c_t, s, sweep, r) = (
        number(&geometry["root_chord_calibers"], "root_chord_calibers")?,
        number(&geometry["tip_chord_calibers"], "tip_chord_calibers")?,
        number(&geometry["span_calibers"], "span_calibers")?,
        number(&geometry["leading_edge_sweep_calibers"], "sweep")?,
        number(&geometry["body_radius_at_fins_calibers"], "body radius")?,
    );
    let d = Finner::DIAMETER;
    let planform = FinPlanform::Trapezoidal {
        root_chord_m: c_r,
        tip_chord_m: c_t,
        span_m: s,
        sweep_m: sweep,
    };
    let fin = FinAero::new(&planform, 0.25 * PI * d * d).map_err(|e| e.to_string())?;
    let k_r = roll_damping_interference(s, r, c_t / c_r).map_err(|e| e.to_string())?;
    Ok(Finner {
        fin,
        count,
        body_radius: r,
        k_r,
    })
}

/// The Basic Finner's `C_lp` against each committed reference.
fn basic_finner(root: &Path) -> Result<Value, String> {
    let reference = read(root, BASIC_FINNER)?;
    let finner = finner_model(&reference)?;
    let k_r = finner.k_r;
    let rows_of = |key: &str| -> Result<Vec<Value>, String> {
        let points = reference[key]
            .as_array()
            .ok_or_else(|| format!("the Basic Finner has no `{key}`"))?;
        points
            .iter()
            .map(|p| {
                let mach = number(&p["mach"], "a Basic Finner `mach`")?;
                let c_lp = number(&p["c_lp"], "a Basic Finner `c_lp`")?;
                let hpr = finner.c_lp(mach)?;
                let error = hpr / c_lp - 1.0;
                println!(
                    "basic-finner {key:<22} Mach {mach:<5} C_lp {hpr:.3} against {c_lp:.3} ({:+.1}%)",
                    100.0 * error
                );
                Ok(json!({
                    "mach": mach,
                    "band": crate::aero_mach::band(mach),
                    "reference_c_lp": c_lp,
                    "hpr_c_lp": hpr,
                    "error": error,
                }))
            })
            .collect()
    };
    Ok(json!({
        "id": "basic-finner",
        "reference": "NAVORD Report 6652 Table 2 (scored); Barrowman 1967 Fig. 5-7 (disputed)",
        "roll_damping_interference": k_r,
        "nol_tunnel": rows_of("nol_tunnel_c_lp")?,
        "wind_tunnel": rows_of("wind_tunnel_c_lp")?,
        "barrowman_theory": rows_of("barrowman_theory_c_lp")?,
    }))
}

/// The project's Mach bands (ADR-143): the core band to Mach 2.5, the extended band to 3.5.
fn envelope_band(mach: f64) -> &'static str {
    if mach <= 2.5 { "core" } else { "extended" }
}

/// Each band's row count, Mach range, mean error and largest error (the one of greatest size,
/// signed), from `(mach, error)` pairs in Mach order.
fn band_statistics(errors: &[(f64, f64)]) -> Vec<Value> {
    let mut out = Vec::new();
    for band in ["core", "extended"] {
        let rows: Vec<(f64, f64)> = errors
            .iter()
            .copied()
            .filter(|(mach, _)| envelope_band(*mach) == band)
            .collect();
        let (Some(first), Some(last)) = (rows.first(), rows.last()) else {
            continue;
        };
        let mean = rows.iter().map(|(_, e)| e).sum::<f64>() / rows.len() as f64;
        let largest = rows
            .iter()
            .map(|(_, e)| *e)
            .fold(0.0_f64, |a, e| if e.abs() > a.abs() { e } else { a });
        out.push(json!({
            "band": band,
            "rows": rows.len(),
            "mach_from": first.0,
            "mach_to": last.0,
            "mean_error": mean,
            "largest_error": largest,
        }));
    }
    out
}

/// DREV's `(mach, C_lp)` rows from the cached CSV: `#` lines are comments, the first other line
/// names the columns.
fn drev_rows(text: &str) -> Result<Vec<(f64, f64)>, String> {
    let mut lines = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let header: Vec<&str> = lines
        .next()
        .ok_or("the DREV cache has no header")?
        .split(',')
        .collect();
    let column = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .ok_or_else(|| format!("the DREV cache has no `{name}` column"))
    };
    let (mach_at, clp_at) = (column("mach")?, column("clp")?);
    lines
        .map(|line| {
            let cells: Vec<&str> = line.split(',').collect();
            let cell = |at: usize, what: &str| -> Result<f64, String> {
                cells
                    .get(at)
                    .and_then(|c| c.trim().parse::<f64>().ok())
                    .filter(|v| v.is_finite())
                    .ok_or_else(|| format!("a DREV row's {what} is not a number: {line}"))
            };
            Ok((cell(mach_at, "Mach")?, cell(clp_at, "C_lp")?))
        })
        .collect()
}

/// The statistics file: hpr's errors against DREV's `C_lp` by band. No `C_lp` of DREV's is written,
/// but with two rows a band, the band's Mach range and hpr's public model, its mean and largest
/// error give the readings back; the report's public release allows that (#763).
fn drev_statistics(root: &Path) -> Result<Value, String> {
    let reference = read(root, BASIC_FINNER)?;
    let finner = finner_model(&reference)?;
    let text = fs::read_to_string(root.join(DREV_CACHE)).map_err(|e| {
        format!("{DREV_CACHE}: {e} (the cached DREV-TM-9703 readings live only under refs/)")
    })?;
    let mut errors = Vec::new();
    for (mach, c_lp) in drev_rows(&text)? {
        if c_lp >= 0.0 {
            return Err(format!(
                "DREV's C_lp at Mach {mach} is not negative: {c_lp}"
            ));
        }
        if !(mach > 0.0 && mach <= ENVELOPE_TOP_MACH) {
            return Err(format!(
                "DREV's row at Mach {mach} is outside the envelope, Mach 0 to {ENVELOPE_TOP_MACH}"
            ));
        }
        let error = finner.c_lp(mach)? / c_lp - 1.0;
        if !error.is_finite() {
            return Err(format!("hpr's error at Mach {mach} is not finite: {error}"));
        }
        println!(
            "basic-finner DREV Table IX Mach {mach:<5} hpr {:+.1}%",
            100.0 * error
        );
        errors.push((mach, error));
    }
    errors.sort_by(|a, b| a.0.total_cmp(&b.0));
    if errors.is_empty() {
        return Err("the DREV cache has no rows".into());
    }
    Ok(json!({
        "generator": "cargo xtask roll-drev",
        "note": "hpr's Basic Finner roll damping C_lp (per unit of p d/(2V), on pi d^2/4) against \
                 DREV-TM-9703 Table IX's range fits (Dupuis and Hathaway 1997, PDF pp. 48-49), \
                 read from the cached copy under refs/. error is hpr's over DREV's, minus 1; \
                 positive reads high. No C_lp of DREV's is written, but with two rows a band, \
                 the Mach range and hpr's public model these statistics give the readings back; \
                 the report is approved for public release (issue #763). Bands: core to Mach \
                 2.5, extended to 3.5 (ADR-143).",
        "bands": band_statistics(&errors),
    }))
}

/// Writes the DREV statistics, or with `--check` fails if the committed file differs.
pub fn run_drev(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("usage:\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let fresh = drev_statistics(&root)?;
    let path = root.join(DREV_STATISTICS);
    if check {
        let committed = fs::read_to_string(&path).map_err(|e| format!("{DREV_STATISTICS}: {e}"))?;
        let committed: Value =
            serde_json::from_str(&committed).map_err(|e| format!("{DREV_STATISTICS}: {e}"))?;
        if !crate::designs::same(&committed, &fresh) {
            return Err(format!(
                "{DREV_STATISTICS} is stale: run `cargo xtask roll-drev` and commit it: {}",
                crate::designs::difference(&committed, &fresh).unwrap_or_default()
            ));
        }
        println!("{DREV_STATISTICS} is current");
        return Ok(());
    }
    let text = serde_json::to_string_pretty(&fresh).map_err(|e| e.to_string())? + "\n";
    fs::write(&path, text).map_err(|e| format!("{DREV_STATISTICS}: {e}"))?;
    println!("wrote {DREV_STATISTICS}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bands split at Mach 2.5 (inclusive below); the largest error keeps its sign; an empty band
    /// is left out.
    #[test]
    fn band_statistics_split_at_mach_2_5_and_keep_the_sign() {
        let stats = band_statistics(&[(1.8, 0.10), (2.5, -0.30), (2.7, 0.05), (3.1, 0.15)]);
        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0]["band"], "core");
        assert_eq!(stats[0]["rows"], 2);
        assert!((stats[0]["mean_error"].as_f64().unwrap() + 0.10).abs() < 1e-12);
        assert_eq!(stats[0]["largest_error"].as_f64(), Some(-0.30));
        assert_eq!(stats[1]["largest_error"].as_f64(), Some(0.15));
        assert_eq!(stats[1]["mach_from"].as_f64(), Some(2.7));
        let only_core = band_statistics(&[(2.0, 0.2)]);
        assert_eq!(only_core.len(), 1);
    }

    /// The pages quote the committed DREV statistics as they are: each band's mean and largest
    /// error, to 0.1%, on the accuracy page and the aero page (CI can't rerun `roll-drev`).
    #[test]
    fn the_pages_quote_the_committed_drev_statistics() {
        let root = crate::designs::root().unwrap();
        let stats = read(&root, DREV_STATISTICS).unwrap();
        let pages = ["docs/accuracy.md", "docs/physics/aero.md"]
            .map(|page| (page, fs::read_to_string(root.join(page)).unwrap()));
        let bands = stats["bands"].as_array().unwrap();
        assert_eq!(bands.len(), 2);
        let shown =
            |band: &Value, key: &str| format!("{:.1}%", 100.0 * band[key].as_f64().unwrap());
        for band in bands {
            let (mean, largest) = (shown(band, "mean_error"), shown(band, "largest_error"));
            for (page, text) in &pages {
                assert!(
                    text.contains(&format!("{mean} high on average"))
                        || text.contains(&format!("{mean} on average")),
                    "{page} doesn't quote the mean {mean}"
                );
                assert!(
                    text.contains(&format!("{largest} at most")),
                    "{page} doesn't quote the largest {largest}"
                );
            }
        }
    }

    /// The cache's comment lines and column order don't matter; a bad or non-finite cell is refused.
    #[test]
    fn drev_rows_read_by_column_name() {
        let rows = drev_rows("# note\nmach,cx0,clp\n1.8,0.5,-22.0\n2.4,0.4,-18.0\n").unwrap();
        assert_eq!(rows, vec![(1.8, -22.0), (2.4, -18.0)]);
        assert!(drev_rows("mach,clp\n1.8,x\n").is_err());
        assert!(drev_rows("mach,clp\n1.8,NaN\n").is_err());
        assert!(drev_rows("mach,cx0\n1.8,0.5\n").is_err());
    }
}
