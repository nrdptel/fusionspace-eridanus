# ADR-384: M7.3g in three: the split by the residual, measured, fails set B (2026-11-27)

- **Status:** accepted; splits [M7.3g][m7-3g] into M7.3g1, M7.3g2 and M7.3g3, each with its own *done when*, M7.3g's own *done when* whole in M7.3g3; changes no share, band, count or flight; [ADR-368][adr-368]'s gap stands
- **Summary:** The first rule M7.3g tried, as issue #1108 stated it before any count, fits the least-squares fit's interval shares apart for fits whose heights' residual is within 1 m (3.3 ft) and for the rest. Measured on sets A and B, it passes every row and class on set A but fails three rows and classes on set B, among them the wind toward the north, which passes today. So it isn't put in force. M7.3g1 ships the tools that measured it and this record. M7.3g2 chooses a second rule on the calibration set alone, its counts predicted first. M7.3g3 puts a rule that passes in force.

[m7-3g]: ../ROADMAP.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
[adr-377]: 0377-no-release-waits-on-users-and-release-0-2-comes-next.md
[i1108]: https://github.com/nrdptel/fusionspace-eridanus/issues/1108
[measured]: https://github.com/nrdptel/fusionspace-eridanus/tree/m7.3g-split-measured
[measured-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/5a4df911c0209a2d3ae64d1e7044bf2c558560c5/validation/reports/fit-intervals.md

**Context.** The least-squares fit states each parameter's 90% interval as its standard error
times two shares, the 5% and 95% points of `(value − truth)/standard error` over the calibration
set's fits, one pair for a log of height alone and one for a log with a horizontal track
([ADR-257][adr-257] §1 holds each interval to three counts on sets A and B). On set B the drag
scale's interval leaves 79 of 1,000 truths below it, where a tail passes with 29 to 74; ADR-368
recorded that gap. M7.3g ([ADR-377][adr-377] put it first in the queue) is to make every
least-squares row pass. The committed report counts each parameter apart for fits whose heights'
RMS residual is under 1 m and for the rest, and one pair of shares covers the two unequally. So
#1108 named the split as the first rule to try, before any count: four pairs a parameter, by the
track and by the residual's class.

**What was measured.** `cargo xtask fit --calibrate` fitted the calibration set in 11 min 27 s on
a 10-core Mac and printed each class's shares. They are fitted on 1,293 fits of logs of height
alone and 997 of logs with a track, the fits with something to fit. At three figures, low and
high, with the count of fits each pair is of:

| parameter | log | heights within 1 m | 1 m or worse |
|---|---|---|---|
| drag scale | height alone | −0.893, 1.25 (739 fits) | −1.73, 1.83 (554) |
| drag scale | with a track | −1.27, 1.60 (729) | −1.32, 2.96 (268) |
| impulse scale | height alone | −1.55, 1.40 | −1.71, 1.62 |
| impulse scale | with a track | −1.51, 1.70 | −1.50, 2.23 |
| wind toward the east | with a track | −1.27, 1.19 | −1.67, 1.71 |
| wind toward the north | with a track | −1.44, 1.27 | −1.76, 1.93 |
| mass | height alone | −1.64, 1.63 | −1.68, 1.73 |
| mass | with a track | −1.66, 1.72 | −1.76, 1.75 |

The other parameters' classes hold as many fits as the drag scale's. The worse class's pair is
the wider in every row. With those shares in force, and set B's drag scale taken off
`KNOWN_GAPS` so that every cell is held, the full `cargo xtask fit` ran for 2 h 17 min on a
10-core Mac and wrote the report at commit 5a4df911 of the branch
[`m7.3g-split-measured`][measured] ([its report][measured-report]). Its least-squares counts,
below / inside / above, against main's report with the shares in force:

| set, parameter | in force | split | split, within 1 m | split, 1 m or worse |
|---|---|---|---|---|
| A, drag scale | 32 / 909 / 59 | 35 / 902 / 63 | 26 / 574 / 41 | 9 / 328 / 22 |
| A, impulse scale | 34 / 911 / 55 | 33 / 914 / 53 | 19 / 589 / 33 | 14 / 325 / 20 |
| A, wind east | 78 / 1350 / 69 | 83 / 1351 / 63 | 69 / 1069 / 39 | 14 / 282 / 24 |
| A, wind north | 58 / 1381 / 58 | 72 / 1367 / 58 | 62 / 1076 / 39 | 10 / 291 / 19 |
| A, mass | 40 / 908 / 52 | 37 / 911 / 52 | 22 / 591 / 28 | 15 / 320 / 24 |
| B, drag scale | 79 / 890 / 31 | **63 / 910 / 27** | **32 / 517 / 12** | 31 / 393 / 15 |
| B, impulse scale | 51 / 912 / 37 | 51 / 915 / 34 | 19 / 523 / 19 | 32 / 392 / 15 |
| B, wind east | 71 / 1353 / 73 | 62 / 1366 / 69 | 35 / 906 / 40 | 27 / 460 / 29 |
| B, wind north | 62 / 1373 / 62 | **49 / 1396 / 52** | **32 / 923 / 26** | 17 / 473 / 26 |
| B, mass | 42 / 900 / 58 | 43 / 899 / 58 | 24 / 499 / 38 | 19 / 400 / 20 |

The run's check named the four bold counts as failing [ADR-257][adr-257] §1's test, and every
other row and class as passing. On set B's wind toward the north the narrowed probe passed too,
where each probe must fail. Each row's probes failed on set A.

**Why set B fails, as far as this shows.** The calibration set alternates set A's model and set
B's, so one pair of shares is a compromise between two kinds of error. Set B's drag scale errs
mostly one way, and within 1 m its interval now leaves too few truths above it: the within
class's pair, −0.893 to 1.25, is fitted on both halves together. That reading is a hypothesis,
not a measurement: no count here splits the calibration set by its model.

**Decisions.**

1. **The split isn't put in force.** It would trade set B's drag scale's failure below for one
   above, and add a failing row, set B's wind toward the north, which passes today. The shares in
   force, the committed report and ADR-368's caveat stay as they are.
2. **M7.3g in three increments.** M7.3g1 is this record and the tools that measured the split.
   M7.3g2 chooses and measures a second rule. M7.3g3 is M7.3g's *done when*, whole, with a rule
   that passed in force. M7.3g is met when M7.3g3 is.
3. **The tools ship.** `cargo xtask fit --calibrate` prints each class's shares with the count of
   fits each pair is of, and `cargo xtask fit --fits a|b <file>` writes a set's counted fits, so
   a rule can be tried on kept fits without fitting a set again. Neither changes a report.
4. **The second rule is chosen on the calibration set alone, its counts predicted first.**
   Sets A and B have now been counted against one rule, so a second is chosen without them: on
   the calibration set's fits, by how each of its two model halves would be covered. Its counts
   on sets A and B are predicted in writing, on #1108, before the full fit counts them. Two
   candidates are named here, neither chosen:
   - each class's shares as the outer of the calibration set's two halves' 5% and 95% points, so
     an interval covers either model, since a fit can't tell which it is flying;
   - the split by the residual's share of the heights' spread, in place of its RMS.
5. **The stop.** Two rules in a row that fail set B end M7.3g. The gap then stays recorded under
   ADR-368, and the pages say what was tried.

**What this does not decide.** Which candidate, or another, M7.3g2 takes. Whether set B's model
is the right test of a fit's intervals: ADR-257 and ADR-368 hold it.

**Consequences.**

- Nothing a user sees changes: `hpr fit` and `FlightLog.fit` state the same intervals with the
  same caveat.
- The measured report stays on the branch `m7.3g-split-measured` at 5a4df911. Running
  `cargo run --profile report-check -p xtask -- fit` there writes it again.
- `xtask/src/fit.rs`: `--calibrate` and `--fits` (M7.3g1).
