//! `hpr compare --plot`: a flight log's altitude and vertical speed beside its simulation's, as
//! designed and under the launch day's conditions, from liftoff to landing, as one SVG figure
//! ([M7.3e][m7-3e], ADR-379).
//!
//! The figure follows `hpr sim --plot`'s ([ADR-157][adr-157]) and the FusionSpace product
//! system's chart rules ([ADR-175][adr-175]): two panels over one time axis, each with an SI scale
//! on its left and a US one on its right, and no legend box: a caption says what each line is.
//!
//! - **The log** is measured, so it is ink and solid, 2 px. Its heights are its own samples less
//!   its pad, unsmoothed; its speed is the smoother's, derived from them
//!   ([`hpr::hpr_forensics::residuals::logged_lines`]).
//! - **Each simulation** is predicted, so it is dashed in the predicted color: thin, 1 px, as
//!   designed; thick, 2 px, under the launch day's conditions. Each is read as the log's sensor
//!   reads it, a barometer's reading or the height climbed, as `hpr compare`'s residuals are
//!   ([`hpr::hpr_forensics::residuals::Simulated::read_lines`]).
//! - **Time** is from each one's own liftoff, as the comparison lines the clocks up.
//!
//! Under the panels, a table gives each reading the log and each simulation's flight give it: the
//! logged value with its bound, where it came from and whether it was measured or derived, its
//! provenance from [M7.2][m7-2]. Then the conditions each simulation flew in and where they came
//! from, how far to trust the figure, and the title block.
//!
//! A logged sample far off both simulations, such as a garbled sample or an ejection charge's
//! pressure spike, would stretch an axis until the flight is a line along its foot. So an axis
//! holds the simulations' values, and the log's within one span of them (the span of the
//! simulations' values) either side; a logged sample past that runs off the panel's edge, where it
//! is clipped, and a note under the table counts such samples and gives the furthest.
//!
//! [m7-3e]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3e
//! [m7-2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2
//! [adr-157]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0157-hpr-sim-plot.md
//! [adr-175]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0175-the-plot-in-the-product-system.md

use hpr::hpr_forensics::residuals::LinePoint;

use super::{
    CANVAS, CAPTION_TOP, CHAIN, CHAR_PX, DASHED, FONT, INDENT, INDENT_PX, INK, INK_MUTED, LEFT,
    LIST_LINE, LIST_WIDTH, MARGIN, NAME_LIMIT, PANEL_GAP, PANEL_HEIGHTS, PREDICTED, RIGHT,
    ROW_RULE, RULE, RULE_STRONG, SIZE, SPILL, SUBTITLE_TOP, SURFACE, TITLE_SIZE, TITLE_TOP,
    TRUST_LABEL, VERTICAL, WIDTH, axis, bank, cut, escape, fit, label, lines, number, pixel, text,
    thin, tick, title_block, us_scale, width_px,
};
use crate::units::FOOT_M;

/// One simulation's lines, each point's time after its liftoff.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Line<'a> {
    /// Its height as the log's sensor reads it, m above the pad.
    pub height: &'a [LinePoint],
    /// That height's rate, m/s, up.
    pub speed: &'a [LinePoint],
}

/// One reading's row of the table: each cell's text.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReadingRow {
    /// The reading's name.
    pub name: String,
    /// The logged value with its bound, or why it is withheld.
    pub logged: String,
    /// Where the logged value came from and whether it was measured or derived.
    pub from: String,
    /// The simulation's as designed.
    pub designed: String,
    /// The launch day's simulation's, where it was flown.
    pub launch_day: Option<String>,
}

/// What the figure shows.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Figure<'a> {
    /// The heading: the log's name, against its simulation.
    pub title: &'a str,
    /// The line under it: the design, its configuration and the logger.
    pub subtitle: &'a str,
    /// The bundled motor catalog's as-of date.
    pub catalog_as_of: &'a str,
    /// The figure's data file, by its name.
    pub data_file: &'a str,
    /// The log's heights, m above its pad.
    pub logged_height: &'a [LinePoint],
    /// The smoother's vertical speed, m/s, where it gives one.
    pub logged_speed: Option<&'a [LinePoint]>,
    /// Why the smoother gives no speed, where it gives none.
    pub speed_withheld: Option<&'a str>,
    /// The flight as designed.
    pub designed: Line<'a>,
    /// The flight under the launch day's conditions, where any are given.
    pub launch_day: Option<Line<'a>>,
    /// The apogee, m above the pad: logged, as designed, and on the launch day.
    pub apogees: (Option<f64>, Option<f64>, Option<f64>),
    /// The readings' rows.
    pub readings: &'a [ReadingRow],
    /// The site's and each simulation's conditions, one sentence each.
    pub conditions: &'a [String],
    /// How the clocks were lined up, one sentence.
    pub clock: &'a str,
    /// The note on how far to trust it, as the command prints it.
    pub trust: &'a str,
}

/// A line drawn in a panel.
struct Drawn<'a> {
    points: &'a [LinePoint],
    /// Measured, so ink and solid, or predicted, so dashed in the predicted color.
    measured: bool,
    /// The stroke's width, px.
    width: f64,
}

/// A panel: its titles, its units and its lines.
struct Panel<'a> {
    id: &'static str,
    title: &'static str,
    ground: bool,
    us_unit: &'static str,
    us_unit_si: f64,
    lines: Vec<Drawn<'a>>,
}

/// What the lines are, said once under the title rather than in a legend box.
const LINES: &str = "Solid ink: the log, its heights as logged above its pad and its speed the \
                     smoother's, derived from them. Dashed: simulated by FusionSpace HPR and read \
                     as the log's sensor reads it; thin, as designed, and thick, under the launch \
                     day's conditions. Time is from each one's liftoff. Chain line: the ground. \
                     Each panel's left scale is SI, its right scale US units.";

/// The table's columns with a launch day: each one's left edge from the left margin, px, and its
/// width, px, 12 px Cascadia Mono's 7.2 px a character.
const COLUMNS_FIVE: [(f64, f64); 5] = [
    (0.0, 172.8),
    (184.0, 230.4),
    (424.0, 158.4),
    (592.0, 129.6),
    (730.0, 129.6),
];
/// Without one: the as-designed column takes the launch day's room too.
const COLUMNS_FOUR: [(f64, f64); 4] =
    [(0.0, 172.8), (184.0, 230.4), (424.0, 158.4), (592.0, 266.4)];

/// How far past the simulations' values, in spans of them, an axis reaches for the log's.
const LOGGED_REACH: f64 = 1.0;
/// How far under zero, as a fraction of a panel's largest value, a value is taken as zero for
/// the axis: one whose line's middle draws no further under its foot than a line's spill
/// ([`SPILL`], 1 px) on the tallest panel ([`PANEL_HEIGHTS`], 360 px), where the panel's clip
/// ends, so at most the lower half of the log's 2 px line is cut there. A logger's noise on the
/// pad is that small: 3 m on a flight of 4 km is a quarter of a pixel there.
const GROUND_SLACK: f64 = SPILL / PANEL_HEIGHTS.1;

/// How wide a line of the caption or the notes draws, px at 12 px: [`LIST_WIDTH`] characters.
const LIST_PX: f64 = LIST_WIDTH as f64 * CHAR_PX;

/// `sentence` in lines for the notes, each drawn within [`LIST_PX`] by its width ([`fill`], so a
/// wide character counts two cells and a control character goes), the later lines indented.
fn note_lines(sentence: &str) -> Vec<String> {
    let mut out = fill(sentence, LIST_PX - INDENT_PX);
    for line in out.iter_mut().skip(1) {
        line.insert_str(0, INDENT);
    }
    out
}

/// The figure, as an SVG document.
pub(crate) fn svg(figure: &Figure<'_>) -> String {
    let panels = panels(figure);
    let (t0, t1, t_step) = axis(
        panels
            .iter()
            .flat_map(|panel| &panel.lines)
            .flat_map(|line| line.points.iter().map(|p| p.time_after_liftoff_s))
            .chain([0.0]),
    );
    let plot_width = WIDTH - LEFT - RIGHT;
    let x_of = |t: f64| LEFT + (t - t0) / (t1 - t0) * plot_width;
    let summary = summary(figure);
    let caption: Vec<String> = [summary.as_str(), LINES]
        .iter()
        .flat_map(|paragraph| fill(paragraph, LIST_PX))
        .collect();
    let mut frames = Vec::with_capacity(panels.len());
    let mut top = CAPTION_TOP + (caption.len() - 1) as f64 * LIST_LINE + PANEL_GAP;
    let mut past = Vec::new();
    for panel in &panels {
        let (lo, hi, step, outside) = value_axis(panel);
        let height = bank(&runs(panel, (lo, hi), &x_of));
        frames.push((top, height, (lo, hi, step)));
        past.push(outside);
        top += height + PANEL_GAP;
    }
    let last_bottom = top - PANEL_GAP;
    let table_top = last_bottom + 64.0;
    let (table, table_bottom) = table(figure, table_top);

    // The conditions, then what the axes leave out, then how far to trust it.
    let mut note = vec!["CONDITIONS".to_owned()];
    for sentence in figure
        .conditions
        .iter()
        .map(String::as_str)
        .chain([figure.clock])
    {
        note.extend(note_lines(sentence));
    }
    if let Some(detail) = figure.speed_withheld {
        note.extend(note_lines(&format!(
            "The log's speed is withheld: {detail}"
        )));
    }
    for (panel, outside) in panels.iter().zip(&past) {
        if let Some((count, furthest)) = outside {
            let unit = if panel.ground { "m" } else { "m/s" };
            note.extend(note_lines(&format!(
                "{count} logged {} past the {} axis, more than a span of the simulations' \
                     values from them, {}: they run off the panel's edge. The furthest is \
                     {}\u{a0}{unit}.",
                if *count == 1 {
                    "sample lies"
                } else {
                    "samples lie"
                },
                if panel.ground { "altitude" } else { "speed" },
                "such as a garbled sample, an ejection charge's pressure spike, or a \
                     simulation far off the flight",
                number(*furthest, 1),
            )));
        }
    }
    note.push(String::new());
    note.push(TRUST_LABEL.to_owned());
    let body = figure
        .trust
        .strip_prefix(crate::trust::LABEL)
        .unwrap_or(figure.trust);
    note.extend(crate::trust::wrap(body, LIST_WIDTH));
    let note_top = table_bottom + 24.0;
    let (block, block_bottom) = title_block(
        &title_cells(figure),
        note_top + (note.len() - 1) as f64 * LIST_LINE + 24.0,
    );
    let height = block_bottom + MARGIN;

    let title = text(figure.title, NAME_LIMIT);
    // Fitted by width, as a wide character draws two cells: at the title's size, within the
    // right margin.
    let drawn_title = escape(&fit(
        figure.title,
        (WIDTH - MARGIN - LEFT) * 12.0 / TITLE_SIZE,
    ));
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"{height}\" \
         viewBox=\"0 0 {WIDTH} {height}\" font-family=\"{FONT}\" font-size=\"12\" \
         role=\"img\" aria-labelledby=\"title\" aria-describedby=\"desc\">\n\
         <title id=\"title\">{title}: logged and simulated altitude and vertical speed against \
         time</title>\n\
         <desc id=\"desc\">{}</desc>\n\
         <metadata>{}</metadata>\n\
         <rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{CANVAS}\"/>\n\
         <text x=\"{LEFT}\" y=\"{TITLE_TOP}\" font-size=\"{TITLE_SIZE}\" font-weight=\"600\" \
         fill=\"{INK}\">{drawn_title}</text>\n\
         <text x=\"{LEFT}\" y=\"{SUBTITLE_TOP}\" fill=\"{INK_MUTED}\">{}</text>\n",
        escape(&summary),
        hpr::hpr_core::tool::stamp(),
        escape(&fit(figure.subtitle, WIDTH - MARGIN - LEFT))
    );
    lines(&mut svg, &caption, CAPTION_TOP, LEFT, INK_MUTED);
    for (panel, (top, height, scale)) in panels.iter().zip(&frames) {
        panel_svg(&mut svg, panel, (*top, *height), *scale, &x_of);
    }
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
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">\
         TIME AFTER LIFTOFF · s</text>\n",
        LEFT + plot_width,
        last_bottom + 32.0
    ));
    svg.push_str(&table);
    lines(&mut svg, &note, note_top, LEFT, INK_MUTED);
    svg.push_str(&block);
    svg.push_str("</svg>\n");
    svg
}

/// The two panels: altitude, then vertical speed, each with the log's line, then the simulations'.
fn panels<'a>(figure: &Figure<'a>) -> Vec<Panel<'a>> {
    let simulated = |of: fn(&Line<'a>) -> &'a [LinePoint]| {
        std::iter::once(Drawn {
            points: of(&figure.designed),
            measured: false,
            width: VERTICAL,
        })
        .chain(figure.launch_day.as_ref().map(|day| Drawn {
            points: of(day),
            measured: false,
            width: SIZE,
        }))
    };
    let logged = |points: Option<&'a [LinePoint]>| {
        points.map(|points| Drawn {
            points,
            measured: true,
            width: SIZE,
        })
    };
    vec![
        Panel {
            id: "altitude",
            title: "ALTITUDE · m above the pad",
            ground: true,
            us_unit: "ft",
            us_unit_si: FOOT_M,
            lines: logged(Some(figure.logged_height))
                .into_iter()
                .chain(simulated(|line| line.height))
                .collect(),
        },
        Panel {
            id: "speed",
            title: "VERTICAL SPEED · m/s, up",
            ground: false,
            us_unit: "ft/s",
            us_unit_si: FOOT_M,
            lines: logged(figure.logged_speed)
                .into_iter()
                .chain(simulated(|line| line.speed))
                .collect(),
        },
    ]
}

/// A panel's value axis, `(lo, hi, step)`, and the logged samples it leaves out: their count and
/// the furthest of them. The axis holds zero, every simulated value, and each logged value within
/// [`LOGGED_REACH`] spans of the simulated values' range.
fn value_axis(panel: &Panel<'_>) -> (f64, f64, f64, Option<(usize, f64)>) {
    let finite = |line: &Drawn<'_>| {
        line.points
            .iter()
            .map(|p| p.value)
            .filter(|v| v.is_finite())
            .collect::<Vec<f64>>()
    };
    let simulated: Vec<f64> = panel
        .lines
        .iter()
        .filter(|line| !line.measured)
        .flat_map(finite)
        .chain([0.0])
        .collect();
    let (least, most) = simulated
        .iter()
        .fold((0.0_f64, 0.0_f64), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
    let reach = LOGGED_REACH * (most - least);
    let (from, to) = (least - reach, most + reach);
    let mut kept = simulated;
    let mut outside = 0;
    let mut furthest: Option<f64> = None;
    for v in panel
        .lines
        .iter()
        .filter(|line| line.measured)
        .flat_map(finite)
    {
        if (from..=to).contains(&v) {
            kept.push(v);
        } else {
            outside += 1;
            let beyond = if v > to { v - to } else { from - v };
            if furthest.is_none_or(|f| beyond > (if f > to { f - to } else { from - f })) {
                furthest = Some(v);
            }
        }
    }
    // A value a hair under the ground, as a barometer's reading or a landing can be, would add a
    // whole step under it: within [`GROUND_SLACK`] of the axis's top it is taken as on the ground,
    // and its line's middle draws within the spill, as the axis spans at least that top.
    let top = kept.iter().fold(0.0_f64, |most, v| most.max(v.abs()));
    let (lo, hi, step) = axis(kept.into_iter().map(|v| {
        if v < 0.0 && -v <= GROUND_SLACK * top {
            0.0
        } else {
            v
        }
    }));
    (lo, hi, step, furthest.map(|f| (outside, f)))
}

/// A panel's lines as drawn, [`thin`]ned, x in px and each value as the fraction of the axis.
fn runs(
    panel: &Panel<'_>,
    (lo, hi): (f64, f64),
    x_of: &dyn Fn(f64) -> f64,
) -> Vec<Vec<(f64, f64)>> {
    panel
        .lines
        .iter()
        .flat_map(|line| {
            let points: Vec<(f64, f64)> = line
                .points
                .iter()
                .map(|p| (x_of(p.time_after_liftoff_s), (p.value - lo) / (hi - lo)))
                .collect();
            thin(&points)
        })
        .collect()
}

/// One panel: its axis titles, plot area, gridlines and ticks, the ground's chain line or its zero
/// line, its US scale and its lines, clipped to it less [`SPILL`].
fn panel_svg(
    svg: &mut String,
    panel: &Panel<'_>,
    (top, height): (f64, f64),
    (lo, hi, step): (f64, f64, f64),
    x_of: &dyn Fn(f64) -> f64,
) {
    let right = WIDTH - RIGHT;
    let bottom = top + height;
    let y_of = |v: f64| bottom - (v - lo) / (hi - lo) * height;
    let id = panel.id;
    svg.push_str(&format!(
        "<g id=\"{id}\">\n<clipPath id=\"{id}-clip\"><rect x=\"{:.1}\" y=\"{:.1}\" \
         width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\n\
         <text x=\"{LEFT}\" y=\"{:.1}\" fill=\"{INK_MUTED}\">{}</text>\n\
         <rect x=\"{LEFT}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{height}\" \
         fill=\"{SURFACE}\"/>\n",
        LEFT - SPILL,
        top - SPILL,
        right - LEFT + 2.0 * SPILL,
        height + 2.0 * SPILL,
        top - 12.0,
        escape(panel.title),
        right - LEFT,
    ));
    let mut k = 0;
    while let Some(v) = tick(lo, hi, step, k) {
        let y = y_of(v);
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
    us_scale(
        svg,
        (panel.us_unit, panel.us_unit_si),
        (top, bottom),
        lo,
        hi,
        &y_of,
    );
    // The simulations first, the log over them.
    for line in panel
        .lines
        .iter()
        .filter(|line| !line.measured)
        .chain(panel.lines.iter().filter(|line| line.measured))
    {
        let points: Vec<(f64, f64)> = line
            .points
            .iter()
            .map(|p| (x_of(p.time_after_liftoff_s), p.value))
            .collect();
        let mut d = String::new();
        for segment in thin(&points) {
            for (j, (x, v)) in segment.iter().enumerate() {
                let command = if j == 0 { 'M' } else { 'L' };
                d.push_str(&format!("{command}{:.1} {:.1}", x, pixel(y_of(*v))));
            }
        }
        if d.is_empty() {
            continue;
        }
        let stroke = if line.measured {
            format!("stroke=\"{INK}\"")
        } else {
            format!("stroke=\"{PREDICTED}\" stroke-dasharray=\"{DASHED}\"")
        };
        svg.push_str(&format!(
            "<path d=\"{d}\" fill=\"none\" {stroke} stroke-width=\"{}\" \
             stroke-linejoin=\"round\" clip-path=\"url(#{id}-clip)\"/>\n",
            line.width,
        ));
    }
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

/// The caption's first paragraph and the figure's `<desc>`: the apogees, logged and simulated,
/// each number joined to its unit by a no-break space.
fn summary(figure: &Figure<'_>) -> String {
    let height = |value: Option<f64>| {
        value.map_or_else(
            || "none read".to_owned(),
            |value| {
                format!(
                    "{}\u{a0}m ({}\u{a0}ft)",
                    number(value, 1),
                    number(value / FOOT_M, 0)
                )
            },
        )
    };
    let (logged, designed, day) = figure.apogees;
    let simulated = match (&figure.launch_day, day) {
        (Some(_), day) => format!(
            "simulated {} as designed and {} under the launch day's conditions",
            height(designed),
            height(day)
        ),
        (None, _) => format!("simulated {} as designed", height(designed)),
    };
    format!(
        "Apogee above the pad: logged {}; {simulated}, each read as the log's sensor reads it.",
        height(logged)
    )
}

/// The readings table, from `top`: a caption, a head over a 2 px ink rule, and a row for each
/// reading, its cells wrapped within their columns, each row under a 1 px rule. The SVG, and the
/// table's bottom, px.
fn table(figure: &Figure<'_>, top: f64) -> (String, f64) {
    let five = figure.launch_day.is_some();
    let columns: &[(f64, f64)] = if five { &COLUMNS_FIVE } else { &COLUMNS_FOUR };
    let heads: &[&str] = if five {
        &["READING", "LOGGED", "FROM", "AS DESIGNED", "LAUNCH DAY"]
    } else {
        &["READING", "LOGGED", "FROM", "AS DESIGNED"]
    };
    let right = WIDTH - MARGIN;
    let head = top + 24.0;
    let mut svg = format!(
        "<g id=\"readings\">\n\
         <text x=\"{LEFT}\" y=\"{top:.1}\" fill=\"{INK_MUTED}\">READINGS · THE LOG'S, AND EACH \
         SIMULATION'S READ AS THE LOG IS</text>\n"
    );
    for ((x, _), name) in columns.iter().zip(heads) {
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{head:.1}\" fill=\"{INK_MUTED}\">{name}</text>",
            LEFT + x
        ));
    }
    svg.push_str(&format!(
        "\n<line x1=\"{LEFT}\" y1=\"{:.1}\" x2=\"{right}\" y2=\"{:.1}\" stroke=\"{INK}\" \
         stroke-width=\"2\"/>\n",
        head + ROW_RULE,
        head + ROW_RULE,
    ));
    let mut y = head + ROW_RULE;
    for row in figure.readings {
        let mut cells = vec![
            row.name.as_str(),
            row.logged.as_str(),
            row.from.as_str(),
            row.designed.as_str(),
        ];
        if five {
            cells.push(row.launch_day.as_deref().unwrap_or(""));
        }
        let wrapped: Vec<Vec<String>> = cells
            .iter()
            .zip(columns)
            .map(|(cell, (_, width))| fill(cell, *width))
            .collect();
        let first = y + LIST_LINE;
        for (cell, (x, _)) in wrapped.iter().zip(columns) {
            lines(&mut svg, cell, first, LEFT + x, INK);
        }
        let tallest = wrapped.iter().map(Vec::len).max().unwrap_or(1).max(1);
        y = first + (tallest - 1) as f64 * LIST_LINE + ROW_RULE;
        svg.push_str(&format!(
            "<line x1=\"{LEFT}\" y1=\"{y:.1}\" x2=\"{right}\" y2=\"{y:.1}\" stroke=\"{RULE}\" \
             stroke-width=\"1\"/>\n"
        ));
    }
    svg.push_str("</g>\n");
    (svg, y)
}

/// `cell`, [`cut`] of control characters, in lines that each draw within `px` ([`width_px`]):
/// broken at spaces, and a word too wide for a line broken where it must be.
fn fill(cell: &str, px: f64) -> Vec<String> {
    let cell = cut(cell, usize::MAX);
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in cell.split(' ').filter(|word| !word.is_empty()) {
        let joined = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if width_px(&joined) <= px {
            line = joined;
            continue;
        }
        if !line.is_empty() {
            out.push(std::mem::take(&mut line));
        }
        // A word wider than the column on its own, broken by characters.
        for c in word.chars() {
            let next = format!("{line}{c}");
            if width_px(&next) > px && !line.is_empty() {
                out.push(std::mem::take(&mut line));
                line.push(c);
            } else {
                line = next;
            }
        }
    }
    if !line.is_empty() || out.is_empty() {
        out.push(line);
    }
    out
}

/// The title block's cells, as `hpr sim --plot`'s: no date of issue, so the same log and flight
/// draw the same figure on any day.
fn title_cells(figure: &Figure<'_>) -> Vec<(&'static str, String)> {
    use hpr::hpr_core::tool::{DESIGNATION, NAME, VERSION};
    vec![
        ("OWNER", "FusionSpace".to_owned()),
        ("TITLE", cut(figure.title, NAME_LIMIT)),
        ("TYPE", "Flight against its simulation".to_owned()),
        ("DESIGNATION", DESIGNATION.to_owned()),
        ("PROGRAM", format!("{NAME} {VERSION}")),
        ("UNITS", "m, m/s, s [ft, ft/s]".to_owned()),
        ("DATA", figure.data_file.to_owned()),
        ("MOTOR CATALOG", format!("as of {}", figure.catalog_as_of)),
    ]
}

/// The figure's data, a CSV file beside it, and its rows: a header with each column's unit, then a
/// row for each point a line draws from, in SI, ASCII and CRLF lines as `hpr sim --plot`'s data.
/// The log's heights and its speed are on clocks of their own, so each of their rows leaves the
/// other's cell empty. A value that is not finite is refused, by its line and time.
pub(crate) fn csv(figure: &Figure<'_>) -> Result<(String, usize), String> {
    let mut out = String::from("line,time after liftoff [s],altitude [m],vertical speed [m/s]\r\n");
    let mut rows = 0;
    let mut row = |line: &str, t: f64, height: Option<f64>, speed: Option<f64>| {
        let mut cells = vec![line.to_owned(), format!("{t:?}")];
        for (name, value) in [("altitude", height), ("vertical speed", speed)] {
            match value {
                Some(value) if !value.is_finite() => {
                    return Err(format!(
                        "the {line} {name} at {t} s is {value}, not a number a CSV can hold"
                    ));
                }
                Some(value) => cells.push(format!("{value:?}")),
                None => cells.push(String::new()),
            }
        }
        out.push_str(&cells.join(","));
        out.push_str("\r\n");
        rows += 1;
        Ok(())
    };
    for p in figure.logged_height {
        row("logged", p.time_after_liftoff_s, Some(p.value), None)?;
    }
    for p in figure.logged_speed.unwrap_or(&[]) {
        row("logged", p.time_after_liftoff_s, None, Some(p.value))?;
    }
    let simulated = std::iter::once(("as designed", figure.designed))
        .chain(figure.launch_day.map(|day| ("launch day", day)));
    for (name, line) in simulated {
        for (height, speed) in line.height.iter().zip(line.speed) {
            row(
                name,
                height.time_after_liftoff_s,
                Some(height.value),
                Some(speed.value),
            )?;
        }
    }
    Ok((out, rows))
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;

    fn line(points: &[(f64, f64)]) -> Vec<LinePoint> {
        points
            .iter()
            .map(|&(time_after_liftoff_s, value)| LinePoint {
                time_after_liftoff_s,
                value,
            })
            .collect()
    }

    /// A cell's lines each draw within its column, a long word broken where it must be.
    #[test]
    fn a_cell_wraps_within_its_column() {
        let px = 129.6;
        let lines = fill(
            "withheld: the smoother needs a sample every 0.5 s at the least, and this log's are \
             1.25 s apart; aVeryLongWordThatCannotFitInOneColumnAtAll",
            px,
        );
        assert!(lines.len() > 3, "{lines:?}");
        for line in &lines {
            assert!(width_px(line) <= px, "{line}");
            assert!(!line.is_empty());
        }
        assert_eq!(fill("", px), vec![String::new()]);
    }

    /// A logged sample far off the simulations runs off its axis and is counted; one within a
    /// span of them stays on it.
    #[test]
    fn a_far_logged_sample_leaves_the_axis_and_is_counted() {
        let simulated = line(&[(0.0, 0.0), (10.0, 1000.0), (20.0, 0.0)]);
        let logged = line(&[(0.0, 0.0), (10.0, 1900.0), (10.5, 7800.0), (20.0, -500.0)]);
        let panel = Panel {
            id: "altitude",
            title: "ALTITUDE",
            ground: true,
            us_unit: "ft",
            us_unit_si: FOOT_M,
            lines: vec![
                Drawn {
                    points: &logged,
                    measured: true,
                    width: SIZE,
                },
                Drawn {
                    points: &simulated,
                    measured: false,
                    width: VERTICAL,
                },
            ],
        };
        let (lo, hi, _, outside) = value_axis(&panel);
        assert!(lo <= -500.0 && (1900.0..7800.0).contains(&hi), "{lo} {hi}");
        assert_eq!(outside, Some((1, 7800.0)));
    }

    /// A figure with every part: a long title, a launch day, long withheld cells, a sample off
    /// the axis and a withheld speed.
    fn full_figure_svg() -> String {
        let height = line(&[
            (0.0, 0.0),
            (5.0, 900.0),
            (10.0, 1200.0),
            (11.0, 9000.0),
            (60.0, -0.4),
        ]);
        let speed = line(&[(0.0, 0.0), (2.0, 250.0), (10.0, 0.0), (60.0, -6.0)]);
        let designed_h = line(&[(0.0, 0.0), (10.0, 1300.0), (62.0, 0.0)]);
        let designed_s = line(&[(0.0, 0.0), (2.0, 260.0), (62.0, -6.0)]);
        let day_h = line(&[(0.0, 0.0), (10.0, 1250.0), (61.0, -0.2)]);
        let day_s = line(&[(0.0, 0.0), (2.0, 255.0), (61.0, -6.0)]);
        let withheld = "withheld: the climb's speed reaches 347.9 m/s, at or above 227.7 m/s, where \
                        the barometer's port may read the transonic flow over the nose";
        let readings = vec![
            ReadingRow {
                name: "apogee, above the pad".to_owned(),
                logged: "1,548.4 m (5,080 ft), within 114.6 m (376 ft), 90% interval 1,475.4 \
                         to 1,620.6 m (4,840 to 5,317 ft)"
                    .to_owned(),
                from: "measured, from the barometric altitude channel".to_owned(),
                designed: "1,548.0 m (5,079 ft)".to_owned(),
                launch_day: Some("1,520.3 m (4,988 ft)".to_owned()),
            },
            ReadingRow {
                name: "main's opening, above the pad".to_owned(),
                logged: withheld.to_owned(),
                from: String::new(),
                designed: "withheld too".to_owned(),
                launch_day: Some("at least 1,464.0 m (4,803 ft)".to_owned()),
            },
        ];
        let conditions = vec![
            "Site: 32.94000° N, 106.91000° W, from the log's GNSS fixes before liftoff; 1401 m \
             above sea level; a 5.18 m rail at 84.0° above the horizon."
                .to_owned(),
            "As designed: the 1976 standard atmosphere and no wind, at the site.".to_owned(),
            // A file name may hold wide characters and a control character.
            "Launch day: the weather profile 打ち上げ日の気象観測データ打ち上げ日の気象観測データ打ち上げ日の\
             気象観測データ打ち上げ日の気象観測データ\u{1}.json, its wind and air."
                .to_owned(),
        ];
        // Wide characters draw two cells each: 33 of them and 33 narrow ones overrun a cut by count.
        let title = "打ち上げ記録打ち上げ記録打ち上げ記録打ち上げ記録打ち上げ記録打ち上げ記録打ち上げ記 \
                     flight-0001-from-a-ground-station.csv against its simulation";
        svg(&Figure {
            title,
            subtitle: "ロケットの設計ファイルの名前がとても長い場合でも図の端を越えないことを確かめる設計.ork, \
                       [M1378R-P]; logged by TeleMetrum v2.0, a ground station export",
            catalog_as_of: "2026-11-01",
            data_file: "out.plot.csv",
            logged_height: &height,
            logged_speed: Some(&speed),
            speed_withheld: Some("the smoother needs samples every 0.5 s"),
            designed: Line {
                height: &designed_h,
                speed: &designed_s,
            },
            launch_day: Some(Line {
                height: &day_h,
                speed: &day_s,
            }),
            apogees: (Some(1200.0), Some(1300.0), Some(1250.0)),
            readings: &readings,
            conditions: &conditions,
            clock: "Clocks: lined up by a shared liftoff, within 0.05 s.",
            trust: "How far to trust it. A log against a simulation.",
        })
    }

    /// Nothing is clipped: every text's box, at 0.6 em a character (two for a wide one), lies on
    /// the figure inside its margins, and each table cell inside its column, short of the next.
    #[test]
    fn no_text_runs_off_the_figure_or_its_column() {
        let svg = full_figure_svg();
        let document = roxmltree::Document::parse(&svg).unwrap();
        let root = document.root_element();
        let height: f64 = root.attribute("height").unwrap().parse().unwrap();
        let mut checked = 0;
        for node in document
            .descendants()
            .filter(|node| node.has_tag_name("text"))
        {
            let content: String = node
                .descendants()
                .filter(roxmltree::Node::is_text)
                .filter_map(|n| n.text())
                .collect::<String>();
            let x: f64 = node.attribute("x").unwrap().parse().unwrap();
            let y: f64 = node.attribute("y").unwrap().parse().unwrap();
            let size: f64 = node
                .attribute("font-size")
                .map_or(12.0, |size| size.parse().unwrap());
            let width = width_px(&content) * size / 12.0;
            let (x0, x1) = match node.attribute("text-anchor") {
                Some("end") => (x - width, x),
                Some("middle") => (x - width / 2.0, x + width / 2.0),
                _ => (x, x + width),
            };
            assert!(x0 >= 8.0 && x1 <= WIDTH - 8.0, "{content:?}: {x0} to {x1}");
            assert!(
                y - size >= 0.0 && y + 0.3 * size <= height,
                "{content:?} at {y}"
            );
            // The table's caption spans the table; each other text of it is a cell.
            let caption = content.starts_with("READINGS");
            if node.parent().and_then(|p| p.attribute("id")) == Some("readings") && !caption {
                let column = COLUMNS_FIVE
                    .iter()
                    .rev()
                    .find(|(left, _)| x0 >= LEFT + left - 0.01)
                    .unwrap();
                assert!(
                    x1 <= LEFT + column.0 + column.1 + 0.01,
                    "{content:?} runs out of its column: {x1}"
                );
                assert!(x1 <= WIDTH - MARGIN, "{content:?}");
            }
            checked += 1;
        }
        assert!(checked > 60, "{checked}");
        // The title is cut to fit over the panels, and the far sample counted.
        assert!(
            svg.contains("1 logged sample lies past the altitude axis"),
            "{svg}"
        );
        assert!(svg.contains("The furthest is 9,000.0\u{a0}m."));
        assert!(svg.contains("speed is withheld: the smoother"));
    }

    /// A landing a hair under the ground doesn't add a step under it.
    #[test]
    fn a_hair_under_the_ground_keeps_the_axis_at_zero() {
        let simulated = line(&[(0.0, 0.0), (10.0, 1500.0), (20.0, -0.3)]);
        let panel = Panel {
            id: "altitude",
            title: "ALTITUDE",
            ground: true,
            us_unit: "ft",
            us_unit_si: FOOT_M,
            lines: vec![Drawn {
                points: &simulated,
                measured: false,
                width: VERTICAL,
            }],
        };
        assert_eq!(value_axis(&panel).0, 0.0);
        let deep = line(&[(0.0, 0.0), (10.0, 1500.0), (20.0, -30.0)]);
        let panel = Panel {
            lines: vec![Drawn {
                points: &deep,
                measured: false,
                width: VERTICAL,
            }],
            ..panel
        };
        assert!(value_axis(&panel).0 < 0.0);
    }

    /// The data hold every point of every line, the log's two on clocks of their own.
    #[test]
    fn the_data_hold_every_line() {
        let height = line(&[(0.0, 0.0), (1.0, 10.0)]);
        let speed = line(&[(0.0, 0.0), (0.5, 20.0), (1.0, 0.0)]);
        let designed_h = line(&[(0.0, 0.0), (1.0, 12.0)]);
        let designed_s = line(&[(0.0, 1.0), (1.0, 2.0)]);
        let figure = Figure {
            title: "log.csv against its simulation",
            subtitle: "rocket.ork, J350",
            catalog_as_of: "2026-11-01",
            data_file: "out.plot.csv",
            logged_height: &height,
            logged_speed: Some(&speed),
            speed_withheld: None,
            designed: Line {
                height: &designed_h,
                speed: &designed_s,
            },
            launch_day: None,
            apogees: (Some(10.0), Some(12.0), None),
            readings: &[],
            conditions: &[],
            clock: "Clocks.",
            trust: "How far to trust it.",
        };
        let (text, rows) = csv(&figure).unwrap();
        assert_eq!(rows, 2 + 3 + 2);
        assert_eq!(text.lines().count(), 1 + rows);
        assert!(text.contains("logged,0.5,,20.0\r\n"), "{text}");
        assert!(text.contains("as designed,1.0,12.0,2.0\r\n"), "{text}");
        let bad = line(&[(0.0, f64::NAN)]);
        let refused = csv(&Figure {
            logged_height: &bad,
            ..figure
        });
        assert!(refused.unwrap_err().contains("not a number"));
    }
}
