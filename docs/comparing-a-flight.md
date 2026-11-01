# Comparing a flight with its simulation

`hpr compare` shows how far a flight's logged height, speed and readings lie from a simulation of it.

This page is for a flyer or a team who has flown and wants to see where the simulation parted from the
flight. NASA Student Launch's handbook asks for this comparison after a team's subscale flight and again
after its full-scale flight. Reading a log with no simulation is [Reading a flight log](reading-a-flight-log.md).

> **How far to trust it.** A residual is one flight against one simulation, and it describes them. It is
> the sum of what the logger, the simulation and the lining up of their clocks each got wrong. It can't
> tell them apart. It says neither is right. The simulated numbers state no error here, and the fit that would explain a residual is
> [not built yet](decisions-and-roadmap.md#m7-3). The method is tested on flights with a known answer. It has
> been run on the 7 public logged flights ([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md)),
> and those show what it gives, not that it is right about them.

## Run it

Give the log, then the design, its motor and the launch, as `hpr sim` takes them ([the command line](cli.md)).
This example's log is invented. It is the repository's Level 1 guide rocket flown in a 6 m/s (13 mph) wind,
with its heights read 4% low and rounded to a foot ([how it is made](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/logs/compare/make.py)).
The simulation flies the same rocket in calm air, so you can see what a known lean looks like:

<!-- cli: example `hpr compare validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json`; written by `cargo xtask cli`; do not edit -->

```text,wrap
$ hpr compare validation/fixtures/logs/compare/level-1-flown.csv validation/fixtures/ork/guides/level-1.ork --mapping validation/fixtures/logs/compare/level-1.mapping.json
level-1-flown.csv against its simulation
  flown: level-1.ork, [H170M-10]
  clocks: a shared liftoff, within 1.73 s; the log's at 10.10 s, the simulation's 0.10 s after ignition
  the simulation's height is read as a barometer on the pad would read it, as the log's is

Residuals, logged less simulated
  height
    ascent   0.0 to 11.5 s, 116 samples
             mean -33.9 m (-111 ft), RMS 37.0 m (122 ft)
             largest -49.0 m (-161 ft) at 11.30 s
    descent  11.5 to 243.9 s, 2324 samples
             mean -26.8 m (-88 ft), RMS 29.5 m (97 ft)
             largest -48.9 m (-160 ft) at 11.60 s
  speed, up
    ascent   0.0 to 11.5 s, 116 samples
             mean -4.2 m/s (-14 ft/s), RMS 14.1 m/s (46 ft/s)
             largest -44.1 m/s (-145 ft/s) at 0.90 s
    descent  11.5 to 243.9 s, 2324 samples
             mean +0.2 m/s (1 ft/s), RMS 0.2 m/s (1 ft/s)
             largest +2.1 m/s (7 ft/s) at 12.00 s

Readings, the simulation read as the log is
  apogee, above the pad
    logged     1015.6 m (3332 ft) (measured), within 76.9 m (253 ft), 90% interval 966.6 to 1064.1 m (3171 to 3491 ft)
    simulated  1064.4 m (3492 ft)
    logged less simulated  -48.9 m (-160 ft)
  apogee's time, after liftoff
    logged     11.55 s (measured), within 1.81 s
    simulated  11.50 s
    logged less simulated  +0.05 s
  top speed
    withheld on both: the climb's speed reaches 269.4 m/s, at or above 227.7 m/s, where the barometer's port may read the transonic flow over the nose
  top acceleration
    withheld on both: the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g
  burnout, after liftoff
    withheld on both: burnout from the speed's peak needs the top speed, which is withheld: the climb's speed reaches 269.4 m/s, at or above 227.7 m/s, where the barometer's port may read the transonic flow over the nose
  flight time
    logged     243.90 s (measured), within 4.95 s, 90% interval 244.55 to 245.79 s
    simulated  245.20 s
    logged less simulated  -1.30 s
  drogue descent
    withheld on both: the descent shows no drogue's leg, which at the ground's density falls faster than 7.2 m/s up to the main's opening. That mark is 1.6 times the rate near the ground, 4.2 m/s, or 3 m/s more if that is more. A drogue's leg would also fall on median faster than 1.4 times the rate near the ground. So the rocket came down under one canopy, or under a drogue no faster than the mark, which reads as one. The main's descent is then the whole descent's, from apogee
  main's opening, above the pad
    withheld on both: the descent shows one canopy
  main descent
    logged     4.4 m/s (14 ft/s) (derived), within 0.4 m/s (2 ft/s), 90% interval 4.1 to 4.6 m/s (13 to 15 ft/s)
    simulated  4.5 m/s (15 ft/s)
    logged less simulated  -0.2 m/s (-1 ft/s)

How far to trust it. A log, measured, against a simulation of it, on the inputs you gave. A residual
is the logged value less the simulated one, the sum of what the logger, the simulation and the
lining up of their clocks each got wrong: it can't tell them apart, and it says neither is right.
The simulated numbers state no error here. Each reading states the bound its error stayed within on
HPR Sim's synthetic flights. They cover rates from 500 to 0.5 samples a second, steps up to 10 m or
100 Pa, and air within 15 K of standard. A reading the log can't support is withheld, with the
reason. The top speed is derived from the heights, and withheld from Mach 0.8, where a barometer's
port may read the flow over the nose. For a record or a certification, the altimeter's own reading
is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/comparing-a-flight.html
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
note: the log was read in the units your mapping or layout states, so its readings' bounds hold only if those are right
note: a log's main altitude, or a column you mapped, is taken as a barometer's: give --height-reads height-climbed for a GPS receiver's
```

<!-- cli: end -->

The height's mean residual is negative in both phases, because the log reads low. It is largest at apogee,
and it shrinks with the height on the way down. A reading says "withheld" when the log can't support it, with
the reason beside it, as here for the top speed and the acceleration. A reading marked "a floor" means the true value may lie any
amount above it.

The simulation flies the conditions you give it. With no wind or air given, it flies a calm standard day,
which isn't the day you flew. `--json` adds every sample's residual, for a plot of your own, and holds the notes.
A logger HPR Sim has no reader for is read by `--mapping` or `--layout`, as in
[Reading your own logger](your-own-logger.md).

## What a residual is

A **residual** is the logged value less the simulated one. A positive height residual means the log read
higher than the simulation did. A positive speed residual means the log read faster upward, so during
the descent it means a slower fall, and a negative one a faster fall. A descent's rate in the readings is a
size, not a velocity, so there a positive difference means the log fell faster.

They come by phase, cut at the log's own burnout and apogee: boost, coast and descent; or ascent and
descent where the log's burnout can't be read whole; or one flight where its apogee can't. Each phase gives:

- the **mean**, with its sign, which shows a steady lean;
- the **RMS** (root mean square), which shows the size whatever the sign;
- the **largest**, with its sign and when it fell, which shows a single bad moment.

Here is a real one: Bella Lui's 2020 flight, from [the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md). Its ascent's height residual has a
mean of −0.2 m (−0.6 ft) and an RMS of 3.2 m (10 ft). Its descent's mean is −9.0 m (−29 ft) and its RMS
10.3 m (34 ft). So the simulation climbed with the log and then fell a little more slowly than it.

## How the two are made comparable

A logged height and a simulated one aren't the same quantity, so the simulation is read the way the log is.
HPR Sim writes the simulated flight as a noise-free log, at the real log's sample interval (held between
0.01 and 2 s), and hands it to
the same reader. Three things follow.

- **The height.** A barometric logger reads the [standard atmosphere](glossary.md)'s altitude of the pressure
  around it, so on a warm day it reads low. The simulated height is turned into what that barometer would read
  in the air you gave the simulation. A log's main altitude, or a column you map as a pressure or an altitude
  above ground or sea level, is taken as a barometer's. If it is a GPS receiver's, give `--height-reads height-climbed`. A log holding only a GPS channel is refused, as
  its liftoff can't be read. The output says which reading it used.
- **The readings.** Apogee, top speed and the rest come through the same filters on both sides. Each logged
  reading keeps its bound, its 90% interval and whether it was measured or derived, as
  [`hpr analyze`](reading-a-flight-log.md) gives it. The simulated cell has no interval or provenance, as it wasn't
  measured. In `--json` it keeps a bound, which is only the reader's.
- **The clocks.** A log's clock has no ignition on it, so the two line up by what the reader calls liftoff on
  each. The output gives the bound on that, from 1.32 to 3.21 s on [the public flights](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md). That is what the reader
  can promise of a liftoff, not a measured misalignment. On a log with coarse steps, the log's liftoff reads
  late, so an ascent's height residual leans positive.

The heights are the log's own samples, not smoothed. An ejection charge's pressure spike, or a garbled row,
shows as a residual. The speeds are [the smoother's](physics/smoother.md), each with its standard error in
`--json`, with any GPS set aside on a barometric log, so they stay on the height's scale. If the smoother can't
read the log, the speed says why.

## What it leaves out

- A log that reads no landing stops at its end, and says so. A log that outlasts the simulated flight counts
  those samples and compares them with nothing.
- A one-stage design with no parachute falls ballistically after apogee, so a note says its descent means nothing.
  `hpr sim` flies no recovery device from a rocket's `.json`; an `.ork` or `.hpr` flies its own.
- On a staged flight the simulated track is the stack's, then the upper stage's, so a booster's log compares
  only up to the split.
- It doesn't draw a plot yet ([M7.3e](decisions-and-roadmap.md#m7-3e)), or fit anything
  ([M7.3b](decisions-and-roadmap.md#m7-3b)).

For a record or a certification, the altimeter's own reading is the one to log, and the RSO decides.
