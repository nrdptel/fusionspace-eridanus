use super::*;

use hpr::hpr_sim::{Channel, Recorder};

/// A flight sample's [`Point`] with only its time and height set.
fn point(time_s: f64, height_m: f64) -> Point {
    Point {
        time_s,
        height_m,
        speed_m_s: height_m / 10.0,
        vertical_speed_m_s: -height_m / 20.0,
        acceleration_m_s2: 9.0,
        vertical_acceleration_m_s2: -9.0,
    }
}

/// An event of `kind` at `time_s`, its sample otherwise a rocket at rest.
fn event(kind: EventKind, time_s: f64) -> FlightEvent {
    let sample = Sample {
        time_s,
        phase: hpr::hpr_sim::Phase::Rail,
        state: hpr::hpr_sim::State {
            position_enu_m: hpr::hpr_core::DVec3::ZERO,
            velocity_enu_m_s: hpr::hpr_core::DVec3::ZERO,
            attitude: hpr::hpr_core::DQuat::IDENTITY,
            body_rate_rad_s: hpr::hpr_core::DVec3::ZERO,
        },
        cg_enu_m: hpr::hpr_core::DVec3::ZERO,
        cg_velocity_enu_m_s: hpr::hpr_core::DVec3::ZERO,
        height_above_ground_m: 0.0,
        vertical_speed_m_s: 0.0,
        acceleration_enu_m_s2: hpr::hpr_core::DVec3::ZERO,
        airspeed_m_s: 0.0,
        mach: 0.0,
        angle_of_attack_rad: 0.0,
        dynamic_pressure_pa: 0.0,
        axial_coefficient: 0.0,
        thrust_n: 0.0,
        mass_kg: 1.0,
        recovery_drag_area_m2: 0.0,
    };
    FlightEvent { kind, sample }
}

/// A small figure: a climb to 100 m at 5 s and a fall to the ground at 20 s.
fn figure_svg(title: &str, unpredicted: Option<(f64, f64)>) -> String {
    let points: Vec<Point> = (0..=20)
        .map(|i| {
            let t = f64::from(i);
            point(
                t,
                if t <= 5.0 {
                    20.0 * t
                } else {
                    100.0 - (t - 5.0) * 100.0 / 15.0
                },
            )
        })
        .collect();
    let events = [
        event(EventKind::Liftoff, 0.0),
        event(EventKind::RailExit, 0.05),
        event(EventKind::Apogee, 5.0),
        event(EventKind::Trigger(0), 5.0),
        event(EventKind::Deployment(0), 5.0),
        event(EventKind::GroundHit, 20.0),
    ];
    let devices = ["Drogue".to_owned()];
    svg(&Figure {
        title,
        subtitle: "configuration 1 of 1: [H54-10]",
        points: &points,
        events: &events,
        devices: &devices,
        unpredicted,
        apogee: Some(Reading {
            value: 100.0,
            time_s: 5.0,
            unpredicted: false,
        }),
        top_speed: Some(Reading {
            value: 1234.56,
            time_s: 4.0,
            unpredicted: false,
        }),
    })
}

/// The text of each `<text>` element, in order.
fn texts(document: &roxmltree::Document<'_>) -> Vec<String> {
    document
        .descendants()
        .filter(|node| node.has_tag_name("text"))
        .map(|node| node.text().unwrap_or_default().to_owned())
        .collect()
}

/// The event table's rows: each row's number, time, altitude and first line of its name.
fn table_rows(document: &roxmltree::Document<'_>) -> Vec<[String; 4]> {
    let table = document
        .descendants()
        .find(|node| node.attribute("id") == Some("events"))
        .unwrap();
    let cells: Vec<(String, String, String)> = table
        .children()
        .filter(|node| node.has_tag_name("text"))
        .map(|node| {
            (
                node.attribute("x").unwrap().to_owned(),
                node.attribute("y").unwrap().to_owned(),
                node.text().unwrap_or_default().to_owned(),
            )
        })
        .collect();
    let at = |x: f64, y: &str| {
        cells
            .iter()
            .find(|(cx, cy, _)| cx.parse::<f64>().unwrap() == x && cy == y)
            .map(|(_, _, text)| text.clone())
    };
    let [number_x, time_x, height_x, name_x] = COLUMNS.map(|x| LEFT + x);
    cells
        .iter()
        // The rows are the lines with a number; the head's "NO." is not one.
        .filter(|(x, _, text)| x.parse::<f64>().unwrap() == number_x && text != "NO.")
        .map(|(_, y, number)| {
            [
                number.clone(),
                at(time_x, y).unwrap(),
                at(height_x, y).unwrap(),
                at(name_x, y).unwrap(),
            ]
        })
        .collect()
}

/// A table row from its cells.
fn row(cells: [&str; 4]) -> [String; 4] {
    cells.map(str::to_owned)
}

/// The figure is well-formed XML with the three panels, a numbered marker for each instant with
/// events, the events listed by time, and no shading when nothing is unpredicted.
#[test]
fn the_figure_is_three_panels_and_its_events() {
    let svg = figure_svg("Probe", None);
    let document = roxmltree::Document::parse(&svg).unwrap();
    let ids: Vec<&str> = document
        .descendants()
        .filter_map(|node| node.attribute("id"))
        .collect();
    for id in [
        "altitude",
        "speed",
        "acceleration",
        "event-1",
        "event-2",
        "event-3",
        "event-4",
    ] {
        assert!(ids.contains(&id), "{id}: {ids:?}");
    }
    assert!(!ids.contains(&"event-5"), "{ids:?}");
    assert_eq!(
        table_rows(&document),
        [
            row(["1", "0.00", "0.0", "liftoff"]),
            row(["2", "0.05", "0.0", "rail exit"]),
            row(["3", "5.00", "0.0", "apogee"]),
            row(["3", "5.00", "0.0", "charge for `Drogue`"]),
            row(["3", "5.00", "0.0", "`Drogue` opens"]),
            row(["4", "20.00", "0.0", "ground hit"]),
        ]
    );
    let texts = texts(&document);
    assert!(texts.contains(&"TIME · s".to_owned()), "{texts:?}");
    assert!(!svg.contains("not a prediction"), "{svg}");
    // Liftoff and rail exit, 2.2 px apart, sit in two marker rows.
    let cy = |id: &str| {
        document
            .descendants()
            .find(|node| node.attribute("id") == Some(id))
            .and_then(|group| group.children().find(|node| node.has_tag_name("circle")))
            .and_then(|circle| circle.attribute("cy"))
            .unwrap()
            .to_owned()
    };
    let cy = |id: &str| cy(id).parse::<f64>().unwrap();
    assert_eq!(cy("event-2") - cy("event-1"), MARKER_ROW);
    assert_eq!(cy("event-3"), cy("event-1"));
}

/// Events grouped under one marker keep their own times in the list: one too close to the
/// last to get a marker of its own isn't listed at its time.
#[test]
fn a_grouped_event_keeps_its_time() {
    // On a 600 s axis, half a pixel is about 0.35 s.
    let points = [point(0.0, 0.0), point(520.0, 0.0)];
    let events = [
        event(EventKind::Liftoff, 0.0),
        event(EventKind::RailExit, 0.27),
        event(EventKind::Apogee, 30.0),
        event(EventKind::Trigger(0), 30.0),
        event(EventKind::GroundHit, 520.0),
    ];
    let devices = ["Main".to_owned()];
    let svg = svg(&Figure {
        title: "t",
        subtitle: "s",
        points: &points,
        events: &events,
        devices: &devices,
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    // The markers' tooltips.
    assert!(
        svg.contains("<title>0.00 s: liftoff; 0.27 s: rail exit</title>"),
        "{svg}"
    );
    assert!(
        svg.contains("<title>30.00 s: apogee, charge for `Main`</title>"),
        "{svg}"
    );
    assert!(!svg.contains("event-4"), "{svg}");
    let document = roxmltree::Document::parse(&svg).unwrap();
    assert_eq!(
        table_rows(&document),
        [
            row(["1", "0.00", "0.0", "liftoff"]),
            row(["1", "0.27", "0.0", "rail exit"]),
            row(["2", "30.00", "0.0", "apogee"]),
            row(["2", "30.00", "0.0", "charge for `Main`"]),
            row(["3", "520.00", "0.0", "ground hit"]),
        ]
    );
}

/// The whole figure, byte for byte, for a small flight with a shaded fall; its title block's
/// line, which holds the version, as `[stamp]`.
#[test]
fn a_small_figure() {
    let svg = figure_svg("Probe", Some((5.0, 20.0)));
    let stamp = hpr::hpr_core::tool::stamp();
    assert_eq!(svg.matches(&stamp).count(), 1, "{svg}");
    insta::assert_snapshot!(svg.replace(&stamp, "[stamp]"));
}

/// The figure names the program that drew it, its version and its designation, once, in its
/// metadata (ADR-164).
#[test]
fn the_figure_names_its_tool() {
    let svg = figure_svg("Probe", None);
    let metadata = format!(
        "<metadata>FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001</metadata>",
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(svg.matches(&metadata).count(), 1, "{svg}");
    roxmltree::Document::parse(&svg).unwrap();
}

/// An unpredicted span is hatched in every panel, labelled above the first, outside the
/// hatching, and said under the table.
#[test]
fn an_unpredicted_fall_is_hatched_and_said() {
    let svg = figure_svg("Probe", Some((5.0, 20.0)));
    assert_eq!(svg.matches("fill=\"url(#hatch)\"").count(), 3, "{svg}");
    assert_eq!(svg.matches(">not a prediction<").count(), 1, "{svg}");
    assert!(svg.contains("Hatched, 5.00 s to 20.00 s:"), "{svg}");
    let document = roxmltree::Document::parse(&svg).unwrap();
    let altitude = document
        .descendants()
        .find(|node| node.attribute("id") == Some("altitude"))
        .unwrap();
    let number = |node: roxmltree::Node<'_, '_>, name: &str| -> f64 {
        node.attribute(name).unwrap().parse().unwrap()
    };
    let hatch = altitude
        .children()
        .find(|node| node.attribute("fill") == Some("url(#hatch)"))
        .unwrap();
    let said = altitude
        .children()
        .find(|node| node.text() == Some("not a prediction"))
        .unwrap();
    assert!(number(said, "y") < number(hatch, "y"), "{svg}");
    // A span starting under the axis title is labeled just past the title, not far away.
    let svg = figure_svg("Probe", Some((0.5, 2.0)));
    let document = roxmltree::Document::parse(&svg).unwrap();
    let said = document
        .descendants()
        .find(|node| node.text() == Some("not a prediction"))
        .unwrap();
    assert_eq!(said.attribute("x"), Some("236.0"));
    assert_eq!(said.attribute("text-anchor"), Some("start"));
}

/// A hostile name can't break the document: markup is escaped, control characters dropped and
/// a long name cut.
#[test]
fn a_name_is_escaped_and_cut() {
    let name = format!("<a href=\"x\">&'\u{7}\u{FFFF}</a>{}", "n".repeat(500));
    let svg = figure_svg(&name, None);
    let document = roxmltree::Document::parse(&svg).unwrap();
    let title = document
        .descendants()
        .find(|node| node.has_tag_name("title"))
        .and_then(|node| node.text())
        .unwrap();
    assert!(title.starts_with("<a href=\"x\">&'  </a>nnn"), "{title}");
    // The 20 characters before the `n`s leave room for 60 of them.
    assert_eq!(
        text(&name, NAME_LIMIT)
            .chars()
            .filter(|c| *c == 'n')
            .count(),
        60
    );
    assert!(text(&name, NAME_LIMIT).ends_with('…'));
    assert_eq!(text("a\nb", 10), "a b");
}

/// A long line wraps at spaces within the width, its later lines indented, and a word longer
/// than the width is cut, so wrapping always ends; an escaped name is never split, as lines are
/// wrapped before they are escaped.
#[test]
fn a_long_event_line_wraps() {
    assert_eq!(wrap("short", 10), ["short"]);
    assert_eq!(
        wrap("one two three four", 10),
        ["one two", "    three", "    four"]
    );
    let long = "x".repeat(25);
    let lines = wrap(&long, 10);
    assert!(
        lines.iter().all(|line| line.chars().count() <= 10),
        "{lines:?}"
    );
    assert_eq!(lines.concat().replace(' ', ""), long);
    // A figure whose device names fill several lines stays well-formed and in its width.
    let points = [point(0.0, 0.0), point(1.0, 10.0)];
    let events = [
        event(EventKind::Trigger(0), 0.5),
        event(EventKind::Deployment(0), 0.5),
    ];
    let devices = [format!("A&B {}", "<chute> ".repeat(12))];
    let (title, subtitle) = ("T".repeat(200), "s ".repeat(200));
    let svg = svg(&Figure {
        title: &title,
        subtitle: &subtitle,
        points: &points,
        events: &events,
        devices: &devices,
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    let document = roxmltree::Document::parse(&svg).unwrap();
    // The name column's lines: the first at the column, the later ones indented.
    let name_x = LEFT + COLUMNS[3];
    let column: Vec<&str> = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("text-anchor").is_none())
        .filter(|node| {
            let x = node.attribute("x").unwrap().parse::<f64>().unwrap();
            x == name_x || x == name_x + 20.0
        })
        .filter_map(|node| node.text())
        .collect();
    // The head, then the charge's lines and the opening's.
    assert_eq!(column[0], "EVENT");
    assert!(column.len() >= 5, "{column:?}");
    assert!(
        column
            .iter()
            .all(|line| line.chars().count() <= EVENT_WIDTH),
        "{column:?}"
    );
    assert!(
        column[1].starts_with("charge for `A&B <chute>"),
        "{column:?}"
    );
    // Every line, a long title and subtitle too, sits inside the figure's width at the
    // typeface's 0.6 em a character.
    for node in document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("text-anchor").is_none())
    {
        let x = node.attribute("x").unwrap().parse::<f64>().unwrap();
        let size = node
            .attribute("font-size")
            .map_or(12.0, |size| size.parse::<f64>().unwrap());
        let width = 0.6 * size * node.text().unwrap_or_default().chars().count() as f64;
        assert!(x + width <= WIDTH, "{x} {:?}", node.text());
    }
}

/// Thinning keeps each column's first, least, greatest and last points in order, and breaks
/// the line at a value that isn't finite.
#[test]
fn thinning_keeps_each_columns_extremes() {
    // Column 0: a peak in the middle, then a dip; column 1: one point; then a gap.
    let points = [
        (0.0, 1.0),
        (0.2, 5.0),
        (0.4, 3.0),
        (0.6, -2.0),
        (0.8, 0.0),
        (1.5, 4.0),
        (2.0, f64::NAN),
        (3.0, 1.0),
        (3.5, 1.0),
    ];
    let runs = thin(&points);
    assert_eq!(
        runs,
        vec![
            vec![(0.0, 1.0), (0.2, 5.0), (0.6, -2.0), (0.8, 0.0), (1.5, 4.0)],
            vec![(3.0, 1.0), (3.5, 1.0)],
        ]
    );
    // A long series keeps at most four points a column and its extremes exactly.
    let long: Vec<(f64, f64)> = (0..100_000)
        .map(|i| {
            let x = f64::from(i) * 1e-3;
            (
                x,
                (x * 7.3).sin() * 100.0 + if i == 54_321 { 1000.0 } else { 0.0 },
            )
        })
        .collect();
    let kept: Vec<(f64, f64)> = thin(&long).concat();
    assert!(kept.len() <= 4 * 100, "{}", kept.len());
    let greatest = |points: &[(f64, f64)]| points.iter().map(|p| p.1).fold(f64::MIN, f64::max);
    let least = |points: &[(f64, f64)]| points.iter().map(|p| p.1).fold(f64::MAX, f64::min);
    assert_eq!(greatest(&kept), greatest(&long));
    assert_eq!(least(&kept), least(&long));
    assert_eq!(kept.first(), long.first());
    assert_eq!(kept.last(), long.last());
}

/// Axes round out to 1, 2 or 5 times a power of ten, about five ticks; a flat or empty series
/// takes 0 to 1.
#[test]
fn axes_are_round() {
    assert_eq!(axis([0.0, 846.1].into_iter()), (0.0, 1000.0, 200.0));
    assert_eq!(axis([-14.1, 65.9].into_iter()), (-20.0, 80.0, 20.0));
    assert_eq!(axis([0.0, 53.94].into_iter()), (0.0, 60.0, 20.0));
    let (lo, hi, step) = axis([0.0, 0.3].into_iter());
    assert_eq!((lo, step), (0.0, 0.1));
    assert!((hi - 0.3).abs() < 1e-15, "{hi}");
    assert_eq!(axis([2.0, 2.0].into_iter()), (0.0, 1.0, 0.2));
    assert_eq!(axis(std::iter::empty()), (0.0, 1.0, 0.2));
    assert_eq!(axis([f64::NAN, 3.0, 7.0].into_iter()), (3.0, 7.0, 1.0));
    assert_eq!(axis([-f64::MAX, f64::MAX].into_iter()), (0.0, 1.0, 0.2));
    let ticks: Vec<f64> = (0..).map_while(|k| tick(0.0, 0.3, 0.1, k)).collect();
    assert_eq!(ticks.len(), 4, "{ticks:?}");
    assert_eq!(label(-0.0, 1.0), "0");
    assert_eq!(label(-1e-13, 0.2), "0.0");
    assert_eq!(label(-20.0, 20.0), "\u{2212}20");
    assert_eq!(label(0.30000000000000004, 0.1), "0.3");
    // Rounding out both ends of 0.5 to 10.5 in steps of 2 would leave 7 ticks; steps of 5
    // leave 4.
    assert_eq!(axis([0.5, 10.5].into_iter()), (0.0, 15.0, 5.0));
    // And 0.5 to 25.5 in steps of 5; steps of 10 leave 4.
    assert_eq!(axis([0.5, 25.5].into_iter()), (0.0, 30.0, 10.0));
    // 3 to 6 ticks over spans of every size and offset.
    for span in (1..400).map(|i| f64::from(i) * 0.37) {
        for offset in [-7.3, -0.5, 0.0, 0.5, 3.1, 123.4] {
            let (lo, hi, step) = axis([offset, offset + span].into_iter());
            let ticks = (0..).map_while(|k| tick(lo, hi, step, k)).count();
            assert!((3..=6).contains(&ticks), "{offset} {span}: {ticks}");
            assert!(lo <= offset && hi >= offset + span, "{offset} {span}");
        }
    }
}

/// Numbers read as the product system writes them: a real minus sign, digits grouped from four,
/// and no negative zero.
#[test]
fn numbers_read_in_the_product_systems_style() {
    assert_eq!(number(1234.5, 1), "1,234.5");
    assert_eq!(number(999.95, 1), "1,000.0");
    assert_eq!(number(1234567.0, 0), "1,234,567");
    assert_eq!(number(123.0, 0), "123");
    assert_eq!(number(-1234.56, 2), "\u{2212}1,234.56");
    assert_eq!(number(-0.04, 1), "0.0");
    assert_eq!(number(-0.0, 0), "0");
    assert_eq!(number(0.5, 2), "0.50");
}

/// The text summary, in the figure's `<desc>` and the caption's first line, gives the apogee, its
/// time and the top speed, with the summary's mark for a peak in an unpredicted fall.
#[test]
fn the_figure_describes_its_apogee_and_top_speed() {
    let svg = figure_svg("Probe", None);
    let document = roxmltree::Document::parse(&svg).unwrap();
    let said = "Apogee 100.0 m above the launch site at 5.00 s; top speed 1,234.6 m/s at 4.00 s.";
    let desc = document
        .descendants()
        .find(|node| node.has_tag_name("desc"))
        .unwrap();
    assert_eq!(desc.text(), Some(said));
    assert_eq!(desc.attribute("id"), Some("desc"));
    assert_eq!(
        document.root_element().attribute("aria-describedby"),
        Some("desc")
    );
    assert!(texts(&document).contains(&said.to_owned()), "{svg}");
    let mut figure = Figure {
        title: "t",
        subtitle: "s",
        points: &[],
        events: &[],
        devices: &[],
        unpredicted: None,
        apogee: None,
        top_speed: Some(Reading {
            value: 70.0,
            time_s: 20.0,
            unpredicted: true,
        }),
    };
    assert_eq!(
        summary(&figure),
        "No apogee; top speed 70.0 m/s at 20.00 s, in the fall: not a prediction."
    );
    figure.top_speed = None;
    assert_eq!(summary(&figure), "No apogee; no top speed.");
}

/// The colors the figure may draw in: the light theme's roles of the product system's foundations
/// (Rev A), each by its role's name.
const TOKENS: [(&str, &str); 8] = [
    ("canvas", "#F3F4F7"),
    ("surface", "#FFFFFF"),
    ("rule", "#D6DAE4"),
    ("rule-strong", "#566079"),
    ("ink", "#0B0F1C"),
    ("ink-muted", "#566079"),
    ("ink-faint", "#98A1B8"),
    ("predicted", "#A22488"),
];

/// Every color the figure draws in is a token: each `fill`, `stroke` and color in a `style` is
/// one of [`TOKENS`], `none` or a pattern of the figure's own.
#[test]
fn the_figure_draws_in_token_colors_only() {
    let allowed: Vec<&str> = TOKENS.iter().map(|(_, hex)| *hex).collect();
    for unpredicted in [None, Some((5.0, 20.0))] {
        let svg = figure_svg("Probe", unpredicted);
        let document = roxmltree::Document::parse(&svg).unwrap();
        let mut seen = 0;
        for node in document.descendants().filter(|node| node.is_element()) {
            assert!(node.attribute("style").is_none(), "{node:?}");
            assert!(node.attribute("color").is_none(), "{node:?}");
            for name in [
                "fill",
                "stroke",
                "stop-color",
                "flood-color",
                "lighting-color",
            ] {
                if let Some(value) = node.attribute(name) {
                    assert!(
                        allowed.contains(&value) || value == "none" || value == "url(#hatch)",
                        "{name}={value}"
                    );
                    seen += 1;
                }
            }
        }
        assert!(seen > 50, "{seen}");
        // Nothing else in the document spells a color.
        let hexes = svg
            .match_indices('#')
            .map(|(at, _)| &svg[at..(at + 7).min(svg.len())])
            .filter(|hex| hex[1..].chars().all(|c| c.is_ascii_hexdigit()) && hex.len() == 7);
        for hex in hexes {
            assert!(allowed.contains(&hex), "{hex}");
        }
    }
}

/// [`TOKENS`] are the pinned product system's light roles, when `refs/fusionspace-design` is
/// checked out (the gate's machines; CI has no `refs/`): each role's value in
/// `light.tokens.json`, its reference to a primitive followed.
#[test]
fn the_tokens_are_the_product_systems() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../refs/fusionspace-design/product/tokens");
    let read = |name: &str| -> Option<serde_json::Value> {
        let text = std::fs::read_to_string(root.join(name)).ok()?;
        Some(serde_json::from_str(&text).unwrap())
    };
    let (Some(light), Some(primitives)) =
        (read("light.tokens.json"), read("primitives.tokens.json"))
    else {
        return;
    };
    for (role, hex) in TOKENS {
        let reference = light["color"][role]["$value"].as_str().unwrap();
        let path = reference.trim_matches(|c| c == '{' || c == '}');
        let mut value = &primitives;
        for key in path.split('.') {
            value = &value[key];
        }
        assert_eq!(value["$value"]["hex"].as_str(), Some(hex), "{role}");
    }
}

/// Every simulated series is dashed in the predicted color; the events' lines are dotted; the
/// ground is a chain line in the altitude panel only; each panel's axis title carries its unit;
/// and there is no legend: no line or swatch outside the panels but the markers' and the table's.
#[test]
fn the_lines_follow_the_product_systems_types() {
    let svg = figure_svg("Probe", None);
    let document = roxmltree::Document::parse(&svg).unwrap();
    let paths: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("path"))
        .collect();
    // Altitude's one series, and two each for speed and acceleration.
    assert_eq!(paths.len(), 5);
    for path in &paths {
        assert_eq!(path.attribute("stroke"), Some(PREDICTED));
        assert_eq!(path.attribute("stroke-dasharray"), Some("8 4"));
        assert_eq!(path.attribute("fill"), Some("none"));
    }
    let lines: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("line"))
        .collect();
    let dashed = |pattern: &str| {
        lines
            .iter()
            .filter(|line| line.attribute("stroke-dasharray") == Some(pattern))
            .collect::<Vec<_>>()
    };
    // Each of the 4 markers' lines above the panels, and through each of the 3 panels.
    assert_eq!(dashed("1 3").len(), 4 * 4);
    let chain = dashed("24 3 1 3");
    assert_eq!(chain.len(), 1);
    assert_eq!(
        chain[0].parent().and_then(|panel| panel.attribute("id")),
        Some("altitude")
    );
    for line in &lines {
        let pattern = line.attribute("stroke-dasharray");
        assert!(
            [None, Some("1 3"), Some("24 3 1 3")].contains(&pattern),
            "{pattern:?}"
        );
        assert!(
            ["1", "2"].contains(&line.attribute("stroke-width").unwrap_or("1")),
            "{line:?}"
        );
    }
    let texts = texts(&document);
    for title in [
        "ALTITUDE · m AGL",
        "SPEED · m/s",
        "ACCELERATION · m/s²",
        "TIME · s",
    ] {
        assert!(texts.contains(&title.to_owned()), "{title}: {texts:?}");
    }
    // Lines outside the panels: the markers' and the table's rules, none of them a swatch.
    let outside = lines
        .iter()
        .filter(|line| {
            !line.ancestors().any(|node| {
                node.attribute("id")
                    .is_some_and(|id| id.starts_with("event-") || id == "events")
                    || PANELS
                        .iter()
                        .any(|panel| node.attribute("id") == Some(panel.id))
                    || node.has_tag_name("pattern")
            })
        })
        .count();
    assert_eq!(outside, 0, "{svg}");
    // Nor a swatch of any other shape: the only rectangles are the canvas, the plot areas and
    // the clips, and the only circles the balloons.
    let rects: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("rect"))
        .collect();
    let in_clip = rects
        .iter()
        .filter(|rect| {
            rect.parent()
                .is_some_and(|node| node.has_tag_name("clipPath"))
        })
        .count();
    let canvas = rects
        .iter()
        .filter(|rect| rect.attribute("fill") == Some(CANVAS))
        .count();
    let areas = rects
        .iter()
        .filter(|rect| rect.attribute("fill") == Some(SURFACE))
        .count();
    assert_eq!((in_clip, canvas, areas), (3, 1, 3));
    assert_eq!(rects.len(), 7, "{svg}");
    let circles: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("circle"))
        .collect();
    assert_eq!(circles.len(), 4);
    for circle in circles {
        let group = circle.parent().and_then(|node| node.attribute("id"));
        assert!(
            group.is_some_and(|id| id.starts_with("event-")),
            "{group:?}"
        );
    }
}

/// The trace holds every point the recorder keeps, with the same height, vertical speed and
/// acceleration, in time order, and a step's start after an event: so a parachute's opening
/// shock is drawn at its full height, within 1% of the summary's peak, found inside steps.
#[test]
fn the_trace_samples_the_recorders_rows_and_each_step() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let bytes =
        std::fs::read(root.join("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork")).unwrap();
    let file = hpr::hpr_io::ork::read(&bytes).unwrap().value;
    let read = hpr::hpr_io::ork::design(&file).value;
    let mut design = read.rocket.clone();
    let configuration = read.motors.default_configuration().unwrap();
    let id = configuration.id.clone();
    let mount = configuration.motors[0].mount.clone();
    let motor = hpr::Motor::from_catalog("H54").unwrap();
    design.configurations.retain(|c| c.id != id);
    design.configurations.push(hpr::hpr_design::Configuration {
        id: id.clone(),
        name: String::new(),
        motors: vec![hpr::hpr_design::MountedMotor {
            mount,
            designation: motor.designation().to_owned(),
            diameter_m: motor.diameter_m(),
            length_m: motor.length_m(),
            motor: motor.solid_motor().clone(),
            delay: motor.delay(),
            ignition: hpr::hpr_design::Ignition::Launch,
            failed_tubes: Vec::new(),
        }],
    });
    let recovery = read.recovery;
    let assembly = design.assemble(&id).unwrap();
    let devices = hpr::ork::recovery(&recovery, &design, &assembly)
        .unwrap()
        .devices;
    let mut rocket = hpr::Rocket::from_design(design, &id).unwrap();
    for device in devices {
        rocket.add_parachute(device);
    }
    let environment = hpr::Environment::new(0.0, 0.0, 0.0).unwrap();
    let channels = vec![
        Channel::Time,
        Channel::HeightAboveGround,
        Channel::VerticalSpeed,
        Channel::Acceleration,
    ];
    let mut recorder = Recorder::new(channels, Some(0.01)).unwrap();
    let mut trace = Trace::new(0.01);
    let flight = hpr::Flight::builder(&rocket, &environment, 1.5)
        .fly_with(&mut (&mut recorder, &mut trace))
        .unwrap();
    let points = trace.points();
    assert!(points.len() > recorder.rows().len(), "{}", points.len());
    assert!(points.windows(2).all(|w| w[0].time_s <= w[1].time_s));
    for row in recorder.rows() {
        let acceleration = hpr::hpr_core::DVec3::new(row[3], row[4], row[5]).length();
        let row = [row[0], row[1], row[2], acceleration, row[5]];
        assert!(
            points.iter().any(|p| [
                p.time_s,
                p.height_m,
                p.vertical_speed_m_s,
                p.acceleration_m_s2,
                p.vertical_acceleration_m_s2
            ] == row),
            "{row:?}"
        );
    }
    let greatest = |value: fn(&Point) -> f64| points.iter().map(value).fold(0.0, f64::max);
    let summary = flight.summary();
    for (plotted, peak) in [
        (greatest(|p| p.speed_m_s), summary.max_speed_m_s),
        (
            greatest(|p| p.acceleration_m_s2),
            summary.max_descent_acceleration_m_s2,
        ),
    ] {
        let peak = peak.unwrap().value;
        assert!(plotted <= peak && plotted > peak * 0.99, "{plotted} {peak}");
    }
}
