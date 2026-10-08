# ADR-134: Monte Carlo dispersion: independent normals, one stream per sample and input (2026-10-01)

- **Status:** accepted
- **Summary:** M6.1a: Monte Carlo dispersion in `hpr_analysis::montecarlo`; each input an independent normal about its nominal value; one random stream per sample, input and copy, keyed by `SeededRng::for_stream`; impulse dispersed with the propellant mass; a drag scale in the aero model; failed samples kept; M6.1 split a to d

**Context.** M6.1 asks for seeded, parallel dispersion over mass and CG, drag, motor impulse and
timing, wind, launch angle and deployment delays; landing ellipses; Morris and Sobol sensitivity;
and 10,000 flights of an L2 design in 10 s. Its *done when* is three bullets, and it carries five
Loft lessons: L52 (thrust scaled without propellant mass), L53 (one random stream for a whole run),
L54 (failed samples dropped), L55 (bearings drawn uniformly, the forecast thrown away) and L96 (the
same seed and zero dispersion). That is more than one pull request.

**Decision.**

1. **M6.1 splits in four**, each with its own *done when* in `ROADMAP.md`: a, the dispersion and
   its reproducibility, with all five lessons' tests; b, landing ellipses against analytic
   Gaussians; c, Morris and Sobol against functions with known indices; d, the 10,000-flight
   timing.
2. **Each input is an independent normal about its nominal value**, its standard deviation given
   by the user and zero by default. This is RocketPy's default reading of a `(nominal, standard
   deviation)` pair (`rocketpy/stochastic/stochastic_model.py:190-199`, v1.13.0). Fractions for quantities that
   scale (mass, drag, impulse, burn time, wind speed), absolute values for the rest. There are no
   presets: NFPA 1125 (the 2019 edition's §8.1.7 and §8.2.7, as quoted in its next revision's
   public first-draft documents, whose first revisions keep them) bounds a motor type's impulse
   spread at 6.7% and a delay's error at 1.5 s or 20%, capped at 3 s, but a bound is not a spread;
   four NAR certification sheets measured 1.3% to 3.1% in impulse and 1.5% to 18% in burn time.
   The guide gives these and leaves the choice to the user.
3. **One stream per sample, input and copy.** `SeededRng::for_stream(seed, &[sample, input,
   copy])` folds the key into one word with SplitMix64's output function and seeds xoshiro256++
   from it, the counter-based idea of Salmon et al. (SC11). A sample is then the same whatever the
   run's length, its thread count, or the other inputs dispersed. Jumping one generator ahead
   (xoshiro's `jump`) would also give disjoint streams, but costs a jump per sample and ties
   sample `k` to the order streams are handed out. Two keys collide only by a 64-bit hash
   collision. An input with zero dispersion draws nothing and changes nothing, so zero dispersion
   flies the nominal flight bit for bit (L96).
4. **Impulse scales the thrust and the propellant mass together** (`dispersed_motor`), keeping
   `I/m_p` (L52): a column's mass, BATES grains' density so the grain geometry is kept. Burn time
   stretches the curve and divides the thrust by the same factor, keeping the impulse. A cluster is
   one draw, because the design holds it as one motor.
5. **Drag is scaled in the aero model** (`AeroModel::with_drag_scale`,
   `Simulation::with_drag_scale`), multiplying whatever gives the zero-lift coefficient: the
   buildup, a table or a drag model. Wrapping the drag in a drag model instead would have made it
   the whole stack's, refused at a powered separation; the scale passes to the sustainer instead.
6. **Wind is turned about its own direction** (`DispersedWind`): the base model's velocity at every
   height, scaled and turned clockwise, so a profile keeps its shape and the forecast's heading is
   the mean (L55). The rail's heading is likewise an offset. A rail drawn past vertical leans the
   other way (`E′ = π − E`, heading and roll turned half a turn), the same attitude.
7. **Delays and the wind's speed are cut at zero**, since a charge can't fire before its event
   and a wind can't blow at less than calm (failing those samples would bias the run towards
   strong winds); every other impossible draw (a negative mass, a rail below the horizon) fails its
   sample, which is kept with its reason and where it failed (`FailedAt`), and counted (L54). A
   probability is reported as two bounds, failures counted as failing and as passing (`Share`).
8. **Statistics on sorted values**: mean and standard deviation shifted by the smallest value
   (Chan, Golub and LeVeque, 1983), quantiles by Hyndman and Fan's definition 7. Sums run over the
   sorted values, so a summary is the same however the samples were flown.
9. **Parallelism is rayon behind `hpr-analysis`'s `parallel` feature**, off by default and never
   built for wasm32 (`ARCHITECTURE.md`'s plan, `clippy.toml`'s rule). Its `collect` keeps the
   samples in order. `run_parallel` uses the caller's pool, so an optimizer calling it per
   candidate starts no threads, and the thread count is the caller's `ThreadPool::install`.
10. **The inputs are `FlightInputs`**, the arguments of `Simulation::new` plus recovery, a drag
    override and the drag scale, because `Simulation` is neither `Clone` nor open. Events,
    separations and staging a `Simulation` can be given aren't in it yet; `hpr::FlightBuilder::inputs`
    builds it, and `FlightBuilder::simulation` now goes through it.
11. **A `Draw` says what a sample flies.** `MonteCarlo::inputs` applies every entry that isn't at
    its nominal value (a factor of 1, an offset of 0), whatever the dispersion, and checks the
    lists' lengths against the rocket, so a draw read back or written by hand flies as it says. An
    entry at its nominal value changes nothing, which keeps zero dispersion bit for bit. The
    public structs (`Dispersion`, `Draw`, `FlightInputs`) are not `#[non_exhaustive]`, so callers
    can write them with `..Default::default()`; adding a dispersion is then a breaking change,
    acceptable before 1.0. `DragOverride` is `#[non_exhaustive]`.

**Consequences.** M6.1a is met (`crates/hpr-analysis/src/montecarlo.rs`'s tests, the guide's
`docs/monte-carlo.md` and its example). Each sample rebuilds its `Simulation`, so a supersonic
design rebuilds its supersonic tables every flight (300 to 700 ms each, `docs/perf.md`); M6.1d has
to share them. Not dispersed: the atmosphere, ignition times, recovery devices' drag, correlations
between inputs. No measured set of repeated flights has checked the spread a run gives.
