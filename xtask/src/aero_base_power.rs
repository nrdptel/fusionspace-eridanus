//! The base drag below Mach 0.8, power off and under power, against the Hermes A-3A drag model's
//! free flight (M1.14i2, ADR-359), written by `cargo xtask aero` to
//! `validation/fixtures/aero/base-drag-under-power.json`.
//!
//! - **The reference** (Jackson 1955, NACA RM SL55F15): rocket-propelled models of the Hermes
//!   A-3A, flown at Wallops with a burning sustainer motor, whose base drag the report gives from
//!   a pressure orifice on the base annulus, power on and power off (Fig. 20), with the error of
//!   the faired curves "believed to be less than ±0.005". The curves below Mach 0.8 are read off
//!   the figure by pixels (`validation/oracles/naca/hermes_base_drag.py`) into
//!   `validation/fixtures/aero/hermes-a3a-base-drag.json`, with the model's geometry. A US
//!   government work: the numbers are committed.
//! - **HPR Sim's base drag** is its buildup's base term ([`hpr_aero::Drag::base`]) for a body of
//!   revolution the model's diameter, ending in the model's base: coasting for power off, and
//!   thrusting for power on. In a flight HPR Sim takes the burning motor case's cross-section
//!   off the base, down to zero (`hpr_sim`'s dynamics, from the mount's motor diameter). Here
//!   the nominal 5 in case is wider than the 4.8 in base, so that rule, as HPR Sim flies it,
//!   leaves no base drag under power. On this model the case sits forward in the body and a
//!   blast tube carries the nozzle to the base, so what the base holds is the nozzle exit. The
//!   same rule given the nozzle exit's area leaves the annulus the report measured on: that
//!   tests the coefficient on the annulus, not the area HPR Sim flies. Beside both, the whole
//!   base under power, OpenRocket 24.12's rule
//!   ([`hpr_aero::AeroModel::with_full_base_drag_under_power`] flies it), which is the power-off
//!   value. All are on the model's maximum cross-section, as the report's coefficients are.
//!   Below Mach 0.8 the base term doesn't depend on the body ahead of the base (the boattail's
//!   factor on it is 1 there), so the body is drawn as the report's nose, a cylinder, and a
//!   straight cone to the base.
//!
//! No target is set: each difference is reported as it is, beside the report's stated error
//! and the reading's own.

use std::fs;
use std::path::Path;

use hpr_aero::{AeroModel, DragConditions, Flow};
use hpr_design::{
    BodyTube, Component, Finish, NoseCone, NoseShape, Overrides, Part, ReferenceDiameter, Rocket,
    Stage, Transition, Wall, materials,
};
use serde_json::{Value, json};

pub const FIXTURE: &str = "validation/fixtures/aero/base-drag-under-power.json";
pub const REFERENCE: &str = "validation/fixtures/aero/hermes-a3a-base-drag.json";
const IN: f64 = 0.0254;
/// Hoerner's constant in his subsonic base drag of a body of revolution, `C_DB = 0.029 /
/// sqrt(C_DfB)` on the base's area (*Fluid-Dynamic Drag*, 1965, p. 3-19, eq. 34), which on the
/// largest cross-section is `0.029 (d_B/d)^3 / sqrt(C_Df)` (p. 3-20, eq. 35).
const HOERNER_K: f64 = 0.029;

/// The fixture, from the reference's geometry and rows.
pub fn generate(root: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(root.join(REFERENCE)).map_err(|e| format!("{REFERENCE}: {e}"))?;
    let reference: Value = serde_json::from_str(&text).map_err(|e| format!("{REFERENCE}: {e}"))?;
    let geometry = &reference["geometry"];
    let fail = |what: &str| format!("{REFERENCE}: {what}");
    let num = |v: &Value, key: &str| v[key].as_f64().ok_or_else(|| fail(key));
    let rocket = model(geometry)?;
    let layout = rocket.layout().map_err(|e| fail(&e.to_string()))?;
    let model = AeroModel::new(&layout).map_err(|e| fail(&e.to_string()))?;
    let circle = |key: &str| -> Result<f64, String> {
        Ok(0.25 * std::f64::consts::PI * (num(geometry, key)? * IN).powi(2))
    };
    let exit_m2 = circle("nozzle_exit_diameter_in")?;
    let base_over_max = num(geometry, "base_diameter_in")? / num(geometry, "diameter_in")?;
    let case_m2 = circle("motor_case_diameter_in")?;
    let stated = num(&reference["accuracy"], "base_drag_coefficient")?;
    let reading = num(&reference["digitization"], "reading_uncertainty")?;
    let reynolds_per_m = num(&reference, "reynolds_per_m")?;
    let mut rows = Vec::new();
    for row in reference["rows"]
        .as_array()
        .ok_or_else(|| fail("no rows"))?
    {
        let mach = num(row, "mach")?;
        let flow = Flow::new(mach, 0.0, 0.0);
        let base = |conditions: DragConditions| -> Result<f64, String> {
            Ok(model
                .buildup_drag(&flow, &conditions)
                .map_err(|e| fail(&e.to_string()))?
                .base)
        };
        let coasting = model
            .buildup_drag(&flow, &DragConditions::coasting(reynolds_per_m))
            .map_err(|e| fail(&e.to_string()))?;
        let off = coasting.base;
        // Hoerner's subsonic base drag on the largest cross-section (1965, p. 3-20, eq. 35):
        // 0.029 (d_B/d)^3 / sqrt(C_Df), C_Df the forebody's drag on that area, here HPR Sim's
        // own skin friction coasting.
        let hoerner = HOERNER_K * base_over_max.powi(3) / coasting.friction.sqrt();
        let on = base(DragConditions::thrusting(reynolds_per_m, exit_m2))?;
        let on_case = base(DragConditions::thrusting(reynolds_per_m, case_m2))?;
        let mut out = json!({ "mach": mach, "hpr_friction": coasting.friction });
        // HPR Sim's own power off and power on, and the whole base under power, each against
        // its measured curve.
        for (name, hpr, key) in [
            ("hpr_power_off", off, "power_off"),
            ("hpr_power_on", on, "power_on"),
            ("hpr_power_on_case", on_case, "power_on"),
            ("whole_base_under_power", off, "power_on"),
            ("hoerner_power_off", hoerner, "power_off"),
        ] {
            out[name] = json!(hpr);
            let Some(measured) = row[key].as_f64() else {
                continue;
            };
            let difference = hpr - measured;
            out[format!("measured_{key}")] = json!(measured);
            out[format!("{name}_difference")] = json!(difference);
            out[format!("{name}_within_stated")] = json!(difference.abs() <= stated);
            out[format!("{name}_within_stated_and_reading")] =
                json!(difference.abs() <= stated + reading);
        }
        rows.push(out);
    }
    Ok(json!({
        "generator": "cargo xtask aero",
        "reference": REFERENCE,
        "note": "HPR Sim's base drag coefficient on the model's maximum cross-section, its \
                 buildup's base term, against the Hermes A-3A drag model A(1)'s measured base \
                 drag (NACA RM SL55F15 Fig. 20) below Mach 0.8. hpr_power_off: coasting. \
                 hpr_power_on: thrusting, HPR Sim's rule with the nozzle exit's area, what the \
                 base holds of the burning motor, off the base: the annulus the report measured \
                 on. hpr_power_on_case: HPR Sim's rule as it flies, the motor case's area off \
                 the base, down to zero: the nominal 5 in case is wider than the 4.8 in base, \
                 so nothing is left. whole_base_under_power: nothing off, OpenRocket 24.12's \
                 rule, the power-off value. hoerner_power_off: Hoerner's subsonic base drag \
                 (Fluid-Dynamic Drag, 1965, p. 3-20, eq. 35), 0.029 (d_B/d)^3 / sqrt(C_Df), \
                 with C_Df HPR Sim's own skin friction coasting (hpr_friction), on the same \
                 area. Each _difference is the rule's \
                 less the measured; _within_stated is |difference| no more than the report's \
                 stated error of the faired curves, and _within_stated_and_reading adds the \
                 reading's own uncertainty.",
        "stated_error": stated,
        "reading_uncertainty": reading,
        "nozzle_exit_area_m2": exit_m2,
        "motor_case_area_m2": case_m2,
        "reference_area_m2": model.reference_area_m2(),
        "rows": rows,
    }))
}

/// The drag model as a body of revolution: the report's L-V Haack nose, a cylinder of the
/// model's diameter where its circular arc is, and a straight cone down to the base. Aluminum, polished; below Mach 0.8 only the
/// base's area and the nozzle exit's enter the base term.
fn model(geometry: &Value) -> Result<Rocket, String> {
    let fail = |what: &str| format!("{REFERENCE}: {what}");
    let num = |v: &Value, key: &str| v[key].as_f64().ok_or_else(|| fail(key));
    let radius = 0.5 * num(geometry, "diameter_in")? * IN;
    let base_radius = 0.5 * num(geometry, "base_diameter_in")? * IN;
    let nose_length = num(geometry, "nose_length_in")? * IN;
    let cylinder_length = num(geometry, "cylinder_length_in")? * IN;
    let afterbody_length = num(geometry, "afterbody_length_in")? * IN;
    let material = materials::find("aluminum_6061")
        .map(materials::BuiltinMaterial::material)
        .ok_or_else(|| fail("no built-in material `aluminum_6061`"))?;
    let component = |id: &str, part: Part| Component {
        id: id.to_owned(),
        name: String::new(),
        part,
        position: None,
        auto: Vec::new(),
        motor_mount: None,
        finish: Some(Finish::Polished {}),
        overrides: Overrides::default(),
        overrides_include_children: false,
        drag_override: None,
        children: Vec::new(),
    };
    Ok(Rocket {
        name: "Hermes A-3A drag model A(1), its body of revolution".to_owned(),
        stages: vec![Stage {
            id: "model".to_owned(),
            name: String::new(),
            components: vec![
                component(
                    "nose",
                    Part::NoseCone(NoseCone {
                        shape: NoseShape::Haack {
                            parameter: 1.0 / 3.0,
                        },
                        length_m: nose_length,
                        base_radius_m: radius,
                        wall: Wall::Filled {},
                        shoulder: None,
                        material: material.clone(),
                    }),
                ),
                component(
                    "body",
                    Part::BodyTube(BodyTube {
                        length_m: cylinder_length,
                        outer_radius_m: radius,
                        thickness_m: radius,
                        material: material.clone(),
                    }),
                ),
                component(
                    "afterbody",
                    Part::Transition(Transition {
                        shape: NoseShape::Conical {},
                        clipped: false,
                        length_m: afterbody_length,
                        fore_radius_m: radius,
                        aft_radius_m: base_radius,
                        wall: Wall::Filled {},
                        fore_shoulder: None,
                        aft_shoulder: None,
                        material,
                    }),
                ),
            ],
            overrides: Overrides::default(),
            override_rule: Default::default(),
            added_mass: None,
            drag_override: None,
            parallel: None,
        }],
        reference_diameter: ReferenceDiameter::Maximum {},
        configurations: Vec::new(),
    })
}

#[cfg(test)]
mod tests;
