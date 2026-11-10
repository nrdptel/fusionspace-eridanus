//! The site's search index, rebuilt so each field holds only its own words (#414).
//!
//! mdBook 0.5.4 builds `searchindex.js` with elasticlunr-rs 3.0.2, which hands each field of a
//! section the words of every field before it: the body's postings hold the heading's words, and
//! the breadcrumbs' postings hold the heading's and the whole body's. In main's index at b7313b32
//! (fifteen pages left out) the breadcrumbs' postings were 3.65 MB of the 10.0 MB (9,999,756 bytes)
//! mdBook counted, 52 KB once rebuilt, and that copy is what held the index at mdBook's 10 MB
//! warning and kept pages out of search. elasticlunr-rs 3.1.0 fixed it (its changelog's "tokens
//! from earlier fields leaking into later fields' indices", pull request #60). mdBook's main branch
//! takes that version, its latest release (0.5.4) doesn't, and until one does [`rebuild`] does the
//! same.
//!
//! - **What it reads.** mdBook keeps each section's heading, body and breadcrumbs as text in the
//!   index's document store, already cut into sections and with runs of white space collapsed. The
//!   rebuild indexes that text again, field by field, splitting words as mdBook does (at white
//!   space and hyphens, lowercased, words over 80 bytes left out: [`tokenize`]) and running
//!   elasticlunr's English pipeline (trimmer, stop words, stemmer).
//! - **What it keeps.** Everything but the three fields' postings: the options, the sections'
//!   addresses, the document store, the field list and pipeline. The rebuilt document store is
//!   compared with mdBook's and must be the same, each section's word count in each field
//!   included, so a section missed or a field's words counted differently fails the build rather
//!   than shipping a thinner index. (A word stemmed differently, with the same count,
//!   would pass: the stemmer is elasticlunr's own, 3.1.0's rather than 3.0.2's.)
//! - **What changes for a reader.** A section is found by the same words. Its body and
//!   breadcrumbs no longer count its heading's and body's words twice or three times, so ranking
//!   follows the weights mdBook gives each field (title 2, body 1, breadcrumbs 1).
//! - **Its size,** checked against [`LIMIT_GZIP_BYTES`], stands for what a reader downloads
//!   when they open the search: the file compressed with gzip at the default level ([`gzip_len`]).
//!   GitHub Pages compresses a little less well: for main's file at b7313b32 it sent 1,754,850
//!   bytes (measured 2026-11-10 05:10 UTC), against 1,675,059 here. The limit and the file are
//!   measured the same way, so the comparison holds.

use std::io::Write as _;
use std::path::Path;

use serde_json::Value;

/// The index's file in the built site.
pub(super) const FILE: &str = "searchindex.js";

/// How mdBook 0.5 opens and closes the file: the index's JSON as one JavaScript string, in
/// single quotes, with each backslash doubled and each single quote escaped.
const PREFIX: &str = "window.search = Object.assign(window.search, JSON.parse('";
const SUFFIX: &str = "'));";

/// The words mdBook logs when the index it built passes 10 MB. That index is the one [`rebuild`]
/// replaces, so the build checks the shipped file's size instead ([`LIMIT_GZIP_BYTES`]).
pub(super) const MDBOOK_SIZE_WARNING: &str = "search index is very large (";

/// The most the shipped index may weigh compressed by [`gzip_len`]: 1,675,059 bytes, main's file
/// at b7313b32 (10,020,860 bytes before compression) measured the same way, the last build before
/// this rebuild, with fifteen pages left out of search to stay under mdBook's warning. Every page
/// back in, the rebuilt index must still cost a reader no more than that.
pub(super) const LIMIT_GZIP_BYTES: usize = 1_675_059;

/// mdBook leaves out words longer than this, in bytes.
const MAX_WORD_BYTES: usize = 80;

/// What the rebuild did, for the build's summary line.
#[derive(Debug, PartialEq)]
pub(super) struct Rebuilt {
    /// Sections in the index.
    pub sections: usize,
    /// The file as mdBook wrote it, in bytes.
    pub mdbook_bytes: usize,
    /// The file as rebuilt, in bytes.
    pub bytes: usize,
    /// The rebuilt file compressed with gzip ([`gzip_len`]), in bytes.
    pub gzip_bytes: usize,
    /// A word in a section's field, counted over the three fields: mdBook's, then the rebuild's.
    pub mdbook_postings: usize,
    pub postings: usize,
}

/// Rebuilds `output`'s [`FILE`] in place and checks it against [`LIMIT_GZIP_BYTES`].
pub(super) fn rebuild(output: &Path) -> Result<Rebuilt, String> {
    rebuild_within(output, LIMIT_GZIP_BYTES)
}

/// [`rebuild`], held to `limit` bytes as gzip sends the file.
fn rebuild_within(output: &Path, limit: usize) -> Result<Rebuilt, String> {
    let path = output.join(FILE);
    let text = std::fs::read_to_string(&path)
        .map_err(|err| format!("could not read the search index {}: {err}", path.display()))?;
    let (rebuilt, report) = rebuild_text(&text)?;
    std::fs::write(&path, &rebuilt)
        .map_err(|err| format!("could not write the search index {}: {err}", path.display()))?;
    if report.gzip_bytes > limit {
        return Err(format!(
            "the search index weighs {} bytes compressed with gzip, more than the {limit} its \
             last build before the rebuild weighed, measured the same way \
             (xtask/src/site/search.rs): shorten the words a page added; raise the limit only \
             with a decision record that measures the new one",
            report.gzip_bytes
        ));
    }
    Ok(report)
}

/// [`rebuild`] on the file's text: the rebuilt text and what changed.
fn rebuild_text(text: &str) -> Result<(String, Rebuilt), String> {
    let json = text
        .strip_prefix(PREFIX)
        .and_then(|rest| rest.strip_suffix(SUFFIX))
        .ok_or_else(|| {
            format!("the search index doesn't open with `{PREFIX}` and close with `{SUFFIX}`")
        })?;
    let mut root: Value = serde_json::from_str(&unescape(json)?)
        .map_err(|err| format!("the search index's JSON doesn't parse: {err}"))?;
    let sections = root
        .get("doc_urls")
        .and_then(Value::as_array)
        .map(Vec::len)
        .ok_or("the search index has no `doc_urls` list")?;
    let old = root
        .get_mut("index")
        .ok_or("the search index has no `index`")?;
    let fresh = reindex(old, sections)?;
    let mdbook_postings = postings(old)?;
    let new_postings = postings(&fresh)?;
    *old = fresh;
    let json = serde_json::to_string(&root)
        .map_err(|err| format!("could not write the search index's JSON: {err}"))?;
    let rebuilt = format!("{PREFIX}{}{SUFFIX}", escape(&json));
    let report = Rebuilt {
        sections,
        mdbook_bytes: text.len(),
        bytes: rebuilt.len(),
        gzip_bytes: gzip_len(rebuilt.as_bytes())?,
        mdbook_postings,
        postings: new_postings,
    };
    Ok((rebuilt, report))
}

/// The three fields an mdBook index holds, in the order mdBook adds them.
const FIELDS: [&str; 3] = ["title", "body", "breadcrumbs"];

/// A fresh elasticlunr index of the sections `old`'s document store holds, refs `0` to
/// `sections - 1`, as JSON. Fails unless everything but the fields' postings matches `old`.
fn reindex(old: &Value, sections: usize) -> Result<Value, String> {
    let fields = old.get("fields").and_then(Value::as_array);
    let expected: Vec<Value> = FIELDS.iter().map(|&f| Value::from(f)).collect();
    if fields != Some(&expected) {
        return Err(format!(
            "the search index's fields are {fields:?}, not mdBook's {FIELDS:?}"
        ));
    }
    let docs = old
        .pointer("/documentStore/docs")
        .and_then(Value::as_object)
        .ok_or("the search index has no document store")?;
    if docs.len() != sections {
        return Err(format!(
            "the search index stores {} sections' text and lists {sections} addresses",
            docs.len()
        ));
    }
    let mut index = elasticlunr::IndexBuilder::new()
        .add_field_with_tokenizer(FIELDS[0], Box::new(tokenize))
        .add_field_with_tokenizer(FIELDS[1], Box::new(tokenize))
        .add_field_with_tokenizer(FIELDS[2], Box::new(tokenize))
        .build();
    for section in 0..sections {
        let reference = section.to_string();
        let doc = docs
            .get(&reference)
            .ok_or_else(|| format!("the search index stores no text for section {reference}"))?;
        let mut texts = Vec::with_capacity(FIELDS.len());
        for field in FIELDS {
            let text = doc.get(field).and_then(Value::as_str).ok_or_else(|| {
                format!("the search index's section {reference} has no `{field}` text")
            })?;
            texts.push(text);
        }
        index.add_doc(&reference, texts);
    }
    let fresh = serde_json::to_value(&index)
        .map_err(|err| format!("could not turn the rebuilt search index into JSON: {err}"))?;
    // Everything but the postings must be mdBook's: the document store holds each section's text
    // and its word count in each field, so a section missed or counted differently shows here.
    for key in old
        .as_object()
        .into_iter()
        .flat_map(|map| map.keys())
        .chain(fresh.as_object().into_iter().flat_map(|map| map.keys()))
    {
        if key != "index" && old.get(key) != fresh.get(key) {
            return Err(format!(
                "the rebuilt search index's `{key}` differs from mdBook's, so the rebuild reads \
                 the sections differently than mdBook did (xtask/src/site/search.rs)"
            ));
        }
    }
    Ok(fresh)
}

/// Splits a field's text into words as mdBook 0.5 does before elasticlunr's pipeline:
/// elasticlunr's white-space split (at white space and hyphens, each word lowercased), with words
/// over [`MAX_WORD_BYTES`] left out. The document store's word counts, which [`reindex`]
/// compares, hold the rebuild to mdBook's split.
fn tokenize(text: &str) -> Vec<String> {
    let mut words = elasticlunr::lang::tokenize_whitespace(text);
    words.retain(|word| word.len() <= MAX_WORD_BYTES);
    words
}

/// The number of (word, section) postings over an index's three fields.
fn postings(index: &Value) -> Result<usize, String> {
    fn count(node: &Value) -> usize {
        node.as_object().map_or(0, |map| {
            map.iter()
                .map(|(key, child)| match key.as_str() {
                    "docs" => child.as_object().map_or(0, serde_json::Map::len),
                    "df" => 0,
                    _ => count(child),
                })
                .sum()
        })
    }
    FIELDS
        .iter()
        .map(|field| {
            index
                .pointer(&format!("/index/{field}/root"))
                .map(count)
                .ok_or_else(|| format!("the search index has no `{field}` postings"))
        })
        .sum()
}

/// The JSON inside mdBook's single-quoted string: `\\` back to `\`, `\'` back to `'`. mdBook
/// writes no other escape, so any other fails.
fn unescape(quoted: &str) -> Result<String, String> {
    let mut out = String::with_capacity(quoted.len());
    let mut chars = quoted.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some(escaped @ ('\\' | '\'')) => out.push(escaped),
                other => {
                    return Err(format!(
                        "the search index's string holds an escape mdBook doesn't write: \\{}",
                        other.map_or_else(|| "at its end".to_owned(), String::from)
                    ));
                }
            }
        } else if c == '\'' {
            return Err("the search index's string holds an unescaped single quote".to_owned());
        } else {
            out.push(c);
        }
    }
    Ok(out)
}

/// The reverse of [`unescape`], as mdBook writes it.
fn escape(json: &str) -> String {
    json.replace('\\', "\\\\").replace('\'', "\\'")
}

/// `bytes`' length compressed with gzip at the default level, as the site check measures a page's
/// download (flate2's pure-Rust backend; `gzip -6` and GitHub Pages give a little more).
fn gzip_len(bytes: &[u8]) -> Result<usize, String> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(bytes)
        .and_then(|()| encoder.finish())
        .map(|compressed| compressed.len())
        .map_err(|err| format!("could not compress the search index: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-section book's index as mdBook 0.5.4 writes it: a page "Alpha" whose heading is
    /// "Alpha page" and whose body is `zebra quokka don't a\b rail-exit`, a word of 81 `y`s, one
    /// of 80 `k`s and `words here.`, then a section "Second part" with body "mongoose giraffe
    /// text.". mdBook curls the apostrophe, splits `rail-exit` and leaves out the 81-byte word:
    /// 9 words in the first body.
    const FIXTURE: &str = include_str!("searchindex-fixture.txt");

    /// Each section's words in `field`, as the index's postings hold them.
    fn words(file: &str, field: &str) -> Vec<(String, Vec<String>)> {
        fn walk(node: &Value, prefix: &str, out: &mut Vec<(String, Vec<String>)>) {
            let Some(map) = node.as_object() else { return };
            for (key, child) in map {
                match key.as_str() {
                    "docs" => {
                        let docs: Vec<String> = child
                            .as_object()
                            .into_iter()
                            .flatten()
                            .map(|(d, _)| d.clone())
                            .collect();
                        if !docs.is_empty() {
                            out.push((prefix.to_owned(), docs));
                        }
                    }
                    "df" => {}
                    _ => walk(child, &format!("{prefix}{key}"), out),
                }
            }
        }
        let json = unescape(
            file.strip_prefix(PREFIX)
                .and_then(|r| r.strip_suffix(SUFFIX))
                .unwrap(),
        )
        .unwrap();
        let root: Value = serde_json::from_str(&json).unwrap();
        let mut out = Vec::new();
        walk(&root["index"]["index"][field]["root"], "", &mut out);
        out
    }

    fn section_words(file: &str, field: &str, section: &str) -> Vec<String> {
        words(file, field)
            .into_iter()
            .filter(|(_, docs)| docs.iter().any(|d| d == section))
            .map(|(word, _)| word)
            .collect()
    }

    #[test]
    fn mdbooks_index_carries_earlier_fields_words_and_the_rebuild_drops_them() {
        // mdBook's own file: section 0's body holds its heading's words, and section 1's
        // breadcrumbs hold its body's.
        assert!(section_words(FIXTURE, "body", "0").contains(&"alpha".to_owned()));
        assert!(section_words(FIXTURE, "breadcrumbs", "1").contains(&"mongoos".to_owned()));
        let (rebuilt, report) = rebuild_text(FIXTURE).unwrap();
        let k80 = "k".repeat(80);
        assert_eq!(
            section_words(&rebuilt, "body", "0"),
            [
                "a\\b",
                "don’t",
                "exit",
                "here",
                k80.as_str(),
                "quokka",
                "rail",
                "word",
                "zebra"
            ]
        );
        assert_eq!(
            section_words(&rebuilt, "breadcrumbs", "1"),
            ["alpha", "part", "second"]
        );
        assert_eq!(section_words(&rebuilt, "title", "1"), ["part", "second"]);
        assert_eq!(report.sections, 2);
        // Title 2 + 2, body 9 + 3, breadcrumbs 2 + 3 (the page name "alpha" repeats in
        // section 0's): mdBook's 37 counted the body twice and the heading three times.
        assert_eq!((report.mdbook_postings, report.postings), (37, 21));
        assert!(report.bytes < report.mdbook_bytes);
    }

    #[test]
    fn the_rebuild_keeps_everything_but_the_postings() {
        let (rebuilt, _) = rebuild_text(FIXTURE).unwrap();
        let parse = |file: &str| -> Value {
            serde_json::from_str(
                &unescape(
                    file.strip_prefix(PREFIX)
                        .and_then(|r| r.strip_suffix(SUFFIX))
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap()
        };
        let (mut before, mut after) = (parse(FIXTURE), parse(&rebuilt));
        for root in [&mut before, &mut after] {
            root["index"].as_object_mut().unwrap().remove("index");
        }
        assert_eq!(before, after);
        // Rebuilding the rebuilt file changes nothing.
        assert_eq!(rebuild_text(&rebuilt).unwrap().0, rebuilt);
    }

    #[test]
    fn a_changed_document_store_fails_the_rebuild() {
        // A word count that the rebuild's split doesn't give: mdBook read the text differently.
        let altered = FIXTURE.replace(
            r#""docInfo":{"0":{"body":9"#,
            r#""docInfo":{"0":{"body":10"#,
        );
        assert_ne!(altered, FIXTURE);
        let err = rebuild_text(&altered).unwrap_err();
        assert!(err.contains("`documentStore` differs"), "{err}");
        // A section's text missing.
        let missing = FIXTURE.replace(r#""1":{"body":"mongoose"#, r#""2":{"body":"mongoose"#);
        assert_ne!(missing, FIXTURE);
        let err = rebuild_text(&missing).unwrap_err();
        assert!(err.contains("no text for section 1"), "{err}");
    }

    #[test]
    fn a_file_mdbook_didnt_write_is_refused() {
        let err = rebuild_text("window.search = {};").unwrap_err();
        assert!(err.contains("doesn't open with"), "{err}");
        assert!(
            unescape(r"a\nb")
                .unwrap_err()
                .contains("escape mdBook doesn't write")
        );
        assert!(
            unescape("a'b")
                .unwrap_err()
                .contains("unescaped single quote")
        );
        assert!(unescape("ab\\").unwrap_err().contains("at its end"));
    }

    #[test]
    fn escaping_round_trips_quotes_and_backslashes() {
        for json in [r#"{"a":"don't \\ stop"}"#, r"\\'", "plain", ""] {
            assert_eq!(unescape(&escape(json)).unwrap(), json);
        }
        assert_eq!(escape(r"it's a\b"), r"it\'s a\\b");
    }

    #[test]
    fn words_split_as_mdbook_splits_them() {
        assert_eq!(
            tokenize("Rail-exit  speed\tIS"),
            ["rail", "exit", "speed", "is"]
        );
        // mdBook's bound is 80 bytes, written out so a moved constant fails here.
        let long = "x".repeat(81);
        let kept = "y".repeat(80);
        assert_eq!(tokenize(&format!("{long} {kept}")), [kept]);
        assert_eq!(tokenize("Don't  é-É"), ["don't", "é", "é"]);
    }

    #[test]
    fn a_heavier_index_than_the_limit_fails() {
        let dir = std::env::temp_dir().join(format!("hpr-search-limit-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(FILE), FIXTURE).unwrap();
        let report = rebuild_within(&dir, usize::MAX).unwrap();
        // Written back rebuilt.
        let written = std::fs::read_to_string(dir.join(FILE)).unwrap();
        assert_eq!(written.len(), report.bytes);
        assert_eq!(gzip_len(written.as_bytes()).unwrap(), report.gzip_bytes);
        // At its own size it passes; a byte less fails.
        std::fs::write(dir.join(FILE), FIXTURE).unwrap();
        assert!(rebuild_within(&dir, report.gzip_bytes).is_ok());
        std::fs::write(dir.join(FILE), FIXTURE).unwrap();
        let err = rebuild_within(&dir, report.gzip_bytes - 1).unwrap_err();
        assert!(err.contains("compressed with gzip"), "{err}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn no_page_is_left_out_of_search() {
        // book.toml's `[output.html.search.chapter]` table is how mdBook leaves a page out; the
        // rebuild made room for every page (#414), so the table must stay empty.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../book.toml");
        let book: toml::Table = std::fs::read_to_string(path).unwrap().parse().unwrap();
        let search = book
            .get("output")
            .and_then(|o| o.get("html"))
            .and_then(|h| h.get("search"))
            .expect("book.toml has an [output.html.search] table");
        assert!(
            search.get("chapter").is_none(),
            "book.toml leaves pages out of search"
        );
        // Search itself stays on, and the table is the one mdBook reads.
        assert_ne!(search.get("enable"), Some(&toml::Value::Boolean(false)));
        assert!(search.get("heading-split-level").is_some());
    }
}
