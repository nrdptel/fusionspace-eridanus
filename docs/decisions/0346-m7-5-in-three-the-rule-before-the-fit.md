# ADR-346: M7.5 in three: the rule before the fit (2026-11-14)

- **Status:** accepted; splits [M7.5][m7-5] into [M7.5a][m7-5a], [M7.5b][m7-5b] and [M7.5c][m7-5c], each with its own *done when*, M7.5's *done when* whole across the three, and so plans every remaining increment of M7.5; fixes, before any correction is fitted across flights or flown, the correction's form, its fit on the calibration half, the flag for a flight whose log and files disagree, and the test with its pass rule, under [ADR-209][adr-209] §6, [ADR-322][adr-322] §2 and [ADR-332][adr-332] §2; carries out M7.5a.
- **Summary:** The correction of the bias the simulators share is to be two scales: one on the motors' impulse, pooled from what the calibration half's logs alone say of the boost, and one on the drag, from those flights' coasts. Each enters only if its 95% interval excludes 1. It ships only if it lowers the mean absolute error and the absolute bias on the held-out half and makes neither worse on the public flights, with and without the flights whose logs and files disagree by more than a certified motor may. This record is merged before that fit is run. Its own increment splits each half's forward error at the boost's cut and reads the calibration half's logs alone.

[m7-5]: ../decisions-and-roadmap.md#m7-5
[m7-5a]: ../decisions-and-roadmap.md#m7-5a
[m7-5b]: ../decisions-and-roadmap.md#m7-5b
[m7-5c]: ../decisions-and-roadmap.md#m7-5c
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

What the reports say on `main` at dd2cb148:

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
Each flight's error is to be split into boost and coast, since the boost report's numbers say
the bias is made in the boost, by the mass or the motor, not by the coast's drag. (c) Flights
whose log disagrees with their files on mass or impulse are to be flagged, and the results
given with and without them; a correction must hold on both. The worry behind (c) is that
some design files don't match the rocket that flew.

**What was looked at before this record.** The calibration half's own boost fits, one flight
at a time: their impulse and drag scales, standard errors and errors, and the same fits made
again with the impulse's stated range widened 10-fold and 100-fold. ADR-322 §2 lets the
correction be chosen on that half. Not looked at: any flight flown with a correction, on
either half or on the public flights; the held-out half's fits, one by one or as the sealed
aggregates; a widened fit of any held-out or public flight.

What that look showed, as the [boost report][boost] now gives it in aggregate:

- **The fits' impulse scales are pulled toward 1.** Each fit is told the motor delivers its
  rated impulse to a standard deviation of 0.0328 (a 90% range of 1 give or take 0.054). The
  calibration fits' impulse scales average 0.958 with a standard deviation of 0.033, as wide
  as what they were told, so the stated range weighs about as much as the log in each.
  Averaging them counts that pull 12 times.
- **The logs alone say more.** Fitted again with the impulse's range 100 times wider, the 12
  calibration logs give impulse scales averaging 0.936 (standard deviation 0.048), all 12
  below 1.
- **The boost says almost nothing of the drag.** Those fits know the drag scale to 0.108 on
  average, against the 0.109 they were told. A drag scale averaged from boost fits would be
  the stated value handed back.
- **A boost can't tell impulse from mass.** A barometer's climb gives acceleration, which is
  thrust less drag over mass, so less impulse and more mass read the same
  ([the identifiability note][identifiability]). The fit is told the structure's mass to
  within 2% of the file's, so its impulse scale carries both.

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
   up to the cut, where thrust and mass set the climb. Both are in percentage points of the
   logged apogee, and they add up to the forward error. The boost report gives, for each half
   and for the flights without a deployment before apogee, the mean share up to the cut with
   its sign, its mean size, and how many flights have it as the larger share. The fit also
   moves the drag, the mass and the wind within what it was told, and those carry into the
   coast, so this is a split of two simulations' errors, not a measurement of the motor.
2. **The log alone.** For what a flight's log says of its boost without the stated range's
   pull, the boost's fit is made once more with the impulse scale's stated standard deviation
   multiplied by 100 and nothing else changed. Its impulse scale $k_i$ and standard error
   $s_i$ are the flight's *boost scale*: the impulse the climb to the cut shows over the rated
   impulse, at the file's mass. The widened range's standard deviation is 3.28, a weight of
   $1/3.28^2 \approx 0.09$ beside a log that knows the scale to 0.1 (a weight of 100), so its
   pull is under a thousandth.
3. **The flag.** A flight is flagged when its log and its files disagree by more than a
   certified motor may: $|k_i - 1| - 2 s_i > 0.131$, in a widened fit that converged. NAR
   tests a motor again as a new type when one recertification firing's total impulse is more
   than 13.1% from the certified mean (NAR Motor Testing Manual v1.5, 2011, §6.2.3, p. 11;
   [the lots note][lots] §1), so a boost farther than that from the rating, by more than its
   own uncertainty, points at the file's mass or motor and not at the lot. The two standard
   errors keep a log that says little from being flagged. A flight with no boost prediction
   can't be flagged, and stays in every set. The flag is not a verdict on a flight: it marks
   which flights the test is run without.
4. **The correction: two scales, each fitted where the log can tell it.**
   - **The impulse scale $k_I$,** from the boosts. Over the calibration flights with a
     converged widened fit, $k_I = \sum w_i k_i / \sum w_i$ with $w_i = 1/(s_i^2 + \tau^2)$
     and $\tau = 0.0328$, the standard deviation the fit states of one motor's impulse. A
     flight's scale differs from the shared one by its own motor and build ($\tau$) and by
     its fit's error ($s_i$), so its variance about the shared scale is the sum, and the
     weights are that variance's inverse. The calibration scales' own spread agrees with that
     $\tau$: $\sqrt{0.048^2 - 0.035^2} \approx 0.033$, taking the fits' averaged standard
     error for each. Its 95% interval is $k_I \pm t/\sqrt{\sum w_i}$, with $t$ the 97.5%
     point of Student's $t$ at one less than the number of flights.
   - **The drag scale $k_D$,** from the coasts. For each calibration flight in the set
     without a deployment before apogee, $d_i$ is the drag scale at which the widened fit's
     flight, every other fitted value held, reaches the logged apogee the errors are taken
     against, searched from 0.25 to 4; a flight with no such scale there gives none and is
     counted. $k_D$ is the mean of the $d_i$, and its 95% interval is that mean give or take
     $t$ times their standard deviation over the square root of their number. The boost
     knows the drag no better than it was told (0.108 against 0.109), so changing the drag
     moves the fitted boost by less than the log can see, and the scale found is the coast's.
   - **A scale enters only if its interval excludes 1,** and needs at least 3 flights.
     Otherwise it stays 1 and the report says so. If neither enters there is no correction,
     and the verdict is a fail.
   - **How it is applied.** A corrected flight is the forward report's flight with every
     motor's thrust multiplied by $k_I$ at its rated burn time and the drag multiplied by
     $k_D$: the same `.ork`, configuration, rail and ERA5 air, nothing else changed. With
     both scales at 1 it must land within 0.5 m of the forward apogee, as the boost report's
     model already must.
   - **Two fits.** One over every calibration flight with a widened fit, and one without the
     flagged ones. The drag scale's flights follow the same two sets.
   - **Through the guard.** Both fits take their flights through
     `collection_boost::Calibration`, which reads each flight's half from its id again and
     refuses a held-out one ([ADR-332][adr-332] §2). A corrected flight takes its scales
     only from those fits.
5. **The test.** Each fit is judged on three sets: the held-out flights the forward report
   compares (23 today), those of them without a deployment before apogee, and the public
   flights the [public report][public] compares (12 today). The fit without the flagged
   flights is judged on those sets less their flagged flights, a public flight's flag coming
   from the widened fit of its own boost where the [public boost report][from-boost] cuts
   its log. On each set, the corrected flights' mean absolute apogee error and the absolute
   value of their mean error are set against the forward simulation's on the same flights.
   - **Pass:** on both held-out sets, the mean absolute error and the absolute bias are each
     lower than the forward simulation's; on the public flights, neither is higher. Both
     fits must pass on all three of their sets. Anything else is a fail.
   - **Beside it, not in the rule:** each set's paired difference of absolute errors with its
     bootstrap interval (10,000 resamples, seed 346) and exact sign-flip interval, as
     [ADR-329][adr-329] gives them; how many flights' errors grow; and the calibration
     half's corrected errors, marked as the flights the correction was fitted on.
   - **A deployment before apogee** lowers the logged apogee beneath any ballistic flight,
     so a correction that lowers apogees gains on those flights for the wrong reason. That
     is why the set without them must pass too.
6. **One test, once.** The held-out half judges this correction and no other. No scale, set,
   weight or limit above changes after a corrected flight is flown. The numbers beside the
   rule choose nothing. After M7.5b the held-out half has been seen against a correction, so
   a later correction of another form is a new decision judged on flights predicted first
   ([M2.7][m2-7]'s second increment), not on this half again.
7. **Small sets.** The collection's terms allow aggregates only ([ADR-151][adr-151]). A set
   that differs from a published one by fewer than 3 flights would give those flights' own
   errors by subtraction. So where fewer than 3 of a set's flights are flagged, the report
   gives the count and the pass or fail of the test without them, and keeps that set's
   numbers in the private audit file that `fixture-flights --check` reproduces. The public
   flights' numbers are given either way.
8. **What ships on a pass (M7.5c).** The fit without the flagged flights: a flagged flight's
   scale measures its files, not the bias the simulators share. The corrected apogee stands
   beside the uncorrected one in each flight's output, never in its place, named with its
   two scales and the number of flights behind them. It is built in, so it works offline. A
   correction that lowers an apogee errs toward the flattering side for a waiver or a
   field's ceiling, so every check against a ceiling takes the higher of the two apogees: a
   test pins that, with a mutation probe. The output calls the impulse scale what it is: the
   boost's shortfall, from the motor, the as-built mass, or both.
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
     matching its committed hash; a decision record gives the verdict. On a fail it records
     that nothing ships, and M7.5 closes with M7.5c dropped.
   - **M7.5c The correction in each flight's output.** Only after a pass. *Done when:* the
     correction ships named in each flight's output as decision 8 says, in the command
     line's flight summary, the Python package's result and the library's; the ceiling
     test and its mutation probe pass; the site's page says what it is, what it was judged
     on and how far to trust it.

**What doesn't change.** M7.5's *done when* is whole across the three: the fit, the test and
the publication are M7.5b's, the shipping M7.5c's, and "a decision record says that it
doesn't, and nothing ships" is M7.5b's fail. No clause is narrowed: decision 5 asks more than
ADR-209 §6 does (two held-out sets, the public flights, both fits). The held-out digest, the
sealed aggregates and their hash stand; M7.5a's change to the report leaves the sealed file
byte for byte as it was. [ADR-209][adr-209] §6 names the mass as built beside drag and
impulse: a boost can't tell it from impulse, so the impulse scale carries it and says so.

**Consequences.**

- The correction is fitted on boosts that stay under 227.7 m/s, Mach 0.8 in the coldest air,
  where the cut's finder stops: the core band's subsonic part. The test flies it on every
  flight compared, faster ones included, and the report says so.
- The public flights read low where the collection reads high. A correction that lowers
  apogees can pass on the held-out half and still fail on the public flights. The rule then
  says nothing ships, and what M7.5b measured stays on the report for the work on causes
  (M2.3c6, M1.14): how much of the error is made up to the cut.
- An impulse scale standing in for mass predicts a lower coast than a heavier rocket would
  fly, since a heavier rocket carries its speed farther. If the shortfall is mostly mass,
  the corrected apogees come out low, and the test's bias clause sees that.
- `fixture-flights` fits each calibration flight predicted a second time, 12 more fits
  today.

**How it was checked.** M7.5a's *done when* fails on `main` at dd2cb148: no record fixes the
correction, and the boost report has neither the split nor the logs alone. M7.5b's and
M7.5c's fail there too: no correction is fitted, the held-out aggregates are sealed, and no
flight's output names one. On the branch, `cargo xtask fixture-flights` writes the report
with the new rows, `cargo xtask fixture-flights --check` reproduces it, and
`validation/reports/held-out-fits.sha256` is unchanged: the sealed file's SHA-256 is still
`608ab0bd…85920`.
