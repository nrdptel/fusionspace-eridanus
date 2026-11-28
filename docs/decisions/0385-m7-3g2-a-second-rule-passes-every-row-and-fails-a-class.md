# ADR-385: M7.3g2: a second rule passes every row and fails a class, so M7.3g ends (2026-11-27)

- **Status:** accepted; meets [M7.3g2][m7-3g]; ends M7.3g under [ADR-384][adr-384] decision 5, with M7.3g3 left open and marked blocked; release 0.2 goes ahead with M7.3g ended, as [ADR-377][adr-377] left to this record, amending its §6 and §7 on what 0.2 holds and [M10.2][m10-2]'s *done when*; changes no share, band, count or flight; [ADR-368][adr-368]'s gap stands
- **Summary:** M7.3g's second rule, chosen on the calibration set's fits alone, fits the least-squares shares apart for the calibration set's best-fitting 5%, by the misfit's share of the heights' spread, and for the rest. Its counts were predicted on issue #1108 before the full fit. With it every least-squares row passes on both sets, but set B's drag scale fails in the worse class, with too many truths below. A rule fails when any row or class fails, so M7.3g ends with M7.3g3 open. Release 0.2 goes ahead without it. The tool that chose the rule, `cargo xtask fit --halves`, ships.

[m7-3g]: ../decisions-and-roadmap.md#m7-3g
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
[adr-377]: 0377-no-release-waits-on-users-and-release-0-2-comes-next.md
[adr-384]: 0384-m7-3g-in-three-the-split-by-the-residual-measured.md
[m10-2]: ../decisions-and-roadmap.md#m10-2
[posted]: https://github.com/nrdptel/fusionspace-eridanus/issues/1108#issuecomment-7232299686
[rule]: https://github.com/nrdptel/fusionspace-eridanus/commit/fce39fdf51b3d4f477c1941c8f1deff0d3bb9e5b
[measured]: https://github.com/nrdptel/fusionspace-eridanus/tree/m7.3g2-misfit-measured
[measured-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/425d4c803d47286017f512dd49cd598b5eff611a/validation/reports/fit-intervals.md

**Context.** The least-squares fit states each parameter's 90% interval as its standard error
times two shares, fitted on the calibration set ([ADR-257][adr-257] §1). On set B the drag scale's
interval leaves 79 of 1,000 truths below it, where a tail passes with 29 to 74 (ADR-368's gap).
M7.3g is to make every least-squares row pass. Its first rule, the split by the heights' RMS
residual at 1 m, failed set B ([ADR-384][adr-384]). Decision 4 there has a second rule chosen on
the calibration set alone, with its counts on sets A and B predicted in writing before the full
fit counts them. Decision 5 says a rule fails when any least-squares row or class fails on set A
or set B, and that a second failing rule ends M7.3g. This record reads "class" as the rule's own
classes, the ones its shares are fitted on: here the misfit's two, the *better* class under the
split and the *worse* class at it or past it. The report's classes by the heights' RMS residual
are still shown, but not held. Under this rule they fail 14 of their 20 cells, as many as with the
shares in force (ADR-384), so the rule fails under either reading. Sets A and B are the
[synthetic test sets](../physics/synthetic-flights.md#the-sets): A flies the simulator's own
model, B a deliberately different one. A *row* is one parameter's counts on a set, and a
*probe* is a changed interval ADR-257 requires to fail.

**How the rule was chosen, on the calibration set alone.** `cargo xtask fit --calibrate --fits`
kept the calibration set's fits (11 min on a 10-core Mac; its shares are ADR-384's to the last
digit). `cargo xtask fit --halves` reads them and tries rules on them. The calibration set
alternates set A's model (even flights) and set B's (odd). For each rule, `--halves` fits the
shares on the whole set, at three figures as the library holds them. It counts them on each
model's half as the report counts sets A and B: the drag, impulse and mass on the fits with
something to fit (838 in set A's half, 800 in set B's: 1,638 of the 2,000, the 1,293 logs of
height alone that ADR-384's shares are fitted on and 345 with a track), and the wind on the fits
of flights with a track logger (536 and 461, ADR-384's 997). A half's counts, scaled to the 1,000 or 1,500 fits a test set
counts, are the rule's prediction for that set. No kept fits of set A or B were read.

Counts alone can't choose. The shares in force pass on both halves, yet they failed on set B
(79 below), where their half's count scales to 64. A half is a sample, and so is a set. So
`--halves` also works out a *chance*. It draws 20,000 test sets' counts about each half's: the
spread a set would show by chance, widened by the half's own error in its shares. (In full: the
counts below and above jointly normal, with the multinomial's covariance times
`1 + n_set/n_half`, each share taken as `(x + ½)/(n + 3/2)`.) The chance is the share of draws in
which every row and class passes ADR-257 §1's test and every probe fails, multiplied over the
parameters and both sets.

The chance ranks rules; it doesn't say how often a rule would pass. The halves are small, so it
is low even for a perfect rule. For halves exactly at 5% / 90% / 5%, it gives each drag, impulse
or mass row about 0.93 and each wind row 0.76 to 0.80, so about 0.25 for the 10 rows together.
Adding a class or a probe can only lower that. The chance also draws each probe apart from its
own row, and the parameters apart, though one flight's errors in them are tied; what that does to
it isn't measured. A rule whose intervals hold exactly 90% would pass each row of 1,000 fits
0.9977 of the time and each of 1,497 0.9976, so all 10 together 0.977 (the multinomial worked
exactly). A test of `fit --halves` pins these figures. The candidates were the
split by the misfit's share at each 5% of its fits:

| rule | chance both sets pass | measured on sets A and B |
|---|---|---|
| the shares in force | 0.042 | fails (set B's drag scale) |
| the split by the residual at 1 m | 0.002 | fails (two rows, two classes) |
| the split by the misfit's share at its 5% point, 0.000634 | 0.022 | fails (see below) |
| the split at its 90% point, 0.00338 | 0.012 | not measured |
| the split at any other 5% step from 10% to 85% | 0.001 to 0.007 | not measured |
| the split at its 95% point | 0.000 | not measured |

Of the splits, the one at the 5% point has the highest chance, and it was chosen. The shares in
force score higher, but they were measured failing, so they can't be the second rule. Two
findings come from the calibration set alone. First, the misfit's share separates the models only
a little. At its median, 0.00113, 48% of set A's half and 45% of set B's fall under it on the
drag, impulse and mass (401 of 838 and 360 of 800), and 63% and 56% on the wind's fits.
(`fit --halves --misfit 0.0011285539583090693` prints the classes' sizes; the post on #1108 gave
56% and 55%, which are the sizes at the 60% point.) At the 5% point the better class is mostly
set A's, 68 of 838 against 25 of 800, but it is too small to carry set B's errors. Second, every
rule tried is far under the chance's ceiling of about 0.25: none is close to fitting both halves
at once. The shares in force's rows pass with chances from 0.57 (set B's wind toward the east),
where a perfect rule's would be 0.76 or more.

**The prediction.** The rule was put in force, with set B's drag scale off `KNOWN_GAPS` so that
every cell is held, at commit [fce39fdf][rule]. Its counts were [posted on #1108][posted] before
the full fit ran. The prediction was that the rule fails, with a chance of 0.022 that it passes
everything. Taken as they stood, the predicted counts failed in one cell, set A's wind toward
the north in the better class.

**What was measured.** The full `cargo xtask fit` at fce39fdf ran for 2 h 17 min on a 10-core
Mac and wrote the report at commit 425d4c80 of the branch [`m7.3g2-misfit-measured`][measured]
([its report][measured-report]). Its least-squares counts, below / inside / above, against the
prediction:

| set, parameter | row, predicted | row, measured | better class, predicted | better class, measured | worse class, predicted | worse class, measured |
|---|---|---|---|---|---|---|
| A, drag scale | 32 / 913 / 55 | 32 / 907 / 61 | 0 / 77 / 4 | 2 / 89 / 4 | 32 / 836 / 51 | 30 / 818 / 57 |
| A, impulse scale | 36 / 919 / 45 | 34 / 913 / 53 | 2 / 79 / 0 | 0 / 91 / 4 | 33 / 841 / 45 | 34 / 822 / 49 |
| A, wind east | 62 / 1374 / 64 | 82 / 1345 / 70 | 3 / 84 / 0 | 2 / 70 / 2 | 59 / 1290 / 64 | 80 / 1275 / 68 |
| A, wind north | 64 / 1372 / 64 | 58 / 1380 / 59 | 0 / 87 / 0 | 1 / 72 / 1 | 64 / 1285 / 64 | 57 / 1308 / 58 |
| A, mass | 44 / 907 / 49 | 39 / 910 / 51 | 4 / 75 / 2 | 1 / 89 / 5 | 41 / 831 / 47 | 38 / 821 / 46 |
| B, drag scale | 59 / 902 / 39 | 74 / 895 / 31 | 3 / 28 / 0 | 1 / 24 / 2 | 56 / 874 / 39 | **73 / 871 / 29** |
| B, impulse scale | 56 / 896 / 48 | 51 / 912 / 37 | 0 / 28 / 3 | 0 / 26 / 1 | 56 / 868 / 45 | 51 / 886 / 36 |
| B, wind east | 85 / 1334 / 81 | 74 / 1350 / 73 | 0 / 20 / 0 | 1 / 10 / 0 | 85 / 1314 / 81 | 73 / 1340 / 73 |
| B, wind north | 78 / 1344 / 78 | 66 / 1370 / 61 | 0 / 20 / 0 | 0 / 11 / 0 | 78 / 1324 / 78 | 66 / 1359 / 61 |
| B, mass | 43 / 908 / 49 | 43 / 900 / 57 | 0 / 31 / 0 | 0 / 26 / 1 | 43 / 877 / 49 | 43 / 874 / 56 |

The predicted wind rows are of 1,500 fits; the sets count 1,497. The run's check named one count
as failing ADR-257 §1's test, the bold one: set B's drag scale in the worse class, with 73 truths
below of 973. That is too many: the 99.9% band on 73 of 973 runs from 5.01% to 10.7%, so it
leaves out the 5% a 90% interval needs. It is the side of ADR-368's gap, where the drag reads
high. Every row passes on both sets, and every probe fails. Set B's drag scale passes as a row at the edge of its band, with 74 below of
1,000 (its band runs from 4.95%), and the class fails by one truth: 72 of 973 would pass. None of
the rules M7.3g tried before passed every row: the shares in force fail set B's drag scale, and
the split by the residual two rows. The prediction was right that the rule fails, but wrong about
where. The cell it named passes. Set B's drag scale, its row, classes and probes together, had a
chance of 0.72 to pass (`fit --halves --misfit 0.000634`).

**Decisions.**

1. **The rule isn't put in force.** A class fails, so by ADR-384 decision 5 the rule fails. The
   shares in force, the committed report and ADR-368's caveat stay as they are.
2. **M7.3g ends.** The second rule failed, so by decision 5, M7.3g ends: M7.3g2 is met, M7.3g3
   stays open, marked blocked, and M7.3g leaves the queue. M2.8 is next. A third rule, or a
   change to the test that M7.3g3 holds a rule to, needs a new record. The gap stays recorded under
   ADR-368, and the fit's page says what was tried.
3. **Release 0.2 goes ahead with M7.3g ended.** ADR-377 said that if M7.3g's fix couldn't pass
   its test, release 0.2 would wait on this decision, not on users. It doesn't wait for a third
   rule. The gap errs on the side that flatters: a drag scale that reads high makes a flight
   predicted with it come out low, the wrong side for a waiver's ceiling (ADR-368). That is why
   it is a recorded gap with a caveat, not a number left unsaid. ADR-381 put the fit on main with
   this interval and ADR-368's caveat, and every surface that states the drag interval carries
   it. This decision adds no surface, and the report keeps the gap visible.
   M10.2's *done when* now reads "M7.3g (met, or ended with its gap recorded: ADR-385)". That
   clause can no longer fail on M7.3g.
4. **The tool ships.** `cargo xtask fit --halves <file>` tries rules on the calibration set's kept
   fits. It prints what fails and the chance for the shares in force, the split by the residual
   and the splits by the misfit's share at each 5% of its fits; `--misfit <share>` prints one
   split in full. It reads no test set's fits and changes no report. `fit --fits` now checks
   that it can write its file before it fits, and refuses a set with fewer than the 1,000 fits
   the report counts.

**What this does not decide.** Whether a third rule is worth trying. M7.3g3's bar is within reach
of a rule that is calibrated on both sets: one whose intervals hold exactly 90% passes all 10 rows
0.977 of the time. What stands in the way is that set A's model and set B's err differently, and
no class tried here tells them apart (ADR-384's reading of why, still a hypothesis). Nor does it
decide whether set B's model is the right test of a fit's intervals: ADR-257 and ADR-368 hold it.

**Consequences.**

- Nothing a user sees changes: `hpr fit` and `FlightLog.fit` state the same intervals with the
  same caveat.
- The measured report stays on the branch `m7.3g2-misfit-measured` at 425d4c80, beside ADR-384's
  `m7.3g-split-measured`. Running `cargo run --profile report-check -p xtask -- fit` there writes
  it again, then fails its check on the one count.
- `xtask/src/fit/halves.rs`: `--halves` and its chance; `xtask/src/fit.rs`: the subcommand, and
  `fit --fits`' checks (M7.3g2).
