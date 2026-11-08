# The private collection's logs, cut at the boost

This page counts how many of the [private collection's](fixture-flights.md) flights have a barometer's log that a prediction from the boost can start from: the log cut just after burnout, fitted, and flown on to apogee, as the [public flights' report](from-boost.md) does for 7 public flights. It predicts nothing yet, so none of these counts measures accuracy. The collection's license allows only aggregates, so no flight is named.

Written by `cargo xtask fixture-flights`; do not edit. It is the second of four steps toward the collection's predictions (M7.3d5, decision record [ADR-326](../../docs/decisions/0326-m7-3d3-in-four-the-halves-sealed-first.md)).

**The halves.** The predictions' errors will be given for two halves: a held-out share of the flights, kept from every later choice of model ([ADR-209 §4](../../docs/decisions/0209-the-2026-10-08-best-on-the-market-measured.md)), and the rest. The collection's [forward errors](fixture-flights.md#apogee-errors) (hpr's simulated apogee against the logged one) were published before the held-out digest was sealed on 2026-11-07, so the model choices made before then could draw on every flight: the held-out flights are kept only from choices made after it (held out, not blind). A flight is held out when a hash of its id puts it there: the first eight bytes of the SHA-256 of the id followed by `ADR-209`, read as a big-endian number, fall below 2⁶⁴ / 3, a third of the flights in expectation ([ADR-322](../../docs/decisions/0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md)). The held-out ids stay private ([ADR-151](../../docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md)). The SHA-256 of their sorted ids is committed in [`held-out.sha256`](held-out.sha256) before any half's number, so any later change to which flights are held out shows as a change to that hash. This page gives nothing for either half.

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
