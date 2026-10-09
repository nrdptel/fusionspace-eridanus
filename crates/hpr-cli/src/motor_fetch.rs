//! `hpr motors fetch`, and the motors `hpr sim` fetches: a motor the bundled catalog lacks, found
//! on ThrustCurve.org by name ([`hpr::hpr_net::on_demand`]) and kept in the cache, so it flies with
//! no network after ([M4.5b](https://hpr.fusionspace.co/decisions-and-roadmap.html#m4-5b),
//! motors on demand; [ADR-154](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0154-motors-fetched-from-thrustcurve-by-name.md),
//! the decision on fetching them). Offline with no copy in the cache, the refusal names the exact
//! command that fetches it.

use hpr::Motor;
use hpr::hpr_net::NetError;
use hpr::hpr_net::on_demand::{self, FindError, Found, Wanted};
use hpr::hpr_net::thrustcurve::{self, DataFile, Format, ThrustCurveError};

use crate::console::Level;
use crate::output::{FileFormat, MotorFetch, ThrustCurveMotor};
use crate::units::inches_figure;
use crate::weather::{client, now_s, read_from, read_from_line};
use crate::{Failure, Out, printable};

/// `hpr motors fetch`'s arguments.
#[derive(Debug, clap::Args)]
pub struct FetchArgs {
    /// The motor: its designation (F27R/L) or common name (F27), as ThrustCurve.org spells it
    pub motor: String,
    /// Its manufacturer, by name or abbreviation, where the name alone matches several motors
    #[arg(long)]
    pub manufacturer: Option<String>,
    /// A ThrustCurve.org data file to take ahead of the usual order, by its id, when the motor has
    /// it and it reads
    #[arg(long, value_name = "ID")]
    pub file: Option<String>,
    /// Answer from the cache only, never the network
    #[arg(long)]
    pub offline: bool,
}

/// Runs `hpr motors fetch`.
pub(crate) fn run(args: &FetchArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let mut wanted = match &args.manufacturer {
        Some(maker) => Wanted::by(maker, &args.motor),
        None => Wanted::named(&args.motor),
    };
    if let Some(file) = &args.file {
        wanted = wanted.preferring(file);
    }
    let (motor, fetched) = fetch(&wanted, args.offline, "hpr motors fetch")?;
    let flies = sim_lines(&args.motor, &wanted, &fetched, args.offline);
    let document = MotorFetch {
        attribution: vec![thrustcurve::ATTRIBUTION.to_owned()],
        motor: fetched,
    };
    let lines = [
        format!(
            "{} ({}), from ThrustCurve.org: {}",
            printable(&document.motor.designation),
            printable(&document.motor.manufacturer),
            read_from_line(&document.motor.read_from).to_lowercase()
        ),
        format!("  curve file       {}", file_words(&document.motor)),
        format!(
            "  case             {:.0} × {:.0} mm ({} × {} in)",
            motor.diameter_m() * 1000.0,
            motor.length_m() * 1000.0,
            inches_figure(motor.diameter_m(), 2),
            inches_figure(motor.length_m(), 1)
        ),
    ];
    to.emit(&document, |out, diagnostics| {
        lines.iter().try_for_each(|line| writeln!(out, "{line}"))?;
        flies
            .iter()
            .try_for_each(|(level, line)| diagnostics.line(*level, line))?;
        writeln!(out, "{}", document.attribution.join(" "))
    })
}

/// What flies the motor fetched: `hpr sim --motor` names a motor alone, so a fetch by maker also
/// caches the search for its designation alone, when that finds the same motor. A name the
/// bundled catalog has flies the catalog's curve, which comes first. A file taken by `--file`
/// flies only for a `.ork` motor HPR Sim matches to it: `hpr sim --motor` takes files in the usual
/// order. A `--file` file not taken says so, ahead of the usual line.
///
/// Each is a line with its prefix: what flies it a `help:` line, a file not taken a `note:`.
fn sim_lines(
    name: &str,
    wanted: &Wanted,
    fetched: &ThrustCurveMotor,
    offline: bool,
) -> Vec<(Level, String)> {
    match &wanted.prefer {
        Some(asked) if *asked == fetched.file_id => {
            return vec![(
                Level::Help,
                "kept in the cache: a .ork motor HPR Sim matches to this file flies it, offline too"
                    .to_owned(),
            )];
        }
        Some(asked) => {
            let mut unasked = wanted.clone();
            unasked.prefer = None;
            let mut lines = vec![(
                Level::Note,
                format!(
                    "file {} was not taken: it is not among the motor's files HPR Sim read, or \
                     makes no motor HPR Sim flies; this is the first by the usual order",
                    printable(asked)
                ),
            )];
            lines.extend(sim_lines(name, &unasked, fetched, offline));
            return lines;
        }
        None => {}
    }
    let alone = if wanted.manufacturer.is_none() {
        Some(name.to_owned())
    } else {
        let designation = Wanted::named(&fetched.designation);
        fetch(&designation, offline, "hpr motors fetch")
            .ok()
            .filter(|(_, again)| again.motor_id == fetched.motor_id)
            .map(|_| fetched.designation.clone())
    };
    let sim = |alone: &str| shell_word(alone).map(|word| format!("hpr sim --motor {word}"));
    let line = match alone {
        Some(alone) if Motor::from_catalog(&alone).is_ok() => format!(
            "the bundled catalog has {} too, and `hpr sim --motor` flies the catalog's curve",
            printable(&alone)
        ),
        Some(alone) if let Some(sim) = sim(&alone) => {
            format!("kept in the cache: `{sim}` flies it, offline too")
        }
        _ => format!(
            "kept in the cache: a .ork naming it {} flies it, offline too",
            printable(&wanted.words())
        ),
    };
    vec![(Level::Help, line)]
}

/// `wanted`, found on ThrustCurve.org through the platform's cache (offline, the cache alone),
/// with the first file, in [`on_demand::find`]'s order, that makes a motor HPR Sim flies; `command`
/// names the command running, for a refusal.
pub(crate) fn fetch(
    wanted: &Wanted,
    offline: bool,
    command: &str,
) -> Result<(Motor, ThrustCurveMotor), Failure> {
    let client = client(offline, command)?;
    let build = |file: &DataFile| -> Result<Motor, String> {
        let text = file.text().map_err(|error| error.to_string())?;
        match file.format {
            Format::Rasp => Motor::from_eng(&text),
            Format::RockSim => Motor::from_rse(&text),
            // `find_with` asks for these two formats alone.
            other => return Err(format!("{} is not a format HPR Sim reads", other.as_str())),
        }
        .map_err(|error| error.to_string())
    };
    let (found, motor) = on_demand::find_with(&client, wanted, now_s(), build)
        .map_err(|error| refused(wanted, &error))?;
    Ok((motor, described(&found)?))
}

/// The command that fetches `wanted`, if every word of it is one any shell takes as it is
/// ([`shell_word`]).
pub(crate) fn command(wanted: &Wanted) -> Option<String> {
    let name = shell_word(&wanted.name)?;
    let file = match &wanted.prefer {
        Some(file) => format!("--file {} ", shell_word(file)?),
        None => String::new(),
    };
    Some(match &wanted.manufacturer {
        Some(maker) => format!(
            "hpr motors fetch --manufacturer {} {file}{name}",
            shell_word(maker)?
        ),
        None => format!("hpr motors fetch {file}{name}"),
    })
}

/// Why `wanted` wasn't fetched: offline with no copy, or the network failing, names the command
/// that fetches it. ThrustCurve's words and the file's reach the terminal as [`printable`] text.
pub(crate) fn refused(wanted: &Wanted, error: &FindError) -> Failure {
    let hint = || match command(wanted) {
        Some(command) => {
            format!("with a network connection, `{command}` fetches it into the cache")
        }
        None => "with a network connection, `hpr motors fetch` given its manufacturer and \
                 designation fetches it into the cache"
            .to_owned(),
    };
    let words = wanted.words();
    if error.is_not_cached() {
        let why = if crate::weather::offline_by_environment() {
            format!(" ({} is set)", crate::weather::OFFLINE_VARIABLE)
        } else {
            String::new()
        };
        Failure::helped(
            printable(&format!(
                "{words} is not in HPR Sim's cache of ThrustCurve.org, and the run is offline{why}"
            )),
            printable(&hint()),
        )
    } else if matches!(
        error,
        FindError::ThrustCurve(ThrustCurveError::Net(NetError::Transport { .. }))
    ) {
        Failure::helped(printable(&format!("{words}: {error}")), printable(&hint()))
    } else {
        Failure::Input(printable(&format!("{words}: {error}")))
    }
}

/// The output's record of a motor found.
fn described(found: &Found) -> Result<ThrustCurveMotor, Failure> {
    let last = found
        .fetched
        .last()
        .ok_or_else(|| Failure::Input("ThrustCurve.org's answer was not read".to_owned()))?;
    Ok(ThrustCurveMotor {
        manufacturer: found
            .record
            .manufacturer_abbrev
            .clone()
            .unwrap_or_else(|| found.record.manufacturer.clone()),
        designation: found.record.designation.clone(),
        common_name: found.record.common_name.clone(),
        motor_id: found.record.motor_id.clone(),
        file_id: found.file.simfile_id.clone(),
        format: match found.file.format {
            Format::RockSim => FileFormat::Rse,
            _ => FileFormat::Eng,
        },
        measured_by: found.file.source.clone(),
        license: found.file.license.clone(),
        page: found.file.source_url.clone(),
        read_from: read_from(last),
    })
}

/// A curve file in words, as [`printable`] text: `5f42…, RockSim (.rse), from a user, public
/// domain`.
pub(crate) fn file_words(motor: &ThrustCurveMotor) -> String {
    let format = match motor.format {
        FileFormat::Eng => "RASP (.eng)",
        FileFormat::Rse => "RockSim (.rse)",
    };
    let measured = match motor.measured_by.as_deref() {
        Some("cert") => "from a certification test".to_owned(),
        Some("mfr") => "from the manufacturer".to_owned(),
        Some("user") => "from a user".to_owned(),
        Some(other) => format!("from `{other}`"),
        None => "its source not stated".to_owned(),
    };
    let license = match motor.license.as_deref() {
        Some("PD") => "public domain".to_owned(),
        Some(other) => format!("license `{other}`"),
        None => "no license stated".to_owned(),
    };
    printable(&format!(
        "{}, {format}, {measured}, {license}",
        motor.file_id
    ))
}

/// `word` as one word that POSIX shells, PowerShell and Windows' `cmd` all take as it is: as it
/// is when it holds only letters, digits and `/._+-`, in double quotes when it also holds spaces;
/// `None` for anything else, or a word that starts with `-` and would read as an option.
pub(crate) fn shell_word(word: &str) -> Option<String> {
    let plain = |c: char| c.is_ascii_alphanumeric() || "/._+-".contains(c);
    if word.is_empty() || word.starts_with('-') || !word.chars().all(|c| plain(c) || c == ' ') {
        return None;
    }
    Some(if word.contains(' ') {
        format!("\"{word}\"")
    } else {
        word.to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_every_shell_takes_as_it_is_or_none() {
        assert_eq!(shell_word("F27R/L").as_deref(), Some("F27R/L"));
        assert_eq!(
            shell_word("Cesaroni Technology").as_deref(),
            Some("\"Cesaroni Technology\"")
        );
        for unsafe_word in [
            "it's",
            "x & calc & x",
            "a\"b",
            "$(id)",
            "-h",
            "",
            "a\u{1b}b",
        ] {
            assert_eq!(shell_word(unsafe_word), None, "{unsafe_word:?}");
        }
    }

    #[test]
    fn the_command_names_the_maker_when_given() {
        assert_eq!(
            command(&Wanted::named("F27R/L")).as_deref(),
            Some("hpr motors fetch F27R/L")
        );
        assert_eq!(
            command(&Wanted::by("Cesaroni Technology", "1266J760-19A")).as_deref(),
            Some("hpr motors fetch --manufacturer \"Cesaroni Technology\" 1266J760-19A")
        );
        assert_eq!(command(&Wanted::by("x & calc", "H128W")), None);
    }

    /// A control character from ThrustCurve's answer, such as an escape, never reaches the
    /// terminal.
    #[test]
    fn thrustcurve_text_reaches_the_terminal_printable() {
        let error = FindError::Ambiguous {
            wanted: "G41".to_owned(),
            matches: 2,
            candidates: vec!["A\u{1b}]0;x\u{7} G41".to_owned(), "B G41".to_owned()],
        };
        let Failure::Input(message) = refused(&Wanted::named("G41"), &error) else {
            panic!("an input failure");
        };
        assert!(!message.chars().any(char::is_control), "{message:?}");
        let motor = ThrustCurveMotor {
            manufacturer: "A".to_owned(),
            designation: "G41".to_owned(),
            common_name: None,
            motor_id: "e00000000000000000000001".to_owned(),
            file_id: "f\u{1b}[2J".to_owned(),
            format: FileFormat::Eng,
            measured_by: Some("x\u{1b}]0;owned\u{7}".to_owned()),
            license: Some("\u{9b}31m".to_owned()),
            page: None,
            read_from: crate::output::ReadFrom::Network {
                fetched_at_unix_s: 0,
            },
        };
        assert!(!file_words(&motor).chars().any(char::is_control));
    }
}
