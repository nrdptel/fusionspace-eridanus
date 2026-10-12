# ADR-250: The measure as a median and a maximum, a reading scale that grows with the window, and the column flush left, from the product system's Rev B (2026-10-12)

- **Status:** accepted; moves the product system's pin from Rev A (`f45454f`) to Rev B (`9bdebfd`), as [ADR-164](0164-the-fusionspace-product-system.md) lets a deliberate update do; reopens the design audit at the new pin ([ADR-208](0208-the-design-audit.md)); amends the page check of [ADR-223](0223-the-pages-frame-first.md)
- **Summary:** Prose on the site stopped at 24 em, about 384 px, so a desktop window was mostly empty. The product system's Rev B states the measure as a median line of 60 to 75 characters and none over 80, set as 29.5 em of the text; grows the type scale with the window on a reading page (16 to 18 px body between 1,024 and 1,600 px); sets the column flush left beside the sidebar up to 1,120 px; and ships the code that measures these rules, which the site's page check now runs unchanged.

**Context.** A review of the site on a 1,512 px desktop window found most of the window empty:
prose ran about 384 px wide inside mdBook's centered 750 px column, about a quarter of the
window. The cause was Rev A's rule, "line length at most 68 characters", which reads two ways.
The site read it as a maximum and held prose to 24 em, so that even a line of narrow letters
stays under 68, which left most lines well short of it. The system's own stylesheet read it as
`68ch`, 68 widths of the digit "0", which is wide in Archivo, so a line of ordinary text runs past
68 characters. Neither the rule nor the stylesheet had been tested on a real site, and the
system's page check stopped at 1,280 px.

The published guidance agrees on a range, not a hard 68: Bringhurst gives 45 to 75 characters
with 66 ideal; the US Web Design System 45 to 90 with a target of 66 (its default measure token
is `68ex`); GOV.UK at most 75; WCAG 2.2's SC 1.4.8 (AAA) no more than 80 per line.

**Decision.**

1. **The measure** (`foundations.md`, *Type*): the median line 60 to 75 characters and none over
   80, spaces counted, set in em of the text: 29.5 em of Archivo. `cargo xtask site` prints what
   the system's counter reads: on 2026-10-12, at all 41 widths, "69 of 73 pages set 20 lines or
   more at the full measure at one or more of these widths, their medians 61 to 67 characters;
   the longest line of prose on any page 78 characters"; the other 4 set too few lines at the
   full measure to judge.
2. **A reading scale** (`foundations.md`, *Type*): a reading page (documentation, a report, an
   article) grows the whole type scale together, ×1 up to a 1,024 px window, ×1.125 from 1,600
   px, linear between, so body text goes from 16 to 18 px (17.7 px at 1,512). It is a clamp() of
   rem and vw, so browser zoom and the reader's font setting still enlarge it (WCAG 1.4.4). One limit
   follows: at 1,512 px, 200% zoom enlarges text 1.81× over what the reader saw at 100%, not 2×,
   since the window's growth is already in the 100% size; 2× arrives by 221%. The √2
   steps keep their ratios and the measure its characters. The site's sidebar and menu bar keep
   the scale's sizes.
3. **The column** (`foundations.md`, *Space and layout*): beside the sidebar it starts at the
   gutter, flush left, and runs up to 1,120 px. Prose stops at the measure; tables, code and
   figures may take the column's width, and a code block is at least as wide as the prose.
   mdBook's page arrows, which would sit over a flush-left column, go under the page at every
   width. The space right of the prose stays empty on a page of prose: filling it with longer
   lines would break the measure.
4. **Implemented and tested where the rule lives.** The product system's Rev B carries the rules,
   its mdBook theme sets the page's frame (the type scale on mdBook's elements, growing inside
   `.content`, the measure, the column and its gutters), and `product/web/check/fusionspace-check.js`
   holds the rules' numbers, generated from the system's build constants, with the code that
   counts the characters of each line of prose as drawn (ported from this site's page check).
   The system's build runs it on its own pages from 320 to 1,920 px.
5. **The site copies Rev B unchanged** (`theme/fusionspace-mdbook.css`,
   `xtask/src/site/fusionspace-check.js`; `cargo xtask site` compares both with the pinned
   commit) and drops what moved into the theme from `hpr.css`: the page's frame, and the fixes
   to mdBook's own defaults that depend on mdBook's markup alone (the roles as `--fs-<role>` in
   every theme, with scripts off and in print; code in ink; square corners; motion; the focus
   ring; forced colors; the fonts' fallbacks; nothing cut off on a phone; print's link
   addresses). `hpr.css` keeps what only this site's build writes (notes, sheets, the title
   block, the theme switch, the system's icons, the print page's cover): 202 lines, from 469. Its page check reads the type
   scale, the measure and the widths from the copied file instead of its own constants; reads
   every size inside `.content` against the scale times the factor the window's width gives, on
   a page that declares the scale (`--fs-text`), so a page whose scale is off is read against the
   rule, not against itself; judges the measure with the system's counter, the median only on 20
   lines or more set at the full measure; fails prose held under the measure by a cap or a width
   (a max-width on `main` or a box around it below the measure, or a width of its own), which
   the median alone can't see, and leaves prose narrowed by its own margins or padding to
   review, since those are how lists, quotes and notes indent; and holds the column's left gutter at every width.
   New canaries show each failure at every width (a line of 81 characters, prose held to a
   median of about 40, prose held by a cap on `main`, by a width of its own and by a max-width in percent, a centered
   column, a declared scale off the system's) and pass its
   counterparts (a line of 80, prose at the measure, prose beside a classic scrollbar, indented
   prose, a page on the reading scale).

**Consequences.** On a 1,512 px window body text is 17.7 px (16 px times the scale's factor there,
1 + 0.125 × 488 / 576) and prose about 522 px wide (29.5 em of it), where it
was 16 px and 384 px; tables and code may use up to 1,120 px instead of 750. Pages of prose still
leave the right of a wide window empty, by the measure's choice. The design audit reopens at
`9bdebfd`: its rows reviewed at `f45454f` whose sections did not change are carried over with
the comparison that shows it, and the five sections Rev B changed are read again. A second mdBook site would still need this site's build steps
for the system's notes, sheets, title block and icons; those stay here until a second site
needs them.
