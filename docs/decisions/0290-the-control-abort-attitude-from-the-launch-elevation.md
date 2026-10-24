# ADR-290: The control abort's 30° measured from the launch elevation, and TP-1353's aileron points to Mach 2.96 (2026-10-24)

- **Status:** accepted; corrects two citations in [M6.4][m6-4] and [M6.5][m6-5] ([issue #690][i690]) and the same abort wording in [ADR-193][adr-193]; neither milestone has started, and no other done-when changes
- **Summary:** The airbrakes and roll surfaces go inert on an attitude more than 30° from the launch elevation, as the IREC student competition's rule §7.3.1 words it, not 30° from vertical. M6.5's supersonic tab check compares against NASA TP-1353's tail-fin ailerons from Mach 1.6 to 2.96, the range of the report's Figure 18 points.

[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-5]: ../decisions-and-roadmap.md#m6-5
[adr-193]: 0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md
[note]: ../research/roll-and-drag-control.md
[i690]: https://github.com/nrdptel/fusionspace-eridanus/issues/690
[i688]: https://github.com/nrdptel/fusionspace-eridanus/issues/688

**Context.** A re-check of the [roll and drag control note][note] ([#688][i688]) found two wrong
source citations in the roadmap entries for M6.4 (airbrakes) and M6.5 (roll tabs and canards).
Inert means the brakes retracted and the roll surfaces neutral.

- The abort rule said "a tilt past 30°", which reads as 30° from vertical. The design, test and
  evaluation guide of IREC, the student rocket competition, says otherwise in §7.3.1, and Launch
  Canada's rule R7.3.1 says the same: the surfaces go inert when the rocket "exceeds 30° from its
  launch elevation". The launch elevation is the rail's angle above the horizon. On a rail canted
  5°, a check measured from vertical trips 5° early on the side the rail leans to and 5° late on
  the other.
- M6.5's done-when cited NASA Technical Paper 1353's tail-fin ailerons at "Mach 1.6–2.86". The
  report's test conditions (p. 5) are Mach 1.60, 1.80, 2.16, 2.96 and 4.63, and its Figure 18
  has aileron points at 1.6, 2.16, 2.96 and 4.63. No point sits at 2.86.

**Decision.**

1. M6.4 and M6.5 put the surfaces inert on an abort, a power loss, or an attitude more than 30°
   from the launch elevation (IREC §7.3.1). ADR-193's list of flight-rule checks reads the same.
2. M6.5's supersonic tab error is published against TP-1353's Figure 18 from Mach 1.6 to 2.96.
   The 4.63 point stays out, as it did before: it is above Mach 3.5, the top of the project's
   range.

**Consequences.** Both milestones keep every other done-when. Three questions are left for each
milestone's own ADR:

- How to read IREC's 30°. The example above takes it as the angle between the rocket's axis and
  the rail's. Read as a change in elevation angle alone, it differs once the rocket tips past
  vertical away from the rail's lean. M6.4's ADR picks one reading and pins it with a test.
- Whether a challenge spec ([M6.3][m6-3], a competition's rules as a file) can select the rule
  of the European Rocketry Challenge (EuRoC) that the brakes extend on an abort (its RQT-0670).
- How to warn of the control-surface vibration the NACA report RM L53I29 measured at Mach 0.54
  to 0.64. That is below Mach 0.7 to 1.25, the band where M6.5 warns of tab reversal and buzz.
