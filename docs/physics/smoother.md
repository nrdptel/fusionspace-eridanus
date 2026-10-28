# The smoother

See how FusionSpace HPR estimates a flight's height and vertical speed from all of a log's
sensors, and how well its 90% intervals hold.

## In short

- **What it models:** a flight's height and vertical speed at every sample of a
  [flight log](../glossary.md#flight-log), from its barometer, axial accelerometer and GNSS
  receiver together. A Kalman filter runs forward over every sample. A Rauch-Tung-Striebel pass
  then runs back, so each moment uses the whole flight. Each estimate comes with a standard error
  and a 90% interval. A [health card](#the-health-card) says how each sensor fared. It is in the
  Rust library, `hpr_flightdata::smoother`.
- **Sources:** the textbook filter and pass back, and the project's notes on sensor noise, the
  port's Mach error and GNSS lag ([Sources](#sources)). The rest are HPR Sim's choices, recorded
  in [ADR-300, the smoother and its check][adr-300].
- **How well it is validated:** only on [the synthetic flights](synthetic-flights.md), whose
  truth is known. There each set's intervals hold the truth at least as often as they claim:
  94.3% to 97.3% of set B's truths lie inside them ([How well it holds](#how-well-it-holds)). One
  logger's cell holds only 89.2% (set A's lowest is 89.7%), which the check's rule still allows. Sets
  A and B were run while its rules were set, so they aren't untouched tests. A real log comes with
  no truth taken apart from it, so the smoother is unvalidated on real logs
  ([evidence row G33](../evidence.md#g33)).
- **What it leaves out:** the horizontal track and the rocket's attitude. `hpr analyze` and Python
  get its intervals and health card with
  [M7.2e, the 90% intervals on every reading](../decisions-and-roadmap.md#m7-2e).
  A log that starts at launch has no pad to zero the barometer on, and is withheld
  ([an open issue](https://github.com/nrdptel/fusionspace-eridanus/issues/982)).

## How it works

The filter's state is the height above the pad and the vertical speed and acceleration. It also
holds the sensors' own errors: the accelerometer's bias, scale and tilt, the barometer's slow
noise and drift, and the receiver's offset. A log without an accelerometer, or with a clipped
one, runs the same model with nothing measuring the acceleration. Each estimate's 90% interval
is the estimate ± 1.645 standard errors.

Three things a log can't settle are carried into each interval instead of guessed:

- The day's air. A barometer reads height through the standard atmosphere. On a day 10 K warmer,
  every height it gives is about 3.5% short, and nothing else in the log can tell. Only a GNSS
  receiver's heights measure it.
- The static port's error. A port reads the flow as well as the air: its pressure coefficient
  times the dynamic pressure. The log is smoothed twice, the coefficient a standard deviation
  apart, and each interval is widened by the difference.
- Charges. Each charge's pressure pulse would read as a jump in height. The barometer is
  de-weighted for half a second after each charge the log names. It is also de-weighted around
  its own apogee in every log.

The barometer is de-weighted from Mach 0.8 too, because near Mach 1 the port's error can reach
hundreds of meters. Any sensor's sample is rejected when it misses the estimate by more than five
of its standard deviations.

A rocket swinging under its canopy can scatter its barometer's readings by meters, far more than
it showed at rest. So past apogee, each barometer sample's noise also holds the log's own scatter
about a smooth curve through six seconds of it. That scatter is taken as correlated over 2
seconds, so a swing isn't averaged away. Swings of 3 to 6 m (10 to 20 ft) that last up to a second stay inside
their intervals in the tests. A slower one, correlated over 3 seconds, is partly read as motion,
and its intervals miss more often than they claim.

A GNSS receiver lags a climb by up to hundreds of meters, all the while flagging its fix good. So
its fixes count only once its speed has agreed with the smoother's for 2 seconds.

## How well it holds

The check, run by `cargo xtask synthetic`, compares each flight's smoothed height and speed with
the truth. It does so at burnout, the top speed, apogee and one moment drawn at random per
flight. A cell fails if either side's misses are shown, at 99.9% confidence, to pass 5%, half of
what a 90% interval allows. Each set's cells and each logger's must hold.

The rules were set on the tuning set. But sets A and B were run as each change landed, and some
changes were found on them ([ADR-300][adr-300], item 13). On set B, 845 of the 1,000 flights were
smoothed; the other 155 logs have no barometer. The share whose truth lay inside the 90% interval
was:

| Moment | Height | Speed | Median standard error (height, speed) |
|---|---:|---:|---:|
| Burnout | 96.7% | 96.6% | 59.1 m (194 ft), 11.3 m/s (37 ft/s) |
| Top speed | 97.3% | 94.3% | 62.3 m (204 ft), 10.9 m/s (36 ft/s) |
| Apogee | 97.3% | 96% | 84.3 m (277 ft), 1.23 m/s (4.04 ft/s) |
| A drawn moment | 97.3% | 97.2% | 18.5 m (61 ft), 1.01 m/s (3.31 ft/s) |

The intervals are wide on purpose. Two terms are expected to lead the height interval: the
day's air, which a log without a GNSS receiver can't measure, and the port's error. The widest
are on barometer-only loggers, in the report's JSON per logger: up to 407 m (1,335 ft) at
burnout and 260 m/s (853 ft/s) at a drawn moment.

Across the three sets, each set's cells run from 94.3% to 99.4%. Per logger, the lowest cells
are 89.2% on set B and 89.7% on set A, both the GNSS logger's apogee height. The rule still
allows them: their misses aren't shown to pass 5% a side.

Trusting the GNSS receiver from liftoff instead fails the check. B's 186 smoothed logs with a
GNSS track were smoothed again that way, the rest left as they were. Then 146 of its 845 apogee
truths lay below the interval, and only 82% inside it. Every set, logger and moment is in
[the report][report].

## The health card

A barometer-only log from the tuning set, as the report prints it:

```text,wrap
barometer: 5593 samples, 5593 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.709; largest test ratio 0.585 at 141 s, Mach 0.01
barometer de-weighted: charge 27.9–29.9 s
pass back: every step smoothed
```

Each sample is fused into the estimate, rejected at the gate, or not weighed. A GNSS fix isn't
weighed before it is trusted, nor an accelerometer sample that is clipped or past apogee. A reset is a sensor taken back after a second of rejections. The test ratio is
a sample's squared miss over five standard deviations squared. Above 1, the sample is rejected.
The mean NIS is the squared miss over its variance. It is near 1 when a sensor's noise is what
the smoother takes it to be. Below 1, the smoother takes the sensor as noisier than it was. The
last line says whether the pass back solved every step.

## Sources

The filter and the pass back are the textbook ones (Särkkä and Svensson, *Bayesian Filtering and
Smoothing*, 2nd edition, 2023). The sensors' errors come from the [sensor noise note][noise]. The
port's error near Mach 1 comes from the [Mach error note][mach], and the GNSS receiver's lag from
the [GNSS note][gnss]. The rest are HPR Sim's choices, recorded in
[ADR-300, the smoother and its check][adr-300].

[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
[adr-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#the-smoother-against-the-truth
