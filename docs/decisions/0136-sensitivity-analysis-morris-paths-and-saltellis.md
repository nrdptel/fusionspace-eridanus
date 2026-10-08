# ADR-136: Sensitivity analysis: Morris paths and Saltelli's Sobol' estimates, with their standard errors (2026-10-01)

- **Status:** accepted
- **Summary:** M6.1c: sensitivity analysis in `hpr_analysis::sensitivity`; uniform independent factors; Morris's paths with `μ`, `μ*`, `σ` and `μ*`'s standard error, and the exact grid moments by `Morris::population`; Saltelli 2010's first-order and Jansen's total estimates on pseudo-random rows, with delta-method standard errors; held to Ishigami and Sobol's g in closed form and calibrated over seeds

**Context.** M6.1c asks for Morris screening and Sobol' indices that match the known indices of
test functions with closed forms (Ishigami, Sobol's g) within sampling error. Open: what a factor
is, how the two methods meet a flight, which estimators, what "within sampling error" is measured
against, and what Morris's "known indices" are, since Morris's measures have no published closed
forms for these functions.

**Decision.**

1. **A factor is a uniform range** (`Factor`: name, low, high), independent of the others, as in
   Morris (1991) and Saltelli and others (2010). Normal inputs are given as a range; other
   distributions and correlations are left out.
2. **Each method is a design, then an analysis** (`Morris::design` → `MorrisDesign::points` →
   `analyse(outputs)`; `Sobol::design` → `SobolDesign::points` → `analyse`), with a closure shortcut
   (`screen`, `indices`). The model is the caller's: a flight is flown by setting the factors into a
   `Draw` and `MonteCarlo::inputs`, as the guide's example does, and the runs can go to any threads
   or machines. No flight-specific factor type: which inputs to vary, and over what, is the caller's
   choice, and `Draw` already names every input `MonteCarlo` can vary. A model that can fail uses
   the two steps, as the closures return a bare `f64`. Factor names must differ (`DuplicateFactor`).
   A design is held to `MAX_DESIGN_POINTS` (2²⁰) points and `MAX_DESIGN_VALUES` (2²⁴) coordinates,
   and a grid to `MAX_LEVELS` (2¹⁶), so a design that passes `new` takes under about 200 MiB, on
   wasm32 too. An index or standard error that overflows from an extreme output is refused, not
   returned. Designs aren't serialized: a configuration and its seed rebuild one bit for bit. Result
   types are `#[non_exhaustive]`, so second-order indices can be added later.
3. **Morris: his paths** (Technometrics 33(2), 1991, eq. (1) p. 163; `B*` p. 164), `p` even,
   `Δ = p/(2(p − 1))`, start levels in the lower half, a sign and an order per path. Each path
   draws from `SeededRng::for_stream(seed, &[j])`. Reported: `μ`, `μ*` (Campolongo, Cariboni and
   Saltelli, 2007), `σ` with `r − 1`, and `μ*`'s standard error, `sd(|d|)/√r`. Effects are in the
   output's units per the factor's whole range. Campolongo's choice of spread-out paths is left
   out. Campolongo's finding that `μ*` ranks as `S_T` does is shown by experiment only, and
   Ishigami's function is a counter-example. A step of about half the range misses `sin² x₂`,
   whose period is half the range: at four levels `μ*` puts `x₂` first and `S_T` puts `x₁`
   first; at six or more `μ*` puts `x₂` last, though `S₂` is the largest first-order index. A
   test pins both, and the guide warns of it.
4. **Morris's known answers are the moments of `Fᵢ`**, the finite distribution each path's effect
   is drawn uniformly from (Morris, p. 164). `Morris::population` computes them exactly by running
   the model at all `p^k` grid points (at most 2²⁴). For Ishigami and the g function at `p = 4`
   they are also derived in closed form in the test file, and `population` matches both to 1e-12.
5. **Sobol': Saltelli and others' design and estimates** (CPC 181, 2010, Table 2, p. 262): rows of
   `A`, `B` and `A_B⁽ⁱ⁾`, `Vᵢ` by (b), which they recommend for its design's points (p. 263), and
   `V_Tᵢ` by Jansen's (f), "the best practice so far" (p. 262). `V` comes from `A` and `B` together
   (the Primer, p. 166). The outputs are shifted by their mean first. Sobol' (2001, p. 277, remark
   3) advises that against a loss of accuracy. Our own further reason: the first-order estimate's
   variance has a term in the output's mean squared, which the shift removes, as an apogee's mean of
   a kilometre would otherwise swamp it. Rows are pseudo-random from `for_stream(seed, &[j])`, not
   quasi-random: rows are independent, so the error is the central limit theorem's.
6. **Standard errors by the delta method**: each index is a smooth function of row means, so its
   error is `√(Σψⱼ²/(N(N − 1)))` with `ψⱼ` row `j`'s linearised influence, including the shift's.
   This is deterministic and needs no bootstrap stream. A unit test computes it a second way,
   the gradient over the raw row means times their sample covariance, to 1e-9. Dropping the
   shift's `D` term fails that test, though no calibration over seeds could see it (it is
   `O(1/√N)`).
7. **"Within sampling error" is two checks.** First, each estimate lies within four of its own
   standard errors of the closed form (Ishigami at `N = 2¹⁵`, g at `2¹⁴`; Morris at 1,000 paths).
   Second, over 1,000 seeds (500 for g's Sobol' indices), `(estimate − known)/error` has a mean
   within `4/√n` of 0 and a standard deviation within `4/√(2n)` of 1. The g function's
   calibrations use a four-factor g (`aᵢ` = 0, 1, 4.5, 9); Morris's `μ*` is calibrated the same
   way on it over 1,000 seeds. Each calibration also
   asserts its own power: the same `z`s divided by 0.85, as from standard errors 15% too small,
   must fail it. The g function's `aᵢ` = 0, 1, 4.5, 9, 99 × 4 span Marrel and others' (2008) four
   classes as the SFU library quotes them. Sobol' and Levitan's printed `b = 0.05` indices are
   held to their three or four printed digits.

**Consequences.** M6.1c is met. In the guide's example, Ishigami's and g's indices at `N` = 8,192
lie within 1.4 standard errors of the closed forms. The rocket's Morris screening, 70 flights,
ranks the impulse and the drag first for the apogee and the wind's speed for the landing. A Sobol'
analysis of a flight costs `N(k + 2)` flights, about 47 ms each in a debug build, so the example
leaves it out. M6.1d's speed work makes it practical. The papers were read from the authors' or
institutions' copies and are cited by DOI, not pinned in `refs.lock.toml`, because no test reads
them. Second-order indices, quasi-random rows, other distributions and a command-line front end are
left out.
