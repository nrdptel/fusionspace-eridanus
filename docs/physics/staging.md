# Staging

## In short

- **What it models:** motors that light at their own times, and a rocket that drops its booster
  under power, or several stages in turn. A motor can light at launch, at a time, a delay after another motor burns out, or
  a delay after its stage is freed, or never. When the stack comes apart
  ([separation](../glossary.md#separation)) with the forward part still to burn, that part (the
  [sustainer](../glossary.md#sustainer)) flies on with its own shape and mass, and the aft part
  (the [booster](../glossary.md#booster)) falls back to its own landing.
- **Sources:** no new physics. The flight is the same rigid-body flight
  ([Rigid-body flight](flight.md)), the masses are the same sums ([The design
  tree](design.md#motors-and-configurations)), and the booster's descent is the same descent
  under a recovery device ([Recovery](recovery.md#separation)). What is new is when each motor
  burns, and which rocket is flying.
- **How well it is validated:** against OpenRocket, not against a real flight. OpenRocket's
  two-stage, three-stage, cluster and air-start examples, read from their `.ork` files, all fly
  within 5% of OpenRocket's apogee and largest speed, flown on OpenRocket's own curves. Five
  apogees are compared with OpenRocket's flight with no parachute, since its parachute opened
  before apogee ([Against OpenRocket](#against-openrocket)). `hpr sim`, on the curves it
  fetches, flies the three-stage example within 5% too. A payload dropped with nothing left to
  burn flies under its own parachute from the split: on OpenRocket's two payload examples, its
  apogee is within 1.1% of OpenRocket's ([A payload dropped with nothing left to
  burn](#a-payload-dropped-with-nothing-left-to-burn)). Boosters strapped beside the core burn
  with it and drop at their own separation: OpenRocket's *Parallel booster staging* flies within
  1.23% of its apogee ([Boosters beside the core](#boosters-beside-the-core)).
  Tests pin the bookkeeping: ignition times to 1e-12 s, the mass step at the split to 1e-12 of the
  mass, and, while the sustainer is not yet burning, the two parts' momenta to 1e-9 of the stack's.
  The flight of a dropped booster or middle stage after its split is not validated at all.
- **What it leaves out:**
  - **The booster's own airframe drag.** After the split the booster is a point: a mass with only
    its recovery device's drag. So the simulator requires a device on it that opens at the
    separation, usually [tumbling](../glossary.md#tumble-recovery), and refuses the flight
    otherwise. It tumbles the booster side-on from the instant it separates, while a real finned
    booster flies nose-first for a while first, so it slows far faster than a real one would. Treat
    the booster's peak and landing point as rough, likely too low and too close to the pad
    ([#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179), a model of the booster's
    own drag).
  - Any push from the separation (no charge or spring), the air flowing between the parts as they
    come apart, and the booster's orientation as it falls.
  - A drag or normal-force table from another program (`Simulation::with_drag_table`), or a drag
    model of your own ([Models of your own](../custom-models.md)): it describes the whole stack,
    so a powered separation under one is refused.
  - With more than one separation, one that leaves nothing ahead of it to burn, such as a payload
    released after the stages ([Several separations](#several-separations)).
  - **The thrust a booster dropped still burning had left.** A `.ork` stage's first burnout can
    drop the stage's other motors while they burn, as OpenRocket does. The dropped part flies with
    no thrust, so its flight leaves out the impulse they had left, which `hpr sim` states
    ([A booster dropped still burning](#a-booster-dropped-still-burning)).

## Using it today

Staging is set in a JSON design file or in code, or read from a `.ork` file. `hpr sim` flies a
`.ork` file's powered separation, each parachute and streamer on its own part
([Separation](../cli.md#separation); [M4.5g1](../decisions-and-roadmap.md#m4-5g1), powered
separation in `hpr sim`). It flies several powered separations in turn
([Several separations](#several-separations)), and a payload dropped with nothing left to burn
when its own device opens at the split ([A payload dropped with nothing left to
burn](#a-payload-dropped-with-nothing-left-to-burn)). It flies the stack whole when the
separation can only come after apogee. To fly a
`.ork` file's staging from a program, start from the example
[`ork_two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs),
and see how such flights compare with OpenRocket's in [Against OpenRocket](#against-openrocket).
For a design of your own, start from the two-stage design
[`synthetic-two-stage-75mm-54mm.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/designs/synthetic-two-stage-75mm-54mm.json)
and the [worked example](#a-worked-example) below. The decision record is [ADR-074][adr-074].

## When a motor lights

Each motor in a [configuration](../glossary.md#configuration) has an `ignition`
([`Ignition`][ignition]). The flight's clock starts at launch (`t = 0`). Each motor burns on its own
clock from its ignition, so its thrust curve and its mass are its own curve shifted to that time.

| `ignition` | lights at | example in a design file |
|---|---|---|
| `launch` (the default) | `t = 0` | nothing written |
| `time` | a time after launch | `"ignition": {"time": {"time_s": 4.0}}` |
| `burnout` | a delay after the motor in another mount burns out | `"ignition": {"burnout": {"mount": "booster-motor-mount", "delay_s": 1.0}}` |
| `separation` | a delay after the separation that drops the stages behind this motor's stage | `"ignition": {"separation": {"delay_s": 0.5}}` |
| `never` | never: it rides loaded, as a motor that fails to light does | `"ignition": "never"` |

**A design that says nothing lights every motor on the pad**, the sustainer's too, and the simulator
does not warn. For a staged flight, give the sustainer's motor its `ignition` and the flight a
`Separation`.

Before a motor lights it is **loaded**: its full propellant mass sits in the rocket, and it gives
no thrust. A motor that never lights, because it is set `never`, or the burnout or separation it
waits for never comes, is carried loaded to the ground. Every ignition time known before the flight, and every point of each shifted thrust curve,
is a [stop time](../glossary.md#stop-time): the integrator ends a step there, so no step starts a
burn half way through ([Time integration](integration.md)). An ignition that waits on a
separation becomes a stop time when the separation fires. The design refuses:

- a burnout of a mount with no motor, or a chain of burnouts that comes back to the motor itself;
- a separation ignition in the aft-most stage, which has nothing aft of it to separate;
- a time or delay that is negative or not a number.

## Powered separation

A separation has a trigger and a stage boundary ([Recovery](recovery.md#separation)). Besides
apogee, a height on the way down, a time, and a motor's [ejection
delay](../glossary.md#ejection-delay), it can now fire a delay after a motor's burnout
(`Trigger::Burnout`).

When it fires, the simulator looks at the forward part, body 0, which keeps the nose:

- **If it still has a motor to burn**, it is a sustainer. That covers a motor burning now, one due
  to light at a known time, and one lit by this separation. The sustainer flies on in [six degrees
  of freedom](../glossary.md#6-dof-six-degrees-of-freedom), and the booster (body 1) descends
  under its own device.
- **If it has nothing to burn**, both parts descend under their devices, as before.

What the simulator refuses:

- **A booster still burning.** Its motors must have burned out, unless the separation is told it
  may drop one ([A booster dropped still burning](#a-booster-dropped-still-burning)). If the
  separation's time is known before the flight, building the `Simulation` returns the error;
  otherwise `run` returns it when the separation fires, with no flight result.
- **A booster with nothing open at the split** (see *What it leaves out*, above). A device on the
  booster with `Trigger::Time { time_s: 0.0 }` and no
  [lag](recovery.md#triggers-lag-and-release) opens there, because a booster's devices
  act only once it flies on its own.
- **A separation that could never fire**, such as one timed from the burnout of the sustainer it
  lights.

**Why the state carries straight across.** The simulator's flight state is the motion of the nose
tip, not of the center of mass ([Rigid-body flight](flight.md)). The sustainer keeps the nose, so
the nose tip's position, velocity, attitude and rotation rate are all still right for it. What
changes is the rocket they belong to:

- **Mass.** The mass drops by exactly the booster's: its stages plus its spent motors. The center
  of mass and the inertia are the sustainer's own stages and motors.
- **Aerodynamics.** The simulator builds the sustainer's aerodynamics from the design cut after the
  stage boundary: the design with the booster's stages removed. It is an ordinary rocket with a
  nose and its own aft end, so its drag includes its own [base](../glossary.md#base-drag). Its
  [reference area](../glossary.md#reference-area) can be smaller than the stack's (the widest body
  may have been the booster's), so a recorded coefficient such as `Sample::axial_coefficient`
  steps at the split even where the force doesn't.

The integrator restarts at the separation, because the mass steps there.

**Momentum.** Nothing pushes the parts apart. Write `m` for a mass, `v` for the velocity of a
center of mass, and `s`, `b` for the sustainer and the booster. The sustainer keeps the nose tip's
velocity `v_O` and rotation `ω`. The booster leaves with the velocity its own center of mass
already had, `v_O + ω × r`, where `r` runs from the nose tip to that center. So the two momenta add
up to the stack's, `m v = m_s v_s + m_b v_b`. A test checks this to 1e-9 of the stack's momentum
on a flight launched 10° off vertical, whose sustainer is not yet lit. With the sustainer burning,
`m v` is not simply additive, because the center of mass also moves inside the body as propellant
burns, so the simulator makes no claim for that case.

**Events.** Along with the separation, the flight records an `Ignition` event for each motor lit
after launch. `Burnout` is recorded when no motor is burning or due to light at a known time. In
the example below that is 5.23 s only: the booster burned out at 1.73 s, but the sustainer was
already due. With a sustainer lit by its separation, whose time isn't known in advance, `Burnout`
is recorded twice, at the booster's burnout and at the sustainer's.

**What the result holds.** The flight's events and final sample are the sustainer's, from the pad
to its landing. [`FlightResult::bodies`][bodies] holds the booster's descent, and
[`FlightResult::landings`][landings] gives both landings, the sustainer's first.

## A worked example

The program
[`crates/hpr-sim/examples/two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/two_stage.rs)
flies a synthetic two-stage design: a 54 mm sustainer on a 75 mm booster, with a J760 in the
booster and an I175 in the sustainer. The stages come apart 0.5 s after the booster's burnout, and
the sustainer lights 1 s after it. The ignition names the booster by its **mount's id**, as the
design file does. The separation's trigger names it by its **index** among the placed motors, as
every trigger does, so the program looks that up:

```rust
sustainer_motor.ignition = Ignition::Burnout {
    mount: "booster-motor-mount".to_owned(),
    delay_s: 1.0,
};
```

```rust
let booster = simulation
    .assembly()
    .motors
    .iter()
    .position(|motor| motor.mount == "booster-motor-mount")
    .ok_or("no booster motor")?;
```

```rust
Device::new("booster tumble", tumble, Trigger::Time { time_s: 0.0 }).on_body(1),
```

```rust
.with_separation(Separation::new(
    Trigger::Burnout {
        motor: booster,
        delay_s: 0.5,
    },
    0,
))?;
```

The `0` is the stage boundary: the stages from the nose through stage 0 are the sustainer. Run it
with `cargo run --example two_stage -p fusionspace-hpr-sim`. It prints:

<!-- quote: crates/hpr-sim/examples/two_stage.output.txt -->
```text
A 54 mm sustainer on a 75 mm booster, J760 then I175, calm air
Not yet validated: see the Accuracy page before trusting these numbers.

event                   time (s)   CG height (m)   speed (m/s)   mass (kg)
liftoff                     0.00             0.5           0.0       2.480
rail exit                   0.20             6.3          61.0       2.410
separation                  2.23           643.0         341.2       1.904
sustainer lights            2.73           797.4         280.7       0.779
sustainer burnout           5.23          1798.9         403.2       0.550
apogee                     18.42          3103.2           0.1       0.550
parachute charge           18.42          3103.2           0.1       0.550
parachute opens            18.42          3103.2           0.1       0.550
sustainer lands           865.34             0.0           3.4       0.550

The booster (1.125 kg) leaves at 2.23 s and 642.7 m, tumbling. It peaks at 745.0 m at 5.11 s and lands at 47.21 s at 17.9 m/s.
```

How to read it:

- **The split.** At 2.23 s the stack weighs 1.904 kg. The booster (its stage and a spent J760)
  takes 1.125 kg of that, so the sustainer flies on at 1.904 − 1.125 = 0.779 kg, which is the mass
  shown when it lights. The booster starts at 642.7 m rather than 643.0 m because its own center of
  mass sits 0.3 m below the stack's.
- **The coast.** Between the separation and the ignition the sustainer coasts for half a second
  and slows from 341.2 to 280.7 m/s. That is about 121 m/s², mostly drag, on a 0.78 kg rocket near
  Mach 1.
- **The burn.** The I175 burns for 2.5 s and the mass falls to 0.550 kg: the sustainer's
  structure and a spent motor.
- **Two landings.** The sustainer lands under its parachute at 3.4 m/s. The booster tumbles from
  the split, climbs only 102.3 m more (to 745.0 m), and lands at 17.9 m/s. That short climb comes
  from tumbling side-on from near Mach 1 at once, and is likely too short (see *What it leaves
  out*).

These numbers are for a made-up rocket that no other program has flown, so nothing checks them
beyond the tests below.

## Several separations

**A rocket of three stages or more flies, dropping its stages under power one after another, from
the tail forward.** At each split the stages behind it drop away and descend to their own
landing, and the rest flies on as a new sustainer. The simulator cuts that sustainer from the whole
design at the new boundary, as it cut the first, and carries the nose tip's state straight across,
as in [Powered separation](#powered-separation). This is checked against OpenRocket's three-stage
example, not against a real flight, and the dropped stages' own flights are not validated. The
decision record is [ADR-160][adr-160] (several powered separations).

**How it is set.** `Simulation::with_separations` takes the separations in the order they fire.
`with_separation` is a list of one, and a flight with one separation flies as it did before, to the
bit.

- **The order.** Each separation's stage boundary is forward of the one before. Separation `k`
  makes body `k + 1`: the stages from its boundary back to the boundary before it, or back to the
  tail. Body 0 keeps the nose. On a three-stage rocket the booster is body 1 and the middle stage
  body 2 (`Separation::body_of` and `Separation::stages_of_body` give the numbering).
- **Each split hands the flight on.** With more than one separation, each must leave the body
  with the nose a motor still to burn.
- **A motor lit at a split stays lit at the next one.** A motor that an earlier separation lit
  counts as burning from that split when the next one fires. So a middle stage whose motor the
  first split lit, and which has burnt out by the second, leaves with that motor's dry mass, not
  its loaded mass.
- **Where else the list goes.** The facade's
  [`FlightBuilder::separations`](../api/hpr/flight/struct.FlightBuilder.html#method.separations)
  takes the same list, and [`hpr::ork::separations`](../api/hpr/ork/fn.separations.html) makes it
  from a `.ork` configuration's stagings, from the tail forward.

**What is refused.**

- When the `Simulation` is built: boundaries that don't move forward; separation times known
  before the flight that come in the other order; and more than one separation together with
  ejections, since a sustainer cut from the design doesn't track the ejected pieces.
- In flight, by name: with more than one separation, one that leaves nothing ahead of it to burn,
  since once every body is a point mass no sustainer is left to part; and a separation that fires
  before the one listed ahead of it.
- Each dropped part must have a device open at its split, as a booster must.
- A `.ork` configuration is left out, with its reason, when a separation at or after apogee sits
  beside another, or when one comes before the separation behind it.

**A worked example: OpenRocket's three-stage example.** OpenRocket's *Three stage low power
rocket* drops its booster at the booster's burnout, as the middle stage lights. It drops the
middle stage at that stage's own burnout, as the sustainer lights. Each configuration is written
sustainer first: [A8-5; B6-0; B6-0] is an A8-5 in the sustainer above B6-0s in the middle stage
and the booster. `cargo xtask ork-flights` flies each configuration on OpenRocket's own thrust
curves, matched by their digests. Δ is HPR Sim less OpenRocket, in per cent of OpenRocket's:

| motors | apogee, OpenRocket (m) | Δ apogee | Δ largest speed | separations, OpenRocket / HPR Sim (s) | notes |
|---|---:|---:|---:|---|---|
| A8-5, B6-0, B6-0 | 277.1 | −1.37% | +0.25% | 0.857 / 0.857; 1.714 / 1.714 | nothing deployed |
| C6-5, B6-0, B6-0 | 505.4 | −0.95% | +1.51% | 0.857 / 0.857; 1.714 / 1.714 | nothing deployed |
| C6-7, C6-0, C6-0 | 689.5 | −1.48% | +1.64% | 1.860 / 1.860; 3.720 / 3.720 | |

How to read it:

- **The splits.** Both come at OpenRocket's times, to the 1 ms the report shows: at the booster's
  burnout, then at the middle stage's.
- **The apogee.** On the first two, OpenRocket's parachute opens before apogee, 0.45 s and 1.57 s
  early, so the apogee is held to OpenRocket's flight with nothing deployed, as for the clusters
  below. Against OpenRocket's own record those two read −1.24% and +0.98%.
- **The descent.** Flown with the file's own parachutes, the sustainer lands within 0.009% of
  OpenRocket's landing speed on each, and its flight time is −1.13%, −0.56% and −1.41% off
  OpenRocket's. All three meet every descent target
  ([Recovery](recovery.md#from-an-openrocket-file)).
- **The mass off the rod.** OpenRocket's recorded mass falls as if every motor of the booster's
  type burned from launch ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)). From launch to
  rod clearance it drops 2.0000, 1.9998 and 2.9944 times as much as HPR Sim's: the count of such
  motors in each configuration. So HPR Sim reads 1.80% to 3.26% heavier as the rocket leaves the rod,
  though the launch masses agree within 0.054%. At that moment HPR Sim's stability margin is 0.039 to
  0.058 calibres shorter than OpenRocket's, about as much as HPR Sim's center of mass sits further
  aft, 0.043 to 0.062 calibres. Whether OpenRocket's light mass accounts for that is not sized.
- **With the curves `hpr sim` fetches.** The file holds no curves, so `hpr sim` fetches them from
  ThrustCurve.org ([Motors from ThrustCurve.org](../cli.md#motors-from-thrustcurveorg)), taking
  the file that holds OpenRocket's curve for each motor's digest
  ([ADR-161](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0161-openrocket-curves-by-digest.md),
  OpenRocket's curves by digest). The largest speeds read 0.31%, 1.55% and 1.67% above
  OpenRocket's: 78.61, 121.55 and 132.02 m/s. The apogees, flown with the file's parachutes,
  read 1.15%, 0.58% and 1.37% below OpenRocket's own record, which opens them too; against its
  flight with nothing deployed, the table's figures, the first two read 1.28% and 2.48% below.
  Before HPR Sim kept a table of which ThrustCurve file holds each OpenRocket curve, the first
  configuration flew another A8 file, which burns out 0.536 s after lighting where OpenRocket's
  burns 0.73 s, and its largest speed read 5.7% high.

**What it leaves out.**

- The dropped middle stage's own flight, like the booster's: a point under its devices' drag,
  tumbling side-on from its split until a device of its own opens, so its peak and landing are
  rough
  ([#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179)).
- An unpowered separation after a powered one, such as a payload released after the stages: it
  is refused. A lone unpowered one flies only as
  [A payload dropped with nothing left to burn](#a-payload-dropped-with-nothing-left-to-burn)
  says.
- A motor lit by a separation can't time a later separation, since its burnout has no time
  before the flight. The `.ork` reader never lights a motor that way.

## A payload dropped with nothing left to burn

**When the stack comes apart with nothing ahead of the split left to burn, every part flies on as
a point with only its own devices' drag, so `hpr sim` flies it only when the part that keeps the
nose has a device of its own open by then.** That is how a payload section usually leaves: the
booster's ejection charge pushes it off after the motor has burned out, sometimes before apogee,
and its parachute opens at once. No new physics is involved. Each part is a point mass under its
open devices, the descent model of [Recovery](recovery.md#separation), from the instant it
separates, as OpenRocket flies a descent (its technical documentation v13.05, §4.2.5).

- **The payload's device.** A `.ork` device set to `lowerstageseparation` opens on the split's
  own trigger, plus its delay. A payload whose device opens later, or that has none, would coast
  with no drag, so `hpr::ork::tumbling` refuses it by name. It is not tumbled instead: a finless
  payload on its own is unstable, and nothing measured says how it flies. To fly such a file, set
  one of the payload's devices to open at the split: in OpenRocket, its deployment event "Lower
  stage separation" with no delay.
- **A device that fired but waits out its lag.** The flight itself refuses a part parted on the
  way up, before apogee and rising, whose device fired by the split but opens after its lag: the
  part would climb through the lag with no drag. A `.ork` never flies one, since the check above
  refuses its delay first; a Monte Carlo run's drawn deployment lag can make one, and that flight
  fails by name and is counted ([Monte Carlo](../monte-carlo.md)). A device set for the part's own
  apogee leaves it to coast as before, which the check above refuses for the payload. A part
  parted by an ejection charge on the way up is refused the same way. A part already falling
  when it parts still falls with no drag through its device's lag: a smaller error, not refused,
  but the flight warns of [#354](https://github.com/nrdptel/fusionspace-eridanus/issues/354), as does any
  flight with a part that starts with nothing open (`hpr mc` reports only its nominal flight's
  warnings, so a drawn lag's isn't).
- **The booster** tumbles side-on from the split until its own device opens, as after a powered
  split, and its flight is not validated
  ([#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179)). Every flight that drops a booster
  warns of #179 ([the separated parts' warnings](../VALIDATION.md#the-separated-parts-warnings)).
- **Not measured:** a real payload whose parachute opens late would land later and further
  away; OpenRocket's record holds no such flight to compare with.
- **The flight's apogee** is the payload's own when the split comes before the stack's apogee.
  A dropped part can climb higher: the C6-3 example's booster, tumbling, peaks 0.9 m above the
  payload, and `hpr sim` says so in a note, as a waiver's height is the highest any part
  reaches.

**A worked example.** OpenRocket's *Deployable payload* on a C6-3 drops its booster at the
charge, 4.86 s after launch. In HPR Sim's flight, from
`hpr sim "Deployable payload.ork" --config "[C6-3]"`, the stack is then
230.4 m up and still rising at 27.6 m/s (OpenRocket's record: 27.9 m/s). The payload's 10 in
(0.254 m) parachute, set to `lowerstageseparation` in the file, opens at that instant, and the
payload climbs 3.3 m more, to 233.7 m above the site at 5.44 s. That height is the center of
mass's, which starts 0.4 m above the site; from where it starts, as the table below compares, it
is 233.3 m. Coasting with no drag, it would have risen 27.6² / (2 × 9.81) = 38.8 m more
instead.

**Against OpenRocket.** All six configurations of the *Deployable payload* and the *ARC payload
rocket* separate at OpenRocket's times. The payload's apogee is within 1.1% of OpenRocket's
record, measured from where each tool's flight starts, and every descent meets the targets of
[Recovery](recovery.md#from-an-openrocket-file). Two of the six separate before apogee:

Each Δ is taken from the report's unrounded heights, so it can differ in the last digit from
the rounded ones shown.

| motors | split (s) | payload's apogee, OpenRocket / HPR Sim (m) | Δ | coasting with no drag: OpenRocket's free payload / HPR Sim (m) | Δ |
|---|---:|---|---:|---|---:|
| C6-3 | 4.86 | 233.2 / 233.3 | +0.04% | 258.5 / 268.8 | +4.00% |
| C6-5 | 6.86 | 265.3 / 264.4 | −0.32% | 266.6 / 265.4 | −0.43% |

The last two columns measure what the refusal avoids. OpenRocket's flight of the same rocket with
nothing deployed lets the payload fly on its own airframe. A coast with no drag puts the C6-3
payload 10.3 m above that, and 3.5 m above the whole stack's own climb. The
[report's table](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#separations-with-nothing-left-to-burn)
has all six. The [M4.5g3 milestone](../decisions-and-roadmap.md#m4-5g3) shipped this; its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md) has the reasoning.

## Boosters beside the core

**Boosters strapped beside the core, a [parallel stage](design.md#parallel-stages), burn with it
and drop at a separation of their own; from a `.ork`, on a rocket of one stage on the axis.** No
new physics is involved. The stack flies whole until
the split, and the split is a powered separation like any other
([Powered separation](#powered-separation)): the core flies on with its own shape and mass, and
the boosters fly as a point with only their devices' drag, tumbling side-on from the split, as a
dropped booster does in the simulator. Their own flight after the split is not validated: their peak
and landing are likely too low and too close to the pad
([#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179)).

- **Which separation drops them.** A separation after stage *k* drops every stage after *k*, so
  a parallel stage is dropped by the separation after the stage it hangs on, together with
  whatever hangs on it. A separation that would drop boosters hung on a stage the nose keeps
  together with an axial booster behind them is refused; drop the axial booster first.
- **When they light.** In a `.ork` file, a booster's motor set to light `automatic`ally lights at
  launch when the boosters hang on the last stage on the axis, as OpenRocket lights them
  ([`.ork`: Delays and ignition](../format/ork.md#delays-and-ignition)).
- **Boosters with no motor stay on.** When no motor in the boosters lights, their separation at
  burnout or at the charge never comes, and they ride to apogee and land with the core, as
  OpenRocket flies them.

**A worked example.** OpenRocket's *Parallel booster staging*, one of the example designs
OpenRocket ships, has an I115W in its core and an E12-0 in each of two boosters. The file holds
no thrust curves, so `hpr sim` fetches them from ThrustCurve.org once
([Motors from ThrustCurve.org](../cli.md#motors-from-thrustcurveorg)). In configuration 1 both light at launch. From
`hpr sim "Parallel booster staging.ork" --config 1`, with its defaults (a 1.5 m rail at sea
level, calm air), the E12s burn out and fire their charges at 2.44 s, 290.1 m up at 207.6 m/s,
and the boosters drop away. The core burns on to 3.51 s and peaks at 1138.2 m at 13.01 s; the
boosters land at 33.0 s. In configuration 2 the boosters hold no motor and stay on: the stack
peaks at 858.2 m. These heights are a little above the table's below, which flies each
configuration under OpenRocket's stored conditions instead (a 1 m rod, at its launch site).

**Against OpenRocket.** Under OpenRocket's own stored conditions, from the
[flights report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md),
recovery as saved:

| motors | split (s), OpenRocket / HPR Sim | apogee, OpenRocket / HPR Sim (m) | Δ | largest speed Δ |
|---|---|---|---:|---:|
| I115W-10 and 2× E12-0 | 2.44 / 2.44 | 1123.6 / 1137.5 | +1.23% | +3.25% |
| I115W-10, boosters empty | none / none | 851.8 / 857.6 | +0.68% | +1.56% |

At rod clearance the masses agree to 0.7 g (0.06%) and the static margins to 0.003 calibre,
and both descents meet the targets of [Recovery](recovery.md#from-an-openrocket-file). The 3.25% in largest speed is not
traced. [M4.5m](../decisions-and-roadmap.md#m4-5m) shipped this; its decision record is
[ADR-171](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0171-a-parallel-stage-is-a-pod-set-that-separates.md).

## A booster dropped still burning

**A powered separation may drop a booster whose motors still burn, when it is told to. The
dropped part then flies with no thrust, so its flight leaves out the impulse those motors had
left.** This is how the simulator flies a `.ork` stage with motors in more than one mount.
OpenRocket 24.12 separates such a stage at its first motor's burnout, measured by a committed probe
([Delays and ignition](../format/ork.md#delays-and-ignition)), and drops the stage's other motors
still burning ([M4.5n](../decisions-and-roadmap.md#m4-5n), a stage's first burnout;
[ADR-172][adr-172]).

- **The opt-in.** `hpr_sim::Separation::drops_burning`, set by `Separation::dropping_burning()`,
  is off by default, and then the booster's motors must have burned out. On, a powered separation
  may drop a motor that has lit and still burns. A motor behind it still to light is refused
  either way, and so is any burning motor behind an unpowered separation. `hpr::ork::separation`
  sets it only for a staging the `.ork` reader marked `drops_burning`. The reader marks only a
  split at its own stage's first `burnout`, and only when the split is powered: a motor ahead of
  it burns then, or lights then or later. A delay after that burnout counts too, though no probe
  has measured a delayed one. Such a split may drop only the stage's own other
  motors, and only those lit strictly before the split. A motor behind it lit at the split
  itself, or one burning in another stage behind, is refused. Any other split while a motor
  behind it burns (a time, an ignition, a charge) is still refused, since leaving its thrust out
  could leave out most of a motor. A motor lit by a charge among several mounts is refused by
  name: which fires its charge first is not measured.
- **What the flight leaves out.** A dropped part flies as a point with no thrust, as every
  dropped part does, with its mass at the split, propellant left included.
  `BodyFlight::impulse_left_n_s` gives the impulse its motors had left: each lit motor's curve
  total, less what it had given by the split. That thrust could have changed the part's speed by
  about that impulse over its mass, a little more as the propellant left burns away. It is 0 for
  every other part.
- **What `hpr sim` says.** It names the motors and gives the number. On the first configuration of
  OpenRocket's *Pods--powered with recovery deployment*, `[C6-7; 2× A3-4, B6-0]`, the booster
  drops at 0.860 s with both A3s still burning. Its flight as a point leaves out 0.509 N·s, as
  the report's
  [*Parts dropped still burning*](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#parts-dropped-still-burning)
  section gives it.

The sustainer's flight is not affected: it keeps no thrust from the dropped motors in either
program. The dropped booster's own flight is not validated
([#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179)).

## Against OpenRocket

**In short.** HPR Sim reads when each motor lights and when the stages come apart from an OpenRocket
`.ork` file, and flies OpenRocket's own two-stage, three-stage, cluster and air-start examples.
Every one of their 15 flights is within 5% of OpenRocket's in apogee and in largest speed, a
tolerance written down before any of them was flown
([M1.9c milestone](../decisions-and-roadmap.md#m1-9c), decision record [ADR-076][adr-076]; the
three-stage example since [M4.5g2](../decisions-and-roadmap.md#m4-5g2), several separations).
Five apogees, three of the cluster's and two of the three-stage example's, are compared with
OpenRocket's flight with no parachute, since its parachute opened before apogee.
This is agreement with another program, not with a real flight.

**What HPR Sim reads.** A `.ork` file says when each motor lights with a word and a delay
([Delays and ignition](../format/ork.md#delays-and-ignition)), and when each stage drops away with
another ([When parachutes open and stages separate](../format/ork.md#when-parachutes-open-and-stages-separate)).
HPR Sim turns them into its own:

| the file says | HPR Sim flies |
|---|---|
| a motor lit at `launch`, or `automatic` in the bottom stage, plus a delay `d` | lit at `t = d`: an air start when `d` is not 0 |
| a motor lit at `burnout` of the stage below, plus `d` | lit `d` after the stage below's first burnout: the first of its motors to burn out, by each one's ignition and curve, in whichever mount ([ADR-172][adr-172]) |
| `ejectioncharge`, or `automatic` in a stage above | lit at the stage below's ejection charge: its burnout plus its ejection delay, plus `d` |
| `never`; `burnout` or `ejectioncharge` in the bottom stage; `ejectioncharge` or `automatic` over a plugged motor; or a motor lit by one of these | never lit: carried loaded, with no thrust |
| a stage separating at `launch` plus a delay, at its motor's `ignition`, `burnout` or `ejection`, or at the `upperignition` of the stage above | a separation at that instant, plus its delay, if at that moment a motor ahead of it is burning or has yet to light (one timed before the rocket leaves the rod fires as it leaves, [#231](https://github.com/nrdptel/fusionspace-eridanus/issues/231)) |
| a stage separating at `apogee`, or at a height on the way down (`altitudedescending`) | no separation: it belongs to the descent, which HPR Sim's flights of a `.ork` don't fly yet, so the configuration flies whole |

HPR Sim flies such a separation when it is powered: at its time a motor ahead of it is burning or has
yet to light, and no motor behind it is. A powered separation at its own stage's first `burnout`
may also drop the stage's other motors still burning, as OpenRocket does
([A booster dropped still burning](#a-booster-dropped-still-burning)). A configuration with several flies them all, from the
tail forward ([Several separations](#several-separations)), as long as each is powered. An `apogee` or `altitudedescending` separation is left to
the descent only on the assumption that every motor is spent by apogee; `cargo xtask ork-flights`
checks that of every flight it reports. When flown, HPR Sim refuses a powered separation that fires
after a recovery device on the part with the nose has opened. A configuration it can't fly as
written is left out with its reason, never flown some other way:

- a separation at or after apogee beside another separation, and one that comes before the
  separation behind it ([Several separations](#several-separations));
- a separation with no motor ahead of it burning or yet to light when it fires, beside another
  separation. Alone, at a time known before the flight, it flies
  ([A payload dropped with nothing left to burn](#a-payload-dropped-with-nothing-left-to-burn)),
  unless the part that keeps the nose would coast with no drag from it. A lone one at `apogee` or
  at a height on the way down can only come after apogee, so the climb is the whole stack's, as
  in OpenRocket;
- a separation while a motor behind it is still burning or yet to light, but for a powered one at
  its own stage's first `burnout`, which may drop the stage's own other motors still burning if
  they lit strictly before the split (the reader marks it `drops_burning`);
- a separation at a height on the way up (`altitudeascending`), which HPR Sim has no trigger for;
- a negative delay on a separation HPR Sim would fly;
- a motor lit at a word HPR Sim does not know, by a stage below that holds no motor, by the charge of
  a stage below with motors in more than one mount (which fires its charge first is not
  measured), or by the charge of a motor below that states no delay;
- a separation at the ignition of a motor that never lights, or at launch, since HPR Sim's flight
  fires a separation only once the rocket is off the rod;
- a configuration in which no motor lights.

A motor set `never`, or lit by an event that never comes, flies unlit and loaded, as OpenRocket
flies it ([M2.2e10](../decisions-and-roadmap.md#m2-2e10), probed by
`validation/oracles/openrocket/unlit_motors.py`): one in the bottom stage lit at the stage below's
burnout or charge, or one lit by a plugged motor's charge. A stage that separates at the burnout or
charge of its own motor that never lights stays on. A stage below with no motor is still refused:
OpenRocket's reading of it has not been probed.

A cluster flies with a motor in every tube ([Clusters](design.md#clusters)).

**The tolerance.** A design is within when every configuration of it that HPR Sim flies has its
apogee and its largest speed within 5% of OpenRocket's. Where OpenRocket's parachute opened before
its apogee, which lowers it, the apogee is compared with OpenRocket's flight of the same
configuration with nothing deployed. OpenRocket's *record* is the set of flights OpenRocket 24.12
flew for this comparison, committed as
[`openrocket-flights.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/openrocket-flights.json).
It holds the flight of the part that keeps the nose, so both numbers are the sustainer's.

**The result.** Δ is HPR Sim less OpenRocket, in per cent of OpenRocket's.

| OpenRocket's example | motors | apogee, OpenRocket (m) | Δ apogee | Δ largest speed | notes |
|---|---|---:|---:|---:|---|
| Two stage high power rocket | H148R-0, then H148R-0 | 678.5 | −1.79% | −0.54% | separates at 1.535 s in both |
| Two stage high power rocket | I357T-14, then I59WN-P | 1,384.2 | −0.13% | −0.04% | separates at 1.515 s in both |
| Three stage low power rocket | A8-5, B6-0, B6-0 | 277.1 | −1.37% | +0.25% | nothing deployed; separates at 0.857 s and 1.714 s in both |
| Three stage low power rocket | C6-5, B6-0, B6-0 | 505.4 | −0.95% | +1.51% | nothing deployed; separates at 0.857 s and 1.714 s in both |
| Three stage low power rocket | C6-7, C6-0, C6-0 | 689.5 | −1.48% | +1.64% | separates at 1.860 s and 3.720 s in both |
| Clustered motors | 4× A8-3 | 58.2 | −0.35% | +0.20% | |
| Clustered motors | 4× B4-4 | 143.1 | −0.79% | +0.31% | nothing deployed |
| Clustered motors | 4× C6-3 | 308.6 | −0.72% | +0.92% | nothing deployed; +9.43% against its parachute opening 2.59 s early |
| Clustered motors | 4× C6-5 | 308.6 | −0.72% | +0.92% | nothing deployed |
| Clustered motors | 4× C6-7 | 308.6 | −0.72% | +0.92% | |
| Airstart timing | 3× I211W-P and a K550W-P, all at launch | 1,317.5 | +0.94% | +0.81% | |
| Airstart timing | the three I211W lit at 1 s | 1,292.5 | +1.03% | +0.76% | |
| Airstart timing | at 2 s | 1,296.0 | +0.84% | +0.72% | |
| Airstart timing | at 4 s | 1,303.7 | +0.59% | +0.51% | |
| Airstart timing | at 6 s | 1,274.2 | +0.48% | +0.46% | |

The full report, with the stability margin, mass and center of mass at rod clearance, is
[`openrocket-flights.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md).
The test `a_two_stage_and_a_cluster_design_are_within_5_percent_of_openrocket` in
`xtask/src/ork_flights.rs` holds it to the tolerance in CI.

**A flight OpenRocket aborted is checked up to the abort.** OpenRocket stops its own flight of
*Pods--powered with recovery deployment*'s first configuration at 1.81 s ("Stage began to tumble
under thrust"), so it has no apogee to compare. Instead, `cargo xtask ork-flights` compares HPR Sim's
height and speed with OpenRocket's just after the separation and at OpenRocket's last row. It
needs the splits at one time and each value within 5%. Both separate at 0.86 s, where HPR Sim is
1.17% lower (22.18 m against 22.44 m) and 0.04% slower. At 1.81 s HPR Sim is 1.18% lower (84.0 m
against 85.0 m) and 4.02% faster (79.9 m/s against 76.8 m/s), so it is met
([the report's section](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#flights-openrocket-aborted);
[`.ork`: Flights OpenRocket aborted](../format/ork.md#flights-openrocket-aborted)). This is
weaker evidence than an apogee: two points on the way up, the second where OpenRocket's flight is
breaking up. When OpenRocket aborts is its own call, too: its probe of the same configuration
with nothing deployed aborts at 2.40 s, not 1.81 s. The sustainer's least static margin is −4.21
calibres at the split. In the record's conditions, a 0.15 m rod at 28.61° N with no wind, HPR Sim's
sustainer turns over at 2.04 s. From `hpr sim`'s defaults, a 1.5 m rail with no wind, it does
not, and why the two differ is not traced. `hpr sim` warns that the sustainer is unstable under
power and that its apogee is not a prediction
([Flight metrics: unstable under power](metrics.md#unstable-under-power);
[#335](https://github.com/nrdptel/fusionspace-eridanus/issues/335)).

**What is not settled.**

- On the first two-stage flight, OpenRocket's mass falls about twice as fast as HPR Sim's before the
  sustainer lights: by 0.0823 kg against 0.0412 kg from launch to the end of the rod, as if both
  H148R motors lost propellant from launch. On the second, with two different motors, the masses
  agree to about 1 part in a million. The three-stage example shows the same fault: OpenRocket's
  drop to the end of the rod is 2.0000, 1.9998 and 2.9944 times HPR Sim's, the count of motors of the
  booster's type ([Several separations](#several-separations)). What that does to the apogee has
  not been sized ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)).
- HPR Sim's apogee is its center of mass's, which jumps forward at the split from the whole stack's
  to the sustainer's. Whether OpenRocket's altitude jumps the same way is not measured
  ([#187](https://github.com/nrdptel/fusionspace-eridanus/issues/187)). The jump is under 1.32 m on
  the two-stage flights, less than 0.2% of either apogee.
- HPR Sim's descent of a separated part needs a recovery device on each part. So the climb's
  comparison tumbles the booster from the split and the sustainer from its apogee; neither acts on
  the climb, which flies no parachute. A second flight, for the
  [descent](recovery.md#from-an-openrocket-file), flies the file's own devices, each on its part
  ([ADR-159][adr-159], powered separation in `hpr sim`). The sustainer's landing speed is +0.00%
  off OpenRocket's on both configurations, and its flight time +0.32% on the H148R-0 and H148R-0
  configuration and +0.73% on the I357T-14, then I59WN-P one. On the three-stage example it lands
  within 0.009% of OpenRocket's speed, and its flight time is −1.13%, −0.56% and −1.41% off. The
  descents of the booster and the middle stage are not compared: OpenRocket's record keeps only
  the sustainer's branch.

**Running it.** `cargo xtask ork-flights` flies them. It needs OpenRocket's jar and its motor
database, which `cargo xtask refs fetch` and the oracle scripts put in place, so CI checks the
committed report rather than flying it.

**Flying a `.ork` file's staging yourself.** The rocket a `.ork` configuration builds does not
carry its separations. The reader hands them over separately, as the configuration's `staging`
and `later_stagings` (`MotorConfiguration::stagings` gives them all, from the tail forward).
`hpr::ork::separations` turns them into the flight's separations, which the program passes to the
flight with `Simulation::with_separations`, or the facade's
[`FlightBuilder::separations`](../api/hpr/flight/struct.FlightBuilder.html#method.separations), along
with a recovery device on each part. For a configuration with one, `hpr::ork::separation`,
`Simulation::with_separation` and `FlightBuilder::separation` still take it alone. The example
[`ork_two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs)
does this for a made-up two-stage `.ork`: it tumbles the booster from the split and the
sustainer from its apogee. Run it with `cargo run --example ork_two_stage -p fusionspace-hpr`. For the
file's own parachutes and streamers instead,
[`hpr::ork::separated_recovery`](../api/hpr/ork/fn.separated_recovery.html) puts each on its part
and adds the tumbles a part needs; `hpr sim` flies a `.ork` that way
([Separation](../cli.md#separation)).

## Tests

In `crates/hpr-sim/src/staging.rs` and `crates/hpr-design/src/config.rs`:

- `serial_plan_timing_and_mass_step` ([Loft lesson L93](../decisions-and-roadmap.md#l93), a check
  carried over from this project's predecessor): the sustainer lights at the booster's burnout plus
  its delay, to 1e-12 s, and the mass steps down by exactly the booster's, to 1e-12 of the mass, and
  holds until the sustainer lights. It also checks a sustainer whose separation never comes: it
  never lights, and it lands loaded.
- `a_sustainer_set_never_to_light_flies_as_a_motor_out` and
  `a_motor_set_never_to_light_stays_loaded`: a motor set `never` flies exactly as one whose only
  tube fails, carried loaded to the end, and a motor lit by that motor's burnout never lights
  either. `a_sustainer_keeps_a_motor_set_never_to_light_through_the_separation`: a sustainer cut at
  a powered separation keeps it unlit.
- In `crates/hpr-io/src/ork/tests.rs`, `openrocket_flies_a_motor_whose_ignition_never_comes_unlit`
  holds OpenRocket's probe record, and `a_motor_waiting_on_one_that_never_lights_is_never_lit` the
  reader's chains and refusals.
- `apogee_separation_fires_in_flight_and_booster_flies_to_landing`
  ([Loft lesson L30](../decisions-and-roadmap.md#l30)): an apogee separation fires at the
  flight's own apogee, and a height separation where the flight crosses the height. Both bodies
  land, powered or not.
- `staged_events_come_in_order`: liftoff, rail exit, separation, ignition, burnout, apogee and
  landing, in that order, with the burnout recorded once at the sustainer's, or twice with a
  sustainer lit by its separation.
- `a_powered_separation_conserves_linear_momentum`: the two bodies' momenta add to the stack's,
  to 1e-9 of it, launched 10° off vertical.
- `an_air_start_lights_at_its_time_and_burns_on_its_own_clock`: a motor lit at 3 s, loaded until
  then, burns out at 3 s plus its burn time.
- `staging_refuses_what_it_cannot_fly`: a drag table or a drag model past a powered separation, a
  separation timed while the booster burns, a delay that is negative or not a number, a booster
  with nothing open at the split, a separation that could never fire, and a powered separation
  after the sustainer's canopy opened.
- `two_powered_separations_drop_the_booster_then_the_middle`: two splits at the booster's and the
  middle stage's burnouts, to 1e-9 s; at each the mass steps by exactly the dropped part's, to
  1e-12 of the mass, and the momenta add to the stack's, to 1e-9 of it. The booster is body 1,
  the middle stage body 2, both land, and the flight repeats to the bit.
  `a_motor_lit_by_the_first_separation_counts_from_it_at_the_second`: a middle stage lit at the
  first split leaves at the second with its motor's dry mass, and a second split timed while that
  motor burns is refused. `several_separations_refuse_what_they_cannot_fly`: the refusals in
  [Several separations](#several-separations).
- `ignition_times_follow_their_events`, `bad_ignitions_are_refused`.
- `strap_on_boosters_burn_beside_the_sustainer_and_drop`: boosters lit at launch drop at their
  separation; the mass steps down by exactly their structure and spent motors, the core burns
  on, and it climbs higher than with the boosters carried to apogee.
  `a_separation_drops_only_what_hangs_together`: the refusal above, and the order that flies.
- `a_powered_separation_drops_a_burning_booster_only_when_told`: a powered separation set
  `dropping_burning` drops a booster still burning, and the booster's `impulse_left_n_s` is its
  curve's total less what it had given by the split, to 1e-12 of it; not set, unpowered, or with
  a motor behind still to light, it is refused. In `crates/hpr-io/src/ork/tests.rs`,
  `a_stage_in_several_mounts_burns_out_first_where_its_motors_say` holds the reader's first
  burnout and its refusals.

[adr-074]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0074-ignition-times-and-powered-staging-the-sustainer.md
[adr-076]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0076-a-ork-files-ignitions-and-one-powered-separation.md
[adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md
[adr-159]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0159-a-powered-separation-in-hpr-sim.md
[adr-160]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0160-several-powered-separations.md
[ignition]: ../api/hpr_design/config/enum.Ignition.html
[bodies]: ../api/hpr_sim/flight/struct.FlightResult.html#structfield.bodies
[landings]: ../api/hpr_sim/flight/struct.FlightResult.html#method.landings
