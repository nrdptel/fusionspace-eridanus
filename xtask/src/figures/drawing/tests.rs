use super::*;

/// The repository's root.
fn root() -> std::path::PathBuf {
    crate::designs::root().unwrap()
}

/// `body` in a 100 × 100 figure whose root sets a token fill and the system's data family, so a
/// probe fails only on what it adds.
fn figure(body: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" fill=\"#0B0F1C\" \
         font-family=\"'Cascadia Mono', monospace\">{body}</svg>"
    )
}

/// Asserts `svg` passes the drawing check.
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

/// Every committed figure draws in the tokens and the system's fonts, the hand-drawn one and
/// the badges among them; and the colors #385 found there fail, each by name.
#[test]
fn the_committed_figures_draw_in_the_tokens_and_the_old_colors_fail() {
    let images = root().join("docs/images");
    for name in [
        "flight-phases.svg",
        "census-badge.svg",
        "real-flights-badge.svg",
        "sim-plot.svg",
    ] {
        passes(&std::fs::read_to_string(images.join(name)).unwrap());
    }
    let phases = std::fs::read_to_string(images.join("flight-phases.svg")).unwrap();
    for old in [
        "#d9480f", "#1c5d99", "#2b8a3e", "#8a6d3b", "#222", "#555", "#2e7d32", "#c62828",
        "#b35c00", "#6e6e6e",
    ] {
        let probe = phases.replacen("stroke=\"#A22488\"", &format!("stroke=\"{old}\""), 1);
        assert_ne!(probe, phases);
        fails(&probe, Kind::Color, &format!("`stroke` `{old}`"));
    }
    let probe = phases.replacen("'Cascadia Mono', ", "Helvetica, ", 1);
    assert_ne!(probe, phases);
    let problems = check(&probe);
    assert!(
        !problems.is_empty() && problems.iter().all(|p| p.kind == Kind::Font),
        "{problems:#?}"
    );
}

/// A token passes in either case and in its three-digit form; anything else fails, a name,
/// `currentColor`, a `url()` with a fallback and a color in a `style` among them.
#[test]
fn only_tokens_pass_as_paint() {
    for paint in ["#A22488", "#a22488", "#fff", "#FFF", "none", "url(#hatch)"] {
        passes(&figure(&format!(
            "<rect width=\"10\" height=\"10\" fill=\"{paint}\" stroke=\"{paint}\"/>"
        )));
    }
    for paint in [
        "#000000",
        "#A22489",
        "white",
        "currentColor",
        "url(#hatch) #fff",
        "#ffff",
        "transparent",
    ] {
        fails(
            &figure(&format!(
                "<rect width=\"10\" height=\"10\" fill=\"{paint}\"/>"
            )),
            Kind::Color,
            &format!("`fill` `{paint}`"),
        );
    }
    fails(
        &figure("<rect width=\"10\" height=\"10\" style=\"fill: #555\"/>"),
        Kind::Color,
        "`fill` `#555`",
    );
    // A color-only property takes no `none` and no reference.
    fails(
        &figure("<g color=\"none\"><rect width=\"10\" height=\"10\"/></g>"),
        Kind::Color,
        "`color` `none`",
    );
    passes(&figure(
        "<g color=\"#566079\"><rect width=\"10\" height=\"10\"/></g>",
    ));
}

/// Every token the table holds passes as a fill, and a value one step off each fails.
#[test]
fn every_token_passes_and_its_neighbor_fails() {
    for (role, hex) in TOKENS {
        passes(&figure(&format!(
            "<rect width=\"10\" height=\"10\" fill=\"{hex}\"/>"
        )));
        let last = u8::from_str_radix(&hex[5..], 16).unwrap();
        let near = format!("{}{:02X}", &hex[..5], last ^ 1);
        assert!(!is_token(&near), "{role}: {near}");
    }
}

/// Partial opacity and gradients blend tokens into colors none holds, so they fail; full
/// opacity passes.
#[test]
fn blends_fail() {
    for property in ["opacity", "fill-opacity", "stroke-opacity"] {
        fails(
            &figure(&format!(
                "<rect width=\"10\" height=\"10\" {property}=\"0.5\"/>"
            )),
            Kind::Color,
            &format!("`{property}` `0.5`"),
        );
        passes(&figure(&format!(
            "<rect width=\"10\" height=\"10\" {property}=\"1\"/>"
        )));
    }
    passes(&figure(
        "<rect width=\"10\" height=\"10\" opacity=\"100%\"/>",
    ));
    fails(
        &figure("<rect width=\"10\" height=\"10\" style=\"opacity:.9\"/>"),
        Kind::Color,
        "`opacity` `.9`",
    );
    fails(
        &figure(
            "<defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"#0B0F1C\"/><stop \
             offset=\"1\" stop-color=\"#FFFFFF\"/></linearGradient></defs>",
        ),
        Kind::Color,
        "a gradient",
    );
}

/// A shape or a line of text with no fill anywhere above it draws in black and fails; a `line`,
/// which has nothing to fill, and a clip's own shape, which is never painted, pass.
#[test]
fn the_default_black_fails() {
    let bare = |body: &str| {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" \
             font-family=\"Archivo, sans-serif\">{body}</svg>"
        )
    };
    for shape in [
        "<rect width=\"10\" height=\"10\"/>",
        "<circle cx=\"5\" cy=\"5\" r=\"5\"/>",
        "<path d=\"M0 0L10 10\" stroke=\"#0B0F1C\"/>",
        "<polyline points=\"0 0 10 10\" stroke=\"#0B0F1C\"/>",
        "<text x=\"5\" y=\"20\">a</text>",
    ] {
        fails(&bare(shape), Kind::Color, "default black");
    }
    passes(&bare(
        "<line x1=\"0\" y1=\"0\" x2=\"10\" y2=\"10\" stroke=\"#0B0F1C\"/>",
    ));
    passes(&bare(
        "<clipPath id=\"c\"><rect width=\"10\" height=\"10\"/></clipPath>",
    ));
    passes(&bare(
        "<g fill=\"none\"><rect width=\"10\" height=\"10\"/></g>",
    ));
    passes(&bare(
        "<rect width=\"10\" height=\"10\" style=\"fill:#FFFFFF\"/>",
    ));
}

/// Text names Cascadia Mono or Archivo first, quoted or not; any other first family, or none,
/// fails.
#[test]
fn text_is_set_in_the_systems_families() {
    for family in [
        "'Cascadia Mono', ui-monospace, Menlo, Consolas, monospace",
        "\"Cascadia Mono\"",
        "Archivo, Arial, sans-serif",
    ] {
        passes(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" fill=\"#0B0F1C\">\
             <text x=\"5\" y=\"20\" font-family=\"{}\">a</text></svg>",
            family.replace('"', "&quot;")
        ));
    }
    for family in [
        "Helvetica, Arial, sans-serif",
        "Verdana,DejaVu Sans,sans-serif",
        "monospace",
    ] {
        fails(
            &format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" \
                 fill=\"#0B0F1C\"><text x=\"5\" y=\"20\" font-family=\"{family}\">a</text></svg>"
            ),
            Kind::Font,
            family,
        );
    }
    fails(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" fill=\"#0B0F1C\">\
         <text x=\"5\" y=\"20\">a</text></svg>",
        Kind::Font,
        "no font-family",
    );
    // The nearest family wins: a group's Archivo over the root's Helvetica.
    passes(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" fill=\"#0B0F1C\" \
         font-family=\"Helvetica\"><g font-family=\"Archivo\"><text x=\"5\" y=\"20\">a</text>\
         </g></svg>",
    );
}

/// Each role in [`TOKENS`] has the value the product system's `light.tokens.json` gives it,
/// resolved through `primitives.tokens.json`, and the table holds every color role of that file
/// but the series (the plot's own). Runs where `cargo xtask refs` has checked the system out.
#[test]
fn the_tokens_are_the_systems() {
    let tokens = root()
        .join(crate::site::theme::DESIGN_REFS)
        .join("product/tokens");
    if !tokens.is_dir() {
        eprintln!("skipped: {} is not checked out", tokens.display());
        return;
    }
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(tokens.join(name)).unwrap()).unwrap()
    };
    let (light, primitives) = (read("light.tokens.json"), read("primitives.tokens.json"));
    let resolve = |value: &serde_json::Value| -> String {
        let path = value.as_str().unwrap();
        let path = path.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut node = &primitives;
        for part in path.split('.') {
            node = &node[part];
        }
        node["$value"]["hex"].as_str().unwrap().to_owned()
    };
    let colors = light["color"].as_object().unwrap();
    for (role, hex) in TOKENS {
        assert_eq!(resolve(&colors[role]["$value"]), hex, "{role}");
    }
    for role in colors.keys().filter(|role| *role != "series") {
        assert!(
            TOKENS.iter().any(|(name, _)| name == role),
            "{role} is missing"
        );
    }
}
