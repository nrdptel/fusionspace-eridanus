# Check stability for a certification flight

**This guide reads hpr's stability numbers for a rocket you plan to fly for a certification, checks
them with every motor you might use, and says how far to trust them.** It is for a flier preparing
a certification flight; the rules it quotes are Level 1's. hpr gives no verdict: the safety codes
ask *you* to check stability, and the range safety officer (RSO) and your certifying member
decide. hpr's margin at rod clearance agrees with OpenRocket 24.12's to within 0.016
[calibres](glossary.md#calibre-caliber) on 41 of OpenRocket's 53 example flights, but no margin
of hpr's has been checked against a measured rocket. On OpenRocket's two pod examples it reads
0.07 calibres above OpenRocket's, and near the speed of sound its flight margin reads high
([how far to trust it](#how-far-to-trust-the-margin)). The outputs on this page
are made by running each command, and CI checks that they still match what `hpr` prints.

The steps:

1. [Know what the rules ask](#what-the-rules-ask).
2. [Read the margin](#read-the-margin).
3. [Check every motor you might fly](#check-every-motor-you-might-fly).
4. [Compare with published guidance](#compare-with-published-guidance).
5. [Check the rail exit speed](#rail-exit-speed).
6. [Fly the rocket you built](#fly-the-rocket-you-built).

## What the rules ask

**The safety codes require a stability check, but name no margin.** The words below are the
organisations' own, read on 2026-10-04; check their current pages before your flight. This is not
legal or regulatory advice.

- The NAR's [High Power Rocket Safety Code](https://nar.org/content.aspx?page_id=22&club_id=114127&module_id=669238)
  (revision of August 2012): "I will check the stability of my rocket before flight and will not
  fly it if it cannot be determined to be stable."
- Tripoli's [Unified Safety Code](https://www.tripoli.org/safetycode) (version 2, effective January
  2026), 7-1.2: "Stability; the flier shall document the location of the center of pressure and be
  able to demonstrate the center of gravity."
- A Level 1 flight uses an H or I motor. The NAR's
  [Level 1 procedures](https://www.nar.org/Level1HPRCertificationProcedures) (revision of January
  1, 2022): "at least one H or I impulse class motor"; [Tripoli's](https://www.tripoli.org/Level1):
  "a single certified H or I motor". Neither page states a margin.

The NAR's procedures add that the candidate may be asked about "the model's center of gravity and
center of pressure, methods used to determine model stability". hpr's output gives both points'
separation; `--json` and `--export` give more.

## Read the margin

**The `static margin` line is the distance from the center of gravity back to the center of
pressure, in body diameters (calibres), as the rocket leaves the rail; then its least value up to
apogee.** A positive margin turns the nose back into the oncoming air when the rocket is disturbed
([stability margin](glossary.md#stability-margin)). This flies the guides' own rocket,
[`level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork),
on its default motor, an AeroTech H170M:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]

static margin         1.70 calibres off the rail; least 1.70 calibres, at 0.12 s, before apogee
apogee                1065.1 m above the site at 11.62 s
rail exit speed       26.3 m/s
delay                 apogee 9.48 s after burnout; the motor's set delay: 10 s; its charge fires 0.52 s after apogee
descent               4.4 m/s at landing under `Parachute`

motor: 1 × H170M (the design's) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.14 s
launched at 0° N, 0° E, 0 m above sea level, from a 1.5 m vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time     height       speed
liftoff               0.00 s      0.4 m     0.0 m/s
rail exit             0.12 s      1.9 m    26.3 m/s
burnout               2.14 s    406.4 m   240.4 m/s
apogee               11.62 s   1065.1 m     0.1 m/s
charge               12.14 s   1063.7 m     5.1 m/s
deployment           12.14 s   1063.7 m     5.1 m/s
ground hit          245.78 s      0.0 m     4.4 m/s
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             285.6 m/s at 1.73 s
top Mach number       0.842
landing               0.7 m from the pad at 245.78 s, at 4.4 m/s
```

<!-- cli: end -->

A calibre here is the rocket's widest body diameter, 66 mm, so a margin of 1.70 calibres puts the
center of pressure 1.70 × 66 mm ≈ 112 mm behind the center of gravity. hpr computes the static
margin at Mach 0, with the air along the rocket's axis. The margin grows
as propellant burns and the center of gravity moves forward, so its least value is usually the
one at the rail. `--json` also gives the flight margin, at the flight's own Mach number
(`min_flight_margin_cal`), and where each least value falls
([Flight metrics](physics/metrics.md#stability-margins)).

## Check every motor you might fly

**The margin depends on the motor, so check each one you might fly, with its delay.** A motor's
mass sits behind the center of gravity, so a heavier motor moves the center of gravity aft and
shrinks the margin. The guide rocket's second
configuration flies an I175WS:

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

`--motor` flies any other motor in the mount, and `--delay` sets its ejection delay
([Pick a motor](pick-a-motor.md#fly-the-rocket-on-each)). There, the heaviest motor tried, a Loki
I377, leaves the guide rocket the smallest margin of the motors flown, and takes it past Mach 1,
where the guidance below asks for more.

## Compare with published guidance

**Published rules of thumb set a lower bound of one calibre, and more for fast flights; these are
the sources' guidance, not hpr's judgement.**

- James Barrowman, who devised the standard way to find the center of pressure, wrote: "A good
  rule of thumb is to have the static margin equal to the largest diameter of the rocket", and
  "Remember that one maximum body diameter is the smallest safe static margin" (*Stability of a
  Model Rocket in Flight*, Centuri Technical Information Report TIR-30, 1970, as reprinted in
  [this scanned compilation](https://www.nakka-rocketry.net/articles/Barrowman.NARAM-8.pdf),
  PDF pages 73 and 76).
- OpenRocket's user guide: "The recommended stability margin for subsonic flights is not less
  than 1.0 caliber, and not less than 2.0 calibers for transonic and supersonic flights"
  ([Overrides and surface finish](https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html)).
- NASA's [Student Launch handbook](https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf)
  for 2027 asks for "a minimum static stability margin of 2.0 while sitting on the pad".
- The Spaceport America Cup's [design guide](https://www.esrarocket.org/s/2026-IREC-DTEG-V11.pdf)
  (2026, version 1.1) states its limits as a share of the rocket's length instead: at least 7.5%
  through a subsonic flight, at most 18% at launch.

**A large margin isn't free.** A very stable rocket turns into a crosswind as it leaves the rail
([weathercocking](glossary.md#weathercocking)), and so flies off the vertical.

## Rail exit speed

**The rocket must leave the rail fast enough to fly straight, and the codes name no figure.** The
NAR's code asks for "rigid guidance until the rocket has attained a speed that ensures a stable
flight"; Tripoli's, 7-3, for guidance "until it has reached the velocity necessary for stable
flight". Competitions set numbers: the Spaceport America Cup's guide asks for at least 25 m/s
(5.3.1), and NASA's Student Launch handbook for 52 ft/s (15.8 m/s).

The reason is the wind. Just off the rail, a crosswind meets the rocket at an angle whose tangent
is the wind speed over the rocket's speed. At the guide rocket's 26.3 m/s in a 4 m/s crosswind,
that is about 8.6°, since 4 / 26.3 = 0.152 and the angle whose tangent is 0.152 is 8.6°. The
larger that [angle of attack](glossary.md#angle-of-attack), the further the rocket turns into the
wind, and hpr's aerodynamics hold only at small angles. Both of hpr's margins are taken with the air
along the axis, at no angle at all. A longer rail raises the exit speed: `--rail-length` sets it,
and `--wind` the wind ([Fly your .ork](fly-your-ork.md#choose-the-configuration-and-the-launch)).

## Fly the rocket you built

**A design's center of gravity is a calculation; your built rocket's is a measurement, and the
codes ask you to demonstrate it.** Weigh the finished rocket, ready to fly but without its motor,
and balance it to find its center of gravity. In OpenRocket, set that mass and center of gravity
as [overrides](glossary.md#override) on the stage, covering its parts, save, and fly the file
again in hpr, which reads those overrides ([the `.ork` page](format/ork.md)). Leave the motor out
of the measurement: an override never covers a motor, in hpr or OpenRocket, and both add the
motor's own mass, so a rocket weighed with its motor in would carry it twice. The margin then
rests on your rocket's measured mass, not the drawing's.

## How far to trust the margin

**hpr's margin has been compared with OpenRocket's, not with a measured rocket.** Both programs
start from Barrowman's method, so agreement says hpr computes it as OpenRocket does, not that
either is right ([Accuracy](accuracy.md#the-census)).

- **OpenRocket's examples:** within 0.016 calibres on 41 of 53 flights, with hpr's margin taken
  with the air along the rocket's axis, as OpenRocket's is. The margin hpr prints is the weakest
  plane's, which on a rocket with a fin set of one or two fins can be lower.
- **The three-stage example:** on its three flights hpr's margin is 0.039 to 0.058 calibres below
  OpenRocket's. hpr's center of mass at rod clearance sits 0.043 to 0.062 calibres further aft
  than OpenRocket's there, where OpenRocket's recorded mass reads light
  ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)); that cause is not yet sized.
- **The tube-fin example:** a rocket with tube fins is 1.08 calibres below OpenRocket's: hpr
  gives 0.79 calibres to OpenRocket's 1.87.
  Neither program has been checked against a measured tube-fin rocket; check such a design in both
  and treat the smaller margin as the more cautious figure, not a bound.
- **The pods-and-winglets example:** on the five flights of *Pods--airframes and winglets*, hpr's
  margin is 0.071 to 0.076 calibres *above* OpenRocket's, the flattering side: hpr calls the
  rocket more stable than OpenRocket does. Two causes are open: how the two programs count fins
  that interfere across sets ([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325)), and the
  center of pressure of its kinked freeform wings
  ([#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)). A flight whose fin sets share a
  station, more than four fins between them, warns of #325, and a flight with a freeform fin
  set warns of #326. On a design with pods or freeform
  fins, check the margin in both and treat the smaller as the more cautious figure, not a bound.
  The three flights of *Pods--powered with recovery deployment* read 0.070 calibres above
  OpenRocket's too, cause not yet traced.
- **Private designs:** on 35 flights of 11 designs, hpr's margin runs from 0.0166 calibres below
  OpenRocket's to 0.1108 above. Four designs read 0.0350 to 0.1108 calibres more stable in hpr
  than in OpenRocket, none with a measured cause yet
  ([#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172) and
  [#186](https://github.com/nrdptel/fusionspace-eridanus/issues/186), two of the open causes;
  [the private designs](format/ork.md#hprs-flights-of-the-private-designs)). A fifth, launched
  from a tilted rod, reads 0.0125 to 0.0366 above, most of it traced to the tilt. Reading more
  stable is the flattering side, so leave room for it near any limit.
- **Near the speed of sound:** between Mach 0.8 and 1.2, hpr's center of pressure is up to 2.36
  calibres behind NASA's wind-tunnel data, and wherever it misses by more than half a calibre it
  sits behind, so the flight margin there reads high: the flattering side
  ([Known gaps](accuracy.md#known-gaps)). The static margin is computed at Mach 0, so it says
  nothing about the rocket's stability in that range; use the transonic guidance
  [above](#compare-with-published-guidance).
