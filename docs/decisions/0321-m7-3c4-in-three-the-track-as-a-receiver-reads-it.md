# ADR-321: M7.3c4 in three: the track as a receiver reports it, then the drag on heights alone, then every count (2026-11-05)

- **Status:** accepted; narrows M7.3c4 into M7.3c5, M7.3c6 and M7.3c7 and plans every remaining increment of M7.3c4, whose *done when* M7.3c7 keeps whole; amends [ADR-320][adr-320] §8 (M7.3c4 as its only remaining increment) and [ADR-317][adr-317] §2 (the likelihood: on a track, the posterior no longer takes the noise [ADR-316][adr-316] §3 and §11 state); amended 2026-11-06 to split M7.3d into M7.3d1, M7.3d2 and M7.3d3, M7.3d3 keeping its *done when* whole (*M7.3d in three*, below), its cut read as the sample where burnout is found
- **Summary:** ADR-320's posterior took the track's noise as the least-squares fit states it, at least 10 m (33 ft), and its wind intervals came out too wide. Its own noise made them too narrow; reading the receiver's filter, centering the draws and integrating out the pad's position brought them in. So on a log with a track the posterior now reads the drift through a second-order filter of stated range and integrates out the track's noise, its run from bin to bin and the pad's position. The wind's four counts now pass, and no count that passed before fails. M7.3c4 is split in three: M7.3c5 the wind, M7.3c6 set A's drag, M7.3c7 every count.

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-317]: 0317-m7-3c-in-two-the-sampler-then-the-test.md
[adr-320]: 0320-m7-3c2-the-bayesian-option-in-two-blocks.md
[report]: ../../validation/reports/fit-intervals.md
[page]: ../physics/flight-fit.md
[note]: ../research/gnss-in-boost.md

## Context

[ADR-320][adr-320]'s report left six of the ten Bayesian counts failing: the wind east and north on
both sets, too wide, set A's drag (27 below and 78 above of 1,000) and set B's impulse (83 below and
29 above). It named the first try for the wind, a noise model the track's residuals settle, and asked
that each count be measured by track and by design first. By design no one rocket carries the wind's
misses: they spread over all ten. By track, 70 of set A's 78 drag truths above fall on logs of height
alone, where the wind's shear is drawn, and set B's impulse misses gather on two designs (20 of 96
on one, 17 of 95 on another). The wind's misses are all on tracked logs, as the wind is counted only
there.

Every count below before the report's is from `cargo xtask fit --try <set> 300 --track`, a set's
first 300 flights with a track logger, which this increment adds for the purpose. Those are among
the 1,497 the report counts the wind on, so these flights were seen while the model was chosen,
as ADR-320's were.

## Decisions

1. **The track's noise, its run and the pad are integrated out.** Each axis's track residuals, in
   the order of their bins, are taken as a first-order autoregression of one standard deviation `σ`
   and one lag-one correlation `φ` for both axes, plus a constant per axis, the pad's position,
   which the reader takes from the fixes before liftoff. With the constants flat, `σ` by Jeffreys'
   prior `1/σ` and `φ` uniform on 0 to 0.99, the integral over `σ` and the constants has a closed
   form (exact Gaussian AR(1) likelihood: J. D. Hamilton, *Time Series Analysis*, 1994, §5.2; the
   scale: G. E. P. Box and G. C. Tiao, *Bayesian Inference in Statistical Analysis*, 1973, §2.4),
   and `φ` is integrated by the midpoint rule on 198 nodes. It costs no flight. A test holds it to
   a dense matrix computation of the same integral.
2. **The track is read through a receiver's navigation filter.** A constant-velocity filter on
   position fixes is a type-2 loop that lags a boost by `a/ωn²` and sheds the error after burnout
   with a time constant about `1/(ζωn)` ([the note][note] §4). The posterior reads the flight's
   drift through such a loop, `ζ = 1/√2`, its natural frequency log-uniform from 0.2 to 5 rad/s,
   integrated by the midpoint rule on 96 nodes. That range reaches past the note's 0.47 to 0.69 rad/s
   both ways; at its top a 10 g boost lags 3.9 m (13 ft). A receiver that follows its own Doppler lags under
   1 m (the note, §4), which would take about 10 rad/s, so it lies outside the range.
   Integrating over the frequency flies nothing either. A test holds the loop to `a/ωn²` within 5 cm, and
   to the synthetic receiver's own loop, written apart, within a millimeter.
3. **The proposal is centered where that posterior peaks.** The least-squares answer weighs the
   track at 10 m or more, so the posterior's peak lies away from it. Three Gauss-Newton steps from
   the answer, each reading the track through the filter the track's likelihood favors there, its
   mean taken off each axis and its noise the misfit's RMS widened for its run, find the peak; the
   proposal's covariance and the inner block's step are those of the last. That's 21 flights a fit
   more, 87.4 a fit in all against 66.4, on the 300 flights of set A. The weights are the posterior's
   own, so where the proposal sits changes only how many draws count.
4. **Only on a log's own noise.** The track is taken this way only where the fit takes its noise
   from the log's residuals. A noise given or known is used as given, so the sampler's closed-form
   tests on a line stand.
5. **M7.3c4 in three.** The wind is the first that moved. Set A's drag on heights alone has a place of
   its own, and set B's impulse a third, not yet measured. So M7.3c4 is split, each with its own
   *done when*, and met when all three are. This plans every remaining increment of M7.3c4:
   - **M7.3c5 The wind on a track.** *Done when:* on a log with a track, the Bayesian option reads
     the track through a receiver's navigation filter, its natural frequency integrated over a
     stated range, and integrates out the track's noise, its run from bin to bin and the pad's
     position; its raw 90% credible intervals on the wind, east and north, pass ADR-257 §1's three
     counts on sets A and B, on the same 1,497 tracked flights, their probes failing; no Bayesian
     count that passes in ADR-320's report fails; the drag and impulse scales stay identified on set A
     and nothing on pure noise; the fit's page says how the track is read and how far to trust it.
   - **M7.3c6 The drag on heights alone.** *Done when:* set A's drag scale's raw 90% credible
     intervals pass ADR-257 §1's three counts on the same 1,000 flights, counted per design, their
     probes failing, and no Bayesian count that passed before it fails; whatever the posterior
     states of the wind's shear on heights alone is stated with its source, not fitted.
   - **M7.3c7 Every count.** *Done when:* M7.3c4's *done when*, whole.

## Evidence

On set A's first 300 flights with a track (299 counted for the wind), below, inside and above:

| The track, as the posterior reads it | wind east | wind north | median effective draws |
|---|---:|---:|---:|
| ADR-320: the least-squares fit's noise | 3, 294, 2 | 5, 287, 7 | 255 |
| its own noise and run (`φ` to 0.9) | 29, 243, 27 | 33, 217, 49 | 111 |
| the same, `φ` to 0.99, the proposal centered | 33, 241, 25 | 34, 217, 48 | 179 |
| and the receiver's filter | 24, 248, 27 | 28, 239, 32 | 42 |
| and the proposal at the filter's peak | 20, 257, 22 | 23, 248, 28 | 154 |
| the same, 1,600 draws read | 19, 262, 18 | 21, 256, 22 | 535 |
| and the pad integrated out (this ADR) | 9, 279, 11 | 8, 278, 13 | 195 |

A tail of 299 holds about 15 truths at 5%. On set B's first 300 with a track, by this ADR's model:
17, 271, 12 east and 16, 267, 17 north, at a median of 185 effective draws; on those flights the
drag came to 15, 280, 5, the impulse 15, 274, 11 and the mass 18, 262, 20. With four times the draws
the counts barely moved, so the narrowness before the pad was the model's, not the sampler's.

The [report][report], on every flight, below, inside and above, with ADR-320's before each:

| Parameter | Set A, ADR-320 | Set A, this ADR | Set B, ADR-320 | Set B, this ADR |
|---|---:|---:|---:|---:|
| drag scale | 27, 895, 78: fails | 30, 887, 83: fails | 66, 884, 50 | 69, 877, 54 |
| impulse scale | 70, 881, 49 | 72, 878, 50 | 83, 888, 29: fails | 85, 881, 34: fails |
| wind toward the east | 29, 1,452, 16: fails | 59, 1,380, 58 | 26, 1,439, 32: fails | 58, 1,353, 86 |
| wind toward the north | 34, 1,429, 34: fails | 55, 1,384, 58 | 31, 1,430, 36: fails | 71, 1,345, 81 |
| mass on the weighed mass | 52, 887, 61 | 56, 882, 62 | 63, 870, 67 | 61, 870, 69 |

On the wind's 1,497 flights a tail passes with 49 to 104 truths and the inside with 1,308 to 1,384;
on the 1,000, a tail with 29 to 74. The four wind counts pass, each probe failing, so M7.3c5's
counts are met; the four counts that passed before still pass. Drag and impulse stay identified on
set A, at median widths of 0.0919 and 0.0391, and nothing is identified on pure noise. The wind's
median width on set A came to 0.364 m/s east and 0.36 north, from 0.431 and 0.439; least squares'
are 0.345 and 0.374. The report took 6,954 s on a 10-core Mac.

Two things moved the wrong way:

- **Set B's drag passes, but so does its widened probe now** (fails, passes, fails; under ADR-320
  all three failed). Its tails moved from 66 and 50 to 69 and 54, so the test can't tell its
  intervals from ones widened to 1.25 times. The count passes; M7.3c7 needs the probe to fail too.
- **Fewer effective draws.** Posteriors short of 100 effective draws rose from 319 to 563 of 2,276
  on set A, and from 392 to 711 of 2,296 on set B; the medians fell from 232 to 153 and from 223 to
  138. On set A's 300 tracked flights in the try runs the median fell from 255 to 195, so the loss
  is on the tracked logs. Every count includes those posteriors, as before.

## What this leaves

- **The filter is the synthetic receiver's shape.** The synthetic flights' receivers are type-2 loops
  of `ζ = 1/√2`, `ωn` from 0.25 to 1 rad/s, and the posterior reads the track through the same
  shape, so the wind's pass doesn't test the shape. A real receiver's loop bandwidth isn't published
  ([the note][note], *Open questions*); the note's measured lags are in altitude, and the lag model
  is its inference. A receiver that isn't a type-2 loop, one that follows its own Doppler (under 1 m
  of lag), or one that drops fixes in the boost differently is outside this model, and no real
  flight has checked it.
- **Set B's wind shears; the posterior's doesn't, on a track.** The drift carries the shear into
  the wind's posterior unmodeled there, as in ADR-320.
- **The wind's pass leans on thin posteriors.** Tallied from the committed slice's lines (every
  21st fit, 294 wind intervals counted), the 218 from posteriors of 100 or more effective draws miss
  12 truths, 5.5%, where 10% belong, and the 76 under 100 miss 11, 14.5%. So the well-drawn
  intervals may still be too wide, and the thin ones, too narrow, may bring the tails up to 5%. The
  four-times-the-draws run above was on the model before the pad, so it doesn't answer this for
  the model shipped. The report should split its counts by effective draws; M7.3c7 needs that
  answered.
- **Set A's wind passes at the wide edge.** 1,380 and 1,384 of 1,497 fall inside, against 1,384 at
  most.
- **Set B's impulse and set A's drag** still fail, by about as much as before: M7.3c6 and M7.3c7.
- **The sets were seen.** The noise model, the filter, the pad and the proposal were chosen after
  reading counts on the first 300 tracked flights of each set.

**Amended 2026-11-06: M7.3d in three.** The same rule, one plan for M7.3, splits M7.3d.

*Context.* [ADR-257][adr-257] §3 sets out the prediction from the boost: the log cut at
burnout, found from the samples up to it before any smoothing or fitting, so that replacing every
later sample with noise leaves the prediction bit-identical; a report on the 7 public logged
flights with an exact sign-flip interval; the collection's flights as aggregates with a paired
bootstrap 95% interval; and a scoreboard row, `accuracy.from-boost`. [ADR-322][adr-322] §2 adds
that the collection's numbers come for each of M2.7's halves, with the held-out half's fits sealed
until M7.5. On 2026-11-06 `hpr_forensics` has the least-squares and Bayesian fits of M7.3b and
M7.3c (`fit.rs` and `posterior.rs` in `crates/hpr-forensics/src/`) but no cut at burnout and no prediction from one. Each
of the three parts touches different code and data (the library, the public flights' report, the
private collection), and each needs a full gate, reviews and CI, so one increment can't hold them.

*Decision.*

1. **M7.3d1 The prediction from the boost.** `hpr_forensics` finds burnout from a log's samples up
   to it, cuts the log there, fits it, and flies the fit on to apogee. A test replaces every later
   sample with noise and the prediction stays bit-identical; a probe that lets the cut read one
   later sample fails it. The fit's page says what the prediction is and how far to trust it.
   The analyzer's burnout reading can't be reused as it is: its spike filter
   (`SPIKE_WINDOW_S`), its burnout mean (`BURNOUT_MEAN_S`) and its noise span (`NOISE_SPAN_S`)
   are centered windows, and `readings::read` takes the whole log at once. The cut is
   therefore set at the last sample any of the finder's windows read, not at burnout itself, and
   the pad and liftoff come from the samples before that cut. The plan:
   - The finder walks the log's height track forward and decides at each sample from it
     and those before it: liftoff 10 m (32.8 ft) above the track's first height, then a speed
     fitted over the last 0.5 s, and the cut where it has stayed 10% below its top, and at least
     10 m/s (32.8 ft/s), for 1 s. A barometer's jump near Mach 1 cut 17 of set A's first 60
     flights before burnout at the first fall, and none with the 1 s hold. The cut is the sample it
     stops on: ADR-257 §3's "cut at burnout" is read as the sample where burnout is found from
     those up to it, so the log keeps the coast's start. On set A's 200 synthetic flights (a
     test) every barometer's log is cut after the true peak of the vertical speed and at most
     2.75 s after burnout, 2 of them up to 0.11 s before it. M7.3d2 gives the error; whether an
     accelerometer's cut, sooner, would do better isn't measured.
   - A GNSS receiver's track isn't cut: in the boost its fix can drop out or lag the climb by
     up to 440 m in one measured flight ([GNSS in boost](../research/gnss-in-boost.md)), and on set A's first 60
     flights such tracks were cut up to 11 s after burnout or not at all.
   - Every reading after that runs on the log truncated at the cut, so later samples can't reach
     it by construction. The test then compares the prediction on the whole log with the one on
     the log whose later samples are noise, bit for bit.
   - The fit needs observations that end at the cut rather than at apogee: `observations` runs
     from a second before liftoff to the apogee reading, which on a log cut on the climb is no apogee at all
     (withheld, or a floor at the cut; `Readings::ends_on_the_climb`).
2. **M7.3d2 The public flights.** A committed report gives the 7 public logged flights' fits, the
   prediction's mean absolute and signed apogee errors, and the forward simulation's on the same
   flights in the same launch-day air and wind, with the exact sign-flip interval on their
   difference.
3. **M7.3d3 The collection by halves.** M7.3d's *done when*, whole: the collection's aggregates for
   each of M2.7's halves under ADR-322's encoding, the held-out ids' hash committed first, the
   held-out fits sealed, and the scoreboard's `accuracy.from-boost` row.

*Consequences.* M7.3d is met when M7.3d3 is. The search index has no room (ADR-323), so the fit
page's paragraph in M7.3d1 needs [#414](https://github.com/nrdptel/fusionspace-eridanus/issues/414)
or a page left out of the index first; whichever comes first is recorded in its PR. The split's
entries take the roadmap past its 52,500-byte budget, which rises to 53,500, as earlier planning
PRs raised it for the increments they added.

[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
