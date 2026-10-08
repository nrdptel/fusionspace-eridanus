# ADR-097: A cause in the drag sized by hpr flying OpenRocket's drag (2026-09-28)

- **Status:** accepted
- **Summary:** A cause in the drag sized by hpr flying OpenRocket's drag

**Context.** `C06/1` is the first supersonic flight in either OpenRocket report. Its apogee is
+13.60% and its largest speed +7.98% from OpenRocket's, with neither named cause of ADR-073.
M2.2e4's bar asks for a written, sized cause. The mass is within 0.77%. Taken per component at
zero angle of attack through OpenRocket's public API (a look not kept as a record), OpenRocket's
drag differs from hpr's in two ways:

- **Base drag while a motor burns.** hpr takes the burning motor's cross-section off the aft base
  (Niskanen 2009, pp. 50–51, who cites Fleeman's *Tactical Missile Design* for it; Loft lesson
  L13). OpenRocket 24.12 does not.
- **Supersonic pressure drag.** hpr's is about twice OpenRocket's. OpenRocket gives the tangent
  ogive nose almost none well above Mach 1, and the airfoiled fins about a quarter of hpr's
  (#222).

The two pull in opposite directions. So neither can be named a cause by being present
(ADR-073's point); each needs a size.

**Decision.**

1. **OpenRocket's base rule is measured on public designs.** A new oracle, `base_drag.py`, flies
   every motor configuration of the jar's example designs with nothing deployed. It records
   OpenRocket's base-drag column over Niskanen's whole-base coefficient (`0.12 + 0.13 M²` below
   Mach 1, `0.25/M` above), with thrust and without, and the burning motors' area over the
   reference area. It is committed as `validation/fixtures/ork/openrocket-base-drag.json`. Of its
   56 flights, the 42 of one data branch (nothing separating) are compared; on each, the
   ratio while a motor burns equals the ratio after, to 1e-12. The motors there cover 9% to 94%
   of the reference area, and one flight burns motors in pods. Taking the area off would drop
   the ratio by about that much. On every flight the drag coefficient is the sum of the
   friction, pressure and base columns while a motor burns, so the base column is what
   OpenRocket flies. A test in `hpr_validate` holds all of this. The 14 flights left out
   separate, which changes the base itself.
2. **An opt-in flies OpenRocket's base rule.** `AeroModel::with_full_base_drag_under_power` and
   `Simulation::with_full_base_drag_under_power` keep the whole base while a motor burns, pods
   included; a sustainer lit after a separation keeps the rule. It is for comparisons with
   OpenRocket, not a better model. hpr keeps its documented source's rule by default. Neither rule
   has been checked against a measured flight; #222 holds both questions.
3. **A cause in the drag is sized by hpr flying OpenRocket's drag.** A second oracle,
   `drag_curves.py`, flies each configuration as `flights.py` does, with nothing deployed. It
   records OpenRocket's drag coefficient along the first branch, launch to apogee, as two curves in
   Mach number:
   - power on: the burning rows, each faster than every earlier one;
   - power off: the rows after the last burning one, each slower than every earlier one.

   `cargo xtask ork-flights --library` flies hpr on them as a drag table. It does this for an
   apogee more than 5% off with no other named cause and no separation. It also flies hpr with
   only OpenRocket's base rule. The curves are private values, so the record stays in
   `corpus-out/`. A probe hpr fails to fly is marked failed, and the flight stays compared.
4. **What that shows, and what it doesn't.** Within 5% of OpenRocket's on OpenRocket's drag,
   the metric's cause is "hpr's own drag coefficient". The apogee and the largest speed are each
   judged by their own number. The label is weaker than ADR-073's causes. Those are things hpr
   knowingly does not do; this is the whole drag model. With mass within 1% and OpenRocket's own
   thrust curves, it bounds the net effect of every other cause on that flight by what is left,
   though parts of it can cancel, and the table in Mach number leaves out OpenRocket's
   dependence on the Reynolds number. It cannot tell a deliberate modelling choice from a defect
   in hpr's drag, nor which code is right.
5. **So each flight it names needs its own written breakdown.** The breakdown says which drag rule
   moves the flight, by how much, and what is left open. A list in the library report's tests
   (`DRAG_CAUSES_WRITTEN`) names each such flight with the record that breaks it down. A new flight
   labelled this way fails the bar's test until one is written.

**Consequences.**

- **`C06/1`'s breakdown** (this record):
  - On OpenRocket's drag: apogee +1.11%, largest speed +1.04%, within the bar.
  - With only OpenRocket's base rule: −10.71% and −3.43%. That switch alone lowers the apogee by
    24.3 percentage points; it is measured by its own flight.
  - Switching the rest of the drag to OpenRocket's then raises it 11.8 percentage points. That number is by
    subtraction, and the split depends on which change is made first.
  - A look at OpenRocket's per-component columns on this flight, not kept as a record, points
    the rest to the supersonic pressure drag, with the friction and base close. It is a lead for
    #222, not a measurement the repository reproduces.
- Which supersonic pressure drag is right, and which base rule, is open (#222). Neither code has
  been checked against a measurement on this shape, and the public report has no supersonic
  flight.
- The library run now needs `corpus-out/openrocket-drag-curves.json`, written by the current
  scripts with the pinned jar. The public report is unchanged: none of its flights is more than
  5% off without a named cause.
