# Fitting a simulation to a flight

`hpr fit` moves a simulation's drag, impulse, wind and mass until it passes through a log's
heights, and gives most of them a 90% interval.

This page is for a flyer who has flown a rocket and wants to know what the flight says about the
simulation of it. Seeing how far the two lie apart, with nothing fitted, is
[Comparing a flight with its simulation](comparing-a-flight.md). The method and its tests are on
[the least-squares fit's page](physics/flight-fit.md).

> **How far to trust it.** A fitted scale is the factor that best fits this one flight. It isn't the
> rocket's drag at every speed. The fit takes the design, the motor's file, the rail, the site and the air as right.
> Whatever they or the simulation get wrong goes into the scales, with nothing to say so.
> The 90% intervals were tested on [synthetic flights](physics/synthetic-flights.md) only, and no
> real flight has checked them ([what it leaves out](#what-it-leaves-out)). One fails its test:
> the drag scale's lower end reads too high more often than a 90% interval should
> ([the caveat](#the-caveat)).

On a log with latitude and longitude (a track) the fitted wind can be wrong, with a narrow interval
about it ([why](#the-wind-on-a-log-with-a-track)).

The tests run the command on one public logged flight, NDRT's 2020 flight, and on the invented log
below. CI's Linux jobs fetch that flight's log from RocketPy's data, so the tests can't pass it over
there. The public flight shows that the command runs on a real log, not that its answer is right. Its fitted drag scale lies below 0.8, outside the range
the intervals were tested over, and prints [marked so](#the-mark-on-a-value-outside-the-tested-range).

## Run it

Give the log, then the design, its motor and the launch, as `hpr sim` takes them
([the command line](cli.md#hpr-fit)). A log that is a plain table needs `--mapping`, the file that
says which column is which ([Reading a flight log](reading-a-flight-log.md#from-the-command-line)).
This example's log is invented. It is the repository's Level 1
guide rocket flown in a 6 m/s (13 mph) wind. Its heights are read 4% low and rounded to a foot
([how it is made](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/logs/compare/make.py)).
The fit is told the day's wind. It also solves for a barometer's scale
([the method](physics/flight-fit.md#how-it-works)), but is told that it is 1 give or take 0.14%,
one standard deviation. That is far less than 4%. See where the error goes:

<!-- cli: example `hpr fit validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json --wind 6`; written by `cargo xtask cli`; do not edit -->

```text,wrap
$ hpr fit validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json --wind 6
level-1-flown.csv, its simulation fitted to it
  flown: level-1.ork, [H170M-10]
  on the launch day: a wind entered of 6.0 m/s from 0°, the same at every height; the 1976 standard atmosphere's air
  each simulated height is read as a barometer on the pad would read it, as the log's is

Read from the log
  liftoff  10.10 s, within 0.89 s
           measured, from the logger's barometric altitude
  apogee   1015.6 m (3332 ft), within 76.9 m (253 ft), at 21.65 s, within 0.93 s
           measured, from the logger's barometric altitude
  heights  51 of 51 quarter-second bins from 1 s before liftoff to apogee
           measured, from the logger's barometric altitude
  drift    none: the log holds no latitude and longitude

Fitted by least squares
                         fitted      90% interval      told before the log
  drag scale             1.036       0.981 to 1.086 *  1.000 ± 0.109
  impulse scale          0.955       0.921 to 0.990    1.000 ± 0.033
  burn time scale        1.035       none              1.000 ± 0.055
  wind toward the east   0.0 m/s     none              0.0 ± 3.9 m/s
                         (0 ft/s)                      (0 ± 13 ft/s)
  wind toward the north  -6.4 m/s    none              -6.0 ± 3.9 m/s
                         (-21 ft/s)                    (-20 ± 13 ft/s)
  structure mass scale   1.003       0.985 to 1.022    1.000 ± 0.011
  * This interval fails its test: the caveat below.
  The burn time scale states no interval: not asked of the fit.
  The wind states no interval: a log of height alone can't tell the wind.
  So the fit is told a wind of at least 5.16 m/s (16.9 ft/s), the wind its intervals were sized in.
  Caveat. The drag scale's 90% interval fails its test. On 1,000 synthetic flights flown with a
  model unlike the fit's own, 80 truths lay below the interval, where a 90% interval leaves 50. So
  its lower end can read too high: take the fitted drag scale as reading high, and a flight
  predicted with it as reading low. Recorded, not fixed:
  https://github.com/nrdptel/fusionspace-eridanus/issues/1108

The fitted flight
  heights' residual  0.8 m (3 ft) RMS, 0.2% of the heights' spread
  apogee             1014.4 m (3328 ft), top Mach 0.80; as stated, 1058.3 m (3472 ft)
  simulations flown  52

How far to trust it. Fitted: the simulation's drag, impulse, wind and mass moved until it passes
through the log, on the inputs you gave. A scale is the factor that best fits this one flight, not
the rocket's drag at every speed, and the simulation's own errors go into the scales with nothing to
say so. The 90% intervals were tested on synthetic flights only: no real flight has checked them.
Don't size a flight to a waiver or a field's ceiling from a fitted scale alone; the altimeter's own
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/fitting-a-flight.html
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5). So the apogee, top speed and drift read low, and the flutter margin and largest dynamic pressure look better than they are. This flight reaches Mach 0.80 at 1.8 s
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9). So the apogee, top speed and drift read low, and the flutter margin and largest dynamic pressure look better than they are. This flight reaches Mach 0.80 at 1.8 s
warning: drag: issue #18: skin friction is taken as fully turbulent; on a smooth surface a laminar run near the nose lowers it, by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, top speed and drift read low, and the flutter margin and largest dynamic pressure look better than they are
warning: stability: issue #172: the static margin read up to 0.1108 calibers above OpenRocket's on four private designs, cause unknown; this flight's may read high by as much
warning: stability: issue #1290: on one of two wind-tunnel models, the center of pressure sat 0.11 to 0.92 calibers aft of the tunnel's at Mach 0.6 and 0.8, as the report's curve reads; fitted over −3° to 4°, about 0.08 at 0.8. So from Mach 0.6 the flight margin may read high. This flight reaches Mach 0.80 at 1.8 s
warning: stability: issue #1198: the center of pressure sat 0.67 to 1.35 calibers aft of a wind tunnel's on two models at Mach 0.9, so past Mach 0.8 the flight margin may read high. This flight reaches Mach 0.80 at 1.8 s
help: issues: https://github.com/nrdptel/fusionspace-eridanus/issues/<number>
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
note: the log was read in the units your mapping or layout states, so the fit holds only if those are right
note: a log's main altitude, or a column you mapped, is taken as a barometer's: give --height-reads height-climbed for a GPS receiver's
note: the fit flies the climb with nothing open: a flight whose parachute opened before apogee was slowed by it, and the fit puts that into the drag scale
```

<!-- cli: end -->

The fitted flight passes through the log: its heights miss by 0.8 m (3 ft) RMS. It got there with
an impulse scale of 0.955 and a drag scale of 1.036. Neither is what happened. The motor gave its
whole impulse and the logger read low. A close fit shows that some set of scales explains the log.
It doesn't show that those scales are the cause.

The line that starts "So the fit is told a wind of at least" prints on every log of height alone
([why](#the-wind-on-a-log-of-height-alone)).

## Read the result

A **scale** is a factor on what the design or the motor's file states. A scale of 1 leaves it as
stated, and 1.05 is 5% more.

**Read from the log** lists what the fit took from the log. It fits the heights from a second
before the liftoff to the apogee, in quarter-second bins, each bin holding the median of its
samples. A log with latitude and longitude adds its drift. Under each is where it came from: the
sensor, and whether it was measured or derived, as [`hpr analyze`](reading-a-flight-log.md) reads
it.

**Fitted by least squares** has a row for each parameter:

- The **drag scale** multiplies the rocket's drag with no lift, at every speed.
- The **impulse scale** multiplies the motor's total impulse, and the **burn time scale** its burn
  time. The burn time's scale is moved so that a longer or shorter burn doesn't lean on the other
  scales. Its interval was never tested, so it states none: "not asked of the fit".
- The **wind** is one constant wind, toward the east and toward the north. The example's wind
  blows from 0°, the north, so it is −6 m/s (−20 ft/s) toward the north. A log of height alone
  can't tell a wind. There it has no interval, and its fitted value mostly repeats what the fit
  was told ([the wind on a log of height alone](#the-wind-on-a-log-of-height-alone)).
- The **structure mass scale** multiplies the mass of the rocket without its motors.

The **90% interval** is sized so that, over many flights, the true value lies inside 9 times in 10
([how well it holds](physics/flight-fit.md#how-well-it-holds)). It holds over a set of flights, not
on each one. Each printed end is rounded outward, so the interval never claims less than it holds.

**Told before the log** is what the fit started from: the stated value, give or take one standard
deviation. Those ranges are the ones the intervals were sized with, so the command has no option to
change them. Compare each fitted value with it. A scale that lands several of those deviations away
is a sign that the design, the motor or the log isn't the flight's. In the example the impulse
scale sits more than one deviation low. In this invented log that comes from the heights read 4%
low, not from the motor.

**The fitted flight** gives the heights' residual, the fitted flight's apogee beside the apogee as
stated, and how many simulations the fit flew. Both apogees are the climb's as the fit flies it,
with no recovery device opening.

The **notes** at the end say what the fit took on trust. They name the units your mapping states
and the kind of height it took the log's for. They also say that nothing opened on the way up.
One note is `hpr sim`'s own, on how the file's recovery devices fly: the flight the warnings come
from is flown whole.

### The wind on a log of height alone

A wind from any direction lowers a flight's top, and heights alone can't tell how much of a low
flight is wind. The intervals were sized on fits told a wind of 5.16 m/s (16.9 ft/s)
([the method](physics/flight-fit.md#how-it-works)), and no test covers a fit told less. So on a log
with no latitude and longitude the fit is told a wind of at least that. A slower wind you give is
raised to it along its own direction, and calm air to that toward the east. A stronger wind stands as given, as the
example's 6 m/s (13 mph) does. A line under the table states the floor. A log with a track is told the wind
you give, with no floor.

The fit's last digits can differ from one computer to another. The outputs on this page were written
on macOS. The tests hold them exactly there. On Linux and Windows they check only the wording: any
number counts as a number, a run of spaces counts as one, and the warnings aren't compared.

### The wind on a log with a track

A log with latitude and longitude lets the fit state the wind's interval. Read it with care. The
fitted wind can settle far from the flight's, with a narrow interval about it.

A test shows it (`fit_can_settle_far_from_the_wind_a_track_was_flown_in`;
[the record of this command's choices](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0381-m7-3f-the-fit-on-the-command-line-and-in-python.md)).
A 54 mm (2.1 in) design flies its own flight in a wind of 9 m/s (20 mph). Fitted from calm air, it
comes out as a wind the other way. Its interval is under 2 m/s (7 ft/s) wide and misses the true
wind. The heights' residual doesn't show it: it lies within 0.5 m (1.7 ft) of the residual of the
fit told the true wind. Told the true wind, each component of the fitted wind stays within 1 m/s
(3.3 ft/s) of it. The wind's interval passes its test on the synthetic flights, so that test doesn't
see this. No test says how near the wind told has to be, and none isolates what a wind fitted wrong
does to the other scales. I haven't found the cause.

So give the day's wind, and take a fitted wind that disagrees with the field's as unsettled. Every
fit of a log with a track prints a caveat under its intervals that says so. Take the other scales
of such a fit as unsettled too. I've recorded the gap and not fixed it
([issue #1646](https://github.com/nrdptel/fusionspace-eridanus/issues/1646)).

### The mark on a value outside the tested range

The intervals were tested on [synthetic flights](physics/synthetic-flights.md) whose true values
were drawn from fixed ranges:

- the drag scale, from 0.8 to 1.2;
- the impulse scale, from 0.94 to 1.06;
- the burn time scale, from 0.9 to 1.1;
- the structure's mass, within 2% of the weighed one;
- the wind, up to 8.9 m/s (29 ft/s).

A fitted value outside its range prints with a **†**, and a line under the
table names the range. No test covers the fit's intervals out there, that parameter's or another's.
The mark is a limit on what was tested. It doesn't say the fit is wrong, and a fit with no mark
isn't shown right by it.

### The caveat

The drag scale's interval is marked with a `*`, and a caveat under the table says why. On 1,000
synthetic flights flown with a model unlike the fit's own, 80 true values, the message's "truths", lay below the interval,
where a 90% interval leaves 50. So the fitted drag can read high, and a flight predicted with it reads low:
the side that flatters a field's ceiling
([which miss flatters](trusting-a-simulation.md#which-miss-flatters)). I've recorded that gap and
not widened the interval to hide it
([issue #1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108)). The caveat prints
with the intervals until the interval passes its test.

### The warnings

After the result come the warnings `hpr sim` gives for the same flight, one for each known issue
the fitted flight meets. Drag that reads high past Mach 0.8 is one
([the accuracy note on every result](cli.md#the-accuracy-note-on-every-result)). They matter more
here. Where the simulation's drag is known to read wrong, the drag scale takes that error in.

## The launch day

Without weather options the fit flies the 1976 standard atmosphere. Give the launch day's air and
wind with the options `hpr sim` takes: `--wind` and `--wind-from`, `--temperature`, `--pressure`, a
`--weather` file, or `--launch-time` for Open-Meteo's archive at the site
([Launch-day weather](weather.md)). The air is flown as given. The wind, averaged over the climb,
becomes what the fit is told before the log. On a log of height alone a wind slower than 5.16 m/s
(16.9 ft/s) is raised to that ([why](#the-wind-on-a-log-of-height-alone)):

```text
hpr fit flight.pf2 rocket.ork --motor CTI-J350 --wind 4 --wind-from 270 --temperature 28
```

A log's main altitude is taken as a barometer's, and the simulated height is read the same way in
that air. If the height is a GPS receiver's, give `--height-reads height-climbed`.

## When it refuses

The command stops with its reason, and prints no numbers, when:

- the log has no apogee the reader will state, or its climb can't be read
  ([when a log has nothing to fit](physics/flight-fit.md#when-a-log-has-nothing-to-fit));
- the flight comes apart in the air, as a stage that separates does. The fit doesn't follow the
  parts;
- no flight of the design passes through the log. The best fit's residual is more than a tenth of
  the heights' spread, the cut
  [the method's page](physics/flight-fit.md#when-a-log-has-nothing-to-fit) gives. Check the design,
  its motor and `--height-reads`, and look at the two with `hpr compare`.

A fit that isn't refused can still be of the wrong rocket. In a test, a 54 mm (2.1 in) design passes the
Level 1 rocket's log to under 2 m (7 ft) RMS. Its drag scale comes out above 1.4 and its impulse
scale below 0.8. Both print marked as outside their tested ranges, and nothing else flags it.

## From Python

`FlightLog.fit` runs the same fit and gives the same numbers as a dict ([Python](python.md)). This
example fits the same log as the command above did. The site is at sea level, in standard air with
the same 6 m/s (13 mph) wind, and the rail is 1.5 m (4.9 ft). `recovery=True` reads the file's
parachute, so the warnings are the whole flight's. It prints each parameter with its interval or
the reason it has none, the caveat's counts, and how many known issues the fitted flight meets:

<!-- python: example -->
```python
from fusionspace import hpr

folder = "validation/fixtures/logs/compare"
log = hpr.FlightLog.read(f"{folder}/level-1-flown.csv", mapping=f"{folder}/level-1.mapping.json")
rocket = hpr.Rocket.from_file("validation/fixtures/ork/guides/level-1.ork", recovery=True)
site = hpr.Environment(0.0, 0.0, 0.0, wind_speed_m_s=6.0)
fitted = log.fit(rocket, site, rail_length_m=1.5)
for row in fitted["parameters"]:
    interval = row["interval_90"]
    said = f"{interval['low']:.3f} to {interval['high']:.3f}" if interval else row["no_interval"]
    print(f"{row['parameter']}: {row['value']:.3f}, {said}")
drag = fitted["parameters"][0]
print(f"{drag['caveat']['below']} of {drag['caveat']['flights']} truths lay below the drag's")
print(len(fitted["issue_warnings"]), "known issues met")
```

```text
drag_scale: 1.036, 0.981 to 1.085
impulse_scale: 0.955, 0.922 to 0.990
burn_time_scale: 1.035, not_asked
wind_east_m_s: 0.014, height_alone
wind_north_m_s: -6.375, height_alone
mass_scale: 1.003, 0.985 to 1.021
80 of 1000 truths lay below the drag's
6 known issues met
```
<!-- python: end -->

`wind_caveat` holds the [caveat on a track's wind](#the-wind-on-a-log-with-a-track), or `None` on
a log of height alone.
`not_asked` is the burn time's scale, whose interval was never tested, and `height_alone` is the
wind on a log with no latitude and longitude. The example rounds each end of an interval to the
nearest thousandth, where the command rounds it outward. The last line counts the
[warnings](#the-warnings): `issue_warnings` holds each one's `issue` number, `message` and `url`.
The environment's wind, averaged over the climb, is what the fit is told before the log. The same
floor holds on a log of height alone. Each parameter's `stated` holds that mean and its standard
deviation. `tested_range` holds the range the intervals were tested over, and
`outside_tested_range` says whether the value lies outside it. `readings` holds what was read from
the log, with each `source` and `provenance`. `fitted["text"]` is the whole result as
lines to print, in SI units.

## What it leaves out

- Everything after apogee, and the accelerometer. The fit reads heights, and a track if the log has
  one.
- A parachute that opened before apogee. The fit flies the climb with nothing open, so an early
  ejection's drag goes into the drag scale. A note says so.
- A wind that changes with height. The fitted wind is one constant wind.
- A logger that reads far low or high. The fit is told a barometer's scale is 1 give or take
  0.14%, and the example's 4% error shows up in the other scales.
- A fitted wind you can rely on, on a log with a track
  ([it can settle far from the flight's](#the-wind-on-a-log-with-a-track)).
- A flight that separates in the air, which is refused.
- A check on a real flight's fitted values against something measured apart from the log, such as a
  weighed mass or a static test's impulse. I have no such flight yet
  ([the register's row on the fit](evidence.md#g38);
  [the issue that asks for one](https://github.com/nrdptel/fusionspace-eridanus/issues/1104)).
- The [Bayesian option](physics/flight-fit.md#the-bayesian-option), which is in the Rust library
  only. More of its intervals fail their tests.

Don't size a flight to a waiver or a field's ceiling from a fitted scale alone. The altimeter's own
reading is the one to log, and the RSO decides.
