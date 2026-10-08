# ADR-098: A tube fin set's automatic radius read as OpenRocket reads it (2026-09-28)

- **Status:** accepted
- **Summary:** A tube fin set's automatic radius read as OpenRocket reads it

**Context.** A tube fin set written `<radius>auto</radius>` asks OpenRocket to size its tubes from
the body they ring (#133). hpr had no rule for it and left the set out with a warning, which made
OpenRocket's *Tube fin rocket* example "an airframe not read exactly as written" (ADR-055). Its
two copies are the only tube fins in the reference library. M2.2e8 asks for the radius as
OpenRocket 24.12 resolves it, and for the example to fly. hpr-aero refuses tube fins until a cited
method exists, so the flight cannot follow from the radius alone.

**Decision.**

1. **Split M2.2e8.** M2.2e8 is now the radius; a new M2.2e9 is a cited aerodynamic method for tube
   fins, so that the example flies with e4's bar on it; the bar of 20 designs moves, unchanged,
   from M2.2e9 to M2.2e10. Between them, the old M2.2e8's *done when* is kept whole.
2. **Measured, not assumed.** `conventions.py` gained 19 probes. On a 50 mm tube: `auto` sets of 1,
   2, 3, 4, 5, 6, 8, 9, 12, 20 and 100 tubes; 6 tubes of a stated 20 mm radius, with and without a
   10 mm radial offset; 12 of a stated radius; 6 `auto` tubes with a wall thicker than their
   radius; and 4 `auto` tubes on a tube whose own radius is `auto`. On a 20 mm tube: 1, 2 and 5
   `auto` tubes. Each probe records the set's resolved radius, wall, count, body radius, centre
   and unit inertias through OpenRocket's public getters. The 107 earlier probes' answers are
   unchanged.
3. **The radius closes the ring.** For `N ≥ 3` tubes on a body of radius `R`, the tubes' axes sit on
   a circle of radius `R + r`, and neighbours touch when the chord between them, `2(R + r)
   sin(π/N)`, is `2r`: `r = R sin(π/N) / (1 − sin(π/N))`. OpenRocket's radius equals it bit for
   bit on every probe in Python, and within 1e-15 in hpr's test (six tubes on 50 mm: 50 mm; four:
   120.7 mm; eight: 31.0 mm). For one or two tubes no finite ring closes, and OpenRocket gives the
   body's radius, on both bodies; hpr does the same.
   `TubeFinSet::closing_radius_m` is the rule, and `AutoDimension::OuterRadius` now applies to a
   tube fin set.
4. **A wall thicker than that radius is cut to it**, so the tubes are solid, as OpenRocket weighs
   them, and as an inner tube's is (ADR-096).
5. **More than 8 tubes are read as 8.** OpenRocket 24.12 reads 9, 12, 20 and 100 as 8, stated
   radius or `auto`. hpr does the same, with a `Dropped` warning, before its own limit of 64 parts
   in a row.
6. **A radial offset moves nothing.** OpenRocket's numbers for the 10 mm offset are those without
   it. hpr already read such a set sitting on the body, with a warning; that stands.
7. **Roll inertia is a departure.** For two tubes or more, OpenRocket's rotational unit inertia is
   larger than `(R + 2r)²`, the most any mass inside the ring can have: 0.3047 m² for six 20 mm
   tubes on 50 mm, against a bound of 0.0081 m². hpr keeps its hollow tubes at `R + r`. For one
   tube, OpenRocket's centre is at `R + r`, where hpr puts it, and its unit inertia is the tube's
   own, `(r² + rᵢ²)/2`, as hpr's is.
8. **Pitch inertia is a second departure.** On all 19 probes OpenRocket's longitudinal unit inertia
   is `N` times one tube's own across its axis, `N((r² + rᵢ²)/4 + L²/12)`, with no term for the
   tubes' distance from the body's axis. hpr's, for three tubes or more, is one tube's own plus
   `(R + r)²/2`, the ring's spread. OpenRocket's is 1.3 to 2.6 times it on the probes (1.77 on six
   tubes of 20 mm radius). On the probes, whose tubes are 0.1 m long, it stays under the bound
   `(R + 2r)² + (L/2)²` for any mass inside the ring; on the *Tube fin rocket*'s longer tubes it is
   18% over it (3.352e-3 m² against 2.834e-3 m²), since `N L²/12` passes `L²/4` once `N > 3`. hpr
   keeps its own. For one tube the two agree; for two, on the probes, OpenRocket's lies between
   hpr's two pitch values, across the pair and along it.
9. **A design with tube fins is weighed, not flown.** `hpr-aero` refuses tube fins, so `hpr-io`'s
   screen leaves every configuration of such a design out as `NotFlown::NoAerodynamicModel`, and its
   stored runs out of the reproduction screen as unflyable. M2.2e9 removes the screen.

**Evidence.** `hpr_validate::openrocket::tests::a_tube_fin_sets_automatic_radius_reads_as_openrocket_does`
holds all 19 probes: each part's mass within 1e-14 of OpenRocket's and its station within 1e-15 m
(the one nose cone, on the probe whose tube takes its radius from it, within 5e-5 and 5e-6 m, the
bore probes' bound), the radius and wall within 1e-15, the count, hpr's roll and pitch against
their closed forms (pitch for one tube and for three or more), and both departures both ways: OpenRocket's pitch as `N` times one tube's own
to 1e-14, its roll past the ring's bound. `hpr_design`'s
`a_closing_ring_of_tubes_touches_the_body_and_its_neighbours` checks the rule by construction.
`cargo xtask ork` now reads both copies of the *Tube fin rocket*: its mass is within 5.0e-6 of
OpenRocket's and its centre within 2.4e-6. Designs within 1% of OpenRocket's mass go from 66 to 68
of 71, and within 1% of its centre from 66 to 68; reduced designs go from 6 to 4. Its pitch
inertia is 1.9800% below OpenRocket's. `cargo xtask ork` swaps OpenRocket's pitch rule into hpr's
structure for each tube fin set (`tube_fin_pitch_shift_kg_m2`, tested on three probes to 1e-12) and
prints the gap left: +0.0023%.

**Consequences.**

- The *Tube fin rocket*'s `[D12-7]` stays unflown in `cargo xtask ork-flights`, now for a reason of
  its own: "tube fins, which hpr has no aerodynamic model for yet". M2.2e9 removes that reason.
  The configurations the reader flies stay 106 of 170, in 27 designs.
- Its roll inertia stays about 99% below OpenRocket's in the mass survey. That is the departure
  above, filed under the survey's existing cause for tube fins.
- The count of 8 is OpenRocket's reading of the file, not a limit of the physics. hpr's own design
  files still take any count.
