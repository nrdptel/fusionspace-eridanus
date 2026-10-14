//! AED Electronics R-DAS raw data, as its Windows software exports a flight to text, read through
//! the importer from an invented file and pinned whole by a snapshot (M7.1c8): CR LF line ends, a
//! configuration naming two of the six analog inputs, 10 Hz from 0.3 s before the trigger, a
//! `Dig` word with letters in it and an `ADC1` cell that isn't a number.

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
        .unwrap_or_else(|| panic!("no `{column}` channel"))
}

/// Every column a channel on the file's clock, counts as written, `Dig` read in hexadecimal; no
/// primary track.
#[test]
fn the_raw_export_keeps_every_count_as_written() {
    let bytes = include_bytes!("samples/rdas-raw.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::RdasText));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::RdasText);
    assert_eq!(log.logger, "AED Electronics R-DAS");
    assert_eq!(log.firmware.as_deref(), Some("4.1"));
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    let quantities: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        quantities,
        [
            ("Acc", Quantity::Unrecognized),
            ("Prs", Quantity::Unrecognized),
            ("Dig", Quantity::Number),
            ("ADC0", Quantity::Unrecognized),
            ("ADC1", Quantity::Unrecognized),
        ]
    );
    let acc = channel(&log, "Acc");
    assert_eq!(acc.time_s.len(), 11);
    assert_eq!((acc.time_s[0], acc.time_s[10]), (-0.3, 0.7));
    // The fifth row: 700 counts, at 0.1 s.
    assert_eq!((acc.time_s[4], acc.values[4]), (0.1, 700.0));
    assert_eq!(channel(&log, "Prs").values[10], 794.0);
    // `00B0` and `80B0` in hexadecimal: 176 and 32,944. Read as decimal they would fail.
    let dig = &channel(&log, "Dig").values;
    assert_eq!(
        (dig[3], dig[7], dig[9], dig[10]),
        (16.0, 32.0, 176.0, 32_944.0)
    );
    // `x` is a gap, counted.
    let adc1 = &channel(&log, "ADC1").values;
    assert!(adc1[5].is_nan() && adc1[6] == 255.0);
    note(&log, "1 cell that isn't a number read as a gap, on line 22");
    let clock = note(&log, "`Time` read in seconds: ");
    assert!(clock.contains("the 10 Hz line 6 states"), "{clock}");
    assert!(!clock.contains("steps are under half"), "{clock}");
    note(
        &log,
        "`Acc`, `Prs`, `ADC0`, `ADC1` kept as written: the converters' counts",
    );
    note(&log, "`Dig` read in hexadecimal and kept as its number: ");
    let configuration = note(&log, "the configuration, as written: ");
    assert!(
        configuration.contains("Trigger: G-switch. | Smart Recovery option enabled"),
        "{configuration}"
    );
    insta::assert_json_snapshot!(log);
}
