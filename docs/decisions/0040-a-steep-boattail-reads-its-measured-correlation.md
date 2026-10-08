# ADR-040: A steep boattail reads its measured correlation no steeper than 16°, and M1.8e's 15% target judged (2026-09-19)

- **Status:** accepted
- **Summary:** A steep boattail reads its measured correlation no steeper than 16°, and M1.8e's 15% target judged

**Context.** Issue #90 left two things open from M1.8e4 and M1.8e6. hpr gave *any* narrowing
transition the supersonic boattail treatment, though Washington and Pettis measured boattails of
4° to 9.5° and TN 3527's footnote 8 claims only "reasonable results for bodies having moderate
amounts of boattail": a 30° boattail took about a twentieth of the lift slender-body theory removes,
which flatters a rocket's stability. And footnote 8's size was pinned only by its sign and its
continuity, never by a hand calculation. Separately, M1.8e's 15% bullet — the Arcas Robin's
body alone within 15% from Mach 1.5, and both configurations' whole rocket within 15% at Mach 3.96
and 4.63 — had never been judged, since until M1.8e7 and M1.8e8 the committed designs didn't fly
the method at all.

**Decision.**

- **The correlation is read no steeper than 16°.** The drag buildup already treats a boattail's
  flow as separating from 16° (Cubbage's steepest attached boattail, NACA RM L57B21). Past that
  angle hpr stops reading Washington and Pettis's correlation any steeper: a boattail takes the
  increment of one of the **same radii** drawn out to 16°, its centre of pressure staying on the
  real boattail. This is issue #90's cap. It is continuous in shape (at 16° the two lengths are
  equal), so nothing jumps as a boattail is drawn steeper.
  - **Why hold it rather than fade it to nothing.** A fade was written first and rejected on
    review. The increment is negative — it takes lift off the tail — so letting it fade moves the
    centre of pressure **aft** and makes a steep boattail look *more* stable, which is the very
    complaint issue #90 filed: with a fade a 30° boattail removed 26 times less lift than
    slender-body theory, against footnote 8's 21 times that the issue measured. Holding the
    correlation keeps the centre of pressure at the forward end of the honest range. On the tests'
    rocket the two rules differ by 1.35 calibres of body centre of pressure at 30° at Mach 1.5,
    0.91 at Mach 2, 0.75 at Mach 3 and 0.67 at Mach 4.63 — largest at the low speeds a hobby
    rocket flies — and both ends are pinned by a test.
  - **One further bound, on the invented length only.** Reading a longer boattail walks the
    correlation's argument toward zero, where Fig. 5's curve comes from the report's lowest
    supersonic runs and rises past Munk's slender-body line — which RD-TM-68-5 plots there for
    comparison at subsonic speeds (p. 3), not as a supersonic bound. hpr does not invent a length
    and then read that branch, so the *extra* the holding removes stops at potential flow's
    `2 (A_aft − A_fore)/A_fore`. The two reads are equal at 16°, so this adds a kink, never a
    jump (`holding_the_correlation_stops_at_potential_flow`).
    - It bites when the aft radius is under about `1 − √(M² − 1)/1.11` of the fore radius (the
      constant is the curve's crossing, 0.635, over `2 tan 16°`, from a trace read to ±0.0003 per
      degree, so it is good to about a percent): two fifths at Mach 1.2, a quarter at 1.3, a
      twentieth at 1.45, nothing much above Mach 1.49.
    - What that is worth is measured, not argued. `what_the_potential_flow_bound_reaches` sweeps
      boattails of 16° to 53.6° narrowing to between a thousandth and three tenths of the fore
      radius, reads the shares back out of the table, and pins three things: at the table's rows
      a boattail never takes off more than potential flow, except in the sliver where its own
      read already passes it; there, the table carries the correlation as published; and at most
      the bound moves a **printed** coefficient (after the join's weight) by about 0.060 per
      radian, at 53.5° narrowing to a thousandth of the radius, the steepest the sweep found the
      method willing to table; the holdback on the boattail's own cross-section is up to about six
      times that, near the join where the weight is small. Deleting the bound fails that test, and so does clipping the floor.
    - No committed design reaches it: the steepest, Calisto's 18.4°, has an aft radius 0.685 of
      its fore radius against the 0.40 the bound would need even at Mach 1.2.
    - It does **not** bound the correlation itself. A boattail's read at its own length is used as
      published, wherever it sits — a genuinely long one reads the same near-Mach-1 branch
      unbounded (a 4° boattail to 0.6 of the radius reads 1.29 times Munk at Mach 1.5). The
      extrapolation list in the guide therefore names longer boattails as well as steeper,
      shorter and narrower ones.
    - Three things are left open. Where a boattail's **own** read already passes potential flow
      the bound does not clip it — that is the correlation as published — so the 16° hold
      contributes nothing there and the boattail flies an extrapolation nothing measured checks;
      the sweep finds that sliver at 16°, 16.5°, 17° and 17.25°, and pins those; it closes as the
      angle or the speed rises rather than at a fixed angle. The bound
      applies at the table's rows, where the shares are computed, so a printed value between a
      bounded row and a floor row can sit a little past potential flow. And when the bound binds,
      hpr reports slender-body theory's size at Washington and Pettis's station, which for a cone
      differs from slender-body theory's own by up to about a tenth of the boattail's length.
  - **What is not known.** Nothing measures a separated boattail's supersonic normal force, so
    neither limit is validated. The 16° itself is Cubbage's, from transonic *drag* data (Mach 0.6
    to 1.28), used here from Mach 1.2 up, where a shoulder's Prandtl–Meyer turn makes separation
    less likely — so if anything it is early. The guide says all of this where the rule is
    described.
  - The Arcas Robin's 15° boattail is unchanged. Calisto's 18.4° now reads its correlation as a
    16° boattail, which moves its four supersonic rows against RASAero II by under half a point,
    each one closer in slope than before (Mach 2: +8.8% to +8.3%).
- **Footnote 8's size, and the tube behind, are integrated by hand.**
  `footnote_eights_boattail_share_by_hand` integrates eq. 19 —
  `Λ = (1 − e^(−η)) Λ_c + e^(−η) Λ₂`, `C_Nα = (2π/A_ref) ∫ Λ r dx` — over both a conical boattail
  element and the tube behind it, as issue #90 asks, from the flow the method reports at each
  corner, and matches `segment_slopes` to 1e-6. It pins the integration and footnote 8's two
  tangent-cone terms (`p_c = p₀`, `Λ_c = 2 tan δ`), checked against hard-coded values. It does
  **not** pin the decay rate `η`, which comes from eq. 9 and is read from the method; over the tube,
  where `Λ_c = 0`, `η` alone sets the answer, and the tube carries the larger part of the pair. The
  new `ShockExpansionBody::element_flows` makes that flow public, so any such check can be written
  from outside the crate.
- **M1.8e's 15% bullet: half met, half not, and recorded** (`arcas-robin-body-gap.json`).
  - **Met:** both configurations' whole rocket at Mach 3.96 and 4.63, +2.8% to −3.3%.
  - **Not met:** the body alone is outside 15% on six of eleven rows — the short model at Mach 1.5
    (+37.7%), 1.8 (+25.9%), 2.3 and 2.96 (+16.9%), and the long at 1.8 (+19.4%) and 2.3 (+15.5%).
    At Mach 3.96 and 4.63 both are within 5%.
  - **How well the miss can be explained: less than it first looked.** Each fitted slope splits
    into its value at `α → 0` and the curvature the plotted angles add, but the measurement's own
    split is soft: over seven points spanning about ±4.5°, the `α |α|` fit's two terms correlate at
    −0.96, so a low slope forces a high curvature. The measured `α → 0` slopes carry ±0.30 to
    ±0.42 per radian and the curvatures at least as much.
  - **What the split still shows.** At `α → 0` hpr is within 1.5 standard errors of the
    measurement on every row outside the target, so the readings cannot convict the
    shock-expansion method, the Newtonian cap or the boattail's share. On five of those six rows
    most of the gap sits in the curvature, which for hpr is body lift — Jorgensen's crossflow
    term, chosen in ADR-037 on the tunnel's high-angle points, reading too large at a few degrees.
    The sixth row is a counterexample and is recorded as one: on the short model at Mach 2.96, 77%
    of the gap is hpr's own `α → 0` slope, 1.2 times the measured. The worst row, the short model
    at Mach 1.5, also sits at `M/f_n` 0.36, below TN 3527's stated 0.4.
  - **Not tuned.** Closing it needs a cited rule for how the crossflow term grows over the first
    few degrees, or measurements at finer angles than the reports plot. Neither is in hand, so the
    gap stays visible in the guide and in the fixture, as M1.8e's bullet allows.

**Consequences.**

- A boattail steeper than 16° takes the same increment as a 16° one of its radii, so it removes
  more lift than reading the correlation raw would give, and its rocket's centre of pressure sits
  forward of that — the conservative end of a range about three quarters of a calibre wide.
  Calisto's four supersonic rows against RASAero II each move under half a point, all closer in
  slope; its Mach 1.3 row stays outside both targets, as it was, and moves slightly in
  (+16.06% to +15.94%, 0.720 to 0.716 calibres). Their centres of pressure move the other way at
  Mach 1.75 and 2.0, from −0.252 to −0.281 and from −0.410 to −0.443 calibres, the last 89% of the
  half-calibre target: taking more lift off the tail moves the whole rocket's centre of pressure
  forward.
- `ShockExpansionBody::element_flows` is new public API: the state behind each element's corner,
  its tangent cone, how fast the pressure relaxes toward it, and the radius eq. 19 needs.
- M1.8e's bullet is now judged in one place, with a test pinning which rows are outside; M1.8e10
  carries issue #87's model switches, the last of M1.8e's open work.
