# ADR-142: Few evaluations by EGO (2026-10-02)

- **Status:** accepted
- **Summary:** M6.2d: EGO (Jones, Schonlau and Welch 1998) with a kriging surrogate fitted by likelihood and the expected improvement searched by CMA-ES; M6.2d split d1 (Branin, Hartmann 3: within 1% of the minimum in 50 evaluations from 20 seeds, worst 0.12%) and d2 (Hartmann 6, which d1's version leaves at a local minimum in 7 runs of 10)

**Context.** M6.2d asks for Bayesian optimization (EGO): Branin and Hartmann reach their minima
in a stated budget. CMA-ES and NSGA-II spend hundreds of flights; a model that takes minutes
per evaluation needs tens.

**Decision.**

1. **EGO** (D. R. Jones, M. Schonlau, W. J. Welch, *J. Global Optim.* 13, 455–492, 1998): a
   kriging surrogate with a constant mean and the correlation `exp(−Σ θₖ |Δxₖ|²)` (eq. (1),
   `pₖ = 2` as the paper fixes it on its tests), `μ`, `σ²` and `θ` of most likelihood (eq. (4);
   `μ̂`, `σ̂²` by eqs. (5), (6)),
   predictions and errors by eqs. (7) and (9), and the next point where the expected
   improvement, eq. (15), is largest. Variables are scaled to `[0, 1]`; values are standardized
   before each fit. `log₁₀ θₖ` is fitted in `[−3, 3]` by CMA-ES (100 evaluations per variable,
   started from the last fit); the expected improvement is maximized by CMA-ES from the best of
   200 random points per variable and from the best point so far. A nugget of 10⁻⁸ on the
   correlation's diagonal keeps its Cholesky factor defined as points crowd near a minimum.
   Departures (review): the paper maximizes the expected improvement exactly by branch and
   bound and warns multistart is unreliable; it runs diagnostic tests and transforms values
   where they fail (`−ln(−y)` on Hartmann 6), which hpr does not. A `+∞` value is a failed
   evaluation, fitted as the largest finite value; where no fit exists (all values equal) the
   next point is uniform in the box. The nugget leaves a small expected improvement at evaluated
   points (the paper's is zero), and about half of Branin's later points land within 10⁻³ of an
   earlier one: a minimum-distance filter is left for d2. At most 1,000 points and 50 variables
   (a fit is `O(m³)`). `egobox` (ARCHITECTURE's first pick) is not used: it brings a large
   linear-algebra tree (`linfa`, `ndarray`) for a method of a few hundred lines.
2. **Initial design**: a Latin hypercube of 10 points per variable (Loeppky, Sacks and Welch
   2009), the most spread out (maximin) of 100 drawn. Variables need two finite bounds;
   integer variables are refused for now.
3. **Stated budget and rule**: the best value within 1% of the minimum's size, Jones et al.'s
   measure of success, in 50 evaluations (the design's 20 or 30 included), from seeds 1 to 20.
   Set from a probe on seeds 1 to 10 at 40 evaluations (Branin's worst 1.03%, so 50), which
   overlap the test's seeds; measured on 20 seeds: worst 1.18e-3 (Branin) and 1.24e-3
   (Hartmann 3) of the minimum (macOS debug). With Jones's stop at 1% of the best value, a run
   may end well short of 1%: on Hartmann 3 a reviewer measured 8 of 20 runs (seeds 1 to 20)
   stopping more than 1% away, the worst 4.7% (Jones et al.'s Table 1: 1.7%).
4. **Split d1, d2.** On Hartmann 6, 7 of 10 runs (seeds 1 to 10, a probe not committed) end at
   its local minimum −3.20 after 100 evaluations, and a run takes 37 s in a debug build. Jones
   et al. needed 121 evaluations there, with the `−ln(−y)` transform. M6.2d2 holds Hartmann 6
   to the same rule; the levers are that transform, a cap on `θ`, and a faster likelihood fit.
   The gap is visible in the guide and the roadmap.
5. **The minima.** Branin's is `5/(4π)` in closed form at three points, checked to 10⁻¹⁴.
   Hartmann's are the printed −3.86278 and −3.32237, each checked by polishing with CMA-ES from
   the printed point to within 5×10⁻⁶. The constants are Dixon and Szegö's (1978) as Surjanovic
   and Bingham's library lists them; Hartmann 3's last centre has first coordinate 0.0381 there
   (some listings print 0.03815, to which the quoted minimizer belongs; the least values differ
   by about 2×10⁻⁶ as its weight, 0.1, is small).

**Consequences.** `hpr_analysis::optimize::ego` (`Ego`, `Optimum`, `Stop`) and
`benchmark::global` (`branin`, `hartmann3`, `hartmann6`); `tests/ego.rs`. No outside
implementation is run as an oracle: the minima are known, which is what the milestone states.
Left out: Hartmann 6 (M6.2d2), noisy outputs (a nugget fitted to the noise), batches of points
per step, constraints and integer variables.
