# ADR-139: Optimization constraints by Deb's feasibility rules (2026-10-01)

- **Status:** accepted
- **Summary:** M6.2b split b1, b2; M6.2b1: constraints for CMA-ES by Deb's (2000) feasibility rules (`Evaluation`, `Run::tell_constrained`), no penalty weight; infeasible candidates ranked, not redrawn; a target and TolFun count only feasible points; held to the sphere with `x₀ ≥ 1`, the tangent problem and CEC 2006 g06 from 20 seeds

**Context.** M6.2b asks for discrete choices (a motor, a catalogue part) and constraints (a
stability margin, a rail-exit speed), held by "hit 3,048 m" with the motor free and each
constraint re-checked on the winner. That is more than one pull request, and the constraints are
needed by the rocket problem whichever way the discrete choices go.

**Decision.**

1. **Split in two.** M6.2b1 constraints, held to test problems with known constrained minima;
   M6.2b2 discrete choices and the rocket problem with M6.2b's done-when.
2. **Deb's feasibility rules** (K. Deb, *CMAME* 186 (2000) 311–338, §3): feasible beats
   infeasible; two feasible by value; two infeasible by total violation `Σ max(0, gⱼ)`, ties by
   value. CMA-ES uses only ranks, so the rules plug into its ranking with no penalty weight to
   tune, and values and violations are never compared. `Evaluation { value, violation }` carries
   both; `Run::tell_constrained` and `Cmaes::minimize_constrained` take them; `Run::tell` is the
   case of zero violations, bit for bit (a unit test compares whole runs).
3. **Infeasible candidates are ranked, not redrawn** (bounds still redraw), so the distribution can
   straddle a constraint's edge and close in on a minimum on it. The best point is the best by the
   same rules; a target counts only for a feasible best; TolFun needs the window's every best
   feasible, and looks at this generation's feasible values only.
4. **Held to three problems** whose constrained minima are known in closed form: the sphere with
   `x₀ ≥ 1` (1 at `(1, 0, …)`), the tangent problem `Σ xᵢ ≥ n` (n at `x = 1`, by Lagrange), both
   in 10 variables, and CEC 2006's g06 (J. J. Liang et al., 2006), whose minimum is the crossing
   of its two circles, `x₀ = 14.095` exactly; the test also checks the published −6961.81387558015.
   Every one of 20 seeds reaches the minimum to 10⁻¹⁰ relative and within 10⁻⁴ of the point,
   also from an infeasible start. A violation is folded from `+0` (an empty `f64` sum is `−0`,
   which `total_cmp` ranks first), and a failed candidate is `Evaluation::failed()`, both `+∞`.

**Consequences.** A user scales each constraint (Deb normalizes them) so they count alike; the
guide says so. Not done: equality constraints (write `|h| − ε ≤ 0`), the adaptive penalty or
augmented-Lagrangian methods of the CMA-ES literature, and any rocket constraint, which comes with
M6.2b2's problem.
