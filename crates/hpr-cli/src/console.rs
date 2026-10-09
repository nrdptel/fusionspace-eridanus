//! Whether each of `hpr`'s two streams gets color, and the prefixes its messages start with: the
//! product system's command-line rules (`product/cli.md`, ADR-164).
//!
//! The order is `cli.md`'s: the `--color` flag, then `NO_COLOR`, then `FORCE_COLOR` or
//! `CLICOLOR_FORCE`, then auto, where a stream is colored when it is a terminal and `TERM` isn't
//! `dumb`. Standard output and standard error are decided apart, so `hpr sim … > out.txt` writes
//! plain text to the file while a warning on the terminal stays colored. A `--json` document never
//! has color, whatever the flag says.
//!
//! The roles are the system's [`fs_style`] constants, ANSI roles only, so the user's terminal
//! theme picks the actual colors. Every colored word is also a word: `error:`, `warning:`,
//! `help:`, `note:`. Every line with one of those prefixes goes to standard error, beside a
//! text result too ([`Diagnostics`]); standard output carries the result alone.

use std::io::{self, Write};

use anstream::ColorChoice;
use anstyle::Style;

use crate::fs_style::{self, ColorWhen};

/// What the process's streams are and what its environment asks, for deciding color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Console {
    /// Whether standard output is a terminal.
    pub stdout_terminal: bool,
    /// Whether standard error is a terminal.
    pub stderr_terminal: bool,
    /// What the environment asks: `Never` when `NO_COLOR` is set, else `Always` when
    /// `FORCE_COLOR` is or `CLICOLOR_FORCE` isn't `0`, else `Auto`. [`fs_style::color_choice`]
    /// reads it, the system's own code.
    pub environment: ColorChoice,
    /// Whether `TERM` is `dumb`.
    pub dumb_terminal: bool,
}

impl Console {
    /// Two pipes and no color variable set: plain text unless `--color always`. [`crate::run`]
    /// uses it, so that tests and the guide's examples print the same text whatever shell runs
    /// them.
    pub const PIPED: Self = Self {
        stdout_terminal: false,
        stderr_terminal: false,
        environment: ColorChoice::Auto,
        dumb_terminal: false,
    };

    /// The process's own streams and environment, as the `hpr` binary runs.
    pub fn process() -> Self {
        use std::io::IsTerminal;
        Self {
            stdout_terminal: io::stdout().is_terminal(),
            stderr_terminal: io::stderr().is_terminal(),
            environment: fs_style::color_choice(ColorWhen::Auto),
            dumb_terminal: std::env::var_os("TERM").is_some_and(|term| term == "dumb"),
        }
    }

    /// The paint for each stream, given `--color` and whether `--json` was given.
    pub(crate) fn paints(&self, flag: ColorWhen, json: bool) -> Paints {
        Paints {
            out: Paint {
                on: !json && self.colors(flag, self.stdout_terminal),
            },
            err: Paint {
                on: self.colors(flag, self.stderr_terminal),
            },
        }
    }

    /// Whether a stream that is (or isn't) a terminal gets color: the flag, then the
    /// environment, then auto.
    fn colors(&self, flag: ColorWhen, terminal: bool) -> bool {
        match flag {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => match self.environment {
                ColorChoice::Always | ColorChoice::AlwaysAnsi => true,
                ColorChoice::Never => false,
                ColorChoice::Auto => terminal && !self.dumb_terminal,
            },
        }
    }
}

/// The paint for standard output and for standard error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Paints {
    /// Standard output's.
    pub(crate) out: Paint,
    /// Standard error's.
    pub(crate) err: Paint,
}

/// Whether one stream gets color, and the styled pieces written to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Paint {
    /// Whether the stream gets escape codes.
    pub(crate) on: bool,
}

/// What a message is, which sets its prefix and the prefix's role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Level {
    /// `error:`, bold red.
    Error,
    /// `warning:`, bold yellow.
    Warning,
    /// `help:`, bold blue: what to do. The most useful line, so it comes last.
    Help,
    /// `note:`, bold.
    Note,
}

impl Level {
    /// The prefix, without its colon.
    fn word(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Help => "help",
            Self::Note => "note",
        }
    }

    /// The prefix's role.
    fn style(self) -> Style {
        match self {
            Self::Error => fs_style::ERROR,
            Self::Warning => fs_style::WARNING,
            Self::Help => fs_style::HELP,
            Self::Note => fs_style::NOTE,
        }
    }
}

impl Paint {
    /// `text` in `style`, or plain when the stream gets no color.
    pub(crate) fn styled(self, style: Style, text: &str) -> String {
        if self.on {
            format!("{}{text}{}", style.render(), style.render_reset())
        } else {
            text.to_owned()
        }
    }

    /// One message: its prefix (`error:`, `warning:`, `help:` or `note:`) in its role, then the
    /// text.
    pub(crate) fn prefixed(self, level: Level, text: &str) -> String {
        let prefix = self.styled(level.style(), &format!("{}:", level.word()));
        format!("{prefix} {text}")
    }

    /// Writes one message line, [`Paint::prefixed`].
    pub(crate) fn line(self, to: &mut dyn Write, level: Level, text: &str) -> io::Result<()> {
        writeln!(to, "{}", self.prefixed(level, text))
    }
}

/// Where a command's `warning:`, `note:` and `help:` lines go beside its text result: standard
/// error, in standard error's paint. `product/cli.md` (*Output*) puts the result alone on
/// standard output and everything else on standard error, so `hpr sim … > flight.txt` keeps
/// the file to the flight and the warnings on the terminal. A text renderer gets one of these
/// and standard output, and can write a prefixed line only here.
///
/// A line that can't be written never stops the result: the first failure is kept, later lines
/// are dropped, and [`Diagnostics::failure`] hands it over once the result is out, so a closed
/// standard error can't cut a flight short on standard output.
pub(crate) struct Diagnostics<'a> {
    /// Standard error.
    to: &'a mut dyn Write,
    /// Standard error's color.
    paint: Paint,
    /// The first write to standard error that failed.
    failed: Option<io::Error>,
}

impl<'a> Diagnostics<'a> {
    /// The lines written to `to` (standard error), in `paint` (standard error's).
    pub(crate) fn new(to: &'a mut dyn Write, paint: Paint) -> Self {
        Self {
            to,
            paint,
            failed: None,
        }
    }

    /// Writes one message line, [`Paint::line`]. It answers `Ok` even when standard error
    /// fails, so the caller goes on writing its result; the failure waits in
    /// [`Diagnostics::failure`].
    pub(crate) fn line(&mut self, level: Level, text: &str) -> io::Result<()> {
        if self.failed.is_none()
            && let Err(error) = self.paint.line(self.to, level, text)
        {
            self.failed = Some(error);
        }
        Ok(())
    }

    /// Flushes standard error, keeping a failure as [`Diagnostics::line`] does.
    pub(crate) fn flush(&mut self) {
        if self.failed.is_none()
            && let Err(error) = self.to.flush()
        {
            self.failed = Some(error);
        }
    }

    /// The first write to standard error that failed, if any, taken.
    pub(crate) fn failure(&mut self) -> Option<io::Error> {
        self.failed.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A console with standard output a terminal, standard error a pipe, and `environment`.
    fn split(environment: ColorChoice) -> Console {
        Console {
            stdout_terminal: true,
            stderr_terminal: false,
            environment,
            dumb_terminal: false,
        }
    }

    /// Standard output's and standard error's decisions.
    fn decided(console: Console, flag: ColorWhen) -> (bool, bool) {
        let paints = console.paints(flag, false);
        (paints.out.on, paints.err.on)
    }

    /// Auto colors a stream only when it is a terminal, each stream on its own.
    #[test]
    fn auto_decides_each_stream_by_whether_it_is_a_terminal() {
        assert_eq!(
            decided(split(ColorChoice::Auto), ColorWhen::Auto),
            (true, false)
        );
        let flipped = Console {
            stdout_terminal: false,
            stderr_terminal: true,
            ..split(ColorChoice::Auto)
        };
        assert_eq!(decided(flipped, ColorWhen::Auto), (false, true));
        assert_eq!(decided(Console::PIPED, ColorWhen::Auto), (false, false));
    }

    /// `TERM=dumb` turns auto's color off on a terminal, but not a color variable or the flag.
    #[test]
    fn a_dumb_terminal_gets_no_color_from_auto_only() {
        let dumb = |environment| Console {
            dumb_terminal: true,
            stderr_terminal: true,
            ..split(environment)
        };
        assert_eq!(
            decided(dumb(ColorChoice::Auto), ColorWhen::Auto),
            (false, false)
        );
        assert_eq!(
            decided(dumb(ColorChoice::Always), ColorWhen::Auto),
            (true, true)
        );
        assert_eq!(
            decided(dumb(ColorChoice::Auto), ColorWhen::Always),
            (true, true)
        );
    }

    /// `NO_COLOR` turns both streams off, terminals or not, unless the flag says `always`.
    #[test]
    fn no_color_beats_auto_but_not_the_flag() {
        let console = split(ColorChoice::Never);
        assert_eq!(decided(console, ColorWhen::Auto), (false, false));
        assert_eq!(decided(console, ColorWhen::Always), (true, true));
    }

    /// `FORCE_COLOR` or `CLICOLOR_FORCE` colors both streams, pipes too, unless the flag says
    /// `never`.
    #[test]
    fn a_forced_color_beats_auto_but_not_the_flag() {
        for environment in [ColorChoice::Always, ColorChoice::AlwaysAnsi] {
            let console = split(environment);
            assert_eq!(decided(console, ColorWhen::Auto), (true, true));
            assert_eq!(decided(console, ColorWhen::Never), (false, false));
        }
    }

    /// The flag decides both streams whatever they are.
    #[test]
    fn the_flag_comes_first() {
        for environment in [ColorChoice::Auto, ColorChoice::Never, ColorChoice::Always] {
            for console in [split(environment), Console::PIPED] {
                assert_eq!(decided(console, ColorWhen::Always), (true, true));
                assert_eq!(decided(console, ColorWhen::Never), (false, false));
            }
        }
    }

    /// `--json` keeps standard output plain even with `--color always`; standard error keeps
    /// its own decision.
    #[test]
    fn json_output_never_has_color() {
        for flag in [ColorWhen::Auto, ColorWhen::Always] {
            let paints = split(ColorChoice::Always).paints(flag, true);
            assert!(!paints.out.on);
            assert!(paints.err.on);
        }
    }

    /// Each prefix in its role, and plain when the stream gets no color.
    #[test]
    fn prefixes_take_their_roles() {
        let cases = [
            (Level::Error, "\u{1b}[1m\u{1b}[31merror:\u{1b}[0m x\n"),
            (Level::Warning, "\u{1b}[1m\u{1b}[33mwarning:\u{1b}[0m x\n"),
            (Level::Help, "\u{1b}[1m\u{1b}[34mhelp:\u{1b}[0m x\n"),
            (Level::Note, "\u{1b}[1mnote:\u{1b}[0m x\n"),
        ];
        for (level, colored) in cases {
            let mut text = Vec::new();
            Paint { on: true }.line(&mut text, level, "x").unwrap();
            assert_eq!(String::from_utf8(text).unwrap(), colored);
            let mut plain = Vec::new();
            Paint { on: false }.line(&mut plain, level, "x").unwrap();
            assert_eq!(
                String::from_utf8(plain).unwrap(),
                format!("{}: x\n", level.word())
            );
        }
    }
}
