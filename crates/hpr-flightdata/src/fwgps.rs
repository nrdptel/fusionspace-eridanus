//! Featherweight's GPS tracker logs, as the tracker's app and its ground station write them to
//! comma-separated files: positions from a GPS receiver, with no barometer.
//!
//! Featherweight's GPS tracker radios its fixes to a ground station, which passes them to a phone
//! app over Bluetooth; the tracker also keeps its own "flight data logging" (Featherweight,
//! *Featherweight GPS Tracker User's Manual*, February 20, 2025, p17). As of 2026-10-12, among the
//! project's sample logs, 40 distinct files hold one of four layouts, each a header line naming the
//! columns, then one row per fix:
//!
//! | Layout | Files | Header |
//! |---|---|---|
//! | The tracker's log, older | 7 | `UTCTIME,UNIXTIME,ALT,LAT,LON,#SATS,FIX,HORZV,VERTV,HEAD,FLAGS,>40,>32,>24,RSSI,BATT` |
//! | The tracker's log, newer | 21 | the same 16, then `Altitude AGL,Launch detection,Apogee detection,Landing detection,Distance (feet)` |
//! | The ground station's log | 8 | `TRACKER,DATE,TIME,LAT,LON,ALT,FIX,HORZV,VERTV,HEAD,FLAGS,#TOT,>40,>32,>24,RSSI,BATT` |
//! | The ground station's log with its radio's figures | 4 | `TRACKER,DATE,TIME,GS Lat,GS Lon,GS Alt asl,TRACKER Lat,TRACKER Lon,TRACKER Alt asl,…` (40 columns) |
//!
//! ```text
//! UTCTIME,UNIXTIME,ALT,LAT,LON,#SATS,FIX,HORZV,VERTV,HEAD,FLAGS,>40,>32,>24,RSSI,BATT
//! Jun 3 2024 15:20:00.100 UTC,1717446000.100,4000,40.1234567,-105.1234567,18,3,0,0,90,0x03,4,9,3,-40,4.010
//! ```
//!
//! A file is one of these when its first line that isn't blank opens with `UTCTIME,UNIXTIME` and
//! names `LAT` and `LON`, or opens with `TRACKER,DATE,TIME` and names `LAT` and `LON` or `TRACKER
//! Lat` and `TRACKER Lon`. Columns are then mapped by name, not by place; a name is matched as
//! written, its spaces at either end trimmed.
//!
//! **No barometer.** The files hold a GPS receiver's positions only, so the record's primary track
//! is empty, as for a Blue Raven's high-rate file ([ADR-253][adr-253] §6, on the RRC3, Eggtimer
//! and Blue Raven readers) and as [ADR-254][adr-254] §2 decided for GPS logs, and every column is
//! a channel, with a note saying so. The readings withhold on such a log.
//!
//! [adr-253]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0253-rrc3-eggtimer-and-blue-raven-csv-first.md
//! [adr-254]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md
//! [adr-258]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md
//!
//! **Channels.** Every column but the clock's parts, the tracker's name and `Found rocket ID` is a
//! channel, in the file's order, in the record's SI unit ([`Quantity::unit`]):
//!
//! | Column | Quantity | Unit from |
//! |---|---|---|
//! | `UTCTIME`; `TIME` | [`Quantity::TimeOfDay`]: UTC; the ground station's clock | the clock (below) |
//! | `UNIXTIME` | [`Quantity::Unrecognized`], as written | a note gives its offset from `UTCTIME` (below) |
//! | `ALT`, `TRACKER Alt asl`, `GS Alt asl`, `Found rocket altitude` | [`Quantity::GnssAltitude`] | feet: the manual and the data (below) |
//! | `Altitude AGL`, `GS_ASL_offset` | [`Quantity::Distance`]: a height, not above sea level | feet, with the altitude |
//! | `LAT`, `LON` and each `… Lat`, `… Lon`, `… lat`, `… lon` | [`Quantity::Latitude`], [`Quantity::Longitude`] | degrees |
//! | `HORZV`, `VERTV` | [`Quantity::Speed`] | ft/s: the manual and the data (below) |
//! | `HEAD` | [`Quantity::Angle`] | degrees: the manual and the data (below) |
//! | `BATT` | [`Quantity::Voltage`] | volts: the data (below) |
//! | `GS_batt_v`, `Found rocket batt_v` | [`Quantity::Voltage`] | the header's `v` |
//! | `Distance (feet)`, `Alt AGL (ft)`, `Horizontal range (ft)`, `Total range (ft)` | [`Quantity::Distance`] | the header's feet |
//! | `LoRa Max Range (kft)` | [`Quantity::Distance`] | the header's thousands of feet |
//! | `#SATS`, `#TOT`, `FIX`, `FLAGS`, `>40`, `>32`, `>24`, `RSSI`, `RSSI Down`, `RSSI Up`, `Found RSSI`, `Found rocket fix`, the radio's packet counts | [`Quantity::Number`], as written | no unit |
//! | `LoRa Success Rate` | [`Quantity::Number`], in percent | written `67%`, "the packet success rate, expressed as a percentage, based on the last 50 packets" (*Featherweight Swift User's Manual*, April 10 2026, p10) |
//! | `Launch detection`, `Apogee detection`, `Landing detection` | [`Quantity::Number`], 1 for `TRUE` and 0 for `FALSE` | the manual's "flag"s (p17) |
//!
//! `>40`, `>32` and `>24` are, in the manual's words, the "Number of GPS satellites with > 40
//! signal strength" and so on (p17), its color table giving signal strengths in dB-Hz (p10). `FLAGS` is written in hexadecimal (`0x03`) in the
//! older tracker log and both ground-station logs, and in decimal in the newer tracker log; either
//! is read as the number it writes. A column the reader doesn't know is kept as written,
//! [`Quantity::Unrecognized`], with a note.
//!
//! **Units no header states.** The manual lists the tracker's log as "Altitude (ASL, Feet)",
//! "Horizontal velocity (feet/second)", "Vertical velocity (feet/second)", "Heading (degrees from
//! North)" and "Tracker battery voltage" (p17), in the order of the tracker log's columns, and the
//! ground station's serial packet gives the same units (Appendix A, p21). Each is checked against
//! the file's own rows, and each file's record carries one note per kind: the decision and what
//! decided it, the same words for every file decided alike, then after the colon this file's
//! evidence, such as `` `ALT` read in feet, from the manual and the horizontal speed: … ``. The
//! checks, and what they found as of 2026-10-12, among the project's 40 sample files:
//!
//! - *The file's length unit.* One rule decides it for every length and speed the manual gives
//!   in feet: the heights, `HORZV` and `VERTV`. Only `HORZV` can be checked against an absolute
//!   length, the great-circle distance between two fixes. Taking each tracker's rows in time
//!   order, and each pair of rows more than 0.5 s and at most 2.5 s apart (from the last row whose
//!   position differed, as the ground station repeats a fix it hasn't heard anew) at a speed of at
//!   least 20 as written, the distance in feet a second over `HORZV` has a median between 0.95 and
//!   1.32 in the 37 files with 10 such pairs or more (1.32 in one ground-station log, the rest
//!   below 1.08); meters per second would give 3.28, knots 1.69, miles per hour 1.47. A median
//!   inside 0.7 to 1.4 keeps the manual's feet (`from the manual and the distance between fixes`,
//!   and for the heights `from the manual and the horizontal speed`); with fewer than 10 pairs, as
//!   in 3 files, the manual decides alone (`from the manual alone`); a median outside the band
//!   keeps every one of those columns as written, with a note. The band's upper edge sits 6% above
//!   the highest sample median and 4.5% below miles per hour's 1.467, so a file in miles per hour
//!   whose fixes ran 4.5% or more short of its speeds would be read in feet per second: the band
//!   errs toward the manual's unit, not away from it. Kilometers per hour would give 0.91, inside
//!   the band: the manual, not the check, rules them out. Pairs a tenth of a second apart are left
//!   out: in one of the older tracker logs, whose rows step back and repeat, the distance over
//!   such pairs runs 1.6 times `HORZV`, where over pairs a second apart it runs 1.02 times.
//! - *Vertical speed.* `VERTV` against the change of the altitude a second, over the same pairs,
//!   shows only that the two share a length unit, not which: a file all in meters agrees as well
//!   as one all in feet. It decides only whether `VERTV` is in the altitude's unit a second
//!   (`` `VERTV` read in feet per second, in the altitude's unit, from the change of altitude ``,
//!   or `from the manual alone` with fewer than 10 pairs); where its median falls outside 0.7 to
//!   1.4, `VERTV` and the heights are kept as written. The medians fall between 0.94 and 1.03 in
//!   the 38 files with 10 such pairs or more.
//! - *Heights.* In the 40-column ground-station log, `Alt AGL (ft)`, whose header says feet, is
//!   `TRACKER Alt asl` less `GS Alt asl` and `GS_ASL_offset` to 0.15 ft on every row with a fix of
//!   all 4 files, which ties those three to feet too: the heights there are kept as written if it
//!   holds on under half the rows, and its note says `` and `Alt AGL (ft)` `` among what decided.
//!   In the newer tracker log, `Altitude AGL` is `ALT` less one height, the pad's, to 0.05 ft (in
//!   all 21 files, on all but one row), which shows the two share a unit but not which.
//!   `Distance (feet)` is, by a median of 1.000 to 1.002 times, the great-circle distance from the
//!   first row's position in the 21 newer tracker logs.
//! - *Heading.* `HEAD` stays within −179 to 180 in every file: degrees, as the manual says. A file
//!   with a value past ±360 keeps it as written.
//! - *Battery.* `BATT` stays between 0 and 4.16 in every file, as one lithium cell's volts do
//!   (the ground station's serial packet writes millivolts, p21, which would be thousands), and
//!   reads 0 on every row of the 8 shorter ground-station logs. A value outside 0 to 5 keeps it as
//!   written.
//!
//! The speeds' and heights' unit is also what fw2kml's KML of the same flights shows (the GPS
//! logs' decision, [ADR-254][adr-254] §5), a check across files the reader can't make.
//!
//! **The clock.** The tracker's clock is `UTCTIME`, written `Jul 14 2031 17:04:31.250 UTC`: an
//! English month's first three letters, the day, the year, the time of day and `UTC`. Read as UTC,
//! it puts the first fix between 6.8 h and 15.4 h of local mean solar time at the fix's longitude
//! in the 28 tracker files. `UNIXTIME` is a Unix time, but a whole number of hours, 4 to 8, ahead
//! of `UTCTIME` on every row of every tracker file, as if the site's offset from UTC had been
//! added where it should have been taken away: read as UTC, it would put a fix at up to 22.9 h of
//! solar time. It is kept as a channel as written, with a note giving its offset. The ground
//! station's clock is `DATE` (`2031-07-14`) and `TIME` (`14:27:09.375`), in a zone the file
//! doesn't state. At the first fix, `TIME` falls between 6.6 h and 13.5 h in 11 of the 12
//! ground-station files, as a clock on the site's zone would on a launch day, and read as UTC it
//! would put 8 of them between 22.8 h and 4.0 h of local mean solar time, at night; in the
//! twelfth, `TIME` is 0.2 h, 16.2 h of solar time read as UTC, as a clock on UTC would be. The
//! times are kept on that clock as written, with a note, `` `DATE` and `TIME` kept as written, in a
//! time zone the file doesn't state: … ``, giving this file's figures after the colon.
//!
//! The log's zero is the first row's time, in the file's order. Channels keep every row in the
//! file's order, so a time may go back: in each of the 7 older tracker logs, rows step back 1 to
//! 11 times, by 0.9 s to 9 s, and in two of them once by about 847 s and 4,407 s; 1 to 76 rows of
//! each repeat a time already written. A note counts how often and how far times go back, and how
//! many rows repeat a time. The rows of the ground-station logs and of the newer tracker logs run
//! forward, but for one repeated time in 4 of the newer. A clock cell that isn't a time is
//! refused, naming its line.
//!
//! **Trackers.** A ground station logs every tracker it hears; 3 of the 8 shorter ground-station
//! logs and 1 of the 4 longer hold several (up to 5). Each tracker has its own set of channels, in
//! the order the trackers first appear, each named with the tracker's trimmed name in brackets,
//! such as `LAT [Rocket-1]`, when the file holds more than one; a file of one tracker keeps the
//! header's names. Past [`MAX_TRACKERS`] trackers, or [`MAX_CHANNELS`] channels in all, a further
//! tracker's rows are left out, counted in a note, so a file of many names can't make the reader
//! hold a set of channels for each.
//!
//! **No fix.** Before it has a fix, a tracker writes 0 for its latitude, longitude and altitude:
//! 372 rows of 6 shorter ground-station logs and 74 of a longer one do. A row whose latitude and
//! longitude are both exactly 0 has them and the altitude beside them read as gaps, counted in a
//! note, for the tracker's, the ground station's and the found rocket's positions alike, as
//! [`crate::altos`] does for AltOS's (0, 0). A latitude beyond ±90° or a longitude beyond ±180° is
//! a gap too, counted per column in a note, as [`crate::kml`] and [`crate::nmea`] read theirs; no
//! sample file has one.
//!
//! **Cells and rows.** A cell is a decimal number, hexadecimal digits after `0x`, `TRUE` or
//! `FALSE` (1 or 0), or a number followed by `%`. A cell that is empty or none of these, such as
//! the literal `NaN` in `Distance (feet)` (27 cells over the 21 newer tracker logs), is a gap,
//! counted per column in a note. A row with fewer cells than the header has the rest as gaps; a
//! row with more is refused, as is a value its unit's factor makes too large for an `f64`, such
//! as `LoRa Max Range (kft)` near 10³⁰⁸, naming its line ([the rule for infinite values][adr-258],
//! §3). A file whose rows, down to
//! any row, hold under a quarter of the cells its header names (past the first 65,536) is refused
//! at that row. Blank lines are skipped; lines may end in CR, LF or both (2 of the shorter
//! ground-station logs end in CR LF); a byte-order mark is dropped.
//!
//! **What the file says about itself.** The logger is a `Featherweight GPS tracker` or a
//! `Featherweight GPS ground station`; the files state no serial number, firmware, flight number,
//! apogee or ground elevation.
//!
//! **Where this comes from.** The layouts and their quirks, from the sample files; the units,
//! from the manual as cited and the files; the tracker log's clock, columns and feet, following
//! Debrief's `lib/parsers/featherweightGps.ts` (MIT, the project owner's own; see
//! `THIRD-PARTY-NOTICES.md`). Where this reader departs from Debrief's: it keeps every row in the
//! file's order on `UTCTIME`, where Debrief sorts by `UNIXTIME` and drops repeated times; it reads
//! the shorter ground-station log, which Debrief doesn't, and keeps each tracker apart, where
//! Debrief reads every row as one tracker's; it keeps every column, where Debrief keeps a chosen
//! few; and it reads a position's no-fix zeros as gaps by the zeros themselves, where Debrief reads
//! them by the `FIX` column.

use std::collections::HashMap;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated, too_sparse};
use crate::perfectflite::FOOT_M;
use crate::table::{Notes, finite, number, numbered_lines, short};

/// The format's name in errors.
const FORMAT: &str = "Featherweight GPS CSV";

/// The most trackers one file's channels are kept for; past it, a tracker's rows are left out.
/// The sample files hold at most 5.
pub const MAX_TRACKERS: usize = 64;

/// The most channels a file's trackers may have in all; past it, a further tracker's rows are left
/// out. Two trackers of the 40-column log hold 74.
pub const MAX_CHANNELS: usize = 4_096;

/// The shortest step between two fixes a speed is checked over, s: over a tenth of a second, the
/// older tracker log's rows, which step back and repeat, can pair fixes that weren't a tenth of a
/// second apart.
const MIN_STEP_S: f64 = 0.5;

/// The longest step between two fixes a speed is checked over, s: the ground station logs about a
/// fix a second, the tracker up to ten.
const MAX_STEP_S: f64 = 2.5;

/// The slowest speed, as written, a speed is checked at: below it a GPS receiver's noise is a
/// large part of the change.
const MIN_SPEED: f64 = 20.0;

/// The fewest pairs of rows a speed check needs to overrule the manual.
const MIN_PAIRS: usize = 10;

/// The band a speed check's median ratio must fall in to agree with the manual: wide enough for
/// the 0.94 to 1.32 seen in the samples, narrow enough to refuse miles per hour (1.47), knots
/// (1.69) and meters per second (3.28). The upper edge is 4.5% below miles per hour's 1.467, so a
/// file in miles per hour whose fixes run that much short of its speeds would read in feet per
/// second: the band errs toward the manual's unit (the module's docs say more).
const AGREES: (f64, f64) = (0.7, 1.4);

/// The most a battery reads in volts: one lithium cell is full at about 4.2 V.
const MAX_CELL_V: f64 = 5.0;

/// The mean radius of the Earth, m: R₁ = 6,371,008.8 m of the Geodetic Reference System 1980
/// (Moritz, *Geodetic Reference System 1980*, Journal of Geodesy 74, 2000). A great circle on this
/// sphere is within half a percent of the ellipsoid's distance anywhere.
const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Nanoseconds in a second and in a day.
const NS_PER_S: i64 = 1_000_000_000;
const NS_PER_DAY: i64 = 86_400 * NS_PER_S;

/// The years a clock cell may name: a GPS receiver's dates fall well inside, and the span keeps a
/// difference of two times, in nanoseconds, inside an `i64`.
const YEARS: std::ops::RangeInclusive<i64> = 1970..=2199;

/// Whether the text is in this reader's layout, judged from its header, never the file's name.
pub(crate) fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .map(|(_, line)| line)
        .find(|line| !line.trim().is_empty())
        .and_then(layout)
        .is_some()
}

/// Which of Featherweight's logs a file is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// The tracker's own log, timed by `UTCTIME`.
    Tracker,
    /// The ground station's log, timed by `DATE` and `TIME`, one tracker's rows or several.
    GroundStation,
}

/// A header line's layout, `None` if it is neither.
///
/// The opening names are checked first, one at a time, and the rest are walked without being
/// collected, so that a long line of other text is turned away without a list of its names.
fn layout(header: &str) -> Option<Layout> {
    let mut names = header.split(',').map(str::trim);
    let layout = match (names.next()?, names.next()?) {
        ("UTCTIME", "UNIXTIME") => Layout::Tracker,
        ("TRACKER", "DATE") if names.next()? == "TIME" => Layout::GroundStation,
        _ => return None,
    };
    let (mut lat, mut lon, mut tracker_lat, mut tracker_lon) = (false, false, false, false);
    for name in names {
        match name {
            "LAT" => lat = true,
            "LON" => lon = true,
            "TRACKER Lat" => tracker_lat = true,
            "TRACKER Lon" => tracker_lon = true,
            _ => {}
        }
    }
    let position = (lat && lon) || (layout == Layout::GroundStation && tracker_lat && tracker_lon);
    position.then_some(layout)
}

/// What a column is to this reader.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    /// `UTCTIME`: the tracker's clock, and a time-of-day channel.
    Clock,
    /// `DATE`: the ground station's clock's date.
    Date,
    /// `TIME`: the ground station's clock's time of day, and a time-of-day channel.
    Time,
    /// `TRACKER`: whose row it is.
    Tracker,
    /// Text that names, such as `Found rocket ID`: no channel.
    Label,
    /// A value of some kind.
    Value(Kind),
}

/// The kinds of value column, each with how its unit is decided.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    /// A height in feet by the manual, checked against the file: above sea level, or not.
    Height { above_sea_level: bool },
    /// `VERTV`.
    VerticalSpeed,
    /// `HORZV`.
    HorizontalSpeed,
    /// `HEAD`.
    Heading,
    /// `BATT`.
    Battery,
    /// `UNIXTIME`.
    UnixTime,
    /// A quantity whose unit the header states, or that has none, made SI by a factor.
    Stated(Quantity, f64),
    /// A name the reader doesn't know.
    Unknown,
}

/// What a column is, by its trimmed name.
fn role(name: &str) -> Role {
    use Quantity as Q;
    let value = |kind| Role::Value(kind);
    let stated = |quantity, factor| Role::Value(Kind::Stated(quantity, factor));
    match name {
        "UTCTIME" => Role::Clock,
        "DATE" => Role::Date,
        "TIME" => Role::Time,
        "TRACKER" => Role::Tracker,
        "Found rocket ID" => Role::Label,
        "UNIXTIME" => value(Kind::UnixTime),
        "ALT" | "TRACKER Alt asl" | "GS Alt asl" | "Found rocket altitude" => value(Kind::Height {
            above_sea_level: true,
        }),
        "Altitude AGL" | "GS_ASL_offset" => value(Kind::Height {
            above_sea_level: false,
        }),
        "LAT" | "TRACKER Lat" | "GS Lat" | "Found rocket lat" => stated(Q::Latitude, 1.0),
        "LON" | "TRACKER Lon" | "GS Lon" | "Found rocket lon" => stated(Q::Longitude, 1.0),
        "HORZV" => value(Kind::HorizontalSpeed),
        "VERTV" => value(Kind::VerticalSpeed),
        "HEAD" => value(Kind::Heading),
        "BATT" => value(Kind::Battery),
        "GS_batt_v" | "Found rocket batt_v" => stated(Q::Voltage, 1.0),
        "Distance (feet)" | "Alt AGL (ft)" | "Horizontal range (ft)" | "Total range (ft)" => {
            stated(Q::Distance, FOOT_M)
        }
        "LoRa Max Range (kft)" => stated(Q::Distance, 1_000.0 * FOOT_M),
        "#SATS" | "#TOT" | "FIX" | "FLAGS" | ">40" | ">32" | ">24" | "RSSI" | "RSSI Down"
        | "RSSI Up" | "Found RSSI" | "Found rocket fix" | "Down Sent" | "Down Rcvd"
        | "Found Sent" | "Up Sent" | "Up Rccvd" | "LoRa Success Rate" | "Launch detection"
        | "Apogee detection" | "Landing detection" => stated(Q::Number, 1.0),
        _ => value(Kind::Unknown),
    }
}

/// One column of the header.
struct Column {
    name: String,
    role: Role,
    /// Cells that were empty or not a value, and the first of them as written.
    gaps: usize,
    gap_example: Option<String>,
    /// Latitudes beyond ±90° or longitudes beyond ±180°, read as gaps.
    out_of_range: usize,
}

/// One tracker's rows: their times and each column's values as written, the clock's columns
/// holding the time of day.
struct Set {
    name: String,
    times_ns: Vec<i64>,
    /// Each row's line in the file.
    lines: Vec<usize>,
    values: Vec<Vec<f64>>,
}

/// A position's three columns, the altitude's if the file has it.
#[derive(Debug, Clone, Copy)]
struct Position {
    lat: usize,
    lon: usize,
    alt: Option<usize>,
    /// Rows read with the latitude and longitude both 0.
    zeros: usize,
}

/// Reads a Featherweight GPS tracker's or ground station's CSV.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't one of the four headers;
/// [`LogError::Syntax`] for a header column with no name, a row with more cells than the header,
/// a row whose clock is missing or isn't a time, rows holding under a quarter of the cells the
/// header names, or a value its unit's factor makes too large for an `f64`; [`LogError::NoData`]
/// if no row follows the header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    let Some(layout) = layout(header) else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't a Featherweight GPS header, which opens \
                 `UTCTIME,UNIXTIME` or `TRACKER,DATE,TIME`",
                short(header)
            ),
        });
    };
    let mut notes = Notes::default();
    let mut columns = Vec::new();
    for (index, name) in header.split(',').map(str::trim).enumerate() {
        if name.is_empty() {
            return Err(syntax(
                header_line,
                format!("the header's column {} has no name", index + 1),
            ));
        }
        let role = role(name);
        if role == Role::Value(Kind::Unknown) {
            notes.push(format!(
                "the column {:?} isn't one this reader knows, and was kept as written",
                short(name)
            ));
        }
        columns.push(Column {
            name: name.to_owned(),
            role,
            gaps: 0,
            gap_example: None,
            out_of_range: 0,
        });
    }
    let find = |role: Role| columns.iter().position(|c| c.role == role);
    // The layout's header check names these columns' places, so each is found.
    let clock = match layout {
        Layout::Tracker => Clock::Utc(find(Role::Clock).unwrap_or(0)),
        Layout::GroundStation => Clock::DateTime {
            date: find(Role::Date).unwrap_or(1),
            time: find(Role::Time).unwrap_or(2),
            tracker: find(Role::Tracker).unwrap_or(0),
        },
    };
    let mut positions = positions(&columns);
    let channels_per_set = columns
        .iter()
        .filter(|c| matches!(c.role, Role::Clock | Role::Time | Role::Value(_)))
        .count();

    let mut sets: Vec<Set> = Vec::new();
    let mut set_of: HashMap<String, usize> = HashMap::new();
    let mut left_out = LeftOut::default();
    let (mut rows, mut cells_written) = (0usize, 0usize);
    let (mut short_rows, mut first_short_row) = (0usize, 0usize);
    let mut first_row: Option<(String, i64)> = None;
    for (line_number, line) in lines {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() > columns.len() {
            return Err(syntax(
                line_number,
                format!(
                    "the row has {} cells, more than the header's {}",
                    cells.len(),
                    columns.len()
                ),
            ));
        }
        rows += 1;
        cells_written += cells.len();
        // Checked row by row, so that a wide header over short rows is refused before the gaps
        // it would leave grow past a quarter more than the cells the file holds.
        if too_sparse(columns.len(), rows, cells_written) {
            return Err(syntax(
                line_number,
                format!(
                    "the header names {} columns, and the {rows} rows down to this one hold only \
                     {cells_written} cells",
                    columns.len()
                ),
            ));
        }
        if cells.len() < columns.len() {
            if short_rows == 0 {
                first_short_row = line_number;
            }
            short_rows += 1;
        }
        let instant = clock.read(&cells, &columns, line_number)?;
        if first_row.is_none() {
            first_row = Some((clock.as_written(&cells), instant));
        }
        let tracker = clock.tracker(&cells);
        let index = match set_of.get(tracker) {
            Some(&index) => index,
            None if sets.is_empty()
                || (sets.len() < MAX_TRACKERS
                    && (sets.len() + 1).saturating_mul(channels_per_set) <= MAX_CHANNELS) =>
            {
                set_of.insert(tracker.to_owned(), sets.len());
                sets.push(Set {
                    name: tracker.to_owned(),
                    times_ns: Vec::new(),
                    lines: Vec::new(),
                    values: vec![Vec::new(); columns.len()],
                });
                sets.len() - 1
            }
            None => {
                left_out.row(tracker);
                continue;
            }
        };
        let set = &mut sets[index];
        set.times_ns.push(instant);
        set.lines.push(line_number);
        read_cells(set, &mut columns, &cells, instant);
        for position in &mut positions {
            no_fix(set, position);
        }
    }
    let Some(first_row) = first_row else {
        return Err(LogError::NoData { format: FORMAT });
    };
    let mut file_notes = Vec::new();
    if short_rows > 0 {
        file_notes.push(format!(
            "{short_rows} rows, the first on line {first_short_row}, have fewer cells than the \
             header's {}: the cells they leave out are gaps",
            columns.len()
        ));
    }
    if let Some(note) = left_out.note() {
        file_notes.push(note);
    }
    build(
        layout, &columns, sets, &positions, &first_row, notes, file_notes,
    )
}

/// Where a layout's clock is.
#[derive(Debug, Clone, Copy)]
enum Clock {
    /// `UTCTIME`'s place.
    Utc(usize),
    /// `DATE`'s, `TIME`'s and `TRACKER`'s places.
    DateTime {
        date: usize,
        time: usize,
        tracker: usize,
    },
}

impl Clock {
    /// A row's time, ns from 1970-01-01 00:00 on the file's clock; a missing cell or one that
    /// isn't a time is refused.
    fn read(self, cells: &[&str], columns: &[Column], line: usize) -> Result<i64, LogError> {
        let cell = |index: usize| {
            cells.get(index).copied().ok_or_else(|| {
                syntax(
                    line,
                    format!(
                        "the row ends after {} cells, before its {:?}",
                        cells.len(),
                        short(&columns[index].name)
                    ),
                )
            })
        };
        let refuse = |what: &str, cell: &str, index: usize| {
            syntax(
                line,
                format!(
                    "the {what} {:?} in cell {} isn't one",
                    short(cell),
                    index + 1
                ),
            )
        };
        match self {
            Self::Utc(index) => {
                let text = cell(index)?;
                utc_time(text).ok_or_else(|| refuse("time", text, index))
            }
            Self::DateTime { date, time, .. } => {
                let (date_text, time_text) = (cell(date)?, cell(time)?);
                let day = civil_date(date_text).ok_or_else(|| refuse("date", date_text, date))?;
                let of_day =
                    time_of_day_ns(time_text).ok_or_else(|| refuse("time", time_text, time))?;
                Ok(day * NS_PER_DAY + of_day)
            }
        }
    }

    /// The row's tracker: the trimmed `TRACKER`, or none for the tracker's own log.
    fn tracker<'a>(self, cells: &[&'a str]) -> &'a str {
        match self {
            Self::Utc(_) => "",
            Self::DateTime { tracker, .. } => cells.get(tracker).copied().unwrap_or_default(),
        }
    }

    /// The clock's cells as the file writes them, cut short.
    fn as_written(self, cells: &[&str]) -> String {
        match self {
            Self::Utc(index) => short(cells.get(index).copied().unwrap_or_default()),
            Self::DateTime { date, time, .. } => short(&format!(
                "{} {}",
                cells.get(date).copied().unwrap_or_default(),
                cells.get(time).copied().unwrap_or_default()
            )),
        }
    }
}

/// Rows of trackers past the caps, counted.
#[derive(Default)]
struct LeftOut {
    names: Vec<String>,
    rows: usize,
}

impl LeftOut {
    fn row(&mut self, tracker: &str) {
        self.rows += 1;
        // Counting names stops at the cap's size, so a file of many can't fill memory with them.
        if self.names.len() < MAX_TRACKERS && !self.names.iter().any(|name| name == tracker) {
            self.names.push(tracker.to_owned());
        }
    }

    fn note(&self) -> Option<String> {
        (self.rows > 0).then(|| {
            let trackers = if self.names.len() < MAX_TRACKERS {
                format!("{} more trackers", self.names.len())
            } else {
                format!("{MAX_TRACKERS} or more further trackers")
            };
            format!(
                "{} rows of {trackers} were left out, past the first {MAX_TRACKERS} trackers or \
                 {MAX_CHANNELS} channels, so that a file of many tracker names can't make the \
                 reader hold a set of channels for each",
                self.rows
            )
        })
    }
}

/// Reads a row's cells into its tracker's columns as written: the clock's columns as the time of
/// day, s, a value column as its cell's value or a gap.
fn read_cells(set: &mut Set, columns: &mut [Column], cells: &[&str], instant: i64) {
    for (index, column) in columns.iter_mut().enumerate() {
        let value = match column.role {
            Role::Date | Role::Tracker | Role::Label => continue,
            Role::Clock => seconds_of_day(instant),
            Role::Time => cells
                .get(index)
                .and_then(|cell| time_of_day_ns(cell))
                .map_or(f64::NAN, |ns| ns as f64 / NS_PER_S as f64),
            Role::Value(kind) => match cells.get(index) {
                None => f64::NAN,
                Some(cell) => match value(cell) {
                    Some(value) if value.abs() > most_degrees(kind) => {
                        column.out_of_range += 1;
                        f64::NAN
                    }
                    Some(value) => value,
                    None => {
                        column.gaps += 1;
                        if column.gap_example.is_none() {
                            column.gap_example = Some(short(cell));
                        }
                        f64::NAN
                    }
                },
            },
        };
        set.values[index].push(value);
    }
}

/// The largest a value of a kind may be either side of 0: 90° for a latitude, 180° for a
/// longitude, as kml.rs and nmea.rs read them; no bound for the rest.
fn most_degrees(kind: Kind) -> f64 {
    match kind {
        Kind::Stated(Quantity::Latitude, _) => 90.0,
        Kind::Stated(Quantity::Longitude, _) => 180.0,
        _ => f64::INFINITY,
    }
}

/// A cell's value as written: a decimal number, hexadecimal digits after `0x`, `TRUE` or `FALSE`
/// as 1 or 0, or a number followed by `%`; `None` for anything else, `NaN` included.
fn value(cell: &str) -> Option<f64> {
    if let Some(value) = number(cell, false) {
        return Some(value);
    }
    match cell {
        "TRUE" => return Some(1.0),
        "FALSE" => return Some(0.0),
        _ => {}
    }
    if let Some(digits) = cell.strip_prefix("0x").or_else(|| cell.strip_prefix("0X")) {
        if digits.is_empty() || digits.len() > 13 || !digits.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return None;
        }
        // At most 13 hexadecimal digits, 52 bits, so the value is below 2⁵² and exact as an f64.
        return u64::from_str_radix(digits, 16).ok().map(|bits| bits as f64);
    }
    cell.strip_suffix('%')
        .filter(|rest| !rest.ends_with(char::is_whitespace))
        .and_then(|rest| number(rest, false))
}

/// The positions the header holds: the tracker's, and in the longer ground-station log the ground
/// station's and the found rocket's, each its latitude, longitude and altitude.
fn positions(columns: &[Column]) -> Vec<Position> {
    let place = |name: &str| columns.iter().position(|c| c.name == name);
    [
        ("LAT", "LON", "ALT"),
        ("TRACKER Lat", "TRACKER Lon", "TRACKER Alt asl"),
        ("GS Lat", "GS Lon", "GS Alt asl"),
        (
            "Found rocket lat",
            "Found rocket lon",
            "Found rocket altitude",
        ),
    ]
    .into_iter()
    .filter_map(|(lat, lon, alt)| {
        Some(Position {
            lat: place(lat)?,
            lon: place(lon)?,
            alt: place(alt),
            zeros: 0,
        })
    })
    .collect()
}

/// A no-fix row's position, its latitude and longitude both 0, read as gaps, and counted.
fn no_fix(set: &mut Set, position: &mut Position) {
    let last = |column: usize| set.values[column].last().copied();
    if last(position.lat) == Some(0.0) && last(position.lon) == Some(0.0) {
        position.zeros += 1;
        for column in [Some(position.lat), Some(position.lon), position.alt]
            .into_iter()
            .flatten()
        {
            if let Some(value) = set.values[column].last_mut() {
                *value = f64::NAN;
            }
        }
    }
}

/// A tracker log's `UTCTIME`, such as `Jul 14 2031 17:04:31.250 UTC`, as ns from 1970-01-01 00:00
/// UTC; `None` for anything else.
fn utc_time(cell: &str) -> Option<i64> {
    let mut parts = cell.split_whitespace();
    let (month, day, year, time, zone) = (
        parts.next()?,
        parts.next()?,
        parts.next()?,
        parts.next()?,
        parts.next()?,
    );
    if parts.next().is_some() || zone != "UTC" {
        return None;
    }
    const MONTHS: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let month = MONTHS
        .iter()
        .position(|name| name.eq_ignore_ascii_case(month))?;
    let day = days(year, u32::try_from(month).ok()? + 1, day)?;
    Some(day * NS_PER_DAY + time_of_day_ns(time)?)
}

/// A ground station's `DATE`, `YYYY-MM-DD`, as days from 1970-01-01; `None` for anything else.
fn civil_date(cell: &str) -> Option<i64> {
    let mut parts = cell.split('-');
    let (year, month, day) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() || year.len() != 4 || !digits(month, 1..=2) {
        return None;
    }
    days(year, month.parse().ok()?, day)
}

/// Whether text is all ASCII digits, as many as the range allows.
fn digits(text: &str, count: std::ops::RangeInclusive<usize>) -> bool {
    count.contains(&text.len()) && text.bytes().all(|b| b.is_ascii_digit())
}

/// A date's days from 1970-01-01, the year four digits within [`YEARS`] and the day one or two
/// digits within the month (leap years by the Gregorian rule).
fn days(year: &str, month: u32, day: &str) -> Option<i64> {
    if !digits(year, 4..=4) || !digits(day, 1..=2) {
        return None;
    }
    let year: i64 = year.parse().ok()?;
    let day: u32 = day.parse().ok()?;
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let length = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    (YEARS.contains(&year) && (1..=length).contains(&day))
        .then(|| days_from_civil(year, month, day))
}

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar, by Howard Hinnant's
/// `days_from_civil` ("chrono-Compatible Low-Level Date Algorithms", 2013, public domain): the
/// year counted from March, so that February's leap day falls at its end.
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * i64::from((month + 9) % 12) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// A time of day, `H:MM:SS` or `HH:MM:SS` with up to nine decimals, as ns after midnight; `None`
/// for anything else, or a time past 23:59:60.999 999 999 (a leap second's).
fn time_of_day_ns(cell: &str) -> Option<i64> {
    let mut parts = cell.split(':');
    let (hours, minutes, seconds) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let (whole, fraction) = seconds.split_once('.').unwrap_or((seconds, ""));
    if !digits(hours, 1..=2)
        || !digits(minutes, 2..=2)
        || !digits(whole, 2..=2)
        || !(fraction.is_empty() || digits(fraction, 1..=9))
        || (seconds.contains('.') && fraction.is_empty())
    {
        return None;
    }
    let (hours, minutes, whole): (i64, i64, i64) = (
        hours.parse().ok()?,
        minutes.parse().ok()?,
        whole.parse().ok()?,
    );
    if hours >= 24 || minutes >= 60 || whole > 60 {
        return None;
    }
    let mut nanos: i64 = 0;
    for (place, digit) in fraction.bytes().enumerate() {
        // At most nine digits, each place a tenth of the one before, from 10⁸ ns.
        nanos += i64::from(digit - b'0') * 10_i64.pow(8 - place as u32);
    }
    Some(((hours * 60 + minutes) * 60 + whole) * NS_PER_S + nanos)
}

/// A time's seconds after its midnight.
fn seconds_of_day(instant: i64) -> f64 {
    instant.rem_euclid(NS_PER_DAY) as f64 / NS_PER_S as f64
}

/// Seconds from one time to another, both ns on one clock.
fn seconds_between(from: i64, to: i64) -> f64 {
    // Both times lie within YEARS, so their difference fits an i64 of nanoseconds.
    (to - from) as f64 / NS_PER_S as f64
}

/// The record: every tracker's channels, the decided units, and the notes.
fn build(
    layout: Layout,
    columns: &[Column],
    sets: Vec<Set>,
    positions: &[Position],
    first_row: &(String, i64),
    notes: Notes,
    file_notes: Vec<String>,
) -> Result<FlightLog, LogError> {
    let decisions = Decisions::from_file(columns, &sets);
    let mut log = FlightLog {
        format: LogFormat::FeatherweightGpsCsv,
        logger: match layout {
            Layout::Tracker => "Featherweight GPS tracker",
            Layout::GroundStation => "Featherweight GPS ground station",
        }
        .to_owned(),
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
        notes: decisions.notes.clone(),
    };
    log.notes.push(
        "the log has no barometer, so the record's altitude track is empty: a GPS tracker's file \
         holds positions only, and every column is a channel"
            .to_owned(),
    );
    log.notes
        .push(clock_note(layout, &sets, positions, first_row));
    if let Some(note) = steps_back_note(&sets) {
        log.notes.push(note);
    }
    if layout == Layout::GroundStation {
        log.notes.push(trackers_note(&sets));
    }
    for position in positions.iter().filter(|p| p.zeros > 0) {
        let names: Vec<String> = [Some(position.lat), Some(position.lon), position.alt]
            .into_iter()
            .flatten()
            .map(|index| format!("`{}`", columns[index].name))
            .collect();
        log.notes.push(format!(
            "{} rows have `{}` and `{}` both 0, as a receiver writes before it has a fix: their \
             {} are read as gaps",
            position.zeros,
            columns[position.lat].name,
            columns[position.lon].name,
            list(&names)
        ));
    }
    log.notes.extend(file_notes);
    let mut notes = notes;
    for column in columns.iter().filter(|c| c.gaps > 0) {
        notes.push(format!(
            "the column {:?} holds {} cells that are empty or not a number, such as {:?}: they \
             are gaps",
            short(&column.name),
            column.gaps,
            column.gap_example.as_deref().unwrap_or_default()
        ));
    }
    for column in columns.iter().filter(|c| c.out_of_range > 0) {
        let (what, most) = match column.role {
            Role::Value(Kind::Stated(Quantity::Latitude, _)) => ("latitudes", "±90°"),
            _ => ("longitudes", "±180°"),
        };
        notes.push(format!(
            "the column {:?} holds {} {what} beyond {most}: they are gaps",
            short(&column.name),
            column.out_of_range
        ));
    }
    log.notes.extend(notes.into_vec());

    let zero = first_row.1;
    let qualify = sets.len() > 1;
    for set in sets {
        let time_s: Vec<f64> = set
            .times_ns
            .iter()
            .map(|&instant| seconds_between(zero, instant))
            .collect();
        for (column, values) in columns.iter().zip(set.values) {
            let Some(quantity) = decisions.quantity(column.role) else {
                continue;
            };
            let factor = decisions.factor(column.role);
            let name = if qualify {
                format!("{} [{}]", short(&column.name), short(&set.name))
            } else {
                column.name.clone()
            };
            // A value that the factor makes infinite, such as thousands of feet near the largest
            // `f64`, is refused on its row's line (ADR-258 §3).
            let mut si = Vec::with_capacity(values.len());
            for (line, value) in set.lines.iter().zip(values) {
                si.push(finite(value * factor, FORMAT, *line, &column.name)?);
            }
            log.channels.push(Channel {
                column: name,
                quantity,
                time_s: time_s.clone(),
                values: si,
            });
        }
    }
    Ok(log)
}

/// The note on the log's clock: its zero, and for the ground station the zone it doesn't state.
fn clock_note(
    layout: Layout,
    sets: &[Set],
    positions: &[Position],
    first_row: &(String, i64),
) -> String {
    match layout {
        Layout::Tracker => format!(
            "the log's clock is `UTCTIME`, read as UTC, and its zero is the first row's, {:?}",
            first_row.0
        ),
        Layout::GroundStation => {
            let solar = first_fix(sets, positions).map_or_else(
                || "the file has no fix to compare its `TIME` with the sun's".to_owned(),
                |(instant, longitude)| {
                    format!(
                        "at the first fix, `TIME` is {}, which read as UTC would be {:.1} h of \
                         local mean solar time at the fix's longitude",
                        clock_text(instant),
                        solar_hours(instant, longitude)
                    )
                },
            );
            format!(
                "`DATE` and `TIME` kept as written, in a time zone the file doesn't state: they \
                 are the log's clock, and its zero is the first row's, {:?}; {solar}",
                first_row.0
            )
        }
    }
}

/// The first row, in the file's order, whose tracker position has a fix: its time and longitude.
fn first_fix(sets: &[Set], positions: &[Position]) -> Option<(i64, f64)> {
    let position = positions.first()?;
    sets.iter()
        .filter_map(|set| {
            (0..set.times_ns.len())
                .find(|&row| {
                    set.values[position.lat][row].is_finite()
                        && set.values[position.lon][row].is_finite()
                })
                .map(|row| (set.times_ns[row], set.values[position.lon][row]))
        })
        .min_by_key(|(instant, _)| *instant)
}

/// A time's time of day, `HH:MM:SS.fff`, to the millisecond below it.
fn clock_text(instant: i64) -> String {
    let millis = instant.rem_euclid(NS_PER_DAY) / 1_000_000;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        millis / 3_600_000,
        millis / 60_000 % 60,
        millis / 1_000 % 60,
        millis % 1_000
    )
}

/// The local mean solar time at a longitude, h, of a time read as UTC: the time of day plus the
/// longitude at 15° an hour, the Earth's mean turn, without the equation of time's quarter hour.
fn solar_hours(instant: i64, longitude_deg: f64) -> f64 {
    (seconds_of_day(instant) / 3_600.0 + longitude_deg / 15.0).rem_euclid(24.0)
}

/// The note on times that go back or repeat within a tracker's rows, if any do.
fn steps_back_note(sets: &[Set]) -> Option<String> {
    let (mut backs, mut largest_ns, mut repeats) = (0usize, 0i64, 0usize);
    for set in sets {
        for pair in set.times_ns.windows(2) {
            if pair[1] < pair[0] {
                backs += 1;
                largest_ns = largest_ns.max(pair[0] - pair[1]);
            }
        }
        let mut sorted = set.times_ns.clone();
        sorted.sort_unstable();
        repeats += sorted.windows(2).filter(|pair| pair[0] == pair[1]).count();
    }
    let kept = "every row is kept, in the file's order";
    if backs > 0 {
        Some(format!(
            "times go back {backs} times from one row to the next, by up to {:.3} s, and {repeats} \
             rows repeat a time already written: {kept}",
            largest_ns as f64 / NS_PER_S as f64
        ))
    } else {
        (repeats > 0).then(|| format!("{repeats} rows repeat a time already written: {kept}"))
    }
}

/// The note naming a ground station's trackers.
fn trackers_note(sets: &[Set]) -> String {
    match sets {
        [one] => format!(
            "the file holds one tracker's rows, {:?}, so its channels keep the header's names",
            short(&one.name)
        ),
        _ => {
            let names: Vec<String> = sets
                .iter()
                .map(|set| format!("{:?} ({} rows)", short(&set.name), set.times_ns.len()))
                .collect();
            format!(
                "the file holds {} trackers' rows, each its own set of channels named with the \
                 tracker in brackets, in the order they first appear: {}",
                sets.len(),
                list(&names)
            )
        }
    }
}

/// The most names a note lists; past it, the rest are counted.
const MAX_LISTED: usize = 6;

/// Names joined as `a`, `a and b`, or `a, b and c`; past [`MAX_LISTED`], the first few and a
/// count of the rest.
fn list(names: &[String]) -> String {
    if names.len() > MAX_LISTED {
        let shown = MAX_LISTED - 1;
        return format!(
            "{} and {} more",
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

/// What a speed check found.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Check {
    /// The median ratio and the pairs it is over, in the band.
    Agrees(f64, usize),
    /// The same, outside the band.
    Disagrees(f64, usize),
    /// Fewer than [`MIN_PAIRS`] pairs, or no column to check.
    TooFew(usize),
}

impl Check {
    fn from_ratios(mut ratios: Vec<f64>) -> Self {
        if ratios.len() < MIN_PAIRS {
            return Self::TooFew(ratios.len());
        }
        ratios.sort_by(f64::total_cmp);
        let median = ratios[ratios.len() / 2];
        if (AGREES.0..=AGREES.1).contains(&median) {
            Self::Agrees(median, ratios.len())
        } else {
            Self::Disagrees(median, ratios.len())
        }
    }

    /// What the check found, such as `the change of `ALT` a second is a median of 1.012 times
    /// `VERTV` over 432 pairs of rows`, the change named by `change`.
    fn words(self, change: &str, speed: &str) -> String {
        match self {
            Self::Agrees(median, pairs) | Self::Disagrees(median, pairs) => format!(
                "{change} is a median of {median:.3} times `{speed}` over {pairs} pairs of rows"
            ),
            Self::TooFew(pairs) => format!(
                "the file has too few pairs of moving rows to check `{speed}` over, {pairs} of the \
                 {MIN_PAIRS} needed"
            ),
        }
    }
}

/// What a speed is checked against.
#[derive(Debug, Clone, Copy)]
enum Track {
    /// An altitude column's change, as written.
    Height(usize),
    /// A position's great-circle distance, ft.
    Position(usize, usize),
}

impl Track {
    /// The row's point, `None` if it has a gap.
    fn at(self, set: &Set, row: usize) -> Option<(f64, f64)> {
        let (a, b) = match self {
            Self::Height(column) => (set.values[column][row], 0.0),
            Self::Position(lat, lon) => (set.values[lat][row], set.values[lon][row]),
        };
        (a.is_finite() && b.is_finite()).then_some((a, b))
    }

    /// The change from one point to another: up, as written, or a distance, ft.
    fn change(self, from: (f64, f64), to: (f64, f64)) -> f64 {
        match self {
            Self::Height(_) => to.0 - from.0,
            Self::Position(..) => great_circle_m(from, to) / FOOT_M,
        }
    }
}

/// The great-circle distance between two positions in degrees, m, on a sphere of
/// [`EARTH_RADIUS_M`], by the haversine formula (Sinnott, "Virtues of the Haversine", *Sky and
/// Telescope* 68(2), 1984, p159).
fn great_circle_m(from: (f64, f64), to: (f64, f64)) -> f64 {
    let (lat1, lat2) = (from.0.to_radians(), to.0.to_radians());
    let (half_dlat, half_dlon) = ((lat2 - lat1) / 2.0, (to.1 - from.1).to_radians() / 2.0);
    let h = half_dlat.sin().powi(2) + lat1.cos() * lat2.cos() * half_dlon.sin().powi(2);
    2.0 * EARTH_RADIUS_M * h.sqrt().min(1.0).asin()
}

/// The change of a track a second over a speed column, for each pair of rows a fix apart taken in
/// time order within each tracker: from the last row whose point differed, as the ground station
/// repeats a fix it hasn't heard anew, to the next that differs, more than [`MIN_STEP_S`] and
/// at most [`MAX_STEP_S`] later, at a speed of at least [`MIN_SPEED`] as written.
fn speed_check(sets: &[Set], track: Track, speed: usize) -> Check {
    let mut ratios = Vec::new();
    for set in sets {
        let mut order: Vec<usize> = (0..set.times_ns.len()).collect();
        order.sort_by_key(|&row| set.times_ns[row]);
        let mut start: Option<(usize, (f64, f64))> = None;
        for row in order {
            let Some(point) = track.at(set, row) else {
                start = None;
                continue;
            };
            if let Some((from, from_point)) = start {
                if point == from_point {
                    continue;
                }
                let step_s = seconds_between(set.times_ns[from], set.times_ns[row]);
                let speed = set.values[speed][row];
                if step_s > MIN_STEP_S && step_s <= MAX_STEP_S && speed.abs() >= MIN_SPEED {
                    ratios.push(track.change(from_point, point) / step_s / speed);
                }
            }
            start = Some((row, point));
        }
    }
    Check::from_ratios(ratios)
}

/// Each decided kind: whether it is read in its unit, and the file's notes on them.
struct Decisions {
    heights: bool,
    vertical: bool,
    horizontal: bool,
    heading: bool,
    battery: bool,
    notes: Vec<String>,
}

impl Decisions {
    fn from_file(columns: &[Column], sets: &[Set]) -> Self {
        let place = |name: &str| columns.iter().position(|c| c.name == name);
        let named = |kind: Kind| -> Vec<String> {
            columns
                .iter()
                .filter(|c| c.role == Role::Value(kind))
                .map(|c| format!("`{}`", short(&c.name)))
                .collect()
        };
        let values = |kind: Kind| {
            columns
                .iter()
                .enumerate()
                .filter(move |(_, c)| c.role == Role::Value(kind))
                .flat_map(move |(index, _)| sets.iter().flat_map(move |s| s.values[index].iter()))
                .copied()
                .filter(|value| value.is_finite())
        };
        let mut notes = Vec::new();
        let altitude = place("ALT").or_else(|| place("TRACKER Alt asl"));
        let position = place("LAT")
            .zip(place("LON"))
            .or_else(|| place("TRACKER Lat").zip(place("TRACKER Lon")));
        let altitude_name = altitude.map_or("ALT", |index| columns[index].name.as_str());

        // One rule decides the file's lengths. `HORZV` against the distance between fixes is the
        // only check tied to an absolute length, so it and the manual decide the unit of every
        // length and speed the manual gives in feet; `VERTV` against the change of altitude only
        // shows whether `VERTV` is in the altitude's unit a second.
        let horizontal = match (position, place("HORZV")) {
            (Some((lat, lon)), Some(speed)) => speed_check(sets, Track::Position(lat, lon), speed),
            _ => Check::TooFew(0),
        };
        let vertical = match (altitude, place("VERTV")) {
            (Some(altitude), Some(speed)) => speed_check(sets, Track::Height(altitude), speed),
            _ => Check::TooFew(0),
        };
        let alt_agl = alt_agl_check(columns, sets);
        let feet = !matches!(horizontal, Check::Disagrees(..));
        let kept = if !feet {
            Some(Kept::HorizontalSpeed)
        } else if alt_agl.is_some_and(|check| !check.agrees()) {
            Some(Kept::AltAgl)
        } else if matches!(vertical, Check::Disagrees(..)) {
            Some(Kept::VerticalSpeed)
        } else {
            None
        };
        let horizontal_words =
            horizontal.words("the distance between fixes a second, in feet,", "HORZV");
        let vertical_words = vertical.words(
            &format!("the change of `{altitude_name}` a second"),
            "VERTV",
        );

        if place("HORZV").is_some() {
            notes.push(match horizontal {
                Check::Agrees(..) => format!(
                    "`HORZV` read in feet per second, from the manual and the distance between \
                     fixes: {horizontal_words}"
                ),
                Check::TooFew(_) => format!(
                    "`HORZV` read in feet per second, from the manual alone: {horizontal_words}"
                ),
                Check::Disagrees(..) => format!(
                    "`HORZV` kept as written, the distance between fixes disagrees with the \
                     manual's feet per second: {horizontal_words}; every length and speed the \
                     manual gives in feet is kept as written with it"
                ),
            });
        }

        let mut heights = named(Kind::Height {
            above_sea_level: true,
        });
        heights.extend(named(Kind::Height {
            above_sea_level: false,
        }));
        if !heights.is_empty() {
            let names = list(&heights);
            let mut evidence = vec![horizontal_words.clone()];
            if place("VERTV").is_some() {
                evidence.push(vertical_words.clone());
            }
            evidence.extend(alt_agl.map(AltAgl::words));
            evidence.extend(altitude_agl_words(columns, sets));
            let evidence = evidence.join("; ");
            notes.push(match kept {
                Some(why) => format!("{names} kept as written, {}: {evidence}", why.words()),
                None => {
                    let by_horizontal = matches!(horizontal, Check::Agrees(..));
                    let by_alt_agl = alt_agl.is_some_and(|check| check.rows > 0);
                    let source = match (by_horizontal, by_alt_agl) {
                        (true, false) => "from the manual and the horizontal speed",
                        (false, false) => "from the manual alone",
                        (true, true) => "from the manual, the horizontal speed and `Alt AGL (ft)`",
                        (false, true) => "from the manual and `Alt AGL (ft)`",
                    };
                    format!("{names} read in feet, {source}: {evidence}")
                }
            });
        }

        if place("VERTV").is_some() {
            notes.push(match (kept, altitude, vertical) {
                (Some(Kept::VerticalSpeed), ..) => format!(
                    "`VERTV` kept as written, not in the altitude's unit a second: \
                     {vertical_words}"
                ),
                (Some(why), ..) => {
                    format!("`VERTV` kept as written, {}: {vertical_words}", why.words())
                }
                (None, None, _) => "`VERTV` read in feet per second, from the manual alone: the \
                                    file has no altitude to compare it with"
                    .to_owned(),
                (None, Some(_), Check::TooFew(_)) => format!(
                    "`VERTV` read in feet per second, in the altitude's unit, from the \
                     manual alone: {vertical_words}"
                ),
                (None, Some(_), _) => format!(
                    "`VERTV` read in feet per second, in the altitude's unit, from the \
                     change of altitude: {vertical_words}"
                ),
            });
        }
        let heights_read = kept.is_none();

        let heading = values(Kind::Heading).fold(None, range);
        let heading_read = heading.is_none_or(|(lo, hi): (f64, f64)| lo >= -360.0 && hi <= 360.0);
        if place("HEAD").is_some() {
            let evidence = heading.map_or_else(
                || "the column holds no value; the manual gives degrees from north".to_owned(),
                |(lo, hi)| format!("it runs from {lo} to {hi}"),
            );
            notes.push(if heading_read {
                format!("`HEAD` read in degrees, from the manual and its range: {evidence}")
            } else {
                format!(
                    "`HEAD` kept as written, its range disagrees with the manual's degrees: \
                     {evidence}"
                )
            });
        }

        let battery = values(Kind::Battery).fold(None, range);
        let battery_read = battery.is_none_or(|(lo, hi): (f64, f64)| lo >= 0.0 && hi <= MAX_CELL_V);
        if place("BATT").is_some() {
            let evidence = match battery {
                None => "the column holds no value".to_owned(),
                Some((lo, hi)) if lo == 0.0 && hi == 0.0 => {
                    "it reads 0 on every row, which no unit changes".to_owned()
                }
                Some((lo, hi)) => format!(
                    "it runs from {lo} to {hi}, where one lithium cell reads up to about 4.2 V and \
                     the ground station's serial millivolts would be thousands"
                ),
            };
            notes.push(if battery_read {
                format!("`BATT` read in volts, from its range: {evidence}")
            } else {
                format!(
                    "`BATT` kept as written, its range isn't one lithium cell's volts: {evidence}"
                )
            });
        }

        if let Some(note) = unix_note(columns, sets) {
            notes.push(note);
        }
        Self {
            heights: heights_read,
            vertical: heights_read,
            horizontal: feet,
            heading: heading_read,
            battery: battery_read,
            notes,
        }
    }

    /// A column's quantity, `None` for one that is no channel.
    fn quantity(&self, role: Role) -> Option<Quantity> {
        let decided = |read: bool, quantity| {
            if read {
                quantity
            } else {
                Quantity::Unrecognized
            }
        };
        Some(match role {
            Role::Date | Role::Tracker | Role::Label => return None,
            Role::Clock | Role::Time => Quantity::TimeOfDay,
            Role::Value(kind) => match kind {
                Kind::Height {
                    above_sea_level: true,
                } => decided(self.heights, Quantity::GnssAltitude),
                Kind::Height {
                    above_sea_level: false,
                } => decided(self.heights, Quantity::Distance),
                Kind::VerticalSpeed => decided(self.vertical, Quantity::Speed),
                Kind::HorizontalSpeed => decided(self.horizontal, Quantity::Speed),
                Kind::Heading => decided(self.heading, Quantity::Angle),
                Kind::Battery => decided(self.battery, Quantity::Voltage),
                Kind::UnixTime | Kind::Unknown => Quantity::Unrecognized,
                Kind::Stated(quantity, _) => quantity,
            },
        })
    }

    /// A column's factor into SI, 1 for one kept as written.
    fn factor(&self, role: Role) -> f64 {
        match role {
            Role::Value(Kind::Height { .. }) if self.heights => FOOT_M,
            Role::Value(Kind::VerticalSpeed) if self.vertical => FOOT_M,
            Role::Value(Kind::HorizontalSpeed) if self.horizontal => FOOT_M,
            Role::Value(Kind::Stated(_, factor)) => factor,
            _ => 1.0,
        }
    }
}

/// The lowest and highest of a list, folded.
fn range(acc: Option<(f64, f64)>, value: f64) -> Option<(f64, f64)> {
    Some(acc.map_or((value, value), |(lo, hi)| (lo.min(value), hi.max(value))))
}

/// Why the heights and `VERTV` are kept as written, in the order the rule asks.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Kept {
    /// `HORZV` disagrees with the distance between fixes in feet: the file's lengths aren't the
    /// manual's feet.
    HorizontalSpeed,
    /// `Alt AGL (ft)`, whose header says feet, isn't the heights' difference.
    AltAgl,
    /// `VERTV` isn't the change of the altitude a second.
    VerticalSpeed,
}

impl Kept {
    /// Why, as a heights' note says before its colon.
    fn words(self) -> &'static str {
        match self {
            Self::HorizontalSpeed => "the horizontal speed disagrees with the manual's feet",
            Self::AltAgl => "`Alt AGL (ft)` disagrees with the manual's feet",
            Self::VerticalSpeed => "`VERTV` isn't their change a second",
        }
    }
}

/// The longer ground-station log's check of its heights: on how many rows `Alt AGL (ft)`, whose
/// header says feet, is `TRACKER Alt asl` less `GS Alt asl` and `GS_ASL_offset`, to 0.15 ft.
#[derive(Debug, Clone, Copy, PartialEq)]
struct AltAgl {
    held: usize,
    rows: usize,
}

impl AltAgl {
    /// Whether the heights are the header's feet: the difference holds on half the rows or more,
    /// or there is no row to check.
    fn agrees(self) -> bool {
        self.rows == 0 || self.held * 2 >= self.rows
    }

    fn words(self) -> String {
        format!(
            "`Alt AGL (ft)` is `TRACKER Alt asl` less `GS Alt asl` and `GS_ASL_offset`, to 0.15 ft, \
             on {} of {} rows",
            self.held, self.rows
        )
    }
}

/// The longer ground-station log's [`AltAgl`] check, `None` for a file without its four columns.
fn alt_agl_check(columns: &[Column], sets: &[Set]) -> Option<AltAgl> {
    let place = |name: &str| columns.iter().position(|c| c.name == name);
    let ((agl, tracker), (station, offset)) =
        place("Alt AGL (ft)")
            .zip(place("TRACKER Alt asl"))
            .zip(place("GS Alt asl").zip(place("GS_ASL_offset")))?;
    let (mut held, mut rows) = (0usize, 0usize);
    for set in sets {
        for row in 0..set.times_ns.len() {
            let at = |column: usize| set.values[column][row];
            let residual = at(agl) - (at(tracker) - at(station) - at(offset));
            if residual.is_finite() {
                rows += 1;
                held += usize::from(residual.abs() <= 0.15);
            }
        }
    }
    Some(AltAgl { held, rows })
}

/// What the newer tracker log's `Altitude AGL` shows: on how many rows it is `ALT` less one
/// height, the pad's. It doesn't decide a unit, as both are in the file's one unit.
fn altitude_agl_words(columns: &[Column], sets: &[Set]) -> Option<String> {
    let place = |name: &str| columns.iter().position(|c| c.name == name);
    let (altitude, agl) = place("ALT").zip(place("Altitude AGL"))?;
    let mut differences: Vec<i64> = sets
        .iter()
        .flat_map(|set| {
            set.values[altitude]
                .iter()
                .zip(&set.values[agl])
                .map(|(alt, agl)| alt - agl)
        })
        .filter(|difference| difference.is_finite())
        // Tenths of a foot, as the newer log writes both; a difference past an i64's span
        // saturates and so matches no other.
        .map(|difference| (difference * 10.0).round() as i64)
        .collect();
    differences.sort_unstable();
    let rows = differences.len();
    let most = differences
        .chunk_by(|a, b| a == b)
        .map(<[i64]>::len)
        .max()
        .unwrap_or(0);
    Some(format!(
        "`Altitude AGL` is `ALT` less one height, to 0.05 ft, on {most} of {rows} rows, as a \
         height above the pad is"
    ))
}

/// The note on `UNIXTIME`: how far it runs from `UTCTIME`, and where each would put the sun.
fn unix_note(columns: &[Column], sets: &[Set]) -> Option<String> {
    let unix = columns
        .iter()
        .position(|c| c.role == Role::Value(Kind::UnixTime))?;
    // The offsets are taken from each row's time, which is `UTCTIME`'s in a tracker's log.
    columns.iter().find(|c| c.role == Role::Clock)?;
    let lon = columns.iter().position(|c| c.name == "LON");
    let mut offsets: Vec<f64> = Vec::new();
    let mut solar: Option<(f64, f64)> = None;
    for set in sets {
        for (row, &instant) in set.times_ns.iter().enumerate() {
            let written = set.values[unix][row];
            if !written.is_finite() {
                continue;
            }
            let utc_s = instant.div_euclid(NS_PER_S) as f64
                + instant.rem_euclid(NS_PER_S) as f64 / NS_PER_S as f64;
            offsets.push(written - utc_s);
            if solar.is_none()
                && let Some(longitude) = lon.map(|lon| set.values[lon][row])
                && longitude.is_finite()
            {
                let unix_of_day = written.rem_euclid(86_400.0) / 3_600.0;
                solar = Some((
                    solar_hours(instant, longitude),
                    (unix_of_day + longitude / 15.0).rem_euclid(24.0),
                ));
            }
        }
    }
    let hours = offsets.first().map(|offset| (offset / 3_600.0).round())?;
    // Cells written to the millisecond, so a whole number of hours is met to well within 0.01 s.
    let whole = offsets
        .iter()
        .all(|offset| (offset - hours * 3_600.0).abs() < 0.01);
    let sun = solar.map_or_else(String::new, |(utc, unix)| {
        format!(
            "; read as UTC at the first fix's longitude, `UTCTIME` gives {utc:.1} h of local mean \
             solar time and `UNIXTIME` {unix:.1} h"
        )
    });
    Some(if whole {
        let offset = if hours >= 0.0 {
            format!("{hours} h ahead of")
        } else {
            format!("{} h behind", -hours)
        };
        format!(
            "`UNIXTIME` kept as written, off `UTCTIME` by whole hours: it runs {offset} \
             `UTCTIME` on every row{sun}"
        )
    } else {
        let (lo, hi) = offsets
            .iter()
            .copied()
            .fold(None, range)
            .unwrap_or_default();
        format!(
            "`UNIXTIME` kept as written, off `UTCTIME` by other than one whole number of hours: \
             it runs from {lo:.3} s to {hi:.3} s ahead of `UTCTIME`{sun}"
        )
    })
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

    const OLDER: &str =
        "UTCTIME,UNIXTIME,ALT,LAT,LON,#SATS,FIX,HORZV,VERTV,HEAD,FLAGS,>40,>32,>24,RSSI,BATT";
    const GROUND: &str =
        "TRACKER,DATE,TIME,LAT,LON,ALT,FIX,HORZV,VERTV,HEAD,FLAGS,#TOT,>40,>32,>24,RSSI,BATT";
    const LONGER: &str = "TRACKER,DATE,TIME,GS Lat,GS Lon,GS Alt asl,TRACKER Lat,TRACKER Lon,\
        TRACKER Alt asl,FIX,HORZV,VERTV,HEAD,FLAGS,#TOT,>40,>32,>24,RSSI Down,RSSI Up,\
        Horizontal range (ft),Alt AGL (ft),Total range (ft),LoRa Max Range (kft),\
        LoRa Success Rate,BATT,GS_batt_v,Down Sent,Down Rcvd,Found Sent,Up Sent,Up Rccvd,\
        Found rocket ID,Found rocket lat,Found rocket lon,Found rocket altitude,Found rocket fix,\
        Found rocket batt_v,Found RSSI,GS_ASL_offset";

    /// Meters per degree of latitude on the reader's sphere.
    fn degree_m() -> f64 {
        EARTH_RADIUS_M.to_radians()
    }

    /// An older tracker log of rows a second apart: each row's altitude, its upward and
    /// horizontal speeds as written, moving north by 30 ft a row.
    fn tracker_file(rows: usize, alt: impl Fn(usize) -> f64, vertv: f64, horzv: f64) -> String {
        tracker_file_with(rows, alt, vertv, horzv, 90, "4.0")
    }

    /// [`tracker_file`] with a heading and a battery cell.
    fn tracker_file_with(
        rows: usize,
        alt: impl Fn(usize) -> f64,
        vertv: f64,
        horzv: f64,
        head: i32,
        batt: &str,
    ) -> String {
        moving_file(rows, alt, vertv, horzv, 30.0 * FOOT_M, head, batt)
    }

    /// [`tracker_file_with`] moving north by `step_m` meters a row.
    fn moving_file(
        rows: usize,
        alt: impl Fn(usize) -> f64,
        vertv: f64,
        horzv: f64,
        step_m: f64,
        head: i32,
        batt: &str,
    ) -> String {
        let mut text = format!("{OLDER}\n");
        for row in 0..rows {
            let lat = 40.0 + step_m * row as f64 / degree_m();
            text.push_str(&format!(
                "Jun 1 2025 18:00:{row:02}.000 UTC,{}.000,{},{lat:.7},-105.0,12,3,{horzv},{vertv},\
                 {head},0x03,1,2,3,-50,{batt}\n",
                1_748_800_800 + row,
                alt(row)
            ));
        }
        text
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels.iter().find(|c| c.column == column).unwrap()
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-9 * b.abs().max(1.0)
    }

    fn climbing(row: usize) -> f64 {
        1000.0 + 100.0 * row as f64
    }

    #[test]
    fn the_four_headers_are_recognized_and_nothing_else() {
        let newer = format!(
            "{OLDER},Altitude AGL,Launch detection,Apogee detection,Landing detection,\
             Distance (feet)"
        );
        for header in [OLDER, newer.as_str(), GROUND, LONGER] {
            assert!(
                recognizes(&format!("\u{feff}\r\n\n{header}\r\n")),
                "{header}"
            );
            assert!(recognizes(&header.replace(',', " , ")), "{header}");
        }
        for other in [
            "",
            "UTCTIME,UNIXTIME,ALT\n",
            "UNIXTIME,UTCTIME,ALT,LAT,LON\n",
            "utctime,unixtime,alt,lat,lon\n",
            "TRACKER,DATE,TIME,ALT,FIX\n",
            "TRACKER,DATE,TIME,GS Lat,GS Lon\n",
            "DATE,TIME,LAT,LON\n",
            "Time,Altitude,Velocity\n",
            "Year,Month,Day,Time,Flight_Time_(s),Sync,Gyro_X\n",
            "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\n",
        ] {
            assert!(!recognizes(other), "{other:?}");
        }
    }

    /// `UTCTIME` against a hand computation, each month's name, leap days by the Gregorian rule,
    /// and a strict reading of everything else.
    #[test]
    fn tracker_times_are_read_strictly() {
        // 2031-07-14 17:04:31.250 UTC is 1,941,815,071.25 s after 1970.
        assert_eq!(
            utc_time("Jul 14 2031 17:04:31.250 UTC"),
            Some(1_941_815_071_250_000_000)
        );
        assert_eq!(utc_time("Jan 1 1970 00:00:00 UTC"), Some(0));
        assert_eq!(
            utc_time("jul  14 2031 17:04:31.250   UTC"),
            utc_time("Jul 14 2031 17:04:31.250 UTC")
        );
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let mut last = i64::MIN;
        for month in months {
            let time = utc_time(&format!("{month} 1 2026 00:00:00 UTC")).unwrap();
            assert!(time > last, "{month}");
            last = time;
        }
        // 2024-02-29 is day 19,782 after 1970-01-01; the next day is March 1.
        assert_eq!(
            utc_time("Feb 29 2024 00:00:00 UTC"),
            Some(19_782 * NS_PER_DAY)
        );
        assert_eq!(
            utc_time("Mar 1 2024 00:00:00 UTC"),
            Some(19_783 * NS_PER_DAY)
        );
        assert!(
            utc_time("Feb 29 2000 12:00:00 UTC").is_some(),
            "divisible by 400"
        );
        for bad in [
            "Feb 29 2023 12:00:00 UTC",
            "Feb 29 2100 12:00:00 UTC",
            "Apr 31 2021 12:00:00 UTC",
            "Apr 0 2021 12:00:00 UTC",
            "Jul 14 2031 17:04:31.250",
            "Jul 14 2031 17:04:31.250 GMT",
            "Jul 14 2031 17:04:31.250 UTC x",
            "July 14 2031 17:04:31 UTC",
            "Jul 14 31 17:04:31 UTC",
            "Jul 14 1969 17:04:31 UTC",
            "Jul 14 2200 17:04:31 UTC",
            "Jul 100 2031 17:04:31 UTC",
            "Jul 14 2031 24:00:00 UTC",
            "",
        ] {
            assert_eq!(utc_time(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn ground_station_dates_and_times_are_read_strictly() {
        // 2000-03-01 is day 11,017; 2199-12-31, the last day read, day 84,005.
        assert_eq!(civil_date("2000-03-01"), Some(11_017));
        assert_eq!(civil_date("2199-12-31"), Some(84_005));
        assert_eq!(civil_date("2026-1-8"), civil_date("2026-01-08"));
        for bad in [
            "31-07-14",
            "2026-13-01",
            "2026-02-29",
            "2031-07-14-1",
            "2031/07/14",
            "",
        ] {
            assert_eq!(civil_date(bad), None, "{bad:?}");
        }
        assert_eq!(time_of_day_ns("0:00:00"), Some(0));
        assert_eq!(time_of_day_ns("14:27:09.375"), Some(52_029_375_000_000));
        assert_eq!(
            time_of_day_ns("23:59:60.123456789"),
            Some(86_400_123_456_789)
        );
        for bad in [
            "12:00",
            "12:00:00:00",
            "24:00:00",
            "12:60:00",
            "12:00:61",
            "12:5:00",
            "123:00:00",
            "12:00:00.",
            "12:00:00.1234567890",
            "12:00:00.x",
            "-1:00:00",
            "12:00:+1",
        ] {
            assert_eq!(time_of_day_ns(bad), None, "{bad:?}");
        }
        // A day's rollover is the date's: the last second of a year, then the first of the next.
        let before = civil_date("2025-12-31").unwrap() * NS_PER_DAY
            + time_of_day_ns("23:59:59.500").unwrap();
        let after = civil_date("2026-01-01").unwrap() * NS_PER_DAY
            + time_of_day_ns("00:00:00.250").unwrap();
        assert_eq!(seconds_between(before, after), 0.75);
        assert_eq!(clock_text(before), "23:59:59.500");
    }

    #[test]
    fn cells_are_numbers_flags_hexadecimal_or_percentages() {
        assert_eq!(value("1.5"), Some(1.5));
        assert_eq!(value("-82"), Some(-82.0));
        assert_eq!(value("TRUE"), Some(1.0));
        assert_eq!(value("FALSE"), Some(0.0));
        assert_eq!(value("0x03"), Some(3.0));
        assert_eq!(value("0xff"), Some(255.0));
        assert_eq!(value("67%"), Some(67.0));
        assert_eq!(value("100%"), Some(100.0));
        for bad in [
            "",
            "NaN",
            "inf",
            "true",
            "0x",
            "0xg",
            "67 %",
            "%",
            "x",
            "0x12345678901234",
        ] {
            assert_eq!(value(bad), None, "{bad:?}");
        }
    }

    /// The great-circle distance against a hand computation: a degree of latitude is
    /// 6,371,008.8 m × π / 180 = 111,195.08 m.
    #[test]
    fn a_degree_of_latitude_is_its_arc() {
        assert!((great_circle_m((40.0, -105.0), (41.0, -105.0)) - 111_195.08).abs() < 0.01);
        assert_eq!(great_circle_m((40.0, -105.0), (40.0, -105.0)), 0.0);
    }

    /// A file whose speeds agree with the change of its altitude and position reads them in feet
    /// per second, and its altitude in feet.
    #[test]
    fn speeds_that_agree_are_read_in_feet_per_second() {
        let log = read(&tracker_file(14, climbing, 100.0, 30.0)).unwrap();
        let up = channel(&log, "VERTV");
        assert_eq!(up.quantity, Quantity::Speed);
        assert!(close(up.values[0], 30.48));
        assert!(close(channel(&log, "HORZV").values[0], 30.0 * FOOT_M));
        let alt = channel(&log, "ALT");
        assert_eq!(alt.quantity, Quantity::GnssAltitude);
        assert!(close(alt.values[0], 304.8), "1000 ft is 304.8 m");
        assert_eq!(
            log.notes[..3],
            [
                "`HORZV` read in feet per second, from the manual and the distance between fixes: \
                 the distance between fixes a second, in feet, is a median of 1.000 times `HORZV` \
                 over 13 pairs of rows",
                "`ALT` read in feet, from the manual and the horizontal speed: the distance between \
                 fixes a second, in feet, is a median of 1.000 times `HORZV` over 13 pairs of rows; \
                 the change of `ALT` a second is a median of 1.000 times `VERTV` over 13 pairs of \
                 rows",
                "`VERTV` read in feet per second, in the altitude's unit, from the change of altitude: the \
                 change of `ALT` a second is a median of 1.000 times `VERTV` over 13 pairs of rows",
            ],
            "{:#?}",
            log.notes
        );
    }

    /// The review's file, all in meters: `ALT` from 1000 rising 50 a second, `VERTV` 50, and
    /// `HORZV` 25 over fixes 25 m apart, 30 rows a second apart. `VERTV` matches the change of
    /// `ALT`, as it would in feet; only `HORZV` against the distance between fixes, 3.28 times it
    /// in feet, shows the file isn't in feet, and every length and speed is kept as written.
    #[test]
    fn a_file_all_in_meters_keeps_its_lengths_as_written() {
        let rising = |row: usize| 1000.0 + 50.0 * row as f64;
        let log = read(&moving_file(30, rising, 50.0, 25.0, 25.0, 90, "4.0")).unwrap();
        for (column, written) in [("ALT", 1000.0), ("VERTV", 50.0), ("HORZV", 25.0)] {
            let channel = channel(&log, column);
            assert_eq!(channel.quantity, Quantity::Unrecognized, "{column}");
            assert_eq!(channel.values[0], written, "{column}");
        }
        let why = "the horizontal speed disagrees with the manual's feet: ";
        assert!(
            log.notes[0].starts_with(
                "`HORZV` kept as written, the distance between fixes disagrees with the manual's \
                 feet per second: the distance between fixes a second, in feet, is a median of \
                 3.280 times `HORZV` over 29 pairs of rows"
            ),
            "{:#?}",
            log.notes
        );
        assert!(log.notes[1].starts_with(&format!("`ALT` kept as written, {why}")));
        assert_eq!(
            log.notes[2],
            format!(
                "`VERTV` kept as written, {why}the change of `ALT` a second is a median of 1.000 \
                 times `VERTV` over 29 pairs of rows"
            )
        );
    }

    /// The same flight written in feet, as the manual gives it: read in feet and feet per second.
    #[test]
    fn the_same_file_in_feet_reads_in_feet() {
        let ft = 1.0 / FOOT_M;
        let rising = |row: usize| (1000.0 + 50.0 * row as f64) / FOOT_M;
        let log = read(&moving_file(
            30,
            rising,
            50.0 * ft,
            25.0 * ft,
            25.0,
            90,
            "4.0",
        ))
        .unwrap();
        for (column, quantity, si) in [
            ("ALT", Quantity::GnssAltitude, 1000.0),
            ("VERTV", Quantity::Speed, 50.0),
            ("HORZV", Quantity::Speed, 25.0),
        ] {
            let channel = channel(&log, column);
            assert_eq!(channel.quantity, quantity, "{column}");
            assert!(
                close(channel.values[0], si),
                "{column}: {}",
                channel.values[0]
            );
        }
        assert!(
            log.notes[1].starts_with(
                "`ALT` read in feet, from the manual and the horizontal speed: the distance \
                 between fixes a second, in feet, is a median of 1.000 times `HORZV` over 29 pairs \
                 of rows"
            ),
            "{:#?}",
            log.notes
        );
    }

    /// A file whose `VERTV` is in meters per second, 3.28 times smaller than the change of `ALT`
    /// in feet a second: `VERTV` and the heights kept as written, with notes saying so.
    #[test]
    fn a_vertical_speed_that_disagrees_is_kept_as_written() {
        let log = read(&tracker_file(14, climbing, 30.48, 30.0)).unwrap();
        let up = channel(&log, "VERTV");
        assert_eq!(up.quantity, Quantity::Unrecognized);
        assert_eq!(up.values[0], 30.48);
        let alt = channel(&log, "ALT");
        assert_eq!(alt.quantity, Quantity::Unrecognized);
        assert_eq!(alt.values[0], 1000.0);
        assert_eq!(
            log.notes[2],
            "`VERTV` kept as written, not in the altitude's unit a second: the change of `ALT` a second is \
             a median of 3.281 times `VERTV` over 13 pairs of rows"
        );
        assert!(
            log.notes[1]
                .starts_with("`ALT` kept as written, `VERTV` isn't their change a second: ")
        );
        // `HORZV` still agrees with the positions, so it is read on its own.
        assert_eq!(channel(&log, "HORZV").quantity, Quantity::Speed);
    }

    /// A `HORZV` in miles per hour (1.467 times smaller than feet per second), a heading past
    /// ±360 and a battery in millivolts are each kept as written.
    #[test]
    fn horizontal_speed_heading_and_battery_that_disagree_are_kept() {
        let mph = 30.0 / 1.466_667;
        let log = read(&tracker_file_with(14, climbing, 100.0, mph, 400, "4010")).unwrap();
        let horizontal = channel(&log, "HORZV");
        assert_eq!(horizontal.quantity, Quantity::Unrecognized);
        assert!(close(horizontal.values[0], mph));
        assert_eq!(channel(&log, "HEAD").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "BATT").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "BATT").values[0], 4010.0);
        let notes = log.notes.join("\n");
        // A `HORZV` that isn't feet per second leaves the file's lengths as written too.
        assert_eq!(channel(&log, "ALT").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "VERTV").quantity, Quantity::Unrecognized);
        for expected in [
            "`HORZV` kept as written, the distance between fixes disagrees with the manual's feet \
             per second: the distance between fixes a second, in feet, is a median of 1.466 times",
            "`HEAD` kept as written, its range disagrees with the manual's degrees: it runs from \
             400 to 400",
            "`BATT` kept as written, its range isn't one lithium cell's volts: it runs from 4010 \
             to 4010",
        ] {
            assert!(notes.contains(expected), "{expected}\n{notes}");
        }
        // Just inside the band, the speed reads in feet per second; just outside, as written.
        let inside = read(&tracker_file(14, climbing, 100.0, 30.0 / 1.39)).unwrap();
        assert_eq!(channel(&inside, "HORZV").quantity, Quantity::Speed);
        let outside = read(&tracker_file(14, climbing, 100.0, 30.0 / 1.41)).unwrap();
        assert_eq!(channel(&outside, "HORZV").quantity, Quantity::Unrecognized);
    }

    /// Too few pairs of moving rows leave the manual's unit, with a note saying the file can't
    /// tell.
    #[test]
    fn too_few_pairs_leave_the_manuals_unit() {
        let log = read(&tracker_file(5, climbing, 30.48, 9.0)).unwrap();
        assert_eq!(channel(&log, "VERTV").quantity, Quantity::Speed);
        assert_eq!(channel(&log, "ALT").quantity, Quantity::GnssAltitude);
        assert_eq!(
            log.notes[..3],
            [
                "`HORZV` read in feet per second, from the manual alone: the file has too few \
                 pairs of moving rows to check `HORZV` over, 0 of the 10 needed",
                "`ALT` read in feet, from the manual alone: the file has too few pairs of moving \
                 rows to check `HORZV` over, 0 of the 10 needed; the file has too few pairs of \
                 moving rows to check `VERTV` over, 4 of the 10 needed",
                "`VERTV` read in feet per second, in the altitude's unit, from the manual alone: the file \
                 has too few pairs of moving rows to check `VERTV` over, 4 of the 10 needed",
            ]
        );
    }

    /// A range in thousands of feet near the largest `f64` is finite as written and infinite in
    /// meters: refused on its row's line (ADR-258 §3).
    #[test]
    fn a_value_its_conversion_makes_infinite_is_refused() {
        let row = |range: &str| {
            format!(
                "T,2026-03-08,09:15:10,20,-100,3000,20.001,-100,4000,3,4,0,45,0x01,24,6,12,2,-50,\
                 -55,364.8,1005,0,{range},100%,4.0,4.1,1,1,0,1,1,,0,0,0,0,0,-60,-5"
            )
        };
        let file = format!("{LONGER}\n{}\n{}\n", row("12.5"), row("1.7e308"));
        assert_eq!(
            read(&file).unwrap_err().to_string(),
            "Featherweight GPS CSV line 3: the column LoRa Max Range (kft) holds a value too \
             large for its unit's conversion to SI"
        );
    }

    /// The longer ground-station log whose `Alt AGL (ft)` doesn't follow its heights keeps them as
    /// written.
    #[test]
    fn heights_that_disagree_with_alt_agl_are_kept() {
        let row = |agl: f64| {
            format!(
                "T,2026-03-08,09:15:10,20,-100,3000,20.001,-100,4000,3,4,0,45,0x01,24,6,12,2,-50,\
                 -55,364.8,{agl},0,12.5,100%,4.0,4.1,1,1,0,1,1,,0,0,0,0,0,-60,-5"
            )
        };
        let agrees = read(&format!("{LONGER}\n{}\n", row(1005.0))).unwrap();
        assert_eq!(
            channel(&agrees, "GS Alt asl").quantity,
            Quantity::GnssAltitude
        );
        let disagrees = read(&format!("{LONGER}\n{}\n", row(304.8))).unwrap();
        assert_eq!(
            channel(&disagrees, "GS Alt asl").quantity,
            Quantity::Unrecognized
        );
        assert_eq!(
            channel(&disagrees, "GS_ASL_offset").quantity,
            Quantity::Unrecognized
        );
        // `VERTV` is in the heights' unit a second, so it is kept with them.
        assert_eq!(channel(&agrees, "VERTV").quantity, Quantity::Speed);
        assert_eq!(
            channel(&disagrees, "VERTV").quantity,
            Quantity::Unrecognized
        );
        assert!(disagrees.notes.contains(
            &"`VERTV` kept as written, `Alt AGL (ft)` disagrees with the manual's feet: the file \
              has too few pairs of moving rows to check `VERTV` over, 0 of the 10 needed"
                .to_owned()
        ));
        assert!(disagrees.notes.iter().any(|note| note.starts_with(
            "`GS Alt asl`, `TRACKER Alt asl`, `Found rocket altitude` and `GS_ASL_offset` kept as \
             written, `Alt AGL (ft)` disagrees with the manual's feet: "
        )));
    }

    #[test]
    fn unixtime_off_by_other_than_whole_hours_is_said() {
        let text = format!(
            "{OLDER}\nJun 1 2025 18:00:00.000 UTC,1748800800.500,1,40,-105,1,3,0,0,0,0x03,1,1,1,\
             1,4\n"
        );
        let log = read(&text).unwrap();
        assert!(
            log.notes.iter().any(|note| note
                == "`UNIXTIME` kept as written, off `UTCTIME` by other than one whole number of \
                    hours: it runs from 0.500 s to 0.500 s ahead of `UTCTIME`; read as UTC at the \
                    first fix's longitude, `UTCTIME` gives 11.0 h of local mean solar time and \
                    `UNIXTIME` 11.0 h"),
            "{:#?}",
            log.notes
        );
    }

    /// A latitude beyond ±90° or a longitude beyond ±180° is a gap, counted per column; the
    /// edges themselves are read.
    #[test]
    fn positions_out_of_range_are_gaps() {
        let row = |lat: &str, lon: &str| {
            format!(
                "Jun 1 2025 18:00:00.000 UTC,1748800800,1,{lat},{lon},1,3,0,0,0,0x03,1,1,1,1,4\n"
            )
        };
        let text = format!(
            "{OLDER}\n{}{}{}{}",
            row("90", "-180"),
            row("90.5", "-105"),
            row("-91", "180.001"),
            row("40", "-181")
        );
        let log = read(&text).unwrap();
        let (lat, lon) = (channel(&log, "LAT"), channel(&log, "LON"));
        assert_eq!(lat.values[0], 90.0);
        assert_eq!(lon.values[0], -180.0);
        assert!(lat.values[1].is_nan() && lon.values[1] == -105.0);
        assert!(lat.values[2].is_nan() && lon.values[2].is_nan());
        assert!(lat.values[3] == 40.0 && lon.values[3].is_nan());
        for expected in [
            "the column \"LAT\" holds 2 latitudes beyond ±90°: they are gaps",
            "the column \"LON\" holds 2 longitudes beyond ±180°: they are gaps",
        ] {
            assert!(
                log.notes.iter().any(|note| note == expected),
                "{expected}\n{:#?}",
                log.notes
            );
        }
    }

    /// A long line that doesn't open as a header is turned away by its first names.
    #[test]
    fn a_long_line_of_other_names_is_not_a_header() {
        let commas = ",".repeat(1 << 20);
        assert!(!recognizes(&commas));
        assert!(!recognizes(&format!("UTCTIME{commas}")));
        assert!(!recognizes(&format!("TRACKER,DATE{commas}")));
        assert!(recognizes(&format!("UTCTIME,UNIXTIME{commas}LAT,LON")));
    }

    #[test]
    fn repeated_times_alone_are_noted() {
        let row = "Jun 1 2025 18:00:00.000 UTC,1748800800,1,40,-105,1,3,0,0,0,0x03,1,1,1,1,4";
        let log = read(&format!("{OLDER}\n{row}\n{row}\n")).unwrap();
        assert!(
            log.notes.contains(
                &"1 rows repeat a time already written: every row is kept, in the file's order"
                    .to_owned()
            )
        );
        assert_eq!(channel(&log, "ALT").time_s, [0.0, 0.0]);
    }

    /// Past [`MAX_TRACKERS`] trackers, a further tracker's rows are left out and counted.
    #[test]
    fn trackers_past_the_cap_are_left_out() {
        let mut text = format!("{GROUND}\n");
        for tracker in 0..70 {
            text.push_str(&format!(
                "T{tracker},2026-01-01,10:00:{:02},1,1,1,3,0,0,0,0x03,1,1,1,1,1,0\n",
                tracker % 60
            ));
        }
        text.push_str("T69,2026-01-01,10:01:00,1,1,1,3,0,0,0,0x03,1,1,1,1,1,0\n");
        let log = read(&text).unwrap();
        assert_eq!(log.channels.len(), MAX_TRACKERS * 15);
        assert_eq!(log.channels[15].column, "TIME [T1]");
        assert!(
            log.notes.contains(
                &"7 rows of 6 more trackers were left out, past the first 64 trackers or 4096 \
                  channels, so that a file of many tracker names can't make the reader hold a \
                  set of channels for each"
                    .to_owned()
            ),
            "{:#?}",
            log.notes
        );
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let row = "Jun 1 2025 18:00:00.000 UTC,1748800800,1,40,-105,1,3,0,0,0,0x03,1,1,1,1,4";
        let cases = [
            (String::new(), "the file is empty"),
            ("\n\r\n".to_owned(), "the file is empty"),
            (
                "Time,Altitude\n0,0\n".to_owned(),
                "isn't a Featherweight GPS header",
            ),
            (format!("{OLDER}\n"), "no data rows"),
            (
                "UTCTIME,UNIXTIME,LAT,LON,,x\n".to_owned(),
                "line 1: the header's column 5 has no name",
            ),
            (
                format!("{OLDER}\n{row},9\n"),
                "line 2: the row has 17 cells, more than the header's 16",
            ),
            (
                format!("{OLDER}\n{}\n", row.replace("Jun 1", "Jun 31")),
                "line 2: the time \"Jun 31 2025 18:00:00.000 UTC\" in cell 1 isn't one",
            ),
            (
                format!("{GROUND}\nT,2026-02-30,10:00:00,1,1,1\n"),
                "line 2: the date \"2026-02-30\" in cell 2 isn't one",
            ),
            (
                format!("{GROUND}\nT,2026-02-03,10:00\n"),
                "line 2: the time \"10:00\" in cell 3 isn't one",
            ),
            (
                format!("{GROUND}\nT,2026-02-03\n"),
                "line 2: the row ends after 2 cells, before its \"TIME\"",
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
    /// a gap for every cell left out; short rows within the bound are gaps, counted.
    #[test]
    fn short_rows_are_gaps_and_a_wide_header_is_refused() {
        let wide: String = (0..20_000).map(|i| format!(",c{i}")).collect();
        let rows: String = (0..100)
            .map(|i| format!("Jun 1 2025 18:00:{:02}.000 UTC\n", i % 60))
            .collect();
        let error = read(&format!("UTCTIME,UNIXTIME,LAT,LON{wide}\n{rows}"))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(
                "line 5: the header names 20004 columns, and the 4 rows down to this one hold \
                 only 4 cells"
            ),
            "{error}"
        );
        let log = read(&format!(
            "{OLDER}\nJun 1 2025 18:00:00.000 UTC,1748800800,1\n"
        ))
        .unwrap();
        assert!(channel(&log, "LAT").values[0].is_nan());
        assert!(log.notes.contains(
            &"1 rows, the first on line 2, have fewer cells than the header's 16: the cells they \
              leave out are gaps"
                .to_owned()
        ));
    }

    /// A column the reader doesn't know is kept as written, with a note; notes about columns stay
    /// few.
    #[test]
    fn unknown_columns_are_kept_and_their_notes_stay_few() {
        let unknown: String = (0..30).map(|i| format!(",u{i}")).collect();
        let row = format!(
            "Jun 1 2025 18:00:00.000 UTC,1748800800,1,40,-105{}",
            ",7".repeat(30)
        );
        let log = read(&format!("UTCTIME,UNIXTIME,ALT,LAT,LON{unknown}\n{row}\n")).unwrap();
        assert_eq!(channel(&log, "u29").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "u29").values, [7.0]);
        assert_eq!(
            log.notes
                .iter()
                .filter(|note| note.contains("isn't one this reader knows"))
                .count(),
            20
        );
        assert!(log.notes.last().unwrap().starts_with("and 10 more notes"));
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has a time for each value on
        /// every channel, and an empty primary track.
        #[test]
        fn any_text_is_read_or_refused(
            rows in proptest::collection::vec(
                (proptest::bool::ANY, "[-0-9.,: xaTRUEFLSNC%Jn]{0,60}"),
                0..10,
            ),
            layout in 0..3usize,
        ) {
            let header = [OLDER, GROUND, LONGER][layout];
            // Most rows open with a clock, so that the rest of the row is read too.
            let clock = if layout == 0 {
                "Jun 1 2025 18:00:00.000 UTC,"
            } else {
                "T,2026-01-01,10:00:00,"
            };
            let rows: Vec<String> = rows
                .into_iter()
                .map(|(timed, rest)| if timed { format!("{clock}{rest}") } else { rest })
                .collect();
            let text = format!("{header}\n{}", rows.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
                for channel in &log.channels {
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                    proptest::prop_assert!(channel.time_s.iter().all(|t| t.is_finite()));
                }
            }
        }
    }
}
