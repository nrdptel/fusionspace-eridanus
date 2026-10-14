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
    // The worked example of docs/format/flight-logs.md reads these first rows.
    assert_eq!(&log.time_s[..3], [0.0, 0.5, 1.0]);
    let speed = log.vertical_speed_m_s.as_ref().unwrap();
    assert_eq!(format!("{:.2}", speed[2]), "26.82");
    let temperature = log.temperature_k.as_ref().unwrap();
    assert_eq!(format!("{:.1}", temperature[0]), "295.1");
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
fn a_text_export_with_no_header_row_reads_as_one_with_it() {
    let headed = import::read(include_bytes!("samples/perfectflite-export.txt")).unwrap();
    let bytes = include_bytes!("samples/perfectflite-export-headerless.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::PerfectFliteText));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFliteText);
    // Every sample as the same rows under the header read, the first row among them.
    assert_eq!(log.time_s, headed.time_s);
    assert_eq!(log.altitude_m, headed.altitude_m);
    assert_eq!(log.temperature_k, headed.temperature_k);
    assert_eq!(log.channels, headed.channels);
    // And one note more, first: the columns were taken, not read.
    assert_eq!(log.notes.len(), headed.notes.len() + 1);
    assert!(log.notes[0].contains("no header row"), "{}", log.notes[0]);
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_row_of_another_shape_is_no_headerless_export() {
    for text in [
        // The temperature with no unit letter: any table of four numbers.
        "0.00\t0\t21.5\t8.9\n0.50\t0\t21.5\t8.9\n",
        // A fifth cell.
        "0.00\t0\t21.5C\t8.9\t1\n",
        // Commas: the headerless export is the text one, tab-separated.
        "0.00,0,21.5C,8.9\n",
    ] {
        assert_eq!(import::detect(text.as_bytes()), None, "{text:?}");
        assert!(import::read(text.as_bytes()).is_err(), "{text:?}");
    }
    // A later row whose temperature is in another unit than the first's is refused.
    let mixed = "0.00\t0\t21.5C\t8.9\n0.50\t0\t70.7F\t8.9\n";
    assert!(import::read(mixed.as_bytes()).is_err());
}

#[test]
fn a_csv_export_may_name_its_temperature_temp() {
    let log = import::read(include_bytes!("samples/perfectflite-export-temp.csv")).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFliteCsv);
    let temperature = log.temperature_k.as_ref().unwrap();
    assert!(
        (temperature[0] - (17.5 + 273.15)).abs() < 1e-9,
        "{}",
        temperature[0]
    );
    assert_eq!(log.altitude_m[4], 38.0 * 0.3048);
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

/// A wide header with short rows would make the readers store a gap for every cell left out: a
/// file far sparser than its header is refused instead, before it can cost much memory.
#[test]
fn a_header_far_wider_than_its_rows_is_refused() {
    let mut export = String::from("Time,Altitude");
    let mut altos = String::from("#version,serial,flight,time,height");
    for i in 0..2_000 {
        export.push_str(",Voltage");
        altos.push_str(&format!(",x{i}"));
    }
    export.push('\n');
    altos.push('\n');
    for i in 0..200 {
        export.push_str(&format!("{}.0,0\n", i));
        altos.push_str(&format!("6,1,1,{}.0,0\n", i));
    }
    for text in [export, altos] {
        let error = import::read(text.as_bytes()).unwrap_err().to_string();
        assert!(error.contains("too few for the"), "{error}");
    }
    // The same rows under a header of their own width read.
    let mut narrow = String::from("Time,Altitude,Voltage\n");
    for i in 0..200 {
        narrow.push_str(&format!("{}.0,0\n", i));
    }
    assert!(import::read(narrow.as_bytes()).is_ok());
}

/// A record with gaps survives its own JSON: each `NaN` is written as `null` and read back.
#[test]
fn a_record_with_gaps_survives_its_json() {
    for bytes in [
        &include_bytes!("samples/raven-fip-status.csv")[..],
        &include_bytes!("samples/altos-easymega-no-pressure.csv")[..],
    ] {
        let log = import::read(bytes).unwrap();
        let gaps = log
            .channels
            .iter()
            .flat_map(|channel| &channel.values)
            .filter(|value| value.is_nan())
            .count();
        assert!(gaps > 0, "the sample has no gap to test");
        let json = serde_json::to_string(&log).unwrap();
        let back: hpr_flightdata::log::FlightLog = serde_json::from_str(&json).unwrap();
        assert_eq!(serde_json::to_string(&back).unwrap(), json);
    }
}
