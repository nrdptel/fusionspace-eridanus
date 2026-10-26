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
    let text = ORK.replace("NOSE", nose).replace("TUBE", tube);
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

/// What the reader says after leaving out a part of each kind (`hpr_io::ork`'s `Counted`).
const LIFT: &str = "Its mass, drag and lift aren't counted, so the mass and drag read low, and the \
                    stability margin can read high or low";
const MASS_AND_DRAG: &str = "Its mass and drag aren't counted, so both read low";
const MASS: &str = "Its mass isn't counted, so the mass reads low. The stability margin reads high \
                    if it sat aft of the center of gravity, low if ahead";
const DESCENT: &str = "Its mass and drag under canopy aren't counted, so the mass reads low and the \
                       descent rate high. The stability margin reads high if it sat aft of the \
                       center of gravity, low if ahead";

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

/// A launch lug left out: its mass and drag aren't counted, so both read low; it carries no lift.
#[test]
fn a_launch_lug_left_out_reads_the_mass_and_drag_low() {
    let lug = |radius: &str| {
        format!(
            r#"<launchlug><name>Lug</name><id>lug</id>
              <material type="bulk" density="1200.0">Polycarbonate</material>
              <axialoffset method="top">0.3</axialoffset><length>0.05</length>
              <radius>{radius}</radius><thickness>0.0005</thickness></launchlug>"#
        )
    };
    let (with, warnings, _) = read("", &lug("0.005"));
    assert!(warnings.is_empty(), "{warnings:?}");
    let (without, warnings, _) = read("", &lug("auto"));
    left_out_with(&warnings, MASS_AND_DRAG);
    assert!(without.mass_kg < with.mass_kg, "{with:?} {without:?}");
    assert!(
        without.drag_area_m2 < with.drag_area_m2,
        "{with:?} {without:?}"
    );
    assert_eq!(without.lift_slope_m2, with.lift_slope_m2);
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
            "A motor file's delays aren't flown: with no delay chosen, a flight fires no motor \
             charge, as if plugged. For a charge at burnout, choose a delay of 0 s (`hpr sim \
             --delay 0`)"
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
