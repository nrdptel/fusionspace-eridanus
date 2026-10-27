# The smoother

See how FusionSpace HPR estimates a flight's height and vertical speed at every sample. It uses
all of a log's sensors together, gives a 90% interval on each estimate, and this page shows how
well those intervals hold.

## In short

- **What it models:** a flight's height and vertical speed at every sample of a
  [flight log](../glossary.md#flight-log), from its barometer, axial accelerometer and GNSS
  receiver together. A Kalman filter runs forward over every sample. A Rauch-Tung-Striebel pass
  then runs back, so each moment uses the whole flight. Each estimate comes with a standard error
  and a 90% interval. A [health card](#the-health-card) says how each sensor fared. It is in the
  Rust library, `hpr_flightdata::smoother`.
- **Sources:** the textbook filter and pass back, and the project's notes on sensor noise, the
  port's Mach error and GNSS lag ([Sources](#sources)). The rest are HPR Sim's choices, recorded
  in [ADR-299, the smoother and its check][adr-299].
- **How well it is validated:** only on [the synthetic flights](synthetic-flights.md), whose
  truth is known. There its intervals hold the truth at least as often as they claim, and more:
  96% to 97.9% of set B's truths lie inside them ([How well it holds](#how-well-it-holds)). A real
  log comes with no truth taken apart from it. So the smoother is unvalidated on real logs
  ([evidence row G33](../evidence.md#g33)).
- **What it leaves out:** the horizontal track and the rocket's attitude. `hpr analyze` and Python
  get the smoother with [M7.2e, the 90% intervals on every reading](../decisions-and-roadmap.md#m7-2e).
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
hundreds of meters. Each barometer sample's noise is the larger of the pad's and the log's own
scatter about a two-second fit. A rocket swinging under its canopy can scatter its readings by
meters, far more than it showed at rest. Any sensor's sample is rejected when it misses the
estimate by more than five of its standard deviations.

A GNSS receiver lags a climb by up to hundreds of meters, all the while flagging its fix good. So
its fixes count only once its speed has agreed with the smoother's for 2 seconds.

## How well it holds

The check, run by `cargo xtask synthetic`, compares each flight's smoothed height and speed with
the truth. It does so at burnout, the top speed, apogee and one moment drawn per flight. On set
B, the share of the 845 smoothed flights whose truth lay inside the 90% interval was:

| Moment | Height | Speed | Median standard error |
|---|---:|---:|---:|
| Burnout | 97% | 97.6% | 58.6 m (192 ft), 11.1 m/s (36 ft/s) |
| Top speed | 97.8% | 97.2% | 61.6 m (202 ft), 10.6 m/s (35 ft/s) |
| Apogee | 97.9% | 97.5% | 84.8 m (278 ft), 0.759 m/s (2.49 ft/s) |
| A drawn moment | 97.8% | 96% | 19.2 m (63 ft), 0.881 m/s (2.89 ft/s) |

The intervals are wide on purpose. Most of the height's comes from the day's air, which a log
without a GNSS receiver can't measure, and from the port's error. The widest are on
barometer-only loggers at burnout and the top speed, up to 407 m (1,335 ft) and 190 m/s (623 ft/s).

Trusting the GNSS receiver from liftoff instead fails the check. At apogee, 151 of B's 845
truths then lay below the interval. Every set, logger and moment is in [the report][report].

## The health card

A barometer-only log from the tuning set, as the report prints it:

```text
barometer: 5593 samples, 5593 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.660; largest test ratio 0.547 at 141 s, Mach 0.01
barometer de-weighted: charge 27.9–29.9 s
pass back: every step smoothed
```

The test ratio is a sample's squared miss over five standard deviations squared. Above 1, the
sample is rejected. The mean NIS is the squared miss over its variance. It is near 1 when a
sensor's noise is what the smoother takes it to be. Below 1, the smoother takes the sensor as
noisier than it was.

## Sources

The filter and the pass back are the textbook ones (Särkkä and Svensson, *Bayesian Filtering and
Smoothing*, 2nd edition, 2023). The sensors' errors come from the [sensor noise note][noise]. The
port's error near Mach 1 comes from the [Mach error note][mach], and the GNSS receiver's lag from
the [GNSS note][gnss]. The rest are HPR Sim's choices, recorded in
[ADR-299, the smoother and its check][adr-299].

[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
[adr-299]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0299-the-smoother-and-its-check.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#the-smoother-against-the-truth
