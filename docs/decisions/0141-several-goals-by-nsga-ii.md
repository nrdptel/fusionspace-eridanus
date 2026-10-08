# ADR-141: Several goals by NSGA-II (2026-10-02)

- **Status:** accepted
- **Summary:** M6.2c: several goals by NSGA-II (Deb et al. 2002) with bounded SBX and polynomial mutation, constrained domination; held to ZDT1 to ZDT3's exact fronts and to pymoo 0.6.2 (every run's GD and IGD within twice pymoo's worst, medians within a factor of 1.25 either way), tournaments paired as pymoo pairs them; a rocket's apogee against static margin, its front flown again and matched by CMA-ES at 2.5 calibres

**Context.** M6.2c asks for NSGA-II: ZDT1 to ZDT3 fronts within a stated generational distance,
and a two-goal rocket problem that gives a front. CMA-ES (ADR-138 to 140) finds one design; a
trade-off needs the set nothing dominates.

**Decision.**

1. **NSGA-II as published** (K. Deb, A. Pratap, S. Agarwal, T. Meyarivan, IEEE TEC 6(2),
   182–197, 2002): fast non-dominated sorting (p. 184), crowding distance with infinite ends and
   each goal's range taken over the front (p. 185; the paper doesn't say front or population;
   pymoo takes the front), the crowded comparison in binary tournaments, and the `2N` survival
   with the split front cut by crowding, largest first (p. 186). A goal of zero or infinite range
   in a front adds nothing between its ends. Constrained domination (§VI, p. 192) handles limits
   and failures: a failed design, or one with a goal of `+∞`, is violation `+∞` with its goals set
   to `+∞` (a first version let a `+∞` goal sit on the front, its other goal unbeaten; review).
   Ties keep the parents-then-children order (stable sorts), and a tournament tie is a coin toss.
   The paper gives no pairing; the first version drew each tournament's two designs uniformly,
   and its fronts' median distances were 5% to 15% above pymoo's. Review traced that to pairing:
   pymoo, like Deb's code, lays two shuffles of the population end to end, so every design plays
   exactly two tournaments. That is adopted.
2. **Operators**: SBX (Deb and Agrawal 1995) with each variable crossed with probability ½ (Deb
   and Beyer 1999, p. 8) and the children swapped with probability ½; polynomial mutation (Deb
   and Goyal 1996). SBX is cut at the bounds by renormalising the density inside them, Deb and
   Agrawal's rule (p. 143); mutation cuts each side at its own bound, keeping half the
   probability on each side, as Deb's code does. These equations appear in no paper we could
   reach; pymoo 0.6.2's `sbx.py` and `pm.py` (Apache-2.0) print them, were read, and hpr's
   comments derive them again (SBX: the unbounded inverse at `u′ = uα/2`; mutation: each half
   truncated). Unit tests hold SBX's spread to its density far from the bounds and cut at
   each (both children, with different cuts), the children's order to ½, and mutation's step at an asymmetric point to each side's cut
   distribution, within five standard errors over 100,000 draws. Three wrong versions the
   review found passing the first tests (`u ≤ ½` for `u ≤ 1/α`, no swap, `δ₂` for `δ₁`) now
   fail them, as do two wrong upper-child cuts found later. A test checks that every design
   plays exactly two tournaments, never against itself. Deb and Deb (2014) give a different bounded mutation, not used. The defaults are
   the paper's (p. 187): 100 designs, 250 generations, `p_c = 0.9`, `η_c = η_m = 20`,
   `p_m = 1/n`. A generation draws from one stream, `for_stream(seed, [generation])`, before its
   children are flown. Every variable needs finite bounds whose difference is finite (the first
   population is uniform between them); integer variables are refused for now. The population is
   at most 2¹²: the sort keeps each design's dominated list, `O(N²)` memory, about 270 MB of
   indices there on a 64-bit machine (2¹⁶, the first cap, would need tens of GB; review). Bounds
   are held within ±`f64::MAX/4` so crossover's sums can't overflow.
3. **The stated distance.** GD is Deb's Υ (§IV-B, p. 188: mean distance from each front design
   to the true front) taken to the curve itself, not to `H = 500` points: a window of `f₁ ± d₀`
   searched on a 400-step grid and refined by golden section (`Zdt::distance_to_front`, checked
   against a 10⁶-point scan). IGD is the mean distance from points of the true front at 1,000
   evenly spaced `f₁` (265 on ZDT3's pieces) to the nearest design. pymoo 0.6.2, set to the
   paper's settings (rank-and-crowding tournaments, which aren't its default, no duplicate
   removal), ran seeds 1 to 20. The bounds were fixed from its runs before hpr's were measured:
   each hpr run within twice pymoo's worst, a guard against gross failure; and hpr's median at
   most 25% above pymoo's, the margin ADR-138 allowed CMA-ES against pycma. Review made the
   median rule two-sided, within a factor of 1.25 either way (at most 20% below): on ZDT every
   variable but the first is best at its low bound, and two of the three wrong operators above
   (no swap, `δ₂` for `δ₁`) put hpr's median GD more than half below pymoo's; the third (`u ≤ ½`)
   stays within 1% to 9% and only its unit test catches it (the review's measurements, not
   committed). The bound on every run is
   GD 2.92e-3 (ZDT1), 2.82e-3 (ZDT2), 1.31e-3 (ZDT3). Measured, with the pairing of item 1:
   hpr's median GD 1% to 8% below pymoo's (ZDT1 1.07e-3 against 1.10e-3), IGD 1.3% to 2.3% below,
   each problem's worst run 1.9 to 2.1 times inside its bound. The per-run IGD bound on ZDT3, 6.78e-2, is set by
   pymoo's one run that missed part of the front and is 12 times the median; the median rule is
   what holds ZDT3's coverage. hpr's `generational_distance` gives pymoo's own GD
   (`pymoo.indicators.gd`, its 500 points) to 10⁻¹². Deb et al.'s Table II reports a mean Υ of
   0.033, 0.072 and 0.115 on these problems, 18 to 87 times pymoo's mean to the same 500 points
   (0.0013 to 0.0019); why was not investigated. pymoo's non-dominated sorting runs on moocore
   (LGPL-2.1-or-later), installed as its dependency in the oracles' environment only: run-only,
   like the OpenRocket jar, never linked or ported; its IGD is not used. Recorded in the notices.
4. **ZDT3's pieces** were solved to 40 digits (mpmath `findroot`): each ends where the curve's
   slope is zero, and the next starts where it falls below that end. Tests check both equations
   and a 200,001-point scan of which points are undominated. pymoo prints the second start as
   0.182228780, a digit short of 0.1822287280.
5. **The rocket problem** (`crates/hpr/examples/pareto_front.rs`): ADR-138's 66 mm J760 rocket,
   its body fixed at 1.0 m so every design shares one supersonic table, nose ballast 0 to 0.8 kg
   and fin span 4 to 10 cm free, goals the apogee and the static margin at launch mass at Mach
   0.3 (both maximized), at least 1.5 calibres. 20 designs for 25 generations, 500 flights, 54 s
   in debug. Its 20 front designs are flown again from fresh builds, each with its own table, to
   the bit. CMA-ES alone at 2.5 calibres (200 flights, a fixed count so the run doesn't depend on
   a tolerance's last bits) reaches 3,052.9 m; the front, interpolated between its designs
   either side, gives 3,051.3 m, 0.05% apart, the example's check being 0.5%. Over seeds 2020 to
   2045 (release build, development machine, not committed) all 26 pass, from −0.46% to +0.18%;
   CMA-ES's own 200-flight answer varies by 0.24% over them. A first try with 24 designs for 15
   generations over wider ranges spread its front from 1.5 to 5.3 calibres, and the ranges were
   narrowed; then 20 designs for 12 generations fell 0.66% short, and the generations were raised
   to 25, the 0.5% unchanged.
   The output was the same from a release build and with the flight's relative tolerance moved
   by four ulps (neither committed), its nearest printed value 0.69 m from a rounding edge, so it
   is expected to match on every platform.

**Consequences.** `hpr_analysis::optimize::nsga2` (`Nsga2`, `Run`, `Goals`, `Member`, `Front`,
`dominates`, `generational_distance`, `inverted_generational_distance`) and
`benchmark::zdt`; `tests/nsga2.rs` with `tests/fixtures/nsga2/pymoo.json`. Left out: integer
variables in NSGA-II, a stopping rule (it runs its generations), reference problems of three or
more goals (DTLZ), and a hypervolume measure. EGO (M6.2d) is next.
