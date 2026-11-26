# ADR-380: The 2026-11-26 review: time to apogee held to the logs, predictions first (2026-11-26)

- **Status:** accepted; promotes [M2.17][m2-17] to the [queue][queue], fourteenth, after [M2.16][m2-16] (the next flight from the last one's log), in two increments: [M2.17a][m2-17a], the rule, the reading and the predictions merged first, then [M2.17b][m2-17b], the errors, for HPR Sim and OpenRocket 24.12.
- **Summary:** A flyer picks a motor's ejection delay, or sets a timer, from the time a simulator says the rocket takes to reach the top. No report scores that time against a log for either program on the private collection. This record promotes the backlog's idea of scoring it: the analyzer's reading of the time from liftoff to apogee gets an interval of its own, checked on synthetic logs; each program's prediction is sealed before any error is computed; then the errors go on new rows `accuracy.time-to-apogee` and `bias.time-to-apogee`, in percent and in seconds.

[m2-17]: ../decisions-and-roadmap.md#m2-17
[m2-17a]: ../decisions-and-roadmap.md#m2-17a
[m2-17b]: ../decisions-and-roadmap.md#m2-17b
[m2-16]: ../decisions-and-roadmap.md#m2-16
[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
[m2-14a]: ../decisions-and-roadmap.md#m2-14a
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-13]: ../decisions-and-roadmap.md#m2-13
[m2-14]: ../decisions-and-roadmap.md#m2-14
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m7-2]: ../decisions-and-roadmap.md#m7-2
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[idea]: ../research/ideas-flight-data.md
[seal-idea]: ../research/ideas-accuracy.md#from-the-2026-11-11-planning-review-adr-338
[optimum]: ../physics/metrics.md#optimum-ejection-delay
[reading]: ../reading-a-flight-log.md#the-90-intervals-and-the-health-card
[builder]: ../the-builder.md
[delay-spread]: ../research/motor-delay-spread.md
[delay-logs]: ../research/delay-offset-in-flight-logs.md
[trust]: ../trusting-a-simulation.md
[accuracy]: ../accuracy.md
[collection]: ../accuracy.md#real-flights-of-the-private-collection
[arch]: ../ARCHITECTURE.md
[era5]: ../format/era5.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[estimates]: ../research/release-estimates.md
[vent]: ../research/altimeter-mach-error.md#4-how-a-vented-bay-lags
[plan]: ../plan.md#when-each-release-may-come
[real]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[boost]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/collection-boost.md
[or-3480]: https://github.com/openrocket/openrocket/issues/3480
[adr-068]: 0068-openrockets-flights-of-the-public-designs-and.md
[adr-289]: 0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-338]: 0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-347]: 0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[adr-352]: 0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[adr-360]: 0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md
[adr-370]: 0370-the-2026-11-22-review-the-next-flight-predicted-from-the-last-ones-log.md

**Context.** Each review raises one bar on accuracy against real flights while the project has
no outside users ([ADR-322][adr-322]). The rows on the [scoreboard][score] and the five milestones
promoted since 2026-11-11 score how high a rocket goes, how fast it comes down
([M2.12][m2-12]), where it lands ([M2.13][m2-13]), how fast it leaves the rail ([M2.14][m2-14]),
its drag in the coast ([M2.15][m2-15]) and its next flight's height ([M2.16][m2-16]). None scores
when it reaches the top.

A flyer uses that time directly. A motor's ejection charge fires a set delay after burnout, and
the delay to pick is the time from burnout to apogee: HPR Sim prints it as the
[optimum delay][optimum], and its [builder][builder] lists a best delay for each motor. A timer or
a flight computer's backup is set the same way, from the time to apogee. OpenRocket gives an
optimum delay too ([ADR-068][adr-068] records what its word means). A charge that fires off the
top fires at speed: near apogee the vertical speed changes by 9.81 m/s (32.2 ft/s) each second,
so a charge 1 s early or late meets about 10 m/s (32 ft/s) of vertical speed that a charge at the
top would not, and one 2 s off about 20 m/s (64 ft/s).

The motor adds its own scatter. NFPA 1125 lets a firing's delay differ from its label by 1.5 s or
20%, whichever is greater, up to 3 s ([motor delay spread][delay-spread]), and on 30 logged
flights the delays' standard deviation was 1.03 s ([delays in flight logs][delay-logs]). A
simulation's own timing error comes on top of that, and nothing here says how large it is.
Delays are a live question for flyers elsewhere too: on 2026-11-20 a user reported that
OpenRocket lists a 1,000 s delay among some motors' defaults ([its issue 3480][or-3480], read
2026-11-26).

What exists today is one unscored table. The [real-flights report][real] lists, for the 7 public
flights compared first, the log's and HPR Sim's time from the first rise through 30 m (98 ft) to
the apogee; the log's apogee there is its highest sample. By this record's arithmetic on that
table, HPR Sim's time differs from the log's by 1.07 s on average (4.63% of the logged time),
with its sign +0.26 s; 3 of the 7 differ by more than 1 s and 2 by more than 1.5 s. One flight's
apogee is 5.55% low while its time is 9.05% long, so the time is not the apogee's error over
again. No report gives a time for the private collection, and none gives OpenRocket's time for any
flight. The backlog has held the idea since 2026-10-24 ([a second observable for each logged
flight][idea], [ADR-289][adr-289]'s review), with the row it proposed, `accuracy.time-to-apogee`;
the 2026-11-11 review added that the collection's times can still be sealed before anyone sees
their errors ([the collection's other predictions][seal-idea], [ADR-338][adr-338]'s review).

The analyzer has most of the reading. From a log's height track it reads the liftoff and the
apogee's time, each with a bound and a 90% interval ([M7.2][m7-2]; [the intervals][reading]). It
gives the time from liftoff to apogee as a figure with no interval of its own.

The collection holds the flights. Counted 2026-11-26 from its manifest's excluded and
early-deployment fields, with no log or design file opened, and from the committed
[collection-boost report][boost]:

- 83 flights in the manifest, 11 marked excluded, 72 not;
- of the 72, 20 have an early deployment noted and 52 don't;
- 64 flights have a log chosen by the collection-boost report's rule: a barometer's track that
  reads on its own and gives an apogee;
- the [accuracy page][collection] compares 60 flights' apogees with their logs.

Five hazards shape the rule.

- **The top of a climb is flat.** Within 1 s of apogee a coasting rocket is within 4.9 m (16 ft)
  of its highest point, so a barometer's noise moves the time of its highest sample by a large
  part of a second. The analyzer's bound on the apogee's time already says so. An error of one
  second means little without the reading's width beside it.
- **The two clocks start differently.** A log starts where its logger did, and the analyzer's
  liftoff is the last sample on the pad. A simulation starts at ignition and moves a little
  later. Both times must run from first motion.
- **A charge before the top ends the climb.** 20 of the 72 flights have one noted. Their logged
  time is not a coast's.
- **The climbs have been read, the times not scored.** The fits from the boost and the
  least-squares fits have read some of these flights' heights against time. No committed report
  has set a plain prediction's time beside a log's for the collection. So the flights are held to
  sealed predictions; they are not blind.
- **A barometer lags.** The pressure inside a vented bay follows the air outside late
  ([how a vented bay lags][vent]), so a barometer's highest reading can come after the top. The
  synthetic check must include it.

**Decision.**

1. **M2.17, time to apogee against the logs, in two increments.** M2.17 is met when both are.
   - **M2.17a, the rule, the reading and the predictions.** *Done when:* a pull request of its
     own merges the three, and computes no error.
     - **The reading.** The analyzer reads the time from liftoff to apogee from a log's
       barometer track, with a 90% interval that holds the liftoff's and the apogee time's
       uncertainty and the barometer's lag. The rule fixes, beforehand, how the two readings'
       intervals combine and where the lag's size comes from, with its source. Where the log has
       a burnout that is not a floor, it also gives the time from burnout to apogee, with its
       interval. The reading never uses a simulation: it lives in `hpr-flightdata`, the crate
       [the architecture][arch] bars from depending on the simulator's.
     - **Which log.** The log the [collection-boost report][boost]'s rule chooses for a flight,
       unchanged.
     - **What it refuses.** A log whose liftoff the analyzer withholds, among them one that
       starts off the pad; one whose apogee is a floor; one that ends before its apogee; and
       one with no barometer's track. No refusal is decided from a time read, a
       prediction or an error.
     - **The synthetic check.** Before it reads any real log, the reading is checked on
       synthetic logs whose liftoff and apogee times are known. The rule states, with a source
       for each, the levels the set covers: of sample rate, of the barometer's noise, step and
       lag, of the pad's length before liftoff, and of the climb's length. The set includes the
       worst value any of those sources gives for each. Each level has at least 50 logs that
       should be read and the set at least 200; each kind of log that must be refused has at
       least 20.
     - **The three lines.** The set must meet [ADR-347][adr-347]'s three lines as
       [M2.14a][m2-14a] states them. Coverage: over the set, the share of readings whose 90%
       interval holds the true time isn't below 90% by a one-sided exact binomial test at the 5%
       level, and at no level under 80%. Width: at every level, the 90th percentile of the
       half-widths is under a bound the rule fixes beforehand from that level's rate, noise,
       step and lag. Refusal: every log that must be refused is refused. Each line is shown able
       to fail: a reader that widens every interval tenfold, one that refuses nothing, one that
       reports every interval at a tenth of its width, and one that counts the time from the
       log's first sample each fail the set.
     - **The predictions.** Each program's prediction is its own time from its rocket's first
       motion off the pad to its apogee, flown as the fixture report flies the flight, with
       nothing deployed before the top: HPR Sim's at this increment's merge, and OpenRocket
       24.12's from the oracle's run. That run stores OpenRocket's time at its highest point
       already; the change that stores its liftoff's time merges here. Where a flight has one
       motor, each program's time from burnout to apogee is recorded beside. A prediction reads
       only the flight's plan, the fields [ADR-333][adr-333] lets a prediction read, the design
       file, the motor file and the day's [ERA5 file][era5]. No time comes from the log.
     - **The error.** It is the predicted time less the read one, over the read one, and the
       same difference in seconds. A prediction is long when it is above the reading. Neither
       side flatters: a charge timed from a long prediction fires after the top and one timed
       from a short prediction before it, each at speed.
     - **The record.** For each flight in the manifest one record holds: its id; its plan; the
       SHA-256 fingerprint (a hash that changes if one byte does) of its design file, of its
       motor's file and of its ERA5 file; the commit that flew it; the log chosen; the liftoff
       and the apogee time it read; the reading with its interval; each program's prediction, or
       the cause it has none; whether an early deployment is noted; and its cause if it is left
       out. A flight that is refused or left out keeps its record, with its cause. A record is
       never written over an older one. No plan field is changed for this comparison.
     - **The seal.** As [M2.15a][m2-15a]'s ([ADR-360][adr-360]): each record's SHA-256 merged
       in a list of its own, and a digest of the sorted ids beside it, both read at the merge
       base.
     - **The public flights.** Each of the 12 gets a record sealed the same way. The 7 compared
       first are marked as not predicted first, since the [real-flights report][real] already
       lists their times.
   - **M2.17b, the errors.** *Done when:*
     - The fixture report computes each error from the records [M2.17a][m2-17a] merged, and
       stops, comparing nothing, when a record's fingerprint or the sorted ids don't match the
       merge base's list. A figure from the model as it then stands may be shown beside, marked
       as not the prediction.
     - The report gives, as aggregates, for each program over the flights that both fly and
       that have no early deployment noted, with the rest counted:
       - the flights compared;
       - the mean absolute and the mean signed error, in percent of the read time and in
         seconds;
       - how many predictions are long and how many short;
       - how many differ from the reading by more than 1.5 s, the figure NFPA 1125 allows a
         motor's own delay;
       - the median half-width of the readings' intervals;
       - how many readings have the prediction inside their interval, out of how many.
     - It gives, for each program, how many flights' sealed apogee error and time error have
       the same sign, out of the flights compared, and the rank correlation of the two errors
       with a 95% interval.
     - It gives the same aggregates on the flights with an early deployment noted, apart, and on
       each half of the collection ([ADR-332][adr-332]), each beside the whole.
     - Where both the log and the programs give a time from burnout to apogee, it gives that
       time's mean absolute and signed error in seconds, with its count, shown and not held.
     - Every flight in the collection's manifest that isn't compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The public flights get the same aggregates, those not predicted first marked.
     - The scoreboard gains `accuracy.time-to-apogee`, the mean absolute error in percent of the
       read time, lower better, and `bias.time-to-apogee`, the mean signed error, nearer zero
       better, each for HPR Sim and OpenRocket 24.12. Both rows give the mean absolute error in
       seconds and the median half-width of the readings' intervals. The bias row says what each
       sign does to a charge timed from the prediction.
     - The scoreboard's check holds HPR Sim's two cells by the calibration half's figures, as it
       holds `accuracy.collection` ([ADR-332][adr-332]); OpenRocket's are shown, not held.
     - A test pins which sign means long, on cases that include a prediction equal to its
       reading, and a probe that flips the sign fails it.
     - [The guide to trusting a simulation][trust], [the accuracy page][accuracy] and the
       [optimum delay's section][optimum] state the measured errors, how many flights they rest
       on, how wide the readings' intervals are and what the figures leave out.
2. **Fourteenth in the queue,** after [M2.16][m2-16] and before [M2.3d4][m2-3d4], a stage's
   override that adds to its parts. It needs no reading the milestones ahead of it add. It
   follows them because reviews add their promotions in the order made.
3. **What the rows can't say.** The time from liftoff to apogee is not a delay. A motor's delay
   runs from its own burnout and scatters by itself ([motor delay spread][delay-spread]), and
   the row leaves that out. The time from burnout to apogee, where both sides give it, also
   holds the error of the motor file's burn time, and the log's burnout is the coarsest of the
   three readings; so it is shown, not held. The rows say nothing of an upper stage's timing
   apart from its rocket's, of a flight whose charge fired before the top, or of a flight no
   barometer logged. A time's error has more than one cause: the impulse, the mass, the drag
   and the day's air each move it, and this record claims no diagnosis from it.
4. **Nothing else moves.** No tolerance changes, this record adds no check today (M2.17b's rows
   are held only once it lands), and no queued milestone's *done when* is changed.
5. **The release estimates, made again.** By [the estimates' method][estimates], on this day's
   queue and at 13.5 merged increments a week (56 in the 29 days from October 28 to November 25),
   the padded dates are December 19, 2026 for release 0.2, May 25, 2027 for 0.3, August 23, 2027
   for 0.4, November 5, 2027 for 0.5 and April 1, 2028 for 1.0. [The plan][plan] posts each as the
   end of its month, so 1.0 moves from the end of March 2028 to the end of April 2028 and the other
   four stand. This record's own milestone is what moves it: M2.17 is sized at 2 to 5 increments,
   as each of the five measurements before it is, ahead of 0.3, in a stretch whose time is
   tripled. Without it the same arithmetic gives March 26, 2028.

**Consequences.**

- The number a flyer picks a delay or sets a timer from gets a real-flight row, with a cell for
  each program.
- When it lands, the sign count and the rank correlation say, for each program, whether its time
  errs with its apogee, flight by flight. The public table's one flight with a low apogee and a
  long time says the two can part.
- The readings are coarse where a log is slow or its barometer noisy; the rows are honest only
  with the readings' width beside them, so [M2.17b][m2-17b]'s *done when* asks for it.
- The queue grows by one entry ahead of release 0.3, and release 1.0's estimate is a month later
  for it (decision 5).
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The counts were made 2026-11-26 from the manifest's excluded and
early-deployment fields; no log or design file was opened. The manifest read is the one the
fixture report reads. The 64 flights with a log chosen, and the rule that chooses it, are the
[collection-boost report][boost]'s. The 7 flights' times are the columns "log time to apogee (s)"
and "hpr time to apogee (s)" of the [real-flights report][real]'s first table, read from its
JSON; the differences are 0.04, 0.96, −1.94, 2.14, 0.30, −0.88 and 1.24 s, and the flight whose
apogee is low and whose time is long is the fourth. That those times run from the first rise
through 30 m to the log's highest sample is `compare_traces` in `hpr-validate`. The analyzer's
liftoff and apogee readings, their bounds and intervals, and the apogee's time after liftoff
with no interval, are the `Liftoff` and `Apogee` types of `hpr-flightdata`'s readings. The 4.9 m
is half of 9.81 m/s² times one second squared. The oracle's `fixture_flights.py` stores the time
at OpenRocket's highest altitude and no liftoff time. The fixture report's JSON holds no time to
apogee, and the scoreboard no `accuracy.time-to-apogee` row, so both *done when*s fail today.
NFPA 1125's allowance and the 30 flights' 1.03 s are quoted from the two research notes named.
OpenRocket's issue 3480 was read for its title and report only. The estimates' arithmetic is in
[the estimates' note][estimates], with the command that counts the 56 merges; the two entries
finished since it was first made on November 24 are out of its table, and M2.17 is in.
`cargo test -p xtask` checks that the queue's ids are open in the roadmap.
