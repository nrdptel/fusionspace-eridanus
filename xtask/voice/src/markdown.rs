//! What the voice check and the site's checks both read in a page's Markdown: the extensions
//! both GitHub and the site render ([`options`]), a model page's *In short* ([`in_short`]), and
//! the internal labels ([`labels`]). `cargo xtask site` uses them from here, so the two checks
//! read a page the same way.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// The site's source directory, relative to the workspace root, as `book.toml` sets it.
pub const SOURCE: &str = "docs";

/// The directory under [`SOURCE`] that holds the model pages, one per model. Each opens with
/// *In short*.
pub const MODELS: &str = "physics";
/// The heading of the section a model page opens with.
const IN_SHORT: &str = "In short";
/// The bold labels that open the items of a model page's *In short* list, in this order: what it
/// models, its source, how well it is validated, and what it leaves out (M0.4 in
/// `docs/roadmap-done.md`).
pub const IN_SHORT_ITEMS: [&str; 4] = [
    "What it models:",
    "Sources:",
    "How well it is validated:",
    "What it leaves out:",
];

/// The Markdown extensions that both GitHub and the site render; the site's math through its
/// preprocessor (`cargo xtask site-math`), so a formula is never read as prose.
pub fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
        | Options::ENABLE_MATH
}

/// The line of each byte offset of `text`, counted from 1.
pub fn line_index(text: &str) -> impl Fn(usize) -> usize {
    let newlines: Vec<usize> = text.match_indices('\n').map(|(at, _)| at).collect();
    move |offset| 1 + newlines.partition_point(|&at| at < offset)
}

/// Where a page's *In short* section is, from its heading to the next heading of level 1 or 2,
/// in bytes; `None` if it has none.
pub fn in_short_span(text: &str) -> Option<std::ops::Range<usize>> {
    let mut start = None;
    let mut heading: Option<(usize, String)> = None;
    for (event, range) in Parser::new_ext(text, options()).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1 | HeadingLevel::H2,
                ..
            }) => {
                if let Some(start) = start {
                    return Some(start..range.start);
                }
                heading = Some((range.start, String::new()));
            }
            Event::Text(words) => {
                if let Some((_, heading)) = heading.as_mut() {
                    heading.push_str(&words);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((at, heading)) = heading.take()
                    && heading.trim() == IN_SHORT
                {
                    start = Some(at);
                }
            }
            _ => {}
        }
    }
    start.map(|start| start..text.len())
}

/// How a model page opens, for the messages of [`in_short`].
const IN_SHORT_FORM: &str = "a model page opens with *In short*: right under its `# title` and \
                             its lead (one paragraph, `parts.rs`), a `## In short` section holding only a bulleted list of four items, \
                             which open with `**What it models:**`, `**Sources:**`, `**How well it \
                             is validated:**` and `**What it leaves out:**`, in that order, each \
                             followed by its answer";

/// A Markdown event and the bytes of the page it comes from.
type Spanned<'a> = (Event<'a>, std::ops::Range<usize>);

/// What is wrong with the *In short* a model page opens with, if anything, as (line, message):
/// the first thing that departs from [`IN_SHORT_FORM`].
pub fn in_short(text: &str) -> Option<(usize, String)> {
    let line = line_index(text);
    let fail = |offset: usize, what: &str| Some((line(offset), format!("{what}: {IN_SHORT_FORM}")));
    let events: Vec<Spanned<'_>> = Parser::new_ext(text, options())
        .into_offset_iter()
        .collect();
    let offset = |at: usize| events.get(at).map_or(text.len(), |(_, range)| range.start);

    // The title.
    if !matches!(
        events.first(),
        Some((
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1,
                ..
            }),
            _
        ))
    ) {
        return fail(offset(0), "the page doesn't open with its title, `# ...`");
    }
    let Some(title_end) = events
        .iter()
        .position(|(event, _)| matches!(event, Event::End(TagEnd::Heading(_))))
    else {
        return fail(text.len(), "the title never ends");
    };

    // `## In short`, right under it and the lead, the paragraph every title has under it, which
    // `parts.rs` checks.
    let mut at = title_end + 1;
    if matches!(events.get(at), Some((Event::Start(Tag::Paragraph), _))) {
        at += events[at..]
            .iter()
            .position(|(event, _)| matches!(event, Event::End(TagEnd::Paragraph)))
            .map_or(events.len() - at, |end| end + 1);
    }
    if !matches!(
        events.get(at),
        Some((
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }),
            _
        ))
    ) {
        return fail(offset(at), "the title is not followed by `## In short`");
    }
    let heading_at = offset(at);
    let mut heading = String::new();
    at += 1;
    while let Some((event, _)) = events.get(at) {
        at += 1;
        match event {
            Event::End(TagEnd::Heading(_)) => break,
            Event::Text(text) | Event::Code(text) => heading.push_str(text),
            _ => {}
        }
    }
    if heading.trim() != IN_SHORT {
        return fail(
            heading_at,
            &format!(
                "the first section is `## {}`, not `## {IN_SHORT}`",
                heading.trim()
            ),
        );
    }

    // Its bulleted list, split into items.
    if !matches!(events.get(at), Some((Event::Start(Tag::List(None)), _))) {
        return fail(
            offset(at),
            "`## In short` doesn't open with a bulleted list",
        );
    }
    let list_at = offset(at);
    at += 1;
    let mut items: Vec<(usize, &[Spanned<'_>])> = Vec::new();
    let mut depth = 0usize;
    let mut item_from = at;
    let list_end = loop {
        let Some((event, _)) = events.get(at) else {
            return fail(text.len(), "the list never ends");
        };
        match event {
            Event::Start(_) => {
                if depth == 0 {
                    item_from = at;
                }
                depth += 1;
            }
            Event::End(_) if depth == 0 => break at,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    items.push((offset(item_from), &events[item_from + 1..at]));
                }
            }
            _ => {}
        }
        at += 1;
    };
    // The section holds nothing else: the next thing is a heading, or the end of the page.
    match events.get(list_end + 1) {
        None
        | Some((
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1 | HeadingLevel::H2,
                ..
            }),
            _,
        )) => {}
        Some((_, range)) => {
            return fail(
                range.start,
                "`## In short` holds more than its list; move the rest under a heading of its own",
            );
        }
    }

    // Each item: a bold label, then its answer.
    let mut labels = Vec::new();
    for (item_at, item) in &items {
        let body = match item.first() {
            Some((Event::Start(Tag::Paragraph), _)) => &item[1..],
            _ => item,
        };
        let mut label = None;
        if let Some((Event::Start(Tag::Strong), _)) = body.first()
            && let Some(close) = body
                .iter()
                .position(|(event, _)| matches!(event, Event::End(TagEnd::Strong)))
        {
            let text: String = body[1..close]
                .iter()
                .filter_map(|(event, _)| match event {
                    Event::Text(text) | Event::Code(text) => Some(text.as_ref()),
                    // A label wrapped over two lines reads as one.
                    Event::SoftBreak | Event::HardBreak => Some(" "),
                    _ => None,
                })
                .collect();
            let answered = body[close + 1..].iter().any(|(event, _)| match event {
                Event::Text(text) | Event::Code(text) => text.chars().any(char::is_alphanumeric),
                _ => false,
            });
            if !answered {
                return fail(*item_at, &format!("`**{text}**` has no answer after it"));
            }
            label = Some(text);
        }
        let Some(label) = label else {
            return fail(*item_at, "an item doesn't open with its label in bold");
        };
        labels.push(label);
    }
    if labels != IN_SHORT_ITEMS {
        return fail(
            list_at,
            &format!(
                "its items open with {}",
                labels
                    .iter()
                    .map(|label| format!("`**{label}**`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }
    None
}

/// The internal labels in `text`, each a whole word: a Loft lesson (`L12`), a decision
/// (`ADR-008`), or a milestone (`M1.5b`, `M2.1b1`).
pub fn labels(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        // Labels start with an ASCII letter, so `at` is on a char boundary whenever one matches.
        let starts_word = at == 0 || !bytes[at - 1].is_ascii_alphanumeric();
        if starts_word
            && matches!(bytes[at], b'L' | b'A' | b'M')
            && let Some(len) = label_at(&bytes[at..])
        {
            found.push(&text[at..at + len]);
            at += len;
        } else {
            at += 1;
        }
    }
    found
}

/// The length of the label at the start of `bytes`, if one is there and ends at a word boundary.
fn label_at(bytes: &[u8]) -> Option<usize> {
    let digits = |from: usize| {
        let count = bytes.get(from..).map_or(0, |rest| {
            rest.iter().take_while(|b| b.is_ascii_digit()).count()
        });
        (count > 0).then_some(from + count)
    };
    let end = if bytes.starts_with(b"ADR-") {
        digits(4)?
    } else if bytes.first() == Some(&b'L') {
        digits(1)?
    } else if bytes.first() == Some(&b'M') {
        // A milestone: M, digits, a dot, digits, then an optional letter and digits.
        let major = digits(1)?;
        if bytes.get(major) != Some(&b'.') {
            return None;
        }
        let mut end = digits(major + 1)?;
        if bytes.get(end).is_some_and(u8::is_ascii_lowercase) {
            end = digits(end + 1).unwrap_or(end + 1);
        }
        end
    } else {
        return None;
    };
    (!bytes.get(end).is_some_and(u8::is_ascii_alphanumeric)).then_some(end)
}
