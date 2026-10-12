# A tour of HPR Sim

Fly a design file with HPR Sim's command line, read what it prints, scatter its inputs, and see how
it did on a real flight.

**This tour is for** a flier who has HPR Sim installed, or is deciding whether to install it, and
wants to see in a few minutes what it does with a design file: what it prints, how to read it,
and how far it has been from real flights. It flies the small rocket the how-to guides use, then
turns to a public flight that left a log. Each step links to the page that covers it in full.
[HPR Sim](the-simulator.md) says what the simulator is and how to install it.

> **How far to trust it.** Every number in this tour's text is read by a test from a committed
> report, and CI reruns its commands and redraws its pictures from what they print. The guide
> rocket's flights are simulations, not measurements: it has no flight log. The public flight at
> the end is one of 7 university rockets' logged flights, where HPR Sim's apogee missed by 6.04%
> on average without signs ([report][real-report]); they check apogee alone. The tour gives
> ranges, not verdicts; the safety code and the range safety officer (the RSO) decide whether a
> rocket flies.

## Fly a design file

**`hpr sim` flies a design file from the launch rail to the ground and prints what happened.**
The tour's rocket is
[`validation/fixtures/ork/guides/level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork),
a small rocket saved by OpenRocket with two motors to choose from. Its motors are in the catalog
built into HPR Sim, so it flies with no network. The picture shows the run as a terminal shows it,
with four parts marked:

![What `hpr sim` prints for the guide rocket, as a terminal shows it, with balloons A to D at the configuration line, the summary, the warnings and the table of events.](images/tour-sim.svg)

- **A, the configuration.** A design file can hold several motor configurations. With none named,
  `hpr sim` flies the file's default and says how to fly the others with `--config`.
- **B, the summary.** The static margin, the balance point and the center of pressure, the apogee,
  the speed off the rail, the motor's delay against the time to apogee, and the descent rate. The
  line under the configuration says what all of them are: a simulated flight, not a measurement.
- **C, the warnings.** Each names a known gap in HPR Sim's models that bears on this flight, the
  issue that tracks it, and which way it pushes the numbers. Here the drag may read high, so the
  apogee may read low: for a field's ceiling, the side that flatters
  ([Trusting a simulation](trusting-a-simulation.md#which-miss-flatters)).
- **D, the events.** Liftoff, rail exit, burnout, apogee, the charge, the parachute and the
  landing, each with its time, height and speed.

The same run as text, to copy or search:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork` -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]
a simulated flight, not a measurement

static margin         1.70 calibers off the rail; least 1.70 calibers, at 0.12 s, before apogee
CG and CP             0.811 m (31.9 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1065.1 m (3494 ft) above the site at 11.62 s
rail exit speed       26.3 m/s (86 ft/s)
delay                 apogee 9.48 s after burnout; the motor's set delay: 10 s; its charge fires 0.52 s after apogee
descent               4.4 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × H170M (from the bundled catalog, as of 2026-09-17) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.14 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.12 s      1.9 m      (6 ft)    26.3 m/s    (86 ft/s)
burnout               2.14 s    406.4 m   (1333 ft)   240.4 m/s   (789 ft/s)
apogee               11.62 s   1065.1 m   (3494 ft)     0.1 m/s     (0 ft/s)
charge               12.14 s   1063.7 m   (3490 ft)     5.1 m/s    (17 ft/s)
deployment           12.14 s   1063.7 m   (3490 ft)     5.1 m/s    (17 ft/s)
ground hit          245.78 s      0.0 m      (0 ft)     4.4 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             285.6 m/s (937 ft/s) at 1.73 s
top Mach number       0.842
landing               0.7 m (2 ft) from the pad at 245.78 s, at 4.4 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, each logged apogee the height climbed in the day's air, the simulated apogee
averaged 8.7% above the logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan
a waiver or a field's ceiling with room above this apogee; the altimeter's reading is the one to
log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

[Fly your .ork](fly-your-ork.md) covers choosing the configuration, the site, the rail and the
wind, and saving the flight to plot or to open in a spreadsheet.

## Scatter its inputs

**`hpr mc` flies the same file many times, each flight's inputs drawn afresh, and prints how far
the apogee and the landing spread.** A design file's masses, its drag and its motor are each a
little off from the real rocket's. The command takes each scatter as one standard deviation, as a
share of the value: `--mass-sd 0.02` for the mass, `--drag-sd 0.05` for the drag and
`--impulse-sd 0.03` for the motor's impulse. `--seed 2026` makes the draws the same on every run.

![What `hpr mc` prints for the guide rocket, as a terminal shows it, with balloons A to C at the apogee's spread, the landing ellipse and the note on how far to trust it.](images/tour-mc.svg)

- **A, the apogee's spread.** The nominal flight, then the runs' mean, their standard deviation,
  and their [percentiles](glossary.md#percentile): most runs fell between the `5%` and `95%`
  columns.
- **B, the landing ellipse.** The area that held half the landings, and the one that held most of
  them. In calm air, as here, the rocket comes down close to the pad.
- **C, how far to trust it.** The spread comes from the scatter the command was told to apply,
  not from errors in the models, which move every run together. This note gives the miss on real
  flights, which the spread leaves out.

The same run as text:

<!-- cli: example `hpr mc validation/fixtures/ork/guides/level-1.ork --runs 200 --seed 2026 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03` -->

```text
$ hpr mc validation/fixtures/ork/guides/level-1.ork --runs 200 --seed 2026 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]

200 simulated flights, seed 2026: 0 failed
                        nominal     mean  std dev       5%   median      95%
apogee (m AGL)           1065.1   1072.4     41.5   1006.2   1072.8   1137.3
apogee (ft AGL)            3494     3519      136     3301     3520     3731
landing distance (m)        0.7      0.7      0.0      0.7      0.7      0.8
landing distance (ft)         2        2        0        2        2        3

200 of 200 flights landed, centered 0.7 m (2 ft) west and 0.0 m (0 ft) north of the pad
landing ellipse                   semi-major            semi-minor  heading   flights inside
50%                             0.0 m (0 ft)          0.0 m (0 ft)   090° T            74.5%
95%                             0.1 m (0 ft)          0.0 m (0 ft)   090° T            98.0%
95%, the next flight            0.1 m (0 ft)          0.0 m (0 ft)   090° T            98.5%

motor: 1 × H170M (from the bundled catalog, as of 2026-09-17) in `Motor mount tube`, lit at launch
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
scattered, one standard deviation each: --mass-sd 0.02, --drag-sd 0.05, --impulse-sd 0.03
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag, the nominal flight: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag, the nominal flight: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag, the nominal flight: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability, the nominal flight: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

How far to trust it. Simulated from the design file, not measured; the spread above comes from the
inputs' scatter alone, not from the model's error. Against 53 logged flights of fliers' own designs,
each logged apogee the height climbed in the day's air, the simulated apogee averaged 8.7% above the
logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan a waiver or a field's
ceiling with room above this apogee; the altimeter's reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

[Monte Carlo dispersion](monte-carlo.md) covers the other scatters, the wind among them, and what
each column means.

## A real flight

The guide rocket has no log, so its numbers can't be checked against the sky. The public flights
in the [real-flights report][real-report] can: each is a university team's rocket, flown by HPR
Sim in a [reanalysis](glossary.md#reanalysis) of its day's weather and set against the team's
altimeter. The tour takes the middle one by a rule, not by choice: of the 7 flights, the one whose
miss, without its sign, is the median is Genesis, EuRoC 2023, the report's case *genesis*.
EuRoC is the European Rocketry Challenge. A rerun of the report that moves the middle names
another flight, and this page then fails its check until it follows.

Genesis's altimeter read 2,916.7 m (9,569 ft) at the top, 23.3 s after liftoff. An altimeter
riding HPR Sim's flight of its design would have read 2,746.0 m (9,009 ft), at 22.4 s: a miss
of −5.85%, so HPR Sim read low. For a waiver or a field's ceiling, a low reading is the side that
flatters: the rocket climbs past the height that was planned for. The report finds the miss
consistent with HPR Sim's drag: flown on the drag the team's example gives, the apogee missed by
−0.08%.

One flight says little alone, so here it is beside the rest. On the 7 flights HPR Sim's apogee
missed by −8.90% to +10.40% one by one, by 6.04% on average without signs, and by −0.25% with
them. That average is outside the project's own 5% target ([Accuracy](accuracy.md#real-flights)).
[Trusting a simulation](trusting-a-simulation.md) sets these beside 53 logged flights of fliers'
own designs, and says which miss to plan for.

**Sources.** The public flights are those RocketPy's documentation flies against their teams'
logs. The weather is ERA5, generated using Copernicus Climate Change Service information 2026, and
contains modified Copernicus Climate Change Service information 2026; neither the European
Commission nor ECMWF is responsible for any use made of it. The
[logged flights' report][fixture-report] gives the full attribution and the ERA5 citations.

## Where next

- **Your own rocket:** [Fly your .ork](fly-your-ork.md), then
  [Check stability for a certification flight](stability-for-certification.md) and
  [Pick a motor](pick-a-motor.md).
- **Every command and flag:** [The command line](cli.md).
- **From Python or Rust:** [Python](python.md) and [Getting started](getting-started.md).
- **Before you rely on a number:** [Trusting a simulation](trusting-a-simulation.md) and
  [Accuracy](accuracy.md).

[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
