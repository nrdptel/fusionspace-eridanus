# ADR-011: Rigid-body flight: equations of motion, aerodynamic coupling, rail, phases and termination (2026-09-17)

- **Status:** accepted
- **Summary:** Rigid-body flight: equations of motion, aerodynamic coupling, rail, phases and termination

**Context.** M1.6b builds the flight on the M1.6a integrator. It needs:

- a variable-mass rigid body with jet damping;
- aerodynamic forces from `hpr-aero`, which has no damping coefficients and refuses `M ≥ 1`;
- a rail with friction and button geometry, and events;
- a flight that names why it stopped;
- a Level 2 flight in 5 ms.

M2.1 will compare with RocketPy, whose documented equations are MIT. Loft's lessons L20, L24, L25
and L26 set tests.

**Decision.**

- **Equations: RocketPy's documented variable-mass formulation about a body-fixed point, with
  that point at the nose tip.**
  - RocketPy's technical documentation, Equations of Motion v0/v1 (Kane plus Reynolds transport,
    quasi-steady internal flow), covers centre-of-mass
    migration, `İ`, the jet's Coriolis force and jet damping in one exact form.
  - With the reference point at the body origin, the state needs no conversion to the design's
    stations.
  - The equations are written out in `flight.md` and checked against their classical limits:
    torque-free Euler motion about the centre of mass, and the classical jet damping
    `I_c ω̇ = [ṁ(r_e²/4 + l²) − İ_c] ω` (to 1e-6).
  - Measured thrust curves already contain the internal momentum terms `T04` subtracts
    (`−m r″ − 2ṁ r′ + m̈(n − r)`). They are kept, as RocketPy keeps them, for M2.1's comparison.
    The cost is at most 0.05 m/s at burnout and 21 N at Valetudo's liftoff (`flight.md`). Revisit
    after M2.3's flight data.
- **The nozzle gyration tensor comes from the integral, not from RocketPy's code.**
  - `S = (r_e²/4) diag(1, 1, 2) + |n|² 1 − n nᵀ`.
  - RocketPy 1.13.0 codes the transverse distance term as `0.25·n²`. M2.1 has to account for the
    difference.
- **Mass-property rates by central differences inside the integration interval** (half-width
  1e-4 s).
  - `hpr-motor` gives no derivatives of the grain inertia.
  - Stop times at every thrust-curve knot and burnout keep each difference on one side of every
    kink, and the mass rate matches the motor's `−F/c` to 1e-6.
  - The four `mass_properties` calls per evaluation are most of the 0.4 µs an evaluation costs
    (`perf.md`). Analytic motor derivatives are a later optimization, not a correctness need.
- **Earth's rotation: Coriolis force at the centre of mass, no Earth rate in the rotational
  equations.** It is at most 7.3e-5 rad/s against body rates of 0.01–10 rad/s. RocketPy does the
  same. Gravity is normal gravity at the centre of mass.
- **Aerodynamics component by component at the local flow.**
  - Each body and fin set sees `v_O − wind + ω × p_i` at its small-angle CP. That gives the pitch
    and yaw damping `hpr-aero` doesn't have, as RocketPy does.
  - The axial force comes from the whole rocket at the centre of mass's airspeed, power-on while a
    motor burns.
  - The linear pitch model matches the flight's period to 8e-5.
  - `hpr-aero` gains `component_count`, `component_normal_force` (allocation-free) and
    `component_station_m`.
- **Fins follow the crossflow at any angle: `C_Nα sin α` instead of `C_Nα α` in flight.**
  - This is Niskanen's substitution for bodies (eq. 3.16–3.17), applied to fins. No large-angle
    fin model is in hand, and stall is not modelled.
  - It changes nothing at small angles and makes the force vanish for tail-first axial flow.
  - With the linear force, a calm vertical flight falling tail first after apogee flipped the fin
    force with rounding noise and ran to the step limit (found in review, pinned by a test).
  - `hpr-aero`'s own `normal_force` stays linear. M1.8 should move a large-angle fin model there.
- **Motors are evaluated inside their burn at interval ends.** A stage evaluated on ignition or
  burnout takes the one-sided limit inside the burn, because the pressure correction switches
  there. The step ending at burnout otherwise saw the burnt-out thrust, and RK4 converged at first
  order.
- **Out-of-range aerodynamics stop the flight.**
  - `M ≥ 1` anywhere is a `SimError::Aero` until M1.8. There is no silent clamp.
  - Large angles of attack use the small-angle models extended as above, recorded in
    `Sample::angle_of_attack_rad`.
- **Rail.**
  - The aft end starts at the rail's foot.
  - The rocket is guided with one degree of freedom until the aft edge of its last rail button or
    lug passes the top (L26), or its aft end without guides. RocketPy ends at the forward button.
  - Tip-off rotation is not modelled.
  - Coulomb friction `μ|ΣN|` uses the net rail reaction across the axis (weight, aerodynamic and
    mass terms), not the sum over the buttons. The default `μ = 0`, for want of a cited value.
  - The pad phase holds the rocket until the force along the rail beats friction. A stall on the
    rail returns it to the pad, held where it stopped rather than sliding back.
- **Events and termination.**
  - The events are liftoff, rail exit, burnout (a stop time), apogee (the centre of mass's
    ellipsoidal-height rate), ground hit (the centre of mass at the site's ellipsoidal height) and
    user events on a `Sample`.
  - Flights end as `GroundHit`, `NoLiftoff`, `StalledOnRail`, `TimeCap` or `StepLimit` (L25).
    Everything else is an error.
  - The atmosphere takes `h − N` with the geoid undulation given in `Environment`.
- **Defaults: `rtol = atol = 1e-8`, unit weights.** A Level 2 flight takes 1.1 ms and its apogee
  is within 1.1e-6 m of the converged value. At 1e-6 it would take 0.6 ms and 7e-5 m.
- **Observation.**
  - An `Observer` trait sees every accepted step (`FlightStep`: the dense state, and a `Sample` from
    one evaluation) and every event.
  - `Recorder` keeps chosen `Channel`s at a fixed interval or at every step, plus event rows.
    It is built, not deserialized, and cleared between flights.
  - Runs don't mutate the `Simulation` (L24).
  - `Environment` shares its atmosphere and wind through `Arc`, so it clones cheaply, and
    `Simulation` is `Send + Sync` for parallel Monte Carlo.

**Consequences.**

- M1.7 adds recovery devices as phases and events on this engine. A parachute's opening is stiff;
  bound the step there.
- M1.8 replaces the Mach refusal and adds roll forcing and damping. Until then the roll rate
  changes only through inertia coupling.
- M1.9 adds ignition times and staging (new stop times and a changing assembly).
- M2.1's RocketPy comparison must align the rail-exit convention, the nozzle gyration tensor and
  RocketPy's per-surface stations.
