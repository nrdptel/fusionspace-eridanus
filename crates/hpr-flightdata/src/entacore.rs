//! Entacore's AIM flight computers' logs, as the AIM XTRA and AIM BASE software exports a flight
//! with *Export to Excel…*: a CSV file (*AIM XTRA/BASE v4.x.x Manual*, Entacore, 2 February 2025,
//! p38).
//!
//! The AIM XTRA samples its sensors at different rates: its barometer every 20 ms, its 100 g
//! accelerometer, 16 g three-axis accelerometer and gyroscope every 10 ms, its magnetometer every
//! 100 ms, its temperature every 500 ms and its GPS's position every 200 ms (manual p2), each slowed
//! by a factor the owner sets for each phase of the flight (p21). So the export gives each group of
//! channels its own clock: "there are multiple time columns. Each time column applies to the
//! columns to its right until another time column is reached" (p40). One header line names the
//! columns; each row below holds the next sample of every group, so a row's cells for different
//! groups were taken at different times, and a group that has run out leaves its cells empty:
//!
//! ```text
//! time,Gyro X,Gyro Y,Gyro Z,time,pressure,Pressure MSL,Pressure AGL,time,acceleration,…
//! -0.52,0.07,-0.14,0.21,-0.51,97700,302.87,0.17,-0.52,1.0039,…
//! ```
//!
//! The AIM BASE ground station's software writes the same layout of the telemetry it received
//! (p28), with fewer channels at slower, irregular rates (p21, p38) and the radio's `RSSI`.
//!
//! **Recognition.** The first line that isn't blank is comma-separated, starts with a `time`
//! column, names at least two, and names at least one of the AIM's own channels, such as `Pressure
//! AGL`, `lat. X accel.` or `eject. bat. voltage` ([`MARKS`]); names are compared trimmed and
//! without regard to case. A Raven's export starts with `Time@…` columns and names none of them.
//!
//! **Channels.** Each name the reader knows, what it is read as, and where its unit comes from.
//! The column keeps the file's name, trimmed:
//!
//! | Column | Quantity | Unit, and from where |
//! |---|---|---|
//! | `time` | the clock of the columns to its right (p40) | seconds, from `GPS Time of Week` (below) |
//! | `pressure` | [`Quantity::Pressure`] | Pa, from the values (below); the manual's sample export writes 100182 (Table 2, p40) |
//! | `Pressure MSL` | [`Quantity::AltitudeMsl`] | m, from the pressure (below): the software's altitude from "Standard Atmospheric table data" (p22) |
//! | `Pressure AGL` | [`Quantity::AltitudeAboveGround`]: the primary track | as `Pressure MSL` |
//! | `acceleration`, `lat. X accel.`, `lat. Y accel.` | [`Quantity::Acceleration`] | g, × 9.806 65 m/s² (standard gravity): the manual plots accelerations "in G's" (p23–24) |
//! | `Gyro X`, `Gyro Y`, `Gyro Z` | [`Quantity::AngularRate`] | degrees per second (p24) |
//! | `Mag X`, `Mag Y`, `Mag Z` | [`Quantity::Unrecognized`], as written | below |
//! | `eject. bat. voltage`, `comp. bat. voltage`, `A Voltage` … `D Voltage` | [`Quantity::Voltage`] | volts, assumed: the manual plots the batteries' and the four lines' voltages without a unit (p24) |
//! | `temperature` | [`Quantity::Temperature`] | °C or °F, assumed with the altitudes' unit (below) |
//! | `lat`, `long` | [`Quantity::Latitude`], [`Quantity::Longitude`] | decimal degrees, as the manual's sample export writes them (Table 2, p40); a value beyond ±90° or ±180° is a gap |
//! | `GPS MSL` | [`Quantity::GnssAltitude`] | assumed with the altitudes' unit |
//! | `GPS Time of Week` | [`Quantity::Unrecognized`], as written | milliseconds (below) |
//! | `SVs`, `RSSI` | [`Quantity::Number`] | the satellites in the fix (p25), the radio's received strength in dBm (p29) |
//! | `launch detect`, `apogee detect`, `A cont.` … `D cont.`, `A on.` … `D on` | [`Quantity::Number`], 0 or 1 | flags; see Events |
//!
//! A column the reader doesn't know is kept as written, [`Quantity::Unrecognized`], with a note.
//! Each unit gets a note on the record saying how it was read and on what evidence.
//!
//! *The clock.* The manual states no unit for `time`. `GPS Time of Week` counts milliseconds: its
//! values stay below a week's 604,800,000, which seconds would pass, and the manual's sample
//! export counts it up by 5,000 over 5 of its `time` before the GPS has a fix (Table 2, p40). The
//! reader takes the median, over the channel's consecutive samples, of its rise over its clock's:
//! within 10% of 1,000, `time` is in seconds by the file. As of 2026-10-13 it is 969.8 to 1,002.4
//! in the 16 AIM files among the project's sample logs (the BASE's clock, the time a packet came in,
//! is the less regular). A file without the channel, or with another median, has its `time` read
//! in seconds all the same, assumed, with the median in the note. The time's zero is the
//! software's: the start of the recording, or the time the owner chose to make zero, such as the
//! launch (p33).
//!
//! *The pressure.* The manual's software shows the pressure in kPa or inHg (p25), but its sample
//! export writes pascals (Table 2, p40). The reader takes the unit from the first sample, on the
//! pad: pascals if it lies within 50,000 to 110,000, as a launch site's does (the bounds
//! [`crate::rrc3`] argues), kilopascals within 50 to 110, inches of mercury (3,386.389 Pa, NIST
//! SP 811) within 14.8 to 32.5; otherwise the column is kept as written. Every sample file is in
//! whole pascals.
//!
//! *The altitudes.* `Pressure MSL` is the pressure's altitude in the standard atmosphere, and
//! `Pressure AGL` the same less the logger's ground altitude: over the 16 sample files `Pressure
//! MSL` is within 0.06 m of the U.S. Standard Atmosphere's pressure altitude of `pressure`, and
//! `Pressure MSL` less `Pressure AGL` is constant to 0.02 m in each. The software offers metric or
//! imperial units (p33); the reader decides between meters and feet as [`crate::rrc3`] does: from
//! the first sample to the one of lowest pressure, the climb the pressure implies in the U.S.
//! Standard Atmosphere, 1976 ([`hpr_atmos::ussa76`]), against the column's climb read in meters
//! and in feet; the unit within 25% is taken, and a file with neither is refused
//! ([`LogError::Unit`]). Below a 10 m climb, or without a pressure of known unit beside them,
//! meters are assumed, the unit of the manual's sample export and of its calibration (p22). The
//! temperature is assumed in °C beside meters and °F beside feet, and `GPS MSL` in the altitudes'
//! unit, as the software sets one unit system for its displays (p33); the sample files' readings,
//! 25.8 to 50.6, are an electronics bay's in °C.
//!
//! *The magnetometer.* The manual gives tesla (p24), but the field the sample files' columns make,
//! 0.0011 to 0.0022, is 20 to 40 times the Earth's at its strongest, 65 µT, so no source bears a
//! unit out; the record has no quantity for a magnetic field either.
//!
//! **The primary track.** The record's time and altitude are `Pressure AGL`'s, on its own clock:
//! the AIM's barometric altitude above the ground it was calibrated to (p22), the altitude above
//! the pad that the logger itself states. A file without it is refused: working one out of the
//! pressure is a reading, not this reader's to make. The primary track takes its samples in time
//! order, the first at each time, leaving out any sample without a time; a note counts the
//! samples put in order or left out (the channel keeps them all, in the file's order). Debrief's
//! sample, a thinned copy of a flight with one row out of order at its end, has one. Where the
//! altitude above ground reads more than 100 m off zero at the track's first sample, as one of the
//! sample BASE files does where its ground reference came from an altitude far from the pad's, a
//! note says so. No column is a vertical speed, so [`FlightLog::vertical_speed_m_s`] is `None`;
//! the temperature and `comp. bat. voltage` fill [`FlightLog::temperature_k`] and
//! [`FlightLog::battery_v`] only on the altitude's clock, which none of the sample files has.
//!
//! **Events.** `launch detect`, `apogee detect` and the four outputs' `A on.` … `D on` are flags
//! the AIM logs every 100 ms (p2); each time one goes from 0 to 1 is an [`Event`] named by its
//! column, at the sample's time, in time order. A flag already 1 at its first sample is no event.
//! The AIM detects the launch some time after leaving the rail, once its filtered velocity has
//! stayed above 30 m/s for 0.5 s (p36), so `launch detect` is no liftoff time.
//!
//! **Rows one cell too wide.** In 11 of the 13 sample files with GPS columns, every row after the
//! GPS group (`time,lat,long,GPS MSL,SVs`) runs out holds one cell more than the header: the group
//! is written as six empty cells, not five, so the groups to its right sit one cell right of
//! their names. A row one cell wider than the header whose GPS group's cells and the one after
//! them are all empty has that one after dropped, counted in a note. After it, in every row of
//! every sample file, each group's cells are all filled or all empty. Any other row of another
//! width is refused, naming its line, so the record never holds more values than the file has
//! cells.
//!
//! **Cells.** A group with any cell filled in a row is a sample: an empty time or value in it is a
//! gap (`NaN`), counted in a note; a cell that isn't a number is refused, naming its line. Lines may
//! end in CR, LF or both, blank lines are skipped, and a byte-order mark is dropped.
//!
//! **Where this comes from.** The layout follows Debrief's reader (`lib/parsers/entacoreAim.ts`,
//! MIT, the project owner's own; see `THIRD-PARTY-NOTICES.md`), which reads each channel on its
//! own clock and recognizes the file by `Pressure MSL` and `Pressure AGL`; the units are the
//! manual's, and the files', as above. Where this reader departs from Debrief's: it keeps every
//! column on its own clock where Debrief resamples a few onto the densest; it takes the altitude
//! above ground from the export's own column where Debrief works one out from the pressure; it
//! drops the extra cell of a run-out GPS group, which Debrief reads shifted; and it states the
//! flags' rises as events.

use hpr_atmos::ussa76::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Entacore AIM CSV";

/// The name of a clock column.
const CLOCK: &str = "time";

/// Names only the AIM's export writes, trimmed, in lower case and without trailing points: a header
/// must name one of them, as well as two clocks, to be recognized.
pub const MARKS: [&str; 10] = [
    "pressure msl",
    "pressure agl",
    "lat. x accel",
    "lat. y accel",
    "eject. bat. voltage",
    "comp. bat. voltage",
    "launch detect",
    "apogee detect",
    "gps time of week",
    "gps msl",
];

/// A pad's pressure in pascals, the bounds [`crate::rrc3`] argues: the standard atmosphere's about
/// 5,570 m up, above any launch site, to above the highest sea-level pressure on record.
const PAD_PA: (f64, f64) = (50_000.0, 110_000.0);

/// Pascals in a kilopascal.
const KPA_PA: f64 = 1_000.0;

/// Pascals in a conventional inch of mercury (NIST SP 811, 2008, B.8).
const INHG_PA: f64 = 3_386.389;

/// The least climb, m, from which the pressure settles the altitudes' unit: the pressure's step,
/// 1 Pa in every sample file, is 0.08 m of altitude at sea level, under 1% of it.
const SETTLING_CLIMB_M: f64 = 10.0;

/// How far, as a share, an altitude column's climb may be from the pressure's for its unit to be
/// taken: feet and meters are 3.28 times apart, so at most one unit can be within 25%.
const ALTITUDE_MARGIN: f64 = 0.25;

/// `GPS Time of Week`'s rise over a second of `time` if `time` is in seconds: milliseconds.
const TOW_RATE: f64 = 1_000.0;

/// How far, as a share, the median rise may be from [`TOW_RATE`] for `time` to be read in seconds
/// by the file; the BASE's least, as of 2026-10-13, is 3% under.
const TOW_MARGIN: f64 = 0.1;

/// A week in milliseconds: a GPS time of week counted in them stays below it.
const WEEK_MS: f64 = 604_800_000.0;

/// How far, m, the altitude above ground may read from zero at the track's first sample before a
/// note says the logger's ground isn't where the record starts.
const GROUND_OFF_M: f64 = 100.0;

/// The most events a log keeps; past it, a note counts the rest, so a file of flags toggling every
/// row can't fill the record with them.
const MAX_EVENTS: usize = 1_000;

/// Whether a role is one a note is about.
type RoleTest = fn(Role) -> bool;

/// What a column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Pressure,
    AltitudeMsl,
    AltitudeAgl,
    GpsAltitude,
    Temperature,
    Acceleration,
    Gyro,
    Magnetic,
    Voltage,
    Latitude,
    Longitude,
    TimeOfWeek,
    /// A count or a signal strength.
    Number,
    /// A 0-or-1 flag, and whether its rise is an event.
    Flag {
        event: bool,
    },
    Unknown,
}

/// The names the reader knows, as [`key`] gives them, and what each is.
const KNOWN: [(&str, Role); 31] = [
    ("pressure", Role::Pressure),
    ("pressure msl", Role::AltitudeMsl),
    ("pressure agl", Role::AltitudeAgl),
    ("gps msl", Role::GpsAltitude),
    ("temperature", Role::Temperature),
    ("acceleration", Role::Acceleration),
    ("lat. x accel", Role::Acceleration),
    ("lat. y accel", Role::Acceleration),
    ("gyro x", Role::Gyro),
    ("gyro y", Role::Gyro),
    ("gyro z", Role::Gyro),
    ("mag x", Role::Magnetic),
    ("mag y", Role::Magnetic),
    ("mag z", Role::Magnetic),
    ("eject. bat. voltage", Role::Voltage),
    ("comp. bat. voltage", Role::Voltage),
    ("a voltage", Role::Voltage),
    ("b voltage", Role::Voltage),
    ("c voltage", Role::Voltage),
    ("d voltage", Role::Voltage),
    ("lat", Role::Latitude),
    ("long", Role::Longitude),
    ("gps time of week", Role::TimeOfWeek),
    ("svs", Role::Number),
    ("rssi", Role::Number),
    ("launch detect", Role::Flag { event: true }),
    ("apogee detect", Role::Flag { event: true }),
    ("a on", Role::Flag { event: true }),
    ("b on", Role::Flag { event: true }),
    ("c on", Role::Flag { event: true }),
    ("d on", Role::Flag { event: true }),
];

/// A header name trimmed, in lower case, and without trailing points, as [`KNOWN`] and [`MARKS`]
/// list them: the export writes both `A on.` and `B on`.
fn key(name: &str) -> String {
    name.trim()
        .trim_end_matches('.')
        .trim()
        .to_ascii_lowercase()
}

/// What a column is, by its name.
fn role_of(name: &str) -> Role {
    let key = key(name);
    // `A cont.` … `D cont.`: an output's continuity, one letter and `cont`.
    if let Some(letter) = key.strip_suffix(" cont")
        && letter.len() == 1
    {
        return Role::Flag { event: false };
    }
    KNOWN
        .iter()
        .find(|(known, _)| *known == key)
        .map_or(Role::Unknown, |(_, role)| *role)
}

/// The first line of the text that isn't blank, any byte-order mark dropped.
fn first_line(text: &str) -> Option<&str> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    text.split(['\r', '\n'])
        .map(str::trim)
        .find(|line| !line.is_empty())
}

/// Whether a header line is an AIM export's: a `time` column first, at least two, and one of the
/// AIM's own names.
fn is_header(line: &str) -> bool {
    let keys: Vec<String> = line.split(',').map(key).collect();
    keys.first().is_some_and(|first| first == CLOCK)
        && keys.iter().filter(|name| *name == CLOCK).count() >= 2
        && keys.iter().any(|name| MARKS.contains(&name.as_str()))
}

/// Whether the text is this layout: its first line that isn't blank is an AIM export's header.
pub(crate) fn recognizes(text: &str) -> bool {
    first_line(text).is_some_and(is_header)
}

/// One data column of a group.
struct Column {
    name: String,
    role: Role,
    /// The values as written, `NaN` for an empty cell.
    values: Vec<f64>,
    /// Empty cells in samples of the group.
    empty: usize,
}

/// A clock and the columns to its right, up to the next.
struct Group {
    /// The clock's column, 0-based.
    first: usize,
    time_s: Vec<f64>,
    columns: Vec<Column>,
    /// Samples whose time cell is empty.
    empty_times: usize,
}

impl Group {
    /// Cells the group takes in a row: its clock and its columns.
    fn width(&self) -> usize {
        1 + self.columns.len()
    }
}

/// Reads an Entacore AIM CSV export from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't an AIM export's header;
/// [`LogError::Syntax`] for a header with an empty name or a clock without columns, a file without
/// `Pressure AGL`, a row of another width than the header's (but for the GPS group's extra cell),
/// or a cell that isn't a number; [`LogError::Unit`] if the altitudes' climb is in neither meters
/// nor feet against the pressure's; [`LogError::NoData`] if `Pressure AGL` has no sample with a
/// time.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    if !is_header(header) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't an AIM export's header: `time` columns and the \
                 AIM's channels, such as `Pressure AGL`",
                short(header)
            ),
        });
    }
    let mut notes = Notes::default();
    let mut groups = header_groups(header, header_line, &mut notes)?;
    let width: usize = groups.iter().map(Group::width).sum();
    // The GPS group: the one whose first column is `lat`.
    let gps = groups
        .iter()
        .find(|group| group.columns.first().is_some_and(|c| key(&c.name) == "lat"))
        .map(|group| (group.first, group.width()));

    let mut repaired = 0usize;
    let mut first_repaired = None;
    for (number, line) in lines {
        let mut cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if let Some((first, gps_width)) = gps
            && cells.len() == width + 1
            && cells
                .get(first..=first + gps_width)
                .is_some_and(|run| run.iter().all(|cell| cell.is_empty()))
        {
            cells.remove(first + gps_width);
            repaired += 1;
            first_repaired.get_or_insert(number);
        }
        if cells.len() != width {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells where the header names {width}",
                    cells.len()
                ),
            ));
        }
        for group in &mut groups {
            read_sample(group, &cells, number)?;
        }
    }
    let repair_note = first_repaired.map(|line| {
        format!(
            "{repaired} rows, the first on line {line}, write the run-out GPS group, `time,lat,…`, \
             one empty cell too wide: the cell after it was dropped, so that the groups to its \
             right are read under their names"
        )
    });
    build(groups, header_line, notes, repair_note)
}

/// The groups the header lays out: each `time` column and the columns to its right.
fn header_groups(header: &str, line: usize, notes: &mut Notes) -> Result<Vec<Group>, LogError> {
    let mut groups: Vec<Group> = Vec::new();
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        if key(name) == CLOCK {
            if let Some(last) = groups.last()
                && last.columns.is_empty()
            {
                return Err(syntax(
                    line,
                    format!(
                        "the clock in column {} has no column of its own",
                        last.first + 1
                    ),
                ));
            }
            groups.push(Group {
                first: index,
                time_s: Vec::new(),
                columns: Vec::new(),
                empty_times: 0,
            });
            continue;
        }
        // Invariant: the header starts with a clock (`is_header`), so a group is open here.
        let Some(group) = groups.last_mut() else {
            continue;
        };
        let role = role_of(name);
        if role == Role::Unknown {
            notes.push(format!(
                "the column {:?} isn't one this reader knows, and was kept as written",
                short(name)
            ));
        }
        group.columns.push(Column {
            name: name.to_owned(),
            role,
            values: Vec::new(),
            empty: 0,
        });
    }
    if let Some(last) = groups.last()
        && last.columns.is_empty()
    {
        return Err(syntax(
            line,
            format!(
                "the clock in column {} has no column of its own",
                last.first + 1
            ),
        ));
    }
    Ok(groups)
}

/// Reads a group's sample from a row as wide as the header, if the row has one: any of its cells
/// filled.
fn read_sample(group: &mut Group, cells: &[&str], line: usize) -> Result<(), LogError> {
    let Some(sample) = cells.get(group.first..group.first + group.width()) else {
        return Err(syntax(line, "the row ends inside a group".to_owned()));
    };
    if sample.iter().all(|cell| cell.is_empty()) {
        return Ok(());
    }
    let cell_value = |at: usize, name: &str| -> Result<f64, LogError> {
        let cell = sample[at];
        if cell.is_empty() {
            return Ok(f64::NAN);
        }
        number(cell, false).ok_or_else(|| {
            syntax(
                line,
                format!(
                    "{:?} in cell {}, under {:?}, isn't a number",
                    short(cell),
                    group.first + at + 1,
                    short(name)
                ),
            )
        })
    };
    let time_s = cell_value(0, CLOCK)?;
    let mut values = Vec::with_capacity(group.columns.len());
    for (at, column) in group.columns.iter().enumerate() {
        values.push(cell_value(at + 1, &column.name)?);
    }
    if time_s.is_nan() {
        group.empty_times += 1;
    }
    group.time_s.push(time_s);
    for (column, value) in group.columns.iter_mut().zip(values) {
        if value.is_nan() {
            column.empty += 1;
        }
        column.values.push(value);
    }
    Ok(())
}

/// How the altitudes' unit was decided.
enum AltitudeEvidence {
    /// From the pressure's climb in the standard atmosphere and the column's.
    Pressure { pressure_m: f64, column_m: f64 },
    /// Assumed, for the reason given.
    Assumed(&'static str),
}

/// The record from the groups read: units decided, every channel, the primary track, the events
/// and the notes.
fn build(
    groups: Vec<Group>,
    header_line: usize,
    notes: Notes,
    repair_note: Option<String>,
) -> Result<FlightLog, LogError> {
    let mut decisions: Vec<String> = Vec::new();

    // The clock.
    decisions.push(time_note(&groups));

    // The pressure, from its first sample.
    let pressure = groups.iter().find_map(|group| {
        group
            .columns
            .iter()
            .position(|c| c.role == Role::Pressure)
            .map(|at| (group, at))
    });
    let pressure_pa = pressure.and_then(|(group, at)| {
        let pad = group.columns[at]
            .values
            .iter()
            .copied()
            .find(|v| v.is_finite())?;
        let (scale, unit) = pressure_unit(pad)?;
        decisions.push(format!(
            "`{}` read in {unit}, from the values: its first sample, on the pad, is {pad}, a \
             launch site's pressure in {unit} (50,000 to 110,000 Pa)",
            group.columns[at].name
        ));
        Some(scale)
    });
    if let (Some((group, at)), None) = (pressure, pressure_pa) {
        let first = group.columns[at]
            .values
            .iter()
            .copied()
            .find(|v| v.is_finite());
        decisions.push(format!(
            "`{}` kept as written, in a unit its values don't settle: its first sample, {}, is a \
             launch site's pressure in none of pascals, kilopascals or inches of mercury",
            group.columns[at].name,
            first.map_or("empty".to_owned(), |v| v.to_string())
        ));
    }

    // The altitudes, against the pressure.
    let altitude_names = names_of(&groups, |role| {
        matches!(role, Role::AltitudeMsl | Role::AltitudeAgl)
    });
    let (feet, evidence) = altitude_unit(&groups, pressure_pa, header_line)?;
    let (unit, scale) = if feet {
        ("feet", FOOT_M)
    } else {
        ("meters", 1.0)
    };
    if !altitude_names.is_empty() {
        decisions.push(match evidence {
            AltitudeEvidence::Pressure {
                pressure_m,
                column_m,
            } => format!(
                "{} read in {unit}, from the pressure: from the first sample to the one of lowest \
                 pressure, the standard atmosphere climbs {pressure_m:.1} m and the column \
                 {column_m:.1} {unit}",
                listed(&altitude_names)
            ),
            AltitudeEvidence::Assumed(reason) => format!(
                "{} read in meters, assumed: {reason}; the manual's sample export (Table 2, p40) \
                 and its calibration (p22) are in meters",
                listed(&altitude_names)
            ),
        });
    }
    let gps_altitude = names_of(&groups, |role| role == Role::GpsAltitude);
    if !gps_altitude.is_empty() {
        decisions.push(format!(
            "{} read in {unit}, assumed: as the barometric altitudes, the software setting one \
             unit system for its displays (p33)",
            listed(&gps_altitude)
        ));
    }
    let temperature = names_of(&groups, |role| role == Role::Temperature);
    if !temperature.is_empty() {
        let degrees = if feet { "Fahrenheit" } else { "Celsius" };
        decisions.push(format!(
            "{} read in degrees {degrees}, assumed: the manual's sensor reads °C or °F (p25), \
             and the software sets one unit system for its displays (p33), here the altitudes' \
             {unit}",
            listed(&temperature)
        ));
    }
    let fixed: [(RoleTest, &str); 5] = [
        (
            |role| role == Role::Acceleration,
            "read in g, from the manual: it plots the accelerations \"in G's\", the long axis's \
             with gravity in it (p23–24)",
        ),
        (
            |role| role == Role::Gyro,
            "read in degrees per second, from the manual: \"degrees/sec\" (p24)",
        ),
        (
            |role| role == Role::Voltage,
            "read in volts, assumed: the manual plots the batteries' and the four lines' \
             voltages without naming a unit (p24)",
        ),
        (
            |role| matches!(role, Role::Latitude | Role::Longitude),
            "read in degrees, from the values: decimal numbers, as the manual's sample export \
             writes them (Table 2, p40)",
        ),
        (
            |role| role == Role::Magnetic,
            "kept as written, in a unit no source bears out: the manual gives tesla (p24), but \
             the field the sample files' columns make, 0.0011 to 0.0022, is 20 to 40 times the \
             Earth's, and the record has no quantity for a magnetic field",
        ),
    ];
    for (is, how) in fixed {
        let names = names_of(&groups, is);
        if names.is_empty() {
            continue;
        }
        if is(Role::Voltage) {
            // One note a voltage: the files hold different sets of them.
            for name in &names {
                decisions.push(format!("`{name}` {how}"));
            }
        } else {
            decisions.push(format!("{} {how}", listed(&names)));
        }
    }

    let time_of_week = names_of(&groups, |role| role == Role::TimeOfWeek);
    if !time_of_week.is_empty() {
        decisions.push(format!(
            "{} kept as written, in milliseconds: the record has no quantity for a time of the \
             GPS week",
            listed(&time_of_week)
        ));
    }

    let base = groups
        .iter()
        .flat_map(|group| &group.columns)
        .any(|column| key(&column.name) == "rssi");

    // Every channel, in SI.
    let mut channel_notes = notes;
    let mut out_of_range = Vec::new();
    let mut channels = Vec::new();
    let mut events = Vec::new();
    let mut events_left_out = 0usize;
    for group in &groups {
        if group.empty_times > 0 {
            channel_notes.push(format!(
                "the clock in column {} leaves {} samples' times empty: they are gaps",
                group.first + 1,
                group.empty_times
            ));
        }
        for column in &group.columns {
            if column.empty > 0 {
                channel_notes.push(format!(
                    "the column {:?} leaves {} samples' values empty: they are gaps",
                    short(&column.name),
                    column.empty
                ));
            }
            let (quantity, values) = si(column, pressure_pa, scale, feet, &mut out_of_range);
            if let Role::Flag { event: true } = column.role {
                for (pair, time) in column.values.windows(2).zip(group.time_s.iter().skip(1)) {
                    if pair[0] == 0.0 && pair[1] == 1.0 && time.is_finite() {
                        if events.len() < MAX_EVENTS {
                            events.push(Event {
                                time_s: *time,
                                name: column.name.clone(),
                            });
                        } else {
                            events_left_out += 1;
                        }
                    }
                }
            }
            channels.push(Channel {
                column: column.name.clone(),
                quantity,
                time_s: group.time_s.clone(),
                values,
            });
        }
    }
    events.sort_by(|a, b| a.time_s.total_cmp(&b.time_s));
    for (name, count) in out_of_range {
        channel_notes.push(format!(
            "the column {:?} holds {count} values beyond a {}'s range: they are gaps",
            short(&name),
            if name.trim().eq_ignore_ascii_case("lat") {
                "latitude"
            } else {
                "longitude"
            }
        ));
    }
    if events_left_out > 0 {
        channel_notes.push(format!(
            "and {events_left_out} more rises of the flags, left out of the events so that the \
             list stays short"
        ));
    }

    // The primary track: `Pressure AGL`'s samples in time order.
    let Some(primary) = channels
        .iter()
        .position(|c| c.quantity == Quantity::AltitudeAboveGround)
    else {
        return Err(syntax(
            header_line,
            "the file has no `Pressure AGL`, the logger's altitude above the ground, which the \
             record's altitude is"
                .to_owned(),
        ));
    };
    let track = &channels[primary];
    let mut samples: Vec<(f64, f64)> = track
        .time_s
        .iter()
        .zip(&track.values)
        .filter(|(time, _)| time.is_finite())
        .map(|(time, value)| (*time, *value))
        .collect();
    let untimed = track.time_s.len() - samples.len();
    let backwards = samples
        .windows(2)
        .filter(|pair| pair[1].0 < pair[0].0)
        .count();
    samples.sort_by(|a, b| a.0.total_cmp(&b.0));
    let before = samples.len();
    samples.dedup_by(|later, earlier| later.0 == earlier.0);
    let repeats = before - samples.len();
    if samples.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    let mut track_notes = Vec::new();
    if backwards > 0 || repeats > 0 || untimed > 0 {
        track_notes.push(format!(
            "the altitude's clock goes back {backwards} times and repeats a time {repeats} \
             times, and {untimed} samples have no time: the record's track takes the samples in \
             time order, the first at each time, and leaves out those without one; the channel \
             keeps them all"
        ));
    }
    if let Some(&(_, ground)) = samples.iter().find(|(_, value)| value.is_finite())
        && ground.abs() > GROUND_OFF_M
    {
        track_notes.push(format!(
            "`{}` reads {ground:.1} m at the track's first sample: the logger's ground isn't \
             where the record starts",
            track.column
        ));
    }
    if base {
        track_notes.push(
            "the file holds `RSSI`, the radio's strength at the AIM BASE: it is the telemetry \
             the ground station received, at the radio's rates, not the flight computer's own log"
                .to_owned(),
        );
    }
    let (time_s, altitude_m): (Vec<f64>, Vec<f64>) = samples.into_iter().unzip();
    let on_track = |channel: &&Channel| channel.time_s == time_s;
    let temperature_k = channels
        .iter()
        .filter(|c| c.quantity == Quantity::Temperature)
        .find(on_track)
        .map(|c| c.values.clone());
    let battery_v = channels
        .iter()
        .filter(|c| key(&c.column) == "comp. bat. voltage")
        .find(on_track)
        .map(|c| c.values.clone());

    let mut all_notes = decisions;
    all_notes.extend(repair_note);
    all_notes.extend(track_notes);
    all_notes.extend(channel_notes.into_vec());
    Ok(FlightLog {
        format: LogFormat::EntacoreAimCsv,
        logger: "Entacore AIM".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k,
        battery_v,
        channels,
        events,
        notes: all_notes,
    })
}

/// A column's quantity and its values in SI. A latitude or longitude out of range is a gap,
/// counted in `out_of_range` by the column's name.
fn si(
    column: &Column,
    pressure_pa: Option<f64>,
    altitude_scale: f64,
    fahrenheit: bool,
    out_of_range: &mut Vec<(String, usize)>,
) -> (Quantity, Vec<f64>) {
    let scaled = |factor: f64| -> Vec<f64> { column.values.iter().map(|v| v * factor).collect() };
    match column.role {
        Role::Pressure => match pressure_pa {
            Some(factor) => (Quantity::Pressure, scaled(factor)),
            None => (Quantity::Unrecognized, column.values.clone()),
        },
        Role::AltitudeMsl => (Quantity::AltitudeMsl, scaled(altitude_scale)),
        Role::AltitudeAgl => (Quantity::AltitudeAboveGround, scaled(altitude_scale)),
        Role::GpsAltitude => (Quantity::GnssAltitude, scaled(altitude_scale)),
        Role::Temperature => (
            Quantity::Temperature,
            column
                .values
                .iter()
                .map(|v| {
                    if fahrenheit {
                        (v - 32.0) * 5.0 / 9.0 + 273.15
                    } else {
                        v + 273.15
                    }
                })
                .collect(),
        ),
        Role::Acceleration => (Quantity::Acceleration, scaled(STANDARD_GRAVITY_MPS2)),
        Role::Gyro => (Quantity::AngularRate, column.values.clone()),
        Role::Voltage => (Quantity::Voltage, column.values.clone()),
        Role::Latitude | Role::Longitude => {
            let (quantity, limit) = if column.role == Role::Latitude {
                (Quantity::Latitude, 90.0)
            } else {
                (Quantity::Longitude, 180.0)
            };
            let mut count = 0;
            let values = column
                .values
                .iter()
                .map(|&v| {
                    if v.abs() > limit {
                        count += 1;
                        f64::NAN
                    } else {
                        v
                    }
                })
                .collect();
            if count > 0 {
                out_of_range.push((column.name.clone(), count));
            }
            (quantity, values)
        }
        Role::Number | Role::Flag { .. } => (Quantity::Number, column.values.clone()),
        Role::TimeOfWeek | Role::Magnetic | Role::Unknown => {
            (Quantity::Unrecognized, column.values.clone())
        }
    }
}

/// The pressure's unit from its first sample, on the pad: the factor to pascals and the unit's
/// name; `None` if it is a launch site's pressure in none of them.
fn pressure_unit(pad: f64) -> Option<(f64, &'static str)> {
    [
        (1.0, "pascals"),
        (KPA_PA, "kilopascals"),
        (INHG_PA, "inches of mercury"),
    ]
    .into_iter()
    .find(|(factor, _)| (PAD_PA.0..=PAD_PA.1).contains(&(pad * factor)))
}

/// The altitudes' unit, `true` for feet, and on what evidence: the climb of `Pressure MSL`, or
/// else `Pressure AGL`, in the pressure's group, against the pressure's in the U.S. Standard
/// Atmosphere, from the first sample to the one of lowest pressure.
fn altitude_unit(
    groups: &[Group],
    pressure_pa: Option<f64>,
    header_line: usize,
) -> Result<(bool, AltitudeEvidence), LogError> {
    let Some(factor) = pressure_pa else {
        return Ok((
            false,
            AltitudeEvidence::Assumed("the pressure's unit isn't known"),
        ));
    };
    let beside = groups.iter().find_map(|group| {
        let pressure = group.columns.iter().find(|c| c.role == Role::Pressure)?;
        let altitude = group
            .columns
            .iter()
            .find(|c| c.role == Role::AltitudeMsl)
            .or_else(|| group.columns.iter().find(|c| c.role == Role::AltitudeAgl))?;
        Some((pressure, altitude))
    });
    let Some((pressure, altitude)) = beside else {
        return Ok((
            false,
            AltitudeEvidence::Assumed("no pressure is beside them on their clock"),
        ));
    };
    let pairs: Vec<(f64, f64)> = pressure
        .values
        .iter()
        .zip(&altitude.values)
        .filter(|(p, a)| p.is_finite() && *p * factor > 0.0 && a.is_finite())
        .map(|(p, a)| (p * factor, *a))
        .collect();
    let (Some(&first), Some(&lowest)) = (
        pairs.first(),
        pairs.iter().min_by(|a, b| a.0.total_cmp(&b.0)),
    ) else {
        return Ok((
            false,
            AltitudeEvidence::Assumed("no sample holds both the pressure and the altitude"),
        ));
    };
    let atmosphere = Ussa76::standard();
    let climb = atmosphere
        .pressure_altitude_m(lowest.0)
        .and_then(|low| atmosphere.pressure_altitude_m(first.0).map(|pad| low - pad));
    let Ok(pressure_m) = climb else {
        return Ok((
            false,
            AltitudeEvidence::Assumed("the pressure has no altitude in the standard atmosphere"),
        ));
    };
    if pressure_m < SETTLING_CLIMB_M {
        return Ok((
            false,
            AltitudeEvidence::Assumed(
                "the pressure climbs less than 10 m, too little to tell meters from feet",
            ),
        ));
    }
    let column = lowest.1 - first.1;
    let within = |climb_m: f64| (climb_m / pressure_m - 1.0).abs() <= ALTITUDE_MARGIN;
    if within(column) {
        Ok((
            false,
            AltitudeEvidence::Pressure {
                pressure_m,
                column_m: column,
            },
        ))
    } else if within(column * FOOT_M) {
        Ok((
            true,
            AltitudeEvidence::Pressure {
                pressure_m,
                column_m: column,
            },
        ))
    } else {
        Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "`{}` climbs {column:.1} from the first sample to the one of lowest pressure, \
                 where the pressure's climb in the standard atmosphere is {pressure_m:.1} m: \
                 neither meters nor feet within 25%",
                altitude.name
            ),
        })
    }
}

/// The note saying how `time` was read: in seconds, from `GPS Time of Week` if its median rise
/// over its clock's is a millisecond's per second, otherwise assumed.
fn time_note(groups: &[Group]) -> String {
    let tow = groups.iter().find_map(|group| {
        group
            .columns
            .iter()
            .find(|c| c.role == Role::TimeOfWeek)
            .map(|c| (group, c))
    });
    let Some((group, column)) = tow else {
        return "`time` read in seconds, assumed: the file has no `GPS Time of Week` to check it \
                by; the manual's sample export counts that up by 5,000 over 5 of its `time` \
                (Table 2, p40)"
            .to_owned();
    };
    let samples: Vec<(f64, f64)> = group
        .time_s
        .iter()
        .zip(&column.values)
        .filter(|(t, v)| t.is_finite() && v.is_finite())
        .map(|(t, v)| (*t, *v))
        .collect();
    let mut rates: Vec<f64> = samples
        .windows(2)
        .filter(|pair| pair[1].0 > pair[0].0)
        .map(|pair| (pair[1].1 - pair[0].1) / (pair[1].0 - pair[0].0))
        .collect();
    let largest = samples
        .iter()
        .map(|s| s.1)
        .fold(f64::NEG_INFINITY, f64::max);
    if rates.is_empty() {
        return format!(
            "`time` read in seconds, assumed: `{}` has too few samples to check it by",
            column.name
        );
    }
    rates.sort_by(f64::total_cmp);
    let median = rates[rates.len() / 2];
    let milliseconds = largest < WEEK_MS;
    if milliseconds && (median / TOW_RATE - 1.0).abs() <= TOW_MARGIN {
        format!(
            "`time` read in seconds, from `{}`: it counts milliseconds, below a week's \
             604,800,000, and climbs by a median of {median:.1} per unit of its `time`",
            column.name
        )
    } else {
        format!(
            "`time` read in seconds, assumed: `{}` climbs by a median of {median:.1} per unit of \
             its `time`{}, not the 1,000 of milliseconds per second",
            column.name,
            if milliseconds {
                ""
            } else {
                ", and passes a week's milliseconds"
            }
        )
    }
}

/// The distinct names of the columns whose role is as asked, in the file's order.
fn names_of(groups: &[Group], is: impl Fn(Role) -> bool) -> Vec<String> {
    // Keyed by the place of the name's key in `KNOWN`, the first name written for each kept, so
    // the same columns give the same note whatever their order, and a header can't list a name
    // more than once a key.
    let mut names: Vec<(usize, String)> = Vec::new();
    for column in groups.iter().flat_map(|group| &group.columns) {
        let key = key(&column.name);
        let Some(place) = KNOWN.iter().position(|(known, _)| *known == key) else {
            continue;
        };
        if is(column.role) && !names.iter().any(|(at, _)| *at == place) {
            names.push((place, column.name.clone()));
        }
    }
    names.sort_by_key(|(place, _)| *place);
    names.into_iter().map(|(_, name)| name).collect()
}

/// Names in backticks, as a list: `` `a` ``, `` `a` and `b` ``, `` `a`, `b` and `c` ``.
fn listed(names: &[String]) -> String {
    let quoted: Vec<String> = names.iter().map(|name| format!("`{name}`")).collect();
    match quoted.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
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

    const HEADER: &str = "time,Gyro X,time,pressure,Pressure MSL,Pressure AGL,time,launch detect,\
                          apogee detect,A on.,time,GPS Time of Week";

    /// The pressure, in pascals, at an altitude in the standard atmosphere's troposphere: the
    /// altimeter formula inverted.
    fn pressure_at(altitude_m: f64) -> f64 {
        101_325.0 * (1.0 - altitude_m / 44_330.8).powf(1.0 / 0.190_263)
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-9 * b.abs().max(1.0)
    }

    /// An invented flight: a pad 300 m up, a climb to 400 m above it, each group on its clock.
    fn flight(altitude_unit: f64) -> String {
        let mut text = format!("{HEADER}\r\n");
        let heights = [0.0, 50.0, 200.0, 400.0, 300.0, 100.0];
        for (row, height) in heights.iter().enumerate() {
            let t = row as f64 - 1.0;
            let p = pressure_at(300.0 + height).round();
            let msl = 44_330.8 * (1.0 - (p / 101_325.0).powf(0.190_263));
            let agl = msl - 300.0;
            let launch = u8::from(row >= 1);
            let apogee = u8::from(row >= 4);
            text.push_str(&format!(
                "{:.2},{},{:.2},{p},{:.3},{:.3},{:.2},{launch},{apogee},{apogee},{:.2},{}\r\n",
                t - 0.01,
                row as f64 * 0.5,
                t,
                msl / altitude_unit,
                agl / altitude_unit,
                t + 0.01,
                t + 0.02,
                500_000_000.0 + (t + 0.02) * 1_000.0
            ));
        }
        text
    }

    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
        matching[0]
    }

    #[test]
    fn recognizes_the_aim_header_and_nothing_like_it() {
        assert!(recognizes(&format!("\u{feff}\n{HEADER}\n")));
        assert!(recognizes(
            "TIME,lat. X accel.,lat. Y accel.,Time,acceleration\n"
        ));
        for other in [
            // One clock only.
            "time,Pressure AGL\n",
            // Two clocks, but none of the AIM's names.
            "time,pressure,time,altitude\n",
            "time,state,state_name,source,pressure,altitude,height,acceleration,speed\n",
            // A Raven's export.
            "Time@Axial Accel (Gs),Axial Accel (Gs),Time@Baro (Atm),Baro (Atm)\n",
            // Not first.
            "Gyro X,time,Pressure AGL,time,pressure\n",
            "",
        ] {
            assert!(!recognizes(other), "{other:?}");
        }
    }

    /// Each group on its own clock, in SI; the primary track `Pressure AGL`'s; meters from the
    /// pressure, seconds from `GPS Time of Week`; the flags' rises as events.
    #[test]
    fn reads_each_group_on_its_clock_in_si() {
        let log = read(&flight(1.0)).unwrap();
        assert_eq!(log.format, LogFormat::EntacoreAimCsv);
        let columns: Vec<(&str, Quantity)> = log
            .channels
            .iter()
            .map(|c| (c.column.as_str(), c.quantity))
            .collect();
        assert_eq!(
            columns,
            [
                ("Gyro X", Quantity::AngularRate),
                ("pressure", Quantity::Pressure),
                ("Pressure MSL", Quantity::AltitudeMsl),
                ("Pressure AGL", Quantity::AltitudeAboveGround),
                ("launch detect", Quantity::Number),
                ("apogee detect", Quantity::Number),
                ("A on.", Quantity::Number),
                ("GPS Time of Week", Quantity::Unrecognized),
            ]
        );
        assert_eq!(log.time_s, [-1.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
        assert!(close(log.channels[0].time_s[0], -1.01));
        assert!(log.altitude_m[3] > 399.0 && log.altitude_m[3] < 401.0);
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [
                (0.01, "launch detect"),
                (3.01, "apogee detect"),
                (3.01, "A on.")
            ]
        );
        assert!(
            note(
                &log,
                "`Pressure MSL` and `Pressure AGL` read in meters, from the pressure: "
            )
            .contains("climbs 400.0 m")
        );
        note(&log, "`time` read in seconds, from `GPS Time of Week`: ");
        note(&log, "`pressure` read in pascals, from the values: ");
        note(
            &log,
            "`Gyro X` read in degrees per second, from the manual: ",
        );
        assert!(log.temperature_k.is_none() && log.battery_v.is_none());
    }

    /// The same flight with its altitudes in feet reads the same heights; one in neither unit is
    /// refused.
    #[test]
    fn the_altitudes_unit_is_decided_from_the_pressure_both_ways() {
        let meters = read(&flight(1.0)).unwrap();
        let feet = read(&flight(FOOT_M)).unwrap();
        for (m, f) in meters.altitude_m.iter().zip(&feet.altitude_m) {
            assert!((m - f).abs() < 1e-3, "{m} {f}");
        }
        note(
            &feet,
            "`Pressure MSL` and `Pressure AGL` read in feet, from the pressure: ",
        );
        let error = read(&flight(0.5)).unwrap_err();
        let LogError::Unit { message, .. } = &error else {
            panic!("{error:?}");
        };
        assert!(message.contains("neither meters nor feet"), "{message}");
    }

    #[test]
    fn a_pressure_unit_comes_from_the_pad_or_is_left_as_written() {
        assert_eq!(pressure_unit(97_000.0), Some((1.0, "pascals")));
        assert_eq!(pressure_unit(97.0), Some((KPA_PA, "kilopascals")));
        assert_eq!(pressure_unit(28.6), Some((INHG_PA, "inches of mercury")));
        for pad in [970.0, 5.0, 300_000.0, 40.0] {
            assert_eq!(pressure_unit(pad), None, "{pad}");
        }
        let text = "time,pressure,Pressure AGL,time,launch detect\n0,970,0,0,0\n1,960,90,1,1\n";
        let log = read(text).unwrap();
        assert_eq!(log.channels[0].quantity, Quantity::Unrecognized);
        note(
            &log,
            "`pressure` kept as written, in a unit its values don't settle: ",
        );
        note(
            &log,
            "`Pressure AGL` read in meters, assumed: the pressure's unit isn't known",
        );
    }

    #[test]
    fn time_is_seconds_by_the_time_of_week_or_assumed() {
        let by_tow = read(&flight(1.0)).unwrap();
        assert!(note(&by_tow, "`time` read in seconds, from ").contains("median of 1000.0"));
        // A time of week rising by 1 per unit of `time`: `time` would be in milliseconds.
        let text: String = flight(1.0)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                if row == 0 {
                    format!("{line}\n")
                } else {
                    let (head, _) = line.rsplit_once(',').unwrap();
                    format!("{head},{}\n", 1_000 + row)
                }
            })
            .collect();
        let log = read(&text).unwrap();
        assert!(note(&log, "`time` read in seconds, assumed: ").contains("median of 1.0"));
        let without =
            read("time,pressure,Pressure AGL,time,launch detect\n0,97000,0,0,0\n").unwrap();
        note(&without, "`time` read in seconds, assumed: the file has no");
    }

    /// The GPS group's extra empty cell is dropped only where the GPS group is empty; any other
    /// row of another width is refused.
    #[test]
    fn a_run_out_gps_group_s_extra_cell_is_dropped() {
        let header = "time,lat,long,GPS MSL,SVs,time,pressure,Pressure AGL";
        let text = format!(
            "{header}\n0,34.5,-117.25,850,8,0,97000,0\n,,,,,,1,96000,85\n,,,,,,2,95000,170\n"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.time_s, [0.0, 1.0, 2.0]);
        assert_eq!(log.channels[0].time_s, [0.0]);
        note(
            &log,
            "2 rows, the first on line 3, write the run-out GPS group",
        );
        // One cell too many where the GPS group isn't empty.
        let wide = format!("{header}\n0,34.5,-117.25,850,8,0,97000,0,\n");
        let error = read(&wide).unwrap_err();
        assert!(
            matches!(error, LogError::Syntax { line: 2, .. }),
            "{error:?}"
        );
        // Without the extra cell, the same rows read the same.
        let narrow = format!("{header}\n0,34.5,-117.25,850,8,0,97000,0\n,,,,,1,96000,85\n");
        let log = read(&narrow).unwrap();
        assert_eq!(log.time_s, [0.0, 1.0]);
        assert!(log.notes.iter().all(|n| !n.contains("run-out")));
    }

    #[test]
    fn the_track_is_put_in_time_order_and_the_channel_kept_as_written() {
        let text = "time,pressure,Pressure AGL,time,launch detect\n\
                    0,97000,0,0,0\n2,96800,17,1,1\n1,96900,8,,\n2,96800,99,,\n,96700,25,,\n";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [0.0, 1.0, 2.0]);
        assert_eq!(log.altitude_m, [0.0, 8.0, 17.0]);
        let agl = &log.channels[1];
        assert_eq!(agl.time_s.len(), 5);
        assert!(agl.time_s[4].is_nan());
        note(
            &log,
            "the altitude's clock goes back 1 times and repeats a time 1 times, and 1",
        );
        note(&log, "the clock in column 1 leaves 1 samples' times empty");
    }

    #[test]
    fn a_ground_far_off_zero_and_a_base_file_are_noted() {
        let text =
            "time,pressure,Pressure AGL,time,RSSI\n0,97000,-2500,0,-90\n1,96000,-2415,1,-95\n";
        let log = read(text).unwrap();
        note(
            &log,
            "`Pressure AGL` reads -2500.0 m at the track's first sample",
        );
        note(&log, "the file holds `RSSI`");
        let log = read("time,pressure,Pressure AGL,time,RSSI\n0,97000,-50,0,-90\n").unwrap();
        assert!(
            log.notes
                .iter()
                .all(|n| !n.contains("track's first sample"))
        );
    }

    #[test]
    fn malformed_files_are_refused_naming_the_line() {
        let cases = [
            ("time,Pressure AGL,time\n", 1),
            ("time,Pressure AGL,,time,x\n", 1),
            ("time,time,Pressure AGL\n", 1),
            ("time,Pressure AGL,time,x\n0,1,0\n", 2),
            ("time,Pressure AGL,time,x\n0,1,0,abc\n", 2),
            ("time,Gyro X,time,launch detect\n0,1,0,0\n", 1),
        ];
        for (text, line) in cases {
            match read(text) {
                Err(LogError::Syntax { line: at, .. }) => assert_eq!(at, line, "{text:?}"),
                other => panic!("{text:?}: {other:?}"),
            }
        }
        assert!(matches!(
            read("time,Pressure AGL,time,x\n,,,1\n"),
            Err(LogError::NoData { .. })
        ));
        assert!(matches!(read(""), Err(LogError::NotThisFormat { .. })));
    }

    #[test]
    fn unknown_columns_and_out_of_range_positions_are_kept_and_noted() {
        let text = "time,Pressure AGL,widget,time,lat,long\n0,0,7,0,91,-117\n1,5,,1,34,200\n";
        let log = read(text).unwrap();
        assert_eq!(log.channels[1].quantity, Quantity::Unrecognized);
        assert!(log.channels[1].values[1].is_nan());
        assert!(log.channels[2].values[0].is_nan() && log.channels[2].values[1] == 34.0);
        assert!(log.channels[3].values[1].is_nan());
        note(&log, "the column \"widget\" isn't one this reader knows");
        note(&log, "the column \"widget\" leaves 1 samples' values empty");
        note(
            &log,
            "the column \"lat\" holds 1 values beyond a latitude's range",
        );
        note(
            &log,
            "the column \"long\" holds 1 values beyond a longitude's range",
        );
    }

    #[test]
    fn continuity_flags_are_numbers_and_not_events() {
        assert_eq!(role_of("A cont."), Role::Flag { event: false });
        assert_eq!(role_of(" B on"), Role::Flag { event: true });
        assert_eq!(role_of("cont"), Role::Unknown);
        assert_eq!(role_of("lat. X accel."), Role::Acceleration);
        let log = read("time,Pressure AGL,time,A cont.\n0,0,0,0\n1,9,1,1\n").unwrap();
        assert!(log.events.is_empty());
    }

    #[test]
    fn the_temperature_follows_the_altitudes_unit() {
        let header = "time,pressure,Pressure MSL,Pressure AGL,time,temperature";
        let mut text = format!("{header}\n");
        for (row, height) in [0.0, 200.0].iter().enumerate() {
            let p = pressure_at(300.0 + height);
            text.push_str(&format!(
                "{row},{p},{},{},{row},68\n",
                (300.0 + height) / FOOT_M,
                height / FOOT_M
            ));
        }
        let log = read(&text).unwrap();
        note(&log, "`temperature` read in degrees Fahrenheit, assumed: ");
        assert!(close(log.channels[3].values[0], 293.15));
        let celsius = read(&flight(1.0).replace("Gyro X", "temperature")).unwrap();
        note(&celsius, "`temperature` read in degrees Celsius, assumed: ");
        assert!(close(celsius.channels[0].values[1], 0.5 + 273.15));
    }

    #[test]
    fn listed_names_read_as_a_sentence() {
        let names = |n: &[&str]| n.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        assert_eq!(listed(&names(&["a"])), "`a`");
        assert_eq!(listed(&names(&["a", "b"])), "`a` and `b`");
        assert_eq!(listed(&names(&["a", "b", "c"])), "`a`, `b` and `c`");
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a strictly increasing
        /// primary track, and every channel as many times as values.
        #[test]
        fn any_text_is_read_or_refused(
            text in "time,pressure,Pressure AGL,(time,lat,long,)?time,[a-z ]{0,6}\n\
                     ([-0-9.,e ]{0,24}\n){0,12}"
        ) {
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
            }
        }
    }
}
