# ADR-176: The spelling check: what it reads and what it leaves alone (2026-10-05)

- **Status:** accepted
- **Summary:** M0.6d is split in two: M0.6d1 adds `cargo xtask spelling` and clears the pages, the README and the scripts; M0.6d2 widens its scope to the crates, `xtask`, the schema and the pages' fenced code blocks. The check reads a file's writing (a page's prose, a source file's comments and strings) and skips code identifiers, string literals that are one bare word, link targets and URLs, HTML tags, links quoting a decision record's title, and the frozen records, which now include `docs/DECISIONS.md`. Quotations, source titles and names that keep their spelling sit on a short list in the check, each with the phrase that holds it; the check counts them on every run and fails on an entry that matches nothing.

**Context.** M0.6d's *done when*: "a CI check over `docs README.md crates xtask schema python
scripts` finds no em dash and no form of centre, metre, catalogue, licence, colour, analyse,
behaviour or modelling outside code identifiers, frozen records (`docs/decisions/`,
`docs/roadmap-done.md`), quotations and source titles; it prints how many exceptions it allows"
([ADR-164](0164-the-fusionspace-product-system.md), `docs/writing.md`). On `main` the scope held
about 1,700 such findings in the pages alone (467 em dashes) and about 1,300 more lines in the
crates. The pages' code blocks quote the crates' examples and output word for word, and
`cargo xtask examples` holds them to it, so a block can only change together with its crate.

**Decision.**

1. **Two increments.** M0.6d1 covers `docs README.md python scripts` outside fenced code blocks;
   M0.6d2 adds `crates xtask schema` and the blocks, so M0.6d's *done when* is met only by both.
   No tracked file sits under a top-level `python` yet (the Python package is under
   `crates/hpr-py`, so d2's), and the check's summary says so rather than claim it.
   A crate's comment or string, and a census or report label written by a crate, changes in d2
   with what quotes it. The one exception in d1 is the census's "center of mass at rod
   clearance" label, changed with the README and accuracy tables it writes.
2. **What counts as writing.** A Markdown page outside its fenced blocks, and plain text, SVG
   and snapshot files whole; in Rust, TypeScript and CSS the comments and string literals; in
   Python, shell, TOML and YAML the `#` comments and strings; in JSON the strings. Inside those,
   it skips inline code (a span may cross lines within its paragraph), link targets, reference
   definitions, URLs and HTML tags, and a word that is an identifier: joined by `_`, or holding a
   digit or an inner capital. A string literal that is one bare word is a key, not a word.
   Renaming identifiers and keys is M0.6e's work, which must keep old files readable.
3. **Frozen records.** `docs/decisions/` and `docs/roadmap-done.md`, as the *done when* names
   them, and `docs/DECISIONS.md`, the index `cargo xtask records` writes from the records' own
   titles and summaries. A link whose text opens `ADR-NNN: ` quotes a frozen title and is
   skipped too, but only when its text is the start of that record's own heading; a paraphrase
   is the page's own writing and is checked.
4. **Exceptions by phrase.** Each allowed match is a (file, phrase, reason) entry, the reason a
   quotation, a source title or a name: ECMWF's name (three places), *Environmental Modelling &
   Software*, a quoted page heading, the design name in the guide's plot, and the word list in
   M0.6d's own *done when*. Every run prints the count by reason; an entry that no longer
   matches fails the check, so the list can't outlive its text.
5. **"Analyses" stays.** It is also the plural of "analysis", the same in US English; the check
   can't tell the verb from the noun, so it reports neither.
6. **Old anchors.** Headings renamed by the spelling change move their anchors; every link in the
   pages follows, and an `<a id>` keeps the one old anchor a crate's rustdoc links to (`aero.md`'s
   *Your rocket's center of pressure*) until d2 updates that link.
7. **Where it runs.** `cargo xtask site` runs it first, so CI's `site` step prints the count, and
   `cargo test -p xtask` fails on any finding.

**Consequences.** The pages, the README and the scripts read in US English without em dashes,
and a new page can't regress without a red check. Placeholders that were em dashes in tables
read "n/a", "none" or "no"; the process scripts print "n/a" too. Until M0.6d2 the crates' prose
and the pages' code blocks still hold UK spellings, and so does any page text a crate writes.
