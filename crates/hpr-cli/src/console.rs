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
use std::time::Duration;

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
    /// How long a run goes before it draws its progress on standard error, when that is a
    /// terminal and the output is text: [`PROGRESS_AFTER`], as `cli.md` (*Output*) asks.
    pub progress_after: Duration,
}

/// How long a run goes before it shows its progress: `cli.md` (*Output*) asks a tool to print
/// something within 100 ms, and gives anything over a second a bar with a count and an estimate,
/// so the bar is up from 100 ms. A run done sooner draws nothing.
pub const PROGRESS_AFTER: Duration = Duration::from_millis(100);

impl Console {
    /// Two pipes and no color variable set: plain text unless `--color always`. [`crate::run`]
    /// uses it, so that tests and the guide's examples print the same text whatever shell runs
    /// them.
    pub const PIPED: Self = Self {
        stdout_terminal: false,
        stderr_terminal: false,
        environment: ColorChoice::Auto,
        dumb_terminal: false,
        progress_after: PROGRESS_AFTER,
    };

    /// The process's own streams and environment, as the `hpr` binary runs.
    pub fn process() -> Self {
        use std::io::IsTerminal;
        Self {
            stdout_terminal: io::stdout().is_terminal(),
            stderr_terminal: io::stderr().is_terminal(),
            environment: fs_style::color_choice(ColorWhen::Auto),
            dumb_terminal: std::env::var_os("TERM").is_some_and(|term| term == "dumb"),
            progress_after: PROGRESS_AFTER,
        }
    }

    /// When a long run draws its progress on standard error: after [`Console::progress_after`]
    /// when standard error is a terminal and the output is text; never with `--json` or on a pipe.
    pub(crate) fn progress(&self, json: bool) -> Option<Duration> {
        (self.stderr_terminal && !json).then_some(self.progress_after)
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
    /// text; in a `help:` or `note:` line, what to type in the Literal role ([`Paint::literals`]).
    pub(crate) fn prefixed(self, level: Level, text: &str) -> String {
        let prefix = self.styled(level.style(), &format!("{}:", level.word()));
        let text = match level {
            Level::Help | Level::Note => self.literals(text),
            Level::Error | Level::Warning => text.to_owned(),
        };
        format!("{prefix} {text}")
    }

    /// `text` with what to type in the Literal role, as `cli.md` (*Color*) asks of hints: each
    /// span between backticks (the backticks kept, plain), and each option written bare, a word
    /// starting `--` and a letter, up to a space or a closing `,;:.)`. Plain without color.
    pub(crate) fn literals(self, text: &str) -> String {
        if !self.on {
            return text.to_owned();
        }
        let mut styled = String::with_capacity(text.len());
        let mut rest = text;
        while !rest.is_empty() {
            if let Some(quoted) = rest.strip_prefix('`')
                && let Some(end) = quoted.find('`')
            {
                styled.push('`');
                styled.push_str(&self.styled(fs_style::LITERAL, &quoted[..end]));
                styled.push('`');
                rest = &quoted[end + 1..];
                continue;
            }
            let starts_word = styled.is_empty() || styled.ends_with([' ', '(']);
            if starts_word
                && let Some(name) = rest.strip_prefix("--")
                && name.starts_with(|c: char| c.is_ascii_alphabetic())
            {
                let end = rest
                    .find(|c: char| c.is_whitespace() || ",;:.)`".contains(c))
                    .unwrap_or(rest.len());
                styled.push_str(&self.styled(fs_style::LITERAL, &rest[..end]));
                rest = &rest[end..];
                continue;
            }
            let mut chars = rest.chars();
            if let Some(c) = chars.next() {
                styled.push(c);
            }
            rest = chars.as_str();
        }
        styled
    }

    /// Writes one message line, [`Paint::prefixed`].
    pub(crate) fn line(self, to: &mut dyn Write, level: Level, text: &str) -> io::Result<()> {
        writeln!(to, "{}", self.prefixed(level, text))
    }
}

/// Standard output for a text result, in standard output's paint: the result's first line, which
/// says what was read, in the Heading role, and each table's header row through
/// [`Text::heading`], as the product system's `cli.md` asks (*Color*, *Output*). It writes
/// through as it goes; with no color it is standard output as it is.
pub(crate) struct Text<'a> {
    /// Standard output.
    to: &'a mut dyn Write,
    /// Standard output's color.
    paint: Paint,
    /// Whether the first line's style has started.
    started: bool,
    /// Whether the first line has ended.
    first_ended: bool,
}

impl<'a> Text<'a> {
    /// The result written to `to` (standard output), in `paint` (standard output's).
    pub(crate) fn new(to: &'a mut dyn Write, paint: Paint) -> Self {
        Self {
            to,
            paint,
            started: false,
            first_ended: false,
        }
    }

    /// Writes `line`, a table's header row, in the Heading role, and a newline. The result's
    /// first line takes the role already, so a header there isn't styled twice.
    pub(crate) fn heading(&mut self, line: &str) -> io::Result<()> {
        if self.first_ended {
            writeln!(self.to, "{}", self.paint.styled(fs_style::HEADING, line))
        } else {
            writeln!(self, "{line}")
        }
    }

    /// Writes each of `lines` and a newline, those at the indices in `headings` as table
    /// headers ([`Text::heading`]).
    pub(crate) fn lines(&mut self, lines: &[String], headings: &[usize]) -> io::Result<()> {
        for (i, line) in lines.iter().enumerate() {
            if headings.contains(&i) {
                self.heading(line)?;
            } else {
                writeln!(self, "{line}")?;
            }
        }
        Ok(())
    }

    /// Ends the first line's style if the result ended inside it, with no newline.
    pub(crate) fn finish(&mut self) -> io::Result<()> {
        if self.started && !self.first_ended {
            self.first_ended = true;
            write!(self.to, "{}", fs_style::HEADING.render_reset())?;
        }
        Ok(())
    }
}

impl Write for Text<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.first_ended || !self.paint.on || buf.is_empty() {
            return self.to.write_all(buf).map(|()| buf.len());
        }
        if !self.started {
            self.started = true;
            write!(self.to, "{}", fs_style::HEADING.render())?;
        }
        match buf.iter().position(|byte| *byte == b'\n') {
            Some(end) => {
                self.to.write_all(&buf[..end])?;
                self.first_ended = true;
                write!(self.to, "{}", fs_style::HEADING.render_reset())?;
                self.to.write_all(&buf[end..])?;
            }
            None => self.to.write_all(buf)?,
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.to.flush()
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
    /// The length, in characters, of the progress line on screen; 0 when there is none.
    progress_shown: usize,
}

impl<'a> Diagnostics<'a> {
    /// The lines written to `to` (standard error), in `paint` (standard error's).
    pub(crate) fn new(to: &'a mut dyn Write, paint: Paint) -> Self {
        Self {
            to,
            paint,
            failed: None,
            progress_shown: 0,
        }
    }

    /// Draws `text` as the progress line, over the one before it: a carriage return, the text,
    /// and spaces over what is left of a longer line before it. A failure waits as
    /// [`Diagnostics::line`]'s does.
    pub(crate) fn progress(&mut self, text: &str) {
        let length = text.chars().count();
        let rest = " ".repeat(self.progress_shown.saturating_sub(length));
        self.raw(&format!("\r{text}{rest}"));
        self.progress_shown = length;
    }

    /// Takes the progress line off the screen, if one is shown, so the next line starts on a
    /// clean one.
    pub(crate) fn end_progress(&mut self) {
        if self.progress_shown > 0 {
            let blank = " ".repeat(self.progress_shown);
            self.raw(&format!("\r{blank}\r"));
            self.progress_shown = 0;
        }
    }

    /// Writes `text` as it is and flushes, keeping a failure as [`Diagnostics::line`] does.
    fn raw(&mut self, text: &str) {
        if self.failed.is_none()
            && let Err(error) = self
                .to
                .write_all(text.as_bytes())
                .and_then(|()| self.to.flush())
        {
            self.failed = Some(error);
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

/// How many cells the progress bar has.
const BAR_CELLS: usize = 20;

/// The progress line of a run of `total` flights, `done` of them flown after `elapsed`: a count,
/// a bar and an estimate of the time left, as `cli.md` (*Output*) asks. The estimate assumes the
/// flights left take as long as those flown; it waits for the first, and once every flight is
/// flown the run is finishing.
pub(crate) fn progress_line(done: u64, total: u64, elapsed: Duration) -> String {
    let share = if total == 0 {
        1.0
    } else {
        done.min(total) as f64 / total as f64
    };
    // Each cell full only when its share is done, so the bar is full only at the end.
    let full = ((share * BAR_CELLS as f64).floor() as usize).min(BAR_CELLS);
    let bar = format!("{}{}", "=".repeat(full), " ".repeat(BAR_CELLS - full));
    let left = if done == 0 {
        "estimating the time left".to_owned()
    } else if done >= total {
        "finishing".to_owned()
    } else {
        let left_s = elapsed.as_secs_f64() * (total.saturating_sub(done)) as f64 / done as f64;
        if left_s < 60.0 {
            format!("about {} s left", (left_s.ceil() as u64).max(1))
        } else {
            format!("about {} min left", (left_s / 60.0).ceil() as u64)
        }
    };
    format!(
        "flying {done} of {total} flights [{bar}] {:.0}%, {left}",
        (100.0 * share).floor()
    )
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
            progress_after: PROGRESS_AFTER,
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

    /// Bold on and off, the Heading role's codes.
    const BOLD: &str = "\u{1b}[1m";
    /// Bold blue, the Literal role's.
    const BLUE: &str = "\u{1b}[1m\u{1b}[34m";
    /// The reset.
    const RESET: &str = "\u{1b}[0m";

    /// The first line takes the Heading role however it is written, in pieces too; a table's
    /// header row takes it through `heading`; the rest is as written; without color, all is.
    #[test]
    fn a_results_first_line_and_headers_are_headings() {
        let write = |on: bool| {
            let mut bytes = Vec::new();
            let mut text = Text::new(&mut bytes, Paint { on });
            write!(text, "design ").unwrap();
            writeln!(text, "(a.ork)").unwrap();
            writeln!(text, "a line").unwrap();
            text.heading("event  time").unwrap();
            text.lines(&["row".to_owned(), "name  unit".to_owned()], &[1])
                .unwrap();
            text.finish().unwrap();
            String::from_utf8(bytes).unwrap()
        };
        assert_eq!(
            write(true),
            format!(
                "{BOLD}design (a.ork){RESET}\na line\n{BOLD}event  time{RESET}\nrow\n\
                 {BOLD}name  unit{RESET}\n"
            )
        );
        assert_eq!(
            write(false),
            "design (a.ork)\na line\nevent  time\nrow\nname  unit\n"
        );
        // A header that is the first line is styled once; a result with no newline is closed.
        let mut bytes = Vec::new();
        let mut text = Text::new(&mut bytes, Paint { on: true });
        text.heading("designation  class").unwrap();
        write!(text, "H170M").unwrap();
        text.finish().unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            format!("{BOLD}designation  class{RESET}\nH170M")
        );
        let mut bytes = Vec::new();
        let mut text = Text::new(&mut bytes, Paint { on: true });
        write!(text, "no newline").unwrap();
        text.finish().unwrap();
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            format!("{BOLD}no newline{RESET}")
        );
    }

    /// In a hint, what to type takes the Literal role: a span between backticks, and an option
    /// written bare, up to a space or closing punctuation; not a negative number, a single
    /// hyphen, or `--` inside a word. Plain without color, and on `error:` and `warning:` lines.
    #[test]
    fn hints_put_what_to_type_in_the_literal_role() {
        let on = Paint { on: true };
        for (text, expected) in [
            (
                "run `hpr motors list --class J`",
                format!("run `{BLUE}hpr motors list --class J{RESET}`"),
            ),
            (
                "--config flies the others: 2",
                format!("{BLUE}--config{RESET} flies the others: 2"),
            ),
            (
                "say which with --mount: a, b",
                format!("say which with {BLUE}--mount{RESET}: a, b"),
            ),
            (
                "such as --impulse-sd 0.03, to scatter one",
                format!("such as {BLUE}--impulse-sd{RESET} 0.03, to scatter one"),
            ),
            ("(--wind-sd).", format!("({BLUE}--wind-sd{RESET}).")),
            (
                "-119.06, -o, a--b, -- x",
                "-119.06, -o, a--b, -- x".to_owned(),
            ),
            ("an unclosed ` stays", "an unclosed ` stays".to_owned()),
        ] {
            assert_eq!(on.literals(text), expected, "{text}");
            assert_eq!(Paint { on: false }.literals(text), text);
        }
        assert_eq!(
            on.prefixed(Level::Note, "--x"),
            format!("{BOLD}note:{RESET} {BLUE}--x{RESET}")
        );
        assert!(on.prefixed(Level::Error, "--x").ends_with(" --x"));
        assert!(on.prefixed(Level::Warning, "`x`").ends_with(" `x`"));
    }

    /// The progress line: a count, a bar full only at the end, and the time left at the pace so
    /// far, in seconds and then minutes, rounded up.
    #[test]
    fn the_progress_line_counts_and_estimates() {
        let seconds = Duration::from_secs_f64;
        assert_eq!(
            progress_line(0, 200, seconds(1.0)),
            "flying 0 of 200 flights [                    ] 0%, estimating the time left"
        );
        assert_eq!(
            progress_line(50, 200, seconds(2.0)),
            "flying 50 of 200 flights [=====               ] 25%, about 6 s left"
        );
        assert_eq!(
            progress_line(199, 200, seconds(10.0)),
            "flying 199 of 200 flights [=================== ] 99%, about 1 s left"
        );
        assert_eq!(
            progress_line(10, 1000, seconds(10.0)),
            "flying 10 of 1000 flights [                    ] 1%, about 17 min left"
        );
        assert_eq!(
            progress_line(200, 200, seconds(30.0)),
            "flying 200 of 200 flights [====================] 100%, finishing"
        );
    }

    /// Each progress line is drawn over the one before, a shorter one blanking what is left,
    /// and ending takes it off the screen; with none shown, ending writes nothing.
    #[test]
    fn progress_draws_over_itself_and_clears() {
        let mut bytes = Vec::new();
        let mut diagnostics = Diagnostics::new(&mut bytes, Paint { on: false });
        diagnostics.end_progress();
        diagnostics.progress("abcd");
        diagnostics.progress("ab");
        diagnostics.end_progress();
        diagnostics.end_progress();
        assert!(diagnostics.failure().is_none());
        assert_eq!(String::from_utf8(bytes).unwrap(), "\rabcd\rab  \r  \r");
    }
}
