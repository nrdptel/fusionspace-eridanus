//! Missile Works' RRC3 raw `.rff` files, read through the importer from invented streams that a
//! small writer below lays out (M7.1c5): MS-NRBF's serialization header, the flight record's class
//! name, then the list's member reference and its array of 16-bit words. The values each test
//! expects are worked by hand from the words it writes.

use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

/// A stream holding `words` in a list of `size` slots (the rest zero, spare capacity) whose array
/// has `capacity` slots, under the object id `id`.
fn stream(words: &[u16], capacity: usize, id: i32) -> Vec<u8> {
    let mut bytes = vec![0u8];
    // The serialization header: the root's id, the header's id, major version 1, minor 0.
    for value in [1i32, -1, 1, 0] {
        bytes.extend(value.to_le_bytes());
    }
    bytes.extend(b"\x0c\x02\x00\x00\x00mDACS.AnalysisSupport+FlightRecord");
    list(&mut bytes, words, capacity, id);
    bytes.push(0x0b);
    bytes
}

/// A list's member reference, size and version, then its array record.
fn list(bytes: &mut Vec<u8>, words: &[u16], capacity: usize, id: i32) {
    bytes.push(0x09);
    bytes.extend(id.to_le_bytes());
    bytes.extend((words.len() as i32).to_le_bytes());
    bytes.extend((words.len() as i32).to_le_bytes());
    bytes.push(0x0f);
    bytes.extend(id.to_le_bytes());
    bytes.extend((capacity as i32).to_le_bytes());
    bytes.push(7);
    for slot in 0..capacity {
        bytes.extend(words.get(slot).copied().unwrap_or(0).to_le_bytes());
    }
}

/// A log of `seconds` seconds: a preamble (a pair, then one word below the pad's floor), then 20
/// readings a second from 1,000.0 mbar falling by 0.1 mbar each, a pair after each 20th, one single
/// word after the 30th reading, and a last `0xFFFF`.
fn words(seconds: usize) -> Vec<u16> {
    let mut words = vec![0x6f00, 0x7f00, 0x0400];
    for n in 0..seconds * 20 {
        words.push(10_000 - n as u16);
        if n == 29 {
            words.push(0x7918);
        }
        if n % 20 == 19 {
            words.extend([0x6e00 + n as u16, 0x7e00]);
        }
    }
    words.push(0xffff);
    words
}

fn read(bytes: &[u8]) -> FlightLog {
    assert_eq!(import::detect(bytes), Some(LogFormat::Rrc3Rff));
    import::read(bytes).unwrap()
}

#[test]
fn a_log_reads_its_pressures_on_the_20_hz_clock_and_keeps_every_other_word() {
    let log = read(&stream(&words(3), 256, 2));
    assert_eq!(log.format, LogFormat::Rrc3Rff);
    assert_eq!(log.logger, "Missile Works RRC3");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());

    let pressure = &log.channels[0];
    assert_eq!(
        (pressure.column.as_str(), pressure.quantity),
        ("Pressure", Quantity::Pressure)
    );
    assert_eq!(pressure.values.len(), 60);
    // 10,000 tenths of a millibar, 100,000 Pa; the 60th reading 9,941, 99,410 Pa, at 59/20 s.
    assert_eq!(pressure.values[0], 100_000.0);
    assert_eq!(pressure.values[59], 99_410.0);
    assert_eq!(pressure.time_s[59], 59.0 / 20.0);

    // A pair after each 20th reading, at that reading's time.
    let (first, second) = (&log.channels[1], &log.channels[2]);
    assert_eq!(first.column, "auxiliary word 1");
    assert_eq!(first.quantity, Quantity::Unrecognized);
    assert_eq!(first.time_s, [19.0 / 20.0, 39.0 / 20.0, 59.0 / 20.0]);
    assert_eq!(
        first.values,
        [
            f64::from(0x6e00 + 19),
            f64::from(0x6e00 + 39),
            f64::from(0x6e00 + 59)
        ]
    );
    assert_eq!(second.values, [f64::from(0x7e00u16); 3]);

    // The single words, the one after the 30th reading and the last `0xFFFF`.
    let single = &log.channels[3];
    assert_eq!(single.column, "single auxiliary word");
    assert_eq!(single.time_s, [29.0 / 20.0, 59.0 / 20.0]);
    assert_eq!(single.values, [f64::from(0x7918u16), 65_535.0]);
    insta::assert_json_snapshot!(log.notes);
}

/// The byte a refusal names, and its message.
fn refused(bytes: &[u8]) -> (usize, String) {
    match import::read(bytes) {
        Err(LogError::Binary {
            format: "RRC3 .rff",
            offset,
            message,
        }) => (offset, message),
        other => panic!("expected a refusal at a byte, got {other:?}"),
    }
}

#[test]
fn a_stream_without_one_list_is_refused() {
    // No list at all: the class name alone.
    let mut bytes = stream(&words(3), 256, 2);
    let at = bytes.iter().position(|b| *b == 0x09).unwrap();
    bytes.truncate(at);
    let (offset, message) = refused(&bytes);
    assert_eq!(offset, 0);
    assert!(message.contains("no list"), "{message}");

    // Two lists that could be the flight data's.
    let mut two = stream(&words(3), 256, 2);
    list(&mut two, &words(3), 256, 3);
    let (_, message) = refused(&two);
    assert!(message.contains("two lists"), "{message}");

    // A list whose slots run past the file's end.
    let mut short = stream(&words(3), 256, 2);
    short.truncate(short.len() - 100);
    let (_, message) = refused(&short);
    assert!(message.contains("runs past the file's end"), "{message}");
}

#[test]
fn readings_must_start_on_the_pad_within_the_preamble() {
    // Eight words below 500 mbar before the first reading.
    let mut low = vec![0x0400u16; 8];
    low.extend(words(3));
    let (_, message) = refused(&stream(&low, 256, 2));
    assert!(message.contains("first 8 words"), "{message}");
}

#[test]
fn a_word_after_the_zero_that_ends_the_log_is_refused() {
    let mut cut = words(3);
    cut.extend([0, 0x0500]);
    let (_, message) = refused(&stream(&cut, 256, 2));
    assert!(message.contains("holds 0x0500"), "{message}");
}

#[test]
fn pairs_and_readings_must_agree_on_how_long_the_log_ran() {
    // 200 readings, 10 s, with no pairs at all.
    let readings: Vec<u16> = (0..200).map(|n| 10_000 - n).collect();
    let (_, message) = refused(&stream(&readings, 256, 2));
    assert!(message.contains("don't agree"), "{message}");
}

#[test]
fn under_two_seconds_of_readings_has_no_data() {
    assert_eq!(
        import::read(&stream(&words(1), 64, 2)),
        Err(LogError::NoData {
            format: "RRC3 .rff"
        })
    );
}
