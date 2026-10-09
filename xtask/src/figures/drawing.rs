//! The drawing check (M0.9d1, #385): a figure draws in the product system's colors and fonts and
//! no others. [`check`](super::check) holds a figure to its frame; this holds it to the
//! FusionSpace product system's foundations (Rev A, `product/foundations.md`; ADR-164), which
//! name every color a drawing may use and the two families it may set text in:
//!
//! - **Colors.** Every paint (`fill`, `stroke`, `stop-color`, `flood-color`, `lighting-color`,
//!   `color`, and the same in a `style`) is `none`, a reference to a pattern of the figure's own
//!   (`url(#id)`), or one of [`TOKENS`]: the light theme's roles and the signal fills with the
//!   text that goes on them. A figure shown through `<img>` can't follow the page's theme, so it
//!   is drawn on the light one, as the plot is. `currentColor`, a color by name and a fallback
//!   after a `url()` fail rather than being resolved.
//! - **No blends.** An opacity below 1 (`opacity`, `fill-opacity`, `stroke-opacity`,
//!   `stop-opacity`) and a gradient draw colors between tokens that no token holds, so both fail.
//! - **No default black.** A shape or a line of text with no `fill` on it or above it draws in
//!   black, `#000000`, which is not a token. A `line` has nothing to fill, and a `clipPath`'s
//!   content is never painted, so neither needs one.
//! - **Fonts.** Each line of text names Cascadia Mono or Archivo first in its `font-family`:
//!   Cascadia Mono for what is read as data or as a drawing, Archivo for prose (*Type*). The
//!   families after it are the fallbacks the frame check measures.
//!
//! Line types are not checked here: the dashed, chain and dotted patterns carry meanings a
//! figure's author must choose, which a pattern match can't tell apart.

use roxmltree::{Document, Node};

use super::{Kind, Problem};

/// The colors a figure may draw in, by role: the light theme's roles and the signal fills of the
/// product system's foundations (Rev A, *Semantic roles*), as `light.tokens.json` resolves them.
/// Roles that share a value (`rule-strong` and `ink-muted`, `action` and `focus`) are listed once
/// each, so a test can compare every one with the system's file.
pub(crate) const TOKENS: [(&str, &str); 22] = [
    ("canvas", "#F3F4F7"),
    ("surface", "#FFFFFF"),
    ("rule", "#D6DAE4"),
    ("rule-strong", "#566079"),
    ("ink", "#0B0F1C"),
    ("ink-muted", "#566079"),
    ("ink-faint", "#98A1B8"),
    ("action", "#3350D6"),
    ("on-action", "#FFFFFF"),
    ("focus", "#3350D6"),
    ("danger", "#AC001E"),
    ("caution", "#B34F0C"),
    ("ok", "#0A6355"),
    ("predicted", "#A22488"),
    ("danger-fill", "#AC001E"),
    ("on-danger-fill", "#FFFFFF"),
    ("caution-fill", "#F5AF20"),
    ("on-caution-fill", "#0B0F1C"),
    ("ok-fill", "#0A6355"),
    ("on-ok-fill", "#FFFFFF"),
    ("info-fill", "#3350D6"),
    ("on-info-fill", "#FFFFFF"),
];

/// The families a line of text may name first.
const FAMILIES: [&str; 2] = ["Cascadia Mono", "Archivo"];

/// Paint properties whose value may be `none` or a `url()`.
const PAINTS: [&str; 2] = ["fill", "stroke"];
/// Properties whose value is a color only.
const COLORS: [&str; 4] = ["stop-color", "flood-color", "lighting-color", "color"];
/// Properties that blend a paint with what is under it.
const OPACITIES: [&str; 4] = ["opacity", "fill-opacity", "stroke-opacity", "stop-opacity"];

/// The shapes a fill paints.
const FILLED: [&str; 6] = ["rect", "circle", "ellipse", "polyline", "polygon", "path"];

/// What the SVG `text` draws outside the product system's colors and fonts: nothing when every
/// paint is a token, nothing blends, nothing falls back to black and every line of text names one
/// of the system's families first. A document that doesn't parse is left to
/// [`check`](super::check), which names it.
pub(crate) fn check(text: &str) -> Vec<Problem> {
    let Ok(doc) = Document::parse(text) else {
        return Vec::new();
    };
    let mut walk = Walk {
        doc: &doc,
        problems: Vec::new(),
    };
    walk.element(doc.root_element(), &Inherited::default());
    walk.problems
}

/// What an element inherits from those around it.
#[derive(Debug, Clone, Default)]
struct Inherited {
    /// Whether a `fill` is set on it or above it.
    filled: bool,
    /// The `font-family` set nearest above it.
    family: Option<String>,
    /// Inside a `clipPath`, whose content is never painted.
    clip: bool,
}

/// The walk over one document.
struct Walk<'a, 'input> {
    doc: &'a Document<'input>,
    problems: Vec<Problem>,
}

impl Walk<'_, '_> {
    /// `node` as a reader finds it: its line and its tag, with its id.
    fn label(&self, node: Node<'_, '_>) -> String {
        let line = self.doc.text_pos_at(node.range().start).row;
        match node.attribute("id") {
            Some(id) => format!("line {line} <{} id=\"{id}\">", node.tag_name().name()),
            None => format!("line {line} <{}>", node.tag_name().name()),
        }
    }

    fn problem(&mut self, kind: Kind, node: Node<'_, '_>, what: String) {
        let label = self.label(node);
        self.problems.push(Problem {
            kind,
            message: format!("{label}: {what}"),
        });
    }

    /// One element and what it holds.
    fn element(&mut self, node: Node<'_, '_>, outer: &Inherited) {
        let name = node.tag_name().name();
        if matches!(name, "title" | "desc" | "metadata") {
            return;
        }
        let mut inherited = outer.clone();
        for (property, value) in declarations(node) {
            self.declaration(node, property, value);
            if property == "fill" {
                inherited.filled = true;
            }
            if property == "font-family" {
                inherited.family = Some(value.to_owned());
            }
        }
        if name == "clipPath" {
            inherited.clip = true;
        }
        if matches!(name, "linearGradient" | "radialGradient") {
            self.problem(
                Kind::Color,
                node,
                "a gradient blends its stops into colors no token holds; fill with one token"
                    .to_owned(),
            );
        }
        if !inherited.clip && !inherited.filled && (FILLED.contains(&name) || name == "text") {
            self.problem(
                Kind::Color,
                node,
                "no fill on it or above it, so it draws in the default black, which is not a \
                 token; set `fill` to a token or `none`"
                    .to_owned(),
            );
        }
        if name == "text" {
            let first = inherited
                .family
                .as_deref()
                .and_then(|list| list.split(',').next())
                .map(|family| family.trim().trim_matches(['\'', '"']));
            if !first.is_some_and(|family| FAMILIES.contains(&family)) {
                let named = inherited.family.as_deref().unwrap_or("no font-family");
                self.problem(
                    Kind::Font,
                    node,
                    format!(
                        "set in `{named}`; name Cascadia Mono (data, labels) or Archivo (prose) \
                         first"
                    ),
                );
            }
        }
        for child in node.children().filter(Node::is_element) {
            self.element(child, &inherited);
        }
    }

    /// One property set on `node`, as an attribute or in its `style`.
    fn declaration(&mut self, node: Node<'_, '_>, property: &str, value: &str) {
        if PAINTS.contains(&property) {
            if value != "none" && !is_reference(value) && !is_token(value) {
                self.problem(Kind::Color, node, not_a_token(property, value));
            }
        } else if COLORS.contains(&property) {
            if !is_token(value) {
                self.problem(Kind::Color, node, not_a_token(property, value));
            }
        } else if OPACITIES.contains(&property) && !is_opaque(value) {
            self.problem(
                Kind::Color,
                node,
                format!(
                    "`{property}` `{value}` blends a token into a color no token holds; draw at \
                     full opacity"
                ),
            );
        }
    }
}

/// The properties `node` sets, as attributes and then in its `style`, values trimmed.
fn declarations<'n>(node: Node<'n, '_>) -> Vec<(&'n str, &'n str)> {
    let mut out: Vec<(&str, &str)> = node
        .attributes()
        .filter(|attribute| attribute.namespace().is_none())
        .map(|attribute| (attribute.name(), attribute.value().trim()))
        .collect();
    if let Some(style) = node.attribute("style") {
        for declaration in style.split(';') {
            if let Some((property, value)) = declaration.split_once(':') {
                out.push((property.trim(), value.trim()));
            }
        }
    }
    out
}

/// Whether `value` is one of [`TOKENS`], written `#RRGGBB` or `#RGB` in either case.
fn is_token(value: &str) -> bool {
    let Some(hex) = value.strip_prefix('#') else {
        return false;
    };
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return false;
    }
    let full = match hex.len() {
        6 => hex.to_ascii_uppercase(),
        3 => hex
            .chars()
            .flat_map(|c| [c, c])
            .collect::<String>()
            .to_ascii_uppercase(),
        _ => return false,
    };
    TOKENS.iter().any(|(_, token)| token[1..] == full)
}

/// Whether `value` is a bare reference to an element of the figure's own, with no fallback.
fn is_reference(value: &str) -> bool {
    value
        .strip_prefix("url(#")
        .and_then(|rest| rest.strip_suffix(')'))
        .is_some_and(|id| !id.is_empty() && !id.contains([')', ' ']))
}

/// Whether the opacity `value` is full: 1 or 100%.
fn is_opaque(value: &str) -> bool {
    match value.strip_suffix('%') {
        Some(percent) => percent.trim().parse::<f64>() == Ok(100.0),
        None => value.parse::<f64>() == Ok(1.0),
    }
}

/// The message for a paint that is not a token.
fn not_a_token(property: &str, value: &str) -> String {
    format!(
        "`{property}` `{value}` is not a color of the product system's tokens (the light theme's \
         roles and the signal fills, foundations.md Rev A); use a role's value"
    )
}

#[cfg(test)]
mod tests;
