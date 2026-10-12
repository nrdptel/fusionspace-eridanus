# When fins flutter

What makes a fin flutter, how the criterion HPR Sim uses puts a speed on it, and which way that
speed can be wrong.

**This guide is for** a flier choosing a fin's material and thickness, or reading a flutter speed
from HPR Sim, a calculator or a spreadsheet, who wants to know what the number means. It explains
what flutter is, what the criterion's speed depends on, what its two readings say about the
guides' rocket, and which errors flatter it. It is one of the guides in *Rocketry explained*,
which explain the physics behind the numbers HPR Sim prints.

> **How far to trust it.** Every number in this guide's text is read by a test from a committed
> file: HPR Sim's flutter numbers for the guides' rocket ([report][rocket-report]). They come from
> D. J. Martin's criterion, written at NACA, NASA's forerunner, which HPR Sim reproduces along
> with Martin's own worked examples ([Fin flutter][model]). Martin drew his line between wings
> that fluttered and wings that didn't from missiles and wind-tunnel models, the unfailed ones
> wings that flew to at least Mach 1.3. Nothing here checks the criterion against a hobby
> rocket's fins, and it leaves out the fins' sweep and how they are attached. So its speed is a
> screening number with an error no one has measured, not a speed at which a fin is known to fail.
> The guide gives ranges, not verdicts; the safety code and the range safety officer (the RSO)
> decide whether a rocket flies.

## What flutter is

Air pushing on a fin twists it a little. The twist changes the angle at which the air meets the
fin, so the push changes, and the fin bends. Below some speed the fin's stiffness damps each
wobble out. Above it, each cycle feeds the next, and the wobble grows until the fin breaks,
sometimes within a second. That speed is the [flutter](glossary.md#flutter) speed.

What resists it is the fin's stiffness against twisting, which comes from its material's
[shear modulus](glossary.md#shear-modulus) and, far more, from its thickness. A fin that feels
stiff when you bend it by hand can still be soft in twist.

## Martin's criterion

Martin reduced the theory of a wing that bends and twists to a few numbers from the fin's
outline, and checked it against the flutter record of missiles and wind-tunnel models
([The criterion][criterion]). His flutter speed is

```text
V_f = a · √(G_E / D)
```

with `a` the speed of sound, `G_E` the fin's shear modulus, and `D` a denominator that holds the
rest: the air's pressure and the fin's shape. Read in words:

- **Thickness counts most.** The flutter speed grows as the thickness ratio (thickness over root
  chord) to the power one and a half: a fin twice as thick flutters at nearly three times the
  speed.
- **Stiffness counts less.** It grows as the square root of the shear modulus: a material four
  times as stiff in shear doubles it.
- **Shape.** A fin with a longer span for its chord (a higher aspect ratio) flutters sooner, and
  so does one whose tip is nearly as wide as its root (a taper ratio near one).
- **Air.** Thinner air raises the flutter speed, so the same fin can fly faster higher up. But the
  air's push at that speed, the [dynamic pressure](glossary.md#dynamic-pressure), is the same at
  every height, so a flight comes closest to its fins' flutter speed where its dynamic pressure
  peaks, at *max q*.

Martin's constant comes from the theory, not a fit to data: it assumes the fin's mass sits at
mid-chord. What his data set is where the line falls, below.

## The guides' rocket

The guides' rocket, a small rocket for a first certification flight that the repository keeps for
its how-to guides ([its design file][level-1]), has 3 fins of birch plywood, 3 mm (0.12 in) thick,
with a 130 mm (5.12 in) root chord, a 60 mm (2.36 in) tip chord and a 75 mm (2.95 in) span. The
design file's plywood is OpenRocket's, which HPR Sim doesn't match to one of its materials, so
this guide takes HPR Sim's built-in birch plywood, which has a cited shear modulus, 0.75 GPa, from
a plywood maker's handbook. In Martin's terms the fins have an aspect ratio of 0.79, a taper ratio
of 0.46 and a thickness ratio of 2.3%. The criterion leaves out one more thing about them:
their leading edges are swept back by 70 mm (2.76 in) over the span.

In the launch site's air, standard air at sea level, the criterion puts their flutter speed at
175 m/s (574 ft/s), Mach 0.51. On the faster of its motors, the rocket reaches 286 m/s (938 ft/s),
Mach 0.84. At its max q, 48.5 kPa, the flutter speed is 0.62 times the airspeed there: by this
criterion, the fins fly faster than their flutter speed. For the flutter speed to reach that
airspeed, the plywood would have to be 4.2 mm (0.17 in) thick.

The `hpr` command doesn't print these yet; in code, HPR Sim's
[`FlutterPanel`][flutter-panel] gives the flutter speed and the ratio `V_f / V` at max q
([A rocket's fins][rocket-fins] has a worked example that CI runs).

## Two readings

The ratio `V_f / V` is the first reading. The second is Martin's own check: where the fin plots on
his chart against the line his data draw. He plotted `D` against `G_E` for missiles and
wind-tunnel models. Wings that fluttered or failed lie above a band, and wings that flew to at
least Mach 1.3 without known failure lie below it. The band runs at a ratio of 0.25 to 0.31, a
flutter speed 1.8 to 2.0 times the speed of sound ([The line in Martin's data][band]). So the
band isn't a speed to stay under: it is how stiff Martin's wings had to be, whatever they flew at.

The guides' rocket's fins plot at 3.79, above the band; to plot below it with the same outline,
they would have to be 7.5 mm (0.30 in) thick. That says less than it seems to: Martin's wings
below the band flew past the speed of sound, and the rocket tops out at Mach 0.84. His data show
nothing about a fin above the band on a slower rocket: not that it is safe, and not that it will
flutter.

Neither reading alone is a line to fly by. A ratio `V_f / V` above one says only that the rocket
stays below a speed Martin's data put no error on, and the chart says only where a fin sits among
missiles that flew faster than most hobby rockets.

## A flutter speed that reads high

A flutter speed that reads high flatters: it says a fin is stiffer, or the rocket further from
flutter, than it is. The figure draws the guides' rocket's flutter speed by thickness, with the
best-known such error above it.

![A chart of the guides' rocket's flutter speed against its fins' thickness: HPR Sim's line rises steeply with thickness and crosses the rocket's top speed well before it reaches Martin's band, with the guides' rocket's fins below both; the halved constant's line, marked as the side that flatters, runs above it.](images/flutter-speed-by-thickness.svg)

Apogee Components' *Peak of Flight* newsletter printed Martin's formula in issues #291 and #411
with its constant halved, so its flutter speed reads 41% high; issue #615 corrected it
([the correction][pof-615]). A calculator or a spreadsheet built from the earlier
issues gives the higher speed, and HPR Sim's predecessor made the same mistake. On the guides'
rocket, the halved constant puts the guides' rocket's flutter speed at 247 m/s (810 ft/s), and the
ratio at max q at 0.88.

The figure marks the halved constant's line in the caution color. The test
[`the_halved_constant_reads_a_higher_flutter_speed`][direction-test] pins which way that error
goes: on any fin, in any air, the halved form's flutter speed is the square root of two times HPR
Sim's, never below it, and its ratio `V_f / V` higher by the same factor. The guide's own test pins
that the marked line is that speed.

Where Martin leaves a choice, HPR Sim takes the one that gives the lower flutter speed: the
thickness ratio at the root, the smallest on a fin of one thickness; a solid fin's shear modulus
as its material's, not the flat plate's reading, which is twice as stiff; and Martin's simpler
taper factor, which lowers the speed for most tapers ([Readings where the source leaves
room][readings]). Two choices can go the other way: a fin with an airfoil section reads a few
percent high, and a booster's fins are held to the whole flight's max q, without a check of the
booster's own after it separates.

What the criterion leaves out may matter more than any of these. Martin's wings were clamped at
the root; a fin glued to a tube twists more there, so its real flutter speed can be lower, on the
flattering side. Sweep, which the guides' rocket's fins have, is left out, and no test pins which
way it moves the speed. So a flutter speed only a little above the top speed is not a margin
anyone has measured.

## What this guide leaves out

- **Fiberglass and printed fins.** G10, the commonest fiberglass fin sheet, has no cited
  shear modulus in HPR Sim, and nor do the common printing plastics; pass your own from a
  measurement or your supplier ([Shear moduli][moduli]).
- **How the fin is built and attached:** fillets, tip-to-tip layups, through-the-wall tabs, and
  the body tube's own bending.
- **Speed effects near the speed of sound**, where flutter speeds can dip, and stall flutter at large angles
  of attack.
- **A rule for how much margin is enough.** That is the safety code's, the RSO's and the fin
  maker's, not this guide's.

[rocket-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/guide-rocket-flutter.json
[level-1]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork
[model]: physics/flutter.md
[criterion]: physics/flutter.md#the-criterion
[band]: physics/flutter.md#the-line-in-martins-data
[readings]: physics/flutter.md#readings-where-the-source-leaves-room
[moduli]: physics/flutter.md#shear-moduli
[rocket-fins]: physics/flutter.md#a-rockets-fins
[flutter-panel]: api/hpr_sim/flutter/struct.FlutterPanel.html
[pof-615]: https://www.apogeerockets.com/education/downloads/Newsletter615.pdf
[direction-test]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/flutter.rs
