# Flight metrics

## In short

- **What it models:** the numbers a flight is judged by. These are the apogee, the top speed and
  Mach number, the peak [dynamic pressure](../glossary.md#dynamic-pressure) ("max q"), the boost's
  peak acceleration and, kept apart, the opening shock. It also gives the
  [stability margin](../glossary.md#stability-margin) from the rail exit to apogee or the first
  [deployment](../glossary.md#deployment), the largest
  [angle of attack](../glossary.md#angle-of-attack) that counts toward the envelope's
  high-angle flag ([below](#the-largest-angle-of-attack)), whether the rocket is unstable while a
  motor burns ([below](#unstable-under-power)), the
  [ejection delay](../glossary.md#ejection-delay) that would fire the charge at apogee, and each
  landing's latitude and longitude. Anything that didn't happen is `None`, never a zero.
- **Sources:** the margin is [Barrowman's](../glossary.md#barrowmans-method) center of pressure
  (to Mach 0.8; faster, as [Aerodynamics](aero.md#your-rockets-center-of-pressure) describes)
  against the [center of mass](../glossary.md#center-of-gravity-cg) (the CG), defined as [RocketPy](../glossary.md#rocketpy) defines its static
  margin and stability margin. The peak search is Kiefer's golden-section search
  ([References](#references)). Latitude and longitude come from hpr's WGS 84 conversions
  ([Geodesy](geodesy.md)).
- **How well it is validated:** each number is only as good as the flight it comes from. The
  flight is checked against RocketPy and OpenRocket ([Accuracy](../accuracy.md)), and the metrics
  add no physics of their own. Tests check each against a hand calculation or against hpr's own
  models evaluated directly ([Tests](#tests)). The margin in the weakest plane is checked against
  hpr's own scan of 3,600 planes in a test, and against one OpenRocket run that is not committed.
  None is validated against a real flight.
- **What it leaves out:**
  - Fin flutter, which has its own page: [Fin flutter](flutter.md). Its margin comes from the
    max q found here.
  - File exports: a later part of the same milestone,
    [M1.10c](../decisions-and-roadmap.md#m1-10c) (CSV, JSON, KML and GeoJSON files).
  - The descent of a separated body, such as a dropped [booster](../glossary.md#booster). It gets
    a landing, but no peaks.
  - A damping ratio: how fast a wobble dies out. Both margins here are static quantities.
  - The margin at the flight's [angle of attack](../glossary.md#angle-of-attack). Both margins
    take the air along the rocket's axis, in the weakest direction it can cross
    ([Stability margins](#stability-margins)). In hpr's model a rocket meeting the air at an angle, as
    it does leaving a rail in wind, can have a smaller or a larger margin than these, depending on
    where its body's lift acts ([Stability margins](#stability-margins) says why they leave it
    out).

## Why these rules

Loft, this project's predecessor, got four things wrong that these metrics are built to avoid:

- It read peaks off a table of recorded rows and missed them, and it counted the opening shock as
  the boost's peak acceleration ([L34](../decisions-and-roadmap.md#l34)).
- It printed zeros for things that never happened, and never said which height an apogee was
  counted from ([L35](../decisions-and-roadmap.md#l35)).
- It published margins of ±12 to 15 calibres for rockets where a margin has no meaning
  ([L33](../decisions-and-roadmap.md#l33)).
- Its optimum ejection delay depended on the delay flown ([L94](../decisions-and-roadmap.md#l94)).

Each is a numbered lesson with a test ([Tests](#tests)).

## Getting the numbers

Fly a `Simulation` with a
[`FlightMetrics`](../api/hpr_sim/metrics/struct.FlightMetrics.html) watching it. Then ask the
watcher for a [`FlightSummary`](../api/hpr_sim/metrics/struct.FlightSummary.html):

```rust,ignore
let mut metrics = FlightMetrics::new();
let flight = simulation.run(&mut metrics)?;
let summary = metrics.summary(&flight, simulation.environment())?;
let best = hpr_sim::metrics::optimum_delays(&simulation)?;
```

A watcher keeps one flight. Call `metrics.clear()` before it watches another; `summary` refuses a
flight unless the watcher saw each of its steps once.

The example program
[`flight_metrics.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/flight_metrics.rs)
does this for Valetudo, the rocket from [Getting started](../getting-started.md). It flies with a
5 m/s wind from the west and one 1.5 m parachute. The charge fires 6 s after
[burnout](../glossary.md#burnout) (the end of the thrust curve), and the canopy is open 0.5 s
later. Run it with `cargo run --example flight_metrics -p fusionspace-hpr-sim`. It prints:

```text
Valetudo on a K400C, a 1.5 m parachute fired 6 s after burnout, open 0.5 s later
Not yet validated: see the Accuracy page before trusting these numbers.

Heights are the center of mass's above the launch site; it starts 0.94 m up.
Apogee:                714.0 m above the site (713.0 m of climb) at 11.16 s
Top speed:             112.3 m/s at 3.00 s, 188 m up
Top Mach number:       0.337 at 3.02 s, 190 m up
Max q:                  6667 Pa at 2.99 s, 186 m up
Boost acceleration:     47.2 m/s² at 0.03 s, 1 m up
Opening shock:         153.4 m/s² at 9.76 s, 698 m up

Rail exit:            16.2 m/s at 0.37 s, 17.2° off the oncoming air
At rail exit (0.37 s): static margin 3.09 cal, flight margin 3.09 cal at Mach 0.051
Least static margin:    3.09 cal at 0.37 s, 4 m up
Least flight margin:    3.09 cal at 0.37 s, 4 m up

Optimum delay:        10.6 s after burnout at 3.26 s (flown: 6.0 s), for an apogee of 778.7 m at 13.83 s
Landing:              32.990000° N, 106.967084° W: 272.6 m east and 0.0 m north of the site, at 11.6 m/s, at 78.67 s
```

The 6 s delay is too short:

- The charge fires at 9.26 s, and the canopy opens at 9.76 s: about 4 s (13.83 − 9.76) before
  the apogee it would have reached.
- The rocket still rises 1.4 s more, to an apogee of 714.0 m.
- With the charge held, it would have coasted to 778.7 m at 13.83 s: the apogee of the
  [Getting started](../getting-started.md) flight, whose drogue fires at apogee.
- So the optimum delay is 10.6 s. The opening shock, 153.4 m/s², is three times the boost's
  47.2 m/s², and it is not counted as the boost's peak.
- Don't size a shock cord from the 153.4 m/s²; it is no bound either way. The canopy reaches
  full drag at line stretch, 0.5 s after the charge, with no filling time (hpr's default), so the
  rocket doesn't slow while it fills: that reads high. hpr also leaves out a canopy's drag
  overshoot near the end of filling: that reads low
  ([The opening load](recovery.md#the-opening-load)).

Top speed comes at 3.00 s, before the 3.26 s burnout, because in the thrust curve's last moments
the motor pushes less than drag and gravity pull back.

## Heights

Every height is of the center of mass: its
[ellipsoidal height](../glossary.md#ellipsoidal-height) above the launch site's, the same as
a flight's `height_above_ground_m`. The center of mass starts above the site, because the rocket
stands on the rail: 0.94 m for Valetudo. So the summary gives both:

- `apogee.height_above_ground_m`: 714.0 m, above the site.
- `apogee.gain_m`: 713.0 m, the climb from where the center of mass stood at launch. OpenRocket's
  altitude counts this way. A flight started in the air (`Simulation::run_free`) has no launch
  height, and no climb: `None`.

A flight ends when its center of mass comes back down to the site's height, so a landing is at
height 0 by this measure.

## Peaks

Each peak comes with its time and its height. The watcher looks at every step the integrator takes
([Time integration and events](integration.md)), not at a table of recorded rows:

1. It evaluates the equations of motion at each step's start, middle and end.
2. It fits a parabola through the three values. When the parabola bends down with its top inside
   the step, the peak may lie inside. A golden-section search then looks for it on the step's
   dense output, the integrator's smooth curve through the step, to a billionth of the time
   since launch (or of 1 s early in the flight).
3. It keeps the largest of what the search finds and the three samples.

Thrust-curve knots, the times where a motor's tabulated thrust changes slope, end steps. So a spike
in the thrust curve is a step's end, and is never averaged away. Loft took its peak acceleration
from a finite difference of the recorded speed, and read it low. Valetudo in a vacuum shows how
much: the watcher's peak matches the hand value from the motor and the masses within the test's
1e-6, and a finite difference of the speed recorded at 100 Hz reads it more than 1% low.

| Peak | What it is |
|---|---|
| Top speed | The center of mass's speed relative to the ground, after liftoff |
| Top Mach number | The airspeed over the local speed of sound |
| Max q | The largest dynamic pressure, ½ρv² on the airspeed |
| Boost acceleration | The largest acceleration of the nose tip, from liftoff until a recovery device opens |
| Opening shock | The largest acceleration while a device is open, the [opening load](../glossary.md#opening-load) over the mass. It follows hpr's inflation model ([Recovery](recovery.md)) |

The accelerations are of the nose tip, which is the body's origin in hpr's
[frames](frames.md). It differs from the center of mass's only by the rocket's turning, which is
small in a straight boost, and by the center of mass's slow drift forward as propellant burns. The accelerations are relative to the
[launch frame](../glossary.md#launch-frame-enu) and straight from the equations of motion, so they
include gravity's pull, as a trajectory's acceleration does. An accelerometer reads something
else: it does not feel gravity.

Nothing is kept while the rocket sits on the pad. A rocket that never lifts off has no peaks at
all: `None`, not zero.

## Stability margins

The margin is how far the [center of pressure](../glossary.md#center-of-pressure-cp) lies behind
the center of mass, in [calibres](../glossary.md#calibre-caliber) of the reference diameter `d`:

```text
margin = (x_cp − x_cg) / d,    x_cp = Σ C_Nα,i x_i / Σ C_Nα,i
```

Here `x` is a station measured aft of the nose tip, and `C_Nα,i` is component `i`'s
[normal-force slope](../glossary.md#normal-force-slope). hpr gives two margins at each instant:

- **Static margin:** the center of pressure at zero angle of attack and Mach 0, against the center
  of mass of that instant. This is RocketPy's `static_margin`. It changes only as propellant burns.
- **Flight margin:** the center of pressure at the flight's own Mach number, still with the air
  along the axis, against the center of mass of that instant. It is defined as RocketPy's
  `stability_margin` is. RocketPy's `min_stability_margin` takes the least over its whole flight,
  on the rail and in the descent too, at its solver's steps, so it can differ from hpr's least
  below. No page compares hpr's margins with RocketPy's yet. Against OpenRocket, in calm air at
  rod clearance, where a rocket is still slow, hpr's margin at the flight's Mach number is within
  0.016 calibres on 41 of the 53 flights of OpenRocket's own examples. This comparison takes hpr's
  margin with the air along the rocket's axis (the 0° plane), as OpenRocket's margin is, not in
  hpr's weakest plane (below). On `[C6-7; B6-0]` of *Pods--powered with recovery deployment*,
  hpr's margin at rod clearance in the report is +3.74 calibres along the axis, while `hpr sim`
  prints a least margin of −4.21 in the weakest plane. Where the two programs differ: on the
  *Tube fin rocket* hpr's is 1.08 calibres below OpenRocket's, [tube fins](aero.md#tube-fins); on the three-stage
  example's three flights 0.039 to 0.058 below, cause not yet sized,
  [#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185); and on the five of *Pods--airframes and
  winglets* 0.071 to 0.076 above, the flattering side,
  [#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325) and
  [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326); and on the three of *Pods--powered with
  recovery deployment* 0.070 above, also not yet traced. On 35 flights of private designs it is
  from 0.017 lower to 0.11 higher, cause not yet traced ([Accuracy](../accuracy.md)). Near Mach 1
  hpr puts the Arcas Robin's center of pressure up to 2.36 calibres behind the wind tunnel's
  ([Normal force through Mach 1](aero.md#normal-force-through-mach-1)), so there its flight margin
  reads high.

  The Mach number moves the center of pressure. On a rocket with fins only at the tail it
  generally moves aft up to where supersonic theory starts for the fins, Mach 1.2 or later: their
  slope grows, and from Mach 0.8 their own center moves aft too. Past that their slope falls and
  the body's lift can grow, and the center of pressure mostly moves forward again
  ([Your rocket's center of pressure](aero.md#your-rockets-center-of-pressure)). Valetudo is slow:
  at its rail exit, at Mach 0.051, both margins read 3.09 calibres.

**Both margins are the weakest direction's.** The air can cross a rocket from any direction around
its axis, and for most rockets the margin is the same in every one. A fin set of one or two fins
breaks that: it carries `Σ sin²(φ − θ_k)` of its force in the plane at roll angle `φ`, for fins at
angles `θ_k` (Niskanen 2009, eq. 3.51, as in [Fins](aero.md#fins)). So a rocket with one has a margin for
each direction, and hpr gives the least, the direction it was found in as `roll_rad`. Until
[#329](https://github.com/nrdptel/fusionspace-eridanus/issues/329), hpr's margins were the 0° direction's
alone, which can be the strongest. A rocket whose fin sets each have three or more fins, or that
flies a normal-force table, keeps its one margin, bit for bit.

A worked case: the sustainer of OpenRocket's *Pods--powered with recovery deployment* example has
a single fin at 0° and a two-fin strake set at 90°. At Mach 0.2, after the booster drops, its
margin is +1.86 calibres with the air crossing at 0°, where the single fin lies edge-on and the
strakes carry the force. At 45° it is +0.17. At 90° the strakes lie edge-on and only the single
fin works: −3.44, unstable. OpenRocket 24.12, asked for that sustainer at 90° in a scratch run
(not committed), puts its center of pressure at 0.2954 m with `C_Nα` 3.49; hpr's are 0.2993 m and
3.47. For one of its staged configurations, `[C6-7; B6-0]`, `hpr sim`'s least margin was +1.56
calibres and is now −4.21, at 0.86 s. That is the sustainer's margin at the split itself, the
moment the booster drops, with its own motor just lit and still full and the air at Mach 0.07;
the −3.44 is a later instant, at Mach 0.2. In the conditions of OpenRocket's record, both programs' flights of that
configuration turn over before apogee: its site is at 28.61° N, where the Earth's rotation tips
hpr's flight, and hpr's angle of attack passes 90° at 2.25 s. At `hpr sim`'s default site, on the
equator, nothing tips hpr's flight: it stays upright and reaches 251.3 m
([the format guide](../format/ork.md#hprs-flights-against-openrockets)).

hpr finds the least in closed form, not by a scan
([`weakest_margin`](../api/hpr_sim/metrics/fn.weakest_margin.html)). Each part's slope varies as
`a + b cos 2φ + c sin 2φ`, so the net slope `S(φ)` and its moment `M(φ) = Σ C_Nα,i x_i` do too.
Three directions, 0°, 60° and 120°, give each sum's `a`, `b` and `c`. The center of pressure `M/S`
is foremost where

```text
P sin 2φ + Q cos 2φ + R = 0,   P = m₀s₁ − m₁s₀,   Q = m₂s₀ − m₀s₂,   R = m₂s₁ − m₁s₂
```

with `m` and `s` the coefficients of `M` and `S`. The margin's conditioning (below) is checked at
its own worst direction the same way. Each root and the 0° direction are evaluated through the
model, and a direction where no margin can be given wins over any number.

Both margins leave out the [angle of attack](../glossary.md#angle-of-attack). In hpr,
[body lift](aero.md#body-lift) grows with the angle and acts at each body's side-view centroid, and
the center of pressure moves toward it. Where that centroid lies ahead of the zero-angle center of
pressure, the margin at an angle shrinks; on a long body with small fins it can lie behind, and the
margin grows. No test pins either direction. But the angle is not a steady property of
the rocket. Valetudo leaves its rail 17.2° off the oncoming air in the example's 5 m/s crosswind,
and near apogee the angle swings toward 90° as the rocket slows and tips over. If the least margin
followed the angle, it would land wherever hpr chose to stop counting large angles, and hpr models
no fin [stall](../glossary.md#stall) that could say where that is. For the margin at a given
angle, call [`margin`](../api/hpr_sim/metrics/fn.margin.html) with
[`Flow::new(mach, angle_rad, roll_rad)`](../api/hpr_aero/model/struct.Flow.html#method.new).

The watcher keeps both margins in `FlightMetrics::stability()`. The series starts at the rail exit
and ends at apogee or when a recovery device opens, whichever comes first. Before the rail exit the
rail holds the rocket, so its margin says nothing about how it flies.

The summary's least margins cover the same span. Each is looked for inside every step, as a peak
is ([Peaks](#peaks)): wherever the margin is defined at a step's start, middle and end and the
parabola through them bends up with its bottom inside the step, a golden-section search finds the
bottom. A later least replaces an earlier one only when lower by more than rounding, so a flat
least keeps its first time. A margin with a sharp corner
near a step's end can still hide from the parabola.

- On four test flights, and in the example, both leasts come at the rail exit, where the rocket is
  heaviest and its center of mass furthest aft. Flown in steps of at most 1 ms, those flights give
  the same leasts to a millionth of a calibre.
- On a two-stage test flight the least flight margin comes inside a step, after the split. There
  it matches a scan of every step at 201 points to 1e-7 calibres, and is never above it.

After a powered separation the margins are the sustainer's, in its own diameter. The split has two
entries in the series: the stack's, then the sustainer's.

### When there is no margin

As the net slope `Σ C_Nα,i` goes to zero, the air's loads become a pure couple: a turning moment
with no line of action. The quotient `x_cp` then runs away. That is how Loft came to publish
margins of ±12 to 15 calibres.

hpr judges the net slope against the sum of its terms' sizes:

```text
κ = Σ |C_Nα,i| / Σ C_Nα,i
```

Each part the model adds up acts at a station `x_i` on the rocket, within its length `L`. The
center of pressure then lies within `κ L` of every station. So a fractional error `ε` in one
part's slope moves the center of pressure by at most about `ε κ² L` (to first order). A part that
is a pure couple on its own (below) has no station, and `κ` doesn't count it.

- A rocket whose parts all push the same way has `κ = 1`, and a 1% error moves its center of
  pressure by at most 1% of its length. All 13 designs in
  [`validation/designs/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/designs) stay
  below 1.5 at Mach 0, 0.3, 0.8, 1.2 and 2 and at angles of attack of 0°, 5°, 10° and 20°.
- At `κ = √10 = 3.16`, a 1% error in one slope can move the center of pressure by a tenth of the
  rocket. Past that, or when the net slope is not positive, hpr gives no margin and no center of
  pressure: `None`. The limit is a chosen bound on that sensitivity, not a measurement.

The pitch-moment slope about the center of mass is always given, because it stays finite:

```text
C_mα = −(Σ C_Nα,i x_i − x_cg Σ C_Nα,i) / d
```

A negative `C_mα` turns the nose back into the wind. With a margin defined, `C_mα = −C_Nα · margin`.

A worked case, with no fins, pinned by a test:

- The rocket is a conical nose 0.3 m long of radius `R` = 0.05 m, a 0.5 m tube, and a conical
  boattail 0.4 m long, narrowing to a radius `r`. So `d` = 0.1 m.
- Barrowman gives the nose a slope of 2 at 0.2 m. The boattail gets `2((r/R)² − 1)`, at
  `0.8 + (0.4/3)(1 + 1/(1 + R/r))` m.
- So `κ = 2/ρ² − 1`, with `ρ = r/R`. The margin is given for `ρ ≥ 0.6932`, an aft radius of
  34.66 mm or more.
- The center of mass is at 0.5 m.

| Boattail's aft radius | Net slope | κ | Margin | `C_mα` |
|---|---|---|---|---|
| 40 mm | 1.28 | 2.13 | −7.46 cal | +9.55 |
| 35 mm | 0.98 | 3.08 | −11.2 cal | +10.98 |
| 34.5 mm | 0.952 | 3.20 | none (the quotient: −11.7 cal) | +11.11 |
| 21.5 mm | 0.370 | 9.82 | none (the quotient: −37.1 cal) | +13.72 |
| 5 mm | 0.020 | 199 | none (the quotient: −741 cal) | +14.82 |

With no fins, every one of these rockets is unstable: its center of pressure lies ahead of its
center of mass, and `C_mα` is positive. The cut is not about how large the margin is. The 35 mm and
34.5 mm rows have margins near −11 calibres; what differs is whether the quotient can be trusted.

A component can be a pure couple on its own. A step down in radius followed by a flare back up
cancels its own slope and still turns the rocket. `C_mα` keeps its moment, which a test also
checks.

## The largest angle of attack

`max_angle_of_attack_rad` is the largest angle between the rocket's axis and the air flowing past
it, in radians, over the span the envelope's high-angle flag watches
([Validation plan: the operating envelope](../VALIDATION.md#operating-envelope);
[ADR-179](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0179-the-envelope-flags.md)):

- from more than 1 s after the rail exit, as the first second off the rail is the expected
  transient in a crosswind;
- to apogee or the first deployment, the span of the least margins above;
- only at instants where a 15° angle would give a
  [normal force](../glossary.md#normal-force) of at least a fifth of the rocket's weight:
  `q (π d²/4) |C_Nα| · 15° ≥ 0.2 m g`, with `q` the
  [dynamic pressure](../glossary.md#dynamic-pressure), `d` the reference diameter and `C_Nα` the
  flight margin's normal-force slope.

The angle is read at each step's start, middle and end, without the search between them that the
peaks have, so an excursion inside one step can be missed. Above 15°, the flight's
`envelope_flags` include `high_angle_of_attack`.

The floor is there because near apogee every flight whose path turns over passes 15°, while the air
is too weak to turn it: gravity does. Valetudo, one of RocketPy's examples, launched off an 84° rail
in calm air, passes 15° about 0.9 s before apogee, at 16.7 m/s through the air, a little faster
than its 16.2 m/s rail exit. There a 15° angle would give a normal force of 1.0% to 1.5% of its
weight, so an error in the aerodynamics at that angle barely moves it. A wind layer met at speed is
different: Valetudo climbing into 30 m/s of wind 300 m up passes 15° where that force is 62% of its
weight, and the flag is raised. On the tests' RocketPy rockets, turn-overs give 0.7% to 5.0% and
wind layers met at speed 62% to 141%; the fifth, chosen rather than measured, sits between. The
tests `a_tilted_calm_climb_passes_15_degrees_only_with_too_little_force`,
`a_wind_layer_met_at_speed_raises_the_high_angle_flag` and
`a_wind_layer_met_in_a_fast_coast_raises_the_high_angle_flag` (in `hpr-sim`'s `metrics.rs`) fly
these cases.

## Unstable under power

A flight that is unstable while a motor burns raises one of two flags, and its apogee is not a
prediction ([#335](https://github.com/nrdptel/fusionspace-eridanus/issues/335)). Unstable means the air turns
the rocket away from its path instead of back. Where it ends up then depends on small
disturbances, and on aerodynamics that hold only at small angles of attack. hpr still flies it,
and the flag changes no number: it marks the flight's path and apogee as not a prediction. A
flight raises at most one of the two flags.

- `unstable_under_power`: the static margin falls below zero while a motor burns. The center of
  pressure is then ahead of the center of mass.
- `unstable_without_margin`: where hpr can give no static margin while a motor burns
  ([When there is no margin](#when-there-is-no-margin)), the pitching moment's slope `C_mα` is
  above zero. It is raised only when the first flag isn't.

`min_powered_static_margin_cal` is the margin the first flag reads. It is the least static margin,
in the weakest plane ([Stability margins](#stability-margins)), from the rail exit to apogee or
the first deployment, over the moments a motor burns. It is found inside steps, as the other least
margins are. A step counts when the thrust at its middle is above zero. hpr ends a step at every
ignition, thrust-curve point and burnout, so a motor that burns at a step's middle burns through
all of it, and the margin at the burnout itself counts. A margin of exactly zero raises nothing;
any margin below it does.

`max_powered_moment_slope_per_rad` is what the second flag reads. Where the parts' normal-force
slopes nearly cancel, or their sum is not positive, hpr gives no margin, since the center of
pressure would be noise. The moment about the center of mass is still well defined there:

```text
C_mα = −(Σ C_Nα,i x_i − C_Nα x_cg) / d
```

with `C_Nα,i` and `x_i` the normal-force slope and station of part `i`, `C_Nα` their sum, `x_cg`
the center of mass, every station aft of the nose tip, and `d` the reference diameter. A negative
`C_mα` turns
the rocket back toward its path; a positive one turns it away. hpr keeps the largest `C_mα` at
each powered step's start, middle and end where the margin is undefined, without searching
between them. A `C_mα` of exactly zero raises nothing; any above it does. Without this flag, a
rocket so unstable that its margin is undefined would raise no warning at all, while a milder one
would.

Only the moments under power count. A rocket that turns then is driven along its new heading by
its own motor, so the path flown follows the turn. A rocket that is unstable only in the coast,
after its motors burn out, raises neither flag: it has no thrust to carry it off its path, and
the least static margin over the whole span, `min_static_margin_cal`, still prints its margin
there.

For example, OpenRocket's example *Pods--powered with recovery deployment*, flown on `[C6-7; B6-0]`
or `[C6-7; 2× A3-4, B6-0]`, separates at 0.86 s. The sustainer left has a static margin of −4.21
calibres with its C6 still burning. On `[C6-7; B6-0]`:

```text
static margin         2.05 calibres off the rail; least -4.21 calibres, at 0.86 s, before apogee
apogee                251.3 m above the site at 5.74 s, not a prediction: unstable under power
warning: unstable: the static margin falls to -4.21 calibres at 0.86 s while a motor burns: the rocket is unstable under power, so the air turns it away from its path rather than back, and its apogee is not a prediction
```

For the second flag, take RocketPy's Valetudo with its fins taken off and a conical tail that
narrows from the body's 40.45 mm radius to 22 mm over 0.08 m. At the rail exit its net slope is
0.59 per radian against a sum of sizes of 3.41, too small for a margin, but `C_mα` is +43.9 per
radian:

```text
apogee                399.1 m above the site at 8.46 s, not a prediction: unstable under power
warning: unstable: while a motor burns, the pitching moment turns the rocket away from its path (C_mα +43.9 per radian at 0.25 s) where hpr can give no static margin: the rocket is unstable under power, so its apogee is not a prediction
```

With the tail ending at 30 mm instead, the margin is defined, −34.37 calibres, and the first flag
fires in its place. These figures are from a run on 2026-10-06; the test
(`sim_flags_a_rocket_unstable_where_it_has_no_margin`) pins only the flag and a slope above 1.

`hpr sim` prints the `warning: unstable:` line and marks the apogee for either flag, and its JSON
lists the flag under `flags`; the library has it in `FlightSummary::envelope_flags`, and Python in
`Flight.envelope_flags`. When this was written (2026-10-06), none of the 25 public `.ork` files
and 12 validation designs that `hpr sim` flies offline raised either flag, on their default
configurations off an 85° rail in calm air and in 8 m/s of wind. No test or report pins that
count.

What it leaves out:

- A rocket unstable only after its motors burn out raises no flag, as above.
- It reads the static margin and `C_mα` at Mach 0, not at the flight's Mach number.
- `C_mα` is read only at each step's start, middle and end, not searched between: a moment
  slope above zero for less than half a step, with no margin, can pass unseen.
- A flight that tips over above 15° raises the high-angle flag
  ([The largest angle of attack](#the-largest-angle-of-attack)) as well; a stable rocket in a
  strong wind can raise that one alone.

The tests `the_powered_margin_counts_only_while_a_motor_burns`,
`a_moment_slope_without_a_margin_counts_only_while_a_motor_burns` and
`a_rocket_without_fins_is_unstable_under_power` (in `hpr-sim`'s `metrics.rs`), and
`the_stability_flag_starts_just_below_a_zero_margin` and
`the_moment_flag_starts_just_above_zero_and_only_without_the_margin_flag` (in `envelope.rs`) pin
it: a margin of −2⁻³⁰ calibres at a burnout raises the flag, +2⁻³⁰ doesn't, nor does −1 calibre
in the coast that follows; with no margin, a `C_mα` of +2⁻³⁰ per radian under power raises the
second flag, −2⁻³⁰ doesn't, nor does +2⁻³⁰ after the burnout. In `hpr-cli`,
`sim_flags_a_rocket_unstable_where_it_has_no_margin` flies the Valetudo example above.

## Optimum ejection delay

The optimum delay is the time from a motor's burnout to apogee: the delay that fires its charge at
the top. [`optimum_delays`](../api/hpr_sim/metrics/fn.optimum_delays.html) flies the rocket again
with every recovery charge held, so that it coasts to the apogee it would reach untouched. So the
answer belongs to the rocket, its motors and its air, not to the delay flown. A charge that fires
too early cuts the coast short. Loft's optimum then came out too short as well, which advised an
even shorter delay. hpr gives the same optimum for delays of 1 s and 20 s.

- A [separation](../glossary.md#separation) with nothing ahead of it left to burn is part of the
  recovery, so it is held too. A two-stage rocket that separates some seconds after its last burnout gets
  the same optimum for a 3 s delay as for a 30 s one.
- A powered separation still happens. The motors in the booster it drops have no optimum: their
  charges fire in the booster, which never reaches the [sustainer's](../glossary.md#sustainer)
  apogee.
- A motor that burns out after the apogee, or never lights, has none either. A flight that never
  reaches an apogee (it never lifts off, or hits its time cap) gives `None`.

## Landing points

A landing is where the center of mass came back down to the site's height. It is given as a WGS 84
latitude and longitude, as east and north meters from the site in its local frame, and as the speed
at the ground hit. The flight's own landing is the stack's (the whole rocket before any
separation), or after a powered separation the sustainer's. Each separated body that lands has its
own landing, with its body number: 0 keeps the nose, and 1 is the stages aft of the split.

A flight that did not land has no landing and no ground-hit speed: `None`. A test checks this for a
flight stopped by its time cap, and the JSON it writes, where the missing values are `null`.

## Tests

In `crates/hpr-sim/src/metrics.rs`, unless named otherwise:

| Test | What it pins |
|---|---|
| `static_margin_undefined_when_cn_alpha_near_zero` | The boattail table above, against Barrowman by hand, to 1e-9 relative, on both sides of the limit ([L33](../decisions-and-roadmap.md#l33)) |
| `ordinary_rockets_keep_their_margin` | All 13 validation designs keep their margin at Mach 0, 0.3, 0.8, 1.2 and 2 and at 0°, 5°, 10° and 20°, with `κ` below 1.5 |
| `a_pure_couple_keeps_its_moment` | The step and flare above: `C_mα` against the hand value, to 1e-9 |
| `a_normal_force_table_gives_its_own_margin` | A table's margin and `C_mα` against its own numbers |
| `the_least_margin_is_the_weakest_plane_s` | Six layouts of a single fin and a two-fin set, at Mach 0.1, 0.6 and 1.5 and three centers of mass: the least margin is never above any of 3,600 scanned planes', and within 1e-5 calibres of the scan's least, including where the weakest plane lies between the fins' planes; fin sets of three or more fins keep the margin along the axis, bit for bit |
| `peak_acceleration_is_analytic_and_excludes_opening_shock` | The boost's peak in a vacuum against the hand value, to 1e-6; a 100 Hz finite difference reads it more than 1% low; an opening shock over three times the boost's is kept apart ([L34](../decisions-and-roadmap.md#l34)) |
| `unlanded_flight_has_no_ground_hit_speed_and_outputs_name_datum` | `None` and `null` for an unlanded flight; the launch height against the rail's geometry, to 1e-9 relative ([L35](../decisions-and-roadmap.md#l35)) |
| `optimum_delay_independent_of_flown_delay` | Delays of 1 s and 20 s give the same optimum, equal to a flight with no recovery ([L94](../decisions-and-roadmap.md#l94)) |
| `peaks_are_refined_inside_steps` | On three rockets, max q and top Mach are above every row of a 1 ms record, and the record's best row is within 0.01% of them; on Valetudo max q comes before top speed, and top speed before top Mach |
| `landings_are_placed_on_the_ellipsoid` | A landing more than 100 m downwind, against the radii of curvature at the site, to second order |
| `stability_is_kept_from_rail_exit_to_apogee` | The series' ends; the static margin at the rail exit and, in a crosswind, the flight margin at the flight's Mach number, against the model and the masses directly, to 1e-12; the least flight margin is at or below every entry |
| `least_margins_do_not_depend_on_where_steps_end` | Valetudo in calm air off a vertical and an 84° rail and in a crosswind, and Prometheus: both leasts are at the rail exit, and agree with steps of at most 1 ms to 1e-6 calibres |
| `a_least_margin_between_step_ends_is_found` | A margin of `2 + (t − 0.37)²` across a step: the search finds 2 at 0.37 s; a margin falling across the step, or undefined in its middle, is not searched |
| `a_summary_needs_the_flight_it_watched` | A watcher refuses a flight when it missed some of its steps or also watched another flight, naming the steps it saw, and sums up one that ended without a step; a flight started in the air has no launch height, and one started on its way down keeps no stability |
| `the_powered_margin_counts_only_while_a_motor_burns` | A margin falling 1 calibre a second across a burnout at 1 s: −2⁻³⁰ calibres at the burnout raises the flag, +2⁻³⁰ doesn't, nor does −1 in the coast after; a least inside a powered step is found by the search; a NaN under power is kept, one in the coast isn't; a margin of −10⁻¹³ replaces +10⁻¹³ though the two are closer than the tie |
| `a_moment_slope_without_a_margin_counts_only_while_a_motor_burns` | With no margin, a `C_mα` of +2⁻³⁰ per radian under power raises `unstable_without_margin`, −2⁻³⁰ doesn't, nor does +2⁻³⁰ after the burnout at 1 s; a NaN under power is kept; a slope where the margin is defined doesn't count; a margin below zero as well raises only `unstable_under_power` |
| `a_rocket_without_fins_is_unstable_under_power` | Valetudo with its fins taken off raises `unstable_under_power` at or before its burnout; stock, its least margin under power is above 1 calibre and it raises nothing |
| `envelope::tests::the_stability_flag_starts_just_below_a_zero_margin` | The flag from the least negative `f64` down; none at 0, −0 or above |
| `envelope::tests::the_moment_flag_starts_just_above_zero_and_only_without_the_margin_flag` | `unstable_without_margin` from the least positive `f64` up, and for a NaN; none at 0, −0 or below; never beside `unstable_under_power` |
| `staging::tests::metrics_follow_a_powered_separation` | Both landings; the sustainer's diameter after the split, with its own entry there; both leasts against a scan of every step at 201 points, to 1e-7 calibres, the flight margin's inside a step; no optimum for the booster's motor |
| `staging::tests::a_held_flight_holds_a_separation_after_the_last_burnout` | A split after the last burnout is held: the same optimum for delays of 3 s and 30 s |

## References

- J. Kiefer, "Sequential minimax search for a maximum", *Proceedings of the American Mathematical
  Society* 4(3), 502–506, 1953. The golden-section search.
- J. S. Barrowman and J. A. Barrowman, "The theoretical prediction of the center of pressure",
  NARAM-8 research and development report, 1966. The slopes and stations, as
  [Aerodynamics](aero.md) uses them.
- S. Niskanen, *Development of an Open Source model rocket simulation software*, MSc thesis,
  Helsinki University of Technology, 2009, eq. 3.51. A fin's share of its force in the plane the
  air crosses in, which makes a one- or two-fin set's margin depend on that plane.

The decision behind these choices is [ADR-077][adr-077], from the
[M1.10a milestone](../decisions-and-roadmap.md#m1-10a).

[adr-077]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0077-flight-metrics-peaks-on-the-dense-output-margins.md
