//! The wind reconstructed from each public flight's GPS-tracked descent, set beside ERA5's
//! ([M1.14b6, wind from the GPS descent][m1-14b6]), written by `cargo xtask descent-wind` to
//! `validation/reports/descent-wind.json` and `.md`.
//!
//! - **The flights** ([`required_ids`] and the report's table) are every public flight HPR Sim
//!   flies or predicts, and RocketPy's other example logs: the seven the real-flights report
//!   flies, the five public flights predicted before their logs were compared, and four more
//!   public example logs in RocketPy's checkout whose rockets HPR Sim doesn't fly (Camões,
//!   Halcyon, Hedy, Valkyrie). The wind needs no motor, so each with a position is read. Each is
//!   listed: read, with its receiver, or not, with the reason.
//! - **Not read yet:** two public team logs in the [public-flights note][note] with GPS fixes in
//!   the descent, Hyperion-II's and UTRGV's ([issue #1605][i1605]); the report says so.
//! - **The reconstruction** is [`hpr_flightdata::descent_wind`] at its default settings.
//! - **ERA5** is read at the launch site and hour as HPR Sim reads a sounding
//!   ([`real_flight::launch_day`]): for a flight the real-flights report flies, its file and
//!   hour; for the other example logs, the file and hour their notebooks give. It is
//!   interpolated by components to each fix's height above sea level, as read from the log. A
//!   flight with no ERA5 file of its day is reconstructed and not compared.
//! - **What is committed.** Each compared flight's sums by height band, from which CI recomputes
//!   the pooled bands. Each fix's wind is committed only for logs read whole from RocketPy's MIT
//!   repository: the EuRoC logs of CATS's flight computers give statistics only, since CATS
//!   publishes EuRoC logs under GPL-3.0 and three of RocketPy's copies are files of
//!   RocketPy-Team's GPL-3.0 fork of its 2023 repository (`THIRD-PARTY-NOTICES.md`,
//!   `rocketpy`).
//! - **Two receivers.** Prometheus carried two Altus Metrum flight computers, each with its own
//!   GPS receiver, and the report gives how far their two winds differ: a floor on the noise
//!   between two receivers, not the method's error, since a lag model's error or a motion both
//!   receivers share cancels in their difference.
//!
//! The logs and ERA5 files are read from the pinned RocketPy checkout under the gitignored
//! `refs/`, so the report is regenerated and checked where that checkout is (the gate's `refs`
//! step). CI, without it, recomputes the bands from the committed sums and the page from the
//! committed report (`hpr_validate::tests`).
//!
//! [m1-14b6]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b6
//! [note]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/public-flights.md
//! [i1605]: https://github.com/nrdptel/fusionspace-eridanus/issues/1605

use std::fmt::Write as _;
use std::path::Path;

use hpr_atmos::Wind as _;
use hpr_core::geodesy::Geodetic;
use hpr_flightdata::descent_wind::{Settings, TrackPoint, WindSample, reconstruct};
use serde::{Deserialize, Serialize};

use crate::public_predictions::{DESIGN_FILE_FLIGHTS, NEW_FLIGHTS};
use crate::real_flight::{self, ERA5_CREDIT, FLIGHTS, ROCKETPY, read_bytes};

/// The committed report.
pub const REPORT_JSON: &str = "validation/reports/descent-wind.json";
/// Its page.
pub const REPORT_MD: &str = "validation/reports/descent-wind.md";

/// The edges between the height bands the comparison is pooled in, m above the pad: the first
/// band is everything below the first edge, the last everything from the last edge up.
pub const BAND_EDGES_M: [f64; 4] = [500.0, 1000.0, 2000.0, 3000.0];

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

/// A bound kept to the report's four decimal places, rounded up to hundredths for print: 1.344 prints as 1.35,
/// and 0.07 stays 0.07 (taken in ten-thousandths first, so `0.07 × 100`'s binary excess doesn't
/// round it up).
#[must_use]
pub fn up_to_hundredths(value: f64) -> f64 {
    ((value * 1e4).round() / 1e2).ceil() / 1e2
}

/// How a track's heights are read.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Height {
    /// The receiver's own altitude above sea level, in a column of the track's file, with meters
    /// per unit. The pad is the first fix's.
    Gnss { column: usize, meters_per_unit: f64 },
    /// A barometric height above the pad from a file on the same clock, interpolated linearly to
    /// each fix's time; above sea level it is that plus the site's elevation.
    Barometric {
        file: &'static str,
        time_column: usize,
        height_column: usize,
    },
}

/// How a new fix is told from a repeated row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NewFix {
    /// A row is a new fix when the receiver's own position differs from the last fix's: its
    /// latitude and longitude, and its altitude where the track reads the receiver's
    /// ([`Height::Gnss`]). A receiver repeats its last fix between updates and after it loses
    /// lock, so a run of repeats is a gap in the track.
    PositionChanges,
    /// AltOS's CSV repeats the last fix on every sensor row; a row is a new fix when its GPS
    /// clock (hour, minute, second columns) changes and its `locked` column is 1. The fix is
    /// timed on the GPS clock, from the log's time at the first fix, since the log's rows fall
    /// up to a quarter of a second off the receiver's second.
    GpsClock { clock: [usize; 3], locked: usize },
}

/// Where a flight's GPS track is and how to read it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Track {
    /// The receiver, in words.
    receiver: &'static str,
    /// The CSV file, under `refs/rocketpy/data/rockets/`.
    file: &'static str,
    /// The column of time, s (zero-based).
    time_column: usize,
    /// The columns of latitude and longitude, degrees (zero-based).
    position_columns: [usize; 2],
    /// The heights.
    height: Height,
    /// How a fix is told from a repeat.
    new_fix: NewFix,
}

/// What a public flight's log holds for the descent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Source {
    /// No GPS track of the descent: the reason.
    None(&'static str),
    /// A GPS track, and a second receiver's on the same rocket, if one was carried.
    Gps {
        track: &'static Track,
        second: Option<&'static Track>,
        /// Whether each fix's wind may be committed.
        publish_fixes: bool,
    },
}

/// Where a flight's site, hour and ERA5 file come from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Day {
    /// The real-flights report's: [`real_flight::FLIGHTS`], by id.
    RealFlights,
    /// The public predictions': title and site from `public_predictions`, by id. Their weather
    /// is not read here, so a flight that reads is reconstructed, not compared: why.
    Predicted { not_compared: &'static str },
    /// A public log HPR Sim doesn't fly and that holds no position, so no site or weather is
    /// read.
    LogOnly,
    /// A public log HPR Sim doesn't fly: its notebook's site (latitude and longitude, degrees;
    /// elevation, m), launch hour in UTC and ERA5 file, under `refs/rocketpy/data/weather/`.
    Notebook {
        site: (f64, f64, f64),
        utc: (i32, u32, u32, u32),
        weather: &'static str,
        source: &'static str,
    },
}

/// One public flight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Public {
    id: &'static str,
    /// The title, for a flight the other tables don't give one.
    title: &'static str,
    day: Day,
    source: Source,
    /// Why the flight's ERA5 file can't be compared with its track, where it can't.
    not_compared: Option<&'static str>,
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

/// A CATS flight computer's EuRoC log: a GNSS file of time, latitude and longitude, and a
/// filtered barometric height above the pad in another, both on the flight computer's clock.
const fn cats(receiver: &'static str, gnss: &'static str, heights: &'static str) -> Track {
    Track {
        receiver,
        file: gnss,
        time_column: 0,
        position_columns: [1, 2],
        height: Height::Barometric {
            file: heights,
            time_column: 0,
            height_column: 1,
        },
        new_fix: NewFix::PositionChanges,
    }
}

const GENESIS: Track = cats(
    "the GNSS log (`gnss_faraday.csv`): latitude and longitude, 1 to 10 fixes a second; height \
     from the flight log's barometric filter",
    "genesis/gnss_faraday.csv",
    "genesis/flight_data_faraday.csv",
);

const CAMOES: Track = cats(
    "the CATS flight computer's GNSS log (`gnss_flight.csv`): latitude and longitude; height \
     from its barometric filter (`flight_data.csv`)",
    "camoes/gnss_flight.csv",
    "camoes/flight_data.csv",
);

const HALCYON: Track = cats(
    "the CATS flight computer's GNSS log (`gnss_halcyon.csv`): latitude and longitude; height \
     from its barometric filter (`altimeter_halcyon.csv`)",
    "astg/gnss_halcyon.csv",
    "astg/altimeter_halcyon.csv",
);

const HEDY: Track = cats(
    "the CATS flight computer's GNSS log (`gnssInfo.csv`): latitude and longitude; height from \
     its barometric filter (`filteredDataInfo.csv`)",
    "hedy/cats_tust/gnssInfo.csv",
    "hedy/cats_tust/filteredDataInfo.csv",
);

const MACIE_XL: &str = "its logs hold heights, speeds and accelerations, no position; the \
                        positions in the OpenRocket file beside them are a simulation's";

const PREDICTED_NOT_COMPARED: &str = "the public predictions' weather, EuRoC 2022's converted \
                                      ERA5 file, is not read here";

/// The report's flights, in its order: the real-flights report's, the predictions', then the
/// example logs HPR Sim doesn't fly.
pub(crate) const PUBLIC: [Public; 16] = [
    Public {
        id: "bella-lui",
        title: "",
        day: Day::RealFlights,
        source: Source::None("its log holds a filtered height and speed only, no position"),
        not_compared: None,
    },
    Public {
        id: "ndrt-2020",
        title: "",
        day: Day::RealFlights,
        source: Source::None(
            "its log holds acceleration and a barometric height only, no position",
        ),
        not_compared: None,
    },
    Public {
        id: "prometheus-2022",
        title: "",
        day: Day::RealFlights,
        source: Source::Gps {
            track: &TELEMETRUM,
            second: Some(&TELEMEGA),
            publish_fixes: true,
        },
        not_compared: Some(
            "RocketPy's ERA5 file for it is of 24 June 2023, a year after the flight, and no \
             file of its day is pinned",
        ),
    },
    Public {
        id: "juno-iii",
        title: "",
        day: Day::RealFlights,
        source: Source::Gps {
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
            publish_fixes: true,
        },
        not_compared: None,
    },
    Public {
        id: "cavour",
        title: "",
        day: Day::RealFlights,
        source: Source::Gps {
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
            publish_fixes: false,
        },
        not_compared: None,
    },
    Public {
        id: "genesis",
        title: "",
        day: Day::RealFlights,
        source: Source::Gps {
            track: &GENESIS,
            second: None,
            publish_fixes: false,
        },
        not_compared: None,
    },
    Public {
        id: "lince",
        title: "",
        day: Day::RealFlights,
        source: Source::None("its log holds a filtered height and acceleration only, no position"),
        not_compared: None,
    },
    Public {
        id: "astra",
        title: "",
        day: Day::Predicted {
            not_compared: PREDICTED_NOT_COMPARED,
        },
        source: Source::Gps {
            track: &Track {
                receiver: "the flight log (`flight_data.csv`): latitude and longitude to four \
                           decimals about once a second; height from its `alt` column, read as \
                           barometric above the pad",
                file: "astra/flight_data.csv",
                time_column: 0,
                position_columns: [2, 3],
                height: Height::Barometric {
                    file: "astra/flight_data.csv",
                    time_column: 0,
                    height_column: 1,
                },
                new_fix: NewFix::PositionChanges,
            },
            second: None,
            publish_fixes: true,
        },
        not_compared: None,
    },
    Public {
        id: "andromeda",
        title: "",
        day: Day::Predicted {
            not_compared: PREDICTED_NOT_COMPARED,
        },
        source: Source::None("its log holds a height and a vertical speed only, no position"),
        not_compared: None,
    },
    Public {
        id: "erebus-11",
        title: "",
        day: Day::Predicted {
            not_compared: PREDICTED_NOT_COMPARED,
        },
        source: Source::None("its log holds a filtered height only, no position"),
        not_compared: None,
    },
    Public {
        id: "macie-xl-1",
        title: "",
        day: Day::Predicted { not_compared: "" },
        source: Source::None(MACIE_XL),
        not_compared: None,
    },
    Public {
        id: "macie-xl-2",
        title: "",
        day: Day::Predicted { not_compared: "" },
        source: Source::None(MACIE_XL),
        not_compared: None,
    },
    Public {
        id: "camoes",
        title: "Camões, EuRoC 2023 (the team's own motor, not flown by HPR Sim)",
        day: Day::Notebook {
            site: (39.390_003_204_345_7, -8.289_538_383_483_8, 107.0),
            utc: (2023, 10, 14, 13),
            weather: "euroc_2023_all_windows.nc",
            source: "camoes_flight_sim.ipynb's environment: 14:00 Portugal time, UTC+1",
        },
        source: Source::Gps {
            track: &CAMOES,
            second: None,
            publish_fixes: false,
        },
        not_compared: None,
    },
    Public {
        id: "halcyon",
        title: "Halcyon, EuRoC 2023 (the team's own motor, not flown by HPR Sim)",
        day: Day::Notebook {
            site: (39.388_692, -8.287_814, 130.0),
            utc: (2023, 10, 13, 13),
            weather: "euroc_2023_all_windows.nc",
            source: "halcyon_flight_sim.ipynb's environment: 14:00 Portugal time, UTC+1",
        },
        source: Source::Gps {
            track: &HALCYON,
            second: None,
            publish_fixes: false,
        },
        not_compared: None,
    },
    Public {
        id: "hedy",
        title: "Hedy, EuRoC 2025 (the team's own motor, not flown by HPR Sim)",
        day: Day::Notebook {
            site: (39.390_460_968_017_58, -8.288_534_164_428_711, 160.0),
            utc: (2025, 10, 12, 16),
            weather: "hedy_euroc_2025_weather_data.nc",
            source: "hedy_flight_sim.ipynb's environment: 16:00 UTC, and the site's elevation \
                     as the notebook prints it",
        },
        source: Source::Gps {
            track: &HEDY,
            second: None,
            publish_fixes: false,
        },
        not_compared: Some(
            "its ERA5 file is netCDF-4, which HPR Sim reads only through a converted copy checked \
             against the source, as EuRoC 2022's is; no such copy is made for this file yet",
        ),
    },
    Public {
        id: "valkyrie",
        title: "Valkyrie, 2025 (not flown by HPR Sim)",
        day: Day::LogOnly,
        source: Source::None("its log holds a height only, no position"),
        not_compared: None,
    },
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

/// One read fix of a descent, where its log's fixes may be committed.
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
    /// Its rate of change, east and north, m/s².
    pub ground_acceleration_m_s2: [f64; 2],
    /// The reconstructed wind, east and north, m/s.
    pub wind_m_s: [f64; 2],
    /// ERA5's wind at the same height, east and north, m/s, where compared.
    pub era5_m_s: Option<[f64; 2]>,
}

/// Sums over one flight's compared fixes in one band, from which a band's mean and
/// root-mean-square differences are recomputed.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Sums {
    /// The fixes.
    pub count: usize,
    /// The sum of the reconstructed speed less ERA5's, m/s.
    pub speed: f64,
    /// The sum of its squares, m²/s².
    pub speed_squares: f64,
    /// The sum of the squared lengths of the reconstructed wind less ERA5's, m²/s².
    pub vector_squares: f64,
}

impl Sums {
    fn add(&mut self, speed: f64, vector: f64) {
        self.count += 1;
        self.speed += speed;
        self.speed_squares += speed * speed;
        self.vector_squares += vector * vector;
    }

    fn join(&mut self, other: &Self) {
        self.count += other.count;
        self.speed += other.speed;
        self.speed_squares += other.speed_squares;
        self.vector_squares += other.vector_squares;
    }

    fn rounded(self) -> Self {
        Self {
            count: self.count,
            speed: round(self.speed),
            speed_squares: round(self.speed_squares),
            vector_squares: round(self.vector_squares),
        }
    }
}

/// One public flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightEntry {
    /// The flight's id.
    pub id: String,
    /// The rocket, the team and the event.
    pub title: String,
    /// Where it stands.
    pub status: Status,
    /// The receiver read, in words; empty when none.
    pub receiver: String,
    /// Where its site, launch hour and ERA5 file come from, when not from the real-flights
    /// report; empty otherwise.
    pub day: String,
    /// The fixes the track holds.
    pub fixes: usize,
    /// The rows dropped as a repeat of the fix before.
    pub repeats: usize,
    /// The longest time between two fixes, s.
    pub longest_gap_s: f64,
    /// The fixes read as a steady fall.
    pub read: usize,
    /// The lowest and highest of them, m above the pad.
    pub heights_m: Option<[f64; 2]>,
    /// Their slowest and fastest descent, m/s.
    pub descent_m_s: Option<[f64; 2]>,
    /// The mean length of what the lag adds to the ground velocity, m/s.
    pub mean_lag_correction_m_s: Option<f64>,
    /// The read fixes whose height lies outside ERA5's levels, and so are not compared.
    pub outside_era5: usize,
    /// The comparison's sums in each band, in [`BAND_EDGES_M`]' order; empty when not compared.
    pub bands: Vec<Sums>,
    /// Each read fix, where its log's fixes may be committed; empty otherwise.
    pub samples: Vec<Sample>,
}

/// The comparison pooled over one height band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Band {
    /// The band, in words, m above the pad.
    pub label: String,
    /// The compared flights with a fix in it.
    pub flights: usize,
    /// Their fixes in it.
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
    /// Each receiver's highest fix, m above sea level.
    pub highest_m: [f64; 2],
    /// The distance between the two receivers' last fixes, m.
    pub last_fixes_apart_m: f64,
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
    /// The report's flights: every one HPR Sim flies or predicts, then RocketPy's other example
    /// logs.
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
    let value = row
        .get(column)
        .ok_or_else(|| format!("{file}:{line}: no column {column}"))?
        .parse::<f64>()
        .map_err(|error| format!("{file}:{line}: column {column}: {error}"))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!("{file}:{line}: column {column} is {value}"))
    }
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

/// A track's fixes, in time order, each new, and the rows dropped as repeats.
fn fixes(root: &Path, track: &Track, elevation_m: f64) -> Result<(Vec<Fix>, usize), String> {
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
    let mut repeats = 0;
    let mut last_clock: Option<f64> = None;
    let mut clock_origin: Option<(f64, f64)> = None;
    let mut pad_msl: Option<f64> = None;
    for (index, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let row = columns(line);
        let at = |column| number(&row, column, track.file, index + 1);
        let mut time_s = at(track.time_column)?;
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
            let now = at(clock[0])? * 3600.0 + at(clock[1])? * 60.0 + at(clock[2])?;
            if last_clock == Some(now) {
                repeats += 1;
                continue;
            }
            last_clock = Some(now);
            let (log_at_first, clock_at_first) = *clock_origin.get_or_insert((time_s, now));
            time_s = log_at_first + (now - clock_at_first);
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
                repeats += 1;
                continue;
            }
        }
        out.push(fix);
    }
    Ok((out, repeats))
}

/// The steady-fall samples of a track, with their fixes.
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

/// The descent rates over 2 m/s that a track's fitted windows give, steady or not: what the
/// report says of a track with no steady fall.
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

/// The band a height above the pad falls in.
fn band_of(height_m: f64) -> usize {
    BAND_EDGES_M.partition_point(|edge| *edge <= height_m)
}

fn band_label(index: usize) -> String {
    let words = |m: f64| {
        if m >= 1000.0 {
            // The edges are whole hundreds of meters.
            format!("{:.0},{:03.0}", (m / 1000.0).floor(), m % 1000.0)
        } else {
            format!("{m:.0}")
        }
    };
    let lower = index.checked_sub(1).and_then(|i| BAND_EDGES_M.get(i));
    match (lower, BAND_EDGES_M.get(index)) {
        (None, Some(upper)) => format!("below {}", words(*upper)),
        (Some(lower), Some(upper)) => format!("{} to {}", words(*lower), words(*upper)),
        (Some(lower), None) => format!("{} and up", words(*lower)),
        (None, None) => String::new(),
    }
}

fn pooled(label: String, sums: &[Sums]) -> Band {
    let mut total = Sums::default();
    for one in sums {
        total.join(one);
    }
    // Sample counts are far below 2^53, exact as f64.
    let n = total.count as f64;
    let of = |value: f64| if total.count == 0 { 0.0 } else { round(value) };
    Band {
        label,
        flights: sums.iter().filter(|one| one.count > 0).count(),
        samples: total.count,
        mean_speed_difference_m_s: of(total.speed / n),
        rms_speed_difference_m_s: of((total.speed_squares / n).sqrt()),
        rms_vector_difference_m_s: of((total.vector_squares / n).sqrt()),
    }
}

/// The bands, and all of them together, from the compared flights' sums.
#[must_use]
pub fn bands(flights: &[FlightEntry]) -> (Vec<Band>, Band) {
    let compared: Vec<&FlightEntry> = flights
        .iter()
        .filter(|flight| flight.status == Status::Compared)
        .collect();
    let bands = (0..=BAND_EDGES_M.len())
        .map(|index| {
            let sums: Vec<Sums> = compared
                .iter()
                .map(|flight| flight.bands.get(index).copied().unwrap_or_default())
                .collect();
            pooled(band_label(index), &sums)
        })
        .collect();
    let totals: Vec<Sums> = compared
        .iter()
        .map(|flight| {
            let mut total = Sums::default();
            for one in &flight.bands {
                total.join(one);
            }
            total
        })
        .collect();
    (bands, pooled("all".to_owned(), &totals))
}

fn real_flight_of(id: &str) -> Result<&'static real_flight::RealFlight, String> {
    FLIGHTS
        .iter()
        .find(|flight| flight.id == id)
        .ok_or_else(|| format!("`{id}` is not among the real-flights report's flights"))
}

/// The flight's ERA5 wind, or why it is not compared.
fn era5_wind(
    root: &Path,
    public: &Public,
) -> Result<Result<hpr_atmos::LayeredWind, String>, String> {
    let (site, utc, weather) = match public.day {
        Day::RealFlights => {
            let flight = real_flight_of(public.id)?;
            (flight.site, flight.utc, flight.weather)
        }
        Day::Predicted { not_compared } => return Ok(Err(not_compared.to_owned())),
        Day::Notebook {
            site, utc, weather, ..
        } => (site, utc, weather),
        Day::LogOnly => return Ok(Err("its log holds no position".to_owned())),
    };
    let weather_path = format!("{ROCKETPY}/data/weather/{weather}");
    let bytes = read_bytes(root, &weather_path, "an ERA5 file").map_err(|e| e.to_string())?;
    let (_, sounding) = real_flight::launch_day_at(public.id, site, utc, &weather_path, &bytes)
        .map_err(|e| e.to_string())?;
    let wind = sounding
        .wind()
        .cloned()
        .ok_or_else(|| format!("{weather_path} gives no wind"))?;
    Ok(Ok(wind))
}

/// A public flight's title and its site's elevation, m.
fn title_and_elevation(public: &Public) -> Result<(String, f64), String> {
    match public.day {
        Day::RealFlights => {
            let flight = real_flight_of(public.id)?;
            Ok((flight.title.to_owned(), flight.site.2))
        }
        Day::Predicted { .. } => {
            if let Some(flight) = NEW_FLIGHTS.iter().find(|f| f.id == public.id) {
                return Ok((flight.title.to_owned(), flight.site.2));
            }
            DESIGN_FILE_FLIGHTS
                .iter()
                .find(|f| f.id == public.id)
                // A design-file flight's logs hold no position, so it is never read and its
                // site's elevation is never used.
                .map(|f| (f.title.to_owned(), 0.0))
                .ok_or_else(|| format!("`{}` is not among the public predictions", public.id))
        }
        Day::Notebook { site, .. } => Ok((public.title.to_owned(), site.2)),
        // A log with no position is never read, so its site's elevation is never used.
        Day::LogOnly => Ok((public.title.to_owned(), 0.0)),
    }
}

fn entry(root: &Path, public: &Public) -> Result<FlightEntry, String> {
    let settings = Settings::default();
    let (title, elevation_m) = title_and_elevation(public)?;
    let mut out = FlightEntry {
        id: public.id.to_owned(),
        title,
        status: Status::Compared,
        receiver: String::new(),
        day: match public.day {
            Day::Notebook { source, .. } => source.to_owned(),
            _ => String::new(),
        },
        fixes: 0,
        repeats: 0,
        longest_gap_s: 0.0,
        read: 0,
        heights_m: None,
        descent_m_s: None,
        mean_lag_correction_m_s: None,
        outside_era5: 0,
        bands: Vec::new(),
        samples: Vec::new(),
    };
    let (track, publish_fixes) = match public.source {
        Source::None(reason) => {
            out.status = Status::NotRun(reason.to_owned());
            return Ok(out);
        }
        Source::Gps {
            track,
            publish_fixes,
            ..
        } => (track, publish_fixes),
    };
    out.receiver = track.receiver.to_owned();
    let (fixes, repeats) = fixes(root, track, elevation_m)?;
    out.fixes = fixes.len();
    out.repeats = repeats;
    out.longest_gap_s = round(
        fixes
            .windows(2)
            .map(|pair| pair[1].time_s - pair[0].time_s)
            .fold(0.0, f64::max),
    );
    let read = read_track(&fixes, &settings)?;
    out.read = read.len();
    if read.is_empty() {
        let mut rates = fall_rates(&fixes)?;
        rates.sort_by(f64::total_cmp);
        let why = match rates.get(rates.len() / 2) {
            Some(median) if *median > settings.max_descent_m_s => format!(
                "no fix falls steadily inside the settings: the median descent rate the fitted \
                 windows give is {median:.0} m/s, past the {:.0} m/s the method is held to",
                settings.max_descent_m_s
            ),
            Some(median) => format!(
                "no fix falls steadily inside the settings: the median descent rate the fitted \
                 windows give is {median:.0} m/s, but no window is steady enough or reaches both \
                 sides"
            ),
            None => "no fitted window falls faster than 2 m/s".to_owned(),
        };
        out.status = Status::NotRun(why);
        return Ok(out);
    }
    let range = |f: &dyn Fn(&(WindSample, Fix)) -> f64| {
        let low = read.iter().map(f).fold(f64::INFINITY, f64::min);
        let high = read.iter().map(f).fold(f64::NEG_INFINITY, f64::max);
        [round(low), round(high)]
    };
    out.heights_m = Some(range(&|(_, fix)| fix.height_m));
    out.descent_m_s = Some(range(&|(sample, _)| sample.descent_m_s));
    let lag = |sample: &WindSample| {
        (sample.wind_m_s[0] - sample.ground_velocity_m_s[0])
            .hypot(sample.wind_m_s[1] - sample.ground_velocity_m_s[1])
    };
    // Read counts are far below 2^53, exact as f64.
    out.mean_lag_correction_m_s = Some(round(
        read.iter().map(|(sample, _)| lag(sample)).sum::<f64>() / read.len() as f64,
    ));
    let wind = match public.not_compared {
        Some(why) => Err(why.to_owned()),
        None => era5_wind(root, public)?,
    };
    let wind = match wind {
        Ok(wind) => Some(wind),
        Err(why) => {
            out.status = Status::NotCompared(why);
            None
        }
    };
    let mut sums = vec![Sums::default(); BAND_EDGES_M.len() + 1];
    for (sample, fix) in &read {
        let era5 = match &wind {
            Some(wind) => {
                let at = wind.wind(fix.height_msl_m).map_err(|e| e.to_string())?;
                // A height outside ERA5's levels is held at the nearest level by HPR Sim; it is
                // not compared here.
                if at.extrapolated.is_some() {
                    out.outside_era5 += 1;
                    None
                } else {
                    Some([at.velocity_enu_m_s.x, at.velocity_enu_m_s.y])
                }
            }
            None => None,
        };
        if let Some(era5) = era5 {
            let speed = sample.wind_m_s[0].hypot(sample.wind_m_s[1]) - era5[0].hypot(era5[1]);
            let vector = (sample.wind_m_s[0] - era5[0]).hypot(sample.wind_m_s[1] - era5[1]);
            sums[band_of(fix.height_m)].add(speed, vector);
        }
        if publish_fixes {
            out.samples.push(Sample {
                time_s: round(sample.time_s),
                height_m: round(fix.height_m),
                height_msl_m: round(fix.height_msl_m),
                descent_m_s: round(sample.descent_m_s),
                ground_m_s: sample.ground_velocity_m_s.map(round),
                ground_acceleration_m_s2: sample.ground_acceleration_m_s2.map(round),
                wind_m_s: sample.wind_m_s.map(round),
                era5_m_s: era5.map(|e| e.map(round)),
            });
        }
    }
    if wind.is_some() {
        out.bands = sums.into_iter().map(Sums::rounded).collect();
    }
    Ok(out)
}

fn receivers(
    root: &Path,
    public: &Public,
    first: &Track,
    second: &Track,
) -> Result<Receivers, String> {
    let settings = Settings::default();
    let (_, elevation_m) = title_and_elevation(public)?;
    let (one_fixes, _) = fixes(root, first, elevation_m)?;
    let (two_fixes, _) = fixes(root, second, elevation_m)?;
    let highest = |fixes: &[Fix]| {
        round(
            fixes
                .iter()
                .map(|fix| fix.height_msl_m)
                .fold(f64::NEG_INFINITY, f64::max),
        )
    };
    let (Some(a), Some(b)) = (one_fixes.last(), two_fixes.last()) else {
        return Err(format!("{}: a receiver holds no fix", public.id));
    };
    let origin = Geodetic::from_degrees(a.latitude_deg, a.longitude_deg, a.height_msl_m)
        .map_err(|e| e.to_string())?;
    let other = Geodetic::from_degrees(b.latitude_deg, b.longitude_deg, b.height_msl_m)
        .map_err(|e| e.to_string())?;
    let apart = TrackPoint::from_geodetic(0.0, origin, other);
    let one = read_track(&one_fixes, &settings)?;
    let two = read_track(&two_fixes, &settings)?;
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
            public.id
        ));
    }
    // Pair counts are far below 2^53, exact as f64.
    Ok(Receivers {
        id: public.id.to_owned(),
        second: second.receiver.to_owned(),
        highest_m: [highest(&one_fixes), highest(&two_fixes)],
        last_fixes_apart_m: round(apart.east_m.hypot(apart.north_m)),
        pairs,
        rms_vector_difference_m_s: round((squares / pairs as f64).sqrt()),
        max_vector_difference_m_s: round(largest),
    })
}

/// The ids of the report's table, in its order.
#[must_use]
pub fn table_ids() -> Vec<&'static str> {
    PUBLIC.iter().map(|public| public.id).collect()
}

/// The ids of the public flights the report must list: the real-flights report's and the public
/// predictions', in that order.
#[must_use]
pub fn required_ids() -> Vec<&'static str> {
    FLIGHTS
        .iter()
        .map(|flight| flight.id)
        .chain(NEW_FLIGHTS.iter().map(|flight| flight.id))
        .chain(DESIGN_FILE_FLIGHTS.iter().map(|flight| flight.id))
        .collect()
}

/// Reads the report's flights and builds the report.
///
/// # Errors
///
/// A file under `refs/` that can't be read or doesn't read as its source says, or a table that
/// doesn't list the [`required_ids`] first, in order.
pub fn run(root: &Path) -> Result<DescentWindReport, String> {
    let required = required_ids();
    let listed: Vec<&str> = PUBLIC.iter().map(|public| public.id).collect();
    if listed.get(..required.len()) != Some(required.as_slice()) {
        return Err(format!(
            "the report's table lists {listed:?}; it must start with the public flights \
             {required:?}"
        ));
    }
    let mut flights = Vec::new();
    let mut pairs = Vec::new();
    for public in &PUBLIC {
        flights.push(entry(root, public)?);
        if let Source::Gps {
            track,
            second: Some(second),
            ..
        } = public.source
        {
            pairs.push(receivers(root, public, track, second)?);
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

    /// Whether the bands are the ones the flights' sums give.
    ///
    /// # Errors
    ///
    /// When they aren't.
    pub fn check_bands(&self) -> Result<(), String> {
        for flight in &self.flights {
            let expected = if flight.status == Status::Compared {
                BAND_EDGES_M.len() + 1
            } else {
                0
            };
            if flight.bands.len() != expected {
                return Err(format!(
                    "`{}` holds {} bands' sums, not {expected}",
                    flight.id,
                    flight.bands.len()
                ));
            }
        }
        let (bands, all) = bands(&self.flights);
        if self.bands != bands {
            return Err(format!(
                "the bands {:?} are not the flights' sums' {bands:?}",
                self.bands
            ));
        }
        if self.all != all {
            return Err(format!(
                "the total {:?} is not the flights' sums' {all:?}",
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
            "Written by `cargo xtask descent-wind` (M1.14b6); don't edit it by hand. The wind is \
             reconstructed from each public flight's GPS track of its descent as the ground \
             velocity plus `w/g` times its rate of change, `w` the descent rate (Hock and \
             Franklin 1999; [the method](../../docs/physics/wind.md#wind-from-a-gps-tracked-descent)). \
             It is set beside ERA5's wind at each fix's height above sea level, read at the \
             launch site and hour. A fix is one position the receiver reported.\n"
        );
        let compared = self.compared();
        if compared < FLIGHTS_FOR_A_COMPARISON {
            let _ = writeln!(
                md,
                "**A method, not yet a comparison.** {compared} of the public flights {} compared \
                 with ERA5; the plan asks for at least {FLIGHTS_FOR_A_COMPARISON} before the \
                 differences below are read as a comparison.\n",
                if compared == 1 { "is" } else { "are" }
            );
        } else {
            let _ = writeln!(
                md,
                "**{compared} public flights compared.** The differences below hold ERA5's miss at \
                 the field and whatever the canopy did, which nothing here tells apart.\n"
            );
        }
        let _ = writeln!(
            md,
            "**Settings.** A quadratic in time fitted over {} s around each fix, at least {} \
             fixes reaching a quarter of the window on each side. A fix is read where the body \
             falls at {} to {} m/s with a vertical acceleration of at most {} m/s². `g` = {} \
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
            "Repeats are rows dropped as a repeat of the fix before; read fixes are those read as \
             a steady fall. The lag correction is the mean length of what `w/g` times the ground \
             velocity's rate of change adds to it.\n"
        );
        let _ = writeln!(
            md,
            "| Flight | Status | Fixes | Repeats | Longest gap (s) | Read | Heights read (m above \
             the pad) | Descent (m/s) | Lag correction (m/s) | Speed less ERA5's, mean (m/s) | \
             Vector difference, RMS (m/s) |"
        );
        let _ = writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|");
        for flight in &self.flights {
            let status = match &flight.status {
                Status::Compared => "compared".to_owned(),
                Status::NotCompared(why) => format!("not compared: {why}"),
                Status::NotRun(why) => format!("not run: {why}"),
            };
            let pair = |range: Option<[f64; 2]>, digits: usize| {
                range.map_or(String::new(), |[low, high]| {
                    format!("{low:.digits$} to {high:.digits$}")
                })
            };
            let mut total = Sums::default();
            for one in &flight.bands {
                total.join(one);
            }
            // Sample counts are far below 2^53, exact as f64.
            let (speed, vector) = if total.count == 0 {
                (String::new(), String::new())
            } else {
                let n = total.count as f64;
                (
                    format!("{:+.2}", total.speed / n),
                    format!("{:.2}", (total.vector_squares / n).sqrt()),
                )
            };
            let counted = |n: usize| {
                if flight.receiver.is_empty() {
                    String::new()
                } else {
                    n.to_string()
                }
            };
            let gap = if flight.fixes < 2 {
                String::new()
            } else {
                format!("{:.1}", flight.longest_gap_s)
            };
            let _ = writeln!(
                md,
                "| {} (`{}`) | {status} | {} | {} | {gap} | {} | {} | {} | {} | {speed} | {vector} |",
                flight.title,
                flight.id,
                counted(flight.fixes),
                counted(flight.repeats),
                counted(flight.read),
                pair(flight.heights_m, 0),
                pair(flight.descent_m_s, 1),
                flight
                    .mean_lag_correction_m_s
                    .map_or(String::new(), |lag| format!("{lag:.2}")),
            );
        }
        let _ = writeln!(md, "\nThe receivers read:\n");
        for flight in self.flights.iter().filter(|f| !f.receiver.is_empty()) {
            let _ = write!(md, "- `{}`: {}", flight.id, flight.receiver);
            if flight.day.is_empty() {
                let _ = writeln!(md, ".");
            } else {
                let _ = writeln!(md, ". Its site and hour: {}.", flight.day);
            }
        }
        let outside: usize = self.flights.iter().map(|f| f.outside_era5).sum();
        let _ = writeln!(md, "\n## Reconstructed less ERA5, by height band\n");
        let _ = writeln!(
            md,
            "Over the compared flights. A difference in speed is the reconstructed wind's speed \
             less ERA5's; the vector difference is the length of the one wind less the other. \
             Neighboring fixes share their fit's window, so the fixes of one flight are not \
             independent. {outside} read {} outside ERA5's levels and {} not compared.\n",
            if outside == 1 {
                "fix lies"
            } else {
                "fixes lie"
            },
            if outside == 1 { "is" } else { "are" }
        );
        let _ = writeln!(
            md,
            "| Band (m above the pad) | Flights | Fixes | Mean speed difference (m/s) | RMS speed \
             difference (m/s) | RMS vector difference (m/s) |"
        );
        let _ = writeln!(md, "|---|---|---|---|---|---|");
        for row in self.bands.iter().chain(std::iter::once(&self.all)) {
            if row.samples == 0 {
                let _ = writeln!(md, "| {} | 0 | 0 | | | |", row.label);
            } else {
                let _ = writeln!(
                    md,
                    "| {} | {} | {} | {:+.2} | {:.2} | {:.2} |",
                    row.label,
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
                "Each receiver's track is reconstructed alone. The difference of their winds is a \
                 floor on the noise between two receivers, not the method's error: an error of \
                 the lag model, or a motion both receivers share, cancels in it.\n"
            );
            for pair in &self.receivers {
                let _ = writeln!(
                    md,
                    "- `{}`, with {}: the two highest fixes are {:.0} m and {:.0} m above sea \
                     level and the last fixes {:.1} m apart. {} fixes are read by both within \
                     {PAIR_S} s; their winds differ by {:.2} m/s RMS, {:.2} m/s at most.",
                    pair.id,
                    pair.second,
                    pair.highest_m[0],
                    pair.highest_m[1],
                    pair.last_fixes_apart_m,
                    pair.pairs,
                    pair.rms_vector_difference_m_s,
                    up_to_hundredths(pair.max_vector_difference_m_s)
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
             - **ERA5's hour and place.** ERA5 is read at the launch site and hour; the descent \
             is minutes later and away from the site.\n\
             - **Drogues and tumbling bodies.** The lag `w/g` assumes the same drag in every \
             direction; a tumbling airframe or a streamer has no measured response.\n\
             - **Each fix of the EuRoC logs.** CATS publishes EuRoC logs under GPL-3.0, and three \
             of RocketPy's copies are files of RocketPy-Team's GPL-3.0 fork of its 2023 \
             repository, so the logs of its flight computers give statistics here, not rows.\n\
             - **Two public team logs.** Hyperion-II's and UTRGV's, in the project's \
             public-flights note, hold GPS fixes in the descent and are not read yet (issue \
             #1605), so the report is short of every public flight with a GPS track of the \
             descent."
        );
        let _ = writeln!(md, "\n## Sources\n\n{ERA5_CREDIT}");
        md
    }

    /// Whether `markdown` is this report's page.
    ///
    /// # Errors
    ///
    /// When it isn't.
    pub fn check_page(&self, markdown: &str) -> Result<(), String> {
        if markdown.replace("\r\n", "\n") == self.to_markdown() {
            Ok(())
        } else {
            Err(format!(
                "{REPORT_MD} is not the page of {REPORT_JSON}: run `cargo xtask descent-wind`"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::up_to_hundredths;

    #[test]
    fn a_bound_rounds_up_to_hundredths_and_keeps_an_exact_one() {
        assert_eq!(up_to_hundredths(1.344), 1.35);
        assert_eq!(up_to_hundredths(1.3401), 1.35);
        assert_eq!(up_to_hundredths(0.07), 0.07);
        assert_eq!(up_to_hundredths(1.35), 1.35);
    }
}
