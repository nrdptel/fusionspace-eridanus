# ADR-038: Blunt and vertical nose tips faster than sound by a Newtonian cap, the method started from the tangent cone (2026-09-19)

- **Status:** accepted
- **Summary:** Blunt and vertical nose tips faster than sound by a Newtonian cap, the method started from the tangent cone

**Context.** The second-order shock-expansion method (NACA TN 3527, ADR-033) starts at a pointed
tip with the flow on a cone. Power-series noses with `n` below 1, Haack series and elliptical noses
leave the tip at 90°, so hpr refused them and their rockets kept slender-body theory past Mach 1:
the committed Arcas Robin designs (whose power-series nose stands in for the model's tabulated
nose with a 0.062-in spherical tip) and the von Kármán noses of Calisto, Cavour, Juno III and
Prometheus 2022 among them. ADR-037 split blunt tips and the lip into M1.8e7 with a lead: NASA
TN D-4865 (C. M. Jackson Jr., W. C. Sawyer and R. S. Smith, 1968) puts a Newtonian cap ahead of
TN 3527's method and compares it with its own tunnel data from Mach 1.50 to 4.63.

**Decision.**

- **The split.** Milestone ids allow one increment level, so the siblings are renumbered: M1.8e7
  is blunt tips, M1.8e8 the lip (with the committed designs' done-when), and the old e8 (issues #87
  and #90, M1.8e's 15% bullet) M1.8e9. e7 keeps "flown with no jump at ±1e-9 in Mach", and takes
  "the Arcas Robin's committed nose (lip left off) through a flight's path in the report" and a
  check against TN D-4865's own sphere-cone; e8 keeps "the committed Arcas Robin designs through a
  flight's path in the report". Like the Arcas Robin's since ADR-036, the sphere-cone check carries
  no target: it shows where hpr stands, set out before measuring.
- **The cap** is TN D-4865's modified Newtonian `C_p = C_p,max sin²δ` (eq. 1, p. 5), `C_p,max`
  from the Rayleigh pitot formula (NACA Report 1135 eq. 100, p. 619). At `α → 0`, in TN 3527's
  loading form (`C_Nα = (2π/A_ref) ∫ Λ r dx`), the wind's slope `δ + α cos φ` gives
  `Λ = C_p,max sin δ cos δ`; a hemisphere then carries its Newtonian drag turned, `C_p,max/2`.
- **The handover** is TN D-4865's (p. 5): where the slope falls to the largest angle a
  two-dimensional wedge turns with an attached shock (NACA Report 1135 eqs. 138 and 168, p. 621 and
  p. 624), which the report chose for its agreement at low supersonic speeds; capped at 24°, the
  steepest cone of TN 3527's Fig. 2, whose slope the method needs. From about Mach 2.1 the cap
  therefore reaches further aft than the report's.
- **The march starts behind it as at a pointed vertex**: the flow on the cone tangent to the body
  at the handover, that cone's loading and no gradient (TN 3527 sketch (a), p. 6), not the
  report's Newtonian pressure and Mach number (eq. 2). Read at `α → 0` as the report's equivalent
  bodies imply (eqs. 4a and 4b: the handover holds still in the wind, so the flow behind it turns
  by `α cos φ` less and its loading is the Prandtl–Meyer flow's `λ/(γM²)`, hpr's reading), the
  report's start, measured in `blunt-tips.json` (`starts`, `sphere_cone`):
  - fails on the Arcas Robin's committed nose from Mach 3.96, where the march reduces the element
    at the nose's end, which hpr refuses aft of a nose (its pressure at the handover lies below the
    tangent cone's at every speed here); since a flight's table is built from Mach 5 down
    (ADR-034), that would leave such a rocket no method;
  - reduces elements from Mach 2.96 (issue #81), so its answer moves with their number (2.544 to
    2.583 per radian at Mach 2.96, 3.361 to 3.418 at 3.5, from 10 to 40 elements);
  - on the report's own sphere-cone, against the measured slope at `α → 0` (fitted both ways,
    ADR-036), reads closer than hpr's at Mach 1.9, 3.95 and 4.63, about the same at 2.96, further
    at 2.3, and 95% high at Mach 1.5, where the handover (12.1°) sits 0.6° above the 11.5° cone
    and the linear range is that small. That is hpr's `α → 0` reading of the report's start; the
    report's method itself, at its own angles, reads 1.844 per radian there.

  The tangent cone's start holds to Mach 5 on the Arcas Robin's nose and moves by under 0.01 per
  radian from 10 to 40 elements. Both stay in the library: `HandoverStart::Newtonian` selects the
  report's, for comparison; a flight takes the tangent cone's.
- **Scope.** A blunt tip is a first segment whose slope is infinite at the tip (power series with
  `n` below 1, Haack, elliptical, and `BodySegment::SphericalCap` in the library for sphere-cones).
  Pointed noses are unchanged. A nose steeper than the handover's slope all the way to its base is
  refused and keeps slender-body theory. The table, the join and its bisected start (ADR-034) are
  unchanged, and so is drag.
- **"In the report"** means, as for M1.8e2, e4 and e6, the committed fixture `blunt-tips.json` and
  the guide's tables pinned to it cell by cell: no validation flight passes Mach 1.2 (Prometheus
  2022 peaks at 1.06), so the whole-flight report is unchanged.
- **TN D-4865's Fig. 8(a)** (model 1, printed p. 101) is read from the report's 300-ppi scan by
  pixel analysis (axes fitted to the labelled grid lines, each circle's centre fitted to its ring)
  into `tn-d-4865-sphere-cone.json`, to about ±0.003; the α = 0 circles read −0.0027 to
  +0.0163, so the plotting itself is good to about ±0.01. It is compared as ADR-036 compares the
  Arcas Robin: hpr's `C_N` at the plotted 0° to 12° (the method's slope with Jorgensen's body lift,
  for a fineness of 1.75, below his Fig. 4's range), fitted with a straight line as the measured
  `C_N` and `C_m` are, the report's own method the same way; the slopes at `α → 0` beside it under
  both curved fits, not judged.

**Consequences.**

- TN D-4865's sphere-cone, like for like: hpr −1.2% to +32.1%, close to Mach 2.3 and high from
  Mach 2.96 (+12.5%, +29.7%, +32.1%), where the report's own method reads +5.2% to +13.6% (−3.3%
  to +13.6% overall); its centre of pressure within 0.06 diameters. At Mach 3.95 and 4.63 hpr's
  slope at `α → 0` is 13% to 21% above the measured, and body lift raises its fitted slope 25% to
  27% above that where the measured curve rises 9% to 16%. At `α → 0` hpr reads −13.5% to +20.5%
  (`α |α|` fit) and −12.3% to +13.6% (`α³`).
- The Arcas Robin's committed nose with the lip left off, like for like: short +37.2% to −4.8%,
  long +19.3% to −4.1%, at or below the fitted secant ogive (+3.4% to +41.0%) at every Mach
  number, by up to 3.8 points to Mach 2.96 and 5.1 to 8.2 past Mach 3, where it reads within 5% of
  the tunnel. Lower is not always closer: past Mach 4 its error changes sign, so at Mach 4.63 it
  reads −4.8% where the ogive reads +3.4%, 1.5 points further out. It is nearer at nine of the
  eleven rows. The committed designs themselves keep slender-body theory for their lip until
  M1.8e8, so M1.8a's short model at Mach 2.96 still reads −16.3%.
- M1.8a (`normal-force-vs-mach.json`): Calisto's von Kármán nose flies the method past Mach 1.2.
  Against RASAero II's potential-flow slope, Mach 1.5 −3.1% to +13.2%, 1.75 −11.7% to +9.7%, 2.0
  −16.8% to +8.8% (now within both targets), 1.3 +12.7% to +16.1% (still outside); 11 of 15 rows
  within both targets (10 before). Against its secant slope to 4°, which carries its crossflow
  lift, 3 of the 11 rows from Mach 0.8 (1 before), Mach 2 −9.2% (−30.6%).
- Unvalidated, and said so: no measurement checks a tip that isn't spherical; Newtonian theory on
  the cap and the tangent cone's start are approximations; #81 still applies behind the handover.
- **A cap that shrinks to nothing doesn't reach the cone it sits on** (issue #101, raised by the
  physics review and pinned by `a_vanishing_cap_does_not_reach_the_cone_it_sits_on`). The march
  keeps its start cone's total pressure the whole way, as TN 3527 does from any vertex, and nothing
  makes that fade with the cap. A power-series nose of `n` = 0.99, a 7.1° cone but for a tip 1e-55
  calibres across, carries 1.21 per radian on its cylinder at Mach 4 where the cone carries 1.37
  (12% less, the body 7%), because the march runs on the 24° cone's total pressure, 107 free
  streams, rather than the 7.1° cone's 151. Fixing it needs a model of the tip's entropy layer
  fading downstream, with a source; the shapes a rocket uses have caps that are small but not
  vanishing, and their share of this bias is unknown.
- **At `α → 0` the handover is held where it sits on the body**, as TN 3527 holds every other
  point; the report's equivalent bodies slide it along the surface instead, and that term is left
  out, its size unmeasured here. The two starts in `blunt-tips.json` differ by more than it alone,
  their pressure and total pressure differing too: 2.53 against 2.87 per radian on the Arcas nose
  at Mach 1.5, 1.702 against 1.700 on the sphere-cone at Mach 2.96.
- **The cap covers much of a slender nose near the join's start:** 59% of the Arcas Robin's nose
  and 48% of a five-calibre von Kármán's length at Mach 1.25, about 5% by Mach 1.5 and under 1.5%
  past Mach 2 (a two-calibre ellipse keeps 13%), against 8% for the report's own sphere-cone at
  Mach 1.5. The join's weight, 0 at its start and 1 a third of a Mach number later, damps it; the
  guide says to read those rows as the blend they are.
- Two switches in shape stay open, of issue #87's family: a vertical tip steeper than 24° to its
  base gets no method, and a pointed tip steeper than Fig. 2's 24° is refused where a vertical one
  flies. Which tangency points merge behind the cap also changes with the handover, so the answer
  steps by about a millionth of a per-radian slope as they do.
- Cost: a vertical-tip rocket builds the supersonic table (Calisto's 363 ms, once per model, only
  once a flow passes Mach 1.2); a subsonic flight never builds it (`docs/perf.md`).
