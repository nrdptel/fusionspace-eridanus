# ADR-140: Discrete choices by CMA-ES with margin (2026-10-02)

- **Status:** accepted
- **Summary:** M6.2b2: integer variables (`Variable::integer`) by CMA-ES with margin (Hamano et al., GECCO 2022), α = 1/(nλ); `Φ` by `libm::erfc`, `Φ⁻¹` by AS 241; held to SphereInt, EllipsoidInt and SphereOneMax at 10 and 20 variables and to `cmaes` 0.13.1's CMAwM; the rocket example hits 3,048 m with the motor and nose free, its body length fixed so designs share a supersonic table

**Context.** M6.2b2 asks for discrete choices, a catalogue motor and part, chosen with continuous
variables under margin and rail-exit limits: "hit 3,048 m" with the motor free, each limit
re-checked on the winner by flying it again. CMA-ES (ADR-138) draws real numbers.

**Decision.**

1. **Integer variables, by CMA-ES with margin** (R. Hamano, S. Saito, M. Nomura, S. Shirakawa,
   GECCO 2022, arXiv:2205.13482v2, §4, eqs. (12) to (24), Algorithm 1, pp. 5–6 and 10; the TELO
   version, arXiv:2212.09260v2, read to confirm). `Variable::integer` takes the whole numbers
   between whole bounds; a draw is encoded to the nearest, a tie to the lower, clamped (§4.1); the
   update sees the real draws; after it, each integer variable's mean and diagonal `A` are
   corrected so a draw keeps a chance `α` of leaving the mean's value (`α/2` each side of an inner
   value). Chosen over enumerating the choices with a continuous run each (exact for a few
   choices, but its cost multiplies with every list) and over pycma's integer handling (a step-size
   floor, Hansen 2011), as the margin method is the one with a published per-generation guarantee
   and an outside implementation to test against. A choice with no order is a whole number in a
   list the user orders; categorical handling is left out.
2. **Defaults and edges.** `α = 1/(n λ)`, the paper's (§5.1, Fig. 4; TELO footnote 2: `n` is every
   variable). A start may lie between whole numbers (the paper's starts are U[1, 3]). Integer
   draws are never redrawn for bounds, which the encoding enforces. At a mean exactly on a
   two-value threshold, eq. (13) leaves `m` and `A` unchanged; `cmaes` 0.13.1 sets `A = 0` there
   (a measure-zero case), and its clipping of `p″` to `[10⁻¹⁰, 0.5 − 10⁻¹⁰]` is not in the paper and
   not copied. Where nothing needs lifting (an end mean within `Φ⁻¹(1 − α)` spreads of its
   threshold, or both sides of an inner one at `α/2` or more) the mean and `A` are left as they
   are, which eqs. (13) and (17) to (24) would return up to rounding; a moved end mean that rounds
   past its reach is stepped one ulp back. The encoding is exact rounding, `⌊x⌋` or `⌊x⌋ + 1`, as
   `⌈x − 0.5⌉` and `x − ⌊x⌋` can round, and gives 0, not −0; the correction finds the mean's value
   the same way. Integer bounds are whole numbers within ±2⁵², where every threshold `k ± 0.5` is
   an `f64`; `within` checks them again when called after `integer`. `Φ` is `erfc` from `libm` (MIT OR Apache-2.0, already in the tree), `Φ⁻¹`
   Wichura's AS 241 (1988), its coefficients checked against the paper's hash sums. They were
   transcribed from the listing in CPython's `statistics` module (PSF License 2.0); only the
   paper's published constants were taken, and the code around them follows the paper.
3. **Held to** SphereInt, EllipsoidInt and SphereOneMax (§5.1), half the variables integer, at 10
   and 20 variables, from the 20 starts `validation/oracles/cmawm/cmawm_runs.py` records: every run
   reaches `f ≤ 10⁻¹⁰` with every integer exact, and the median evaluations are within 25% of
   `cmaes.CMAwM`'s with its negative weights zeroed (ours have none), the bound ADR-138 set
   against pycma. Measured: −4.6% to +2.7%. `cmaes` takes `c_μ` without Table 1's 1/4 (11 to 14%
   smaller; `tests/mixed.rs` checks `λ`, `α`, the weights, `μ_eff`, `c₁`, `c_c`, `c_σ`, `d_σ`, `E‖N(0, I)‖`
   and Table 1's `c_μ` against the fixture's) and counts `h_σ`'s generation one later; both stay, as ADR-138's parameters are the
   tutorial's. A unit test checks the margin after every generation of a run with a binary, an
   integer at an end value and one inside, and another checks one correction of each kind at a
   set state against the equations in 40-digit arithmetic; it fails with `α` for `α/2`, with `A`
   left at 1, or with the excess not given back. With the correction taken out by hand, SphereOneMax at
   10 and 20 and SphereInt at 20 fail (runs stall); SphereInt at 10 and EllipsoidInt at 10 and 20 still pass.
4. **The rocket problem** (`crates/hpr/examples/motor_and_nose.rs`): a 2.6 in rocket of Madcow
   parts, the motor (five 54 mm, by total impulse) and the nose (four) integer, the ballast and
   fin span continuous; goal the squared miss from 3,048 m; limits from the IREC Design, Test &
   Evaluation Guide 2025 over the whole ascent, rail exit to apogee: a flight ("dynamic") margin
   of at least 1.5 calibres (§10.3.1, read as ADR-077's flight margin), a static margin of at most
   4 and a flight margin of at most 6 (§10.4.1), and 30 m/s off a 3 m rail (§10.2.1). The least
   flight margin is the flight's searched least; the most of each is taken at step ends (exact for
   the static margin, which grows through the burn and then holds; a flight-margin peak between
   step ends could be missed). They run from the rail exit, where the flight metrics start
   (ADR-077), where §10.3.1 says "from launch". A first version held only the static margin at
   launch to 1.5 to 4; its winner rose past 4 after burnout (physics review), so the limits now
   cover the ascent. Seed 2026
   picks the K400C with the plastic 3:1 ogive in 360 flights; flown again, default and
   100-times-tighter tolerances, it is within 0.1 m with every limit kept. Its answer is not
   unique: a scan of every motor and nose over ballast and fin span (development machine, not
   committed) found the J760, K400C and K940 each able to hit 3,048 m within the limits with any
   of the four noses. So the printed choice is the run's, not the problem's; perturbing every
   flight by changing the integrator's relative tolerance by up to 1% left the run's 360 flights
   and its point unchanged, as did moving `σ` by one to three ulps each generation (neither trial
   committed), so the output is expected to match on every platform, and CI checks it on three. Two other goals were
   tried first and dropped, measured on the development machine and not committed: the lightest
   rocket that hits (with the body length free) settled on four different noses from four seeds,
   its optima 1% apart, as an apogee band makes every nose change infeasible; the smallest motor
   that can hit settled on the K400C from seed 2026 though the J760 searched alone met every limit,
   as a goal that changes only between choices gives the search nothing to follow. The guide says
   so and points at one continuous run per choice for such questions.
5. **The body length is fixed** in the example: the 2.6 in rocket passes Mach 1.2, and a flight
   past it builds its shape's supersonic table (0.2 to 0.3 s in release). A free length makes
   every design a new shape; fixed, the example shares one table per nose
   (`Simulation::share_supersonic_table`) and runs in 3.5 s in release and 32 s in debug, where CI
   runs it. The winning plastic nose gets no table: the catalogue lists it 0.05 mm narrower than
   the tube, and the march refuses any step in the outline above 1e-6 of the area (#87), so its
   flight, which tops out at Mach 1.23, uses slender-body theory past Mach 1.2. Its effect on the
   apogee is not measured.

**Consequences.** Any continuous problem runs bit for bit as before (`A` stays 1; the CMA-ES tests
pass unchanged). `Variable` writes an `integer` field only when true, and reads it as `false`
when absent. Left out:
categorical choices, the margin as a setting, and a guarantee of the best of several feasible
choices; NSGA-II (M6.2c) is next.
