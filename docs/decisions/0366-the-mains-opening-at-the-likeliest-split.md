# ADR-366: The main's opening at the likeliest single change of the descent's rate (2026-11-21)

- **Status:** accepted; changes how [ADR-283][adr-283] §6 finds the main's opening where the descent's rate passes the main's mark more than once; changes no mark, window or bound
- **Summary:** The canopies reading walked back from landing and stopped at the first rate past the main's mark, so a swing under the main or a coarse log's scatter was read as the opening ([#903](https://github.com/nrdptel/fusionspace-eridanus/issues/903)). The main's descent now starts at the likeliest place for one change in how often the rate is within the mark, the change-point of a two-valued sequence (Hinkley and Hinkley, 1970). On the issue's real flight the opening moves from 108.45 s to 90.00 s, inside the issue's 87 to 92 s, and the drogue's rate from 14.6 to 17.3 m/s (48 to 57 ft/s) against a measured 17.3 m/s (57 ft/s). The committed synthetic census is unchanged (`cargo xtask synthetic --check` on this branch).

[adr-283]: 0283-the-rest-of-the-readings.md
[issue]: https://github.com/nrdptel/fusionspace-eridanus/issues/903
[page]: ../physics/log-readings.md#the-canopies
[report]: ../../validation/reports/synthetic-flights.md
[survey]: ../../validation/reports/log-import.md
[residuals]: ../../validation/reports/flight-residuals.md

**Context.** [ADR-283][adr-283] §6 finds the main's opening as Debrief does. The rate near the
ground sets a mark, 1.6 times that rate or 3 m/s more if that is more. Walking back from landing
on the rate's 2 s running median, the first rate past the mark ends the main's descent. Two
cases showed that stop wrong, both under [#903][issue]:

- **A swing.** On a StratoLogger CF's flight from a public forum thread, the rocket swings under
  its open main. The walk stopped at a swing and read the opening at 108.45 s, where the log's
  header gives the main charge at 87.15 s. The drogue's descent then averaged in about 18 s of
  the main's and read 14.6 m/s against about 17.3 m/s. A drogue rate read slow is the flattering
  side for a descent-energy figure.
- **A coarse log.** At 5 Hz in 10 Pa steps a one-second fit holds five samples of 0.93 m steps,
  and its rate scatters by a few metres a second. On one flight of the synthetic tuning set,
  flown with the tip-off phase on, the median passed the mark at one sample 56 s after the
  opening. The 2 s before it were the main's too, so the reading was one canopy, with the main's
  rate 5 m/s outside its bound. `cargo xtask synthetic` stops on it, which blocks M1.14b15.

The committed census never drew either case. No bound was loosened to pass it.

**Decision.**

1. **The split.** Each rate of the descent, from apogee to the last the reading uses before
   landing, is slow
   (within the mark) or fast (past it). For a split at $s$ of the $n$ rates, with $a$ slow ones
   before it and $c$ after it,

   $$
   L(s) = \ell(a, s) + \ell(c, n - s), \qquad
   \ell(k, m) = k \ln\frac{k}{m} + (m - k) \ln\frac{m - k}{m}
   $$

   The main's descent starts at the split with the greatest $L$, among the splits whose later
   side holds the larger share of slow rates. Of two as likely, to a part in $10^9$, the one
   nearer landing is taken. Where no split is likelier than no change at all, the whole descent
   is the main's if at least half its rates are slow, and none of it otherwise.
2. **Its source.** This is the maximum-likelihood change-point of a sequence of two-valued
   variables whose chance changes once, each side's chance taken at its own share (D. V. Hinkley
   and E. A. Hinkley, "Inference about the change-point in a sequence of binomial variables",
   *Biometrika* 57, 1970, pp. 477–488). The rates here aren't independent, a running median of
   overlapping fits, so the likelihood is the rule's reason and no stated confidence.
3. **It is the walk where the rates change once.** Where they go from fast to slow once, after
   any slow ones of the start from rest at apogee, the split is the walk's start: a test tries
   every such descent of up to 19, 40 and 40 rates. The split is never later than the walk's
   start: every rate after that start is slow, and adding slow rates to an earlier side that
   holds a fast one only lowers $\ell$ there.
4. **Nothing else changes.** The marks, the running median, the density scaling, the two tests
   for a drogue and the opening's bound are ADR-283's. The rule has no fitted number: the tie's
   tolerance covers the logarithm's last bits, which differ between platforms.

**An alternative tried first.** The split that leaves the fewest rates on the wrong side of
the mark, slow before it or fast after it, is the same likelihood with the chance of a
wrong-side rate forced equal on the two sides. It passed the invented cases and the census. On
the real flight it read 92.85 s, past the issue's window by more than the opening's bound,
because a fast stretch soon after the main opens outlasts the slow rates before it. In the survey of
real logs it also took the drogue's descent from three logs of two families, one of them a
flight whose drogue falls near the mark, so that nearly half its drogue's rates are slow.
Taking each side's chance from its own share fixes both, with no number added.

**What was measured.**

| check | walk | split |
|---|---|---|
| the swing, invented: 3 logs | the test fails: the first log's opening reads 165.15 s for 120.54 s, bound 1.0 s | opening and both rates in bound on each |
| the coarse log, invented: 732 logs | 1 read as one canopy (the main at 4.05 m/s) | 732 read two canopies, in bound |
| the real flight's opening (issue: 87 to 92 s) | 108.45 s | 90.00 s, bound 0.76 s |
| the real flight's drogue rate (issue: about 17.3 m/s) | 14.6 m/s | 17.3 m/s, within 1.4 m/s |
| Genesis (EuRoC 2023), a public flight: the drogue's rate (simulated 23.73 m/s) | withheld: one canopy | 25.68 m/s |
| Genesis: the main's opening height (simulated 797.33 m) | withheld | 806.18 m |
| Genesis: the main's rate (simulated 11.60 m/s) | 19.18 m/s, the whole descent's | 11.53 m/s |
| `cargo xtask synthetic --check`, 2,200 flights, run on this change's branch | passes | passes: the committed census is unchanged |

Seven mutations of the split each fail a test: ties taken away from landing; the later side's
larger share not asked for; the later side's term dropped; the earlier side's term dropped; the
answer with no change always the whole descent; its likelihood left out; and the count of
wrong-side rates in the likelihood's place. The gate flies a slice of the census, and the
nightly run all of it on five platforms.

On the real logs of [the survey][survey] the readings that say one canopy go from 181 to 173.
The drogue's descent is given on 7 more logs, in four families, and one more is withheld for a
short leg. No family's count of drogue's descents given falls.

The Genesis rows are [the residuals report][residuals]'s, which lists a public flight's log
beside its simulation. The simulation takes nothing from the log's canopies, so the log's
806 m against its 797 m shows the two agree, on a flight the rule wasn't made on. It is no
more than that: no bound is stated for the height, and the simulation's drogue rate is below
the log's interval, 24.50 to 26.88 m/s.

**What is left.** One real flight with a logged charge time and one beside its simulation are
thin. A physics review found a shape where the split is wrong and the walk was right: a drogue
falling near the mark, slow for a few seconds and then fast just before the main opens (30
slow, 200 fast, 100 slow, 25 fast, 1,000 slow splits at 230, where the walk's 355 is the
opening). The opening then reads early with a bound that doesn't cover it
([#1593](https://github.com/nrdptel/fusionspace-eridanus/issues/1593)).
The opening's bound is proven where the rate passes the mark once; past that it is held to the
tests' flights and these two. A canopy's filling time is still outside it
([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)).

**Which way it errs.** The split is never later than the walk. A drogue's leg that ended late
took in the main's slower descent and read slow, the flattering side; the split can only shorten
that leg. A split too early would give the main's leg some of the drogue's fall and read the
main's rate fast, the safe side for landing energy. It would also leave the drogue's leg more
of its start from rest at apogee and read the drogue's rate slow, the flattering side; how
often that happens on real logs isn't measured.

**Consequences.**

- `hpr-flightdata`'s canopies reading calls `likeliest_split`. Its unit tests hold the rule by
  hand-worked cases and against every split tried in turn; the two invented flights and a test
  on the cached real log hold the reading.
- [The readings page][page] states the rule with a worked case, the three tests and what is
  left.
- M1.14b15 can regenerate [the synthetic census][report] on the tip-off default.
