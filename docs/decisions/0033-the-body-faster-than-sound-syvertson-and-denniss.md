# ADR-033: The body faster than sound: Syvertson and Dennis's second-order shock-expansion method (2026-09-19)

- **Status:** accepted
- **Summary:** The body faster than sound: Syvertson and Dennis's second-order shock-expansion method

**Context.** M1.8a measured the gap (ADR-027): past Mach 3 the Arcas Robin's body alone lifts 3.9
to 4.6 per radian in NASA's wind tunnel (TN D-4014), where hpr's slender-body terms, with body
lift at the plotted angles, give 2.3 to 2.8. M1.8e asks for a cited supersonic method for noses,
boattails and crossflow, and holds the Arcas Robin to 15%. The research for it found that the
measured slopes, fitted over about ±4°, carry crossflow lift the linear term doesn't, and that no
source covers a 15° boattail. Doing all of it, and flying it, is more than one pull request.

**Decision.**

- **Split M1.8e** into M1.8e1, the method as a tested library model, and M1.8e2, flying it (the
  body's terms taking Mach, a join from subsonic, the boattail, crossflow), which carries M1.8e's
  bullet unchanged. M1.8e1's targets were set before measuring: within 0.05 per radian and 0.1
  calibers of TN 3527's own second-order values, and within its stated ±0.2 per radian and ±0.2
  calibers of its measurements (Summary, p. 1).
- **The method: Syvertson and Dennis's second-order shock-expansion method** (NACA TN 3527, 1956),
  the multi-step form, over MIL-HDBK-762's charts (Figs. 5-4 to 5-7, from the RAeS data sheets):
  the charts are carpet plots read by hand with an ambiguous corner, and they stop at an
  afterbody of 7 in their scaled length `l_a/(d√(M² − 1))`, which the Arcas Robin's cylinder
  passes below Mach 2.2 (11.8 at Mach 1.5); the method
  takes any pointed profile, and its report tabulates its own values and its measurements, at
  `α → 0`, for 144 bodies (Tables I and II), public data a test can hold it to. Van Dyke's hybrid
  theory, which Jorgensen (NASA TR R-474, p. 26) recommends faster than sound, is a
  characteristics solution, far more code.
- **Its inputs.** The tip cone by the Taylor–Maccoll equation (NACA Report 1135 eq. 177),
  integrated by classical Runge–Kutta at up to 0.001 rad, the step shrinking so the equation's
  denominator (zero where the flow normal to the rays is sonic) changes by at most 2% per step,
  and a shock angle found by regula falsi to rounding. A fixed step, in the first draft, gave
  slender cones (under about 2°, a tangent ogive's last elements) pressures that jumped and NaN
  (code and physics review); the step is a continuous function of the state rather than an
  error estimate's accept-or-reject, so platforms differ in last bits only. Below 5e-4 rad
  (0.029°) the start is too near the singular line even so (the second physics review measured
  errors of 45% at 0.006° and a wrong `Ok` from a capped run), so there the flow is slender-cone
  linear theory's, blended linearly into Taylor–Maccoll up to twice that angle, and a run whose
  surface misses the cone by over 1e-3 of its angle is an error. Checked against NACA Report
  1135's cone charts, slender-cone linear theory, monotone from 1e-6 rad, smooth in Mach, and on
  the weak shock up to detachment (a third physics review caught the strong shock just under it); Prandtl–Meyer from `afterbody` (ADR-030). The tangent cones' slopes are TN 3527's
  Fig. 2, read by hand at 0° to 24° for Mach 3 to 10, linear between readings, the Mach 3 curve
  held below Mach 3 and the Mach 10 curve above (an assumption M1.8e2 must measure).
- **The report's tangent body**: ten elements per curved piece, tangent at `x/l = 0, 0.1, …, 1.0`
  (footnote 9, p. 15), one per cone or cylinder. With 40 elements per curve in place of 10, no
  ogive-cylinder tried moves by 0.01.
- **Its limit** (p. 13): the exponential relaxation holds only where the gradient behind a corner
  has the sign of `p_c − p₂` (`η ≥ 0`). The report states that as a condition and doesn't say
  how it continued where it fails. hpr's reading: the element becomes the generalized method's
  (which the report says the equations reduce to at `η = 0`), its pressure constant and no
  gradient carried on. A first version carried the gradient on;
  on the fineness-3 ogive from Mach 5.05 it then ran away and the surface flow went subsonic at
  every element count. A second held the pressure but kept the gradient, and grew toward the
  generalized method's value (5.4 per radian, against the report's 2.8) as elements were added.
  This reading converges: 10 and 320 elements agree within 0.008 per radian. An element aft of
  the nose that would need it is refused, since it would carry its loading over any length
  (physics review found a guard on the last element only bypassed by a small boattail, and one on
  cylinders and boattails bypassed by a long shallow flare at Mach 16). The report's range of Mach number over nose fineness, 0.4 to 2, isn't enforced: its
  own Mach 6.28 rows are at 2.09; M1.8e2 decides for flights.
- **Boattails** by footnote 8 (`p_c = p₀`, slope 2), unvalidated here; the Arcas Robin's 57° lip
  is past Fig. 2 and left out.
- **The Arcas Robin's nose** is the secant ogive through its tip and base nearest the report's
  coordinate table (golden-section search on the arc radius): radius ratio 1.744, rms miss
  0.003 in, tip half-angle 10.76°. hpr's committed design keeps its power-series nose (ADR-027),
  whose tangent is vertical at the tip, which the method refuses.

**Result.** Not met, recorded: 75 of 528 comparisons outside the targets. Against the report's own
values: slopes 102 of 144 within 0.05 (−0.134 to +0.146), CPs 125 of 144 within 0.1 calibers
(−0.670 to +0.257). 49 of those misses are where the march stays inside the method's limit.
There a second implementation of the same equations agrees with hpr within 0.0001 per radian on
all 72 cone-cylinders (by the report's Appendix C closed form) and within 0.0006 per radian and
0.0003 calibers on the 60 ogive-cylinders inside the limit. It was written from the paper during
this work: a Python script with SciPy's cone solver, patched to hpr's hand-read Fig. 2, kept in
`refs/scratch/m18e/` and not committed. The same author wrote it, so it can't catch a misreading
both share. So the printed values depart from the equations as read here: the fineness-7 cone
on long cylinders reads high, and the ogives read low, growing with the cylinder. Why is unknown.
The report read its cone pressures from charts; sampling the loading only at tangent points moves
it by 0.004 (physics review). The other 12 are the fineness-3 ogive at Mach 5.05 and 6.28,
where the march reaches the limit near the tip. At Mach 5.05 hpr's CP is 0.19 to 0.67 calibers
ahead of the report, whose measurements agree with it, so the gap is hpr's (validation audit).
No reading tried reproduces both Mach numbers (issue #81). Against its measurements: slopes 117
of 120 within ±0.2 (−0.278 to +0.251), CPs 109 of 120 (−0.540 to +0.328). Six are on the
fineness-7 cone on long cylinders (the report already 0.07 to 0.15 high), three where the report
is itself 0.20 to 0.22 off, four on the fineness-3 ogive at Mach 5.05 (#81), and one at −0.206.
The Arcas Robin's nose and cylinder has no target. Short model: −18.7% to +16.4% (within 5% from
Mach 1.8 to 2.96; −15.0% and −18.7% at 3.96 and 4.63). Long model: −13.7% to −26.4%. The boattail
by footnote 8 takes 0.03 to 0.18 off. The method's slope grows with Mach (2.55 to 3.37) but not
with the longer cylinder, whose measured extra 0.57 at Mach 3.96 goes with its side area. That is
crossflow, M1.8e2's to settle.

**Consequences.** `hpr_aero::shock_expansion` (`ShockExpansionBody`, `cone_flow`,
`cone_normal_force_slope`) is public and flies nothing yet. `cargo xtask aero` writes
`validation/fixtures/aero/shock-expansion.json`, and `shock_expansion::tests` recomputes it and
pins its misses. TN 3527 is pinned in `refs.lock.toml`; its tables are in
`validation/fixtures/aero/tn3527-bodies.json` (a U.S. government work). M1.8e2 has to decide
what M1.8e1 leaves: noses with a blunt or vertical tip (power series, elliptical, Haack), the
boattail, Mach numbers below 3, crossflow at the angles flown, and the join to the subsonic terms.
