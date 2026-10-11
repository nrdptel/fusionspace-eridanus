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
//!     stylesheets' reduced-motion rules applied in place, any motion at all;
//!   - **a selection off the action role**: each element that shows text of its own must draw
//!     selected text on the action role at 22 % opacity (`::selection`, as the system's
//!     `fusionspace.css` sets it; `principles.md`, *One sweep*), not the browser's own.
//!
//!   - **a text mark drawn as an icon**: a text of marks alone (arrows, technical symbols,
//!     shapes, symbols and dingbats, guillemets, an icon font's private characters), in the
//!     page's chrome, outside its `main`, or as a `::before` or `::after` anywhere: mdBook's `❱`
//!     fold toggles, its `✓` beside the chosen theme and its `»` before a heading jumped to. The
//!     first reading reads every element with an id as the one jumped to (each `:target` rule
//!     copied with an attribute in its place). A key's legend (`kbd`) and prose in `main` may
//!     hold one.
//!
//!   Each page is read in the theme it loads in (mdBook's navy for a browser whose system is
//!   dark), then as a reader with scripts off gets it (the root's `js` class taken off, which
//!   applies mdBook's `html:not(.js)` rules), with a light system and with a dark one (the
//!   stylesheets' dark-scheme rules applied in place), then switched as mdBook's theme menu
//!   switches it and read in the other once the switch's transitions have run. Hover isn't
//!   read, nor what a script sets as it runs; focus and the script's smooth scroll are read
//!   below.
//! - **The page's frame** (#384, ADR-223), read once at each width in the theme the page loads
//!   in:
//!   - **type off the system's scale** (`foundations.md`, *Type*): every element that shows text,
//!     drawn or hidden until asked for, its `::before` and `::after` and every text field
//!     included, must have the size and line height of one of the scale's tokens in that token's
//!     family (Cascadia Mono 12/16, 14/20, 20/24, 28/32, 40/44; Archivo 14/20, 16/24, 20/28),
//!     capitals only at Cascadia Mono 12/16 and 14/20, no weight under 400, tabular figures
//!     in Archivo, and each family's metric-matched fallback (the system's `fonts.css`) second
//!     in its stack. A superscript or subscript may be off the scale, never under 12 px. The
//!     sidebar's text must be Cascadia Mono 14 px, and its current page underlined 2 px
//!     (`web.md`, *Page anatomy*). Boxes of 2 px or less, for screen readers, are skipped;
//!   - **a line of prose past 68 characters**: each paragraph, list item, quote or caption in
//!     `main`, outside tables and code, counted line by line as drawn, spaces between words
//!     counted once. A character more than half a line below the one before starts a new line;
//!     a word broken across lines is split where it breaks; characters are counted by code
//!     point;
//!   - **gutters off the system's** (`foundations.md`, *Width*): `main` must sit at least 16 px
//!     from each side of the pane that holds it (mdBook's `.page-wrapper`, beside the sidebar)
//!     in a window under 720 px, and 32 px from 720 px, and no further on either side unless
//!     it has reached its widest;
//!   - **a code block or table held to the prose measure**: one in `main` that scrolls sideways
//!     inside a box with a `max-width`, as a list item around it would be at the measure;
//!   - **an icon off the system's sizes** (`foundations.md`, *Icons*): each icon drawn in the
//!     spans mdBook keeps its chrome's icons in (`.fa-svg`, the menu bar's buttons, the page
//!     arrows, the search spinner, a code block's buttons), drawn now at this width, must be
//!     square at 16, 20 or 24 px. Which icons they are is the file check's (`chrome.rs`);
//!   - **a header off the system's** (`web.md`, *Page anatomy*; ADR-229): each lockup must be
//!     24 px tall, filled in the ink role of the page's theme, drawn whole inside every box that
//!     clips it and the window, and kept a quarter of its height (the brand's clear space) from
//!     the window's sides and every icon and text in its header. From 720 px no menu button is
//!     drawn and the sidebar is; under 720 px the menu button is. Wherever the sidebar is drawn,
//!     a screen reader and the keyboard reach it (not `aria-hidden`, every link in the tab
//!     order), and wherever it is put away neither does. Read again with scripts off (the root
//!     without `js`, mdBook's sidebar box unticked, transitions held): from 720 px the sidebar is
//!     drawn and no menu button is, and under 720 px the menu button is. And last, a window under
//!     720 px is widened to 800 px: once the sidebar's slide has run, it must be drawn and
//!     reached, with no menu button. A page with the site's header and no sidebar or menu button
//!     fails. That every page has the header, its lockup, the sidebar and the menu button is the
//!     file check's (`frame.rs`);
//!   - **a title block off the system's or not last**: each must sit inside a 2 px solid border
//!     in the ink role on every side, its fields' names and entries in Cascadia Mono, with no
//!     text, picture or field drawn below its top outside it, other than in a box fixed to the
//!     window. That every page ends with one is the file check's (`frame.rs`).
//! - **A page that prints off the system's** (#383; `web.md`, *Print*), read once a run, at its
//!   checked width nearest [`PRINT_PX`], with each media list that names `print` or `screen` set
//!   to how it applies on paper (a headless browser can't print from a page, so the print rules
//!   are read, not a printed page), in the theme the page is in and, on mdBook's pages that
//!   switch, the other:
//!   - a color off the light theme's roles, or a shape or contrast off the system's, as on
//!     screen; and on a page that switches, an element drawn in other colors from the other theme;
//!   - navigation or a control drawn, or anything inside one: a `nav`, a button, a text field,
//!     a menu or a search;
//!   - a link drawn out of the page (not to a place on it), in `main` or outside it as the title
//!     block's are, with no `::before` or `::after` holding its address;
//!   - a table row, a note or a title block that can split across sheets (`break-inside`);
//!   - a title block not drawn, or without its version and date of issue drawn;
//!   - anything fixed to the window, element or `::before` or `::after`, which a browser draws
//!     on every sheet over what stands there;
//!   - on the print page, whose chapters wait in a template (`print.rs`), the chapters not all
//!     placed in the page when it is about to print (`beforeprint`), or not all gone once
//!     printing ends (`afterprint`).
//! - What a keyboard, forced colors and a sticky bar need (#383; `web.md`, *Theme* and
//!   *Accessibility*), each read in the theme the page loads in, contrast in every reading:
//!   - **text under the contrast floor**: WCAG 2.2 AA (1.4.3), the floor `foundations.md`
//!     (*Contrast*) sets everywhere: 4.5 : 1 for text, 3 : 1 at 24 px or 18.66 px bold, each
//!     element's own text and a `::before` or `::after`'s against the background it is drawn on,
//!     faded by any opacity between them;
//!   - **a focus ring off the system's**: one element of each kind the keyboard reaches (as its
//!     short path names it), focused as by the keyboard, must draw a 2 px solid outline in the
//!     action role, 2 px off; a control hidden for its picture draws it on the element after it;
//!   - **something forced colors would erase**: with the stylesheets' `forced-colors: active`
//!     rules applied, a block set apart from what is behind it only by its fill (table parts
//!     aside: their cells have rules) must have a border or an outline, an inline icon must be
//!     drawn in its text color, and one painted through a mask must keep its own colors;
//!   - **a sticky bar that can hide what is focused**: a bar fixed or stuck to the window's top,
//!     at least half its width and under half its height, reaching further down than the root's
//!     `scroll-padding-top` (2.4.11);
//!   - and, as motion, a smooth scroll a script asks for when the menu bar's title is clicked, as
//!     mdBook's does.
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
//! that does, which must pass, text at 15 px, on mdBook's 23.2 px line, in a serif, without
//! tabular figures, at weight 300, a sentence in capitals, an `::after` at 11 px, a superscript
//! at 11 px, a chapter link in Archivo, a current page with no underline and one underlined 1 px,
//! a line of 69 characters after a short one and one of 68, which must pass, mdBook's 20 px
//! gutters, a right gutter wider than the left, a code block in a list item held to the measure,
//! four notes drawn as the system draws them, which must pass, a quote block, a caution whose
//! strip says Note, a signal word in body type, a strip with no rule or fill, a warning and a
//! caution without their fills, a warning bordered 1 px, a note with no left or right border and
//! one with a dashed top or bottom, a body before its head, a strip below its message,
//! Archivo with no fallback second, a label in faint ink, a paragraph faded to half and a title
//! faded the same, which must pass, links with no ring and with the browser's own, a hidden
//! checkbox whose picture takes the browser's ring, a box set apart by its fill alone and one with
//! a border in forced colors, which must pass, an icon in a fill of its own, one painted through
//! a mask and one that keeps its colors, which must pass, a bar fixed to the top without scroll
//! padding and one with it, which must pass, a title that scrolls smoothly when clicked, an icon
//! at 13 px, one 24 by 20 px, icons at 16, 20 and 24 px and one at 13 px not drawn, which must
//! pass, an icon at 0 px, selected text with no color of the page's, on the danger role at 22 % and
//! on the action role opaque, a page's frame drawn as the system's, which must pass, a lockup
//! 20 px tall, one in the action role, one cut by its box, one with the product's name inside
//! its clear space, a menu button only from 720 px, a sidebar drawn and hidden from screen
//! readers, one drawn with its link out of the tab order, one put away and still read, one put
//! away from 720 px and hidden while drawn under it, a sidebar and menu button put away with
//! scripts off, a sidebar drawn only when the window was wide at load (and a lockup 20 px tall
//! from 720 px), a title block bordered 1 px, one bordered
//! in a rule's color, one with its entries in Archivo, one with a paragraph below it, a mark as
//! text outside `main`, one as a heading's `::before` and one only before a paragraph jumped to,
//! marks as words in prose and a key's legend, which must pass, a page printed in navy's colors
//! from navy and one printed in light's, which must pass, a link printed in a blue of its own, a
//! screen-only rule that leaves a color off the roles on paper, navigation and a control drawn
//! on paper, a page arrow fixed inside a `nav` with no height, a link printed without its
//! address in `main` and one outside it, links with theirs, which must pass, a row that can
//! split, a rule fixed to the window's top on paper, and a title block hidden on paper, printed
//! without its date and able to split; an
//! intro and sheets drawn as the system's, which must pass, sheets under a 1 px rule and under
//! a rule's color, numbered out of order, a section's name outside a sheet, a sheet's number in
//! Archivo and one below its name, and a designation tag with no chamfer, below its title, at
//! 14 px and in a dashed border; a theme switch drawn and working as the system's, which must
//! pass, one offering Navy for Dark, one with Light pressed for a first-time reader, one beside
//! mdBook's theme button drawn, one that sets no theme when pressed, one that keeps a saved Coal,
//! one placed the other way round, one in its places with buttons 32 px tall, also read in the
//! sidebar opened under 720 px, and, in the header at every width, one with buttons 32 px tall,
//! one in Archivo, one
//! with its pressed button unfilled and one drawn with scripts off; and an ordinary page
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
/// The width a page is read on paper at, in CSS pixels: about the printable width of an A4 sheet
/// (718 px) and a Letter one (739 px) inside Chrome's default margins of 0.4 in. Each run reads
/// print at its checked width nearest this ([`print_width`]).
const PRINT_PX: u32 = 720;

/// The window's height, in CSS pixels. The whole page is measured, not only what this shows.
const HEIGHT_PX: u32 = 900;
/// Frames each tab loads at once. Pages of one tab share its main thread, so more frames mostly
/// overlap the waits on the network and fonts.
const FRAMES_PER_TAB: usize = 3;
/// The most tabs, each its own browser process; fewer on a machine with fewer cores. With four,
/// the check took 114 s on an M-series Mac on 2026-10-09, with five readings of each page's style
/// at each width, 160 s once it read each page's frame too, and 191 s with what a keyboard and
/// forced colors need (#432).
const MOST_TABS: usize = 4;
/// How long the check waits for any news from the browser before it gives up.
const STALL: Duration = Duration::from_secs(300);
/// How long the whole check may take.
const DEADLINE: Duration = Duration::from_secs(30 * 60);
/// The environment variable that names the browser's program.
const CHROME_ENV: &str = "HPR_CHROME";
/// Where the check's own files are served, beside the site.
pub(super) const CHECK_PREFIX: &str = "__check/";
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

/// The width of `checked` nearest [`PRINT_PX`], the narrower of two as near; 0 for none.
fn print_width(checked: &[u32]) -> u32 {
    checked
        .iter()
        .copied()
        .min_by_key(|width| (width.abs_diff(PRINT_PX), *width))
        .unwrap_or(0)
}

/// Which of [`widths`] one run checks: part `index` of `count`, so CI can spread the check over
/// several machines. The parts interleave (part 1 of 3 takes the 1st, 4th, 7th width and so on),
/// so each runs from the narrowest widths to the widest and the parts take about as long; every
/// width is in exactly one part. Each part's canaries are checked at its own widths, and a canary
/// must show its problem at every width, so each part proves on its own that it can fail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Part {
    /// Which part, from 1.
    pub index: usize,
    /// How many parts.
    pub count: usize,
}

impl Part {
    /// Every width, in one run.
    pub const WHOLE: Part = Part { index: 1, count: 1 };

    /// Reads `K/N`: part K of N, with 1 ≤ K ≤ N and each part holding at least one width.
    pub fn parse(text: &str) -> Result<Part, String> {
        let most = widths().len();
        let bad =
            || format!("`{text}` isn't a part K/N of the page check, with 1 ≤ K ≤ N ≤ {most}");
        let (index, count) = text.split_once('/').ok_or_else(bad)?;
        let index: usize = index.parse().map_err(|_| bad())?;
        let count: usize = count.parse().map_err(|_| bad())?;
        if index == 0 || index > count || count > most {
            return Err(bad());
        }
        Ok(Part { index, count })
    }

    /// The step between this part's widths, in CSS pixels.
    pub fn step_px(self) -> u32 {
        // At most 41 parts (`parse`), so the cast keeps the value.
        STEP_PX * self.count as u32
    }

    /// The widths this part checks, in order.
    pub fn widths(self) -> Vec<u32> {
        widths()
            .into_iter()
            .enumerate()
            .filter(|(at, _)| at % self.count == self.index - 1)
            .map(|(_, width)| width)
            .collect()
    }
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
    /// Motion off the system's durations and easings, any with reduced motion set, or a smooth
    /// scroll a script asks for (#383).
    Motion,
    /// Text off the system's type scale, Archivo without tabular figures, a family without its
    /// metric-matched fallback second, or navigation not in Cascadia Mono 14 px with the current
    /// page underlined 2 px (#384).
    Type,
    /// A line of prose longer than the system's 68 characters (#384).
    Measure,
    /// Gutters other than the system's: 16 px on a phone, 32 px from 720 px (#384).
    Gutter,
    /// A code block or table that scrolls sideways inside a box held to the prose measure.
    Squeezed,
    /// A quote block in a page's `main`, or a note not drawn as the system's: its kind's signal
    /// word in a `label` strip across its top, set apart by a rule or a fill, inside its border
    /// (#384).
    Note,
    /// Text under WCAG 2.2 AA's contrast against what it is drawn on (#383).
    Contrast,
    /// An element the keyboard reaches without the system's 2 px action-colored focus ring, 2 px
    /// off (#383).
    Focus,
    /// Something forced colors would erase: a box set apart only by its fill, an icon not in its
    /// text color, or one painted through a mask with its colors forced (#383).
    Forced,
    /// A bar stuck to the top of the window taller than the page's `scroll-padding-top` (#383).
    Sticky,
    /// An icon of the page's chrome drawn at a size other than the system's 16, 20 or 24 px,
    /// square (#383).
    Icon,
    /// Selected text drawn on something other than the action role at 22 % (#383).
    Selection,
    /// A header off the system's: a lockup not 24 px tall in the ink role, cut, or without its
    /// clear space; a menu button from 720 px or none under it; a sidebar put away from 720 px,
    /// or one a screen reader or the keyboard reaches while put away, or can't while drawn (#384).
    Header,
    /// A title block off the system's: not inside a 2 px ink border, its entries not in Cascadia
    /// Mono, or something drawn below it (#384).
    TitleBlock,
    /// A text of marks alone drawn as an icon: in the page's chrome, outside its `main`, or as a
    /// `::before` or `::after` anywhere (#383).
    Mark,
    /// A page that prints off the system's: a color other than the light theme's, navigation or
    /// a control drawn, a link without its address, a row, note or title block that can split,
    /// or a title block without its version and date (#383).
    Print,
    /// An intro or a sheet off the system's: a designation tag not in Cascadia Mono 12, not
    /// chamfered, not inside a 1 px border or not above its title; a section's name outside a
    /// sheet's rail; a sheet without its 2 px ink rule on top, or with its number not drawn,
    /// not `SHEET n / N` in order, not in Cascadia Mono 12 or not above its name (#384).
    Sheet,
    /// A theme switch off the system's: not in the header from 960 px or in the sidebar under
    /// it; its buttons not Auto, Light and Dark, under 44 px tall or not in Cascadia Mono 14;
    /// a first-time reader's pressed button not Auto alone, or a fill not the ink exactly where
    /// pressed; mdBook's theme button drawn; drawn with scripts off; the page in a theme the
    /// switch doesn't offer; pressing Dark, then Auto, not setting the theme; or a theme saved
    /// that it doesn't offer, Coal, not forgotten for Auto (#384, #429).
    Switch,
}

impl Kind {
    const ALL: [Kind; 24] = [
        Kind::Wide,
        Kind::Cut,
        Kind::Overlap,
        Kind::Shrunk,
        Kind::Color,
        Kind::Shape,
        Kind::Motion,
        Kind::Type,
        Kind::Measure,
        Kind::Gutter,
        Kind::Squeezed,
        Kind::Note,
        Kind::Contrast,
        Kind::Focus,
        Kind::Forced,
        Kind::Sticky,
        Kind::Icon,
        Kind::Selection,
        Kind::Header,
        Kind::TitleBlock,
        Kind::Mark,
        Kind::Print,
        Kind::Sheet,
        Kind::Switch,
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
            Kind::Type => "type off the system's scale",
            Kind::Measure => "a line of prose past 68 characters",
            Kind::Gutter => "gutters off the system's",
            Kind::Squeezed => "a code block or table held to the prose measure",
            Kind::Note => "a quote block, or a note not the system's",
            Kind::Contrast => "text under the contrast floor",
            Kind::Focus => "a focus ring off the system's",
            Kind::Forced => "something forced colors would erase",
            Kind::Sticky => "a sticky bar that can hide what is focused",
            Kind::Icon => "an icon off the system's sizes",
            Kind::Selection => "a selection off the action role",
            Kind::Header => "a header off the system's",
            Kind::TitleBlock => "a title block off the system's or not last",
            Kind::Mark => "a text mark drawn as an icon",
            Kind::Print => "a page that prints off the system's",
            Kind::Sheet => "an intro or a sheet off the system's",
            Kind::Switch => "a theme switch off the system's",
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

/// The system's note styles (`fusionspace.css`, `.fs-note`), for the note canaries, which load
/// the ordinary canary page's styles, not the site's.
macro_rules! note_style {
    () => {
        "<style>.fs-note { border: 1px solid #566079; background: #FFFFFF; margin: 16px 0; } \
.fs-note-head { display: flex; gap: 8px; padding: 4px 12px; font: 600 12px/16px \
'Cascadia Mono', 'Cascadia Mono Fallback', monospace; text-transform: uppercase; \
border-bottom: 1px solid #566079; color: #0B0F1C; } .fs-note-head svg { width: 14px; \
height: 14px; } .fs-note-body { padding: 12px; } .fs-note-body > * { margin: 0; } \
.fs-note[data-kind=\"caution\"] { border-color: #F5AF20; } .fs-note[data-kind=\"caution\"] > \
.fs-note-head { background: #F5AF20; border-color: #F5AF20; } .fs-note[data-kind=\"warning\"] \
{ border: 2px solid #AC001E; } .fs-note[data-kind=\"warning\"] > .fs-note-head { background: \
#AC001E; color: #FFFFFF; border-color: #AC001E; } @media print { .fs-note { break-inside: \
avoid; } }</style>"
    };
}

/// A note of `kind` with the signal word `word` and a short message, as `xtask site` draws one
/// (`notes.rs`), without its icon.
macro_rules! note {
    ($kind:literal, $word:literal) => {
        concat!(
            "<aside class=\"fs-note\" data-kind=\"",
            $kind,
            "\"><div class=\"fs-note-head\">",
            $word,
            "</div><div class=\"fs-note-body\"><p>A message.</p></div></aside>"
        )
    };
}

/// An icon on the system's grid, drawn `$width` by `$height`, for the icon canaries.
macro_rules! icon {
    ($width:literal, $height:literal) => {
        concat!(
            "<svg viewBox=\"0 0 24 24\" style=\"width: ",
            $width,
            "; height: ",
            $height,
            "\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" \
             aria-hidden=\"true\"><path d=\"M9 5 L16 12 L9 19\"/></svg>"
        )
    };
}

/// The note canaries' pages.
const NOTES_OK: &str = concat!(
    note_style!(),
    note!("trust", "How far to trust it"),
    note!("note", "Note"),
    note!("caution", "Caution"),
    note!("warning", "Warning")
);
const NOTES_WRONG_WORD: &str = concat!(note_style!(), note!("caution", "Note"));
// Each failing note canary breaks one property, so each of the check's rules has a canary of
// its own.
const NOTES_TYPE: &str = concat!(
    note_style!(),
    "<style>.fs-note-head { font: 16px/24px 'Archivo', 'Archivo Fallback', sans-serif !important; \
     font-variant-numeric: tabular-nums !important; text-transform: none !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_NO_RULE: &str = concat!(
    note_style!(),
    "<style>.fs-note-head { border-bottom: 0 !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_WARNING_UNFILLED: &str = concat!(
    note_style!(),
    "<style>.fs-note[data-kind=\"warning\"] > .fs-note-head { background: none !important; color: \
     #0B0F1C !important; }</style>",
    note!("warning", "Warning")
);
const NOTES_CAUTION_UNFILLED: &str = concat!(
    note_style!(),
    "<style>.fs-note[data-kind=\"caution\"] > .fs-note-head { background: none !important; \
     }</style>",
    note!("caution", "Caution")
);
const NOTES_WARNING_THIN: &str = concat!(
    note_style!(),
    "<style>.fs-note[data-kind=\"warning\"] { border-width: 1px !important; }</style>",
    note!("warning", "Warning")
);
// One side of a note's border each: no width on the left or right, dashed on the top or bottom.
const NOTES_NO_LEFT: &str = concat!(
    note_style!(),
    "<style>.fs-note { border-left-width: 0 !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_NO_RIGHT: &str = concat!(
    note_style!(),
    "<style>.fs-note { border-right-width: 0 !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_DASHED_TOP: &str = concat!(
    note_style!(),
    "<style>.fs-note { border-top-style: dashed !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_DASHED_BOTTOM: &str = concat!(
    note_style!(),
    "<style>.fs-note { border-bottom-style: dashed !important; }</style>",
    note!("trust", "How far to trust it")
);
const NOTES_BODY_FIRST: &str = concat!(
    note_style!(),
    "<aside class=\"fs-note\" data-kind=\"trust\"><div class=\"fs-note-body\"><p>A \
     message.</p></div><div class=\"fs-note-head\">How far to trust it</div></aside>"
);
const NOTES_BELOW: &str = concat!(
    note_style!(),
    "<style>.fs-note { display: flex; flex-direction: column-reverse; }</style>",
    note!("trust", "How far to trust it")
);

/// The system's frame (`web.md`, *Page anatomy*), for the frame canaries, which load the
/// ordinary canary page's styles, not the site's: a header holding a menu button under 720 px
/// and the lockup with the product's name, a sidebar drawn from 720 px and reached only there,
/// then the title block, last.
macro_rules! frame_style {
    () => {
        "<style>.frame-header { display: flex; align-items: center; gap: 16px; padding: 13px \
16px; } .fs-logo { display: flex; align-items: center; gap: 12px; color: inherit; \
text-decoration: none; } .fs-lockup { height: 24px; width: auto; flex: none; } .fs-product { \
font: 600 20px/24px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; } \
#mdbook-sidebar-toggle { display: inline-flex; } #mdbook-sidebar-toggle svg { width: 20px; \
height: 20px; } #mdbook-sidebar { display: none; padding: 0 16px; font: 14px/20px 'Cascadia \
Mono', 'Cascadia Mono Fallback', monospace; } @media (min-width: 720px) { \
#mdbook-sidebar-toggle { display: none; } #mdbook-sidebar { display: block; } } .fs-titleblock \
{ display: grid; grid-template-columns: repeat(auto-fit, minmax(132px, 1fr)); border: 2px solid \
#0B0F1C; background: #FFFFFF; margin: 48px 16px 0; } .fs-titleblock > div { padding: 6px 10px \
8px; display: grid; gap: 2px; min-width: 0; } .fs-titleblock .k { font: 12px/16px 'Cascadia \
Mono', 'Cascadia Mono Fallback', monospace; text-transform: uppercase; color: #566079; } \
.fs-titleblock .v { font: 14px/20px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; \
overflow-wrap: anywhere; } @media print { html #mdbook-sidebar, html #mdbook-sidebar-toggle { \
display: none !important; } .fs-titleblock { break-inside: avoid; } }</style>"
    };
}

/// A lockup as `frame.rs` draws one, a single path in the lockup's view box.
macro_rules! lockup {
    () => {
        "<svg class=\"fs-lockup\" viewBox=\"0 0 680.485 115\" fill=\"currentColor\" \
         aria-hidden=\"true\"><path \
         fill=\"currentColor\" d=\"M0 0H680V115H0Z\"/></svg>"
    };
}

/// The sidebar as the site's script leaves it: hidden from screen readers and the keyboard, and
/// reached from 720 px, where it is drawn, as the window's width changes too.
macro_rules! sidebar {
    () => {
        "<nav id=\"mdbook-sidebar\" aria-hidden=\"true\"><a href=\"#\" tabindex=\"-1\">A \
         chapter</a></nav><script>(function () { const m = matchMedia('(min-width: 720px)'); \
         const s = document.getElementById('mdbook-sidebar'); const set = function () { \
         s.setAttribute('aria-hidden', String(!m.matches)); s.querySelector('a').tabIndex = \
         m.matches ? 0 : -1; }; set(); m.addEventListener('change', set); })();</script>"
    };
}

/// A page's frame with `$style` added, `$lockup` in its header and `$sidebar` after it, then the
/// title block and `$after`; its root marked `js`, as the site's script marks it, and mdBook's
/// sidebar box, for the reading with scripts off.
macro_rules! frame {
    ($style:expr, $lockup:expr, $sidebar:expr, $after:expr) => {
        concat!(
            frame_style!(),
            $style,
            "<input type=\"checkbox\" id=\"mdbook-sidebar-toggle-anchor\" checked \
             style=\"display: none\"><script>document.documentElement.classList.add('js');\
             </script><header class=\"frame-header\"><label id=\"mdbook-sidebar-toggle\" \
             aria-label=\"Menu\"><svg viewBox=\"0 0 24 24\" fill=\"none\" \
             stroke=\"currentColor\" stroke-width=\"1.5\" aria-hidden=\"true\"><path d=\"M4 7H20M4 \
             12H20M4 17H20\"/></svg></label><a class=\"fs-logo\" href=\"#\" \
             aria-label=\"FusionSpace HPR\">",
            $lockup,
            "<span class=\"fs-product\">HPR</span></a></header>",
            $sidebar,
            "<footer class=\"fs-titleblock\" aria-label=\"Title block\"><div><span \
             class=\"k\">Owner</span><span class=\"v\">FusionSpace</span></div><div><span \
             class=\"k\">Title</span><span class=\"v\">A canary</span></div><div><span \
             class=\"k\">Version</span><span class=\"v\">0.1.0</span></div><div><span \
             class=\"k\">Date of issue</span><span class=\"v\">2026-10-10</span></div></footer>",
            $after
        )
    };
}

/// What a canary needs to be read in mdBook's navy theme too: the root marked `js` and in the light
/// theme, mdBook's code stylesheet for navy (empty), and, on screen only, the navy theme's dark
/// color scheme, its links, its search hit and its selection.
macro_rules! switches {
    () => {
        "<link rel=\"stylesheet\" id=\"mdbook-tomorrow-night-css\" href=\"data:text/css,\">\
         <style>@media screen { .navy { color-scheme: dark; } .navy a { color: #768DF5; } \
         .navy mark { color: #0B0F1C; background: #768DF5; } .navy ::selection { \
         background-color: rgb(118 141 245 / 0.22); } }</style>\
         <script>document.documentElement.classList.add('js', 'light');</script>"
    };
}

/// The system's intro and sheets (`fusionspace.css`, `.fs-tag` and `.fs-sheet`) as
/// `theme/hpr.css` draws them, for the sheet canaries: a chapter's intro, the designation tag
/// over its title, then two sheets, numbered `$first` and `$second`, after `$style`.
macro_rules! sheets {
    ($style:expr, $first:expr, $second:expr) => {
        concat!(
            "<style>.fs-tag { --c: 4px; display: flex; width: fit-content; margin: 0 0 16px; \
             padding: 2px 10px 2px 8px; min-height: 24px; font: 400 12px/16px 'Cascadia Mono', \
             'Cascadia Mono Fallback', monospace; color: #0B0F1C; border: 1px solid #566079; \
             clip-path: polygon(0 0, calc(100% - var(--c)) 0, 100% var(--c), 100% 100%, 0 \
             100%); } .fs-intro > h1 { margin: 0; } .fs-sheet { border-top: 2px solid #0B0F1C; \
             margin-top: 32px; padding-top: 16px; } .fs-sheet-rail { display: grid; gap: 8px; \
             margin-bottom: 16px; } .fs-sheet-rail > h2 { margin: 0; font: 600 20px/24px \
             'Cascadia Mono', 'Cascadia Mono Fallback', monospace; } .fs-sheet-no { font: 400 \
             12px/16px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; letter-spacing: \
             0.06em; color: #566079; }</style>",
            $style,
            "<div class=\"fs-intro\"><p class=\"fs-tag\">FS-ACHERNAR · SW · TOOL 001</p><h1>A \
             chapter</h1><p>Its opening.</p></div><section class=\"fs-sheet\"><div \
             class=\"fs-sheet-rail\"><span class=\"fs-sheet-no\">",
            $first,
            "</span><h2>The first sheet</h2></div><div class=\"fs-sheet-body\"><p>Its \
             text.</p></div></section><section class=\"fs-sheet\"><div \
             class=\"fs-sheet-rail\"><span class=\"fs-sheet-no\">",
            $second,
            "</span><h2>The second sheet</h2></div><div class=\"fs-sheet-body\"><p>Its \
             text.</p></div></section>"
        )
    };
}

/// The theme switch's buttons as `switch.rs` writes them, Auto pressed, as `theme/hpr.js` marks
/// it for a first-time reader.
macro_rules! switch_buttons {
    () => {
        "<li role=\"none\"><button type=\"button\" class=\"theme\" \
         aria-pressed=\"true\">Auto</button></li><li role=\"none\"><button type=\"button\" \
         class=\"theme\" aria-pressed=\"false\">Light</button></li><li role=\"none\"><button \
         type=\"button\" class=\"theme\" aria-pressed=\"false\">Dark</button></li>"
    };
}

/// Where a switch canary's script puts the switch: as `theme/hpr.js` does, in the header from
/// 960 px and at the top of the sidebar under it; in the header at every width; or the other
/// way round.
macro_rules! switch_placed {
    () => {
        "<script>(function () { const g = document.getElementById('mdbook-theme-list'); const m = \
         matchMedia('(min-width: 960px)'); const place = function () { if (m.matches) \
         document.querySelector('header').appendChild(g); else \
         document.getElementById('mdbook-sidebar').prepend(g); }; place(); \
         m.addEventListener('change', place); })();</script>"
    };
}
macro_rules! switch_in_header {
    () => {
        "<script>document.querySelector('header').appendChild(\
         document.getElementById('mdbook-theme-list'));</script>"
    };
}
macro_rules! switch_placed_inverted {
    () => {
        "<script>(function () { const g = document.getElementById('mdbook-theme-list'); const m = \
         matchMedia('(min-width: 960px)'); const place = function () { if (m.matches) \
         document.getElementById('mdbook-sidebar').prepend(g); else \
         document.querySelector('header').appendChild(g); }; place(); \
         m.addEventListener('change', place); })();</script>"
    };
}

/// A switch canary's script that works it as mdBook's does with `theme/hpr.js`: a theme saved
/// that the switch doesn't offer is forgotten, the root in the one the system asks for; then
/// `switch_clicks!`.
macro_rules! switch_works {
    () => {
        concat!(
            "<script>(function () { let s = null; try { s = \
             localStorage.getItem('mdbook-theme'); } catch (e) { } if (s === null || s === \
             'light' || s === 'navy') return; localStorage.removeItem('mdbook-theme'); const r \
             = document.documentElement; r.classList.remove('light', 'navy'); \
             r.classList.add(matchMedia('(prefers-color-scheme: dark)').matches ? 'navy' : \
             'light'); })();</script>",
            switch_clicks!()
        )
    };
}

/// A switch canary's script that puts the root in a saved theme, as mdBook's does before its
/// own script runs, and doesn't forget one the switch doesn't offer; then `switch_clicks!`.
macro_rules! switch_keeps {
    () => {
        concat!(
            "<script>try { const s = localStorage.getItem('mdbook-theme'); if (s) \
             document.documentElement.classList.add(s); } catch (e) { }</script>",
            switch_clicks!()
        )
    };
}

/// A switch canary's script that presses as mdBook's does: a button pressed puts the root in its
/// theme (Auto in the one the system asks for) and is the one pressed.
macro_rules! switch_clicks {
    () => {
        "<script>document.getElementById('mdbook-theme-list').addEventListener('click', function \
         (e) { const b = e.target.closest('button'); if (!b) return; const r = \
         document.documentElement; const n = b.textContent.trim(); const t = n === 'Dark' ? \
         'navy' : n === 'Light' ? 'light' : matchMedia('(prefers-color-scheme: dark)').matches \
         ? 'navy' : 'light'; r.classList.remove('light', 'navy'); r.classList.add(t); \
         this.querySelectorAll('button').forEach(function (x) { x.setAttribute('aria-pressed', \
         String(x === b)); }); });</script>"
    };
}

/// A page's frame (as [`frame!`] draws it) with the theme switch (`fusionspace.css`, `.fs-seg`,
/// as `theme/hpr.css` draws it) after its sidebar: `$style` added, `$buttons` in the switch,
/// then `$place` placing it and `$work` working it. The header wraps, as the site's does on a
/// phone, so a switch in it at every width fits, and the sidebar opens under 720 px as mdBook's
/// does, the root marked `sidebar-visible`.
macro_rules! switch_frame {
    ($style:expr, $buttons:expr, $place:expr, $work:expr) => {
        frame!(
            concat!(
                "<style>.frame-header { flex-wrap: wrap; } html.sidebar-visible \
                 #mdbook-sidebar { display: block; } #mdbook-theme-list { display: \
                 inline-flex; margin: 0; padding: 0; list-style: none; border: 1px solid \
                 #566079; } #mdbook-theme-list li { display: flex; } #mdbook-theme-list button { \
                 min-height: 44px; padding: 0 12px; font: 400 14px/20px 'Cascadia Mono', \
                 'Cascadia Mono Fallback', monospace; color: #566079; background: transparent; \
                 border: 0; border-left: 1px solid #566079; } #mdbook-theme-list \
                 li:first-child button { border-left: 0; } #mdbook-theme-list \
                 [aria-pressed=\"true\"] { color: #F3F4F7; background: #0B0F1C; } \
                 html:not(.js) #mdbook-theme-list { display: none; } @media (forced-colors: \
                 active) { #mdbook-theme-list [aria-pressed=\"true\"] { outline: 2px solid \
                 CanvasText; outline-offset: -4px; } } @media print { #mdbook-theme-list { \
                 display: none; } }</style>",
                $style
            ),
            lockup!(),
            concat!(
                sidebar!(),
                "<ul id=\"mdbook-theme-list\" class=\"theme-popup fs-seg\" role=\"group\" \
                 aria-label=\"Theme\">",
                $buttons,
                "</ul>",
                $place,
                $work
            ),
            ""
        )
    };
}

/// The canaries ([`Canary`]), loaded on every run before the site's pages.
const CANARIES: [Canary; 135] = [
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
        expect: &[Kind::Color, Kind::Print],
        inside: "<p style=\"color: #008200\">A label in green</p>",
        after: "",
    },
    Canary {
        file: "canary-radius.html",
        what: "a box with rounded corners on screen, square on paper",
        expect: &[Kind::Shape],
        inside: "<style>@media screen { .rounded { border-radius: 8px; } }</style><p \
                 class=\"rounded\" style=\"border: 1px solid #566079\">A rounded box</p>",
        after: "",
    },
    Canary {
        file: "canary-shadow.html",
        what: "a box with a soft shadow",
        expect: &[Kind::Shape, Kind::Print],
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
        expect: &[Kind::Color, Kind::Print],
        inside: "<style>html { background: #008200; }</style>",
        after: "",
    },
    Canary {
        file: "canary-filter.html",
        what: "a label tinted by a filter, as mdBook tints its copy icon, whose colors the check \
               can't read",
        expect: &[Kind::Color, Kind::Print],
        inside: "<p style=\"filter: invert(45%)\">A tinted label</p>",
        after: "",
    },
    Canary {
        file: "canary-backdrop.html",
        what: "a label over a blurred backdrop, a filter whose colors the check can't read",
        expect: &[Kind::Color, Kind::Print],
        inside: "<p style=\"backdrop-filter: blur(2px)\">A label over a blur</p>",
        after: "",
    },
    Canary {
        file: "canary-content-image.html",
        what: "an image drawn as a label's `::before`, whose colors the check can't read",
        expect: &[Kind::Color, Kind::Print],
        inside: "<style>.icon::before { content: url(\"data:image/svg+xml,%3Csvg \
                 xmlns='http://www.w3.org/2000/svg' width='16' height='16'%3E%3C/svg%3E\"); }\
                 </style><p class=\"icon\">A label with an icon</p>",
        after: "",
    },
    Canary {
        file: "canary-mark.html",
        what: "a search hit filled in ink, a role, but not the action fill a selection takes",
        expect: &[Kind::Color, Kind::Print],
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
                 @media (prefers-color-scheme: dark) { html:not(.js) .late { color: #566079; } }\
                 </style>\
                 <p class=\"late\">Green after a delay</p>",
        after: "",
    },
    Canary {
        file: "canary-navy.html",
        what: "a label in a highlighter's green in the navy theme only, read after the switch \
               mdBook's theme menu makes",
        expect: &[Kind::Color, Kind::Print],
        inside: "<script>document.documentElement.classList.add('js', 'light');</script>\
                 <link rel=\"stylesheet\" id=\"mdbook-tomorrow-night-css\" href=\"data:text/css,\">\
                 <style>.navy { color-scheme: dark; } .navy mark { color: #0B0F1C; background: #768DF5; } \
                 .navy ::selection { background-color: rgb(118 141 245 / 0.22); } \
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
    Canary {
        file: "canary-type-size.html",
        what: "a label at 15 px, between the system's sizes",
        expect: &[Kind::Type],
        inside: "<p style=\"font-size: 15px\">A label at 15 px</p>",
        after: "",
    },
    Canary {
        file: "canary-type-line.html",
        what: "a paragraph at 16 px on mdBook's 1.45 em line, 23.2 px, not the system's 24",
        expect: &[Kind::Type],
        inside: "<p style=\"line-height: 1.45em\">A paragraph on mdBook's line height</p>",
        after: "",
    },
    Canary {
        file: "canary-type-family.html",
        what: "a label in a serif, neither of the system's families",
        expect: &[Kind::Type],
        inside: "<p style=\"font-family: Georgia, serif\">A label in a serif</p>",
        after: "",
    },
    Canary {
        file: "canary-type-fallback.html",
        what: "a label in Archivo with no metric-matched fallback second in its stack",
        expect: &[Kind::Type],
        inside: "<p style=\"font-family: 'Archivo', sans-serif\">A label without its fallback</p>",
        after: "",
    },
    Canary {
        file: "canary-type-figures.html",
        what: "a number in Archivo without tabular figures",
        expect: &[Kind::Type],
        inside: "<p style=\"font-variant-numeric: normal\">Apogee 1,234 m</p>",
        after: "",
    },
    Canary {
        file: "canary-type-capitals.html",
        what: "a sentence of body text in capitals",
        expect: &[Kind::Type],
        inside: "<p style=\"text-transform: uppercase\">A sentence in capitals</p>",
        after: "",
    },
    Canary {
        file: "canary-type-weight.html",
        what: "a label at weight 300, lighter than the system's lightest",
        expect: &[Kind::Type],
        inside: "<p style=\"font-weight: 300\">A light label</p>",
        after: "",
    },
    Canary {
        file: "canary-type-pseudo.html",
        what: "text drawn as a label's `::after` at 11 px, below the system's smallest",
        expect: &[Kind::Type],
        inside: "<style>.tagged::after { content: \" new\"; font-size: 11px; }</style>\
                 <p class=\"tagged\">A label</p>",
        after: "",
    },
    Canary {
        file: "canary-nav-font.html",
        what: "a chapter link in Archivo, not the navigation's Cascadia Mono",
        expect: &[Kind::Type],
        inside: "<ol class=\"chapter\"><li><a href=\"#\" style=\"font: 14px/20px Archivo, \
                 sans-serif\">A chapter in Archivo</a></li></ol>",
        after: "",
    },
    Canary {
        file: "canary-nav-current.html",
        what: "the current page's link with no underline",
        expect: &[Kind::Type],
        inside: "<ol class=\"chapter\"><li><a class=\"active\" href=\"#\" style=\"font: 14px/20px \
                 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; text-decoration: none\">The current page</a></li></ol>",
        after: "",
    },
    Canary {
        file: "canary-nav-thin.html",
        what: "the current page's link underlined 1 px, not 2",
        expect: &[Kind::Type],
        inside: "<ol class=\"chapter\"><li><a class=\"active\" href=\"#\" style=\"font: 14px/20px \
                 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; text-decoration: underline 1px\">The current \
                 page</a></li></ol>",
        after: "",
    },
    Canary {
        file: "canary-type-sup.html",
        what: "a superscript at 11 px, below the system's smallest",
        expect: &[Kind::Type],
        inside: "<p>An area in m<sup style=\"font-size: 11px\">2</sup></p>",
        after: "",
    },
    Canary {
        file: "canary-measure.html",
        what: "a line of 69 characters, as drawn, after a short one, in a box that scrolls \
               sideways",
        expect: &[Kind::Measure],
        inside: "<div class=\"scroll\"><p style=\"white-space: nowrap; max-width: none\">A short \
                 line<br>A \
                 <strong>drag</strong>  coefficient of <code>0.45</code> at Mach 0.3\n on \
                 m<sup>2</sup> of area, and a long tail</p></div>",
        after: "",
    },
    Canary {
        file: "canary-measure-68.html",
        what: "a line of 68 characters, as drawn, after a short one, which must pass",
        expect: &[],
        inside: "<div class=\"scroll\"><p style=\"white-space: nowrap; max-width: none\">A short \
                 line<br>A \
                 <strong>drag</strong>  coefficient of <code>0.45</code> at Mach 0.3\n on \
                 m<sup>2</sup> of area and a long tail</p></div>",
        after: "",
    },
    Canary {
        file: "canary-notes.html",
        what: "a trust note, a note, a caution and a warning drawn as the system draws them, \
               which must pass",
        expect: &[],
        inside: NOTES_OK,
        after: "",
    },
    Canary {
        file: "canary-note-quote.html",
        what: "a quote block in `main`, not a note",
        expect: &[Kind::Note],
        inside: "<blockquote><p>A plain quote block.</p></blockquote>",
        after: "",
    },
    Canary {
        file: "canary-note-word.html",
        what: "a caution whose strip says Note",
        expect: &[Kind::Note],
        inside: NOTES_WRONG_WORD,
        after: "",
    },
    Canary {
        file: "canary-note-type.html",
        what: "a note whose signal word is set in body type, not the label token",
        expect: &[Kind::Note],
        inside: NOTES_TYPE,
        after: "",
    },
    Canary {
        file: "canary-note-no-rule.html",
        what: "a trust note whose strip has no rule under it, and no fill",
        expect: &[Kind::Note],
        inside: NOTES_NO_RULE,
        after: "",
    },
    Canary {
        file: "canary-note-warning-unfilled.html",
        what: "a warning whose strip has a rule but not the danger fill",
        expect: &[Kind::Note],
        inside: NOTES_WARNING_UNFILLED,
        after: "",
    },
    Canary {
        file: "canary-note-caution-unfilled.html",
        what: "a caution whose strip has a rule but not the caution fill",
        expect: &[Kind::Note],
        inside: NOTES_CAUTION_UNFILLED,
        after: "",
    },
    Canary {
        file: "canary-note-warning-thin.html",
        what: "a warning bordered 1 px, not 2 px",
        expect: &[Kind::Note],
        inside: NOTES_WARNING_THIN,
        after: "",
    },
    Canary {
        file: "canary-note-no-left.html",
        what: "a note with no border on its left",
        expect: &[Kind::Note],
        inside: NOTES_NO_LEFT,
        after: "",
    },
    Canary {
        file: "canary-note-no-right.html",
        what: "a note with no border on its right",
        expect: &[Kind::Note],
        inside: NOTES_NO_RIGHT,
        after: "",
    },
    Canary {
        file: "canary-note-dashed-top.html",
        what: "a note whose top border is dashed",
        expect: &[Kind::Note],
        inside: NOTES_DASHED_TOP,
        after: "",
    },
    Canary {
        file: "canary-note-dashed-bottom.html",
        what: "a note whose bottom border is dashed",
        expect: &[Kind::Note],
        inside: NOTES_DASHED_BOTTOM,
        after: "",
    },
    Canary {
        file: "canary-note-body-first.html",
        what: "a note whose body comes before its head",
        expect: &[Kind::Note],
        inside: NOTES_BODY_FIRST,
        after: "",
    },
    Canary {
        file: "canary-note-below.html",
        what: "a note whose strip sits below its message",
        expect: &[Kind::Note],
        inside: NOTES_BELOW,
        after: "",
    },
    Canary {
        file: "canary-squeezed.html",
        what: "a code block in a list item held to the prose measure, scrolling sideways",
        expect: &[Kind::Squeezed],
        inside: "<ul><li style=\"max-width: 10em\">An item<pre><code>let speed_m_per_s = \
                 drag_coefficient * reference_area_m2;</code></pre></li></ul>",
        after: "",
    },
    Canary {
        file: "canary-gutter-right.html",
        what: "a right gutter of 56 px, wider than the left, which is the system's",
        expect: &[Kind::Gutter],
        inside: "<style>.content { padding-right: 56px !important; }</style>",
        after: "",
    },
    Canary {
        file: "canary-gutter.html",
        what: "mdBook's 20 px gutters: wider than a phone's 16 px, narrower than 32 px from 720 px",
        expect: &[Kind::Gutter],
        inside: "<style>.content { padding: 0 20px !important; }</style>",
        after: "",
    },
    Canary {
        file: "canary-contrast.html",
        what: "a label in the system's faint ink on the canvas, at 2.35 : 1, on screen only",
        expect: &[Kind::Contrast],
        inside: "<style>@media screen { .faint { color: #98A1B8; } }</style><p \
                 class=\"faint\">A faint label</p>",
        after: "",
    },
    Canary {
        file: "canary-contrast-faded.html",
        what: "a paragraph of ink faded to half by its opacity, at 3.51 : 1",
        expect: &[Kind::Contrast, Kind::Print],
        inside: "<p style=\"opacity: 0.5\">A faded paragraph</p>",
        after: "",
    },
    Canary {
        file: "canary-contrast-large.html",
        what: "a 28 px title of ink faded to half, at 3.51 : 1, over large text's 3 : 1, which \
               must pass",
        expect: &[],
        inside: "<p style=\"font: 600 28px/32px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; opacity: 0.5\">A \
                 faded title</p>",
        after: "",
    },
    Canary {
        file: "canary-focus.html",
        what: "a link whose keyboard focus draws no ring",
        expect: &[Kind::Focus],
        inside: "<style>.bare:focus-visible { outline: none; }</style>\
                 <p><a class=\"bare\" href=\"#\">A link without a ring</a></p>",
        after: "",
    },
    Canary {
        file: "canary-focus-auto.html",
        what: "a link whose keyboard focus draws the browser's own ring, as mdBook leaves it",
        expect: &[Kind::Focus],
        inside: "<style>.own:focus-visible { outline: auto; }</style>\
                 <p><a class=\"own\" href=\"#\">A link with the browser's ring</a></p>",
        after: "",
    },
    Canary {
        file: "canary-focus-hidden.html",
        what: "a hidden checkbox whose ring, drawn on the picture after it, is the browser's own, \
               as mdBook's zoom draws it",
        expect: &[Kind::Focus],
        inside: "<style>.zoom:focus-visible + img { outline: auto; }</style>\
                 <p><label><input class=\"checkbox-img zoom\" type=\"checkbox\"><img \
                 src=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='40' \
                 height='20'%3E%3C/svg%3E\" alt=\"A small figure\"></label></p>",
        after: "",
    },
    Canary {
        file: "canary-forced-fill.html",
        what: "a box set apart from the canvas by its fill alone, which forced colors erase",
        expect: &[Kind::Forced],
        inside: "<div style=\"background: #FFFFFF; padding: 8px 0\"><p>A layer drawn by its \
                 fill</p></div>",
        after: "",
    },
    Canary {
        file: "canary-forced-border.html",
        what: "a box set apart by its fill, with a border in forced colors, which must pass",
        expect: &[],
        inside: "<style>@media (forced-colors: active) { .layer { border: 1px solid CanvasText; } }\
                 </style><div class=\"layer\" style=\"background: #FFFFFF; padding: 8px 0\"><p>A \
                 layer with a border</p></div>",
        after: "",
    },
    Canary {
        file: "canary-forced-icon.html",
        what: "an inline icon drawn in a fill of its own, not its text color",
        expect: &[Kind::Forced],
        inside: "<p>An icon <svg fill=\"currentColor\" width=\"16\" height=\"16\" viewBox=\"0 0 16 \
                 16\"><rect width=\"16\" height=\"16\" fill=\"#3350D6\"/></svg> in a fill of its \
                 own</p>",
        after: "",
    },
    Canary {
        file: "canary-forced-mask.html",
        what: "an icon painted through a mask in its text color, which forced colors repaint in \
               the canvas color",
        expect: &[Kind::Forced],
        inside: "<style>.masked::before { content: \"\"; display: inline-block; width: 16px; \
                 height: 16px; background-color: currentColor; \
                 -webkit-mask: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'/%3E\"); \
                 mask: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'/%3E\"); }\
                 </style><p class=\"masked\">A label with a masked icon</p>",
        after: "",
    },
    Canary {
        file: "canary-forced-mask-kept.html",
        what: "an icon painted through a mask that keeps its own colors in forced colors, which \
               must pass",
        expect: &[],
        inside: "<style>.masked::before { content: \"\"; display: inline-block; width: 16px; \
                 height: 16px; background-color: currentColor; \
                 -webkit-mask: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'/%3E\"); \
                 mask: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'/%3E\"); } \
                 @media (forced-colors: active) { .masked::before { forced-color-adjust: none; \
                 background-color: ButtonText; } }</style><p class=\"masked\">A label with a \
                 masked icon</p>",
        after: "",
    },
    Canary {
        file: "canary-sticky.html",
        what: "a 60 px bar fixed to the window's top with no `scroll-padding-top`",
        expect: &[Kind::Sticky],
        // Put away on paper, as the site's own bar is, so that only the sticky reading finds it.
        inside: "<style>@media print { .bar { display: none; } }</style>",
        after: "<div class=\"bar\" style=\"position: fixed; top: 0; left: 0; right: 0; \
                height: 60px; background: #F3F4F7\"></div>",
    },
    Canary {
        file: "canary-sticky-padded.html",
        what: "a 60 px bar fixed to the window's top over a 60 px `scroll-padding-top`, which \
               must pass",
        expect: &[],
        inside: "<style>html { scroll-padding-top: 60px; } @media print { .bar { display: none; \
                 } }</style>",
        after: "<div class=\"bar\" style=\"position: fixed; top: 0; left: 0; right: 0; \
                height: 60px; background: #F3F4F7\"></div>",
    },
    Canary {
        file: "canary-smooth.html",
        what: "a title whose click asks for a smooth scroll to the top, as mdBook's script does",
        expect: &[Kind::Motion],
        inside: "<p class=\"menu-title\">A title</p><script>document.querySelector('.menu-title')\
                 .addEventListener('click', function () { document.scrollingElement.scrollTo({ \
                 top: 0, behavior: 'smooth' }); });</script>",
        after: "",
    },
    Canary {
        file: "canary-icon-small.html",
        what: "an icon drawn at 13 px, as mdBook draws its menu bar's glyphs",
        expect: &[Kind::Icon],
        inside: concat!(
            "<p><span class=fa-svg>",
            icon!("13px", "13px"),
            "</span></p>"
        ),
        after: "",
    },
    Canary {
        file: "canary-icon-oblong.html",
        what: "an icon 24 px wide and 20 px high",
        expect: &[Kind::Icon],
        inside: concat!(
            "<p><span class=fa-svg>",
            icon!("24px", "20px"),
            "</span></p>"
        ),
        after: "",
    },
    Canary {
        file: "canary-icon-sized.html",
        what: "icons drawn at 16, 20 and 24 px, and one at 13 px not drawn, which must pass",
        expect: &[],
        inside: concat!(
            "<p><span class=fa-svg>",
            icon!("16px", "16px"),
            "</span><span class=fa-svg>",
            icon!("20px", "20px"),
            "</span><span class=fa-svg>",
            icon!("24px", "24px"),
            "</span><span class=fa-svg style=\"display: none\">",
            icon!("13px", "13px"),
            "</span></p>"
        ),
        after: "",
    },
    Canary {
        file: "canary-selection-default.html",
        what: "selected text with no color of the page's, as the browser reports a page without \
               the rule: transparent",
        expect: &[Kind::Selection],
        inside: "<style>::selection { background-color: initial !important; }</style>",
        after: "",
    },
    Canary {
        file: "canary-icon-zero.html",
        what: "an icon drawn at 0 px, as one whose size rule is lost inside a box of no font size",
        expect: &[Kind::Icon],
        inside: concat!("<p><span class=fa-svg>", icon!("0px", "0px"), "</span></p>"),
        after: "",
    },
    Canary {
        file: "canary-selection-color.html",
        what: "selected text on the danger role at 22 %, the right opacity in the wrong color",
        expect: &[Kind::Selection],
        inside: "<style>::selection { background-color: rgb(172 0 30 / 0.22) !important; }</style>",
        after: "",
    },
    Canary {
        file: "canary-selection-opaque.html",
        what: "selected text on the action role, opaque",
        expect: &[Kind::Selection],
        inside: "<style>::selection { background-color: #3350D6 !important; }</style>",
        after: "",
    },
    Canary {
        file: "canary-frame.html",
        what: "a page's frame drawn as the system's, which must pass",
        expect: &[],
        inside: "",
        after: frame!("", lockup!(), sidebar!(), ""),
    },
    Canary {
        file: "canary-lockup-small.html",
        what: "a lockup 20 px tall",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>.fs-lockup { height: 20px !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-lockup-action.html",
        what: "a lockup in the action role, not the ink",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>.fs-logo { color: #3350D6 !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-lockup-cut.html",
        what: "a lockup cut by a box 60 px wide",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "",
            concat!(
                "<span style=\"display: inline-block; width: 60px; overflow: hidden\">",
                lockup!(),
                "</span>"
            ),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-lockup-crowded.html",
        what: "a lockup with the product's name 2 px from it, inside its clear space",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>.fs-logo { gap: 2px !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-menu-wide.html",
        what: "a menu button drawn only from 720 px",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>#mdbook-sidebar-toggle { display: none !important; } @media (min-width: \
             720px) { #mdbook-sidebar-toggle { display: inline-flex !important; } }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-unread.html",
        what: "a sidebar drawn at every width and hidden from screen readers",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>#mdbook-sidebar { display: block !important; }</style>",
            lockup!(),
            "<nav id=\"mdbook-sidebar\" aria-hidden=\"true\"><a href=\"#\">A chapter</a></nav>",
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-untabbed.html",
        what: "a sidebar drawn at every width with its link out of the tab order",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>#mdbook-sidebar { display: block !important; }</style>",
            lockup!(),
            "<nav id=\"mdbook-sidebar\" aria-hidden=\"false\"><a href=\"#\" tabindex=\"-1\">A \
             chapter</a></nav>",
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-read.html",
        what: "a sidebar put away at every width and still read and reached",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>#mdbook-sidebar { display: none !important; }</style>",
            lockup!(),
            "<nav id=\"mdbook-sidebar\" aria-hidden=\"false\"><a href=\"#\">A chapter</a></nav>",
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-narrow.html",
        what: "a sidebar put away from 720 px and drawn under it, hidden from screen readers",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>#mdbook-sidebar { display: block !important; } @media (min-width: 720px) { \
             #mdbook-sidebar { display: none !important; } }</style>",
            lockup!(),
            "<nav id=\"mdbook-sidebar\" aria-hidden=\"true\"><a href=\"#\" tabindex=\"-1\">A \
             chapter</a></nav>",
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-noscript.html",
        what: "a sidebar and a menu button put away with scripts off",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>html:not(.js) #mdbook-sidebar, html:not(.js) #mdbook-sidebar-toggle { \
             display: none !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-sidebar-widened.html",
        what: "a sidebar drawn only when the window was wide at load, and from 720 px a lockup \
               20 px tall",
        expect: &[Kind::Header],
        inside: "",
        after: frame!(
            "<style>html:not(.wide-at-load) #mdbook-sidebar { display: none !important; } \
             @media (min-width: 720px) { .fs-lockup { height: 20px !important; } }</style>\
             <script>if (matchMedia('(min-width: 720px)').matches) \
             document.documentElement.classList.add('wide-at-load');</script>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-block-thin.html",
        what: "a title block bordered 1 px",
        expect: &[Kind::TitleBlock],
        inside: "",
        after: frame!(
            "<style>.fs-titleblock { border-width: 1px !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-block-rule.html",
        what: "a title block bordered in a rule's color, not the ink",
        expect: &[Kind::TitleBlock],
        inside: "",
        after: frame!(
            "<style>.fs-titleblock { border-color: #566079 !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-block-archivo.html",
        what: "a title block with its entries in Archivo",
        expect: &[Kind::TitleBlock],
        inside: "",
        after: frame!(
            "<style>.fs-titleblock .v { font-family: 'Archivo', 'Archivo Fallback', sans-serif \
             !important; font-variant-numeric: tabular-nums !important; }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-sheets.html",
        what: "an intro and sheets drawn as the system's, which must pass",
        expect: &[],
        inside: concat!(sheets!("", "SHEET 1 / 2", "SHEET 2 / 2"), ""),
        after: "",
    },
    Canary {
        file: "canary-sheet-thin.html",
        what: "a sheet under a 1 px rule",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-sheet { border-top-width: 1px !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-sheet-rule.html",
        what: "a sheet under a rule in a rule's color, not the ink",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-sheet { border-top-color: #566079 !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-sheet-order.html",
        what: "sheets numbered out of order",
        expect: &[Kind::Sheet],
        inside: concat!(sheets!("", "SHEET 2 / 2", "SHEET 1 / 2"), ""),
        after: "",
    },
    Canary {
        file: "canary-sheet-outside.html",
        what: "a section's name outside a sheet",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!("", "SHEET 1 / 2", "SHEET 2 / 2"),
            "<h2 style=\"font: 600 20px/24px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace\">A loose section</h2>"
        ),
        after: "",
    },
    Canary {
        file: "canary-sheet-number-archivo.html",
        what: "a sheet's number in Archivo 16 px",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-sheet-no { font: 400 16px/24px 'Archivo', 'Archivo Fallback', sans-serif !important; font-variant-numeric: tabular-nums !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-sheet-number-below.html",
        what: "a sheet's number below its name",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-sheet-rail { display: flex !important; flex-direction: column-reverse; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-tag-square.html",
        what: "a designation tag with no chamfered corner",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-tag { clip-path: none !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-tag-below.html",
        what: "a designation tag below its title",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-intro { display: flex; flex-direction: column-reverse; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-tag-large.html",
        what: "a designation tag in Cascadia Mono 14 px",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-tag { font-size: 14px !important; line-height: 20px !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-tag-dashed.html",
        what: "a designation tag in a dashed border",
        expect: &[Kind::Sheet],
        inside: concat!(
            sheets!(
                "<style>.fs-tag { border-style: dashed !important; }</style>",
                "SHEET 1 / 2",
                "SHEET 2 / 2"
            ),
            ""
        ),
        after: "",
    },
    Canary {
        file: "canary-switch.html",
        what: "a theme switch drawn and working as the system's, in the header from 960 px and the sidebar under it, which must pass",
        expect: &[],
        inside: "",
        after: switch_frame!("", switch_buttons!(), switch_placed!(), switch_works!()),
    },
    Canary {
        file: "canary-switch-names.html",
        what: "a theme switch offering Navy, not Dark",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "",
            "<li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"true\">Auto</button></li><li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"false\">Light</button></li><li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"false\">Navy</button></li>",
            switch_placed!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-pressed.html",
        what: "a theme switch with Light pressed for a first-time reader",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "",
            "<li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"false\">Auto</button></li><li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"true\">Light</button></li><li role=\"none\"><button type=\"button\" class=\"theme\" aria-pressed=\"false\">Dark</button></li>",
            switch_placed!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-toggle.html",
        what: "a theme switch beside mdBook's theme menu button, drawn",
        expect: &[Kind::Switch],
        inside: "<style>#mdbook-theme-toggle { font: inherit; color: inherit; background: transparent; border: 1px solid #566079; } @media print { #mdbook-theme-toggle { display: none; } }</style><p><button id=\"mdbook-theme-toggle\" type=\"button\">Themes</button></p>",
        after: switch_frame!("", switch_buttons!(), switch_placed!(), switch_works!()),
    },
    Canary {
        file: "canary-switch-dead.html",
        what: "a theme switch that sets no theme when pressed, once a run, at the width that reads print",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!("", switch_buttons!(), switch_placed!(), ""),
    },
    Canary {
        file: "canary-switch-retired.html",
        what: "a theme switch that keeps a saved theme it doesn't offer, Coal, read once a run, at the width that reads print",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!("", switch_buttons!(), switch_placed!(), switch_keeps!()),
    },
    Canary {
        file: "canary-switch-narrow.html",
        what: "a theme switch in its places, its buttons 32 px tall, read in the sidebar opened under 720 px too",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "<style>#mdbook-theme-list button { min-height: 32px !important; }</style>",
            switch_buttons!(),
            switch_placed!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-inverted.html",
        what: "a theme switch in the sidebar from 960 px and in the header under it",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "",
            switch_buttons!(),
            switch_placed_inverted!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-short.html",
        what: "a theme switch in the header at every width, its buttons 32 px tall",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "<style>#mdbook-theme-list button { min-height: 32px !important; }</style>",
            switch_buttons!(),
            switch_in_header!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-archivo.html",
        what: "a theme switch in the header at every width, its buttons in Archivo",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "<style>#mdbook-theme-list button { font-family: 'Archivo', 'Archivo Fallback', sans-serif !important; font-variant-numeric: tabular-nums; }</style>",
            switch_buttons!(),
            switch_in_header!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-unfilled.html",
        what: "a theme switch in the header at every width, its pressed button not filled",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "<style>#mdbook-theme-list [aria-pressed=\"true\"] { color: #0B0F1C !important; background: transparent !important; }</style>",
            switch_buttons!(),
            switch_in_header!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-switch-noscript.html",
        what: "a theme switch in the header at every width, drawn with scripts off",
        expect: &[Kind::Switch],
        inside: "",
        after: switch_frame!(
            "<style>html:not(.js) #mdbook-theme-list { display: inline-flex !important; }</style>",
            switch_buttons!(),
            switch_in_header!(),
            switch_works!()
        ),
    },
    Canary {
        file: "canary-mark-chrome.html",
        what: "a mark drawn as text outside the page's main",
        expect: &[Kind::Mark],
        inside: "",
        after: "<p><a href=\"#\">❱</a></p>",
    },
    Canary {
        file: "canary-mark-before.html",
        what: "a mark drawn as a heading's ::before",
        expect: &[Kind::Mark],
        inside: "<style>main h1::before { content: \"»\"; }</style>",
        after: "",
    },
    Canary {
        file: "canary-mark-target.html",
        what: "a mark drawn before a paragraph only when it is jumped to",
        expect: &[Kind::Mark],
        inside: "<style>p:target::before { content: \"» \"; }</style>\
                 <p id=\"jump\">A paragraph a link can jump to.</p>",
        after: "",
    },
    Canary {
        file: "canary-mark-words.html",
        what: "marks used as words in prose and a key's legend outside main, which must pass",
        expect: &[],
        inside: "<p>Drag falls as Mach → 0; press <kbd style=\"font: 14px/20px 'Cascadia \
                 Mono', 'Cascadia Mono Fallback', monospace\">←</kbd> to go back.</p><p>✓</p>",
        after: "<p>Press <kbd style=\"font: 14px/20px 'Cascadia Mono', 'Cascadia Mono \
                Fallback', monospace\">→</kbd> for the next chapter.</p>",
    },
    Canary {
        file: "canary-print-navy.html",
        what: "a page that prints in the navy theme's colors when read in navy",
        expect: &[Kind::Print],
        inside: "",
        after: concat!(
            switches!(),
            "<style>.navy, .navy body { color: #F3F4F7; background: #0B0F1C; }</style>"
        ),
    },
    Canary {
        file: "canary-print-navy-light.html",
        what: "a page that prints in the light theme's colors when read in navy, which must pass",
        expect: &[],
        inside: "",
        after: concat!(
            switches!(),
            "<style>.navy, .navy body { color: #F3F4F7; background: #0B0F1C; } @media print { \
             .navy, .navy body { color: #0B0F1C; background: #F3F4F7; } }</style>"
        ),
    },
    Canary {
        file: "canary-print-color.html",
        what: "a link printed in a blue of its own",
        expect: &[Kind::Print],
        inside: "<style>@media print { a { color: #4183C4; } }</style>",
        after: "",
    },
    Canary {
        file: "canary-print-screen.html",
        what: "a screen-only rule that keeps its color on paper",
        expect: &[Kind::Print],
        inside: "<style>main p { color: #4183C4; } @media screen { main p { color: #0B0F1C; } \
                 }</style>",
        after: "",
    },
    Canary {
        file: "canary-print-nav.html",
        what: "navigation drawn on paper",
        expect: &[Kind::Print],
        inside: "<nav><a href=\"#\">The next chapter</a></nav>",
        after: "",
    },
    Canary {
        file: "canary-print-arrows.html",
        what: "a page arrow drawn on paper, fixed to the window's side inside a nav with no height",
        expect: &[Kind::Print],
        inside: "",
        after: "<nav><a href=\"#\" aria-label=\"The next chapter\" style=\"position: fixed; \
                top: 0; bottom: 0; right: 0; width: 24px\"></a></nav>",
    },
    Canary {
        file: "canary-print-control.html",
        what: "a control drawn on paper",
        expect: &[Kind::Print],
        inside: "<p><span role=\"button\" tabindex=\"0\">Change the theme</span></p>",
        after: "",
    },
    Canary {
        file: "canary-print-link.html",
        what: "a link to another page printed without its address",
        expect: &[Kind::Print],
        inside: "<p>See <a href=\"other.html\">another page</a>.</p>",
        after: "",
    },
    Canary {
        file: "canary-print-link-outside.html",
        what: "a link outside main, as a title block's, printed without its address",
        expect: &[Kind::Print],
        inside: "<style>@media print { main a[href]:not([href^='#'])::after { content: ' (' \
                 attr(href) ')'; } }</style>",
        after: "<p>See <a href=\"other.html\">another page</a>.</p>",
    },
    Canary {
        file: "canary-print-address.html",
        what: "links to another page, in main and outside it, printed with their addresses, which \
               must pass",
        expect: &[],
        inside: "<style>@media print { a[href]:not([href^='#'])::after { content: ' (' \
                 attr(href) ')'; } }</style><p>See <a href=\"other.html\">another page</a>.</p>",
        after: "<p>See <a href=\"https://example.com/\">another site</a>.</p>",
    },
    Canary {
        file: "canary-print-fixed.html",
        what: "a rule fixed to the window's top on paper, drawn over the top of every sheet",
        expect: &[Kind::Print],
        inside: "<style>@media print { main::before { content: ''; position: fixed; top: 0; \
                 left: 0; right: 0; height: 4px; } }</style>",
        after: "",
    },
    Canary {
        file: "canary-print-row.html",
        what: "a table row that can split across sheets",
        expect: &[Kind::Print],
        inside: "<style>@media print { tr { break-inside: auto !important; } }</style>",
        after: "",
    },
    Canary {
        file: "canary-print-block-hidden.html",
        what: "a title block not drawn on paper",
        expect: &[Kind::Print],
        inside: "",
        after: frame!(
            "<style>@media print { .fs-titleblock { display: none !important; } }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-print-block-undated.html",
        what: "a title block printed without its date of issue",
        expect: &[Kind::Print],
        inside: "",
        after: frame!(
            "<style>@media print { .fs-titleblock > div:nth-child(4) { display: none \
             !important; } }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-print-block-split.html",
        what: "a title block that can split across sheets",
        expect: &[Kind::Print],
        inside: "",
        after: frame!(
            "<style>@media print { .fs-titleblock { break-inside: auto !important; } }</style>",
            lockup!(),
            sidebar!(),
            ""
        ),
    },
    Canary {
        file: "canary-block-followed.html",
        what: "a title block with a paragraph below it",
        expect: &[Kind::TitleBlock],
        inside: "",
        after: frame!(
            "",
            lockup!(),
            sidebar!(),
            "<p>A line after the title block.</p>"
        ),
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
body {{ margin: 0; font: 16px/24px 'Archivo', 'Archivo Fallback', sans-serif; font-variant-numeric: tabular-nums; overflow-x: hidden; color: #0B0F1C; background: #F3F4F7; }}
html {{ color: #0B0F1C; background: #F3F4F7; }}
a {{ color: #3350D6; }}
:focus-visible, .checkbox-img:focus-visible + img {{ outline: 2px solid #3350D6; outline-offset: 2px; }}
mark {{ color: #FFFFFF; background: #3350D6; }}
@media (prefers-color-scheme: dark) {{ mark {{ color: #0B0F1C; background: #768DF5; }} }}
::selection {{ background-color: rgb(51 80 214 / 0.22); }}
@media (prefers-color-scheme: dark) {{ ::selection {{ background-color: rgb(118 141 245 / 0.22); }} }}
input {{ color: inherit; }}
.content {{ overflow-y: auto; }}
main {{ overflow-x: clip; }}
.content {{ padding: 0 16px; }}
@media (min-width: 720px) {{ .content {{ padding: 0 32px; }} }}
main p:not(:has(> .checkbox-label)), main li {{ max-width: 26em; }}
code, pre {{ font: 14px/20px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; }}
.sidebar {{ font: 14px/20px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; }}
.sidebar a.active {{ text-decoration: underline 2px; }}
img {{ max-width: 100%; }}
.checkbox-img {{ position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0, 0, 0, 0); }}
.checkbox-img:not(:checked) ~ .img-wrapper {{ display: none; }}
h1 {{ font: 600 28px/32px 'Cascadia Mono', 'Cascadia Mono Fallback', monospace; }}
.scroll, pre {{ overflow-x: auto; }}
td {{ white-space: nowrap; padding: 0 12px; }}
.word {{ overflow-wrap: anywhere; }}
.sidebar {{ position: fixed; left: 0; top: 0; bottom: 0; width: 300px; transform: translateX(-100%); }}
@media print {{ .sidebar {{ display: none; }} tr {{ break-inside: avoid; }} }}
</style></head><body>
<nav class=\"sidebar\"><ol><li><a class=\"active\" href=\"#\">A chapter in a sidebar drawer put away</a></li></ol></nav>
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

/// The canaries whose problem shows only where the theme switch is pressed or the page loaded
/// with a theme saved, which the harness does once a run, at the width that reads print
/// ([`print_width`]).
const PRESSED: [&str; 2] = ["canary-switch-dead.html", "canary-switch-retired.html"];

/// Runs the page check on the site built in `output`.
pub(super) fn check(output: &Path, part: Part) -> Result<Report, String> {
    let started = Instant::now();
    let checked = part.widths();
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
        "widths": checked,
        "print_width": print_width(&checked),
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
    let collected = collect(&receiver, &mut browser, total, tabs, &checked);
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

    // Every canary not found as it should be, so one run names them all.
    let mut broken = Vec::new();
    for canary in &CANARIES {
        let page = format!("{CHECK_PREFIX}{}", canary.file);
        let measured = results
            .get(&page)
            .ok_or_else(|| format!("the page check is broken: no results for {page}"))?;
        if let Err(err) = canary_verdict(canary, measured) {
            broken.push(err);
        }
    }
    if !broken.is_empty() {
        return Err(broken.join("\n"));
    }
    let mut problems = Vec::new();
    for page in &pages {
        let measured = results
            .get(page)
            .ok_or_else(|| format!("the page check has no results for {page}"))?;
        problems.extend(page_problems(page, measured, part.step_px()));
    }
    Ok(Report {
        pages: pages.len(),
        widths: checked.len(),
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
    /// The page's frame (#384): its type, measure and gutters, the first few of each kind.
    #[serde(default)]
    r#type: Vec<Style>,
    #[serde(default)]
    type_count: usize,
    #[serde(default)]
    measure: Vec<Style>,
    #[serde(default)]
    measure_count: usize,
    #[serde(default)]
    gutter: Vec<Style>,
    #[serde(default)]
    gutter_count: usize,
    #[serde(default)]
    squeezed: Vec<Style>,
    #[serde(default)]
    squeezed_count: usize,
    #[serde(default)]
    note: Vec<Style>,
    #[serde(default)]
    note_count: usize,
    /// What a keyboard, forced colors or a sticky bar needs (#383), the first few of each kind.
    #[serde(default)]
    contrast: Vec<Style>,
    #[serde(default)]
    contrast_count: usize,
    #[serde(default)]
    focus: Vec<Style>,
    #[serde(default)]
    focus_count: usize,
    #[serde(default)]
    forced: Vec<Style>,
    #[serde(default)]
    forced_count: usize,
    #[serde(default)]
    sticky: Vec<Style>,
    #[serde(default)]
    sticky_count: usize,
    /// The chrome's icons off the system's sizes, and selections off the action role (#383).
    #[serde(default)]
    icon: Vec<Style>,
    #[serde(default)]
    icon_count: usize,
    #[serde(default)]
    selection: Vec<Style>,
    #[serde(default)]
    selection_count: usize,
    /// The page's header and title block off the system's (#384).
    #[serde(default)]
    header: Vec<Style>,
    #[serde(default)]
    header_count: usize,
    #[serde(default)]
    title_block: Vec<Style>,
    #[serde(default)]
    title_block_count: usize,
    /// Text marks drawn as icons, and what prints off the system's (#383).
    #[serde(default)]
    mark: Vec<Style>,
    #[serde(default)]
    mark_count: usize,
    #[serde(default)]
    print: Vec<Style>,
    #[serde(default)]
    print_count: usize,
    /// The intro and sheets, and the theme switch, off the system's (#384).
    #[serde(default)]
    sheet: Vec<Style>,
    #[serde(default)]
    sheet_count: usize,
    #[serde(default)]
    theme_switch: Vec<Style>,
    #[serde(default)]
    theme_switch_count: usize,
    /// Whether the page was read on paper at this width, as at one width of each run.
    #[serde(default)]
    printed: bool,
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
            Kind::Type => self.type_count > 0,
            Kind::Measure => self.measure_count > 0,
            Kind::Gutter => self.gutter_count > 0,
            Kind::Squeezed => self.squeezed_count > 0,
            Kind::Note => self.note_count > 0,
            Kind::Contrast => self.contrast_count > 0,
            Kind::Focus => self.focus_count > 0,
            Kind::Forced => self.forced_count > 0,
            Kind::Sticky => self.sticky_count > 0,
            Kind::Icon => self.icon_count > 0,
            Kind::Selection => self.selection_count > 0,
            Kind::Header => self.header_count > 0,
            Kind::TitleBlock => self.title_block_count > 0,
            Kind::Mark => self.mark_count > 0,
            Kind::Print => self.print_count > 0,
            Kind::Sheet => self.sheet_count > 0,
            Kind::Switch => self.theme_switch_count > 0,
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
            Kind::Type => Style::list(&self.r#type),
            Kind::Measure => Style::list(&self.measure),
            Kind::Gutter => Style::list(&self.gutter),
            Kind::Squeezed => Style::list(&self.squeezed),
            Kind::Note => Style::list(&self.note),
            Kind::Contrast => Style::list(&self.contrast),
            Kind::Focus => Style::list(&self.focus),
            Kind::Forced => Style::list(&self.forced),
            Kind::Sticky => Style::list(&self.sticky),
            Kind::Icon => Style::list(&self.icon),
            Kind::Selection => Style::list(&self.selection),
            Kind::Header => Style::list(&self.header),
            Kind::TitleBlock => Style::list(&self.title_block),
            Kind::Mark => Style::list(&self.mark),
            Kind::Print => Style::list(&self.print),
            Kind::Sheet => Style::list(&self.sheet),
            Kind::Switch => Style::list(&self.theme_switch),
        }
    }
}

/// An element whose computed style keeps one of mdBook's defaults, or breaks the page's frame.
#[derive(Debug, Deserialize)]
struct Style {
    /// A short path to its element, `tag#id.class>...`, and `::before` or `::after` for those.
    selector: String,
    /// The property, as CSS names it.
    property: String,
    /// Its computed value.
    value: String,
    /// The theme it was read in, and whether with reduced motion; for the frame, the window's
    /// width.
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

/// Reads one page's results, and checks they hold every width in `checked` once, in order.
fn parse_results(body: &str, checked: &[u32]) -> Result<(String, Vec<Measured>), String> {
    let results: PageResults =
        serde_json::from_str(body).map_err(|err| format!("unreadable results: {err}"))?;
    let got: Vec<u32> = results
        .widths
        .iter()
        .map(|measured| measured.width)
        .collect();
    if got != checked {
        return Err(format!(
            "the results for {} are at widths {got:?}, not the widths checked, {checked:?} px",
            results.page
        ));
    }
    Ok((results.page, results.widths))
}

/// The widths in `list`, which is in order, as ranges of consecutive steps: `320 to 400, 480`.
fn ranges(list: &[u32], step_px: u32) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut start = None;
    let mut last = 0;
    for &width in list {
        match start {
            Some(_) if width == last + step_px => {}
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
fn page_problems(page: &str, measured: &[Measured], step_px: u32) -> Vec<String> {
    let mut problems = Vec::new();
    let failed: Vec<&Measured> = measured.iter().filter(|m| m.error.is_some()).collect();
    if let Some(first) = failed.first() {
        let at: Vec<u32> = failed.iter().map(|m| m.width).collect();
        problems.push(format!(
            "{page}: could not be measured at {} px: {}",
            ranges(&at, step_px),
            first.error.as_deref().unwrap_or_default()
        ));
    }
    if failed.is_empty() && measured.iter().filter(|m| m.printed).count() != 1 {
        problems.push(format!(
            "{page}: was not read on paper at exactly one width of the run"
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
                ranges(&at, step_px),
                first.width,
                first.example(kind)
            ));
        }
    }
    problems
}

/// Whether a canary was found as it should be: each problem it expects at every width, no other.
/// What prints is read at one width of the run, so a print problem is expected there only, and
/// a run that never read the canary on paper is broken.
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
            let expected = canary.expect.contains(&kind)
                && (kind != Kind::Print || m.printed)
                && (!PRESSED.contains(&canary.file) || m.printed);
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
    if measured.iter().filter(|m| m.printed).count() != 1 {
        return Err(broken(
            "was not read on paper at exactly one width of the run".to_string(),
        ));
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
    checked: &[u32],
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
                let (page, measured) = parse_results(&body, checked)?;
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
pub(super) fn find_chrome() -> Result<PathBuf, String> {
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
pub(super) struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    pub(super) fn new() -> Result<Scratch, String> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        let dir =
            std::env::temp_dir().join(format!("hpr-page-check-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir)
            .map_err(|err| format!("could not create {}: {err}", dir.display()))?;
        Ok(Scratch { dir })
    }

    pub(super) fn profile(&self, tab: usize) -> PathBuf {
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

/// Starts one headless browser that the vitals check drives through DevTools
/// ([`super::vitals`]): its own fresh profile, a blank page, and DevTools on a port the browser
/// picks and writes into the profile.
pub(super) fn launch_debuggable(
    chrome: &Path,
    scratch: &Scratch,
    tab: usize,
) -> Result<Child, String> {
    launch_with(
        chrome,
        scratch,
        tab,
        "about:blank",
        &["--remote-debugging-port=0"],
    )
}

/// Starts one headless browser, with its own fresh profile, on `url`.
fn launch_one(chrome: &Path, scratch: &Scratch, tab: usize, url: &str) -> Result<Child, String> {
    launch_with(chrome, scratch, tab, url, &[])
}

/// Starts one headless browser, with its own fresh profile and the arguments `extra`, on `url`.
fn launch_with(
    chrome: &Path,
    scratch: &Scratch,
    tab: usize,
    url: &str,
    extra: &[&str],
) -> Result<Child, String> {
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
        .args(extra)
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
pub(super) struct Server {
    pub(super) address: SocketAddr,
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
    /// Whether text is sent compressed with gzip to a browser that accepts it, as GitHub Pages
    /// sends it (the vitals check); the page check reads the site uncompressed.
    compress: bool,
}

impl Server {
    /// The server the vitals check loads the site from: the site and the check's own files, text
    /// compressed as GitHub Pages compresses it.
    pub(super) fn start_compressed(site: PathBuf) -> Result<Server, String> {
        let (sender, _) = mpsc::channel();
        Server::open(site, String::new(), Vec::new(), sender, true)
    }

    fn start(
        site: PathBuf,
        plan: String,
        queue: Vec<String>,
        sender: Sender<News>,
    ) -> Result<Server, String> {
        Server::open(site, plan, queue, sender, false)
    }

    fn open(
        site: PathBuf,
        plan: String,
        queue: Vec<String>,
        sender: Sender<News>,
        compress: bool,
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
            compress,
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
    let mut gzip = false;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line).map_err(|err| err.to_string())?;
        head_bytes += read;
        if read == 0 || line == "\r\n" || line == "\n" || head_bytes > MOST_HEAD_BYTES {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                length = value.trim().parse().unwrap_or(usize::MAX);
            } else if name.trim().eq_ignore_ascii_case("accept-encoding") {
                gzip = value
                    .split(',')
                    .any(|coding| coding.split(';').next().unwrap_or("").trim() == "gzip");
            }
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
    let (body, encoding) = if shared.compress && gzip && compressible(kind) {
        (
            gzipped(&body)?,
            "Content-Encoding: gzip\r\nVary: Accept-Encoding\r\n",
        )
    } else {
        (body, "")
    };
    let head = format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\n{encoding}\
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

/// Whether GitHub Pages compresses a response of type `kind`: text, scripts, JSON and SVG, not
/// fonts or images, which are compressed already.
fn compressible(kind: &str) -> bool {
    kind.starts_with("text/") || kind.starts_with("application/json") || kind == "image/svg+xml"
}

/// `body` compressed with gzip at the default level.
fn gzipped(body: &[u8]) -> Result<Vec<u8>, String> {
    use flate2::Compression;
    use flate2::write::GzEncoder;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(body)
        .and_then(|()| encoder.finish())
        .map_err(|err| format!("could not compress a response: {err}"))
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
    if method == "GET"
        && let Some((kind, body, delay)) = super::vitals::served(name)
    {
        // The vitals check's late stylesheet is held back here, on its own connection.
        thread::sleep(delay);
        return (200, kind, body);
    }
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
        assert_eq!(ranges(&[], STEP_PX), "");
        assert_eq!(ranges(&[320], STEP_PX), "320");
        assert_eq!(ranges(&[320, 360, 400], STEP_PX), "320 to 400");
        assert_eq!(ranges(&[320, 360, 400, 480], STEP_PX), "320 to 400, 480");
        assert_eq!(
            ranges(&[320, 400, 440, 1920], STEP_PX),
            "320, 400 to 440, 1920"
        );
        // A part's widths are a part's step apart: 120 px for part 1 of 3.
        let step = Part { index: 1, count: 3 }.step_px();
        assert_eq!(ranges(&[320, 440, 560, 800], step), "320 to 560, 800");
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
            r#"{{"width":{width},"error":null,"printed":{printed},"client_width":{cw},"scroll_width":{scroll},
            "past":[{past_list}],"past_count":{past},"cut":[{cut_list}],"cut_count":{cut},
            "overlaps":[{overlap_list}],"overlap_count":{overlap}}}"#,
            cw = width - 15,
            printed = width == PRINT_PX,
            past_list = examples(past, "right"),
            cut_list = examples(cut, "lost"),
            overlap_list = overlaps.join(","),
        )
    }

    fn page_json(page: &str, each: impl Fn(u32) -> String) -> String {
        page_json_at(page, &widths(), each)
    }

    fn page_json_at(page: &str, at: &[u32], each: impl Fn(u32) -> String) -> String {
        let list: Vec<String> = at.iter().copied().map(each).collect();
        format!(r#"{{"page":"{page}","widths":[{}]}}"#, list.join(","))
    }

    #[test]
    fn the_parts_split_the_widths_evenly_and_cover_each_once() {
        assert_eq!(Part::WHOLE.widths(), widths());
        for count in [2, 3, 4, 41] {
            let mut all: Vec<u32> = Vec::new();
            for index in 1..=count {
                let part = Part::parse(&format!("{index}/{count}")).unwrap();
                let list = part.widths();
                // Interleaved: every part starts among the narrowest and ends among the widest.
                assert!(
                    list[0] < NARROWEST_PX + STEP_PX * count as u32,
                    "{part:?} {list:?}"
                );
                assert!(
                    *list.last().unwrap() + STEP_PX * count as u32 > WIDEST_PX,
                    "{part:?} {list:?}"
                );
                assert!(list.len().abs_diff(widths().len() / count) <= 1, "{part:?}");
                all.extend(list);
            }
            all.sort_unstable();
            assert_eq!(all, widths(), "{count} parts");
        }
        assert_eq!(Part::parse("2/3").unwrap().widths()[..3], [360, 480, 600]);
    }

    #[test]
    fn a_part_is_read_as_k_of_n() {
        assert_eq!(Part::parse("1/1"), Ok(Part::WHOLE));
        assert_eq!(Part::parse("3/3"), Ok(Part { index: 3, count: 3 }));
        for bad in [
            "0/3", "4/3", "1/42", "3", "a/3", "1/", "/3", "-1/3", "1/3/3",
        ] {
            let err = Part::parse(bad).unwrap_err();
            assert!(err.contains(&format!("`{bad}`")), "{bad}: {err}");
        }
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
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        assert_eq!(page, "guide.html");
        let problems = page_problems(&page, &measured, STEP_PX);
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
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        let problems = page_problems(&page, &measured, STEP_PX);
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
    fn what_a_keyboard_and_forced_colors_need_is_named_by_kind() {
        let body = page_json("guide.html", |w| {
            let plain = width_json(w, w - 15, 0, 0, 0);
            let sticky = usize::from(w >= 1080);
            plain.replacen(
                "\"overlap_count\":0}",
                &format!(
                    r#""overlap_count":0,"contrast":[{{"selector":"main>p","property":"color","value":"rgb(152, 161, 184) at 2.35 : 1, under 4.5 : 1","theme":"light"}}],"contrast_count":1,"focus":[{{"selector":"button#mdbook-theme-toggle","property":"outline","value":"1px auto rgb(0, 95, 204), offset 0px","theme":"light"}}],"focus_count":1,"forced":[{{"selector":"main>pre>code","property":"background-color","value":"rgb(255, 255, 255) with no border","theme":"light"}}],"forced_count":3,"sticky":[],"sticky_count":{sticky}}}"#
                ),
                1,
            )
        });
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        let problems = page_problems(&page, &measured, STEP_PX);
        assert_eq!(problems.len(), 4, "{problems:#?}");
        assert_eq!(
            problems[0],
            "guide.html: text under the contrast floor at 320 to 1920 px; at 320 px, `main>p` has \
             color `rgb(152, 161, 184) at 2.35 : 1, under 4.5 : 1` (light)"
        );
        assert_eq!(
            problems[1],
            "guide.html: a focus ring off the system's at 320 to 1920 px; at 320 px, \
             `button#mdbook-theme-toggle` has outline `1px auto rgb(0, 95, 204), offset 0px` \
             (light)"
        );
        assert!(
            problems[2].starts_with(
                "guide.html: something forced colors would erase at 320 to 1920 px; at 320 px, \
                 `main>pre>code` has background-color"
            ),
            "{}",
            problems[2]
        );
        assert!(
            problems[3].starts_with(
                "guide.html: a sticky bar that can hide what is focused at 1080 to 1920 px"
            ),
            "{}",
            problems[3]
        );
        // Each kind has a canary that must show it alone, and one beside it that must pass.
        for kind in [
            Kind::Contrast,
            Kind::Focus,
            Kind::Forced,
            Kind::Sticky,
            Kind::Icon,
            Kind::Selection,
        ] {
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
        for file in [
            "canary-contrast-large.html",
            "canary-forced-border.html",
            "canary-forced-mask-kept.html",
            "canary-sticky-padded.html",
            "canary-icon-sized.html",
        ] {
            assert!(passing.contains(&file), "{file}: {passing:?}");
        }
        assert!(
            CANARIES.iter().any(
                |canary| canary.file == "canary-smooth.html" && canary.expect == [Kind::Motion]
            )
        );
    }

    #[test]
    fn the_header_and_the_title_block_have_canaries_both_ways() {
        // Each kind has canaries that must show it alone, and the frame drawn as the system's
        // must pass.
        for (kind, least) in [(Kind::Header, 11), (Kind::TitleBlock, 4)] {
            let failing = CANARIES
                .iter()
                .filter(|canary| canary.expect == [kind])
                .count();
            assert!(failing >= least, "{kind:?}: {failing}");
        }
        assert!(CANARIES.iter().any(|canary| {
            canary.file == "canary-frame.html"
                && canary.expect.is_empty()
                && canary.after.contains("class=\"fs-lockup\"")
                && canary.after.contains("<footer class=\"fs-titleblock\"")
        }));
        assert_eq!(Kind::Header.describe(), "a header off the system's");
        assert_eq!(
            Kind::TitleBlock.describe(),
            "a title block off the system's or not last"
        );
    }

    #[test]
    fn the_sheets_and_the_switch_have_canaries_both_ways() {
        // Each kind has canaries that must show it alone, and one drawn as the system's that
        // must pass.
        for (kind, least, passing) in [
            (Kind::Sheet, 10, "canary-sheets.html"),
            (Kind::Switch, 11, "canary-switch.html"),
        ] {
            let failing = CANARIES
                .iter()
                .filter(|canary| canary.expect == [kind])
                .count();
            assert!(failing >= least, "{kind:?}: {failing}");
            assert!(
                CANARIES
                    .iter()
                    .any(|canary| canary.file == passing && canary.expect.is_empty()),
                "{passing}"
            );
        }
        assert_eq!(
            Kind::Sheet.describe(),
            "an intro or a sheet off the system's"
        );
        assert_eq!(Kind::Switch.describe(), "a theme switch off the system's");
    }

    #[test]
    fn the_frame_is_named_by_kind() {
        let body = page_json("guide.html", |w| {
            let plain = width_json(w, w - 15, 0, 0, 0);
            let gutter = if w >= 720 { 1 } else { 0 };
            plain.replacen(
                "\"overlap_count\":0}",
                &format!(
                    r#""overlap_count":0,"type":[{{"selector":"main>h1","property":"font","value":"32px/41.6px Cascadia Mono","theme":"{w} px"}}],"type_count":1,"measure":[{{"selector":"main>p","property":"a line of 118 characters","value":"The drag coefficient…","theme":"{w} px"}}],"measure_count":4,"gutter":[],"gutter_count":{gutter}}}"#
                ),
                1,
            )
        });
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        let problems = page_problems(&page, &measured, STEP_PX);
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert_eq!(
            problems[0],
            "guide.html: type off the system's scale at 320 to 1920 px; at 320 px, `main>h1` has \
             font `32px/41.6px Cascadia Mono` (320 px)"
        );
        assert_eq!(
            problems[1],
            "guide.html: a line of prose past 68 characters at 320 to 1920 px; at 320 px, \
             `main>p` has a line of 118 characters `The drag coefficient…` (320 px)"
        );
        assert!(
            problems[2].starts_with("guide.html: gutters off the system's at 720 to 1920 px"),
            "{}",
            problems[2]
        );
        // Each kind has a canary that must show it, and the ordinary page and a line of exactly
        // 68 characters must show none.
        for kind in [
            Kind::Type,
            Kind::Measure,
            Kind::Gutter,
            Kind::Squeezed,
            Kind::Note,
        ] {
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
        assert!(passing.contains(&"canary-measure-68.html"), "{passing:?}");
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
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        let problems = page_problems(&page, &measured, STEP_PX);
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
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        assert!(page_problems(&page, &measured, STEP_PX).is_empty());
    }

    #[test]
    fn print_is_read_at_the_width_nearest_a_sheet_of_paper() {
        assert_eq!(print_width(&widths()), 720);
        assert_eq!(print_width(&[320, 680, 760, 1920]), 680);
        assert_eq!(print_width(&[1040, 400]), 400);
        assert_eq!(print_width(&[]), 0);
        for count in [1, 2, 3, 4] {
            for index in 1..=count {
                let list = Part { index, count }.widths();
                let at = print_width(&list);
                assert!(list.contains(&at), "{index}/{count}");
                assert!(
                    at.abs_diff(PRINT_PX) < STEP_PX * count as u32,
                    "{index}/{count}"
                );
            }
        }
    }

    #[test]
    fn what_prints_is_read_once_and_named() {
        let printing = r#""overlap_count":0,"print":[{"selector":"main>p>a::after","property":"content","value":"no address (cli.html)","theme":"print, from light"}],"print_count":1}"#;
        let body = page_json("guide.html", |w| {
            let plain = width_json(w, w - 15, 0, 0, 0);
            if w == PRINT_PX {
                plain.replacen("\"overlap_count\":0}", printing, 1)
            } else {
                plain
            }
        });
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        assert_eq!(
            page_problems(&page, &measured, STEP_PX),
            [
                "guide.html: a page that prints off the system's at 720 px; at 720 px, \
              `main>p>a::after` has content `no address (cli.html)` (print, from light)"
            ]
        );
        // A canary that prints wrong shows it at the one width read on paper, and passes.
        let link = canary("canary-print-link.html");
        assert_eq!(link.expect, &[Kind::Print]);
        assert!(canary_verdict(link, &measured).is_ok());
        // Not read on paper at all, or at two widths: the check is broken, and a page fails.
        for printed in [&[][..], &[680, 720][..]] {
            let body = page_json("guide.html", |w| {
                width_json(w, w - 15, 0, 0, 0).replacen(
                    &format!("\"printed\":{}", w == PRINT_PX),
                    &format!("\"printed\":{}", printed.contains(&w)),
                    1,
                )
            });
            let (page, measured) = parse_results(&body, &widths()).unwrap();
            assert_eq!(
                page_problems(&page, &measured, STEP_PX),
                ["guide.html: was not read on paper at exactly one width of the run"]
            );
            let err = canary_verdict(canary("canary-mark-words.html"), &measured).unwrap_err();
            assert!(err.contains("was not read on paper"), "{err}");
        }
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
        let (page, measured) = parse_results(&body, &widths()).unwrap();
        let problems = page_problems(&page, &measured, STEP_PX);
        assert_eq!(
            problems,
            ["index.html: could not be measured at 360 px: did not load in 60 s"]
        );
    }

    #[test]
    fn results_must_hold_every_width() {
        let body = r#"{"page":"a.html","widths":[{"width":320,"error":null}]}"#;
        assert!(
            parse_results(body, &widths())
                .unwrap_err()
                .contains("not the widths checked")
        );
        assert!(
            parse_results("{", &widths())
                .unwrap_err()
                .contains("unreadable")
        );
        // A part's results hold its own widths, not every width.
        let part = Part { index: 2, count: 3 };
        let ok = page_json_at("a.html", &part.widths(), |w| width_json(w, w - 15, 0, 0, 0));
        assert_eq!(parse_results(&ok, &part.widths()).unwrap().1.len(), 14);
        assert!(parse_results(&ok, &widths()).is_err());
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
        let (_, measured) = parse_results(&found, &widths()).unwrap();
        assert!(canary_verdict(overlap, &measured).is_ok());
        // Missed at one width: the check is broken.
        let missed = page_json("x", |w| width_json(w, w - 15, 0, 0, usize::from(w != 760)));
        let (_, measured) = parse_results(&missed, &widths()).unwrap();
        let err = canary_verdict(overlap, &measured).unwrap_err();
        assert!(
            err.contains("at 760 px: `text over text` was not found"),
            "{err}"
        );
        // Found with another problem: broken too.
        let extra = page_json("x", |w| width_json(w, w - 15, 0, 1, 1));
        let (_, measured) = parse_results(&extra, &widths()).unwrap();
        let err = canary_verdict(overlap, &measured).unwrap_err();
        assert!(err.contains("`text cut off by its box` was found"), "{err}");
        // The ordinary page passes only with nothing found.
        let ordinary = canary("canary-ordinary.html");
        let clean = page_json("x", |w| width_json(w, w - 15, 0, 0, 0));
        let (_, measured) = parse_results(&clean, &widths()).unwrap();
        assert!(canary_verdict(ordinary, &measured).is_ok());
        let wide = page_json("x", |w| width_json(w, w + 10, 0, 0, 0));
        let (_, measured) = parse_results(&wide, &widths()).unwrap();
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
