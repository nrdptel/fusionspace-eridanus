//! A course's Teensy data logger: one sensor's reading a line, each line opening with the
//! sensor's tag and the logger's clock in microseconds, in two layouts.
//!
//! The Harvey Mudd College advanced rocketry course flies a data logger it builds around a Teensy
//! board, and publishes its flights' files on its [*Flight Data* page][flight-data], which lists two kinds of
//! `.TXT` file, "Teensy Data Logger" and "New Teensy Data Logger", read by the course's own
//! scripts. No document describes either layout, so this reader takes them from the files, and
//! the sample logs' survey (`cargo xtask logs`) holds each part of the reading that isn't stated
//! to a physical check ([ADR-274][adr-274]).
//!
//! **The tab-separated layout** ([`LogFormat::TeensyTabs`]) writes a tag, the clock, then the
//! values, each field after a tab, shown here as spaces:
//!
//! ```text
//! A  2000  0.25  -0.50  9.75
//! G  2000  1.50  0.50  -2.00
//! M  2000  20.10  -3.40  17.80
//! Alt  3100  P  95000  MSL  540.42
//! HG  3600  0.4788  -0.9577  10.5337
//! ADC Pin-35  3700  0.0120000000
//! ADC Pin-36  3800  0.0150000000
//! ```
//!
//! No line states a unit. Three of the values are read in SI, each held by the survey to the
//! files:
//!
//! | Values | Read as | Held to |
//! |---|---|---|
//! | `A`'s three | [`Quantity::Acceleration`], m/s², as written | their magnitude on the pad, the 5 s ending 1 s before liftoff, and over the last 2 s, after landing, each within 3 % of standard gravity |
//! | `Alt`'s `P` | [`Quantity::Pressure`], Pa, as written | `MSL` worked out from it, below |
//! | `Alt`'s `MSL` | [`Quantity::AltitudeMsl`], m, as written | on every line, 44,330 × (1 − (P / 101,325)^(1/5.255)), the altitude formula of the Bosch BMP180 barometer's data sheet (BST-BMP180-DS000, §3.6), to within 0.0103 m: its half step, 0.005 m, as it is written to the hundredth, and an allowance of 44,330 × 2⁻²³ m for the logger's own rounding of the formula, whose form no document gives |
//!
//! The `MSL` is so a pressure altitude, above where the pressure would be 101,325 Pa, not a
//! height above the sea. The rest is kept as written, [`Quantity::Unrecognized`], as nothing
//! states its unit and no check settles one: the gyroscope (`G`), the magnetometer (`M`), a
//! quantity HPR Sim doesn't hold, the high-g accelerometer (`HG`), whose magnitude during the
//! boost, while `A` reads from 3 g to 90 % of its greatest, is 1.37 to 2.39 times `A`'s in the
//! sample logs where both are read as m/s², and the two analog inputs. A
//! value is named by its tag and its place, counted from 1, such as `A 1` or `HG 3`, or by its tag
//! and the name the line gives it, such as `Alt P`.
//!
//! **The layout with named fields** ([`LogFormat::TeensyNamed`]) opens with a line for each
//! sensor stating its units, then writes a tag, `t`, the clock, and name–value pairs, comma
//! separated:
//!
//! ```text
//! ICM data in us, milli-g, deg/s, milli-T, deg C
//! MS5 data in us, deg C, Pa
//! ICM, t, 500000500, xa, 12.20, ya, -20.50, za, 998.40, xg, 0.18, yg, -0.12, zg, 0.06, xm, 30.15, ym, 4.20, zm, -18.75, T, 24.50
//! MS5, t, 500001000, T, 24.10, P, 95000
//! ```
//!
//! The first unit is the clock's, and must be `us`. The others go to the fields by their names:
//! a name less a leading `x`, `y` or `z` is its group, so `xa`, `ya` and `za` are one, and the
//! units go to the groups in the order the first line of the tag names them; one unit stated goes to
//! every field if no name carries an axis's `x`, `y` or `z`, as `T1` and `T2` don't, or if all are
//! one group. A tag whose units don't fit its groups so, or with no units line, such as a `VOL` line
//! the sample file writes, is kept as written, and a note says so. The units read are `milli-g` and
//! `g`, to m/s², `deg/s`, `deg C`, to kelvins, and `Pa`; any other is kept as written, as is
//! `milli-T`, a magnetic field. The survey holds two of the stated units to the files: the
//! `milli-g` accelerometer's magnitude on the pad and over the file's last 2 s, after landing,
//! within 3 % of 1,000 milli-g, as for `A`, and the `Pa` pressure on the pad between 50 and 110
//! kPa, as no launch site's is outside it. The others rest on the file's statement: in the sample
//! file, the high-g accelerometer read in its stated `g` is 0.996 times the `milli-g` one over the
//! boost, which the survey reports. A value
//! is named by its tag and its name, such as `ICM xa`.
//!
//! **The clock**, in both layouts, is the logger's count of microseconds, a whole number, read as
//! seconds from the logger's own zero. The named layout states its unit; the tab-separated one
//! doesn't, and the survey holds the reading to the flights: the time from liftoff to apogee,
//! by the clock so read, is within a factor of 2 of √(2h/g) for the apogee's height h, a coarse
//! check (the burn adds time and drag takes some away), where a clock read wrongly by a power of
//! ten is 10 or more times off. Each tag is a channel set on its own clock, as each sensor is read at its own rate.
//!
//! **What refuses the file.** A line that isn't a tag, a whole-number clock and values, or whose
//! tag's layout (its count of values, or its names) differs from the tag's first line, or whose
//! clock doesn't increase from the tag's line before ([`LogError::Syntax`], naming its line). A
//! logger's clock of this kind runs over after 2³² microseconds, 71.6 minutes, so a longer
//! recording is refused there. A file's last line, if the file ends with no line end and the
//! line can't be read, is a line the logger was writing when it stopped: it is dropped, and a
//! note says so. At most [`MAX_TAGS`] tags, [`MAX_VALUES`] values on a line and names of
//! [`MAX_NAME_BYTES`]: a file past them isn't a logger's.
//!
//! The record has no barometric track, as neither layout writes an altitude above the pad:
//! [`FlightLog::time_s`] and [`FlightLog::altitude_m`] are empty, and every value is a channel.
//!
//! [adr-274]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md
//! [flight-data]: https://pages.hmc.edu/spjut/AdvRoc/FlightData.md.html

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::table::{finite, number, numbered_lines, short};

/// The two layouts' names in errors.
const TABS: &str = "Teensy data logger, tab-separated";
const NAMED: &str = "Teensy data logger, named fields";

/// How many of a file's first lines that aren't blank are searched for its marks.
const RECOGNITION_LINES: usize = 100;

/// The most tags a file may hold, over four times the sample logs' seven.
pub const MAX_TAGS: usize = 32;

/// The most values a line may hold, over six times the sample logs' ten.
pub const MAX_VALUES: usize = 64;

/// The longest a tag, a value's name or a unit may be, bytes, three times the sample logs'
/// longest, `ADC Pin-35`.
pub const MAX_NAME_BYTES: usize = 32;

/// The largest clock, µs: a 32-bit count of microseconds.
const CLOCK_LIMIT_US: u64 = u32::MAX as u64;

/// The value of a field in a line: by its place, or under a name the line gives it.
#[derive(Debug, Clone, PartialEq)]
struct Line<'a> {
    tag: &'a str,
    clock_us: u64,
    /// Each value's name, `None` for a value by its place.
    names: Vec<Option<&'a str>>,
    values: Vec<f64>,
}

/// A field as a tag, a name or a unit: not empty, not a number, within [`MAX_NAME_BYTES`].
fn name(field: &str) -> Result<&str, String> {
    if field.is_empty() {
        Err("an empty field where a name should be".to_owned())
    } else if field.len() > MAX_NAME_BYTES {
        Err(format!(
            "a name longer than {MAX_NAME_BYTES} bytes, {}",
            short(field)
        ))
    } else if number(field, false).is_some() {
        Err(format!("a number, {field}, where a name should be"))
    } else {
        Ok(field)
    }
}

/// The clock: a whole number of microseconds, below the 32-bit count's limit.
fn clock(field: &str) -> Result<u64, String> {
    let parsed =
        if field.is_empty() || field.len() > 20 || !field.bytes().all(|b| b.is_ascii_digit()) {
            None
        } else {
            field.parse::<u64>().ok()
        };
    match parsed {
        Some(us) if us <= CLOCK_LIMIT_US => Ok(us),
        Some(_) => Err(format!(
            "a clock, {field} µs, past a 32-bit count of microseconds"
        )),
        None => Err(format!(
            "a clock, {}, that isn't a whole number of microseconds",
            short(field)
        )),
    }
}

/// The fields after the clock: all numbers, by place, or name–number pairs.
fn values<'a>(fields: &[&'a str]) -> Result<(Vec<Option<&'a str>>, Vec<f64>), String> {
    if fields.len() > 2 * MAX_VALUES {
        return Err(format!("more than {MAX_VALUES} values"));
    }
    let numbers: Option<Vec<f64>> = fields.iter().map(|f| number(f, false)).collect();
    if let Some(numbers) = numbers {
        if numbers.is_empty() {
            return Err("no values after the clock".to_owned());
        }
        if numbers.len() > MAX_VALUES {
            return Err(format!("more than {MAX_VALUES} values"));
        }
        return Ok((vec![None; numbers.len()], numbers));
    }
    if !fields.len().is_multiple_of(2) {
        return Err("values neither all numbers nor in name–number pairs".to_owned());
    }
    let mut names = Vec::with_capacity(fields.len() / 2);
    let mut numbers = Vec::with_capacity(fields.len() / 2);
    let (pairs, _) = fields.as_chunks::<2>();
    for [field, value] in pairs {
        let field = name(field)?;
        if names.contains(&Some(field)) {
            return Err(format!("the name {field} twice on one line"));
        }
        names.push(Some(field));
        numbers
            .push(number(value, false).ok_or_else(|| {
                format!("{field}'s value, {}, that isn't a number", short(value))
            })?);
    }
    Ok((names, numbers))
}

/// A tab-separated line: a tag, the clock, the values.
fn tab_line(text: &str) -> Result<Line<'_>, String> {
    let fields: Vec<&str> = text.split('\t').collect();
    if fields.len() < 3 {
        return Err("fewer than a tag, a clock and a value, by tabs".to_owned());
    }
    let tag = name(fields[0])?;
    let clock_us = clock(fields[1])?;
    let (names, values) = values(&fields[2..])?;
    Ok(Line {
        tag,
        clock_us,
        names,
        values,
    })
}

/// A line of the named layout: a tag, `t`, the clock, then name–value pairs, by commas.
fn named_line(text: &str) -> Result<Line<'_>, String> {
    let fields: Vec<&str> = text.split(',').map(str::trim).collect();
    if fields.len() < 5 || fields[1] != "t" {
        return Err("not a tag, `t`, a clock and name–value pairs, by commas".to_owned());
    }
    let tag = name(fields[0])?;
    let clock_us = clock(fields[2])?;
    let (names, values) = values(&fields[3..])?;
    if names.iter().any(Option::is_none) {
        return Err("values with no names in a layout that names them".to_owned());
    }
    Ok(Line {
        tag,
        clock_us,
        names,
        values,
    })
}

/// A units line of the named layout, `ICM data in us, milli-g, …`: the tag and its units.
fn units_line(text: &str) -> Option<(&str, Vec<&str>)> {
    let (tag, units) = text.split_once(" data in ")?;
    let units: Vec<&str> = units.split(',').map(str::trim).collect();
    (name(tag.trim()).is_ok() && units.iter().all(|unit| name(unit).is_ok()))
        .then_some((tag.trim(), units))
}

/// Whether the text is the tab-separated layout: its first line that isn't blank is a tag, a
/// clock and values by tabs, and one of its first 100 is an `Alt` line, `Alt`, the clock, `P`
/// and a pressure, `MSL` and an altitude.
pub fn recognizes_tabs(text: &str) -> bool {
    let mut lines = numbered_lines(text)
        .map(|(_, line)| line)
        .filter(|line| !line.trim().is_empty())
        .take(RECOGNITION_LINES);
    let Some(first) = lines.next() else {
        return false;
    };
    tab_line(first).is_ok()
        && std::iter::once(first).chain(lines).any(|line| {
            tab_line(line)
                .is_ok_and(|line| line.tag == "Alt" && line.names == [Some("P"), Some("MSL")])
        })
}

/// Whether the text is the named layout: its first line that isn't blank states a tag's units,
/// the clock's `us` first, and one of its first 100 is a line of name–value pairs.
pub fn recognizes_named(text: &str) -> bool {
    let mut lines = numbered_lines(text)
        .map(|(_, line)| line)
        .filter(|line| !line.trim().is_empty())
        .take(RECOGNITION_LINES);
    let Some(first) = lines.next() else {
        return false;
    };
    units_line(first).is_some_and(|(_, units)| units[0] == "us")
        && lines.any(|line| named_line(line).is_ok())
}

/// The lines of one tag.
struct Tag<'a> {
    tag: &'a str,
    first_line: usize,
    names: Vec<Option<&'a str>>,
    last_us: u64,
    /// Each line's number, for an error about one of its values.
    lines: Vec<usize>,
    time_s: Vec<f64>,
    values: Vec<Vec<f64>>,
}

/// The tags' lines, in the order the file first writes them, and what the reading dropped.
struct Lines<'a> {
    tags: Vec<Tag<'a>>,
    units: Vec<(&'a str, Vec<&'a str>, usize)>,
    dropped_last: bool,
}

/// Reads every line by `parse`, after any units lines the named layout opens with.
fn lines<'a>(
    text: &'a str,
    format: &'static str,
    parse: fn(&'a str) -> Result<Line<'a>, String>,
    with_units: bool,
) -> Result<Lines<'a>, LogError> {
    let syntax = |line: usize, message: String| LogError::Syntax {
        format,
        line,
        message,
    };
    let whole = text;
    let mut numbered = numbered_lines(text)
        .filter(|(_, line)| !line.trim().is_empty())
        .peekable();
    let mut tags: Vec<Tag<'a>> = Vec::new();
    let mut units = Vec::new();
    let mut dropped_last = false;
    while let Some((number, text)) = numbered.next() {
        // A line the logger was cut off in: the last that isn't blank, with no line end after it.
        let cut = numbered.peek().is_none() && {
            let end = text.as_ptr() as usize - whole.as_ptr() as usize + text.len();
            !whole[end..].contains(['\n', '\r'])
        };
        if with_units
            && tags.is_empty()
            && let Some((tag, stated)) = units_line(text)
        {
            if units.iter().any(|(t, _, _)| *t == tag) {
                return Err(syntax(number, format!("a second units line for {tag}")));
            }
            if units.len() == MAX_TAGS {
                return Err(syntax(number, format!("more than {MAX_TAGS} units lines")));
            }
            units.push((tag, stated, number));
            continue;
        }
        let checked =
            parse(text).and_then(
                |line| match tags.iter().position(|tag| tag.tag == line.tag) {
                    Some(index) => {
                        let tag = &tags[index];
                        if line.names != tag.names {
                            return Err(format!(
                                "a {} line laid out unlike its first, on line {}",
                                line.tag, tag.first_line
                            ));
                        }
                        if line.clock_us <= tag.last_us {
                            return Err(format!(
                                "a {} line whose clock, {} µs, doesn't come after the one before",
                                line.tag, line.clock_us
                            ));
                        }
                        Ok((Some(index), line))
                    }
                    None if tags.len() == MAX_TAGS => Err(format!("more than {MAX_TAGS} tags")),
                    None => Ok((None, line)),
                },
            );
        let (index, line) = match checked {
            Ok(found) => found,
            Err(_) if cut => {
                dropped_last = true;
                continue;
            }
            Err(message) => return Err(syntax(number, message)),
        };
        let index = index.unwrap_or_else(|| {
            tags.push(Tag {
                tag: line.tag,
                first_line: number,
                names: line.names.clone(),
                last_us: 0,
                lines: Vec::new(),
                time_s: Vec::new(),
                values: vec![Vec::new(); line.values.len()],
            });
            tags.len() - 1
        });
        let tag = &mut tags[index];
        tag.last_us = line.clock_us;
        tag.lines.push(number);
        tag.time_s.push(line.clock_us as f64 / 1e6);
        for (column, value) in tag.values.iter_mut().zip(line.values) {
            column.push(value);
        }
    }
    if tags.is_empty() {
        return Err(LogError::NoData { format });
    }
    Ok(Lines {
        tags,
        units,
        dropped_last,
    })
}

/// A value's column: its tag and its name, or its tag and its place.
fn column(tag: &str, name: Option<&str>, place: usize) -> String {
    match name {
        Some(name) => format!("{tag} {name}"),
        None => format!("{tag} {}", place + 1),
    }
}

fn record(format: LogFormat, channels: Vec<Channel>, notes: Vec<String>) -> FlightLog {
    FlightLog {
        format,
        logger: "Teensy data logger".to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            apogee_m: None,
            ground_elevation_msl_m: None,
            samples: None,
            figures: Vec::new(),
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    }
}

const DROPPED_LAST: &str = "the last line, which ends the file with no line end and can't be \
                            read, dropped: a line the logger was writing when it stopped";

/// Reads the tab-separated layout.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the text isn't the layout ([`recognizes_tabs`]);
/// [`LogError::Syntax`] for a line that can't be read (see the module's docs);
/// [`LogError::NoData`] for a file with no lines.
pub fn read(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes_tabs(text) {
        return Err(LogError::NotThisFormat {
            format: TABS,
            message: "the first line isn't a tag, a clock and values by tabs, or no `Alt` line \
                      with `P` and `MSL` comes within the first 100"
                .to_owned(),
        });
    }
    let Lines {
        tags, dropped_last, ..
    } = lines(text, TABS, tab_line, false)?;
    let mut channels = Vec::new();
    let mut kept = Vec::new();
    for tag in tags {
        let read_by_place = tag.tag == "A" && tag.names == [None, None, None];
        for (place, (name, values)) in tag.names.iter().zip(tag.values).enumerate() {
            let quantity = match (tag.tag, *name) {
                _ if read_by_place => Quantity::Acceleration,
                ("Alt", Some("P")) => Quantity::Pressure,
                ("Alt", Some("MSL")) => Quantity::AltitudeMsl,
                _ => Quantity::Unrecognized,
            };
            let column = column(tag.tag, *name, place);
            if quantity == Quantity::Unrecognized {
                kept.push(format!("`{column}`"));
            }
            channels.push(Channel {
                column,
                quantity,
                time_s: tag.time_s.clone(),
                values,
            });
        }
    }
    let mut notes = vec![
        "each tag's lines on their own clock, the logger's count of microseconds, read as \
         seconds from the logger's zero; no line states the unit"
            .to_owned(),
        "`A` read in m/s² and `Alt`'s `P` in Pa and `MSL` in m, as written. No line states a \
         unit, and the sample logs' survey holds these to the files"
            .to_owned(),
    ];
    if !kept.is_empty() {
        notes.push(format!(
            "kept as written, as no line states their unit: {}",
            kept.join(", ")
        ));
    }
    if dropped_last {
        notes.push(DROPPED_LAST.to_owned());
    }
    Ok(record(LogFormat::TeensyTabs, channels, notes))
}

/// A conversion to SI.
type ToSi = fn(f64) -> f64;

/// A stated unit HPR Sim reads: its quantity and its conversion to SI.
fn stated_unit(unit: &str) -> Option<(Quantity, ToSi)> {
    Some(match unit {
        "milli-g" => (Quantity::Acceleration, |v| {
            v * STANDARD_GRAVITY_MPS2 / 1_000.0
        }),
        "g" => (Quantity::Acceleration, |v| v * STANDARD_GRAVITY_MPS2),
        "deg/s" => (Quantity::AngularRate, |v| v),
        "deg C" => (Quantity::Temperature, |v| v + 273.15),
        "Pa" => (Quantity::Pressure, |v| v),
        _ => return None,
    })
}

/// A field's group: its name less a leading `x`, `y` or `z`.
fn group(name: &str) -> &str {
    match name.strip_prefix(['x', 'y', 'z']) {
        Some(rest) if !rest.is_empty() => rest,
        _ => name,
    }
}

/// Each field's stated unit, or why the tag's are kept as written.
fn units_of<'a>(names: &[Option<&str>], units: Option<&[&'a str]>) -> Result<Vec<&'a str>, String> {
    let Some(units) = units else {
        return Err("no units line states its units".to_owned());
    };
    let mut groups: Vec<&str> = Vec::new();
    for name in names.iter().flatten() {
        if !groups.contains(&group(name)) {
            groups.push(group(name));
        }
    }
    let fields = &units[1..];
    // One unit for fields of several groups only where no name carries an axis, as `T1` and `T2`
    // don't: a sensor's `xa` and `xg` can't share one.
    let unaxed = names.iter().flatten().all(|name| group(name) == *name);
    if fields.len() == 1 && (groups.len() == 1 || unaxed) {
        Ok(vec![fields[0]; names.len()])
    } else if fields.len() == groups.len() {
        Ok(names
            .iter()
            .flatten()
            .map(|name| {
                // Every name's group is among `groups`, built from the same names.
                let at = groups.iter().position(|g| *g == group(name)).unwrap_or(0);
                fields[at]
            })
            .collect())
    } else {
        Err(format!(
            "its units line states {} for {} groups of fields",
            if fields.len() == 1 {
                "1 unit".to_owned()
            } else {
                format!("{} units", fields.len())
            },
            groups.len()
        ))
    }
}

/// Reads the layout with named fields.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the text isn't the layout ([`recognizes_named`]);
/// [`LogError::Unit`] for a units line whose clock isn't in `us`; [`LogError::Syntax`] for a
/// line that can't be read (see the module's docs); [`LogError::NoData`] for a file with no
/// data lines.
pub fn read_named(text: &str) -> Result<FlightLog, LogError> {
    if !recognizes_named(text) {
        return Err(LogError::NotThisFormat {
            format: NAMED,
            message: "the first line doesn't state a tag's units, the clock's `us` first, or no \
                      line of name–value pairs comes within the first 100"
                .to_owned(),
        });
    }
    let Lines {
        tags,
        units,
        dropped_last,
    } = lines(text, NAMED, named_line, true)?;
    if let Some((tag, stated, line)) = units.iter().find(|(_, stated, _)| stated[0] != "us") {
        return Err(LogError::Unit {
            format: NAMED,
            line: *line,
            message: format!(
                "{tag}'s clock is stated in {}, not `us`, the one unit of time this reader takes",
                short(stated[0])
            ),
        });
    }
    let mut channels = Vec::new();
    let mut notes = vec![
        "each tag's lines on their own clock, the logger's count of microseconds as its units \
         lines state, read as seconds from the logger's zero"
            .to_owned(),
    ];
    let mut unread_units: Vec<String> = Vec::new();
    for tag in tags {
        let stated = units
            .iter()
            .find(|(t, _, _)| *t == tag.tag)
            .map(|(_, stated, _)| stated.as_slice());
        let field_units = match units_of(&tag.names, stated) {
            Ok(field_units) => Some(field_units),
            Err(why) => {
                notes.push(format!("{}'s values kept as written: {why}", tag.tag));
                None
            }
        };
        for (place, (name, values)) in tag.names.iter().zip(tag.values).enumerate() {
            let column = column(tag.tag, *name, place);
            let unit = field_units.as_ref().map(|units| units[place]);
            let (quantity, values) = match unit.and_then(stated_unit) {
                Some((quantity, to_si)) => {
                    let values = values
                        .into_iter()
                        .zip(&tag.lines)
                        .map(|(value, line)| finite(to_si(value), NAMED, *line, &column))
                        .collect::<Result<Vec<f64>, LogError>>()?;
                    (quantity, values)
                }
                None => {
                    if let Some(unit) = unit {
                        unread_units.push(format!("`{column}`, in {unit}"));
                    }
                    (Quantity::Unrecognized, values)
                }
            };
            channels.push(Channel {
                column,
                quantity,
                time_s: tag.time_s.clone(),
                values,
            });
        }
    }
    if !unread_units.is_empty() {
        notes.push(format!(
            "kept as written, in a unit stated that HPR Sim doesn't hold: {}",
            unread_units.join(", ")
        ));
    }
    if dropped_last {
        notes.push(DROPPED_LAST.to_owned());
    }
    Ok(record(LogFormat::TeensyNamed, channels, notes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABS_TEXT: &str = "A\t2000\t0.25\t-0.50\t9.75\r\n\
                             G\t2000\t1.5\t0.5\t-2.0\r\n\
                             Alt\t3000\tP\t95000\tMSL\t540.42\r\n\
                             HG\t3500\t0.48\t-0.96\t10.53\r\n\
                             A\t25000\t0.30\t-0.45\t9.80\r\n\
                             G\t25000\t1.4\t0.6\t-2.1\r\n\
                             Alt\t26000\tP\t94990\tMSL\t541.29\r\n\
                             HG\t26500\t0.48\t-0.48\t10.05\r\n";

    const NAMED_TEXT: &str = "ICM data in us, milli-g, deg/s, milli-T, deg C\r\n\
                              MS5 data in us, deg C, Pa\r\n\
                              ADC data in us, deg C\r\n\
                              ICM, t, 1000000, xa, 10.00, ya, -20.00, za, 1000.00, xg, 0.5, yg, -0.5, zg, 0.0, xm, 40.0, ym, 5.0, zm, -20.0, T, 25.00\r\n\
                              MS5, t, 1000500, T, 24.50, P, 95000\r\n\
                              ADC, t, 1000700, T1, 20.000, T2, 21.500\r\n\
                              VOL, t, 1000900, V, 4.100\r\n\
                              ICM, t, 1001000, xa, 11.00, ya, -21.00, za, 999.00, xg, 0.4, yg, -0.4, zg, 0.1, xm, 40.5, ym, 5.5, zm, -20.5, T, 25.25\r\n";

    fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
        log.channels
            .iter()
            .find(|c| c.column == column)
            .unwrap_or_else(|| panic!("no {column}"))
    }

    #[test]
    fn each_tag_is_read_on_its_own_clock_in_microseconds() {
        assert!(recognizes_tabs(TABS_TEXT));
        assert!(!recognizes_named(TABS_TEXT));
        let log = read(TABS_TEXT).unwrap();
        let a = channel(&log, "A 3");
        assert_eq!(a.quantity, Quantity::Acceleration);
        assert_eq!(a.time_s, [0.002, 0.025]);
        assert_eq!(a.values, [9.75, 9.80]);
        let p = channel(&log, "Alt P");
        assert_eq!(
            (p.quantity, p.time_s.as_slice()),
            (Quantity::Pressure, &[0.003, 0.026][..])
        );
        assert_eq!(channel(&log, "Alt MSL").quantity, Quantity::AltitudeMsl);
        for kept in ["G 1", "G 2", "G 3", "HG 1", "HG 2", "HG 3"] {
            assert_eq!(
                channel(&log, kept).quantity,
                Quantity::Unrecognized,
                "{kept}"
            );
        }
        assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    }

    #[test]
    fn named_fields_take_their_groups_units() {
        assert!(recognizes_named(NAMED_TEXT));
        assert!(!recognizes_tabs(NAMED_TEXT));
        let log = read_named(NAMED_TEXT).unwrap();
        let za = channel(&log, "ICM za");
        assert_eq!(za.quantity, Quantity::Acceleration);
        assert_eq!(za.values[0], 1_000.0 * STANDARD_GRAVITY_MPS2 / 1_000.0);
        assert_eq!(za.time_s, [1.0, 1.001]);
        assert_eq!(channel(&log, "ICM yg").quantity, Quantity::AngularRate);
        assert_eq!(channel(&log, "ICM xm").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "ICM xm").values[0], 40.0);
        assert_eq!(channel(&log, "ICM T").values[0], 25.0 + 273.15);
        assert_eq!(channel(&log, "MS5 P").values, [95_000.0]);
        assert_eq!(channel(&log, "MS5 T").values, [24.5 + 273.15]);
        // One unit for two fields: both take it.
        assert_eq!(channel(&log, "ADC T2").values, [21.5 + 273.15]);
        // No units line: kept as written.
        assert_eq!(channel(&log, "VOL V").quantity, Quantity::Unrecognized);
        assert!(
            log.notes
                .iter()
                .any(|n| n.starts_with("VOL's values kept as written"))
        );
        assert!(log.notes.iter().any(|n| n.contains("`ICM xm`, in milli-T")));
    }

    #[test]
    fn units_that_dont_fit_the_groups_are_kept_as_written() {
        let text = NAMED_TEXT.replace("MS5 data in us, deg C, Pa", "MS5 data in us, deg C, Pa, V");
        let log = read_named(&text).unwrap();
        assert_eq!(channel(&log, "MS5 P").quantity, Quantity::Unrecognized);
        assert!(log.notes.iter().any(|n| n
            == "MS5's values kept as written: its units line states 3 units for 2 groups of fields"));
    }

    #[test]
    fn one_unit_goes_to_every_field_only_without_axes() {
        let text = NAMED_TEXT.replace(
            "ICM data in us, milli-g, deg/s, milli-T, deg C",
            "ICM data in us, milli-g",
        );
        let log = read_named(&text).unwrap();
        assert_eq!(channel(&log, "ICM xa").quantity, Quantity::Unrecognized);
        assert_eq!(channel(&log, "ICM xg").quantity, Quantity::Unrecognized);
        assert!(log.notes.iter().any(|n| n
            == "ICM's values kept as written: its units line states 1 unit for 4 groups of fields"));
        // `T1` and `T2` carry no axis: the one unit goes to both.
        assert_eq!(channel(&log, "ADC T1").quantity, Quantity::Temperature);
    }

    #[test]
    fn a_clock_not_in_microseconds_is_refused_by_its_line() {
        let text = NAMED_TEXT.replace("MS5 data in us,", "MS5 data in ms,");
        let error = read_named(&text).unwrap_err();
        assert!(matches!(error, LogError::Unit { line: 2, .. }), "{error}");
    }

    #[test]
    fn a_clock_that_doesnt_increase_is_refused_by_its_line() {
        let text = TABS_TEXT.replace("A\t25000", "A\t2000");
        let error = read(&text).unwrap_err();
        let LogError::Syntax { line, message, .. } = &error else {
            panic!("{error}");
        };
        assert_eq!(*line, 5);
        assert!(message.contains("doesn't come after"), "{message}");
    }

    #[test]
    fn a_line_laid_out_unlike_its_tags_first_is_refused() {
        let text = TABS_TEXT.replace("A\t25000\t0.30\t-0.45\t9.80", "A\t25000\t0.30\t-0.45");
        let error = read(&text).unwrap_err();
        assert!(
            matches!(&error, LogError::Syntax { line: 5, message, .. } if message.contains("unlike its first, on line 1")),
            "{error}"
        );
    }

    #[test]
    fn a_cut_last_line_is_dropped_only_with_no_line_end() {
        let cut = format!("{TABS_TEXT}A\t48000\t0.31\t-0.4");
        let log = read(&cut).unwrap();
        assert_eq!(channel(&log, "A 1").values.len(), 2);
        assert!(log.notes.iter().any(|n| n == DROPPED_LAST));
        let ended = format!("{cut}\r\n");
        let error = read(&ended).unwrap_err();
        assert!(matches!(error, LogError::Syntax { line: 9, .. }), "{error}");
        // A bad last line with a line end and blank space after it is refused, not dropped.
        let trailing = format!("{cut}\r\n \t");
        let error = read(&trailing).unwrap_err();
        assert!(matches!(error, LogError::Syntax { line: 9, .. }), "{error}");
        // A whole last line with no line end is read.
        let whole = TABS_TEXT.trim_end();
        let log = read(whole).unwrap();
        assert_eq!(channel(&log, "HG 1").values.len(), 2);
        assert!(!log.notes.iter().any(|n| n == DROPPED_LAST));
    }

    #[test]
    fn a_clock_past_32_bits_or_not_whole_is_refused() {
        assert!(clock("4294967295").is_ok());
        assert!(clock("4294967296").unwrap_err().contains("32-bit"));
        for bad in ["", "1.5", "-3", "+3", "12a", "99999999999999999999999"] {
            assert!(clock(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_bounds_hold_at_their_limits() {
        let at = |count: usize| {
            let values = vec!["1"; count].join("\t");
            format!("X\t1\t{values}\r\nAlt\t2\tP\t95000\tMSL\t540.42\r\n")
        };
        assert!(read(&at(MAX_VALUES)).is_ok());
        assert!(!recognizes_tabs(&at(MAX_VALUES + 1)));
        let long = "N".repeat(MAX_NAME_BYTES);
        let named = |tag: &str| format!("{tag}\t1\t1\r\nAlt\t2\tP\t95000\tMSL\t540.42\r\n");
        assert!(read(&named(&long)).is_ok());
        assert!(!recognizes_tabs(&named(&format!("{long}N"))));
        let mut many = String::from("Alt\t1\tP\t95000\tMSL\t540.42\r\n");
        for tag in 1..MAX_TAGS {
            many.push_str(&format!("T{tag}\t1\t1\r\n"));
        }
        assert!(read(&many).is_ok());
        many.push_str("Last\t1\t1\r\n");
        let error = read(&many).unwrap_err();
        assert!(
            matches!(&error, LogError::Syntax { message, .. } if message.contains("more than 32 tags")),
            "{error}"
        );
    }

    #[test]
    fn neither_layout_takes_the_others_or_a_table() {
        for text in [
            "Time,Altitude\n0,0\n1,5\n",
            "A\t1\t2\t3\n",
            "ICM, t, 1, xa, 2\n",
            "ICM data in ms, g\nICM, t, 1, xa, 2\n",
        ] {
            assert!(!recognizes_tabs(text) && !recognizes_named(text), "{text}");
        }
    }

    #[test]
    fn a_value_too_large_for_its_conversion_is_refused_by_its_line() {
        let text = NAMED_TEXT.replace("za, 999.00", "za, 1.7e308");
        let error = read_named(&text).unwrap_err();
        assert!(
            matches!(&error, LogError::Syntax { line: 8, message, .. } if message.contains("ICM za")),
            "{error}"
        );
    }

    #[test]
    fn a_name_twice_or_a_value_with_no_name_is_refused() {
        let twice = NAMED_TEXT.replace("T, 24.50, P, 95000", "P, 24.50, P, 95000");
        assert!(matches!(
            read_named(&twice),
            Err(LogError::Syntax { line: 5, .. })
        ));
        let unnamed = NAMED_TEXT.replace("V, 4.100", "4.100, 4.2");
        assert!(matches!(
            read_named(&unnamed),
            Err(LogError::Syntax { line: 7, .. })
        ));
    }
}
