# ADR-231: The page anatomy split again: the drawing first (2026-10-10)

- **Status:** accepted; splits M0.9d9 ([ADR-229][adr-229]'s rest of #384) in two; adds M0.9d11
- **Summary:** M0.9d9 took the rest of #384: the intro (designation tag, status chip, lead), sheets with their rail and two to six a page, every page opening with how far to trust it, the header's theme switch and headings in order. M0.9d9 now covers what the build can draw on every page as it stands: after mdBook builds the site, `cargo xtask site` opens each page's intro with the designation tag, makes each `h2` section a sheet with `SHEET n / N` in its rail under a 2 px ink rule, and turns mdBook's six-theme menu into the system's segmented switch (Auto, Light, Dark), in the header from 960 px and at the top of the sidebar below. A file check fails a page without them or with a heading that skips a level, and the page check fails each drawn off the system's, with canaries. The new M0.9d11 keeps the old done-when whole (#384 closed, every rule held) and takes what rewrites the pages' own words: the status chip and the one-sentence lead, two to six sheets a page, and every page opening with how far to trust it.

[adr-229]: 0229-the-page-anatomy-the-frame-first.md

**Context.** ADR-229 left M0.9d9 five parts of `web.md`'s *Page anatomy* and *Accessibility*.
Three can be drawn on every page from what mdBook already writes: the designation tag over the
title, a sheet for each `h2` section with its number and name in the rail, and the theme switch
(#429's third item: mdBook's menu lists six themes for two looks). A fourth, headings in order, is
a check the pages already pass. The rest changes each page's text. A status chip and a lead need a
status and one sentence written for each of 62 pages; the system's two to six sheets a page
needs 39 of them restructured (measured on this branch: 3 to 16 `h2` sections on every page but
the glossary, which has 187); and every page opening with what works and how far to trust it is
the same rewrite of each page's first lines. One increment holds the drawing with its checks and
canaries; the rewrite of 62 pages is a second.

The theme switch needs room the header doesn't have at every width. The header measured on the
built site at 720 px, the sidebar open beside it, is 412 px wide with 26 px free; the switch's
three buttons in Cascadia Mono 14 with the system's 12 px padding take about 186 px. With mdBook's
theme button gone, the row holds them from about 854 px, and from about 870 px beside a 15 px
scroll bar (Linux, Windows), so 880 px would leave the row a few pixels to spare; 960 px leaves
it about 80. Under 720 px the lockup already takes a row of its own, and the switch would need a
third.

**Decision.**

1. **M0.9d9: the drawing.**
   - **The intro's tag.** The build sets the site's designation (`FS-ACHERNAR · SW · TOOL 001`,
     the title block's) in the system's chamfered tag above every page's title, the two in an
     intro block the status chip and lead join in M0.9d11.
   - **Sheets.** The build wraps each `h2` and what follows it, up to the next `h2` or the end
     of the page (on the print page, of the chapter), in a sheet: a `section` with a 2 px ink
     rule on top and a rail holding `SHEET n / N` in `label` type over the sheet's name, the
     system's `.fs-sheet` markup. The rail stands over the sheet's text, as the system's sheet
     draws it in a narrow box: beside the sidebar the docs column is a measure wide, with no room
     for a 184 px rail beside the prose. An `h2` the build can't wrap, inside another element,
     fails the build.
   - **The theme switch.** The build turns mdBook's theme menu into the system's segmented
     control, a group of three buttons, Auto, Light and Dark, each 44 px tall in Cascadia Mono
     14, the chosen one filled in ink and `aria-pressed`; mdBook's script still sets the theme,
     through its own buttons, and its menu button stays in the page, hidden, as its script needs
     it. The Field theme joins with #452. The switch stands in the header from 960 px, where the
     header holds it on one row with room to spare, and `theme/hpr.js` moves it to the top of the sidebar below,
     the page's navigation, which a menu button opens under 720 px as `web.md` allows. It works
     only with scripts, so without them it isn't drawn.
   - **Headings in order.** A file check fails a built page whose headings skip a level going
     down (an `h2` followed by an `h4`).
   - The file check fails a built page without the tag, an `h2` outside a sheet, a sheet
     numbered out of order, a theme menu not turned into the switch, or a heading out of order.
     The page check fails, at every width, a sheet whose top rule isn't 2 px of ink, whose number
     isn't in `label` type above its name, a tag not chamfered or not in Cascadia Mono 12, and a
     switch whose buttons aren't Auto, Light and Dark, 44 px tall in Cascadia Mono 14, with the
     page's theme the one pressed and filled in ink, not in the header from 960 px or in the
     sidebar below, or that doesn't change the theme when pressed. Each has canaries both ways.
2. **M0.9d11: the pages' own words,** with M0.9d9's old done-when unchanged: #384 closed, each
   rule it lists held by `xtask site`'s page check or a site test. It takes the intro's status
   chip and one-sentence lead, two to six sheets a page, and every page opening with what works
   and how far to trust it. It goes after the queued M0.9d10.

**Consequences.** Under 960 px a reader finds the theme switch with the chapters, not in the
header; under 720 px behind the menu button. Every page's `h2` sections now carry a sheet number,
which on a page of more than six sections, until M0.9d11, reads past the system's six. The
rewrite is tested on mdBook 0.5.4's output; a newer mdBook that changes the theme menu or the
headings' markup fails the build until the rewrite learns it. #429 stays open for its other
items; its third is fixed here.
