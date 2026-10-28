# Pick a motor

Shortlist the motors that fit your rocket, fly it on each one, and choose an ejection delay, all
with `hpr` on the command line.

**This guide shortlists the motors that fit a rocket, flies it on each, and picks an ejection
delay, all with `hpr` on the command line.** It's for a flier choosing a motor for a design they
already have.

> **How far to trust it.** HPR Sim's flights here are estimates from a model. A motor's data, as
> `hpr motors show` says, is copied from its curve file, not measured. Against 53 logged flights of
> fliers' own designs, HPR Sim's apogee averaged 8.7% above the logged one. It was within 10% on
> 26 and read low on 13, by at most 20%.
>
> `hpr sim` gives these figures under each flight below. [Accuracy](accuracy.md) says how close it
> comes elsewhere. The outputs on this page are made by running each command, and CI checks that
> they still match what `hpr` prints. The motor's printed data and your range safety officer have
> the last word.

The steps:

1. [List the motors that fit](#list-the-motors-that-fit).
2. [Fly the rocket on each](#fly-the-rocket-on-each).
3. [Compare them](#compare-them).
4. [Choose the delay](#choose-the-delay).
5. [Put the choice in your design](#put-the-choice-in-your-design).

The examples use the guides' own rocket,
[`level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork),
with a 38 mm motor mount. [Fly your .ork](fly-your-ork.md) introduces it.

## List the motors that fit

**Start from the mount's diameter: a motor's case must match it.** In OpenRocket, the motor mount
tube's inner diameter is the size. A 38 mm mount takes 38 mm motors. `hpr motors list` filters
the 1,004 motors built into the simulator by diameter, impulse class or maker. Here, Loki's 38 mm
H motors:

<!-- cli: example `hpr motors list --diameter 38 --class H --manufacturer Loki`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors list --diameter 38 --class H --manufacturer Loki
6 motors from ThrustCurve.org's solid motor catalog: every solid motor's record and every data file but a restricted one, captured 2026-10-28; size and delays from each motor's default file's header, the rest from its curve, or from ThrustCurve.org's record where no file of it is bundled.

designation  maker  class  dia mm (in)   len mm (in)  impulse N·s  avg N  burn s  delays
H100-SF      Loki   H        38 (1.50)    178  (7.0)        200.9  104.4    1.92  5-8-10-13-17
H90-LR       Loki   H        38 (1.50)    178  (7.0)        237.1   93.4    2.54  5-8-10-14
H144-LW      Loki   H        38 (1.50)    178  (7.0)        239.2  153.3    1.56  5-8-10-13-17
H125-CT      Loki   H        38 (1.50)  177.8  (7.0)        241.7  125.0    1.93  8-18
H160-LB      Loki   H        38 (1.50)    178  (7.0)        255.4  177.8    1.44  7-9-12-15
H500-LW      Loki   H        38 (1.50)    292 (11.5)        319.8  517.5    0.62  5-7-9-12-15
```

<!-- cli: end -->

The figures are worked out from each motor's default ThrustCurve.org curve file, the curve
the simulator flies ([The bundled motors](physics/motor.md#the-bundled-motors)). The `delays` column
lists the ejection delays as the file's header writes them, in seconds after burnout. A `-` separates
the settings: `5-9-13` is 5, 9 and 13 s, and `8-18` is 8 and 18 s. The header can list fewer delays
than the maker sells, so check the motor's instructions. `P` (or `1000`) is a plugged motor with no
ejection charge.

A Level 1 certification flight uses an H or I motor (NAR and Tripoli's Level 1 rules;
[Check stability for a certification flight](stability-for-certification.md#what-the-rules-ask)
quotes them).

**Every solid motor ThrustCurve.org listed on October 28, 2026 is built in,** such as
`--motor H128W`. The 80 with no curve file are listed but can't fly. A curve file more than 1% off ThrustCurve.org's figures flies with a
warning, as the H128W's does. A newer motor's curve is fetched once online and kept
([Fetch a motor the simulator doesn't have](fly-your-ork.md#fetch-a-motor-the-simulator-doesnt-have)).
`hpr motors search` lists what vendors have in stock and at what price
([Motors you can buy](cli.md#motors-you-can-buy)). `hpr motors show` gives one motor's figures as
the simulator reads its curve:

<!-- cli: example `hpr motors show I175WS`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors show I175WS
I175WS (AeroTech), from the bundled catalog
  impulse class    I (320.01–640 N·s)
  total impulse    333.2 N·s
  average thrust   177.5 N measured; 175 N nominal, as the designation names it
  peak thrust      252.6 N
  burn time        1.88 s, from 0.023 s to 1.900 s (NFPA 1125, 5% of peak)
  propellant       WS, the maker's code for Super White Lightning, as ThrustCurve.org names it
  propellant mass  168.0 g (5.9 oz) of 348.0 g (12.3 oz) loaded
  casing           38 mm (1.50 in) across, 214 mm (8.4 in) long
  delays           5 s, 9 s, 13 s
  curve file       https://www.thrustcurve.org/simfiles/5f4294d20002e900000008bf/ (flown)
                   RASP (.eng), from a certification test, public domain
                   passes the 1% check: total impulse +0.05%, average thrust +0.57%, burn time -0.66%
  curve file       https://www.thrustcurve.org/simfiles/5f9245ad1bca580004171795/
                   RockSim (.rse), from a user, public domain
                   passes the 1% check: total impulse +0.05%, average thrust +0.57%, burn time -0.66%

How far to trust it. Copied from ThrustCurve.org's curve file, downloaded 2026-10-28, not measured
by HPR Sim: the size, masses and delays from the file's header, the impulse, thrusts and burn time
from its curve, which can differ from the maker's rated figures. Each file is checked against
ThrustCurve.org's figures and OpenRocket's reading of its impulse and peak thrust, not against the
maker's data: checks of the file, not of the motor. The motor's printed data and its maker's
instructions come first, and the RSO decides.
More: https://hpr.fusionspace.co/pick-a-motor.html
```

<!-- cli: end -->

## Fly the rocket on each

**`--motor` flies the design with another motor in its mount, and `--delay` sets that motor's
ejection delay.** Without `--delay`, the motor fires no charge. A parachute set to open on the
charge, as the guide rocket's is, then never opens. Give one of the motor's delays, in seconds, or
`P` for a plugged motor when your electronics fire the charges. This flies the guide rocket on a
Loki H125, with a 10 s delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor H125-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]
a simulated flight, not a measurement

static margin         1.59 calibers off the rail; least 1.59 calibers, at 0.17 s, before apogee
CG and CP             0.818 m (32.2 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                914.5 m (3000 ft) above the site at 11.42 s
rail exit speed       25.9 m/s (85 ft/s)
delay                 apogee 9.42 s after burnout; the motor's set delay: 10 s; its charge fires 0.58 s after apogee
descent               4.7 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × H125-CT (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.00 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.02 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.17 s      1.9 m      (6 ft)    25.9 m/s    (85 ft/s)
burnout               2.00 s    303.8 m    (997 ft)   201.2 m/s   (660 ft/s)
apogee               11.42 s    914.5 m   (3000 ft)     0.1 m/s     (0 ft/s)
charge               12.00 s    912.8 m   (2995 ft)     5.6 m/s    (18 ft/s)
deployment           12.00 s    912.8 m   (2995 ft)     5.6 m/s    (18 ft/s)
ground hit          202.71 s      0.0 m      (0 ft)     4.7 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             217.0 m/s (712 ft/s) at 1.56 s
top Mach number       0.639
landing               0.7 m (2 ft) from the pad at 202.71 s, at 4.7 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

And on a harder-hitting I motor, a Loki I377, with the same delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor I377-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]
a simulated flight, not a measurement

static margin         1.18 calibers off the rail; least 1.18 calibers, at 0.08 s, before apogee
CG and CP             0.845 m (33.3 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1320.4 m (4332 ft) above the site at 12.02 s
rail exit speed       42.3 m/s (139 ft/s)
delay                 apogee 10.59 s after burnout; the motor's set delay: 10 s; its charge fires 0.59 s before apogee
descent               5.0 m/s (16 ft/s) at landing under `Parachute`

motor: 1 × I377-CT (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 11.43 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: design checks: the motor in `Motor mount tube` reaches 22.0 mm past the mount's forward end (configuration [H170M-10] with --motor I377-CT --delay 10)
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #222: supersonic pressure drag on an ogive nose or airfoil fins is about twice OpenRocket's, and which is right is unresolved. If HPR Sim's drag is high, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/222)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.3 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.08 s      1.8 m      (6 ft)    42.3 m/s   (139 ft/s)
burnout               1.43 s    384.0 m   (1260 ft)   335.1 m/s  (1099 ft/s)
charge               11.43 s   1318.2 m   (4325 ft)    10.1 m/s    (33 ft/s)
deployment           11.43 s   1318.2 m   (4325 ft)    10.1 m/s    (33 ft/s)
apogee               12.02 s   1320.4 m   (4332 ft)     0.0 m/s     (0 ft/s)
ground hit          269.86 s      0.0 m      (0 ft)     5.0 m/s    (16 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             371.7 m/s (1219 ft/s) at 1.08 s
top Mach number       1.095
landing               0.8 m (3 ft) from the pad at 269.86 s, at 5.0 m/s (16 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

The `warning` says the I377's case reaches 22.0 mm (0.87 in) past the mount tube's forward end.
The 292 mm (11.5 in) case sits 10 mm (0.39 in) out of the back of the rocket's 260 mm (10 in)
mount tube. The simulator flies it as drawn. Whether it fits the real rocket is for you to check.

`--motor` replaces the motor of the configuration flown. `--config` picks which of the file's
configurations it goes in, and `--mount` picks which mount, if the design has several.

## Compare them

**Read the same lines for each motor: margin, apogee, rail exit speed, delay and descent.**
Between the two flights above:

- The I377 climbs higher. If your field has a ceiling, its waiver sets it, not the simulator.
- The I377 leaves the rail much faster. The safety codes ask for a speed that ensures a stable
  flight but give no number. Competitions set their own, such as 25 m/s (82 ft/s) for the
  Spaceport America Cup
  ([Rail exit speed](stability-for-certification.md#rail-exit-speed) quotes them).
- The stability margin changes with the motor. A motor's mass sits behind the center of gravity.
  So a heavier motor moves the center of gravity aft, toward the center of pressure, and shrinks
  the margin. Check it with every motor you might fly
  ([Check stability for a certification flight](stability-for-certification.md)).
- The I377 takes the rocket past the speed of sound, Mach 1. The H125 stays well below it, and
  the H170M, at Mach 0.842, barely enters the range from Mach 0.8 to 1.2. In that range, HPR Sim's
  center of pressure is up to 2.36 calibers behind NASA's wind-tunnel data
  ([Known gaps](accuracy.md#known-gaps)). So its margin there reads high, and published guidance
  asks for a larger margin
  ([Compare with published guidance](stability-for-certification.md#compare-with-published-guidance)).

To fly a few more motors, change `--motor` and `--delay` and run it again. `--json` prints each
flight as data, for a script that tabulates many.

## Choose the delay

**The `delay` line's first figure is the coast, from burnout to apogee. A delay near it fires the
charge near apogee, where the rocket moves slowest.** The second figure is the delay you gave, and
the third says how far from apogee the charge then fires. In the events table, the `charge` row
gives the speed at that moment. The further the charge is from apogee, the faster the rocket moves
when the parachute comes out.

In the H125 flight above, the coast is 9.42 s, and the 10 s delay fires the charge 0.58 s after
apogee. The H125 comes in delays of 8, 10, 12, 14 and 18 s, more than its file's header lists, so
10 s is the nearest. The simulator flies any delay you give. Give one the maker sells, or one your
motor's delay can be adjusted to.

A delay is a burning fuse, and real ones vary from motor to motor. The simulator flies the stated
time exactly. [Monte Carlo dispersion](monte-carlo.md), in the library, can scatter it.

The guide rocket's own second configuration, `[I175WS-9]`, is an example where the set delay
fires before apogee:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --config 2`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --config 2
Level 1 guide rocket (level-1.ork)
configuration 2 of 2: [I175WS-9]
help: --config flies the others, by number or name: 1 [H170M-10]
a simulated flight, not a measurement

static margin         1.67 calibers off the rail; least 1.67 calibers, at 0.14 s, before apogee
CG and CP             0.813 m (32.0 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1100.7 m (3611 ft) above the site at 11.48 s
rail exit speed       28.9 m/s (95 ft/s)
delay                 apogee 9.51 s after burnout; the motor's set delay: 9 s; its charge fires 0.51 s before apogee
descent               4.5 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × I175WS (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 10.97 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.02 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.14 s      1.9 m      (6 ft)    28.9 m/s    (95 ft/s)
burnout               1.97 s    385.2 m   (1264 ft)   260.5 m/s   (855 ft/s)
charge               10.97 s   1099.1 m   (3606 ft)     8.0 m/s    (26 ft/s)
deployment           10.97 s   1099.1 m   (3606 ft)     8.0 m/s    (26 ft/s)
apogee               11.48 s   1100.7 m   (3611 ft)     0.0 m/s     (0 ft/s)
ground hit          247.48 s      0.0 m      (0 ft)     4.5 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             281.3 m/s (923 ft/s) at 1.75 s
top Mach number       0.830
landing               0.7 m (2 ft) from the pad at 247.48 s, at 4.5 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

## Put the choice in your design

**The simulator doesn't change your `.ork` file: set the motor and its delay in OpenRocket, save,
and fly the file again.** Without `--motor`, `hpr sim` flies the motors and delays the file holds,
as [Fly your .ork](fly-your-ork.md) shows. `hpr sim`'s `delay` line checks the delay you set.

## What it leaves out

- Each motor flies one thrust curve, ThrustCurve.org's, and its stated delay exactly. Real motors
  differ a little from their published curves.
- The simulator's design checks flag a motor wider than its mount, and warn on one longer than its
  tube. They don't know your motor retainer, thrust ring or closure.
- The simulator gives no verdict: it ranks nothing as safe or unsafe. The motor's printed data, the
  safety codes and your range safety officer decide.
