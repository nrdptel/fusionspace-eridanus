# How a flight is simulated

See what lies behind each number HPR Sim prints, stage by stage from ignition to landing, and which
model computes it.

This page uses plain words. Read it before the model pages it links.

> **How far to trust it.** This page describes the method, not how well it works; every number it
> shows is an estimate from the models it names. When both codes fly the same drag, whole flights
> match RocketPy's in height, speed and time. They match in where they go too, except for rockets
> that leave the rail slowly in a wind ([Accuracy](accuracy.md#whole-flights-against-rocketpy)).
> With HPR Sim's own drag, against RocketPy flying the drag its examples ship, heights differ by
> −7.280% to +10.302%
> ([report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md);
> [Accuracy](accuracy.md#whole-flights-with-each-codes-own-drag)). [Accuracy](accuracy.md) keeps
> every result so far; for a real flight, your altimeter's log and your range safety officer have
> the last word.

![A rocket's flight seen from the side. It lifts off the pad, leaves the rail, burns out, and coasts to apogee west of the pad, into the wind. It then drifts east under a drogue and a main parachute, and lands east of the pad. Seven numbered points mark the events: liftoff, rail exit, burnout, apogee, the drogue opening, the main opening, and landing.](images/flight-phases.svg)

The drawing is the shape of the [Getting started](getting-started.md) example's flight, not to
scale. Its apogee is 779 m (2,556 ft) up and 88.7 m (291 ft) west of the pad. It lands
90.2 m (296 ft) east of the pad.

## What goes in

Building a simulation gathers five inputs, and works out once everything that doesn't change
during the flight.

| input | what the simulator takes from it | pages |
|---|---|---|
| the rocket | A tree of parts, such as a nose cone, body tubes and a fin set, with their positions. Their shapes and materials give each part's mass, [center of gravity](glossary.md#center-of-gravity-cg) and inertia. The design's checks run first, and a rocket that can't exist, such as one with a motor wider than its mount, is refused | [Design tree](physics/design.md), [Shapes](physics/shapes.md), [Mass properties](physics/mass.md) |
| the motor | The thrust at every instant, from its thrust curve. Its mass, center of gravity and inertia as its propellant burns away, so the whole rocket gets lighter, and its center of gravity moves, during the burn | [Solid motors](physics/motor.md) |
| the aerodynamics | Built from the rocket's shape and surface: the [normal force](glossary.md#normal-force) on each body part and fin set, the [center of pressure](glossary.md#center-of-pressure-cp) where it acts, and the drag. They depend on the [Mach number](glossary.md#mach-number), the [angle of attack](glossary.md#angle-of-attack) and the [Reynolds number](glossary.md#reynolds-number) | [Aerodynamics](physics/aero.md) |
| the surroundings | The launch site on the [WGS 84](glossary.md#wgs-84) model of the Earth's shape, and the [launch frame](glossary.md#launch-frame-enu): east, north and up from the pad. Gravity that changes with latitude and height, and the [Coriolis acceleration](glossary.md#coriolis-acceleration), a small sideways push that anything moving over the rotating Earth appears to feel. The air's density, pressure, temperature and speed of sound, and the wind, at each height. A turbulence model exists, but no flight uses it yet | [Geodesy](physics/geodesy.md), [Frames](physics/frames.md), [Gravity](physics/gravity.md), [Atmosphere](physics/atmosphere.md), [Wind](physics/wind.md), [Turbulence](physics/turbulence.md) |
| the rail, recovery and settings | The rail's length, direction and friction; each parachute or streamer and when it fires; how finely to step through time | [Rigid-body flight](physics/flight.md), [Recovery](physics/recovery.md), [Time integration](physics/integration.md) |

## From the pad to the ground

The numbers match the drawing.

1. **Ignition and liftoff.** Every motor lights at time zero unless its design gives it a later
   ignition: an [air start](glossary.md#air-start), or a sustainer lit after its booster
   ([Staging](physics/staging.md)). A two-stage design that says nothing lights both stages on the
   pad.

   The rocket stands on the rail, its aft end at the rail's foot. It holds still until the push up
   the rail, mostly the thrust, beats the weight's pull down it and the rail's friction. That
   instant is [liftoff](glossary.md#liftoff).
2. **On the rail.** The rocket slides along the rail without turning: one degree of freedom.
   [Rail exit](glossary.md#rail-exit-and-rail-exit-velocity) comes when the rocket's last rail
   guide leaves the top of the rail. That guide is a rail button or a launch lug (a short tube on
   the body). A rocket that stops on the rail comes to rest there
   ([Rigid-body flight](physics/flight.md#phases)).
3. **Powered flight, to burnout.** Off the rail, the rocket is a rigid body free to move and turn
   in every direction. Those are the six degrees of freedom of a
   [6-DOF](glossary.md#6-dof-six-degrees-of-freedom) simulator. At every instant HPR Sim adds up the
   forces and their turning effects:
   - the thrust of each burning motor, along the rocket's axis. So a
     [cluster](glossary.md#cluster) of motors that all light at once is flown, with their thrusts
     added. A motor off the center line adds a turning effect, as when one motor of a cluster
     fails to light ([Clusters](physics/design.md#clusters)). Tests check it;
   - the weight and the [Coriolis](glossary.md#coriolis-acceleration) force, at the center of
     gravity;
   - the air's forces, from the air's velocity past the rocket, wind included. They're the drag
     along the axis, and each body part's and fin set's normal force at its own center of
     pressure;
   - the burning propellant's effects: the center of gravity moving inside the rocket, and jet
     damping (the exhaust carrying away some of any turning motion). They include the
     propellant's [internal momentum](glossary.md#internal-momentum) too, which is counted twice
     (see [What is left out](#what-is-left-out)).

   From these it works out how the rocket speeds up and turns
   ([Rigid-body flight](physics/flight.md)). A crosswind meets the rocket partly from the side. The
   fins' normal force, behind the center of gravity, swings the nose into the wind, so the rocket
   climbs upwind ([weathercocking](glossary.md#weathercocking)). The top speed usually comes just
   before [burnout](glossary.md#burnout), once the thrust no longer beats the drag and the weight.
   Burnout is the end of the last motor's thrust curve.
4. **Coast, to apogee.** The same equations with no thrust: drag and gravity slow the climb.
   [Apogee](glossary.md#apogee) is where the vertical speed falls through zero.
5. **The drogue.** Each recovery device fires its charge at its trigger: apogee, a height on the
   way down, a time, or a motor's [ejection delay](glossary.md#ejection-delay). After its lag, a
   set time from the charge to its lines stretching, it [deploys](glossary.md#deployment). From
   the first deployment the rocket is a single point with mass. Its attitude (which way it
   points) freezes. It falls under gravity and the open devices' drag while the wind carries it
   along, which is its [drift](glossary.md#drift).
6. **The main.** A second device, typically the main set to a height above the ground, adds its
   drag to the drogue's. A device can also cut another away as it opens. A rocket can
   [separate](glossary.md#separation) into parts that each come down on their own.
   [Streamers](glossary.md#streamer) and [tumbling](glossary.md#tumble-recovery) are recovery
   devices too ([Recovery](physics/recovery.md)).
7. **Landing.** The flight ends when the center of gravity comes back down to the launch site's
   height.

A flight can also end in other ways, each reported by name
([Rigid-body flight](physics/flight.md#events-and-termination)):

| ending | what happened |
|---|---|
| on the pad | the motors burnt out before the rocket lifted off |
| stalled on the rail | it lifted off, then stopped on the rail after the motors burnt out |
| time cap | the flight reached its time limit, 3,600 s (an hour) after ignition by default, before landing |
| step limit | the integrator (below) used up its budget of steps, a million attempted steps by default, before landing |
| separated | the rocket split into parts, and each part's own descent says where it landed |

The time cap and the step limit are safety stops, so a flight that doesn't land still ends.
A [stiff](glossary.md#stiff-problem) stretch of flight, one that forces very short steps, can show
up as the step limit ([Time integration](physics/integration.md#defaults-and-limits)). Both limits
are fields of `FlightSettings` (`max_time_s` and `step_limit`), and a program can change them.

Anything else that stops a flight is an error, such as reaching Mach 5. Mach 5 is the top speed
the simulator's normal force and drag cover.

## How the simulator steps through time

The rocket's state at any instant is 13 numbers. Three give where it is, and three how fast it
moves. Four give which way it points, as a quaternion, a compact way to store a rotation. The last
three give how fast it turns.

The equations above turn a state into its rate of change. An integrator builds the flight from
them by stepping forward in time, one short step after another
([Time integration](physics/integration.md)).

The simulator's default method is [Dormand-Prince 5(4)](glossary.md#dormand-prince-and-rk4). On
each step it makes two estimates of the new state, one of fifth order and one of fourth; that's the
"5(4)". The higher a method's order, the faster its error shrinks as the step gets shorter. HPR Sim
keeps the fifth-order estimate, and takes the difference between the two as the step's error.

HPR Sim sizes the next step to keep that error within a [tolerance](glossary.md#tolerance). Steps
are short where things change fast, at liftoff and burnout, and long in a steady descent
([adaptive time step](glossary.md#adaptive-time-step)). At the default settings, the Getting
started flight's apogee is within about a micrometer of the answer at much tighter settings
([Rigid-body flight](physics/flight.md#integration-settings)). At those default settings a whole
flight takes about a millisecond of computing
([the same measurement](physics/flight.md#integration-settings)).

Some moments where a force changes abruptly are known in advance, such as each point of the thrust
curve and burnout. These are [stop times](glossary.md#stop-time). A step always ends exactly
there, so none straddles a jump.

Other moments are found during the flight, such as liftoff, rail exit, apogee, an altitude trigger
and landing. These are [events](glossary.md#event). When the quantity that defines one changes
sign within a step, the simulator finds the instant it crossed zero inside that step.

You get back every event, with a snapshot of the flight at that instant. The snapshot holds the
time, position, velocity, height, airspeed, Mach number, angle of attack, thrust, mass and more. A
program can also watch every step as it happens, as
[Getting started](getting-started.md#the-program-step-by-step) does to find the top speed. Or it
can record chosen quantities at a fixed interval with a `Recorder`
([Recording a trajectory](recording-a-trajectory.md)).

## What is left out

Each model page lists what its model leaves out. These gaps matter most for a whole flight:

- Near and past Mach 1, the drag is lightly checked. The normal force, center of pressure and
  drag all carry a flight from Mach 0 to 5. The normal force was checked against a wind tunnel to
  Mach 4.63 ([Aerodynamics](physics/aero.md#normal-force-through-mach-1)). The drag was checked at
  Mach 0.3 against other programs' curves, and from Mach 0.6 to 4.63 against the same wind tunnel.
  Against the tunnel it reads high at every Mach number it measured, most of all with fins past
  Mach 1
  ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)).

  Near and above the speed of sound the drag is Niskanen's semi-empirical method (formulas fitted
  to measurements). Against [Calisto](glossary.md#example-rockets)'s curve, the one real
  [RASAero II](glossary.md#rasaero-ii) export, the drag reads 10.2% low to 16.5% high from Mach
  0.85 to 1.15. From Mach 1.2 to 2.0 it reads 5.1% to 15.0% low, the most at Mach 2
  ([Aerodynamics](physics/aero.md#drag-against-rasaero-ii-through-mach-2);
  [the drag against RASAero II](decisions-and-roadmap.md#m1-8b2);
  [what backs it](evidence.md#g26)).
- The aerodynamics are for small angles of attack, with no [stall](glossary.md#stall). But a
  flight uses them at every angle: just off the rail in a strong crosswind, and near apogee.
- Staging, clusters and air starts are checked against OpenRocket on three of its examples. Each
  motor lights at its own time, and a sustainer flies on after a powered separation
  ([Staging](physics/staging.md)). The two-stage, cluster and air-start examples' 12 flights are
  each within 5% of OpenRocket's apogee and largest speed. Three cluster apogees are compared with
  OpenRocket's flight with no parachute, since its parachute opened before apogee
  ([a two-stage and a cluster design](decisions-and-roadmap.md#m1-9c)).

  A motor that fails to light in a cluster is checked by tests only
  ([Clusters](physics/design.md#clusters)). A rocket that separates more than once isn't compared
  yet. Neither is one that ejects a nose cone or a payload to land on its own. Those pieces are
  checked against exact answers only
  ([Recovery: ejected pieces](physics/recovery.md#ejected-pieces)).
- Moving and released mass are checked against exact answers only. That's ballast or a payload
  that slides along the airframe in flight, or leaves it. The exact answers are hand calculations
  and conservation laws, not another simulator or a real flight
  ([Moving mass](physics/moving-mass.md), [Released mass](physics/released-mass.md)).
- [Tip-off](glossary.md#tip-off), thrust misalignment (a motor pushing slightly off the rocket's
  axis) and turbulence, which no milestone plans yet. Roll from canted fins and roll damping are
  modeled, and checked against measurements only from Mach 1.5 up
  ([Roll: forcing and damping](physics/aero.md#roll-forcing-and-damping)).
- One term is counted twice. A thrust curve measured on a test stand already includes the
  propellant's [internal momentum](glossary.md#internal-momentum), and the equations of motion add
  it again, as RocketPy's do. HPR Sim keeps it so the two codes can be compared like for like.
  On the Getting started rocket it adds 21 N to the push at liftoff. It changes the burnout speed
  by at most 0.05 m/s (0.2 ft/s) ([Rigid-body flight](physics/flight.md#equations-of-motion)).
- Under a parachute, four things are left out. One is the drag overshoot as a canopy fills, so the
  opening load the simulator reports is no safe bound. By default a canopy opens at once. The
  others are the air carried along with the canopy ([added mass](glossary.md#added-mass)), the
  airframe's own drag, and the rocket swinging below the canopy ([Recovery](physics/recovery.md)).
- There's no terrain: the ground is flat, at the pad's height.

