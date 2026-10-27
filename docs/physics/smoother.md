# The smoother

See how FusionSpace HPR estimates a flight's height and vertical speed at every sample from all
of a log's sensors together, with a 90% interval on each, and how well those intervals hold.

## In short

- **What it does:** it fuses a [flight log](../glossary.md#flight-log)'s barometer, axial
  accelerometer and GNSS receiver. A Kalman filter runs forward over every sample, and a
  Rauch–Tung–Striebel pass runs back, so each moment uses the whole flight. Each estimate comes
  with a standard error, and its 90% interval is the estimate ± 1.645 of them. A
  [health card](#the-health-card) says how each sensor fared.
- **Where it is:** the Rust library, `hpr_flightdata::smoother`. `hpr analyze` and Python get it
  with [M7.2e](../decisions-and-roadmap.md#m7-2e).
- **How well it is validated:** only on [the synthetic flights](synthetic-flights.md), whose
  truth is known. There its intervals hold the truth about as often as they claim
  ([How well it holds](#how-well-it-holds)). Real flights have no truth beside them, so it is
  unvalidated on real logs ([evidence row G33](../evidence.md#g33)).
- **Sources:** the filter and pass back are the textbook ones (Särkkä and Svensson, *Bayesian
  Filtering and Smoothing*, 2023). The sensors' errors come from the
  [sensor noise note][noise], the port's error near Mach 1 from the [Mach error note][mach], and
  the GNSS receiver's lag from the [GNSS note][gnss]. The rest are HPR Sim's choices, recorded in
  [ADR-299, the smoother and its check][adr-299].

## How it works

The filter's state is the height above the pad, the vertical speed and acceleration, and the
sensors' own errors: the accelerometer's bias, scale and tilt, the barometer's slow noise and
drift, and the receiver's offset. A log without an accelerometer, or with a clipped one, runs the
same model with nothing measuring the acceleration.

Three things a log can't settle are carried into each interval instead of guessed:

- **The day's air.** A barometer reads height through the standard atmosphere. On a day 10 K
  warmer, every height it gives is about 3.5% short, and nothing else in the log can tell.
  Only a GNSS receiver's heights measure it.
- **The static port's error.** A port reads the flow as well as the air, by its pressure
  coefficient times the dynamic pressure. The log is smoothed twice, the coefficient a standard
  deviation apart, and each interval widened by the difference.
- **Charges.** Each charge's pressure pulse would read as a jump in height. The barometer is
  de-weighted for half a second after each charge the log names, and around its own apogee in
  every log.

The barometer is also de-weighted from Mach 0.8, where the port's error near Mach 1 can reach
hundreds of meters. Each barometer sample's noise is the larger of the pad's and the log's own
scatter about a two-second fit around it: a rocket swinging under its canopy can scatter its
readings by meters, far more than it showed at rest. Each sensor's sample is rejected when it
misses the estimate by more than five of its standard deviations. A GNSS receiver lags a climb by up to hundreds of meters while
flagging its fix good, so its fixes count only once its speed has agreed with the smoother's for
2 seconds.

## How well it holds

The check, run by `cargo xtask synthetic`, compares each flight's smoothed height and speed with
the truth at burnout, the top speed, apogee and one moment drawn per flight. On set B, the share
of the 845 smoothed flights whose truth lay inside the 90% interval was:

| Moment | Height | Speed | Median standard error |
|---|---:|---:|---:|
| Burnout | 97% | 97.6% | 58.6 m, 11.1 m/s |
| Top speed | 97.8% | 97.2% | 61.6 m, 10.6 m/s |
| Apogee | 97.9% | 97.5% | 84.8 m, 0.759 m/s |
| A drawn moment | 97.8% | 96% | 19.2 m, 0.881 m/s |

The intervals are wide on purpose: most of the height's comes from the day's air, which a log
without a GNSS receiver can't measure. The widest are on barometer-only flights through Mach 1,
up to 407 m and 190 m/s. Trusting the GNSS receiver from liftoff instead fails the check: at
apogee 151 of B's 845 truths lay below the interval. Every set, logger and moment is in
[the report][report].

## The health card

A barometer-only log from the tuning set, as the report prints it:

```text
barometer: 5593 samples, 5593 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.660; largest test ratio 0.547 at 141 s, Mach 0.01
barometer de-weighted: charge 27.9–29.9 s
pass back: every step smoothed
```

The test ratio is a sample's squared miss over five standard deviations squared: above 1 it is
rejected. The mean NIS, the squared miss over its variance, is near 1 when a sensor's noise is
what the smoother takes it to be; below 1, the smoother takes it as noisier than it was.

[noise]: ../research/altimeter-sensor-noise.md
[mach]: ../research/altimeter-mach-error.md
[gnss]: ../research/gnss-in-boost.md
[adr-299]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0299-the-smoother-and-its-check.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#the-smoother-against-the-truth
