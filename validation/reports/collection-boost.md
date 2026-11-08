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

**The predictions.** Each cut flight that the [forward report](fixture-flights.md) compares is flown as it flies it in hpr: its `.ork` and motor configuration, its rail and its day's ERA5 air, with nothing deployed. The fit changes only the motor's impulse and burn time, the drag, the structure's mass and a wind constant with height, centered on ERA5's wind averaged over the climb. Flown with nothing changed in ERA5's own wind, each lands within 0.50 m of the forward simulation's apogee (largest 0.000 m). The log up to the cut is fitted and the fit flown on to apogee, as the [public flights' report](from-boost.md) does. Both errors are against the same number: the logged apogee as a height climbed in the day's air, as the forward report takes it.

| | calibration half | held-out half |
|---|---:|---:|
| flights cut | 14 | 12 |
| … no prediction: hpr's forward simulation doesn't fly it | 2 | 2 |
| … no prediction: too little up to the cut to fit | 0 | 1 |
| predicted | 12 | 9 |
| … with a deployment before apogee | 4 | 6 |
| … cut less than the motor's burn after liftoff | 0 | 0 |
| … cut, the burn or the liftoff not known | 2 | 1 |
| … with another log of the flight on a barometer's track | 6 | 7 |
| … … the logs' apogees more than 2% apart | 0 | 0 |
| **every flight predicted** | **12** | **9** |
| prediction from the boost: mean absolute error | 3.22% | 3.56% |
| … mean error with its sign | -0.05% | +1.50% |
| forward simulation, same flights: mean absolute error | 9.35% | 16.86% |
| … mean error with its sign | +8.87% | +15.83% |
| absolute errors' difference, prediction less forward (points) | -6.13 | -13.29 |
| … paired bootstrap 95% interval | -10.07 to -2.70 | -19.01 to -7.97 |
| … exact sign-flip 95% interval | -10.63 to -2.07 | -20.42 to -6.45 |
| **without a deployment before apogee** | **8** | **3** |
| prediction from the boost: mean absolute error | 3.04% | 3.08% |
| … mean error with its sign | -1.11% | -1.54% |
| forward simulation, same flights: mean absolute error | 10.06% | 10.40% |
| … mean error with its sign | +9.34% | +10.40% |
| absolute errors' difference, prediction less forward (points) | -7.02 | -7.32 |
| … paired bootstrap 95% interval | -12.66 to -2.28 | -10.38 to -3.50 |
| … exact sign-flip 95% interval | -13.76 to -0.79 | unbounded |

A difference below zero means the prediction came closer. Both methods fly nothing deployed, and the fit reads the log only up to its cut, so a deployment before apogee lowers the logged apogee beneath both; the rows without those flights leave them out, as the forward report's own set does. Where a flight has other logs on a barometer's track, the rule above takes the one nearest the logged apogee; two rows count how often there was such a choice, and how often the candidates' apogees lay more than 2% apart, where the choice could move an error. Interval ends are rounded outward. The bootstrap resamples each half's flights 10,000 times (seed 329), each flight's two errors kept together, and takes the 2.5% and 97.5% points of the resampled means. On a few flights it can cover less than 95%, so the exact sign-flip interval, which the public flights' report uses, stands beside it: it assumes only that each flight's difference is drawn independently and symmetrically about the center.

**The calibration half's fits.** 12 fits of the logs up to the cut, 12 of them converged; their residuals' RMS averages 2.72 m. Over those fits:

| | mean | standard deviation |
|---|---:|---:|
| drag scale | 1.018 | 0.048 |
| impulse scale | 0.957 | 0.033 |
| structure mass scale | 1.004 | 0.002 |

The held-out half's fit aggregates, the drag and impulse scales a later correction of the shared bias would aim at, stay private until that correction is chosen on the calibration half alone; the SHA-256 of their file is committed in [`held-out-fits.sha256`](held-out-fits.sha256), so a change to them shows.
