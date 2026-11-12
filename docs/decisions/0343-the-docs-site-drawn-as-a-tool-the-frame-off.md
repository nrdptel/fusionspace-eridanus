# ADR-343: The docs site drawn as a tool: the frame off (2026-11-12)

- **Status:** accepted; closes [M0.17a][m0-17a] ([ADR-339][adr-339]'s first increment); supersedes the parts of [ADR-164][adr-164], [ADR-223][adr-223], [ADR-231][adr-231] and [ADR-235][adr-235] listed under *Superseded* below
- **Summary:** The docs site follows the product system at `dd7552f`, the commit that gave it three levels, and is drawn as a tool. Pages lose the designation tag and the sheet numbers; each `##` section is a part under a 2 px rule. Every page ends with a one-line footer: the mark and "FusionSpace HPR", the version, the date of issue, the site's status chip and a link to Start here. Start here keeps the title block as the site's about box, and the print page still ends with it. Headings, labels, navigation, note heads, chips and the theme switch are in Archivo in sentence case at the system's web sizes; CAUTION and WARNING keep their capitals, in the typeface's wide cut.

[m0-17a]: ../decisions-and-roadmap.md#m0-17a
[m0-17b]: ../decisions-and-roadmap.md#m0-17b
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-223]: 0223-the-pages-frame-first.md
[adr-231]: 0231-the-page-anatomy-the-drawing-first.md
[adr-234]: 0234-two-to-six-sheets-the-check-first.md
[adr-235]: 0235-every-page-opens-with-trust.md
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
`dd7552f` on 2026-11-09, says a record (a drawing, a report, an exported plot) keeps the whole
sheet: a designation tag, sheet numbers and a title block at its foot. A front page and a tool
keep what the drawing means (line types, trust notes, signal colors, units) and leave the frame
off. A docs site is a tool. Its page ends with "a one-line footer: mark, version, date,
designation link", its labels are in sentence case, and a level table in `foundations.md`,
*Type*, gives each web element its size: a title 32/36, a section head 24/28, a heading inside
a section 16/24, a label or column head 13/18, a button or navigation 15/20, a status chip or a
note head 13/16, and a signal word 13/16 in the wide cut (weight 640, width 118%) in capitals.

**Decision.**

1. **The pin moves to `dd7552f`.** `theme/` copies the system's mdBook theme, fonts (with the
   new `Archivo-WideSemiBold.woff2` and its `@font-face` rule), icons, card and check script at
   that commit, and `cargo xtask site` compares each with it byte for byte when
   `refs/fusionspace-design` is checked out. `validation/refs.lock.toml`, `theme/README.md`,
   `THIRD-PARTY-NOTICES.md` and the README name it.
2. **No record's frame on a page.** The build draws an intro (the title, then its one-sentence
   lead) and a part for each `##` section: a 2 px rule in the text's color and the section's
   name, with no number. The file check fails a page with a designation tag, a sheet number or
   a sheet's rail in it; the page check fails one drawn, and each has canaries both ways.
3. **A one-line footer on every page but the print page.** In order: the mark, 24 px tall, and
   "FusionSpace HPR"; `Version` and the workspace's version; `Issued` and the date of the
   commit the site is built from; the status chip; and "About HPR Sim", a link to Start here.
   The line is read from the site's title block, so it can't drift from it. The mark is 24 px,
   not the system's 20 px example, because the brand kit's README sets 24 px as the smallest
   the mark is drawn at, and the page check already holds every lockup to it.
4. **The status chip moves into the footer.** The system's tool level puts status "in the
   about box". The site keeps it on every page all the same, in the footer, because the done-when
   names the chips and a page's status is a claim about how far to trust it: *In preparation*
   (outlined in the link color, a square icon) until 1.0, *Released* (filled in green, a
   circle) after. The chip is in sentence case, Archivo 13/16 at weight 600.
5. **Start here keeps the title block as the about box.** Its table (designation, version,
   date of issue, data, units, status) stays where it is, and is the one place a page shows
   the designation. It is a title block, so its status keeps the drawing word in capitals,
   IN PREPARATION; the chip reads it and shows it in sentence case. The footer leaves out the data's date and the units: they're facts about
   the site, not the page, and the footer links to them.
6. **The print page stays a record.** The printed guide is a document a reader keeps, so it
   ends with the whole title block, the page's own title after the site's, as before; its
   cover is unchanged.
7. **Two to six parts a page, as before.** The system's page anatomy asked for two to six
   sheets, and the site's check ([ADR-234][adr-234], [ADR-235][adr-235]) counted them. The
   count still serves a reader, so it stays, for the parts.
8. **Type at the web level table.** Navigation and the theme switch's buttons are Archivo
   15/20; page-arrow labels, column heads and the about box's field names 13/18 semibold; note
   heads and the chip 13/16 semibold, in sentence case. A caution's and a warning's head say
   CAUTION and WARNING in capitals, in the wide cut. The site writes no DANGER note: its kinds
   are the trust note, note, caution and warning, and any other alert fails the build. The page
   check adds the level table's sizes to the system's scale for Archivo, holds capitals to the
   signal word's size, and its canaries cover a signal word in sentence case, one in the narrow
   cut and a trust note in capitals.

**Superseded.**

- [ADR-164][adr-164]: the docs site's row, "a title block", now reads: a footer line on every
  page, the title block on Start here and at the end of the print page.
- [ADR-223][adr-223]: §2's mapping. `h3` is a heading inside a part in sentence case, not in
  capitals; column heads and field names are Archivo 13/18, not `label` capitals; §4's "title
  blocks on every page, sheets with their rail, the intro (designation tag, status chip,
  `lead`)" is now the footer, the parts and the intro of a title and its lead.
- [ADR-231][adr-231]: the intro's tag and the sheet numbers are gone, and with them the file
  check's and page check's rules for them; the status chip moves to the footer.
- [ADR-235][adr-235]: "two to six sheets" reads "two to six parts", and the trust note sits
  between the lead and the first part. The rest stands.

**Consequences.**

- A reader sees one line at a page's foot instead of a six-row block, and no record's marks
  over a title.
- The figures `hpr sim --plot` draws and the exports stay records with the title block
  ([ADR-339][adr-339] §2).
- The page check's canaries change with the frame: those for the tag and the sheet numbers now
  fail as a record's frame drawn; the chip's canaries check it in the footer. The least count
  of each kind the canaries must show moves with them: 13 for parts, 19 for footers and title
  blocks.
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
