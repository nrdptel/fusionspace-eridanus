//! Missile Works RTx's `.rtx` file and its CSV and text exports, read through the importer from an
//! invented set (M7.1c12): 30 fixes of a made-up flight, 1 s apart from 17:59:50 UTC, from a pad
//! at 1,400 m to 2,000 m at the tenth fix and back. The 13th and 14th fixes repeat the 11th's and
//! 12th's times, as a sample file's do, and the 21st fix's word 1 reads 65,282 where it should read
//! 2, as a garbled one's does. The position starts at 32°59.01234′ N, 106°58.54321′ W and moves
//! 0.00003′ north and 0.00002′ east a fix. The list has room for 40 records; the CSV writes the
//! position as `DDMM.mmmmm`, the text file in degrees cut at the sixth decimal, and both hold
//! every fix but the last. The values each test expects are worked by hand from those.

use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};
use hpr_flightdata::rtx;

const RAW: &[u8] = include_bytes!("samples/rtx.rtx");
const CSV: &str = include_str!("samples/rtx.csv");
const TEXT: &str = include_str!("samples/rtx.txt");

/// Where `_size`, the items' array record, its length, its type and its slots are, in the `.rtx`.
const SIZE_AT: usize = 186;
const ARRAY_AT: usize = 194;
const LENGTH_AT: usize = 199;
const TYPE_AT: usize = 203;
const SLOTS_AT: usize = 204;

/// The channel of a column.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// The invented `.rtx` with a 32-bit integer set at an offset.
fn with_int(at: usize, value: i32) -> Vec<u8> {
    let mut bytes = RAW.to_vec();
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
    bytes
}

/// The invented `.rtx` with slot `k` set to the slot of a word.
fn with_word(k: usize, word: u16) -> Vec<u8> {
    with_int(SLOTS_AT + 4 * k, rtx::slot(word))
}

/// An `.rtx` of the given records, with room for as many, as the RTx's software writes one.
fn build(records: &[[u16; 16]]) -> Vec<u8> {
    let mut bytes = RAW[..SIZE_AT].to_vec();
    let slots = 16 * records.len() as i32;
    bytes.extend_from_slice(&slots.to_le_bytes());
    bytes.extend_from_slice(&(slots + 1).to_le_bytes());
    bytes.extend_from_slice(&RAW[ARRAY_AT..LENGTH_AT]);
    bytes.extend_from_slice(&slots.to_le_bytes());
    bytes.push(8);
    for word in records.iter().flatten() {
        bytes.extend_from_slice(&rtx::slot(*word).to_le_bytes());
    }
    bytes.push(0x0b);
    bytes
}

/// A record at a time of day, `HHMMSS`, at 32°59.01234′ N, 106°58.54321′ W and 1,400 m.
fn fix(hhmmss: u32) -> [u16; 16] {
    let (lat, lon) = (101_234u32, 154_321u32);
    [
        (hhmmss & 0xffff) as u16,
        (hhmmss >> 16) as u16,
        3259,
        (lat & 0xffff) as u16,
        (lat >> 16) as u16,
        10658,
        (lon & 0xffff) as u16,
        (lon >> 16) as u16,
        1400,
        0,
        2,
        0x574e,
        122 + 9 * 256,
        0,
        0,
        1500,
    ]
}

/// A refusal's offset and message.
#[cfg(test)]
fn refusal(bytes: &[u8]) -> (usize, String) {
    match rtx::read_raw(bytes) {
        Err(LogError::Binary {
            format: "RTx .rtx",
            offset,
            message,
        }) => (offset, message),
        other => panic!("{other:?}"),
    }
}

/// Every word a channel on the GPS's clock, in the order the export writes them, and word 15;
/// no primary track; the garbled fix's time a gap and the repeated pair kept in order.
#[test]
fn an_rtx_reads_its_fixes() {
    assert_eq!(import::detect(RAW), Some(LogFormat::RtxRaw));
    let log = import::read(RAW).unwrap();
    assert_eq!(log.format, LogFormat::RtxRaw);
    assert_eq!(log.logger, "Missile Works RTx");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert_eq!(log.stated.samples, Some(30));
    let columns: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(
        columns,
        [
            ("Latitude", Quantity::Latitude),
            ("Longitude", Quantity::Longitude),
            ("Altitude", Quantity::GnssAltitude),
            ("Velocity", Quantity::Unrecognized),
            ("VDOP", Quantity::Number),
            ("NavStatus", Quantity::Number),
            ("Sats", Quantity::Number),
            ("SOG", Quantity::Unrecognized),
            ("COG", Quantity::Angle),
            ("word 15", Quantity::Unrecognized),
        ]
    );
    // 17:59:50 on: 0 to 11 s, then 10 and 11 again, then 12 s on; the 21st fix, which would be
    // 18 s, is a gap.
    let time = &channel(&log, "Altitude").time_s;
    let expected: Vec<f64> = (0..30)
        .map(|k| match k {
            0..12 => k as f64,
            20 => f64::NAN,
            _ => (k - 2) as f64,
        })
        .collect();
    assert_eq!(time.len(), 30);
    for (k, (t, e)) in time.iter().zip(&expected).enumerate() {
        assert!(t == e || (t.is_nan() && e.is_nan()), "{k}: {t} vs {e}");
    }
    // 32 + 59.01234 / 60 and −(106 + 58.54321 / 60), then 0.00003′ north and 0.00002′ east.
    let lat = &channel(&log, "Latitude").values;
    let lon = &channel(&log, "Longitude").values;
    assert!((lat[0] - 32.983_539).abs() < 1e-12, "{}", lat[0]);
    assert!((lon[0] + 106.975_720_166_666_7).abs() < 1e-12, "{}", lon[0]);
    assert!((lat[29] - (32.0 + 59.01321 / 60.0)).abs() < 1e-12);
    assert!((lon[29] + (106.0 + 58.54263 / 60.0)).abs() < 1e-12);
    let alt = &channel(&log, "Altitude").values;
    assert_eq!((alt[0], alt[10], alt[29]), (1400.0, 2000.0, 1400.0));
    // Climbing 60 m in a second reads 255 × 60; falling 10 m reads 10.
    let velocity = &channel(&log, "Velocity").values;
    assert_eq!((velocity[5], velocity[11]), (15_300.0, 10.0));
    let (nav, sats) = (
        &channel(&log, "NavStatus").values,
        &channel(&log, "Sats").values,
    );
    assert_eq!(
        (nav[0], sats[0], nav[29], sats[29]),
        (122.0, 9.0, 148.0, 0.0)
    );
    assert_eq!(channel(&log, "COG").values[2], 46.0);
    assert_eq!(channel(&log, "word 15").values[29], 1529.0);
    let notes = log.notes.join("\n");
    for expected in [
        "its zero is the first record's, 17:59:50",
        "times go back 1 times from one record to the next, by up to 1 s, and 2 records repeat",
        "1 records hold a time that isn't a time of day, such as 4278370088",
        "the list holds 30 records of 16 words",
    ] {
        assert!(notes.contains(expected), "{expected}\n{notes}");
    }
}

/// The CSV and the text export hold every fix but the last, as the `.rtx` reads them: the CSV's
/// positions exactly, the text file's cut toward zero at the sixth decimal; the altitude in whole
/// feet, the meters × 3.280 84 rounded down, read as 0.3048 m each.
#[test]
fn the_exports_hold_every_fix_but_the_last() {
    let raw = import::read(RAW).unwrap();
    for (text, format, names) in [
        (
            CSV,
            LogFormat::RtxCsv,
            ["Latitude", "Longitude", "Altitude", "Velocity", "COG"],
        ),
        (
            TEXT,
            LogFormat::RtxText,
            ["Lat", "Lon", "Alt", "Vel", "COG"],
        ),
    ] {
        assert_eq!(import::detect(text.as_bytes()), Some(format));
        let log = import::read(text.as_bytes()).unwrap();
        assert_eq!(log.format, format);
        assert_eq!(log.stated.samples, Some(29));
        assert_eq!(log.channels.len(), 9);
        let ours = channel(&log, names[0]);
        for (k, t) in ours.time_s.iter().enumerate() {
            let theirs = channel(&raw, "Latitude").time_s[k];
            assert!(*t == theirs || (t.is_nan() && theirs.is_nan()), "{k}");
        }
        for (name, raw_name) in names[..2].iter().zip(["Latitude", "Longitude"]) {
            for (k, value) in channel(&log, name).values.iter().enumerate() {
                let truth: f64 = channel(&raw, raw_name).values[k];
                let short = truth.abs() - value.abs();
                if format == LogFormat::RtxCsv {
                    assert_eq!(*value, truth, "{name} {k}");
                } else {
                    assert!((-1e-9..1e-6).contains(&short), "{name} {k}: {short}");
                }
            }
        }
        for (k, value) in channel(&log, names[2]).values.iter().enumerate() {
            let feet = (channel(&raw, "Altitude").values[k] * 3.280_84).floor();
            assert!((value - feet * 0.3048).abs() < 1e-9, "{k}");
        }
        assert_eq!(
            channel(&log, names[3]).values,
            channel(&raw, "Velocity").values[..29]
        );
        assert_eq!(
            channel(&log, names[4]).values,
            channel(&raw, "COG").values[..29]
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.contains("1 records hold a time"))
        );
    }
}

/// The record each reader makes, whole.
#[test]
fn records_snapshot() {
    for (name, bytes) in [
        ("raw", RAW),
        ("csv", CSV.as_bytes()),
        ("text", TEXT.as_bytes()),
    ] {
        let log = import::read(bytes).unwrap();
        insta::assert_json_snapshot!(name, log);
    }
}

/// Each layout fault refused at its byte, by its own message.
#[test]
fn layout_faults_are_refused_at_their_byte() {
    // The class's member name `_items` misspelt.
    let mut bytes = RAW.to_vec();
    bytes[159] = b'S';
    assert_eq!(
        refusal(&bytes),
        (
            159,
            "the stream isn't the RTx's list here: the list's members".to_owned()
        )
    );
    let mut bytes = RAW.to_vec();
    bytes[ARRAY_AT + 1] = 3;
    assert_eq!(refusal(&bytes).0, ARRAY_AT + 1);
    assert!(refusal(&bytes).1.contains("the items' array record"));
    let mut bytes = RAW.to_vec();
    bytes[TYPE_AT] = 7;
    assert_eq!(refusal(&bytes).0, TYPE_AT);
    assert!(refusal(&bytes).1.contains("`Int32`"));
    for size in [17, 16 * 41, -16] {
        let (offset, message) = refusal(&with_int(SIZE_AT, size));
        assert_eq!(offset, SIZE_AT);
        assert!(
            message.contains(&format!("`_size`, {size}, isn't a whole number")),
            "{message}"
        );
    }
    let (offset, message) = refusal(&with_int(LENGTH_AT, 641));
    assert_eq!(offset, LENGTH_AT);
    assert!(
        message.contains("641 slots run past the file's end"),
        "{message}"
    );
    assert!(
        refusal(&with_int(LENGTH_AT, -1))
            .1
            .contains("negative length")
    );
    // The list's length one short leaves a slot's bytes before the end record.
    let (offset, message) = refusal(&with_int(LENGTH_AT, 639));
    assert_eq!(offset, SLOTS_AT + 4 * 639);
    assert!(message.contains("`MessageEnd`"), "{message}");
    let mut bytes = RAW.to_vec();
    bytes.push(0);
    assert!(refusal(&bytes).1.contains("`MessageEnd`"));
    let mut bytes = RAW.to_vec();
    *bytes.last_mut().unwrap() = 0x0c;
    assert!(refusal(&bytes).1.contains("`MessageEnd`"));
    assert!(
        refusal(&RAW[..SIZE_AT + 4])
            .1
            .contains("ends inside the list's counts")
    );
    assert!(
        refusal(&RAW[..ARRAY_AT + 7])
            .1
            .contains("ends inside the items' array record")
    );
}

/// A slot that isn't a word, and one in use past `_size`, refused at its byte.
#[test]
fn slots_are_held_to_the_mapping() {
    for raw in [1_000_000_000, -5, rtx::slot(2) + 1] {
        let (offset, message) = refusal(&with_int(SLOTS_AT + 4 * 7, raw));
        assert_eq!(offset, SLOTS_AT + 4 * 7);
        assert!(
            message.contains(&format!("slot 7, {raw}, isn't a 16-bit word")),
            "{message}"
        );
    }
    let (offset, message) = refusal(&with_int(SLOTS_AT + 4 * 480, 1));
    assert_eq!(offset, SLOTS_AT + 4 * 480);
    assert!(message.contains("slot 480, past the list's `_size` of 480, isn't 0"));
    // A zero slot is the word 0, a valid one.
    assert!(rtx::read_raw(&with_int(SLOTS_AT + 4 * 9, 0)).is_ok());
    // The hemispheres' word: `S` in its low byte turns the first fix's latitude south, `E` in its
    // high byte its longitude east.
    let log = rtx::read_raw(&with_word(11, u16::from_le_bytes(*b"SE"))).unwrap();
    assert!((log.channels[0].values[0] + 32.983_539).abs() < 1e-12);
    assert!((log.channels[1].values[0] - 106.975_720_166_666_7).abs() < 1e-12);
    assert!(log.channels[0].values[1] > 0.0);
}

/// No record, or none whose time is a time of day: no data.
#[test]
fn a_file_with_no_time_has_no_data() {
    assert_eq!(
        rtx::read_raw(&build(&[])),
        Err(LogError::NoData { format: "RTx .rtx" })
    );
    assert_eq!(
        rtx::read_raw(&build(&[fix(246_000), fix(126_000)])),
        Err(LogError::NoData { format: "RTx .rtx" })
    );
    assert!(matches!(
        rtx::read_raw(b"\x00\x01\x00\x00\x00 not a list"),
        Err(LogError::NotThisFormat { .. })
    ));
}

/// A time more than 12 hours behind the one before is the next day's: past midnight UTC the
/// clock runs on.
#[test]
fn the_clock_runs_past_midnight() {
    let log = rtx::read_raw(&build(&[
        fix(235_958),
        fix(235_959),
        fix(0),
        fix(1),
        fix(2),
    ]))
    .unwrap();
    assert_eq!(log.channels[0].time_s, [0.0, 1.0, 2.0, 3.0, 4.0]);
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("each read as the next day's"))
    );
    // A step back of 12 hours or less stays a step back.
    let log = rtx::read_raw(&build(&[fix(180_000), fix(60_000)])).unwrap();
    assert_eq!(log.channels[0].time_s, [0.0, -43_200.0]);
}

/// A coordinate out of range is a gap, and so are both where both read 0, each counted.
#[test]
fn positions_out_of_range_are_gaps() {
    let mut zero = fix(120_000);
    zero[2..8].copy_from_slice(&[0, 34_464, 1, 0, 34_464, 1]);
    let mut minutes = fix(120_001);
    minutes[2] = 3260;
    let mut hemisphere = fix(120_002);
    hemisphere[11] = u16::from_le_bytes(*b"NX");
    let mut lead = fix(120_003);
    lead[3..5].copy_from_slice(&[0, 0]);
    let log = rtx::read_raw(&build(&[zero, minutes, hemisphere, lead, fix(120_004)])).unwrap();
    let lat = &log.channels[0].values;
    let lon = &log.channels[1].values;
    assert!(lat[0].is_nan() && lon[0].is_nan());
    assert!(lat[1].is_nan() && !lon[1].is_nan());
    assert!(!lat[2].is_nan() && lon[2].is_nan());
    assert!(lat[3].is_nan() && !lon[3].is_nan());
    assert!(!lat[4].is_nan() && !lon[4].is_nan());
    // A southern and eastern fix reads negative and positive.
    let mut south_east = fix(120_005);
    south_east[11] = u16::from_le_bytes(*b"SE");
    let log = rtx::read_raw(&build(&[south_east])).unwrap();
    assert!(log.channels[0].values[0] < 0.0 && log.channels[1].values[0] > 0.0);
    let log = rtx::read_raw(&build(&[zero, minutes, hemisphere, lead])).unwrap();
    let notes = log.notes.join("\n");
    for expected in [
        "2 latitudes are out of range",
        "1 longitudes are out of range",
        "1 records have the latitude and longitude both 0",
    ] {
        assert!(notes.contains(expected), "{expected}\n{notes}");
    }
}

/// An export's row of other than ten cells, or a cell that isn't its field's, refused by its
/// line; a time that isn't a time of day a gap.
#[test]
fn export_faults_are_refused_by_line() {
    let header = "UTC Time,Latitude,Longitude,Altitude,Velocity,VDOP,NavStatus,Sats,SOG,COG\n";
    let good = "120000,3259.01234N,10658.54321W,4593,0,2,122,9,12,40\n";
    assert!(rtx::read_export(&format!("{header}{good}")).is_ok());
    for (row, message) in [
        (
            "120000,3259.01234N,10658.54321W,4593,0,2,122,9,12\n",
            "9 cells, not 10",
        ),
        (
            "120000,3259.01234N,10658.54321W,4593,0,2,122,9,12,40,1\n",
            "11 cells, not 10",
        ),
        (
            "12a000,3259.01234N,10658.54321W,4593,0,2,122,9,12,40\n",
            "`UTC Time` 12a000",
        ),
        (
            "120000,3259.0123N,10658.54321W,4593,0,2,122,9,12,40\n",
            "`Latitude` 3259.0123N",
        ),
        (
            "120000,3259.01234W,10658.54321W,4593,0,2,122,9,12,40\n",
            "`Latitude` 3259.01234W",
        ),
        (
            "120000,3259.01234N,x°,4593,0,2,122,9,12,40\n",
            "`Longitude` x°",
        ),
        (
            "120000,3259.01234N,10658.54321W,high,0,2,122,9,12,40\n",
            "`Altitude` high",
        ),
    ] {
        match rtx::read_export(&format!("{header}{good}{row}")) {
            Err(LogError::Syntax {
                format: "RTx CSV",
                line: 3,
                message: got,
            }) => assert!(got.contains(message), "{got}"),
            other => panic!("{row}: {other:?}"),
        }
    }
    let log = rtx::read_export(&format!(
        "{header}{good}99999999,3259.01234N,10658.54321W,4593,0,2,122,9,12,40\n\
         120002, 95.000000°,-106.975720°,4593,0,2,122,9,12,40\n"
    ))
    .unwrap();
    assert!(log.channels[0].time_s[1].is_nan());
    assert!(log.channels[0].values[2].is_nan());
    assert_eq!(
        rtx::read_export(header),
        Err(LogError::NoData { format: "RTx CSV" })
    );
    assert!(matches!(
        rtx::read_export("UTC,Lat,Lon\n"),
        Err(LogError::NotThisFormat { .. })
    ));
}

proptest::proptest! {
    /// Any bytes changed in the `.rtx`, and any cut, are read or refused, never a panic.
    #[test]
    fn any_rtx_is_read_or_refused(
        changes in proptest::collection::vec((0usize..2765, proptest::prelude::any::<u8>()), 0..8),
        cut in 0usize..2800,
    ) {
        let mut bytes = RAW.to_vec();
        for (at, byte) in changes {
            bytes[at] = byte;
        }
        bytes.truncate(cut);
        let _ = rtx::read_raw(&bytes);
        let _ = import::read(&bytes);
    }

    /// Any words in the records are read or refused, never a panic, and a read keeps every
    /// record.
    #[test]
    fn any_words_are_read(words in proptest::collection::vec(proptest::prelude::any::<u16>(), 16..160)) {
        let records: Vec<[u16; 16]> = words.as_chunks::<16>().0.to_vec();
        if let Ok(log) = rtx::read_raw(&build(&records)) {
            proptest::prop_assert_eq!(log.channels[0].values.len(), records.len());
        }
    }

    /// Any cells after the header are read or refused, never a panic, and a read keeps its rows.
    #[test]
    fn any_export_is_read_or_refused(
        rows in proptest::collection::vec(
            ("[0-9]{0,11}", "[ -]?[0-9.]{0,12}[NSEW°]?", "[ -]?[0-9.]{0,12}[NSEW°]?", "[-0-9.e]{0,6}"),
            0..20,
        ),
    ) {
        let mut text = "UTC\tLat\t\tLon\t\tAlt\tVel\tVDOP\tNav\tSats\tSOG\tCOG\n".to_owned();
        for (time, lat, lon, alt) in &rows {
            text += &format!("{time}\t{lat}\t{lon}\t{alt}\t0\t2\t122\t9\t0\t0\n");
        }
        if let Ok(log) = rtx::read_export(&text) {
            proptest::prop_assert_eq!(log.channels[0].values.len(), rows.len());
        }
    }
}
