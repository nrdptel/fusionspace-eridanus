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
    #[serde(with = "gaps")]
    pub altitude_m: Vec<f64>,
    /// The vertical speed, m/s, up positive, as the logger computes it; `None` if the file has
    /// no such column. A barometric logger works it out from its own altitude.
    #[serde(with = "optional_gaps", default)]
    pub vertical_speed_m_s: Option<Vec<f64>>,
    /// The logger's temperature sensor, K; `None` if the file has no such column. It sits in the
    /// electronics bay, so it reads the bay, not the air.
    #[serde(with = "optional_gaps", default)]
    pub temperature_k: Option<Vec<f64>>,
    /// The logger's battery voltage, V; `None` if the file has no such column.
    #[serde(with = "optional_gaps", default)]
    pub battery_v: Option<Vec<f64>>,
    /// Every data column the file holds, in the file's order, each on its own clock and in SI:
    /// the primary track's columns above included, so that the record keeps every sample the
    /// logger wrote. A clock column is the time of its channels, not a channel itself. Empty in
    /// a record written before the field was.
    #[serde(default)]
    pub channels: Vec<Channel>,
    /// What the logger says happened and when, such as an AltOS flight computer's change of
    /// state: in the file's order, or in time order where the file states them apart from its
    /// rows, as an Eggtimer's summary does.
    #[serde(default)]
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
#[non_exhaustive]
pub struct Channel {
    /// The column's name as the file writes it, such as `Axial Accel (Gs)`: it says which axis
    /// or sensor a quantity came from where the file says so.
    pub column: String,
    /// What the column measures, which sets its unit.
    pub quantity: Quantity,
    /// Each sample's time, s, from the log's zero, the same zero as [`FlightLog::time_s`]; `NaN`
    /// where the file gives the sample no time, as a KML path without times does (the GPS logs'
    /// decision, [ADR-254][adr-254]). In the file's order, so a time may go back where the
    /// logger's clock did.
    ///
    /// [adr-254]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md
    #[serde(with = "gaps")]
    pub time_s: Vec<f64>,
    /// Each sample's value in the quantity's unit ([`Quantity::unit`]); `NaN` where the file left
    /// the cell empty or wrote something that isn't a number.
    #[serde(with = "gaps")]
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
    /// A rate of turn, degrees per second, such as a gyroscope's about one of its axes.
    AngularRate,
    /// A distance, m, such as a GNSS receiver's range from the pad.
    Distance,
    /// A time of day on the logger's own clock, s after its midnight, in whatever time zone the
    /// clock was set to: files state none.
    TimeOfDay,
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
            Self::AngularRate => "°/s",
            Self::TimeOfDay => "s",
            Self::Number | Self::Unrecognized => "",
        }
    }
}

/// Something the logger says happened, at a time on the log's clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
    /// Missile Works' mDACS export of an RRC3's flight: a header row `Time, Altitude, Pressure,
    /// Velocity, Temperature, Events, Voltages`, then rows on one clock.
    Rrc3Csv,
    /// An Eggtimer's flight data, as its own download writes it: a header row such as `T, Alt,
    /// VRaw, VFilt`, then rows on one clock.
    EggtimerCsv,
    /// The Featherweight Blue Raven's CSV, as its phone app exports a flight: a low-rate file of
    /// its barometric and inertial figures, or a high-rate file of its inertial sensors, each
    /// with a header row such as `Year,Month,Day,Time,Flight_Time_(s),…`.
    BlueRavenCsv,
    /// A Featherweight GPS tracker's CSV, as its ground station's app or software writes it: the
    /// tracker's log, with a header row such as `UTCTIME,UNIXTIME,ALT,LAT,LON,#SATS,…`, or the
    /// ground station's, such as `TRACKER,DATE,TIME,LAT,LON,ALT,…`. Positions only, no barometer.
    FeatherweightGpsCsv,
    /// A KML file (OGC 07-147r2) holding a GPS track as a path or a `gx:Track`, as AltOS, Missile
    /// Works' RTx, Big Red Bee's BeeLine, fw2kml and others write it. Positions only.
    Kml,
    /// A GPS receiver's NMEA 0183 sentences as it wrote them, such as `$GPGGA,…*hh`. Positions
    /// only.
    Nmea0183,
    /// PerfectFlite's older `.pfa`, as its software saves an Alt15K/WD's flight: `Ground:`,
    /// `Apogee:` and `NumSamps:` lines, then `**!!Data:` and one altitude a line.
    PerfectFlitePfa,
    /// An Eggtimer TRS's flight as its terminal download prints it: the flight settings and the
    /// flight summary, then the table the CSV download writes.
    EggtimerText,
    /// AltimeterCloud's CSV of a Rocketry Ltd Nano V1's or Mercury's flight, as a flight page on
    /// altimetercloud.com hands it back: a header row such as `Time(ms),Altitude(m),…`.
    AltimeterCloudCsv,
    /// Entacore AIM's CSV, as its software exports a flight: each channel beside its own `time`
    /// column.
    EntacoreAimCsv,
}

/// Serde for a list of values whose gaps are `NaN`: each gap written as `null`, and a `null` read
/// back as `NaN`, so that a record survives its own JSON.
mod gaps {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// A list written with its gaps as `null`.
    pub struct Written<'a>(pub &'a [f64]);

    impl Serialize for Written<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(
                self.0
                    .iter()
                    .map(|value| (!value.is_nan()).then_some(*value)),
            )
        }
    }

    pub fn serialize<S: Serializer>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error> {
        Written(values).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<f64>, D::Error> {
        let values: Vec<Option<f64>> = Vec::deserialize(deserializer)?;
        Ok(values
            .into_iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect())
    }
}

/// [`gaps`] for a list the file may not have: absent or `null` is `None`.
mod optional_gaps {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        values: &Option<Vec<f64>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match values {
            Some(values) => serializer.serialize_some(&super::gaps::Written(values)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<f64>>, D::Error> {
        let values: Option<Vec<Option<f64>>> = Option::deserialize(deserializer)?;
        Ok(values.map(|values| {
            values
                .into_iter()
                .map(|value| value.unwrap_or(f64::NAN))
                .collect()
        }))
    }
}

/// Whether a file's rows hold too few of the cells its header names: under a quarter of them,
/// past the first 65,536. A reader stores a cell a short row leaves out as a gap, so without this
/// floor a small file with a wide header and short rows could make it allocate far more than the
/// file holds: the readers refuse such a file instead.
pub(crate) fn too_sparse(header_columns: usize, rows: usize, cells_written: usize) -> bool {
    header_columns.saturating_mul(rows) > cells_written.saturating_mul(4).saturating_add(65_536)
}

/// The altitude step of one pascal at sea level, m: `1 / (ρ₀ g₀)`, with the International Standard
/// Atmosphere's sea-level density ρ₀ = 1.225 kg/m³ (ISO 2533:1975) and standard gravity
/// g₀ = 9.806 65 m/s² (CGPM 1901). A logger whose pressure moves in steps of so many pascals
/// can't tell two altitudes nearer than that many times this apart near the ground, however many
/// digits its altitude has; aloft, where the air is thinner, the step is larger.
const ONE_PASCAL_M: f64 = 1.0 / (1.225 * 9.806_65);

/// The smallest step the Raven's pressure moves by, atm: 4 × 10⁻⁵ atm (4.053 Pa), the smallest
/// step of `Baro (Atm)` in 74 of the 78 distinct Raven files among the project's sample logs
/// (the others step by 8 to 16 Pa), though the export prints five decimals.
const RAVEN_PRESSURE_STEP_ATM: f64 = 4.0e-5;

/// The step an RRC3's export writes its pressure in, Pa: 0.1 hPa (10 Pa), the smallest step of
/// `Pressure` in all 50 RRC3 files among the project's sample logs.
const RRC3_PRESSURE_STEP_PA: f64 = 10.0;

/// The Blue Raven's pressure step, atm: 2 × 10⁻⁵ atm (2.0265 Pa), as its USB download writes the
/// pressure in atmospheres times 50,000 (*Blue Raven User's Guide*, 2026-05-12, p23). The app's
/// file prints the pressure to 10⁻⁴ atm, but its altitude above the pad steps by 0.5 to 0.6 ft near
/// the pad, as this finer step makes it.
const BLUE_RAVEN_PRESSURE_STEP_ATM: f64 = 2.0e-5;

/// The Alt15K/WD's altitude resolution at sea level, ft: "4’@SL, 6’@15KMSL" (*Alt15K/WD User’s
/// Manual*, PerfectFlite, printed p. 10). Its `.pfa` writes whole feet, finer than the sensor
/// tells apart.
const PFA_RESOLUTION_FT: f64 = 4.0;

impl LogFormat {
    /// The altitude's resolution in the format, m: the smallest step between two altitudes the
    /// file can tell apart near the ground. A PerfectFlite and an Eggtimer write whole feet,
    /// though the sensor of an Alt15K/WD, whose `.pfa` does too, tells apart only 4 ft at sea
    /// level; AltOS and Entacore AIM write their pressure in whole pascals, 0.083 m of altitude at
    /// sea level; AltimeterCloud writes its altitude to the millimeter, and it steps by that; the
    /// Raven's
    /// pressure moves in steps of 4 × 10⁻⁵ atm, 0.337 m at sea level, the Blue Raven's in steps of
    /// 2 × 10⁻⁵ atm, 0.169 m, and an RRC3's export in steps of 0.1 hPa, 0.832 m. Aloft, those
    /// from a pressure are larger. A GPS log has no barometric altitude; its formats give the
    /// coarsest step its GNSS altitude is written in among the sample files: whole feet in
    /// Featherweight's older tracker log, whole meters in Missile Works' and Big Red Bee's KML,
    /// and a tenth of a meter in NMEA's `GGA`. A receiver's own error is far larger.
    pub fn altitude_resolution_m(self) -> f64 {
        match self {
            Self::PerfectFlitePf2 | Self::PerfectFliteCsv | Self::PerfectFliteText => {
                crate::perfectflite::FOOT_M
            }
            Self::AltosCsv => ONE_PASCAL_M,
            Self::RavenFipCsv => RAVEN_PRESSURE_STEP_ATM * 101_325.0 * ONE_PASCAL_M,
            Self::EggtimerCsv => crate::perfectflite::FOOT_M,
            Self::Rrc3Csv => RRC3_PRESSURE_STEP_PA * ONE_PASCAL_M,
            Self::BlueRavenCsv => BLUE_RAVEN_PRESSURE_STEP_ATM * 101_325.0 * ONE_PASCAL_M,
            Self::FeatherweightGpsCsv => crate::perfectflite::FOOT_M,
            Self::Kml => 1.0,
            Self::Nmea0183 => 0.1,
            Self::PerfectFlitePfa => PFA_RESOLUTION_FT * crate::perfectflite::FOOT_M,
            Self::EggtimerText => crate::perfectflite::FOOT_M,
            Self::AltimeterCloudCsv => 0.001,
            Self::EntacoreAimCsv => ONE_PASCAL_M,
        }
    }

    /// The format's usual file extension, with its dot.
    pub fn extension(self) -> &'static str {
        match self {
            Self::PerfectFlitePf2 => ".pf2",
            Self::PerfectFliteCsv
            | Self::AltosCsv
            | Self::RavenFipCsv
            | Self::Rrc3Csv
            | Self::EggtimerCsv
            | Self::BlueRavenCsv
            | Self::FeatherweightGpsCsv
            | Self::AltimeterCloudCsv
            | Self::EntacoreAimCsv => ".csv",
            Self::PerfectFliteText | Self::Nmea0183 | Self::EggtimerText => ".txt",
            Self::PerfectFlitePfa => ".pfa",
            Self::Kml => ".kml",
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
            Self::Rrc3Csv => "RRC3 CSV",
            Self::EggtimerCsv => "Eggtimer CSV",
            Self::BlueRavenCsv => "Blue Raven CSV",
            Self::FeatherweightGpsCsv => "Featherweight GPS CSV",
            Self::Kml => "KML",
            Self::Nmea0183 => "NMEA 0183",
            Self::PerfectFlitePfa => "PerfectFlite .pfa",
            Self::EggtimerText => "Eggtimer text",
            Self::AltimeterCloudCsv => "AltimeterCloud CSV",
            Self::EntacoreAimCsv => "Entacore AIM CSV",
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Each format's resolution against a hand computation: 1 / (1.225 × 9.806 65) =
    /// 0.083 242 m for a pascal, 4 × 10⁻⁵ × 101 325 = 4.053 Pa for the Raven's step,
    /// 2 × 10⁻⁵ × 101 325 = 2.0265 Pa for the Blue Raven's, and 10 Pa for an RRC3's.
    #[test]
    fn each_formats_altitude_resolution_is_its_step() {
        let cases = [
            (LogFormat::PerfectFlitePf2, 0.3048),
            (LogFormat::PerfectFliteCsv, 0.3048),
            (LogFormat::PerfectFliteText, 0.3048),
            (LogFormat::AltosCsv, 0.083_242),
            (LogFormat::RavenFipCsv, 0.337_38),
            (LogFormat::Rrc3Csv, 0.832_42),
            (LogFormat::EggtimerCsv, 0.3048),
            (LogFormat::BlueRavenCsv, 0.168_69),
            (LogFormat::FeatherweightGpsCsv, 0.3048),
            (LogFormat::Kml, 1.0),
            (LogFormat::Nmea0183, 0.1),
            (LogFormat::PerfectFlitePfa, 1.2192),
            (LogFormat::EggtimerText, 0.3048),
            (LogFormat::EntacoreAimCsv, 0.083_242),
            (LogFormat::AltimeterCloudCsv, 0.001),
        ];
        for (format, expected) in cases {
            let resolution = format.altitude_resolution_m();
            assert!(
                (resolution - expected).abs() < 5e-6,
                "{format:?}: {resolution} m, expected {expected} m"
            );
        }
    }
}
