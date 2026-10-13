//! An Eggtimer TRS's terminal download saved inside an RTF document, read through the importer
//! from an invented file and pinned whole by a snapshot (M7.1c4, ADR-258 §4). The document is
//! written as a Mac text editor writes one: font and color tables, an `\*` destination, the
//! document's information, a paragraph a line ended by `\` at the line's end, an escaped byte
//! in Windows 1252, escaped braces, a `\tab` and a `\u` character with its fallback. It holds two
//! downloads one after another, a made-up flight and a short second one.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

const SAMPLE: &[u8] = include_bytes!("samples/eggtimer-rtf-two-downloads.rtf");

fn events(log: &FlightLog) -> Vec<(f64, &str)> {
    log.events
        .iter()
        .map(|event| (event.time_s, event.name.as_str()))
        .collect()
}

/// The first download is read as the text reader reads one, its notes after the one saying the
/// text was unwrapped; the second is counted, its title's line the document's.
#[test]
fn the_first_download_is_read_and_the_second_counted() {
    assert_eq!(import::detect(SAMPLE), Some(LogFormat::EggtimerRtf));
    let log = import::read(SAMPLE).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerRtf);
    assert_eq!(log.logger, "Eggtimer TRS");
    assert_eq!(log.firmware.as_deref(), Some("2.07"));
    // The first table's 14 rows, in milliseconds, and none of the second's.
    assert_eq!(log.time_s.len(), 14);
    assert_eq!(log.time_s[..3], [0.0, 0.05, 0.1]);
    assert_eq!(log.time_s.last(), Some(&0.65));
    assert_eq!(log.channels.len(), 10);
    assert_eq!(log.channels[2].column, "FAlt");
    assert_eq!(log.channels[2].quantity, Quantity::AltitudeAboveGround);
    let apogee = log.stated.apogee_m.unwrap();
    assert!((apogee - 120.0 * FOOT_M).abs() < 1e-9, "{apogee}");
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert_eq!(peak, apogee);
    let ground = log.stated.ground_elevation_msl_m.unwrap();
    assert!((ground - 800.0 * FOOT_M).abs() < 1e-9, "{ground}");
    assert_eq!(
        events(&log),
        [
            (0.1, "LDA"),
            (0.25, "LowV"),
            (0.4, "Apogee"),
            (0.45, "N-O"),
            (0.45, "Drogue")
        ]
    );
    assert!(log.notes[0].starts_with("the text was unwrapped from an RTF document"));
    let has = |text: &str| log.notes.iter().any(|note| note.contains(text));
    // The escaped braces and the `\tab` are the setting's text; the escaped byte, an ellipsis in
    // Windows 1252, makes the comments line a name line, left out of the record.
    assert!(has("`Bench mode {test}` 2"), "{:#?}", log.notes);
    assert!(has("1 line of the flight settings holds no setting"));
    // `\u176` is a degree sign, its fallback `?` skipped.
    assert!(has("`Base Temp: 61.0 \u{b0}F`"), "{:#?}", log.notes);
    // A summary time past the table's last sample isn't in seconds: no event.
    assert!(has(
        "the flight summary's times past the table's last, 0.65 s, kept as written and not read \
         as events: no event of the flight comes after its last sample, so they aren't in \
         seconds: `Unlock` 9500"
    ));
    assert!(has(
        "the document holds 2 Eggtimer downloads one after another; the first was read, as a \
         record holds one flight, and the titles of those that weren't are on lines 47"
    ));
    let json = serde_json::to_string(&log).unwrap();
    assert!(!json.contains('\u{fffd}'));
    assert!(!json.contains("Nobody"));
    assert!(!json.contains("bench\""));
    insta::assert_json_snapshot!(log);
}

/// A bad cell in the table is refused naming the RTF document's line.
#[test]
fn a_bad_cell_names_the_documents_line() {
    let text = String::from_utf8_lossy(SAMPLE).replacen("\n250,60,", "\n250,x,", 1);
    let error = import::read(text.as_bytes()).unwrap_err().to_string();
    assert_eq!(
        error,
        "Eggtimer RTF line 35: \"x\" in cell 2, under \"Alt\", isn't a number"
    );
}

/// A value its conversion makes infinite is refused on the document's line (ADR-258 §3).
#[test]
fn a_value_its_conversion_makes_infinite_is_refused_on_the_documents_line() {
    let text = String::from_utf8_lossy(SAMPLE)
        .replacen(
            "T,Alt,Veloc,FAlt,FVeloc,LDA",
            "T,Alt,Veloc,FAlt,AccelG,LDA",
            1,
        )
        .replacen(
            "\n250,60,400.00,46.00,340.00,",
            "\n250,60,400.00,46.00,1.7e308,",
            1,
        );
    let error = import::read(text.as_bytes()).unwrap_err().to_string();
    assert_eq!(
        error,
        "Eggtimer RTF line 35: the column AccelG holds a value too large for its unit's \
         conversion to SI"
    );
}

/// An RTF document that doesn't hold an Eggtimer download is refused by name, and one whose
/// braces don't balance at its line.
#[test]
fn other_documents_are_refused_by_name() {
    let notes = b"{\\rtf1\\ansi{\\fonttbl\\f0 Helvetica;}\\f0 Packing list\\par 1,2,3\\par}";
    assert_eq!(import::detect(notes), Some(LogFormat::EggtimerRtf));
    let error = import::read(notes).unwrap_err().to_string();
    assert!(
        error.starts_with(
            "not read as Eggtimer RTF: the RTF document's text holds no Eggtimer terminal \
             download"
        ),
        "{error}"
    );
    let open = String::from_utf8_lossy(SAMPLE).replace("\n}\n", "\n");
    let error = import::read(open.as_bytes()).unwrap_err().to_string();
    assert_eq!(
        error,
        "Eggtimer RTF line 54: the document ends with 1 groups open: its braces don't balance"
    );
}
