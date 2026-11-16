//! The Basic Finner's normal force and center of pressure against DREV-TM-9703's range fits
//! (M1.14b1), written by `cargo xtask aero` to
//! `validation/fixtures/aero/basic-finner-normal-force.json`.
//!
//! - **The reference** (Dupuis and Hathaway 1997, Table IX, PDF pp. 48–49): `C_Nα` and `C_mα`
//!   from six-degree-of-freedom fits of free flights in DREV's aeroballistic range, each row
//!   several shots at one Mach number, typed into `validation/fixtures/aero/basic-finner-drev.json`
//!   with the fits' probable errors. The six rows inside the envelope run from Mach 1.077 to 3.147.
//! - **The center of pressure** is not printed: it is the moment reference less `C_mα / C_Nα`
//!   calibers (`C_mα` is negative for a stable body, so the CP lies aft of the reference).
//! - **hpr** is its small-angle slope and CP straight into the wind, on the committed design
//!   `validation/designs/range-basic-finner.json`. The fits hold their cubic terms at zero on
//!   these rows, so each measured slope is a linear fit over the angles flown; beside the
//!   small-angle figures the report gives hpr's `C_N/α` and CP at each fit's root-mean-square yaw.
//!
//! No target is set: each error is reported as it is, beside the measurement's own probable error.

use std::path::Path;

use hpr_aero::{AeroModel, Flow};
use hpr_design::Rocket;
use serde_json::{Value, json};

pub const FIXTURE: &str = "validation/fixtures/aero/basic-finner-normal-force.json";
pub const REFERENCE: &str = "validation/fixtures/aero/basic-finner-drev.json";
/// The top of the extended Mach band (ADR-143): a reference row above it is refused.
const ENVELOPE_TOP_MACH: f64 = 3.5;
/// The table prints each probable error in whole percent, so one printed as 0 is below this.
const PRINTED_ZERO_PERCENT: f64 = 0.5;

fn read(root: &Path, name: &str) -> Result<Value, String> {
    let text = std::fs::read_to_string(root.join(name)).map_err(|e| format!("{name}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{name}: {e}"))
}

fn number(value: &Value, what: &str) -> Result<f64, String> {
    value
        .as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| format!("{what} is not a number"))
}

/// The project's Mach bands (ADR-143): below Mach 1.5 a row is outside M1.14b's band and is
/// reported apart; the core band runs to Mach 2.5 and the extended band to 3.5.
pub fn band(mach: f64) -> &'static str {
    if mach < 1.5 {
        "below Mach 1.5"
    } else if mach <= 2.5 {
        "core, Mach 1.5 to 2.5"
    } else {
        "extended, Mach 2.5 to 3.5"
    }
}

/// The measured center of pressure, in calibers aft of the nose tip, and its probable error.
///
/// `x_cp = x_ref − C_mα / C_Nα`, both slopes the fit's linear terms on the body's diameter. The
/// probable error is the ratio's, to first order, the two slopes' errors taken as independent
/// (the report gives no correlation): the arm `|C_mα / C_Nα|` times the root sum of
/// squares of the two relative probable errors. A probable error printed as 0% is taken at the
/// print's resolution, [`PRINTED_ZERO_PERCENT`], so the result never understates the table.
pub fn measured_cp(
    moment_reference_calibers: f64,
    cn_alpha: f64,
    cm_alpha: f64,
    cn_alpha_error_percent: f64,
    cm_alpha_error_percent: f64,
) -> (f64, f64) {
    let arm = -cm_alpha / cn_alpha;
    let relative = |percent: f64| 0.01 * percent.max(PRINTED_ZERO_PERCENT);
    let error =
        arm.abs() * relative(cn_alpha_error_percent).hypot(relative(cm_alpha_error_percent));
    (moment_reference_calibers + arm, error)
}

/// The fixture, from the committed design and the committed reference.
pub fn generate(root: &Path) -> Result<Value, String> {
    let reference = read(root, REFERENCE)?;
    let design = reference["design"]
        .as_str()
        .ok_or("the reference names no `design`")?;
    let rocket: Rocket =
        serde_json::from_value(read(root, &format!("validation/designs/{design}"))?)
            .map_err(|e| format!("{design}: {e}"))?;
    let layout = rocket.layout().map_err(|e| format!("{design}: {e}"))?;
    let model = AeroModel::new(&layout).map_err(|e| format!("{design}: {e}"))?;
    let d = model.reference_diameter_m();
    let moment_reference = number(
        &reference["moment_reference_calibers"],
        "moment_reference_calibers",
    )?;
    let mut rows = Vec::new();
    for row in reference["rows"]
        .as_array()
        .ok_or("the reference has no `rows`")?
    {
        let mach = number(&row["mach"], "a row's `mach`")?;
        if !(mach > 1.0 && mach <= ENVELOPE_TOP_MACH) {
            return Err(format!(
                "the row at Mach {mach} is outside Mach 1 to {ENVELOPE_TOP_MACH}"
            ));
        }
        let cn_alpha = number(&row["cn_alpha"], "cn_alpha")?;
        let cm_alpha = number(&row["cm_alpha"], "cm_alpha")?;
        if !(cn_alpha > 0.0 && cm_alpha < 0.0) {
            return Err(format!(
                "the row at Mach {mach} is not a stable body's: C_N_alpha {cn_alpha}, C_m_alpha {cm_alpha}"
            ));
        }
        let cn_error_percent = number(
            &row["cn_alpha_probable_error_percent"],
            "cn_alpha_probable_error_percent",
        )?;
        let cm_error_percent = number(
            &row["cm_alpha_probable_error_percent"],
            "cm_alpha_probable_error_percent",
        )?;
        if cn_error_percent < 0.0 || cm_error_percent < 0.0 {
            return Err(format!(
                "the row at Mach {mach} has a negative probable error"
            ));
        }
        let (cp, cp_error) = measured_cp(
            moment_reference,
            cn_alpha,
            cm_alpha,
            cn_error_percent,
            cm_error_percent,
        );
        let (hpr_cn_alpha, hpr_cp_m) = crate::aero_mach::hpr_at(&model, mach)?;
        let hpr_cp = hpr_cp_m / d;
        // hpr's slope by part: the nose and cylinder, then the fins with their body interference.
        let parts = model
            .components(&Flow::axial(mach))
            .map_err(|e| format!("Mach {mach}: {e}"))?;
        let (bodies, fins) = parts.split_at(model.bodies().len());
        let part = |kept: &[hpr_aero::ComponentNormalForce]| {
            let slope: f64 = kept.iter().map(|p| p.normal_force.slope_per_rad).sum();
            let moment: f64 = kept.iter().map(|p| p.normal_force.moment_slope_m).sum();
            (slope, moment / slope / d)
        };
        let (hpr_body, hpr_body_cp) = part(bodies);
        let (hpr_fins, hpr_fins_cp) = part(fins);
        // hpr at the fit's root-mean-square yaw, where its body lift adds to the slope.
        let mean_squared_yaw_deg2 = number(&row["mean_squared_yaw_deg2"], "mean_squared_yaw_deg2")?;
        if mean_squared_yaw_deg2 <= 0.0 {
            return Err(format!(
                "the row at Mach {mach} has no positive mean squared yaw"
            ));
        }
        let rms_yaw_deg = mean_squared_yaw_deg2.sqrt();
        let at_yaw = model
            .normal_force(&Flow::new(mach, rms_yaw_deg.to_radians(), 0.0))
            .map_err(|e| format!("Mach {mach}: {e}"))?;
        let hpr_cp_at_yaw = at_yaw
            .cp_station_m
            .ok_or(format!("Mach {mach}: no CP at {rms_yaw_deg} deg"))?
            / d;
        let cn_alpha_error = hpr_cn_alpha / cn_alpha - 1.0;
        let cp_error_calibers = hpr_cp - cp;
        println!(
            "basic-finner DREV Mach {mach:<5} C_N_alpha {hpr_cn_alpha:.2} against {cn_alpha:.2} \
             ({:+.1}%, fit {cn_error_percent}%); CP {hpr_cp:.2} against {cp:.2} calibers \
             ({cp_error_calibers:+.2}, fit {cp_error:.2})",
            100.0 * cn_alpha_error
        );
        rows.push(json!({
            "mach": mach,
            "band": band(mach),
            "reference_cn_alpha_per_rad": cn_alpha,
            "reference_cn_alpha_probable_error": 0.01 * cn_error_percent.max(PRINTED_ZERO_PERCENT),
            "reference_cp_calibers": cp,
            "reference_cp_probable_error_calibers": cp_error,
            "hpr_cn_alpha_per_rad": hpr_cn_alpha,
            "hpr_cp_calibers": hpr_cp,
            "hpr_body_cn_alpha_per_rad": hpr_body,
            "hpr_body_cp_calibers": hpr_body_cp,
            "hpr_fins_cn_alpha_per_rad": hpr_fins,
            "hpr_fins_cp_calibers": hpr_fins_cp,
            "cn_alpha_error": cn_alpha_error,
            "cp_error_calibers": cp_error_calibers,
            "rms_yaw_deg": rms_yaw_deg,
            "hpr_cn_over_alpha_at_rms_yaw_per_rad": at_yaw.slope_per_rad,
            "hpr_cp_at_rms_yaw_calibers": hpr_cp_at_yaw,
            "cn_alpha_error_at_rms_yaw": at_yaw.slope_per_rad / cn_alpha - 1.0,
            "cp_error_at_rms_yaw_calibers": hpr_cp_at_yaw - cp,
        }));
    }
    if rows.is_empty() {
        return Err("the reference has no rows".into());
    }
    Ok(json!({
        "generator": "cargo xtask aero",
        "note": "hpr's Basic Finner normal-force slope C_N_alpha (per radian, on pi d^2/4) and \
                 center of pressure (calibers aft of the nose tip) at a small angle of attack, \
                 against DREV-TM-9703 Table IX's six-degree-of-freedom range fits (Dupuis and \
                 Hathaway 1997, PDF pp. 48-49; validation/fixtures/aero/basic-finner-drev.json). \
                 The measured center of pressure is the moment reference less C_m_alpha over \
                 C_N_alpha; its probable error is the ratio's, to first order, from the fits' \
                 printed probable errors taken as independent, one printed as 0% taken as 0.5%. \
                 cn_alpha_error is \
                 hpr's over DREV's, minus 1; cp_error_calibers is hpr's center of pressure less \
                 DREV's, positive aft: a center of pressure too far aft overstates stability. \
                 hpr_body_* is the nose and cylinder's part of hpr's slope and where it acts, \
                 hpr_fins_* the four fins' with their interference with the body; the report \
                 measured no such parts. The fits hold their cubic terms at zero on these \
                 rows, so each measured slope is a linear fit over the angles flown: \
                 rms_yaw_deg is the square root of the fit's mean squared yaw, \
                 hpr_cn_over_alpha_at_rms_yaw_per_rad and hpr_cp_at_rms_yaw_calibers are hpr's \
                 C_N over alpha and center of pressure at that angle of attack, where its body \
                 lift adds to the slope, and the two *_at_rms_yaw errors are those against \
                 DREV's. \
                 No target is set (M1.14b1). Bands: ADR-143's, with the rows below Mach 1.5 \
                 apart.",
        "id": reference["id"],
        "design": design,
        "reference_diameter_m": d,
        "moment_reference_calibers": moment_reference,
        "rows": rows,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The center of pressure lies aft of the moment reference by `−C_mα / C_Nα`, and its probable
    /// error is the ratio's: worked by hand for a body with `C_Nα` 10 ± 3% and `C_mα` −20 ± 4%
    /// about 5 calibers: 7 calibers, ± 2 × 5% = 0.1.
    #[test]
    fn the_measured_center_of_pressure_and_its_error_are_the_ratios() {
        let (cp, error) = measured_cp(5.0, 10.0, -20.0, 3.0, 4.0);
        assert!((cp - 7.0).abs() < 1e-12);
        assert!((error - 0.1).abs() < 1e-12);
        // A probable error printed as 0% counts as half a percent, not as none.
        let (_, floor) = measured_cp(5.0, 10.0, -20.0, 0.0, 0.0);
        assert!((floor - 2.0 * 0.005 * 2.0_f64.sqrt()).abs() < 1e-12);
        // An unstable body's center of pressure is ahead of the reference.
        assert!(measured_cp(5.0, 10.0, 20.0, 1.0, 1.0).0 < 5.0);
    }

    /// The pages quote the committed report as it is: the aerodynamics page each row of its
    /// three tables, the decision record the small-angle figures, and the accuracy page each
    /// row's two small-angle errors.
    #[test]
    fn the_pages_quote_the_committed_report() {
        let root = crate::designs::root().unwrap();
        let report = read(&root, FIXTURE).unwrap();
        let reference = read(&root, REFERENCE).unwrap();
        let page = |name: &str| std::fs::read_to_string(root.join(name)).unwrap();
        let aero = page("docs/physics/aero.md");
        let accuracy = page("docs/accuracy.md");
        let record =
            page("docs/decisions/0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md");
        let f = |v: &Value| v.as_f64().unwrap();
        // A signed figure as the pages write it, with a true minus sign.
        let signed = |text: String| text.replace('-', "\u{2212}");
        let rows = report["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 6);
        for (row, reading) in rows.iter().zip(reference["rows"].as_array().unwrap()) {
            let mach = f(&row["mach"]);
            let fit = f(&reading["cn_alpha_probable_error_percent"]);
            let slope = format!("{:.2} ({fit}%)", f(&row["reference_cn_alpha_per_rad"]));
            let slope_error = signed(format!("{:.1}%", 100.0 * f(&row["cn_alpha_error"])));
            let cp = format!(
                "{:.2} ({:.2})",
                f(&row["reference_cp_calibers"]),
                f(&row["reference_cp_probable_error_calibers"])
            );
            let cp_error = signed(format!("{:.2}", f(&row["cp_error_calibers"])));
            let (hpr, hpr_cp) = (
                format!("{:.2}", f(&row["hpr_cn_alpha_per_rad"])),
                format!("{:.2}", f(&row["hpr_cp_calibers"])),
            );
            let slope_row = format!(
                "| {mach} | {slope} | {:.2} | {:.2} | {hpr} | {slope_error} |",
                f(&row["hpr_body_cn_alpha_per_rad"]),
                f(&row["hpr_fins_cn_alpha_per_rad"]),
            );
            let cp_row = format!("| {mach} | {cp} | {hpr_cp} | {cp_error} |");
            assert!(aero.contains(&slope_row), "the aero page lacks {slope_row}");
            assert!(aero.contains(&cp_row), "the aero page lacks {cp_row}");
            let yaw_row = format!(
                "| {mach} | {:.2} | {:.2} | {} | {} |",
                f(&row["rms_yaw_deg"]),
                f(&row["hpr_cn_over_alpha_at_rms_yaw_per_rad"]),
                signed(format!(
                    "{:.1}%",
                    100.0 * f(&row["cn_alpha_error_at_rms_yaw"])
                )),
                signed(format!("{:.2}", f(&row["cp_error_at_rms_yaw_calibers"]))),
            );
            assert!(aero.contains(&yaw_row), "the aero page lacks {yaw_row}");
            let record_row = format!(
                "| {mach} | {slope} | {hpr} | {slope_error} | {cp} | {hpr_cp} | {cp_error} |"
            );
            assert!(record.contains(&record_row), "ADR-351 lacks {record_row}");
            // The accuracy page gives each error unsigned, as "low" and "forward".
            for figure in [
                slope_error.trim_start_matches('\u{2212}'),
                cp_error.trim_start_matches('\u{2212}'),
            ] {
                assert!(
                    accuracy.contains(figure),
                    "the accuracy page lacks {figure}"
                );
            }
        }
    }

    /// The bands' edges: Mach 1.5 opens the core band, Mach 2.5 closes it.
    #[test]
    fn bands_split_at_mach_1_5_and_2_5() {
        assert_eq!(band(1.4999), "below Mach 1.5");
        assert_eq!(band(1.5), "core, Mach 1.5 to 2.5");
        assert_eq!(band(2.5), "core, Mach 1.5 to 2.5");
        assert_eq!(band(2.5001), "extended, Mach 2.5 to 3.5");
    }
}
