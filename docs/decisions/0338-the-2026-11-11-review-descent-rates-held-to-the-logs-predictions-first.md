# ADR-338: The 2026-11-11 review: descent rates held to the logs, predictions first (2026-11-11)

- **Status:** accepted; promotes [M2.12][m2-12] to the [queue][queue], seventh, after [M2.3c2][m2-3c2], from the [ideas backlog][ideas-fd]'s descent rates against the log, in two increments: [M2.12a][m2-12a], the rule and the predictions merged first, then [M2.12b][m2-12b], the errors.
- **Summary:** Every scoreboard row that reads a logged flight reads its apogee. A rocket's descent rate under its parachutes sets its landing speed and its drift, and HPR Sim's is held to other simulators and to drop tests, not to flights. The analyzer reads each canopy's rate from a log. On the five public flights that have both, the simulated rate under the main runs from 7.59 m/s (24.9 ft/s) slower than the log's to 1.07 m/s (3.5 ft/s) faster. The milestone scores the private collection's flights likewise, after the comparison's rule and each flight's predicted rates are merged, so each prediction is fixed before anyone sees its error. The scoreboard gains rows for the rate's error and its sign.

[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-12a]: ../decisions-and-roadmap.md#m2-12a
[m2-12b]: ../decisions-and-roadmap.md#m2-12b
[m2-3c2]: ../decisions-and-roadmap.md#m2-3c2
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[m7-2]: ../decisions-and-roadmap.md#m7-2
[m7-3a]: ../decisions-and-roadmap.md#m7-3a
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[ideas-fd]: ../research/ideas-flight-data.md
[recovery]: ../physics/recovery.md
[readings]: ../reading-a-flight-log.md
[cli]: ../cli.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[residuals]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[i1052]: https://github.com/nrdptel/fusionspace-eridanus/issues/1052
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-339]: 0339-m0-17-the-docs-site-at-the-tool-level-queued-second.md

**Context.** This review raises one bar on accuracy against real flights, as each review does
while the project has no outside users ([ADR-322][adr-322]). The last one queued five more
public flights, to take the public apogee set from 7 flights to 12, each prediction merged
before its error ([ADR-328][adr-328]). Apogee is still the only thing [the scoreboard][score]
measures against a log: its accuracy, bias and uncertainty rows all read the logged apogee.

A flyer acts on a second number. `hpr sim` prints a `descent` line: the vertical speed under
each set of open parachutes, when the next one opens and at landing ([the command line's
page][cli]). It sizes a main, a drift budget and a landing's energy. A simulated rate slower
than the flight's is too gentle: the flyer expects a softer landing than the one that happens.
That is the flattering side, the worst class of error for a safety number
([ADR-144][adr-144]).

[The recovery page][recovery] says what that line rests on: RocketPy's descents for five
rockets, OpenRocket's landing speeds on its example flights, and drop tests of tumbling and
streamers. The page names no comparison with a real flight, and no report gives the rate's
error over flights as one number.

The logs hold the other side. The analyzer reads each canopy's descent rate from a log, with a
90% interval ([M7.2][m7-2], [reading a flight log][readings]). [The residuals
report][residuals] ([M7.3a][m7-3a]) already sets the two side by side on the seven public
flights, flight by flight, each flown under the parachutes its example gives it. Five have a
logged and a simulated rate under the main:

| Flight | Logged, m/s | Its 90% interval | Simulated, m/s | Logged less simulated |
|---|---:|---|---:|---:|
| Bella Lui | 19.16 | 18.20 to 20.10 | 18.65 | +0.51 |
| NDRT 2020 | 3.62 | 3.09 to 4.14 | 4.69 | −1.07 |
| Prometheus | 6.50 | 6.17 to 6.84 | 7.15 | −0.64 |
| Genesis | 19.18 | 18.35 to 20.00 | 11.60 | +7.59 |
| Lince | 8.59 | 8.21 to 8.96 | 7.10 | +1.49 |

On three of the five the simulated rate is the slower one, and on one the reading's interval
holds it. Two flights also have both rates under the drogue. The report gives no average and
no count by sign, and [the private collection's report][fixture], the 60 flights the
scoreboard's main rows rest on, compares no descent at all.

Two things make a first number easy to get wrong. A log at 19 m/s (62 ft/s) under a main may
be a canopy that didn't open, which is a fault of the flight, not of the model. The report
can't say whether Genesis's is one: its log gives no rate under the drogue and no height for
the main's opening. And a barometer reads a descent slow on a hot day, as it reads a height
low. A rule chosen after the errors are seen could drop the flights that read worst. So the
rule comes first, and each side of the comparison is fixed before the errors are read.

**Decision.**

1. **M2.12, descent rates against the logs, in two increments.** M2.12 is met when both are.
   - **M2.12a, the rule and the predictions.** *Done when:* a pull request of its own merges
     the rule of the comparison and both of its sides, and computes no error.
     - The rule names which simulated rate is set against which reading, for the main and for
       the drogue.
     - It names the air both rates are read in, so that a hot day moves both sides alike.
     - It names which flights count as a recovery fault. The test reads the flight's record,
       such as its flyer's notes or an event its log lacks, and never the size of a rate or of
       an error.
     - It names the cause each flight left out is counted under.
     - For each flight of the collection, the SHA-256 of its predicted rates and of its log's
       readings, each rate with its interval, is merged in a list of its own, read as
       `predictions.sha256` is read for the apogees ([ADR-333][adr-333]). The predictions are
       flown with the design file's parachutes, which the apogee predictions leave closed.
   - **M2.12b, the errors.** *Done when:*
     - The fixture report computes each error from the prediction and the readings merged by
       M2.12a. A flight is refused when either is missing, has no hash, or doesn't match its
       hash, the list read at the merge base as [ADR-333][adr-333]'s is. A rate from the model
       as it then stands may be shown beside, marked as not the prediction.
     - The report gives, as aggregates, for the main and for the drogue: the flights compared;
       the mean absolute and the mean signed error of HPR Sim's rate against the log's, in
       percent of the log's; OpenRocket 24.12's beside them where its run gives a rate; how
       many simulated rates are slower than the log's (too gentle) and how many faster; and
       how many the reading's 90% interval holds.
     - It gives the same on each half of the collection, the calibration half and the held-out
       half ([ADR-332][adr-332]), and the same without the recovery faults, beside the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights' rows of the residuals report get the same aggregates, marked as not
       blind.
     - The scoreboard gains `accuracy.descent`, lower better, and `bias.descent`, nearer zero
       better, with too gentle named as the flattering side. Both read the rate under the
       main over every compared flight, the recovery faults included. HPR Sim's cells are held
       by the calibration half, as `accuracy.collection` is, and OpenRocket's are shown.
     - A test pins which sign means too gentle, and a probe that flips the sign fails it.
     - The recovery page states the measured error, how many flights it rests on and what it
       leaves out.
2. **Seventh in the queue,** after [M2.3c2][m2-3c2], the logged traces, and before
   [M2.3c9][m2-3c9], every flight OpenRocket flies. M2.3c2 brings each flight's logged trace
   into the fixture report up to apogee, and M2.12 reads on from there. The items ahead of it
   keep [ADR-322][adr-322]'s order, with [ADR-328][adr-328]'s twelve public flights and
   [ADR-339][adr-339]'s docs site inserted.
3. **What the row can't say.** A reading is an estimate too. Its 90% intervals are checked
   on synthetic flights and not yet on real ones ([#1052][i1052]). An error in the row
   is the simulation's and the reading's together, and M2.12b says so beside the number. A
   design file's parachute is the flyer's entry: its drag area is often a maker's figure or a
   default. The row measures the prediction a flyer gets from the file as saved, which is the
   number they act on.
4. **Nothing else moves.** No tolerance changes, no check is added by this record, and no
   queued milestone's *done when* is touched.

**Consequences.**

- The first real-flight row on an output other than apogee is on the roadmap, and it is a
  safety number. The collection's descent errors will be blind: each prediction and each
  reading is fixed in the repository before anyone compares the two. Nothing in a committed
  report compares them today.
- The public flights' descent residuals are already published, so their aggregates are marked
  as not blind. A reader can re-run those.
- The recovery page's line that no descent has been compared with a real flight predates the
  residuals report. This record's pull request corrects it to name the five flights, and
  M2.12b replaces it with the measured error.
- The backlog's entry is marked promoted. The review's new ideas are in
  [the backlog][ideas], under its date.

**How it was checked.** The table's figures are the committed residuals report's, row for row.
The scoreboard's row ids were listed from the committed scoreboard: each that reads a log is an
accuracy, bias or uncertainty row on apogee. The fixture report has no descent column today,
and the scoreboard no `accuracy.descent`, so both *done when*s fail today. `cargo test -p
xtask` checks that the queue's ids are open in the roadmap.
