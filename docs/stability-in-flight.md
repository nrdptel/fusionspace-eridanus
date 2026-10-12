# Stability in flight

A rocket's center of pressure is not fixed: it moves as the rocket speeds up and as it meets the
air at an angle. This guide shows how far it moves, and which way HPR Sim's can be wrong.

**This guide is for** a flier who reads a stability margin, from HPR Sim, another simulator or a
hand calculation, and wants to know what it means once the rocket is flying. It explains where the
center of pressure comes from, why speed and the angle of attack move it, how well HPR Sim's
follows a wind tunnel's, and which error flatters a margin. It is one of the guides in *Rocketry
explained*, which explain the physics behind the numbers HPR Sim prints.

> **How far to trust it.** Every number in this guide's text is read by a test from a committed
> file: HPR Sim's center of pressure for the guides' rocket ([report][rocket-report]), and HPR
> Sim's against NASA's wind-tunnel tests of the Arcas Robin sounding rocket
> ([comparison][tunnel]). That is one rocket's shape in one tunnel, at small angles: no hobby
> rocket's center of pressure has been measured for these comparisons, and HPR Sim's whole rocket
> at a larger angle has not been checked against a measurement. Near the speed of sound HPR Sim's
> center of pressure read aft of the tunnel's by up to 2.36 calibers, the side that flatters a
> margin. The guide gives ranges, not verdicts; the safety code and the range safety officer (the
> RSO) decide whether a rocket flies.

## Where the air pushes

A rocket's weight acts at its [center of gravity](glossary.md#center-of-gravity-cg) (CG), the
point it balances on. When a gust or a tilted rail turns it so that the air meets it at an angle,
the [angle of attack](glossary.md#angle-of-attack), the air pushes on its side. That push, the
[normal force](glossary.md#normal-force), acts at the
[center of pressure](glossary.md#center-of-pressure-cp) (CP). If the CP is behind the CG, the push
turns the nose back into the oncoming air, as a weathervane turns; if it is ahead, the push turns
the rocket further off, and it tumbles.

The [stability margin](glossary.md#stability-margin) is the distance from the CG back to the CP,
in body diameters, called [calibers](glossary.md#caliber). Each part of the rocket pushes on its
own: the nose near the front, the fins at the back. The CP is the average of the parts' stations,
each weighted by how hard that part pushes per unit of angle, its normal-force slope `C_Nα`:

```text
x_CP   = Σ (C_Nα,i · x_i) / Σ C_Nα,i
margin = (x_CP − x_CG) / d
```

with `x_i` each part's own center of pressure, measured back from the nose tip, and `d` the
body's diameter. So anything that makes one part push harder than the others moves the CP toward
that part. The margin HPR Sim prints as `static margin` is the one this gives at low speed, with
the air straight along the rocket: the number [Barrowman's method](glossary.md#barrowmans-method)
gives by hand. [Check stability for a certification flight](stability-for-certification.md)
reads it.

## Speed moves it

Below the speed of sound, air squeezes a little as it flows past. The faster the rocket, the more
it squeezes, and the harder each fin pushes for the same angle, by the
[Prandtl–Glauert](physics/aero.md#fins) factor. The nose's push, in the
[slender-body theory](glossary.md#slender-body-theory) that sets it, doesn't change with speed.
So as a rocket with fins at its tail speeds up, the fins gain on the nose, and the CP moves aft:
the margin grows.

Faster than sound, the air can't move aside ahead of a fin, and the fin's push falls as the speed
climbs, as [supersonic linear theory](glossary.md#supersonic-linear-theory) gives it. The body's
push grows. So past the speed of sound the CP turns and moves forward again: the margin shrinks.
No closed-form method covers the speeds between, so HPR Sim joins its subsonic and supersonic fin
methods with a straight line, the join ([Fins through the speed of sound][fins-through]).

The guides' rocket, a small rocket for a first certification flight that the repository keeps for
its how-to guides ([its design file][level-1]), shows it. Straight into the wind, its center of pressure sits 13.99
calibers behind its nose tip at low speed. By Mach 0.8 it has moved 0.19 calibers aft, by Mach
1.2, 1.04 calibers aft, and by Mach 3 it sits 1.88 calibers forward of where it started
([report][rocket-report]). Supersonic linear theory starts for this rocket's fins at Mach 1.37.
Every caliber the CP moves is a caliber of margin, since the CG doesn't move with speed.

So the margin printed at low speed is not the margin the rocket has at its top speed. HPR Sim's
`--json` gives the flight margin too, at the flight's own Mach number, and its least value from
the rail to apogee (`min_flight_margin_cal`, [Flight metrics](physics/metrics.md#stability-margins)).

## Against a wind tunnel

NASA tested half-scale models of the Arcas Robin, a sounding rocket with a long nose and swept
fins, in wind tunnels from Mach 0.6 to 4.63 ([comparison][tunnel]). From Mach 0.8 to 1.2 the
tunnel's center of pressure moved aft by 2.60 calibers, then forward by 5.23 calibers by Mach
4.63: the shape the section above describes, larger. The figure sets HPR Sim's against it.

![A chart of the Arcas Robin's center of pressure against Mach number: the wind tunnel's readings and HPR Sim's line both rise to a peak just past the speed of sound and fall after it; in the shaded band near the speed of sound HPR Sim's sits well aft of the tunnel's, marked as the side that flatters a margin.](images/stability-cp-against-mach.svg)

Each reading in the tunnel is good to between 0.05 and 0.11 calibers either way, the band drawn
around its dot. At Mach 0.6, HPR Sim's sat 0.12 calibers aft of the tunnel's, and from Mach 1.2
up, it stayed within 0.34 calibers of it. In the join it didn't: at Mach 0.9 it sat 1.34 calibers
aft. The tunnel's CP moved aft later and more steeply than the straight line between Mach 0.8 and
1.2 puts it. On the longer model the tunnel also tested, it sat up to 2.36 calibers aft, at
Mach 1. On the model drawn, 9 of the 12 speeds are within 0.50 calibers, the target the comparison
set before measuring ([Aerodynamics](physics/aero.md#verification) has the table, the slopes and
the longer model).

## Which error flatters

A CP that reads aft of the true one makes the margin read high: the rocket is less stable than the
number says. That is the flattering side for stability, and the side to leave room for near any
limit. A CP that reads forward costs margin: the rocket is more stable than the number says.

The figure marks the flattering side. A test ties the mark to the sign: the figure's code marks
each speed whose error in the comparison is positive, and
[`a_cp_aft_of_the_reference_reads_a_higher_margin`][direction-test] holds that a CP aft of the
reference's makes a positive error, and a margin higher by that much at any CG. At 7 of the 12
speeds HPR Sim's center of pressure sat aft of the tunnel's, and the largest of those errors are
in the join. So for a rocket that flies near the speed of sound, read HPR Sim's flight margin there as
possibly high; the printed static margin, taken at low speed, says nothing about those speeds. A
sample is not a bound: a rocket
unlike the Arcas Robin may err by more.

## Angle moves it

At an angle of attack the body itself pushes sideways too, as a cylinder does in a side wind.
That push is the [body lift](glossary.md#body-lift). It grows with the square of the angle's sine,
so it is small at small angles and large at large ones, and it acts near the middle of the body's
outline seen from the side. On most rockets that point is ahead of the fins, so as the angle grows
the CP moves forward, and the margin shrinks.

At Mach 0.3, the guides' rocket's center of pressure moves forward by 0.25 calibers at 2°, 0.57
calibers at 5°, 1.01 calibers at 10°, and 1.34 calibers at 15° ([report][rocket-report]). The
printed margins, static and flight, are the small-angle ones, so they don't show this. The
simulated flight does: HPR Sim flies the body lift at each moment's angle, so a rocket leaving the
rail in a crosswind, or knocked by a gust, turns as this smaller margin turns it.

HPR Sim's body lift has been checked against wind-tunnel tests of bodies
without fins ([Body lift](physics/aero.md#body-lift)), but no measurement here checks a whole
rocket's center of pressure at a large angle, and no test pins which way it errs there. So this
section marks no side.

## What this guide leaves out

- **The center of gravity's own error.** The margin is only as good as the CG. HPR Sim takes the
  masses from the design file; weigh and balance the built rocket, motor in
  ([Check stability for a certification flight](stability-for-certification.md#fly-the-rocket-you-built)).
- **Dynamic stability:** how quickly a disturbed rocket settles, which also depends on its mass,
  its length and the air's damping, and coning or roll. HPR Sim flies these, but this guide covers
  only where the air's push acts.
- **Shapes the comparisons don't cover.** Tube fins, pods and freeform fins are checked only
  against OpenRocket; compare the margin in both programs
  ([Check stability for a certification flight](stability-for-certification.md#accuracy-of-the-margin)).
- **A rule for how much margin is enough.** That is the safety code's, the RSO's and the published
  guidance's ([Compare with published guidance](stability-for-certification.md#compare-with-published-guidance)),
  not this guide's.

[rocket-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/guide-rocket-cp.json
[tunnel]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/normal-force-vs-mach.json
[level-1]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork
[fins-through]: physics/aero.md#fins-through-mach-1
[direction-test]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/xtask/src/aero_mach.rs
