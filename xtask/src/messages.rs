//! The census of what the design commands say (M0.14d2; ADR-292 §5 and its amendment for
//! M0.14d2): the errors, warnings and notes `hpr sim`, `hpr mc`, `hpr convert` and `hpr motors
//! list`/`show` print, read from the source of `hpr-cli` and the nine crates under them, so that
//! [`crate::voice`] can hold them as it holds a command's help.
//!
//! The source doesn't say which literal ends up in a message: a refusal's `what` is written where
//! the error is made, a warning's text is often built in one function and pushed in another. So
//! the census reads more than the messages. That holds a stock phrase or first person anywhere
//! in the code's prose, though the extra prose also dilutes the rates:
//!
//! - **The files** ([`CRATES`]): each tracked `.rs` file under a crate's `src/`, less test code (an
//!   item under `#[cfg(test)]`, and a module declared under it with the modules inside it) and
//!   less [`LEFT_OUT`], the files of M0.14d3's commands.
//! - **What it reads**: each `#[error("…")]` text, and each other string literal in the code that
//!   reads as prose ([`is_prose`]), less markup ([`is_markup`]). Not comments, doc comments, byte
//!   strings or other attributes
//!   (the help's, which its snapshots hold, and a lint's `reason`, read only by the code's
//!   readers). An `#[error(transparent)]`, or an `#[error]` text with no word in it (`"{0}"`),
//!   passes on another error's text, which is read where it's written; those are counted as
//!   forwarded.
//! - **How**: each line of a literal is a line of the crate's census ([`Message::lines`]), a
//!   placeholder (`{path}`, `{0:.1}`) and text in backticks each one code word ([`CODE`]). The
//!   pieces a message joins at run time are read one by one, so [`Census::with_placeholder`]
//!   counts the literals the census can't see whole.

use std::fs;
use std::path::Path;

use crate::voice::CODE;

/// The crates read, each by its directory under `crates/`: `hpr-cli`, whose dispatch, output and
/// option readers M0.14d2 takes with its own commands, and the nine ADR-292 §1 lists under them.
pub(crate) const CRATES: [&str; 10] = [
    "hpr-cli",
    "hpr",
    "hpr-sim",
    "hpr-io",
    "hpr-motor",
    "hpr-design",
    "hpr-aero",
    "hpr-core",
    "hpr-format",
    "hpr-atmos",
];

/// Files of a crate in [`CRATES`] left to M0.14d3, which reads the data commands, each with the
/// reason.
const LEFT_OUT: [(&str, &str); 5] = [
    ("crates/hpr-cli/src/analyze.rs", "`hpr analyze` (M0.14d3)"),
    ("crates/hpr-cli/src/weather.rs", "`hpr weather` (M0.14d3)"),
    (
        "crates/hpr-cli/src/motor_fetch.rs",
        "`hpr motors fetch` (M0.14d3)",
    ),
    (
        "crates/hpr-cli/src/motor_search.rs",
        "`hpr motors search` (M0.14d3)",
    ),
    (
        "crates/hpr-cli/src/waiting.rs",
        "what a fetch prints while it waits (M0.14d3)",
    ),
];

/// One literal read: where it is, and its text as the census reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Message {
    /// The file, relative to the workspace root.
    pub file: String,
    /// The line, from 1, the literal starts on.
    pub line: usize,
    /// Whether it is an `#[error]` text.
    pub error: bool,
    /// Its lines with a word of letters in them, unescaped, each placeholder and each span in
    /// backticks read as ` CODE `.
    pub lines: Vec<String>,
}

/// What the census read in one crate.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Census {
    /// The crate's source directory, `crates/hpr-io/src/`: the page it's held as.
    pub dir: String,
    /// The files read.
    pub files: usize,
    /// The literals read, in file order.
    pub messages: Vec<Message>,
    /// `#[error]` texts that pass on another error's text: `transparent`, or no word in it.
    pub forwarded: usize,
    /// Literals read that hold a placeholder: messages the census can't see whole.
    pub with_placeholder: usize,
}

impl Census {
    /// How many of [`Census::messages`] are `#[error]` texts.
    pub(crate) fn errors(&self) -> usize {
        self.messages.iter().filter(|m| m.error).count()
    }
}

/// The census of every crate in [`CRATES`], from the tracked files under `root`.
pub(crate) fn census(root: &Path) -> Result<Vec<Census>, String> {
    let dirs: Vec<String> = CRATES.iter().map(|c| format!("crates/{c}/src/")).collect();
    let refs: Vec<&str> = dirs.iter().map(String::as_str).collect();
    let tracked = crate::voice::tracked(root, &refs)?;
    left_out_are_tracked(&tracked)?;
    let mut out = Vec::new();
    for dir in dirs {
        let mut files: Vec<(String, String)> = Vec::new();
        for path in tracked
            .iter()
            .filter(|p| p.starts_with(&dir) && p.ends_with(".rs"))
        {
            let text = fs::read_to_string(root.join(path)).map_err(|e| format!("{path}: {e}"))?;
            files.push((path.clone(), text));
        }
        let census = read_crate(&dir, &files);
        if census.files == 0 {
            return Err(format!(
                "the message census reads no file under {dir}, so it would hold nothing there"
            ));
        }
        out.push(census);
    }
    Ok(out)
}

/// Each file [`LEFT_OUT`] names is among the `tracked` ones, so a renamed file can't slip into
/// the census, nor a left-out entry outlive its file.
fn left_out_are_tracked(tracked: &[String]) -> Result<(), String> {
    for (path, _) in LEFT_OUT {
        if !tracked.iter().any(|file| file == path) {
            return Err(format!(
                "the message census leaves out {path}, which isn't tracked: update LEFT_OUT in \
                 xtask/src/messages.rs"
            ));
        }
    }
    Ok(())
}

/// The census of one crate's files, `(path, text)`: each file's literals, less [`LEFT_OUT`] and
/// the modules a file declares under `#[cfg(test)]`.
fn read_crate(dir: &str, files: &[(String, String)]) -> Census {
    let mut census = Census {
        dir: dir.to_owned(),
        ..Census::default()
    };
    let mut scanned: Vec<(String, Scan)> = files
        .iter()
        .map(|(path, text)| (path.clone(), scan(text)))
        .collect();
    scanned.sort_by(|a, b| a.0.cmp(&b.0));
    // Each file a test module covers: the module's own file and every file under its directory.
    let mut test_dirs: Vec<String> = Vec::new();
    for (path, scan) in &scanned {
        for module in &scan.test_modules {
            let base = module_dir(path);
            test_dirs.push(format!("{base}{module}.rs"));
            test_dirs.push(format!("{base}{module}/"));
        }
    }
    let is_test = |path: &str| {
        test_dirs.iter().any(|entry| {
            if entry.ends_with('/') {
                path.starts_with(entry.as_str())
            } else {
                path == entry
            }
        })
    };
    for (path, scan) in scanned {
        if LEFT_OUT.iter().any(|(left, _)| *left == path) || is_test(&path) || scan.whole_test {
            continue;
        }
        census.files += 1;
        census.forwarded += scan.forwarded;
        for literal in scan.literals {
            let lines = reading_lines(&literal.text);
            if is_markup(&literal.text)
                || !(literal.error && has_letter_word(&lines) || is_prose(&lines))
            {
                continue;
            }
            if has_placeholder(&literal.text) {
                census.with_placeholder += 1;
            }
            census.messages.push(Message {
                file: path.clone(),
                line: literal.line,
                error: literal.error,
                lines: lines
                    .into_iter()
                    .filter(|line| has_letter_word(std::slice::from_ref(line)))
                    .collect(),
            });
        }
    }
    census
}

/// The directory a file's child modules sit in: its own for `lib.rs`, `main.rs` and `mod.rs`,
/// else one named by its stem (`ork/attached.rs` → `ork/attached/`).
fn module_dir(path: &str) -> String {
    let (parent, name) = path.rsplit_once('/').unwrap_or(("", path));
    let parent = if parent.is_empty() {
        String::new()
    } else {
        format!("{parent}/")
    };
    match name {
        "lib.rs" | "main.rs" | "mod.rs" => parent,
        _ => format!("{parent}{}/", name.trim_end_matches(".rs")),
    }
}

/// Whether a literal is markup, a piece of an SVG figure or a file's XML: it starts with a tag
/// and holds a closing tag, a tag closed on itself, an attribute, a declaration or a comment.
/// A message that only opens by naming an element (`<Shape> is ...`, the ORC reader's notes)
/// is prose and is read. The text a figure shows is mostly written in a literal of its own;
/// the few short labels written inside a tag (`not a prediction`) go with the markup.
fn is_markup(text: &str) -> bool {
    let text = text.trim_start();
    text.starts_with('<')
        && (text.starts_with("<?")
            || text.starts_with("<!")
            || text.contains("</")
            || text.contains("/>")
            || text.contains("=\""))
}

/// Whether a literal's lines read as prose: three words or more, a code word counting as one,
/// and at least one of them a word of letters.
fn is_prose(lines: &[String]) -> bool {
    let words: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .filter(|token| token.chars().any(|c| c.is_alphanumeric() || c == CODE))
        .collect();
    words.len() >= 3 && has_letter_word(lines)
}

/// Whether any of `lines` holds a word with a letter in it.
fn has_letter_word(lines: &[String]) -> bool {
    lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .any(|token| token.chars().any(char::is_alphabetic))
}

/// Whether a literal holds a format placeholder: `{…}` that isn't an escaped `{{`.
fn has_placeholder(text: &str) -> bool {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' {
            if chars.peek() == Some(&'{') {
                chars.next();
            } else if chars.clone().any(|c| c == '}') {
                return true;
            }
        }
    }
    false
}

/// A literal's lines as the census reads them: split at each line break, each placeholder and
/// each span in backticks read as one code word, `{{` and `}}` as braces.
fn reading_lines(text: &str) -> Vec<String> {
    text.split('\n')
        .map(|line| {
            let mut out = String::new();
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '{' if chars.peek() == Some(&'{') => {
                        chars.next();
                        out.push('{');
                    }
                    '}' if chars.peek() == Some(&'}') => {
                        chars.next();
                        out.push('}');
                    }
                    '{' | '`' => {
                        let close = if c == '{' { '}' } else { '`' };
                        let rest: String = chars.clone().collect();
                        if let Some(len) = rest.find(close) {
                            for _ in rest[..len].chars() {
                                chars.next();
                            }
                            chars.next();
                            out.push(' ');
                            out.push(CODE);
                            out.push(' ');
                        } else {
                            out.push(c);
                        }
                    }
                    _ => out.push(c),
                }
            }
            out.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect()
}

/// A string literal found in a file.
#[derive(Debug, PartialEq, Eq)]
struct Literal {
    line: usize,
    error: bool,
    text: String,
}

/// What [`scan`] found in a file.
#[derive(Debug, Default, PartialEq, Eq)]
struct Scan {
    literals: Vec<Literal>,
    /// The modules the file declares under `#[cfg(test)]` with `mod name;`.
    test_modules: Vec<String>,
    /// `#![cfg(test)]` at the top: the whole file is test code.
    whole_test: bool,
    forwarded: usize,
}

/// A token of Rust source, as far as the census needs one.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    /// A string literal, unescaped (a raw one as written), and the line it starts on.
    Str(String, usize),
    /// An identifier, keyword or number.
    Word(String),
    /// One character of punctuation.
    Punct(char),
    /// A byte string, a C string or a char literal: never a message.
    Other,
}

/// The tokens of `text`: comments (doc comments too) dropped, lifetimes and labels dropped.
fn lex(text: &str) -> Vec<Token> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    let starts: Vec<usize> = text.match_indices('\n').map(|(at, _)| at).collect();
    let line_at = |at: usize| starts.partition_point(|&start| start < at) + 1;
    while i < bytes.len() {
        let c = bytes[i];
        let rest = &text[i..];
        let after_word = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if rest.starts_with("//") {
            i = rest.find('\n').map_or(bytes.len(), |n| i + n);
        } else if rest.starts_with("/*") {
            // Block comments nest in Rust.
            let mut depth = 0;
            let mut j = i;
            while j < bytes.len() {
                // Bytes, not `str` slices: a comment's text can hold characters of more than one
                // byte, and `j` steps through them a byte at a time.
                if bytes[j..].starts_with(b"/*") {
                    depth += 1;
                    j += 2;
                } else if bytes[j..].starts_with(b"*/") {
                    depth -= 1;
                    j += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            i = j;
        } else if let Some((open, hashes, prefixed)) = raw_open(text, i).filter(|_| !after_word) {
            let close = format!("\"{}", "#".repeat(hashes));
            let start = i + open;
            let end = text[start..].find(&close).map_or(text.len(), |n| start + n);
            tokens.push(if prefixed {
                Token::Other
            } else {
                Token::Str(text[start..end].to_owned(), line_at(i))
            });
            i = (end + close.len()).min(bytes.len());
        } else if c == b'"'
            || (matches!(c, b'b' | b'c') && bytes.get(i + 1) == Some(&b'"') && !after_word)
        {
            let prefixed = c != b'"';
            let start = i + if prefixed { 2 } else { 1 };
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = j.min(bytes.len());
            tokens.push(if prefixed {
                Token::Other
            } else {
                Token::Str(unescape(&text[start..end]), line_at(i))
            });
            i = end + 1;
        } else if c == b'\'' {
            match char_literal_len(text, i) {
                Some(len) => {
                    tokens.push(Token::Other);
                    i += len;
                }
                None => {
                    // A lifetime or a label: the quote and the name after it.
                    i += 1;
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
                    {
                        i += 1;
                    }
                }
            }
        } else if c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80 {
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] >= 0x80)
            {
                i += 1;
            }
            tokens.push(Token::Word(text[start..i].to_owned()));
        } else if c.is_ascii_whitespace() {
            i += 1;
        } else {
            tokens.push(Token::Punct(char::from(c)));
            i += 1;
        }
    }
    tokens
}

/// At a raw string (`r"`, `r#"`, `br"`, `cr#"`), the length of its opening, its count of `#`,
/// and whether a `b` or `c` prefixes it.
fn raw_open(text: &str, i: usize) -> Option<(usize, usize, bool)> {
    let bytes = text.as_bytes();
    let mut j = i;
    let prefixed = matches!(bytes.get(j), Some(b'b' | b'c'));
    if prefixed {
        j += 1;
    }
    if bytes.get(j) != Some(&b'r') {
        return None;
    }
    j += 1;
    let hashes = bytes[j..].iter().take_while(|&&b| b == b'#').count();
    (bytes.get(j + hashes) == Some(&b'"')).then_some((j + hashes + 1 - i, hashes, prefixed))
}

/// At a `'`, the length of the char literal it opens (`'a'`, `'\n'`, `'\u{2019}'`), or `None` for
/// a lifetime or a label.
fn char_literal_len(text: &str, i: usize) -> Option<usize> {
    let rest = &text[i + 1..];
    let mut chars = rest.char_indices();
    let (_, first) = chars.next()?;
    if first == '\\' {
        // An escape runs to the next quote: `'\''` is the one that holds a quote.
        let body = &rest[1..];
        let close = if body.starts_with('\'') {
            1
        } else {
            body.find('\'')?
        };
        return Some(1 + 1 + close + 1);
    }
    let width = first.len_utf8();
    rest[width..].starts_with('\'').then_some(1 + width + 1)
}

/// A string literal's text with its escapes read: `\n`, `\t`, `\\`, `\"`, `\'`, `\0`, `\x41`,
/// `\u{…}`, and a backslash before a line break, which drops the break and the next line's
/// leading spaces.
fn unescape(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => {}
            Some('0') => out.push('\0'),
            Some('\n') => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                if let Some(c) = u8::from_str_radix(&hex, 16).ok().map(char::from) {
                    out.push(c);
                }
            }
            Some('u') => {
                let code: String = chars
                    .by_ref()
                    .skip_while(|c| *c == '{')
                    .take_while(|c| *c != '}')
                    .collect();
                if let Some(c) = u32::from_str_radix(&code, 16).ok().and_then(char::from_u32) {
                    out.push(c);
                }
            }
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

/// A file's literals and its test modules, read from its tokens: attributes are read for
/// `#[error]` and `#[cfg(test)]` and otherwise skipped, and an item under `#[cfg(test)]` is
/// skipped whole.
fn scan(text: &str) -> Scan {
    let tokens = lex(text);
    let mut found = Scan::default();
    let mut i = 0;
    let mut test_item = false;
    // Whether only inner attributes have come so far: a `#![cfg(test)]` there is the file's own.
    let mut at_head = true;
    while i < tokens.len() {
        if tokens[i] == Token::Punct('#') {
            let inner = tokens.get(i + 1) == Some(&Token::Punct('!'));
            let open = i + 1 + usize::from(inner);
            if tokens.get(open) == Some(&Token::Punct('[')) {
                let close = matching(&tokens, open);
                let body = &tokens[open + 1..close.min(tokens.len())];
                at_head &= inner;
                if is_cfg_test(body) {
                    if inner && at_head {
                        found.whole_test = true;
                        return found;
                    }
                    if inner {
                        // A module's own `#![cfg(test)]`, inside its braces: the rest of the
                        // module is test code.
                        i = block_end(&tokens, close + 1);
                        continue;
                    }
                    test_item = true;
                } else if let Some(text) = error_text(body) {
                    match text {
                        Some((text, line)) if !text.trim().is_empty() => {
                            found.literals.push(Literal {
                                line,
                                error: true,
                                text,
                            });
                            if !text_has_word(&found.literals[found.literals.len() - 1].text) {
                                found.forwarded += 1;
                            }
                        }
                        _ => found.forwarded += 1,
                    }
                }
                i = close + 1;
                continue;
            }
        }
        at_head = false;
        if test_item {
            if let Some(module) = declared_module(&tokens[i..]) {
                found.test_modules.push(module);
            }
            i = item_end(&tokens, i);
            test_item = false;
            continue;
        }
        if let Token::Str(text, line) = &tokens[i] {
            found.literals.push(Literal {
                line: *line,
                error: false,
                text: text.clone(),
            });
        }
        i += 1;
    }
    found
}

/// Whether an `#[error]` text has a word of letters outside its placeholders.
fn text_has_word(text: &str) -> bool {
    has_letter_word(
        &reading_lines(text)
            .into_iter()
            .map(|l| l.replace(CODE, ""))
            .collect::<Vec<_>>(),
    )
}

/// Whether an attribute's body is exactly `cfg(test)`.
fn is_cfg_test(body: &[Token]) -> bool {
    body == [
        Token::Word("cfg".to_owned()),
        Token::Punct('('),
        Token::Word("test".to_owned()),
        Token::Punct(')'),
    ]
}

/// For an `error(…)` attribute's body, its text and line, or `Some(None)` for
/// `error(transparent)`; `None` for any other attribute.
fn error_text(body: &[Token]) -> Option<Option<(String, usize)>> {
    if body.first() != Some(&Token::Word("error".to_owned()))
        || body.get(1) != Some(&Token::Punct('('))
    {
        return None;
    }
    Some(match body.get(2) {
        Some(Token::Str(text, line)) => Some((text.clone(), *line)),
        _ => None,
    })
}

/// The name in `mod name;` at the start of an item (after `pub`, `pub(crate)` and the like).
fn declared_module(tokens: &[Token]) -> Option<String> {
    let mut k = 0;
    if tokens.first() == Some(&Token::Word("pub".to_owned())) {
        k = 1;
        if tokens.get(1) == Some(&Token::Punct('(')) {
            k = matching(tokens, 1) + 1;
        }
    }
    match (tokens.get(k), tokens.get(k + 1), tokens.get(k + 2)) {
        (Some(Token::Word(m)), Some(Token::Word(name)), Some(Token::Punct(';'))) if m == "mod" => {
            Some(name.clone())
        }
        _ => None,
    }
}

/// The index of the bracket that closes the one at `open`, or the end.
fn matching(tokens: &[Token], open: usize) -> usize {
    let mut depth = 0usize;
    for (k, token) in tokens.iter().enumerate().skip(open) {
        match token {
            Token::Punct('(' | '[' | '{') => depth += 1,
            Token::Punct(')' | ']' | '}') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return k;
                }
            }
            _ => {}
        }
    }
    tokens.len()
}

/// The words that start an item or a `let` statement, after any `pub` or `pub(...)`.
const ITEM_WORDS: [&str; 15] = [
    "fn", "mod", "impl", "struct", "enum", "union", "const", "static", "use", "type", "trait",
    "extern", "unsafe", "async", "let",
];

/// Where the code under an outer `#[cfg(test)]` starting at `start` ends, by one rule: at the
/// first `;` outside any bracket; after a `}` that closes back to its own level, unless the token
/// after it goes on with the same code ([`goes_on`]: `A { x } =>`, `{ .. } else`, `S { .. }.f()`),
/// with a `,` right after it taken too; at a `,` outside any bracket, except in an item or a `let`,
/// whose `<A, B>` this doesn't count as brackets; or before the bracket that closes the list it's
/// in. So an item ends after its body or its `;`, and a variant, a field or a match arm (its
/// pattern's braces and its block's) at its comma or its block's end.
fn item_end(tokens: &[Token], start: usize) -> usize {
    let mut first = start;
    if tokens.get(first) == Some(&Token::Word("pub".to_owned())) {
        first += 1;
        if tokens.get(first) == Some(&Token::Punct('(')) {
            first = matching(tokens, first) + 1;
        }
    }
    let item = matches!(tokens.get(first), Some(Token::Word(word)) if ITEM_WORDS.contains(&word.as_str())
        || word == "macro_rules");
    let mut depth = 0usize;
    for (k, token) in tokens.iter().enumerate().skip(start) {
        match token {
            Token::Punct('(' | '[' | '{') => depth += 1,
            Token::Punct(')' | ']' | '}') if depth == 0 => return k,
            Token::Punct(')' | ']') => depth -= 1,
            Token::Punct('}') => {
                depth -= 1;
                if depth == 0 && !goes_on(tokens.get(k + 1)) {
                    return if tokens.get(k + 1) == Some(&Token::Punct(',')) {
                        k + 2
                    } else {
                        k + 1
                    };
                }
            }
            Token::Punct(';') if depth == 0 => return k + 1,
            Token::Punct(',') if depth == 0 && !item => return k + 1,
            _ => {}
        }
    }
    tokens.len()
}

/// Whether the token after a closing brace goes on with the same code: a method or `?`, an
/// `else` or `as`, or a binary operator (a pattern's `=>`, an or-pattern's `|`, `+` or `==`).
/// A `-`, `&` or `!` is left out: each can also start the next arm or statement, and ending there
/// reads more of the code, not less.
fn goes_on(next: Option<&Token>) -> bool {
    match next {
        Some(Token::Punct(c)) => matches!(
            c,
            '.' | '?' | '=' | '|' | '+' | '*' | '/' | '%' | '^' | '<' | '>'
        ),
        Some(Token::Word(word)) => word == "else" || word == "as",
        _ => false,
    }
}

/// Where the block holding `start` ends: after the bracket that closes it.
fn block_end(tokens: &[Token], start: usize) -> usize {
    let mut depth = 0usize;
    for (k, token) in tokens.iter().enumerate().skip(start) {
        match token {
            Token::Punct('(' | '[' | '{') => depth += 1,
            Token::Punct(')' | ']' | '}') if depth == 0 => return k + 1,
            Token::Punct(')' | ']' | '}') => depth -= 1,
            _ => {}
        }
    }
    tokens.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(source: &str) -> Vec<(bool, String)> {
        scan(source)
            .literals
            .into_iter()
            .map(|l| (l.error, l.text))
            .collect()
    }

    #[test]
    fn reads_error_texts_and_literals_but_not_comments_or_attributes() {
        let source = r##"
//! A crate doc "not read".
/// A doc "not read".
#[derive(Debug, thiserror::Error)]
pub enum E {
    #[error("the file {path} can't be read")]
    Read { path: String },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}
#[expect(clippy::too_many_lines, reason = "a reason is not read")]
fn f() {
    // "a comment"
    /* "a block /* nested */ comment" */
    let a = "one two three";
    let b = r#"a raw "quoted" one"#;
    let c = b"bytes are not read";
    let d = 'x';
    let e: &'static str = "after a lifetime";
}
"##;
        assert_eq!(
            texts(source),
            vec![
                (true, "the file {path} can't be read".to_owned()),
                (true, "{0}".to_owned()),
                (false, "one two three".to_owned()),
                (false, "a raw \"quoted\" one".to_owned()),
                (false, "after a lifetime".to_owned()),
            ]
        );
        // `transparent` and the text with no word in it are forwarded.
        assert_eq!(scan(source).forwarded, 2);
    }

    #[test]
    fn skips_test_items_and_test_modules() {
        let source = r#"
fn kept() { let _ = "read in the census"; }
#[cfg(test)]
mod tests {
    #[test]
    fn t() { assert_eq!(1, 1, "not read at all"); }
}
#[cfg(test)]
pub(crate) mod testing;
#[cfg(test)]
fn helper() -> Result<(), String> { Err("not read either".into()) }
#[cfg(not(test))]
fn release() { let _ = "read as it is not a test"; }
const LAST: &str = "read after the skipped items";
"#;
        let scanned = scan(source);
        let read: Vec<&str> = scanned.literals.iter().map(|l| l.text.as_str()).collect();
        assert_eq!(
            read,
            [
                "read in the census",
                "read as it is not a test",
                "read after the skipped items"
            ]
        );
        assert_eq!(scanned.test_modules, ["testing"]);
        assert!(scan("#![cfg(test)]\nfn f() { let _ = \"x y z\"; }").whole_test);
        assert!(scan("#![allow(dead_code)]\n#![cfg(test)]\nfn f() {}").whole_test);
    }

    /// A `#[cfg(test)]` on a variant, a field or a match arm skips that one, not the list it's
    /// in, and a module's own `#![cfg(test)]` skips that module, not the rest of the file.
    #[test]
    fn a_test_variant_field_or_module_skips_only_itself() {
        let read = |source: &str| -> Vec<String> {
            let scanned = scan(source);
            assert!(!scanned.whole_test, "{source}");
            scanned.literals.into_iter().map(|l| l.text).collect()
        };
        assert_eq!(
            read("enum E { #[cfg(test)] A, #[error(\"real one here\")] B, }"),
            ["real one here"]
        );
        assert_eq!(
            read(
                "struct S { #[cfg(test)] pub a: HashMap<u8, u8>, b: u8 } \
                 const C: &str = \"after the struct\";"
            ),
            ["after the struct"]
        );
        assert_eq!(
            read(
                "fn f(x: u8) -> &'static str { match x { #[cfg(test)] 0 => \"a test arm\", \
                 _ => \"the other arm\" } }"
            ),
            ["the other arm"]
        );
        assert_eq!(
            read(
                "fn f(x: u8) -> &'static str { match x { #[cfg(test)] 0 => { \"a test block\" } \
                 _ => \"the arm after a block\", } }"
            ),
            ["the arm after a block"]
        );
        assert_eq!(
            read(
                "fn f() { #[cfg(test)] { let _ = \"a test block\"; } let _ = \"after the block\"; } \
                 #[cfg(test)] thread_local! { static A: u8 = 1; } const C: &str = \"after a macro\";"
            ),
            ["after the block", "after a macro"]
        );
        assert_eq!(
            read("enum E { #[cfg(test)] A { x: u8 }, #[error(\"the next variant\")] B }"),
            ["the next variant"]
        );
        assert_eq!(
            read(
                "fn f() { match c { #[cfg(test)] A { x } => \"a test arm\", _ => \"the live arm\" } \
                 match c { #[cfg(test)] A { x } | B { x } => { \"a test block\" } \
                 _ => \"the next live arm\" } }"
            ),
            ["the live arm", "the next live arm"]
        );
        assert_eq!(
            read(
                "fn f() { #[cfg(test)] let x = if c { \"one test word\" } else { \"two test words\" }; \
                 #[cfg(test)] x = S { a } + \"more test words\"; let _ = \"the live words\"; }"
            ),
            ["the live words"]
        );
        assert_eq!(
            read(
                "fn a() { let _ = \"first words here\"; } mod t { #![cfg(test)] \
                 fn x() { let _ = \"test words here\"; } } fn b() { let _ = \"later words here\"; }"
            ),
            ["first words here", "later words here"]
        );
    }

    /// A block comment's text can hold characters of more than one byte.
    #[test]
    fn a_block_comment_may_hold_any_character() {
        let scanned = scan("/* a → b, /* ° */ ü */ fn f() { let _ = \"x y z\"; }");
        let read: Vec<&str> = scanned.literals.iter().map(|l| l.text.as_str()).collect();
        assert_eq!(read, ["x y z"]);
    }

    #[test]
    fn a_test_module_covers_its_file_and_its_children() {
        let files = vec![
            (
                "crates/x/src/lib.rs".to_owned(),
                "#[cfg(test)]\nmod tests;\nmod ork;\nfn f() { let _ = \"read in lib\"; }"
                    .to_owned(),
            ),
            (
                "crates/x/src/tests.rs".to_owned(),
                "fn t() { let _ = \"a test module's words\"; }".to_owned(),
            ),
            (
                "crates/x/src/ork.rs".to_owned(),
                "#[cfg(test)]\nmod tests;\nfn g() { let _ = \"read in ork\"; }".to_owned(),
            ),
            (
                "crates/x/src/ork/tests.rs".to_owned(),
                "fn t() { let _ = \"also a test's words\"; }".to_owned(),
            ),
            (
                "crates/x/src/ork/tests/deeper.rs".to_owned(),
                "fn t() { let _ = \"under a test module\"; }".to_owned(),
            ),
        ];
        let census = read_crate("crates/x/src/", &files);
        let read: Vec<(&str, usize)> = census
            .messages
            .iter()
            .map(|m| (m.file.as_str(), m.line))
            .collect();
        assert_eq!(
            read,
            [("crates/x/src/lib.rs", 4), ("crates/x/src/ork.rs", 3)]
        );
        assert_eq!(census.files, 2);
    }

    #[test]
    fn placeholders_and_backticks_are_code_words() {
        let lines = reading_lines("give `--runs` {n:.1} or {{none}}\nsecond line");
        assert_eq!(
            lines,
            [
                format!("give {CODE} {CODE} or {{none}}"),
                "second line".to_owned()
            ]
        );
        assert!(has_placeholder("a {path} b"));
        assert!(!has_placeholder("a {{literal}} b"));
    }

    #[test]
    fn prose_needs_three_words_and_one_of_letters() {
        let prose = |text: &str| is_prose(&reading_lines(text));
        assert!(prose("no such file"));
        assert!(prose("{path}: no such"));
        assert!(!prose("two words"));
        assert!(!prose("{a} {b} {c}"));
        assert!(!prose("1 2 3"));
        assert!(!prose("snake_case_key"));
    }

    #[test]
    fn markup_is_not_read() {
        let files = vec![(
            "crates/x/src/lib.rs".to_owned(),
            "fn f() { let _ = format!(\"<text x=\\\"{x}\\\" y=\\\"{y}\\\" fill=\\\"ink\\\">{label}</text>\"); \
             let _ = \"the caption a figure shows\"; }"
                .to_owned(),
        )];
        let census = read_crate("crates/x/src/", &files);
        let read: Vec<&str> = census
            .messages
            .iter()
            .map(|m| m.lines[0].as_str())
            .collect();
        assert_eq!(read, ["the caption a figure shows"]);
        assert!(is_markup("  <g id=\"event\">"));
        assert!(!is_markup("give a value, not <FILE>"));
        assert!(is_markup("\n</coordinates>\n"));
        assert!(is_markup("<?xml version=\"1.0\"?>"));
        assert!(is_markup("<!-- {} -->\n<engine-database>"));
        assert!(is_markup("<name>{}</name>"));
        assert!(is_markup("<br/>"));
        assert!(!is_markup("<Shape> is `{}`, not one of CONICAL or OGIVE"));
        assert!(!is_markup(
            "<{name}> names a material of the wrong kind for the part"
        ));
    }

    #[test]
    fn escapes_and_continuations_are_read() {
        assert_eq!(unescape(r#"a\nb\t\\\"\'"#), "a\nb\t\\\"'");
        assert_eq!(unescape("one \\\n        two"), "one two");
        assert_eq!(unescape(r"\u{2019}\x41"), "\u{2019}A");
    }

    #[test]
    fn a_left_out_file_is_not_read_and_must_exist() {
        let files = vec![(
            "crates/hpr-cli/src/weather.rs".to_owned(),
            "fn f() { let _ = \"the weather's words wait\"; }".to_owned(),
        )];
        assert!(
            read_crate("crates/hpr-cli/src/", &files)
                .messages
                .is_empty()
        );
        let mut tracked: Vec<String> = LEFT_OUT
            .iter()
            .map(|(path, _)| (*path).to_owned())
            .collect();
        assert_eq!(left_out_are_tracked(&tracked), Ok(()));
        tracked.remove(2);
        let error = left_out_are_tracked(&tracked).unwrap_err();
        assert!(error.contains(LEFT_OUT[2].0), "{error}");
    }
}
