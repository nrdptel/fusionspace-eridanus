# ADR-085: Ejected pieces: an airframe that parts at any joint (2026-09-26)

- **Status:** accepted
- **Summary:** Ejected pieces: an airframe that parts at any joint

**Context.** M1.11 asks for a separation at any joint, not only at a stage boundary: an ejected
nose cone, a body section or a payload carried inside, each flown to its own landing under its
own recovery device or tumbling, with triggers as a device's and an optional ejection impulse.
*Done when:* a design that ejects its nose cone and a payload, each under its own parachute, lands
every piece and reports each landing point; the masses sum to the rocket's and momentum is
conserved at each split to M1.7c's tolerances (1e-12 and 1e-9); each descent rate matches the
analytic terminal velocity. What is in hand is ADR-014's separation: one split at a stage boundary
into two bodies, each a point mass under its own devices, with body 0 the nose's, and ADR-074's
powered case, where body 0 flies on as a sustainer on a design cut at the boundary. `hpr-design`
has no notion of a joint below a stage, but its layout places every component with its own mass
(`own`) and its subtree's (`with_children`), and a parent before its children.

**Decision.**

1. **Split the milestone.** M1.11a is the pieces: partings at any joint and around a payload,
   several of them, every body landed; its *done when* is the milestone's. M1.11b is the ejection
   impulse and a tumbling model for a piece that is not a whole stage.
2. **An `Ejection` beside the `Separation`, not a new separation.** `Ejection { trigger, parting }`
   with `Parting::AftOf { component }` (the joint just aft of a body component) or
   `Parting::Payload { component }` (an internal component and its subtree), given by id. The
   separation keeps its type, its API and ADR-074's powered path unchanged. A powered separation
   in a flight with ejections is refused in flight: the sustainer flies on a cut design whose
   components are not the pieces the ejections name.
3. **The pieces are fixed before the flight.** Every joint that can part (the separation's
   boundary and each `AftOf`) cuts the body components, nose to tail, into sections; each payload
   is a piece of its own. Piece 0 is the nose's, the separation's is 1, and ejection `k`'s is the
   next number after those, in the order given. At any moment a body is the pieces still joined,
   numbered by its lead piece: the section nearest the nose among them, or a payload alone. So
   numbering does not depend on the order the triggers fire in, body 0 is always the nose's, and a
   separation-only flight numbers its bodies as before.
4. **A piece's mass is its components'.** A stage wholly in one piece counts by its placed mass,
   with its overrides, which keeps a separation-only flight bit-identical (every earlier test and
   example output is unchanged). A stage that parts counts each subtree that stays in one piece by
   `with_children`, and a component whose subtree parts by `own` plus its children's. An override
   that doesn't say how its mass divides is refused rather than spread: a stage's, or a
   component's that covers what it holds (`overrides_include_children`). So is a payload that is
   one copy of several in a cluster of tubes, an external part, or one inside another payload (the
   numbering handles it, but nothing tests it yet). A motor belongs to its mount's piece.
5. **Each parting adds no impulse.** At the first parting, from the rigid stack, every body starts
   at its own centre of mass with that point's velocity, `v_O + ω × r_cg`, as in ADR-014. A later
   parting happens to a body already flying as a point mass, which has no attitude to place its
   pieces by. Both pieces start at its point and velocity, and its mass steps down; the error in
   position is at most the rocket's length. In both cases the momenta add up exactly, less
   rounding; on the way down that is by construction. Partings that fire in one pass are taken
   one at a time, asking after each which are still the body's: a parting can move another's
   piece to the body that leaves, which then parts it at its start. Review found the first draft
   asking once, which counted a piece twice.
6. **Every motor must have burned out** when an ejection fires, as ADR-014 requires of an aft
   body's: a trigger known before the flight to come earlier is refused when it is given, and one
   that fires early is an error in flight. An ejection before a separation that would light a
   motor is refused too, since the pieces would never light it. So a separation that fires on the
   way down never lights one.
7. **Devices act when their body flies.** As in ADR-014, only body 0's devices act before the
   airframe first parts. After it, a body's devices act on it, and a piece still joined to its
   body waits: its devices' drag area was computed for that piece, not for the body.
8. **Checking devices against bodies.** A builder checks that every body known so far has a
   device. That no device names a body that nothing makes is checked when the flight starts,
   because a separation or ejection given later can still make it, and an ejection's number
   counts the separation. `separations_outside_their_domain_are_refused` now asserts that refusal
   at `run`, with its text and value; the refusal itself is unchanged.
9. **Shock cords are not modelled as such.** Pieces tied together fly as one, which is what not
   declaring an ejection gives.

**Consequences.**

- The milestone's design (`pieces::tests`, and the example `ejected_pieces`) flies the 54 mm test
  rocket with a 250 g payload: nose cone at apogee, payload at 300 m. All three bodies land, each
  within 0.1% of its own `v_e` in uniform air. Masses add to 1e-12 at both partings, and momenta
  to 1e-9, with wind and a 0.6 rad/s body rate at the first.
- `BodyFlight` gains `pieces`; its `mass_kg` is the mass at landing, and a body's events include
  the partings on its way down (`EventKind::Ejection`, or `Separation` if the separation comes
  after an ejection). `SimError::Parting` names the component a refused parting runs through.
- Left for M1.11b: the ejection impulse (a charge or spring's push, equal and opposite on the
  two pieces), and a tumbling model over a piece's components (`tumbling_stages` covers whole
  stages only). Left for later: a sustainer's pieces; the attitude of a body after it parts; the
  `.ork` importer's reading of OpenRocket's own component-level recovery.
