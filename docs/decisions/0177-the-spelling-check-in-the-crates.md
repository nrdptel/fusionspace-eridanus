# ADR-177: The spelling check in the crates: words first, em dashes next (2026-10-05)

- **Status:** accepted
- **Summary:** M0.6d2 is re-split in two before it shipped: M0.6d2 widens `cargo xtask spelling` to `crates xtask schema` and the pages' fenced code blocks, read by each block's language, and clears every UK spelling there; M0.6d3 rewrites the 266 em dashes left in them, which the check counts and prints until then. A name in a format string and a key quoted inside a string are identifiers; the check's own file isn't read; probe names an oracle's recording keys by are allowed names.

**Context.** M0.6d2's *done when*: "the same check's scope is d's whole list, adding
`crates xtask schema` and the pages' fenced code blocks, with no findings"
([ADR-176](0176-the-spelling-check.md)). Run on `main`, the widened check as shipped finds 1,446 UK
spellings, which `--fix` replaces, and 266 em dashes (286 before it stopped reading its own file,
§4), each needing its sentence rewritten by hand, too many for one pull request. Running `--fix`
over the crates also showed several kinds of string that hold a word but are data, not writing (§3,
§5).

**Decision.**

1. **Two increments.** M0.6d2, *the crates' words*: the scope is d's whole list and the pages' fenced code blocks,
   with no UK spelling found; an em dash in `crates/`, `xtask/`, `schema/` or a page's code block
   is counted, not reported, and the summary prints the count ("266 em dashes ... left for
   M0.6d3"). M0.6d3, *the crates' em dashes*, takes d2's old *done when* word for word and adds "no em dash left for later", so no criterion is dropped. Ids stop at one increment level, so d2 couldn't take children: a milestone id has one level of increments, `M0.6d2`, and no more.
2. **Code blocks by their language.** A `rust` or `ts` block is read as Rust, `python`, `sh`,
   `bash` or `toml` as `#`-commented code, `json` as JSON, and any other (`text`, a program's
   output, `xml`) as prose, so an identifier in a code block is skipped as it is in the crates.
3. **Identifiers inside strings.** A word right inside braces in a string, `{centre}` or
   `{centre:e}`, is a format string's variable; a word between escaped quotes, `\"colour\"`, is a
   JSON key in a test's document. Both are identifiers, M0.6e's to rename. `--fix` broke the build
   on the first and a test on the second before the rule.
4. **The check's own file isn't read.** `xtask/src/spelling.rs` holds every misspelling on purpose,
   in its word list and its tests' inputs; the summary names it.
5. **More allowed names, each with its reason.** The names of OpenRocket probes that the oracle's
   recording (`validation/fixtures/ork/openrocket-conventions.json`) keys its answers by: renaming
   them means rerunning the oracle, so they stay until a probe changes. A GeoTIFF unit's Rust
   variant `Metre`, which an example prints and a page quotes. A test's copy of the first decision
   record's file name and title, which are frozen.
6. **Text that a crate writes changes with it.** The census page, the OpenRocket flights reports,
   the real flights report and seven aero fixtures' notes are regenerated; only their wording and
   the input hashes that cover it change.

**Consequences.** The crates' comments, doc comments, messages and labels read in US spelling,
and so do the reports they write. The em dashes stay visible as a count on every run until
M0.6d3. The allowed matches grow from 15 to 30, each entry listed with its reason.
