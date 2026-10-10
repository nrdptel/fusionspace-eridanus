# ADR-226: The gap to OpenRocket split by cause before M1.14 starts (2026-10-10)

- **Status:** accepted; adds M2.3c6 ahead of [M1.14][m1-14], toward [ADR-209][adr-209] §7
- **Summary:** on the 55 logged flights of the private collection, FusionSpace HPR's apogees miss the logs by a mean absolute 13.96% and OpenRocket 24.12's by 13.08%. Nothing says which of its inputs makes up the 0.88-point gap. M2.3c6 splits each flight's difference into thrust, mass and drag parts before M1.14 tries to close it.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-220]: 0220-the-margin-bar-tightened-and-m1-14-held-to-release-0-3.md
[m1-14]: ../decisions-and-roadmap.md#m1-14

**Context.** A planning review on 2026-10-10 read the committed reports. The accuracy numbers have
not moved since 2026-10-08: the work since then was the documentation site's design. On the
private collection's 55 logged flights (aggregates only), hpr's apogee is +9.83% from the logs on
average with a mean absolute error (MAE) of 13.96%; OpenRocket 24.12's is +9.00% and 13.08%.
[ADR-209][adr-209] §7 asks hpr's MAE to be 0.3 points below OpenRocket's; it is 0.88 points above.
[ADR-220][adr-220] held [M1.14][m1-14] to that target.

M1.14 shrinks errors model by model, and [ADR-143][adr-143]'s stop rule ends it after two
increments that move no measured error. Without knowing which input carries the gap, its first
increments are guesses, and two wrong guesses stop the milestone with the gap still open.

**Decision.** Add M2.3c6, queued after M2.3c3 and before M2.7 and M1.14. For each flight both
simulators fly, the fixture report gives hpr's apogee less OpenRocket's and splits it in a fixed
order: fly hpr with OpenRocket's thrust curve, then also its mass and center of mass over time,
then also its drag coefficient against Mach. Each step's change in apogee is that input's part;
what is left after the third is "other" (stability, launch, atmosphere, integration). Because the
steps are taken one after another, the parts add up to the whole difference exactly.

The report gives the parts as aggregates: each part's mean and mean absolute value over the
flights, and how many flights each part dominates. Order matters for interacting inputs, so the
report states the order and gives the drag-first order beside it for the drag part.

**Consequences.**
- M1.14's first increment can name the part it shrinks and the size it can at most remove.
- It adds no user-facing surface: it is a report section, so the limit of three surfaces in
  progress is not touched.
- It depends on OpenRocket's exports holding the three inputs at its flight's own times; where an
  export lacks one, that flight is listed as not split, with the reason, not dropped.
- No done-when is loosened and no release moves: Release 0.3's target is unchanged.
