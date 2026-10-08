# ADR-041: A lip's shelter is weighed as the drag buildup weighs it, not switched at a threshold (2026-09-20)

- **Status:** accepted
- **Summary:** A lip's shelter is weighed as the drag buildup weighs it, not switched at a threshold

**Context.** Issue #87 lists five places where the body's supersonic normal force is continuous in
Mach but jumps with a tiny change of shape. All five flip the same gate — whether the
shock-expansion method covers the body at all — so each is worth the *whole body*, not the part
that changed. The largest was the lip's: ADR-039 let a lip ride along when the drag buildup's wake
covered it wholly, and the aero rule read that as a threshold, `fraction >= 1`. On the tests'
rocket at Mach 3 and 4°, drawing the lip a ten-thousandth of the boattail's drop taller took a
third off the normal force and moved the centre of pressure 1.77 calibres forward.

**Decision.**

- **The weight is the wake's own share.** `crate::drag::WakeTerm` already grades a lip's shelter
  continuously as it rises out of the wake — all of it up to a quarter of the boattail's drop in
  diameter, none from a half, a straight line between (ADR-030, from TN D-4014 p. 6 and Cubbage).
  The normal force now reads that same number as a weight on the method's share
  (`SupersonicBody::shape_weight`, which multiplies the Mach join's weight) instead of a threshold
  on it. Slender-body theory takes the rest, exactly as it does below the Mach join.
  - The run's *shape* does not change across the band: the lip stays inside the run, carrying
    nothing, and the weight carries the whole body to slender-body theory by the far edge. So the
    set of parts being blended never jumps.
  - **What is borrowed, and what is new.** The band and its grading are ADR-030's, already
    decided and already cited. Using that fraction as a weight on the *normal force* is new:
    ADR-030 fitted it to how much of a lip's own step pressure **drag** the wake removes, not to
    how much of the body's lift the method should carry. The narrower alternative — weighing only
    the lip's own share — is not available, because the lip's method share is already zero and a
    part-method, part-slender-body body is exactly the mixture ADR-034 refuses. So the whole body
    is weighed, which keeps the total an honest convex combination of two admissible whole-body
    models at the cost below.
  - **It removes the jump, not the disagreement.** At one fixed shape — a lip rising a quarter of
    the boattail's drop — the two models still differ by 33.0% of the normal force and 1.77
    calibres of centre of pressure, and nothing measured says which is right. What the band buys
    is that a rocket crosses that difference over its whole width instead of in a ten-thousandth
    of its geometry. As a design sensitivity that is still steep: over the rise band the printed
    force moves 29.1% and the centre of pressure 0.93 calibres (1.25 mm of lip radius), and over a
    gap of one boattail drop in diameter, 33.8% and 1.97 calibres. A dispersion run over a lip
    tolerance will see it.
  - **It is the whole wake fraction**, not the rise alone: the shelter fades with the rise, with
    any tube between the lip and the boattail, and with anything else in the way. Both are pinned.
  - **Two rough edges, recorded.** The weight clamps at 1, so the blend is continuous but has a
    corner there (about −0.95 calibres per millimetre on one side, flat on the other). And with
    two sheltered lips the run takes the smallest fraction, so raising one lip gives the other
    normal force the drag buildup says it does not have; one lip is the only case any committed
    design has, and a rocket with two wants its own decision.
  - **One threshold is left, at the far edge.** A share of shelter under a millionth counts as
    none (`SUPERSONIC_SHELTER_FLOOR`), because weighing the method in at less than that changes
    no number a reader could see and building its table costs half a second — which matters in
    Monte Carlo, where near-edge shapes actually appear. The step it leaves is that millionth of
    the difference between the two models.
- **Nothing measured moves.** Both committed Arcas Robin designs have lips rising 0.17 of their
  boattail's drop, inside the wake's full-shelter quarter, so their weight is exactly 1 and every
  aero fixture is unchanged — checked by `cargo xtask aero --check`.
- **What each remaining switch is worth, measured** (`issue_87s_switches_are_this_big` for the
  four of #87's list, `a_lip_in_a_boattails_wake_carries_nothing` for the lip's two rows). On the
  tests' rocket at Mach 3 and 4°:

  | drawing this | normal force | centre of pressure |
  |---|---|---|
  | a step in radius, past a billionth of the local radius | −8.7% | 1.03 calibres |
  | a flare behind the run, however small | −27.5% | 0.29 calibres |
  | a pointed tip past TN 3527 Fig. 2's 24° | −10.4% | 1.14 calibres |
  | a vertical tip steeper than the cap's handover to its base | −7.0% | 0.64 calibres |
  | a lip leaving its boattail's wake by rising or sitting back (a ramp since this ADR) | −29 to −34% | 0.93 to 1.97 calibres |
  | a lip longer than its boattail's drop in diameter, however little it rises | −33.0% | 1.77 calibres, forward |

  Across the first four the centre of pressure moves **aft**, so a rocket that trips one reads
  more stable. The lip rows go the other way — on that body the boattail takes enough lift off
  that slender-body theory's centre of pressure sits forward of the method's — so the direction is
  the body's, not the switch's; the size is what carries over. The last row is a switch this ADR
  does **not** smooth: shelter requires the lip to be no longer than the boattail's drop in
  diameter, the wake's own scale, and that length is a threshold. It is also the one the tests use
  to take a rocket off the method without changing a radius or an angle.

  M1.8e11 takes the two tips, where NASA SP-3007's cone tables reach 30° and retire Fig. 2's edge.
  The step and the flare are M1.8e12: nothing measures what either carries faster than sound, so
  there is nothing to blend toward and no band anyone can cite. The step's threshold is not the
  coverage gate's millionth of the area but the method's own element layout, which refuses two
  elements parallel but apart by more than a billionth of the radius — about 3e-11 m on the tests'
  rocket.

**Consequences.**

- A lip drawn taller, or set further back, moves a rocket between the two models smoothly, in the
  wake's own proportion; dispersion and optimisation see a slope instead of a cliff, of the size
  above.
- Where the weight is below 1 at every speed, a component's station and its own centre of pressure
  part company — the station blends stations, the force blends slopes and moments — by about two
  calibres at half weight. The moment is not affected: the station is only where a flight samples
  the local flow (`ω × p`), so the cost is an error in the incidence sampled there, of order
  `Δx ω / V` — about 1e-4 rad at Mach 3 and 1 rad/s — which is a fraction `Δx/(x − x_cg)` of that
  component's own pitch and yaw damping: appreciable for a part near the centre of mass, small in
  the total, which the fins dominate. The static margin is untouched.
  The mismatch is ADR-034's and has always existed inside the Mach join; M1.8e10 makes it
  reachable at every supersonic Mach number, and
  [issue #106](https://github.com/nrdptel/hpr-sim/issues/106) records it with its size.
- `SupersonicBody::shape_weight` is new public API, and `SupersonicBody::weight` now includes it.
- Five switches keep their sizes on record, in the guide and in this ADR, until M1.8e11 and
  M1.8e12 close or bound them: the four of issue #87's list, and the lip's own length, which that
  list did not name.
