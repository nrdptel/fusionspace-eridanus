# ADR-062: Fins and rail buttons against OpenRocket; roll inertia explained (2026-09-21)

- **Status:** accepted
- **Summary:** Fins and rail buttons against OpenRocket; roll inertia explained

**Context.** M2.2b2 (ADR-061) carried seven things: clusters, fillets, airfoil fins, elliptical
fins, the parts kept unread, a rail button's place (#151), and a roll inertia a median 2.1% from
OpenRocket's that nothing explained, even on Loft's public designs, whose mass, centre of mass and
pitch inertia agree within 0.1%. That is more than one pull request.

**Decision.**

1. **M2.2b2 splits.** b2 takes the fins, the rail button and the roll inertia. A new b3 takes
   clusters, fillets, packed parts that OpenRocket sizes itself, and the parts kept unread. The old
   b3, stored results (Loft lesson L87), becomes b4. Every item of the old *done when* is in b2's
   or b3's.
2. **Measured on probes.** `validation/oracles/openrocket/conventions.py` gains 33 probes, each a
   tube and one part: fin sets of every outline, one fin, sections, a tab, fillets and a cant;
   rectangles of two chords, two spans and on two tubes; every attached and packed part; rail
   buttons, one or a row of two, from each end. The tube alone is OpenRocket's, so a probe's roll
   inertia less the tube's is its part's. `hpr_validate::openrocket::tests` holds hpr to each or
   pins the gap.
3. **Roll inertia is the fins, and it is explained.** A bulkhead, centering ring, inner tube, mass
   component, parachute, shock cord and streamer are OpenRocket's in roll inertia to 1e-15; a rail
   button is apart by up to 1.4e-5 of its probe's. A fin set is not. OpenRocket gives two or more
   fins their mass times `R² + R hₑ + hₑ²/3`, with `hₑ² = A h / c_r`: the roll inertia of a thin
   rod from the body at radius `R` out to `R + hₑ`, where `A` is one fin's area, `h` its span and
   `c_r` its root chord; one fin, the same rod about its middle, `m hₑ²/12`. This is **inferred
   from OpenRocket's output**, each probe less its tube, not taken from its source. It holds to
   1e-12 on every fin probe but two: an ellipse (OpenRocket's is a 30-sided polygon) and a cant
   (−2.66e-5). A tab's mass takes the planform's value, and neither the section nor the thickness
   enters. Every non-rectangular probe has a span half its root chord; the Loft demos (span 0.39 to
   0.50 of the root) hold to 5e-6 as well. hpr keeps its own, the exact integral of `r²` over the
   fin: **a departure**, pinned per set. The two agree on a rectangle without a tab, bar the
   thickness term OpenRocket leaves out (+0.013%). They part on a tapered fin (hpr +2.41% on the
   probe's trapezoid, −2.14% on its triangle) and more on a tab, whose mass the rod puts out with
   the fin's though it lies 40–50 mm from the axis (−5.12%).
   `hpr_validate::openrocket::openrocket_fin_set_roll_kg_m2` states the rule.
4. **Fin sections: hpr's own, pinned.** OpenRocket weighs a fin set as outline × thickness ×
   a factor: 0.99 for a rounded section and 0.85 for an airfoil, at 3 and 6 mm alike. hpr
   integrates the section: a semicircular edge (0.9914 of the slab on the probe) and NACA's
   four-digit section (0.6851, Abbott and von Doenhoff). A file saying `airfoil` does not say which
   airfoil. hpr's is a cited shape, and a builder who weighed the fins overrides the mass anyway.
   So hpr keeps its sections, a departure: its airfoil fins are 19.4% lighter. The elliptical fin
   stays the exact ellipse too (OpenRocket's polygon is 0.18% lighter).
5. **A rail button's place is OpenRocket's (#151).** OpenRocket gives a button no length. It
   puts the button's centre where a part of no length would sit, from whichever end the file
   measures, and a row's first button there, the rest following aft. `hpr_io::ork` now moves the
   offset so hpr's button, placed by its forward edge, has its centre there. The probes check the
   top, middle and bottom, one button and two. After and absolute are read the same way, unprobed;
   a part placed after a button still follows hpr's button, which has a length.
6. **How the survey says it.** `cargo xtask ork` prints the roll inertia with OpenRocket's rule in
   hpr's place, on OpenRocket's own mass for each fin set: paired by id, or in an older file with
   no ids by the set of the same name nearest in mass (within 30%). A file outside 1% needs a
   cause, or the survey fails: a mass override covering the parts inside (ADR-061's departure), a
   reduced design, a fin set with nothing to pair, or packed parts hpr weighs as point masses,
   counted only for a gap of their sign and no larger than they could add in a packing as wide as
   their tube.
7. **Findings left for b3, each pinned.** A parachute that writes no packed size is packed 25 mm
   long and 12.5 mm in radius by OpenRocket; hpr reads no radius, with a warning. A mass override
   on a packed part that weighs nothing is spread over the packing by OpenRocket (`m r²/2`) and is
   a point mass in hpr (−0.805% on the probe). Fillets' mass (0.81% of the probe's structure at
   5 mm) is still left out. Untraced and small: a cant's mass (−4.19e-5 on the probe's structure),
   fins' pitch inertia (up to 0.41% of a probe's, on one fin), a lug's pitch (3.1e-4), and a rail
   button's inertias (up to 5.0e-4). A hypothesis for the last two, to test in b3: OpenRocket
   leaves out the off-axis term in pitch.

**Consequences.** On 2026-09-21 (`cargo xtask ork`, OpenRocket's record unchanged):
- **Roll inertia with OpenRocket's fin rule in hpr's place:** median 0.001% (hpr's own: 2.112%);
  within 0.1% on 49 of 74 (was 11) and within 1% on 55 (was 29). Five of Loft's six public demos
  come within 5e-6. The sixth, `demo-boattail.ork`, is 0.093% apart; with OpenRocket's polygon
  ellipse it is 1.5e-6 (a test).
- **The 19 files still outside 1%** (15 by content) each have a cause. By content: a mass override
  covering the parts inside 7, a reduced design 6, packed point masses 3; some have two.
- **Mass and centre of mass** unchanged: 61 and 62 of 74 within 1%. The rail button moved no file
  across a threshold.
- **hpr's own numbers change** only where a `.ork` rail button sits, and so the rail guide a flight
  leaves the rail by. How far depends on the end the file measures from (r is the button's radius,
  s the spacing, n the count):
  - from the top, after a part, or absolute: forward r;
  - from the middle: aft (n − 1)s/2, so one button does not move;
  - from the bottom: aft r + (n − 1)s.

  A row placed from the middle or the bottom now leaves the rail later.

---
