# ADR-368: The fit's drag interval fails set B's test on the default flight: recorded, not fitted away (2026-11-22)

- **Status:** accepted; changes what `cargo xtask fit` holds for one cell of [ADR-257][adr-257] §1's test, the least-squares drag scale on set B, from passing to failing as recorded; changes no share, band, count or flight
- **Summary:** With the tip-off phase the default ([ADR-367][adr-367]), the least-squares fit's 90% interval on the drag scale leaves 80 of set B's 1,000 truths below it, where a tail passes with 29 to 74. It read 71 before. The interval is not widened to pass: its shares come from the calibration set alone. The cell is recorded as failing on its lower tail, the report and the fit's page say so, the check stops if it passes or fails anywhere else, and [#1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108) holds the fix.

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-320]: 0320-m7-3c2-the-bayesian-option-in-two-blocks.md
[adr-363]: 0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[adr-367]: 0367-the-tip-off-phase-is-the-default-what-moved-and-what-flies-held-straight.md
[issue]: https://github.com/nrdptel/fusionspace-eridanus/issues/1108
[report]: ../../validation/reports/fit-intervals.md
[page]: ../physics/flight-fit.md#how-well-it-holds
[sets]: ../physics/synthetic-flights.md#the-sets

**Context.** The least-squares fit states a 90% interval on each parameter, and
[ADR-257][adr-257] §1 tests it on two sets of synthetic flights the interval's shares never saw
([the sets][sets]). On each set the truths below, inside and above the interval are counted, and
each count must sit in a 99.9% Clopper-Pearson band around 5%, 90% and 5%. At 1,000 flights a
tail passes with 29 to 74 truths and the inside with 868 to 930. Set A flies HPR Sim's own
model. Set B flies a different one on purpose: its drag rises otherwise through Mach 1, its wind
grows with height and its thrust curve is tilted.

`cargo xtask fit` stopped when any least-squares interval failed that test, and the
least-squares fit's record had every parameter passing on both sets ([ADR-316][adr-316]). The
Bayesian option's intervals don't pass yet and are printed as they come
([ADR-320][adr-320] §8).

Set B's drag scale passed at the edge, as that record said. Its fit ran high: 71 truths fell
below the interval and 29 above, and the fit's page said that 4 more below would fail it.

[ADR-367][adr-367] made the tip-off phase the default, so the synthetic flights fly it, both the
flight a log is made from and the flights the fit tries. The shares were fitted again on the
calibration set, as each run requires. The regenerated [report][report] reads:

| set B, drag scale, least squares | below | inside | above | mean error |
|---|---:|---:|---:|---:|
| held straight on the rail (before) | 71 | 900 | 29 | +0.00522 |
| the default, with the phase | 80 | 889 | 31 | +0.00711 |

The 9 more below are on five of the ten designs: Juno III's count rose by 3, Cavour's and
Genesis's by 2 each, Lince's and Valetudo's by 1 each. The other five designs' counts below are
as they were. The fits themselves moved: the mean error is the fitted scale less its truth, and
the shares don't enter it. The refitted shares moved too. On a log of height alone the drag
scale's went from −1.26 and 1.43 standard errors to −1.33 and 1.48, wider on both sides. On a
log with a track they went from −1.29 and 2.12 to −1.28 and 2.09, a little narrower. How many of
the 9 each accounts for is not separated here. Set A's drag scale reads 32 below, 909 inside
and 59 above, and passes. So does every other parameter on both sets.

Why the phase moves set B's fits upward is not measured. Set B's thrust and wind are shaped
otherwise than the fit's model, and both act on a rocket turning about its last guide. That
may be the path, and it is a guess until a run isolates it.

**Decision.**

1. **The interval is not widened to pass.** The shares come from the calibration set and
   nothing else ([ADR-257][adr-257]). Choosing them by set B's counts would fit the interval on
   the set that judges it, and the test would then say nothing.

2. **The synthetic flights are not held straight to pass.** A user's fit flies the default.
   Counts taken on flights held straight on the rail would state a coverage for a flight
   nobody flies unless they ask for it. The gap would still be there, unseen.

3. **The default is not held back for it.** The phase rests on its derivation and on
   RocketPy's rewritten phase ([ADR-363][adr-363]), and the fit's interval on set B was 4
   truths from failing without it. The fit's interval is the thing to fix.

4. **The cell is recorded as failing, and the check holds it there.** `cargo xtask fit` keeps a
   list of known gaps, `KNOWN_GAPS`, with one entry: the drag scale on set B by least squares.
   A cell on the list must fail on its lower tail alone: the band on the share below above 5%,
   the bands on the share above and the share inside holding 5% and 90%. The run stops if the
   cell passes, so that the entry is removed when the gap closes. It stops if the cell fails on
   another count too. Every other least-squares cell must pass as before, and every probe must
   fail as before. The counts are pinned as all the report's are: the run that writes the
   report reproduces it to the byte, and other platforms within their stated room of 5 flights
   ([ADR-316][adr-316]). At 5 fewer, 75 below, the cell still fails, so the entry holds on
   those platforms too. A test,
   `the_committed_report_keeps_its_rules_by_its_counts`, holds each of those rules on the
   committed report.

5. **Every place that states the interval says it fails here.** The [report][report] marks the
   row "fails" and says under the table that it is recorded. The [fit's page][page] says it in
   its opening lines and where it gives the counts. The accuracy page's row and the evidence
   register's row for the fit say it too.

6. **[#1108][issue] holds the fix and moves up to `P-high`.** It already asks for shares split
   by how well a flight's heights fit, fitted on the calibration set alone, with each class
   passing on both sets. On the regenerated report, set B's drag interval holds 353 of the 439
   flights whose heights fit to 1 m or worse, with 58 below, and 536 of the 561 that fit better,
   with 22 below. When the cell passes, the check stops until the entry is taken off the list.

**Which way it errs.** The truths lie below the interval too often, so the fit's drag scale
reads high on such a flight. A flight predicted again with that scale would come out low. A low
apogee is the wrong side to err on for someone sizing a flight to a waiver, which is why this is
recorded where a reader of the fit will see it, not left in a table.

**What this does not decide.** How the interval is fixed. Whether set B's model is the right
stand-in for a real flight: no real flight has checked the fit's intervals yet. Nor the
Bayesian option's counts, which have their own milestones.

**Consequences.**

- The fit's page no longer says each least-squares interval passes on both sets. A reader is
  told to take a fitted drag scale's lower end as too high about 8 times in 100 on a flight
  whose model differs as set B's does, where 5 are meant.
- Until [#1108][issue] closes, no new surface prints a fitted drag scale's interval without
  that caveat beside it.
- A later change that moves this cell further, or moves another least-squares cell out of its
  band, stops `cargo xtask fit` as before.
