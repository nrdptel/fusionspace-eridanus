# ADR-348: M7.5b's verdict: the shared-bias correction fails on the public flights, and nothing ships (2026-11-14)

- **Status:** accepted; carries out [M7.5b][m7-5b] by [ADR-346][adr-346] decisions 2 to 7, unchanged; records its verdict, a fail, so nothing ships, [M7.5c][m7-5c] is dropped and [M7.5][m7-5] closes (ADR-346 decision 9).
- **Summary:** Fitted on the calibration half as ADR-346 fixed it, the correction is an impulse scale of 0.94 with the drag left at 1, in the fit without the flagged flight. It lowers the held-out half's mean absolute apogee error from 16.26% to 10.91% and its mean error from +13.36% to +5.84%. Both fits pass all four held-out sets. On the 12 public flights it raises the mean absolute error from 9.00% to 12.16% and takes the mean error from −4.91% to −12.01%; the other fit does worse. The rule asks that the public flights get no worse, so the verdict is a fail. The held-out half's sealed fit aggregates are published, their file matching its committed hash.

[m7-5]: ../decisions-and-roadmap.md#m7-5
[m7-5b]: ../decisions-and-roadmap.md#m7-5b
[m7-5c]: ../decisions-and-roadmap.md#m7-5
[m2-7]: ../decisions-and-roadmap.md#m2-7
[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[m1-14]: ../decisions-and-roadmap.md#m1-14
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[boost]: ../../validation/reports/collection-boost.md
[public]: ../../validation/reports/real-flights.md
[from-boost]: ../../validation/reports/from-boost.md

**Context.** [ADR-346][adr-346] fixed, before any fit, the correction of the bias the simulators
share: two scales, one on every motor's impulse and one on the drag, fitted on the calibration
half's logs and judged on the held-out half and the public flights, with and without the flights
whose log and files disagree by more than NAR's retest rule allows. It fixed the pass rule: on
the four held-out sets, the mean absolute error and the absolute bias each lower than the forward
simulation's; on the two public sets, neither higher. Both fits must pass all six.

**The sets,** fixed by the reports at the head of `main` when this branch was cut, commit
11677e78:

| set | flights | flagged | less the flagged |
|---|---:|---:|---:|
| held-out flights the forward report compares | 23 | 1 | 22 |
| … without a deployment before apogee | 13 | 0 | 13 |
| public flights the public report compares | 12 | 0 | 12 |
| calibration flights predicted (the fits' flights) | 12 | 1 | 11 |

A held-out flight's flag comes from the fit of its own boost alone: 9 held-out logs were fitted
for that and for nothing else. A public flight's comes likewise where the [public boost
report][from-boost] cuts and fits its log, 2 of the 12; the other 10 can't be flagged and stay in
every set.

**The runs.** Five, all on this branch, each flying the correction on the held-out half and the
public flights.

1. The first. Its page named a failed clause by its set alone, so a set and the same set less its
   flagged flights read as one clause twice. It wrote "none" for a sign-flip interval it doesn't
   count past 20 flights, and labeled a scale's last row "why it stays 1" where the scale entered.
2. The second, after those three fixes to the page's words. Every number of the first run's
   private record is the same in the second, the list of failed clauses aside.
3. The third, after the page set each set beside itself less its flagged flights: in the second
   run's page those rows came after the public flights' and read as theirs. Its private record is
   the second's byte for byte.
4. The fourth, after a review of the third. Its page holds back the other fit's counts of flights
   too, as §7 asks (it gives that fit by its pass or fail on each set, the public rows in full),
   and marks each number it holds back apart from one that doesn't exist. Its verdict is a list. A
   public flight predicted first that fails to fly corrected now fails alone, not with the others.
   Its private record gains those two marks and nothing else: every number is the third run's.
5. The fifth, after a review of the fourth's words: the page names whose terms hold numbers
   back. Its private record is the fourth's byte for byte.

No scale, set, weight or limit changed between them, and no number of the fits or the
corrected flights did.

**The fits.** ADR-346 §4's estimator for the impulse is the weighted mean with its spread scaled
by the Birge ratio, as Merkatas, Toman, Possolo and Schlamminger give it (*Metrologia* 56, 054001,
2019, §3.1). Its Student's $t$ at $n - 1$ is the posterior of the mean under a reference prior
(Bodnar and Eriksson, arXiv:2104.01977, 2021, the proof of Theorem 1). The $t$ points come from
the closed forms in Abramowitz and Stegun (1964, 26.7.3 and 26.7.4), held to the NIST/SEMATECH
e-Handbook's table to three decimals.

One calibration flight is flagged, fewer than 3. So, by ADR-346 §7, the fit without it is given
as flown and the fit over every calibration flight by its pass or fail. What the page holds back
is in a private file whose SHA-256 is committed in `validation/reports/correction-held-back.sha256`.
The public rows give the other fit's errors in full, as §7 says they are given either way; none
of the 12 public flights is flagged. Those are over all 12 flights, and §7 holds back what would
give a flagged flight's own.

| fit without the flagged flight | drag scale | impulse scale |
|---|---:|---:|
| values | 7 coasts solved of 7 | 11 fits converged of 11 |
| interval excludes 1 | no | yes |
| as flown | 1.00 | 0.94 |

**The test,** as [the boost report][boost] gives it (the corrected flights' mean absolute error
and mean error against the forward simulation's, %):

| set | fit without the flagged flight | fit over every flight |
|---|---|---|
| held-out, 23 | 16.26 → 10.91; +13.36 → +5.84; passes | passes |
| … without a deployment before apogee, 13 | 11.30 → 6.84; +6.88 → −0.05; passes | passes |
| held-out less flagged, 22 | passes | passes |
| … without a deployment, less flagged, 13 | passes | passes |
| public, 12 | 9.00 → 12.16; −4.91 → −12.01; fails | 9.00 → 13.21; −4.91 → −13.21; fails |
| public less flagged, 12 | fails, as above | fails, as above |

Flown with both scales at 1, every flight either report compares lands 0.00 m from its forward
apogee, to the report's two decimals. The rule allows 0.5 m.

**Decision.**

1. **The verdict is a fail.** Eight clauses failed, all on the public flights: for each fit, on
   the public set and on it less its flagged flights, the mean absolute error is higher than the
   forward simulation's, and so is the absolute bias. Nothing ships: no flight's output carries a
   correction, and [M7.5c][m7-5c] is dropped.
2. **M7.5 closes.** Its *done when*: "or a decision record says that it doesn't, and nothing
   ships", then the sealed held-out fit aggregates published. The boost report now gives them,
   and their file still matches `validation/reports/held-out-fits.sha256` (`608ab0bd…85920`),
   committed before any correction was fitted.
3. **The held-out half has been seen against a correction.** By ADR-346 §6, a later correction
   of another form is a new decision, judged on flights predicted first ([M2.7][m2-7]'s second
   increment), not on this half again.

**What it says.** One scale didn't serve both sources. The collection reads high and the public
flights read low, as ADR-346's consequences foresaw. A lower impulse cuts the held-out half's bias
by more than half, and on the flights without a deployment before apogee to nearly nothing. On the
public flights it more than doubles the mean error's size, from −4.91% to −12.01%. The 12 flights
are few. For this fit, 8 of them grew in absolute error, and the mean of all 12 changes is +3.16
percentage points of absolute error; the paired bootstrap interval of that mean, −0.31 to 6.30,
holds 0: the harm is not shown with confidence. For the fit over every calibration flight the mean
change is 4.21 points (9 of 12 grew) and its paired bootstrap interval, 0.51 to 7.52, excludes 0,
though its exact sign-flip interval, −0.03 to 8.44, just holds it. Either way the public flights
give no support to one bias, shared with the collection's flights, that a scale could take out. It
doesn't say where the collection's high reading comes from: the test separates no cause from
another. The boost report keeps the measurement, less what ADR-346 §7 holds back, for the work on
causes: [M2.3c6][m2-3c6], the gap to OpenRocket by cause, and [M1.14][m1-14], accuracy inside the
envelope. It gives how much of each half's error is made up to the cut, and what a correction of
the boost does to each set.

**Consequences.**

- The scoreboard, the forward reports and every flight's output are unchanged.
- `fixture-flights` now also flies the public flights corrected, so it needs the public flights'
  checkouts under `refs/` as `real-flights` does. It runs longer: a fit for each step of each
  coast's bisection, and every compared flight flown again with each correction.
- The public flights' fly paths (`hpr_validate::real_flight::fly_corrected`,
  `public_predictions::corrected_apogees`) take a correction for this test, each flight's result
  its own. With none they are the paths they were.

**How it was checked.** `cargo xtask fixture-flights --check` reproduces the report and both
digests. Unit tests pin the $t$ points against the printed table, the bisection, each scale's entry
rule, the pass rule's strict and non-strict sides, a failed flight failing its set, what the
page holds back (the other fit's counts included), and a held-back set's failure named by its
fail alone.
