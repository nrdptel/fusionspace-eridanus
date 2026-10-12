//! PerfectFlite's CSV and text exports and its `.pf2`, read through the importer from invented
//! files and pinned whole by snapshots (M7.1a, ADR-251).

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

#[test]
fn the_csv_export_reads_every_column() {
    let log = import::read(include_bytes!("samples/perfectflite-export.csv")).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFliteCsv);
    // Five columns besides the time, the one this reader doesn't know among them.
    let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
    assert_eq!(
        quantities,
        [
            Quantity::AltitudeAboveGround,
            Quantity::Speed,
            Quantity::Temperature,
            Quantity::Voltage,
            Quantity::Unrecognized
        ]
    );
    assert_eq!(
        log.altitude_m.iter().copied().fold(0.0, f64::max),
        900.0 * 0.3048
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn the_text_export_takes_its_temperature_unit_from_the_cells() {
    let log = import::read(include_bytes!("samples/perfectflite-export.txt")).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFliteText);
    let temperature = log.temperature_k.as_ref().unwrap();
    assert!(
        (temperature[0] - (21.5 + 273.15)).abs() < 1e-9,
        "{}",
        temperature[0]
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn the_committed_pf2_reads_into_channels() {
    let bytes = include_bytes!("../../../validation/fixtures/logs/synthetic-pnut.pf2");
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFlitePf2);
    // Every channel is on the log's clock and as long as it.
    for channel in &log.channels {
        assert_eq!(channel.time_s, log.time_s, "{}", channel.column);
        assert_eq!(channel.values.len(), log.time_s.len());
    }
    let altitude = log
        .channels_of(Quantity::AltitudeAboveGround)
        .next()
        .unwrap();
    assert_eq!(altitude.values, log.altitude_m);
    // Its `Data:` line's names, as written; the file is too long to snapshot whole.
    let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
    assert_eq!(
        columns,
        ["Altitude", "Velocity", "Temperature (F)", "Voltage"]
    );
}

#[test]
fn a_cell_with_another_columns_unit_is_refused() {
    let text = "Time,Altitude,Velocity,Temperature (F)\n0.00,0,0,70.1C\n";
    let error = import::read(text.as_bytes()).unwrap_err();
    assert!(error.to_string().contains("unit"), "{error}");
}
