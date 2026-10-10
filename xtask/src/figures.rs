//! The figure check (M0.7a1): nothing a figure draws may be cut off by its edge, by a clip, or by
//! other text. The project's rule is that nothing is ever clipped by an edge or a corner; where
//! something doesn't fit, the layout changes, never the crop. This holds the SVG figures to it.
//!
//! `check` reads one SVG and lists what is wrong with it:
//!
//! - **The frame** is the root `<svg>`'s `viewBox`, or `0 0 width height` without one. Every mark
//!   (`rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `path`) and every line of
//!   `text` must lie inside it, after each `transform` on the way down is applied (`matrix`,
//!   `translate`, `scale`, `rotate`, `skewX`, `skewY`). A transformed box is bounded by its
//!   transformed corners, which can only make it larger.
//! - **Paths** are bounded by their points and every Bézier control point (a curve lies inside
//!   its control points' hull), and an elliptical arc by its whole ellipse, found as the SVG
//!   specification converts an arc's endpoints to its center (its radii scaled up where they are
//!   too small to reach).
//! - **Strokes** (`stroke` inherited, none by default; `stroke-width` inherited, 1 by default)
//!   widen a mark by half their width: exactly for a straight segment, by a square around each
//!   point of a curve. A miter join (the default) reaches `w/2 · 1/sin(θ/2)` from its corner,
//!   θ the angle between the segments, while that ratio is within `stroke-miterlimit` (4 by
//!   default), and is beveled past it. A square cap reaches `w/2` past a line's end; with a
//!   dash pattern, every segment end is taken as capped.
//! - **Markers** (`marker-start`, `-mid`, `-end`) are bounded at each vertex they apply to by a
//!   circle as wide as the farthest corner of the marker's viewport from its reference point,
//!   which holds at any `orient`. A marker's own content must fit its viewport, which clips it.
//! - **Clips.** A `clip-path` to a `clipPath` of one rectangle is a frame of its own: what it
//!   holds must lie inside the rectangle, since what leaves it is cut off, and the rectangle
//!   must lie inside the frames around it. A `pattern`'s content must fit its tile likewise.
//! - **Text** is as wide as the sum of its characters' advances at its `font-size` (inherited,
//!   16 by default), each the widest of a set of common system fonts: a figure shown through
//!   `<img>` can't load a web font, so it draws in whichever of the reader's fonts its
//!   `font-family` list finds. The widths are in [`glyphs`], which `scripts/glyph-widths.py`
//!   writes from the fonts' own tables, regular and bold (`font-weight` 500 and up), proportional
//!   and monospace (a list of monospace families only), with each character's ink above and
//!   below the baseline and past its advance. The generic families stand for each system's
//!   defaults, which the table holds (`serif` and no family at all: Times, Times New Roman,
//!   DejaVu Serif). A newline or tab in the text draws as a space, as in a browser, and runs of
//!   spaces as one. `text-anchor` places the line; `dominant-baseline` `auto` or `alphabetic`
//!   puts its baseline at `y`, and `middle`, `central` or `hanging` anywhere from `y` to an em
//!   below it.
//! - **Text over text.** Two lines of text whose boxes overlap by more than
//!   [`OVERLAP_PX`] each way fail, since one would hide part of the other.
//! - **Fail closed.** What the check can't measure fails rather than passes: an element outside
//!   the list above (`use`, `image`, `style`, `foreignObject`, a `tspan`, a nested `svg`), an
//!   attribute that could move or size something without the check reading it
//!   (`letter-spacing`, `textLength`, `vector-effect`, a `style` that sets anything but paint),
//!   a `clip-path` on the root or on a clip's own rectangle, a length in units other than px, a
//!   negative size, stroke or font size (a browser draws another value instead) or a miter limit
//!   below 1, a font family the table doesn't cover (`system-ui` among them: Segoe UI on
//!   Windows), and a character the table lacks.
//!
//! [`drawing`] holds the same figures to the product system's colors and fonts (M0.9d1): every
//! paint a token, nothing blended, no default black, text in Cascadia Mono or Archivo.
//!
//! Its tests run both checks on every SVG under `docs/` and `theme/` (and fail on a tracked SVG
//! anywhere else), and `cargo xtask cli` (and its test) on each `--plot` figure it draws.
//! `cargo xtask figures` runs them on those SVGs by hand.
//!
//! What it leaves out: kerning, which almost always narrows a line, and ligatures, which do.
//! The table covers the fonts its header lists; a reader whose system draws in another font
//! with wider glyphs isn't covered.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use roxmltree::{Document, Node};

pub(crate) mod drawing;
mod glyphs;
#[cfg(test)]
mod tests;

/// The usage line in `cargo xtask help`.
pub const USAGE: &str = "  \
figures                  Check every SVG under docs/ and theme/: nothing it draws leaves its viewBox or
                           a clip, no text overlaps other text, and it draws in the design
                           system's color tokens and fonts only.";

/// How far two lines of text may overlap, px each way, before they fail: rounding, no more.
const OVERLAP_PX: f64 = 0.01;
/// How far a mark may pass a frame's edge, px: floating-point rounding, no more.
const EDGE_PX: f64 = 1e-6;
/// The SVG namespace.
const SVG: &str = "http://www.w3.org/2000/svg";
/// The XML namespace, of `xml:space` and `xml:lang`.
const XML: &str = "http://www.w3.org/XML/1998/namespace";

/// The attributes the check reads, and those that can't move or size anything. Any other fails.
const ATTRIBUTES: &[&str] = &[
    // Read.
    "viewBox",
    "width",
    "height",
    "x",
    "y",
    "x1",
    "y1",
    "x2",
    "y2",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "d",
    "points",
    "transform",
    "stroke",
    "stroke-width",
    "stroke-linejoin",
    "stroke-linecap",
    "stroke-miterlimit",
    "stroke-dasharray",
    "marker-start",
    "marker-mid",
    "marker-end",
    "markerWidth",
    "markerHeight",
    "markerUnits",
    "refX",
    "refY",
    "orient",
    "font-family",
    "font-size",
    "font-weight",
    "text-anchor",
    "dominant-baseline",
    "clip-path",
    "clipPathUnits",
    "patternUnits",
    "patternContentUnits",
    "patternTransform",
    "style",
    // Paint, names and accessibility: nothing they set moves or sizes a mark.
    "id",
    "class",
    "version",
    "role",
    "focusable",
    "tabindex",
    "fill",
    "fill-opacity",
    "fill-rule",
    "opacity",
    "stroke-opacity",
    "stroke-dashoffset",
    "pathLength",
    "clip-rule",
    "color",
    "shape-rendering",
    "text-rendering",
    "gradientUnits",
    "gradientTransform",
    "offset",
    "stop-color",
    "stop-opacity",
    "spreadMethod",
    "fx",
    "fy",
    "fr",
];

/// The properties a `style` attribute may set: paint only. The check reads geometry from
/// attributes, so a `style` that set it would go unread.
const STYLE_PROPERTIES: &[&str] = &[
    "fill",
    "fill-opacity",
    "fill-rule",
    "opacity",
    "stroke-opacity",
    "color",
    "stop-color",
    "stop-opacity",
    "shape-rendering",
    "text-rendering",
];

/// Families that are drawn with the metrics of a family the table measured: Liberation and
/// Google's Arimo, Tinos and Cousine are made metric-compatible with Arial, Times New Roman and
/// Courier New.
const PROPORTIONAL_ALIASES: &[&str] = &["Liberation Sans", "Arimo", "Liberation Serif", "Tinos"];
/// See [`PROPORTIONAL_ALIASES`].
const MONOSPACE_ALIASES: &[&str] = &["Liberation Mono", "Cousine"];
/// The generic families a browser maps to its default proportional fonts.
/// Not `system-ui`: it is Segoe UI on Windows, which the table doesn't measure.
const PROPORTIONAL_GENERICS: &[&str] = &["serif", "sans-serif", "ui-sans-serif"];
/// The generic families a browser maps to its default monospace fonts.
const MONOSPACE_GENERICS: &[&str] = &["monospace", "ui-monospace"];

/// Runs the command: checks every SVG under `docs/` and `theme/`.
pub fn run(args: &[String]) -> Result<(), String> {
    if !args.is_empty() {
        return Err(format!("unknown arguments {args:?}\n\n{USAGE}"));
    }
    let root = crate::designs::root()?;
    let checked = check_docs(&root)?;
    println!(
        "figures: {checked} SVGs under docs/ and theme/ draw nothing clipped, no text over text, and in \
         token colors and the system's fonts only"
    );
    Ok(())
}

/// The folders whose SVGs are checked: the docs, and mdBook's theme beside `book.toml`.
pub(crate) const FOLDERS: [&str; 2] = ["docs", "theme"];

/// Whether `name`, relative to the root, is an SVG the theme copies unchanged from the product
/// system (`site::theme::COPIED`), which `cargo xtask site` compares with the pinned revision
/// instead: the favicon, the system's brand mark (its gradient and its drawing program's markup
/// are the brand's, as in the brand files under `.github/brand/`), and the icons the site draws
/// inline, in the text's color (`currentColor`, which no figure may use: an image has no text
/// color), whose color the page check reads (ADR-228).
pub(crate) fn systems_copy(name: &Path) -> bool {
    crate::site::theme::COPIED
        .iter()
        .any(|(file, _)| file.ends_with(".svg") && name == Path::new("theme").join(file))
}

/// Checks every SVG under [`FOLDERS`] of `root`; how many there are, or what is wrong with them.
pub(crate) fn check_docs(root: &Path) -> Result<usize, String> {
    let mut files = Vec::new();
    for folder in FOLDERS {
        let dir = root.join(folder);
        if dir.is_dir() {
            svgs(&dir, &mut files)?;
        }
    }
    files.retain(|file| !file.strip_prefix(root).is_ok_and(systems_copy));
    files.sort();
    let mut failures = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
        let name = file
            .strip_prefix(root)
            .unwrap_or(file)
            .display()
            .to_string();
        for problem in check(&text).into_iter().chain(drawing::check(&text)) {
            failures.push(format!("{name}: {problem}"));
        }
    }
    if failures.is_empty() {
        Ok(files.len())
    } else {
        Err(format!(
            "{} problems in figures; change the layout so it fits, never the crop, and draw in \
             the tokens:\n  {}",
            failures.len(),
            failures.join("\n  ")
        ))
    }
}

/// The `.svg` files under `dir`, into `out`.
fn svgs(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if path.is_dir() {
            svgs(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "svg") {
            out.push(path);
        }
    }
    Ok(())
}

/// What kind of problem a figure has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Something the check can't measure: an element, an attribute or a value it doesn't know.
    Unsupported,
    /// A value that doesn't parse: a number, a path's data, a transform, a reference.
    Malformed,
    /// A character the glyph table doesn't have.
    Character,
    /// A mark or a line of text that leaves the figure's frame.
    OutsideFrame,
    /// Content that leaves the clip, marker viewport or pattern tile that cuts it off.
    Clipped,
    /// Two lines of text whose boxes overlap.
    Overlap,
    /// A paint that is not one of the product system's tokens, a blend, or the default black.
    Color,
    /// A line of text set in a family other than the product system's two.
    Font,
}

/// One thing wrong with a figure.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Problem {
    /// What kind it is.
    pub(crate) kind: Kind,
    /// What and where, in words.
    pub(crate) message: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

/// What is wrong with the SVG `text`: nothing when it draws inside its frame and its clips, with
/// no text over text, and the check could measure all of it.
pub(crate) fn check(text: &str) -> Vec<Problem> {
    let doc = match Document::parse(text) {
        Ok(doc) => doc,
        Err(e) => {
            return vec![Problem {
                kind: Kind::Malformed,
                message: format!("not XML: {e}"),
            }];
        }
    };
    let mut checker = Checker {
        doc: &doc,
        ids: doc
            .descendants()
            .filter_map(|node| Some((node.attribute("id")?, node)))
            .collect(),
        problems: Vec::new(),
        texts: Vec::new(),
    };
    checker.root();
    checker.overlaps();
    checker.problems
}

/// A point, px.
type P = (f64, f64);

/// A glyph table: each character, then its advance, its ink's overhang left of its start and
/// right of its advance, and its ink's height above and depth below the baseline, in thousandths
/// of an em, sorted by character.
type Table = &'static [(char, [u16; 5])];

/// An affine map, `x' = a x + c y + e`, `y' = b x + d y + f`, as SVG's `matrix(a b c d e f)`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix([f64; 6]);

impl Matrix {
    /// The identity.
    const IDENTITY: Self = Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

    /// `self` after `inner`: a point goes through `inner` first.
    fn then(self, inner: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [p, q, r, s, t, u] = inner.0;
        Self([
            a * p + c * q,
            b * p + d * q,
            a * r + c * s,
            b * r + d * s,
            a * t + c * u + e,
            b * t + d * u + f,
        ])
    }

    /// Where `p` goes.
    fn apply(self, (x, y): P) -> P {
        let [a, b, c, d, e, f] = self.0;
        (a * x + c * y + e, b * x + d * y + f)
    }

    /// The inverse map, unless the map collapses the plane.
    fn inverse(self) -> Option<Self> {
        let [a, b, c, d, e, f] = self.0;
        let det = a * d - b * c;
        if !(det.is_finite() && det.abs() > 1e-12) {
            return None;
        }
        Some(Self([
            d / det,
            -b / det,
            -c / det,
            a / det,
            (c * f - d * e) / det,
            (b * e - a * f) / det,
        ]))
    }

    /// A translation.
    fn translate(x: f64, y: f64) -> Self {
        Self([1.0, 0.0, 0.0, 1.0, x, y])
    }
}

/// An axis-aligned box, px.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Bounds {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl Bounds {
    /// The least box holding `points`, if there are any.
    fn of(points: impl IntoIterator<Item = P>) -> Option<Self> {
        let mut points = points.into_iter();
        let (x, y) = points.next()?;
        let mut b = Self {
            x0: x,
            y0: y,
            x1: x,
            y1: y,
        };
        for (x, y) in points {
            b.x0 = b.x0.min(x);
            b.y0 = b.y0.min(y);
            b.x1 = b.x1.max(x);
            b.y1 = b.y1.max(y);
        }
        Some(b)
    }

    /// Whether the box lies inside `frame`, to [`EDGE_PX`]; a NaN anywhere is outside.
    fn within(&self, frame: &Self) -> bool {
        self.x0 >= frame.x0 - EDGE_PX
            && self.y0 >= frame.y0 - EDGE_PX
            && self.x1 <= frame.x1 + EDGE_PX
            && self.y1 <= frame.y1 + EDGE_PX
    }

    /// The corners.
    fn corners(&self) -> [P; 4] {
        [
            (self.x0, self.y0),
            (self.x1, self.y0),
            (self.x0, self.y1),
            (self.x1, self.y1),
        ]
    }
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x {:.2} to {:.2}, y {:.2} to {:.2}",
            self.x0, self.x1, self.y0, self.y1
        )
    }
}

/// How a stroke's segments meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Join {
    Miter,
    Round,
    Bevel,
}

/// How a stroke's open ends are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cap {
    Butt,
    Round,
    Square,
}

/// Where a line of text sits on its `x`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Anchor {
    Start,
    Middle,
    End,
}

/// The inherited properties the check reads.
#[derive(Debug, Clone)]
struct Style {
    stroked: bool,
    stroke_width: f64,
    join: Join,
    miter_limit: f64,
    cap: Cap,
    dashed: bool,
    /// The `marker-start`, `-mid` and `-end` ids.
    markers: [Option<String>; 3],
    family: Option<String>,
    font_size: f64,
    bold: bool,
    anchor: Anchor,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            stroked: false,
            stroke_width: 1.0,
            join: Join::Miter,
            miter_limit: 4.0,
            cap: Cap::Butt,
            dashed: false,
            markers: [None, None, None],
            family: None,
            font_size: 16.0,
            bold: false,
            anchor: Anchor::Start,
        }
    }
}

/// A region that cuts off what it holds: the viewBox, a clip, a marker's viewport or a
/// pattern's tile.
#[derive(Debug, Clone)]
struct Frame {
    /// What it is, in words.
    name: String,
    /// What leaving it is.
    kind: Kind,
    /// The region, in its own coordinates.
    rect: Bounds,
    /// From the walk's base coordinates to the frame's.
    from_base: Matrix,
}

/// Where in the document a walk is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    /// Drawn on the figure.
    Drawn,
    /// Not drawn by itself (`defs`, a `clipPath`): checked only for what it uses.
    Hidden,
    /// Drawn in a marker's or a pattern's own coordinates, at places the check doesn't follow.
    Inside,
}

/// The state an element inherits.
#[derive(Debug, Clone)]
struct Context {
    /// From the element's user space to the walk's base coordinates.
    ctm: Matrix,
    style: Style,
    frames: Vec<Frame>,
    place: Place,
}

/// One subpath: its segments, and whether it is closed.
#[derive(Debug, Clone, Default)]
struct Subpath {
    start: P,
    segments: Vec<Segment>,
    closed: bool,
}

/// A segment of a path.
#[derive(Debug, Clone, Copy)]
enum Segment {
    Line(P, P),
    Quad(P, P, P),
    Cubic(P, P, P, P),
    /// An elliptical arc from `from` to `to` about `center`, its radii and the x axis's angle,
    /// rad, its start angle and sweep, rad.
    Arc {
        from: P,
        to: P,
        center: P,
        radii: P,
        phi: f64,
        start: f64,
        sweep: f64,
    },
}

impl Segment {
    fn start(&self) -> P {
        match *self {
            Self::Line(a, _) | Self::Quad(a, _, _) | Self::Cubic(a, _, _, _) => a,
            Self::Arc { from, .. } => from,
        }
    }

    fn end(&self) -> P {
        match *self {
            Self::Line(_, b) | Self::Quad(_, _, b) | Self::Cubic(_, _, _, b) => b,
            Self::Arc { to, .. } => to,
        }
    }

    /// Points whose hull holds the segment: a Bézier's control points, an arc's whole
    /// ellipse's box.
    fn hull(&self) -> Vec<P> {
        match *self {
            Self::Line(a, b) => vec![a, b],
            Self::Quad(a, b, c) => vec![a, b, c],
            Self::Cubic(a, b, c, d) => vec![a, b, c, d],
            Self::Arc {
                from,
                to,
                center: (cx, cy),
                radii: (rx, ry),
                phi,
                ..
            } => {
                let (sin, cos) = phi.sin_cos();
                let hx = (rx * rx * cos * cos + ry * ry * sin * sin).sqrt();
                let hy = (rx * rx * sin * sin + ry * ry * cos * cos).sqrt();
                vec![
                    from,
                    to,
                    (cx - hx, cy - hy),
                    (cx + hx, cy - hy),
                    (cx - hx, cy + hy),
                    (cx + hx, cy + hy),
                ]
            }
        }
    }

    /// The direction it leaves its start in, unless it has no length there.
    fn start_direction(&self) -> Option<P> {
        match *self {
            Self::Line(a, b) => unit(sub(b, a)),
            Self::Quad(a, b, c) => unit(sub(b, a)).or_else(|| unit(sub(c, a))),
            Self::Cubic(a, b, c, d) => unit(sub(b, a))
                .or_else(|| unit(sub(c, a)))
                .or_else(|| unit(sub(d, a))),
            Self::Arc {
                radii,
                phi,
                start,
                sweep,
                ..
            } => arc_direction(radii, phi, start, sweep),
        }
    }

    /// The direction it arrives at its end in, unless it has no length there.
    fn end_direction(&self) -> Option<P> {
        match *self {
            Self::Line(a, b) => unit(sub(b, a)),
            Self::Quad(a, b, c) => unit(sub(c, b)).or_else(|| unit(sub(c, a))),
            Self::Cubic(a, b, c, d) => unit(sub(d, c))
                .or_else(|| unit(sub(d, b)))
                .or_else(|| unit(sub(d, a))),
            Self::Arc {
                radii,
                phi,
                start,
                sweep,
                ..
            } => arc_direction(radii, phi, start + sweep, sweep),
        }
    }
}

/// `a − b`.
fn sub(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1)
}

/// `v` scaled to length 1, unless it has none.
fn unit(v: P) -> Option<P> {
    let length = v.0.hypot(v.1);
    (length > 1e-12 && length.is_finite()).then(|| (v.0 / length, v.1 / length))
}

/// The direction an arc runs in at angle `theta`, its sweep's sign taken.
fn arc_direction((rx, ry): P, phi: f64, theta: f64, sweep: f64) -> Option<P> {
    let (sin, cos) = phi.sin_cos();
    let (st, ct) = theta.sin_cos();
    let v = (
        -rx * st * cos - ry * ct * sin,
        -rx * st * sin + ry * ct * cos,
    );
    unit(if sweep < 0.0 { (-v.0, -v.1) } else { v })
}

/// The four corners of the square of half-side `r` about `p`.
fn square(p: P, r: f64) -> [P; 4] {
    [
        (p.0 - r, p.1 - r),
        (p.0 + r, p.1 - r),
        (p.0 - r, p.1 + r),
        (p.0 + r, p.1 + r),
    ]
}

/// The walk over one document.
struct Checker<'a, 'input> {
    doc: &'a Document<'input>,
    ids: HashMap<&'a str, Node<'a, 'input>>,
    problems: Vec<Problem>,
    /// Each drawn line of text: what it says, where, and its box in the root's coordinates.
    texts: Vec<(String, Bounds)>,
}

impl<'a, 'input> Checker<'a, 'input> {
    /// Records a problem.
    fn problem(&mut self, kind: Kind, message: String) {
        self.problems.push(Problem { kind, message });
    }

    /// `node` as a reader finds it: its line and its tag, with its id.
    fn label(&self, node: Node<'_, '_>) -> String {
        let line = self.doc.text_pos_at(node.range().start).row;
        match node.attribute("id") {
            Some(id) => format!("line {line} <{} id=\"{id}\">", node.tag_name().name()),
            None => format!("line {line} <{}>", node.tag_name().name()),
        }
    }

    /// The root element, its frame, and everything under it.
    fn root(&mut self) {
        let root = self.doc.root_element();
        if root.tag_name().namespace() != Some(SVG) || root.tag_name().name() != "svg" {
            let label = self.label(root);
            self.problem(
                Kind::Unsupported,
                format!("{label}: the root is not an SVG <svg> element"),
            );
            return;
        }
        self.attributes(root);
        if root.has_attribute("transform") {
            let label = self.label(root);
            self.problem(
                Kind::Unsupported,
                format!("{label}: a transform on the root, which the check doesn't follow"),
            );
        }
        if root.has_attribute("clip-path") {
            // Its clip's user space is the viewBox's in one reading and the viewport's in
            // another, and browsers differ; a clip on a <g> inside it has one.
            let label = self.label(root);
            self.problem(
                Kind::Unsupported,
                format!(
                    "{label}: a clip-path on the root, which browsers place differently; clip a \
                     <g> inside it"
                ),
            );
        }
        let frame = match root.attribute("viewBox") {
            Some(text) => self.view_box(root, text),
            None => {
                let width = self.length(root, "width", None);
                let height = self.length(root, "height", None);
                match (width, height) {
                    (Some(w), Some(h)) => Some(Bounds {
                        x0: 0.0,
                        y0: 0.0,
                        x1: w,
                        y1: h,
                    }),
                    _ => {
                        let label = self.label(root);
                        self.problem(
                            Kind::Malformed,
                            format!("{label}: no viewBox, and no width and height to frame it"),
                        );
                        None
                    }
                }
            }
        };
        let Some(rect) = frame else {
            return;
        };
        let style = self.style(root, &Style::default());
        let context = Context {
            ctm: Matrix::IDENTITY,
            style,
            frames: vec![Frame {
                name: format!("the viewBox ({rect})"),
                kind: Kind::OutsideFrame,
                rect,
                from_base: Matrix::IDENTITY,
            }],
            place: Place::Drawn,
        };
        self.children(root, &context);
    }

    /// A `viewBox`'s rectangle.
    fn view_box(&mut self, node: Node<'_, '_>, text: &str) -> Option<Bounds> {
        match numbers(text).as_deref() {
            Some(&[x, y, w, h]) if w > 0.0 && h > 0.0 => Some(Bounds {
                x0: x,
                y0: y,
                x1: x + w,
                y1: y + h,
            }),
            _ => {
                let label = self.label(node);
                self.problem(
                    Kind::Malformed,
                    format!("{label}: viewBox `{text}` is not four numbers with a positive size"),
                );
                None
            }
        }
    }

    /// Each child element of `node`, in `context`.
    fn children(&mut self, node: Node<'a, 'input>, context: &Context) {
        for child in node.children() {
            if child.is_element() {
                self.element(child, context);
            } else if child.is_text() && !child.text().unwrap_or_default().trim().is_empty() {
                // Text outside a <text> draws nothing, but a stray one is a sign of a mistake.
                let label = self.label(node);
                self.problem(
                    Kind::Unsupported,
                    format!("{label}: holds text outside a <text> element"),
                );
            }
        }
    }

    /// One element and what it holds.
    fn element(&mut self, node: Node<'a, 'input>, outer: &Context) {
        let name = node.tag_name().name();
        if node.tag_name().namespace() != Some(SVG) {
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: an element outside the SVG namespace"),
            );
            return;
        }
        if matches!(name, "title" | "desc" | "metadata") {
            return;
        }
        self.attributes(node);
        let mut context = Context {
            ctm: outer.ctm.then(self.transform(node, "transform")),
            style: self.style(node, &outer.style),
            frames: outer.frames.clone(),
            place: outer.place,
        };
        if let Some(reference) = node.attribute("clip-path") {
            self.clip(node, reference, &mut context);
        }
        match name {
            "g" => self.children(node, &context),
            "defs" => {
                context.place = Place::Hidden;
                self.children(node, &context);
            }
            "clipPath" => self.clip_path(node, &context),
            "marker" => self.marker(node, &context),
            "pattern" => self.pattern(node, &context),
            "linearGradient" | "radialGradient" => self.gradient(node),
            "rect" | "circle" | "ellipse" | "line" | "polyline" | "polygon" | "path" => {
                self.shape(node, &context);
            }
            "text" => self.text(node, &context),
            _ => {
                let label = self.label(node);
                self.problem(
                    Kind::Unsupported,
                    format!("{label}: an element the figure check can't measure"),
                );
            }
        }
    }

    /// Fails each attribute the check neither reads nor knows to be harmless.
    fn attributes(&mut self, node: Node<'_, '_>) {
        for attribute in node.attributes() {
            let name = attribute.name();
            let known = match attribute.namespace() {
                None => ATTRIBUTES.contains(&name) || name.starts_with("aria-"),
                Some(XML) => matches!(name, "space" | "lang"),
                Some(_) => false,
            };
            if !known {
                let label = self.label(node);
                self.problem(
                    Kind::Unsupported,
                    format!("{label}: attribute `{name}`, which the figure check doesn't read"),
                );
            }
        }
        if let Some(style) = node.attribute("style") {
            for declaration in style.split(';').filter(|d| !d.trim().is_empty()) {
                let property = declaration.split(':').next().unwrap_or_default().trim();
                if !STYLE_PROPERTIES.contains(&property) {
                    let label = self.label(node);
                    self.problem(
                        Kind::Unsupported,
                        format!(
                            "{label}: style sets `{property}`, which the check reads only as \
                             an attribute"
                        ),
                    );
                }
            }
        }
        if let Some(space) = node.attribute((XML, "space"))
            && !matches!(space, "default" | "preserve")
        {
            let label = self.label(node);
            self.problem(
                Kind::Malformed,
                format!("{label}: xml:space `{space}` is neither default nor preserve"),
            );
        }
    }

    /// The style `node` draws in, from its parent's `outer`.
    fn style(&mut self, node: Node<'_, '_>, outer: &Style) -> Style {
        let mut style = outer.clone();
        if let Some(stroke) = node.attribute("stroke") {
            style.stroked = stroke.trim() != "none";
        }
        if node.has_attribute("stroke-width")
            && let Some(width) = self.size(node, "stroke-width", None)
        {
            style.stroke_width = width;
        }
        if let Some(join) = node.attribute("stroke-linejoin") {
            style.join = match join.trim() {
                "miter" => Join::Miter,
                "round" => Join::Round,
                "bevel" => Join::Bevel,
                other => {
                    self.unsupported_value(node, "stroke-linejoin", other);
                    Join::Miter
                }
            };
        }
        if let Some(cap) = node.attribute("stroke-linecap") {
            style.cap = match cap.trim() {
                "butt" => Cap::Butt,
                "round" => Cap::Round,
                "square" => Cap::Square,
                other => {
                    self.unsupported_value(node, "stroke-linecap", other);
                    Cap::Square
                }
            };
        }
        if node.has_attribute("stroke-miterlimit") {
            match self.length(node, "stroke-miterlimit", None) {
                Some(limit) if limit >= 1.0 => style.miter_limit = limit,
                Some(_) => {
                    let label = self.label(node);
                    let text = node.attribute("stroke-miterlimit").unwrap_or_default();
                    self.problem(
                        Kind::Malformed,
                        format!("{label}: stroke-miterlimit `{text}` is below 1"),
                    );
                }
                None => {}
            }
        }
        if let Some(dashes) = node.attribute("stroke-dasharray") {
            style.dashed = dashes.trim() != "none";
        }
        for (i, name) in ["marker-start", "marker-mid", "marker-end"]
            .into_iter()
            .enumerate()
        {
            if let Some(value) = node.attribute(name) {
                style.markers[i] = match value.trim() {
                    "none" => None,
                    reference => self.reference(node, name, reference).map(str::to_owned),
                };
            }
        }
        if let Some(family) = node.attribute("font-family") {
            style.family = Some(family.to_owned());
        }
        if node.has_attribute("font-size")
            && let Some(size) = self.size(node, "font-size", None)
        {
            style.font_size = size;
        }
        if let Some(weight) = node.attribute("font-weight") {
            style.bold = match weight.trim() {
                "normal" => false,
                "bold" => true,
                number => match number.parse::<u16>() {
                    Ok(w) if (1..=1000).contains(&w) => w >= 500,
                    _ => {
                        self.unsupported_value(node, "font-weight", number);
                        true
                    }
                },
            };
        }
        if let Some(anchor) = node.attribute("text-anchor") {
            style.anchor = match anchor.trim() {
                "start" => Anchor::Start,
                "middle" => Anchor::Middle,
                "end" => Anchor::End,
                other => {
                    self.unsupported_value(node, "text-anchor", other);
                    Anchor::Start
                }
            };
        }
        if node.has_attribute("dominant-baseline") && node.tag_name().name() != "text" {
            // Inherited in SVG 2 and not in SVG 1.1: read only where it is set.
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: dominant-baseline set on a container; set it on the <text>"),
            );
        }
        style
    }

    /// Records an attribute value the check doesn't know.
    fn unsupported_value(&mut self, node: Node<'_, '_>, name: &str, value: &str) {
        let label = self.label(node);
        self.problem(
            Kind::Unsupported,
            format!("{label}: {name} `{value}`, which the figure check can't measure"),
        );
    }

    /// The id a `url(#id)` reference names.
    fn reference<'v>(&mut self, node: Node<'_, '_>, name: &str, value: &'v str) -> Option<&'v str> {
        let id = value
            .trim()
            .strip_prefix("url(")
            .and_then(|rest| rest.strip_suffix(')'))
            .map(|inner| inner.trim().trim_matches(['"', '\'']))
            .and_then(|inner| inner.strip_prefix('#'));
        if id.is_none() {
            let label = self.label(node);
            self.problem(
                Kind::Malformed,
                format!("{label}: {name} `{value}` is not a url(#id) reference"),
            );
        }
        id
    }

    /// A length attribute in px, or `default` where it isn't set.
    fn length(&mut self, node: Node<'_, '_>, name: &str, default: Option<f64>) -> Option<f64> {
        let Some(text) = node.attribute(name) else {
            if default.is_none() {
                let label = self.label(node);
                self.problem(Kind::Malformed, format!("{label}: no {name}"));
            }
            return default;
        };
        let trimmed = text.trim();
        let number = trimmed.strip_suffix("px").unwrap_or(trimmed);
        let mut scanner = Scanner::new(number);
        match scanner.number() {
            Some(value) if scanner.done() => Some(value),
            _ => {
                let label = self.label(node);
                let kind = if number.ends_with(|c: char| c.is_ascii_alphabetic() || c == '%') {
                    Kind::Unsupported
                } else {
                    Kind::Malformed
                };
                self.problem(
                    kind,
                    format!("{label}: {name} `{text}` is not a number of px"),
                );
                None
            }
        }
    }

    /// A length that can't be negative, as [`Self::length`] reads it. A negative one fails as
    /// malformed: a browser takes it as invalid and draws in the inherited or default value, or
    /// draws nothing, never the number written.
    fn size(&mut self, node: Node<'_, '_>, name: &str, default: Option<f64>) -> Option<f64> {
        let value = self.length(node, name, default)?;
        if value < 0.0 {
            let label = self.label(node);
            let text = node.attribute(name).unwrap_or_default();
            self.problem(
                Kind::Malformed,
                format!("{label}: {name} `{text}` is negative"),
            );
            return None;
        }
        Some(value)
    }

    /// The map a transform attribute names; the identity where it is absent or malformed.
    fn transform(&mut self, node: Node<'_, '_>, name: &str) -> Matrix {
        let Some(text) = node.attribute(name) else {
            return Matrix::IDENTITY;
        };
        match transform(text) {
            Some(matrix) => matrix,
            None => {
                let label = self.label(node);
                self.problem(
                    Kind::Malformed,
                    format!("{label}: {name} `{text}` doesn't parse"),
                );
                Matrix::IDENTITY
            }
        }
    }

    /// Adds the frame of `node`'s `clip-path` to `context`, and checks the clip itself lies
    /// inside the frames around it.
    fn clip(&mut self, node: Node<'a, 'input>, reference: &str, context: &mut Context) {
        if context.place == Place::Inside {
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: a clip-path inside a marker or pattern"),
            );
            return;
        }
        if reference.trim() == "none" {
            return;
        }
        let Some(id) = self.reference(node, "clip-path", reference) else {
            return;
        };
        let Some(clip) = self.ids.get(id).copied() else {
            let label = self.label(node);
            self.problem(
                Kind::Malformed,
                format!("{label}: clip-path names #{id}, which isn't in the figure"),
            );
            return;
        };
        let Some((matrix, rect)) = self.clip_rect(clip) else {
            return;
        };
        let to_base = context.ctm.then(matrix);
        let Some(from_base) = to_base.inverse() else {
            let label = self.label(clip);
            self.problem(
                Kind::Malformed,
                format!("{label}: a transform that collapses the clip"),
            );
            return;
        };
        if context.place != Place::Hidden {
            let what = format!("{} (the clip of {})", self.label(clip), self.label(node));
            let corners = rect.corners().map(|p| matrix.apply(p));
            self.place(&what, &corners, context);
        }
        context.frames.push(Frame {
            name: format!("its clip, {} ({rect})", self.label(clip)),
            kind: Kind::Clipped,
            rect,
            from_base,
        });
    }

    /// A `clipPath` of one rectangle: the map from the rectangle's coordinates to the user space
    /// of the element it clips, and the rectangle.
    fn clip_rect(&mut self, clip: Node<'a, 'input>) -> Option<(Matrix, Bounds)> {
        let label = self.label(clip);
        if clip.tag_name().name() != "clipPath" {
            self.problem(Kind::Malformed, format!("{label}: not a clipPath"));
            return None;
        }
        if clip
            .attribute("clipPathUnits")
            .is_some_and(|units| units.trim() != "userSpaceOnUse")
        {
            self.problem(
                Kind::Unsupported,
                format!("{label}: clipPathUnits other than userSpaceOnUse"),
            );
            return None;
        }
        let shapes: Vec<Node<'_, '_>> = clip
            .children()
            .filter(|c| c.is_element() && !matches!(c.tag_name().name(), "title" | "desc"))
            .collect();
        let [rect] = shapes.as_slice() else {
            self.problem(
                Kind::Unsupported,
                format!("{label}: a clipPath of anything but one <rect>"),
            );
            return None;
        };
        if rect.tag_name().name() != "rect" || clip.has_attribute("clip-path") {
            self.problem(
                Kind::Unsupported,
                format!("{label}: a clipPath of anything but one <rect>"),
            );
            return None;
        }
        if rect.has_attribute("clip-path") {
            self.problem(
                Kind::Unsupported,
                format!("{label}: a clip-path on the rectangle of a clipPath, which cuts the clip"),
            );
            return None;
        }
        let matrix = self
            .transform(clip, "transform")
            .then(self.transform(*rect, "transform"));
        let bounds = self.rect_box(*rect)?;
        Some((matrix, bounds))
    }

    /// A `clipPath` where it is defined: its content isn't drawn, but must be what a clip may
    /// hold.
    fn clip_path(&mut self, node: Node<'a, 'input>, context: &Context) {
        let hidden = Context {
            place: Place::Hidden,
            ..context.clone()
        };
        self.children(node, &hidden);
    }

    /// A gradient: only `stop`s.
    fn gradient(&mut self, node: Node<'_, '_>) {
        for child in node.children().filter(Node::is_element) {
            if child.tag_name().name() == "stop" {
                self.attributes(child);
            } else if !matches!(child.tag_name().name(), "title" | "desc") {
                let label = self.label(child);
                self.problem(
                    Kind::Unsupported,
                    format!("{label}: a gradient holds only <stop>s"),
                );
            }
        }
    }

    /// A `marker`: its content must fit its viewport, which clips it.
    fn marker(&mut self, node: Node<'a, 'input>, context: &Context) {
        if context.place == Place::Inside {
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: a marker inside a marker or pattern"),
            );
            return;
        }
        let Some(viewport) = self.marker_viewport(node) else {
            return;
        };
        let label = self.label(node);
        let inside = Context {
            ctm: Matrix::IDENTITY,
            style: context.style.clone(),
            frames: vec![Frame {
                name: format!("the viewport of {label} ({})", viewport.rect),
                kind: Kind::Clipped,
                rect: viewport.rect,
                from_base: Matrix::IDENTITY,
            }],
            place: Place::Inside,
        };
        self.children(node, &inside);
    }

    /// A marker's viewport in its content's coordinates, and how far it reaches from its
    /// reference point, in marker units.
    fn marker_viewport(&mut self, node: Node<'_, '_>) -> Option<Viewport> {
        let width = self.size(node, "markerWidth", Some(3.0))?;
        let height = self.size(node, "markerHeight", Some(3.0))?;
        let reference = (
            self.length(node, "refX", Some(0.0))?,
            self.length(node, "refY", Some(0.0))?,
        );
        if node
            .attribute("markerUnits")
            .is_some_and(|units| !matches!(units.trim(), "strokeWidth" | "userSpaceOnUse"))
        {
            let units = node.attribute("markerUnits").unwrap_or_default();
            self.unsupported_value(node, "markerUnits", units);
            return None;
        }
        // The viewBox maps into the viewport as preserveAspectRatio's default does: scaled to
        // fit (meet), centered.
        let (scale, offset, view) = match node.attribute("viewBox") {
            Some(text) => {
                let view = self.view_box(node, text)?;
                let (vw, vh) = (view.x1 - view.x0, view.y1 - view.y0);
                let scale = (width / vw).min(height / vh);
                let offset = (
                    (width - vw * scale) / 2.0 - view.x0 * scale,
                    (height - vh * scale) / 2.0 - view.y0 * scale,
                );
                (scale, offset, Some(view))
            }
            None => (1.0, (0.0, 0.0), None),
        };
        if !(scale.is_finite() && scale > 0.0 && width >= 0.0 && height >= 0.0) {
            let label = self.label(node);
            self.problem(Kind::Malformed, format!("{label}: an empty viewport"));
            return None;
        }
        // The viewport, in the content's coordinates.
        let rect = match view {
            Some(_) => Bounds {
                x0: -offset.0 / scale,
                y0: -offset.1 / scale,
                x1: (width - offset.0) / scale,
                y1: (height - offset.1) / scale,
            },
            None => Bounds {
                x0: 0.0,
                y0: 0.0,
                x1: width,
                y1: height,
            },
        };
        let at = (
            reference.0 * scale + offset.0,
            reference.1 * scale + offset.1,
        );
        let reach = [(0.0, 0.0), (width, 0.0), (0.0, height), (width, height)]
            .iter()
            .map(|corner| (corner.0 - at.0).hypot(corner.1 - at.1))
            .fold(0.0, f64::max);
        let per_stroke = node
            .attribute("markerUnits")
            .is_none_or(|units| units.trim() == "strokeWidth");
        Some(Viewport {
            rect,
            reach,
            per_stroke,
        })
    }

    /// A `pattern`: its content must fit its tile, which clips it.
    fn pattern(&mut self, node: Node<'a, 'input>, context: &Context) {
        let label = self.label(node);
        if context.place == Place::Inside {
            self.problem(
                Kind::Unsupported,
                format!("{label}: a pattern inside a marker or pattern"),
            );
            return;
        }
        if node.attribute("patternUnits").map(str::trim) != Some("userSpaceOnUse")
            || node
                .attribute("patternContentUnits")
                .is_some_and(|units| units.trim() != "userSpaceOnUse")
            || node.has_attribute("viewBox")
        {
            self.problem(
                Kind::Unsupported,
                format!(
                    "{label}: a pattern the check measures has patternUnits=\"userSpaceOnUse\", \
                     content in user space and no viewBox"
                ),
            );
            return;
        }
        let _ = self.transform(node, "patternTransform");
        let (Some(x), Some(y), Some(w), Some(h)) = (
            self.length(node, "x", Some(0.0)),
            self.length(node, "y", Some(0.0)),
            self.size(node, "width", None),
            self.size(node, "height", None),
        ) else {
            return;
        };
        let rect = Bounds {
            x0: x,
            y0: y,
            x1: x + w,
            y1: y + h,
        };
        let inside = Context {
            ctm: Matrix::IDENTITY,
            style: context.style.clone(),
            frames: vec![Frame {
                name: format!("the tile of {label} ({rect})"),
                kind: Kind::Clipped,
                rect,
                from_base: Matrix::IDENTITY,
            }],
            place: Place::Inside,
        };
        self.children(node, &inside);
    }

    /// A `rect`'s box, unstroked.
    fn rect_box(&mut self, node: Node<'_, '_>) -> Option<Bounds> {
        let x = self.length(node, "x", Some(0.0))?;
        let y = self.length(node, "y", Some(0.0))?;
        let w = self.size(node, "width", None)?;
        let h = self.size(node, "height", None)?;
        Some(Bounds {
            x0: x,
            y0: y,
            x1: x + w,
            y1: y + h,
        })
    }

    /// A shape: where it draws, stroke and markers with it, in its frames.
    fn shape(&mut self, node: Node<'a, 'input>, context: &Context) {
        if node.children().any(|c| {
            c.is_element() && !matches!(c.tag_name().name(), "title" | "desc" | "metadata")
        }) {
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: a shape holding other elements"),
            );
        }
        let style = &context.style;
        let half = if style.stroked {
            style.stroke_width / 2.0
        } else {
            0.0
        };
        let name = node.tag_name().name();
        let mut points = Vec::new();
        let mut subpaths = Vec::new();
        match name {
            "rect" => {
                let Some(b) = self.rect_box(node) else {
                    return;
                };
                if b.x1 == b.x0 || b.y1 == b.y0 {
                    return;
                }
                // Every corner turns a right angle: a miter reaches half the width out along each
                // side, as a round or beveled join stays within.
                let grown = Bounds {
                    x0: b.x0 - half,
                    y0: b.y0 - half,
                    x1: b.x1 + half,
                    y1: b.y1 + half,
                };
                points.extend(grown.corners());
            }
            "circle" | "ellipse" => {
                let (Some(cx), Some(cy)) = (
                    self.length(node, "cx", Some(0.0)),
                    self.length(node, "cy", Some(0.0)),
                ) else {
                    return;
                };
                let radii = if name == "circle" {
                    self.size(node, "r", None).map(|r| (r, r))
                } else {
                    match (self.size(node, "rx", None), self.size(node, "ry", None)) {
                        (Some(rx), Some(ry)) => Some((rx, ry)),
                        _ => None,
                    }
                };
                let Some((rx, ry)) = radii else {
                    return;
                };
                points.extend(
                    Bounds {
                        x0: cx - rx - half,
                        y0: cy - ry - half,
                        x1: cx + rx + half,
                        y1: cy + ry + half,
                    }
                    .corners(),
                );
            }
            "line" => {
                let ends = ["x1", "y1", "x2", "y2"].map(|a| self.length(node, a, Some(0.0)));
                let [Some(x1), Some(y1), Some(x2), Some(y2)] = ends else {
                    return;
                };
                subpaths.push(Subpath {
                    start: (x1, y1),
                    segments: vec![Segment::Line((x1, y1), (x2, y2))],
                    closed: false,
                });
            }
            "polyline" | "polygon" => {
                let text = node.attribute("points").unwrap_or_default();
                let Some(values) = numbers(text).filter(|v| v.len() % 2 == 0) else {
                    let label = self.label(node);
                    self.problem(
                        Kind::Malformed,
                        format!("{label}: points `{text}` is not pairs of numbers"),
                    );
                    return;
                };
                let vertices: Vec<P> = values.chunks(2).map(|p| (p[0], p[1])).collect();
                if let Some(&start) = vertices.first() {
                    let mut segments: Vec<Segment> = vertices
                        .windows(2)
                        .map(|w| Segment::Line(w[0], w[1]))
                        .collect();
                    let closed = name == "polygon";
                    if closed && let Some(&last) = vertices.last() {
                        segments.push(Segment::Line(last, start));
                    }
                    subpaths.push(Subpath {
                        start,
                        segments,
                        closed,
                    });
                }
            }
            _ => {
                let text = node.attribute("d").unwrap_or_default();
                match path(text) {
                    Ok(parsed) => subpaths = parsed,
                    Err(why) => {
                        let label = self.label(node);
                        self.problem(Kind::Malformed, format!("{label}: d doesn't parse: {why}"));
                        return;
                    }
                }
            }
        }
        for subpath in &subpaths {
            outline(subpath, style, half, &mut points);
        }
        if !subpaths.is_empty() {
            self.markers(node, &subpaths, context, &mut points);
        }
        if context.place != Place::Hidden {
            let label = self.label(node);
            self.place(&label, &points, context);
        }
    }

    /// Adds to `points` the reach of the markers on a path's vertices.
    fn markers(
        &mut self,
        node: Node<'_, '_>,
        subpaths: &[Subpath],
        context: &Context,
        points: &mut Vec<P>,
    ) {
        let markers = context.style.markers.clone();
        if markers.iter().all(Option::is_none) {
            return;
        }
        if context.place == Place::Inside {
            let label = self.label(node);
            self.problem(
                Kind::Unsupported,
                format!("{label}: markers inside a marker or pattern"),
            );
            return;
        }
        let vertices: Vec<P> = subpaths
            .iter()
            .flat_map(|s| std::iter::once(s.start).chain(s.segments.iter().map(Segment::end)))
            .collect();
        let last = vertices.len().saturating_sub(1);
        for (i, id) in markers.iter().enumerate() {
            let Some(id) = id else {
                continue;
            };
            let marker = self.ids.get(id.as_str()).copied();
            let Some(marker) = marker.filter(|m| m.tag_name().name() == "marker") else {
                let label = self.label(node);
                self.problem(
                    Kind::Malformed,
                    format!("{label}: its marker #{id} isn't a <marker> in the figure"),
                );
                continue;
            };
            let Some(viewport) = self.marker_viewport(marker) else {
                continue;
            };
            let reach = if viewport.per_stroke {
                viewport.reach * context.style.stroke_width
            } else {
                viewport.reach
            };
            let at: &[P] = match i {
                0 => &vertices[..vertices.len().min(1)],
                1 if vertices.len() > 2 => &vertices[1..last],
                1 => &[],
                _ => &vertices[last..],
            };
            for &vertex in at {
                points.extend(square(vertex, reach));
            }
        }
    }

    /// Checks `points`, in the user space of an element in `context`, against each frame.
    fn place(&mut self, what: &str, points: &[P], context: &Context) {
        for frame in &context.frames {
            let map = frame.from_base.then(context.ctm);
            let Some(reach) = Bounds::of(points.iter().map(|&p| map.apply(p))) else {
                continue;
            };
            if !reach.within(&frame.rect) {
                self.problem(
                    frame.kind,
                    format!("{what} reaches {reach}, outside {}", frame.name),
                );
            }
        }
    }

    /// A line of `text`: in its frames, and recorded for the overlap check.
    fn text(&mut self, node: Node<'a, 'input>, context: &Context) {
        let label = self.label(node);
        if context.place == Place::Inside {
            self.problem(
                Kind::Unsupported,
                format!("{label}: text inside a marker or pattern"),
            );
            return;
        }
        let mut content = String::new();
        for child in node.children() {
            if child.is_text() {
                content.push_str(child.text().unwrap_or_default());
            } else if child.is_element()
                && !matches!(child.tag_name().name(), "title" | "desc" | "metadata")
            {
                let inner = self.label(child);
                self.problem(
                    Kind::Unsupported,
                    format!("{inner}: an element inside <text>, which the check can't measure"),
                );
                return;
            }
        }
        let preserve = node
            .ancestors()
            .find_map(|n| n.attribute((XML, "space")))
            .is_some_and(|space| space == "preserve");
        let content = collapse(&content, preserve);
        let (Some(x), Some(y)) = (
            self.length(node, "x", Some(0.0)),
            self.length(node, "y", Some(0.0)),
        ) else {
            return;
        };
        let low = match node.attribute("dominant-baseline").map(str::trim) {
            None | Some("auto" | "alphabetic") => 0.0,
            // The baseline sits below `y`, by less than an em: half the x-height for middle,
            // half the em box for central, about 0.8 em for hanging.
            Some("middle" | "central" | "hanging") => 1.0,
            Some(other) => {
                self.unsupported_value(node, "dominant-baseline", other);
                return;
            }
        };
        if content.is_empty() {
            return;
        }
        let Some(ink) = self.ink(&label, &content, &context.style) else {
            return;
        };
        let size = context.style.font_size;
        let width = ink.advance * size;
        let left = match context.style.anchor {
            Anchor::Start => x,
            Anchor::Middle => x - width / 2.0,
            Anchor::End => x - width,
        };
        // A stroke on text widens each glyph by half its width, more at a sharp miter.
        let stroke = if context.style.stroked {
            let half = context.style.stroke_width / 2.0;
            match context.style.join {
                Join::Miter => half * context.style.miter_limit,
                Join::Round | Join::Bevel => half,
            }
        } else {
            0.0
        };
        let local = Bounds {
            x0: left - ink.left * size - stroke,
            y0: y - ink.top * size - stroke,
            x1: left + width + ink.right * size + stroke,
            y1: y + low * size + ink.bottom * size + stroke,
        };
        let what = format!("{label} \"{content}\"");
        if context.place == Place::Hidden {
            return;
        }
        self.place(&what, &local.corners(), context);
        if context.place == Place::Drawn
            && let Some(root) = Bounds::of(local.corners().map(|p| context.ctm.apply(p)))
        {
            self.texts.push((what, root));
        }
    }

    /// The ink of `content` in `style`, in ems: its advance, and how far its ink reaches past
    /// its start and end, above and below its baseline; none if the table lacks a character.
    fn ink(&mut self, label: &str, content: &str, style: &Style) -> Option<Ink> {
        let tables = self.tables(label, style)?;
        let mut ink = Ink::default();
        let mut missing = Vec::new();
        for c in content.chars() {
            // The largest of each table's row; none if any table lacks the character.
            let rows: Option<Vec<[u16; 5]>> = tables
                .iter()
                .map(|table| {
                    let i = table.binary_search_by(|(k, _)| k.cmp(&c)).ok()?;
                    Some(table[i].1)
                })
                .collect();
            let found = rows.and_then(|rows| {
                rows.into_iter()
                    .reduce(|a, b| std::array::from_fn(|i| a[i].max(b[i])))
            });
            let Some([advance, left, right, top, bottom]) = found else {
                missing.push(format!("U+{:04X} `{c}`", u32::from(c)));
                continue;
            };
            let em = |v: u16| f64::from(v) / 1000.0;
            ink.advance += em(advance);
            ink.left = ink.left.max(em(left));
            ink.right = ink.right.max(em(right));
            ink.top = ink.top.max(em(top));
            ink.bottom = ink.bottom.max(em(bottom));
        }
        if missing.is_empty() {
            Some(ink)
        } else {
            missing.dedup();
            self.problem(
                Kind::Character,
                format!(
                    "{label} \"{content}\": {} not in the glyph table (scripts/glyph-widths.py)",
                    missing.join(", ")
                ),
            );
            None
        }
    }

    /// The glyph tables a style's text may draw from: monospace where every family in its list
    /// is, proportional where every one is, both where they mix.
    fn tables(&mut self, label: &str, style: &Style) -> Option<Vec<Table>> {
        let (mut proportional, mut monospace) = (false, false);
        let family = style.family.as_deref().unwrap_or("serif");
        for name in family.split(',') {
            let name = name.trim().trim_matches(['"', '\'']).trim();
            let is = |list: &[&str]| list.iter().any(|f| f.eq_ignore_ascii_case(name));
            if is(glyphs::PROPORTIONAL_FAMILIES)
                || is(PROPORTIONAL_ALIASES)
                || is(PROPORTIONAL_GENERICS)
            {
                proportional = true;
            } else if is(glyphs::MONOSPACE_FAMILIES)
                || is(MONOSPACE_ALIASES)
                || is(MONOSPACE_GENERICS)
            {
                monospace = true;
            } else {
                self.problem(
                    Kind::Unsupported,
                    format!(
                        "{label}: font family `{name}` isn't in the glyph table \
                         (scripts/glyph-widths.py measures the families it may draw in)"
                    ),
                );
                return None;
            }
        }
        let mut tables = Vec::new();
        if proportional {
            tables.push(if style.bold {
                glyphs::PROPORTIONAL_BOLD
            } else {
                glyphs::PROPORTIONAL
            });
        }
        if monospace {
            tables.push(if style.bold {
                glyphs::MONOSPACE_BOLD
            } else {
                glyphs::MONOSPACE
            });
        }
        Some(tables)
    }

    /// Fails each two lines of text whose boxes overlap.
    fn overlaps(&mut self) {
        let mut found = Vec::new();
        for (i, (a, box_a)) in self.texts.iter().enumerate() {
            for (b, box_b) in &self.texts[i + 1..] {
                let width = box_a.x1.min(box_b.x1) - box_a.x0.max(box_b.x0);
                let height = box_a.y1.min(box_b.y1) - box_a.y0.max(box_b.y0);
                if width > OVERLAP_PX && height > OVERLAP_PX {
                    found.push(format!(
                        "{a} ({box_a}) overlaps {b} ({box_b}) by {width:.2} × {height:.2} px"
                    ));
                }
            }
        }
        for message in found {
            self.problem(Kind::Overlap, message);
        }
    }
}

/// A marker's viewport: its rectangle in its content's coordinates, how far it reaches from its
/// reference point in marker units, and whether those are stroke widths.
#[derive(Debug, Clone, Copy)]
struct Viewport {
    rect: Bounds,
    reach: f64,
    per_stroke: bool,
}

/// How far a line of text's ink reaches, in ems.
#[derive(Debug, Clone, Copy, Default)]
struct Ink {
    advance: f64,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

/// Text as a browser lays it out: with `xml:space="preserve"`, each newline and tab a space; by
/// default, as CSS's `white-space: normal` (SVG 2), each newline and tab a space, runs of spaces
/// made one, and the ends trimmed. (SVG 1.1 dropped newlines instead; browsers draw a space,
/// which is wider.)
fn collapse(text: &str, preserve: bool) -> String {
    if preserve {
        return text.replace(['\n', '\r', '\t'], " ");
    }
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\n' | '\r' | '\t' | ' ' => {
                if !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            c => out.push(c),
        }
    }
    out.trim_matches(' ').to_owned()
}

/// Adds to `points` what holds a subpath's stroke of half-width `half`: each straight segment's
/// stroke exactly, a square of `half` about each control point of a curve, and the reach of its
/// joins and caps.
fn outline(subpath: &Subpath, style: &Style, half: f64, points: &mut Vec<P>) {
    let segments = &subpath.segments;
    let square_cap = style.cap == Cap::Square;
    for segment in segments {
        match *segment {
            Segment::Line(a, b) => match unit(sub(b, a)) {
                Some((tx, ty)) => {
                    let n = (-ty * half, tx * half);
                    for p in [a, b] {
                        points.push((p.0 + n.0, p.1 + n.1));
                        points.push((p.0 - n.0, p.1 - n.1));
                    }
                }
                // A segment with no length draws a dot under a round or square cap.
                None if style.cap != Cap::Butt => points.extend(square(a, half)),
                None => points.push(a),
            },
            _ => {
                // A square cap on a dash ending mid-curve reaches √2 · half from the curve.
                let reach = if square_cap && style.dashed {
                    half * std::f64::consts::SQRT_2
                } else {
                    half
                };
                for p in segment.hull() {
                    points.extend(square(p, reach));
                }
            }
        }
    }
    if half == 0.0 {
        if segments.is_empty() {
            points.push(subpath.start);
        }
        return;
    }
    // Caps: at the open ends, or at every segment's ends under a dash pattern.
    let mut ends: Vec<(P, Option<P>)> = Vec::new();
    if style.dashed {
        for segment in segments {
            let backward = segment.start_direction().map(|(x, y)| (-x, -y));
            ends.push((segment.start(), backward));
            ends.push((segment.end(), segment.end_direction()));
        }
    } else if !subpath.closed
        && let (Some(first), Some(last)) = (segments.first(), segments.last())
    {
        let backward = first.start_direction().map(|(x, y)| (-x, -y));
        ends.push((first.start(), backward));
        ends.push((last.end(), last.end_direction()));
    }
    for (p, outward) in ends {
        match (style.cap, outward) {
            (Cap::Butt, _) => {}
            (Cap::Round, _) => points.extend(square(p, half)),
            (Cap::Square, Some((tx, ty))) => {
                let (nx, ny) = (-ty * half, tx * half);
                let tip = (p.0 + tx * half, p.1 + ty * half);
                points.push((tip.0 + nx, tip.1 + ny));
                points.push((tip.0 - nx, tip.1 - ny));
            }
            (Cap::Square, None) => points.extend(square(p, half * std::f64::consts::SQRT_2)),
        }
    }
    // Joins: between each two segments, and from the last to the first of a closed subpath. A
    // segment with no length, such as a closepath back to where the path already is, joins
    // nothing: its neighbours join each other.
    let drawn: Vec<&Segment> = segments
        .iter()
        .filter(|s| s.start_direction().is_some())
        .collect();
    let mut joins: Vec<(&Segment, &Segment)> = drawn.windows(2).map(|w| (w[0], w[1])).collect();
    if subpath.closed
        && drawn.len() > 1
        && let (Some(last), Some(first)) = (drawn.last(), drawn.first())
    {
        joins.push((last, first));
    }
    for (incoming, outgoing) in joins {
        let vertex = incoming.end();
        match style.join {
            Join::Round => points.extend(square(vertex, half)),
            // A bevel's corners are the two segments' own.
            Join::Bevel => {}
            Join::Miter => {
                match (incoming.end_direction(), outgoing.start_direction()) {
                    (Some(a), Some(b)) => {
                        // The miter's length over the stroke's width is 1/sin(θ/2), θ the angle
                        // between the segments: sin(θ/2) = √((1 + a·b)/2) for unit directions.
                        let cosine = (a.0 * b.0 + a.1 * b.1).clamp(-1.0, 1.0);
                        let sine = ((1.0 + cosine) / 2.0).sqrt();
                        let ratio = if sine > 0.0 {
                            1.0 / sine
                        } else {
                            f64::INFINITY
                        };
                        if ratio <= style.miter_limit {
                            points.extend(square(vertex, half * ratio));
                        }
                    }
                    // A direction the check can't take: as far as the limit lets a miter go.
                    _ => points.extend(square(vertex, half * style.miter_limit)),
                }
            }
        }
    }
}

/// Numbers in a list, separated by spaces or commas; none if anything else is there.
fn numbers(text: &str) -> Option<Vec<f64>> {
    let mut scanner = Scanner::new(text);
    let mut out = Vec::new();
    while !scanner.done() {
        out.push(scanner.number()?);
    }
    Some(out)
}

/// A transform list's map.
fn transform(text: &str) -> Option<Matrix> {
    let mut matrix = Matrix::IDENTITY;
    let mut rest = text.trim();
    while !rest.is_empty() {
        let open = rest.find('(')?;
        let close = rest.find(')')?;
        if close < open {
            return None;
        }
        let name = rest[..open].trim();
        let args = numbers(&rest[open + 1..close])?;
        let step = match (name, args.as_slice()) {
            ("matrix", &[a, b, c, d, e, f]) => Matrix([a, b, c, d, e, f]),
            ("translate", &[x]) => Matrix::translate(x, 0.0),
            ("translate", &[x, y]) => Matrix::translate(x, y),
            ("scale", &[s]) => Matrix([s, 0.0, 0.0, s, 0.0, 0.0]),
            ("scale", &[sx, sy]) => Matrix([sx, 0.0, 0.0, sy, 0.0, 0.0]),
            ("rotate", &[angle]) => rotation(angle),
            ("rotate", &[angle, cx, cy]) => Matrix::translate(cx, cy)
                .then(rotation(angle))
                .then(Matrix::translate(-cx, -cy)),
            ("skewX", &[angle]) => Matrix([1.0, 0.0, angle.to_radians().tan(), 1.0, 0.0, 0.0]),
            ("skewY", &[angle]) => Matrix([1.0, angle.to_radians().tan(), 0.0, 1.0, 0.0, 0.0]),
            _ => return None,
        };
        matrix = matrix.then(step);
        rest = rest[close + 1..].trim_start_matches(|c: char| c.is_whitespace() || c == ',');
    }
    matrix.0.iter().all(|v| v.is_finite()).then_some(matrix)
}

/// A rotation by `degrees`, clockwise on screen.
fn rotation(degrees: f64) -> Matrix {
    let (sin, cos) = degrees.to_radians().sin_cos();
    Matrix([cos, sin, -sin, cos, 0.0, 0.0])
}

/// A reader of SVG's number grammar, as path data and lists write it: numbers may run together
/// where a sign or a second point separates them (`1-2`, `.5.5`).
struct Scanner<'s> {
    text: &'s [u8],
    at: usize,
}

impl<'s> Scanner<'s> {
    fn new(text: &'s str) -> Self {
        Self {
            text: text.as_bytes(),
            at: 0,
        }
    }

    /// Skips spaces and at most one comma.
    fn separators(&mut self) {
        let mut comma = false;
        while let Some(&c) = self.text.get(self.at) {
            if c.is_ascii_whitespace() {
                self.at += 1;
            } else if c == b',' && !comma {
                comma = true;
                self.at += 1;
            } else {
                break;
            }
        }
    }

    /// Whether only separators are left.
    fn done(&mut self) -> bool {
        self.separators();
        self.at >= self.text.len()
    }

    /// The next byte, after separators, without taking it.
    fn peek(&mut self) -> Option<u8> {
        self.separators();
        self.text.get(self.at).copied()
    }

    /// The next number.
    fn number(&mut self) -> Option<f64> {
        self.separators();
        let start = self.at;
        let digits = |s: &mut Self| {
            let from = s.at;
            while s.text.get(s.at).is_some_and(u8::is_ascii_digit) {
                s.at += 1;
            }
            s.at - from
        };
        if matches!(self.text.get(self.at), Some(b'+' | b'-')) {
            self.at += 1;
        }
        let mut count = digits(self);
        if self.text.get(self.at) == Some(&b'.') {
            self.at += 1;
            count += digits(self);
        }
        if count == 0 {
            self.at = start;
            return None;
        }
        if matches!(self.text.get(self.at), Some(b'e' | b'E')) {
            let mark = self.at;
            self.at += 1;
            if matches!(self.text.get(self.at), Some(b'+' | b'-')) {
                self.at += 1;
            }
            if digits(self) == 0 {
                self.at = mark;
            }
        }
        let text = std::str::from_utf8(&self.text[start..self.at]).ok()?;
        text.parse::<f64>().ok().filter(|v| v.is_finite())
    }

    /// An arc's flag: one `0` or `1`.
    fn flag(&mut self) -> Option<bool> {
        let flag = match self.peek()? {
            b'0' => false,
            b'1' => true,
            _ => return None,
        };
        self.at += 1;
        Some(flag)
    }
}

/// A path's `d`, as subpaths of absolute segments.
fn path(d: &str) -> Result<Vec<Subpath>, String> {
    let mut scanner = Scanner::new(d);
    let mut subpaths: Vec<Subpath> = Vec::new();
    let mut current: P = (0.0, 0.0);
    let mut command: Option<u8> = None;
    // The last cubic's or quadratic's second control point, for S and T.
    let mut last_cubic: Option<P> = None;
    let mut last_quad: Option<P> = None;
    // After a closepath, the next drawing command starts a new subpath at the same point.
    let mut reopen = false;
    while let Some(next) = scanner.peek() {
        let letter = if next.is_ascii_alphabetic() {
            scanner.at += 1;
            next
        } else {
            match command {
                // Coordinates after a moveto are linetos.
                Some(b'M') => b'L',
                Some(b'm') => b'l',
                Some(c) if !matches!(c, b'Z' | b'z') => c,
                _ => return Err(format!("a number with no command at byte {}", scanner.at)),
            }
        };
        if subpaths.is_empty() && !matches!(letter, b'M' | b'm') {
            return Err("it doesn't start with a moveto".to_owned());
        }
        let relative = letter.is_ascii_lowercase();
        let origin = if relative { current } else { (0.0, 0.0) };
        let point = |s: &mut Scanner<'_>| -> Result<P, String> {
            let x = s.number();
            let y = s.number();
            match (x, y) {
                (Some(x), Some(y)) => Ok((origin.0 + x, origin.1 + y)),
                _ => Err(format!("a coordinate missing at byte {}", s.at)),
            }
        };
        let mut segment = None;
        let (mut cubic, mut quad) = (None, None);
        match letter.to_ascii_uppercase() {
            b'M' => {
                current = point(&mut scanner)?;
                reopen = false;
                subpaths.push(Subpath {
                    start: current,
                    ..Subpath::default()
                });
            }
            b'L' => segment = Some(Segment::Line(current, point(&mut scanner)?)),
            b'H' | b'V' => {
                let value = scanner
                    .number()
                    .ok_or_else(|| format!("a coordinate missing at byte {}", scanner.at))?;
                let to = if letter.eq_ignore_ascii_case(&b'H') {
                    (if relative { current.0 + value } else { value }, current.1)
                } else {
                    (current.0, if relative { current.1 + value } else { value })
                };
                segment = Some(Segment::Line(current, to));
            }
            b'C' => {
                let (b, c, d) = (
                    point(&mut scanner)?,
                    point(&mut scanner)?,
                    point(&mut scanner)?,
                );
                segment = Some(Segment::Cubic(current, b, c, d));
                cubic = Some(c);
            }
            b'S' => {
                let b =
                    last_cubic.map_or(current, |c| (2.0 * current.0 - c.0, 2.0 * current.1 - c.1));
                let (c, d) = (point(&mut scanner)?, point(&mut scanner)?);
                segment = Some(Segment::Cubic(current, b, c, d));
                cubic = Some(c);
            }
            b'Q' => {
                let (b, c) = (point(&mut scanner)?, point(&mut scanner)?);
                segment = Some(Segment::Quad(current, b, c));
                quad = Some(b);
            }
            b'T' => {
                let b =
                    last_quad.map_or(current, |q| (2.0 * current.0 - q.0, 2.0 * current.1 - q.1));
                let c = point(&mut scanner)?;
                segment = Some(Segment::Quad(current, b, c));
                quad = Some(b);
            }
            b'A' => {
                let missing = || "an arc's parameter missing".to_owned();
                let rx = scanner.number().ok_or_else(missing)?;
                let ry = scanner.number().ok_or_else(missing)?;
                let angle = scanner.number().ok_or_else(missing)?;
                let large = scanner.flag().ok_or_else(missing)?;
                let sweep = scanner.flag().ok_or_else(missing)?;
                let to = point(&mut scanner)?;
                segment = arc(current, to, (rx, ry), angle, large, sweep);
                if segment.is_none() {
                    // An arc to its own start draws nothing.
                    current = to;
                }
            }
            b'Z' => {
                if reopen {
                    // Z right after Z draws nothing more.
                } else if let Some(subpath) = subpaths.last_mut() {
                    subpath.segments.push(Segment::Line(current, subpath.start));
                    subpath.closed = true;
                    current = subpath.start;
                    reopen = true;
                }
            }
            _ => return Err(format!("command `{}`", char::from(letter))),
        }
        if let Some(segment) = segment {
            if reopen {
                subpaths.push(Subpath {
                    start: current,
                    ..Subpath::default()
                });
                reopen = false;
            }
            current = segment.end();
            if let Some(subpath) = subpaths.last_mut() {
                subpath.segments.push(segment);
            }
        }
        last_cubic = cubic;
        last_quad = quad;
        command = Some(letter);
    }
    Ok(subpaths)
}

/// An arc from `from` to `to` as the SVG specification's endpoint-to-center conversion gives it
/// (SVG 1.1, Implementation Notes, F.6.5 and F.6.6): radii scaled up where they can't span the
/// endpoints; a line where a radius is zero; none where the endpoints are one point.
fn arc(from: P, to: P, (rx, ry): P, degrees: f64, large: bool, sweep: bool) -> Option<Segment> {
    if from == to {
        return None;
    }
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    if rx == 0.0 || ry == 0.0 {
        return Some(Segment::Line(from, to));
    }
    let phi = degrees.to_radians();
    let (sin, cos) = phi.sin_cos();
    let (dx, dy) = ((from.0 - to.0) / 2.0, (from.1 - to.1) / 2.0);
    let x1 = cos * dx + sin * dy;
    let y1 = -sin * dx + cos * dy;
    let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let numerator = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
    let denominator = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let mut root = (numerator / denominator).max(0.0).sqrt();
    if large == sweep {
        root = -root;
    }
    let (cx1, cy1) = (root * rx * y1 / ry, -root * ry * x1 / rx);
    let center = (
        cos * cx1 - sin * cy1 + (from.0 + to.0) / 2.0,
        sin * cx1 + cos * cy1 + (from.1 + to.1) / 2.0,
    );
    let angle = |u: P, v: P| {
        let a = (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1);
        if a.is_finite() { a } else { 0.0 }
    };
    let u = ((x1 - cx1) / rx, (y1 - cy1) / ry);
    let v = ((-x1 - cx1) / rx, (-y1 - cy1) / ry);
    let start = angle((1.0, 0.0), u);
    let mut delta = angle(u, v);
    if !sweep && delta > 0.0 {
        delta -= std::f64::consts::TAU;
    } else if sweep && delta < 0.0 {
        delta += std::f64::consts::TAU;
    }
    Some(Segment::Arc {
        from,
        to,
        center,
        radii: (rx, ry),
        phi,
        start,
        sweep: delta,
    })
}
