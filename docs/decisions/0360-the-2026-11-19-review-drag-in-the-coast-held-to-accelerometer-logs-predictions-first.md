# ADR-360: The 2026-11-19 review: drag in the coast held to accelerometer logs, predictions first (2026-11-19)

- **Status:** accepted; promotes [M2.15][m2-15] to the [queue][queue], sixth, after [M2.14][m2-14] (the speed off the rail against the accelerometer logs), in two increments: [M2.15a][m2-15a], the rule, the readings and the predictions merged first, then [M2.15b][m2-15b], the errors, for HPR Sim and OpenRocket 24.12 both.
- **Summary:** Both simulators' apogees run high on the private collection's logged flights, by +7.72% and +6.72% on average, and the apogee alone can't say why: too little drag and too much thrust read the same way. Coasting, an accelerometer along the rocket's axis reads the air's force along that axis over the mass, with no thrust and no gravity in the reading. The milestone reads that below Mach 0.8 from each log with an acceleration column, merges the readings and both programs' predictions before any error is computed, and then scores both programs on new rows, `accuracy.coast-drag` and `bias.coast-drag`.

[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
[m2-15b]: ../decisions-and-roadmap.md#m2-15b
[m2-14]: ../decisions-and-roadmap.md#m2-14
[m2-12a]: ../decisions-and-roadmap.md#m2-12a
[m2-13a]: ../decisions-and-roadmap.md#m2-13a
[m2-14a]: ../decisions-and-roadmap.md#m2-14a
[m2-14b]: ../decisions-and-roadmap.md#m2-14b
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m1-14i]: ../decisions-and-roadmap.md#m1-14i
[m1-14i2]: ../decisions-and-roadmap.md#m1-14i2
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[trust]: ../trusting-a-simulation.md
[flatters]: ../trusting-a-simulation.md#which-miss-flatters
[arch]: ../ARCHITECTURE.md
[era5]: ../format/era5.md
[readings]: ../physics/log-readings.md
[specific-force]: ../glossary.md#specific-force
[aoa]: ../glossary.md#angle-of-attack
[normal-force]: ../glossary.md#normal-force
[ref-area]: ../glossary.md#reference-area
[accuracy]: ../accuracy.md
[collection]: ../accuracy.md#real-flights-of-the-private-collection
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-338]: 0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-347]: 0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[adr-348]: 0348-m7-5b-the-correction-fails-on-the-public-flights.md
[adr-352]: 0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[adr-359]: 0359-drag-below-mach-0-8-next-the-drag-part-of-the-gap-to-openrocket-by-mach-band.md

**Context.** This review raises one bar on accuracy against real flights, as each review does
while the project has no outside users ([ADR-322][adr-322]). The last three queued a descent
rate, a landing point and a speed off the rail against the logs. This one goes after the
apogee's own error.

On the [private collection][collection]'s 60 logged flights, HPR Sim's apogee is +7.72% high on
average and OpenRocket 24.12's +6.72% ([the scoreboard][score]). Two records looked for the cause
and neither could settle it. [ADR-348][adr-348] fitted one correction of thrust and drag on the
collection's calibration half, the half models may be tuned on: it came out as an impulse scale
of 0.94 with the drag left at 1, and it failed on the public flights. [ADR-359][adr-359] split
the gap between the two programs, not the gap to the logs, by the Mach number the drag acts at,
and found its largest part below Mach 0.8. Its decision 5 says what that split can't settle:
"Both codes climb above the logs on average, +7.72% and +6.72%, so more drag from any cause
moves the mean toward them, and a wrong mass or thrust would read the same way."

An accelerometer separates them. It doesn't feel gravity: it reads every other force on the
rocket over its mass, the [specific force][specific-force]. Coasting, with the motor out, the
part along the rocket's axis is the air's axial force over the mass ([what an accelerometer
reads][readings]), whatever the thrust was. The axial force is the drag when the rocket flies
straight into the wind. At an [angle of attack][aoa] it is the drag times the angle's cosine less
the lift times its sine, a share that grows with the angle through the
[normal force][normal-force]. So the milestone reads and predicts the axial force, not the drag.

Divided by the dynamic pressure, half the air's density times the airspeed squared, the axial
force over the mass becomes `C_A·A/m` in m²/kg: the axial force coefficient times the
[reference area][ref-area], over the mass. Both programs compute it along their own flights:
HPR Sim as its drag model's axial coefficient, OpenRocket as its axial drag coefficient.

The logs hold it, unread. Counted 2026-11-19 from the collection's manifest and its catalog's
column names, with no log opened:

- 83 flights in the manifest;
- 48 with a log the catalog lists with an acceleration column ([ADR-352][adr-352]'s count);
- 42 of those 48 not marked excluded;
- 34 of those 42 with no early deployment noted.

[M2.14][m2-14] will read the same channels during the boost, with the same refusals and the same
check against the barometer. As with the rail-exit speed, a first number is easy to shade: by
which coasts count, by where the coast is taken to start or end, or by how the airspeed is read.
So the rule, the readings and both programs' predictions come first: each is merged, and its
SHA-256 fingerprint recorded, before any error is computed.

**Decision.**

1. **M2.15, drag in the coast against the accelerometer logs, in two increments.** M2.15 is met
   when both are.
   - **M2.15a, the rule, the readings and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no error.
     - **The reading.** The analyzer reads, from a log's acceleration channel along the
       rocket's axis over the coast, `C_A·A/m` at Mach 0.3, 0.5 and 0.7 where the coast passes
       them, each with a 90% interval. The rule may add Mach numbers below 0.8; it may drop
       none. It divides by the dynamic pressure of the airspeed: the speed over the ground less
       the day's wind at the reading's height from the [ERA5 file][era5], with the wind's
       uncertainty in the interval. The air's density and speed of sound come from the same
       file. The rule fixes, beforehand, how the speed is read from the log, how the coast's
       start (the motor's burnout) and its end (the first of apogee and any deployment) are
       found, and where the channel's zero in the coast comes from. The reading never uses a
       simulation: it lives in `hpr-flightdata`, the crate [the architecture][arch] bars from
       depending on the simulator's.
     - **What it refuses.** The five kinds of log [M2.14a][m2-14a] refuses: one that starts
       after the rocket moved or has too few pad samples, a channel whose axis or handling of
       gravity can't be settled, a channel at its sensor's limit, too few samples, and a column
       that is no sensor's reading. Also a coast that passes none of the Mach numbers with as
       many samples as the rule fixes, and a log in which a motor lights or a part separates
       inside the coast. No refusal is decided from a `C_A·A/m` read, a prediction or an error.
     - **The synthetic check.** That reading is checked, before it reads any real log, on
       synthetic flights from liftoff to apogee whose `C_A·A/m` is known, their thrust tailing
       off as the plans' motor files do. The rule states, with a source for each, the levels
       the set covers: of sample rate, of the coast's top Mach number, of the flight path's
       angle from vertical, of the angle of attack, of the wind and its change with height, of
       the accelerometer's noise, zero offset and scale error, and of the barometer's noise and
       lag where the speed is read from it. The set includes the worst value any of those
       sources gives for each. Each level has at least 50 flights that should be read and the
       set at least 200; each kind of log that must be refused has at least 20.
     - **The three lines.** The set must meet [ADR-347][adr-347]'s three lines as
       [M2.14a][m2-14a] states them, counted by reading, not by log. Coverage: over the set,
       the share of readings whose 90% interval holds the true value isn't below 90% by a
       one-sided exact binomial test at the 5% level, and at no level under 80%. Width: at every
       level, the 90th percentile of the half-widths is under a bound the rule fixes beforehand
       from that level's rate, noise, zero offset, scale error, wind and samples in the coast.
       Refusal: every log that must be refused is refused. Each line is shown able to fail: a
       reader that widens every interval tenfold, one that refuses nothing, one that leaves the
       sensor's scale out of its interval, one that leaves its zero offset out, and one that
       divides by the speed over the ground each fail the set.
     - **The barometer check.** A log's accelerometer is checked against its barometer by
       [M2.14a][m2-14a]'s rule, unchanged. A flight that fails it is left out under that cause,
       and a log with no barometer under its own. Where a flight has more than one log with a
       usable channel, [M2.14a][m2-14a]'s rule names which gives the reading.
     - **The predictions.** Each program's prediction is its own `C_A·A/m` along its own coast,
       at the same Mach numbers, at its own angle of attack and over its own mass there: HPR
       Sim's from its drag model's axial coefficient, OpenRocket 24.12's from its axial drag
       coefficient, reference area and mass in its flight's history, on the same plan. The run
       that stores OpenRocket's history merges here. A Mach number a log's coast passes and a
       program's coast doesn't has no prediction from that program; the record says so, and the
       reading is counted under that cause for that program. A prediction reads only the
       flight's plan, the fields [ADR-333][adr-333] lets a prediction read, the design file and
       the day's ERA5 file. No thrust, mass or time comes from the log.
     - **The error.** It is the predicted `C_A·A/m` less the read one over the read one. A
       prediction is high when it is above the reading. Which side flatters depends on the plan,
       as for the apogee ([which miss flatters][flatters]): drag predicted high makes the apogee
       read low, which flatters a waiver or a field's ceiling; drag predicted low makes it read
       high, which flatters a contest's lowest altitude.
     - **The record.** For each flight whose cataloged log lists an acceleration column, 48 on
       2026-11-19, one record holds: its id; its plan; the SHA-256 of its design file, of its
       motor's file and of its ERA5 file; the commit that flew it; the coast's start and end
       it read; each reading with its interval and its count of samples; each program's
       prediction at each Mach number, or the cause it has none; each program's apogee from
       the run that gave its prediction; the barometer check's figure; where the plan's mass
       came from (weighed, the design file's, or an override), read by a rule the record names;
       and its cause if it is left out. A flight that is refused or left out keeps its record,
       with its cause. A record is never written over an older one. No plan field is changed
       for this comparison.
     - **The seal.** The records are sealed as [M2.14a][m2-14a]'s are: each record's SHA-256 is
       merged in a list of its own and a digest of the sorted ids beside it, both read at the
       merge base, so no record is dropped, added or rewritten unseen and a branch can't change
       the sealed set.
     - **The public flights.** Each is listed as having a log with an acceleration channel the
       reading can use, or none. Those with one get a record and are sealed the same way.
   - **M2.15b, the errors.** *Done when:*
     - The fixture report computes each error from the records [M2.15a][m2-15a] merged. The run
       stops, comparing nothing, on the cases [M2.14b][m2-14b]'s run stops on. A figure from the
       model as it then stands may be shown beside, marked as not the prediction.
     - A flight's error, for each program, is the mean of its readings' errors over the Mach
       numbers that the reading and both programs share, so a flight that passes three counts
       once and both programs are averaged over the same points. The readings dropped for that
       are counted by cause.
     - The report gives, as aggregates, for each program over the flights both fly, with the
       rest counted:
       - the flights compared;
       - the mean absolute and the mean signed error, in percent of the read value, over the
         flights and, beside them, over the readings;
       - the same at each Mach number, with its count;
       - how many predictions are high and how many low;
       - the median half-width of the readings' intervals;
       - how many readings have the prediction inside their interval, out of how many.
     - It gives, for each program, how many flights' sealed apogee error, against the logged
       apogee the fixture report reads, and coast-drag error
       have the signs drag would explain (drag low with the apogee high, or drag high with it
       low), out of the flights compared, and the rank correlation of the two errors with a 95%
       interval.
     - It gives the same on each half of the collection ([ADR-332][adr-332]) and on the flights
       whose plan's mass is weighed, each beside the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights with a usable channel get the same aggregates, marked as not
       predicted first.
     - The scoreboard gains `accuracy.coast-drag`, the mean absolute error in percent of the
       read `C_A·A/m`, lower better, and `bias.coast-drag`, the mean signed error, nearer zero
       better, each for HPR Sim and OpenRocket 24.12. The bias row names which side flatters
       which plan. Both rows give the median half-width of the readings' intervals.
     - The scoreboard's check holds HPR Sim's two cells by the calibration half's figures, as it
       holds `accuracy.collection` ([ADR-332][adr-332]); OpenRocket's are shown, not held.
     - A test pins which sign means high, on cases that include a prediction equal to its
       reading, and a probe that flips the sign fails it.
     - [The guide to trusting a simulation][trust] and [the accuracy page][accuracy] state the
       measured errors, how many flights they rest on, how wide the readings' intervals are and
       what the figures leave out.
2. **Sixth in the queue,** after [M2.14][m2-14] and before [M2.3d4][m2-3d4], a stage's override
   that adds to its parts. It reads the channels M2.14a teaches the analyzer to read and refuse,
   and uses its barometer check unchanged, so it follows it.
3. **What the rows can't say.** A reading is the axial force over the real mass; a prediction is
   over the mass the plan flies. Where the plan's mass is wrong, the error holds that too, which
   is why [M2.15b][m2-15b] gives the weighed-mass flights apart. The rows are of the coast, with
   the motor out and below Mach 0.8: they say nothing of the base under a burning motor, of drag
   past Mach 0.8, or of which part of the rocket the drag comes from. That last is
   [M1.14i2][m1-14i2]'s, the planned increment that splits the drag below Mach 0.8 by term, and
   a whole rocket's coefficient is not the measured reference of one term its *done when* asks
   for.
4. **Nothing else moves.** No tolerance changes, this record adds no check today (M2.15b's rows
   are held only once it lands), and no queued
   milestone's *done when* is changed, [M1.14i][m1-14i]'s among them. The roadmap's lines for
   three of them are shortened (Consequences); their records are not edited.

**Consequences.**

- A real-flight row that tells drag from thrust is on the roadmap, with a cell for each program.
- When it lands, the sign test and the rank correlation say, for each program, whether its coast
  drag errs with its apogee, flight by flight.
- The readings are coarse where the coast is short and slow; the rows are honest only with the
  readings' width beside them, so [M2.15b][m2-15b]'s *done when* asks for it.
- The review's new ideas are in [the backlog][ideas], under its date.
- To keep the roadmap within its byte budget, the [M2.12a][m2-12a], [M2.13a][m2-13a] and
  [M2.14a][m2-14a] lines no longer restate their records' *done when* after pointing at them.
  Each still points at its record's decision 1: [ADR-338][adr-338], [ADR-347][adr-347] and
  [ADR-352][adr-352], unchanged.

**How it was checked.** The two mean errors are [the scoreboard][score]'s `bias.collection` cells
over 60 flights. The quoted sentence is [ADR-359][adr-359]'s decision 5; the impulse scale of
0.94 is [ADR-348][adr-348]'s summary. The 48 is [ADR-352][adr-352]'s count; the 42 and 34 are of
the same 48 by the manifest's excluded and early-deployment fields. That the axial force differs
from the drag by the lift's share at an angle follows from resolving the two along the axis.
The scoreboard's row ids were listed from the committed scoreboard: none reads a drag. The
analyzer reads no axial force coefficient, the fixture report has no coast-drag column and the
scoreboard has no `accuracy.coast-drag`, so both *done when*s fail today. `cargo test -p xtask`
checks that the queue's ids are open in the roadmap.
