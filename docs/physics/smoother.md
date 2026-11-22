# The smoother

See how FusionSpace HPR estimates a flight's height and vertical speed from all of a log's
sensors, and how well its 90% intervals hold.

## In short

- **What it models:** a flight's height and vertical speed at every sample of a
  [flight log](../glossary.md#flight-log), from its barometer, axial accelerometer and GNSS
  receiver together. A Kalman filter runs forward over every sample. A Rauch-Tung-Striebel pass
  then runs back, so each moment uses the whole flight. Each estimate comes with a standard error
  and a 90% interval. A [health card](#the-health-card) says how each sensor fared. It is in the
  Rust library, `hpr_flightdata::smoother`. [`hpr analyze`](../cli.md#hpr-analyze) prints the
  smoothed apogee and the card, and Python's `FlightLog.smoothed()` gives the whole track.
- **Sources:** the textbook filter and pass back, and the project's notes on sensor noise, the
  port's Mach error and GNSS lag ([Sources](#sources)). The rest are HPR Sim's choices, recorded
  in [ADR-300, the smoother and its check][adr-300].
- **How well it is validated:** only on [the synthetic flights](synthetic-flights.md), whose
  truth is known. There, at burnout, the top speed, apogee and a drawn moment, 94.1% to 97.5% of
  set B's truths lie inside the 90% intervals ([How well it holds](#how-well-it-holds)). Two
  logger cells fall under 90%: 88.7% on set B and 80% on the tuning set, which the check's rule allows. Sets
  A and B were run while its rules were set, so they aren't untouched tests. A real log comes with
  no truth taken apart from it, so the smoother is unvalidated on real logs
  ([evidence row G33](../evidence.md#g33)). Tracking from the ground can't stand in for one: a
  contest's optical track gives one height and no speed. Its apogee scatters about 2% (one
  standard deviation) from a calibrated altimeter at best, and 3% to 5% typically. So it can expose
  a gross bias but can't test the 90% intervals ([the note on truth flights][truth]). A test
  range's best tracking is good enough, but no public hobby log was found with a range's track.
- **What it leaves out:** the horizontal track and the rocket's attitude. A log that starts at
  launch has no pad to zero the barometer on, so the pad is assumed, the card says so, and its
  accelerometer is set aside ([A log written from launch](#a-log-written-from-launch), and
  [evidence row G33](../evidence.md#g33)). A log too long to hold in memory is withheld. So is a
  track whose vertical speed passes 11,186 m/s (36,699 ft/s), Earth's escape speed. That is an estimate that
  ran away, not a rocket ([A track that runs away](#a-track-that-runs-away)).

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
what a 90% interval allows. Each set's cells and each logger's must hold, and a log withheld for
any reason but having no barometer fails the check.

The rules were set on the tuning set. But sets A and B were run as each change landed, and some
changes were found on them ([ADR-300][adr-300], item 13). On set B, 845 of the 1,000 flights were
smoothed; the other 155 logs have no barometer. The share whose truth lay inside the 90% interval
was:

| Moment | Height | Speed | Median standard error (height, speed) |
|---|---:|---:|---:|
| Burnout | 96.8% | 96.8% | 59.3 m (195 ft), 11.2 m/s (37 ft/s) |
| Top speed | 97.4% | 94.1% | 62 m (203 ft), 10.9 m/s (36 ft/s) |
| Apogee | 97.3% | 96% | 85.1 m (279 ft), 1.24 m/s (4.07 ft/s) |
| A drawn moment | 97.2% | 97.5% | 18.5 m (61 ft), 1.02 m/s (3.35 ft/s) |

The intervals are wide on purpose. Two terms are expected to lead the height interval: the
day's air, which a log without a GNSS receiver can't measure, and the port's error. The widest
are on barometer-only loggers, in the report's JSON per logger: up to 403 m (1,322 ft) at
burnout and 260 m/s (853 ft/s) at a drawn moment.

Across the three sets, each set's cells run from 94.1% to 98.8%. Per logger, the lowest cells
are 88.7% on set B and 90.3% on set A, both the GNSS logger's apogee height, and 80% on the
tuning set, the RRC3's speed at apogee: of its 30 truths, 6 lay above the interval. The rule
still allows them: their misses aren't shown to pass 5% a side.

The check also runs a setting made to fail. Each log with a GNSS track is smoothed again with
its receiver trusted from liftoff, which is wrong on purpose: a receiver lags a climb. `hpr
analyze` can't be set that way. The check must catch it, and it does. B's 186 smoothed logs with
a GNSS track were smoothed again that way, the rest left as they were. Six of them ran away and
were withheld ([A track that runs away](#a-track-that-runs-away)). That leaves 839 of B's 845
smoothed flights. Of those, 144 apogee truths lay below the interval, and only 82.4% inside it.
Every set and moment is in [the report][mutation].

### A track that runs away

The smoother withholds a track whose vertical speed passes 11,186 m/s (36,699 ft/s) anywhere in
the log, up or down. That is Earth's escape speed at its surface, the 11.186 km/s of
[NASA's Earth Fact Sheet](https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html). No
rocket that comes back flew it, so a track that reads it is an estimate that ran away. The
bound catches a runaway. It doesn't make a track under it right: the intervals and the health
card do that work. [The decision record][adr-365] has the measurements behind it.

**If your log is withheld.** `hpr analyze` prints `withheld:` on its `smoother` row and this
reason, with your log's speed and time in it:

```text
the smoother's vertical speed reads <speed> m/s at <time> s, past Earth's escape speed, 11186 m/s either way: the estimate ran away
```

That row takes the place of the smoothed apogee and the health card. The
[readings](log-readings.md) above it (liftoff, apogee, top speed, landing) are read from the
log, not from the smoothed track. Go by those. Of the [sample logs' 1,058 records][survey], 3
are withheld so: one `altos-csv` log and two `held-table` logs (`altos-csv` and `held-table`
are log families in that survey). Why those three run away isn't traced yet
([an open issue](https://github.com/nrdptel/fusionspace-eridanus/issues/1073)). It doesn't by
itself mean a sensor is broken. Before the bound, `hpr analyze` printed those three tracks, so
output saved earlier for such a log shows a runaway track.

**How the check's wrong setting runs away.** Each sample passes a gate: one that misses the
estimate by more than five of its standard deviations is rejected. A sensor rejected for a
whole second is taken back, which the [health card](#the-health-card) counts as a reset. The
smoother then treats the estimate, not the sensor, as wrong. It widens the uncertainty of the
height, speed and acceleration by the size of the gap, so the estimate jumps to the sensor.
That is right when the estimate has drifted and the sensor hasn't.

It goes wrong when two sensors of the height disagree past their gates and both can be taken
back. A take-back reads the gap as motion the filter missed and leaves the speed and
acceleration wide open. The other sensor's samples, fused against that, swing the speed. A
second later the next take-back can meet a wider gap, and the gap can grow without limit.

The barometer and the GNSS receiver both measure height. As shipped, the receiver is never
taken back. After two rejections in a row it is no longer trusted, and it must agree with the
smoother for 2 s again. The barometer can be taken back, and so can the accelerometer before
apogee, but those two don't measure the same thing. Trusted from liftoff, the receiver is taken
back as the barometer is, and the two can feed each other.

An invented flight shows it (the smoother's test
`a_track_past_escape_speed_is_withheld_as_diverged`). The rocket's top speed is 98.1 m/s
(322 ft/s). Its barometer logs at 20 Hz. Its receiver logs at 5 Hz and, from apogee on, its
altitude error grows by 20 m (66 ft) every second, as no real receiver's does. Trusted from
liftoff, the track is withheld as diverged. Trusted by agreement, as shipped, the same log
smooths. Its top speed reads within 2 m/s (6.6 ft/s) of the true 98.1 m/s (322 ft/s). At 30 s,
on the way down, the true height and speed lie inside their 90% intervals.

On the synthetic flights, the wrong setting runs away on 8 of the 185 logs set A smooths
again. It does so on 6 of set B's 186 and on 1 of the tuning set's 33
([the report][mutation]). Those logs leave the wrong setting's cells, and it still fails on the
rest, as it must. As shipped, the check fails if a single synthetic log with a barometer is
withheld, for this reason or another, and none is.

## A log written from launch

Many loggers write little before they detect the launch. Of the
[1,058 records of the sample logs][survey], 546 are smoothed: 372 on an assumed pad and 174 on their
own. A log with no zero to stand on, such as a track of pressures alone with no pad, gives no
liftoff and is still withheld. With fewer than five barometer samples on the pad, the smoother
assumes it:

- **The height** stands on the same zero as [the readings](log-readings.md): the logger's own, or
  whatever pad samples the log does hold. A zero that is the pad's only by a column's name may be
  off by up to 19.9 m (65 ft) either way. The height's standard error carries it as an even spread
  over that range, 11.5 m (38 ft).
- **The barometer's noise** comes from the flight's own scatter past apogee.
- **Nothing is held still.** The height, speed and acceleration start with wide priors, HPR Sim's
  choices: 100 m (328 ft), 100 m/s (328 ft/s) and about 30 g. Priors that wide would take a bad sample as
  the truth. So for the first second, or the first ten samples if that is longer, each sample fused
  is the median of the five about it, and the first two aren't weighed. The readings set aside one or
  two bad samples that stand high, so they don't move the readings' zero, except for the cases
  [the climb](log-readings.md#the-pad-and-the-climb) names, on the slowest logs among them.
- **The accelerometer is set aside** if it has too little of the pad to take its level from.

To test this, each synthetic flight's log is cut at liftoff and smoothed again. That keeps the
logger's true zero, so the test holds no error in that zero. On the [report][launch]'s sets A and B, every cell
holds, overall and per logger (the command fails otherwise): 94.7% to 98.7% of truths lie inside the
90% intervals overall. Real logs have no such truth ([evidence row G33](../evidence.md#g33)). Their
smoothed apogee falls outside the reading's bound on 11 of the 372 on an assumed pad and 19 of the
174 on their own. A log's own pad doesn't prevent it
([an open issue](https://github.com/nrdptel/fusionspace-eridanus/issues/1073)). The card then says
so, and the reading is the one to go by.

[launch]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#logs-written-from-launch
[survey]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md#the-smoother

## The health card

A barometer-only log from the tuning set, as the report prints it:

```text,wrap
barometer: 5625 samples, 5625 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.714; largest test ratio 0.646 at 141 s, Mach 0.01
barometer de-weighted: charge 28.1–30.1 s
pass back: every step smoothed
```

Each sample is fused into the estimate, rejected at the gate, or not weighed. A GNSS fix isn't
weighed before it is trusted, nor an accelerometer sample that is clipped or past apogee. A reset is a sensor taken back after a second of rejections. The test ratio is
a sample's squared miss over five standard deviations squared. Above 1, the sample is rejected.
The mean NIS (normalized innovation squared) is the squared miss over its variance. It is near 1
when a sensor's noise is what the smoother takes it to be. Below 1, the smoother takes the sensor as
noisier than it was, which isn't a fault. The
last line says whether the pass back solved every step.

Three more lines appear only when they apply: `pad assumed`, `accelerometer set aside`, and
`smoothed apogee ... past the reading's bound: go by the reading`.

On a log that ends on the climb, the smoothed apogee is a [floor](../glossary.md#floor) with no
interval, as the reading's is: the track's highest point is only where the log stopped, not the
rocket's peak. That holds where the reading's apogee is withheld too.

## Sources

The filter and the pass back are the textbook ones (Särkkä and Svensson, *Bayesian Filtering and
Smoothing*, 2nd edition, 2023). The sensors' errors come from the [sensor noise note][noise]. The
port's error near Mach 1 comes from the [Mach error note][mach], and the GNSS receiver's lag from
the [GNSS note][gnss]. The rest are HPR Sim's choices, recorded in
[ADR-300, the smoother and its check][adr-300].

[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
[truth]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/smoother-truth-flights.md
[adr-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#the-smoother-against-the-truth
[mutation]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#trusting-the-gnss-receiver-from-liftoff
[adr-365]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0365-a-smoothed-track-past-earths-escape-speed-is-withheld-as-diverged.md
