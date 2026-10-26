//! Which way the numbers lean when the `.ork` reader leaves a part out, and how a motor file's
//! delay of 0 is flown (#952).
//!
//! Each test reads one made-up design twice: once with a part the reader reads, once with the same
//! part written so the reader leaves it out, by a real reason it has for that. It then checks each
//! lean the warning claims, against the rocket with the part: the mass, the drag, the normal force,
//! the center of gravity and the stability margin of the structure, and the descent rate of a
//! flight. The warning's own words are checked too, so the words and the numbers can't drift apart.

use hpr_aero::{AeroModel, DragConditions, Flow};
use hpr_design::{Configuration, Ignition, MountedMotor};
use hpr_motor::{Delay, eng};

use super::environment;
use crate::{Flight, Motor, Rocket};

/// A made-up rocket, as OpenRocket 24.12 writes one: a hollow 150 mm ogive on a 600 mm cardboard
/// tube 33 mm across, with three fins at its aft end and the tube a motor mount with no motor.
/// `NOSE` and `TUBE` are where a test puts the parts inside the nose cone and on or inside the
/// tube.
const ORK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<openrocket version="1.10" creator="OpenRocket 24.12">
  <rocket><name>Left out</name>
    <subcomponents>
      <stage><name>Stage</name><id>stage</id><subcomponents>
        <nosecone><name>Nose</name><id>nose</id>
          <material type="bulk" density="1000.0">Plastic</material>
          <length>0.15</length><thickness>0.002</thickness><shape>ogive</shape>
          <aftradius>0.0165</aftradius><subcomponents>NOSE</subcomponents></nosecone>
        <bodytube><name>Tube</name><id>tube</id>
          <material type="bulk" density="680.0">Cardboard</material>
          <length>0.6</length><thickness>0.001</thickness><radius>0.0165</radius>
          <motormount><ignitionevent>automatic</ignitionevent><ignitiondelay>0.0</ignitiondelay>
            <overhang>0.0</overhang></motormount>
          <subcomponents><trapezoidfinset><name>Fins</name><id>fins</id>
            <material type="bulk" density="680.0">Plywood</material>
            <axialoffset method="bottom">0.0</axialoffset><instancecount>3</instancecount>
            <rootchord>0.08</rootchord><tipchord>0.04</tipchord><height>0.05</height>
            <sweeplength>0.03</sweeplength><thickness>0.003</thickness>
            <crosssection>rounded</crosssection></trapezoidfinset>TUBE</subcomponents>
        </bodytube></subcomponents></stage>
    </subcomponents></rocket>
</openrocket>"#;

/// The Mach number the aerodynamics are read at, with the air along the axis.
const MACH: f64 = 0.3;

/// The Reynolds number per meter at [`MACH`] near sea level, 1/m: about 102 m/s over a kinematic
/// viscosity of 1.46e-5 m²/s.
const REYNOLDS_PER_M: f64 = 7.0e6;

/// The rocket's numbers that a part left out can move.
#[derive(Debug, Clone, Copy)]
struct Numbers {
    /// The structure's mass, kg.
    mass_kg: f64,
    /// The structure's center of gravity, m aft of the nose tip.
    cg_station_m: f64,
    /// The zero-lift drag at [`MACH`] as an area, `C_D · A_ref`, m², so that a change in the
    /// reference area can't pass for one in the drag.
    drag_area_m2: f64,
    /// The normal-force slope as an area, `C_Nα · A_ref`, m² per radian.
    lift_slope_m2: f64,
    /// The static margin of the structure, calibers.
    margin_cal: f64,
    /// How many recovery devices the design holds.
    devices: usize,
}

/// [`ORK`] with `nose` inside its nose cone and `tube` on and inside its tube, read: its numbers,
/// the reader's warnings, and the design.
fn read(nose: &str, tube: &str) -> (Numbers, Vec<String>, hpr_io::ork::Design) {
    read_text(&ORK.replace("NOSE", nose).replace("TUBE", tube))
}

/// A `.ork` document's text, read as [`read`] reads one.
fn read_text(text: &str) -> (Numbers, Vec<String>, hpr_io::ork::Design) {
    let file = hpr_io::ork::read(text.as_bytes()).unwrap();
    let design = hpr_io::ork::design(&file.value);
    let warnings = design.warnings.iter().map(|w| w.message.clone()).collect();
    let design = design.value;
    let layout = design.rocket.layout().unwrap();
    let aero = AeroModel::new(&layout).unwrap();
    let area_m2 = aero.reference_area_m2();
    let drag = aero
        .drag(
            &Flow::axial(MACH),
            &DragConditions::coasting(REYNOLDS_PER_M),
        )
        .unwrap();
    let lift = aero.normal_force(&Flow::new(MACH, 0.0, 0.0)).unwrap();
    let cg_station_m = -layout.structure.cg_m.z;
    let cp_station_m = lift.cp_station_m.unwrap();
    let numbers = Numbers {
        mass_kg: layout.structure.mass_kg,
        cg_station_m,
        drag_area_m2: drag.zero_lift_coefficient * area_m2,
        lift_slope_m2: lift.slope_per_rad * area_m2,
        margin_cal: (cp_station_m - cg_station_m) / layout.reference_diameter_m,
        devices: design.recovery.devices.len(),
    };
    (numbers, warnings, design)
}

/// The one warning of `warnings` that says a part was left out, which must end in `sentence`.
fn left_out_with(warnings: &[String], sentence: &str) {
    let left: Vec<&String> = warnings.iter().filter(|w| w.contains("left out")).collect();
    assert_eq!(left.len(), 1, "{warnings:?}");
    assert!(left[0].ends_with(sentence), "{}", left[0]);
}

/// The station of the center of the part with id `id` in `design`, m aft of the nose tip, and
/// its mass, kg.
fn part(design: &hpr_io::ork::Design, id: &str) -> (f64, f64) {
    let layout = design.rocket.layout().unwrap();
    let (_, placed) = layout.find(id).unwrap();
    (-placed.own.cg_m.z, placed.own.mass_kg)
}

/// What the reader says after leaving out a part of each kind, with nothing else changing
/// (`hpr_io::ork`'s `Lean`).
const LIFT: &str = "Its mass, drag and lift aren't counted, so the mass and drag read low. The \
                    stability margin can read high or low";
const MASS_AND_DRAG: &str = "Its mass and drag aren't counted, so the mass and drag read low. The \
                             stability margin reads high for a part aft of the center of gravity, \
                             low for one ahead";
const MASS: &str = "Its mass isn't counted, so the mass reads low. The stability margin reads high \
                    for a part aft of the center of gravity, low for one ahead";
const DESCENT: &str = "Its mass and drag under canopy aren't counted, so the mass reads low. The \
                       descent rate reads high if it opens in flight, low if it doesn't. The \
                       stability margin reads high for a part aft of the center of gravity, low \
                       for one ahead";

/// A freeform fin set of three, `place` on the tube, its outline's second point `y`: a number it
/// reads, or one it can't, which leaves the set out.
fn fins(place: &str, y: &str) -> String {
    format!(
        r#"<freeformfinset><name>Extra fins</name><id>extra</id>
          <material type="bulk" density="130.0">Balsa</material>
          <axialoffset method="{place}">0.0</axialoffset><instancecount>3</instancecount>
          <thickness>0.002</thickness><finpoints><point x="0.0" y="0.0"/>
          <point x="0.03" y="{y}"/><point x="0.06" y="0.04"/><point x="0.08" y="0.0"/>
          </finpoints></freeformfinset>"#
    )
}

/// A fin set left out: its mass, drag and lift aren't counted, so the mass, the drag and the
/// normal force read low, and the margin moves either way. Light fins at the aft end left out
/// read it low; the same fins at the front, canards, read it high. (Heavy fins at the aft end
/// can read it high too: their mass moves the center of gravity further than their lift moves
/// the center of pressure.)
#[test]
fn a_fin_set_left_out_reads_the_mass_drag_and_lift_low_and_the_margin_either_way() {
    for (place, margin_reads_high) in [("bottom", false), ("top", true)] {
        let (with, warnings, _) = read("", &fins(place, "0.04"));
        assert!(warnings.is_empty(), "{warnings:?}");
        let (without, warnings, _) = read("", &fins(place, "four"));
        left_out_with(&warnings, LIFT);
        assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
        assert!(
            without.drag_area_m2 < with.drag_area_m2,
            "{with:?} {without:?}"
        );
        assert!(
            without.lift_slope_m2 < with.lift_slope_m2,
            "{with:?} {without:?}"
        );
        assert_eq!(
            without.margin_cal > with.margin_cal,
            margin_reads_high,
            "{place}: {with:?} {without:?}"
        );
    }
}

/// A launch lug left out: its mass and drag aren't counted, so the mass and drag read low. It
/// carries no lift, so the center of pressure stays, and the margin moves as its mass does: high
/// for a lug aft of the center of gravity, low for one ahead.
#[test]
fn a_launch_lug_left_out_reads_the_mass_and_drag_low_and_the_margin_by_where_it_sat() {
    let lug = |end: &str, radius: &str| {
        format!(
            r#"<launchlug><name>Lug</name><id>lug</id>
              <material type="bulk" density="1200.0">Polycarbonate</material>
              <axialoffset method="{end}">0.0</axialoffset><length>0.05</length>
              <radius>{radius}</radius><thickness>0.0005</thickness></launchlug>"#
        )
    };
    for (end, aft) in [("top", false), ("bottom", true)] {
        let (with, warnings, design) = read("", &lug(end, "0.005"));
        assert!(warnings.is_empty(), "{warnings:?}");
        let (without, warnings, _) = read("", &lug(end, "auto"));
        left_out_with(&warnings, MASS_AND_DRAG);
        let (station_m, _) = part(&design, "lug");
        assert_eq!(station_m > with.cg_station_m, aft, "{station_m} {with:?}");
        assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
        assert!(
            without.drag_area_m2 < with.drag_area_m2,
            "{with:?} {without:?}"
        );
        assert_eq!(without.lift_slope_m2, with.lift_slope_m2);
        assert_eq!(
            without.margin_cal > with.margin_cal,
            aft,
            "{end}: {with:?} {without:?}"
        );
    }
}

/// A part inside the body left out: only its mass isn't counted. The center of gravity moves away
/// from where it sat, so the margin reads high for one aft of the center of gravity and low for
/// one ahead of it; the drag and the normal force don't move.
#[test]
fn a_part_inside_left_out_reads_the_mass_low_and_the_margin_by_where_it_sat() {
    let inner = |offset_m: f64, thickness: &str| {
        format!(
            r#"<innertube><name>Inner</name><id>inner</id>
              <material type="bulk" density="680.0">Cardboard</material>
              <axialoffset method="top">{offset_m}</axialoffset><length>0.1</length>
              <outerradius>0.012</outerradius><thickness>{thickness}</thickness></innertube>"#
        )
    };
    for offset_m in [0.0, 0.4] {
        let (with, warnings, design) = read("", &inner(offset_m, "0.001"));
        assert!(warnings.is_empty(), "{warnings:?}");
        let (without, warnings, _) = read("", &inner(offset_m, "-0.001"));
        left_out_with(&warnings, MASS);
        let (station_m, mass_kg) = part(&design, "inner");
        assert!(mass_kg > 0.0);
        let aft = station_m > with.cg_station_m;
        // Forward of the center of gravity at the top of the tube, aft of it lower down.
        assert_eq!(aft, offset_m > 0.0, "{station_m} {with:?}");
        assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
        // x_cg' − x_cg = m (x_cg − x_p)/(M − m): away from the part.
        let moved_m = without.cg_station_m - with.cg_station_m;
        let expected_m = mass_kg * (with.cg_station_m - station_m) / (with.mass_kg - mass_kg);
        assert!(
            (moved_m - expected_m).abs() < 1e-12,
            "{moved_m} {expected_m}"
        );
        assert_eq!(
            without.margin_cal > with.margin_cal,
            aft,
            "{with:?} {without:?}"
        );
        assert_eq!(without.drag_area_m2, with.drag_area_m2);
        assert_eq!(without.lift_slope_m2, with.lift_slope_m2);
    }
}

/// A parachute in the nose cone, packed `radius`: a number, or `auto`, which a nose cone has no
/// bore to give, and which leaves it out. It opens at apogee.
fn chute(radius: &str, event: &str) -> String {
    format!(
        r#"<parachute><name>Chute</name><id>chute</id>
          <axialoffset method="bottom">0.0</axialoffset><packedlength>0.03</packedlength>
          <packedradius>{radius}</packedradius><cd>auto</cd>
          <material type="surface" density="0.067">Ripstop nylon</material>
          <deployevent>{event}</deployevent><deploydelay>0.0</deploydelay>
          <diameter>0.45</diameter></parachute>"#
    )
}

/// A made-up 29 mm motor's `.eng` text, its delays listed as `delays`.
fn eng_text(delays: &str) -> String {
    format!(
        "; A made-up motor for the tests.\n\
         T15 29 114 {delays} 0.06 0.1 Test\n\
         0.1 25.0\n1.0 15.0\n3.0 15.0\n3.2 0.0\n;\n"
    )
}

/// `design`'s rocket with `motor` in its tube, lit at launch, as `hpr sim --motor` puts one in, and
/// the file's recovery devices.
fn flown(design: &hpr_io::ork::Design, motor: &Motor) -> (Rocket, crate::ork::Recovered) {
    let mut rocket = design.rocket.clone();
    rocket.configurations = vec![Configuration {
        id: "mine".to_owned(),
        name: String::new(),
        motors: vec![MountedMotor {
            mount: "tube".to_owned(),
            designation: motor.designation().to_owned(),
            diameter_m: motor.diameter_m(),
            length_m: motor.length_m(),
            motor: motor.solid_motor().clone(),
            delay: motor.delay(),
            ignition: Ignition::Launch,
            failed_tubes: Vec::new(),
        }],
    }];
    let assembly = rocket.assemble("mine").unwrap();
    let recovered = crate::ork::recovery(&design.recovery, &rocket, &assembly).unwrap();
    let mut flown = Rocket::from_design(rocket, "mine").unwrap();
    for device in &recovered.devices {
        flown.add_parachute(device.clone());
    }
    (flown, recovered)
}

/// A parachute left out: its mass and its drag under canopy aren't counted, so the mass reads low
/// and the descent rate high. It sat in the nose, ahead of the center of gravity, so the margin
/// reads low.
#[test]
fn a_parachute_left_out_reads_the_mass_low_and_the_descent_rate_high() {
    let (with, warnings, design) = read(&chute("0.008", "apogee"), "");
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, left) = read(&chute("auto", "apogee"), "");
    left_out_with(&warnings, DESCENT);
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
    let (station_m, _) = part(&design, "chute");
    assert!(station_m < with.cg_station_m, "{station_m} {with:?}");
    assert!(without.margin_cal < with.margin_cal, "{with:?} {without:?}");
    assert_eq!((with.devices, without.devices), (1, 0));
    assert_eq!(without.drag_area_m2, with.drag_area_m2);

    let motor = Motor::from_eng(&eng_text("6"))
        .unwrap()
        .with_delay_s(6.0)
        .unwrap();
    // The reader keeps the device of a part it left out as one it doesn't fly, and `hpr sim`
    // flies no design with a part left out. The rocket as read flies without it.
    assert_eq!(left.recovery.unread.len(), 1);
    let descent = |design: &hpr_io::ork::Design| {
        let mut design = design.clone();
        design.recovery.unread.clear();
        let (rocket, _) = flown(&design, &motor);
        let flight = Flight::builder(&rocket, &environment(), 1.5).fly().unwrap();
        flight.landing().unwrap().descent_rate_m_s
    };
    let (under_canopy, falling) = (descent(&design), descent(&left));
    assert!(falling > under_canopy, "{under_canopy} {falling}");
}

/// A motor file's delay of 0 isn't flown: with no delay chosen, no motor charge fires, and a
/// device set to open on one doesn't open. Chosen as 0 s, the charge fires at burnout. The
/// warning says both (`hpr_motor::delay`).
#[test]
fn a_delay_of_0_flies_no_charge_until_one_is_chosen() {
    let text = eng_text("0");
    let entry = eng::parse(&text).unwrap().value.entries.remove(0);
    let delays = entry.delays();
    assert_eq!(delays.delays, [Delay::ZeroOrPlugged]);
    let warning = &delays.warnings[0].message;
    assert!(
        warning.ends_with(
            "A motor file's delays aren't flown: with none chosen, no motor charge fires, as if \
             plugged. For a charge at burnout, choose a delay of 0 (`hpr sim --delay 0`, or \
             `delay_s=0` in Python)"
        ),
        "{warning}"
    );

    let (_, _, design) = read(&chute("0.008", "ejection"), "");
    let motor = Motor::from_eng(&text).unwrap();
    assert_eq!(motor.delay(), None);
    let (rocket, recovered) = flown(&design, &motor);
    assert!(recovered.devices.is_empty());
    assert_eq!(
        recovered.not_deployed,
        [(
            "Chute".to_owned(),
            crate::ork::NeverDeploys::NoEjectionCharge
        )]
    );
    let flight = Flight::builder(&rocket, &environment(), 1.5).fly().unwrap();
    assert!(
        flight
            .result()
            .event(hpr_sim::EventKind::Deployment(0))
            .is_none()
    );

    let (rocket, recovered) = flown(&design, &motor.with_delay_s(0.0).unwrap());
    assert_eq!(recovered.devices.len(), 1);
    let flight = Flight::builder(&rocket, &environment(), 1.5).fly().unwrap();
    let events = flight.result();
    let opened_s = events
        .event(hpr_sim::EventKind::Deployment(0))
        .unwrap()
        .sample
        .time_s;
    let burnout_s = events
        .event(hpr_sim::EventKind::Burnout)
        .unwrap()
        .sample
        .time_s;
    assert!(
        (opened_s - burnout_s).abs() < 1e-9,
        "{opened_s} {burnout_s}"
    );
}

/// A finish the writer can't name reads back as regular paint, 60 µm: a smoother one's friction
/// drag then reads high, a rougher one's low (the export's warning).
#[test]
fn a_finish_read_back_as_paint_moves_the_friction_drag_the_way_the_warning_says() {
    // Neither is one of OpenRocket's five finishes (20 µm is its smooth paint).
    for (roughness_m, reads_high) in [(10e-6, true), (100e-6, false)] {
        let (_, _, mut design) = read("", "");
        for component in &mut design.rocket.stages[0].components {
            component.finish = Some(hpr_design::Finish::Custom { roughness_m });
        }
        let drag_area_m2 = |design: &hpr_io::ork::Design| {
            let layout = design.rocket.layout().unwrap();
            let aero = AeroModel::new(&layout).unwrap();
            aero.drag(
                &Flow::axial(MACH),
                &DragConditions::coasting(REYNOLDS_PER_M),
            )
            .unwrap()
            .zero_lift_coefficient
                * aero.reference_area_m2()
        };
        let written = hpr_io::ork::export::write(&design, &[]).unwrap();
        let ending = if reads_high {
            "can read high"
        } else {
            "can read low"
        };
        assert!(
            written
                .warnings
                .iter()
                .any(|w| w.message.ends_with(&format!("friction drag {ending}"))),
            "{:?}",
            written.warnings
        );
        let back = hpr_io::ork::design(&hpr_io::ork::read(&written.value).unwrap().value).value;
        assert_eq!(
            drag_area_m2(&back) > drag_area_m2(&design),
            reads_high,
            "{roughness_m}"
        );
    }
}

/// An inner tube, `offset_m` from the top of the body tube, 100 mm long and 24 mm across, with a
/// wall of `thickness` (a negative one leaves it out) and `inside` in it.
fn inner_tube(offset_m: f64, thickness: &str, inside: &str) -> String {
    format!(
        r#"<innertube><name>Inner</name><id>inner</id>
          <material type="bulk" density="680.0">Cardboard</material>
          <axialoffset method="top">{offset_m}</axialoffset><length>0.1</length>
          <outerradius>0.012</outerradius><thickness>{thickness}</thickness>
          <subcomponents>{inside}</subcomponents></innertube>"#
    )
}

/// A tube a centering ring takes its bore from, left out: each ring closes to a solid disc, so
/// the mass reads *high* here, against "the mass reads low" for a tube alone, and the margin
/// reads low for a tube aft of the center of gravity, against "high" for a tube alone. The
/// warning gives the mass and margin no direction, and says why.
#[test]
fn a_tube_a_ring_takes_its_bore_from_gives_the_mass_and_margin_no_direction() {
    let ring = |name: &str, offset_m: f64| {
        format!(
            r#"<centeringring><name>{name}</name><id>{name}</id>
              <material type="bulk" density="630.0">Plywood</material>
              <axialoffset method="top">{offset_m}</axialoffset><length>0.012</length>
              <outerradius>auto</outerradius><innerradius>auto</innerradius></centeringring>"#
        )
    };
    let rings = format!("{}{}", ring("fore", 0.4), ring("aft", 0.488));
    let (with, warnings, design) = read("", &format!("{}{rings}", inner_tube(0.4, "0.001", "")));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read("", &format!("{}{rings}", inner_tube(0.4, "-0.001", "")));
    left_out_with(
        &warnings,
        "Its mass isn't counted. The mass and stability margin can read high or low, as parts \
         sized from it change too",
    );
    // The rings' bores close: the rocket without the tube is heavier than with it, and its
    // center of gravity moves toward the tube, not away.
    assert!(without.mass_kg > with.mass_kg, "{with:?} {without:?}");
    let (station_m, _) = part(&design, "inner");
    assert!(station_m > with.cg_station_m, "{station_m} {with:?}");
    assert!(without.margin_cal < with.margin_cal, "{with:?} {without:?}");
}

/// A part under a body tube whose override sets the mass and the center of gravity of all it
/// holds, left out: neither moves, to the bit, and the warning says the override sets them.
#[test]
fn a_part_an_override_above_covers_leaves_the_mass_and_center_of_gravity_as_set() {
    let overridden = |tube: &str| {
        ORK.replace("NOSE", "").replace("TUBE", tube).replace(
            "<length>0.6</length><thickness>0.001</thickness><radius>0.0165</radius>",
            "<length>0.6</length><thickness>0.001</thickness><radius>0.0165</radius>\
             <overridemass>0.2</overridemass><overridesubcomponentsmass>true\
             </overridesubcomponentsmass><overridecg>0.3</overridecg>\
             <overridesubcomponentscg>true</overridesubcomponentscg>",
        )
    };
    let (with, warnings, _) = read_text(&overridden(&inner_tube(0.4, "0.001", "")));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read_text(&overridden(&inner_tube(0.4, "-0.001", "")));
    left_out_with(
        &warnings,
        "Its mass isn't counted. The mass and center of gravity stay as the body tube's override \
         sets them",
    );
    assert_eq!(without.mass_kg, with.mass_kg);
    assert_eq!(without.cg_station_m, with.cg_station_m);
    assert_eq!(without.margin_cal, with.margin_cal);
}

/// Two fins with a tab `tab_height` deep, at the aft end of their tube: on a pod's tube of no
/// length (`pod`), or on the body tube.
fn tabbed_fins(tab_height: &str, pod: bool) -> String {
    let fins = format!(
        r#"<trapezoidfinset><name>Winglets</name><id>winglets</id><fincount>2</fincount>
          <material type="bulk" density="680.0">Plywood</material>
          <rootchord>0.03</rootchord><tipchord>0.015</tipchord><height>0.02</height>
          <sweeplength>0.01</sweeplength><thickness>0.003</thickness>
          <tabheight>{tab_height}</tabheight><tablength>0.02</tablength>
          <axialoffset method="bottom">0.0</axialoffset></trapezoidfinset>"#
    );
    if !pod {
        return fins;
    }
    format!(
        r#"<podset><name>Pods</name><id>pods</id><instancecount>1</instancecount>
          <radiusoffset method="relative">0.01</radiusoffset>
          <axialoffset method="top">0.0</axialoffset><subcomponents>
          <bodytube><name>Flat</name><id>flat</id><length>0.0</length><thickness>0.0</thickness>
            <radius>0.0</radius><subcomponents>{fins}</subcomponents></bodytube>
          </subcomponents></podset>"#
    )
}

/// A fin tab left out: only its mass isn't counted, inside the fins' root. A tab on a pod's tube
/// of no length has no wall to sit in, so the reader leaves it out and says so. No rocket holds
/// that tab, so its lean is measured on the same fins on the body tube, where the tab is read: with
/// it, the mass reads higher, and the drag and the normal force are the same to the bit. The tab
/// sat aft of the center of gravity, so the margin reads high without it.
#[test]
fn a_fin_tab_left_out_reads_the_mass_low_and_leaves_the_drag_and_lift() {
    let (_, warnings, _) = read("", &tabbed_fins("0.005", true));
    left_out_with(&warnings, MASS);
    assert!(
        warnings
            .iter()
            .any(|w| w.starts_with("a tab on the fins `winglets` on a pod's tube of no length")),
        "{warnings:?}"
    );

    let (with, warnings, design) = read("", &tabbed_fins("0.005", false));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read("", &tabbed_fins("0.0", false));
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
    assert_eq!(without.drag_area_m2, with.drag_area_m2);
    assert_eq!(without.lift_slope_m2, with.lift_slope_m2);
    let (station_m, _) = part(&design, "winglets");
    assert!(station_m > with.cg_station_m, "{station_m} {with:?}");
    assert!(without.margin_cal > with.margin_cal, "{with:?} {without:?}");
}

/// A pod set holding a tube with a parachute in it, left out by hanging it from the nose cone:
/// the warning says its lift and its canopy, and the mass, drag and normal force all read low
/// against the same pod set on the body tube.
#[test]
fn a_pod_set_holding_a_parachute_says_its_lift_and_its_canopy() {
    let pods = r#"<podset><name>Pods</name><id>pods</id><instancecount>2</instancecount>
          <radiusoffset method="relative">0.0</radiusoffset>
          <axialoffset method="top">0.0</axialoffset><subcomponents>
          <nosecone><name>Pod nose</name><id>pod-nose</id><length>0.03</length>
            <material type="bulk" density="1000.0">Plastic</material>
            <thickness>0.001</thickness><shape>ogive</shape><aftradius>0.01</aftradius></nosecone>
          <bodytube><name>Pod</name><id>pod</id><length>0.1</length><thickness>0.001</thickness>
            <material type="bulk" density="680.0">Cardboard</material>
            <radius>0.01</radius><subcomponents>
            <parachute><name>Pod chute</name><id>pod-chute</id>
              <axialoffset method="top">0.0</axialoffset><packedlength>0.03</packedlength>
              <packedradius>0.008</packedradius><cd>auto</cd>
              <material type="surface" density="0.067">Ripstop nylon</material>
              <deployevent>apogee</deployevent><diameter>0.3</diameter></parachute>
          </subcomponents></bodytube></subcomponents></podset>"#;
    let (with, _, _) = read("", pods);
    let (without, warnings, _) = read(pods, "");
    let left: Vec<&String> = warnings.iter().filter(|w| w.contains("left out")).collect();
    assert_eq!(
        left,
        [
            "a pod set hangs from a nose cone, and HPR Sim hangs one only from a body tube; it was \
          left out, and the 3 parts inside it. Their mass, drag, lift and drag under canopy aren't \
          counted, so the mass and drag read low. The descent rate reads high if one of them \
          opens in flight, low if none does. The stability margin can read high or low"
        ],
        "{warnings:?}"
    );
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
    assert!(
        without.drag_area_m2 < with.drag_area_m2,
        "{with:?} {without:?}"
    );
    assert!(
        without.lift_slope_m2 < with.lift_slope_m2,
        "{with:?} {without:?}"
    );
}

/// A parachute and a mass packed together, `offset_m` from the top of their parent.
fn packed(offset_m: f64) -> String {
    format!(
        r#"<parachute><name>Inner chute</name><id>inner-chute</id>
          <axialoffset method="top">{offset_m}</axialoffset><packedlength>0.03</packedlength>
          <packedradius>0.008</packedradius><cd>auto</cd>
          <material type="surface" density="0.067">Ripstop nylon</material>
          <deployevent>apogee</deployevent><diameter>0.3</diameter></parachute>
        <masscomponent><name>Weight</name><id>weight</id><mass>0.01</mass>
          <axialoffset method="top">{offset_m}</axialoffset><packedlength>0.01</packedlength>
          <packedradius>0.005</packedradius></masscomponent>"#
    )
}

/// A tube left out with a parachute and a mass inside it: the warning names the 2 parts and says
/// what all three carried, the canopy among it. The tube sat aft of the center of gravity, so the
/// margin reads high, and the parachute goes too.
#[test]
fn a_part_left_out_with_parts_inside_says_what_they_all_carried() {
    let (with, warnings, _) = read("", &inner_tube(0.4, "0.001", &packed(0.0)));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read("", &inner_tube(0.4, "-0.001", &packed(0.0)));
    let left: Vec<&String> = warnings.iter().filter(|w| w.contains("left out")).collect();
    assert_eq!(
        left,
        [
            "a tube of negative wall thickness; it was left out, and the 2 parts inside it. Their \
          mass and drag under canopy aren't counted, so the mass reads low. The descent rate \
          reads high if one of them opens in flight, low if none does. The stability margin \
          reads high for a part aft of the center of gravity, low for one ahead"
        ],
        "{warnings:?}"
    );
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
    assert!(without.margin_cal > with.margin_cal, "{with:?} {without:?}");
    assert_eq!((with.devices, without.devices), (1, 0));
}

/// Parts written inside a part that holds none in HPR Sim, a mass component, are left out, and
/// it stays: the warning names the 2 parts and what they carried, and the mass reads low against
/// the same parts beside it.
#[test]
fn parts_inside_a_part_that_holds_none_say_what_they_carried() {
    let weight = |inside: &str| {
        format!(
            r#"<masscomponent><name>Ballast</name><id>ballast</id><mass>0.02</mass>
              <axialoffset method="top">0.0</axialoffset><packedlength>0.02</packedlength>
              <packedradius>0.01</packedradius><subcomponents>{inside}</subcomponents>
              </masscomponent>"#
        )
    };
    let (with, warnings, _) = read("", &format!("{}{}", weight(""), packed(0.0)));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read("", &weight(&packed(0.0)));
    let left: Vec<&String> = warnings.iter().filter(|w| w.contains("left out")).collect();
    assert_eq!(
        left,
        [
            "a mass component holds no parts in HPR Sim, so the 2 parts inside it were left out. \
          Their mass and drag under canopy aren't counted, so the mass reads low. The descent \
          rate reads high if one of them opens in flight, low if none does. The stability margin \
          reads high for a part aft of the center of gravity, low for one ahead"
        ],
        "{warnings:?}"
    );
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
}

/// A `.ork` motor whose `<delay>` can't be read has none: it fires no ejection charge, so a
/// parachute set to open at the charge never opens, as the warning says. The warning itself means
/// no configuration of the file is flown as read (ADR-055), so the motor as read, with no delay,
/// is put in the same rocket read with a delay of 6 s, where the parachute opens.
#[test]
fn an_unreadable_ork_delay_fires_no_charge_and_says_so() {
    let with_motor = |delay: &str| {
        ORK.replace("NOSE", &chute("0.008", "ejection"))
            .replace("TUBE", "")
            .replace(
                "<rocket><name>Left out</name>",
                r#"<rocket><name>Left out</name><motorconfiguration configid="c"/>"#,
            )
            .replace(
                "<overhang>0.0</overhang></motormount>",
                &format!(
                    r#"<overhang>0.0</overhang><motor configid="c"><type>single</type>
                      <manufacturer>Estes</manufacturer><designation>F15</designation>
                      <diameter>0.029</diameter><length>0.114</length><delay>{delay}</delay>
                      </motor></motormount>"#
                ),
            )
    };
    let read = |text: &str| {
        let file = hpr_io::ork::read(text.as_bytes()).unwrap();
        hpr_io::ork::design(&file.value)
    };
    let recovered = |design: &hpr_io::ork::Design| {
        let assembly = design.rocket.assemble("c").unwrap();
        crate::ork::recovery(&design.recovery, &design.rocket, &assembly).unwrap()
    };
    let six = read(&with_motor("6.0"));
    assert!(six.warnings.is_empty(), "{:?}", six.warnings);
    assert_eq!(recovered(&six.value).devices.len(), 1);

    let soon = read(&with_motor("soon"));
    let warnings: Vec<&str> = soon.warnings.iter().map(|w| w.message.as_str()).collect();
    assert_eq!(
        warnings,
        [
            "`delay` says `soon`, which is neither `none` nor a delay; it was left out. This motor \
          fires no ejection charge, as if plugged, so a device set to open at its stage's charge \
          opens only on another motor's"
        ],
    );
    let configuration = &soon.value.motors.configurations[0];
    assert_eq!(configuration.motors[0].delay, None);
    assert!(configuration.left_out.is_some());
    let mut as_read = six.value.clone();
    as_read.rocket.configurations[0].motors[0].delay = configuration.motors[0].delay;
    let never = recovered(&as_read);
    assert!(never.devices.is_empty());
    assert_eq!(
        never.not_deployed,
        [(
            "Chute".to_owned(),
            crate::ork::NeverDeploys::NoEjectionCharge
        )]
    );
}
