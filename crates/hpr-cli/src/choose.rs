//! Configurations and mounts by name: what `hpr sim` calls them, and which one `--config` or
//! `--mount` means.
//!
//! An OpenRocket file names its configurations and parts by UUID, which a reader can't tell apart,
//! so the output calls each by its name. A configuration the file leaves unnamed is called by its
//! motors and their delays, in brackets, such as `[H128W-14]`. Configurations are also numbered in the file's order,
//! so two of the same name can still be told apart; `--config` takes the number, the name or the
//! id.

use hpr::hpr_motor::Delay;

use crate::Failure;

/// A configuration's name for the output: `name`, or where it is blank its motors in brackets
/// ([`motor_label`]), such as `[H128W-14]` or `[D12-5; D12-5]`, as OpenRocket writes such a name
/// into its examples; `[no motors]` with none.
pub(crate) fn configuration_label<'a>(
    name: &str,
    motors: impl Iterator<Item = (&'a str, Option<Delay>)>,
) -> String {
    let name = name.trim();
    if !name.is_empty() {
        return crate::printable(name);
    }
    let designations: Vec<String> = motors
        .map(|(designation, delay)| motor_label(designation, delay))
        .collect();
    if designations.is_empty() {
        "[no motors]".to_owned()
    } else {
        format!("[{}]", designations.join("; "))
    }
}

/// A motor as a flyer writes it: its designation and its delay, such as `C6-5`, or `C6-P`
/// plugged, or `C6-0` for a `0` that may mean either. A designation that already carries a
/// dash, such as `1266J760-19A`, is left as it is.
pub(crate) fn motor_label(designation: &str, delay: Option<Delay>) -> String {
    let designation = crate::printable(designation);
    if designation.contains('-') {
        return designation;
    }
    match delay {
        Some(delay) => format!("{designation}-{}", delay_label(delay)),
        None => designation,
    }
}

/// A delay as a flyer writes it after a motor's dash: `5`, `P` plugged, or `0` for a `0` that may
/// mean either.
pub(crate) fn delay_label(delay: Delay) -> String {
    match delay {
        Delay::Seconds(s) => format!("{s}"),
        Delay::Plugged => "P".to_owned(),
        Delay::ZeroOrPlugged => "0".to_owned(),
        _ => "?".to_owned(),
    }
}

/// A part's name for the output: its name, or its id where it has none.
pub(crate) fn part_label(id: &str, name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        crate::printable(id)
    } else {
        crate::printable(name)
    }
}

/// One of a list to choose from: its id and its name for the output.
pub(crate) struct Choice<'a> {
    /// The id.
    pub id: &'a str,
    /// The name, as [`configuration_label`] or [`part_label`] gives it.
    pub label: String,
}

/// Which of `choices` `wanted` means, by index. It may be a choice's id (exactly, or with its case
/// ignored where no id matches exactly), with `numbered` its number counting from 1, or its name
/// (exactly, or with its case and any brackets ignored where none matches exactly). Every reading
/// that matches must mean the same choice: a word that is one choice's id and another's number,
/// or a name several share, is refused, naming each choice it could mean. `what` names the kind,
/// such as `configuration`; `whose`, where they come from, such as `the file`.
pub(crate) fn pick(
    choices: &[Choice<'_>],
    wanted: &str,
    numbered: bool,
    what: &str,
    whose: &str,
) -> Result<usize, Failure> {
    let either = |exact: Vec<usize>, loose: &dyn Fn() -> Vec<usize>| {
        if exact.is_empty() { loose() } else { exact }
    };
    let by_id = either(matching(choices, |c| c.id == wanted), &|| {
        matching(choices, |c| c.id.eq_ignore_ascii_case(wanted))
    });
    let number = wanted
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|number| numbered && (1..=choices.len()).contains(number));
    let by_name = either(matching(choices, |c| c.label == wanted), &|| {
        let wanted = bare(wanted);
        matching(choices, |c| bare(&c.label).eq_ignore_ascii_case(wanted))
    });
    let mut meant: Vec<usize> = by_id
        .into_iter()
        .chain(number.map(|number| number - 1))
        .chain(by_name)
        .collect();
    meant.sort_unstable();
    meant.dedup();
    let wanted = crate::printable(wanted);
    match meant[..] {
        [index] => Ok(index),
        [] => Err(Failure::helped(
            format!("{whose} has no {what} `{wanted}`"),
            format!(
                "give one of {whose}'s {what}s: {}",
                listed(choices, numbered)
            ),
        )),
        _ => {
            let candidates: Vec<Choice<'_>> = meant
                .iter()
                .map(|&index| Choice {
                    id: choices[index].id,
                    label: if numbered {
                        format!("{} {}", index + 1, choices[index].label)
                    } else {
                        choices[index].label.clone()
                    },
                })
                .collect();
            let ids = || {
                candidates
                    .iter()
                    .map(|c| format!("`{}`", crate::printable(c.id)))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            // A number that is also an id or a name can't be given again: then by name or id.
            let hint = match (numbered, number) {
                (true, None) => "say which by number".to_owned(),
                (true, Some(_)) => format!("say which by name, or by id ({})", ids()),
                (false, _) => format!("say which by id ({})", ids()),
            };
            Err(Failure::helped(
                format!(
                    "`{wanted}` could mean {} of {whose}'s {what}s, {}",
                    candidates.len(),
                    listed(&candidates, false)
                ),
                hint,
            ))
        }
    }
}

/// The choices in words: `1 [H128W], 2 Long flight`, or with no numbers the names alone.
pub(crate) fn listed(choices: &[Choice<'_>], numbered: bool) -> String {
    if choices.is_empty() {
        return "none".to_owned();
    }
    choices
        .iter()
        .enumerate()
        .map(|(index, c)| {
            if numbered {
                format!("{} {}", index + 1, c.label)
            } else {
                c.label.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The indices of the choices `is` holds for.
fn matching(choices: &[Choice<'_>], is: impl Fn(&Choice<'_>) -> bool) -> Vec<usize> {
    choices
        .iter()
        .enumerate()
        .filter(|(_, c)| is(c))
        .map(|(index, _)| index)
        .collect()
}

/// A name without surrounding space or brackets: `[H128W]` is `H128W`.
fn bare(name: &str) -> &str {
    let name = name.trim();
    name.strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or(name)
        .trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn choices<'a>(pairs: &[(&'a str, &str)]) -> Vec<Choice<'a>> {
        pairs
            .iter()
            .map(|&(id, label)| Choice {
                id,
                label: label.to_owned(),
            })
            .collect()
    }

    fn refusal(result: Result<usize, Failure>) -> String {
        match result {
            Err(Failure::Input(message)) => message,
            Err(Failure::Helped { message, help }) => {
                format!("{message}; help: {}", help.join("; "))
            }
            other => panic!("not refused as input: {:?}", other.map_err(|_| ())),
        }
    }

    #[test]
    fn a_configuration_is_called_by_its_name_or_its_motors() {
        let five = Some(Delay::Seconds(5.0));
        assert_eq!(
            configuration_label(" Long flight ", [("H128W", None)].into_iter()),
            "Long flight"
        );
        assert_eq!(
            configuration_label("", [("H128W", None)].into_iter()),
            "[H128W]"
        );
        assert_eq!(
            configuration_label("  ", [("D12", five), ("D12", five)].into_iter()),
            "[D12-5; D12-5]"
        );
        assert_eq!(configuration_label("", std::iter::empty()), "[no motors]");
        assert_eq!(motor_label("C6", Some(Delay::Seconds(6.5))), "C6-6.5");
        assert_eq!(motor_label("H999N", Some(Delay::Plugged)), "H999N-P");
        assert_eq!(motor_label("D12", Some(Delay::ZeroOrPlugged)), "D12-0");
        assert_eq!(motor_label("1266J760-19A", five), "1266J760-19A");
        assert_eq!(part_label("a1b2", ""), "a1b2");
        assert_eq!(part_label("a1b2", "Inner Tube"), "Inner Tube");
    }

    #[test]
    fn config_takes_an_id_a_number_or_a_name() {
        let list = choices(&[
            ("098D8C95", "[H999N]"),
            ("b38ea08b", "Long flight"),
            ("db9b08a1", "[G80T]"),
            ("3", "[G80T]"),
        ]);
        let pick = |wanted| pick(&list, wanted, true, "configuration", "the file");
        assert_eq!(pick("098D8C95").ok(), Some(0));
        assert_eq!(pick("098d8c95").ok(), Some(0));
        assert_eq!(pick("2").ok(), Some(1));
        // An id that reads as another's number means two configurations: neither is guessed.
        assert_eq!(
            refusal(pick("3")),
            "`3` could mean 2 of the file's configurations, 3 [G80T], 4 [G80T]; help: say which by \
             name, or by id (`db9b08a1`, `3`)"
        );
        assert_eq!(pick("long FLIGHT").ok(), Some(1));
        assert_eq!(pick("h999n").ok(), Some(0));
        assert_eq!(pick("[H999N]").ok(), Some(0));
        assert_eq!(
            refusal(pick("G80T")),
            "`G80T` could mean 2 of the file's configurations, 3 [G80T], 4 [G80T]; help: say which by \
             number"
        );
        assert_eq!(pick("4").ok(), Some(3));
        assert_eq!(
            refusal(pick("5")),
            "the file has no configuration `5`; help: give one of the file's configurations: 1 [H999N], 2 Long flight, \
             3 [G80T], 4 [G80T]"
        );
    }

    /// Ids that read as numbers, as a program writing a design's JSON may number its
    /// configurations from 0: each number means two configurations, and is refused.
    #[test]
    fn a_number_that_is_another_s_id_is_refused() {
        let list = choices(&[
            ("0", "[L1040LR-3]"),
            ("1", "[L1040LR-5]"),
            ("2", "[L1040LR-7]"),
        ]);
        let pick = |wanted| pick(&list, wanted, true, "configuration", "the design");
        assert_eq!(
            refusal(pick("2")),
            "`2` could mean 2 of the design's configurations, 2 [L1040LR-5], 3 [L1040LR-7]; help: say \
             which by name, or by id (`1`, `2`)"
        );
        assert_eq!(pick("3").ok(), Some(2));
        assert_eq!(pick("0").ok(), Some(0));
        assert_eq!(pick("l1040lr-5").ok(), Some(1));
    }

    #[test]
    fn mount_takes_an_id_or_a_name() {
        let list = choices(&[
            ("m1", "Inner Tube"),
            ("m2", "Booster mount"),
            ("m3", "Inner Tube"),
        ]);
        let pick = |wanted| pick(&list, wanted, false, "motor mount", "the design");
        assert_eq!(pick("m2").ok(), Some(1));
        assert_eq!(pick("booster mount").ok(), Some(1));
        assert_eq!(
            refusal(pick("Inner Tube")),
            "`Inner Tube` could mean 2 of the design's motor mounts, Inner Tube, Inner Tube; help: say \
             which by id (`m1`, `m3`)"
        );
        // No numbers for mounts: `1` is a name like any other.
        assert!(refusal(pick("1")).starts_with(
            "the design has no motor mount `1`; help: give one of the design's motor mounts: Inner Tube, \
             Booster mount"
        ));
    }
}
