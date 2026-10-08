# ADR-065: Stored results are references only when current and structurally plausible (2026-09-24)

- **Status:** accepted
- **Summary:** Stored results are references only when current and structurally plausible

**Context.** Loft lesson [L87][l87] says stored results were counted as references merely because a
`.ork` carried them. That made an outdated or not-simulated run, a missing summary, or an impossible
flight able to affect a validation gate and its denominator. ADR-057 correctly requires preserving
the file's stored results, but it did not yet distinguish faithful read-back from reference
eligibility. A stored result has no geometry fingerprint, so numerical disagreement cannot prove that
the design changed; a later comparison layer must establish design identity independently.

**Decision.**

1. **Keep first, classify second.** `StoredSimulation::reference_exclusion` is a pure classifier.
   It never changes or hides `Design::simulations`. The survey continues to count every stored run,
   its status, conditions, summary, branches, rows, events and parser warnings. A stored reference
   must name both `RK4Simulator` and `BarrowmanCalculator`, the simulator and aerodynamic-calculator
   identifiers shown in the public file specification's Simulation Data example. Missing and
   unsupported identifiers have stable exclusion reasons. They are provenance markers, not a full
   identity fingerprint for an OpenRocket version, settings or design.
2. **Only current, plausible OpenRocket runs can be stored references.** An explicitly `uptodate`
   run is eligible only when it has finite, non-negative stored values, positive ascent altitude,
   speed and time-to-apogee, and a flight time no shorter than time to apogee when present. Missing
   results or any required summary value exclude it. `outdated`, `notsimulated`, `external`,
   `loaded`, `cantrun`, `aborted`, missing-status and unknown statuses each have a stable exclusion
   reason. An external result remains readable, but is not silently promoted to an OpenRocket
   reference.
3. **Reject contradictions that the stored data itself proves.** Time columns may not run
   backwards, time or altitude may not be negative, and a stored time-series altitude may not differ
   from the stored apogee by more than `max(0.1% of apogee, 1 mm)`. This is a data-integrity policy
   for stored-value rounding, not a claim about flight-model accuracy. When several branches record
   an apogee event, the first such branch in file order is treated as the primary flight branch;
   the reader does not choose the branch whose maximum happens to match best. A single
   altitude-bearing branch is checked even when its apogee event is absent; multiple branches
   without an apogee event are uninspectable because their summary branch cannot be identified.
   Stored time-series rows and events must be finite, non-negative, and ordered. When summaries are
   present, rows may not exceed the stored flight time, event times may not exceed it, and the first
   apogee event must agree with `timetoapogee` within 1 ms plus floating-point rounding. This
   allowance covers the time columns' three-decimal stored precision; it is a data-integrity rule,
   not a model-accuracy tolerance.
   Fatal events, including normalized forms such as OpenRocket 24.12's `SIM_ABORT`, are excluded.
   No arbitrary minimum apogee is invented.
4. **Keep hpr reproduction as a separate screen.** `Design::reproduction_exclusion` reports reduced
   designs, missing or unknown configurations, and configurations hpr cannot assemble. These are
   limitations on hpr's ability to reproduce a stored reference, not evidence that the stored
   OpenRocket result is internally invalid. A later flight-comparison gate must report both screens.
5. **Count the policies separately.** `cargo xtask ork` reports stored-reference eligibility and
   hpr-reproduction eligibility alongside the unconditional stored-data census. On 2026-09-25, the
   survey found 91 stored-reference-eligible and 83 excluded runs among 174 stored simulations:
   47 inconsistent, 17 external, 11 outdated, 7 not-simulated and 1 missing simulator. Of the 91,
   1 is reproducible by hpr and 90 are not: 79 configurations are unflyable and 11 designs are reduced. The
   per-file detail remains in the private survey report; public docs quote only these aggregate
   counts.

**Consequences.** Stale statuses, missing provenance and contradictions cannot enter stored-reference
screens, distributions or stored-result denominators, while every result remains readable and
round-trippable. Hpr's current motor catalog and reduced-design boundary no longer masquerade as
stored-data defects. A future geometry fingerprint or comparison proof can add a new exclusion
reason without changing read-back.

[l87]: https://nrdptel.github.io/hpr-sim/decisions-and-roadmap.html#l87
