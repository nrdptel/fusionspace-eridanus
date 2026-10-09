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

/// What this check leaves unread, the frame check it always runs beside refuses: a `<style>`
/// sheet, a filter, a mask, a blend mode, an image, a foreign object and animation each fail
/// there as something it can't measure. A `style` property in capitals and a `tspan`'s family
/// fail here too.
#[test]
fn the_frame_check_refuses_what_this_one_leaves_unread() {
    let root = |body: &str| {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" fill=\"#0B0F1C\" \
             font-family=\"'Cascadia Mono', monospace\">{body}</svg>"
        )
    };
    for body in [
        "<style>rect{fill:#FF0000}</style><rect width=\"10\" height=\"10\"/>",
        "<filter id=\"f\"><feColorMatrix type=\"hueRotate\" values=\"90\"/></filter><rect \
         width=\"10\" height=\"10\" filter=\"url(#f)\"/>",
        "<mask id=\"m\"><rect width=\"10\" height=\"10\" fill=\"#566079\"/></mask><rect \
         width=\"10\" height=\"10\" mask=\"url(#m)\"/>",
        "<rect width=\"10\" height=\"10\" style=\"mix-blend-mode:multiply\"/>",
        "<image width=\"10\" height=\"10\" href=\"data:image/png;base64,AA==\"/>",
        "<foreignObject width=\"10\" height=\"10\"/>",
        "<rect width=\"10\" height=\"10\"><set attributeName=\"fill\" to=\"#FF0000\"/></rect>",
        "<rect width=\"10\" height=\"10\"><animate attributeName=\"fill\" to=\"#FF0000\"/></rect>",
        "<text x=\"5\" y=\"20\" style=\"font: 12px Helvetica\">a</text>",
        "<text x=\"5\" y=\"20\">a<tspan font-family=\"Helvetica\">b</tspan></text>",
        "<rect width=\"10\" height=\"10\" style=\"FILL:#FF0000\"/>",
    ] {
        let svg = root(body);
        assert!(
            super::super::check(&svg)
                .iter()
                .any(|p| p.kind == Kind::Unsupported),
            "{svg}"
        );
    }
    fails(
        &figure("<rect width=\"10\" height=\"10\" style=\"FILL:#FF0000\"/>"),
        Kind::Color,
        "`fill` `#FF0000`",
    );
    fails(
        &figure("<text x=\"5\" y=\"20\">a<tspan font-family=\"Helvetica\">b</tspan></text>"),
        Kind::Font,
        "Helvetica",
    );
    // A family's case doesn't matter to a browser, so it doesn't here.
    passes(&figure(
        "<text x=\"5\" y=\"20\" font-family=\"cascadia mono\">a</text>",
    ));
}

/// The badges as the census writes them now, in each state the committed ones don't show, pass
/// both checks: with no rows, the census badge is a caution (no gated metric passes) and the real
/// flights' badge unjudged.
#[test]
fn freshly_written_badges_pass_in_every_state() {
    let badges = hpr_validate::census::badges(&[]);
    assert_eq!(badges.len(), 2);
    for (name, svg) in &badges {
        let mut problems = super::super::check(svg);
        problems.extend(check(svg));
        assert!(problems.is_empty(), "{name}: {problems:#?}\n{svg}");
    }
    assert!(badges[0].1.contains("fill=\"#F5AF20\""), "{}", badges[0].1);
    assert!(badges[1].1.contains("none compared"), "{}", badges[1].1);
}

/// Every SVG the repository tracks sits under a folder `cargo xtask figures` reads, so none ships
/// unchecked.
#[test]
fn every_tracked_svg_is_checked() {
    let output = std::process::Command::new("git")
        .current_dir(root())
        .args(["ls-files", "-z", "--", "*.svg"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git ls-files failed");
    let tracked: Vec<String> = output
        .stdout
        .split(|&b| b == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect();
    assert!(tracked.len() >= 4, "{tracked:?}");
    for path in &tracked {
        assert!(
            super::super::FOLDERS
                .iter()
                .any(|folder| path.starts_with(&format!("{folder}/"))),
            "{path} is outside {:?}",
            super::super::FOLDERS
        );
    }
}
