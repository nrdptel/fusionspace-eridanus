use hpr_core::DVec3;
use hpr_design::Part;

use super::*;
use crate::environment::Environment;
use crate::flight::{EventKind, FlightSettings, Simulation, Termination};
use crate::testing::{design, site};

#[test]
fn guides_are_the_buttons_aft_edges_and_the_aft_end() {
    let rocket = design("synthetic-54mm-three-fin");
    let assembly = rocket.assemble("i175").unwrap();
    let guides = Guides::of(&assembly);
    let row = assembly
        .layout
        .components
        .iter()
        .find(|c| matches!(c.part, Part::RailButton(_)))
        .unwrap();
    let Part::RailButton(button) = &row.part else {
        unreachable!("matched above");
    };
    assert_eq!(button.count, 2);
    let first = row.fore_station_m + button.outer_diameter_m;
    let last = first + button.spacing_m;
    let close = |got: Option<f64>, want: f64| (got.unwrap() - want).abs() < 1e-12;
    assert!(close(guides.first_guide_station_m, first));
    assert!(close(guides.last_guide_station_m, last));
    let nozzle = assembly.motors[0].nozzle_station_m();
    assert_eq!(guides.aft_station_m, assembly.layout.length_m.max(nozzle));
    assert!((guides.exit_travel_m(2.0) - (2.0 - (guides.aft_station_m - last))).abs() < 1e-12);
    assert!(guides.first_guide_exit_travel_m(2.0) < guides.exit_travel_m(2.0));
}

#[test]
fn the_aft_end_is_the_aft_most_point_of_the_outside() {
    // #1254: a swept fin whose tip's trailing edge runs past the tube's aft end is what the
    // rocket stands on, so the aft end is there, not at the nozzle exit or the tube's end; the
    // rail's travel and the start move with it. Before, only the body's length and the nozzle
    // exits counted.
    let rocket = design("synthetic-54mm-three-fin");
    let mut assembly = rocket.assemble("i175").unwrap();
    let before = Guides::of(&assembly);
    let body_aft = assembly.layout.length_m;
    let fins = assembly
        .layout
        .components
        .iter_mut()
        .find(|c| matches!(c.part, Part::FinSet(_)))
        .unwrap();
    let Part::FinSet(set) = &mut fins.part else {
        unreachable!("matched above");
    };
    // 200 mm root flush with the tube's end, 100 mm tip, swept 150 mm: the tip's trailing edge
    // sits 50 mm past the tube.
    let (root, tip, sweep) = (0.2, 0.1, 0.15);
    set.planform = hpr_design::FinPlanform::Trapezoidal {
        root_chord_m: root,
        tip_chord_m: tip,
        span_m: 0.08,
        sweep_m: sweep,
    };
    fins.fore_station_m = body_aft - root;
    fins.length_m = root;
    let nozzle = assembly
        .motors
        .iter()
        .map(|motor| motor.nozzle_station_m())
        .fold(f64::NEG_INFINITY, f64::max);
    let tip_trailing_edge = body_aft - root + sweep + tip;
    assert!(tip_trailing_edge > nozzle.max(body_aft) + 0.04);
    let guides = Guides::of(&assembly);
    assert!((guides.aft_station_m - tip_trailing_edge).abs() < 1e-12);
    // Less travel on the same rail by what the aft end moved, from the old aft end (the nozzle
    // exit, past the tube's end in this design) to the tip's trailing edge.
    assert_eq!(before.aft_station_m, nozzle);
    assert!(nozzle > body_aft);
    let shorter = before.exit_travel_m(2.44) - guides.exit_travel_m(2.44);
    assert!(
        (shorter - (tip_trailing_edge - before.aft_station_m)).abs() < 1e-12,
        "{shorter}"
    );
    // Tube fins 30 mm past the tube's end stand the rocket on their ends instead.
    let fins = assembly
        .layout
        .components
        .iter_mut()
        .find(|c| matches!(c.part, Part::FinSet(_)))
        .unwrap();
    let Part::FinSet(set) = &fins.part else {
        unreachable!("matched above");
    };
    let tubes = hpr_design::TubeFinSet {
        count: 6,
        length_m: 0.2,
        outer_radius_m: 0.012,
        thickness_m: 0.001,
        base_angle_rad: 0.0,
        material: set.material.clone(),
    };
    fins.fore_station_m = body_aft - 0.17;
    fins.length_m = tubes.length_m;
    fins.part = Part::TubeFinSet(tubes);
    let tube_ends = body_aft + 0.03;
    assert!(tube_ends > nozzle.max(body_aft) + 0.02);
    assert!((Guides::of(&assembly).aft_station_m - tube_ends).abs() < 1e-12);
    // A part with no copies isn't there.
    let fins = assembly
        .layout
        .components
        .iter_mut()
        .find(|c| matches!(c.part, Part::TubeFinSet(_)))
        .unwrap();
    fins.copies.clear();
    assert_eq!(Guides::of(&assembly).aft_station_m, before.aft_station_m);
}

#[test]
fn lugs_buttons_and_pods_past_the_tube_s_end_are_the_aft_end_too() {
    fn button_row(airframe: &mut hpr_design::Component) -> &mut hpr_design::Component {
        airframe
            .children
            .iter_mut()
            .find(|child| matches!(child.part, Part::RailButton(_)))
            .unwrap()
    }

    // #1254: every part on the outside counts, not only fins. Each is placed through the design
    // tree with its aft end 20 mm past the airframe's, which is past the nozzle exit too.
    let base = design("synthetic-54mm-three-fin");
    let assembly = base.assemble("i175").unwrap();
    let old_aft = Guides::of(&assembly).aft_station_m;
    let past = assembly.layout.length_m + 0.02;
    assert!(past > old_aft + 0.01);
    let bottom = Some(hpr_design::Position::Bottom { aft_offset_m: 0.02 });
    let aft_end_with = |edit: &dyn Fn(&mut hpr_design::Component)| {
        let mut rocket = base.clone();
        edit(&mut rocket.stages[0].components[1]);
        Guides::of(&rocket.assemble("i175").unwrap()).aft_station_m
    };

    // The button row moved aft: its last button's aft edge is the aft end (and the last guide).
    let buttons = aft_end_with(&|airframe| button_row(airframe).position = bottom);
    assert!((buttons - past).abs() < 1e-12, "{buttons}");

    // A row of two launch lugs in the buttons' place.
    let lugs = aft_end_with(&|airframe| {
        let row = button_row(airframe);
        row.position = bottom;
        row.part = Part::LaunchLug(hpr_design::LaunchLug {
            length_m: 0.03,
            outer_radius_m: 0.004,
            thickness_m: 0.0005,
            angle_rad: 0.0,
            count: 2,
            spacing_m: 0.3,
            material: hpr_design::Material::bulk("paper", 700.0),
        });
    });
    assert!((lugs - past).abs() < 1e-12, "{lugs}");

    // Two pods, each a tube 0.2 m long, hung off the airframe's aft end.
    let pods = aft_end_with(&|airframe| {
        let mut tube = airframe.clone();
        tube.id = "pod-tube".to_owned();
        tube.children = Vec::new();
        tube.part = Part::BodyTube(hpr_design::BodyTube {
            length_m: 0.2,
            outer_radius_m: 0.01,
            thickness_m: 0.001,
            material: hpr_design::Material::bulk("paper", 700.0),
        });
        let mut set = button_row(airframe).clone();
        set.id = "pods".to_owned();
        set.part = Part::PodSet(hpr_design::PodSet {
            count: 2,
            radial_offset_m: 0.04,
            angle_rad: 0.0,
        });
        set.position = bottom;
        set.children = vec![tube];
        airframe.children.push(set);
    });
    assert!((pods - past).abs() < 1e-12, "{pods}");
}

#[test]
fn rail_exit_is_when_the_last_button_leaves() {
    // Loft lesson L26: the rail had no friction or button geometry, and "the last button clears"
    // was blamed for a gap the oracle couldn't show. Here the rocket is guided until the aft edge
    // of its aft button passes the top of the rail, not its forward button (RocketPy's exit) nor
    // its aft end. Friction slows the exit.
    let fly = |friction_coefficient: f64| {
        let rail = Rail {
            length_m: 3.0,
            azimuth_rad: 0.5,
            elevation_rad: 1.3,
            roll_rad: 0.2,
            friction_coefficient,
        };
        let sim = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            Environment::standard(site()).unwrap(),
            rail,
            FlightSettings {
                max_time_s: 1.0,
                ..FlightSettings::default()
            },
        )
        .unwrap();
        let start = sim.initial_state();
        let result = sim.run(&mut ()).unwrap();
        assert_eq!(result.termination, Termination::TimeCap);
        let exit = result.event(EventKind::RailExit).unwrap().sample;
        let travel = (exit.state.position_enu_m - start.position_enu_m).dot(rail.direction_enu());
        // Guided: no rotation and no motion across the rail before the exit.
        assert_eq!(exit.state.body_rate_rad_s, DVec3::ZERO);
        let across =
            (exit.state.position_enu_m - start.position_enu_m) - rail.direction_enu() * travel;
        assert!(across.length() < 1e-9, "{across}");
        (sim.guides(), rail, travel, exit)
    };
    let (guides, rail, travel, frictionless) = fly(0.0);
    let expected = guides.exit_travel_m(rail.length_m);
    assert!((travel - expected).abs() < 1e-6, "{travel} vs {expected}");
    assert!(expected > guides.first_guide_exit_travel_m(rail.length_m) + 0.1);
    assert!(expected < rail.length_m);
    let (_, _, travel_rubbing, rubbing) = fly(0.3);
    assert!((travel_rubbing - expected).abs() < 1e-6);
    assert!(rubbing.time_s > frictionless.time_s);
    assert!(rubbing.airspeed_m_s < frictionless.airspeed_m_s);
}

/// The Valetudo with `count` rail buttons `spacing_m` apart, the first where the design's is.
fn with_buttons(count: u32, spacing_m: f64) -> hpr_design::Rocket {
    fn set(value: &mut serde_json::Value, count: u32, spacing_m: f64) -> bool {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(button) = map.get_mut("rail_button") {
                    button["count"] = count.into();
                    button["spacing_m"] = spacing_m.into();
                    return true;
                }
                map.values_mut().any(|inner| set(inner, count, spacing_m))
            }
            serde_json::Value::Array(items) => {
                items.iter_mut().any(|inner| set(inner, count, spacing_m))
            }
            _ => false,
        }
    }
    let mut rocket = serde_json::to_value(design("rocketpy-valetudo")).unwrap();
    assert!(
        set(&mut rocket, count, spacing_m),
        "the design has rail buttons"
    );
    serde_json::from_value(rocket).unwrap()
}

/// Every step's phase and the state it ends in.
#[derive(Default)]
struct Steps(Vec<(crate::Phase, f64, crate::State)>);

impl crate::Observer for Steps {
    fn step(&mut self, step: &dyn crate::FlightStep) -> Result<(), crate::SimError> {
        self.0
            .push((step.phase(), step.end_s(), step.state_at(step.end_s())));
        Ok(())
    }
}

/// The Valetudo's own rail (`validation/reports/tip-off.json`): 5.7 m at 84.7°.
fn valetudo_rail(friction_coefficient: f64) -> Rail {
    Rail {
        length_m: 5.7,
        azimuth_rad: 0.3,
        elevation_rad: 84.7_f64.to_radians(),
        roll_rad: 0.0,
        friction_coefficient,
    }
}

fn first_second(rocket: &hpr_design::Rocket, rail: Rail) -> Simulation {
    Simulation::new(
        rocket,
        "example",
        Environment::standard(site()).unwrap(),
        rail,
        FlightSettings {
            max_time_s: 1.0,
            ..FlightSettings::default()
        },
    )
    .unwrap()
}

#[test]
fn guides_name_the_guide_ahead_of_the_last_and_the_pivot() {
    let guides = |rocket: &hpr_design::Rocket| Guides::of(&rocket.assemble("example").unwrap());
    // Two buttons: the one ahead of the last is the first, and the pivot is the last one's
    // center, half a button ahead of its aft edge.
    let two = guides(&design("rocketpy-valetudo"));
    let diameter = 0.0113;
    assert_eq!(two.next_guide_station_m, two.first_guide_station_m);
    let last = two.last_guide_station_m.unwrap();
    assert!((two.pivot_station_m.unwrap() - (last - 0.5 * diameter)).abs() < 1e-12);
    assert_eq!(
        two.tip_off_travel_m(5.7),
        Some(two.first_guide_exit_travel_m(5.7))
    );
    assert!(two.tip_off_travel_m(5.7).unwrap() < two.exit_travel_m(5.7));
    // Three buttons 0.4 m apart: the one ahead of the last is the middle one, 0.4 m ahead.
    let three = guides(&with_buttons(3, 0.4));
    let first = three.first_guide_station_m.unwrap();
    assert!((three.last_guide_station_m.unwrap() - (first + 0.8)).abs() < 1e-12);
    assert!((three.next_guide_station_m.unwrap() - (first + 0.4)).abs() < 1e-12);
    assert!((three.exit_travel_m(5.7) - three.tip_off_travel_m(5.7).unwrap() - 0.4).abs() < 1e-12);
    // One button, or two at one station, hold the rocket until they leave: no such stretch.
    for one in [guides(&with_buttons(1, 0.0)), guides(&with_buttons(2, 0.0))] {
        assert_eq!(one.next_guide_station_m, None);
        assert_eq!(one.tip_off_travel_m(5.7), None);
        assert!(one.pivot_station_m.is_some());
    }
    // An older record without the two stations still reads, as a rocket with no such stretch.
    let old: Guides = serde_json::from_str(
        r#"{"aft_station_m":2.0,"first_guide_station_m":0.9,"last_guide_station_m":1.9}"#,
    )
    .unwrap();
    assert_eq!(
        (old.next_guide_station_m, old.pivot_station_m),
        (None, None)
    );
}

#[test]
fn tip_off_turns_the_rocket_about_its_last_guide_and_keeps_that_guide_on_the_rail() {
    let rocket = design("rocketpy-valetudo");
    for friction in [0.0, 0.3] {
        let rail = valetudo_rail(friction);
        let guided = first_second(&rocket, rail);
        let tipping = first_second(&rocket, rail).with_tip_off();
        assert!(tipping.tip_off() && !guided.tip_off());
        let start = tipping.initial_state();
        let guides = tipping.guides();
        let pivot = DVec3::new(0.0, 0.0, -guides.pivot_station_m.unwrap());
        let along = rail.direction_enu();
        let pivot_start = start.point_enu_m(pivot);

        // Guided, as before: no phase of its own, and no turning at the exit.
        let mut steps = Steps::default();
        let straight = guided.run(&mut steps).unwrap();
        assert!(
            steps
                .0
                .iter()
                .all(|(phase, ..)| *phase != crate::Phase::TipOff)
        );
        let straight = straight.event(EventKind::RailExit).unwrap().sample;
        assert_eq!(straight.state.body_rate_rad_s, DVec3::ZERO);

        let mut steps = Steps::default();
        let result = tipping.run(&mut steps).unwrap();
        let exit = result.event(EventKind::RailExit).unwrap().sample;
        // The phase runs from where the forward button leaves to where the last one does.
        let travel = |state: &crate::State| (state.point_enu_m(pivot) - pivot_start).dot(along);
        let phases: Vec<crate::Phase> = steps.0.iter().map(|(phase, ..)| *phase).collect();
        let first = phases
            .iter()
            .position(|p| *p == crate::Phase::TipOff)
            .unwrap();
        let last = phases
            .iter()
            .rposition(|p| *p == crate::Phase::TipOff)
            .unwrap();
        assert!(
            phases[..first]
                .iter()
                .all(|p| matches!(p, crate::Phase::Pad | crate::Phase::Rail))
        );
        assert!(
            phases[first..=last]
                .iter()
                .all(|p| *p == crate::Phase::TipOff)
        );
        assert!(phases[last + 1..].iter().all(|p| *p == crate::Phase::Free));
        let before = &steps.0[first - 1].2;
        assert!(
            (travel(before) - guides.tip_off_travel_m(rail.length_m).unwrap()).abs() < 1e-6,
            "tip-off began {} m up the rail",
            travel(before)
        );
        assert_eq!(before.body_rate_rad_s, DVec3::ZERO);
        assert!((travel(&exit.state) - guides.exit_travel_m(rail.length_m)).abs() < 1e-6);
        // Through the phase the last guide keeps to the rail's line and the roll is held.
        for (_, _, state) in &steps.0[first..=last] {
            let moved = state.point_enu_m(pivot) - pivot_start;
            let across = moved - along * moved.dot(along);
            assert!(
                across.length() < 1e-9,
                "μ = {friction}: the guide is {across} off the rail"
            );
            assert_eq!(state.body_rate_rad_s.z, 0.0);
        }
        // It leaves turning nose-down, at the degree or two a second gravity gives it over the
        // 1.154 m between its buttons, its axis a few hundredths of a degree below the rail's.
        let axis = exit.state.unit_attitude().mul_vec3(DVec3::Z);
        let turning = exit
            .state
            .unit_attitude()
            .mul_vec3(exit.state.body_rate_rad_s.cross(DVec3::Z));
        assert!(turning.z < 0.0 && axis.z < along.z);
        let rate_deg_s = exit.state.body_rate_rad_s.length().to_degrees();
        assert!(
            (1.5..3.0).contains(&rate_deg_s),
            "μ = {friction}: {rate_deg_s} deg/s"
        );
        let off_deg = axis.dot(along).clamp(-1.0, 1.0).acos().to_degrees();
        assert!(
            (0.03..0.1).contains(&off_deg),
            "μ = {friction}: {off_deg} deg"
        );
        // The same flight twice is the same flight.
        let again = tipping.run(&mut ()).unwrap();
        assert_eq!(again.final_sample, result.final_sample);
    }
}

#[test]
fn tip_off_changes_nothing_for_a_rocket_with_one_guide_or_none_to_turn_it() {
    // One button: no stretch on a last guide alone, so the flight is the guided flight, bit for
    // bit.
    let one = with_buttons(1, 0.0);
    let rail = valetudo_rail(0.0);
    let guided = first_second(&one, rail).run(&mut ()).unwrap();
    let mut steps = Steps::default();
    let asked = first_second(&one, rail)
        .with_tip_off()
        .run(&mut steps)
        .unwrap();
    assert!(
        steps
            .0
            .iter()
            .all(|(phase, ..)| *phase != crate::Phase::TipOff)
    );
    assert_eq!(asked.final_sample, guided.final_sample);
    assert_eq!(asked.events, guided.events);

    // Two buttons on a vertical rail in still air with no Earth rotation: nothing acts across
    // the rail, so the rocket pivots about nothing and leaves as straight as it was.
    let simulation = Simulation::new(
        &design("rocketpy-valetudo"),
        "example",
        crate::testing::analytic_environment(crate::testing::UniformAir::sea_level(), 9.806_65),
        Rail::vertical(5.7),
        FlightSettings {
            max_time_s: 1.0,
            ..FlightSettings::default()
        },
    )
    .unwrap()
    .with_tip_off();
    let mut steps = Steps::default();
    let result = simulation.run(&mut steps).unwrap();
    assert!(
        steps
            .0
            .iter()
            .any(|(phase, ..)| *phase == crate::Phase::TipOff)
    );
    let exit = result.event(EventKind::RailExit).unwrap().sample;
    assert!(exit.state.body_rate_rad_s.length() < 1e-12);
    let axis = exit.state.unit_attitude().mul_vec3(DVec3::Z);
    assert!((axis - DVec3::Z).length() < 1e-12);
}

#[test]
fn a_rocket_that_stops_on_its_last_guide_is_held_there() {
    // A rail far longer than the burn, 10° off the vertical: the rocket coasts to a stop on it.
    // Cut the rail so that the stop falls where only the last guide is still on it: the rocket
    // starts to pivot, stops, and is held where it stopped, its turning stopped with it.
    let rocket = design("rocketpy-valetudo");
    let leaning = |length_m: f64| Rail {
        length_m,
        azimuth_rad: 0.0,
        elevation_rad: 80.0_f64.to_radians(),
        roll_rad: 0.0,
        friction_coefficient: 0.0,
    };
    let fly = |rail: Rail, tip_off: bool| {
        let simulation = Simulation::new(
            &rocket,
            "example",
            Environment::standard(site()).unwrap(),
            rail,
            FlightSettings::default(),
        )
        .unwrap();
        let simulation = if tip_off {
            simulation.with_tip_off()
        } else {
            simulation
        };
        let start = simulation.initial_state();
        let mut steps = Steps::default();
        let result = simulation.run(&mut steps).unwrap();
        (simulation.guides(), start, steps, result)
    };
    let (guides, start, _, long) = fly(leaning(2000.0), false);
    assert_eq!(long.termination, Termination::StalledOnRail);
    let along = leaning(1.0).direction_enu();
    let stop_m = (long.final_sample.state.position_enu_m - start.position_enu_m).dot(along);
    // The stop midway between where the forward button and the last one would leave.
    let lead = guides.aft_station_m - guides.last_guide_station_m.unwrap();
    let between = guides.last_guide_station_m.unwrap() - guides.next_guide_station_m.unwrap();
    let rail = leaning(stop_m + lead + 0.5 * between);
    let (guides, start, steps, held) = fly(rail, true);
    assert!(guides.tip_off_travel_m(rail.length_m).unwrap() < stop_m);
    assert!(stop_m < guides.exit_travel_m(rail.length_m));
    assert_eq!(held.termination, Termination::StalledOnRail);
    assert!(held.event(EventKind::RailExit).is_none());
    assert!(
        steps
            .0
            .iter()
            .any(|(phase, ..)| *phase == crate::Phase::TipOff)
    );
    let state = held.final_sample.state;
    assert_eq!(state.velocity_enu_m_s, DVec3::ZERO);
    assert_eq!(state.body_rate_rad_s, DVec3::ZERO);
    // It turned nose-down before it stopped, and its last guide is still on the rail's line.
    let axis = state.unit_attitude().mul_vec3(DVec3::Z);
    assert!(axis.z < along.z);
    let pivot = DVec3::new(0.0, 0.0, -guides.pivot_station_m.unwrap());
    let moved = state.point_enu_m(pivot) - start.point_enu_m(pivot);
    assert!((moved - along * moved.dot(along)).length() < 1e-6);
    assert!(
        (moved.dot(along) - stop_m).abs() < 0.05,
        "{} vs {stop_m}",
        moved.dot(along)
    );
}

#[test]
fn a_rail_that_holds_only_the_last_guide_pivots_from_liftoff() {
    // A rail so short that the forward button starts above its top: the last guide holds the
    // rocket alone from the start, so it pivots from liftoff.
    let rocket = design("rocketpy-valetudo");
    let guides = Guides::of(&rocket.assemble("example").unwrap());
    let lead = guides.aft_station_m - guides.last_guide_station_m.unwrap();
    let rail = Rail {
        length_m: lead + 0.3,
        ..valetudo_rail(0.0)
    };
    assert!(guides.tip_off_travel_m(rail.length_m).unwrap() < 0.0);
    let mut steps = Steps::default();
    let result = first_second(&rocket, rail)
        .with_tip_off()
        .run(&mut steps)
        .unwrap();
    assert_eq!(
        result.event(EventKind::Liftoff).unwrap().sample.phase,
        crate::Phase::TipOff
    );
    assert!(
        steps
            .0
            .iter()
            .all(|(phase, ..)| *phase != crate::Phase::Rail)
    );
    let exit = result.event(EventKind::RailExit).unwrap().sample;
    assert!(exit.state.body_rate_rad_s.length() > 0.0);
}
