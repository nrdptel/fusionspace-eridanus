# Pick a motor

**This guide shortlists the motors that fit a rocket, flies the rocket on each, and picks an
ejection delay, all with `hpr` on the command line.** It is for a flier choosing a motor for a
design they already have. hpr's figures are estimates from a model:
[Accuracy](accuracy.md) says how close they come, and the motor's printed data and your range
safety officer have the last word. The outputs on this page are made by running each command, and
CI checks that they still match what `hpr` prints.

The steps:

1. [List the motors that fit](#list-the-motors-that-fit).
2. [Fly the rocket on each](#fly-the-rocket-on-each).
3. [Compare them](#compare-them).
4. [Choose the delay](#choose-the-delay).
5. [Put the choice in your design](#put-the-choice-in-your-design).

The examples use the guides' own rocket,
[`level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork),
with a 38 mm motor mount; [Fly your .ork](fly-your-ork.md) introduces it.

## List the motors that fit

**Start from the mount's diameter: a motor's case must match it.** In OpenRocket, the motor mount
tube's inner diameter is the size; a 38 mm mount takes 38 mm motors. `hpr motors list` filters
the 32 motors built into hpr by diameter, impulse class or maker:

<!-- cli: example `hpr motors list --diameter 38`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors list --diameter 38
6 motors from ThrustCurve.org data files marked public domain, downloaded 2026-09-17; size, masses and delays from each file's header, the rest from its curve.

designation  maker     class  dia mm  len mm  impulse N·s  avg N  burn s  delays
G69N         AeroTech  G          38     106        136.3   72.1    1.89  1000
H170M        AeroTech  H          38     191        318.0  165.4    1.92  2,4,6,8,10,14
H125-CT      Loki      H          38   177.8        241.7  125.0    1.93  8-18
I175WS       AeroTech  I          38     214        333.2  177.5    1.88  5-9-13
411I175-14A  Cesaroni  I          38     245        411.4  174.1    2.36  6-8-9-11-12-13
I377-CT      Loki      I          38     292        525.8  377.9    1.39  8-18
```

<!-- cli: end -->

The figures are worked out from each motor's public-domain ThrustCurve.org curve file, the curve
hpr flies ([The bundled motors](physics/motor.md#the-bundled-motors)). The `delays` column lists
the ejection delays as the file's header writes them, in seconds after burnout, with `-` between
settings (`5-9-13` is 5, 9 and 13 s; `8-18` is 8 and 18 s), which can be fewer than the maker
sells: check the motor's instructions. `P` (or `1000`) is a plugged motor with no
ejection charge. A
Level 1 certification flight uses an H or I motor (NAR and Tripoli's Level 1 rules;
[Check stability for a certification flight](stability-for-certification.md#what-the-rules-ask)
quotes them).

**Many more motors exist than the 32 built in.** Name any motor ThrustCurve.org lists, such as
`--motor H128W`, and hpr fetches its thrust curve once online and keeps it
([Fetch a motor hpr doesn't have](fly-your-ork.md#fetch-a-motor-hpr-doesnt-have)).
`hpr motors search` lists what vendors have in stock and at what price
([Motors you can buy](cli.md#motors-you-can-buy)). `hpr motors show` gives one motor's figures as
hpr reads its curve:

<!-- cli: example `hpr motors show I175WS`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors show I175WS
I175WS (AeroTech), from the bundled catalog
  impulse class    I
  total impulse    333.2 N·s
  average thrust   177.5 N
  peak thrust      252.6 N
  burn time        1.88 s, from 0.023 s to 1.900 s (NFPA 1125, 5% of peak)
  propellant       168.0 g of 348.0 g loaded
  casing           38 mm across, 214 mm long
  delays           5 s, 9 s, 13 s
  curve file       https://www.thrustcurve.org/simfiles/5f4294d20002e900000008bf/ (public domain)
```

<!-- cli: end -->

## Fly the rocket on each

**`--motor` flies the design with another motor in its mount, and `--delay` sets that motor's
ejection delay.** Without `--delay`, the motor fires no charge, so a parachute set to open on the
charge, as the guide rocket's is, never opens. Give one of the motor's delays, in seconds, or `P`
for a plugged motor when your electronics fire the charges. This flies the guide rocket on a Loki
H125, with a 10 s delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor H125-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]

static margin         1.59 calibres off the rail; least 1.59 calibres, at 0.17 s, before apogee
apogee                914.5 m above the site at 11.42 s
rail exit speed       25.9 m/s
delay                 apogee 9.42 s after burnout; the motor's set delay: 10 s; its charge fires 0.58 s after apogee
descent               4.7 m/s at landing under `Parachute`

motor: 1 × H125-CT (from the bundled catalog) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.00 s
launched at 0° N, 0° E, 0 m above sea level, from a 1.5 m vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time     height       speed
liftoff               0.02 s      0.4 m     0.0 m/s
rail exit             0.17 s      1.9 m    25.9 m/s
burnout               2.00 s    303.8 m   201.2 m/s
apogee               11.42 s    914.5 m     0.1 m/s
charge               12.00 s    912.8 m     5.6 m/s
deployment           12.00 s    912.8 m     5.6 m/s
ground hit          202.71 s      0.0 m     4.7 m/s
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             217.0 m/s at 1.56 s
top Mach number       0.639
landing               0.7 m from the pad at 202.71 s, at 4.7 m/s
```

<!-- cli: end -->

And on the hardest-hitting motor in the list, a Loki I377, with the same delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor I377-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]

static margin         1.18 calibres off the rail; least 1.18 calibres, at 0.08 s, before apogee
apogee                1320.4 m above the site at 12.02 s
rail exit speed       42.3 m/s
delay                 apogee 10.59 s after burnout; the motor's set delay: 10 s; its charge fires 0.59 s before apogee
descent               5.0 m/s at landing under `Parachute`

motor: 1 × I377-CT (from the bundled catalog) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 11.43 s
launched at 0° N, 0° E, 0 m above sea level, from a 1.5 m vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: design checks: the motor in `Motor mount tube` reaches 22.0 mm past the mount's forward end (configuration [H170M-10] with --motor I377-CT --delay 10)
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #222: supersonic pressure drag on an ogive nose or airfoil fins is about twice OpenRocket's, and which is right is unresolved: if HPR Sim's is high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/222)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time     height       speed
liftoff               0.00 s      0.3 m     0.0 m/s
rail exit             0.08 s      1.8 m    42.3 m/s
burnout               1.43 s    384.0 m   335.1 m/s
charge               11.43 s   1318.2 m    10.1 m/s
deployment           11.43 s   1318.2 m    10.1 m/s
apogee               12.02 s   1320.4 m     0.0 m/s
ground hit          269.86 s      0.0 m     5.0 m/s
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             371.7 m/s at 1.08 s
top Mach number       1.095
landing               0.8 m from the pad at 269.86 s, at 5.0 m/s
```

<!-- cli: end -->

The `warning` says the I377's 292 mm case, which sits 10 mm out of the back of the rocket's
260 mm mount tube, reaches 22.1 mm past the tube's forward end. hpr flies it as drawn; whether it fits the real rocket is for you to check.
`--motor` replaces the motor of the configuration flown, so `--config` picks which of the file's
configurations it goes in, and `--mount` which mount, if the design has several.

## Compare them

**Read the same five lines for each motor: margin, apogee, rail exit speed, delay and descent.**
Between the two flights above:

- **Apogee.** The I377 climbs higher. If your field has a ceiling, its waiver sets it, not hpr.
- **Rail exit speed.** The I377 leaves the rail much faster. The safety codes ask for a speed
  that ensures a stable flight but give no number; competitions set their own, such as 25 m/s
  for the Spaceport America Cup
  ([Rail exit speed](stability-for-certification.md#rail-exit-speed) quotes them).
- **Stability margin.** It changes with the motor: a motor's mass sits behind the center of
  gravity, so a heavier motor moves the center of gravity aft, toward the center of pressure, and
  shrinks the margin. Check it with every motor you might
  fly ([Check stability for a certification flight](stability-for-certification.md)).
- **Top Mach number.** The I377 takes the rocket past the speed of sound, Mach 1; the H125
  stays well below it, and the H170M, at Mach 0.842, just reaches the range below. Between Mach
  0.8 and 1.2, hpr's center of pressure is up to 2.36 calibres behind NASA's wind-tunnel data, so
  its margin there reads high ([Known gaps](accuracy.md#known-gaps)), and published guidance asks
  for a larger margin
  ([Compare with published guidance](stability-for-certification.md#compare-with-published-guidance)).

Flying a few more motors is quick: change `--motor` and `--delay` and run again. `--json` prints
each flight as data, for a script that tabulates many.

## Choose the delay

**The `delay` line's first figure is the coast, from burnout to apogee: a delay near it fires the
charge near apogee, where the rocket moves slowest.** The second figure is the delay you gave, and
the third says how far from apogee the charge then fires. In the events table, the `charge` row
gives the speed at that moment: the further the charge is from apogee, the faster the rocket
moves when the parachute comes out.

In the H125 flight above, the coast is 9.42 s, and the 10 s delay fires the charge 0.58 s after
apogee. The H125's delays are 8, 10, 12, 14 and 18 s, so 10 s is the nearest. hpr flies any
delay you give; give one the maker sells, or one your motor's delay can be adjusted to. A delay is a
burning fuse, and real ones vary from motor to motor; hpr flies the stated time exactly.
[Monte Carlo dispersion](monte-carlo.md), in the library, can scatter it.

The guide rocket's own second configuration, `[I175WS-9]`, is an example where the set delay
fires before apogee:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --config 2`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --config 2
Level 1 guide rocket (level-1.ork)
configuration 2 of 2: [I175WS-9]
help: --config flies the others, by number or name: 1 [H170M-10]

static margin         1.67 calibres off the rail; least 1.67 calibres, at 0.14 s, before apogee
apogee                1100.7 m above the site at 11.48 s
rail exit speed       28.9 m/s
delay                 apogee 9.51 s after burnout; the motor's set delay: 9 s; its charge fires 0.51 s before apogee
descent               4.5 m/s at landing under `Parachute`

motor: 1 × I175WS (the design's) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 10.97 s
launched at 0° N, 0° E, 0 m above sea level, from a 1.5 m vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time     height       speed
liftoff               0.02 s      0.4 m     0.0 m/s
rail exit             0.14 s      1.9 m    28.9 m/s
burnout               1.97 s    385.2 m   260.5 m/s
charge               10.97 s   1099.1 m     8.0 m/s
deployment           10.97 s   1099.1 m     8.0 m/s
apogee               11.48 s   1100.7 m     0.0 m/s
ground hit          247.48 s      0.0 m     4.5 m/s
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             281.3 m/s at 1.75 s
top Mach number       0.830
landing               0.7 m from the pad at 247.48 s, at 4.5 m/s
```

<!-- cli: end -->

## Put the choice in your design

**hpr doesn't change your `.ork` file: set the motor and its delay in OpenRocket, save, and fly
the file again.** Without `--motor`, `hpr sim` flies the motors and delays the file holds, as
[Fly your .ork](fly-your-ork.md) shows, and its `delay` line checks the delay you set.

## What it leaves out

- **Motor-to-motor differences.** Each motor flies one thrust curve, ThrustCurve.org's, and its
  stated delay exactly. Real motors differ a little from their published curves.
- **Fit.** hpr's design checks flag a motor wider than its mount, and warn on one longer than its
  tube; they don't know your motor retainer, thrust ring or closure.
- **A verdict.** hpr ranks nothing as safe or unsafe. The motor's printed data, the safety codes
  and your range safety officer decide.
