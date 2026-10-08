# ADR-077: Flight metrics: peaks on the dense output, margins only where they mean something, and `None` for what didn't happen (2026-09-26)

- **Status:** accepted
- **Summary:** Flight metrics: peaks on the dense output, margins only where they mean something, and `None` for what didn't happen

**Context.** M1.10 asks for the stability margin over the flight, the optimum ejection delay, max q,
flutter, the landing point in latitude and longitude, and five file exports. That is more than one
pull request, so it is split in three: a, the flight metrics; b, fin flutter; c, the exports. Four
Loft lessons bear on a: L33 (margins of ±12 to 15 calibres published as the net normal-force slope
went to zero), L34 (the opening shock taken as the peak acceleration, and a finite difference that
read a thrust spike low), L35 (zeros for "never happened", and heights with no datum) and L94 (an
optimum delay that moved with the delay flown). hpr-sim is a pure core crate, so the metrics can
compute but not write files.

**Decision.**

1. **The split.** M1.10a: a `metrics` module in hpr-sim with an observer, `FlightMetrics`, and a
   `FlightSummary`; L33, L34, L35 and L94 go live. M1.10b: flutter from its primary source, with
   L32. M1.10c: the exports, written as text or bytes by the core, with the parent's schema and
   parsing checks. The parent's three *done when* bullets are unchanged; a carries the first, b
   the third, c the second.
2. **Peaks on the dense output.** The observer evaluates the equations of motion at each accepted
   step's start, middle and end. When the parabola through the three bends down with its top inside
   the step, it runs a golden-section search (Kiefer 1953) on the step's dense output, down to 1e-9
   of the flight's clock (at least 1 s), and keeps the largest of that and the three samples. (A
   first version searched only when the middle beat both ends, and missed peaks in a step's outer
   quarter by up to 1e-4: Juno III's max q, 27205 Pa for 27208.)
   Thrust-curve knots and events already end steps (M1.6a), so a spike's peak is a step's end.
   Peaks start at liftoff; a rocket that never lifts off has none. Measured on Valetudo in a
   vacuum: the peak acceleration matches the hand value from the motor and the masses to 1.6e-7
   (the test holds 1e-6), where a 100 Hz finite difference reads it 1.3% low.
3. **Acceleration.** The nose tip's (the body origin's) acceleration relative to the launch frame,
   as `Sample::acceleration_enu_m_s2` gives it, including gravity. The boost's peak is kept in the
   rail and free phases, the opening shock's in the descent phase, apart (L34).
4. **Two margins.** The static margin is the centre of pressure at Mach 0 with the air along the
   axis, against the centre of mass of the instant: RocketPy's `static_margin`. The roadmap's
   "dynamic" margin is read as the flight margin: the margin at the flight's own Mach number with
   the air along the axis, defined as RocketPy's `stability_margin` is (RocketPy's
   `min_stability_margin` takes its least over the whole flight, rail and descent included, so it
   can differ). A pitch damping ratio, the other reading, is not built here. Both margins are kept
   at each step's end from the rail exit to apogee or the first deployment, since on the rail the
   rail holds the rocket; a powered separation adds the sustainer's own entry at the split. The
   least of each is searched for inside steps, as a peak is, and a later least replaces an earlier
   one only when lower by more than 1e-12 of the earlier (or 1e-12 calibres below one calibre), so
   a flat least keeps its first time. The angle of attack is left out, after three versions that
   followed it failed review: its least came at the apogee in calm air (1.28 calibres on Valetudo); a floor on the dynamic
   pressure at the rail exit's let the apogee through off a tilted rail, which still crosses the
   air there as fast as it left the rail; and a 15° cap on the angle (the limit on a fin's cant)
   put the least on the cap, where it moved with the step size (0.84 to 1.23 calibres on one
   trajectory in review), and hpr models no stall to justify that cap. `metrics::margin` gives the
   margin at any angle for whoever wants it.
5. **No margin where it would be noise (L33).** With `κ = Σ |C_Nα,i| / Σ C_Nα,i` over the
   components (the table's own slope with a normal-force table), each acting at a station on the
   rocket, the centre of pressure lies within `κ L` of every station, so a fractional error `ε` in
   one slope moves it by up to about `ε κ² L` (to first order). A component that is a pure couple
   on its own has no station, and `κ` doesn't count it. The margin and
   the centre of pressure are `None` when the net slope is not positive or `κ > √10`, where a 1%
   error can move the centre of pressure a tenth of the rocket. (A first draft took the bound as
   `ε κ L` and the limit as 10; review showed the bound fails when the centre of pressure lies off
   the rocket, which is when `κ` is large.) The limit is a chosen bound on that sensitivity, not a
   measurement. All 13 designs in `validation/designs/` stay below `κ = 1.35` from Mach 0 to 2
   and to 20°, so it
   leaves ordinary designs their margin. The pitch-moment slope about the centre of mass,
   `C_mα = −(Σ C_Nα,i x_i − x_cg Σ C_Nα,i)/d`, is always given, from a new
   `NormalForce::moment_slope_m` that keeps a component's pure couple.
6. **The optimum delay (L94).** A crate-private copy of the simulation with every stack charge
   held flies to its apogee; each motor that burns out before it gets `apogee − burnout`. A
   separation with nothing ahead of it left to burn is recovery and is held too; a powered one
   still happens, and the motors of the body it drops get no optimum, since their charges fire in
   that body. No apogee gives `None`.
7. **Datum and absence (L35).** Heights are the centre of mass's ellipsoidal height above the
   site's; the summary adds the starting height, and the apogee's gain from it (OpenRocket's
   altitude). Every metric that may not happen is an `Option`, which is `null` in JSON.
8. **Landings.** The flight's own (the stack's, or the sustainer's after a powered separation) and
   each separated body's that landed, in WGS 84 latitude and longitude through `LaunchFrame`, with
   east and north metres and the ground-hit speed.
9. **`FlightStep::stability`.** Each step gives the margins at a time from the model flying (the
   sustainer's after a powered split), so a new required trait method; hpr-validate's test stub
   refuses it. A watcher keeps one flight, and `summary` refuses a flight unless it saw each of
   its accepted steps once.

**Consequences.** The site gains *Flight metrics*, with an example whose output CI checks. The
margin limit of √10 is a choice that a later model of component uncertainties could replace with a
measured one. The metrics watch only the main flight: a separated body gets a landing but no
peaks.
