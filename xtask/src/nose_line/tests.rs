use hpr_aero::{AeroModel, DragConditions, Flow};
use hpr_io::ork;
use serde_json::json;

use super::*;
use crate::gap_split::{Apogees, Parts, split};
use crate::histories::own::OWN_HISTORY_BOUND;
use crate::histories::tests::{Public, public_flights};

/// M2.3c10's probe: three invented designs and OpenRocket 24.12's drag columns for each.
const PROBE: &str = "validation/fixtures/ork/openrocket-nose-pressure-drag.json";

fn read(path: &str) -> Value {
    let root = crate::ork::root().unwrap();
    serde_json::from_str(&std::fs::read_to_string(root.join(path)).unwrap()).unwrap()
}

/// A probe design's layout.
fn probe_layout(probe: &Value, design: &str) -> Layout {
    let document = probe["designs"][design]["document"].as_str().unwrap();
    let read = ork::read(document.as_bytes()).unwrap();
    ork::design(&read.value).value.rocket.layout().unwrap()
}

/// A column as `nose_columns.py` writes one.
fn column(rows: &[(f64, f64)], area_m2: f64, shape: &str) -> Value {
    json!({
        "rows": rows.iter().map(|(mach, cd)| json!([mach, cd])).collect::<Vec<_>>(),
        "reference_area_m2": area_m2,
        "nose": {"shape": shape, "shape_parameter": if shape == "OGIVE" { 1.0 } else { 0.0 }},
    })
}

/// The public record was written by the scripts in the tree, by OpenRocket 24.12, on the public
/// designs as they are, and names the reasons this module knows.
#[test]
fn the_public_record_is_the_oracles() {
    let root = crate::ork::root().unwrap();
    let record = read(PUBLIC_RECORD);
    let inputs = record["inputs_sha256"].as_object().unwrap();
    assert!(inputs.contains_key(ORACLE));
    for (path, recorded) in inputs {
        let bytes = std::fs::read(root.join(path)).unwrap();
        assert_eq!(
            recorded.as_str(),
            Some(crate::ork_supply::sha256(&bytes).as_str()),
            "{path} changed since {PUBLIC_RECORD} was written; run {ORACLE} --public again"
        );
    }
    assert_eq!(check_record(&record, PUBLIC_RECORD), Ok(()));
    let designs = record["designs"].as_array().unwrap();
    assert_eq!(designs.len(), 2);
    for design in designs {
        let file = design["file"].as_str().unwrap();
        let bytes = std::fs::read(root.join(file)).unwrap();
        assert_eq!(
            crate::ork_supply::sha256(&bytes),
            design["sha256"].as_str().unwrap()
        );
        for flight in design["flights"].as_array().unwrap() {
            let column = NoseColumn::of(flight).unwrap().unwrap();
            assert_eq!(column.shape, ("CONICAL".to_owned(), 0.0));
            assert!(!column.is_tangent_ogive());
            assert_eq!(column.table.domain(), (1.0, 2.5));
        }
    }
    // A record with another reason, or from another release, is refused.
    let mut other = record.clone();
    other["reasons"]["something_else"] = json!("");
    assert!(check_record(&other, "x").unwrap_err().contains("reasons"));
    let mut other = record.clone();
    other["openrocket"] = json!("23.09");
    assert!(check_record(&other, "x").unwrap_err().contains("24.12"));
}

/// On M2.3c10's public probe, a fineness-3 tangent ogive on a 50 mm tube, the swap takes HPR
/// Sim's pressure drag of the nose out of the rocket's drag and puts OpenRocket 24.12's in, above
/// Mach 1 and nowhere else. HPR Sim's is the equivalent cone's, 0.15571, 0.13028 and 0.10421 at
/// Mach 1.3, 1.5 and 2; OpenRocket's is 0.00098, 0.00073 and 0.00047. The nose's base is the
/// rocket's reference area here.
#[test]
fn the_swap_replaces_the_nose_s_pressure_drag_above_mach_1_only() {
    let probe = read(PROBE);
    let nose_id = probe["nose_id"].as_str().unwrap();
    let layout = probe_layout(&probe, "tangent ogive");
    assert_eq!(tangent_ogive_nose(&layout), Some((nose_id, 1)));
    let recorded: Vec<(f64, f64, f64)> = probe["designs"]["tangent ogive"]["machs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|at| {
            (
                at["mach"].as_f64().unwrap(),
                at["components"][nose_id]["pressure"].as_f64().unwrap(),
                at["reference_area_m2"].as_f64().unwrap(),
            )
        })
        .collect();
    let area = recorded[0].2;
    assert_eq!(area, probe["nose_base_area_m2"].as_f64().unwrap());
    let rows: Vec<(f64, f64)> = recorded.iter().map(|(mach, cd, _)| (*mach, *cd)).collect();
    let theirs = NoseColumn::of(&column(&rows, area, "OGIVE"))
        .unwrap()
        .unwrap();
    assert!(theirs.is_tangent_ogive());
    let own = AeroModel::new(&layout).unwrap();
    assert!((own.reference_area_m2() - area).abs() < 1e-15);
    let swapped = AeroModel::new(&layout)
        .unwrap()
        .with_drag_model(NoseSwap::new(nose_id, theirs.clone()));
    let conditions = DragConditions::coasting(1.0e7);
    let drag = |model: &AeroModel, mach: f64| {
        model
            .drag(&Flow::new(mach, 0.0, 0.0), &conditions)
            .map(|drag| drag.zero_lift_coefficient)
    };
    let expected = [(0.15571, 0.00098), (0.13028, 0.00073), (0.10421, 0.00047)];
    for ((mach, openrocket, _), (hpr_pinned, openrocket_pinned)) in recorded.iter().zip(expected) {
        let nose = own
            .buildup_components(&Flow::new(*mach, 0.0, 0.0), &conditions)
            .unwrap()
            .into_iter()
            .find(|component| component.id == nose_id)
            .unwrap()
            .drag
            .pressure;
        assert!((nose - hpr_pinned).abs() < 5e-6, "Mach {mach}: {nose}");
        assert!((openrocket - openrocket_pinned).abs() < 5e-6, "Mach {mach}");
        let (before, after) = (drag(&own, *mach).unwrap(), drag(&swapped, *mach).unwrap());
        assert!(
            (after - (before - nose + openrocket)).abs() < 1e-15,
            "Mach {mach}: {before} to {after}"
        );
        // The nose's own drag is most of what the swap takes out.
        assert!(before - after > 0.99 * hpr_pinned - 0.001, "Mach {mach}");
    }
    // A column on another reference area is put on the rocket's.
    let double = NoseColumn::of(&column(&rows, 2.0 * area, "OGIVE"))
        .unwrap()
        .unwrap();
    let on_double = AeroModel::new(&layout)
        .unwrap()
        .with_drag_model(NoseSwap::new(nose_id, double));
    let (mach, openrocket, _) = recorded[1];
    let moved = drag(&on_double, mach).unwrap() - drag(&swapped, mach).unwrap();
    assert!((moved - openrocket).abs() < 1e-15, "{moved}");
    // At Mach 1 and below, HPR Sim's own drag, bit for bit.
    for mach in [0.3, 0.9, 1.0] {
        assert_eq!(drag(&swapped, mach).unwrap(), drag(&own, mach).unwrap());
    }
    // Between Mach 1 and the column's first row, and past its last, the flight is refused: the
    // collection's columns start at Mach 1.
    for mach in [1.1, 2.01] {
        let error = drag(&swapped, mach).unwrap_err().to_string();
        assert!(error.contains("drag model"), "{error}");
    }
    // A part the buildup doesn't list is refused too.
    let lost = AeroModel::new(&layout)
        .unwrap()
        .with_drag_model(NoseSwap::new("no such part", theirs));
    assert!(drag(&lost, 1.5).is_err());
    assert_eq!(drag(&lost, 0.9).unwrap(), drag(&own, 0.9).unwrap());
}

/// The line's flights are those whose own flight passes Mach 1 with a tangent-ogive nose at the
/// front of the airframe: not a cone, not a secant ogive, and not a flight that only reaches
/// Mach 1.
#[test]
fn the_line_s_flights_pass_mach_1_with_a_tangent_ogive_nose() {
    let probe = read(PROBE);
    let ogive = probe_layout(&probe, "tangent ogive");
    assert!(in_line(1.000_001, &ogive));
    assert!(in_line(2.4, &ogive));
    for max_mach in [0.0, 0.99, 1.0, f64::NAN] {
        assert!(!in_line(max_mach, &ogive), "{max_mach}");
    }
    for control in ["cone", "cone, wider body"] {
        let cone = probe_layout(&probe, control);
        assert_eq!(tangent_ogive_nose(&cone), None);
        assert!(!in_line(1.5, &cone));
    }
    // The same nose as a secant ogive, of twice the tangent radius, is not in the line.
    let document = probe["designs"]["tangent ogive"]["document"]
        .as_str()
        .unwrap();
    let mut rocket = ork::design(&ork::read(document.as_bytes()).unwrap().value)
        .value
        .rocket;
    let Part::NoseCone(nose) = &mut rocket.stages[0].components[0].part else {
        panic!("the probe opens with its nose");
    };
    assert_eq!(nose.shape, NoseShape::Ogive { radius_ratio: 1.0 });
    nose.shape = NoseShape::Ogive { radius_ratio: 2.0 };
    assert!(!in_line(1.5, &rocket.layout().unwrap()));
}

fn fly(simulation: &Simulation) -> Result<f64, String> {
    crate::fixture_flights::climbed_m(simulation)
}

/// The public Mach probe, which passes Mach 1.6.
fn mach_probe() -> Public {
    let mut flights = public_flights();
    let probe = flights.remove(1);
    assert!(probe.file.ends_with("mach-probe.ork"), "{}", probe.file);
    probe
}

/// OpenRocket's column for the public Mach probe's nose.
fn mach_probe_column() -> Value {
    let record = read(PUBLIC_RECORD);
    let design = record["designs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|design| design["file"].as_str().unwrap().ends_with("mach-probe.ork"))
        .unwrap();
    design["flights"][0].clone()
}

/// The control. The public Mach probe's nose is a cone, which the two programs give the same
/// pressure drag at Mach 1.3, 1.5 and 2 (M2.3c10):
/// flown past Mach 1.6 on OpenRocket's thrust and mass, with the cone's column from OpenRocket
/// swapped in, HPR Sim's apogee moves by under 0.01% of itself. And a swap that never applies
/// leaves the flight as it was, bit for bit.
#[test]
fn a_cone_s_column_from_openrocket_moves_the_public_mach_probe_by_under_a_ten_thousandth() {
    let probe = mach_probe();
    let layout = probe.build().unwrap().assembly().layout.clone();
    assert_eq!(tangent_ogive_nose(&layout), None);
    assert!(max_mach(&probe.build().unwrap()).unwrap() > 1.6);
    let nose_id = layout.body().next().unwrap().id.clone();
    let inputs = OpenRocketInputs::of(&probe.flight).unwrap().unwrap();
    let on_histories = || {
        probe
            .build()
            .unwrap()
            .with_thrust_history(inputs.thrust.clone())
            .unwrap()
            .with_mass_history(inputs.mass.clone())
            .unwrap()
    };
    let cone = NoseColumn::of(&mach_probe_column()).unwrap().unwrap();
    let before_m = fly(&on_histories()).unwrap();
    let after_m =
        fly(&on_histories().with_drag_model(NoseSwap::new(&nose_id, cone.clone()))).unwrap();
    let moved = (after_m - before_m) / before_m;
    eprintln!("the cone's column moves the Mach probe's apogee by {moved:+.3e} of itself");
    assert!(moved.abs() < 1e-4, "{moved}");
    assert!(moved != 0.0, "the swap did not apply");
    let never = NoseSwap {
        above_mach: f64::INFINITY,
        ..NoseSwap::new(&nose_id, cone)
    };
    assert_eq!(
        fly(&on_histories().with_drag_model(never)).unwrap(),
        before_m
    );
}

/// The public Mach probe with its nose made a tangent ogive, in HPR Sim alone: OpenRocket's
/// record stays the cone's, so the numbers size nothing between the programs; the flight is here
/// to drive the line's code.
fn ogive_probe() -> Public {
    mach_probe().edited(|rocket| {
        let Part::NoseCone(nose) = &mut rocket.stages[0].components[0].part else {
            panic!("the probe opens with its nose");
        };
        nose.shape = NoseShape::Ogive { radius_ratio: 1.0 };
    })
}

/// A column like OpenRocket 24.12's for a tangent ogive: almost nothing, at every Mach number.
fn almost_nothing(shape: &str) -> Value {
    let cone = mach_probe_column();
    column(
        &[(1.0, 0.001), (2.5, 0.0005)],
        cone["reference_area_m2"].as_f64().unwrap(),
        shape,
    )
}

/// A flight of the line is swapped, its line and the rest summing to the drag part, with the
/// line below zero: HPR Sim climbs lower on its own nose drag than on almost none. Or it is
/// skipped, under the first reason that applies. A flight outside the line is not counted.
#[test]
fn a_flight_of_the_line_is_swapped_or_skipped_under_its_reason() {
    let probe = ogive_probe();
    let own_m = fly(&probe.build().unwrap()).unwrap();
    let openrocket_m = probe.flight["apogee_m"].as_f64().unwrap();
    let build = |_air: Air| probe.build();
    let split = split(
        &build,
        &fly,
        &probe.flight,
        (own_m, openrocket_m),
        OWN_HISTORY_BOUND,
    )
    .unwrap();
    let ogive = almost_nothing("OGIVE");
    let of = |column: Option<&Value>, split: &Result<SplitFlight, NotSplit>| {
        flight(&build, &fly, &probe.flight, column, split, openrocket_m)
    };
    let Some(NoseFlight::Swapped {
        nose,
        rest,
        drag,
        top_mach,
    }) = of(Some(&ogive), &split).unwrap()
    else {
        panic!("the flight is swapped");
    };
    assert_eq!(top_mach, max_mach(&probe.build().unwrap()).unwrap());
    eprintln!("nose {nose:+.3}% + rest {rest:+.3}% = drag {drag:+.3}%");
    assert_eq!(drag, split.as_ref().unwrap().parts.drag);
    assert!((nose + rest - drag).abs() <= SUM_TOLERANCE_PERCENT);
    assert!(nose < -1.0, "{nose}");
    // The rest runs to the flight on OpenRocket's whole drag, not to the one in its air after.
    let mut other_air = *split.as_ref().unwrap();
    other_air.apogees.air_m += 100.0;
    assert_eq!(
        of(Some(&ogive), &Ok(other_air)),
        Ok(Some(NoseFlight::Swapped {
            nose,
            rest,
            drag,
            top_mach
        }))
    );

    let skipped = |name: &str| {
        let index = SKIPPED.iter().position(|(reason, _)| *reason == name);
        Ok(Some(NoseFlight::Skipped(index.unwrap())))
    };
    let not_split = Err(NotSplit {
        reason: 0,
        detail: String::new(),
    });
    assert_eq!(of(Some(&ogive), &not_split), skipped("not_split"));
    for reason in REASONS {
        let lacks = json!({ "lacks": reason });
        assert_eq!(of(Some(&lacks), &split), skipped("no_column"));
    }
    // A column that ends below the flight's top Mach number: HPR Sim fails the flight.
    let short = column(
        &[(1.0, 0.001), (1.2, 0.001)],
        ogive["reference_area_m2"].as_f64().unwrap(),
        "OGIVE",
    );
    assert_eq!(of(Some(&short), &split), skipped("refused"));
    // No record of the flight, or a nose OpenRocket read as another shape, stops the run.
    assert!(of(None, &split).unwrap_err().contains("doesn't hold"));
    assert!(
        of(Some(&almost_nothing("CONICAL")), &split)
            .unwrap_err()
            .contains("CONICAL")
    );

    // The nose's drag stated in the design.
    let stated = ogive_probe().edited(|rocket| {
        rocket.stages[0].components[0].drag_override = Some(hpr_design::DragOverride {
            coefficient: 0.05,
            include_children: false,
        });
    });
    let build_stated = |_air: Air| stated.build();
    assert_eq!(
        flight(
            &build_stated,
            &fly,
            &probe.flight,
            Some(&ogive),
            &split,
            openrocket_m
        ),
        skipped("stated")
    );

    // A nose on each of three pods beside the airframe: the flight still passes Mach 1, and it
    // is skipped before any column is read.
    let several = ogive_probe().edited(|rocket| {
        let tube = rocket.stages[0]
            .components
            .iter_mut()
            .find(|component| matches!(component.part, Part::BodyTube(_)))
            .unwrap();
        tube.children.push(pod_set());
    });
    let (_, noses) = tangent_ogive_nose(&several.build().unwrap().assembly().layout).unwrap();
    // The layout lists a pod's nose once, for its three copies.
    assert_eq!(noses, 2);
    assert!(max_mach(&several.build().unwrap()).unwrap() > 1.0);
    let build_several = |_air: Air| several.build();
    assert_eq!(
        flight(
            &build_several,
            &fly,
            &probe.flight,
            Some(&ogive),
            &split,
            openrocket_m
        ),
        skipped("several_noses")
    );

    // The cone, and the ogive on a flight that stays below Mach 1, are outside the line.
    let cone = mach_probe();
    let build_cone = |_air: Air| cone.build();
    assert_eq!(
        flight(&build_cone, &fly, &cone.flight, None, &split, openrocket_m),
        Ok(None)
    );
    let slow = public_flights().remove(0).edited(|rocket| {
        let Part::NoseCone(nose) = &mut rocket.stages[0].components[0].part else {
            panic!("the design opens with its nose");
        };
        nose.shape = NoseShape::Ogive { radius_ratio: 1.0 };
    });
    assert!(max_mach(&slow.build().unwrap()).unwrap() < 1.0);
    let build_slow = |_air: Air| slow.build();
    assert_eq!(
        flight(&build_slow, &fly, &slow.flight, None, &split, openrocket_m),
        Ok(None)
    );
}

/// The public `pods-bodies-3` design.
fn pods_bodies_3() -> hpr_design::Rocket {
    let root = crate::ork::root().unwrap();
    let bytes =
        std::fs::read(root.join("validation/fixtures/ork/pod-flights/pods-bodies-3.ork")).unwrap();
    ork::design(&ork::read(&bytes).unwrap().value).value.rocket
}

/// That design's set of three pods, each with a nose cone, as it hangs on its tube.
fn pod_set() -> hpr_design::Component {
    fn find(components: &[hpr_design::Component]) -> Option<hpr_design::Component> {
        components.iter().find_map(|component| {
            if matches!(component.part, Part::PodSet(_)) {
                Some(component.clone())
            } else {
                find(&component.children)
            }
        })
    }
    find(&pods_bodies_3().stages[0].components).unwrap()
}

/// A design with a nose on each pod holds more than one nose cone, which the line counts.
#[test]
fn a_nose_on_each_pod_is_counted() {
    let mut rocket = pods_bodies_3();
    let Part::NoseCone(nose) = &mut rocket.stages[0].components[0].part else {
        panic!("the design opens with its nose");
    };
    nose.shape = NoseShape::Ogive { radius_ratio: 1.0 };
    let layout = rocket.layout().unwrap();
    let (_, noses) = tangent_ogive_nose(&layout).unwrap();
    assert!(noses > 1, "{noses}");
}

fn swapped(nose: f64, rest: f64, top_mach: f64) -> NoseFlight {
    NoseFlight::Swapped {
        nose,
        rest,
        drag: nose + rest,
        top_mach,
    }
}

/// The report's lines count each flight once, give sizes from three flights swapped and not from
/// two, and are held to themselves.
#[test]
fn the_lines_count_each_flight_and_give_sizes_from_three_swapped() {
    let three = [
        swapped(-2.0, 1.0, 1.02),
        swapped(-4.0, 0.5, 1.5),
        NoseFlight::Skipped(2),
        swapped(-3.0, 2.25, 1.214),
        NoseFlight::Skipped(4),
    ];
    let line = NoseLine::of(&three);
    assert_eq!((line.flights, line.swapped), (5, 3));
    let counts: Vec<usize> = line.skipped.iter().map(|t| t.flights).collect();
    assert_eq!(counts, [0, 0, 1, 0, 1]);
    let nose = line.nose.unwrap();
    assert_eq!(
        (
            nose.mean_percent,
            nose.median_percent,
            nose.mean_absolute_percent
        ),
        (-3.0, -3.0, 3.0)
    );
    assert_eq!(line.rest.unwrap().mean_percent, 1.25);
    assert_eq!(line.drag.unwrap().mean_percent, -1.75);
    // (1.02 + 1.5 + 1.214) / 3 = 1.2447, to two decimals.
    assert_eq!(line.hpr_mean_top_mach, Some(1.24));
    assert_eq!(line.check_consistent(10, 8), Ok(()));
    let page = line.to_markdown();
    assert!(page.contains("5 of the flights both simulators fly. Of those, 3 are swapped."));
    assert!(page.contains("| the nose's pressure drag above Mach 1 | -3.00% | -3.00% | 3.00% |"));
    assert!(page.contains("| OpenRocket's record holds no column for the nose | 1 |"));
    assert!(page.contains("by HPR Sim's own flights, average 1.24:"));

    // Two swapped: counted, and no sizes.
    let two = NoseLine::of(&three[..3]);
    assert_eq!((two.flights, two.swapped), (3, 2));
    assert_eq!((two.nose, two.rest, two.drag), (None, None, None));
    assert_eq!(two.hpr_mean_top_mach, None);
    assert_eq!(two.check_consistent(10, 8), Ok(()));
    let page = two.to_markdown();
    assert!(page.contains("are not given"));
    assert!(!page.contains("| mean |"));
    assert!(!page.contains("average"));
    // None at all.
    let none = NoseLine::of(&[]);
    assert_eq!(none.check_consistent(10, 8), Ok(()));
    assert!(none.to_markdown().contains("0 of the flights"));

    // What the check refuses.
    let refused = |edit: &dyn Fn(&mut NoseLine), why: &str| {
        let mut line = line.clone();
        edit(&mut line);
        let error = line.check_consistent(10, 8).unwrap_err();
        assert!(error.contains(why), "{error}");
    };
    refused(&|line| line.swapped = 4, "are not its 5 flights");
    refused(&|line| line.skipped[0].flights = 1, "are not its 5 flights");
    refused(&|line| line.skipped.swap(0, 1), "the reasons are");
    refused(&|line| line.nose = None, "sizes missing");
    refused(&|line| line.hpr_mean_top_mach = None, "sizes missing");
    refused(
        &|line| line.hpr_mean_top_mach = Some(0.99),
        "mean top Mach number",
    );
    refused(
        &|line| line.hpr_mean_top_mach = Some(f64::NAN),
        "mean top Mach number",
    );
    refused(
        &|line| line.nose.as_mut().unwrap().mean_percent = -2.9,
        "sum to",
    );
    assert!(
        line.check_consistent(4, 8)
            .unwrap_err()
            .contains("both fly")
    );
    assert!(line.check_consistent(10, 2).unwrap_err().contains("split"));
    let mut given = two.clone();
    given.nose = line.nose;
    given.rest = line.rest;
    given.drag = line.drag;
    given.hpr_mean_top_mach = line.hpr_mean_top_mach;
    assert!(
        given
            .check_consistent(10, 8)
            .unwrap_err()
            .contains("sizes given")
    );
    // The parts are not read by this module's tests of the report.
    let _ = (Parts::of, Apogees::clone);
}
