//! Numbers as a person types them on the command line, read as the product system's `data.md`
//! asks (*Copying and typing numbers*): spaces around the number are trimmed, a comma is read
//! only as a thousands separator between groups of three digits (`1,280` is 1280), and any other
//! comma (`3,9`) is refused with a question about it rather than a parser's words. A comma read
//! as a separator is shown back as a `note:` with the value read and its unit ([`read_as`]).
//!
//! Every numeric option takes its value through [`Number`]; a test walks the command tree and
//! holds each one to it, so an option added with clap's own number parser fails it.

use std::ffi::OsStr;
use std::marker::PhantomData;
use std::str::FromStr;

use clap::error::{ContextKind, ContextValue, ErrorKind};

/// The parser of an option whose value is a number of type `T`: [`read`], refusing anything else
/// as clap refuses a value, with what to give as a hint, which `hpr` writes as a `help:` line.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Number<T>(PhantomData<fn() -> T>);

/// [`Number`] for `T`, as an option's `value_parser`.
pub(crate) const fn number<T>() -> Number<T> {
    Number(PhantomData)
}

impl<T> clap::builder::TypedValueParser for Number<T>
where
    T: FromStr + Clone + Send + Sync + 'static,
{
    type Value = T;

    fn parse_ref(
        &self,
        command: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<T, clap::Error> {
        let text = value.to_string_lossy();
        read(&text).map_err(|hint| {
            let mut error = clap::Error::new(ErrorKind::InvalidValue).with_cmd(command);
            if let Some(arg) = arg {
                error.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String(arg.to_string()),
                );
            }
            error.insert(
                ContextKind::InvalidValue,
                ContextValue::String(text.into_owned()),
            );
            error.insert(
                ContextKind::Suggested,
                ContextValue::StyledStrs(vec![hint.into()]),
            );
            error
        })
    }
}

/// `text` as a number: trimmed, its thousands separators dropped ([`ungrouped`]), then read by
/// `T`'s [`FromStr`]. The error is the hint to give: a question when a comma isn't a thousands
/// separator, else what a number looks like.
pub(crate) fn read<T: FromStr>(text: &str) -> Result<T, String> {
    plain(text)?.parse::<T>().map_err(|_| {
        // A type that refuses a fraction takes a whole number.
        if "0.5".parse::<T>().is_err() {
            "give a whole number in digits, such as 1000".to_owned()
        } else {
            "give a number in digits, with a period for the decimal point, such as 1280 or -33.9"
                .to_owned()
        }
    })
}

/// `text` trimmed and without its thousands separators ([`ungrouped`]), for a reader of its own
/// to read; the error is [`read`]'s question about a comma that isn't one.
pub(crate) fn plain(text: &str) -> Result<String, String> {
    let trimmed = text.trim();
    ungrouped(trimmed).ok_or_else(|| {
        let decimal = trimmed.replacen(',', ".", 1);
        if trimmed.matches(',').count() == 1 && decimal.parse::<f64>().is_ok() {
            format!(
                "a comma is read only between groups of three digits, as in 1,280: is {trimmed} \
                 meant as {decimal}? Write the decimal point as a period"
            )
        } else {
            format!(
                "a comma is read only between groups of three digits, as in 1,280, and {trimmed} \
                 has one elsewhere; write the number without it"
            )
        }
    })
}

/// `text` without its thousands separators, when it has any; `None` when it has a comma that
/// isn't one. A separator is a comma in the whole-number part, after a first group of one to
/// three digits and before each group of exactly three: `1,280`, `-12,000.5`, but not `3,9`,
/// `1,28`, `,280` or `1.2,5`.
fn ungrouped(text: &str) -> Option<String> {
    if !text.contains(',') {
        return Some(text.to_owned());
    }
    let unsigned = text.strip_prefix(['-', '+']).unwrap_or(text);
    let sign = &text[..text.len() - unsigned.len()];
    let end = unsigned
        .find(|c: char| !c.is_ascii_digit() && c != ',')
        .unwrap_or(unsigned.len());
    let (whole, rest) = unsigned.split_at(end);
    if rest.contains(',') {
        return None;
    }
    let mut groups = whole.split(',');
    let first = groups.next()?;
    let digits = |group: &str| group.bytes().all(|b| b.is_ascii_digit());
    if !(1..=3).contains(&first.len()) || !digits(first) {
        return None;
    }
    let mut plain = format!("{sign}{first}");
    for group in groups {
        if group.len() != 3 || !digits(group) {
            return None;
        }
        plain.push_str(group);
    }
    plain.push_str(rest);
    Some(plain)
}

/// The unit an option's value is typed in, by the value name its help shows: `M` is meters.
fn unit(value_name: &str) -> Option<&'static str> {
    match value_name {
        "M" => Some("m"),
        "MM" => Some("mm"),
        "DEG" => Some("°"),
        "S" => Some("s"),
        "M_S" => Some("m/s"),
        "DOLLARS" => Some("US dollars"),
        _ => None,
    }
}

/// A `note:` for each number the command line gave with a thousands separator, saying what it
/// was read as and in what unit (`--elevation 1,280 read as 1280 m`), so a comma meant otherwise
/// is caught before the run is trusted. `command` is the command that parsed `matches`.
pub(crate) fn read_as(command: &clap::Command, matches: &clap::ArgMatches) -> Vec<String> {
    let mut notes = Vec::new();
    if let Some((name, sub)) = matches.subcommand()
        && let Some(subcommand) = command.find_subcommand(name)
    {
        notes.extend(read_as(subcommand, sub));
    }
    for arg in command.get_arguments() {
        let Some(long) = arg.get_long() else {
            continue;
        };
        let Ok(Some(raw)) = matches.try_get_raw(arg.get_id().as_str()) else {
            continue;
        };
        for value in raw {
            let text = value.to_string_lossy();
            let trimmed = text.trim();
            if !trimmed.contains(',') {
                continue;
            }
            let Some(plain) = ungrouped(trimmed) else {
                continue;
            };
            let unit = arg
                .get_value_names()
                .and_then(|names| names.first())
                .and_then(|name| unit(name.as_str()));
            let read = match unit {
                Some("°") => format!("{plain}°"),
                Some(unit) => format!("{plain} {unit}"),
                None => plain,
            };
            notes.push(format!("--{long} {trimmed} read as {read}"));
        }
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spaces_are_trimmed_and_thousands_separators_dropped() {
        assert_eq!(read::<f64>(" 2.0 "), Ok(2.0));
        assert_eq!(read::<f64>("1,280"), Ok(1280.0));
        assert_eq!(read::<f64>("-12,000.5"), Ok(-12000.5));
        assert_eq!(read::<f64>("+1,000,000"), Ok(1e6));
        assert_eq!(read::<u64>("1,000"), Ok(1000));
        assert_eq!(read::<f64>("1e3"), Ok(1000.0));
    }

    #[test]
    fn any_other_comma_is_refused_with_a_question() {
        let hint = read::<f64>("3,9").unwrap_err();
        assert!(hint.contains("is 3,9 meant as 3.9?"), "{hint}");
        for text in [
            "1,28", ",280", "1,2,3", "1.2,5", "1,280,", "1234,567", "-,5", "1,,280",
        ] {
            assert!(read::<f64>(text).is_err(), "{text}");
        }
        assert!(
            read::<f64>("1.2,5")
                .unwrap_err()
                .contains("has one elsewhere")
        );
    }

    #[test]
    fn a_word_gets_what_a_number_looks_like() {
        let hint = read::<f64>("abc").unwrap_err();
        assert!(hint.starts_with("give a number in digits"), "{hint}");
        assert!(read::<u64>("1.5").is_err());
    }
}
