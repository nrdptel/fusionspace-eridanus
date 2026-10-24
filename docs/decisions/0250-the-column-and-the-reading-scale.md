# ADR-250: Prose the width of its column, the column centered beside the sidebar, and a reading scale that grows with the window, from the product system's Rev B and Rev C (2026-10-12)

- **Status:** accepted; moves the product system's pin from Rev A (`f45454f`) to Rev C (`943492d`), as [ADR-164](0164-the-fusionspace-product-system.md) lets a deliberate update do; reopens the design audit at the new pin ([ADR-208](0208-the-design-audit.md)); amends the page check of [ADR-223](0223-the-pages-frame-first.md)
- **Summary:** Prose on the site stopped at 24 em, about 384 px, so a desktop window was mostly empty. The product system's Rev B and Rev C let prose take its column's whole width, center the column beside the sidebar up to 1,120 px, grow the type scale with the window on a reading page (16 to 18 px body between 1,024 and 1,600 px), and ship the code that checks these rules, which the site's page check now runs unchanged.

**Context.** A review of the site on a 1,512 px desktop window found most of the window empty:
prose ran about 384 px wide inside mdBook's centered 750 px column, about a quarter of the
window. The cause was Rev A's rule, "line length at most 68 characters", which reads two ways.
The site read it as a maximum and held prose to 24 em, so that even a line of narrow letters
stays under 68, which left most lines well short of it. The system's own stylesheet read it as
`68ch`, 68 widths of the digit "0", which is wide in Archivo, so a line of ordinary text runs past
68 characters. Neither the rule nor the stylesheet had been tested on a real site, and the
system's page check stopped at 1,280 px.

Rev B, the first answer, kept a measure from print's guidance: a median line of 60 to 75
characters and none over 80 (Bringhurst 45 to 75, the US Web Design System's target of 66,
GOV.UK's 75, WCAG 2.2's SC 1.4.8 at 80), set as 29.5 em of the text, with the column flush left
beside the sidebar. It still left most of a wide window empty: prose about 522 px wide in a
1,512 px window, and an empty band right of it. The review asked for text that grows with the
window, and Rev C, the same day, makes the column the limit instead.

**Decision.**

1. **Line length is the column's** (`foundations.md`, *Type*): prose takes its column's whole
   width, as tables, code and figures do, and nothing holds it narrower. The column stops at
   1,120 px, so that is the longest line. `cargo xtask site` prints what the system's counter
   reads, as a report, not a rule: on 2026-10-12, at all 41 widths, "70 of 73 pages set 20
   lines of prose or more at one or more of these widths, their medians 27 to 118 characters;
   the longest line of prose on any page 156 characters". The highest medians come from wide
   windows, where the text beside each sheet's rail is 904 px wide. That is longer
   than print's 45 to 75 and the US Web Design System's 90, by choice. On screen, long lines read
   faster: 100 characters against 25 (Dyson and Kipping, *Visible Language* 32, 1998), and 95
   the fastest of 35 to 95 (Shaikh and Chaparro, Human Factors and Ergonomics Society 49th
   Annual Meeting, 2005). Dyson and Kipping's comprehension measure was a check that people had
   read the text: readers ticked whether five questions could be answered from it. Line length
   didn't change it. Their readers rated 55 characters easiest and 100 hardest, with 25 fifth
   of six, so they favored medium lines, not short ones. Shaikh and Chaparro's readers had no
   significant preference: 30% liked 35 characters best and 30% liked 95. The same lab's fuller
   comprehension test points the other way from the speed results: 55 characters were
   understood better than 100, F(2, 68) = 3.17, p < .05 (Dyson and Haselgrove, *International
   Journal of Human-Computer Studies* 54, 2001, p. 598). No study found separates 60 to 75
   characters from 95 to 110 on comprehension, so the choice stands as a trade, not a finding
   ([prose measure on screen](../research/prose-measure-on-screen.md), §1 and §3). A reader
   who wants shorter lines can narrow the window.
2. **A reading scale** (`foundations.md`, *Type*): a reading page (documentation, a report, an
   article) grows the whole type scale together, ×1 up to a 1,024 px window, ×1.125 from 1,600
   px, linear between, so body text goes from 16 to 18 px (17.7 px at 1,512). It is a clamp() of
   rem and vw, so browser zoom and the reader's font setting still enlarge it (WCAG 1.4.4). One limit
   follows: at 1,512 px, 200% zoom enlarges text 1.81× over what the reader saw at 100%, not 2×,
   since the window's growth is already in the 100% size; 2× arrives by 221%. The √2
   steps keep their ratios, and a line's characters grow more slowly than its column. The
   site's sidebar and menu bar keep the scale's sizes.
3. **The column** (`foundations.md`, *Space and layout*): centered in the space beside the
   sidebar, up to 1,120 px, with gutters of 16 px on a phone and 32 px from 720 px, so a window
   wider than the column has equal margins on both sides instead of one empty band on the
   right. Text and tables stay flush left inside it. Where the column is 880 px wide or more,
   each sheet's rail stands beside its text, 184 px wide with a 32 px gap, as the system's
   `.fs-sheet` sets it; the column, not the window, decides, since the sidebar takes part of the
   window. Beside the rail the text is 216 px narrower, which a figure can't spare: in a grid,
   the 960 px plot was drawn 692 px wide at a 1,280 px window, its 12 px labels at 8.7 px. So the
   rail is a float, and a figure clears it and takes the whole column: it starts below the rail,
   never over it, and is shown at its size wherever the column holds it. mdBook's page arrows,
   which would sit over the column, go under the page at every width.
4. **Implemented and tested where the rule lives.** The product system's Rev C carries the rules,
   its mdBook theme sets the page's frame (the type scale on mdBook's elements, growing inside
   `.content`, and the column with its gutters), and `product/web/check/fusionspace-check.js`
   holds the rules' numbers, generated from the system's build constants, with the code that
   finds prose held narrower than the box that holds it, and a line counter (ported from this
   site's page check) for the report. The system's build runs the check on its own pages from
   320 to 1,920 px.
5. **The site copies Rev C unchanged** (`theme/fusionspace-mdbook.css`,
   `xtask/src/site/fusionspace-check.js`; `cargo xtask site` compares both with the pinned
   commit) and drops what moved into the theme from `hpr.css`: the page's frame, and the fixes
   to mdBook's own defaults that depend on mdBook's markup alone (the roles as `--fs-<role>` in
   every theme, with scripts off and in print; code in ink; square corners; motion; the focus
   ring; forced colors; the fonts' fallbacks; nothing cut off on a phone; print's link
   addresses). `hpr.css` keeps what only this site's build writes (notes, sheets, the title
   block, the theme switch, the system's icons, the print page's cover). Its page check reads
   the type scale and the widths from the copied file instead of its own constants; reads every
   size inside `.content` against the scale times the factor the window's width gives, on a
   page that declares the scale (`--fs-text`), so a page whose scale is off is read against the
   rule, not against itself; fails prose held narrower than the box that holds it, with the
   system's check: a box narrower than its holder allows is drawn again with no max-width and
   no width of its own, and fails if it grows; fails a column whose `max-width` isn't 1,120 px;
   and holds the gutters at 16 or 32 px until the column reaches its widest, and the column
   centered after. New canaries show each failure at every width (prose held to 10 em by a
   max-width, by a width and in a box centered by its margins; mdBook's 750 px column; a column
   let run the pane's width; a 200 px column flush left) and pass their counterparts (prose
   filling its column in paragraphs, a list, a definition list and a grid's and a flex box's
   cells; a page on the reading scale).

**Consequences.** On a 1,512 px window body text is 17.7 px (16 px times the scale's factor there,
1 + 0.125 × 488 / 576) and the prose takes the 1,120 px column, where it was 16 px and 384 px;
in a wider window the column stays 1,120 px, centered. Lines run longer than print's guidance,
by the choice above. The design audit reopens at `943492d`: its rows reviewed at `9bdebfd` whose
sections did not change are carried over with the comparison that shows it, and the four
sections Rev C changed (*Status*, *Type*, *Space and layout*, *Documentation sites*) are read
again. A second mdBook site would still need this site's build steps for the system's notes,
sheets, title block and icons; those stay here until a second site needs them.
