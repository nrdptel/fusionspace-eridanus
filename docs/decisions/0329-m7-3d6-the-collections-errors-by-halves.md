# ADR-329: M7.3d6, the collection's errors by halves (2026-11-08)

- **Status:** accepted; carries out [M7.3d6][m7-3] under [ADR-326][adr-326] §1 and [ADR-322][adr-322] §2.
- **Summary:** Each private collection flight with a cut log, among the flights the forward report compares, goes to the fit up to its cut, on the flight as `fixture-flights` flies it in HPR Sim, and is flown on to apogee: 21 predicted, 12 in the calibration half and 9 held out; 1 has too little up to its cut to fit. The prediction's mean absolute apogee error is 3.22% in the calibration half and 3.56% in the held-out half, against the forward simulation's 9.35% and 16.86% on the same flights; the paired bootstrap 95% intervals on the difference are −10.07 to −2.70 and −19.01 to −7.97 points. The held-out half's fit aggregates stay private; their file's SHA-256 is committed and checked.

[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-325]: 0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md
[adr-327]: 0327-m7-3d5-the-fall-must-hold-and-still-fall.md
[i181]: https://github.com/nrdptel/fusionspace-eridanus/issues/181
[i1332]: https://github.com/nrdptel/fusionspace-eridanus/issues/1332
[i359]: https://github.com/nrdptel/fusionspace-eridanus/issues/359
[i364]: https://github.com/nrdptel/fusionspace-eridanus/issues/364
[m7-3]: ../ROADMAP.md#phase-5-flight-data-and-forensics

**Context.** [ADR-326][adr-326] sealed the held-out flights (M7.3d4) and [ADR-327][adr-327] fixed the cut (M7.3d5):
of the collection's 64 chosen logs, 26 are cut, none inside its motor's burn where the burn is
known. M7.3d6 asks for each cut log's fit flown to apogee as `fixture-flights` flies it, each
half's errors beside the forward simulation's on the same flights, [ADR-257][adr-257] §3's paired
bootstrap 95% intervals, and the calibration half's fit aggregates, the held-out half's kept
private with their file's hash committed. Choices 1 to 4, 7, and 5's bootstrap and its seed
were made before any half's number was computed. The set without deployments (6), the outward
rounding (5) and the log-choice count (8) came after the first numbers; the rounding moved a
printed bound by 0.01 at most, and the others add numbers without changing any.

**Decision.**

1. **The flight is the forward simulation's.** The fit's model flies the flight's `.ork`, its
   configuration with the separations the climb flies, its rail and its day's ERA5 air, with
   `accept_design_errors` as the forward report sets it. It changes the motors' impulse and burn,
   the drag, the structure's mass and the wind, which is constant with height, as the public
   flights' report does ([ADR-325][adr-325]). Flown with nothing changed in ERA5's own wind it
   must land within 0.5 m of the forward apogee, or the run fails. It lands within 0.000 m on
   every flight: both read the same apogee sample. With the drag 1% higher in that check, one
   flight landed 0.98 m off and the run stopped (run once, not committed). The structure's mass
   scale holds until the flight first splits (`Simulation::with_structure_mass_scale`), so on a
   staged flight it acts on the boost alone; its stated range is ±2%, and the calibration half's
   fits move it by 0.002 (standard deviation).
2. **Only the flights the forward report compares.** A flight the forward report leaves out, or
   that hpr or OpenRocket doesn't fly, has no forward error to stand beside, and a flight HPR Sim
   doesn't fly has no flight to fit. They are counted under their reason, not predicted: 4 cut
   flights, 2 in each half. HPR Sim's forward simulation refuses the design of 3 (the forward report lists its
   refusals by cause, open issues such as [#359][i359] and [#181][i181]); the collection has no ERA5 file of the
   4th flight's day ([#364][i364]), counted under that cause, as the forward report counts it. So
   every cut log `fixture-flights` flies in HPR Sim goes to the fit (none of the 26 was left
   out or missed by OpenRocket alone): 21 are
   fitted and flown on to apogee, and 1 is refused as too little up to its cut to fit
   (`Unpredicted::NothingToFit`: no pad or height track read, a window up to the cut empty or
   too long, or no more bins with a sample than the fit's 11 unknowns). The
   *done when*'s "each cut log's fit" is read over the cut logs that have a flight to fly and a
   fit, as the public flights' report counts its flights with no prediction; the 5 without are
   counted on the page under their reason, not dropped.
3. **One reference for both errors.** The prediction's apogee is the height climbed. It is set
   against the manifest's logged apogee as a height climbed in the day's air, the number the
   forward errors use. It is not the chosen log's own reading: that log may be another altimeter
   in the same rocket.
4. **What the fit is told is the public report's.** The synthetic flights' stated ranges, the
   wind centered on ERA5's averaged evenly in height over the forward climb. Tuning them on these
   flights would be a model choice drawing on the held-out half ([ADR-209][adr-209] §4).
5. **The interval.** The difference is each flight's absolute prediction error less its forward
   one, as on the public flights. Its paired bootstrap interval is the percentile interval
   (Efron and Tibshirani, 1993, §13.3) on 10,000 resamples, seed 329, both fixed before any number
   (`hpr_forensics::boost::paired_bootstrap_interval`). On a few flights (each set here has 12 or
   fewer) a percentile bootstrap can cover less than its level (§13.5; the coverage test runs on
   40), so the exact sign-flip interval stands beside it. On 3 flights
   or fewer a 95% percentile interval is only the smallest and largest difference, which holds a
   symmetric center 75% of the time on 3 and gives two flights' own values: the page says "too few
   flights to resample" there instead. The page rounds each interval's ends outward.
6. **Two sets a half.** Every flight predicted, and those without a deployment before apogee,
   the forward report's own second set: 10 of the 21 predicted flights deployed something before
   apogee, which lowers the logged apogee somewhat beneath both methods; the fit reads the log
   only up to its cut. Without them the difference is −7.02 points (8 flights) and −7.32 (3).
7. **The held-out fits, sealed.** The drag, impulse and structure mass scales' mean and standard
   deviation, the fits converged and their mean RMS are written for each half. The held-out
   half's go to the gitignored `corpus-out/hpr-sim-fixtures/held-out-fits.json`. Its SHA-256 is
   committed in `validation/reports/held-out-fits.sha256`, and `fixture-flights --check` fails
   when the recomputed aggregates' digest differs, or the private file, where this machine has
   it, isn't them. Nothing is given per flight ([ADR-151][adr-151]); each flight's outcome goes
   to the gitignored `corpus-out/hpr-sim-fixtures/collection-predictions.json`, for checking the
   aggregates on this machine. The page carries the collection's and ERA5's credits.

8. **The log's choice, bounded by a count ([#1332][i1332]).** The rule takes, of a flight's logs
   on a barometer's track, the one whose apogee is nearest the logged one. A gain error in a log
   would scale its boost's heights too, so the page counts where the rule had a choice: 13 of the
   21 predicted flights had another such log (6 and 7 by half), and on none did those logs'
   apogees lie more than 2% apart. Only the chosen log is cut now; the counts didn't move. The
   heights are like with like: the fit sets the log's readings against the simulated barometer's
   readings in the day's air, and both errors set a height climbed against the logged apogee as
   a height climbed. #1332's test of the counting on invented logs stays open.

**What it shows.** The prediction's mean absolute error is below the forward simulation's in
both halves, and in both sets of each. Each paired bootstrap interval the page gives lies below
zero, and so does each sign-flip interval but one: the held-out half's 3 flights without a
deployment before apogee are too few to resample, and their sign-flip interval is unbounded. In
the held-out half the forward error falls from 16.86% to 10.40% without the deployments while
the prediction's moves from 3.56% to 3.08%; why the forward error moves more isn't traced. The calibration half's fits read the motor's impulse
0.957 on average (standard deviation 0.033) and the drag 1.018 (0.048). That is not a measured
impulse: the scale takes up some of what the model gets wrong in the boost, as the other scales do. The forward errors on
these flights run higher than on the whole collection (+8.87% and +15.83% with their sign,
against +7.76% on the 60 flights compared); why isn't traced. The halves aren't blind: the
forward errors on every flight were published before the seal ([ADR-322][adr-322]).

**What doesn't change.** M7.3d7's *done when*, and with it M7.3d's, stands whole: the scoreboard
row, each other tool's cell read from its docs.

**Consequences.** `fixture-flights --check`, in the gate's `refs` step, sends 22 flights to the
fit in parallel: 114 s against 100 s before, one run each on the maintainer's machine. CI, without `refs/`, holds the page to its data, the counts to each
other and the held-out fits' digest to its form.
