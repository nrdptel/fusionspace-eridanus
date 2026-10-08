# ADR-137: Ten thousand flights: a run's flights share the nominal's layout and supersonic table (2026-10-01)

- **Status:** accepted
- **Summary:** M6.1d: a Monte Carlo run's flights share the nominal design's supersonic table (`AeroModel::share_supersonic_table`, when the covered segments and reference area are equal) and its layout (`Rocket::lay_out`, `LaidOut::relay`, `Simulation::from_laid_out`); evaluations borrowed, not copied; 10,000 flights timed on Valetudo (the flight benchmarks' Level 2 rocket) and on a supersonic K940 rocket, by a bench target; per-evaluation speed-ups left to #285

**Context.** M6.1d asks for 10,000 flights of a Level 2 design in 10 s or less on the development
machine, recorded in `docs/perf.md`. Each Monte Carlo sample builds its own `Simulation` from its
draw. Before this change, timed on an Apple M5 with 10 threads, release build, flights to the
ground under a drogue and a main:

- Valetudo on a K400C, peaking at Mach 0.3 to 0.4, took 4.59 s for 10,000 flights.
- A 66 mm rocket with a 54 mm mount on a K940, peak Mach 1.6 to 2.0, took 264 s. Each of its
  flights built its own supersonic table on its first pass of Mach 1.2: most of a 193 ms flight.
- Building a simulation laid the design out twice, 0.4 ms each time.

Open: which design is "an L2 design", and how to stop paying for that work every flight without
changing a flight.

**Decision.**

1. **The flights of a run share the nominal design's supersonic table.**
   - `MonteCarlo::new` builds the nominal design's `AeroModel`. Each sample calls
     `Simulation::share_supersonic_table` with it.
   - The table is a pure function of the shock-expansion run's segments and the reference area
     (`SupersonicBody::new(run, reference_area_m2)`). So `AeroModel::share_supersonic_table`
     shares only when both are equal (`PartialEq`, `==` on `f64`), and leaves a different shape or
     reference area alone.
   - No dispersion changes the shape. Masses and centres are stage overrides, a dispersed motor
     keeps its geometry, and drag is a scale on the model's output. So every sample of a design
     shares, yet nothing is assumed: a sample that did differ would build its own table.
   - The table sits behind `Arc<OnceLock>`. The first sample that needs it builds it, and threads
     that need it meanwhile wait.
   - A sustainer built at a powered separation still builds its own table.
2. **The flights of a run share the nominal design's layout.**
   - `Rocket::lay_out` gives a `LaidOut`: an owned copy of the design, its layout, and each
     stage's masses before its overrides. `LaidOut::check` and `LaidOut::assemble` share the one
     layout, and `Simulation::from_laid_out` builds on it; `Simulation::new` is `from_laid_out`
     of `lay_out`.
   - `LaidOut::relay(rocket)` keeps the placed parts when `rocket` differs only in its stages'
     overrides and its configurations. Every other field is compared, by an exhaustive
     destructure, so a new field can't be missed. It redoes only the stages, by the same code as
     `Rocket::layout`; any other difference lays the design out from the start.
   - Because `LaidOut`'s fields are private, a layout can't be paired with another design. That
     answers the review of a first version, which had public `check_with_layout` and
     `assemble_with_layout` taking any layout.
   - Samples, and the new `MonteCarlo::fly` for draws made by hand (the sensitivity guide's),
     use both. `FlightInputs::fly` builds everything itself.
3. **The integrator borrows each evaluation** from the phase's cache rather than copying a few
   hundred bytes out of it at every stage and event check.
4. **"An L2 design" is Valetudo**, RocketPy's 9.7 kg rocket on a K400C. It has been the "typical
   Level 2" flight of `hpr-sim`'s flight benchmarks since M1.6b. The K940 rocket is timed beside
   it, because supersonic flights are where a run was slow. Both fly to the ground under a drogue
   and a main.
5. **The measurement is a bench target, not a test:** `crates/hpr/benches/ten_thousand.rs`, run by
   `cargo bench -p hpr --features parallel --bench ten_thousand`.
   - A wall-clock limit in CI would fail on shared runners for reasons that have nothing to do with
     the code.
   - Each timed run starts from `MonteCarlo::new`, so the table's one build is counted. The program
     prints the fastest of three runs, and where one flight's time goes.
   - It does nothing unless `cargo bench` runs it.

**Checks.** Every flight is unchanged bit for bit:

- `samples_share_the_nominal_table_and_fly_as_alone` flies supersonic samples against the same
  inputs flown alone, through `sample`, `fly`, and fresh runs on two and five threads. It checks
  that the samples built the nominal's table, which fails with sharing off.
- `models_built_apart_share_the_table_only_on_the_same_body` fails if either half of the sharing
  rule is dropped.
- `a_relaid_design_is_laid_out_as_alone` holds relaid layouts, checks and assemblies to fresh ones,
  and fails if `relay` keeps the parts of a changed design.

**Consequences.** M6.1d is met.

- Valetudo's 10,000 flights take 3.00 s and the K940 rocket's 9.43 s (2.91 to 3.00 s and 9.33 to
  9.47 s over three runs of the program during the last changes), with the same apogee and Mach statistics as before.
- The supersonic run is about 6% inside the budget. A profile, taken before item 3, puts 11% of its
  busy time in `atan2`
  (flow angles, the geodetic conversion, gravity) and 8% in `pow` (drag terms, the atmosphere).
  Each fix changes the flown numbers in their last bits, so every committed report, including the
  corpus reports that CI can't fly, would have to be regenerated. That is left to #285.
- A sensitivity analysis of a supersonic flight through `MonteCarlo::fly` now costs a few
  milliseconds a flight, not a fifth of a second.
