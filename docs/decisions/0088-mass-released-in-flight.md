# ADR-088: Mass released in flight (2026-09-26)

- **Status:** accepted
- **Summary:** Mass released in flight

**Context.** M1.12b (ADR-087's split of VISION V18) asks for ballast or a payload that leaves the
rocket while the rest flies on in six degrees of freedom, with the mass properties after a release
matching hand-computed values and the release conserving mass and momentum. A separation or an
ejection already parts the airframe, but once it does every body is a point mass (ADR-085); only
a powered separation flies a sustainer on in six degrees of freedom, by swapping the vehicle
under a state that carries across, the nose tip's.

**Decision.**

1. **A `MassRelease` lets one internal part go** (by component id, with everything inside it) on
   the triggers a recovery device has, as a shift is started (ADR-087): one with a trigger known
   before the flight comes then, and the flight watches for the apogee and for a height on the
   way down. The parts a release refuses are a shift's: a body component or an external one, one
   copy of a cluster's, a part that holds a motor, a part in a stage, or inside a component, whose
   overridden mass covers it; and a part released twice or inside another that is released, a
   part with no mass, and releases that would leave the airframe none, its motors aside. It is
   refused on the pad or the rail, where the part has nowhere to go.
2. **The rest flies on by a vehicle swap**, as at a powered separation: the stack's structure
   loses the part (`MassProperties::without_part`, the parallel-axis theorem run backwards), the
   state carries straight across, and the integrator restarts at the same instant. The
   aerodynamics are unchanged: the part was inside the airframe.
3. **The part leaves at the velocity its centre had in the airframe**, `v_O + ω × c`, from where
   it was. Every material point keeps its velocity, so after burnout the rest and the part carry
   the rocket's momentum and angular momentum exactly; nothing pushes them apart. During a burn
   the reported centre-of-mass velocity includes the centre's drift along the airframe, which
   steps at the release, so the reported momenta differ by `Ṁ (cg' − cg)`: stated in the docs,
   not a loss. A spring or a charge that pushes is left for later, as the ejection impulse was
   added to ejections in ADR-086.
4. **The part then falls as a point mass** under a drag area the user gives (`drag_area_m2`,
   positive): a separated body's equations (ADR-085) with a fixed `C_D S`, to the ground or the
   time cap, in `FlightResult::released`. A point mass drops the part's own spin, `I_p ω`; its
   share of the angular momentum is stated with the test. hpr's tumble model needs body tubes and
   fins, so the area is the user's; the docs point to that model's body term, `0.56` of the side
   profile, as a start. A zero area (a fall as if in a vacuum) is refused.
5. **The flight has one apogee.** The rest's centre sits apart from the rocket's, so on a flight
   that turns it can rise for a moment after the rocket's apogee (review found a second `Apogee`
   8 ms later off an 85° rail), or already be falling when a part leaves just before it. Once a
   part has left and an apogee is recorded, the flight stops watching for one; a release that
   leaves the rest falling before any apogee makes the apogee, and fires the devices waiting for
   it, there. A release on the apogee fires on the recorded one, as a device does, and the
   releases are scanned again until no part leaves, so a part waiting for the apogee leaves at
   it however the others move the rest's centre (re-review found a second one riding to the
   ground). Whether the rest is at or past its apogee is judged once per scan, on the stack as
   the scan began, so the parts listed before one waiting for the apogee don't decide it (a third
   review found the list order changing the apogee's mass by 0.23 kg); a height trigger is judged
   there too. Past the recorded apogee the stack counts as descending for a height trigger, a
   device's included: the third review found a main set above the apogee never opening when a
   part let go there set the rest's centre rising, landing at 98 m/s instead of about 7.
6. **A release can land the rest.** A part let go just above the ground, forward of the centre
   of a rocket falling nose up, can step the rest's centre to or below the ground, which a
   crossing from above never sees (re-review found the flight running underground to the time
   cap); the rocket has then landed at the release (a `Recorder`'s last row is the whole rocket
   there, before the part left).
   A rest stepped below the ground while climbing has not landed; that flight is refused. A part
   let go at or below the ground has landed too.
7. **The optimum-delay flight holds a release, or a shift, fired by a motor's delay**, with the
   charge that fires it (review found the optimum delay moving from 13.53 s to 12.62 s with the
   delay flown). Releases and shifts on other triggers still happen in it.
8. **No release with a separation, ejections or mass shifts**, in either order: their pieces and
   parts are fixed before the flight with every part where the design puts it.
9. **`Simulation::mass_properties(flight, t)`** leaves out a part released at or before `t`,
   taking parts out in the order the flight did.

**Consequences.**

- `releases::tests` holds the milestone's bullets. The 54 mm test design dropping its 200 g of
  ballast at 5 s: its mass, centre and inertia after the release match the two-body hand
  calculation (the rest's inertia is the whole's less the part's own and its reduced mass times
  `|L|² E − L Lᵀ`) and the design built without the ballast to 1e-15, and every step after it
  flies the rest's mass and centre.
- In free flight, with no air and no gravity, the rocket turning about all three axes and the
  ballast 1 cm off the axis, the rest and the part keep the rocket's momentum to 1.5e-13 of
  itself and its angular momentum about their common centre of mass to 7.3e-12, over 213 steps.
  The part leaves 0.146 m/s from the rocket centre's velocity: leaving at the nose tip's velocity
  would miss the momentum by 5.0e-3, and at the centre's by 3.4e-3. Its own spin is 1.5e-3 of
  the angular momentum.
- Under a drogue in uniform air, a release on the way down leaves the rest landing at the
  lighter rocket's terminal speed and the part at its own, both to 1e-6 m/s.
- A flight with no release runs the same arithmetic as before: the separated bodies' equations
  moved into `Simulation::point_mass_derivative` unchanged, and the validation report, the corpus
  flights and the real flights reproduce.
- Left for later: a push at the release; a release in a flight with a separation, ejections or
  shifts; recovery devices on a released part.
