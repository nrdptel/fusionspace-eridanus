//! The Basic Finner's drag and pitch damping against DREV-TM-9703's range fits (M1.14b2),
//! written by `cargo xtask aero` to `validation/fixtures/aero/basic-finner-drag-damping.json`.
//!
//! - **The reference** (Dupuis and Hathaway 1997): each row's `C_X0` and `C_mq` from Table IX
//!   (PDF pp. 48–49), each shot's own `C_X0` from Table VIII (PDF pp. 44–46) and its Reynolds
//!   number from Table V (PDF p. 41), typed into `validation/fixtures/aero/basic-finner-drev.json`.
//! - **The drag.** HPR Sim's zero-lift drag buildup of the committed design at the row's Mach
//!   number and the mean Reynolds number of its shots. The design can't draw the range model's
//!   fins, each a single wedge sharp at its leading edge and 0.08 calibers thick at its trailing
//!   edge: it draws them square, 0.08 calibers thick. So the square edges' two terms are taken out
//!   (the blunt leading edge's stagnation pressure, which the wedge doesn't feel, and HPR Sim's
//!   trailing-edge rule) and the wedge's are put in, from cited sources
//!   (`validation/fixtures/aero/basic-finner-wedge.json`,
//!   `docs/research/basic-finner-wedge-drag.md`): the faces' pressure behind an attached oblique
//!   shock ([`hpr_aero::blunt_tip::wedge_pressure_coefficient`], NACA Report 1135), or Vincenti
//!   and Wagoner's transonic value where the bow wave stands off; and the trailing edges' base
//!   drag from two-dimensional measurements with a turbulent boundary layer, a band. The report
//!   gives the drag as drawn and with HPR Sim's own trailing-edge rule beside it.
//! - **The pitch damping** is HPR Sim's read-out about the report's moment reference, 5.5
//!   calibers from the nose tip ([`hpr_aero::AeroModel::pitch_damping`]), per radian of
//!   `q d/(2V)`, the convention DREV's eq. \[1\] implies.
//!
//! No target is set: each error is reported as it is, beside the measurement's own probable error
//! and the spread of the row's shots.

use std::path::Path;

use hpr_aero::blunt_tip::{wedge_detachment_angle_rad, wedge_pressure_coefficient};
use hpr_aero::drag::{base_drag_coefficient, stagnation_drag_coefficient};
use hpr_aero::{AeroModel, DragConditions, Flow};
use hpr_design::{FinCrossSection, Rocket};
use serde_json::{Value, json};

use crate::aero_finner::{REFERENCE, band, number, read, row_name};

pub const FIXTURE: &str = "validation/fixtures/aero/basic-finner-drag-damping.json";
pub const WEDGE: &str = "validation/fixtures/aero/basic-finner-wedge.json";
/// The table prints each probable error in whole percent, so one printed as 0 is below this.
const PRINTED_ZERO_PERCENT: f64 = 0.5;
/// The ratio of specific heats of air.
const GAMMA: f64 = 1.4;
/// How far the transonic similarity parameter may sit from the one the detached faces' value was
/// read at: the figure's reading error is about this wide in it.
const SIMILARITY_TOLERANCE: f64 = 0.005;

/// The fixture, from the committed design, the committed reference and the wedge's figures.
pub fn generate(root: &Path) -> Result<Value, String> {
    let reference = read(root, REFERENCE)?;
    let wedge = read(root, WEDGE)?;
    let design = reference["design"]
        .as_str()
        .ok_or("the reference names no `design`")?;
    let rocket: Rocket =
        serde_json::from_value(read(root, &format!("validation/designs/{design}"))?)
            .map_err(|e| format!("{design}: {e}"))?;
    compare(&reference, &wedge, design, &rocket)
}

/// The fin set's frontal area `N t s` over the reference area, from the drag buildup's own terms;
/// refused unless the design has one fin set, square in section and unswept, the shape whose two
/// edge terms the comparison takes out.
fn square_fins_frontal_ratio(model: &AeroModel) -> Result<f64, String> {
    let fins: Vec<_> = model
        .drag_terms()
        .iter()
        .filter_map(|terms| terms.fins)
        .collect();
    match fins.as_slice() {
        [fins]
            if fins.cross_section == FinCrossSection::Square
                && fins.leading_edge_sweep_rad == 0.0 =>
        {
            Ok(fins.frontal_area_ratio)
        }
        _ => Err(format!(
            "the design must have one fin set, square in section and unswept: it has {} fin sets",
            fins.len()
        )),
    }
}

/// The fins' part of [`AeroModel::pitch_damping`]: its sum taken over the lifting surfaces alone,
/// `−(2/d²) Σ C_Nα,i (s_i − x_r)(c_i − x_r)`, the rest being the body's.
fn fins_pitch_damping(
    model: &AeroModel,
    mach: f64,
    reference_station_m: f64,
) -> Result<f64, hpr_aero::AeroError> {
    let flow = Flow::axial(mach);
    let x_r = reference_station_m;
    let mut sum = 0.0;
    for index in model.fin_set_start()..model.component_count() {
        let force = model.component_normal_force(index, &flow)?;
        let station = model.component_station_m(index, mach)?;
        sum += (station - x_r) * (force.moment_slope_m - force.slope_per_rad * x_r);
    }
    let d = model.reference_diameter_m();
    Ok(-2.0 * sum / (d * d))
}

/// The wedge faces' drag on the reference area at `mach`, as `(value, low, high)`: behind an
/// attached shock, the exact face pressure on the frontal area, no band; where the bow wave
/// stands off, Vincenti and Wagoner's front-wedge value read at the row's similarity parameter,
/// with its reading error. `deflection_rad` is a face's angle to the chord, so the wedge's
/// thickness ratio on its double wedge's chord is `tan δ`.
fn wedge_faces(
    wedge: &Value,
    mach: f64,
    deflection_rad: f64,
    frontal: f64,
) -> Result<(f64, f64, f64), String> {
    let at = |e: hpr_aero::AeroError| format!("Mach {mach}: {e}");
    if deflection_rad <= wedge_detachment_angle_rad(mach).map_err(at)? {
        let faces = frontal * wedge_pressure_coefficient(mach, deflection_rad).map_err(at)?;
        return Ok((faces, faces, faces));
    }
    let detached = &wedge["detached_faces"];
    let field = |key: &str| number(&detached[key], key).map_err(|e| format!("detached_faces: {e}"));
    if field("mach")? != mach {
        return Err(format!(
            "Mach {mach}: the wedge's bow wave stands off, and the detached faces' value is for \
             Mach {}",
            field("mach")?
        ));
    }
    let thickness_ratio = deflection_rad.tan();
    let similarity = (mach * mach - 1.0) / ((GAMMA + 1.0) * thickness_ratio).powf(2.0 / 3.0);
    if (similarity - field("similarity_parameter")?).abs() > SIMILARITY_TOLERANCE {
        return Err(format!(
            "Mach {mach}: the similarity parameter is {similarity}, not the {} the detached \
             faces' value was read at",
            field("similarity_parameter")?
        ));
    }
    // c_d on the double wedge's chord, twice that on the fin's own (half as long), then the
    // four fins' planform over the reference area, each one caliber square: 4 d²/(π d²/4).
    let to_reference = |generalized: f64| {
        let on_chord =
            generalized * thickness_ratio.powf(5.0 / 3.0) / (GAMMA + 1.0).powf(1.0 / 3.0);
        2.0 * on_chord * 16.0 / std::f64::consts::PI
    };
    let (read, error) = (
        field("front_wedge_generalized_drag")?,
        field("reading_error")?,
    );
    Ok((
        to_reference(read),
        to_reference(read - error),
        to_reference(read + error),
    ))
}

/// The trailing edges' two-dimensional base-drag band at `mach`, on their own area.
fn trailing_edge_band(wedge: &Value, mach: f64) -> Result<(f64, f64), String> {
    let rows = wedge["trailing_edge_base"]
        .as_array()
        .ok_or("the wedge's figures have no `trailing_edge_base`")?;
    let row = rows
        .iter()
        .find(|row| row["mach"].as_f64() == Some(mach))
        .ok_or(format!(
            "the wedge's figures have no trailing edge at Mach {mach}"
        ))?;
    let low = number(&row["low"], "low")?;
    let high = number(&row["high"], "high")?;
    if !(0.0 < low && low <= high) {
        return Err(format!(
            "Mach {mach}: the trailing edge's band {low} to {high}"
        ));
    }
    Ok((low, high))
}

/// The comparison of `rocket`, the design named `design`, with `reference`.
fn compare(
    reference: &Value,
    wedge: &Value,
    design: &str,
    rocket: &Rocket,
) -> Result<Value, String> {
    let layout = rocket.layout().map_err(|e| format!("{design}: {e}"))?;
    let model = AeroModel::new(&layout).map_err(|e| format!("{design}: {e}"))?;
    let d = model.reference_diameter_m();
    let length_m = model.length_m();
    let x_ref_calibers = number(
        &reference["moment_reference_calibers"],
        "moment_reference_calibers",
    )?;
    let geometry = &reference["geometry"];
    let thickness = number(
        &geometry["fin_base_thickness_calibers"],
        "fin_base_thickness_calibers",
    )?;
    let chord = number(&geometry["fin_chord_calibers"], "fin_chord_calibers")?;
    // Each face of the single wedge turns the flow by half the trailing edge's thickness over
    // the chord.
    let deflection = (0.5 * thickness / chord).atan();
    let frontal = square_fins_frontal_ratio(&model).map_err(|e| format!("{design}: {e}"))?;
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
        let cx0 = field("cx0")?;
        let cmq = field("cmq")?;
        let cx0_error_percent = field("cx0_probable_error_percent")?;
        let cmq_error_percent = field("cmq_probable_error_percent")?;
        if !(cx0 > 0.0 && cmq < 0.0 && cx0_error_percent >= 0.0 && cmq_error_percent >= 0.0) {
            return Err(format!(
                "{name}: C_X0 {cx0} must be positive, C_mq {cmq} negative and their probable \
                 errors not negative"
            ));
        }
        let fitted = row["cx0_fitted_in_the_row"]
            .as_bool()
            .ok_or(format!("{name}: no `cx0_fitted_in_the_row`"))?;
        let shots = row["shots"]
            .as_array()
            .filter(|shots| !shots.is_empty())
            .ok_or(format!("{name}: no `shots`"))?;
        let (mut low, mut high, mut reynolds) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
        for shot in shots {
            let what = |key: &str| {
                number(&shot[key], &format!("a shot's {key}")).map_err(|e| format!("{name}: {e}"))
            };
            let (shot_cx0, shot_re) = (what("cx0")?, what("reynolds_on_length")?);
            if !(shot_cx0 > 0.0 && shot_re > 0.0) {
                return Err(format!(
                    "{name}: a shot's C_X0 or Reynolds number is not positive"
                ));
            }
            low = low.min(shot_cx0);
            high = high.max(shot_cx0);
            reynolds += shot_re;
        }
        // The shots' mean Reynolds number, on the model's length, per meter.
        let reynolds_per_m = reynolds / shots.len() as f64 / length_m;
        let at = |e: hpr_aero::AeroError| format!("Mach {mach}: {e}");
        let drag = model
            .drag(
                &Flow::axial(mach),
                &DragConditions::coasting(reynolds_per_m),
            )
            .map_err(at)?;
        let leading_edge = frontal * stagnation_drag_coefficient(mach).map_err(at)?;
        let trailing_edge = frontal * base_drag_coefficient(mach).map_err(at)?;
        let (faces, faces_low, faces_high) = wedge_faces(wedge, mach, deflection, frontal)?;
        let (base_low, base_high) = trailing_edge_band(wedge, mach)?;
        let (edge_low, edge_high) = (frontal * base_low, frontal * base_high);
        let hpr_as_drawn = drag.zero_lift_coefficient;
        // Everything HPR Sim draws, less the square edges' two terms.
        let unedged = hpr_as_drawn - leading_edge - trailing_edge;
        let hpr = unedged + faces + 0.5 * (edge_low + edge_high);
        let (hpr_low, hpr_high) = (
            unedged + faces_low + edge_low,
            unedged + faces_high + edge_high,
        );
        let with_its_rule = unedged + faces + trailing_edge;
        let x_ref_m = x_ref_calibers * d;
        let hpr_cmq = model.pitch_damping(mach, x_ref_m).map_err(at)?;
        let hpr_cmq_fins = fins_pitch_damping(&model, mach, x_ref_m).map_err(at)?;
        let cx0_error = hpr / cx0 - 1.0;
        let cmq_error = hpr_cmq / cmq - 1.0;
        println!(
            "basic-finner DREV Mach {mach:<5} C_D0 {hpr:.3} ({hpr_low:.3}-{hpr_high:.3}; as drawn \
             {hpr_as_drawn:.3}) against C_X0 {cx0:.3} ({:+.1}%; shots {low:.3}-{high:.3}); C_mq \
             {hpr_cmq:.0} against {cmq:.0} ({:+.1}%, fit {cmq_error_percent}%)",
            100.0 * cx0_error,
            100.0 * cmq_error
        );
        let written = json!({
            "mach": mach,
            "band": band(mach),
            "reynolds_per_m": reynolds_per_m,
            "reference_cx0": cx0,
            "reference_cx0_probable_error": 0.01 * cx0_error_percent.max(PRINTED_ZERO_PERCENT),
            "reference_cx0_fitted_in_the_row": fitted,
            "shots": shots.len(),
            "shots_cx0_low": low,
            "shots_cx0_high": high,
            "hpr_cd0_as_drawn": hpr_as_drawn,
            "hpr_friction": drag.friction,
            "hpr_pressure": drag.pressure,
            "hpr_base": drag.base,
            "hpr_fins_leading_edge": leading_edge,
            "hpr_fins_trailing_edge": trailing_edge,
            "wedge_shock_attached": faces_low == faces_high,
            "wedge_faces": faces,
            "wedge_faces_low": faces_low,
            "wedge_faces_high": faces_high,
            "wedge_trailing_edge_low": edge_low,
            "wedge_trailing_edge_high": edge_high,
            "wedge_trailing_edge_at_vacuum": frontal * 2.0 / (GAMMA * mach * mach),
            "hpr_cd0": hpr,
            "hpr_cd0_low": hpr_low,
            "hpr_cd0_high": hpr_high,
            "cd0_error": cx0_error,
            "cd0_error_low": hpr_low / cx0 - 1.0,
            "cd0_error_high": hpr_high / cx0 - 1.0,
            "hpr_cd0_with_its_trailing_edge_rule": with_its_rule,
            "cd0_error_with_its_trailing_edge_rule": with_its_rule / cx0 - 1.0,
            "cd0_error_as_drawn": hpr_as_drawn / cx0 - 1.0,
            "reference_cmq": cmq,
            "reference_cmq_probable_error": 0.01 * cmq_error_percent.max(PRINTED_ZERO_PERCENT),
            "hpr_cmq": hpr_cmq,
            "hpr_cmq_fins": hpr_cmq_fins,
            "hpr_cmq_body": hpr_cmq - hpr_cmq_fins,
            "cmq_error": cmq_error,
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
        "note": "hpr's Basic Finner zero-lift drag coefficient (on pi d^2/4) and pitch-damping \
                 coefficient C_mq (per radian of q d/(2V), about 5.5 calibers from the nose tip) \
                 against DREV-TM-9703 Table IX's range fits (Dupuis and Hathaway 1997, PDF pp. \
                 48-49; validation/fixtures/aero/basic-finner-drev.json). reynolds_per_m is the \
                 mean of the row's shots' Reynolds numbers (Table V) over the model's length. \
                 hpr_cd0_as_drawn is the buildup of the committed design, whose fins are square, \
                 0.08 calibers thick; hpr_fins_leading_edge and hpr_fins_trailing_edge are its \
                 square edges' two terms. hpr_cd0 is the buildup with those two taken out and the \
                 single wedge's put in (validation/fixtures/aero/basic-finner-wedge.json): \
                 wedge_faces, the faces' pressure behind an attached oblique shock (NACA Report \
                 1135) or, where wedge_shock_attached is false, Vincenti and Wagoner's transonic \
                 value with its reading error as wedge_faces_low and _high; and the middle of the \
                 trailing edges' two-dimensional turbulent base-drag band, wedge_trailing_edge_low \
                 to _high. hpr_cd0_low and _high take both bands' ends; \
                 wedge_trailing_edge_at_vacuum is the most a base can add, at zero pressure. \
                 hpr_cd0_with_its_trailing_edge_rule keeps HPR Sim's own trailing-edge rule in \
                 place of the band. Each *error is hpr's over DREV's, minus 1. \
                 reference_cx0_probable_error and reference_cmq_probable_error are the printed \
                 ones as fractions, one printed as 0% taken as 0.5%; where \
                 reference_cx0_fitted_in_the_row is false, the table held C_X0 and each shot had \
                 its own, whose range is shots_cx0_low to _high. hpr_cmq is \
                 hpr_aero::AeroModel::pitch_damping, hpr_cmq_fins its fins' part. No target is \
                 set (M1.14b2). Bands: ADR-143's, with the rows below Mach 1.5 apart.",
        "id": reference["id"],
        "design": design,
        "reference_diameter_m": d,
        "moment_reference_calibers": x_ref_calibers,
        "wedge_deflection_rad": deflection,
        "fins_frontal_area_ratio": frontal,
        "rows": rows,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The aerodynamics page's two tables and the figures its sentences and the accuracy page
    /// quote come from the committed report, so a figure that drifts from it on a page can't pass
    /// for it. The decision record is not held: it says what was found on its date.
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
        // A figure as the pages write it, with a true minus sign.
        let minus = |text: String| text.replace('-', "\u{2212}");
        let percent = |x: f64| minus(format!("{:+.1}%", 100.0 * x));
        let unsigned = |x: f64| format!("{:.1}%", 100.0 * x.abs());
        let rows = report["rows"].as_array().unwrap();
        let readings = reference["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(readings.len(), 6);
        for (row, reading) in rows.iter().zip(readings) {
            let mach = f(&row["mach"]);
            let unedged = f(&row["hpr_cd0_as_drawn"])
                - f(&row["hpr_fins_leading_edge"])
                - f(&row["hpr_fins_trailing_edge"]);
            let drag_row = format!(
                "| {mach} | {:.3} | {:.3} to {:.3} | {unedged:.3} | {:.3} | {:.3} to {:.3} | {:.3} \
                 ({:.3} to {:.3}) | {} ({} to {}) |",
                f(&row["reference_cx0"]),
                f(&row["shots_cx0_low"]),
                f(&row["shots_cx0_high"]),
                f(&row["wedge_faces"]),
                f(&row["wedge_trailing_edge_low"]),
                f(&row["wedge_trailing_edge_high"]),
                f(&row["hpr_cd0"]),
                f(&row["hpr_cd0_low"]),
                f(&row["hpr_cd0_high"]),
                percent(f(&row["cd0_error"])),
                percent(f(&row["cd0_error_low"])),
                percent(f(&row["cd0_error_high"])),
            );
            has(&aero, "the aero page", &drag_row);
            let damping_row = minus(format!(
                "| {mach} | {:.1} ({}%) | {:.1} | {:.1} | {:.1} |",
                f(&row["reference_cmq"]),
                f(&reading["cmq_probable_error_percent"]),
                f(&row["hpr_cmq_body"]),
                f(&row["hpr_cmq_fins"]),
                f(&row["hpr_cmq"]),
            ));
            has(
                &aero,
                "the aero page",
                &format!("{damping_row} {} |", percent(f(&row["cmq_error"]))),
            );
        }
        let at = |i: usize, key: &str| f(&rows[i][key]);
        // The bands, as both pages give them, unsigned: "high" and "low" say which way.
        let by_band = format!(
            "From Mach 1.5 to 2.5, the band this comparison was added for, the drag reads {} and {} \
             low and the damping {} and {} low. Below Mach 1.5 the drag reads {} high and {} low, \
             and the damping {} high and {} low. From Mach 2.5 to 3.5 the drag reads {} and {} low \
             and the damping {} and {} low.",
            unsigned(at(2, "cd0_error")),
            unsigned(at(3, "cd0_error")),
            unsigned(at(2, "cmq_error")),
            unsigned(at(3, "cmq_error")),
            unsigned(at(0, "cd0_error")),
            unsigned(at(1, "cd0_error")),
            unsigned(at(0, "cmq_error")),
            unsigned(at(1, "cmq_error")),
            unsigned(at(4, "cd0_error")),
            unsigned(at(5, "cd0_error")),
            unsigned(at(4, "cmq_error")),
            unsigned(at(5, "cmq_error")),
        );
        has(&aero, "the aero page", &by_band);
        let accuracy_row = format!(
            "the drag {} and {} low from Mach 1.5 to 2.5, {} and {} low from Mach 2.5 to 3.5, \
             below every shot of its row and below DREV's even with the fins' bases at a vacuum; \
             below Mach 1.5, {} high at Mach 1.077, above every shot, and {} low at 1.293, inside \
             the shots' spread. The cause is not found. The pitch damping {} and {} low from Mach \
             1.5 to 2.5, {} and {} low from 2.5 to 3.5, and {} low at Mach 1.293",
            unsigned(at(2, "cd0_error")),
            unsigned(at(3, "cd0_error")),
            unsigned(at(4, "cd0_error")),
            unsigned(at(5, "cd0_error")),
            unsigned(at(0, "cd0_error")),
            unsigned(at(1, "cd0_error")),
            unsigned(at(2, "cmq_error")),
            unsigned(at(3, "cmq_error")),
            unsigned(at(4, "cmq_error")),
            unsigned(at(5, "cmq_error")),
            unsigned(at(1, "cmq_error")),
        );
        has(&accuracy, "the accuracy page", &accuracy_row);
        has(
            &accuracy,
            "the accuracy page",
            &format!("At Mach 1.077 it is {} high", unsigned(at(0, "cmq_error"))),
        );
        // The ranges the sentences quote, each over the rows it names.
        let range = |key: &str, from: usize| {
            let values = rows[from..].iter().map(|row| f(&row[key]).abs());
            let (low, high) = values.fold((f64::INFINITY, 0.0_f64), |(low, high), x| {
                (low.min(x), high.max(x))
            });
            (unsigned(low), unsigned(high))
        };
        let (low, high) = range("cd0_error", 2);
        has(
            &aero,
            "the aero page",
            &format!(
                "zero-lift drag reads {low} to {high} below the measured one from Mach 1.83 to 3.15"
            ),
        );
        let (low, high) = range("cmq_error", 2);
        has(
            &aero,
            "the aero page",
            &format!("its pitch damping {low} to {high} below it."),
        );
        let (low, high) = range("cd0_error_as_drawn", 0);
        has(
            &aero,
            "the aero page",
            &format!("as drawn HPR Sim's drag reads {low} to {high} above DREV's"),
        );
        let reynolds = rows.iter().map(|row| f(&row["reynolds_per_m"]) / 1e6);
        let (low, high) = reynolds.fold((f64::INFINITY, 0.0_f64), |(low, high), x| {
            (low.min(x), high.max(x))
        });
        has(
            &aero,
            "the aero page",
            &format!("Reynolds numbers, {low:.1} to {high:.1} million per meter."),
        );
        // How far the two-dimensional trailing-edge band lies above HPR Sim's rule, and how many
        // of the fit's probable errors the damping misses by where the shock stands attached.
        let ratios = rows.iter().flat_map(|row| {
            let rule = f(&row["hpr_fins_trailing_edge"]);
            [
                f(&row["wedge_trailing_edge_low"]) / rule,
                f(&row["wedge_trailing_edge_high"]) / rule,
            ]
        });
        let (low, high) = ratios.fold((f64::INFINITY, 0.0_f64), |(low, high), x| {
            (low.min(x), high.max(x))
        });
        has(
            &aero,
            "the aero page",
            &format!("It lies {low:.1} to {high:.1} times above HPR Sim's own trailing-edge rule"),
        );
        let misses = rows[1..]
            .iter()
            .map(|row| f(&row["cmq_error"]).abs() / f(&row["reference_cmq_probable_error"]));
        let (low, high) = misses.fold((f64::INFINITY, 0.0_f64), |(low, high), x| {
            (low.min(x), high.max(x))
        });
        has(
            &aero,
            "the aero page",
            &format!("by {low:.1} to {high:.1} times the fit's probable error"),
        );
    }
}
