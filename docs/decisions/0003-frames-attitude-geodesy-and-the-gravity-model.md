# ADR-003: Frames, attitude, geodesy and the gravity model (2026-09-17)

- **Status:** accepted
- **Summary:** Frames, attitude, geodesy and the gravity model

**Context.** M1.1 fixes the conventions that every later crate uses: the frames, the attitude
representation, the Earth model, and interpolation tables. Loft simulated a flat Earth with
constant 9.80665 m/s² gravity and never stated a frame. That left it 0.3% off RocketPy's gravity
at the equator (lesson L1), and its apogee datum and ground-hit frame were never defined (L35).
The comparisons in M2.1 need hpr's conventions to map exactly onto RocketPy's.

**Decision.**

- **Math types:** glam's `f64` types (`DVec3`, `DQuat`, `DMat3`) with the `f64`, `serde` and
  `std` features only. `hpr-core` re-exports them.
- **Frames** (`docs/physics/frames.md`):
  - **ECEF:** WGS 84.
  - **Launch frame `L`:** East-North-Up, origin at the pad, `z_L` along the ellipsoid normal.
    State propagates in `L`, which is Earth-fixed.
  - **Body frame `B`:** `+z_B` along the axis toward the nose, `x_B` along the design's zero
    radial direction.
  - **Attitude:** a unit Hamilton quaternion mapping body to launch components, with
    `q̇ = ½ q ⊗ (0, ω_B)` and renormalization after every step.
  - **Launch angles:** `q = R_z(−A) R_x(E − π/2) R_z(φ)`. This is RocketPy's 3-1-3 initial
    attitude with heading, inclination and rail-button angle, pinned by a RocketPy oracle
    fixture (`validation/oracles/rocketpy/attitude.py`).
  - **Rationale:** RocketPy is the main oracle, so matching its axes removes a class of sign
    errors from the comparisons. At the identity attitude the rocket stands vertical on the pad.
- **Heights:** every height in the core is ellipsoidal (`Geodetic::height_m`). Heights above sea
  level are converted at the I/O boundary. The flight engine reports altitude from the geodetic
  height of the position, not from `z_L`.
- **Geodetic conversion:** the forward conversion is NGA.STND.0036 eq. 4-14. The inverse is
  Karney's (2011, appendix B) version of Vermeille's closed form. It needs no iteration, is exact
  to rounding, and fails only within 43 km of the Earth's centre, where it returns an error.
  Bowring's iterative method was rejected: it needs an iteration count and a convergence
  tolerance, and its primary source is paywalled. Karney's paper is open.
- **Gravity:** WGS 84 normal gravity from its four defining parameters (NGA.STND.0036 ch. 4,
  app. B), with the exact ellipsoidal-harmonic closed form as the default.
  - The flight engine chooses from `constant`, `vertical_taylor` (RocketPy's), `vertical` and
    `ellipsoidal` (the default). Earth rotation contributes Coriolis only, by default. The
    centrifugal term is inside normal gravity.
  - `q` and `q′` use their `atan` series below `ε = 0.5` to avoid cancellation. That makes the
    derived constants round to Table 3.6.
  - **Rationale:** exact at every height a hobby rocket reaches, cheap enough for the inner loop
    (35.5 ns a call on the dev machine, `docs/perf.md`), and able to reproduce RocketPy's
    formula exactly when a comparison needs it.
- **Reference values:** no publication tabulates normal gravity by latitude and height.
  `validation/oracles/wgs84/normal_gravity.py` evaluates the published formulas at 40 digits with
  mpmath, which was added to the oracle environment. It checks the derived constants against the
  tables and the vector against the gradient of the normal potential. Its JSON output is committed
  under `validation/fixtures/earth/`, and the Rust tests read it with `include_str!`, so CI needs
  no Python.
- **Tables:** `Table1D` offers linear or natural-cubic interpolation, with a `clamp`, `linear` or
  `error` extrapolation policy. Every lookup reports whether it extrapolated. A monotone cubic
  waits until a model needs one (`docs/physics/interpolation.md`).
- **JSON floats:** `serde_json` has `float_roundtrip` enabled workspace-wide, so a parsed number
  is the exact `f64` that was written.

**Consequences.**

- M1.6 must:
  - use the geodetic height for ground contact and apogee;
  - decide whether its rotational dynamics include the frame rate `Ω` (at most 7.3e-5 rad/s);
  - call `Earth::gravity_enu_mps2` and `Earth::rotation_acceleration_enu_mps2` instead of a
    constant.
- M2.1 cases that compare with RocketPy select `vertical_taylor`. They must also account for
  RocketPy feeding height above sea level to its formula, and holding gravity constant above
  `max_expected_height` (80 km by default).
- M1.4 fixes the body reference point and the station mapping `z_B = z_ref − s`.
