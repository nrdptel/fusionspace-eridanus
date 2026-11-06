# ADR-324: M7.3d in three: the prediction, then the public flights, then the halves (2026-11-06)

- **Status:** accepted; narrows M7.3d into M7.3d1, M7.3d2 and M7.3d3 and plans every remaining increment of M7.3d, whose *done when* M7.3d3 keeps whole; [ADR-257][adr-257] §3 and [ADR-322][adr-322] §2 stand unchanged.
- **Summary:** M7.3d asks for a new prediction (apogee from a log cut at burnout), a committed report on the 7 public flights, and the private collection's numbers for both of M2.7's halves with the held-out fits sealed. Nothing of the prediction exists yet, and each part needs its own full gate and review, so the milestone is shipped in three increments: the prediction and its noise test first, then the public flights' report, then the collection by halves and the scoreboard row, which keeps the old *done when* whole.

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md

**Context.** [ADR-257][adr-257] §3 sets out the prediction from the boost: the log cut at
burnout, found from the samples up to it before any smoothing or fitting, so that replacing every
later sample with noise leaves the prediction bit-identical; a report on the 7 public logged
flights with an exact sign-flip interval; the collection's flights as aggregates with a paired
bootstrap 95% interval; and a scoreboard row, `accuracy.from-boost`. [ADR-322][adr-322] §2 adds
that the collection's numbers come for each of M2.7's halves, with the held-out half's fits sealed
until M7.5. On 2026-11-06 `hpr_forensics` has the least-squares and Bayesian fits of M7.3b and
M7.3c (`crates/hpr-forensics/src/fit.rs`) but no cut at burnout and no prediction from one. Each
of the three parts touches different code and data (the library, the public flights' report, the
private collection), and each needs a full gate, reviews and CI, so one increment can't hold them.

**Decision.**

1. **M7.3d1 The prediction from the boost.** `hpr_forensics` finds burnout from a log's samples up
   to it, cuts the log there, fits it, and flies the fit on to apogee. A test replaces every later
   sample with noise and the prediction stays bit-identical; a probe that lets the cut read one
   later sample fails it. The fit's page says what the prediction is and how far to trust it.
   The analyzer's burnout reading can't be reused as it is: its spike filter
   (`SPIKE_WINDOW_S`), its burnout mean (`BURNOUT_MEAN_S`) and its noise span (`NOISE_SPAN_S`)
   are centered windows, and `readings::read` takes the whole log at once. The cut is
   therefore set at the last sample any of the finder's windows read, not at burnout itself, and
   the pad and liftoff come from the samples before that cut.
2. **M7.3d2 The public flights.** A committed report gives the 7 public logged flights' fits, the
   prediction's mean absolute and signed apogee errors, and the forward simulation's on the same
   flights in the same launch-day air and wind, with the exact sign-flip interval on their
   difference.
3. **M7.3d3 The collection by halves.** M7.3d's *done when*, whole: the collection's aggregates for
   each of M2.7's halves under ADR-322's encoding, the held-out ids' hash committed first, the
   held-out fits sealed, and the scoreboard's `accuracy.from-boost` row.

**Consequences.** M7.3d is met when M7.3d3 is. The search index has no room (ADR-323), so the fit
page's paragraph in M7.3d1 needs [#414](https://github.com/nrdptel/fusionspace-eridanus/issues/414)
or a page left out of the index first; whichever comes first is recorded in its PR. The three
entries take the roadmap past its 52,500-byte budget, which rises to 53,500, as earlier planning
PRs raised it for the increments they added.
