//! Rocketronics' AltiMAX altimeters: a flight as the AltiMAX PC software exports it to Excel, a
//! bare `.xls` worksheet ([`xls`](crate::xls)) whose sheet reads as this table.
//!
//! The maker's manuals say only that a flight "can be exported to Excel as altitude values"
//! (*AltiMAX G4 Manual*, Rocketronics, 2021, p. 52); none describes the columns. One header row
//! names them, and the one export among the sample logs has these eleven:
//!
//! ```text
//! TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m],ACCEL [m/s2],
//! SPEED [m/s],EVENT,EVENT,VOLT. CHANNEL,VOLTAGE [V]
//! ```
//!
//! **Channels.** Each column is read into the record's SI unit ([`Quantity::unit`]) on the
//! `TIME` clock:
//!
//! | Column | Quantity | Read as |
//! |---|---|---|
//! | `TIME` | the clock | seconds, though it names no unit: held below |
//! | `PRESS FILTER [hpa]`, `PRESS RAW [hpa]` | pressure | the unit its values show, held below |
//! | `HEIGHT FILTER [m]`, `HEIGHT RAW [m]` | altitude above the pad | m: the altimeter sets its sensors "to "0"" when armed (*AltiMAX G3 User Manual*, p. 10) |
//! | `ACCEL [m/s2]` | acceleration | m/s², as written: below |
//! | `SPEED [m/s]` | speed | m/s, as written: below |
//! | `EVENT`, numbers | a number, as written | 0 on a row with no event |
//! | `EVENT`, text | the flight's events | below |
//! | `VOLT. CHANNEL` | a number, as written | which voltage the row's `VOLTAGE [V]` is |
//! | `VOLTAGE [V]` | one voltage channel for each `VOLT. CHANNEL` number, on its own rows | V |
//!
//! A column the reader doesn't know is kept as written, as [`Quantity::Unrecognized`], with a
//! note.
//!
//! **The primary track** is `HEIGHT RAW [m]`, the height from the barometer's own reading; the
//! filtered height is a channel, as an Eggtimer's filtered altitude is.
//!
//! **The accelerometer's columns, as the sample shows them.** No manual describes them. `ACCEL`
//! is measured along an axis pointing down the rocket: in the sample it reads −0.94 g before
//! liftoff and below 0 through the boost. `SPEED` is `ACCEL` summed over the clock, gravity left
//! in: over the sample's boost its change is `ACCEL`'s to within 0.2 %. So `SPEED` isn't the
//! rocket's speed, and the record's vertical speed is left empty; both columns are kept, signs
//! and all, as channels.
//!
//! **Units the file doesn't state, or states wrongly, held to physics.** `TIME` names no unit. The
//! altimeter stores 200 samples a second until apogee and fewer after it (*AltiMAX G4 Manual*,
//! p. 8 and p. 10), so the clock is read in seconds when its shortest step is 5 ms, within
//! 10 %; otherwise the file is refused. The pressure columns say `[hpa]`, but the sample's hold
//! pascals: six-digit values whose heights follow them in pascals, not in hectopascals. Each
//! pressure column is read in the unit that puts its first value, on the pad, between 500 and
//! 1,100 hPa: hPa as stated, or Pa, with a note. The two ranges don't overlap, so only one can;
//! a value in neither is refused.
//!
//! **Events.** The two `EVENT` columns are a code and its name. A row whose code isn't 0 is an
//! event, named as the text column writes it, such as `BARO-LIFTOFF` or `APOGEE PRESSURE`; the
//! sample writes `PAD` beside every code 0. The names follow the manual's list of events
//! (*AltiMAX G3 User Manual*, p. 15), in the software's own spelling; the codes aren't documented,
//! so they are kept as written.
//!
//! **Voltages.** `VOLT. CHANNEL` cycles through 1 to 5, a row each, and `VOLTAGE [V]` is that
//! channel's voltage: the reader gives each number its own voltage channel, `VOLTAGE [V] (channel
//! 1)` and so on. The software lists five voltages, the battery's, the igniter's and three pyro
//! channels' (*AltiMAX G4 Manual*, p. 42), but not which number is which, so none is named.
//!
//! **Cells.** A row holds as many cells as the header; an empty value is a gap, `NaN`, counted in
//! a note. The clock must increase from row to row, and a row without a time is refused.
//!
//! ```
//! use hpr_flightdata::altimax;
//!
//! // An invented export: three rows, the second an event.
//! let text = "TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m],\
//!             ACCEL [m/s2],SPEED [m/s],EVENT,EVENT,VOLT. CHANNEL,VOLTAGE [V]\n\
//!             0.005,98000,0,98000,0,-9.8,0,0,PAD,1,7.4\n\
//!             0.01,97990,0.8,97989,0.9,40,1,3,BARO-LIFTOFF,2,7.3\n\
//!             0.015,97975,2.1,97974,2.2,60,2,0,PAD,3,0.1\n";
//! let log = altimax::read(text)?;
//! assert_eq!(log.altitude_m, [0.0, 0.9, 2.2]);
//! assert_eq!(log.events[0].name, "BARO-LIFTOFF");
//! assert_eq!(log.events[0].time_s, 0.01);
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

use crate::error::LogError;
use crate::log::{Channel, Event, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{Notes, finite, numbered_lines, short};

const FORMAT: &str = "AltiMAX Excel export";

/// The first five columns, by which the layout is told.
const MARKS: [&str; 5] = [
    "TIME",
    "PRESS FILTER [hpa]",
    "HEIGHT FILTER [m]",
    "PRESS RAW [hpa]",
    "HEIGHT RAW [m]",
];

const TIME: &str = "time";
const HEIGHT_RAW: &str = "height raw [m]";
const EVENT: &str = "event";
const VOLT_CHANNEL: &str = "volt. channel";
const VOLTAGE: &str = "voltage [v]";

/// The clock's shortest step, s: 200 samples a second (*AltiMAX G4 Manual*, p. 8).
const STEP_S: f64 = 0.005;

/// A pad's pressure, Pa: 500 to 1,100 hPa, the band the other readers hold a pressure to.
const PAD_PA: std::ops::RangeInclusive<f64> = 50_000.0..=110_000.0;

/// Whether `text` is an AltiMAX export: its first line that isn't blank opens with the five
/// columns `TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m]`, in any
/// case.
pub fn recognizes(text: &str) -> bool {
    numbered_lines(text)
        .map(|(_, line)| line)
        .find(|line| !line.trim().is_empty())
        .is_some_and(|header| {
            let names: Vec<&str> = header.split(',').map(str::trim).collect();
            names.len() >= MARKS.len()
                && MARKS
                    .iter()
                    .zip(&names)
                    .all(|(mark, name)| mark.eq_ignore_ascii_case(name))
        })
}

/// What a column is read as.
#[derive(Clone, Copy, PartialEq)]
enum Role {
    Time,
    /// A pressure, its unit settled by its first value.
    Pressure,
    Value(Quantity, f64),
    /// The event's name, text.
    EventName,
    /// A voltage, split by the voltage channel's numbers.
    Voltage,
}

fn role(name: &str) -> Role {
    match name.trim().to_ascii_lowercase().as_str() {
        TIME => Role::Time,
        "press filter [hpa]" | "press raw [hpa]" => Role::Pressure,
        "height filter [m]" | HEIGHT_RAW => Role::Value(Quantity::AltitudeAboveGround, 1.0),
        "accel [m/s2]" => Role::Value(Quantity::Acceleration, 1.0),
        "speed [m/s]" => Role::Value(Quantity::Speed, 1.0),
        EVENT | VOLT_CHANNEL => Role::Value(Quantity::Number, 1.0),
        VOLTAGE => Role::Voltage,
        _ => Role::Value(Quantity::Unrecognized, 1.0),
    }
}

/// Reads an AltiMAX export's table.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] when the first line isn't the export's header ([`recognizes`]);
/// [`LogError::Syntax`] when a row's cells aren't as many as the header's, a time is missing, isn't
/// a number or doesn't increase, or a value isn't a number; [`LogError::Unit`] when the clock's
/// shortest step isn't 5 ms or a pressure's first value is a pad's in neither hPa nor Pa;
/// [`LogError::NoData`] when no row follows the header.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    if !recognizes(text) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: format!(
                "its first line isn't an AltiMAX export's header, which opens `{}`",
                MARKS.join(",")
            ),
        });
    }
    let mut lines = numbered_lines(text).filter(|(_, line)| !line.trim().is_empty());
    // Invariant: `recognizes` found the header line.
    let Some((header_line, header)) = lines.next() else {
        return Err(LogError::NoData { format: FORMAT });
    };
    let names: Vec<String> = header
        .split(',')
        .map(|name| name.trim().to_owned())
        .collect();
    // The second column named `EVENT` holds the events' names; the first, their codes.
    let mut roles: Vec<Role> = names.iter().map(|name| role(name)).collect();
    if let Some(second) = names
        .iter()
        .enumerate()
        .filter(|(_, name)| name.eq_ignore_ascii_case(EVENT))
        .nth(1)
        .map(|(index, _)| index)
    {
        roles[second] = Role::EventName;
    }
    let code = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(EVENT));
    let volt_channel = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(VOLT_CHANNEL));
    let mut notes = Notes::default();
    for (name, role) in names.iter().zip(&roles) {
        if *role == Role::Value(Quantity::Unrecognized, 1.0) {
            notes.push(format!(
                "the column {:?} isn't one this reader knows, and was kept as written",
                short(name)
            ));
        }
    }

    let mut time_s: Vec<f64> = Vec::new();
    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); names.len()];
    let mut event_names: Vec<String> = Vec::new();
    let mut gaps = vec![0usize; names.len()];
    for (number, line) in lines {
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        if cells.len() != names.len() {
            return Err(syntax(
                number,
                format!(
                    "the row has {} cells, and the header names {}",
                    cells.len(),
                    names.len()
                ),
            ));
        }
        let mut name_cell = String::new();
        for (index, (cell, role)) in cells.iter().zip(&roles).enumerate() {
            match role {
                Role::EventName => (*cell).clone_into(&mut name_cell),
                Role::Time => {
                    let time = cell.parse::<f64>().ok().filter(|time| time.is_finite());
                    let Some(time) = time else {
                        return Err(syntax(
                            number,
                            format!("the time {:?} isn't a number", short(cell)),
                        ));
                    };
                    if time_s.last().is_some_and(|last| time <= *last) {
                        return Err(syntax(
                            number,
                            format!(
                                "the time {time} doesn't follow the row before it, {}",
                                time_s.last().copied().unwrap_or_default()
                            ),
                        ));
                    }
                    time_s.push(time);
                }
                _ => {
                    let value = if cell.is_empty() {
                        gaps[index] += 1;
                        f64::NAN
                    } else {
                        let value = cell.parse::<f64>().ok().ok_or_else(|| {
                            syntax(
                                number,
                                format!(
                                    "{:?}, under {:?}, isn't a number",
                                    short(cell),
                                    short(&names[index])
                                ),
                            )
                        })?;
                        finite(value, FORMAT, number, &names[index])?
                    };
                    columns[index].push(value);
                }
            }
        }
        event_names.push(name_cell);
    }
    if time_s.is_empty() {
        return Err(LogError::NoData { format: FORMAT });
    }
    clock_in_seconds(&time_s, header_line)?;

    let mut log = FlightLog {
        format: LogFormat::AltimaxXls,
        logger: "Rocketronics AltiMAX".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
        time_s: time_s.clone(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels: Vec::new(),
        events: Vec::new(),
        notes: Vec::new(),
    };
    for (index, role) in roles.iter().enumerate() {
        let name = &names[index];
        let values = &columns[index];
        match role {
            Role::Time | Role::EventName => {}
            Role::Pressure => {
                let scale = pressure_scale(name, values, header_line, &mut notes)?;
                log.channels.push(Channel {
                    column: name.clone(),
                    quantity: Quantity::Pressure,
                    time_s: time_s.clone(),
                    values: values.iter().map(|value| value * scale).collect(),
                });
            }
            Role::Value(quantity, scale) => {
                if name.eq_ignore_ascii_case(HEIGHT_RAW) {
                    log.altitude_m.clone_from(values);
                }
                log.channels.push(Channel {
                    column: name.clone(),
                    quantity: *quantity,
                    time_s: time_s.clone(),
                    values: values.iter().map(|value| value * scale).collect(),
                });
            }
            Role::Voltage => {
                let numbers = volt_channel.map(|at| columns[at].as_slice());
                split_voltages(&mut log, name, values, numbers);
            }
        }
    }
    if log.altitude_m.is_empty() {
        return Err(syntax(
            header_line,
            "the header names no `HEIGHT RAW [m]`, the altitude above the pad".to_owned(),
        ));
    }
    if let Some(code) = code {
        for ((time, code), name) in time_s.iter().zip(&columns[code]).zip(&event_names) {
            if *code != 0.0 && !code.is_nan() {
                log.events.push(Event {
                    time_s: *time,
                    name: name.clone(),
                });
            }
        }
    }
    for (index, count) in gaps.iter().enumerate() {
        if *count > 0 {
            notes.push(format!(
                "the column {:?} holds {count} empty cells: they are gaps",
                short(&names[index])
            ));
        }
    }
    notes.push(format!(
        "`TIME` names no unit; read in seconds, its shortest step {STEP_S} s, the 200 samples a \
         second the maker's manual gives"
    ));
    log.notes = notes.into_vec();
    Ok(log)
}

/// Gives each voltage channel number its own channel of `values`, on the rows that name it; with
/// no channel column, the voltages are one channel.
fn split_voltages(log: &mut FlightLog, name: &str, values: &[f64], channels: Option<&[f64]>) {
    let Some(channels) = channels else {
        log.channels.push(Channel {
            column: name.to_owned(),
            quantity: Quantity::Voltage,
            time_s: log.time_s.clone(),
            values: values.to_vec(),
        });
        return;
    };
    let mut numbers: Vec<f64> = channels.iter().copied().filter(|n| n.is_finite()).collect();
    numbers.sort_by(f64::total_cmp);
    numbers.dedup();
    for number in numbers {
        let rows: Vec<usize> = (0..values.len())
            .filter(|row| channels[*row] == number)
            .collect();
        log.channels.push(Channel {
            column: format!("{name} (channel {number})"),
            quantity: Quantity::Voltage,
            time_s: rows.iter().map(|row| log.time_s[*row]).collect(),
            values: rows.iter().map(|row| values[*row]).collect(),
        });
    }
}

/// Holds the clock, which names no unit, to the altimeter's 200 samples a second: its shortest
/// step, read in seconds, must be 5 ms within 10 %.
fn clock_in_seconds(time_s: &[f64], header_line: usize) -> Result<(), LogError> {
    let shortest = time_s
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .fold(f64::INFINITY, f64::min);
    if time_s.len() < 2 || (shortest - STEP_S).abs() <= 0.1 * STEP_S {
        return Ok(());
    }
    Err(LogError::Unit {
        format: FORMAT,
        line: header_line,
        message: format!(
            "`TIME` names no unit, and its shortest step, {shortest}, isn't the 5 ms of the 200 \
             samples a second the altimeter stores, read in seconds"
        ),
    })
}

/// The factor that takes a pressure column to pascals: 100 when its first value is a pad's in hPa,
/// as the header states, 1 when it is a pad's in Pa, with a note.
fn pressure_scale(
    name: &str,
    values: &[f64],
    header_line: usize,
    notes: &mut Notes,
) -> Result<f64, LogError> {
    let Some(first) = values.iter().copied().find(|value| value.is_finite()) else {
        return Ok(100.0);
    };
    if PAD_PA.contains(&(first * 100.0)) {
        Ok(100.0)
    } else if PAD_PA.contains(&first) {
        notes.push(format!(
            "the column {:?} says hPa, but its values are pascals: its first, on the pad, lies \
             between 500 and 1,100 hPa only when read in Pa, and was read so",
            short(name)
        ));
        Ok(1.0)
    } else {
        Err(LogError::Unit {
            format: FORMAT,
            line: header_line,
            message: format!(
                "the column {:?}'s first value, {first}, is a pad's pressure in neither hPa nor \
                 Pa: neither puts it between 500 and 1,100 hPa",
                short(name)
            ),
        })
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

    const HEADER: &str = "TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m],\
                          ACCEL [m/s2],SPEED [m/s],EVENT,EVENT,VOLT. CHANNEL,VOLTAGE [V]";

    /// An export of `rows`, each `(time, pressure, height, event code, event, voltage channel,
    /// voltage)`, its filtered columns the raw ones.
    fn export(rows: &[(f64, f64, f64, u32, &str, u32, f64)]) -> String {
        let mut text = format!("{HEADER}\n");
        for (time, pressure, height, code, event, channel, volts) in rows {
            text.push_str(&format!(
                "{time},{pressure},{height},{pressure},{height},-9.8,0,{code},{event},{channel},\
                 {volts}\n"
            ));
        }
        text
    }

    fn sample() -> String {
        export(&[
            (0.005, 98_000.0, 0.0, 0, "PAD", 1, 7.4),
            (0.010, 97_990.0, 0.9, 3, "BARO-LIFTOFF", 2, 7.3),
            (0.015, 97_975.0, 2.2, 0, "PAD", 1, 7.39),
            (0.040, 97_950.0, 4.3, 9, "APOGEE PRESSURE", 2, 7.29),
        ])
    }

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .unwrap_or_else(|| panic!("no channel {column}"))
    }

    #[test]
    fn an_export_reads_with_its_events_and_voltages() {
        let log = read(&sample()).unwrap();
        assert_eq!(log.format, LogFormat::AltimaxXls);
        assert_eq!(log.time_s, [0.005, 0.01, 0.015, 0.04]);
        assert_eq!(log.altitude_m, [0.0, 0.9, 2.2, 4.3]);
        let names: Vec<(f64, &str)> = log
            .events
            .iter()
            .map(|event| (event.time_s, event.name.as_str()))
            .collect();
        assert_eq!(names, [(0.01, "BARO-LIFTOFF"), (0.04, "APOGEE PRESSURE")]);
        // Each voltage channel number on its own rows.
        let first = channel(&log, "VOLTAGE [V] (channel 1)");
        assert_eq!(
            (first.time_s.as_slice(), first.values.as_slice()),
            (&[0.005, 0.015][..], &[7.4, 7.39][..])
        );
        let second = channel(&log, "VOLTAGE [V] (channel 2)");
        assert_eq!(second.values, [7.3, 7.29]);
        assert_eq!(channel(&log, "EVENT").values, [0.0, 3.0, 0.0, 9.0]);
        assert_eq!(
            channel(&log, "HEIGHT FILTER [m]").quantity,
            Quantity::AltitudeAboveGround
        );
        assert_eq!(log.vertical_speed_m_s, None);
    }

    #[test]
    fn a_pressure_is_read_in_the_unit_that_puts_the_pad_in_its_band() {
        // Pascals under `[hpa]`: read in Pa, with a note.
        let log = read(&sample()).unwrap();
        assert_eq!(channel(&log, "PRESS RAW [hpa]").values[0], 98_000.0);
        assert!(
            log.notes
                .iter()
                .any(|note| note.contains("says hPa, but its values are pascals"))
        );
        // Hectopascals as stated: read in hPa, no note.
        let hpa = export(&[
            (0.005, 980.0, 0.0, 0, "PAD", 1, 7.4),
            (0.01, 979.9, 0.9, 0, "PAD", 2, 7.3),
        ]);
        let log = read(&hpa).unwrap();
        assert_eq!(channel(&log, "PRESS RAW [hpa]").values[0], 98_000.0);
        assert!(!log.notes.iter().any(|note| note.contains("says hPa")));
        // The band's edges, 500 and 1,100 hPa, in each unit; just past them, neither.
        for (first, pascals) in [
            (500.0, 50_000.0),
            (1_100.0, 110_000.0),
            (50_000.0, 50_000.0),
            (110_000.0, 110_000.0),
        ] {
            let text = export(&[
                (0.005, first, 0.0, 0, "PAD", 1, 7.4),
                (0.01, first, 0.0, 0, "PAD", 1, 7.4),
            ]);
            assert_eq!(
                channel(&read(&text).unwrap(), "PRESS RAW [hpa]").values[0],
                pascals
            );
        }
        for first in [499.9, 1_100.1, 49_999.0, 110_001.0, 5_000.0] {
            let text = export(&[
                (0.005, first, 0.0, 0, "PAD", 1, 7.4),
                (0.01, first, 0.0, 0, "PAD", 1, 7.4),
            ]);
            match read(&text) {
                Err(LogError::Unit { message, .. }) => {
                    assert!(message.contains("neither hPa nor Pa"), "{message}");
                }
                other => panic!("{first}: expected a unit refusal, got {other:?}"),
            }
        }
    }

    #[test]
    fn the_clock_is_read_in_seconds_only_at_the_altimeters_rate() {
        // A clock in milliseconds steps by 5, not 0.005.
        let ms = export(&[
            (5.0, 98_000.0, 0.0, 0, "PAD", 1, 7.4),
            (10.0, 97_990.0, 0.9, 0, "PAD", 2, 7.3),
        ]);
        match read(&ms) {
            Err(LogError::Unit { message, .. }) => assert!(message.contains("5 ms"), "{message}"),
            other => panic!("expected a unit refusal, got {other:?}"),
        }
        // Within 10 % of 5 ms, 4.5 to 5.5 ms, just inside each edge; just past each, not.
        for (step, reads) in [
            (0.004_51, true),
            (0.005_49, true),
            (0.004_49, false),
            (0.005_51, false),
        ] {
            let text = export(&[
                (1.0, 98_000.0, 0.0, 0, "PAD", 1, 7.4),
                (1.0 + step, 97_990.0, 0.9, 0, "PAD", 2, 7.3),
            ]);
            assert_eq!(read(&text).is_ok(), reads, "step {step}");
        }
    }

    #[test]
    fn a_broken_row_is_refused_on_its_line() {
        let mut text = sample();
        text.push_str("0.03,1,2\n");
        match read(&text) {
            Err(LogError::Syntax { line, message, .. }) => {
                assert_eq!(line, 6);
                assert!(
                    message.contains("has 3 cells, and the header names 11"),
                    "{message}"
                );
            }
            other => panic!("expected a syntax refusal, got {other:?}"),
        }
        let back = export(&[
            (0.01, 98_000.0, 0.0, 0, "PAD", 1, 7.4),
            (0.005, 97_990.0, 0.9, 0, "PAD", 2, 7.3),
        ]);
        assert!(matches!(read(&back), Err(LogError::Syntax { line: 3, .. })));
        let word = sample().replace("-9.8", "x");
        assert!(matches!(read(&word), Err(LogError::Syntax { line: 2, .. })));
    }

    #[test]
    fn an_empty_cell_is_a_gap_counted_in_a_note() {
        let text = sample().replacen(",-9.8,", ",,", 1);
        let log = read(&text).unwrap();
        assert!(channel(&log, "ACCEL [m/s2]").values[0].is_nan());
        assert!(
            log.notes
                .iter()
                .any(|note| note.contains("\"ACCEL [m/s2]\" holds 1 empty cells"))
        );
    }

    #[test]
    fn the_layout_is_told_by_its_first_five_columns() {
        assert!(recognizes(&sample()));
        assert!(recognizes(&sample().to_ascii_lowercase()));
        assert!(!recognizes(
            "TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa]\n"
        ));
        assert!(!recognizes("Time,Altitude\n0,0\n"));
        assert!(matches!(
            read("Time,Altitude\n0,0\n"),
            Err(LogError::NotThisFormat { .. })
        ));
        assert!(matches!(
            read(&format!("{HEADER}\n")),
            Err(LogError::NoData { .. })
        ));
    }
}
