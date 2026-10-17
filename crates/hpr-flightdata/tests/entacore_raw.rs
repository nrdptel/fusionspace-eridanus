//! Entacore AIM XTRA's own files, the raw dump of its memory and the `.xtra` its software makes
//! of it, read from invented bytes (M7.1c23): a made-up flight of 13 records, written once as the
//! dump's big-endian raw values and once as the `.xtra`'s converted ones, so that the two readers
//! must agree value for value, as the sample's files do. The values each test expects are worked
//! by hand from the bytes.

use hpr_flightdata::altimax_als;
use hpr_flightdata::entacore_raw;
use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// A dump's recording opens with type `0x13` and 101 bytes the reader doesn't read.
const OPENING: usize = 1 + 101;

/// Where the `.xtra`'s first record starts.
const XTRA_RECORDS_AT: usize = 54;

/// A record of the invented flight: the ticks before it, its raw type and big-endian payload,
/// and its `.xtra` class and little-endian payload, as the software converts it.
type Record = (u32, u8, Vec<u8>, i16, Vec<u8>);

/// The invented flight.
#[cfg(test)]
fn flight() -> Vec<Record> {
    let f32s = |values: &[f32]| {
        values
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>()
    };
    // The status: its state number, then 4 output lines' on and continuity flags, each set of four
    // bytes after its length, 4, and a third set of zeros.
    let status = |state: u32, on: [u8; 4], cont: [u8; 4]| {
        let mut bytes = state.to_le_bytes().to_vec();
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend(on);
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend(cont);
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend([0; 4]);
        bytes
    };
    let degrees = f64::from(1e-7_f32);
    let mut gps = (297_000_000.0 * degrees).to_le_bytes().to_vec();
    gps.extend((-950_000_000.0 * degrees).to_le_bytes());
    gps.extend((150_000.0_f32 * 0.001).to_le_bytes());
    gps.extend(9_u32.to_le_bytes());
    gps.push(1);
    let mut gps_time = 123_456_000_u32.to_le_bytes().to_vec();
    gps_time.extend(2_400_u16.to_le_bytes());
    gps_time.extend([1, 0]);
    gps_time.extend(18_u32.to_le_bytes());
    vec![
        // The pad: 97,700 Pa, 1 g, a battery count of 2,048, 21.50 °C, the state 0, every line
        // continuous.
        (10, 0x03, vec![0x01, 0x7D, 0xA4], 4, f32s(&[97_700.0])),
        (5, 0x02, vec![0x01, 0x00], 2, f32s(&[1.0])),
        (
            5,
            0x04,
            vec![0x80, 0x00],
            5,
            f32s(&[(2_048.0 * 13.299_999_237_060_547_f64 / 4_096.0) as f32]),
        ),
        (0, 0x06, vec![0x08, 0x66], 7, f32s(&[2_150.0_f32 * 0.01])),
        (10, 0x10, vec![0x00, 0xF0], 13, status(0, [0; 4], [1; 4])),
        // The boost, 7 g, its 3 by 2 g sideways, and launch detected.
        (300, 0x02, vec![0x07, 0x00], 2, f32s(&[7.0])),
        (0, 0x0b, vec![0x03, 0x00, 0xFE, 0x00], 3, f32s(&[3.0, -2.0])),
        (0, 0x10, vec![0x10, 0xF0], 13, status(1, [0; 4], [1; 4])),
        // Apogee: output C fires.
        (
            130,
            0x07,
            vec![0x10, 0x00],
            17,
            [
                vec![0, 0],
                f32s(&[4_096.0_f32 * (3.3_f64 / 16_384.0) as f32]),
            ]
            .concat(),
        ),
        (
            0,
            0x10,
            vec![0x54, 0xF0],
            13,
            status(3, [0, 0, 1, 0], [1; 4]),
        ),
        // A fix: 29.7° N, 95° W, 150 m, 9 satellites; the GPS's time.
        (
            1,
            0x0e,
            [
                &297_000_000_i32.to_be_bytes()[..],
                &(-950_000_000_i32).to_be_bytes(),
                &150_000_i32.to_be_bytes(),
                &[0x29],
            ]
            .concat(),
            10,
            gps,
        ),
        (
            1,
            0x12,
            [
                &123_456_000_u32.to_be_bytes()[..],
                &2_400_u16.to_be_bytes(),
                &[0x92],
            ]
            .concat(),
            14,
            gps_time,
        ),
        (1, 0x03, vec![0x01, 0x7C, 0xDC], 4, f32s(&[97_500.0])),
    ]
}

/// The tick bytes for `n` ticks, 126 at most a byte: `0xFF` is erased memory.
#[cfg(test)]
fn ticks(mut n: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    while n > 0 {
        let k = n.min(126);
        bytes.push(0x80 | k as u8);
        n -= k;
    }
    bytes
}

/// The dump of one recording of the flight, then erased memory.
#[cfg(test)]
fn dump() -> Vec<u8> {
    let mut bytes = vec![0x13];
    bytes.extend([0; 101]);
    for (n, raw_type, payload, ..) in flight() {
        bytes.extend(ticks(n));
        bytes.push(raw_type);
        bytes.extend(payload);
    }
    bytes.extend([0xFF; 16]);
    bytes
}

/// Each class's version, as the sample's archive states it.
#[cfg(test)]
fn version(class: i16) -> u32 {
    match class {
        2 | 3 => 3,
        10 => 4,
        14 => 2,
        4..=7 | 13 => 1,
        _ => 0,
    }
}

/// The `.xtra` the software makes of the flight, stating `count` records, and the times it
/// writes: the ticks summed in 32-bit floats, a tick byte at a time.
#[cfg(test)]
fn xtra(count: u64) -> (Vec<u8>, Vec<f32>) {
    let mut bytes = b"\x16\0\0\0serialization::archive\x10\0\x04\x04\x04\x08\x01\0\0\0".to_vec();
    bytes.extend([0, 3, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend(count.to_le_bytes());
    let mut seen = Vec::new();
    let mut time = 0.0_f32;
    let mut times = Vec::new();
    for (index, (n, _, _, class, payload)) in flight().into_iter().enumerate() {
        for byte in ticks(n) {
            time += f32::from(byte & 0x7F) * 0.001;
        }
        times.push(time);
        bytes.extend(class.to_le_bytes());
        if !seen.contains(&class) {
            bytes.push(1);
            bytes.extend(version(class).to_le_bytes());
            seen.push(class);
        }
        bytes.extend((2 * index as u32).to_le_bytes());
        if index == 0 {
            bytes.extend([1, 1, 0, 0, 0]);
        }
        bytes.extend((2 * index as u32 + 1).to_le_bytes());
        bytes.extend(3.3_f32.to_le_bytes());
        bytes.push(0);
        bytes.extend(time.to_le_bytes());
        bytes.extend(payload);
    }
    (bytes, times)
}

/// The channel of a column.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// The events, as names and times.
#[cfg(test)]
fn events(log: &FlightLog) -> Vec<(&str, f64)> {
    log.events
        .iter()
        .map(|e| (e.name.as_str(), e.time_s))
        .collect()
}

/// A dump's refusal, its offset and message.
#[cfg(test)]
fn dump_refusal(bytes: &[u8]) -> (usize, String) {
    match entacore_raw::read_dump(bytes) {
        Err(LogError::Binary {
            format: "Entacore AIM raw dump",
            offset,
            message,
        }) => (offset, message),
        other => panic!("{other:?}"),
    }
}

/// An `.xtra`'s refusal, its offset and message.
#[cfg(test)]
fn xtra_refusal(bytes: &[u8]) -> (usize, String) {
    match entacore_raw::read_xtra(bytes) {
        Err(LogError::Binary {
            format: "Entacore AIM .xtra",
            offset,
            message,
        }) => (offset, message),
        other => panic!("{other:?}"),
    }
}

/// Each raw value in its unit, on the ticks read as milliseconds; the status's changes and an
/// output's firing as events; no primary track.
#[test]
fn a_dump_reads_in_units() {
    let bytes = dump();
    assert!(entacore_raw::recognizes_dump(&bytes));
    assert_eq!(import::detect(&bytes), Some(LogFormat::EntacoreDump));
    let log = import::read(&bytes).unwrap();
    assert_eq!(log.format, LogFormat::EntacoreDump);
    assert_eq!(log.logger, "Entacore AIM XTRA");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert_eq!(log.stated.samples, Some(13));

    let pressure = channel(&log, "pressure");
    assert_eq!(pressure.quantity, Quantity::Pressure);
    assert_eq!(pressure.time_s, [0.01, 0.463]);
    assert_eq!(pressure.values, [97_700.0, 97_500.0]);
    // 256 counts a g, in m/s² by standard gravity.
    let accel = channel(&log, "acceleration");
    assert_eq!(accel.time_s, [0.015, 0.33]);
    assert_eq!(accel.values, [9.806_65, 7.0 * 9.806_65]);
    assert_eq!(channel(&log, "lat. X accel.").values, [3.0 * 9.806_65]);
    assert_eq!(channel(&log, "lat. Y accel.").values, [-2.0 * 9.806_65]);
    // 2,048 of 4,096 steps of 3.3 V × 133/33, the factor taken in 32-bit floats: one step of an
    // f32 below 6.65 V.
    let battery = channel(&log, "battery 1 voltage");
    assert_eq!(battery.quantity, Quantity::Voltage);
    assert_eq!(battery.values, [6.649_999_618_530_273]);
    assert!(battery.values[0] < f64::from(6.65_f32));
    let temperature = channel(&log, "temperature");
    assert_eq!(temperature.quantity, Quantity::Temperature);
    assert_eq!(temperature.values, [21.5 + 273.15]);
    // 4,096 of 16,384 steps of 3.3 V: a quarter of f32(3.3).
    assert_eq!(
        channel(&log, "A Voltage").values,
        [f64::from(3.3_f32) / 4.0]
    );
    assert_eq!(channel(&log, "state").values, [0.0, 1.0, 3.0]);
    assert_eq!(channel(&log, "C on").values, [0.0, 0.0, 1.0]);
    assert_eq!(channel(&log, "D cont").values, [1.0, 1.0, 1.0]);
    let lat = channel(&log, "lat");
    assert_eq!(lat.quantity, Quantity::Latitude);
    assert!((lat.values[0] - 29.7).abs() < 1e-5);
    assert!((channel(&log, "long").values[0] + 95.0).abs() < 1e-5);
    assert_eq!(channel(&log, "GPS MSL").values, [150.0]);
    assert_eq!(channel(&log, "SVs").values, [9.0]);
    assert_eq!(channel(&log, "GPS flag").values, [1.0]);
    assert_eq!(channel(&log, "GPS Time of Week").values, [123_456_000.0]);
    assert_eq!(channel(&log, "GPS week").values, [2_400.0]);
    assert_eq!(channel(&log, "GPS time flag 1").values, [1.0]);
    assert_eq!(channel(&log, "GPS time flag 2").values, [0.0]);
    assert_eq!(channel(&log, "GPS leap seconds").values, [18.0]);
    assert_eq!(
        events(&log),
        [("state 1", 0.33), ("state 3", 0.46), ("C on", 0.46)]
    );
    assert!(log.notes[0].starts_with("recording 1 of the dump's 1, from byte 0"));
}

/// The `.xtra` of the same flight reads to the same channels, value for value, on the times the
/// software summed; its notes say so.
#[test]
fn the_xtra_and_the_dump_agree_value_for_value() {
    let (bytes, times) = xtra(13);
    assert!(entacore_raw::recognizes_xtra(&bytes));
    assert_eq!(import::detect(&bytes), Some(LogFormat::EntacoreXtra));
    let ours = import::read(&bytes).unwrap();
    assert_eq!(ours.format, LogFormat::EntacoreXtra);
    let theirs = import::read(&dump()).unwrap();
    let columns = |log: &FlightLog| {
        log.channels
            .iter()
            .map(|c| c.column.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(columns(&ours), columns(&theirs));
    for (a, b) in ours.channels.iter().zip(&theirs.channels) {
        assert_eq!(
            (a.quantity, &a.values),
            (b.quantity, &b.values),
            "{}",
            a.column
        );
    }
    // The times: the f32 sums, which differ from the ticks in ms.
    assert_eq!(
        channel(&ours, "pressure").time_s,
        [f64::from(times[0]), f64::from(times[12])]
    );
    // 10 ticks of f32(0.001), rounded to an f32: 0.010 000 000 7, not 10 ms.
    assert_eq!(f64::from(times[0]), 0.010_000_000_707_805_157);
    assert_ne!(f64::from(times[0]), 0.01);
    let names = |log: &FlightLog| {
        log.events
            .iter()
            .map(|e| e.name.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(names(&ours), names(&theirs));
    assert!(
        ours.notes
            .iter()
            .any(|n| n.contains("summed in 32-bit floats"))
    );
}

/// Both records, pinned.
#[test]
fn snapshots() {
    insta::assert_json_snapshot!("dump", entacore_raw::read_dump(&dump()).unwrap());
    insta::assert_json_snapshot!("xtra", entacore_raw::read_xtra(&xtra(13).0).unwrap());
}

/// A dump of two recordings: `read_dump` reads each, and the importer refuses it, naming where
/// the second starts.
#[test]
fn a_dump_of_two_recordings() {
    let one = dump();
    let data = &one[..one.len() - 16];
    let bytes = [data, data, &[0xFF; 4][..]].concat();
    let logs = entacore_raw::read_dump(&bytes).unwrap();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].channels, logs[1].channels);
    assert!(logs[1].notes[0].starts_with(&format!(
        "recording 2 of the dump's 2, from byte {}",
        data.len()
    )));
    match import::read(&bytes) {
        Err(LogError::Binary {
            offset, message, ..
        }) => {
            assert_eq!(offset, data.len());
            assert!(message.contains("`entacore_raw::read_dump` reads each"));
        }
        other => panic!("{other:?}"),
    }
}

/// The dump's refusals, each naming its byte.
#[test]
fn dump_refusals_name_the_byte() {
    let bytes = dump();
    let first = dump_at(0);
    assert_eq!(first, OPENING + 1);
    let edit = |at: usize, byte: u8| {
        let mut bytes = dump();
        bytes[at] = byte;
        bytes
    };
    // The pressure's type, at the first record, made one no sample holds.
    let (offset, message) = dump_refusal(&edit(first, 0x7E));
    assert_eq!(offset, first);
    assert!(message.contains("type 0x7e, which no sample holds"));
    // A battery's low bits; an output line's top bits; the GPS byte's bits 6–7; the status's low
    // bits, and a state number the sample doesn't hold.
    let battery = dump_at(2);
    assert_eq!(bytes[battery], 0x04);
    assert!(
        dump_refusal(&edit(battery + 2, 0x01))
            .1
            .contains("bits 0–3")
    );
    let line = dump_at(8);
    assert_eq!(bytes[line], 0x07);
    assert!(dump_refusal(&edit(line + 1, 0x50)).1.contains("bits 14–15"));
    let fix = dump_at(10);
    assert_eq!(bytes[fix], 0x0e);
    assert!(dump_refusal(&edit(fix + 13, 0x69)).1.contains("bits 6–7"));
    let status = dump_at(4);
    assert_eq!(bytes[status], 0x10);
    assert_eq!(dump_refusal(&edit(status + 2, 0xF1)).0, status);
    assert!(
        dump_refusal(&edit(status + 1, 0x20))
            .1
            .contains("no state the sample holds")
    );
    // Data after erased memory.
    let mut gap = bytes.clone();
    let end = gap.len() - 16;
    gap[end + 3] = 0x02;
    let (offset, message) = dump_refusal(&gap);
    assert_eq!(offset, end + 3);
    assert!(message.contains(&format!("erased memory that starts at byte {end}")));
    // Ending inside a record, or inside a recording's opening.
    assert!(
        dump_refusal(&bytes[..first + 2])
            .1
            .contains("ends inside a record of type 0x03")
    );
    assert!(dump_refusal(&bytes[..50]).1.contains("opening bytes"));
    // Not a dump; a recording with no record; a first pressure not a pad's.
    assert!(matches!(
        entacore_raw::read_dump(&bytes[1..]),
        Err(LogError::NotThisFormat { .. })
    ));
    assert!(matches!(
        entacore_raw::read_dump(&[&bytes[..OPENING], &[0xFF; 8][..]].concat()),
        Err(LogError::NoData { .. })
    ));
    let mut hectopascals = bytes;
    hectopascals[first + 1..first + 4].copy_from_slice(&[0x00, 0x03, 0xD1]);
    assert!(matches!(
        entacore_raw::read_dump(&hectopascals),
        Err(LogError::NotThisFormat { message, .. }) if message.contains("977, isn't a pad's")
    ));
}

/// The `.xtra`'s refusals, each naming its byte, and the notes of a cut or trailed file.
#[test]
fn xtra_refusals_and_notes() {
    let (bytes, _) = xtra(13);
    let edit = |at: usize, byte: u8| {
        let mut bytes = xtra(13).0;
        bytes[at] = byte;
        bytes
    };
    assert!(matches!(
        entacore_raw::read_xtra(&bytes[1..]),
        Err(LogError::NotThisFormat { .. })
    ));
    assert_eq!(xtra_refusal(&edit(37, 4)).0, 36);
    // The first record's class, made one no sample holds; its version; its header's opening byte.
    let (offset, message) = xtra_refusal(&edit(XTRA_RECORDS_AT, 99));
    assert_eq!(offset, XTRA_RECORDS_AT);
    assert!(message.contains("class 99, which no sample holds"));
    let (offset, message) = xtra_refusal(&edit(XTRA_RECORDS_AT + 3, 2));
    assert_eq!(offset, XTRA_RECORDS_AT + 2);
    assert!(message.contains("class 4's version is 2, where the sample's is 1"));
    assert!(
        xtra_refusal(&edit(XTRA_RECORDS_AT + 2, 2))
            .1
            .contains("opens with 2")
    );
    // Its object id; its base's 3.3; the byte after it.
    assert!(
        xtra_refusal(&edit(XTRA_RECORDS_AT + 7, 2))
            .1
            .contains("its object id is 2, not 0")
    );
    let base = XTRA_RECORDS_AT + 7 + 4 + 5 + 4;
    assert_eq!(bytes[base..base + 4], 3.3_f32.to_le_bytes());
    assert!(
        xtra_refusal(&edit(base, 0))
            .1
            .contains("its base isn't the sample's")
    );
    assert!(
        xtra_refusal(&edit(base + 4, 1))
            .1
            .contains("its base isn't the sample's")
    );
    // A time that isn't a number.
    let mut nan = bytes.clone();
    nan[base + 5..base + 9].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(xtra_refusal(&nan).1.contains("its time isn't a number"));
    // The status's held bytes: a set's length, the third set.
    let status = find_class(&bytes, 13);
    assert!(
        xtra_refusal(&edit(status + 4, 5))
            .1
            .contains("first set states 5 bytes")
    );
    assert!(
        xtra_refusal(&edit(status + 27, 1))
            .1
            .contains("third set of four bytes isn't zero")
    );
    // An output line's flag bytes.
    let line = find_class(&bytes, 17);
    assert!(
        xtra_refusal(&edit(line, 1))
            .1
            .contains("two flag bytes aren't zero")
    );

    // Cut inside its third record, or after its second; trailing bytes; no record.
    let third = find_record(&bytes, 2);
    let cut = entacore_raw::read_xtra(&bytes[..third + 5]).unwrap();
    assert_eq!(cut.stated.samples, Some(2));
    assert!(
        cut.notes[0]
            .starts_with("the file ends inside record 3 of the 13 it states, 5 bytes into it")
    );
    let cut = entacore_raw::read_xtra(&bytes[..third]).unwrap();
    assert_eq!(
        cut.notes[0],
        "the file ends after record 2 of the 13 it states: read those"
    );
    let trailed = [&bytes[..], &[0; 114]].concat();
    let log = entacore_raw::read_xtra(&trailed).unwrap();
    assert_eq!(log.notes[0], "114 bytes after the 13 records aren't read");
    assert!(matches!(
        entacore_raw::read_xtra(&xtra(0).0[..XTRA_RECORDS_AT]),
        Err(LogError::NoData { .. })
    ));
}

/// Where record `index`'s type byte is in the invented dump.
#[cfg(test)]
fn dump_at(index: usize) -> usize {
    let mut at = OPENING;
    for (k, (n, _, payload, ..)) in flight().into_iter().enumerate() {
        at += ticks(n).len();
        if k == index {
            return at;
        }
        at += 1 + payload.len();
    }
    panic!("no record {index}")
}

/// Where the payload of the first record of `class` starts in the invented `.xtra`.
#[cfg(test)]
fn find_class(bytes: &[u8], class: i16) -> usize {
    let index = flight().iter().position(|r| r.3 == class).unwrap();
    let start = find_record(bytes, index);
    // Its class, its header the first time, its id, its base.
    start + 2 + 5 + 4 + 13
}

/// Where record `index` of the invented `.xtra` starts.
#[cfg(test)]
fn find_record(bytes: &[u8], index: usize) -> usize {
    let mut at = XTRA_RECORDS_AT;
    let mut seen = Vec::new();
    for (k, (_, _, _, class, payload)) in flight().into_iter().enumerate() {
        if k == index {
            return at;
        }
        at += 2
            + if seen.contains(&class) { 0 } else { 5 }
            + 4
            + if k == 0 { 5 } else { 0 }
            + 13
            + payload.len();
        seen.push(class);
    }
    assert_eq!(at, bytes.len());
    at
}

/// None of the other readers' invented binary samples is taken for an AIM file or an `.ALS`.
#[test]
fn no_other_sample_is_taken_for_these() {
    for bytes in [
        &include_bytes!("samples/altos-eeprom-telemetrum-v1.eeprom")[..],
        include_bytes!("samples/altos-eeprom-telemetrum-v2.eeprom"),
        include_bytes!("samples/altos-telemetry.telem"),
        include_bytes!("samples/arts2-raw.odf"),
        include_bytes!("samples/marsa.dat"),
        include_bytes!("samples/perfectflite-alt15k.pfa"),
        include_bytes!("samples/rtx.rtx"),
        include_bytes!("samples/eggtimer-rtf-two-downloads.rtf"),
    ] {
        assert!(!entacore_raw::recognizes_dump(bytes));
        assert!(!entacore_raw::recognizes_xtra(bytes));
        assert!(!altimax_als::recognizes(bytes));
    }
}
