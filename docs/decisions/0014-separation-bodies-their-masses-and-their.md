# ADR-014: Separation: bodies, their masses and their descents (2026-09-17)

- **Status:** accepted
- **Summary:** Separation: bodies, their masses and their descents

**Context.** M1.7c has to fly every body of a separated rocket to its own landing, with its own
mass properties and drag. What is in hand:

- The descent phase (ADR-012) is a point mass: it needs a mass and a drag area, and deliberately
  has no airframe aerodynamics.
- `hpr-design` carries `MassProperties` per stage (`Layout::stages`) and a stage index on every
  placed motor, so a body's mass properties are a sum over its stages.
- `hpr-aero` needs a nose-first layout, so a partial airframe has **no** aerodynamic model. A body
  that had to fly aerodynamically would need M1.8's work and a way to build a model for a
  headless stack.
- M1.9 will add staging, where a sustainer lights after separation and keeps flying under thrust.

**Decision.**

- **A separation is a trigger plus a stage boundary** (`Separation { trigger, after_stage }`), with
  the same triggers a device has. Stages `0..=after_stage` keep the nose (body 0), the rest form
  body 1. Two bodies for M1.7c; more splits are additive and nothing in the types forbids them
  later.
- **The ascent ends at the separation**: `Termination::Separated`, a `Separation` event, and one
  `BodyFlight` per body in `FlightResult::bodies`. Nothing continues in six degrees of freedom,
  because no body has an aerodynamic model.
- **A body is its own stages and their motors.** Its mass properties are the sum, so the bodies'
  masses add to the rocket's at that instant (a test). A body's mass is then held **constant**
  through its descent, which is why a separation must follow the last burnout. That is **enforced**
  when the trigger fires, not merely documented: whether it fires before the burnout depends on the
  flight, so it is a flight-time `SimError::Domain` rather than a setup check. A release across the
  separation is refused when the devices are given, because a line cuts a device on its own body.
- **The separation adds no impulse.** Each body starts at its own centre of mass with the velocity
  that point already had (`v_O + ω × r_cg`), so the bodies' **linear** momenta add to the stack's
  (a test). Angular momentum is not conserved: the bodies drop their rotation, which discards each
  body's spin about its own centre (31% of the total at the test's 0.6 rad/s). The identity is
  exact only after burnout, because `v_cg` also carries the centre of mass's motion inside the
  body — which is the other reason the burnout rule is enforced. An ejection charge's impulse, the
  tip-off it gives each body and the tumbling that follows are not modelled; hpr says so rather
  than inventing a spring constant.
- **Every body must carry a recovery device, and that device must open.** A flight whose bodies
  are not all covered is refused when it is set up; a body that reaches the ground with no
  deployment at all is refused in flight. With no airframe drag in the descent, either case would
  be a fall in a vacuum, which is a wrong number rather than a missing feature. The corollary is
  recorded in `recovery.md`: a body coasts drag-free between the separation and its first
  deployment, which reads high in arrival speed. A spent booster's device is
  normally `DeviceDrag::tumbling_stages` over its own stages, which is §3.5's model applied to
  that body rather than to the whole stack (the whole-stack form is still there for a stack that
  tumbles without separating).
- **The bodies fly as 6-state point masses** through the same integrator, with the descent's
  equations less the thrust, and their own stop times and events: their devices' trigger times,
  the deployments, ends of filling and releases their devices already have, their own apogee, and
  their own deployment heights. Review found the first draft missing the apogee and the carried
  times, which made a body separated while climbing fall ballistically and a deployment scheduled
  before the separation vanish. They share the flight's devices and their progress, so a canopy
  that opened before the separation stays open on the body that carries it.
- **Only body 0's devices act before the separation.** A device for another body has a drag area
  computed for that body, which is not a model of the whole stack.
- **The separation's own trigger is a stop time, and its height is a located event**, as a
  device's is; review found it polled at whatever boundary happened to come next, which fired a
  timed separation 186 s late and an altitude one never.
- **A body's descent is not observed.** `Observer` sees a 13-element rigid-body step; a body's
  state is six. Its events and samples are in its `BodyFlight` (`BodySample`, `BodyEvent`), which
  is what a report needs. Wiring the observer to bodies can come with M2.1's reports.

**Consequences.**

- M1.9's staging extends this: a body that keeps flying needs an aerodynamic model for a headless
  stack, which is the real work, and the ignition times and mass variation that go with it.
- M2.1's landing metrics take the bodies, not just the final sample.
- A separation with three or more bodies needs `Separation::BODIES` generalised to a list of
  boundaries; nothing else changes.
- Because a body is a point mass, its attitude is not tracked at all after separation, so nothing
  can report how a booster is oriented as it tumbles.
