# ADR-155: Design checks that match reality: a fit tolerance, caps, and a motor's drawn case (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5c: an internal part wider than its parent's bore warns within ISO 2768-1's coarse general tolerance for the bore's diameter, or as a ring capping its parent's end, and is an error past that. A nominal motor size wider than its mount's bore warns while AeroTech's drawn case fits it, and is an error past that. OpenRocket's examples raise no design error; the private library's errors fall from 13 flights to 8

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8c asks that
"sub-millimetre overlaps and a nominal motor in its matching tube warn, with a cited tolerance;
physically impossible fits stay errors". Before this decision, `hpr_design::checks` compared radii
with 1 nm of slack (`LENGTH_TOLERANCE_M`), and two radial findings were errors:

- `internal_part_wider_than_parent`: a part reaching farther from its parent's axis than the
  parent's bore. It stopped three of OpenRocket 24.12's examples.
  - *3D printable nose cone and fins* has a coupler 24.10 mm across sitting 11.4 mm deep in a
    printed fin can with a 23.19 mm bore: an overlap of 0.46 mm.
  - *Two stage high power rocket* glues bulkheads sized to the airframe's 49.53 mm bore to the
    ends of couplers with a 48.44 mm bore. It also has a thick coupler and its bulkheads 0.20 mm
    wider than the bore they sit in.
- `motor_wider_than_mount`: a nominal 29 mm motor in a 1.140 in (28.956 mm) bore. That is
  OpenRocket's *Chute release* and LOC Precision's motor tube
  ([#280](https://github.com/nrdptel/hpr-sim/issues/280)).

Each of those can be built. Errors are meant for designs that can't exist as described, whose
simulation would be wrong.

No published standard gives a tolerance on motor case diameters. NAR's *Motor Testing Manual*
(v1.5, 2011, §3.5.4) lists the sizes 13 to 98 mm with none, and certification sheets give
nominal sizes. AeroTech's RMS dimensional drawings (RMS-18/20, RMS-24/40, RMS-29/40-120 and
HP RMS-29/120, 2003–2004, archived from `aerotech-rocketry.com` in 2005) give each case's outside
diameter in inches, `.XXX` to ±0.005 in. The PDFs' text gives 0.698 in for 18 mm, 0.938 in for
24 mm and 1.125 in for 29 mm. The rendered drawings show each as the case's outline in the
forward-closure view; the larger figure on each drawing is the aft closure's flange, which stays
outside the mount. The 38, 54, 75 and 98 mm drawings give 1.500, 2.125, 2.965 and 3.870 in.
ThrustCurve.org and RASP files carry the nominal size, which is what hpr holds.

No standard covers hobby airframe parts either. ISO 2768-1:1989 sets general tolerances for
dimensions drawn without their own. Its table 1, class c (coarse), gives ±0.5 mm for 6 to
30 mm, ±0.8 mm for 30 to 120 mm and ±1.2 mm for 120 to 400 mm.

A first version of this decision made only a part through the airframe's outside an error. Review
found it let a ring 1 mm wider than its bore, deep inside its tube, pass with a warning; on the
private library, 11 such overlaps were over 1 mm. It also missed parts copied to a cluster's
tubes. This version replaces it.

**Decision.**

1. **A fit tolerance.** An internal part that reaches past its parent's room (a tube's bore, or a
   nose cone's or transition's largest outer radius) by no more than `fit_tolerance_m` is a fit
   to sand: `internal_part_tight_in_parent`, a warning. The tolerance is ISO 2768-1 class c for
   the room's diameter. A bore made at its upper limit and a part made at its lower one close a
   radial overlap of that tolerance. The standard is for machined parts, and hpr borrows its
   coarse class for hobby parts. Past the tolerance, the part is
   `internal_part_wider_than_parent`, an error as before, measured about its parent's axis as
   before, so a part in a clustered tube is measured in that tube.
2. **A cap.** A centering ring or bulkhead on its parent's axis whose span covers one of its
   parent's end faces, inside or outside, but not both, can be glued against that end. A clustered
   or off-axis tube is not around a ring on the body axis, so a ring there is no cap. Wider than its parent's room past the tolerance, it
   is measured instead against what holds its parent, with the same tolerance. If it fits there,
   it is `ring_against_parent_end`, a warning. A tube is never a cap.
3. **A nominal motor size gets its case's slack.** For 18, 24 and 29 mm, the sizes whose largest
   drawn case (nominal plus 0.005 in) is narrower than the name, `motor_fit_slack_m` is the name
   less that case: 0.1438, 0.0478 and 0.298 mm. A motor of that diameter wider than its mount's
   bore by no more than the slack warns, `motor_tight_in_mount`. Past it, the case can't go in, so
   it is `motor_wider_than_mount`, an error. The other sizes, and any diameter that isn't a
   nominal size, get no slack.
4. **Nothing becomes stricter.** A nominal 38 mm motor in a 38.05 mm bore raises nothing, though
   AeroTech's 38 mm case would not fit it ([#312](https://github.com/nrdptel/hpr-sim/issues/312)).
   A part in a nose cone is still measured against the cone's largest radius, not the radius where
   it sits ([#313](https://github.com/nrdptel/hpr-sim/issues/313)).

**Consequences.**

- `hpr sim` flies *3D printable nose cone and fins* and *Chute release* as saved, with warnings.
  *Two stage high power rocket* raises no design error, and now stops only on its powered
  separation (M4.5g).
- `cargo xtask ork-flights` lists no design error on any OpenRocket example.
  `cargo xtask ork-flights --library` lists errors on 8 flights, down from 13:
  - an inner part past the fit tolerance (`C07/1`, `C11/2` and `/3`, `C12/1` to `/3`), each by
    at least 0.6 mm beyond its tolerance;
  - a motor whose drawn case can't fit (`C06/1`);
  - a fin off its body (`C08/1`).

  `C03`'s five flights only warn.
- A warning still counts the overlapping ring of material twice. Within the tolerance that ring
  is at most 0.5 mm thick at 6 to 30 mm. The 3D-printed example's is 0.39 cm³, 0.48 g of PETG,
  which makes the rocket a little heavier. That is the unflattering side for apogee and the
  flattering side for a waiver's ceiling, small either way.
- Tests pin each edge both ways:
  - A bulkhead 0.8 mm wider than a 49 mm bore warns, and 0.1 µm more is an error.
  - At the coupler's end, a bulkhead is a cap up to the airframe's bore plus its tolerance. It is
    an error inside the coupler, or past that limit. A tube at the end is an error, and so is a
    ring at the end of a clustered or off-axis tube, or one covering both ends.
  - Each band edge of the tolerance table is pinned.
  - A 29 mm motor warns in a 28.956 mm bore. It is an error in 28.702 mm less 1 µm, and in the
    Loft demo's 28.0 mm.
  - A 24 mm motor 1 µm past its slack, 38 and 54 mm motors 1 µm over their bores, and a 28.9 mm
    motor 1 µm over are errors.
- Mutation probes make the tests fail: the slack given to every diameter, the tolerance doubled,
  and a cap allowed anywhere along its parent, off its parent's axis, or over both ends.
- The cases are one maker's. A Cesaroni case wasn't found in a primary source, and may differ.
