# ADR-257: Calibrated intervals for the flight readings and fits, and the post-flight comparison (2026-10-13)

- **Status:** accepted; tightens [M7.2][m7-2] and [M7.3][m7-3]; no release moves and no milestone is added
- **Summary:** M7.2's "within the stated error" gains, and M7.3's "within tolerance" becomes, one test: on at least 1,000 synthetic flights on fixed seeds, each stated 90% interval must miss on each side about 5% of the time and hold the truth about 90%, judged by 99.9% Clopper–Pearson bands, and the test must fail when the intervals are narrowed, widened or shifted. M7.3 also reports, on the public logged flights and the collection, apogee predicted from the log up to burnout beside the forward simulation's, which fills a new scoreboard row, and draws the predicted and logged flight under launch-day conditions in one command, the analysis NASA Student Launch asks for after a team's flights.

[m7-2]: ../decisions-and-roadmap.md#m7-2
[m7-3]: ../decisions-and-roadmap.md#m7-3
[m10-2]: ../decisions-and-roadmap.md#m10-2
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-240]: 0240-the-scoreboard.md
[sl27]: https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf

**Context.** The flight analyzer ([release 0.2][m10-2]) is M7.1 to M7.3. Its two numeric
*done when* clauses had no number in them:

- M7.2: "the smoother recovers the truth within the stated error";
- M7.3: "synthetic-truth recovery is within tolerance, and real-flight fits are reported".

Either can be met by stating a wide error. An interval that is too wide misleads as much as one
too narrow: a flyer reading an apogee for a waiver, or a fitted drag coefficient for the next
flight, needs the stated range to mean what it says.

The users are named in a rule book. NASA Student Launch's 2027 handbook ([its text][sl27], posted
in August 2026) asks each team, after its subscale flight, to "compare the predicted flight model to
the actual flight data" and to estimate the full-scale rocket's drag coefficient from it (p. 26).
After the full-scale flight it asks them to "update your simulated flight model with launch day
condition data and compare the predicted flight performance to the actual flight data", and to
estimate the drag coefficient from launch data and "use this value to run a post-flight
simulation" (p. 32). Requirement 2.6.8.1 wants altitude and velocity against time, liftoff through
landing (p. 8). That is M7.3's fit and residuals.

A fit that only matches the flight it was fitted to proves little. The honest test is to fit the log up to burnout and predict the
apogee the log then records, and to compare that with the forward simulation on the same flights.

**Decision.**

1. **Every stated interval, tested both ways.** Every reading M7.2 doesn't withhold states a 90%
   interval (apogee, maximum velocity and acceleration, burnout, descent rates, flight time); so
   does each parameter M7.3 fits (drag coefficient scale, impulse scale, wind as one constant
   vector, and mass within its weighed uncertainty), for the least-squares fit and for the
   Bayesian option. M7.2's old clause stays beside
   the test: the smoothed trajectory recovers the truth within its stated error.
   - **The flights.** At least 1,000 synthetic flights, each drawing its design (from at least 10
     public designs), motor, wind, launch angle and sensor noise independently, on seeds fixed
     before any interval is tuned; a seed changed after a failure needs an ADR. Any factor that
     scales the intervals is fitted on other seeds. Two sets: one from the model the fit uses, and
     one from a generator that differs from it in named ways (drag against Mach number shaped
     differently, wind shear, a thrust curve shaped differently), where a parameter's truth is
     its fit to the noise-free flight. Truths are drawn from stated ranges, not from the Bayesian
     option's prior, so a fit that learns nothing from the log can't pass by construction.
   - **The test.** Three counts, each with a two-sided 99.9% Clopper–Pearson interval: flights
     whose truth lies above the stated interval and flights whose truth lies below it, each of
     which must contain 5%, and flights whose truth it holds, which must contain 90%. At 1,000
     flights a tail passes with 29 to 74 misses (2.9% to 7.4%). Testing each tail on its own
     catches an interval that misses on one side only. The 99.9% level is for the family of about
     14 intervals: in 400,000 simulated runs of 1,000 flights, a correctly stated interval passed
     99.8% of the time, and all 14 together about 97%. If a correct implementation still fails,
     the flight count doubles on new seeds fixed the same way, recorded in an ADR; seeds are never
     swapped one by one. The counts are also reported per design, so a miss on one design can't
     hide behind another's.
   - **Mutation probes.** The test must fail when every interval is narrowed to 0.8 of its width,
     when widened to 1.25 (for normal errors they then hold the truth about 81% and 96% of the
     time), and when every reading is shifted down by one stated standard error. In the same
     simulation the narrowed probe passed about 3 runs in a million and the others none; the fixed
     seeds pin that each fails.
   - **What a log can tell.** Thrust and drag act through their ratio to mass, so the fit takes
     the weighed mass, with its stated uncertainty, as an input (fitting mass only within that
     uncertainty) and fits the impulse and drag scales against it. A fit that ignores the log
     would pass the coverage test by construction, with intervals as wide as the prior's, so
     coverage alone doesn't count as a fit: a parameter counts as identified only when its median
     interval over the flights is at most half the width of the 90% interval its prior or stated
     range gives with no log. Otherwise it is reported as not identified by the log, and the
     drag and impulse scales must be identified on the same-model set. A probe feeds the fit pure
     noise in place of a log, and every parameter must come back as not identified.
2. **The apogee reading's error side.** M7.2 reports the apogee reading's mean error on the
   synthetic flights with its sign. A reading too low is the flattering side for a waiver
   ([ADR-144][adr-144]): the upper tail's count in §1 is that side's test, and the shifted-down
   probe its mutation probe.
3. **Apogee predicted from the boost.** The log is cut at burnout, found from the samples up to
   it, before any smoothing or fitting; a test replaces every later sample with noise, and the
   prediction stays bit-identical. M7.3's committed report gives the fits for the 7 public logged
   flights and, as aggregates only, for the collection's flights with a usable log. Beside the
   prediction's mean absolute error, and mean error with its sign, it gives the forward
   simulation's on the same flights, flown in the same launch-day air and wind. The public
   flights' difference gets an exact sign-flip interval, since a bootstrap undercovers at 7
   flights; the collection's gets a paired bootstrap 95% interval. This fills a new
   [scoreboard][adr-240] row, `accuracy.from-boost`. Each other tool's cell is read from its docs:
   a number if the tool can do it, otherwise *can't*. No bar is set on the difference: the report
   shows it, whichever way it falls.
4. **The post-flight comparison in one command.** One `hpr` command draws a flight's predicted and
   logged altitude and velocity against time, liftoff to landing. It draws the prediction twice:
   as designed, and under launch-day conditions. Those are the log's site and date with the
   weather source's wind and air, or values the user enters, so it works offline. A test runs it
   on a public flight. The plot carries the provenance M7.2 gave each reading.
5. **What doesn't change.** The order, the releases and the other clauses of both *done when*
   texts stay. Every old clause is kept or made stricter: M7.2's stated error stays and gains the
   coverage test beside it; M7.3's undefined tolerance becomes the coverage test. The command
   draws residuals M7.3 already reports. M7.1c4's entry drops two layouts M7.1c2 already reads
   (`.pfa`, AltimeterCloud; ADR-255) and keeps Entacore's unread AIM XTRA; its *done when* is
   unchanged.

**Consequences.** The analyzer's ranges become claims a test can refute, both ways. The new row
gives the scoreboard a cell no compared tool fills. A Student Launch team gets its post-flight
analysis from one command, and can check it: the report says how well a fit to the boost
predicted apogee on flights anyone can download.
