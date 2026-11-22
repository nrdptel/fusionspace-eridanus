# ADR-370: The 2026-11-22 review: the next flight predicted from the last one's log (2026-11-22)

- **Status:** accepted; promotes [M2.16][m2-16] to the [queue][queue], seventh, after [M2.15][m2-15] (drag in the coast against the logs), in two increments: [M2.16a][m2-16a], the synthetic check, the fits and the predictions merged first, then [M2.16b][m2-16b], the errors, with HPR Sim's carried fit set against the corrections a flyer can make by hand.
- **Summary:** A flyer who has logged one flight of a rocket predicts the next from it. HPR Sim can fit its drag to the first flight's log; whether that fit makes the next flight's apogee, on another motor and another day, any better is unmeasured. Fifteen rockets in the private collection have two or more logged flights that count. This record fixes the rule now, before any of those flights is fitted; the milestone checks the method on synthetic pairs, fits each earlier flight, merges every prediction, and then scores HPR Sim's carried fit against the uncorrected prediction and against corrections by hand, on new rows `accuracy.second-flight` and `bias.second-flight`.

[m2-16]: ../decisions-and-roadmap.md#m2-16
[m2-3c5]: ../decisions-and-roadmap.md#m2-3c5
[m2-16a]: ../decisions-and-roadmap.md#m2-16a
[m2-16b]: ../decisions-and-roadmap.md#m2-16b
[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-13]: ../decisions-and-roadmap.md#m2-13
[m2-14]: ../decisions-and-roadmap.md#m2-14
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m7-3]: ../decisions-and-roadmap.md#m7-3
[queue]: ../ROADMAP.md#queue
[ideas]: ../research/ideas.md
[idea]: ../research/ideas-flight-data.md#from-the-2026-10-31-planning-review-adr-310-adr-311
[fit]: ../physics/flight-fit.md
[sets]: ../physics/synthetic-flights.md#the-sets
[trust]: ../trusting-a-simulation.md
[flatters]: ../trusting-a-simulation.md#which-miss-flatters
[accuracy]: ../accuracy.md
[collection]: ../accuracy.md#real-flights-of-the-private-collection
[era5]: ../format/era5.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[or-features]: https://openrocket.readthedocs.io/en/latest/introduction/features.html
[planner]: https://github.com/Hide-In-Minion/hpr-launch-planner
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[adr-333]: 0333-m2-7b-new-flights-predicted-first.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[adr-348]: 0348-m7-5b-the-correction-fails-on-the-public-flights.md
[adr-360]: 0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md

**Context.** Each review raises one bar on accuracy against real flights while the project has
no outside users ([ADR-322][adr-322]). The four milestones promoted since 2026-11-11 score other
outputs, or say where the apogee's error comes from: descent rates ([M2.12][m2-12]), landing
points ([M2.13][m2-13]), the speed off the rail ([M2.14][m2-14]) and the drag in the coast
([M2.15][m2-15]). This one scores the apogee in the way a flyer with a log already uses a
simulator: to predict the next flight.

Flyers tune a simulator to their own flights. A launch planner published on 2026-11-20 keeps a log
of predicted against actual flights, and its "Suggest tuning" proposes correction factors from them
([its README][planner], read 2026-11-22); it states no check of how well they carry to the next
flight. OpenRocket's guide lists importing altimeter data as a planned feature and overriding a
part's or a stage's drag coefficient as a present one ([OpenRocket's features][or-features], read
2026-11-22). So with it a flyer can correct by hand: scale the next prediction by the last flight's
miss, or set a drag coefficient that brings the last flight's apogee to its log. The backlog has held the idea since
2026-10-31 ([a second flight on the first flight's drag][idea]), with the row it proposed,
`accuracy.second-flight`.

HPR Sim already has the tool. Its [least-squares fit][fit] changes a drag scale, an impulse scale,
a wind and the mass until the simulated flight passes through a log's heights up to apogee, each
pulled toward a stated value by a range it is told. On a real flight the simulator's own errors go
into those scales. A drag scale fitted to one flight is therefore a correction for that rocket.
Nothing says yet whether it carries to the same rocket's next flight, on another motor, another day
and another loading.

The collection holds the test. Counted 2026-11-22 from its manifest and its catalog's rocket-name,
flyer and motor fields, with no log opened (a rocket here is one name flown by one flyer, each
lower-cased and trimmed). The manifest lists all 83 flights the collection pairs with a design and
marks 11 of them excluded, left out of the errors against a log: 7 had an airbrake or canard
acting in the climb, one a motor anomaly, one a design that isn't the flown rocket, two another
reason. The [accuracy page][collection] compares 60 of the
83 with their logs.

- 83 flights in the manifest, 49 rockets; 16 rockets have two or more flights, 50 flights in all;
- 72 flights not marked excluded, in which 15 rockets have two or more flights, 45 flights in all:
  30 later flights, each with an earlier flight of its rocket to fit;
- on those 45 flights, 12 of the 15 rockets flew more than one motor, by the catalog's motor field;
- of the 72, those with no early deployment noted: 11 rockets with two or more flights, 31 flights,
  20 later flights.

Four hazards shape the rule.

- **The flights aren't independent.** A rocket's later flights share its errors, so the test rests
  on 15 rockets, not 30 flights, and a few rockets hold many of the flights.
- **Most of the answer is known already.** The fixture report's run already computes each flight's
  error without a correction, for both programs (the report publishes aggregates only); a
  correction by ratio follows from two of those. So four of the six predictions below have errors
  that could be computed today; only those that use a fit are new. So every choice that decides which flights count, and how, is
  fixed here, before any of these flights is fitted, not left to the milestone.
- **The fit's ranges are priors, not walls.** A fitted scale can land outside its stated range, and
  widening a range moves the fitted value itself. So the ranges are fixed here, unchanged from the
  collection's fits.
- **The fit's drag interval fails one of its synthetic tests** ([ADR-368][adr-368], issue #1108).
  The milestone uses the fitted value only and prints no interval from the fit's drag scale.

**Decision.**

1. **M2.16, the next flight from the last one's log, in two increments, by the rule below.** M2.16
   is met when both increments are.
   - **The rule, fixed by this record.**
     - **One rocket:** the same rocket name and flyer in the catalog, each lower-cased and trimmed.
       Two flights of one rocket whose design files' outer shapes differ (the outside parts in
       order, each one's length and diameter, the fins' count, chords, span and sweep, any of them
       by more than 1 mm) are kept, marked as changed, and counted apart.
     - **The order:** by the manifest's launch time; flights at the same time, by flight id. Each
       flight after a rocket's first is a later flight. Its source flight is the latest earlier
       flight of its rocket that has a usable fit. Where a launch time is marked assumed, the order
       rests on it, and the record says so.
     - **Which flights:** a later flight is compared when the fixture report compares it. A source
       flight is one the manifest doesn't mark excluded. A flight with an early deployment noted
       is never a source flight, since a drogue or a split in its coast isn't in the fit's model;
       as a later flight it is compared and counted apart.
     - **The fit:** [M7.3][m7-3]'s least-squares fit as it stands at [M2.16a][m2-16a]'s merge, on
       the source flight's logged heights up to apogee, with that flight's own plan, motor file and
       the day's [ERA5 file][era5], flown in ERA5's own wind, the air the predictions fly. That is
       new: the collection's fits solve for a constant wind and fly ERA5's only in a check before
       the fit (`era5_wind`). Here the wind is not solved for. The drag's, impulse's and mass's stated
       values and ranges are those the collection's predictions from the boost use, unchanged:
       the impulse's range is not widened. A fit is usable when the fit stops by its own rule on a
       log it doesn't refuse; its value is used whatever it is, and the record gives it with how
       many of its range's standard deviations it lies from the stated value.
     - **What is carried:** the drag scale. The later flight is flown as the fixture report flies
       it, with only its drag times that scale.
   - **M2.16a, the synthetic check, the fits and the predictions.** *Done when:* a pull request of
     its own merges everything below, and computes no error on a real flight.
     - **The synthetic check.** Before any real flight is fitted, the method is checked on at
       least 200 synthetic pairs: one design flown on two motors on two days whose wind profiles
       differ, the truth flown on [set B's model][sets] (the synthetic flights' deliberately
       different drag, wind profile and thrust curve), its drag off by a scale drawn from the fit's stated range, each motor's
       impulse off its rating by a scale drawn around 0.94 ([ADR-348][adr-348]'s fitted value)
       with the fit's stated spread, and in at least 50 pairs the second flight past Mach 0.8 and
       the first not. The fit is given the first day's true wind, as the real fits are given ERA5's,
       and each second flight is predicted with its true impulse and wind, so what differs is the
       drag and set B's tilt of the thrust curve. On the pairs whose drag scale is more than 5% from 1, the carried scale's absolute
       apogee error must be the smaller on more pairs than not by a one-sided sign test at the 5%
       level, against carrying nothing; ties are dropped, and a test with no untied pair fails. A
       probe that replaces each fitted scale with 1 must fail it. The result is given on the pairs past Mach 0.8 apart.
     - **The predictions.** Each later flight gets six: (1) HPR Sim's apogee with the carried drag
       scale; (2) HPR Sim's apogee as the fixture report flies it, its plain prediction; (3) HPR
       Sim's plain prediction with both the drag and the impulse scales carried; (4) OpenRocket
       24.12's plain apogee, from the oracle's run as the fixture report flies it; and, for each
       program, (5, 6) the ratio correction: the later flight's plain apogee times the source
       flight's logged apogee over the source flight's plain apogee. For example, a source flight
       logged at 900 m and predicted at 1,000 m gives a ratio of 0.9, so a later flight predicted
       at 1,200 m becomes 1,080 m. The record also gives OpenRocket's apogee with its stages' drag
       coefficient overridden by one value, found by bisection so that the source flight's apogee
       matches its log, where the oracle can set that override; where it can't, the record says so
       and gives the cause. Where a program can't fly the source flight, that program's ratio
       correction has none, and its later flights are counted by that cause. A prediction reads only the later flight's plan, the fields [ADR-333][adr-333]
       lets a prediction read, its design file, its motor file and the day's ERA5 file; nothing
       comes from the later flight's log.
     - **The record.** For each later flight one record holds: its id and its source flight's;
       the rocket, by an anonymous id; whether the shapes changed; whether either flight has an
       early deployment noted; whether either launch time is assumed; the SHA-256 fingerprint (a hash
       that changes if one byte does) of both design files, both motor
       files and both ERA5 files; the commit that flew it; the fitted scales and the fit's stop
       state; each prediction, or the cause it has none; and its cause if it is left out. A record
       is never written over an older one. No plan field is changed for this comparison.
     - **The seal.** As [M2.15a][m2-15a]'s ([ADR-360][adr-360]): each record's SHA-256 merged in
       a list of its own, and a digest of the sorted ids beside it, both read at the merge base.
   - **M2.16b, the errors.** *Done when:*
     - The fixture report computes each error from the records [M2.16a][m2-16a] merged, and
       stops, comparing nothing, when a record's fingerprint or the sorted ids don't match the
       merge base's list. A figure from the model as it then stands may be shown beside, marked as
       not the prediction.
     - A later flight's error, for each prediction, is its apogee less the logged one, over the
       logged one, as the fixture report defines the apogee error.
     - The report gives, as aggregates, for each prediction over the later flights that all six
       have, with the rest counted by cause:
       - the later flights and the rockets compared;
       - the mean absolute and the mean signed error over the rockets, each rocket's later flights
         averaged first, and over the flights;
       - how many predictions are high and how many low;
       - the difference in absolute error between the carried fit and each other prediction,
         averaged over rockets, paired by flight, with a 95% interval from a bootstrap that
         resamples rockets and the exact sign-flip interval on the rockets' differences
         ([ADR-257][adr-257]), Hartigan's, as [the fit's page][fit] uses it.
     - It gives the same on the changed-shape pairs, on the later flights with an early deployment
       noted, on those whose plain prediction passes Mach 0.8 while their source flight's doesn't,
       on each half of the collection by the later flight's half ([ADR-332][adr-332]), and on the
       later flights whose motor is or isn't the source flight's, each beside the whole.
     - Every flight in the manifest that isn't a later flight compared is counted by cause, so
       the compared and the not compared add up to the manifest.
     - The scoreboard gains `accuracy.second-flight`, the mean absolute error over the rockets,
       lower better, and `bias.second-flight`, the mean signed error over the rockets, nearer zero
       better. HPR Sim's cell is the carried fit; OpenRocket 24.12's is its ratio correction over
       the same later flights. The rows show the other predictions
       beside, not held, and the bias row names which side flatters which plan
       ([which miss flatters][flatters]).
     - The scoreboard's check holds HPR Sim's two cells by the calibration half's figures, as it
       holds `accuracy.collection` ([ADR-332][adr-332]); OpenRocket's are shown, not held.
     - A test pins which sign means high, on cases that include a prediction equal to its log,
       and a probe that flips the sign fails it.
     - [The guide to trusting a simulation][trust], [the accuracy page][accuracy] and the
       scoreboard's rows state the measured errors, how many rockets and flights they rest on, the
       intervals, and what the rows leave out. While fewer than 10 rockets are compared, each says
       the figures rest on too few rockets to rank the methods.
2. **Seventh in the queue,** after [M2.15][m2-15] and before [M2.3d4][m2-3d4], a stage's override
   that adds to its parts. It needs no reading those milestones add: the fit and the fixture
   report exist. It follows them because reviews add their promotions in the order made.
3. **What the rows can't say.** A carried drag scale holds what the fit puts in the drag: the
   drag model's error, the finish, and whatever of the source flight the fit can't tell from drag.
   A mass or thrust error can go into the scales the fit solves for them, and some may stay in
   the drag: a barometer's boost
   can't tell a motor that delivered less from a rocket built heavier, and the impulse scale
   carries both ([the fit's page][fit]). With the impulse's range widened, the twelve
   calibration-half flights' impulse scales average 0.936, all below 1 (the same page), which may
   be a shortfall the motors share; so the record also carries both scales, shown beside. The rows
   say nothing of a rocket's first flight, which is what [the collection's rows][collection]
   score. A rocket that changed between its flights is counted apart, but a change the design
   files don't show, a repair or new paint, stays inside the error.
4. **Nothing else moves.** No tolerance changes, this record adds no check today (M2.16b's rows
   are held only once it lands), and no queued milestone's *done when* is changed. Issue #1108 is
   unchanged. The carried scale is a per-rocket input, so [ADR-346][adr-346]'s rule for a
   correction of the model, judged on flights predicted first, doesn't apply to it; a later change
   of the model's defaults made from these fits would be such a correction ([ADR-348][adr-348]).

**Consequences.**

- A real-flight row that scores HPR Sim's fit to a log, the capability the scoreboard's
  `accuracy.from-boost` row says OpenRocket and RocketPy lack, on the task a flyer uses it for, is
  on the roadmap, with the corrections a flyer can make by hand beside it.
- When it lands, the paired differences say, rocket by rocket, whether the fit carries better than
  a ratio, and whether either beats no correction.
- With 15 rockets, the intervals will be wide; the rows are honest only with them beside, so
  [M2.16b][m2-16b]'s *done when* asks for them.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The counts were made 2026-11-22 from the manifest's flight id, excluded,
early-deployment and launch-time fields, and the catalog's rocket name, flyer and motor fields,
grouping by name and flyer; no log or design file was opened. By the manifest's motor names, which
write one motor two ways, 13 of the 15 rockets flew more than one; by the catalog's motor field,
12. The manifest read is the one the fixture report reads, after [M2.3c5][m2-3c5]'s recheck of the
collection. No two flights of one rocket share a launch time. The planner's tuning feature is in its
README's feature list and its "Suggest tuning" step. OpenRocket's features page lists importing
altimeter data among planned features and overriding the drag coefficient of parts or stages among
present ones. What the fit fits, how its
ranges pull, the twelve impulse scales and the boost's mass-or-motor reading are from [the fit's
page][fit]; that the collection's fits fly ERA5's own wind only in a check before the fit is
`era5_wind` in `xtask`'s collection fits. The excluded flights' causes are the manifest's. The scoreboard has no `accuracy.second-flight` row and the fixture report no
carried-fit column, so both *done when*s fail today. `cargo test -p xtask` checks that the queue's
ids are open in the roadmap.
