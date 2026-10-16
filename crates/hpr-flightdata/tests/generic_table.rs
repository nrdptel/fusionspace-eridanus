//! Tables read by their headers, through the importer, from two invented files pinned whole by
//! snapshots (M7.1c13). The first is a made-up flight at 10 rows a second, its units in
//! underscored endings (`time_ms`, `baro__kpa`, `accel_z_g`), a flight phase in words, a speed
//! whose name and unit disagree (`vz_filtered_ms`) and one with no unit. The second has a `sep=;`
//! line, a byte-order mark, CR LF line ends, decimal commas, units in brackets, a quoted name
//! holding the separator, a temperature whose values contradict its unit, and a column of remarks.
//! The values each test expects are worked by hand from the files. Two more invented files are
//! read by a caller's mapping (M7.1c22): stamped rows, an accelerometer's tuple behind a date and
//! time with a line marking motion; and rows written as lists on a 32-bit count of microseconds
//! that turns over, printed 9 × 2³² high in the upper half of its turn.

use hpr_flightdata::generic;
use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const UNITS: &[u8] = include_bytes!("samples/table-units.csv");
const SEMICOLON: &[u8] = include_bytes!("samples/table-semicolon.csv");
const STAMPED: &[u8] = include_bytes!("samples/table-stamped.txt");
const LIST: &[u8] = include_bytes!("samples/table-list.txt");

/// Each channel's name and quantity.
fn layout(log: &FlightLog) -> Vec<(&str, Quantity)> {
    log.channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect()
}

/// The values of a column's channel.
#[cfg(test)]
fn values<'a>(log: &'a FlightLog, column: &str) -> &'a [f64] {
    &log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
        .values
}

#[test]
fn underscored_units_read_and_the_rest_is_kept() {
    assert_eq!(import::detect(UNITS), Some(LogFormat::GenericTable));
    let log = import::read(UNITS).unwrap();
    assert_eq!(log.format, LogFormat::GenericTable);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert_eq!(
        layout(&log),
        [
            ("alt_baro_agl_ft", Quantity::AltitudeAboveGround),
            ("baro__kpa", Quantity::Pressure),
            ("accel_z_g", Quantity::Acceleration),
            ("vz_filtered_ms", Quantity::Unrecognized),
            ("batt_v", Quantity::Voltage),
            ("Velocity_Up", Quantity::Unrecognized),
        ]
    );
    let clock = &log.channels[0].time_s;
    assert_eq!(clock[1], 0.1);
    assert_eq!(clock[7], 0.7);
    // 64 ft × 0.3048; 98.50 kPa; 6.20 g × 9.806 65.
    assert!((values(&log, "alt_baro_agl_ft")[7] - 19.5072).abs() < 1e-12);
    assert_eq!(values(&log, "baro__kpa")[0], 98_500.0);
    assert!((values(&log, "accel_z_g")[1] - 60.801_23).abs() < 1e-9);
    // Kept as written: the speed whose unit is a time's, and the one with no unit.
    assert_eq!(values(&log, "vz_filtered_ms")[3], 15.0);
    assert!(values(&log, "Velocity_Up")[5].is_nan());
    let events: Vec<(f64, &str)> = log
        .events
        .iter()
        .map(|e| (e.time_s, e.name.as_str()))
        .collect();
    assert_eq!(
        events,
        [
            (0.0, "PAD"),
            (0.2, "BOOST"),
            (0.4, "COAST"),
            (0.7, "APOGEE")
        ]
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("\"vz_filtered_ms\"'s name says a speed"))
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("\"Velocity_Up\" states no unit"))
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_semicolon_table_with_decimal_commas_reads_by_its_brackets() {
    assert_eq!(import::detect(SEMICOLON), Some(LogFormat::GenericTable));
    let log = import::read(SEMICOLON).unwrap();
    assert_eq!(
        layout(&log),
        [
            ("Altitude (m ASL)", Quantity::Altitude),
            ("Temp [K]", Quantity::Unrecognized),
            ("Speed; up (m/s)", Quantity::Speed),
            ("Lat (deg)", Quantity::Latitude),
        ]
    );
    assert_eq!(log.channels[0].time_s, [0.0, 0.5, 1.0, 1.5]);
    assert_eq!(values(&log, "Altitude (m ASL)")[1], 1512.5);
    assert_eq!(values(&log, "Speed; up (m/s)")[2], 81.5);
    // 22.5 K is colder than any logger reads: the column stays as written.
    assert_eq!(values(&log, "Temp [K]")[0], 22.5);
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("\"Temp [K]\"'s values contradict"))
    );
    // The remarks are words, in a column whose name says no state: left out.
    assert!(log.events.is_empty());
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("\"Remark\" holds text"))
    );
    insta::assert_json_snapshot!(log);
}

/// The mapping the header states, changed by a caller, reads the same file another way.
#[test]
fn a_callers_mapping_reads_what_the_header_leaves_out() {
    let text = import::decode(UNITS);
    let mut mapping = generic::mapping(&text).unwrap();
    assert_eq!(mapping.header_line, Some(2));
    assert_eq!(mapping.clock, 0);
    let up = mapping
        .columns
        .iter_mut()
        .find(|c| c.name == "Velocity_Up")
        .unwrap();
    up.quantity = Some(Quantity::Speed);
    up.unit = Some("ft/s".to_owned());
    let log = generic::read_with(&text, &mapping).unwrap();
    assert!((values(&log, "Velocity_Up")[2] - 14.0 * 0.3048).abs() < 1e-12);
    // A unit that measures something else than the quantity is refused, naming both.
    let up = mapping
        .columns
        .iter_mut()
        .find(|c| c.name == "Velocity_Up")
        .unwrap();
    up.unit = Some("kPa".to_owned());
    let error = generic::read_with(&text, &mapping).unwrap_err().to_string();
    assert!(
        error.contains("a speed") && error.contains("a pressure"),
        "{error}"
    );
}

/// A mapping's JSON, as a caller writes it.
#[cfg(test)]
fn mapping(json: &str) -> generic::Mapping {
    serde_json::from_str(json).expect("a mapping")
}

/// Stamped rows: the clock from each stamp, the tuple's three axes, and the mark an event at the
/// next row's time.
#[test]
fn stamped_rows_read_by_a_mapping() {
    let text = import::decode(STAMPED);
    assert!(import::read(STAMPED).is_err());
    let log = generic::read_with(
        &text,
        &mapping(
            r#"{"header_line": null, "separator": ",", "decimal_comma": false, "clock": 0,
                "rows": "stamped", "columns": [
                {"name": "stamp", "quantity": null, "unit": "s"},
                {"name": "x", "quantity": "acceleration", "unit": "m/s^2"},
                {"name": "y", "quantity": "acceleration", "unit": "m/s^2"},
                {"name": "z", "quantity": "acceleration", "unit": "m/s^2"}]}"#,
        ),
    )
    .unwrap();
    // 2024-06-02 is day 19,876 from 1970-01-01.
    let midnight = 19_876.0 * 86_400.0;
    assert_eq!(
        log.channels[2].time_s,
        [
            midnight - 1.5,
            midnight - 1.375,
            midnight - 1.25,
            midnight - 1.125,
            midnight
        ]
    );
    assert_eq!(values(&log, "z"), [9.81, 9.79, 40.2, 45.6, 0.0]);
    assert_eq!(log.events.len(), 1);
    assert_eq!(
        (log.events[0].name.as_str(), log.events[0].time_s),
        ("MOTION DETECTED", midnight - 1.25)
    );
    insta::assert_json_snapshot!(log);
}

/// Rows written as lists on a clock that turns over: each count taken modulo 2³², one turn
/// added, and the words of the last column left out as text.
#[test]
fn list_rows_on_a_clock_that_turns_over() {
    let text = import::decode(LIST);
    let log = generic::read_with(
        &text,
        &mapping(
            r#"{"header_line": null, "separator": ",", "decimal_comma": false, "clock": 0,
                "clock_wrap": 4294967296.0, "rows": "list", "columns": [
                {"name": "clock", "quantity": null, "unit": "us"},
                {"name": "pressure", "quantity": "pressure", "unit": "Pa"},
                {"name": "fix", "quantity": null, "unit": null}]}"#,
        ),
    )
    .unwrap();
    assert_eq!(layout(&log), [("pressure", Quantity::Pressure)]);
    assert_eq!(
        log.channels[0].time_s,
        [4_293.967_296, 4_294.467_296, 4_294.967_296, 4_295.467_296]
    );
    assert_eq!(
        values(&log, "pressure"),
        [101_325.0, 101_310.5, 101_290.0, 101_270.25]
    );
    assert_eq!(log.format, LogFormat::GenericTable);
    insta::assert_json_snapshot!(log);
}
