# ADR-234: The pages' sheets and trust split again: the sheet count's check first (2026-10-11)

- **Status:** accepted; splits M0.9d12 ([ADR-233][adr-233]'s rest of #384) in two; adds M0.9d13
- **Summary:** M0.9d12 took the rest of #384: two to six sheets a page and every page opening with how far to trust it. M0.9d12 now covers the sheet count's check: `cargo xtask site` and a site test count each page's `##` sections, its sheets, and fail a page with fewer than two or more than six, except the 38 pages over six on a list in `xtask/src/site/sheets.rs`, which only shrinks: a listed page at six or fewer fails until it comes off, and so does a listed page the summary doesn't hold. The glossary, 187 sections, one a term, now groups its terms in six by first letter, each term a `###` heading with the same anchor. The new M0.9d13 keeps the old done-when whole (#384 closed, every rule held) and takes the 38 pages' grouping and the trust openings.

[adr-233]: 0233-the-intro-chip-and-lead-first.md

**Context.** `web.md`, *Page anatomy*, asks for "two to six sheets per page", a sheet being a
section with its number and name in the rail. Measured on `main` at 78bc0084: of the summary's 62
pages none has fewer than three `##` sections, 23 have three to six, and 39 have more, from 7
to 16, with the glossary at 187. Grouping a page's sections is a judgment about its plan, page
by page, and so is rewriting each page's opening to say how far to trust it; together they are
more than one increment. A check that holds every page but a named list makes the rule hold
now for the 23 pages that meet it and for every new page, and the list shows what is left.

**Decision.**

1. **A sheet is a `##` section of the page's source,** counted as the parser reads it, so a
   heading in a code block isn't one. The build already makes each `##` a sheet and fails one
   it can't wrap (ADR-231), so the source's count is the built page's. The print page holds
   every chapter and counts each chapter's sheets apart, as the source does.
2. **Two to six,** `web.md`'s numbers. A page under two fails with no exception; none is
   today. A page over six fails unless it is on `OVER_SIX`, which a test holds exact: a listed
   page at six or fewer fails, as does a listed name the summary doesn't list. The not-found
   page, outside the summary, has no sections and isn't counted: it is a message, not a page
   of the guide.
3. **The glossary groups its terms by first letter** in six sheets of 19 to 40 terms (numbers
   and A to C, D to F, G to L, M to P, Q to S, T to W). Each term keeps its own heading, now a
   `###`, which mdBook gives the same anchor, so every link to a term still lands.
4. **M0.9d13, the pages' sheets and trust,** keeps M0.9d12's old done-when unchanged: #384
   closed, each rule it lists held by `xtask site`'s page check or a site test. It takes the
   38 listed pages and every page opening with what works and how far to trust it.

**Consequences.** A new page with more than six sections fails until it is grouped. The
glossary's terms show in the `###` heading's capitals, as every `###` on the site does; a term's
text, its anchor and its links are unchanged. The trust openings stay as they are, in *In short*
on model pages and trust notes elsewhere, until M0.9d13.
