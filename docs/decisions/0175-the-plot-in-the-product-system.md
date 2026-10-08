# ADR-175: The plot in the product system's chart style (2026-10-05)

- **Status:** accepted
- **Summary:** M0.6c: `hpr sim --plot` draws in the light theme's color roles of the product system's foundations (Rev A) and no other color; every series is simulated, so dashed in the predicted color, a quantity's size thick and its vertical part thin; events are dotted lines under numbered balloons with a table of every event's number, time, altitude and name; the ground is a chain line; axis titles are `QUANTITY · unit`; no legend box, the line types said once in a caption whose first line, also the SVG's `<desc>`, gives the summary's apogee, its time and top speed. The fall that is not a prediction is hatched, its label outside the hatching. Amends ADR-157 §2, §3 and §5.

**Context.** M0.6c's *done when*: "`hpr sim --plot` draws in token colors only (a test lists
them), every simulated series dashed in the predicted color, events dotted with numbered balloons
and a table, ground a chain line, unit-bearing axis titles, no legend box, and an SVG `<desc>`
giving apogee, its time and the top speed"
([ADR-164](0164-the-fusionspace-product-system.md)). The product system's `product/data.md`
(*Charts*) and `product/foundations.md` (*Semantic roles*, *Data colors*, *Lines and shape*) set
those rules. Before it, the figure ([ADR-157](0157-hpr-sim-plot.md)) drew in the dataviz
reference palette's blue and orange, a solid line for each quantity's size and a dashed one for
its vertical part, with a swatch legend beside each panel's heading, events under dashed lines
listed one marker to a line, and the unpredicted fall shaded gray.

**Decision.**

1. **Colors by role, light theme.** The figure uses eight roles' light values: `canvas` for the
   page, `surface` for the plot areas and balloons, `rule` for gridlines and the table's row
   rules, `rule-strong` for the axes, zero lines, the ground and the event lines, `ink` for text
   and balloons, `ink-muted` for labels, ticks and captions, `ink-faint` for the hatching, and
   `predicted` for the data. A test lists them and fails on any other color in the document; a
   second test holds them to `light.tokens.json` when `refs/fusionspace-design` is checked out.
   The light theme only: an SVG file can't follow the viewer's theme reliably, and light is the
   system's default for anything used outdoors or printed.
2. **All series are predicted.** Everything the figure draws is simulated, so every series is
   Nebula and dashed `8 4`. The two series of the speed and acceleration panels are told apart by
   width, 2 px for the size and 1 px for the vertical part, the line weights the system allows
   for a dashed line; the vertical part is also below zero for most of the descent.
3. **Lines.** Events are dotted (`1 3`, 1 px) under 8 px balloons, stacked in rows as before; the
   altitude panel's zero is the ground, a chain line (`24 3 1 3`, 1 px), with no solid axis
   under it; every other line is solid, 1 px, except the 2 px rule under the table's head.
4. **Axis titles and numbers.** `ALTITUDE · m AGL`, `SPEED · m/s`, `ACCELERATION · m/s²`,
   `TIME · s`, in SI as the rest of `hpr`'s output (ADR-164). Numbers on the figure are for
   reading: a real minus sign and digits grouped from four. Ticks are 1, 2 or 5 times a power of
   ten, now 3 to 6 to an axis: a step whose rounded-out axis would hold 7 takes the next one.
   Everything is set in Cascadia Mono, then fixed-width fallbacks, so the wrap widths hold.
5. **No legend; a caption and a table.** Under the title, a caption gives the summary, then says
   what the dashed, thick, thin, chain and dotted lines mean and what each quantity is. Under
   the figure, a table, `EVENTS · SIMULATED BY HPR-SIM`, gives each event its own row (several
   rows can share a balloon's number), with its time and the altitude in the event's sample.
6. **The description.** The SVG's `<desc>`, named by `aria-describedby`, and the caption's first
   line are one sentence: "Apogee 324.0 m above the launch site at 9.12 s; top speed 65.9 m/s at
   2.73 s." on the guide's example, from the flight summary, with the summary's "in the fall: not
   a prediction" mark on a top speed it marks.
7. **Hatching.** The unpredicted fall is hatched (1 px `ink-faint` at 45°, 6 px pitch) rather
   than shaded, as the system draws uncertain areas, and its label moves above the first panel,
   since hatching never goes behind text.

**Consequences.** The figure no longer looks like a generic chart; it reads as a FusionSpace
drawing beside the docs site (M0.6a) and the CLI (M0.6b). With every series one color, the eye
tells size from vertical part only by width and sign, which the caption states. The dark and
field themes are not drawn; a later UI draws its own charts. The system's other chart rules are
not all met: balloons that would collide stack in rows rather than stepping right along a leader,
the panels' proportions are fixed rather than banked to 45°, and there is no residual panel or
spread band, as `--plot` draws no measured data or Monte Carlo spread.
