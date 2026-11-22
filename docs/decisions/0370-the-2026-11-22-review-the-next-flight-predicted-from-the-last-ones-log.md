# ADR-370: The 2026-11-22 review: the next flight predicted from the last one's log (2026-11-22)

- **Status:** accepted; promotes [M2.16][m2-16] to the [queue][queue], seventh, after [M2.15][m2-15] (drag in the coast against the logs), in two increments: [M2.16a][m2-16a], the rule, the fits and the predictions merged first, then [M2.16b][m2-16b], the errors, with HPR Sim's fit set against the hand correction a flyer can make with OpenRocket 24.12.
- **Summary:** A flyer who has logged one flight of a rocket predicts the next from it. HPR Sim can fit its drag to the first flight's log; whether that fit makes the next flight's apogee, on another motor and another day, any better is unmeasured. Sixteen rockets in the private collection have two or more logged flights. The milestone fixes the rule, fits each earlier flight, and merges every prediction of the later flights before any error is computed; then it scores HPR Sim's carried fit against the plain prediction and against the by-hand ratio correction, for both programs, on new rows `accuracy.second-flight` and `bias.second-flight`.

[m2-16]: ../decisions-and-roadmap.md#m2-16
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
[trust]: ../trusting-a-simulation.md
[flatters]: ../trusting-a-simulation.md#which-miss-flatters
[accuracy]: ../accuracy.md
[collection]: ../accuracy.md#real-flights-of-the-private-collection
[era5]: ../format/era5.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[or-features]: https://openrocket.readthedocs.io/en/latest/introduction/features.html#planned-future-features
[planner]: https://github.com/Hide-In-Minion/hpr-launch-planner
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
flight. OpenRocket's guide lists importing altimeter data only as a planned feature
([OpenRocket's guide][or-features]), so the correction a flyer can make with it is by hand: scale
the next prediction by the last flight's logged apogee over its predicted one. The backlog has held
the idea since 2026-10-31 ([a second flight on the first flight's drag][idea]), with the row it
proposed, `accuracy.second-flight`.

HPR Sim already has the tool. Its [least-squares fit][fit] changes a drag scale, an impulse scale,
a constant wind and the mass until the simulated flight passes through a log's heights up to
apogee. On a real flight the simulator's own errors go into those scales. A drag scale fitted to
one flight is therefore a correction for that rocket. Nothing says yet whether it carries to the
same rocket's next flight, on another motor, another day and another loading.

The collection holds the test. Counted 2026-11-22 from its manifest and its catalog's rocket-name
and flyer fields, with no log opened (a rocket here is one name flown by one flyer):

- 83 flights in the manifest, 49 rockets; 16 rockets have two or more flights, 50 flights in all;
- 74 flights not marked excluded, in which the same 16 rockets hold 48 flights: 32 later flights,
  each with an earlier flight of its rocket to fit;
- 14 of those 16 rockets flew more than one motor;
- of the 74, those with no early deployment noted: 11 rockets with two or more flights, 32 flights,
  21 later flights.

Four hazards shape the rule. The later flights of one rocket aren't independent: the test rests
on 16 rockets, not 32 flights. On a log of heights alone, too much drag and too little impulse
look alike, so a fitted drag scale can hold a motor's shortfall that won't recur on another motor.
The fit's drag interval fails one of its synthetic tests ([ADR-368][adr-368], issue #1108), so the
milestone prints no interval from it. And a first number is easy to shade by which flights count as
one rocket, which earlier flight is fitted and which fitted value is carried. So the rule, the fits
and every prediction come first: each is merged, and its SHA-256 fingerprint recorded, before any
error is computed, as [M2.15a][m2-15a]'s are ([ADR-360][adr-360]).

**Decision.**

1. **M2.16, the next flight from the last one's log, in two increments.** M2.16 is met when both
   are.
   - **M2.16a, the rule, the fits and the predictions.** *Done when:* a pull request of its own
     merges the four below, and computes no error.
     - **One rocket.** The rule fixes, before any fit is run, when two flights are the same
       rocket: the same rocket name and flyer in the catalog, and design files whose outer shape
       matches (the outside parts in order, each one's length and diameter, and the fins'
       planform and count) within a tolerance the rule states. A pair whose shapes differ is kept,
       marked as changed, and counted apart; it is never dropped.
     - **The source flight.** A rocket's flights are ordered by launch time, by a tie rule the rule
       states for flights whose order isn't known. Each flight after the first is a later flight.
       It is predicted from the latest earlier flight of its rocket that has a usable fit: its
       source flight.
     - **The fit.** [M7.3][m7-3]'s least-squares fit, as it stands at M2.16a's merge, on the
       source flight's log up to apogee, with that flight's own plan, motor file and day's
       [ERA5 file][era5]. The rule fixes beforehand the drag scale's stated range on a real flight
       (the synthetic sets' 0.8 to 1.2 may be widened, never narrowed) and what makes a fit
       usable: the fit's own stop rule met, the drag scale inside its range rather than at an end,
       and a log the fit doesn't refuse. A fit that fails any of them is recorded with its cause.
       The fit reads a source flight's log whichever half of the collection it is in
       ([ADR-332][adr-332]): its scale is an input of that rocket's next flight, not a setting of
       the model. No part of the rule may be chosen from an error on any flight.
     - **What is carried.** The drag scale only. The impulse scale, the wind and the mass belong
       to the source flight's motor, day and loading: 14 of the 16 rockets flew more than one
       motor, and carrying the impulse would apply one motor's shortfall to another.
     - **The predictions.** Each later flight gets five: HPR Sim's apogee with the carried drag
       scale; HPR Sim's apogee with none, flown as the fixture report flies it; OpenRocket 24.12's
       apogee, from the oracle's run as the fixture report flies it; and, for each program, the
       hand correction, that program's plain apogee of the later flight times the source flight's
       logged apogee over the same program's plain apogee of the source flight. The source
       flight's logged apogee is read as the fixture report reads it. A prediction reads only the
       later flight's plan, the fields [ADR-333][adr-333] lets a prediction read, its design file,
       its motor file and the day's ERA5 file. Nothing comes from the later flight's log.
     - **The synthetic check.** Before it fits a real flight, the method is checked on synthetic
       pairs: one design flown on two motors, with drag differing from the simulator's by a known
       scale and each motor's impulse off its rating by its own scale. On a set of at least 200
       pairs, the carried drag scale must bring the second flight's apogee within a bound the rule
       fixes beforehand from the fit's stated widths, on at least 90% of the pairs. A probe that
       carries the impulse scale too must fail that on the pairs whose two motors' scales differ,
       and a probe that carries nothing must fail it on the pairs whose drag scale is away from 1.
     - **The record.** For each later flight one record holds: its id and its source flight's;
       the rocket, by an anonymous id; whether the shapes changed; the SHA-256 of both design
       files, both motor files and both ERA5 files; the commit that flew it; the fitted drag scale
       and the fit's stop state; each of the five predictions, or the cause it has none; and its
       cause if it is left out. A record is never written over an older one. No plan field is
       changed for this comparison.
     - **The seal.** As [M2.15a][m2-15a]'s: each record's SHA-256 merged in a list of its own,
       and a digest of the sorted ids beside it, both read at the merge base.
   - **M2.16b, the errors.** *Done when:*
     - The fixture report computes each error from the records [M2.16a][m2-16a] merged, and
       stops, comparing nothing, when a record's fingerprint or the sorted ids don't match the
       merge base's list. A figure from the model as it then stands may be shown beside, marked as
       not the prediction.
     - A later flight's error, for each prediction, is its apogee less the logged one, over the
       logged one, as the fixture report defines the apogee error.
     - The report gives, as aggregates, for each of the five predictions over the later flights
       that all five have, with the rest counted by cause:
       - the later flights and the rockets compared;
       - the mean absolute and the mean signed error over the flights, and over the rockets with
         each rocket's later flights averaged first, the latter with a 95% interval from a
         bootstrap that resamples rockets;
       - how many predictions are high and how many low;
       - the difference in absolute error between the carried fit and each of the other four,
         averaged over rockets, with a 95% interval from the same bootstrap, paired by flight.
     - It gives the same on the changed-shape pairs apart, on each half of the collection by the
       later flight's half ([ADR-332][adr-332]), and on the later flights whose motor is or isn't
       the source flight's, each beside the whole.
     - Every flight in the collection's manifest that isn't a later flight compared is counted by
       cause, so the compared and the not compared add up to the manifest.
     - The scoreboard gains `accuracy.second-flight`, the mean absolute error over the later
       flights, lower better, and `bias.second-flight`, the mean signed error, nearer zero
       better. HPR Sim's cell is the carried fit; OpenRocket 24.12's is its hand correction,
       citing the guide's planned-feature line as the reason it has no fit. The rows show HPR
       Sim's hand correction and both programs' plain figures beside them, not held, and the bias
       row names which side flatters which plan ([which miss flatters][flatters]).
     - The scoreboard's check holds HPR Sim's two cells by the calibration half's figures, as it
       holds `accuracy.collection` ([ADR-332][adr-332]); OpenRocket's are shown, not held.
     - A test pins which sign means high, on cases that include a prediction equal to its log,
       and a probe that flips the sign fails it.
     - [The guide to trusting a simulation][trust] and [the accuracy page][accuracy] state the
       measured errors, how many rockets and flights they rest on, the bootstrap intervals, and
       what the rows leave out. While fewer than 10 rockets are compared, both pages say the
       figures rest on too few rockets to rank the methods.
2. **Seventh in the queue,** after [M2.15][m2-15] and before [M2.3d4][m2-3d4], a stage's override
   that adds to its parts. It needs no reading those milestones add: the fit and the fixture
   report exist. It follows them because reviews add their promotions in the order made.
3. **What the rows can't say.** A carried drag scale holds every error that is the same on both
   flights of a rocket, not only the drag: the drag model, the finish, a design file's mass where
   both flights use it, a site's air. So the rows measure the correction, not the drag model. They
   say nothing of a rocket's first flight, which is what [the collection's rows][collection]
   score. A rocket that changed between its flights is counted apart, but a change the design
   files don't show, a repair or new paint, stays inside the error.
4. **Nothing else moves.** No tolerance changes, this record adds no check today (M2.16b's rows
   are held only once it lands), and no queued milestone's *done when* is changed. Issue #1108 is
   unchanged; M2.16 prints no interval from the fit's drag scale. The rule's correction is a
   per-rocket input, so [ADR-346][adr-346]'s rule for a correction of the model, judged on flights
   predicted first, doesn't apply to it; a later change of the model's defaults made from these
   fits would be such a correction ([ADR-348][adr-348]).

**Consequences.**

- A real-flight row that scores HPR Sim's fit to a log, the capability the scoreboard's
  `accuracy.from-boost` row says OpenRocket and RocketPy lack, on the task a flyer uses it for,
  is on the roadmap, with OpenRocket's best by-hand answer beside it.
- When it lands, the paired differences say, rocket by rocket, whether the fit carries better than
  a ratio, and whether either beats no correction.
- With 16 rockets, the intervals will be wide; the rows are honest only with them beside, so
  [M2.16b][m2-16b]'s *done when* asks for them.
- The review's new ideas are in [the backlog][ideas], under its date.

**How it was checked.** The counts were made 2026-11-22 from the manifest's flight ids, excluded
and early-deployment fields, and the catalog's rocket name, flyer and motor fields, grouping by name
and flyer; no log or design file was opened. The planner's tuning feature is in its README's
feature list and its "Suggest tuning" step. OpenRocket's planned-feature line is the one the
scoreboard's `accuracy.from-boost` row cites. What the fit fits, and the 0.8 to 1.2 range, are from
[the fit's page][fit]. The scoreboard has no `accuracy.second-flight` row and the fixture report
no carried-fit column, so both *done when*s fail today. `cargo test -p xtask` checks that the
queue's ids are open in the roadmap.
