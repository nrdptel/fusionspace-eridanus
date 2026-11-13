# ADR-343: The docs site drawn as a tool: the frame off (2026-11-12)

- **Status:** accepted; closes [M0.17a][m0-17a] ([ADR-339][adr-339]'s first increment); supersedes the parts of [ADR-164][adr-164], [ADR-223][adr-223], [ADR-225][adr-225], [ADR-229][adr-229], [ADR-231][adr-231], [ADR-232][adr-232], [ADR-233][adr-233], [ADR-234][adr-234], [ADR-235][adr-235] and [ADR-252][adr-252] listed under *Superseded* below
- **Summary:** The docs site follows the product system at `dd7552f`, the commit that gave it three levels, and is drawn as a tool. Pages lose the designation tag and the sheet numbers; each `##` section is a part under a 2 px rule. Every page but the print page ends with a one-line footer: the mark and "FusionSpace HPR", the version, the date of issue, the site's status chip and a link to Start here. Start here keeps the title block as the site's about box, and the print page still ends with it. Labels, navigation, note heads, chips and the switch are in Archivo, in sentence case, at the system's web sizes; CAUTION and WARNING keep capitals, in the wide cut.

[m0-17a]: ../decisions-and-roadmap.md#m0-17a
[m0-17b]: ../decisions-and-roadmap.md#m0-17b
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-223]: 0223-the-pages-frame-first.md
[adr-225]: 0225-the-page-anatomy-the-notes-first.md
[adr-229]: 0229-the-page-anatomy-the-frame-first.md
[adr-231]: 0231-the-page-anatomy-the-drawing-first.md
[adr-232]: 0232-the-pages-vitals-measured.md
[adr-233]: 0233-the-intro-chip-and-lead-first.md
[adr-234]: 0234-two-to-six-sheets-the-check-first.md
[adr-235]: 0235-every-page-opens-with-trust.md
[adr-252]: 0252-the-sidebar-put-away-and-named-page-arrows.md
[adr-284]: 0284-the-maintainers-own-voice.md
[adr-339]: 0339-m0-17-the-docs-site-at-the-tool-level-queued-second.md
[design12]: https://github.com/nrdptel/fusionspace-design/pull/12
[audit]: ../research/design-conformance.md
[i1442]: https://github.com/nrdptel/fusionspace-eridanus/issues/1442
[i1443]: https://github.com/nrdptel/fusionspace-eridanus/issues/1443
[i1444]: https://github.com/nrdptel/fusionspace-eridanus/issues/1444
[i1445]: https://github.com/nrdptel/fusionspace-eridanus/issues/1445

**Context.** [ADR-339][adr-339] queued [M0.17a][m0-17a]: move the site to the product system's
[three-levels change][design12] and take off the frame a record wears. The change, merged as
`dd7552f` on 2026-11-10, says a record (a drawing, a report, an exported plot) keeps the
drawing's frame, with the full title block kept for drawings, boards and an about box. A front
page and a tool keep what the drawing means (line types, trust notes, signal colors, units) and
leave the frame off. A docs site is a tool. Its page ends with "a one-line footer: tool and
version · data and its date · units · About" (`web.md`, *Page anatomy*), its labels are in
sentence case, and a level table in `foundations.md`, *Type*, gives each web element its size:
a title 32 to 39 px on lines 1.1 times that and a section head 25 to 31 on 1.15, both growing
with the window, in the wide cut; a heading inside a section 16/24, a label or column head
13/18, a button, navigation or segmented control 15/20, a status chip or a note head 13/16, and
a signal word 13/16 in the wide cut (weight 640, width 118%) in capitals. The system's mdBook
theme draws the title at 32/36 and a section head at 24/28 on a 16 px body, growing with the
reading scale.

**Decision.**

1. **The pin moves to `dd7552f`.** `theme/` copies the system's mdBook theme, fonts (with the
   new `Archivo-WideSemiBold.woff2` and its `@font-face` rule) and icons at that commit, with
   the card in `docs/images/` and the check script in `xtask/src/site/`, and `cargo xtask site` compares each with it when `refs/fusionspace-design` is
   checked out: byte for byte, and `fonts.css` rule by rule with its `fonts/` folder dropped. `validation/refs.lock.toml`, `theme/README.md`,
   `THIRD-PARTY-NOTICES.md` and the README name it.
2. **No record's frame on a page.** The build draws an intro (the title, then its one-sentence
   lead) and a part for each `##` section: a 2 px rule in the text's color and the section's
   name, with no number. The file check fails a page with a designation tag, a sheet number or
   a sheet's rail in it, and the page check fails one drawn; each has a canary that must fail,
   and the parts' canary must pass.
3. **A one-line footer on every page but the print page.** In order: the mark, 24 px tall, and
   "FusionSpace HPR"; `Version` and the workspace's version; `Issued` and the date of the
   commit the site is built from; the status chip; and "About HPR Sim", a link to Start here.
   The build writes the line from the site's title block, so it can't drift from it. The mark is 24 px,
   not the system's 20 px example, because the brand kit's README sets 24 px as the smallest
   the mark is drawn at, and the page check already holds every lockup to it.
4. **The status chip moves into the footer.** The system's tool level puts status "in the
   about box". The site keeps it on every page all the same, in the footer, because the done-when
   names the chips and a page's status is a claim about how far to trust it: *In preparation*
   (outlined in the link color, a square icon) while the title block's status is IN
   PREPARATION, as it is while the pages describe the next release, and *Released* (filled in
   green, a circle) once it says RELEASED. The chip is in sentence case, Archivo 13/16 at
   weight 600, as ADR-339's done-when asks and the level table sets a chip, in its words as
   written. The system's `writing.md` still lists status words in capitals on chips; the two
   disagree at the pin, and the site follows the level table.
5. **Start here keeps the title block as the about box.** Its table (owner, title,
   designation, version, date of issue, status, units, data and fonts) stays where it is, and
   with the print page's block is the one place the site's frame shows the designation. It is a title block, so its status keeps the drawing word in capitals,
   IN PREPARATION; the chip reads it and shows it in sentence case. The footer leaves out the data's date and the units: they're facts about
   the site, not the page, and the footer links to them.
6. **The print page stays a record.** The printed guide is a document a reader keeps, so it
   ends with the whole title block, the page's own title after the site's, as before, which
   ADR-339's done-when keeps. The system ends a record with a one-line footer too and keeps the
   full block for drawings, boards and an about box; the printed guide, a document a reader
   keeps whole, is the site's nearest to a drawing, so it keeps the block. Its
   cover keeps its title, lead and button and, like every intro, loses the tag and the chip.
7. **Two to six parts a page, as before.** The system's page anatomy asked for two to six
   sheets, and the site's check ([ADR-234][adr-234], [ADR-235][adr-235]) counted them. The
   count still serves a reader, so it stays, for the parts.
8. **Type at the web level table.** Navigation and the theme switch's buttons are Archivo
   15/20; page-arrow labels, column heads and the about box's field names 13/18 semibold; note
   heads and the chip 13/16 semibold, in sentence case. A caution's and a warning's head say
   CAUTION and WARNING in capitals, in the wide cut. The site writes no DANGER note: its kinds
   are the trust note, note, caution and warning, and any other alert fails the build. The page
   check adds to the system's scale, for Archivo, the sizes the system's mdBook theme draws the
   table's levels at (the title 32/36, a section head 24/28 and the menu bar's title 20/24 on a
   16 px body) and the table's fixed sizes, holds capitals to the
   signal word's size and the print title block's `label` and `heading` tokens, and its canaries cover a signal word in sentence case, one in the narrow
   cut and a trust note in capitals. Code in a heading or a column head takes the heading's
   size, as the system's theme sets it (`font-size: inherit`), so the check accepts Cascadia
   Mono there at the heading's own level and nowhere else, with a canary each way.
9. **The page arrows' icons drop from 24 to 16 px.** The system's theme sizes them 1 em of the page's
   text, the system's 1× the text, which grows to 18 px on a wide window. The page check holds
   each icon in mdBook's chrome (its `.fa-svg` spans) to the system's fixed sizes, 16, 20 and
   24 px, so `hpr.css` holds them at 16 px.
10. **The wide cut is asked for early.** Titles and section names now draw in Archivo's wide
    cut, so `theme/head.hbs` preloads it with the four faces it already asked for, and a title
    is less likely to be drawn first in a stand-in and redrawn. [ADR-232][adr-232] found a
    fifth face pushed the largest contentful paint to 2.50 s in a probe; `cargo xtask site`'s
    vitals check holds every page of the guide to its limits with the five: 2.5 s for that
    paint, 0.1 for layout shift and 200 ms for a tap's answer.
11. **The footer line holds on paper.** The system's print rules keep the footer line, with its
    version and date, on the page. The page check fails a footer line not drawn on paper, one
    printed without its version or its date of issue, and one that can split across sheets,
    each with a canary.
12. **Room above the title.** With nothing over it, the intro stands 32 px under the header,
    one of the system's spacing steps. The tag row over it used to leave 40 px, or 72 px where
    the chip wrapped under the tag on a phone.
13. **The menu bar's title on a row of its own from 720 to 779 px.** In the wide cut, the
    lockup, the product's name and the space between and around them need 217 px. At 720 px
    the default 300 px sidebar and the buttons leave 223 px, which a 15 px scroll bar, as on
    Linux, cut to 208 px, so the right-hand buttons wrapped under too little scroll padding. From about 729 px the row fits even with that bar;
    the title keeps its own row to 779 px as a margin. While the sidebar is shown there, the
    title takes its own row under the buttons, as on a phone, and a heading jumped to clears the
    taller bar.
14. **The sidebar's list fades at its edges.** The list scrolls in its own box, so the
    window's bottom edge sliced its last row in half, as a look at the finished pages showed.
    The box now fades over its last 24 px and its first 10 px, mdBook's top padding there. It
    leaves 32 px after its last entry, the fade's 24 px and 8 px to spare, so the list ends
    whole above the fade, and pads its scrolling 10 px at the top and 24 px at the bottom, so an entry the
    keyboard focuses scrolls clear of the fade. The current page's entry loads clear of the
    fade; after a click in the list, `theme/hpr.js` keeps the clicked entry where it stood
    only if it shows whole and clear of the fade, and otherwise centers it. Under forced
    colors the site's stylesheet drops the fade. The page check reads the fade from the box's
    mask, a gradient running down the box from transparent to opaque and back to transparent
    at its bottom edge, and fails a list that scrolls without one, with less room after its
    last entry than the bottom fade, with less scroll padding than either fade, or with the
    current entry under either fade, a canary for each. An entry the reader scrolls to the
    box's edge mid-list is faded there rather than cut; that is the fade's purpose.

**Superseded.**

- [ADR-164][adr-164]: the docs site's row, "a title block", now reads: a footer line on every
  page of the guide but the print page, the title block on Start here and at the end of the
  print page.
- [ADR-223][adr-223]: §1's capitals only at Cascadia Mono 12/16 and 14/20, and the sidebar in
  Cascadia Mono 14 px: a signal word is in capitals at Archivo 13/16 too, and the sidebar is
  Archivo 15/20. §2's mapping. `h1` and `h2` move from the Cascadia Mono `title` and
  `subtitle` tokens to Archivo's wide cut at the theme's 32/36 and 24/28; `h3` is a heading
  inside a part in sentence case, not in capitals; column heads and Start here's field names
  are Archivo 13/18, not `label` capitals (the print page's block keeps them). §4's "title
  blocks on every page, sheets with their rail, the intro (designation tag, status chip,
  `lead`)" is now the footer, the parts and the intro of a title and its lead.
- [ADR-225][adr-225]: a note's head is Archivo 13/16 in sentence case, not Cascadia Mono 12/16
  in capitals; a caution's and a warning's keep their signal word in capitals, in the wide cut.
- [ADR-229][adr-229]: the title block is the last part of Start here and ends the print page;
  other pages end with the footer line.
- [ADR-232][adr-232]: §3's four preloaded faces are five, the wide cut added for the titles,
  which are no longer Cascadia Mono SemiBold (§10 above). §6's cover and §7's printed
  chapters no longer open with the designation tag.
- [ADR-231][adr-231]: the intro's tag and the sheet numbers are gone, and with them the file
  check's and page check's rules for them; the status chip moves to the footer; the theme
  switch's buttons are Archivo 15/20, not Cascadia Mono 14.
- [ADR-234][adr-234]: the count of two to six is of parts, not sheets (§7 above).
- [ADR-233][adr-233]: the chip is in the footer, in Archivo 13/16 in sentence case, not over
  the title in Cascadia Mono capitals.
- [ADR-235][adr-235]: "two to six sheets" reads "two to six parts", and the trust note sits
  between the lead and the first part. The rest stands.
- [ADR-252][adr-252]: a page arrow's direction word is a label in sentence case, Archivo 13/18
  semibold, not `label` capitals.

**Consequences.**

- A reader sees one line at a page's foot instead of the title block, and no record's marks
  over a title.
- The figures `hpr sim --plot` draws and anything exported as a report stay records with the title block
  ([ADR-339][adr-339] §2).
- The page check's canaries change with the frame: those for the tag and the sheet numbers now
  fail as a record's frame drawn; the chip's canaries check it in the footer. The tests set
  the least number of canaries of the header, the parts, the footers and title blocks, and the
  page on paper at the counts there are: 21, 14, 20 and 19.
- [M0.17b][m0-17b] adds the Field theme and the acceptance pass for the whole restyle.

**The design audit.** Moving the pin reopens the [audit][audit] ([ADR-284][adr-284]).
The audit
read again the new section, *Three levels*, and the 28 sections the change rewrote, at
`dd7552f`; every other row carries over, restated at the new commit. 65 sections apply to
hpr-sim: 32 met, 26 not met, 3 later and 4 that don't apply, where the earlier pin had 64: 37
met and 20 not met. The new level asks more than the frame this record takes off, and the gaps
are filed: the footer line's data date and units, the header's rule before a tool's name and
`data-level` ([#1442][i1442]); the figures' and the plot's words in Archivo with sentence-case
axis titles ([#1443][i1443]); the README's top as a front page ([#1444][i1444]); and the system
theme's table of contents in px, not rem ([#1445][i1445]). The theme switch, set in px here at
first, is in rem.

**How it was checked.** `cargo test -p xtask` (the frame's, the parts' and the page check's
tests, the canary counts, and the audit's pin), `cargo xtask site` at every width in light and
dark, `cargo xtask voice` and `cargo xtask figures`, as the pull request shows.
