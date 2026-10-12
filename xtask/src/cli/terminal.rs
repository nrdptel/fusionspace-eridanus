//! Terminal figures (M0.8a1, ADR-247 §3): a run of `hpr` drawn as a terminal shows it, from the
//! same in-process run as an example on the same page, so the picture and the text a reader can
//! copy or search come from one program and agree.
//!
//! A figure draws the command line and then each line the command printed, on the product
//! system's white panel, in Cascadia Mono. A line wider than the panel wraps at its last space
//! that fits, or mid-word if none does, and its rest goes on indented, as a terminal continues
//! it; the widths are the figure check's own ([`crate::figures::monospace_advance`]), so nothing
//! the figure draws can leave its frame. Spaces are kept (`xml:space="preserve"`), so a table's
//! columns stay aligned.
//!
//! Each callout is a letter in a balloon, in the gutter left of the first line that starts with
//! its text, joined to that line by a leader. A callout whose line the run doesn't print fails,
//! so a renamed output line can't drop a balloon unnoticed. The callouts are letters, not
//! numbers, so the prose that explains them holds no number the guides' check would have to
//! quote (ADR-242); the words that explain them stay in the prose.

use std::fmt::Write as _;

/// A terminal figure: the page whose example it draws, the command, the file it goes to, and its
/// callouts, each a letter and the start of the line it marks.
pub(super) struct Shot {
    pub(super) page: &'static str,
    pub(super) command: &'static str,
    pub(super) path: &'static str,
    pub(super) callouts: &'static [(char, &'static str)],
}

/// The figure's colors: the product system's light roles (`figures::drawing::TOKENS`).
const CANVAS: &str = "#F3F4F7";
const SURFACE: &str = "#FFFFFF";
const RULE: &str = "#D6DAE4";
const INK: &str = "#0B0F1C";
const INK_MUTED: &str = "#566079";

/// The figure's width, px: wide enough for a line of 100 columns, the width `hpr` wraps its own
/// paragraphs to, so they don't wrap again.
const WIDTH: f64 = 760.0;
/// The text's size and the distance between its baselines, px.
const SIZE: f64 = 11.0;
const LINE: f64 = 16.0;
/// The panel's left and right edges, and the space inside it around the text, px.
const PANEL_LEFT: f64 = 40.0;
const PANEL_RIGHT: f64 = 752.0;
const PAD: f64 = 12.0;
/// The balloons' center, left of the panel, and their radius, px.
const BALLOON_X: f64 = 18.0;
const BALLOON_R: f64 = 9.0;
/// How a wrapped line's rest goes on.
const CONTINUED: &str = "    ";
/// The font stack every figure's data is set in.
const MONO: &str = "'Cascadia Mono', ui-monospace, Menlo, Consolas, monospace";

/// The widest a line may draw, in ems: the panel's inside, less a pixel either side for a glyph's
/// ink past its advance.
fn widest() -> f64 {
    (PANEL_RIGHT - PANEL_LEFT - 2.0 * PAD - 2.0) / SIZE
}

/// `line` as rows that each fit in `widest` ems: whole when it fits, else broken at the last
/// space that fits after its indent, or mid-word if none does, each rest indented by
/// [`CONTINUED`].
fn wrapped(line: &str, widest: f64) -> Result<Vec<String>, String> {
    let width = |text: &str| {
        crate::figures::monospace_advance(text).ok_or_else(|| {
            format!("\"{line}\" holds a character the figure check's glyph table lacks")
        })
    };
    let mut rows = Vec::new();
    let mut rest = line.to_owned();
    while width(&rest)? > widest {
        // The longest start of `rest` that fits, by whole characters.
        let mut fits = 0;
        for (at, c) in rest.char_indices() {
            if width(&rest[..at + c.len_utf8()])? > widest {
                break;
            }
            fits = at + c.len_utf8();
        }
        let indent = rest.len() - rest.trim_start().len();
        let at = rest[..fits]
            .rfind(' ')
            .filter(|&space| space > indent && rest[..space].trim_end().len() > indent)
            .unwrap_or(fits);
        if at == 0 {
            return Err(format!("\"{line}\" can't be broken to fit the panel"));
        }
        rows.push(rest[..at].trim_end().to_owned());
        rest = format!("{CONTINUED}{}", rest[at..].trim_start());
    }
    rows.push(rest);
    Ok(rows)
}

/// Text set for an SVG's character data or attribute.
fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}

/// The figure of `block`, a fenced example as `cli.rs`'s `example` writes it (its first line the
/// shell's `$` and the command), with `callouts`; an error if a callout's line isn't printed, two
/// callouts mark one line, or a line can't be drawn.
pub(super) fn draw(block: &str, callouts: &[(char, &str)]) -> Result<String, String> {
    let lines: Vec<&str> = block
        .strip_prefix("```text\n")
        .and_then(|body| body.strip_suffix("```\n"))
        .ok_or("the example is not a fenced text block")?
        .lines()
        .collect();
    let command = lines
        .first()
        .and_then(|line| line.strip_prefix("$ "))
        .ok_or("the example doesn't start with its command")?;
    // Each printed line's first row, and every row.
    let mut rows: Vec<(usize, String)> = Vec::new();
    let mut first_row = Vec::with_capacity(lines.len());
    for (at, line) in lines.iter().enumerate() {
        first_row.push(rows.len());
        for row in wrapped(line, widest())? {
            rows.push((at, row));
        }
    }
    let mut marked: Vec<(char, usize)> = Vec::new();
    for &(letter, start) in callouts {
        let line = lines
            .iter()
            .skip(1)
            .position(|line| line.starts_with(start))
            .map(|at| at + 1)
            .ok_or_else(|| {
                format!("callout {letter}: `{command}` prints no line that starts \"{start}\"")
            })?;
        let row = first_row[line];
        if let Some((other, _)) = marked.iter().find(|(_, at)| *at == row) {
            return Err(format!("callouts {other} and {letter} mark the same line"));
        }
        marked.push((letter, row));
    }
    let top = 16.0;
    let first_baseline = top + PAD + SIZE;
    let height = first_baseline + LINE * (rows.len() - 1) as f64 + PAD + 8.0 + top;
    let panel_height = height - 2.0 * top;
    let baseline = |row: usize| first_baseline + LINE * row as f64;
    let letters: Vec<String> = marked
        .iter()
        .map(|(letter, _)| letter.to_string())
        .collect();
    let mut svg = String::new();
    let _ = writeln!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"{height}\" \
         viewBox=\"0 0 {WIDTH} {height}\" font-family=\"{MONO}\" font-size=\"{SIZE}\" \
         role=\"img\" aria-labelledby=\"title\" aria-describedby=\"desc\">"
    );
    let _ = writeln!(
        svg,
        "<title id=\"title\">{}</title>",
        escaped(&format!("What `{command}` prints, as a terminal shows it"))
    );
    let _ = writeln!(
        svg,
        "<desc id=\"desc\">{}</desc>",
        escaped(&format!(
            "The command line, then each line it printed, long lines wrapped; balloons {} mark \
             the lines the page explains. Drawn by cargo xtask cli from the same run as the \
             page's text.",
            letters.join(", ")
        ))
    );
    let _ = writeln!(
        svg,
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{CANVAS}\"/>"
    );
    let _ = writeln!(
        svg,
        "<rect x=\"{PANEL_LEFT}\" y=\"{top}\" width=\"{}\" height=\"{panel_height}\" \
         fill=\"{SURFACE}\" stroke=\"{RULE}\" stroke-width=\"1\"/>",
        PANEL_RIGHT - PANEL_LEFT
    );
    let x = PANEL_LEFT + PAD;
    for (row, (line, text)) in rows.iter().enumerate() {
        if text.trim().is_empty() {
            continue;
        }
        // The command line stands out from what it printed.
        let weight = if *line == 0 {
            " font-weight=\"600\""
        } else {
            ""
        };
        let _ = writeln!(
            svg,
            "<text x=\"{x}\" y=\"{}\" xml:space=\"preserve\"{weight} fill=\"{INK}\">{}</text>",
            baseline(row),
            escaped(text)
        );
    }
    for (letter, row) in &marked {
        // The balloon's center at the row's middle: the baseline less a third of the size.
        let y = baseline(*row) - SIZE / 3.0;
        let _ = writeln!(
            svg,
            "<line x1=\"{}\" y1=\"{y:.1}\" x2=\"{PANEL_LEFT}\" y2=\"{y:.1}\" stroke=\"{INK_MUTED}\" \
             stroke-width=\"1\"/>\
             <circle cx=\"{BALLOON_X}\" cy=\"{y:.1}\" r=\"{BALLOON_R}\" fill=\"{INK}\"/>\
             <text x=\"{BALLOON_X}\" y=\"{:.1}\" text-anchor=\"middle\" font-weight=\"600\" \
             fill=\"{SURFACE}\">{letter}</text>",
            BALLOON_X + BALLOON_R,
            y + SIZE / 3.0
        );
    }
    svg.push_str("</svg>\n");
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the figure check and the drawing check find in `svg`.
    fn problems(svg: &str) -> Vec<String> {
        let mut found = crate::figures::check(svg);
        found.extend(crate::figures::drawing::check(svg));
        found.iter().map(ToString::to_string).collect()
    }

    fn block(lines: &[&str]) -> String {
        format!("```text\n{}\n```\n", lines.join("\n"))
    }

    #[test]
    fn a_terminal_figure_draws_every_line_and_its_callouts_in_its_frame() {
        let run = block(&[
            "$ hpr sim rocket.ork",
            "apogee                1065.1 m (3494 ft) above the site at 11.62 s",
            "",
            "event                   time                 height",
            "liftoff               0.00 s      0.4 m      (1 ft)",
        ]);
        let svg = draw(&run, &[('A', "apogee"), ('B', "event")]).unwrap();
        assert_eq!(problems(&svg), Vec::<String>::new(), "{svg}");
        assert!(svg.contains(">$ hpr sim rocket.ork</text>"));
        // Spaces are kept, so the columns stay where the program put them.
        assert!(svg.contains("xml:space=\"preserve\""));
        assert!(svg.contains(">liftoff               0.00 s      0.4 m      (1 ft)</text>"));
        assert_eq!(svg.matches("<circle").count(), 2);
        assert!(svg.contains(">A</text>") && svg.contains(">B</text>"));
    }

    #[test]
    fn a_long_line_wraps_inside_the_panel_and_still_passes_the_checks() {
        let warning = format!(
            "warning: {}",
            "the drag reads high, so the apogee reads low; ".repeat(8)
        );
        let unbroken = "x".repeat(400);
        let run = block(&["$ hpr sim rocket.ork", &warning, &unbroken]);
        let svg = draw(&run, &[('A', "warning:")]).unwrap();
        assert_eq!(problems(&svg), Vec::<String>::new(), "{svg}");
        for line in [&warning, &unbroken] {
            let rows = wrapped(line, widest()).unwrap();
            assert!(rows.len() > 1, "{line}");
            for row in &rows {
                assert!(
                    crate::figures::monospace_advance(row).unwrap() <= widest(),
                    "{row}"
                );
            }
            // Nothing is lost: the rows hold the line's words, in order.
            let words: Vec<&str> = rows.iter().flat_map(|row| row.split_whitespace()).collect();
            assert_eq!(words.concat(), line.split_whitespace().collect::<String>());
        }
        // A wrapped line's rest is indented, and a line that fits is left whole.
        assert!(wrapped(&warning, widest()).unwrap()[1].starts_with(CONTINUED));
        assert_eq!(wrapped("short", widest()).unwrap(), ["short"]);
        // A paragraph `hpr` wrapped to 100 columns itself is not wrapped again.
        let hundred = "x".repeat(100);
        assert_eq!(wrapped(&hundred, widest()).unwrap(), std::slice::from_ref(&hundred));
    }

    #[test]
    fn a_wider_panel_line_would_fail_the_frame_check() {
        // The probe for the wrap: a line drawn whole past the panel's width leaves the frame.
        let line = "x".repeat(200);
        let svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"40\" \
             viewBox=\"0 0 {WIDTH} 40\" font-family=\"{MONO}\" font-size=\"{SIZE}\">\
             <rect width=\"{WIDTH}\" height=\"40\" fill=\"{CANVAS}\"/>\
             <text x=\"{}\" y=\"24\" fill=\"{INK}\">{line}</text></svg>",
            PANEL_LEFT + PAD
        );
        assert!(!problems(&svg).is_empty());
    }

    #[test]
    fn a_callout_whose_line_is_not_printed_fails() {
        let run = block(&["$ hpr sim rocket.ork", "apogee 1065.1 m"]);
        let error = draw(&run, &[('A', "apogee"), ('B', "descent")]).unwrap_err();
        assert!(
            error.contains("callout B") && error.contains("\"descent\""),
            "{error}"
        );
        // The command line is not a line the command printed.
        let error = draw(&run, &[('A', "$ hpr")]).unwrap_err();
        assert!(error.contains("callout A"), "{error}");
        let error = draw(&run, &[('A', "apogee"), ('B', "apo")]).unwrap_err();
        assert!(error.contains("mark the same line"), "{error}");
        assert!(draw("apogee\n", &[]).unwrap_err().contains("fenced"));
    }
}
