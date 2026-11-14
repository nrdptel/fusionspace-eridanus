# ADR-347: The 2026-11-14 review: landing points held to tracked landings, predictions first (2026-11-14)

- **Status:** accepted; promotes [M2.13][m2-13] to the [queue][queue], sixth, after [M2.12][m2-12], in two increments: [M2.13a][m2-13a], the rule, the readings and the predictions merged first, then [M2.13b][m2-13b], the errors.
- **Summary:** `hpr sim` prints how far from the pad a rocket lands, and no report sets that against a real flight. 41 of the private collection's 83 flights are cataloged with a latitude and a longitude column or as a tracker's file (no track has been read yet), and each has the day's winds by height. The milestone reads each track's pad and landing points, merges them and each flight's predicted landing point before any error is computed, then scores the miss. The scoreboard gains a row for the miss and a row for whether the predictions land short. A landing predicted nearer the pad than the real one is the flattering side.

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
[arch]: ../ARCHITECTURE.md
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

The landing place is a number a flyer acts on. `hpr sim` prints a `landing` line with
the distance from the pad, and its machine-readable output gives the landing's east and north
offsets and its latitude and longitude ([the command line's page][cli]). A flyer reads it to
judge whether the rocket comes down inside the field. A landing predicted nearer the pad than
the real one is the flattering side, the worst class of error for a safety number
([ADR-144][adr-144]).

What that line rests on today is another simulator. [The validation report][latest] compares
the drift under the parachutes with RocketPy's on its example rockets, east and north apart.
That shows the two programs integrate the same descent in the same wind. It can't show that
either one lands where a rocket landed. The command line's page says so of its own notes: a
forecast's winds have been compared with no measured ones, and drift has no measured spread yet.

The logs hold the other side, unread. 41 of the collection's 83 flights have a log that the
collection's catalog lists with a latitude and a longitude column or as a tracker's file
(counted 2026-11-14 from the catalog; no track has been read for this count). All 41 have
the day's ERA5 air on file, 38 state the rail's heading, and the manifest marks 15 as
deployed before apogee. [The private collection's report][fixture] already flies 68 of the
83 in their day's air, winds by height included, from the rail the manifest gives, or
1.8 m (6 ft) of rail and due north where it gives none. It flies them with nothing deployed, since it
compares apogees, and keeps only the apogee.

The analyzer can't supply the reading yet. `hpr analyze` takes its readings from a barometer's
altitude or pressure, and its page says: "A GPS track gives none." ([Reading a flight
log][readings].) The library's fit reads a track's positions from liftoff to apogee only, to
fit one wind ([the flight fit][fit]). The fit's code, and the code that writes a flight's
logs out as a trajectory to replay, take the pad's point from the fixes before liftoff, or
from the first fix. Neither takes a landing point as a reading: nothing says where the rocket
came to rest, or how sure that is.

The research notes say how large a miss the winds alone can make. ERA5's winds differ from
weather-balloon winds by 1.4 to 1.9 m/s (3.1 to 4.3 mph) RMS in speed through the lowest 3 km
(9,800 ft). The note turns that into 100 to 400 m (330 to 1,310 ft) of drift at one standard
deviation, from speed alone, on its two worked descents; an error in direction adds to it
([wind read from a descent][wind-descent]). Over the Plains and the West its wind in
the boundary layer runs 3% to 16% slow ([ERA5's wind direction][era5-direction]). If that
holds on a launch day, a predicted drift is short through those heights, the flattering way.
A flat or elliptical main can glide sideways at about 1 to 3 m/s (2 to 7 mph), which HPR Sim
doesn't model ([canopy glide][glide]). So a first number is easy to shade: a rule chosen after the misses
are seen could drop the windy days or the gliding canopies. The rule and both sides of the
comparison come first, as they do for the descent rates.

**Decision.**

1. **M2.13, landing points against the tracked landings, in two increments.** M2.13 is met
   when both are.
   - **M2.13a, the rule, the readings and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no miss.
     - The analyzer reads, from a log's position track, the pad's point and the point where
       the rocket came to rest, each with a 90% interval. It refuses a track with no fix on
       the pad or one that ends in the air. The reading never uses a simulation: it lives in
       `hpr-flightdata`, the crate [the architecture][arch] bars from depending on the
       simulator's.
     - That reading is checked on synthetic tracks whose pad and landing are known, before
       it reads any real track. The rule states, with a source for each, the levels of fix
       noise and of dropout (the share of fixes lost and the longest gap) the set covers.
       The worst level any of those sources gives is one of them. Each level has at least
       50 tracks that should be read and the set at least 200. Each kind of track that
       must be refused, one with no fix on the pad and one ending in the air, has at least
       20.
     - A reading is wrong when the true point lies outside its interval, or when a track
       that should be read is refused. The set must meet three lines, each given with its
       counts.
       - Coverage: over the whole set, the share of the tracks that should be read whose
         intervals hold the true point isn't below 90% by a one-sided exact binomial test
         at the 5% level, and at no level is it under 80%.
       - Width: at every level, the 90th percentile of the half-widths is under a bound
         the rule fixes beforehand from that level's noise and longest gap.
       - Refusal: every track that must be refused is refused.
     - Each line is shown able to fail. A reader that widens every interval tenfold, one
       that refuses nothing, and one that moves every landing point by its level's noise
       each fail the set.
     - The rule names which simulated landing point is set against which reading, and the
       parachutes: the design file's, as [M2.12a][m2-12a] flies them.
     - A prediction reads only its flight's plan, the fields [ADR-333][adr-333] lets a
       prediction read, and the design file's parachutes. Its winds are the day's ERA5
       file's, and no wind or deployment comes from the flight's log or its track.
     - A plan's own fields were read by hand, and [ADR-333][adr-333] says the curation took
       some flights' rail angle or pad elevation from their logs. So each flight's record,
       below, states where
       the plan's launch hour, rail angle, rail heading and pad elevation came from: the
       launch's records, the flight's own log, a value assumed where the plan states none,
       or unknown. The rule names what is assumed. No plan field is changed for this
       comparison.
     - It defines the miss, the distance between the predicted and the tracked landing
       points, and which side is short: a prediction is short when its landing point is
       nearer the pad than the tracked one, by distance from the pad, whatever the
       direction, and beyond otherwise. The parts of a miss along and across the tracked drift are a diagnosis
       only, and the rule fixes the tracked drift below which a flight gets none.
     - It names which flights count as a recovery fault, a flight whose recovery failed,
       such as a parachute that never opened. The test reads the flight's record, such as
       its flyer's notes or an event its log lacks, and never the size of a miss.
     - It names the cause each flight left out is counted under.
     - For each flight of the collection with a track, one record holds its id, its plan,
       the SHA-256 of its design file and of its ERA5 file, the commit that flew it, the
       launch hour, the rail and the pad elevation flown with the source of each, the
       parachutes flown, the
       predicted landing point as east and north offsets from the pad, the same flown in
       calm air, OpenRocket 24.12's landing point where it flies the flight, the track's
       two points with their intervals, whether it is a recovery fault, and its cause if it
       is left out. A record is never written over an older one.
     - Each record's SHA-256, a fingerprint of the record, is merged in a list of its own,
       read as `predictions.sha256` is read for the apogees ([ADR-333][adr-333]). A digest
       of the sealed flights' ids is merged beside the list, as one of the manifest's ids
       is for the apogees. The list holds one hash for each sealed id and no other: the
       run stops on a sealed id whose record's hash isn't on it, and on a hash that no
       record matches.
     - The OpenRocket 24.12 run whose landing points go in the records flies the design
       file's parachutes in the same winds. Today's run sets every parachute never to
       deploy, so its landing points are ballistic and aren't used. That change merges
       here.
     - Each public flight is listed with the landing point its example or its paper
       publishes and that source, or as having none. Those with one are sealed the same way.
   - **M2.13b, the errors.** *Done when:*
     - The fixture report computes each miss from the record [M2.13a][m2-13a] merged. The
       run stops, comparing nothing, when a sealed flight's record is missing, isn't on the
       list read at the merge base as [ADR-333][adr-333]'s is, doesn't match its hash, or
       is of another plan, design file or ERA5 file than the flight compared. A landing
       point from the model as it then stands may be shown beside, marked as not the
       prediction.
     - The report gives, as aggregates: the flights compared; the median and the mean miss,
       in meters and in percent of the logged apogee; how many predictions are short and how
       many beyond; the difference in distance from the pad, predicted less tracked, as its
       mean in meters and as the sum over the flights divided by the sum of their tracked
       distances; the miss along and across the tracked drift; and how many misses the
       track's own intervals hold.
     - It gives the same flights' miss from the sealed calm-air point, so a reader sees
       whether the day's winds made the prediction better than none.
     - It gives OpenRocket 24.12's miss from its sealed points beside HPR Sim's, both over
       the flights both fly, with the rest counted.
     - It gives the same on each half of the collection, the calibration half and the
       held-out half ([ADR-332][adr-332]); without the recovery faults; and without the
       flights whose launch hour, rail or pad elevation came from their own log, was
       assumed, or is of unknown source; each beside the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights with a published landing point get the same aggregates, marked as
       not predicted first.
     - The scoreboard gains `accuracy.landing`, the median miss in percent of the logged
       apogee, lower better. Its row gives the same flights' calm-air median in that unit
       and the median in meters, so a reader sees what no wind at all would score.
     - The scoreboard gains `bias.landing`, the summed difference in distance from the pad
       over the summed tracked distances, in percent, nearer zero better, with short, the
       negative side, named as the flattering one.
     - Both rows read the sealed misses over every compared flight, the recovery faults
       included. HPR Sim's cells are held by the calibration half, as `accuracy.collection`
       is, and OpenRocket's are shown.
     - A test pins which sign means short, on cases that include a prediction on the far
       side of the pad from the tracked landing and one at the pad itself, and a probe that
       flips the sign fails it.
     - The guide to trusting a simulation and the command line's page state the measured
       miss, how many flights it rests on and what it leaves out.
2. **Sixth in the queue,** after [M2.12][m2-12], the descent rates, and before
   [M2.3d4][m2-3d4], a stage's override that adds to its parts. A drift is a wind times a
   time under a parachute, so the landing reads on from the descent rates, and
   [M2.12a][m2-12a]'s predictions will fly the design file's parachutes. Errors against
   logged flights stay ahead of the stage probes, where [ADR-342][adr-342] put the public
   flights' errors.
3. **What the row can't say.** A miss is the simulation's, the wind analysis's and the
   canopy's glide together, and [M2.13b][m2-13b] says so beside the number. The winds are a reanalysis
   made after the day. A flyer at the pad has a forecast, which this row doesn't score. The
   row is a point's miss. Whether a scattered run's landing region holds the tracked landings
   is the backlog's two landing-spread ideas ([flight data][ideas-fd], [pad and
   recovery][ideas-pad]), which can read [M2.13a][m2-13a]'s sealed readings once they exist, and the
   apogee's ranges are [M2.8][m2-8]'s.
4. **Nothing else moves.** No tolerance changes, no check is added by this record, and no
   queued milestone's *done when* is touched.

**Consequences.**

- A real-flight row on a third output, after apogee and the descent rate, is on the roadmap,
  and it is a safety number with a named flattering side. The collection's landing misses will be predicted first:
  each prediction and each reading is fixed in the repository before anyone compares the two,
  with where its plan's launch hour, rail and pad elevation came from beside it.
- The analyzer gains its first readings from a position track, the pad's point and the
  landing point with their intervals, checked against synthetic tracks before they are used
  on a real one.
- The first numbers will carry the wind analysis's error, which the note puts at 100 to 400 m
  (330 to 1,310 ft) of drift at one standard deviation, before any error in direction. The row is honest only with the
  calm-air miss beside it, so [M2.13b][m2-13b]'s *done when* asks for both.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The guide's sentence and the analyzer's are quoted from their pages.
The count of 41 is of manifest flights with a cataloged log that lists a latitude and a
longitude column or is a tracker's file; the 38, the 15 and the ERA5 files are the manifest's
fields for those 41. The scoreboard's row ids were listed from [the committed
scoreboard][score]: none reads a landing point. The fixture report has no landing column
today, the analyzer reads no landing point from a track, and the scoreboard has no
`accuracy.landing`, so both *done when*s fail today.
`cargo test -p xtask` checks that the queue's ids are open in the roadmap.
