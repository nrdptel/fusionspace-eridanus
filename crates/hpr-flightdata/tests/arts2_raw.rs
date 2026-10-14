//! Ozark Aerospace ARTS2's raw data, the `.odf` file and the raw CSV export, read through the
//! importer from an invented pair (M7.1c11): 150 records of a made-up flight, an 8 g boost for
//! 2.4 s then a coast at −1.3 g, its barometer's counts from the standard atmosphere's pressure
//! at the climb. The settings block holds the pad's barometer count 800 and eight times its
//! accelerometer count, 4,100 (512.5 counts), and calibration sums of 64 × 496, 32,250 and
//! 64 × 512: 8 counts a g, upward as the count rises. The values each test expects are worked by
//! hand from those.

use hpr_flightdata::arts2_raw;
use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

const ODF: &[u8] = include_bytes!("samples/arts2-raw.odf");
const CSV: &str = include_str!("samples/arts2-raw.csv");

/// Where the records start, the record count, and the settings block, in the `.odf`.
const RECORDS_AT: usize = 305;
const COUNT_AT: usize = 289;
const BLOCK_AT: usize = 33;

/// The channel of a column.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// The invented `.odf` with a 16-bit word set at an offset.
fn with_word(at: usize, value: u16) -> Vec<u8> {
    let mut bytes = ODF.to_vec();
    bytes[at..at + 2].copy_from_slice(&value.to_le_bytes());
    bytes
}

/// The invented `.odf` cut to its first `n` records, its count set to match.
fn with_records(n: usize) -> Vec<u8> {
    let mut bytes = ODF[..RECORDS_AT + 8 * n].to_vec();
    bytes[COUNT_AT..COUNT_AT + 4].copy_from_slice(&(n as u32).to_le_bytes());
    bytes.extend_from_slice(&[0; 124]);
    bytes
}

/// The `.odf` with every record's words passed through `edit`.
fn with_every_record(edit: impl Fn(usize, &mut [u16; 4])) -> Vec<u8> {
    let mut bytes = ODF.to_vec();
    for k in 0..150 {
        let at = RECORDS_AT + 8 * k;
        let mut words =
            [0, 2, 4, 6].map(|w| u16::from_le_bytes([bytes[at + w], bytes[at + w + 1]]));
        edit(k, &mut words);
        for (w, word) in words.iter().enumerate() {
            bytes[at + 2 * w..at + 2 * w + 2].copy_from_slice(&word.to_le_bytes());
        }
    }
    bytes
}

/// A refusal's offset and message.
#[cfg(test)]
fn refusal(bytes: &[u8]) -> (usize, String) {
    match arts2_raw::read_odf(bytes) {
        Err(LogError::Binary {
            format: "ARTS2 .odf",
            offset,
            message,
        }) => (offset, message),
        other => panic!("{other:?}"),
    }
}

/// The record's four words as channels in units, 0.02 s apart from the first: the acceleration
/// by the calibration, the pressure by the sensor's transfer function, the flags byte and the
/// state as written; no primary track.
#[test]
fn an_odf_reads_in_units() {
    assert_eq!(import::detect(ODF), Some(LogFormat::Arts2Odf));
    let log = import::read(ODF).unwrap();
    assert_eq!(log.format, LogFormat::Arts2Odf);
    assert_eq!(log.logger, "Ozark Aerospace ARTS2");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert_eq!(log.stated.samples, Some(150));
    let columns: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        columns,
        [
            ("Acc(HI)", Quantity::Acceleration),
            ("Acc/Baro(LO)", Quantity::Unrecognized),
            ("Baro(HI)", Quantity::Pressure),
            ("State", Quantity::Number),
        ]
    );
    let acc = channel(&log, "Acc(HI)");
    assert_eq!(acc.values.len(), 150);
    assert_eq!(acc.time_s[1], 0.02);
    assert_eq!(acc.time_s[149], 149.0 / 50.0);
    // The first record, the baselines: 512 against the pad's 512.5, −1/16 g.
    assert!(
        (acc.values[0] + 9.806_65 / 16.0).abs() < 1e-12,
        "{}",
        acc.values[0]
    );
    // Count 576 in the boost: (576 − 512.5) / 8 g = 7.9375 g = 77.840 m/s².
    assert!(
        (acc.values[1] - 7.9375 * 9.806_65).abs() < 1e-9,
        "{}",
        acc.values[1]
    );
    // Count 800: (800 / 1,024 + 0.1518) / 0.010 59 kPa = 88,106.7 Pa.
    let baro = channel(&log, "Baro(HI)");
    assert!(
        (baro.values[0] - 88_106.704).abs() < 1e-3,
        "{}",
        baro.values[0]
    );
    assert_eq!(channel(&log, "State").values[0], 64.0);
    let figures: Vec<(&str, &str, Vec<f64>)> = log
        .stated
        .figures
        .iter()
        .map(|f| (f.name.as_str(), f.written.as_str(), f.values.clone()))
        .collect();
    assert_eq!(figures[1], ("Zero G baseline", "4100", vec![512.5]));
    assert_eq!(
        figures[2],
        (
            "calibration sums",
            "31744,32250,32768",
            vec![31_744.0, 32_250.0, 32_768.0]
        )
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("a ratio of 1.012, within 0.6 to 5/3")),
        "{:#?}",
        log.notes
    );
}

/// The raw CSV holds the `.odf`'s records, word for word, as counts on its own clock.
#[test]
fn the_raw_csv_keeps_the_same_records_as_counts() {
    assert_eq!(import::detect(CSV.as_bytes()), Some(LogFormat::Arts2RawCsv));
    let csv = import::read(CSV.as_bytes()).unwrap();
    assert_eq!(csv.format, LogFormat::Arts2RawCsv);
    assert!(csv.time_s.is_empty() && csv.altitude_m.is_empty());
    let quantities: Vec<Quantity> = csv.channels.iter().map(|c| c.quantity).collect();
    assert_eq!(
        quantities,
        [
            Quantity::Unrecognized,
            Quantity::Unrecognized,
            Quantity::Unrecognized,
            Quantity::Number,
        ]
    );
    let odf = import::read(ODF).unwrap();
    for (column, from_count) in [
        ("Acc(HI)", None),
        ("Baro(HI)", Some(())),
        ("Acc/Baro(LO)", None),
        ("State", None),
    ] {
        let ours = channel(&csv, column);
        let theirs = channel(&odf, column);
        assert_eq!(ours.time_s, theirs.time_s, "{column}");
        if from_count.is_some() {
            for (count, pa) in ours.values.iter().zip(&theirs.values) {
                let expected = (count / 1_024.0 + 0.1518) / 0.010_59 * 1_000.0;
                assert_eq!(*pa, expected);
            }
        } else if column == "Acc(HI)" {
            for (count, a) in ours.values.iter().zip(&theirs.values) {
                assert!(((count - 512.5) / 8.0 * 9.806_65 - a).abs() < 1e-9);
            }
        } else {
            assert_eq!(ours.values, theirs.values, "{column}");
        }
    }
}

/// The two records' notes and figures, and the first rows, pinned whole.
#[test]
fn both_records_are_pinned() {
    for (name, bytes) in [("odf", ODF), ("csv", CSV.as_bytes())] {
        let mut log = import::read(bytes).unwrap();
        for channel in &mut log.channels {
            channel.time_s.truncate(4);
            channel.values.truncate(4);
        }
        insta::assert_json_snapshot!(name, log);
    }
}

/// A file that doesn't open as an `.odf` isn't one; the importer says it reads no such layout.
#[test]
fn another_file_isnt_an_odf() {
    let mut bytes = ODF.to_vec();
    bytes[4] = b'o';
    assert!(matches!(
        arts2_raw::read_odf(&bytes),
        Err(LogError::NotThisFormat {
            format: "ARTS2 .odf",
            ..
        })
    ));
    assert_eq!(import::detect(&bytes), None);
}

/// Each layout fault is refused at its byte, by its own message.
#[test]
fn a_file_laid_out_otherwise_is_refused() {
    // One byte short: 150 records make 1,629 bytes.
    let (offset, message) = refusal(&ODF[..ODF.len() - 1]);
    assert_eq!(offset, COUNT_AT);
    assert!(
        message.contains("make a file of 1629 bytes, not 1628"),
        "{message}"
    );
    // Shorter than the header and the settings.
    let (_, message) = refusal(&ODF[..400]);
    assert!(message.contains("before its 429 of header"), "{message}");
    // The memory sizes disagree; then agree but can't hold the records.
    let (offset, message) = refusal(&with_word(COUNT_AT + 8, 1_000));
    assert_eq!(offset, COUNT_AT);
    assert!(message.contains("the sizes must agree"), "{message}");
    let mut bytes = ODF.to_vec();
    for k in 0..3 {
        bytes[COUNT_AT + 4 + 4 * k..COUNT_AT + 8 + 4 * k].copy_from_slice(&100u32.to_le_bytes());
    }
    assert!(
        refusal(&bytes)
            .1
            .contains("150 records and memory sizes 100")
    );
    // A record's flags byte without the barometer's low bits, at record 5.
    let at = RECORDS_AT + 8 * 5;
    let flags = u16::from_le_bytes([ODF[at + 2], ODF[at + 3]]);
    let (offset, message) = refusal(&with_word(at + 2, flags ^ 0b100));
    assert_eq!(offset, at);
    assert!(message.starts_with("record 5: the flags byte"), "{message}");
    // A count over the converter's, and a state over a byte.
    assert!(refusal(&with_word(at, 1_024)).1.contains("10-bit"));
    assert!(
        refusal(&with_word(at + 6, 256))
            .1
            .contains("aren't both a byte")
    );
}

/// The first record must be the settings block's baselines.
#[test]
fn a_first_record_that_isnt_the_baselines_is_refused() {
    let (offset, message) = refusal(&with_word(BLOCK_AT + 10, 801));
    assert_eq!(offset, RECORDS_AT);
    assert!(message.contains("aren't the pad's baselines"), "{message}");
    // The accelerometer's: 512 is 4,096 to 4,103 eighths cut; 4,104 isn't.
    assert!(arts2_raw::read_odf(&with_word(BLOCK_AT + 12, 4_103)).is_ok());
    assert!(
        refusal(&with_word(BLOCK_AT + 12, 4_104))
            .1
            .contains("baselines")
    );
}

/// A pad pressure outside the sensor's rated 20 to 105 kPa is refused: count 983 is 104.98 kPa
/// and 984 is 105.07.
#[test]
fn a_pad_pressure_outside_the_sensors_range_is_refused() {
    let pad = |count: u16| {
        let mut bytes = with_every_record(|k, words| {
            if k == 0 {
                words[2] = count;
                words[1] = (count & 3) << 2;
            }
        });
        bytes[BLOCK_AT + 10..BLOCK_AT + 12].copy_from_slice(&count.to_le_bytes());
        arts2_raw::read_odf(&bytes)
    };
    let error = pad(984).unwrap_err().to_string();
    assert!(
        error.contains("outside the MPX4100A's 20 to 105 kPa"),
        "{error}"
    );
    // 983 passes the band; the climb by the pressure, from a pad 1.6 km below the flight's
    // start, is then far more than the acceleration's.
    let error = pad(983).unwrap_err().to_string();
    assert!(error.contains("not both upward"), "{error}");
}

/// A calibration the ADXL78's data sheet doesn't bear out is refused at the sums.
#[test]
fn a_calibration_the_data_sheet_doesnt_bear_out_is_refused() {
    // Ends 7 counts a g apart: 34.2 mV.
    let mut bytes = with_word(BLOCK_AT + 20, 32_768 - 896);
    let (offset, message) = refusal(&bytes);
    assert_eq!(offset, BLOCK_AT + 20);
    assert!(
        message.contains("outside the ADXL78's 36.1 to 39.9"),
        "{message}"
    );
    // The side reading outside its ends.
    bytes = with_word(BLOCK_AT + 22, 33_000);
    assert!(refusal(&bytes).1.contains("isn't zero g"));
}

/// The climb check refuses a flight whose barometer never moves, one whose acceleration's scale
/// is off by 4 (the clock off by 2 would do the same), and one too short to measure.
#[test]
fn a_climb_that_disagrees_or_cant_be_measured_is_refused() {
    let still = with_every_record(|_, words| {
        words[2] = 800;
        // Count 800's low two bits are 0.
        words[1] &= !0b1100;
    });
    assert!(refusal(&still).1.contains("not both upward"));
    // Counts 4 times further from the pad: the climb by the acceleration 4 times the pressure's.
    let fast = with_every_record(|k, words| {
        if k > 0 {
            words[0] = (f64::from(words[0]) * 4.0 - 3.0 * 512.5).round() as u16;
        }
    });
    let message = refusal(&fast).1;
    assert!(message.contains("a ratio of 0.25"), "{message}");
    let (offset, message) = refusal(&with_records(102));
    assert_eq!(offset, RECORDS_AT);
    assert!(
        message.contains("can't be measured: 102 records"),
        "{message}"
    );
    assert!(arts2_raw::read_odf(&with_records(103)).is_ok());
}

/// No records, or the baselines' alone, hold no data.
#[test]
fn a_file_without_samples_has_no_data() {
    for n in [0, 1] {
        assert!(matches!(
            arts2_raw::read_odf(&with_records(n)),
            Err(LogError::NoData {
                format: "ARTS2 .odf"
            })
        ));
    }
}

/// The CSV's faults, each refused at its line: a short row, a count that isn't one, a count over
/// the converter's, a time that doesn't rise, a flags byte without the barometer's low bits.
#[test]
fn a_raw_csv_row_that_cant_be_a_record_is_refused() {
    let header = "MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n0.000,512,0,800,64\n";
    let cases = [
        ("0.020,576,0,800\n", "4 cells, not 5"),
        ("0.020,57.6,0,800,0\n", "`Acc(HI)` 57.6 isn't a count"),
        ("0.020,1024,0,800,0\n", "10-bit"),
        ("0.000,576,0,800,0\n", "isn't after the row before's"),
        (
            "0.020,576,4,800,0\n",
            "doesn't hold the barometer count 800's low bits",
        ),
        ("nan,576,0,800,0\n", "`MET` nan isn't a number"),
    ];
    for (row, why) in cases {
        match arts2_raw::read_csv(&format!("{header}{row}")) {
            Err(LogError::Syntax {
                format: "ARTS2 raw CSV",
                line: 3,
                message,
            }) => assert!(message.contains(why), "{row}: {message}"),
            other => panic!("{row}: {other:?}"),
        }
    }
    assert!(matches!(
        arts2_raw::read_csv("MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n\n"),
        Err(LogError::NoData { .. })
    ));
    assert!(matches!(
        arts2_raw::read_csv("# MET Acc Vel iAlt pAlt\n0.000,0,0,0,0\n"),
        Err(LogError::NotThisFormat { .. })
    ));
}

/// The header's names are compared as written, spaces about them aside; the interpreted export's
/// isn't taken for it.
#[test]
fn the_raw_csv_is_recognized_by_its_header() {
    assert!(arts2_raw::recognizes_csv(
        "\n MET, Acc(HI) ,Acc/Baro(LO),Baro(HI),State\r\n"
    ));
    assert!(!arts2_raw::recognizes_csv(
        "met,acc(hi),acc/baro(lo),baro(hi),state\n"
    ));
    assert!(!arts2_raw::recognizes_csv(
        "MET,Acc(HI),Acc/Baro(LO),Baro(HI)\n"
    ));
    assert_eq!(
        import::detect(b"# MET Acc Vel iAlt pAlt\n0.000,0.00,0.00,0.00,0.00\n"),
        Some(LogFormat::Arts2Csv)
    );
}

proptest::proptest! {
    /// Any bytes changed in the `.odf`, and any cut, are read or refused, never a panic.
    #[test]
    fn any_odf_is_read_or_refused(
        changes in proptest::collection::vec((0usize..1629, proptest::prelude::any::<u8>()), 0..8),
        cut in 0usize..1700,
    ) {
        let mut bytes = ODF.to_vec();
        for (at, byte) in changes {
            bytes[at] = byte;
        }
        bytes.truncate(cut);
        let _ = arts2_raw::read_odf(&bytes);
        let _ = import::read(&bytes);
    }

    /// Any cells after the header are read or refused, never a panic, and a read keeps its rows.
    #[test]
    fn any_raw_csv_is_read_or_refused(
        rows in proptest::collection::vec(
            ("[-0-9.e]{0,6}", "[0-9]{0,5}", "[0-9]{0,4}", "[0-9]{0,5}", "[0-9x]{0,4}"),
            0..20,
        ),
    ) {
        let mut text = "MET,Acc(HI),Acc/Baro(LO),Baro(HI),State\n".to_owned();
        for (met, acc, flags, baro, state) in &rows {
            text += &format!("{met},{acc},{flags},{baro},{state}\n");
        }
        if let Ok(log) = arts2_raw::read_csv(&text) {
            proptest::prop_assert_eq!(log.channels[0].values.len(), rows.len());
        }
    }
}
