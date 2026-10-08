# ADR-086: Ejection impulse and tumbling pieces (2026-09-26)

- **Status:** accepted
- **Summary:** Ejection impulse and tumbling pieces

**Context.** M1.11b, the rest of M1.11 after ADR-085: an optional impulse, equal and opposite on
the two pieces, and a tumble model over a piece's own components. *Done when:* an impulse gives
each piece the hand-computed change of velocity and conserves momentum to 1e-9, and a tumbling
nose cone lands at its tumble model's terminal speed. Two things were open. A body that parts on
the way down is a point mass with no attitude, so an impulse there has no axis to act along. And
the tumble model (ADR-013, the OpenRocket technical documentation's §3.5) took each body
component's side area from its end diameters, 25% low on Valetudo's tangent ogive nose: for a
lone nose cone that is the whole area.

**Decision.**

1. **An impulse `J` per ejection, N·s,** `Ejection::with_impulse`, default zero and written only
   when set. It is refused unless finite and zero or more. A separation keeps none: its type and
   ADR-014's behaviour are unchanged.
2. **Direction.** The side forward of the joint takes `+J` toward the nose and the side aft `−J`;
   a payload leaves forward, out of its host, as one does when the nose cone comes off first.
   Each body's velocity changes by its push over its own mass, so the momentum is unchanged by
   construction. A payload that leaves aft is not given a way to say so yet, so a push on a
   payload in the nose's own piece, which is closed at the nose, is refused when the flight
   starts (a separation given later can put it in a piece of its own; review found the first
   draft refusing it by builder order).
3. **The axis while the airframe flies whole with nothing open** is its attitude at that instant,
   and the push is added to each body's `v_O + ω × r_cg`. A device that opens in the same pass
   as the parting freezes that same attitude, so it counts. When several splits part the stack
   in one pass, each pushes the two bodies on its own two sides.
4. **Otherwise the body has no attitude to go by**: a point mass after a parting, or a whole
   stack whose attitude froze when a device opened at an earlier time (review found the first
   draft pushing such a stack along its apogee attitude, nearly sideways, after minutes under a
   drogue). hpr then goes by the velocity through the air `v − w`:
   - a body hanging from a device, any but a tumble, open just before that instant (deployed
     before it and not released before it) points its forward end against `v − w`, toward the
     device, assumed to have left through that end, as a main does once the nose cone is off
     (review found the first draft pushing a payload down, away from the canopy it leaves toward).
     What happens at the parting's own instant, a deployment or a release, doesn't count yet, so
     the answer doesn't depend on the inflation law (review found the push flipping with it, at
     a deployment and then at a release);
   - a body with nothing open, or only a tumble, points its nose along `v − w`, as a statically
     stable airframe does;
   - below 1 mm/s through the air (a body's own apogee in still air), up. That speed is drift or
     round-off, not a flight path.

   Nothing in hand says how a body hangs under a drogue, so these are stated assumptions. The
   push itself moves little where every device opens as its piece leaves: in the example, 1 N·s
   at apogee and at 300 m moves the airframe's and the payload's landings by 0.0 m and 1.3 m.
5. **Partings at one instant part a body together,** on the way down as at the first parting:
   which fire and the direction are decided on the body as the pass starts, every firing split
   opens, and each final body takes the pushes of the joints on its sides. Review found the first
   draft taking them one at a time, so that the list order moved the nose cone's push from 15.85
   to 17.66 m/s, and a push could delay a split that fired with it. The events are the parting
   body's, each with the body before and after the whole instant. A pushed payload whose
   section's forward joint hasn't parted by then is an error in flight.
6. **The body after a parting is recorded.** `BodyEvent::after` holds the body just after a piece
   leaves it on the way down (mass without the piece, velocity after the push), so the remaining
   body's change of velocity can be checked. It is `None` for every other event, and not written.
7. **A piece tumbles over its own components.** `Simulation::tumbling_piece(k)` applies the tumble
   model to the body components and fin sets of piece `k`, which leads body `k`. A payload is
   refused, since it has no body tube or fin of its own. So is a piece the airframe doesn't part
   into. Cut into sections, the pieces' drag areas add to the whole airframe's.
8. **The side profile is integrated.** `A_bt` is now `∫ d dx` over each nose cone's and
   transition's own profile, by `hpr_design::revolve`'s planform area. Tubes and cones are
   unchanged. It is a fix to merged physics, not a new model: the documentation defines `A_bt`
   as the side profile area, and the end-diameter reading was a stated approximation. Valetudo
   tumbling moves from 36.77 to 36.38 m/s, and the `.ork` two-stage example's tumbling
   sustainer from 10.6 to 10.4 m/s. The drop-test replay and the two-stage booster are
   unchanged, since they have no curved part.
9. **A lone nose cone is outside the model's fit.** The constants were fitted to whole model
   rockets. The milestone asks that the nose cone land at *its model's* terminal speed, which is
   what the test shows. That the model is right for a nose cone on its own is not claimed.

**Consequences.**

- `pieces::tests` pins the impulse at both kinds of parting. With 1 N·s, in wind, from a stack
  tilted 60° up toward 30° east of north, moving sideways and turning at 0.6 rad/s, each body's
  change of velocity is `J/m` to 1e-9: along the hand-computed rail axis at the first parting,
  15.9 m/s on the 0.063 kg nose cone, and 4 m/s on the 250 g payload at the second, against the
  airframe's velocity through the air. The momenta add up to 1e-9 at both partings. Each
  direction rule has its own test, and so do a separation with a pushed ejection and two pushed
  partings in one pass.
- The tumbling 0.25 m tangent-ogive nose cone of the 54 mm test design has 0.56 times its
  closed-form side area, to 1e-12. It lands at 13.849 m/s, its model's `v_e` to 1e-6.
- Left for later: a payload that leaves aft; the attitude of a body after it parts; an impulse
  on a separation.
