//! The Featherweight app's summaries of a Blue Raven's and a GPS tracker's flight, read through
//! the importer from invented files and pinned whole by snapshots (M7.1c6). The Blue Raven's has
//! every row the app writes, in its order, padded to four cells as some files are; the tracker's
//! is its 15 rows. Every number is made up.

use hpr_flightdata::import;
use hpr_flightdata::log::{Figure, FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// An invented Blue Raven summary: the app's 48 rows, every row padded to four cells.
const BLUE_RAVEN: &str = "\
Rocket Name,Demo rocket,,
Firmware,a1b2c3 01/02/2026 03:04:05,,
Serial number,SN100 Demo,,
Max Altitude,2500.25 feet,,
Max velocity,500.50 feet/sec,,
Pad altitude ASL,800.75 feet,,
Launch date,01-Jun-26,,
Launch time,10:00:00.000,,
Time to first burnout,1.5 sec,,
Time to Apo channel fire,14.0 sec,,
Time to gyro overload,0.0 sec,,
Time to third channel fire,15.0 sec,,
Time to main channel fire,60.0 sec,,
Time to fourth channel fire,0.0 sec,,
Launch rod angle,3.0 deg,,
Tilt angle at burnout,4.0 deg,,
Roll rate at burnout,-30.0 deg/sec,,
Thrust tilt efficiency,98.50%,,
Number of ascent rolls,-0.250,,
Horiz. velocity at apogee,40.0 feet/sec,,
Inertial navigation max alt,2600.0 feet,,
Max motor burn acceleration,12.0 Gs,,
Peak drag deceleration,3.0 Gs,,
Apo channel max accel,20.0 Gs,,
Main channel max accel,25.0 Gs,,
3rd channel max accel,10.0 Gs,,
4th channel max accel,0.0 Gs,,
Max landing accel,5.0 Gs,,
Drogue descent rate,-60.0 feet/sec,,
Main chute deploy altitude,700.0 feet,,
Main chute descent rate,-20.0 feet,,
Landing Time,10:02:00.000,,
Temperature on pad,77.0 F,,
Max Temperature,80.6 F,,
Min Temperature,68.0 F,,
Battery voltage on pad,4.10 V,,
Min Battery voltage,4.00 V,,
Battery voltage at landing,4.05 V,,
Build date,02-Jan-26,,
Build time,03:04:05.000,,
Flight number,7,,
Direction of 1st motion,-1.0000,0.0000,0.0000
Gravity direction on pad,-0.6000,0.8000,0.0000
Low-G accel pad offset,0.0010,0.0000,-0.0020
Hi-G accel pad offset,0.50,0.25,1.00
Pad gyro rates (deg/sec),0.10,-0.20,0.30
Pad Hi-G accel,0.05,0.90,1.50
Pad Low-G Accel,-1.00,0.01,0.00
";

/// An invented GPS tracker's summary, its 15 rows.
const TRACKER: &str = "\r
Rocket Name,Demo tracker\r
Launch time UTC,2026-06-01 15:00:00 UTC\r
Launch time local time zone,2026-06-01 10:00:00\r
Max Altitude,2000.00 feet\r
Pad altitude,1500.0 ft\r
Pad Latitude,35.12345 deg\r
Pad Longtitude,-100.54321 deg\r
Distance at launch,10 ft\r
Distance at apogee,900 ft\r
Distance at landing,2500 ft\r
Max vertical velocity,450 ft/sec\r
Max horizontal velocity,120 ft/sec\r
Horizontal velocity at apogee,60 ft/sec\r
Horizontal velocity at laniding,5 ft/sec\r
Vertical velocity at landing,-20 ft/sec\r
";

/// The figure of a name, if the log states it.
fn figure<'a>(log: &'a FlightLog, name: &str) -> Option<&'a Figure> {
    log.stated.figures.iter().find(|figure| figure.name == name)
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * b.abs().max(1.0)
}

/// The Blue Raven's summary: every row a figure, in SI where the unit is one its name allows, the
/// main's descent rate and the pad accelerations kept as written, and no samples.
#[test]
fn a_blue_raven_summary_keeps_every_figure() {
    assert_eq!(
        import::detect(BLUE_RAVEN.as_bytes()),
        Some(LogFormat::FeatherweightSummaryCsv)
    );
    let log = import::read(BLUE_RAVEN.as_bytes()).unwrap();
    assert_eq!(log.format, LogFormat::FeatherweightSummaryCsv);
    assert_eq!(log.logger, "Featherweight Blue Raven");
    assert_eq!(log.serial_number.as_deref(), Some("SN100 Demo"));
    assert_eq!(log.firmware.as_deref(), Some("a1b2c3 01/02/2026 03:04:05"));
    assert_eq!(log.flight_number, Some(7));
    assert_eq!(log.stated.figures.len(), 48);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty() && log.channels.is_empty());
    assert!(close(log.stated.apogee_m.unwrap(), 2500.25 * FOOT_M));
    assert!(close(
        log.stated.ground_elevation_msl_m.unwrap(),
        800.75 * FOOT_M
    ));

    let speed = figure(&log, "Max velocity").unwrap();
    assert_eq!(speed.quantity, Some(Quantity::Speed));
    assert!(close(speed.values[0], 500.5 * FOOT_M));
    assert_eq!(speed.written, "500.50 feet/sec");
    let temperature = figure(&log, "Temperature on pad").unwrap();
    assert!(close(temperature.values[0], 298.15), "{temperature:?}");
    assert!(close(
        figure(&log, "Max motor burn acceleration").unwrap().values[0],
        12.0 * 9.806_65
    ));
    assert_eq!(
        figure(&log, "Thrust tilt efficiency").unwrap().values,
        [0.985]
    );
    assert_eq!(
        figure(&log, "Time to Apo channel fire").unwrap().quantity,
        Some(Quantity::Duration)
    );
    let gyro = figure(&log, "Pad gyro rates (deg/sec)").unwrap();
    assert_eq!(gyro.quantity, Some(Quantity::AngularRate));
    assert_eq!(gyro.values, [0.1, -0.2, 0.3]);
    let gravity = figure(&log, "Gravity direction on pad").unwrap();
    assert_eq!(gravity.quantity, Some(Quantity::Number));
    assert_eq!(gravity.written, "-0.6000,0.8000,0.0000");
    assert_eq!(figure(&log, "Launch date").unwrap().quantity, None);

    // Kept as written: a rate in feet, and accelerations with no unit.
    let main = figure(&log, "Main chute descent rate").unwrap();
    assert_eq!(main.quantity, Some(Quantity::Unrecognized));
    assert_eq!(main.values, [-20.0]);
    let pad = figure(&log, "Pad Low-G Accel").unwrap();
    assert_eq!(pad.quantity, Some(Quantity::Unrecognized));
    assert_eq!(pad.values, [-1.0, 0.01, 0.0]);
    assert_eq!(log.notes.len(), 6, "{:#?}", log.notes);
    assert!(log.notes[0].starts_with("a summary holds no samples"));
    assert_eq!(
        log.notes[1],
        "`Main chute descent rate` kept as written, in a unit that isn't a speed: the file \
         writes `-20.0 feet`"
    );
    insta::assert_json_snapshot!(log);
}

/// The tracker's summary: its altitude above the pad it states, the pad's GNSS altitude, its
/// position and its speeds, CR LF line ends and a blank first line.
#[test]
fn a_trackers_summary_reads_its_pad_and_its_speeds() {
    let log = import::read(TRACKER.as_bytes()).unwrap();
    assert_eq!(log.format, LogFormat::FeatherweightSummaryCsv);
    assert_eq!(log.logger, "Featherweight GPS tracker");
    assert_eq!(
        (log.serial_number.as_ref(), log.flight_number),
        (None, None)
    );
    assert!(close(log.stated.apogee_m.unwrap(), 2000.0 * FOOT_M));
    assert!(close(
        log.stated.ground_elevation_msl_m.unwrap(),
        1500.0 * FOOT_M
    ));
    assert_eq!(
        figure(&log, "Pad altitude").unwrap().quantity,
        Some(Quantity::GnssAltitude)
    );
    assert_eq!(figure(&log, "Pad Latitude").unwrap().values, [35.12345]);
    assert_eq!(
        figure(&log, "Pad Longtitude").unwrap().quantity,
        Some(Quantity::Longitude)
    );
    assert!(close(
        figure(&log, "Vertical velocity at landing").unwrap().values[0],
        -20.0 * FOOT_M
    ));
    assert_eq!(log.notes.len(), 1, "{:#?}", log.notes);
    assert!(log.notes[0].ends_with("the tracker's CSV exported beside it"));
    insta::assert_json_snapshot!(log);
}

/// A figure whose cells aren't the numbers its name needs is kept as written, never guessed: a
/// direction with two components, a velocity with no unit, a latitude past the pole, an unknown
/// name, and a flight number that isn't whole.
#[test]
fn a_figure_that_doesnt_fit_its_name_is_kept_as_written() {
    let text = "Rocket Name,Demo\nMax Altitude,100 feet\nDirection of 1st motion,1,0\n\
                Max velocity,100\nPad Latitude,95 deg\nSpin rate,3 rps\nFlight number,2.5\n";
    let log = import::read(text.as_bytes()).unwrap();
    let kept = |name| figure(&log, name).unwrap().quantity;
    assert_eq!(
        kept("Direction of 1st motion"),
        Some(Quantity::Unrecognized)
    );
    assert_eq!(kept("Max velocity"), Some(Quantity::Unrecognized));
    assert_eq!(kept("Pad Latitude"), Some(Quantity::Unrecognized));
    assert_eq!(kept("Spin rate"), Some(Quantity::Unrecognized));
    assert_eq!(figure(&log, "Spin rate").unwrap().values, [3.0]);
    assert_eq!(log.flight_number, None);
    let decisions: Vec<&str> = log
        .notes
        .iter()
        .skip(1)
        .map(|note| note.split(": ").next().unwrap())
        .collect();
    assert_eq!(
        decisions,
        [
            "`Direction of 1st motion` kept as written, as it isn't 3 plain numbers",
            "`Max velocity` kept as written, with no unit, where a speed needs one",
            "`Pad Latitude` kept as written, as no latitude is that large",
            "a figure HPR Sim doesn't know kept as written",
        ]
    );
}

/// Padding and odd rows: a row of nothing but commas is skipped, a value keeps its own text and
/// spaces, an empty value and a temperature below absolute zero are kept as written.
#[test]
fn padding_is_dropped_and_impossible_values_kept_as_written() {
    let text = "Rocket Name,Demo,,,\n,,,\nMax Altitude,100 feet,,\n , , \n\
                Firmware,v1.2, build 3,,\nMax velocity,,,\nTemperature on pad,-500 F\n";
    let log = import::read(text.as_bytes()).unwrap();
    assert_eq!(log.stated.figures.len(), 5);
    assert_eq!(log.firmware.as_deref(), Some("v1.2, build 3"));
    assert_eq!(figure(&log, "Rocket Name").unwrap().written, "Demo");
    let speed = figure(&log, "Max velocity").unwrap();
    assert_eq!((speed.quantity, speed.written.as_str()), (None, ""));
    let cold = figure(&log, "Temperature on pad").unwrap();
    assert_eq!(cold.quantity, Some(Quantity::Unrecognized));
    assert_eq!(cold.values, [-500.0]);
    let decisions: Vec<&str> = log
        .notes
        .iter()
        .skip(1)
        .map(|note| note.split(": ").next().unwrap())
        .collect();
    assert_eq!(
        decisions,
        [
            "`Max velocity` kept as written, with no value",
            "`Temperature on pad` kept as written, as no temperature is below absolute zero",
        ]
    );
}

proptest::proptest! {
    /// Any bytes changed in a summary, or any rows appended to one, are read or refused, never a
    /// panic; what is read keeps every value finite.
    #[test]
    fn any_summary_is_read_or_refused(
        changes in proptest::collection::vec((0usize..2200, proptest::prelude::any::<u8>()), 0..8),
        rows in proptest::collection::vec("[ -~]{0,40}", 0..6),
    ) {
        let mut bytes = BLUE_RAVEN.as_bytes().to_vec();
        for (at, byte) in changes {
            if let Some(slot) = bytes.get_mut(at) {
                *slot = byte;
            }
        }
        for row in rows {
            bytes.extend(row.as_bytes());
            bytes.push(b'\n');
        }
        if let Ok(log) = import::read(&bytes) {
            for figure in &log.stated.figures {
                proptest::prop_assert!(figure.values.iter().all(|value| value.is_finite()));
            }
        }
    }
}
