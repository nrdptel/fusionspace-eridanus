//! The page check (M0.7a1): every page of the built guide, loaded in a real browser at every
//! window width from 320 to 1,920 px in 40 px steps, must show all of itself. Nothing may be cut
//! off by an edge, at any width a reader might use.
//!
//! At each width it fails a page on:
//!
//! - **Wider than the window.** The page scrolls sideways (`scrollWidth > clientWidth`, the
//!   body's included, which counts overflow even where `overflow-x: hidden` hides it), or any text
//!   or box reaches past the window's right or left edge outside a box that scrolls sideways. A
//!   drawer put away (a fixed box wholly off the left edge, as mdBook's sidebar is on a narrow
//!   window) is skipped with what it holds; nothing else may sit off to the left. mdBook clips
//!   its page body with `overflow-x: clip`, which hides overflow from `scrollWidth`; the walk over
//!   boxes and text catches it there too. The boxes that hold the page's `main` stand for the
//!   window: mdBook's `.content` scrolls (`overflow-y: auto`, so sideways too), and one that
//!   scrolls or clips and is wider inside than out fails, since the page then scrolls sideways
//!   inside it, or is cut. One whose overflow is visible hands what reaches past it, such as a
//!   figure wider than the prose column, to the box around it, which is measured in turn. Only
//!   boxes within `main`, a code block or a table's wrapper, may scroll sideways.
//! - **Text cut off.** Text that a box which doesn't scroll (`overflow: hidden` or `clip`,
//!   `text-overflow: ellipsis`) cuts by more than 1 px, so part of it can never be seen. Boxes of
//!   2 px or less hide their text on purpose, for screen readers only, and are skipped.
//! - **Text over text.** Two runs of visible text whose rectangles overlap more than 2 px sideways
//!   and more than a quarter of the shorter one's height: a label drawn over another. Two lines of
//!   a tight line height, or two runs that touch, don't count.
//! - **A figure shown smaller than drawn** (#382). A figure's text is drawn at its size, so a
//!   figure shown smaller reads smaller: the plot's 12 px text, in its 960 px figure shown in
//!   mdBook's 750 px column, read at 9.4 px. Where the box holding the page's `main` has room for
//!   a figure, up to the product system's widest content (1,120 px, `foundations.md`, *Width*),
//!   it must be shown at its size; where it hasn't, the reader must be able to open it at its
//!   size, through mdBook's zoom (a checkbox in the figure's label that shows a copy over the
//!   page). The theme lets a figure reach past the prose column for this (`theme/hpr.css`). A
//!   figure with no size the check can read (broken, or an SVG without one) fails.
//! - **mdBook's defaults** (#383), by computed style: the root element and every element of the
//!   body, with its `::before` and `::after`, drawn now or hidden until asked for (the help
//!   popup, the copy tooltip), plus a search hit and a search result added as mdBook's search
//!   makes them:
//!   - **a color off the system's roles** in the page's theme (`foundations.md`, *Semantic
//!     roles*: its text, background, drawn borders, outline, underline, and an SVG's fill and
//!     stroke). Transparent passes, and a background may be a role seen through, as a dialog's
//!     dimmed canvas is. A filter, a backdrop filter or an image drawn as `::before` or `::after`
//!     paints colors the check can't read, and fails; a search hit takes the action fill;
//!   - **a rounded corner or a shadow** (`foundations.md`, *Lines and shape*);
//!   - **motion off the system's durations** (0, 100, 160 or 240 ms) or easings, a smooth
//!     scroll, or an animation that never ends (`foundations.md`, *Motion*); and with the
//!     stylesheets' reduced-motion rules applied in place, any motion at all.
//!
//!   Each page is read in the theme it loads in (mdBook's navy for a browser whose system is
//!   dark), then as a reader with scripts off gets it (the root's `js` class taken off, which
//!   applies mdBook's `html:not(.js)` rules), with a light system and with a dark one (the
//!   stylesheets' dark-scheme rules applied in place), then switched as mdBook's theme menu
//!   switches it and read in the other once the switch's transitions have run. Hover and focus
//!   aren't read, nor what a script sets as it runs (mdBook's `scrollTo` with a smooth scroll).
//!
//! **How.** A small web server on `127.0.0.1` serves the built site and, under `/__check/`, the
//! harness ([`HARNESS_JS`]), the plan and the canaries. A few headless Chrome processes (headless
//! Chrome opens one tab each, up to [`MOST_TABS`]) run the harness and take the pages from the
//! server one at a time, the longest first. Each page loads fresh into a frame of each width, with
//! nothing remembered from an earlier load (mdBook shows its sidebar by width unless the reader
//! chose), so each load is what a first-time reader at that width gets. The harness loads every
//! font face the page declares and waits for them and two frames, so the page is measured in its
//! own fonts (a face that fails to load fails the page), measures, and posts the results back to
//! the server. Same origin is why
//! there is a server: a page opened from `file://` can't be measured from another. Scrollbars stay
//! on, as a reader sees them.
//!
//! **Canaries.** Every run also loads pages built to fail ([`CANARIES`]), laid out as mdBook lays
//! a page, its `main` in a content box that scrolls: a label moved past the right edge, outside
//! `main`, inside it and inside a box that clips it, a label moved past the left edge inside
//! `main`, a label cut short by its box, two labels drawn over each other, a figure shown at half
//! its size with room for all of it, one wider than any window with no zoom, a figure reaching
//! past a narrow `main` as the theme lays it out, which must pass, a label moved past the
//! window's edge from such a `main`, a label off the system's colors, the same only in navy (read
//! after the switch), the same only with scripts off, and only once a delayed change to it has run,
//! the root element off them, a label tinted by a filter, one over a backdrop filter, one with an
//! image as its `::before`, a search hit in ink, a rounded box, a box with a shadow, a 0.3 s
//! transition, one at the system's base duration that doesn't snap with reduced motion set and one
//! that does, which must pass, and an ordinary page
//! (wrapping text, a wide table and a long line of code in boxes that scroll, an unbreakable word
//! that wraps anywhere, a figure wider than any window with mdBook's zoom, a sidebar drawer put
//! away off the left edge) that must pass. If one isn't
//! found as expected at every width, the check itself is broken, and it fails.
//!
//! The browser is Chrome or Chromium: [`CHROME_ENV`] names its program, else the usual places for
//! each system are tried. Without one the check fails, saying how to name it; it never passes
//! without having looked. The rustdoc under `api/` is rustdoc's own layout, and isn't checked.

use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde::Deserialize;

use super::html_files;

/// The narrowest window the pages are checked at, in CSS pixels: a small phone.
const NARROWEST_PX: u32 = 320;
/// The widest, in CSS pixels: a desktop monitor.
const WIDEST_PX: u32 = 1920;
/// The step between widths, in CSS pixels.
const STEP_PX: u32 = 40;
/// The window's height, in CSS pixels. The whole page is measured, not only what this shows.
const HEIGHT_PX: u32 = 900;
/// Frames each tab loads at once. Pages of one tab share its main thread, so more frames mostly
/// overlap the waits on the network and fonts.
const FRAMES_PER_TAB: usize = 3;
/// The most tabs, each its own browser process; fewer on a machine with fewer cores. With four,
/// the check took 114 s on an M-series Mac on 2026-10-09, with five readings of each page's style
/// at each width (#432).
const MOST_TABS: usize = 4;
/// How long the check waits for any news from the browser before it gives up.
const STALL: Duration = Duration::from_secs(300);
/// How long the whole check may take.
const DEADLINE: Duration = Duration::from_secs(30 * 60);
/// The environment variable that names the browser's program.
const CHROME_ENV: &str = "HPR_CHROME";
/// Where the check's own files are served, beside the site.
const CHECK_PREFIX: &str = "__check/";
/// The harness each tab runs: loads, measures and reports.
const HARNESS_JS: &str = include_str!("pages.js");
/// The page each tab opens, which runs [`HARNESS_JS`]. Its first script reports an error in the
/// harness itself, such as one that stops it from running at all.
const HARNESS_HTML: &str = "<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">\
<title>Page check</title><style>html, body { margin: 0; }</style>\
<script>addEventListener('error', function (event) { fetch('/__check/error', \
{ method: 'POST', body: String(event.message) }); });</script></head>\
<body><script src=\"/__check/pages.js\"></script></body></html>\n";
/// The largest request body the server takes: one page's results.
const MOST_BODY_BYTES: usize = 64 << 20;
/// The largest request head the server takes.
const MOST_HEAD_BYTES: usize = 64 << 10;

/// What the check found.
pub(super) struct Report {
    /// The pages of the site checked.
    pub pages: usize,
    /// The widths each page was checked at.
    pub widths: usize,
    /// How long the check took, in seconds.
    pub seconds: f64,
    /// One line per page and kind of problem; empty when every page passes.
    pub problems: Vec<String>,
}

/// The widths every page is checked at, in CSS pixels: 320 to 1,920 in steps of 40.
fn widths() -> Vec<u32> {
    (NARROWEST_PX..=WIDEST_PX)
        .step_by(STEP_PX as usize)
        .collect()
}

/// The kinds of problem a page can have at a width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    /// It scrolls sideways, or something reaches past the window's edge.
    Wide,
    /// Text cut off by a box that doesn't scroll.
    Cut,
    /// Text drawn over other text.
    Overlap,
    /// A figure shown smaller than drawn, with room to show it at its size or no zoom to.
    Shrunk,
    /// A color that is not one of the product system's roles in the page's theme (#383).
    Color,
    /// A rounded corner or a shadow (#383).
    Shape,
    /// Motion off the system's durations and easings, or any with reduced motion set (#383).
    Motion,
}

impl Kind {
    const ALL: [Kind; 7] = [
        Kind::Wide,
        Kind::Cut,
        Kind::Overlap,
        Kind::Shrunk,
        Kind::Color,
        Kind::Shape,
        Kind::Motion,
    ];

    fn describe(self) -> &'static str {
        match self {
            Kind::Wide => "wider than the window",
            Kind::Cut => "text cut off by its box",
            Kind::Overlap => "text over text",
            Kind::Shrunk => "a figure shown smaller than drawn",
            Kind::Color => "a color off the system's roles",
            Kind::Shape => "a rounded corner or a shadow",
            Kind::Motion => "motion off the system's durations",
        }
    }
}

/// A page built to test the check, served at `/__check/<file>`.
struct Canary {
    file: &'static str,
    /// What it is, for the message when it isn't found as expected.
    what: &'static str,
    /// The problems it must show at every width; it must show no other.
    expect: &'static [Kind],
    /// What it adds to the ordinary page: inside its `<main>`, which clips sideways as mdBook's
    /// page body does, and after it.
    inside: &'static str,
    after: &'static str,
}

/// The canaries ([`Canary`]), loaded on every run before the site's pages.
const CANARIES: [Canary; 25] = [
    Canary {
        file: "canary-ordinary.html",
        what: "an ordinary page, which must pass",
        expect: &[],
        inside: "",
        after: "",
    },
    Canary {
        file: "canary-past.html",
        what: "a label moved past the right edge",
        expect: &[Kind::Wide],
        inside: "",
        after: "<p>A label <span style=\"position: relative; left: 2000px\">moved out</span></p>",
    },
    Canary {
        file: "canary-past-content.html",
        what: "a label moved past the right edge inside the page's content, which scrolls",
        expect: &[Kind::Wide],
        inside: "<p>A label <span style=\"position: relative; left: 2000px\">moved out</span></p>",
        after: "",
    },
    Canary {
        file: "canary-past-clipped.html",
        what: "a label moved past the right edge inside a box that clips it",
        expect: &[Kind::Wide, Kind::Cut],
        inside: "<div style=\"overflow-x: clip\"><p>A label \
                 <span style=\"position: relative; left: 2000px\">moved out</span></p></div>",
        after: "",
    },
    Canary {
        file: "canary-past-left.html",
        what: "a label moved past the left edge inside the page's content",
        expect: &[Kind::Wide],
        inside: "<p>A label <span style=\"position: relative; left: -3000px\">moved out</span></p>",
        after: "",
    },
    Canary {
        file: "canary-cut.html",
        what: "a label cut short by its box",
        expect: &[Kind::Cut],
        inside: "<p style=\"width: 6em; white-space: nowrap; overflow: hidden; \
                 text-overflow: ellipsis\">A label much longer than its box</p>",
        after: "",
    },
    Canary {
        file: "canary-overlap.html",
        what: "two labels drawn over each other",
        expect: &[Kind::Overlap],
        inside: "<div style=\"position: relative; height: 3em\">\
                 <span style=\"position: absolute; left: 0; top: 0\">Apogee 1,234 m</span>\
                 <span style=\"position: absolute; left: 1em; top: 0.2em\">Top speed 210 m/s</span>\
                 </div>",
        after: "",
    },
    Canary {
        file: "canary-past-column.html",
        what: "a figure reaching past a narrow `main` on both sides, inside the page, as the \
               theme lays one out, which must pass",
        expect: &[],
        inside: "<style>main { overflow-x: visible; max-width: 300px; margin: 0 auto; } \
                 .content { container-type: inline-size; } \
                 .checkbox-label { display: flex; justify-content: center; width: 100cqi; \
                 margin-inline: calc((100% - 100cqi) / 2); }</style>",
        after: "",
    },
    Canary {
        file: "canary-past-visible.html",
        what: "a label moved past the right edge of a `main` whose overflow is visible, as \
               mdBook's is",
        expect: &[Kind::Wide],
        inside: "<style>main { overflow-x: visible; max-width: 300px; margin: 0 auto; } \
                 .content { container-type: inline-size; } \
                 .checkbox-label { display: flex; justify-content: center; width: 100cqi; \
                 margin-inline: calc((100% - 100cqi) / 2); }</style>\
                 <p>A label <span style=\"position: relative; left: 2000px\">moved out</span></p>",
        after: "",
    },
    Canary {
        file: "canary-figure-room.html",
        what: "a figure shown at half its size where the page has room for all of it",
        expect: &[Kind::Shrunk],
        inside: concat!(
            "<p><label class=\"checkbox-label\"><input class=\"checkbox-img\" type=\"checkbox\">",
            "<img src=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='200' ",
            "height='40'%3E%3C/svg%3E\" style=\"width: 100px\" alt=\"A figure\">",
            "<span class=\"img-wrapper\"><img src=\"data:image/svg+xml,%3Csvg ",
            "xmlns='http://www.w3.org/2000/svg' width='200' height='40'%3E%3C/svg%3E\" ",
            "alt=\"A figure\"></span></label></p>"
        ),
        after: "",
    },
    Canary {
        file: "canary-figure-no-zoom.html",
        what: "a figure wider than any window, with no zoom to see it at its size",
        expect: &[Kind::Shrunk],
        inside: concat!(
            "<p><img src=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' ",
            "width='4000' height='2000'%3E%3C/svg%3E\" alt=\"A wide figure\"></p>"
        ),
        after: "",
    },
    Canary {
        file: "canary-color.html",
        what: "a label in a highlighter's green, off the system's roles",
        expect: &[Kind::Color],
        inside: "<p style=\"color: #008200\">A label in green</p>",
        after: "",
    },
    Canary {
        file: "canary-radius.html",
        what: "a box with rounded corners",
        expect: &[Kind::Shape],
        inside: "<p style=\"border: 1px solid #566079; border-radius: 8px\">A rounded box</p>",
        after: "",
    },
    Canary {
        file: "canary-shadow.html",
        what: "a box with a soft shadow",
        expect: &[Kind::Shape],
        inside: "<p style=\"box-shadow: 0 4px 24px #0B0F1C\">A box with a shadow</p>",
        after: "",
    },
    Canary {
        file: "canary-motion.html",
        what: "a color change over 0.3 s, off the system's durations",
        expect: &[Kind::Motion],
        inside: "<p style=\"transition: color 0.3s ease\">A slow label</p>",
        after: "",
    },
    Canary {
        file: "canary-reduced-motion.html",
        what: "a color change at the system's base duration that doesn't snap with reduced \
               motion set",
        expect: &[Kind::Motion],
        inside: "<p style=\"transition: color 160ms cubic-bezier(0.2, 0, 0, 1)\">A label that \
                 never snaps</p>",
        after: "",
    },
    Canary {
        file: "canary-root.html",
        what: "the page's root element in a highlighter's green, which only a read of `html` sees",
        expect: &[Kind::Color],
        inside: "<style>html { background: #008200; }</style>",
        after: "",
    },
    Canary {
        file: "canary-filter.html",
        what: "a label tinted by a filter, as mdBook tints its copy icon, whose colors the check \
               can't read",
        expect: &[Kind::Color],
        inside: "<p style=\"filter: invert(45%)\">A tinted label</p>",
        after: "",
    },
    Canary {
        file: "canary-backdrop.html",
        what: "a label over a blurred backdrop, a filter whose colors the check can't read",
        expect: &[Kind::Color],
        inside: "<p style=\"backdrop-filter: blur(2px)\">A label over a blur</p>",
        after: "",
    },
    Canary {
        file: "canary-content-image.html",
        what: "an image drawn as a label's `::before`, whose colors the check can't read",
        expect: &[Kind::Color],
        inside: "<style>.icon::before { content: url(\"data:image/svg+xml,%3Csvg \
                 xmlns='http://www.w3.org/2000/svg' width='16' height='16'%3E%3C/svg%3E\"); }\
                 </style><p class=\"icon\">A label with an icon</p>",
        after: "",
    },
    Canary {
        file: "canary-mark.html",
        what: "a search hit filled in ink, a role, but not the action fill a selection takes",
        expect: &[Kind::Color],
        inside: "<p>A <mark style=\"color: #F3F4F7; background: #0B0F1C\">search hit</mark> in \
                 ink</p>",
        after: "",
    },
    Canary {
        file: "canary-no-script.html",
        what: "a label in a highlighter's green only for a reader with scripts off, read with \
               the page's `js` class taken off",
        expect: &[Kind::Color],
        inside: "<script>document.documentElement.classList.add('js');</script>\
                 <style>html:not(.js) .bare { color: #008200; }</style>\
                 <p class=\"bare\">Green only without scripts</p>",
        after: "",
    },
    Canary {
        file: "canary-no-script-slow.html",
        what: "a label that turns a highlighter's green 240 ms after scripts are off, keeping its \
               role color until then, and only with a light system: only the light reading with scripts \
               off, made after that transition, sees it",
        expect: &[Kind::Color],
        inside: "<script>document.documentElement.classList.add('js');</script>\
                 <style>.late { transition: color 0s 240ms; } \
                 html:not(.js) .late { color: #008200; } \
                 @media (prefers-color-scheme: dark) { html:not(.js) .late { color: #F3F4F7; } }\
                 </style>\
                 <p class=\"late\">Green after a delay</p>",
        after: "",
    },
    Canary {
        file: "canary-navy.html",
        what: "a label in a highlighter's green in the navy theme only, read after the switch \
               mdBook's theme menu makes",
        expect: &[Kind::Color],
        inside: "<script>document.documentElement.classList.add('js', 'light');</script>\
                 <link rel=\"stylesheet\" id=\"mdbook-tomorrow-night-css\" href=\"data:text/css,\">\
                 <style>.navy { color-scheme: dark; } .navy mark { background: #768DF5; } \
                 .navy .green { color: #008200; }</style>\
                 <p class=\"green\">Green only in navy</p>",
        after: "",
    },
    Canary {
        file: "canary-motion-snaps.html",
        what: "a color change at the system's base duration that snaps with reduced motion set, \
               which must pass",
        expect: &[],
        inside: "<style>.snaps { transition: color 160ms cubic-bezier(0.2, 0, 0, 1); } \
                 @media (prefers-reduced-motion: reduce) { .snaps { transition-duration: 0s; } }\
                 </style><p class=\"snaps\">A label that snaps</p>",
        after: "",
    },
];

/// A canary's page: an ordinary page, as mdBook lays one out, plus what the canary adds.
fn canary_html(canary: &Canary) -> String {
    let cells: String = (0..16)
        .map(|i| format!("<td>Mach {}.{}</td>", i / 4, (i % 4) * 25))
        .collect();
    let rows: String = (0..4).map(|_| format!("<tr>{cells}</tr>")).collect();
    let word = "Unbreakable".repeat(16);
    // A figure wider than any window, which mdBook's zoom shows at its size.
    let wide = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='4000' \
                height='2000'%3E%3C/svg%3E";
    let code = "let speed_m_per_s = drag_coefficient * reference_area_m2 * dynamic_pressure_pa \
                / mass_kg; // a line far longer than a narrow window";
    format!(
        "<!doctype html>
<html lang=\"en\"><head><meta charset=\"utf-8\">
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<title>{what}</title>
<style>
body {{ margin: 0; font: 16px/1.5 sans-serif; overflow-x: hidden; color: #0B0F1C; background: #F3F4F7; }}
html {{ color: #0B0F1C; background: #F3F4F7; }}
a {{ color: #3350D6; }}
mark {{ color: #FFFFFF; background: #3350D6; }}
@media (prefers-color-scheme: dark) {{ mark {{ color: #0B0F1C; background: #768DF5; }} }}
input {{ color: inherit; }}
.content {{ overflow-y: auto; }}
main {{ overflow-x: clip; }}
.content {{ padding: 0 16px; }}
img {{ max-width: 100%; }}
.checkbox-img {{ position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0, 0, 0, 0); }}
.checkbox-img:not(:checked) ~ .img-wrapper {{ display: none; }}
h1 {{ font-size: 2em; line-height: 1.2; }}
.scroll, pre {{ overflow-x: auto; }}
td {{ white-space: nowrap; padding: 0 12px; }}
.word {{ overflow-wrap: anywhere; }}
.sidebar {{ position: fixed; left: 0; top: 0; bottom: 0; width: 300px; transform: translateX(-100%); }}
</style></head><body>
<nav class=\"sidebar\"><ol><li><a href=\"#\">A chapter in a sidebar drawer put away</a></li></ol></nav>
<div class=\"content\"><main>
<h1>An ordinary page, with a title long enough to wrap in a narrow window</h1>
<p>Paragraphs wrap to the window. This one holds <code>inline code</code>, an area in
m<sup>2</sup>, a drag coefficient C<sub>D</sub> and a <a href=\"#\">link</a>, and runs on for a
few lines so that it wraps at every width the check tries.</p>
<div class=\"scroll\"><table>{rows}</table></div>
<pre><code>{code}</code></pre>
<p class=\"word\">{word}</p>
<p><label class=\"checkbox-label\"><input class=\"checkbox-img\" type=\"checkbox\"><img src=\"{wide}\" alt=\"A wide figure\"><span class=\"img-wrapper\"><img src=\"{wide}\" alt=\"A wide figure\"></span></label></p>
{inside}
</main></div>
{after}
</body></html>
",
        what = canary.what,
        inside = canary.inside,
        after = canary.after,
    )
}

/// Runs the page check on the site built in `output`.
pub(super) fn check(output: &Path) -> Result<Report, String> {
    let started = Instant::now();
    let mut pages = Vec::new();
    html_files(output, "", &mut pages)?;
    pages.retain(|page| !page.starts_with(&format!("{}/", super::API)));
    // The longest pages first, so the tabs finish together.
    let mut sized: Vec<(u64, String)> = pages
        .iter()
        .map(|page| {
            let bytes = fs::metadata(output.join(page)).map_or(0, |meta| meta.len());
            (bytes, page.clone())
        })
        .collect();
    sized.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let mut queue: Vec<String> = CANARIES
        .iter()
        .map(|canary| format!("{CHECK_PREFIX}{}", canary.file))
        .collect();
    queue.extend(sized.into_iter().map(|(_, page)| page));
    let total = queue.len();

    let chrome = find_chrome()?;
    let tabs = thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .clamp(1, MOST_TABS);
    let plan = serde_json::json!({
        "widths": widths(),
        "height": HEIGHT_PX,
        "parallel": FRAMES_PER_TAB,
    })
    .to_string();
    let (sender, receiver) = mpsc::channel();
    let server = Server::start(output.to_path_buf(), plan, queue, sender)?;
    let scratch = Scratch::new()?;
    let urls: Vec<String> = (0..tabs)
        .map(|tab| {
            format!(
                "http://{}/{CHECK_PREFIX}harness.html?tab={tab}",
                server.address
            )
        })
        .collect();
    let mut browser = Browser::launch(&chrome, &scratch, &urls)?;
    let collected = collect(&receiver, &mut browser, total, tabs);
    browser.stop();
    drop(server);
    let results = collected.map_err(|err| {
        let mut logs = String::new();
        for tab in 0..tabs {
            let log = fs::read_to_string(scratch.log(tab)).unwrap_or_default();
            let tail: Vec<&str> = log.lines().rev().take(10).collect();
            let tail: Vec<&str> = tail.into_iter().rev().collect();
            logs.push_str(&format!(
                "\n  browser {tab}'s log ends:\n    {}",
                tail.join("\n    ")
            ));
        }
        format!("the page check, in {}: {err}{logs}", chrome.display())
    })?;
    drop(scratch);

    for canary in &CANARIES {
        let page = format!("{CHECK_PREFIX}{}", canary.file);
        let measured = results
            .get(&page)
            .ok_or_else(|| format!("the page check is broken: no results for {page}"))?;
        canary_verdict(canary, measured)?;
    }
    let mut problems = Vec::new();
    for page in &pages {
        let measured = results
            .get(page)
            .ok_or_else(|| format!("the page check has no results for {page}"))?;
        problems.extend(page_problems(page, measured));
    }
    Ok(Report {
        pages: pages.len(),
        widths: widths().len(),
        seconds: started.elapsed().as_secs_f64(),
        problems,
    })
}

/// What the harness measured on one page at one width.
#[derive(Debug, Deserialize)]
struct Measured {
    width: u32,
    /// Why the page couldn't be measured, if it couldn't.
    error: Option<String>,
    #[serde(default)]
    client_width: f64,
    #[serde(default)]
    scroll_width: f64,
    /// Text and boxes past the window's edge, the first few.
    #[serde(default)]
    past: Vec<Example>,
    #[serde(default)]
    past_count: usize,
    /// Text cut off by a box, the first few.
    #[serde(default)]
    cut: Vec<Example>,
    #[serde(default)]
    cut_count: usize,
    /// Text over text, the first few.
    #[serde(default)]
    overlaps: Vec<Overlap>,
    #[serde(default)]
    overlap_count: usize,
    /// Figures shown smaller than drawn, the first few.
    #[serde(default)]
    shrunk: Vec<Figure>,
    #[serde(default)]
    shrunk_count: usize,
    /// mdBook's defaults by computed style (#383), the first few of each kind.
    #[serde(default)]
    color: Vec<Style>,
    #[serde(default)]
    color_count: usize,
    #[serde(default)]
    shape: Vec<Style>,
    #[serde(default)]
    shape_count: usize,
    #[serde(default)]
    motion: Vec<Style>,
    #[serde(default)]
    motion_count: usize,
}

impl Measured {
    fn has(&self, kind: Kind) -> bool {
        match kind {
            Kind::Wide => self.scroll_width > self.client_width || self.past_count > 0,
            Kind::Cut => self.cut_count > 0,
            Kind::Overlap => self.overlap_count > 0,
            Kind::Shrunk => self.shrunk_count > 0,
            Kind::Color => self.color_count > 0,
            Kind::Shape => self.shape_count > 0,
            Kind::Motion => self.motion_count > 0,
        }
    }

    /// The first example of `kind` at this width, in words.
    fn example(&self, kind: Kind) -> String {
        match kind {
            Kind::Wide => {
                let mut parts = Vec::new();
                if self.scroll_width > self.client_width {
                    parts.push(format!(
                        "the page is {} px wide in a {} px window",
                        self.scroll_width, self.client_width
                    ));
                }
                parts.extend(self.past.iter().map(|example| {
                    format!(
                        "`{}` (\"{}\") reaches {} px",
                        example.selector,
                        example.text,
                        example.right.unwrap_or_default()
                    )
                }));
                parts.join("; ")
            }
            Kind::Cut => self
                .cut
                .iter()
                .map(|example| {
                    format!(
                        "`{}` (\"{}\") loses {} px",
                        example.selector,
                        example.text,
                        example.lost.unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>()
                .join("; "),
            Kind::Overlap => self
                .overlaps
                .iter()
                .map(|overlap| {
                    format!(
                        "\"{}\" and \"{}\" overlap {} by {} px",
                        overlap.a, overlap.b, overlap.width, overlap.height
                    )
                })
                .collect::<Vec<_>>()
                .join("; "),
            Kind::Shrunk => self
                .shrunk
                .iter()
                .map(|figure| {
                    format!(
                        "`{}` (\"{}\") drawn {} px wide, shown {} px, with {}",
                        figure.selector, figure.text, figure.natural, figure.shown, figure.why
                    )
                })
                .collect::<Vec<_>>()
                .join("; "),
            Kind::Color => Style::list(&self.color),
            Kind::Shape => Style::list(&self.shape),
            Kind::Motion => Style::list(&self.motion),
        }
    }
}

/// An element whose computed style keeps one of mdBook's defaults.
#[derive(Debug, Deserialize)]
struct Style {
    /// A short path to its element, `tag#id.class>...`, and `::before` or `::after` for those.
    selector: String,
    /// The property, as CSS names it.
    property: String,
    /// Its computed value.
    value: String,
    /// The theme it was read in, and whether with reduced motion.
    theme: String,
}

impl Style {
    fn list(styles: &[Style]) -> String {
        styles
            .iter()
            .map(|style| {
                format!(
                    "`{}` has {} `{}` ({})",
                    style.selector, style.property, style.value, style.theme
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}

/// A figure shown smaller than drawn.
#[derive(Debug, Deserialize)]
struct Figure {
    /// A short path to its `img`, `tag#id.class>...`.
    selector: String,
    /// The start of its `src`.
    text: String,
    /// Its width as drawn and as shown, in CSS pixels.
    natural: f64,
    shown: f64,
    /// Why that fails: the room the page has for it, or no zoom.
    why: String,
}

/// A piece of text or a box that has a problem.
#[derive(Debug, Deserialize)]
struct Example {
    /// A short path to its element, `tag#id.class>...`.
    selector: String,
    /// The start of its text.
    text: String,
    /// How far it reaches, in CSS pixels, when it is past an edge: its right side past the
    /// window's right edge, or its left side, negative, past the left edge.
    right: Option<f64>,
    /// How much of it a box cuts off, in CSS pixels, when it is cut.
    lost: Option<f64>,
}

/// Two runs of text drawn over each other.
#[derive(Debug, Deserialize)]
struct Overlap {
    a: String,
    b: String,
    /// How far they overlap sideways and up and down, in CSS pixels.
    width: f64,
    height: f64,
}

/// One page's results, as the harness posts them.
#[derive(Debug, Deserialize)]
struct PageResults {
    page: String,
    widths: Vec<Measured>,
}

/// Reads one page's results, and checks they hold every width once, in order.
fn parse_results(body: &str) -> Result<(String, Vec<Measured>), String> {
    let results: PageResults =
        serde_json::from_str(body).map_err(|err| format!("unreadable results: {err}"))?;
    let got: Vec<u32> = results
        .widths
        .iter()
        .map(|measured| measured.width)
        .collect();
    if got != widths() {
        return Err(format!(
            "the results for {} are at widths {got:?}, not every width from {NARROWEST_PX} to \
             {WIDEST_PX} px",
            results.page
        ));
    }
    Ok((results.page, results.widths))
}

/// The widths in `list`, which is in order, as ranges of consecutive steps: `320 to 400, 480`.
fn ranges(list: &[u32]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut start = None;
    let mut last = 0;
    for &width in list {
        match start {
            Some(_) if width == last + STEP_PX => {}
            Some(first) => {
                parts.push(span(first, last));
                start = Some(width);
            }
            None => start = Some(width),
        }
        last = width;
    }
    if let Some(first) = start {
        parts.push(span(first, last));
    }
    parts.join(", ")
}

fn span(first: u32, last: u32) -> String {
    if first == last {
        first.to_string()
    } else {
        format!("{first} to {last}")
    }
}

/// One line per kind of problem `page` has, and one for the widths it didn't load at.
fn page_problems(page: &str, measured: &[Measured]) -> Vec<String> {
    let mut problems = Vec::new();
    let failed: Vec<&Measured> = measured.iter().filter(|m| m.error.is_some()).collect();
    if let Some(first) = failed.first() {
        let at: Vec<u32> = failed.iter().map(|m| m.width).collect();
        problems.push(format!(
            "{page}: could not be measured at {} px: {}",
            ranges(&at),
            first.error.as_deref().unwrap_or_default()
        ));
    }
    for kind in Kind::ALL {
        let hit: Vec<&Measured> = measured
            .iter()
            .filter(|m| m.error.is_none() && m.has(kind))
            .collect();
        if let Some(first) = hit.first() {
            let at: Vec<u32> = hit.iter().map(|m| m.width).collect();
            problems.push(format!(
                "{page}: {} at {} px; at {} px, {}",
                kind.describe(),
                ranges(&at),
                first.width,
                first.example(kind)
            ));
        }
    }
    problems
}

/// Whether a canary was found as it should be: each problem it expects at every width, no other.
fn canary_verdict(canary: &Canary, measured: &[Measured]) -> Result<(), String> {
    let broken = |what: String| {
        format!(
            "the page check is broken: {CHECK_PREFIX}{} ({}) {what}",
            canary.file, canary.what
        )
    };
    for m in measured {
        if let Some(err) = &m.error {
            return Err(broken(format!(
                "could not be measured at {} px: {err}",
                m.width
            )));
        }
        for kind in Kind::ALL {
            let expected = canary.expect.contains(&kind);
            if m.has(kind) != expected {
                let verb = if expected {
                    "was not found"
                } else {
                    "was found"
                };
                let example = if expected {
                    String::new()
                } else {
                    format!(": {}", m.example(kind))
                };
                return Err(broken(format!(
                    "at {} px: `{}` {verb}{example}",
                    m.width,
                    kind.describe()
                )));
            }
        }
    }
    Ok(())
}

/// What the server hands the check's main thread.
#[derive(Debug)]
enum News {
    /// One page's results, as posted.
    Results(String),
    /// A tab has no pages left.
    Done,
    /// The harness failed.
    Failed(String),
}

/// Waits for every tab to finish, gathering each page's results.
fn collect(
    receiver: &Receiver<News>,
    browser: &mut Browser,
    pages: usize,
    tabs: usize,
) -> Result<BTreeMap<String, Vec<Measured>>, String> {
    let started = Instant::now();
    let mut last_news = Instant::now();
    let mut results = BTreeMap::new();
    let mut done = 0;
    while done < tabs {
        if started.elapsed() > DEADLINE {
            return Err(format!(
                "not finished in {} minutes ({} of {pages} pages done)",
                DEADLINE.as_secs() / 60,
                results.len()
            ));
        }
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(News::Results(body)) => {
                let (page, measured) = parse_results(&body)?;
                results.insert(page, measured);
                last_news = Instant::now();
            }
            Ok(News::Done) => {
                done += 1;
                last_news = Instant::now();
            }
            Ok(News::Failed(message)) => return Err(format!("the harness failed: {message}")),
            Err(RecvTimeoutError::Disconnected) => return Err("the server stopped".to_owned()),
            Err(RecvTimeoutError::Timeout) => {
                if let Some(status) = browser.exited() {
                    return Err(format!(
                        "the browser exited ({status}) with {} of {pages} pages done",
                        results.len()
                    ));
                }
                if last_news.elapsed() > STALL {
                    return Err(format!(
                        "no news from the browser for {} s ({} of {pages} pages done)",
                        STALL.as_secs(),
                        results.len()
                    ));
                }
            }
        }
    }
    if results.len() != pages {
        return Err(format!(
            "results for {} of {pages} pages; the tabs finished without the rest",
            results.len()
        ));
    }
    Ok(results)
}

/// The browser's program: [`CHROME_ENV`], else the first of the usual places that exists.
fn find_chrome() -> Result<PathBuf, String> {
    if let Some(named) = std::env::var_os(CHROME_ENV).filter(|value| !value.is_empty()) {
        let path = PathBuf::from(&named);
        if path.is_file() {
            return Ok(path);
        }
        if let Some(found) = on_path(&named.to_string_lossy()) {
            return Ok(found);
        }
        return Err(format!(
            "{CHROME_ENV} is `{}`, which is not a program; set it to the path of Chrome's or \
             Chromium's program",
            path.display()
        ));
    }
    let mut tried = Vec::new();
    for candidate in usual_places() {
        if candidate.is_file() {
            return Ok(candidate);
        }
        tried.push(candidate.display().to_string());
    }
    for name in [
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "chrome",
    ] {
        if let Some(found) = on_path(name) {
            return Ok(found);
        }
        tried.push(format!("{name} on PATH"));
    }
    Err(format!(
        "the page check needs Chrome or Chromium, and found neither (tried {}); install one, or \
         set {CHROME_ENV} to the path of its program, such as `{CHROME_ENV}=/usr/bin/chromium`",
        tried.join(", ")
    ))
}

/// Where Chrome and Chromium install on this system, outside `PATH`.
fn usual_places() -> Vec<PathBuf> {
    let mut places = Vec::new();
    if cfg!(target_os = "macos") {
        for app in [
            "Google Chrome.app/Contents/MacOS/Google Chrome",
            "Chromium.app/Contents/MacOS/Chromium",
        ] {
            places.push(Path::new("/Applications").join(app));
            if let Some(home) = std::env::var_os("HOME") {
                places.push(Path::new(&home).join("Applications").join(app));
            }
        }
    } else if cfg!(windows) {
        for base in ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"] {
            if let Some(dir) = std::env::var_os(base) {
                places.push(Path::new(&dir).join("Google/Chrome/Application/chrome.exe"));
                places.push(Path::new(&dir).join("Chromium/Application/chrome.exe"));
            }
        }
    }
    places
}

/// `name` in a directory on `PATH`, if it is there.
fn on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exe = if cfg!(windows) && !name.ends_with(".exe") {
        format!("{name}.exe")
    } else {
        name.to_owned()
    };
    std::env::split_paths(&path)
        .map(|dir| dir.join(&exe))
        .find(|candidate| candidate.is_file())
}

/// A fresh folder for the browser's profile and log, removed when dropped.
struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    fn new() -> Result<Scratch, String> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        let dir =
            std::env::temp_dir().join(format!("hpr-page-check-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir)
            .map_err(|err| format!("could not create {}: {err}", dir.display()))?;
        Ok(Scratch { dir })
    }

    fn profile(&self, tab: usize) -> PathBuf {
        self.dir.join(format!("profile-{tab}"))
    }

    fn log(&self, tab: usize) -> PathBuf {
        self.dir.join(format!("browser-{tab}.log"))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // The browser's helpers can take a moment to let go of the profile after it exits.
        for _ in 0..20 {
            if fs::remove_dir_all(&self.dir).is_ok() || !self.dir.exists() {
                return;
            }
            thread::sleep(Duration::from_millis(100));
        }
        eprintln!("note: could not remove {}", self.dir.display());
    }
}

/// The headless browsers, one per tab (headless Chrome opens one page), stopped when dropped.
struct Browser {
    children: Vec<Child>,
}

impl Browser {
    fn launch(chrome: &Path, scratch: &Scratch, urls: &[String]) -> Result<Browser, String> {
        let mut browser = Browser {
            children: Vec::new(),
        };
        for (tab, url) in urls.iter().enumerate() {
            browser
                .children
                .push(launch_one(chrome, scratch, tab, url)?);
        }
        Ok(browser)
    }

    /// How a browser exited, if one has.
    fn exited(&mut self) -> Option<String> {
        self.children
            .iter_mut()
            .find_map(|child| match child.try_wait() {
                Ok(Some(status)) => Some(status.to_string()),
                Ok(None) => None,
                Err(err) => Some(err.to_string()),
            })
    }

    fn stop(&mut self) {
        for mut child in self.children.drain(..) {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// Starts one headless browser, with its own fresh profile, on `url`.
fn launch_one(chrome: &Path, scratch: &Scratch, tab: usize, url: &str) -> Result<Child, String> {
    let log_path = scratch.log(tab);
    let log = fs::File::create(&log_path)
        .map_err(|err| format!("could not create {}: {err}", log_path.display()))?;
    let profile = scratch.profile(tab);
    fs::create_dir_all(&profile)
        .map_err(|err| format!("could not create {}: {err}", profile.display()))?;
    let mut command = Command::new(chrome);
    command
        .arg("--headless=new")
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-gpu")
        .arg("--disable-extensions")
        .arg("--disable-background-networking")
        .arg("--window-size=1920,1080")
        // A light system, on every machine: mdBook picks a page's first theme by it, and a page
        // read without scripts is read in that one only, so the check would differ by host.
        .arg("--blink-settings=preferredColorScheme=1")
        .arg(format!("--user-data-dir={}", profile.display()));
    if cfg!(target_os = "macos") {
        // No prompt for the keychain from a fresh profile.
        command.arg("--use-mock-keychain");
    }
    if cfg!(target_os = "linux") {
        // The pages are the site just built, served from this machine, so the sandbox guards
        // nothing here; and Ubuntu's AppArmor (24.04 on) keeps it from starting on CI.
        command.arg("--no-sandbox").arg("--password-store=basic");
    }
    let err_log = log
        .try_clone()
        .map_err(|err| format!("could not open the browser's log: {err}"))?;
    command
        .arg(url)
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(err_log)
        .spawn()
        .map_err(|err| format!("could not start {}: {err}", chrome.display()))
}

impl Drop for Browser {
    fn drop(&mut self) {
        self.stop();
    }
}

/// The static file server the browser loads the site and the harness from.
struct Server {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

/// What every connection's thread shares.
struct Shared {
    /// The built site.
    site: PathBuf,
    /// The plan, as `/__check/pages.json` serves it.
    plan: String,
    /// The pages no tab has taken yet.
    queue: Mutex<VecDeque<String>>,
    sender: Mutex<Sender<News>>,
}

impl Server {
    fn start(
        site: PathBuf,
        plan: String,
        queue: Vec<String>,
        sender: Sender<News>,
    ) -> Result<Server, String> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|err| format!("could not start the page check's server: {err}"))?;
        let address = listener
            .local_addr()
            .map_err(|err| format!("could not read the server's address: {err}"))?;
        let stop = Arc::new(AtomicBool::new(false));
        let shared = Arc::new(Shared {
            site,
            plan,
            queue: Mutex::new(queue.into()),
            sender: Mutex::new(sender),
        });
        let stopping = Arc::clone(&stop);
        let thread = thread::spawn(move || {
            for stream in listener.incoming() {
                if stopping.load(Ordering::SeqCst) {
                    break;
                }
                let Ok(stream) = stream else { continue };
                let shared = Arc::clone(&shared);
                thread::spawn(move || {
                    // A connection that fails is the browser's to report, as a load that failed.
                    let _ = serve(stream, &shared);
                });
            }
        });
        Ok(Server {
            address,
            stop,
            thread: Some(thread),
        })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        // Wakes the listener, which then sees `stop`.
        let _ = TcpStream::connect(self.address);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// A response: status, content type and body.
type Response = (u16, &'static str, Vec<u8>);

/// Answers one request on `stream`.
fn serve(stream: TcpStream, shared: &Shared) -> Result<(), String> {
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|err| err.to_string())?;
    let mut writer = stream.try_clone().map_err(|err| err.to_string())?;
    let mut reader = BufReader::new(stream);
    let mut head_bytes = 0;
    let mut request_line = String::new();
    head_bytes += reader
        .read_line(&mut request_line)
        .map_err(|err| err.to_string())?;
    let mut length = 0usize;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line).map_err(|err| err.to_string())?;
        head_bytes += read;
        if read == 0 || line == "\r\n" || line == "\n" || head_bytes > MOST_HEAD_BYTES {
            break;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.trim().eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().unwrap_or(usize::MAX);
        }
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();
    let (status, kind, body) = if head_bytes > MOST_HEAD_BYTES {
        (431, "text/plain", b"request head too large".to_vec())
    } else if length > MOST_BODY_BYTES {
        (413, "text/plain", b"request body too large".to_vec())
    } else {
        let mut body = vec![0; length];
        reader
            .read_exact(&mut body)
            .map_err(|err| err.to_string())?;
        respond(method, target, body, shared)
    };
    let head = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\n\
         Cache-Control: max-age=600\r\nConnection: close\r\n\r\n",
        reason(status),
        body.len()
    );
    writer
        .write_all(head.as_bytes())
        .map_err(|err| err.to_string())?;
    if method != "HEAD" {
        writer.write_all(&body).map_err(|err| err.to_string())?;
    }
    writer.flush().map_err(|err| err.to_string())
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Content Too Large",
        431 => "Request Header Fields Too Large",
        _ => "Internal Server Error",
    }
}

/// The response to one request: the check's own paths, else a file of the site.
fn respond(method: &str, target: &str, body: Vec<u8>, shared: &Shared) -> Response {
    let Some(path) = site_path(target) else {
        return (400, "text/plain", b"bad path".to_vec());
    };
    if let Some(name) = path.strip_prefix(CHECK_PREFIX) {
        return respond_check(method, name, body, shared);
    }
    if method != "GET" && method != "HEAD" {
        return (405, "text/plain", b"GET only".to_vec());
    }
    let mut file = shared.site.join(&path);
    if file.is_dir() {
        file = file.join("index.html");
    }
    match fs::read(&file) {
        Ok(bytes) => (200, content_type(&path), bytes),
        Err(_) => (404, "text/plain", b"not found".to_vec()),
    }
}

/// The check's own paths under `/__check/`.
fn respond_check(method: &str, name: &str, body: Vec<u8>, shared: &Shared) -> Response {
    let send = |news: News| {
        if let Ok(sender) = shared.sender.lock() {
            // The main thread has stopped listening only once it has failed already.
            let _ = sender.send(news);
        }
    };
    let text = || String::from_utf8_lossy(&body).into_owned();
    match (method, name) {
        ("GET", "harness.html") => (200, "text/html; charset=utf-8", HARNESS_HTML.into()),
        ("GET", "pages.js") => (200, "text/javascript; charset=utf-8", HARNESS_JS.into()),
        ("GET", "pages.json") => (200, "application/json", shared.plan.clone().into_bytes()),
        ("POST", "next") => {
            let next = shared
                .queue
                .lock()
                .ok()
                .and_then(|mut queue| queue.pop_front());
            let answer = serde_json::json!({ "page": next }).to_string();
            (200, "application/json", answer.into_bytes())
        }
        ("POST", "result") => {
            send(News::Results(text()));
            (200, "application/json", b"{}".to_vec())
        }
        ("POST", "done") => {
            send(News::Done);
            (200, "application/json", b"{}".to_vec())
        }
        ("POST", "error") => {
            send(News::Failed(text()));
            (200, "application/json", b"{}".to_vec())
        }
        ("GET", _) => match CANARIES.iter().find(|canary| canary.file == name) {
            Some(canary) => (200, "text/html; charset=utf-8", canary_html(canary).into()),
            None => (404, "text/plain", b"not found".to_vec()),
        },
        _ => (405, "text/plain", b"not allowed".to_vec()),
    }
}

/// The file a request's target names, relative to the site: query and fragment dropped, escapes
/// decoded, `index.html` for a folder. `None` for a target that would leave the site (`..`), or
/// names a drive or a backslash path, or isn't an absolute path.
fn site_path(target: &str) -> Option<String> {
    let path = target.split(['?', '#']).next()?;
    let path = path.strip_prefix('/')?;
    let decoded = percent_decode(path)?;
    let mut parts = Vec::new();
    for part in decoded.split('/') {
        if part.is_empty() {
            continue;
        }
        if part == "." || part == ".." || part.contains(['\\', ':', '\0']) {
            return None;
        }
        parts.push(part);
    }
    let mut out = parts.join("/");
    if out.is_empty() {
        out.push_str("index.html");
    } else if decoded.ends_with('/') {
        out.push_str("/index.html");
    }
    Some(out)
}

/// `%XX` escapes decoded; `None` for a broken escape or bytes that aren't UTF-8.
fn percent_decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// The content type a file of the site is served with, by its extension.
fn content_type(path: &str) -> &'static str {
    let extension = path.rsplit_once('.').map_or("", |(_, ext)| ext);
    match extension.to_ascii_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "txt" | "md" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_run_from_320_to_1920_in_steps_of_40() {
        let list = widths();
        assert_eq!(list.len(), 41);
        assert_eq!(list.first(), Some(&320));
        assert_eq!(list.last(), Some(&1920));
        assert!(list.windows(2).all(|pair| pair[1] - pair[0] == 40));
    }

    #[test]
    fn ranges_join_consecutive_widths() {
        assert_eq!(ranges(&[]), "");
        assert_eq!(ranges(&[320]), "320");
        assert_eq!(ranges(&[320, 360, 400]), "320 to 400");
        assert_eq!(ranges(&[320, 360, 400, 480]), "320 to 400, 480");
        assert_eq!(ranges(&[320, 400, 440, 1920]), "320, 400 to 440, 1920");
    }

    #[test]
    fn paths_stay_inside_the_site() {
        assert_eq!(site_path("/"), Some("index.html".to_owned()));
        assert_eq!(site_path("/a.html?x=1#y"), Some("a.html".to_owned()));
        assert_eq!(
            site_path("/physics/"),
            Some("physics/index.html".to_owned())
        );
        assert_eq!(site_path("/a%20b.html"), Some("a b.html".to_owned()));
        assert_eq!(site_path("//a.html"), Some("a.html".to_owned()));
        assert_eq!(site_path("/../secret"), None);
        assert_eq!(site_path("/a/../../secret"), None);
        assert_eq!(site_path("/%2e%2e/secret"), None);
        assert_eq!(site_path("/a/%2E%2E/b"), None);
        assert_eq!(site_path("/..%5csecret"), None);
        assert_eq!(site_path("/C:/Windows"), None);
        assert_eq!(site_path("/a%00b"), None);
        assert_eq!(site_path("/a%zz"), None);
        assert_eq!(site_path("/a%2"), None);
        assert_eq!(site_path("/%ff"), None);
        assert_eq!(site_path("a.html"), None);
        assert_eq!(site_path("http://example.com/a"), None);
    }

    /// One width's results as the harness posts them.
    fn width_json(width: u32, scroll: u32, past: usize, cut: usize, overlap: usize) -> String {
        let examples = |n: usize, field: &str| {
            let list: Vec<String> = (0..n.min(3))
                .map(|_| format!(r#"{{"selector":"main>p","text":"Apogee","{field}":2400}}"#))
                .collect();
            list.join(",")
        };
        let overlaps: Vec<String> = (0..overlap.min(3))
            .map(|_| r#"{"a":"Apogee","b":"Top speed","width":40,"height":12}"#.to_owned())
            .collect();
        format!(
            r#"{{"width":{width},"error":null,"client_width":{cw},"scroll_width":{scroll},
            "past":[{past_list}],"past_count":{past},"cut":[{cut_list}],"cut_count":{cut},
            "overlaps":[{overlap_list}],"overlap_count":{overlap}}}"#,
            cw = width - 15,
            past_list = examples(past, "right"),
            cut_list = examples(cut, "lost"),
            overlap_list = overlaps.join(","),
        )
    }

    fn page_json(page: &str, each: impl Fn(u32) -> String) -> String {
        let list: Vec<String> = widths().into_iter().map(each).collect();
        format!(r#"{{"page":"{page}","widths":[{}]}}"#, list.join(","))
    }

    #[test]
    fn results_parse_and_name_each_problem() {
        let body = page_json("guide.html", |w| {
            let fits = w - 15;
            match w {
                320..=400 => width_json(w, fits + 90, 1, 0, 0),
                480 => width_json(w, fits, 2, 0, 0),
                1920 => width_json(w, fits, 0, 1, 4),
                _ => width_json(w, fits, 0, 0, 0),
            }
        });
        let (page, measured) = parse_results(&body).unwrap();
        assert_eq!(page, "guide.html");
        let problems = page_problems(&page, &measured);
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert!(problems[0].starts_with("guide.html: wider than the window at 320 to 400, 480 px; at 320 px, the page is 395 px wide in a 305 px window; `main>p` (\"Apogee\") reaches 2400 px"), "{}", problems[0]);
        assert!(
            problems[1].starts_with("guide.html: text cut off by its box at 1920 px"),
            "{}",
            problems[1]
        );
        assert!(problems[2].contains("text over text at 1920 px; at 1920 px, \"Apogee\" and \"Top speed\" overlap 40 by 12 px"), "{}", problems[2]);
    }

    #[test]
    fn mdbook_defaults_are_named_by_kind() {
        let body = page_json("guide.html", |w| {
            let plain = width_json(w, w - 15, 0, 0, 0);
            plain.replacen(
                "\"overlap_count\":0}",
                r#""overlap_count":0,"color":[{"selector":"main>pre>code>span.hljs-string","property":"color","value":"rgb(0, 130, 0)","theme":"light"}],"color_count":1,"shape":[],"shape_count":0,"motion":[{"selector":"nav.sidebar","property":"transition","value":"0.3s ease","theme":"navy"}],"motion_count":2}"#,
                1,
            )
        });
        let (page, measured) = parse_results(&body).unwrap();
        let problems = page_problems(&page, &measured);
        assert_eq!(problems.len(), 2, "{problems:#?}");
        assert_eq!(
            problems[0],
            "guide.html: a color off the system's roles at 320 to 1920 px; at 320 px, \
             `main>pre>code>span.hljs-string` has color `rgb(0, 130, 0)` (light)"
        );
        assert_eq!(
            problems[1],
            "guide.html: motion off the system's durations at 320 to 1920 px; at 320 px, \
             `nav.sidebar` has transition `0.3s ease` (navy)"
        );
        // Each kind has a canary that must show it, and the ordinary page and one that snaps
        // with reduced motion must show none.
        for kind in [Kind::Color, Kind::Shape, Kind::Motion] {
            assert!(
                CANARIES.iter().any(|canary| canary.expect == [kind]),
                "{kind:?}"
            );
        }
        let passing: Vec<&str> = CANARIES
            .iter()
            .filter(|canary| canary.expect.is_empty())
            .map(|canary| canary.file)
            .collect();
        assert!(passing.contains(&"canary-motion-snaps.html"), "{passing:?}");
    }

    #[test]
    fn a_figure_shown_smaller_than_drawn_is_named() {
        let body = page_json("cli.html", |w| {
            let plain = width_json(w, w - 15, 0, 0, 0);
            if w < 1320 {
                return plain;
            }
            plain.replacen(
                "\"overlap_count\":0}",
                r#""overlap_count":0,"shrunk":[{"selector":"main>p>label>img","text":"images/sim-plot.svg","natural":960,"shown":750,"why":"room for 960 px"}],"shrunk_count":1}"#,
                1,
            )
        });
        let (page, measured) = parse_results(&body).unwrap();
        let problems = page_problems(&page, &measured);
        assert_eq!(problems.len(), 1, "{problems:#?}");
        assert!(
            problems[0].starts_with(
                "cli.html: a figure shown smaller than drawn at 1320 to 1920 px; at 1320 px, \
                 `main>p>label>img` (\"images/sim-plot.svg\") drawn 960 px wide, shown 750 px, \
                 with room for 960 px"
            ),
            "{}",
            problems[0]
        );
        // The canaries hold both ways the rule fails, and the ordinary page a zoomable figure.
        let shrunk: Vec<&str> = CANARIES
            .iter()
            .filter(|canary| canary.expect == [Kind::Shrunk])
            .map(|canary| canary.file)
            .collect();
        assert_eq!(
            shrunk,
            ["canary-figure-room.html", "canary-figure-no-zoom.html"]
        );
        assert!(canary_html(&CANARIES[0]).contains("class=\"checkbox-img\""));
    }

    #[test]
    fn a_page_without_problems_has_none() {
        let body = page_json("index.html", |w| width_json(w, w - 15, 0, 0, 0));
        let (page, measured) = parse_results(&body).unwrap();
        assert!(page_problems(&page, &measured).is_empty());
    }

    #[test]
    fn a_failed_load_is_a_problem() {
        let body = page_json("index.html", |w| {
            if w == 360 {
                r#"{"width":360,"error":"did not load in 60 s"}"#.to_owned()
            } else {
                width_json(w, w - 15, 0, 0, 0)
            }
        });
        let (page, measured) = parse_results(&body).unwrap();
        let problems = page_problems(&page, &measured);
        assert_eq!(
            problems,
            ["index.html: could not be measured at 360 px: did not load in 60 s"]
        );
    }

    #[test]
    fn results_must_hold_every_width() {
        let body = r#"{"page":"a.html","widths":[{"width":320,"error":null}]}"#;
        assert!(parse_results(body).unwrap_err().contains("not every width"));
        assert!(parse_results("{").unwrap_err().contains("unreadable"));
    }

    /// The canary served as `file`.
    fn canary(file: &str) -> &'static Canary {
        CANARIES
            .iter()
            .find(|c| c.file == file)
            .unwrap_or_else(|| panic!("no canary {file}"))
    }

    #[test]
    fn canaries_must_show_what_they_expect_and_nothing_else() {
        let overlap = canary("canary-overlap.html");
        assert_eq!(overlap.expect, &[Kind::Overlap]);
        let found = page_json("x", |w| width_json(w, w - 15, 0, 0, 1));
        let (_, measured) = parse_results(&found).unwrap();
        assert!(canary_verdict(overlap, &measured).is_ok());
        // Missed at one width: the check is broken.
        let missed = page_json("x", |w| width_json(w, w - 15, 0, 0, usize::from(w != 760)));
        let (_, measured) = parse_results(&missed).unwrap();
        let err = canary_verdict(overlap, &measured).unwrap_err();
        assert!(
            err.contains("at 760 px: `text over text` was not found"),
            "{err}"
        );
        // Found with another problem: broken too.
        let extra = page_json("x", |w| width_json(w, w - 15, 0, 1, 1));
        let (_, measured) = parse_results(&extra).unwrap();
        let err = canary_verdict(overlap, &measured).unwrap_err();
        assert!(err.contains("`text cut off by its box` was found"), "{err}");
        // The ordinary page passes only with nothing found.
        let ordinary = canary("canary-ordinary.html");
        let clean = page_json("x", |w| width_json(w, w - 15, 0, 0, 0));
        let (_, measured) = parse_results(&clean).unwrap();
        assert!(canary_verdict(ordinary, &measured).is_ok());
        let wide = page_json("x", |w| width_json(w, w + 10, 0, 0, 0));
        let (_, measured) = parse_results(&wide).unwrap();
        assert!(canary_verdict(ordinary, &measured).is_err());
    }

    /// The left edge has its canary, and the ordinary page a drawer put away that must pass.
    #[test]
    fn the_left_edge_has_canaries_both_ways() {
        let left = canary("canary-past-left.html");
        assert_eq!(left.expect, &[Kind::Wide]);
        assert!(left.inside.contains("left: -3000px"));
        let ordinary = canary_html(canary("canary-ordinary.html"));
        assert!(ordinary.contains("<nav class=\"sidebar\">"));
        assert!(ordinary.contains("transform: translateX(-100%)"));
    }

    #[test]
    fn every_canary_is_served_and_named_once() {
        let mut files: Vec<&str> = CANARIES.iter().map(|canary| canary.file).collect();
        files.sort_unstable();
        files.dedup();
        assert_eq!(files.len(), CANARIES.len());
        for canary in &CANARIES {
            let html = canary_html(canary);
            assert!(html.contains(canary.inside) && html.contains(canary.after));
            assert!(html.starts_with("<!doctype html>"));
        }
    }

    /// Sends one raw request to `address` and returns the status line and body.
    fn request(address: SocketAddr, raw: &str) -> (String, String) {
        let mut stream = TcpStream::connect(address).unwrap();
        stream.write_all(raw.as_bytes()).unwrap();
        let mut answer = String::new();
        stream.read_to_string(&mut answer).unwrap();
        let (head, body) = answer.split_once("\r\n\r\n").unwrap();
        (head.lines().next().unwrap().to_owned(), body.to_owned())
    }

    #[test]
    fn the_server_serves_the_site_and_refuses_to_leave_it() {
        let dir = tempfile::tempdir().unwrap();
        let site = dir.path().join("site");
        fs::create_dir_all(site.join("physics")).unwrap();
        fs::write(site.join("index.html"), "landing").unwrap();
        fs::write(site.join("physics/index.html"), "physics").unwrap();
        fs::write(dir.path().join("secret.txt"), "secret").unwrap();
        let (sender, receiver) = mpsc::channel();
        let server = Server::start(
            site,
            "{\"plan\":1}".to_owned(),
            vec!["index.html".to_owned()],
            sender,
        )
        .unwrap();
        let get = |path: &str| request(server.address, &format!("GET {path} HTTP/1.1\r\n\r\n"));
        assert_eq!(
            get("/"),
            ("HTTP/1.1 200 OK".to_owned(), "landing".to_owned())
        );
        assert_eq!(get("/physics/").1, "physics");
        assert_eq!(get("/physics").1, "physics");
        assert!(get("/../secret.txt").0.contains("400"));
        assert!(get("/%2e%2e/secret.txt").0.contains("400"));
        assert!(get("/missing.html").0.contains("404"));
        assert_eq!(get("/__check/pages.json").1, "{\"plan\":1}");
        assert!(get("/__check/pages.js").1.contains("/__check/result"));
        assert!(
            get("/__check/canary-overlap.html")
                .1
                .contains("Top speed 210 m/s")
        );
        let post = |path: &str, body: &str| {
            request(
                server.address,
                &format!(
                    "POST {path} HTTP/1.1\r\nContent-Length: {}\r\n\r\n{body}",
                    body.len()
                ),
            )
        };
        assert_eq!(post("/__check/next", "{}").1, "{\"page\":\"index.html\"}");
        assert_eq!(post("/__check/next", "{}").1, "{\"page\":null}");
        post("/__check/result", "{\"page\":\"index.html\"}");
        post("/__check/done", "{}");
        assert!(matches!(
            receiver.recv().unwrap(),
            News::Results(body) if body == "{\"page\":\"index.html\"}"
        ));
        assert!(matches!(receiver.recv().unwrap(), News::Done));
        let big = format!(
            "POST /__check/result HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
            MOST_BODY_BYTES + 1
        );
        assert!(request(server.address, &big).0.contains("413"));
        assert!(post("/index.html", "x").0.contains("405"));
    }
}
