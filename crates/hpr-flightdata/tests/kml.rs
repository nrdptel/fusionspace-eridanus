//! KML files holding a GPS track, read through the importer from invented files and pinned whole
//! by snapshots (M7.1c1, ADR-254). The files hold made-up positions and times in each writer's
//! layout: fw2kml's path and `gx:Track`, Missile Works' RTx, AltOS's three kinds, Big Red Bee's
//! BeeLine, an unnamed writer's, GPS Visualizer's, and pins with time stamps.

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};

/// The importer's record of a sample, which must be KML with an empty primary track.
#[expect(
    clippy::unwrap_used,
    reason = "a test's helper: a sample that isn't read fails the test"
)]
fn read(bytes: &[u8]) -> FlightLog {
    assert_eq!(import::detect(bytes), Some(LogFormat::Kml));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::Kml);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert!(
        log.notes[0].starts_with("the record's altitude track is empty"),
        "{:?}",
        log.notes
    );
    for channel in &log.channels {
        assert_eq!(
            channel.time_s.len(),
            channel.values.len(),
            "{}",
            channel.column
        );
    }
    log
}

/// The columns and quantities of a record's channels.
fn columns(log: &FlightLog) -> Vec<(&str, Quantity)> {
    log.channels
        .iter()
        .map(|channel| (channel.column.as_str(), channel.quantity))
        .collect()
}

/// Whether a note opens with the text.
fn noted(log: &FlightLog, opening: &str) -> bool {
    log.notes.iter().any(|note| note.starts_with(opening))
}

/// fw2kml's path: one placemark, its altitude in meters from the standard, no time at all.
#[test]
fn an_fw2kml_path_reads_with_its_times_as_gaps() {
    let log = read(include_bytes!("samples/kml-fw2kml-path.kml"));
    assert_eq!(log.logger, "fw2kml");
    assert_eq!(
        columns(&log),
        [
            ("fw2kml - Flight 0 latitude", Quantity::Latitude),
            ("fw2kml - Flight 0 longitude", Quantity::Longitude),
            ("fw2kml - Flight 0 altitude", Quantity::GnssAltitude),
        ]
    );
    let altitude = &log.channels[2];
    assert_eq!(
        altitude.values,
        [500.0, 500.5, 620.25, 810.0, 790.75, 505.125]
    );
    assert!(altitude.time_s.iter().all(|time| time.is_nan()));
    assert!(noted(&log, "the file gives none of its 6 points a time"));
    assert!(noted(
        &log,
        "`coordinates` read in meters, from the KML standard: "
    ));
    insta::assert_json_snapshot!(log);
}

/// fw2kml's `gx:Track`: each point at its `when`, in UTC, across midnight; the tour's camera
/// isn't a track.
#[test]
fn a_gx_track_reads_each_point_at_its_when() {
    let log = read(include_bytes!("samples/kml-fw2kml-track.kml"));
    assert_eq!(log.channels.len(), 3);
    assert_eq!(log.channels[0].time_s, [0.0, 0.1, 0.2, 0.5, 1.5]);
    assert_eq!(log.channels[2].values[4], 805.0);
    assert!(noted(
        &log,
        "the log's zero is the first point the file times: `when` 2030-01-02T23:59:59.800Z"
    ));
    assert!(noted(
        &log,
        "`when` read in UTC, from the KML standard: 5 times"
    ));
    insta::assert_json_snapshot!(log);
}

/// The same document three times over, each with an offset from UTC: read once, with a note.
#[test]
fn a_document_repeated_is_read_once() {
    let log = read(include_bytes!("samples/kml-concatenated.kml"));
    assert_eq!(log.channels.len(), 3);
    assert_eq!(log.channels[0].time_s, [0.0, 0.5, 1.0]);
    assert!(noted(&log, "the file holds its KML document 3 times over"));
    insta::assert_json_snapshot!(log);
}

/// Missile Works' RTx: a byte-order mark, CR LF, and a ground track of two-number tuples that
/// gives no altitude channel.
#[test]
fn an_rtx_file_reads_its_flight_and_ground_tracks() {
    let log = read(include_bytes!("samples/kml-rtx-ground-track.kml"));
    assert_eq!(log.logger, "Missile Works RTx");
    assert_eq!(
        columns(&log),
        [
            ("Flight Track latitude", Quantity::Latitude),
            ("Flight Track longitude", Quantity::Longitude),
            ("Flight Track altitude", Quantity::GnssAltitude),
            ("Ground Track latitude", Quantity::Latitude),
            ("Ground Track longitude", Quantity::Longitude),
        ]
    );
    assert_eq!(log.channels[3].values, log.channels[0].values);
    assert_eq!(
        log.channels[2].values.iter().copied().fold(0.0, f64::max),
        498.0
    );
    insta::assert_json_snapshot!(log);
}

/// AltOS's GPS path: the serial and flight numbers from the document's name, the product and
/// firmware from its snippet; the comments' `time` all 0, so no point has a time; their `alt`
/// equals the tuple's, so it adds no channel.
#[test]
fn an_altos_gps_path_reads_its_satellites_and_no_time() {
    let log = read(include_bytes!("samples/kml-altos-gps.kml"));
    assert_eq!(log.logger, "AltOS TeleGPS-v9.9");
    assert_eq!(log.serial_number.as_deref(), Some("1234"));
    assert_eq!(log.flight_number, Some(4));
    assert_eq!(log.firmware.as_deref(), Some("9.9.9"));
    assert_eq!(
        columns(&log),
        [
            ("GPS Altitude latitude", Quantity::Latitude),
            ("GPS Altitude longitude", Quantity::Longitude),
            ("GPS Altitude altitude", Quantity::GnssAltitude),
            ("GPS Altitude satellites", Quantity::Number),
        ]
    );
    assert!(noted(&log, "the comments' `time` is 0 beside every point"));
    assert!(log.channels[0].time_s.iter().all(|time| time.is_nan()));
    insta::assert_json_snapshot!(log);
}

/// AltOS's paths by flight state: the tuple's altitude is the barometer's, the comment's the GPS
/// receiver's, kept as written as it equals the tuple's nowhere, and the comment's time the clock,
/// in seconds from the first point.
#[test]
fn altos_flight_states_read_the_barometer_and_the_comment_times() {
    let log = read(include_bytes!("samples/kml-altos-states.kml"));
    assert_eq!(log.logger, "AltOS");
    assert_eq!(
        (log.serial_number.as_deref(), log.flight_number),
        (Some("4321"), Some(7))
    );
    let names: Vec<(&str, Quantity)> = columns(&log)[..5].to_vec();
    assert_eq!(
        names,
        [
            ("pad latitude", Quantity::Latitude),
            ("pad longitude", Quantity::Longitude),
            ("pad altitude", Quantity::AltitudeMsl),
            ("pad GPS altitude", Quantity::Unrecognized),
            ("pad satellites", Quantity::Number),
        ]
    );
    assert_eq!(log.channels.len(), 15);
    let boost = &log.channels[5..10];
    assert_eq!(boost[0].time_s, [1.0, 2.0, 3.01]);
    assert_eq!(boost[2].values, [601.5, 900.75, 1500.5]);
    assert_eq!(boost[3].values, [602.0, 640.0, 640.0]);
    assert!(noted(
        &log,
        "`time` read in seconds, from the steps between points. "
    ));
    assert!(noted(
        &log,
        "`alt` kept as written, as it equals the tuple's altitude at no point, so nothing in \
         the file shows its unit: 8 points carry both"
    ));
    assert!(noted(
        &log,
        "`coordinates` read in meters as the barometer's altitude above sea level, from the GPS \
         altitude AltOS writes beside each point. In 3 tracks the tuple's altitude differs from the \
         comment's `alt` at 8 of the 8 points"
    ));
    insta::assert_json_snapshot!(log);
}

/// AltOS's barometric and GPS paths with a point at each event: points are one-sample tracks,
/// repeated names numbered; a byte that isn't UTF-8 in the snippet does no harm; the stated
/// apogee is the snippet's `Max baro height`.
#[test]
fn altos_event_points_are_one_sample_tracks() {
    let log = read(include_bytes!("samples/kml-altos-events.kml"));
    assert_eq!(log.logger, "AltOS TeleMetrum-v9.9");
    let apogee = log.stated.apogee_m.unwrap();
    assert!((apogee - 3600.0 * 0.3048).abs() < 1e-9, "{apogee}");
    let latitudes: Vec<&str> = log
        .channels
        .iter()
        .filter(|channel| channel.quantity == Quantity::Latitude)
        .map(|channel| channel.column.as_str())
        .collect();
    assert_eq!(
        latitudes,
        [
            "Barometric Altitude latitude",
            "Boost (Baro) latitude",
            "Boost (Baro) (2) latitude",
            "Drogue (Baro) latitude",
            "GPS Altitude latitude",
            "Drogue (GPS) latitude",
        ]
    );
    let quantity = |column: &str| {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .map(|channel| channel.quantity)
    };
    assert_eq!(
        quantity("Boost (Baro) altitude"),
        Some(Quantity::AltitudeMsl)
    );
    assert_eq!(
        quantity("Boost (Baro) GPS altitude"),
        Some(Quantity::GnssAltitude)
    );
    assert_eq!(
        quantity("Drogue (GPS) altitude"),
        Some(Quantity::GnssAltitude)
    );
    assert_eq!(quantity("Drogue (GPS) GPS altitude"), None);
    assert!(log.events.is_empty());
    insta::assert_json_snapshot!(log);
}

/// Big Red Bee's BeeLine: zero-padded meters, each point's UTC time of day carried past
/// midnight, the comments before the first point and the memory dump after the document left
/// out with notes.
#[test]
fn a_beeline_file_reads_its_utc_times_past_midnight() {
    let log = read(include_bytes!("samples/kml-beeline.kml"));
    assert_eq!(log.logger, "Big Red Bee BeeLine");
    assert_eq!(
        columns(&log),
        [
            ("gps latitude", Quantity::Latitude),
            ("gps longitude", Quantity::Longitude),
            ("gps altitude", Quantity::GnssAltitude),
            ("gps satellites", Quantity::Number),
            ("gps offset", Quantity::Number),
        ]
    );
    assert_eq!(log.channels[2].values[0], 742.0);
    assert_eq!(
        log.channels[0].time_s,
        [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    );
    assert_eq!(log.channels[4].values[1], 4015.0);
    for opening in [
        "the file holds 1 comments outside its document",
        "the document holds 2 comments outside any `coordinates`",
        "2 comments in `coordinates` follow no point's tuple",
        "the time of day steps back past midnight 1 times",
        "the log's zero is the first point the file times: `UTC` 23:59:57 UTC",
    ] {
        assert!(noted(&log, opening), "{opening}: {:?}", log.notes);
    }
    insta::assert_json_snapshot!(log);
}

/// An unnamed writer's `Alt`/`Seconds` comments: the height above the ground a distance, the
/// `Seconds` the clock from the first point.
#[test]
fn a_height_above_ground_is_a_distance() {
    let log = read(include_bytes!("samples/kml-seconds.kml"));
    assert_eq!(log.logger, "KML");
    assert_eq!(
        columns(&log),
        [
            ("Path latitude", Quantity::Latitude),
            ("Path longitude", Quantity::Longitude),
            ("Path height above ground", Quantity::Distance),
        ]
    );
    let time = &log.channels[0].time_s;
    assert_eq!(time[0], 0.0);
    assert!((time[1] - 0.75).abs() < 1e-12, "{time:?}");
    assert!(noted(&log, "`Seconds` read in seconds, from its name: "));
    insta::assert_json_snapshot!(log);
}

/// GPS Visualizer's paths, each of a `MultiGeometry`'s lines its own part; the altitude kept as
/// written, as the writer takes a source's altitude as meters unless told otherwise.
#[test]
fn gps_visualizers_altitude_is_kept_as_written() {
    let log = read(include_bytes!("samples/kml-gpsvisualizer.kml"));
    assert_eq!(log.logger, "GPS Visualizer");
    assert_eq!(
        columns(&log),
        [
            ("2030-01-02-Log part 1 latitude", Quantity::Latitude),
            ("2030-01-02-Log part 1 longitude", Quantity::Longitude),
            ("2030-01-02-Log part 1 altitude", Quantity::Unrecognized),
            ("2030-01-02-Log part 2 latitude", Quantity::Latitude),
            ("2030-01-02-Log part 2 longitude", Quantity::Longitude),
            ("2030-01-02-Log part 2 altitude", Quantity::Unrecognized),
        ]
    );
    assert!(noted(
        &log,
        "`coordinates` kept as written, as GPS Visualizer takes a source's altitude as meters \
         unless the source's header names feet. 2 tracks carry an altitude, the highest 1620 \
         above the first"
    ));
    insta::assert_json_snapshot!(log);
}

/// Pins: a placemark's time stamp times its point and makes it an event; the path and the
/// pin with no stamp keep their times as gaps; a path with no altitude mode keeps no altitude.
#[test]
fn pins_with_time_stamps_are_events() {
    let log = read(include_bytes!("samples/kml-pins.kml"));
    let events: Vec<(&str, f64)> = log
        .events
        .iter()
        .map(|event| (event.name.as_str(), event.time_s))
        .collect();
    assert_eq!(events, [("Pad", 0.0), ("Landing", 150.5)]);
    assert_eq!(
        columns(&log),
        [
            ("Path latitude", Quantity::Latitude),
            ("Path longitude", Quantity::Longitude),
            ("Pad latitude", Quantity::Latitude),
            ("Pad longitude", Quantity::Longitude),
            ("Pad altitude", Quantity::GnssAltitude),
            ("Landing latitude", Quantity::Latitude),
            ("Landing longitude", Quantity::Longitude),
            ("Note latitude", Quantity::Latitude),
            ("Note longitude", Quantity::Longitude),
            ("Note altitude", Quantity::Unrecognized),
        ]
    );
    assert!(noted(&log, "4 of the file's 6 points have no time"));
    insta::assert_json_snapshot!(log);
}

/// A file nesting its elements past the reader's limit is refused before the parser descends it,
/// on a thread with a small stack, through the importer; one at the limit reads.
#[expect(
    clippy::disallowed_types,
    reason = "a test's thread, to read on a small stack; not the pure core"
)]
#[test]
fn nesting_past_the_limit_is_refused_not_a_crash() {
    use hpr_flightdata::kml::MAX_DEPTH;
    // `kml`, then `levels` of `a`, then `Point` and `coordinates`.
    let nested = |levels: usize| {
        format!(
            "<kml>{}<Point><coordinates>1,2,3</coordinates></Point>{}</kml>",
            "<a>".repeat(levels),
            "</a>".repeat(levels)
        )
    };
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(move || {
            let log = import::read(nested(MAX_DEPTH - 3).as_bytes()).unwrap();
            assert_eq!(log.channels.len(), 3);
            for levels in [MAX_DEPTH - 2, 300] {
                let error = import::read(nested(levels).as_bytes())
                    .unwrap_err()
                    .to_string();
                assert!(
                    error.contains(&format!("nests elements {} deep", levels + 3)),
                    "{error}"
                );
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// A document type declaration after the XML declaration: the importer hands the file to this
/// reader, which refuses it by name.
#[test]
fn a_document_type_is_refused_by_name() {
    let text = "<?xml version=\"1.0\"?>\n<!DOCTYPE kml [<!ENTITY a \"aaaa\">]>\n<kml>&a;</kml>\n";
    assert_eq!(import::detect(text.as_bytes()), Some(LogFormat::Kml));
    let error = import::read(text.as_bytes()).unwrap_err().to_string();
    assert!(error.contains("declares a document type"), "{error}");
}

/// A path with no time survives its own JSON: each channel's `NaN` time is written as `null` and
/// read back as `NaN`.
#[test]
fn a_timeless_path_survives_its_json() {
    let log = read(include_bytes!("samples/kml-fw2kml-path.kml"));
    assert!(
        log.channels
            .iter()
            .all(|channel| channel.time_s.iter().all(|time| time.is_nan()))
    );
    let json = serde_json::to_string(&log).unwrap();
    assert!(json.contains("\"time_s\":[null,null,"), "{json}");
    let back: FlightLog = serde_json::from_str(&json).unwrap();
    for (channel, original) in back.channels.iter().zip(&log.channels) {
        assert_eq!(channel.time_s.len(), original.time_s.len());
        assert!(channel.time_s.iter().all(|time| time.is_nan()));
        assert_eq!(channel.values, original.values);
    }
    assert_eq!(serde_json::to_string(&back).unwrap(), json);
}
