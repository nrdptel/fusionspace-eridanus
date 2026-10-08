# ADR-206: The name check in two steps: the packages' text, then the site (2026-10-08)

- **Status:** accepted; splits [ADR-205, one name and one design][adr-205] §1's M0.9a in two and narrows its §3
- **Summary:** M0.9a is split. M0.9a1 builds `cargo xtask names` with both of ADR-205's rules, its allowlist and its tests, and holds every surface ADR-205 §1 lists to it except the site's pages: the text that ships inside a published package (the README, the crates' descriptions, READMEs, rustdoc and messages, the Python package, the schemas and their bindings, the licenses, notices and CHANGELOG) and the banners, plus the landing page, whose first paragraph now names the suite. M0.9a2 adds the pages `SUMMARY.md` renders and the records page, about 2,300 more mentions. Release 0.1 waits for M0.9a1 only, since a published version's package text is fixed and the site's is not. Code compiled only for tests ships nothing and is not checked; a string literal's `hpr` is the command when a subcommand, a flag or a placeholder follows it.

[adr-199]: 0199-the-2026-10-07-fusionspace-hpr.md
[adr-205]: 0205-the-2026-10-08-one-name-one-design.md

**Context.** Measured on `main` at `aeae382` with the finished check, the surfaces ADR-205 §1 lists
outside the site's pages held 644 mentions in 111 files that its allowlist doesn't cover (52 of
the old name, 592 a bare lowercase name in prose), and the landing page's first paragraph named the
product but not the suite. The 62 pages `SUMMARY.md` renders held about 2,300 bare mentions more,
430 of them on the `.ork` format page and 400 on the aerodynamics page. Each rewrite is small, but each is a sentence a reader sees, and a
review can read a few hundred, not three thousand, in one pull request. The two halves differ in
one way that matters: crates.io and PyPI keep the text a version was published with, while the
site is rebuilt from `main` on every merge.

**Decision.**

1. **M0.9a1, the packages' text.** `cargo xtask names` (run by `cargo test -p xtask`) applies both
   of ADR-205 §1's rules, with its allowlist and its three tests, to: the README and the landing
   page; every crate's `Cargo.toml` description and README; the rustdoc and string literals under
   `src/` of each published crate and of the Python bindings, whose rustdoc is the Python
   package's docstrings; the Python package's docstrings, comments and `pyproject.toml`; the
   schemas' descriptions and titles, and the Python and TypeScript bindings generated from them;
   every `LICENSE-MIT` and `LICENSE-APACHE`, the release archives' license template,
   `THIRD-PARTY-NOTICES.md` and `CHANGELOG.md`; and the banners' `project.json`. The landing
   page's test reads its first paragraph, which must name each product in `SHIPPED` and the suite
   on its own (not only as the start of a product's name).
2. **M0.9a2, the site's pages,** adds the pages `SUMMARY.md` renders and the records page, whose
   rows inherit only the allowance of the ADR their `[adr-NNN]` link names. Its *done when* is
   M0.9a's.
3. **Release 0.1 waits for M0.9a1,** which narrows ADR-205 §3: what M0.9a2 changes is the site,
   which can change after publishing.
4. **What the check reads as prose, and what it leaves out** (chosen):
   - code formatting is Markdown's: code spans and blocks in Markdown, rustdoc, docstrings,
     comments and descriptions; reStructuredText's double backticks read as code spans too;
   - a word joined to the name is another name: `hpr-core`, `hpr_sim`, `.hpr`,
     `hpr.fusionspace.co`, `@fusionspace/hpr`, `hpr::sim`; so is a package's name since
     [ADR-199][adr-199], such as `fusionspace-hpr-sim`;
   - a string literal has no code formatting, so its `hpr` is the command when a subcommand of
     `hpr_cli::command()`, a flag or a `{}` placeholder follows, or when it is the whole literal
     (a command or extension name);
   - an item under `#[cfg(test)]`, and a module file declared under it (`mod tests;`), is compiled
     only for tests and ships nothing, so neither is read. An item is what the attribute's first
     word in code (after other attributes and `pub`, `unsafe`, `async`) names by its keyword, such
     as `fn`, `mod`, `impl` or `const`, read to the end of its body or its `;`; a field, a variant,
     a match arm or a statement under the attribute is read like the code around it, erring strict.
     Strings and comments never decide it;
   - an unpublished crate's description is checked too, since it costs nothing.
5. **The allowlist's entries are narrow by rule** (chosen): each gives at least 8 bytes of text
   beyond the mentions in it, so an entry names a place and not just the name, and states how many
   mentions it covers, so a new mention inside a copy of its text fails too.
6. **The format's name.** Prose calls the design format "the HPR design format", by the suite's
   name in capitals, which the check allows; its files stay `.hpr` and `.hprz`.

**Consequences.** M0.9a1's allowlist has 27 entries, covering 32 mentions: history ("called … before 0.1"), old stamps
that must still read, the anchors of headings that name the `hpr sim` command, ADR file names, the
private fixtures repository's name and the flight engine crate's folder. M0.9a2 will add the
site's history and anchors under the same rules. A heading written "HPR Sim" gets an anchor that
spells the old name, so headings that are linked to say "the simulator" instead.
