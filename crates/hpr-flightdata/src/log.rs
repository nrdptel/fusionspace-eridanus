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
    /// A barometric altitude above the logger's own zero, m; in a Featherweight GPS tracker's
    /// summary ([`LogFormat::FeatherweightSummaryCsv`]), its receiver's altitude above the pad it
    /// states.
    AltitudeAboveGround,
    /// A barometric altitude above mean sea level, m.
    AltitudeMsl,
    /// An altitude from GNSS, above mean sea level as the receiver states it, m.
    GnssAltitude,
    /// An altitude the logger integrated from its accelerometer, above its zero, m.
    InertialAltitude,
    /// An altitude whose sensor or zero the file doesn't state, m: a table read by its header
    /// ([`LogFormat::GenericTable`]) names an altitude and its unit, and often no more.
    Altitude,
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
    /// A length of time, s, such as a summary's time from liftoff to an event.
    Duration,
    /// A number with no unit, such as a count of satellites, a signal strength, a flag or a
    /// state code, as the file writes it, or a fraction, such as a percentage the file writes,
    /// divided by 100.
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
            | Self::Altitude
            | Self::Distance => "m",
            Self::Pressure => "Pa",
            Self::Speed => "m/s",
            Self::Acceleration => "m/s²",
            Self::Temperature => "K",
            Self::Voltage => "V",
            Self::Current => "A",
            Self::Latitude | Self::Longitude | Self::Angle => "°",
            Self::AngularRate => "°/s",
            Self::TimeOfDay | Self::Duration => "s",
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
    /// AerospaceNU's SillyGoose flight computer's log, as the board writes it: tab-separated
    /// text under a header such as `timestampMs pressurePa tempK …`.
    SillyGooseText,
    /// Silicdyne's Fluctus flight computer's CSV: a `sep=;` line, then a semicolon-separated
    /// table whose names carry their units, such as `time (ms);baro-altitude (m);…`.
    FluctusCsv,
    /// Ozark Aerospace's ARTS2 interpreted data: a `# MET Acc Vel iAlt pAlt` header, then
    /// comma-separated rows from launch.
    Arts2Csv,
    /// AerospaceNU's flight computer board's CSV, as the club's tools write a flight: a header
    /// row opening `,packetType,timestamp_ms,…` or `,timestamp_s,…`, then one row a sample.
    AerospaceNuFcbCsv,
    /// AerospaceNU's ground station's text, as it parses the flight computer board's downlink: a
    /// `RUN START` line, then one packet a line, `HH:MM:SS.ffffff: <packet> {…}`.
    AerospaceNuGroundStationText,
    /// A phone app's `launch….csv`: a header row opening `Seconds,Timestamp,AltitudeMSLGPS,…`,
    /// then one row a sample. The app isn't named in its files.
    PhoneLaunchCsv,
    /// An Eggtimer TRS's terminal download, as [`LogFormat::EggtimerText`] reads it, saved inside
    /// an RTF document.
    EggtimerRtf,
    /// The Featherweight Raven's raw download, `.FIPa`, as the Featherweight Interface Program
    /// saves the altimeter's memory: a text header, then checksummed binary frames.
    RavenFipa,
    /// Missile Works' RRC3 raw flight file, `.rff`, as mDACS saves a download: a .NET
    /// `BinaryFormatter` stream holding the altimeter's 16-bit words.
    Rrc3Rff,
    /// A Featherweight summary, as the Featherweight UI app exports a Blue Raven's or a GPS
    /// tracker's flight beside its data: one `name,value` row a figure, such as `Max
    /// Altitude,2500.25 feet`, and no samples.
    FeatherweightSummaryCsv,
    /// AED Electronics' R-DAS raw data, as its Windows software exports a flight to text: a
    /// `*** Configuration ***` section, then a table of the converters' counts.
    RdasText,
    /// AltOS radio telemetry, as AltosUI saves the packets a TeleDongle hears to a `.telem`
    /// file: one `TELEM` line a packet, its bytes in hexadecimal.
    AltosTelemetry,
    /// AltOS's raw flight download, as AltosUI saves a flight computer's memory to an `.eeprom`
    /// file: a JSON object of the board's configuration then the memory in hexadecimal, or, from
    /// older versions, the configuration as the board prints it then one record a line.
    AltosEeprom,
    /// Ozark Aerospace ARTS2's flight as its PC software saves it to an `.odf` file: the board's
    /// settings and calibration, then the sensors' counts.
    Arts2Odf,
    /// Ozark Aerospace ARTS2's raw data, as its PC software exports a flight: a
    /// `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header, then the sensors' counts.
    Arts2RawCsv,
    /// Missile Works' RTx/GPS tracker's fixes, as mDACS saves a download to an `.rtx` file: a
    /// .NET `BinaryFormatter` stream of one `List<Int32>`, each slot a 16-bit word mapped modulo
    /// 10⁹. Positions only.
    RtxRaw,
    /// The RTx's fixes as mDACS exports them to CSV: a `UTC Time,Latitude,Longitude,Altitude,…`
    /// header, then one row a fix. Positions only.
    RtxCsv,
    /// The RTx's fixes as mDACS exports them to text: a tab-separated `UTC Lat Lon Alt …` header,
    /// then one row a fix. Positions only.
    RtxText,
    /// A table of samples from a logger no other reader knows, read by what its header states: a
    /// header row naming the columns, the units in the names, such as `Time (s),Alt AGL (ft)`,
    /// then one row a sample ([`crate::generic`]); or read by a column mapping the caller gives.
    GenericTable,
    /// A file of binary records end to end, each a run of fields, read by a layout the caller
    /// states, since the file states none ([`crate::records`]).
    BinaryRecords,
    /// A serial console's capture of a flight computer's shell, its memory dumped as base64
    /// between marker lines such as `----++++//[[( data start )]]\\++++----`, each block's
    /// records read by a layout the caller states ([`crate::capture`]).
    ConsoleCapture,
    /// The Featherweight Blue Raven's flight data as its USB port downloads it to a serial
    /// terminal: one `@ LOG_LOW` or `@ LOG_HIR` line a sample, each closed by a CRC
    /// ([`crate::blueraven_usb`]).
    BlueRavenUsb,
    /// A course's Teensy data logger, tab-separated: a sensor's tag, the clock in microseconds,
    /// then its values, such as `Alt	49743	P	92082	MSL	799.61` ([`crate::teensy`]).
    TeensyTabs,
    /// A course's Teensy data logger with named fields: units lines such as `MS5 data in us, deg
    /// C, Pa`, then lines such as `MS5, t, 1330004120, T, 32.90, P, 92185` ([`crate::teensy`]).
    TeensyNamed,
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

impl LogFormat {
    /// The altitude's resolution in the format, m: the smallest step between two altitudes the file
    /// can tell apart near the ground. A PerfectFlite and an Eggtimer write whole feet, an
    /// Alt15K/WD's `.pfa` too, though that sensor's own resolution is 4 ft at sea level (*Alt15K/WD
    /// User’s Manual*, p. 10): the step the file writes is taken, not the coarser one, so that a
    /// tolerance built on it stays tight; AltOS and Entacore AIM write their pressure in whole
    /// pascals, 0.083 m of altitude at sea level; AltimeterCloud writes its altitude to the
    /// millimeter, and it steps by that, a SillyGoose to the centimeter, a Fluctus to the whole
    /// meter, and an ARTS2's barometric altitude moves by its barometer's count, 8.8 m at most
    /// near the ground among the sample files; AerospaceNU's flight computer board reads whole
    /// pascals, 0.083 m near sea level, in its CSV and in its ground station's text, where its
    /// altitude is a filtered estimate with no step of its own; a phone app's `launch` CSV writes
    /// its pressure as a 32-bit float, in steps of 2⁻¹⁴ hPa between 512 and 1,024 hPa,
    /// 0.000 51 m at sea level: the barometer's step, as its barometric altitude, the track, is
    /// written in finer steps of its own, as small as 0.25 mm; the Raven's pressure moves in steps of 4 × 10⁻⁵ atm, 0.337
    /// m at sea level, the Blue Raven's in steps of 2 × 10⁻⁵ atm, 0.169 m, and an RRC3's export in
    /// steps of 0.1 hPa, 0.832 m. Aloft, those from a pressure are larger. A GPS log has no
    /// barometric altitude; its formats give the coarsest step its GNSS altitude is written in
    /// among the sample files: whole feet in Featherweight's older tracker log, whole meters in
    /// Missile Works' and Big Red Bee's KML, and a tenth of a meter in NMEA's `GGA`. A receiver's
    /// own error is far larger. A Featherweight summary has no track: its maximum altitude is
    /// written to the hundredth of a foot. An R-DAS's raw text has no altitude either, only its
    /// barometer's counts: one is 0.09 kPa in its maker's manual, 7.49 m near sea level. AltOS's
    /// telemetry sends the board's height above the pad in whole meters. An AltOS `.eeprom` file
    /// holds no altitude either: its coarsest pressure is a TeleMetrum v1's, whose counts move by
    /// 4 in the sample files, 13.6 Pa, 1.13 m near sea level. Nor does an ARTS2's `.odf` or raw
    /// CSV: its barometer's count is 92.2 Pa, 7.68 m near sea level. Missile Works' RTx writes its
    /// GPS altitude in whole meters in its `.rtx` file and in whole feet in its exports. A table read by
    /// its header has no track, and no step: 0; nor do binary records read by a stated layout, or a
    /// console capture's, which are read as such a table. A Blue Raven's USB download has no
    /// track either, and its pressure is written in the CSV's steps, 1/50,000 atm: the CSV's step.
    /// Nor does a Teensy data logger's file, whose pressure is written in whole pascals: 1 Pa,
    /// 0.083 m near sea level.
    pub fn altitude_resolution_m(self) -> f64 {
        match self {
            Self::PerfectFlitePf2 | Self::PerfectFliteCsv | Self::PerfectFliteText => {
                crate::perfectflite::FOOT_M
            }
            Self::AltosCsv => ONE_PASCAL_M,
            Self::RavenFipCsv => RAVEN_PRESSURE_STEP_ATM * 101_325.0 * ONE_PASCAL_M,
            Self::EggtimerCsv => crate::perfectflite::FOOT_M,
            Self::Rrc3Csv => RRC3_PRESSURE_STEP_PA * ONE_PASCAL_M,
            Self::TeensyTabs | Self::TeensyNamed => ONE_PASCAL_M,
            Self::BlueRavenCsv | Self::BlueRavenUsb => {
                BLUE_RAVEN_PRESSURE_STEP_ATM * 101_325.0 * ONE_PASCAL_M
            }
            Self::FeatherweightGpsCsv => crate::perfectflite::FOOT_M,
            Self::Kml => 1.0,
            Self::Nmea0183 => 0.1,
            Self::PerfectFlitePfa | Self::EggtimerText => crate::perfectflite::FOOT_M,
            Self::AltimeterCloudCsv => 0.001,
            Self::EntacoreAimCsv => ONE_PASCAL_M,
            Self::SillyGooseText => crate::sillygoose::ALTITUDE_STEP_M,
            Self::FluctusCsv => crate::fluctus::ALTITUDE_STEP_M,
            Self::Arts2Csv => crate::arts2::ALTITUDE_STEP_M,
            Self::AerospaceNuFcbCsv => crate::aerospacenu_fcb::ALTITUDE_STEP_M,
            Self::AerospaceNuGroundStationText => crate::aerospacenu_gs::ALTITUDE_STEP_M,
            Self::PhoneLaunchCsv => crate::phone_launch::ALTITUDE_STEP_M,
            Self::EggtimerRtf => crate::perfectflite::FOOT_M,
            Self::RavenFipa => RAVEN_PRESSURE_STEP_ATM * 101_325.0 * ONE_PASCAL_M,
            Self::Rrc3Rff => RRC3_PRESSURE_STEP_PA * ONE_PASCAL_M,
            Self::FeatherweightSummaryCsv => 0.01 * crate::perfectflite::FOOT_M,
            Self::RdasText => crate::rdas::ALTITUDE_STEP_M,
            Self::AltosTelemetry => 1.0,
            Self::AltosEeprom => crate::altos_eeprom::ALTITUDE_STEP_M,
            Self::Arts2Odf | Self::Arts2RawCsv => crate::arts2_raw::ALTITUDE_STEP_M,
            Self::RtxRaw => crate::rtx::ALTITUDE_STEP_M,
            Self::RtxCsv | Self::RtxText => crate::rtx::EXPORT_ALTITUDE_STEP_M,
            Self::GenericTable | Self::BinaryRecords | Self::ConsoleCapture => 0.0,
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
            | Self::EntacoreAimCsv
            | Self::FluctusCsv
            | Self::Arts2Csv
            | Self::AerospaceNuFcbCsv
            | Self::PhoneLaunchCsv
            | Self::FeatherweightSummaryCsv
            | Self::Arts2RawCsv
            | Self::RtxCsv
            | Self::GenericTable => ".csv",
            Self::PerfectFliteText
            | Self::Nmea0183
            | Self::EggtimerText
            | Self::SillyGooseText
            | Self::AerospaceNuGroundStationText
            | Self::RdasText
            | Self::RtxText
            | Self::BlueRavenUsb
            | Self::TeensyTabs
            | Self::TeensyNamed => ".txt",
            Self::EggtimerRtf => ".rtf",
            Self::RavenFipa => ".FIPa",
            Self::Rrc3Rff => ".rff",
            Self::PerfectFlitePfa => ".pfa",
            Self::Kml => ".kml",
            Self::AltosTelemetry => ".telem",
            Self::AltosEeprom => ".eeprom",
            Self::Arts2Odf => ".odf",
            Self::RtxRaw => ".rtx",
            Self::BinaryRecords => ".bin",
            Self::ConsoleCapture => ".cap",
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
            Self::SillyGooseText => "SillyGoose text",
            Self::FluctusCsv => "Silicdyne Fluctus CSV",
            Self::Arts2Csv => "ARTS2 CSV",
            Self::AerospaceNuFcbCsv => "AerospaceNU flight computer CSV",
            Self::AerospaceNuGroundStationText => "AerospaceNU ground station text",
            Self::PhoneLaunchCsv => "phone app launch CSV",
            Self::EggtimerRtf => "Eggtimer RTF",
            Self::RavenFipa => "Raven .FIPa",
            Self::Rrc3Rff => "RRC3 .rff",
            Self::FeatherweightSummaryCsv => "Featherweight summary CSV",
            Self::RdasText => "R-DAS text",
            Self::AltosTelemetry => "AltOS telemetry",
            Self::AltosEeprom => "AltOS .eeprom",
            Self::Arts2Odf => "ARTS2 .odf",
            Self::Arts2RawCsv => "ARTS2 raw CSV",
            Self::RtxRaw => "RTx .rtx",
            Self::RtxCsv => "RTx CSV",
            Self::RtxText => "RTx text",
            Self::GenericTable => "a table read by its header",
            Self::BinaryRecords => "binary records by a stated layout",
            Self::ConsoleCapture => "console capture of base64 records",
            Self::BlueRavenUsb => "Blue Raven USB download",
            Self::TeensyTabs => "Teensy data logger, tab-separated",
            Self::TeensyNamed => "Teensy data logger, named fields",
        }
    }
}

/// What a flight log states about the flight, in SI.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Stated {
    /// The apogee the logger computed, m above its own zero; `None` if the file states none, or
    /// states something that isn't a height, such as a PerfectFlite's `PWRLOSS`.
    pub apogee_m: Option<f64>,
    /// The launch site's elevation, m above mean sea level, as the logger states it.
    pub ground_elevation_msl_m: Option<f64>,
    /// How many samples the file says it holds.
    pub samples: Option<usize>,
    /// Every figure a summary file states, in the file's order, such as a Featherweight
    /// summary's `Max velocity,500.50 feet/sec`; empty for a file of samples, and then left out of
    /// the record's JSON, so that a record written before the field was reads the same.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub figures: Vec<Figure>,
}

/// A figure a file states once about its flight, such as a summary's `Max Altitude,2500.25
/// feet`: kept as the file writes it, and in SI where its unit is known.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Figure {
    /// The figure's name as the file writes it, such as `Max Altitude`.
    pub name: String,
    /// The value as the file writes it, unit and all, its cells joined by commas, such as
    /// `2500.25 feet` or `-1.0000,0.0000,0.0000`.
    pub written: String,
    /// What the figure measures, which sets the unit of [`Figure::values`]; `None` for text, such
    /// as a date, a time of day or a name, and [`Quantity::Unrecognized`] for numbers kept as
    /// written, in a unit HPR Sim doesn't know or that the figure's name contradicts.
    pub quantity: Option<Quantity>,
    /// The value in the quantity's unit ([`Quantity::unit`]), one number for each the file
    /// writes: three for a vector, such as a direction's components along the logger's axes;
    /// empty for text. Every one a reader writes is finite: a `NaN` here would be written to JSON
    /// as `null`, which doesn't read back.
    pub values: Vec<f64>,
}

impl Figure {
    /// A figure, its value in the quantity's unit.
    pub fn new(
        name: impl Into<String>,
        written: impl Into<String>,
        quantity: Option<Quantity>,
        values: Vec<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            written: written.into(),
            quantity,
            values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each format's resolution against a hand computation: 1 / (1.225 × 9.806 65) =
    /// 0.083 242 m for a pascal, 4 × 10⁻⁵ × 101 325 = 4.053 Pa for the Raven's step,
    /// 2 × 10⁻⁵ × 101 325 = 2.0265 Pa for the Blue Raven's, 10 Pa for an RRC3's, and
    /// 1,000 / (1,024 × 0.010 59) = 92.2165 Pa for an ARTS2's count.
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
            (LogFormat::PerfectFlitePfa, 0.3048),
            (LogFormat::EggtimerText, 0.3048),
            (LogFormat::EntacoreAimCsv, 0.083_242),
            (LogFormat::AltimeterCloudCsv, 0.001),
            (LogFormat::SillyGooseText, 0.01),
            (LogFormat::FluctusCsv, 1.0),
            (LogFormat::Arts2Csv, 8.805_672),
            (LogFormat::RdasText, 7.491_793),
            (LogFormat::AltosEeprom, 1.129_595),
            (LogFormat::Arts2Odf, 7.676_218),
            (LogFormat::Arts2RawCsv, 7.676_218),
            (LogFormat::RtxRaw, 1.0),
            (LogFormat::RtxCsv, 0.3048),
            (LogFormat::RtxText, 0.3048),
            (LogFormat::GenericTable, 0.0),
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
