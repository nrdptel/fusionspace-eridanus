# The private collection's logs, cut at the boost

This page counts how many of the [private collection's](fixture-flights.md) flights have a barometer's log that a prediction from the boost can start from: the log cut after burnout, fitted, and flown on to apogee, as the [public flights' report](from-boost.md) does for 7 public flights. Then it gives those predictions' apogee errors for each half of the flights, beside the forward simulation's on the same flights. The collection's license allows only aggregates, so no flight is named.

Written by `cargo xtask fixture-flights`; do not edit. It is the third of four steps toward the collection's numbers for apogee from the boost (M7.3d6, decision record [ADR-326](../../docs/decisions/0326-m7-3d3-in-four-the-halves-sealed-first.md); this step's choices are in [ADR-329](../../docs/decisions/0329-m7-3d6-the-collections-errors-by-halves.md)).

**The halves.** The predictions' errors are given for two halves: a held-out share of the flights, kept from every later choice of model ([ADR-209 §4](../../docs/decisions/0209-the-2026-10-08-best-on-the-market-measured.md)), and the rest. The collection's [forward errors](fixture-flights.md#apogee-errors) (hpr's simulated apogee against the logged one) were published before the held-out digest was sealed on 2026-11-07, so the model choices made before then could draw on every flight: the held-out flights are kept only from choices made after it (held out, not blind). A flight is held out when a hash of its id puts it there: the first eight bytes of the SHA-256 of the id followed by `ADR-209`, read as a big-endian number, fall below 2⁶⁴ / 3, a third of the flights in expectation ([ADR-322](../../docs/decisions/0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md)). The held-out ids stay private ([ADR-151](../../docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md)). The SHA-256 of their sorted ids is committed in [`held-out.sha256`](held-out.sha256) before any half's number, so any later change to which flights are held out shows as a change to that hash. This page gives each half's errors, and the calibration half's fits as aggregates.

**Each flight's log.** A flight may have several logs, from more than one altimeter or file.

- **Which files:** those the collection's catalog lists for the flight, outside its `_scratch/` folder, which the collection's pinned version doesn't fix. Each is read on its own: a workbook without exactly one sheet that reads as a log, or a log that needs a second file or a stated column layout to read, counts as unread. A table whose altitude column doesn't say which sensor measured it reads, but gives no apogee, so it can't be chosen.
- **Which log:** of those whose height track is a barometer's (the logger's altitude, a pressure, or a barometric altitude channel, never a GNSS receiver's) and gives an apogee, the one whose apogee is nearest the logged apogee the collection states; on a tie, the first in the catalog's order.
- **The cut:** the library's boost finder cuts it where its climb speed falls from its top, or says why it can't.

| | flights |
|---|---:|
| in the collection's manifest | 83 |
| left out of the apogee errors by the manifest | 11 |
| no file reads on its own | 3 |
| a file read, but none gives an apogee on a barometer's track | 5 |
| a log chosen | 64 |
| chosen, then refused a cut | 38 |
| … refused: a log slower than 8 Hz | 2 |
| … refused: the transonic limit before a cut | 36 |
| chosen and cut | 26 |
| … cut less than the motor's burn after liftoff | 0 |
| … cut, the motor's burn or the log's liftoff not known | 6 |

Of the flights not left out, 241 files were tried, 199 read on their own and 117 gave an apogee on a barometer's track. Of the chosen logs, 1 read an apogee more than 2% from the logged one: the nearest log may be another altimeter's, or a reading the analyzer takes differently. The 2% is a line for counting, not a filter.

**The refusals.** The finder refuses a log whose fitted climb speed reaches 227.7 m/s (747 ft/s), Mach 0.8 in the coldest air, before its cut: nearer the [speed of sound](../../docs/glossary.md#transonic-and-supersonic) a barometer's reading may follow the airflow, not the height. A log in which no half second holds 5 samples (under 8 a second) is too sparse to follow the climb.

**Cuts inside the burn.** A barometer's reading can dip in the boost and catch up, so the finder cuts only where the climb speed's fall has held 0.75 s and the speed is the lowest it has been over that span ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)). A cut counts as inside the burn when it comes less than the motor's burn after the log's liftoff: the burn is the end of the motor's thrust curve, as the flight's `.ork` and the curves supplied for it give it; a design with more than one motor or a staging event counts as not known. Liftoff follows ignition, so that also takes in a cut up to that lag after the burn ends. A prediction from such a cut wouldn't be the method's case.

**The predictions.** Each cut flight that the [forward report](fixture-flights.md) compares is flown as it flies it in hpr: its `.ork` and motor configuration, its rail and its day's ERA5 air, with nothing deployed. The fit changes only the motor's impulse and burn time, the drag, the structure's mass and a wind constant with height, centered on ERA5's wind averaged over the climb. Flown with nothing changed in ERA5's own wind, the fit's model must land within 0.50 m of the forward simulation's apogee; the largest miss was 0.000 m. The log up to the cut is fitted and the fit flown on to apogee, as the [public flights' report](from-boost.md) does. Both errors are against the same number: the logged apogee as a height climbed in the day's air, as the forward report takes it. Each set's last three rows give the difference and its intervals: each flight's absolute error from the prediction less the forward simulation's, averaged, in percentage points; below zero, the prediction came closer.

| | calibration half | held-out half |
|---|---:|---:|
| flights cut | 14 | 12 |
| … no prediction: hpr's forward simulation doesn't fly it | 1 | 2 |
| … no prediction: the collection has no ERA5 file of the flight's day | 1 | 0 |
| … no prediction: too little up to the cut to fit | 0 | 1 |
| predicted | 12 | 9 |
| … with a deployment before apogee | 4 | 6 |
| … cut less than the motor's burn after liftoff | 0 | 0 |
| … cut, the burn or the liftoff not known | 2 | 1 |
| … with another log of the flight on a barometer's track | 6 | 7 |
| … … the logs' apogees more than 2% apart | 0 | 0 |
| **every flight predicted** | **12** | **9** |
| prediction from the boost: mean absolute error | 3.19% | 3.60% |
| … mean error with its sign | -0.08% | +1.34% |
| forward simulation, same flights: mean absolute error | 9.32% | 16.75% |
| … mean error with its sign | +8.84% | +15.73% |
| forward error made up to the cut, forward less prediction (points) | +8.92 | +14.39 |
| … flights where it is the larger share | 11 | 9 |
| absolute errors' difference, prediction less forward (points) | -6.13 | -13.16 |
| … paired bootstrap 95% interval | -10.08 to -2.71 | -18.77 to -7.93 |
| … exact sign-flip 95% interval | -10.64 to -2.08 | -20.29 to -6.49 |
| **without a deployment before apogee** | **8** | **3** |
| prediction from the boost: mean absolute error | 3.03% | 3.08% |
| … mean error with its sign | -1.12% | -1.54% |
| forward simulation, same flights: mean absolute error | 10.05% | 10.40% |
| … mean error with its sign | +9.33% | +10.40% |
| forward error made up to the cut, forward less prediction (points) | +10.44 | +11.93 |
| … flights where it is the larger share | 7 | 3 |
| absolute errors' difference, prediction less forward (points) | -7.02 | -7.32 |
| … paired bootstrap 95% interval | -12.65 to -2.29 | too few flights to resample |
| … exact sign-flip 95% interval | -13.76 to -0.80 | unbounded |

Both methods fly nothing deployed, and the fit reads the log only up to its cut, so a deployment before apogee lowers the logged apogee beneath both; the rows without those flights leave them out, as the forward report's own set does. Where a flight has other logs on a barometer's track, the rule above takes the one nearest the logged apogee; two rows count how often there was such a choice, and how often the candidates' apogees lay more than 2% apart, where the choice could move an error. Interval ends are rounded outward. The bootstrap resamples each half's flights 10,000 times (seed 329), each flight's two errors kept together, and takes the 2.5% and 97.5% points of the resampled means. On a few flights it can cover less than 95%, so the exact sign-flip interval, which the public flights' report uses, stands beside it: it assumes only that each flight's difference is drawn independently and symmetrically about the center. On 3 flights or fewer the bootstrap's 95% points are only the smallest and largest difference, so the page gives none. On 5 flights or fewer no pattern of signs is rare enough at 95%, so the sign-flip interval is the whole line: "unbounded".

**Up to the cut and after it.** The cut is where the fitted part of a log ends, after burnout. One row of each set gives the share of the forward error made up to the cut, in percentage points of the logged apogee; above zero, the forward simulation had climbed too high by then. The row under it counts the flights whose share up to the cut is the larger of the two in size. The prediction follows the log up to the cut and the model from there, so its error, in the rows above, is the share made after the cut, in the coast. The forward error less the prediction's is the share made up to the cut, where the motor's thrust, the rocket's mass and the boost's own drag set the climb: taking the boost from the log removes it. A flight's two shares add up to its forward error. The fit also moves the drag, the mass and the wind within what it was told of them, and those carry into the coast, so the split is of these two simulations' errors, not a measurement of the motor or the drag alone.

**The calibration half's fits.** 12 fits of the logs up to the cut, 12 of them converged; their residuals' RMS averages 2.72 m. Over those fits:

| | mean | standard deviation |
|---|---:|---:|
| drag scale | 1.018 | 0.049 |
| impulse scale | 0.958 | 0.033 |
| structure mass scale | 1.003 | 0.002 |

**The held-out half's fits,** kept private until the correction below was chosen on the calibration half alone, are published with it: 9 fits, 9 of them converged; their residuals' RMS averages 2.90 m. Their file still matches the SHA-256 committed in [`held-out-fits.sha256`](held-out-fits.sha256) before the correction was fitted.

| | mean | standard deviation |
|---|---:|---:|
| drag scale | 1.044 | 0.047 |
| impulse scale | 0.936 | 0.045 |
| structure mass scale | 1.005 | 0.003 |

**The calibration half's logs alone.** Each fit above was told that the motor delivers its rated impulse to a standard deviation of 0.033 (a 90% range of 1 give or take 0.054), and that pulls its impulse scale toward 1. For what a log alone says, each of the calibration half's 12 predicted flights is fitted once more with that range made 100 times wider and nothing else changed: 12 fits, 12 of them converged. Such a fit's impulse scale is the impulse the climb to the cut shows over the motor's rated impulse, at the mass in the files and the model's drag. A barometer's boost can't tell less impulse from more mass ([why](../../docs/research/drag-impulse-identifiability.md)), so a scale below 1 can be a motor that delivered less, a rocket built heavier than its files, or something the model gets wrong in the boost. The fit can't say which. The table is over the converged fits: a few flights of one half of one collection, so the mean says what these logs show, not what motors deliver.

| | calibration half |
|---|---:|
| impulse scale, the log alone: mean | 0.936 |
| … the mean's standard error | 0.014 |
| … standard deviation | 0.048 |
| … its standard error in a fit, averaged | 0.035 |
| the drag scale's standard error in those fits, averaged | 0.108 |
| fits with the scale below 1 | 12 |
| flights flagged: the log and the files disagree | 1 |

The fits were told the drag scale to a standard deviation of 0.109, and they come back knowing it little better. In a boost the drag and the impulse trade against each other, so the climb to the cut says almost nothing of the drag by itself: a drag scale has to come from the coast.

A flight is flagged when its log and its files disagree by more than NAR's retest rule allows one firing: its impulse scale's distance from 1, less 2 of its own standard errors, is more than 13.1%, in a fit that converged. NAR, one of the bodies that certify motors, tests a motor again as a new type when one recertification firing's total impulse is more than 13.1% from the certified mean (NAR Motor Testing Manual v1.5, 2011, §6.2.3). The manual says that of a test firing; this page takes it as the most a flown motor's lot strays, an inference. Past it the lot is an unlikely cause, and the files' mass or motor, or the model of the boost, a likelier one. The flag removes no flight from this page's errors. [ADR-346, the rule before the fit](../../docs/decisions/0346-m7-5-in-three-the-rule-before-the-fit.md), fixes how the correction of the shared bias is to be fitted on these scales and judged with and without the flagged flights, before any correction was fitted or flown. The held-out half's logs are fitted this way too, below, for their flags alone.

**The correction of the shared bias, fitted and judged (M7.5b).** [ADR-346](../../docs/decisions/0346-m7-5-in-three-the-rule-before-the-fit.md) fixed, before any of this was computed, how a correction of the bias the simulators share is fitted on the calibration half and judged on the held-out half and the public flights. It is two scales: one on every motor's impulse, one on the drag. The drag scale comes from the calibration flights' coasts: for each flight without a deployment before apogee, the drag at which its boost, refitted with the drag held there, flies on to the logged apogee (a bisection from 0.25 to 4, to 0.001); the scale is their mean, its 95% interval from Student's t. The impulse scale comes from the same flights' boosts, every one this time, refitted at that drag: their scales' mean weighted by one over each fit's variance plus a motor's own spread (0.0328, what the fit is told of one motor's impulse), its interval from the scales' weighted spread. A scale enters only if at least 3 values give an interval that excludes 1, rounded to two decimals. Two fits are made: over every calibration flight predicted, and without the ones flagged above. Some numbers are held back: the collection's terms allow only figures over several flights, and two published figures over sets that differ by one or two flights would give those flights' own values by subtraction.

Each fit's correction is flown on every flight the forward report compares, and on the public flights as [their report](real-flights.md) flies them: every motor's thrust and propellant scaled by the impulse scale at its rated burn time, the drag by the drag scale, nothing else changed. Flown with both scales at 1, a collection flight lands at most 0.00 m from its forward apogee and a public flight at most 0.00 m (the rule allows 0.5 m). It passes only if, on the held-out flights and on those without a deployment before apogee, each with and without their flagged flights, its mean absolute error and the size of its mean error are both lower than the forward simulation's, and on the public flights, with and without theirs, neither is higher; both fits must pass. The fits rest on boosts below Mach 0.8, the cut's limit, and are flown on faster flights too.

**The verdict: fail.** 8 clauses failed, all on the public flights, so no correction ships. Each, by fit and set:

- without the flagged flights, public flights the public report compares: the mean absolute error is not at most the forward simulation's.
- without the flagged flights, public flights the public report compares: the absolute bias is not at most the forward simulation's.
- without the flagged flights, public flights the public report compares, less their flagged flights: the mean absolute error is not at most the forward simulation's.
- without the flagged flights, public flights the public report compares, less their flagged flights: the absolute bias is not at most the forward simulation's.
- every calibration flight, public flights the public report compares: the mean absolute error is not at most the forward simulation's.
- every calibration flight, public flights the public report compares: the absolute bias is not at most the forward simulation's.
- every calibration flight, public flights the public report compares, less their flagged flights: the mean absolute error is not at most the forward simulation's.
- every calibration flight, public flights the public report compares, less their flagged flights: the absolute bias is not at most the forward simulation's.

Flagged flights: 1 of the calibration half's predicted flights, 1 of the held-out set's (0 of those without a deployment before apogee), from 9 held-out logs fitted alone for the flag and for nothing else, and 0 of the public flights, from the 2 whose boost the [public boost report](from-boost.md) fits. Fewer than 3 calibration flights are flagged, so, as ADR-346 §7 says, the fit without them is given by its scales as flown, its counts and whether each interval excludes 1, and the fit over every calibration flight by its pass or fail on each set, with the public flights' numbers in full. A collection set less fewer than 3 flagged flights is given by its count and its pass or fail. What is held back is in a private file whose SHA-256 is committed in [`correction-held-back.sha256`](correction-held-back.sha256).

| | without the flagged flights | every calibration flight |
|---|---:|---:|
| calibration flights predicted | 11 | 12 |
| **drag scale** |  |  |
| … coasts solved, of those without a deployment before apogee | 7 of 7 | held back |
| … the values' mean | held back | held back |
| … 95% interval | held back | held back |
| … the interval excludes 1 | no | held back |
| … as flown | 1.00 | held back |
| … entered, or why not | its interval holds 1 | held back |
| **impulse scale** |  |  |
| … fits converged, of the flights predicted | 11 of 11 | held back |
| … the values' mean | held back | held back |
| … 95% interval | held back | held back |
| … the interval excludes 1 | yes | held back |
| … as flown | 0.94 | held back |
| … entered, or why not | it entered | held back |

The test. Each set's last rows give each flight's absolute error corrected less forward, averaged, in percentage points (below zero, the correction came closer), with its paired bootstrap interval (10,000 resamples, seed 346) and exact sign-flip interval, and how many flights' errors grew: beside the rule, not in it.

| | without the flagged flights | every calibration flight |
|---|---:|---:|
| **held-out flights the forward report compares** | **23** | **23** |
| forward simulation: mean absolute error | 16.26% | held back |
| corrected: mean absolute error | 10.91% | held back |
| forward simulation: mean error with its sign | +13.36% | held back |
| corrected: mean error with its sign | +5.84% | held back |
| corrected flights that failed to fly | 0 | held back |
| passes | yes | yes |
| absolute errors' difference, corrected less forward (points) | -5.35 | held back |
| … paired bootstrap 95% interval | -7.40 to -2.95 | held back |
| … exact sign-flip 95% interval | not counted past 20 flights | held back |
| flights whose error grew | 4 | held back |
| **held-out flights the forward report compares, less their flagged flights** | **22** | **22** |
| forward simulation: mean absolute error | held back | held back |
| corrected: mean absolute error | held back | held back |
| forward simulation: mean error with its sign | held back | held back |
| corrected: mean error with its sign | held back | held back |
| corrected flights that failed to fly | held back | held back |
| passes | yes | yes |
| absolute errors' difference, corrected less forward (points) | held back | held back |
| … paired bootstrap 95% interval | held back | held back |
| … exact sign-flip 95% interval | held back | held back |
| flights whose error grew | held back | held back |
| **… without a deployment before apogee** | **13** | **13** |
| forward simulation: mean absolute error | 11.30% | held back |
| corrected: mean absolute error | 6.84% | held back |
| forward simulation: mean error with its sign | +6.88% | held back |
| corrected: mean error with its sign | -0.05% | held back |
| corrected flights that failed to fly | 0 | held back |
| passes | yes | yes |
| absolute errors' difference, corrected less forward (points) | -4.46 | held back |
| … paired bootstrap 95% interval | -7.06 to -1.52 | held back |
| … exact sign-flip 95% interval | -7.75 to -1.09 | held back |
| flights whose error grew | 3 | held back |
| **… without a deployment before apogee, less their flagged flights** | **13** | **13** |
| forward simulation: mean absolute error | held back | held back |
| corrected: mean absolute error | held back | held back |
| forward simulation: mean error with its sign | held back | held back |
| corrected: mean error with its sign | held back | held back |
| corrected flights that failed to fly | held back | held back |
| passes | yes | yes |
| absolute errors' difference, corrected less forward (points) | held back | held back |
| … paired bootstrap 95% interval | held back | held back |
| … exact sign-flip 95% interval | held back | held back |
| flights whose error grew | held back | held back |
| **public flights the public report compares** | **12** | **12** |
| forward simulation: mean absolute error | 9.00% | 9.00% |
| corrected: mean absolute error | 12.16% | 13.21% |
| forward simulation: mean error with its sign | -4.91% | -4.91% |
| corrected: mean error with its sign | -12.01% | -13.21% |
| corrected flights that failed to fly | 0 | 0 |
| passes | no | no |
| absolute errors' difference, corrected less forward (points) | +3.16 | +4.21 |
| … paired bootstrap 95% interval | -0.31 to 6.30 | 0.51 to 7.52 |
| … exact sign-flip 95% interval | -0.87 to 7.18 | -0.03 to 8.44 |
| flights whose error grew | 8 | 9 |
| **public flights the public report compares, less their flagged flights** | **12** | **12** |
| forward simulation: mean absolute error | 9.00% | 9.00% |
| corrected: mean absolute error | 12.16% | 13.21% |
| forward simulation: mean error with its sign | -4.91% | -4.91% |
| corrected: mean error with its sign | -12.01% | -13.21% |
| corrected flights that failed to fly | 0 | 0 |
| passes | no | no |
| absolute errors' difference, corrected less forward (points) | +3.16 | +4.21 |
| … paired bootstrap 95% interval | -0.31 to 6.30 | 0.51 to 7.52 |
| … exact sign-flip 95% interval | -0.87 to 7.18 | -0.03 to 8.44 |
| flights whose error grew | 8 | 9 |
| **calibration flights the forward report compares (fitted on these: not a test)** | **37** | **37** |
| forward simulation: mean absolute error | 9.42% | held back |
| corrected: mean absolute error | 8.13% | held back |
| forward simulation: mean error with its sign | +4.22% | held back |
| corrected: mean error with its sign | -2.22% | held back |
| corrected flights that failed to fly | 0 | held back |
| absolute errors' difference, corrected less forward (points) | -1.28 | held back |
| … paired bootstrap 95% interval | -3.12 to 0.54 | held back |
| … exact sign-flip 95% interval | not counted past 20 flights | held back |
| flights whose error grew | 16 | held back |

"Held back" marks a number ADR-346 §7 keeps off this page, where a set or a fit differs from a published one by fewer than 3 flights. The calibration flights are the ones the correction was fitted on, so their errors say how well it fits, not how well it predicts. The impulse scale stands for the boost's whole shortfall: a motor that delivered less, a rocket built heavier than its files, or the model of the boost; the drag scale carries what the as-built mass does to the coast too.

**Sources.** Flight data published by university rocketry teams, rocketry courses and hobbyists on GitHub, team websites and The Rocketry Forum.

Generated using Copernicus Climate Change Service information 2026. Contains modified Copernicus Climate Change Service information 2026 (the Earth Data Hub subsets). Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2020), *The ERA5 global reanalysis*, Q. J. R. Meteorol. Soc. 146, 1999–2049, <https://doi.org/10.1002/qj.3803>; ERA5 hourly data on single levels (<https://doi.org/10.24381/cds.adbb2d47>) and on pressure levels (<https://doi.org/10.24381/cds.bd0915c6>), Copernicus Climate Data Store.
