//! Entacore's AIM flight computers' logs, as the AIM XTRA and AIM BASE software exports a flight
//! with *Export to Excel…*: a CSV file (*AIM XTRA/BASE v4.x.x Manual*, Entacore, 2 February 2025,
//! p38).
//!
//! The AIM XTRA samples its sensors at different rates: its barometer every 20 ms, its 100 g
//! accelerometer, 16 g three-axis accelerometer and gyroscope every 10 ms, its magnetometer every
//! 100 ms, its temperature every 500 ms and its GPS's position every 200 ms (manual p2), each
//! written at a whole multiple of its period that the owner sets for each phase of the flight
//! (p21). So the export gives each group of channels its own clock: "there are multiple time
//! columns. Each time column applies to the columns to its right until another time column is
//! reached" (p40). One header line names the columns; each row below holds the next sample of
//! every group, so a row's cells for different groups were taken at different times, and a group
//! that has run out leaves its cells empty:
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
//! AGL`, `lat. X accel.` or `eject. bat. voltage` (the reader's `MARKS`); names are compared
//! trimmed, without trailing points and without regard to case. A Raven's export starts with
//! `Time@…` columns and names none of them.
//!
//! **Channels.** Each name the reader knows, what it is read as, and where its unit comes from.
//! The column keeps the file's name, trimmed. A header naming a column twice is refused:
//!
//! | Column | Quantity | Unit, and from where |
//! |---|---|---|
//! | `time` | the clock of the columns to its right (p40) | seconds or milliseconds, from `GPS Time of Week` or the step (below) |
//! | `pressure` | [`Quantity::Pressure`] | Pa, kPa or inHg, from the values (below); the manual's sample export writes 100182 (Table 2, p40) |
//! | `Pressure MSL` | [`Quantity::AltitudeMsl`] | m or ft, from the pressure (below): the software's altitude from "Standard Atmospheric table data" (p22) |
//! | `Pressure AGL` | [`Quantity::AltitudeAboveGround`]: the primary track | as `Pressure MSL` |
//! | `acceleration`, `lat. X accel.`, `lat. Y accel.` | [`Quantity::Acceleration`] | g, × 9.806 65 m/s² (standard gravity): the manual plots accelerations "in G's" (p23–24) |
//! | `Gyro X`, `Gyro Y`, `Gyro Z` | [`Quantity::AngularRate`] | degrees per second (p24) |
//! | `Mag X`, `Mag Y`, `Mag Z` | [`Quantity::Unrecognized`], as written | below |
//! | `eject. bat. voltage`, `comp. bat. voltage`, `A Voltage` … `D Voltage` | [`Quantity::Voltage`] | volts, from the values (below) |
//! | `temperature` | [`Quantity::Temperature`] | °C or °F, from the altitudes' unit and the manual (below) |
//! | `lat`, `long` | [`Quantity::Latitude`], [`Quantity::Longitude`] | decimal degrees, as the manual's sample export writes them (Table 2, p40); a value beyond ±90° or ±180° is a gap |
//! | `GPS MSL` | [`Quantity::GnssAltitude`] | the altitudes' unit, if it agrees with `Pressure MSL` (below) |
//! | `GPS Time of Week` | [`Quantity::Unrecognized`], as written | milliseconds, where the clock is decided by it (below) |
//! | `SVs` | [`Quantity::Number`] | the satellites used in the GPS's solution (p25) |
//! | `RSSI` | [`Quantity::Number`], as written | dBm, the strength of the radio signal the AIM BASE received (p29), not an SI unit |
//! | `launch detect`, `apogee detect`, `A cont.` … `D cont.`, `A on.` … `D on` | [`Quantity::Number`], 0 or 1 | flags; see Events |
//!
//! A column the reader doesn't know is kept as written, [`Quantity::Unrecognized`], with a note.
//! Each unit gets a note on the record saying how it was read and on what evidence; a unit no
//! evidence settles is kept as written, [`Quantity::Unrecognized`], and the note says why.
//!
//! *The clock.* The manual states no unit for `time`. `GPS Time of Week` counts milliseconds: its
//! values stay below a week's 604,800,000, and the manual's sample export counts it up by 5,000
//! over 5 of its `time` in seconds (Table 2, p40). The reader takes the median, over the
//! channel's consecutive samples, of its rise over its clock's: within 10% of 1,000, `time` is in
//! seconds; within 10% of 1, in milliseconds, and every clock is read × 0.001. As of 2026-10-13
//! the median is 969.8 to 1,002.4 in the 16 AIM files among the project's sample logs (the BASE's
//! clock, the time a packet came in, is the less regular). Without such a channel, the unit comes
//! from the step, as the Eggtimer reader's does: the AIM writes each sensor every 10 to 500 ms
//! (p2) times a whole multiple the owner sets (p21), so the fastest clock's median step reads
//! 0.009 to 2 in seconds (the 10 ms period less 10%, to 2 s; 0.0097 to 0.773 in the sample
//! files) and 9 to 2,000 in milliseconds. A step in neither range, or no clock with two samples,
//! is refused ([`LogError::Unit`]). The time's zero is the software's: the start of the
//! recording, or the time the owner chose to make zero, such as the launch (p33).
//!
//! *The pressure.* The manual's software shows the pressure in kPa or inHg (p25), but its sample
//! export writes pascals (Table 2, p40). The reader takes the unit from the first value that is
//! a launch site's pressure in one of them, on the pad, so a placeholder ahead of it is passed
//! over: pascals within 50,000 to 110,000 (the bounds [`crate::rrc3`] argues), kilopascals
//! within 50 to 110, inches of mercury (3,386.389 Pa, NIST SP 811) within 14.8 to 32.5;
//! otherwise the column is kept as written. Every sample file is in whole pascals.
//!
//! *The altitudes.* `Pressure MSL` is the pressure's altitude in the standard atmosphere, and
//! `Pressure AGL` the same less the logger's ground altitude: over the 16 sample files `Pressure
//! MSL` is within 0.06 m of the U.S. Standard Atmosphere's pressure altitude of `pressure`, and
//! `Pressure MSL` less `Pressure AGL` is constant to 0.02 m in each. The software offers metric or
//! imperial units (p33); the reader decides between meters and feet as [`crate::rrc3`] does: from
//! the pressure's pad sample to the one of lowest pressure, the climb the pressure implies in the
//! U.S. Standard Atmosphere, 1976 ([`hpr_atmos::ussa76`]), against the column's climb read in
//! meters and in feet; the unit within 25% is taken. A climb under 2 m (where the pressure's
//! 1 Pa step, 0.08 m at sea level, is 4% of it), a file without a pressure of known unit beside
//! the altitudes on their clock, or a climb in neither unit is refused ([`LogError::Unit`]): the
//! primary track has no unit to be read in.
//!
//! *The temperature* is read in °C beside meters and °F beside feet: the software sets one unit
//! system, metric or imperial, for all its displays (p33), and its sensor reads °C or °F (p25).
//! Its first value must also be one an electronics bay can have in that unit, −40 to 60 °C
//! (−40 to 140 °F), or the column is kept as written; the sample files' first values, 27.4 to
//! 50.6, are in °C.
//!
//! *The voltages* are read in volts where every value lies within 0 to 20: the computer's battery
//! is 3.5 to 8.4 V and the ejection battery at most 10 V (p10), and the lines' readings sit
//! between, where millivolts would be thousands. A column with a value outside is kept as
//! written. The sample files' values are 0.05 to 8.33.
//!
//! *`GPS MSL`* is read in the altitudes' unit if it agrees with `Pressure MSL`: over the GPS
//! samples within 1 s of a barometric one, the median difference is within 300 m in that unit
//! and beyond 300 m in the other. 300 m is the error a day's weather gives a standard-atmosphere
//! altitude when the sea-level pressure is 36 hPa off the standard 1,013.25 (8.4 m a hPa near sea
//! level); the manual's own sample export reads 283 m by GPS beside 95.6 m by pressure (Table 2,
//! p40), and the sample files' medians are 28 to 173 m. Otherwise, as at a site too low for the
//! units' factor of 3.28 to show, the column is kept as written.
//!
//! *The magnetometer.* The manual gives tesla (p24), but the field the sample files' three
//! columns make, 0.0011 to 0.0022 in the 9 with them, would be 17 to 34 times the Earth's at its
//! strongest, about 65 µT, so no source bears a unit out; the record has no quantity for a
//! magnetic field either, and each file's note gives its own range.
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
//! them are all empty has that one after dropped, counted in a note, if every group's cells are
//! then all filled or all empty, as they are in every such row of every sample file; otherwise
//! the extra cell is somewhere else, and the row is refused, naming its line. Any other row of
//! another width is refused too, so the record never holds more values than the file has cells.
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

use std::collections::BTreeSet;

use hpr_atmos::ussa76::Ussa76;
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, finite, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Entacore AIM CSV";

/// The name of a clock column.
const CLOCK: &str = "time";

/// Names only the AIM's export writes, trimmed, in lower case and without trailing points: a header
/// must name one of them, as well as two clocks, to be recognized.
pub(crate) const MARKS: &[&str] = &[
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
/// 1 Pa in every sample file, is 0.08 m of altitude at sea level, 4% of it and 1% of an 8 m hop,
/// inside [`ALTITUDE_MARGIN`]; and the altitude columns are worked out from the same samples, so
/// the sensor's noise moves both climbs alike.
const SETTLING_CLIMB_M: f64 = 2.0;

/// How far, as a share, an altitude column's climb may be from the pressure's for its unit to be
/// taken: feet and meters are 3.28 times apart, so at most one unit can be within 25%.
const ALTITUDE_MARGIN: f64 = 0.25;

/// `GPS Time of Week`'s rise over a second of `time` if `time` is in seconds: milliseconds.
const TOW_RATE: f64 = 1_000.0;

/// How far, as a share, the median rise may be from [`TOW_RATE`] (seconds) or 1 (milliseconds)
/// for `time`'s unit to be taken from it; the BASE's least, as of 2026-10-13, is 3% under 1,000.
const TOW_MARGIN: f64 = 0.1;

/// A week in milliseconds: a GPS time of week counted in them stays below it.
const WEEK_MS: f64 = 604_800_000.0;

/// The fastest clock's median step, as written, read as seconds: the AIM's fastest period, 10 ms
/// (manual p2), less 10%, to 2 s, its sensors' periods times the whole multiples the owner sets
/// (p21). The 16 sample files' are 0.0097 to 0.773.
const STEP_SECONDS: (f64, f64) = (0.009, 2.0);

/// The same steps as [`STEP_SECONDS`], written in milliseconds.
const STEP_MILLISECONDS: (f64, f64) = (9.0, 2_000.0);

/// The most a voltage column may read, in volts, to be read in them: twice the ejection
/// battery's 10 V cap, the computer's battery being 3.5 to 8.4 V (manual p10).
const VOLTS_MAX: f64 = 20.0;

/// The temperatures, °C, an electronics bay on a pad can have: a winter launch to a bay in the
/// desert sun.
const BAY_C: (f64, f64) = (-40.0, 60.0);

/// How far, m, `GPS MSL` may read from `Pressure MSL` in the same unit: the error of a
/// standard-atmosphere altitude when a day's sea-level pressure is 36 hPa off the standard
/// 1,013.25, at 8.4 m a hPa near sea level.
const GPS_AGREEMENT_M: f64 = 300.0;

/// How far apart in time, s, a GPS and a barometric sample may be to be compared.
const PAIR_S: f64 = 1.0;

/// The Earth's magnetic field at its strongest, about 65 µT, in tesla.
const EARTH_FIELD_T: f64 = 65e-6;

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
    /// The satellites used in the GPS's solution.
    Satellites,
    /// The radio's received strength at the AIM BASE, dBm.
    SignalStrength,
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
    ("svs", Role::Satellites),
    ("rssi", Role::SignalStrength),
    ("launch detect", Role::Flag { event: true }),
    ("apogee detect", Role::Flag { event: true }),
    ("a on", Role::Flag { event: true }),
    ("b on", Role::Flag { event: true }),
    ("c on", Role::Flag { event: true }),
    ("d on", Role::Flag { event: true }),
];

/// A header name trimmed and without trailing points, its case kept.
fn bare(name: &str) -> &str {
    name.trim().trim_end_matches('.').trim()
}

/// A header name as [`KNOWN`] and [`MARKS`] list them: [`bare`], in lower case. The export writes
/// both `A on.` and `B on`.
fn key(name: &str) -> String {
    bare(name).to_ascii_lowercase()
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
/// AIM's own names. The names are compared in place, and the walk stops once both are seen, so a
/// long first line costs no copy of each cell.
fn is_header(line: &str) -> bool {
    let mut names = line.split(',').map(bare);
    if !names
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case(CLOCK))
    {
        return false;
    }
    let mut clocks = 1usize;
    let mut marked = false;
    for name in names {
        if name.eq_ignore_ascii_case(CLOCK) {
            clocks += 1;
        } else if !marked && MARKS.iter().any(|mark| name.eq_ignore_ascii_case(mark)) {
            marked = true;
        }
        if clocks >= 2 && marked {
            return true;
        }
    }
    false
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
    /// Whether no evidence settled the unit its role would be read in, so the column is kept as
    /// written, [`Quantity::Unrecognized`].
    as_written: bool,
}

/// A clock and the columns to its right, up to the next.
struct Group {
    /// The clock's column, 0-based.
    first: usize,
    /// The times, as written until the clock's unit is decided, then in seconds.
    time_s: Vec<f64>,
    columns: Vec<Column>,
    /// The line each sample was read from, one to a time.
    lines: Vec<usize>,
    /// Samples whose time cell is empty.
    empty_times: usize,
}

impl Group {
    /// Cells the group takes in a row: its clock and its columns.
    fn width(&self) -> usize {
        1 + self.columns.len()
    }

    /// Whether the row holds some of the group's cells filled and some empty.
    fn part_filled(&self, cells: &[&str]) -> bool {
        cells
            .get(self.first..self.first + self.width())
            .is_some_and(|run| {
                run.iter().any(|cell| cell.is_empty()) && run.iter().any(|cell| !cell.is_empty())
            })
    }
}

/// Reads an Entacore AIM CSV export from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't an AIM export's header;
/// [`LogError::Syntax`] for a header with an empty name, a name given twice or a clock without
/// columns, a file without `Pressure AGL`, a row of another width than the header's (but for the
/// GPS group's extra cell), or a cell that isn't a number; [`LogError::NoData`] if `Pressure AGL`
/// has no sample with a time; [`LogError::Unit`] if neither `GPS Time of Week` nor the clocks'
/// step settles the clock's unit, or the pressure doesn't settle the altitudes' (module docs).
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
            // Dropped there, the cell must leave every group whole, as the export writes them;
            // otherwise the extra cell is elsewhere, and the row is refused as any row of another
            // width is.
            if let Some(group) = groups.iter().find(|group| group.part_filled(&cells)) {
                return Err(syntax(
                    number,
                    format!(
                        "the row has {} cells where the header names {width}. Dropping the \
                         cell after its empty GPS group would leave the group in columns {} to {} \
                         part filled",
                        width + 1,
                        group.first + 1,
                        group.first + group.width()
                    ),
                ));
            }
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
             one empty cell too wide. The cell after it was dropped, so that the groups to its \
             right are read under their names"
        )
    });
    build(groups, header_line, notes, repair_note)
}

/// The groups the header lays out: each `time` column and the columns to its right.
fn header_groups(header: &str, line: usize, notes: &mut Notes) -> Result<Vec<Group>, LogError> {
    let mut groups: Vec<Group> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        let name_key = key(name);
        if name_key == CLOCK {
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
                lines: Vec::new(),
                empty_times: 0,
            });
            continue;
        }
        if !seen.insert(name_key) {
            return Err(syntax(
                line,
                format!(
                    "the header names {:?} twice, in column {} and before",
                    short(name),
                    index + 1
                ),
            ));
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
            as_written: false,
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
    group.lines.push(line);
    for (column, value) in group.columns.iter_mut().zip(values) {
        if value.is_nan() {
            column.empty += 1;
        }
        column.values.push(value);
    }
    Ok(())
}

/// The place, group and column, of the first column with the role.
fn locate(groups: &[Group], role: Role) -> Option<(usize, usize)> {
    groups.iter().enumerate().find_map(|(at, group)| {
        group
            .columns
            .iter()
            .position(|c| c.role == role)
            .map(|column| (at, column))
    })
}

/// The record from the groups read: units decided, every channel, the primary track, the events
/// and the notes.
fn build(
    mut groups: Vec<Group>,
    header_line: usize,
    notes: Notes,
    repair_note: Option<String>,
) -> Result<FlightLog, LogError> {
    // The primary track's column, and a sample of it with a time, before any unit is decided.
    let Some((agl_group, _)) = locate(&groups, Role::AltitudeAgl) else {
        return Err(syntax(
            header_line,
            "the file has no `Pressure AGL`, the logger's altitude above the ground, which the \
             record's altitude is"
                .to_owned(),
        ));
    };
    if !groups[agl_group].time_s.iter().any(|t| t.is_finite()) {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut decisions: Vec<String> = Vec::new();

    // The clock, applied to every group.
    let clock = clock(&groups, header_line)?;
    for group in &mut groups {
        for time in &mut group.time_s {
            *time *= clock.to_s;
        }
    }
    decisions.push(clock.note);

    // The pressure, from its first value that is a pad's.
    let pressure = locate(&groups, Role::Pressure);
    let mut pressure_pa: Option<(f64, usize)> = None;
    if let Some((group, at)) = pressure {
        let column = &groups[group].columns[at];
        let pad = column
            .values
            .iter()
            .enumerate()
            .find_map(|(index, &value)| pressure_unit(value).map(|unit| (index, value, unit)));
        match pad {
            Some((index, value, (factor, unit))) => {
                decisions.push(format!(
                    "`{}` read in {unit}, from the values: its first value that is a launch \
                     site's pressure, {value}{}, is one in {unit}. A launch site's pressure is \
                     50,000 to 110,000 Pa, in pascals, kilopascals or inches of mercury",
                    column.name,
                    if index > 0 {
                        format!(", after {index} samples that aren't")
                    } else {
                        String::new()
                    }
                ));
                pressure_pa = Some((factor, index));
            }
            None => decisions.push(format!(
                "`{}` kept as written, in a unit its values don't settle: none of them is a \
                 launch site's pressure. A launch site's pressure is 50,000 to 110,000 Pa, in \
                 pascals, kilopascals or inches of mercury",
                column.name
            )),
        }
    }

    // The altitudes, against the pressure.
    let altitude_names = names_of(&groups, |role| {
        matches!(role, Role::AltitudeMsl | Role::AltitudeAgl)
    });
    let (feet, pressure_m, column_m) = altitude_unit(&groups, pressure, pressure_pa, header_line)?;
    let (unit, scale) = if feet {
        ("feet", FOOT_M)
    } else {
        ("meters", 1.0)
    };
    decisions.push(format!(
        "{} read in {unit}, from the pressure: the standard atmosphere climbs {pressure_m:.1} m \
         from the pressure's pad sample to the one of lowest pressure. Over the same span the \
         column climbs {column_m:.1} {unit}",
        listed(&altitude_names)
    ));
    if let Some((note, agrees)) = gps_altitude(&groups, scale, unit) {
        decisions.push(note);
        if !agrees {
            set_as_written(&mut groups, |role| role == Role::GpsAltitude);
        }
    }
    decide_temperatures(&mut groups, feet, unit, &mut decisions);
    decide_voltages(&mut groups, &mut decisions);

    let fixed: [(RoleTest, &str); 4] = [
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
            |role| matches!(role, Role::Latitude | Role::Longitude),
            "read in degrees, from the values: decimal numbers, as the manual's sample export \
             writes them (Table 2, p40)",
        ),
        (
            |role| role == Role::Satellites,
            "kept as written, a count, from the manual: the satellites used in the GPS's \
             solution (p25)",
        ),
    ];
    for (is, how) in fixed {
        let names = names_of(&groups, is);
        if !names.is_empty() {
            decisions.push(format!("{} {how}", listed(&names)));
        }
    }
    let rssi = names_of(&groups, |role| role == Role::SignalStrength);
    if !rssi.is_empty() {
        decisions.push(format!(
            "{} kept as written, in dBm, from the manual: the strength of the radio signal the \
             AIM BASE received, \"in decibels (dBm)\" (p29). It is a power level in decibels \
             against a milliwatt, not an SI unit. The record has no quantity for a signal's \
             strength",
            listed(&rssi)
        ));
    }
    if let Some(note) = magnetic_note(&groups) {
        decisions.push(note);
    }
    let time_of_week = names_of(&groups, |role| role == Role::TimeOfWeek);
    if !time_of_week.is_empty() {
        decisions.push(match clock.tow_rate_per_s {
            Some(rate) => format!(
                "{} kept as written, in milliseconds, from the values: they stay below a week's \
                 604,800,000. They rise by a median of {rate:.1} a second of the clock. The \
                 record has no quantity for a time of the GPS week",
                listed(&time_of_week)
            ),
            None => format!(
                "{} kept as written, in a unit its values don't settle: they don't rise by a \
                 millisecond's per second of `time`. That is so in either of its units. The \
                 record has no quantity for a time of the GPS week",
                listed(&time_of_week)
            ),
        });
    }
    decisions.extend(flag_notes(&groups));

    let base = groups
        .iter()
        .flat_map(|group| &group.columns)
        .any(|column| column.role == Role::SignalStrength);

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
            let pressure_factor = pressure_pa.map(|(factor, _)| factor);
            let (quantity, written) = si(column, pressure_factor, scale, feet, &mut out_of_range);
            // A value that its unit's conversion makes infinite, a pressure in kPa or inHg, an
            // acceleration in g or a temperature in °F, is refused on its row's line (ADR-258 §3).
            let mut values = Vec::with_capacity(written.len());
            for (line, value) in group.lines.iter().zip(written) {
                values.push(finite(value, FORMAT, *line, &column.name)?);
            }
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
        // Invariant: `Pressure AGL` was found above, and its quantity is always this one.
        return Err(LogError::NoData { format: FORMAT });
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
             times, and {untimed} samples have no time. The record's track takes the samples in \
             time order, the first at each time, and leaves out those without one. The channel \
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
            "the file holds `RSSI`, the radio's strength at the AIM BASE. It is the telemetry \
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
        .filter(|c| c.quantity == Quantity::Voltage && key(&c.column) == "comp. bat. voltage")
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

/// Marks every column whose role is as asked as kept as written.
fn set_as_written(groups: &mut [Group], is: impl Fn(Role) -> bool) {
    for column in groups.iter_mut().flat_map(|group| &mut group.columns) {
        if is(column.role) {
            column.as_written = true;
        }
    }
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
    if column.as_written {
        return (Quantity::Unrecognized, column.values.clone());
    }
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
        Role::Satellites | Role::SignalStrength | Role::Flag { .. } => {
            (Quantity::Number, column.values.clone())
        }
        Role::TimeOfWeek | Role::Magnetic | Role::Unknown => {
            (Quantity::Unrecognized, column.values.clone())
        }
    }
}

/// The pressure's unit from a value on the pad: the factor to pascals and the unit's name; `None`
/// if it is a launch site's pressure in none of them.
fn pressure_unit(pad: f64) -> Option<(f64, &'static str)> {
    [
        (1.0, "pascals"),
        (KPA_PA, "kilopascals"),
        (INHG_PA, "inches of mercury"),
    ]
    .into_iter()
    .find(|(factor, _)| (PAD_PA.0..=PAD_PA.1).contains(&(pad * factor)))
}

/// The altitudes' unit, `true` for feet, with the pressure's climb in meters and the column's as
/// written: the climb of `Pressure MSL`, or else `Pressure AGL`, in the pressure's group, against
/// the pressure's in the U.S. Standard Atmosphere, from the pressure's pad sample to the one of
/// lowest pressure. A file where that can't be measured is refused.
fn altitude_unit(
    groups: &[Group],
    pressure: Option<(usize, usize)>,
    pressure_pa: Option<(f64, usize)>,
    header_line: usize,
) -> Result<(bool, f64, f64), LogError> {
    let refuse = |why: String| LogError::Unit {
        format: FORMAT,
        line: header_line,
        message: format!(
            "the altitudes' unit, meters or feet, can't be decided from the pressure: {why}"
        ),
    };
    let Some((group, at)) = pressure else {
        return Err(refuse("the file has no `pressure`".to_owned()));
    };
    let group = &groups[group];
    let pressure = &group.columns[at];
    let Some((factor, pad)) = pressure_pa else {
        return Err(refuse(format!(
            "`{}` is in a unit its values don't settle",
            pressure.name
        )));
    };
    let Some(altitude) = group
        .columns
        .iter()
        .find(|c| c.role == Role::AltitudeMsl)
        .or_else(|| group.columns.iter().find(|c| c.role == Role::AltitudeAgl))
    else {
        return Err(refuse(
            "no altitude column is beside the pressure on its clock".to_owned(),
        ));
    };
    let pairs: Vec<(f64, f64)> = pressure
        .values
        .iter()
        .zip(&altitude.values)
        .skip(pad)
        .filter(|(p, a)| p.is_finite() && *p * factor > 0.0 && a.is_finite())
        .map(|(p, a)| (p * factor, *a))
        .collect();
    let (Some(&first), Some(&lowest)) = (
        pairs.first(),
        pairs.iter().min_by(|a, b| a.0.total_cmp(&b.0)),
    ) else {
        return Err(refuse(
            "no sample holds both the pressure and the altitude".to_owned(),
        ));
    };
    let atmosphere = Ussa76::standard();
    let climb = atmosphere
        .pressure_altitude_m(lowest.0)
        .and_then(|low| atmosphere.pressure_altitude_m(first.0).map(|pad| low - pad));
    let Ok(pressure_m) = climb else {
        return Err(refuse(
            "the pressure has no altitude in the standard atmosphere".to_owned(),
        ));
    };
    if pressure_m < SETTLING_CLIMB_M {
        return Err(refuse(format!(
            "the pressure climbs {pressure_m:.2} m, under the {SETTLING_CLIMB_M} m from which \
             meters and feet can be told apart"
        )));
    }
    let column = lowest.1 - first.1;
    let within = |climb_m: f64| (climb_m / pressure_m - 1.0).abs() <= ALTITUDE_MARGIN;
    if within(column) {
        Ok((false, pressure_m, column))
    } else if within(column * FOOT_M) {
        Ok((true, pressure_m, column))
    } else {
        Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "`{}` climbs {column:.1} from the pressure's pad sample to the one of lowest \
                 pressure. The pressure's climb in the standard atmosphere is \
                 {pressure_m:.1} m. The column's climb is neither meters nor feet within 25%",
                altitude.name
            ),
        })
    }
}

/// How `time` is read.
struct Clock {
    /// The factor from `time` as written to seconds.
    to_s: f64,
    /// The note saying how, and on what evidence.
    note: String,
    /// `GPS Time of Week`'s median rise a second of the clock, where it decided the unit.
    tow_rate_per_s: Option<f64>,
}

/// The median of the values, `None` if there are none.
fn median(mut values: Vec<f64>) -> Option<f64> {
    values.sort_by(f64::total_cmp);
    values.get(values.len() / 2).copied()
}

/// The clock's unit: from `GPS Time of Week`'s median rise over its clock's, a millisecond's per
/// second (seconds) or per millisecond (milliseconds); else from the fastest clock's median step
/// against the AIM's periods; else refused.
fn clock(groups: &[Group], header_line: usize) -> Result<Clock, LogError> {
    let tow = groups.iter().find_map(|group| {
        group
            .columns
            .iter()
            .find(|c| c.role == Role::TimeOfWeek)
            .map(|c| (group, c))
    });
    let why = match tow {
        None => "the file has no `GPS Time of Week` to decide it by".to_owned(),
        Some((group, column)) => {
            let samples: Vec<(f64, f64)> = group
                .time_s
                .iter()
                .zip(&column.values)
                .filter(|(t, v)| t.is_finite() && v.is_finite())
                .map(|(t, v)| (*t, *v))
                .collect();
            let rates: Vec<f64> = samples
                .windows(2)
                .filter(|pair| pair[1].0 > pair[0].0)
                .map(|pair| (pair[1].1 - pair[0].1) / (pair[1].0 - pair[0].0))
                .collect();
            let below_a_week = samples.iter().all(|s| s.1 < WEEK_MS);
            let near = |rate: f64, target: f64| (rate / target - 1.0).abs() <= TOW_MARGIN;
            match median(rates) {
                None => format!("`{}` has too few samples to decide it by", column.name),
                Some(_) if !below_a_week => format!(
                    "`{}` passes a week's 604,800,000 milliseconds, so it doesn't count them",
                    column.name
                ),
                Some(rate) if near(rate, TOW_RATE) => {
                    return Ok(Clock {
                        to_s: 1.0,
                        note: format!(
                            "`time` read in seconds, from `{}`: it counts milliseconds, below a \
                             week's 604,800,000. It climbs by a median of {rate:.1} per unit of \
                             its `time`, a millisecond's per second",
                            column.name
                        ),
                        tow_rate_per_s: Some(rate),
                    });
                }
                Some(rate) if near(rate, 1.0) => {
                    return Ok(Clock {
                        to_s: 0.001,
                        note: format!(
                            "`time` read in milliseconds, from `{}`: it counts milliseconds, \
                             below a week's 604,800,000. It climbs by a median of {rate:.3} per \
                             unit of its `time`, a millisecond's per millisecond",
                            column.name
                        ),
                        tow_rate_per_s: Some(rate * 1_000.0),
                    });
                }
                Some(rate) => format!(
                    "`{}` climbs by a median of {rate:.1} per unit of its `time`, neither the \
                     1,000 of milliseconds per second nor the 1 per millisecond",
                    column.name
                ),
            }
        }
    };
    // The fastest clock's median step, as written.
    let step = groups
        .iter()
        .filter_map(|group| {
            let times: Vec<f64> = group
                .time_s
                .iter()
                .copied()
                .filter(|t| t.is_finite())
                .collect();
            median(
                times
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .filter(|step| *step > 0.0)
                    .collect(),
            )
        })
        .min_by(f64::total_cmp);
    let periods = "the AIM writes its sensors every 10 to 500 ms (manual p2) times a whole \
                   multiple the owner sets (p21)";
    match step {
        Some(step) if (STEP_SECONDS.0..=STEP_SECONDS.1).contains(&step) => Ok(Clock {
            to_s: 1.0,
            note: format!(
                "`time` read in seconds, from the step: {why}. The fastest clock's median step, \
                 {step} as written, is a step of 0.009 to 2 s. That matches how {periods}. \
                 Milliseconds would be 9 to 2,000"
            ),
            tow_rate_per_s: None,
        }),
        Some(step) if (STEP_MILLISECONDS.0..=STEP_MILLISECONDS.1).contains(&step) => Ok(Clock {
            to_s: 0.001,
            note: format!(
                "`time` read in milliseconds, from the step: {why}. The fastest clock's median \
                 step, {step} as written, is a step of 9 to 2,000 ms. That matches how \
                 {periods}. Seconds would be 0.009 to 2"
            ),
            tow_rate_per_s: None,
        }),
        Some(step) => Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "the clock's unit can't be decided: {why}. The fastest clock's median step, \
                 {step}, is a step in neither seconds (0.009 to 2) nor milliseconds (9 to \
                 2,000). Both ranges match how {periods}"
            ),
        }),
        None => Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "the clock's unit can't be decided: {why}, and no clock has two samples to \
                 measure a step by"
            ),
        }),
    }
}

/// The note on `GPS MSL`'s unit, and whether it agrees with the altitudes': the median
/// difference, over the GPS samples within [`PAIR_S`] of a `Pressure MSL` sample, within
/// [`GPS_AGREEMENT_M`] in the altitudes' unit and beyond it in the other. `None` without the
/// column.
fn gps_altitude(groups: &[Group], scale: f64, unit: &str) -> Option<(String, bool)> {
    let (gps_group, gps_at) = locate(groups, Role::GpsAltitude)?;
    let gps = &groups[gps_group].columns[gps_at];
    let kept = |why: String| {
        Some((
            format!(
                "`{}` kept as written, in a unit `Pressure MSL` doesn't settle: {why}",
                gps.name
            ),
            false,
        ))
    };
    let Some((baro_group, baro_at)) = locate(groups, Role::AltitudeMsl) else {
        return kept("the file has no `Pressure MSL` to compare it with".to_owned());
    };
    let mut barometric: Vec<(f64, f64)> = groups[baro_group]
        .time_s
        .iter()
        .zip(&groups[baro_group].columns[baro_at].values)
        .filter(|(t, v)| t.is_finite() && v.is_finite())
        .map(|(t, v)| (*t, *v))
        .collect();
    barometric.sort_by(|a, b| a.0.total_cmp(&b.0));
    let other = if scale == 1.0 { FOOT_M } else { 1.0 };
    let other_unit = if scale == 1.0 { "feet" } else { "meters" };
    let mut same_m = Vec::new();
    let mut other_m = Vec::new();
    for (time, value) in groups[gps_group].time_s.iter().zip(&gps.values) {
        if !(time.is_finite() && value.is_finite()) {
            continue;
        }
        let after = barometric.partition_point(|(t, _)| t < time);
        let nearest = [after.checked_sub(1), Some(after)]
            .into_iter()
            .flatten()
            .filter_map(|at| barometric.get(at))
            .min_by(|a, b| (a.0 - time).abs().total_cmp(&(b.0 - time).abs()));
        if let Some(&(t, baro)) = nearest
            && (t - time).abs() <= PAIR_S
        {
            same_m.push((value - baro) * scale);
            other_m.push(value * other - baro * scale);
        }
    }
    let pairs = same_m.len();
    let (Some(same), Some(differs)) = (median(same_m), median(other_m)) else {
        return kept("no GPS sample is within 1 s of a `Pressure MSL` sample".to_owned());
    };
    let measured = format!(
        "over {pairs} GPS samples within 1 s of one of it, the median difference is {same:.1} m \
         read in {unit} and {differs:.1} m in {other_unit}"
    );
    if same.abs() <= GPS_AGREEMENT_M && differs.abs() > GPS_AGREEMENT_M {
        Some((
            format!(
                "`{}` read in {unit}, from `Pressure MSL`: {measured}, within the 300 m a day's \
                 weather moves a standard-atmosphere altitude only in {unit}",
                gps.name
            ),
            true,
        ))
    } else {
        kept(format!(
            "{measured}, so the 300 m a day's weather moves a standard-atmosphere altitude \
             doesn't tell the units apart"
        ))
    }
}

/// Decides each temperature column's unit from the altitudes', and its first value's range.
fn decide_temperatures(groups: &mut [Group], feet: bool, unit: &str, decisions: &mut Vec<String>) {
    let (degrees, low, high) = if feet {
        (
            "Fahrenheit",
            BAY_C.0 * 9.0 / 5.0 + 32.0,
            BAY_C.1 * 9.0 / 5.0 + 32.0,
        )
    } else {
        ("Celsius", BAY_C.0, BAY_C.1)
    };
    let system = if feet { "imperial" } else { "metric" };
    for column in groups.iter_mut().flat_map(|group| &mut group.columns) {
        if column.role != Role::Temperature {
            continue;
        }
        let first = column.values.iter().copied().find(|v| v.is_finite());
        match first {
            Some(value) if (low..=high).contains(&value) => decisions.push(format!(
                "`{}` read in degrees {degrees}, from the heights' unit and the manual, p33: the \
                 heights are in {unit}, decided from the pressure. The software sets one \
                 unit system for all its displays (p33), here {system}, its sensor reading °C or \
                 °F (p25). The first value, {value}, is a bay's temperature in degrees \
                 {degrees}, {low} to {high}",
                column.name
            )),
            _ => {
                column.as_written = true;
                decisions.push(format!(
                    "`{}` kept as written, in a unit its values don't settle: the heights are in \
                     {unit}, so the displays are {system} (p33). Its first value, {}, isn't \
                     a bay's temperature in degrees {degrees}, {low} to {high}",
                    column.name,
                    first.map_or("none".to_owned(), |v| v.to_string())
                ));
            }
        }
    }
}

/// Decides each voltage column's unit from its values: volts if every one lies within 0 to
/// [`VOLTS_MAX`].
fn decide_voltages(groups: &mut [Group], decisions: &mut Vec<String>) {
    for column in groups.iter_mut().flat_map(|group| &mut group.columns) {
        if column.role != Role::Voltage {
            continue;
        }
        let (low, high) = column
            .values
            .iter()
            .filter(|v| v.is_finite())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &v| {
                (low.min(v), high.max(v))
            });
        if low <= high && low >= 0.0 && high <= VOLTS_MAX {
            decisions.push(format!(
                "`{}` read in volts, from the values: they lie within {low} to {high}. That is inside \
                 the 0 to 20 V of a battery or an output line. The computer's battery is 3.5 to \
                 8.4 V and the ejection battery at most 10 V (manual p10). Millivolts would \
                 be thousands",
                column.name
            ));
        } else {
            column.as_written = true;
            decisions.push(format!(
                "`{}` kept as written, in a unit its values don't settle: {}",
                column.name,
                if low <= high {
                    format!(
                        "they lie within {low} to {high}, not all inside the 0 to 20 V of a \
                         battery or an output line (manual p10)"
                    )
                } else {
                    "it holds no value".to_owned()
                }
            ));
        }
    }
}

/// The note on the magnetometer's columns, kept as written: the field they would make in the
/// manual's tesla, against the Earth's. `None` without them.
fn magnetic_note(groups: &[Group]) -> Option<String> {
    let names = names_of(groups, |role| role == Role::Magnetic);
    if names.is_empty() {
        return None;
    }
    // The field from the magnetic columns of the first group holding one, sample by sample.
    let group = groups
        .iter()
        .find(|group| group.columns.iter().any(|c| c.role == Role::Magnetic))?;
    let axes: Vec<&Column> = group
        .columns
        .iter()
        .filter(|c| c.role == Role::Magnetic)
        .collect();
    let (low, high) = (0..group.time_s.len())
        .filter_map(|at| {
            let squares: Option<f64> = axes
                .iter()
                .map(|axis| axis.values.get(at).copied().filter(|v| v.is_finite()))
                .try_fold(0.0, |sum, v| v.map(|v| sum + v * v));
            squares.map(f64::sqrt)
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), v| {
            (low.min(v), high.max(v))
        });
    let field = if low <= high {
        format!(
            "Read so, the field {} make{}, {low:.5} to {high:.5}, would be {:.0} to {:.0} times \
             the Earth's at its strongest, about 65 µT",
            if axes.len() == 1 {
                "its column"
            } else {
                "the columns"
            },
            if axes.len() == 1 { "s" } else { "" },
            low / EARTH_FIELD_T,
            high / EARTH_FIELD_T
        )
    } else {
        "The file holds no sample of them all to check it by".to_owned()
    };
    Some(format!(
        "{} kept as written, in a unit no source bears out: the manual gives tesla (p24). \
         {field}. The record has no quantity for a magnetic field",
        listed(&names)
    ))
}

/// The notes on the flags: kept as written, 0 or 1 by their values, or else in a unit no source
/// states.
fn flag_notes(groups: &[Group]) -> Vec<String> {
    let mut flags = Vec::new();
    let mut others = Vec::new();
    for column in groups.iter().flat_map(|group| &group.columns) {
        if let Role::Flag { .. } = column.role {
            if column
                .values
                .iter()
                .all(|v| v.is_nan() || *v == 0.0 || *v == 1.0)
            {
                flags.push(column.name.clone());
            } else {
                others.push(column.name.clone());
            }
        }
    }
    let mut notes = Vec::new();
    if !flags.is_empty() {
        notes.push(format!(
            "{} kept as written, flags, from the values: each is 0 or 1, the AIM's flight event \
             status and its lines' continuity (manual p2, p36)",
            listed(&flags)
        ));
    }
    if !others.is_empty() {
        notes.push(format!(
            "{} kept as written, in a unit no source states: they hold values other than 0 and 1, \
             so they are no flags",
            listed(&others)
        ));
    }
    notes
}

/// The names of the columns whose role is as asked, in the order of [`KNOWN`], so the same
/// columns give the same note whatever their order (a header names each once).
fn names_of(groups: &[Group], is: impl Fn(Role) -> bool) -> Vec<String> {
    let mut names: Vec<(usize, String)> = groups
        .iter()
        .flat_map(|group| &group.columns)
        .filter(|column| is(column.role))
        .filter_map(|column| {
            let key = key(&column.name);
            KNOWN
                .iter()
                .position(|(known, _)| *known == key)
                .map(|place| (place, column.name.clone()))
        })
        .collect();
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

    /// The standard atmosphere's altitude of a pressure, in the troposphere.
    fn altitude_of(pressure_pa: f64) -> f64 {
        44_330.8 * (1.0 - (pressure_pa / 101_325.0).powf(0.190_263))
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
            let msl = altitude_of(p);
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

    /// A two-row file of the pressure, its altitude above a pad 300 m up and one more group, the
    /// times 0 and 1 s: a climb of 100 m.
    fn small(extra_header: &str, extra_rows: [&str; 2]) -> String {
        let mut text = format!("time,pressure,Pressure AGL,{extra_header}\n");
        for (row, height) in [0.0, 100.0].iter().enumerate() {
            let p = pressure_at(300.0 + height).round();
            let agl = altitude_of(p) - altitude_of(pressure_at(300.0).round());
            text.push_str(&format!("{row},{p},{agl:.3},{}\n", extra_rows[row]));
        }
        text
    }

    fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
        let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
        assert_eq!(matching.len(), 1, "{start}: {:?}", log.notes);
        matching[0]
    }

    /// The survey's reading of a note (`xtask logs`' `unit_decision`): the text before the first
    /// `: ` of a note opening with a name in backticks that is `read in` or `kept as written`.
    fn counted(note: &str) -> Option<&str> {
        let (decision, _) = note.split_once(": ")?;
        (decision.starts_with('`')
            && (decision.contains("` read in ") || decision.contains("` kept as written")))
        .then_some(decision)
    }

    fn unit_error(text: &str) -> String {
        match read(text) {
            Err(LogError::Unit { message, .. }) => message,
            other => panic!("{other:?}"),
        }
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
        // A long first line is walked, not copied, and stops at the first answer.
        let long = format!("time,Pressure AGL,time,{}\n", "x,".repeat(200_000));
        assert!(recognizes(&long));
        assert!(!recognizes(&format!("time,{}\n", "x,".repeat(200_000))));
    }

    /// Each group on its own clock, in SI; the primary track `Pressure AGL`'s; meters from the
    /// pressure, seconds from `GPS Time of Week`; the flags' rises as events; every column's unit
    /// in a note the survey counts.
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
        note(
            &log,
            "`launch detect`, `apogee detect` and `A on.` kept as written, flags, from the \
             values: ",
        );
        note(
            &log,
            "`GPS Time of Week` kept as written, in milliseconds, from the values: ",
        );
        assert!(log.temperature_k.is_none() && log.battery_v.is_none());
        // Every column is named in a note the survey counts, and none is assumed.
        let decisions: Vec<&str> = log.notes.iter().filter_map(|n| counted(n)).collect();
        for channel in &log.channels {
            let named = format!("`{}`", channel.column);
            assert!(
                decisions.iter().any(|d| d.contains(&named)),
                "{named}: {decisions:?}"
            );
        }
        assert!(log.notes.iter().all(|n| !n.contains("assumed")));
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
        assert!(unit_error(&flight(0.5)).contains("neither meters nor feet"));
    }

    /// A hop of 8 m, written in feet, is told from meters: the pressure's 1 Pa step is about 1%
    /// of the climb. Under 2 m the file is refused, never assumed.
    #[test]
    fn a_short_hop_in_feet_is_decided_and_a_flat_file_refused() {
        let hop = |height_m: f64| {
            let mut text = "time,pressure,Pressure AGL,time,launch detect\n".to_owned();
            for (row, height) in [0.0, height_m / 2.0, height_m, 0.0].iter().enumerate() {
                let p = pressure_at(100.0 + height).round();
                let agl = altitude_of(p) - altitude_of(pressure_at(100.0).round());
                text.push_str(&format!("{row},{p},{:.2},{row},0\n", agl / FOOT_M));
            }
            text
        };
        let log = read(&hop(8.0)).unwrap();
        note(&log, "`Pressure AGL` read in feet, from the pressure: ");
        let top = log.altitude_m.iter().copied().fold(0.0, f64::max);
        assert!((top - 8.0).abs() < 0.2, "{top}");
        assert!(unit_error(&hop(1.0)).contains("under the 2 m"));
    }

    #[test]
    fn a_pressure_unit_comes_from_the_pad_or_the_file_is_refused() {
        assert_eq!(pressure_unit(97_000.0), Some((1.0, "pascals")));
        assert_eq!(pressure_unit(97.0), Some((KPA_PA, "kilopascals")));
        assert_eq!(pressure_unit(28.6), Some((INHG_PA, "inches of mercury")));
        for pad in [970.0, 5.0, 300_000.0, 40.0, 0.0, f64::NAN] {
            assert_eq!(pressure_unit(pad), None, "{pad}");
        }
        // A placeholder ahead of the pad's pressure is passed over, and the climb measured from
        // the pad's sample.
        let text = "time,pressure,Pressure AGL,time,launch detect\n\
                    0,0,0,0,0\n1,97000,0,1,0\n2,96000,85.2,2,1\n";
        let log = read(text).unwrap();
        assert!(
            note(&log, "`pressure` read in pascals, from the values: ")
                .contains("97000, after 1 samples that aren't")
        );
        note(&log, "`Pressure AGL` read in meters, from the pressure: ");
        // A pressure in no unit leaves the altitudes with none: refused.
        let text = "time,pressure,Pressure AGL,time,launch detect\n0,970,0,0,0\n1,960,90,1,1\n";
        assert!(unit_error(text).contains("`pressure` is in a unit its values don't settle"));
        let text = "time,Pressure AGL,time,launch detect\n0,0,0,0\n1,90,1,1\n";
        assert!(unit_error(text).contains("the file has no `pressure`"));
    }

    /// `time` in seconds or milliseconds by `GPS Time of Week`, else by the step, else refused.
    #[test]
    fn the_clock_is_decided_by_the_time_of_week_or_the_step() {
        let by_tow = read(&flight(1.0)).unwrap();
        assert!(note(&by_tow, "`time` read in seconds, from ").contains("median of 1000.0"));
        // The same flight with every clock written in milliseconds reads the same times.
        let millis: String = flight(1.0)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                if row == 0 {
                    return format!("{line}\n");
                }
                let cells: Vec<String> = line
                    .split(',')
                    .enumerate()
                    .map(|(at, cell)| {
                        if [0, 2, 6, 10].contains(&at) {
                            format!("{}", (cell.parse::<f64>().unwrap() * 1_000.0).round())
                        } else {
                            cell.to_owned()
                        }
                    })
                    .collect();
                format!("{}\n", cells.join(","))
            })
            .collect();
        let log = read(&millis).unwrap();
        assert!(
            note(
                &log,
                "`time` read in milliseconds, from `GPS Time of Week`: "
            )
            .contains("median of 1.000")
        );
        for (a, b) in log.time_s.iter().zip(&by_tow.time_s) {
            assert!((a - b).abs() < 1e-9, "{a} {b}");
        }
        assert!((log.events[0].time_s - 0.01).abs() < 1e-9);
        note(
            &log,
            "`GPS Time of Week` kept as written, in milliseconds, from the values: ",
        );
        // A time of week rising by 7 per unit of `time`: the step decides, 1 s.
        let odd: String = flight(1.0)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                if row == 0 {
                    return format!("{line}\n");
                }
                let (head, _) = line.rsplit_once(',').unwrap();
                format!("{head},{}\n", 1_000 + 7 * row)
            })
            .collect();
        let log = read(&odd).unwrap();
        assert!(
            note(&log, "`time` read in seconds, from the step: ")
                .contains("neither the 1,000 of milliseconds per second")
        );
        note(
            &log,
            "`GPS Time of Week` kept as written, in a unit its values don't settle: ",
        );
        // No time of week: steps of 1 s, or 1,000 ms.
        let seconds = read(&small("time,launch detect", ["0,0", "1,1"])).unwrap();
        assert!(
            note(&seconds, "`time` read in seconds, from the step: ")
                .contains("the file has no `GPS Time of Week`")
        );
        let ms = small("time,launch detect", ["0,0", "1000,1"]).replace("\n1,", "\n1000,");
        let ms = read(&ms).unwrap();
        note(&ms, "`time` read in milliseconds, from the step: ");
        assert_eq!(ms.time_s, [0.0, 1.0]);
        // A step of 5 is neither; a single row has none.
        let five = small("time,launch detect", ["0,0", "5,1"]).replace("\n1,", "\n5,");
        assert!(unit_error(&five).contains("in neither seconds"));
        let one = format!(
            "time,pressure,Pressure AGL,time,launch detect\n0,{},0,0,0\n",
            pressure_at(300.0)
        );
        assert!(unit_error(&one).contains("no clock has two samples"));
    }

    /// The GPS group's extra empty cell is dropped only where the GPS group is empty and every
    /// group is then whole; any other row of another width is refused.
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
        // The extra cell at the row's end: dropping the one after the GPS group would leave the
        // next group part filled and shift the last, so the row is refused.
        let header = "time,lat,long,GPS MSL,SVs,time,temperature,time,Pressure AGL";
        let elsewhere = format!("{header}\n,,,,,,,1.0,5.0,\n");
        match read(&elsewhere) {
            Err(LogError::Syntax {
                line: 2, message, ..
            }) => {
                assert!(message.contains("columns 6 to 7 part filled"), "{message}");
            }
            other => panic!("{other:?}"),
        }
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
        assert!(note(&log, "`RSSI` kept as written, in dBm, from the manual: ").contains("(p29)"));
        assert_eq!(log.channels[2].quantity, Quantity::Number);
        let log = read(&small("time,RSSI", ["0,-90", "1,-95"])).unwrap();
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
            // A column named twice, in another case or without its point.
            ("time,Pressure AGL,time,pressure agl\n0,1,0,1\n", 1),
            ("time,Pressure AGL,A on.,time,A on\n0,1,0,0,0\n", 1),
        ];
        for (text, line) in cases {
            match read(text) {
                Err(LogError::Syntax { line: at, .. }) => assert_eq!(at, line, "{text:?}"),
                other => panic!("{text:?}: {other:?}"),
            }
        }
        let Err(error) = read("time,Pressure AGL,A on.,time,a on\n") else {
            panic!("read");
        };
        assert!(error.to_string().contains("twice"), "{error}");
        assert!(matches!(
            read("time,Pressure AGL,time,x\n,,,1\n"),
            Err(LogError::NoData { .. })
        ));
        assert!(matches!(read(""), Err(LogError::NotThisFormat { .. })));
    }

    /// An acceleration in g, a pressure in kPa and a temperature in °F near the largest `f64` are
    /// finite as written and infinite in SI: each is refused on its line (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let g = small("time,acceleration", ["0,1", "1,-1.7e308"]);
        assert_eq!(
            read(&g).unwrap_err().to_string(),
            "Entacore AIM CSV line 3: the column acceleration holds a value too large for its \
             unit's conversion to SI"
        );

        let mut kpa = String::from("time,pressure,Pressure AGL,time,launch detect\n");
        for (row, height) in [0.0, 100.0].iter().enumerate() {
            let p = pressure_at(300.0 + height).round();
            let agl = altitude_of(p) - altitude_of(pressure_at(300.0).round());
            kpa.push_str(&format!("{row},{},{agl:.3},{row},0\n", p / 1_000.0));
        }
        kpa.push_str("2,1.7e308,50,2,0\n");
        assert_eq!(
            read(&kpa).unwrap_err().to_string(),
            "Entacore AIM CSV line 4: the column pressure holds a value too large for its unit's \
             conversion to SI"
        );

        // In feet, so the temperature is read in °F.
        let mut fahrenheit = String::from("time,pressure,Pressure AGL,time,temperature\n");
        for (row, height) in [0.0, 100.0].iter().enumerate() {
            let p = pressure_at(300.0 + height).round();
            let agl = altitude_of(p) - altitude_of(pressure_at(300.0).round());
            let cell = if row == 0 { "70" } else { "-1.7e308" };
            fahrenheit.push_str(&format!("{row},{p},{:.3},{row},{cell}\n", agl / FOOT_M));
        }
        assert_eq!(
            read(&fahrenheit).unwrap_err().to_string(),
            "Entacore AIM CSV line 3: the column temperature holds a value too large for its \
             unit's conversion to SI"
        );
    }

    #[test]
    fn unknown_columns_and_out_of_range_positions_are_kept_and_noted() {
        let text = small("widget,time,lat,long", ["7,0,91,-117", ",1,34,200"]);
        let log = read(&text).unwrap();
        assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
        assert!(log.channels[2].values[1].is_nan());
        assert!(log.channels[3].values[0].is_nan() && log.channels[3].values[1] == 34.0);
        assert!(log.channels[4].values[1].is_nan());
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
        let log = read(&small("time,A cont.", ["0,0", "1,1"])).unwrap();
        assert!(log.events.is_empty());
        note(&log, "`A cont.` kept as written, flags, from the values: ");
        let log = read(&small("time,A cont.", ["0,0", "1,7"])).unwrap();
        note(
            &log,
            "`A cont.` kept as written, in a unit no source states: ",
        );
    }

    #[test]
    fn the_temperature_follows_the_altitudes_unit_within_a_bay_s_range() {
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
        assert!(
            note(
                &log,
                "`temperature` read in degrees Fahrenheit, from the heights' unit and the manual, \
                 p33: "
            )
            .contains("here imperial")
        );
        assert!(close(log.channels[3].values[0], 293.15));
        let celsius = read(&flight(1.0).replace("Gyro X", "temperature")).unwrap();
        note(
            &celsius,
            "`temperature` read in degrees Celsius, from the heights' unit and the manual, p33: ",
        );
        assert!(close(celsius.channels[0].values[1], 0.5 + 273.15));
        // 68 is no bay's temperature in °C: kept as written.
        let hot = read(&small("time,temperature", ["0,68", "1,68"])).unwrap();
        note(
            &hot,
            "`temperature` kept as written, in a unit its values don't settle: ",
        );
        assert_eq!(hot.channels[2].quantity, Quantity::Unrecognized);
        assert_eq!(hot.channels[2].values[0], 68.0);
    }

    #[test]
    fn voltages_are_volts_by_their_values_or_kept_as_written() {
        let log = read(&small(
            "time,comp. bat. voltage,A Voltage",
            ["0,3.9,0", "1,3.8,7.5"],
        ))
        .unwrap();
        assert!(
            note(
                &log,
                "`comp. bat. voltage` read in volts, from the values: "
            )
            .contains("within 3.8 to 3.9")
        );
        note(&log, "`A Voltage` read in volts, from the values: ");
        assert_eq!(log.channels[2].quantity, Quantity::Voltage);
        // Millivolts: kept as written.
        let log = read(&small("time,comp. bat. voltage", ["0,3900", "1,3850"])).unwrap();
        note(
            &log,
            "`comp. bat. voltage` kept as written, in a unit its values don't settle: ",
        );
        assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
    }

    /// `GPS MSL` in the altitudes' unit where it agrees with `Pressure MSL` and the other unit
    /// doesn't; kept as written where it disagrees, or where the site is too low to tell.
    #[test]
    fn the_gps_altitude_is_decided_against_the_barometric_one() {
        let file = |pad_m: f64, gps: &dyn Fn(f64) -> f64| {
            let mut text =
                "time,pressure,Pressure MSL,Pressure AGL,time,lat,long,GPS MSL,SVs\n".to_owned();
            for (row, height) in [0.0, 100.0, 300.0].iter().enumerate() {
                let p = pressure_at(pad_m + height).round();
                let msl = altitude_of(p);
                let agl = msl - altitude_of(pressure_at(pad_m).round());
                text.push_str(&format!(
                    "{row},{p},{msl:.2},{agl:.2},{row}.3,34.5,-117.25,{:.1},9\n",
                    gps(msl)
                ));
            }
            text
        };
        // 40 m above the barometric altitude, in meters.
        let log = read(&file(800.0, &|msl| msl + 40.0)).unwrap();
        assert!(
            note(&log, "`GPS MSL` read in meters, from `Pressure MSL`: ")
                .contains("over 3 GPS samples")
        );
        assert_eq!(log.channels[5].quantity, Quantity::GnssAltitude);
        // The same in feet beside meters: kept as written.
        let log = read(&file(800.0, &|msl| (msl + 40.0) / FOOT_M)).unwrap();
        note(
            &log,
            "`GPS MSL` kept as written, in a unit `Pressure MSL` doesn't settle: ",
        );
        assert_eq!(log.channels[5].quantity, Quantity::Unrecognized);
        // A site 50 m up: feet and meters both within 300 m, so neither is taken.
        let log = read(&file(50.0, &|msl| msl + 20.0)).unwrap();
        assert!(
            note(
                &log,
                "`GPS MSL` kept as written, in a unit `Pressure MSL` doesn't settle: "
            )
            .contains("doesn't tell the units apart")
        );
    }

    #[test]
    fn the_magnetometer_note_gives_the_file_s_field() {
        let log = read(&small(
            "time,Mag X,Mag Y,Mag Z",
            ["0,0.0012,0.0016,0", "1,0.0006,0.0008,0"],
        ))
        .unwrap();
        let mag = note(
            &log,
            "`Mag X`, `Mag Y` and `Mag Z` kept as written, in a unit no source bears out: ",
        );
        // 0.001 T and 0.002 T are 15 and 31 times 65 µT.
        assert!(
            mag.contains("0.00100 to 0.00200, would be 15 to 31 times"),
            "{mag}"
        );
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
