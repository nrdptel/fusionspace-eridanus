//! The Featherweight Raven's flight logs, as the Featherweight Interface Program (FIP) exports
//! them to a comma-separated file.
//!
//! The Raven samples its sensors at different rates, so the export gives each channel its own
//! clock: a `Time@` column, then the channel's values, then, in most exports, a status column
//! named `bILBA`. One header line names the columns; each row below holds the next sample of every
//! channel, so a row's cells for different channels were taken at different times:
//!
//! ```text
//! Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,Time@Baro (Atm),Baro (Atm),bILBA,…
//! 0,1.02,b0000,0.04375,0.97,b0000,…
//! 0.0025,1.01,b0000,0.09375,0.97,b0000,…
//! ```
//!
//! **Channels.** The names the reader knows, each read into the record's SI unit
//! ([`Quantity::unit`]); the column keeps the file's name, brackets and all:
//!
//! | Column | Quantity | From |
//! |---|---|---|
//! | `Axial Accel (Gs)`, `Lateral Accel (Gs)` | acceleration | g, × 9.80665 m/s² (standard gravity) |
//! | `Baro (Atm)` | pressure | atm, × 101,325 Pa |
//! | `Current Draw (A)` | current | A |
//! | `Temperature (F)` | temperature | °F |
//! | `Volts Battery (V)`, `Volts Pyro Apogee (V)` and the other `Volts … (V)` | voltage | V |
//! | `Velocity (Accel-Ft/Sec)`, `[Velocity (Accel-Ft/Sec)]` | speed | ft/s |
//! | `[Velocity (Accel-MPH)]` | speed | mph, × 0.44704 m/s |
//! | `[Altitude (Accel-Ft)]` | inertial altitude | ft |
//! | `[Altitude (Baro-Ft-AGL)]`, `Altitude (Ft-AGL)` | altitude above the pad | ft |
//! | `[Altitude (Baro-Ft-ASL)]` | altitude above sea level | ft |
//! | `LiftOff` | a number, as written | |
//! | `bILBA` | a number: the status bits | |
//!
//! A column the reader doesn't know is kept as written, as [`Quantity::Unrecognized`], with a
//! note. The bracketed names are figures the FIP works out from what the Raven recorded rather
//! than recordings: their names say so (`Accel`, `Baro`), and each sits on the clock of the sensor
//! it comes from.
//!
//! **The status column.** Each `bILBA` cell is `b` and four binary digits, such as `b1100`, and is
//! kept as a [`Quantity::Number`] channel, beside the channel it follows, holding the digits'
//! value read with the leftmost digit the most significant: `b0101` is 5, `b1100` is 12. No
//! published source says what the digits mean. Featherweight's Raven manual (*Raven User's
//! Manual, updated for the Raven4*, 2020-08-15) says the Raven records "all flight events used for
//! deployment logic" at 20 Hz and that the FIP can plot them, but it describes neither the export
//! nor this column; Debrief uses the column only to recognize the format. The letters `I`, `L`,
//! `B` and `A` may name flight events (`L` liftoff, `B` burnout and `A` apogee would fit the
//! manual's words), but that is a guess, so the reader states no events from them.
//!
//! **Rows of different lengths.** The channels run for different spans, and a channel with no
//! sample in a row is written as two empty cells, its time's and its value's, *without* its status
//! cell. In an export with status columns, a row is therefore one cell shorter for each channel
//! that has stopped, and every channel after it is shifted one cell to the left of its header; an
//! export without them keeps every row as wide as its header. The reader walks each row channel by
//! channel: two empty cells are a channel with no sample in that row, anything else a sample with
//! its status, and the row must end where the last channel does. This rule is the files', not a
//! document's: Debrief's reader assumes every row as wide as the header, which in an export with
//! status columns reads every channel after the first one to stop from the wrong cells. Debrief's
//! public sample of a Raven flight bears the rule out row for row, as do the 79 Raven exports in
//! the private collection of flights this project checks against.
//!
//! **Layouts.** Four were seen: each channel's clock and status (16 channels); each channel's clock
//! without status columns (13, 16 or 18 channels); a two-channel export headed `Time-Axial Accel
//! (Gs), Axial Accel (Gs), bILBA, Time (s), Altitude (Ft-AGL), bILBA`, with a space after each
//! comma; and an export with one `Time` column for every channel, which in the one such file read
//! repeats whole rows. A column whose name starts with `Time` starts a clock; the columns up to the
//! next clock are its channels, each followed by its status column if it has one.
//!
//! **The primary track.** The record's time and altitude are `[Altitude (Baro-Ft-AGL)]`'s, on its
//! own clock: the Raven's barometric altitude above the pad. The two-channel layout has no such
//! column; its `Altitude (Ft-AGL)` names no sensor, and is taken instead, with a note: it is the
//! only altitude above the pad the file holds, and in the one such file read it runs at 20 Hz, the
//! rate the manual gives for the barometer (the accelerometer's is 400 Hz). A file with neither is
//! refused: working an altitude out of `Baro (Atm)` is a reading, not this reader's to make. No
//! column is a vertical speed from the barometer, so [`FlightLog::vertical_speed_m_s`] is `None`;
//! the velocities are the accelerometer's, kept as channels. The temperature and battery voltage
//! fill [`FlightLog::temperature_k`] and [`FlightLog::battery_v`] only when they share the
//! altitude's clock, as they do in the one-clock layout; otherwise they are channels only.
//!
//! **Cells.** `-1.#QNAN` and the other ways Windows writes a value that isn't a number, as some
//! exports write the accelerometer's derived channels before or after their data, are gaps,
//! `NaN`, counted in a note; so is an empty value in a sample that has a time. Within a channel,
//! a time that repeats the one before is dropped with a note saying how many of the repeats
//! differed from the sample they repeat (the one-clock layout's repeated rows are identical
//! copies); a time that goes back is refused, naming the line. A byte-order mark is dropped, lines
//! may end in CR, LF or both, and the cells may be padded with spaces.
//!
//! **Where this comes from.** The layout, the `Time@`/`bILBA` header and the units follow the
//! reader in Debrief (`lib/parsers/featherweightFip.ts` and `lib/parsers/multiTimebase.ts`, MIT,
//! the project owner's own; see `THIRD-PARTY-NOTICES.md`), which was written from exported files
//! and cites no document; the clocks' rates are Featherweight's manual's. Where this reader departs
//! from Debrief's: it keeps every channel on its own clock where Debrief resamples them onto the
//! densest; it walks the rows as above; it reads the layouts without status columns or with one
//! clock, which Debrief doesn't recognize; it takes the altitude above the pad from the export's
//! own column where Debrief works one out from the pressure; and it refuses a time that goes back
//! where Debrief sorts the samples.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;

/// The format's name in errors.
const FORMAT: &str = "Raven FIP CSV";

/// Pascals in a standard atmosphere, exactly (ISO 2533).
const ATM_PA: f64 = 101_325.0;

/// Meters per second in a mile per hour, exactly (the international mile, 1,609.344 m, per 3,600 s).
const MPH_M_S: f64 = 0.447_04;

/// The name of a status column, as the FIP writes it (compared without regard to case).
const STATUS: &str = "bilba";

/// The name of the barometric altitude above the pad, the primary track, brackets stripped.
const BARO_AGL: &str = "altitude (baro-ft-agl)";

/// The two-channel layout's altitude above the pad, which names no sensor.
const BARE_AGL: &str = "altitude (ft-agl)";

/// How a known column's values become SI.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Scale {
    /// Multiplied by a factor.
    By(f64),
    /// Degrees Fahrenheit to kelvin.
    Fahrenheit,
}

/// The columns the reader knows, brackets stripped and in lower case, with what each measures and
/// how its values become SI. The `Volts … (V)` columns are matched by their shape instead
/// ([`known`]).
const KNOWN: [(&str, Quantity, Scale); 12] = [
    (
        "axial accel (gs)",
        Quantity::Acceleration,
        Scale::By(STANDARD_GRAVITY_MPS2),
    ),
    (
        "lateral accel (gs)",
        Quantity::Acceleration,
        Scale::By(STANDARD_GRAVITY_MPS2),
    ),
    ("baro (atm)", Quantity::Pressure, Scale::By(ATM_PA)),
    ("current draw (a)", Quantity::Current, Scale::By(1.0)),
    ("temperature (f)", Quantity::Temperature, Scale::Fahrenheit),
    (
        "velocity (accel-ft/sec)",
        Quantity::Speed,
        Scale::By(FOOT_M),
    ),
    ("velocity (accel-mph)", Quantity::Speed, Scale::By(MPH_M_S)),
    (
        "altitude (accel-ft)",
        Quantity::InertialAltitude,
        Scale::By(FOOT_M),
    ),
    (BARO_AGL, Quantity::AltitudeAboveGround, Scale::By(FOOT_M)),
    (BARE_AGL, Quantity::AltitudeAboveGround, Scale::By(FOOT_M)),
    (
        "altitude (baro-ft-asl)",
        Quantity::AltitudeMsl,
        Scale::By(FOOT_M),
    ),
    ("liftoff", Quantity::Number, Scale::By(1.0)),
];

/// A column name brackets stripped, trimmed and in lower case, as [`KNOWN`] lists them.
fn key(name: &str) -> String {
    let name = name.trim();
    let name = name
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .unwrap_or(name);
    name.trim().to_ascii_lowercase()
}

/// What a known column measures and how it becomes SI; `None` for a column the reader doesn't know.
fn known(name: &str) -> Option<(Quantity, Scale)> {
    let key = key(name);
    if key.starts_with("volts ") && key.ends_with("(v)") {
        return Some((Quantity::Voltage, Scale::By(1.0)));
    }
    KNOWN
        .iter()
        .find(|(known, _, _)| *known == key)
        .map(|(_, quantity, scale)| (*quantity, *scale))
}

/// Whether a header cell starts a clock: `Time@…`, `Time-…`, `Time (s)` or `Time`.
fn is_clock(name: &str) -> bool {
    name.get(..4)
        .is_some_and(|start| start.eq_ignore_ascii_case("time"))
}

/// One data column of a clock: its value cells, and its status cells if it has a status column.
struct Column {
    name: String,
    quantity: Quantity,
    scale: Scale,
    values: Vec<f64>,
    status: Option<Vec<f64>>,
    /// Cells left empty in samples that have a time.
    empty: usize,
    /// Cells that aren't a number, such as `-1.#QNAN`, and the first of them as written.
    not_numbers: usize,
    not_number_example: String,
}

/// A clock and the columns on it.
struct Clock {
    name: String,
    time_s: Vec<f64>,
    columns: Vec<Column>,
    /// Repeated times dropped, and how many of them differed from the sample they repeat.
    repeats: usize,
    differing_repeats: usize,
}

impl Clock {
    /// Cells a sample takes: the time, each value and each status.
    fn sample_width(&self) -> usize {
        1 + self.columns.len() + self.columns.iter().filter(|c| c.status.is_some()).count()
    }

    /// Cells a row gives the clock when it has no sample: the time and each value, empty.
    fn empty_width(&self) -> usize {
        1 + self.columns.len()
    }
}

/// Reads a Featherweight Raven's FIP CSV export from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line isn't a FIP export's header (a clock column first,
/// and a channel name the Raven writes); [`LogError::Syntax`] for a header the reader can't lay
/// out, a file with no altitude above the pad, a row whose cells don't fall into its channels, a
/// time or value that isn't a number, or a time that goes back; [`LogError::NoData`] if the
/// altitude has no samples.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file is empty".to_owned(),
        });
    };
    let mut notes = Notes::default();
    let mut clocks = header_clocks(header, header_line, &mut notes)?;

    for (number, line) in lines {
        read_row(&mut clocks, line, number)?;
    }
    build(clocks, header_line, notes)
}

/// The file's lines, numbered from 1, ended by CR, LF or CR LF.
fn numbered_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let mut rest = Some(text);
    let mut number = 0;
    std::iter::from_fn(move || {
        let current = rest?;
        number += 1;
        match current.find(['\r', '\n']) {
            Some(end) => {
                let skip = if current[end..].starts_with("\r\n") {
                    2
                } else {
                    1
                };
                rest = Some(&current[end + skip..]);
                Some((number, &current[..end]))
            }
            None => {
                rest = None;
                (!current.is_empty()).then_some((number, current))
            }
        }
    })
}

/// The clocks the header lays out, each with its columns.
fn header_clocks(header: &str, line: usize, notes: &mut Notes) -> Result<Vec<Clock>, LogError> {
    let names: Vec<&str> = header.split(',').map(str::trim).collect();
    let is_raven = names.first().is_some_and(|first| is_clock(first))
        && names.iter().any(|name| known(name).is_some());
    if !is_raven {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line, {:?}, isn't a FIP export's header, a clock such as \
                 `Time@Axial Accel (Gs)` and the Raven's channels",
                short(header)
            ),
        });
    }
    let mut clocks: Vec<Clock> = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let column_number = index + 1;
        if name.is_empty() {
            return Err(syntax(
                line,
                format!("the header's column {column_number} has no name"),
            ));
        }
        if is_clock(name) {
            if let Some(last) = clocks.last()
                && last.columns.is_empty()
            {
                return Err(syntax(
                    line,
                    format!("the clock {:?} has no channel", short(&last.name)),
                ));
            }
            clocks.push(Clock {
                name: (*name).to_owned(),
                time_s: Vec::new(),
                columns: Vec::new(),
                repeats: 0,
                differing_repeats: 0,
            });
            continue;
        }
        // Invariant: the first name is a clock's, checked above, so a clock is open here.
        let Some(clock) = clocks.last_mut() else {
            continue;
        };
        if name.eq_ignore_ascii_case(STATUS) {
            match clock.columns.last_mut() {
                Some(column) if column.status.is_none() => column.status = Some(Vec::new()),
                _ => {
                    return Err(syntax(
                        line,
                        format!("the status column {column_number} follows no channel of its own"),
                    ));
                }
            }
            continue;
        }
        let (quantity, scale) = known(name).unwrap_or_else(|| {
            notes.push(format!(
                "the column {:?} isn't one this reader knows, and was kept as written",
                short(name)
            ));
            (Quantity::Unrecognized, Scale::By(1.0))
        });
        clock.columns.push(Column {
            name: (*name).to_owned(),
            quantity,
            scale,
            values: Vec::new(),
            status: None,
            empty: 0,
            not_numbers: 0,
            not_number_example: String::new(),
        });
    }
    if let Some(last) = clocks.last()
        && last.columns.is_empty()
    {
        return Err(syntax(
            line,
            format!("the clock {:?} has no channel", short(&last.name)),
        ));
    }
    Ok(clocks)
}

/// Reads one row into the clocks: each clock either has a sample in it, its time, values and
/// statuses, or two or more empty cells, its time's and its values', without its status cells.
fn read_row(clocks: &mut [Clock], line: &str, number: usize) -> Result<(), LogError> {
    let cells: Vec<&str> = line.split(',').map(str::trim).collect();
    let mut at = 0;
    for clock in clocks.iter_mut() {
        let empty_width = clock.empty_width();
        let Some(head) = cells.get(at..at + empty_width) else {
            return Err(syntax(
                number,
                format!(
                    "the row ends at cell {} before the clock {:?}'s cells",
                    cells.len(),
                    short(&clock.name)
                ),
            ));
        };
        if head.iter().all(|cell| cell.is_empty()) {
            at += empty_width;
            continue;
        }
        let width = clock.sample_width();
        let Some(sample) = cells.get(at..at + width) else {
            return Err(syntax(
                number,
                format!(
                    "the row ends at cell {} inside the clock {:?}'s sample",
                    cells.len(),
                    short(&clock.name)
                ),
            ));
        };
        read_sample(clock, sample, number, at)?;
        at += width;
    }
    if at != cells.len() {
        return Err(syntax(
            number,
            format!(
                "the row has {} cells and its channels' samples take {at}",
                cells.len()
            ),
        ));
    }
    Ok(())
}

/// Reads one clock's sample, its cells starting at the row's cell `first` (0-based).
fn read_sample(
    clock: &mut Clock,
    sample: &[&str],
    number: usize,
    first: usize,
) -> Result<(), LogError> {
    let time = sample[0];
    let time_s = time
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| {
            syntax(
                number,
                format!(
                    "the time {:?} in cell {} isn't a finite number",
                    short(time),
                    first + 1
                ),
            )
        })?;
    // Each column's value, and its status where it has one, as they stand in the row.
    let mut parsed: Vec<(f64, Option<f64>)> = Vec::with_capacity(clock.columns.len());
    let mut at = 1;
    for column in &mut clock.columns {
        let cell = sample[at];
        let value = value_cell(column, cell).ok_or_else(|| {
            syntax(
                number,
                format!(
                    "{:?} in cell {}, under {:?}, isn't a number",
                    short(cell),
                    first + at + 1,
                    short(&column.name)
                ),
            )
        })?;
        at += 1;
        let status = if column.status.is_some() {
            let cell = sample[at];
            let status = status_cell(cell).ok_or_else(|| {
                syntax(
                    number,
                    format!(
                        "the status {:?} in cell {} isn't `b` and binary digits",
                        short(cell),
                        first + at + 1
                    ),
                )
            })?;
            at += 1;
            Some(status)
        } else {
            None
        };
        parsed.push((value, status));
    }
    if let Some(&last) = clock.time_s.last() {
        if time_s < last {
            return Err(syntax(
                number,
                format!(
                    "the time {time_s} s on the clock {:?} comes before the one above it, {last} s",
                    short(&clock.name)
                ),
            ));
        }
        if time_s == last {
            clock.repeats += 1;
            let same = clock
                .columns
                .iter()
                .zip(&parsed)
                .all(|(column, (value, status))| {
                    same_bits(column.values.last(), Some(value))
                        && same_bits(
                            column.status.as_ref().and_then(|statuses| statuses.last()),
                            status.as_ref(),
                        )
                });
            if !same {
                clock.differing_repeats += 1;
            }
            return Ok(());
        }
    }
    clock.time_s.push(time_s);
    for (column, (value, status)) in clock.columns.iter_mut().zip(parsed) {
        column.values.push(value);
        if let (Some(statuses), Some(status)) = (column.status.as_mut(), status) {
            statuses.push(status);
        }
    }
    Ok(())
}

/// Whether two optional values are the same number, `NaN` matching `NaN`.
fn same_bits(a: Option<&f64>, b: Option<&f64>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a.to_bits() == b.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

/// A value cell in SI; `NaN`, counted, for an empty cell or one that isn't a number in the way
/// Windows writes them; `None` for anything else that isn't a number.
fn value_cell(column: &mut Column, cell: &str) -> Option<f64> {
    if cell.is_empty() {
        column.empty += 1;
        return Some(f64::NAN);
    }
    match cell.parse::<f64>() {
        Ok(value) if value.is_finite() => Some(si(column.scale, value)),
        Ok(_) => Some(not_a_number(column, cell)),
        Err(_) if is_windows_non_number(cell) => Some(not_a_number(column, cell)),
        Err(_) => None,
    }
}

/// Counts a cell that isn't a number and returns the gap it leaves.
fn not_a_number(column: &mut Column, cell: &str) -> f64 {
    if column.not_numbers == 0 {
        column.not_number_example = short(cell);
    }
    column.not_numbers += 1;
    f64::NAN
}

/// Whether a cell is how Microsoft's C runtime writes a value that isn't a finite number:
/// `1.#QNAN`, `1.#SNAN`, `1.#IND` or `1.#INF`, perhaps signed and followed by digits.
fn is_windows_non_number(cell: &str) -> bool {
    let unsigned = cell.trim_start_matches(['-', '+']);
    let Some(rest) = unsigned.strip_prefix("1.#") else {
        return false;
    };
    let word = rest.trim_end_matches(|c: char| c.is_ascii_digit());
    ["QNAN", "SNAN", "IND", "INF"]
        .iter()
        .any(|known| word.eq_ignore_ascii_case(known))
}

/// A status cell's value: `b` and binary digits, the leftmost the most significant, so `b0101`
/// is 5; `NaN` for an empty cell; `None` for anything else, or more digits than an `f64` holds
/// exactly.
fn status_cell(cell: &str) -> Option<f64> {
    if cell.is_empty() {
        return Some(f64::NAN);
    }
    let digits = cell.strip_prefix(['b', 'B'])?;
    if digits.is_empty() || digits.len() > 52 || !digits.bytes().all(|b| b == b'0' || b == b'1') {
        return None;
    }
    // At most 52 binary digits, so the value is below 2⁵² and exact as an f64.
    u64::from_str_radix(digits, 2).ok().map(|bits| bits as f64)
}

/// A value in SI.
fn si(scale: Scale, value: f64) -> f64 {
    match scale {
        Scale::By(factor) => value * factor,
        Scale::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
    }
}

/// The record from the clocks read: every channel, the primary track, and the notes.
fn build(clocks: Vec<Clock>, header_line: usize, mut notes: Notes) -> Result<FlightLog, LogError> {
    let mut log = FlightLog {
        format: LogFormat::RavenFipCsv,
        logger: "Featherweight Raven".to_owned(),
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
        notes: Vec::new(),
    };
    for clock in clocks {
        if clock.repeats > 0 {
            notes.push(format!(
                "the clock {:?} repeats the time before it {} times; the repeats were dropped, \
                 {} of them differing from the sample they repeat",
                short(&clock.name),
                clock.repeats,
                clock.differing_repeats
            ));
        }
        for column in clock.columns {
            if column.not_numbers > 0 {
                notes.push(format!(
                    "the column {:?} holds {} cells that aren't a number, such as {:?}: they are \
                     gaps",
                    short(&column.name),
                    column.not_numbers,
                    column.not_number_example
                ));
            }
            if column.empty > 0 {
                notes.push(format!(
                    "the column {:?} leaves {} samples' values empty: they are gaps",
                    short(&column.name),
                    column.empty
                ));
            }
            log.channels.push(Channel {
                column: column.name,
                quantity: column.quantity,
                time_s: clock.time_s.clone(),
                values: column.values,
            });
            if let Some(status) = column.status {
                log.channels.push(Channel {
                    column: "bILBA".to_owned(),
                    quantity: Quantity::Number,
                    time_s: clock.time_s.clone(),
                    values: status,
                });
            }
        }
    }

    let primary = log
        .channels
        .iter()
        .find(|channel| key(&channel.column) == BARO_AGL)
        .or_else(|| {
            let bare = log
                .channels
                .iter()
                .find(|channel| key(&channel.column) == BARE_AGL);
            if let Some(channel) = bare {
                log.notes.push(format!(
                    "the file has no `[Altitude (Baro-Ft-AGL)]`; its {:?}, which names no sensor, \
                     was taken as the altitude above the pad",
                    channel.column
                ));
            }
            bare
        });
    let Some(primary) = primary else {
        return Err(syntax(
            header_line,
            "the file has no altitude above the pad, `[Altitude (Baro-Ft-AGL)]` or `Altitude \
             (Ft-AGL)`"
                .to_owned(),
        ));
    };
    if primary.time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    log.time_s.clone_from(&primary.time_s);
    log.altitude_m.clone_from(&primary.values);
    // The temperature and battery voltage fill the primary track only on the altitude's clock.
    let on_primary_clock = |channel: &&Channel| channel.time_s == log.time_s;
    let temperature = log
        .channels_of(Quantity::Temperature)
        .find(on_primary_clock)
        .map(|channel| channel.values.clone());
    let battery = log
        .channels_of(Quantity::Voltage)
        .filter(|channel| key(&channel.column).contains("battery"))
        .find(on_primary_clock)
        .map(|channel| channel.values.clone());
    log.temperature_k = temperature;
    log.battery_v = battery;

    log.notes.extend(notes.kept);
    if notes.left_out > 0 {
        log.notes.push(format!(
            "and {} more notes, left out so that a broken file's list stays short",
            notes.left_out
        ));
    }
    Ok(log)
}

/// The most notes a log keeps; past it, one more note counts the rest.
const MAX_NOTES: usize = 20;

/// Notes as the reader meets them: the first [`MAX_NOTES`] kept, the rest only counted, so a
/// header of many unknown columns can't fill memory with them.
#[derive(Default)]
struct Notes {
    kept: Vec<String>,
    left_out: usize,
}

impl Notes {
    fn push(&mut self, note: String) {
        if self.kept.len() < MAX_NOTES {
            self.kept.push(note);
        } else {
            self.left_out += 1;
        }
    }
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// Text from the file as an error or note quotes it: cut to [`QUOTED_CHARS`] characters, so a
/// binary file's "first line" doesn't fill the message.
fn short(text: &str) -> String {
    if text.chars().count() <= QUOTED_CHARS {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(QUOTED_CHARS - 1).collect();
        cut.push('…');
        cut
    }
}

/// The most characters of the file an error or note quotes.
const QUOTED_CHARS: usize = 80;

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,\
                          Time@Baro (Atm),Baro (Atm),bILBA,\
                          Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)],bILBA";

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    /// Each channel on its own clock, in SI, the statuses read as binary numbers, the primary
    /// track the barometric altitude's.
    #[test]
    fn reads_each_channel_on_its_own_clock() {
        let text = format!(
            "{HEADER}\r\n\
             0,1,b0000,0.01,1,b0000,0.01,0,b0000\r\n\
             0.0025,2,b0101,0.06,0.5,b1100,0.06,100,b1100\r\n"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::RavenFipCsv);
        assert_eq!(log.logger, "Featherweight Raven");
        let columns: Vec<&str> = log.channels.iter().map(|c| c.column.as_str()).collect();
        assert_eq!(
            columns,
            [
                "Axial Accel (Gs)",
                "bILBA",
                "Baro (Atm)",
                "bILBA",
                "[Altitude (Baro-Ft-AGL)]",
                "bILBA"
            ]
        );
        assert_eq!(log.channels[0].time_s, [0.0, 0.0025]);
        assert_eq!(log.channels[0].values, [9.806_65, 2.0 * 9.806_65]);
        assert_eq!(log.channels[1].values, [0.0, 5.0]);
        assert_eq!(log.channels[2].values, [101_325.0, 50_662.5]);
        assert_eq!(log.channels[3].values, [0.0, 12.0]);
        assert_eq!(log.time_s, [0.01, 0.06]);
        assert_eq!(log.altitude_m, [0.0, 30.48]);
        assert!(log.vertical_speed_m_s.is_none());
        assert!(log.events.is_empty());
        assert!(log.notes.is_empty(), "{:?}", log.notes);
    }

    /// A channel with no sample in a row is two empty cells without its status, and the channels
    /// after it move one cell left; the row must end where its channels do.
    #[test]
    fn a_stopped_channel_drops_its_status_cell() {
        let text = format!(
            "{HEADER}\n\
             0,1,b0000,0.01,1,b0000,0.01,0,b0000\n\
             0.0025,1,b0000,,,0.06,10,b1000\n\
             0.005,1,b0000,,,,\n\
             ,,,,,\n"
        );
        let log = read(&text).unwrap();
        assert_eq!(log.channels[0].time_s, [0.0, 0.0025, 0.005]);
        assert_eq!(log.channels[2].time_s, [0.01]);
        assert_eq!(log.time_s, [0.01, 0.06]);
        assert!(close(log.altitude_m[1], 3.048));
        assert_eq!(log.channels[5].values, [0.0, 8.0]);

        // The same row with the status cell kept is one cell too long.
        let text = format!("{HEADER}\n0.0025,1,b0000,,,,0.06,10,b1000\n");
        let error = read(&text).unwrap_err().to_string();
        assert!(error.contains("line 2:"), "{error}");
    }

    /// Without status columns every row is as wide as the header; a stopped channel is two empty
    /// cells in place.
    #[test]
    fn without_status_columns_rows_keep_their_width() {
        let text = "Time@Axial Accel (Gs),Axial Accel (Gs),Time@Temperature (F),Temperature (F),\
                    Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)]\n\
                    0,1,0.01,32,0.01,0\n\
                    0.0025,1,,,0.06,1\n";
        let log = read(text).unwrap();
        assert_eq!(log.channels.len(), 3);
        assert_eq!(log.channels[1].values, [273.15]);
        assert!(log.temperature_k.is_none(), "not on the altitude's clock");
        assert_eq!(log.time_s, [0.01, 0.06]);
    }

    /// One clock for every channel: the temperature and battery share the altitude's clock and
    /// fill the primary track; a repeated row is dropped with a note.
    #[test]
    fn one_clock_layout_fills_the_track() {
        let text = "Time,LiftOff,Axial Accel (Gs),Temperature (F),Volts Battery (V),\
                    Volts Pyro Main (V),[Altitude (Baro-Ft-AGL)],[Velocity (Accel-MPH)]\n\
                    0,0,1,212,8.1,0.1,0,0\n\
                    0.001,0,1,212,8.1,0.1,0,0\n\
                    0.001,0,1,212,8.1,0.1,0,0\n\
                    0.002,1,5,212,8.0,0.1,1,10\n";
        let log = read(text).unwrap();
        assert_eq!(log.time_s, [0.0, 0.001, 0.002]);
        let temperature = log.temperature_k.as_deref().unwrap();
        assert!(close(temperature[0], 373.15));
        assert_eq!(log.battery_v.as_deref(), Some(&[8.1, 8.1, 8.0][..]));
        assert_eq!(log.channels[0].quantity, Quantity::Number);
        assert!(close(log.channels[6].values[2], 4.4704));
        assert_eq!(
            log.notes,
            [
                "the clock \"Time\" repeats the time before it 1 times; the repeats were dropped, 0 \
              of them differing from the sample they repeat"
            ]
        );
    }

    /// The two-channel layout's spaced header and bare `Altitude (Ft-AGL)`, taken with a note.
    #[test]
    fn the_bare_altitude_is_taken_with_a_note() {
        let text = "Time-Axial Accel (Gs), Axial Accel (Gs), bILBA, Time (s), Altitude (Ft-AGL), \
                    bILBA\n0, 1, b0000, 0, 10, b1000\n";
        let log = read(text).unwrap();
        assert_eq!(log.altitude_m, [3.048]);
        assert!(log.notes[0].contains("names no sensor"), "{:?}", log.notes);
    }

    /// Windows' `-1.#QNAN` and an empty value are gaps, counted; other junk is refused.
    #[test]
    fn windows_non_numbers_are_gaps() {
        let text = format!(
            "{HEADER}\n0,-1.#QNAN,b0000,0.01,,b0000,0.01,0,b0000\n\
             0.0025,1.#INF,b0000,0.06,1,b0000,0.06,0,b0000\n"
        );
        let log = read(&text).unwrap();
        assert!(log.channels[0].values.iter().all(|v| v.is_nan()));
        assert!(log.channels[2].values[0].is_nan());
        assert_eq!(log.notes.len(), 2, "{:?}", log.notes);
        assert!(log.notes[0].contains("2 cells that aren't a number, such as \"-1.#QNAN\""));
        assert!(log.notes[1].contains("leaves 1 samples' values empty"));
        assert!(is_windows_non_number("-1.#IND00"));
        assert!(!is_windows_non_number("1.#X"));
        let error = read(&format!("{HEADER}\n0,x,b0000,,,,\n")).unwrap_err();
        assert!(
            error.to_string().contains("line 2: \"x\" in cell 2"),
            "{error}"
        );
    }

    /// The status digits: the leftmost the most significant; anything but `b` and binary digits
    /// refused.
    #[test]
    fn status_cells_are_binary_numbers() {
        assert_eq!(status_cell("b0101"), Some(5.0));
        assert_eq!(status_cell("b1000"), Some(8.0));
        assert_eq!(status_cell("b1111"), Some(15.0));
        assert!(status_cell("").unwrap().is_nan());
        for bad in ["0101", "b", "b012", "bx", &format!("b{}", "1".repeat(53))] {
            assert_eq!(status_cell(bad), None, "{bad}");
        }
    }

    /// Each refusal names its line and what is wrong.
    #[test]
    fn malformed_files_are_refused_where_they_go_wrong() {
        let cases = [
            (String::new(), "the file is empty"),
            (
                "Time,Altitude\n0,0\n".to_owned(),
                "isn't a FIP export's header",
            ),
            (
                "Axial Accel (Gs),Time\n".to_owned(),
                "isn't a FIP export's header",
            ),
            (format!("{HEADER}\n"), "no data rows"),
            (
                "Time@Axial Accel (Gs),Axial Accel (Gs)\n0,1\n".to_owned(),
                "line 1: the file has no altitude above the pad",
            ),
            (
                "Time@x,Time@Axial Accel (Gs),Axial Accel (Gs)\n".to_owned(),
                "line 1: the clock \"Time@x\" has no channel",
            ),
            (
                "Time@Axial Accel (Gs),bILBA,Axial Accel (Gs)\n".to_owned(),
                "line 1: the status column 2 follows no channel",
            ),
            (
                "Time@Axial Accel (Gs),Axial Accel (Gs),,x\n".to_owned(),
                "line 1: the header's column 3 has no name",
            ),
            (
                format!("{HEADER}\n0,1,b0,0.5,1,b0,0.5,0,b0\n0,1,b0,0.4,1,b0,0.6,0,b0\n"),
                "line 3: the time 0.4 s on the clock \"Time@Baro (Atm)\" comes before",
            ),
            (
                format!("{HEADER}\nx,1,b0,,,,\n"),
                "line 2: the time \"x\" in cell 1",
            ),
            (
                format!("{HEADER}\n0,1,b0,0,1,b0,0,1,b2\n"),
                "line 2: the status \"b2\" in cell 9",
            ),
            (
                format!("{HEADER}\n0,1,b0,0,1\n"),
                "line 2: the row ends at cell 5",
            ),
            (
                format!("{HEADER}\n0,1,b0,,,0.1,1,b0,7\n"),
                "line 2: the row has 9 cells and its channels' samples take 8",
            ),
            (format!("{HEADER}\n0,1,b0,,,,\n"), "no data rows"),
        ];
        for (text, expected) in cases {
            let error = read(&text).unwrap_err().to_string();
            assert!(error.contains(expected), "{text:?}: {error}");
        }
    }

    /// A byte-order mark is dropped; CR-only line ends read as lines; blank lines are skipped.
    #[test]
    fn marks_and_line_ends() {
        let text = format!("\u{feff}\r{HEADER}\r0,1,b0,0,1,b0,0,1,b0\r\r");
        let log = read(&text).unwrap();
        assert_eq!(log.altitude_m, [FOOT_M]);
        let lines: Vec<(usize, &str)> = numbered_lines("a\r\nb\rc\n\nd").collect();
        assert_eq!(lines, [(1, "a"), (2, "b"), (3, "c"), (4, ""), (5, "d")]);
    }

    /// A header of many unknown columns keeps a short list of notes and counts the rest.
    #[test]
    fn notes_stay_few() {
        let unknown: String = (0..100).map(|i| format!(",u{i}")).collect();
        let text = format!("Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)]{unknown}\n");
        let row = format!("0{}\n", ",1".repeat(101));
        let log = read(&format!("{text}{row}")).unwrap();
        assert_eq!(log.notes.len(), MAX_NOTES + 1);
        assert!(
            log.notes[MAX_NOTES].starts_with("and 80 more notes"),
            "{:?}",
            log.notes
        );
        assert_eq!(log.channels.len(), 101);
        assert_eq!(log.channels[100].quantity, Quantity::Unrecognized);
    }

    /// A file that isn't text is refused with a short quote of it.
    #[test]
    fn a_long_line_is_quoted_short() {
        let junk = "\u{fffd}".repeat(100_000);
        let error = read(&junk).unwrap_err().to_string();
        assert!(error.len() < 400, "{} bytes", error.len());
    }

    proptest::proptest! {
        /// Any text is read or refused, never a panic; what is read has increasing times and an
        /// altitude for each.
        #[test]
        fn any_text_is_read_or_refused(rows in proptest::collection::vec("[-0-9.b,x#QNA]{0,24}", 0..10)) {
            let text = format!("{HEADER}\n{}", rows.join("\n"));
            if let Ok(log) = read(&text) {
                proptest::prop_assert!(log.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                proptest::prop_assert_eq!(log.time_s.len(), log.altitude_m.len());
                for channel in &log.channels {
                    proptest::prop_assert!(channel.time_s.windows(2).all(|pair| pair[1] > pair[0]));
                    proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                }
            }
        }
    }
}
