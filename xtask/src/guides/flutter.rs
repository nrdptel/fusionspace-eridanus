//! The fin-flutter guide (M0.7a4): what Martin's criterion says about the guide rocket's fins,
//! read two ways, and the halved constant that makes a flutter speed read high.
//!
//! Its numbers come from [`REPORT`], which `cargo xtask guides` writes and `--check` fails when
//! the design no longer gives it: the fins' panel by NACA TN 4197 (`hpr_sim::flutter`), their
//! flutter speed at the launch site with the built-in birch plywood's shear modulus, each
//! configuration's flight as `hpr sim --json` prints it, and the flutter speed through a sweep
//! of thicknesses, which the guide's figure draws. The figure marks in the caution color the
//! speed the halved constant of *Peak of Flight* #291 and #411 gives, √2 above HPR Sim's: the
//! flattering side, whose direction `hpr_sim::flutter`'s
//! `the_halved_constant_reads_a_higher_flutter_speed` pins.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use hpr_atmos::Ussa76;
use hpr_design::{Component, FinPlanform, FinSet, Part, materials};
use hpr_sim::FlutterPanel;
use hpr_sim::flutter::FIGURE_3_BAND;
use serde_json::{Value, json};

use super::{
    CANVAS, CAUTION, INK, INK_MUTED, PLOT_BOTTOM, PLOT_LEFT, PLOT_RIGHT, PLOT_TOP, Quote, RULE,
    Reports, WIDTH, escaped, fixed, meters, written as written_value,
};

/// The guide rocket's flutter numbers, as HPR Sim gives them.
pub(crate) const REPORT: &str = "validation/reports/guide-rocket-flutter.json";
/// The guides' own rocket.
const DESIGN: &str = "validation/fixtures/ork/guides/level-1.ork";
/// The built-in material whose shear modulus the guide takes for the fins: the design's birch
/// plywood is OpenRocket's, at OpenRocket's density, which `materials::shear_modulus_of` doesn't
/// match to a built-in material.
const MATERIAL: &str = "birch_plywood";
/// The thicknesses of the figure's sweep, mm: 1 to 8 in steps of a half.
const THICKNESS_STEPS: usize = 15;
const THICKNESS_FIRST_MM: f64 = 1.0;
const THICKNESS_STEP_MM: f64 = 0.5;
/// The least Mach number Martin's unfailed wings flew to: his figure 3's open points are "wings
/// that flew to at least M = 1.3 without known failure" (NACA TN 4197, 1958, p. 19, figure 3's
/// key), so his band says nothing about a fin above it on a slower rocket.
const UNFAILED_MACH: f64 = 1.3;
/// The halved constant, per psi: *Peak of Flight* #291 (Z. Howard, 2011, eq. 1, p. 3) prints
/// the denominator `1.337 A³ P (λ + 1) / (2 (A + 2) (t/c)³)`, and #411 (J. D. Sahr, 2016, eq. 1,
/// p. 2) the same rearranged; Martin's eq. 18 gives `39.3 / 14.696 = 2.674` in its place, as #615
/// (J. K. Bennett, 2023, pp. 2 and 5) corrects. Loft's constant was the same (L32).
const HALVED_PER_PSI: f64 = 1.337;

/// The guide rocket's one fin set, from its `.ork`.
fn fins(root: &Path) -> Result<FinSet, String> {
    fn find(components: &[Component]) -> Vec<&FinSet> {
        components
            .iter()
            .flat_map(|component| match &component.part {
                Part::FinSet(set) => vec![set],
                _ => find(&component.children),
            })
            .collect()
    }
    let bytes = fs::read(root.join(DESIGN)).map_err(|error| format!("{DESIGN}: {error}"))?;
    let file = hpr_io::ork::read(&bytes).map_err(|error| format!("{DESIGN}: {error}"))?;
    let design = hpr_io::ork::design(&file.value).value;
    let sets: Vec<FinSet> = design
        .rocket
        .stages
        .iter()
        .flat_map(|stage| find(&stage.components))
        .cloned()
        .collect();
    match sets.as_slice() {
        [set] => Ok(set.clone()),
        _ => Err(format!(
            "{DESIGN} has {} fin sets; the guide reads one",
            sets.len()
        )),
    }
}

/// `hpr sim --json` on the design's configuration `number`, offline, as the command prints it.
fn fly(root: &Path, number: usize) -> Result<Value, String> {
    // Tests fly the design on several threads of one process, so each call has its own folder.
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let scratch = std::env::temp_dir().join(format!(
        "hpr-guide-flutter-{}-{}",
        std::process::id(),
        CALLS.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::create_dir_all(&scratch);
    let args = [
        "hpr".into(),
        "sim".into(),
        root.join(DESIGN).into_os_string(),
        "--config".into(),
        number.to_string().into(),
        "--json".into(),
    ];
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = hpr_cli::run_offline(args, &mut out, &mut err, &scratch.join("cache"));
    let _ = fs::remove_dir_all(&scratch);
    if exit.code() != 0 {
        return Err(format!(
            "hpr sim {DESIGN} --config {number} failed: {}",
            String::from_utf8_lossy(&err)
        ));
    }
    serde_json::from_slice(&out).map_err(|error| format!("{DESIGN} --config {number}: {error}"))
}

/// The finite number at `pointer` in a flight's JSON.
fn flown(flight: &Value, pointer: &str) -> Result<f64, String> {
    flight
        .pointer(pointer)
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("{DESIGN}'s flight has no {pointer}"))
}

/// The report: the fins' panel and modulus, their flutter numbers at the launch site, each
/// configuration's peak dynamic pressure and the ratio of the flutter speed to the airspeed
/// there, the halved constant's speed, and the flutter speed through a sweep of thicknesses.
pub(crate) fn generate(root: &Path) -> Result<Value, String> {
    let set = fins(root)?;
    let FinPlanform::Trapezoidal {
        root_chord_m,
        tip_chord_m,
        span_m,
        sweep_m,
    } = set.planform
    else {
        return Err(format!("{DESIGN}'s fins are not a trapezoid"));
    };
    let sim = |error: hpr_sim::SimError| format!("{DESIGN}: {error}");
    let panel = FlutterPanel::of_fins(&set).map_err(sim)?;
    let modulus = materials::shear_modulus(MATERIAL).ok_or(format!("no `{MATERIAL}` modulus"))?;
    let g = modulus.shear_modulus_pa;

    // The first flight gives the launch site and the configurations; the rest fly in turn.
    let first = fly(root, 1)?;
    let count = first
        .pointer("/design/configurations")
        .and_then(Value::as_array)
        .map(Vec::len)
        .ok_or(format!("{DESIGN}'s flight lists no configurations"))?;
    let elevation_m = flown(&first, "/launch/elevation_m")?;
    if first.pointer("/launch/atmosphere").and_then(Value::as_str) != Some("standard") {
        return Err(format!(
            "{DESIGN} flies in other than the standard atmosphere"
        ));
    }
    let air = Ussa76::standard()
        .sample(elevation_m)
        .map_err(|error| error.to_string())?
        .air;
    let (pressure_pa, sound_m_s) = (air.pressure_pa, air.speed_of_sound_m_s);

    let q_f = panel.flutter_dynamic_pressure_pa(g).map_err(sim)?;
    let speed = panel
        .flutter_speed_m_s(g, pressure_pa, sound_m_s)
        .map_err(sim)?;
    let ratio = panel.figure_3_ratio(g, pressure_pa).map_err(sim)?;
    // The halved constant's denominator, `1.337 K p (λ + 1)/2`, in Pa when `p` is.
    let halved_pa =
        HALVED_PER_PSI * panel.shape_factor() * pressure_pa * (panel.taper_ratio() + 1.0) / 2.0;
    let halved_speed = sound_m_s * (g / halved_pa).sqrt();

    let mut flights = Vec::new();
    for number in 1..=count {
        let flight = if number == 1 {
            first.clone()
        } else {
            fly(root, number)?
        };
        let q = flown(&flight, "/summary/max_dynamic_pressure_pa/value")?;
        flights.push(json!({
            "configuration": number,
            "motors": flight
                .pointer("/design/configuration_label")
                .and_then(Value::as_str)
                .ok_or(format!("{DESIGN}'s configuration {number} has no label"))?,
            "max_speed_m_s": flown(&flight, "/summary/max_speed_m_s/value")?,
            "max_mach": flown(&flight, "/summary/max_mach/value")?,
            "max_dynamic_pressure_pa": q,
            "max_dynamic_pressure_height_m": flown(
                &flight,
                "/summary/max_dynamic_pressure_pa/height_above_ground_m",
            )?,
            // `FlutterMargin::speed_ratio`: √(q_f / q) at the flight's peak.
            "speed_ratio": (q_f / q).sqrt(),
            "halved_speed_ratio": halved_speed / speed * (q_f / q).sqrt(),
        }));
    }

    let mut by_thickness = Vec::new();
    for step in 0..THICKNESS_STEPS {
        #[allow(clippy::cast_precision_loss, reason = "a handful of steps")]
        let thickness_m = (THICKNESS_FIRST_MM + THICKNESS_STEP_MM * step as f64) / 1000.0;
        let thicker = FlutterPanel::new(
            panel.aspect_ratio(),
            panel.taper_ratio(),
            thickness_m / root_chord_m,
        )
        .map_err(sim)?;
        let at = thicker
            .flutter_speed_m_s(g, pressure_pa, sound_m_s)
            .map_err(sim)?;
        by_thickness.push(json!({
            "thickness_m": thickness_m,
            "flutter_speed_m_s": at,
            "halved_flutter_speed_m_s": at * halved_speed / speed,
            "figure_3_ratio": thicker.figure_3_ratio(g, pressure_pa).map_err(sim)?,
        }));
    }

    Ok(json!({
        "generator": "cargo xtask guides",
        "design": DESIGN,
        "note": "HPR Sim's fin-flutter numbers for the guides' rocket, by D. J. Martin's criterion \
                 (NACA TN 4197, eq. 18) as hpr_sim::flutter gives it, with the built-in material's \
                 shear modulus. launch: the launch site's standard air. flights: each \
                 configuration flown by hpr sim --json; speed_ratio is the flutter speed over the \
                 airspeed at the flight's peak dynamic pressure, sqrt(q_f / q). halved_*: the same \
                 with the constant 1.337 per psi printed in Peak of Flight #291 and #411, half of \
                 Martin's. by_thickness: the same fin outline at other thicknesses, at the launch \
                 site. A model's output, not a measurement.",
        "fins": {
            "count": set.count,
            "root_chord_m": root_chord_m,
            "tip_chord_m": tip_chord_m,
            "span_m": span_m,
            "sweep_m": sweep_m,
            "thickness_m": set.thickness_m,
            "material": set.material.name,
            "aspect_ratio": panel.aspect_ratio(),
            "taper_ratio": panel.taper_ratio(),
            "thickness_ratio": panel.thickness_ratio(),
        },
        "shear_modulus": {
            "material": MATERIAL,
            "shear_modulus_pa": g,
            "source": modulus.source,
        },
        "launch": {
            "elevation_m": elevation_m,
            "pressure_pa": pressure_pa,
            "speed_of_sound_m_s": sound_m_s,
        },
        "flutter_dynamic_pressure_pa": q_f,
        "flutter_speed_m_s": speed,
        "figure_3_ratio": ratio,
        "figure_3_band": FIGURE_3_BAND,
        "unfailed_mach": UNFAILED_MACH,
        "halved_per_psi": HALVED_PER_PSI,
        "halved_flutter_speed_m_s": halved_speed,
        "flights": flights,
        "by_thickness": by_thickness,
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
    let apart = apart(committed.as_ref(), &now);
    Ok((!apart.is_empty()).then(|| {
        let shown: Vec<&str> = apart.iter().take(3).map(String::as_str).collect();
        format!(
            "{REPORT} is not what {DESIGN} gives now ({}); run `cargo xtask guides` and commit it",
            shown.join("; ")
        )
    }))
}

/// Where `now` departs from the `committed` report: numbers more than
/// [`crate::ork_flights::CHECK_RELATIVE`] of their size apart, since a flight's last bits move
/// between platforms' maths libraries at its own scale, far below the guide's places; and any
/// key, text or count that differs. A missing report departs everywhere.
///
/// The one exception is where a flight's dynamic pressure peaks, held to [`PEAK_PLACE_M`].
fn apart(committed: Option<&Value>, now: &Value) -> Vec<String> {
    let mut apart = Vec::new();
    let Some(committed) = committed else {
        apart.push("no committed report".to_owned());
        return apart;
    };
    let mut committed = committed.clone();
    if let (Some(then), Some(flights)) = (
        committed.get_mut("flights").and_then(Value::as_array_mut),
        now.get("flights").and_then(Value::as_array),
    ) {
        for (then, flight) in then.iter_mut().zip(flights) {
            let (Some(a), Some(b)) = (then.get(PEAK_HEIGHT), flight.get(PEAK_HEIGHT)) else {
                continue;
            };
            if let (Some(a), Some(b)) = (a.as_f64(), b.as_f64())
                && (a - b).abs() <= PEAK_PLACE_M
            {
                then[PEAK_HEIGHT] = flight[PEAK_HEIGHT].clone();
            }
        }
    }
    crate::ork_flights::same(&committed, now, "", &mut apart);
    apart
}

/// A flight's height at its peak dynamic pressure, in the report.
const PEAK_HEIGHT: &str = "max_dynamic_pressure_height_m";
/// How far apart a committed and a regenerated report may put [`PEAK_HEIGHT`], m. A peak's place
/// is ill-conditioned where its value is not: near a smooth maximum the dynamic pressure changes
/// with the square of the distance from it, so the flight's last bits, which move between
/// platforms' maths libraries, move the place by about their square root. CI put the guide
/// rocket's second flight's max q 0.13 mm apart on Linux and Windows from macOS (318.51107 m
/// against 318.51093 m to 318.51104 m) while every peak value agreed to within
/// [`crate::ork_flights::CHECK_RELATIVE`]. A centimeter is a hundredth of the meter the guide
/// prints.
const PEAK_PLACE_M: f64 = 0.01;

/// A part's size in millimeters to `decimals` places, then in inches to two, converted from the
/// millimeters as written: `3 mm (0.12 in)`.
fn millimeters(value_m: f64, decimals: usize) -> String {
    let mm = value_m * 1000.0;
    let inches = written_value(mm, decimals) / 25.4;
    format!("{} mm ({} in)", fixed(mm, decimals), fixed(inches, 2))
}

/// A speed in meters a second to the meter, then in feet a second to the foot, converted from
/// the speed as written: `175 m/s (574 ft/s)`.
fn speed(value: f64) -> String {
    let feet = written_value(value, 0) / 0.3048;
    format!("{} m/s ({} ft/s)", fixed(value, 0), fixed(feet, 0))
}

/// A thickness the fins would need, rounded up to a tenth of a millimeter and then in inches to a
/// thousandth, so neither figure states less than the criterion asks. A thousandth keeps the
/// rounded-up inches within the site's conversion check, half a last place plus what rounding
/// the millimeters moves them.
fn needed(value_m: f64) -> String {
    let mm = (value_m * 1e4).ceil() / 10.0;
    let inches = (written_value(mm, 1) / 25.4 * 1000.0).ceil() / 1000.0;
    format!("{} mm ({} in)", fixed(mm, 1), fixed(inches, 3))
}

/// The flight with the highest peak dynamic pressure, where the fins come closest to their
/// flutter speed: its pointer in [`REPORT`]. The page calls it the faster flight and draws its
/// top speed, so it must also be the fastest.
fn closest(r: &Reports) -> Result<String, String> {
    let count = r
        .get(REPORT, "/flights")?
        .as_array()
        .map(Vec::len)
        .ok_or_else(|| format!("{REPORT} has no flights"))?;
    let mut best: Option<(String, f64)> = None;
    let mut fastest: Option<(String, f64)> = None;
    for at in 0..count {
        let pointer = format!("/flights/{at}");
        let q = r.number(REPORT, &format!("{pointer}/max_dynamic_pressure_pa"))?;
        let speed = r.number(REPORT, &format!("{pointer}/max_speed_m_s"))?;
        if best.as_ref().is_none_or(|(_, most)| q > *most) {
            best = Some((pointer.clone(), q));
        }
        if fastest.as_ref().is_none_or(|(_, most)| speed > *most) {
            fastest = Some((pointer, speed));
        }
    }
    match (best, fastest) {
        (Some((best, _)), Some((fastest, _))) if best == fastest => Ok(best),
        (Some(_), Some(_)) => Err(format!(
            "{REPORT}'s flight with the highest max q is not its fastest; the page calls it the \
             faster"
        )),
        _ => Err(format!("{REPORT} has no flights")),
    }
}

/// Where a figure 3 ratio sits against Martin's band.
fn against_band(ratio: f64, band: [f64; 2]) -> &'static str {
    if ratio > band[1] {
        "above"
    } else if ratio < band[0] {
        "below"
    } else {
        "on"
    }
}

/// Martin's band as the report gives it.
fn band(r: &Reports) -> Result<[f64; 2], String> {
    let band = [
        r.number(REPORT, "/figure_3_band/0")?,
        r.number(REPORT, "/figure_3_band/1")?,
    ];
    if band[0] < band[1] && band[0] > 0.0 {
        Ok(band)
    } else {
        Err(format!("{REPORT}'s band is not two rising positive ratios"))
    }
}

/// The thickness at which the fins' figure 3 ratio reaches the band's lower edge, m: the ratio
/// goes as the thickness to the minus third power, the outline held.
fn thickness_below_band(r: &Reports) -> Result<f64, String> {
    let ratio = r.number(REPORT, "/figure_3_ratio")?;
    let [low, _] = band(r)?;
    Ok(r.number(REPORT, "/fins/thickness_m")? * (ratio / low).cbrt())
}

/// The thickness at which the flutter speed reaches the airspeed at the closest flight's peak
/// dynamic pressure, m: the flutter speed goes as the thickness to the power one and a half.
fn thickness_for_airspeed(r: &Reports) -> Result<f64, String> {
    let flight = closest(r)?;
    let ratio = r.number(REPORT, &format!("{flight}/speed_ratio"))?;
    if ratio <= 0.0 {
        return Err(format!("{REPORT}'s speed ratio is not positive"));
    }
    Ok(r.number(REPORT, "/fins/thickness_m")? * ratio.powf(-2.0 / 3.0))
}

/// "0.63 times the airspeed there: faster than the flutter speed", the direction tied to the
/// number so the hole moves with either.
fn times_airspeed(ratio: f64) -> String {
    let side = if ratio < 1.0 {
        "the fins fly faster than their flutter speed"
    } else {
        "the fins fly slower than their flutter speed"
    };
    format!(
        "{} times the airspeed there: by this criterion, {side}",
        fixed(ratio, 2)
    )
}

/// The fin-flutter guide's quotes.
pub(super) fn quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "that flew to at least Mach {}",
            holes: |r| Ok(vec![fixed(r.number(REPORT, "/unfailed_mach")?, 1)]),
        },
        Quote {
            template: "has {} fins of {}, {} thick, with a {} root chord, a {} tip chord and a {} \
                       span",
            holes: |r| {
                Ok(vec![
                    r.count(REPORT, "/fins/count")?.to_string(),
                    r.text(REPORT, "/fins/material")?.to_lowercase(),
                    millimeters(r.number(REPORT, "/fins/thickness_m")?, 0),
                    millimeters(r.number(REPORT, "/fins/root_chord_m")?, 0),
                    millimeters(r.number(REPORT, "/fins/tip_chord_m")?, 0),
                    millimeters(r.number(REPORT, "/fins/span_m")?, 0),
                ])
            },
        },
        Quote {
            template: "their leading edges are swept back by {} over the span",
            holes: |r| Ok(vec![millimeters(r.number(REPORT, "/fins/sweep_m")?, 0)]),
        },
        Quote {
            template: "a cited shear modulus, {} GPa",
            holes: |r| {
                Ok(vec![fixed(
                    r.number(REPORT, "/shear_modulus/shear_modulus_pa")? / 1e9,
                    2,
                )])
            },
        },
        Quote {
            template: "an aspect ratio of {}, a taper ratio of {} and a thickness ratio of {}",
            holes: |r| {
                Ok(vec![
                    fixed(r.number(REPORT, "/fins/aspect_ratio")?, 2),
                    fixed(r.number(REPORT, "/fins/taper_ratio")?, 2),
                    fixed(r.number(REPORT, "/fins/thickness_ratio")? * 100.0, 1) + "%",
                ])
            },
        },
        Quote {
            template: "the criterion puts their flutter speed at {}, Mach {}",
            holes: |r| {
                let at = r.number(REPORT, "/flutter_speed_m_s")?;
                let sound = r.number(REPORT, "/launch/speed_of_sound_m_s")?;
                Ok(vec![speed(at), fixed(at / sound, 2)])
            },
        },
        Quote {
            template: "On the faster of its motors, the rocket reaches {}, Mach {}",
            holes: |r| {
                let flight = closest(r)?;
                Ok(vec![
                    speed(r.number(REPORT, &format!("{flight}/max_speed_m_s"))?),
                    fixed(r.number(REPORT, &format!("{flight}/max_mach"))?, 2),
                ])
            },
        },
        Quote {
            template: "Max q comes {} above the pad, in air close to the launch site's",
            holes: |r| {
                let flight = closest(r)?;
                Ok(vec![meters(
                    r.number(REPORT, &format!("{flight}/max_dynamic_pressure_height_m"))?,
                    0,
                )])
            },
        },
        Quote {
            template: "At its max q, {} kPa, the flutter speed is {}",
            holes: |r| {
                let flight = closest(r)?;
                Ok(vec![
                    fixed(
                        r.number(REPORT, &format!("{flight}/max_dynamic_pressure_pa"))? / 1e3,
                        1,
                    ),
                    times_airspeed(r.number(REPORT, &format!("{flight}/speed_ratio"))?),
                ])
            },
        },
        Quote {
            template: "the plywood would have to be {} thick",
            holes: |r| Ok(vec![needed(thickness_for_airspeed(r)?)]),
        },
        Quote {
            template: "The band runs at a ratio of {} to {}, a flutter speed {} to {} times the \
                       speed of sound",
            holes: |r| {
                let [low, high] = band(r)?;
                // The ratio is `(a / V_f)²`, so its higher edge is the slower flutter speed.
                Ok(vec![
                    fixed(low, 2),
                    fixed(high, 2),
                    fixed(1.0 / high.sqrt(), 1),
                    fixed(1.0 / low.sqrt(), 1),
                ])
            },
        },
        Quote {
            template: "The guides' rocket's fins plot at {}, {} the band",
            holes: |r| {
                let ratio = r.number(REPORT, "/figure_3_ratio")?;
                Ok(vec![
                    fixed(ratio, 2),
                    against_band(ratio, band(r)?).to_owned(),
                ])
            },
        },
        Quote {
            template: "to plot below it with the same outline, they would have to be {} thick",
            holes: |r| Ok(vec![needed(thickness_below_band(r)?)]),
        },
        Quote {
            template: "the rocket tops out at Mach {}",
            holes: |r| {
                let flight = closest(r)?;
                Ok(vec![fixed(
                    r.number(REPORT, &format!("{flight}/max_mach"))?,
                    2,
                )])
            },
        },
        Quote {
            template: "its flutter speed reads {} high",
            holes: |r| {
                let halved = r.number(REPORT, "/halved_flutter_speed_m_s")?;
                let hpr = r.number(REPORT, "/flutter_speed_m_s")?;
                Ok(vec![format!("{}%", fixed((halved / hpr - 1.0) * 100.0, 0))])
            },
        },
        Quote {
            template: "the halved constant puts the guides' rocket's flutter speed at {}, and the \
                       ratio at max q at {}",
            holes: |r| {
                let flight = closest(r)?;
                Ok(vec![
                    speed(r.number(REPORT, "/halved_flutter_speed_m_s")?),
                    fixed(
                        r.number(REPORT, &format!("{flight}/halved_speed_ratio"))?,
                        2,
                    ),
                ])
            },
        },
    ]
}

/// The figure's thickness axis, mm, and its speed axis's top, m/s, in steps of this.
const THICKNESS_AXIS_MM: [f64; 2] = [1.0, 8.0];
const SPEED_STEP: f64 = 200.0;

/// The x of a thickness, m.
fn thickness_x(thickness_m: f64) -> f64 {
    let [first, last] = THICKNESS_AXIS_MM;
    PLOT_LEFT + (thickness_m * 1000.0 - first) / (last - first) * (PLOT_RIGHT - PLOT_LEFT)
}

/// One row of the thickness sweep.
struct Row {
    thickness_m: f64,
    speed: f64,
    halved: f64,
}

/// The sweep's rows, each recorded as read.
fn rows(r: &Reports) -> Result<Vec<Row>, String> {
    let count = r
        .get(REPORT, "/by_thickness")?
        .as_array()
        .map(Vec::len)
        .ok_or_else(|| format!("{REPORT} has no thickness sweep"))?;
    let rows = (0..count)
        .map(|at| {
            let number = |key: &str| r.number(REPORT, &format!("/by_thickness/{at}/{key}"));
            Ok(Row {
                thickness_m: number("thickness_m")?,
                speed: number("flutter_speed_m_s")?,
                halved: number("halved_flutter_speed_m_s")?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    if rows
        .windows(2)
        .any(|pair| pair[0].thickness_m >= pair[1].thickness_m)
    {
        return Err(format!("{REPORT}'s thicknesses are not in rising order"));
    }
    Ok(rows)
}

/// The speed axis's top: the next step above every row's speeds and `others`.
fn axis_top(rows: &[Row], others: &[f64]) -> Result<f64, String> {
    let most = rows
        .iter()
        .map(|row| row.halved.max(row.speed))
        .chain(others.iter().copied())
        .reduce(f64::max)
        .ok_or("no rows")?;
    Ok((most / SPEED_STEP).ceil() * SPEED_STEP)
}

/// The fin-flutter guide's figure: the guide rocket's flutter speed at the launch site against
/// its fins' thickness, HPR Sim's line and the halved constant's above it in the caution color,
/// Martin's band shaded, the rocket's top speed on the faster of its motors as a dash-dot line, and
/// its own fins as a dot.
pub(super) fn draw_speed_by_thickness(r: &Reports) -> Result<String, String> {
    let rows = rows(r)?;
    let [low, high] = band(r)?;
    let sound = r.number(REPORT, "/launch/speed_of_sound_m_s")?;
    let flight = closest(r)?;
    let top = r.number(REPORT, &format!("{flight}/max_speed_m_s"))?;
    let own = (
        r.number(REPORT, "/fins/thickness_m")?,
        r.number(REPORT, "/flutter_speed_m_s")?,
    );
    // Martin's band as speeds: a ratio `(a / V_f)²` at its higher edge is the slower speed.
    let (band_slow, band_fast) = (sound / high.sqrt(), sound / low.sqrt());
    let [first, last] = THICKNESS_AXIS_MM;
    if rows
        .iter()
        .map(|row| row.thickness_m)
        .chain([own.0])
        .any(|thickness_m| thickness_m * 1000.0 < first || thickness_m * 1000.0 > last)
    {
        return Err(format!(
            "a thickness past the figure's axis, {first} to {last} mm"
        ));
    }
    let speed_top = axis_top(&rows, &[band_fast, top, own.1])?;
    let y = |speed: f64| PLOT_BOTTOM - speed / speed_top * (PLOT_BOTTOM - PLOT_TOP);
    let height = PLOT_BOTTOM + 150.0;
    let title = "The guides' rocket's flutter speed by fin thickness";
    let lines = [
        "HPR Sim's flutter speed at the launch site (line), by Martin's criterion, for the"
            .to_owned(),
        "rocket's birch plywood fin outline at each thickness. Shaded: Martin's band, where his"
            .to_owned(),
        "line between wings that fluttered and wings that didn't falls. Dash-dot line: the"
            .to_owned(),
        "rocket's top speed on the faster of its motors.".to_owned(),
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
    // Martin's band, behind everything.
    let _ = writeln!(
        svg,
        "<rect x=\"{PLOT_LEFT}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{RULE}\"/>",
        y(band_fast),
        PLOT_RIGHT - PLOT_LEFT,
        y(band_slow) - y(band_fast)
    );
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">Martin's band</text>",
        PLOT_LEFT + 8.0,
        f64::midpoint(y(band_fast), y(band_slow)) + 4.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">flutter speed, m/s</text>",
        PLOT_TOP - 14.0
    );
    // The speed axis.
    let mut tick = 0.0;
    while tick <= speed_top {
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
        tick += SPEED_STEP;
    }
    // The top speed, a chain line across the plot.
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{PLOT_RIGHT}\" y2=\"{:.1}\" stroke=\"{INK}\" \
         stroke-width=\"1\" stroke-dasharray=\"24 3 1 3\"/>",
        y(top),
        y(top)
    );
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"{INK}\">top speed</text>",
        PLOT_RIGHT - 4.0,
        y(top) + 16.0
    );
    // The two lines through the sweep: the halved constant's in the caution color, then HPR
    // Sim's.
    let path = |speed: &dyn Fn(&Row) -> f64| -> String {
        rows.iter()
            .enumerate()
            .map(|(at, row)| {
                format!(
                    "{} {:.1} {:.1}",
                    if at == 0 { "M" } else { "L" },
                    thickness_x(row.thickness_m),
                    y(speed(row))
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let _ = writeln!(
        svg,
        "<path d=\"{}\" fill=\"none\" stroke=\"{CAUTION}\" stroke-width=\"3\"/>",
        path(&|row| row.halved)
    );
    let _ = writeln!(
        svg,
        "<path d=\"{}\" fill=\"none\" stroke=\"{PREDICTED}\" stroke-width=\"2\"/>",
        path(&|row| row.speed)
    );
    // The rocket's own fins.
    let (x, at) = (thickness_x(own.0), y(own.1));
    let _ = writeln!(
        svg,
        "<circle cx=\"{x:.1}\" cy=\"{at:.1}\" r=\"4\" fill=\"{INK}\"/><text x=\"{:.1}\" \
         y=\"{:.1}\" fill=\"{INK}\">the guides' rocket</text>",
        x + 8.0,
        at + 18.0
    );
    // The axes.
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_TOP}\" x2=\"{PLOT_LEFT}\" y2=\"{PLOT_BOTTOM}\" \
         stroke=\"{INK}\" stroke-width=\"1\"/><line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_BOTTOM}\" \
         x2=\"{PLOT_RIGHT}\" y2=\"{PLOT_BOTTOM}\" stroke=\"{INK}\" stroke-width=\"1\"/>"
    );
    let mut tick = first;
    while tick <= last {
        let _ = writeln!(
            svg,
            "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">{}</text>",
            thickness_x(tick / 1000.0),
            PLOT_BOTTOM + 18.0,
            fixed(tick, 0)
        );
        tick += 1.0;
    }
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">fin thickness, \
         mm</text>",
        f64::midpoint(PLOT_LEFT, PLOT_RIGHT),
        PLOT_BOTTOM + 40.0
    );
    // The key: the flattering side first.
    let key = PLOT_BOTTOM + 64.0;
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{CAUTION}\" \
         stroke-width=\"3\"/><text x=\"{:.1}\" y=\"{key:.1}\" font-weight=\"600\" \
         fill=\"{CAUTION}\">flatters: the halved constant's flutter speed, √2 above HPR Sim's</text>",
        key - 4.0,
        PLOT_LEFT + 20.0,
        key - 4.0,
        PLOT_LEFT + 28.0
    );
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{PREDICTED}\" \
         stroke-width=\"2\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">HPR Sim: Martin's \
         constant</text>",
        key + 16.0,
        PLOT_LEFT + 20.0,
        key + 16.0,
        PLOT_LEFT + 28.0,
        key + 20.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Drawn by cargo xtask guides from \
         {REPORT}.</text>",
        height - 32.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Criterion and band: D. J. Martin, NACA TN \
         4197 (1958), eq. 18 and figure 3.</text>",
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
        let q = now["flights"][0]["max_dynamic_pressure_pa"]
            .as_f64()
            .unwrap();
        let relative = crate::ork_flights::CHECK_RELATIVE;
        moved["flights"][0]["max_dynamic_pressure_pa"] = json!(q * (1.0 + 10.0 * relative));
        assert_eq!(
            apart(Some(&moved), &now),
            [format!(
                "/flights/0/max_dynamic_pressure_pa: {} then, {q} now",
                q * (1.0 + 10.0 * relative)
            )]
        );
        let mut shorter = now.clone();
        shorter["by_thickness"].as_array_mut().unwrap().pop();
        assert!(!apart(Some(&shorter), &now).is_empty());
        assert!(!apart(None, &now).is_empty());
        // The peak's height is held to a centimeter, not to the relative tolerance.
        let height = now["flights"][1][PEAK_HEIGHT].as_f64().unwrap();
        let mut near = now.clone();
        near["flights"][1][PEAK_HEIGHT] = json!(height + 0.5 * PEAK_PLACE_M);
        assert!(apart(Some(&near), &now).is_empty());
        let mut far = now.clone();
        far["flights"][1][PEAK_HEIGHT] = json!(height + 2.0 * PEAK_PLACE_M);
        assert_eq!(apart(Some(&far), &now).len(), 1);
        assert!(apart(Some(&now.clone()), &now).is_empty());
    }

    /// The report's flutter speed and figure 3 ratio are `hpr_sim::flutter`'s on the design's
    /// fins, and the thicknesses the guide quotes are where the library's numbers reach the
    /// airspeed and the band.
    #[test]
    fn the_quoted_thicknesses_reach_their_lines() {
        let r = reports();
        let root = crate::designs::root().unwrap();
        let panel = FlutterPanel::of_fins(&fins(&root).unwrap()).unwrap();
        let g = materials::shear_modulus(MATERIAL).unwrap().shear_modulus_pa;
        let p = r.number(REPORT, "/launch/pressure_pa").unwrap();
        let ratio = panel.figure_3_ratio(g, p).unwrap();
        assert!((ratio / r.number(REPORT, "/figure_3_ratio").unwrap() - 1.0).abs() < 1e-12);
        let chord = r.number(REPORT, "/fins/root_chord_m").unwrap();
        let at = |thickness: f64| {
            FlutterPanel::new(panel.aspect_ratio(), panel.taper_ratio(), thickness / chord).unwrap()
        };
        let band_edge = at(thickness_below_band(&r).unwrap())
            .figure_3_ratio(g, p)
            .unwrap();
        assert!((band_edge - FIGURE_3_BAND[0]).abs() < 1e-9, "{band_edge}");
        let flight = closest(&r).unwrap();
        let q = r
            .number(REPORT, &format!("{flight}/max_dynamic_pressure_pa"))
            .unwrap();
        let q_f = at(thickness_for_airspeed(&r).unwrap())
            .flutter_dynamic_pressure_pa(g)
            .unwrap();
        assert!((q_f / q - 1.0).abs() < 1e-9, "{q_f} against {q}");
    }

    /// The figure's caution line is the halved constant's flutter speed, √2 above HPR Sim's at
    /// every thickness: the side `hpr_sim::flutter`'s
    /// `the_halved_constant_reads_a_higher_flutter_speed` pins as a flutter speed that reads
    /// high. It is the only caution stroke in the figure.
    #[test]
    fn the_flattering_line_is_the_halved_constant() {
        let r = reports();
        for row in rows(&r).unwrap() {
            let ratio = row.halved / row.speed;
            assert!((ratio - 2f64.sqrt()).abs() < 1e-3, "{ratio}");
        }
        let figure = draw_speed_by_thickness(&r).unwrap();
        let caution: Vec<&str> = figure
            .lines()
            .filter(|line| line.contains(&format!("stroke=\"{CAUTION}\"")))
            .collect();
        assert_eq!(caution.len(), 2, "the line and its key: {caution:?}");
        let line = caution
            .iter()
            .find(|line| line.starts_with("<path"))
            .unwrap();
        let rows = rows(&r).unwrap();
        let sound = r.number(REPORT, "/launch/speed_of_sound_m_s").unwrap();
        let flight = closest(&r).unwrap();
        let others = [
            sound / FIGURE_3_BAND[0].sqrt(),
            r.number(REPORT, &format!("{flight}/max_speed_m_s"))
                .unwrap(),
            r.number(REPORT, "/flutter_speed_m_s").unwrap(),
        ];
        let top = axis_top(&rows, &others).unwrap();
        for row in &rows {
            let point = format!(
                "{:.1} {:.1}",
                thickness_x(row.thickness_m),
                PLOT_BOTTOM - row.halved / top * (PLOT_BOTTOM - PLOT_TOP)
            );
            assert!(line.contains(&point), "{point} not in {line}");
        }
    }

    /// The figure passes the figure checks and draws what the guide's description says.
    #[test]
    fn the_figure_draws_its_description() {
        let figure = draw_speed_by_thickness(&reports()).unwrap();
        assert!(crate::figures::check(&figure).is_empty());
        assert!(crate::figures::drawing::check(&figure).is_empty());
        assert!(figure.contains(">Martin's band<"));
        assert!(figure.contains(">top speed<"));
        assert!(figure.contains("flatters: the halved constant"));
    }

    /// Needed thicknesses round up, so the page never states less than the criterion asks.
    #[test]
    fn needed_thicknesses_round_up() {
        assert_eq!(needed(0.004_01), "4.1 mm (0.162 in)");
        // 4.09 mm needed: 4.1 mm, and 0.162 in, 4.115 mm, not 0.16 in, 4.064 mm.
        assert_eq!(needed(0.004_09), "4.1 mm (0.162 in)");
        assert_eq!(needed(0.007_41), "7.5 mm (0.296 in)");
        assert_eq!(millimeters(0.003, 0), "3 mm (0.12 in)");
        assert_eq!(speed(175.4), "175 m/s (574 ft/s)");
    }
}
