# ADR-190: The friction and freeform fin warnings; M10.1d3 split in three (2026-10-06)

- **Status:** accepted
- **Summary:** M10.1d3 splits in three: d3 the warnings read from the design and the drag alone (#18 on every flight on hpr's drag, #326 on every freeform fin set, both at any speed); d5 the separated parts' warnings (#179, #354); d6 the seven unknown signs. Each takes in its issue's unmeasured range.

**Context.** [ADR-189](0189-the-safety-and-shape-warnings.md) left M10.1d3 four warnings and seven
unknown signs. #179 and #354 need a condition read from a flight's separations, and each sign needs
evidence read issue by issue; with their reviews they don't fit in one pull request. #18 and #326
need no flight state: one is a property of hpr's drag, the other of a fin's outline.

**Decision.**

1. **M10.1d3 splits in three** (milestone ids take one increment level, so the new siblings are
   numbered after M10.1d4): d3 #18 and #326; d5 #179 and #354; d6 the seven unknown signs
   (#8, #104, #106, #213, #219, #360, #361). M10.1d4, the checklist, now waits on d5 and d6 too.
2. **#18, fully turbulent friction,** warns on every flight flown on hpr's drag, at any speed and
   on any finish. Its sign is fixed: Barrowman 1967's laminar coefficient `1.328/√R` (eq. 4-2)
   and his transitional one, Niskanen's smooth turbulent one less `1700/R` (eq. 4-6), are both
   below the smooth turbulent coefficient hpr takes (Niskanen 2009 eq. 3.78) for `R` above about
   1e4, so where a run stays laminar hpr's friction reads high and the apogee low. Below about
   1e4 (up to about 8e4 at Mach 3.5) the laminar value is the higher, but the dynamic pressure
   there is near zero, so no output moves. Where Niskanen's roughness limit (eq. 3.80) sets hpr's
   value, Barrowman's model also takes the flow as turbulent throughout, so hpr is never below it
   there either. The condition is unsized: Barrowman says a rough surface stays turbulent
   throughout but gives no roughness at which that starts. So no finish or speed is excluded.
   The warning quotes the issue's one sized case, 3.6% of the friction on RocketPy's Calisto at
   Mach 0.3 (`1700/R` at `R = 1.8e7` over eq. 3.78), pinned by a test as `LAMINAR_FRICTION_PERCENT`. It sits in its own function,
   `friction_issue_warnings`, beside the shape-read `issue_warnings`, and a drag of the flight's
   own meets it no more than the others.
3. **#326, a kinked freeform fin's center of pressure,** warns on every freeform fin set at any
   speed. OpenRocket 24.12 was probed on one kinked outline (the *Pods--airframes and winglets*
   example's wings: hpr's center of pressure 1.6 mm aft, about 0.047 calibres of margin); no
   straight-edged freeform outline was probed, and no outline at another speed, so neither is
   excluded. Trapezoids and ellipses don't warn: M2.2's comparisons cover them.

**Consequences.** `CHANGELOG.md` marks #18 and #326 warned. Every flight on hpr's drag now prints
a `warning: drag: issue #18` line, so the docs' sample outputs gain it. #179 and #354 stay
unwarned until M10.1d5 and the seven signs unrecorded until M10.1d6; Release 0.1 is not met until
M10.1d4. A warning on every flight says less than one with a sharp condition; when an `AeroModel`
option for laminar friction lands (the issue's suggested home), #18's condition can narrow to the
flights that don't take it.
