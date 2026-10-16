//! AltOS's radio telemetry decoded to a table: one row a packet, under a header naming the
//! packet's fields, `serial,tick,ptype,state,v_batt,…,height,crc`.
//!
//! An Altus Metrum flight computer's radio packet opens with the board's serial number, its clock
//! in ticks of 10 ms and the packet's type, and the type lays out the rest ([`crate::altos_telem`];
//! *AltOS Telemetry*, Bdale Garbee and Keith Packard, v1.9.22.2, 2025, §2.1, p. 2). A TeleMega
//! sends its sensors in two packets, and type `0x09`, "TeleMega Kalman and Voltage Data for 15V
//! boards", holds the flight's `state`, `v_batt`, `v_pyro`, the six pyro channels' continuity
//! `sense`, `ground_pres`, `ground_accel`, `accel_plus_g`, `accel_minus_g`, `acceleration` in
//! sixteenths of a m/s², `speed` in sixteenths of a m/s and `height` in meters (§2.3, Tables 4
//! and 6, pp. 3–4). One sample file is such packets decoded to a CSV, by software it doesn't
//! name: its header names a row's serial number, its tick, its packet type and a `crc` flag,
//! then the packet's fields by the document's names, the scales already applied:
//!
//! ```text
//! serial,tick,ptype,state,v_batt,v_pyro,sense,v_apogee,v_main,ground_pres,ground_accel,accel_plus_g,accel_minus_g,acceleration,speed,height,crc
//! 1234,402.94,9,3,4.1,4.19,0.00 0.00 0.00 4.20,4.2,4.14,98500,-28,-29,10,62.25,27.375,6,TRUE
//! ```
//!
//! **Recognition.** The header, after any byte-order mark, opens `serial,tick,ptype,`.
//!
//! **Reading by the header.** Each column is read by its name:
//!
//! | Columns | Read as | Why |
//! |---|---|---|
//! | `tick` | the clock, s, as written | the table writes the tick in seconds, to the hundredth, the tick's 10 ms |
//! | `height` | [`Quantity::AltitudeAboveGround`], m, and the primary track | the document's meters above the pad: the board's own estimate |
//! | `acceleration` | [`Quantity::Acceleration`], m/s² | the document's sixteenths of a m/s², divided out: the board's estimate along its axis, about 0 at rest |
//! | `speed` | [`Quantity::Speed`], m/s | the document's sixteenths of a m/s, divided out: the board's estimate, which leans on its accelerometer |
//! | `ground_pres` | [`Quantity::Pressure`], Pa, where the column's first value lies between 10 and 120 kPa, as every launch site's pad does; as written otherwise | the document's barometer reading on the ground |
//! | `v_batt`, `v_pyro`, `v_apogee`, `v_main`, each number of `sense` | [`Quantity::Voltage`], V | the document sends the converter's counts, but the table holds volts: 4.1 for the battery, a single LiPo cell's, the battery a TeleMega takes (*The Altus Metrum System*, v1.9.22.2, §3.2, p. 6); only `v_batt` is held to a band, by the survey |
//! | `serial`, `ptype`, `state` | [`Quantity::Number`], as written | the board's serial number, the packet's type and the flight's state |
//! | `ground_accel`, `accel_plus_g`, `accel_minus_g` | [`Quantity::Unrecognized`], as written | the accelerometer's counts, as the telemetry reader keeps them |
//!
//! Every other column is kept as written, [`Quantity::Unrecognized`], with a note. `sense` holds
//! several numbers in one cell, by spaces, one a pyro channel: each is its own channel, `sense 1`
//! and on, and every row must hold as many as the first. A TeleMega has six pyro channels, four of
//! them programmable (*The Altus Metrum System*, §1, p. 3), and the sample file's `sense` holds
//! four, beside `v_apogee` and `v_main`.
//!
//! **The flight's states.** Each change of `state` is an [`Event`], named as AltosUI's exports
//! name the state's number ([`crate::altos_telem`]): 2 `pad`, 3 `boost`, 4 `fast`, 5 `coast`, 6
//! `drogue`, 7 `main` and 8 `landed`; another number is `state N`.
//!
//! **What is left out, and refused.** A row whose `crc` is `FALSE`, a packet whose check failed,
//! is left out and counted in a note: its bytes are noise. A `crc` that is neither `TRUE` nor
//! `FALSE`, a row with another number of cells than the header, a row whose serial number or packet
//! type isn't the first row's (two boards, or one table holding two packets' fields), and a tick
//! no later than the one before refuse the file at the row's line. The board's tick turns over
//! every 655.36 s (§2.1), and a table of seconds can't tell a turn from a board switched off and
//! on, so this reader doesn't guess. A last line the file ends in with no line end, which isn't a
//! whole row, is dropped with a note. A cell that is empty or not a finite number is a gap, `NaN`,
//! counted per column in a note.
//!
//! **The primary track.** [`FlightLog::altitude_m`] is `height`, the board's own estimate of its
//! height above the pad, on the rows' ticks; [`FlightLog::battery_v`] is `v_batt`. The vertical
//! speed is absent: `speed` leans on the accelerometer, as for [AltOS's CSV](crate::altos).
//!
//! The document is published under CC BY-SA 3.0; this reader restates its facts, not its tables.

use crate::altos_telem::state_name;
use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};

/// The format's name in errors.
const FORMAT: &str = "AltOS telemetry CSV";

/// What the header opens with.
const HEADER_START: &str = "serial,tick,ptype,";

/// The most columns a header may name.
pub const MAX_COLUMNS: usize = 256;

/// The most numbers a `sense` cell may hold.
pub const MAX_SENSES: usize = 16;

/// The pressures a launch site's pad can have, Pa.
const PAD_PRESSURE_PA: std::ops::RangeInclusive<f64> = 10_000.0..=120_000.0;

/// The most characters of the file an error or note quotes.
const QUOTED_CHARS: usize = 80;

/// What a column is to this reader.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    /// `tick`: the clock.
    Clock,
    /// `crc`: whether the row's packet passed its check.
    Crc,
    /// `sense`: several voltages in one cell.
    Senses,
    /// `ground_pres`: a pressure, Pa where the pad's lies in [`PAD_PRESSURE_PA`].
    Pressure,
    /// A channel of this quantity, as written.
    Value(Quantity),
}

/// A column's role from its name, and whether the reader knows the name.
fn role(name: &str) -> (Role, bool) {
    use Quantity as Q;
    let value = |quantity| (Role::Value(quantity), true);
    match name {
        "tick" => (Role::Clock, true),
        "crc" => (Role::Crc, true),
        "sense" => (Role::Senses, true),
        "ground_pres" => (Role::Pressure, true),
        "height" => value(Q::AltitudeAboveGround),
        "acceleration" => value(Q::Acceleration),
        "speed" => value(Q::Speed),
        "v_batt" | "v_pyro" | "v_apogee" | "v_main" => value(Q::Voltage),
        "serial" | "ptype" | "state" => value(Q::Number),
        "ground_accel" | "accel_plus_g" | "accel_minus_g" => value(Q::Unrecognized),
        _ => (Role::Value(Q::Unrecognized), false),
    }
}

/// Whether a text is AltOS's telemetry decoded to a table: its header, after any byte-order
/// mark, opens `serial,tick,ptype,`.
pub fn recognizes(text: &str) -> bool {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    text.split(['\r', '\n'])
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with(HEADER_START))
}

fn syntax(line: usize, message: String) -> LogError {
    LogError::Syntax {
        format: FORMAT,
        line,
        message,
    }
}

/// A cell's value: a finite number, or `None`.
fn number_in(cell: &str) -> Option<f64> {
    cell.parse::<f64>().ok().filter(|value| value.is_finite())
}

/// Reads AltOS's telemetry decoded to a table, from its text.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the header doesn't open `serial,tick,ptype,`;
/// [`LogError::Syntax`] for a header naming a column twice, more than [`MAX_COLUMNS`] or no
/// `height` or `crc`, and for a row that the module's docs say refuses the file;
/// [`LogError::NoData`] if no row passed its check.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the header doesn't open `serial,tick,ptype,`".to_owned(),
        });
    }
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let ends_unended = !text.ends_with(['\r', '\n']);
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = text
        .split('\n')
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .peekable();
    // `recognizes` has found the header.
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NoData { format: FORMAT });
    };
    let names: Vec<&str> = header.split(',').map(str::trim).collect();
    if names.len() > MAX_COLUMNS {
        return Err(syntax(
            header_line,
            format!("the header names more than {MAX_COLUMNS} columns"),
        ));
    }
    for (index, name) in names.iter().enumerate() {
        if names[..index].contains(name) {
            return Err(syntax(
                header_line,
                format!("the header names `{}` twice", short(name)),
            ));
        }
    }
    let find = |wanted: &str| names.iter().position(|name| *name == wanted);
    for required in ["height", "crc"] {
        if find(required).is_none() {
            return Err(syntax(
                header_line,
                format!("the header names no `{required}` column"),
            ));
        }
    }
    let roles: Vec<(Role, bool)> = names.iter().map(|name| role(name)).collect();
    // `recognizes` has found `tick` and `ptype`, and `serial` before them.
    let (clock, ptype, serial) = (find("tick"), find("ptype"), find("serial"));

    let mut time_s: Vec<f64> = Vec::new();
    let mut values: Vec<Vec<f64>> = vec![Vec::new(); names.len()];
    let mut senses: Vec<Vec<f64>> = Vec::new();
    let mut gaps = vec![0_usize; names.len()];
    let mut failed = 0;
    let mut first: Option<(String, String)> = None;
    let mut dropped_last = false;
    while let Some((number, line)) = lines.next() {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() != names.len() {
            if ends_unended && lines.peek().is_none() && cells.len() < names.len() {
                dropped_last = true;
                continue;
            }
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells and the header names {}",
                    cells.len(),
                    names.len()
                ),
            ));
        }
        let cell = |index: Option<usize>| index.map_or("", |index| cells[index]);
        // A packet that failed its check is noise: left out before its serial number or type
        // can stand for the table's.
        let passed = match cell(find("crc")) {
            "TRUE" => true,
            "FALSE" => false,
            other => {
                return Err(syntax(
                    number,
                    format!("the `crc` cell is {:?}, not TRUE or FALSE", short(other)),
                ));
            }
        };
        if !passed {
            failed += 1;
            continue;
        }
        let identity = (cell(serial).to_owned(), cell(ptype).to_owned());
        match &first {
            None => first = Some(identity),
            Some(stated) if *stated != identity => {
                return Err(syntax(
                    number,
                    format!(
                        "the row's serial number and packet type, {:?} and {:?}, aren't the \
                         first row's, {:?} and {:?}: one table holds one board's packets of one \
                         type",
                        short(&identity.0),
                        short(&identity.1),
                        short(&stated.0),
                        short(&stated.1)
                    ),
                ));
            }
            Some(_) => {}
        }
        let Some(time) = number_in(cell(clock)) else {
            return Err(syntax(
                number,
                format!("the tick {:?} isn't a number", short(cell(clock))),
            ));
        };
        if let Some(&before) = time_s.last()
            && time <= before
        {
            return Err(syntax(
                number,
                format!(
                    "the tick {time} s isn't later than the one before, {before} s: the board's \
                     tick turns over every 655.36 s, and a table of seconds can't tell a turn \
                     from a board switched off and on"
                ),
            ));
        }
        time_s.push(time);
        for (index, (role, _)) in roles.iter().enumerate() {
            match role {
                Role::Clock | Role::Crc => {}
                Role::Senses => {
                    let parts: Vec<&str> = cells[index].split_whitespace().collect();
                    if senses.is_empty() {
                        if parts.is_empty() || parts.len() > MAX_SENSES {
                            return Err(syntax(
                                number,
                                format!(
                                    "the `sense` cell holds {} numbers: from 1 to {MAX_SENSES}",
                                    parts.len()
                                ),
                            ));
                        }
                        senses = vec![Vec::new(); parts.len()];
                    }
                    if parts.len() != senses.len() {
                        return Err(syntax(
                            number,
                            format!(
                                "the `sense` cell holds {} numbers and the first row's {}",
                                parts.len(),
                                senses.len()
                            ),
                        ));
                    }
                    for (channel, part) in senses.iter_mut().zip(parts) {
                        channel.push(number_in(part).unwrap_or_else(|| {
                            gaps[index] += 1;
                            f64::NAN
                        }));
                    }
                }
                Role::Pressure | Role::Value(_) => {
                    values[index].push(number_in(cells[index]).unwrap_or_else(|| {
                        gaps[index] += 1;
                        f64::NAN
                    }));
                }
            }
        }
    }
    if time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes = Vec::new();
    if failed > 0 {
        notes.push(format!(
            "{failed} {} whose `crc` is FALSE, a packet that failed its check, left out",
            if failed == 1 { "row" } else { "rows" }
        ));
    }
    let mut channels = Vec::new();
    for (index, (role, known)) in roles.iter().enumerate() {
        let name = names[index];
        let quantity = match role {
            Role::Clock | Role::Crc => continue,
            Role::Senses => {
                for (place, values) in std::mem::take(&mut senses).into_iter().enumerate() {
                    channels.push(Channel {
                        column: format!("sense {}", place + 1),
                        quantity: Quantity::Voltage,
                        time_s: time_s.clone(),
                        values,
                    });
                }
                if gaps[index] > 0 {
                    notes.push(format!(
                        "the `sense` column has {} numbers that aren't, read as gaps",
                        gaps[index]
                    ));
                }
                continue;
            }
            Role::Pressure => {
                let pad = values[index].first().copied().unwrap_or(f64::NAN);
                if PAD_PRESSURE_PA.contains(&pad) {
                    Quantity::Pressure
                } else {
                    notes.push(format!(
                        "`{name}`'s first value isn't a pad's pressure in Pa; it is kept as written"
                    ));
                    Quantity::Unrecognized
                }
            }
            Role::Value(quantity) => *quantity,
        };
        if !known {
            notes.push(format!(
                "the column `{}` isn't one of the packet's fields this reader knows; its values \
                 are kept as written",
                short(name)
            ));
        }
        if gaps[index] > 0 {
            notes.push(format!(
                "the `{}` column has {} empty or not a number, read as gaps",
                short(name),
                gaps[index]
            ));
        }
        channels.push(Channel {
            column: name.to_owned(),
            quantity,
            time_s: time_s.clone(),
            values: std::mem::take(&mut values[index]),
        });
    }
    let channel_values = |column: &str| {
        channels
            .iter()
            .find(|channel| channel.column == column)
            .map(|channel| channel.values.clone())
    };
    // `read` has refused a header without `height`.
    let altitude_m = channel_values("height").unwrap_or_default();
    let battery_v = channel_values("v_batt");
    let mut events = Vec::new();
    if let Some(states) = channel_values("state") {
        let mut last = None;
        for (&time, &state) in time_s.iter().zip(&states) {
            if state.is_finite() && last != Some(state) {
                // A whole number from 0 to 255 is a state's byte; another number is quoted short,
                // so a file of huge ones can't make long names.
                let name = (0..=u8::MAX)
                    .find(|&byte| f64::from(byte) == state)
                    .map_or_else(
                        || format!("state {}", short(&state.to_string())),
                        state_name,
                    );
                events.push(Event { time_s: time, name });
                last = Some(state);
            }
        }
    }
    notes.push(
        "`acceleration` is the board's own estimate along its axis with gravity taken out: about \
         0 at rest, not 1 g; `speed` leans on the accelerometer, so the record has no vertical \
         speed"
            .to_owned(),
    );
    if dropped_last {
        notes.push(
            "the last line, which ends the file with no line end and isn't a whole row, dropped"
                .to_owned(),
        );
    }
    let serial_number = first
        .map(|(serial, _)| serial)
        .filter(|serial| !serial.is_empty())
        .map(|serial| short(&serial));
    Ok(FlightLog {
        format: LogFormat::AltosTelemetryCsv,
        logger: "AltOS".to_owned(),
        serial_number,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s,
        altitude_m,
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v,
        channels,
        events,
        notes,
    })
}

/// Text from the file as an error or note quotes it, cut to [`QUOTED_CHARS`] characters.
fn short(text: &str) -> String {
    if text.chars().count() <= QUOTED_CHARS {
        text.to_owned()
    } else {
        let mut cut: String = text.chars().take(QUOTED_CHARS - 1).collect();
        cut.push('…');
        cut
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "serial,tick,ptype,state,v_batt,v_pyro,sense,v_apogee,v_main,\
                          ground_pres,ground_accel,accel_plus_g,accel_minus_g,acceleration,speed,\
                          height,crc\r\n";

    /// A row of invented values.
    fn row(tick: &str, state: u8, height: i32, crc: &str) -> String {
        format!(
            "1234,{tick},9,{state},4.1,4.2,0.00 0.00 0.00 4.20,4.2,4.14,98500,-28,-29,10,\
             -0.5625,-1.25,{height},{crc}\r\n"
        )
    }

    fn table(rows: &[String]) -> String {
        format!("\u{feff}{HEADER}{}", rows.concat())
    }

    fn error_line(text: &str) -> (usize, String) {
        match read(text) {
            Err(LogError::Syntax { line, message, .. }) => (line, message),
            other => panic!("expected a syntax error, got {other:?}"),
        }
    }

    /// A packet that failed its check is noise: its serial number can't refuse the table nor
    /// stand for it.
    #[test]
    fn a_failed_packet_neither_refuses_the_table_nor_names_its_board() {
        let noisy = row("95.00", 2, 0, "FALSE").replacen("1234", "9z9", 1);
        let mut rows = rows();
        rows.insert(0, noisy);
        let log = read(&table(&rows)).unwrap();
        assert_eq!(log.serial_number.as_deref(), Some("1234"));
        assert_eq!(log.time_s.len(), 3);
    }

    /// A state that isn't a byte is named, but its number is quoted short.
    #[test]
    fn a_state_that_isnt_a_byte_is_quoted_short() {
        let huge = row("402.94", 3, 6, "TRUE").replace(",9,3,", ",9,1e300,");
        let log = read(&table(&[row("95.27", 2, 0, "TRUE"), huge])).unwrap();
        let name = &log.events[1].name;
        assert!(
            name.starts_with("state 1000") && name.chars().count() <= 6 + QUOTED_CHARS,
            "{name}"
        );
    }

    fn rows() -> Vec<String> {
        vec![
            row("95.27", 2, 0, "TRUE"),
            row("402.94", 3, 6, "TRUE"),
            row("403.05", 3, 9, "FALSE"),
            row("403.16", 4, 14, "TRUE"),
        ]
    }

    #[test]
    fn each_column_reads_by_its_name() {
        let text = table(&rows());
        assert!(recognizes(&text));
        let log = read(&text).unwrap();
        assert_eq!(log.format, LogFormat::AltosTelemetryCsv);
        assert_eq!(log.time_s, [95.27, 402.94, 403.16]);
        assert_eq!(log.altitude_m, [0.0, 6.0, 14.0]);
        assert_eq!(log.battery_v.as_deref(), Some(&[4.1, 4.1, 4.1][..]));
        assert_eq!(log.serial_number.as_deref(), Some("1234"));
        let quantity = |column: &str| {
            log.channels
                .iter()
                .find(|c| c.column == column)
                .map(|c| c.quantity)
        };
        assert_eq!(quantity("acceleration"), Some(Quantity::Acceleration));
        assert_eq!(quantity("speed"), Some(Quantity::Speed));
        assert_eq!(quantity("ground_pres"), Some(Quantity::Pressure));
        assert_eq!(quantity("v_main"), Some(Quantity::Voltage));
        assert_eq!(quantity("ground_accel"), Some(Quantity::Unrecognized));
        assert_eq!(quantity("state"), Some(Quantity::Number));
        assert_eq!(quantity("tick"), None);
        assert_eq!(quantity("crc"), None);
        let senses: Vec<&Channel> = log
            .channels
            .iter()
            .filter(|c| c.column.starts_with("sense "))
            .collect();
        assert_eq!(senses.len(), 4);
        assert_eq!(senses[3].values, [4.2, 4.2, 4.2]);
        assert!(senses.iter().all(|c| c.quantity == Quantity::Voltage));
        let events: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|e| (e.time_s, e.name.as_str()))
            .collect();
        assert_eq!(
            events,
            [(95.27, "pad"), (402.94, "boost"), (403.16, "fast")]
        );
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("1 row whose `crc` is FALSE"))
        );
    }

    #[test]
    fn a_row_of_another_board_or_packet_type_is_refused() {
        let mut other_board = rows();
        other_board[1] = other_board[1].replacen("1234", "4321", 1);
        assert_eq!(error_line(&table(&other_board)).0, 3);
        let mut other_type = rows();
        other_type[1] = other_type[1].replacen(",9,", ",10,", 1);
        let (line, message) = error_line(&table(&other_type));
        assert_eq!(line, 3);
        assert!(message.contains("one type"), "{message}");
    }

    #[test]
    fn a_tick_no_later_than_the_one_before_is_refused() {
        let mut back = rows();
        back[3] = row("10.00", 6, 20, "TRUE");
        let (line, message) = error_line(&table(&back));
        assert_eq!(line, 5);
        assert!(message.contains("655.36"), "{message}");
        // A failed row's tick doesn't count.
        let mut failed_back = rows();
        failed_back[2] = row("1.00", 3, 9, "FALSE");
        assert!(read(&table(&failed_back)).is_ok());
    }

    #[test]
    fn a_crc_or_a_row_that_isnt_one_is_refused() {
        let mut crc = rows();
        crc[1] = row("402.94", 3, 6, "yes");
        assert!(error_line(&table(&crc)).1.contains("not TRUE or FALSE"));
        let mut long = rows();
        long[1] = long[1].replace(",TRUE", ",TRUE,1");
        assert!(error_line(&table(&long)).1.contains("18 cells"));
        let mut senses = rows();
        senses[1] = senses[1].replace("0.00 0.00 0.00 4.20", "0.00 4.20");
        assert!(error_line(&table(&senses)).1.contains("first row's 4"));
        let mut tick = rows();
        tick[1] = row("x", 3, 6, "TRUE");
        assert!(error_line(&table(&tick)).1.contains("isn't a number"));
    }

    #[test]
    fn a_cut_last_row_is_dropped_only_with_no_line_end() {
        let mut text = table(&rows());
        text.push_str("1234,403.27,9,4");
        let log = read(&text).unwrap();
        assert_eq!(log.time_s.len(), 3);
        assert!(log.notes.iter().any(|n| n.contains("no line end")));
        text.push_str("\r\n");
        assert_eq!(error_line(&text).0, 6);
        // A whole last row with no line end reads, as the sample file's does.
        let whole = table(&rows()).trim_end().to_owned();
        assert_eq!(read(&whole).unwrap().time_s.len(), 3);
    }

    #[test]
    fn a_header_without_height_or_crc_or_naming_one_twice_is_refused() {
        let no_crc = table(&rows()).replace(",height,crc", ",height,check");
        assert!(error_line(&no_crc).1.contains("no `crc`"));
        let twice = table(&rows()).replace(",v_main,", ",v_batt,");
        assert!(error_line(&twice).1.contains("`v_batt` twice"));
        let wide = format!(
            "serial,tick,ptype,height,crc,{}\n",
            (0..MAX_COLUMNS)
                .map(|i| format!("c{i}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        assert!(error_line(&wide).1.contains("more than 256"));
    }

    #[test]
    fn an_unknown_column_and_a_pressure_off_any_pad_are_kept_as_written() {
        let text = table(&rows())
            .replace(",crc\r\n", ",crc,rssi\r\n")
            .replace(",TRUE\r\n", ",TRUE,-80\r\n")
            .replace(",FALSE\r\n", ",FALSE,-80\r\n")
            .replace(",98500,", ",985,");
        let log = read(&text).unwrap();
        let rssi = log.channels.iter().find(|c| c.column == "rssi").unwrap();
        assert_eq!(rssi.quantity, Quantity::Unrecognized);
        let pressure = log
            .channels
            .iter()
            .find(|c| c.column == "ground_pres")
            .unwrap();
        assert_eq!(pressure.quantity, Quantity::Unrecognized);
        assert!(log.notes.iter().any(|n| n.contains("`rssi`")));
    }
}
