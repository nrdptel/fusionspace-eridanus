# ADR-222: The page anatomy split: the page's frame first (2026-10-09)

- **Status:** accepted; splits M0.9d3 ([ADR-208][adr-208]'s M0.9d) in two
- **Summary:** #384 lists eight parts of the product system's page that the site doesn't draw. M0.9d3 now covers the frame every page shares, all in the stylesheet: text on the system's type scale (with tabular figures in Archivo and navigation in Cascadia Mono 14 px, the current page underlined 2 px), prose at most 68 characters a line, and gutters of 16 px on a phone and 32 px from 720 px, each held by `xtask site`'s page check at every width with a canary. The new M0.9d5 keeps the old done-when whole (#384 closed, every rule held), after M0.9d4, and takes what changes each page's markup: title blocks, sheets, the intro, the lockup header with no menu button from 720 px, the system's notes, and every page opening with how far to trust it.

[adr-208]: 0208-the-design-audit.md

**Context.** M0.9d3 was one increment for all of #384. Three of its parts are rules of the
stylesheet that the page check can read from computed style and layout, the way M0.9d2 read
colors: the type scale and figures (`foundations.md`, *Type*), the measure (*Type*, *Width*), and
the gutters (*Width*). The rest change what each page is made of: a title block and an intro on
every page need a preprocessor or a template, sheets need the long model pages cut to six `h2`
sections or fewer, the system's notes need the trust and caution blockquotes rewritten, and the
lockup header with no menu button from 720 px needs the sidebar kept open there by script, its
`aria-hidden` and link focus included, which mdBook's script sets only from 1,080 px. One cycle
holds the frame with its checks; it doesn't hold all eight.

**Decision.**

1. **M0.9d3: the page's frame.** Every element that shows text, drawn or hidden until asked for,
   its `::before` and `::after` included, has a size and line height of one of the scale's
   tokens in that token's family: Cascadia Mono 12/16, 14/20, 20/24, 28/32 or 40/44; Archivo
   14/20, 16/24 or 20/28 (`hero` is for print covers and splash screens only). Capitals only at
   12/16 and 14/20 in Cascadia Mono (`label`, `heading`), no weight under 400, tabular figures
   wherever Archivo is set. A superscript or subscript sits outside its line's scale but never
   under 12 px. The sidebar's text is Cascadia Mono 14 px and its current page is underlined
   2 px. No line of prose in `main` (a paragraph, a list item, a quote, a caption) holds more
   than 68 characters as drawn, spaces between words counted once. The column sits 16 px from
   the pane's edges on a window under 720 px and 32 px from 720 px, or further where it has
   reached its widest.
2. **The mapping.** `h1` is the `title` token, `h2` the `subtitle` (a sheet's name), `h3` the
   `heading` (a section head inside a sheet, in capitals), and `h4` to `h6` run-in heads in body
   type, Archivo 16/24 semibold: the system has no fourth level, and 34 headings use one. Body
   text is 16/24, tables' cells too; footnotes and the copy tooltip `small`; column heads and the
   title block's field names `label`; code 14/20, at its heading's own size inside a Cascadia
   Mono heading. Archivo is weighed only against the floor: bold for emphasis is as written.
3. **The measure is 24 em.** 68 characters of Archivo at 16 px take about 400 px; a line dense
   with narrow letters holds 69 at 25 em (one list item on the ORK format page did), so prose
   stops at 24 em (384 px). Tables, code and figures keep mdBook's 750 px column, flush left
   with the prose; a paragraph that holds a figure keeps the column too, as ADR-219 lays figures
   out from it.
4. **M0.9d5: the rest of #384**, with M0.9d3's old done-when unchanged: title blocks on every
   page, sheets with their rail, the intro (designation tag, status chip, `lead`), the header's
   lockup and no menu button from 720 px, the system's notes, and every page opening with how
   far to trust it. It goes after M0.9d4, already queued.

**Consequences.** The page check reads each page's frame once per width, in the theme it loads
in; type doesn't change with the theme. Counting characters per line costs about two range reads
a word; the whole check took 161 s for 66 pages at 41 widths on the maintainer's machine, against
about 114 s before (#432 tracks its time). The prose column is narrower than the tables and code
beside it, which is the system's rule (prose to 68 characters, content to 1,120 px), not a
layout of its own.
