//! A GPS receiver's NMEA 0183 sentences, read through the importer from invented files and pinned
//! whole by snapshots (M7.1c1, ADR-254). The files hold made-up positions, times and dates, each
//! sentence with a checksum that checks out unless it is meant to fail, and the quirks the
//! project's sample logs show: noise before the first sentence, empty times and a placeholder date
//! before the fix, the real date while the status is still void, a 2 s step back, a second of dead
//! reckoning, `GSV` bursts, a tail cut off in NUL bytes, and a receiver that never gets a fix.

use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The channel named as given.
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    let matching: Vec<&Channel> = log.channels.iter().filter(|c| c.column == column).collect();
    assert_eq!(matching.len(), 1, "{column}: {:?}", names(log));
    matching[0]
}

fn names(log: &FlightLog) -> Vec<&str> {
    log.channels.iter().map(|c| c.column.as_str()).collect()
}

/// The note that opens as given, the only one.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
    matching[0]
}

/// The note that opens as given, the only one, holding the part given.
fn note_with(log: &FlightLog, start: &str, part: &str) {
    let note = note(log, start);
    assert!(note.contains(part), "{part}\n{note}");
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * b.abs().max(1.0)
}

/// The worked example in `docs/format/flight-logs.md` ("NMEA 0183" section): its two sentences,
/// copied from the page, read to the numbers the page states, so that the page can't drift from
/// the reader.
#[test]
fn the_flight_logs_page_worked_example_reads_as_the_page_says() {
    // Keep these two lines identical to the page's example.
    let text = "$GPGGA,123519.000,4807.0380,N,01131.0000,E,1,08,0.9,545.4,M,46.9,M,,0000*59\r\n\
                $GPRMC,123519.000,A,4807.0380,N,01131.0000,E,022.4,084.4,230394,,,A*62\r\n";
    let log = import::read(text.as_bytes()).unwrap();
    assert_eq!(log.format, LogFormat::Nmea0183);
    let latitude = channel(&log, "GPGGA latitude").values[0];
    assert!((latitude - 48.1173).abs() <= 1e-9, "{latitude}");
    let longitude = channel(&log, "GPGGA longitude").values[0];
    assert!(close(longitude, 11.0 + 31.0 / 60.0), "{longitude}");
    assert!((longitude - 11.5167).abs() < 5e-5);
    assert_eq!(channel(&log, "GPGGA altitude").values, [545.4]);
    let speed = channel(&log, "GPRMC speed").values[0];
    assert!(close(speed, 22.4 * 1852.0 / 3600.0), "{speed}");
    assert!((speed - 11.52).abs() < 5e-3);
    assert_eq!(channel(&log, "GPRMC course").values, [84.4]);
    note_with(
        &log,
        "`RMC date` read in UTC days",
        "the log's zero, the first sentence with a time, is 12:35:19.000 UTC on 1994-03-23",
    );
}

/// A cold start: the first line noise, two seconds with no time, two with a placeholder date and
/// one with the real date while void, then a fix; the clock steps back 2 s; one second of dead
/// reckoning; and the file ends in a sentence cut off by NUL bytes.
#[test]
fn a_cold_start_reads_from_its_first_time() {
    let bytes = include_bytes!("samples/nmea-cold-start.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::Nmea0183));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::Nmea0183);
    assert_eq!(log.logger, "NMEA 0183 receiver (talker GP)");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert!(log.events.is_empty());

    // Two seconds with an empty time, then the zero; the clock steps from 8 s back to 6 s.
    let status = channel(&log, "GPRMC status");
    assert_eq!(status.time_s.len(), 16);
    assert!(status.time_s[..2].iter().all(|t| t.is_nan()));
    let times: Vec<f64> = status.time_s[2..].to_vec();
    assert_eq!(
        times,
        [
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 6.0, 7.0, 8.0, 9.0, 10.0
        ]
    );
    // Void before the fix and in the second of dead reckoning.
    let valid: Vec<f64> = status.values[2..].to_vec();
    assert_eq!(
        valid,
        [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0
        ]
    );
    assert_eq!(channel(&log, "GPRMC mode").values[14], 6.0);
    assert_eq!(channel(&log, "GPGGA fix quality").values[14], 6.0);

    // 4807.0380,N is 48° 7.038′, 48.1173°; 22.40 knots is 11.5236 m/s.
    let latitude = channel(&log, "GPGGA latitude");
    assert_eq!(latitude.quantity, Quantity::Latitude);
    assert!(latitude.values[..5].iter().all(|v| v.is_nan()));
    assert!(close(latitude.values[5], 48.1173), "{}", latitude.values[5]);
    let speed = channel(&log, "GPRMC speed");
    assert_eq!(speed.quantity, Quantity::Speed);
    assert!(close(speed.values[11], 22.4 * 1852.0 / 3600.0));
    assert!((speed.values[11] - 11.5236).abs() < 5e-5);
    let altitude = channel(&log, "GPGGA altitude");
    assert_eq!(altitude.quantity, Quantity::GnssAltitude);
    assert_eq!(altitude.values[5], 545.4);
    assert_eq!(
        channel(&log, "GPGGA geoid separation").quantity,
        Quantity::Distance
    );

    // `GSV` on its second's time: two bursts of ten satellites.
    let in_view = channel(&log, "GPGSV satellites in view");
    assert_eq!(in_view.time_s, [4.0, 10.0]);
    assert_eq!(in_view.values, [10.0, 10.0]);
    let snr = channel(&log, "GPGSV PRN 30 SNR");
    assert_eq!(snr.time_s, [4.0, 10.0]);
    assert!(snr.values.iter().all(|v| v.is_nan()));
    assert_eq!(channel(&log, "GPGSV PRN 12 elevation").values, [60.0, 60.0]);
    assert_eq!(
        log.channels
            .iter()
            .filter(|c| c.column.ends_with(" azimuth"))
            .count(),
        10
    );
    assert_eq!(
        channel(&log, "GPGSA satellites used").values[5..8],
        [8.0, 8.0, 8.0]
    );

    note_with(
        &log,
        "`RMC date` read in UTC days, from status-`A` sentences only: ",
        "is 16:42:07.000 UTC on 2024-09-21, the date of the first `RMC` with status `A`; 4 \
         status-`V` `RMC` carry 2 distinct dates, 1 of which no status-`A` one gives",
    );
    note(
        &log,
        "`GSA` and `GSV` read in the time of the `GGA` or `RMC` before them, from the sentence \
         order: in 14 of the file's 14 `GGA` and `RMC` pairs, the `RMC` has the `GGA`'s time; 2 \
         come before any sentence with a time",
    );
    note(&log, "4 `GGA` and `RMC` sentences have an empty time");
    note(&log, "the time goes back 1 times, by up to 2.000 s");
    note(&log, "1 lines hold no sentence, no `$`");
    note(
        &log,
        "1 lines hold a sentence cut off before its checksum, the first line 56",
    );
    note(
        &log,
        "the file holds 18 NUL bytes, or stand-ins (U+FFFD) for bytes that aren't UTF-8, on 2 lines",
    );
    insta::assert_json_snapshot!(log);
}

/// Noise glued to the first sentence, two sentences on one line, a damaged sentence, a type
/// this reader doesn't read, a fix south and west, and midnight passed with a date either side.
#[test]
fn noise_beside_sentences_and_midnight() {
    let bytes = include_bytes!("samples/nmea-noise-and-midnight.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::Nmea0183));
    let log = import::read(bytes).unwrap();
    let altitude = channel(&log, "GPGGA altitude");
    assert_eq!(altitude.time_s, [0.0, 1.0, 2.0, 3.0, 4.0]);
    assert_eq!(altitude.values[1..], [917.36, 918.14, 918.93, 919.61]);
    let latitude = channel(&log, "GPRMC latitude");
    assert!(close(latitude.values[1], -41.2875));
    let longitude = channel(&log, "GPRMC longitude");
    assert!(close(longitude.values[1], -174.7762));
    // The `GSA` run onto the `GGA`'s line is read; the damaged one isn't.
    assert_eq!(
        channel(&log, "GPGSA fix type").values,
        [1.0, 3.0, 3.0, 3.0, 3.0]
    );

    note_with(
        &log,
        "`RMC date` read in UTC days",
        "is 23:59:57.000 UTC on 2025-12-31, the date of the first `RMC` with status `A`",
    );
    note(
        &log,
        "the time of day passes midnight 1 times: without a date, a time of day more than 12 \
         hours before the last is the next day's",
    );
    note(
        &log,
        "1 lines hold bytes beside their sentences, the first line 1",
    );
    note(
        &log,
        "1 lines hold a sentence whose checksum fails, or with bytes NMEA 0183 doesn't allow, \
         the first line 12",
    );
    note(
        &log,
        "sentences of types this reader doesn't read, `GPVTG` (1): their fields are left out",
    );
    assert!(
        log.notes
            .iter()
            .all(|n| !n.starts_with("the time goes back"))
    );
    insta::assert_json_snapshot!(log);
}

/// A receiver that never gets a fix still reads: its positions all gaps, its times of day alone,
/// with notes; its last sentence fails its checksum among bytes that aren't text.
#[test]
fn a_receiver_that_never_gets_a_fix_reads_with_gaps() {
    let bytes = include_bytes!("samples/nmea-never-fixed.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::Nmea0183));
    let log = import::read(bytes).unwrap();
    for column in [
        "GPGGA latitude",
        "GPGGA longitude",
        "GPRMC latitude",
        "GPRMC longitude",
        "GPGGA altitude",
    ] {
        let channel = channel(&log, column);
        assert_eq!(channel.values.len(), 10, "{column}");
        assert!(channel.values.iter().all(|v| v.is_nan()), "{column}");
    }
    let quality = channel(&log, "GPGGA fix quality");
    assert_eq!(
        quality.time_s[1..],
        [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );
    assert_eq!(quality.values, [0.0; 10]);
    assert_eq!(channel(&log, "GPGSV PRN 18 SNR").values, [33.0]);
    assert_eq!(channel(&log, "GPGSV PRN 18 SNR").time_s, [4.0]);
    note_with(
        &log,
        "`RMC date` read in UTC days",
        "no `RMC` has status `A`, so the times are times of day alone, from the first sentence \
         with a time, at 03:14:07.420 UTC; 7 status-`V` `RMC` carry 1 distinct dates",
    );
    note(
        &log,
        "no `GGA` or `RMC` holds a position: the receiver never had a fix",
    );
    note(
        &log,
        "1 lines hold a sentence whose checksum fails, or with bytes NMEA 0183 doesn't allow, \
         the first line 33",
    );
    insta::assert_json_snapshot!(log);
}
