# Drag through the speed of sound

What a rocket's drag is made of, how it climbs near the speed of sound, and which way HPR Sim's
drag can be wrong.

This guide is for a flier who reads a drag coefficient, from HPR Sim, another simulator or a
hand calculation. They want to know why it changes with speed and how far to trust the drag a
simulation flies. They also want to know why a faster motor doesn't fly a rocket as much higher as
its extra impulse suggests.

The guide explains the drag's three parts, the rise near the speed of
sound and what the finish changes. Then it shows how well HPR Sim's drag follows a wind tunnel's,
and which error flatters. It's one of the guides in *Rocketry explained*, which explain the
physics behind the numbers HPR Sim prints.

> **How far to trust it.** Every number in this guide's text is read by a test from three committed
> files. One holds HPR Sim's drag for the guides' rocket ([report][rocket-report]). Another sets
> HPR Sim's against NASA's wind-tunnel tests of the Arcas Robin sounding rocket
> ([comparison][tunnel]). The third holds the logged flights of
> [Trusting a simulation](trusting-a-simulation.md).
>
> The tunnel comparison is one rocket's shape, two half-scale models of it in two of NASA's tunnels,
> their bases left out. No hobby rocket's drag has been measured for these comparisons. On the model
> the figure draws, HPR Sim's forebody drag read high at every speed, by +7.0% to +154.1%, rounded
> outward. That's the side that flatters an apogee ceiling.
>
> On hobby rockets the evidence leans the other way. HPR Sim's apogee
> averaged +8.73% above the logged one on the 53 flights, and drag is one of the inputs that could
> explain it. The guide gives ranges, not verdicts. The safety code and the range safety officer
> (the RSO) decide whether a rocket flies.

## What drag is made of

Drag is the air holding a rocket back along its path. It grows with the
[dynamic pressure](glossary.md#dynamic-pressure) $q$. At low speed that's the pressure the oncoming
air would exert if it were brought to rest. The dynamic pressure grows with the air's density and
the square of the speed. A simulator turns it into the drag $D$, a force, with the rocket's
[drag coefficient](glossary.md#drag-coefficient) $C_D$ and its
[reference area](glossary.md#reference-area) $A$, the body's cross-section:

$$
\begin{aligned}
q &= \tfrac12 \rho v^2, \\
D &= q\, C_D A
\end{aligned}
$$

Here $\rho$ is the air's density and $v$ the airspeed. The drag coefficient is how draggy the
shape is, whatever its size. It changes with speed, given here as a
[Mach number](glossary.md#mach-number), the airspeed over the speed of sound. It's the sum of
three parts:

- **Skin friction:** the air rubbing along the surface, in the thin
  [boundary layer](glossary.md#boundary-layer) it drags along. It acts over the whole outside of
  the rocket, so a longer rocket, or one with bigger fins, has more of it.
- **Pressure drag:** the air pushing back on every face that meets it: the nose, a shoulder, the
  fins' front edges. A smooth, pointed nose has almost none below the speed of sound. A flat fin
  edge has some at any speed.
- **[Base drag](glossary.md#base-drag):** the low pressure behind the flat aft end, where the
  air can't follow the body round its corner. It pulls back on the base like a suction cup.

HPR Sim builds the coefficient up part by part, following Niskanen's thesis, which OpenRocket
also follows ([Aerodynamics](physics/aero.md#drag)).

The guides' rocket shows the parts, as HPR Sim computes them. It's a small rocket for a first
certification flight that the repository keeps for its how-to guides ([its design file][level-1]).
No measurement of this rocket's drag exists. At Mach 0.3 in sea-level air its drag coefficient is
0.60: skin friction makes 59% of it, the base 22% and the fins' square edges 19%. Its ogive nose,
smooth into the body, adds next to nothing at that speed ([report][rocket-report]).

A worked example. At Mach 0.5 in sea-level air the guides' rocket flies at 170 m/s (558 ft/s), so
the dynamic pressure is 17.7 kPa. Its reference area is the body's cross-section, 66 mm (2.60 in)
across. Its drag coefficient there is 0.63, so the air holds it back with 38 N, the weight of
3.9 kg.

## Through the speed of sound

Air flowing round a nose speeds up, so as a rocket nears the speed of sound the air beside its
nose passes it first. Shocks form there, and the pressure pushing back on the nose climbs: the
transonic drag rise. Past the speed of sound the nose pushes a shock wave ahead of it. Its
pressure drag, now called [wave drag](glossary.md#wave-drag), settles to a value set by the nose's
shape and how slender it is.

The base's share of the coefficient grows with speed too, until the
speed of sound, and then falls. The suction itself keeps growing, though more slowly. Skin
friction's falls with speed, by a step at the speed of sound in HPR Sim's rule and faster past it
([Aerodynamics](physics/aero.md#drag-through-mach-1) has the formulas).

The guides' rocket's drag coefficient climbs to 0.69 by Mach 0.8 and 0.84 at Mach 1. It peaks at
0.88 at Mach 1.07, 45% above its value at Mach 0.3, and by Mach 2 it's back down to 0.57
([report][rocket-report]). The base's part of the drag coefficient grows from 0.15 at Mach 0.5 to
0.25 at Mach 1, then falls ([report][rocket-report]). The nose's pressure drag, next to nothing at
Mach 0.5, reaches 0.19 at Mach 1.1 ([report][rocket-report]).

Drag grows with the square of the speed as well, so the two compound. Doubling the speed makes
the dynamic pressure four times larger, and the coefficient is higher besides. So in the same air,
at Mach 1 it meets 5.3 times the drag it meets at Mach 0.5.

That's why a bigger motor buys less height near the speed of sound than its extra impulse
suggests. More of the extra goes into pushing air. On the faster of its two motors the guides'
rocket reaches Mach 0.84, where the rise has begun. At Mach 0.85, just past that, its drag
coefficient is 0.71, 17% above its value at Mach 0.3
([When fins flutter](when-fins-flutter.md) flies it).

## Surface finish

Skin friction depends on how rough the surface is. On a smooth surface the friction falls slowly
as the rocket gets faster or longer. On a rough one, above a speed that depends on how rough it
is, the roughness alone sets it. HPR Sim takes the roughness from each part's finish, a height in
micrometers from Barrowman's table of surfaces ([Aerodynamics](physics/aero.md#drag)).

With every outside part given the same finish, the guides' rocket's drag coefficient at Mach 0.3
is: polished, 0.5 µm, 0.49; aircraft paint, 20 µm, HPR Sim's default, 0.53; the design's own
60 µm, 0.60; and badly sprayed paint, 200 µm, 0.70 ([report][rocket-report]). So at Mach 0.3
the drag coefficient is 43% higher with the badly sprayed paint than with the polished finish. The
design's finish is what OpenRocket calls regular paint. Which finish a built rocket has is a
judgment: the table describes surfaces, not a hobby rocket's paint, seams, fillets and decals.

## Against a wind tunnel

NASA measured the drag of two half-scale models of the Arcas Robin. It's a sounding rocket with a
long nose, thin, sharp swept fins and a short [boattail](glossary.md#boattail). The short model is
the Arcas Robin itself, and the long one has a stretched body. They flew in two wind tunnels from
Mach 0.6 to 4.63 ([comparison][tunnel]).

The models sat on a [sting](glossary.md#sting), which disturbs the air behind the base. So the
comparison is of the [forebody](glossary.md#forebody): the drag less the base's. With its fins on,
the model's measured forebody drag rose from 0.30 at Mach 0.6 to 0.69 at Mach 1. Then it fell to
0.18 by Mach 4.63: the same rise and fall, on another rocket. The figure sets HPR Sim's forebody
drag against the tunnel's, on the short model.

![A chart of the Arcas Robin's forebody drag against Mach number: the wind tunnel's readings rise to a peak at the speed of sound and fall after it; HPR Sim's line runs above them at every speed, closest, as a share of the reading, at the speed of sound and furthest at the fastest speeds, each gap marked as the side that flatters an apogee ceiling.](images/drag-against-mach.svg)

Each of the tunnel's points was read off the reports' plots to within 0.002 to 0.008 in drag
coefficient either way. That's the band drawn round its dot, and most bands are smaller than the
dot.

HPR Sim's read +12.9% at Mach 0.6. It read +51.0% at Mach 0.9, the most of the readings
below the speed of sound. It read +7.1% at Mach 1, the least. Then it read more at every faster
speed, to +154.0% at Mach 4.63, and at the faster speeds most of that is the fins. HPR Sim gives a
fin's front edge the drag of a rounded edge, which grows with speed, where a thin, sharp fin's
falls.

With the fins off, HPR Sim's read +12.0% to +44.6% high on the short model, and up to +54.2% on
the long one. Most of that comes from the boattail, which HPR Sim's rule over-predicts at these
speeds.

The models' finish wasn't reported. The comparison takes it as polished, a guess, and a
rougher one would read higher still. Only 2 of the 44 readings, both models' with their fins on
and off, are within 10%, the target the comparison set before measuring.
[Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel) has the tables and the
causes.

## Which error flatters

A drag that reads high makes the simulated rocket fly lower and slower than the real one. The
simulated apogee, top speed and max q, the peak dynamic pressure, all read low. For a flight near a
waiver's ceiling, the height a launch's airspace permit allows, that's the flattering side. The
real rocket may climb higher than the number says. A max q that reads low flatters the fins and
the airframe too ([When fins flutter](when-fins-flutter.md)).

A drag that reads low errs the other way: the rocket flies lower than the number says. That's the
optimistic side for a flier aiming at a target altitude.

The figure marks the flattering side: its thick bars mark each speed where HPR Sim's drag reads
above the tunnel's. The test [`a_higher_drag_reads_a_lower_apogee`][direction-test] pins what that
sign means, on a rocket that passes the speed of sound and on one that doesn't. On both, the same
flight with its drag scaled up reaches a lower apogee, a lower top speed and a lower max q.
Scaled down, it reaches higher ones.

It pins the sign, not how often HPR Sim errs high. The marks
show that. On the Arcas Robin, HPR Sim's forebody drag read high at every speed. So for a rocket
of that shape, with thin, sharp fins and a boattail, read its apogee as possibly low.

The tunnel's rocket isn't a hobby rocket, though. On the logged hobby flights of
[Trusting a simulation](trusting-a-simulation.md), HPR Sim's apogee averaged +8.73% above the
logged one on the 53 flights.

On those rockets something leans the other way. Drag that reads low
is one candidate. The motor's thrust and the rocket's mass are others. No comparison yet sizes HPR
Sim's drag alone on a hobby rocket, so neither direction is a rule for yours.

## What this guide leaves out

- **The base while the motor burns.** HPR Sim takes the burning motor's cross-section off the
  base's area, as Niskanen does. OpenRocket keeps the whole base. No measurement here checks
  either ([power-on drag](glossary.md#power-on-and-power-off-drag)).
- **Drag at an angle of attack.** A rocket flying at an angle meets more drag. HPR Sim scales its
  drag coefficient by a factor of the angle, which no comparison here checks.
- **Launch lugs, rail buttons and fillets.** The guides' rocket has none modeled. Each adds drag,
  and HPR Sim models lugs and buttons but not fillets
  ([Aerodynamics](physics/aero.md#validity-and-open-questions)).
- **The other comparisons.** HPR Sim's drag is also compared with RASAero II's curves and with a
  handbook's worked calculation, neither a measurement
  ([Aerodynamics](physics/aero.md#drag-verification)).

**Sources.** The tunnel readings are NASA's, works of the US Government. The logged flights'
weather is ERA5, generated using Copernicus Climate Change Service information 2026, and contains
modified Copernicus Climate Change Service information 2026. Neither the European Commission nor
ECMWF is responsible for any use made of it. The [report][fixture-report] gives the full
attribution and the ERA5 citations.

[rocket-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/guide-rocket-drag.json
[tunnel]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/drag-vs-mach.json
[level-1]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork
[direction-test]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/tests.rs
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
