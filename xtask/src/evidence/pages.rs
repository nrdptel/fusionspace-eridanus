//! The guide's pages as the evidence check reads them: which pages, the blocks of each, the gap a
//! block states and the rows it links.

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use super::{PAGE_NAME, REPOSITORY, SUMMARY};

/// The pages `docs/SUMMARY.md` lists that the check doesn't read, from `docs/`: the records and
/// the process (the roadmap and its decisions, the plan, the writing guide, the release steps),
/// which state no claim about the software, and the report's own page, whose rows restate every
/// gap. The one place to change which pages are read.
pub(crate) const EXCLUDED: [&str; 5] = [
    "decisions-and-roadmap.md",
    "plan.md",
    "writing.md",
    "releasing.md",
    PAGE_NAME,
];

/// The phrases that state a gap in what backs a claim, lowercase. A block of a guide page whose
/// text holds one, as whole words outside quotation marks ([`gap`]), must link the register's
/// row for the gap. A phrase ending in `*` would take any ending (`invent*`, "invented" and
/// "invents"); none does yet. Removing a phrase takes a decision record amending ADR-278: a test
/// pins the list.
pub(crate) const PHRASES: [&str; 23] = [
    "no sample",
    "invented files only",
    "three invented files",
    "against an invented",
    "one invented flight",
    "assumed:",
    "is assumed",
    "are assumed",
    "unvalidated",
    "not validated",
    "checked against nothing",
    "least sure",
    "nothing measures which",
    "nothing here is checked",
    "no oracle coverage",
    "tests alone",
    "one real",
    "stand-in searches",
    "outside the 5% target",
    "rests on one",
    "unchecked",
    "assumptions, not measurements",
    "differ is not traced",
];

/// One block of a page: a paragraph, a heading, a table's row, or a list item's own text.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Block {
    /// The line it starts on, from 1.
    pub line: usize,
    /// Its text as a reader sees it, inline code included, each run of white space one space.
    pub text: String,
    /// The destination of each link in it, as written.
    pub links: Vec<String>,
}

/// The pages `summary` (the text of `docs/SUMMARY.md`) lists, from `docs/`, in its order, once
/// each, less [`EXCLUDED`].
pub(crate) fn scanned(summary: &str) -> Vec<String> {
    let mut pages = Vec::new();
    for event in Parser::new_ext(summary, crate::site::options()) {
        let Event::Start(Tag::Link { dest_url, .. }) = event else {
            continue;
        };
        let dest = dest_url.as_ref();
        if is_outside(dest) || dest.contains('#') || !dest.ends_with(".md") {
            continue;
        }
        let Some(name) = join("", dest) else { continue };
        if name != SUMMARY && !EXCLUDED.contains(&name.as_str()) && !pages.contains(&name) {
            pages.push(name);
        }
    }
    pages
}

/// The blocks of a page, by the line each starts on. A paragraph inside a list item or a quote is a
/// block of its own; a list item's text outside any paragraph (a tight list's) is the item's.
/// Code blocks belong to no block; raw HTML's visible text joins the block it stands in, or is
/// its own block where it stands alone.
pub(crate) fn blocks(text: &str) -> Vec<Block> {
    let line = line_index(text);
    let mut open: Vec<Block> = Vec::new();
    let mut done: Vec<Block> = Vec::new();
    let mut code = 0_usize;
    // An HTML block, read whole before its text is taken: a comment may span its lines.
    let mut html: Option<(usize, String)> = None;
    for (event, range) in Parser::new_ext(text, crate::site::options()).into_offset_iter() {
        let at = line(range.start);
        match event {
            Event::Start(
                Tag::Paragraph | Tag::Heading { .. } | Tag::TableHead | Tag::TableRow | Tag::Item,
            ) => open.push(Block {
                line: at,
                ..Block::default()
            }),
            Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::TableHead
                | TagEnd::TableRow
                | TagEnd::Item,
            ) => done.extend(open.pop()),
            Event::Start(Tag::CodeBlock(_)) => code += 1,
            Event::End(TagEnd::CodeBlock) => code = code.saturating_sub(1),
            Event::Start(Tag::TableCell) | Event::SoftBreak | Event::HardBreak => {
                push_text(&mut open, &mut done, at, " ");
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                push_links(&mut open, &mut done, at, vec![dest_url.to_string()]);
            }
            Event::Text(words)
            | Event::Code(words)
            | Event::InlineMath(words)
            | Event::DisplayMath(words) => {
                if code == 0 {
                    push_text(&mut open, &mut done, at, &words);
                }
            }
            Event::Start(Tag::HtmlBlock) => html = Some((at, String::new())),
            Event::Html(raw) => match html.as_mut() {
                Some((_, block)) => block.push_str(&raw),
                None => push_html(&mut open, &mut done, at, &raw),
            },
            Event::End(TagEnd::HtmlBlock) => {
                if let Some((start, raw)) = html.take() {
                    push_html(&mut open, &mut done, start, &raw);
                }
            }
            Event::InlineHtml(raw) => push_html(&mut open, &mut done, at, &raw),
            _ => {}
        }
    }
    done.extend(open.drain(..).rev());
    for block in &mut done {
        block.text = block.text.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    done.retain(|block| !block.text.is_empty() || !block.links.is_empty());
    done.sort_by_key(|block| block.line);
    done
}

/// Adds `words` to the innermost open block, or, with none open, as a block of its own.
fn push_text(open: &mut [Block], done: &mut Vec<Block>, line: usize, words: &str) {
    match open.last_mut() {
        Some(block) => block.text.push_str(words),
        None => done.push(Block {
            line,
            text: words.to_owned(),
            links: Vec::new(),
        }),
    }
}

/// Adds `links` to the innermost open block, or, with none open, as a block of its own.
fn push_links(open: &mut [Block], done: &mut Vec<Block>, line: usize, links: Vec<String>) {
    match open.last_mut() {
        Some(block) => block.links.extend(links),
        None => done.push(Block {
            line,
            text: String::new(),
            links,
        }),
    }
}

/// Adds raw HTML's visible text and the `href` of each `<a>` in it to the innermost open block,
/// or, with none open, as a block of its own.
fn push_html(open: &mut [Block], done: &mut Vec<Block>, line: usize, raw: &str) {
    let text = visible_text(raw);
    let links = hrefs(raw);
    match open.last_mut() {
        Some(block) => {
            block.text.push_str(&text);
            block.links.extend(links);
        }
        None => done.push(Block { line, text, links }),
    }
}

/// The line of each byte offset of `text`, counted from 1.
fn line_index(text: &str) -> impl Fn(usize) -> usize {
    let newlines: Vec<usize> = text.match_indices('\n').map(|(at, _)| at).collect();
    move |offset| 1 + newlines.partition_point(|&at| at < offset)
}

/// The elements whose tags stand inside a word's run of text: dropping them joins what is on
/// either side, where any other tag leaves a space.
const INLINE: [&str; 14] = [
    "a", "abbr", "b", "code", "em", "i", "kbd", "mark", "small", "span", "strong", "sub", "sup",
    "wbr",
];

/// The text a reader sees in raw HTML: without its comments and tags, a few entities read.
fn visible_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        if let Some(comment) = rest.strip_prefix("<!--") {
            rest = comment.split_once("-->").map_or("", |(_, after)| after);
            out.push(' ');
        } else if (rest[1..].chars().next())
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '/' || c == '!')
        {
            let (tag, after) = rest[1..].split_once('>').unwrap_or((&rest[1..], ""));
            let name: String = (tag.trim_start_matches('/').chars())
                .take_while(char::is_ascii_alphanumeric)
                .map(|c| c.to_ascii_lowercase())
                .collect();
            if !INLINE.contains(&name.as_str()) {
                out.push(' ');
            }
            rest = after;
        } else {
            // A `<` that opens no tag is text.
            out.push('<');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out.replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&rsquo;", "'")
        .replace("&amp;", "&")
}

/// The `href` of each `<a>` element in raw HTML, outside its comments.
fn hrefs(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = html;
    while let Some(at) = rest.find('<') {
        rest = &rest[at..];
        if let Some(comment) = rest.strip_prefix("<!--") {
            rest = comment.split_once("-->").map_or("", |(_, after)| after);
            continue;
        }
        let (tag, after) = rest[1..].split_once('>').unwrap_or((&rest[1..], ""));
        rest = after;
        let is_a = tag
            .strip_prefix(['a', 'A'])
            .is_some_and(|after| after.starts_with(char::is_whitespace));
        if is_a && let Some(href) = attribute(tag, "href") {
            found.push(href);
        }
    }
    found
}

/// The value of the attribute `name` in a tag's text (what stands between `<` and `>`), quoted
/// with `"` or `'`.
fn attribute(tag: &str, name: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    let mut from = 0;
    while let Some(found) = lower[from..].find(name) {
        let at = from + found;
        from = at + name.len();
        let spaced = lower[..at].ends_with(char::is_whitespace);
        let value = lower[from..]
            .trim_start()
            .strip_prefix('=')
            .map(str::trim_start);
        if let (true, Some(value)) = (spaced, value) {
            let start = tag.len() - value.len();
            let quote = value.chars().next()?;
            if quote != '"' && quote != '\'' {
                return None;
            }
            let inner = &tag[start + 1..];
            return inner.split_once(quote).map(|(value, _)| value.to_owned());
        }
    }
    None
}

/// The phrase of [`PHRASES`] that `text` (a block's) states a gap in, as [`gap_in`] finds it.
pub(crate) fn gap(text: &str) -> Option<&'static str> {
    gap_in(text, &PHRASES)
}

/// The phrase of `phrases` that `text` states a gap in: the first in the text, on a tie the
/// longest. Matching ignores case, reads a curly apostrophe as a straight one, and skips what
/// stands in quotation marks ([`unquoted`]): a page quoting another document ("is assumed and
/// should not be specified") states no gap of its own.
pub(crate) fn gap_in<'a>(text: &str, phrases: &[&'a str]) -> Option<&'a str> {
    let text = unquoted(&text.to_lowercase()).replace(['\u{2019}', '\u{2018}'], "'");
    (phrases.iter())
        .filter_map(|phrase| first_match(&text, phrase).map(|at| (at, *phrase)))
        .min_by_key(|(at, phrase)| (*at, std::cmp::Reverse(phrase.len())))
        .map(|(_, phrase)| phrase)
}

/// `text` without what stands between a pair of straight double quotes, or between a curly
/// opening and closing double quote, each span a space; a quote with no partner drops nothing.
pub(crate) fn unquoted(text: &str) -> String {
    let text = without_pairs(text, '"', '"');
    without_pairs(&text, '\u{201c}', '\u{201d}')
}

/// `text` without each span from `open` to the next `close`, both included, each a space.
fn without_pairs(text: &str, open: char, close: char) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(open) {
        let inside = &rest[start + open.len_utf8()..];
        let Some(end) = inside.find(close) else {
            break;
        };
        out.push_str(&rest[..start]);
        out.push(' ');
        rest = &inside[end + close.len_utf8()..];
    }
    out.push_str(rest);
    out
}

/// Whether a link's destination is the repository's list of issues itself, less any query, any
/// fragment and a trailing slash, or the list of one label's issues, rather than one numbered
/// issue.
pub(crate) fn is_issue_list(dest: &str) -> bool {
    let dest = dest.split(['?', '#']).next().unwrap_or_default();
    let dest = dest.trim_end_matches('/');
    let repository = format!("https://github.com/{REPOSITORY}/");
    dest.strip_prefix(repository.as_str())
        .is_some_and(|rest| rest == "issues" || rest.starts_with("labels/"))
}

/// Where `phrase` first stands in `text` as whole words: no letter or digit right before it,
/// nor right after it unless it ends in `*`.
pub(crate) fn first_match(text: &str, phrase: &str) -> Option<usize> {
    let (stem, any_ending) = match phrase.strip_suffix('*') {
        Some(stem) => (stem, true),
        None => (phrase, false),
    };
    if stem.is_empty() {
        return None;
    }
    let mut from = 0;
    while let Some(found) = text[from..].find(stem) {
        let at = from + found;
        let before = text[..at].chars().next_back();
        let after = text[at + stem.len()..].chars().next();
        if !before.is_some_and(char::is_alphanumeric)
            && (any_ending || !after.is_some_and(char::is_alphanumeric))
        {
            return Some(at);
        }
        from = at + text[at..].chars().next().map_or(1, char::len_utf8);
    }
    None
}

/// The row a link on `page` (from `docs/`) names, as `G12`: a destination that resolves, from
/// the page's directory, to the report's page with the fragment `g` and a number, in lowercase
/// as the report writes its anchors (`#G1` lands on no row, since an anchor's case counts).
/// `None` for any other link.
pub(crate) fn row_named(page: &str, dest: &str) -> Option<String> {
    if is_outside(dest) {
        return None;
    }
    let (path, fragment) = dest.split_once('#')?;
    let path = path.split_once('?').map_or(path, |(path, _)| path);
    // A link to a fragment of its own page; the report's page is never read.
    if path.is_empty() {
        return None;
    }
    if join(parent(page), path)? != PAGE_NAME {
        return None;
    }
    let number = fragment.strip_prefix('g')?;
    (!number.is_empty() && number.bytes().all(|b| b.is_ascii_digit())).then(|| format!("G{number}"))
}

/// The way from `page`'s directory up to `docs/`: `../` for `format/flight-logs.md`, nothing for
/// a page at the top.
pub(crate) fn up_to_docs(page: &str) -> String {
    "../".repeat(page.matches('/').count())
}

/// Whether a link leaves the guide's own files: a web address, a mail address or a path from
/// the root.
fn is_outside(dest: &str) -> bool {
    dest.contains("://") || dest.starts_with("mailto:") || dest.starts_with('/')
}

/// The directory of a `/`-separated path: `format` for `format/flight-logs.md`.
fn parent(name: &str) -> &str {
    name.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// The relative path `rel` resolved against the directory `dir`, both `/`-separated; `None` if
/// it climbs out of the directory they are relative to.
fn join(dir: &str, rel: &str) -> Option<String> {
    let mut parts: Vec<&str> = dir.split('/').filter(|part| !part.is_empty()).collect();
    for part in rel.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            _ => parts.push(part),
        }
    }
    Some(parts.join("/"))
}
