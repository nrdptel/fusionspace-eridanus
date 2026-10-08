# ADR-010: Time integration: Dormand–Prince with dense output, RK4, stop times and events (2026-09-17)

- **Status:** accepted
- **Summary:** Time integration: Dormand–Prince with dense output, RK4, stop times and events

**Context.** M1.6 asks for adaptive Dormand–Prince 5(4) with dense output and event
root-finding, plus a fixed-step RK4 option. M1.6 is split into M1.6a (the integrator and events)
and M1.6b (the flight). Loft's RK4 had no error control and no apogee convergence check (L21). It
didn't root-find events (L22), and it let burnout fall inside steps (L23). No crate in the
workspace had an ODE integrator or a root finder. Hairer and Wanner's `DOPRI5` is the reference
implementation of the method in their book and is BSD-2-Clause.

**Decision.**

- **One integrator in `hpr_sim::integrator`, no dependency.** It works on `[f64; N]` states
  through an `OdeSystem<N>` trait whose derivative can fail with the system's own error.
  Established Rust ODE crates either lack event location on dense output or pull in a
  linear-algebra stack. The method is a few hundred lines, and owning it lets the tests pin every
  coefficient.
- **Port `DOPRI5` as the reference.** Take its coefficients, the RMS error norm, the PI controller
  (`β = 0.04`, growth limits 1/5 to 10, safety 0.9), the starting step and the dense output
  (`CONTD5`). The stiffness detection is left out. The port is attributed in
  `THIRD-PARTY-NOTICES.md`, and the source is pinned (`hairer-dopri5`).
- **Per-component tolerance weights come from the system.** `Adaptive` holds one relative and one
  absolute tolerance, so it serializes, and `OdeSystem::absolute_tolerance_weights` scales `atol`
  per component. The default is `rtol = atol = 1e-8`. M1.6b sets the flight's weights and may
  change the defaults, from its benchmark.
- **`advance(system, t_stop)` is the only driver, and the system carries everything.**
  - `OdeSystem<N>` has the derivative and provided methods for tolerance weights, events and
    `accept_step`. One flight-phase type can then compute forces, define its events, record
    each step and stop the run, without the double borrows that separate event and observer
    arguments would force.
  - `advance` stops exactly at `t_stop`, at events (`Advance::Events`), or when `accept_step`
    breaks (`Advance::Stopped`).
  - Discontinuities are stop times, and the caller sets the phase between calls, because the step
    that ends at a stop time evaluates its last stage there.
  - Each call evaluates `f` afresh, and the step-size estimate carries over.
- **Events are sign changes between step ends, located by Brent's method on the dense output** to
  1e-12 s. The integrator stops at the end of the final bracket on the far side of the zero, with
  the dense-output state there, and reports every event past its zero at that state
  (`fired_events()`).
  - Stopping past the zero guarantees that the next call doesn't report those events again. A
    fresh full-order step can land a hair short of the zero and trigger again. Reporting all of them
    keeps coincident events, such as two deployments at apogee, from being lost.
  - The cost: the state at an event is fourth order (third for RK4) rather than fifth. The tests
    show event times within about 1.5e-8 s at the default tolerances.
  - Double crossings inside one step go unseen, and `max_step_s` bounds that. A `reset` that moves
    an event function back across zero re-fires it, so systems normalize inside their functions.
- **Departures from `DOPRI5`, all for robustness.**
  - A derivative that fails in a trial stage, or a non-finite error estimate, is a rejection. It
    becomes an error only when the step can't shrink.
  - A final interval within rounding counts as reached.
  - An overflowing step is an error.
  - The controller, the starting step and the error norm are otherwise `DOPRI5`'s, pinned by
    Hairer's Arenstorf problem, whose evaluation and step counts match an independent
    transcription.
- **RK4 is fixed-step,** shortened only to land on a stop time or an event. Its dense output is the
  cubic Hermite interpolant, whose end derivative is the next step's first stage.
- **Failures are errors, never quiet stops.**
  - The errors are `Derivative`, `StepTooSmall`, `NotFinite`, `StepLimit` (default 10⁶ attempted
    steps, adjustable with `set_step_limit`), `EventNotFinite`, `Backward` and `Settings`.
  - The integrator stays at its last accepted step and can resume.
  - M1.6b maps the errors to termination reasons (L25).
- **Settings files.**
  - `Method` is tagged `dopri5` or `rk4` and refuses unknown fields.
  - `Adaptive` keeps public fields with `Default`, for `..Adaptive::default()`. The crates are
    unpublished, so adding a field later is acceptable.

**Consequences.**

- M1.6b builds the flight as `OdeSystem<13>` phases (rail, powered, coast) separated by stop times
  (motor ignition and burnout, thrust-curve knots where they matter) and events (liftoff, rail
  exit, apogee, ground contact from the ellipsoidal height). It sets the tolerance weights for
  position, velocity, quaternion and body rate. It normalizes the quaternion where it is used, and
  resets only away from events.
- The recorder samples `Step::state_at` at its output times rather than forcing steps onto them.
- Stiff phases (a canopy opening in M1.7) will show up as small steps or `StepTooSmall`. M1.7
  decides whether they need a bounded step or a different method.
