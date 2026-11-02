# The least-squares fit

See how the library fits a flight's [log](../glossary.md#flight-log) to a simulation, and what that
tells of its drag, impulse, wind and mass.

## In short

- **What it models:** the simulator's inputs are changed until the simulated flight passes through
  the log's samples up to apogee, read the way the log's sensor reads it. It fits a scale on the
  drag and one on the impulse. It fits a constant wind in two components, and the mass within what
  the weighing says. Each comes back with a 90% interval.
- **Sources:** Levenberg-Marquardt as in *Numerical Recipes*; the widening for correlated residuals
  from Wilks; the wind's growth with height from IEC 61400-1 ([Sources](#sources)). The test of an
  interval is [ADR-257's][adr-257].
- **How well it is validated:** only on [synthetic flights](synthetic-flights.md), and only as a
  whole. Each parameter's interval passes on two sets of test flights
  ([the sets](synthetic-flights.md#the-sets)). Set A flies the simulator's own model; set B flies a
  deliberately different one. The wind passes only on logs with a track, the log's latitude and
  longitude ([How well it holds](#how-well-it-holds)). Choices in the model were made after seeing those two
  sets; the intervals' sizes come from a third. No real flight has checked it
  ([evidence row G38](../evidence.md#g38)). An interval holds 90% over a set, not on each design or
  flight: on set B, the Prometheus design's drag interval holds 68 of its 96 truths. Set B's drag
  passes at the edge of its test. On set B, "truth" is the same fit run on the flight's noise-free
  samples, since that model has no true drag scale. On a real flight, the model's own errors go into
  the scales with nothing to say so.
- **What it leaves out:** anything past apogee, and the accelerometer. A log whose only height is a
  GNSS receiver's isn't fitted, since the log reader takes no apogee from it; that's every log with
  nothing to fit among the sets' counted draws, about one in six ([when](#when-a-log-has-nothing-to-fit)). Its wind is
  constant with height; on a log of height alone, a wind that grows with height only widens the intervals. It's
  told the design, rail, site and the day's air, and takes them as right. It's in the library only:
  no command runs it on a log yet.

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

Flight 140 of set A flies the Calisto design, its getting-started variant with the motor at
−1.255 m (−4.12 ft), with a logger that holds a track. The report keeps only counts, so a test,
`the_pages_worked_example_is_a_real_flights`, fits this flight again and checks these rows.
Its heights fit to 0.97 m (3.2 ft), in 63 simulated flights. A factor is times the simulator's
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

[ADR-257][adr-257] sets the test. On each set, the truths below an interval, inside it and above it
are counted. Each count must fit a 99.9% Clopper-Pearson band (an exact binomial interval) around
5%, 90% and 5%. An
interval narrowed to 0.8, widened to 1.25, or shifted down one standard error must fail it.
[The report][report] holds every count and each design's.

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
  better ([#1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108)).
- Some fits stop short of the bottom of $\chi^2$. Among the flights counted and the wind's, 2 on set
  A and 7 on set B did, counting set B's truths' own fits.
- A wind on a log of height alone isn't counted and shouldn't be read. Height alone can't tell a
  steady wind from one that changes with height.

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

[adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0316-m7-3b-the-least-squares-fit.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fit-intervals.md
