# ADR-346: M7.5 in three: the rule before the fit (2026-11-14)

- **Status:** accepted; splits [M7.5][m7-5] into [M7.5a][m7-5a], [M7.5b][m7-5b] and [M7.5c][m7-5c], each with its own *done when*, M7.5's *done when* whole across the three, and so plans every remaining increment of M7.5; fixes, before any correction is fitted across flights or flown, the correction's form, its fit on the calibration half, the flag for a flight whose log and files disagree, and the test with its pass rule, under [ADR-209][adr-209] §6, [ADR-322][adr-322] §2 and [ADR-332][adr-332] §2; carries out M7.5a.
- **Summary:** The correction of the bias the simulators share is two scales: one on the motors' impulse, pooled from what the calibration half's logs alone say of the boost, and one on the drag, from those flights' coasts. Each enters only if its 95% interval excludes 1. It ships only if it lowers the mean absolute error and the absolute bias on the held-out half and makes neither worse on the public flights, with and without the flights whose logs and files disagree by more than NAR's retest rule allows. This record is merged before that fit is run. Its own increment splits each half's forward error at the boost's cut and reads the calibration half's logs alone.

[m7-5]: ../decisions-and-roadmap.md#m7-5
[m7-5a]: ../decisions-and-roadmap.md#m7-5a
[m7-5b]: ../decisions-and-roadmap.md#m7-5b
[m7-5c]: ../decisions-and-roadmap.md#m7-5
[m2-7]: ../decisions-and-roadmap.md#m2-7
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-329]: 0329-m7-3d6-the-collections-errors-by-halves.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[boost]: ../../validation/reports/collection-boost.md
[forward]: ../../validation/reports/fixture-flights.md
[public]: ../../validation/reports/real-flights.md
[from-boost]: ../../validation/reports/from-boost.md
[identifiability]: ../research/drag-impulse-identifiability.md
[lots]: ../research/hpr-lot-impulse.md

**Context.** M7.5's *done when*: a correction fitted on the calibration flights' logs, for
airframe drag and delivered impulse, lowers the held-out mean absolute error and bias, and
ships named in each flight's output; or a decision record says that it doesn't, and nothing
ships. Either way, chosen first, the sealed held-out fit aggregates are then published, their
file matching its committed hash ([ADR-322][adr-322] §2).

What the reports say on `main` at commit dd2cb148:

- **The collection reads high, and more so in the held-out half.** HPR Sim's mean apogee error
  is +4.22% over the calibration half's 37 flights (mean absolute 9.42%) and +13.36% over the
  held-out half's 23 (16.26%) ([the forward report][forward]).
- **The public flights read low.** Over the 12 public flights the mean error is −4.91% and
  the mean absolute error 9.00% ([the public flights' report][public]).
- **A prediction from the boost lands close.** With the log fitted up to a cut after burnout
  and flown on, the mean absolute error is 3.19% on the calibration half's 12 predicted flights
  and 3.60% on the held-out half's 9, where the forward simulation's is 9.32% and 16.75% on the
  same flights ([the boost report][boost]).

The maintainer asked three things of this milestone on 2026-11-12, and that a decision record
fix them before any fit. (a) The correction must be no worse on the 12 public flights. (b)
Each flight's error is to be split into boost and coast: a prediction from the boost misses
by 3% to 4% where the forward simulation misses by 9% to 17% on the same flights, which
points at the boost, at the mass or the motor, before the coast's drag. (c) Flights
whose log disagrees with their files on mass or impulse are to be flagged, and the results
given with and without them; a correction must hold on both. The worry behind (c) is that
some design files don't match the rocket that flew.

**What was looked at before this record.** The calibration half's own boost fits, one flight
at a time: their impulse and drag scales, standard errors and errors, and the same fits made
again with the impulse's stated range widened 100-fold, as the report now gives them.
ADR-322 §2 lets the correction be chosen on that half. Not looked at: any flight flown with
a correction, on either half or on the public flights; the held-out half's fits, one by one
or as the sealed aggregates; a widened fit of any held-out or public flight. Two things were
computed outside the committed code, and no number was kept from either. The widened fits
were first made 10-fold as well as 100-fold, to see that the scales had stopped moving with
the range; 100 was kept, for decision 2's arithmetic. And a check of this record's wording
took a weighted mean of the calibration scales once, to test a sentence below.

What that look showed, as the [boost report][boost] now gives it in aggregate:

- **The fits' impulse scales are pulled toward 1.** Each fit is told the motor delivers its
  rated impulse to a standard deviation of 0.0328 (a 90% range of 1 give or take 0.054). The
  calibration fits' impulse scales average 0.958, where the same logs alone give 0.936
  (next): the stated range holds back about a third of what the logs say. Averaging those
  fits would count that pull 12 times.
- **The logs alone say more.** Fitted again with the impulse's range 100 times wider, the 12
  calibration logs give impulse scales averaging 0.936 (standard deviation 0.048, the mean's
  standard error 0.014), all 12 below 1. That is 12 flights of one half of one collection.
- **The boost says almost nothing of the drag by itself.** Those fits know the drag scale to
  0.108 on average, against the 0.109 they were told, with the impulse free to move beside
  it. A drag scale averaged from boost fits would be the stated value handed back.
- **A boost can't tell impulse from mass.** A barometer's climb gives acceleration, which is
  thrust less drag over mass, so less impulse and more mass read the same
  ([the identifiability note][identifiability]). The fit is told the structure's mass to
  within 2% of the file's, so its impulse scale carries both, and whatever the model gets
  wrong in the boost.

What is left of M7.5, surveyed: the rule (this record); two measurements that need no fit
across flights (the error split at the cut, and the logs alone with the flag); the fit across
the calibration half; the corrected flights of the held-out half and of the public flights;
the verdict and the sealed aggregates' publication; and, only on a pass, the correction in
each flight's output across the command line, the Python package and the library. The last is
a user-facing change to every flight result with a safety side: a correction that lowers a
predicted apogee errs toward a waiver that is too low. That is more than one session, and the
rule has to reach `main` before the fit it governs.

**Decision.**

1. **The forward error is split at the cut.** For each flight with a prediction from the
   boost, the prediction's error is the share made after the cut: the log is followed up to
   the cut and the model from there. The forward error less the prediction's is the share made
   up to the cut, where thrust, mass and the boost's own drag set the climb. Both are in
   percentage points of the logged apogee, and they add up to the forward error. The boost
   report gives, for each half and for the flights without a deployment before apogee, the
   mean share up to the cut with its sign, and how many flights have it as the larger share.
   It gives no mean of the share's size: beside the signed mean, that would give the sum of a
   set's negative shares, one flight's own value where a single flight has one (decision 7).
   The fit also moves the drag, the mass and the wind within what it was told, and those
   carry into the coast, so this is a split of two simulations' errors, not a measurement of
   the motor.
2. **The log alone.** For what a flight's log says of its boost without the stated range's
   pull, the boost's fit is made once more with the impulse scale's stated standard deviation
   multiplied by 100 and nothing else changed: it is still told the drag is 1 to a standard
   deviation of 0.109. Its impulse scale $k_i$ and standard error $s_i$ are what the log
   alone says: the impulse the climb to the cut shows over the rated impulse, at the file's
   mass and the model's drag. The widened range's standard deviation is 3.28, a weight of
   $1/3.28^2 \approx 0.09$. A fit with the calibration half's average standard error, 0.035,
   weighs $1/0.035^2 \approx 800$, so the range's pull is about a ten-thousandth of the
   log's. A fit counts as converged where the fit says so (`Fit::converged`).
3. **The flag.** A flight is flagged when its log and its files disagree by more than NAR's
   retest rule allows one firing: $|k_i - 1| - 2 s_i > 0.131$, in decision 2's fit, converged.
   NAR tests a motor again as a new type when one recertification firing's total impulse is
   more than 13.1% from the certified mean (NAR Motor Testing Manual v1.5, 2011, §6.2.3,
   p. 11; [the lots note][lots] §1). The manual says that of a test firing, measured from the
   certified mean. Taking it as the most a flown motor's lot strays from its file's curve,
   whoever certified the motor, is this record's inference; the repeat tests that note found
   ran from −8.5% to +9.5%. A boost farther than 13.1% from the rating, by more than its own
   uncertainty, is then unlikely to be the lot: the file's mass or motor, or the model of the
   boost, is the likelier miss. The two standard errors keep a log that says little from
   being flagged. The flag reads one flight's own fit and no other flight's. It is measured
   from the rating, not from a scale the flights share, so where they share a shortfall it
   takes low scales more readily than high ones, and a fit without the flagged flights leans
   toward 1, the side of a smaller correction. A flight with no boost prediction can't be
   flagged, and stays in every set. The flag is not a verdict on a flight: it marks which
   flights the test is also run without.
4. **The correction: two scales, each fitted where the log can tell it, the drag first.**
   - **The drag scale $k_D$,** from the coasts. In a boost the drag and the impulse trade
     against each other. Even over a whole log, liftoff to apogee, they correlate at +0.976
     in [the identifiability note][identifiability]'s example, and a boost alone says almost
     nothing of the drag, as above. So the drag can't be changed under a fitted boost with
     the impulse held: the boost would leave the log. So, for each calibration flight in
     the fit's set without a deployment before apogee and for a trial drag scale $d$, the
     boost is fitted again as in decision 2 but told the drag is $d$, to a thousandth of its
     stated standard deviation, and that fit is flown on to apogee. The impulse takes up the
     boost; the coast flies the drag $d$. The flight's $d_i$ is the $d$ at which that apogee
     is the logged apogee the errors are taken against, found by bisection from 0.25 and 4
     until the two ends are within 0.001. The apogee falls as $d$ rises, so the bisection
     finds a $d$ wherever the two ends bracket the logged apogee. A flight has none if its
     apogees at 0.25 and at 4 lie on the same side of the logged one, or if a fit along the
     way doesn't converge. $k_D$ is the mean
     of the $d_i$, and its 95% interval is that mean give or take $t$ times their standard
     deviation over the square root of their number $n$, with $t$ the 97.5% point of
     Student's $t$ at $n - 1$.
   - **The drag scale enters** only if every flight of that set has a $d_i$ (a flight left
     out would be a censored one), there are at least 3, and the interval excludes 1.
     Otherwise it stays 1 and the report says why.
   - **The impulse scale $k_I$,** from the boosts, at that drag. Each calibration flight in
     the fit's set is fitted as in decision 2, told the drag is $k_D$ to its stated standard
     deviation; where $k_D$ stays 1 these are decision 2's fits. Over the converged ones,
     $k_I = \sum w_i k_i / \sum w_i$ with $w_i = 1/(s_i^2 + \tau^2)$ and $\tau = 0.0328$,
     the standard deviation the fit states of one motor's impulse. A flight's scale differs
     from the shared one by its own motor and build ($\tau$) and by its fit's error ($s_i$),
     so its variance about the shared scale is the sum, and the weights are that variance's
     inverse. The calibration scales' own spread is near that $\tau$ on a rough check:
     $\sqrt{0.048^2 - 0.035^2} \approx 0.033$, with the fits' averaged standard error
     standing in for each fit's own. The interval takes its width from the scales' own spread, not from $\tau$: it is
     $k_I \pm t \sqrt{\sum w_i (k_i - k_I)^2 / ((n - 1) \sum w_i)}$, the weighted spread of
     the scales about their mean, with $t$ at $n - 1$ for $n$ fits. M7.5b reads and cites
     that estimator's source when it builds it.
   - **The impulse scale enters** only if there are at least 3 fits and the interval
     excludes 1. Otherwise it stays 1. If neither scale enters there is no correction, and
     the verdict is a fail.
   - **Two decimals.** A scale that enters is rounded to two decimals, a tie going up, the
     drag scale before the impulse fits are told it and both before any corrected flight is
     flown. A scale that rounds to 1.00 has not entered. The rounded scales are the correction, in the test
     and in what ships. The logs-alone mean's standard error is 0.014 today, so a third
     decimal would say nothing, and decision 7 needs the rounding.
   - **How it is applied.** A corrected flight is the forward report's flight changed
     through the same functions the fit's model uses: every motor through
     `dispersed_motor(motor, k_I, 1.0)`, which multiplies the thrust and the propellant's
     mass by $k_I$ at the rated burn time, and the drag multiplied by $k_D$
     (`with_drag_scale`). The `.ork`, configuration, rail and ERA5 air are the forward
     report's. With both scales at 1 it must land within 0.5 m of the forward apogee, as the
     boost report's model already must. Every motor of a flight is scaled: one lit after the
     cut, an upper stage's or a cluster's, though no boost measured it, and one that isn't
     certified, since a correction in a flight's output wouldn't know. A flight with staging
     or more than one motor enters the fits as any other.
   - **Measured in one model, applied in another.** The fit's model flies a wind constant
     with height and its own burn time and mass; the corrected flight flies ERA5's wind, the
     rated burn time and the file's mass. The scales are carried across, and the test on
     flights the fit never saw is what says whether that holds.
   - **Two fits.** One over every calibration flight, and one without the flagged ones. Each
     runs this whole decision on its own flights.
   - **Through the guard.** Both fits take their flights through
     `collection_boost::Calibration`, which reads each flight's half from its id again and
     refuses a held-out one ([ADR-332][adr-332] §2). A corrected flight takes its scales
     only from those fits.
5. **The test.** Each fit is judged on six sets: the held-out flights the forward report
   compares, those of them without a deployment before apogee, the public flights the
   [public report][public] compares, and each of the three again less its flagged flights.
   A held-out flight's flag comes from decision 2's fit of its own boost, and a public
   flight's likewise where the [public boost report][from-boost] cuts its log. Those fits
   are made in M7.5b's run of the test, for the flag alone. None enters decision 4's fits,
   which the guard refuses, and until then the report makes none. On each set,
   the corrected flights' mean absolute apogee error and the absolute value of their mean
   error are set against the forward simulation's on the same flights.
   - **Pass:** on the four held-out sets, the mean absolute error and the absolute bias are
     each lower than the forward simulation's; on the two public sets, neither is higher,
     and equal passes. Both fits must pass on all six sets. Anything else is a fail, and the
     verdict's record names each clause that failed.
   - **The sets are fixed by the reports,** not by M7.5b. They are the flights the two
     reports compare at the head of `main` when M7.5b's branch is cut: 23 held-out flights and 12
     public ones today. M7.5b's record names that head's commit and each set's count. Its pull
     request changes neither the forward simulation nor which flights a report compares.
   - **A public flight is flown as its own report flies it:** its design, its air and its
     motor as they are there, with only the motors and the drag changed as decision 4 says.
   - **As computed.** The comparisons are of the numbers as computed, not as a page rounds
     them. A corrected flight that fails to fly fails every set it is in.
   - **Beside it, not in the rule:** each set's paired difference of absolute errors with its
     bootstrap interval (10,000 resamples, seed 346) and exact sign-flip interval, as
     [ADR-329][adr-329] gives them; how many flights' errors grow; and the calibration
     half's corrected errors, marked as the flights the correction was fitted on.
   - **A deployment before apogee** lowers the logged apogee beneath any ballistic flight,
     so a correction that lowers apogees gains on those flights for the wrong reason. That
     is why the sets without them must pass too.
   - **Why both fits face every set.** A held-out flight's flag comes from its own log. If
     the fit without the flagged flights were judged only on sets less their flagged
     flights, a held-out flight the correction hurts could flag itself out of the test.
6. **One test, once.** The held-out half judges this correction and no other. No scale, set,
   weight or limit above changes after a corrected flight is flown. The numbers beside the
   rule choose nothing. If a defect is found in the correction's code after a corrected
   flight has been flown, everything is run again and the verdict's record lists every run
   and what changed between them. After M7.5b the held-out half has been seen against a
   correction, so a later correction of another form is a new decision judged on flights
   predicted first ([M2.7][m2-7]'s second increment), not on this half again.
7. **Small sets.** The collection's terms allow aggregates only ([ADR-151][adr-151]). Two
   published numbers over sets that differ by fewer than 3 flights can give those flights'
   own values by subtraction. That holds for a set's errors and for the fits themselves: two
   weighted means with their intervals give the weight and the scale of a flight only one
   of them holds. This increment's table of the logs alone is over every calibration fit,
   and it counts as published. So:
   - where fewer than 3 calibration flights are flagged, the report gives, of the fit
     without the flagged flights, its scales as flown (decision 4's two decimals), its
     counts of flights and whether each interval excludes 1. Of the other fit it gives only
     the pass or fail on each set. Neither fit's unrounded scale nor its interval is given:
     beside this increment's table they would give a flagged flight's scale. Where 3 or
     more are flagged, both fits' scales and intervals are given;
   - where fewer than 3 of a collection set's flights are flagged, the report gives that
     set's numbers with every flight, and of the set less its flagged flights only the count
     and the pass or fail.

   What is held back stays in a private file whose SHA-256 is committed and held by
   `fixture-flights --check`, as the held-out aggregates' is. The public flights' numbers
   are given either way.

   **What this increment's table already gives.** A mean and a spread over 12 fits bound
   each of them, and the count of flagged flights with the flag's rule bounds a flagged
   one from the other side. With 1 of 12 flagged, that places its scale to within about
   0.04 either way. It is a fitted scale of a flight the page doesn't name. It gives no
   name, size or mass and rebuilds no design, which is what the terms guard against, so
   the table stays. M7.5b adds little or nothing to that: a scale rounded to two decimals
   over 11 flights, set against the table's mean over 12, places the flagged scale only to
   about 0.06 either way, and more loosely where that fit's weights are uneven.
8. **What ships on a pass (M7.5c).** The fit without the flagged flights, which by
   decision 5 has passed on every set, the full ones included: a flagged flight's scale
   measures its files, not the bias the simulators share. The corrected apogee stands
   beside the uncorrected one in each flight's output, never in its place, named with its
   two scales and the number of flights behind them. It is built in, so it works offline.
   Only the apogee is given corrected. No other output reads the corrected flight: the
   drift and landing area, a delay's choice, the flutter margin and the descent stay the
   uncorrected flight's. Where the correction lowers and slows a flight, a corrected flight
   would flatter each of those. Where either scale is on the raising side (a drag scale
   below 1, or an impulse scale above 1), the uncorrected flight can be the flattering one,
   and M7.5c then gives each of those outputs from the less favourable of the two flights. A correction that
   lowers an apogee errs toward the flattering side for a waiver or a field's ceiling, so
   every check against a ceiling takes the higher of the two apogees: a test pins that, with
   a mutation probe. The output calls each scale what it is: the impulse scale is the
   boost's shortfall, from the motor, the as-built mass or the model of the boost; the drag
   scale is the coast's, and carries what the as-built mass does to the coast too.
9. **The increments.**
   - **M7.5a The rule before the fit.** *Done when:* this record, merged before any
     correction is fitted across flights or flown, fixes the correction's form, its fit on
     the calibration half, the flag and the test with its pass rule (decisions 2 to 7);
     `cargo xtask fixture-flights` gives each half's forward error split at the cut, and
     what the calibration half's logs alone say of the boost with its flagged flights
     counted; `fixture-flights --check` holds them, the sealed held-out aggregates' hash
     unchanged.
   - **M7.5b The correction, fitted and judged.** *Done when:* by decisions 2 to 7,
     unchanged, the correction is fitted on the calibration half through the guard and flown
     on the held-out half and the public flights, with and without the flagged flights; the
     boost report gives each number decision 5 names and the verdict, and `fixture-flights
     --check` holds them; then the sealed held-out fit aggregates are published, their file
     matching its committed hash; a decision record gives the verdict and, on a fail, each
     clause that failed. On a fail it records that nothing ships, and M7.5 closes with
     M7.5c dropped.
   - **M7.5c The correction in each flight's output.** Only after a pass. *Done when:* the
     correction ships named in each flight's output as decision 8 says, in the command
     line's flight summary, the Python package's result and the library's; the ceiling
     test and its mutation probe pass; the site's page says what it is, what it was judged
     on and how far to trust it.

**What doesn't change.** M7.5's *done when* is whole across the three: the fit, the test and
the publication are M7.5b's, the shipping M7.5c's, and "a decision record says that it
doesn't, and nothing ships" is M7.5b's fail. No clause is narrowed: decision 5 asks more than
ADR-209 §6 does (two held-out sets, the public flights, both fits, each with and without the
flagged flights). The held-out digest, the sealed aggregates and their hash stand; M7.5a's
change to the report leaves the sealed file byte for byte as it was. [ADR-209][adr-209] §6
names the mass as built beside drag and impulse: a boost can't tell it from impulse, so the
impulse scale carries it and says so.

**Consequences.**

- The correction is fitted on boosts that stay under 227.7 m/s, Mach 0.8 in the coldest air,
  where the cut's finder stops: the core band's subsonic part. The test flies it on every
  flight compared, faster ones included, and the report says so.
- The public flights read low where the collection reads high. A correction that lowers
  apogees can pass on the held-out half and still fail on the public flights. The rule then
  says nothing ships, and what M7.5b measured stays on the report for the work on causes
  (the gap to OpenRocket by cause, M2.3c6, and the accuracy work inside the envelope,
  M1.14): how much of the error is made up to the cut.
- An impulse scale standing in for mass predicts a lower coast than a heavier rocket would
  fly, since a heavier rocket carries its speed farther. On the calibration flights the
  drag scale takes that up, so a drag scale below 1 can be mass and not drag.
- `fixture-flights` fits each calibration flight predicted a second time, 12 more fits
  today. M7.5b's drag scale costs a fit for each step of each coast's bisection.

**How it was checked.** M7.5a's *done when* fails on `main` at commit dd2cb148: no record
fixes the correction, and the boost report has neither the split nor the logs alone. M7.5b's
and M7.5c's fail there too: no correction is fitted, the held-out aggregates are sealed, and
no flight's output names one. On the branch, `cargo xtask fixture-flights` writes the report
with the new rows, `cargo xtask fixture-flights --check` reproduces it, and
`validation/reports/held-out-fits.sha256` is unchanged: the sealed file's SHA-256 is still
`608ab0bd…85920`.
