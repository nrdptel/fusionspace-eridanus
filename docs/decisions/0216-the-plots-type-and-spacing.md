# ADR-216: The plot's type and spacing on the scale, numbers kept with their units; M0.9c7 split in three (2026-10-09)

- **Status:** accepted; carries out the product system's `foundations.md` (*Type*, *Space and layout*) and `data.md` (*Numbers*) for the plot's part of #382 (M0.9c7), and splits #382's other items and #400 into M0.9c12 and M0.9c13
- **Summary:** the `hpr sim --plot` figure draws every line at the 12 px `label` size, balloon numerals included, and its title at the 20 px, weight 600 `subtitle` size; its title, caption, balloon rows, event table and indents step on the 4 px scale (8, 16, 24 and 32 px); the time axis is titled at its right end; and a number and its unit are joined by a no-break space in the caption, the notes and the balloons' tooltips. Each is held by a plot test read from the drawn SVG. Colliding balloons on a leader, banking, a title block and the CSV are M0.9c12; the site's figure width and the site's US units (#400) are M0.9c13, both queued after M0.9c9.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md

**Context.** The design audit ([ADR-208][adr-208]) found the plot's balloon numerals at 10 px,
under the product system's 12 px floor; its title at 16 px, which is no type token; its spacing
off the 4 px scale (70, 26, 18 and 20 px steps); its time axis title centred; and a caption that
could end a line between a number and its unit (#382). The same issue lists larger work: balloons
stepping along a leader, banking the panels to 45°, a title block and a CSV beside the figure, and
the site figure's width. #400 asks US units in the site's prose. Together they are more than one
increment.

**Decision.**

1. **Type.** Every line of the figure is 12 px, the `label` size. The title takes the `subtitle`
   token, 20 px at weight 600, cut at 66 characters so that at 0.6 em a character it ends over the
   panels. Balloons grow to a 10 px radius so a two-digit numeral fits inside; a three-digit one
   (a hundred event times) is left to M0.9c12's balloon work.
2. **Spacing.** The title sits 8 px under the margin, the subtitle and caption 24 px apart, the
   lines 16 px apart (the `label` line), the balloon rows 24 px apart and 24 px under the caption;
   the event table starts 32 px under the time axis's title, its head 24 px under its title, its
   rows 24 px apart with their rules 8 px under each line, and a wrapped line indents 24 px. The
   test reads these steps from the drawn SVG, not only from the constants.
3. **The time axis title** is right-aligned at the axis's end, under its last tick, as a drawing
   labels an axis; the panels' titles sit at the start of theirs.
4. **No-break spaces.** The caption's summary, the hatching's note and the balloons' tooltips
   join each number to its unit with U+00A0. The caption wraps only at plain spaces, so no line
   ends between them; a test sweeps every wrap width.
5. **The split.** M0.9c7 ("the plot and the site", done when #382 and #400 are closed with every
   item they list) becomes three increments: M0.9c7 (this), M0.9c12 (balloons on a leader,
   banking, the title block and the CSV) and M0.9c13 (#382 and #400 closed, with every item they
   list). Milestone ids allow one increment level, so the new ones take the next free numbers.
   They are queued after M0.9c9 so that no queued milestone moves later. The parent M0.9c now has
   eleven steps.

**Consequences.** The plot's *Type*, *Space and layout* and *Numbers* rows name the two tests;
each row stays not met for the site. The caption and notes keep the `label` size and line, though
`foundations.md` gives captions the 14 px Archivo `small` token: the figure is set in one
fixed-width face so its columns line up, and a second face waits for M0.9c12's title block. The
records page keeps M0.9c7's new title.
