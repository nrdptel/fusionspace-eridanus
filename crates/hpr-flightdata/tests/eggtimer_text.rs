//! An Eggtimer TRS's terminal download, read through the importer from invented files and pinned
//! whole by snapshots (M7.1c2). The files hold made-up flights in the two firmware spellings: 2.07's
//! with a `Flight Summary:` heading and CR LF line ends, and 2.10's with CR line ends alone, a
//! settings line of bytes that aren't UTF-8, and its main channel off.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

const FOOT_M: f64 = 0.3048;

fn events(log: &FlightLog) -> Vec<(f64, &str)> {
    log.events
        .iter()
        .map(|event| (event.time_s, event.name.as_str()))
        .collect()
}

/// Firmware 2.07's download: the settings and the summary, then the table in milliseconds. The
/// summary's apogee and site elevation are stated, its times are its markers' events but for the
/// top speed's, an event of its own.
#[test]
fn the_2_07_download_reads_its_summary_and_table() {
    let bytes = include_bytes!("samples/eggtimer-text-trs-2-07.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::EggtimerText));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerText);
    assert_eq!(log.logger, "Eggtimer TRS");
    assert_eq!(log.firmware.as_deref(), Some("2.07"));
    assert_eq!(log.time_s.len(), 52);
    assert_eq!(log.time_s[..3], [0.0, 0.05, 0.1]);
    assert_eq!(log.time_s.last(), Some(&12.0));
    assert_eq!(log.channels.len(), 10);
    assert_eq!(log.channels[2].column, "Filt_Alt");
    assert_eq!(log.channels[2].quantity, Quantity::AltitudeAboveGround);
    // The stated apogee, 480 ft, is the table's highest `Alt`.
    let apogee = log.stated.apogee_m.unwrap();
    assert!((apogee - 480.0 * FOOT_M).abs() < 1e-9, "{apogee}");
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert_eq!(peak, apogee);
    let ground = log.stated.ground_elevation_msl_m.unwrap();
    assert!((ground - 1150.0 * FOOT_M).abs() < 1e-9, "{ground}");
    assert_eq!(
        events(&log),
        [
            (0.9, "LDA"),
            (1.0, "Max Velocity"),
            (3.5, "LowV"),
            (6.0, "Apogee"),
            (6.5, "N-O"),
            (6.5, "Drogue"),
            (10.5, "Main"),
        ]
    );
    // The clock and both altitudes are decided by two checks: the coast up to the row before the
    // `Apogee` marker, 3 s of it at 2 rows a second, slows at 1 g in feet.
    assert!(
        log.notes[0].starts_with("`T` read in milliseconds, from the step and the coast: "),
        "{}",
        log.notes[0]
    );
    assert!(log.notes[0].contains(
        "over the rows from 2.5 s to 5.5 s, up to the row before the marker `Apogee` at 6 s"
    ));
    assert!(log.notes[1].starts_with("`Alt` read in feet, from the manual and the coast: "));
    assert!(log.notes[3].starts_with("`Filt_Alt` read in feet, from the manual and the coast: "));
    assert!(log.notes.iter().any(|note| note
        == "the flight summary's times are its table markers' to the millisecond, one event \
            each, the marker's: `Apogee` at `Apogee`, `LDA` at `LDA`, `Unlock` at `LowV`, \
            `N-O` at `N-O`, `Drogue Deploy` at `Drogue`, `Main Deploy` at `Main`"));
    // The owner's name for the flight computer is left out of the record.
    assert!(!serde_json::to_string(&log).unwrap().contains("MY EGGTIMER"));
    insta::assert_json_snapshot!(log);
}

/// Firmware 2.10's download with CR line ends, the board's name line in bytes that aren't UTF-8,
/// and the main channel off: its summary time of 0 is no event.
#[test]
fn the_2_10_download_with_cr_line_ends_reads_alike() {
    let bytes = include_bytes!("samples/eggtimer-text-cr-only.txt");
    assert!(!bytes.contains(&b'\n'));
    // 0xB1 alone, a Latin-1 `±`, isn't UTF-8.
    assert!(bytes.contains(&0xb1));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::EggtimerText);
    assert_eq!(log.firmware.as_deref(), Some("2.10"));
    assert_eq!(log.time_s.len(), 52);
    let apogee = log.stated.apogee_m.unwrap();
    let peak = log.altitude_m.iter().copied().fold(f64::MIN, f64::max);
    assert_eq!(peak, apogee);
    let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(
        names,
        ["LDA", "Max Veloc", "LowV", "Apogee", "N-O", "Drogue"]
    );
    assert!(log.notes.iter().any(|note| note
        == "the flight summary's times of 0, no event, as the TRS manual writes zeroes for a \
            channel that didn't fire (p25): `Main`"));
    assert!(log.notes.iter().any(|note| note.starts_with(
        "the flight settings, as the file writes them: `Drogue <1=Normal,2=Backup,0=OFF>` 1; \
         `Main Alt <50-2000 by 50, 0=OFF>` 0; `Freq (Ctrl-F to change)` 921"
    )));
    assert!(!serde_json::to_string(&log).unwrap().contains('\u{fffd}'));
    insta::assert_json_snapshot!(log);
}

/// A bad cell in the table is refused naming the file's line, past the preamble.
#[test]
fn a_bad_cell_names_the_files_line() {
    let text = String::from_utf8_lossy(include_bytes!("samples/eggtimer-text-trs-2-07.txt"))
        .replacen("\r\n150,2,", "\r\n150,x,", 1);
    let error = import::read(text.as_bytes()).unwrap_err().to_string();
    assert_eq!(
        error,
        "Eggtimer text line 35: \"x\" in cell 2, under \"Alt\", isn't a number"
    );
}
