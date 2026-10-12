//! The flight record every log reader produces: a clock, the channels the logger recorded on it,
//! and what the file states about itself.
//!
//! Units are SI, converted from the file's own at reading: meters, meters per second, kelvin,
//! volts. A gap in a channel, such as a row that left a column out, is `NaN`.

use serde::{Deserialize, Serialize};

/// A flight log, read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightLog {
    /// The file format it was read from.
    pub format: LogFormat,
    /// The logger, as the file names it, such as `PerfectFlite Pnut`.
    pub logger: String,
    /// The logger's serial number, as the file states it.
    pub serial_number: Option<String>,
    /// The logger's firmware version, as the file states it.
    pub firmware: Option<String>,
    /// The flight's number in the logger's memory, as the file states it.
    pub flight_number: Option<u32>,
    /// What the file states about the flight: the logger's own figures, kept beside HPR Sim's
    /// readings and never in place of them.
    pub stated: Stated,
    /// Each sample's time, s, from the logger's own zero. Strictly increasing.
    pub time_s: Vec<f64>,
    /// The barometric altitude, m, as the logger converts its pressure, above the logger's own
    /// zero: for a PerfectFlite, its reading on the pad.
    pub altitude_m: Vec<f64>,
    /// The vertical speed, m/s, up positive, as the logger computes it; `None` if the file has
    /// no such column. A barometric logger works it out from its own altitude.
    pub vertical_speed_m_s: Option<Vec<f64>>,
    /// The logger's temperature sensor, K; `None` if the file has no such column. It sits in the
    /// electronics bay, so it reads the bay, not the air.
    pub temperature_k: Option<Vec<f64>>,
    /// The logger's battery voltage, V; `None` if the file has no such column.
    pub battery_v: Option<Vec<f64>>,
    /// Every data column the file holds, in the file's order, each on its own clock and in SI:
    /// the primary track's columns above included, so that the record keeps every sample the
    /// logger wrote. A clock column is the time of its channels, not a channel itself.
    pub channels: Vec<Channel>,
    /// What the logger says happened and when, such as an AltOS flight computer's change of
    /// state, in the file's order.
    pub events: Vec<Event>,
    /// What the reader noticed and worked around, such as a sample count that differs from the
    /// one the file states.
    pub notes: Vec<String>,
}

impl FlightLog {
    /// The channels measuring a quantity, in the file's order.
    pub fn channels_of(&self, quantity: Quantity) -> impl Iterator<Item = &Channel> {
        self.channels
            .iter()
            .filter(move |channel| channel.quantity == quantity)
    }
}

/// One column of a flight log, on its own clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// The column's name as the file writes it, such as `Axial Accel (Gs)`: it says which axis
    /// or sensor a quantity came from where the file says so.
    pub column: String,
    /// What the column measures, which sets its unit.
    pub quantity: Quantity,
    /// Each sample's time, s, from the log's zero, the same zero as [`FlightLog::time_s`].
    pub time_s: Vec<f64>,
    /// Each sample's value in the quantity's unit ([`Quantity::unit`]); `NaN` where the file left
    /// the cell empty or wrote something that isn't a number.
    pub values: Vec<f64>,
}

/// What a channel measures. The unit is SI but for angles, which are in degrees as GNSS
/// receivers and loggers write them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Quantity {
    /// A barometric altitude above the logger's own zero, m.
    AltitudeAboveGround,
    /// A barometric altitude above mean sea level, m.
    AltitudeMsl,
    /// An altitude from GNSS, above mean sea level as the receiver states it, m.
    GnssAltitude,
    /// An altitude the logger integrated from its accelerometer, above its zero, m.
    InertialAltitude,
    /// Air pressure, Pa.
    Pressure,
    /// A speed or velocity the logger computed, m/s; its column says from which sensor and along
    /// which direction.
    Speed,
    /// An acceleration, m/s²; its column says along which axis, and whether gravity is in it.
    Acceleration,
    /// A temperature, K.
    Temperature,
    /// An electric potential, V, such as a battery's or a pyro channel's.
    Voltage,
    /// An electric current, A.
    Current,
    /// A latitude, degrees north.
    Latitude,
    /// A longitude, degrees east.
    Longitude,
    /// An angle, degrees, such as a tilt or a bearing.
    Angle,
    /// A distance, m, such as a GNSS receiver's range from the pad.
    Distance,
    /// A number with no unit, such as a count of satellites, a signal strength, a flag or a
    /// state code, as the file writes it.
    Number,
    /// A column HPR Sim doesn't know: its values as the file writes them, in a unit HPR Sim
    /// doesn't know either.
    Unrecognized,
}

impl Quantity {
    /// The unit of a channel's values.
    pub fn unit(self) -> &'static str {
        match self {
            Self::AltitudeAboveGround
            | Self::AltitudeMsl
            | Self::GnssAltitude
            | Self::InertialAltitude
            | Self::Distance => "m",
            Self::Pressure => "Pa",
            Self::Speed => "m/s",
            Self::Acceleration => "m/s²",
            Self::Temperature => "K",
            Self::Voltage => "V",
            Self::Current => "A",
            Self::Latitude | Self::Longitude | Self::Angle => "°",
            Self::Number | Self::Unrecognized => "",
        }
    }
}

/// Something the logger says happened, at a time on the log's clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// When, s, from the log's zero.
    pub time_s: f64,
    /// What, in the logger's own word, such as `boost` or `drogue`.
    pub name: String,
}

/// A flight log's file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum LogFormat {
    /// PerfectFlite's `.pf2`, as its software writes a Pnut's, a StratoLogger's or a
    /// StratoLoggerCF's flights.
    PerfectFlitePf2,
    /// PerfectFlite's comma-separated export: a header row such as `Time, Altitude, Velocity,
    /// Temperature (F), Voltage`, then rows in seconds and feet.
    PerfectFliteCsv,
    /// PerfectFlite's tab-separated text export: a header row such as `Time Altitude Temp.
    /// Voltage`, then rows in seconds and feet.
    PerfectFliteText,
    /// AltOS's CSV, as AltosUI exports a TeleMetrum's, TeleMega's, EasyMega's or EasyMini's
    /// flight: a `#version,serial,flight,…` header, then rows in SI.
    AltosCsv,
    /// The Featherweight Raven's FIP CSV, as the Featherweight Interface Program exports it: each
    /// channel its own `Time@` column, its values, and a status column.
    RavenFipCsv,
}

/// The altitude step of one pascal at sea level, m: ISA's density and standard gravity,
/// `1 / (1.225 × 9.80665)`. A logger that writes its pressure in whole pascals can't tell two
/// altitudes nearer than this apart near the ground, however many digits its altitude has.
const ONE_PASCAL_M: f64 = 1.0 / (1.225 * 9.806_65);

impl LogFormat {
    /// The altitude's resolution in the format, m: the smallest step between two altitudes the
    /// file can tell apart near the ground. A PerfectFlite writes whole feet; AltOS writes its
    /// pressure in whole pascals and the Raven in steps of 10⁻⁵ atm (1.013 25 Pa), each step
    /// 0.083 m or so of altitude at sea level, more aloft.
    pub fn altitude_resolution_m(self) -> f64 {
        match self {
            Self::PerfectFlitePf2 | Self::PerfectFliteCsv | Self::PerfectFliteText => {
                crate::perfectflite::FOOT_M
            }
            Self::AltosCsv => ONE_PASCAL_M,
            Self::RavenFipCsv => 1.013_25 * ONE_PASCAL_M,
        }
    }

    /// The format's usual file extension, with its dot.
    pub fn extension(self) -> &'static str {
        match self {
            Self::PerfectFlitePf2 => ".pf2",
            Self::PerfectFliteCsv | Self::AltosCsv | Self::RavenFipCsv => ".csv",
            Self::PerfectFliteText => ".txt",
        }
    }

    /// The format's name for people, such as `AltOS CSV`.
    pub fn name(self) -> &'static str {
        match self {
            Self::PerfectFlitePf2 => "PerfectFlite .pf2",
            Self::PerfectFliteCsv => "PerfectFlite CSV",
            Self::PerfectFliteText => "PerfectFlite text",
            Self::AltosCsv => "AltOS CSV",
            Self::RavenFipCsv => "Raven FIP CSV",
        }
    }
}

/// What a flight log states about the flight, in SI.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Stated {
    /// The apogee the logger computed, m above its own zero; `None` if the file states none, or
    /// states something that isn't a height, such as a PerfectFlite's `PWRLOSS`.
    pub apogee_m: Option<f64>,
    /// The launch site's elevation, m above mean sea level, as the logger states it.
    pub ground_elevation_msl_m: Option<f64>,
    /// How many samples the file says it holds.
    pub samples: Option<usize>,
}
