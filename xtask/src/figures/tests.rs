use super::*;

/// The repository's root.
fn root() -> PathBuf {
    crate::designs::root().unwrap()
}

/// A committed figure under `docs/images/`.
fn committed(name: &str) -> String {
    std::fs::read_to_string(root().join("docs/images").join(name)).unwrap()
}

/// `body` in a 100 × 100 figure.
fn figure(body: &str) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">{body}</svg>")
}

/// The problems of `svg` of `kind` whose message holds `needle`.
fn found(svg: &str, kind: Kind, needle: &str) -> usize {
    check(svg)
        .iter()
        .filter(|p| p.kind == kind && p.message.contains(needle))
        .count()
}

/// Asserts `svg` passes the check.
#[track_caller]
fn passes(svg: &str) {
    let problems = check(svg);
    assert!(problems.is_empty(), "{problems:#?}\n{svg}");
}

/// Asserts `svg` has exactly the one problem, of `kind`, naming `needle`.
#[track_caller]
fn fails(svg: &str, kind: Kind, needle: &str) {
    let problems = check(svg);
    assert!(
        problems.len() == 1 && problems[0].kind == kind && problems[0].message.contains(needle),
        "expected one {kind:?} naming `{needle}`, got {problems:#?}\n{svg}"
    );
}

/// `text` with `from` replaced by `to`, which must be there once.
fn replaced(text: &str, from: &str, to: &str) -> String {
    assert_eq!(text.matches(from).count(), 1, "`{from}`");
    text.replace(from, to)
}

/// Every figure under `docs/` draws inside its frame and its clips, with no text over text.
#[test]
fn every_figure_under_docs_fits() {
    let checked = check_docs(&root()).unwrap_or_else(|e| panic!("{e}"));
    assert!(checked >= 4, "only {checked} SVGs found under docs/");
}

/// A label of a committed figure moved past its right edge fails, naming it; where it was, it
/// passes.
#[test]
fn a_label_moved_out_of_a_figure_fails() {
    let svg = committed("flight-phases.svg");
    passes(&svg);
    let east = "<text x=\"796\" y=\"414\" fill=\"#555\" text-anchor=\"end\">east</text>";
    let moved = replaced(
        &svg,
        east,
        "<text x=\"850\" y=\"414\" fill=\"#555\" text-anchor=\"end\">east</text>",
    );
    fails(&moved, Kind::OutsideFrame, "\"east\" reaches");
    // A legend line shifted right until its end passes the edge fails too.
    let legend = "<text x=\"540\" y=\"271\">The main opens";
    let moved = replaced(&svg, legend, "<text x=\"560\" y=\"271\">The main opens");
    fails(
        &moved,
        Kind::OutsideFrame,
        "\"The main opens, after its height and lag\"",
    );
}

/// Two labels of a committed figure moved onto each other fail, naming both.
#[test]
fn two_labels_overlapping_fail() {
    let svg = committed("flight-phases.svg");
    let moved = replaced(
        &svg,
        "<text x=\"540\" y=\"135\">Rail exit",
        "<text x=\"540\" y=\"110\">Rail exit",
    );
    let problems = check(&moved);
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert_eq!(problems[0].kind, Kind::Overlap);
    assert!(
        problems[0]
            .message
            .contains("\"Liftoff: it starts up the rail\"")
            && problems[0]
                .message
                .contains("\"Rail exit: the rocket flies free\""),
        "{}",
        problems[0].message
    );
    // Lines a line apart don't overlap.
    passes(&figure(
        "<text x=\"5\" y=\"20\" font-size=\"12\" font-family=\"monospace\">first line</text>\
         <text x=\"5\" y=\"38\" font-size=\"12\" font-family=\"monospace\">second line</text>",
    ));
}

/// A transform on the way down moves a mark out of the frame.
#[test]
fn a_transform_moves_a_mark_out() {
    let rect = "<rect x=\"10\" y=\"10\" width=\"40\" height=\"10\"/>";
    passes(&figure(rect));
    fails(
        &figure(&format!("<g transform=\"translate(60 0)\">{rect}</g>")),
        Kind::OutsideFrame,
        "<rect>",
    );
    // Turned about the middle, a square's corners reach past the frame.
    let square = "<rect x=\"10\" y=\"10\" width=\"80\" height=\"80\"/>";
    passes(&figure(square));
    fails(
        &figure(&format!("<g transform=\"rotate(45, 50, 50)\">{square}</g>")),
        Kind::OutsideFrame,
        "<rect>",
    );
    // A scale composes with the translate before it.
    fails(
        &figure(
            "<g transform=\"translate(30) scale(2)\"><circle cx=\"30\" cy=\"30\" r=\"10\"/></g>",
        ),
        Kind::OutsideFrame,
        "x 70.00 to 110.00, y 40.00 to 80.00",
    );
}

/// Half a stroke's width out of the frame fails; a butt cap at the edge doesn't reach past it,
/// and a square cap does.
#[test]
fn half_a_stroke_out_fails() {
    passes(&figure(
        "<rect x=\"0\" y=\"10\" width=\"50\" height=\"10\"/>",
    ));
    fails(
        &figure("<rect x=\"0\" y=\"10\" width=\"50\" height=\"10\" stroke=\"#000\"/>"),
        Kind::OutsideFrame,
        "x -0.50 to 50.50",
    );
    let line = "<line x1=\"10\" y1=\"0\" x2=\"10\" y2=\"50\" stroke=\"#000\" stroke-width=\"4\"";
    passes(&figure(&format!("{line}/>")));
    fails(
        &figure(&format!("{line} stroke-linecap=\"square\"/>")),
        Kind::OutsideFrame,
        "y -2.00 to 52.00",
    );
    // Inherited from a group, and none where the group says none.
    fails(
        &figure(
            "<g stroke=\"#000\" stroke-width=\"4\"><line x1=\"1\" y1=\"1\" x2=\"9\" y2=\"1\"/></g>",
        ),
        Kind::OutsideFrame,
        "<line>",
    );
    passes(&figure(
        "<g stroke=\"#000\" stroke-width=\"4\"><line x1=\"1\" y1=\"1\" x2=\"9\" y2=\"1\" \
         stroke=\"none\"/></g>",
    ));
}

/// A sharp miter join reaches past the frame while the miter limit allows it; beveled past the
/// limit, it stays in.
#[test]
fn a_miter_join_reaches_out_unless_beveled() {
    // The apex's angle is 2·atan(10/80), 14.3°: a miter 8.1 times the stroke's width, past the
    // default limit of 4, so beveled.
    let polyline = "<polyline points=\"40,90 50,10 60,90\" fill=\"none\" stroke=\"#000\" \
                    stroke-width=\"4\"";
    passes(&figure(&format!("{polyline}/>")));
    fails(
        &figure(&format!("{polyline} stroke-miterlimit=\"10\"/>")),
        Kind::OutsideFrame,
        "<polyline>",
    );
    passes(&figure(&format!(
        "{polyline} stroke-miterlimit=\"10\" stroke-linejoin=\"round\"/>"
    )));
    // A path's join between a line and a curve is held to the same rule.
    let path = "<path d=\"M40 90 L50 10 C55 50 58 70 60 90\" fill=\"none\" stroke=\"#000\" \
                stroke-width=\"4\"";
    passes(&figure(&format!("{path}/>")));
    fails(
        &figure(&format!("{path} stroke-miterlimit=\"20\"/>")),
        Kind::OutsideFrame,
        "<path>",
    );
}

/// A marker at a line's end reaches as far as its viewport's farthest corner from its
/// reference point, in stroke widths; its content must fit that viewport.
#[test]
fn a_marker_reaches_out() {
    // A 4 × 4 viewport, its reference at the middle: 2√2 stroke widths, 5.66 px at width 2.
    let marker = "<defs><marker id=\"m\" viewBox=\"0 0 10 10\" refX=\"5\" refY=\"5\" \
                  markerWidth=\"4\" markerHeight=\"4\"><circle cx=\"5\" cy=\"5\" r=\"5\"/>\
                  </marker></defs>";
    let line = |x2: u32| {
        format!(
            "{marker}<line x1=\"10\" y1=\"50\" x2=\"{x2}\" y2=\"50\" stroke=\"#000\" \
             stroke-width=\"2\" marker-end=\"url(#m)\"/>"
        )
    };
    passes(&figure(&line(94)));
    fails(&figure(&line(95)), Kind::OutsideFrame, "x 10.00 to 100.66");
    // The marker's content past its viewport is cut off.
    let clipped = replaced(&line(90), "r=\"5\"", "r=\"6\"");
    fails(
        &figure(&clipped),
        Kind::Clipped,
        "the viewport of line 1 <marker id=\"m\">",
    );
}

/// An arc whose endpoints are inside the frame but whose bulge isn't fails.
#[test]
fn an_arcs_bulge_counts() {
    // A half circle of radius 40 from (10, 30) to (90, 30) over the top reaches y = −10.
    fails(
        &figure("<path d=\"M10 30 A40 40 0 0 1 90 30\"/>"),
        Kind::OutsideFrame,
        "y -10.00 to 70.00",
    );
    passes(&figure("<path d=\"M10 50 A40 40 0 0 1 90 50\"/>"));
    // Radii too small to span the endpoints are scaled up: here to the same half circle.
    fails(
        &figure("<path d=\"M10 30 a1 1 0 0 1 80 0\"/>"),
        Kind::OutsideFrame,
        "y -10.00 to 70.00",
    );
}

/// A curve is held to its control points, and relative and implicit commands are followed.
#[test]
fn a_path_is_read_whole() {
    fails(
        &figure("<path d=\"M10 10 C10 -20 90 -20 90 10\"/>"),
        Kind::OutsideFrame,
        "y -20.00 to 10.00",
    );
    // m, then implicit relative linetos, h and v: to (10,10), (30,10), (30,30), (95,30), (95,105).
    fails(
        &figure("<path d=\"m10 10 20 0 0 20h65v75\"/>"),
        Kind::OutsideFrame,
        "x 10.00 to 95.00, y 10.00 to 105.00",
    );
    // S reflects the last control point, (60, 60) about (90, 90) to (120, 120), out of the
    // frame though the points it writes are all inside.
    passes(&figure(
        "<path d=\"M10 10 C20 20 60 60 90 90 C90 90 80 85 85 85\"/>",
    ));
    fails(
        &figure("<path d=\"M10 10 C20 20 60 60 90 90 S80 85 85 85\"/>"),
        Kind::OutsideFrame,
        "x 10.00 to 120.00, y 10.00 to 120.00",
    );
    for bad in ["10 10 L 20", "M10 10 L20", "M10 10 X 5 5", "M1e 2"] {
        assert_eq!(
            found(
                &figure(&format!("<path d=\"{bad}\"/>")),
                Kind::Malformed,
                "d doesn't parse"
            ),
            1,
            "{bad}"
        );
    }
}

/// `text-anchor` places a line: ending at x near the left edge pushes it out to the left.
#[test]
fn text_anchor_end_pushes_text_left() {
    let text = |anchor: &str| {
        figure(&format!(
            "<text x=\"20\" y=\"50\" font-family=\"Arial\" font-size=\"10\" \
             text-anchor=\"{anchor}\">far too long</text>"
        ))
    };
    passes(&text("start"));
    fails(
        &text("end"),
        Kind::OutsideFrame,
        "\"far too long\" reaches x -",
    );
    // Inherited from a group.
    let grouped = figure(
        "<g text-anchor=\"middle\"><text x=\"5\" y=\"50\" font-family=\"Arial\" \
         font-size=\"10\">wide</text></g>",
    );
    fails(&grouped, Kind::OutsideFrame, "\"wide\" reaches x -");
}

/// A line of text is at least as wide as the widest font draws it: Verdana's `W` is 2025
/// font units of 2048 wide, so at 100 px no table may hold it under 98.9 px.
#[test]
fn the_glyph_tables_are_sorted_and_hold_known_widths() {
    for table in [
        glyphs::PROPORTIONAL,
        glyphs::PROPORTIONAL_BOLD,
        glyphs::MONOSPACE,
        glyphs::MONOSPACE_BOLD,
    ] {
        assert!(table.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(table.len() > 250, "{}", table.len());
    }
    let advance = |table: Table, c: char| {
        let i = table.binary_search_by(|(k, _)| k.cmp(&c)).unwrap();
        table[i].1[0]
    };
    assert!(advance(glyphs::PROPORTIONAL, 'W') >= 989);
    // Every monospace font measured is at least 0.6 em a character but Consolas, 0.55.
    assert!(advance(glyphs::MONOSPACE, 'i') >= 600);
    assert!(advance(glyphs::PROPORTIONAL_BOLD, 'W') >= advance(glyphs::PROPORTIONAL, 'W'));
}

/// A `dominant-baseline` that lifts the baseline is measured as far as an em below `y`; one the
/// check doesn't know fails.
#[test]
fn a_baseline_moves_text_down() {
    let text = |baseline: &str| {
        figure(&format!(
            "<text x=\"10\" y=\"92\" font-family=\"Arial\" font-size=\"10\"{baseline}>text</text>"
        ))
    };
    passes(&text(""));
    fails(
        &text(" dominant-baseline=\"hanging\""),
        Kind::OutsideFrame,
        "\"text\"",
    );
    fails(
        &text(" dominant-baseline=\"mathematical\""),
        Kind::Unsupported,
        "dominant-baseline `mathematical`",
    );
}

/// What the check can't measure fails rather than passes.
#[test]
fn what_the_check_cant_measure_fails() {
    for (body, needle) in [
        (
            "<use href=\"#a\"/>",
            "<use>: an element the figure check can't measure",
        ),
        (
            "<image width=\"10\" height=\"10\"/>",
            "<image>: an element the figure check can't measure",
        ),
        (
            "<style>text { font-size: 40px }</style>",
            "<style>: an element the figure check can't measure",
        ),
        (
            "<foreignObject width=\"10\" height=\"10\"/>",
            "<foreignObject>: an element the figure check can't measure",
        ),
        (
            "<svg width=\"10\" height=\"10\"/>",
            "<svg>: an element the figure check can't measure",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Arial\">a<tspan dy=\"5\">b</tspan></text>",
            "<tspan>: an element inside <text>",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Arial\" letter-spacing=\"9\">a</text>",
            "attribute `letter-spacing`",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Arial\" textLength=\"90\">a</text>",
            "attribute `textLength`",
        ),
        (
            "<line x1=\"5\" y1=\"5\" x2=\"9\" y2=\"9\" stroke=\"#000\" vector-effect=\"non-scaling-stroke\"/>",
            "attribute `vector-effect`",
        ),
        (
            "<rect width=\"5\" height=\"5\" style=\"stroke-width: 20\"/>",
            "style sets `stroke-width`",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Arial\" font-size=\"2em\">a</text>",
            "font-size `2em`",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Comic Sans MS\">a</text>",
            "font family `Comic Sans MS`",
        ),
        (
            "<g dominant-baseline=\"middle\"/>",
            "dominant-baseline set on a container",
        ),
        (
            "<rect width=\"5\" height=\"5\" stroke=\"#000\" stroke-linejoin=\"arcs\"/>",
            "stroke-linejoin `arcs`",
        ),
    ] {
        assert_eq!(
            found(&figure(body), Kind::Unsupported, needle),
            1,
            "{body}\n{:#?}",
            check(&figure(body))
        );
    }
    // A root with no frame, and one that isn't SVG.
    assert_eq!(
        found(
            "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
            Kind::Malformed,
            "no viewBox"
        ),
        1
    );
    assert_eq!(found("<svg/>", Kind::Unsupported, "not an SVG"), 1);
    // Without a viewBox, the width and height frame it.
    passes(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\"><rect width=\"20\" height=\"20\"/></svg>",
    );
    assert_eq!(
        found(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\"><rect width=\"21\" height=\"20\"/></svg>",
            Kind::OutsideFrame,
            "the viewBox (x 0.00 to 20.00"
        ),
        1
    );
}

/// A character the glyph table lacks fails, naming it.
#[test]
fn an_unknown_character_fails() {
    fails(
        &figure("<text x=\"5\" y=\"50\" font-family=\"Arial\">a 漢 b</text>"),
        Kind::Character,
        "U+6F22",
    );
    passes(&figure(
        "<text x=\"5\" y=\"50\" font-family=\"Arial\" font-size=\"8\">−20 m/s² · 5 °C</text>",
    ));
}

/// What a clip holds must lie inside its rectangle, and the rectangle inside the frame.
#[test]
fn a_clip_cuts_nothing() {
    let clip = "<clipPath id=\"c\"><rect x=\"10\" y=\"10\" width=\"50\" height=\"50\"/></clipPath>";
    passes(&figure(&format!(
        "{clip}<rect x=\"20\" y=\"20\" width=\"40\" height=\"10\" clip-path=\"url(#c)\"/>"
    )));
    fails(
        &figure(&format!(
            "{clip}<rect x=\"20\" y=\"20\" width=\"41\" height=\"10\" clip-path=\"url(#c)\"/>"
        )),
        Kind::Clipped,
        "outside its clip, line 1 <clipPath id=\"c\">",
    );
    // Half a stroke past the clip is cut off too, and a group's clip holds its content.
    fails(
        &figure(&format!(
            "{clip}<g clip-path=\"url(#c)\"><line x1=\"10\" y1=\"30\" x2=\"60\" y2=\"30\" \
             stroke=\"#000\" stroke-linecap=\"round\"/></g>"
        )),
        Kind::Clipped,
        "<line>",
    );
    // The clip's rectangle out of the frame.
    let wide =
        "<clipPath id=\"c\"><rect x=\"-10\" y=\"10\" width=\"50\" height=\"50\"/></clipPath>";
    fails(
        &figure(&format!(
            "{wide}<rect x=\"20\" y=\"20\" width=\"10\" height=\"10\" clip-path=\"url(#c)\"/>"
        )),
        Kind::OutsideFrame,
        "(the clip of line 1 <rect>)",
    );
    // Anything but one rectangle isn't measured.
    let circle = "<clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"20\"/></clipPath>";
    fails(
        &figure(&format!(
            "{circle}<rect width=\"5\" height=\"5\" clip-path=\"url(#c)\"/>"
        )),
        Kind::Unsupported,
        "a clipPath of anything but one <rect>",
    );
}

/// A pattern's content must fit its tile: the plot's hatching once drew its line on the tile's
/// edge, where the tile cut off half its width.
#[test]
fn a_pattern_tile_cuts_nothing() {
    let pattern = |x: u32| {
        figure(&format!(
            "<defs><pattern id=\"h\" width=\"6\" height=\"6\" patternUnits=\"userSpaceOnUse\" \
             patternTransform=\"rotate(45)\"><line x1=\"{x}\" y1=\"0\" x2=\"{x}\" y2=\"6\" \
             stroke=\"#000\" stroke-width=\"1\"/></pattern></defs>\
             <rect width=\"50\" height=\"50\" fill=\"url(#h)\"/>"
        ))
    };
    passes(&pattern(3));
    fails(
        &pattern(0),
        Kind::Clipped,
        "the tile of line 1 <pattern id=\"h\">",
    );
}

/// A closepath back to where the path already is joins nothing: the corner's miter is the one
/// its two real sides make, 45° here, 2.61 half-widths, not the limit's 4.
#[test]
fn a_closepath_with_no_length_joins_its_neighbours() {
    let path = |width: &str| {
        figure(&format!(
            "<path d=\"M3 50 L50 3 L50 50 L3 50 Z\" fill=\"none\" stroke=\"#000\" \
             stroke-width=\"{width}\"/>"
        ))
    };
    passes(&path("2"));
    fails(&path("2.4"), Kind::OutsideFrame, "x -0.14 to");
}

/// A clip the check can't place fails: one on the root `<svg>`, whose user space browsers read
/// differently (its viewBox's or its viewport's), and one on a clipPath's rectangle, which cuts
/// the clip itself.
#[test]
fn a_clip_the_check_cant_place_fails() {
    let clip = "<clipPath id=\"c\"><rect x=\"10\" y=\"10\" width=\"50\" height=\"50\"/></clipPath>";
    fails(
        &format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" \
             clip-path=\"url(#c)\">{clip}<rect x=\"20\" y=\"20\" width=\"10\" height=\"10\"/></svg>"
        ),
        Kind::Unsupported,
        "a clip-path on the root",
    );
    let nested = "<clipPath id=\"d\"><rect x=\"0\" y=\"0\" width=\"20\" height=\"20\"/></clipPath>\
                  <clipPath id=\"c\"><rect x=\"10\" y=\"10\" width=\"50\" height=\"50\" \
                  clip-path=\"url(#d)\"/></clipPath>";
    fails(
        &figure(&format!(
            "{nested}<rect x=\"20\" y=\"20\" width=\"10\" height=\"10\" clip-path=\"url(#c)\"/>"
        )),
        Kind::Unsupported,
        "a clip-path on the rectangle of",
    );
}

/// A length that can't be negative, written negative, fails: a browser draws in the inherited or
/// default value instead, or draws nothing, and the check can't know which.
#[test]
fn a_negative_size_fails() {
    for (body, needle) in [
        (
            "<line x1=\"10\" y1=\"10\" x2=\"20\" y2=\"10\" stroke=\"#000\" stroke-width=\"-4\"/>",
            "stroke-width `-4` is negative",
        ),
        (
            "<text x=\"5\" y=\"50\" font-family=\"Arial\" font-size=\"-10\">a</text>",
            "font-size `-10` is negative",
        ),
        (
            "<rect width=\"-5\" height=\"5\"/>",
            "width `-5` is negative",
        ),
        (
            "<circle cx=\"50\" cy=\"50\" r=\"-5\"/>",
            "r `-5` is negative",
        ),
        (
            "<ellipse cx=\"50\" cy=\"50\" rx=\"5\" ry=\"-5\"/>",
            "ry `-5` is negative",
        ),
        (
            "<defs><marker id=\"m\" markerWidth=\"-3\"><circle r=\"1\"/></marker></defs>",
            "markerWidth `-3` is negative",
        ),
        (
            "<defs><pattern id=\"h\" width=\"-6\" height=\"6\" patternUnits=\"userSpaceOnUse\">\
             <rect width=\"1\" height=\"1\"/></pattern></defs>",
            "width `-6` is negative",
        ),
        (
            "<rect width=\"5\" height=\"5\" stroke=\"#000\" stroke-miterlimit=\"0.5\"/>",
            "stroke-miterlimit `0.5` is below 1",
        ),
    ] {
        assert_eq!(
            found(&figure(body), Kind::Malformed, needle),
            1,
            "{body}\n{:#?}",
            check(&figure(body))
        );
    }
}

/// A newline inside `<text>` draws as a space, as a browser lays text out (CSS's
/// `white-space: normal`), so `W`, a newline and `W` is as wide as `W W`.
#[test]
fn a_newline_in_text_draws_as_a_space() {
    assert_eq!(collapse("W\nW", false), "W W");
    assert_eq!(collapse("\n  W\r\n\tW  \n", false), "W W");
    let text = |content: &str| {
        figure(&format!(
            "<text x=\"80\" y=\"50\" font-family=\"Arial\" font-size=\"10\">{content}</text>"
        ))
    };
    let spaced = check(&text("W W"));
    assert!(!spaced.is_empty());
    assert_eq!(check(&text("W\nW")), spaced);
}

/// Text in no named family draws in the default serif, Times on a Mac, whose Greek capitals are
/// wider than the other families'; ten Alphas at 100 px don't fit 750 px.
#[test]
fn the_default_serif_is_measured() {
    fails(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 760 120\"><text x=\"10\" \
         y=\"100\" font-size=\"100\">ΑΑΑΑΑΑΑΑΑΑ</text></svg>",
        Kind::OutsideFrame,
        "\"ΑΑΑΑΑΑΑΑΑΑ\" reaches",
    );
    assert!(glyphs::PROPORTIONAL_FAMILIES.contains(&"Times"));
    // `system-ui` is Segoe UI on Windows, which the table doesn't measure.
    fails(
        &figure("<text x=\"5\" y=\"50\" font-family=\"system-ui\">a</text>"),
        Kind::Unsupported,
        "font family `system-ui`",
    );
}
