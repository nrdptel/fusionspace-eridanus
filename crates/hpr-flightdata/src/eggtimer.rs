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
//! is read in seconds. A note on the record says which, and why. The terminal download's table
//! checks this, and its altitudes' feet, against the coast too (below).
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
//! - **The table**, read as [`read`] reads the CSV download, every rule above the same, but for
//!   its clock and its altitudes, each settled by two checks that agree ([ADR-258][adr-258] §2),
//!   the coast's below; an error names the file's line.
//! - **The title**: the logger, such as `Eggtimer TRS`, and its last word, if it starts with a
//!   digit, as the firmware.
//! - **The flight settings**, the lines `...<setting>...<value>`, listed as written in one note.
//!   A settings line with no value is the name the owner gave the flight computer (the manual's
//!   settings screen shows `MY EGGTIMER`, p24): free text that may name a person, it is left out
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
//!   channel that didn't fire (p25). Nor is a time past the table's last: no event of the flight
//!   comes after its last sample, so such a time isn't in seconds, and it is kept as written in a
//!   note rather than read in another unit on one check. The events are in time order, the
//!   table's first where two share a time.
//! - **The other summary lines**, listed as written in one note: the base temperature with its
//!   unit mark (`F` in the sample files, `C` in the manual's example), the flight time, the beep
//!   code, the top speed and acceleration, and the heights at each event; `Last Flight Phase` with
//!   its meaning, from the manual's codes (p37).
//!
//! Each note lists at most 40 lines, the rest counted. The summary's figures beside the table's
//! are the logger's own and are never put in place of HPR Sim's readings.
//!
//! **The coast check.** The terminal download's clock and altitudes rest on two checks each, not
//! one ([ADR-258][adr-258] §2): `T` on its step (above) and the coast, `Alt` and its filtered
//! `FAlt` (`Filt_Alt`, `KAlt`) on the manuals' feet (above) and the coast. In the coast just before
//! the top nothing pushes the rocket and, as it slows, its drag fades: its altitude curves down
//! at gravity, 32.17 ft/s² in feet and seconds, 9.81 m/s² in meters. The reader fits a parabola,
//! by least squares, to each altitude over the 3 s up to the window's end (each row the median of
//! the 5 rows about it within the window, so that a spike of one or two rows is passed over), and
//! reads its curvature as a share of standard gravity:
//!
//! - The window ends at `Alt`'s top: the highest median of 5 rows, then the highest row among
//!   those 5. A charge marked at or before the top ends it at the row before: `Apogee`, `N-O`, a
//!   deployment channel (`Drogue`, `Main`, `AUX`, `CHA`, `CHB`, `CH1`…) or a column the reader
//!   doesn't know; an Eggtimer can fire one on the way up, and its canopy's drag would add to
//!   gravity. Then a jump in the 4 s before the end, a row more than half the window's rise from
//!   its median (an ejection's pressure), or a motor's marker there (`BurnT`, `BO#…`, `Boost#…`),
//!   ends it at the row before, again until neither is left. `LDA`, `LowV`, `MaxV` and `MaxA`
//!   change nothing on the rocket.
//! - The coast must hold 6 rows with an altitude (the manuals' slowest climb, 4 samples a second,
//!   puts 12 in 3 s), the table must reach back the whole 3 s, the altitude must rise over them,
//!   and the parabola must turn down and top out within 1 s of the window's end, so that the
//!   window is the slow end of the coast, where a rocket's own drag is a few hundredths of a g.
//! - A slowing of 0.7 to 1.3 g bears a unit out: drag near the top is small, and feet and meters
//!   are 3.28 apart, so at most one is within it. A clock 1,000 times off puts the slowing 10⁶
//!   times off.
//!
//! `T` is read in the step's unit when `Alt` slows at 0.7 to 1.3 g in feet or in meters on that
//! clock; otherwise, or with no coast to fit, the file is refused ([`LogError::Unit`]): nothing
//! in it would have a time. `Alt` is read in feet when it slows at 0.7 to 1.3 g in feet;
//! otherwise it is kept as written, [`Quantity::Unrecognized`], the record holding no barometric
//! altitude, and so is each speed that is its change. Each filtered altitude is fitted over the
//! same window and read in feet, or kept as written with its speed, the same way. A note on the
//! record says which, with the window and the slowing.
//!
//! What can't be seen: a canopy this Eggtimer didn't fire, out within 4 s of the window's end with
//! no jump. One out longer before the top is no matter: a canopy whose drag grows as the speed
//! squared, `g (v / v_t)²` for a descent rate `v_t`, stops a rising rocket within `(v_t / g) π/2`,
//! so one out more than 4 s before has `v_t` over `8g/π` and slows the 3 s up to the top by at
//! most `v_t tan(3g / v_t) / 3 s`, 2.05 g, which reads 0.62 g in feet, short of the band. As of
//! 2026-10-13, in the 3 terminal downloads among the project's sample logs (two text files, one
//! RTF), `Alt` slows at 0.96 to 1.02 g in feet and the filtered altitudes at 1.01 to 1.04 g; one
//! window ends at the row before the `Apogee` marker, one before a jump and one at the top.
//!
//! **The download saved as RTF.** A terminal capture pasted into a text editor and saved as an
//! RTF document is the same text wrapped in RTF's markup, [`LogFormat::EggtimerRtf`]. The reader
//! unwraps it, not parses it ([ADR-258][adr-258] §4): it keeps the document's plain text, by a
//! small subset of the RTF specification ([`read_rtf`] lists it), and reads that as the terminal
//! download above, an error naming the document's line. As of 2026-10-13, of the three sample
//! files filed as RTF, one is such a document, written by a Mac's text editor, a line of text a
//! line of the file, and holding four downloads one after another: the first is read and the
//! others counted in a note. The other two hold no Eggtimer download and are refused by name.
//! Its downloads' firmware writes most summary times without the TRS manual's spacing
//! (`T@Apogee`), so they are kept as written; the one with it lies far past the table's last
//! sample, so it can't be in seconds, and it is kept as written too, never an event.
//!
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, finite, number, numbered_lines, short};

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
    read_table(text, Checks::Csv)
}

/// How the table's clock and altitudes are decided: as the CSV download's are, the clock by its
/// step and the altitudes by the manuals, or, for the terminal download, each also by the coast
/// (module docs, *The coast check*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Checks {
    Csv,
    Coast,
}

/// The table read as [`read`] reads it, its units decided by `checks`; [`LogError::Unit`] for a
/// clock the coast doesn't bear out, naming the header's line.
fn read_table(text: &str, checks: Checks) -> Result<FlightLog, LogError> {
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
    build(columns, &rows, blank_rows, notes, header_line, checks)
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
    header_line: usize,
    checks: Checks,
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
    let clock = time_unit(&written_time);
    let units_per_second = clock.map_or(1.0, |(per_second, _)| per_second);
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
    let coast = match checks {
        Checks::Csv => None,
        Checks::Coast => Some(coast_units(
            &columns,
            &written,
            &time_s,
            clock,
            header_line,
        )?),
    };
    let speeds = speed_checks(&mut columns, &written, &step_s, &mut notes);

    // One note per name decided, so a header repeating a name doesn't repeat its note. On the
    // terminal download's path, an altitude the coast doesn't bear out is kept as written, and
    // so is a speed that is its change.
    let decided = |index: usize| {
        coast
            .as_ref()
            .and_then(|units| units.altitudes.get(index).cloned().flatten())
    };
    let mut unit_notes = vec![match &coast {
        Some(units) => units.clock.clone(),
        None => time_note(clock),
    }];
    let mut kept: Vec<usize> = Vec::new();
    match decided(0) {
        Some((note, keep)) => {
            unit_notes.push(note);
            if keep {
                kept.push(0);
            }
        }
        None => unit_notes.push(altitude_note(&columns, &written)),
    }
    let mut noted: Vec<&str> = Vec::new();
    for (index, (column, check)) in columns.iter().zip(&speeds).enumerate() {
        let decision = decided(index).filter(|_| column.kind == Kind::FilteredAltitude);
        let of_kept = check.filter(|check| {
            matches!(column.kind, Kind::RawSpeed | Kind::FilteredSpeed)
                && decided(check.altitude).is_some_and(|(_, keep)| keep)
        });
        if decision.as_ref().is_some_and(|(_, keep)| *keep) || of_kept.is_some() {
            kept.push(index);
        }
        if noted.contains(&column.name.as_str()) {
            continue;
        }
        let note = match (decision, of_kept) {
            (Some((note, _)), _) => Some(note),
            (None, Some(check)) => {
                let altitude = columns
                    .get(check.altitude)
                    .map(|altitude| short(&altitude.name))
                    .unwrap_or_default();
                Some(format!(
                    "`{}` kept as written: it is the change in `{altitude}` over the step, and \
                     `{altitude}` is kept as written",
                    short(&column.name)
                ))
            }
            (None, None) => unit_note(column, *check, &columns),
        };
        if let Some(note) = note {
            noted.push(&column.name);
            unit_notes.push(note);
        }
    }
    for index in kept {
        if let Some(column) = columns.get_mut(index) {
            column.kind = Kind::Unknown;
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
        // A value in g that the factor makes infinite is refused on its row's line (ADR-258 §3).
        let (quantity, factor) = column.kind.quantity();
        let mut si = Vec::with_capacity(values.len());
        for ((line, _), value) in rows.iter().zip(values) {
            si.push(finite(value * factor, FORMAT, *line, &column.name)?);
        }
        log.channels.push(Channel {
            column: column.name.clone(),
            quantity,
            time_s: time_s.clone(),
            values: si,
        });
    }
    log.events = events(&columns, &written, &time_s, &mut notes);

    // The primary track: `Alt`, unless it is kept as written, and the barometer's raw speed, or
    // its filtered one. Invariant: `Alt` is the header's name after `T` (`is_header`), so it is
    // the first channel.
    log.altitude_m = columns
        .first()
        .filter(|column| column.kind == Kind::Altitude)
        .and(log.channels.first())
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

/// The time's units in a second, 1 or 1,000, decided from the median step between rows, and that
/// median; `None` for a file of one row, with no step. Dividing by 1,000 keeps a whole number of
/// milliseconds the nearest `f64` to its seconds, where multiplying by 0.001 can miss it by one
/// step.
fn time_unit(written: &[f64]) -> Option<(f64, f64)> {
    let mut steps: Vec<f64> = written.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if steps.is_empty() {
        return None;
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
    let per_second = if median > MILLISECONDS_ABOVE {
        1000.0
    } else {
        1.0
    };
    Some((per_second, median))
}

/// The clock's unit as [`time_unit`] decides it, by name.
fn clock_unit(per_second: f64) -> &'static str {
    if per_second == 1.0 {
        "seconds"
    } else {
        "milliseconds"
    }
}

/// What the step says of the clock, as a note writes it after `from the step`.
fn step_evidence(median: f64) -> String {
    format!(
        "the median step between rows is {}, and the reader reads one above 5 as milliseconds (at \
         the manuals' 1 to 33 samples a second, a step is 0.03 to 1 in seconds and 30 to 1,000 in \
         milliseconds)",
        plain(median, 4)
    )
}

/// The note saying how the CSV download's `T` was read: by its step, or in seconds for a file of
/// one row.
fn time_note(clock: Option<(f64, f64)>) -> String {
    match clock {
        Some((per_second, median)) => format!(
            "`T` read in {}, from the step: {}",
            clock_unit(per_second),
            step_evidence(median)
        ),
        None => "`T` read in seconds, assumed: the file holds one row, so there is no step \
                 between rows to tell seconds from milliseconds"
            .to_owned(),
    }
}

// The coast check: the terminal download's second check of its clock and altitudes.

/// How long, s, the coast window is: the altitude is fitted over the 3 s up to the window's end.
/// Longer windows reach back to faster flight, where drag adds more to gravity; shorter ones leave
/// the fit to a barometer's noise. As of 2026-10-13, over the 3 s windows of the 3 terminal
/// downloads among the project's sample logs, `Alt` slows at 0.96 to 1.02 g in feet.
const COAST_WINDOW_S: f64 = 3.0;

/// How far before the window's end, s, a jump in the altitude or a motor's marker ends the window
/// early. A canopy whose drag grows as the speed squared, `g (v / v_t)²` for a descent rate `v_t`,
/// stops a rising rocket within `(v_t / g) π/2`; so one out more than 4 s before the top has `v_t`
/// over `8g/π`, and over a 3 s window ending at the top it slows the rocket by at most
/// `v_t tan(3g / v_t) / 3 s`, under 2.05 g (`tan(x)/x` at `x = 3π/8`), which a parabola's fit
/// weights toward the window's middle and lowers further: read in feet, 0.62 g, short of
/// [`COAST_BAND`]. A canopy out within 4 s of the end is ended at, by its charge's marker or its
/// jump.
const COAST_SEARCH_S: f64 = 4.0;

/// How far, as a share of the altitude's rise over the window, a row's altitude may be from its
/// neighbors' median before it is a jump: an ejection's pressure, or a canopy opening. A coast
/// rises `g W²/2` over a window `W` into its top, 145 ft over 3 s. As of 2026-10-13, in the 3
/// terminal downloads among the project's sample logs, a row in the window strays from its
/// median by under a quarter of the window's rise, and the one jump that ends a window by several
/// times it.
const COAST_JUMP_SHARE: f64 = 0.5;

/// The rows on each side of a row whose median is its level: 2, so that a spike of one or two rows
/// is passed over.
const LEVEL_ROWS: usize = 2;

/// The fewest rows with an altitude the coast window must hold: twice the parabola's three
/// figures. The manuals' slowest climb, 4 samples a second (Classic p19), puts 12 in 3 s.
const COAST_ROWS: usize = 6;

/// How far, s, from the window's end the parabola fitted to it may top out: the window must end
/// near the coast's top, where the rocket is slow and its drag small. A window that ends 1 s
/// before its top ends at about 1 g's fall of 1 s, 9.8 m/s, where a rocket's drag is a few
/// hundredths of a g.
const COAST_TOP_S: f64 = 1.0;

/// How fast, as a share of standard gravity, the altitude must slow over the coast for a unit to
/// be read: gravity alone near the top, and the drag left there. Feet and meters are 3.28 apart,
/// so at most one unit falls within it; a clock 1,000 times off puts the slowing 10⁶ times off.
const COAST_BAND: (f64, f64) = (0.7, 1.3);

/// The most times the coast window may be cut short at a jump or a motor's marker before the
/// reader gives up on finding a clean coast: a flight has a few such rows near its top, and a
/// file with more, such as one spike every few rows, would otherwise cost a scan of the window
/// for each.
const COAST_CUTS: usize = 16;

/// What a marker column marks, for the coast window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkerRole {
    /// A detection that changes nothing on the rocket: launch, the end of the Mach lockout, the
    /// top speed and acceleration.
    Detection,
    /// The motor: a burnout or a boost; within [`COAST_SEARCH_S`] of the window's end, it ends
    /// the window.
    Motor,
    /// A charge, or a detection a charge may fire at: the apogee, the nose-over, a deployment
    /// channel, and any name the reader doesn't know (a Proton's owner names its channels). One at
    /// or before the top ends the window.
    Charge,
}

/// What a marker column marks, by its name.
fn marker_role(name: &str) -> MarkerRole {
    let numbered = |prefix: &str| {
        name.strip_prefix(prefix)
            .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
    };
    if ["LDA", "LowV", "MaxV", "MaxA"].contains(&name) {
        MarkerRole::Detection
    } else if name == "BurnT" || numbered("BO#") || numbered("Boost#") {
        MarkerRole::Motor
    } else {
        MarkerRole::Charge
    }
}

/// The coast window and how fast each altitude column slows over it.
struct Coast {
    /// The window as a note describes it: its rows, its times and what ends it.
    window: String,
    /// For each column, as [`Column`]s are, how fast it slows over the window, its unit per
    /// second squared (positive slowing as it rises); `None` for a column that isn't an altitude
    /// or can't be fitted.
    slowing: Vec<Option<f64>>,
}

/// The median of `values`' finite numbers from `row - half` to `row + half`, with `half` at most
/// [`LEVEL_ROWS`] and shrunk to stay within `from..=to`, so that the level of a steady climb is
/// the climb; `NaN` without one.
fn level(values: &[f64], from: usize, to: usize, row: usize) -> f64 {
    let half = LEVEL_ROWS
        .min(row.saturating_sub(from))
        .min(to.saturating_sub(row));
    let mut near: Vec<f64> = values
        .get(row.saturating_sub(half)..=row.saturating_add(half).min(to))
        .unwrap_or_default()
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    if near.is_empty() {
        return f64::NAN;
    }
    near.sort_by(f64::total_cmp);
    let middle = near.len() / 2;
    // Invariant: `near` isn't empty, so `middle` and, for an even count, the one before it are in
    // it.
    if near.len().is_multiple_of(2) {
        (near[middle - 1] + near[middle]) / 2.0
    } else {
        near[middle]
    }
}

/// The least-squares parabola through points `(x, y)`: its slope and its second derivative at
/// `x = 0`; `None` where they aren't defined.
fn parabola(points: &[(f64, f64)]) -> Option<(f64, f64)> {
    let (mut s, mut t) = ([0.0_f64; 5], [0.0_f64; 3]);
    for &(x, y) in points {
        let mut power = 1.0;
        for (order, sum) in s.iter_mut().enumerate() {
            *sum += power;
            if let Some(moment) = t.get_mut(order) {
                *moment += power * y;
            }
            power *= x;
        }
    }
    // The normal equations, `[s0 s1 s2; s1 s2 s3; s2 s3 s4] (a, b, c) = (t0, t1, t2)`, by Cramer's
    // rule.
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let matrix = [[s[0], s[1], s[2]], [s[1], s[2], s[3]], [s[2], s[3], s[4]]];
    let whole = det(matrix);
    let solve = |column: usize| {
        let mut m = matrix;
        for (row, value) in m.iter_mut().zip(t) {
            row[column] = value;
        }
        det(m) / whole
    };
    let (slope, half_curvature) = (solve(1), solve(2));
    (slope.is_finite() && half_curvature.is_finite()).then_some((slope, 2.0 * half_curvature))
}

/// The coast window before `Alt`'s top (module docs, *The coast check*) and how fast each altitude
/// column slows over it; why there is none otherwise.
fn coast(columns: &[Column], written: &[Vec<f64>], time_s: &[f64]) -> Result<Coast, String> {
    // Invariant: `Alt` is the header's name after `T` (`is_header`), so it is the first column.
    let alt = written.first().map_or(&[][..], Vec::as_slice);
    let count = alt.len().min(time_s.len());
    let Some(last) = count.checked_sub(1) else {
        return Err("the table has no rows".to_owned());
    };
    let levels: Vec<f64> = (0..count).map(|row| level(alt, 0, last, row)).collect();
    let highest = |values: &[f64], from: usize| {
        values
            .iter()
            .enumerate()
            .filter(|(_, value)| value.is_finite())
            .fold(
                None,
                |best: Option<(usize, f64)>, (row, &value)| match best {
                    Some((_, high)) if high >= value => best,
                    _ => Some((from + row, value)),
                },
            )
            .map(|(row, _)| row)
    };
    // The top: the highest level, a median that passes over a spike, then the highest row
    // within the rows that median takes in, as a median lags a sharp top.
    let Some(top) = highest(&levels, 0).and_then(|row| {
        let from = row.saturating_sub(LEVEL_ROWS);
        highest(
            alt.get(from..=row.saturating_add(LEVEL_ROWS).min(last))?,
            from,
        )
    }) else {
        return Err("`Alt` holds no number".to_owned());
    };
    let time = |row: usize| time_s.get(row).copied().unwrap_or(f64::NAN);
    let marked = |values: &[f64], row: usize| {
        values
            .get(row)
            .is_some_and(|value| *value != 0.0 && !value.is_nan())
    };
    let markers = |role: MarkerRole| {
        columns
            .iter()
            .zip(written)
            .filter(move |(column, _)| {
                column.kind == Kind::Marker && marker_role(&column.name) == role
            })
            .map(|(column, values)| (short(&column.name), values.as_slice()))
    };

    // The window ends at the top, or before the first charge at or before it.
    let mut end = top;
    let mut ending = "up to the altitude's top".to_owned();
    let charge = markers(MarkerRole::Charge)
        .filter_map(|(name, values)| {
            (0..=top)
                .find(|row| marked(values, *row))
                .map(|row| (row, name))
        })
        .min_by_key(|(row, _)| *row);
    if let Some((row, name)) = charge {
        let Some(before) = row.checked_sub(1) else {
            return Err(format!(
                "`{name}` is marked on the first row, before any coast"
            ));
        };
        end = before;
        ending = format!(
            "up to the row before the marker `{name}` at {} s",
            plain(time(row), 3)
        );
    }
    // The first row at or after a time, at or before `end`: the times strictly increase (the
    // reader refuses a file whose time doesn't), so a binary search finds it.
    let first_from = |end: usize, from_s: f64| {
        let row = time_s
            .get(..=end)
            .map_or(0, |times| times.partition_point(|t| *t < from_s));
        (row <= end).then_some(row)
    };
    let motors: Vec<(String, &[f64])> = markers(MarkerRole::Motor).collect();
    // Then before the first jump, or motor marker, within the search span before the end; each
    // move ends it earlier, so the loop ends, after at most `COAST_CUTS` moves.
    let mut cuts = 0;
    let first = loop {
        let end_s = time(end);
        let Some(first) = first_from(end, end_s - COAST_WINDOW_S) else {
            return Err("the table holds no row before the window's end".to_owned());
        };
        if time(0) > end_s - COAST_WINDOW_S {
            return Err(format!(
                "the table starts {} s before the coast's end, {}, short of the {} s window",
                plain(end_s - time(0), 3),
                ending,
                plain(COAST_WINDOW_S, 1)
            ));
        }
        let rise = level(alt, 0, last, end) - level(alt, 0, last, first);
        if rise.is_nan() || rise <= 0.0 {
            return Err(format!(
                "`Alt` doesn't rise over the {} s {ending}",
                plain(COAST_WINDOW_S, 1)
            ));
        }
        let from = first_from(end, end_s - COAST_SEARCH_S).unwrap_or(end);
        let stop = (from..=end).find_map(|row| {
            let value = alt.get(row).copied().unwrap_or(f64::NAN);
            let level = levels.get(row).copied().unwrap_or(f64::NAN);
            if value.is_finite() && (value - level).abs() > COAST_JUMP_SHARE * rise {
                return Some((
                    row,
                    format!(
                        "up to the row before `Alt` jumps at {} s",
                        plain(time(row), 3)
                    ),
                ));
            }
            motors
                .iter()
                .find(|(_, values)| {
                    marked(values, row) && !row.checked_sub(1).is_some_and(|up| marked(values, up))
                })
                .map(|(name, _)| {
                    (
                        row,
                        format!(
                            "up to the row before the motor's marker `{name}` at {} s",
                            plain(time(row), 3)
                        ),
                    )
                })
        });
        match stop {
            Some((row, why)) => {
                cuts += 1;
                if cuts > COAST_CUTS {
                    return Err(format!(
                        "the coast window is cut short more than {COAST_CUTS} times by jumps or \
                         motor markers before the top, the last {why}"
                    ));
                }
                let Some(before) = row.checked_sub(1) else {
                    return Err(format!("the coast window ends at the first row, {why}"));
                };
                end = before;
                ending = why;
            }
            None => break first,
        }
    };

    let (start_s, end_s) = (time(first), time(end));
    let window = format!(
        "the rows from {} s to {} s, {ending}",
        plain(start_s, 3),
        plain(end_s, 3)
    );
    let fit = |values: &[f64]| {
        let points: Vec<(f64, f64)> = (first..=end)
            .map(|row| (time(row) - end_s, level(values, first, end, row)))
            .filter(|(_, value)| value.is_finite())
            .collect();
        (points.len(), parabola(&points))
    };
    let (rows, alt_fit) = fit(alt);
    if rows < COAST_ROWS {
        return Err(format!(
            "over {window}, `Alt` holds {rows} numbers, under the {COAST_ROWS} the reader asks"
        ));
    }
    let Some((slope, curvature)) = alt_fit.filter(|(_, curvature)| *curvature < 0.0) else {
        return Err(format!(
            "over {window}, the parabola fitted to `Alt` doesn't turn down"
        ));
    };
    let top_after_s = -slope / curvature;
    if top_after_s.abs() > COAST_TOP_S {
        return Err(format!(
            "over {window}, the parabola fitted to `Alt` tops out {} s from the window's end, \
             further than the {} s the reader asks",
            plain(top_after_s, 2),
            plain(COAST_TOP_S, 1)
        ));
    }
    let slowing = columns
        .iter()
        .zip(written)
        .enumerate()
        .map(|(index, (column, values))| match column.kind {
            Kind::Altitude if index == 0 => Some(-curvature),
            Kind::FilteredAltitude => match fit(values) {
                (rows, Some((_, curvature))) if rows >= COAST_ROWS => Some(-curvature),
                _ => None,
            },
            _ => None,
        })
        .collect();
    Ok(Coast { window, slowing })
}

/// Standard gravity in feet per second squared.
fn gravity_ft_s2() -> f64 {
    STANDARD_GRAVITY_MPS2 / FOOT_M
}

/// Whether a slowing, in g, is within [`COAST_BAND`].
fn in_band(slowing_g: f64) -> bool {
    (COAST_BAND.0..=COAST_BAND.1).contains(&slowing_g)
}

/// The terminal download's clock and altitudes, decided by the step and the manuals and checked
/// against the coast (module docs, *The coast check*): the clock's note, and for each column, as
/// [`Column`]s are, the note deciding an altitude and whether it is kept as written.
struct CoastUnits {
    clock: String,
    altitudes: Vec<Option<(String, bool)>>,
}

/// Decides the terminal download's clock and altitudes (module docs, *The coast check*);
/// [`LogError::Unit`] on the header's line for a clock the coast doesn't bear out.
fn coast_units(
    columns: &[Column],
    written: &[Vec<f64>],
    time_s: &[f64],
    clock: Option<(f64, f64)>,
    header_line: usize,
) -> Result<CoastUnits, LogError> {
    let refuse = |message: String| LogError::Unit {
        format: FORMAT,
        line: header_line,
        message,
    };
    let Some((per_second, median)) = clock else {
        return Err(refuse(
            "the clock `T` can't be settled: the table holds one row, so it has no step and no \
             coast to tell seconds from milliseconds"
                .to_owned(),
        ));
    };
    let unit = clock_unit(per_second);
    let coast = coast(columns, written, time_s).map_err(|why| {
        refuse(format!(
            "the clock `T` is in {unit} by its step, but no coast bears that out: {why}"
        ))
    })?;
    let g_ft = gravity_ft_s2();
    let slowing = coast.slowing.first().copied().flatten().unwrap_or(f64::NAN);
    let (in_feet, in_meters) = (slowing / g_ft, slowing / STANDARD_GRAVITY_MPS2);
    let length = if in_band(in_feet) {
        "feet"
    } else if in_band(in_meters) {
        "meters"
    } else {
        return Err(refuse(format!(
            "the clock `T` is in {unit} by its step, but on that clock `Alt` slows at {} g in \
             feet and {} g in meters over {}, in neither the {} to {} g the reader takes",
            plain(in_feet, 3),
            plain(in_meters, 3),
            coast.window,
            plain(COAST_BAND.0, 1),
            plain(COAST_BAND.1, 1)
        )));
    };
    let band = format!("{} to {} g", plain(COAST_BAND.0, 1), plain(COAST_BAND.1, 1));
    let clock_note = format!(
        "`T` read in {unit}, from the step and the coast: {}; on that clock `Alt` slows at {} g \
         in {length} over {}, where the reader takes {band} in feet or in meters and a clock \
         1,000 times off would put it 10⁶ times off",
        step_evidence(median),
        plain(if length == "feet" { in_feet } else { in_meters }, 3),
        coast.window
    );
    let alt_feet = length == "feet";
    let altitudes = columns
        .iter()
        .zip(&coast.slowing)
        .enumerate()
        .map(|(index, (column, slowing))| {
            let name = short(&column.name);
            match column.kind {
                Kind::Altitude if index == 0 && alt_feet => Some((
                    format!(
                        "`{name}` read in feet, from the manual and the coast: {ALTITUDE_MANUALS}; \
                         over {}, it slows at {} g in feet, within the {band} the reader takes, \
                         where in meters it would be {} g{}",
                        coast.window,
                        plain(in_feet, 3),
                        plain(in_meters, 3),
                        accelerometer_ratio(columns, written)
                    ),
                    false,
                )),
                Kind::Altitude if index == 0 => Some((
                    format!(
                        "`{name}` kept as written: the manuals give feet, but over {}, it slows \
                         at {} g in feet and {} g in meters, where the reader takes {band} in \
                         feet; the record holds no barometric altitude",
                        coast.window,
                        plain(in_feet, 3),
                        plain(in_meters, 3)
                    ),
                    true,
                )),
                Kind::FilteredAltitude => Some(match slowing.map(|value| value / g_ft) {
                    Some(g) if in_band(g) => (
                        format!(
                            "`{name}` read in feet, from the manual and the coast: it is the \
                             logger's filtered `Alt` (Quantum p38, Proton p45), in feet; over \
                             {}, it slows at {} g in feet, within the {band} the reader takes",
                            coast.window,
                            plain(g, 3)
                        ),
                        false,
                    ),
                    Some(g) => (
                        format!(
                            "`{name}` kept as written: the manuals give it in feet, as the \
                             logger's filtered `Alt` (Quantum p38, Proton p45), but over {}, it \
                             slows at {} g in feet, outside the {band} the reader takes",
                            coast.window,
                            plain(g, 3)
                        ),
                        true,
                    ),
                    None => (
                        format!(
                            "`{name}` kept as written: the manuals give it in feet, as the \
                             logger's filtered `Alt` (Quantum p38, Proton p45), but no parabola \
                             can be fitted to it over {}",
                            coast.window
                        ),
                        true,
                    ),
                }),
                _ => None,
            }
        })
        .collect();
    Ok(CoastUnits {
        clock: clock_note,
        altitudes,
    })
}

/// The note saying how `Alt` was read in feet; for a file with the accelerometer in both g and
/// ft/s², what the two say of each other.
fn altitude_note(columns: &[Column], written: &[Vec<f64>]) -> String {
    format!(
        "`Alt` read in feet, from the manual: {ALTITUDE_MANUALS}{}",
        accelerometer_ratio(columns, written)
    )
}

/// What the manuals say of `Alt`'s unit, as a note quotes it.
const ALTITUDE_MANUALS: &str = "the Classic and TRS manuals (p37) state feet; the Quantum and \
                                Proton manuals give every height in feet and no Eggtimer manual \
                                offers meters";

/// For a file with the accelerometer in both g and ft/s², what the two say of each other, as a
/// clause the altitude's note ends with; empty otherwise.
fn accelerometer_ratio(columns: &[Column], written: &[Vec<f64>]) -> String {
    let mut note = String::new();
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
/// [`read`] refuses in the table, naming the file's line; [`LogError::Unit`] on the header's line
/// for a clock its step and the coast don't both bear out (module docs, *The coast check*).
pub fn read_text(text: &str) -> Result<FlightLog, LogError> {
    read_download(text, TEXT_FORMAT, &|line| line)
}

/// A terminal download read as [`read_text`] reads it, its errors naming `format` and the line
/// `file_line` gives for each of the text's lines: the line itself in a text file, the RTF
/// document's line in [`read_rtf`].
fn read_download(
    text: &str,
    format: &'static str,
    file_line: &dyn Fn(usize) -> usize,
) -> Result<FlightLog, LogError> {
    let Some(download) = split_download(text) else {
        return Err(LogError::NotThisFormat {
            format,
            message: format!(
                "no title `Eggtimer … Flight Settings` comes before an Eggtimer header row, \
                 such as `T,Alt,Veloc,…`, within the first {MAX_PREAMBLE_LINES} lines that \
                 aren't blank"
            ),
        });
    };
    let shift = download.header_line.saturating_sub(1);
    let mut log = read_table(download.table, Checks::Coast).map_err(|error| match error {
        LogError::Syntax { line, message, .. } => LogError::Syntax {
            format,
            line: file_line(line.saturating_add(shift)),
            message,
        },
        LogError::Unit { line, message, .. } => LogError::Unit {
            format,
            line: file_line(line.saturating_add(shift)),
            message,
        },
        LogError::NoData { .. } => LogError::NoData { format },
        LogError::NotThisFormat { message, .. } => LogError::NotThisFormat { format, message },
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
             computer (TRS manual p24), left out of the record as free text that may name a \
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
/// writes zeroes for a channel that didn't fire (TRS p25); a time past the table's last, by more
/// than [`SAME_TIME_S`], is no event either, as no event of the flight comes after its last
/// sample: such a time isn't in seconds. The events end up in time order, the table's first where
/// two share a time.
fn summary_events(log: &mut FlightLog, preamble: &Preamble) {
    if preamble.times.is_empty() {
        return;
    }
    let (mut same, mut own, mut differ, mut zero, mut past) = (
        Items::default(),
        Items::default(),
        Items::default(),
        Items::default(),
        Items::default(),
    );
    let last_s = log.time_s.last().copied().unwrap_or(0.0);
    let mut events = Vec::new();
    for (name, time_s) in &preamble.times {
        if *time_s == 0.0 {
            zero.push(format!("`{name}`"));
            continue;
        }
        if *time_s > last_s + SAME_TIME_S {
            past.push(format!("`{name}` {}", plain(*time_s, 3)));
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
    // Said only of times read: those of 0 and those past the table aren't.
    if !(same.is_empty() && own.is_empty() && differ.is_empty()) {
        log.notes.push(how.to_owned());
    }
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
             channel that didn't fire (p25): {}",
            zero.joined(", ")
        ));
    }
    if !past.is_empty() {
        log.notes.push(format!(
            "the flight summary's times past the table's last, {} s, kept as written and not \
             read as events: no event of the flight comes after its last sample, so they aren't \
             in seconds: {}",
            plain(last_s, 3),
            past.joined(", ")
        ));
    }
    log.events.extend(events);
    log.events
        .sort_by(|first, second| first.time_s.total_cmp(&second.time_s));
}

// The terminal download saved inside an RTF document (ADR-258 §4).

/// The RTF-wrapped download's name in errors.
const RTF_FORMAT: &str = "Eggtimer RTF";

/// The deepest an RTF document's groups may nest. Word processors nest a few levels, a style or a
/// table a few more; a document deeper than this is refused rather than tracked.
pub const MAX_RTF_DEPTH: usize = 64;

/// The most letters of a control word the reader keeps: the RTF specification caps a control word
/// at 32 characters. A longer one is read to its end and matches no word the reader knows.
const MAX_CONTROL_WORD: usize = 32;

/// The most fallback characters a `\ucN` may ask to be skipped after each `\uN`; the
/// specification's default is 1.
const MAX_FALLBACK: usize = 16;

/// What Windows code page 1252 maps bytes 0x80 to 0x9F to, in order; `None` for the five bytes it
/// leaves undefined. From 0xA0 on, the code page is ISO 8859-1, whose bytes are the same Unicode
/// code points (the Unicode Consortium's mapping table for it, `CP1252.TXT`).
const CP1252_80_9F: [Option<char>; 32] = [
    Some('\u{20ac}'),
    None,
    Some('\u{201a}'),
    Some('\u{0192}'),
    Some('\u{201e}'),
    Some('\u{2026}'),
    Some('\u{2020}'),
    Some('\u{2021}'),
    Some('\u{02c6}'),
    Some('\u{2030}'),
    Some('\u{0160}'),
    Some('\u{2039}'),
    Some('\u{0152}'),
    None,
    Some('\u{017d}'),
    None,
    None,
    Some('\u{2018}'),
    Some('\u{2019}'),
    Some('\u{201c}'),
    Some('\u{201d}'),
    Some('\u{2022}'),
    Some('\u{2013}'),
    Some('\u{2014}'),
    Some('\u{02dc}'),
    Some('\u{2122}'),
    Some('\u{0161}'),
    Some('\u{203a}'),
    Some('\u{0153}'),
    None,
    Some('\u{017e}'),
    Some('\u{0178}'),
];

/// Whether the text is an RTF document: it opens with `{\rtf`, after any byte-order mark and
/// white space.
pub(crate) fn recognizes_rtf(text: &str) -> bool {
    text.trim_start_matches(|c: char| c == '\u{feff}' || c.is_whitespace())
        .starts_with("{\\rtf")
}

/// An RTF document's plain text, its lines each mapped to the document's line it starts on.
#[derive(Debug, Default)]
struct Unwrapped {
    /// The text, its lines ended by `\n` alone.
    text: String,
    /// For each of the text's lines, the document's line, from 1, that it starts on.
    starts: Vec<usize>,
    /// Whether the next character written starts a line.
    at_line_start: bool,
    /// Escaped bytes the document's code page leaves undefined, or in a code page the reader
    /// doesn't map, each read as U+FFFD.
    unmapped: usize,
    /// Characters after the document's closing brace that aren't white space, left unread.
    after_end: usize,
}

impl Unwrapped {
    /// Writes a character found on the document's line `line`; CR and LF end a line.
    fn push(&mut self, c: char, line: usize) {
        if c == '\r' || c == '\n' {
            self.line_break(line);
            return;
        }
        if self.at_line_start {
            self.starts.push(line);
            self.at_line_start = false;
        }
        self.text.push(c);
    }

    /// Ends the text's line, at the document's line `line`.
    fn line_break(&mut self, line: usize) {
        if self.at_line_start {
            self.starts.push(line);
        }
        self.text.push('\n');
        self.at_line_start = true;
    }

    /// The document's line that the text's line `line`, from 1, starts on; past the last, the
    /// last's.
    fn file_line(&self, line: usize) -> usize {
        line.checked_sub(1)
            .and_then(|index| self.starts.get(index))
            .or(self.starts.last())
            .copied()
            .unwrap_or(1)
    }
}

/// One group's state: whether its text is skipped, as a destination the reader doesn't read, and
/// how many fallback characters follow each `\uN` in it.
#[derive(Debug, Clone, Copy)]
struct Group {
    skip: bool,
    fallback: usize,
}

/// A refusal of the RTF document's markup at its line.
fn rtf_syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: RTF_FORMAT,
        line,
        message,
    }
}

/// The plain text of an RTF document, by the subset of the RTF specification that
/// [`read_rtf`] lists.
fn unwrap_rtf(document: &str) -> Result<Unwrapped, LogError> {
    let mut out = Unwrapped {
        at_line_start: true,
        ..Unwrapped::default()
    };
    let mut chars = document
        .strip_prefix('\u{feff}')
        .unwrap_or(document)
        .chars();
    let mut line = 1_usize;
    let mut groups: Vec<Group> = Vec::new();
    let mut opened = false;
    let mut code_page_1252 = true;
    // Fallback characters left to skip after a `\uN`.
    let mut skip_fallback = 0_usize;
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.as_str().starts_with('\n') {
                    chars.next();
                }
                line += 1;
            }
            '\n' => line += 1,
            '{' => {
                if groups.len() >= MAX_RTF_DEPTH {
                    return Err(rtf_syntax(
                        line,
                        format!("the document's groups nest deeper than {MAX_RTF_DEPTH}"),
                    ));
                }
                let group = groups.last().copied().unwrap_or(Group {
                    skip: false,
                    fallback: 1,
                });
                groups.push(group);
                opened = true;
                skip_fallback = 0;
            }
            '}' => {
                if groups.pop().is_none() {
                    return Err(rtf_syntax(
                        line,
                        "a `}` closes a group that was never opened".to_owned(),
                    ));
                }
                skip_fallback = 0;
                if groups.is_empty() {
                    out.after_end = chars.filter(|c| !c.is_whitespace()).count();
                    return Ok(out);
                }
            }
            _ if groups.is_empty() => {
                if !c.is_whitespace() {
                    return Err(rtf_syntax(
                        line,
                        "text comes before the document's opening `{`".to_owned(),
                    ));
                }
            }
            '\\' => {
                let skip = groups.last().is_some_and(|group| group.skip);
                let Some(next) = chars.next() else {
                    break;
                };
                if next.is_ascii_alphabetic() {
                    let mut word = String::new();
                    word.push(next);
                    while let Some(letter) = chars.as_str().chars().next() {
                        if !letter.is_ascii_alphabetic() {
                            break;
                        }
                        chars.next();
                        if word.len() <= MAX_CONTROL_WORD {
                            word.push(letter);
                        }
                    }
                    let parameter = control_parameter(&mut chars);
                    if chars.as_str().starts_with(' ') {
                        chars.next();
                    }
                    control_word(
                        &word,
                        parameter,
                        &mut ControlState {
                            out: &mut out,
                            groups: &mut groups,
                            chars: &mut chars,
                            line: &mut line,
                            code_page_1252: &mut code_page_1252,
                            skip_fallback: &mut skip_fallback,
                        },
                    );
                    continue;
                }
                let symbol = match next {
                    '\\' | '{' | '}' => Some(next),
                    '\'' => {
                        let high = chars.next().and_then(|c| c.to_digit(16));
                        let low = chars.next().and_then(|c| c.to_digit(16));
                        let (Some(high), Some(low)) = (high, low) else {
                            return Err(rtf_syntax(
                                line,
                                "a `\\'` escape isn't followed by two hexadecimal digits"
                                    .to_owned(),
                            ));
                        };
                        let c = code_page_character(high * 16 + low, code_page_1252);
                        if c.is_none() && !skip && skip_fallback == 0 {
                            out.unmapped += 1;
                        }
                        Some(c.unwrap_or(char::REPLACEMENT_CHARACTER))
                    }
                    '~' => Some('\u{a0}'),
                    '_' => Some('-'),
                    '\r' | '\n' => {
                        // `\` at a line's end is `\par`.
                        if !skip {
                            out.line_break(line);
                        }
                        if next == '\r' && chars.as_str().starts_with('\n') {
                            chars.next();
                        }
                        line += 1;
                        continue;
                    }
                    // `\*` marks a destination a reader that doesn't know it may skip: this one
                    // skips every such destination.
                    '*' => {
                        if let Some(group) = groups.last_mut() {
                            group.skip = true;
                        }
                        continue;
                    }
                    // `\-`, an optional hyphen, and the symbols the reader doesn't know.
                    _ => None,
                };
                if let Some(c) = symbol
                    && !skip
                {
                    if skip_fallback > 0 {
                        skip_fallback -= 1;
                    } else {
                        out.push(c, line);
                    }
                }
            }
            _ => {
                if groups.last().is_some_and(|group| group.skip) {
                    continue;
                }
                if skip_fallback > 0 {
                    skip_fallback -= 1;
                } else if c == '\t' || !c.is_control() {
                    out.push(c, line);
                }
            }
        }
    }
    if opened {
        Err(rtf_syntax(
            line,
            format!(
                "the document ends with {} groups open: its braces don't balance",
                groups.len()
            ),
        ))
    } else {
        Err(LogError::NotThisFormat {
            format: RTF_FORMAT,
            message: "the file holds no RTF group, `{\\rtf…}`".to_owned(),
        })
    }
}

/// What a control word may change as [`unwrap_rtf`] reads it.
struct ControlState<'a, 'b> {
    out: &'a mut Unwrapped,
    groups: &'a mut Vec<Group>,
    chars: &'a mut std::str::Chars<'b>,
    line: &'a mut usize,
    code_page_1252: &'a mut bool,
    skip_fallback: &'a mut usize,
}

/// Acts on one control word and its number, if any.
fn control_word(word: &str, parameter: Option<i64>, state: &mut ControlState<'_, '_>) {
    let skip = state.groups.last().is_some_and(|group| group.skip);
    match word {
        "fonttbl" | "colortbl" | "stylesheet" | "info" | "pict" | "object" => {
            if let Some(group) = state.groups.last_mut() {
                group.skip = true;
            }
        }
        "uc" => {
            if let Some(group) = state.groups.last_mut() {
                group.fallback = usize::try_from(parameter.unwrap_or(1))
                    .map_or(0, |count| count.min(MAX_FALLBACK));
            }
        }
        "ansicpg" => *state.code_page_1252 = parameter == Some(1252),
        "mac" | "pc" | "pca" => *state.code_page_1252 = false,
        "bin" => {
            // The data's bytes aren't text: skipped, however many it states, to the document's
            // end at most.
            let count = usize::try_from(parameter.unwrap_or(0)).unwrap_or(0);
            for _ in 0..count {
                match state.chars.next() {
                    Some('\n') => *state.line += 1,
                    Some(_) => {}
                    None => break,
                }
            }
        }
        _ if skip => {}
        "u" => {
            // A signed 16-bit number, negative for the code points from 0x8000.
            let code = parameter.unwrap_or(0);
            let code = if code < 0 { code + 0x1_0000 } else { code };
            let c = u32::try_from(code)
                .ok()
                .and_then(char::from_u32)
                .unwrap_or(char::REPLACEMENT_CHARACTER);
            state.out.push(c, *state.line);
            *state.skip_fallback = state.groups.last().map_or(1, |group| group.fallback);
        }
        "par" | "line" | "sect" | "page" | "row" => state.out.line_break(*state.line),
        "tab" | "cell" => state.out.push('\t', *state.line),
        _ => {
            if let Some(c) = word_character(word) {
                state.out.push(c, *state.line);
            }
        }
    }
}

/// A control word's number, if one follows: an optional `-` and digits, read saturating at the
/// `i64` range, so a number of any length is read without overflow.
fn control_parameter(chars: &mut std::str::Chars<'_>) -> Option<i64> {
    let mut ahead = chars.as_str().chars();
    let negative = ahead.next() == Some('-') && ahead.next().is_some_and(|c| c.is_ascii_digit());
    if negative {
        chars.next();
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.as_str().chars().next().and_then(|c| c.to_digit(10)) {
        chars.next();
        value = Some(
            value
                .unwrap_or(0)
                .saturating_mul(10)
                .saturating_add(i64::from(digit)),
        );
    }
    value.map(|value| if negative { -value } else { value })
}

/// The character a `\'hh` byte stands for: ASCII below 0x80; from 0x80, Windows 1252's if that
/// is the document's code page, `None` where it leaves the byte undefined or in another code page.
fn code_page_character(byte: u32, code_page_1252: bool) -> Option<char> {
    match byte {
        0..0x80 => char::from_u32(byte),
        _ if !code_page_1252 => None,
        0x80..0xa0 => usize::try_from(byte - 0x80)
            .ok()
            .and_then(|index| CP1252_80_9F.get(index))
            .copied()
            .flatten(),
        _ => char::from_u32(byte),
    }
}

/// The character a control word writes, for the few words that write one.
fn word_character(word: &str) -> Option<char> {
    match word {
        "emdash" => Some('\u{2014}'),
        "endash" => Some('\u{2013}'),
        "emspace" | "enspace" | "qmspace" => Some(' '),
        "bullet" => Some('\u{2022}'),
        "lquote" => Some('\u{2018}'),
        "rquote" => Some('\u{2019}'),
        "ldblquote" => Some('\u{201c}'),
        "rdblquote" => Some('\u{201d}'),
        _ => None,
    }
}

/// Whether a line of a download's text could be one of its table's rows: it opens with a digit,
/// a sign or a point. A line that doesn't, after a table, is the next download's lead-in, such as
/// its `Flight Summary:` heading.
fn row_like(line: &str) -> bool {
    line.trim_start()
        .starts_with(|c: char| c.is_ascii_digit() || matches!(c, '-' | '+' | '.'))
}

/// The lines, from 1, of each download's title after the line `after_line`: a line `Eggtimer …`
/// ending in `Flight Settings`, or followed by a line `Flight Settings`, as [`split_download`]
/// reads a title.
fn later_titles(text: &str, after_line: usize) -> Vec<usize> {
    let lines: Vec<(usize, &str)> = numbered_lines(text)
        .map(|(number, line)| (number, line.trim()))
        .filter(|(number, line)| *number > after_line && !line.is_empty())
        .collect();
    lines
        .iter()
        .enumerate()
        .filter(|(index, (_, line))| {
            line.strip_prefix("Eggtimer ").is_some_and(|rest| {
                rest.ends_with("Flight Settings")
                    || lines
                        .get(index + 1)
                        .is_some_and(|(_, next)| *next == "Flight Settings")
            })
        })
        .map(|(_, (number, _))| *number)
        .collect()
}

/// Reads an Eggtimer TRS's terminal download saved inside an RTF document, as a text editor saves
/// what a terminal program captured: the document's plain text, unwrapped from its markup, read
/// as [`read_text`] reads a terminal download ([ADR-258][adr-258] §4).
///
/// The text is unwrapped by the subset of the RTF specification (Microsoft, *Rich Text Format
/// (RTF) Specification*, version 1.9.1, 2008) that a saved terminal capture needs:
///
/// - groups, `{` to `}`, nested at most [`MAX_RTF_DEPTH`] deep; the document is one group, and
///   an unbalanced brace is refused;
/// - control words, `\` and letters, then an optional signed number and one space ending the
///   word, which isn't text; a number of any length is read saturating, never overflowing;
/// - `\par`, `\line`, `\sect`, `\page` and `\row` as line breaks, and `\` at a line's end as
///   `\par`; `\tab` and `\cell` as a tab; `\emdash`, `\endash`, `\bullet`, the space words and
///   the quote words as their characters;
/// - control symbols: `\\`, `\{` and `\}` as the character; `\~` a no-break space, `\_` a hyphen,
///   `\-` nothing; `\'hh` a byte in the document's code page, Windows 1252 (`\ansi`, the default,
///   or `\ansicpg1252`), any other code page's bytes from 0x80 counted and read as U+FFFD;
/// - `\uN` a Unicode character, the `\ucN` fallback characters after it skipped;
/// - destinations skipped whole: a group opening `\*`, and the font, color and style tables, the
///   document's information, pictures and objects; `\binN`'s N bytes skipped;
/// - raw line ends, other control words and other control symbols ignored, as the specification
///   says a reader may.
///
/// The work is one pass with a stack of groups, no recursion, and the text is never longer than
/// the document.
///
/// A document may hold several downloads one after another, each with its title, settings,
/// summary and table. A record holds one flight: the first download is read, its table ending at
/// its last row before the next download's lead-in, and the others are counted in a note with the
/// lines of their titles. An error names the RTF document's line.
///
/// [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the file isn't an RTF document, or if its text holds no Eggtimer
/// download: no title `Eggtimer … Flight Settings` before an Eggtimer header row within the first
/// [`MAX_PREAMBLE_LINES`] lines that aren't blank; [`LogError::Syntax`] for RTF markup the reader
/// refuses (unbalanced braces, groups nested deeper than [`MAX_RTF_DEPTH`], a `\'` escape without
/// two hexadecimal digits); otherwise what [`read_text`] refuses, naming the document's line.
pub fn read_rtf(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes_rtf(text) {
        return Err(LogError::NotThisFormat {
            format: RTF_FORMAT,
            message: "the file doesn't open with `{\\rtf`, an RTF document's opening group"
                .to_owned(),
        });
    }
    let unwrapped = unwrap_rtf(text)?;
    let Some(first) = split_download(&unwrapped.text) else {
        return Err(LogError::NotThisFormat {
            format: RTF_FORMAT,
            message: format!(
                "the RTF document's text holds no Eggtimer terminal download: no title \
                 `Eggtimer … Flight Settings` comes before an Eggtimer header row, such as \
                 `T,Alt,Veloc,…`, within the first {MAX_PREAMBLE_LINES} lines that aren't blank"
            ),
        });
    };
    let others = later_titles(&unwrapped.text, first.header_line);
    // The first download ends at its last row-like line before the next one's title.
    let download = match others.first() {
        Some(&next_title) => {
            let last_row = numbered_lines(&unwrapped.text)
                .take_while(|(number, _)| *number < next_title)
                .filter(|(_, line)| row_like(line))
                .map(|(number, _)| number)
                .last()
                .unwrap_or(first.header_line)
                .max(first.header_line);
            // The text's lines end in `\n` alone, so its line `last_row` ends at that newline.
            let end = unwrapped
                .text
                .match_indices('\n')
                .nth(last_row.saturating_sub(1))
                .map_or(unwrapped.text.len(), |(at, _)| at);
            unwrapped.text.get(..end).unwrap_or(&unwrapped.text)
        }
        None => unwrapped.text.as_str(),
    };
    let mut log = read_download(download, RTF_FORMAT, &|line| unwrapped.file_line(line))?;
    log.format = LogFormat::EggtimerRtf;
    log.notes.insert(
        0,
        "the text was unwrapped from an RTF document, as a text editor saves a terminal \
         program's capture: its groups, control words and escapes taken out, its paragraph and \
         line breaks read as the download's lines; a line named is the document's"
            .to_owned(),
    );
    if !others.is_empty() {
        let lines: Vec<String> = others
            .iter()
            .take(MAX_LISTED)
            .map(|line| unwrapped.file_line(*line).to_string())
            .collect();
        let more = others.len().saturating_sub(MAX_LISTED);
        log.notes.push(format!(
            "the document holds {} Eggtimer downloads one after another; the first was read, as \
             a record holds one flight, and the titles of those that weren't are on lines {}{}",
            others.len() + 1,
            lines.join(", "),
            if more > 0 {
                format!(" and {more} more")
            } else {
                String::new()
            }
        ));
    }
    if unwrapped.unmapped > 0 {
        log.notes.push(format!(
            "{} escaped bytes are undefined in the document's code page, or in a code page the \
             reader doesn't map (it maps Windows 1252), and were read as U+FFFD",
            unwrapped.unmapped
        ));
    }
    if unwrapped.after_end > 0 {
        log.notes.push(format!(
            "{} characters after the document's closing brace weren't read",
            unwrapped.after_end
        ));
    }
    Ok(log)
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

    /// A value in g near the largest `f64` is finite as written and infinite in m/s²: refused on
    /// its line, in the CSV and in the terminal download, whose table is the same (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let csv = "T,Alt,AccelG,LDA\n0,0,1,0\n0.05,1,-1.7e308,0\n";
        assert_eq!(
            read(csv).unwrap_err().to_string(),
            "Eggtimer CSV line 3: the column AccelG holds a value too large for its unit's \
             conversion to SI"
        );
        // The download's table: the invented flight's `T` and `Alt`, with `AccelG` beside them.
        let rows: String = flight(1000.0, 0.05, G_FT, None, 15.0)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                let cells: Vec<&str> = line.split(',').take(2).collect();
                let g = if row == 1 { "-1.7e308" } else { "1" };
                format!("{},{g},0\n", cells.join(","))
            })
            .collect();
        let text =
            format!("Eggtimer TRS 2.07 Flight Settings\n...Units...Feet\nT,Alt,AccelG,LDA\n{rows}");
        assert_eq!(
            read_text(&text).unwrap_err().to_string(),
            "Eggtimer text line 5: the column AccelG holds a value too large for its unit's \
             conversion to SI"
        );
        // The largest value that stays finite in m/s² is read.
        let largest = f64::MAX / 9.806_65;
        let log = read(&format!("T,Alt,AccelG\n0,0,{largest:e}\n")).unwrap();
        assert!(log.channels[1].values[0].is_finite());
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

    /// Feet's standard gravity, ft/s².
    const G_FT: f64 = 32.174;

    /// A charge before the top: when it fires, s, and its canopy's descent rate, the table's unit
    /// a second.
    type Drogue = Option<(f64, f64)>;

    /// An invented flight's table rows, in the TRS layout (`TRS`): a boost of 1 s, then a coast
    /// slowing at `slowing` the table's unit a second squared (`G_FT` for 1 g in feet), topping
    /// out at 10 s; or, if `drogue` fires first, a canopy's drag `slowing (v / v_t)²` added from
    /// then on, `v_t` its descent rate, closed-form; then a descent at 40 a second to `end_s`. A
    /// row every `step_s`, `T` written `per_second` to the second, `Alt` in whole units, `Veloc`
    /// and `FVeloc` its change over the step, `FAlt` equal to it; `LDA` at 0.1 s, `LowV` at 5 s,
    /// `Apogee` at the top, `N-O` and `Drogue` at the drogue's time or 10.5 s.
    fn flight(per_second: f64, step_s: f64, slowing: f64, drogue: Drogue, end_s: f64) -> String {
        let boost = 9.0 * slowing;
        let coast = |t: f64| boost / 2.0 + boost * (t - 1.0) - slowing * (t - 1.0).powi(2) / 2.0;
        let altitude = |t: f64| -> f64 {
            if t <= 1.0 {
                return boost * t * t / 2.0;
            }
            match drogue {
                Some((at, rate)) if t > at => {
                    let angle = (slowing * (10.0 - at) / rate).atan();
                    let stop = at + rate * angle / slowing;
                    let rise = |t: f64| {
                        coast(at)
                            + rate * rate / slowing
                                * ((angle - slowing * (t - at) / rate).cos() / angle.cos()).ln()
                    };
                    if t <= stop {
                        rise(t)
                    } else {
                        rise(stop) - 40.0 * (t - stop)
                    }
                }
                _ if t <= 10.0 => coast(t),
                _ => coast(10.0) - 40.0 * (t - 10.0),
            }
        };
        let count = (end_s / step_s).round() as usize;
        let times: Vec<f64> = (0..=count).map(|row| row as f64 * step_s).collect();
        let heights: Vec<f64> = times.iter().map(|t| altitude(*t).round()).collect();
        let at = |time: f64| (time / step_s).round() as usize;
        let top = at(match drogue {
            Some((time, rate)) if time < 10.0 => {
                time + rate * (slowing * (10.0 - time) / rate).atan() / slowing
            }
            _ => 10.0,
        });
        let deploy = at(drogue.map_or(10.5, |(time, _)| time));
        let mut text = String::new();
        for (row, (t, h)) in times.iter().zip(&heights).enumerate() {
            let speed = row
                .checked_sub(1)
                .map_or(0.0, |up| (h - heights[up]) / step_s);
            let mark = |on: bool| if on { *h } else { 0.0 };
            text.push_str(&format!(
                "{},{h},{speed:.2},{h:.2},{speed:.2},{},{},{},{},{},0\n",
                (t * per_second * 1000.0).round() / 1000.0,
                mark(row == at(0.1)),
                mark(row == at(5.0)),
                mark(row == top),
                mark(row == deploy),
                mark(row == deploy),
            ));
        }
        text
    }

    /// The invented flight's table in milliseconds, a row every 50 ms, with no charge before its
    /// top and its header.
    fn table() -> String {
        format!("{TRS}\n{}", flight(1000.0, 0.05, G_FT, None, 15.0))
    }

    /// An invented terminal download: firmware 2.07's spellings, the table in milliseconds.
    fn download() -> String {
        format!(
            "Flight Summary:\n\
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
             Apogee: 1448 ft\n\
             T @ Apogee: 10.000\n\
             Max Velocity: 290 ft/sec\n\
             T @ Max Velocity: 1.000\n\
             Time @ LDA:0.100\n\
             Altitude @ Main Deploy: 0 ft\n\
             T @ Main Deploy: 0.000\n\
             {}",
            table()
        )
    }

    #[test]
    fn recognizes_the_terminal_download_and_no_other_layout() {
        let table = table();
        for text in [
            download(),
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
            download().replace("Flight Settings", "Settings"),
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
        assert!(recognizes(&table) && !recognizes(&download()));
    }

    /// The title, the settings, the summary's heights and times, and the table, as the module's
    /// documentation says each is read.
    #[test]
    fn reads_the_settings_the_summary_and_the_table() {
        let log = read_text(&download()).unwrap();
        assert_eq!(log.format, LogFormat::EggtimerText);
        assert_eq!(log.logger, "Eggtimer TRS");
        assert_eq!(log.firmware.as_deref(), Some("2.07"));
        assert_eq!(log.time_s.len(), 301);
        assert_eq!(log.time_s[..3], [0.0, 0.05, 0.1]);
        assert!(close(log.stated.apogee_m.unwrap(), 1448.0 * 0.3048));
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
                (1.0, "Max Velocity"),
                (5.0, "LowV"),
                (10.0, "Apogee"),
                (10.5, "N-O"),
                (10.5, "Drogue")
            ]
        );
        let notes = &log.notes[6..];
        assert_eq!(
            notes,
            [
                "`ASL Alt.` read in feet, from its unit mark: the flight summary writes 1000 ft; \
                 the launch site's elevation above sea level, as the logger measured it (TRS \
                 manual p36)",
                "`Apogee` read in feet, from its unit mark: the flight summary writes 1448 ft; \
                 the logger's apogee above its zero, kept beside the table's altitudes (TRS \
                 manual p36)",
                "`T @` and `Time @` read in seconds, from the manual: the TRS manual's download \
                 writes the flight summary's times in `secs.` (p36), and its LDA at 0.700 secs. \
                 as \"700ms into the flight\" (p37)",
                "the flight summary's times are its table markers' to the millisecond, one event \
                 each, the marker's: `Apogee` at `Apogee`, `LDA` at `LDA`",
                "the flight summary's times of no marker in the table, each an event of its own: \
                 `Max Velocity`",
                "the flight summary's times of 0, no event, as the TRS manual writes zeroes for a \
                 channel that didn't fire (p25): `Main Deploy`",
                "the flight settings, as the file writes them: `Drogue <1=Normal,2=Backup,0=OFF>` \
                 1; `Main Altitude <50-2000 by 50, 0=OFF>` 300; `Frequency (Ctrl-F to change)` \
                 915; `ID Code` 4",
                "1 line of the flight settings holds no setting: the name its owner gave the \
                 flight computer (TRS manual p24), left out of the record as free text that may \
                 name a person",
                "the flight summary's other lines, as the file writes them: `Base Temp: 20.5 C`; \
                 `Last Flight Phase: 9` (landing detected, TRS manual p37); `Max Velocity: 290 \
                 ft/sec`; `Altitude @ Main Deploy: 0 ft`",
            ]
        );
        assert!(
            log.notes[0].starts_with("`T` read in milliseconds, from the step and the coast: ")
        );
        assert!(log.notes[1].starts_with("`Alt` read in feet, from the manual and the coast: "));
        assert!(log.notes[3].starts_with("`FAlt` read in feet, from the manual and the coast: "));
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
             T @ Apogee: 10.100 secs.\n\
             T @ Nose-Over: 10.5 s\n\
             T @ Unlock: soon\n\
             a line with no colon\n\
             {}",
            table()
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
                (5.0, "LowV"),
                (10.0, "Apogee"),
                (10.1, "Apogee"),
                (10.5, "N-O"),
                (10.5, "Drogue")
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
             as events: `Apogee` at 10.1 s, the marker `Apogee` at 10 s"
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
        let bad = download().replacen("\n100,1,", "\n100,x,", 1);
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
        let download = download();
        let headless = download
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
        // A table in seconds out to 40 s, past the last time, so every time is an event.
        let rows = flight(1.0, 0.05, G_FT, None, 40.0);
        let text = format!(
            "Eggtimer {} 9.99 Flight Settings\n{settings}{times}{TRS}\n{rows}",
            "Z".repeat(500)
        );
        let log = read_text(&text).unwrap();
        assert!(log.logger.chars().count() < 100);
        assert_eq!(log.firmware.as_deref(), Some("9.99"));
        // The 30 times, and the table's 5 markers.
        assert_eq!(log.events.len(), 35);
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

    /// The invented flight as a terminal download, its table changed row by row by `change`, which
    /// gets each row's index and cells.
    fn coast_download(table: &str, change: impl Fn(usize, &mut Vec<String>)) -> String {
        let rows: String = table
            .lines()
            .enumerate()
            .map(|(row, line)| {
                let mut cells: Vec<String> = line.split(',').map(str::to_owned).collect();
                change(row, &mut cells);
                format!("{}\n", cells.join(","))
            })
            .collect();
        format!("Eggtimer TRS 2.07 Flight Settings\n{TRS}\n{rows}")
    }

    /// The note that opens with `prefix`, or a panic listing the notes.
    fn note_from<'a>(log: &'a FlightLog, prefix: &str) -> &'a str {
        log.notes
            .iter()
            .find(|note| note.starts_with(prefix))
            .unwrap_or_else(|| panic!("no note opens with {prefix:?}: {:#?}", log.notes))
    }

    /// The slowing, in g, a note gives after `slows at`, for `unit`, `feet` or `meters`.
    fn slowing_g(note: &str, unit: &str) -> f64 {
        let after = note.split("slows at ").nth(1).unwrap_or_default();
        let (number, rest) = after.split_once(" g in ").unwrap_or_default();
        if rest.starts_with(unit) {
            return number.parse().unwrap();
        }
        // The other unit follows: `… g in feet and … g in meters`.
        let other = rest.split(" and ").nth(1).unwrap_or_default();
        let (number, rest) = other.split_once(" g in ").unwrap_or_default();
        assert!(rest.starts_with(unit), "{note}");
        number.parse().unwrap()
    }

    /// A clock in milliseconds or in seconds, and an altitude in feet, each pass both of their
    /// checks: the coast slows at 1 g in feet on the step's clock.
    #[test]
    fn the_coast_bears_out_the_clock_and_feet() {
        for (per_second, unit) in [(1000.0, "milliseconds"), (1.0, "seconds")] {
            let text = coast_download(&flight(per_second, 0.05, G_FT, None, 15.0), |_, _| {});
            let log = read_text(&text).unwrap();
            assert_eq!(log.time_s.len(), 301);
            assert!(close(log.time_s[200], 10.0));
            let clock = note_from(&log, "`T` read in ");
            assert!(
                clock.starts_with(&format!(
                    "`T` read in {unit}, from the step and the coast: the median step between \
                     rows is "
                )),
                "{clock}"
            );
            // The window ends at the top: the first row of the highest whole foot, 9.9 s.
            assert!(
                clock.contains(
                    " g in feet over the rows from 6.9 s to 9.9 s, up to the altitude's top, \
                     where the reader takes 0.7 to 1.3 g in feet or in meters and a clock 1,000 \
                     times off would put it 10⁶ times off"
                ),
                "{clock}"
            );
            assert!((slowing_g(clock, "feet") - 1.0).abs() < 0.01, "{clock}");
            let alt = note_from(&log, "`Alt` ");
            assert!(
                alt.starts_with(
                    "`Alt` read in feet, from the manual and the coast: the Classic and TRS \
                     manuals (p37) state feet; the Quantum and Proton manuals give every height \
                     in feet and no Eggtimer manual offers meters; over the rows from 6.9 s to \
                     9.9 s, up to the altitude's top, it slows at "
                ),
                "{alt}"
            );
            assert!(alt.contains(" g in feet, within the 0.7 to 1.3 g the reader takes, where in"));
            assert!((slowing_g(alt, "feet") - 1.0).abs() < 0.01, "{alt}");
            assert!(note_from(&log, "`FAlt` ").starts_with("`FAlt` read in feet, from the "));
            assert!(note_from(&log, "`Veloc` ").starts_with("`Veloc` read in feet per second"));
            assert_eq!(log.channels[0].quantity, Quantity::AltitudeAboveGround);
            assert_eq!(log.channels[2].quantity, Quantity::AltitudeAboveGround);
            assert!(close(log.altitude_m[200], 1448.0 * FOOT_M));
            assert!(log.vertical_speed_m_s.is_some());
        }
    }

    /// A table in meters slows at 1 g in meters: the clock is borne out, as a clock 1,000 times
    /// off would put the slowing 10⁶ times off, but the manuals' feet aren't, so `Alt` and
    /// `FAlt`, and the speeds that are their change, are kept as written.
    #[test]
    fn an_altitude_in_meters_is_kept_as_written() {
        let text = coast_download(
            &flight(1000.0, 0.05, STANDARD_GRAVITY_MPS2, None, 15.0),
            |_, _| {},
        );
        let log = read_text(&text).unwrap();
        let clock = note_from(&log, "`T` ");
        assert!(clock.starts_with("`T` read in milliseconds, from the step and the coast"));
        assert!(
            clock.contains(" g in meters over the rows from "),
            "{clock}"
        );
        assert!((slowing_g(clock, "meters") - 1.0).abs() < 0.02, "{clock}");
        let alt = note_from(&log, "`Alt` ");
        assert!(
            alt.starts_with(
                "`Alt` kept as written: the manuals give feet, but over the rows from "
            ),
            "{alt}"
        );
        assert!(alt.ends_with(
            " g in meters, where the reader takes 0.7 to 1.3 g in feet; the record holds no \
             barometric altitude"
        ));
        assert!((slowing_g(alt, "feet") - FOOT_M).abs() < 0.01, "{alt}");
        assert!((slowing_g(alt, "meters") - 1.0).abs() < 0.02, "{alt}");
        let falt = note_from(&log, "`FAlt` ");
        assert!(
            falt.starts_with(
                "`FAlt` kept as written: the manuals give it in feet, as the logger's filtered \
                 `Alt` (Quantum p38, Proton p45), but over the rows from "
            ),
            "{falt}"
        );
        assert!(falt.ends_with(" g in feet, outside the 0.7 to 1.3 g the reader takes"));
        assert_eq!(
            note_from(&log, "`Veloc` "),
            "`Veloc` kept as written: it is the change in `Alt` over the step, and `Alt` is kept \
             as written"
        );
        assert_eq!(
            note_from(&log, "`FVeloc` "),
            "`FVeloc` kept as written: it is the change in `FAlt` over the step, and `FAlt` is \
             kept as written"
        );
        for channel in &log.channels[..4] {
            assert_eq!(
                channel.quantity,
                Quantity::Unrecognized,
                "{}",
                channel.column
            );
        }
        // As written: whole meters.
        assert_eq!(log.channels[0].values[200], 441.0);
        assert!(log.altitude_m.is_empty());
        assert_eq!(log.vertical_speed_m_s, None);
    }

    /// A filtered altitude in another unit than `Alt`'s is kept as written on its own check, with
    /// its speed; `Alt` is still read in feet.
    #[test]
    fn a_filtered_altitude_the_coast_doesnt_bear_out_is_kept() {
        let table = flight(1000.0, 0.05, G_FT, None, 15.0);
        let text = coast_download(&table, |row, cells| {
            let meters = |cells: &[String]| cells[1].parse::<f64>().unwrap() * FOOT_M;
            let up: Option<f64> = row
                .checked_sub(1)
                .and_then(|up| table.lines().nth(up))
                .map(|line| line.split(',').map(str::to_owned).collect::<Vec<_>>())
                .map(|cells| meters(&cells));
            let here = meters(cells);
            cells[3] = format!("{here:.2}");
            cells[4] = format!("{:.2}", up.map_or(0.0, |up| (here - up) / 0.05));
        });
        let log = read_text(&text).unwrap();
        assert!(note_from(&log, "`Alt` ").starts_with("`Alt` read in feet, from the manual and"));
        assert!(note_from(&log, "`FAlt` ").starts_with("`FAlt` kept as written: "));
        let falt = note_from(&log, "`FAlt` ");
        assert!(falt.ends_with(" g in feet, outside the 0.7 to 1.3 g the reader takes"));
        assert!((slowing_g(falt, "feet") - FOOT_M).abs() < 0.01, "{falt}");
        assert!(note_from(&log, "`FVeloc` ").starts_with("`FVeloc` kept as written: "));
        assert_eq!(log.channels[0].quantity, Quantity::AltitudeAboveGround);
        assert_eq!(log.channels[2].quantity, Quantity::Unrecognized);
        assert_eq!(log.channels[3].quantity, Quantity::Unrecognized);
        assert!(close(log.altitude_m[200], 1448.0 * FOOT_M));
    }

    /// A clock in milliseconds written 250 times a second, a step of 4, reads as seconds by its
    /// step; on that clock the coast's 3 s hold one row, and the file is refused. So is a coast
    /// that slows at g in neither feet nor meters, and a table with no coast.
    #[test]
    fn a_coast_cut_at_every_few_rows_is_refused_after_a_bounded_search() {
        // A spike on every third row of a long climb, one row a second: each cut ends the window
        // a few rows earlier, and with no bound the cuts would run back to the first row, each
        // costing a search of the rows before it, quadratic in the rows.
        let mut text = String::from("Eggtimer TRS 2.07 Flight Settings\nT,Alt,Apogee\n");
        for row in 0..20_000 {
            let spike = if row % 3 == 1 { 100_000 } else { 0 };
            text.push_str(&format!("{row},{},0\n", row + spike));
        }
        let error = read_text(&text).unwrap_err().to_string();
        assert!(
            error.contains(&format!(
                "the coast window is cut short more than {COAST_CUTS} times by jumps or motor \
                 markers before the top"
            )),
            "{error}"
        );
    }

    #[test]
    fn a_clock_the_coast_doesnt_bear_out_is_refused() {
        let text = coast_download(&flight(1000.0, 0.004, G_FT, None, 12.0), |_, _| {});
        let error = read_text(&text).unwrap_err();
        let LogError::Unit {
            format: TEXT_FORMAT,
            line: 2,
            message,
        } = &error
        else {
            panic!("{error:?}");
        };
        assert!(
            message.starts_with(
                "the clock `T` is in seconds by its step, but no coast bears that out: "
            ),
            "{message}"
        );
        // A coast slowing at 3 g in feet, 9.84 g in meters, is gravity in neither unit.
        let text = coast_download(&flight(1000.0, 0.05, 3.0 * G_FT, None, 15.0), |_, _| {});
        let error = read_text(&text).unwrap_err().to_string();
        assert!(
            error.starts_with(
                "Eggtimer text line 2: the clock `T` is in milliseconds by its step, but on that \
                 clock `Alt` slows at 3 g in feet and 9.841 g in meters over the rows from "
            ),
            "{error}"
        );
        assert!(error.ends_with(", in neither the 0.7 to 1.3 g the reader takes"));
        // A table of one row has neither a step nor a coast.
        let one = coast_download("0,0,0,0,0,0,0,0,0,0,0", |_, _| {});
        assert!(matches!(
            read_text(&one),
            Err(LogError::Unit { message, .. }) if message.starts_with(
                "the clock `T` can't be settled: the table holds one row"
            )
        ));
        // A table that stops before its top has no coast up to it.
        let short = coast_download(&flight(1000.0, 0.05, G_FT, None, 1.5), |_, _| {});
        let error = read_text(&short).unwrap_err().to_string();
        assert!(
            error.contains("is in milliseconds by its step, but no coast bears that out: "),
            "{error}"
        );
    }

    /// A charge before the top, a drogue out at 9.5 s: the coast ends at the row before its
    /// marker, and its canopy's drag is left out; without the marker, the window would run on
    /// through the canopy to the top at 9.7 s.
    #[test]
    fn a_charge_before_the_top_ends_the_coast() {
        let text = coast_download(
            &flight(1000.0, 0.05, G_FT, Some((9.5, 10.0)), 15.0),
            |_, _| {},
        );
        let log = read_text(&text).unwrap();
        let alt = note_from(&log, "`Alt` ");
        assert!(
            alt.starts_with(
                "`Alt` read in feet, from the manual and the coast: the Classic and TRS manuals"
            ),
            "{alt}"
        );
        assert!(
            alt.contains(
                "over the rows from 6.45 s to 9.45 s, up to the row before the marker `N-O` at \
                 9.5 s, it slows at 1 g in feet"
            ),
            "{alt}"
        );
    }

    /// A table in meters whose drogue is out at 80 m/s, 1.85 s into the flight, and slows the
    /// rocket to its top before 4 s: fitted up to the top, the canopy's drag makes the 3 s before
    /// it slow at 1.27 g read in feet, and the meters would pass for feet. The coast ends before
    /// the charge, where too little of the table is left, and the clock is refused.
    #[test]
    fn a_canopy_that_would_pass_meters_for_feet_is_left_out() {
        let text = coast_download(
            &flight(
                1000.0,
                0.05,
                STANDARD_GRAVITY_MPS2,
                Some((1.85, 15.0)),
                15.0,
            ),
            |_, _| {},
        );
        let error = read_text(&text).unwrap_err().to_string();
        assert_eq!(
            error,
            "Eggtimer text line 2: the clock `T` is in milliseconds by its step, but no coast \
             bears that out: the table starts 1.8 s before the coast's end, up to the row before \
             the marker `N-O` at 1.85 s, short of the 3 s window"
        );
    }

    /// A jump in the altitude, an ejection's pressure on one row, ends the coast at the row
    /// before it; one far from the window's end is no matter.
    #[test]
    fn a_jump_ends_the_coast() {
        let jumped = |at: usize| {
            coast_download(
                &flight(1000.0, 0.05, G_FT, None, 15.0),
                move |row, cells| {
                    if row == at {
                        cells[1] = "900".to_owned();
                    }
                },
            )
        };
        let log = read_text(&jumped(190)).unwrap();
        assert!(
            note_from(&log, "`Alt` ").contains(
                "over the rows from 6.45 s to 9.45 s, up to the row before `Alt` jumps at 9.5 s, \
                 it slows at 1 g in feet"
            ),
            "{}",
            note_from(&log, "`Alt` ")
        );
        // At 5 s, the jump is before the search's 4 s: the window ends at the top.
        let log = read_text(&jumped(100)).unwrap();
        assert!(
            note_from(&log, "`Alt` ").contains("from 6.9 s to 9.9 s, up to the altitude's top")
        );
        // At 8 s, the window ends 2 s before the coast's top, further than 1 s: no coast.
        let error = read_text(&jumped(160)).unwrap_err().to_string();
        assert!(
            error.contains(
                "up to the row before `Alt` jumps at 8 s, the parabola fitted to `Alt` tops out \
                 2.05 s from the window's end, further than the 1 s the reader asks"
            ),
            "{error}"
        );
    }

    #[test]
    fn markers_are_detections_motors_or_charges() {
        for name in ["LDA", "LowV", "MaxV", "MaxA"] {
            assert_eq!(marker_role(name), MarkerRole::Detection, "{name}");
        }
        for name in ["BurnT", "BO#1", "Boost#2"] {
            assert_eq!(marker_role(name), MarkerRole::Motor, "{name}");
        }
        for name in [
            "Apogee", "N-O", "Drogue", "Main", "AUX", "CHA", "CH3", "Boost#", "Sustain",
        ] {
            assert_eq!(marker_role(name), MarkerRole::Charge, "{name}");
        }
    }

    #[test]
    fn a_parabola_and_a_level_are_fitted_exactly() {
        let points: Vec<(f64, f64)> = (0..10)
            .map(|step| {
                let x = f64::from(step) * 0.25 - 2.0;
                (x, 5.0 + 3.0 * x - 16.0 * x * x)
            })
            .collect();
        let (slope, curvature) = parabola(&points).unwrap();
        assert!((slope - 3.0).abs() < 1e-9 && (curvature + 32.0).abs() < 1e-9);
        assert_eq!(parabola(&points[..2]), None);
        // A spike of one row is passed over; at the ends the median narrows to the row.
        let values = [1.0, 2.0, 300.0, 4.0, 5.0, f64::NAN, 7.0];
        assert_eq!(level(&values, 0, 6, 2), 4.0);
        assert_eq!(level(&values, 0, 6, 0), 1.0);
        assert_eq!(level(&values, 0, 6, 5), 6.0);
        assert_eq!(level(&values, 2, 3, 3), 4.0);
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
                "Eggtimer TRS 2.07 Flight Settings\n{}\n{}",
                lines.join("\n"),
                table()
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

    /// The RTF document's text: groups, control words with their delimiter, control symbols,
    /// escapes in Windows 1252, Unicode characters with their fallback, and the destinations
    /// skipped.
    #[test]
    fn rtf_unwraps_to_its_text() {
        let document = "{\\rtf1\\ansi{\\fonttbl\\f0 Helvetica;}{\\colortbl;\\red0;}\
                        {\\stylesheet{\\s0 Normal;}}{\\info{\\title Hidden}}{\\*\\generator X;}\
                        a\\\\b\\{c\\}\\'85\\'41\\tab d\\par e\\line f\\~g\\_h\\-i\\u8364?j\
                        {\\uc0\\u955 k}\\emdash\\b0\\par}";
        let text = unwrap_rtf(document).unwrap().text;
        assert_eq!(
            text,
            "a\\b{c}\u{2026}A\td\ne\nf\u{a0}g-hi\u{20ac}j\u{3bb}k\u{2014}\n"
        );
    }

    /// Each of the text's lines maps to the document's line it starts on; raw line ends aren't
    /// text, and `\` at a line's end is a paragraph.
    #[test]
    fn rtf_lines_map_to_the_documents() {
        let unwrapped = unwrap_rtf("{\\rtf1\n\\f0 a\\\r\nb\\par\n\nc}").unwrap();
        assert_eq!(unwrapped.text, "a\nb\nc");
        assert_eq!(unwrapped.starts, [2, 3, 5]);
        assert_eq!(unwrapped.file_line(3), 5);
        assert_eq!(unwrapped.file_line(9), 5);
        assert_eq!(unwrapped.file_line(0), 5);
    }

    /// Numbers of any size are read without overflow; `\bin`'s data is skipped, to the end at
    /// most; a byte outside Windows 1252, or undefined in it, is counted.
    #[test]
    fn rtf_numbers_binary_and_code_pages_are_bounded() {
        let huge = "9".repeat(400);
        let text = unwrap_rtf(&format!(
            "{{\\rtf1\\fs{huge} a\\u{huge}?\\u-3913?{{\\uc{huge}\\u65 bc}} d\\fs-{huge} e}}"
        ))
        .unwrap()
        .text;
        assert_eq!(text, "a\u{fffd}\u{f0b7}A de");
        assert_eq!(unwrap_rtf("{\\rtf1 a\\bin3 {}}b}").unwrap().text, "ab");
        let error = unwrap_rtf(&format!("{{\\rtf1 a\\bin{huge} b}}"))
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "Eggtimer RTF line 1: the document ends with 1 groups open: its braces don't balance"
        );
        let other = unwrap_rtf("{\\rtf1\\ansi\\ansicpg1251 \\'e0\\'41\\'81}").unwrap();
        assert_eq!(other.text, "\u{fffd}A\u{fffd}");
        assert_eq!(other.unmapped, 2);
        let undefined = unwrap_rtf("{\\rtf1\\ansi \\'81\\'e9\\'9f}").unwrap();
        assert_eq!(undefined.text, "\u{fffd}\u{e9}\u{178}");
        assert_eq!(undefined.unmapped, 1);
        let after = unwrap_rtf("{\\rtf1 a}\r\n x}").unwrap();
        assert_eq!((after.text.as_str(), after.after_end), ("a", 2));
    }

    /// Markup the reader refuses, each at its line: unbalanced braces, groups nested past the
    /// cap, a broken escape, text outside the document's group.
    #[test]
    fn rtf_markup_is_refused_where_it_goes_wrong() {
        let deep = "{".repeat(MAX_RTF_DEPTH + 1);
        let cases = [
            (
                "}{\\rtf1 a}",
                "line 1: a `}` closes a group that was never opened",
            ),
            (
                "{\\rtf1\n{a}\n",
                "line 3: the document ends with 1 groups open",
            ),
            (
                deep.as_str(),
                "line 1: the document's groups nest deeper than 64",
            ),
            (
                "{\\rtf1\n\\'4g}",
                "line 2: a `\\'` escape isn't followed by two hexadecimal digits",
            ),
            (
                "x{\\rtf1 a}",
                "line 1: text comes before the document's opening `{`",
            ),
            ("", "the file holds no RTF group"),
        ];
        for (document, expected) in cases {
            let error = unwrap_rtf(document).unwrap_err().to_string();
            assert!(error.contains(expected), "{document:?}: {error}");
        }
        // A deep document that stays under the cap is read.
        let nested = format!(
            "{}a{}",
            "{".repeat(MAX_RTF_DEPTH),
            "}".repeat(MAX_RTF_DEPTH)
        );
        assert_eq!(unwrap_rtf(&nested).unwrap().text, "a");
    }

    /// A file that isn't RTF, or whose text holds no download, is refused by name.
    #[test]
    fn rtf_without_a_download_is_refused_by_name() {
        for (text, expected) in [
            ("T,Alt\n0,0\n", "doesn't open with `{\\rtf`"),
            (
                "{\\rtf1 Eggtimer notes\\par T,Alt\\par 0,0}",
                "the RTF document's text holds no Eggtimer terminal download",
            ),
        ] {
            let error = read_rtf(text).unwrap_err();
            assert!(
                matches!(&error, LogError::NotThisFormat { format: "Eggtimer RTF", message }
                    if message.contains(expected)),
                "{error}"
            );
        }
    }

    proptest::proptest! {
        /// Any markup is unwrapped or refused, never a panic; the text is never longer than the
        /// document, and each of its lines has the document's line it starts on.
        #[test]
        fn any_rtf_is_unwrapped_or_refused(
            body in "[{}\\\\a-z0-9 '\\-*~_?\r\n]{0,80}"
        ) {
            let document = format!("{{\\rtf1 {body}}}");
            if let Ok(unwrapped) = unwrap_rtf(&document) {
                proptest::prop_assert!(unwrapped.text.len() <= document.len());
                let text = &unwrapped.text;
                let lines = text.matches('\n').count()
                    + usize::from(!text.is_empty() && !text.ends_with('\n'));
                proptest::prop_assert_eq!(lines, unwrapped.starts.len());
            }
            let _ = read_rtf(&document);
        }
    }
}
