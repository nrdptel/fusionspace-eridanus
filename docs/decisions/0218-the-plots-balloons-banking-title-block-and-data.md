# ADR-218: The plot's balloons on leaders, banked panels, title block and data file (2026-10-09)

- **Status:** accepted; carries out the product system's `data.md` (*Charts*) and `principles.md` §1 for the plot's part of #382 left by [ADR-216][adr-216] (M0.9c12)
- **Summary:** the `hpr sim --plot` figure sets its balloons in one row, each that would collide stepped right along a short leader, growing for three-digit numerals; each panel's height is banked to 45° by Cleveland's length-weighted average orientation, between 120 and 360 px; the figure ends in an ISO 7200-style title block; and `--plot f.svg` writes the numbers it draws to `f.plot.csv`, with a `f.plot.meta.json` sidecar. Each is held by a test that fails without it.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md
[adr-216]: 0216-the-plots-type-and-spacing.md

**Context.** The design audit ([ADR-208][adr-208]) found that the plot's balloons, when two
instants were close, dropped to a second row instead of stepping along a leader; that every panel
was a fixed 808 × 180 px, so a flight's climb drew steep and its long descent nearly level; that the
figure ended in one line naming the program, not a title block; and that its data was offered
neither as a table of the series nor as a CSV (#382). [ADR-216][adr-216] split these into this
increment, and left three-digit balloon numerals to it.

**Decision.**

1. **Balloons.** One row, in time order. A balloon closer than a step (a diameter and a 4 px gap,
   24 px) to the one before it moves right to a step away, and a thin solid leader runs from its
   circle's edge to its event's dotted line. Where the steps would pass the figure's margin, the
   last balloons step back left from it instead, so each stays whole. Every leader runs down at
   least 35° from level, so it leaves the row between its neighbours, more than a radius from
   their centers; the point where leaders meet their lines moves down, in 4 px steps, as far as
   that needs. Only past a row's capacity (38 two-digit balloons) do balloons take more rows,
   assigned in turn so each row spreads along the axis; rows are painted from the top, so a lower
   row covers the leaders behind it. Past 99 balloons the radius grows 4 px a digit, so a numeral
   always fits inside its circle.
2. **Banking.** Each panel's height is the one at which its drawn lines run at 45° on average,
   each segment weighted by its length: the average absolute orientation of W. S. Cleveland,
   M. E. McGill and R. McGill, "The Shape Parameter of a Two-Variable Graph", *JASA* 83(402),
   1988. It is found by bisection, rounded to the 4 px grid, and held between 120 and 360 px so a
   panel is neither a sliver nor a tower; a panel whose lines are all level takes 120 px. The width
   stays fixed by the sheet. A climb and a slower descent can't both run at 45°; the weighting puts
   them on either side, the longer line nearer. Median-slope banking (Cleveland's 1993 book) was
   rejected: a flight's long descent holds most of the samples, so the median is the descent's
   slope alone and the climb would draw vertical.
3. **Title block.** The figure ends in a box with a 2 px ink border, its cells divided by 1 px
   rules and set in rows across the sheet's text width: OWNER, TITLE, TYPE (`Simulated flight`,
   ISO 7200's document type), DESIGNATION, PROGRAM (name and version), UNITS, DATA (the figure's
   CSV) and MOTOR CATALOG (its date, which moves there from its own line, in a cell of its own so a
   long file name is cut and the date never is). Keys and values are
   Cascadia Mono at 12 px, as `foundations.md` sets title blocks. There is no date of issue: the
   same flight draws the same figure, byte for byte, on any day. There is no status field: the
   drawing statuses (IN PREPARATION, RELEASED) describe a controlled document, not a run's output.
   The `<metadata>` keeps the one-line stamp for software that reads it.
4. **Data.** `--plot f.svg` writes `f.plot.csv` beside it: one row per point the figure draws, a
   header with units in brackets, SI, plain numbers, CRLF lines, as `--export`'s CSV; and
   `f.plot.meta.json`, the same sidecar an exported CSV has. The `.plot` part keeps the names apart
   from a natural `--export f.csv` beside it. A run that reads or writes either name is refused
   before it flies. `hpr sim --json` reports both as `plot_data`.
5. **The caption stays in Cascadia Mono.** [ADR-216][adr-216] left Archivo `small` for the caption
   to wait for this increment. It stays out: a figure shown through `<img>` draws in the reader's
   system fonts, Archivo is not one, and its fallback is an unknown proportional face whose widths
   neither the caption's wrapping nor the figure check (`xtask figures`) can count.

**Consequences.** The figure is taller: the guide's example grows from 1,364 to 1,844 px. Its
altitude and speed panels reach the 360 px bound, where their lines average 39° and 38° (22° at
the old 180 px), and its acceleration panel banks at 252 px, 45°. The audit's *data.md · Charts* row now lists
only the spread band (#401, M2.8) as unmet, and *principles.md · 1* and *web.md · Accessibility*
only the site's parts. The site's figure is still scaled into its 750 px column, so its 12 px text
shows at about 9.4 px; that is M0.9c13, with #400.
