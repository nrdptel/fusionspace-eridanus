//! A phone app's `launch….csv`: one row a sample, from the phone's GPS, accelerometer, gyroscope,
//! magnetometer and, in the newer files, its barometer.
//!
//! The app isn't named in its files, and no published description of the format was found. Its
//! files are named `launch` and a number, and open on a header row whose first names are
//! `Seconds,Timestamp,AltitudeMSLGPS`:
//!
//! ```text
//! Seconds,Timestamp,AltitudeMSLGPS,Record,Static,Latitude,Longitude,GTotal,Gx,Gy,Gz,GMax,…
//! 0.000,1941868800000,100,1,12,40.123456,-100.234567,1.000000,0.000000,0.932039,0.362358,…
//! ```
//!
//! Two generations are among the project's sample logs, as of 2026-10-13 27 files: 19 older ones of
//! 35 columns (8 of them with `Reserved1` and `Reserved2` where the others have `Revolutions` and
//! `RevApogee`), whose `AltitudeRelativeBarometer` is 0 on every row, and 8 newer ones of 67 or 69
//! columns, which add a barometer (`hPa`, and a barometric altitude and speed that move), a
//! temperature, the acceleration with gravity taken out, a satellite count and an external
//! sensor's columns. The rows come about 20 times a second; the GPS's columns change about once a
//! second and repeat their last fix between.
//!
//! **Recognition.** The first line that isn't blank opens with `Seconds,Timestamp,AltitudeMSLGPS`:
//! no other layout among the project's sample logs does.
//!
//! **Rows.** Every row is as wide as the header; a row of another width, a cell that isn't a
//! number, a row without a time or a time that doesn't increase is refused, naming its line. An
//! empty cell is a gap. Blank lines are skipped; lines may end in CR, LF or both, and a byte-order
//! mark is dropped. A name the header gives twice, or leaves empty, is refused.
//!
//! **Columns.** `Seconds` is the clock: seconds from the app's start, as its name says, and the
//! log's zero is the app's. Every other column is a channel on that clock, in the file's order.
//! No source states the other columns' units, so each is read in a unit only where two
//! independent checks on the file's own values agree on it ([ADR-258][adr-258] §2); otherwise it
//! is kept as written, [`Quantity::Unrecognized`], with a note giving the evidence. A column the
//! reader doesn't know is kept as written too.
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
//!
//! | Column | Read as | Checks (below) |
//! |---|---|---|
//! | `Seconds` | the clock, s | its name |
//! | `Timestamp` | [`Quantity::TimeOfDay`], UTC: a Unix time in ms | its steps against the clock's; its date |
//! | `hPa` | [`Quantity::Pressure`], hPa × 100 Pa | its name; the pad's pressure a launch site's |
//! | `AltitudeRelativeBarometer` | [`Quantity::AltitudeAboveGround`], m: the primary track | the pressure altitude of `hPa`; gravity in the coast |
//! | `VelocityBarometer`, `VelocityBarometerRaw` | [`Quantity::Speed`], m/s: the first the primary track's | the barometric altitude's rate; gravity in the coast |
//! | `Gx`, `Gy`, `Gz`, `GTotal` | [`Quantity::Acceleration`], g × 9.806 65 m/s² | 1 g at rest; the boost against the barometric climb |
//! | `NoGravGx`, `NoGravGy`, `NoGravGz`, `NoGravGTotal` | [`Quantity::Acceleration`], g, gravity taken out | the accelerometer's unit and gravity's 1 g between them |
//! | `Yaw` | [`Quantity::Angle`], radians × 180/π | within ±π; the magnetic field's direction |
//! | `Roll` | [`Quantity::Angle`], radians × 180/π | within ±π; gravity's direction |
//! | `Pitch` | [`Quantity::Angle`], radians × 180/π | within ±π/2; gravity's direction |
//! | `GyroX`, `GyroY`, `GyroZ` | [`Quantity::AngularRate`], rad/s × 180/π | the attitude's turn; a full scale in °/s |
//! | `Heading` | [`Quantity::Angle`], degrees | within 0 to 360 and past 2π; its wrap's period, 360 |
//! | `Latitude`, `Longitude` | [`Quantity::Latitude`], [`Quantity::Longitude`], degrees, both or neither | the latitude past π/2 and within 90; the longitude past π and within 180 |
//! | `AltitudeMSLGPS`, `AltitudeMSLMaxGPS` | [`Quantity::Unrecognized`], as written | no second check (below) |
//! | `Record`, `Static`, `UsbState`, `Satellites` | [`Quantity::Number`], as written | flags, a state and a count |
//! | `GMax`, `NoGravGMax`, `AltitudeRelativeMaxBarometer`, `VelocityBarometerMax`, `VelocityBarometerMin` | [`Quantity::Unrecognized`], as written | running extremes, not tied to their columns |
//! | `Temperature` | [`Quantity::Unrecognized`], as written | °C and °F both possible |
//! | `MagnetometerX`, `…Y`, `…Z`, `HeadingX`, `…Y`, `…Z` | [`Quantity::Unrecognized`], as written | no quantity for a magnetic field |
//! | `Revolutions`, `RevApogee`, `RevolutionsClock`, `HorizontalResGPS`, `VerticalResGPS`, `Battery`, `AltitudeRelativeSerial`, `AltitudeRelativeMaxSerial`, `Usb…`, `Reserved…` | [`Quantity::Unrecognized`], as written | no source states their units |
//!
//! **The checks.** The two checks of a pair read different numbers: two columns, a column against
//! another of settled unit or direction, or one column's extent against its steps. Each item below
//! says what each check reads and which rival units it rules out on its own. A check the file
//! can't run (too few rows, no barometer, no rest) counts as failed.
//! - *The pressure.* `hPa` names its unit. The first reading, on the pad, must also be a launch
//!   site's, 500 to 1,100 hPa, the bounds [`crate::rrc3`] cites (the U.S. Standard Atmosphere's
//!   pressure about 5,570 m up, and above the highest sea-level pressure on record); otherwise the
//!   column is kept as written and everything below that needs it fails.
//! - *The barometric altitude, in meters.* (1) Above the first row it is the pressure altitude of
//!   `hPa` above the first row's, in the U.S. Standard Atmosphere, 1976 ([`Ussa76`]), within 3 m
//!   and 2% of the climb, over a climb of at least 30 m: in feet it would be 3.28 times as large.
//!   (2) In the coast, from 8 s to 3 s before the highest altitude and no earlier than a second
//!   after the steepest climb, nothing pushes the rocket, and gravity and drag slow it by at
//!   least 1 g: a parabola fitted to the altitude there must fall at 0.7 g to 0.7 × 3.28 g, so
//!   that an altitude in feet, falling at 1 g or more in feet per second squared, can't pass. A
//!   rocket with more drag than that fails closed. As of 2026-10-13, in the 8 newer sample files,
//!   (1) holds within 1.5 m in all 8 and (2) gives 1.09 g to 1.66 g in the 7 with a coast in the
//!   file; the eighth starts after its coast.
//! - *The barometric speeds, in m/s.* (1) Against the barometric altitude's rate, over half a
//!   second about each row, a least-squares gain of 0.8 to 1.25 (ft/s, mph and km/h are 3.28,
//!   2.24 and 3.6); (2) in the same coast window, the column falls at 0.7 g to 0.7 × 3.28 g. As of
//!   2026-10-13, gains of 0.89 to 0.96 and slowings of 1.04 g to 1.66 g in the 7.
//! - *The accelerations, in g.* (1) The quietest second before the steepest climb, the pad,
//!   reads 0.9 to 1.1 in the axes' magnitude, as an accelerometer at rest reads 1 g (in m/s², 9.8);
//!   (2) from that second to the steepest climb, the magnitude less 1 g, times 9.806 65 m/s² and
//!   summed over time, the speed the boost gave, comes to 0.5 to 2 times the barometric climb rate
//!   there, an independent sensor's speed. `GTotal` is read with the axes where it is their
//!   magnitude within 0.11 on every row. The newer files' `NoGrav…` columns are read in g with the
//!   axes where the median magnitude of the axes less them, the gravity the phone took out, is
//!   0.95 to 1.05. As of 2026-10-13, (1) gives 1.00 to 1.08 and (2) 0.83 to 1.12 in 6 of the 8
//!   newer files; of the other two, one reads 1.19 in the second before its climb, the file
//!   starting at the launch, and one has no barometric climb. The older files have no barometer,
//!   so (2) can't run: their accelerations, which in 18 of the 19 read 0.94 to 1.06 at their
//!   quietest second and clip near 2 on each axis, are kept as written. The `NoGrav…` columns'
//!   median gives 1.00 to 1.12 in the 8, 6 of them within 0.95 to 1.05.
//! - *The attitude, in radians.* Each angle: (1) every reading within ±π (the pitch, ±π/2), the
//!   angle's extent, which a column in degrees or gradians fails once the phone turns past 3.14 of
//!   them from 0; (2) its direction against a sensor that measures it with no unit, which rules out
//!   every unit but radians on its own. On the rows where the accelerometer reads its rest, within
//!   3% of the magnitude of its quietest second (before the steepest climb where the barometer
//!   gives one, else in the whole file), the accelerometer's axes point up whatever their unit, and
//!   the magnetometer's field, taken across up, points east whatever its unit. The angle read in
//!   radians must lie from what they give by a median of at most 0.2 rad over at least 20 rows,
//!   and by at most half the median the angle read in degrees gives. Rows where the angle lies
//!   within 0.2 of 0 (for the roll, of a half-turn), where an angle in degrees reads nearly as one
//!   in radians, are left out. The angles are those of Android's `SensorManager.getOrientation`
//!   (Android API reference). `Pitch`, about x: |sin pitch| is gravity's share along the phone's y
//!   axis, `|Gy|` over the axes' magnitude. `Roll`, about y: its distance from the nearest
//!   half-turn is gravity's angle about y, atan(`|Gx|` / `|Gz|`), on rows with the y axis at least
//!   0.2 from vertical. Neither uses gravity's sign, so axes that point the other way pass too.
//!   `Yaw`, the azimuth: the angle from magnetic north toward east of the phone's y axis, with east
//!   the field across up and north up across east, on rows with the y axis at least 0.2 from
//!   vertical. As of 2026-10-13, (2) gives medians of 0.17 rad or less for the yaw in 10 files, the
//!   8 newer and 2 of the 19 older; in 16 of the other older ones the yaw lies a median 0.90 to
//!   3.02 rad from the field's azimuth, and one has too few rows, so their yaw is kept as written.
//!   For the roll, 0.07 rad or less in 24 files; 0.34 and 0.36 rad in two, and too few rows in
//!   one, kept.
//! - *The gyroscope, in rad/s.* (1) With the attitude read in radians, the turn between rows five
//!   apart (the angle of the rotation from one attitude to the other, each yaw about z, then pitch
//!   about x, then roll about y) is 0.7 to 1.3 times the gyroscope's rate summed over those rows,
//!   by the median over at least 10 such spans, those with an axis at its full scale left out; in
//!   degrees per second it would be 57 times smaller. (2) An axis sits at its largest reading on at
//!   least 5 rows, its full scale, and that is within 1% of one a MEMS gyroscope offers, ±125,
//!   ±250, ±500, ±1,000 or ±2,000 °/s (Bosch Sensortec, *BMI160 data sheet*, BST-BMI160-DS000,
//!   table 2), read in rad/s. As of 2026-10-13, (1) gives 0.80 to 1.02 in the 9 files whose
//!   attitude is read whole, and the newer files' gyroscopes stop at 1,000 °/s; the older ones'
//!   never stop at a full scale, and are kept as written.
//! - *The heading, in degrees.* (1) Its extent: every reading within 0 to 360 and one past 2π.
//!   Radians, which never pass 2π, always fail it; a unit that runs to more than 360, gradians to
//!   400, fails it once the heading passes 360 of it. (2) Its period, from its steps: some step
//!   between rows crosses north by more than 300 and, less 360 in its direction, turns as the
//!   steps on either side do, of their sign and within a factor of 2 of each. Crossing north, a
//!   heading steps by its period less its turn in that row. Radians can't step 300; a heading in
//!   gradians steps 400 less its turn, and less 360 that turns against the rows on either side
//!   unless the heading turns 80 to 100 gradians a row, 4 to 5 turns a second at 20 rows a second.
//! - *The position, in degrees.* `Latitude` and `Longitude` are decided together, both read or
//!   both kept, on two columns' extents: (1) the latitude's largest reading from 0 lies past π/2,
//!   which radians never pass, and within 90; (2) the longitude's past π and within 180. Radians
//!   fail both, as do scaled integers (such as 10⁻⁷°). NMEA 0183's degrees and minutes run
//!   together (4230.0 for 42.5°), and minutes of arc, pass the latitude's only within 1.5° of the
//!   equator and the longitude's only within 3° of the prime meridian: the pair passes both only
//!   for a site in the open sea of the Gulf of Guinea. A site in degrees within 1.57° of the
//!   equator or 3.14° of the prime meridian fails closed, both kept. Neither check can separate a
//!   unit within about 11% of the degree: the gradian, 0.9°, passes both below 81° of latitude and
//!   162° of longitude. No check in the file rules it out; the reader takes degrees over it
//!   because phones' location services give positions in degrees (Android's
//!   `Location.getLatitude`, Apple's `CLLocationDegrees`).
//! - *The time stamp, a Unix time in ms.* (1) Each step between rows, divided by 1,000, is the
//!   clock's step, `Seconds`, within 1.5 ms; (2) the first, read so, falls between 2000 and 2100, where
//!   read in seconds it would be tens of thousands of years on. It is kept as the time of day in
//!   UTC, [`Quantity::TimeOfDay`], s after midnight.
//! - *The GPS altitude, kept as written.* Against the pressure altitude at the GPS's fresh fixes
//!   (the rows where the altitude or position changes), a least-squares gain near 1 says meters
//!   and 3.28 feet, but that is one check, and the files hold no second. Fixes once a second, and
//!   lagging in flight, can't show gravity's pull in the coast. On the pad, the GPS's height above
//!   sea level against the pad's pressure altitude separates the units only on a high pad. The
//!   extremes of sea-level pressure on record, 870 to 1,083.8 hPa (WMO, *Records of Weather and
//!   Climate Extremes*), move the pressure altitude about −570 m to +1,270 m from the height, a
//!   band 1,840 m wide (wider with the GPS's own error). A reading `N` is `N` m read in meters and
//!   0.3048 `N` m read in feet, 0.695 `N` m apart, so both readings fall in the band for some
//!   weather unless `N` tops 1,840 / 0.695, about 2,650: a pad above about 2,650 m if the column is
//!   in meters, or above 810 m if it is in feet. That check isn't used: no sample file's pad reads
//!   that high, so it would settle none of them, and the gain it would pair with has no pass band
//!   of its own yet. The note gives the gain where the file has a barometer. As of 2026-10-13, the
//!   gain is 0.69 to 1.11 in 7 of the 8 newer files; in the eighth the GPS didn't follow the
//!   flight.
//!
//! **The primary track.** The barometric altitude, where it is read in meters, on every row, its
//! vertical speed `VelocityBarometer` where that is read in m/s. Otherwise the track is empty, as a
//! GPS log's is ([ADR-254][adr-254] §2), and the readings withhold on the log; the channels keep
//! every column. The temperature and battery tracks are always empty: neither column's unit is
//! settled.
//!
//! [adr-254]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md
//!
//! **The altitude's step.** `hPa` is written from a 32-bit float, which between 512 and 1,024 hPa
//! steps by 2⁻¹⁴ hPa: every pressure in the 8 newer sample files is a whole number of those. That
//! is 0.000 51 m of altitude at sea level, [`ALTITUDE_STEP_M`], the barometer's step; the
//! barometer's own noise is far larger. The track, `AltitudeRelativeBarometer`, is written in finer
//! steps of its own, as small as 0.25 mm in those files.
//!
//! **Values too large.** A cell is refused if its unit's conversion to SI makes it infinite, such
//! as `1.7e308` in `hPa` ([ADR-258][adr-258] §3).

use std::collections::HashSet;

use hpr_atmos::ussa76::Ussa76;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{self, Notes, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "phone app launch CSV";

/// The pressure's step, hPa: a 32-bit float's between 512 and 1,024 hPa, 2⁻¹⁴ hPa (module docs).
const PRESSURE_STEP_HPA: f64 = 1.0 / 16_384.0;

/// The barometer's step as an altitude, m: the `hPa` column's step, 2⁻¹⁴ hPa, at sea level, where
/// a pascal is `1 / (ρ₀ g₀)` m with the International Standard Atmosphere's ρ₀ = 1.225 kg/m³ (ISO
/// 2533:1975) and standard gravity g₀ (CGPM 1901): 0.000 508 m. It is the barometer's step, not the
/// track's: `AltitudeRelativeBarometer` is written in finer steps, as small as 0.25 mm.
pub const ALTITUDE_STEP_M: f64 = PRESSURE_STEP_HPA * 100.0 / (1.225 * G0_M_S2);

/// Standard gravity, m/s² (CGPM 1901).
const G0_M_S2: f64 = 9.806_65;

/// A foot, m (the international foot, 1959).
const FOOT_M: f64 = 0.3048;

/// The pad's pressure a launch site can have, hPa (module docs; [`crate::rrc3`]'s bounds).
const PAD_HPA: (f64, f64) = (500.0, 1_100.0);

/// The least climb, m, of the pressure altitude over which the barometric altitude is compared.
const MIN_CLIMB_M: f64 = 30.0;

/// How far, m and as a share of the climb, the barometric altitude may lie from the pressure
/// altitude.
const PRESSURE_MATCH: (f64, f64) = (3.0, 0.02);

/// The coast window, s before the highest altitude: from, to.
const COAST_WINDOW_S: (f64, f64) = (8.0, 3.0);

/// The least length, s, and rows of the coast window.
const MIN_COAST: (f64, usize) = (2.0, 10);

/// The span, s, over which the steepest climb is measured; its pairs of rows are at least half of
/// it apart.
const CLIMB_SPAN_S: f64 = 1.0;

/// The share of 1 g the coast must slow by at least, in meters; and of 1 g in feet, at most.
const COAST_SLACK: f64 = 0.7;

/// The gain of a barometric speed against the altitude's rate that reads it in m/s.
const SPEED_GAIN: (f64, f64) = (0.8, 1.25);

/// Half the span, s, over which the altitude's rate is taken about each row.
const RATE_HALF_SPAN_S: f64 = 0.25;

/// The quiet span, s, and its least rows, for the accelerometer at rest.
const REST: (f64, usize) = (1.0, 5);

/// The magnitude, g, an accelerometer at rest reads.
const REST_G: (f64, f64) = (0.9, 1.1);

/// The boost's speed over the barometric climb rate that reads the accelerometer in g.
const BOOST_RATIO: (f64, f64) = (0.5, 2.0);

/// How far `GTotal` may lie from the axes' magnitude, g, to be read with them.
const TOTAL_TIE_G: f64 = 0.11;

/// The median magnitude of the axes less the `NoGrav…` axes, g, that reads those in g.
const GRAVITY_TIE_G: (f64, f64) = (0.95, 1.05);

/// The attitude's direction checks, gravity's for the pitch and roll and the magnetic field's for
/// the yaw: the rest band's share, the least angle, rad, a row's angle and axis must keep from
/// where the comparison says nothing (module docs), the largest median difference, rad, and the
/// least rows.
const ATTITUDE_CHECK: (f64, f64, f64, usize) = (0.03, 0.2, 0.2, 20);

/// The gyroscope's rotation check: rows a span, the summed turn's bounds, rad, the ratio's bounds
/// and the least spans.
const GYRO_SPAN_ROWS: usize = 5;
const GYRO_TURN_RAD: (f64, f64) = (0.1, 2.5);
const GYRO_RATIO: (f64, f64) = (0.7, 1.3);
const GYRO_MIN_SPANS: usize = 10;

/// A gyroscope axis within this share of its largest reading is at its full scale.
const FULL_SCALE_SHARE: f64 = 0.99;

/// The least rows at the full scale, how near, as a share, a reading must be to the largest to be
/// at it, and the least of them in a row: a sensor at its rail holds there, where a turning axis
/// only passes its peak.
const PLATEAU: (usize, f64, usize) = (5, 0.001, 2);

/// MEMS gyroscopes' full scales, °/s (Bosch Sensortec, *BMI160 data sheet*, table 2).
const FULL_SCALES_DPS: [f64; 5] = [125.0, 250.0, 500.0, 1_000.0, 2_000.0];

/// How near, as a share, a plateau must be to a full scale.
const FULL_SCALE_MATCH: f64 = 0.01;

/// The least step, °, of a heading's wrap across north.
const HEADING_WRAP_DEG: f64 = 300.0;

/// How far, as a factor either way, a wrap's step less 360° may lie from the steps on either side
/// of it.
const HEADING_WRAP_FACTOR: f64 = 2.0;

/// How far, s, a `Timestamp` step over 1,000 may lie from the clock's, `Seconds`.
const STAMP_STEP_S: f64 = 0.0015;

/// 2000-01-01 and 2100-01-01, 00:00 UTC, in ms since 1970.
const STAMP_RANGE_MS: (f64, f64) = (946_684_800_000.0, 4_102_444_800_000.0);

/// The least fresh fixes, and the least climb of the pressure altitude among them, m, for the GPS
/// altitude's gain.
const MIN_FIXES: usize = 10;

/// A day, s.
const DAY_S: f64 = 86_400.0;

/// What a header column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Clock,
    Stamp,
    GpsAltitude,
    Latitude,
    Longitude,
    /// One of the accelerometer's axes, `Gx`, `Gy` or `Gz`.
    Axis,
    Total,
    /// One of the axes with gravity taken out.
    NoGravAxis,
    NoGravTotal,
    Gyro,
    Heading,
    Yaw,
    Pitch,
    Roll,
    BaroAltitude,
    Pressure,
    BaroSpeed,
    /// A flag, state or count, kept as a number.
    Code,
    /// A running extreme of another column.
    Extreme,
    Temperature,
    /// A column kept as written for the reason given.
    Kept(&'static str),
    Unknown,
}

/// Why a magnetometer's column is kept.
const MAGNETIC: &str = "by name a magnetometer's reading, in a unit no source states, and the \
                        record has no quantity for a magnetic field";
/// Why the turn counts are kept.
const TURNS: &str = "by name a count of turns or its clock, which no check in the file ties \
                     to the gyroscope";
/// Why the GPS's accuracies are kept.
const ACCURACY: &str = "by name the GPS's accuracy, in a unit no source states";
/// Why the external sensor's columns are kept.
const EXTERNAL: &str = "by name an external sensor's, in a unit no source states";
/// Why the reserved columns are kept.
const RESERVED: &str = "reserved, by name; no source states what it holds";

/// What a column named so holds.
fn kind_of(name: &str) -> Kind {
    match name {
        "Seconds" => Kind::Clock,
        "Timestamp" => Kind::Stamp,
        "AltitudeMSLGPS" | "AltitudeMSLMaxGPS" => Kind::GpsAltitude,
        "Latitude" => Kind::Latitude,
        "Longitude" => Kind::Longitude,
        "Gx" | "Gy" | "Gz" => Kind::Axis,
        "GTotal" => Kind::Total,
        "NoGravGx" | "NoGravGy" | "NoGravGz" => Kind::NoGravAxis,
        "NoGravGTotal" => Kind::NoGravTotal,
        "GyroX" | "GyroY" | "GyroZ" => Kind::Gyro,
        "Heading" => Kind::Heading,
        "Yaw" => Kind::Yaw,
        "Pitch" => Kind::Pitch,
        "Roll" => Kind::Roll,
        "AltitudeRelativeBarometer" => Kind::BaroAltitude,
        "hPa" => Kind::Pressure,
        "VelocityBarometer" | "VelocityBarometerRaw" => Kind::BaroSpeed,
        "Record" | "Static" | "UsbState" | "Satellites" => Kind::Code,
        "GMax"
        | "NoGravGMax"
        | "AltitudeRelativeMaxBarometer"
        | "VelocityBarometerMax"
        | "VelocityBarometerMin" => Kind::Extreme,
        "Temperature" => Kind::Temperature,
        "MagnetometerX" | "MagnetometerY" | "MagnetometerZ" | "HeadingX" | "HeadingY"
        | "HeadingZ" => Kind::Kept(MAGNETIC),
        "Revolutions" | "RevApogee" | "RevolutionsClock" => Kind::Kept(TURNS),
        "HorizontalResGPS" | "VerticalResGPS" => Kind::Kept(ACCURACY),
        "Battery" => Kind::Kept("by name the battery's state, in a unit no source states"),
        "AltitudeRelativeSerial" | "AltitudeRelativeMaxSerial" => Kind::Kept(EXTERNAL),
        _ if name.starts_with("Usb") => Kind::Kept(EXTERNAL),
        _ if name.starts_with("Reserved") => Kind::Kept(RESERVED),
        _ => Kind::Unknown,
    }
}

/// Whether the text is the app's CSV: a header row opening `Seconds,Timestamp,AltitudeMSLGPS`.
pub fn recognizes(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with("Seconds,Timestamp,AltitudeMSLGPS"))
}

/// A header column: what it holds and its name as written.
struct Column {
    kind: Kind,
    name: String,
}

/// The rows read: each row's line and time, and each column's cells, `NaN` for a gap.
struct Table {
    columns: Vec<Column>,
    lines: Vec<usize>,
    time_s: Vec<f64>,
    /// Per header column, the clock's too.
    values: Vec<Vec<f64>>,
}

impl Table {
    /// The cells of the column named so.
    fn column(&self, name: &str) -> Option<&[f64]> {
        self.index(name).map(|index| self.values[index].as_slice())
    }

    fn index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|column| column.name == name)
    }

    /// The names of the columns of a kind, in the file's order.
    fn names_of(&self, kinds: &[Kind]) -> Vec<&str> {
        self.columns
            .iter()
            .filter(|column| kinds.contains(&column.kind))
            .map(|column| column.name.as_str())
            .collect()
    }
}

/// Reads a phone app's `launch….csv` from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't the app's header;
/// [`LogError::Syntax`] for a header naming a column twice or leaving a name empty, a row of the
/// wrong width, a cell that isn't a number, a time that is missing or doesn't increase, or a value
/// its unit's conversion makes infinite, naming the line; [`LogError::NoData`] if no row follows
/// the header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text).skip_while(|(_, line)| line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(not_this_format());
    };
    if !header
        .trim()
        .starts_with("Seconds,Timestamp,AltitudeMSLGPS")
    {
        return Err(not_this_format());
    }
    let columns = columns(header, header_line)?;
    let table = read_rows(lines, columns)?;
    if table.time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    build(&table)
}

fn not_this_format() -> LogError {
    LogError::NotThisFormat {
        format: FORMAT,
        message: "its first line isn't the app's header, opening \
                  `Seconds,Timestamp,AltitudeMSLGPS`"
            .to_owned(),
    }
}

/// The header's columns; a name given twice, or left empty, is refused.
fn columns(header: &str, line: usize) -> Result<Vec<Column>, LogError> {
    let mut columns = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if !seen.insert(name) {
            return Err(syntax(
                line,
                format!("the header names {:?} twice", short(name)),
            ));
        }
        columns.push(Column {
            kind: kind_of(name),
            name: name.to_owned(),
        });
    }
    Ok(columns)
}

/// Reads the data rows (module docs, *Rows*).
fn read_rows<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
    columns: Vec<Column>,
) -> Result<Table, LogError> {
    let width = columns.len();
    let mut table = Table {
        values: columns.iter().map(|_| Vec::new()).collect(),
        columns,
        lines: Vec::new(),
        time_s: Vec::new(),
    };
    for (number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        // One more than the header names, so that a wider row is still told apart.
        let cells: Vec<&str> = line.split(',').map(str::trim).take(width + 1).collect();
        if cells.len() != width {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells where the header names {width}",
                    if cells.len() > width {
                        format!("more than {width}")
                    } else {
                        cells.len().to_string()
                    }
                ),
            ));
        }
        let mut row = Vec::with_capacity(width);
        for (column, cell) in table.columns.iter().zip(&cells) {
            row.push(if cell.is_empty() {
                f64::NAN
            } else {
                table::number(cell, false).ok_or_else(|| {
                    syntax(
                        number,
                        format!("{:?} under {:?} isn't a number", short(cell), column.name),
                    )
                })?
            });
        }
        let time_s = row[0];
        if time_s.is_nan() {
            return Err(syntax(number, "the row has no time".to_owned()));
        }
        if let Some(&last) = table.time_s.last()
            && time_s <= last
        {
            return Err(syntax(
                number,
                format!("the time {time_s} s doesn't come after the one above it, {last} s"),
            ));
        }
        for (store, value) in table.values.iter_mut().zip(row) {
            store.push(value);
        }
        table.lines.push(number);
        table.time_s.push(time_s);
    }
    Ok(table)
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// The names as a note lists them: `` `a`, `b` and `c` ``.
fn listed(names: &[&str]) -> String {
    let quoted: Vec<String> = names
        .iter()
        .map(|name| format!("`{}`", short(name)))
        .collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// How a column's values become the record's.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Convert {
    AsWritten,
    Scale(f64),
    /// A Unix time in ms to the time of day in UTC, s.
    TimeOfDay,
}

/// A column's reading: its quantity and conversion.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Reading {
    quantity: Quantity,
    convert: Convert,
}

const KEPT: Reading = Reading {
    quantity: Quantity::Unrecognized,
    convert: Convert::AsWritten,
};

/// The outcome of a pair of checks: the evidence, and whether both passed.
struct Outcome {
    passed: bool,
    evidence: String,
}

impl Outcome {
    /// Both checks' evidence, the outcome passing only if both did.
    fn both(first: (bool, String), second: (bool, String)) -> Self {
        Self {
            passed: first.0 && second.0,
            evidence: format!("(1) {}. (2) {}", first.1, second.1),
        }
    }
}

/// The record from the rows read: each column's unit decided, the notes and the channels.
fn build(table: &Table) -> Result<FlightLog, LogError> {
    let mut readings: Vec<Reading> = vec![KEPT; table.columns.len()];
    let mut notes: Vec<String> = vec![
        "the app isn't named in the file. No source describes its format, so every unit is \
         decided from the file's own values or kept as written"
            .to_owned(),
    ];
    let mut set = |table: &Table, names: &[&str], reading: Reading| {
        for name in names {
            if let Some(index) = table.index(name) {
                readings[index] = reading;
            }
        }
    };
    let time = &table.time_s;

    // The pressure.
    let pressure_pa: Option<Vec<f64>> = table.column("hPa").and_then(|hpa| {
        let first = hpa.iter().copied().find(|v| v.is_finite());
        let within = first.is_some_and(|p| (PAD_HPA.0..=PAD_HPA.1).contains(&p));
        let evidence = first.map_or("it holds no reading".to_owned(), |p| {
            format!("the first reading, on the pad, is {p:.2}")
        });
        if within {
            notes.push(format!(
                "`hPa` read in hectopascals, from its name and the pad: {evidence}, within the \
                 {} to {} hPa of a launch site",
                PAD_HPA.0, PAD_HPA.1
            ));
            Some(hpa.iter().map(|v| v * 100.0).collect())
        } else {
            notes.push(format!(
                "`hPa` kept as written, its name refuted by the pad: {evidence}, outside the {} \
                 to {} hPa of a launch site",
                PAD_HPA.0, PAD_HPA.1
            ));
            None
        }
    });
    if pressure_pa.is_some() {
        set(
            table,
            &["hPa"],
            Reading {
                quantity: Quantity::Pressure,
                convert: Convert::Scale(100.0),
            },
        );
    }
    let pressure_altitude: Option<Vec<f64>> = pressure_pa.as_ref().map(|pa| {
        let atmosphere = Ussa76::standard();
        pa.iter()
            .map(|p| atmosphere.pressure_altitude_m(*p).unwrap_or(f64::NAN))
            .collect()
    });

    // The barometric altitude.
    let altitude = table.column("AltitudeRelativeBarometer");
    let coast = altitude.and_then(|alt| coast(time, alt));
    let mut altitude_m = false;
    if let Some(alt) = altitude {
        if alt.iter().all(|v| *v == 0.0 || v.is_nan()) {
            notes.push(
                "`AltitudeRelativeBarometer` kept as written, holding no reading: it is 0 on \
                 every row, as in a file from a phone without a barometer"
                    .to_owned(),
            );
        } else {
            let outcome = Outcome::both(
                matches_pressure(alt, pressure_altitude.as_deref()),
                coast_check(coast.as_ref(), |c| decel_of_altitude(time, alt, c)),
            );
            if outcome.passed {
                altitude_m = true;
                notes.push(format!(
                    "`AltitudeRelativeBarometer` read in meters, from the pressure and the \
                     coast: {}",
                    outcome.evidence
                ));
                set(
                    table,
                    &["AltitudeRelativeBarometer"],
                    Reading {
                        quantity: Quantity::AltitudeAboveGround,
                        convert: Convert::Scale(1.0),
                    },
                );
            } else {
                notes.push(format!(
                    "`AltitudeRelativeBarometer` kept as written, in a unit two checks don't \
                     settle: {}",
                    outcome.evidence
                ));
            }
        }
    }
    let coast = coast.filter(|_| altitude_m);

    // The barometric speeds.
    let mut speed_m_s = false;
    if let Some(alt) = altitude.filter(|_| altitude_m) {
        let rate = rate_of(time, alt);
        for name in table.names_of(&[Kind::BaroSpeed]) {
            let Some(speed) = table.column(name) else {
                continue;
            };
            let gain = slope(rate.iter().copied().zip(speed.iter().copied()));
            let first = match gain {
                Some(gain) => (
                    (SPEED_GAIN.0..=SPEED_GAIN.1).contains(&gain),
                    format!(
                        "against the barometric altitude's rate it has a gain of {gain:.3}, \
                         where {} to {} reads m/s",
                        SPEED_GAIN.0, SPEED_GAIN.1
                    ),
                ),
                None => (
                    false,
                    "it has no rows beside the barometric altitude's rate".to_owned(),
                ),
            };
            let second = coast_check(coast.as_ref(), |c| {
                let rows = &c.window;
                slope(rows.iter().map(|&row| (time[row], speed[row]))).map(|s| -s)
            });
            let outcome = Outcome::both(first, second);
            if outcome.passed {
                speed_m_s |= name == "VelocityBarometer";
                notes.push(format!(
                    "`{}` read in meters per second, from the barometric altitude and the \
                     coast: {}",
                    short(name),
                    outcome.evidence
                ));
                set(
                    table,
                    &[name],
                    Reading {
                        quantity: Quantity::Speed,
                        convert: Convert::Scale(1.0),
                    },
                );
            } else {
                notes.push(format!(
                    "`{}` kept as written, in a unit two checks don't settle: {}",
                    short(name),
                    outcome.evidence
                ));
            }
        }
    } else {
        let names = table.names_of(&[Kind::BaroSpeed]);
        if !names.is_empty() {
            notes.push(format!(
                "{} kept as written, in a unit two checks don't settle: the barometric altitude \
                 they would be checked against isn't read in meters",
                listed(&names)
            ));
        }
    }

    // The accelerometer.
    accelerations(table, coast.as_ref(), &mut notes, &mut set);

    // The attitude and the gyroscope.
    let up = rest_directions(table, coast.as_ref());
    let mut radians = [false; 3];
    for (slot, name) in ["Yaw", "Pitch", "Roll"].into_iter().enumerate() {
        let Some(values) = table.column(name) else {
            continue;
        };
        let (outcome, evidence) = match name {
            "Pitch" => (
                Outcome::both(
                    within(values, std::f64::consts::FRAC_PI_2, "±π/2"),
                    pitch_gravity(up.as_deref(), values),
                ),
                "gravity's direction",
            ),
            "Roll" => (
                Outcome::both(
                    within(values, std::f64::consts::PI, "±π"),
                    roll_gravity(up.as_deref(), values),
                ),
                "gravity's direction",
            ),
            _ => (
                Outcome::both(
                    within(values, std::f64::consts::PI, "±π"),
                    yaw_magnetic(table, up.as_deref(), values),
                ),
                "the magnetic field's direction",
            ),
        };
        if outcome.passed {
            radians[slot] = true;
            notes.push(format!(
                "`{name}` read in radians, from its range and {evidence}: {}",
                outcome.evidence
            ));
            set(
                table,
                &[name],
                Reading {
                    quantity: Quantity::Angle,
                    convert: Convert::Scale(180.0 / std::f64::consts::PI),
                },
            );
        } else {
            notes.push(format!(
                "`{name}` kept as written, in a unit two checks don't settle: {}",
                outcome.evidence
            ));
        }
    }
    gyroscope(table, radians == [true; 3], &mut notes, &mut set);
    if let Some(values) = table.column("Heading") {
        let outcome = Outcome::both(heading_range(values), heading_wraps(values));
        if outcome.passed {
            notes.push(format!(
                "`Heading` read in degrees, from its range and its wrap's period: {}",
                outcome.evidence
            ));
            set(
                table,
                &["Heading"],
                Reading {
                    quantity: Quantity::Angle,
                    convert: Convert::Scale(1.0),
                },
            );
        } else {
            notes.push(format!(
                "`Heading` kept as written, in a unit two checks don't settle: {}",
                outcome.evidence
            ));
        }
    }

    // The position, the time stamp and the GPS altitude.
    let position = table.names_of(&[Kind::Latitude, Kind::Longitude]);
    if !position.is_empty() {
        let outcome = Outcome::both(
            position_range(table, "Latitude", 90.0),
            position_range(table, "Longitude", 180.0),
        );
        if outcome.passed {
            notes.push(format!(
                "{} read in degrees, from the latitude's range and the longitude's: {}",
                listed(&position),
                outcome.evidence
            ));
            for (name, quantity) in [
                ("Latitude", Quantity::Latitude),
                ("Longitude", Quantity::Longitude),
            ] {
                set(
                    table,
                    &[name],
                    Reading {
                        quantity,
                        convert: Convert::Scale(1.0),
                    },
                );
            }
        } else {
            notes.push(format!(
                "{} kept as written, in a unit two checks don't settle: {}",
                listed(&position),
                outcome.evidence
            ));
        }
    }
    if let Some(stamp) = table.column("Timestamp") {
        let outcome = Outcome::both(stamp_steps(time, stamp), stamp_date(stamp));
        if outcome.passed {
            notes.push(format!(
                "`Timestamp` read in milliseconds since 1970 UTC, from the clock and the date: \
                 {}; kept as the time of day in UTC",
                outcome.evidence
            ));
            set(
                table,
                &["Timestamp"],
                Reading {
                    quantity: Quantity::TimeOfDay,
                    convert: Convert::TimeOfDay,
                },
            );
        } else {
            notes.push(format!(
                "`Timestamp` kept as written, in a unit two checks don't settle: {}",
                outcome.evidence
            ));
        }
    }
    let gps = table.names_of(&[Kind::GpsAltitude]);
    if !gps.is_empty() {
        notes.push(format!(
            "{} kept as written, in a unit no second check settles: {}",
            listed(&gps),
            gps_evidence(table, pressure_altitude.as_deref())
        ));
    }

    // The rest, as written.
    kept_notes(table, &mut notes, &mut set);
    let mut line_notes = Notes::default();
    for column in &table.columns {
        if column.kind == Kind::Unknown {
            line_notes.push(format!(
                "`{}` kept as written, in a unit no source states: this reader doesn't know the \
                 column",
                short(&column.name)
            ));
        }
    }
    notes.extend(line_notes.into_vec());

    // The channels.
    let mut log = FlightLog {
        format: LogFormat::PhoneLaunchCsv,
        logger: "phone app".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::new(),
        events: Vec::new(),
        notes,
    };
    for ((column, values), reading) in table.columns.iter().zip(&table.values).zip(&readings) {
        if column.kind == Kind::Clock {
            continue;
        }
        let mut converted = Vec::with_capacity(values.len());
        for (value, line) in values.iter().zip(&table.lines) {
            let value = match reading.convert {
                Convert::AsWritten => *value,
                Convert::Scale(scale) => value * scale,
                Convert::TimeOfDay => (value / 1000.0).rem_euclid(DAY_S),
            };
            converted.push(table::finite(value, FORMAT, *line, &column.name)?);
        }
        if column.name == "AltitudeRelativeBarometer" && altitude_m {
            log.altitude_m.clone_from(&converted);
            log.time_s.clone_from(time);
        } else if column.name == "VelocityBarometer" && speed_m_s && altitude_m {
            log.vertical_speed_m_s = Some(converted.clone());
        }
        log.channels.push(Channel {
            column: column.name.clone(),
            quantity: reading.quantity,
            time_s: time.clone(),
            values: converted,
        });
    }
    if !altitude_m {
        log.notes.push(
            "the primary track is empty: the file's barometric altitude isn't read in meters, so \
             the readings withhold on the log; every column is a channel"
                .to_owned(),
        );
    }
    Ok(log)
}

/// The notes on the columns kept as written for their kind, and the codes read as numbers.
fn kept_notes(
    table: &Table,
    notes: &mut Vec<String>,
    set: &mut impl FnMut(&Table, &[&str], Reading),
) {
    let codes = table.names_of(&[Kind::Code]);
    if !codes.is_empty() {
        notes.push(format!(
            "{} kept as written, as numbers with no unit: by their names flags, states or \
             counts",
            listed(&codes)
        ));
        set(
            table,
            &codes,
            Reading {
                quantity: Quantity::Number,
                convert: Convert::AsWritten,
            },
        );
    }
    let extremes = table.names_of(&[Kind::Extreme]);
    if !extremes.is_empty() {
        notes.push(format!(
            "{} kept as written, as running extremes. By their names they are the most or least of \
             another column so far. The reader doesn't tie them to that column's unit",
            listed(&extremes)
        ));
    }
    if let Some(values) = table.column("Temperature") {
        notes.push(format!(
            "`Temperature` kept as written, in a unit no two checks settle. Its readings, {}, \
             would be a phone's warmth in degrees Celsius or a cool day's in degrees Fahrenheit. \
             Nothing else in the file tells them apart",
            span(values)
        ));
    }
    let mut reasons: Vec<&'static str> = Vec::new();
    for column in &table.columns {
        if let Kind::Kept(why) = column.kind
            && !reasons.contains(&why)
        {
            reasons.push(why);
        }
    }
    for why in reasons {
        let names: Vec<&str> = table
            .columns
            .iter()
            .filter(|column| column.kind == Kind::Kept(why))
            .map(|column| column.name.as_str())
            .collect();
        notes.push(format!("{} kept as written: {why}", listed(&names)));
    }
}

/// The least and greatest of the readings, as a note writes them.
fn span(values: &[f64]) -> String {
    let (low, high) = values
        .iter()
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &v| {
            (low.min(v), high.max(v))
        });
    if low.is_finite() {
        format!("{} to {}", brief(low), brief(high))
    } else {
        "none".to_owned()
    }
}

/// A value to three decimals at most, as a note writes it: `37.2`, not `37.20000076293945`.
fn brief(value: f64) -> String {
    let written = format!("{value:.3}");
    let trimmed = written.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// A value to two decimals, a rounded 0 written without its sign.
fn hundredths(value: f64) -> String {
    let written = format!("{value:.2}");
    if written == "-0.00" {
        "0.00".to_owned()
    } else {
        written
    }
}

/// The least-squares slope of `y` on `x` over the pairs with both finite; `None` with fewer than
/// three or no spread in `x`.
fn slope(pairs: impl Iterator<Item = (f64, f64)>) -> Option<f64> {
    let points: Vec<(f64, f64)> = pairs
        .filter(|(x, y)| x.is_finite() && y.is_finite())
        .collect();
    if points.len() < 3 {
        return None;
    }
    let n = points.len() as f64;
    let mean_x = points.iter().map(|p| p.0).sum::<f64>() / n;
    let mean_y = points.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = points.iter().map(|p| (p.0 - mean_x).powi(2)).sum();
    let sxy: f64 = points.iter().map(|p| (p.0 - mean_x) * (p.1 - mean_y)).sum();
    (sxx > 0.0).then(|| sxy / sxx)
}

/// The second derivative of a least-squares parabola through the points, `2c` of
/// `y = a + b t + c t²`; `None` with fewer than four points or a singular fit.
fn curvature(points: &[(f64, f64)]) -> Option<f64> {
    if points.len() < 4 {
        return None;
    }
    let n = points.len() as f64;
    let mean_t = points.iter().map(|p| p.0).sum::<f64>() / n;
    // Sums of powers of the centered time, and of y times them.
    let mut s = [0.0_f64; 5];
    let mut r = [0.0_f64; 3];
    for &(t, y) in points {
        let x = t - mean_t;
        let mut power = 1.0;
        for (k, sum) in s.iter_mut().enumerate() {
            *sum += power;
            if k < 3 {
                r[k] += y * power;
            }
            power *= x;
        }
    }
    // The normal equations, solved by Cramer's rule.
    let m = [[s[0], s[1], s[2]], [s[1], s[2], s[3]], [s[2], s[3], s[4]]];
    let det = |m: &[[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let whole = det(&m);
    if whole.abs() < f64::MIN_POSITIVE || !whole.is_finite() {
        return None;
    }
    let mut third = m;
    for (row, value) in third.iter_mut().zip(r) {
        row[2] = value;
    }
    Some(2.0 * det(&third) / whole)
}

/// The median of the finite values; `None` if there are none.
fn median(mut values: Vec<f64>) -> Option<f64> {
    values.retain(|v| v.is_finite());
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    Some(if values.len().is_multiple_of(2) {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    })
}

/// The coast before the highest barometric altitude, measured on the column as written.
struct Coast {
    /// The steepest climb's rate, the column's unit per second, and its middle, s.
    climb: (f64, f64),
    /// The rows of the window (module docs), and its times, s.
    window: Vec<usize>,
    window_s: (f64, f64),
}

/// The coast of an altitude column (module docs, *The barometric altitude*); `None` without a
/// highest altitude or a climb before it. The window may be short: [`coast_check`] says so.
fn coast(time: &[f64], altitude: &[f64]) -> Option<Coast> {
    let apogee = altitude
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_finite())
        .fold(None, |best: Option<(usize, f64)>, (row, &v)| match best {
            Some((_, top)) if top >= v => best,
            _ => Some((row, v)),
        })?
        .0;
    let mut climb: Option<(f64, f64)> = None;
    let mut from = 0;
    for to in 0..=apogee {
        while time[to] - time[from] > CLIMB_SPAN_S {
            from += 1;
        }
        let dt = time[to] - time[from];
        if dt >= CLIMB_SPAN_S / 2.0 {
            let rate = (altitude[to] - altitude[from]) / dt;
            if rate.is_finite() && climb.is_none_or(|(best, _)| rate > best) {
                climb = Some((rate, (time[to] + time[from]) / 2.0));
            }
        }
    }
    let climb = climb.filter(|(rate, _)| *rate > 0.0)?;
    let start = (time[apogee] - COAST_WINDOW_S.0).max(climb.1 + CLIMB_SPAN_S);
    let end = time[apogee] - COAST_WINDOW_S.1;
    let window = (0..apogee)
        .filter(|&row| (start..=end).contains(&time[row]) && altitude[row].is_finite())
        .collect();
    Some(Coast {
        climb,
        window,
        window_s: (start, end),
    })
}

/// How fast an altitude column falls in the coast window, the column's unit per second squared.
fn decel_of_altitude(time: &[f64], altitude: &[f64], coast: &Coast) -> Option<f64> {
    let points: Vec<(f64, f64)> = coast
        .window
        .iter()
        .map(|&row| (time[row], altitude[row]))
        .collect();
    curvature(&points).map(|c| -c)
}

/// The coast check: the slowing `decel` measures, in the column's unit per second squared, within
/// [`COAST_SLACK`] g to [`COAST_SLACK`] g in feet.
fn coast_check(coast: Option<&Coast>, decel: impl FnOnce(&Coast) -> Option<f64>) -> (bool, String) {
    let Some(coast) = coast else {
        return (
            false,
            "the file has no climb to a highest altitude to find the coast in".to_owned(),
        );
    };
    let (start, end) = coast.window_s;
    if end - start < MIN_COAST.0 || coast.window.len() < MIN_COAST.1 {
        return (
            false,
            format!(
                "the coast before the highest altitude holds {} rows over {:.2} s, fewer than {} \
                 over {} s. The coast runs from a second after the steepest climb to 3 s before \
                 the top",
                coast.window.len(),
                (end - start).max(0.0),
                MIN_COAST.1,
                MIN_COAST.0
            ),
        );
    }
    let low = COAST_SLACK * G0_M_S2;
    let high = COAST_SLACK * G0_M_S2 / FOOT_M;
    match decel(coast) {
        Some(value) => (
            (low..high).contains(&value),
            format!(
                "in the coast from {start:.2} s to {end:.2} s it slows by {} per second per \
                 second, {} g in meters. Gravity and drag give at least 1 g, and {low:.2} to \
                 {high:.2} m/s² reads meters",
                hundredths(value),
                hundredths(value / G0_M_S2)
            ),
        ),
        None => (false, "its coast can't be fitted".to_owned()),
    }
}

/// Check 1 of the barometric altitude: its climb above the first row against the pressure
/// altitude's.
fn matches_pressure(altitude: &[f64], pressure_altitude: Option<&[f64]>) -> (bool, String) {
    let Some(pressure_altitude) = pressure_altitude else {
        return (false, "the file has no pressure read in hPa".to_owned());
    };
    let pairs: Vec<(f64, f64)> = altitude
        .iter()
        .zip(pressure_altitude)
        .map(|(a, p)| (*a, *p))
        .filter(|(a, p)| a.is_finite() && p.is_finite())
        .collect();
    let Some(&(a0, p0)) = pairs.first() else {
        return (false, "it has no row beside a pressure altitude".to_owned());
    };
    let (low, high) = pairs
        .iter()
        .fold((0.0_f64, 0.0_f64), |(low, high), (_, p)| {
            (low.min(p - p0), high.max(p - p0))
        });
    let climb = high - low;
    let off = pairs
        .iter()
        .map(|(a, p)| ((a - a0) - (p - p0)).abs())
        .fold(0.0_f64, f64::max);
    let allowed = PRESSURE_MATCH.0 + PRESSURE_MATCH.1 * climb;
    if climb < MIN_CLIMB_M {
        return (
            false,
            format!(
                "the pressure altitude climbs {climb:.1} m, under the {MIN_CLIMB_M} m that \
                 tells meters from feet"
            ),
        );
    }
    (
        off <= allowed,
        format!(
            "above the first row it lies at most {off:.2} from the pressure altitude of `hPa` in \
             the U.S. Standard Atmosphere, in meters. The climb is {climb:.1} m, where \
             {allowed:.1} is allowed"
        ),
    )
}

/// The rate of an altitude column at each row, over [`RATE_HALF_SPAN_S`] either side.
fn rate_of(time: &[f64], altitude: &[f64]) -> Vec<f64> {
    let mut rates = Vec::with_capacity(time.len());
    let (mut low, mut high) = (0, 0);
    for row in 0..time.len() {
        while time[row] - time[low] > RATE_HALF_SPAN_S {
            low += 1;
        }
        while high + 1 < time.len() && time[high + 1] - time[row] <= RATE_HALF_SPAN_S {
            high += 1;
        }
        let dt = time[high] - time[low];
        rates.push(if dt > 0.0 {
            (altitude[high] - altitude[low]) / dt
        } else {
            f64::NAN
        });
    }
    rates
}

/// The quietest stretch of [`REST`]'s span among the rows before `before`, by the summed variance
/// of the three axes: its last row and its mean magnitude.
fn quietest(time: &[f64], axes: [&[f64]; 3], before: usize) -> Option<(usize, f64)> {
    let n = before.min(time.len());
    // Running sums of each axis and its square, and of rows with a gap.
    let mut sums = vec![[0.0_f64; 6]; n + 1];
    let mut gaps = vec![0_usize; n + 1];
    for row in 0..n {
        let mut next = sums[row];
        let mut gap = 0;
        for (k, axis) in axes.iter().enumerate() {
            let v = axis[row];
            if v.is_finite() {
                next[2 * k] += v;
                next[2 * k + 1] += v * v;
            } else {
                gap = 1;
            }
        }
        sums[row + 1] = next;
        gaps[row + 1] = gaps[row] + gap;
    }
    let mut best: Option<(f64, usize, [f64; 3])> = None;
    let mut end = 0;
    for start in 0..n {
        end = end.max(start);
        while end < n && time[end] - time[start] <= REST.0 {
            end += 1;
        }
        let rows = end - start;
        if rows < REST.1 || gaps[end] != gaps[start] {
            continue;
        }
        let count = rows as f64;
        let mut variance = 0.0;
        let mut mean = [0.0; 3];
        for (k, m) in mean.iter_mut().enumerate() {
            let s1 = sums[end][2 * k] - sums[start][2 * k];
            let s2 = sums[end][2 * k + 1] - sums[start][2 * k + 1];
            *m = s1 / count;
            variance += s2 / count - *m * *m;
        }
        if best.is_none_or(|(least, ..)| variance < least) {
            best = Some((variance, end - 1, mean));
        }
    }
    best.map(|(_, last, mean)| (last, mean.iter().map(|m| m * m).sum::<f64>().sqrt()))
}

/// The accelerometer's columns (module docs, *The accelerations*).
fn accelerations(
    table: &Table,
    coast: Option<&Coast>,
    notes: &mut Vec<String>,
    set: &mut impl FnMut(&Table, &[&str], Reading),
) {
    let (Some(x), Some(y), Some(z)) = (table.column("Gx"), table.column("Gy"), table.column("Gz"))
    else {
        let names = table.names_of(&[Kind::Axis, Kind::Total, Kind::NoGravAxis, Kind::NoGravTotal]);
        if !names.is_empty() {
            notes.push(format!(
                "{} kept as written, in a unit two checks don't settle: the file hasn't all three \
                 of the accelerometer's axes",
                listed(&names)
            ));
        }
        return;
    };
    let time = &table.time_s;
    let magnitude: Vec<f64> = (0..time.len())
        .map(|row| (x[row] * x[row] + y[row] * y[row] + z[row] * z[row]).sqrt())
        .collect();
    let mut group = vec!["Gx", "Gy", "Gz"];
    if let Some(total) = table.column("GTotal") {
        let off = total
            .iter()
            .zip(&magnitude)
            .filter(|(t, m)| t.is_finite() && m.is_finite())
            .map(|(t, m)| (t - m).abs())
            .fold(0.0_f64, f64::max);
        if off <= TOTAL_TIE_G {
            group.push("GTotal");
        } else {
            notes.push(format!(
                "`GTotal` kept as written, not the axes' magnitude: it lies up to {off:.3} from \
                 it, more than {TOTAL_TIE_G}"
            ));
        }
    }
    let burnout_row = climb_row(time, coast);
    let rest = quietest(time, [x, y, z], burnout_row.unwrap_or(time.len()));
    let first = match rest {
        Some((last, g)) => (
            (REST_G.0..=REST_G.1).contains(&g),
            format!(
                "the quietest second{}, to {:.2} s, reads {g:.3} in the axes' magnitude, where \
                 {} to {} reads g at rest",
                if burnout_row.is_some() {
                    " before the steepest climb"
                } else {
                    ""
                },
                time[last],
                REST_G.0,
                REST_G.1
            ),
        ),
        None => (false, "the file has no quiet second".to_owned()),
    };
    let second = match (coast, rest, burnout_row) {
        (Some(coast), Some((last, _)), Some(burnout_row)) => {
            let speed: f64 = (last..burnout_row)
                .filter(|&row| row + 1 < time.len() && magnitude[row].is_finite())
                .map(|row| (magnitude[row] - 1.0) * G0_M_S2 * (time[row + 1] - time[row]))
                .sum();
            let ratio = speed / coast.climb.0;
            (
                (BOOST_RATIO.0..=BOOST_RATIO.1).contains(&ratio),
                format!(
                    "from then to the steepest climb, at {:.2} s, the magnitude less 1 g, read in \
                     g, gives {speed:.1} m/s. That is {ratio:.2} times the barometric climb rate, where \
                     {} to {} reads g",
                    coast.climb.1, BOOST_RATIO.0, BOOST_RATIO.1
                ),
            )
        }
        _ => (
            false,
            "the file has no barometric climb in meters to check the boost against".to_owned(),
        ),
    };
    let outcome = Outcome::both(first, second);
    let reading = Reading {
        quantity: Quantity::Acceleration,
        convert: Convert::Scale(G0_M_S2),
    };
    if outcome.passed {
        notes.push(format!(
            "{} read in standard gravity, from the pad and the boost: {}",
            listed(&group),
            outcome.evidence
        ));
        set(table, &group, reading);
    } else {
        notes.push(format!(
            "{} kept as written, in a unit two checks don't settle: {}",
            listed(&group),
            outcome.evidence
        ));
    }
    let (Some(nx), Some(ny), Some(nz)) = (
        table.column("NoGravGx"),
        table.column("NoGravGy"),
        table.column("NoGravGz"),
    ) else {
        let names = table.names_of(&[Kind::NoGravAxis, Kind::NoGravTotal]);
        if !names.is_empty() {
            notes.push(format!(
                "{} kept as written, in a unit two checks don't settle: the file hasn't all three \
                 axes with gravity taken out",
                listed(&names)
            ));
        }
        return;
    };
    let mut no_grav = vec!["NoGravGx", "NoGravGy", "NoGravGz"];
    let gravity = median(
        (0..time.len())
            .map(|row| {
                ((x[row] - nx[row]).powi(2)
                    + (y[row] - ny[row]).powi(2)
                    + (z[row] - nz[row]).powi(2))
                .sqrt()
            })
            .collect(),
    );
    if let Some(total) = table.column("NoGravGTotal") {
        let off = (0..time.len())
            .map(|row| {
                (total[row] - (nx[row] * nx[row] + ny[row] * ny[row] + nz[row] * nz[row]).sqrt())
                    .abs()
            })
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max);
        if off <= TOTAL_TIE_G {
            no_grav.push("NoGravGTotal");
        } else {
            notes.push(format!(
                "`NoGravGTotal` kept as written, not the magnitude of the axes with gravity taken \
                 out: it lies up to {off:.3} from it, more than {TOTAL_TIE_G}"
            ));
        }
    }
    let tied = gravity.is_some_and(|g| (GRAVITY_TIE_G.0..=GRAVITY_TIE_G.1).contains(&g));
    let gravity_text = gravity.map_or("none".to_owned(), |g| format!("{g:.3}"));
    if outcome.passed && tied {
        notes.push(format!(
            "{} read in standard gravity, with the accelerometer's axes. The axes less them, the \
             gravity the phone took out, have a median magnitude of {gravity_text}. A median of \
             {} to {} is 1 g",
            listed(&no_grav),
            GRAVITY_TIE_G.0,
            GRAVITY_TIE_G.1
        ));
        set(table, &no_grav, reading);
    } else {
        notes.push(format!(
            "{} kept as written, in a unit two checks don't settle: {}; the axes less them have a \
             median magnitude of {gravity_text}",
            listed(&no_grav),
            if outcome.passed {
                "the accelerometer's axes are read in g"
            } else {
                "the accelerometer's axes aren't read in g"
            }
        ));
    }
}

/// Check 1 of an angle: every reading within `most` either side of 0.
fn within(values: &[f64], most: f64, written: &str) -> (bool, String) {
    let largest = values
        .iter()
        .filter(|v| v.is_finite())
        .fold(None, |m: Option<f64>, v| {
            Some(m.map_or(v.abs(), |m| m.max(v.abs())))
        });
    match largest {
        Some(largest) => (
            largest <= most + 1e-6,
            format!(
                "its largest reading from 0 is {largest:.4}, where radians stay within {written}"
            ),
        ),
        None => (false, "it holds no reading".to_owned()),
    }
}

/// The first row of the steepest climb, where the barometric altitude gives one: the rows before
/// it hold the pad.
fn climb_row(time: &[f64], coast: Option<&Coast>) -> Option<usize> {
    coast.map(|c| time.partition_point(|t| *t < c.climb.1))
}

/// Where the phone is still, the direction up, as a unit vector along the accelerometer's axes, on
/// each row whose magnitude lies within [`ATTITUDE_CHECK`]'s share of the quietest second's before
/// the steepest climb (or in the whole file, without a barometric climb); `None` on the others. At
/// rest an accelerometer reads the push that holds the phone up, so its axes point up whatever
/// their unit. `Err` gives why there are no such rows.
fn rest_directions(table: &Table, coast: Option<&Coast>) -> Result<Vec<Option<[f64; 3]>>, String> {
    let (Some(x), Some(y), Some(z)) = (table.column("Gx"), table.column("Gy"), table.column("Gz"))
    else {
        return Err("the file hasn't the accelerometer's three axes".to_owned());
    };
    let time = &table.time_s;
    let before = climb_row(time, coast).unwrap_or(time.len());
    let Some((_, rest)) = quietest(time, [x, y, z], before) else {
        return Err("the file has no quiet second".to_owned());
    };
    let share = ATTITUDE_CHECK.0;
    Ok((0..time.len())
        .map(|row| {
            let magnitude = (x[row] * x[row] + y[row] * y[row] + z[row] * z[row]).sqrt();
            ((magnitude - rest).abs() <= share * rest && magnitude > 0.0)
                .then(|| [x[row] / magnitude, y[row] / magnitude, z[row] / magnitude])
        })
        .collect())
}

/// The distance, rad, between two angles on the circle, 0 to π.
fn circular(a: f64, b: f64) -> f64 {
    let d = (a - b).rem_euclid(std::f64::consts::TAU);
    d.min(std::f64::consts::TAU - d)
}

/// The distance, rad, of an angle from the nearest half-turn, 0 to π/2: `atan |tan a|`.
fn from_half_turn(a: f64) -> f64 {
    let d = a.rem_euclid(std::f64::consts::PI);
    d.min(std::f64::consts::PI - d)
}

/// A direction check's verdict from the differences on its rows, read in radians and in degrees
/// (module docs): the median in radians at most [`ATTITUDE_CHECK`]'s and at most half that in
/// degrees, over at least its rows.
fn direction_verdict(
    radians: Vec<f64>,
    degrees: Vec<f64>,
    rows_are: &str,
    differs: &str,
) -> (bool, String) {
    let (_, _, most, rows) = ATTITUDE_CHECK;
    let count = radians.len();
    match (median(radians), median(degrees)) {
        (Some(r), Some(d)) if count >= rows => (
            r <= most && r <= d / 2.0,
            format!(
                "on {count} rows where {rows_are}, {differs} by a median {r:.3} read in radians \
                 and {d:.3} in degrees"
            ),
        ),
        _ => (
            false,
            format!("only {count} rows are where {rows_are}, fewer than {rows}"),
        ),
    }
}

/// Check 2 of the pitch: gravity's direction where the phone is still (module docs).
fn pitch_gravity(up: Result<&[Option<[f64; 3]>], &String>, pitch: &[f64]) -> (bool, String) {
    let up = match up {
        Ok(up) => up,
        Err(why) => return (false, why.clone()),
    };
    let least = ATTITUDE_CHECK.1;
    let mut radians = Vec::new();
    let mut degrees = Vec::new();
    for (p, u) in pitch.iter().zip(up) {
        if let Some(u) = u
            && p.abs() >= least
        {
            let gravity = u[1].abs();
            radians.push((p.sin().abs() - gravity).abs());
            degrees.push((p.to_radians().sin().abs() - gravity).abs());
        }
    }
    direction_verdict(
        radians,
        degrees,
        &format!("the accelerometer reads its rest and the pitch is at least {least} from level"),
        "|sin pitch| differs from gravity's share along y",
    )
}

/// Check 2 of the roll: gravity's direction about the phone's y axis where the phone is still
/// (module docs).
fn roll_gravity(up: Result<&[Option<[f64; 3]>], &String>, roll: &[f64]) -> (bool, String) {
    let up = match up {
        Ok(up) => up,
        Err(why) => return (false, why.clone()),
    };
    let least = ATTITUDE_CHECK.1;
    let mut radians = Vec::new();
    let mut degrees = Vec::new();
    for (r, u) in roll.iter().zip(up) {
        if let Some(u) = u
            && u[0].hypot(u[2]) >= least.sin()
            && from_half_turn(*r) >= least
        {
            let gravity = u[0].abs().atan2(u[2].abs());
            radians.push((from_half_turn(*r) - gravity).abs());
            degrees.push((from_half_turn(r.to_radians()) - gravity).abs());
        }
    }
    direction_verdict(
        radians,
        degrees,
        &format!(
            "the accelerometer reads its rest, its y axis is at least {least} from vertical and \
             the roll at least {least} from a half-turn"
        ),
        "the roll's angle from the nearest half-turn differs from gravity's about y, \
         atan(|Gx| / |Gz|),",
    )
}

/// Check 2 of the yaw: the magnetic field's direction where the phone is still (module docs).
fn yaw_magnetic(
    table: &Table,
    up: Result<&[Option<[f64; 3]>], &String>,
    yaw: &[f64],
) -> (bool, String) {
    let up = match up {
        Ok(up) => up,
        Err(why) => return (false, why.clone()),
    };
    let (Some(mx), Some(my), Some(mz)) = (
        table.column("MagnetometerX"),
        table.column("MagnetometerY"),
        table.column("MagnetometerZ"),
    ) else {
        return (
            false,
            "the file hasn't the magnetometer's three axes".to_owned(),
        );
    };
    let least = ATTITUDE_CHECK.1;
    let mut radians = Vec::new();
    let mut degrees = Vec::new();
    for (row, (a, u)) in yaw.iter().zip(up).enumerate() {
        let Some(u) = u else {
            continue;
        };
        if u[1].abs() > least.cos() || a.abs() < least {
            continue;
        }
        // East is the field across up, north up across east; the azimuth is the phone's y axis's
        // angle from north toward east.
        let field = [mx[row], my[row], mz[row]];
        let east = cross(field, *u);
        let size = (east[0] * east[0] + east[1] * east[1] + east[2] * east[2]).sqrt();
        if !(size.is_finite() && size > 0.0) {
            continue;
        }
        let east = [east[0] / size, east[1] / size, east[2] / size];
        let north = cross(*u, east);
        let azimuth = east[1].atan2(north[1]);
        radians.push(circular(*a, azimuth));
        degrees.push(circular(a.to_radians(), azimuth));
    }
    direction_verdict(
        radians,
        degrees,
        &format!(
            "the accelerometer reads its rest, its y axis is at least {least} from vertical and \
             the yaw at least {least} from 0"
        ),
        "the yaw differs from the azimuth of the phone's y axis from magnetic north, which the \
         magnetometer's and the accelerometer's directions give,",
    )
}

/// The cross product `a × b`.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// A rotation matrix from the attitude in radians: yaw about z, then pitch about x, then roll
/// about y.
fn attitude(yaw: f64, pitch: f64, roll: f64) -> [[f64; 3]; 3] {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let (sr, cr) = roll.sin_cos();
    let z = [[cy, -sy, 0.0], [sy, cy, 0.0], [0.0, 0.0, 1.0]];
    let x = [[1.0, 0.0, 0.0], [0.0, cp, -sp], [0.0, sp, cp]];
    let y = [[cr, 0.0, sr], [0.0, 1.0, 0.0], [-sr, 0.0, cr]];
    multiply(&multiply(&z, &x), &y)
}

fn multiply(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut product = [[0.0; 3]; 3];
    for (i, row) in product.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    product
}

/// The angle, rad, of the rotation from one attitude to another.
fn turn(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> f64 {
    let trace: f64 = (0..3)
        .map(|i| (0..3).map(|k| a[k][i] * b[k][i]).sum::<f64>())
        .sum();
    ((trace - 1.0) / 2.0).clamp(-1.0, 1.0).acos()
}

/// The gyroscope's columns (module docs, *The gyroscope*).
fn gyroscope(
    table: &Table,
    attitude_in_radians: bool,
    notes: &mut Vec<String>,
    set: &mut impl FnMut(&Table, &[&str], Reading),
) {
    let names = table.names_of(&[Kind::Gyro]);
    if names.is_empty() {
        return;
    }
    let axes: Vec<&[f64]> = ["GyroX", "GyroY", "GyroZ"]
        .iter()
        .filter_map(|name| table.column(name))
        .collect();
    let first = if axes.len() != 3 {
        (false, "the file hasn't all three of its axes".to_owned())
    } else if !attitude_in_radians {
        (
            false,
            "the attitude, `Yaw`, `Pitch` and `Roll`, isn't all read in radians".to_owned(),
        )
    } else {
        gyro_turns(table, &axes)
    };
    let second = full_scale(&names, table);
    let outcome = Outcome::both(first, second);
    if outcome.passed {
        notes.push(format!(
            "{} read in radians per second, from the attitude and the full scale: {}",
            listed(&names),
            outcome.evidence
        ));
        set(
            table,
            &names,
            Reading {
                quantity: Quantity::AngularRate,
                convert: Convert::Scale(180.0 / std::f64::consts::PI),
            },
        );
    } else {
        notes.push(format!(
            "{} kept as written, in a unit two checks don't settle: {}",
            listed(&names),
            outcome.evidence
        ));
    }
}

/// Check 1 of the gyroscope: the attitude's turn over spans of rows against the rate summed.
fn gyro_turns(table: &Table, axes: &[&[f64]]) -> (bool, String) {
    let (Some(yaw), Some(pitch), Some(roll)) = (
        table.column("Yaw"),
        table.column("Pitch"),
        table.column("Roll"),
    ) else {
        return (false, "the file hasn't the whole attitude".to_owned());
    };
    let time = &table.time_s;
    // The axes that stop at a full scale, and where.
    let full: Vec<Option<f64>> = axes
        .iter()
        .map(|axis| {
            let (most, stops) = plateau(axis);
            stops.then_some(most)
        })
        .collect();
    let mut ratios = Vec::new();
    let mut end = GYRO_SPAN_ROWS;
    while end < time.len() {
        let start = end - GYRO_SPAN_ROWS;
        let rows = start + 1..=end;
        let clipped = rows.clone().any(|row| {
            axes.iter().zip(&full).any(|(axis, most)| {
                most.is_some_and(|most| axis[row].abs() >= FULL_SCALE_SHARE * most)
            })
        });
        let summed: f64 = rows
            .map(|row| {
                let rate: f64 = axes
                    .iter()
                    .map(|axis| axis[row] * axis[row])
                    .sum::<f64>()
                    .sqrt();
                rate * (time[row] - time[row - 1])
            })
            .sum();
        let a = attitude(yaw[start], pitch[start], roll[start]);
        let b = attitude(yaw[end], pitch[end], roll[end]);
        let angle = turn(&a, &b);
        if !clipped
            && summed.is_finite()
            && angle.is_finite()
            && (GYRO_TURN_RAD.0..=GYRO_TURN_RAD.1).contains(&summed)
        {
            ratios.push(angle / summed);
        }
        end += GYRO_SPAN_ROWS;
    }
    let spans = ratios.len();
    match median(ratios) {
        Some(ratio) if spans >= GYRO_MIN_SPANS => (
            (GYRO_RATIO.0..=GYRO_RATIO.1).contains(&ratio),
            format!(
                "over {spans} spans of {GYRO_SPAN_ROWS} rows, the attitude turns a median {ratio:.3} \
                 times the rate summed, read in radians, where {} to {} reads rad/s",
                GYRO_RATIO.0, GYRO_RATIO.1
            ),
        ),
        _ => (
            false,
            format!(
                "only {} of {GYRO_SPAN_ROWS} rows turn by {} to {} rad off the full scale, \
                 fewer than {GYRO_MIN_SPANS}",
                counted(spans, "span", "spans"),
                GYRO_TURN_RAD.0,
                GYRO_TURN_RAD.1
            ),
        ),
    }
}

/// Check 2 of the gyroscope: an axis at a MEMS gyroscope's full scale, read in rad/s.
fn full_scale(names: &[&str], table: &Table) -> (bool, String) {
    let mut found: Option<(String, f64, f64)> = None;
    let mut largest_seen = Vec::new();
    for name in names {
        let Some(values) = table.column(name) else {
            continue;
        };
        let (most, stops) = plateau(values);
        let at = values
            .iter()
            .filter(|v| v.abs() >= most * (1.0 - PLATEAU.1))
            .count();
        let degrees = most.to_degrees();
        largest_seen.push(format!(
            "`{name}` {most:.4} on {}",
            counted(at, "row", "rows")
        ));
        if stops
            && let Some(scale) = FULL_SCALES_DPS
                .iter()
                .find(|scale| (degrees - **scale).abs() <= FULL_SCALE_MATCH * **scale)
        {
            found = Some(((*name).to_owned(), most, *scale));
            break;
        }
    }
    match found {
        Some((name, most, scale)) => (
            true,
            format!(
                "`{name}` stops at {most:.4} on at least {} rows, {:.1} °/s read in rad/s, a \
                 gyroscope's full scale of {scale} °/s",
                PLATEAU.0,
                most.to_degrees()
            ),
        ),
        None => (
            false,
            format!(
                "no axis stops at a gyroscope's full scale of 125 to 2,000 °/s read in rad/s. A stop \
                 is on {} rows or more, {} of them in a row. The largest readings are {}",
                PLATEAU.0,
                PLATEAU.2,
                largest_seen.join(", ")
            ),
        ),
    }
}

/// The largest reading from 0 of a column, and whether the column stops there: at least
/// [`PLATEAU`]'s rows within its share of it, enough of them in a row.
fn plateau(values: &[f64]) -> (f64, bool) {
    let most = values
        .iter()
        .filter(|v| v.is_finite())
        .fold(0.0_f64, |m, v| m.max(v.abs()));
    let at = |v: &f64| v.abs() >= most * (1.0 - PLATEAU.1);
    let count = values.iter().filter(|v| at(v)).count();
    let (mut run, mut longest) = (0, 0);
    for value in values {
        run = if at(value) { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    (most, count >= PLATEAU.0 && longest >= PLATEAU.2)
}

/// `1 row`, `2 rows`: a count with its noun.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// Check 1 of the heading: within 0 to 360 and past 2π.
fn heading_range(values: &[f64]) -> (bool, String) {
    let readings: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    let inside = readings.iter().all(|v| (0.0..=360.0).contains(v));
    let largest = readings.iter().fold(f64::NEG_INFINITY, |m, v| m.max(*v));
    (
        !readings.is_empty() && inside && largest > std::f64::consts::TAU,
        format!(
            "its readings, {}, {} within 0 to 360, the largest past the 2π radians reach",
            span(values),
            if inside { "lie" } else { "don't lie" }
        ),
    )
}

/// Check 2 of the heading: its wrap's period. A step between rows larger than
/// [`HEADING_WRAP_DEG`] that, less 360 in its direction, continues the turn of the steps on either
/// side: of their sign and within [`HEADING_WRAP_FACTOR`] of each's size.
fn heading_wraps(values: &[f64]) -> (bool, String) {
    let count = values
        .windows(4)
        .filter(|w| {
            let (before, step, after) = (w[1] - w[0], w[2] - w[1], w[3] - w[2]);
            let rest = step - 360.0_f64.copysign(step);
            step.abs() > HEADING_WRAP_DEG
                && [before, after].iter().all(|side| {
                    side.is_finite()
                        && *side != 0.0
                        && side.signum() == rest.signum()
                        && rest.abs() <= HEADING_WRAP_FACTOR * side.abs()
                        && side.abs() <= HEADING_WRAP_FACTOR * rest.abs()
                })
        })
        .count();
    (
        count > 0,
        format!(
            "{} north as a heading in degrees does: by more than {HEADING_WRAP_DEG}. Each, \
             less 360, turns as the steps on either side do, within a factor of \
             {HEADING_WRAP_FACTOR}",
            counted(
                count,
                "step between rows crosses",
                "steps between rows cross"
            )
        ),
    )
}

/// One check of the position: the column's largest reading from 0 past what radians reach,
/// `most` in degrees in radians, and within `most`, what degrees reach.
fn position_range(table: &Table, name: &str, most: f64) -> (bool, String) {
    let what = if name == "Latitude" {
        "the latitude"
    } else {
        "the longitude"
    };
    let Some(values) = table.column(name) else {
        return (false, format!("the file has no `{name}`"));
    };
    let radian_most = most.to_radians();
    let readings: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    let largest = readings.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
    (
        !readings.is_empty() && largest <= most && largest > radian_most,
        format!(
            "{what}'s largest reading from 0 is {largest:.1}, where radians reach \
             {radian_most:.4} and degrees {most}"
        ),
    )
}

/// Check 1 of the time stamp: its steps against the clock's, `Seconds`.
fn stamp_steps(time: &[f64], stamp: &[f64]) -> (bool, String) {
    let mut off = 0.0_f64;
    let mut steps = 0;
    for row in 1..time.len() {
        let step = (stamp[row] - stamp[row - 1]) / 1000.0;
        if step.is_finite() {
            off = off.max((step - (time[row] - time[row - 1])).abs());
            steps += 1;
        }
    }
    (
        steps > 0 && off <= STAMP_STEP_S,
        format!(
            "over {steps} steps between rows, its step over 1,000 lies at most {off:.4} s from \
             the clock's, `Seconds`"
        ),
    )
}

/// Check 2 of the time stamp: a date between 2000 and 2100 in ms.
fn stamp_date(stamp: &[f64]) -> (bool, String) {
    match stamp.iter().copied().find(|v| v.is_finite()) {
        Some(first) => (
            (STAMP_RANGE_MS.0..=STAMP_RANGE_MS.1).contains(&first),
            format!(
                "its first reading, read in ms since 1970, {} between 2000 and 2100",
                if (STAMP_RANGE_MS.0..=STAMP_RANGE_MS.1).contains(&first) {
                    "falls"
                } else {
                    "doesn't fall"
                }
            ),
        ),
        None => (false, "it holds no reading".to_owned()),
    }
}

/// The GPS altitude's evidence: its gain against the pressure altitude at the fresh fixes, and
/// why that isn't enough (module docs).
fn gps_evidence(table: &Table, pressure_altitude: Option<&[f64]>) -> String {
    const WHY: &str = "A GPS's fixes once a second can't show gravity's pull in the coast. A \
                       pad's height above sea level can't tell meters from feet within the \
                       weather's reach";
    let (Some(gps), Some(pressure_altitude)) = (table.column("AltitudeMSLGPS"), pressure_altitude)
    else {
        return format!("the file has no pressure in hPa to compare it with. {WHY}");
    };
    let latitude = table.column("Latitude");
    let longitude = table.column("Longitude");
    let changed = |column: Option<&[f64]>, row: usize| {
        column.is_some_and(|c| {
            c[row].is_finite() && (row == 0 || c[row].to_bits() != c[row - 1].to_bits())
        })
    };
    let fixes: Vec<(f64, f64)> = (0..gps.len())
        .filter(|&row| changed(Some(gps), row) || changed(latitude, row) || changed(longitude, row))
        .map(|row| (pressure_altitude[row], gps[row]))
        .filter(|(p, g)| p.is_finite() && g.is_finite())
        .collect();
    let (low, high) = fixes
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), (p, _)| {
            (l.min(*p), h.max(*p))
        });
    match slope(fixes.iter().copied()) {
        Some(gain) if fixes.len() >= MIN_FIXES && high - low >= MIN_CLIMB_M => format!(
            "at its {} fresh fixes it rises {gain:.3} times as fast as the pressure altitude, \
             near 1 in meters and 3.28 in feet. But that is one check. {WHY}",
            fixes.len()
        ),
        _ => format!(
            "its {} fresh fixes span too little of the pressure altitude to compare. {WHY}",
            fixes.len()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The invented newer-layout flight the integration tests read too: 5 rows a second, a pad,
    /// 1 s of boost at 15 g net, a drag-free coast to 1,176.8 m at 16 s, a descent at 15 m/s.
    const NEWER: &str = include_str!("../tests/samples/phone-launch-newer.csv");
    /// The invented older-layout file: 24 rows at 20 Hz on the pad, the phone turning.
    const OLDER: &str = include_str!("../tests/samples/phone-launch-older.csv");

    /// One row of a file being edited: its names and its cells as written.
    struct Row<'a> {
        names: &'a [&'a str],
        cells: Vec<String>,
    }

    impl Row<'_> {
        fn at(&self, name: &str) -> usize {
            self.names.iter().position(|n| *n == name).unwrap()
        }

        fn get(&self, name: &str) -> f64 {
            self.cells[self.at(name)].parse().unwrap()
        }

        fn set(&mut self, name: &str, value: f64) {
            let at = self.at(name);
            self.cells[at] = format!("{value}");
        }

        fn scale(&mut self, name: &str, factor: f64) {
            let value = self.get(name);
            self.set(name, value * factor);
        }
    }

    /// The text with each data row edited, its header kept.
    fn edit(text: &str, change: impl Fn(&mut Row<'_>)) -> String {
        let mut lines = text.lines();
        let header = lines.next().unwrap();
        let names: Vec<&str> = header.split(',').collect();
        let mut out = vec![header.to_owned()];
        for line in lines {
            let mut row = Row {
                names: &names,
                cells: line.split(',').map(str::to_owned).collect(),
            };
            change(&mut row);
            out.push(row.cells.join(","));
        }
        out.join("\n") + "\n"
    }

    /// The columns scaled by a factor on every row.
    fn scaled(text: &str, names: &[&str], factor: f64) -> String {
        edit(text, |row| {
            for name in names {
                row.scale(name, factor);
            }
        })
    }

    fn log_of(text: &str) -> FlightLog {
        read(text).unwrap()
    }

    /// The one note that opens as given.
    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
        matching[0]
    }

    fn quantity(log: &FlightLog, column: &str) -> Quantity {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap()
            .quantity
    }

    fn values<'a>(log: &'a FlightLog, column: &str) -> &'a [f64] {
        &log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap()
            .values
    }

    /// The syntax error's line and message.
    fn refusal(text: &str) -> (usize, String) {
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("expected a syntax error, got {other:?}"),
        }
    }

    /// The pressure, hPa, at a height above the invented pad, 100 m above sea level, by the
    /// altimeter formula.
    fn pressure_hpa(height_m: f64) -> f64 {
        1013.25 * (1.0 - (100.0 + height_m) / 44_330.8).powf(1.0 / 0.190_263)
    }

    #[test]
    fn recognizes_the_header_and_nothing_else() {
        assert!(recognizes(NEWER));
        assert!(recognizes(OLDER));
        assert!(recognizes("\n  Seconds,Timestamp,AltitudeMSLGPS,Record\n"));
        assert!(!recognizes("Timestamp,Seconds,AltitudeMSLGPS\n"));
        assert!(!recognizes("time (ms);deltaTime (ms)\n"));
        assert!(matches!(
            read("Time,Altitude\n0,0\n"),
            Err(LogError::NotThisFormat { .. })
        ));
        assert!(matches!(read(""), Err(LogError::NotThisFormat { .. })));
    }

    /// Each refusal, pinned by its line and words.
    #[test]
    fn malformed_files_are_refused_by_line() {
        let header = "Seconds,Timestamp,AltitudeMSLGPS,GTotal";
        assert_eq!(
            refusal(&format!("{header},\n0,1,2,3,4\n")),
            (1, "the header's column 5 has no name".to_owned())
        );
        assert_eq!(
            refusal(&format!("{header},GTotal\n")),
            (1, "the header names \"GTotal\" twice".to_owned())
        );
        assert_eq!(
            refusal(&format!("{header}\n0,1,2,3\n\n0.05,1,2\n")),
            (4, "the row has 3 cells where the header names 4".to_owned())
        );
        assert_eq!(
            refusal(&format!("{header}\n0,1,2,3,4\n")),
            (
                2,
                "the row has more than 4 cells where the header names 4".to_owned()
            )
        );
        assert_eq!(
            refusal(&format!("{header}\n0,1,2,x\n")),
            (2, "\"x\" under \"GTotal\" isn't a number".to_owned())
        );
        assert_eq!(
            refusal(&format!("{header}\n,1,2,3\n")),
            (2, "the row has no time".to_owned())
        );
        assert_eq!(
            refusal(&format!("{header}\n0.1,1,2,3\n0.1,1,2,3\n")),
            (
                3,
                "the time 0.1 s doesn't come after the one above it, 0.1 s".to_owned()
            )
        );
        assert_eq!(
            read(&format!("{header}\n\n")),
            Err(LogError::NoData { format: FORMAT })
        );
    }

    /// The infinite-value rule (ADR-258 §3, #518): a finite cell its conversion overflows is
    /// refused on its line.
    #[test]
    fn a_value_its_conversion_overflows_is_refused_by_line() {
        let text = edit(NEWER, |row| {
            if row.get("Seconds") == 3.0 {
                row.set("hPa", 1.7e308);
            }
        });
        // The header is line 1 and the rows start at -2.0 s, 5 a second: 3.0 s is line 27.
        assert_eq!(
            refusal(&text),
            (
                27,
                "the column hPa holds a value too large for its unit's conversion to SI".to_owned()
            )
        );
        // As written, the same cell under a column kept as written is a value like any other.
        let kept = edit(NEWER, |row| {
            if row.get("Seconds") == 3.0 {
                row.set("UsbV1", 1.7e308);
            }
        });
        assert_eq!(values(&log_of(&kept), "UsbV1")[25], 1.7e308);
    }

    /// The newer flight: every unit two checks settle is read, the primary track filled.
    #[test]
    fn the_newer_flight_reads_its_units() {
        let log = log_of(NEWER);
        assert_eq!(log.format, LogFormat::PhoneLaunchCsv);
        assert_eq!(log.time_s.len(), 111);
        for (column, expected) in [
            ("Timestamp", Quantity::TimeOfDay),
            ("AltitudeMSLGPS", Quantity::Unrecognized),
            ("AltitudeMSLMaxGPS", Quantity::Unrecognized),
            ("Record", Quantity::Number),
            ("Latitude", Quantity::Latitude),
            ("Longitude", Quantity::Longitude),
            ("GTotal", Quantity::Acceleration),
            ("Gx", Quantity::Acceleration),
            ("GMax", Quantity::Unrecognized),
            ("MagnetometerX", Quantity::Unrecognized),
            ("GyroY", Quantity::AngularRate),
            ("Heading", Quantity::Angle),
            ("Yaw", Quantity::Angle),
            ("Pitch", Quantity::Angle),
            ("Roll", Quantity::Angle),
            ("AltitudeRelativeBarometer", Quantity::AltitudeAboveGround),
            ("Temperature", Quantity::Unrecognized),
            ("NoGravGz", Quantity::Acceleration),
            ("NoGravGTotal", Quantity::Acceleration),
            ("VelocityBarometer", Quantity::Speed),
            ("VelocityBarometerRaw", Quantity::Speed),
            ("VelocityBarometerMin", Quantity::Unrecognized),
            ("hPa", Quantity::Pressure),
            ("Satellites", Quantity::Number),
            ("UsbUR8", Quantity::Unrecognized),
        ] {
            assert_eq!(quantity(&log, column), expected, "{column}");
        }
        // The highest altitude, 1,176.8 m at 16 s, row 90; the speed there 0.
        let top = log
            .altitude_m
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        assert!((top - 1_176.8).abs() < 0.1, "{top}");
        assert_eq!(log.vertical_speed_m_s.as_ref().unwrap()[90], 0.0);
        // Conversions: 1 g, a pitch of 1.2 rad, 1,000 °/s, the pad's pressure in Pa, 08:00 UTC.
        assert!((values(&log, "GTotal")[0] - G0_M_S2).abs() < 1e-9);
        assert!((values(&log, "Pitch")[0] - 1.2_f64.to_degrees()).abs() < 1e-9);
        assert!((values(&log, "GyroY")[11] - 1_000.0).abs() < 1e-3);
        assert!((values(&log, "hPa")[0] - pressure_hpa(0.0) * 100.0).abs() < 0.01);
        assert_eq!(values(&log, "Timestamp")[0], 28_800.0);
        assert_eq!(values(&log, "Timestamp")[5], 28_801.0);
        assert!(note(&log, "`AltitudeMSLGPS`").contains("rises 1.000 times"));
    }

    /// The barometric altitude in feet fails the pressure check and is kept as written; so is
    /// the speed checked against it, and the primary track is empty.
    #[test]
    fn a_barometric_altitude_in_feet_is_kept_as_written() {
        let log = log_of(&scaled(NEWER, &["AltitudeRelativeBarometer"], 1.0 / FOOT_M));
        assert_eq!(
            quantity(&log, "AltitudeRelativeBarometer"),
            Quantity::Unrecognized
        );
        let kept = note(&log, "`AltitudeRelativeBarometer` kept as written");
        assert!(kept.contains("at most 2684."), "{kept}");
        assert!(log.altitude_m.is_empty() && log.time_s.is_empty());
        assert_eq!(quantity(&log, "VelocityBarometer"), Quantity::Unrecognized);
        assert_eq!(quantity(&log, "Gx"), Quantity::Unrecognized);
        assert!(note(&log, "the primary track is empty").contains("withhold"));
    }

    /// An altitude that the pressure bears out but whose coast slows by 0.3 g, less than gravity
    /// alone, fails the second check and is kept as written.
    #[test]
    fn a_coast_slower_than_gravity_keeps_the_altitude_as_written() {
        let text = edit(NEWER, |row| {
            let height = 0.3 * row.get("AltitudeRelativeBarometer");
            row.set("AltitudeRelativeBarometer", height);
            row.set("hPa", pressure_hpa(height));
        });
        let log = log_of(&text);
        let kept = note(&log, "`AltitudeRelativeBarometer` kept as written");
        assert!(
            kept.contains("(1) above the first row it lies at most 0.0"),
            "{kept}"
        );
        assert!(kept.contains("0.30 g in meters"), "{kept}");
        assert_eq!(
            quantity(&log, "AltitudeRelativeBarometer"),
            Quantity::Unrecognized
        );
    }

    /// A pressure the pad refutes as hPa, kilopascals, is kept as written, and nothing that needs
    /// it is read.
    #[test]
    fn a_pressure_in_kilopascals_is_kept_as_written() {
        let log = log_of(&scaled(NEWER, &["hPa"], 0.1));
        assert!(
            note(&log, "`hPa` kept as written").contains("100.13, outside the 500 to 1100 hPa")
        );
        assert_eq!(quantity(&log, "hPa"), Quantity::Unrecognized);
        assert!(
            note(&log, "`AltitudeRelativeBarometer` kept as written")
                .contains("(1) the file has no pressure read in hPa")
        );
        assert!(note(&log, "`AltitudeMSLGPS`").contains("no pressure in hPa"));
    }

    /// A speed in feet per second fails both its checks; a speed flat through the coast fails the
    /// second alone.
    #[test]
    fn a_speed_is_read_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["VelocityBarometer"], 1.0 / FOOT_M));
        assert_eq!(quantity(&log, "VelocityBarometer"), Quantity::Unrecognized);
        assert!(log.vertical_speed_m_s.is_none());
        let kept = note(&log, "`VelocityBarometer` kept as written");
        assert!(kept.contains("gain of 3.28"), "{kept}");
        let flat = edit(NEWER, |row| {
            let t = row.get("Seconds");
            if (7.0..=14.0).contains(&t) {
                row.set("VelocityBarometer", 40.0);
            }
        });
        let log = log_of(&flat);
        let kept = note(&log, "`VelocityBarometer` kept as written");
        assert!(kept.contains("where 0.8 to 1.25 reads m/s. (2)"), "{kept}");
        assert!(kept.contains("slows by 0.00"), "{kept}");
        assert_eq!(quantity(&log, "VelocityBarometerRaw"), Quantity::Speed);
    }

    /// Accelerations in m/s² fail the pad's 1 g; a boost three times the barometer's fails the
    /// second check alone.
    #[test]
    fn accelerations_are_read_in_g_only_where_both_checks_hold() {
        let all = [
            "GTotal",
            "Gx",
            "Gy",
            "Gz",
            "NoGravGTotal",
            "NoGravGx",
            "NoGravGy",
            "NoGravGz",
        ];
        let log = log_of(&scaled(NEWER, &all, G0_M_S2));
        let kept = note(&log, "`Gx`, `Gy`, `Gz` and `GTotal` kept as written");
        assert!(
            kept.contains("reads 9.705 in the axes' magnitude"),
            "{kept}"
        );
        assert_eq!(quantity(&log, "Gx"), Quantity::Unrecognized);
        assert_eq!(quantity(&log, "NoGravGx"), Quantity::Unrecognized);
        let boost = edit(NEWER, |row| {
            let t = row.get("Seconds");
            if t > 0.0 && t <= 1.0 {
                for name in ["GTotal", "Gx", "Gy", "Gz"] {
                    row.scale(name, 3.0);
                }
            }
        });
        let log = log_of(&boost);
        let kept = note(&log, "`Gx`, `Gy`, `Gz` and `GTotal` kept as written");
        // The phone turns on the invented pad, so the quietest second's mean reads a little
        // under 1.
        assert!(kept.contains("reads 0.990"), "{kept}");
        assert!(
            kept.contains("3.21 times the barometric climb rate"),
            "{kept}"
        );
    }

    /// `GTotal` that isn't the axes' magnitude is kept apart from them.
    #[test]
    fn a_total_that_isnt_the_axes_magnitude_is_kept() {
        let log = log_of(&scaled(NEWER, &["GTotal"], 2.0));
        assert!(note(&log, "`GTotal` kept as written").contains("up to 16.000"));
        assert_eq!(quantity(&log, "GTotal"), Quantity::Unrecognized);
        assert_eq!(quantity(&log, "Gx"), Quantity::Acceleration);
    }

    /// The yaw turned by a whole number of turns and half a radian, so that it stays within ±π.
    fn yaw_turned(text: &str, by: f64) -> String {
        edit(text, |row| {
            let yaw = row.get("Yaw");
            row.set(
                "Yaw",
                (yaw + by + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU)
                    - std::f64::consts::PI,
            );
        })
    }

    /// The yaw in degrees fails its range; a yaw half a radian off the magnetic field's azimuth
    /// fails the second check alone, and the gyroscope then can't be read either. The field's
    /// direction, not its size, is checked: a magnetometer in another unit changes nothing.
    #[test]
    fn a_yaw_is_read_in_radians_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["Yaw"], 180.0 / std::f64::consts::PI));
        assert!(
            note(&log, "`Yaw` kept as written").contains("its largest reading from 0 is 177.98")
        );
        assert_eq!(quantity(&log, "Yaw"), Quantity::Unrecognized);
        assert!(
            note(&log, "`GyroX`, `GyroY` and `GyroZ` kept as written")
                .contains("isn't all read in radians")
        );
        let log = log_of(&yaw_turned(NEWER, 0.5));
        let kept = note(&log, "`Yaw` kept as written");
        assert!(
            kept.contains("(1) its largest reading from 0 is 3.1"),
            "{kept}"
        );
        assert!(kept.contains("by a median 0.500 read in radians"), "{kept}");
        assert_eq!(quantity(&log, "Yaw"), Quantity::Unrecognized);
        let names = ["MagnetometerX", "MagnetometerY", "MagnetometerZ"];
        let log = log_of(&scaled(NEWER, &names, 1_000.0));
        let read = note(&log, "`Yaw` read in radians");
        assert!(read.contains("by a median 0.000 read in radians"), "{read}");
        // Without a magnetometer the second check can't run, and fails.
        let log = log_of(&scaled(NEWER, &names, 0.0));
        assert!(
            note(&log, "`Yaw` kept as written").contains("(2) only 0 rows are where"),
            "{:#?}",
            log.notes
        );
    }

    /// The pitch in degrees fails its range; a pitch gravity denies fails the second check alone.
    #[test]
    fn a_pitch_is_read_in_radians_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["Pitch"], 180.0 / std::f64::consts::PI));
        assert_eq!(quantity(&log, "Pitch"), Quantity::Unrecognized);
        let log = log_of(&edit(NEWER, |row| row.set("Pitch", 0.5)));
        let kept = note(&log, "`Pitch` kept as written");
        assert!(
            kept.contains("(1) its largest reading from 0 is 0.5000"),
            "{kept}"
        );
        assert!(kept.contains("by a median 0.453 read in radians"), "{kept}");
    }

    /// The roll in degrees fails its range; a roll half a radian off gravity's direction fails the
    /// second check alone. Gravity's direction, not its size, is checked: an accelerometer in m/s²
    /// changes nothing.
    #[test]
    fn a_roll_is_read_in_radians_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["Roll"], 180.0 / std::f64::consts::PI));
        let kept = note(&log, "`Roll` kept as written");
        assert!(
            kept.contains("its largest reading from 0 is 178.76"),
            "{kept}"
        );
        assert_eq!(quantity(&log, "Roll"), Quantity::Unrecognized);
        let log = log_of(&edit(NEWER, |row| {
            let roll = row.get("Roll");
            row.set("Roll", roll * 0.5);
        }));
        let kept = note(&log, "`Roll` kept as written");
        assert!(
            kept.contains("(1) its largest reading from 0 is 1.5"),
            "{kept}"
        );
        assert!(kept.contains("differs from gravity's about y"), "{kept}");
        assert_eq!(quantity(&log, "Roll"), Quantity::Unrecognized);
        let axes = ["Gx", "Gy", "Gz", "GTotal"];
        let log = log_of(&scaled(NEWER, &axes, G0_M_S2));
        let read = note(&log, "`Roll` read in radians");
        assert!(read.contains("by a median 0.000 read in radians"), "{read}");
    }

    /// The gyroscope in degrees per second fails the attitude's turn; one that stops at no
    /// gyroscope's full scale, 15 rad/s or 859 °/s, fails the second check alone.
    #[test]
    fn a_gyroscope_is_read_in_rad_per_s_only_where_both_checks_hold() {
        let names = ["GyroX", "GyroY", "GyroZ"];
        let log = log_of(&scaled(NEWER, &names, 180.0 / std::f64::consts::PI));
        let kept = note(&log, "`GyroX`, `GyroY` and `GyroZ` kept as written");
        // Summed in degrees, no span's rate falls in the turns the attitude can measure.
        assert!(kept.contains("(1) only 0 spans of 5 rows"), "{kept}");
        let log = log_of(&edit(NEWER, |row| {
            if row.get("GyroY") > 10.0 {
                row.set("GyroY", 15.0);
            }
        }));
        let kept = note(&log, "`GyroX`, `GyroY` and `GyroZ` kept as written");
        assert!(
            kept.contains("where 0.7 to 1.3 reads rad/s. (2) no axis stops"),
            "{kept}"
        );
        assert!(kept.contains("`GyroY` 15.0000 on 5 rows"), "{kept}");
        assert_eq!(quantity(&log, "GyroY"), Quantity::Unrecognized);
    }

    /// The heading in radians or gradians fails its range; one that never crosses north fails the
    /// second check alone.
    #[test]
    fn a_heading_is_read_in_degrees_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["Heading"], std::f64::consts::PI / 180.0));
        assert_eq!(quantity(&log, "Heading"), Quantity::Unrecognized);
        let log = log_of(&scaled(NEWER, &["Heading"], 400.0 / 360.0));
        assert!(note(&log, "`Heading` kept as written").contains("don't lie within 0 to 360"));
        let log = log_of(&edit(NEWER, |row| {
            let t = row.get("Seconds");
            row.set("Heading", 180.0 + t);
        }));
        assert!(note(&log, "`Heading` kept as written").contains("(2) 0 steps between rows"));
    }

    /// The wrap's period, by hand: a heading turning 10 a row that crosses north as degrees do,
    /// from 350 to 0, passes; one that crosses as gradians do, from 390 to 0, steps by 30 less
    /// than 360 against the turn and fails, as does a jump against the turn on either side.
    #[test]
    fn the_headings_wrap_has_the_period_of_degrees() {
        assert!(heading_wraps(&[330.0, 340.0, 350.0, 0.0, 10.0, 20.0]).0);
        assert!(heading_wraps(&[30.0, 20.0, 10.0, 0.0, 350.0, 340.0]).0);
        assert!(!heading_wraps(&[370.0, 380.0, 390.0, 0.0, 10.0, 20.0]).0);
        assert!(!heading_wraps(&[30.0, 20.0, 10.0, 0.0, 390.0, 380.0]).0);
        // The step, less 360, must turn as the rows on either side do, and as fast within 2.
        assert!(!heading_wraps(&[360.0, 350.0, 340.0, 0.0, 10.0, 20.0]).0);
        assert!(!heading_wraps(&[349.0, 350.0, 351.0, 9.0, 10.0, 11.0]).0);
        assert!(!heading_wraps(&[340.0, 350.0, 350.0, 0.0, 10.0, 20.0]).0);
    }

    /// A position in radians fails both checks; a latitude near the equator fails the first alone
    /// and a longitude near the prime meridian the second, and either keeps both, never guessed;
    /// so does a latitude in NMEA's degrees and minutes. A pair in degrees and minutes within 1°
    /// of (0°, 0°), open sea, passes both: the limit the module docs state.
    #[test]
    fn a_position_is_read_in_degrees_only_where_both_checks_hold() {
        let both = ["Latitude", "Longitude"];
        let log = log_of(&scaled(NEWER, &both, std::f64::consts::PI / 180.0));
        let kept = note(&log, "`Latitude` and `Longitude` kept as written");
        assert!(
            kept.contains("(1) the latitude's largest reading from 0 is 0.7"),
            "{kept}"
        );
        assert_eq!(quantity(&log, "Latitude"), Quantity::Unrecognized);
        assert_eq!(quantity(&log, "Longitude"), Quantity::Unrecognized);
        for (name, value) in [
            ("Latitude", 1.25),
            ("Longitude", -3.0),
            ("Latitude", 4006.0),
        ] {
            let log = log_of(&edit(NEWER, |row| row.set(name, value)));
            assert!(
                note(&log, "`Latitude` and `Longitude` kept as written")
                    .contains(&format!("largest reading from 0 is {:.1}", value.abs()))
            );
            assert_eq!(quantity(&log, "Latitude"), Quantity::Unrecognized, "{name}");
            assert_eq!(
                quantity(&log, "Longitude"),
                Quantity::Unrecognized,
                "{name}"
            );
        }
        let log = log_of(&edit(NEWER, |row| {
            row.set("Latitude", 30.0);
            row.set("Longitude", 112.0);
        }));
        assert_eq!(quantity(&log, "Latitude"), Quantity::Latitude);
        assert_eq!(quantity(&log, "Longitude"), Quantity::Longitude);
        let mut lines: Vec<String> = NEWER.lines().map(str::to_owned).collect();
        for line in &mut lines {
            let mut cells: Vec<&str> = line.split(',').collect();
            cells.remove(6);
            *line = cells.join(",");
        }
        let log = log_of(&(lines.join("\n") + "\n"));
        let kept = note(&log, "`Latitude` kept as written");
        assert!(kept.contains("(2) the file has no `Longitude`"), "{kept}");
    }

    /// A time stamp in seconds fails both checks; one whose steps stray from the clock's fails
    /// the first alone.
    #[test]
    fn a_time_stamp_is_read_only_where_both_checks_hold() {
        let log = log_of(&scaled(NEWER, &["Timestamp"], 0.001));
        let kept = note(&log, "`Timestamp` kept as written");
        assert!(
            kept.contains("doesn't fall between 2000 and 2100"),
            "{kept}"
        );
        assert_eq!(quantity(&log, "Timestamp"), Quantity::Unrecognized);
        let log = log_of(&edit(NEWER, |row| {
            if row.get("Seconds") == 4.0 {
                row.scale("Timestamp", 1.0);
                let stamp = row.get("Timestamp");
                row.set("Timestamp", stamp + 20.0);
            }
        }));
        let kept = note(&log, "`Timestamp` kept as written");
        assert!(kept.contains("at most 0.0200 s from"), "{kept}");
        assert!(kept.contains("(2) its first reading, read in ms since 1970, falls"));
    }

    /// The GPS altitude is kept as written whatever its gain: in meters the one check passes, in
    /// feet it doesn't, and one check never settles a unit.
    #[test]
    fn the_gps_altitude_is_kept_as_written_either_way() {
        let log = log_of(NEWER);
        assert_eq!(quantity(&log, "AltitudeMSLGPS"), Quantity::Unrecognized);
        assert!(note(&log, "`AltitudeMSLGPS` and `AltitudeMSLMaxGPS` kept as written, in a unit no second check settles: at its 23 fresh fixes it rises 1.000").contains("that is one check"));
        let feet = scaled(
            NEWER,
            &["AltitudeMSLGPS", "AltitudeMSLMaxGPS"],
            1.0 / FOOT_M,
        );
        let log = log_of(&feet);
        assert_eq!(quantity(&log, "AltitudeMSLGPS"), Quantity::Unrecognized);
        assert!(note(&log, "`AltitudeMSLGPS`").contains("rises 3.281 times"));
        assert_eq!(values(&log, "AltitudeMSLGPS")[0], 100.0 / FOOT_M);
    }

    /// The older file: no barometer, so the accelerations' second check can't run and the primary
    /// track is empty; the attitude, heading, position and time stamp are read.
    #[test]
    fn the_older_layout_keeps_what_it_cant_check() {
        let log = log_of(OLDER);
        assert!(log.altitude_m.is_empty() && log.time_s.is_empty());
        assert!(
            note(
                &log,
                "`AltitudeRelativeBarometer` kept as written, holding no reading"
            )
            .contains("0 on every row")
        );
        let kept = note(&log, "`Gx`, `Gy`, `Gz` and `GTotal` kept as written");
        assert!(
            kept.contains("(1) the quietest second, to 1.15 s, reads 0.999"),
            "{kept}"
        );
        assert!(
            kept.contains("(2) the file has no barometric climb"),
            "{kept}"
        );
        for (column, expected) in [
            ("Yaw", Quantity::Angle),
            ("Pitch", Quantity::Angle),
            ("Roll", Quantity::Angle),
            ("Heading", Quantity::Angle),
            ("GyroX", Quantity::Unrecognized),
            ("Timestamp", Quantity::TimeOfDay),
            ("AltitudeRelativeSerial", Quantity::Unrecognized),
            ("Battery", Quantity::Unrecognized),
        ] {
            assert_eq!(quantity(&log, column), expected, "{column}");
        }
        assert_eq!(log.channels.len(), 34);
    }

    /// A column the reader doesn't know is kept as written with a note; an empty cell is a gap.
    #[test]
    fn unknown_columns_and_empty_cells() {
        let mut lines: Vec<String> = OLDER.lines().map(str::to_owned).collect();
        lines[0].push_str(",Mystery");
        for line in &mut lines[1..] {
            line.push_str(",7");
        }
        lines[3] = lines[3].replacen(",0.85,", ",,", 1);
        let log = log_of(&(lines.join("\n") + "\n"));
        assert_eq!(quantity(&log, "Mystery"), Quantity::Unrecognized);
        assert!(note(&log, "`Mystery` kept as written").contains("doesn't know the column"));
        assert!(values(&log, "Battery")[2].is_nan());
    }

    /// The step, by hand: 100 / 16,384 Pa at 1 / (1.225 × 9.806 65) m a pascal, 0.000 508 07 m.
    #[test]
    fn hundredths_drop_the_sign_of_a_rounded_zero() {
        assert_eq!(hundredths(-0.001), "0.00");
        assert_eq!(hundredths(-0.01), "-0.01");
        assert_eq!(hundredths(9.806), "9.81");
    }

    #[test]
    fn the_altitude_step_is_the_pressures() {
        assert!((ALTITUDE_STEP_M - 0.000_508_070).abs() < 1e-9);
        assert!(
            (LogFormat::PhoneLaunchCsv.altitude_resolution_m() - ALTITUDE_STEP_M).abs() < 1e-15
        );
    }

    /// The fit helpers against hand values.
    #[test]
    fn the_fits_recover_known_lines_and_parabolas() {
        let line: Vec<(f64, f64)> = (0..10).map(|i| (i as f64, 3.0 + 2.0 * i as f64)).collect();
        assert!((slope(line.iter().copied()).unwrap() - 2.0).abs() < 1e-12);
        let parabola: Vec<(f64, f64)> = (0..10)
            .map(|i| {
                let t = 100.0 + 0.2 * i as f64;
                (t, 5.0 - 3.0 * t - 4.9 * t * t)
            })
            .collect();
        assert!((curvature(&parabola).unwrap() + 9.8).abs() < 1e-6);
        assert_eq!(median(vec![3.0, f64::NAN, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(vec![4.0, 1.0, 3.0, 2.0]), Some(2.5));
        let quarter = attitude(0.0, 0.0, 0.0);
        let turned = attitude(0.5, 0.0, 0.0);
        assert!((turn(&quarter, &turned) - 0.5).abs() < 1e-12);
        assert_eq!(brief(37.200_000_762_939_45), "37.2");
        assert_eq!(brief(-0.000_1), "0");
    }
}
