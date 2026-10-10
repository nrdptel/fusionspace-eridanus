# Working notes

**What this is.** The code map and working notes: where a subsystem's tools are, what a check
needs, and traps already found. Read the part for the area you are working on; update it when you
learn something the next person needs. Each bullet is one topic; keep the file under 200 lines.

- **Code map and notes:** M6.2d2: EGO on Hartmann 6, which d1's version leaves at −3.20 in 7 runs of
  10 (seeds 1 to 10) after 100 evaluations, 37 s a run in debug; Jones et al. needed 121 and a
  `−ln(−y)` transform (probe: `Ego::new` on six `[0, 1]` variables, `hartmann6`); try fitting `pₖ`,
  capping `θ`, and a cheaper likelihood fit.
- EGO (ADR-142) is `optimize::ego`: `Ego` (two finite bounds, continuous; `minimize` → `Optimum`,
  `Stop`), private `Kriging` (Cholesky, nugget 1e-8; about half the later points land within 1e-3
  of an earlier one, `log₁₀ θ` by CMA-ES in [−3, 3]), maximin Latin hypercube; `benchmark::global`
  (`branin`, `hartmann3`, `hartmann6`);
- `tests/ego.rs` (20 seeds, 50 evaluations, ~35 s debug); page
  `docs/optimization.md#few-evaluations-ego`. NSGA-II (ADR-141) is `optimize::nsga2`: `Nsga2`
  (continuous variables with two finite bounds only; `start` → `candidates` →
  `tell`/`tell_constrained` with `Goals`, or `minimize[_constrained]` → `Front`), `dominates`,
  `generational_distance`, `inverted_generational_distance`;
- `benchmark::zdt::Zdt` (`distance_to_front`, to the curve; `ZDT3_PIECES`);
- `tests/nsga2.rs` holds it to `tests/fixtures/nsga2/pymoo.json`
  (`validation/oracles/nsga2/pymoo_runs.py`, pymoo 0.6.2 at the paper's settings; its moocore, LGPL,
  ranks the oracle's fronts, run-only); example `crates/hpr/examples/pareto_front.rs` (54 s debug;
  front re-flown, CMA-ES at 2.5 cal), page `docs/optimization.md#trade-offs-a-pareto-front`.
- Integer variables (ADR-140): `Variable::integer` (whole bounds, `encode`: nearest, ties down,
  clamped), CMA-ES with margin in `cmaes.rs` (`correct_margin`, `Run::margin_scale`,
  `Run::integer_margin` = 1/(nλ)), private `optimize::normal` (`libm::erfc`, AS 241),
  `benchmark::mixed`;
- `tests/mixed.rs` holds six cases to `tests/fixtures/cmawm/cmawm.json`
  (`validation/oracles/cmawm/cmawm_runs.py`, cmaes 0.13.1, "positive" = weights zeroed); example
  `crates/hpr/examples/motor_and_nose.rs` (2.6 in Madcow rocket, body fixed so a `Flyer` shares one
  supersonic table per nose; limits are IREC's margins over the ascent; 32 s debug; its answer isn't
  unique, J760/K400C/K940 can all hit), page
  `docs/optimization.md#choices-a-motor-a-catalog-part`.
- Constraints (ADR-139): `optimize::Evaluation` (`constrained(value, &[g])`, `rank`),
  `Run::tell_constrained`, `Cmaes::minimize_constrained`, `benchmark::constrained::{sphere_above,
  tangent, g06}`, `tests/constrained.rs`; `Optimum::violation`.
- CMA-ES (ADR-138) is `hpr_analysis::optimize`: `Variable` (start, step, `within` bounds),
  `cmaes::{Cmaes, Run, Optimum, Stop, Parameters}` (`start` → `candidates` → `tell`, `minimize`;
  works in variables ÷ steps), private `eigen` (Jacobi), `benchmark`;
- `tests/optimize.rs` holds it to `tests/fixtures/cmaes/pycma.json`
  (`validation/oracles/cmaes/pycma_runs.py`, pycma 4.5.0); example
  `crates/hpr/examples/optimization.rs` (the 3,048 m J760 rocket), page `docs/optimization.md`.
- Speed (ADR-137): `crates/hpr/benches/ten_thousand.rs` (`cargo bench -p fusionspace-hpr --features parallel
  --bench ten_thousand`) times 10,000 flights; samples share the nominal's layout (`Rocket::lay_out`
  → `LaidOut`, `relay`, `Simulation::from_laid_out`) and supersonic table
  (`share_supersonic_table`), via `MonteCarlo`'s `shared`;
- `MonteCarlo::fly` shares for hand-made draws; the supersonic run is 9.4 s of 10, per-evaluation
  cuts are #285 (they move last bits: regenerate every report).
- Sensitivity (ADR-136) is `hpr_analysis::sensitivity`: `Factor` (uniform range), `morris::{Morris,
  MorrisDesign, ElementaryEffects}` (`design` → `points` → `analyze`, `screen`, exact `population`
  ≤2²⁴ points), `sobol::{Sobol, SobolDesign, SobolIndices}` (Saltelli (b), Jansen (f), outputs
  shifted by their mean, delta-method errors), `benchmark::{Ishigami, SobolG}`;
- `tests/sensitivity.rs` holds both to the closed forms and calibrates the errors over seeds;
  example `crates/hpr/examples/sensitivity.rs` (Morris on the MC rocket via
  `MonteCarlo::fly(&Draw)`), page `docs/sensitivity.md`; flights by hand go through
  `MonteCarlo::fly`.
- Ellipses (ADR-135) are `hpr_analysis::ellipse`: `Run::landing` → `Scatter` (failures counted),
  `ellipse` (`k² = −2 ln(1−p)`), `prediction_ellipse` (Hotelling), `share_inside`. Monte Carlo
  (ADR-134) is `hpr_analysis::montecarlo`: `FlightInputs` (`hpr::FlightBuilder::inputs`;
- `simulation()` goes through it), `Dispersion` (sds, zero draws nothing), `MonteCarlo::{draw,
  inputs, sample, run, run_parallel}` (feature `parallel`: rayon, the caller's pool;
- `inputs` applies any non-nominal `Draw` entry, lengths checked), streams
  `SeededRng::for_stream(seed, &[sample, input, copy])` (the `Input` numbers are part of every
  seeded result), `Simulation::with_drag_scale` (passes to a sustainer); `statistics::Distribution`
  (shifted moments, Hyndman–Fan 7, `Share` bounds); example `crates/hpr/examples/monte_carlo.rs`,
  page `docs/monte-carlo.md`;
- NFPA 1125 and four NAR sheets pinned in `refs.lock.toml`. Catalog parts (ADR-133) are
  `hpr::rocket::catalog`: `from_catalog` on `Nose`, `Tube`, `Transition`, `MotorTube` and the new
  `Fitting` (`Rocket::add_fitting`);
- `tests/catalog_openrocket.rs` holds every part (or counts its gap) to
  `crates/hpr/tests/fixtures/orc/openrocket-built.json`
  (`validation/oracles/openrocket/orc_built.py`: rerun after a reader or builder change); a hollow
  shoulder takes its part's wall (OR's is zero); #280: a nominal 29 mm motor in LOC's 28.956 mm
  bore only warns since ADR-155 (`motor_tight_in_mount`).
- The reader (ADR-132) is `hpr_io::orc`: `read`, `bundled()`, `Catalog::find` / `search`;
  `tests/orc_openrocket.rs` holds it to `crates/hpr-io/tests/fixtures/orc/openrocket-presets.json`
  (`orc_presets.py`). `hpr motors search` (ADR-131) is `crates/hpr-cli/src/motor_search.rs`: the
  finder's list via `weather::client` (shared), `--from` a saved list, `--max-price` exact cents on
  `cheapest_in_stock`;
- `tests/motors_search.rs` filters the recording's JSON itself and edits a temp copy for the $150
  example (the recording has none). ThrustCurve (ADR-130) is `hpr_net::thrustcurve`:
  `fetch_finder_records` (three whole searches), `join` (exact maker + designation; 282 of 282),
  `fetch_download` + `DataFile::read` (`.eng`/`.rse` via `hpr_motor`);
- `validation/reports/thrustcurve-join.md` is `Join::report`, rewritten by
  `HPR_WRITE_THRUSTCURVE_JOIN=1`. The finder (ADR-129) is `hpr_net::motor_finder`, eight answers of
  one build in `tests/fixtures/replay/motor-finder-*`. GeoTIFF (ADR-128) is `hpr_io::geotiff` over
  `tiff` (LZW, Deflate), geographic CRSs near WGS 84 only, the containing pixel as GDAL places it,
  GDAL's scale and offset;
- `validation/oracles/geotiff/dem.py cut|read` writes `crates/hpr-io/tests/fixtures/geotiff/` and
  rasterio's (1.5.2, in `pyproject.toml`) `rasterio.json`; `tests/geotiff_rasterio.rs` runs the
  whole `refs/dem/` tile where fetched; no `hpr` command or flight reads it. Geodesics (ADR-127) are
  `hpr_core::geodesic` over `geographiclib-rs`, `f` past 1/150 refused;
- `tests/geodtest.rs` checks Karney's every-500th and 21 mirror lines in CI and the whole
  `refs/sources/geodtest/GeodTest.dat` locally, its table against `validation/reports/geodesics.md`
  on a macOS debug build only (`HPR_WRITE_GEODESICS=1` rewrites it); nothing in a flight uses them.
- Elevation (ADR-126) is `hpr_net::elevation`, heights above EGM2008 as Open-Meteo gives them, `N`
  left to the caller; no `hpr` command yet. WMM2025 (ADR-125) is `hpr_core::magnetic`, its files in
  `crates/hpr-core/data/wmm2025/`; NCEI's 100-point file's `X` is held to its measured 7.2e-4 nT
  residue (pygeomag 1.1.0 agrees with hpr); nothing in a flight uses the field yet.
- GRIB2 5.40 (ADR-124) goes through `hayro-jpeg2000`, lossless to 21 bits, strict, its header held
  to NCEP's coding; `whole_file.py cut-rap` writes the RAP fixture's reading; a whole RAP file
  through `hpr weather rap --from` is untried.
- `cargo xtask grib2-values FILE` + `whole_file.py compare` check a whole file by hand; the GFS file
  is `refs/gfs/` (AWS `noaa-gfs-bdp-pds`), and `tests/weather.rs` flies it where present. `hpr
  weather` (`crates/hpr-cli/src/weather.rs`) runs the library's readers; `tests/weather.rs` fills a
  cache over `Replay` and runs `--offline`; `hpr sim` flying a profile is #265.
- Sources mirror `hpr_net::nomads`/`wyoming`/`open_meteo`: a request building the URL, a pure
  parser, `fetch` through `Client::fetch_checked`; recordings under
  `crates/hpr-net/tests/fixtures/replay/` (index keyed by the exact URL), never fetched live in
  tests; ecCodes (`validation/oracles/grib2/eccodes_dump.py`, in
  `validation/oracles/pyproject.toml`) reads GRIB2 as the oracle.
- `Http` drops a failed answer's body. HTTPS was checked once by hand, not in CI. Python (ADR-114 to
  116): `crates/hpr-py` wraps the builder; `models.rs` holds Python drag and wind (a call
  re-attaches the GIL; a raised exception is kept in `Raised` and `Flight` raises it).
- `gate.sh python` builds the wheel and runs pytest, `docs/python.md`'s blocks and
  `examples/calisto.py` (its table is in the page) included. Format (ADR-111 to 113):
  `hpr_format::DesignFile` over `hpr_io::ork::Design`; a type change needs `cargo xtask format`
  (schema, TS, Python;
- `cargo test` runs node and python3) and `cargo xtask ork`; one that stops old documents reading
  needs a new minor version, a step in `migrate.rs`, the old schema kept, and a fixture its program
  wrote (ADR-112 §2). `.ork` export (ADR-109, 110): writers mirror readers; after a writer change,
  rerun ork.md's three commands and commit the report.
- Logs (ADR-108): `synthetic-pnut.pf2` is rewritten by `HPR_WRITE_SYNTHETIC_LOG=1`; the public Pnut
  test runs only where `refs/` has Debrief. CLI (ADR-105 to 108): a command goes live by leaving
  `registry::PLANNED`, adding its output type to `output::schemas`, then `cargo xtask cli`; examples
  name repo files from the root, and write bare names to scratch. `hpr sim`'s recovery:
  `hpr::ork::recovery` (ADR-153); a powered split: `hpr::ork::separated_recovery`, `tumbling`,
  `FlightBuilder::separation` (ADR-159); several: `separations`, cut per boundary (ADR-160);
  a lone split with nothing left to burn: `Separation::powered_at`, `RecoveryRefused::Coasts`
  (ADR-165); its comparison flies the climb whole (`ork_flights::climbing`).
- Tube fins: OR's slope and center per part in `openrocket-tube-fin-aero.json` (`tube_fin_aero.py`,
  ADR-102); a new OR with #3235 moves its center past Mach 0.5; body interference open (#234). #185:
  `unlit_motors.py`. Tube-fin drag likely low (#228); public drag curves: `PUBLIC_DRAG_CURVES`'s
  doc. Library runs need `drag_curves.py` (ADR-097).
- Probes (ADR-093, ADR-094): `pod_probes.py`, `rod_probes.py`, then `flights.py` (its docstring's
  command) and `motor_database.py ... refs validation/fixtures/ork/{pod,rod}-flights --jar`. #216: a
  `.ork` part with no `<finish>` gets hpr's 20 µm, OR's 60 µm.
- **Census (ADR-084):** a regenerated report that moves a row needs `cargo xtask census --accept
  --reason "<why>"` in the same PR, or `validate --check` fails. #200: Linux's reproduction bound.
  M2.3c (ADR-083): fly a pair with `hpr_validate::real_flight`, commit only statistics; `xtask
  real-flights --check` needs `refs/rocketpy`.
- Flutter (ADR-078): moduli in `materials::SHEAR_MODULI`; metrics (ADR-077): margin `None` past `κ =
  √10`. `.ork` since M1.9c (ADR-076): ignitions, clusters, powered splits fly (several since ADR-160),
  and a lone unpowered one (ADR-165); open: #185. Leads, not causes: #177, private flights above sea level reading low, `C03`, `C09` margins
  (#172).
- After any physics change run `cargo xtask ork-flights --check`, `--library --check` and
  `real-flights --check`: CI can't fly them; corpus reruns jitter (≤5e-7).
- **Page rules** (ADR-016 to ADR-020): relative links between pages, GitHub URLs for the rest
  (rustdoc too), labels as links to their rows (a lesson a page names needs a row in
  `decisions-and-roadmap.md`), none in headings; new pages in `SUMMARY.md`, a new library a row in
  `docs/api.md`; milestone rows are kept by `cargo xtask records`.
- `ROADMAP.md` has byte and line-width budgets (ADR-147, M0.5c); never reflow or
  shorten a *done when* to fit: done entries move to `docs/roadmap-done.md` by `cargo xtask
  records`, unchanged. (Before M0.5, M2.2a to b4, M3.1c3 and c4 were reflowed into prose.)
- **Validation (M2.1, ADR-021 to ADR-026):** CI checks the report on every platform; predicted mode's 3%
  are *targets*; every whole flight names both RMS metrics, each held to 3% of its reference's
  apogee or max speed (ADR-024). The wind oracle flies RocketPy 1.13.0 with #1188 and #1196 by
  `corrections.py`.
- **Regeneration is not bit-identical**: use `cargo xtask validate` (debug), never `--release`;
  checks allow 1e-12 but **compare a fixture's strings as text**. `cargo xtask aero` rewrites
  `arcas-robin-gap.json`'s last digits whatever you changed.
- **M1.8a to e19** (ADR-027 to ADR-050). Measurements: the guide's aero page and *Known issues*.
  Working notes: `cargo xtask aero` writes the aero fixtures (scratch in `refs/scratch/m18*/`);
  `SupersonicBody` tabulates every 0.05 Mach from max(1.2, its start), joined over 0.3;
- `BEFORE_M1_8E6` keeps the old rules and `CONE_SLOPES` runs to 30°; a mesh-following answer is
  marked by the pressure **crossing** its tangent cone's, not `η < 0`; the near-flat flare's edges
  come from `flare_reduction_turns_rad` (#108).
- **Debrief, folded in** (ADR-046): `hpr-flightdata` is off `hpr-sim` and must stay off it (`forbids
  = ["hpr-sim"]`, walked by `cargo xtask wasm-check`); sim-versus-flight goes in `hpr-forensics`;
  notes in `debrief-{log-formats,flight-readings,porting-boundary}.md`. **Port from its `lib/`,
  never its `COMPETITION.md`** (GPL-3 Java); its 12 public fixtures may be used.
- **`.ork` readings (ADR-052 to ADR-054):** angles are **degrees**, but *which way they turn* is
  assumed (OR's `+x` aft, hpr's `+z` at the nose): on the guide's not-settled list for M2.2. A
  cached `auto` number is what OpenRocket last resolved, never an input (24.12 ignores it on
  reading) and `cargo xtask ork` holds 67 of 71 to it; nothing caches an `outerradius` or
  `innerradius`.
- Finish heights: the author's forum post, in `refs/sources/openrocket-finish/`.
- **Private flight collection `refs/hpr-sim-fixtures`** (ADR-151,
  [rule 4](../../CONTRIBUTING.md#4-private-data-stays-private)): a link to the sibling checkout
  `../hpr-sim-fixtures`, pinned at `31771d8`; `cargo xtask refs verify hpr-sim-fixtures` takes
  ~20 s. Start from its `README.md`, `flights.csv` (a row a flight; `tier`
  A 89, B 289, C 90) and `files.csv` (each file's `role`); each flight's `NOTES.md` lists its gaps.
  Values keep the source's units; `unknown` is unknown. Designs: 395 `.ork`, 107 `.rkt`, 37 `.CDX1`;
  Raven `.FIPa` logs have a decoded CSV beside them; ERA5 for 298 flights, its relative humidity
  about −8 to 163 percent as published (clamp it). Publish aggregate statistics only, never flight
  ids (they name the rocket); ERA5-derived numbers carry the Copernicus attribution. `cargo xtask
  ork` leaves it out unless named with `--dir`; the Python oracles skip it only as a link (#306).
  It may serve M2.3c (tier A pairs), M1.14 (real flights, some supersonic) and M4.5; none started.
- **Process notes:** `cargo test -p xtask` guards ROADMAP, notices, lessons and the lock;
  oracles run from the repo root with `refs/venv/bin/python` (Java 17 for the OpenRocket ones);
  `xtask designs`, `examples` and `ork` rewrite their outputs.
