//! The wind reconstructed from each public flight's GPS-tracked descent, set beside ERA5's
//! ([M1.14b6][m1-14b6]), written by `cargo xtask descent-wind` to
//! `validation/reports/descent-wind.json` and `.md`.
//!
//! - **The flights** are the public real flights the real-flights report flies
//!   ([`real_flight::FLIGHTS`]): RocketPy's example rockets with a flight log. Each is listed:
//!   read, with its receiver and columns ([`SOURCES`]), or not, with the reason.
//! - **The reconstruction** is [`hpr_flightdata::descent_wind`] at its default settings.
//! - **ERA5** is the file and hour the real-flights report flies the flight in, read as HPR Sim
//!   reads it ([`real_flight::launch_day`]): the wind it flies, interpolated by components in
//!   height. A flight whose file is not of its day is reconstructed and not compared.
//! - **Two receivers.** Prometheus carried two Altus Metrum flight computers, each with its own
//!   GPS receiver. The report reconstructs both and gives how far the two winds differ: a floor
//!   for the receivers' and the method's own noise, which no canopy motion enters, since both
//!   rode the same rocket.
//!
//! The logs and ERA5 files are read from the pinned RocketPy checkout under the gitignored
//! `refs/`, so the report is regenerated and checked where that checkout is (the gate's `refs`
//! step). CI, without it, recomputes the bands from the committed samples and the page from the
//! committed report (`hpr_validate::tests`).
//!
//! [m1-14b6]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b6

use std::fmt::Write as _;
use std::path::Path;

use hpr_atmos::Wind as _;
use hpr_core::geodesy::Geodetic;
use hpr_flightdata::descent_wind::{Settings, TrackPoint, WindSample, reconstruct};
use serde::{Deserialize, Serialize};

use crate::real_flight::{self, FLIGHTS, ROCKETPY, RealFlight, RealFlightError, read_bytes};

/// The committed report.
pub const REPORT_JSON: &str = "validation/reports/descent-wind.json";
/// Its page.
pub const REPORT_MD: &str = "validation/reports/descent-wind.md";

/// The height bands the comparison is pooled in, m above the pad: lower edges, and the last
/// band's top.
pub const BAND_EDGES_M: [f64; 6] = [0.0, 500.0, 1000.0, 2000.0, 3000.0, 5000.0];

/// The fewest flights a comparison needs before the validation plan may call it one rather than a
/// method ([the plan's line for M1.14b6, the wind from the GPS descent](https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b6)).
pub const FLIGHTS_FOR_A_COMPARISON: usize = 3;

/// Two receivers' fixes are paired when their times are this close, s.
pub const PAIR_S: f64 = 0.5;

/// Values are kept to this many decimals, so the committed report reads the same on every
/// platform and CI can recompute its statistics from it.
const DECIMALS: i32 = 4;

fn round(value: f64) -> f64 {
    let scale = 10_f64.powi(DECIMALS);
    (value * scale).round() / scale
}

/// How a track's heights are read.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Height {
    /// The receiver's own altitude above sea level, in a column of the track's file, with meters
    /// per unit. The pad is the first fix's.
    Gnss {
        /// The column (zero-based).
        column: usize,
        /// Meters per unit.
        meters_per_unit: f64,
    },
    /// A barometric height above the pad from another file on the same clock, interpolated
    /// linearly to each fix's time; above sea level it is that plus the site's elevation, as the
    /// real-flights report flies it.
    Barometric {
        /// The file, under `refs/rocketpy/data/rockets/`.
        file: &'static str,
        /// The column of time, s (zero-based).
        time_column: usize,
        /// The column of height, m (zero-based).
        height_column: usize,
    },
}

/// How a new fix is told from a repeated row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewFix {
    /// A row is a new fix when the receiver's own position differs from the last fix's: its
    /// latitude and longitude, and its altitude where the track reads the receiver's
    /// ([`Height::Gnss`]). A receiver repeats its last fix between updates and after it loses
    /// lock, so a run of repeats is a gap in the track: no fix is read whose window does not
    /// reach a quarter of its width past it on each side.
    PositionChanges,
    /// AltOS's CSV repeats the last fix on every sensor row; a row is a new fix when its GPS
    /// clock (hour, minute, second columns) changes and its `locked` column is 1.
    GpsClock {
        /// The hour, minute and second columns.
        clock: [usize; 3],
        /// The `locked` column.
        locked: usize,
    },
}

/// Where a flight's GPS track is and how to read it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Track {
    /// The receiver, in words.
    pub receiver: &'static str,
    /// The CSV file, under `refs/rocketpy/data/rockets/`.
    pub file: &'static str,
    /// The column of time, s (zero-based).
    pub time_column: usize,
    /// The columns of latitude and longitude, degrees (zero-based).
    pub position_columns: [usize; 2],
    /// The heights.
    pub height: Height,
    /// How a fix is told from a repeat.
    pub new_fix: NewFix,
}

/// What a public flight's log holds for the descent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source {
    /// No GPS track of the descent: the reason.
    None(&'static str),
    /// A GPS track, and a second receiver's on the same rocket, if one was carried.
    Gps {
        /// The track.
        track: &'static Track,
        /// A second receiver on the same rocket.
        second: Option<&'static Track>,
        /// Why its ERA5 file can't be compared with it, if it can't.
        not_compared: Option<&'static str>,
    },
}

const TELEMETRUM: Track = Track {
    receiver: "the TeleMetrum's GPS (Altus Metrum), 1 fix a second",
    file: "prometheus/2022-06-24-serial-5115-flight-0001-TeleMetrum.csv",
    time_column: 4,
    position_columns: [19, 20],
    height: Height::Gnss {
        column: 21,
        meters_per_unit: 1.0,
    },
    new_fix: NewFix::GpsClock {
        clock: [25, 26, 27],
        locked: 17,
    },
};

const TELEMEGA: Track = Track {
    receiver: "the TeleMega's GPS (Altus Metrum), 1 fix a second",
    file: "prometheus/2022-06-24-serial-6583-flight-0003-TeleMega.csv",
    time_column: 4,
    position_columns: [34, 35],
    height: Height::Gnss {
        column: 36,
        meters_per_unit: 1.0,
    },
    new_fix: NewFix::GpsClock {
        clock: [40, 41, 42],
        locked: 32,
    },
};

/// Each public flight's source, by id, in [`FLIGHTS`]' order.
pub const SOURCES: [(&str, Source); 7] = [
    (
        "bella-lui",
        Source::None("its log holds a filtered height and speed only, no position"),
    ),
    (
        "ndrt-2020",
        Source::None("its log holds acceleration and a barometric height only, no position"),
    ),
    (
        "prometheus-2022",
        Source::Gps {
            track: &TELEMETRUM,
            second: Some(&TELEMEGA),
            not_compared: Some(
                "RocketPy's ERA5 file for it is of 24 June 2023, a year after the flight, and no \
                 file of its day is pinned",
            ),
        },
    ),
    (
        "juno-iii",
        Source::Gps {
            track: &Track {
                receiver: "the COTS GNSS log (`cots_GNSS.csv`), 1 fix a second in the descent; \
                           the team's own telemetry, which the README calls possibly noisy, is \
                           not read",
                file: "juno3/cots_GNSS.csv",
                time_column: 0,
                position_columns: [4, 5],
                height: Height::Gnss {
                    column: 1,
                    meters_per_unit: 0.3048,
                },
                new_fix: NewFix::PositionChanges,
            },
            second: None,
            not_compared: None,
        },
    ),
    (
        "cavour",
        Source::Gps {
            track: &Track {
                receiver: "the CATS Vega's radio telemetry: latitude and longitude sent to four \
                           decimals (about 11 m north and 9 m east), height from its barometric \
                           filter",
                file: "polito/altimeter_cavour.csv",
                time_column: 0,
                position_columns: [3, 4],
                height: Height::Barometric {
                    file: "polito/altimeter_cavour.csv",
                    time_column: 0,
                    height_column: 1,
                },
                new_fix: NewFix::PositionChanges,
            },
            second: None,
            not_compared: None,
        },
    ),
    (
        "genesis",
        Source::Gps {
            track: &Track {
                receiver: "the GNSS log (`gnss_faraday.csv`): latitude and longitude, 1 to 10 \
                           fixes a second; height from the flight log's barometric filter",
                file: "genesis/gnss_faraday.csv",
                time_column: 0,
                position_columns: [1, 2],
                height: Height::Barometric {
                    file: "genesis/flight_data_faraday.csv",
                    time_column: 0,
                    height_column: 1,
                },
                new_fix: NewFix::PositionChanges,
            },
            second: None,
            not_compared: None,
        },
    ),
    (
        "lince",
        Source::None("its log holds a filtered height and acceleration only, no position"),
    ),
];

/// Where a flight stands in the report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// Reconstructed and compared with ERA5.
    Compared,
    /// Reconstructed, not compared: why.
    NotCompared(String),
    /// Not reconstructed: why.
    NotRun(String),
}

/// One read fix of a descent.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    /// The fix's time on the log's clock, s.
    pub time_s: f64,
    /// Its height above the pad, m.
    pub height_m: f64,
    /// Its height above sea level, m.
    pub height_msl_m: f64,
    /// The descent rate, m/s.
    pub descent_m_s: f64,
    /// The ground velocity, east and north, m/s.
    pub ground_m_s: [f64; 2],
    /// The reconstructed wind, east and north, m/s.
    pub wind_m_s: [f64; 2],
    /// ERA5's wind at the same height, east and north, m/s, where compared.
    pub era5_m_s: Option<[f64; 2]>,
}

impl Sample {
    fn speed(wind: [f64; 2]) -> f64 {
        wind[0].hypot(wind[1])
    }

    /// The reconstructed speed less ERA5's, m/s, where compared.
    #[must_use]
    pub fn speed_difference_m_s(&self) -> Option<f64> {
        self.era5_m_s
            .map(|era5| Self::speed(self.wind_m_s) - Self::speed(era5))
    }

    /// The length of the reconstructed wind less ERA5's, m/s, where compared.
    #[must_use]
    pub fn vector_difference_m_s(&self) -> Option<f64> {
        self.era5_m_s
            .map(|era5| (self.wind_m_s[0] - era5[0]).hypot(self.wind_m_s[1] - era5[1]))
    }

    /// What the lag correction adds to the ground velocity, m/s.
    #[must_use]
    pub fn correction_m_s(&self) -> f64 {
        (self.wind_m_s[0] - self.ground_m_s[0]).hypot(self.wind_m_s[1] - self.ground_m_s[1])
    }
}

/// One public flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightEntry {
    /// The flight's id in the real-flights report.
    pub id: String,
    /// The rocket, the team and the event.
    pub title: String,
    /// Where it stands.
    pub status: Status,
    /// The receiver read, in words; empty when none.
    pub receiver: String,
    /// The fixes the track holds.
    pub fixes: usize,
    /// The fixes read as a steady fall.
    pub samples: Vec<Sample>,
}

/// The comparison pooled over one height band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Band {
    /// The band's bottom, m above the pad.
    pub lower_m: f64,
    /// Its top, m above the pad.
    pub upper_m: f64,
    /// The compared flights with a sample in it.
    pub flights: usize,
    /// Their samples in it.
    pub samples: usize,
    /// The reconstructed speed less ERA5's: the mean, m/s.
    pub mean_speed_difference_m_s: f64,
    /// And the root-mean-square, m/s.
    pub rms_speed_difference_m_s: f64,
    /// The root-mean-square length of the vector difference, m/s.
    pub rms_vector_difference_m_s: f64,
}

/// Two receivers on one rocket, each reconstructed alone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receivers {
    /// The flight.
    pub id: String,
    /// The second receiver, in words.
    pub second: String,
    /// The fixes of the two read at times within [`PAIR_S`] of each other.
    pub pairs: usize,
    /// The root-mean-square length of the difference of their winds, m/s.
    pub rms_vector_difference_m_s: f64,
    /// The largest, m/s.
    pub max_vector_difference_m_s: f64,
}

/// The report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescentWindReport {
    /// The reconstruction's settings.
    pub settings: Settings,
    /// Every public flight.
    pub flights: Vec<FlightEntry>,
    /// The comparison by band, over the compared flights.
    pub bands: Vec<Band>,
    /// The same over every band.
    pub all: Band,
    /// Two receivers on one rocket, where a flight carried them.
    pub receivers: Vec<Receivers>,
}

fn columns(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}

fn number(row: &[&str], column: usize, file: &str, line: usize) -> Result<f64, String> {
    row.get(column)
        .ok_or_else(|| format!("{file}:{line}: no column {column}"))?
        .parse::<f64>()
        .map_err(|error| format!("{file}:{line}: column {column}: {error}"))
}

/// `(time, value)` pairs from a CSV with one header line.
fn series(text: &str, file: &str, time: usize, value: usize) -> Result<Vec<(f64, f64)>, String> {
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let row = columns(line);
        out.push((
            number(&row, time, file, index + 1)?,
            number(&row, value, file, index + 1)?,
        ));
    }
    Ok(out)
}

/// `series` at `time`, linear between its two neighbors; `None` outside it.
fn interpolate(series: &[(f64, f64)], time: f64) -> Option<f64> {
    let upper = series.partition_point(|(t, _)| *t < time);
    if upper == 0 {
        return series.first().and_then(|(t, v)| (*t == time).then_some(*v));
    }
    let (t1, v1) = *series.get(upper)?;
    let (t0, v0) = series[upper - 1];
    if t1 == t0 {
        return Some(v1);
    }
    Some(v0 + (time - t0) / (t1 - t0) * (v1 - v0))
}

/// A fix: time, latitude, longitude, height above the pad and above sea level.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Fix {
    time_s: f64,
    latitude_deg: f64,
    longitude_deg: f64,
    height_m: f64,
    height_msl_m: f64,
}

/// The track's fixes, in time order, each new.
fn fixes(root: &Path, track: &Track, elevation_m: f64) -> Result<Vec<Fix>, String> {
    let path = format!("{ROCKETPY}/data/rockets/{}", track.file);
    let bytes = read_bytes(root, &path, "a GPS track").map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&bytes);
    let barometric = match track.height {
        Height::Barometric {
            file,
            time_column,
            height_column,
        } => {
            let path = format!("{ROCKETPY}/data/rockets/{file}");
            let bytes =
                read_bytes(root, &path, "a barometric height").map_err(|e| e.to_string())?;
            Some(series(
                &String::from_utf8_lossy(&bytes),
                file,
                time_column,
                height_column,
            )?)
        }
        Height::Gnss { .. } => None,
    };
    let mut out: Vec<Fix> = Vec::new();
    let mut last_clock: Option<[f64; 3]> = None;
    let mut pad_msl: Option<f64> = None;
    for (index, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let row = columns(line);
        let at = |column| number(&row, column, track.file, index + 1);
        let time_s = at(track.time_column)?;
        let latitude_deg = at(track.position_columns[0])?;
        let longitude_deg = at(track.position_columns[1])?;
        if latitude_deg == 0.0 && longitude_deg == 0.0 {
            // No fix yet.
            continue;
        }
        if let NewFix::GpsClock { clock, locked } = track.new_fix {
            if at(locked)? != 1.0 {
                continue;
            }
            let now = [at(clock[0])?, at(clock[1])?, at(clock[2])?];
            if last_clock == Some(now) {
                continue;
            }
            last_clock = Some(now);
        }
        let (height_m, height_msl_m) = match (track.height, &barometric) {
            (
                Height::Gnss {
                    column,
                    meters_per_unit,
                },
                _,
            ) => {
                let msl = at(column)? * meters_per_unit;
                let pad = *pad_msl.get_or_insert(msl);
                (msl - pad, msl)
            }
            (Height::Barometric { .. }, Some(heights)) => {
                let Some(height) = interpolate(heights, time_s) else {
                    continue;
                };
                (height, height + elevation_m)
            }
            (Height::Barometric { .. }, None) => {
                return Err(format!("{}: no barometric heights read", track.file));
            }
        };
        let fix = Fix {
            time_s,
            latitude_deg,
            longitude_deg,
            height_m,
            height_msl_m,
        };
        if let Some(previous) = out.last() {
            if fix.time_s <= previous.time_s {
                continue;
            }
            // Only the receiver's own values tell a new fix: a barometric height changes while
            // a receiver that has lost its fix repeats its last position.
            let receivers_height = matches!(track.height, Height::Gnss { .. });
            if track.new_fix == NewFix::PositionChanges
                && fix.latitude_deg == previous.latitude_deg
                && fix.longitude_deg == previous.longitude_deg
                && (!receivers_height || fix.height_msl_m == previous.height_msl_m)
            {
                continue;
            }
        }
        out.push(fix);
    }
    Ok(out)
}

/// The steady-fall samples of a track, with their fixes' heights.
fn read_track(fixes: &[Fix], settings: &Settings) -> Result<Vec<(WindSample, Fix)>, String> {
    let Some(first) = fixes.first() else {
        return Ok(Vec::new());
    };
    let origin = Geodetic::from_degrees(first.latitude_deg, first.longitude_deg, 0.0)
        .map_err(|e| e.to_string())?;
    let mut points = Vec::with_capacity(fixes.len());
    for fix in fixes {
        let point = Geodetic::from_degrees(fix.latitude_deg, fix.longitude_deg, fix.height_msl_m)
            .map_err(|e| e.to_string())?;
        let mut enu = TrackPoint::from_geodetic(fix.time_s, origin, point);
        // The height is the log's, not the tangent plane's, which drops by the earth's curvature.
        enu.up_m = fix.height_msl_m;
        points.push(enu);
    }
    let samples = reconstruct(&points, settings).map_err(|e| e.to_string())?;
    Ok(samples
        .into_iter()
        .filter_map(|sample| {
            fixes
                .iter()
                .find(|fix| fix.time_s == sample.time_s)
                .map(|fix| (sample, *fix))
        })
        .collect())
}

/// The descent rates a track's fitted windows give, steady or not, after its highest fix: what
/// the report says of a track with no steady fall.
fn fall_rates(fixes: &[Fix]) -> Result<Vec<f64>, String> {
    let open = Settings {
        min_descent_m_s: 1e-9,
        max_descent_m_s: f64::MAX,
        max_vertical_acceleration_m_s2: f64::MAX,
        ..Settings::default()
    };
    Ok(read_track(fixes, &open)?
        .into_iter()
        .map(|(sample, _)| sample.descent_m_s)
        .filter(|rate| *rate > 2.0)
        .collect())
}

fn band(lower_m: f64, upper_m: f64, flights: &[FlightEntry]) -> Band {
    let mut ids = 0;
    let mut count = 0_usize;
    let mut sum = 0.0;
    let mut sum_squares = 0.0;
    let mut vector_squares = 0.0;
    for flight in flights {
        if flight.status != Status::Compared {
            continue;
        }
        let mut any = false;
        for sample in &flight.samples {
            if !(lower_m..upper_m).contains(&sample.height_m) {
                continue;
            }
            let (Some(speed), Some(vector)) = (
                sample.speed_difference_m_s(),
                sample.vector_difference_m_s(),
            ) else {
                continue;
            };
            any = true;
            count += 1;
            sum += speed;
            sum_squares += speed * speed;
            vector_squares += vector * vector;
        }
        if any {
            ids += 1;
        }
    }
    // Sample counts are far below 2^53, exact as f64.
    let n = count as f64;
    let of = |total: f64| if count == 0 { 0.0 } else { round(total) };
    Band {
        lower_m,
        upper_m,
        flights: ids,
        samples: count,
        mean_speed_difference_m_s: of(sum / n),
        rms_speed_difference_m_s: of((sum_squares / n).sqrt()),
        rms_vector_difference_m_s: of((vector_squares / n).sqrt()),
    }
}

/// The bands, and all of them together, from the flights' samples.
#[must_use]
pub fn bands(flights: &[FlightEntry]) -> (Vec<Band>, Band) {
    let bands = BAND_EDGES_M
        .windows(2)
        .map(|edges| band(edges[0], edges[1], flights))
        .collect();
    let all = band(
        BAND_EDGES_M[0],
        BAND_EDGES_M[BAND_EDGES_M.len() - 1],
        flights,
    );
    (bands, all)
}

fn era5_wind(root: &Path, flight: &RealFlight) -> Result<hpr_atmos::LayeredWind, RealFlightError> {
    let weather_path = format!("{ROCKETPY}/data/weather/{}", flight.weather);
    let bytes = read_bytes(root, &weather_path, "an ERA5 file")?;
    let (_, sounding) = real_flight::launch_day(flight, &weather_path, &bytes)?;
    sounding
        .wind()
        .cloned()
        .ok_or_else(|| RealFlightError::Input {
            flight: flight.id.to_owned(),
            what: format!("{weather_path} gives no wind"),
        })
}

fn entry(root: &Path, flight: &RealFlight, source: &Source) -> Result<FlightEntry, String> {
    let settings = Settings::default();
    let mut out = FlightEntry {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        status: Status::Compared,
        receiver: String::new(),
        fixes: 0,
        samples: Vec::new(),
    };
    let (track, not_compared) = match source {
        Source::None(reason) => {
            out.status = Status::NotRun((*reason).to_owned());
            return Ok(out);
        }
        Source::Gps {
            track,
            not_compared,
            ..
        } => (track, not_compared),
    };
    out.receiver = track.receiver.to_owned();
    let fixes = fixes(root, track, flight.site.2)?;
    out.fixes = fixes.len();
    let read = read_track(&fixes, &settings)?;
    if read.is_empty() {
        let mut rates = fall_rates(&fixes)?;
        rates.sort_by(f64::total_cmp);
        let why = match rates.get(rates.len() / 2) {
            Some(median) => format!(
                "no fix falls steadily inside the settings: the median descent rate after \
                 apogee is {median:.0} m/s, past the {:.0} m/s the method is held to, so no canopy \
                 held the fall",
                settings.max_descent_m_s
            ),
            None => "no fix falls steadily inside the settings".to_owned(),
        };
        out.status = Status::NotRun(why);
        return Ok(out);
    }
    let wind = match not_compared {
        Some(why) => {
            out.status = Status::NotCompared((*why).to_owned());
            None
        }
        None => Some(era5_wind(root, flight).map_err(|e| e.to_string())?),
    };
    for (sample, fix) in read {
        let era5 = match &wind {
            Some(wind) => {
                let at = wind.wind(fix.height_msl_m).map_err(|e| e.to_string())?;
                // A height outside ERA5's levels is held at the nearest level by HPR Sim; it is
                // not compared here.
                at.extrapolated
                    .is_none()
                    .then_some([round(at.velocity_enu_m_s.x), round(at.velocity_enu_m_s.y)])
            }
            None => None,
        };
        out.samples.push(Sample {
            time_s: round(sample.time_s),
            height_m: round(fix.height_m),
            height_msl_m: round(fix.height_msl_m),
            descent_m_s: round(sample.descent_m_s),
            ground_m_s: sample.ground_velocity_m_s.map(round),
            wind_m_s: sample.wind_m_s.map(round),
            era5_m_s: era5,
        });
    }
    Ok(out)
}

fn receivers(
    root: &Path,
    flight: &RealFlight,
    first: &Track,
    second: &Track,
) -> Result<Receivers, String> {
    let settings = Settings::default();
    let one = read_track(&fixes(root, first, flight.site.2)?, &settings)?;
    let two = read_track(&fixes(root, second, flight.site.2)?, &settings)?;
    let mut pairs = 0_usize;
    let mut squares = 0.0;
    let mut largest: f64 = 0.0;
    for (a, _) in &one {
        let Some((b, _)) = two
            .iter()
            .filter(|(b, _)| (b.time_s - a.time_s).abs() <= PAIR_S)
            .min_by(|(x, _), (y, _)| {
                (x.time_s - a.time_s)
                    .abs()
                    .total_cmp(&(y.time_s - a.time_s).abs())
            })
        else {
            continue;
        };
        let difference = (a.wind_m_s[0] - b.wind_m_s[0]).hypot(a.wind_m_s[1] - b.wind_m_s[1]);
        pairs += 1;
        squares += difference * difference;
        largest = largest.max(difference);
    }
    if pairs == 0 {
        return Err(format!(
            "{}: the two receivers share no read fix",
            flight.id
        ));
    }
    // Pair counts are far below 2^53, exact as f64.
    Ok(Receivers {
        id: flight.id.to_owned(),
        second: second.receiver.to_owned(),
        pairs,
        rms_vector_difference_m_s: round((squares / pairs as f64).sqrt()),
        max_vector_difference_m_s: round(largest),
    })
}

/// Reads every public flight and builds the report.
///
/// # Errors
///
/// A file under `refs/` that can't be read or doesn't read as its source says, or a source
/// table that doesn't list [`FLIGHTS`] in order.
pub fn run(root: &Path) -> Result<DescentWindReport, String> {
    let mut flights = Vec::new();
    let mut pairs = Vec::new();
    for (flight, (id, source)) in FLIGHTS.iter().zip(SOURCES.iter()) {
        if flight.id != *id {
            return Err(format!(
                "the sources list `{id}` where the flights list `{}`",
                flight.id
            ));
        }
        flights.push(entry(root, flight, source)?);
        if let Source::Gps {
            track,
            second: Some(second),
            ..
        } = source
        {
            pairs.push(receivers(root, flight, track, second)?);
        }
    }
    let (bands, all) = bands(&flights);
    Ok(DescentWindReport {
        settings: Settings::default(),
        flights,
        bands,
        all,
        receivers: pairs,
    })
}

impl DescentWindReport {
    /// The flights compared with ERA5.
    #[must_use]
    pub fn compared(&self) -> usize {
        self.flights
            .iter()
            .filter(|flight| flight.status == Status::Compared)
            .count()
    }

    /// Whether the bands are the ones the samples give.
    ///
    /// # Errors
    ///
    /// The first band that differs.
    pub fn check_bands(&self) -> Result<(), String> {
        let (bands, all) = bands(&self.flights);
        for (got, want) in self.bands.iter().zip(&bands) {
            if got != want {
                return Err(format!("band {got:?} is not the samples' {want:?}"));
            }
        }
        if self.bands.len() != bands.len() || self.all != all {
            return Err(format!(
                "the bands or their total ({:?}) are not the samples'",
                self.all
            ));
        }
        Ok(())
    }

    /// The report's page.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let s = &self.settings;
        let mut md = String::new();
        let _ = writeln!(md, "# Wind from the GPS descent, against ERA5\n");
        let _ = writeln!(
            md,
            "Written by `cargo xtask descent-wind` (M1.14b6); don't edit it by hand. The wind is reconstructed from each \
             public flight's GPS track of its descent as the ground velocity plus `w/g` times its \
             rate of change, `w` the descent rate (Hock and Franklin 1999; \
             [the method](../../docs/physics/wind.md#wind-from-a-gps-tracked-descent)), and set \
             beside the ERA5 wind HPR Sim flies the flight in.\n"
        );
        let compared = self.compared();
        if compared < FLIGHTS_FOR_A_COMPARISON {
            let _ = writeln!(
                md,
                "**A method, not yet a comparison.** {compared} of the public flights {} compared \
                 with ERA5; the plan asks for at least {FLIGHTS_FOR_A_COMPARISON} before the \
                 differences below are read as ERA5's miss at a launch field.\n",
                if compared == 1 { "is" } else { "are" }
            );
        }
        let _ = writeln!(
            md,
            "**Settings.** A quadratic in time fitted over {} s around each fix, at least {} \
             fixes reaching a quarter of the window on each side; a fix is read where the body \
             falls at {} to {} m/s with a vertical acceleration of at most {} m/s²; `g` = {} \
             m/s².\n",
            s.window_s,
            s.min_points,
            s.min_descent_m_s,
            s.max_descent_m_s,
            s.max_vertical_acceleration_m_s2,
            s.gravity_m_s2
        );
        let _ = writeln!(md, "## The flights\n");
        let _ = writeln!(
            md,
            "| Flight | Status | Fixes | Read | Heights read (m above the pad) | Descent (m/s) | \
             Lag correction, mean (m/s) | Speed less ERA5's, mean (m/s) |"
        );
        let _ = writeln!(md, "|---|---|---|---|---|---|---|---|");
        for flight in &self.flights {
            let status = match &flight.status {
                Status::Compared => "compared".to_owned(),
                Status::NotCompared(why) => format!("not compared: {why}"),
                Status::NotRun(why) => format!("not run: {why}"),
            };
            let samples = &flight.samples;
            let range = |f: fn(&Sample) -> f64| {
                let low = samples.iter().map(f).fold(f64::INFINITY, f64::min);
                let high = samples.iter().map(f).fold(f64::NEG_INFINITY, f64::max);
                (low, high)
            };
            let (heights, descents, correction, speed) = if samples.is_empty() {
                ("".to_owned(), "".to_owned(), "".to_owned(), "".to_owned())
            } else {
                let (h0, h1) = range(|x| x.height_m);
                let (d0, d1) = range(|x| x.descent_m_s);
                // Sample counts are far below 2^53, exact as f64.
                let n = samples.len() as f64;
                let lag = samples.iter().map(Sample::correction_m_s).sum::<f64>() / n;
                let differences: Vec<f64> = samples
                    .iter()
                    .filter_map(Sample::speed_difference_m_s)
                    .collect();
                let speed = if differences.is_empty() {
                    String::new()
                } else {
                    format!(
                        "{:+.2}",
                        differences.iter().sum::<f64>() / differences.len() as f64
                    )
                };
                (
                    format!("{h0:.0} to {h1:.0}"),
                    format!("{d0:.1} to {d1:.1}"),
                    format!("{lag:.2}"),
                    speed,
                )
            };
            let _ = writeln!(
                md,
                "| {} (`{}`) | {status} | {} | {} | {heights} | {descents} | {correction} | {speed} |",
                flight.title,
                flight.id,
                flight.fixes,
                samples.len()
            );
        }
        let _ = writeln!(md, "\nThe receivers read:\n");
        for flight in self.flights.iter().filter(|f| !f.receiver.is_empty()) {
            let _ = writeln!(md, "- `{}`: {}.", flight.id, flight.receiver);
        }
        let _ = writeln!(md, "\n## Reconstructed less ERA5, by height band\n");
        let _ = writeln!(
            md,
            "Over the compared flights. A difference in speed is the reconstructed wind's speed \
             less ERA5's; the vector difference is the length of the one wind less the other. \
             The samples of one flight are not independent: each fix's fit shares its window \
             with its neighbors', and one flight's wind and ERA5's miss are each correlated over \
             hundreds of meters.\n"
        );
        let _ = writeln!(
            md,
            "| Band (m above the pad) | Flights | Samples | Mean speed difference (m/s) | RMS \
             speed difference (m/s) | RMS vector difference (m/s) |"
        );
        let _ = writeln!(md, "|---|---|---|---|---|---|");
        for row in self.bands.iter().chain(std::iter::once(&self.all)) {
            let label = if std::ptr::eq(row, &self.all) {
                "all".to_owned()
            } else {
                format!("{:.0} to {:.0}", row.lower_m, row.upper_m)
            };
            if row.samples == 0 {
                let _ = writeln!(md, "| {label} | 0 | 0 | | | |");
            } else {
                let _ = writeln!(
                    md,
                    "| {label} | {} | {} | {:+.2} | {:.2} | {:.2} |",
                    row.flights,
                    row.samples,
                    row.mean_speed_difference_m_s,
                    row.rms_speed_difference_m_s,
                    row.rms_vector_difference_m_s
                );
            }
        }
        if !self.receivers.is_empty() {
            let _ = writeln!(md, "\n## Two receivers on one rocket\n");
            let _ = writeln!(
                md,
                "Both rode the same rocket, so the canopy's glide and swing are the same in both: \
                 their difference is the receivers' and the method's own noise.\n"
            );
            for pair in &self.receivers {
                let _ = writeln!(
                    md,
                    "- `{}`, with {}: {} fixes read by both within {PAIR_S} s; their winds differ \
                     by {:.2} m/s RMS, {:.2} m/s at most.",
                    pair.id,
                    pair.second,
                    pair.pairs,
                    pair.rms_vector_difference_m_s,
                    pair.max_vector_difference_m_s
                );
            }
        }
        let _ = writeln!(md, "\n## What it leaves out\n");
        let _ = writeln!(
            md,
            "- **The canopy's own motion.** A canopy that glides moves across the wind by `w tan \
             γ` at a glide angle `γ`; a swing on the shock cord moves the rocket back and forth. \
             Neither is measured for hobby canopies, so neither is bounded: the differences above \
             hold them along with ERA5's miss.\n\
             - **ERA5's hour and place.** ERA5 is read at the hour the real-flights report flies \
             the flight in and at the launch site; the descent is minutes later and drifts \
             hundreds of meters.\n\
             - **Drogues and tumbling bodies.** The lag `w/g` assumes the same drag in every \
             direction; a tumbling airframe or a streamer has no measured response."
        );
        md
    }

    /// Whether `markdown` is this report's page.
    ///
    /// # Errors
    ///
    /// When it isn't.
    pub fn check_page(&self, markdown: &str) -> Result<(), String> {
        if markdown == self.to_markdown() {
            Ok(())
        } else {
            Err(format!(
                "{REPORT_MD} is not the page of {REPORT_JSON}: run `cargo xtask descent-wind`"
            ))
        }
    }
}
