//! What `--help` and `-h` end with: two or three examples of real use, then where the guide is,
//! as the product system's `cli.md` (*Help*) orders a command's help: one line saying what it
//! does, `Usage:`, the commands, the options, the examples, the docs.
//!
//! `--help` gives each example with a line saying what it does; `-h`, the short version, gives
//! the command lines alone. Both end with the guide's page for the command. The commands are in
//! the Literal role and the section title in the Heading role, as clap's own sections are
//! ([`crate::fs_style::CLAP_STYLES`]); clap drops the styles when the stream gets no color.

use clap::builder::StyledStr;

use crate::fs_style;

/// The guide's page on the command line.
pub(crate) const GUIDE: &str = "https://hpr.fusionspace.co/cli.html";

/// One example: what it does, and the line to type.
pub(crate) struct Example {
    /// What it does, in a few words.
    pub(crate) what: &'static str,
    /// The command line, as typed.
    pub(crate) line: &'static str,
}

/// A command's examples and its section of the guide, by its path of names under `hpr`.
pub(crate) struct CommandHelp {
    /// The command's names under `hpr`: `[]` for `hpr` itself, `["motors", "list"]`.
    pub(crate) path: &'static [&'static str],
    /// Its examples, two or three.
    pub(crate) examples: &'static [Example],
    /// Its section of the guide: the anchor on [`GUIDE`], or `""` for the page's top.
    pub(crate) anchor: &'static str,
}

/// Every available command's examples and guide section. A test holds that each example parses
/// as `hpr` would read it, and that every command `hpr` runs has an entry.
pub(crate) const COMMANDS: &[CommandHelp] = &[
    CommandHelp {
        path: &[],
        examples: &[
            Example {
                what: "Fly a design on the motor it was saved with",
                line: "hpr sim my-rocket.ork",
            },
            Example {
                what: "Fly it on another motor, in a 4 m/s wind from the west",
                line: "hpr sim my-rocket.ork --motor H170M --delay 10 --wind 4 --wind-from 270",
            },
            Example {
                what: "Work out a catalog motor's figures",
                line: "hpr motors show H170M",
            },
        ],
        anchor: "",
    },
    CommandHelp {
        path: &["sim"],
        examples: &[
            Example {
                what: "Fly a design on the motor it was saved with",
                line: "hpr sim my-rocket.ork",
            },
            Example {
                what: "Fly it on another motor, in a 4 m/s wind from the west",
                line: "hpr sim my-rocket.ork --motor H170M --delay 10 --wind 4 --wind-from 270",
            },
            Example {
                what: "Save its recording and draw its altitude, speed and acceleration",
                line: "hpr sim my-rocket.ork --export flight.csv --plot flight.svg",
            },
        ],
        anchor: "hpr-sim",
    },
    CommandHelp {
        path: &["mc"],
        examples: &[
            Example {
                what: "Fly a design 200 times, its wind scattered, and see where it lands",
                line: "hpr mc my-rocket.ork --runs 200 --wind 4 --wind-sd 0.25 --wind-from-sd 15",
            },
            Example {
                what: "Scatter the motor's impulse by 3% and save every flight",
                line: "hpr mc my-rocket.ork --runs 1000 --impulse-sd 0.03 --export flights.csv",
            },
        ],
        anchor: "hpr-mc",
    },
    CommandHelp {
        path: &["convert"],
        examples: &[
            Example {
                what: "Write a catalog motor's thrust curve as a .eng file",
                line: "hpr convert H170M H170M.eng",
            },
            Example {
                what: "Turn an OpenRocket design into an HPR design",
                line: "hpr convert my-rocket.ork my-rocket.hpr",
            },
        ],
        anchor: "hpr-convert",
    },
    CommandHelp {
        path: &["motors"],
        examples: &[
            Example {
                what: "List the catalog's 38 mm H motors",
                line: "hpr motors list --class H --diameter 38",
            },
            Example {
                what: "Work out a motor's figures from its thrust curve",
                line: "hpr motors show H170M",
            },
            Example {
                what: "Find an L motor in stock",
                line: "hpr motors search --class L --in-stock",
            },
        ],
        anchor: "hpr-motors",
    },
    CommandHelp {
        path: &["motors", "list"],
        examples: &[
            Example {
                what: "List the catalog's 38 mm H motors",
                line: "hpr motors list --class H --diameter 38",
            },
            Example {
                what: "List Cesaroni's motors",
                line: "hpr motors list --manufacturer Cesaroni",
            },
        ],
        anchor: "listing-the-catalog",
    },
    CommandHelp {
        path: &["motors", "show"],
        examples: &[
            Example {
                what: "Work out a catalog motor's figures",
                line: "hpr motors show H170M",
            },
            Example {
                what: "Read a motor file",
                line: "hpr motors show my-motor.eng",
            },
        ],
        anchor: "a-motors-figures",
    },
    CommandHelp {
        path: &["motors", "search"],
        examples: &[
            Example {
                what: "Find an L motor in stock",
                line: "hpr motors search --class L --in-stock",
            },
            Example {
                what: "Find 54 mm motors in stock for at most $150",
                line: "hpr motors search --diameter 54 --in-stock --max-price 150",
            },
        ],
        anchor: "motors-you-can-buy",
    },
    CommandHelp {
        path: &["motors", "fetch"],
        examples: &[
            Example {
                what: "Fetch a motor the bundled catalog lacks, so `hpr sim` flies it offline",
                line: "hpr motors fetch F27",
            },
            Example {
                what: "Name its maker where the name alone matches several motors",
                line: "hpr motors fetch G80 --manufacturer AeroTech",
            },
        ],
        anchor: "motors-from-thrustcurveorg",
    },
    CommandHelp {
        path: &["weather"],
        examples: &[
            Example {
                what: "Fetch the forecast for a launch at Black Rock and save its profile",
                line: "hpr weather open-meteo --latitude 40.865 --longitude -119.063 \
                       --time 2026-09-26T16:00Z -o profile.json",
            },
            Example {
                what: "Read a balloon sounding launched before it",
                line: "hpr weather wyoming --station 72489 --time 2026-09-26T16:00Z",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["weather", "open-meteo"],
        examples: &[
            Example {
                what: "Fetch the forecast for a launch at Black Rock and save its profile",
                line: "hpr weather open-meteo --latitude 40.865 --longitude -119.063 \
                       --time 2026-09-26T16:00Z -o profile.json",
            },
            Example {
                what: "Ask the archive of past forecasts for a launch already flown",
                line: "hpr weather open-meteo --latitude 40.865 --longitude -119.063 \
                       --time 2025-09-20T16:00Z --historical",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["weather", "wyoming"],
        examples: &[
            Example {
                what: "Read the Reno balloon sounding launched before a launch",
                line: "hpr weather wyoming --station 72489 --time 2026-09-26T16:00Z",
            },
            Example {
                what: "Ask for it decoded from BUFR, a row every second or two of the ascent",
                line: "hpr weather wyoming --station 72489 --time 2026-09-26T16:00Z --bufr",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["weather", "gfs"],
        examples: &[
            Example {
                what: "Read a GFS answer saved earlier, at a site",
                line: "hpr weather gfs --latitude 40.865 --longitude -119.063 --from gfs.grib2",
            },
            Example {
                what: "Fetch one run's forecast for 12 hours after its start",
                line: "hpr weather gfs --latitude 40.865 --longitude -119.063 \
                       --cycle 2026-09-26T00Z --hour 12",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["weather", "rap"],
        examples: &[
            Example {
                what: "Read a RAP answer saved earlier, at a site",
                line: "hpr weather rap --latitude 40.865 --longitude -119.063 --from rap.grib2",
            },
            Example {
                what: "Fetch one run's forecast for 3 hours after its start",
                line: "hpr weather rap --latitude 40.865 --longitude -119.063 \
                       --cycle 2026-09-26T12Z --hour 3",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["weather", "era5"],
        examples: &[
            Example {
                what: "Read an ERA5 file at a site and time",
                line: "hpr weather era5 era5.nc --latitude 40.865 --longitude -119.063 \
                       --time 2020-02-22T13:00Z",
            },
            Example {
                what: "Save the profile for `hpr sim`",
                line: "hpr weather era5 era5.nc --latitude 40.865 --longitude -119.063 \
                       --time 2020-02-22T13:00Z -o profile.json",
            },
        ],
        anchor: "hpr-weather",
    },
    CommandHelp {
        path: &["analyze"],
        examples: &[
            Example {
                what: "Read a PerfectFlite log's apogee, top speed and timeline",
                line: "hpr analyze flight.pf2",
            },
            Example {
                what: "The same as one JSON document",
                line: "hpr analyze flight.pf2 --json",
            },
        ],
        anchor: "hpr-analyze",
    },
    CommandHelp {
        path: &["completions"],
        examples: &[
            Example {
                what: "Complete `hpr`'s commands and options in zsh",
                line: "hpr completions zsh > ~/.zfunc/_hpr",
            },
            Example {
                what: "In bash",
                line: "hpr completions bash > ~/.local/share/bash-completion/completions/hpr",
            },
        ],
        anchor: "hpr-completions",
    },
];

/// The guide's address for `anchor`.
pub(crate) fn guide(anchor: &str) -> String {
    if anchor.is_empty() {
        GUIDE.to_owned()
    } else {
        format!("{GUIDE}#{anchor}")
    }
}

/// What `-h` ends with: the examples' lines, then the guide.
pub(crate) fn short(help: &CommandHelp) -> StyledStr {
    let mut text = section("Examples:");
    for example in help.examples {
        text.push_str(&format!("  {}\n", literal(example.line)));
    }
    text.push_str(&format!("\n{}", guide_line(help)));
    StyledStr::from(text)
}

/// What `--help` ends with: each example with what it does, then the guide.
pub(crate) fn long(help: &CommandHelp) -> StyledStr {
    let mut text = section("Examples:");
    for example in help.examples {
        text.push_str(&format!(
            "  {}:\n      {}\n",
            example.what,
            literal(example.line)
        ));
    }
    text.push_str(&format!("\n{}", guide_line(help)));
    StyledStr::from(text)
}

/// The section's title, in the Heading role, and a newline.
fn section(title: &str) -> String {
    format!(
        "{}{title}{}\n",
        fs_style::HEADING.render(),
        fs_style::HEADING.render_reset()
    )
}

/// A command line, in the Literal role.
fn literal(line: &str) -> String {
    format!(
        "{}{line}{}",
        fs_style::LITERAL.render(),
        fs_style::LITERAL.render_reset()
    )
}

/// The last line: where the guide's section on the command is.
fn guide_line(help: &CommandHelp) -> String {
    format!(
        "{}Guide:{} {}",
        fs_style::HEADING.render(),
        fs_style::HEADING.render_reset(),
        guide(help.anchor)
    )
}

/// `command` with each command's examples and guide after its help, [`COMMANDS`]'s.
pub(crate) fn with_examples(command: clap::Command) -> clap::Command {
    fn under(command: clap::Command, path: &[&str]) -> clap::Command {
        let command = match COMMANDS.iter().find(|help| help.path == path) {
            Some(help) => command.after_help(short(help)).after_long_help(long(help)),
            None => command,
        };
        let names: Vec<String> = command
            .get_subcommands()
            .map(|sub| sub.get_name().to_owned())
            .collect();
        names.iter().fold(command, |command, name| {
            let path: Vec<&str> = path.iter().copied().chain([name.as_str()]).collect();
            command.mut_subcommand(name, |sub| under(sub, &path))
        })
    }
    under(command, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words of an example's line as a shell passes them, up to a redirection.
    fn words(line: &str) -> Vec<&str> {
        line.split_whitespace()
            .take_while(|word| *word != ">")
            .collect()
    }

    /// Each example parses as `hpr` reads it, and runs the command it is listed under or one under
    /// that.
    #[test]
    fn every_example_parses_as_its_own_command() {
        for help in COMMANDS {
            for example in help.examples {
                let words = words(example.line);
                let matches = crate::command()
                    .try_get_matches_from(&words)
                    .unwrap_or_else(|error| panic!("{}: {error}", example.line));
                let mut path = Vec::new();
                let mut at = &matches;
                while let Some((name, sub)) = at.subcommand() {
                    path.push(name);
                    at = sub;
                }
                // `hpr`'s own examples, and a group's, run the commands under it.
                assert!(
                    path.starts_with(help.path) && !path.is_empty(),
                    "{}",
                    example.line
                );
                assert!(!example.what.is_empty(), "{}", example.line);
            }
        }
    }

    /// Every command `hpr` runs, and `hpr` itself, has two or three examples and a guide
    /// section; a command not available yet has none, and no entry names a command `hpr`
    /// doesn't have.
    #[test]
    fn every_available_command_has_examples_and_a_guide() {
        fn walk(command: &clap::Command, path: &mut Vec<String>, found: &mut Vec<Vec<String>>) {
            found.push(path.clone());
            for sub in command.get_subcommands() {
                let name = sub.get_name().to_owned();
                if name == "help" {
                    continue;
                }
                if path.is_empty()
                    && matches!(
                        crate::registry::availability(&name),
                        Some(crate::registry::Availability::Planned { .. })
                    )
                {
                    assert!(
                        COMMANDS
                            .iter()
                            .all(|help| help.path.first() != Some(&name.as_str())),
                        "{name} isn't available, so it has no examples"
                    );
                    continue;
                }
                path.push(name);
                walk(sub, path, found);
                path.pop();
            }
        }
        let mut found = Vec::new();
        walk(&crate::command(), &mut Vec::new(), &mut found);
        for path in &found {
            let help = COMMANDS
                .iter()
                .find(|help| help.path == path.as_slice())
                .unwrap_or_else(|| panic!("hpr {path:?} has no examples"));
            assert!(
                (2..=3).contains(&help.examples.len()),
                "hpr {path:?}: {} examples",
                help.examples.len()
            );
        }
        assert_eq!(found.len(), COMMANDS.len(), "an entry names no command");
    }

    /// Each guide section is a heading of the guide's page, by mdBook's anchor rule: lower case,
    /// spaces to hyphens, letters, digits, hyphens and underscores kept.
    #[test]
    fn every_guide_section_is_on_the_page() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/cli.md"),
        )
        .unwrap();
        let anchors: Vec<String> = page
            .lines()
            .filter_map(|line| {
                line.trim_start_matches('#')
                    .strip_prefix(' ')
                    .filter(|_| line.starts_with('#'))
            })
            .map(|heading| {
                heading
                    .to_lowercase()
                    .chars()
                    .filter_map(|c| match c {
                        ' ' => Some('-'),
                        c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
                        _ => None,
                    })
                    .collect()
            })
            .collect();
        for help in COMMANDS.iter().filter(|help| !help.anchor.is_empty()) {
            assert!(
                anchors.iter().any(|anchor| anchor == help.anchor),
                "{}: no heading on docs/cli.md",
                help.anchor
            );
        }
    }

    /// The short help gives the lines alone, the long one what each does; both end with the
    /// guide, and without color carry no escape code.
    #[test]
    fn short_and_long_end_with_the_guide() {
        let sim = COMMANDS.iter().find(|help| help.path == ["sim"]).unwrap();
        let plain_short = short(sim).to_string();
        let plain_long = long(sim).to_string();
        assert!(
            plain_short.starts_with("Examples:\n  hpr sim my-rocket.ork\n"),
            "{plain_short}"
        );
        assert!(
            plain_long.starts_with(
                "Examples:\n  Fly a design on the motor it was saved with:\n      hpr sim my-rocket.ork\n"
            ),
            "{plain_long}"
        );
        for text in [&plain_short, &plain_long] {
            assert!(
                text.ends_with("\n\nGuide: https://hpr.fusionspace.co/cli.html#hpr-sim"),
                "{text}"
            );
            assert!(!text.contains('\u{1b}'), "{text}");
        }
        let colored = short(sim).ansi().to_string();
        assert!(
            colored.contains("\u{1b}[1m\u{1b}[34mhpr sim my-rocket.ork\u{1b}[0m"),
            "{colored:?}"
        );
        assert!(
            colored.starts_with("\u{1b}[1mExamples:\u{1b}[0m"),
            "{colored:?}"
        );
    }
}
