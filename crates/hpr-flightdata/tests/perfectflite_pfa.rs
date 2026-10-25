//! PerfectFlite's older `.pfa`, as DataCap saves an Alt15K/WD's flight, read through the importer
//! from invented files and pinned whole by snapshots (M7.1c2). The files hold made-up flights in
//! the layout the sample logs share: one with its lines ending in CR, as most are, whose preamble
//! agrees with its samples; one ending them in CR LF, whose `NumSamps:` states more samples than
//! it holds and which carries a comment. Then files built in the test: one at a sample count
//! DataCap's help names for a logger that lost power, and malformed ones, each refused at its line.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// CR-only lines: 24 samples 0.1 s apart from 0, feet above the pad; the stated apogee is their
/// highest and the stated count theirs, so the notes are only the unit decision, the clock's zero
/// and what `Ground:` is.
#[test]
fn the_cr_file_reads_a_sample_every_tenth_of_a_second() {
    let bytes = include_bytes!("samples/perfectflite-alt15k.pfa");
    assert!(
        !bytes.contains(&b'\n'),
        "the sample should end its lines in CR only"
    );
    assert_eq!(import::detect(bytes), Some(LogFormat::PerfectFlitePfa));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFlitePfa);
    assert_eq!(log.logger, "PerfectFlite Alt15K/WD");
    assert_eq!(log.time_s.len(), 24);
    assert_eq!(&log.time_s[..4], [0.0, 0.1, 0.2, 0.3]);
    assert_eq!(log.time_s[23], 2.3);
    // The eighth sample, at 0.7 s, is the first at 80 ft: the launch detection.
    assert_eq!(log.altitude_m[7], 88.0 * FOOT_M);
    assert!(log.altitude_m[6] < 80.0 * FOOT_M);
    assert_eq!(log.stated.apogee_m, Some(274.0 * FOOT_M));
    assert_eq!(
        log.altitude_m.iter().copied().fold(f64::MIN, f64::max),
        274.0 * FOOT_M
    );
    assert_eq!(log.stated.samples, Some(24));
    assert_eq!(log.stated.ground_elevation_msl_m, None);
    let quantities: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    assert_eq!(quantities, [("Data", Quantity::AltitudeAboveGround)]);
    assert_eq!(log.notes.len(), 3, "{:?}", log.notes);
    note(&log, "`Data` read in feet, 0.1 s apart, from the manual. ");
    note(
        &log,
        "time zero is 0.7 s before the logger detected the launch, not liftoff",
    );
    note(
        &log,
        "the file's `Ground:`, 1150 ft, is the pad's pressure altitude",
    );
    insta::assert_json_snapshot!(log);
}

/// CR LF lines read alike; a stated count the samples don't match and the comment are noted.
#[test]
fn the_cr_lf_file_notes_its_count_and_comment() {
    let bytes = include_bytes!("samples/perfectflite-alt15k-crlf.pfa");
    assert!(bytes.windows(2).any(|pair| pair == b"\r\n"));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::PerfectFlitePfa);
    assert_eq!(log.time_s.len(), 16);
    assert_eq!(log.time_s[15], 1.5);
    assert_eq!(log.stated.samples, Some(24));
    assert_eq!(
        note(&log, "the file states 24 samples"),
        "the file states 24 samples and holds 16"
    );
    assert_eq!(
        note(&log, "the file's comment"),
        "the file's comment: \"Invented test flight\""
    );
    insta::assert_json_snapshot!(log);
}

/// A file of 2,200 samples, the count DataCap's help gives for a revision 2 logger that lost power
/// in flight, reads, with a note saying the samples after this flight's may be older flights'.
#[test]
fn a_full_memory_count_reads_with_a_warning() {
    let mut text = String::from(
        "PerfectFlite Alt15K/WD\rGround: 500\rApogee: 90\rNumSamps: 2200\rComments: \r\r**!!Data: \r",
    );
    for index in 0..2_200 {
        text.push_str(&format!("{:05}\r", (index % 91)));
    }
    let log = import::read(text.as_bytes()).unwrap();
    assert_eq!(log.time_s.len(), 2_200);
    let warning = note(
        &log,
        "the file holds 2200 samples. DataCap's help gives that count",
    );
    assert!(warning.contains("may be older flights'"), "{warning}");
}

/// A malformed sample line is refused, naming its line.
#[test]
fn a_malformed_sample_is_refused_at_its_line() {
    let text = "PerfectFlite Alt15K/WD\r\nGround: 500\r\nApogee: 12\r\nNumSamps: 3\r\nComments: \r\n\
                \r\n**!!Data: \r\n00000\r\n000x2\r\n00012\r\n";
    let error = import::read(text.as_bytes()).unwrap_err().to_string();
    assert!(
        error.starts_with(".pfa line 9: \"000x2\" isn't a sample"),
        "{error}"
    );
}
