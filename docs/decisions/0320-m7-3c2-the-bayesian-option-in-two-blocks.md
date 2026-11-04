# ADR-320: M7.3c2 in two: the Bayesian option draws the wind first and integrates the sensor out; its raw intervals don't pass yet (2026-11-03)

- **Status:** accepted; carries out [ADR-317][adr-317]'s M7.3c2 under [ADR-257][adr-257] §1 as far as M7.3c3, splits M7.3c2 in two (§8), and amends ADR-317 §1 (the proposal) and §2 (the likelihood); amends [ADR-316][adr-316] §7 (CI fits in 16 parts) and holds the fit's slice of [ADR-149][adr-149] with its posteriors (§9)
- **Summary:** M7.3c1's one Student-t about the least-squares answer kept a median of 14 effective draws in 300 on set A's first 60 flights, as the wind's ridge bends on heights alone. So the wind is drawn first and the rest about one Gauss-Newton step in that wind; a pilot widens the proposal where the posterior reaches further; the four linear sensor unknowns integrate out; on heights alone the wind's shear is drawn, uniform from 0 to 0.2. On 1,000 flights of each set the raw 90% intervals pass 4 of the 10 counts: set A's drag and set B's impulse fail, and the wind is too wide on both. So M7.3c3, the option counted, is met; M7.3c4 keeps the intervals passing.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-317]: 0317-m7-3c-in-two-the-sampler-then-the-test.md
[report]: ../../validation/reports/fit-intervals.md
[page]: ../physics/flight-fit.md

## Context

M7.3c2 asks for a Bayesian option whose raw 90% credible intervals pass ADR-257 §1's counts on sets
A and B, beside the least-squares fit's, on the same fits. ADR-317 planned one Student-t of 4 degrees
of freedom about the least-squares answer, its scale that fit's covariance, the likelihood the fit's
own. This records what that gave on the synthetic flights and what replaced it, in the order it came
up. Every count below before the report's was measured in development on set A's and set B's first
flights, with `cargo xtask fit --try`; sets A and B were seen throughout, as they were for the
least-squares fit (ADR-316).

## Decisions

1. **The wind first, the rest about it.** On set A's first 60 flights, 45 with something to fit,
   ADR-317's one Student-t kept a median of 13.6 effective draws in 300 (Kish's count), and the
   drag's raw intervals missed 8 of 45 truths. Growing its scale by the least-squares fit's widening
   for the wind made it worse, 5.6. On a log of height alone the heights can't tell the wind, and the
   drag, impulse and clock follow the wind along a curved ridge: the χ² of a draw rose up to 300
   times faster than the Laplace approximation said, most along the wind. So the wind's two
   components are drawn first, from a Student-t of their own block of the covariance; then the
   other unknowns from a Student-t of their covariance given the wind, about where one Gauss-Newton
   step from the answer, on the answer's Jacobian, puts them in that wind. That step costs a flight,
   so each draw flies twice. The proposal's density is a product of the two Student-t's, known in
   closed form, so the weights stay exact whatever the step gives. On the same flights the median
   rose to 74 of 300, and the drag missed none of 45.
2. **A pilot widens, never narrows.** With 4,000 draws on the worst of those flights, the
   posterior's clock offset reached 1.51 times as far as the Laplace approximation's, and the burn
   time 1.32 times. So 100 pilot draws estimate each block's spread about the proposal's center, and
   the main draws' scale is widened along each direction where that spread is larger, up to twice
   the standard deviation; it is never narrowed. Only the main draws are read, so that given the
   pilot the answer is plain importance sampling (M.-S. Oh and J. O. Berger, *J. Statist. Comput.
   Simul.* 41, 1992). Widening the second block's scale by 1.25 or 1.5 in every direction instead
   cut the median to 65 and 48 of 300: width paid for in nine dimensions is paid in each. With the
   pilot, the worst flight's effective draws rose from 6 to 28; the median held at about a quarter
   of the draws read.
3. **On heights alone the wind's shear is an unknown.** With 1 and 2 in place, on the first 300
   flights of each set, set B's drag missed 37 truths below its interval and 11 above, of 246, and
   34 of the 37 were on logs of height alone. Set B's wind grows with height by an exponent from
   0.14 to 0.20, which the posterior's constant wind can't. The least-squares fit carries that in
   its errors, by a sheared strong wind (ADR-316 §12); the Bayesian option carries it in the
   posterior. On a log of height alone the shear exponent is drawn with the wind, stated uniform from
   0, a constant wind, to 0.2, IEC 61400-1's normal profile, and flown so. It is stated, not fitted:
   no set's flights chose its ends. With a track, the drift speaks to the wind and the shear isn't
   drawn, as the least-squares fit's errors don't carry it there. Set B's drag then missed 25 below
   and 12 above. But set A's impulse missed 23 and 25 of 242, and its mass 21 and 21: with a twelfth
   unknown, the median effective draws fell to 36 of 200.
4. **The sensor's linear unknowns are integrated out.** On three of set A's flights, the wind
   block's weights alone kept 86 to 119 of about 165 draws, and the other block's alone 1 to 59: the
   loss was in the second block. Four of its nine unknowns, the height offset and a barometer's
   gain, drift and port coefficient, enter the residuals linearly, each with a normal prior. They are
   integrated out in closed form: `−2 ln p(θ) = χ²_min(θ) + ln det H(θ)`, `H` the χ²'s curvature in
   them, which changes with the flight through the gain's and the port's columns. This leaves the
   reported parameters' posterior as it was, since the integral is exact; the draws are of seven or
   eight unknowns, not eleven or twelve. Then set A's drag missed 5 below and 17 above of 242, its
   impulse 12 and 19, its mass 14 and 19; set B's drag 19 and 11 of 246, its impulse 18 and 6, its
   mass 14 and 16. The median effective draws came to 66 of 200 on set A, and to 127 on its 48 logs
   with a track.
5. **400 draws read, after 100 pilot draws.** At 400 read draws on the same 300 flights of set A,
   its impulse missed 11 below and 17 above, its mass 11 and 16, its drag 4 and 15, and the median
   effective draws rose to 127. Fewer effective draws gave narrower intervals here. 400 is a judgment
   between that and the cost.
6. **The intervals are raw.** As ADR-317 §3 has it: the posterior's 5% and 95% points as they come,
   no shares. A log that said nothing states its stated range by either method; a fitted log with
   no posterior states none, and misses on both sides.
7. **The cost.** A fit with something to fit draws 500 times, each draw two flights: about 1,000
   flights, against the least-squares fit's 70 on set A. Set A's first 300 flights, 242 of them with
   a posterior, took 2,600 CPU-seconds with 400 read draws, the least-squares fits included, on the
   Mac that wrote the report. The report took 3 hours 1 minute there, 58,121 user CPU-seconds, against
   about 16 minutes before; another check shared the machine for its last hour. The 4,572
   posteriors the report counts made 2,286,000 draws; the uncounted flights fitted past them drew
   posteriors too. CI fits it in 16 parts, not 8 (ADR-316 §7; §9). The report's
   threads now each take the next flight left, not every tenth, since a fit with a posterior costs
   many times one with nothing to fit; the run above had them every tenth and kept 5.4 of 10 cores
   busy on average. The calibration set draws no posterior, as only the least-squares shares are
   fitted there. Neither changes a number, as no fit depends on what its thread fitted before it.
8. **M7.3c2 in two.** The report's raw intervals fail six of the ten counts, below. Mending
   them takes another model of the track's noise at least, and causes for the drag's and the
   impulse's not yet measured, each tried on a three-hour report: more than this increment holds.
   ADR-317 §3 rules out shares. So M7.3c2 is split, its done-when kept
   whole and met when both are. **M7.3c3, the Bayesian option, counted:** the option, run on the same
   flights, every count and probe on the report beside the least-squares fit's with its pass or fail,
   the counts held by `fit --check`, identified on set A and not on noise, its draws counted. It's met
   by this report. While it stands, a Bayesian count that fails breaks no check; the least-squares
   fit's still do, and each method's identification on set A and on noise does. **M7.3c4, the
   Bayesian intervals pass:** every count of ADR-257 §1 on sets A and B, each probe failing,
   `fit --check` failing when one doesn't, on the same flights, drag and impulse still identified on
   set A and nothing on noise, the page saying how far to trust it. This plans every remaining
   increment of M7.3c2: M7.3c4 is the only one, with what to try first under *What this leaves*.
   The roadmap holds both done-whens in full.
9. **Under the time budget.** This increment first ran before the time budget's slices
   ([ADR-149][adr-149], amended for M0.16), and its full report doesn't fit the gate's 30 minutes.
   So the fit's slice, every 21st fit of each run, carries each posterior too: a line for its
   draws (how many it made, how many failed, what they're worth, whether that's enough), then each
   parameter's 5% and 95% points and median, with the side its truth falls on where the report
   counts it. The gate holds it byte for byte, each pull request in its words. A fit with something
   to fit and no posterior says so in a line of its own. On the 10-core Mac the slice took 335 s,
   2,934 CPU-seconds, against 79 s by least squares alone. By least squares alone a job of the slice,
   whole or on windows-latest half, already took 422 to 943 s on CI's runners (four runs,
   2026-11-04), so each platform now fits it
   in three parts, macos-latest in four and windows-latest, the slowest, in six. A whole job scaled
   by the Mac's 4.24, its build too, makes a ceiling for a part, if the runners slow by the Mac's
   ratio: 1,000 s at four on macos-latest, whose worst job took 943 s, and 1,280 s at six on
   windows-latest, whose worst half took 904 s, past the 20 minutes unless that job's build took
   more than 194 s. The build, which each part makes whole, brings them lower by an amount the
   first pull request's run measures. The night fits the whole report in 16
   parts on Linux, not the 32 this increment first ran with. A full run on main has 70 jobs; the slice's
   parts add 13 and 16 night parts 8, 91 in all, where 32 would make 107, past the 100 that
   `cargo xtask ci-budget` reads in one page and refuses beyond. The whole report, written again on the
   merged code with each thread taking the next flight left, took 6,496 s (1 h 48 min) on the Mac,
   against §7's 3 h 1 min and about 16 minutes by least squares alone: about seven times. The 8
   parts took 875 to 1,322 s by least squares (2026-11-04). One of 16 fits half as many flights,
   each about seven times as costly, so it should take 50 to 80 minutes, an estimate the next
   night measures, under the 6 hours GitHub gives a job.

## Evidence

[The report][report] holds the counts, on the same 1,000 fitted flights of each set and the same
1,497 tracked flights for the wind as the least-squares fit's, whose counts didn't move. The Bayesian
option's below, inside and above:

| Parameter | Set A | Set B |
|---|---:|---:|
| drag scale | 27, 895, 78: fails | 66, 884, 50: passes |
| impulse scale | 70, 881, 49: passes | 83, 888, 29: fails |
| wind toward the east | 29, 1,452, 16: fails | 26, 1,439, 32: fails |
| wind toward the north | 34, 1,429, 34: fails | 31, 1,430, 36: fails |
| mass on the weighed mass | 52, 887, 61: passes | 63, 870, 67: passes |

Every probe fails but the narrowed one on the north wind, on both sets. Drag and impulse are
identified on set A (median widths 0.0917 and 0.0391, of 0.36 and 0.108 stated); on every kind of pure
noise, nothing is, though the fit refuses every noise log before a posterior is drawn, so that says
nothing of the sampler. On set A, 319 of 2,276 posteriors had under 100 effective draws, the least 1.13
and the median 232 of all 2,276; on set B, 392 of 2,296, the least 1.01 and the median 223. Of 1,138,000 draws on
set A, 124 failed; of 1,148,000 on set B, 179. Every flight counted with something to fit has a posterior.
At 1,000 flights a tail passes with 29 to 74 truths; on the wind's 1,497, with 49 to 104, and the
inside with 1,308 to 1,384.

## What this leaves

- **Sets A and B were seen.** The blocks, the pilot, the integration, the shear and the draws were
  each chosen after reading counts on the first 300 flights of each. Nothing is fitted to them; the
  shear's ends are stated, and the intervals are raw.
- **The wind is too wide on a track.** The Bayesian option's median width is 0.431 m/s east on
  set A, against least squares' 0.345, and 1.07% to 2.40% of truths fall in each tail, not 5%. The posterior reads
  the track's noise as the least-squares fit states it, floored at 10 m (33 ft) and widened for
  residuals that run on (ADR-316 §3, §11). On logs with a track the least-squares shares in force
  narrow that back, −1.36 and 1.29 east, −1.5 and 1.38 north, against ±1.645 for a bell curve; the
  posterior has none. That's the likely cause, not yet measured. A noise model the track's own residuals settle, its scale and
  correlation drawn with the rest, is M7.3c4's first try.
- **The shear is stated, not known.** A wind that grows faster than IEC's 0.2, or one that turns
  with height, lies outside it. Set A's wind doesn't shear, yet its drag's intervals sit low, 78
  truths above and a mean error of −0.0104; 70 of the 78 are on logs of height alone, where the
  shear is drawn. Whether the shear's draws move it, and which way, isn't measured yet. Set B's impulse sits high, 83 below.
  M7.3c4 measures each count by track and by design first.
- **A posterior far from its peak isn't reached.** A second mode the pilot doesn't find is still
  missed, and few effective draws are reported, not mended (ADR-317): 711 of the 4,572 posteriors
  had under 100.
- **The Gauss-Newton step uses the answer's Jacobian,** not one at each wind: more steps (two or
  three, at a flight each) moved the median effective draws by under 10%.
