//! The `hpr` command-line tool: its commands, the types their `--json` output takes, and the
//! registry the README's command table is generated from.
//!
//! **Guide:** [The command line][guide-cli] says what each command does, shows its output, and
//! lists the exit codes. This crate is the tool's own code; programs should use the [`hpr`]
//! library instead, which this crate calls.
//!
//! [guide-cli]: https://hpr.fusionspace.co/cli.html
//!
//! [`run`] is the whole tool: the `hpr` binary is a `main` that hands it the process's arguments
//! and exits with the [`Exit`] it returns. Tests and `cargo xtask cli` call it the same way, so the
//! output they check is the output a user sees.
//!
//! Every command prints text for a person, or, with `--json`, exactly one JSON document on
//! standard output: the command's own output type ([`output`]) when it succeeds, and an
//! [`output::ErrorDocument`] when it doesn't. The text puts the result alone on standard output
//! and every `error:`, `warning:`, `note:` and `help:` line on standard error ([`console`]). [`schemas`] generates the published JSON Schema
//! of each, which `cargo xtask cli` writes to `schema/cli/`.
//!
//! Commands whose milestone hasn't come yet are registered with the rest, so that `hpr --help`,
//! the completions and the README list them, and they refuse with [`Exit::NotAvailable`] and the
//! milestone that brings them ([`registry::availability`]).

#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "the command-line tool reads files; it is not part of the pure core"
)]

pub mod analyze;
mod choose;
pub mod console;
pub mod convert;
mod convert_design;
#[allow(
    missing_docs,
    reason = "copied unchanged from the product system (ADR-164), whose `ColorWhen` variants \
              carry no doc comments; `--color`'s help says what they mean"
)]
// Kept byte for byte as the system writes it, so `cargo fmt` doesn't rewrap it.
#[rustfmt::skip]
pub mod fs_style;
pub mod mc;
mod mc_text;
pub mod motor_fetch;
pub mod motor_search;
pub mod motors;
mod openrocket_curves;
pub mod output;
mod plot;
pub mod registry;
pub mod sim;
mod sim_text;
pub mod weather;

use std::ffi::OsString;
use std::io::{self, Write};
use std::path::Path;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::Shell;
use serde::Serialize;

use crate::console::{Console, Diagnostics, Level, Paint, Paints};
use crate::fs_style::ColorWhen;
use crate::output::{Completions, ErrorDocument, ErrorKind};
use crate::registry::Availability;

/// What `hpr --version` prints after the name: the version and the designation (ADR-164).
/// A test holds it to [`hpr::hpr_core::tool`]'s.
const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " · FS-ACHERNAR · SW · TOOL 001");

/// The name `hpr --version` prints before [`VERSION`], and each command's `--version` too:
/// `FusionSpace HPR` ([ADR-199][adr-199] §3), so the line is the stamp files carry
/// ([`hpr::hpr_core::tool::stamp`]). The command itself stays `hpr`.
///
/// [adr-199]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md
const DISPLAY_NAME: &str = hpr::hpr_core::tool::NAME;

/// The command line as clap parses it: [`Cli`]'s derived command, with [`DISPLAY_NAME`] on it and
/// on every command under it. Without that, clap names a command's version by its
/// binary's name and the command's, joined with a hyphen.
pub fn command() -> clap::Command {
    fn named(command: clap::Command) -> clap::Command {
        command.display_name(DISPLAY_NAME).mut_subcommands(named)
    }
    named(Cli::command())
}

/// How a run of `hpr` ended: its process exit status.
///
/// | code | meaning |
/// |---|---|
/// | 0 | the command did what was asked |
/// | 1 | it couldn't: an input was missing, unreadable or refused |
/// | 2 | the command line was wrong: an unknown command or option, or a missing argument |
/// | 3 | the command is registered but its milestone hasn't come yet |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    /// The command did what was asked.
    Success,
    /// The command couldn't do it: an input was missing, unreadable or refused.
    Failure,
    /// The command line was wrong. Clap's own convention, which its usage errors keep.
    Usage,
    /// The command exists but isn't available yet.
    NotAvailable,
}

impl Exit {
    /// The process exit status.
    pub fn code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::Usage => 2,
            Self::NotAvailable => 3,
        }
    }
}

/// The command line: global options and one command.
#[derive(Debug, Parser)]
#[command(
    name = "hpr",
    version = VERSION,
    about = "FusionSpace HPR · Sim: flight simulation for hobby and high-power rockets",
    long_about = "FusionSpace HPR · Sim: flight simulation for hobby and high-power rockets.\n\n\
                  Every command prints text, or one JSON document with --json. \
                  Commands marked \"not available yet\" name the milestone that brings them. \
                  Guide: https://hpr.fusionspace.co/cli.html",
    propagate_version = true,
    arg_required_else_help = true,
    styles = fs_style::CLAP_STYLES
)]
pub struct Cli {
    /// Print one JSON document on standard output instead of text; errors too.
    #[arg(long, global = true)]
    pub json: bool,
    /// When to color: auto colors a stream that is a terminal; NO_COLOR, FORCE_COLOR and
    /// CLICOLOR_FORCE apply under auto. JSON output never has color
    #[arg(long, global = true, value_enum, default_value_t = ColorWhen::Auto, value_name = "WHEN")]
    pub color: ColorWhen,
    /// The command to run.
    #[command(subcommand)]
    pub command: Command,
}

/// The commands, in the order `hpr --help` lists them.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Fly a .ork, an .hpr or .hprz design, or a rocket's .json from a rail and print its flight;
    /// export its recording
    Sim(sim::SimArgs),
    /// Convert a motor file between .eng and .rse, or a catalog motor to either; or a design
    /// between .ork, .hpr and .hprz
    Convert(convert::ConvertArgs),
    /// Look up motors in the bundled catalog, read a .eng or .rse motor file, fetch a curve from
    /// ThrustCurve.org, or search vendors' stock and prices
    #[command(subcommand)]
    Motors(motors::MotorsCommand),
    /// Fetch a launch day's weather, or read a weather file, as a profile of air and wind
    #[command(subcommand)]
    Weather(weather::WeatherCommand),
    /// Fly a design many times, each flight's inputs scattered at random, and print how its
    /// apogee and landing spread; export every flight
    Mc(mc::McArgs),
    /// Search a design's parameters for a goal (not available yet)
    Optimize(Planned),
    /// Compare a flight log with its simulation (not available yet)
    Compare(Planned),
    /// Read a flight log and print its readings, with no design file
    Analyze(analyze::AnalyzeArgs),
    /// Diagnose what went wrong in a flight from its log (not available yet)
    Diagnose(Planned),
    /// Print a shell completion script for `hpr`
    Completions(CompletionsArgs),
}

impl Command {
    /// The command's name, as typed.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sim(_) => "sim",
            Self::Convert(_) => "convert",
            Self::Motors(_) => "motors",
            Self::Weather(_) => "weather",
            Self::Mc(_) => "mc",
            Self::Optimize(_) => "optimize",
            Self::Compare(_) => "compare",
            Self::Analyze(_) => "analyze",
            Self::Diagnose(_) => "diagnose",
            Self::Completions(_) => "completions",
        }
    }
}

/// The arguments of a command that isn't available yet: anything, so that the refusal names the
/// milestone instead of complaining about an option the command will one day have.
#[derive(Debug, clap::Args)]
pub struct Planned {
    /// Accepted and ignored until the command arrives.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub args: Vec<OsString>,
}

/// `hpr completions`'s arguments.
#[derive(Debug, clap::Args)]
pub struct CompletionsArgs {
    /// The shell to write the script for.
    #[arg(value_enum)]
    pub shell: Shell,
}

/// Why a command stopped. [`run`] reports it as text or as an [`ErrorDocument`].
#[derive(Debug)]
pub(crate) enum Failure {
    /// An input was missing, unreadable or refused.
    Input(String),
    /// An input was refused, and the user can do something about it: the message, then each
    /// thing to do, written as `help:` lines after the `error:` line.
    Helped {
        /// What went wrong.
        message: String,
        /// What to do, the most useful last.
        help: Vec<String>,
    },
    /// The command's milestone hasn't come yet.
    NotAvailable {
        /// The command.
        command: &'static str,
        /// The milestone that brings it.
        milestone: &'static str,
    },
    /// Standard output, or standard error beside it, couldn't be written, such as a closed pipe.
    Output(io::Error),
}

impl Failure {
    /// An input refused with one thing to do about it.
    pub(crate) fn helped(message: impl Into<String>, help: impl Into<String>) -> Self {
        Self::Helped {
            message: message.into(),
            help: vec![help.into()],
        }
    }
}

/// Where a command's output goes, and in which form.
pub(crate) struct Out<'a> {
    /// Standard output: the result alone.
    pub(crate) out: &'a mut dyn Write,
    /// Whether `--json` was given.
    pub(crate) json: bool,
    /// Standard error, for the text's `warning:`, `note:` and `help:` lines.
    pub(crate) diagnostics: Diagnostics<'a>,
}

impl Out<'_> {
    /// Writes `value` as one JSON document, or `text` when `--json` wasn't given, and flushes.
    /// `text` writes the result to standard output, its first argument, and its `warning:`,
    /// `note:` and `help:` lines to standard error, its second; a JSON document carries those
    /// in its own fields, so with `--json` standard error stays empty. Only a failure to write
    /// either stream is a [`Failure::Output`]; one on standard error waits until the result is
    /// written, and a closed standard error ends quietly.
    pub(crate) fn emit<T: Serialize>(
        &mut self,
        value: &T,
        text: impl FnOnce(&mut dyn Write, &mut Diagnostics<'_>) -> io::Result<()>,
    ) -> Result<(), Failure> {
        let written = if self.json {
            write_json(self.out, value)
        } else {
            text(self.out, &mut self.diagnostics)
        };
        written
            .and_then(|()| self.out.flush())
            .map_err(Failure::Output)?;
        // Standard error is reported only once the result is out. A reader that closed it
        // early stopped reading the messages, not the result, so that ends quietly, as a
        // closed standard output does.
        self.diagnostics.flush();
        match self.diagnostics.failure() {
            Some(error) if error.kind() != io::ErrorKind::BrokenPipe => {
                Err(Failure::Output(error))
            }
            _ => Ok(()),
        }
    }
}

/// Writes one pretty-printed JSON document, stamped with [`output::TOOL`], and a newline.
pub(crate) fn write_json<T: Serialize>(out: &mut dyn Write, value: &T) -> io::Result<()> {
    let stamped = output::Stamped {
        tool: output::TOOL,
        document: value,
    };
    serde_json::to_writer_pretty(&mut *out, &stamped).map_err(io::Error::from)?;
    writeln!(out)
}

/// `text` with each control character, such as an escape that would reach the terminal from a
/// vendor's page or a fetched answer, shown as `?`.
pub(crate) fn printable(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_control() { '?' } else { c })
        .collect()
}

/// Runs `hpr` as [`run`] does, but offline with `cache` as its cache: every fetch answers from
/// `cache` alone, whatever `--offline` says. For a caller that runs commands in-process and must
/// not reach the network, such as the documentation's examples; it holds for this call on this
/// thread only.
pub fn run_offline<I, T>(args: I, out: &mut dyn Write, err: &mut dyn Write, cache: &Path) -> Exit
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    weather::with_offline_cache(cache, || run(args, out, err))
}

/// Runs `hpr` on a command line, `args[0]` being the program's name, writing to `out` (standard
/// output) and `err` (standard error), both taken for pipes: [`run_with`] on [`Console::PIPED`].
pub fn run<I, T>(args: I, out: &mut dyn Write, err: &mut dyn Write) -> Exit
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    run_with(args, out, err, Console::PIPED)
}

/// Runs `hpr` on a command line, `args[0]` being the program's name, writing to `out` (standard
/// output) and `err` (standard error), which `console` describes; it and `--color` decide each
/// stream's color ([`console`]).
///
/// With `--json` anywhere on the line before a `--`, standard output gets exactly one JSON
/// document, success or failure, and standard error stays empty; `--help` and `--version` still
/// print text. That scan decides, not [`Cli::json`]: a command that isn't available yet takes
/// every argument after its name as its own, `--json` included. `--color` is scanned the same
/// way, so a usage error is colored as asked.
pub fn run_with<I, T>(args: I, out: &mut dyn Write, err: &mut dyn Write, console: Console) -> Exit
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    // Scanned before parsing so that a usage error can be reported as JSON too. After `--`,
    // `--json` is an argument, such as a file's name.
    let options: Vec<&OsString> = args.iter().skip(1).take_while(|arg| *arg != "--").collect();
    let json = options.iter().any(|arg| *arg == "--json");
    let paints = console.paints(color_flag(&options), json);
    let parsed = command()
        .try_get_matches_from(&args)
        .and_then(|matches| Cli::from_arg_matches(&matches))
        .map_err(|error| error.format(&mut command()));
    let cli = match parsed {
        Ok(cli) => cli,
        Err(error) => return usage(&error, json, paints, out, err),
    };
    let command = cli.command.name();
    let mut to = Out {
        out,
        json,
        diagnostics: Diagnostics::new(err, paints.err),
    };
    // The registry decides what refuses, so the table and the tool can't disagree.
    let outcome = match registry::availability(command) {
        Some(Availability::Planned { milestone }) => {
            Err(Failure::NotAvailable { command, milestone })
        }
        Some(Availability::Available { .. }) => match cli.command {
            Command::Motors(motors) => motors::run(&motors, &mut to),
            Command::Sim(args) => sim::run(&args, &mut to),
            Command::Mc(args) => mc::run(&args, &mut to),
            Command::Convert(args) => convert::run(&args, &mut to),
            Command::Analyze(args) => analyze::run(&args, &mut to),
            Command::Weather(weather) => weather::run(&weather, &mut to),
            Command::Completions(args) => completions(args.shell, &mut to),
            Command::Optimize(_) | Command::Compare(_) | Command::Diagnose(_) => {
                Err(Failure::Input(format!(
                    "hpr {command} is marked available in the command registry, but this build \
                 has no code for it"
                )))
            }
        },
        None => Err(Failure::Input(format!(
            "hpr {command} has no entry in the command registry"
        ))),
    };
    match outcome {
        Ok(()) => Exit::Success,
        Err(failure) => report(failure, command, json, paints.err, out, err),
    }
}

/// The last `--color` on the command line, as `--color WHEN` or `--color=WHEN`; auto when there
/// is none or clap will refuse its value.
fn color_flag(options: &[&OsString]) -> ColorWhen {
    use clap::ValueEnum;
    let mut flag = ColorWhen::Auto;
    let mut pairs = options.iter().map(|arg| arg.to_str()).peekable();
    while let Some(arg) = pairs.next() {
        let value = match arg {
            Some("--color") => pairs.peek().copied().flatten(),
            Some(other) => other.strip_prefix("--color="),
            None => None,
        };
        if let Some(value) = value {
            flag = ColorWhen::from_str(value, false).unwrap_or(ColorWhen::Auto);
        }
    }
    flag
}

/// Reports a failure on standard error, or as an [`ErrorDocument`] with `--json`.
fn report(
    failure: Failure,
    command: &'static str,
    json: bool,
    paint: Paint,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Exit {
    let (document, exit) = match failure {
        Failure::Input(message) => {
            let (message, help) = library_hint(message);
            (
                ErrorDocument::new(ErrorKind::Input, message, Some(command), None).with_help(help),
                Exit::Failure,
            )
        }
        Failure::Helped { message, help } => (
            ErrorDocument::new(ErrorKind::Input, message, Some(command), None).with_help(help),
            Exit::Failure,
        ),
        Failure::NotAvailable { command, milestone } => {
            let message = format!(
                "hpr {command} is not available yet: it arrives with milestone {milestone} \
                 (https://hpr.fusionspace.co/decisions-and-roadmap.html#{})",
                registry::anchor(milestone)
            );
            (
                ErrorDocument::new(
                    ErrorKind::NotAvailable,
                    message,
                    Some(command),
                    Some(milestone),
                ),
                Exit::NotAvailable,
            )
        }
        // A closed pipe (`hpr motors list | head -1`): the reader stopped reading, which is its
        // choice, not a failure, and whether it happens depends on the pipe's buffer. Anything
        // else goes to standard error, as JSON can't be written to the stream that just failed.
        Failure::Output(error) => {
            if error.kind() == io::ErrorKind::BrokenPipe {
                return Exit::Success;
            }
            let _ = paint.line(
                err,
                Level::Error,
                &format!("couldn't write the output: {error}"),
            );
            return Exit::Failure;
        }
    };
    // Nowhere is left to report a failure to write the report; the exit status still says it.
    let _ = if json {
        write_json(out, &document)
    } else {
        paint
            .line(err, Level::Error, &document.error.message)
            .and_then(|()| {
                document
                    .error
                    .help
                    .iter()
                    .try_for_each(|help| paint.line(err, Level::Help, help))
            })
    };
    exit
}

/// Reports clap's usage error, help or version text.
fn usage(
    error: &clap::Error,
    json: bool,
    paints: Paints,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Exit {
    let rendered = |paint: Paint| {
        let text = error.render();
        let text = if paint.on {
            text.ansi().to_string()
        } else {
            text.to_string()
        };
        clap_hints(&text, paint)
    };
    // Help and version requests are not errors: clap sends them to standard output with status 0.
    if !error.use_stderr() {
        let _ = write!(out, "{}", rendered(paints.out));
        return Exit::Success;
    }
    // As in `report`: the exit status is all that is left if this fails.
    let _ = if json {
        let text = clap_hints(&error.render().to_string(), Paint::default());
        let (message, help): (Vec<&str>, Vec<&str>) = text
            .lines()
            .partition(|line| !line.starts_with(HELP_PREFIX));
        let help = help
            .iter()
            .map(|line| line.trim_start_matches(HELP_PREFIX).to_owned())
            .collect();
        let mut message = message;
        message.dedup_by(|line, before| line.is_empty() && before.is_empty());
        let message = message.join("\n");
        let document =
            ErrorDocument::new(ErrorKind::Usage, message.trim(), None, None).with_help(help);
        write_json(out, &document)
    } else {
        write!(err, "{}", rendered(paints.err))
    };
    Exit::Usage
}

/// The hints the libraries end some of their messages with, after `"; "`: each is split off and
/// written as a `help:` line, as `product/cli.md` asks (ADR-174).
fn library_hints() -> impl Iterator<Item = &'static str> {
    [
        hpr::hpr_motor::motor::UNITS_HINT,
        hpr::hpr_atmos::error::INCOMPLETE_COLUMN_HINT,
    ]
    .into_iter()
}

/// A refusal's message with a library's hint at its end split off, or as it is.
fn library_hint(message: String) -> (String, Vec<String>) {
    for hint in library_hints() {
        if let Some(fact) = message.strip_suffix(&format!("; {hint}")) {
            return (fact.to_owned(), vec![hint.to_owned()]);
        }
    }
    (message, Vec::new())
}

/// How a `help:` line starts on a stream without color.
const HELP_PREFIX: &str = "help: ";

/// Clap's usage error with each of its hints as a `help:` line, as `product/cli.md` asks: its
/// indented `tip:` lines, its indented `[possible values: …]` and its closing "For more
/// information, try '--help'.", in `paint`.
///
/// Clap writes them in its own words and styles, which this matches as text: a test holds them
/// to what a usage error prints, so a clap release that words them otherwise fails it.
fn clap_hints(text: &str, paint: Paint) -> String {
    let help = paint.styled(fs_style::HELP, "help:");
    let tip = format!("  {}", paint.styled(fs_style::OK, "tip:"));
    let text = text.replace(&tip, &help).replace(
        "For more information, try ",
        &format!("{help} for more information, try "),
    );
    let mut lines: Vec<String> = text
        .split('\n')
        .map(|line| {
            match line
                .strip_prefix("  [possible values: ")
                .and_then(|values| values.strip_suffix(']'))
            {
                Some(values) => format!("{help} it takes one of {values}"),
                None => line.to_owned(),
            }
        })
        .collect();
    lines.dedup_by(|line, before| line.is_empty() && before.is_empty());
    lines.join("\n")
}

/// `hpr completions <shell>`.
fn completions(shell: Shell, to: &mut Out<'_>) -> Result<(), Failure> {
    let mut script = Vec::new();
    clap_complete::generate(shell, &mut command(), "hpr", &mut script);
    let script = String::from_utf8(script)
        .map_err(|error| Failure::Input(format!("the completion script isn't UTF-8: {error}")))?;
    let document = Completions {
        shell: shell.to_string(),
        script,
    };
    to.emit(&document, |out, _| {
        out.write_all(document.script.as_bytes())
    })
}

/// The JSON Schema of each `--json` output, by file name: what `cargo xtask cli` writes to
/// `schema/cli/`.
pub fn schemas() -> Vec<(&'static str, String)> {
    output::schemas()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_version_line_is_the_tools_version_and_designation() {
        assert_eq!(
            super::VERSION,
            format!(
                "{} · {}",
                hpr::hpr_core::tool::VERSION,
                hpr::hpr_core::tool::DESIGNATION
            )
        );
    }

    /// ADR-199 §3: `hpr --version`, and every command's, prints the stamp, which opens with
    /// FusionSpace HPR, and `--help` opens with it too; the command is still `hpr`.
    #[test]
    fn version_and_help_open_with_fusionspace_hpr() {
        let line = format!("{}\n", hpr::hpr_core::tool::stamp());
        assert!(line.starts_with("FusionSpace HPR "), "{line}");
        for args in [
            &["hpr", "--version"][..],
            &["hpr", "-V"],
            &["hpr", "sim", "--version"],
            &["hpr", "motors", "list", "--version"],
            &["hpr", "weather", "--version"],
        ] {
            let error = command().try_get_matches_from(args).unwrap_err();
            assert_eq!(
                error.kind(),
                clap::error::ErrorKind::DisplayVersion,
                "{args:?}"
            );
            assert_eq!(error.to_string(), line, "{args:?}");
        }
        for help in [
            command().render_help().to_string(),
            command().render_long_help().to_string(),
        ] {
            assert!(help.starts_with("FusionSpace HPR · Sim: "), "{help}");
            assert!(help.contains("Usage: hpr "), "{help}");
        }
    }

    /// A writer whose reader has gone, as `hpr motors list | head -1` leaves standard output.
    struct ClosedPipe;

    impl Write for ClosedPipe {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::ErrorKind::BrokenPipe.into())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// A closed pipe ends the run with status 0 and says nothing: the reader chose to stop.
    /// A standard error that refuses every write with `kind`.
    struct Refusing(io::ErrorKind);

    impl Write for Refusing {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(self.0.into())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn a_failing_standard_error_never_cuts_the_result_short() {
        let design = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../validation/fixtures/ork/guides/level-1.ork"
        );
        let args = ["hpr", "sim", design];
        let (mut out, mut err) = (Vec::new(), Vec::new());
        assert_eq!(run(args, &mut out, &mut err), Exit::Success);
        assert!(
            String::from_utf8_lossy(&err).contains("warning:"),
            "the flight should print warnings for this test to mean anything"
        );
        for (kind, exit) in [
            (io::ErrorKind::BrokenPipe, Exit::Success),
            (io::ErrorKind::Other, Exit::Failure),
        ] {
            let mut cut = Vec::new();
            assert_eq!(run(args, &mut cut, &mut Refusing(kind)), exit, "{kind:?}");
            assert_eq!(
                String::from_utf8_lossy(&cut),
                String::from_utf8_lossy(&out),
                "{kind:?}: the result must be written whole"
            );
        }
    }

    #[test]
    fn a_closed_pipe_ends_quietly() {
        for json in [false, true] {
            let mut args = vec!["hpr", "motors", "list"];
            if json {
                args.push("--json");
            }
            let mut err = Vec::new();
            assert_eq!(run(args, &mut ClosedPipe, &mut err), Exit::Success);
            assert!(err.is_empty(), "{}", String::from_utf8_lossy(&err));
        }
    }

    /// Each library's hint at the end of a refusal is split off as its `help:` line, and a
    /// message without one, or with a hint's words elsewhere, stays whole.
    #[test]
    fn a_librarys_hint_is_split_off() {
        for hint in library_hints() {
            let (message, help) = library_hint(format!("x.eng: it can't be; {hint}"));
            assert_eq!(
                (message.as_str(), help),
                ("x.eng: it can't be", vec![hint.to_owned()])
            );
            let whole = format!("{hint}; and then something else");
            assert_eq!(library_hint(whole.clone()), (whole, Vec::new()));
        }
        assert_eq!(library_hints().count(), 2);
    }

    /// The exit codes the guide's table lists.
    #[test]
    fn exit_codes_are_the_documented_ones() {
        let codes = [
            Exit::Success,
            Exit::Failure,
            Exit::Usage,
            Exit::NotAvailable,
        ]
        .map(Exit::code);
        assert_eq!(codes, [0, 1, 2, 3]);
    }
}
