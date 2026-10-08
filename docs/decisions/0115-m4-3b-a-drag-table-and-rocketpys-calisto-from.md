# ADR-115: M4.3b: a drag table, and RocketPy's Calisto from Python (2026-09-29)

- **Status:** accepted
- **Summary:** M4.3b: a drag table on the builder, gravity and a parachute release in Python; Calisto measured in the example, not in the library

**Context.** M4.3b asks for a flight that takes a drag table (`C_D0` by Mach, power on and off),
and a notebook-style example that flies RocketPy's Calisto from Python within M2.1's 3% on every
scored metric, run in CI. The validation suite's `fly_whole_flight` flies that case through
`hpr_sim` directly. Its setup differs from what the builder and the package offered in four ways:
the drag table (`Simulation::with_drag_table`), RocketPy's gravity formula
(`GravityModel::VerticalTaylor`), one parachute at a time (`Device::with_release_by`), and metrics
measured as RocketPy defines them. RocketPy's metrics follow the dry centre of mass from its start
at the ground. Its rail exit is the forward button's, after `effective_1rl`. Its landing is when
that point is back at its starting height.

**Decision.**

1. **The builder takes a table.** `FlightBuilder::drag_table(DragTable)` sets it as `drag_model`
   sets a model. The last one set is flown, as `AeroModel` has it. `Environment::with_gravity`
   swaps the gravity model and keeps the site and the Earth's rotation.
2. **The package exposes them.** `hpr.DragTable(power_off, power_on=None, *,
   reference_diameter_m=None)` takes `(mach, cd)` rows, linear between rows and held at the ends,
   as `parse_mach_csv`'s tables are. `DragTable.from_csv` reads RocketPy's two-column files. The
   rest are keyword arguments: `Flight(..., drag_table=)`, `Environment(..., gravity=)` and
   `add_parachute(..., released_by=)`. `gravity` names one of the three models that take no number,
   read through `GravityModel`'s `serde` tag. `released_by` is the other parachute's index, as in
   Rust; a bad index is refused when the rocket flies, by the simulation's own check.
3. **Drag that pushes is refused.** A drag table's negative coefficient, in a row or past or
   between its rows (a curve that extrapolates linearly, a cubic), was flown as it was, as drag
   pushing the rocket along (#257); a drag model's was already refused. The aerodynamics now
   refuse a table's as they do a model's, where a flight meets it, and the package refuses a
   negative row when a table is made. The corpus checks (`cargo xtask ork-flights --check`,
   `--library --check`, `real-flights --check`) and the validation report reproduce unchanged.
4. **RocketPy's definitions stay in the example.** The library gains no RocketPy-shaped metrics.
   `crates/hpr-py/examples/calisto.py` measures them on the flight's recording and events: the dry
   centre of mass is found by turning its body-frame position by each row's attitude. `h0` comes
   from a first flight of the rocket before its parachutes are added. The rail exit is found in
   the step that crosses `effective_1rl`, filled in with a cubic for the speed that matches the
   speed and the acceleration at both ends. The landing is interpolated linearly between the two
   steps where `height_above_ground_m`, the height the suite uses, crosses `h0`. The main opens
   `h0` above RocketPy's 800 m, as the suite has it. Two settings differ from the suite's and
   don't matter here: the builder's 3,600 s time limit (the suite's is 6,000 s; the flight takes
   261 s), and a table that holds its end values where the suite's refuses a Mach number past
   them (the flight stays below Mach 0.74, the table runs to 3).
5. **Checked twice.** `test_calisto.py` runs the example on the built wheel. It takes the scored
   metrics from the committed report's rows for the case that carry a relative tolerance: 14 of
   the 16 it scores, the other two being the trajectory's RMS rows, held to absolute bounds by the
   suite alone. It holds each to 3% of RocketPy's value, and to 1e-5 of the suite's own measurement of
   the same flight. The guide's printed table must equal what the example prints.

**Consequences.** Every metric scored in percent is within 3%; the largest difference is the
landing drift's, +1.257%, as in the suite's report. The example agrees with the suite's hpr
numbers to 3.2e-6 or better; the largest gap is the top speed's, which the example takes at the
integrator's steps and the suite also finds inside them. A wrong gravity model or a main opening
at 800 m rather than 800 m + h0 moves the landing drift by 1.8e-4 or 5.9e-4, so the test's 1e-5 catches
it. Only the windy case is flown from Python; the calm one and predicted mode stay the suite's. The example reads
the reference fixture's JSON, so it runs only from a checkout, not from an installed wheel alone.
