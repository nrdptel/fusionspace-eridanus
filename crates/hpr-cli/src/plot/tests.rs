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
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
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

/// The event table's rows: each row's number, time, altitude in meters and in feet, and first
/// line of its name.
fn table_rows(document: &roxmltree::Document<'_>) -> Vec<[String; 5]> {
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
    let [number_x, time_x, height_x, feet_x, name_x] = COLUMNS.map(|x| LEFT + x);
    cells
        .iter()
        // The rows are the lines with a number; the head's "NO." is not one.
        .filter(|(x, _, text)| x.parse::<f64>().unwrap() == number_x && text != "NO.")
        .map(|(_, y, number)| {
            [
                number.clone(),
                at(time_x, y).unwrap(),
                at(height_x, y).unwrap(),
                at(feet_x, y).unwrap(),
                at(name_x, y).unwrap(),
            ]
        })
        .collect()
}

/// A table row from its cells.
fn row(cells: [&str; 5]) -> [String; 5] {
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
            row(["1", "0.00", "0.0", "0", "liftoff"]),
            row(["2", "0.05", "0.0", "0", "rail exit"]),
            row(["3", "5.00", "0.0", "0", "apogee"]),
            row(["3", "5.00", "0.0", "0", "charge for `Drogue`"]),
            row(["3", "5.00", "0.0", "0", "`Drogue` opens"]),
            row(["4", "20.00", "0.0", "0", "ground hit"]),
        ]
    );
    let texts = texts(&document);
    assert!(texts.contains(&"TIME · s".to_owned()), "{texts:?}");
    assert!(!svg.contains("not a prediction"), "{svg}");
    // Liftoff and rail exit, 2.2 px apart, sit in one row, rail exit's balloon stepped right.
    let at = |id: &str, name: &str| {
        document
            .descendants()
            .find(|node| node.attribute("id") == Some(id))
            .and_then(|group| group.children().find(|node| node.has_tag_name("circle")))
            .and_then(|circle| circle.attribute(name))
            .unwrap()
            .parse::<f64>()
            .unwrap()
    };
    assert_eq!(at("event-2", "cy"), at("event-1", "cy"));
    assert_eq!(at("event-3", "cy"), at("event-1", "cy"));
    assert_eq!(at("event-2", "cx") - at("event-1", "cx"), MARKER_ROW);
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
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &points,
        events: &events,
        devices: &devices,
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    // The markers' tooltips.
    assert!(
        svg.contains("<title>0.00\u{a0}s: liftoff; 0.27\u{a0}s: rail exit</title>"),
        "{svg}"
    );
    assert!(
        svg.contains("<title>30.00\u{a0}s: apogee, charge for `Main`</title>"),
        "{svg}"
    );
    assert!(!svg.contains("event-4"), "{svg}");
    let document = roxmltree::Document::parse(&svg).unwrap();
    assert_eq!(
        table_rows(&document),
        [
            row(["1", "0.00", "0.0", "0", "liftoff"]),
            row(["1", "0.27", "0.0", "0", "rail exit"]),
            row(["2", "30.00", "0.0", "0", "apogee"]),
            row(["2", "30.00", "0.0", "0", "charge for `Main`"]),
            row(["3", "520.00", "0.0", "0", "ground hit"]),
        ]
    );
}

/// The whole figure, byte for byte, for a small flight with a shaded fall; what holds the
/// version as `[stamp]`, in its metadata, and `[program]`, in its title block.
#[test]
fn a_small_figure() {
    let svg = figure_svg("Probe", Some((5.0, 20.0)));
    let stamp = hpr::hpr_core::tool::stamp();
    assert_eq!(svg.matches(&stamp).count(), 1, "{svg}");
    let svg = svg.replace(&stamp, "[stamp]");
    let program = format!(
        "{} {}",
        hpr::hpr_core::tool::NAME,
        hpr::hpr_core::tool::VERSION
    );
    assert_eq!(svg.matches(&program).count(), 1, "{svg}");
    insta::assert_snapshot!(svg.replace(&program, "[program]"));
}

/// The figure ends with how far to trust it, under a label set as its other labels are, as
/// `hpr sim`'s text does (ADR-211), after the hatching's note when there is one.
#[test]
fn the_figure_ends_with_how_far_to_trust_it() {
    for unpredicted in [None, Some((5.0, 20.0))] {
        let svg = figure_svg("Probe", unpredicted);
        let label = format!(">{TRUST_LABEL}</text>");
        assert_eq!(svg.matches(&label).count(), 1, "{svg}");
        let note = svg.find(&label).unwrap_or(usize::MAX);
        assert!(svg.find("EVENTS · SIMULATED").unwrap_or(0) < note);
        if unpredicted.is_some() {
            assert!(svg.find("Hatched,").unwrap_or(usize::MAX) < note);
        }
        // Nothing but the note's lines after it, then the title block.
        let after = &svg[note..];
        assert!(
            after.contains(">Simulated from the design file, not measured."),
            "{after}"
        );
        assert!(after.contains("55 logged flights"), "{after}");
        assert!(
            after.contains("accuracy.html</text>\n<g id=\"title-block\">"),
            "{after}"
        );
        assert_eq!(after.matches("<g ").count(), 1, "{after}");
        assert!(after.ends_with("</g>\n</svg>\n"), "{after}");
    }
}

/// The title block's cells, in order: each key, its value, and the key's x and baseline. A key
/// is muted and its value, in ink, on the next line at the same x.
fn title_block(document: &roxmltree::Document<'_>) -> Vec<(String, String, f64, f64)> {
    let block = document
        .descendants()
        .find(|node| node.attribute("id") == Some("title-block"))
        .unwrap();
    let texts: Vec<_> = block
        .children()
        .filter(|node| node.has_tag_name("text"))
        .collect();
    let number = |node: &roxmltree::Node<'_, '_>, name: &str| -> f64 {
        node.attribute(name).unwrap().parse().unwrap()
    };
    texts
        .chunks(2)
        .map(|pair| {
            let [key, value] = pair else {
                panic!("a key without its value: {pair:?}")
            };
            assert_eq!(key.attribute("fill"), Some(INK_MUTED));
            assert_eq!(value.attribute("fill"), Some(INK));
            assert_eq!(number(key, "x"), number(value, "x"));
            assert_eq!(number(value, "y") - number(key, "y"), 16.0);
            (
                key.text().unwrap().to_owned(),
                value.text().unwrap_or_default().to_owned(),
                number(key, "x"),
                number(key, "y"),
            )
        })
        .collect()
}

/// The figure names the program that drew it, its version and its designation, in its metadata
/// (ADR-164) and in its title block, where a printed copy shows it, as a drawing's does
/// (ADR-214): a plot on paper can be traced to the build that drew it.
#[test]
fn the_figure_names_its_tool_and_version_where_a_reader_sees_them() {
    let svg = figure_svg("Probe", None);
    let stamp = format!(
        "FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001",
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(
        svg.matches(&format!("<metadata>{stamp}</metadata>"))
            .count(),
        1,
        "{svg}"
    );
    let document = roxmltree::Document::parse(&svg).unwrap();
    let cells = title_block(&document);
    let value = |key: &str| {
        cells
            .iter()
            .find(|cell| cell.0 == key)
            .map(|cell| cell.1.clone())
    };
    let program = format!("FusionSpace HPR {}", env!("CARGO_PKG_VERSION"));
    assert_eq!(value("PROGRAM"), Some(program.clone()));
    assert_eq!(
        value("DESIGNATION").as_deref(),
        Some("FS-ACHERNAR · SW · TOOL 001")
    );
    // Each once in the text a reader sees.
    let texts = texts(&document);
    for said in [program.as_str(), "FS-ACHERNAR · SW · TOOL 001"] {
        assert_eq!(texts.iter().filter(|text| text.contains(said)).count(), 1);
    }
}

/// The figure gives the bundled motor catalog's as-of date as the exports do (#403), in its title
/// block's cell of its own, beside the file of the figure's data, so a printed copy carries both;
/// a data file's name too long for its cell is cut, never the date. Before #403 the plot gave no
/// date.
#[test]
fn the_figure_dates_the_motor_catalog_in_its_title_block() {
    let long = format!("{}.plot.csv", "a-long-flight-name-".repeat(7));
    for data_file in ["probe.plot.csv", long.as_str()] {
        let points = [point(0.0, 0.0), point(10.0, 50.0)];
        let svg = svg(&Figure {
            title: "Probe",
            subtitle: "s",
            catalog_as_of: "2026-01-02",
            data_file,
            points: &points,
            events: &[],
            devices: &[],
            unpredicted: None,
            apogee: None,
            top_speed: None,
        });
        let document = roxmltree::Document::parse(&svg).unwrap();
        let cells = title_block(&document);
        let value = |key: &str| cells.iter().find(|cell| cell.0 == key).unwrap().1.clone();
        assert_eq!(value("MOTOR CATALOG"), "as of 2026-01-02", "{svg}");
        let data = value("DATA");
        if data_file.len() < 100 {
            assert_eq!(data, data_file);
        } else {
            let kept = data.strip_suffix('…').unwrap();
            assert!(long.starts_with(kept) && kept.len() > 100, "{data}");
        }
        assert_eq!(
            texts(&document)
                .iter()
                .filter(|text| text.contains("2026-01-02"))
                .count(),
            1
        );
    }
}

/// The figure ends in a title block (`product/principles.md` §1; #382): after the trust note, a
/// box with a 2 px ink border (`foundations.md`, *Lines*) holding the fields of
/// `product/web.md`'s, from ISO 7200, each a capital key over its value, in rows within the
/// sheet's text width, the figure's last thing. Before #382's fix the figure ended in one line.
#[test]
fn the_figure_ends_in_a_title_block() {
    let long = "A design with a name long enough to fill most of a title block's first row, \
                and then some";
    for title in ["Probe", long] {
        let svg = figure_svg(title, None);
        let document = roxmltree::Document::parse(&svg).unwrap();
        let cells = title_block(&document);
        let keys: Vec<&str> = cells.iter().map(|cell| cell.0.as_str()).collect();
        assert_eq!(
            keys,
            [
                "OWNER",
                "TITLE",
                "TYPE",
                "DESIGNATION",
                "PROGRAM",
                "UNITS",
                "DATA",
                "MOTOR CATALOG"
            ]
        );
        let value = |key: &str| cells.iter().find(|cell| cell.0 == key).unwrap().1.clone();
        assert_eq!(value("OWNER"), "FusionSpace");
        assert_eq!(value("TITLE"), cut(title, NAME_LIMIT));
        assert_eq!(value("TYPE"), "Simulated flight");
        assert_eq!(value("UNITS"), "m, m/s, m/s², s [ft, ft/s, g]");
        let block = document
            .descendants()
            .find(|node| node.attribute("id") == Some("title-block"))
            .unwrap();
        let border = block
            .children()
            .find(|node| node.has_tag_name("rect"))
            .unwrap();
        let number = |node: roxmltree::Node<'_, '_>, name: &str| -> f64 {
            node.attribute(name).unwrap().parse().unwrap()
        };
        assert_eq!(border.attribute("stroke"), Some(INK));
        assert_eq!(border.attribute("stroke-width"), Some("2"));
        let (left, top) = (number(border, "x"), number(border, "y"));
        let (right, bottom) = (
            left + number(border, "width"),
            top + number(border, "height"),
        );
        assert_eq!((left, right), (LEFT, WIDTH - MARGIN));
        // Each cell's text inside the box, at a cell's padding from its left edge, each key 16 px
        // under its row's top, and no two cells of a row overlapping at 0.6 em a character.
        for (i, (key, value, x, y)) in cells.iter().enumerate() {
            assert!((y - 16.0 - top) % CELL_HEIGHT == 0.0, "{key}: {y}");
            assert!(y + 16.0 + 12.0 <= bottom, "{key}");
            let wide = 0.6 * 12.0 * key.chars().count().max(value.chars().count()) as f64;
            let end = cells
                .get(i + 1)
                .filter(|next| next.3 == *y)
                .map_or(right, |next| next.2 - CELL_PAD);
            assert!(x + wide <= end, "{key}: {value} at {x} runs past {end}");
        }
        // Whole rows, then the sheet's margin.
        let height: f64 = document
            .root_element()
            .attribute("height")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(((bottom - top) / CELL_HEIGHT).fract(), 0.0, "{svg}");
        assert_eq!(height, bottom + MARGIN);
        // Its last: nothing drawn after it.
        assert!(svg.ends_with("</g>\n</svg>\n"));
        assert_eq!(block.next_sibling_element(), None);
    }
}

/// An unpredicted span is hatched in every panel, labelled above the first, outside the
/// hatching, and said under the table.
#[test]
fn an_unpredicted_fall_is_hatched_and_said() {
    let svg = figure_svg("Probe", Some((5.0, 20.0)));
    assert_eq!(svg.matches("fill=\"url(#hatch)\"").count(), 3, "{svg}");
    assert_eq!(svg.matches(">not a prediction<").count(), 1, "{svg}");
    assert!(
        svg.contains("Hatched, 5.00\u{a0}s to 20.00\u{a0}s:"),
        "{svg}"
    );
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
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &points,
        events: &events,
        devices: &devices,
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    let document = roxmltree::Document::parse(&svg).unwrap();
    // The name column's lines: the first at the column, the later ones indented.
    let name_x = LEFT + COLUMNS[4];
    let column: Vec<&str> = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("text-anchor").is_none())
        .filter(|node| {
            let x = node.attribute("x").unwrap().parse::<f64>().unwrap();
            x == name_x || x == name_x + INDENT_PX
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
        // The title, cut, ends over the panels.
        if size == TITLE_SIZE {
            assert!(x + width <= WIDTH - RIGHT, "{x} {:?}", node.text());
        }
    }
}

/// Each panel gives both units (ADR-164 §6, ADR-210): its SI title on the left, its US unit's
/// over a second scale on the right, whose ticks sit at round US values where those values fall on
/// the SI axis; the event table gives each height in feet beside meters.
#[test]
fn each_panel_gives_its_us_units_on_a_right_scale() {
    let mut points: Vec<Point> = (0..=20)
        .map(|i| point(f64::from(i), 5.0 * f64::from(i)))
        .collect();
    points[20].acceleration_m_s2 = 4.0 * STANDARD_GRAVITY_M_S2;
    let mut apogee = event(EventKind::Apogee, 20.0);
    apogee.sample.height_above_ground_m = 100.0;
    let svg = svg(&Figure {
        title: "t",
        subtitle: "s",
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &points,
        events: &[apogee],
        devices: &[],
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    let document = roxmltree::Document::parse(&svg).unwrap();
    let right = WIDTH - RIGHT;
    let panel = |id: &str| {
        document
            .descendants()
            .find(|node| node.attribute("id") == Some(id))
            .unwrap()
    };
    let number = |node: roxmltree::Node<'_, '_>, name: &str| -> f64 {
        node.attribute(name).unwrap().parse().unwrap()
    };
    // The right scale's texts of a panel: its title, then its tick labels, with their y.
    let right_texts = |id: &str| -> Vec<(String, f64)> {
        panel(id)
            .children()
            .filter(|node| node.has_tag_name("text"))
            .filter(|node| number(*node, "x") == right + 2.0 * TICK)
            .map(|node| (node.text().unwrap().to_owned(), number(node, "y")))
            .collect()
    };
    for (id, title, unit) in [
        ("altitude", "ALTITUDE · m AGL", "ft AGL"),
        ("speed", "SPEED · m/s", "ft/s"),
        ("acceleration", "ACCELERATION · m/s²", "g"),
    ] {
        let texts = right_texts(id);
        assert_eq!(texts[0].0, unit, "{id}: {texts:?}");
        assert!(
            panel(id)
                .children()
                .any(|node| node.text() == Some(title) && number(node, "x") == LEFT),
            "{id}"
        );
        // 3 to 6 ticks, each with its mark.
        assert!((3..=6).contains(&(texts.len() - 1)), "{id}: {texts:?}");
    }
    // The altitude axis runs 0 to 100 m: 0 to 328 ft, ticks every 100 ft, each at its height
    // in meters on the left scale.
    let altitude = right_texts("altitude");
    let labels: Vec<&str> = altitude[1..]
        .iter()
        .map(|(text, _)| text.as_str())
        .collect();
    assert_eq!(labels, ["0", "100", "200", "300"]);
    let top = altitude[0].1 + 12.0;
    // The panel's height, as drawn: its plot area's.
    let height = panel("altitude")
        .children()
        .find(|node| node.has_tag_name("rect") && node.attribute("fill") == Some(SURFACE))
        .map(|node| number(node, "height"))
        .unwrap();
    let y_of_m = |m: f64| top + height - m / 100.0 * height;
    for (text, y) in &altitude[1..] {
        let feet: f64 = text.parse().unwrap();
        assert!(
            (y - 4.0 - y_of_m(feet * FOOT_M)).abs() < 0.051,
            "{text}: {y}"
        );
    }
    // The acceleration reaches 4 g: whole g's on the right.
    let g: Vec<String> = right_texts("acceleration")[1..]
        .iter()
        .map(|(text, _)| text.clone())
        .collect();
    assert!(g.contains(&"4".to_owned()), "{g:?}");
    // The apogee's row gives 100.0 m and 328 ft.
    assert_eq!(
        table_rows(&document),
        [row(["1", "20.00", "100.0", "328", "apogee"])]
    );
}

/// A second scale's ticks: 3 to 6, all inside its span, at round steps, over spans of every
/// size and offset, positive and negative; none on an empty or infinite span.
#[test]
fn second_scale_ticks_are_round_and_inside() {
    assert_eq!(
        inner_ticks(0.0, 328.08),
        (vec![0.0, 100.0, 200.0, 300.0], 100.0)
    );
    assert_eq!(inner_ticks(-1.0, 1.0), (vec![-1.0, 0.0, 1.0], 1.0));
    assert_eq!(inner_ticks(-0.9, 0.9), (vec![-0.5, 0.0, 0.5], 0.5));
    assert_eq!(inner_ticks(0.0, 9.80665), (vec![0.0, 2.5, 5.0, 7.5], 2.5));
    assert_eq!(inner_ticks(1.0, 1.0).0, Vec::<f64>::new());
    assert_eq!(inner_ticks(0.0, f64::INFINITY).0, Vec::<f64>::new());
    assert_eq!(inner_ticks(f64::NAN, 1.0).0, Vec::<f64>::new());
    for span in (1..500).map(|i| f64::from(i) * 0.731) {
        for offset in [-1234.5, -7.3, -0.5, 0.0, 0.5, 3.1, 123.4, 98765.4] {
            let (ticks, step) = inner_ticks(offset, offset + span);
            assert!((3..=6).contains(&ticks.len()), "{offset} {span}: {ticks:?}");
            for v in &ticks {
                assert!(*v >= offset - 1e-9 * span && *v <= offset + span + 1e-9 * span);
            }
            let mantissa = step / 10f64.powf(step.log10().floor());
            assert!(
                [1.0, 2.0, 2.5, 5.0]
                    .iter()
                    .any(|m| (mantissa - m).abs() < 1e-9),
                "{step}"
            );
        }
    }
    assert_eq!(step_decimals(250.0), 0);
    assert_eq!(step_decimals(2.5), 1);
    assert_eq!(step_decimals(0.25), 2);
    assert_eq!(step_decimals(0.1), 1);
    // A step far below one still prints its digits, so its ticks don't all read `0`.
    assert_eq!(step_decimals(1e-7), 7);
    assert_eq!(step_decimals(2.5e-7), 8);
    assert_eq!(step_decimals(5e-4), 4);
    assert_eq!(step_decimals(0.0), 0);
    assert_eq!(step_decimals(f64::NAN), 0);
    let (ticks, step) = inner_ticks(0.0, 3e-7);
    let labels: Vec<String> = ticks
        .iter()
        .map(|v| number(*v, step_decimals(step)))
        .collect();
    assert_eq!(labels, ["0.0000000", "0.0000001", "0.0000002", "0.0000003"]);
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
    let said = "Apogee 100.0\u{a0}m (328\u{a0}ft) above the launch site at 5.00\u{a0}s; top speed \
                1,234.6\u{a0}m/s (4,050\u{a0}ft/s) at 4.00\u{a0}s.";
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
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
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
        "No apogee; top speed 70.0\u{a0}m/s (230\u{a0}ft/s) at 20.00\u{a0}s, in the fall: not a \
         prediction."
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
        "ft AGL",
        "SPEED · m/s",
        "ft/s",
        "ACCELERATION · m/s²",
        "g",
        "TIME · s",
    ] {
        assert!(texts.contains(&title.to_owned()), "{title}: {texts:?}");
    }
    // Lines outside the panels: the markers' and the table's rules, none of them a swatch.
    let outside = lines
        .iter()
        .filter(|line| {
            !line.ancestors().any(|node| {
                node.attribute("id").is_some_and(|id| {
                    id.starts_with("event-") || id == "events" || id == "title-block"
                }) || PANELS
                    .iter()
                    .any(|panel| node.attribute("id") == Some(panel.id))
                    || node.has_tag_name("pattern")
            })
        })
        .count();
    assert_eq!(outside, 0, "{svg}");
    // Nor a swatch of any other shape: the only rectangles are the canvas, the plot areas, the
    // clips and the title block's border, and the only circles the balloons.
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
    let block = rects
        .iter()
        .filter(|rect| {
            rect.parent()
                .is_some_and(|node| node.attribute("id") == Some("title-block"))
        })
        .count();
    assert_eq!((in_clip, canvas, areas, block), (3, 1, 4, 1));
    assert_eq!(rects.len(), 8, "{svg}");
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

/// The product system's spacing steps, px (`product/foundations.md`, *Space and layout*: a 4 px
/// base on an 8 px rhythm).
const SPACE_SCALE: [f64; 10] = [2.0, 4.0, 8.0, 12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0];

/// The figure's type sits on the product system's scale (#382): every line is the 12 px `label`
/// token but the title, the 20 px, weight 600 `subtitle` token, so no text, a balloon's numeral
/// among them, is under the 12 px floor; a two-digit numeral fits inside its balloon; the steps
/// between the title, subtitle, caption, marker rows and lines are on the 4 px scale, as drawn;
/// and the time axis's title sits at the axis's end.
#[test]
fn the_figure_sets_its_type_on_the_scale() {
    let svg = figure_svg("Probe", Some((6.0, 20.0)));
    let document = roxmltree::Document::parse(&svg).unwrap();
    let root = document.root_element();
    assert_eq!(root.attribute("font-size"), Some("12"));
    let sized: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("font-size").is_some())
        .collect();
    // Only the title sets a size of its own.
    assert_eq!(sized.len(), 1, "{svg}");
    assert_eq!(sized[0].attribute("font-size"), Some("20"));
    assert_eq!(sized[0].attribute("font-weight"), Some("600"));
    assert_eq!(sized[0].text(), Some("Probe"));
    assert_eq!(TITLE_SIZE, 20.0);
    // Each balloon's numeral, at the inherited 12 px, inside its circle: two digits at 0.6 em
    // a character are narrower than the circle's chord at the figures' top and bottom, 0.7 em
    // tall, centered.
    for group in document.descendants().filter(|node| {
        node.attribute("id")
            .is_some_and(|id| id.starts_with("event-"))
    }) {
        let numeral = group
            .children()
            .find(|node| node.has_tag_name("text"))
            .unwrap();
        assert_eq!(numeral.attribute("font-size"), None);
        let circle = group
            .children()
            .find(|node| node.has_tag_name("circle"))
            .unwrap();
        let r: f64 = circle.attribute("r").unwrap().parse().unwrap();
        let half_height = 0.7 * 12.0 / 2.0;
        let chord = 2.0 * (r * r - half_height * half_height).sqrt();
        assert!(2.0 * 0.6 * 12.0 <= chord, "{r}");
    }
    // The steps, each on the scale.
    for step in [
        TITLE_TOP - MARGIN,
        SUBTITLE_TOP - TITLE_TOP,
        CAPTION_TOP - SUBTITLE_TOP,
        CAPTION_GAP,
        MARKER_ROW,
        LIST_LINE,
        ROW_RULE,
        INDENT_PX,
    ] {
        assert!(SPACE_SCALE.contains(&step), "{step}");
    }
    let y = |node: roxmltree::Node<'_, '_>, name: &str| -> f64 {
        node.attribute(name).unwrap().parse().unwrap()
    };
    let on_scale = |steps: &[f64]| {
        for step in steps {
            assert!(SPACE_SCALE.contains(step), "{step} of {steps:?}");
        }
    };
    // As drawn: the balloon rows, from the caption's last line.
    let mut rows: Vec<f64> = document
        .descendants()
        .filter(|node| node.has_tag_name("circle"))
        .map(|node| y(node, "cy"))
        .collect();
    rows.sort_by(f64::total_cmp);
    rows.dedup();
    // One row: balloons that would collide step right, not down.
    assert_eq!(rows.len(), 1, "{rows:?}");
    let caption_last = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("x") == Some("76"))
        .map(|node| y(node, "y"))
        .filter(|baseline| *baseline < rows[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let mut steps = vec![rows[0] - caption_last];
    steps.extend(rows.windows(2).map(|pair| pair[1] - pair[0]));
    on_scale(&steps);
    // The event table: its title under the time axis's, its head, and its rows' rules.
    let text_y = |label: &str| {
        document
            .descendants()
            .find(|node| node.has_tag_name("text") && node.text() == Some(label))
            .map(|node| y(node, "y"))
            .unwrap()
    };
    let table = text_y("EVENTS · SIMULATED BY FUSIONSPACE HPR");
    let head = text_y("NO.");
    let events = document
        .descendants()
        .find(|node| node.attribute("id") == Some("events"))
        .unwrap();
    let rules: Vec<f64> = events
        .children()
        .filter(|node| node.has_tag_name("line"))
        .map(|node| y(node, "y1"))
        .collect();
    assert!(rules.len() >= 3, "{rules:?}");
    let mut steps = vec![table - text_y("TIME · s"), head - table, rules[0] - head];
    steps.extend(rules.windows(2).map(|pair| pair[1] - pair[0]));
    on_scale(&steps);
    // As drawn: the title, the subtitle and the caption's lines.
    let ys: Vec<f64> = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && node.attribute("x") == Some("76"))
        .map(|node| node.attribute("y").unwrap().parse().unwrap())
        .take(4)
        .collect();
    assert_eq!(ys, [32.0, 56.0, 80.0, 96.0], "{svg}");
    // The time axis's title, right-aligned at the axis's end.
    let time = document
        .descendants()
        .find(|node| node.has_tag_name("text") && node.text() == Some("TIME · s"))
        .unwrap();
    assert_eq!(time.attribute("text-anchor"), Some("end"));
    assert_eq!(
        time.attribute("x").unwrap().parse::<f64>().unwrap(),
        WIDTH - RIGHT
    );
    // The title block: from the note's last line to its top, a key, its value, and its bottom.
    let block = document
        .descendants()
        .find(|node| node.attribute("id") == Some("title-block"))
        .unwrap();
    let border = block
        .children()
        .find(|node| node.has_tag_name("rect"))
        .unwrap();
    let note_last = document
        .descendants()
        .filter(|node| node.has_tag_name("text") && !node.ancestors().any(|a| a == block))
        .map(|node| y(node, "y"))
        .fold(f64::NEG_INFINITY, f64::max);
    let cells: Vec<f64> = block
        .children()
        .filter(|node| node.has_tag_name("text"))
        .map(|node| y(node, "y"))
        .take(2)
        .collect();
    let top = y(border, "y");
    on_scale(&[
        top - note_last,
        cells[0] - top,
        cells[1] - cells[0],
        CELL_HEIGHT - (cells[1] - top),
        CELL_PAD,
        LEADER_GAP,
    ]);
}

/// No line of the figure parts a number from its unit (#382; `product/data.md`, *Numbers*):
/// within a line they are joined by a no-break space, never a plain one, at which the caption
/// and notes wrap; and no line ends on a number whose unit opens the next.
#[test]
fn a_number_keeps_its_unit_on_its_line() {
    const UNITS: [&str; 7] = ["m", "ft", "s", "m/s", "ft/s", "m/s²", "ft/s²"];
    let is_unit =
        |word: &str| UNITS.contains(&word.trim_end_matches(|c: char| ",;:.)".contains(c)));
    let ends_in_a_number = |word: &str| word.ends_with(|c: char| c.is_ascii_digit());
    let svg = figure_svg("Probe", Some((6.0, 20.0)));
    let document = roxmltree::Document::parse(&svg).unwrap();
    // The text drawn and the balloons' tooltips.
    let lines: Vec<String> = document
        .descendants()
        .filter(|node| node.has_tag_name("text") || node.has_tag_name("title"))
        .map(|node| node.text().unwrap_or_default().to_owned())
        .collect();
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with("0.00\u{a0}s: liftoff")),
        "{lines:?}"
    );
    // The caption and the hatching's note, which give numbers with units.
    assert!(
        lines
            .iter()
            .any(|line| line.contains("100.0\u{a0}m (328\u{a0}ft)")),
        "{lines:?}"
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("6.00\u{a0}s to 20.00\u{a0}s")),
        "{lines:?}"
    );
    for line in &lines {
        let words: Vec<&str> = line.split(' ').collect();
        for pair in words.windows(2) {
            assert!(!(ends_in_a_number(pair[0]) && is_unit(pair[1])), "{line:?}");
        }
    }
    // The caption's and the notes' lines, flush left or indented, as they wrap.
    let wrapped: Vec<String> = document
        .descendants()
        .filter(|node| {
            node.has_tag_name("text") && matches!(node.attribute("x"), Some("76") | Some("100"))
        })
        .map(|node| node.text().unwrap_or_default().to_owned())
        .collect();
    assert!(wrapped.len() > 5, "{wrapped:?}");
    for pair in wrapped.windows(2) {
        let last = pair[0].split(' ').next_back().unwrap_or_default();
        let first = pair[1].split(' ').next().unwrap_or_default();
        assert!(!(ends_in_a_number(last) && is_unit(first)), "{pair:?}");
    }
    // At every width the caption could wrap to, from the longest word and the indent, below
    // which `wrap` cuts inside a word, the summary keeps each number with its unit.
    let summary = summary(&Figure {
        title: "t",
        subtitle: "s",
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &[],
        events: &[],
        devices: &[],
        unpredicted: None,
        apogee: Some(Reading {
            value: 1234.5,
            time_s: 17.25,
            unpredicted: false,
        }),
        top_speed: Some(Reading {
            value: 210.0,
            time_s: 2.5,
            unpredicted: true,
        }),
    });
    let longest = summary
        .split(' ')
        .map(|word| word.chars().count())
        .max()
        .unwrap();
    for width in longest + INDENT.len() + 1..=LIST_WIDTH {
        for pair in wrap(&summary, width).windows(2) {
            let last = pair[0].split(' ').next_back().unwrap_or_default();
            let first = pair[1].trim_start().split(' ').next().unwrap_or_default();
            assert!(
                !(ends_in_a_number(last) && is_unit(first)),
                "{width} {pair:?}"
            );
        }
    }
}

/// A balloon as drawn: its center's x and y, its event line's x, and its leader's two ends if it
/// has one.
type Balloon = (f64, f64, f64, Option<((f64, f64), (f64, f64))>);

/// A panel as drawn: its height, and its lines' runs of points.
type DrawnPanel = (f64, Vec<Vec<(f64, f64)>>);

/// Each balloon's center and row y, its event line's x, and its leader's two ends if it has one,
/// in its number's order.
fn balloons(document: &roxmltree::Document<'_>) -> Vec<Balloon> {
    let number = |node: roxmltree::Node<'_, '_>, name: &str| -> f64 {
        node.attribute(name).unwrap().parse().unwrap()
    };
    document
        .descendants()
        .filter(|node| {
            node.attribute("id")
                .is_some_and(|id| id.starts_with("event-"))
        })
        .map(|group| {
            let circle = group
                .children()
                .find(|node| node.has_tag_name("circle"))
                .unwrap();
            let lines: Vec<_> = group
                .children()
                .filter(|node| node.has_tag_name("line"))
                .collect();
            let dotted = lines
                .iter()
                .find(|line| line.attribute("stroke-dasharray") == Some(DOTTED))
                .unwrap();
            let leaders: Vec<_> = lines
                .iter()
                .filter(|line| line.attribute("stroke-dasharray").is_none())
                .collect();
            assert!(leaders.len() <= 1);
            let leader = leaders.first().map(|line| {
                (
                    (number(**line, "x1"), number(**line, "y1")),
                    (number(**line, "x2"), number(**line, "y2")),
                )
            });
            assert_eq!(number(*dotted, "x1"), number(*dotted, "x2"));
            // A leader meets the event's line where the dotted line starts.
            if let Some((_, end)) = leader {
                assert_eq!(end, (number(*dotted, "x1"), number(*dotted, "y1")));
            }
            let n: usize = group.attribute("id").unwrap()["event-".len()..]
                .parse()
                .unwrap();
            (
                n,
                (
                    number(circle, "cx"),
                    number(circle, "cy"),
                    number(*dotted, "x1"),
                    leader,
                ),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>()
        .into_values()
        .collect()
}

/// The figure's balloons, as a drawing sets them, for `events` over a flight from 0 to 100 s:
/// each stepped balloon's leader runs from its circle's edge to its line, below the row, and
/// passes no other balloon; no two balloons in a row are nearer than [`MARKER_ROW`], in time
/// order; every balloon is whole on the figure.
fn balloons_hold(events: &[FlightEvent]) -> Vec<Balloon> {
    let points: Vec<Point> = (0..=100).map(|i| point(f64::from(i), 1.0)).collect();
    let svg = svg(&Figure {
        title: "t",
        subtitle: "s",
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &points,
        events,
        devices: &[],
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    let document = roxmltree::Document::parse(&svg).unwrap();
    let balloons = balloons(&document);
    let (radius, step) = balloon_size(balloons.len());
    for (i, (cx, cy, x, leader)) in balloons.iter().enumerate() {
        assert!(
            cx - radius >= MARGIN && cx + radius <= WIDTH - MARGIN,
            "{i}: {cx}"
        );
        match leader {
            None => assert!(
                (cx - x).abs() < 0.05,
                "{i}: {cx} off its line {x} with no leader"
            ),
            Some((start, end)) => {
                assert!(
                    ((start.0 - cx).hypot(start.1 - cy) - radius).abs() < 0.1,
                    "{i}: {start:?}"
                );
                assert!(end.1 > cy + radius, "{i}: {end:?}");
                // No other balloon of its row on its way: each center at least a radius from the
                // segment. (A second row, past 38 instants, may sit on the first's leaders.)
                for (j, (ox, oy, _, _)) in balloons.iter().enumerate() {
                    if j == i || oy != cy {
                        continue;
                    }
                    let (dx, dy) = (end.0 - start.0, end.1 - start.1);
                    let t = (((ox - start.0) * dx + (oy - start.1) * dy) / (dx * dx + dy * dy))
                        .clamp(0.0, 1.0);
                    let d = (start.0 + t * dx - ox).hypot(start.1 + t * dy - oy);
                    assert!(d >= radius, "{i}'s leader crosses {j}: {d}");
                }
            }
        }
    }
    // In each row, in time order, a step apart at least.
    let mut rows: Vec<f64> = balloons.iter().map(|b| b.1).collect();
    rows.sort_by(f64::total_cmp);
    rows.dedup();
    for row in rows {
        let xs: Vec<f64> = balloons
            .iter()
            .filter(|b| b.1 == row)
            .map(|b| b.0)
            .collect();
        for pair in xs.windows(2) {
            assert!(pair[1] - pair[0] >= step - 0.05, "{pair:?}");
        }
    }
    balloons
}

/// Balloons that would collide step right along a short leader, in one row (`product/data.md`,
/// *Charts*; #382), at the axis's start and its end, where they step back left from the
/// figure's margin to stay whole. Before #382's fix they dropped to lower rows.
#[test]
fn colliding_balloons_step_right_on_a_leader() {
    // Liftoff, rail exit 0.4 s on: 3.2 px apart, the second stepped right.
    let start = balloons_hold(&[
        event(EventKind::Liftoff, 0.0),
        event(EventKind::RailExit, 0.4),
        event(EventKind::Apogee, 50.0),
    ]);
    assert_eq!(start.len(), 3);
    assert!(start.iter().all(|b| b.1 == start[0].1), "{start:?}");
    assert!(start[0].3.is_none() && start[2].3.is_none(), "{start:?}");
    assert_eq!(start[1].0 - start[0].0, MARKER_ROW);
    assert!(start[1].3.is_some(), "{start:?}");
    // Three in the axis's last 4 px: stepped back from the margin, the last on a leader too.
    let end = balloons_hold(&[
        event(EventKind::Apogee, 99.5),
        event(EventKind::Trigger(0), 99.75),
        event(EventKind::GroundHit, 100.0),
    ]);
    assert!(end.iter().all(|b| b.1 == end[0].1), "{end:?}");
    assert_eq!(end[2].0, WIDTH - MARGIN - MARKER_RADIUS, "{end:?}");
    assert!(end.iter().filter(|b| b.3.is_some()).count() >= 2, "{end:?}");
}

/// More balloons than a row holds, 50 at instants 0.8 px apart, take a second row in turn, each set
/// as one, every balloon whole on the figure.
#[test]
fn a_crowd_of_balloons_stays_on_the_figure() {
    let events: Vec<FlightEvent> = (0..50)
        .map(|i| event(EventKind::Apogee, 40.0 + 0.1 * f64::from(i)))
        .collect();
    let balloons = balloons_hold(&events);
    assert_eq!(balloons.len(), 50);
    let mut rows: Vec<f64> = balloons.iter().map(|b| b.1).collect();
    rows.sort_by(f64::total_cmp);
    rows.dedup();
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_eq!(rows[1] - rows[0], MARKER_ROW);
    // In turn: the first balloon in the first row, the second in the second.
    assert_eq!(
        (balloons[0].1, balloons[1].1, balloons[2].1),
        (rows[0], rows[1], rows[0])
    );
}

/// The length-weighted average of a polyline's segments' angles from level, rad: Cleveland's
/// average absolute orientation, measured on the drawn path's pixels.
fn orientation(paths: &[Vec<(f64, f64)>]) -> f64 {
    let (mut sum, mut length) = (0.0, 0.0);
    for path in paths {
        for pair in path.windows(2) {
            let (dx, dy) = ((pair[1].0 - pair[0].0).abs(), (pair[1].1 - pair[0].1).abs());
            let l = dx.hypot(dy);
            sum += dy.atan2(dx) * l;
            length += l;
        }
    }
    sum / length
}

/// Each panel's height and its lines' points as drawn, px.
fn drawn_panels(svg: &str) -> Vec<DrawnPanel> {
    let document = roxmltree::Document::parse(svg).unwrap();
    PANELS
        .iter()
        .map(|panel| {
            let group = document
                .descendants()
                .find(|node| node.attribute("id") == Some(panel.id))
                .unwrap();
            let height: f64 = group
                .children()
                .find(|node| node.has_tag_name("rect") && node.attribute("fill") == Some(SURFACE))
                .and_then(|node| node.attribute("height"))
                .unwrap()
                .parse()
                .unwrap();
            let mut paths = Vec::new();
            for path in group.children().filter(|node| node.has_tag_name("path")) {
                let d = path.attribute("d").unwrap();
                for run in d.split('M').filter(|run| !run.is_empty()) {
                    paths.push(
                        run.split('L')
                            .map(|xy| {
                                let (x, y) = xy.split_once(' ').unwrap();
                                (x.parse().unwrap(), y.parse().unwrap())
                            })
                            .collect(),
                    );
                }
            }
            (height, paths)
        })
        .collect()
}

/// Each panel's height is banked to 45° (`product/data.md`, *Charts*, after Cleveland; #382):
/// within [`PANEL_HEIGHTS`], its lines, as drawn, run at 45° on average, weighted by length,
/// to the 4 px grid's rounding; at the least height they run steeper, at the greatest flatter.
/// A sawtooth climb and fall bank inside the bounds, away from the 180 px every panel had
/// before #382's fix, at which its lines ran at 48°.
#[test]
fn panels_bank_toward_45_degrees() {
    // Legs of 4 s, 161.6 px wide, up and down 100 m.
    let saw: Vec<Point> = (0..=20)
        .map(|i| {
            let t = f64::from(i);
            let phase = t % 8.0;
            let h = 25.0 * if phase <= 4.0 { phase } else { 8.0 - phase };
            Point {
                time_s: t,
                height_m: h,
                speed_m_s: h / 10.0,
                vertical_speed_m_s: -h / 20.0,
                acceleration_m_s2: h / 10.0,
                vertical_acceleration_m_s2: -h / 10.0,
            }
        })
        .collect();
    let saw_svg = svg(&Figure {
        title: "t",
        subtitle: "s",
        catalog_as_of: "2026-01-02",
        data_file: "probe.csv",
        points: &saw,
        events: &[],
        devices: &[],
        unpredicted: None,
        apogee: None,
        top_speed: None,
    });
    let (least, most) = PANEL_HEIGHTS;
    let tolerance = 1.0_f64.to_radians();
    let target = 45.0_f64.to_radians();
    for svg in [saw_svg.clone(), figure_svg("Probe", None)] {
        for (height, paths) in drawn_panels(&svg) {
            let angle = orientation(&paths);
            assert_eq!(height % 4.0, 0.0);
            let level = paths
                .iter()
                .all(|run| run.windows(2).all(|pair| pair[0].1 == pair[1].1));
            if level {
                assert_eq!(height, least);
            } else if height == least {
                assert!(
                    angle >= target - tolerance,
                    "{height}: {}",
                    angle.to_degrees()
                );
            } else if height == most {
                assert!(
                    angle <= target + tolerance,
                    "{height}: {}",
                    angle.to_degrees()
                );
            } else {
                assert!(
                    least < height && height < most && (angle - target).abs() <= tolerance,
                    "{height}: {}",
                    angle.to_degrees()
                );
            }
        }
    }
    // Every altitude leg rises the whole axis, 0 to 100 m, over 161.6 px, so the lines run at 45°
    // at 161.6 px, 160 on the 4 px grid, whatever the weighting.
    assert_eq!(drawn_panels(&saw_svg)[0].0, 160.0);
}

/// A panel whose lines are all level has no slope to bank and takes the least height.
#[test]
fn a_level_panel_takes_the_least_height() {
    let svg = figure_svg("Probe", None);
    // The small figure's acceleration is a constant 9 m/s², up and down.
    let panels = drawn_panels(&svg);
    assert_eq!(panels[2].0, PANEL_HEIGHTS.0);
}

/// The figure's data as CSV (`product/data.md`, *Charts* and *Files and exports*; #382): a header
/// row with units in brackets, then each point's values, in SI, CRLF lines; a value that is not
/// finite is refused by its column.
#[test]
fn the_figures_data_is_a_csv() {
    let text = csv(&[point(0.0, 10.0), point(1.5, 20.0)]).unwrap();
    assert_eq!(
        text,
        "time [s],altitude [m AGL],speed [m/s],vertical speed [m/s],acceleration [m/s^2],\
         vertical acceleration [m/s^2]\r\n\
         0.0,10.0,1.0,-0.5,9.0,-9.0\r\n\
         1.5,20.0,2.0,-1.0,9.0,-9.0\r\n"
    );
    assert!(text.is_ascii());
    let mut bad = point(2.0, 1.0);
    bad.speed_m_s = f64::NAN;
    let error = csv(&[point(0.0, 1.0), bad]).unwrap_err();
    assert!(error.contains("speed [m/s] at 2 s is NaN"), "{error}");
}

/// A hundred instants and more draw three-digit numerals: every balloon grows so its numeral
/// fits inside, 4 px of radius a digit past two, and steps as far, each still whole on the figure.
#[test]
fn three_digit_balloons_hold_their_numerals() {
    let events: Vec<FlightEvent> = (0..120)
        .map(|i| event(EventKind::Apogee, 0.5 * f64::from(i)))
        .collect();
    let balloons = balloons_hold(&events);
    assert_eq!(balloons.len(), 120);
    assert_eq!(balloon_size(120), (14.0, 32.0));
    assert_eq!(balloon_size(99), (MARKER_RADIUS, MARKER_ROW));
    // "120" at 0.6 em a digit inside the circle's chord at the figures' top and bottom.
    let (r, half_height) = (14.0_f64, 0.7 * 12.0 / 2.0);
    assert!(3.0 * 0.6 * 12.0 <= 2.0 * (r * r - half_height * half_height).sqrt());
}
