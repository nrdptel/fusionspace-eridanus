//! The drag guide (M0.7a5): how a rocket's drag rises through the speed of sound, what its base
//! and its finish add, and which way HPR Sim's drag can be wrong.
//!
//! Its numbers come from two files. [`REPORT`] holds the guide rocket's zero-lift drag as HPR
//! Sim's aerodynamics give it, through Mach number in standard sea-level air, term by term, and
//! at one low speed under four finishes; `cargo xtask guides` writes it, and `--check` fails a
//! committed copy the design no longer gives. [`TUNNEL`] is the committed comparison of HPR Sim's
//! forebody drag with NASA's wind-tunnel tests of the Arcas Robin (`cargo xtask aero`, ADR-027,
//! ADR-028), which the guide's figure draws, each speed where HPR Sim's drag reads high, the side
//! that flatters an apogee ceiling, marked.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_aero::{AeroModel, Drag, DragConditions, Flow};
use hpr_atmos::{AirState, Ussa76};
use hpr_design::{Finish, Layout, Part, PlacedComponent};
use serde_json::{Value, json};

use super::stability::mach;
use super::{
    CANVAS, CAUTION, INK, INK_MUTED, PLOT_BOTTOM, PLOT_LEFT, PLOT_RIGHT, PLOT_TOP, Quote, RULE,
    Reports, WIDTH, escaped, fixed, percent, signed_percent,
};
pub(crate) use crate::aero_drag::FIXTURE as TUNNEL;

/// The guide rocket's drag, as HPR Sim gives it.
pub(crate) const REPORT: &str = "validation/reports/guide-rocket-drag.json";
/// A drag coefficient the page calls next to nothing: it rounds to zero at the page's two places.
const NEXT_TO_NOTHING: f64 = 0.005;
/// The guides' own rocket.
const DESIGN: &str = "validation/fixtures/ork/guides/level-1.ork";
/// The Mach numbers of the report's sweep.
const MACHS: [f64; 13] = [
    0.1, 0.3, 0.5, 0.8, 0.85, 0.9, 0.95, 1.0, 1.05, 1.1, 1.2, 1.5, 2.0,
];
/// The fine sweep that finds the drag's peak: Mach 0.5 to 2 in hundredths.
const PEAK_FROM: u32 = 50;
const PEAK_TO: u32 = 200;
/// The Mach number of the finish comparison.
const FINISH_MACH: f64 = 0.3;
/// The finishes compared, besides the design's own, each with the report's name for it.
const FINISHES: [(&str, Finish); 3] = [
    ("polished", Finish::Polished {}),
    ("aircraft paint", Finish::MassProductionPaint {}),
    ("badly sprayed paint", Finish::PoorPaint {}),
];
/// The tunnel comparison the guide draws: the short model, the Arcas Robin itself.
const TUNNEL_MODEL: &str = "arcas-robin-short";
/// The tunnel's other model, the Arcas Robin with a longer body.
const TUNNEL_LONG: &str = "arcas-robin-long";

/// The guide rocket's layout, from its `.ork`.
fn layout(root: &Path) -> Result<Layout, String> {
    let bytes = fs::read(root.join(DESIGN)).map_err(|error| format!("{DESIGN}: {error}"))?;
    let file = hpr_io::ork::read(&bytes).map_err(|error| format!("{DESIGN}: {error}"))?;
    let design = hpr_io::ork::design(&file.value).value;
    design
        .rocket
        .layout()
        .map_err(|error| format!("{DESIGN}: {error}"))
}

/// Whether the air flows over `component`, so its finish counts: the airframe's body components and
/// the parts outside it. A part inside ignores its finish.
fn outside(component: &PlacedComponent) -> bool {
    component.parent.is_none()
        || matches!(
            component.part,
            Part::FinSet(_)
                | Part::TubeFinSet(_)
                | Part::LaunchLug(_)
                | Part::RailButton(_)
                | Part::PodSet(_)
        )
}

/// The aerodynamic model of `layout`.
fn aero(layout: &Layout) -> Result<AeroModel, String> {
    AeroModel::new(layout).map_err(|error| format!("{DESIGN}: {error}"))
}

/// The 1976 standard atmosphere's air at sea level.
fn sea_level() -> Result<AirState, String> {
    Ussa76::standard()
        .sample(0.0)
        .map(|sample| sample.air)
        .map_err(|error| error.to_string())
}

/// The drag at Mach `mach`, coasting, in standard sea-level air: the Reynolds number per meter
/// is the airspeed over the air's kinematic viscosity.
fn drag(model: &AeroModel, mach: f64) -> Result<(Drag, f64), String> {
    let air = sea_level()?;
    let reynolds_per_m = mach * air.speed_of_sound_m_s / air.kinematic_viscosity_m2_s();
    let drag = model
        .drag(
            &Flow::axial(mach),
            &DragConditions::coasting(reynolds_per_m),
        )
        .map_err(|error| format!("{DESIGN} at Mach {mach}: {error}"))?;
    if drag.table.is_some() || drag.stated != 0.0 {
        return Err(format!(
            "{DESIGN} states its drag; the guide explains HPR Sim's own"
        ));
    }
    Ok((drag, reynolds_per_m))
}

/// The pressure drag of the nose cone and of the fins, from the buildup by component.
fn nose_and_fins(
    layout: &Layout,
    model: &AeroModel,
    mach: f64,
    reynolds_per_m: f64,
) -> Result<(f64, f64), String> {
    let noses: Vec<&str> = layout
        .components
        .iter()
        .filter(|component| matches!(component.part, Part::NoseCone(_)))
        .map(|component| component.id.as_str())
        .collect();
    if noses.len() != 1 {
        return Err(format!(
            "{DESIGN} has {} nose cones; the guide reads one",
            noses.len()
        ));
    }
    let fins: Vec<&str> = model.fin_sets().iter().map(|set| set.id.as_str()).collect();
    let parts = model
        .buildup_components(
            &Flow::axial(mach),
            &DragConditions::coasting(reynolds_per_m),
        )
        .map_err(|error| format!("{DESIGN} at Mach {mach}: {error}"))?;
    let of = |ids: &[&str]| -> f64 {
        parts
            .iter()
            .filter(|part| ids.contains(&part.id.as_str()))
            .map(|part| part.drag.pressure)
            .sum()
    };
    Ok((of(&noses), of(&fins)))
}

/// The report: the drag through [`MACHS`], term by term, its peak, and the drag at
/// [`FINISH_MACH`] under the design's finish and [`FINISHES`].
pub(crate) fn generate(root: &Path) -> Result<Value, String> {
    let air = sea_level()?;
    let layout = layout(root)?;
    let model = aero(&layout)?;
    let mut by_mach = Vec::new();
    for mach in MACHS {
        let (drag, reynolds_per_m) = drag(&model, mach)?;
        let (nose, fins) = nose_and_fins(&layout, &model, mach, reynolds_per_m)?;
        by_mach.push(json!({
            "mach": mach,
            "reynolds_per_m": reynolds_per_m,
            "cd": drag.zero_lift_coefficient,
            "friction": drag.friction,
            "pressure": drag.pressure,
            "base": drag.base,
            "parasitic": drag.parasitic,
            "nose_pressure": nose,
            "fins_pressure": fins,
        }));
    }
    let mut peak = (0.0, f64::NEG_INFINITY);
    for hundredths in PEAK_FROM..=PEAK_TO {
        let mach = f64::from(hundredths) / 100.0;
        let cd = drag(&model, mach)?.0.zero_lift_coefficient;
        if cd > peak.1 {
            peak = (mach, cd);
        }
    }
    let roughness = |finish: &Finish| {
        finish
            .roughness_m()
            .map_err(|error| format!("{DESIGN}: {error}"))
    };
    let own = layout
        .components
        .iter()
        .filter(|component| outside(component))
        .map(|component| component.finish)
        .collect::<Vec<_>>();
    let own_finish = *own.first().ok_or(format!("{DESIGN} has no components"))?;
    if own.iter().any(|finish| *finish != own_finish) {
        return Err(format!(
            "{DESIGN}'s parts have different finishes; the guide reads one"
        ));
    }
    let (at_finish_mach, _) = drag(&model, FINISH_MACH)?;
    let mut by_finish = vec![(
        roughness(&own_finish)?,
        json!({
            "finish": "the design's",
            "roughness_m": roughness(&own_finish)?,
            "cd": at_finish_mach.zero_lift_coefficient,
            "friction": at_finish_mach.friction,
        }),
    )];
    for (name, finish) in FINISHES {
        let mut finished = layout.clone();
        for component in &mut finished.components {
            if outside(component) {
                component.finish = finish;
            }
        }
        let (drag, _) = drag(&aero(&finished)?, FINISH_MACH)?;
        by_finish.push((
            roughness(&finish)?,
            json!({
                "finish": name,
                "roughness_m": roughness(&finish)?,
                "cd": drag.zero_lift_coefficient,
                "friction": drag.friction,
            }),
        ));
    }
    by_finish.sort_by(|a, b| a.0.total_cmp(&b.0));
    let by_finish: Vec<Value> = by_finish.into_iter().map(|(_, row)| row).collect();
    Ok(json!({
        "generator": "cargo xtask guides",
        "design": DESIGN,
        "note": "HPR Sim's zero-lift drag coefficient for the guides' rocket, coasting, on its \
                 reference area, from its aerodynamics alone, in the 1976 standard atmosphere's \
                 sea-level air (air). by_mach: cd and its terms (friction, pressure, base, parasitic), \
                 with the nose's and the fins' pressure drag apart. peak: the largest cd from Mach \
                 0.5 to 2 in hundredths. by_finish: at finish_mach, cd and its friction with every \
                 part given each finish, smoothest first. A model's output, not a measurement.",
        "air": {
            "density_kg_m3": air.density_kg_m3,
            "speed_of_sound_m_s": air.speed_of_sound_m_s,
        },
        "reference_diameter_m": model.reference_diameter_m(),
        "reference_area_m2": model.reference_area_m2(),
        "length_m": model.length_m(),
        "by_mach": by_mach,
        "peak": { "mach": peak.0, "cd": peak.1 },
        "finish_mach": FINISH_MACH,
        "by_finish": by_finish,
    }))
}

/// The report as it is committed.
pub(crate) fn written(report: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|text| text + "\n")
        .map_err(|error| format!("{REPORT}: {error}"))
}

/// A problem, if the committed report is not what the design gives now.
pub(crate) fn stale(root: &Path) -> Result<Option<String>, String> {
    let now = generate(root)?;
    let committed = fs::read_to_string(root.join(REPORT))
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok());
    Ok(match committed {
        Some(committed) if super::stability::same(&committed, &now) => None,
        _ => Some(format!(
            "{REPORT} is not what {DESIGN} gives now; run `cargo xtask guides` and commit it"
        )),
    })
}

/// The number `key` of the guide rocket's row `at` through Mach number.
fn rocket(r: &Reports, at: usize, key: &str) -> Result<f64, String> {
    r.number(REPORT, &format!("/by_mach/{at}/{key}"))
}

/// The Mach number of the guide rocket's row `at`, as the guide writes it.
fn rocket_mach(r: &Reports, at: usize) -> Result<String, String> {
    Ok(mach(rocket(r, at, "mach")?))
}

/// A drag coefficient to two places.
fn coefficient(value: f64) -> String {
    fixed(value, 2)
}

/// A share or a rise, `value` a fraction, as a whole percent.
fn share(value: f64) -> String {
    percent(value * 100.0, 0)
}

/// An error, `value` a fraction, as a signed percent to one place.
fn error(value: f64) -> String {
    signed_percent(value * 100.0, 1)
}

/// The least of a set of errors, a fraction, rounded down to a tenth of a percent, so the page
/// never states a smaller spread than the comparison holds.
fn least(value: f64) -> String {
    signed_percent((value * 1000.0).floor() / 10.0, 1)
}

/// The largest of a set of errors, rounded up to a tenth of a percent, for the same reason.
fn most(value: f64) -> String {
    signed_percent((value * 1000.0).ceil() / 10.0, 1)
}

/// A roughness height in micrometers, to a tenth when it has one.
fn micrometers(value_m: f64) -> String {
    let um = value_m * 1e6;
    let text = fixed(um, 1);
    match text.strip_suffix(".0") {
        Some(whole) => whole.to_owned(),
        None => text,
    }
}

/// The guide rocket's drag at the finish comparison's speed under the finish `at`, its name
/// checked: its roughness and its drag coefficient.
fn finish(r: &Reports, at: usize, name: &str) -> Result<(f64, f64), String> {
    let row = format!("/by_finish/{at}");
    let named = r.text(REPORT, &format!("{row}/finish"))?;
    if named != name {
        return Err(format!("{REPORT}'s finish {at} is {named}, not {name}"));
    }
    Ok((
        r.number(REPORT, &format!("{row}/roughness_m"))?,
        r.number(REPORT, &format!("{row}/cd"))?,
    ))
}

/// One reading of the tunnel comparison.
struct TunnelRow {
    mach: f64,
    measured: f64,
    uncertainty: f64,
    hpr: f64,
    error: f64,
}

impl Reports {
    /// The pointer of the drag comparison `id` in [`TUNNEL`]. Finding it reads the references'
    /// ids, a lookup, not a value a quote shows, so they aren't recorded.
    fn drag_model(&self, id: &str) -> Result<String, String> {
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

    /// The rows of the comparison `id` with the fins on or off, each recorded as read, sorted
    /// into rising Mach order; the list is recorded too, since the quotes count it. Which rows have their
    /// fins on is a lookup, not recorded.
    fn drag_rows(&self, id: &str, fins_on: bool) -> Result<Vec<TunnelRow>, String> {
        let model = self.drag_model(id)?;
        let all = self
            .get(TUNNEL, &format!("{model}/rows"))?
            .as_array()
            .ok_or_else(|| format!("{TUNNEL}'s {model} has no rows"))?;
        let wanted = if fins_on { "on" } else { "off" };
        let mut rows = all
            .iter()
            .enumerate()
            .filter(|(_, row)| row["fins"].as_str() == Some(wanted))
            .map(|(at, _)| {
                let number = |key: &str| self.number(TUNNEL, &format!("{model}/rows/{at}/{key}"));
                Ok(TunnelRow {
                    mach: number("mach")?,
                    measured: number("reference_forebody_c_a")?,
                    uncertainty: number("reference_uncertainty")?,
                    hpr: number("hpr_forebody_c_d")?,
                    error: number("error")?,
                })
            })
            .collect::<Result<Vec<TunnelRow>, String>>()?;
        if rows.is_empty() {
            return Err(format!(
                "{TUNNEL}'s {id} has no rows with its fins {wanted}"
            ));
        }
        // The quotes read the first and last rows as the slowest and fastest.
        rows.sort_by(|a, b| a.mach.total_cmp(&b.mach));
        if rows.windows(2).any(|pair| pair[0].mach == pair[1].mach) {
            return Err(format!("{TUNNEL}'s {id} has two rows at one Mach number"));
        }
        Ok(rows)
    }

    /// The short model's rows with its fins on, which the figure draws.
    fn drawn_rows(&self) -> Result<Vec<TunnelRow>, String> {
        self.drag_rows(TUNNEL_MODEL, true)
    }
}

/// The row with the least error, and the one with the most.
fn extremes(rows: &[TunnelRow]) -> Result<(&TunnelRow, &TunnelRow), String> {
    let low = rows
        .iter()
        .min_by(|a, b| a.error.total_cmp(&b.error))
        .ok_or("no rows")?;
    let high = rows
        .iter()
        .max_by(|a, b| a.error.total_cmp(&b.error))
        .ok_or("no rows")?;
    Ok((low, high))
}

/// The drag guide's quotes.
pub(super) fn quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "On the model the figure draws, HPR Sim's forebody drag read high at {}, \
                       by {} to {}",
            holes: |r| {
                let rows = r.drawn_rows()?;
                let (low, high) = extremes(&rows)?;
                let above = rows.iter().filter(|row| row.error > 0.0).count();
                let at = if above == rows.len() {
                    "every speed".to_owned()
                } else {
                    format!("{above} of its {} speeds", rows.len())
                };
                Ok(vec![at, least(low.error), most(high.error)])
            },
        },
        Quote {
            template: "HPR Sim's apogee averaged {} the logged one on the {} flights",
            holes: |r| {
                Ok(vec![
                    super::above(super::set_number(r, "hpr", "mean_percent")?),
                    super::flights(r)?.to_string(),
                ])
            },
        },
        Quote {
            template: "At Mach {} in sea-level air its drag coefficient is {}: skin friction makes \
                       {} of it, the base {} and the fins' square edges {}",
            holes: |r| {
                let at = |key: &str| rocket(r, 1, key);
                let (cd, fins) = (at("cd")?, at("fins_pressure")?);
                // The sentence names every part of the drag: no lugs, and a nose that adds
                // nothing at two places (the page says it adds next to nothing).
                let nose = at("nose_pressure")?;
                if nose >= NEXT_TO_NOTHING
                    || at("parasitic")? != 0.0
                    || (at("pressure")? - fins - nose).abs() > 1e-12
                {
                    return Err("the guide rocket's drag at low speed has other parts".to_owned());
                }
                Ok(vec![
                    rocket_mach(r, 1)?,
                    coefficient(cd),
                    share(at("friction")? / cd),
                    share(at("base")? / cd),
                    share(fins / cd),
                ])
            },
        },
        Quote {
            template: "At Mach {} in sea-level air the guides' rocket flies at {}, so the dynamic \
                       pressure is {} kPa",
            holes: |r| {
                let sound = r.number(REPORT, "/air/speed_of_sound_m_s")?;
                let density = r.number(REPORT, "/air/density_kg_m3")?;
                let mach_value = rocket(r, 2, "mach")?;
                let speed = mach_value * sound;
                Ok(vec![
                    mach(mach_value),
                    super::flutter::speed(speed),
                    fixed(0.5 * density * speed * speed / 1000.0, 1),
                ])
            },
        },
        Quote {
            template: "the body's cross-section, {} across, and its drag coefficient there is {}, \
                       so the air holds it back with {} N, the weight of {} kg",
            holes: |r| {
                let sound = r.number(REPORT, "/air/speed_of_sound_m_s")?;
                let density = r.number(REPORT, "/air/density_kg_m3")?;
                let area = r.number(REPORT, "/reference_area_m2")?;
                let speed = rocket(r, 2, "mach")? * sound;
                let cd = rocket(r, 2, "cd")?;
                let drag_n = 0.5 * density * speed * speed * cd * area;
                Ok(vec![
                    super::flutter::millimeters(r.number(REPORT, "/reference_diameter_m")?, 0),
                    coefficient(cd),
                    fixed(drag_n, 0),
                    fixed(drag_n / hpr_core::gravity::STANDARD_GRAVITY_MPS2, 1),
                ])
            },
        },
        Quote {
            template: "its drag coefficient climbs to {} by Mach {} and {} at Mach {}, and peaks \
                       at {} at Mach {}, {} above its value at Mach {}",
            holes: |r| {
                let low = rocket(r, 1, "cd")?;
                let peak = r.number(REPORT, "/peak/cd")?;
                Ok(vec![
                    coefficient(rocket(r, 3, "cd")?),
                    rocket_mach(r, 3)?,
                    coefficient(rocket(r, 7, "cd")?),
                    rocket_mach(r, 7)?,
                    coefficient(peak),
                    mach(r.number(REPORT, "/peak/mach")?),
                    share(peak / low - 1.0),
                    rocket_mach(r, 1)?,
                ])
            },
        },
        Quote {
            template: "By Mach {} it is back down to {}",
            holes: |r| Ok(vec![rocket_mach(r, 12)?, coefficient(rocket(r, 12, "cd")?)]),
        },
        Quote {
            template: "The nose's pressure drag, next to nothing at Mach {}, reaches {} at Mach {}",
            holes: |r| {
                if rocket(r, 2, "nose_pressure")? >= NEXT_TO_NOTHING {
                    return Err("the guide rocket's nose has pressure drag at Mach 0.5".to_owned());
                }
                Ok(vec![
                    rocket_mach(r, 2)?,
                    coefficient(rocket(r, 9, "nose_pressure")?),
                    rocket_mach(r, 9)?,
                ])
            },
        },
        Quote {
            template: "The base's grows from {} at Mach {} to {} at Mach {}, then falls",
            holes: |r| {
                let at_one = rocket(r, 7, "base")?;
                if rocket(r, 8, "base")? >= at_one {
                    return Err("the guide rocket's base drag doesn't fall past Mach 1".to_owned());
                }
                Ok(vec![
                    coefficient(rocket(r, 2, "base")?),
                    rocket_mach(r, 2)?,
                    coefficient(at_one),
                    rocket_mach(r, 7)?,
                ])
            },
        },
        Quote {
            template: "at Mach {} it meets {} times the drag it meets at Mach {}",
            holes: |r| {
                let (fast, slow) = (rocket(r, 7, "cd")?, rocket(r, 2, "cd")?);
                Ok(vec![
                    rocket_mach(r, 7)?,
                    fixed(
                        (rocket(r, 7, "mach")? / rocket(r, 2, "mach")?).powi(2) * fast / slow,
                        1,
                    ),
                    rocket_mach(r, 2)?,
                ])
            },
        },
        Quote {
            template: "On the faster of its motors the guides' rocket reaches Mach {}, where the \
                       rise has begun: by Mach {} its drag coefficient is {}, {} above its value \
                       at Mach {}",
            holes: |r| {
                let count = r
                    .get(super::flutter::REPORT, "/flights")?
                    .as_array()
                    .map(Vec::len)
                    .ok_or("no flights")?;
                let mut top = f64::NEG_INFINITY;
                for at in 0..count {
                    top = top
                        .max(r.number(super::flutter::REPORT, &format!("/flights/{at}/max_mach"))?);
                }
                // The nose's drag starts to rise at Mach 0.8.
                if top < 0.8 {
                    return Err("the guide rocket stays below the drag rise".to_owned());
                }
                let cd = rocket(r, 4, "cd")?;
                Ok(vec![
                    mach(top),
                    rocket_mach(r, 4)?,
                    coefficient(cd),
                    share(cd / rocket(r, 1, "cd")? - 1.0),
                    rocket_mach(r, 1)?,
                ])
            },
        },
        Quote {
            template: "drag coefficient at Mach {} is: polished, {} µm, {}; aircraft paint, {} µm, \
                       HPR Sim's default, {}; the design's own {} µm, {}; and badly sprayed paint, \
                       {} µm, {}",
            holes: |r| {
                let names = [
                    "polished",
                    "aircraft paint",
                    "the design's",
                    "badly sprayed paint",
                ];
                let mut holes = vec![mach(r.number(REPORT, "/finish_mach")?)];
                for (at, name) in names.iter().enumerate() {
                    let (roughness, cd) = finish(r, at, name)?;
                    holes.push(micrometers(roughness));
                    holes.push(coefficient(cd));
                }
                Ok(holes)
            },
        },
        Quote {
            template: "at Mach {} the drag coefficient is {} higher with the badly sprayed paint \
                       than with the polished finish",
            holes: |r| {
                let (_, smooth) = finish(r, 0, "polished")?;
                let (_, rough) = finish(r, 3, "badly sprayed paint")?;
                Ok(vec![
                    mach(r.number(REPORT, "/finish_mach")?),
                    share(rough / smooth - 1.0),
                ])
            },
        },
        Quote {
            template: "from Mach {} to {}",
            holes: |r| {
                let rows = r.drawn_rows()?;
                let first = rows.first().ok_or("no rows")?;
                let last = rows.last().ok_or("no rows")?;
                Ok(vec![mach(first.mach), mach(last.mach)])
            },
        },
        Quote {
            template: "With its fins on, the model's measured forebody drag rose from {} at Mach {} \
                       to {} at Mach {}, then fell to {} by Mach {}",
            holes: |r| {
                let rows = r.drawn_rows()?;
                let first = rows.first().ok_or("no rows")?;
                let last = rows.last().ok_or("no rows")?;
                let peak = rows
                    .iter()
                    .max_by(|a, b| a.measured.total_cmp(&b.measured))
                    .ok_or("no rows")?;
                if peak.mach <= first.mach || peak.mach >= last.mach {
                    return Err("the tunnel's drag peaks at an end of its speeds".to_owned());
                }
                Ok(vec![
                    coefficient(first.measured),
                    mach(first.mach),
                    coefficient(peak.measured),
                    mach(peak.mach),
                    coefficient(last.measured),
                    mach(last.mach),
                ])
            },
        },
        Quote {
            template: "HPR Sim's read {} at Mach {}, {} at Mach {}, the most below the speed of \
                       sound, and {} at Mach {}, the least; then more at every faster speed, to {} \
                       at Mach {}",
            holes: |r| {
                let rows = r.drawn_rows()?;
                let first = rows.first().ok_or("no rows")?;
                let below = rows
                    .iter()
                    .filter(|row| row.mach < 1.0)
                    .max_by(|a, b| a.error.total_cmp(&b.error))
                    .ok_or("no row below Mach 1")?;
                let (low, high) = extremes(&rows)?;
                let after: Vec<&TunnelRow> =
                    rows.iter().filter(|row| row.mach > low.mach).collect();
                if after.windows(2).any(|pair| pair[1].error <= pair[0].error)
                    || after.first().is_some_and(|row| row.error <= low.error)
                {
                    return Err(
                        "HPR Sim's error doesn't grow at every speed past its least".to_owned()
                    );
                }
                Ok(vec![
                    error(first.error),
                    mach(first.mach),
                    error(below.error),
                    mach(below.mach),
                    least(low.error),
                    mach(low.mach),
                    most(high.error),
                    mach(high.mach),
                ])
            },
        },
        Quote {
            template: "With its fins off, HPR Sim's read {} to {} high",
            holes: |r| {
                let rows = r.drag_rows(TUNNEL_MODEL, false)?;
                let (low, high) = extremes(&rows)?;
                if low.error <= 0.0 {
                    return Err("HPR Sim's drag reads low with the fins off".to_owned());
                }
                Ok(vec![least(low.error), most(high.error)])
            },
        },
        Quote {
            template: "read off the reports' plots to within {} to {} either way",
            holes: |r| {
                let rows = r.drawn_rows()?;
                let uncertainties = rows.iter().map(|row| row.uncertainty);
                let low = uncertainties.clone().reduce(f64::min).ok_or("no rows")?;
                let high = uncertainties.reduce(f64::max).ok_or("no rows")?;
                Ok(vec![fixed(low, 3), fixed(high, 3)])
            },
        },
        Quote {
            template: "{} of both models' {} readings are within {}",
            holes: |r| {
                let target = r.number(TUNNEL, "/target_rel")?;
                let mut rows = Vec::new();
                for id in [TUNNEL_MODEL, TUNNEL_LONG] {
                    for fins_on in [true, false] {
                        rows.extend(r.drag_rows(id, fins_on)?);
                    }
                }
                let within = rows.iter().filter(|row| row.error.abs() <= target).count();
                Ok(vec![
                    within.to_string(),
                    rows.len().to_string(),
                    share(target),
                ])
            },
        },
    ]
}

/// The figure's Mach axis, from zero to this, in steps of one.
const MACH_TOP: f64 = 5.0;
/// The drag axis's step.
const CD_STEP: f64 = 0.2;
/// The x of Mach `value`.
fn mach_x(value: f64) -> f64 {
    PLOT_LEFT + value / MACH_TOP * (PLOT_RIGHT - PLOT_LEFT)
}

/// The drag guide's figure: the tunnel's forebody drag on the Arcas Robin with its fins on
/// against HPR Sim's through Mach number, each reading with its uncertainty as a band, the speed
/// of sound ruled, and each speed where HPR Sim's reads high, the side that flatters an apogee
/// ceiling, marked.
pub(super) fn draw_drag_against_mach(reports: &Reports) -> Result<String, String> {
    let rows = reports.drawn_rows()?;
    let high = rows.iter().filter(|row| row.error > 0.0).count();
    let top = (rows
        .iter()
        .map(|row| row.hpr.max(row.measured + row.uncertainty))
        .reduce(f64::max)
        .ok_or("no rows")?
        / CD_STEP)
        .ceil();
    let top = top * CD_STEP;
    if rows
        .iter()
        .any(|row| row.mach < 0.0 || row.mach > MACH_TOP || row.measured - row.uncertainty < 0.0)
    {
        return Err(format!(
            "a row past the figure's axes, Mach 0 to {MACH_TOP} and drag from 0"
        ));
    }
    let y = |cd: f64| PLOT_BOTTOM - cd / top * (PLOT_BOTTOM - PLOT_TOP);
    let height = PLOT_BOTTOM + 150.0;
    let title = "The Arcas Robin's forebody drag by Mach";
    let lines = [
        "NASA's wind tunnel (dots, each with its reading's uncertainty as a band) against",
        "HPR Sim (line): the drag coefficient less the base's share, fins on, by Mach number.",
        "Each speed's gap between the two is drawn from the tunnel's dot to HPR Sim's line.",
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
    // The speed of sound, behind everything.
    let sound = mach_x(1.0);
    let _ = writeln!(
        svg,
        "<rect x=\"{:.1}\" y=\"{PLOT_TOP}\" width=\"2\" height=\"{:.1}\" fill=\"{RULE}\"/>",
        sound - 1.0,
        PLOT_BOTTOM - PLOT_TOP
    );
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" fill=\"{INK_MUTED}\">Mach 1</text>",
        sound + 6.0,
        PLOT_BOTTOM - 10.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">forebody drag coefficient</text>",
        PLOT_TOP - 16.0
    );
    // The drag axis.
    let steps = (top / CD_STEP).round() as usize;
    for step in 0..=steps {
        let tick = step as f64 * CD_STEP;
        let at = y(tick);
        let _ = writeln!(
            svg,
            "<line x1=\"{:.1}\" y1=\"{at:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{at:.1}\" \
             stroke=\"{INK_MUTED}\" stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" \
             text-anchor=\"end\" fill=\"{INK_MUTED}\">{}</text>",
            PLOT_LEFT - 4.0,
            PLOT_LEFT - 8.0,
            at + 4.0,
            fixed(tick, 1)
        );
    }
    // Each speed's gap, HPR Sim's less the tunnel's: caution where HPR Sim's reads high.
    for row in &rows {
        let x = mach_x(row.mach);
        let (color, width) = if row.error > 0.0 {
            (CAUTION, 3.0)
        } else {
            (INK_MUTED, 1.0)
        };
        let _ = writeln!(
            svg,
            "<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{color}\" \
             stroke-width=\"{width}\"/>",
            y(row.measured),
            y(row.hpr)
        );
    }
    // The tunnel's readings, each with its band.
    for row in &rows {
        let x = mach_x(row.mach);
        let (upper, lower) = (
            y(row.measured + row.uncertainty),
            y(row.measured - row.uncertainty),
        );
        let _ = writeln!(
            svg,
            "<rect x=\"{:.1}\" y=\"{upper:.1}\" width=\"10\" height=\"{:.1}\" fill=\"{RULE}\" \
             stroke=\"{INK_MUTED}\" stroke-width=\"1\"/><circle cx=\"{x:.1}\" cy=\"{:.1}\" \
             r=\"3\" fill=\"{INK}\"/>",
            x - 5.0,
            lower - upper,
            y(row.measured)
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
                y(row.hpr)
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
         fill=\"{CAUTION}\">flatters a ceiling: HPR Sim's drag high, its apogee low, at {high} \
         of {} speeds</text>",
        key - 10.0,
        key + 2.0,
        PLOT_LEFT + 10.0,
        rows.len()
    );
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{:.1}\" stroke=\"{INK_MUTED}\" \
         stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">the other side: \
         HPR Sim's drag low, its apogee high</text>",
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
        moved["by_mach"][7]["cd"] = json!(now["by_mach"][7]["cd"].as_f64().unwrap() + 1e-8);
        assert!(!super::super::stability::same(&now, &moved));
        let mut shorter = now.clone();
        shorter["by_finish"].as_array_mut().unwrap().pop();
        assert!(!super::super::stability::same(&now, &shorter));
    }

    /// The figure's caution marks are the speeds where HPR Sim's forebody drag reads above the
    /// tunnel's, a positive `error`: the side `hpr_sim`'s `a_higher_drag_reads_a_lower_apogee`
    /// pins as an apogee, a top speed and a max q that read low.
    #[test]
    fn the_flattering_marks_are_the_speeds_that_read_high() {
        let reports = reports();
        let figure = draw_drag_against_mach(&reports).unwrap();
        let rows = reports.drawn_rows().unwrap();
        let marks = figure
            .lines()
            .filter(|line| {
                line.starts_with("<line") && line.contains(CAUTION) && !line.contains("<text")
            })
            .count();
        let high = rows.iter().filter(|row| row.error > 0.0).count();
        assert!(high > 0);
        assert_eq!(marks, high);
        for row in &rows {
            let x = format!("x1=\"{:.1}\"", mach_x(row.mach));
            let line = figure
                .lines()
                .find(|line| line.starts_with(&format!("<line {x}")))
                .unwrap();
            assert_eq!(line.contains(CAUTION), row.error > 0.0, "{line}");
            assert_eq!(row.error > 0.0, row.hpr > row.measured, "Mach {}", row.mach);
        }
    }

    /// The figure passes the figure checks and draws what the guide's description says.
    #[test]
    fn the_figure_draws_its_description() {
        let figure = draw_drag_against_mach(&reports()).unwrap();
        assert!(crate::figures::check(&figure).is_empty());
        assert!(crate::figures::drawing::check(&figure).is_empty());
        assert!(figure.contains(">Mach 1<"));
        assert!(figure.contains("flatters a ceiling"));
    }

    /// The finishes the guide names are the ones the report compares, and the one it calls HPR
    /// Sim's default, aircraft paint, is.
    #[test]
    fn the_finishes_are_barrowmans() {
        assert_eq!(Finish::default(), Finish::MassProductionPaint {});
        let report = generate(&crate::designs::root().unwrap()).unwrap();
        let names: Vec<&str> = report["by_finish"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["finish"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "polished",
                "aircraft paint",
                "the design's",
                "badly sprayed paint"
            ]
        );
    }

    /// The least and largest errors round outward, so a spread is never stated narrower.
    #[test]
    fn spreads_round_outward() {
        assert_eq!(least(0.0708), "+7.0%");
        assert_eq!(most(1.5401), "+154.1%");
        assert_eq!(most(0.5), "+50.0%");
        assert_eq!(least(-0.0708), "−7.1%");
    }

    /// Roughness heights drop a zero tenth.
    #[test]
    fn roughness_reads_in_micrometers() {
        assert_eq!(micrometers(0.5e-6), "0.5");
        assert_eq!(micrometers(20e-6), "20");
        assert_eq!(micrometers(200e-6), "200");
    }
}
