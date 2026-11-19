# ADR-360: The 2026-11-19 review: drag in the coast held to accelerometer logs, predictions first (2026-11-19)

- **Status:** accepted; promotes [M2.15][m2-15] to the [queue][queue], sixth, after [M2.14][m2-14], in two increments: [M2.15a][m2-15a], the rule, the readings and the predictions merged first, then [M2.15b][m2-15b], the errors, for HPR Sim and OpenRocket 24.12 both.
- **Summary:** Both simulators' apogees run high on the private collection's logged flights, by +7.72% and +6.72% on average, and the apogee alone can't say why: too little drag and too much thrust read the same way. The fit that tried to tell them apart put it all on the thrust ([ADR-348][adr-348]); the split by Mach band put the largest part of the gap between the two programs in the drag below Mach 0.8 ([ADR-359][adr-359]). An accelerometer settles the drag on its own. Coasting, it reads the drag over the mass, with no thrust and no gravity in the reading. The milestone reads that, below Mach 0.8, from each log with an acceleration column, merges the readings and both programs' predictions before any error is computed, and then scores both programs on new rows, `accuracy.coast-drag` and `bias.coast-drag`, with drag predicted too low as the flattering side.

[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
[m2-15b]: ../decisions-and-roadmap.md#m2-15b
[m2-14]: ../decisions-and-roadmap.md#m2-14
[m2-14a]: ../decisions-and-roadmap.md#m2-14a
[m2-14b]: ../decisions-and-roadmap.md#m2-14b
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m1-14i]: ../decisions-and-roadmap.md#m1-14i
[m1-14i2]: ../decisions-and-roadmap.md#m1-14i2
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[trust]: ../trusting-a-simulation.md
[arch]: ../ARCHITECTURE.md
[era5]: ../format/era5.md
[readings]: ../physics/log-readings.md
[specific-force]: ../glossary.md#specific-force
[accuracy]: ../accuracy.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[adr-347]: 0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[adr-348]: 0348-m7-5b-the-correction-fails-on-the-public-flights.md
[adr-352]: 0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[adr-359]: 0359-drag-below-mach-0-8-next-the-drag-part-of-the-gap-to-openrocket-by-mach-band.md

**Context.** This review raises one bar on accuracy against real flights, as each review does
while the project has no outside users ([ADR-322][adr-322]). The last four queued a descent
rate, a landing point and a speed off the rail against the logs. This one goes after the
apogee's own error, which has not moved in two months.

On the private collection's 60 logged flights, HPR Sim's apogee is +7.72% high on average and
OpenRocket 24.12's +6.72% ([the scoreboard][score]). Two attempts to place that error disagree.
[ADR-348][adr-348] fitted one correction of thrust and drag to the calibration half's apogees:
it came out as an impulse scale of 0.94 with the drag left at 1, and it failed on the public
flights. [ADR-359][adr-359] split the gap between the two programs by the Mach number the drag
acts at and found its largest part below Mach 0.8. Its decision 5 says what neither can settle:
"Both codes climb above the logs on average, +7.72% and +6.72%, so more drag from any cause
moves the mean toward them, and a wrong mass or thrust would read the same way." An apogee is one number, and drag,
thrust and mass all move it.

An accelerometer separates them. It doesn't feel gravity: it reads every other force on the
rocket over its mass, the [specific force][specific-force]. Coasting, with the motor out,
that is the drag's pull back over the mass ([what an accelerometer reads][readings]). So in the
coast the reading is the drag over the mass, whatever the thrust was, and the speed and air
it acts at can be read from the same flight. At the small angles of attack of a coast, the force
along the axis is the drag to within the angle's cosine, 0.14% at 3°.

The logs hold it, unread. 48 of the collection's 83 flights have a log the catalog lists with an
acceleration column ([ADR-352][adr-352], counted 2026-11-16). Of those, 42 are not marked
excluded in the manifest, 34 of the 42 have no early deployment noted, and 30 of those 34 have
a rate of 100 Hz or more in the catalog (counted 2026-11-19 from the manifest and the catalog's
column names and rates; no log was opened for this count). [M2.14][m2-14] will read the same
channels during the boost, with the same refusals and the same check against the barometer.

As with the rail-exit speed, a first number is easy to shade: by which coasts count, by where
the coast is taken to start, or by how the speed the drag is divided by is read. So the rule,
the readings and both programs' predictions come first.

**Decision.**

1. **M2.15, drag in the coast against the accelerometer logs, in two increments.** M2.15 is met
   when both are.
   - **M2.15a, the rule, the readings and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no error.
     - The analyzer reads, from a log's acceleration channel along the rocket's axis over the
       coast, the rocket's drag area over its mass, `C_D·A/m` in m²/kg, at Mach 0.3, 0.5 and
       0.7 where the coast passes them, each with a 90% interval. The rule may add Mach numbers
       below 0.8; it may drop none. The coast runs from the motor's burnout, found from the
       log by a rule fixed beforehand, to the first of apogee and any deployment. The speed and
       the Mach number come from the same log, by a method the rule fixes; the air's density
       and speed of sound come from the day's [ERA5 file][era5] at the reading's height. The
       reading never uses a simulation: it lives in `hpr-flightdata`, the crate
       [the architecture][arch] bars from depending on the simulator's.
     - It refuses the five kinds of log [M2.14a][m2-14a] refuses, and also a coast that passes
       none of the Mach numbers with as many samples as the rule fixes, and a log in which a
       motor lights or a part separates inside the coast. Each is told from the log's columns,
       its board and its events, never from a reading's value.
     - That reading is checked on synthetic coasts whose `C_D·A/m` is known, before it reads
       any real log. The rule states, with a source for each, the levels the set covers: of
       sample rate, of the coast's top Mach number, of the flight path's angle from vertical,
       of the angle of attack, of the accelerometer's noise, zero offset and scale error, and
       of the barometer's noise and lag where the speed is read from it. The set includes the
       worst value any of those sources gives for each. Each level has at least 50 coasts that
       should be read and the set at least 200; each kind of log that must be refused has at
       least 20.
     - The set must meet the three lines [ADR-347][adr-347] sets for the landing readings,
       coverage, width and refusal, as [M2.14a][m2-14a] states them, each with its counts.
       Each line is shown able to fail: a reader that widens every interval tenfold, one that
       refuses nothing, and one that leaves the sensor's scale out of its interval each fail
       the set.
     - A log's accelerometer is checked against its barometer by [M2.14a][m2-14a]'s rule,
       unchanged. A flight that fails it is left out under that cause, and a log with no
       barometer under its own.
     - Where a flight has more than one log with a usable channel, [M2.14a][m2-14a]'s rule
       names which gives the reading.
     - HPR Sim's prediction is its own `C_D·A/m` along its own flight, power off, at the same
       Mach numbers: the drag coefficient and reference area its flight used, over the mass it
       flew. It reads only the flight's plan, the fields [ADR-333][adr-333] lets a prediction
       read, the design file and the day's ERA5 file. No thrust, mass or time comes from the
       log.
     - OpenRocket 24.12's prediction is the same quantity from its own flight's history on the
       same plan: its drag coefficient, reference area and mass at the same Mach numbers in its
       coast. The run that stores that history merges here.
     - It defines the error, the predicted `C_D·A/m` less the read one over the read one, and
       which side flatters: a prediction is low when it is below the reading, and low drag
       makes the apogee high.
     - Each flight's record states where the plan's mass came from: weighed, the design
       file's, or an override, as the manifest gives it. No plan field is changed for this
       comparison.
     - It names the cause each flight left out is counted under.
     - For each flight whose cataloged log lists an acceleration column, one record holds its
       id, its plan, the SHA-256 of its design file, of its motor's file and of its ERA5 file,
       the commit that flew it, the burnout and coast end it read, each reading with its
       interval and its count of samples, each program's prediction at the same Mach numbers,
       the barometer check's figure, the mass's source, and its cause if it is left out. A
       flight that is refused or left out keeps its record, with its cause. A record is never
       written over an older one.
     - The records are sealed as [M2.14a][m2-14a]'s are: each record's SHA-256 merged in a list
       of its own and a digest of the sorted ids beside it, both read at the merge base, so no
       record is dropped, added or rewritten unseen and a branch can't change the sealed set.
     - Each public flight is listed as having a log with an acceleration channel the reading
       can use, or none. Those with one get a record and are sealed the same way.
   - **M2.15b, the errors.** *Done when:*
     - The fixture report computes each error from the records [M2.15a][m2-15a] merged. The run
       stops, comparing nothing, on the cases [M2.14b][m2-14b]'s stops on. A figure from the
       model as it then stands may be shown beside, marked as not the prediction.
     - A flight's error is the mean of its readings' errors, so a flight that passes three Mach
       numbers counts once. The report gives, as aggregates, for each program over the flights
       both fly, with the rest counted: the flights compared; the mean absolute and the mean
       signed error, in percent of the read value; the same at each Mach number with its
       count; how many predictions are low and how many high; the median half-width of the
       readings' intervals; and for how many flights the prediction lies inside its reading's
       interval.
     - It gives, for each program, how many flights' apogee error and coast-drag error have the
       signs too little drag would give (drag low, apogee high) or too much would (drag high,
       apogee low), out of the flights compared, and the rank correlation of the two errors
       with a 95% interval.
     - It gives the same on each half of the collection ([ADR-332][adr-332]) and on the flights
       whose plan's mass is weighed, each beside the whole.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights with a usable channel get the same aggregates, marked as not
       predicted first.
     - The scoreboard gains `accuracy.coast-drag`, the mean absolute error in percent of the
       read `C_D·A/m`, lower better, for HPR Sim and OpenRocket 24.12, and `bias.coast-drag`,
       the mean signed error, nearer zero better, with low, the negative side, named as the
       flattering one. Both rows give the median half-width of the readings' intervals.
     - The scoreboard's check holds HPR Sim's two cells by the calibration half's figures, as
       it holds `accuracy.collection` ([ADR-332][adr-332]); OpenRocket's are shown, not held.
     - A test pins which sign means low, on cases that include a prediction equal to its
       reading, and a probe that flips the sign fails it.
     - The guide to trusting a simulation and the accuracy page state the measured errors, how
       many flights they rest on, how wide the readings' intervals are and what the figures
       leave out.
2. **Sixth in the queue,** after [M2.14][m2-14], the speed off the rail, and before
   [M2.3d4][m2-3d4]. It reads the channels M2.14a teaches the analyzer to read and refuse, and
   uses its barometer check unchanged, so it follows it.
3. **What the rows can't say.** A reading is the drag over the real mass; a prediction is over
   the mass the plan flies. Where the plan's mass is wrong, the error holds that too, which is
   why [M2.15b][m2-15b] gives the weighed-mass flights apart. The rows are of the coast, power
   off and below Mach 0.8: they say nothing of the base under a burning motor, of drag past
   Mach 0.8, or of which part of the rocket the drag comes from. That last is
   [M1.14i2][m1-14i2]'s split by term, and a whole rocket's coefficient is not the measured
   reference of one term that its *done when* asks for.
4. **Nothing else moves.** No tolerance changes, no check is added by this record, and no
   queued milestone's *done when* is touched, [M1.14i][m1-14i]'s among them.

**Consequences.**

- A real-flight row that tells drag from thrust is on the roadmap, with a cell for each program.
- When it lands, the apogee bias has a measured drag part on each compared flight: the sign
  test and the rank correlation say whether the coast's drag carries the bias or the boost
  does.
- The readings are coarse where the coast is short and slow; the rows are honest only with the
  readings' width beside them, so [M2.15b][m2-15b]'s *done when* asks for it.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The two mean errors are [the scoreboard][score]'s `bias.collection` cells
over 60 flights. The quoted sentence is [ADR-359][adr-359]'s decision 5; the impulse scale of
0.94 is [ADR-348][adr-348]'s summary. The 48 is [ADR-352][adr-352]'s count; the 42, 34 and 30
are of the same 48 by the manifest's excluded and early-deployment fields and the catalog's
rates. The angle's figure is 1 − cos 3° = 0.0014. The scoreboard's row ids were listed from the
committed scoreboard: none reads a drag. The analyzer reads no drag area, the fixture report
has no coast-drag column and the scoreboard has no `accuracy.coast-drag`, so both *done when*s
fail today. `cargo test -p xtask` checks that the queue's ids are open in the roadmap.
