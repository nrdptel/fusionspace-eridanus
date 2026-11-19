//! The voice check's report as a site page (#1495). The report isn't committed, since any two pull
//! requests that edited pages conflicted on it. The site's page,
//! [`voice::PAGE`], is a stub in the repository; the build's preprocessor puts the report written
//! from the pages ([`voice::site_report`]) in its place, so the page is always the pages' own.

use std::path::Path;

use serde_json::Value;
use xtask_voice::voice;

/// Replaces the content of [`voice::PAGE`]'s chapter in mdBook's `book` with the report for the
/// pages under `context`'s root. The chapter must be listed once and must still be the stub
/// ([`voice::STUB_MARKER`]): a page written there by hand is refused, not replaced unseen, and so
/// is a table of contents that drops the page.
pub(super) fn fill(context: &Value, book: &mut Value) -> Result<(), String> {
    let root = context
        .get("root")
        .and_then(Value::as_str)
        .ok_or("mdBook's context names no book root")?;
    let src = context
        .pointer("/config/book/src")
        .and_then(Value::as_str)
        .unwrap_or("src");
    let mut chapters = Vec::new();
    find(book, src, &mut chapters);
    let [content] = chapters.as_mut_slice() else {
        return Err(format!(
            "the site lists {} chapters at {}, not one; the voice report needs its page once",
            chapters.len(),
            voice::PAGE
        ));
    };
    let Value::String(text) = content else {
        return Err(format!("{}'s chapter has no text", voice::PAGE));
    };
    if !text.contains(voice::STUB_MARKER) {
        return Err(format!(
            "{} isn't the stub the build fills (it lacks `{}`); the voice report is written \
             there at build time, so the page holds only its place",
            voice::PAGE,
            voice::STUB_MARKER
        ));
    }
    *text = voice::site_report(Path::new(root))?;
    Ok(())
}

/// Checks the built page: the site's `voice-report.html` under `output` holds the report, not the
/// stub, so a build that skipped [`fill`] fails rather than publish the placeholder.
pub(super) fn check_built(output: &Path) -> Result<(), String> {
    let name = Path::new(voice::PAGE)
        .with_extension("html")
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or("the voice report's page has no file name")?;
    let html = std::fs::read_to_string(output.join(&name))
        .map_err(|err| format!("the built site has no {name}: {err}"))?;
    if html.contains(voice::STUB_MARKER) || !html.contains(BUILT_SIGN) {
        return Err(format!(
            "the built {name} is {}'s stub, not the voice report: the build's preprocessor \
             didn't fill it",
            voice::PAGE
        ));
    }
    Ok(())
}

/// A header cell of the report's first table, which only the filled page has.
const BUILT_SIGN: &str = "Limit per 100 words";

/// Collects the `content` of each chapter whose source, under `src`, is [`voice::PAGE`], whatever
/// the nesting.
fn find<'a>(value: &'a mut Value, src: &str, found: &mut Vec<&'a mut Value>) {
    match value {
        Value::Object(map) => {
            let ours = map
                .get("Chapter")
                .and_then(|chapter| chapter.get("source_path"))
                .and_then(Value::as_str)
                .is_some_and(|path| {
                    format!("{}/{}", src.trim_end_matches('/'), path.replace('\\', "/"))
                        == voice::PAGE
                });
            for (key, child) in map.iter_mut() {
                if ours && key == "Chapter" {
                    if let Value::Object(chapter) = child {
                        for (field, inner) in chapter.iter_mut() {
                            match field.as_str() {
                                "content" => found.push(inner),
                                "sub_items" => find(inner, src, found),
                                _ => {}
                            }
                        }
                    }
                } else {
                    find(child, src, found);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                find(item, src, found);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn chapter(path: &str, content: &str) -> Value {
        json!({"Chapter": {"name": "x", "content": content, "source_path": path, "sub_items": []}})
    }

    fn context() -> Value {
        let root = crate::designs::root().unwrap();
        json!({"root": root.to_str().unwrap(), "config": {"book": {"src": "docs"}}})
    }

    fn stub() -> String {
        format!(
            "# How the pages read\n\nA stub.\n\n{}\n",
            voice::STUB_MARKER
        )
    }

    #[test]
    fn the_stub_gets_the_report_and_other_pages_stay() {
        let mut book = json!({"items": [
            chapter("writing.md", "# Writing\n"),
            {"Chapter": {"name": "w", "content": "# W\n", "source_path": "w.md",
                "sub_items": [chapter("voice-report.md", &stub())]}},
        ]});
        fill(&context(), &mut book).unwrap();
        assert_eq!(book["items"][0]["Chapter"]["content"], "# Writing\n");
        let filled = book["items"][1]["Chapter"]["sub_items"][0]["Chapter"]["content"]
            .as_str()
            .unwrap();
        assert!(filled.starts_with("# How the pages read\n"));
        assert!(!filled.contains(voice::STUB_MARKER));
        assert!(filled.contains("| Pattern | Limit per 100 words |"));
    }

    #[test]
    fn the_built_check_refuses_the_stub_and_passes_the_report() {
        let dir = std::env::temp_dir().join(format!("voice-built-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let page = dir.join("voice-report.html");
        std::fs::write(&page, format!("<main>{}</main>", voice::STUB_MARKER)).unwrap();
        let err = check_built(&dir).unwrap_err();
        assert!(err.contains("stub"), "{err}");
        std::fs::write(&page, "<main><p>A placeholder page.</p></main>").unwrap();
        assert!(check_built(&dir).is_err());
        std::fs::write(&page, format!("<table><th>{BUILT_SIGN}</th></table>")).unwrap();
        assert_eq!(check_built(&dir), Ok(()));
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(
            check_built(&dir)
                .unwrap_err()
                .contains("no voice-report.html")
        );
    }

    #[test]
    fn a_page_without_the_marker_a_missing_page_and_a_page_twice_are_refused() {
        let mut written = json!({"items": [chapter("voice-report.md", "# By hand\n")]});
        let err = fill(&context(), &mut written).unwrap_err();
        assert!(err.contains("isn't the stub"), "{err}");
        assert_eq!(written["items"][0]["Chapter"]["content"], "# By hand\n");
        let mut missing = json!({"items": [chapter("writing.md", &stub())]});
        let err = fill(&context(), &mut missing).unwrap_err();
        assert!(err.contains("0 chapters"), "{err}");
        let mut twice = json!({"items": [
            chapter("voice-report.md", &stub()),
            chapter("voice-report.md", &stub()),
        ]});
        let err = fill(&context(), &mut twice).unwrap_err();
        assert!(err.contains("2 chapters"), "{err}");
    }
}
