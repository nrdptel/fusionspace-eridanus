# ADR-290: The control abort's 30° measured from the launch elevation, and TP-1353's aileron points to Mach 2.96 (2026-10-24)

- **Status:** accepted; corrects two citations in [M6.4][m6-4] and [M6.5][m6-5] (issue #690) and the same abort wording in [ADR-193][adr-193]; neither milestone has started, and no other done-when changes
- **Summary:** The airbrakes and roll surfaces go inert on an attitude more than 30° from the launch elevation, as IREC's rule §7.3.1 words it, not on a tilt of 30° from vertical; and M6.5's supersonic tab check compares against NASA TP-1353's tail-fin ailerons at Mach 1.6 to 2.96, the range of the report's Figure 18 points, not 1.6 to 2.86.

[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-5]: ../decisions-and-roadmap.md#m6-5
[adr-193]: 0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md
[note]: ../research/roll-and-drag-control.md

**Context.** A re-check of the [roll and drag control note][note] (#688) found two wrong source
citations in the roadmap entries for M6.4 (airbrakes) and M6.5 (roll tabs and canards).

- The abort rule said "a tilt past 30°". IREC's design, test and evaluation guide (§7.3.1) and
  Launch Canada's rules (R7.3.1) measure the 30° from the launch elevation: the surfaces go inert
  when the rocket "exceeds 30° from its launch elevation". On a rail canted 5°, a check measured
  from vertical trips 5° early on the side the rail leans to and 5° late on the other.
- M6.5's done-when cited TP-1353's ailerons at "Mach 1.6–2.86". The report's test conditions
  (p. 5) are Mach 1.60, 1.80, 2.16, 2.96 and 4.63, and Figure 18's aileron points sit at 1.6,
  2.16, 2.96 and 4.63. No point sits at 2.86.

**Decision.**

1. M6.4 and M6.5 put the surfaces inert on an abort, a power loss, or an attitude more than 30°
   from the launch elevation (IREC §7.3.1). ADR-193's list of flight-rule checks reads the same.
2. M6.5's supersonic tab error is published against TP-1353's Figure 18 from Mach 1.6 to 2.96.
   The 4.63 point stays outside, beyond the core and extended Mach bands.

**Consequences.** Both milestones keep every other done-when. Two questions the re-check raised
are left for each milestone's own ADR: whether a challenge spec ([M6.3][m6-3]) can select EuRoC's
rule of extending the brakes on an abort (RQT-0670), and how to warn of the control-surface
vibration RM L53I29 saw from Mach 0.54 to 0.64, below the tab band M6.5 warns of (Mach 0.7 to
1.25).
