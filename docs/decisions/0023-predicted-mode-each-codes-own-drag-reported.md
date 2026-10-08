# ADR-023: Predicted mode: each code's own drag, reported against a target (2026-09-18)

- **Status:** accepted
- **Summary:** Predicted mode: each code's own drag, reported against a target

**Context.** M2.1 asks for every whole-flight case to run in two modes: same-drag, which M2.1b2
scores, and predicted, in which hpr flies its own aerodynamics. Its done-when asks for
predicted-mode results in the report, "with explained gaps", and for `M ≥ 1` cases to be reported
as gaps until M1.8. The committed whole-flight reference flies a declared constant `C_D0` of 0.5,
so hpr's own drag scored against it would measure hpr's drag against an arbitrary number. The
like-for-like reference is RocketPy flying each example's own drag, whose curves carry their own
terms and stay in the gitignored `refs/` (ADR-009). And neither code's drag is the truth: each
example's came from RASAero, OpenRocket or its team, and M1.5b already measured hpr's drag 47%
below Valetudo's table and 6.0% below Juno III's at Mach 0.3.

**Decision.**

- **A second reference,** `validation/fixtures/flight/rocketpy-whole-flight-own-drag.json`, from
  `flight.py --own-drag`: the same six cases, each with the drag its example flies in RocketPy
  1.13.0. That is the Calisto, Valetudo and Juno III curves read from `refs/rocketpy`, NDRT 2020's
  constant 0.44, Bella Lui's 0.43, and Prometheus 2022's `prometheus_cd_at_ma` (ported from
  RocketPy's MIT test fixtures, with 1.02 times it power-on). "As RocketPy flies it" was checked in
  RocketPy's source: `Rocket.__init__` fixes the drag the flight reads (`power_off_drag_7d`), so
  the Juno III notebook's rescaling and Bella Lui's replacement curve, both applied to the rocket
  afterwards, never reach the flight, and the reference does not apply them either. The fixture
  records each curve's path and SHA-256 and each constant, never a curve's values. It reproduces
  byte for byte, and the same-drag fixture is unchanged by the new flag.
- **A case says its mode:** `mode = "predicted"` under `[flight.whole_flight]` (`DragMode`;
  same-drag is the default). A predicted case flies the design with no drag table. Each mode
  refuses the other's reference: a predicted case against a declared table would score hpr's drag
  against a constant, and a same-drag case against the own-drag reference would fly a table the
  reference never flew. The L75 checks (design, dry mass, reference area, motor) apply to both.
- **Targets, not gates.** Each predicted metric keeps M2.1's 3% as a *target* and gets a verdict of
  `within target` or `outside target` (`Verdict::WithinTarget`, `Verdict::OutsideTarget`, from
  `Comparison::targeted`). Neither counts as scored, neither fails the run, and the rows sit in the
  report's own *Predicted mode* section, apart from the gated table and from *not scored*, the
  harness's escape hatch. Every miss is explained in its case file with its measurement. The
  report is pinned like the rest (ADR-022), so a predicted number that moves still has to be
  committed. A test pins that every predicted row, and no other, is a target row.
- **Prometheus 2022 is a known gap in predicted mode too:** RocketPy on its own drag peaks at
  Mach 1.049, and the harness checks the gap as it does the same-drag one (L85).
- **Predicted mode flies at rtol = atol = 1e-11,** same-drag mode at the default 1e-8. Its
  drag calls `ln` and `powf`, whose last bits differ between platforms' maths libraries, which
  moves the adaptive step sequence, so the answer differs by the solver's global error: at 1e-8,
  CI measured NDRT 2020's predicted apogee 1.7e-7 apart on macOS and Linux (1404.058522 against
  1404.058761 m), past ADR-022's 1e-7 reproduction bound. On macOS that apogee is 1404.058522,
  .057883, .058122 and .058145 m at 1e-8 to 1e-11, so 1e-11 converges it to about 1e-5 m, for
  0.5 s more over the suite. The bound is not loosened.
- **Peaks are found between the solver's steps, not only at them.** At 1e-11, CI still found
  NDRT 2020's predicted max Mach 6.6e-6 apart on macOS and Linux, and its max speed 1.2e-7 apart
  on macOS and Windows. The harness read each peak at the steps' ends, so a peak was off by
  wherever the step control put them, and that differs between platforms. Moving predicted mode's
  tolerance by 1e-7 of itself stands in for that: it moved NDRT's max speed by 2.4e-6 and Bella
  Lui's by 2.7e-6, but the event-located apogee by only 1.3e-9. (Changing the tolerance by one
  part in 10⁷ imitates another platform's step sequence on one machine.) Now, wherever speed, Mach or
  acceleration rises out of a step's start and falls into its end, a golden-section search on the
  step's dense output finds the peak between them. The same perturbation then moves no metric by
  more than 7.9e-9. The step-end reading had been low by up to 6.3e-5 (same-drag NDRT's max
  speed). Every verdict stands, and five of *Accuracy*'s cells moved in the third decimal. The
  bound is not loosened. This departs on purpose from measuring as RocketPy does (Loft lesson L80,
  that a metric must mean what the reference means): RocketPy reads its maxima at its solution's
  points only. hpr's peak now reads at or above its own step-end reading, by at most the 6.3e-5
  above. How much RocketPy's sampling misses depends on its steps and was not measured; its
  maxima move by at most 7.8e-6 between its tight and loose runs. Either way it is far inside 3%.
  The alternative, a platform-dependent number in a report that must reproduce on three
  platforms, is worse. The search applies in both modes, and to the power-on maximum.
  Its limits, from sampling every step at 400 points: a step whose quantity turns more than once,
  or jumps (the skin friction at the critical Reynolds number), is not searched, and none of those
  is a flight's maximum today. The computed acceleration carries about 1e-7 m/s² of numerical
  noise, so a smooth acceleration peak is found to about 1e-8 of itself and its time to about
  1e-4 s. Both are [issue #53](https://github.com/nrdptel/hpr-sim/issues/53).
- The set of predicted rows outside their target is pinned by a test, as the not-scored set is,
  so a case file's "nothing else misses" cannot go stale unnoticed.
- `scripts/regenerate-references.sh` regenerates the new reference with the others, and now
  prints how far each fixture moved, number by number. ADR-022's first dispatched run, on GitHub's
  macOS runner, showed why: RocketPy's fixtures moved in their last digits (the descents by at most
  3.6e-11 relative; in the whole flights, a landing height of 2e-8 m by 3e-9 m), which changed
  their hashes and so the report, with no printed metric moving.

**Alternatives.**

- Scoring predicted mode against the same-drag reference: it measures hpr's drag against 0.5.
- Gating it at 3%: two of five apogees miss by 10%, from drag tables that are not the truth
  either, and hpr's drag runs on placeholder fin edges and finishes where the examples record
  none. A gate would fail the suite on a disagreement nobody can yet settle, or be loosened to
  pass, which rule 2 forbids. M2.1's own done-when asks for predicted-mode results "reported, with
  explained gaps", not passed. `VALIDATION.md`'s initial targets called the 3% "targets, not
  gates, until the first report exists"; since then it is a gate in same-drag mode and, by this
  decision, a target in predicted mode.
- Declaring every predicted metric *not scored*: it would bury the same-drag suite's eleven open
  misses among 75 routine notes, and a target that is met would read like one that is not.
- Committing the examples' curves, or reading them in CI: their terms forbid the first (ADR-009),
  and CI has no RocketPy checkout.

**Consequences.**

- The heights: Calisto −0.527%, Bella Lui +1.118%, Juno III +3.181%, Valetudo +10.007% and NDRT
  2020 +10.232%, each where hpr's drag sits against the example's. Flown on the same drag, all five
  agree within 1.710%.
- A reader can see how hpr's own aerodynamics compare with the drag RocketPy's examples ship, and
  why; nothing says which drag is right until real flights (M2.3).
- When M1.8 lifts the Mach limit, both Prometheus cases fail until their gaps are removed.
- A predicted case's known gap is justified by the reference reaching Mach 1, as a same-drag
  one's is. hpr's own drag could take a rocket past Mach 1 where RocketPy's example stays below
  it; no case does that today (both Prometheus references exceed Mach 1), and such a case would
  have to leave the lock or wait for M1.8.
- A predicted flight past Mach 0.8 would fly hpr's drag beyond the range its build-up is
  documented for, and the harness does not flag it; none does today (Calisto peaks at Mach 0.746).
