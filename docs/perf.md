# Performance

Measured numbers only, newest first within each section. Record the machine, the toolchain, and
the command, so a later run can be compared like for like.

## A step's start taken from the step before (2026-10-20)

- **What this is.** The flight metrics read each accepted step's start, middle and end. The end
  has come from the integrator since the entry below; the start, the same time and state as the
  step before's end, was evaluated afresh. A step now keeps the integrator's evaluation from the
  step before (`PhaseSystem::previous` in `crates/hpr-sim/src/flight.rs`) and takes the start from
  it when the time and all 13 state components match bit for bit. In one profile of the K940's
  flights (macOS `sample`, not committed) the metrics fell from 35% of a flight to 28%.
- **Measured** on an Apple M5 (10 cores), rustc 1.98.1, main e247ddfe against the change,
  alternating: `cargo bench -p fusionspace-hpr --features parallel --bench ten_thousand` for the
  10,000 flights (each figure the fastest of three), `cargo bench -p fusionspace-hpr-sim --bench
  flight -- "flight metrics"`, and the wall time of a release `hpr mc
  validation/fixtures/ork/guides/level-1.ork --runs 10000`:

  | measurement | before | after |
  |---|---|---|
  | Valetudo: 10,000 flights on 10 threads | 2.57 / 2.52 s | 2.36 / 2.38 s (−7%) |
  | K940: 10,000 flights on 10 threads | 7.85 / 7.85 s | 7.18 / 7.33 s (−7 to −9%) |
  | "Valetudo K400C with its flight metrics" | 2.036 / 2.032 ms | 1.875 / 1.857 / 1.877 ms (−8%) |
  | `hpr mc level-1.ork --runs 10000` | 3.27 / 3.34 / 3.37 s | 3.08 / 3.10 / 3.12 s (−7%) |

- **What moves.** No output bit. `shared_step_evaluations_change_no_output_bit` (below) covers the
  new path: a flight held on the pad, where the state stays the same to the bit while the time
  moves, sampled inside its 1 ms steps, fails it if the step before's evaluation is taken on the
  state alone. The committed reports and `hpr mc`'s output reproduce byte for byte.

## A step's evaluations shared by its observers (2026-10-15)

- **What this is.** Every `hpr sim` and Monte Carlo flight carries `FlightMetrics`, which tracks
  the peaks, the events and the stability margin. It reads the vehicle's state at each accepted
  step's start, middle and end, once for the sample and again for the margin, and the end is
  usually the state the integrator has just evaluated. A profile of the K940's dispersed flights
  (macOS `sample`, 13,003 samples) put 47% of a flight's time in that observer. A step now keeps
  each time's evaluation (`StepView::evaluation` in `crates/hpr-sim/src/flight.rs`), and takes the
  end's from the integrator when its time and state match exactly. Within a step, an evaluation
  depends on the time alone, so nothing it returns changes.
- **Measured** on an Apple M5 (10 cores), rustc 1.98.1, main 6537d2c9 against the change,
  alternating, two runs each of `cargo bench -p fusionspace-hpr --features parallel --bench
  ten_thousand` (each figure the fastest of three):

  | measurement | before | after |
  |---|---|---|
  | Valetudo: 10,000 flights on 10 threads | 3.02 / 3.07 s | 2.59 / 2.72 s (−11 to −14%) |
  | Valetudo: a flight on one thread | 1.920 / 1.980 ms | 1.649 / 1.649 ms (−14 to −17%) |
  | K940: 10,000 flights on 10 threads | 9.46 / 9.97 s | 8.12 / 8.38 s (−14 to −16%) |
  | K940: a flight on one thread | 5.821 / 5.866 ms | 4.864 / 4.861 ms (−16 to −17%) |

  A new criterion bench carries the observer, which "Valetudo K400C to the ground" doesn't:
  "Valetudo K400C with its flight metrics" read 2.347 / 2.354 ms before and 2.036 / 2.030 ms after
  (−13 to −14%; `cargo bench -p fusionspace-hpr-sim --bench flight`). The 47% is from one
  profile, not committed: macOS `sample` for 18 s over dispersed K940 flights on one thread.
  Each slot is a `OnceCell` read in place: a first version that copied the four slots in and out
  of a `Cell` on every call gained 10 to 13%.
- **What moves.** No output bit. `shared_step_evaluations_change_no_output_bit`
  (`crates/hpr-sim/src/tests.rs`) flies five flights both ways (a test-only switch evaluates every
  instant afresh): past Mach 1, Valetudo, Valetudo under a drogue and a main that open over time,
  Valetudo on the fixed-step RK4, and a two-stage stack, all in a crosswind. It compares the
  metrics, the summary and every channel recorded at 10 ms bit for bit. Two mutations fail it:
  taking a canopy's drag area at the step's start on the shared path only (the two paths share
  one call, so an edit to both changes nothing it compares), and answering from another time's
  slot. The
  state compared before reusing the integrator's evaluation decides nothing today (that
  evaluation is always the step's end), so removing it passes; the time compared does decide. The
  committed reports reproduce byte for byte (the gate's `validate`, `examples` and `refs` checks).

## The site's layout check on six tabs (#469)

- **What this is.** `cargo xtask site` lays out every page at every checked width in Chrome, one
  browser process per tab, up to `MOST_TABS` (`xtask/src/site/pages.rs`), fewer on a machine with
  fewer cores. The check reads layout, not timing, so running more tabs at once doesn't change
  what it finds. The page-vitals pass after it still runs alone, in one browser.
- **Measured** on an Apple M5 (10 cores, 16 GB), main f0c1a3cc, a full `cargo xtask site
  --locked` each, in alternating order: the layout check took 182 / 182 s at four tabs, 134 s at
  six and 118 / 119 s at eight; the whole step 675–703 s, 618 s and 592–594 s. The browsers'
  summed resident memory peaked at 8.0–8.5, 10.4 and 11.9–12.0 GB (shared pages counted more than
  once); no swapping in any run. Six was chosen to leave memory for work running beside the gate.
- **What moves.** The layout line's counts and every other line of the step's log but the vitals'
  are byte-identical across the five runs, seconds masked; the vitals line's readings vary as much
  between two four-tab runs. CI's 4-core runners still run four tabs, so CI's time doesn't change.
  The vitals pass (383 s of the step here) is #469's larger part and is unchanged.

## The aerodynamics crate optimized in test builds (#494)

- **What this is.** `fusionspace-hpr-aero` is built at `opt-level = 3` in the test profile (the
  workspace `Cargo.toml`), which `cargo test` and `cargo nextest` use, but not in the dev profile
  that `cargo xtask` runs. Its tests sweep the flare, boattail and potential-flow models over many
  shapes and Mach numbers, and unoptimized they took a third of the workspace's test time.
- **Measured** on an Apple M5, rustc 1.98.1, main ee26ec70, warm builds: the crate's 245 tests
  49.9–50.0 s → 30.0–30.1 s (`cargo nextest run -p fusionspace-hpr-aero`, two runs each); the whole
  workspace's nextest run 167.5–167.8 s → 142.4–142.8 s (three runs each, −15%), its summed
  test-seconds 1,032 → 794 (the aerodynamics crate's 352 → 202, `fusionspace-hpr-sim`'s 108 → 69). The
  first test build after the change took 31.6 s.
- **What moves.** Nothing committed. In the dev profile the same setting moved the refs/ reports'
  full-precision numbers in about the ninth digit (in `ork-flights`, one apogee's relative error
  1.2320332389% → 1.2320332403%; the cause wasn't traced, likely the `sincos` merge #397 found), so
  the reports' generator keeps the unoptimized build and still reproduces every report byte for
  byte (`cargo xtask validate --check`, `examples --check` and the `refs` checks). The tests pass
  at either level.

## The analysis crate optimized in test builds (#397)

- **What this is.** `fusionspace-hpr-analysis` (Monte Carlo statistics, the optimizers) is built
  at `opt-level = 3` in the dev profile (the workspace `Cargo.toml`), which the tests, `cargo
  xtask examples` and `cargo xtask cli` use. Its EGO tests fit Gaussian-process surrogates, which
  unoptimized code does about 12× slower.
- **Measured** on an Apple M5, rustc 1.98.1, main 7cdc08b6, warm builds, two runs each: the
  crate's 176 tests 49.0–51.2 s → 3.1–3.2 s wall (330 → 28 test-seconds); the whole workspace's
  nextest run, 2,433 tests, 150.3 / 153.6 s → 116.6 / 116.7 s (−23%). A rebuild after touching
  the crate takes 4.6 s instead of 4.2 s. The examples step doesn't change (133.7 s): its time is
  flight simulation in other crates.
- **What moves.** Every committed report and example output reproduces byte for byte (`cargo
  xtask validate`, `examples`, `cli` and the five `refs` checks, regenerated and diffed). At full
  precision the dev build's Monte Carlo statistics move by about 1e-12 of their value, and 3 of
  300 dispersed flights by up to 1.5e-6 of a peak angle of attack: at opt-level 3 LLVM merges the
  dispersed wind's `sin_cos` (`DispersedWind::wind`, called in every flight's derivative) into the
  platform's `sincos`, which can round one ulp differently, and a flight can grow that. Results stay bit-identical run to run on one platform, which is the promise; they
  were already different between the dev and release builds.

## Ten thousand flights (M6.1d)

- **What this is.** How long 10,000 dispersed flights of a Level 2 rocket take, which
  [M6.1d](decisions-and-roadmap.md#m6-1d) holds to 10 s. A Level 2 rocket is one on a J, K or L
  motor, the classes a Level 2 certification allows.
- **Benchmark:** `cargo bench -p fusionspace-hpr --features parallel --bench ten_thousand`
  (`crates/hpr/benches/ten_thousand.rs`), release profile. It takes about a minute after it
  compiles. Each run is `MonteCarlo::new` and then 10,000 flights through `run_parallel`, with one
  worker thread per core. The program times three runs and prints the fastest. Then, on one
  thread, it prints where a flight's time goes.
- **When and where:** 2026-10-01 on an Apple M5 with 10 cores (4 fast performance cores, 6 slower
  efficiency cores) and rustc 1.98.1, with nothing else running or building.
- **Inputs:** two rockets. Each flies from a 3 m rail at 85° heading west into a 5 m/s west wind
  at Spaceport America, and each runs to the ground under a 0.6 m drogue at apogee and a 2.4 m main
  at 150 m, as under [Recovery](#recovery-m17a) below. Ten inputs are dispersed: the nine of
  [the guide's example](monte-carlo.md#a-run-of-200-flights), with the same standard deviations,
  plus a standard deviation of 0.2 s on each parachute's deployment lag.
  - **Valetudo** (`rocketpy-valetudo.json`), 9.7 kg on an AeroTech K400C. Apogee 762 m on
    average, peak Mach 0.29 to 0.40.
  - **A 66 mm rocket with a 54 mm motor mount on a Cesaroni K940**, built in the program: 3.4 kg
    on the pad, fiberglass. Apogee 3,688 m on average, peak Mach 1.58 to 1.97, so every flight
    needs the supersonic table.
- No flight failed in any run.

| measurement | before M6.1d | after |
|---|---|---|
| Valetudo: 10,000 flights on 10 threads | 4.59 s | **3.00 s** |
| Valetudo: a flight on one thread | 2.76 ms | 1.91 ms |
| K940: 10,000 flights on 10 threads | 264 s | **9.43 s** |
| K940: a flight on one thread | 193 ms | 5.71 ms |
| laying a flight's design out, either rocket | 0.40 to 0.46 ms, twice | 0.001 to 0.002 ms |
| building the K940's supersonic table | in every flight | once a run, 198 ms |

- **The budget** is 10 s. Valetudo takes 3 s, under a third of it. The supersonic rocket takes
  9.4 s, about 6% inside it; [#285](https://github.com/nrdptel/fusionspace-eridanus/issues/285) lists where
  its remaining time goes. The program was run three times while the last small changes went
  in, each run printing its fastest of three: 2.91 to 3.00 s and 9.33 to 9.47 s. The table is
  the last run, on the finished code. On a busy or slower machine the supersonic run will take
  more than 10 s. Other machines and operating systems haven't been timed.
- **Re-measured on 2026-10-08** at ec9fd1a, on the same machine and rustc with no other build
  running, one run of the same program (each figure its fastest of three):
  Valetudo 3.05 s (1.950 ms a flight on one thread), the K940 9.67 s (5.996 ms), 1.7% to 5.0%
  slower than the table, just past the earlier runs' 2.91 to 3.00 s and 9.33 to 9.47 s; the K940
  now sits 3% inside the 10 s budget. `hpr mc --runs 10000` on the level-1 guide's `.ork`, a release
  build, took 3.90 s and peaked at 20.4 MB resident (`/usr/bin/time -l`); one `hpr sim` peaked at
  6.3 MB. Criterion's "Valetudo K400C to the ground" read 1.344 ms, against 1.385 ms after M1.8e6
  (2026-09-19, below).
- **The flow angles' cosines and sines from the velocity** ([#285](https://github.com/nrdptel/fusionspace-eridanus/issues/285)'s
  first item, 2026-10-09). Measured in one sitting, the code before the change (1655a0d) first, then
  after:
  - Valetudo: 3.03 → 2.98 s (1.950 → 1.923 ms a flight).
  - The K940: 9.73 → 9.51 s (5.938 → 5.837 ms a flight).
  - Criterion's "K400C to the ground": 1.3451 → 1.3171 ms, −2.1%.

  A second run after the change read 3.00 s and 9.48 s. The two `atan2` calls are still there,
  because the aerodynamic model needs the angles themselves. Removing them would need
  `hpr-aero`'s flow to carry the cosines too.
- **Before** is the same program, run on 5c7f3d2 (the commit before M6.1d) in the same sitting,
  minus the lines of its one-thread breakdown that time steps the old code didn't have. The
  layout row's "before" is the new program's time to lay a design out alone, which the old code
  did twice per flight; the old program couldn't time it on its own.
- **The flights are unchanged.** Both columns give the same apogee and Mach statistics, to the
  digits printed. The unit test `samples_share_the_nominal_table_and_fly_as_alone`
  (`crates/hpr-analysis/src/montecarlo.rs`) holds a supersonic sample to the same inputs flown
  alone, bit for bit, on one, two and five threads.
- **What changed:**
  - **One supersonic table a run.** Before, each flight built its own table on its first pass of
    Mach 1.2: most of its 193 ms. A dispersion changes masses, motors, drag and weather,
    never the rocket's shape, and the table follows from the shape alone. So a run's flights
    share the nominal design's table (`AeroModel::share_supersonic_table`). The first flight that
    needs it builds it, and the others wait for it.
  - **One layout a run.** Building a flight laid its design out twice, once for its checks and
    once to assemble it. Now a flight takes the nominal design's layout and redoes only its
    stages' masses (`LaidOut::relay`).
  - **No copy of each evaluation.** The integrator asked for the state's evaluation, a few
    hundred bytes, by copying it out of a cache; now it borrows it.
- **Threads.** Ten threads fly 6.0 times as fast as one on the supersonic rocket, and 6.4 times
  on Valetudo, not 10 times, because six of the ten cores are the slower efficiency cores.

## Build memory during a full build

- **Benchmark:** `scripts/build-memory.sh`, which builds into an empty `CARGO_TARGET_DIR` and sums
  the resident memory of the build's process group once a second. 2026-09-19 on an Apple M5
  (10 cores, 16 GB) with rustc 1.98.1. Each row is the mean of its repetitions. Summing processes
  double-counts the pages they share, so compare rows with each other rather than reading one as
  exact, and a one-second sample can pass over a shorter spike, so these are floors.
- **Why:** a run window ended with macOS suspending applications for want of memory. These numbers
  say how much of a 16 GB machine the build accounts for.

Compiling only, `cargo test --workspace --all-features --no-run`, three repetitions each:

| setting | peak memory | wall time | written to `target/` |
|---|---|---|---|
| one job per core, `debug = 2` | 2.55 GB | 11.7 s | 1201 MB |
| `CARGO_BUILD_JOBS=6`, `debug = 2` | 1.62 GB | 13.7 s | 1200 MB |
| one job per core, `debug = "line-tables-only"` | 2.37 GB | 11.3 s | 1037 MB |

Compiling **and running** the tests, `cargo test --workspace --all-features`, two repetitions each
(`HPR_MEASURE_RUN=1`). `CARGO_BUILD_JOBS` does not reach test execution (each test binary
otherwise runs one thread per core), so the second row caps `RUST_TEST_THREADS` too:

| setting | peak memory | wall time |
|---|---|---|
| one per core | 2.58 GB | 25.5 s |
| `CARGO_BUILD_JOBS=6` and `RUST_TEST_THREADS=6` | 1.72 GB | 29.0 s |

- **What this says.** Building *and* running the whole suite peaks at 2.58 GB, barely above
  compiling alone. So the build contributes to memory pressure but is not the main consumer on a
  16 GB machine; what else is open matters more. Capping both to six takes 0.86 GB off that peak
  for 3.5 s on a 25 s run, which is why the development machine's build scripts export
  `CARGO_BUILD_JOBS=6` and `RUST_TEST_THREADS=6`.
- **What it rules out.** Thinning debug information does not reliably cut peak memory. Across two
  sittings the default came in at 2.42, 2.47 and 2.55 GB and `line-tables-only` at
  2.53 and 2.37 GB, overlapping ranges, so the 0.18 GB gap in the table above cannot be told
  apart from run-to-run spread. It does cut what is written to `target/` by 14%, consistently.
  That is a disk saving rather than a memory one, so no profile was changed.
- **Not measured here:** `cargo doc`, `cargo xtask site` (which builds rustdoc into a second
  target directory) and `cargo xtask validate`.
- Numbers taken with those variables set inherit that six-job cap. Measure from a plain shell,
  or set `CARGO_BUILD_JOBS` explicitly, when comparing against the rows above.

## A flare through the method (M1.8e17)

- **Measured:** the supersonic table of a rocket with a conical flare, timed by hand (a throwaway
  release-mode test, best of three), 2026-09-20 on an Apple M5.

| measurement | time |
|---|---|
| a 2° flare, never drawn out, built once per model | 389 ms |
| a 30° flare, drawn out at most rows, built once per model | 716 ms |
| the same rocket on `SupersonicFlare::SlenderBody` | 4.8 µs |

- **What changed.** A flared rocket used to have no table at all: a flare ended the method's run,
  so the third row is what every flared rocket paid before this milestone. Now it builds one, and
  a flare whose corner needs drawing out costs roughly twice a plain one, because each row marches
  the body ahead of the flare for the flow at its corner and then lays out and marches a second
  body with the flare drawn. Both are one-time costs behind the table's `OnceLock`, paid only once
  a flow passes Mach 1.2 and shared by a model's clones. A Monte Carlo run
  ([M6.1a](decisions-and-roadmap.md#m6-1a)) builds a model per flight, and since
  [M6.1d](decisions-and-roadmap.md#m6-1d) its flights share the nominal flight's table, so a
  supersonic design pays for it once a run (Ten thousand flights, above). Two obvious savings are
  left on the table for a later milestone: reading the flow at an
  interior station instead of marching the fore body again, and swapping the last segment of one
  body instead of rebuilding it.

## Blunt tips faster than sound (M1.8e7)

- **Measured:** the supersonic table of a rocket with a vertical nose tip, timed by hand (a
  throwaway release-mode test, best of three), 2026-09-19 on an Apple M5.

| measurement | before | after |
|---|---|---|
| Calisto's supersonic table (von Kármán nose, boattail), built once per model | none: slender-body theory | 363 ms |

- **What changed.** A vertical tip (power-series, Haack or elliptical nose) now flies the
  shock-expansion method behind a Newtonian cap, so its rocket builds the table a pointed nose
  already did: about 125 marches of the method, each laying out the nose's elements again from
  the cap's handover, which moves with the Mach number. The table is built only once a flow passes
  Mach 1.2, so a subsonic flight never pays for it, and a model's clones share it. Pointed noses
  are unchanged.

## Body lift and the measured boattail (M1.8e6)

- **Benchmarks:** `cargo bench -p fusionspace-hpr-aero --bench normal_force -- normal_force` and
  `cargo bench -p fusionspace-hpr-sim --bench flight -- "K400C to the ground"`, criterion, release profile,
  2026-09-19 on an Apple M5, `main` before M1.8e6 and the milestone's branch run back to back.
  The supersonic table was timed by hand (a throwaway release-mode test, best of three).

| measurement | before | after |
|---|---|---|
| `AeroModel::normal_force`, synthetic two-stage, Mach 0.6 at 0.05 rad | 26.8 ns | 51.2 ns |
| `AeroModel::normal_force`, Calisto, the same flow | 23.1 ns | 44.0 ns |
| Valetudo K400C to the ground | 1.321 ms | 1.385 ms |
| NDRT 2020's supersonic table, built once per model | 316 ms | 616 ms |

- **Where the time goes.** Body lift now takes Jorgensen's factor at the flow's crossflow Mach
  number: two table lookups and two divisions per evaluation, where Galejs's was a constant. A
  whole flight pays 4.9% more, well under the M1.6 budget of 5 ms. A fast path below crossflow
  Mach 0.2, where most flights stay and both tables are straight lines, would win most of it back.
- **The table.** A boattail the shock-expansion method covers now marches the forebody twice per
  row, once as it is and once with a cylinder in the boattail's place
  ([issue #98](https://github.com/nrdptel/fusionspace-eridanus/issues/98)); bodies without one are unchanged.

## Normal-force tables (M1.8d)

- **Benchmark:** `cargo bench -p fusionspace-hpr-sim --bench flight -- "K400C to the ground|normal-force
  table"`, criterion, release profile, 2026-09-19 on an Apple M5 with rustc 1.98.1.
- **Input:** the M1.6b flight below, and the same flight on a table of hpr's own normal force at
  0°, 2° and 4° and every Mach 0.01 from 0 to 1, the shape of a RASAero II export.

| flight | median |
|---|---|
| Valetudo K400C to the ground | 1.122 ms |
| the same on a normal-force table | 1.502 ms |

- **Where the time goes.** With a table, each component is evaluated twice per derivative
  evaluation, in its local flow and in the center of mass's, and the table is looked up once:
  34% more here, 3.3 times under the M1.6 budget of 5 ms. The second evaluation repeats the
  center of mass's flow angles for every component; passing them in would save part of it if a
  table flight ever becomes a hot path.

## Roll (M1.8c)

- **Benchmark:** `cargo bench -p fusionspace-hpr-aero --bench normal_force -- roll`, criterion, release
  profile, 2026-09-19 on an Apple M5 with rustc 1.98.1.
- **Input:** `synthetic-two-stage-75mm-54mm.json`'s two fin sets.

| call | median |
|---|---|
| `AeroModel::roll`, Mach 0.6 | 10.5 ns |
| `AeroModel::roll`, Mach 1.0 (the transonic join) | 6.1 ns |
| `AeroModel::roll`, Mach 2.0 | 35.2 ns |

- **Where the time goes.** Each fin set's span moments and the join's two ends are built with the
  model (`FinRollTerms`), so below Mach 0.8 and on the join a call is a few multiplications per
  set. Faster than sound each set clips its outline against the tip's Mach cone twice, as the
  normal force does. The code review measured the first version, which rebuilt those terms on
  every call, at up to 670 ns for elliptical fins (a 257-sided outline); now only the clip scales
  with the outline.

## Recovery (M1.7a)

- **Benchmark:** `cargo bench -p fusionspace-hpr-sim --bench flight`, criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** the M1.6b flight below, with a 0.6 m flat circular drogue at apogee (0.5 s lag) and a
  2.4 m main at 150 m (1 s lag) that releases the drogue, both filling by Knacke's law.

| call | median |
|---|---|
| `Simulation::run`, Valetudo K400C with a drogue and a main to the ground | 0.88 ms |

- **A recovered flight is cheaper than a ballistic one** (the M1.6b row re-measured at 1.17 ms in
  the same run), although it lasts 61.3 s against 29 s: it reaches the ground at 6.57 m/s.
- **Work.** 2,085 derivative evaluations, 328 accepted steps and 14 rejected, against the
  ballistic flight's 2,578 and 410. The descent phase evaluates no airframe aerodynamics, and
  after burnout the mass properties need no central differences, so a descent step is cheaper as
  well as longer than the ballistic dive it replaces.
- **The recovery scan** (the numeric triggers) adds **24** evaluations over the whole flight, one
  per integration interval while a device is pending, shared by every pending device. `Stats` does
  not count them, so they are 1.1% of the work above and not in that 2,085.
- **The 5 ms budget** for a Level 2 flight therefore also holds with recovery, 5.7 times under it.

## Flight (M1.6b)

- **Benchmark:** `cargo bench -p fusionspace-hpr-sim --bench flight`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:**
  - Valetudo (`rocketpy-valetudo.json`, K400C, 9.7 kg), from a 3 m vertical rail at Spaceport
    America.
  - The 1976 standard atmosphere, 5 m/s wind from the west.
  - The default settings (Dormand–Prince 5(4), `rtol = atol = 1e-8`), from ignition to the ground
    (29 s, apogee 874 m, when measured).
  - Since M2.1b2 the motor gets no pressure-thrust correction at the site, so the same inputs now
    reach 779 m and the ground at 27.5 s. The flight timings here and under Recovery (M1.7a) above
    predate that change.

| call | median |
|---|---|
| `Simulation::run`, Valetudo K400C to the ground | 1.10 ms |
| `Simulation::run`, Valetudo K400C with every channel recorded at 10 ms | 2.49 ms |

- **The M1.6 budget** is 5 ms for a typical Level 2 flight, so this is 4.5 times under it.
- **Work.** 2578 derivative evaluations, 410 accepted steps and 15 rejected, so about 0.4 µs per
  evaluation including the event checks. The event functions reuse the evaluation cached for the
  step's last stage.
- **Where the time goes.**
  - Each evaluation calls `Assembly::mass_properties` four times, for the value and the central
    differences of its rates. That is most of the cost, at about 60–100 ns each (M1.4b).
  - The other calls are one geodetic conversion, the atmosphere and wind, drag (50–110 ns) and a
    normal force per component (about 25 ns each).
  - Analytic motor rates would remove three of the four mass calls.
- **Recording** every channel every 10 ms adds 2900 full evaluations, 1.4 ms.
- **Tolerance.** Loosening to 1e-6 halves the time (0.59 ms), at a 7e-5 m apogee error
  (`docs/physics/flight.md`).

## Drag (M1.5b)

- **Benchmark:** `cargo bench -p fusionspace-hpr-aero --bench drag`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** the two layouts below, at Mach 0.6, `α` 0.05 rad, sea-level Reynolds number, one
  54 mm motor thrusting; and Calisto with a 200-row override table.

| call | median |
|---|---|
| `AeroModel::drag`, synthetic two-stage | 107 ns |
| `AeroModel::drag`, Calisto | 47 ns |
| `AeroModel::drag`, Calisto with a 200-row table | 13 ns |

- **Where the time goes.** Each component's skin friction takes a logarithm and a power, and each
  fin set a power for its leading edge; the table is a binary search.
- **After M1.8b1** (2026-09-18, same machine and inputs): 119 ns, 50 ns and 11.3 ns. Each nose,
  shoulder and step now evaluates its pressure-drag curve, a power at Mach 0.6 (eq. 3.87), and a
  power and a logarithm on Stoney's measured shapes from Mach 0.8. A new point, Calisto coasting
  at Mach 1.5 (`AeroModel::drag, Calisto at Mach 1.5`), takes 110 ns: its von Kármán nose is on
  Stoney's curve there, and the supersonic friction and stagnation terms take more powers.
- **After M1.8b3** (2026-09-18, same machine and inputs; `main` measured in the same sitting in
  brackets): 117.5 ns (119.1), 54.7 ns (50.3) and 11.0 ns (11.4); Calisto at Mach 1.5, 239.8 ns
  (108.2). Faster than sound a boattail's drag inverts the Prandtl–Meyer function, a bracketed
  Newton iteration of about ten evaluations: the 132 ns. A rocket without a boattail, or below
  Mach 0.8, doesn't pay it; a supersonic flight of a few thousand evaluations pays well under a
  millisecond. A table of each boattail's drag against Mach number, built with the model, would
  remove it if it ever matters.
- **Normal force after M1.5b.** Re-measured in the same sitting: `AeroModel::new` 3.86 µs and
  11.2 µs, `normal_force` 28.0 ns (two-stage) and 22.2 ns (Calisto), where `main` before M1.5b
  measured 14.8 ns and 11.1 ns in that sitting (the M1.5a numbers below came from an earlier
  sitting). Nothing on the normal-force path changed. Removing the new drag fields from
  `AeroModel` recovered part of it (21.4 ns and 19.5 ns), and `#[inline]` on the hot helpers
  changed nothing, so it looks like code layout; M1.6's flight benchmark will show whether it
  matters.

## Normal force (M1.5a)

- **Benchmark:** `cargo bench -p fusionspace-hpr-aero --bench normal_force`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** resolved layouts of `synthetic-two-stage-75mm-54mm.json` (two fin sets, two
  transitions) and `rocketpy-calisto-getting-started-motor-at-minus-1.255.json` (von Kármán nose,
  boattail, one fin set), at Mach 0.6, `α` 0.05 rad.

| call | median |
|---|---|
| `AeroModel::new`, synthetic two-stage | 3.54 µs |
| `AeroModel::new`, Calisto | 10.7 µs |
| `AeroModel::normal_force`, synthetic two-stage | 11.0 ns |
| `AeroModel::normal_force`, Calisto | 8.5 ns |

- **Where the time goes.** Building the model integrates each nose and transition's filled volume
  and planform once. Evaluation is a sum over components with one square root per fin set.

## Design tree and assembly (M1.4b)

- **Benchmark:** `cargo bench -p fusionspace-hpr-design --bench design`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** from `validation/designs/`, `synthetic-two-stage-75mm-54mm.json` (17 components,
  two bundled motors) and `rocketpy-calisto-getting-started-motor-at-minus-1.255.json` (6
  components, a 2 mm von Kármán nose wall, BATES grains).

| call | median |
|---|---|
| `Rocket::layout`, synthetic two-stage | 654 µs |
| `Rocket::layout`, Calisto | 2.38 ms |
| `Assembly::mass_properties(t)`, two motors | 62 ns |
| `Assembly::mass_properties(t)`, Calisto (BATES grains) | 54 ns |

- **Where the time goes.** Resolving a design costs what its walls cost (M1.4a below): Calisto's
  nose wall is 2 ms of its 2.4 ms. Resolve once per design.
- **Per step.** The flight engine calls `mass_properties(t)` at each derivative evaluation. At about
  60 ns, a few thousand evaluations cost well under a millisecond of M1.6's 5 ms budget. Folding
  the motors into the structure pairwise, instead of collecting them into a `Vec`, took it from 85
  and 118 ns (review fix).

## Mass properties from geometry (M1.4a)

The calls a design edit makes, or a Monte Carlo sample that perturbs dimensions.

- **Benchmark:** `cargo bench -p fusionspace-hpr-design --bench mass`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** a 4-inch (101.6 mm) von Kármán nose 5 calibers long, and a tangent-ogive boattail
  from 101.6 to 76.2 mm over 0.1 m, each filled or with a 2 mm wall. The fins are four
  trapezoidal G10 fins (0.2/0.08 m chords, 0.11 m span) with an airfoil section, and the same set
  as a five-point freeform outline with rounded edges.

| call | median |
|---|---|
| `revolve`, von Kármán nose, filled | 10.1 µs |
| `revolve`, von Kármán nose, 2 mm wall | 2.00 ms |
| `revolve`, ogive boattail, 2 mm wall | 274 µs |
| `NoseCone::mass_properties`, 2 mm wall | 1.99 ms |
| `FinSet::mass_properties`, trapezoidal airfoil | 305 ns |
| `FinSet::mass_properties`, freeform rounded | 11.2 µs |

- **Wall cost.** Each quadrature node finds the inner radius by a 32-point scan and a
  golden-section search, about 70 profile evaluations. The first version refined the minimizing
  station to 1e-15 t and scanned 256 stations for where the wall fills in: 2.25 ms. Near the
  minimum the value is quadratic in the station, so 1e-9 t is enough (1.84 ms), and a 64-station
  scan still brackets the one fill-in point a nose has (1.35 ms), with every test unchanged.
  The review fixes first made the end points candidates and split at `t` from each end (1.70 ms).
  Then they dropped tangent extensions and split wherever the nearest surface point moves between
  the surface and a rim, which needs a 128-station scan (2.00 ms). The filled nose went from 8.2 to 10.1 µs with the
  Haack series and the precise arc.
- **Where it matters.** Mass properties are computed once per design, not per derivative
  evaluation. A 10,000-sample Monte Carlo run that perturbs nose dimensions would spend about
  20 s in walls; M6.1 can cache or perturb mass directly.

## Solid motors (M1.3)

The motor calls the flight engine makes on every derivative evaluation, and reading motor data.

- **Benchmark:** `cargo bench -p fusionspace-hpr-motor --bench motor`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** the bundled Loki M1378LR curve (46 points, 4.08 s) at t = 2.345 s. The grain model
  is four BATES grains, 0.256 m long. The `.rse` input is the same curve written as a RockSim
  file.

| call | median |
|---|---|
| `ThrustCurve::thrust_n` | 3.54 ns |
| `SolidMotor::state`, envelope column | 12.7 ns |
| `SolidMotor::state`, BATES grains | 33.3 ns |
| `eng::parse`, one bundled curve | 3.47 µs |
| `rse::parse`, the same curve | 10.9 µs |
| `Catalog::bundled` (index only, 32 motors) | 33.3 µs |

- **Inner-loop cost.** About 10⁴ derivative evaluations per flight at 33 ns is 0.3 ms.
- **Grain solve.** The first version took 441 ns: at the root, Newton's step landed on the edge of
  its bracket, and the solver then bisected about 50 more times. Stopping once a step is within
  rounding brought it to 28 ns with the same results (33 ns once NaN inputs were passed through).
- **`.rse` line numbers.** Looking each node's line up in the text again made reading quadratic
  (1.2 s for 1.35 MB). A table of line starts, built once, fixed it; this curve went from 14.2 to
  10.9 µs.

## Atmosphere, wind and turbulence (M1.2)

The calls the flight engine makes on every derivative evaluation, and one gust-field build.

- **Benchmark:** `cargo bench -p fusionspace-hpr-atmos --bench atmosphere`, which is criterion, release
  profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Inputs:** 4.4 km above sea level. The sounding has 30 humid levels with wind, from a 1400 m
  site to 12 km, and its pressures above the first level are filled in.

| call | median |
|---|---|
| `Ussa76::sample` | 9.14 ns |
| `SoundingProfile::sample`, 30 levels | 21.7 ns |
| `LayeredWind::wind`, 30 levels | 7.49 ns |
| `LayeredWind::wind`, 1 level | 3.64 ns |
| `GustField::gust` | 1.87 ns |
| `DrydenGenerator::advance`, 1 m | 55.3 ns |
| `GustField::generate`, 20 km at 1 m (20 001 samples) | 1.10 ms |

- **Inner-loop cost.** About 10⁴ derivative evaluations per flight, each calling a 30-level
  sounding, its wind and a gust lookup, is about 0.3 ms: well inside M1.6's 5 ms budget.
- **Generation cost.** Most of a generator step is the incomplete-gamma series and five normal
  draws, 55 ns in all. A gust field is built once per flight, or once per Monte Carlo sample.

## Earth model (M1.1)

The calls the flight engine makes on every derivative evaluation.

- **Benchmark:** `cargo bench -p fusionspace-hpr-core --bench earth`, which is criterion, release profile.
- **When and where:** 2026-09-17 on an Apple M5 with rustc 1.98.1.
- **Position:** 3 km up and a few hundred meters downrange of a 1400 m site.

| call | median |
|---|---|
| `Earth::gravity_enu_mps2`, `constant` | 2.45 ns |
| `Earth::gravity_enu_mps2`, `vertical_taylor` | 3.81 ns |
| `Earth::gravity_enu_mps2`, `ellipsoidal` (the default) | 35.5 ns |
| `Earth::gravity_enu_mps2`, `vertical` | 53.4 ns |
| `Earth::rotation_acceleration_enu_mps2` (Coriolis) | 1.3 ns |
| `LaunchFrame::geodetic_from_enu` (Karney's closed form) | 27.7 ns |

- **Inner-loop cost.** A 6-DOF flight of a few thousand accepted steps evaluates the derivative
  about 10⁴ times. The default model costs about 0.4 ms of that, well inside M1.6's 5 ms budget.
- **Series cut-off.** The `q`/`q′` series stops once its terms fall below rounding. At the Earth's
  eccentricity that takes about 8 terms, where a fixed 40 took 53 ns for the `ellipsoidal` model.
- **Why `vertical` is slower than `ellipsoidal`.** It builds the full vector at a geodetic point,
  converts that point to ECEF, and then rotates the result.
