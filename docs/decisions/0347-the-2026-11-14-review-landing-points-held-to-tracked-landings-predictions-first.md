# ADR-347: The 2026-11-14 review: landing points held to tracked landings, predictions first (2026-11-14)

- **Status:** accepted; promotes [M2.13][m2-13] to the [queue][queue], sixth, after [M2.12][m2-12], in two increments: [M2.13a][m2-13a], the rule, the readings and the predictions merged first, then [M2.13b][m2-13b], the errors.
- **Summary:** `hpr sim` prints how far from the pad a rocket lands, and no report sets that against a real flight. 41 of the private collection's 83 flights have a log with a position track, and each has the day's winds by height. The milestone reads each track's pad and landing points, merges them and each flight's predicted landing point before any error is computed, then scores the miss. The scoreboard gains a row for the miss and a row for its sign along the drift. A landing predicted nearer the pad than the real one is the flattering side.

[m2-13]: ../decisions-and-roadmap.md#m2-13
[m2-13a]: ../decisions-and-roadmap.md#m2-13a
[m2-13b]: ../decisions-and-roadmap.md#m2-13b
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-12a]: ../decisions-and-roadmap.md#m2-12a
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-8]: ../decisions-and-roadmap.md#m2-8
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[ideas-fd]: ../research/ideas-flight-data.md
[ideas-pad]: ../research/ideas-pad-and-recovery.md
[trust]: ../trusting-a-simulation.md
[cli]: ../cli.md
[readings]: ../reading-a-flight-log.md
[fit]: ../physics/flight-fit.md
[wind-descent]: ../research/wind-from-descent.md
[era5-direction]: ../research/era5-wind-direction-us.md
[glide]: ../research/hobby-canopy-glide-and-swing.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[latest]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-338]: 0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-342]: 0342-the-errors-before-the-remaining-openrocket-parity-probes.md

**Context.** This review raises one bar on accuracy against real flights, as each review does
while the project has no outside users ([ADR-322][adr-322]). The last one queued the first
real-flight row on an output other than apogee, the descent rate under each parachute
([ADR-338][adr-338]). [The guide to trusting a simulation][trust] still says what is left: the
reports "measure apogee alone: no comparison here checks a top speed, a descent or a landing
place against a real flight".

The landing place is the third number a flyer acts on. `hpr sim` prints a `landing` line with
the distance from the pad, and its machine-readable output gives the landing's east and north
offsets and its latitude and longitude ([the command line's page][cli]). A flyer reads it to
judge whether the rocket comes down inside the field. A landing predicted nearer the pad than
the real one is the flattering side, the worst class of error for a safety number
([ADR-144][adr-144]).

What that line rests on today is another simulator. [The validation report][latest] holds the
drift under the parachutes to RocketPy's on its example rockets, to its east and north parts.
That shows the two programs integrate the same descent in the same wind. It can't show that
either one lands where a rocket landed. The command line's page says so of its own notes: a
forecast's winds have been compared with no measured ones, and drift has no measured spread yet.

The logs hold the other side, unread. 41 of the collection's 83 flights have a log that the
collection's catalog lists with a latitude and a longitude column or as a tracker's file
(counted 2026-11-14 from the catalog's column names; no track has been read for this count).
All 41 have the day's ERA5 air on file, 38 state the rail's heading, and 15 carry a note of
a deployment earlier than planned. [The private collection's report][fixture] already flies
every flight in that air, winds by height included, from the rail its manifest gives. It
flies them with nothing deployed, since it compares apogees, and keeps only the apogee.

The analyzer can't supply the reading yet. `hpr analyze` takes its readings from a barometer's
altitude or pressure, and its page says: "A GPS track gives none." ([Reading a flight
log][readings].) The library's fit reads a track's positions from liftoff to apogee only, to
fit one wind ([the flight fit][fit]). Nothing reads where a track starts and where it ends.

The research notes say how large the miss may be before any model is at fault. ERA5 misses
weather-balloon winds by 1.4 to 1.9 m/s (3.1 to 4.3 mph) in speed through the lowest 3 km
(9,800 ft), which is 100 to 400 m (330 to 1,310 ft) of drift on that note's two worked
descents ([wind read from a descent][wind-descent]). Over the Plains and the West its wind at
100 m (330 ft) runs 12% slow on average, so a predicted drift there would be short in the
lowest few hundred meters, the flattering way ([ERA5's wind direction][era5-direction]). A flat or
elliptical main can glide sideways at about 1 to 3 m/s (2 to 7 mph), which no simulator models
([canopy glide][glide]). So a first number is easy to shade: a rule chosen after the misses
are seen could drop the windy days or the gliding canopies. The rule and both sides of the
comparison come first, as they do for the descent rates.

**Decision.**

1. **M2.13, landing points against the tracked landings, in two increments.** M2.13 is met
   when both are.
   - **M2.13a, the rule, the readings and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no miss.
     - The analyzer reads, from a log's position track, the pad's point and the point where
       the rocket came to rest, each with an interval. It refuses a track with no fix on the
       pad or one that ends in the air. The reading never uses a simulation.
     - That reading is run on synthetic tracks whose pad and landing are known, with the
       fix noise and the dropouts the logger notes give. The share it reads wrong or refuses
       is reported.
     - The rule names which simulated landing point is set against which reading, the winds
       the prediction flies in, and the parachutes: the design file's, as [M2.12a][m2-12a]
       flies them.
     - It names which flights count as a recovery fault. The test reads the flight's record,
       such as its flyer's notes or an event its log lacks, and never the size of a miss.
     - It names the cause each flight left out is counted under.
     - For each flight of the collection with a track, the SHA-256 of its predicted landing
       point, as east and north offsets from the pad, and of its track's two points with
       their intervals, is merged in a list of its own, read as `predictions.sha256` is read
       for the apogees ([ADR-333][adr-333]).
     - Each public flight is listed with the landing point its example or its paper
       publishes and that source, or as having none. Those with one are sealed the same way.
   - **M2.13b, the errors.** *Done when:*
     - The fixture report computes each miss, the distance between the predicted and the
       tracked landing points, from the prediction and the reading merged by M2.13a. A
       flight is refused when either is missing, has no hash, or doesn't match its hash, the
       list read at the merge base as [ADR-333][adr-333]'s is.
     - The report gives, as aggregates: the flights compared; the median and the mean miss,
       in meters and in percent of the logged apogee; the miss along the predicted drift's
       direction with its sign, in percent of the tracked drift, and how many predictions
       fall short of the tracked landing and how many beyond it; the miss across that
       direction; and how many misses the track's own intervals hold.
     - It gives the same flights' miss with the prediction flown in calm air, which is the
       tracked drift itself, so a reader sees whether the day's winds made the prediction
       better than none.
     - It gives OpenRocket 24.12's miss beside HPR Sim's where its run gives a landing
       point.
     - It gives the same on each half of the collection, the calibration half and the
       held-out half ([ADR-332][adr-332]), and the same without the recovery faults, beside
       the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The scoreboard gains `accuracy.landing`, the median miss in percent of the logged
       apogee, lower better, and `bias.landing`, the mean signed miss along the drift in
       percent of the tracked drift, nearer zero better, with short named as the flattering
       side. Both read every compared flight, the recovery faults included. HPR Sim's cells
       are held by the calibration half, as `accuracy.collection` is, and OpenRocket's are
       shown.
     - A test pins which sign means short, and a probe that flips the sign fails it.
     - The guide to trusting a simulation and the command line's page state the measured
       miss, how many flights it rests on and what it leaves out.
2. **Sixth in the queue,** after [M2.12][m2-12], the descent rates, and before
   [M2.3d4][m2-3d4], a stage's override that adds to its parts. A drift is a wind times a
   time under a parachute, so the landing reads on from the descent rates, and M2.12a's
   predictions already fly the design file's parachutes. Errors against logged flights stay
   ahead of the stage probes, as [ADR-342][adr-342] ordered them.
3. **What the row can't say.** A miss is the simulation's, the wind analysis's and the
   canopy's glide together, and M2.13b says so beside the number. The winds are a reanalysis
   made after the day. A flyer at the pad has a forecast, which this row doesn't score. The
   row is a point's miss. Whether a scattered run's landing region holds the tracked landings
   is the backlog's two landing-spread ideas ([flight data][ideas-fd], [pad and
   recovery][ideas-pad]), which can read M2.13a's sealed readings once they exist, and the
   apogee's ranges are [M2.8][m2-8]'s.
4. **Nothing else moves.** No tolerance changes, no check is added by this record, and no
   queued milestone's *done when* is touched.

**Consequences.**

- The second real-flight row on an output other than apogee is on the roadmap, and it is a
  safety number with a named flattering side. The collection's landing misses will be blind:
  each prediction and each reading is fixed in the repository before anyone compares the two.
- The analyzer gains its first reading from a position track, checked against synthetic tracks
  before it is used on a real one.
- The first numbers will carry the wind analysis's error, which the notes put at hundreds of
  meters on an ordinary day. The row is honest only with the calm-air miss beside it, so
  M2.13b's *done when* asks for both.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The guide's sentence and the analyzer's are quoted from their pages.
The count of 41 is of manifest flights with a catalogued log that lists a latitude and a
longitude column or is a tracker's file; the 38, the 15 and the ERA5 files are the manifest's
fields for those 41. The scoreboard's row ids were listed from the committed scoreboard: none
reads a landing point. The fixture report has no landing column today, the analyzer no reading
from a track, and the scoreboard no `accuracy.landing`, so both *done when*s fail today.
`cargo test -p xtask` checks that the queue's ids are open in the roadmap.
