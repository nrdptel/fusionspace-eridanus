//! The fit of a flight to its log ([`crate::fit`]): M7.3f.

use hpr_design::Part;
use hpr_flightdata::log::{Channel as LogChannel, FlightLog, LogFormat, Quantity, Stated};
use hpr_flightdata::readings::{Provenance, Source};
use hpr_forensics::residuals::HeightKind;
use hpr_forensics::synthetic::Set;
use hpr_forensics::synthetic::fits::{ASKED, Asked, KNOWN_GAPS, known_gap_of};
use hpr_sim::{Channel, Recorder};

use super::{built, environment, trapezoid};
use crate::fit::{
    self, Caveat, INTERVAL_ISSUE_URL, LogFit, NoInterval, Parameter, Refused, WindPastScan,
};
use crate::rocket::{Fins, MotorTube, Nose, Transition, Tube, material};
use crate::{
    CanopyType, Device, DeviceDrag, Environment, FinCrossSection, Flight, FlightBuilder, Motor,
    NoseShape, Rocket, Trigger,
};

/// The mean radius the fit turns a fix's angles into meters by, m (IUGG R₁).
const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// A parachute opened at apogee, so that a log's climb is the fit's: nothing open on the way up.
fn at_apogee() -> Device {
    Device::new(
        "parachute",
        DeviceDrag::canopy(CanopyType::FlatCircular, 0.9),
        Trigger::Apogee,
    )
}

/// A log of `launch`'s own flight with its drag scaled by `drag_scale`: its height above the
/// pad every 0.05 s, after 10 s on the pad and with 10 s on the ground at the end. With `fixes`,
/// its latitude and longitude too, about the site at (32.99°, −106.97°).
fn logged(launch: &FlightBuilder<'_>, drag_scale: f64, fixes: bool) -> FlightLog {
    let pad_s = 10.0;
    let mut inputs = launch.inputs().unwrap();
    inputs.drag_scale = drag_scale;
    let mut recorder = Recorder::new(
        vec![Channel::Time, Channel::HeightAboveGround, Channel::Position],
        Some(0.05),
    )
    .unwrap();
    inputs.simulation().unwrap().run(&mut recorder).unwrap();
    let rows = recorder.rows();
    let first = rows[0].clone();
    let last = rows[rows.len() - 1].clone();
    let mut time_s = Vec::new();
    let mut samples = Vec::new();
    for step in 0..200 {
        time_s.push(f64::from(step) * 0.05);
        samples.push((0.0, 0.0, 0.0));
    }
    for row in rows {
        time_s.push(row[0] + pad_s);
        samples.push((row[1] - first[1], row[2] - first[2], row[3] - first[3]));
    }
    for step in 1..=200 {
        time_s.push(last[0] + pad_s + f64::from(step) * 0.05);
        samples.push((last[1] - first[1], last[2] - first[2], last[3] - first[3]));
    }
    let (latitude_deg, longitude_deg) = (32.99_f64, -106.97_f64);
    let channels = if fixes {
        let latitude = samples
            .iter()
            .map(|&(_, _, north)| latitude_deg + (north / EARTH_RADIUS_M).to_degrees())
            .collect();
        let longitude = samples
            .iter()
            .map(|&(_, east, _)| {
                longitude_deg
                    + (east / (EARTH_RADIUS_M * latitude_deg.to_radians().cos())).to_degrees()
            })
            .collect();
        vec![
            LogChannel::new("latitude", Quantity::Latitude, time_s.clone(), latitude),
            LogChannel::new("longitude", Quantity::Longitude, time_s.clone(), longitude),
        ]
    } else {
        Vec::new()
    };
    FlightLog {
        format: LogFormat::GenericTable,
        logger: "a test's".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        altitude_m: samples.iter().map(|sample| sample.0).collect(),
        time_s,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes: Vec::new(),
    }
}

fn numbers(fitted: &LogFit) -> Vec<u32> {
    fitted
        .issue_warnings
        .iter()
        .map(hpr_sim::issues::IssueWarning::number)
        .collect()
}

fn row(fitted: &LogFit, parameter: Parameter) -> fit::Fitted {
    *fitted.parameter(parameter).unwrap()
}

fn holds(fitted: &LogFit, parameter: Parameter, truth: f64) -> bool {
    let interval = row(fitted, parameter).interval_90.unwrap();
    interval.low <= truth && truth <= interval.high
}

/// What a test rocket's tail carries behind its fins.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tail {
    /// Nothing: the tube ends the rocket.
    Plain,
    /// A boattail of 15.2°, steeper than #72's 10°.
    SteepBoattail,
    /// A shallow boattail and a lip behind it longer than the boattail's drop in diameter (#120).
    LongLip,
    /// A boattail that starts a millimeter inside the tube: a step in radius (#87).
    Step,
}

/// A 54 mm rocket on an I377 with this nose, these fins' edges and this tail. Its fins span
/// 90 mm.
fn shaped(nose: NoseShape, nose_length_m: f64, edges: FinCrossSection, tail: Tail) -> Rocket {
    let mut rocket = Rocket::new("A 54 mm test rocket", 0.0563).unwrap();
    rocket
        .add_nose(
            Nose::hollow(nose, nose_length_m, 0.0015, material("abs").unwrap())
                .with_capped_shoulder(0.06, 0.0015),
        )
        .unwrap()
        .add_tube(Tube::new(0.8, 0.00115, material("kraft_phenolic").unwrap()))
        .unwrap()
        .add_motor_tube(MotorTube::new(
            0.3,
            0.038,
            0.001,
            material("kraft_phenolic").unwrap(),
        ))
        .unwrap()
        .add_fins(
            Fins::new(
                3,
                trapezoid([0.12, 0.05, 0.09, 0.07]),
                0.003175,
                material("birch_plywood").unwrap(),
            )
            .with_cross_section(edges),
        )
        .unwrap();
    let cone = |length_m: f64, aft_diameter_m: f64| {
        Transition::conical(length_m, aft_diameter_m, 0.0015, material("abs").unwrap())
    };
    match tail {
        Tail::Plain => {}
        Tail::SteepBoattail | Tail::Step => {
            rocket.add_transition(cone(0.03, 0.040)).unwrap();
        }
        Tail::LongLip => {
            rocket
                .add_transition(cone(0.08, 0.040).named("boattail"))
                .unwrap()
                .add_transition(cone(0.03, 0.046).named("lip"))
                .unwrap();
        }
    }
    rocket
        .set_motor(
            Motor::from_catalog("I377CT")
                .unwrap()
                .with_delay_s(12.0)
                .unwrap(),
        )
        .unwrap()
        .add_parachute(at_apogee());
    if tail != Tail::Step {
        return rocket;
    }
    let mut design = rocket.design().clone();
    for component in &mut design.stages[0].components {
        if let Part::Transition(transition) = &mut component.part {
            transition.fore_radius_m -= 0.001;
        }
    }
    let id = design.configurations[0].id.clone();
    let mut stepped = Rocket::from_design(design, &id).unwrap();
    stepped.add_parachute(at_apogee());
    stepped
}

/// A log of the design's own flight, flown with 15% more drag, fits to that drag: each scale's
/// 90% interval holds what the flight was flown with, each parameter says what it states, and
/// each thing read from the log says where it came from.
#[test]
fn a_fit_finds_the_drag_a_log_was_flown_with() {
    let rocket = built();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    // A log of height alone is told the wind the intervals were sized in, so the log is flown in
    // it: toward the east, which is from 270°.
    let told = Environment::new(32.99, -106.97, 1400.0)
        .unwrap()
        .with_constant_wind(fit::height_alone_wind_m_s(), 270.0)
        .unwrap();
    let log = logged(&Flight::builder(&rocket, &told, 1.8), 1.15, false);
    let fitted = fit::fit(&launch, &log, Some(HeightKind::Geometric)).unwrap();
    assert!(fitted.converged);
    assert!(fitted.rms_m < 1.0, "{}", fitted.rms_m);
    assert_eq!(
        fitted
            .parameters
            .iter()
            .map(|f| f.parameter)
            .collect::<Vec<_>>(),
        Parameter::ALL
    );
    let drag = row(&fitted, Parameter::DragScale);
    assert!((drag.value - 1.15).abs() < 0.02, "{}", drag.value);
    assert!(holds(&fitted, Parameter::DragScale, 1.15));
    // The stated drag, 1, lies outside it: the log moved the fit.
    assert!(!holds(&fitted, Parameter::DragScale, 1.0));
    assert!(holds(&fitted, Parameter::ImpulseScale, 1.0));
    assert!(holds(&fitted, Parameter::MassScale, 1.0));
    // The burn time's scale isn't asked of the fit, and a height can't tell the wind.
    let burn = row(&fitted, Parameter::BurnTimeScale);
    assert_eq!(
        (burn.interval_90, burn.no_interval),
        (None, Some(NoInterval::NotAsked))
    );
    for wind in [Parameter::WindEast, Parameter::WindNorth] {
        let wind = row(&fitted, wind);
        assert_eq!(
            (wind.interval_90, wind.no_interval, wind.caveat),
            (None, Some(NoInterval::HeightAlone), None)
        );
    }
    // Calm air was given. A log of height alone is told the wind its intervals were sized in:
    // the calibration's own, 20 mph over √3, toward the east.
    let told_east = row(&fitted, Parameter::WindEast).stated.mean;
    assert_eq!(told_east, fit::height_alone_wind_m_s());
    assert_eq!(
        told_east,
        hpr_forensics::synthetic::fits::height_alone_wind_m_s()
    );
    assert!((told_east - 20.0 * 0.447_04 / 3.0_f64.sqrt()).abs() < 1e-12);
    assert_eq!(row(&fitted, Parameter::WindNorth).stated.mean, 0.0);
    // Every value lies inside what its interval was tested over, so none is marked.
    for fitted in &fitted.parameters {
        assert!(!fitted.outside_tested_range, "{fitted:?}");
    }
    // Each range is what the synthetic flights drew: the test's truths, and no further.
    let tested = |parameter| {
        let tested = row(&fitted, parameter).tested_range;
        (tested.low, tested.high)
    };
    assert_eq!(tested(Parameter::DragScale), (0.8, 1.2));
    assert_eq!(tested(Parameter::ImpulseScale), (0.94, 1.06));
    assert_eq!(tested(Parameter::BurnTimeScale), (0.90, 1.10));
    assert_eq!(tested(Parameter::MassScale), (1.0 / 1.02, 1.0 / 0.98));
    for wind in [Parameter::WindEast, Parameter::WindNorth] {
        let (low, high) = tested(wind);
        assert!(
            (high - 20.0 * 0.447_04).abs() < 1e-12 && low == -high,
            "{high}"
        );
    }
    // What was read, and where from: the log's own altitude, measured.
    let read = fitted.readings;
    let liftoff = read.liftoff.unwrap();
    assert!((liftoff.time_s - 10.0).abs() < 0.5, "{liftoff:?}");
    assert_eq!(
        (liftoff.source, liftoff.provenance),
        (Source::Barometer, Provenance::Measured)
    );
    assert_eq!(
        (read.apogee.source, read.apogee.provenance),
        (Source::Barometer, Provenance::Measured)
    );
    assert_eq!(
        (read.height_track.source, read.height_track.provenance),
        (Source::Barometer, Provenance::Measured)
    );
    // A height climbed is read past Mach 0.8 too: every bin.
    assert_eq!(read.height_track.bins, read.height_track.bins_read);
    assert!(read.height_track.bins > 40);
    assert_eq!(read.horizontal_track, None);
    assert_eq!(fitted.height_read_as, HeightKind::Geometric);
    // The fitted flight is the log's: more drag, so lower than the flight as stated.
    let fitted_apogee_m = fitted.fitted_apogee_m.unwrap();
    assert!(fitted_apogee_m < fitted.stated_apogee_m - 10.0);
    assert!((fitted_apogee_m - read.apogee.height_m).abs() < 5.0);
    // The text says each.
    let text = fitted.lines().join("\n");
    for said in [
        "liftoff ",
        "(measured, from the logger's barometric altitude)",
        "bins of 0.25 s from 1 s before liftoff, read as a height climbed",
        "drift          none: a log of height alone",
        "  told before the log\n  drag scale  ",
        " *  1.000 ± 0.109\n  impulse scale  ",
        "  none  ",
        "  * This interval fails its test: the caveat below.\n  The burn time scale states no \
         interval: not asked of the fit.\n  The wind states no interval: a log of height alone \
         can't tell the wind.\n  So the fit is told a wind of at least 5.16 m/s, the wind its \
         intervals were sized in.\n",
        "Caveat. The drag scale's 90% interval fails its test.",
        "Warning. issue #172",
    ] {
        assert!(text.contains(said), "{said:?} in\n{text}");
    }
    assert!(!text.contains('†'), "{text}");
    // A printed interval never claims less than the number holds: each end is rounded outward,
    // by less than the last digit printed.
    for parameter in [
        Parameter::DragScale,
        Parameter::ImpulseScale,
        Parameter::MassScale,
    ] {
        let starts = format!("  {}  ", parameter.name());
        let line = text.lines().find(|line| line.starts_with(&starts)).unwrap();
        let cells: Vec<&str> = line.split_whitespace().collect();
        let to = cells.iter().position(|cell| *cell == "to").unwrap();
        let (low, high): (f64, f64) = (
            cells[to - 1].parse().unwrap(),
            cells[to + 1].parse().unwrap(),
        );
        let held = row(&fitted, parameter).interval_90.unwrap();
        assert!(low <= held.low && held.low - low < 0.001, "{line}");
        assert!(high >= held.high && high - held.high < 0.001, "{line}");
    }
    // A log of the same flight in still air fits too: its drag's interval still holds 1.15.
    let still = fit::fit(
        &launch,
        &logged(&launch, 1.15, false),
        Some(HeightKind::Geometric),
    )
    .unwrap();
    assert!(holds(&still, Parameter::DragScale, 1.15));
    // It reads back as it was written.
    let json = serde_json::to_string(&fitted).unwrap();
    assert_eq!(serde_json::from_str::<LogFit>(&json).unwrap(), fitted);
    // Read as a barometer's, the same log fits too, with a barometer's unknowns open.
    let by_barometer = fit::fit(&launch, &log, None).unwrap();
    assert_eq!(by_barometer.height_read_as, HeightKind::Barometric);
    assert!((row(&by_barometer, Parameter::DragScale).value - 1.15).abs() < 0.1);
}

/// A log with a horizontal track speaks to the wind: flown in a wind from the west and fitted
/// from a launch in calm air, each wind component states an interval, and the east one's holds
/// the wind flown.
#[test]
fn a_log_with_a_track_states_the_wind() {
    let rocket = built();
    let calm = environment();
    let windy = Environment::new(32.99, -106.97, 1400.0)
        .unwrap()
        .with_constant_wind(5.0, 270.0)
        .unwrap();
    let log = logged(&Flight::builder(&rocket, &windy, 1.8), 1.0, true);
    let launch = Flight::builder(&rocket, &calm, 1.8);
    let fitted = fit::fit(&launch, &log, Some(HeightKind::Geometric)).unwrap();
    let track = fitted.readings.horizontal_track.unwrap();
    assert_eq!(
        (track.source, track.provenance),
        (Source::Gnss, Provenance::Measured)
    );
    assert!(track.bins > 40);
    let east = row(&fitted, Parameter::WindEast);
    assert_eq!(east.no_interval, None);
    assert!((east.value - 5.0).abs() < 1.0, "{}", east.value);
    assert!(holds(&fitted, Parameter::WindEast, 5.0));
    assert!(!holds(&fitted, Parameter::WindEast, 0.0));
    assert!(holds(&fitted, Parameter::WindNorth, 0.0));
    assert!(holds(&fitted, Parameter::DragScale, 1.0));
    let text = fitted.lines().join("\n");
    assert!(
        text.contains("east and north of the pad  (measured, from the GNSS fixes)"),
        "{text}"
    );
    // On a log with a track, the wind given is the wind told: calm here.
    assert_eq!(east.stated.mean, 0.0);
    assert_eq!(row(&fitted, Parameter::WindNorth).stated.mean, 0.0);
    // Its fitted wind is slower than the fastest its interval was tested in, so it isn't marked.
    let tested = east.tested_range;
    assert!((tested.high - 20.0 * 0.447_04).abs() < 1e-12, "{tested:?}");
    assert_eq!(tested.low, -tested.high);
    assert!(!east.outside_tested_range);
    assert!(!text.contains('†'), "{text}");
    // Within the scan's reach nothing better was left past it, so no second answer is said.
    assert_eq!(fitted.wind_past_scan, None);
    assert!(!text.contains("second answer"), "{text}");
    // A fit that left one prints it as a caveat of its own, before the warnings.
    let past = WindPastScan {
        wind_east_m_s: 12.12,
        wind_north_m_s: 0.0,
        chi2: 15.24,
        standing_chi2: 127.8,
    };
    let said = LogFit {
        wind_past_scan: Some(past),
        ..fitted
    };
    let lines = said.lines();
    assert!(
        lines.contains(&format!("Caveat. {}", past.message())),
        "{lines:?}"
    );
}

/// The second answer a fit leaves past its wind scan's reach ([ADR-382] §7) says the wind it
/// ended at as a speed and the way it blows from, how the two answers fit, and what to do. Its
/// speed is rounded up; the two misfits are rounded toward each other, to enough places that
/// the lower still prints lower, and print as found where six places can't tell them apart.
///
/// [ADR-382]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0382-the-fits-wind-scan-on-a-log-with-a-track.md
#[test]
fn a_second_answer_past_the_scan_says_its_wind_and_how_the_two_fit() {
    // As the 54 mm design's flight in 12.5 m/s toward the east left it, fitted from calm air.
    let past = WindPastScan {
        wind_east_m_s: 12.123,
        wind_north_m_s: 0.003,
        chi2: 15.236,
        standing_chi2: 127.804,
    };
    assert!((past.speed_m_s() - 12.123).abs() < 1e-3);
    // Toward the east is from the west, 270° clockwise from north.
    assert!(
        (past.from_deg() - 269.986).abs() < 1e-3,
        "{}",
        past.from_deg()
    );
    assert_eq!(past.told(), (12.2, 270.0));
    assert_eq!(
        past.message(),
        "The wind has a second answer, which the fit didn't take. A search ended at a wind of \
         12.2 m/s (27.3 mph) from 270°, past the 12 m/s (26.8 mph) the fit's scan of the wind \
         reaches. It fits the log better than the fitted wind: a misfit (χ²) of 15.3 against \
         127.8. Past the scan's reach the fit can't tell the flight's wind from a wrong one, so \
         the fitted wind stands. If the day's wind was near the second answer, fit again with \
         that wind given. If you have no record of the day's wind, take the fitted wind, and \
         the scales fitted with it, as unsettled."
    );
    let told = |past: &WindPastScan| {
        let (chi2, standing_chi2) = past.misfits_told();
        format!("{chi2} against {standing_chi2}")
    };
    assert_eq!(told(&past), "15.3 against 127.8");
    // Two misfits that one decimal place would print crossed, or as equals, get the places
    // that keep the lower one lower.
    let fits = |chi2: f64, standing_chi2: f64| WindPastScan {
        chi2,
        standing_chi2,
        ..past
    };
    assert_eq!(told(&fits(15.02, 15.04)), "15.02 against 15.04");
    assert!(
        fits(15.02, 15.04)
            .message()
            .contains("a misfit (χ²) of 15.02 against 15.04."),
    );
    assert_eq!(told(&fits(15.0, 15.1)), "15.0 against 15.1");
    assert_eq!(told(&fits(15.01, 15.19)), "15.01 against 15.19");
    assert_eq!(told(&fits(99.7, 128.0)), "99.7 against 128.0");
    // Two that six places can't tell apart print as found: rounded toward each other there,
    // these would print as 15.000001 against 15.000000, the higher one first.
    let close = fits(15.000_000_2, 15.000_000_4);
    assert_eq!(told(&close), "15.0000002 against 15.0000004");
    assert!(
        close
            .message()
            .contains("a misfit (χ²) of 15.0000002 against 15.0000004."),
    );
    assert_eq!(told(&fits(15.0, 15.0 + 1e-9)), "15 against 15.000000001");
    // A standing answer with no finite misfit prints as that.
    assert_eq!(told(&fits(15.236, f64::INFINITY)), "15.3 against inf");
    // Over five sizes of misfit and gaps from 10 times the value down to a part in 10¹¹ of
    // it: the pair prints in its order, and the gap printed is no wider than the gap found,
    // but for the part in 10⁹ that the rounding lets go.
    for base in [0.004, 0.5, 15.0, 127.804, 99_999.5] {
        for power in 0..=12 {
            let (chi2, standing_chi2) = (base, base * (1.0 + 10_f64.powi(1 - power)));
            let case = fits(chi2, standing_chi2);
            let (first, second) = case.misfits_told();
            let (first, second): (f64, f64) = (first.parse().unwrap(), second.parse().unwrap());
            assert!(
                first < second,
                "{chi2} against {standing_chi2}: {first}, {second}"
            );
            assert!(
                first >= chi2 * (1.0 - 2e-9) && second <= standing_chi2 * (1.0 + 2e-9),
                "{chi2} against {standing_chi2}: {first}, {second}"
            );
        }
    }
    // The way a wind blows from, each quarter: toward the north is from the south.
    let toward = |east: f64, north: f64| WindPastScan {
        wind_east_m_s: east,
        wind_north_m_s: north,
        ..past
    };
    for (east, north, from) in [
        (0.0, 13.0, 180.0),
        (-13.0, 0.0, 90.0),
        (0.0, -13.0, 0.0),
        (9.2, 9.2, 225.0),
    ] {
        let wind = toward(east, north);
        assert!((wind.from_deg() - from).abs() < 1e-9, "{east}, {north}");
        assert_eq!(wind.told().1, from, "{east}, {north}");
    }
    // A way just short of north is told as 0°, not 360°, and the speed never rounds down.
    let nearly_north = toward(0.05, -13.01);
    assert!(nearly_north.from_deg() > 359.7);
    assert_eq!(nearly_north.told(), (13.1, 0.0));
    assert_eq!(toward(12.0, 0.0).told().0, 12.0);
    // The way stays under 360° where the turn rounds up to it, and a wind straight from the
    // north is 0°, not −0°.
    let hair_short = toward(1e-20, -13.0).from_deg();
    assert!((0.0..360.0).contains(&hair_short), "{hair_short}");
    assert_eq!(hair_short, 0.0);
    let from_north = toward(0.0, -13.0).from_deg();
    assert!(
        from_north == 0.0 && from_north.is_sign_positive(),
        "{from_north}"
    );
    let from_north = toward(-0.0, -13.0).from_deg();
    assert!(
        from_north == 0.0 && from_north.is_sign_positive(),
        "{from_north}"
    );
}

/// What a log of height alone is told of the wind: at least the wind its intervals' shares were
/// fitted in, along the wind the launch gives. No test covers a fit of such a log told less.
#[test]
fn a_log_of_height_alone_is_told_at_least_the_wind_its_intervals_were_sized_in() {
    let rocket = built();
    let least = fit::height_alone_wind_m_s();
    let site = |speed_m_s: f64, from_deg: f64| {
        Environment::new(32.99, -106.97, 1400.0)
            .unwrap()
            .with_constant_wind(speed_m_s, from_deg)
            .unwrap()
    };
    let calm = environment();
    let log = logged(&Flight::builder(&rocket, &calm, 1.8), 1.0, false);
    let fitted = |environment: &Environment| {
        fit::fit(
            &Flight::builder(&rocket, environment, 1.8),
            &log,
            Some(HeightKind::Geometric),
        )
        .unwrap()
    };
    let told = |fitted: &LogFit| {
        (
            row(fitted, Parameter::WindEast).stated.mean,
            row(fitted, Parameter::WindNorth).stated.mean,
        )
    };
    // Calm air: the least wind, toward the east, as the synthetic flights' fits were told.
    let in_calm = fitted(&calm);
    assert_eq!(told(&in_calm), (least, 0.0));
    // A slower wind than that, from the south: raised to it, along its own way, toward the north.
    let (east, north) = told(&fitted(&site(2.0, 180.0)));
    assert!(
        east.abs() < 1e-9 && (north - least).abs() < 1e-9,
        "{east} {north}"
    );
    // A faster one, from the west, stands as given.
    let faster = fitted(&site(7.0, 270.0));
    let (east, north) = told(&faster);
    assert!(
        (east - 7.0).abs() < 1e-9 && north.abs() < 1e-9,
        "{east} {north}"
    );
    // The wind told widens the drag's interval: the more wind, the wider.
    let width = |fitted: &LogFit| {
        let interval = row(fitted, Parameter::DragScale).interval_90.unwrap();
        interval.high - interval.low
    };
    assert!(
        width(&faster) > width(&in_calm),
        "{} {}",
        width(&faster),
        width(&in_calm)
    );
}

/// ADR-163 §4's list, each issue on a fit whose flight meets its condition (M7.3f): #67, #68,
/// #70, #72 and #222 on an ogive nose, airfoil fins and a steep boattail past Mach 1; #121 on a
/// conical tip past the cone tables' 30°; #120 on a lip longer than its boattail's drop; #87 on a
/// step in radius; #172 on every one. A subsonic flight's fit carries none whose condition is a
/// speed it never reached.
#[test]
fn a_fit_carries_each_warning_its_flight_meets() {
    let environment = environment();
    let ogive = NoseShape::Ogive { radius_ratio: 1.0 };
    let rounded = FinCrossSection::Rounded;
    let fitted = |rocket: &Rocket| {
        let launch = Flight::builder(rocket, &environment, 1.8);
        let log = logged(&launch, 1.0, false);
        fit::fit(&launch, &log, Some(HeightKind::Geometric)).unwrap()
    };
    let sharp = fitted(&shaped(
        ogive,
        0.25,
        FinCrossSection::Airfoil,
        Tail::SteepBoattail,
    ));
    assert!(sharp.fitted_max_mach.unwrap() > 1.2);
    let steep_tip = fitted(&shaped(NoseShape::Conical {}, 0.045, rounded, Tail::Plain));
    let lip = fitted(&shaped(ogive, 0.25, rounded, Tail::LongLip));
    let step = fitted(&shaped(ogive, 0.25, rounded, Tail::Step));
    // In `hpr_sim::issues`' order: the drag's, then friction's #18, then the stability's and the
    // flight path's, which aren't on the list but are the same flight's.
    assert_eq!(
        numbers(&sharp),
        [67, 68, 70, 72, 222, 73, 18, 172, 1290, 1198, 106]
    );
    assert_eq!(numbers(&steep_tip), [67, 68, 18, 121, 172, 1290, 1198]);
    assert_eq!(numbers(&lip), [67, 68, 222, 18, 120, 172, 1290, 1198]);
    assert_eq!(
        numbers(&step),
        [67, 68, 72, 222, 73, 18, 87, 172, 1290, 1198]
    );
    // Each of the list's nine is carried by one of them, with its own message, in the text.
    let all = [&sharp, &steep_tip, &lip, &step];
    for number in [87, 120, 121, 222, 172, 67, 70, 72, 68] {
        let carrier = all
            .iter()
            .find(|fitted| numbers(fitted).contains(&number))
            .unwrap_or_else(|| panic!("no fit carries #{number}"));
        let warning = carrier
            .issue_warnings
            .iter()
            .find(|warning| warning.number() == number)
            .unwrap();
        let said = format!("Warning. {}", warning.message());
        assert!(
            said.starts_with(&format!("Warning. issue #{number}: ")),
            "{said}"
        );
        assert!(carrier.lines().contains(&said), "{said}");
    }
    // The same launch flown plainly raises the same.
    let rocket = shaped(ogive, 0.25, FinCrossSection::Airfoil, Tail::SteepBoattail);
    let flown = Flight::builder(&rocket, &environment, 1.8).fly().unwrap();
    assert_eq!(numbers(&sharp), super::numbers(&flown));
    // The warnings are the fitted flight's, not the stated one's. A log flown with half the drag
    // fits to a faster flight, which passes Mach 0.6 and meets #1290; as stated it stays under.
    let slow = built();
    let launch = Flight::builder(&slow, &environment, 1.8);
    let stated = launch.fly().unwrap();
    assert!(stated.max_mach().unwrap() < 0.58);
    assert_eq!(super::numbers(&stated), [18, 172]);
    let faster = fit::fit(&launch, &logged(&launch, 0.5, false), None).unwrap();
    assert!(faster.fitted_max_mach.unwrap() > 0.61);
    assert_eq!(numbers(&faster), [18, 172, 1290]);
    // That drag scale lies outside what its interval was tested over, and is marked so.
    let drag = row(&faster, Parameter::DragScale);
    assert!(drag.value < 0.8 && drag.outside_tested_range, "{drag:?}");
    assert!(!row(&faster, Parameter::ImpulseScale).outside_tested_range);
    assert!(!row(&faster, Parameter::MassScale).outside_tested_range);
    let text = faster.lines().join("\n");
    assert!(
        text.contains(
            "\n  † The drag scale lies outside 0.800 to 1.200, the range the intervals were tested \
             over.\n"
        ),
        "{text}"
    );
    assert_eq!(text.matches('†').count(), 2, "{text}");
    // A subsonic flight meets no condition that is a speed past Mach 0.8.
    let slow = built();
    let launch = Flight::builder(&slow, &environment, 1.8);
    let slow = fit::fit(&launch, &logged(&launch, 1.0, false), None).unwrap();
    assert!(slow.fitted_max_mach.unwrap() < 0.8);
    let found = numbers(&slow);
    assert!(found.contains(&172), "{found:?}");
    for number in [67, 68, 70, 72, 222, 87, 120, 121] {
        assert!(!found.contains(&number), "#{number} in {found:?}");
    }
}

/// ADR-368's caveat: the interval that fails ADR-257 §1's test carries it, and no other does.
/// Its numbers are the committed report's, and it says which way the interval errs: the truths
/// lie below it, so the fitted drag reads high and a flight predicted from it reads low.
#[test]
fn the_interval_that_fails_its_test_carries_the_caveat() {
    // The list `cargo xtask fit` holds the report to: one cell, set B's drag scale.
    assert_eq!(KNOWN_GAPS, [(Set::B, Asked::DragScale, 79)]);
    let rocket = built();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    let fitted = fit::fit(&launch, &logged(&launch, 1.0, false), None).unwrap();
    let caveat = row(&fitted, Parameter::DragScale).caveat.unwrap();
    assert_eq!(
        caveat,
        Caveat {
            below: 79,
            flights: 1_000,
            meant: 50
        }
    );
    // Every parameter carries a caveat exactly when its interval is on the list.
    for fitted in &fitted.parameters {
        let on_the_list = match fitted.parameter {
            Parameter::DragScale => known_gap_of(Asked::DragScale),
            Parameter::ImpulseScale => known_gap_of(Asked::ImpulseScale),
            Parameter::MassScale => known_gap_of(Asked::MassScale),
            Parameter::WindEast => known_gap_of(Asked::WindEast),
            Parameter::WindNorth => known_gap_of(Asked::WindNorth),
            Parameter::BurnTimeScale => None,
        };
        assert_eq!(
            fitted.caveat.is_some(),
            on_the_list.is_some() && fitted.interval_90.is_some(),
            "{:?}",
            fitted.parameter
        );
    }
    assert_eq!(
        ASKED
            .iter()
            .filter(|asked| known_gap_of(**asked).is_some())
            .count(),
        1
    );
    // The committed report's counts for that cell: the caveat's numbers, and its direction.
    let report: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../validation/reports/fit-intervals.json"
    ))
    .unwrap();
    let set_b = report["sets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|set| set["set"] == "b")
        .unwrap();
    let counts = &set_b["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["asked"] == "drag-scale")
        .unwrap()["test"]["counts"];
    assert_eq!(counts["given"], caveat.flights);
    assert_eq!(counts["below"], caveat.below);
    // Below, not above: the side the message names. 5% of the flights is what is meant.
    assert!(counts["below"].as_u64().unwrap() > counts["above"].as_u64().unwrap());
    assert_eq!(caveat.meant * 20, caveat.flights);
    assert!(caveat.below > caveat.meant);
    let message = caveat.message(Parameter::DragScale);
    assert_eq!(
        message,
        format!(
            "The drag scale's 90% interval fails its test. On 1,000 synthetic flights flown \
             with a model unlike the fit's own, 79 truths lay below the interval, where a 90% \
             interval leaves 50. So its lower end can read too high: take the fitted drag scale \
             as reading high, and a flight predicted with it as reading low. Recorded, not \
             fixed: {INTERVAL_ISSUE_URL}"
        )
    );
    assert!(fitted.lines().contains(&format!("Caveat. {message}")));
    // The row itself points at it.
    let text = fitted.lines().join("\n");
    let marked: Vec<&str> = text.lines().filter(|line| line.contains(" *  ")).collect();
    assert_eq!(marked.len(), 1, "{text}");
    assert!(marked[0].starts_with("  drag scale  "), "{text}");
    assert!(text.contains("\n  * This interval fails its test: the caveat below.\n"));
}

/// What can't be fitted is refused, each by its own reason: a log with no flight in it, a flight
/// that comes apart, and a log no flight of the design passes through.
#[test]
fn a_log_that_cant_be_fitted_is_refused() {
    let rocket = built();
    let environment = environment();
    let launch = Flight::builder(&rocket, &environment, 1.8);
    let log = logged(&launch, 1.0, false);
    // A log that never leaves the pad.
    let flat = FlightLog {
        altitude_m: vec![0.0; log.time_s.len()],
        ..log.clone()
    };
    let refused = fit::fit(&launch, &flat, None).unwrap_err();
    assert!(
        matches!(&refused, Refused::Log(why) if why.starts_with("its apogee is withheld: ")),
        "{refused}"
    );
    // A flight that separates.
    let staged = launch
        .clone()
        .separation(hpr_sim::Separation::new(Trigger::Apogee, 0));
    let refused = fit::fit(&staged, &log, None).unwrap_err();
    assert!(matches!(refused, Refused::Staged(1)), "{refused}");
    // The rocket's own heights on a clock at half speed: no drag, impulse or mass makes the same
    // climb take twice as long.
    let other = FlightLog {
        time_s: log.time_s.iter().map(|t| 2.0 * t).collect(),
        ..log.clone()
    };
    let refused = fit::fit(&launch, &other, Some(HeightKind::Geometric)).unwrap_err();
    assert!(
        matches!(refused, Refused::SaysNothing { misfit_share } if misfit_share > 0.1),
        "{refused}"
    );
    assert!(
        refused
            .to_string()
            .starts_with("no flight of this design passes through the log: "),
        "{refused}"
    );
    // A launch that can't be flown says why, as flying it would.
    let leaning = launch.clone().inclination_deg(120.0);
    let refused = fit::fit(&leaning, &log, None).unwrap_err();
    assert!(
        matches!(&refused, Refused::Flight(crate::Error::Domain { value, .. }) if *value == 120.0),
        "{refused}"
    );
}

/// `hpr fit`'s forward model ([`fit::Flown`]) flies what the synthetic fits' does
/// ([`hpr_forensics::synthetic::fits::Flown`]), the model whose fits sized the 90% intervals the
/// command states: one public design, the first draw of set A on it, flown under one set of
/// parameters, none of them 1 or 0, in a constant wind and in a sheared one. Each flight is held
/// equal to the other's bit for bit: every moment's height climbed, barometer's reading, their
/// rates, the drift east and north and the Mach number ([`Simulated`]'s equality), and its heights
/// as each sensor reads them. [issue #1647](https://github.com/nrdptel/fusionspace-eridanus/issues/1647).
///
/// The launch is stated to the command as a person would: the design file, its configuration, the
/// draw's site, air and rail, through [`FlightBuilder`]. Where the two models are told different
/// things, it is meant, and the test tells them alike:
///
/// - The synthetic fit's structure is `weighed` times the true one (its kitchen scale's error,
///   [`WEIGHED_WITHIN`](hpr_forensics::synthetic::fits::WEIGHED_WITHIN)); the command's is the one
///   the person states, which is what was weighed. So the synthetic one is flown at `weighed` 1.
/// - Each flies a sheared wind from the pad's height above mean sea level: the command's from the
///   site's height less the geoid's undulation, the synthetic one's from the draw's elevation. A
///   draw's site has no undulation, so the two are the same height.
#[test]
fn hpr_fit_flies_the_flight_its_intervals_were_sized_with() {
    use hpr_atmos::{AtmosphereModel, Ussa76};
    use hpr_forensics::fit::{Forward, Parameters};
    use hpr_forensics::residuals::Simulated;
    use hpr_forensics::synthetic::designs::{DESIGNS, DesignFiles};
    use hpr_forensics::synthetic::truth::RAIL_LENGTH_M;
    use hpr_forensics::synthetic::{self, Model};
    use std::collections::BTreeMap;

    const DESIGN: &str = "rocketpy-calisto-getting-started-motor-at-minus-1.255";
    let json = include_str!(
        "../../../../validation/designs/rocketpy-calisto-getting-started-motor-at-minus-1.255.json"
    );
    let design = DESIGNS.iter().position(|d| d.name == DESIGN).unwrap();
    let draw = (0..)
        .map(|index| synthetic::draw(Set::A, index))
        .find(|draw| draw.design == design)
        .unwrap();
    assert_eq!(draw.model, Model::Same);
    // Only the drawn design's file is read; the others are left empty.
    let mut files = vec![String::new(); DESIGNS.len()];
    files[design] = json.to_owned();
    let files = DesignFiles::new(files).unwrap();

    // The launch as the command is told it.
    let rocket = Rocket::from_design(
        serde_json::from_str(json).unwrap(),
        DESIGNS[design].configuration,
    )
    .unwrap();
    let environment = Environment::new(
        synthetic::truth::SITE_LATITUDE_DEG,
        synthetic::truth::SITE_LONGITUDE_DEG,
        draw.elevation_m,
    )
    .unwrap()
    .with_atmosphere(AtmosphereModel::Standard(
        // The draws' air: the 1976 standard, offset, over sea-level pressure.
        Ussa76::with_offset(draw.temperature_offset_k, 101_325.0).unwrap(),
    ));
    let launch = Flight::builder(&rocket, &environment, RAIL_LENGTH_M).rail(hpr_sim::Rail {
        elevation_rad: std::f64::consts::FRAC_PI_2 - draw.tilt_rad,
        azimuth_rad: draw.azimuth_rad,
        ..hpr_sim::Rail::vertical(RAIL_LENGTH_M)
    });
    let mut inputs = launch.inputs().unwrap();
    inputs.recovery.clear();
    let mut command = fit::Flown::new(inputs);

    let mut tables = BTreeMap::new();
    let mut sized = synthetic::fits::Flown::new(&draw, &files, 1.0, &mut tables);

    let p = Parameters {
        drag_scale: 1.07,
        impulse_scale: 0.97,
        burn_time_scale: 1.04,
        wind_east_m_s: 3.1,
        wind_north_m_s: -1.7,
        mass_scale: 1.012,
    };
    let until_s = 60.0;
    let held = |of: &str, command: &Simulated, sized: &Simulated| {
        for kind in [HeightKind::Barometric, HeightKind::Geometric] {
            let (command_heights, _) = command.read_lines(kind, 0.0);
            let (sized_heights, _) = sized.read_lines(kind, 0.0);
            assert!(
                command_heights.len() > 100,
                "{of}: a climb of a few moments"
            );
            assert_eq!(command_heights, sized_heights, "{of}: {kind:?} heights");
            assert!(command.top_reading_m(kind).is_some(), "{of}: no apogee");
            assert_eq!(
                command.top_reading_m(kind),
                sized.top_reading_m(kind),
                "{of}: {kind:?} apogee"
            );
        }
        assert!(
            command == sized,
            "{of}: a moment's drift, rate or Mach number"
        );
    };
    held(
        "constant wind",
        &command.fly(&p, until_s).unwrap(),
        &sized.fly(&p, until_s).unwrap(),
    );
    held(
        "sheared wind",
        &command.fly_sheared(&p, until_s, 1.0 / 7.0).unwrap(),
        &sized.fly_sheared(&p, until_s, 1.0 / 7.0).unwrap(),
    );
}
