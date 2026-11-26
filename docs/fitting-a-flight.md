# Fitting a simulation to a flight

`hpr fit` moves a simulation's drag, motor impulse, wind and mass until its flight passes through a
log's heights. It prints each fitted value with a 90% interval.

This page is for a flyer who has flown a rocket and wants to know what the flight says about the
simulation of it. Seeing how far the two lie apart, with nothing fitted, is
[Comparing a flight with its simulation](comparing-a-flight.md). The method and its tests are on
[the least-squares fit's page](physics/flight-fit.md).

> **How far to trust it.** A fitted scale is the factor that best fits this one flight. It isn't the
> rocket's drag at every speed. The fit takes the design, the motor's file, the rail, the site and the
> air as right, so whatever they or the simulation get wrong goes into the scales with nothing to say so.
> The 90% intervals were tested on [synthetic flights](physics/synthetic-flights.md) only, and no
> real flight has checked them ([what it leaves out](#what-it-leaves-out)). One fails its test:
> the drag scale's lower end reads too high more often than a 90% interval should
> ([the caveat](#the-caveat)).

The tests run the command on one public logged flight, NDRT's 2020 flight. That shows it runs on a
real log, not that its answer is right.

## Run it

Give the log, then the design, its motor and the launch, as `hpr sim` takes them
([the command line](cli.md#hpr-fit)). This example's log is invented. It is the repository's Level 1
guide rocket flown in a 6 m/s (13 mph) wind, with its heights read 4% low and rounded to a foot
([how it is made](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/logs/compare/make.py)).
The fit is told calm air, so you can see what it does with a lean it has no parameter for:

<!-- cli: example `hpr fit validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json`; written by `cargo xtask cli`; do not edit -->

```text,wrap
$ hpr fit validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json
level-1-flown.csv, its simulation fitted to it
  flown: level-1.ork, [H170M-10]
  as designed: the 1976 standard atmosphere and no wind, at the site
  each simulated height is read as a barometer on the pad would read it, as the log's is

Read from the log
  liftoff  10.10 s, within 0.89 s
           measured, from the logger's barometric altitude
  apogee   1015.6 m (3332 ft), within 76.9 m (252 ft), at 21.65 s, within 0.93 s
           measured, from the logger's barometric altitude
  heights  51 of 51 quarter-second bins from liftoff to apogee
           measured, from the logger's barometric altitude
  drift    none: the log holds no latitude and longitude

Fitted by least squares
                         fitted            90% interval      told before the log
  drag scale             1.046             1.004 to 1.084 *  1.000 ± 0.109
  impulse scale          0.951             0.919 to 0.985    1.000 ± 0.033
  burn time scale        1.037             none              1.000 ± 0.055
  wind toward the east   0.1 m/s (0 ft/s)  none              0.0 ± 3.9 m/s (0 ± 13 ft/s)
  wind toward the north  0.1 m/s (0 ft/s)  none              0.0 ± 3.9 m/s (0 ± 13 ft/s)
  structure mass scale   1.004             0.986 to 1.021    1.000 ± 0.011
  * This interval fails its test: the caveat below.
  The burn time scale states no interval: not asked of the fit.
  The wind states no interval: a log of height alone can't tell the wind.
  Caveat. The drag scale's 90% interval fails its test. On 1,000 synthetic flights flown with a
  model unlike the fit's own, 80 truths lay below the interval, where a 90% interval leaves 50. So
  its lower end can read too high: take the fitted drag scale as reading high, and a flight
  predicted with it as reading low. Recorded, not fixed:
  https://github.com/nrdptel/fusionspace-eridanus/issues/1108

The fitted flight
  heights' residual  0.8 m (3 ft) RMS, 0.2% of the heights' spread
  apogee             1014.3 m (3328 ft), top Mach 0.80; as stated, 1065.1 m (3494 ft)
  simulations flown  54

How far to trust it. Fitted: the simulation's drag, impulse, wind and mass moved until it passes
through the log, on the inputs you gave. A scale is the factor that best fits this one flight, not
the rocket's drag at every speed, and the simulation's own errors go into the scales with nothing to
say so. The 90% intervals were tested on synthetic flights only: no real flight has checked them.
Don't size a flight to a waiver or a field's ceiling from a fitted scale alone; the altimeter's own
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/fitting-a-flight.html
warning: drag: issue #18: skin friction is taken as fully turbulent; on a smooth surface a laminar run near the nose lowers it, by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, top speed and drift read low, and the flutter margin and largest dynamic pressure look better than they are
warning: stability: issue #172: the static margin read up to 0.1108 calibers above OpenRocket's on four private designs, cause unknown; this flight's may read high by as much
warning: stability: issue #1290: on one of two wind-tunnel models, the center of pressure sat 0.11 to 0.92 calibers aft of the tunnel's at Mach 0.6 and 0.8, as the report's curve reads; fitted over −3° to 4°, about 0.08 at 0.8. So from Mach 0.6 the flight margin may read high. This flight reaches Mach 0.80 at 1.8 s
help: issues: https://github.com/nrdptel/fusionspace-eridanus/issues/<number>
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
note: the log was read in the units your mapping or layout states, so the fit holds only if those are right
note: a log's main altitude, or a column you mapped, is taken as a barometer's: give --height-reads height-climbed for a GPS receiver's
note: the fit flies the climb with nothing open: a flight whose parachute opened before apogee was slowed by it, and the fit puts that into the drag scale
```

<!-- cli: end -->

The fitted flight passes through the log: its heights miss by 0.8 m (3 ft) RMS. It got there with
an impulse scale of 0.951 and a drag scale of 1.046. Neither is what happened. The motor gave its
whole impulse and the logger read low, and the fit has no parameter for a logger that reads low. A
close fit shows that some set of scales explains the log. It doesn't show that those scales are the
cause.

## Read the result

**Read from the log** lists what the fit took from the log. The liftoff and the apogee bound the
samples it fits, and the heights between them are averaged in quarter-second bins. A log with
latitude and longitude adds its drift. Under each is where it came from: the sensor, and whether it
was measured or derived, as [`hpr analyze`](reading-a-flight-log.md) reads it.

**Fitted by least squares** has a row for each parameter:

- The **drag scale** multiplies the rocket's drag with no lift, at every speed.
- The **impulse scale** multiplies the motor's total impulse, and the **burn time scale** its burn
  time. The burn time's scale is moved so that a longer or shorter burn doesn't lean on the other
  scales. It isn't asked of the fit, so it has no interval.
- The **wind** is one constant wind, toward the east and toward the north. A log of height alone
  can't tell a wind, so there it has no interval and shouldn't be read.
- The **structure mass scale** multiplies the mass of the rocket without its motors.

The **90% interval** is sized so that, over many flights, the true value lies inside 9 times in 10
([how well it holds](physics/flight-fit.md#how-well-it-holds)). It holds over a set of flights, not
on each one.

**Told before the log** is what the fit started from: the stated value, give or take one standard
deviation. Those ranges are the ones the intervals were sized with, so the command has no option to
change them. Compare each fitted value with it. A scale that lands several of those deviations away
is a sign that the design, the motor or the log isn't the flight's. In the example the impulse
scale sits more than one deviation low, which is the logger's lean.

**The fitted flight** gives the heights' residual, the fitted flight's apogee beside the apogee as
stated, and how many simulations the fit flew.

### The caveat

The drag scale's interval is marked, and a caveat under the table says why. On 1,000 synthetic
flights flown with a model unlike the fit's own, 80 true values lay below the interval, where a 90%
interval leaves 50. So the fitted drag can read high, and a flight predicted with it reads low:
the side that flatters a field's ceiling
([which miss flatters](trusting-a-simulation.md#which-miss-flatters)). I've recorded that gap and
not widened the interval to hide it
([issue #1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108)). The caveat prints
with the intervals until the interval passes its test.

### The warnings

After the result come the warnings `hpr sim` gives for the same flight: each known issue the fitted
flight meets, such as drag that reads high past Mach 0.8
([the accuracy note on every result](cli.md#the-accuracy-note-on-every-result)). They matter more
here. Where the simulation's drag is known to read wrong, the drag scale takes that error in.

## The launch day

Without weather options the fit flies the 1976 standard atmosphere in calm air. Give the launch
day's air and wind as `hpr compare` takes them: `--wind` and `--wind-from`, `--temperature`,
`--pressure`, a `--weather` file, or `--launch-time` for Open-Meteo's archive at the site
([Launch-day weather](weather.md)). The air is flown as given. The wind, averaged over the climb,
becomes what the fit is told before the log:

```text
hpr fit flight.pf2 rocket.ork --motor CTI-J350 --wind 4 --wind-from 270 --temperature 28
```

A log's main altitude is taken as a barometer's, and the simulated height is read the same way in
that air. If the height is a GPS receiver's, give `--height-reads height-climbed`.

## When it refuses

The command stops with its reason, and prints no numbers, when:

- the log has no apogee the reader will state, or too little of its climb
  ([when a log has nothing to fit](physics/flight-fit.md#when-a-log-has-nothing-to-fit));
- the flight separates into stages, which the fit doesn't follow;
- no flight of the design passes through the log. The best fit's residual is more than a tenth of
  the heights' spread. Check the design, its motor and `--height-reads`, and look at the two with
  `hpr compare`.

## From Python

`FlightLog.fit` runs the same fit and gives the same numbers as a dict ([Python](python.md)). Its
`text` is the result as lines to print, in SI units:

<!-- python: example -->
```python
from fusionspace import hpr

folder = "validation/fixtures/logs/compare"
log = hpr.FlightLog.read(f"{folder}/level-1-flown.csv", mapping=f"{folder}/level-1.mapping.json")
rocket = hpr.Rocket.from_file("validation/fixtures/ork/guides/level-1.ork", recovery=True)
fitted = log.fit(rocket, hpr.Environment(0.0, 0.0, 0.0), rail_length_m=1.5)
for row in fitted["parameters"]:
    interval = row["interval_90"]
    said = f"{interval['low']:.3f} to {interval['high']:.3f}" if interval else row["no_interval"]
    print(f"{row['parameter']}: {row['value']:.3f}, {said}")
drag = fitted["parameters"][0]
print(f"{drag['caveat']['below']} of {drag['caveat']['flights']} truths lay below the drag's")
print([warning["issue"] for warning in fitted["issue_warnings"]])
```

```text
drag_scale: 1.046, 1.004 to 1.084
impulse_scale: 0.951, 0.919 to 0.985
burn_time_scale: 1.037, not_asked
wind_east_m_s: 0.101, height_alone
wind_north_m_s: 0.116, height_alone
mass_scale: 1.004, 0.986 to 1.021
80 of 1000 truths lay below the drag's
[18, 172, 1290]
```
<!-- python: end -->

The environment's wind, averaged over the climb, is what the fit is told before the log. Each
parameter's `stated` holds that mean and its standard deviation, and `readings` holds what was read
from the log with each `source` and `provenance`.

## What it leaves out

- Everything after apogee, and the accelerometer. The fit reads heights, and a track if the log has
  one.
- A parachute that opened before apogee. The fit flies the climb with nothing open, so an early
  ejection's drag goes into the drag scale. A note says so.
- A wind that changes with height. The fitted wind is one constant wind.
- Staged flights, which are refused.
- A check on a real flight's fitted values against something measured apart from the log, such as a
  weighed mass or a static test's impulse. I have no such flight yet
  ([the register's row on the fit](evidence.md#g38);
  [the issue that asks for one](https://github.com/nrdptel/fusionspace-eridanus/issues/1104)).
- The [Bayesian option](physics/flight-fit.md#the-bayesian-option), which is in the Rust library
  only. More of its intervals fail their tests.

Don't size a flight to a waiver or a field's ceiling from a fitted scale alone. The altimeter's own
reading is the one to log, and the RSO decides.
