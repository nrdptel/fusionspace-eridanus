# Comparing a flight with its simulation

`hpr compare` shows how far a flight's logged height, speed and readings lie from a simulation of it.

This page is for a flyer or a team who has flown and wants to see where the simulation parted from the
flight. NASA Student Launch's handbook asks for this comparison after a team's subscale flight and again
after its full-scale flight. Reading a log with no simulation is [Reading a flight log](reading-a-flight-log.md).

> **How far to trust it.** A residual is one flight against one simulation, and it describes them. It is
> the sum of what the logger and the simulation each got wrong, and it can't tell them apart. It says neither
> is right. The simulated numbers state no error here, and the fit that would explain a residual is
> [not built yet](decisions-and-roadmap.md#m7-3). The method is tested on flights with a known answer. It has
> been run on the 7 public logged flights ([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md)),
> and those show what it gives, not that it is right about them.

## Run it

Give the log, then the design, its motor and the launch, as `hpr sim` takes them ([the command line](cli.md)):

```text
hpr compare flight.pf2 rocket.ork --motor CTI-J350 --wind 4
```

The simulation flies the conditions you give it. With no wind or air given, it flies a calm standard day,
which isn't the day you flew. `--json` adds every sample's residual, for a plot of your own. A logger HPR
Sim has no reader for is read by `--mapping` or `--layout`, as in
[Reading your own logger](your-own-logger.md).

## What a residual is

A **residual** is the logged value less the simulated one. A positive height residual means the log read
higher than the simulation did. A positive speed residual means the log read faster upward, so during
the descent it means a slower fall.

They come by phase, cut at the log's own burnout and apogee: boost, coast and descent, or ascent and
descent where the log's burnout can't be read. Each phase gives:

- the **mean**, with its sign, which shows a steady lean;
- the **RMS** (root mean square), which shows the size whatever the sign;
- the **largest**, with its sign and when it fell, which shows a single bad moment.

Here is a real one: Bella Lui's 2020 flight, from [the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md). Its ascent's height residual has a
mean of −0.2 m (−0.66 ft) and an RMS of 3.2 m (10 ft). Its descent's mean is −9.0 m (−30 ft) and its RMS
10.3 m (34 ft). So the simulation climbed with the log and then fell a little more slowly than it.

## How the two are made comparable

A logged height and a simulated one aren't the same quantity, so the simulation is read the way the log is.
HPR Sim writes the simulated flight as a noise-free log, at the real log's sample interval, and hands it to
the same reader. Three things follow.

- **The height.** A barometric logger reads the standard atmosphere's altitude of the pressure around it, so
  on a warm day it reads low. The simulated height is turned into what that barometer would read in the air
  you gave the simulation. A height from a GPS receiver is compared as a height climbed. The output says which.
- **The readings.** Apogee, top speed and the rest come through the same filters on both sides. Each logged
  reading keeps its bound, its 90% interval and whether it was measured or derived, as
  [`hpr analyze`](reading-a-flight-log.md) gives it. The simulated cell has none of these, as it wasn't measured.
- **The clocks.** A log's clock has no ignition on it, so the two line up by what the reader calls liftoff on
  each. The output gives the bound on that, from 1.31 to 3.21 s on [the public flights](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/flight-residuals.md). That is what the reader
  can promise of a liftoff, not a measured misalignment.

The heights are the log's own samples, not smoothed. An ejection charge's pressure spike, or a garbled last
row, shows as a residual. The speeds are the smoother's ([how it works](physics/log-readings.md)), each with
its standard error in `--json`. If the smoother can't read the log, the speed says why.

## What it leaves out

- A log that reads no landing stops at its end, and says so. A log that outlasts the simulated flight counts
  those samples and compares them with nothing.
- A design with no parachute falls ballistically after apogee, so a note says its descent means nothing.
  `hpr sim` flies no recovery device from a rocket's `.json`; an `.ork` or `.hpr` flies its own.
- On a staged flight the simulated track is the stack's, then the upper stage's, so a booster's log compares
  only up to the split.
- It doesn't draw a plot yet ([M7.3e](decisions-and-roadmap.md#m7-3e)), or fit anything
  ([M7.3b](decisions-and-roadmap.md#m7-3b)).

For a record or a certification, the altimeter's own reading is the one to log, and the RSO decides.
