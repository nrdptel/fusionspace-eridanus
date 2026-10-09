//! `hpr sim --plot`: the flight's altitude, speed and acceleration against time, its events
//! marked, as one SVG figure (decision record ADR-157).
//!
//! The figure is fixed, so every flight's reads the same way: three panels over one time axis,
//! from launch to the flight's end.
//!
//! - **Altitude:** the center of gravity's height above the launch site, m.
//! - **Speed:** the size of the center of gravity's velocity over the ground, and its vertical
//!   speed (up is positive, the thin line), m/s: the quantities of the summary's top speed and
//!   its descent rates.
//! - **Acceleration:** the size of the nose tip's acceleration relative to the launch frame,
//!   the quantity of the summary's peak acceleration, and its vertical part (up is positive, the
//!   thin line), m/s². It is zero on the pad, where the rocket stands still, and so is not what
//!   an accelerometer reads: in a coast it is gravity and drag, about −10 m/s² and more upward.
//!
//! Each panel's left scale is SI and its right scale the US unit a flyer in the United States
//! reads (feet, feet per second, g), at round values of its own, as the command line puts US units
//! in brackets after the SI (ADR-164 §6, ADR-210). The two scales measure one quantity, so the
//! panel still holds one quantity, as the product system's charts ask.
//!
//! Each group of events at one instant is a numbered balloon above the panels with a dotted line
//! through them, and each event a row of the table below, with its time, altitude and name. Where
//! the rocket falls on its airframe alone after apogee, with no recovery device open within
//! [`crate::sim::BRAKED_WITHIN_S`] of it, the panels are hatched and labeled "not a prediction":
//! HPR Sim's aerodynamics hold only at small angles of attack.
//!
//! The figure follows the FusionSpace product system's chart rules (ADR-175): its colors are the
//! light theme's roles, every series is simulated and so dashed in the predicted color, the
//! ground is a chain line, and a caption says what the lines mean in place of a legend.
//!
//! The SVG is written by hand, as the census badges are, with no plotting dependency. A series
//! keeps, of the samples in each pixel column, the first, the least, the greatest and the last
//! ([`thin`]), so a long flight's file stays small and no peak is cut.

use hpr::hpr_sim::{EventKind, FlightEvent, FlightStep, Observer, Sample, SimError};

use crate::units::{FOOT_M, STANDARD_GRAVITY_M_S2};

/// The figure's width, px.
const WIDTH: f64 = 960.0;
/// The space left of each panel, for its tick labels, px.
const LEFT: f64 = 76.0;
/// The space right of each panel, px, for its US scale's tick labels.
const RIGHT: f64 = 76.0;
/// The space right of the caption's lines and the event table, px.
const MARGIN: f64 = 24.0;
/// A US scale's tick mark, px out from the panel's right edge.
const TICK: f64 = 4.0;
/// A panel's plot height, px.
const PANEL_HEIGHT: f64 = 180.0;
/// From one panel's bottom to the next one's top, its axis title between them, px.
const PANEL_GAP: f64 = 48.0;
/// The title's size, px: the product system's `subtitle` token, 20 / 24 px at weight 600
/// (`product/foundations.md`, *Type*); every other line is its 12 / 16 px `label`.
const TITLE_SIZE: f64 = 20.0;
/// The title's baseline, px from the top: the margin and a 8 px step.
const TITLE_TOP: f64 = MARGIN + 8.0;
/// The subtitle's baseline, px from the top: one `subtitle` line under the title.
const SUBTITLE_TOP: f64 = TITLE_TOP + 24.0;
/// The first caption line's baseline, px from the top: a 24 px step under the subtitle.
const CAPTION_TOP: f64 = SUBTITLE_TOP + 24.0;
/// From the last caption line to the first marker row's center, px.
const CAPTION_GAP: f64 = 24.0;
/// From one marker row to the next, px; also the least distance between two markers in a row.
const MARKER_ROW: f64 = 24.0;
/// A marker's radius, px, so its 12 px number, two digits wide, sits inside it.
const MARKER_RADIUS: f64 = 10.0;
/// From one line of text under the figure, or of the caption, to the next, px: the `label`
/// token's line.
const LIST_LINE: f64 = 16.0;
/// The most characters of a name the figure prints: a design's name, or a device's.
const NAME_LIMIT: usize = 80;
/// The most characters of the title the figure draws, its ellipsis apart, so the 20 px title
/// ends over the panels at 0.6 em a character.
const TITLE_LIMIT: usize = 66;
/// The most characters in a line of the caption or of the note under the table, which then
/// wraps: the figure's width less its margins, at 12 px Cascadia Mono's 7.2 px a character.
const LIST_WIDTH: usize = 118;
/// The most characters in a line of the event table's last column, which then wraps.
const EVENT_WIDTH: usize = 53;
/// The event table's columns: the right edges of the number, the time, the altitude in meters
/// and in feet, and the left edge of the events' names, px from the left margin.
const COLUMNS: [f64; 5] = [24.0, 150.0, 300.0, 440.0, 464.0];
/// The figure's typeface: the product system's face for labels and numbers, then fallbacks of the
/// same fixed width, so the wrapping widths above hold.
const FONT: &str = "'Cascadia Mono', ui-monospace, Menlo, Consolas, monospace";

// The figure's colors: the light theme's roles from the FusionSpace product system's foundations
// (Rev A, `product/foundations.md`, "Semantic roles"; ADR-164, ADR-175), and no other color.
/// The page.
const CANVAS: &str = "#F3F4F7";
/// The panels' plot areas and the markers' fill.
const SURFACE: &str = "#FFFFFF";
/// Gridlines and the table's row rules.
const RULE: &str = "#D6DAE4";
/// Axes, the zero lines, the ground's chain line and the events' dotted lines.
const RULE_STRONG: &str = "#566079";
/// Text, the markers' outline and the rule under the table's head.
const INK: &str = "#0B0F1C";
/// Labels, units, tick labels and captions.
const INK_MUTED: &str = "#566079";
/// The hatching over a fall that is not a prediction.
const INK_FAINT: &str = "#98A1B8";
/// Simulated data, always dashed too.
const PREDICTED: &str = "#A22488";

// The line types, from the foundations' "Lines and shape".
/// Dashed: predicted or simulated data.
const DASHED: &str = "8 4";
/// Chain: a reference, here the ground.
const CHAIN: &str = "24 3 1 3";
/// Dotted: an event, with a numbered balloon.
const DOTTED: &str = "1 3";

/// The flight at one instant, as the figure draws it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Point {
    /// Time since launch, s.
    pub time_s: f64,
    /// The center of gravity's height above the launch site, m.
    pub height_m: f64,
    /// The center of gravity's speed over the ground, m/s.
    pub speed_m_s: f64,
    /// The rate of that height, m/s, positive up.
    pub vertical_speed_m_s: f64,
    /// The size of the nose tip's acceleration relative to the launch frame, m/s².
    pub acceleration_m_s2: f64,
    /// Its vertical part, m/s², positive up.
    pub vertical_acceleration_m_s2: f64,
}

impl Point {
    fn of(sample: &Sample) -> Self {
        Self {
            time_s: sample.time_s,
            height_m: sample.height_above_ground_m,
            speed_m_s: sample.cg_velocity_enu_m_s.length(),
            vertical_speed_m_s: sample.vertical_speed_m_s,
            acceleration_m_s2: sample.acceleration_enu_m_s2.length(),
            vertical_acceleration_m_s2: sample.acceleration_enu_m_s2.z,
        }
    }
}

/// Watches a flight for the figure: a [`Point`] at every multiple of the interval after launch
/// within the flight and at every event, as [`hpr::hpr_sim::Recorder`] keeps its rows, and at
/// every integration step's start and end. An event's sample is the flight just before it acts,
/// and the next step's start just after, so a jump, such as a parachute's opening shock, is drawn
/// at its instant at its full height, not at the next interval's sample.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Trace {
    interval_s: f64,
    /// The next interval sample is at `next_index · interval_s`.
    next_index: u64,
    points: Vec<Point>,
}

impl Trace {
    /// A trace sampled every `interval_s` seconds, which the caller has checked is finite and
    /// positive (`hpr sim` refuses any other `--interval` first).
    pub(crate) fn new(interval_s: f64) -> Self {
        Self {
            interval_s,
            next_index: 0,
            points: Vec::new(),
        }
    }

    /// The points, in time order.
    pub(crate) fn points(&self) -> &[Point] {
        &self.points
    }

    /// Keeps `sample`'s point, unless it repeats the last one exactly.
    fn record(&mut self, sample: &Sample) {
        let point = Point::of(sample);
        if self.points.last() != Some(&point) {
            self.points.push(point);
        }
    }
}

impl Observer for Trace {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let dt = self.interval_s;
        if !(dt.is_finite() && dt > 0.0) {
            return Err(SimError::Domain {
                what: "plot interval",
                value: dt,
            });
        }
        self.record(&step.sample(step.start_s())?);
        // A flight that starts after launch samples from its start.
        let first = (step.start_s() / dt).ceil();
        if (self.next_index as f64) < first {
            self.next_index = first as u64;
        }
        loop {
            let t = self.next_index as f64 * dt;
            if t > step.end_s() {
                break;
            }
            if t > step.start_s() && t < step.end_s() {
                self.record(&step.sample(t)?);
            }
            self.next_index += 1;
        }
        self.record(&step.sample(step.end_s())?);
        Ok(())
    }

    fn event(&mut self, event: &FlightEvent) {
        self.record(&event.sample);
    }
}

/// A figure the figure's summary gives, as `hpr sim`'s summary gives it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Reading {
    /// The value, in the unit of the quantity it is.
    pub value: f64,
    /// When, s after launch.
    pub time_s: f64,
    /// Whether it came in a fall that is not a prediction, as the summary marks it.
    pub unpredicted: bool,
}

/// What the figure shows.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Figure<'a> {
    /// The heading: the design's name.
    pub title: &'a str,
    /// The line under it: the configuration flown.
    pub subtitle: &'a str,
    /// The flight, in time order.
    pub points: &'a [Point],
    /// The flight's events, in time order.
    pub events: &'a [FlightEvent],
    /// The recovery devices' names, by the index the events give them.
    pub devices: &'a [String],
    /// From when to when the rocket fell on its airframe alone, s: not a prediction.
    pub unpredicted: Option<(f64, f64)>,
    /// The summary's apogee: its height above the launch site, m, and its time.
    pub apogee: Option<Reading>,
    /// The summary's top speed, m/s, and its time.
    pub top_speed: Option<Reading>,
}

/// One series of a panel: simulated, so dashed in the predicted color, told from the panel's
/// other series by its width, which the caption explains.
struct Series {
    value: fn(&Point) -> f64,
    /// The stroke's width, px.
    width: f64,
}

/// One panel: its axis titles, its series and its US scale.
struct Panel {
    /// The id of its group in the SVG.
    id: &'static str,
    /// Its axis title, over the left scale: the quantity in capitals, then its SI unit.
    title: &'static str,
    series: &'static [Series],
    /// Whether its zero is the ground, drawn as a chain line.
    ground: bool,
    /// The right scale's unit, US, its title over that scale.
    us_unit: &'static str,
    /// One US unit in the panel's SI unit: a foot in meters, a g in m/s².
    us_unit_si: f64,
}

/// The size of a quantity, drawn thick.
const SIZE: f64 = 2.0;
/// Its vertical part, drawn thin.
const VERTICAL: f64 = 1.0;
/// How far past its panel's edge a series may draw, px: half the thick line's width, so a series
/// that runs along its axis's end, as the altitude does on the ground, draws whole. Only a value
/// off its axis reaches the clip.
const SPILL: f64 = SIZE / 2.0;

/// The fixed figure set, top to bottom.
const PANELS: &[Panel] = &[
    Panel {
        id: "altitude",
        title: "ALTITUDE · m AGL",
        series: &[Series {
            value: |p| p.height_m,
            width: SIZE,
        }],
        ground: true,
        us_unit: "ft AGL",
        us_unit_si: FOOT_M,
    },
    Panel {
        id: "speed",
        title: "SPEED · m/s",
        series: &[
            Series {
                value: |p| p.speed_m_s,
                width: SIZE,
            },
            Series {
                value: |p| p.vertical_speed_m_s,
                width: VERTICAL,
            },
        ],
        ground: false,
        us_unit: "ft/s",
        us_unit_si: FOOT_M,
    },
    Panel {
        id: "acceleration",
        title: "ACCELERATION · m/s²",
        series: &[
            Series {
                value: |p| p.acceleration_m_s2,
                width: SIZE,
            },
            Series {
                value: |p| p.vertical_acceleration_m_s2,
                width: VERTICAL,
            },
        ],
        ground: false,
        us_unit: "g",
        us_unit_si: STANDARD_GRAVITY_M_S2,
    },
];

/// The trust note's label, as the figure sets its labels.
const TRUST_LABEL: &str = "HOW FAR TO TRUST IT";

/// What the lines are, said once under the title rather than in a legend box.
const LINES: &str = "Dashed: simulated by FusionSpace HPR; thick, the size, and thin, the vertical \
                     part, up positive. Chain line: the ground. Dotted: an event, numbered as in the table. \
                     Each panel's left scale is SI, its right scale US units. Altitude is the center of gravity's height above the launch site; speed is \
                     over the ground; acceleration is the nose tip's, relative to the launch \
                     frame, not what an accelerometer reads.";

/// Events within half a pixel of each other, as one marker.
struct Group {
    x: f64,
    /// Which marker row it sits in, from the top.
    row: usize,
    /// Each event's time, s, its name and its height above the launch site, m, in order.
    events: Vec<(f64, String, f64)>,
}

/// The figure, as an SVG document.
pub(crate) fn svg(figure: &Figure<'_>) -> String {
    let (t0, t1, t_step) = axis(
        figure
            .points
            .iter()
            .map(|p| p.time_s)
            .chain(figure.events.iter().map(|e| e.sample.time_s))
            .chain([0.0]),
    );
    let plot_width = WIDTH - LEFT - RIGHT;
    let x_of = |t: f64| LEFT + (t - t0) / (t1 - t0) * plot_width;
    let groups = groups(figure, &x_of);
    let summary = summary(figure);
    let caption: Vec<String> = [summary.as_str(), LINES]
        .iter()
        .flat_map(|paragraph| wrap(paragraph, LIST_WIDTH))
        // A caption's paragraphs run flush left.
        .map(|line| line.trim_start().to_owned())
        .collect();
    let markers_top = CAPTION_TOP + (caption.len() - 1) as f64 * LIST_LINE + CAPTION_GAP;
    let rows = groups.iter().map(|g| g.row + 1).max().unwrap_or(0);
    let panels_top = markers_top + rows as f64 * MARKER_ROW;
    let panel_top = |i: usize| panels_top + PANEL_GAP + i as f64 * (PANEL_HEIGHT + PANEL_GAP);
    let last_bottom = panel_top(PANELS.len() - 1) + PANEL_HEIGHT;
    let table_top = last_bottom + 68.0;
    let (table, table_bottom) = table(&groups, table_top);
    // What the hatching means, under the table, then how far to trust the figure, last, as the
    // text form ends.
    let mut note = Vec::new();
    if let Some((from_s, to_s)) = figure.unpredicted {
        let hatched = format!(
            "Hatched, {}\u{a0}s to {}\u{a0}s: the rocket falls on its airframe alone, on aerodynamics \
             that hold only at small angles of attack, so this is not a prediction.",
            number(from_s, 2),
            number(to_s, 2)
        );
        note = wrap(&hatched, LIST_WIDTH)
            .into_iter()
            .map(|line| line.trim_start().to_owned())
            .collect();
        // A blank line between the two paragraphs.
        note.push(String::new());
    }
    // The note's label as the figure's other labels are set, in capitals on a line of its own.
    note.push(TRUST_LABEL.to_owned());
    let trust = crate::trust::flight();
    let body = trust.strip_prefix(crate::trust::LABEL).unwrap_or(&trust);
    note.extend(crate::trust::wrap(body, LIST_WIDTH));
    // The title block's line, last and where a printed copy shows it, as a drawing's: the
    // program, its version and its designation (ADR-164, ADR-214).
    note.push(String::new());
    note.push(hpr::hpr_core::tool::stamp());
    let note_top = table_bottom + 24.0;
    let height = note_top + (note.len() - 1) as f64 * LIST_LINE + 14.0;

    let title = text(figure.title, NAME_LIMIT);
    let drawn_title = text(figure.title, TITLE_LIMIT);
    // The hatching's line runs down the middle of its tile, which clips it, so no part of its
    // width is cut off.
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"{height}\" \
         viewBox=\"0 0 {WIDTH} {height}\" font-family=\"{FONT}\" font-size=\"12\" \
         role=\"img\" aria-labelledby=\"title\" aria-describedby=\"desc\">\n\
         <title id=\"title\">{title}: altitude, speed and acceleration against time</title>\n\
         <desc id=\"desc\">{}</desc>\n\
         <metadata>{}</metadata>\n\
         <defs><pattern id=\"hatch\" width=\"6\" height=\"6\" patternUnits=\"userSpaceOnUse\" \
         patternTransform=\"rotate(45)\"><line x1=\"3\" y1=\"0\" x2=\"3\" y2=\"6\" \
         stroke=\"{INK_FAINT}\" stroke-width=\"1\"/></pattern></defs>\n\
         <rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{CANVAS}\"/>\n\
         <text x=\"{LEFT}\" y=\"{TITLE_TOP}\" font-size=\"{TITLE_SIZE}\" font-weight=\"600\" \
         fill=\"{INK}\">{drawn_title}</text>\n\
         <text x=\"{LEFT}\" y=\"{SUBTITLE_TOP}\" fill=\"{INK_MUTED}\">{}</text>\n",
        escape(&summary),
        // The title block's line (ADR-164): the program, its version and its designation.
        hpr::hpr_core::tool::stamp(),
        text(figure.subtitle, LIST_WIDTH)
    );
    lines(&mut svg, &caption, CAPTION_TOP, LEFT, INK_MUTED);

    // The markers, numbered in time order, above the panels.
    for (n, group) in groups.iter().enumerate() {
        let cy = markers_top + group.row as f64 * MARKER_ROW;
        svg.push_str(&format!(
            "<g id=\"event-{}\"><title>{}</title>\
             <line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"{RULE_STRONG}\" \
             stroke-width=\"1\" stroke-dasharray=\"{DOTTED}\"/>\
             <circle cx=\"{x:.1}\" cy=\"{cy}\" r=\"{MARKER_RADIUS}\" fill=\"{SURFACE}\" \
             stroke=\"{INK}\" stroke-width=\"1\"/>\
             <text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\" \
             fill=\"{INK}\">{}</text></g>\n",
            n + 1,
            escape(&event_line(group)),
            cy + MARKER_RADIUS,
            // It stops short of the first panel's axis title; the panels carry their own lines.
            panels_top + 16.0,
            // A 12 px numeral's figures, about 8.4 px tall, centered on the marker's.
            cy + 4.0,
            n + 1,
            x = group.x,
        ));
    }

    for (i, panel) in PANELS.iter().enumerate() {
        panel_svg(
            &mut svg,
            figure,
            panel,
            panel_top(i),
            &x_of,
            &groups,
            i == 0,
        );
    }

    // The shared time axis, under the last panel.
    let mut k = 0;
    while let Some(t) = tick(t0, t1, t_step, k) {
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">{}</text>\n",
            x_of(t),
            last_bottom + 16.0,
            label(t, t_step)
        ));
        k += 1;
    }
    // Its title at the axis's end, as a drawing labels an axis, flush with the panels' right
    // edge as the panels' titles are flush with their left.
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">\
         TIME · s</text>\n",
        LEFT + plot_width,
        last_bottom + 32.0
    ));
    svg.push_str(&table);
    lines(&mut svg, &note, note_top, LEFT, INK_MUTED);
    svg.push_str("</svg>\n");
    svg
}

/// The figure's text summary, its `<desc>` and its caption's first line: the apogee and its
/// time, and the top speed and its time, as `hpr sim`'s summary gives them, each number joined
/// to its unit by a no-break space (U+00A0, `product/data.md`, *Numbers*), so the caption, which
/// [`wrap`]s at plain spaces, never ends a line between them.
fn summary(figure: &Figure<'_>) -> String {
    let apogee = figure.apogee.map_or_else(
        || "No apogee".to_owned(),
        |apogee| {
            format!(
                "Apogee {}\u{a0}m ({}\u{a0}ft) above the launch site at {}\u{a0}s",
                number(apogee.value, 1),
                number(apogee.value / FOOT_M, 0),
                number(apogee.time_s, 2)
            )
        },
    );
    let speed = figure.top_speed.map_or_else(
        || "no top speed".to_owned(),
        |speed| {
            format!(
                "top speed {}\u{a0}m/s ({}\u{a0}ft/s) at {}\u{a0}s{}",
                number(speed.value, 1),
                number(speed.value / FOOT_M, 0),
                number(speed.time_s, 2),
                if speed.unpredicted {
                    ", in the fall: not a prediction"
                } else {
                    ""
                }
            )
        },
    );
    format!("{apogee}; {speed}.")
}

/// Plain lines of text, one under the next from `top`, a wrapped line's indent drawn by its x,
/// as SVG collapses leading spaces; an empty line leaves its space blank.
fn lines(svg: &mut String, lines: &[String], top: f64, left: f64, ink: &str) {
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let (x, line) = match line.strip_prefix(INDENT) {
            Some(rest) => (left + 20.0, rest),
            None => (left, line.as_str()),
        };
        svg.push_str(&format!(
            "<text x=\"{x}\" y=\"{:.1}\" fill=\"{ink}\">{}</text>\n",
            top + i as f64 * LIST_LINE,
            escape(line)
        ));
    }
}

/// The event table under the figure: a caption saying what it is and where it came from, a head
/// of labels with their units over a 2 px ink rule, and a row for each event, with its marker's
/// number, its time, the height above the launch site then, in meters and in feet, and its name,
/// wrapped, each row under a 1 px rule. The SVG, and the table's bottom, px.
fn table(groups: &[Group], top: f64) -> (String, f64) {
    let [number_x, time_x, height_x, feet_x, name_x] = COLUMNS.map(|x| LEFT + x);
    let right = WIDTH - MARGIN;
    let head = top + 22.0;
    let mut svg = format!(
        "<g id=\"events\">\n\
         <text x=\"{LEFT}\" y=\"{top:.1}\" fill=\"{INK_MUTED}\">EVENTS · SIMULATED BY FUSIONSPACE HPR</text>\n\
         <text x=\"{number_x}\" y=\"{head:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">NO.</text>\
         <text x=\"{time_x}\" y=\"{head:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">TIME · s</text>\
         <text x=\"{height_x}\" y=\"{head:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">\
         ALTITUDE · m AGL</text>\
         <text x=\"{feet_x}\" y=\"{head:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">\
         ALTITUDE · ft AGL</text>\
         <text x=\"{name_x}\" y=\"{head:.1}\" fill=\"{INK_MUTED}\">EVENT</text>\n\
         <line x1=\"{LEFT}\" y1=\"{:.1}\" x2=\"{right}\" y2=\"{:.1}\" stroke=\"{INK}\" \
         stroke-width=\"2\"/>\n",
        head + 7.0,
        head + 7.0,
    );
    let mut y = head + 7.0;
    for (n, group) in groups.iter().enumerate() {
        for (time_s, name, height_m) in &group.events {
            let names = wrap(name, EVENT_WIDTH);
            let first = y + LIST_LINE;
            svg.push_str(&format!(
                "<text x=\"{number_x}\" y=\"{first:.1}\" text-anchor=\"end\" fill=\"{INK}\">{}</text>\
                 <text x=\"{time_x}\" y=\"{first:.1}\" text-anchor=\"end\" fill=\"{INK}\">{}</text>\
                 <text x=\"{height_x}\" y=\"{first:.1}\" text-anchor=\"end\" fill=\"{INK}\">{}</text>\
                 <text x=\"{feet_x}\" y=\"{first:.1}\" text-anchor=\"end\" fill=\"{INK}\">{}</text>\n",
                n + 1,
                number(*time_s, 2),
                number(*height_m, 1),
                number(*height_m / FOOT_M, 0),
            ));
            lines(&mut svg, &names, first, name_x, INK);
            y = first + (names.len() - 1) as f64 * LIST_LINE + 7.0;
            svg.push_str(&format!(
                "<line x1=\"{LEFT}\" y1=\"{y:.1}\" x2=\"{right}\" y2=\"{y:.1}\" stroke=\"{RULE}\" \
                 stroke-width=\"1\"/>\n"
            ));
        }
    }
    svg.push_str("</g>\n");
    (svg, y)
}

/// One panel: its axis title, its plot area, hatching, gridlines and tick labels, the ground's
/// chain line or its zero line, the event lines through it, and its series, clipped to it less
/// [`SPILL`].
fn panel_svg(
    svg: &mut String,
    figure: &Figure<'_>,
    panel: &Panel,
    top: f64,
    x_of: &dyn Fn(f64) -> f64,
    groups: &[Group],
    first: bool,
) {
    let right = WIDTH - RIGHT;
    let bottom = top + PANEL_HEIGHT;
    let values = figure
        .points
        .iter()
        .flat_map(|p| panel.series.iter().map(move |s| (s.value)(p)))
        .chain([0.0]);
    let (lo, hi, step) = axis(values);
    let y_of = |v: f64| bottom - (v - lo) / (hi - lo) * PANEL_HEIGHT;
    let id = panel.id;
    svg.push_str(&format!(
        "<g id=\"{id}\">\n<clipPath id=\"{id}-clip\"><rect x=\"{:.1}\" y=\"{:.1}\" \
         width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\n\
         <text x=\"{LEFT}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">{}</text>\n\
         <rect x=\"{LEFT}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{PANEL_HEIGHT}\" \
         fill=\"{SURFACE}\"/>\n",
        LEFT - SPILL,
        top - SPILL,
        right - LEFT + 2.0 * SPILL,
        PANEL_HEIGHT + 2.0 * SPILL,
        top - 12.0,
        panel.title,
        right - LEFT,
    ));
    if let Some((from_s, to_s)) = figure.unpredicted {
        let (x0, x1) = (x_of(from_s).max(LEFT), x_of(to_s).min(right));
        if x1 > x0 {
            svg.push_str(&format!(
                "<rect x=\"{x0:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{PANEL_HEIGHT}\" \
                 fill=\"url(#hatch)\"/>\n",
                x1 - x0
            ));
            // Above the first panel, on its axis title's line, as hatching never goes behind
            // text: from the hatching's left edge, or just past the title where the hatching
            // starts under it, unless that runs past the panel's right edge; then ending a
            // character short of it, apart from the US scale's title past it.
            if first {
                let start = x0.max(LEFT + 160.0);
                let (x, anchor) = if right - start >= 130.0 {
                    (start, "start")
                } else {
                    (right - 8.0, "end")
                };
                svg.push_str(&format!(
                    "<text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\" \
                     fill=\"{INK_MUTED}\">not a prediction</text>\n",
                    top - 12.0
                ));
            }
        }
    }
    let mut k = 0;
    while let Some(v) = tick(lo, hi, step, k) {
        let y = y_of(v);
        // Zero is the ground in the altitude panel, a chain line; elsewhere a solid axis line.
        let line = match (v == 0.0, panel.ground) {
            (true, true) => {
                format!("stroke=\"{RULE_STRONG}\" stroke-width=\"1\" stroke-dasharray=\"{CHAIN}\"")
            }
            (true, false) => format!("stroke=\"{RULE_STRONG}\" stroke-width=\"1\""),
            (false, _) => format!("stroke=\"{RULE}\" stroke-width=\"1\""),
        };
        svg.push_str(&format!(
            "<line x1=\"{LEFT}\" y1=\"{y:.1}\" x2=\"{right}\" y2=\"{y:.1}\" {line}/>\
             <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">{}</text>\n",
            LEFT - 8.0,
            y + 4.0,
            label(v, step)
        ));
        k += 1;
    }
    us_scale(svg, panel, top, lo, hi, &y_of);
    for group in groups {
        svg.push_str(&format!(
            "<line x1=\"{x:.1}\" y1=\"{top:.1}\" x2=\"{x:.1}\" y2=\"{bottom:.1}\" \
             stroke=\"{RULE_STRONG}\" stroke-width=\"1\" stroke-dasharray=\"{DOTTED}\"/>\n",
            x = group.x
        ));
    }
    for series in panel.series {
        let points: Vec<(f64, f64)> = figure
            .points
            .iter()
            .map(|p| (x_of(p.time_s), (series.value)(p)))
            .collect();
        let mut d = String::new();
        for segment in thin(&points) {
            for (j, (x, v)) in segment.iter().enumerate() {
                let command = if j == 0 { 'M' } else { 'L' };
                d.push_str(&format!("{command}{:.1} {:.1}", x, pixel(y_of(*v))));
            }
        }
        if !d.is_empty() {
            svg.push_str(&format!(
                "<path d=\"{d}\" fill=\"none\" stroke=\"{PREDICTED}\" stroke-width=\"{}\" \
                 stroke-dasharray=\"{DASHED}\" stroke-linejoin=\"round\" \
                 clip-path=\"url(#{id}-clip)\"/>\n",
                series.width,
            ));
        }
    }
    // The axes: the left one, and the bottom one unless the ground's chain line is there.
    svg.push_str(&format!(
        "<line x1=\"{LEFT}\" y1=\"{top:.1}\" x2=\"{LEFT}\" y2=\"{bottom:.1}\" \
         stroke=\"{RULE_STRONG}\" stroke-width=\"1\"/>\n"
    ));
    if lo != 0.0 {
        svg.push_str(&format!(
            "<line x1=\"{LEFT}\" y1=\"{bottom:.1}\" x2=\"{right}\" y2=\"{bottom:.1}\" \
             stroke=\"{RULE_STRONG}\" stroke-width=\"1\"/>\n"
        ));
    }
    svg.push_str("</g>\n");
}

/// A panel's right scale, in its US unit: its title over it, on the line of the panel's own, the
/// right axis, and a tick mark and label at each of [`inner_ticks`]' round values within the
/// panel's SI axis, `lo` to `hi`. It has no gridlines: those are the SI scale's.
fn us_scale(
    svg: &mut String,
    panel: &Panel,
    top: f64,
    lo: f64,
    hi: f64,
    y_of: &dyn Fn(f64) -> f64,
) {
    let right = WIDTH - RIGHT;
    let bottom = top + PANEL_HEIGHT;
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">{}</text>\n\
         <line x1=\"{right}\" y1=\"{top:.1}\" x2=\"{right}\" y2=\"{bottom:.1}\" \
         stroke=\"{RULE_STRONG}\" stroke-width=\"1\"/>\n",
        right + 2.0 * TICK,
        top - 12.0,
        panel.us_unit,
    ));
    let (ticks, step) = inner_ticks(lo / panel.us_unit_si, hi / panel.us_unit_si);
    for v in ticks {
        let y = y_of(v * panel.us_unit_si);
        svg.push_str(&format!(
            "<line x1=\"{right}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" \
             stroke=\"{RULE_STRONG}\" stroke-width=\"1\"/>\
             <text x=\"{:.1}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">{}</text>\n",
            right + TICK,
            right + 2.0 * TICK,
            y + 4.0,
            number(v, step_decimals(step))
        ));
    }
}

/// The ticks of a second scale over `lo` to `hi`, all inside it, and their step: the longest step
/// of 1, 2, 2.5 or 5 times a power of ten that puts at least 3 ticks on it. That leaves at most 6,
/// the product system's 3 to 6 (`product/data.md`): the step before it put at most 2 on the span,
/// so the span is under 3 of that step, and each step is at least half the one before, so under
/// 6 of this one. None on a span that is empty or not finite.
fn inner_ticks(lo: f64, hi: f64) -> (Vec<f64>, f64) {
    let span = hi - lo;
    if !(span.is_finite() && span > 0.0 && lo.is_finite() && hi.is_finite()) {
        return (Vec::new(), 1.0);
    }
    // From a step at least as long as the span, which puts at most 2 ticks on it, downward.
    let top = span.log10().ceil();
    for decade in 1..40 {
        let magnitude = 10f64.powf(top - f64::from(decade));
        for factor in [10.0, 5.0, 2.5, 2.0] {
            let step = factor * magnitude;
            let first = (lo / step - 1e-9).ceil();
            let last = (hi / step + 1e-9).floor();
            if last - first + 1.0 >= 3.0 {
                let ticks = (0..)
                    .map(|k| (first + f64::from(k)) * step)
                    .take_while(|v| *v <= hi + step * 1e-9)
                    .take(7)
                    .collect();
                return (ticks, step);
            }
        }
    }
    (Vec::new(), 1.0)
}

/// The decimals a tick step needs to print exactly: none for 5 or 250, one for 0.5 or 2.5, two
/// for 0.25, seven for 1e-7. The tolerance is relative to the step, so a step far below one
/// still needs its digits (an absolute one read 1e-7 as whole, and printed every tick as `0`);
/// a step that is zero or not finite needs none.
fn step_decimals(step: f64) -> usize {
    if !(step.is_finite() && step != 0.0) {
        return 0;
    }
    (0..=12)
        .find(|&decimals| {
            let scaled = step * 10f64.powi(decimals as i32);
            (scaled - scaled.round()).abs() <= 1e-9 * scaled.abs()
        })
        .unwrap_or(12)
}

/// A pixel coordinate, held to where a viewer draws: a value far off its axis draws past the
/// clip at a bounded coordinate, not one with hundreds of digits.
fn pixel(y: f64) -> f64 {
    y.clamp(-1.0e4, 1.0e4)
}

/// The events grouped by instant (within half a pixel), each placed in the highest marker row
/// where it keeps [`MARKER_ROW`] from the one before it.
fn groups(figure: &Figure<'_>, x_of: &dyn Fn(f64) -> f64) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for event in figure.events {
        let x = x_of(event.sample.time_s);
        if !x.is_finite() {
            continue;
        }
        let row = (
            event.sample.time_s,
            event_name(event.kind, figure.devices),
            event.sample.height_above_ground_m,
        );
        match groups.last_mut() {
            Some(group) if (x - group.x).abs() < 0.5 => group.events.push(row),
            _ => groups.push(Group {
                x,
                row: 0,
                events: vec![row],
            }),
        }
    }
    // The x of each row's last marker.
    let mut rows: Vec<f64> = Vec::new();
    for group in &mut groups {
        group.row = rows
            .iter()
            .position(|last| group.x - last >= MARKER_ROW)
            .unwrap_or(rows.len());
        match rows.get_mut(group.row) {
            Some(last) => *last = group.x,
            None => rows.push(group.x),
        }
    }
    groups
}

/// A group's marker's tooltip: its events, each after its own time, the events of one printed
/// time sharing it: `0.00 s: liftoff; 0.27 s: rail exit`.
fn event_line(group: &Group) -> String {
    let mut parts: Vec<(String, Vec<&str>)> = Vec::new();
    for (time_s, name, _) in &group.events {
        let time = format!("{} s", number(*time_s, 2));
        match parts.last_mut() {
            Some((last, names)) if *last == time => names.push(name),
            _ => parts.push((time, vec![name])),
        }
    }
    parts
        .iter()
        .map(|(time, names)| format!("{time}: {}", names.join(", ")))
        .collect::<Vec<_>>()
        .join("; ")
}

/// What a wrapped line's later lines start with.
const INDENT: &str = "    ";

/// `line`, plain text, in lines of at most `width` characters (more than [`INDENT`]'s), broken
/// after a space where it can be, the later lines indented.
fn wrap(line: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut rest: Vec<char> = line.chars().collect();
    while rest.len() > width {
        let cut = rest[..width]
            .iter()
            .rposition(|c| *c == ' ')
            // Past the indent, so each line takes something.
            .filter(|at| *at > INDENT.len())
            .map_or(width, |at| at + 1);
        lines.push(rest[..cut].iter().collect::<String>().trim_end().to_owned());
        rest = INDENT.chars().chain(rest[cut..].iter().copied()).collect();
    }
    lines.push(rest.into_iter().collect());
    lines
}

/// An event, in words, its device named.
fn event_name(kind: EventKind, devices: &[String]) -> String {
    let device = |i: usize| {
        devices.get(i).map_or_else(
            || format!("device {}", i + 1),
            |name| format!("`{}`", cut(name, NAME_LIMIT)),
        )
    };
    match kind {
        EventKind::Liftoff => "liftoff".to_owned(),
        EventKind::RailExit => "rail exit".to_owned(),
        EventKind::Burnout => "burnout".to_owned(),
        EventKind::Apogee => "apogee".to_owned(),
        EventKind::GroundHit => "ground hit".to_owned(),
        EventKind::Trigger(i) => format!("charge for {}", device(i)),
        EventKind::Deployment(i) => format!("{} opens", device(i)),
        EventKind::Release(i) => format!("{} released", device(i)),
        EventKind::Separation => "separation".to_owned(),
        EventKind::Ejection(_) => "ejection".to_owned(),
        EventKind::Shift(_) => "mass shift".to_owned(),
        EventKind::MassRelease(_) => "mass release".to_owned(),
        EventKind::User(_) => "user event".to_owned(),
        EventKind::Ignition(_) => "ignition".to_owned(),
        _ => "other event".to_owned(),
    }
}

/// The axis over `values`' finite ones: its ends, rounded out to whole steps, and its tick
/// step, 1, 2 or 5 times a power of ten: the least at or above a fifth of the span that leaves 3
/// to 6 ticks, as the product system's charts take (`product/data.md`). With no finite value, or
/// none apart, `0` to `1`; with a span past `f64`'s range, the same, the values then drawn off
/// it.
fn axis(values: impl Iterator<Item = f64>) -> (f64, f64, f64) {
    let (lo, hi) = values
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v), hi.max(v))
        });
    let span = hi - lo;
    if !(span.is_finite() && span > 0.0) {
        return (0.0, 1.0, 0.2);
    }
    let raw = span / 5.0;
    let magnitude = 10f64.powf(raw.log10().floor());
    let mut step = match raw / magnitude {
        n if n <= 1.0 => 1.0,
        n if n <= 2.0 => 2.0,
        n if n <= 5.0 => 5.0,
        _ => 10.0,
    } * magnitude;
    // The span is over 2 steps and at most 5, so 4 to 7 ticks once both ends are rounded out.
    // With 7, the next step, at most 2.5 times as long: the span, then over 4 steps, is over 1.6
    // of the next and at most 2.5, so 2 to 4 intervals, 3 to 5 ticks.
    if (hi / step).ceil() - (lo / step).floor() > 5.0 {
        step *= if (step / magnitude).round() == 2.0 {
            2.5
        } else {
            2.0
        };
    }
    ((lo / step).floor() * step, (hi / step).ceil() * step, step)
}

/// The axis's `k`th tick from its low end, while it is on the axis (at most 20).
fn tick(lo: f64, hi: f64, step: f64, k: usize) -> Option<f64> {
    let v = lo + k as f64 * step;
    // A step's rounding leaves the last tick a hair past the end.
    (k <= 20 && v <= hi + step * 1e-9).then_some(v)
}

/// A tick's label, with the decimals its step needs, as [`number`] writes it.
fn label(v: f64, step: f64) -> String {
    let decimals = if step >= 1.0 {
        0
    } else {
        (-step.log10().floor()).clamp(0.0, 12.0) as usize
    };
    number(v, decimals)
}

/// `v` to `decimals` places, as the product system's numbers are read (`product/data.md`): a
/// real minus sign, U+2212, and digits grouped in threes with a comma from four digits; no
/// `−0`.
fn number(v: f64, decimals: usize) -> String {
    let rounded = format!("{:.decimals$}", v + 0.0);
    let (negative, digits) = match rounded.strip_prefix('-') {
        Some(digits) => (digits.chars().any(|c| c != '0' && c != '.'), digits),
        None => (false, rounded.as_str()),
    };
    let (whole, fraction) = digits.split_at(digits.find('.').unwrap_or(digits.len()));
    let mut out = String::from(if negative { "\u{2212}" } else { "" });
    for (i, c) in whole.chars().enumerate() {
        if whole.len() >= 4 && i > 0 && (whole.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.push_str(fraction);
    out
}

/// A series' points to draw, `(x, value)` in time order, in runs between non-finite ones: of
/// the points in each pixel column (`x`'s whole part), the first, the least, the greatest and
/// the last, in their order. So the line keeps every column's extremes, hence every peak, and
/// the file holds at most four points a column however long the flight.
fn thin(points: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
    fn flush(column: &mut Vec<(f64, f64)>, run: &mut Vec<(f64, f64)>) {
        let Some(last) = column.len().checked_sub(1) else {
            return;
        };
        let (mut least, mut greatest) = (0, 0);
        for (i, (_, v)) in column.iter().enumerate() {
            if *v < column[least].1 {
                least = i;
            }
            if *v > column[greatest].1 {
                greatest = i;
            }
        }
        let mut keep = [0, least, greatest, last];
        keep.sort_unstable();
        let mut previous = None;
        for i in keep {
            if previous != Some(i) {
                run.push(column[i]);
                previous = Some(i);
            }
        }
        column.clear();
    }
    let mut runs = Vec::new();
    let (mut run, mut column) = (Vec::new(), Vec::new());
    let mut current = None;
    for &(x, v) in points {
        if !(x.is_finite() && v.is_finite()) {
            flush(&mut column, &mut run);
            if !run.is_empty() {
                runs.push(std::mem::take(&mut run));
            }
            current = None;
            continue;
        }
        let at = x.floor();
        if current != Some(at) {
            flush(&mut column, &mut run);
            current = Some(at);
        }
        column.push((x, v));
    }
    flush(&mut column, &mut run);
    if !run.is_empty() {
        runs.push(run);
    }
    runs
}

/// `s` as SVG text, [`cut`] at `limit` characters and [`escape`]d, so a file's name can't break
/// or swell the figure.
fn text(s: &str, limit: usize) -> String {
    escape(&cut(s, limit))
}

/// `s` cut at `limit` characters with an ellipsis, the characters XML may not hold, and the
/// other control characters, replaced by a space.
fn cut(s: &str, limit: usize) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == limit {
            out.push('…');
            break;
        }
        out.push(if c.is_control() || c == '\u{FFFE}' || c == '\u{FFFF}' {
            ' '
        } else {
            c
        });
    }
    out
}

/// `s` with its markup characters escaped.
fn escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests;
