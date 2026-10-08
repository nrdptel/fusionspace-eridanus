# ADR-074: Ignition times and powered staging: the sustainer flies on as a rigid body (2026-09-25)

- **Status:** accepted
- **Summary:** Ignition times and powered staging: the sustainer flies on as a rigid body

**Context.** M1.9 asks for stage separation on a burnout plus a delay, a height or a time; sustainer
ignition; the booster flown through its recovery; clusters in one mount with their thrust offset;
and a two-stage and a cluster design each within the per-case tolerance of OpenRocket. What was in
hand: every motor lit at `t = 0`; a separation (ADR-014) allowed only after the last burnout, with
both bodies descending as point masses; and ADR-014's warning that a body flying on would need "an
aerodynamic model for a headless stack". Loft staged before the flight, so an apogee or height
separation fell back to the burnout and no booster was flown (L30); its tests held the sustainer's
ignition time, the mass step and a trigger that never lights (L93). That is more than one pull
request.

**Decision.**

1. **Split into a to c.** M1.9a: ignition times and powered staging (L30, L93), and the parent's
   event-ordering bullet. M1.9b: several motors in one mount and a motor out (L31). M1.9c: the
   `.ork` ignition and separation settings and clusters flown against OpenRocket, the parent's
   first bullet. Each has its own *done when* in `ROADMAP.md`; the parent's is unchanged.
2. **Ignition belongs to the configuration.** `MountedMotor::ignition` is `Launch` (the default,
   and omitted from a design file, so older files read the same), a `Time` after launch, a `delay`
   after another mount's motor's `Burnout`, or a `delay` after the `Separation` that frees the
   motor's stage. Times are on the flight's clock (`t = 0` at launch). Each motor burns on its own
   clock from its ignition, and one not yet lit, or never lit, is carried loaded. Every time but a
   separation's is known before the flight and is a stop time, with the curve's knots shifted by
   it. Assembly refuses a burnout of a mount holding no motor, a chain of burnouts that returns to
   itself, and a time or delay that is negative or not finite.
3. **A powered separation keeps the nose body flying.** When the separation fires and body 0 still
   has a motor to burn (burning, due at a known time, or lit by this separation), body 0 is the
   sustainer and flies on in six degrees of freedom. The flight's state is the nose tip's
   (ADR-011), which the sustainer keeps, so the state carries across unchanged. The integrator
   restarts there, because the mass steps down by the booster's. Its aerodynamics are hpr's own, on
   the design cut after the boundary, built only when a separation is powered, so an unpowered one
   never needs it: the sustainer keeps the nose, so it needs no headless model. Only a body
   without the nose would, and that body is the booster, which descends as a point mass under its
   own device as in ADR-014. The booster's motors must be spent: a separation while one burns or
   is still to light is refused, before the flight when the time is known and in flight
   otherwise. **A device on the booster must be open at the split.** In the first draft the
   example's booster, its tumble set to its own apogee, left at 341 m/s and coasted drag-free: it
   landed at 350.18 s, against 47.21 s with the tumble open at the split, having climbed far above
   the sustainer. A body-1 device with `Time { time_s: 0.0 }` opens there, since a body's
   devices act only once it flies. hpr tumbles the booster side-on (ADR-012) from the instant it
   separates, while a real finned booster flies nose-first for a while, so from near Mach 1 it
   stops almost at once (the example's peaks 102 m above the split). That is likely low, and #179
   holds a model of the booster's own drag.
4. **An unpowered separation is ADR-014's, unchanged.** If body 0 has no thrust to come, both
   bodies descend as point masses. So a sustainer that separates after its own burnout, in free
   flight, still falls without airframe drag until its device opens. That is ADR-014's limit, not
   a new one, and it is kept rather than changed here, because it is what every existing
   separation test flies.
5. **No impulse.** The sustainer keeps the nose tip's velocity and rotation; the booster leaves
   with its own centre of mass's velocity, `v_O + ω × r`. With the sustainer unlit and the booster
   spent, the two momenta add to the stack's (a test, to 1e-9). With the sustainer burning,
   `m v_cg` is not additive, because the motion of the centre of mass inside the body carries mass
   flow terms, so no such claim is made.
6. **Triggers and events.** `Trigger::Burnout { motor, delay_s }` fires a delay after a motor's
   burnout, for separations and devices alike. A trigger on a motor that never lights never fires.
   The height trigger stays ADR-012's (descending); an ascending one waits for M1.9c, if the `.ork`
   settings need it. `EventKind::Ignition(i)` records a motor lit after launch. `Burnout` is
   recorded when no motor is burning or due to light at a known time, and again if one lights
   afterwards, as a sustainer lit by its separation does. So the same staging planned two ways
   (lit at the booster's burnout plus 1 s, or at a separation 0.5 s later plus 0.5 s) records one
   `Burnout` or two, and `FlightResult::event` returns the first. A per-motor burnout event is
   left for when a report needs it.
7. **Refused rather than guessed.** A drag or normal-force table is the whole stack's, so a
   powered separation under one is an error. So is a powered separation after a device opened on
   body 0, which the point-mass descent can't fly under thrust. A separation timed from a motor
   with no ignition known before the flight (the sustainer it lights, say) could never fire, and is
   refused when given; so is a separation ignition in the last stage, which nothing can free.
8. **Found on the way.** A body whose device opened on the stack before the separation was refused
   as reaching the ground with nothing open: the check read only the body's own events. An apogee
   separation with an apogee parachute on the sustainer hit it. The check now reads the devices'
   state. The test that pins the refusal (`a_body_whose_device_never_opens_is_refused`) passed
   only because of that bug. Its booster's trigger, a height above the body, fires at once on a
   body already descending below it (the altimeter rule). It now uses a time after the landing,
   and asserts that the booster is the body refused. Review also found the stack's apogee event
   firing a booster's apogee charge while the booster was still attached; it now fires only the
   stack's own devices, and each body finds its own apogee.

**Consequences.**

- M1.9a's bar is met by tests in `hpr_sim::staging` and `hpr_design::config`. They cover the
  sustainer's ignition at the booster's burnout plus its delay (to 1e-12 s), and the mass stepping
  down by exactly the booster's and holding until the sustainer lights. They cover a separation
  that never fires, leaving the sustainer unlit and loaded to the ground. They cover the event
  order (liftoff, rail exit, separation, ignition, burnout, apogee, landing), an apogee and a
  height separation firing in flight with both bodies landing, an air start at 3 s, linear
  momentum on a rail 10° off vertical, and each refusal.
- The staged flight is not validated. M1.9c compares it with OpenRocket; until then the staging
  page says so.
- Not modelled: the booster's own airframe drag (#179), an ejection or separation impulse, the
  flow between separating bodies, a motor's ignition transient beyond its curve, and the booster's
  attitude (ADR-014).
