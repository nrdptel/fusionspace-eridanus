//! A course's Teensy data logger, read through the importer from invented files and pinned whole
//! by snapshots (M7.1c18): the tab-separated layout, its last line cut off where the logger
//! stopped, and the layout with named fields, with a tag no units line states.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// The channel of a column.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// Each tag on its own clock, `A`, `P` and `MSL` in SI as written, the rest kept as written, and
/// the cut last line dropped.
#[test]
fn the_tab_separated_layout_reads_each_tag_on_its_own_clock() {
    let bytes = include_bytes!("samples/teensy-tabs.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::TeensyTabs));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.logger, "Teensy data logger");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    let columns: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    let kept = Quantity::Unrecognized;
    assert_eq!(
        columns,
        [
            ("A 1", Quantity::Acceleration),
            ("A 2", Quantity::Acceleration),
            ("A 3", Quantity::Acceleration),
            ("G 1", kept),
            ("G 2", kept),
            ("G 3", kept),
            ("M 1", kept),
            ("M 2", kept),
            ("M 3", kept),
            ("Alt P", Quantity::Pressure),
            ("Alt MSL", Quantity::AltitudeMsl),
            ("HG 1", kept),
            ("HG 2", kept),
            ("HG 3", kept),
            ("ADC Pin-35 1", kept),
            ("ADC Pin-36 1", kept),
        ]
    );
    let a = channel(&log, "A 3");
    assert_eq!(a.time_s, [0.002, 0.025]);
    assert_eq!(a.values, [9.75, 9.80]);
    let msl = channel(&log, "Alt MSL");
    assert_eq!(msl.time_s, [0.0031, 0.0261]);
    assert_eq!(msl.values, [540.42, 541.47]);
    assert_eq!(channel(&log, "HG 3").values, [10.5337, 10.0549]);
    note(&log, "the last line, which ends the file with no line end");
    insta::assert_json_snapshot!(log);
}

/// Each field in its group's stated unit, `milli-T` and the unstated `VOL` kept as written.
#[test]
fn the_named_layout_reads_each_field_in_its_stated_unit() {
    let bytes = include_bytes!("samples/teensy-named.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::TeensyNamed));
    let log = import::read(bytes).unwrap();
    let za = channel(&log, "ICM za");
    assert_eq!(za.quantity, Quantity::Acceleration);
    assert_eq!(za.time_s, [500.0005, 500.0015]);
    assert_eq!(za.values[0], 998.40 * 9.806_65 / 1_000.0);
    assert_eq!(channel(&log, "LIS zh").values[1], 9.806_65);
    assert_eq!(channel(&log, "ICM xg").quantity, Quantity::AngularRate);
    assert_eq!(channel(&log, "ICM T").values[0], 24.50 + 273.15);
    assert_eq!(channel(&log, "MS5 P").values, [95_000.0, 94_998.0]);
    assert_eq!(channel(&log, "ADC T1").quantity, Quantity::Temperature);
    let xm = channel(&log, "ICM xm");
    assert_eq!((xm.quantity, xm.values[0]), (Quantity::Unrecognized, 30.15));
    let vol = channel(&log, "VOL V");
    assert_eq!((vol.quantity, vol.values[0]), (Quantity::Unrecognized, 4.105));
    note(&log, "VOL's values kept as written: no units line states its units");
    let units = note(&log, "kept as written, in a unit stated");
    assert!(units.contains("`ICM zm`, in milli-T"), "{units}");
    insta::assert_json_snapshot!(log);
}
