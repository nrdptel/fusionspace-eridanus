//! The Featherweight Blue Raven's flight logs, as its phone app exports them to comma-separated
//! files.
//!
//! The Blue Raven records three sets of data for each flight: a summary, a low-rate set "recorded
//! at 50 Hz" and a high-rate set "recorded at 500 Hz" (Featherweight, *Blue Raven User's Guide*,
//! updated May 12 2026, p18); the Featherweight UI app downloads them over Bluetooth and exports
//! each as a file (p19). This reader reads the two data files, each on its own: a header line
//! naming the columns, then one row per sample, every column on the row's time:
//!
//! ```text
//! Year,Month,Day,Time,Flight_Time_(s),Sync,Gyro_X,Gyro_Y,Gyro_Z,Accel_X,Accel_Y,Accel_Z,Quat_1,…
//! 2026,5,23,10:15:00.120,-2.000,0,0.10,-0.20,0.05,0.01,-0.02,1.00,0.7071,…
//! ```
//!
//! A file is the app's when its first line that isn't blank opens with the six columns
//! `Year,Month,Day,Time,Flight_Time_(s),Sync`, as both files do. The high-rate file has 18 columns,
//! the low-rate file 101; columns are mapped by name, not by place, so the low-rate file's
//! variants read alike (in some, `Axial Accel (Gs),Lateral Accel (Gs)` stand where others have
//! `3rd_Volts,4th_Volts`, and one names the fifth FER column `Apo_FER_H  ex`, with two spaces).
//! A name is matched with its spaces taken out and without regard to case, and kept as written.
//! The low-rate file repeats some names (`Reserved`, and the four output channels' condition
//! flags such as `LT_AGL1`): each is its own channel, in the file's order.
//!
//! **Channels.** Every column but the clock is a channel, read into the record's SI unit
//! ([`Quantity::unit`]):
//!
//! | Column | Quantity | From |
//! |---|---|---|
//! | `Flight_Time_(s)` | the clock, s, zero at liftoff | the header |
//! | `Time` | [`Quantity::TimeOfDay`], s after the logger's midnight | written `HH:MM:SS.fff`; no zone is stated |
//! | `Year`, `Month`, `Day`, `Sync` | [`Quantity::Number`] | the date the logger's clock gives; `Sync` is "a millisecond counter … recorded in both the low-rate and high-rate data so that the two data sets can be aligned", which "rolls over every 250 msec" (p23) |
//! | `Temperature_(F)` | [`Quantity::Temperature`] | the header's °F |
//! | `Baro_Press_(atm)` | [`Quantity::Pressure`] | the header's atm, × 101,325 Pa |
//! | `Baro_Altitude_AGL_(feet)`, `Baro_Altitude_ASL_(feet)` | [`Quantity::AltitudeAboveGround`], [`Quantity::AltitudeMsl`] | the header's feet |
//! | `Batt_Volts`, `Apo_Volts`, `Main_Volts`, `3rd_Volts`, `4th_Volts`, `Aux_Volts` | [`Quantity::Voltage`] | the header's volts |
//! | `Axial Accel (Gs)`, `Lateral Accel (Gs)` | [`Quantity::Acceleration`] | the header's g, × 9.806 65 m/s² (standard gravity) |
//! | `Velocity_Up`, `Velocity_DR`, `Velocity_CR` | [`Quantity::Speed`] | ft/s: the manual and the summary (below) |
//! | `Inertial_Altitude` | [`Quantity::InertialAltitude`] | ft: the manual and the summary (below) |
//! | `Inertial_DR_Position`, `Inertial_CR_position` | [`Quantity::Distance`] | ft: the manual (below) |
//! | `Tilt_Angle_(deg)`, `Future_Angle_(deg)`, `Roll_Angle_(deg)` | [`Quantity::Angle`] | the header's degrees |
//! | `Gyro_X`, `Gyro_Y`, `Gyro_Z` | [`Quantity::AngularRate`] | °/s: the manual and the summary (below) |
//! | `Accel_X`, `Accel_Y`, `Accel_Z` | [`Quantity::Acceleration`] | g, × 9.806 65 m/s²: the manual and the pad (below) |
//! | `Quat_1` … `Quat_4` | [`Quantity::Number`] | the attitude quaternion, "the rotation axis first and the magnitude term fourth" (p24), of unit norm |
//! | `Current` | [`Quantity::Unrecognized`], as written | no source states its unit (below) |
//! | `Rocket_FER_Hex`, `Apo_FER_Hex` and the other `…_FER_Hex` | [`Quantity::Number`]: the register's bits | hexadecimal in the file (below) |
//! | `Liftoff`, `Apogee` and the other flags | [`Quantity::Number`], 0 or 1 | below |
//! | `Reserved`, `Reserved_1` … | [`Quantity::Unrecognized`], as written | the manual describes no reserved column |
//!
//! A column the reader doesn't know is kept as written, [`Quantity::Unrecognized`], with a note.
//!
//! **Units no header states.** Thirteen columns carry no unit in their names. The manual states
//! each one's unit for the logger's USB download, which carries the same figures (p23–24):
//! velocities in ft/sec, the inertial altitude and the down-range and cross-range positions in
//! feet, gyro rates in deg/s × 100, accelerations in G × 100, current in mA. The app writes the
//! gyro rates and accelerations unscaled by the hundred, so its file's units are decided from what
//! the app's own files show. The counts below are, as of 2026-10-12, over the 80 Blue Raven files
//! among the project's sample logs (45 low-rate, 35 high-rate), most of them in the private
//! collection of flights this project checks against:
//!
//! - *Velocities and the inertial altitude, in feet.* Beside a flight, the app's summary file
//!   states "Max velocity" in "feet/sec" and "Inertial navigation max alt" in "feet". Of the 32
//!   summaries that sit beside a low-rate file, 25 state an inertial maximum within 0.5 % of a
//!   low-rate file's highest `Inertial_Altitude`, and in those the highest `Velocity_Up` is 0.69
//!   to 1.0 times the summary's maximum velocity, which counts every direction; in meters per
//!   second it would be 0.3 times. Debrief's public pair agrees: its file's highest speed and
//!   inertial altitude are within 0.5 % of its summary's, in feet. The barometer agrees too: integrated over
//!   the climb to the barometer's highest sample, `Velocity_Up` gives 0.90 to 1.20 times the
//!   rise of `Baro_Altitude_AGL_(feet)` in 43 of the 45 low-rate files. The down-range and
//!   cross-range positions take the altitude's unit, as the manual gives them.
//! - *Gyro rates in degrees per second.* The summary states "Pad gyro rates (deg/sec)", and the
//!   manual gives the gyros' range as ±2000 deg/sec (p23): in 21 of the 35 high-rate files the
//!   largest rate sits at a common ceiling of 2,292 to 2,294, just past that range, as a
//!   saturated gyro reads.
//! - *Accelerations in g.* On the pad, the median magnitude of the three axes is 0.990 to 1.014 in
//!   34 of the 35 high-rate files, as a logger at rest reads 1 g.
//! - *Current, kept as written.* No document states the app's unit. The column reads 0 in every
//!   row of 6 high-rate files, and in the other 29 but for brief runs, peaking at 0.05 to 13, 13
//!   of them between 2 and 3.5. Amperes would fit the manual's word that the Blue Raven
//!   "internally limits the current to around 4 Amps" (p5), and the USB download's milliamperes
//!   would put a firing output's current at a few thousandths of an ampere; but the files show a
//!   size, not a unit, so the column stays [`Quantity::Unrecognized`] until a source states it,
//!   with a note giving this file's figures.
//!
//! Each file's record carries one note per kind decided this way, the decision and this file's
//! own evidence, such as `` `Gyro_X`, `Gyro_Y` and `Gyro_Z` read in degrees per second, from the
//! manual and the summary: … ``.
//!
//! **Flags, event registers and events.** The manual's five flight event registers hold "one bit
//! for each event", written in hexadecimal (p24): the rocket's (`Rocket_FER_Hex`) and each output
//! channel's (`Apo_FER_Hex`, `Main_FER_Hex`, `3rd_FER_Hex`, `4th_FER_Hex`). The cells are lowercase
//! hexadecimal digits without `0x`, read as the number they write, `1a5` as 421. The files name
//! the bits: the 0/1 columns after the registers spell them out, and in every row of the 45
//! low-rate files read, `Rocket_FER_Hex` equals the 13 flags `Liftoff` … `Reserved` read with
//! `Liftoff` as bit 0, and each channel's register equals its 14 condition flags `LT_AGL1` …
//! `Reserved`, `LT_AGL1` as bit 0. Each flag is kept as a channel, 0 or 1. The manual's tables of
//! deployment settings list the rocket's events as liftoff, apogee, pressure increasing, burnout
//! coast and each output fired (p26–28); the channel flags are the conditions an output's logic
//! checks, such as `LT_AGL1`, "AGL < AGL1" (p25).
//!
//! Seven flags mark a moment and become an [`Event`], named as the column, at each row where the
//! flag goes from 0 to 1: `Liftoff`, `Apogee`, `Burnout_Coast`, `Apo_fired`, `Main_fired`,
//! `3rd_fired` and `4th_fired`. As of 2026-10-12, in the 45 low-rate files among the project's
//! sample logs, each rises at most 15 times in a file, and in most once or twice (twice in a file
//! holding two flights, or a stage's second burn). The others stay channels only: `Press_Increasing`,
//! `Normal_Ascent`, `Accel_Vel_LE_0`, `ECI_Vvel_le_0` and `Tilt Exceeded 90deg` state a condition
//! that comes and goes (`ECI_Vvel_le_0` rises up to 34 times in a file), and the channel flags
//! are a deployment logic's conditions, not what happened. A flag already 1 on the first row
//! rose before the file began, so it gives no event, with a note; past [`MAX_EVENTS_PER_FLAG`]
//! rises in one column, the rest are counted in a note instead.
//!
//! **Clocks.** `Flight_Time_(s)` is the clock: strictly increasing in every file read, from a
//! negative time on the pad in all but one. A time that doesn't increase is refused, naming its
//! line. The wall clock `Time` is a channel only: as of 2026-10-12, among the sample logs, it
//! holds one value for several rows at a time in 5 of the high-rate files, and steps back in 4 of
//! the low-rate files, from once to hundreds of times, by up to about 14.5 hours. It is kept as
//! written, and a note says when it goes back. `Sync` steps by 2 a row in the high-rate files and
//! by 20 in the low-rate ones (40 where the file is logged every 0.04 s), in milliseconds, wrapping
//! at 250.
//!
//! **The primary track.** For a low-rate file, [`FlightLog::time_s`] is `Flight_Time_(s)` and
//! [`FlightLog::altitude_m`] is `Baro_Altitude_AGL_(feet)`, the barometer's altitude above the
//! pad; the temperature and battery are `Temperature_(F)` and `Batt_Volts`. The vertical speed is
//! `None`: `Velocity_Up` is the inertial solution's, which "lose\[s\] track of which way is up"
//! when the rocket turns faster than the gyros measure (p23), not the barometer's, so it is a
//! channel only. As of 2026-10-12, in 10 of the 45 low-rate files `Inertial_Altitude` steps by
//! more than 32,768 ft between two rows, as a 16-bit count of feet does when it wraps; it is kept
//! as written, with a note. A high-rate file holds no altitude: it is read with an empty primary track and every
//! column as a channel, with a note that the low-rate file of the same flight holds the altitude.
//!
//! **Cells and rows.** A cell that is empty, or isn't a number (or a time of day, or hexadecimal
//! digits, as its column needs) is a gap, `NaN`, counted per column in a note; none of the files
//! read has one. A row with fewer cells than the header has the rest as gaps, counted in a note;
//! a row with more is refused, as is a row whose clock cell isn't a number. A file whose rows, down
//! to any row, hold under a quarter of the cells its header names (past the first 65,536) is
//! refused at that row, so a wide header over short rows can't make the reader allocate
//! far more than the file holds. Blank lines are skipped;
//! lines may end in CR, LF or both; a byte-order mark is dropped; cells may be padded with spaces.
//!
//! **What the file says about itself.** The logger is a `Featherweight Blue Raven`; the data
//! files state no serial number, firmware, flight number or apogee (the summary file does).
//!
//! **Where this comes from.** The layout, from the files themselves and Featherweight's manual as
//! cited; the column names and the gyro and acceleration units agree with Debrief's reader
//! (`lib/parsers/blueraven.ts`, MIT, the project owner's own; see `THIRD-PARTY-NOTICES.md`). Where
//! this reader departs from Debrief's: it keeps every column, where Debrief keeps a chosen few; it
//! reads a high-rate file on its own, where Debrief refuses it and points to the low-rate file; it
//! keeps `Inertial_Altitude` whole, with a note, where Debrief cuts it off where it wraps or
//! leaves the barometer's altitude by more than the flight's height.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated, too_sparse};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Blue Raven CSV";

/// The columns both of the app's files open with, in order.
const LEAD: [&str; 6] = ["Year", "Month", "Day", "Time", "Flight_Time_(s)", "Sync"];

/// The place of the clock, `Flight_Time_(s)`, among [`LEAD`]'s columns (0-based).
const CLOCK_INDEX: usize = 4;

/// Pascals in a standard atmosphere, exactly (ISO 2533).
const ATM_PA: f64 = 101_325.0;

/// The most events one flag column gives; past it, its rises are counted in a note. The files
/// read rise at most 15 times in a column.
pub const MAX_EVENTS_PER_FLAG: usize = 16;

/// How far `Inertial_Altitude` may step between two rows, ft, before the step is counted as a
/// wrap: half the span of a 16-bit count of feet.
const WRAP_STEP_FT: f64 = 32_768.0;

/// The most rows from the file's start the acceleration's pad magnitude is taken over when the
/// file has no row before liftoff.
const PAD_ROWS: usize = 50;

/// Whether the text is in this reader's layout, judged from its header, never the file's name:
/// its first line that isn't blank opens with [`LEAD`]'s six columns and names more.
pub(crate) fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .map(|(_, line)| line)
        .find(|line| !line.trim().is_empty())
        .is_some_and(opens_with_lead)
}

/// Whether a header line opens with [`LEAD`]'s columns and names at least one more.
fn opens_with_lead(header: &str) -> bool {
    let mut cells = header.split(',').map(str::trim);
    LEAD.iter().all(|name| cells.next() == Some(*name)) && cells.next().is_some()
}

/// How a value becomes SI.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Scale {
    /// Multiplied by a factor.
    By(f64),
    /// Degrees Fahrenheit to kelvin.
    Fahrenheit,
}

/// What a column is to this reader.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    /// `Flight_Time_(s)`: the clock.
    Clock,
    /// `Time`: the wall clock, `HH:MM:SS.fff`.
    TimeOfDay,
    /// A number in a quantity, made SI by a scale.
    Value(Quantity, Scale),
    /// An event register, hexadecimal digits.
    Hex,
    /// A 0/1 flag; `event` if its rise marks a moment.
    Flag { event: bool },
}

/// The kinds of column whose unit no header states, each decided once, with a note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decided {
    Velocity,
    Inertial,
    Gyro,
    Accel,
}

/// A column's name with its spaces taken out, in lower case, as [`role`] matches it.
fn key(name: &str) -> String {
    name.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase()
}

/// What a column is, and which decided kind it falls under; `None` for a name the reader doesn't
/// know. The clock is matched only in its place ([`CLOCK_INDEX`]).
fn role(name: &str) -> Option<(Role, Option<Decided>)> {
    use Quantity as Q;
    let key = key(name);
    let value = |quantity, scale| Some((Role::Value(quantity, scale), None));
    let decided = |quantity, scale, kind| Some((Role::Value(quantity, scale), Some(kind)));
    let feet = Scale::By(FOOT_M);
    let gravities = Scale::By(STANDARD_GRAVITY_MPS2);
    let one = Scale::By(1.0);
    match key.as_str() {
        "time" => Some((Role::TimeOfDay, None)),
        "year" | "month" | "day" | "sync" | "quat_1" | "quat_2" | "quat_3" | "quat_4" => {
            value(Q::Number, one)
        }
        "temperature_(f)" => value(Q::Temperature, Scale::Fahrenheit),
        "baro_press_(atm)" => value(Q::Pressure, Scale::By(ATM_PA)),
        "baro_altitude_agl_(feet)" => value(Q::AltitudeAboveGround, feet),
        "baro_altitude_asl_(feet)" => value(Q::AltitudeMsl, feet),
        "axialaccel(gs)" | "lateralaccel(gs)" => value(Q::Acceleration, gravities),
        "tilt_angle_(deg)" | "future_angle_(deg)" | "roll_angle_(deg)" => value(Q::Angle, one),
        "velocity_up" | "velocity_dr" | "velocity_cr" => decided(Q::Speed, feet, Decided::Velocity),
        "inertial_altitude" => decided(Q::InertialAltitude, feet, Decided::Inertial),
        "inertial_dr_position" | "inertial_cr_position" => {
            decided(Q::Distance, feet, Decided::Inertial)
        }
        "gyro_x" | "gyro_y" | "gyro_z" => decided(Q::AngularRate, one, Decided::Gyro),
        "accel_x" | "accel_y" | "accel_z" => decided(Q::Acceleration, gravities, Decided::Accel),
        "current" => value(Q::Unrecognized, one),
        "liftoff" | "apogee" | "burnout_coast" | "apo_fired" | "main_fired" | "3rd_fired"
        | "4th_fired" => Some((Role::Flag { event: true }, None)),
        "press_increasing" | "normal_ascent" | "accel_vel_le_0" | "eci_vvel_le_0"
        | "tiltexceeded90deg" | "lt_agl1" | "gt_agl2" | "lt_fang1" | "gt_fang2" | "gt_futang"
        | "lt_tval1" | "gt_tval2" | "lt_vel1" | "gt_vel2" | "lt_bvel1" | "gt_burn" | "armed" => {
            Some((Role::Flag { event: false }, None))
        }
        _ if key.ends_with("_volts") => value(Q::Voltage, one),
        _ if key.ends_with("_fer_hex") => Some((Role::Hex, None)),
        _ => None,
    }
}

/// One column of the file, as read so far.
struct Column {
    name: String,
    key: String,
    role: Role,
    decided: Option<Decided>,
    values: Vec<f64>,
    /// Cells that were empty or not what the column holds, and the first of them as written.
    gaps: usize,
    gap_example: Option<String>,
    /// For an event flag: the events it gave, rises past the cap, and whether the first row was
    /// already set.
    events_kept: usize,
    rises_left_out: usize,
    set_on_first_row: bool,
}

impl Column {
    fn quantity(&self) -> Quantity {
        match self.role {
            Role::Value(quantity, _) => quantity,
            Role::TimeOfDay => Quantity::TimeOfDay,
            Role::Clock | Role::Hex | Role::Flag { .. } => Quantity::Number,
        }
    }
}

/// Reads a Featherweight Blue Raven's low-rate or high-rate CSV, as its phone app exports it.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't the app's header; [`LogError::Syntax`] for
/// a header column with no name, a row with more cells than the header, a row whose
/// `Flight_Time_(s)` is missing, isn't a number or doesn't come after the row above's, or rows
/// holding under a quarter of the cells the header names; [`LogError::NoData`] if no row follows
/// the header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    if !opens_with_lead(header) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't the app's header, which opens `{}`",
                short(header),
                LEAD.join(",")
            ),
        });
    }
    // Notes about single columns, capped; notes about the whole file, one each at most.
    let mut notes = Notes::default();
    let mut file_notes = Vec::new();
    let mut columns = header_columns(header, header_line, &mut notes, &mut file_notes)?;

    let mut time_s: Vec<f64> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let (mut rows, mut cells_written) = (0usize, 0usize);
    let mut short_rows = 0usize;
    let mut first_short_row = 0usize;
    for (number, line) in lines {
        let cells = read_row(&mut columns, &mut time_s, &mut events, line, number)?;
        rows += 1;
        cells_written += cells;
        // Checked row by row, so that a wide header over short rows is refused before the gaps
        // it would leave grow past a quarter more than the cells the file holds.
        if too_sparse(columns.len(), rows, cells_written) {
            return Err(syntax(
                number,
                format!(
                    "the header names {} columns, and the {rows} rows down to this one hold only \
                     {cells_written} cells",
                    columns.len()
                ),
            ));
        }
        if cells < columns.len() {
            if short_rows == 0 {
                first_short_row = number;
            }
            short_rows += 1;
        }
    }
    if rows == 0 {
        return Err(LogError::NoData { format: FORMAT });
    }
    if short_rows > 0 {
        file_notes.push(format!(
            "{short_rows} rows, the first on line {first_short_row}, have fewer cells than the \
             header's {}: the cells they leave out are gaps",
            columns.len()
        ));
    }
    Ok(build(columns, time_s, events, notes, file_notes))
}

/// The columns the header names, each with its role; unknown names noted.
fn header_columns(
    header: &str,
    line: usize,
    notes: &mut Notes,
    file_notes: &mut Vec<String>,
) -> Result<Vec<Column>, LogError> {
    let mut columns = Vec::new();
    let mut reserved = 0usize;
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        let (role, decided) = if index == CLOCK_INDEX {
            (Role::Clock, None)
        } else if key(name).starts_with("reserved") {
            reserved += 1;
            (Role::Value(Quantity::Unrecognized, Scale::By(1.0)), None)
        } else {
            role(name).unwrap_or_else(|| {
                notes.push(format!(
                    "the column {:?} isn't one this reader knows, and was kept as written",
                    short(name)
                ));
                (Role::Value(Quantity::Unrecognized, Scale::By(1.0)), None)
            })
        };
        columns.push(Column {
            name: name.to_owned(),
            key: key(name),
            role,
            decided,
            values: Vec::new(),
            gaps: 0,
            gap_example: None,
            events_kept: 0,
            rises_left_out: 0,
            set_on_first_row: false,
        });
    }
    if reserved > 0 {
        file_notes.push(format!(
            "{reserved} columns named `Reserved…` hold what the manual doesn't describe, and were \
             kept as written"
        ));
    }
    Ok(columns)
}

/// Reads one row into the clock and the columns, and any event its flags mark; returns how many
/// cells it has.
fn read_row(
    columns: &mut [Column],
    time_s: &mut Vec<f64>,
    events: &mut Vec<Event>,
    line: &str,
    number: usize,
) -> Result<usize, LogError> {
    let mut cells = line.split(',').map(str::trim);
    // The clock comes before every flag, so a flag's event takes the row's time.
    let mut row_time = f64::NAN;
    let mut count = 0;
    for (index, column) in columns.iter_mut().enumerate() {
        let Some(cell) = cells.next() else {
            if column.role == Role::Clock {
                return Err(syntax(
                    number,
                    format!(
                        "the row ends after {count} cells, before its {:?}",
                        short(&column.name)
                    ),
                ));
            }
            column.values.push(f64::NAN);
            continue;
        };
        count += 1;
        if column.role == Role::Clock {
            let time = number_cell(cell).ok_or_else(|| {
                syntax(
                    number,
                    format!(
                        "the time {:?} in cell {} isn't a finite number",
                        short(cell),
                        index + 1
                    ),
                )
            })?;
            if let Some(&last) = time_s.last()
                && time <= last
            {
                return Err(syntax(
                    number,
                    format!("the time {time} s doesn't come after the one above it, {last} s"),
                ));
            }
            row_time = time;
            time_s.push(time);
            continue;
        }
        let value = match column.role {
            Role::Value(_, scale) => number_cell(cell).map(|value| si(scale, value)),
            Role::TimeOfDay => time_of_day(cell),
            Role::Hex => hex(cell),
            Role::Flag { .. } => number_cell(cell),
            Role::Clock => None,
        };
        let value = value.unwrap_or_else(|| {
            column.gaps += 1;
            if column.gap_example.is_none() {
                column.gap_example = Some(short(cell));
            }
            f64::NAN
        });
        if let Role::Flag { event: true } = column.role {
            flag_event(column, value, row_time, events);
        }
        column.values.push(value);
    }
    if cells.next().is_some() {
        return Err(syntax(
            number,
            format!(
                "the row has {} cells, more than the header's {}",
                line.split(',').count(),
                columns.len()
            ),
        ));
    }
    Ok(count)
}

/// An event when a flag rises from 0 to 1, up to [`MAX_EVENTS_PER_FLAG`] per column; a flag set
/// on the first row is marked instead. Called before the value is stored.
fn flag_event(column: &mut Column, value: f64, time_s: f64, events: &mut Vec<Event>) {
    let risen = value == 1.0;
    match column.values.last() {
        None => column.set_on_first_row = risen,
        Some(&before) if before == 0.0 && risen => {
            if column.events_kept < MAX_EVENTS_PER_FLAG {
                column.events_kept += 1;
                events.push(Event {
                    time_s,
                    name: column.name.clone(),
                });
            } else {
                column.rises_left_out += 1;
            }
        }
        Some(_) => {}
    }
}

/// A cell as a finite number, `None` if it isn't one.
fn number_cell(cell: &str) -> Option<f64> {
    number(cell, false)
}

/// A value in SI.
fn si(scale: Scale, value: f64) -> f64 {
    match scale {
        Scale::By(factor) => value * factor,
        Scale::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
    }
}

/// A wall-clock cell, `H:MM:SS` or `HH:MM:SS` with any decimals, as seconds after midnight;
/// `None` for anything else, or a time past 23:59:60.999….
fn time_of_day(cell: &str) -> Option<f64> {
    let mut parts = cell.split(':');
    let (hours, minutes, seconds) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let digits = |text: &str| !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
    let (whole, fraction) = seconds.split_once('.').unwrap_or((seconds, "0"));
    if !(digits(hours) && hours.len() <= 2)
        || !(digits(minutes) && minutes.len() == 2)
        || !(digits(whole) && whole.len() == 2)
        || !digits(fraction)
    {
        return None;
    }
    let hours: u32 = hours.parse().ok()?;
    let minutes: u32 = minutes.parse().ok()?;
    let seconds: f64 = seconds.parse().ok()?;
    (hours < 24 && minutes < 60 && seconds < 61.0)
        .then(|| f64::from(hours * 3_600 + minutes * 60) + seconds)
}

/// An event register's cell, hexadecimal digits with or without `0x`, as the number they write;
/// `None` for anything else, or more digits than an `f64` holds exactly.
fn hex(cell: &str) -> Option<f64> {
    let digits = cell
        .strip_prefix("0x")
        .or_else(|| cell.strip_prefix("0X"))
        .unwrap_or(cell);
    if digits.is_empty() || digits.len() > 13 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    // At most 13 hexadecimal digits, 52 bits, so the value is below 2⁵² and exact as an f64.
    u64::from_str_radix(digits, 16).ok().map(|bits| bits as f64)
}

/// The record from the columns read: every channel, the primary track, the events and the notes.
fn build(
    columns: Vec<Column>,
    time_s: Vec<f64>,
    events: Vec<Event>,
    mut notes: Notes,
    mut file_notes: Vec<String>,
) -> FlightLog {
    let mut log = FlightLog {
        format: LogFormat::BlueRavenCsv,
        logger: "Featherweight Blue Raven".to_owned(),
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
        events,
        notes: decided_notes(&columns, &time_s),
    };

    for column in &columns {
        if column.gaps > 0 {
            notes.push(format!(
                "the column {:?} holds {} cells that are empty or not what it holds, such as \
                 {:?}: they are gaps",
                short(&column.name),
                column.gaps,
                column.gap_example.as_deref().unwrap_or_default()
            ));
        }
        if column.set_on_first_row {
            notes.push(format!(
                "the flag {:?} is already 1 on the first row: it rose before the file begins, so \
                 the file doesn't say when",
                short(&column.name)
            ));
        }
        if column.rises_left_out > 0 {
            notes.push(format!(
                "the flag {:?} rises {} more times than the {MAX_EVENTS_PER_FLAG} kept as events",
                short(&column.name),
                column.rises_left_out
            ));
        }
        match column.role {
            Role::TimeOfDay => {
                let (backs, largest) = steps_back(&column.values);
                if backs > 0 {
                    notes.push(format!(
                        "the logger's time of day, {:?}, goes back {backs} times, by up to \
                         {largest:.3} s: it is kept as written, and `Flight_Time_(s)` is the clock",
                        short(&column.name)
                    ));
                }
            }
            Role::Value(Quantity::InertialAltitude, _) => {
                let wraps = column
                    .values
                    .windows(2)
                    .filter(|pair| (pair[1] - pair[0]).abs() / FOOT_M > WRAP_STEP_FT)
                    .count();
                if wraps > 0 {
                    notes.push(format!(
                        "{:?} steps by more than 32,768 ft between two rows {wraps} times, as a \
                         16-bit count of feet does when it wraps: it is kept as written",
                        short(&column.name)
                    ));
                }
            }
            _ => {}
        }
    }

    let altitude = columns.iter().find(|c| c.key == "baro_altitude_agl_(feet)");
    if let Some(altitude) = altitude {
        log.time_s.clone_from(&time_s);
        log.altitude_m.clone_from(&altitude.values);
        log.temperature_k = columns
            .iter()
            .find(|c| c.key == "temperature_(f)")
            .map(|c| c.values.clone());
        log.battery_v = columns
            .iter()
            .find(|c| c.key == "batt_volts")
            .map(|c| c.values.clone());
    } else {
        file_notes.push(
            "the file has no `Baro_Altitude_AGL_(feet)`, so the record's altitude track is empty: \
             a Blue Raven's high-rate file holds its inertial sensors only, and the low-rate file \
             of the same flight holds the altitude"
                .to_owned(),
        );
    }

    for column in columns {
        if column.role == Role::Clock {
            continue;
        }
        log.channels.push(Channel {
            quantity: column.quantity(),
            column: column.name,
            time_s: time_s.clone(),
            values: column.values,
        });
    }
    log.notes.extend(file_notes);
    log.notes.extend(notes.into_vec());
    log
}

/// How many times a list of times of day goes back, and the largest step back, s.
fn steps_back(values: &[f64]) -> (usize, f64) {
    values
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .filter(|back| *back > 0.0)
        .fold((0, 0.0), |(count, largest), back| {
            (count + 1, f64::max(largest, back))
        })
}

/// The notes on units no header states: one per decided kind the file holds, its columns, the
/// decision, and this file's evidence.
fn decided_notes(columns: &[Column], time_s: &[f64]) -> Vec<String> {
    let find = |key: &str| columns.iter().find(|c| c.key == key);
    let altitude = find("baro_altitude_agl_(feet)");
    let kinds = [
        (
            Decided::Velocity,
            "feet per second",
            "the manual and the summary",
        ),
        (Decided::Inertial, "feet", "the manual and the summary"),
        (
            Decided::Gyro,
            "degrees per second",
            "the manual and the summary",
        ),
        (
            Decided::Accel,
            "standard gravities",
            "the manual and the pad",
        ),
    ];
    let mut notes = Vec::new();
    for (kind, unit, source) in kinds {
        let names: Vec<String> = columns
            .iter()
            .filter(|c| c.decided == Some(kind))
            .map(|c| format!("`{}`", short(&c.name)))
            .collect();
        if names.is_empty() {
            continue;
        }
        let evidence = match kind {
            Decided::Velocity => velocity_evidence(find("velocity_up"), altitude, time_s),
            Decided::Inertial => inertial_evidence(find("inertial_altitude"), altitude),
            Decided::Gyro => gyro_evidence(columns),
            Decided::Accel => {
                accel_evidence([find("accel_x"), find("accel_y"), find("accel_z")], time_s)
            }
        };
        notes.push(format!(
            "{} read in {unit}, from {source}: {evidence}",
            list(&names)
        ));
    }
    if let Some(current) = find("current") {
        notes.push(current_note(current));
    }
    notes
}

/// The most names a decided-unit note lists; past it, the rest are counted.
const MAX_LISTED: usize = 6;

/// Names joined as `a`, `a and b`, or `a, b and c`; past [`MAX_LISTED`], the first few and a
/// count of the rest, so that a header repeating a name many times keeps its note short.
fn list(names: &[String]) -> String {
    if names.len() > MAX_LISTED {
        let shown = MAX_LISTED - 1;
        return format!(
            "{} and {} more columns",
            names[..shown].join(", "),
            names.len() - shown
        );
    }
    match names {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// The highest finite value of a list and its place.
fn highest(values: &[f64]) -> Option<(usize, f64)> {
    values
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .fold(None, |best, (index, value)| match best {
            Some((_, top)) if top >= value => best,
            _ => Some((index, value)),
        })
}

/// `Velocity_Up` integrated over the climb to the barometer's highest sample, against the
/// barometer's rise over the same rows, both in feet.
fn velocity_evidence(up: Option<&Column>, altitude: Option<&Column>, time_s: &[f64]) -> String {
    let manual = "the manual gives the USB download's velocities in ft/sec";
    let (Some(up), Some(altitude)) = (up, altitude) else {
        return format!(
            "{manual}; the file has no `Velocity_Up` and barometric altitude to compare"
        );
    };
    let Some((peak, top)) = highest(&altitude.values) else {
        return format!("{manual}; the file's barometric altitude has no value");
    };
    let first = altitude.values.iter().find(|value| value.is_finite());
    let rise_ft = first.map_or(f64::NAN, |first| (top - first) / FOOT_M);
    let climbed_ft: f64 = (1..=peak)
        .filter_map(|index| {
            let (v0, v1) = (up.values.get(index - 1)?, up.values.get(index)?);
            let dt = time_s.get(index)? - time_s.get(index - 1)?;
            (v0.is_finite() && v1.is_finite()).then(|| 0.5 * (v0 + v1) * dt / FOOT_M)
        })
        .sum();
    if rise_ft.is_finite() && rise_ft > 0.0 {
        format!(
            "over the climb to the barometer's highest sample, `Velocity_Up` integrates to \
             {climbed_ft:.0} ft against the barometer's rise of {rise_ft:.0} ft, a ratio of {:.2}",
            climbed_ft / rise_ft
        )
    } else {
        format!("{manual}; the barometric altitude doesn't rise to compare with")
    }
}

/// `Inertial_Altitude`'s peak against the barometer's, both in feet.
fn inertial_evidence(inertial: Option<&Column>, altitude: Option<&Column>) -> String {
    let manual = "the manual gives the USB download's inertial altitude and ranges in feet";
    let peaks = inertial
        .and_then(|c| highest(&c.values))
        .zip(altitude.and_then(|c| highest(&c.values)));
    match peaks {
        Some(((_, inertial), (_, baro))) => format!(
            "`Inertial_Altitude` peaks at {:.0} ft against the barometric altitude's {:.0} ft",
            inertial / FOOT_M,
            baro / FOOT_M
        ),
        None => format!("{manual}; the file has no inertial and barometric peaks to compare"),
    }
}

/// The largest gyro rate in the file, against the gyros' stated range.
fn gyro_evidence(columns: &[Column]) -> String {
    let largest = columns
        .iter()
        .filter(|c| c.decided == Some(Decided::Gyro))
        .flat_map(|c| c.values.iter())
        .filter(|value| value.is_finite())
        .fold(0.0_f64, |largest, value| largest.max(value.abs()));
    format!(
        "the largest rate in the file is {largest:.0}, against the gyros' range of ±2000 deg/sec \
         the manual gives"
    )
}

/// The median magnitude of the three accelerations on the pad, in g: over the rows before
/// liftoff (a negative `Flight_Time_(s)`), or the first [`PAD_ROWS`] rows if there are none.
fn accel_evidence(axes: [Option<&Column>; 3], time_s: &[f64]) -> String {
    let [Some(x), Some(y), Some(z)] = axes else {
        return "the file lacks an axis to take the magnitude from; the manual gives the USB \
                download's accelerations in G"
            .to_owned();
    };
    let before = time_s.iter().take_while(|time| **time < 0.0).count();
    let (rows, which) = if before > 0 {
        (before, format!("the {before} rows before liftoff"))
    } else {
        let rows = time_s.len().min(PAD_ROWS);
        (rows, format!("the first {rows} rows"))
    };
    let mut magnitudes: Vec<f64> = (0..rows)
        .filter_map(|i| {
            let g =
                (x.values.get(i)?.powi(2) + y.values.get(i)?.powi(2) + z.values.get(i)?.powi(2))
                    .sqrt()
                    / STANDARD_GRAVITY_MPS2;
            g.is_finite().then_some(g)
        })
        .collect();
    magnitudes.sort_by(f64::total_cmp);
    match magnitudes.get(magnitudes.len() / 2) {
        Some(median) => format!(
            "the median magnitude of the three axes over {which} is {median:.3}, where a logger \
             at rest reads 1 g"
        ),
        None => format!("the three axes have no value over {which} to take the magnitude from"),
    }
}

/// The note on `Current`, kept as written: how often it reads other than 0, and its largest value.
fn current_note(current: &Column) -> String {
    let values = &current.values[..];
    let finite = values.iter().filter(|value| value.is_finite()).count();
    let nonzero = values
        .iter()
        .filter(|value| value.is_finite() && **value != 0.0)
        .count();
    let largest = values
        .iter()
        .filter(|value| value.is_finite())
        .fold(0.0_f64, |largest, value| largest.max(value.abs()));
    format!(
        "`{}` kept as written, in a unit no source states: it is other than 0 in {nonzero} of \
         {finite} rows, at most {largest:.3}; amperes would fit the manual's limit of about 4 A on \
         the board's current, the USB download's milliamperes would not",
        short(&current.name)
    )
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The low-rate file's first 45 columns, with the fifth register's name as most files write it.
    const LOW_LEAD: &str = "Year,Month,Day,Time,Flight_Time_(s),Sync,Temperature_(F),Baro_Press_(atm),\
        Baro_Altitude_ASL_(feet),Baro_Altitude_AGL_(feet),Batt_Volts,Apo_Volts,Main_Volts,3rd_Volts,\
        4th_Volts,Velocity_Up,Velocity_DR,Velocity_CR,Inertial_Altitude,Inertial_DR_Position,\
        Inertial_CR_position,Tilt_Angle_(deg),Future_Angle_(deg),Roll_Angle_(deg),Reserved_1,\
        Reserved_2,Reserved_3,Rocket_FER_Hex,Apo_FER_H  ex,Main_FER_Hex,3rd_FER_Hex,4th_FER_Hex,\
        Liftoff,Apogee,Press_Increasing,Burnout_Coast,Apo_fired,Main_fired,3rd_fired,4th_fired,\
        Normal_Ascent,Accel_Vel_LE_0,ECI_Vvel_le_0,Tilt Exceeded 90deg,Reserved";

    /// One output channel's condition flags, repeated for each of the four.
    const CHANNEL_FLAGS: &str = "LT_AGL1,GT_AGL2,LT_FANG1,GT_FANG2,GT_FUTANG,LT_TVAL1,GT_TVAL2,\
        LT_VEL1,GT_VEL2,LT_BVEL1,GT_BURN,Armed,Reserved,Reserved";

    const HIGH_HEADER: &str = "Year,Month,Day,Time,Flight_Time_(s),Sync,Gyro_X,Gyro_Y,Gyro_Z,\
        Accel_X,Accel_Y,Accel_Z,Quat_1,Quat_2,Quat_3,Quat_4,Aux_Volts,Current";

    fn low_header() -> String {
        format!("{LOW_LEAD}{}", format!(",{CHANNEL_FLAGS}").repeat(4))
    }

    /// A low-rate row: its time, time of day, altitude above the pad in feet, upward velocity in
    /// ft/s, and the rocket's 13 flags, `Liftoff` first; the registers written from the flags.
    fn low_row(time: f64, of_day: &str, agl_ft: f64, up_ft_s: f64, flags: [u8; 13]) -> String {
        let register: u32 = flags
            .iter()
            .enumerate()
            .map(|(bit, flag)| u32::from(*flag) << bit)
            .sum();
        let flags: Vec<String> = flags.iter().map(u8::to_string).collect();
        format!(
            "2026,4,18,{of_day},{time},0,50,0.95,{},{agl_ft},4.1,4.0,4.0,0,0,{up_ft_s},2,-1,{agl_ft},\
             10,-3,2.5,3,40,0,0,0,{register:x},0,0,0,0,{}{}",
            1000.0 + agl_ft,
            flags.join(","),
            ",0".repeat(56)
        )
    }

    fn high_row(time: f64, of_day: &str, current: &str) -> String {
        format!("2026,4,18,{of_day},{time},0,1.5,-2,300,0.6,0,0.8,0,0,0.6,0.8,0.02,{current}")
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-9 * b.abs().max(1.0)
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .unwrap()
    }

    const NO_FLAGS: [u8; 13] = [0; 13];
    const LIFTOFF: [u8; 13] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const APOGEE: [u8; 13] = [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];

    #[test]
    fn the_app_header_is_recognized_by_its_six_first_columns() {
        assert!(recognizes(&format!("\u{feff}\r\n{}\n", low_header())));
        assert!(recognizes(HIGH_HEADER));
        assert!(recognizes(
            "Year, Month, Day, Time, Flight_Time_(s), Sync, Gyro_X\n"
        ));
        for other in [
            "",
            "Year,Month,Day,Time,Flight_Time_(s),Sync\n",
            "Year,Month,Day,Time,Flight_Time_(s),Axial Accel (Gs)\n",
            "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA\n",
            "Time,Altitude,Velocity\n",
            "year,month,day,time,flight_time_(s),sync,x\n",
        ] {
            assert!(!recognizes(other), "{other:?}");
        }
    }

    /// The Raven's FIP check runs first in the importer, on a header that opens with `Time` and
    /// names `Axial Accel (Gs)`: a Blue Raven low-rate header naming that column still reads as
    /// a Blue Raven's.
    #[test]
    fn a_low_rate_header_naming_the_ravens_axial_accel_is_a_blue_ravens() {
        let header =
            low_header().replace("3rd_Volts,4th_Volts", "Axial Accel (Gs),Lateral Accel (Gs)");
        let text = format!(
            "{header}\n{}\n",
            low_row(0.0, "10:00:00.000", 0.0, 0.0, NO_FLAGS)
        );
        assert_eq!(
            crate::import::detect(text.as_bytes()),
            Some(LogFormat::BlueRavenCsv)
        );
        let log = crate::import::read(text.as_bytes()).unwrap();
        let axial = channel(&log, "Axial Accel (Gs)");
        assert_eq!(axial.quantity, Quantity::Acceleration);
        assert_eq!(axial.values, [0.0]);
    }

    /// The low-rate file: every column but the clock a channel in the file's order and in SI,
    /// the primary track the barometer's, the registers read as hexadecimal, and events where
    /// a moment's flag rises.
    #[test]
    fn reads_a_low_rate_file() {
        let text = format!(
            "{}\n{}\n{}\n{}\n{}\n",
            low_header(),
            low_row(-0.04, "10:00:00.000", 0.0, 0.0, NO_FLAGS),
            low_row(-0.02, "10:00:00.020", 0.5, 0.0, NO_FLAGS),
            low_row(0.0, "10:00:00.040", 1.0, 30.0, LIFTOFF),
            low_row(0.02, "10:00:00.060", 2.0, 60.0, APOGEE),
        );
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::BlueRavenCsv);
        assert_eq!(log.logger, "Featherweight Blue Raven");
        assert_eq!(log.channels.len(), 100);
        assert_eq!(log.channels[4].column, "Sync", "the clock is no channel");
        assert_eq!(log.time_s, [-0.04, -0.02, 0.0, 0.02]);
        assert!(close(log.altitude_m[3], 2.0 * FOOT_M));
        assert!(log.vertical_speed_m_s.is_none());
        assert!(close(log.temperature_k.as_ref().unwrap()[0], 283.15));
        assert_eq!(log.battery_v.as_deref(), Some(&[4.1; 4][..]));

        let of_day = channel(&log, "Time");
        assert_eq!(of_day.quantity, Quantity::TimeOfDay);
        assert!(close(of_day.values[1], 36_000.02));
        assert!(close(
            channel(&log, "Baro_Press_(atm)").values[0],
            0.95 * 101_325.0
        ));
        let up = channel(&log, "Velocity_Up");
        assert_eq!(up.quantity, Quantity::Speed);
        assert!(close(up.values[3], 60.0 * FOOT_M));
        assert_eq!(
            channel(&log, "Inertial_DR_Position").quantity,
            Quantity::Distance
        );
        assert_eq!(channel(&log, "Roll_Angle_(deg)").values[0], 40.0);
        assert_eq!(channel(&log, "Reserved_1").quantity, Quantity::Unrecognized);
        // `1` | `2` | `16` = 0x13: Liftoff, Apogee and Apo_fired.
        let register = channel(&log, "Rocket_FER_Hex");
        assert_eq!(register.quantity, Quantity::Number);
        assert_eq!(register.values, [0.0, 0.0, 1.0, 19.0]);
        assert_eq!(channel(&log, "Apo_FER_H  ex").quantity, Quantity::Number);
        let names: Vec<(&str, f64)> = log
            .events
            .iter()
            .map(|event| (event.name.as_str(), event.time_s))
            .collect();
        assert_eq!(
            names,
            [("Liftoff", 0.0), ("Apogee", 0.02), ("Apo_fired", 0.02)]
        );
        assert_eq!(
            log.channels
                .iter()
                .filter(|c| c.column == "LT_AGL1")
                .count(),
            4
        );

        assert!(
            log.notes[0].starts_with(
                "`Velocity_Up`, `Velocity_DR` and `Velocity_CR` read in feet per second, from the \
                 manual and the summary: over the climb to the barometer's highest sample, \
                 `Velocity_Up` integrates to 1 ft against the barometer's rise of 2 ft"
            ),
            "{:?}",
            log.notes
        );
        assert!(log.notes[1].starts_with(
            "`Inertial_Altitude`, `Inertial_DR_Position` and `Inertial_CR_position` read in feet, \
             from the manual and the summary: `Inertial_Altitude` peaks at 2 ft against the \
             barometric altitude's 2 ft"
        ));
        assert_eq!(
            log.notes[2],
            "12 columns named `Reserved…` hold what the manual doesn't describe, and were kept as \
             written"
        );
        assert_eq!(log.notes.len(), 3, "{:?}", log.notes);
    }

    /// The high-rate file: no altitude, so an empty primary track with a note, and the inertial
    /// sensors in SI with a note on each kind's unit.
    #[test]
    fn reads_a_high_rate_file_with_an_empty_track() {
        let text = format!(
            "{HIGH_HEADER}\r\n{}\r\n{}\r\n{}\r\n",
            high_row(-0.004, "10:00:00.000", "0.000"),
            high_row(-0.002, "10:00:00.000", "0.000"),
            high_row(0.0, "10:00:00.004", "2.5"),
        );
        let log = read(&text).unwrap();
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
        assert!(log.temperature_k.is_none() && log.battery_v.is_none());
        assert_eq!(log.channels.len(), 17);
        let gyro = channel(&log, "Gyro_Z");
        assert_eq!(gyro.quantity, Quantity::AngularRate);
        assert_eq!(gyro.time_s, [-0.004, -0.002, 0.0]);
        assert_eq!(gyro.values, [300.0; 3]);
        let accel = channel(&log, "Accel_Z");
        assert!(close(accel.values[0], 0.8 * STANDARD_GRAVITY_MPS2));
        assert_eq!(channel(&log, "Quat_4").quantity, Quantity::Number);
        assert_eq!(channel(&log, "Aux_Volts").quantity, Quantity::Voltage);
        let current = channel(&log, "Current");
        assert_eq!(current.quantity, Quantity::Unrecognized);
        assert_eq!(current.values, [0.0, 0.0, 2.5]);
        assert_eq!(
            log.notes,
            [
                "`Gyro_X`, `Gyro_Y` and `Gyro_Z` read in degrees per second, from the manual and \
                 the summary: the largest rate in the file is 300, against the gyros' range of \
                 ±2000 deg/sec the manual gives",
                "`Accel_X`, `Accel_Y` and `Accel_Z` read in standard gravities, from the manual \
                 and the pad: the median magnitude of the three axes over the 2 rows before \
                 liftoff is 1.000, where a logger at rest reads 1 g",
                "`Current` kept as written, in a unit no source states: it is other than 0 in 1 of \
                 3 rows, at most 2.500; amperes would fit the manual's limit of about 4 A on the \
                 board's current, the USB download's milliamperes would not",
                "the file has no `Baro_Altitude_AGL_(feet)`, so the record's altitude track is \
                 empty: a Blue Raven's high-rate file holds its inertial sensors only, and the \
                 low-rate file of the same flight holds the altitude",
            ]
        );
        assert!(log.events.is_empty());
        // The readings withhold on a Blue Raven's record, so an empty track reaches nothing that
        // would index it.
        let readings = crate::readings::read(&log);
        assert!(readings.apogee.value().is_none());
    }

    /// Each decided kind's note keeps the same words before its colon whatever the file's
    /// numbers, so a survey can count them; without the rows to compare, the note says so.
    #[test]
    fn decided_notes_keep_their_prefix() {
        let text = format!("{HIGH_HEADER}\n{}\n", high_row(0.5, "10:00:00.000", "0"));
        let log = read(&text).unwrap();
        assert!(log.notes[1].starts_with(
            "`Accel_X`, `Accel_Y` and `Accel_Z` read in standard gravities, from the manual and \
             the pad: the median magnitude of the three axes over the first 1 rows is 1.000"
        ));
        let header =
            "Year,Month,Day,Time,Flight_Time_(s),Sync,Velocity_Up,Inertial_Altitude,Accel_X";
        let log = read(&format!("{header}\n2026,1,1,0:00:00,0,0,1,1,1\n")).unwrap();
        assert_eq!(
            log.notes[..3],
            [
                "`Velocity_Up` read in feet per second, from the manual and the summary: the \
                 manual gives the USB download's velocities in ft/sec; the file has no \
                 `Velocity_Up` and barometric altitude to compare",
                "`Inertial_Altitude` read in feet, from the manual and the summary: the manual \
                 gives the USB download's inertial altitude and ranges in feet; the file has no \
                 inertial and barometric peaks to compare",
                "`Accel_X` read in standard gravities, from the manual and the pad: the file lacks \
                 an axis to take the magnitude from; the manual gives the USB download's \
                 accelerations in G",
            ]
        );
    }

    /// A flag already set on the first row gives no event, with a note; a flag that rises more
    /// often than the cap gives the first [`MAX_EVENTS_PER_FLAG`] and counts the rest; a
    /// condition flag gives none.
    #[test]
    fn flags_give_events_only_where_they_rise() {
        let mut text = low_header();
        let mut main = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for row in 0..40 {
            main[5] = u8::from(row % 2 == 1);
            main[10] = u8::from(row % 2 == 1);
            text.push('\n');
            text.push_str(&low_row(f64::from(row), "10:00:00", 0.0, 0.0, main));
        }
        let log = read(&text).unwrap();
        assert_eq!(log.events.len(), MAX_EVENTS_PER_FLAG);
        assert!(log.events.iter().all(|event| event.name == "Main_fired"));
        assert_eq!(log.events[0].time_s, 1.0);
        assert_eq!(log.events[15].time_s, 31.0);
        let notes = log.notes.join("\n");
        assert!(
            notes.contains("the flag \"Liftoff\" is already 1 on the first row"),
            "{notes}"
        );
        assert!(
            notes.contains("the flag \"Main_fired\" rises 4 more times than the 16 kept"),
            "{notes}"
        );
    }

    #[test]
    fn a_time_of_day_that_goes_back_is_kept_with_a_note() {
        let text = format!(
            "{HIGH_HEADER}\n{}\n{}\n{}\n{}\n",
            high_row(0.0, "23:59:59.990", "0"),
            high_row(0.1, "0:00:00.000", "0"),
            high_row(0.2, "0:00:00.010", "0"),
            high_row(0.3, "0:00:00.005", "0"),
        );
        let log = read(&text).unwrap();
        let of_day = channel(&log, "Time");
        assert!(close(of_day.values[0], 86_399.99));
        assert_eq!(of_day.values[1], 0.0);
        assert!(
            log.notes.contains(
                &"the logger's time of day, \"Time\", goes back 2 times, by up to 86399.990 s: it \
                  is kept as written, and `Flight_Time_(s)` is the clock"
                    .to_owned()
            ),
            "{:?}",
            log.notes
        );
    }

    #[test]
    fn a_note_lists_a_few_names() {
        let names: Vec<String> = (0..9).map(|i| format!("`g{i}`")).collect();
        assert_eq!(list(&names[..1]), "`g0`");
        assert_eq!(list(&names[..3]), "`g0`, `g1` and `g2`");
        assert_eq!(list(&names[..6]), "`g0`, `g1`, `g2`, `g3`, `g4` and `g5`");
        assert_eq!(
            list(&names),
            "`g0`, `g1`, `g2`, `g3`, `g4` and 4 more columns"
        );
    }

    #[test]
    fn times_of_day_and_registers_are_read_strictly() {
        assert_eq!(time_of_day("00:00:00"), Some(0.0));
        assert!(close(time_of_day("9:05:07.25").unwrap(), 32_707.25));
        assert!(close(time_of_day("23:59:60.5").unwrap(), 86_400.5));
        for bad in [
            "",
            "12:00",
            "12:00:00:00",
            "24:00:00",
            "12:60:00",
            "12:5:00",
            "12:00:61",
            "123:00:00",
            "12:00:00.",
            "12:00:00.x",
            "-1:00:00",
            "12:00:+1",
            "1e1:00:00",
        ] {
            assert_eq!(time_of_day(bad), None, "{bad:?}");
        }
        assert_eq!(hex("0"), Some(0.0));
        assert_eq!(hex("1a5"), Some(421.0));
        assert_eq!(hex("0x1A5"), Some(421.0));
        assert_eq!(hex("fffffffffffff"), Some(4_503_599_627_370_495.0));
        for bad in ["", "0x", "g", "-1", "1.5", "ffffffffffffff"] {
            assert_eq!(hex(bad), None, "{bad:?}");
        }
    }

    /// An empty cell, or one that isn't what its column holds, is a gap counted in a note; a
    /// short row's missing cells are gaps, counted.
    #[test]
    fn gaps_and_short_rows_are_counted() {
        let text = format!(
            "{HIGH_HEADER}\n\
             2026,4,18,noon,0,0,x,,300,0.6,0,0.8,0,0,0.6,0.8,0.02,0\n\
             2026,4,18,10:00:00,0.002,0,1,2,3\n\
             2026,4,18,10:00:00,0.004\n"
        );
        let log = read(&text).unwrap();
        assert!(channel(&log, "Time").values[0].is_nan());
        let gyro_x = channel(&log, "Gyro_X");
        assert!(gyro_x.values[0].is_nan());
        assert_eq!(gyro_x.values[1], 1.0);
        assert!(gyro_x.values[2].is_nan());
        assert!(channel(&log, "Current").values[1].is_nan());
        let notes = log.notes.join("\n");
        for expected in [
            "the column \"Time\" holds 1 cells that are empty or not what it holds, such as \
             \"noon\": they are gaps",
            "the column \"Gyro_X\" holds 1 cells that are empty or not what it holds, such as \
             \"x\"",
            "the column \"Gyro_Y\" holds 1 cells that are empty or not what it holds, such as \
             \"\"",
            "2 rows, the first on line 3, have fewer cells than the header's 18: the cells they \
             leave out are gaps",
        ] {
            assert!(notes.contains(expected), "{expected}\n{notes}");
        }
    }

    /// `Inertial_Altitude` stepping by more than half a 16-bit span of feet is noted, and kept.
    #[test]
    fn an_inertial_wrap_is_noted() {
        let header = "Year,Month,Day,Time,Flight_Time_(s),Sync,Inertial_Altitude";
        let text = format!("{header}\n2026,1,1,0:00:00,0,0,32767\n2026,1,1,0:00:00,1,0,-32768\n");
        let log = read(&text).unwrap();
        assert!(close(
            channel(&log, "Inertial_Altitude").values[1],
            -32_768.0 * FOOT_M
        ));
        assert!(log.notes.contains(
            &"\"Inertial_Altitude\" steps by more than 32,768 ft between two rows 1 times, as a \
              16-bit count of feet does when it wraps: it is kept as written"
                .to_owned()
        ));
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let row = |time: &str| format!("2026,4,18,10:00:00,{time},0,1,1,1,1,1,1,0,0,0,1,0,0");
        let cases = [
            (String::new(), "the file is empty"),
            ("\n\r\n".to_owned(), "the file is empty"),
            ("Time,Altitude\n0,0\n".to_owned(), "isn't the app's header"),
            (format!("{HIGH_HEADER}\n"), "no data rows"),
            (
                "Year,Month,Day,Time,Flight_Time_(s),Sync,,x\n".to_owned(),
                "line 1: the header's column 7 has no name",
            ),
            (
                format!("{HIGH_HEADER}\n{},9\n", row("0")),
                "line 2: the row has 19 cells, more than the header's 18",
            ),
            (
                format!("{HIGH_HEADER}\n{}\nx,y\n", row("0")),
                "line 3: the row ends after 2 cells, before its \"Flight_Time_(s)\"",
            ),
            (
                format!("{HIGH_HEADER}\n{}\n", row("soon")),
                "line 2: the time \"soon\" in cell 5 isn't a finite number",
            ),
            (
                format!("{HIGH_HEADER}\n{}\n", row("")),
                "line 2: the time \"\" in cell 5",
            ),
            (
                format!("{HIGH_HEADER}\n{}\n\n{}\n", row("1"), row("1")),
                "line 4: the time 1 s doesn't come after the one above it, 1 s",
            ),
            (
                format!("{HIGH_HEADER}\n{}\n{}\n", row("1"), row("0.5")),
                "line 3: the time 0.5 s doesn't come after",
            ),
        ];
        for (text, expected) in cases {
            let error = read(&text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
        let LogError::NotThisFormat { format, .. } = read("x").unwrap_err() else {
            panic!("refused for another reason");
        };
        assert_eq!(format, FORMAT);
    }

    /// A header naming far more columns than its rows fill is refused before the reader allocates
    /// a gap for every cell left out.
    #[test]
    fn a_wide_header_over_short_rows_is_refused() {
        let wide: String = (0..20_000).map(|i| format!(",c{i}")).collect();
        let header = format!("Year,Month,Day,Time,Flight_Time_(s),Sync{wide}");
        let rows: String = (0..100).map(|i| format!("0,0,0,0,{i},0\n")).collect();
        let error = read(&format!("{header}\n{rows}")).unwrap_err().to_string();
        assert!(
            error.contains("line 5: the header names 20006 columns, and the 4 rows down to this one hold only 24 cells"),
            "{error}"
        );
    }

    /// Lines end in CR, LF or both; a byte-order mark is dropped; blank lines are skipped.
    #[test]
    fn line_ends_and_marks() {
        let rows = [
            high_row(0.0, "10:00:00", "0"),
            high_row(0.002, "10:00:00", "0"),
        ];
        for end in ["\r", "\n", "\r\n"] {
            let text = format!(
                "\u{feff}{end}{HIGH_HEADER}{end}{}{end}{end}{}{end}",
                rows[0], rows[1]
            );
            let log = read(&text).unwrap();
            assert_eq!(log.channels[0].time_s, [0.0, 0.002], "{end:?}");
        }
    }

    /// A header of many unknown columns keeps a short list of notes and counts the rest; the
    /// unit notes stay outside the cap.
    #[test]
    fn notes_stay_few() {
        let unknown: String = (0..100).map(|i| format!(",u{i}")).collect();
        let header = format!("Year,Month,Day,Time,Flight_Time_(s),Sync,Gyro_X{unknown}");
        let row = format!("2026,1,1,0:00:00,0,0,1{}", ",1".repeat(100));
        let log = read(&format!("{header}\n{row}\n")).unwrap();
        assert!(log.notes[0].starts_with("`Gyro_X` read in degrees per second"));
        assert_eq!(log.notes.len(), 1 + 1 + 20 + 1, "{:?}", log.notes);
        assert!(log.notes[1].starts_with("the file has no `Baro_Altitude_AGL_(feet)`"));
        assert!(
            log.notes[22].starts_with("and 80 more notes"),
            "{:?}",
            log.notes
        );
        assert_eq!(log.channels.len(), 106);
        assert_eq!(log.channels[105].quantity, Quantity::Unrecognized);
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has increasing times and a
        /// value for each on every channel.
        #[test]
        fn any_text_is_read_or_refused(
            rows in proptest::collection::vec("[-0-9.,:xaf ]{0,40}", 0..10),
            low in proptest::bool::ANY,
        ) {
            let header = if low { low_header() } else { HIGH_HEADER.to_owned() };
            let text = format!("{header}\n{}", rows.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert!(channel.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
                for event in &log.events {
                    proptest::prop_assert!(event.time_s.is_finite());
                }
            }
        }
    }
}
