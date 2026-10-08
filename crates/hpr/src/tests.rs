//! The builder against the design tree it makes, and its refusals.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "tests state their expectations by unwrapping and panicking"
)]

use hpr_design::{
    AutoDimension, BodyTube, Component, Configuration, DesignError, FinCrossSection, FinPlanform,
    FinSet, Ignition, InnerTube, MassComponent, MotorMount, MountedMotor, NoseCone, Overrides,
    Packing, Part, ReferenceDiameter, Shoulder, Stage, Wall,
};
use hpr_motor::{Delay, MotorError};
use hpr_sim::{EventKind, FlightSettings, Rail, Simulation};

use crate::rocket::{Fins, Mass, MotorTube, Nose, Transition, Tube, material};
use crate::{
    CanopyType, Device, DeviceDrag, Environment, Error, Flight, Motor, NoseShape, Order, Position,
    Rocket, Trigger,
};

/// The rocket of the example `own_rocket` in `hpr-sim`, built part by part with the builder.
fn built() -> Rocket {
    let mut rocket = Rocket::new("My 54 mm rocket", 0.0563).unwrap();
    rocket
        .add_nose(
            Nose::hollow(
                NoseShape::Ogive { radius_ratio: 1.0 },
                0.22,
                0.0015,
                material("abs").unwrap(),
            )
            .with_capped_shoulder(0.06, 0.0015),
        )
        .unwrap()
        .add_tube(Tube::new(0.9, 0.00115, material("kraft_phenolic").unwrap()))
        .unwrap()
        .add_motor_tube(
            MotorTube::new(0.2, 0.029, 0.001, material("kraft_phenolic").unwrap())
                .with_overhang_m(0.005),
        )
        .unwrap()
        .add_fins(
            Fins::new(
                3,
                trapezoid([0.1, 0.04, 0.045, 0.05]),
                0.003175,
                material("birch_plywood").unwrap(),
            )
            .with_cross_section(FinCrossSection::Rounded),
        )
        .unwrap()
        .add_mass(Mass::new(0.2, Position::Top { aft_offset_m: 0.07 }).packed(0.15, 0.05))
        .unwrap()
        .set_motor(
            Motor::from_catalog("168H54-10A")
                .unwrap()
                .with_delay_s(10.0)
                .unwrap(),
        )
        .unwrap()
        .add_parachute(parachute());
    rocket
}

/// A trapezoidal planform from its root chord, tip chord, span and sweep, m.
fn trapezoid([root_chord_m, tip_chord_m, span_m, sweep_m]: [f64; 4]) -> FinPlanform {
    FinPlanform::Trapezoidal {
        root_chord_m,
        tip_chord_m,
        span_m,
        sweep_m,
    }
}

/// The example's parachute, opened by the motor's ejection charge.
fn parachute() -> Device {
    Device::new(
        "parachute",
        DeviceDrag::canopy(CanopyType::FlatCircular, 0.9),
        Trigger::MotorDelay { motor: 0 },
    )
}

/// A node of the design tree, as the example makes one.
fn component(id: &str, part: Part, position: Option<Position>) -> Component {
    Component {
        id: id.to_owned(),
        name: String::new(),
        part,
        position,
        auto: Vec::new(),
        motor_mount: None,
        finish: None,
        overrides: Overrides::default(),
        overrides_include_children: false,
        drag_override: None,
        children: Vec::new(),
    }
}

/// The same rocket as the example `own_rocket` builds it, as struct literals, with its ids.
fn by_hand() -> hpr_design::Rocket {
    let mut nose = component(
        "nose",
        Part::NoseCone(NoseCone {
            shape: NoseShape::Ogive { radius_ratio: 1.0 },
            length_m: 0.22,
            base_radius_m: 0.0,
            wall: Wall::Shell {
                thickness_m: 0.0015,
            },
            shoulder: Some(Shoulder {
                length_m: 0.06,
                outer_radius_m: 0.0,
                thickness_m: 0.0015,
                capped: true,
            }),
            material: material("abs").unwrap(),
        }),
        None,
    );
    nose.auto = vec![AutoDimension::BaseRadius, AutoDimension::ShoulderRadius];
    let mut airframe = component(
        "airframe",
        Part::BodyTube(BodyTube {
            length_m: 0.9,
            outer_radius_m: 0.02815,
            thickness_m: 0.00115,
            material: material("kraft_phenolic").unwrap(),
        }),
        None,
    );
    let mut mount = component(
        "motor-mount",
        Part::InnerTube(InnerTube {
            length_m: 0.2,
            outer_radius_m: 0.0155,
            thickness_m: 0.001,
            radial_offset_m: 0.0,
            angle_rad: 0.0,
            material: material("kraft_phenolic").unwrap(),
            cluster_m: Vec::new(),
        }),
        Some(Position::Bottom { aft_offset_m: 0.0 }),
    );
    mount.motor_mount = Some(MotorMount { overhang_m: 0.005 });
    let fins = component(
        "fins",
        Part::FinSet(FinSet {
            count: 3,
            planform: FinPlanform::Trapezoidal {
                root_chord_m: 0.1,
                tip_chord_m: 0.04,
                span_m: 0.045,
                sweep_m: 0.05,
            },
            thickness_m: 0.003175,
            cross_section: FinCrossSection::Rounded,
            tab: None,
            fillet: None,
            cant_rad: 0.0,
            base_angle_rad: 0.0,
            material: material("birch_plywood").unwrap(),
        }),
        Some(Position::Bottom { aft_offset_m: 0.0 }),
    );
    let bay = component(
        "recovery-bay",
        Part::MassComponent(MassComponent {
            mass_kg: 0.2,
            packing: Packing {
                length_m: 0.15,
                radius_m: 0.025,
                radial_offset_m: 0.0,
                angle_rad: 0.0,
            },
        }),
        Some(Position::Top { aft_offset_m: 0.07 }),
    );
    airframe.children = vec![mount, fins, bay];
    let catalog = hpr_motor::Catalog::bundled().unwrap();
    let entry = catalog.find("168H54-10A").next().unwrap();
    hpr_design::Rocket {
        name: "My 54 mm rocket".to_owned(),
        stages: vec![Stage {
            id: "sustainer".to_owned(),
            name: String::new(),
            components: vec![nose, airframe],
            overrides: Overrides::default(),
            drag_override: None,
            parallel: None,
        }],
        reference_diameter: ReferenceDiameter::Maximum {},
        configurations: vec![Configuration {
            id: "h54".to_owned(),
            name: String::new(),
            motors: vec![MountedMotor {
                mount: "motor-mount".to_owned(),
                designation: entry.designation.clone(),
                diameter_m: entry.diameter_mm / 1000.0,
                length_m: entry.length_mm / 1000.0,
                motor: entry.bundled_motor().unwrap(),
                delay: Some(Delay::Seconds(10.0)),
                ignition: Ignition::Launch,
                failed_tubes: Vec::new(),
            }],
        }],
    }
}

/// Spaceport America's site, 1,400 m up, in calm air.
fn environment() -> Environment {
    Environment::new(32.99, -106.97, 1400.0).unwrap()
}

/// The builder's rocket is the example's, placed and flown: the same mass properties through the
/// burn, the same center of pressure, and the same flight, bit for bit. Only the ids differ, and
/// the nose's base radius, which the example leaves automatic and the builder gives.
#[test]
fn the_builder_makes_the_rocket_own_rocket_builds_by_hand() {
    let rocket = built();
    let hand = by_hand();
    let (a, b) = (rocket.assemble().unwrap(), hand.assemble("h54").unwrap());
    for t in [0.0, 0.5, 1.0, 2.0, 3.0, 10.0] {
        assert_eq!(a.mass_properties(t), b.mass_properties(t), "t = {t} s");
    }
    assert_eq!(a.dry_mass_properties(), b.dry_mass_properties());
    assert_eq!(a.layout.length_m, b.layout.length_m);
    assert_eq!(a.layout.reference_diameter_m, b.layout.reference_diameter_m);

    // The example prints this margin: 1.92 calibres at liftoff, CP 0.779 m from the nose.
    let margin = rocket.margin(0.0, 0.3).unwrap();
    assert_eq!(format!("{:.2}", margin.margin_cal.unwrap()), "1.92");
    assert_eq!(format!("{:.3}", margin.cp_station_m.unwrap()), "0.779");

    let flight = Flight::builder(&rocket, &environment(), 1.8).fly().unwrap();
    let by_hand = Simulation::new(
        &hand,
        "h54",
        environment().sim().clone(),
        Rail::vertical(1.8),
        FlightSettings::default(),
    )
    .unwrap()
    .with_recovery(vec![parachute()])
    .unwrap()
    .run(&mut ())
    .unwrap();
    assert_eq!(flight.result(), &by_hand);
    // And the example's printed apogee, 1144.5 m at 13.92 s.
    assert_eq!(format!("{:.1}", flight.apogee_m().unwrap()), "1144.5");
    assert_eq!(format!("{:.2}", flight.apogee_time_s().unwrap()), "13.92");
    let apogee = by_hand.event(EventKind::Apogee).unwrap().sample;
    assert_eq!(flight.apogee_m(), Some(apogee.height_above_ground_m));
}

/// A separation goes to the flight, and is refused, not dropped, where the flight can't fly it:
/// a one-stage rocket has no stage aft of the split. A Monte Carlo run's inputs carry it, and
/// their simulation refuses it the same way.
#[test]
fn a_separation_reaches_the_flight_or_is_refused() {
    let rocket = built();
    let environment = environment();
    let separation = hpr_sim::Separation::new(Trigger::Apogee, 0);
    let builder = Flight::builder(&rocket, &environment, 1.8).separation(separation);
    assert!(
        matches!(
            builder.simulation(),
            Err(Error::Sim(hpr_sim::SimError::Domain { what, value }))
                if what.contains("no stage aft of it") && value == 0.0
        ),
        "{:?}",
        builder.simulation().err()
    );
    let inputs = builder.inputs().unwrap();
    assert_eq!(inputs.separations, [separation]);
    assert!(
        matches!(
            inputs.simulation(),
            Err(hpr_sim::SimError::Domain { what, value })
                if what.contains("no stage aft of it") && value == 0.0
        ),
        "{:?}",
        inputs.simulation().err()
    );
    // Without one, the inputs carry none.
    let whole = Flight::builder(&rocket, &environment, 1.8)
        .inputs()
        .unwrap();
    assert!(whole.separations.is_empty());
}

/// The design the builder makes: its ids, the automatic shoulder radius, the motor in its tube.
#[test]
fn the_builder_names_its_parts_and_configuration() {
    let rocket = built();
    let design = rocket.design();
    let stage = &design.stages[0];
    let ids =
        |components: &[Component]| components.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
    assert_eq!(ids(&stage.components), ["nose", "tube"]);
    assert_eq!(
        ids(&stage.components[1].children),
        ["motor-tube", "fins", "mass"]
    );
    assert_eq!(stage.components[0].auto, [AutoDimension::ShoulderRadius]);
    assert_eq!(rocket.configuration_id(), Some("168H54-10A"));
    let motors = &design.configurations[0].motors;
    assert_eq!(motors.len(), 1);
    assert_eq!(motors[0].mount, "motor-tube");
    assert_eq!(motors[0].delay, Some(Delay::Seconds(10.0)));

    // A second part of a kind gets a numbered id; a transition takes the diameter before it.
    let mut stepped = Rocket::new("Stepped", 0.1).unwrap();
    let paper = material("kraft_phenolic").unwrap();
    stepped
        .add_tube(Tube::new(0.5, 0.002, paper.clone()))
        .unwrap()
        .add_transition(Transition::conical(0.1, 0.05, 0.002, paper.clone()))
        .unwrap()
        .add_tube(Tube::new(0.5, 0.002, paper))
        .unwrap();
    let components = &stepped.design().stages[0].components;
    assert_eq!(ids(components), ["tube", "transition", "tube-2"]);
    let Part::Transition(transition) = &components[1].part else {
        panic!("not a transition: {:?}", components[1].part);
    };
    assert_eq!(
        (transition.fore_radius_m, transition.aft_radius_m),
        (0.05, 0.025)
    );
    // The tube behind it takes its aft diameter.
    let Part::BodyTube(tube) = &components[2].part else {
        panic!("not a tube: {:?}", components[2].part);
    };
    assert_eq!(tube.outer_radius_m, 0.025);
}

/// Parts in an order the tree can't take, and a rocket read from a design, are refused.
#[test]
fn parts_out_of_order_are_refused() {
    let paper = material("kraft_phenolic").unwrap();
    let fins = Fins::new(3, trapezoid([0.1, 0.05, 0.05, 0.03]), 0.003, paper.clone());
    let mut rocket = Rocket::new("Out of order", 0.05).unwrap();
    assert!(matches!(
        rocket.add_fins(fins.clone()),
        Err(Error::Order(Order::NoTube))
    ));
    assert!(matches!(
        rocket.add_transition(Transition::conical(0.1, 0.03, 0.002, paper.clone())),
        Err(Error::Order(Order::NothingBeforeTransition))
    ));
    let motor = Motor::from_catalog("F52C").unwrap();
    assert!(matches!(
        rocket.set_motor(motor.clone()),
        Err(Error::Order(Order::NoMotorTube))
    ));
    rocket
        .add_tube(Tube::new(0.5, 0.001, paper.clone()))
        .unwrap();
    let ogive = NoseShape::Ogive { radius_ratio: 1.0 };
    assert!(matches!(
        rocket.add_nose(Nose::solid(ogive, 0.1, paper.clone())),
        Err(Error::Order(Order::NoseNotFirst))
    ));
    let tube = MotorTube::new(0.1, 0.029, 0.001, paper.clone());
    rocket.add_motor_tube(tube.clone()).unwrap();
    assert!(matches!(
        rocket.add_motor_tube(tube),
        Err(Error::Order(Order::SecondMotorTube))
    ));
    // No motor yet: nothing to fly or to weigh.
    assert!(matches!(rocket.assemble(), Err(Error::NoMotor)));
    assert!(matches!(
        Flight::builder(&rocket, &environment(), 1.0).fly(),
        Err(Error::NoMotor)
    ));

    let mut read = Rocket::from_design(by_hand(), "h54").unwrap();
    assert!(matches!(
        read.add_tube(Tube::new(0.5, 0.001, paper)),
        Err(Error::Order(Order::ReadFromDesign))
    ));
    assert!(matches!(
        Rocket::from_design(by_hand(), "j350"),
        Err(Error::NoSuchConfiguration(id)) if id == "j350"
    ));
    // A read design flies as it is, with the recovery devices added to it.
    read.add_parachute(parachute());
    let flight = Flight::builder(&read, &environment(), 1.8).fly().unwrap();
    assert_eq!(format!("{:.1}", flight.apogee_m().unwrap()), "1144.5");
}

/// Motors from the catalog and from a `.eng` file, and the names that find none or several.
#[test]
fn motors_come_from_the_catalog_or_a_file() {
    let catalog = Motor::from_catalog("h54").unwrap();
    assert_eq!(catalog.designation(), "168H54-10A");
    assert_eq!((catalog.diameter_m(), catalog.length_m()), (0.029, 0.187));
    assert_eq!(catalog.delay(), None);
    assert!(matches!(
        Motor::from_catalog("Z9000"),
        Err(Error::NoSuchMotor(name)) if name == "Z9000"
    ));
    assert!(matches!(
        catalog.clone().with_delay_s(-1.0),
        Err(Error::Domain { what: "motor delay, s", value }) if value == -1.0
    ));
    assert_eq!(
        catalog.clone().with_delay(Delay::Plugged).unwrap().delay(),
        Some(Delay::Plugged)
    );
    assert!(matches!(
        catalog.clone().with_delay(Delay::Seconds(f64::NAN)),
        Err(Error::Domain {
            what: "motor delay, s",
            ..
        })
    ));
    // A common name two motors share is refused, both listed, not guessed.
    match Motor::from_catalog("I175") {
        Err(Error::AmbiguousMotor { name, candidates }) => {
            assert_eq!(name, "I175");
            assert_eq!(candidates.len(), 2, "{candidates:?}");
            assert_eq!(candidates, ["I175WS (AeroTech)", "411I175-14A (Cesaroni)"]);
            // Each is listed by a designation that finds it alone.
            for candidate in &candidates {
                let designation = candidate.split(' ').next().unwrap();
                assert_eq!(
                    Motor::from_catalog(designation).unwrap().designation(),
                    designation
                );
            }
        }
        other => panic!("expected an ambiguous name, got {other:?}"),
    }
    // The F15's curve is a RockSim `.rse` file.
    let f15 = Motor::from_catalog("F15").unwrap();
    assert_eq!((f15.diameter_m(), f15.length_m()), (0.029, 0.114));

    let text = include_str!("../../hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000863.eng");
    // Its header: `I377CT 38 292 8-18 0.25 0.56 Loki`, millimeters read as meters.
    let file = Motor::from_eng(text).unwrap();
    assert_eq!(file.designation(), "I377CT");
    assert_eq!((file.diameter_m(), file.length_m()), (0.038, 0.292));
    assert_eq!(file.delay(), None);
    let two = format!("{text}\n{text}");
    assert!(matches!(Motor::from_eng(&two), Err(Error::MotorCount(2))));

    let text = include_str!("../../hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000719.rse");
    // Its engine: `code="H170M" dia="38." len="191." initWt="330." propWt="182.5"`, sizes in
    // millimeters and masses in grams, converted to meters and kilograms.
    let file = Motor::from_rse(text).unwrap();
    assert_eq!(file.designation(), "H170M");
    assert_eq!((file.diameter_m(), file.length_m()), (0.038, 0.191));
    assert_eq!(file.delay(), None);
    let solid = file.solid_motor();
    let propellant = solid.propellant_initial_mass_kg();
    assert!((propellant - 0.1825).abs() < 1e-12, "{propellant}");
    let loaded = propellant + solid.dry().mass_kg;
    assert!((loaded - 0.330).abs() < 1e-12, "{loaded}");
    let hybrid = text.replacen("Type=\"reloadable\"", "Type=\" Hybrid\"", 1);
    assert_ne!(hybrid, text);
    assert!(matches!(
        Motor::from_rse(&hybrid),
        Err(Error::Motor(MotorError::Inconsistent(message))) if message.contains("hybrid")
    ));
    let engine = text.find("<engine ").unwrap();
    let end = text.find("</engine>").unwrap() + "</engine>".len();
    let two = format!("{}{}{}", &text[..end], &text[engine..end], &text[end..]);
    assert!(matches!(Motor::from_rse(&two), Err(Error::MotorCount(2))));
    assert!(matches!(
        Motor::new(" ", catalog.solid_motor().clone(), 0.029, 0.1),
        Err(Error::EmptyDesignation)
    ));
    assert!(matches!(
        Motor::new("x", catalog.solid_motor().clone(), 0.0, 0.1),
        Err(Error::Domain { what: "motor diameter, m", value }) if value == 0.0
    ));
}

/// The environment's site and wind, and what it refuses.
#[test]
fn the_environment_takes_a_site_and_a_wind() {
    let calm = environment();
    let site = calm.sim().site();
    assert_eq!(site.height_m, 1400.0);
    assert_eq!(site.latitude_rad, 32.99_f64.to_radians());
    let windy = calm.clone().with_constant_wind(5.0, 270.0).unwrap();
    // A west wind blows toward the east: +x in the launch frame.
    let wind = windy.sim().wind.wind(1500.0).unwrap().velocity_enu_m_s;
    assert!(
        (wind.x - 5.0).abs() < 1e-12 && wind.y.abs() < 1e-12,
        "{wind:?}"
    );
    assert!(matches!(
        Environment::new(91.0, 0.0, 0.0),
        Err(Error::Core(_))
    ));
    assert!(matches!(
        calm.clone().with_constant_wind(5.0, f64::NAN),
        Err(Error::Domain {
            what: "wind direction, degrees",
            ..
        })
    ));
    assert!(matches!(
        calm.with_constant_wind(-1.0, 0.0),
        Err(Error::Atmos(_))
    ));
}

/// A leaning rail: the rocket drifts the way it leans, and a flat rail is refused.
#[test]
fn the_rail_leans_where_it_is_headed() {
    let rocket = built();
    let environment = environment();
    let east = Flight::builder(&rocket, &environment, 1.8)
        .inclination_deg(80.0)
        .heading_deg(90.0)
        .fly()
        .unwrap();
    // East, and a little south: the Earth's rotation turns a flight to its right in the northern
    // hemisphere (0.16 m in 337 m here; the plumb line's curve alone gives 0.001 m).
    let apogee = east.result().event(EventKind::Apogee).unwrap().sample;
    let (east_m, north_m) = (apogee.cg_enu_m.x, apogee.cg_enu_m.y);
    assert!(
        east_m > 100.0 && north_m < -0.1 && north_m > -1e-3 * east_m,
        "{:?}",
        apogee.cg_enu_m
    );
    // Refused in the degrees they were given: flat, and past the vertical.
    for inclination_deg in [0.0, 95.0, f64::NAN] {
        let refused = Flight::builder(&rocket, &environment, 1.8)
            .inclination_deg(inclination_deg)
            .fly();
        assert!(
            matches!(
                refused,
                Err(Error::Domain { what: "rail inclination, degrees above the horizon", value })
                    if value.to_bits() == inclination_deg.to_bits()
            ),
            "{inclination_deg}: {refused:?}"
        );
    }
    // A whole rail keeps its own angles, exactly, unless the degrees are set too.
    let rail = Rail {
        azimuth_rad: 0.3,
        elevation_rad: 1.4,
        ..Rail::vertical(1.8)
    };
    let launch = Flight::builder(&rocket, &environment, 1.0).rail(rail);
    assert_eq!(launch.simulation().unwrap().rail(), rail);
    let steeper = launch.inclination_deg(89.0).simulation().unwrap();
    assert_eq!(steeper.rail().elevation_rad, 89.0_f64.to_radians());
    assert_eq!(steeper.rail().azimuth_rad, 0.3);
}

/// A packed mass's position places its packing's end, so packing it moves its center by half its
/// length (the guide says so); placed by its middle, it doesn't move.
#[test]
fn packing_a_mass_moves_its_center_unless_placed_by_its_middle() {
    // A tube, its motor, and the bay: the rocket's mass and the `z` of its center of gravity.
    let with_bay = |bay: Mass| {
        let paper = material("kraft_phenolic").unwrap();
        let mut rocket = Rocket::new("Bay", 0.0563).unwrap();
        rocket
            .add_tube(Tube::new(0.9, 0.00115, paper.clone()))
            .unwrap()
            .add_motor_tube(MotorTube::new(0.2, 0.029, 0.001, paper))
            .unwrap()
            .add_mass(bay)
            .unwrap()
            .set_motor(Motor::from_catalog("H54").unwrap())
            .unwrap();
        let properties = rocket.mass_properties(0.0).unwrap();
        (properties.mass_kg, properties.cg_m.z)
    };
    let top = Position::Top { aft_offset_m: 0.07 };
    let (mass_kg, point_z) = with_bay(Mass::new(0.2, top));
    let (_, packed_z) = with_bay(Mass::new(0.2, top).packed(0.15, 0.05));
    // The bay's center moves 0.075 m aft, and the rocket's by 0.2 × 0.075 / its mass.
    let shift_m = 0.2 * 0.075 / mass_kg;
    assert!(
        ((point_z - packed_z) - shift_m).abs() < 1e-12,
        "{} against {shift_m}",
        point_z - packed_z
    );
    let middle = Position::Middle { aft_offset_m: 0.0 };
    let (_, point_z) = with_bay(Mass::new(0.2, middle));
    let (_, packed_z) = with_bay(Mass::new(0.2, middle).packed(0.15, 0.05));
    assert!((point_z - packed_z).abs() < 1e-15, "{point_z} {packed_z}");
}

/// The builder refuses a fin set of more than [`FinSet::MAX_COUNT`] fins by its count (issue
/// #255: four billion fins were taken, and the margin never came back), and takes the bound. On
/// the example's 54 mm rocket with the issue's 50/20/30/20 mm fins, the margin of 8 fins is a
/// number and of 64 is the aerodynamics' own refusal, both returned at once.
#[test]
fn a_fin_count_over_the_bound_is_refused_and_the_margin_returns() {
    let planform = trapezoid([0.05, 0.02, 0.03, 0.02]);
    let fins = |count| Fins::new(count, planform.clone(), 0.003, material("abs").unwrap());
    let mut rocket = Rocket::new("R", 0.0563).unwrap();
    rocket
        .add_nose(Nose::solid(
            NoseShape::Ogive { radius_ratio: 1.0 },
            0.22,
            material("abs").unwrap(),
        ))
        .unwrap()
        .add_tube(Tube::new(0.9, 0.001, material("abs").unwrap()))
        .unwrap();
    for count in [FinSet::MAX_COUNT + 1, 4_000_000_000] {
        match rocket.add_fins(fins(count)) {
            Err(Error::Design(DesignError::Domain { what, value })) => {
                assert_eq!(what, "fin count (1 to 64)");
                assert_eq!(value, f64::from(count));
            }
            other => panic!("{count} fins: {other:?}"),
        }
    }
    rocket.add_fins(fins(FinSet::MAX_COUNT)).unwrap();

    let with_fins = |count| {
        let mut design = built().design().clone();
        if let Part::FinSet(set) = &mut design.stages[0].components[1].children[1].part {
            set.count = count;
            set.planform = planform.clone();
        } else {
            panic!("the example's fins moved");
        }
        Rocket::from_design(design, "168H54-10A").unwrap()
    };
    let eight = with_fins(8).static_margin_cal(0.0, 0.3).unwrap();
    assert!(eight.is_some_and(f64::is_finite), "{eight:?}");
    match with_fins(FinSet::MAX_COUNT).static_margin_cal(0.0, 0.3) {
        Err(Error::Aero(hpr_aero::AeroError::InComponent { id, source })) => {
            assert_eq!(id, "fins");
            match *source {
                hpr_aero::AeroError::Domain { what, value } => {
                    assert_eq!(what, "fin count (1 to 8 have a normal-force model)");
                    assert_eq!(value, 64.0);
                }
                other => panic!("64 fins: {other:?}"),
            }
        }
        other => panic!("64 fins: {other:?}"),
    }
}

/// Weighing a rocket runs the checks a flight runs: a motor wider than its tube is refused by
/// both, not weighed by one and refused by the other.
#[test]
fn weighing_refuses_what_flying_refuses() {
    let mut rocket = built();
    rocket
        .set_motor(Motor::from_catalog("K400C").unwrap())
        .unwrap();
    let wider = |findings: &[hpr_design::Finding]| {
        findings
            .iter()
            .any(|finding| matches!(finding, hpr_design::Finding::MotorWiderThanMount { .. }))
    };
    match rocket.mass_properties(0.0) {
        Err(Error::DesignChecks(findings)) => assert!(wider(&findings), "{findings:?}"),
        other => panic!("expected the design checks, got {other:?}"),
    }
    assert!(matches!(
        rocket.margin(0.0, 0.3),
        Err(Error::DesignChecks(_))
    ));
    match Flight::builder(&rocket, &environment(), 1.8).fly() {
        Err(Error::Sim(hpr_sim::SimError::DesignChecks(findings))) => {
            assert!(wider(&findings), "{findings:?}");
        }
        other => panic!("expected the design checks, got {other:?}"),
    }
    assert!(matches!(
        built().mass_properties(-1.0),
        Err(Error::Domain {
            what: "time, s",
            ..
        })
    ));
}

/// `fly_with` shows the observer every step: a recorder's last row is the flight's end.
#[test]
fn an_observer_sees_the_flight() {
    use hpr_sim::{Channel, Recorder};
    let rocket = built();
    let mut recorder = Recorder::new(vec![Channel::Time, Channel::Mass], None).unwrap();
    let flight = Flight::builder(&rocket, &environment(), 1.8)
        .fly_with(&mut recorder)
        .unwrap();
    let last = recorder.rows().last().unwrap();
    let end = flight.result().final_sample;
    assert_eq!((last[0], last[1]), (end.time_s, end.mass_kg));
    assert_eq!(
        flight,
        Flight::builder(&rocket, &environment(), 1.8).fly().unwrap()
    );
    // A flight's record reads back as the same flight.
    let text = serde_json::to_string(&flight).unwrap();
    assert_eq!(serde_json::from_str::<Flight>(&text).unwrap(), flight);
    // A record saved before the drag issue warnings were kept reads back, with none.
    let mut old: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(
        old.as_object_mut()
            .unwrap()
            .remove("issue_warnings")
            .is_some()
    );
    let read = serde_json::from_value::<Flight>(old).unwrap();
    assert!(read.issue_warnings().is_empty());
    assert_eq!(read.summary(), flight.summary());
}

/// hpr's own drag buildup, handed back through the drag-model trait unchanged.
#[derive(Debug)]
struct HprsOwn;

impl hpr_aero::DragModel for HprsOwn {
    fn zero_lift_drag(&self, query: &hpr_aero::DragQuery<'_>) -> Result<f64, hpr_aero::AeroError> {
        Ok(query.buildup()?.zero_lift_coefficient)
    }
}

/// The same drag coefficient at every flow.
#[derive(Debug)]
struct ConstantDrag(f64);

impl hpr_aero::DragModel for ConstantDrag {
    fn zero_lift_drag(&self, _query: &hpr_aero::DragQuery<'_>) -> Result<f64, hpr_aero::AeroError> {
        Ok(self.0)
    }
}

/// Whether `flight`, flown on a drag of its own, is `own`, flown on hpr's, bit for bit: the same
/// result, summary and warnings, less hpr's drag's #18 (fully turbulent friction), which a drag
/// of the flight's own never meets.
fn same_flight_less_18(flight: &Flight, own: &Flight) {
    assert_eq!(flight.result(), own.result());
    assert_eq!(flight.summary(), own.summary());
    let mut warnings = own.issue_warnings().to_vec();
    let at = warnings.iter().position(|warning| warning.number() == 18);
    assert!(at.is_some(), "hpr's drag meets #18");
    warnings.retain(|warning| warning.number() != 18);
    assert_eq!(flight.issue_warnings(), warnings.as_slice());
}

/// A drag model is flown in hpr's place: one handing back hpr's own drag flies the same flight,
/// bit for bit, and a constant one flies as the same constant table.
#[test]
fn a_drag_model_is_flown_in_place_of_hprs_drag() {
    let rocket = built();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    let own = launch.fly().unwrap();
    same_flight_less_18(&launch.clone().drag_model(HprsOwn).fly().unwrap(), &own);

    let table = hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", None).unwrap();
    let by_table = launch
        .simulation()
        .unwrap()
        .with_drag_table(table)
        .run(&mut ())
        .unwrap();
    let by_model = launch.clone().drag_model(ConstantDrag(0.5)).fly().unwrap();
    assert_eq!(by_model.result(), &by_table);
    assert_ne!(by_model.apogee_m(), own.apogee_m());

    // More drag, a lower apogee; the last model set is the one flown.
    let draggier = launch.clone().drag_model(ConstantDrag(0.9)).fly().unwrap();
    assert!(draggier.apogee_m().unwrap() < by_model.apogee_m().unwrap());
    let last = launch
        .clone()
        .drag_model(ConstantDrag(0.9))
        .drag_model(HprsOwn)
        .fly()
        .unwrap();
    same_flight_less_18(&last, &own);

    // One shared model flies two builders' flights the same.
    let shared: std::sync::Arc<dyn hpr_aero::DragModel> = std::sync::Arc::new(ConstantDrag(0.5));
    let other = Flight::builder(&rocket, &environment, 1.8);
    assert_eq!(
        launch
            .clone()
            .shared_drag_model(shared.clone())
            .fly()
            .unwrap(),
        other.shared_drag_model(shared).fly().unwrap()
    );
    assert_eq!(
        launch
            .clone()
            .shared_drag_model(std::sync::Arc::new(ConstantDrag(0.5)))
            .fly()
            .unwrap(),
        by_model
    );

    // A model that answers nonsense stops the flight, named.
    let error = launch.drag_model(ConstantDrag(-1.0)).fly().unwrap_err();
    assert!(
        matches!(
            &error,
            Error::Sim(hpr_sim::SimError::Aero(hpr_aero::AeroError::Domain { what, value }))
                if *what == "zero-lift drag coefficient from a drag model" && *value == -1.0
        ),
        "{error:?}"
    );
}

/// A drag table is flown in hpr's place, as the simulation's own table is: a constant table
/// flies as the same constant model, its power-on curve only while the motor burns, and one on
/// another reference diameter rescaled by the areas. The last model or table set is flown.
#[test]
fn a_drag_table_is_flown_in_place_of_hprs_drag() {
    let rocket = built();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    let own = launch.fly().unwrap();
    let half = hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", None).unwrap();
    let by_table = launch.clone().drag_table(half.clone()).fly().unwrap();
    assert_eq!(
        by_table,
        launch.clone().drag_model(ConstantDrag(0.5)).fly().unwrap()
    );
    assert_eq!(
        by_table.result(),
        &launch
            .simulation()
            .unwrap()
            .with_drag_table(half.clone())
            .run(&mut ())
            .unwrap()
    );

    // Whichever of a model and a table is set last is flown.
    assert_eq!(
        launch
            .clone()
            .drag_model(HprsOwn)
            .drag_table(half.clone())
            .fly()
            .unwrap(),
        by_table
    );
    same_flight_less_18(
        &launch
            .clone()
            .drag_table(half.clone())
            .drag_model(HprsOwn)
            .fly()
            .unwrap(),
        &own,
    );

    // Less drag while the motor burns, a higher apogee than power off's alone, and a lower one
    // than with the power-on curve all the way.
    let apogee = |table: hpr_aero::DragTable| {
        launch
            .clone()
            .drag_table(table)
            .fly()
            .unwrap()
            .apogee_m()
            .unwrap()
    };
    let both =
        apogee(hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", Some("0,0.2\n1,0.2\n")).unwrap());
    let low = apogee(hpr_aero::DragTable::from_csv("0,0.2\n1,0.2\n", None).unwrap());
    let high = by_table.apogee_m().unwrap();
    assert!(high < both && both < low, "{high} {both} {low}");

    // A quarter of the coefficient on twice the diameter is the same drag.
    // `built`'s body diameter, its largest and so its reference diameter.
    let reference_diameter_m = 0.0563;
    let wide = hpr_aero::DragTable::from_csv("0,0.125\n1,0.125\n", None)
        .unwrap()
        .with_reference_diameter_m(2.0 * reference_diameter_m)
        .unwrap();
    let rescaled = apogee(wide);
    assert!((rescaled - high).abs() <= 1e-9 * high, "{rescaled} {high}");

    // Drag that pushes is refused where the flight meets it, in either curve, and past a table's
    // rows where it extrapolates below zero.
    use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
    let falling = Table1D::new(
        vec![0.1, 0.2],
        vec![0.1, 0.5],
        Interpolation::Linear,
        Extrapolation::Linear,
    )
    .unwrap();
    let tables = [
        hpr_aero::DragTable::from_csv("0,-0.01\n1,0.5\n", None).unwrap(),
        hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", Some("0,-0.2\n1,0.5\n")).unwrap(),
        hpr_aero::DragTable::new(falling, None),
    ];
    for table in tables {
        let error = launch.clone().drag_table(table).fly().unwrap_err();
        assert!(
            matches!(
                &error,
                Error::Sim(hpr_sim::SimError::Aero(hpr_aero::AeroError::Domain { what, value }))
                    if *what == "zero-lift drag coefficient from a drag table" && *value < 0.0
            ),
            "{error:?}"
        );
    }
}

/// A gravity model other than the default is flown, and the Earth's rotation kept with it.
#[test]
fn a_gravity_model_is_flown_in_place_of_the_default() {
    use hpr_core::earth::GravityModel;
    let rocket = built();
    let taylor = environment()
        .with_gravity(GravityModel::VerticalTaylor)
        .unwrap();
    assert_eq!(
        taylor.sim().earth.gravity_model(),
        GravityModel::VerticalTaylor
    );
    assert_eq!(
        taylor.sim().earth.rotation(),
        environment().sim().earth.rotation()
    );
    assert_eq!(taylor.sim().site(), environment().sim().site());
    let own = Flight::builder(&rocket, &environment(), 1.8).fly().unwrap();
    let by_taylor = Flight::builder(&rocket, &taylor, 1.8).fly().unwrap();
    // The two formulas differ by about 1e-8 near the ground (docs/physics/gravity.md), not by
    // nothing.
    let (a, b) = (own.apogee_m().unwrap(), by_taylor.apogee_m().unwrap());
    assert!(a != b && (a - b).abs() < 1e-6 * a, "{a} {b}");
    // A weaker uniform field, a higher flight.
    let light = environment()
        .with_gravity(GravityModel::Constant { g_mps2: 9.0 })
        .unwrap();
    let lighter = Flight::builder(&rocket, &light, 1.8).fly().unwrap();
    assert!(lighter.apogee_m().unwrap() > a);
    let error = environment()
        .with_gravity(GravityModel::Constant { g_mps2: -1.0 })
        .unwrap_err();
    assert!(
        matches!(
            error,
            Error::Core(hpr_core::CoreError::Domain { what, value })
                if what.starts_with("constant gravity magnitude") && value == -1.0
        ),
        "{error:?}"
    );
}

/// Loft lesson L95: a degenerate design must be refused or fly to finite numbers, never to a NaN
/// or a hang. The builder refuses each one as it is given, naming it. Put straight into a design
/// the builder can't check, each is refused before the flight or flies finite.
#[test]
fn degenerate_designs_error_or_stay_finite() {
    let abs = || material("abs").unwrap();
    let ogive = NoseShape::Ogive { radius_ratio: 1.0 };
    let domain = |result: Result<&mut Rocket, Error>| match result {
        Err(Error::Domain { what, value }) => (what, value),
        other => panic!("expected a domain error, got {other:?}"),
    };

    // Zero radius.
    assert!(matches!(
        Rocket::new("Zero", 0.0),
        Err(Error::Domain { what: "rocket diameter, m", value }) if value == 0.0
    ));
    let mut rocket = Rocket::new("Degenerate", 0.05).unwrap();
    let zero_tube = Tube::new(0.5, 0.001, abs()).with_diameter_m(0.0);
    assert_eq!(domain(rocket.add_tube(zero_tube)).0, "tube diameter, m");

    // NaN tokens, wherever a number goes.
    let (what, value) = domain(rocket.add_nose(Nose::solid(ogive, f64::NAN, abs())));
    assert!(what == "nose length, m" && value.is_nan());
    // A shape parameter out of its range: an ogive's radius ratio, its arc's radius over a tangent
    // ogive's, is at least the nose's radius over its length, 0.025 / 0.2 here.
    let short_arc = NoseShape::Ogive { radius_ratio: 0.1 };
    let design_domain = |result: Result<&mut Rocket, Error>| match result {
        Err(Error::Design(DesignError::Domain { what, value })) => (what, value),
        other => panic!("expected the design's domain error, got {other:?}"),
    };
    let (what, value) = design_domain(rocket.add_nose(Nose::solid(short_arc, 0.2, abs())));
    assert!(what.contains("ogive") && value == 0.1, "{what} {value}");
    let nan_wall = Nose::hollow(ogive, 0.2, f64::NAN, abs());
    assert_eq!(domain(rocket.add_nose(nan_wall)).0, "nose wall, m");
    let nan_tube = Tube::new(f64::INFINITY, 0.001, abs());
    assert_eq!(domain(rocket.add_tube(nan_tube)).0, "tube length, m");
    rocket.add_tube(Tube::new(0.5, 0.001, abs())).unwrap();
    let nan_mass = Mass::new(f64::NAN, Position::Top { aft_offset_m: 0.0 });
    assert_eq!(domain(rocket.add_mass(nan_mass)).0, "mass, kg");
    let nan_place = Mass::new(
        0.1,
        Position::Top {
            aft_offset_m: f64::NAN,
        },
    );
    assert_eq!(domain(rocket.add_mass(nan_place)).0, "position, m");
    assert!(matches!(
        rocket.add_fins(Fins::new(
            3,
            trapezoid([0.1, f64::NAN, 0.05, 0.0]),
            0.003,
            abs()
        )),
        Err(Error::Design(_))
    ));
    assert!(matches!(
        Rocket::new("NaN", f64::NAN),
        Err(Error::Domain { .. })
    ));
    assert!(matches!(
        Environment::new(f64::NAN, 0.0, 0.0),
        Err(Error::Core(_))
    ));

    // Zero fins.
    assert!(matches!(
        rocket.add_fins(Fins::new(
            0,
            trapezoid([0.1, 0.05, 0.05, 0.0]),
            0.003,
            abs()
        )),
        Err(Error::Design(_))
    ));

    // Negative mass.
    let negative = Mass::new(-0.1, Position::Top { aft_offset_m: 0.0 });
    let (what, value) = domain(rocket.add_mass(negative));
    assert!(what == "mass, kg" && value == -0.1);

    // The same, put straight into the tree of a rocket that flies, where only the design's own
    // checks see them: each is refused before the flight, by the part it is in, or flies to
    // finite numbers. Without any fin set the rocket is unstable, and tumbles.
    let refused = |id: &'static str, what: &'static str| -> Refusal { Some((id, what)) };
    let cases: [(&str, Spoil, Refusal); 9] = [
        (
            "zero tube radius",
            |design| {
                if let Part::BodyTube(tube) = &mut design.stages[0].components[1].part {
                    tube.outer_radius_m = 0.0;
                }
            },
            // The nose's shoulder takes the tube's inner radius, and finds it negative first.
            refused("nose", "outer radius"),
        ),
        (
            "zero tube radius, no shoulder before it",
            |design| {
                if let Part::NoseCone(nose) = &mut design.stages[0].components[0].part {
                    nose.shoulder = None;
                }
                design.stages[0].components[0].auto.clear();
                if let Part::BodyTube(tube) = &mut design.stages[0].components[1].part {
                    tube.outer_radius_m = 0.0;
                }
            },
            refused("tube", "outer radius"),
        ),
        (
            "NaN nose length",
            |design| {
                if let Part::NoseCone(nose) = &mut design.stages[0].components[0].part {
                    nose.length_m = f64::NAN;
                }
            },
            refused("nose", "body component length"),
        ),
        (
            "NaN fin span",
            |design| {
                if let Part::FinSet(FinSet {
                    planform: FinPlanform::Trapezoidal { span_m, .. },
                    ..
                }) = &mut design.stages[0].components[1].children[1].part
                {
                    *span_m = f64::NAN;
                }
            },
            refused("fins", "fin span"),
        ),
        (
            "zero fins",
            |design| {
                if let Part::FinSet(fins) = &mut design.stages[0].components[1].children[1].part {
                    fins.count = 0;
                }
            },
            refused("fins", "fin count (1 to 64)"),
        ),
        (
            "four billion fins",
            |design| {
                if let Part::FinSet(fins) = &mut design.stages[0].components[1].children[1].part {
                    fins.count = 4_000_000_000;
                }
            },
            refused("fins", "fin count (1 to 64)"),
        ),
        (
            "no fin set",
            |design| {
                design.stages[0].components[1].children.remove(1);
            },
            None,
        ),
        (
            "negative mass",
            |design| {
                if let Part::MassComponent(mass) =
                    &mut design.stages[0].components[1].children[2].part
                {
                    mass.mass_kg = -0.1;
                }
            },
            refused("mass", "mass"),
        ),
        (
            "negative mass override",
            |design| {
                design.stages[0].overrides.mass_kg = Some(-1.0);
            },
            refused("sustainer", "mass override (kg)"),
        ),
    ];
    for (name, spoil, expected) in cases {
        let mut design = built().design().clone();
        let before = design.clone();
        spoil(&mut design);
        assert_ne!(design, before, "{name}: the case changed nothing");
        let outcome = Rocket::from_design(design, "168H54-10A").and_then(|mut rocket| {
            rocket.add_parachute(parachute());
            Flight::builder(&rocket, &environment(), 1.8).fly()
        });
        match (outcome, expected) {
            (Ok(flight), None) => assert_flies_finite(name, &flight),
            (
                Err(Error::Sim(hpr_sim::SimError::Design(DesignError::InComponent { id, source }))),
                Some((part, quantity)),
            ) if id == part
                && matches!(*source, DesignError::Domain { what, .. } if what == quantity) => {}
            (other, expected) => panic!("{name}: expected {expected:?}, got {other:?}"),
        }
    }
}

/// A change that spoils a design.
type Spoil = fn(&mut hpr_design::Rocket);

/// The part and the quantity that refuse a spoiled design, or `None` if it flies.
type Refusal = Option<(&'static str, &'static str)>;

/// Every number a flight reports is finite.
fn assert_flies_finite(name: &str, flight: &Flight) {
    let summary = flight.summary();
    let peaks = [
        summary.rail_exit_speed_m_s,
        summary.max_speed_m_s,
        summary.max_mach,
        summary.max_dynamic_pressure_pa,
        summary.max_acceleration_m_s2,
    ];
    for peak in peaks.into_iter().flatten() {
        assert!(
            peak.value.is_finite() && peak.time_s.is_finite(),
            "{name}: {peak:?}"
        );
    }
    for event in &flight.result().events {
        let sample = &event.sample;
        let numbers = [
            sample.time_s,
            sample.height_above_ground_m,
            sample.cg_velocity_enu_m_s.length(),
            sample.mass_kg,
            sample.mach,
        ];
        assert!(
            numbers.iter().all(|x| x.is_finite()),
            "{name}: {:?} at {:?}",
            numbers,
            event.kind
        );
    }
}

/// The height above sea level, m, at which the flight refused a wind whose velocity isn't finite,
/// from its error; any other error fails the test.
fn refused_wind_height_msl_m(error: &Error) -> f64 {
    match error {
        Error::Sim(hpr_sim::SimError::Domain { what, value })
            if what.contains("the wind's velocity is not finite") =>
        {
            assert!(error.to_string().contains("wind"), "{error}");
            *value
        }
        other => panic!("not the wind's refusal: {other:?}"),
    }
}

/// A wind of your own that isn't a finite velocity stops the climb where it is read, with an
/// error naming the wind and the height (issue #237). Before, the drag's Reynolds number was the
/// first to see it, and the error named that instead.
#[test]
fn a_wind_that_is_not_finite_stops_the_flight() {
    /// A west wind of `speed_m_s` above 300 m over the site, and calm below.
    #[derive(Debug)]
    struct Aloft(f64);
    impl hpr_atmos::Wind for Aloft {
        fn wind(&self, height_msl_m: f64) -> Result<hpr_atmos::WindSample, hpr_atmos::AtmosError> {
            let east_m_s = if height_msl_m > 1700.0 { self.0 } else { 0.0 };
            Ok(hpr_atmos::WindSample {
                velocity_enu_m_s: hpr_core::DVec3::new(east_m_s, 0.0, 0.0),
                extrapolated: None,
            })
        }
    }
    let rocket = built();
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let environment = environment().with_wind(Aloft(bad));
        let error = Flight::builder(&rocket, &environment, 1.8)
            .fly()
            .unwrap_err();
        // Refused on the way up through 1,700 m, within a step of it.
        let height_msl_m = refused_wind_height_msl_m(&error);
        assert!(
            height_msl_m > 1700.0 && height_msl_m < 1750.0,
            "{bad}: {height_msl_m}"
        );
    }
    // The same wind, finite, flies.
    let environment = environment().with_wind(Aloft(5.0));
    assert!(Flight::builder(&rocket, &environment, 1.8).fly().is_ok());
}

/// Under a canopy there is no drag model to see a wind that isn't finite; the flight refuses it
/// where it reads it, naming the wind and the height (issue #237). Before, the integrator's own
/// check that the state stays finite fired, and the error said only that the integration failed.
#[test]
fn a_wind_that_is_not_finite_under_a_canopy_stops_the_descent() {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// A west wind of `.0` below 1,900 m above sea level (500 m above the site) once the flight
    /// has been above 2,400 m, and calm otherwise: the rocket meets it only coming down.
    #[derive(Debug)]
    struct OnTheWayDown(f64, AtomicBool);
    impl hpr_atmos::Wind for OnTheWayDown {
        fn wind(&self, height_msl_m: f64) -> Result<hpr_atmos::WindSample, hpr_atmos::AtmosError> {
            if height_msl_m > 2400.0 {
                self.1.store(true, Ordering::Relaxed);
            }
            let down = self.1.load(Ordering::Relaxed) && height_msl_m < 1900.0;
            Ok(hpr_atmos::WindSample {
                velocity_enu_m_s: hpr_core::DVec3::new(if down { self.0 } else { 0.0 }, 0.0, 0.0),
                extrapolated: None,
            })
        }
    }
    let rocket = built();
    // In calm air the parachute opens above 2,400 m (1,000 m over the site), so the rocket is
    // under it when it comes down through 1,900 m.
    let calm = Flight::builder(&rocket, &environment(), 1.8).fly().unwrap();
    let opened = calm
        .result()
        .event(EventKind::Deployment(0))
        .unwrap()
        .sample;
    assert!(opened.height_above_ground_m > 1000.0, "{opened:?}");
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let environment = environment().with_wind(OnTheWayDown(bad, AtomicBool::new(false)));
        let error = Flight::builder(&rocket, &environment, 1.8)
            .fly()
            .unwrap_err();
        // Refused on the way down through 1,900 m, within a step of it.
        let height_msl_m = refused_wind_height_msl_m(&error);
        assert!(
            height_msl_m < 1900.0 && height_msl_m > 1850.0,
            "{bad}: {height_msl_m}"
        );
    }
    // The same wind, finite, flies to the ground.
    let environment = environment().with_wind(OnTheWayDown(5.0, AtomicBool::new(false)));
    assert!(Flight::builder(&rocket, &environment, 1.8).fly().is_ok());
}

/// The fields of the air a test atmosphere spoils, with the words the flight's refusal uses for
/// each, and whether the flight takes a zero (a vacuum's density and pressure).
const AIR_FIELDS: [(&str, bool); 5] = [
    ("density", true),
    ("pressure", true),
    ("temperature", false),
    ("speed of sound", false),
    ("viscosity", false),
];

/// The standard atmosphere's air at `height_msl_m`, with field `field` of [`AIR_FIELDS`] set to
/// `value` where `spoiled` is true.
fn spoiled_air(
    height_msl_m: f64,
    field: usize,
    value: f64,
    spoiled: bool,
) -> Result<hpr_atmos::AirSample, hpr_atmos::AtmosError> {
    let mut sample = hpr_atmos::Ussa76::standard().sample(height_msl_m)?;
    if spoiled {
        let air = &mut sample.air;
        *[
            &mut air.density_kg_m3,
            &mut air.pressure_pa,
            &mut air.temperature_k,
            &mut air.speed_of_sound_m_s,
            &mut air.dynamic_viscosity_pa_s,
        ]
        .into_iter()
        .nth(field)
        .unwrap() = value;
    }
    Ok(sample)
}

/// The height above sea level, m, at which the flight refused the air's `field`, from its error;
/// any other error fails the test.
fn refused_air_height_msl_m(error: &Error, field: &str) -> f64 {
    match error {
        Error::Sim(hpr_sim::SimError::Domain { what, value })
            if what.starts_with("height above sea level, m, at which the air's ")
                && what.contains(&format!("the air's {field} is ")) =>
        {
            assert!(error.to_string().contains(field), "{error}");
            *value
        }
        other => panic!("not the refusal of the air's {field}: {other:?}"),
    }
}

/// Each field of the air the flight can't use stops the climb where it is read, with an error
/// naming the field and the height (issue #301). Before, a density that was negative or minus
/// infinity turned the drag off and the rocket climbed about twice as high, with no error; a
/// temperature or pressure that wasn't finite flew on unseen, and an infinite speed of sound or
/// viscosity flew with a Mach or Reynolds number of zero. A zero density or pressure, a vacuum's,
/// is taken.
#[test]
fn air_the_flight_cannot_use_stops_the_climb() {
    /// The standard atmosphere, with field `.0` set to `.1` above 1,700 m above sea level (300 m
    /// over the site).
    #[derive(Debug)]
    struct Aloft(usize, f64);
    impl hpr_atmos::Atmosphere for Aloft {
        fn air(&self, height_msl_m: f64) -> Result<hpr_atmos::AirSample, hpr_atmos::AtmosError> {
            spoiled_air(height_msl_m, self.0, self.1, height_msl_m > 1700.0)
        }
    }
    let rocket = built();
    for (field, &(name, takes_zero)) in AIR_FIELDS.iter().enumerate() {
        let zero = if takes_zero { vec![] } else { vec![0.0] };
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0]
            .into_iter()
            .chain(zero)
        {
            let environment = environment().with_atmosphere(Aloft(field, bad));
            let error = Flight::builder(&rocket, &environment, 1.8)
                .fly()
                .unwrap_err();
            // Refused on the way up through 1,700 m, within a step of it.
            let height_msl_m = refused_air_height_msl_m(&error, name);
            assert!(
                height_msl_m > 1700.0 && height_msl_m < 1750.0,
                "{name} {bad}: {height_msl_m}"
            );
        }
        if takes_zero {
            let environment = environment().with_atmosphere(Aloft(field, 0.0));
            assert!(
                Flight::builder(&rocket, &environment, 1.8).fly().is_ok(),
                "{name} 0"
            );
        }
    }
}

/// Under a canopy the flight reads the air too, and refuses each field it can't use there, naming
/// the field and the height (issue #301). Before, a density that was negative or minus infinity
/// turned the canopy's drag off and the rocket landed at about 99 m/s with no error; one that was
/// NaN or infinite stopped the flight with only "the integration failed"; the other fields went
/// unread.
#[test]
fn air_the_flight_cannot_use_under_a_canopy_stops_the_descent() {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// The standard atmosphere, with field `.0` set to `.1` below 1,900 m above sea level (500 m
    /// over the site) once the flight has been above 2,400 m: the rocket meets it only coming
    /// down.
    #[derive(Debug)]
    struct OnTheWayDown(usize, f64, AtomicBool);
    impl hpr_atmos::Atmosphere for OnTheWayDown {
        fn air(&self, height_msl_m: f64) -> Result<hpr_atmos::AirSample, hpr_atmos::AtmosError> {
            if height_msl_m > 2400.0 {
                self.2.store(true, Ordering::Relaxed);
            }
            let down = self.2.load(Ordering::Relaxed) && height_msl_m < 1900.0;
            spoiled_air(height_msl_m, self.0, self.1, down)
        }
    }
    let rocket = built();
    // In the standard atmosphere the parachute opens above 2,400 m (1,000 m over the site), so the
    // rocket is under it when it comes down through 1,900 m.
    let calm = Flight::builder(&rocket, &environment(), 1.8).fly().unwrap();
    let opened = calm
        .result()
        .event(EventKind::Deployment(0))
        .unwrap()
        .sample;
    assert!(opened.height_above_ground_m > 1000.0, "{opened:?}");
    for (field, &(name, takes_zero)) in AIR_FIELDS.iter().enumerate() {
        let zero = if takes_zero { vec![] } else { vec![0.0] };
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0]
            .into_iter()
            .chain(zero)
        {
            let environment =
                environment().with_atmosphere(OnTheWayDown(field, bad, AtomicBool::new(false)));
            let error = Flight::builder(&rocket, &environment, 1.8)
                .fly()
                .unwrap_err();
            // Refused on the way down through 1,900 m, within a step of it.
            let height_msl_m = refused_air_height_msl_m(&error, name);
            assert!(
                height_msl_m < 1900.0 && height_msl_m > 1850.0,
                "{name} {bad}: {height_msl_m}"
            );
        }
    }
}

/// The library's two-stage example's `.ork` text: an F15 in each stage, the booster dropping at
/// its burnout.
fn two_stage_ork() -> &'static str {
    let example = include_str!("../examples/ork_two_stage.rs");
    let start = example.find("r#\"").unwrap() + 3;
    let end = example[start..].find("\"#").unwrap() + start;
    &example[start..end]
}

/// A staged flight warns of #179, its booster flying on as a point with only its tumble's drag
/// (M10.1d5, ADR-200); the booster's tumble opens at the split, so it meets no #354. Parted at
/// apogee, with nothing left to burn, it warns the same on a drag table of its own, which a
/// powered separation refuses; with the sustainer's canopy fired at apogee but open 1 s later, it
/// meets #354 too. The same rocket flown whole meets neither.
#[test]
fn a_staged_flight_warns_of_its_boosters_drag() {
    let file = hpr_io::ork::read(two_stage_ork().as_bytes()).unwrap().value;
    let design = hpr_io::ork::design(&file).value;
    let configuration = &design.motors.configurations[0];
    let assembly = design.rocket.assemble(&configuration.id).unwrap();
    let separations = crate::ork::separations(configuration.staging.as_ref(), &assembly).unwrap();
    let devices =
        crate::ork::tumbling(Vec::new(), &design.rocket, &assembly, &separations).unwrap();
    let mut rocket = Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    for device in devices {
        rocket.add_parachute(device);
    }
    let environment = environment();
    let separated = |number: u32, flight: &Flight| {
        flight
            .issue_warnings()
            .iter()
            .any(|warning| warning.number() == number)
    };
    let flight = Flight::builder(&rocket, &environment, 1.5)
        .separations(separations.clone())
        .fly()
        .unwrap();
    assert!(!flight.result().bodies.is_empty());
    assert!(separated(179, &flight));
    assert!(!separated(354, &flight));
    let message = flight
        .issue_warnings()
        .iter()
        .find(|warning| warning.number() == 179)
        .unwrap()
        .message();
    assert!(
        message.starts_with("issue #179: a booster dropped"),
        "{message}"
    );
    assert!(message.contains("drift likely read short"), "{message}");
    let at_apogee: Vec<_> = separations
        .iter()
        .map(|separation| hpr_sim::Separation {
            trigger: Trigger::Apogee,
            ..*separation
        })
        .collect();
    let mut parted = Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    for device in crate::ork::tumbling(Vec::new(), &design.rocket, &assembly, &at_apogee).unwrap() {
        parted.add_parachute(device);
    }
    let table = hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", None).unwrap();
    let own = Flight::builder(&parted, &environment, 1.5)
        .separations(at_apogee.clone())
        .drag_table(table)
        .fly()
        .unwrap();
    assert_eq!(own.result().termination, hpr_sim::Termination::Separated);
    assert!(separated(179, &own));
    assert!(!separated(354, &own));
    // The sustainer's own canopy, fired at apogee but open 1 s later, in place of its tumble: it
    // falls with nothing open for that second, so #354 too.
    let mut lagging = Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    for device in crate::ork::tumbling(Vec::new(), &design.rocket, &assembly, &at_apogee).unwrap() {
        if device.body != 0 {
            lagging.add_parachute(device);
        }
    }
    lagging.add_parachute(
        Device::new(
            "sustainer canopy",
            DeviceDrag::canopy(CanopyType::FlatCircular, 0.6),
            Trigger::Apogee,
        )
        .with_lag_s(1.0),
    );
    let falling = Flight::builder(&lagging, &environment, 1.5)
        .separations(at_apogee.clone())
        .fly()
        .unwrap();
    assert!(separated(179, &falling) && separated(354, &falling));
    let message = falling
        .issue_warnings()
        .iter()
        .find(|warning| warning.number() == 354)
        .unwrap()
        .message();
    assert!(
        message.starts_with("issue #354: a separated part"),
        "{message}"
    );
    assert!(
        message.contains("its drift likely reads short"),
        "{message}"
    );
    let bare = Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    let whole = Flight::builder(&bare, &environment, 1.5).fly().unwrap();
    assert!(whole.result().bodies.is_empty());
    assert!(!separated(179, &whole) && !separated(354, &whole));
}

/// A staged flight's separations reach a Monte Carlo run (M4.6a): with nothing dispersed, every
/// sample is the builder's own flight, bit for bit, the stack coming apart; dispersed, every
/// sample flies, and the tumbles hpr adds for the parts are not deployed, so no lag is drawn for
/// them.
#[test]
fn a_staged_flight_is_dispersed_with_its_separation() {
    use hpr_analysis::montecarlo::{Dispersion, FailedAt, MonteCarlo, Outcome};
    let file = hpr_io::ork::read(two_stage_ork().as_bytes()).unwrap().value;
    let design = hpr_io::ork::design(&file).value;
    let configuration = &design.motors.configurations[0];
    let assembly = design.rocket.assemble(&configuration.id).unwrap();
    let separations = crate::ork::separations(configuration.staging.as_ref(), &assembly).unwrap();
    assert_eq!(separations.len(), 1);
    let devices =
        crate::ork::tumbling(Vec::new(), &design.rocket, &assembly, &separations).unwrap();
    assert!(
        devices
            .iter()
            .all(|device| matches!(device.drag, DeviceDrag::Tumble { .. }))
    );
    let mut rocket = Rocket::from_design(design.rocket.clone(), &configuration.id).unwrap();
    for device in devices {
        rocket.add_parachute(device);
    }
    let environment = environment();
    let builder = Flight::builder(&rocket, &environment, 1.5).separations(separations.clone());
    let flight = builder.fly().unwrap();
    assert_eq!(flight.result().termination, hpr_sim::Termination::GroundHit);
    assert!(!flight.result().bodies.is_empty());

    let inputs = builder.inputs().unwrap();
    assert_eq!(inputs.separations, separations);
    let nominal = MonteCarlo::new(inputs.clone(), Dispersion::default()).unwrap();
    for index in 0..3 {
        let sample = nominal.sample(7, index);
        let summary = sample.summary().unwrap();
        assert_eq!(summary, flight.summary(), "sample {index}");
        let (flown, built) = (summary.landing.unwrap(), flight.summary().landing.unwrap());
        assert_eq!(flown.east_m.to_bits(), built.east_m.to_bits());
        assert_eq!(flown.north_m.to_bits(), built.north_m.to_bits());
        assert_eq!(summary.body_landings.len(), 1);
    }

    let dispersion = Dispersion {
        dry_mass_sd_fraction: 0.02,
        impulse_sd_fraction: 0.03,
        burn_time_sd_fraction: 0.02,
        rail_elevation_sd_rad: 1.0_f64.to_radians(),
        deployment_lag_sd_s: 0.5,
        ..Dispersion::default()
    };
    let run = MonteCarlo::new(inputs, dispersion).unwrap().run(7, 8);
    assert_eq!(run.failed().count(), 0);
    let apogees: Vec<f64> = run
        .samples
        .iter()
        .map(|sample| {
            sample
                .summary()
                .unwrap()
                .apogee
                .unwrap()
                .height_above_ground_m
        })
        .collect();
    assert!(
        apogees.iter().skip(1).all(|apogee| *apogee != apogees[0]),
        "{apogees:?}"
    );
    for sample in &run.samples {
        assert_eq!(sample.draw.deployment_lag_offset_s, [0.0, 0.0]);
        assert_eq!(sample.summary().unwrap().body_landings.len(), 1);
    }

    // A split timed in seconds isn't dispersed: at the nominal burnout plus 0.01 s, a booster
    // whose drawn burn is longer still burns there, and its flight fails by name and is counted;
    // the split at the burnout, above, follows the drawn burn and never does.
    let split_s = flight
        .result()
        .event(hpr_sim::EventKind::Separation)
        .unwrap()
        .sample
        .time_s;
    let timed = vec![hpr_sim::Separation {
        trigger: hpr_sim::Trigger::Time {
            time_s: split_s + 0.01,
        },
        ..separations[0]
    }];
    let builder = Flight::builder(&rocket, &environment, 1.5).separations(timed);
    builder.fly().unwrap();
    let dispersion = Dispersion {
        burn_time_sd_fraction: 0.05,
        ..Dispersion::default()
    };
    let run = MonteCarlo::new(builder.inputs().unwrap(), dispersion)
        .unwrap()
        .run(7, 16);
    let failed = run.failed().count();
    assert!(0 < failed && failed < 16, "{failed} of 16 failed");
    let booster = assembly
        .motors
        .iter()
        .position(|placed| placed.stage > separations[0].after_stage)
        .unwrap();
    for sample in &run.samples {
        let scale = sample.draw.burn_time_scale[booster];
        match &sample.outcome {
            Outcome::Failed { at, reason } => {
                assert_eq!(*at, FailedAt::Flight);
                assert!(
                    reason.contains("the aft body's motors must have burned out"),
                    "{reason}"
                );
                assert!(
                    scale > 1.0 + 0.01 / split_s,
                    "sample {}: {scale}",
                    sample.index
                );
            }
            Outcome::Flown { .. } => assert!(
                scale < 1.0 + 0.01 / split_s,
                "sample {}: {scale}",
                sample.index
            ),
        }
    }
}

/// The shape's stability warnings are hpr's normal force's, not its drag's (ADR-189): a second
/// fin set at the first one's station meets #325 on hpr's drag and on a drag of the flight's own
/// alike, where the drag warnings go.
#[test]
fn a_drag_of_the_flights_own_keeps_the_shapes_stability_warnings() {
    let mut rocket = built();
    rocket
        .add_fins(
            Fins::new(
                3,
                trapezoid([0.1, 0.04, 0.045, 0.05]),
                0.003175,
                material("birch_plywood").unwrap(),
            )
            .with_cross_section(FinCrossSection::Rounded),
        )
        .unwrap();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    let numbers = |flight: &Flight| -> Vec<u32> {
        flight
            .issue_warnings()
            .iter()
            .map(hpr_sim::issues::IssueWarning::number)
            .collect()
    };
    // hpr's drag adds #18 (fully turbulent friction); a drag of the flight's own drops it.
    assert_eq!(numbers(&launch.fly().unwrap()), [18, 325, 172]);
    assert_eq!(
        numbers(&launch.clone().drag_model(HprsOwn).fly().unwrap()),
        [325, 172]
    );
    let table = hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n", None).unwrap();
    assert_eq!(
        numbers(&launch.clone().drag_table(table).fly().unwrap()),
        [325, 172]
    );
    // The rocket as built, one fin set: #18 and #172.
    assert_eq!(
        numbers(&Flight::builder(&built(), &environment, 1.8).fly().unwrap()),
        [18, 172]
    );
}

/// The synthetic 54 mm validation design, on its own I175 or, with `motor`, a catalog motor in
/// its place; without its rail buttons, the only part off its axis, when `buttonless`.
fn synthetic(motor: Option<&str>, buttonless: bool) -> Rocket {
    let mut design: hpr_design::Rocket = serde_json::from_str(include_str!(
        "../../../validation/designs/synthetic-54mm-three-fin.json"
    ))
    .unwrap();
    if let Some(name) = motor {
        let motor = Motor::from_catalog(name).unwrap();
        let mounted = &mut design.configurations[0].motors[0];
        mounted.designation = motor.designation().to_owned();
        mounted.diameter_m = motor.diameter_m();
        mounted.length_m = motor.length_m();
        mounted.motor = motor.solid_motor().clone();
    }
    if buttonless {
        design.stages[0].components[1]
            .children
            .retain(|child| !matches!(child.part, Part::RailButton(_)));
    }
    let id = design.configurations[0].id.clone();
    Rocket::from_design(design, &id).unwrap()
}

fn numbers(flight: &Flight) -> Vec<u32> {
    flight
        .issue_warnings()
        .iter()
        .map(hpr_sim::issues::IssueWarning::number)
        .collect()
}

fn warning(flight: &Flight, number: u32) -> &hpr_sim::issues::IssueWarning {
    flight
        .issue_warnings()
        .iter()
        .find(|warning| warning.number() == number)
        .unwrap_or_else(|| panic!("#{number} in {:?}", numbers(flight)))
}

/// The unknown signs' warnings on flights (M10.1d6, ADR-201): the synthetic design's rail
/// buttons put its center of gravity off the axis (#219), on hpr's drag or a table of its own;
/// without them it meets none. On an I377 it passes Mach 1.2, into the body's supersonic join
/// (#106), which its own I175 never reaches.
#[test]
fn a_flight_warns_of_its_off_axis_mass_and_its_supersonic_damping_stations() {
    let environment = environment();
    let fast = synthetic(Some("I377CT"), false);
    let launch = Flight::builder(&fast, &environment, 1.8);
    let flight = launch.fly().unwrap();
    let top = flight.summary().max_mach.unwrap().value;
    assert!(top > 1.3, "{top}");
    let found = numbers(&flight);
    assert!(found.contains(&219) && found.contains(&106), "{found:?}");
    let station = warning(&flight, 106);
    assert_eq!(station.parts, ["nose", "sustainer-airframe"]);
    let message = station.message();
    assert!(
        message.contains("from Mach 1.20 to 1.50, a body part's pitch and yaw damping"),
        "{message}"
    );
    let message = warning(&flight, 219).message();
    assert!(message.contains(" mm off the rocket's axis"), "{message}");
    // A drag table of the flight's own drops hpr's drag warnings, not these.
    let table = hpr_aero::DragTable::from_csv("0,0.5\n1,0.5\n2,0.5\n", None).unwrap();
    let tabled = numbers(&launch.clone().drag_table(table).fly().unwrap());
    assert!(tabled.contains(&219) && tabled.contains(&106), "{tabled:?}");
    assert!(!tabled.contains(&18), "{tabled:?}");
    // On its own I175 it stays below the join; without its buttons it sits on its axis.
    let slow = Flight::builder(&synthetic(None, false), &environment, 1.8)
        .fly()
        .unwrap();
    assert!(slow.summary().max_mach.unwrap().value < 1.2);
    let found = numbers(&slow);
    assert!(found.contains(&219) && !found.contains(&106), "{found:?}");
    let symmetric = Flight::builder(&synthetic(Some("I377CT"), true), &environment, 1.8)
        .fly()
        .unwrap();
    let found = numbers(&symmetric);
    assert!(!found.contains(&219) && found.contains(&106), "{found:?}");
    // Each of these is of kind flight.
    for number in [106, 219] {
        assert_eq!(
            warning(&flight, number).issue.kind(),
            hpr_sim::issues::IssueKind::Flight
        );
    }
}

/// A single pod off the axis warns of #213 on a flight, naming its pod set; two pods evenly
/// spaced don't (M10.1d6, ADR-201).
#[test]
fn a_flight_with_a_single_pod_warns_of_its_left_out_moments() {
    let podded = |count: u32| {
        let mut design = synthetic(None, true).design().clone();
        let airframe = &mut design.stages[0].components[1];
        let mut tube = airframe.clone();
        tube.id = "pod-tube".to_owned();
        tube.children = Vec::new();
        tube.part = Part::BodyTube(BodyTube {
            length_m: 0.2,
            outer_radius_m: 0.01,
            thickness_m: 0.001,
            material: material("kraft_phenolic").unwrap(),
        });
        let mut pods = tube.clone();
        pods.id = "pods".to_owned();
        pods.part = Part::PodSet(hpr_design::PodSet {
            count,
            radial_offset_m: 0.04,
            angle_rad: 0.3,
        });
        pods.position = Some(Position::Top { aft_offset_m: 0.3 });
        pods.children = vec![tube];
        airframe.children.push(pods);
        let id = design.configurations[0].id.clone();
        Rocket::from_design(design, &id).unwrap()
    };
    let environment = environment();
    let single = Flight::builder(&podded(1), &environment, 1.8)
        .fly()
        .unwrap();
    assert_eq!(warning(&single, 213).parts, ["pods"]);
    let pair = Flight::builder(&podded(2), &environment, 1.8)
        .fly()
        .unwrap();
    assert!(!numbers(&pair).contains(&213), "{:?}", numbers(&pair));
}

/// A tabulated wind whose near-calm level turns inside the flight warns of #8; the same table
/// interpolated by components, or turning only above the flight, doesn't (M10.1d6, ADR-201).
#[test]
fn a_flight_through_a_near_calm_turn_warns_of_its_swinging_wind() {
    use hpr_atmos::{LayeredWind, WindInterpolation, WindLevel};
    let rocket = built();
    let fly = |base_m: f64, interpolation| {
        // 1,400 m is the site's height above sea level, where the table starts.
        let levels = vec![
            WindLevel {
                height_msl_m: 1400.0 + base_m,
                speed_m_s: 0.4,
                direction_from_rad: 0.0,
            },
            WindLevel {
                height_msl_m: 1400.0 + base_m + 500.0,
                speed_m_s: 8.0,
                direction_from_rad: 1.5 * std::f64::consts::PI,
            },
        ];
        let wind = LayeredWind::new(levels, interpolation).unwrap();
        Flight::builder(&rocket, &environment().with_wind(wind), 1.8)
            .fly()
            .unwrap()
    };
    let swinging = fly(0.0, WindInterpolation::SpeedDirection);
    let message = warning(&swinging, 8).message();
    assert!(
        message.contains("across 1400 to 1900 m above sea level, from 0.4 m/s at 0° to 8.0 m/s"),
        "{message}"
    );
    assert_eq!(
        warning(&swinging, 8).issue.kind(),
        hpr_sim::issues::IssueKind::Flight
    );
    let components = fly(0.0, WindInterpolation::Components);
    assert!(!numbers(&components).contains(&8));
    let apogee_m = swinging.summary().apogee.unwrap().height_above_ground_m;
    let above = fly(apogee_m + 50.0, WindInterpolation::SpeedDirection);
    assert!(!numbers(&above).contains(&8), "{:?}", numbers(&above));
}

/// The synthetic two-stage design on catalog motors (M10.1d6, ADR-201): `booster` lit at launch,
/// `sustainer` lit 0.2 s after the separation, which comes `coast_s` after the booster burns out;
/// a canopy on the sustainer at apogee, and the booster tumbling from the split.
fn two_stage_flight(booster: &str, sustainer: &str, coast_s: f64) -> Flight {
    let mut design: hpr_design::Rocket = serde_json::from_str(include_str!(
        "../../../validation/designs/synthetic-two-stage-75mm-54mm.json"
    ))
    .unwrap();
    let id = design.configurations[0].id.clone();
    for (mount, name) in [
        ("booster-motor-mount", booster),
        ("sustainer-motor-mount", sustainer),
    ] {
        let motor = Motor::from_catalog(name).unwrap();
        let mounted = design.configurations[0]
            .motors
            .iter_mut()
            .find(|mounted| mounted.mount == mount)
            .unwrap();
        mounted.designation = motor.designation().to_owned();
        mounted.diameter_m = motor.diameter_m();
        mounted.length_m = motor.length_m();
        mounted.motor = motor.solid_motor().clone();
        if mount == "sustainer-motor-mount" {
            mounted.ignition = Ignition::Separation { delay_s: 0.2 };
        }
    }
    let burnout_s = Motor::from_catalog(booster)
        .unwrap()
        .solid_motor()
        .burnout_time_s();
    let tumble = DeviceDrag::tumbling_stages(&design.assemble(&id).unwrap(), (1, 1)).unwrap();
    let mut rocket = Rocket::from_design(design, &id).unwrap();
    rocket.add_parachute(Device::new(
        "sustainer main",
        DeviceDrag::canopy(CanopyType::FlatCircular, 1.2),
        Trigger::Apogee,
    ));
    // A booster's devices act only once it flies, so a time of zero opens it at the split.
    rocket.add_parachute(
        Device::new("booster tumble", tumble, Trigger::Time { time_s: 0.0 }).on_body(1),
    );
    let separation = hpr_sim::Separation::new(
        Trigger::Time {
            time_s: burnout_s + coast_s,
        },
        0,
    );
    Flight::builder(&rocket, &environment(), 1.8)
        .separation(separation)
        .fly()
        .unwrap()
}

/// The joins of a flight's #106 warning, by vehicle.
fn joins(flight: &Flight) -> Vec<hpr_sim::issues::SupersonicJoin> {
    match &warning(flight, 106).detail {
        Some(hpr_sim::issues::IssueDetail::SupersonicJoins { joins }) => joins.clone(),
        other => panic!("{other:?}"),
    }
}

/// #106 and #219 on a two-stage flight are each vehicle's (M10.1d6, ADR-201): on a J450DM the
/// stack peaks below its own join, which starts at Mach 1.38, but the sustainer, lit by the
/// separation on an H170M, peaks at Mach 1.33 at 4.36 s, after the 2.61 s split, past its own,
/// which starts at 1.20. The warning quotes the sustainer's span, by name, and the sustainer's
/// center of gravity, farther off the axis than the stack's ever is.
#[test]
fn a_sustainer_past_its_own_join_warns_of_106_with_its_own_span() {
    let flight = two_stage_flight("J450DM", "H170M", 0.3);
    let top = flight.summary().max_mach.unwrap();
    assert!((top.value - 1.3295).abs() < 5e-4, "{top:?}");
    assert!((top.time_s - 4.36).abs() < 0.01, "{top:?}");
    let split_s = flight
        .result()
        .event(EventKind::Separation)
        .unwrap()
        .sample
        .time_s;
    assert!((split_s - 2.611).abs() < 1e-9, "{split_s}");
    let [join] = joins(&flight)[..] else {
        panic!("{:?}", joins(&flight))
    };
    assert_eq!(join.vehicle, 1);
    assert_eq!(join.join_start_mach, 1.2);
    assert_eq!(join.join_end_mach, Some(1.5));
    let message = warning(&flight, 106).message();
    assert!(
        message.contains("from Mach 1.20 to 1.50 on the sustainer after the first separation"),
        "{message}"
    );
    assert!(!message.contains("1.38"), "{message}");
    assert_eq!(warning(&flight, 106).parts, ["nose", "sustainer-airframe"]);
    // The stack's own join starts past the flight's top: read on the stack alone, the flight
    // would warn of none.
    let stack = hpr_aero::AeroModel::new(
        &serde_json::from_str::<hpr_design::Rocket>(include_str!(
            "../../../validation/designs/synthetic-two-stage-75mm-54mm.json"
        ))
        .unwrap()
        .layout()
        .unwrap(),
    )
    .unwrap();
    let stack_start = stack.supersonic_body().unwrap().join_start_mach;
    assert!((stack_start - 1.3805).abs() < 5e-5, "{stack_start}");
    assert!(top.value < stack_start);
    // #219 quotes the sustainer's offset at its burnout, 0.0996 mm, past the stack's largest at
    // its ends, 0.0632 mm.
    let offset_m = match warning(&flight, 219).detail {
        Some(hpr_sim::issues::IssueDetail::CenterOfGravityOffset { offset_m }) => offset_m,
        ref other => panic!("{other:?}"),
    };
    assert!((offset_m / 9.96e-5 - 1.0).abs() < 0.01, "{offset_m:e}");
    let mut design: hpr_design::Rocket = serde_json::from_str(include_str!(
        "../../../validation/designs/synthetic-two-stage-75mm-54mm.json"
    ))
    .unwrap();
    design.configurations.truncate(1);
    let id = design.configurations[0].id.clone();
    let ends = {
        let assembly = design.assemble(&id).unwrap();
        [
            assembly.mass_properties(0.0),
            assembly.dry_mass_properties(),
        ]
        .iter()
        .map(|mass| mass.cg_m.x.hypot(mass.cg_m.y))
        .fold(0.0, f64::max)
    };
    assert!(offset_m > 1.4 * ends, "{offset_m:e} {ends:e}");
}

/// A sustainer that stays below its own join start adds no #106 (M10.1d6, ADR-201). On a J760
/// with an H54 the stack passes its own join and warns alone, its span unnamed; with an H125CT
/// the stack peaks at Mach 1.3801, just short of its join's 1.3805, and the sustainer below 1.2,
/// so though the flight passes Mach 1.2, the sustainer's join start, neither vehicle meets it.
#[test]
fn a_sustainer_below_its_own_join_adds_no_106() {
    let stacked = two_stage_flight("J760", "H54", 2.0);
    let top = stacked.summary().max_mach.unwrap();
    assert!(top.value > 1.43 && top.time_s < 2.0, "{top:?}");
    let [join] = joins(&stacked)[..] else {
        panic!("{:?}", joins(&stacked))
    };
    assert_eq!(join.vehicle, 0);
    assert!((join.join_start_mach - 1.3805).abs() < 5e-5, "{join:?}");
    let message = warning(&stacked, 106).message();
    assert!(
        message.contains("from Mach 1.38 to 1.68, a body"),
        "{message}"
    );
    assert!(!message.contains("sustainer after"), "{message}");
    assert_eq!(
        warning(&stacked, 106).parts,
        ["nose", "sustainer-airframe", "interstage"]
    );
    let short = two_stage_flight("J760", "H125CT", 2.0);
    let top = short.summary().max_mach.unwrap().value;
    assert!(top > 1.2 && top < 1.3805, "{top}");
    assert!(!numbers(&short).contains(&106), "{:?}", numbers(&short));
    // Its sustainer is a flown vehicle; only its own speed keeps it out.
    assert!(numbers(&short).contains(&219));
}

/// #219 between its ends (M10.1d6, ADR-201): two 24 mm mounts 14 mm either side of the axis of
/// the buttonless synthetic design, each with an E31, one lit at launch and one 1 s after the
/// first burns out. With both loaded or both spent the center of gravity is on the axis, but
/// between the burns one motor's propellant is gone and the other's isn't: it warns, quoting
/// that offset.
#[test]
fn an_off_axis_airstart_warns_of_219_between_two_centered_ends() {
    let mut design = synthetic(None, true).design().clone();
    let airframe = &mut design.stages[0].components[1];
    let mount = airframe
        .children
        .iter()
        .find(|child| child.motor_mount.is_some())
        .unwrap()
        .clone();
    // The rings fit the one central mount; the two side by side stand in their place.
    airframe
        .children
        .retain(|child| !matches!(child.part, Part::InnerTube(_) | Part::CenteringRing(_)));
    for (id, angle_rad) in [("mount-a", 0.0), ("mount-b", std::f64::consts::PI)] {
        let mut tube = mount.clone();
        tube.id = id.to_owned();
        tube.part = Part::InnerTube(InnerTube {
            length_m: 0.1,
            outer_radius_m: 0.0125,
            thickness_m: 0.0005,
            radial_offset_m: 0.014,
            angle_rad,
            material: material("kraft_phenolic").unwrap(),
            cluster_m: Vec::new(),
        });
        airframe.children.push(tube);
    }
    let motor = Motor::from_catalog("E31").unwrap();
    let burnout_s = motor.solid_motor().burnout_time_s();
    let template = design.configurations[0].motors[0].clone();
    design.configurations[0].motors = [
        ("mount-a", Ignition::Launch),
        (
            "mount-b",
            Ignition::Time {
                time_s: burnout_s + 1.0,
            },
        ),
    ]
    .into_iter()
    .map(|(mount, ignition)| MountedMotor {
        mount: mount.to_owned(),
        designation: motor.designation().to_owned(),
        diameter_m: motor.diameter_m(),
        length_m: motor.length_m(),
        motor: motor.solid_motor().clone(),
        ignition,
        ..template.clone()
    })
    .collect();
    let id = design.configurations[0].id.clone();
    let assembly = design.assemble(&id).unwrap();
    // Both ends are on the axis, to rounding: all #219 read before it read the burns.
    for mass in [
        assembly.mass_properties(0.0),
        assembly.dry_mass_properties(),
    ] {
        let offset_m = mass.cg_m.x.hypot(mass.cg_m.y);
        assert!(offset_m < 1e-15, "{offset_m:e}");
    }
    // Between the burns the loaded motor's propellant sits 14 mm off the axis, unbalanced.
    let between =
        assembly.mass_properties_lit(burnout_s + 0.5, &[Some(0.0), Some(burnout_s + 1.0)]);
    let propellant_kg = motor.solid_motor().propellant_mass_kg(0.0);
    let expected_m = propellant_kg * 0.014 / between.mass_kg;
    let between_m = between.cg_m.x.hypot(between.cg_m.y);
    assert!(
        (between_m / expected_m - 1.0).abs() < 1e-9,
        "{between_m:e} {expected_m:e}"
    );
    let rocket = Rocket::from_design(design, &id).unwrap();
    let flight = Flight::builder(&rocket, &environment(), 1.8).fly().unwrap();
    let offset_m = match warning(&flight, 219).detail {
        Some(hpr_sim::issues::IssueDetail::CenterOfGravityOffset { offset_m }) => offset_m,
        ref other => panic!("{other:?}"),
    };
    assert!(
        (offset_m / expected_m - 1.0).abs() < 1e-9,
        "{offset_m:e} {expected_m:e}"
    );
}

/// #8 reads the wind's heights above mean sea level, the site's ellipsoidal height less the geoid's
/// undulation (M10.1d6, ADR-201). With an undulation of −25 m the site's 1,375 m above the
/// ellipsoid is 1,400 m above sea level. A near-calm turn whose lower level sits 25 m below the
/// flight's highest point, above sea level, is inside the flight and warns; read with the
/// undulation's sign the other way, the top would sit 50 m lower, below the level, and nothing
/// would.
#[test]
fn a_geoid_undulation_moves_the_flights_heights_for_its_wind() {
    use hpr_atmos::{LayeredWind, WindInterpolation, WindLevel};
    let rocket = built();
    let site = hpr_core::geodesy::Geodetic::from_degrees(32.99, -106.97, 1375.0).unwrap();
    let fly = |lower_msl_m: f64| {
        let levels = vec![
            WindLevel {
                height_msl_m: lower_msl_m,
                speed_m_s: 0.4,
                direction_from_rad: 0.0,
            },
            WindLevel {
                height_msl_m: lower_msl_m + 500.0,
                speed_m_s: 8.0,
                direction_from_rad: 1.5 * std::f64::consts::PI,
            },
        ];
        let environment = Environment::from_sim(
            hpr_sim::Environment::standard(site)
                .unwrap()
                .with_geoid_undulation_m(-25.0),
        )
        .with_wind(LayeredWind::new(levels, WindInterpolation::SpeedDirection).unwrap());
        Flight::builder(&rocket, &environment, 1.8).fly().unwrap()
    };
    // The flight's highest point above sea level: the table's lower level sits below it all the
    // way up, so the wind and the apogee hardly move with the level.
    let top_msl_m = |flight: &Flight| 1400.0 + hpr_sim::issues::highest_height_m(flight.result());
    let first = fly(1400.0);
    let lower_msl_m = top_msl_m(&first) - 25.0;
    let flight = fly(lower_msl_m);
    let top = top_msl_m(&flight);
    // Inside the flight as read, and at or past its top if the undulation were added.
    assert!(
        lower_msl_m < top && lower_msl_m >= top - 50.0,
        "{lower_msl_m} {top}"
    );
    assert!(
        (top - lower_msl_m - 25.0).abs() < 5.0,
        "{lower_msl_m} {top}"
    );
    assert!(numbers(&flight).contains(&8), "{:?}", numbers(&flight));
    // Placed past the top, it doesn't.
    let above = fly(top + 25.0);
    assert!(!numbers(&above).contains(&8), "{:?}", numbers(&above));
}
