//! An Eggtimer's flight data, as the flight computer's own download writes it: the Eggtimer
//! Classic's and TRS's over a serial cable, the Quantum's and Proton's from their WiFi page.
//!
//! The file is comma-separated text: one header row naming the columns, then one row per sample,
//! every column on the row's time. There is no preamble and the header states no units:
//!
//! ```text
//! T,Alt,Veloc,FAlt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main
//! 0,0,0,0,0,0,0,0,0,0,0
//! 0.05,4,80,0.8,16,0,0,0,0,0,0
//! 0.1,9,100,2.44,32.8,0,0,0,0,0,0
//! ```
//!
//! **Layouts.** The first column is the time, `T`, and the second the barometric altitude, `Alt`.
//! Then come the logger's figures, which differ by model and firmware: a raw velocity, a filtered
//! altitude and velocity (`FAlt`, `Filt_Alt` or `KAlt`, then `FVeloc` or `KVeloc`), and on the
//! Proton the accelerometer's figures. Last come the event markers, from `LDA` on. The Classic's
//! manual shows `T,Alt,Veloc,KAlt,KVeloc,LDA,BurnT,LowV,Apogee,N-O,CHA,CHB` (*Eggtimer Flight
//! Computer User's Manual*, firmware 1.52, board RevD, p37), the TRS's
//! `T,Alt,Veloc,FAlt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main` (*Eggtimer TRS Flight Computer User's
//! Manual*, firmware 2.07, p37), and an older guide `T`, `Alt`, `VRaw`, `VFilt` (*Using Excel to
//! Graph Eggtimer Flight Data*, p3). The project's sample logs add a layout without the raw
//! velocity (`T,Alt,FAlt,FVeloc,…`) and the Proton's 22 columns, whose accelerometer columns are
//! named `AccelG`, `AccelFPSS`, `VAccel`, `AltAccel` and filtered `FAccel…` where its manual writes
//! `Accel`, `AccelFPSS`, `VelocAcc` and `AltAcc` (*Eggtimer Proton User's Guide*, software 2.01C,
//! p45). The Proton lets its owner name each deployment channel, up to 8 characters, and writes
//! that name in the header (*Eggtimer Proton Release Notes*, version 2.01C, p2), so a marker
//! column may carry any name.
//!
//! A file is this layout if its first line that isn't blank is comma-separated, names `T` then
//! `Alt`, and names at least one more column this reader knows (the table below). That keeps out a
//! PerfectFlite export (`Time,Altitude,…`) and an RRC3's (`Time,Altitude,Pressure,…`).
//!
//! **Units.** The header states none. Each column, what it is read as, and where that comes from:
//!
//! | Column | Read as | From |
//! |---|---|---|
//! | `T` | the clock: seconds, or milliseconds when the step says so (below) | the Classic's and TRS's manuals (p37) and the Excel guide (p3): milliseconds; the Quantum's (*Eggtimer Quantum User's Guide*, software 1.09G, p38) and Proton's (p45): seconds |
//! | `Alt` | [`Quantity::AltitudeAboveGround`], ft | the Classic's and TRS's manuals (p37): "the altitude in feet"; the Quantum's and Proton's state no unit, but set every altitude in feet (Quantum p24: launch detect "50’-500’") |
//! | `FAlt`, `Filt_Alt`, `KAlt` | [`Quantity::AltitudeAboveGround`], ft: the logger's filtered `Alt` | Quantum p38, Proton p45; Classic p37 ("the two K fields are filtered data") |
//! | `Veloc`, `VRaw` | [`Quantity::Speed`], ft/s: the change in `Alt` over the step | Classic and TRS p37 ("velocity in ft/sec"); Quantum p38; the files (below) |
//! | `FVeloc`, `KVeloc`, `VFilt` | [`Quantity::Speed`], ft/s: the change in the filtered altitude over the step | Quantum p38, Proton p46; the Excel guide (p3, p7: "Velocity in ft/sec"); the files |
//! | `AccelG`, `Accel`, `FAccelG` | [`Quantity::Acceleration`], g × 9.806 65 m/s² (standard gravity) | Proton p45 ("in G's"); the name |
//! | `AccelFPSS`, `FAccelFPSS` | [`Quantity::Acceleration`], ft/s² | Proton p45 ("ft/sec^2"); the name |
//! | `VAccel`, `VelocAcc`, `FAccelVel` | [`Quantity::Speed`], ft/s, from the accelerometer | Proton p45 ("the velocity derived from the accelerometer in ft/sec") |
//! | `AltAccel`, `AltAcc`, `FAccelAlt` | [`Quantity::InertialAltitude`], ft | Proton p45 ("the DISTANCE TRAVELED derived from the accelerometer in ft.") |
//! | `LDA`, `LowV`, `Apogee`, `N-O`, `Drogue`, `Main`, `AUX`, `BurnT`, `CHA`, `CHB`, `CH1`…, `MaxV`, `MaxA`, `Boost#1`…, `BO#1`…, and any column after `LDA` | [`Quantity::Unrecognized`], as written: an event marker (below) | Quantum p38–39, Proton p46 |
//!
//! No Eggtimer manual offers a unit other than feet: the Classic's, TRS's, Quantum's and Proton's
//! manuals and release notes name no meter or metric setting. On the 43 Eggtimer files among the
//! project's sample logs, `Veloc` equals the change in `Alt` over the step, in that step's unit,
//! in all but a few rows of each, and `FVeloc` the change in the filtered altitude; in the four
//! with an accelerometer, `AccelFPSS` is about 32 times `AccelG` (32.17 ft/s² is one g), `VAccel`
//! tracks the sum of `AccelFPSS` over the steps, and `AccelG` reads about 0 on the pad, so the
//! acceleration has gravity taken out; it is kept as written. The `F`-prefixed accelerometer
//! columns are read as their unfiltered namesakes, as `FAlt` is `Alt` filtered: no manual
//! describes them.
//!
//! **The time's unit.** The Classic's and TRS's downloads, and the older guide's, write `T` in
//! milliseconds; the Quantum's and Proton's in seconds; and the sample logs hold eleven-column
//! files in both. The logger takes 4 to 33 samples a second going up and 1 to 10 coming down
//! (Classic p19–20; Quantum p25), so a step between rows is 0.03 to 1 in seconds and 30 to 1,000
//! in milliseconds. The reader takes the median step between rows: above 5, a factor of 5 clear of
//! either range, `T` is in milliseconds; otherwise in seconds. A file of one row has no step, and
//! is read in seconds. A note on the record says which, and why.
//!
//! **Event markers.** The manuals put the markers last, from `LDA` on: each is 0 at every sample
//! but the one that triggers its event, where the Quantum's and Proton's manuals say it holds "the
//! RAW altitude data", in feet (Quantum p38; Proton p46). `MaxV` and `MaxA` hold the top speed and
//! acceleration instead, ft/s and ft/s² (*Eggtimer Proton Release Notes*, version 1.02N, p13: "now
//! referenced to the actual value"); in the sample logs they are the whole part of `VAccel` and
//! `AccelFPSS` on their row. A marker may also be held from its event to the end of the file, as
//! three are in one of the sample logs. Each marker column is kept as a channel as written,
//! [`Quantity::Unrecognized`]: its 0 means "no event", not 0 ft, so converting it as a height would
//! mislead. Each run of non-zero samples is an [`Event`] named as the column, at the run's first
//! sample. An event that happened at a raw altitude of exactly 0 ft can't be told from no event,
//! and is lost. A column after `LDA` whose name this reader doesn't know, such as a Proton
//! channel's own name, is read as a marker, with a note.
//!
//! **The primary track.** [`FlightLog::altitude_m`] is `Alt`, the barometer's raw altitude above
//! the logger's zero. The vertical speed is `Veloc` (or `VRaw`), the barometer's; without it, the
//! filtered `FVeloc` (or `KVeloc`, `VFilt`), also the barometer's, with a note. `VAccel` is the
//! accelerometer's, a channel only. The file names no model, serial number or flight, so the logger
//! reads `Eggtimer`.
//!
//! **Cells.** Lines end in CR, LF, CR LF or CR CR LF (a blank line after every row, which is
//! skipped). A row whose every cell is empty, as some downloads end with hundreds of, is skipped
//! and counted in a note. An empty cell, or a row shorter than the header, leaves a gap, `NaN`,
//! counted per column in a note. A cell that isn't a number, a row longer than the header, an empty
//! time, and a time that doesn't increase are refused, naming the line. A column this reader
//! doesn't know, before `LDA`, is kept as written, [`Quantity::Unrecognized`], with a note.
//!
//! **Where this comes from.** The layouts and units are the manuals' above; the time-unit rule, the
//! blank-row rule and the column names they don't show are the sample logs'. Debrief's reader
//! (`lib/parsers/eggtimer.ts`, MIT, the project owner's own) also tells milliseconds from seconds
//! by the median step, over the first 60 rows with its line at 3. Where this reader departs from
//! it: it keeps every column and reads the markers as events, where Debrief maps the time, the
//! altitude and one velocity; it takes the raw `Veloc` as the vertical speed where Debrief prefers
//! the filtered one; it reads the layouts without `Veloc`, which Debrief doesn't recognize; and it
//! refuses what Debrief passes over, naming the line. Debrief's note that the logger can be set to
//! metric units cites no source, and none of the manuals above offers such a setting.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Eggtimer CSV";

/// The median step between rows above which `T` is read in milliseconds. At the manuals' 1 to 33
/// samples a second, a step is 0.03 to 1 in seconds and 30 to 1,000 in milliseconds: 5 is a
/// factor of 5 clear of the first and 6 of the second.
const MILLISECONDS_ABOVE: f64 = 5.0;

/// How far a speed may be from its altitude's change over the step and still count as that
/// change, in the file's units: 1 %, or 1 ft/s, whichever is larger. The cells are rounded to
/// whole feet or hundredths, and a few rows fall across a change of sampling rate.
const SPEED_MATCH_RELATIVE: f64 = 0.01;
const SPEED_MATCH_FT_S: f64 = 1.0;

/// What a column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// `Alt`: the barometer's raw altitude, ft.
    Altitude,
    /// `FAlt`, `Filt_Alt`, `KAlt`: the barometric altitude filtered, ft.
    FilteredAltitude,
    /// `Veloc`, `VRaw`: the change in `Alt` over the step, ft/s.
    RawSpeed,
    /// `FVeloc`, `KVeloc`, `VFilt`: the change in the filtered altitude over the step, ft/s.
    FilteredSpeed,
    /// The accelerometer in g.
    AccelerationG,
    /// The accelerometer in ft/s².
    AccelerationFtS2,
    /// The accelerometer's velocity, ft/s.
    InertialSpeed,
    /// The accelerometer's distance traveled, ft.
    InertialAltitude,
    /// An event marker: 0, or at its event the raw altitude (or, for `MaxV` and `MaxA`, the
    /// speed or acceleration).
    Marker,
    /// A column the reader doesn't know.
    Unknown,
}

impl Kind {
    /// What the column measures, and the factor that takes it to SI.
    fn quantity(self) -> (Quantity, f64) {
        match self {
            Self::Altitude | Self::FilteredAltitude => (Quantity::AltitudeAboveGround, FOOT_M),
            Self::RawSpeed | Self::FilteredSpeed | Self::InertialSpeed => (Quantity::Speed, FOOT_M),
            Self::AccelerationG => (Quantity::Acceleration, STANDARD_GRAVITY_MPS2),
            Self::AccelerationFtS2 => (Quantity::Acceleration, FOOT_M),
            Self::InertialAltitude => (Quantity::InertialAltitude, FOOT_M),
            Self::Marker | Self::Unknown => (Quantity::Unrecognized, 1.0),
        }
    }
}

/// The column names the reader knows, but for the time and the markers, as the manuals and the
/// sample logs write them.
const KNOWN: [(&str, Kind); 21] = [
    ("Alt", Kind::Altitude),
    ("FAlt", Kind::FilteredAltitude),
    ("Filt_Alt", Kind::FilteredAltitude),
    ("KAlt", Kind::FilteredAltitude),
    ("Veloc", Kind::RawSpeed),
    ("VRaw", Kind::RawSpeed),
    ("FVeloc", Kind::FilteredSpeed),
    ("KVeloc", Kind::FilteredSpeed),
    ("VFilt", Kind::FilteredSpeed),
    ("Accel", Kind::AccelerationG),
    ("AccelG", Kind::AccelerationG),
    ("FAccelG", Kind::AccelerationG),
    ("AccelFPSS", Kind::AccelerationFtS2),
    ("FAccelFPSS", Kind::AccelerationFtS2),
    ("VAccel", Kind::InertialSpeed),
    ("VelocAcc", Kind::InertialSpeed),
    ("FAccelVel", Kind::InertialSpeed),
    ("AltAccel", Kind::InertialAltitude),
    ("AltAcc", Kind::InertialAltitude),
    ("FAccelAlt", Kind::InertialAltitude),
    // The first of the markers: every column after it is one too.
    ("LDA", Kind::Marker),
];

/// The marker names the manuals and the sample logs write, `LDA` aside; `CH`, `BO#` and `Boost#`
/// followed by digits are matched by their shape ([`kind_of`]).
const MARKERS: [&str; 11] = [
    "LowV", "Apogee", "N-O", "Drogue", "Main", "AUX", "BurnT", "CHA", "CHB", "MaxV", "MaxA",
];

/// What a column name is; `None` for one the reader doesn't know.
fn kind_of(name: &str) -> Option<Kind> {
    if let Some((_, kind)) = KNOWN.iter().find(|(known, _)| *known == name) {
        return Some(*kind);
    }
    let numbered = |prefix: &str| {
        name.strip_prefix(prefix)
            .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
    };
    (MARKERS.contains(&name) || numbered("CH") || numbered("BO#") || numbered("Boost#"))
        .then_some(Kind::Marker)
}

/// Whether a header row is an Eggtimer's: comma-separated, `T` then `Alt`, then at least one more
/// name the reader knows.
fn is_header(line: &str) -> bool {
    let mut names = line.split(',').map(str::trim);
    names.next() == Some("T")
        && names.next() == Some("Alt")
        && names.any(|name| kind_of(name).is_some())
}

/// Whether the text is an Eggtimer's download, judged from its first line that isn't blank, never
/// the file's name.
pub(crate) fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .map(|(_, line)| line.trim())
        .find(|line| !line.is_empty())
        .is_some_and(is_header)
}

/// One column of the file, as the header names it.
struct Column {
    name: String,
    kind: Kind,
}

/// Reads an Eggtimer's flight data from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't an Eggtimer header row
/// (`T`, `Alt` and a name this reader knows); [`LogError::Syntax`] for a header column with no
/// name, a row longer than the header, a cell that isn't a number, an empty time, a time that
/// doesn't increase, or rows that hold too few of the cells the header names;
/// [`LogError::NoData`] if there are no rows.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
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
                "its first line, {:?}, isn't an Eggtimer header row: `T`, `Alt` and the \
                 Eggtimer's columns, such as `Veloc` or `LDA`",
                short(header)
            ),
        });
    }
    let mut notes = Notes::default();
    let columns = header_columns(header, header_line, &mut notes)?;

    // Each row's cells as numbers, `NaN` where empty; a row leaves out the cells past its end.
    let mut rows: Vec<(usize, Vec<f64>)> = Vec::new();
    let mut cells_written = 0_usize;
    let mut blank_rows = 0_usize;
    for (number, line) in lines {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.iter().all(|cell| cell.is_empty()) {
            blank_rows += 1;
            continue;
        }
        if cells.len() > columns.len() + 1 {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells and the header names {}",
                    cells.len(),
                    columns.len() + 1
                ),
            ));
        }
        let values = row_values(&cells, &columns, number)?;
        cells_written = cells_written.saturating_add(cells.len());
        rows.push((number, values));
        if crate::log::too_sparse(columns.len() + 1, rows.len(), cells_written) {
            return Err(syntax(
                number,
                format!(
                    "{} rows hold {cells_written} of the {} cells the header's {} columns name: \
                     too few for the header to be this file's",
                    rows.len(),
                    rows.len().saturating_mul(columns.len() + 1),
                    columns.len() + 1
                ),
            ));
        }
    }
    if rows.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    build(&columns, &rows, blank_rows, notes)
}

/// The header's columns after the time, each with what it is. The time is `T` and the altitude
/// `Alt`, checked by [`is_header`].
fn header_columns(header: &str, line: usize, notes: &mut Notes) -> Result<Vec<Column>, LogError> {
    let mut columns = Vec::new();
    let mut after_lda = false;
    for (index, name) in header.split(',').map(str::trim).enumerate().skip(1) {
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        let kind = match kind_of(name) {
            Some(kind) => kind,
            None if after_lda => {
                notes.push(format!(
                    "the column {:?} isn't one this reader knows; it comes after `LDA`, where the \
                     manuals put only event markers, and was read as one",
                    short(name)
                ));
                Kind::Marker
            }
            None => {
                notes.push(format!(
                    "the column {:?} isn't one this reader knows, and was kept as written",
                    short(name)
                ));
                Kind::Unknown
            }
        };
        after_lda |= name == "LDA";
        columns.push(Column {
            name: name.to_owned(),
            kind,
        });
    }
    Ok(columns)
}

/// A row's cells as numbers in the file's units, the time first; `NaN` for an empty cell. The
/// time must be there.
fn row_values(cells: &[&str], columns: &[Column], line: usize) -> Result<Vec<f64>, LogError> {
    let mut values = Vec::with_capacity(cells.len());
    for (index, cell) in cells.iter().enumerate() {
        if cell.is_empty() {
            if index == 0 {
                return Err(syntax(
                    line,
                    "the time, the row's first cell, is empty".to_owned(),
                ));
            }
            values.push(f64::NAN);
            continue;
        }
        let Some(value) = number(cell, false) else {
            let under = match index.checked_sub(1).and_then(|at| columns.get(at)) {
                Some(column) => short(&column.name),
                None => "T".to_owned(),
            };
            return Err(syntax(
                line,
                format!(
                    "{:?} in cell {}, under {under:?}, isn't a number",
                    short(cell),
                    index + 1
                ),
            ));
        };
        values.push(value);
    }
    Ok(values)
}

/// The record from the rows read: the clock, every channel, the events and the notes.
fn build(
    columns: &[Column],
    rows: &[(usize, Vec<f64>)],
    blank_rows: usize,
    mut notes: Notes,
) -> Result<FlightLog, LogError> {
    // The time, strictly increasing, in the file's unit.
    let mut written_time = Vec::with_capacity(rows.len());
    for (number, values) in rows {
        // Invariant: `row_values` refuses a row whose time is empty, and a stored row has a cell.
        let time = values.first().copied().unwrap_or(f64::NAN);
        if let Some(&last) = written_time.last()
            && time <= last
        {
            return Err(syntax(
                *number,
                format!("the time {time} comes at or before the one above it, {last}"),
            ));
        }
        written_time.push(time);
    }
    let mut unit_notes = Vec::new();
    let units_per_second = time_unit(&written_time, &mut unit_notes);
    let time_s: Vec<f64> = written_time
        .iter()
        .map(|time| time / units_per_second)
        .collect();

    // Each column's values as written; a row that stops short leaves gaps.
    let written: Vec<Vec<f64>> = (1..=columns.len())
        .map(|index| {
            rows.iter()
                .map(|(_, values)| values.get(index).copied().unwrap_or(f64::NAN))
                .collect()
        })
        .collect();

    unit_notes.push(altitude_note(columns, &written));
    let step_s: Vec<f64> = time_s.windows(2).map(|pair| pair[1] - pair[0]).collect();
    for (index, column) in columns.iter().enumerate().skip(1) {
        if let Some(note) = unit_note(index, column, columns, &written, &step_s) {
            unit_notes.push(note);
        }
    }

    let mut log = FlightLog {
        format: LogFormat::EggtimerCsv,
        logger: "Eggtimer".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: time_s.clone(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::with_capacity(columns.len()),
        events: Vec::new(),
        notes: unit_notes,
    };
    for (column, values) in columns.iter().zip(&written) {
        let gaps = values.iter().filter(|value| value.is_nan()).count();
        if gaps > 0 {
            notes.push(format!(
                "the column {:?} leaves {gaps} cells empty: they are gaps",
                short(&column.name)
            ));
        }
        let (quantity, factor) = column.kind.quantity();
        log.channels.push(Channel {
            column: column.name.clone(),
            quantity,
            time_s: time_s.clone(),
            values: values.iter().map(|value| value * factor).collect(),
        });
    }

    // The events: each run of non-zero samples in a marker column, at its first sample, in the
    // file's order, row by row.
    let markers: Vec<usize> = columns
        .iter()
        .enumerate()
        .filter(|(_, column)| column.kind == Kind::Marker)
        .map(|(index, _)| index)
        .collect();
    let marked = |value: f64| value != 0.0 && !value.is_nan();
    for (row, time) in time_s.iter().enumerate() {
        for &index in &markers {
            let value = written[index][row];
            let before = row
                .checked_sub(1)
                .is_some_and(|above| marked(written[index][above]));
            if marked(value) && !before {
                log.events.push(Event {
                    time_s: *time,
                    name: columns[index].name.clone(),
                });
            }
        }
    }

    // The primary track: `Alt`, and the barometer's raw speed, or its filtered one.
    log.altitude_m = log.channels[0].values.clone();
    let speed = |kind: Kind| {
        columns
            .iter()
            .position(|column| column.kind == kind)
            .map(|index| (index, log.channels[index].values.clone()))
    };
    if let Some((_, values)) = speed(Kind::RawSpeed) {
        log.vertical_speed_m_s = Some(values);
    } else if let Some((index, values)) = speed(Kind::FilteredSpeed) {
        log.notes.push(format!(
            "the file has no raw velocity, `Veloc`; the vertical speed is its filtered barometric \
             {:?}",
            columns[index].name
        ));
        log.vertical_speed_m_s = Some(values);
    }
    if blank_rows > 0 {
        log.notes.push(format!(
            "{blank_rows} rows hold only empty cells, and were skipped"
        ));
    }
    log.notes.extend(notes.into_vec());
    Ok(log)
}

/// The time's units in a second, 1 or 1,000, decided from the median step between rows, with the
/// note that says how. Dividing by 1,000 keeps a whole number of milliseconds the nearest `f64`
/// to its seconds, where multiplying by 0.001 can miss it by one step.
fn time_unit(written: &[f64], unit_notes: &mut Vec<String>) -> f64 {
    let mut steps: Vec<f64> = written.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if steps.is_empty() {
        unit_notes.push(
            "`T` read in seconds, assumed: the file holds one row, so there is no step between \
             rows to tell seconds from milliseconds"
                .to_owned(),
        );
        return 1.0;
    }
    steps.sort_by(f64::total_cmp);
    let middle = steps.len() / 2;
    let median = if steps.len().is_multiple_of(2) {
        (steps[middle - 1] + steps[middle]) / 2.0
    } else {
        steps[middle]
    };
    let (unit, per_second) = if median > MILLISECONDS_ABOVE {
        ("milliseconds", 1000.0)
    } else {
        ("seconds", 1.0)
    };
    unit_notes.push(format!(
        "`T` read in {unit}, from the step: the median step between rows is {}, and the reader \
         reads one above 5 as milliseconds (at the manuals' 1 to 33 samples a second, a step is \
         0.03 to 1 in seconds and 30 to 1,000 in milliseconds)",
        plain(median, 4)
    ));
    per_second
}

/// The note saying how `Alt` was read in feet; for a file with the accelerometer in both g and
/// ft/s², what the two say of each other.
fn altitude_note(columns: &[Column], written: &[Vec<f64>]) -> String {
    let mut note = "`Alt` read in feet, from the manual: the Classic's and TRS's manuals (p37) \
                    give the altitude in feet, the Quantum's and Proton's set every altitude in \
                    feet, and no Eggtimer manual offers another unit"
        .to_owned();
    let find = |name: &str| {
        columns
            .iter()
            .position(|column| column.name == name)
            .and_then(|index| written.get(index))
    };
    if let (Some(g), Some(ft_s2)) = (find("AccelG").or_else(|| find("Accel")), find("AccelFPSS")) {
        // The least-squares ratio of the two through zero.
        let (mut product, mut square) = (0.0, 0.0);
        for (g, ft_s2) in g.iter().zip(ft_s2) {
            if g.is_finite() && ft_s2.is_finite() {
                product += g * ft_s2;
                square += g * g;
            }
        }
        let ratio = product / square;
        if ratio.is_finite() {
            note.push_str(&format!(
                "; the file's `AccelFPSS` is {} times its `AccelG`, and one g is 32.17 ft/s²",
                plain(ratio, 2)
            ));
        }
    }
    note
}

/// The note saying how a length or speed column other than `Alt` was read, `None` for one whose
/// name states its unit, or that is kept as written. `index` is the column's among `columns`.
fn unit_note(
    index: usize,
    column: &Column,
    columns: &[Column],
    written: &[Vec<f64>],
    step_s: &[f64],
) -> Option<String> {
    let name = &column.name;
    let note = match column.kind {
        Kind::FilteredAltitude => format!(
            "`{name}` read in feet, from the manual: it is the logger's filtered `Alt` (Quantum \
             p38, Proton p45), in the same unit"
        ),
        Kind::RawSpeed => speed_note(name, "Alt", &written[index], &written[0], step_s),
        Kind::FilteredSpeed => match columns
            .iter()
            .position(|other| other.kind == Kind::FilteredAltitude)
        {
            Some(altitude) => speed_note(
                name,
                &columns[altitude].name,
                &written[index],
                &written[altitude],
                step_s,
            ),
            None => format!(
                "`{name}` read in feet per second, from the manual: the logger's filtered \
                 velocity, which the Excel guide (p3, p7) graphs in ft/sec"
            ),
        },
        Kind::InertialSpeed => format!(
            "`{name}` read in feet per second, from the manual: the Proton's manual (p45) gives \
             the accelerometer's velocity in ft/sec"
        ),
        Kind::InertialAltitude => format!(
            "`{name}` read in feet, from the manual: the Proton's manual (p45) gives the \
             accelerometer's distance traveled in ft"
        ),
        Kind::Altitude
        | Kind::AccelerationG
        | Kind::AccelerationFtS2
        | Kind::Marker
        | Kind::Unknown => return None,
    };
    Some(note)
}

/// The note saying a speed was read in feet per second because it is its altitude's change over
/// the step, with how many rows bear that out.
fn speed_note(
    name: &str,
    altitude_name: &str,
    speed: &[f64],
    altitude: &[f64],
    step_s: &[f64],
) -> String {
    let (mut rows, mut matching) = (0_usize, 0_usize);
    for (row, step) in step_s.iter().enumerate() {
        let (Some(&low), Some(&high), Some(&value)) =
            (altitude.get(row), altitude.get(row + 1), speed.get(row + 1))
        else {
            continue;
        };
        let change = (high - low) / step;
        if !(change.is_finite() && value.is_finite()) {
            continue;
        }
        rows += 1;
        if (value - change).abs() <= (SPEED_MATCH_RELATIVE * change.abs()).max(SPEED_MATCH_FT_S) {
            matching += 1;
        }
    }
    format!(
        "`{name}` read in feet per second, from the altitude: it is the change in \
         `{altitude_name}` over the step, within 1 % or 1 ft/s, in {matching} of {rows} rows \
         after the first"
    )
}

/// A number as a note writes it: rounded to `decimals` places, trailing zeros dropped.
fn plain(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        text
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

    const TRS: &str = "T,Alt,Veloc,FAlt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main";

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    /// The text before a note's first `: `, which must be the same for every file decided the
    /// same way.
    fn prefix(note: &str) -> &str {
        note.split(": ").next().unwrap_or(note)
    }

    #[test]
    fn recognizes_the_eggtimer_headers_and_no_others() {
        for header in [
            TRS,
            "T,Alt,Veloc,Filt_Alt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main",
            "T,Alt,FAlt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main,AUX",
            "T,Alt,Veloc,KAlt,KVeloc,LDA,BurnT,LowV,Apogee,N-O,CHA,CHB",
            "T, Alt, VRaw, VFilt",
            "T,Alt,Veloc,AccelG,AccelFPSS,VAccel,AltAccel,FAlt,FVeloc,FAccelG,FAccelFPSS,\
             FAccelVel,FAccelAlt,LDA,MaxV,MaxA,BO#1,LowV,Apogee,N-O,CH1,CH2",
            "T,Alt,Apogee",
        ] {
            assert!(recognizes(&format!("\r\n{header}\r\n0,0\r\n")), "{header}");
        }
        for text in [
            "",
            "T,Alt\n",
            "T,Alt,Foo\n",
            "T\tAlt\tVeloc\n",
            "t,alt,veloc\n",
            "Time,Altitude,Velocity\n",
            "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages\n",
            "Alt,T,Veloc\n",
            "# T,Alt,Veloc\n",
        ] {
            assert!(!recognizes(text), "{text:?}");
        }
        assert_eq!(kind_of("CH12"), Some(Kind::Marker));
        assert_eq!(kind_of("Boost#3"), Some(Kind::Marker));
        assert_eq!(kind_of("CH"), None);
        assert_eq!(kind_of("BO#x"), None);
    }

    /// The TRS layout in seconds: feet and feet per second to SI, the markers kept as written and
    /// read as events, the raw barometric velocity the vertical speed.
    #[test]
    fn reads_the_layout_in_seconds() {
        let text = format!(
            "{TRS}\n\
             0,0,0,0,0,0,0,0,0,0,0\n\
             0.05,10,200,2,40,0,0,0,0,0,0\n\
             0.1,25,300,7,100,25,0,0,0,0,0\n\
             0.15,30,100,13.5,130,0,0,30,0,0,0\n\
             0.2,29,-20,18,90,0,0,0,29,29,0\n"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::EggtimerCsv);
        assert_eq!(log.logger, "Eggtimer");
        assert_eq!(log.time_s, [0.0, 0.05, 0.1, 0.15, 0.2]);
        assert!(close(log.altitude_m[2], 25.0 * 0.3048));
        let speed = log.vertical_speed_m_s.as_deref().unwrap();
        assert!(close(speed[1], 200.0 * 0.3048));
        assert_eq!(log.channels.len(), 10);
        assert_eq!(log.channels[2].quantity, Quantity::AltitudeAboveGround);
        assert!(close(log.channels[2].values[3], 13.5 * 0.3048));
        assert_eq!(log.channels[3].quantity, Quantity::Speed);
        // The markers as written, in feet: 0 is "no event".
        assert_eq!(log.channels[4].column, "LDA");
        assert_eq!(log.channels[4].quantity, Quantity::Unrecognized);
        assert_eq!(log.channels[4].values, [0.0, 0.0, 25.0, 0.0, 0.0]);
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|event| (event.time_s, event.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [
                (0.1, "LDA"),
                (0.15, "Apogee"),
                (0.2, "N-O"),
                (0.2, "Drogue")
            ]
        );
        assert_eq!(log.notes.len(), 5, "{:?}", log.notes);
        assert_eq!(
            log.notes[0],
            "`T` read in seconds, from the step: the median step between rows is 0.05, and the \
             reader reads one above 5 as milliseconds (at the manuals' 1 to 33 samples a second, \
             a step is 0.03 to 1 in seconds and 30 to 1,000 in milliseconds)"
        );
        assert!(log.notes[1].starts_with("`Alt` read in feet, from the manual: "));
        assert_eq!(
            log.notes[2],
            "`Veloc` read in feet per second, from the altitude: it is the change in `Alt` over \
             the step, within 1 % or 1 ft/s, in 4 of 4 rows after the first"
        );
        assert!(log.notes[3].starts_with("`FAlt` read in feet, from the manual: "));
        assert!(
            log.notes[4].starts_with("`FVeloc` read in feet per second, from the altitude: ")
                && log.notes[4].ends_with("in 4 of 4 rows after the first"),
            "{}",
            log.notes[4]
        );
    }

    /// A median step above 5 is milliseconds; at 5 or below, seconds. The notes' text before the
    /// first `: ` is the same for files decided the same way.
    #[test]
    fn the_step_decides_the_time_unit() {
        let rows = |step: f64| -> String {
            (0..4)
                .map(|i| format!("{},{},{}\n", f64::from(i) * step, i * 10, 0))
                .collect()
        };
        let millis = read(&format!("T,Alt,LDA\n{}", rows(50.0))).unwrap();
        assert_eq!(millis.time_s, [0.0, 0.05, 0.1, 0.15]);
        assert!(millis.notes[0].starts_with("`T` read in milliseconds, from the step: "));
        assert!(millis.notes[0].contains("median step between rows is 50,"));
        let slow = read(&format!("T,Alt,LDA\n{}", rows(1000.0))).unwrap();
        assert_eq!(prefix(&slow.notes[0]), prefix(&millis.notes[0]));
        assert!(slow.notes[0].contains("is 1000,"));

        let at_the_line = read(&format!("T,Alt,LDA\n{}", rows(5.0))).unwrap();
        assert_eq!(at_the_line.time_s, [0.0, 5.0, 10.0, 15.0]);
        let above = read(&format!("T,Alt,LDA\n{}", rows(5.01))).unwrap();
        assert!(close(above.time_s[1], 0.00501));
        let seconds = read(&format!("T,Alt,LDA\n{}", rows(0.5))).unwrap();
        assert_eq!(
            prefix(&seconds.notes[0]),
            "`T` read in seconds, from the step"
        );
        assert_eq!(prefix(&at_the_line.notes[0]), prefix(&seconds.notes[0]));

        // The median, not the mean: a long pause doesn't turn seconds into milliseconds.
        let paused = read("T,Alt,LDA\n0,0,0\n0.05,1,0\n0.1,2,0\n100,3,0\n").unwrap();
        assert_eq!(paused.time_s, [0.0, 0.05, 0.1, 100.0]);

        let one = read("T,Alt,LDA\n120,4,0\n").unwrap();
        assert_eq!(one.time_s, [120.0]);
        assert!(one.notes[0].starts_with("`T` read in seconds, assumed: "));
    }

    /// The Proton's columns: g and ft/s² to m/s², the accelerometer's velocity and distance, the
    /// two accelerations' ratio in the altitude's note; `MaxV` and `MaxA` markers as written.
    #[test]
    fn reads_the_accelerometer_columns() {
        let text = "T,Alt,Veloc,AccelG,AccelFPSS,VAccel,AltAccel,LDA,MaxV,MaxA,CH1\r\
                    -0.1,0,0,0,0,0,0,0,0,0,0\r\
                    0,1,10,2,64.4,0,0,0,0,0,0\r\
                    0.1,12,110,10,321.7,40,2,0,0,321,0\r\
                    0.2,40,280,5,160.9,80,9,40,80,0,0\r";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [-0.1, 0.0, 0.1, 0.2]);
        let quantities: Vec<Quantity> = log.channels.iter().map(|c| c.quantity).collect();
        assert_eq!(
            quantities,
            [
                Quantity::AltitudeAboveGround,
                Quantity::Speed,
                Quantity::Acceleration,
                Quantity::Acceleration,
                Quantity::Speed,
                Quantity::InertialAltitude,
                Quantity::Unrecognized,
                Quantity::Unrecognized,
                Quantity::Unrecognized,
                Quantity::Unrecognized,
            ]
        );
        assert!(close(log.channels[2].values[2], 10.0 * 9.806_65));
        assert!(close(log.channels[3].values[2], 321.7 * 0.3048));
        assert!(close(log.channels[4].values[3], 80.0 * 0.3048));
        assert!(close(log.channels[5].values[3], 9.0 * 0.3048));
        assert_eq!(log.channels[7].values, [0.0, 0.0, 0.0, 80.0]);
        let names: Vec<&str> = log.events.iter().map(|event| event.name.as_str()).collect();
        assert_eq!(names, ["MaxA", "LDA", "MaxV"]);
        assert!(
            log.notes[1].ends_with(
                "; the file's `AccelFPSS` is 32.17 times its `AccelG`, and one g is 32.17 ft/s²"
            ),
            "{}",
            log.notes[1]
        );
        assert!(log.notes[3].starts_with("`VAccel` read in feet per second, from the manual: "));
        assert!(log.notes[4].starts_with("`AltAccel` read in feet, from the manual: "));
        assert_eq!(log.notes.len(), 5, "{:?}", log.notes);
    }

    /// A marker held over several samples, or to the end, is one event; two runs are two.
    #[test]
    fn each_run_of_a_marker_is_one_event() {
        let text = "T,Alt,Veloc,Apogee,Main\n\
                    0,100,0,0,0\n0.5,101,2,101,0\n1,100,-2,101,0\n1.5,99,-2,0,0\n\
                    2,98,-2,98,98\n2.5,97,-2,0,98\n";
        let log = read(text).unwrap();
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|event| (event.time_s, event.name.as_str()))
            .collect();
        assert_eq!(events, [(0.5, "Apogee"), (2.0, "Apogee"), (2.0, "Main")]);
    }

    /// Without a raw velocity, the filtered one is the vertical speed, with a note; without a
    /// filtered altitude, its unit is the manual's.
    #[test]
    fn the_filtered_velocity_stands_in_for_the_raw_one() {
        let log = read("T,Alt,FAlt,FVeloc,LDA\n0,0,0,0,0\n0.05,2,1,20,0\n").unwrap();
        assert_eq!(
            log.vertical_speed_m_s.as_deref(),
            Some(&[0.0, 20.0 * 0.3048][..])
        );
        assert!(
            log.notes
                .iter()
                .any(|note| note
                    .contains("the vertical speed is its filtered barometric \"FVeloc\""))
        );
        let old = read("T, Alt, VRaw, VFilt\n0, 0, 0, 0\n50, 1, 20, 4\n100, 3, 40, 12\n").unwrap();
        assert_eq!(old.time_s, [0.0, 0.05, 0.1]);
        assert_eq!(
            old.vertical_speed_m_s.as_deref(),
            Some(&[0.0, 20.0 * 0.3048, 40.0 * 0.3048][..])
        );
        assert!(old.notes[2].starts_with("`VRaw` read in feet per second, from the altitude: "));
        assert!(old.notes[3].starts_with("`VFilt` read in feet per second, from the manual: "));
        assert!(
            read("T,Alt,LDA\n0,0,0\n")
                .unwrap()
                .vertical_speed_m_s
                .is_none()
        );
    }

    /// An unknown column before `LDA` is kept as written; after it, it is a marker with events.
    #[test]
    fn unknown_columns_are_kept_and_named_channels_are_markers() {
        let log = read("T,Alt,Veloc,Baro,LDA,Apogee,Drogue.1\n0,0,0,7,0,0,0\n0.05,4,80,8,4,0,4\n")
            .unwrap();
        assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
        assert_eq!(log.channels[2].values, [7.0, 8.0]);
        let names: Vec<&str> = log.events.iter().map(|event| event.name.as_str()).collect();
        assert_eq!(names, ["LDA", "Drogue.1"]);
        let notes = log.notes.join("\n");
        assert!(notes.contains("\"Baro\" isn't one this reader knows, and was kept as written"));
        assert!(notes.contains("\"Drogue.1\" isn't one this reader knows; it comes after `LDA`"));
    }

    /// Rows of empty cells are skipped and counted; an empty cell and a short row leave gaps.
    #[test]
    fn empty_rows_are_skipped_and_empty_cells_are_gaps() {
        let text = format!(
            "{TRS}\r\n0,0,0,0,0,0,0,0,0,0,0\r\n0.05,2,,1,20,0,0,0,0,0,0\r\n0.1,5,60\r\n\
             ,,,,,,,,,,\r\n,,,,,,,,,,\r\n , ,\r\n"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.time_s, [0.0, 0.05, 0.1]);
        assert!(log.channels[1].values[1].is_nan());
        assert!(log.channels[2].values[2].is_nan());
        let notes = log.notes.join("\n");
        assert!(
            notes.contains("3 rows hold only empty cells, and were skipped"),
            "{notes}"
        );
        assert!(notes.contains("the column \"Veloc\" leaves 1 cells empty: they are gaps"));
        assert!(notes.contains("the column \"Main\" leaves 1 cells empty: they are gaps"));
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let cases = [
            (String::new(), "the file is empty"),
            ("\r\n\r\n".to_owned(), "the file is empty"),
            (
                "Time,Altitude\n0,0\n".to_owned(),
                "isn't an Eggtimer header row",
            ),
            (format!("{TRS}\n"), "no data rows"),
            (format!("{TRS}\n,,,,,,,,,,\n"), "no data rows"),
            (
                "T,Alt,,LDA\n".to_owned(),
                "line 1: the header's column 3 has no name",
            ),
            (
                "T,Alt,LDA\n0,0,0,0\n".to_owned(),
                "line 2: the row has 4 cells and the header names 3",
            ),
            (
                "T,Alt,LDA\n0,0,0\n0.05,x,0\n".to_owned(),
                "line 3: \"x\" in cell 2, under \"Alt\", isn't a number",
            ),
            (
                "T,Alt,LDA\n0,0,0\n0.05,1,NaN\n".to_owned(),
                "line 3: \"NaN\" in cell 3, under \"LDA\"",
            ),
            (
                "T,Alt,LDA\nx,0,0\n".to_owned(),
                "line 2: \"x\" in cell 1, under \"T\"",
            ),
            (
                "T,Alt,LDA\n,0,0\n".to_owned(),
                "line 2: the time, the row's first cell, is empty",
            ),
            (
                "T,Alt,LDA\n0,0,0\n0,1,0\n".to_owned(),
                "line 3: the time 0 comes at or before the one above it, 0",
            ),
            (
                "T,Alt,LDA\n\n1,0,0\n\n0.5,1,0\n".to_owned(),
                "line 5: the time 0.5 comes at or before",
            ),
        ];
        for (text, expected) in cases {
            let error = read(&text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
    }

    /// A wide header over short rows is refused before its columns are allocated.
    #[test]
    fn a_wide_header_over_short_rows_is_refused() {
        let names: String = (0..2000).map(|i| format!(",u{i}")).collect();
        let rows = "0\n".repeat(200);
        let text = format!("T,Alt,LDA{names}\n{rows}");
        let error = read(&text).unwrap_err().to_string();
        assert!(
            error.contains("too few for the header to be this file's"),
            "{error}"
        );
    }

    /// CR, LF, CR LF and CR CR LF line ends read the same.
    #[test]
    fn line_ends() {
        let rows = ["T,Alt,Veloc,LDA", "0,0,0,0", "0.05,2,40,0", "0.1,6,80,6"];
        let reference = read(&rows.join("\n")).unwrap();
        for end in ["\r", "\n", "\r\n", "\r\r\n"] {
            let text = format!("{}{end}", rows.join(end));
            assert_eq!(read(&text).unwrap(), reference, "{end:?}");
        }
        assert_eq!(reference.events.len(), 1);
    }

    /// A header of many unknown columns keeps a short list of notes and counts the rest; the unit
    /// notes stay outside the cap.
    #[test]
    fn notes_stay_few() {
        let unknown: String = (0..100).map(|i| format!(",u{i}")).collect();
        let text = format!("T,Alt,Veloc{unknown}\n0,0,0{}\n", ",1".repeat(100));
        let log = read(&text).unwrap();
        assert_eq!(log.channels.len(), 102);
        // `T`, `Alt` and `Veloc`'s unit notes, 20 notes about columns, and one counting the rest.
        assert_eq!(log.notes.len(), 3 + 20 + 1, "{:?}", log.notes);
        assert!(
            log.notes[23].starts_with("and 80 more notes"),
            "{:?}",
            log.notes[23]
        );
    }

    /// A file that isn't text is refused with a short quote of it.
    #[test]
    fn a_long_line_is_quoted_short() {
        let junk = "\u{fffd}".repeat(100_000);
        let error = read(&junk).unwrap_err().to_string();
        assert!(error.len() < 400, "{} bytes", error.len());
    }

    #[test]
    fn numbers_are_written_plainly() {
        assert_eq!(plain(0.050_000_000_000_000_71, 4), "0.05");
        assert_eq!(plain(50.0, 4), "50");
        assert_eq!(plain(32.166, 2), "32.17");
        assert_eq!(plain(-0.0, 2), "-0");
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has increasing times and every
        /// channel a value for each.
        #[test]
        fn any_text_is_read_or_refused(
            rows in proptest::collection::vec("[-0-9.,x \r]{0,30}", 0..12)
        ) {
            let text = format!("{TRS}\n{}", rows.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
                for event in &log.events {
                    proptest::prop_assert!(log.time_s.contains(&event.time_s));
                }
            }
        }
    }
}
