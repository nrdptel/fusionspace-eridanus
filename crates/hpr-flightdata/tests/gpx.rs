//! GPX files read through the importer from invented files, every value held to gpxpy 1.6.2's
//! reading of the same bytes and each record pinned whole by a snapshot (M7.1d1, ADR-276).
//!
//! `validation/oracles/gpx/gpxpy_read.py` wrote gpxpy's reading of every `samples/gpx-*.gpx` to
//! `fixtures/gpx/gpxpy.json`: each track's segments and points, the waypoints and the routes. The
//! samples hold made-up positions, heights and times: a phone app's GPX 1.1 track with waypoints
//! and a route, a tracker's GPX 1.0 track in two segments with courses and speeds, and a GPX 1.1
//! file with no namespace holding two flights and a walk. Each must read to the channels and
//! figures gpxpy's reading gives, every latitude, longitude, elevation, time, satellite count,
//! dilution of precision, course and speed equal to the bit.

#![allow(
    clippy::disallowed_methods,
    reason = "the test lists the sample files, to hold the oracle to all of them"
)]
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "tests stop at the failure, as `#[test]` functions may (clippy.toml)"
)]

use std::collections::HashSet;
use std::path::Path;

use hpr_flightdata::import;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct Oracle {
    gpxpy: String,
    files: Vec<File>,
}

#[derive(Deserialize)]
struct File {
    file: String,
    sha256: String,
    version: String,
    creator: Option<String>,
    tracks: Vec<Track>,
    waypoints: Vec<Point>,
    routes: Vec<Route>,
}

#[derive(Deserialize)]
struct Track {
    name: Option<String>,
    segments: Vec<Vec<Point>>,
}

#[derive(Deserialize)]
struct Route {
    name: Option<String>,
    points: Vec<Point>,
}

/// A point as gpxpy read it; `None` where it read nothing. The waypoints and route points have no
/// satellites, dilutions, course or speed in the reading.
#[derive(Deserialize)]
struct Point {
    lat: f64,
    lon: f64,
    ele: Option<f64>,
    time_us: Option<i64>,
    name: Option<String>,
    #[serde(default)]
    sat: Option<f64>,
    #[serde(default)]
    hdop: Option<f64>,
    #[serde(default)]
    vdop: Option<f64>,
    #[serde(default)]
    pdop: Option<f64>,
    #[serde(default)]
    speed: Option<f64>,
    #[serde(default)]
    course: Option<f64>,
}

/// A channel's value at a point, `None` where the point holds none.
type Pick = fn(&Point) -> Option<f64>;

fn oracle() -> Oracle {
    serde_json::from_str(include_str!("fixtures/gpx/gpxpy.json")).unwrap()
}

fn samples() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples"))
}

/// The importer's record of a sample, which must be GPX with an empty primary track.
fn read(bytes: &[u8]) -> FlightLog {
    assert_eq!(import::detect(bytes), Some(LogFormat::Gpx));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::Gpx);
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    assert!(
        log.notes[0].starts_with("the record's altitude track is empty"),
        "{:?}",
        log.notes
    );
    for channel in &log.channels {
        assert_eq!(channel.time_s.len(), channel.values.len());
    }
    log
}

/// A name not yet used, gaining ` (2)`, ` (3)` … if it is, as the reader's names are.
fn unique(used: &mut HashSet<String>, name: String) -> String {
    let mut candidate = name.clone();
    let mut number = 1;
    while used.contains(&candidate) {
        number += 1;
        candidate = format!("{name} ({number})");
    }
    used.insert(candidate.clone());
    candidate
}

/// Whether two values are the same double, a gap (`NaN`) matching a gap.
fn same(ours: f64, theirs: f64) -> bool {
    ours.to_bits() == theirs.to_bits() || ours.is_nan() && theirs.is_nan()
}

/// A channel or figure as gpxpy's reading gives it: its name, quantity, times and values.
type Expected = (String, Quantity, Vec<f64>, Vec<f64>);

/// The channels and figures gpxpy's reading of a file gives, by the reader's documented rules:
/// each segment with points a track named after its track, numbered where the track has several
/// segments; a channel for each element some point of the segment holds; the times in seconds
/// from the first time the file gives a track point (else a waypoint or route point); the
/// waypoints and route points as figures, by name or number.
fn expected(file: &File) -> (Vec<Expected>, Vec<(String, Quantity, f64)>) {
    let track_points = file
        .tracks
        .iter()
        .flat_map(|track| &track.segments)
        .flatten();
    let places = file
        .waypoints
        .iter()
        .chain(file.routes.iter().flat_map(|route| &route.points));
    let zero = track_points
        .clone()
        .find_map(|point| point.time_us)
        .or_else(|| places.clone().find_map(|point| point.time_us));
    // A difference of whole microseconds below 2⁵³ converts exactly, so this is the correctly
    // rounded value of the same quotient the reader takes in nanoseconds.
    let seconds = |time: Option<i64>| time.map_or(f64::NAN, |us| (us - zero.unwrap()) as f64 / 1e6);

    let mut channels = Vec::new();
    let mut used = HashSet::new();
    for track in &file.tracks {
        let base = track.name.clone().unwrap_or_else(|| "track".to_owned());
        let several = track.segments.len() > 1;
        for (index, points) in track.segments.iter().enumerate() {
            if points.is_empty() {
                continue;
            }
            let label = if several {
                format!("{base} segment {}", index + 1)
            } else {
                base.clone()
            };
            let name = unique(&mut used, label);
            let time_s: Vec<f64> = points.iter().map(|point| seconds(point.time_us)).collect();
            let picks: [(&str, Quantity, Pick); 9] = [
                ("latitude", Quantity::Latitude, |p| Some(p.lat)),
                ("longitude", Quantity::Longitude, |p| Some(p.lon)),
                ("elevation", Quantity::Altitude, |p| p.ele),
                ("satellites", Quantity::Number, |p| p.sat),
                ("HDOP", Quantity::Number, |p| p.hdop),
                ("VDOP", Quantity::Number, |p| p.vdop),
                ("PDOP", Quantity::Number, |p| p.pdop),
                ("speed", Quantity::Speed, |p| p.speed),
                ("course", Quantity::Angle, |p| p.course),
            ];
            for (suffix, quantity, pick) in picks {
                if points.iter().any(|point| pick(point).is_some()) {
                    let values = points
                        .iter()
                        .map(|point| pick(point).unwrap_or(f64::NAN))
                        .collect();
                    channels.push((format!("{name} {suffix}"), quantity, time_s.clone(), values));
                }
            }
        }
    }

    let mut labels = Vec::new();
    for (index, point) in file.waypoints.iter().enumerate() {
        let name = point
            .name
            .clone()
            .unwrap_or_else(|| (index + 1).to_string());
        labels.push((format!("waypoint {name}"), point));
    }
    for (index, route) in file.routes.iter().enumerate() {
        let name = route
            .name
            .clone()
            .unwrap_or_else(|| (index + 1).to_string());
        for (number, point) in route.points.iter().enumerate() {
            labels.push((format!("route {name} point {}", number + 1), point));
        }
    }
    let mut figures = Vec::new();
    let mut used = HashSet::new();
    for (label, point) in labels {
        let label = unique(&mut used, label);
        figures.push((format!("{label} latitude"), Quantity::Latitude, point.lat));
        figures.push((format!("{label} longitude"), Quantity::Longitude, point.lon));
        if let Some(ele) = point.ele {
            figures.push((format!("{label} elevation"), Quantity::Altitude, ele));
        }
        if point.time_us.is_some() {
            figures.push((
                format!("{label} time"),
                Quantity::Duration,
                seconds(point.time_us),
            ));
        }
    }
    (channels, figures)
}

/// Every sample's every value equals gpxpy's reading of the same bytes, and the reading covers
/// every sample.
#[test]
fn every_value_of_the_samples_equals_gpxpys() {
    let oracle = oracle();
    assert_eq!(oracle.gpxpy, "1.6.2");
    let mut on_disk: Vec<String> = std::fs::read_dir(samples())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("gpx-") && name.ends_with(".gpx"))
        .collect();
    on_disk.sort();
    let covered: Vec<&str> = oracle.files.iter().map(|file| file.file.as_str()).collect();
    assert_eq!(
        covered, on_disk,
        "rerun validation/oracles/gpx/gpxpy_read.py"
    );

    let mut versions = HashSet::new();
    let (mut values, mut times, mut figures_compared) = (0, 0, 0);
    for file in &oracle.files {
        let bytes = std::fs::read(samples().join(&file.file)).unwrap();
        let digest: String = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(digest, file.sha256, "{}: rerun gpxpy_read.py", file.file);
        versions.insert(file.version.clone());
        let log = read(&bytes);
        assert_eq!(Some(log.logger.clone()), file.creator, "{}", file.file);

        let (channels, figures) = expected(file);
        let ours: Vec<(&str, Quantity)> = log
            .channels
            .iter()
            .map(|channel| (channel.column.as_str(), channel.quantity))
            .collect();
        let theirs: Vec<(&str, Quantity)> = channels
            .iter()
            .map(|(column, quantity, _, _)| (column.as_str(), *quantity))
            .collect();
        assert_eq!(ours, theirs, "{}", file.file);
        for (channel, (column, _, time_s, expected)) in log.channels.iter().zip(&channels) {
            for (index, (ours, theirs)) in channel.values.iter().zip(expected).enumerate() {
                assert!(
                    same(*ours, *theirs),
                    "{} {column}[{index}]: {ours} against gpxpy's {theirs}",
                    file.file
                );
            }
            for (index, (ours, theirs)) in channel.time_s.iter().zip(time_s).enumerate() {
                assert!(
                    same(*ours, *theirs),
                    "{} {column} time[{index}]: {ours} against gpxpy's {theirs}",
                    file.file
                );
            }
            assert_eq!(channel.values.len(), expected.len());
            values += channel.values.len();
            times += channel.time_s.len();
        }

        let ours: Vec<(&str, Option<Quantity>, &[f64])> = log
            .stated
            .figures
            .iter()
            .map(|figure| {
                (
                    figure.name.as_str(),
                    figure.quantity,
                    figure.values.as_slice(),
                )
            })
            .collect();
        assert_eq!(ours.len(), figures.len(), "{}", file.file);
        for ((name, quantity, value), (their_name, their_quantity, theirs)) in
            ours.iter().zip(&figures)
        {
            assert_eq!(
                (*name, *quantity),
                (their_name.as_str(), Some(*their_quantity))
            );
            assert_eq!(value.len(), 1);
            assert!(
                same(value[0], *theirs),
                "{} {name}: {} against gpxpy's {theirs}",
                file.file,
                value[0]
            );
            figures_compared += 1;
        }
    }
    // Both versions are read, and the comparison isn't empty.
    assert_eq!(
        versions,
        HashSet::from(["1.0".to_owned(), "1.1".to_owned()])
    );
    assert_eq!((values, times, figures_compared), (196, 196, 12));
}

/// The phone app's GPX 1.1 track: its satellites and dilutions as channels, a point with no `ele`
/// a gap, its waypoints and route kept apart as figures, its extensions counted and not read.
#[test]
fn a_phone_apps_track_reads_with_its_places_kept_apart() {
    let log = read(include_bytes!("samples/gpx-1.1-phone-app.gpx"));
    assert_eq!(log.logger, "Invented Tracker App 2.3");
    let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
    assert_eq!(
        columns,
        [
            "Flight 7 latitude",
            "Flight 7 longitude",
            "Flight 7 elevation",
            "Flight 7 satellites",
            "Flight 7 HDOP",
            "Flight 7 VDOP",
            "Flight 7 PDOP",
        ]
    );
    // No channel holds a waypoint's or a route point's position: the track's 16 points only.
    assert!(
        log.channels
            .iter()
            .all(|channel| channel.values.len() == 16)
    );
    // The guide's worked example: the first point and the 10th, the highest, 9 s later and
    // 466.4 m (1,530 ft) higher.
    let elevation = &log.channels[2];
    assert_eq!(
        (log.channels[0].values[0], log.channels[1].values[0]),
        (35.200_012, -97.100_034)
    );
    assert_eq!(
        (log.channels[0].values[9], log.channels[1].values[9]),
        (35.200_165_9, -97.099_961_1)
    );
    assert_eq!((elevation.values[0], elevation.values[9]), (402.0, 868.4));
    assert_eq!((elevation.time_s[0], elevation.time_s[9]), (0.0, 9.0));
    let rise_m = elevation.values[9] - elevation.values[0];
    assert!((rise_m - 466.4).abs() < 1e-9, "{rise_m}");
    assert_eq!((rise_m / 0.3048).round(), 1530.0);
    let highest = elevation
        .values
        .iter()
        .copied()
        .filter(|value| !value.is_nan())
        .fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(highest, elevation.values[9]);
    assert!(elevation.values[12].is_nan());
    assert_eq!(elevation.time_s[..3], [0.0, 1.0, 2.001]);
    let names: Vec<&str> = log.stated.figures.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        names[..5],
        [
            "waypoint Pad latitude",
            "waypoint Pad longitude",
            "waypoint Pad elevation",
            "waypoint Pad time",
            "waypoint Landing latitude",
        ]
    );
    // The pad's waypoint was marked 24 s before the track's first point, the log's zero.
    assert_eq!(log.stated.figures[3].values, [-24.0]);
    assert_eq!(log.stated.figures[3].written, "2031-07-14T17:03:40Z");
    assert!(names.contains(&"route Recovery walk point 3 longitude"));
    assert!(log.events.is_empty());
    for note in [
        "`ele` read in meters, from GPX 1.1's schema and GPX 1.0's manual, as an altitude whose \
         datum the file doesn't state",
        "the log's zero is `2031-07-14T17:04:04.000Z`, the first time the file gives a track point",
        "waypoints and routes kept apart from the tracks, as figures the file states, as places \
         marked or planned, not flown: 2 waypoints, 1 route, 5 places read",
        "elements no reading uses, not read: `extensions` in 2 track points, `sym` in 1 waypoint",
    ] {
        assert!(
            log.notes.iter().any(|n| n.starts_with(note)),
            "{note}\n{:#?}",
            log.notes
        );
    }
    insta::assert_json_snapshot!(log);
}

/// The tracker's GPX 1.0 track: two segments, numbered; courses and speeds read; times stated
/// with an offset moved to UTC; a point with no fix kept as written, counted.
#[test]
fn a_trackers_gpx_1_0_reads_its_segments_courses_and_speeds() {
    let log = read(include_bytes!("samples/gpx-1.0-tracker.gpx"));
    let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
    assert_eq!(columns[0], "Rocket segment 1 latitude");
    assert!(columns.contains(&"Rocket segment 2 speed"));
    assert!(columns.contains(&"Rocket segment 2 course"));
    let second = log
        .channels
        .iter()
        .find(|c| c.column == "Rocket segment 2 speed")
        .unwrap();
    // 08:30:06.5 at −04:00 is 12:30:06.5 UTC, 6.5 s after the first point's 12:30:00 UTC.
    assert_eq!(second.time_s, [6.5, 7.5, 8.5, 9.5, 10.5]);
    assert_eq!(second.quantity.unit(), "m/s");
    assert!(log.notes.iter().any(|n| n.starts_with(
        "track points whose `fix` is `none`, where the receiver had no fix, kept as written: 1"
    )));
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("other namespaces' elements in 1 track point"))
    );
    insta::assert_json_snapshot!(log);
}

/// Two flights of one name and an unnamed walk: named apart (the first track's empty second
/// segment numbers its first), the earlier flight second in the file and so before the zero;
/// times with no zone read as UTC; a point with no time a gap.
#[test]
fn two_flights_in_one_file_read_as_two_tracks() {
    let log = read(include_bytes!("samples/gpx-1.1-two-flights.gpx"));
    let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
    assert_eq!(
        columns,
        [
            "Flight segment 1 latitude",
            "Flight segment 1 longitude",
            "Flight segment 1 elevation",
            "Flight latitude",
            "Flight longitude",
            "Flight elevation",
            "track latitude",
            "track longitude",
        ]
    );
    let first = &log.channels[0].time_s;
    assert_eq!(first[..3], [0.0, 1.0, 2.25]);
    assert!(first[3].is_nan());
    assert_eq!(first[4], 4.000_125);
    // 08:02:10 is 1 h 12 min 50 s, 4,370 s, before the zero, 09:15:00.
    assert_eq!(log.channels[3].time_s, [-4370.0, -4369.0, -4368.0]);
    assert!(log.channels[6].time_s.iter().all(|t| t.is_nan()));
    for note in [
        "times with no zone read as UTC, as GPX 1.1's schema and GPX 1.0's manual say every time \
         is: 7 of the file's \
         times state none",
        "track points with no `time` keep their places, their times gaps: 3 of 10",
        "tracks and segments with no point, left out: 0 tracks with no segment, 1 segment with \
         no point",
    ] {
        assert!(
            log.notes.iter().any(|n| n.starts_with(note)),
            "{note}\n{:#?}",
            log.notes
        );
    }
    insta::assert_json_snapshot!(log);
}
