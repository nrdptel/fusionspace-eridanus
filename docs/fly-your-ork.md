# Fly your .ork

**This guide flies a rocket you designed in [OpenRocket](glossary.md#openrocket) with `hpr sim`,
from the command line, and says how to read what it prints.** It is for a flier who has a `.ork`
file and wants a second simulator's view of it. Every flight here is a model's estimate, not a
measurement: [Accuracy](accuracy.md) says how close HPR Sim comes to OpenRocket and to real flights,
and its summary sits at the top of the [README](https://github.com/nrdptel/fusionspace-eridanus#accuracy-at-a-glance).
The outputs on this page are made by running each command, and CI checks that they still match
what `hpr` prints.

The steps:

1. [Fly the file as it is](#fly-the-file-as-it-is).
2. [Choose the configuration and the launch](#choose-the-configuration-and-the-launch).
3. [Fetch a motor the simulator doesn't have](#fetch-a-motor-the-simulator-doesnt-have).
4. [Plot it, or save the numbers](#plot-it-or-save-the-numbers).
5. [When the simulator refuses](#when-the-simulator-refuses).

You need `hpr` installed: the README's
[Install and first flight](https://github.com/nrdptel/fusionspace-eridanus#install-and-first-flight) takes a
few minutes. The examples fly the guides' own rocket,
[`validation/fixtures/ork/guides/level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork):
a 66 mm, 0.82 kg rocket with a 38 mm motor mount and one parachute on the motor's ejection
charge, saved with two motors, an AeroTech H170M and an I175WS. OpenRocket 24.12 opens it. Run
the commands from a copy of the repository, or put your own file's path in its place.

## Fly the file as it is

**`hpr sim` with the file's name flies its default configuration, with the motors, delays and
parachutes the file holds.** The guide rocket's motors are among the 32 built into the simulator, so
it flies with no network:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]

static margin         1.70 calibres off the rail; least 1.70 calibres, at 0.12 s, before apogee
CG and CP             0.811 m (31.9 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1065.1 m (3494 ft) above the site at 11.62 s
rail exit speed       26.3 m/s (86 ft/s)
delay                 apogee 9.48 s after burnout; the motor's set delay: 10 s; its charge fires 0.52 s after apogee
descent               4.4 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × H170M (the design's) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.14 s
launched at 0° N, 0° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                height                    speed
liftoff               0.00 s      0.4 m     (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.12 s      1.9 m     (6 ft)    26.3 m/s    (86 ft/s)
burnout               2.14 s    406.4 m  (1333 ft)   240.4 m/s   (789 ft/s)
apogee               11.62 s   1065.1 m  (3494 ft)     0.1 m/s     (0 ft/s)
charge               12.14 s   1063.7 m  (3490 ft)     5.1 m/s    (17 ft/s)
deployment           12.14 s   1063.7 m  (3490 ft)     5.1 m/s    (17 ft/s)
ground hit          245.78 s      0.0 m     (0 ft)     4.4 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             285.6 m/s (937 ft/s) at 1.73 s
top Mach number       0.842
landing               0.7 m (2 ft) from the pad at 245.78 s, at 4.4 m/s (15 ft/s)
```

<!-- cli: end -->

The lines under the configuration are the ones to read first. Each height and speed is in SI,
with feet or feet per second in parentheses:

- **`static margin`**: how far the [center of pressure](glossary.md#center-of-pressure-cp) sits
  behind the [center of gravity](glossary.md#center-of-gravity-cg), in body diameters
  ([calibres](glossary.md#calibre-caliber)), as the rocket leaves the rail, and its least value
  before apogee. A rocket with a fin set of one or two fins has a different margin for each
  direction the air crosses it, and `hpr sim` prints the least
  ([Flight metrics](physics/metrics.md#stability-margins)). [Check stability for a certification flight](stability-for-certification.md)
  says how to use it.
- **`CG and CP`**: where the center of gravity and the center of pressure sit as the rocket leaves
  the rail, the two points the margin comes from, measured from the nose tip.
- **`apogee`**: the highest point of the center of gravity, above the launch site.
- **`rail exit speed`**: the speed as the rocket leaves the rail. The slower it is, the more a
  crosswind tips the rocket just after.
- **`delay`**: the coast from burnout to apogee, beside the delay the file sets. Here the H170M's
  10 s charge fires about half a second after apogee. [Pick a motor](pick-a-motor.md#choose-the-delay)
  says how to choose one.
- **`descent`**: how fast the rocket comes down under its open parachutes, without the wind's
  drift.

The `recovery` line names each parachute, when it opened and its drag area; the events table
gives each event's time, height and speed. The `note` says how the parachute is flown: it opens
fully at once, as OpenRocket's does. [The command line](cli.md#flying-a-design) explains every
line, and any `warning` you may see.

## Choose the configuration and the launch

**`--config` picks another of the file's configurations, by its number, its name or its motors.**
The output's second and third lines list them. A configuration the file leaves unnamed is called
by its motors and delays in brackets, so the guide rocket's second is `[I175WS-9]`, and
`--config 2`, `--config I175WS-9` and `--config i175ws-9` all fly it.

**`hpr sim` does not read the launch conditions saved with the file's simulations: give them as
options.** Without them, the rocket flies from a 1.5 m vertical rail at sea level, at 0° N, 0° E,
in calm air. Set the site's elevation for every real field: air thins with height, and a rocket
climbs higher from a high one. This flies the second configuration from a field 1,400 m up, off a 2.4
m rail tilted 5° toward the west, in a 4 m/s west wind:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --config 2 --elevation 1400 --rail-length 2.4 --inclination 85 --heading 270 --wind 4 --wind-from 270`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --config 2 --elevation 1400 --rail-length 2.4 --inclination 85 --heading 270 --wind 4 --wind-from 270
Level 1 guide rocket (level-1.ork)
configuration 2 of 2: [I175WS-9]
help: --config flies the others, by number or name: 1 [H170M-10]

static margin         1.69 calibres off the rail; least 1.69 calibres, at 0.17 s, before apogee
CG and CP             0.811 m (31.9 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1174.3 m (3853 ft) above the site at 11.51 s
rail exit speed       36.8 m/s (121 ft/s)
delay                 apogee 9.54 s after burnout; the motor's set delay: 9 s; its charge fires 0.54 s before apogee
descent               4.9 m/s (16 ft/s) at landing under `Parachute`

motor: 1 × I175WS (the design's) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 10.97 s
launched at 0° N, 0° E, 1400 m (4593 ft) above sea level, from a 2.4 m (7.9 ft) rail 85° above the horizon, leaning toward 270°, in a 4 m/s (9 mph) wind from 270°
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.87 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.87 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                height                    speed
liftoff               0.02 s      0.4 m     (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.17 s      2.8 m     (9 ft)    36.8 m/s   (121 ft/s)
burnout               1.97 s    389.9 m  (1279 ft)   271.2 m/s   (890 ft/s)
charge               10.97 s   1172.0 m  (3845 ft)    15.4 m/s    (50 ft/s)
deployment           10.97 s   1172.0 m  (3845 ft)    15.4 m/s    (50 ft/s)
apogee               11.51 s   1174.3 m  (3853 ft)     1.0 m/s     (3 ft/s)
ground hit          246.17 s      0.0 m     (0 ft)     6.3 m/s    (21 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             290.9 m/s (954 ft/s) at 1.75 s
top Mach number       0.874
landing               755.8 m (2480 ft) from the pad at 246.17 s, at 6.3 m/s (21 ft/s)
```

<!-- cli: end -->

`--inclination` is the rail's angle above the horizon, so 85 is 5° off vertical (OpenRocket
gives the same rail as 5°, from the vertical), and `--heading 270` leans it toward the west, into the wind. The wind is the same at every
height. [The launch](cli.md#the-launch) lists every option and its default.

## Fetch a motor the simulator doesn't have

**A motor outside the simulator's 32 is fetched from [ThrustCurve.org](glossary.md#thrustcurveorg)
the first time you fly it, then kept in the simulator's cache, so it flies offline after.** A `.ork`
names its motor but rarely carries its [thrust curve](glossary.md#thrust-curve). With no network,
and no copy in the cache, `hpr sim` refuses and prints the command that fetches it. This test rocket
names an AeroTech H128W:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --offline`, exits 1; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --offline
error: configuration [H128W-0] can't be flown as the file has it: no thrust curve for H128W: no embedded curve, and no motor of that manufacturer and designation in the bundled catalog; AeroTech H128W is not in HPR Sim's cache of ThrustCurve.org, and the run is offline
help: with a network connection, `hpr motors fetch --manufacturer AeroTech H128W` fetches it into the cache
help: give a motor with --motor
$ echo $?
1
```

<!-- cli: end -->

Run that command once with a network connection, or fly the file once online:

```text
hpr motors fetch --manufacturer AeroTech H128W
```

The curve is ThrustCurve's, found by the file's manufacturer and designation. For the motors of
OpenRocket's own example designs, HPR Sim knows which file holds the curve OpenRocket flies and
takes it; for any other motor, the file may not be the one OpenRocket flies. A note under the flight
names the curve file, who measured it, and which of the two it is.
[Motors from ThrustCurve.org](cli.md#motors-from-thrustcurveorg) says how the simulator picks one.
If you have the motor's `.eng` or `.rse` file, fly it with `--motor`:
`hpr sim my-rocket.ork --motor AeroTech_H128W.eng`.

## Plot it, or save the numbers

**`--plot` draws the flight as a picture; `--export` saves every number.**

```text
hpr sim validation/fixtures/ork/guides/level-1.ork --plot level-1.svg --export level-1.csv
```

The plot is one fixed figure: altitude, speed and acceleration against time, each event marked.
An SVG opens in any web browser; [Plotting the flight](cli.md#plotting-the-flight) shows one. The
export holds every quantity the simulator tracks, every 0.01 s; the file's extension picks the
format (`.csv`, `.json`, `.parquet`, `.geojson` or `.kml`), and
[Exporting a flight](exporting-a-flight.md) says what each column means. `--json` prints the
summary itself as data, for a script to read.

## When the simulator refuses

**`hpr sim` refuses, with the reason, rather than fly something other than your design.** The
common reasons, and what to do:

| it says | what to do |
|---|---|
| no thrust curve for the motor, and the run is offline | fetch it once online, as [above](#fetch-a-motor-the-simulator-doesnt-have), or give a motor file with `--motor` |
| the design's checks found errors | each error names the parts, such as a motor wider than its mount. Fix the design; or, to fly it anyway, add `--accept-design-errors`, and the flight's notes list the errors |
| a part the simulator couldn't read exactly as written | The simulator flies only what it reads exactly. The message names the part; the [`.ork` page](format/ork.md) lists what it reads |
| a stage that separates, a second motor lit in flight | powered separations fly without `--motor`, one or several in turn, as on a three-stage rocket ([Separation](cli.md#separation)); with `--motor`, drop the option to fly the file's own motors. A payload dropped with nothing left to burn flies when its own parachute opens at the split; one that would coast with no drag first is refused: set one of the payload's devices to open at "Lower stage separation" with no delay ([M4.5g3](decisions-and-roadmap.md#m4-5g3), a payload's split) |
| it can't tell which configuration to fly | the file has several and no default: name one with `--config` |

Warnings are not refusals. A `warning:` line flags something the design's checks found unusual
but buildable, such as a motor longer than its mount tube, and the flight goes on. Warnings,
notes and `help:` lines go to standard error, so `hpr sim rocket.ork > flight.txt` saves the
flight alone and leaves them on the terminal.

## What it leaves out

- **The launch conditions in your file.** Give them as options,
  [above](#choose-the-configuration-and-the-launch).
- **Real weather.** One wind at every height, in the [standard atmosphere](glossary.md#standard-atmosphere).
  `hpr weather` fetches a launch day's profile, but `hpr sim` doesn't fly it yet
  ([#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265), flying a weather profile).
- **A fall with no parachute open.** The landing is then marked "not a prediction"
  ([the landing](cli.md#the-landing)).
- **A verdict.** The simulator prints estimates. Whether a rocket is fit to fly is for you, the
  motor's printed data and your range safety officer to decide.
