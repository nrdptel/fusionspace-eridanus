# ADR-091: A pod of no length weighs nothing and holds its parts on its axis (2026-09-27)

- **Status:** accepted
- **Summary:** A pod of no length weighs nothing and holds its parts on its axis

**Context.** M1.13b1 left four of the corpus's nine pod sets out (ADR-090). Three are a pod whose
only part is a body tube of no length, no radius and no wall, holding fins (OpenRocket's own
example *Pods--airframes and winglets*, twice in the corpus) or a launch lug (a private design).
OpenRocket names that tube "(phantom body)". The fourth pod set holds nothing at all. `hpr-design`
refused all four shapes: a hollow cylinder of no length, and a pod set with no body component.

**Decision.**

1. **A body tube of no length weighs nothing**, whatever its radius, since it has no wall. Its
   radius and thickness must still be finite and not negative, and the wall no thicker than the
   radius. Only a pod can hold one: a stage still refuses a body component of no length.
2. **What hangs from it sits on a tube of its radius**, as on any tube. With the phantom body's
   radius of 0, that is the pod's own axis: fin roots on it, and a lug's axis its own radius out
   from it. Everything turns with its pod. The pod's widest radius is the tube's.
3. **An empty pod set is read and weighs nothing**, in `hpr-design` as in the reader.
4. **The design checks** treat a pod set of no length as touching its tube when it sits between
   the tube's ends. A part on a pod's tube of no length is never off that tube, nor past its end.
5. **What a tube of no length cannot hold** is left out with a warning, rather than refusing the
   whole design:
   - a part inside it, since it has no room: the pod set is left out;
   - a fin tab deeper than its radius, since it has no wall: the tab is dropped.
   A pod with a nose cone or transition of no length, or a part of negative length, is still left
   out. The corpus has none of these.
6. **An override on an empty pod set is dropped**, with a warning. OpenRocket 24.12 puts that mass
   at the rocket's tip, on its axis, which no design means.
7. **Evidence.** OpenRocket 24.12 was run as an oracle on nine more probes
   (`validation/oracles/openrocket/pods.py`):
   - fins on a tube of no length, four ways: two at 90°; three on each of two pods; two on a tube
     10 mm in radius; and two pods at 30° on a 10 mm tube, three fins each at 20°;
   - a lug turned to 180°, and one at 0°;
   - the empty pod set, with and without an override;
   - a pod set of M1.13b1's kind placed from its tube's `bottom`.
   The probes' numbers are invented. The nine earlier probes came back unchanged.

**Consequences.**

- `cargo xtask ork` reads all 9 corpus pod sets, holding 12 pods, and leaves none out. Cached
  numbers still agree on 71 of 75.
- `hpr_validate`'s tests hold these to OpenRocket on the eighteen probes:
  - every fin root and lug axis in a pod is where OpenRocket puts it, to 1e-15 m;
  - every tube, fin set and lug in a pod has OpenRocket's mass and centre to 2e-15, and a pod's
    nose cone within 1e-7;
  - the roll inertia is within 2e-9, with OpenRocket's fin shortcut (ADR-062);
  - the bare airframe is pinned at 1.17e-7 in mass and −1.125e-7 in centre from OpenRocket's,
    all of it the nose cone, and no probe with pods is further than that.
  - Where pods hold fins, the pitch gap is pinned to three figures, as on the airframe:
    OpenRocket's pitch rule for fins is not known.
- On the 71 designs, those within 0.1% of OpenRocket's mass go from 56 to 57, and within 0.1% of
  its centre from 59 to 62. Roll inertia within 0.1%, with the fin shortcut, goes from 53 to 56.
  Reduced designs go from 8 to 7.
- The private design with the lug pod still does not fly. `openrocket-library-flights.md` lists it
  as "hpr's flight of it failed", not as a reduced airframe. The flight's error names the cause:
  the aerodynamics refuse pods until M1.13c. An empty pod set flies: the aerodynamics and the
  tumble model let it through, since it adds no force and no drag area.
- The RocketSerializer cross-check still leaves out parts in pods (issue #211).
- A lug that writes no angle may be at 180° in OpenRocket, not 0° as hpr reads it. One uncommitted
  probe showed this; it is issue #210.
