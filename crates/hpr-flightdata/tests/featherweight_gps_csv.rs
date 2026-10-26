//! Featherweight's GPS tracker and ground-station CSV, read through the importer from invented
//! files and pinned whole by snapshots (M7.1c1, ADR-254). The files hold made-up positions and
//! numbers in the four layouts: the tracker's older log of 16 columns and newer of 21, and the
//! ground station's log of 17 columns and of 40.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The channel of a column's name, one only.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    let matching: Vec<&Channel> = log.channels.iter().filter(|c| c.column == column).collect();
    assert_eq!(matching.len(), 1, "{column}");
    matching[0]
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * b.abs().max(1.0)
}

/// The older tracker log: rows that step back and repeat kept in the file's order on
/// `UTCTIME`, across a leap day's UTC midnight; hexadecimal flags; `UNIXTIME` 6 h ahead, as
/// written.
#[test]
fn the_older_tracker_log_keeps_its_rows_in_the_files_order() {
    let log = import::read(include_bytes!("samples/fwgps-tracker-older.csv")).unwrap();
    assert_eq!(log.format, LogFormat::FeatherweightGpsCsv);
    assert_eq!(log.logger, "Featherweight GPS tracker");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert!(log.battery_v.is_none());
    // Every column a channel, `UTCTIME` as UTC's time of day.
    assert_eq!(log.channels.len(), 16);
    let of_day = channel(&log, "UTCTIME");
    assert_eq!(of_day.quantity, Quantity::TimeOfDay);
    assert!(close(of_day.values[0], 86_399.6));
    assert!(close(of_day.values[5], 0.0));
    let alt = channel(&log, "ALT");
    assert_eq!(alt.quantity, Quantity::GnssAltitude);
    assert_eq!(alt.values[0], 2000.0 * 0.3048);
    let times: Vec<f64> = alt
        .time_s
        .iter()
        .map(|t| (t * 10.0).round() / 10.0)
        .collect();
    assert_eq!(
        times,
        [0.0, 0.1, 0.2, 0.3, 1.2, 0.4, 0.5, 0.5, 0.6, 1.6],
        "the file's order, past midnight into March"
    );
    assert_eq!(channel(&log, "FLAGS").values[0], 3.0);
    let unix = channel(&log, "UNIXTIME");
    assert_eq!(unix.quantity, Quantity::Unrecognized);
    assert_eq!(unix.values[0], 1_709_272_799.6);
    assert!(
        log.notes.iter().any(|note| note.starts_with(
            "`UNIXTIME` kept as written, off `UTCTIME` by whole hours: it runs 6 h ahead of \
             `UTCTIME` on every row"
        )),
        "{:#?}",
        log.notes
    );
    assert!(log.notes.contains(
        &"times go back 1 times from one row to the next, by up to 0.800 s, and 1 rows repeat a \
          time already written: every row is kept, in the file's order"
            .to_owned()
    ));
    insta::assert_json_snapshot!(log);
}

/// The newer tracker log: feet and feet per second decided by the file's own rows, the
/// detection flags as 1 and 0, and a `NaN` cell a gap.
#[test]
fn the_newer_tracker_log_decides_its_units_from_its_rows() {
    let log = import::read(include_bytes!("samples/fwgps-tracker-newer.csv")).unwrap();
    assert_eq!(log.channels.len(), 21);
    let alt = channel(&log, "ALT");
    assert!(close(alt.values[13], 5300.0 * 0.3048));
    let agl = channel(&log, "Altitude AGL");
    assert_eq!(agl.quantity, Quantity::Distance);
    assert!(close(agl.values[1], 100.0 * 0.3048));
    let up = channel(&log, "VERTV");
    assert_eq!(up.quantity, Quantity::Speed);
    assert!(close(up.values[0], 30.48));
    assert!(close(channel(&log, "HORZV").values[0], 9.144));
    assert_eq!(channel(&log, "HEAD").quantity, Quantity::Angle);
    assert_eq!(channel(&log, "BATT").quantity, Quantity::Voltage);
    assert_eq!(
        channel(&log, "Launch detection").values[..3],
        [0.0, 1.0, 1.0]
    );
    let distance = channel(&log, "Distance (feet)");
    assert!(distance.values[0].is_nan());
    assert!(close(distance.values[2], 60.0 * 0.3048));
    // `HORZV` against the distance between fixes decides the file's feet; `VERTV` against the
    // change of `ALT` shows only that it is in `ALT`'s unit a second.
    assert_eq!(
        log.notes[..3],
        [
            "`HORZV` read in feet per second, from the manual and the distance between fixes: the \
             distance between fixes a second, in feet, is a median of 1.000 times `HORZV` over 13 \
             pairs of rows",
            "`ALT` and `Altitude AGL` read in feet, from the manual and the horizontal speed: the \
             distance between fixes a second, in feet, is a median of 1.000 times `HORZV` over 13 \
             pairs of rows; the change of `ALT` a second is a median of 1.000 times `VERTV` over 13 \
             pairs of rows; `Altitude AGL` is `ALT` less one height, to 0.05 ft, on 14 of 14 rows, \
             as a height above the pad is",
            "`VERTV` read in feet per second, in the altitude's unit, from the change of altitude: the \
             change of `ALT` a second is a median of 1.000 times `VERTV` over 13 pairs of rows",
        ]
    );
    insta::assert_json_snapshot!(log);
}

/// The ground station's log: CR LF, two trackers each with their own channels, named with the
/// tracker; a no-fix row's zeros as gaps; the date carrying the clock past midnight.
#[test]
fn the_ground_stations_log_keeps_each_tracker_apart() {
    let log = import::read(include_bytes!("samples/fwgps-ground-station.csv")).unwrap();
    assert_eq!(log.logger, "Featherweight GPS ground station");
    // 15 channels a tracker: every column but `TRACKER` and `DATE`.
    assert_eq!(log.channels.len(), 30);
    assert_eq!(log.channels[0].column, "TIME [Alpha]");
    assert_eq!(log.channels[15].column, "TIME [Bravo]");
    let lat = channel(&log, "LAT [Alpha]");
    assert_eq!(lat.time_s, [0.0, 1.0, 2.0]);
    assert!(lat.values[0].is_nan());
    assert_eq!(lat.values[1], 45.12345);
    assert!(channel(&log, "ALT [Alpha]").values[0].is_nan());
    assert_eq!(channel(&log, "ALT [Bravo]").time_s, [1.5, 2.5]);
    assert_eq!(channel(&log, "FLAGS [Bravo]").values, [15.0, 15.0]);
    let notes = log.notes.join("\n");
    for expected in [
        "the file holds 2 trackers' rows, each its own set of channels named with the tracker in \
         brackets, in the order they first appear: \"Alpha\" (3 rows) and \"Bravo\" (2 rows)",
        "1 rows have `LAT` and `LON` both 0, as a receiver writes before it has a fix: their \
         `LAT`, `LON` and `ALT` are read as gaps",
        "`DATE` and `TIME` kept as written, in a time zone the file doesn't state. They are the \
         log's clock, and its zero is the first row's, \"2025-12-31 23:59:58.500\". At the first \
         fix, `TIME` is ",
    ] {
        assert!(notes.contains(expected), "{expected}\n{notes}");
    }
    insta::assert_json_snapshot!(log);
}

/// The ground station's log with its radio's figures: the heights decided by `Alt AGL (ft)`, a
/// percentage as its number, the found rocket's (0, 0) as gaps, and `Found rocket ID` no channel.
#[test]
fn the_ground_stations_longer_log_reads_every_figure() {
    let log = import::read(include_bytes!("samples/fwgps-ground-station-lora.csv")).unwrap();
    // 40 columns less `TRACKER`, `DATE` and `Found rocket ID`.
    assert_eq!(log.channels.len(), 37);
    assert!(log.channels.iter().all(|c| !c.column.contains('[')));
    assert!(log.channels.iter().all(|c| c.column != "Found rocket ID"));
    let tracker = channel(&log, "TRACKER Alt asl");
    assert_eq!(tracker.quantity, Quantity::GnssAltitude);
    assert!(close(tracker.values[1], 4005.0 * 0.3048));
    assert_eq!(
        channel(&log, "LoRa Success Rate").values,
        [100.0, 67.0, 50.0]
    );
    let range = channel(&log, "LoRa Max Range (kft)");
    assert_eq!(range.quantity, Quantity::Distance);
    assert!(close(range.values[0], 12.5 * 304.8));
    assert!(channel(&log, "Found rocket lat").values[0].is_nan());
    assert!(channel(&log, "Found rocket altitude").values[0].is_nan());
    assert!(close(
        channel(&log, "GS_ASL_offset").values[0],
        -5.0 * 0.3048
    ));
    // Too few moving rows to check `HORZV`, so the manual and `Alt AGL (ft)`, whose header says
    // feet, decide the heights.
    assert!(log.notes.contains(
        &"`GS Alt asl`, `TRACKER Alt asl`, `Found rocket altitude` and `GS_ASL_offset` read in \
          feet, from the manual and `Alt AGL (ft)`: the file has too few pairs of moving rows to \
          check `HORZV` over, 0 of the 10 needed; the file has too few pairs of moving rows to \
          check `VERTV` over, 2 of the 10 needed; `Alt AGL (ft)` is `TRACKER Alt asl` less `GS Alt \
          asl` and `GS_ASL_offset`, to 0.15 ft, on 3 of 3 rows"
            .to_owned()
    ));
    insta::assert_json_snapshot!(log);
}
