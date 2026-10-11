# ADR-235: Every page in two to six sheets and opening with how far to trust it (2026-10-11)

- **Status:** accepted; closes M0.9d13 ([ADR-234][adr-234]'s rest of #384) and with it M0.9d
- **Summary:** The 38 pages that had more than six `##` sections are grouped into two to six sheets, so the check fails any page outside that range and the list of exceptions is gone. Every page but a model page now holds a trust note, a quote block opening with `**How far to trust it.**`, between its title's lead and its first sheet; a model page opens with *In short* instead. `cargo xtask site` and a site test fail a page that doesn't.

[adr-234]: 0234-two-to-six-sheets-the-check-first.md

**Context.** `web.md`, *Page anatomy*, asks for "two to six sheets per page", and *Documentation
sites* says "every docs page starts with what works and how far to trust it". After M0.9d12, 38
pages had 7 to 16 sheets on a list the check excused, and 16 of the 62 pages had no trust note
before their first sheet: 14 had none at all, and 2 had one further down, inside a sheet.

**Decision.**

1. **Grouping keeps every heading's words.** A page with more than six sections gets `##`
   group headings, each named for what it holds, at most four words, with a one-sentence lead;
   the sections under one move down a level, keeping their words. mdBook makes a heading's
   anchor from its words, not its level, so every link into a page still lands, and the site's
   link check holds that. A group's name never repeats another heading's anchor on its page,
   which would make mdBook number a later one. Where two neighboring sections belong together,
   an existing `##` serves as the group. A model page keeps *In short* as its first sheet.
2. **No exceptions.** `OVER_SIX` in `xtask/src/site/sheets.rs` and its test are removed; a page
   over six or under two fails `cargo xtask site` and the site test
   `every_page_has_two_to_six_sheets`.
3. **The opening is the text before the first sheet.** A page's title and lead say what it
   covers; before its first `##`, a top-level quote block whose first paragraph opens with the
   bold `How far to trust it.` says how far to trust it. One rule says what is the page's: the
   quote block stands inside no other block, so a note in a list, another quote block or a
   footnote doesn't count; nor does an alert (`[!NOTE]`), another kind of note. The rule asks where the note is, not
   that it be the first block: a page may set out its steps or a command before it, as long as
   the note comes before the first sheet. More trust notes further down stay allowed, each on a
   result someone might fly on.
4. **Model pages open with *In short*,** whose *How well it is validated* item says how far to
   trust the model, as the check has held since M0.4. Their opening isn't asked for a note too.
5. **Pages of rules or records** (the writing rules, the glossary, the release steps, the
   decision index) carry the note as well: it says what kind of text the page is, what checks
   it, and what to rely on instead. The product system asks it of every page, and the note's
   shape fits text that isn't a result.
6. **The check is its own step,** after the sheets, as `site openings`, so the site's fixture
   pages in the tests, which aren't real pages, don't need notes. A site test runs it on the
   real pages, and another on hand-made pages: a note in its place passes, wrapped or followed
   by a list; no note, one only after the first sheet, one in a list, a nested quote block or a
   footnote, an alert carrying the label, another label, or the label later in a paragraph fails.

**Consequences.** #384 closes, and with #383 and #385 closed, M0.9d is done. The design
audit's *Page anatomy*, *Site patterns* and *Show the working* rows are met; the rest that
cited #384 now cite the open issues left (#386, #387, #401, #429, #452). Two gaps no issue
tracked are issues now: the search field's missing label (#465), and the README banners, which
no check reads, keeping *Type* not met (#467). A new page
with more than six sections, or without a note before its first sheet, fails the build until it
is grouped or given one.
