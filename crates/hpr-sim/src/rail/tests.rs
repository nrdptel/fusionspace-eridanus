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
    // rocket stands on, so the aft end is there, not at the tube's end; the rail's travel and the
    // start move with it. Before, a fin set counted only by its root chord.
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
    // 50 mm less travel on the same rail than with the tube's end at its foot.
    let shorter = before.exit_travel_m(2.44) - guides.exit_travel_m(2.44);
    assert!(
        (shorter - (tip_trailing_edge - before.aft_station_m)).abs() < 1e-12,
        "{shorter}"
    );
    // A fin set with no copies isn't there.
    let fins = assembly
        .layout
        .components
        .iter_mut()
        .find(|c| matches!(c.part, Part::FinSet(_)))
        .unwrap();
    fins.copies.clear();
    assert_eq!(Guides::of(&assembly).aft_station_m, before.aft_station_m);
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
