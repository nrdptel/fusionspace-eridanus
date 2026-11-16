# ADR-352: The 2026-11-16 review: speed off the rail held to accelerometer logs, predictions first (2026-11-16)

- **Status:** accepted; promotes [M2.14][m2-14] to the [queue][queue], fifth, after [M2.13][m2-13], in two increments: [M2.14a][m2-14a], the rule, the readings and the predictions merged first, then [M2.14b][m2-14b], the errors.
- **Summary:** `hpr sim` prints how fast a rocket leaves its rail, and no report sets that speed against a real flight: it is held to another simulator flown on the same inputs. 48 of the private collection's 83 flights are cataloged with an acceleration channel. The milestone reads, from each such log, the rocket's speed after the same travel along the rail, by summing the logged acceleration from rest on the pad. It merges those readings and each flight's predicted speed before any error is computed, then scores the error. The scoreboard gains a row for the error and a row for whether the predictions run fast. A speed predicted higher than the flight's is the flattering side.

[m2-14]: ../decisions-and-roadmap.md#m2-14
[m2-14a]: ../decisions-and-roadmap.md#m2-14a
[m2-14b]: ../decisions-and-roadmap.md#m2-14b
[m2-13]: ../decisions-and-roadmap.md#m2-13
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m1-14b5]: ../decisions-and-roadmap.md#m1-14b5
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[trust]: ../trusting-a-simulation.md
[arch]: ../ARCHITECTURE.md
[cli]: ../cli.md
[metrics]: ../physics/metrics.md
[accuracy]: ../accuracy.md
[readings]: ../reading-a-flight-log.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[readings-rs]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-flightdata/src/readings.rs
[prfl]: https://github.com/dustintlee-blahblah/PocketRocketFlightLab/blob/main/VALIDATION.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-310]: 0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-338]: 0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-342]: 0342-the-errors-before-the-remaining-openrocket-parity-probes.md
[adr-347]: 0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md

**Context.** This review raises one bar on accuracy against real flights, as each review does
while the project has no outside users ([ADR-322][adr-322]). The last two queued the descent
rate under each parachute ([ADR-338][adr-338]) and the landing point ([ADR-347][adr-347]).
[The guide to trusting a simulation][trust] says the reports "measure apogee alone: no
comparison here checks a top speed, a descent or a landing place against a real flight". The
speed off the rail isn't on that list either.

It is a number a flyer and a range safety officer act on. `hpr sim` prints a `rail exit speed`
line ([the command line's page][cli]): the speed of the center of mass over the ground when the
aft-most rail button or lug passes the rail's top ([flight metrics][metrics]). A rocket that
leaves the rail too slowly isn't yet held straight by its fins. A speed predicted higher than
the real one is the flattering side, the worst class of error for a safety number
([ADR-144][adr-144]).

What that line rests on today is another simulator. The comparisons with RocketPy flown on the
same inputs hold the rail-exit speeds within 0.05% ([accuracy][accuracy],
[ADR-310][adr-310]). That shows two programs integrate the same thrust curve and the same mass
alike. It can't show that a real motor comes up to thrust as its file says, or what a real rail
takes from the rocket: by default `hpr sim`'s rail "has no friction" ([the command line's
page][cli]).

The logs hold the other side, unread. 48 of the collection's 83 flights have a log that the
collection's catalog lists with an acceleration channel, 43 of them sampled at 100 Hz or faster
(counted 2026-11-16 from the catalog's column names; no log was opened for this count). 46 of
the 48 plans state a rail length. For 32 of those 46, the manifest's note on where the rail
came from names the design file's simulation setting, not a record of the pad. So the
comparison can't lean on the real rail's length. It doesn't need to: the prediction and the
reading can be taken after the same travel, and on a rail the rocket moves along one line.

The analyzer can't supply the reading yet. `hpr analyze` reads an accelerometer along the
rocket's axis ([reading a flight log][readings]), and it has no reading of a speed after a
stated travel.

A reading of this kind is coarse, and the arithmetic says where. At a steady 10 g (98 m/s², 322
ft/s²) a rocket covers 1.8 m (6 ft) of rail in 0.19 s and leaves it at 18.8 m/s (62 ft/s). A
logger at 100 Hz takes 19 samples in that time. A speed taken after a fixed travel forgives a
late start: a sum started one sample late, 0.01 s, has missed 5 mm (0.2 in) of the 1.8 m, and
at steady acceleration it reads the same speed after its own 1.8 m. It doesn't forgive the
sensor's scale. The analyzer takes an accelerometer's scale as known to 10%
([`ACCEL_SCALE_SHARE`][readings-rs]); after a fixed travel the speed goes as the square root of
the acceleration, so that alone is 5% of the speed. Another simulator's notes say one logger
family's accelerometer "read 12–21% low against a second recorder on the same flights", and
that it keeps only flights where "the height the accelerometer makes of the climb is within 8%
of the barometer's" ([its validation page][prfl], read 2026-11-16). So a first number is easy
to shade: by which loggers are dropped, by which rates count as fine enough, or by how a
sensor's scale is checked. The rule and both sides of the comparison come first, as they do for
the descent rates and the landing points.

**Decision.**

1. **M2.14, speed off the rail against the accelerometer logs, in two increments.** M2.14 is
   met when both are.
   - **M2.14a, the rule, the readings and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no error.
     - The analyzer reads, from a log's acceleration channel along the rocket's axis, the
       rocket's speed after a stated travel from rest, with a 90% interval. The channel's zero
       is its own mean over the samples on the pad before the rocket moves, so the reading
       needs no rail angle. The reading never uses a simulation: it lives in `hpr-flightdata`,
       the crate [the architecture][arch] bars from depending on the simulator's.
     - It refuses a log that starts after the rocket moved or has fewer pad samples than the
       rule fixes, a channel whose axis or whose handling of gravity the analyzer can't
       settle, a channel that reaches its sensor's limit during the travel, and a log with
       fewer samples over the travel than the rule fixes.
     - That reading is checked on synthetic boosts whose speed after the travel is known,
       before it reads any real log. The rule states, with a source for each, the levels the
       set covers: of sample rate, of pad time before the motion, of noise, of zero offset and
       of scale error. The worst level any of those sources gives is one of them. Each level
       has at least 50 boosts that should be read and the set at least 200. Each kind of log
       that must be refused, the four above, has at least 20.
     - A reading is wrong when the true speed lies outside its interval, or when a log that
       should be read is refused. The set must meet the three lines [ADR-347][adr-347] sets
       for the landing readings, each given with its counts. Coverage: over the whole set, the
       share of the logs that should be read whose intervals hold the true speed isn't below
       90% by a one-sided exact binomial test at the 5% level, and at no level is it under
       80%. Width: at every level, the 90th percentile of the half-widths is under a bound the
       rule fixes beforehand from that level's rate, noise and scale error. Refusal: every log
       that must be refused is refused.
     - Each line is shown able to fail. A reader that widens every interval tenfold, one that
       refuses nothing, and one that leaves the sensor's scale out of its interval each fail
       the set.
     - The rule fixes, with its source, how a log's accelerometer is checked against the same
       log's barometer before its reading counts, and how far the two may disagree. The check
       reads no simulation and no error. A flight that fails it is left out under that cause.
     - The rule names the travel: the distance the predicted rocket moves from rest to its
       rail exit, from the plan's rail length and the design file's guides. It names which
       simulated speed is set against the reading: the airframe's after that travel, which an
       accelerometer fixed to it reads. `hpr sim` prints the center of mass's, which runs a
       little higher as the propellant burns ([flight metrics][metrics]); the record holds
       both.
     - Where a flight has more than one log with a usable channel, the rule names which one
       gives the flight's reading, by the logs' own properties and never by a reading's value.
     - A prediction reads only its flight's plan, the fields [ADR-333][adr-333] lets a
       prediction read, and the design file. Its air is the day's ERA5 file's. No thrust, mass
       or time of first motion comes from the flight's log.
     - Each flight's record, below, states where the plan's rail length and rail angle came
       from: the launch's records, the flight's own log, the design file's simulation setting,
       a value assumed where the plan states none, or unknown. The rule names what is assumed.
       No plan field is changed for this comparison.
     - It defines the error, the predicted speed less the read one over the read one, and
       which side is fast: a prediction is fast when it is higher than the reading.
     - It names the cause each flight left out is counted under.
     - For each flight of the collection with an acceleration channel, one record holds its
       id, its plan, the SHA-256 of its design file, of its motor's file and of its ERA5
       file, the commit that flew it, the rail flown with the source of each field, the
       travel, the predicted airframe speed after it and the printed rail exit speed, the
       speed after the same travel from the motor's average thrust and the liftoff mass alone,
       by a formula the rule gives,
       OpenRocket 24.12's speed after the same travel where it flies the flight, the reading
       with its interval and its count of samples over the travel, the barometer check's
       figure, and its cause if it is left out. A record is never written over an older one.
     - Each record's SHA-256, a fingerprint of the record, is merged in a list of its own,
       read as `predictions.sha256` is read for the apogees ([ADR-333][adr-333]). A digest of
       the sealed flights' ids is merged beside the list. The list holds one hash for each
       sealed id and no other: the run stops on a sealed id whose record's hash isn't on it,
       and on a hash that no record matches.
     - OpenRocket 24.12's speed is read from its own flight's history at the same travel, not
       from its launch-rod figure, whose travel may differ. The run that stores that history
       merges here.
     - Each public flight is listed as having a log with an acceleration channel the reading
       can use, or none. Those with one are sealed the same way.
   - **M2.14b, the errors.** *Done when:*
     - The fixture report computes each error from the record [M2.14a][m2-14a] merged. The
       run stops, comparing nothing, when a sealed flight's record is missing, isn't on the
       list read at the merge base as [ADR-333][adr-333]'s is, doesn't match its hash, or is
       of another plan, design file, motor file or ERA5 file than the flight compared. A speed
       from the model as it then stands may be shown beside, marked as not the prediction.
     - The report gives, as aggregates: the flights compared; the mean absolute and the mean
       signed error, in percent of the read speed; how many predictions are fast and how many
       slow; the median half-width of the readings' intervals; and how many errors the
       reading's own interval holds.
     - It gives the same flights' error from the sealed average-thrust speed, so a reader sees
       whether the simulation beats the estimate a pocket calculator makes.
     - It gives OpenRocket 24.12's error from its sealed speeds beside HPR Sim's, both over
       the flights both fly, with the rest counted.
     - It gives the same on each half of the collection, the calibration half and the
       held-out half ([ADR-332][adr-332]), and on the flights whose rail length is a record of
       the pad, each beside the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights with a usable channel get the same aggregates, marked as not
       predicted first.
     - The scoreboard gains `accuracy.rail-exit`, the mean absolute error in percent of the
       read speed, lower better. Its row gives the same flights' average-thrust figure and the
       median half-width of the readings' intervals, so a reader sees what the readings can
       resolve.
     - The scoreboard gains `bias.rail-exit`, the mean signed error in percent, nearer zero
       better, with fast, the positive side, named as the flattering one.
     - Both rows read the sealed errors over every compared flight. HPR Sim's cells are held
       by the calibration half, as `accuracy.collection` is, and OpenRocket's are shown.
     - A test pins which sign means fast, on cases that include a prediction equal to its
       reading, and a probe that flips the sign fails it.
     - The guide to trusting a simulation and the command line's page state the measured
       error, how many flights it rests on, how wide the readings' intervals are and what the
       figure leaves out.
2. **Fifth in the queue,** after [M2.13][m2-13], the landing points, and before
   [M2.3d4][m2-3d4], a stage's override that adds to its parts. It follows the two outputs
   already queued, so their order stands. Errors against logged flights stay ahead of the
   stage probes, where [ADR-342][adr-342] put the public flights' errors.
3. **What the row can't say.** The speed is the rocket's along the rail after the plan's
   travel. It is the real rail's exit speed only on the flights whose rail length is a record
   of the pad, which [M2.14b][m2-14b] gives apart. It says nothing of the wind at the pad,
   of the push a rocket gets as it leaves the rail ([M1.14b5][m1-14b5]'s), or of whether a
   speed is enough: that is the range safety officer's call. A reading is an accelerometer's,
   several percent wide, so [M2.14b][m2-14b] says beside the error how many errors the
   readings' own intervals hold.
4. **Nothing else moves.** No tolerance changes, no check is added by this record, and no
   queued milestone's *done when* is touched.

**Consequences.**

- A real-flight row on a fourth output, after apogee, the descent rate and the landing point,
  is on the roadmap. It is a safety number with a named flattering side, and the first row on
  the boost itself, where the thrust curve and the liftoff mass decide the number.
- The analyzer gains its first reading from summed acceleration, a speed after a stated
  travel with its interval, checked against synthetic boosts before it is used on a real log.
- The first numbers will be coarse: the arithmetic above puts a reading's interval at several
  percent. The row is honest only with the readings' width and the average-thrust figure
  beside it, so [M2.14b][m2-14b]'s *done when* asks for both.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The guide's sentence, the command line's two and the other simulator's
two are quoted from their pages. The count of 48 is of manifest flights with a cataloged log
that lists an acceleration column, and the 43 of those with a cataloged rate of 100 Hz or more;
the 46 and the 32 are the manifest's rail length and its note of the rail's source for those
48. The arithmetic is, at a steady acceleration `a` over a travel `L`, the time `sqrt(2 L / a)`
and the speed `sqrt(2 a L)`, and for a start late by `dt` the travel missed, `a dt² / 2`, with
the acceleration, the travel and the sample time the context gives. The scoreboard's row ids
were listed from [the committed scoreboard][score]: none reads a speed off the rail. The
fixture report has no such column today, the analyzer reads no speed after a travel, and the
scoreboard has no `accuracy.rail-exit`, so both *done when*s fail today. `cargo test -p xtask`
checks that the queue's ids are open in the roadmap.
