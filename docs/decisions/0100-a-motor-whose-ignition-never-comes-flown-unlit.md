# ADR-100: A motor whose ignition never comes flown unlit, as OpenRocket flies it (2026-09-28)

- **Status:** accepted
- **Summary:** A motor whose ignition never comes flown unlit, as OpenRocket flies it

**Context.** M2.2e10 asks for 20 designs across the two OpenRocket reports with all five spreads;
M2.2e9 left 19. Of the candidates, the private design `C04` was held back by one reading alone: a
motor set to light at an event that never comes. ADR-076 refused such a motor ("hpr has no motor
that is never lit on purpose"), and with it the configuration. It also read `C04`'s record as
lighting that motor at launch. That was wrong: the ignition at launch in the record is another
motor's, set to `launch`, and the record shows the motor in question never lit and rode with the
stack. The rest wait on larger work: a payload's own drag (#184), two separations (#183), parallel
stages and the pod examples' parts.

**Decision.**

1. **`hpr_design::Ignition::Never`.** A motor set so is placed as a motor out is in every tube
   (`PlacedMotor::fails`): carried loaded, with no thrust, never a burnout that lights another
   motor. That reuses M1.9b's failed tube, whose mass and timing are already tested. The check for
   a cycle of burnouts takes it as lit, as it takes every motor. A sustainer cut for a powered
   separation keeps it unlit.
2. **The `.ork` words that never fire read as `Never`.** `never`; `burnout` or `ejectioncharge` in
   the bottom stage, which has no stage below (`automatic` there still means launch); and
   `ejectioncharge` or `automatic` over a plugged motor (`<delay>none</delay>`). A motor lit by
   one that never lights is written `Never` too, so every reader of the ignitions agrees. A
   separation at the burnout or ejection charge of a stage's own motors that never light never
   comes, and is dropped.
3. **Still refused.** A word hpr does not know; a stage below with no motor (the same event that
   never comes, but not probed) or with motors in more than one mount; a motor below that states no
   delay; a separation at the ignition of a motor that never lights, or at the charge of the
   stage's own plugged motor (not probed); a configuration in which no motor lights, which would
   not leave the pad; and, newly reachable, a separation at launch, since hpr's flight fires a
   separation only once the rocket is off the rod.
4. **Measured, not assumed.** `validation/oracles/openrocket/unlit_motors.py` runs OpenRocket 24.12
   on its *Two stage high power rocket* example's second configuration (an I59WN-P over an
   I357T-14). In three flights the booster never lights (`never`; `burnout` with a `burnout`
   separation; `ejectioncharge` with an `ejection` separation) and the sustainer lights at launch.
   In two the booster is plugged and lit at launch, and the sustainer waits on its charge
   (`ejectioncharge`, `automatic`). In all five, OpenRocket logs one ignition, at launch, no
   separation, one branch, and by apogee the stack has lost exactly the lit motor's propellant
   (0.272 kg or 0.1792 kg, to 1e-9 kg). Two controls: as written, both motors light and the
   booster drops; with the booster not plugged, its charge lights the sustainer 14 s after its
   burnout, which also measures ADR-076's reading of `ejectioncharge`. The record is
   `validation/fixtures/ork/openrocket-unlit-motors.json`, pinned to its scripts' and the jar's
   digests by the test `openrocket_flies_a_motor_whose_ignition_never_comes_unlit`; the reader's
   tests set the same words on synthetic designs and pin `Never`.
5. **Two different motors, on purpose.** The same probe on the first configuration, an H148R-0 in
   each stage, shows OpenRocket's mass column losing nothing while the sustainer burns (#185). It is
   recorded as `h148r-pair-booster-never` and held by the same test, so a change is seen.

**Consequences.**

- `C04/1` flies: apogee +0.88%, largest speed +1.37%, margin +0.0223 calibres, launch mass
  −0.001%, within M2.2e4's bar with no cause needed. The library report has 11 of its 12 designs
  in all five spreads, and with the public report's 9 the reports hold 20: M2.2's bar of 20
  designs is met. `cargo xtask census --accept` records the row.
- `C04/1` is a staged flight, the kind #185 concerns. Its mass at rod
  clearance agrees with OpenRocket's to 0.000% (the report's row). A check run locally on the
  private file, not committed, found OpenRocket's mass falling by each burning motor's propellant
  along the flight, so #185's symptom does not show there; no committed check holds that, and the
  reports screen no flight for #185.
- A separation timed after launch but before the rocket leaves the rod still reads, and the flight
  fires it only as the rocket leaves (#231); only one at launch itself is refused.
- A motor waiting on one that never lights is read `Never` by inference: none of the probes chains
  two.
- `cargo xtask ork`: the reader flies 109 configurations of 170, in 30 designs (108 in 29 before);
  one is left out as *a motor hpr can't light as written* (2 before), a public pod example whose
  stage below has three mounts.
- The public report does not move: none of its configurations had a motor whose ignition never
  comes.
- A design whose only motor is set to light at an event that never comes is still left out, now
  as one in which no motor lights; hpr gives no warning for a motor it flies unlit.
- M2.2e is met (its *done when* is M2.2's, unchanged). M2.2 stays open until M2.2b, whose five
  parts are met, is rolled up.
