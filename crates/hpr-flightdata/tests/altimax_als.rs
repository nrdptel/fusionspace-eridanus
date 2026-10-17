//! Rocketronics AltiMAX's own `.ALS` file, read through the importer from invented bytes
//! (M7.1c23): twelve records of a made-up flight, the pad at 98,000 Pa, a boost of 8 g and a
//! coast, the last four records 25 ms apart. The values each test expects are worked by hand
//! from the bytes.

use hpr_flightdata::altimax_als;
use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// Where the records start, and a record's length.
const HEADER: usize = 32;
const RECORD: usize = 22;

/// The records: event code, raw pressure in Pa (the filtered one half a pascal above it),
/// acceleration counts of 0.01 g, and whether the record is 25 ms after the one before.
const RECORDS: [(u8, u32, i16, bool); 12] = [
    (0, 98_000, -100, false),
    (0, 98_000, -100, false),
    (3, 97_990, -800, false),
    (0, 97_950, -790, false),
    (0, 97_880, -300, false),
    (0, 97_800, 20, false),
    (0, 97_740, 5, false),
    (0, 97_700, 2, false),
    (9, 97_690, 0, true),
    (42, 97_700, -100, true),
    (0, 97_720, -100, true),
    (14, 97_740, -100, true),
];

/// The invented file: its header, with the zero and apogee pressures and the last record's
/// index, then the records, each record's voltage channel 1 to 5 in turn and its count 500 plus
/// its index.
fn als() -> Vec<u8> {
    let mut bytes = vec![0_u8; HEADER];
    bytes[4..8].copy_from_slice(&98_000_u32.to_le_bytes());
    bytes[8..12].copy_from_slice(&97_690_u32.to_le_bytes());
    bytes[26..28].copy_from_slice(&11_u16.to_le_bytes());
    for (index, (event, raw_pa, counts, slow)) in RECORDS.into_iter().enumerate() {
        let channel = (index % 5 + 1) as u8;
        bytes.push(channel << 3 | if slow { 0b101 } else { 0b001 });
        bytes.push(event);
        bytes.extend((raw_pa as f32 + 0.5).to_le_bytes());
        bytes.extend(raw_pa.to_le_bytes());
        bytes.extend(counts.to_le_bytes());
        bytes.extend((500 + index as u16).to_le_bytes());
        bytes.extend([0; 8]);
    }
    bytes
}

/// The byte where record `index` starts.
fn at(index: usize) -> usize {
    HEADER + RECORD * index
}

/// The channel of a column.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// A refusal's offset and message.
fn refusal(bytes: &[u8]) -> (usize, String) {
    match altimax_als::read(bytes) {
        Err(LogError::Binary {
            format: "AltiMAX .ALS",
            offset,
            message,
        }) => (offset, message),
        other => panic!("{other:?}"),
    }
}

/// The clock, 5 ms a record and 25 ms for the slower four, summed from the first at 5 ms.
fn clock() -> Vec<f64> {
    let mut time = 0.0;
    RECORDS
        .iter()
        .map(|(.., slow)| {
            time += if *slow { 0.025 } else { 0.005 };
            time
        })
        .collect()
}

/// Every stored value as a channel on the records' clock, in units: the pressures as stored,
/// the acceleration at 0.0981 m/s² a count, the event code and voltage channel as numbers, and
/// each voltage channel's counts on its own records; no primary track.
#[test]
fn an_als_reads_in_units() {
    let bytes = als();
    assert!(altimax_als::recognizes(&bytes));
    assert_eq!(import::detect(&bytes), Some(LogFormat::AltimaxAls));
    let log = import::read(&bytes).unwrap();
    assert_eq!(log.format, LogFormat::AltimaxAls);
    assert_eq!(log.logger, "Rocketronics AltiMAX");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert_eq!(log.stated.samples, Some(12));
    let figures: Vec<_> = log
        .stated
        .figures
        .iter()
        .map(|f| (f.name.as_str(), f.written.as_str(), f.values.clone()))
        .collect();
    assert_eq!(
        figures,
        [
            ("zero pressure", "98000 Pa", vec![98_000.0]),
            ("apogee pressure", "97690 Pa", vec![97_690.0])
        ]
    );

    let time = clock();
    assert!((time[7] - 0.04).abs() < 1e-12 && (time[11] - 0.14).abs() < 1e-12);
    let filtered = channel(&log, "PRESS FILTER");
    assert_eq!(filtered.quantity, Quantity::Pressure);
    assert_eq!(filtered.time_s, time);
    assert_eq!(filtered.values[0], 98_000.5);
    let raw = channel(&log, "PRESS RAW");
    assert_eq!(raw.values[8], 97_690.0);
    let accel = channel(&log, "ACCEL");
    assert_eq!(accel.quantity, Quantity::Acceleration);
    // −800 counts of 0.01 g, at the export's 9.81 m/s² a g: the boost, the axis down the rocket.
    assert_eq!(accel.values[2], -800.0 * 0.0981);
    assert_eq!(accel.values[0], -100.0 * 0.0981);
    assert_eq!(channel(&log, "EVENT").values[2], 3.0);
    assert_eq!(
        channel(&log, "VOLT. CHANNEL").values[..6],
        [1.0, 2.0, 3.0, 4.0, 5.0, 1.0]
    );
    let fifth = channel(&log, "VOLTAGE count (channel 5)");
    assert_eq!(fifth.quantity, Quantity::Unrecognized);
    assert_eq!(fifth.time_s, [time[4], time[9]]);
    assert_eq!(fifth.values, [504.0, 509.0]);
    assert_eq!(
        channel(&log, "VOLTAGE count (channel 1)").values,
        [500.0, 505.0, 510.0]
    );

    let events: Vec<_> = log
        .events
        .iter()
        .map(|e| (e.name.as_str(), e.time_s))
        .collect();
    assert_eq!(
        events,
        [
            ("BARO-LIFTOFF", time[2]),
            ("APOGEE PRESSURE", time[8]),
            ("event 42", time[9]),
            ("LAND", time[11])
        ]
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("the clock isn't stored"))
    );
    assert!(log.notes.iter().any(|n| n.contains("kept as written")));
}

/// The whole record, pinned.
#[test]
fn snapshot() {
    insta::assert_json_snapshot!(altimax_als::read(&als()).unwrap());
}

/// A file cut inside its records is read to its last whole one, with a note; a trailer is left
/// unread, with a note.
#[test]
fn a_short_file_reads_its_whole_records_and_a_trailer_is_noted() {
    let bytes = als();
    let cut = altimax_als::read(&bytes[..at(5) + 7]).unwrap();
    assert_eq!(channel(&cut, "PRESS RAW").values.len(), 5);
    assert!(
        cut.notes
            .iter()
            .any(|n| n == "the header states 12 records, but the file ends after 5: read those")
    );
    let mut trailed = bytes.clone();
    trailed.extend([0; 100]);
    let log = altimax_als::read(&trailed).unwrap();
    assert_eq!(channel(&log, "PRESS RAW").values.len(), 12);
    assert!(
        log.notes
            .iter()
            .any(|n| n == "100 bytes after the records, a trailer, aren't read")
    );
}

/// Each refusal names its byte.
#[test]
fn refusals_name_the_byte() {
    let edit = |at: usize, byte: u8| {
        let mut bytes = als();
        bytes[at] = byte;
        bytes
    };
    // Bit 1 of a kind set; a channel out of turn; a nonzero byte among the eight zeros.
    let (offset, message) = refusal(&edit(at(3), 4 << 3 | 0b011));
    assert_eq!(offset, at(3));
    assert!(message.starts_with("record 4: its kind, 0x23, has bit 0 clear or bit 1 set"));
    let (offset, message) = refusal(&edit(at(4), 1 << 3 | 0b001));
    assert_eq!(offset, at(4));
    assert!(
        message.contains("its voltage channel is 1, where the channels take turns and 5 follows 4")
    );
    let (offset, message) = refusal(&edit(at(5) + 21, 1));
    assert_eq!(offset, at(5));
    assert!(message.contains("last 8 bytes aren't zero"));
    // A channel past the fifth.
    let (_, message) = refusal(&edit(at(0), 6 << 3 | 0b001));
    assert!(message.contains("names voltage channel 6"));

    // The first raw pressure not a pad's, in hectopascals, say.
    let mut bytes = als();
    bytes[at(0) + 6..at(0) + 10].copy_from_slice(&980_u32.to_le_bytes());
    let (offset, message) = refusal(&bytes);
    assert_eq!(offset, at(0) + 6);
    assert!(message.contains("980 Pa, isn't a pad's"));

    // A filtered pressure that isn't a number.
    let mut bytes = als();
    bytes[at(7) + 2..at(7) + 6].copy_from_slice(&f32::NAN.to_le_bytes());
    assert_eq!(refusal(&bytes).0, at(7) + 2);

    // A record past the header's count: a flight longer than its 16-bit count.
    let mut bytes = als();
    bytes[26..28].copy_from_slice(&10_u16.to_le_bytes());
    let (offset, message) = refusal(&bytes);
    assert_eq!(offset, at(11));
    assert!(message.contains("a record follows the 11 the header states"));

    // Too short for a header and a record.
    assert!(matches!(
        altimax_als::read(&als()[..at(1) - 1]),
        Err(LogError::NotThisFormat { .. })
    ));
}

/// The recognizer takes the file and its cut copies, and nothing whose records break the rules.
#[test]
fn recognition() {
    let bytes = als();
    assert!(altimax_als::recognizes(&bytes[..at(1)]));
    assert!(!altimax_als::recognizes(&bytes[..at(1) - 1]));
    assert!(!altimax_als::recognizes(&[0; 400]));
    let mut zeroed = bytes.clone();
    zeroed[at(2) + 14] = 7;
    assert!(!altimax_als::recognizes(&zeroed));
    let mut turned = bytes;
    turned[at(1)] = 3 << 3 | 1;
    assert!(!altimax_als::recognizes(&turned));
}
