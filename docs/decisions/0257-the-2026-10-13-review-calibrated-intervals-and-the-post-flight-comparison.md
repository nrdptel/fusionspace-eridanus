# ADR-257: Calibrated intervals for the flight readings and fits, and the post-flight comparison (2026-10-13)

- **Status:** accepted; tightens [M7.2][m7-2] and [M7.3][m7-3]; no release moves and no milestone is added
- **Summary:** M7.2's "within the stated error" and M7.3's "within tolerance" become one test: on at least 200 seeded synthetic flights, a stated 90% interval must hold the truth in a share whose 95% Clopper–Pearson interval contains 90%, and the test must fail when the intervals are narrowed or widened. M7.3 also reports, on the public logged flights and the collection, apogee predicted from the log up to burnout beside the forward simulation's, which fills a new scoreboard row, and draws the predicted and logged flight under launch-day conditions in one command, the analysis NASA Student Launch asks for after a team's flights.

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

1. **One coverage test for every stated interval.** It runs on at least 200 seeded synthetic
   flights: simulated flights plus the noise models M7.2 documents. A stated 90% interval passes
   when the share of flights whose truth it holds has a two-sided 95% Clopper–Pearson interval
   that contains 90%. The test must fail when every interval is narrowed to 0.8 of its width and
   when widened to 1.25. For normal errors those widths hold the truth about 81% and 96% of the
   time. At 200 flights only a count from 171 to 188 passes, so both probes are expected to fail;
   on a fresh seed each would still pass by chance about 7% and 10% of the time, so the probes
   run on fixed seeds and the test pins that they fail. M7.2 applies it to each reading that states
   an interval (apogee, maximum velocity and acceleration, burnout, descent rates, flight time).
   M7.3 applies it to each fitted parameter (drag coefficient scale, mass, impulse scale, wind),
   for the least-squares fit and for the Bayesian option. M7.2's old clause stays beside the test:
   the smoothed trajectory recovers the truth within its stated error.
2. **The apogee reading's error side.** M7.2 reports the apogee reading's mean error on the
   synthetic flights with its sign. A reading too low is the flattering side for a waiver
   ([ADR-144][adr-144]), so a test pins which way it errs.
3. **Apogee predicted from the boost.** M7.3's committed report gives the fits for the 7 public
   logged flights and, as aggregates only, for the collection's flights with a usable log. It
   also gives the mean absolute error, and the mean error with its sign, of the apogee predicted
   by a fit to the log up to M7.2's burnout reading. Beside them are the forward simulation's on
   the same flights and a paired bootstrap 95% interval on the difference. This fills a new
   [scoreboard][adr-240] row, `accuracy.from-boost`. Each other tool's cell is read from its docs:
   a number if the tool can do it, otherwise *can't*. No bar is set on the difference: the
   report shows it, whichever way it falls.
4. **The post-flight comparison in one command.** One `hpr` command draws a flight's predicted and
   logged altitude and velocity against time, liftoff to landing. It draws the prediction twice:
   as designed, and under launch-day conditions. Those are the log's site and date with the
   weather source's wind and air, or values the user enters, so it works offline. A test runs it
   on a public flight. The plot carries the provenance M7.2 gave each reading.
5. **What doesn't change.** The order, the releases and the other clauses of both *done when*
   texts stay. Every old clause is kept or made stricter: M7.2's stated error stays and gains the
   coverage test beside it; M7.3's undefined tolerance becomes the coverage test. The command
   draws residuals M7.3 already reports. M7.1c4's entry drops three layouts M7.1c2 already reads
   (`.pfa`, Entacore AIM, AltimeterCloud; ADR-255); its *done when* is unchanged.

**Consequences.** The analyzer's ranges become claims a test can refute, both ways. The new row
gives the scoreboard a cell no compared tool fills. A Student Launch team gets its post-flight
analysis from one command, and can check it: the report says how well a fit to the boost
predicted apogee on flights anyone can download.
