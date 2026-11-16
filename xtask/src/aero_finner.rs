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

/// A row's name in an error: its Mach number, or its place in the reference (counting from 1)
/// when its `mach` can't be read.
fn row_name(row: &Value, index: usize) -> String {
    match number(&row["mach"], "mach") {
        Ok(mach) => format!("the row at Mach {mach}"),
        Err(_) => format!("row {} of the reference", index + 1),
    }
}

/// The airframe's bodies and its one fin set among `parts`, in [`AeroModel::components`]'
/// order: the airframe's bodies, then the pods' bodies, then the lifting surfaces. The split is
/// right only when the design has one fin set, on the airframe, and it is the one part that
/// follows the airframe's bodies, so any other design (no fins, a second fin set, a fin set on a
/// pod, a pod with a body) is refused. A pod with neither body nor fins adds no term and is let
/// be. Tube fins never reach here: HPR Sim refuses them from Mach 0.8, below every row.
fn bodies_and_fins<'a, T>(model: &AeroModel, parts: &'a [T]) -> Result<(&'a [T], &'a [T]), String> {
    let bodies = model.bodies().len();
    let fin_sets = model.fin_sets();
    let on_pods = fin_sets.iter().filter(|set| set.pods.is_some()).count();
    if fin_sets.len() != 1 || on_pods != 0 || parts.len() != bodies + 1 {
        return Err(format!(
            "the design must have one fin set, on the airframe, and no pod with a body: it has \
             {} fin sets ({on_pods} on pods), {} pod sets with a body and {} parts beside its \
             {bodies} bodies",
            fin_sets.len(),
            model.pod_sets().len(),
            parts.len().saturating_sub(bodies)
        ));
    }
    Ok(parts.split_at(bodies))
}

/// A part's slope and center of pressure, calibers aft of the nose tip, from its terms' slopes and
/// moment slopes: `moment / slope / d`. A part with no slope has no center of pressure, so it is
/// refused, naming the Mach number, rather than written as a `null` that `aero --check` would
/// take as equal to a `null`. Terms that cancel to rounding count as no slope, by the model's own
/// test for a body's slope (`slender_station_m` in `hpr-aero`): within 1e-12 of their size.
fn part(mach: f64, what: &str, terms: &[(f64, f64)], d: f64) -> Result<(f64, f64), String> {
    let slope: f64 = terms.iter().map(|(slope, _)| slope).sum();
    let moment: f64 = terms.iter().map(|(_, moment)| moment).sum();
    let size: f64 = terms.iter().map(|(slope, _)| slope.abs()).sum();
    let cp = moment / slope / d;
    if !(slope.is_finite() && slope.abs() > 1e-12 * size && cp.is_finite()) {
        return Err(format!(
            "Mach {mach}: HPR Sim's {what} has a slope of {slope} per radian and a moment slope \
             of {moment} m per radian, so no center of pressure"
        ));
    }
    Ok((slope, cp))
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
    compare(&reference, design, &rocket)
}

/// The comparison of `rocket`, the design named `design`, with `reference`.
fn compare(reference: &Value, design: &str, rocket: &Rocket) -> Result<Value, String> {
    let layout = rocket.layout().map_err(|e| format!("{design}: {e}"))?;
    let model = AeroModel::new(&layout).map_err(|e| format!("{design}: {e}"))?;
    let d = model.reference_diameter_m();
    let moment_reference = number(
        &reference["moment_reference_calibers"],
        "moment_reference_calibers",
    )?;
    let mut rows = Vec::new();
    for (index, row) in reference["rows"]
        .as_array()
        .ok_or("the reference has no `rows`")?
        .iter()
        .enumerate()
    {
        let name = row_name(row, index);
        let field = |key: &str| number(&row[key], key).map_err(|e| format!("{name}: {e}"));
        let mach = field("mach")?;
        if !(mach > 1.0 && mach <= ENVELOPE_TOP_MACH) {
            return Err(format!(
                "the row at Mach {mach} is outside Mach 1 to {ENVELOPE_TOP_MACH}"
            ));
        }
        let cn_alpha = field("cn_alpha")?;
        let cm_alpha = field("cm_alpha")?;
        if !(cn_alpha > 0.0 && cm_alpha < 0.0) {
            return Err(format!(
                "the row at Mach {mach} is not a stable body's: C_N_alpha {cn_alpha}, C_m_alpha {cm_alpha}"
            ));
        }
        let cn_error_percent = field("cn_alpha_probable_error_percent")?;
        let cm_error_percent = field("cm_alpha_probable_error_percent")?;
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
        let terms: Vec<(f64, f64)> = parts
            .iter()
            .map(|p| (p.normal_force.slope_per_rad, p.normal_force.moment_slope_m))
            .collect();
        let (bodies, fins) =
            bodies_and_fins(&model, &terms).map_err(|e| format!("{design}: {e}"))?;
        let (hpr_body, hpr_body_cp) = part(mach, "body", bodies, d)?;
        let (hpr_fins, hpr_fins_cp) = part(mach, "fin set", fins, d)?;
        // hpr at the fit's root-mean-square yaw, where its body lift adds to the slope.
        let mean_squared_yaw_deg2 = field("mean_squared_yaw_deg2")?;
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
        let written = json!({
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
        });
        // A non-finite figure is written as `null`, which `aero --check` would take as equal to
        // a `null`: refuse it here, by name.
        if let Some((key, _)) = written
            .as_object()
            .and_then(|fields| fields.iter().find(|(_, value)| value.is_null()))
        {
            return Err(format!("Mach {mach}: `{key}` is not a finite number"));
        }
        rows.push(written);
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
                 reference_cn_alpha_probable_error is that of the slope, as a fraction (0.02 \
                 for 2%). cn_alpha_error is hpr's over DREV's, minus 1; cp_error_calibers is \
                 hpr's center of pressure less DREV's, positive aft: a center of pressure too \
                 far aft overstates stability. \
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

    /// The pages quote the committed report as it is. The aerodynamics page: each row of its
    /// three tables, its *By band* paragraph, its ranges and its worked example. The accuracy
    /// page: its row's sentences and its known gap's ranges. Each sentence is built from the
    /// report, band by band, and looked for whole, so a figure that happens to stand elsewhere
    /// on a page can't pass for it. The decision record is not held: it says what was found on
    /// its date.
    #[test]
    fn the_pages_quote_the_committed_report() {
        let root = crate::designs::root().unwrap();
        let report = read(&root, FIXTURE).unwrap();
        let reference = read(&root, REFERENCE).unwrap();
        // A page as running text: each run of spaces and line breaks one space, and a quoted
        // note's `>` marks dropped.
        let page = |name: &str| {
            let text = std::fs::read_to_string(root.join(name)).unwrap();
            let words = text.split_whitespace().filter(|word| *word != ">");
            words.collect::<Vec<_>>().join(" ")
        };
        let aero = page("docs/physics/aero.md");
        let accuracy = page("docs/accuracy.md");
        let has = |text: &str, name: &str, sentence: &str| {
            assert!(text.contains(sentence), "{name} lacks: {sentence}");
        };
        let f = |v: &Value| v.as_f64().unwrap();
        // A signed figure as the pages write it, with a true minus sign.
        let signed = |text: String| text.replace('-', "\u{2212}");
        let unsigned = |text: &str| text.trim_start_matches('\u{2212}').to_string();
        let rows = report["rows"].as_array().unwrap();
        let readings = reference["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(readings.len(), 6);
        // Each band's errors as the sentences give them, unsigned: "low" and "forward".
        let mut slopes: Vec<(&str, Vec<String>)> = Vec::new();
        let mut centers: Vec<(&str, Vec<String>)> = Vec::new();
        for (row, reading) in rows.iter().zip(readings) {
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
            has(&aero, "the aero page", &slope_row);
            has(
                &aero,
                "the aero page",
                &format!("| {mach} | {cp} | {hpr_cp} | {cp_error} |"),
            );
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
            has(&aero, "the aero page", &yaw_row);
            // The sentences say "low" and "forward", so every error must be negative.
            assert!(slope_error.starts_with('\u{2212}'), "Mach {mach}");
            assert!(cp_error.starts_with('\u{2212}'), "Mach {mach}");
            let band = row["band"].as_str().unwrap();
            if slopes.last().is_none_or(|(last, _)| *last != band) {
                slopes.push((band, Vec::new()));
                centers.push((band, Vec::new()));
            }
            slopes.last_mut().unwrap().1.push(unsigned(&slope_error));
            centers.last_mut().unwrap().1.push(unsigned(&cp_error));
        }
        // The sentences are written for two rows a band, in this order.
        let names: Vec<&str> = slopes.iter().map(|(band, _)| *band).collect();
        assert_eq!(names, [band(1.0), band(2.0), band(3.0)]);
        assert!(slopes.iter().all(|(_, figures)| figures.len() == 2));
        // Two slope errors: "a and b low", or the one figure when the two round alike.
        let low = |figures: &[String]| {
            if figures[0] == figures[1] {
                format!("{} low at both Mach numbers", figures[0])
            } else {
                format!("{} and {} low", figures[0], figures[1])
            }
        };
        let both = |figures: &[String]| format!("{} and {}", figures[0], figures[1]);
        let (below, core, extended) = (&slopes[0].1, &slopes[1].1, &slopes[2].1);
        let (below_cp, core_cp, extended_cp) = (&centers[0].1, &centers[1].1, &centers[2].1);

        has(
            &aero,
            "the aero page's By band",
            &format!(
                "From Mach 1.5 to 2.5, the band this comparison was added for, the slope reads \
                 {} and the center of pressure {} calibers forward. Below Mach 1.5 the slope \
                 reads {}, with the center of pressure {} calibers forward. From Mach 2.5 to \
                 3.5 the slope reads {}, and the center of pressure {} calibers forward.",
                low(core),
                both(core_cp),
                low(below),
                both(below_cp),
                low(extended),
                both(extended_cp),
            ),
        );

        // The ranges: the least and the greatest of a column, at the pages' rounding.
        let range = |values: Vec<f64>, digits: usize, unit: &str| {
            let least = values.iter().copied().fold(f64::INFINITY, f64::min);
            let most = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            format!("{least:.digits$}{unit} to {most:.digits$}{unit}")
        };
        let column = |rows: &[Value], key: &str| rows.iter().map(|r| f(&r[key])).collect();
        let of = |g: &dyn Fn(&Value) -> f64| rows.iter().map(g).collect::<Vec<f64>>();
        let fits = range(column(readings, "cn_alpha_probable_error_percent"), 0, "%");
        let machs = range(column(rows, "mach"), 2, "");
        let slope_range = range(of(&|r| -100.0 * f(&r["cn_alpha_error"])), 1, "%");
        let cp_range = range(of(&|r| -f(&r["cp_error_calibers"])), 2, "");
        let yaw_gain = range(
            of(&|r| 100.0 * (f(&r["cn_alpha_error_at_rms_yaw"]) - f(&r["cn_alpha_error"]))),
            1,
            "",
        );
        let yaw_shift = range(
            of(&|r| f(&r["cp_error_calibers"]) - f(&r["cp_error_at_rms_yaw_calibers"])),
            2,
            "",
        );
        let squared_yaw = range(column(readings, "mean_squared_yaw_deg2"), 1, "");

        // The comparison's row on the accuracy page, cut at its line's end and then at the next
        // cell, so that its sentences can't be found in the prose below the table.
        let raw = std::fs::read_to_string(root.join("docs/accuracy.md")).unwrap();
        let row_start = raw
            .find("| [Aerodynamics](physics/aero.md#the-basic-finners-normal-force-against-drevs-range) |")
            .expect("the accuracy page has the comparison's row");
        let row_line = raw[row_start..].lines().next().unwrap();
        let row_line = row_line.split_whitespace().collect::<Vec<_>>().join(" ");
        let row_text = row_line.split(" | [").next().unwrap();
        has(
            row_text,
            "the accuracy page's row",
            &format!(
                "the slope low on every row, where the fits' probable errors are {fits}: {} \
                 from Mach 1.5 to 2.5, {} below 1.5, and {} from Mach 2.5 to 3.5. The center \
                 of pressure forward of the measured one on every row, which understates \
                 stability: {} calibers from Mach 1.5 to 2.5, {} below, {} above. At each \
                 fit's root-mean-square yaw, about 1°, the slope's miss is {yaw_gain} \
                 percentage points smaller.",
                low(core),
                low(below),
                low(extended),
                both(core_cp),
                both(below_cp),
                both(extended_cp),
            ),
        );
        has(
            row_text,
            "the accuracy page's row",
            &format!("Mach {machs}, six Mach numbers"),
        );
        // The known gap on the accuracy page and the summary at the aero page's top.
        let gap = format!(
            "from Mach {machs} show HPR Sim's slope {slope_range} low and its center of pressure \
             {cp_range} calibers forward"
        );
        has(&accuracy, "the accuracy page's known gap", &gap);
        has(&aero, "the aero page's summary", &gap);
        has(
            &aero,
            "the aero page's In short",
            &format!(
                "HPR Sim's normal-force slope reads {slope_range} below the measured one from \
                 Mach {machs}, and its center of pressure lies {cp_range} calibers forward of \
                 the measured one."
            ),
        );
        has(
            &aero,
            "the aero page's trust note",
            &format!("the fits' own, {fits} on the slope"),
        );
        has(
            &aero,
            "the aero page at the root-mean-square yaw",
            &format!("mean squared yaw, {squared_yaw} square degrees."),
        );
        has(
            &aero,
            "the aero page at the root-mean-square yaw",
            &format!(
                "the slope's miss is {yaw_gain} percentage points smaller, and the center of \
                 pressure {yaw_shift} calibers further forward:"
            ),
        );

        // What it means: every slope low by more than its fit's probable error; the center of
        // pressure forward by more than its probable error on all rows but those named, narrowly
        // on one, and by more than three times it on every row from a Mach number up.
        // "Both hold" at each fit's root-mean-square yaw too.
        assert!(
            rows.windows(2).all(|w| f(&w[0]["mach"]) < f(&w[1]["mach"])),
            "rows in Mach order"
        );
        for row in rows {
            let error = f(&row["reference_cn_alpha_probable_error"]);
            assert!(-f(&row["cn_alpha_error"]) > error, "{row}");
            assert!(-f(&row["cn_alpha_error_at_rms_yaw"]) > error, "{row}");
            assert!(f(&row["cp_error_calibers"]) < 0.0, "{row}");
            assert!(f(&row["cp_error_at_rms_yaw_calibers"]) < 0.0, "{row}");
        }
        // Each row forward by at most its probable error, "narrowly" more (at most 1.5 times),
        // or "more than three times it"; none in between.
        let times = |row: &Value| {
            -f(&row["cp_error_calibers"]) / f(&row["reference_cp_probable_error_calibers"])
        };
        assert!(
            rows.iter().all(|r| times(r) <= 1.5 || times(r) > 3.0),
            "no row between 1.5 and 3 times its probable error"
        );
        let within: Vec<String> = rows
            .iter()
            .filter(|r| times(r) <= 1.0)
            .map(|r| r["mach"].to_string())
            .collect();
        let narrow: Vec<&Value> = rows
            .iter()
            .filter(|r| times(r) > 1.0 && times(r) <= 1.5)
            .collect();
        let beyond = rows.iter().position(|r| times(r) > 3.0).unwrap();
        assert!(
            rows[beyond..].iter().all(|r| times(r) > 3.0),
            "the rows past three times are a tail"
        );
        assert_eq!(
            (within.len(), narrow.len()),
            (1, 1),
            "the sentence names one row of each"
        );
        has(
            &aero,
            "the aero page's What it means",
            &format!(
                "Its center of pressure is forward on every row, and by more than the probable \
                 error on all but Mach {}: narrowly at Mach {}, {:.2} against {:.2} calibers, \
                 and by more than three times it from Mach {} up.",
                within[0],
                narrow[0]["mach"],
                -f(&narrow[0]["cp_error_calibers"]),
                f(&narrow[0]["reference_cp_probable_error_calibers"]),
                rows[beyond]["mach"],
            ),
        );

        // The worked example and the margins, at the first row of the band the comparison was
        // added for.
        let at = rows.iter().position(|r| r["band"] == band(2.0)).unwrap();
        let (row, reading) = (&rows[at], &readings[at]);
        let (cn, cm) = (f(&reading["cn_alpha"]), f(&reading["cm_alpha"]));
        let about = f(&report["moment_reference_calibers"]);
        let measured = f(&row["reference_cp_calibers"]);
        let percent = f(&reading["cn_alpha_probable_error_percent"])
            .hypot(f(&reading["cm_alpha_probable_error_percent"]).max(PRINTED_ZERO_PERCENT));
        has(
            &aero,
            "the aero page's worked example",
            &format!(
                "at Mach {} the table gives $C_{{N\\alpha}} = {cn}$ and $C_{{m\\alpha}} = {}$, so \
                 the center of pressure is ${about:.2} + {}/{cn} = {measured:.2}$ from the nose tip",
                f(&row["mach"]),
                cm,
                cm.abs(),
            ),
        );
        has(
            &aero,
            "the aero page's worked example",
            &format!(
                "Together they are {percent:.2}% of the {:.2}-caliber arm, {:.2} calibers.",
                measured - about,
                f(&row["reference_cp_probable_error_calibers"]),
            ),
        );
        has(
            &aero,
            "the aero page's margins",
            &format!(
                "the measured margin at Mach {} is {:.2} calibers and HPR Sim's is {:.2}.",
                f(&row["mach"]),
                measured - about,
                f(&row["hpr_cp_calibers"]) - about,
            ),
        );
    }

    /// The committed reference and design, as values a test can change before comparing them.
    fn committed() -> (Value, String, Value) {
        let root = crate::designs::root().unwrap();
        let reference = read(&root, REFERENCE).unwrap();
        let design = reference["design"].as_str().unwrap().to_string();
        let rocket = read(&root, &format!("validation/designs/{design}")).unwrap();
        (reference, design, rocket)
    }

    fn compared(reference: &Value, design: &str, rocket: &Value) -> Result<Value, String> {
        compare(
            reference,
            design,
            &serde_json::from_value(rocket.clone()).unwrap(),
        )
    }

    /// A part with no slope has no center of pressure: it is refused by its Mach number, not
    /// written as `null`.
    #[test]
    fn a_part_with_no_slope_is_refused_by_its_mach_number() {
        let error = part(1.832, "body", &[(0.0, 0.0)], 0.03).unwrap_err();
        assert!(
            error.contains("Mach 1.832") && error.contains("body"),
            "{error}"
        );
        // Two terms that cancel are the same case, and so are terms that cancel to rounding.
        assert!(part(2.0, "fin set", &[(1.0, 0.2), (-1.0, 0.1)], 0.03).is_err());
        // A slope with a moment that isn't finite has no center of pressure either.
        assert!(part(2.0, "body", &[(1.0, f64::NAN)], 0.03).is_err());
        assert!(part(2.0, "body", &[(f64::INFINITY, 0.1)], 0.03).is_err());
        assert!(
            part(
                2.0,
                "fin set",
                &[(0.3, 0.2), (-0.1, 0.1), (-0.2, 0.0)],
                0.03
            )
            .is_err()
        );
        let (slope, cp) = part(2.0, "fin set", &[(2.0, 0.6), (1.0, 0.3)], 0.03).unwrap();
        assert_eq!(slope, 3.0);
        assert!((cp - 10.0).abs() < 1e-12);
    }

    /// The committed design compares; one with a second fin set, or none, is refused, since HPR
    /// Sim's slope is split into the bodies and one fin set.
    #[test]
    fn a_design_with_other_than_one_fin_set_is_refused() {
        let (reference, design, mut rocket) = committed();
        assert!(compared(&reference, &design, &rocket).is_ok());
        let children = rocket["stages"][0]["components"][1]["children"]
            .as_array_mut()
            .unwrap();
        let mut second = children[0].clone();
        second["id"] = json!("fins-forward");
        second["position"]["station_m"] = json!(0.15);
        children.push(second);
        let error = compared(&reference, &design, &rocket).unwrap_err();
        assert!(error.contains("it has 2 fin sets"), "{error}");
        // A finless body is the other side of the same rule.
        let (_, _, mut finless) = committed();
        finless["stages"][0]["components"][1]["children"] = json!([]);
        let error = compared(&reference, &design, &finless).unwrap_err();
        assert!(error.contains("it has 0 fin sets"), "{error}");
        // A pod's body in the fins' place: one part after the airframe's bodies, but no fin set.
        let pod = json!({
            "id": "pod",
            "name": "",
            "part": {"pod_set": {"count": 1, "radial_offset_m": 0.03, "angle_rad": 0.0}},
            "position": {"from": "absolute", "station_m": 0.2},
            "children": [{
                "id": "pod-nose",
                "name": "",
                "part": finless["stages"][0]["components"][0]["part"].clone(),
            }],
        });
        finless["stages"][0]["components"][1]["children"] = json!([pod]);
        let error = compared(&reference, &design, &finless).unwrap_err();
        assert!(
            error.contains("it has 0 fin sets (0 on pods), 1 pod sets"),
            "{error}"
        );
        // The same pod beside the airframe's fins: one fin set, but two parts after the bodies.
        let (_, _, mut podded) = committed();
        let children = podded["stages"][0]["components"][1]["children"]
            .as_array_mut()
            .unwrap();
        children.push(finless["stages"][0]["components"][1]["children"][0].clone());
        let error = compared(&reference, &design, &podded).unwrap_err();
        assert!(
            error.contains("it has 1 fin sets (0 on pods), 1 pod sets"),
            "{error}"
        );
        // The fins on a pod's tube, the airframe finless: refused, with the fins counted as on a
        // pod. The pod's tube adds a part, so the part count refuses this one too.
        let (_, _, mut pod_fins) = committed();
        let fins = pod_fins["stages"][0]["components"][1]["children"][0].clone();
        let mut pod = finless["stages"][0]["components"][1]["children"][0].clone();
        let mut tube = pod_fins["stages"][0]["components"][1].clone();
        tube["id"] = json!("pod-tube");
        tube["children"] = json!([fins]);
        pod["children"] = json!([tube]);
        pod_fins["stages"][0]["components"][1]["children"] = json!([pod]);
        let error = compared(&reference, &design, &pod_fins).unwrap_err();
        assert!(error.contains("it has 1 fin sets (1 on pods)"), "{error}");
        // The same fins on a phantom tube (no length, no radius), which adds no part: only the
        // count of fin sets on pods refuses it.
        let tube = &mut pod_fins["stages"][0]["components"][1]["children"][0]["children"][0];
        tube["part"]["body_tube"]["length_m"] = json!(0.0);
        tube["part"]["body_tube"]["outer_radius_m"] = json!(0.0);
        tube["part"]["body_tube"]["thickness_m"] = json!(0.0);
        let error = compared(&reference, &design, &pod_fins).unwrap_err();
        assert!(
            error.contains("(1 on pods), 0 pod sets with a body and 1 parts"),
            "{error}"
        );
    }

    /// A row with a missing or unreadable field is named by its Mach number, or by its place in
    /// the reference when `mach` itself is the missing field.
    #[test]
    fn a_row_with_a_missing_field_is_named() {
        let (reference, design, rocket) = committed();
        let mut missing = reference.clone();
        missing["rows"][2]["cn_alpha"] = Value::Null;
        let error = compared(&missing, &design, &rocket).unwrap_err();
        assert!(
            error.contains("the row at Mach 1.832") && error.contains("cn_alpha"),
            "{error}"
        );
        let mut no_mach = reference.clone();
        no_mach["rows"][3]["mach"] = json!("2.375");
        let error = compared(&no_mach, &design, &rocket).unwrap_err();
        assert!(
            error.contains("row 4 of the reference") && error.contains("mach"),
            "{error}"
        );
        // A figure that comes out non-finite is refused by its field, not written as `null`.
        let mut tiny = reference.clone();
        tiny["rows"][1]["cn_alpha"] = json!(5e-324);
        let error = compared(&tiny, &design, &rocket).unwrap_err();
        assert!(
            error.contains("Mach 1.293") && error.contains("is not a finite number"),
            "{error}"
        );
    }

    /// The reference names the report it was typed from by the hash the reference library pins,
    /// so the cells can be checked against the very file.
    #[test]
    fn the_reference_names_the_pinned_report() {
        let root = crate::designs::root().unwrap();
        let reference = read(&root, REFERENCE).unwrap();
        let text = std::fs::read_to_string(root.join("validation/refs.lock.toml")).unwrap();
        let lock: toml::Value = toml::from_str(&text).unwrap();
        let pin = lock["file"]
            .as_array()
            .unwrap()
            .iter()
            .find(|file| file["name"].as_str() == Some("drev-tm-9703"))
            .expect("the lock pins DREV-TM-9703");
        assert_eq!(
            reference["source_sha256"].as_str(),
            pin["sha256"].as_str(),
            "the reference's source_sha256 is the lock's pin"
        );
        // A YYYY-MM-DD string, no earlier than the report's first fetch (THIRD-PARTY-NOTICES.md).
        let typed_on = reference["typed_on"].as_str().unwrap();
        let fields: Vec<&str> = typed_on.split('-').collect();
        let digits =
            |text: &str, n: usize| text.len() == n && text.bytes().all(|c| c.is_ascii_digit());
        assert!(
            fields.len() == 3
                && digits(fields[0], 4)
                && digits(fields[1], 2)
                && digits(fields[2], 2),
            "typed_on {typed_on} is not YYYY-MM-DD"
        );
        assert!(typed_on >= "2026-10-18", "typed_on {typed_on}");
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
