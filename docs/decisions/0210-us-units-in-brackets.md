# ADR-210: US units in brackets on the command line; no units switch and SI exports until the app (2026-10-08)

- **Status:** accepted; carries out [ADR-164, the product system][adr-164] §6 for M0.9c2 (#378), and declines two of `data.md`'s rules for the command line until M9.1
- **Summary:** `hpr sim`, `hpr mc`, `hpr motors show` and the plot give US units in brackets after the SI value, as `hpr analyze` already did: feet and feet per second for heights and speeds, mph for wind, inches for the CG and CP stations and a motor's size, ounces or pounds for masses. The plot gains a right-hand US scale on each panel and a feet column in its event table. `hpr motors show` gives the class's impulse range, the nominal thrust beside the measured one where they differ, and the propellant's name where the catalog has it. The command line gets no `--units` switch, and the CSV and JSON exports stay SI, until the app's units control (M9.1). Other readouts that still print SI alone are #392.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-079]: 0079-exports-as-text-built-in-the-core-heights-on.md
[adr-208]: 0208-the-design-audit.md

**Context.** [ADR-164][adr-164] §6 kept the command line SI first, with US units in brackets
(`390.1 m (1280 ft)`), instead of `data.md`'s US-first default. Only `hpr analyze` did that. The
design audit ([ADR-208][adr-208]) found `hpr sim`, `hpr mc` and the plot printing SI alone, and
the landing page claiming otherwise (#378). `data.md`'s *Units for rocketry* also asks for one
control that switches units everywhere (`--units` in a command line), and its *Files and
exports* asks CSV files to be SI with an option for US units.

**Decision.**

1. **Brackets, everywhere `hpr sim`, `hpr mc` and `hpr motors show` print a quantity of
   `data.md`'s table:**
   - heights and distances in feet, speeds in feet per second, wind in mph;
   - the CG and CP stations and a motor's size in inches;
   - masses in ounces, and in pounds from one pound;
   - thrust and impulse stay N and N·s, as the table says.

   The rail's length is given in feet (`1.5 m (4.9 ft)`), since launch rails are sold and
   named in feet. Terminal output keeps its numbers ungrouped (`cli.md`). One helper,
   `crates/hpr-cli/src/units.rs`, formats them all, with exact conversion factors; its tests
   pin rounding at a half, signs, zero and NaN.
2. **The plot gives both scales:** SI on the left, US on the right, each with three to six round
   ticks, and the event table a feet column. The figure check (`xtask figures`) holds both to
   the no-clipping rule.
3. **No `--units` switch on the command line yet** (chosen over adding one now). Brackets give
   both units at once, so a flyer reads feet without choosing. A switch that "applies everywhere
   at once" would have to reach the exports and the JSON, whose names carry their SI unit. The
   app's units control (M9.1, US by default as ADR-164 §5 says) is where one control fits.
4. **The exports stay SI** (chosen over a US option now). The CSV's and JSON's column names, such
   as `height_above_ground_m`, are a published format ([ADR-079][adr-079]) that scripts read; a
   spreadsheet converts in one column. The CSV header's units in brackets are #380's, a separate
   rule.
5. **What is left** goes to #392, for M0.9c4: `hpr weather`, `hpr motors list`, `hpr analyze`'s
   acceleration in g, the `note:` lines (whose words are also the JSON's notes) and the site's
   prose.

**Alternatives considered.**

- **`--units us|si` now.** Rejected for the reasons in 3: a switch that leaves the exports in SI
  is two controls, not one.
- **A US option for CSV alone.** Rejected: it forks a published column format for a conversion a
  spreadsheet does in one step.

**Consequences.** The audit's *data.md · Units for rocketry* row stays not met, for #392 only,
and its *Files and exports* row for #380. When M9.1 brings the units control, this record is
revisited for the command line and the exports together.
