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
//! that name in the header (Proton p26; *Eggtimer Proton Release Notes*, version 2.01C, p2), so a
//! marker column may carry any name.
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
//! | `Alt` | [`Quantity::AltitudeAboveGround`], ft | the Classic and TRS manuals (p37) state feet; the Quantum and Proton manuals state no unit for the download, but give every height in feet (Quantum p24: launch detect "50’-500’"), and no Eggtimer manual offers meters |
//! | `FAlt`, `Filt_Alt`, `KAlt` | [`Quantity::AltitudeAboveGround`], ft: the logger's filtered `Alt` | Quantum p38, Proton p45; Classic p37 ("the two K fields are filtered data") |
//! | `Veloc`, `VRaw` | [`Quantity::Speed`], ft/s: the change in `Alt` over the step | Classic and TRS p37 ("velocity in ft/sec"); Quantum p38, Proton p45; checked against `Alt` (below) |
//! | `FVeloc`, `KVeloc`, `VFilt` | [`Quantity::Speed`], ft/s: the change in the filtered altitude over the step | Quantum p38, Proton p46; the Excel guide (p3, p7: "Velocity in ft/sec"); checked against the filtered altitude (below) |
//! | `Accel` | [`Quantity::Acceleration`], g × 9.806 65 m/s² (standard gravity) | Proton p45 ("in G's") |
//! | `AccelG`, `FAccelG` | [`Quantity::Acceleration`], g × 9.806 65 m/s² | the name; Proton p45 |
//! | `AccelFPSS`, `FAccelFPSS` | [`Quantity::Acceleration`], ft/s² | Proton p45 ("ft/sec^2"); the name |
//! | `VAccel`, `VelocAcc`, `FAccelVel` | [`Quantity::Speed`], ft/s, from the accelerometer | Proton p45 ("the velocity derived from the accelerometer in ft/sec") |
//! | `AltAccel`, `AltAcc`, `FAccelAlt` | [`Quantity::InertialAltitude`], ft | Proton p45 ("the DISTANCE TRAVELED derived from the accelerometer in ft.") |
//! | `LDA`, `LowV`, `Apogee`, `N-O`, `Drogue`, `Main`, `AUX`, `BurnT`, `CHA`, `CHB`, `CH1`…, `MaxV`, `MaxA`, `Boost#1`…, `BO#1`…, and any column after `LDA` | [`Quantity::Unrecognized`], as written: an event marker (below) | Quantum p38–39, Proton p46 |
//!
//! No Eggtimer manual offers a unit other than feet: the Classic's, TRS's, Quantum's and Proton's
//! manuals and release notes name no meter or metric setting. Each column whose name doesn't state
//! its unit gets a note on the record saying how it was read, once per name.
//!
//! **Speeds checked.** A speed is read in feet per second only if the rows bear out that it is its
//! altitude's change over the step: `Veloc` and `VRaw` of `Alt`, `FVeloc`, `KVeloc` and `VFilt` of
//! the first filtered altitude, within 1 % or 1 ft/s, on at least 90 % of the rows after the first.
//! A speed that falls short is kept as written, [`Quantity::Unrecognized`], with a note; a file of
//! one row, or a filtered speed with no filtered altitude, has nothing to check, and is read as the
//! manual says. As of 2026-10-12, over the 43 Eggtimer files among the project's sample logs, the
//! least share is 95.9 %, and with the clock read in the wrong unit, a factor of 1,000 off, at
//! most 9.5 %: the check also guards the time's unit (below).
//!
//! **The accelerometer.** As of 2026-10-12, in the 4 of those files with an accelerometer,
//! `AccelFPSS` is about 32 times `AccelG` (32.17 ft/s² is one g), `VAccel` tracks the sum of
//! `AccelFPSS` over the steps, and `AccelG`'s median before the start of flight (`T` below 0) is
//! 0 to 0.2 g. Standing on the pad, an accelerometer that keeps gravity reads 1 g, so the Proton's
//! acceleration has gravity taken out. Its manual doesn't say so; its release notes have it
//! calibrate the accelerometer as it is armed, standing up (2.01C, p8). A note on the record says
//! whether this file's acceleration holds gravity, from its own pad reading: taken out below
//! 0.5 g, held within 0.5 g of 1 g, not known otherwise or with no rows before the start of
//! flight. The values are kept as written either way. The `F`-prefixed accelerometer columns are
//! read as their unfiltered namesakes, as `FAlt` is `Alt` filtered: no manual describes them.
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
//! `AccelFPSS` on their row. A marker may also be held from its event to the end of the file, as 3
//! marker columns among the 43 sample files are (as of 2026-10-12). Each marker column is kept as a
//! channel as written, [`Quantity::Unrecognized`]: its 0 means "no event", not 0 ft, so converting
//! it as a height would mislead. One note on the record lists them. Each run of non-zero samples is
//! an [`Event`] named as the column (a name past 80 characters cut short, as a note quotes it), at
//! the run's first sample, up to [`MAX_EVENTS_PER_MARKER`] a column; past that, the rises are
//! counted in a note. An event that happened at a raw altitude of exactly 0 ft can't be told from
//! no event, and is lost. A column after `LDA` whose name this reader doesn't know, such as a
//! Proton channel's own name, is read as a marker, with a note.
//!
//! **The primary track.** [`FlightLog::altitude_m`] is `Alt`, the barometer's raw altitude above
//! the logger's zero. The vertical speed is `Veloc` (or `VRaw`), the barometer's; without it, or
//! with it kept as written because its check failed, the filtered `FVeloc` (or `KVeloc`, `VFilt`),
//! also the barometer's, with a note. `VAccel` is the
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
//!
//! **The terminal download.** A TRS's flight can also be downloaded in a terminal program, which
//! prints three parts: the flight settings, the summary data ("apogee, length of flight, maximum
//! velocity, etc.") and the detail data, the table above (*Eggtimer TRS Flight Computer User's
//! Manual*, firmware 2.07, p35). The manual shows the first two over a page (p36) and the table
//! after them (p37); the file the terminal captures holds all three, as [`LogFormat::EggtimerText`]:
//!
//! ```text
//! Flight Summary:
//! Eggtimer TRS 2.07    Flight Settings
//! ...Drogue <1=Normal,2=Backup,0=OFF>...1
//! ...Main Altitude <50-2000 by 50, 0=OFF>...600
//! ...Frequency (Ctrl-F to change)...915
//! ...ID Code...3
//! ...MY EGGTIMER
//! ASL Alt.: 1200 ft
//! Base Temp: 70.0 F
//! Apogee: 1500 ft
//! T @ Apogee: 9.500
//! Max Velocity: 400 ft/sec
//! T @ Max Velocity: 1.300
//! Altitude @ Main Deploy: 590 ft
//! T @ Main Deploy: 40.000
//! T,Alt,Veloc,Filt_Alt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main
//! 0,0,0.00,0.00,0.00,0,0,0,0,0,0
//! ```
//!
//! Firmware 2.10 writes `Main Alt`, `Freq`, `Max Veloc`, `Alt @ …` and `T @ Drogue` where 2.07
//! writes `Main Altitude`, `Frequency`, `Max Velocity`, `Altitude @ …` and `T @ Drogue Deploy`.
//!
//! A file is this layout if a title, a line `Eggtimer <model> <firmware>` ending in `Flight
//! Settings` (or followed by a line `Flight Settings`, as the manual's example prints it, p36), comes
//! before an Eggtimer header row, both within the first [`MAX_PREAMBLE_LINES`] lines that aren't
//! blank. A `Flight Summary:` heading may come before the title; any other line there is counted
//! in a note. The CSV download opens with its header row, so it is never this layout.
//!
//! What the reader keeps:
//!
//! - **The table**, read as [`read`] reads the CSV download, every rule above the same, its time's
//!   unit decided by its step; an error names the file's line.
//! - **The title**: the logger, such as `Eggtimer TRS`, and its last word, if it starts with a
//!   digit, as the firmware.
//! - **The flight settings**, the lines `...<setting>...<value>`, listed as written in one note.
//!   A settings line with no value is the name the owner gave the flight computer (the manual's
//!   settings screen shows `MY EGGTIMER`, p23): free text that may name a person, it is left out
//!   of the record and only counted.
//! - **`ASL Alt.`**, "the measured ASL altitude of the launch site" (p36), as
//!   [`Stated::ground_elevation_msl_m`], and **`Apogee`** as [`Stated::apogee_m`]: each read by
//!   its unit mark, `ft` (or `m`), with a note; without one it is kept as written. In both sample
//!   files the stated apogee is the table's highest `Alt`, so it is above the logger's zero.
//! - **The summary's times**, `T @ …` and `Time @ LDA`, as [`Event`]s, in seconds: the manual's
//!   download writes them in `secs.` and its LDA at 0.700 secs. is "700ms into the flight" (p36,
//!   p37); the sample files leave the mark out. A time that is its marker's event in the table,
//!   within half a millisecond, is that event, kept once: `Apogee` is `Apogee`'s, `LDA` `LDA`'s,
//!   `Unlock` `LowV`'s (the end of the Mach lockout, p37), `N-O` and `Nose-Over` `N-O`'s, `Drogue`
//!   and `Drogue Deploy` `Drogue`'s, `Main` and `Main Deploy` `Main`'s. As of 2026-10-13, in both
//!   sample files, every such time is its marker's to the millisecond. Any other time, such as `T
//!   @ Max Velocity`, or one that differs from its marker's, is an event of its own named as the
//!   summary names it, a difference noted. A time of 0 is no event: the manual writes zeroes for a
//!   channel that didn't fire (p24). The events are in time order, the table's first where two
//!   share a time.
//! - **The other summary lines**, listed as written in one note: the base temperature with its
//!   unit mark (`F` in the sample files, `C` in the manual's example), the flight time, the beep
//!   code, the top speed and acceleration, and the heights at each event; `Last Flight Phase` with
//!   its meaning, from the manual's codes (p37).
//!
//! Each note lists at most 40 lines, the rest counted. The summary's figures beside the table's
//! are the logger's own and are never put in place of HPR Sim's readings.

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

/// The least share of the rows after the first on which a speed must be its altitude's change
/// over the step for the reader to read it in feet per second; below it, the column is kept as
/// written. As of 2026-10-12, over the 43 Eggtimer files among the project's sample logs, the
/// least share is 95.9 %, and with the clock read in the wrong unit, a factor of 1,000 off, the
/// most is 9.5 %: 90 % sits between the two, near the first.
const SPEED_MATCH_SHARE: f64 = 0.9;

/// The most events one marker column gives; past it, the column's rises are counted in a note
/// instead. A marker marks one event of the flight (Quantum p38, Proton p46): as of 2026-10-12, no
/// marker column among the 43 Eggtimer files in the project's sample logs rises more than once.
/// The cap keeps a file whose marker flips on every row from filling the record with events.
pub const MAX_EVENTS_PER_MARKER: usize = 16;

/// The pad reading, in g, below which an acceleration is taken to have gravity out, and within
/// which of 1 g to have it in: halfway between the two.
const GRAVITY_READING_G: f64 = 0.5;

/// The most names a note lists; past it, the rest are counted.
const MAX_LISTED: usize = 6;

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

impl Column {
    /// Whether the column's name states its unit: `AccelG` and `FAccelG` name g, `AccelFPSS` and
    /// `FAccelFPSS` ft/s². No other name the reader knows does.
    fn names_its_unit(&self) -> bool {
        match self.kind {
            Kind::AccelerationG => self.name.ends_with('G'),
            Kind::AccelerationFtS2 => self.name.ends_with("FPSS"),
            _ => false,
        }
    }
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
    build(columns, &rows, blank_rows, notes)
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
    mut columns: Vec<Column>,
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
    let step_s: Vec<f64> = time_s.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let checks = speed_checks(&mut columns, &written, &step_s, &mut notes);

    // One note per name decided, so a header repeating a name doesn't repeat its note.
    unit_notes.push(altitude_note(&columns, &written));
    let mut noted: Vec<&str> = Vec::new();
    for (column, check) in columns.iter().zip(&checks) {
        if noted.contains(&column.name.as_str()) {
            continue;
        }
        if let Some(note) = unit_note(column, *check, &columns) {
            noted.push(&column.name);
            unit_notes.push(note);
        }
    }
    unit_notes.extend(gravity_note(&columns, &written, &written_time));
    unit_notes.extend(marker_note(&columns));

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
    log.events = events(&columns, &written, &time_s, &mut notes);

    // The primary track: `Alt`, and the barometer's raw speed, or its filtered one. Invariant:
    // `Alt` is the header's name after `T` (`is_header`), so it is the first channel.
    log.altitude_m = log
        .channels
        .first()
        .map(|channel| channel.values.clone())
        .unwrap_or_default();
    let speed = |kind: Kind| {
        columns
            .iter()
            .zip(&log.channels)
            .find(|(column, _)| column.kind == kind)
            .map(|(column, channel)| (column.name.as_str(), channel.values.clone()))
    };
    if let Some((_, values)) = speed(Kind::RawSpeed) {
        log.vertical_speed_m_s = Some(values);
    } else if let Some((name, values)) = speed(Kind::FilteredSpeed) {
        log.notes.push(format!(
            "the file has no raw velocity, `Veloc`, read as a speed; the vertical speed is its \
             filtered barometric {name:?}"
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

/// What the rows say of a speed: the column of the altitude it should be the change of, and on
/// how many of the rows after the first it is, of how many can be checked.
#[derive(Debug, Clone, Copy)]
struct SpeedCheck {
    altitude: usize,
    matching: usize,
    rows: usize,
}

/// Each speed checked against the altitude it should be the change of: `Veloc` and `VRaw` against
/// `Alt`, `FVeloc`, `KVeloc` and `VFilt` against the file's first filtered altitude. A speed that
/// matches on fewer than [`SPEED_MATCH_SHARE`] of the rows checked is kept as written, with a
/// note; the others get their check, `None` for a column that isn't a speed or has no altitude to
/// check it against.
fn speed_checks(
    columns: &mut [Column],
    written: &[Vec<f64>],
    step_s: &[f64],
    notes: &mut Notes,
) -> Vec<Option<SpeedCheck>> {
    let filtered = columns
        .iter()
        .position(|column| column.kind == Kind::FilteredAltitude);
    let mut checks = vec![None; columns.len()];
    for (index, check) in checks.iter_mut().enumerate() {
        // `Alt` is the first column: the header's name after `T` (`is_header`).
        let altitude = match columns.get(index).map(|column| column.kind) {
            Some(Kind::RawSpeed) => Some(0),
            Some(Kind::FilteredSpeed) => filtered,
            _ => None,
        };
        let (Some(altitude), Some(speed)) = (altitude, written.get(index)) else {
            continue;
        };
        let Some(altitude_values) = written.get(altitude) else {
            continue;
        };
        let (matching, rows) = speed_matches(speed, altitude_values, step_s);
        // Row counts are far below 2^52, where an `f64` holds every whole number.
        let enough = rows == 0 || matching as f64 >= SPEED_MATCH_SHARE * rows as f64;
        if enough {
            *check = Some(SpeedCheck {
                altitude,
                matching,
                rows,
            });
            continue;
        }
        let altitude_name = columns
            .get(altitude)
            .map(|column| short(&column.name))
            .unwrap_or_default();
        if let Some(column) = columns.get_mut(index) {
            notes.push(format!(
                "`{}` kept as written, not read as a speed: it is the change in \
                 `{altitude_name}` over the step, within 1 % or 1 ft/s, in only {matching} of \
                 {rows} rows after the first, under the 90 % the reader asks",
                short(&column.name)
            ));
            column.kind = Kind::Unknown;
        }
    }
    checks
}

/// On how many rows after the first a speed is its altitude's change over the step, within 1 %
/// or 1 ft/s, of how many hold both and a finite change.
fn speed_matches(speed: &[f64], altitude: &[f64], step_s: &[f64]) -> (usize, usize) {
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
    (matching, rows)
}

/// The events: each run of non-zero samples in a marker column, at its first sample, row by row
/// and in the file's order within a row; past [`MAX_EVENTS_PER_MARKER`] in one column, the rest
/// counted in a note. An event's name is the column's, cut as a note quotes it.
fn events(
    columns: &[Column],
    written: &[Vec<f64>],
    time_s: &[f64],
    notes: &mut Notes,
) -> Vec<Event> {
    struct Marker<'a> {
        name: String,
        values: &'a [f64],
        kept: usize,
        left_out: usize,
    }
    let mut markers: Vec<Marker> = columns
        .iter()
        .zip(written)
        .filter(|(column, _)| column.kind == Kind::Marker)
        .map(|(column, values)| Marker {
            name: short(&column.name),
            values,
            kept: 0,
            left_out: 0,
        })
        .collect();
    let marked = |value: f64| value != 0.0 && !value.is_nan();
    let mut events = Vec::new();
    for (row, time) in time_s.iter().enumerate() {
        for marker in &mut markers {
            let value = marker.values.get(row).copied().unwrap_or(f64::NAN);
            let before = row
                .checked_sub(1)
                .and_then(|above| marker.values.get(above))
                .is_some_and(|&above| marked(above));
            if !marked(value) || before {
                continue;
            }
            if marker.kept < MAX_EVENTS_PER_MARKER {
                marker.kept += 1;
                events.push(Event {
                    time_s: *time,
                    name: marker.name.clone(),
                });
            } else {
                marker.left_out += 1;
            }
        }
    }
    for marker in markers.iter().filter(|marker| marker.left_out > 0) {
        notes.push(format!(
            "the marker {:?} rises {} more times than the {MAX_EVENTS_PER_MARKER} kept as events",
            marker.name, marker.left_out
        ));
    }
    events
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
    // Invariant: `steps` isn't empty (above), so `middle` is a place in it, and so is the one
    // before it when the count is even.
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
    let mut note = "`Alt` read in feet, from the manual: the Classic and TRS manuals (p37) state \
                    feet; the Quantum and Proton manuals give every height in feet and no \
                    Eggtimer manual offers meters"
        .to_owned();
    let find = |names: &[&str]| {
        columns
            .iter()
            .zip(written)
            .find(|(column, _)| names.contains(&column.name.as_str()))
    };
    if let (Some((g_column, g)), Some((ft_column, ft_s2))) =
        (find(&["AccelG", "Accel"]), find(&["AccelFPSS"]))
    {
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
                "; the file's `{}` is {} times its `{}`, and one g is 32.17 ft/s²",
                ft_column.name,
                plain(ratio, 2),
                g_column.name
            ));
        }
    }
    note
}

/// The note saying how a column other than `Alt` was read, `None` for one whose name states its
/// unit, or that is kept as written. `check` is what the rows say of a speed.
fn unit_note(column: &Column, check: Option<SpeedCheck>, columns: &[Column]) -> Option<String> {
    let name = short(&column.name);
    let evidence = |what: &str| match check {
        Some(check) if check.rows > 0 => format!(
            "it is the change in `{}` over the step, within 1 % or 1 ft/s, in {} of {} rows after \
             the first, where the reader asks 90 %",
            columns
                .get(check.altitude)
                .map(|altitude| short(&altitude.name))
                .unwrap_or_default(),
            check.matching,
            check.rows
        ),
        Some(_) => format!("the file has one row, so no step to check it against {what} on"),
        None => format!("the file has no {what} to check it against"),
    };
    let note = match column.kind {
        Kind::FilteredAltitude => format!(
            "`{name}` read in feet, from the manual: it is the logger's filtered `Alt` (Quantum \
             p38, Proton p45), in the same unit"
        ),
        Kind::RawSpeed => format!(
            "`{name}` read in feet per second, from the manual: the Classic and TRS manuals (p37) \
             give the velocity in ft/sec, and the Proton's (p45) as the change in `Alt` over the \
             step; {}",
            evidence("`Alt`")
        ),
        Kind::FilteredSpeed => format!(
            "`{name}` read in feet per second, from the manual: the Proton's manual (p46) gives \
             it as the change in the filtered altitude over the step, and the Excel guide (p7) \
             graphs the velocity in ft/sec; {}",
            evidence("filtered altitude")
        ),
        Kind::AccelerationG if !column.names_its_unit() => format!(
            "`{name}` read in g, from the manual: the Proton's manual (p45) gives the \
             accelerometer's raw value in G's"
        ),
        Kind::AccelerationFtS2 if !column.names_its_unit() => format!(
            "`{name}` read in feet per second squared, from the manual: the Proton's manual \
             (p45) gives the accelerometer's data in ft/sec^2"
        ),
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

/// The note saying whether the file's acceleration has gravity in it, judged from its first
/// acceleration column on the pad: the median of its rows before the start of flight, where `T`
/// is below 0 (the Proton's `T` counts from the start of flight, p45). Standing on the pad, an
/// accelerometer that keeps gravity reads 1 g and one that takes it out 0. `None` for a file
/// without one.
fn gravity_note(columns: &[Column], written: &[Vec<f64>], written_time: &[f64]) -> Option<String> {
    let ft_s2_per_g = STANDARD_GRAVITY_MPS2 / FOOT_M;
    let mut accelerations = columns
        .iter()
        .zip(written)
        .filter_map(|(column, values)| match column.kind {
            Kind::AccelerationG => Some((column, values, 1.0)),
            Kind::AccelerationFtS2 => Some((column, values, ft_s2_per_g)),
            _ => None,
        });
    let (column, values, per_g) = accelerations.next()?;
    let others = if accelerations.next().is_some() {
        "; the file's other acceleration columns are the same accelerometer's"
    } else {
        ""
    };
    let name = short(&column.name);
    let mut pad: Vec<f64> = written_time
        .iter()
        .zip(values)
        .filter(|(time, value)| **time < 0.0 && value.is_finite())
        .map(|(_, value)| value / per_g)
        .collect();
    let source = "the Proton's manual doesn't say, and its release notes (2.01C, p8) have it \
                  calibrate the accelerometer as it is armed, standing up";
    if pad.is_empty() {
        return Some(format!(
            "`{name}` may or may not hold gravity, not known: the file has no rows before the \
             start of flight (`T` below 0) to read the pad on; {source}{others}"
        ));
    }
    pad.sort_by(f64::total_cmp);
    // Invariant: `pad` isn't empty (above), so `middle` is a place in it, and so is the one
    // before it when the count is even.
    let middle = pad.len() / 2;
    let median = if pad.len().is_multiple_of(2) {
        (pad[middle - 1] + pad[middle]) / 2.0
    } else {
        pad[middle]
    };
    let reading = format!(
        "its median over the {} rows before the start of flight (`T` below 0) is {} g, where an \
         accelerometer that keeps gravity reads 1 g standing still and one that takes it out 0; \
         {source}{others}",
        pad.len(),
        plain(median, 2)
    );
    Some(if median.abs() < GRAVITY_READING_G {
        format!("`{name}` has gravity taken out, from the pad: {reading}")
    } else if (median - 1.0).abs() < GRAVITY_READING_G {
        format!("`{name}` holds gravity, from the pad: {reading}")
    } else {
        format!("`{name}` may or may not hold gravity, not known: {reading}")
    })
}

/// The note listing the marker columns, kept as written; `None` for a file without one.
fn marker_note(columns: &[Column]) -> Option<String> {
    let markers: Vec<String> = columns
        .iter()
        .filter(|column| column.kind == Kind::Marker)
        .map(|column| format!("`{}`", short(&column.name)))
        .take(MAX_LISTED + 1)
        .collect();
    let count = columns
        .iter()
        .filter(|column| column.kind == Kind::Marker)
        .count();
    if count == 0 {
        return None;
    }
    let speeds = if columns
        .iter()
        .any(|column| column.name == "MaxV" || column.name == "MaxA")
    {
        ", and the Proton's release notes (1.02N, p13) the top speed, ft/s, in `MaxV` and the \
         top acceleration, ft/s², in `MaxA`"
    } else {
        ""
    };
    Some(format!(
        "the marker columns, kept as written: {}; each is 0 but at the sample that marks its \
         event, where the Quantum's and Proton's manuals (p38, p46) put the raw altitude, in \
         feet{speeds}",
        list(&markers, count)
    ))
}

/// Names joined as `a`, `a and b`, or `a, b and c`; of more than [`MAX_LISTED`], the first few
/// and a count of the rest. `count` is how many there are; `names` may hold only the first.
fn list(names: &[String], count: usize) -> String {
    if count > MAX_LISTED {
        let shown = MAX_LISTED - 1;
        return format!(
            "{} and {} more columns",
            names.get(..shown).unwrap_or(names).join(", "),
            count - shown
        );
    }
    match names {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
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

// The terminal download: the flight settings and the flight summary, then the table above.

/// The terminal download's name in errors.
const TEXT_FORMAT: &str = "Eggtimer text";

/// The most lines that aren't blank the reader looks through for the title and the table's
/// header row. The manual's download (TRS p36) and the sample logs' put about 30 before the
/// header.
pub const MAX_PREAMBLE_LINES: usize = 200;

/// The most items one note about the preamble lists, each cut as a note quotes it; past it, the
/// rest are counted.
const MAX_PREAMBLE_ITEMS: usize = 40;

/// How far apart, s, a summary's time and a marker's event may be and still be the same moment:
/// half a millisecond, the summary's last digit.
const SAME_TIME_S: f64 = 0.000_5;

/// What each `Last Flight Phase` code means (TRS manual p37).
const FLIGHT_PHASES: [&str; 10] = [
    "flight not started",
    "waiting for LDA",
    "LDA reached",
    "not used",
    "not used",
    "Mach lockout expired",
    "nose-over",
    "drogue",
    "main deployed",
    "landing detected",
];

/// Whether the text is an Eggtimer's terminal download: a title `Eggtimer … Flight Settings`, then
/// an Eggtimer header row, within the first [`MAX_PREAMBLE_LINES`] lines that aren't blank.
pub(crate) fn recognizes_text(text: &str) -> bool {
    split_download(text).is_some()
}

/// A terminal download, split: its title, the preamble's lines after it, and the table.
struct Download<'a> {
    /// The title's words between `Eggtimer` and `Flight Settings`, such as `TRS 2.07`.
    title: &'a str,
    /// How many lines that aren't blank come before the title, `Flight Summary:` aside.
    before_title: usize,
    /// The lines between the title and the table's header row, blank ones left out, each
    /// trimmed.
    preamble: Vec<&'a str>,
    /// The text from the header row on.
    table: &'a str,
    /// The header row's line number in the file.
    header_line: usize,
}

/// The download's parts; `None` unless a title `Eggtimer … Flight Settings`, or a line
/// `Eggtimer …` over a line `Flight Settings`, comes before an Eggtimer header row, both within
/// the first [`MAX_PREAMBLE_LINES`] lines that aren't blank.
fn split_download(text: &str) -> Option<Download<'_>> {
    let mut title: Option<&str> = None;
    // A line `Eggtimer …` that doesn't end in `Flight Settings`, which the next line may finish.
    let mut pending: Option<&str> = None;
    let mut before_title = 0_usize;
    let mut preamble = Vec::new();
    let lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    for (number, line) in lines.take(MAX_PREAMBLE_LINES) {
        let trimmed = line.trim();
        if let Some(title) = title {
            if is_header(trimmed) {
                // `line` is a slice of `text`: the table starts where it does.
                let start = (line.as_ptr() as usize).checked_sub(text.as_ptr() as usize)?;
                return Some(Download {
                    title,
                    before_title,
                    preamble,
                    table: text.get(start..)?,
                    header_line: number,
                });
            }
            preamble.push(trimmed);
            continue;
        }
        if let Some(words) = pending.take() {
            if trimmed == "Flight Settings" {
                title = Some(words);
                continue;
            }
            before_title += 1;
        }
        if let Some(rest) = trimmed.strip_prefix("Eggtimer ") {
            match rest.strip_suffix("Flight Settings") {
                Some(words) => title = Some(words.trim()),
                None => pending = Some(rest.trim()),
            }
        } else if is_header(trimmed) {
            // A table with no title before it is the CSV download's.
            return None;
        } else if trimmed != "Flight Summary:" {
            before_title += 1;
        }
    }
    None
}

/// The logger's model and firmware from the title's words: the last word the firmware if it
/// starts with a digit, such as `2.07`, the words before it the model, such as `TRS`.
fn title_words(words: &str) -> (String, Option<String>) {
    let words: Vec<&str> = words.split_whitespace().collect();
    match words.split_last() {
        Some((last, model)) if last.starts_with(|c: char| c.is_ascii_digit()) => {
            (short(&model.join(" ")), Some(short(last)))
        }
        _ => (short(&words.join(" ")), None),
    }
}

/// A preamble's items as a note lists them: the first [`MAX_PREAMBLE_ITEMS`] kept, the rest
/// counted.
#[derive(Default)]
struct Items {
    kept: Vec<String>,
    left_out: usize,
}

impl Items {
    fn push(&mut self, item: String) {
        if self.kept.len() < MAX_PREAMBLE_ITEMS {
            self.kept.push(item);
        } else {
            self.left_out += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.kept.is_empty()
    }

    /// The items joined by `separator`, and a count of those left out.
    fn joined(&self, separator: &str) -> String {
        let mut text = self.kept.join(separator);
        if self.left_out > 0 {
            text.push_str(&format!("{separator}and {} more", self.left_out));
        }
        text
    }
}

/// What the preamble after the title states.
#[derive(Default)]
struct Preamble {
    /// The flight settings, each label and value as written.
    settings: Items,
    /// How many settings lines hold no value: the name the owner gave the flight computer.
    names: usize,
    /// The summary's lines not read into the record, as written.
    summary: Items,
    /// `ASL Alt.`, m above sea level.
    ground_m: Option<f64>,
    /// `Apogee`, m above the logger's zero.
    apogee_m: Option<f64>,
    /// Each summary time, `T @ …` or `Time @ …`: the event's name as written, and s.
    times: Vec<(String, f64)>,
    /// How many of the times carry a unit mark, such as `secs.`.
    marked_times: usize,
    /// The notes saying how the heights were read.
    unit_notes: Vec<String>,
}

/// Reads the preamble's lines after the title.
fn read_preamble(lines: &[&str]) -> Preamble {
    let mut preamble = Preamble::default();
    for line in lines {
        if let Some(body) = line.strip_prefix("...") {
            match body.rsplit_once("...") {
                Some((label, value)) => preamble.settings.push(format!(
                    "`{}` {}",
                    short(label.trim()),
                    short(value.trim())
                )),
                None => preamble.names += 1,
            }
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            preamble.summary.push(format!("`{}`", short(line)));
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        let as_written = format!("`{}`", short(&format!("{key}: {value}")));
        let event = key
            .strip_prefix("T @")
            .or_else(|| key.strip_prefix("Time @"))
            .map(str::trim)
            .filter(|name| !name.is_empty());
        match (key, event) {
            (_, Some(name)) if preamble.times.len() < MAX_PREAMBLE_LINES => {
                match summary_time(value) {
                    Some((time_s, marked)) => {
                        preamble.times.push((short(name), time_s));
                        preamble.marked_times += usize::from(marked);
                    }
                    None => preamble.summary.push(as_written),
                }
            }
            ("ASL Alt." | "Apogee", _) => {
                let (field, what) = if key == "Apogee" {
                    (
                        &mut preamble.apogee_m,
                        "the logger's apogee above its zero, kept beside the table's altitudes",
                    )
                } else {
                    (
                        &mut preamble.ground_m,
                        "the launch site's elevation above sea level, as the logger measured it",
                    )
                };
                match (field.is_some(), height(value)) {
                    (false, Some((height_m, unit))) => {
                        *field = Some(height_m);
                        preamble.unit_notes.push(format!(
                            "`{key}` read in {unit}, from its unit mark: the flight summary \
                             writes {}; {what} (TRS manual p36)",
                            short(value)
                        ));
                    }
                    (false, None) => {
                        preamble.unit_notes.push(format!(
                            "`{key}` kept as written, with no unit mark the reader knows: the \
                             flight summary writes {:?}, where the TRS manual's writes `ft` (p36)",
                            short(value)
                        ));
                        preamble.summary.push(as_written);
                    }
                    (true, _) => preamble.summary.push(as_written),
                }
            }
            ("Last Flight Phase", _) => {
                let meaning = value
                    .parse::<usize>()
                    .ok()
                    .and_then(|code| FLIGHT_PHASES.get(code));
                preamble.summary.push(match meaning {
                    Some(meaning) => format!("{as_written} ({meaning}, TRS manual p37)"),
                    None => as_written,
                });
            }
            _ => preamble.summary.push(as_written),
        }
    }
    preamble
}

/// A height as the summary writes it, such as `1500 ft`: m, and the unit's name; `None` without a
/// number and a unit mark `ft` or `m`.
fn height(value: &str) -> Option<(f64, &'static str)> {
    let mut words = value.split_whitespace();
    let written = number(words.next()?, false)?;
    let unit = words.next()?;
    if words.next().is_some() {
        return None;
    }
    match unit {
        "ft" => Some((written * FOOT_M, "feet")),
        "m" => Some((written, "meters")),
        _ => None,
    }
}

/// A time as the summary writes it, such as `9.500` or `9.500 secs.`: s, and whether it carries
/// a unit mark; `None` for anything else.
fn summary_time(value: &str) -> Option<(f64, bool)> {
    let mut words = value.split_whitespace();
    let seconds = number(words.next()?, false)?;
    let marked = match (words.next(), words.next()) {
        (None, _) => false,
        (Some("secs." | "secs" | "sec." | "sec" | "s"), None) => true,
        _ => return None,
    };
    Some((seconds, marked))
}

/// The table's marker for an event the summary names: the summary's `Unlock` is the end of the
/// Mach lockout, which the `LowV` marker marks (TRS manual p37).
fn marker_for(summary_name: &str) -> Option<&'static str> {
    match summary_name {
        "Apogee" => Some("Apogee"),
        "Unlock" => Some("LowV"),
        "N-O" | "Nose-Over" => Some("N-O"),
        "Drogue" | "Drogue Deploy" => Some("Drogue"),
        "Main" | "Main Deploy" => Some("Main"),
        "LDA" => Some("LDA"),
        _ => None,
    }
}

/// Reads an Eggtimer TRS's flight from its terminal download: the flight settings and the flight
/// summary, then the table the CSV download writes, read as [`read`] reads it.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if no title `Eggtimer … Flight Settings` comes before an Eggtimer
/// header row within the first [`MAX_PREAMBLE_LINES`] lines that aren't blank; otherwise what
/// [`read`] refuses in the table, naming the file's line.
pub fn read_text(text: &str) -> Result<FlightLog, LogError> {
    let Some(download) = split_download(text) else {
        return Err(LogError::NotThisFormat {
            format: TEXT_FORMAT,
            message: format!(
                "no title `Eggtimer … Flight Settings` comes before an Eggtimer header row, \
                 such as `T,Alt,Veloc,…`, within the first {MAX_PREAMBLE_LINES} lines that \
                 aren't blank"
            ),
        });
    };
    let shift = download.header_line.saturating_sub(1);
    let mut log = read(download.table).map_err(|error| match error {
        LogError::Syntax { line, message, .. } => LogError::Syntax {
            format: TEXT_FORMAT,
            line: line.saturating_add(shift),
            message,
        },
        LogError::NoData { .. } => LogError::NoData {
            format: TEXT_FORMAT,
        },
        LogError::NotThisFormat { message, .. } => LogError::NotThisFormat {
            format: TEXT_FORMAT,
            message,
        },
        other => other,
    })?;
    let (model, firmware) = title_words(download.title);
    log.format = LogFormat::EggtimerText;
    log.logger = if model.is_empty() {
        "Eggtimer".to_owned()
    } else {
        format!("Eggtimer {model}")
    };
    log.firmware = firmware;
    let preamble = read_preamble(&download.preamble);
    log.stated.apogee_m = preamble.apogee_m;
    log.stated.ground_elevation_msl_m = preamble.ground_m;
    log.notes.extend(preamble.unit_notes.iter().cloned());
    summary_events(&mut log, &preamble);
    if !preamble.settings.is_empty() {
        log.notes.push(format!(
            "the flight settings, as the file writes them: {}",
            preamble.settings.joined("; ")
        ));
    }
    if preamble.names > 0 {
        log.notes.push(format!(
            "{} of the flight settings holds no setting: the name its owner gave the flight \
             computer (TRS manual p23), left out of the record as free text that may name a \
             person",
            if preamble.names == 1 {
                "1 line".to_owned()
            } else {
                format!("{} lines", preamble.names)
            }
        ));
    }
    if !preamble.summary.is_empty() {
        log.notes.push(format!(
            "the flight summary's other lines, as the file writes them: {}",
            preamble.summary.joined("; ")
        ));
    }
    if download.before_title > 0 {
        log.notes.push(format!(
            "{} lines before the title `Eggtimer … Flight Settings` weren't read",
            download.before_title
        ));
    }
    Ok(log)
}

/// The summary's times read as events, with the notes saying how: a time that is its marker's
/// event in the table, to [`SAME_TIME_S`], is that event; one of no marker, or of another time,
/// is an event of its own, named as the summary names it; a time of 0 is no event, as the manual
/// writes zeroes for a channel that didn't fire (TRS p24). The events end up in time order, the
/// table's first where two share a time.
fn summary_events(log: &mut FlightLog, preamble: &Preamble) {
    if preamble.times.is_empty() {
        return;
    }
    let (mut same, mut own, mut differ, mut zero) = (
        Items::default(),
        Items::default(),
        Items::default(),
        Items::default(),
    );
    let mut events = Vec::new();
    for (name, time_s) in &preamble.times {
        if *time_s == 0.0 {
            zero.push(format!("`{name}`"));
            continue;
        }
        let marker = marker_for(name);
        let marked: Vec<f64> = log
            .events
            .iter()
            .filter(|event| Some(event.name.as_str()) == marker)
            .map(|event| event.time_s)
            .collect();
        match (marker, marked.first()) {
            (Some(marker), Some(_))
                if marked
                    .iter()
                    .any(|marked_s| (marked_s - time_s).abs() <= SAME_TIME_S) =>
            {
                same.push(format!("`{name}` at `{marker}`"));
                continue;
            }
            (Some(marker), Some(marked_s)) => differ.push(format!(
                "`{name}` at {} s, the marker `{marker}` at {} s",
                plain(*time_s, 3),
                plain(*marked_s, 3)
            )),
            _ => own.push(format!("`{name}`")),
        }
        events.push(Event {
            time_s: *time_s,
            name: name.clone(),
        });
    }
    let how = if preamble.marked_times == preamble.times.len() {
        "`T @` and `Time @` read in seconds, from their unit mark: the flight summary writes \
         `secs.` after each, as the TRS manual's download does (p36)"
    } else {
        "`T @` and `Time @` read in seconds, from the manual: the TRS manual's download writes \
         the flight summary's times in `secs.` (p36), and its LDA at 0.700 secs. as \"700ms \
         into the flight\" (p37)"
    };
    log.notes.push(how.to_owned());
    if !same.is_empty() {
        log.notes.push(format!(
            "the flight summary's times are its table markers' to the millisecond, one event \
             each, the marker's: {}",
            same.joined(", ")
        ));
    }
    if !own.is_empty() {
        log.notes.push(format!(
            "the flight summary's times of no marker in the table, each an event of its own: {}",
            own.joined(", ")
        ));
    }
    if !differ.is_empty() {
        log.notes.push(format!(
            "the flight summary's times that differ from their markers' in the table, both kept \
             as events: {}",
            differ.joined("; ")
        ));
    }
    if !zero.is_empty() {
        log.notes.push(format!(
            "the flight summary's times of 0, no event, as the TRS manual writes zeroes for a \
             channel that didn't fire (p24): {}",
            zero.joined(", ")
        ));
    }
    log.events.extend(events);
    log.events
        .sort_by(|first, second| first.time_s.total_cmp(&second.time_s));
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
        assert_eq!(log.notes.len(), 6, "{:?}", log.notes);
        assert_eq!(
            log.notes[0],
            "`T` read in seconds, from the step: the median step between rows is 0.05, and the \
             reader reads one above 5 as milliseconds (at the manuals' 1 to 33 samples a second, \
             a step is 0.03 to 1 in seconds and 30 to 1,000 in milliseconds)"
        );
        assert_eq!(
            log.notes[1],
            "`Alt` read in feet, from the manual: the Classic and TRS manuals (p37) state feet; \
             the Quantum and Proton manuals give every height in feet and no Eggtimer manual \
             offers meters"
        );
        assert_eq!(
            log.notes[2],
            "`Veloc` read in feet per second, from the manual: the Classic and TRS manuals (p37) \
             give the velocity in ft/sec, and the Proton's (p45) as the change in `Alt` over the \
             step; it is the change in `Alt` over the step, within 1 % or 1 ft/s, in 4 of 4 rows \
             after the first, where the reader asks 90 %"
        );
        assert!(log.notes[3].starts_with("`FAlt` read in feet, from the manual: "));
        assert!(
            log.notes[4].starts_with("`FVeloc` read in feet per second, from the manual: ")
                && log.notes[4].contains("the change in `FAlt` over the step")
                && log.notes[4].contains("in 4 of 4 rows after the first"),
            "{}",
            log.notes[4]
        );
        assert_eq!(
            log.notes[5],
            "the marker columns, kept as written: `LDA`, `LowV`, `Apogee`, `N-O`, `Drogue` and \
             `Main`; each is 0 but at the sample that marks its event, where the Quantum's \
             and Proton's manuals (p38, p46) put the raw altitude, in feet"
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
        // `AccelG` and `AccelFPSS` name their units, so get no unit note.
        assert!(log.notes[2].starts_with("`Veloc` read in feet per second, from the manual: "));
        assert!(log.notes[3].starts_with("`VAccel` read in feet per second, from the manual: "));
        assert!(log.notes[4].starts_with("`AltAccel` read in feet, from the manual: "));
        assert_eq!(
            log.notes[5],
            "`AccelG` has gravity taken out, from the pad: its median over the 1 rows before the \
             start of flight (`T` below 0) is 0 g, where an accelerometer that keeps gravity reads \
             1 g standing still and one that takes it out 0; the Proton's manual doesn't say, and \
             its release notes (2.01C, p8) have it calibrate the accelerometer as it is armed, \
             standing up; the file's other acceleration columns are the same accelerometer's"
        );
        assert!(
            log.notes[6].starts_with(
                "the marker columns, kept as written: `LDA`, `MaxV`, `MaxA` and `CH1`; "
            ) && log.notes[6].ends_with(
                ", and the Proton's release notes (1.02N, p13) the top speed, ft/s, in `MaxV` \
                     and the top acceleration, ft/s², in `MaxA`"
            ),
            "{}",
            log.notes[6]
        );
        assert_eq!(log.notes.len(), 7, "{:?}", log.notes);
    }

    /// The Proton manual's `Accel` doesn't name its unit, so it gets a note, and is the one the
    /// altitude's note and the gravity note name, as written. A pad reading near 1 g holds
    /// gravity; one near neither, or no rows before the start of flight, leaves it not known.
    #[test]
    fn the_acceleration_unit_and_gravity_are_noted() {
        let log = read(
            "T,Alt,Accel,AccelFPSS,LDA
-0.1,0,1,32.2,0
0,0,1.1,35.4,0
0.1,3,6,193,3
",
        )
        .unwrap();
        let notes = log.notes.join("\n");
        assert!(
            notes.contains(
                "`Accel` read in g, from the manual: the Proton's manual (p45) gives the \
                 accelerometer's raw value in G's"
            ),
            "{notes}"
        );
        assert!(
            notes.contains("the file's `AccelFPSS` is 32.17 times its `Accel`"),
            "{notes}"
        );
        assert!(
            notes.contains(
                "`Accel` holds gravity, from the pad: its median over the 1 rows before the start \
                 of flight (`T` below 0) is 1 g"
            ),
            "{notes}"
        );
        assert!(!notes.contains("AccelG"), "{notes}");
        assert!(close(log.channels[1].values[0], 9.806_65));

        let unsure = read(
            "T,Alt,AccelG,LDA
-0.2,0,3,0
-0.1,0,3,0
0,1,4,0
",
        )
        .unwrap();
        assert!(
            unsure.notes.iter().any(|note| note.starts_with(
                "`AccelG` may or may not hold gravity, not known: its median over the 2 rows \
                 before the start of flight (`T` below 0) is 3 g"
            )),
            "{:?}",
            unsure.notes
        );
        let no_pad = read(
            "T,Alt,FAccelFPSS,LDA
0,0,0,0
0.1,1,64,0
",
        )
        .unwrap();
        assert!(
            no_pad.notes.iter().any(|note| note.starts_with(
                "`FAccelFPSS` may or may not hold gravity, not known: the file has no rows before \
                 the start of flight"
            )),
            "{:?}",
            no_pad.notes
        );
        // A pad reading in ft/s² is judged in g.
        let in_ft = read(
            "T,Alt,AccelFPSS,LDA
-0.1,0,32.2,0
0,1,40,0
",
        )
        .unwrap();
        assert!(
            in_ft
                .notes
                .iter()
                .any(|note| note.starts_with("`AccelFPSS` holds gravity, from the pad: ")),
            "{:?}",
            in_ft.notes
        );
    }

    /// A speed that isn't its altitude's change over the step on 90 % of the rows is kept as
    /// written, and the vertical speed falls to the filtered one; at 90 % it is read.
    #[test]
    fn a_speed_that_isnt_its_altitudes_change_is_kept_as_written() {
        // Every `Veloc` is 1,000 times the change: a clock in milliseconds read as seconds would
        // do this.
        let text = "T,Alt,Veloc,FAlt,FVeloc,LDA
0,0,0,0,0,0
1,10,10000,5,5,0
2,30,20000,15,10,0
";
        let log = read(text).unwrap();
        assert_eq!(log.channels[1].column, "Veloc");
        assert_eq!(log.channels[1].quantity, Quantity::Unrecognized);
        assert_eq!(log.channels[1].values, [0.0, 10000.0, 20000.0]);
        assert_eq!(log.channels[3].quantity, Quantity::Speed);
        assert_eq!(
            log.vertical_speed_m_s.as_deref(),
            Some(&[0.0, 5.0 * 0.3048, 10.0 * 0.3048][..])
        );
        let notes = log.notes.join("\n");
        assert!(
            notes.contains(
                "`Veloc` kept as written, not read as a speed: it is the change in `Alt` over the \
                 step, within 1 % or 1 ft/s, in only 0 of 2 rows after the first, under the 90 % \
                 the reader asks"
            ),
            "{notes}"
        );
        assert!(!notes.contains("`Veloc` read in"), "{notes}");
        assert!(notes.contains("the file has no raw velocity, `Veloc`, read as a speed"));

        // 9 of 10 rows match: read; 8 of 10: kept as written.
        let rows = |misses: usize| -> String {
            (0..=10)
                .map(|i| {
                    let speed = if i > 0 && i <= misses {
                        99
                    } else {
                        10 * usize::from(i > 0)
                    };
                    format!("{i},{},{speed},0\n", 10 * i)
                })
                .collect()
        };
        let nine = read(&format!("T,Alt,Veloc,LDA\n{}", rows(1))).unwrap();
        assert_eq!(nine.channels[1].quantity, Quantity::Speed);
        assert!(
            nine.notes
                .iter()
                .any(|note| note.contains("in 9 of 10 rows after the first"))
        );
        let eight = read(&format!("T,Alt,Veloc,LDA\n{}", rows(2))).unwrap();
        assert_eq!(eight.channels[1].quantity, Quantity::Unrecognized);
        assert!(eight.vertical_speed_m_s.is_none());
    }

    /// A header repeating a name gets that name's unit note once.
    #[test]
    fn a_repeated_name_is_noted_once() {
        let log = read(
            "T,Alt,Veloc,Veloc,LDA
0,0,0,0,0
0.1,1,10,10,0
",
        )
        .unwrap();
        let veloc = log
            .notes
            .iter()
            .filter(|note| note.starts_with("`Veloc` read in"))
            .count();
        assert_eq!(veloc, 1, "{:?}", log.notes);
        assert_eq!(log.channels[2].quantity, Quantity::Speed);
    }

    /// A marker column with a very long name rising on every other row gives at most
    /// [`MAX_EVENTS_PER_MARKER`] events, each naming it cut short, and one note counting the rest:
    /// the record stays near the file's size.
    #[test]
    fn a_long_marker_name_rising_often_is_bounded() {
        let name = "X".repeat(100_000);
        let rows: String = (0..4000).map(|i| format!("{i},0,0,{}\n", i % 2)).collect();
        let text = format!("T,Alt,LDA,{name}\n{rows}");
        let log = read(&text).unwrap();
        assert_eq!(log.events.len(), MAX_EVENTS_PER_MARKER);
        assert!(
            log.events
                .iter()
                .all(|event| event.name.chars().count() <= 80)
        );
        assert!(log.events[0].name.ends_with('…'));
        assert_eq!(log.events[15].time_s, 31.0);
        let notes = log.notes.join("\n");
        assert!(
            notes.contains(&format!(
                "rises {} more times than the 16 kept as events",
                2000 - MAX_EVENTS_PER_MARKER
            )),
            "{}",
            short(&notes)
        );
        let bytes: usize = log.notes.iter().map(String::len).sum::<usize>()
            + log
                .events
                .iter()
                .map(|event| event.name.len())
                .sum::<usize>();
        assert!(bytes < 10_000, "{bytes} bytes of notes and event names");
        // The channel keeps the name as written.
        assert_eq!(log.channels[2].column.len(), 100_000);
    }

    /// The marker note lists at most a few columns, and counts the rest.
    #[test]
    fn the_marker_note_stays_short() {
        let markers: String = (1..=40).map(|i| format!(",CH{i}")).collect();
        let text = format!(
            "T,Alt,LDA{markers}
0,0{}
",
            ",0".repeat(41)
        );
        let log = read(&text).unwrap();
        let note = log
            .notes
            .iter()
            .find(|note| note.starts_with("the marker columns, kept as written: "))
            .unwrap();
        assert!(
            note.starts_with(
                "the marker columns, kept as written: `LDA`, `CH1`, `CH2`, `CH3`, `CH4` and 36 more \
                 columns; "
            ),
            "{note}"
        );
        assert!(
            read(
                "T,Alt,Veloc
0,0,0
"
            )
            .unwrap()
            .notes
            .iter()
            .all(|note| !note.contains("marker columns"))
        );
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
        assert!(old.notes[2].starts_with("`VRaw` read in feet per second, from the manual: "));
        assert!(
            old.notes[3].ends_with("the file has no filtered altitude to check it against"),
            "{}",
            old.notes[3]
        );
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

    /// An invented terminal download: firmware 2.07's spellings, the table in milliseconds.
    const DOWNLOAD: &str = "Flight Summary:\n\
        \n\
        Eggtimer TRS 2.07    Flight Settings\n\
        ...Drogue <1=Normal,2=Backup,0=OFF>...1\n\
        ...Main Altitude <50-2000 by 50, 0=OFF>...300\n\
        ...Frequency (Ctrl-F to change)...915\n\
        ...ID Code...4\n\
        ...SECRET NAME\n\
        ASL Alt.: 1000 ft\n\
        Base Temp: 20.5 C\n\
        Last Flight Phase: 9\n\
        Apogee: 30 ft\n\
        T @ Apogee: 0.150\n\
        Max Velocity: 300 ft/sec\n\
        T @ Max Velocity: 0.100\n\
        Time @ LDA:0.100\n\
        Altitude @ Main Deploy: 0 ft\n\
        T @ Main Deploy: 0.000\n\
        T,Alt,Veloc,Filt_Alt,FVeloc,LDA,LowV,Apogee,N-O,Drogue,Main\n\
        0,0,0.00,0.00,0.00,0,0,0,0,0,0\n\
        50,10,200.00,2.00,40.00,0,0,0,0,0,0\n\
        100,25,300.00,7.00,100.00,25,0,0,0,0,0\n\
        150,30,100.00,13.50,130.00,0,0,30,0,0,0\n\
        200,29,-20.00,18.00,90.00,0,0,0,29,29,0\n";

    #[test]
    fn recognizes_the_terminal_download_and_no_other_layout() {
        let table = format!("{TRS}\n0,0,0,0,0,0,0,0,0,0,0\n");
        for text in [
            DOWNLOAD.to_owned(),
            format!("Eggtimer TRS 2.10    Flight Settings\r{table}"),
            format!("\u{feff}Eggtimer TRS 2.07\nFlight Settings\n...ID Code...0\n{table}"),
            format!("Flight Summary:\nEggtimer Flight Settings\n{table}"),
            format!("Saved Flight Summary\nEggtimer TRS 2.07 Flight Settings\n{table}"),
        ] {
            assert!(recognizes_text(&text), "{text:?}");
            assert!(read_text(&text).is_ok(), "{text:?}");
        }
        for text in [
            String::new(),
            table.clone(),
            DOWNLOAD.replace("Flight Settings", "Settings"),
            format!("Eggtimer TRS 2.07\nFlight Summary:\n{table}"),
            format!("{table}Eggtimer TRS 2.07 Flight Settings\n{table}"),
            "Eggtimer TRS 2.07 Flight Settings\nTime,Altitude,Velocity\n0,0,0\n".to_owned(),
            "Eggtimer TRS 2.07 Flight Settings\nT,Alt\n0,0\n".to_owned(),
            format!(
                "Eggtimer TRS 2.07 Flight Settings\n{}{table}",
                "Beep Code: 0\n".repeat(MAX_PREAMBLE_LINES)
            ),
            "PerfectFlite Pnut\nEggtimer TRS 2.07 Flight Settings\n".to_owned(),
        ] {
            assert!(!recognizes_text(&text), "{text:?}");
            assert!(
                matches!(
                    read_text(&text),
                    Err(LogError::NotThisFormat {
                        format: TEXT_FORMAT,
                        ..
                    })
                ),
                "{text:?}"
            );
        }
        // The preamble may fill all but the header's line of those looked through.
        let full = format!(
            "Eggtimer TRS 2.07 Flight Settings\n{}{table}",
            "Beep Code: 0\n".repeat(MAX_PREAMBLE_LINES - 2)
        );
        assert!(recognizes_text(&full));
        // The CSV download isn't this layout, nor this one the CSV's.
        assert!(recognizes(&table) && !recognizes(DOWNLOAD));
    }

    /// The title, the settings, the summary's heights and times, and the table, as the module's
    /// documentation says each is read.
    #[test]
    fn reads_the_settings_the_summary_and_the_table() {
        let log = read_text(DOWNLOAD).unwrap();
        assert_eq!(log.format, LogFormat::EggtimerText);
        assert_eq!(log.logger, "Eggtimer TRS");
        assert_eq!(log.firmware.as_deref(), Some("2.07"));
        assert_eq!(log.time_s, [0.0, 0.05, 0.1, 0.15, 0.2]);
        assert!(close(log.stated.apogee_m.unwrap(), 30.0 * 0.3048));
        assert!(close(
            log.stated.ground_elevation_msl_m.unwrap(),
            1000.0 * 0.3048
        ));
        assert_eq!(log.stated.samples, None);
        // The table's markers, and the summary's `Max Velocity`, which no marker marks, in time
        // order; `Apogee` and `LDA` are their markers', and `Main Deploy` at 0 is no event.
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|event| (event.time_s, event.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [
                (0.1, "LDA"),
                (0.1, "Max Velocity"),
                (0.15, "Apogee"),
                (0.2, "N-O"),
                (0.2, "Drogue")
            ]
        );
        let notes = &log.notes[6..];
        assert_eq!(
            notes,
            [
                "`ASL Alt.` read in feet, from its unit mark: the flight summary writes 1000 ft; \
                 the launch site's elevation above sea level, as the logger measured it (TRS \
                 manual p36)",
                "`Apogee` read in feet, from its unit mark: the flight summary writes 30 ft; the \
                 logger's apogee above its zero, kept beside the table's altitudes (TRS manual \
                 p36)",
                "`T @` and `Time @` read in seconds, from the manual: the TRS manual's download \
                 writes the flight summary's times in `secs.` (p36), and its LDA at 0.700 secs. \
                 as \"700ms into the flight\" (p37)",
                "the flight summary's times are its table markers' to the millisecond, one event \
                 each, the marker's: `Apogee` at `Apogee`, `LDA` at `LDA`",
                "the flight summary's times of no marker in the table, each an event of its own: \
                 `Max Velocity`",
                "the flight summary's times of 0, no event, as the TRS manual writes zeroes for a \
                 channel that didn't fire (p24): `Main Deploy`",
                "the flight settings, as the file writes them: `Drogue <1=Normal,2=Backup,0=OFF>` \
                 1; `Main Altitude <50-2000 by 50, 0=OFF>` 300; `Frequency (Ctrl-F to change)` \
                 915; `ID Code` 4",
                "1 line of the flight settings holds no setting: the name its owner gave the \
                 flight computer (TRS manual p23), left out of the record as free text that may \
                 name a person",
                "the flight summary's other lines, as the file writes them: `Base Temp: 20.5 C`; \
                 `Last Flight Phase: 9` (landing detected, TRS manual p37); `Max Velocity: 300 \
                 ft/sec`; `Altitude @ Main Deploy: 0 ft`",
            ]
        );
        assert!(log.notes[0].starts_with("`T` read in milliseconds, from the step: "));
        // The owner's name for the flight computer is nowhere in the record.
        let json = serde_json::to_string(&log).unwrap();
        assert!(!json.contains("SECRET"));
    }

    /// A time with its unit mark, a time that isn't its marker's, a height in meters, one with no
    /// unit mark, a repeated one, and lines before the title.
    #[test]
    fn summary_lines_read_by_what_they_state() {
        let text = format!(
            "Captured with a terminal\n\
             Eggtimer TRS 2.07    Flight Settings\n\
             Apogee: 9 m\n\
             Apogee: 30 ft\n\
             ASL Alt.: 1000\n\
             T @ Apogee: 0.200 secs.\n\
             T @ Nose-Over: 0.2 s\n\
             T @ Unlock: soon\n\
             a line with no colon\n\
             {TRS}\n\
             0,0,0,0,0,0,0,0,0,0,0\n\
             50,10,200,2,40,0,0,0,0,0,0\n\
             100,25,300,7,100,25,0,0,0,0,0\n\
             150,30,100,13.5,130,0,0,30,0,0,0\n\
             200,29,-20,18,90,0,0,0,29,29,0\n"
        );
        let log = read_text(&text).unwrap();
        assert_eq!(log.stated.apogee_m, Some(9.0));
        assert_eq!(log.stated.ground_elevation_msl_m, None);
        let names: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|event| (event.time_s, event.name.as_str()))
            .collect();
        assert_eq!(
            names,
            [
                (0.1, "LDA"),
                (0.15, "Apogee"),
                (0.2, "N-O"),
                (0.2, "Drogue"),
                (0.2, "Apogee")
            ]
        );
        let has = |wanted: &str| log.notes.iter().any(|note| note == wanted);
        assert!(has(
            "`Apogee` read in meters, from its unit mark: the flight summary writes 9 m; the \
             logger's apogee above its zero, kept beside the table's altitudes (TRS manual p36)"
        ));
        assert!(has(
            "`ASL Alt.` kept as written, with no unit mark the reader knows: the flight summary \
             writes \"1000\", where the TRS manual's writes `ft` (p36)"
        ));
        assert!(has(
            "`T @` and `Time @` read in seconds, from their unit mark: the flight summary writes \
             `secs.` after each, as the TRS manual's download does (p36)"
        ));
        assert!(has(
            "the flight summary's times that differ from their markers' in the table, both kept \
             as events: `Apogee` at 0.2 s, the marker `Apogee` at 0.15 s"
        ));
        assert!(has(
            "the flight summary's times are its table markers' to the millisecond, one event \
             each, the marker's: `Nose-Over` at `N-O`"
        ));
        assert!(has(
            "the flight summary's other lines, as the file writes them: `Apogee: 30 ft`; `ASL \
             Alt.: 1000`; `T @ Unlock: soon`; `a line with no colon`"
        ));
        assert!(has(
            "1 lines before the title `Eggtimer … Flight Settings` weren't read"
        ));
    }

    /// What the table refuses is refused naming the file's line, and the download's format.
    #[test]
    fn the_tables_errors_name_the_files_line() {
        let bad = DOWNLOAD.replace("100,25,300.00", "100,x,300.00");
        assert_eq!(
            read_text(&bad).unwrap_err(),
            LogError::Syntax {
                format: TEXT_FORMAT,
                line: 22,
                message: "\"x\" in cell 2, under \"Alt\", isn't a number".to_owned(),
            }
        );
        let cr = bad.replace('\n', "\r");
        assert!(matches!(
            read_text(&cr),
            Err(LogError::Syntax { line: 22, .. })
        ));
        let headless = DOWNLOAD
            .split_once("0,0,0.00")
            .map(|(head, _)| head)
            .unwrap();
        assert_eq!(
            read_text(headless).unwrap_err(),
            LogError::NoData {
                format: TEXT_FORMAT
            }
        );
    }

    /// A long preamble's notes list at most [`MAX_PREAMBLE_ITEMS`] lines, each cut short.
    #[test]
    fn the_preamble_notes_stay_short() {
        let settings: String = (0..150)
            .map(|index| format!("...Setting {index} {}...1\n", "y".repeat(500)))
            .collect();
        let times: String = (0..30)
            .map(|index| format!("T @ Thing {index}: {}.5\n", index + 1))
            .collect();
        let text = format!(
            "Eggtimer {} 9.99 Flight Settings\n{settings}{times}{TRS}\n0,0,0,0,0,0,0,0,0,0,0\n",
            "Z".repeat(500)
        );
        let log = read_text(&text).unwrap();
        assert!(log.logger.chars().count() < 100);
        assert_eq!(log.firmware.as_deref(), Some("9.99"));
        assert_eq!(log.events.len(), 30);
        let settings = log
            .notes
            .iter()
            .find(|note| note.starts_with("the flight settings"))
            .unwrap();
        assert!(settings.ends_with("; and 110 more"), "{settings}");
        assert!(settings.len() < 40 * 100 + 200, "{} bytes", settings.len());
        for note in &log.notes {
            assert!(note.len() < 5_000, "{} bytes", note.len());
        }
    }

    #[test]
    fn the_title_names_the_model_and_firmware() {
        assert_eq!(
            title_words("TRS 2.07"),
            ("TRS".to_owned(), Some("2.07".to_owned()))
        );
        assert_eq!(title_words("TRS"), ("TRS".to_owned(), None));
        assert_eq!(title_words(""), (String::new(), None));
        assert_eq!(
            title_words("2.10"),
            (String::new(), Some("2.10".to_owned()))
        );
        assert_eq!(height("12 ft"), Some((12.0 * 0.3048, "feet")));
        assert_eq!(height("12"), None);
        assert_eq!(height("12 ft extra"), None);
        assert_eq!(summary_time("1.5 secs."), Some((1.5, true)));
        assert_eq!(summary_time("1.5"), Some((1.5, false)));
        assert_eq!(summary_time("1.5 hours"), None);
    }

    proptest::proptest! {
        /// Any preamble is read or refused, never a panic; what is read has increasing times and
        /// its events in time order.
        #[test]
        fn any_preamble_is_read_or_refused(
            lines in proptest::collection::vec("[.:@ A-Za-z0-9<>=,-]{0,30}", 0..12)
        ) {
            let text = format!(
                "Eggtimer TRS 2.07 Flight Settings\n{}\n{TRS}\n0,0,0,0,0,0,0,0,0,0,0\n",
                lines.join("\n")
            );
            if let Ok(log) = read_text(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert!(
                    log.events.windows(2).all(|pair| pair[0].time_s <= pair[1].time_s)
                );
            }
        }
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
