# ADR-189: The unstable-under-power flag and four shape warnings; M10.1d2 split in three (2026-10-06)

- **Status:** accepted
- **Summary:** M10.1d2 splits in three: d2 the flag for a flight unstable under power (#335) and the warnings read from the design's shape (#64, #73, #325, #367) with #172's bound raised to 0.1108 calibres; d3 the warnings for separated parts, laminar friction and kinked fins (#18, #179, #326, #354) and the seven unknown signs; d4 the checklist and its `Release` dispatch. Each new warning's condition takes in the unmeasured range next to its measured one.

**Context.** [ADR-188](0188-release-notes-and-the-flattering-side-survey.md) gave M10.1d2 eight
missing warnings, #172's bound, seven unknown signs, #335 and the checklist. Each warning needs a
condition read from the design or the flight and tests on both sides of it, and each condition below
had to be chosen where its issue leaves a range unmeasured. Four of the eight read only the design's
shape; #18, #179, #326 and #354 need conditions read from a flight's separations, its surface and a
freeform fin's outline, and the unknown signs need evidence read issue by issue. Together they don't
fit in one pull request with their review. #335 was closed by mistake on 2026-10-06: a pull request's body
said M10.1d2 "fixes #335", and GitHub closed the issue on that merge. It was reopened.

**Decision.**

1. **M10.1d2 splits in three** (milestone ids take one increment level, so the siblings are
   renumbered): d2 #335, #64, #73, #325, #367 and #172's bound; d3 #18, #179, #326, #354 and the
   seven unknown signs; d4 ADR-162 §2's checklist at a commit on `main` with a `Release` dispatch.
   M10.1d2 keeps its id with a narrower *done when*; M10.1 is met when its increments are.
2. **#335, a flight unstable under power,** is an envelope flag, `unstable_under_power`, not an
   issue warning: it is not a known error in a model but a flight whose path no rigid-body
   prediction holds. It is raised when the least static margin (the weakest plane's, ADR-168)
   from the rail exit to apogee or the first deployment, over the steps where a motor burns, is
   below zero (`FlightSummary::min_powered_static_margin_cal`). A step counts when the thrust at
   its middle is positive: every ignition, thrust-curve knot and burnout ends a step, so a step
   burns throughout or not at all. A margin of exactly zero raises nothing, as the envelope's
   edges are exclusive; a NaN margin or thrust while burning raises it, so a broken number can't
   hide it. Where hpr can give no margin at a powered instant (the net normal-force slope at or
   below zero, or the components' slopes cancelling past `MARGIN_CONDITION_LIMIT`), the
   pitch-moment slope `C_mα` still says which way the air turns the rocket: a second flag,
   `unstable_without_margin`, is raised when the largest `C_mα` there is above zero, so a rocket
   too unstable to have a margin can't pass silently (found in review: a finless rocket with a
   steep tail read `C_mα` +43.9 per radian and raised nothing). A flight raises one of the two at
   most. The flags set no threshold above zero: the margin a flight needs is the RSO's and the
   safety code's call.
3. **#172's bound** is the largest margin gap measured on the private designs, 0.1108 calibres
   (*How far to trust the margin* on the stability page), not the issue's own 0.073.
4. **#64, a forward-swept fin**, warns past Mach 1, as ADR-188 set: the measured rise of up to
   7.7% starts past linear theory's start at Mach 1.2, and the join into it begins below. A
   trapezoid sweeps forward when its tip's leading edge is ahead of its root's; a freeform fin
   when any point of its outline is ahead of the root's leading edge, or any edge of it runs
   outward and forward, as a kinked leading edge does past its kink. A NaN counts.
5. **#73, the subsonic boattail rule**, warns for a boattail steeper than `atan(1/6)`, 9.46°,
   where Niskanen's eq. 3.88 starts to give it drag (`γ < 3`). Its measured over-predictions are
   at 15° and 16°; between 9.46° and 15° is unmeasured and taken in. It warns on any flight, since
   every flight is subsonic for a while. The rule's other error, no drag for a long boattail, is
   on the safe side and doesn't warn.
6. **#325, fin sets at one station**, warns when two or more fin sets on one stage, on the
   airframe or in one pod set, have roots that overlap or touch along the axis, with more than
   four fins between them; four or fewer have no interference factor to miss (Niskanen 2009
   table 3.3). OpenRocket's own rule is unsized between overlapping roots and a common aft
   station; overlap is the wider of the two, so it misses no case the other would warn.
   #64 and #325 are errors in hpr's normal force, so they go with the stability warnings: a
   flight on a drag of its own keeps them, and one on a normal-force table of its own doesn't
   (ADR-181's rule).
7. **#367, a packed part where it can't sit**, names the issue in the design checks' existing
   warning (`packed_part_wider_than_parent`) when its parent has more room anywhere aft of the
   part than the least along it, as a nose cone or a shoulder does: the part could fit only
   farther aft, so its drawn center of gravity is forward of where it can be. A part that runs
   past the parent's aft end is measured at that end. In a tube or a
   boattail the room is no wider aft, the drawn place is no worse than the real one for the
   margin, and the warning stays as it was. The issue's own ask, a bound that turns the warning
   into an error, stays open.

**Consequences.** `CHANGELOG.md` marks these five warned; #18, #179, #326 and #354 stay
unwarned until M10.1d3, and Release 0.1 is not met until M10.1d4. The public-design count of
drag warnings on the validation page was taken before #73 joined the list and says so.
