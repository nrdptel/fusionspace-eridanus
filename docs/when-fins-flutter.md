# When fins flutter

What makes a fin flutter, how the criterion HPR Sim uses puts a speed on it, and which way that
speed can be wrong.

This guide is for a flier choosing a fin's material and thickness, or reading a flutter speed,
who wants to know what it means. The speed can come from HPR Sim, a calculator or a spreadsheet.
The guide explains what flutter is and what the criterion's speed depends on. Then it shows what
the criterion's two readings say about the guides' rocket, and which errors flatter it. It's one
of the guides in *Rocketry explained*, which explain the physics behind the numbers HPR Sim
prints.

> **How far to trust it.** Every number in digits in this guide's text is read by a test
> from a committed file. The file holds HPR Sim's flutter numbers for the guides' rocket
> ([report][rocket-report]). They come from D. J. Martin's criterion, which HPR Sim reproduces
> along with Martin's own worked examples ([Fin flutter][model]).
>
> Martin drew his line from missiles and wind-tunnel models; the unfailed ones were wings that flew
> to at least Mach 1.3. Nothing here checks the criterion on a hobby rocket's fins. It leaves out
> their sweep and how they're attached. So its speed is a screening number, not a speed at which a
> fin is known to fail. No measurement known to this project sizes its error.

Martin wrote the criterion at NACA, NASA's forerunner. The guide gives ranges, not verdicts. The
safety code and the range safety officer (the RSO) decide whether a rocket flies.

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
outline. He checked it against the flutter record of missiles and wind-tunnel models
([The criterion][criterion]). His flutter speed is

$$
V_f = a \sqrt{\frac{G_E}{D}}
$$

Here $a$ is the speed of sound and $G_E$ the fin's shear modulus. The denominator $D$ holds the
rest: the air's pressure and the fin's shape. Read in words:

- Thickness counts most. The flutter speed grows as the thickness ratio (thickness over root
  chord) to the power one and a half. So a fin twice as thick flutters at nearly three times the
  speed.
- Stiffness counts less. The flutter speed grows as the square root of the shear modulus: a
  material four times as stiff in shear doubles it.
- A fin with a longer span for its chord (a higher aspect ratio) flutters sooner. On a fin of one
  root chord, span and thickness, a narrower tip lowers the flutter speed too. It raises the
  aspect ratio by more than the narrower tip's taper helps.
- Thinner air raises the flutter speed, so the same fin can fly faster higher up.
- The air's push at the flutter speed, the [dynamic pressure](glossary.md#dynamic-pressure)
  there, is the same at every height. So a flight comes closest to its fins' flutter speed where
  its own dynamic pressure peaks, at [max q](glossary.md#dynamic-pressure).

Martin's constant comes from the theory, not a fit to data: it assumes the fin's mass sits at
mid-chord. His data decide only where his line falls, below.

## The guides' rocket

The guides' rocket is a small rocket for a first certification flight that the repository keeps
for its how-to guides ([its design file][level-1]). It has 3 fins of birch plywood, 3 mm (0.12 in)
thick. Each fin has a 130 mm (5.12 in) root chord, a 60 mm (2.36 in) tip chord and a 75 mm
(2.95 in) span.

The design file's plywood is OpenRocket's, which HPR Sim doesn't match to one of
its materials. So in this guide I use HPR Sim's built-in birch plywood. It has a cited shear
modulus, 0.75 GPa, from a plywood maker's handbook. In Martin's terms the fins have an aspect
ratio of 0.79, a taper ratio of 0.46 and a thickness ratio of 2.3%.

The criterion leaves out one more thing about them: their leading edges are swept back by 70 mm
(2.76 in) over the span.

The criterion puts their flutter speed at 175 m/s (574 ft/s), Mach 0.51, in the launch site's air:
standard air at sea level. The rocket reaches 286 m/s (938 ft/s), Mach 0.84, on the faster of its
motors. Max q comes 298 m (978 ft) above the pad, in air close to the launch site's. At max q,
48.5 kPa, the flutter speed is 0.62 times the airspeed there: by this criterion, the fins fly
faster than their flutter speed.

No measurement known to this project sizes the criterion's error
on fins like these. So this reading marks the fins for a closer look. It doesn't say they will
fail. For the flutter speed to reach the airspeed at max q, the plywood would have to be 4.2 mm
(0.166 in) thick.

The `hpr` command doesn't print these yet. In code, HPR Sim's [`FlutterPanel`][flutter-panel]
gives the flutter speed and the ratio $V_f/V$ at max q. [A rocket's fins][rocket-fins] has a
worked example that CI runs.

## Two readings

The ratio $V_f/V$ is the first reading. The second is Martin's own check: where the fin plots on
his chart against the line his data draw.

His chart gives each wing one number, the denominator $D$ over its shear modulus $G_E$. The higher
it is, the less stiff the wing for the air it flies in. Wings that fluttered or failed lie above a
band, and wings that flew to at least Mach 1.3 without known failure lie below it. The band runs at
a ratio of 0.25 to 0.31, a flutter speed 1.8 to 2.0 times the speed of sound ([The line in Martin's
data][band]). So the band isn't a speed to stay under: it's how stiff Martin's unfailed wings were,
all of them faster than sound.

The guides' rocket's fins plot at 3.79, above the band. To plot below it with the same outline,
they would have to be 7.5 mm (0.296 in) thick.

The figure in *A flutter speed that reads high* shows the same fact the other way up: the fins'
flutter speed sits below the band. That says less than it seems to. The rocket tops out at Mach
0.84, while Martin's wings below the band flew past the speed of sound. His data show nothing about
a fin above the band on a slower rocket: not that it's safe, and not that it will flutter.

Of the two, Martin's chart is the stricter, and it's his own check. A fin below the band is among
wings that flew faster than sound unharmed. A ratio `V_f / V` above one says only that the rocket
stays below the criterion's speed. No measurement known to this project sizes that speed's error
on hobby fins. Neither is a line to fly by.

## A flutter speed that reads high

A flutter speed that reads high flatters: it says a fin is stiffer, or the rocket further from
flutter, than it is. The figure draws the guides' rocket's flutter speed by thickness, with the
best-known such error above it.

![A chart of the guides' rocket's flutter speed against its fins' thickness: HPR Sim's line rises steeply with thickness and crosses the rocket's top speed well before it reaches Martin's band; the guides' rocket's fins sit on HPR Sim's line, below the top speed and the band; the halved constant's line, marked as the side that flatters, runs above it.](images/flutter-speed-by-thickness.svg)

Apogee Components' *Peak of Flight* newsletter printed Martin's formula in issues
[#291][pof-291] and [#411][pof-411] with its constant halved. So its flutter speed reads 41% high.
Issue [#615][pof-615] corrected it.

A calculator or a spreadsheet that copied the printed form gives the higher speed. Loft, HPR Sim's
predecessor, made the same mistake ([a lesson from Loft](decisions-and-roadmap.md#l32)). The halved
constant puts the guides' rocket's flutter speed at 247 m/s (810 ft/s), and the ratio at max q at
0.88.

The figure marks the halved constant's line in the caution color. The test
[`the_halved_constant_reads_a_higher_flutter_speed`][direction-test] pins which way that error
goes, on three fins in two kinds of air. It pins the halved form's flutter speed at the square
root of two times HPR Sim's, never below it. It pins the halved form's ratio `V_f / V` higher by
the same factor.

Only the constant differs, so the factor is the same on every fin in any air. The
guide's own test pins that the marked line is that speed.

Where Martin leaves a choice, HPR Sim takes the one that gives the lower flutter speed
([Readings where the source leaves room][readings]):

- the thickness ratio at the root, the smallest on a fin of one thickness;
- a solid fin's shear modulus as its material's, not the flat plate's reading, which is twice as
  stiff;
- Martin's simpler taper factor, which lowers the speed for most tapers.

Two choices can go the other way. A fin with an airfoil section reads a few percent high. A
booster's fins are held to the whole flight's max q, without a check of the booster's own after
it separates.

And the correction in #615 reads where the fin's mass sits from the whole fin's
outline, not from Martin's section. On a swept fin like the guides' rocket's, that puts the mass
further aft and the flutter speed lower still. HPR Sim keeps Martin's reading.

What the criterion leaves out may matter more than any of these. Martin's wings were clamped at
the root. A fin glued to a tube twists more there, so its real flutter speed can be lower, on the
flattering side. Sweep, which the guides' rocket's fins have, is left out, and no test pins which
way it moves the speed. So a flutter speed only a little above the top speed isn't a margin anyone
has measured.

## What this guide leaves out

- **Fiberglass and printed fins.** G10, the commonest fiberglass fin sheet, has no cited shear
  modulus in HPR Sim, and nor do the common printing plastics. Pass your own from a measurement
  or your supplier ([Shear moduli][moduli]).
- **How the fin is built and attached:** fillets, tip-to-tip layups, through-the-wall tabs, and
  the body tube's own bending.
- **Speed effects near the speed of sound**, where flutter speeds can dip, and stall flutter at
  large angles of attack.
- **A rule for how much margin is enough.** That's the safety code's, the RSO's and the fin
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
[pof-291]: https://www.apogeerockets.com/education/downloads/Newsletter291.pdf
[pof-411]: https://www.apogeerockets.com/education/downloads/Newsletter411.pdf
[pof-615]: https://www.apogeerockets.com/education/downloads/Newsletter615.pdf
[direction-test]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/flutter.rs
