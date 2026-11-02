# The least-squares fit

See how the library fits a flight's [log](../glossary.md#flight-log) to a simulation of it: what the
flight says about the rocket's drag, its motor's impulse, the wind and the mass, and how far each
could still be off.

## In short

- **What it models:** the simulator's inputs are changed until the simulated flight, read the way
  the log's sensor reads it, passes through the log's samples up to apogee. It fits a scale on the
  drag, a scale on the impulse, a constant wind in two components, and the mass within what the
  weighing says. Each comes back with a 90% interval.
- **Sources:** Levenberg-Marquardt as in *Numerical Recipes* ([Sources](#sources)). The test of an
  interval is [ADR-257's][adr-257].
- **How well it is validated:** only on [synthetic flights](synthetic-flights.md), and only as a
  whole. There, each parameter's interval passes on two sets of test flights ([How well it
  holds](#how-well-it-holds)). Choices in the model were made after seeing those two sets. The
  intervals' sizes come from a third. No real flight has checked it
  ([evidence row G38](../evidence.md#g38)). On a real flight, the model's own errors go into the
  scales with nothing to say so.
- **What it leaves out:** anything past apogee, the accelerometer, and a wind that changes with
  height. It's told the design, rail, site and the day's air, and takes them as right. It's in the
  library only: no command runs it on a log yet.

## How it works

The fit reads the log's heights, as a median every 0.25 s, from a second before liftoff to apogee.
Where the log holds latitude and longitude, it reads the drift as well, which is what speaks to the
wind. It flies the simulation about 70 to 120 times a fit, which takes about a second. It steps the
inputs by the Levenberg-Marquardt rule until the sum below stops falling:

$$\chi^2 = \sum_i \frac{(y_i - \hat y_i)^2}{\sigma^2} + \sum_k \frac{(r_k - \hat r_k)^2}{\sigma_r^2} + \sum_j \frac{(x_j - \mu_j)^2}{s_j^2}$$

Here $y_i$ is a logged height and $\hat y_i$ the simulated height, read as the sensor reads it.
$r_k$ and $\hat r_k$ are the same for the track, and $x_j$ is an input. The last sum holds each input
to what's stated of it, as a range: $\mu_j$ is the range's middle, and $s_j$ sets its width. That's
why a log with no word on the wind gives back the wind's stated range. $\sigma$ and $\sigma_r$ are
the fit's own residuals, at least 3 m (10 ft) on height and 10 m (33 ft) on the track.

A barometer's errors would otherwise land in the drag, so the fit also solves for them: the
reading's gain, its drift, and its port's pressure coefficient. It solves for the log's clock offset
and height offset too. Bins at or past Mach 0.8 are left out for a barometer. The burn time also has
a scale, not reported, since a motor's impulse and its burn trade against each other.

The parameters' standard errors come from the fit's curvature. Each 90% interval is a standard
error times two shares fitted on 2,000 other flights, the [calibration
set](synthetic-flights.md#the-sets). There's one pair of shares for logs of height alone and one for
logs with a track.

A log with nothing to fit, such as a track with no usable apogee, gives back its stated range and
says so.

### A worked example

Flight 140 of set A flies the Calisto design with a logger that holds a track. Its heights fit to
0.96 m (3.2 ft), in 63 simulated flights:

| Parameter | Fitted (factor) | 90% interval (factor) | Truth (factor) |
|---|---:|---:|---:|
| drag scale | 0.899 | 0.843 to 0.934 | 0.897 |
| impulse scale | 1.010 | 0.990 to 1.028 | 1.019 |
| mass on the weighed mass | 1.000 | 0.982 to 1.017 | 1.013 |

| Parameter | Fitted (m/s) | 90% interval (m/s) | Truth (m/s) |
|---|---:|---:|---:|
| wind toward the east, m/s | 4.48 | 4.18 to 4.80 | 4.59 |
| wind toward the north, m/s | −4.21 | −4.47 to −3.90 | −4.31 |

So this flight says its drag was about 10% under what the simulator took, and that the motor gave
about its rated impulse. It says the wind blew at about 4.5 m/s (14.7 ft/s) east and 4.2 m/s
(13.8 ft/s) south. The interval on the mass is nearly its stated ±2%: this flight doesn't weigh the
rocket any better than a scale does.

## How well it holds

[ADR-257][adr-257] sets the test. On each set, the truths below an interval, inside it and above it
must each fit a 99.9% Clopper-Pearson band around 5%, 90% and 5%. An interval narrowed to 0.8,
widened to 1.25, or shifted down one standard error must fail it. [The report][report] holds every
count and each design's.

Set B flies a different model from the one the fit flies. Its truth is the same fit, run on the
flight's noise-free samples. A wind's counts are on flights whose logs hold a track.

The counts are flights:

| Parameter | Set A below, inside, above (no unit) | Set B below, inside, above (no unit) |
|---|---:|---:|
| drag scale | 45, 897, 58 | 71, 893, 36 |
| impulse scale | 35, 907, 58 | 58, 913, 29 |
| wind toward the east, m/s | 77, 1,351, 69 | 81, 1,347, 69 |
| wind toward the north, m/s | 85, 1,336, 76 | 78, 1,352, 67 |
| mass on the weighed mass | 36, 912, 52 | 43, 906, 51 |

A log tells of a parameter when the median interval on set A is at most half the stated range's 90%
width:

| Parameter | Median width (factor) | Stated width (factor) |
|---|---:|---:|
| drag scale | 0.112 | 0.36 |
| impulse scale | 0.0425 | 0.108 |
| mass on the weighed mass | 0.0353 | 0.036 |

So it tells of the drag and impulse, and not of the mass. A wind's median width is 0.432 m/s
(1.42 ft/s) east and 0.446 m/s (1.46 ft/s) north, of 12.7 m/s (41.7 ft/s) stated, on logs with a
track. On pure noise nothing is told. The reader turns noise away, and noise of up to 1,000 m
(3,281 ft) either way, handed straight to the fit, leaves every interval at about its stated width.

Those counts have limits:

- Sets A and B were seen. The barometer's unknowns, the track's floor, the split of shares by track
  and set B's truth were all chosen after reading their counts. Only the shares themselves come from
  flights kept apart.
- Coverage holds over a set, not on each design. On set B, the Prometheus design's drag interval
  holds 69 of its 98 truths.
- Coverage holds over a set, not on each flight. A flight whose heights fit to 1 m (3.3 ft) or worse
  misses more often. Set B's drag interval holds 309 of 382 such flights, and 452 of 463 that fit
  better.
- Flights with nothing to fit are counted. They're 180 of set A's 1,000 and 155 of set B's, each
  stating its range. That range holds about 90% whatever the fit does, so they lean each count
  toward passing.
- A wind on a log of height alone isn't counted and shouldn't be read. Height alone can't tell a
  steady wind from one that changes with height.

## Sources

- K. Levenberg, "A method for the solution of certain non-linear problems in least squares",
  *Quarterly of Applied Mathematics* 2, 1944.
- D. W. Marquardt, "An algorithm for least-squares estimation of nonlinear parameters", *Journal of
  the Society for Industrial and Applied Mathematics* 11, 1963.
- W. H. Press et al., *Numerical Recipes*, §15.5: the damped normal equations and the covariance.
- [ADR-316, the fit's decisions, with the measurements behind them][adr-316].

[adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fit-intervals.md
