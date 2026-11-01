//! Ghost files: a flight's logs as time-synced trajectories in a frame at the pad, written as
//! JSON, CZML (Cesium's scene format) and glTF (an animated 3D scene), for a viewer to replay
//! ([M7.2g, ghost files][m7-2g]; [ADR-313][adr-313]).
//!
//! **The frame.** East, north and up in meters from the pad, on WGS 84 ([`LaunchFrame`], which
//! [`docs/physics/geodesy.md`][geodesy] derives from NGA.STND.0036). Time is seconds from the
//! log's liftoff ([`timeline::place`]), so two logs of one flight share a clock. A log with no
//! liftoff stands alone, on its own clock, with [`Track::synced`] false.
//!
//! **What is in a track, and how far to trust it.** The height comes from the first of these a
//! log has ([`UpFrom`]): the log's own altitude; a column whose zero the log doesn't state, less
//! its reading on the pad; or its GNSS altitude less the receiver's on the pad. Not the
//! [smoother]'s: on a barometer alone it can run far from the log through a sudden swing (6,618 m
//! past the card on RocketPy's Lince flight, as its recovery fires, where the health card says
//! to go by the reading), and a file for a viewer has no card. East and north come only from GNSS fixes,
//! joined linearly between fixes; a barometer alone gives a straight vertical line, and the file
//! says so. These are a log's numbers laid out for a viewer, not a reconstruction of the flight:
//! a transonic spike smaller than the screen below stays in a barometer's height, a receiver's lag on the climb stays in
//! east and north, and the [readings][crate::readings] remain the place for an apogee.
//!
//! **Heights screened.** A height reached from the last one kept faster than [`FASTEST_M_S`], plus
//! [`HEIGHT_SCATTER_M`], is left out and counted ([`Track::heights_too_fast`]): a logger's last
//! rows can be garbage, as Juno III's 10,700 m and −2,490 m are. The first height kept is the first
//! that agrees with the middle of the log's first [`OPENING_ROWS`], so one bad first row can't shut
//! the flight out. A height more than [`BELOW_PAD_M`] under the pad is left out and counted
//! ([`Track::heights_below_pad`]): a receiver with no lock can write a height near sea level, as
//! one of Juno III's does for 3,400 rows. A moment with no height, where the track has heights, is
//! left out and counted, so a track has a height at every moment or at none; a viewer's line
//! crosses the gap. A clock past [`CLOCK_LIMIT_S`] is left out and counted, too.
//!
//! **Fixes screened.** A fix is left out, and counted ([`FixCounts`]), when it is not a place
//! (out of range, or a receiver's zero), when its clock doesn't go forward, when it lies more
//! than [`REACH_M`] from the track's median, or when reaching it from the last fix kept would take
//! more than [`FASTEST_M_S`] plus [`FIX_SCATTER_M`], the first fix kept chosen as the first height is.
//!
//! **The formats' limits.** CZML and glTF need a number at every moment, so where a track has no
//! fix yet or no longer it is held at the nearest fix; a track with no height is drawn at 0; a pad
//! of unknown place is drawn at 0°N 0°E ([`UNKNOWN_PAD`]), and of unknown height on the ellipsoid;
//! a receiver's height stands in for a height above the ellipsoid, which is off by the local
//! geoid, within about 110 m (EGM2008); each says so in its own text. CZML needs a
//! date, and logs rarely hold one, so every file uses [`CZML_EPOCH`] as liftoff, or as the first
//! moment of a log with no liftoff: the time on its clock is seconds from then, not the day of the
//! flight. glTF stores 32-bit floats and its
//! times can't be negative, so its clock starts at the earliest moment and
//! its root's `extras` give that moment; its numbers stand about 0.06 ms apart at 1,000 s, and
//! about 2 mm apart at 30 km.
//!
//! [m7-2g]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions-and-roadmap.md#m7-2g
//! [adr-313]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0313-ghost-files.md
//! [geodesy]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/physics/geodesy.md
//! [smoother]: crate::smoother

use base64::Engine as _;
use hpr_core::frames::LaunchFrame;
use hpr_core::geodesy::Geodetic;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::filter::median;
use crate::log::{Channel, FlightLog, Quantity};
use crate::readings::{Reading, Readings};
use crate::timeline;

/// The most a fix may lie from the track's median, m. A hobby or high-power flight lands well
/// inside it; a receiver's garbage, such as a longitude with its sign flipped, doesn't.
pub const REACH_M: f64 = 200_000.0;
/// The fastest a track may move between fixes, m/s: above [the envelope's Mach 3.5][envelope] at
/// sea level, about 1,190 m/s.
///
/// [envelope]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
pub const FASTEST_M_S: f64 = 1_500.0;
/// What two fixes may differ by standing still, m: a receiver's scatter with few satellites.
pub const FIX_SCATTER_M: f64 = 200.0;
/// What two heights may differ by a moment apart, m: a barometer's step at the worst of its noise.
pub const HEIGHT_SCATTER_M: f64 = 50.0;
/// How many of the last fixes before liftoff give the pad's place, by their median.
pub const PAD_FIXES: usize = 20;
/// What a CZML file calls liftoff, or a first moment; no log's date (module docs).
pub const CZML_EPOCH: &str = "2000-01-01T12:00:00Z";
/// Where a pad of unknown place is drawn, degrees and m, in CZML (module docs).
pub const UNKNOWN_PAD: (f64, f64, f64) = (0.0, 0.0, 0.0);
/// The highest or lowest a stated pad may stand, m: past any launch site, and a height a float
/// can still add to and subtract from.
pub const PAD_HEIGHT_LIMIT_M: f64 = 20_000.0;
/// The most a height may stand under the pad, m: a landing well down a valley from the pad stays
/// inside it, and a receiver's height near sea level from a pad on a plateau doesn't. Rows past it
/// are counted, and the file's text says so, so a deeper valley shows.
pub const BELOW_PAD_M: f64 = 500.0;
/// The most a clock may read, either way, s: about 317 years. A Unix clock in seconds fits; a
/// garbage row, which would overflow a date, doesn't.
pub const CLOCK_LIMIT_S: f64 = 1e10;
/// The longest a log may run from its opening, s: about 116 days. A row past it is a garbage clock.
pub const LONGEST_LOG_S: f64 = 1e7;
/// How many of a log's first usable rows find the first one to trust, by their middle value.
pub const OPENING_ROWS: usize = 9;
/// The radius of glTF's marker on each track, m: the size of a rocket seen from a field away.
const MARKER_RADIUS_M: f64 = 5.0;

/// A log to lay out, by the name a person knows it by, such as its file's name.
#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    /// Its name.
    pub name: &'a str,
    /// The log.
    pub log: &'a FlightLog,
    /// Its readings, which hold the liftoff.
    pub read: &'a Readings,
}

/// The pad's place as the caller states it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatedPad {
    /// Latitude, degrees north.
    pub latitude_deg: f64,
    /// Longitude, degrees east.
    pub longitude_deg: f64,
    /// Height above the WGS 84 ellipsoid, m; `None` to take it from the fixes, if there are any.
    pub height_m: Option<f64>,
}

/// Why a ghost can't be made.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum GhostError {
    /// No logs were given.
    #[error("no logs to lay out")]
    NoLogs,
    /// A log holds neither a height nor a usable GNSS track.
    #[error("{name}: the log holds no height and no usable GNSS track")]
    NoTrack {
        /// The log.
        name: String,
    },
    /// One of several logs has no liftoff, so nothing puts it on the others' clock.
    #[error("{name}: the log has no liftoff to share a timeline by ({reason})")]
    NotOnTimeline {
        /// The log.
        name: String,
        /// Why its liftoff was withheld.
        reason: String,
    },
    /// The stated pad isn't a place.
    #[error("the pad isn't a place: {what}")]
    BadPad {
        /// What is wrong with it.
        what: String,
    },
    /// A ghost read back from outside this crate holds a track no file can draw.
    #[error("{name}: the track can't be drawn: {what}")]
    BadTrack {
        /// The track.
        name: String,
        /// What is wrong with it.
        what: String,
    },
}

/// Where the pad's place came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PadFrom {
    /// The caller stated it.
    Stated,
    /// The median of the last fixes before liftoff.
    FixesBeforeLiftoff,
    /// The first fix: no log holds one before liftoff, or none has a liftoff. The rocket may
    /// already have left: [`Pad::first_fix_after_liftoff_s`] says by how long, where it can.
    FirstFix,
    /// No log holds a place: east and north are of nothing, and each drawn at 0.
    Unknown,
}

/// The pad: the frame's origin.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Pad {
    /// Latitude, degrees north; `None` when no log holds a place.
    pub latitude_deg: Option<f64>,
    /// Longitude, degrees east; `None` when no log holds a place.
    pub longitude_deg: Option<f64>,
    /// The pad's height, m: the caller's above the ellipsoid, or else the receiver's, most often
    /// above sea level, from fixes before liftoff or the first fix. Files draw it as if above the
    /// ellipsoid (module docs). `None` when no one states it.
    pub height_m: Option<f64>,
    /// Whether the caller stated [`Pad::height_m`], rather than a receiver.
    pub height_stated: bool,
    /// Where the place came from.
    pub from: PadFrom,
    /// For [`PadFrom::FirstFix`], how long after liftoff that fix was, s; `None` when it wasn't
    /// after, or no liftoff is known.
    pub first_fix_after_liftoff_s: Option<f64>,
}

/// Where a track's height came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UpFrom {
    /// The log's altitude as it wrote it, above the logger's zero.
    LogAltitude,
    /// A column above sea level or an unstated zero, less its own reading on the pad: the median
    /// of its last [`PAD_FIXES`] samples before liftoff, or of its first with no liftoff.
    AltitudeLessPad,
    /// The GNSS altitude less the receiver's on the pad.
    GnssAltitude,
    /// The log holds no height above the pad: each is `null`, and drawn at 0.
    Unknown,
}

/// A track's GNSS fixes, counted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FixCounts {
    /// The rows of the log's latitude and longitude columns.
    pub given: usize,
    /// Those kept.
    pub kept: usize,
    /// Left out: an empty cell, not a place, or a receiver's zero.
    pub not_a_place: usize,
    /// Left out: the clock didn't go forward.
    pub clock_back: usize,
    /// Left out: past [`REACH_M`] from the median.
    pub out_of_reach: usize,
    /// Left out: too fast from the last kept ([`FASTEST_M_S`]).
    pub too_fast: usize,
}

/// One log's trajectory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Track {
    /// The log's name, as the caller gave it.
    pub name: String,
    /// The logger the log's format names, as a person reads it.
    pub logger: String,
    /// Whether [`Track::time_s`] counts from the log's liftoff; if not, it is the log's own clock.
    pub synced: bool,
    /// The liftoff on the log's own clock, s, when one was read.
    pub liftoff_s: Option<f64>,
    /// The bound on the liftoff's error, s, when one was read. It is every moment's bound on this
    /// timeline.
    pub liftoff_bound_s: Option<f64>,
    /// Where the height came from.
    pub up_from: UpFrom,
    /// The moments kept, each later than the last, s.
    pub time_s: Vec<f64>,
    /// East of the pad at each moment, m; `null` before the first fix and after the last.
    pub east_m: Vec<Option<f64>>,
    /// North of the pad at each moment, m; `null` as for east.
    pub north_m: Vec<Option<f64>>,
    /// Height above the pad at each moment, m; every one `null` when [`UpFrom::Unknown`], none
    /// otherwise.
    pub up_m: Vec<Option<f64>>,
    /// The fixes.
    pub fixes: FixCounts,
    /// The widest time between two kept fixes, s; east and north are a straight line across it.
    pub widest_fix_gap_s: Option<f64>,
    /// The log's rows left out for a clock that didn't go forward or is past [`CLOCK_LIMIT_S`], or
    /// a height empty or not a number.
    pub moments_left_out: usize,
    /// The log's rows left out for a height no flight could reach ([`FASTEST_M_S`]).
    pub heights_too_fast: usize,
    /// The log's rows left out for a height past [`BELOW_PAD_M`] under the pad.
    pub heights_below_pad: usize,
}

/// A flight's ghost: its logs' trajectories on one clock in one frame, as the JSON writes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Ghost {
    /// `hpr-ghost`.
    pub format: String,
    /// The format's version.
    pub version: u32,
    /// The frame in words: east, north and up from the pad, in meters.
    pub frame: String,
    /// The pad.
    pub pad: Pad,
    /// Each log's trajectory.
    pub tracks: Vec<Track>,
}

/// The fixes kept from a log, and each's height where it has one.
#[derive(Debug, Default)]
struct Fixes {
    time_s: Vec<f64>,
    latitude_deg: Vec<f64>,
    longitude_deg: Vec<f64>,
    height_m: Vec<Option<f64>>,
    counts: FixCounts,
}

/// The log's first channel of a quantity.
fn channel(log: &FlightLog, quantity: Quantity) -> Option<&Channel> {
    log.channels_of(quantity).next()
}

/// The pad frame at a place; `None` if it isn't one.
fn frame_at(latitude_deg: f64, longitude_deg: f64, height_m: f64) -> Option<LaunchFrame> {
    let origin = Geodetic::from_degrees(latitude_deg, longitude_deg, height_m).ok()?;
    LaunchFrame::wgs84(origin).ok()
}

/// East and north of `frame`'s origin, m.
fn east_north(frame: &LaunchFrame, latitude_deg: f64, longitude_deg: f64) -> Option<(f64, f64)> {
    let point =
        Geodetic::from_degrees(latitude_deg, longitude_deg, frame.origin().height_m).ok()?;
    let enu = frame.enu_from_geodetic(point);
    Some((enu.x, enu.y))
}

/// Whether a latitude and longitude are a place: in range, and not a receiver's zero.
fn a_place(latitude_deg: f64, longitude_deg: f64) -> bool {
    latitude_deg.abs() <= 90.0
        && longitude_deg.abs() <= 180.0
        && !(latitude_deg.abs() < 0.01 && longitude_deg.abs() < 0.01)
}

/// How many of the first `samples`, a time and a value each, don't agree with the middle of the
/// first [`OPENING_ROWS`]: reached from it no faster than [`FASTEST_M_S`], plus `scatter_m`.
fn opening_outliers(samples: &[(f64, f64)], scatter_m: f64) -> usize {
    let mut opening = samples[..samples.len().min(OPENING_ROWS)].to_vec();
    opening.sort_by(|a, b| a.1.total_cmp(&b.1));
    let Some(&(middle_s, middle)) = opening.get(opening.len() / 2) else {
        return 0;
    };
    samples
        .iter()
        .position(|(time, value)| {
            (value - middle).abs() <= FASTEST_M_S * (time - middle_s).abs() + scatter_m
        })
        .unwrap_or(0)
}

/// A test of a clock's readings: within [`CLOCK_LIMIT_S`], and [`LONGEST_LOG_S`] of the middle of
/// the first [`OPENING_ROWS`] within it.
fn clock_test(times: &[f64]) -> impl Fn(f64) -> bool + use<> {
    let mut opening: Vec<f64> = times
        .iter()
        .copied()
        .filter(|time| time.abs() <= CLOCK_LIMIT_S)
        .take(OPENING_ROWS)
        .collect();
    let middle = median(&mut opening).unwrap_or(0.0);
    move |time: f64| time.abs() <= CLOCK_LIMIT_S && (time - middle).abs() <= LONGEST_LOG_S
}

/// The fixes of `log`, screened (module docs).
fn screened_fixes(log: &FlightLog) -> Fixes {
    let mut out = Fixes::default();
    let (Some(latitude), Some(longitude)) = (
        channel(log, Quantity::Latitude),
        channel(log, Quantity::Longitude),
    ) else {
        return out;
    };
    let rows = latitude.values.len().min(longitude.values.len());
    let height = channel(log, Quantity::GnssAltitude).filter(|c| c.values.len() == rows);
    out.counts.given = rows;
    let in_clock = clock_test(&latitude.time_s);
    let mut places = Vec::new();
    for row in 0..rows {
        let (lat, lon, time) = (
            latitude.values[row],
            longitude.values[row],
            latitude.time_s.get(row).copied().unwrap_or(f64::NAN),
        );
        if in_clock(time) && a_place(lat, lon) {
            places.push((row, time, lat, lon));
        } else {
            out.counts.not_a_place += 1;
        }
    }
    let middle = |of: fn(&(usize, f64, f64, f64)) -> f64| {
        let mut values: Vec<f64> = places.iter().map(of).collect();
        median(&mut values)
    };
    let (Some(mid_lat), Some(mid_lon)) = (middle(|p| p.2), middle(|p| p.3)) else {
        return out;
    };
    let Some(frame) = frame_at(mid_lat, mid_lon, 0.0) else {
        out.counts.not_a_place += places.len();
        return out;
    };
    let mut within = Vec::with_capacity(places.len());
    for (row, time, lat, lon) in places {
        let Some((east, north)) = east_north(&frame, lat, lon) else {
            out.counts.not_a_place += 1;
            continue;
        };
        if east.hypot(north) > REACH_M {
            out.counts.out_of_reach += 1;
            continue;
        }
        within.push((row, time, lat, lon, east, north));
    }
    let easts: Vec<(f64, f64)> = within.iter().map(|fix| (fix.1, fix.4)).collect();
    let norths: Vec<(f64, f64)> = within.iter().map(|fix| (fix.1, fix.5)).collect();
    let opening =
        opening_outliers(&easts, FIX_SCATTER_M).max(opening_outliers(&norths, FIX_SCATTER_M));
    out.counts.too_fast += opening;
    let mut last: Option<(f64, f64, f64)> = None;
    for &(row, time, lat, lon, east, north) in within.iter().skip(opening) {
        if let Some((t, e, n)) = last {
            if time <= t {
                out.counts.clock_back += 1;
                continue;
            }
            if (east - e).hypot(north - n) > FASTEST_M_S * (time - t) + FIX_SCATTER_M {
                out.counts.too_fast += 1;
                continue;
            }
        }
        last = Some((time, east, north));
        out.time_s.push(time);
        out.latitude_deg.push(lat);
        out.longitude_deg.push(lon);
        out.height_m.push(
            height
                .map(|c| c.values[row])
                .filter(|h| h.is_finite() && *h != 0.0),
        );
        out.counts.kept += 1;
    }
    out
}

/// The liftoff of a log's readings, and its bound.
fn liftoff_of(read: &Readings) -> Option<(f64, f64)> {
    match timeline::place(read) {
        Reading::Read(placed) => Some((placed.zero_s, placed.bound_s)),
        Reading::Withheld(_) => None,
    }
}

/// The pad from what the caller stated and each log's fixes.
fn pad_of(
    stated: Option<StatedPad>,
    logs: &[(Option<(f64, f64)>, Fixes)],
) -> Result<(Pad, Option<f64>), GhostError> {
    // The first log with fixes before liftoff, or else the first with any.
    let before = logs.iter().find_map(|(liftoff, fixes)| {
        let (zero_s, _) = (*liftoff)?;
        let end = fixes.time_s.iter().take_while(|t| **t < zero_s).count();
        (end > 0).then_some((fixes, end.saturating_sub(PAD_FIXES)..end))
    });
    let from_fixes = before.as_ref().map(|(fixes, range)| {
        let mut lat = fixes.latitude_deg[range.clone()].to_vec();
        let mut lon = fixes.longitude_deg[range.clone()].to_vec();
        let mut height: Vec<f64> = fixes.height_m[range.clone()]
            .iter()
            .flatten()
            .copied()
            .collect();
        (median(&mut lat), median(&mut lon), median(&mut height))
    });
    let first = logs
        .iter()
        .find(|(_, fixes)| !fixes.time_s.is_empty())
        .map(|(liftoff, fixes)| {
            let late = liftoff
                .map(|(zero_s, _)| fixes.time_s[0] - zero_s)
                .filter(|late| *late >= 0.0);
            (fixes, late)
        });
    // The height the fixes give the pad: from before liftoff, or the first fix's own unless it was
    // after liftoff. Never a later fix's, which may be in flight.
    let height_of_fixes = from_fixes
        .and_then(|(_, _, height)| height)
        .or_else(|| match first {
            Some((fixes, None)) if before.is_none() => fixes.height_m[0],
            _ => None,
        });
    if let Some(pad) = stated {
        if !(pad.latitude_deg.abs() <= 90.0 && pad.longitude_deg.abs() <= 180.0) {
            return Err(GhostError::BadPad {
                what: format!(
                    "latitude {} and longitude {} are out of range",
                    pad.latitude_deg, pad.longitude_deg
                ),
            });
        }
        if pad
            .height_m
            .is_some_and(|h| h.is_nan() || h.abs() > PAD_HEIGHT_LIMIT_M)
        {
            return Err(GhostError::BadPad {
                what: format!("its height is past {PAD_HEIGHT_LIMIT_M} m, or not a number"),
            });
        }
        let height_m = pad.height_m.or(height_of_fixes);
        return Ok((
            Pad {
                latitude_deg: Some(pad.latitude_deg),
                longitude_deg: Some(pad.longitude_deg),
                height_m,
                height_stated: pad.height_m.is_some(),
                from: PadFrom::Stated,
                first_fix_after_liftoff_s: None,
            },
            // The receiver's own reading on the pad is the zero for its heights, whose datum may
            // not be the stated height's.
            height_of_fixes.or(height_m),
        ));
    }
    if let Some((Some(lat), Some(lon), _)) = from_fixes {
        return Ok((
            Pad {
                latitude_deg: Some(lat),
                longitude_deg: Some(lon),
                height_m: height_of_fixes,
                height_stated: false,
                from: PadFrom::FixesBeforeLiftoff,
                first_fix_after_liftoff_s: None,
            },
            height_of_fixes,
        ));
    }
    if let Some((fixes, late)) = first {
        return Ok((
            Pad {
                latitude_deg: Some(fixes.latitude_deg[0]),
                longitude_deg: Some(fixes.longitude_deg[0]),
                height_m: height_of_fixes,
                height_stated: false,
                from: PadFrom::FirstFix,
                first_fix_after_liftoff_s: late,
            },
            height_of_fixes,
        ));
    }
    Ok((
        Pad {
            latitude_deg: None,
            longitude_deg: None,
            height_m: None,
            height_stated: false,
            from: PadFrom::Unknown,
            first_fix_after_liftoff_s: None,
        },
        None,
    ))
}

/// The height's moments and values, on the log's clock.
struct Heights {
    from: UpFrom,
    time_s: Vec<f64>,
    up_m: Vec<Option<f64>>,
}

/// A column's own reading on the pad: the median of its last [`PAD_FIXES`] samples before liftoff,
/// or, with no liftoff or none before it, of its first.
fn column_pad(column: &Channel, liftoff: Option<(f64, f64)>) -> Option<f64> {
    let usable = || {
        column
            .time_s
            .iter()
            .zip(&column.values)
            .filter(|(time, value)| time.is_finite() && value.is_finite())
    };
    let before: Vec<f64> = liftoff.map_or_else(Vec::new, |(zero_s, _)| {
        usable()
            .filter(|(time, _)| **time < zero_s)
            .map(|(_, value)| *value)
            .collect()
    });
    let mut values = if before.is_empty() {
        usable().take(PAD_FIXES).map(|(_, value)| *value).collect()
    } else {
        before[before.len().saturating_sub(PAD_FIXES)..].to_vec()
    };
    median(&mut values)
}

/// The first usable height of `log`, by the order of [`UpFrom`]. `gnss_zero_m` is the receiver's
/// reading on the pad.
fn heights_of(
    log: &FlightLog,
    fixes: &Fixes,
    liftoff: Option<(f64, f64)>,
    gnss_zero_m: Option<f64>,
) -> Option<Heights> {
    // A height above ground needs nothing taken off. Without one, a column of another zero does,
    // whichever of the log's fields holds it, so it comes first.
    let above_ground = channel(log, Quantity::AltitudeAboveGround);
    let other_zero = channel(log, Quantity::Altitude).or(channel(log, Quantity::AltitudeMsl));
    if let (None, Some(column)) = (above_ground, other_zero)
        && let Some(pad) = column_pad(column, liftoff)
    {
        return Some(Heights {
            from: UpFrom::AltitudeLessPad,
            time_s: column.time_s.clone(),
            up_m: column.values.iter().map(|h| Some(*h - pad)).collect(),
        });
    }
    if !log.altitude_m.is_empty() && log.altitude_m.len() == log.time_s.len() {
        return Some(Heights {
            from: UpFrom::LogAltitude,
            time_s: log.time_s.clone(),
            up_m: log.altitude_m.iter().map(|h| Some(*h)).collect(),
        });
    }
    if let Some(column) = above_ground {
        return Some(Heights {
            from: UpFrom::LogAltitude,
            time_s: column.time_s.clone(),
            up_m: column.values.iter().map(|h| Some(*h)).collect(),
        });
    }
    if fixes.time_s.is_empty() {
        return None;
    }
    let zero = gnss_zero_m.filter(|_| fixes.height_m.iter().any(Option::is_some));
    Some(Heights {
        from: if zero.is_some() {
            UpFrom::GnssAltitude
        } else {
            UpFrom::Unknown
        },
        time_s: fixes.time_s.clone(),
        up_m: fixes
            .height_m
            .iter()
            .map(|h| zero.and_then(|zero| h.map(|h| h - zero)))
            .collect(),
    })
}

/// `values` at `at`, a straight line between the two `times` either side; `None` outside them.
/// `times` go forward, so the search is a walk that `cursor` keeps.
fn join(times: &[f64], values: &[f64], at: f64, cursor: &mut usize) -> Option<f64> {
    let (&first, &last) = (times.first()?, times.last()?);
    if at < first || at > last {
        return None;
    }
    while *cursor + 1 < times.len() && times[*cursor + 1] < at {
        *cursor += 1;
    }
    let (a, b) = (*cursor, (*cursor + 1).min(times.len() - 1));
    if times[b] <= times[a] {
        return Some(values[a]);
    }
    let weight = ((at - times[a]) / (times[b] - times[a])).clamp(0.0, 1.0);
    Some(values[a] + weight * (values[b] - values[a]))
}

/// Lays one log out in the pad's frame.
fn track_of(
    input: Input<'_>,
    liftoff: Option<(f64, f64)>,
    fixes: &Fixes,
    pad: &Pad,
    gnss_zero_m: Option<f64>,
) -> Result<Track, GhostError> {
    let heights =
        heights_of(input.log, fixes, liftoff, gnss_zero_m).ok_or_else(|| GhostError::NoTrack {
            name: input.name.to_owned(),
        })?;
    let frame = match (pad.latitude_deg, pad.longitude_deg) {
        (Some(lat), Some(lon)) => frame_at(lat, lon, 0.0),
        _ => None,
    };
    let (fix_east, fix_north): (Vec<f64>, Vec<f64>) = match &frame {
        Some(frame) => fixes
            .latitude_deg
            .iter()
            .zip(&fixes.longitude_deg)
            .map(|(lat, lon)| east_north(frame, *lat, *lon).unwrap_or((f64::NAN, f64::NAN)))
            .unzip(),
        None => Default::default(),
    };
    let zero_s = liftoff.map_or(0.0, |(zero_s, _)| zero_s);
    let mut track = Track {
        name: input.name.to_owned(),
        logger: input.log.logger.clone(),
        synced: liftoff.is_some(),
        liftoff_s: liftoff.map(|(zero_s, _)| zero_s),
        liftoff_bound_s: liftoff.map(|(_, bound_s)| bound_s),
        up_from: heights.from,
        time_s: Vec::with_capacity(heights.time_s.len()),
        east_m: Vec::with_capacity(heights.time_s.len()),
        north_m: Vec::with_capacity(heights.time_s.len()),
        up_m: Vec::with_capacity(heights.time_s.len()),
        fixes: fixes.counts,
        widest_fix_gap_s: fixes
            .time_s
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .reduce(f64::max),
        moments_left_out: 0,
        heights_too_fast: 0,
        heights_below_pad: 0,
    };
    let in_clock = clock_test(&heights.time_s);
    let above_floor = |height: f64| height >= -BELOW_PAD_M;
    // The rows that could be the first height kept, and which of them is.
    let candidates: Vec<(usize, (f64, f64))> = heights
        .time_s
        .iter()
        .zip(&heights.up_m)
        .enumerate()
        .filter_map(|(row, (&time, &up))| {
            let height = up.filter(|h| h.is_finite() && above_floor(*h))?;
            in_clock(time).then_some((row, (time, height)))
        })
        .collect();
    let samples: Vec<(f64, f64)> = candidates.iter().map(|(_, sample)| *sample).collect();
    let first_row = candidates
        .get(opening_outliers(&samples, HEIGHT_SCATTER_M))
        .map_or(0, |(row, _)| *row);
    let has_heights = heights.from != UpFrom::Unknown;
    let (mut east_cursor, mut north_cursor) = (0, 0);
    let mut last = f64::NEG_INFINITY;
    let mut last_up: Option<(f64, f64)> = None;
    for (row, (&time, &up)) in heights.time_s.iter().zip(&heights.up_m).enumerate() {
        if !in_clock(time)
            || time <= last
            || up.is_some_and(|h| !h.is_finite())
            || (has_heights && up.is_none())
        {
            track.moments_left_out += 1;
            continue;
        }
        if up.is_some_and(|height| !above_floor(height)) {
            track.heights_below_pad += 1;
            continue;
        }
        if row < first_row {
            track.heights_too_fast += 1;
            continue;
        }
        if let (Some(height), Some((t, held))) = (up, last_up)
            && (height - held).abs() > FASTEST_M_S * (time - t) + HEIGHT_SCATTER_M
        {
            track.heights_too_fast += 1;
            continue;
        }
        last = time;
        if let Some(height) = up {
            last_up = Some((time, height));
        }
        track.time_s.push(time - zero_s);
        track
            .east_m
            .push(join(&fixes.time_s, &fix_east, time, &mut east_cursor));
        track
            .north_m
            .push(join(&fixes.time_s, &fix_north, time, &mut north_cursor));
        track.up_m.push(up);
    }
    if track.time_s.is_empty() {
        return Err(GhostError::NoTrack {
            name: input.name.to_owned(),
        });
    }
    Ok(track)
}

/// Lays `logs` out as one flight's ghost. With more than one log, each needs a liftoff.
pub fn build(logs: &[Input<'_>], pad: Option<StatedPad>) -> Result<Ghost, GhostError> {
    if logs.is_empty() {
        return Err(GhostError::NoLogs);
    }
    let mut placed = Vec::with_capacity(logs.len());
    for input in logs {
        let liftoff = liftoff_of(input.read);
        if liftoff.is_none()
            && logs.len() > 1
            && let Reading::Withheld(withheld) = &input.read.liftoff
        {
            return Err(GhostError::NotOnTimeline {
                name: input.name.to_owned(),
                reason: withheld.detail.clone(),
            });
        }
        placed.push((liftoff, screened_fixes(input.log)));
    }
    let (pad, gnss_zero_m) = pad_of(pad, &placed)?;
    let tracks = logs
        .iter()
        .zip(&placed)
        .map(|(input, (liftoff, fixes))| track_of(*input, *liftoff, fixes, &pad, gnss_zero_m))
        .collect::<Result<Vec<Track>, GhostError>>()?;
    Ok(Ghost {
        format: "hpr-ghost".to_owned(),
        version: 1,
        frame: "east, north and up from the pad, m, on WGS 84; time from liftoff, s".to_owned(),
        pad,
        tracks,
    })
}

/// A track's moments as CZML and glTF draw them: every number known (module docs).
fn drawn(track: &Track) -> Vec<[f64; 4]> {
    let nearest = |values: &[Option<f64>]| -> Vec<f64> {
        let mut filled: Vec<f64> = Vec::with_capacity(values.len());
        let mut held = values.iter().flatten().next().copied().unwrap_or(0.0);
        for value in values {
            held = value.unwrap_or(held);
            filled.push(held);
        }
        filled
    };
    let (east, north) = (nearest(&track.east_m), nearest(&track.north_m));
    track
        .time_s
        .iter()
        .enumerate()
        .map(|(i, time)| [*time, east[i], north[i], track.up_m[i].unwrap_or(0.0)])
        .collect()
}

/// What each drawing must say it has assumed, in words.
fn assumed(ghost: &Ghost, track: &Track) -> String {
    let mut words = vec![format!(
        "{}: seconds from {}; east, north and up from the pad.",
        track.name,
        if track.synced {
            "liftoff, by the log's reading"
        } else {
            "the log's own zero: it has no liftoff"
        }
    )];
    if ghost.pad.from == PadFrom::Unknown {
        words.push("The pad's place isn't known: it is drawn at 0N 0E.".to_owned());
    } else if ghost.pad.height_m.is_none() {
        words.push(
            "The pad's height isn't known: the path starts on the WGS 84 ellipsoid, which can be \
             far under or over the ground."
                .to_owned(),
        );
    } else if !ghost.pad.height_stated {
        words.push(
            "The pad's height is the receiver's, drawn as if above the ellipsoid: off by the \
             local geoid, within about 110 m, if it states sea level."
                .to_owned(),
        );
    }
    if ghost.pad.from == PadFrom::FirstFix {
        words.push("The pad is the track's first fix.".to_owned());
    }
    if track.fixes.kept == 0 {
        words.push("The log has no GNSS track: a straight vertical line.".to_owned());
    } else if track.east_m.iter().any(Option::is_none) {
        words.push("Outside the fixes it is held at the nearest.".to_owned());
    }
    if track.up_from == UpFrom::Unknown {
        words.push("The log holds no height above the pad: drawn at 0.".to_owned());
    }
    if track.up_from == UpFrom::AltitudeLessPad {
        words.push(
            "The log's altitude states no zero at the pad: taken less its own reading there, or \
             its first rows' with no liftoff."
                .to_owned(),
        );
    }
    let left_out = track.moments_left_out + track.heights_too_fast + track.heights_below_pad;
    if left_out > 0 {
        words.push(format!(
            "{left_out} of the log's rows are left out: a line crosses each gap."
        ));
    }
    words.join(" ")
}

impl Ghost {
    /// Why no file can draw this ghost, if so: it holds what [`build`] never writes. A ghost is
    /// public data, and JSON read back can hold anything.
    fn check(&self) -> Result<(), GhostError> {
        let bad = |name: &str, what: &str| GhostError::BadTrack {
            name: name.to_owned(),
            what: what.to_owned(),
        };
        if self.tracks.is_empty() {
            return Err(GhostError::NoLogs);
        }
        if let (Some(lat), Some(lon)) = (self.pad.latitude_deg, self.pad.longitude_deg)
            && frame_at(lat, lon, self.pad.height_m.unwrap_or(0.0)).is_none()
        {
            return Err(GhostError::BadPad {
                what: format!("latitude {lat} and longitude {lon}"),
            });
        }
        for track in &self.tracks {
            let moments = track.time_s.len();
            if moments == 0 {
                return Err(bad(&track.name, "it has no moments"));
            }
            if track.east_m.len() != moments
                || track.north_m.len() != moments
                || track.up_m.len() != moments
            {
                return Err(bad(&track.name, "its columns differ in length"));
            }
            if track
                .time_s
                .iter()
                .any(|t| t.is_nan() || t.abs() > CLOCK_LIMIT_S)
                || track.time_s.windows(2).any(|pair| pair[1] <= pair[0])
            {
                return Err(bad(
                    &track.name,
                    "its clock doesn't go forward inside its limit",
                ));
            }
            let finite = |values: &[Option<f64>]| values.iter().flatten().all(|v| v.is_finite());
            if !finite(&track.east_m) || !finite(&track.north_m) || !finite(&track.up_m) {
                return Err(bad(&track.name, "a place isn't a number"));
            }
        }
        Ok(())
    }

    /// The moment of [`CZML_EPOCH`] plus `seconds`, written with whole milliseconds; `up` rounds
    /// forward and otherwise back, so that a span holds every moment. [`Ghost::check`] holds
    /// `seconds` to [`CLOCK_LIMIT_S`], so the milliseconds fit.
    #[allow(
        clippy::cast_possible_truncation,
        reason = "under 1e14 milliseconds, by the clock's limit: far inside i64"
    )]
    fn czml_time(seconds: f64, up: bool) -> String {
        let scaled = seconds * 1000.0;
        // A millisecond past the rounding: a reader's sums over a date mustn't land outside.
        let ms = (if up {
            scaled.ceil() + 1.0
        } else {
            scaled.floor() - 1.0
        }) as i64
            + 12 * 3_600_000;
        let (days, ms) = (ms.div_euclid(86_400_000), ms.rem_euclid(86_400_000));
        let (year, month, day) = date_from_days(days);
        format!(
            "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
            ms / 3_600_000,
            ms / 60_000 % 60,
            ms / 1000 % 60,
            ms % 1000
        )
    }

    /// What a CZML file's epoch stands for on the tracks' clock.
    fn czml_zero_s(&self) -> f64 {
        if self.tracks.iter().any(|track| track.synced) {
            return 0.0;
        }
        self.tracks
            .iter()
            .filter_map(|track| track.time_s.first())
            .copied()
            .fold(f64::INFINITY, f64::min)
    }

    /// The ghost as a CZML document: a position for each track on [`CZML_EPOCH`], Earth-fixed.
    ///
    /// # Errors
    ///
    /// When the ghost holds what no file can draw, as JSON from elsewhere can.
    pub fn to_czml(&self) -> Result<String, GhostError> {
        self.check()?;
        let (lat, lon, height) = (
            self.pad.latitude_deg.unwrap_or(UNKNOWN_PAD.0),
            self.pad.longitude_deg.unwrap_or(UNKNOWN_PAD.1),
            self.pad.height_m.unwrap_or(UNKNOWN_PAD.2),
        );
        let frame = frame_at(lat, lon, height);
        // Liftoff at the epoch, or with none the first moment: a date far from it would cross leap
        // seconds, which a date in the file counts and seconds from the epoch don't.
        let zero_s = self.czml_zero_s();
        let span =
            self.tracks
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(start, end), track| {
                    (
                        start.min(track.time_s.first().copied().unwrap_or(0.0) - zero_s),
                        end.max(track.time_s.last().copied().unwrap_or(0.0) - zero_s),
                    )
                });
        let (start, end) = (
            Self::czml_time(span.0, false),
            Self::czml_time(span.1, true),
        );
        let mut packets = vec![json!({
            "id": "document",
            "name": "FusionSpace HPR ghost",
            "version": "1.0",
            "clock": {
                "interval": format!("{start}/{end}"),
                "currentTime": start,
                "multiplier": 1,
            },
        })];
        for (index, track) in self.tracks.iter().enumerate() {
            let mut cartesian = Vec::with_capacity(track.time_s.len() * 4);
            for [time, east, north, up] in drawn(track) {
                let at = frame
                    .as_ref()
                    .map_or(hpr_core::DVec3::ZERO, |frame: &LaunchFrame| {
                        frame.ecef_from_enu(hpr_core::DVec3::new(east, north, up))
                    });
                cartesian.extend([time - zero_s, at.x, at.y, at.z]);
            }
            let color = TRACK_COLORS[index % TRACK_COLORS.len()];
            packets.push(json!({
                "id": format!("track-{}", index + 1),
                "name": track.name,
                "description": escaped(&assumed(self, track)),
                "availability": format!(
                    "{}/{}",
                    Self::czml_time(track.time_s[0] - zero_s, false),
                    Self::czml_time(track.time_s[track.time_s.len() - 1] - zero_s, true),
                ),
                "point": {"pixelSize": 8, "color": {"rgba": color}},
                "path": {"width": 2, "material": {"solidColor": {"color": {"rgba": color}}}},
                "position": {
                    "epoch": CZML_EPOCH,
                    "referenceFrame": "FIXED",
                    "interpolationAlgorithm": "LINEAR",
                    "interpolationDegree": 1,
                    "cartesian": cartesian,
                },
            }));
        }
        Ok(Value::Array(packets).to_string())
    }

    /// The ghost as one glTF 2.0 file, its buffer inside as base64: for each track, a line of its
    /// whole path and a small marker that moves along it. +x is east, +y up, −z north, in meters,
    /// as glTF's axes run.
    #[allow(
        clippy::cast_possible_truncation,
        reason = "glTF holds 32-bit floats: the module docs size the loss"
    )]
    ///
    /// # Errors
    ///
    /// As [`Ghost::to_czml`].
    pub fn to_gltf(&self) -> Result<String, GhostError> {
        self.check()?;
        let mut buffer = Bytes::default();
        let mut accessors = Vec::new();
        let mut meshes = Vec::new();
        let mut nodes = Vec::new();
        let mut samplers = Vec::new();
        let mut channels = Vec::new();
        let mut materials = Vec::new();
        let t0 = self
            .tracks
            .iter()
            .filter_map(|track| track.time_s.first())
            .copied()
            .fold(f64::INFINITY, f64::min);
        // The marker: an octahedron, the fewest faces that read as a point from any side.
        let marker_points = {
            let r = MARKER_RADIUS_M as f32;
            [
                [r, 0.0, 0.0],
                [-r, 0.0, 0.0],
                [0.0, r, 0.0],
                [0.0, -r, 0.0],
                [0.0, 0.0, r],
                [0.0, 0.0, -r],
            ]
        };
        let marker_position = buffer.vec3(&marker_points, &mut accessors);
        let marker_position = buffer.same_bytes_for_a_mesh(marker_position, &mut accessors);
        let marker_indices = buffer.indices(
            &[
                0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 4, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3, 5,
            ],
            &mut accessors,
        );
        for (index, track) in self.tracks.iter().enumerate() {
            let mut times: Vec<f32> = Vec::with_capacity(track.time_s.len());
            let mut places: Vec<[f32; 3]> = Vec::with_capacity(track.time_s.len());
            for [time, east, north, up] in drawn(track) {
                // Rounding can make two moments one, which glTF refuses: the first stays.
                let time = (time - t0) as f32;
                if times.last().is_none_or(|last| time > *last) {
                    times.push(time);
                    places.push([east as f32, up as f32, -north as f32]);
                }
            }
            let input = buffer.scalars(&times, &mut accessors);
            let output = buffer.vec3(&places, &mut accessors);
            // An accessor and a view can't be both an animation's and a mesh's: two of each over
            // the same bytes.
            let line = buffer.same_bytes_for_a_mesh(output, &mut accessors);
            let material = materials.len();
            let color = TRACK_COLORS[index % TRACK_COLORS.len()];
            materials.push(json!({
                "name": track.name,
                "pbrMetallicRoughness": {
                    "baseColorFactor": [
                        f64::from(color[0]) / 255.0,
                        f64::from(color[1]) / 255.0,
                        f64::from(color[2]) / 255.0,
                        1.0,
                    ],
                },
            }));
            meshes.push(json!({
                "name": format!("{} path", track.name),
                "primitives": [{"attributes": {"POSITION": line}, "mode": 3, "material": material}],
            }));
            meshes.push(json!({
                "name": format!("{} marker", track.name),
                "primitives": [{
                    "attributes": {"POSITION": marker_position},
                    "indices": marker_indices,
                    "material": material,
                }],
            }));
            nodes.push(json!({
                "name": format!("{} path", track.name),
                "mesh": meshes.len() - 2,
                "extras": {"description": assumed(self, track)},
            }));
            let moving = nodes.len();
            nodes.push(json!({
                "name": format!("{} marker", track.name),
                "mesh": meshes.len() - 1,
                "translation": places[0],
            }));
            channels.push(json!({
                "sampler": samplers.len(),
                "target": {"node": moving, "path": "translation"},
            }));
            samplers.push(json!({"input": input, "output": output, "interpolation": "LINEAR"}));
        }
        let scene: Vec<usize> = (0..nodes.len()).collect();
        let base64 = base64::engine::general_purpose::STANDARD.encode(&buffer.bytes);
        Ok(json!({
            "asset": {"version": "2.0", "generator": "FusionSpace HPR"},
            "extras": {
                "first_moment_s": t0,
                "frame": "+x east, +y up, -z north, m, from the pad",
                "pad": self.pad,
            },
            "scene": 0,
            "scenes": [{"nodes": scene}],
            "nodes": nodes,
            "meshes": meshes,
            "materials": materials,
            "animations": [{"name": "flight", "channels": channels, "samplers": samplers}],
            "accessors": accessors,
            "bufferViews": buffer.views,
            "buffers": [{
                "byteLength": buffer.bytes.len(),
                "uri": format!("data:application/octet-stream;base64,{base64}"),
            }],
        })
        .to_string())
    }
}

/// The civil date `days` after 1 January 2000: year, month, day. The proleptic Gregorian calendar,
/// by Hinnant's `civil_from_days` (*chrono-Compatible Low-Level Date Algorithms*).
fn date_from_days(days: i64) -> (i64, i64, i64) {
    // Counted in eras of 400 years from 1 March 2000, the start of one.
    let z = days - 60;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = 2000 + year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// Text as CZML's description holds it: a viewer shows that as HTML, and a file's name isn't.
fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The tracks' colors, red, green, blue and alpha, from 0 to 255: the first bright, the rest
/// paler, so that one log reads as the flight and the others as its ghosts.
const TRACK_COLORS: [[u8; 4]; 4] = [
    [255, 122, 26, 255],
    [120, 200, 255, 255],
    [190, 240, 150, 255],
    [255, 220, 120, 255],
];

/// glTF's numbers for what a view holds: a mesh's vertices, and its corners.
const ARRAY_BUFFER: u32 = 34962;
const ELEMENT_ARRAY_BUFFER: u32 = 34963;

/// glTF's binary buffer as it is built, and its views.
#[derive(Default)]
struct Bytes {
    bytes: Vec<u8>,
    views: Vec<Value>,
}

impl Bytes {
    /// A new accessor and view over the bytes of `accessor`, marked as a mesh's vertices.
    fn same_bytes_for_a_mesh(&mut self, accessor: usize, accessors: &mut Vec<Value>) -> usize {
        let mut copy = accessors[accessor].clone();
        let view = copy["bufferView"]
            .as_u64()
            .and_then(|view| usize::try_from(view).ok())
            .unwrap_or(0);
        let mut own = self.views[view].clone();
        own["target"] = json!(ARRAY_BUFFER);
        self.views.push(own);
        copy["bufferView"] = json!(self.views.len() - 1);
        accessors.push(copy);
        accessors.len() - 1
    }

    /// Starts a view at a multiple of 4, as every accessor here needs.
    fn view(&mut self, data: &[u8]) -> usize {
        while !self.bytes.len().is_multiple_of(4) {
            self.bytes.push(0);
        }
        self.views.push(json!({
            "buffer": 0,
            "byteOffset": self.bytes.len(),
            "byteLength": data.len(),
        }));
        self.bytes.extend_from_slice(data);
        self.views.len() - 1
    }

    /// One float a moment, with the bounds glTF requires of an animation's times.
    fn scalars(&mut self, values: &[f32], accessors: &mut Vec<Value>) -> usize {
        let data: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        let view = self.view(&data);
        let (least, most) = values
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), v| {
                (a.min(*v), b.max(*v))
            });
        accessors.push(json!({
            "bufferView": view,
            "componentType": 5126,
            "count": values.len(),
            "type": "SCALAR",
            "min": [least],
            "max": [most],
        }));
        accessors.len() - 1
    }

    /// Three floats a moment, with the bounds glTF requires of a position.
    fn vec3(&mut self, values: &[[f32; 3]], accessors: &mut Vec<Value>) -> usize {
        let data: Vec<u8> = values
            .iter()
            .flatten()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let view = self.view(&data);
        let bound = |pick: fn(f32, f32) -> f32, start: f32| -> [f32; 3] {
            values.iter().fold([start; 3], |held, v| {
                [
                    pick(held[0], v[0]),
                    pick(held[1], v[1]),
                    pick(held[2], v[2]),
                ]
            })
        };
        accessors.push(json!({
            "bufferView": view,
            "componentType": 5126,
            "count": values.len(),
            "type": "VEC3",
            "min": bound(f32::min, f32::INFINITY),
            "max": bound(f32::max, f32::NEG_INFINITY),
        }));
        accessors.len() - 1
    }

    /// A triangle list's corners.
    fn indices(&mut self, values: &[u16], accessors: &mut Vec<Value>) -> usize {
        let data: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        let view = self.view(&data);
        self.views[view]["target"] = json!(ELEMENT_ARRAY_BUFFER);
        accessors.push(json!({
            "bufferView": view,
            "componentType": 5123,
            "count": values.len(),
            "type": "SCALAR",
        }));
        accessors.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import;
    use crate::readings;

    /// A table as its own reader reads it, with a pad, a climb and a descent: liftoff at 10 s.
    fn climb() -> FlightLog {
        let mut text = String::from("Time (s),Barometric altitude AGL (m)\n");
        for row in 0..400 {
            let t = f64::from(row) * 0.1;
            let up = if t < 10.0 {
                0.0
            } else if t < 13.0 {
                0.5 * 60.0 * (t - 10.0).powi(2)
            } else {
                270.0 + 180.0 * (t - 13.0) - 4.9 * (t - 13.0).powi(2)
            };
            text.push_str(&format!("{t:.2},{:.2}\n", up.max(0.0)));
        }
        import::read_one(text.as_bytes(), None, None).unwrap()
    }

    /// A log holding nothing to draw.
    fn no_height() -> FlightLog {
        let mut log = climb();
        log.time_s.clear();
        log.altitude_m.clear();
        log.channels.clear();
        log
    }

    /// Adds a latitude and a longitude column, with their clock and any GNSS altitude.
    fn with_fixes(mut log: FlightLog, fixes: &[(f64, f64, f64)], height: Option<f64>) -> FlightLog {
        let time: Vec<f64> = fixes.iter().map(|f| f.0).collect();
        let mut add = |quantity, values: Vec<f64>| {
            log.channels.push(Channel {
                column: format!("{quantity:?}"),
                quantity,
                time_s: time.clone(),
                values,
            });
        };
        add(Quantity::Latitude, fixes.iter().map(|f| f.1).collect());
        add(Quantity::Longitude, fixes.iter().map(|f| f.2).collect());
        if let Some(height) = height {
            add(Quantity::GnssAltitude, vec![height; fixes.len()]);
        }
        log
    }

    fn built(log: &FlightLog, pad: Option<StatedPad>) -> Result<Ghost, GhostError> {
        let read = readings::read(log);
        build(
            &[Input {
                name: "a.csv",
                log,
                read: &read,
            }],
            pad,
        )
    }

    fn little_ghost(east: Option<f64>, up: Option<f64>) -> Ghost {
        Ghost {
            format: "hpr-ghost".to_owned(),
            version: 1,
            frame: String::new(),
            pad: Pad {
                latitude_deg: Some(0.0),
                longitude_deg: Some(0.0),
                height_m: Some(0.0),
                height_stated: true,
                from: PadFrom::Stated,
                first_fix_after_liftoff_s: None,
            },
            tracks: vec![Track {
                name: "a.csv".to_owned(),
                logger: "a".to_owned(),
                synced: true,
                liftoff_s: Some(0.0),
                liftoff_bound_s: Some(1.0),
                up_from: UpFrom::LogAltitude,
                time_s: vec![0.0, 1.0],
                east_m: vec![east, Some(1.0)],
                north_m: vec![Some(2.0), Some(2.0)],
                up_m: vec![up, Some(3.0)],
                fixes: FixCounts::default(),
                widest_fix_gap_s: None,
                moments_left_out: 0,
                heights_too_fast: 0,
                heights_below_pad: 0,
            }],
        }
    }

    #[test]
    fn time_counts_from_liftoff_and_a_barometer_alone_is_a_vertical_line() {
        let log = climb();
        let ghost = built(&log, None).unwrap();
        let track = &ghost.tracks[0];
        let zero = track.liftoff_s.unwrap();
        assert!((9.0..11.0).contains(&zero), "{zero}");
        assert!(track.synced);
        let column = channel(&log, Quantity::AltitudeAboveGround).unwrap();
        assert_eq!(track.time_s[0], column.time_s[0] - zero);
        assert_eq!(track.up_from, UpFrom::LogAltitude);
        assert_eq!(ghost.pad.from, PadFrom::Unknown);
        assert!(track.east_m.iter().all(Option::is_none));
        assert!(track.north_m.iter().all(Option::is_none));
        // The height is the log's, not the smoother's.
        let i = column.values.iter().position(|h| *h > 1_000.0).unwrap();
        assert_eq!(track.up_m[i], Some(column.values[i]));
    }

    #[test]
    fn east_and_north_match_the_ellipsoids_radii_of_curvature() {
        // Small steps from a pad at latitude φ: north = M(φ)·Δφ and east = N(φ)·cos φ·Δλ, with M
        // the meridian radius and N the prime vertical's, a(1−e²)/(1−e²sin²φ)^{3/2} and
        // a/(1−e²sin²φ)^{1/2}; a route independent of the ECEF rotation under test. At 39.39°N:
        // M = 6,361,145.5 m and N = 6,386,751.8 m, from a = 6,378,137 and e² = 0.00669438.
        let (lat, lon, step) = (39.39_f64, -8.29, 1e-3);
        let fixes = [
            (5.0, lat, lon),
            (9.0, lat, lon),
            (20.0, lat + step, lon + step),
        ];
        let ghost = built(&with_fixes(climb(), &fixes, None), None).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::FixesBeforeLiftoff);
        assert_eq!(ghost.pad.latitude_deg, Some(lat));
        let track = &ghost.tracks[0];
        let zero = track.liftoff_s.unwrap();
        let i = track
            .time_s
            .iter()
            .position(|t| (t + zero - 20.0).abs() < 1e-9)
            .unwrap();
        let (m, n) = (6_361_145.5_f64, 6_386_751.8_f64);
        let north = m * step.to_radians();
        let east = n * lat.to_radians().cos() * step.to_radians();
        assert!((track.north_m[i].unwrap() - north).abs() < 0.005, "{north}");
        assert!((track.east_m[i].unwrap() - east).abs() < 0.005, "{east}");
        // Past the last fix, nothing is claimed; halfway across a gap, the straight line.
        assert_eq!(*track.east_m.last().unwrap(), None);
        let j = track
            .time_s
            .iter()
            .position(|t| (t + zero - 14.5).abs() < 1e-9)
            .unwrap();
        assert!((track.north_m[j].unwrap() - north / 2.0).abs() < 0.005);
        assert_eq!(track.widest_fix_gap_s, Some(11.0));
    }

    #[test]
    fn the_pad_is_the_median_of_the_last_fixes_before_liftoff() {
        // A receiver settling: 10 fixes 0.01° off, then 20 at the pad, one of them 100 m off. The
        // first or a mean of all would miss; a fix after liftoff has no say.
        let (lat, lon) = (39.39, -8.29);
        let mut fixes: Vec<(f64, f64, f64)> = (0..10)
            .map(|k| (0.1 * f64::from(k), lat + 0.01, lon))
            .collect();
        fixes.extend((10..30).map(|k| (0.1 * f64::from(k), lat, lon)));
        fixes[20].1 = lat + 0.001;
        fixes.push((20.0, lat + 0.005, lon));
        let ghost = built(&with_fixes(climb(), &fixes, None), None).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::FixesBeforeLiftoff);
        assert_eq!(ghost.pad.latitude_deg, Some(lat));
        assert_eq!(ghost.pad.longitude_deg, Some(lon));
    }

    #[test]
    fn a_first_fix_after_liftoff_says_how_late_it_was() {
        let ghost = built(&with_fixes(climb(), &[(20.0, 39.39, -8.29)], None), None).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::FirstFix);
        let zero = ghost.tracks[0].liftoff_s.unwrap();
        assert_eq!(ghost.pad.first_fix_after_liftoff_s, Some(20.0 - zero));
    }

    #[test]
    fn each_bad_fix_is_left_out_and_counted_by_its_own_reason() {
        let (lat, lon) = (39.39, -8.29);
        let fixes = [
            (1.0, lat, lon),
            (2.0, 0.0, 0.0),      // a receiver's zero
            (3.0, 95.0, lon),     // no latitude
            (4.0, f64::NAN, lon), // an empty cell
            (5.0, lat, lon),
            (5.0, lat, lon),        // the clock stood still
            (4.5, lat, lon),        // and went back
            (6.0, lat, -lon),       // a flipped sign: out past the reach
            (7.0, lat + 0.09, lon), // 10 km in a second
            (8.0, lat, lon),
        ];
        let ghost = built(&with_fixes(climb(), &fixes, None), None).unwrap();
        let counts = ghost.tracks[0].fixes;
        assert_eq!(
            counts,
            FixCounts {
                given: 10,
                kept: 3,
                not_a_place: 3,
                clock_back: 2,
                out_of_reach: 1,
                too_fast: 1,
            }
        );
    }

    #[test]
    fn a_fast_track_inside_the_envelope_is_kept() {
        // 1,200 m/s, about Mach 3.5, north for 10 s: every fix stands (1° is about 110,950 m).
        let (lat, lon) = (39.39, -8.29);
        let fixes: Vec<(f64, f64, f64)> = (0..11)
            .map(|k| (f64::from(k), lat + f64::from(k) * 1_200.0 / 110_950.0, lon))
            .collect();
        let ghost = built(&with_fixes(climb(), &fixes, None), None).unwrap();
        assert_eq!(ghost.tracks[0].fixes.kept, 11);
    }

    #[test]
    fn a_height_no_flight_could_reach_is_left_out() {
        let mut log = climb();
        let column = log
            .channels
            .iter_mut()
            .find(|c| c.quantity == Quantity::AltitudeAboveGround)
            .unwrap();
        let n = column.values.len();
        column.values[n - 2] = 90_000.0;
        column.values[n - 1] = -90_000.0;
        let ghost = built(&log, None).unwrap();
        let track = &ghost.tracks[0];
        // The one too high by its speed; the one too low by the floor, which it meets first.
        assert_eq!(track.heights_too_fast, 1);
        assert_eq!(track.heights_below_pad, 1);
        assert_eq!(track.time_s.len(), n - 2);
        assert!(track.up_m.iter().flatten().all(|h| h.abs() < 90_000.0));
    }

    #[test]
    fn gnss_altitude_is_measured_from_the_pads() {
        let (lat, lon) = (39.39, -8.29);
        let fixes = [(0.0, lat, lon), (1.0, lat, lon)];
        let mut log = with_fixes(no_height(), &fixes, Some(1_400.0));
        log.channels.last_mut().unwrap().values = vec![1_400.0, 1_500.0];
        let ghost = built(&log, None).unwrap();
        let track = &ghost.tracks[0];
        assert_eq!(track.up_from, UpFrom::GnssAltitude);
        assert_eq!(ghost.pad.from, PadFrom::FirstFix);
        assert_eq!(ghost.pad.height_m, Some(1_400.0));
        assert_eq!(track.up_m, vec![Some(0.0), Some(100.0)]);
        assert!(!track.synced);
    }

    #[test]
    fn a_place_alone_has_no_height_and_nothing_is_refused() {
        let (lat, lon) = (39.39, -8.29);
        let ghost = built(
            &with_fixes(no_height(), &[(0.0, lat, lon), (1.0, lat, lon)], None),
            None,
        );
        assert_eq!(ghost.unwrap().tracks[0].up_from, UpFrom::Unknown);
    }

    #[test]
    fn a_log_with_nothing_to_draw_is_refused_by_name() {
        assert_eq!(
            built(&no_height(), None),
            Err(GhostError::NoTrack {
                name: "a.csv".to_owned()
            })
        );
    }

    #[test]
    fn a_log_with_no_liftoff_cant_join_others_on_a_timeline() {
        let good = climb();
        let bad = with_fixes(no_height(), &[(0.0, 39.39, -8.29)], None);
        let (read_good, read_bad) = (readings::read(&good), readings::read(&bad));
        let inputs = [
            Input {
                name: "good.csv",
                log: &good,
                read: &read_good,
            },
            Input {
                name: "bad.csv",
                log: &bad,
                read: &read_bad,
            },
        ];
        let Err(GhostError::NotOnTimeline { name, .. }) = build(&inputs, None) else {
            panic!("a log with no liftoff joined a timeline");
        };
        assert_eq!(name, "bad.csv");
        assert_eq!(build(&[], None), Err(GhostError::NoLogs));
    }

    #[test]
    fn a_stated_pad_wins_and_must_be_a_place() {
        let log = with_fixes(climb(), &[(5.0, 39.39, -8.29)], Some(1_000.0));
        let stated = StatedPad {
            latitude_deg: 39.4,
            longitude_deg: -8.3,
            height_m: None,
        };
        let ghost = built(&log, Some(stated)).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::Stated);
        assert_eq!(ghost.pad.latitude_deg, Some(39.4));
        assert_eq!(ghost.pad.height_m, Some(1_000.0));
        let far = StatedPad {
            latitude_deg: 91.0,
            ..stated
        };
        assert!(matches!(
            built(&log, Some(far)),
            Err(GhostError::BadPad { what }) if what.contains("out of range")
        ));
        let huge = StatedPad {
            height_m: Some(1e300),
            ..stated
        };
        assert!(matches!(
            built(&log, Some(huge)),
            Err(GhostError::BadPad { what }) if what.contains("height")
        ));
        let nan = StatedPad {
            height_m: Some(f64::NAN),
            ..stated
        };
        assert!(matches!(
            built(&log, Some(nan)),
            Err(GhostError::BadPad { what }) if what.contains("height")
        ));
    }

    #[test]
    fn czml_is_earth_fixed_on_its_epoch_by_hand() {
        // At 0°N 0°E, up is +x, east +y and north +z, so (e, n, u) = (1, 2, 3) m is
        // (a + 3, 1, 2) in the Earth-fixed frame, with a = 6,378,137 m.
        let text = little_ghost(Some(1.0), Some(3.0)).to_czml().unwrap();
        let packets: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(packets[0]["id"], "document");
        assert_eq!(packets[0]["version"], "1.0");
        assert_eq!(
            packets[0]["clock"]["interval"],
            "2000-01-01T11:59:59.999Z/2000-01-01T12:00:01.001Z"
        );
        let position = &packets[1]["position"];
        assert_eq!(position["epoch"], "2000-01-01T12:00:00Z");
        assert_eq!(position["referenceFrame"], "FIXED");
        let numbers = position["cartesian"].as_array().unwrap();
        assert_eq!(numbers.len(), 8);
        let at = |i: usize| numbers[i].as_f64().unwrap();
        assert_eq!(at(4), 1.0);
        assert!((at(5) - 6_378_140.0).abs() < 1e-6, "{}", at(5));
        assert!((at(6) - 1.0).abs() < 1e-6);
        assert!((at(7) - 2.0).abs() < 1e-6);
    }

    #[test]
    fn czml_dates_carry_over_midnight_and_back() {
        assert_eq!(Ghost::czml_time(0.0, false), "2000-01-01T11:59:59.999Z");
        assert_eq!(Ghost::czml_time(43_200.0, true), "2000-01-02T00:00:00.001Z");
        assert_eq!(
            Ghost::czml_time(-43_201.0, false),
            "1999-12-31T23:59:58.999Z"
        );
    }

    #[test]
    fn what_a_drawing_assumes_it_says() {
        let mut ghost = little_ghost(None, None);
        ghost.tracks[0].fixes.kept = 1;
        // Held at the nearest fix; an unknown height at 0.
        assert_eq!(drawn(&ghost.tracks[0])[0], [0.0, 1.0, 2.0, 0.0]);
        assert!(assumed(&ghost, &ghost.tracks[0]).contains("held at the nearest"));
        ghost.pad.from = PadFrom::Unknown;
        ghost.tracks[0].fixes.kept = 0;
        ghost.tracks[0].up_from = UpFrom::Unknown;
        let words = assumed(&ghost, &ghost.tracks[0]);
        assert!(words.contains("drawn at 0N 0E"), "{words}");
        assert!(words.contains("straight vertical line"), "{words}");
        assert!(words.contains("no height"), "{words}");
    }

    #[test]
    fn gltfs_bytes_are_what_the_spec_lays_out_by_hand() {
        let text = little_ghost(Some(1.0), Some(3.0)).to_gltf().unwrap();
        let gltf: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(gltf["asset"]["version"], "2.0");
        let uri = gltf["buffers"][0]["uri"].as_str().unwrap();
        let body = uri
            .strip_prefix("data:application/octet-stream;base64,")
            .unwrap();
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(body)
            .unwrap();
        assert_eq!(gltf["buffers"][0]["byteLength"], bytes.len());
        // The marker's first corner: x = 5 as a little-endian IEEE 754 single, 0x40A00000.
        assert_eq!(bytes[..4], [0x00, 0x00, 0xA0, 0x40]);
        // Its 6 corners take 72 bytes, 24 indices 48, so the track's times start at 120: 0 and 1
        // (0x3F800000); then its places, +x east, +y up, −z north: (1, 3, −2) twice.
        assert_eq!(bytes[120..128], [0, 0, 0, 0, 0x00, 0x00, 0x80, 0x3F]);
        let single = |value: f32| value.to_le_bytes();
        assert_eq!(bytes[128..132], single(1.0));
        assert_eq!(bytes[132..136], single(3.0));
        assert_eq!(bytes[136..140], single(-2.0));
        assert_eq!(single(-2.0), [0x00, 0x00, 0x00, 0xC0]);
        // Times start at the earliest moment, and the root says which that is.
        assert_eq!(gltf["extras"]["first_moment_s"], 0.0);
        let sampler = &gltf["animations"][0]["samplers"][0];
        assert_eq!(sampler["interpolation"], "LINEAR");
        assert_eq!(
            gltf["accessors"][sampler["input"].as_u64().unwrap() as usize]["min"][0],
            0.0
        );
        assert_eq!(
            gltf["accessors"][sampler["input"].as_u64().unwrap() as usize]["max"][0],
            1.0
        );
        // The path's line and the animation hold one set of bytes under two views.
        let line = gltf["meshes"][0]["primitives"][0]["attributes"]["POSITION"]
            .as_u64()
            .unwrap();
        let output = sampler["output"].as_u64().unwrap();
        assert_ne!(line, output);
        let view = |accessor: u64| {
            gltf["bufferViews"][gltf["accessors"][accessor as usize]["bufferView"]
                .as_u64()
                .unwrap() as usize]
                .clone()
        };
        assert_eq!(view(line)["byteOffset"], view(output)["byteOffset"]);
        assert_eq!(view(line)["target"], 34962);
        assert_eq!(view(output)["target"], Value::Null);
    }

    #[test]
    fn two_moments_a_float_cant_part_become_one_in_gltf() {
        let mut ghost = little_ghost(Some(1.0), Some(3.0));
        ghost.tracks[0].time_s = vec![1_000.0, 1_000.000_01, 1_001.0];
        ghost.tracks[0].east_m = vec![Some(1.0), Some(2.0), Some(3.0)];
        ghost.tracks[0].north_m = vec![Some(0.0); 3];
        ghost.tracks[0].up_m = vec![Some(0.0); 3];
        let gltf: Value = serde_json::from_str(&ghost.to_gltf().unwrap()).unwrap();
        let input = gltf["animations"][0]["samplers"][0]["input"]
            .as_u64()
            .unwrap() as usize;
        // 1,000.00001 − 1,000 is kept as a time (the clock starts there) so all three stand ...
        assert_eq!(gltf["accessors"][input]["count"], 3);
        // ... but on a clock that starts 1,000 s earlier they can't be told apart.
        ghost.tracks[0].time_s = vec![0.0, 1_000.0, 1_000.000_01, 1_001.0];
        ghost.tracks[0].east_m = vec![Some(0.0), Some(1.0), Some(2.0), Some(3.0)];
        ghost.tracks[0].north_m = vec![Some(0.0); 4];
        ghost.tracks[0].up_m = vec![Some(0.0); 4];
        let gltf: Value = serde_json::from_str(&ghost.to_gltf().unwrap()).unwrap();
        assert_eq!(gltf["accessors"][input]["count"], 3);
    }

    /// A log from the rows of a table, as its own reader reads it.
    fn table(header: &str, rows: impl Iterator<Item = String>) -> FlightLog {
        let mut text = format!("{header}\n");
        for row in rows {
            text.push_str(&row);
            text.push('\n');
        }
        import::read_one(text.as_bytes(), None, None).unwrap()
    }

    /// A rising receiver: fixes each 0.5 s from `pad_m`, none moving sideways.
    fn receiver(heights: &[f64]) -> FlightLog {
        let mut log = no_height();
        let fixes: Vec<(f64, f64, f64)> = (0..heights.len())
            .map(|i| (i as f64 * 0.5, 40.0, -105.0))
            .collect();
        log = with_fixes(log, &fixes, Some(0.0));
        let height = log
            .channels
            .iter_mut()
            .find(|c| c.quantity == Quantity::GnssAltitude)
            .unwrap();
        height.values = heights.to_vec();
        log
    }

    #[test]
    fn a_moment_with_no_height_is_left_out_not_drawn_at_the_pad() {
        let mut heights: Vec<f64> = (0..40).map(|i| 1_400.0 + f64::from(i) * 20.0).collect();
        heights[20] = f64::NAN;
        heights[21] = 0.0;
        let ghost = built(&receiver(&heights), None).unwrap();
        let track = &ghost.tracks[0];
        assert_eq!(track.up_from, UpFrom::GnssAltitude);
        assert_eq!(track.moments_left_out, 2);
        assert_eq!(track.time_s.len(), 38);
        assert!(track.up_m.iter().all(Option::is_some));
        assert!(!track.time_s.contains(&10.0) && !track.time_s.contains(&10.5));
        assert!(assumed(&ghost, track).contains("2 of the log's rows are left out"));
    }

    #[test]
    fn a_height_far_under_the_pad_is_left_out_and_one_inside_the_floor_kept() {
        let mut heights = vec![1_400.0; 30];
        // Inside the floor, reached slowly; then a receiver's height near sea level, in reach of
        // the speed screen, which only the floor can catch.
        heights[10] = 1_400.0 - BELOW_PAD_M + 1.0;
        heights[20] = 1_400.0 - BELOW_PAD_M - 1.0;
        let ghost = built(&receiver(&heights), None).unwrap();
        let track = &ghost.tracks[0];
        assert_eq!(track.heights_below_pad, 1);
        assert_eq!(track.heights_too_fast, 0);
        assert_eq!(track.up_m[10], Some(1.0 - BELOW_PAD_M));
        assert_eq!(track.time_s.len(), 29);
        assert!(track.up_m.iter().flatten().all(|h| *h >= -BELOW_PAD_M));
    }

    #[test]
    fn a_column_of_another_zero_is_taken_less_its_own_pad() {
        for header in [
            "Time (s),Altitude (m)",
            "Time (s),Barometric altitude MSL (m)",
        ] {
            let log = table(
                header,
                (0..200).map(|i| {
                    let t = f64::from(i) * 0.1;
                    format!("{t:.1},{:.1}", 1_400.0 + (t - 10.0).max(0.0) * 100.0)
                }),
            );
            let ghost = built(&log, None).unwrap();
            let track = &ghost.tracks[0];
            assert_eq!(track.up_from, UpFrom::AltitudeLessPad, "{header}");
            assert_eq!(track.up_m[0], Some(0.0), "{header}");
            let top = track.up_m.iter().flatten().copied().fold(0.0, f64::max);
            assert!((top - 990.0).abs() < 1e-6, "{header}: {top}");
            assert!(
                assumed(&ghost, track).contains("its own reading"),
                "{header}"
            );
        }
    }

    #[test]
    fn a_height_above_ground_stays_as_the_log_wrote_it() {
        let ghost = built(&climb(), None).unwrap();
        assert_eq!(ghost.tracks[0].up_from, UpFrom::LogAltitude);
    }

    #[test]
    fn the_pads_height_never_comes_from_a_fix_in_flight() {
        // Liftoff at 10 s; fixes throughout, their heights only from 30 s.
        let fixes: Vec<(f64, f64, f64)> = (0..40).map(|i| (f64::from(i), 40.0, -105.0)).collect();
        let mut log = with_fixes(climb(), &fixes, Some(0.0));
        let height = log
            .channels
            .iter_mut()
            .find(|c| c.quantity == Quantity::GnssAltitude)
            .unwrap();
        height.values = (0..40)
            .map(|i| if i < 30 { f64::NAN } else { 3_000.0 })
            .collect();
        let ghost = built(&log, None).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::FixesBeforeLiftoff);
        assert_eq!(ghost.pad.height_m, None);
        assert!(assumed(&ghost, &ghost.tracks[0]).contains("isn't known"));

        // A first fix after liftoff: its place stands for the pad, flagged; its height doesn't.
        let late: Vec<(f64, f64, f64)> = (0..10)
            .map(|i| (20.0 + f64::from(i), 40.0, -105.0))
            .collect();
        let ghost = built(&with_fixes(climb(), &late, Some(3_000.0)), None).unwrap();
        assert_eq!(ghost.pad.from, PadFrom::FirstFix);
        assert_eq!(ghost.pad.height_m, None);
    }

    #[test]
    fn a_stated_pad_height_doesnt_move_a_receivers_heights() {
        let heights: Vec<f64> = (0..40).map(|i| 1_400.0 + f64::from(i)).collect();
        let stated = StatedPad {
            latitude_deg: 40.0,
            longitude_deg: -105.0,
            height_m: Some(1_380.0),
        };
        let ghost = built(&receiver(&heights), Some(stated)).unwrap();
        assert_eq!(ghost.pad.height_m, Some(1_380.0));
        assert_eq!(ghost.tracks[0].up_m[0], Some(0.0));
        assert!(!assumed(&ghost, &ghost.tracks[0]).contains("receiver's"));

        // A stated place with no height takes the receiver's, and says so.
        let place = StatedPad {
            height_m: None,
            ..stated
        };
        let ghost = built(&receiver(&heights), Some(place)).unwrap();
        assert_eq!(ghost.pad.height_m, Some(1_400.0));
        assert!(assumed(&ghost, &ghost.tracks[0]).contains("local geoid"));
    }

    #[test]
    fn a_receivers_pad_height_says_what_it_stands_for() {
        let heights = vec![1_400.0; 30];
        let ghost = built(&receiver(&heights), None).unwrap();
        assert!(assumed(&ghost, &ghost.tracks[0]).contains("local geoid"));
    }

    #[test]
    fn one_bad_first_row_doesnt_shut_out_the_flight() {
        for bad in [30_000.0, 90_000.0, -40_000.0] {
            let log = table(
                "Time (s),Barometric altitude AGL (m)",
                std::iter::once(format!("-0.1,{bad}")).chain((0..400).map(|i| {
                    let t = f64::from(i) * 0.1;
                    format!("{t:.1},{:.1}", (t - 10.0).max(0.0) * 50.0)
                })),
            );
            let track = built(&log, None).unwrap().tracks.remove(0);
            assert_eq!(track.heights_too_fast + track.heights_below_pad, 1, "{bad}");
            assert_eq!(track.time_s.len(), 400, "{bad}");
            assert!(track.up_m.iter().flatten().all(|h| *h < 1_600.0), "{bad}");
        }
    }

    #[test]
    fn one_bad_first_fix_doesnt_shut_out_the_track() {
        let mut fixes = vec![(-0.1, 40.5, -105.0)];
        fixes.extend((0..30).map(|i| (f64::from(i) * 0.1, 40.0, -105.0)));
        let track = built(&with_fixes(no_height(), &fixes, Some(1_400.0)), None)
            .unwrap()
            .tracks
            .remove(0);
        assert_eq!(track.fixes.too_fast, 1);
        assert_eq!(track.fixes.kept, 30);
    }

    #[test]
    fn the_opening_finds_a_good_first_row_and_a_quick_start_stays() {
        // A log starting on the climb at Mach 3: every row is reachable from the middle.
        let samples: Vec<(f64, f64)> = (0..20)
            .map(|i| (f64::from(i) * 0.1, f64::from(i) * 100.0))
            .collect();
        assert_eq!(opening_outliers(&samples, HEIGHT_SCATTER_M), 0);
        assert_eq!(opening_outliers(&[], HEIGHT_SCATTER_M), 0);
        let mut two_bad = samples.clone();
        two_bad[0].1 = 50_000.0;
        two_bad[1].1 = -50_000.0;
        assert_eq!(opening_outliers(&two_bad, HEIGHT_SCATTER_M), 2);
    }

    #[test]
    fn dates_are_the_calendars() {
        // Worked by the calendar: 2000 is a leap year, and 2100 isn't.
        for (days, date) in [
            (0, (2000, 1, 1)),
            (30, (2000, 1, 31)),
            (31, (2000, 2, 1)),
            (59, (2000, 2, 29)),
            (60, (2000, 3, 1)),
            (365, (2000, 12, 31)),
            (366, (2001, 1, 1)),
            (-1, (1999, 12, 31)),
            (-31, (1999, 12, 1)),
            (-32, (1999, 11, 30)),
            (36_583, (2100, 2, 28)),
            (36_584, (2100, 3, 1)),
        ] {
            assert_eq!(date_from_days(days), date, "{days}");
        }
        assert_eq!(
            Ghost::czml_time(3e6 + 0.0015, false),
            "2000-02-05T05:20:00.000Z"
        );
    }

    #[test]
    fn a_garbage_clock_is_left_out_and_writes_nothing_wrong() {
        for last in ["1.1e7", "1e17", "1e19", "1e39", "1e300"] {
            let log = table(
                "Time (s),Barometric altitude AGL (m)",
                (0..400)
                    .map(|i| {
                        let t = f64::from(i) * 0.1;
                        format!("{t:.1},{:.1}", (t - 10.0).max(0.0) * 50.0)
                    })
                    .chain(std::iter::once(format!("{last},0"))),
            );
            let ghost = built(&log, None).unwrap();
            assert_eq!(ghost.tracks[0].time_s.len(), 400, "{last}");
            assert_eq!(ghost.tracks[0].moments_left_out, 1, "{last}");
            let czml: Value = serde_json::from_str(&ghost.to_czml().unwrap()).unwrap();
            assert!(
                czml[0]["clock"]["interval"]
                    .as_str()
                    .unwrap()
                    .contains("2000-01-01")
            );
            let gltf: Value = serde_json::from_str(&ghost.to_gltf().unwrap()).unwrap();
            assert!(
                gltf["accessors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|a| { !a.to_string().contains("null") }),
                "{last}"
            );
        }
    }

    #[test]
    fn a_unix_clock_starts_czml_at_its_first_moment() {
        let heights: Vec<f64> = (0..30).map(|i| 1_400.0 + f64::from(i)).collect();
        let mut log = receiver(&heights);
        for channel in &mut log.channels {
            for time in &mut channel.time_s {
                *time += 1.7e9;
            }
        }
        let ghost = built(&log, None).unwrap();
        assert!(ghost.tracks[0].time_s[0] > 1.6e9);
        let czml: Value = serde_json::from_str(&ghost.to_czml().unwrap()).unwrap();
        assert_eq!(
            czml[0]["clock"]["interval"],
            "2000-01-01T11:59:59.999Z/2000-01-01T12:00:14.501Z"
        );
        assert_eq!(czml[1]["position"]["cartesian"][0], 0.0);
    }

    #[test]
    fn a_name_is_text_in_czmls_description() {
        let mut ghost = little_ghost(Some(1.0), Some(1.0));
        ghost.tracks[0].name = "<img src=x>&.csv".to_owned();
        let czml: Value = serde_json::from_str(&ghost.to_czml().unwrap()).unwrap();
        assert!(
            czml[1]["description"]
                .as_str()
                .unwrap()
                .starts_with("&lt;img src=x&gt;&amp;.csv")
        );
    }

    #[test]
    fn a_ghost_from_elsewhere_is_refused_not_a_panic() {
        let refused = |edit: fn(&mut Ghost)| {
            let mut ghost = little_ghost(Some(1.0), Some(1.0));
            edit(&mut ghost);
            assert!(ghost.to_czml().is_err());
            assert!(ghost.to_gltf().is_err());
        };
        refused(|g| g.tracks.clear());
        refused(|g| g.tracks[0].time_s.clear());
        refused(|g| {
            g.tracks[0].east_m.pop();
        });
        refused(|g| {
            g.tracks[0].north_m.pop();
        });
        refused(|g| {
            g.tracks[0].up_m.pop();
        });
        refused(|g| g.tracks[0].time_s[1] = 0.0);
        refused(|g| g.tracks[0].time_s[1] = 1e300);
        refused(|g| g.tracks[0].time_s[1] = f64::NAN);
        refused(|g| g.tracks[0].east_m[0] = Some(f64::INFINITY));
        refused(|g| g.pad.latitude_deg = Some(200.0));
        let ok = little_ghost(Some(1.0), Some(1.0));
        assert!(ok.to_czml().is_ok() && ok.to_gltf().is_ok());
    }
}
