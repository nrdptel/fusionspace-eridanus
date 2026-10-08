# ADR-138: Optimization: CMA-ES first, held to test functions and to pycma (2026-10-01)

- **Status:** accepted
- **Summary:** M6.2 split a to e; M6.2a: CMA-ES (Hansen's tutorial, Table 1, positive weights only) over continuous variables, worked in each variable divided by its step, with optional bounds by resampling, ask-and-tell (`Run::tell`), Jacobi eigen-decomposition each generation; held to four test functions' minima, to pycma 4.5.0's evaluation counts (medians within 25%) and to a dense recomputation of every generation; the 3,048 m problem re-flown from a fresh build and at 100× tighter tolerances

**Context.** M6.2 asks for an optimization engine: continuous and discrete variables (motor
choice, catalogue parts), constraints, single- and multi-objective goals, CMA-ES, NSGA-II and
Bayesian optimization (EGO), and a robust mode with Monte Carlo in the loop. Done when benchmark
functions converge to known optima within tolerance and a "hit 3,048 m" problem is solved, the
result validated by re-simulation. That is more than one pull request.

**Decision.**

1. **Split in five.** M6.2a CMA-ES over continuous variables, with both of M6.2's done-when tests
   for it; M6.2b discrete variables (a motor, a catalogue part) and constraints (a stability
   margin, a rail-exit speed); M6.2c several objectives (NSGA-II, a Pareto front); M6.2d Bayesian
   optimization (EGO); M6.2e robust designs (a Monte Carlo run's statistic as the objective).
   M6.2 closes when all five have.
2. **CMA-ES as N. Hansen's tutorial gives it** (arXiv:1604.00772v2, 2023): Figure 6's update,
   eqs. (38) to (47), and Table 1's defaults, eqs. (48) to (58), with the negative weights zero,
   as in the tutorial's own code (p. 36). The active variant is left out: one fewer thing to get
   wrong, and the tests' pycma runs are non-active too. `C` is decomposed every generation, which
   the tutorial's lazy schedule (B.2) also gives for up to about 85 variables at the default
   population. The eigen-decomposition is our own cyclic Jacobi (Golub and Van Loan, §8.5), with
   Demmel and Veselić's (1992) stopping test, so the smallest eigenvalue of an ill-conditioned
   covariance keeps its relative accuracy. It is tested against a closed-form spectrum and a
   graded matrix's closed-form determinant. No linear-algebra dependency is added.
3. **Variables carry a start, a step and optional bounds** (`Variable`). The strategy works in
   each variable divided by its step, starting as Figure 6 does (`σ = 1`, `C = I`), the
   tutorial's "a scaling of the variables should be applied" (its footnote, p. 29). A first draft
   started at `C = diag(stepᵢ²)` instead; review found that measures ConditionCov on the unscaled
   `C`, so steps of 10⁻⁴ and 10⁴ stopped a solvable run after one generation. Bounds are handled
   by drawing a candidate again from its own stream until it falls inside, the second of the
   tutorial's two methods for an optimum inside the feasible region (B.5), up to 1,000 draws
   (`Stop::Bounds`, or `OutOfBounds` for the first generation); a candidate that overflows ends
   the run as ConditionCov. No repair, which the tutorial advises against; an optimum on a bound is left to
   M6.2b's constraint handling.
4. **Ask and tell.** `Cmaes::start` draws a generation; `Run::tell(values)` takes the model's
   values in order. So the caller evaluates candidates however it likes, as the sensitivity
   designs allow (ADR-136). `minimize` is the closure shortcut. A value may be `+∞` (a flight that
   fails ranks last); NaN and `−∞` are refused with the evaluation's index.
5. **Streams.** Candidate `k` of generation `g` draws from `SeededRng::for_stream(seed, &[g, k])`,
   resampling included, so a run is bit for bit the same on one platform.
6. **Stops** are the tutorial's TolX (in the scaled variables), TolFun and ConditionCov (B.3), with
   its 10⁻¹² tolerances, plus a finite target value and an evaluation cap (10,000 by default). An
   overflowed `σ` counts as ConditionCov. NoEffectAxis, NoEffectCoord, Stagnation and TolXUp are
   left out. `Cmaes` deserializes through the same checks as its builder; `Parameters` is read
   from a run, not built.
7. **Held to an outside implementation.** pycma 4.5.0 (BSD-3-Clause, the author's), pinned in
   `validation/oracles/pyproject.toml`, runs the four test functions of Hansen, Müller and
   Koumoutsakos (2003, Table 1) at ten variables, from the same starts and steps, non-active, every
   stop but the target (10⁻¹⁰, the paper's) and the cap off, seeds 1 to 20
   (`validation/oracles/cmaes/pycma_runs.py` → `crates/hpr-analysis/tests/fixtures/cmaes/pycma.json`).
   The random numbers differ, so runs are compared by median evaluations, required within 25%
   (set before the first comparison). pycma's `c_σ` and `d_σ` (`n + μ_eff + 3`), its exact
   `E‖N‖`, its `h_σ` test and its step-size clip depart from Table 1 by its author's choice; the
   weights, `μ_eff`, `c₁`, `c_μ` and `c_c` it shares with Table 1 agree to 1e-15.

**Consequences.** M6.2a is met. Medians: sphere 1,635 (pycma 1,640), ellipsoid 5,920 (5,910),
rotated ellipsoid 6,010 (5,930), Rosenbrock 6,445 (6,195); every sphere and ellipsoid run reaches
10⁻¹⁰, Rosenbrock from 17 of 20 seeds, the other 3 in its local minimum near `(−1, 1, …, 1)`
(pycma 19 of 20; Kern, Hansen and Koumoutsakos, 2006, report 17 to 19 of 20 at 4 to 16 variables).
The test requires at least 17, which the measurement meets exactly; once, over seeds 1 to 300,
290 reached the global minimum (97%), so 17 of 20 is the low end of chance, not a weak method. A
second test recomputes twelve generations by a separate dense implementation of Figure 6 and
Table 1 (`C^(−1/2)` by the Denman–Beavers iteration), steps 0.5, 2 and 0.01, a linear model that
takes `h_σ` through both values: mean, `σ` and `C` agree to 1e-12. One-off mutations: dropping the
rank-μ update takes the ellipsoid's median to 7,875 and fails the pycma comparison; `h_σ` held at
1, its exponent `2g` for `2(g + 1)`, and the mean's step unscaled each pass the pycma comparison
(found in review) and fail the dense recomputation. The guide's example finds the ballast and
body length of a J760 rocket for a 3,048 m apogee and a 2.20-calibre margin together (one goal
alone leaves a curve of answers, and the run stopped at whichever it met first): 0.3373 kg and
1.1084 m in 300 flights, apogee 3,047.9997 m, margin 2.199996; flown again from a fresh build it
gives the optimizer's value to the bit, and at tolerances 100 times tighter 3,048.0011 m. The
example prints only what doesn't depend on the platform's last bits: CI first failed on Linux
and Windows, where `ln` and `exp` round differently and Windows' sphere run took 1,580
evaluations to macOS's 1,660. So it prints whether each test function reached its minimum, and
the design to 0.01: the ballast 2.3 g and the body 3.4 mm from a rounding edge, against a
solution good to about 10⁻⁴. The papers are cited, not pinned in `refs.lock.toml`, as
no test reads them. Left out: active CMA, restarts
(IPOP/BIPOP), repair or penalty bound handling, and the command line and Python.
