# The least-squares fit

See how the library fits a flight's [log](../glossary.md#flight-log) to a simulation for drag,
impulse, wind and mass, and where its Bayesian option fails its tests.

## In short

- **What it models:** the simulator's inputs are changed until the simulated flight passes through
  the log's samples up to apogee, read the way the log's sensor reads it. It fits a scale on the
  drag and one on the impulse. It fits a constant wind in two components, and the mass within what
  the weighing says. Each comes back with a 90% interval.
- **Sources:** Levenberg-Marquardt as in *Numerical Recipes*; the widening for correlated residuals
  from Wilks; the wind's growth with height from IEC 61400-1 ([Sources](#sources)). The test of an
  interval is [ADR-257's][adr-257].
- **How well it is validated:** use the least-squares intervals; the Bayesian option's don't pass
  yet. Both are tested only on [synthetic flights](synthetic-flights.md), and only as a whole. By
  least squares, each parameter's interval passes on two sets of test flights
  ([the sets](synthetic-flights.md#the-sets)), set B's drag at the edge of its test. Set A flies the
  simulator's own model; set B flies a deliberately different one. A truth is the value a test
  flight was flown with. On set B it's the fit to the same flight with no noise, for all but the
  mass. The wind passes only on logs with a track, the log's latitude and longitude. Choices in the model were made after seeing those two sets; the intervals' sizes come
  from a third. An interval holds 90% over a set, not on each design or flight
  ([How well it holds](#how-well-it-holds)). No real flight has checked its intervals
  ([evidence row G38](../evidence.md#g38)). On a real flight, the model's own errors go into the
  scales with nothing to say so. The Bayesian option draws the unknowns' possible values at random rather than
  reading the curvature at the answer. Its intervals pass 8 of their 10 tests, 5 parameters on each
  of 2 sets. The wind passes on both sets, on a model of the receiver tuned on those same flights.
  Set B's drag passes, but so does a copy widened on purpose. A tail passes with at most 74 of
  1,000 truths. Set A's drag reads low, 83 above its intervals, and set B's impulse high, 85 below.
  Each makes the rocket look better than it flew. Until all pass, use the least-squares intervals
  ([The Bayesian option](#the-bayesian-option)).
- **What it leaves out:** anything past apogee, and the accelerometer. A log whose only height is a
  GNSS receiver's isn't fitted, since the log reader takes no apogee from it; that's every log with
  nothing to fit among the sets' counted draws, about one in six ([when](#when-a-log-has-nothing-to-fit)). Its wind is
  constant with height; on a log of height alone, a wind that grows with height only widens the intervals. It's
  told the design, rail, site and the day's air, and takes them as right. It's in the library only,
  with no `hpr` command yet. Its [prediction from the boost](#apogee-from-the-boost) has been
  compared with a logged apogee on 2 public flights, both cut after their burn, on logs that
  stay under 227.7 m/s (747 ft/s): an anecdote, not a measure.

## How it works

The fit reads the log's heights, as a median every 0.25 s, from a second before liftoff to apogee.
A track is the log's latitude and longitude, from a GNSS receiver such as GPS. Where the log holds
one, the fit reads the drift as well, which is what speaks to the wind. It flies the simulation about 70 times a fit, which
takes about a second of one processor core. It steps the inputs by the
Levenberg-Marquardt rule, a damped Gauss-Newton search, until the sum $\chi^2$ stops falling. It
has three parts, the heights', the track's and the stated ranges':

$$\chi^2 = H + T + P$$

$$H = \sum_i \frac{(y_i - \hat y_i)^2}{\sigma^2}$$

$$T = \sum_k \frac{(r_k - \hat r_k)^2}{\sigma_r^2}$$

$$P = \sum_j \frac{(x_j - \mu_j)^2}{s_j^2}$$

Here $y_i$ is a logged height and $\hat y_i$ the simulated height, read as the sensor reads it.
$r_k$ and $\hat r_k$ are the same for the track, and $x_j$ is an input. The last part holds each input
to what's stated of it, as a range: $\mu_j$ is the range's middle, and $s_j$ sets its width. That's
why a log with no word on the wind gives back the wind's stated range. $\sigma$ and $\sigma_r$ are
the root-mean-square of the fit's own residuals. They're at least 3 m (10 ft) on height, a
barometer's noise ([log readings](log-readings.md)). On the track they're at least 10 m (33 ft), a
judgment for a receiver's errors, which run on from fix to fix ([ADR-316][adr-316]).

On the synthetic flights, the stated ranges are those the flights draw from
([what a flight draws](synthetic-flights.md#what-a-flight-draws)):

| Input | Stated range | Its 90% width (factor; the wind's in m/s) |
|---|---|---:|
| drag scale (times the simulator's drag) | 0.8 to 1.2 | 0.36 |
| impulse scale (times the motor's rated impulse) | 0.94 to 1.06 | 0.108 |
| mass (times the weighed mass) | 0.98 to 1.02 | 0.036 |
| wind, each of east and north | a speed up to 8.94 m/s (29.3 ft/s), any way | 12.7 (41.7 ft/s) |

A sensor's errors would otherwise land in the drag, so the fit also solves for them. On every log
that's the clock offset, the height offset and a height that drifts over the flight. On a barometer
it's also the reading's gain, a scale on the heights it reads. And it's the port's pressure
coefficient: how much the airflow past the vent lowers the pressure it reads. A barometer's bins at
or past Mach 0.8 are left out, where the port's error grows past the range the fit holds. The cut is
taken again from the flight each pass finds. The burn time also has a scale, not reported, since a motor's
impulse and its burn trade against each other. A wind of either sign lowers the top. So a log of height
alone is flown in a wind of the stated range's root-mean-square speed, 5.16 m/s (16.9 ft/s).

The parameters' standard errors come from the fit's curvature, taken from the product of its
Jacobian with itself as Gauss-Newton does ([standard error](../glossary.md#standard-error)). Two things widen them:

- **Residuals that run on.** A misfit is correlated from bin to bin, so the bins tell less than as
  many independent ones. The heights' and the track's noise variances are each widened by
  $(1+\rho)/(1-\rho)$, where $\rho$ is the residuals' correlation from one bin to the next, capped
  at 0.9. A residual with $\rho = 0.5$ triples it. It's an approximation: Wilks gives the factor for
  the variance of a mean, and the cap is a judgment. The calibrated shares take up what it misses.
- **A wind heights can't tell.** On a log of height alone the wind is unknown, and its loss grows as
  its speed squared. The fit moves its answer, to first order, to a calm and to a strong wind: the
  stated wind's speed plus half its 90% width. On the synthetic flights that's 5.16 m/s (16.9 ft/s)
  plus 6.35 m/s (20.8 ft/s), 11.5 m/s (37.7 ft/s) at 18.3 m (60 ft) above the pad, past the 8.94 m/s (29.3 ft/s)
  they draw. It's grown with height as $(z/18.3\,\text{m})^{0.2}$. With $\delta$ the larger of the
  two moves, $\delta^2/3$ joins the variance, as a uniform spread over $\pm\delta$ would.

Each 90% interval is then a standard error times two factors, its low and high shares. They're found
on the [calibration set](synthetic-flights.md#the-sets), kept apart from the test flights. They're
the 5th and 95th percentiles there of each fit's error over its standard error. Logs of height alone have one pair of
shares, on the calibration set's height-alone logs with something to fit. Logs with a track have
another, on 1,000 flights of its logger with a barometer and receiver.

Each fit also says whether it stopped at the bottom of $\chi^2$ within 60 steps a pass.

### When a log has nothing to fit

A log with nothing to fit gives back its stated range and says so. That's any of these:

- the log has no height channel, or the log reader finds no apogee or pad height in it;
- its climb, from liftoff (or its first sample) to apogee, isn't positive or would run past
  1,000 s;
- its climb fills too few 0.25 s bins, as many as the fit's unknowns or fewer;
- a barometer has too few bins below Mach 0.8;
- the fit's heights stand far from the log: an RMS residual over 0.1 of the spread of the heights
  it read;
- a simulation can't fly, or the curvature can't be inverted.

On the synthetic flights, every log with nothing to fit has only a GNSS receiver's height. That's
216 of set A's first 1,216, and 193 of set B's first 1,193. No other cause in the list
removed a flight from either set ([the report][report]).

### A worked example

Flight 140 of set A flies the Calisto design (one of the
[example rockets](../glossary.md#example-rockets)) in its getting-started variant with the motor at
−1.255 m (−4.12 ft), with a logger that holds a track. The report keeps only counts, so a test,
`the_pages_worked_example_is_a_real_flights`, fits this flight again and checks these rows.
Its heights fit to 0.97 m (3.2 ft), in 84 simulated flights. The table is the least-squares fit's;
21 of those flights are the Bayesian option's, finding where to start its draws. A factor is times the simulator's
value; the mass's, times the weighed mass:

| Parameter | Fitted (factor) | 90% interval (factor) | Truth (factor) |
|---|---:|---:|---:|
| drag scale | 0.903 | 0.857 to 0.931 | 0.897 |
| impulse scale | 1.011 | 0.991 to 1.028 | 1.019 |
| mass on the weighed mass | 1.000 | 0.982 to 1.017 | 1.013 |

| Parameter | Fitted (m/s) | 90% interval (m/s) | Truth (m/s) |
|---|---:|---:|---:|
| wind toward the east, m/s | 4.45 | 4.24 to 4.67 | 4.59 |
| wind toward the north, m/s | −4.18 | −4.38 to −3.97 | −4.31 |

So this flight says its drag was about 10% under what the simulator took, and that the motor gave
about its rated impulse. It says the wind blew at about 4.5 m/s (14.8 ft/s) toward the east and 4.2 m/s
(13.8 ft/s) toward the south. The interval on the mass is nearly its stated ±2%: this flight doesn't weigh the
rocket any better than a scale does.

## How well it holds

[ADR-257][adr-257] sets the test. Set B's drag curve, thrust and wind are shaped otherwise, so it
has no flown drag or impulse scale or wind. Their truth is the same fit run on the flight's
noise-free samples; only the mass keeps its flown value. On each set, the truths below an interval, inside it and above it
are counted. Each count must fit a 99.9% Clopper-Pearson band (an exact binomial interval) around
5%, 90% and 5%. An
interval narrowed to 0.8, widened to 1.25, or shifted down one standard error must fail it.
[The report][report] holds every count and each design's ([how CI checks it](../VALIDATION.md#in-ci-and-regenerating-the-references)).

Each set counts its first 1,000 logs with something to fit. Of set A's first 1,216 flights, 216 had
nothing to fit; of set B's first 1,193, 193. Each has only a GNSS receiver's height. A wind's counts are on 1,500 flights of the logger with
a barometer and a GNSS receiver, some past those. Three of them on each set are left out: nothing
to fit, or no track read.

The counts are flights:

| Parameter | Set A below, inside, above (no unit) | Set B below, inside, above (no unit) |
|---|---:|---:|
| drag scale | 34, 905, 61 | 70, 901, 29 |
| impulse scale | 35, 908, 57 | 54, 910, 36 |
| wind toward the east, m/s | 89, 1,343, 65 | 75, 1,349, 73 |
| wind toward the north, m/s | 67, 1,366, 64 | 71, 1,362, 64 |
| mass on the weighed mass | 40, 911, 49 | 42, 900, 58 |

A log tells of a parameter, the report's "identified", when the median interval on set A is at most
half the stated range's 90% width:

| Parameter | Median width (factor) | Stated width (factor) |
|---|---:|---:|
| drag scale | 0.115 | 0.36 |
| impulse scale | 0.0399 | 0.108 |
| mass on the weighed mass | 0.035 | 0.036 |

So it tells of the drag and impulse, and not of the mass. On logs with a track, a wind's median
width is 0.345 m/s (1.13 ft/s) east and 0.374 m/s (1.23 ft/s) north, of 12.7 m/s (41.7 ft/s) stated.

On pure noise nothing is told ([the report's noise table][report]). The log reader turns noise of
±5 m (16 ft) and ±1,000 m (3,281 ft) away. Noise is also handed straight to the fit. On height alone it's
±5 m (16 ft), ±50 m (164 ft) or ±1,000 m (3,281 ft). With a track of ±5 m (16 ft), it's ±5 m (16 ft)
or ±1,000 m (3,281 ft). It says nothing in all 200
flights of each. The fit's residual is at least 0.958 of the heights' spread, against 0.1 allowed.

Those counts have limits:

- Sets A and B were seen. The barometer's unknowns, the track's floor, the split of shares by track
  and set B's truth were chosen after reading their counts. So were which logs are counted and which
  hold a wind, the wind's flight count, the misfit's cut of 0.1, and both widenings. Only the shares themselves come from flights kept apart.
- Set B's drag passes at the edge: 29 above is the fewest that pass at 1,000. At 70 below, 5 more
  would fail it too. Its fit runs high by 0.005 on average.
- Coverage holds over a set, not on each design. On set B, the Prometheus design's drag interval
  holds 68 of its 96 truths, and its wind toward the north 127 of 161.
- Coverage holds over a set, not on each flight. A flight whose heights fit to 1 m (3.3 ft) or worse
  misses more often. Set B's drag interval holds 376 of 453 such flights, and 525 of 547 that fit
  better (the issue that tracks it: [#1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108)).
- Some fits stop short of the bottom of $\chi^2$. Among the flights counted and the wind's, 2 on set
  A and 7 on set B did, counting set B's truths' own fits.
- A wind on a log of height alone isn't counted and shouldn't be read. Height alone can't tell a
  steady wind from one that changes with height.

## The Bayesian option

`hpr_forensics::fit::bayesian` runs the least-squares fit, then draws the posterior of the same
unknowns around its answer. [ADR-317][adr-317] chose importance sampling; [ADR-320][adr-320] draws
the wind first, then the rest. Its 90% intervals don't pass
their test yet: 8 of the 10 tests pass, 5 parameters on each of 2 sets. One of those 8, set B's
drag, passes with a probe that should fail passing too. Set A's drag and set B's
impulse miss too many truths on one side, so there they're too narrow or off center. Until they
pass, read the least-squares intervals. Like the fit, it's in the library only, and no real flight has checked
it.

The posterior is what the fit's unknowns could be, given the log and the stated ranges. The
least-squares fit reads its uncertainty off the curvature at its answer, as if the posterior were a
bell curve. It then multiplies that standard error by two shares, factors fitted on the calibration
set so that its intervals hold ([How it works](#how-it-works)). The Bayesian option uses none. The Bayesian option
draws points around the answer at random and flies each one. It weighs each point by the posterior's
value there divided by how likely the draw was to land there. So the points it lands on often don't
count extra: that's importance sampling. Each parameter's 90% interval is the weighted points' 5% and 95%
points, as they come, with no shares. So a posterior that bends or leans one way shows as it is.

How it draws:

- **The wind first.** On a log of height alone the heights can't tell the wind. The drag, the
  impulse and the clock offset follow the wind along a curve. So each draw takes the wind first, then the
  other unknowns around where one step of the least-squares fit's search puts them in that wind. That step is a flight,
  so each draw flies twice.
- **A pilot widens the draws.** 100 pilot draws measure how far the posterior reaches. Where it
  reaches past the bell curve, the next 400 draws are spread wider, up to twice as wide, and never
  narrower (Oh and Berger). Only those 400 are read.
- **The sensor's linear unknowns are worked out exactly.** The height offset, the drift, and a
  barometer's gain and port coefficient each enter the heights linearly. So their part of the
  posterior has a closed form. Seven unknowns are drawn, not eleven; eight with the shear, below.
- **On height alone, the wind's shear is drawn too.** How fast the wind grows with height is an
  unknown there. It's an exponent stated anywhere from 0, a constant wind, to 0.2, IEC 61400-1's
  normal profile. The least-squares fit carries the same doubt in its errors instead.
- **On a track, the receiver's filter, noise and pad are unknowns too** ([ADR-321][adr-321]). A
  GNSS receiver smooths its raw fixes with a filter. The filter lags behind a hard boost and catches
  up after burnout ([GNSS in the boost](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md)).
  The posterior models it as a standard two-pole loop of damping $1/\sqrt{2}$, the usual
  choice for such a loop. The loop's speed is its natural frequency: the lower it is, the more the filter lags.
  Under a steady acceleration $a$ the lag settles at $a/\omega_n^2$. A 10 g boost leaves a 5 rad/s
  filter 3.9 m (13 ft) behind. A 1 rad/s one would trail by 98 m (322 ft), had the boost lasted
  long enough.
  - The frequency is anywhere from 0.2 to 5 rad/s, evenly in its logarithm. The synthetic
    receivers' run from 0.25 to 1 rad/s. The research note's simulation of a boost used 0.47 to
    0.69 rad/s. The loop speeds of the u-blox, Quectel and SkyTraq parts it read are unpublished,
    and no consumer receiver's filter lag has been measured against a truth in a real boost.
  - The track's noise runs on from one bin to the next by a correlation $\varphi$ anywhere from 0
    to 0.99. That's a first-order autoregression; its exact likelihood is J. D. Hamilton's (*Time
    Series Analysis*, 1994, §5.2). Its size takes Jeffreys' prior $1/\sigma$ and is integrated out in
    closed form (G. E. P. Box and G. C. Tiao, *Bayesian Inference in Statistical Analysis*, 1973,
    §2.4).
  - The pad's position is an offset of its own on each axis, under a flat prior, also integrated
    out in closed form.

  None of these is fitted, and nothing more is flown for them. Only the search for where the draws
  start flies more, about 21 flights a fit. If the fit is given a noise level rather than taking
  it from the log, the track is read the old way, as the least-squares fit states it.

  This model's limits: the synthetic receivers are filters of this same shape, inside the stated
  range. So the wind's pass below doesn't test the shape. The noise model, the filter, the pad and
  where the draws start were chosen after reading counts on the first 300 tracked flights of each
  set. All of set B's and 299 of set A's are among the 1,497 counted. A receiver of another shape is outside this model. So is one that also measures its
  speed from the satellites' Doppler shift and lags under 1 m (3.3 ft). No real flight has checked it.

A fit with something to fit flies about 1,000 times, against about 70 for least squares on set A.
On a log with a track, the search for the posterior's peak adds about 21. In one run on a 10-core
Mac, the whole report took 1 hour 56 minutes, against about 16 minutes by least squares alone. So each pull request checks every
21st fit, its posterior with it, and CI fits the whole each night
([In CI](../VALIDATION.md#in-ci-and-regenerating-the-references)).

### How well the Bayesian option holds

It's held to the same test as the least-squares fit, on the same flights
([How well it holds](#how-well-it-holds)). The counts are flights:

| Parameter | Set A below, inside, above (no unit) | Set B below, inside, above (no unit) |
|---|---:|---:|
| drag scale | 30, 887, 83: fails | 69, 877, 54 |
| impulse scale | 72, 878, 50 | 85, 881, 34: fails |
| wind toward the east, m/s | 59, 1,380, 58 | 58, 1,353, 86 |
| wind toward the north, m/s | 55, 1,384, 58 | 71, 1,345, 81 |
| mass on the weighed mass | 56, 882, 62 | 61, 870, 69 |

On the 1,000 flights of drag, impulse and mass, a tail passes with 29 to 74 truths. On the wind's
1,497, a tail passes with 49 to 104, and the inside with 1,308 to 1,384. What holds and what fails,
as far as it's known:

- **The wind passes on both sets** since the track is read as a receiver reports it
  ([ADR-321][adr-321]). Before, the posterior took the track's noise as the least-squares fit
  states it. Then from 1.06% to 2.41% of truths fell in each tail, where 5% belong. Its median width
  on set A is now 0.364 m/s (1.19 ft/s) east, against least squares' 0.345 m/s (1.13 ft/s). Set
  B's wind leans: 86 and 81 of its truths fall above, near the 104 that pass. Its mean error is
  −0.035 m/s (−0.11 ft/s) east. Set B's wind grows with height, and on a track the posterior's wind
  doesn't, which may be why; that isn't measured. Set A's wind is close to too wide: 1,380 and
  1,384 of 1,497 truths fall inside, and 1,384 is the most that pass.
- **The wind's pass leans on thin posteriors.** On the committed slice, every 21st fit, 294 wind
  intervals are counted. The 218 from posteriors of 100 or more effective draws miss 12 truths,
  5.5%, where 10% belong. The 76 under 100 miss 11, 14.5%. So the well-drawn intervals may still be
  too wide, and the thin ones, too narrow, may bring the tails up to 5%. The report doesn't split
  its counts this way yet.
- **Set A's drag sits low and set B's impulse high.** 83 of set A's drag truths fall above, and
  85 of set B's impulse truths below. 70 of set A's 83 above are on logs of height alone, 776 of
  its 1,000 flights, as before the track was read anew. Height-alone logs are the only ones where
  the shear is drawn, though set A's wind doesn't shear. That points at the shear's stated range,
  but it isn't measured yet, and set B's impulse leans the other way. Set A's drag is
  [M7.3c6][m7-3c6]; set B's impulse and the rest are [M7.3c7][m7-3c7].
- **Set A's impulse passes near its edge.** 72 of its truths fall below, 2 short of 74, the most
  that pass. That's the same side as set B's: it too reads high, which flatters the motor.
- **Set B's drag passes, but so does its widened probe.** The test can't tell its intervals from
  ones widened to 1.25 times, so they may be wider than they need be.

Drag and impulse are identified on set A, at median widths of 0.0919 and 0.0391 (least squares:
0.115 and 0.0399), of 0.36 and 0.108 stated. The mass isn't, at 0.034 of 0.036. On pure noise
nothing is identified by either method. But the reader or the fit turns away every noise log before a posterior is
drawn, so that says nothing of the sampler itself.

The draws are counted too. A posterior's effective draws are how many equally weighted draws its
weighted ones are worth (Kish): 400 draws with one heavy weight are worth about 1. Under 100, a
result is flagged as too few to read, but its interval still counts in the table above: so few
draws can leave an interval too narrow or off center, so take it as rough. All 10 tests, the 8
that pass and the 2 that fail, include those posteriors. On the wind, the slice suggests they
cause more than their share of misses (above). Of the
flights counted, 2,276 on set A had a posterior, 563 of them with fewer than 100 effective draws;
across all 2,276, the fewest was 1 and the median 153. On set B, 711 of 2,296 had fewer than 100;
the fewest was 1 and the median 138. That's about a quarter on set A and nearly a third on set B, the
loss on logs with a track: before the track was read anew, 319 and 392 fell short. A
draw fails when it can't be weighed, as when its flight can't fly. That's 126 of 1,138,000 on set A,
and 149 of 1,148,000 on set B. Every
flight counted with something to fit has a posterior. [M7.3c4][m7-3c4] is to make the counts pass.

### The sampler's tests

The sampler, `hpr_forensics::posterior`, is also tested on made-up distributions whose answer is
known. In one, the unknowns are $x$ and a noise term, each a standard normal, and $y$ is $0.5x^2$
plus the noise. $y$'s standard deviation is 1.22. A bell curve at the peak would put $y$'s median at
0 and its 90% interval at ±1.645, a standard normal's 5% and 95% points. With 160,000 draws, the
sampler gives a median of 0.43 and an interval of −1.35 to 2.57.

The tests hold each of those three within 0.08 of the true distribution's points, worked out by
Simpson's rule: about a fifteenth of $y$'s standard deviation. Drawn in two blocks, $x$ first, they
hold within 0.08, or 0.15 with the second block centered poorly. Other tests hold a correlated bell
curve within 0.06 standard deviations, show the sampler going wrong when the draw's own
likelihood is left out of the weights, and check that a pilot widens and never narrows. On the
fit, a straight line's drag, impulse and two winds come within a tenth of a standard error of their
closed forms. A result under 100 effective draws is flagged as too few to read. Run `cargo test -p fusionspace-hpr-forensics posterior` and
`cargo test -p fusionspace-hpr-forensics fit::tests`.

## Apogee from the boost

The library predicts a flight's apogee from its climb and the start of its coast: a log cut after
burnout, once the climb speed has kept falling for 0.75 s (`hpr_forensics::boost`). That is 1.08 s to 2.30 s after burnout on the synthetic
flights below, and 1.3 s (Bella Lui) and 3.5 s (NDRT 2020) after the burn on the two public ones.
It is run on 7 of the 12 public flights, those compared along their climb.
It predicts 2 of the 7, Bella Lui and NDRT 2020, both cut after the motor's burn.
That is too few to say at 95% whether it beats the forward simulation
([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/from-boost.md)).
The other 5 reach the speed limit described below before a cut.
The private collection's manifest keeps 72 flights in its apogee errors. Of them, 64 have a barometer's log
the analyzer reads an apogee from ([the count](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/collection-boost.md)).
26 are cut, 36 refused at the speed limit and 2 as sampled under 8 Hz.
Of those 26, the 20 whose motor's burn and log's liftoff are known are each cut after its thrust curve's burn. For the other 6 it isn't known.
Of the 26, 21 are predicted. HPR Sim's forward simulation refuses the design of 3, and 1 has no weather file for its day. There is nothing to fit or compare for those.
One more has too little up to its cut to fit. The flights are split into two halves by a hash of each flight's id
([ADR-322, held-out flights](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md)).
A third is held out in expectation and the rest are the calibration half; neither is blind, as the fixture report's forward errors on its 60 flights were published before the split.
On the 12 in the calibration half the prediction's mean absolute apogee error is 3.19%, against the forward simulation's
9.32% on the same flights. On the 9 held out it is 3.60%, against 16.75%. The difference is each flight's absolute
error from the prediction less the forward simulation's, averaged, in percentage points. Each half's paired bootstrap
95% interval on it lies below zero: −10.08 to −2.71 and −18.77 to −7.93 points. The numbers are in
[the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/collection-boost.md), and the
choices behind them in [ADR-329](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0329-m7-3d6-the-collections-errors-by-halves.md).
Both methods fly nothing deployed, and 10 of the 21 flights deployed something before apogee. Without them (8 and 3
flights) the difference is −7.0 and −7.3 points. Three flights are too few to resample, and their sign-flip interval
is unbounded. The fitted impulse scale averages 0.958 in the calibration half: the thrust curve's impulse times 0.958. That
is a fit's factor, not a measured impulse. It may take up some of what the model gets wrong in the boost, as the other
scales may.

[The same report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/collection-boost.md)
splits each flight's forward error at the cut. The numbers below are from it. The prediction's error is the share made
after the cut. The forward error less the prediction's is the share made up to it. There the motor's thrust, the
rocket's mass and the boost's own drag set the climb. On the calibration half that share averages +8.92 percentage
points of the logged apogee: the forward simulation's climb up to the cut put its apogee that much too high. It is
the larger share on 11 of the 12 flights. On the held-out half it averages +14.39 points and is the larger share on all 9. Without the flights
that deployed something before apogee it is +10.44 and +11.93 points. The prediction's own mean error is near zero,
−0.08% and +1.34%, so the share up to the cut is nearly the whole forward error. This is a split of two simulations'
errors, not a measurement of the motor. The fit also moves the drag, the mass and the wind within what it was told,
and those carry into the coast.

Each fit is told the motor delivers its rated impulse to a standard deviation of 0.033, and that pulls its impulse
scale toward 1. So each calibration log is fitted again with that range made 100 times wider, and the log alone
speaks. The 12 impulse scales then average 0.936, with a standard error of 0.014 on that mean, and all 12 are below 1.
That is 12 flights of one half of one collection: it says what these logs show, not what motors deliver. A
barometer's boost can't tell a motor that delivered less from a rocket built heavier than its files. That scale
carries both, and whatever the model gets wrong in the boost.

One of the 12 flights is flagged: its log and its files disagree by more than the limit below.
The cause could be the files, the motor or the model of the boost. The limit is 13.1%. Past that, NAR, one of the
bodies that certify motors, tests a motor again as a new type. NAR's rule is for a test firing against the certified
mean. Using it for a flown motor against its rated impulse is this project's inference, and the report gives the
source. The scale's distance from 1, less two of its own standard errors, must be more than the limit. A flagged
flight stays in every number on this page.
[ADR-346, the rule before the fit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0346-m7-5-in-three-the-rule-before-the-fit.md),
fixes how a correction is fitted on the calibration half. It also fixes how the correction is judged, on the held-out
half and the public flights. It was written after a look at the calibration half's fits and before any correction was
fitted or flown.

**The correction was fitted and judged, and it failed.** It is two scales, fitted on the calibration half: one on
every motor's impulse, one on the drag. The drag scale comes from the coasts. For each flight with nothing deployed
before apogee, a bisection finds the drag at which the boost, refitted with the drag held there, flies on to the
logged apogee. The scale is those values' mean, and its 95% interval comes from Student's t. The impulse scale comes
from the boosts, refitted at that drag. It is their weighted mean, and its interval comes from their weighted spread
([Merkatas and others, *Metrologia*, 2019](https://arxiv.org/abs/1905.09551), §3.1). A scale counts only if its interval excludes 1, and it is
rounded to two decimals.

Without the flagged flight, the drag's interval holds 1, so the drag stays as the model has it. The impulse scale
is 0.94: the boosts, on the files' masses, climb as if every motor gave 94% of its rated impulse. That correction
takes the held-out half's mean absolute error from 16.26% to 10.91%, and its mean error from +13.36% to +5.84%.
On the 12 public flights it takes the mean absolute error from 9.00% to 12.16%, and the mean error from −4.91%
to −12.01%. The rule said the public flights must get no worse, so the verdict is a fail and nothing ships. The fit
over every calibration flight fails the same way.
[ADR-348](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0348-m7-5b-the-correction-fails-on-the-public-flights.md)
records it. The collection reads high and the public flights read low, and one scale didn't serve both. That is
evidence against one bias shared by both sets of flights. It doesn't say where the collection's high reading comes
from. The report keeps the measurement for the work on causes, less what the rule holds back to protect the
collection's flights.

The [scoreboard](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md) lists the
calibration half's 3.19% as `accuracy.from-boost`, with which flights it counts
([ADR-330](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0330-m7-3d7-the-from-boost-scoreboard-row.md), the row).

**The cut.** The library follows the climb speed: a straight line fitted to each half second of
heights. One reading far from the rest is left out, as the analyzer's spike rule does. The cut needs
the speed to stay 10% below its peak, and at least 10 m/s (32.8 ft/s) below it, for 0.75 s. It
comes at the first sample where that holds and the speed is the lowest of those 0.75 s. It also never comes unless the fitted height stands more than 10 m (32.8 ft) above the pad.
The pad is the fitted height where the line last stood still: where that half second's fitted speed was
under 5 m/s (16.4 ft/s), up or down. Before that, it is the first window that gives a line, five samples or more.
Three bad readings in a row on the pad can lift the fit past 10 m (32.8 ft) for one sample. The fitted speed
then falls back and stays down, as it does after burnout, so this height rule stops a cut there.
They can also read a speed faster than the flight's own top. Once they leave the window the line
stands still again. The pad is read afresh there, and the top speed and climb read before it are dropped
([#1362](https://github.com/nrdptel/fusionspace-eridanus/issues/1362)). A step up on the pad that
stays settles the same way, at its new height. Bad readings close enough to ignition leave
no still window between them and the climb. They can then still be the top speed and cut the log early
([#1441](https://github.com/nrdptel/fusionspace-eridanus/issues/1441)); a test pins that gap as it stands.
On the test's flights that starts 0.2 s before ignition, and on one of them the cut then comes before burnout.
How far out it starts on other flights isn't bounded.
A fast climb doesn't stand still before its cut. A reading that dips in the boost can bring the speed
under 5 m/s (16.4 ft/s), and the pad is then read again there; the dip test below still finds every cut after burnout.
Two figures back the 5 m/s (16.4 ft/s), each measured once on sets A and B's first 200 synthetic flights and not committed.
No speed from liftoff to the cut read under 17.6 m/s (57.7 ft/s), and 98.8% and 98.4% of the pad's windows read still.
A climb whose top speed is under about 22 m/s (72 ft/s) reaches 5 m/s (16.4 ft/s) before its cut.
That is 5 m/s (16.4 ft/s), plus the 10 m/s (32.8 ft/s) fall, plus 0.75 s of gravity; drag raises it a little.
Such a slow climb is now refused as never lifting off; the walk before this rule cut some (measured once, not committed). A test refuses a climb whose top is 18 m/s (59 ft/s) and cuts one at 30 m/s (98 ft/s). After burnout gravity alone
takes 9.8 m/s (32.2 ft/s) off the speed every second, drag more, so the speed keeps falling. A
barometer's reading can dip during the boost and catch up
([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)). In the burns of the seven public flights'
logs compared first, the speed fell that far for up to 0.48 s, then recovered (measured once). The deepest such
fall was 63.2 m/s (207 ft/s), a one-off measurement not committed. The 0.75 s passes such a
dip. The catch-up can read above the true speed. On 778 of the 1,793 dipped logs below a fall lasted over 0.5 s, likely the fall back from that overshoot (not traced). But on the synthetic dips the speed
had begun to rise again while the motor burned, so it wasn't the lowest of the last 0.75 s. So on the
synthetic flights' motors the cut comes after burnout, and the log keeps the start of the coast. The log up to the cut is fitted, and the fit
flown on to apogee.

**What gets no cut.**

- A GNSS tracker's log. In the boost a receiver's altitude can drop out or lag the climb
  ([GNSS in the boost](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md)).
  One flyer reports a lag of up to 440 m (1,444 ft).
- A barometer's log whose fitted climb speed reaches 227.7 m/s (747 ft/s) before the cut, Mach 0.8 in the
  coldest air. Nearer the speed of sound ([transonic](../glossary.md#transonic-and-supersonic)
  flow), the pressure at the port may stop following the height. The analyzer withholds its top
  speed there too.
- A log sampled slower than 8 or faster than 500 times a second.
- A log that never climbs 10 m (32.8 ft) above where it last stood still.
- A climb whose top speed is under about 22 m/s (72 ft/s): it stands still again before its cut.
- A log whose speed never drops as far as the cut needs for 0.75 s.

[Set A](synthetic-flights.md#the-sets)'s first 200 synthetic flights are mostly fast. Of them, 51
were cut, each after the true peak of its vertical speed. Each cut came 1.08 s to 2.30 s after
burnout. 43 GNSS logs and 106 fast flights were refused. The slowest of those truly peaked at
221.3 m/s (726 ft/s): the fitted speed reads a little above the true one. Five single bad readings were tried, 12 to 100 m (39 to 328 ft) off. They sat at
the pad, the first sample, or halfway or four fifths through the burn. They moved the cut on 8 of the 255 spiked logs of cut flights (785 in all), none by more than 0.70 s (measured).
The same five were then tried as two and as three bad readings in a row. Of the 255 logs of cut flights at each length, two in a row
moved 9 cuts, by up to 0.70 s. Three in a row moved 23, by up to 1.35 s. Three in a row also left 40 of
those 255 logs uncut: the speed then read past 227.7 m/s (747 ft/s), so they were refused. Every cut in these
runs still came after the peak and after burnout. Four or more in a row weren't tried. Steps up on
the pad that stay were tried too, from halfway through it, on the 157 barometer logs of those 200 flights. Steps of 8 m (26 ft), 12 m (39 ft), 30 m (98 ft) and 60 m (197 ft)
each cut the same 51 flights, at the same sample as without the step. They were refused for their speed on the other 106. A test pins
that each cut lands at the clean log's sample; the counts are measured.
A step of 100 m (328 ft) was refused on all 157. Before the pad was read where the line stood still,
steps of 12 to 60 m (39 to 197 ft) cut 142 of those logs before the peak. That was measured once, not committed.
A test flies three logs it builds, each with a blip on the pad faster than the flight's top speed.
A fourth has a 15 m (49 ft) step. Each is now cut where its clean log is. Before, all four were cut
early, two of them before burnout, one 0.05 s after ignition. The test also moves the bad readings
across the pad, starting them anywhere from 1.50 s to 2.75 s, with ignition at 3.0 s. Each log is cut where its clean log is, after its peak and burnout,
or refused for its speed. From 2.80 s on these flights they can still be the top
([#1441](https://github.com/nrdptel/fusionspace-eridanus/issues/1441), the gap above). Without the height rule, three readings 12 m (39 ft) too high
in a row on the pad gave 33 cuts before the speed's peak, of the 157 logs spiked
there (all but the 43 GNSS logs; measured once, not pinned by a test). 32 came
before ignition, the other 0.04 s after it, and all 2.4 to 7.9 s before burnout. A test pins the order, the
spikes, the place of every cut and at least 51 cuts; the counts and times are measured.

A second test puts a dip in each cut flight's height: 0.2 to 0.6 s long, 10 to 40 m (33 to
131 ft) deep. It starts at ignition, or 0.3 or 0.6 of the way through the burn. Of 2,295 dipped logs,
1,793 have a speed that falls by the cut's amount (10% of the top, at least 10 m/s (33 ft/s)) before burnout. 1,642 of them were cut, 0.14 s to 2.30 s after
burnout, and 151 refused at the speed limit, which a catch-up can reach. None was cut in the burn.
On 797 dipped logs every fall lasted 0.5 s at most and recovered in the burn, as the public logs'
did. Those falls went as deep as 98.9 m/s (324 ft/s), past the public logs' 63.2 m/s (207 ft/s).
On 218 more a fall still ran at burnout. Its depth takes in the coast's, so it isn't counted.

**Why it can be trusted not to peek.** A test replaces every sample's value and time after the
cut with noise, and the prediction stays the same, bit for bit. It runs on three logs, one with a GNSS
receiver's channels beside its barometer. Letting the prediction read half a second more changes
it. A channel on a clock of its own is cut at its first time past the cut or out of order; the samples before that are read wherever the file puts them. Run
`cargo test -p fusionspace-hpr-forensics boost`.

**A worked example.** NDRT 2020's motor burns 3.43 s. Its barometer's log is cut 6.90 s after
liftoff, its climb speed at least 10% below its 168.5 m/s (553 ft/s) top for 0.75 s. That top
is 1.4 s into the coast, a reading above the speed at burnout. So the cut comes about 3.5 s after the burn ends. The fit to the log
up to there flies on to 1,273 m (4,177 ft), as the barometer reads it. The log's apogee is 1,320 m
(4,332 ft): the prediction is 3.6% low. The forward simulation, nothing fitted, reads 1,458 m
(4,782 ft), 10.4% high. ERA5's wind, averaged and held constant with height as the fit holds it,
moves it down 3.8 points: the rocket reaches 1,407 m (4,616 ft), 6.6% high. Fitting the boost
moves it down another 10.1 points, past the log, to the 3.6% low. The log up to the cut says
little of the drag, so the fit leaves it at 0.95, near the 1 it was told. It reads the impulse
0.955. The whole log reads the impulse 0.95 and the drag 0.71. The other flights are in
([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/from-boost.md)).

## Sources

- K. Levenberg, "A method for the solution of certain non-linear problems in least squares",
  *Quarterly of Applied Mathematics* 2, 1944.
- D. W. Marquardt, "An algorithm for least-squares estimation of nonlinear parameters", *Journal of
  the Society for Industrial and Applied Mathematics* 11, 1963.
- W. H. Press et al., *Numerical Recipes*, §15.5: the damped normal equations and the covariance.
- D. S. Wilks, *Statistical Methods in the Atmospheric Sciences*, 3rd ed., Academic Press, 2011,
  eq. 5.12: a correlated series' effective number of independent values.
- IEC 61400-1:2019, *Wind energy generation systems, part 1: design requirements*, §6.3.2.2: the
  normal wind profile, a power law of exponent 0.2.
- [ADR-316, the fit's decisions, with the measurements behind them][adr-316].
- J. Geweke, "Bayesian inference in econometric models using Monte Carlo integration",
  *Econometrica* 57(6), 1989, pp. 1317–1339: importance sampling, and a proposal's tails.
- L. Kish, *Survey Sampling*, Wiley, 1965: the effective sample size of weighted draws.
- M.-S. Oh and J. O. Berger, "Adaptive importance sampling in Monte Carlo integration", *Journal
  of Statistical Computation and Simulation* 41, 1992: widening the draws from a pilot's.
- [ADR-317, the Bayesian option's plan][adr-317].
- [ADR-320, the Bayesian option's decisions, with the measurements behind them][adr-320].
- J. A. Hartigan, "Using subsample values as typical values", *Journal of the American
  Statistical Association* 64, 1969, pp. 1303–1317: the exact sign-flip interval the public
  flights' report gives on the prediction's difference from the forward simulation.
- [ADR-325, the public flights' prediction, with the measurements behind it][adr-325].
- B. Efron and R. J. Tibshirani, *An Introduction to the Bootstrap*, Chapman & Hall, 1993, §13.3:
  the collection's percentile interval.

[adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
[adr-317]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0317-m7-3c-in-two-the-sampler-then-the-test.md
[adr-320]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0320-m7-3c2-the-bayesian-option-in-two-blocks.md
[adr-321]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
[adr-325]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[m7-3c4]: ../decisions-and-roadmap.md#m7-3c4
[m7-3c6]: ../decisions-and-roadmap.md#m7-3c6
[m7-3c7]: ../decisions-and-roadmap.md#m7-3c7
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fit-intervals.md
