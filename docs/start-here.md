# Start here

**FusionSpace HPR is a suite of open-source tools for hobby and high-power rockets. Its first
product, FusionSpace HPR · Sim (HPR Sim for short), is a flight
simulator, and it is not ready to rely on yet.** A flight analyzer, a competition kit and an app
are planned ([Decisions and the roadmap](decisions-and-roadmap.md)). This site explains what the
simulator models, where each model comes from, and how well each one has been checked. It is
written for a rocketeer who knows some physics and some code, but not this project. This page
covers what HPR Sim is, what works today, what doesn't yet, how far to trust it, and how to read
the other pages.

> [!NOTE]
> **Every number HPR Sim produces is an estimate from a model, not a measurement, and never a
> go/no-go verdict.** Your motor's printed data and your range safety officer are authoritative.

## What the simulator is

HPR Sim (called hpr-sim before release 0.1) simulates a rocket's flight from the launch rail to
landing. It is a
[6-DOF](glossary.md#6-dof-six-degrees-of-freedom) simulator: it follows all six degrees of freedom,
the rocket's position along three axes and its rotation about three. Meanwhile the motor burns, and
the rocket's mass, [center of gravity](glossary.md#center-of-gravity-cg) and inertia (how hard it is
to turn) change.

It is a Rust library first, meant to be built into other programs, as
[RocketPy](glossary.md#rocketpy) is. RocketPy is an open-source rocket flight simulator, written in
Python and used as a Python library. HPR Sim has a Python package too ([Python](python.md)), and a
graphical app is planned.
OpenRocket `.ork` design files are read today. The command-line tool, `hpr`, flies a `.ork` or a
rocket's JSON, looks up and converts motor files, searches vendors' motor stock and prices,
re-runs the validation, reads an altimeter's flight log, and fetches a launch day's weather
([The command line](cli.md)).

It is also built to be checked. Every model cites a published source, and tests pin every model.
The simulator is compared against RocketPy, [OpenRocket](https://openrocket.info/) and the logs
of real flights, with the results committed to the repository.

For now it covers only commercial off-the-shelf solid rocket motors
([COTS motors](glossary.md#cots-motor)), the kind you buy from a manufacturer. That stays true
until the first app.

Nothing has been released yet. Release 0.1.0's files are built and checked but not published;
until they are, you build it from source, and once they are, they install as
[Install a release](getting-started.md#install-a-release) says. What a release holds, and how each
of its files is checked, is in [How a release is built](releasing.md);
[the changelog](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md) says what 0.1.0 does,
how far to trust it, and its known gaps, the open issues that make a number look safer than it is
among them.
Releases come at product boundaries, and no dates are promised
([Phase 8, releases](decisions-and-roadmap.md#m10-1)); what each holds, and how likely each goal
is, is in [Plan](plan.md):

- 0.1, the simulator: the library, the command line and the Python package, with Monte Carlo
  dispersion from the command line and Python. It waits on
  [M2.3c1, the logged flights' apogees](decisions-and-roadmap.md#m2-3c1) and named fixes
  ([release 0.1](decisions-and-roadmap.md#m10-1)). It ships with its known limits listed, among
  them drag that reads high above Mach 0.8, so apogee there reads low.
- 0.2, the flight analyzer: the common altimeters' logs read on their own, with no design file, then
  compared with its simulation and the rocket's drag fitted from it.
- 0.3, accuracy and diagnosis: the simulator measured against logged flights and improved, and a
  flight's likely faults ranked from its log.
- 0.4, the competition kit: rules presets for Student Launch, IREC and others, an optimizer, the
  numbers a submission asks for, ejection-charge sizing, and RASAero and RocketPy files.
- 0.5, an app preview: open a design or a log, fly it and replay it in 3D, with the simulated
  flight drawn as a "ghost" beside the real one, on the desktop and in a web browser.
- 1.0, the app, with its design editor.

After 1.0, new product lines come one at a time, in an order that may still change: designing
your own solid motors, then parachutes, then a pad-day mode on your phone (the day's forecast,
the best launch hour, drift on the field, a flight card), then flight computers and trackers.
Hybrids and liquids come last
([the second-pass plan](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0163-the-second-pass-products-and-priorities.md)).

## What works today

These parts are built and tested. Each page gives its sources, and most say what they leave out.
The pages, the program's messages and the library's names use US spelling for the eight words
[the style guide](writing.md) checks. The saved forms that held a British-spelled key (a landing
ellipse, a design-check finding, a laid-out rocket and a few more) still read the old key and
write the new one, so an older build may not read what this one saves; design files never held
one and are unchanged ([M0.6e](decisions-and-roadmap.md#m0-6e), US names).

| part | what it does | pages |
|---|---|---|
| Earth | Gravity from [WGS 84](glossary.md#wgs-84) (the model of the Earth's shape and gravity that GPS uses), varying with latitude and height; the Earth's rotation; launch-site coordinates, the distance and bearing between two places, and a site's elevation looked up online or read from a GeoTIFF file | [Frames](physics/frames.md), [Geodesy](physics/geodesy.md), [Gravity](physics/gravity.md), [A launch site's elevation](elevation.md) |
| Air | The 1976 US [Standard Atmosphere](glossary.md#standard-atmosphere) up to 86 km (282,152 ft), with temperature offsets, humidity, and [soundings](glossary.md#sounding) (measured or forecast profiles of pressure, temperature and wind against height), including the weather of a real day from an [ERA5](glossary.md#era5) file, an Open-Meteo forecast, a weather balloon or NOAA's GFS and RAP forecasts | [Atmosphere](physics/atmosphere.md), [ERA5 weather files](format/era5.md), [Launch-day weather](weather.md), [Weather-balloon soundings](soundings.md), [NOAA forecasts: GFS and RAP](nomads.md) |
| Wind | Constant, layered, power-law and logarithmic wind profiles; [turbulence](glossary.md#turbulence-dryden) (random gusts) from a random-number generator started from a [seed](glossary.md#seed), a number you choose: the same seed gives exactly the same gusts every time on the same platform (operating system and processor) | [Wind](physics/wind.md), [Turbulence](physics/turbulence.md) |
| Motors | Reads `.eng` and `.rse` [thrust curves](glossary.md#thrust-curve); thrust, mass, center of gravity and inertia through the burn; [32 bundled motors](physics/motor.md#the-bundled-motors); a motor the catalog lacks, fetched from ThrustCurve.org by name and cached, so it flies offline after ([M4.5b](decisions-and-roadmap.md#m4-5b), motors on demand); motors in stock and their prices, from motor.fusionspace.co | [Solid motors](physics/motor.md), [`.eng` files](format/eng.md), [`.rse` files](format/rse.md), [Motor stock and prices](motor-stock.md) |
| Rocket | Nose cones, body tubes, transitions (tapered sections between tubes of different diameters), fins and other parts, their materials, the whole rocket's mass properties, and design checks. Everyday fits warn and only what can't be built is refused ([M4.5c](decisions-and-roadmap.md#m4-5c), design checks that match reality); a packed part, such as a payload or a parachute, drawn wider than its bore warns, as its width sets only its own inertia ([M4.5l](decisions-and-roadmap.md#m4-5l), a packed part wider than its bore); a part inside a nose cone or transition is checked against the room where it sits, and warns if it fits at the wide end but meets the wall where the cone narrows ([M10.1a](decisions-and-roadmap.md#m10-1a), the flight and design fixes) | [Design tree](physics/design.md), [Shapes](physics/shapes.md), [Mass properties](physics/mass.md) |
| Aerodynamics | The [center of pressure](glossary.md#center-of-pressure-cp) (where the aerodynamic force acts; its distance behind the center of gravity is the [stability margin](glossary.md#stability-margin)), the [normal force](glossary.md#normal-force) (the sideways force when the rocket flies at an angle to the airflow, its [angle of attack](glossary.md#angle-of-attack)) and drag. For small angles of attack: the normal force and the drag from [Mach](glossary.md#mach-number) 0 to 5, both checked against a wind tunnel from 0.6 to 4.63 (the drag reads high at most speeds). A part's or stage's own drag coefficient, stated in a `.ork` file, flies in place of its shape's drag as OpenRocket flies it, checked on 25 OpenRocket probe designs; OpenRocket's *Base drag hack* example, which relies on it, flies within 4.52% of OpenRocket's apogee on two motors and 7.80% high on the third, where HPR Sim gives its very blunt nose a value read between two measured heads and OpenRocket more, so the 5% bar is not met there ([M4.5h](decisions-and-roadmap.md#m4-5h), a part's drag override; [M4.5o](decisions-and-roadmap.md#m4-5o), a blunt nose's measured drag; [A part's stated drag coefficient](physics/aero.md#a-parts-stated-drag-coefficient)) | [Aerodynamics](physics/aero.md) |
| Flight | The launch rail, powered flight and coast to apogee, with an [adaptive time step](glossary.md#adaptive-time-step) and [events](glossary.md#event) such as burnout and apogee | [Rigid-body flight](physics/flight.md), [Time integration](physics/integration.md) |
| Recovery | Parachutes, [streamers](glossary.md#streamer) and [tumbling](glossary.md#tumble-recovery), the [drift](glossary.md#drift) they carry the rocket downwind, and a rocket that [separates](glossary.md#separation) into bodies that each descend on their own. A `.ork` file's parachutes and streamers fly as OpenRocket flies them, in `hpr sim` too, within 0.02% of OpenRocket's landing speed on 50 of its 53 example flights and within 0.12% on all 53 ([M4.5a](decisions-and-roadmap.md#m4-5a), a `.ork`'s recovery flown). `hpr sim` flies a `.ork`'s powered separation, each part under its own devices; the dropped booster's landing is rough ([M4.5g1](decisions-and-roadmap.md#m4-5g1), powered separation in `hpr sim`). Several powered separations fly in turn: OpenRocket's three-stage example flies as saved, all three configurations within 1.48% of OpenRocket's apogee and 1.64% of its largest speed on OpenRocket's own curves, and within 2.48% and 1.67% in `hpr sim` on the curves it fetches ([M4.5g2](decisions-and-roadmap.md#m4-5g2), several separations). A payload dropped with nothing left to burn flies when its own parachute opens at the split: OpenRocket's two payload examples fly, the payload's apogee within 1.1% of OpenRocket's ([M4.5g3](decisions-and-roadmap.md#m4-5g3), a payload's split) | [Recovery](physics/recovery.md#from-an-openrocket-file) |
| Design files | Opens an OpenRocket `.ork` file (zip, gzip or plain XML) and reads the whole design: the stages and body components, the tubes, rings, fins, lugs and recovery gear on and inside them, the motor configurations, when parachutes open, and the simulations OpenRocket stored. The airframe's shape is cross-checked against a second reader and OpenRocket itself (positions against OpenRocket alone; mass and center of gravity in [Mass properties](physics/mass.md#checked-against-openrocket)). Pods are read, weighed and flown, checked against OpenRocket on five small probe designs and on OpenRocket's *Pods--airframes and winglets* ([Pods](physics/aero.md#pods)), and motors in more than one pod set fly, as do rail buttons' screw heads, weighed: OpenRocket's *Pods--powered with recovery deployment* flies, 0.36% below OpenRocket's apogee with only the sustainer's motor and 1.86% below with only the booster's motors, and in the conditions of OpenRocket's record its staged flight turns over before apogee in both programs: its site is at 28.61° N, where the Earth's rotation tips HPR Sim's flight, and HPR Sim's angle of attack passes 90° at 2.25 s ([M4.5i](decisions-and-roadmap.md#m4-5i), *Pods--powered* flies); its first configuration, whose booster burns out first in its main tube and drops its pods still burning, flies too, checked against OpenRocket's flight at two points up to where OpenRocket aborts it, not at an apogee: within 5% in height and speed at the separation and at OpenRocket's last row, weaker evidence than an apogee ([M4.5n](decisions-and-roadmap.md#m4-5n), a stage's first burnout), a parallel stage (boosters strapped beside the core) is read, weighed and flown on a rocket of one stage on the axis, and dropped at its own separation (OpenRocket's *Parallel booster staging* flies both its configurations within 1.3% of OpenRocket's apogee; [M4.5m](decisions-and-roadmap.md#m4-5m), parallel stages), fins on a nose cone or a transition are read with their root along its surface, as OpenRocket draws it (OpenRocket's *Pods--airframes and winglets* flies, each apogee within 0.5% of OpenRocket's, its stability margin 0.07 calibres above OpenRocket's, the flattering side; [M4.5g4](decisions-and-roadmap.md#m4-5g4), fins on a nose cone), a part HPR Sim cannot shape honestly is left out with a warning, and a configuration flies only when every motor in it has a thrust curve and lights at a moment HPR Sim can fly: `hpr sim` flies 136 of the 170 motor configurations in the reference library and OpenRocket's examples as saved, 54 of OpenRocket's 56 among them, and 9 more with `--accept-design-errors`; 25 are refused, mostly for a motor with no curve ([How many configurations `hpr sim` flies](format/ork.md#how-many-configurations-hpr-sim-flies); [M4.5j](decisions-and-roadmap.md#m4-5j), the corpus count). A design is written back out as a `.ork` that reads back as the same design ([writing a `.ork`](format/ork.md#writing-a-ork-back-out)), and `hpr convert` or a Rust program can keep it as a `.hpr` file in the HPR design format, and write the `.ork` back from that. TypeScript and Python programs can read a `.hpr` with generated types ([The HPR design format](format/hpr.md)). OpenRocket's parts catalog, the 16 `.orc` files it ships with 3,449 makers' parts, is built in and read as OpenRocket reads it; a program can look a part up, and the builder makes a rocket of the parts, each weighing what OpenRocket weighs it at but for a few departures the builder's page names ([Parts from a catalog](the-builder.md#parts-from-a-catalog), [`.orc` parts catalogs](format/orc.md)) | [`.ork` design files](format/ork.md), [The HPR design format](format/hpr.md), [`.orc` parts catalogs](format/orc.md) |
| Flight logs | Reads a PerfectFlite altimeter's `.pf2` log on its own, with no design file, and takes liftoff, apogee, the top speed, landing and the descent from it, each saying where it came from or why the log can't support it | [Reading a flight log](reading-a-flight-log.md), [Flight-log readings](physics/log-readings.md), [`.pf2` files](format/pf2.md) |
| Monte Carlo | One rocket flown many times with its mass, drag, motor, wind, rail and recovery delays scattered, seeded so a run repeats exactly; the spread of the apogee, the landing or any number a flight reports, with failed flights counted. From Rust or `hpr mc`, staged flights included; every flight exported as CSV, or, for a design without staging, returned to Python as NumPy arrays, the same numbers to the bit with both built in release mode | [Monte Carlo dispersion](monte-carlo.md), [`hpr mc`](cli.md#hpr-mc) |
| Optimization | Three optimizers search for the design that makes a result best. [CMA-ES](glossary.md#cma-es) tunes a design's numbers (a ballast mass, a body length) and picks from choices such as a motor or a catalog nose cone, within limits such as a minimum stability margin. [NSGA-II](glossary.md#nsga-ii) weighs two goals against each other, such as apogee against stability. [EGO](optimization.md#few-evaluations-ego), efficient global optimization, is for models so slow that only tens of evaluations can be afforded. Each is held to test functions whose answers are known, and CMA-ES and NSGA-II also to outside implementations. On a rocket, an answer is only as good as HPR Sim's flight models. From Rust only; EGO is checked on two, three and six variables, and competition rule files are not built yet | [Optimization](optimization.md) |
| Python | The `hpr` package: the builder's rocket, motor, site and flight from Python, with the recording as NumPy arrays and a design read from a file. A Monte Carlo run, its flights as NumPy arrays. Built from source, not on PyPI; no staging; a design read from a file flies its stored parachutes with `recovery=True`, and most `.ork` configurations are refused as they don't fly as written | [Python](python.md) |

## What doesn't work yet

These are the gaps you are most likely to meet. Each model page lists what its own model leaves
out.

### Speed and angle of attack

- **Drag near and past Mach 1 is lightly checked.** Since
  [M1.8b1](decisions-and-roadmap.md#m1-8b1) (drag through Mach 1), a flight on HPR Sim's own drag flies
  from Mach 0 to 5, and one that reaches Mach 5, the top of the models, stops with an error.
  - Near and above the speed of sound, the
    [transonic and supersonic](glossary.md#transonic-and-supersonic) speeds, the drag is Niskanen's
    2009 method, which is semi-empirical: formulas fitted to measurements.
  - Against NASA's wind-tunnel tests of the Arcas Robin sounding rocket, from Mach 0.6 to 4.63, it
    reads high at most speeds, most of all with fins past Mach 1
    ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)).
  - So for a rocket that goes past Mach 1, expect HPR Sim's apogee to come out low rather than high.
    Its base drag, on the flat aft end, hasn't been checked faster than Mach 0.3 at all.
  - It has been compared with [RASAero II](glossary.md#rasaero-ii)'s drag near Mach 1; the
    comparison is not uniformly within the 10% target. See [Accuracy](accuracy.md) for the cases
    and errors. A drag table from another tool can replace HPR Sim's own drag
    ([Aerodynamics](physics/aero.md#drag)).
- **The normal force still has gaps near Mach 1 and on some supersonic bodies.** The current body
  and Arcas Robin comparisons, including their measured error ranges, are in
  [Aerodynamics](physics/aero.md) and [Accuracy](accuracy.md).
- **Small angles of attack only.** Nothing models [stall](glossary.md#stall), the loss of lift at
  a large angle of attack, yet a flight uses the same models at every angle. So results near rail
  exit in a strong crosswind, and near apogee, are the least trustworthy.

### Staging, two-stage rockets, clusters and air starts

- **Staging is new, and checked against OpenRocket on its own examples.** Each motor lights at its
  own time, an [air start](glossary.md#air-start) included, and a [sustainer](glossary.md#sustainer)
  flies on after dropping its [booster](glossary.md#booster), which lands on its own
  ([Staging](physics/staging.md)). Tests check the bookkeeping. OpenRocket's two-stage and
  three-stage examples, its cluster example and its air-start example, 15 flights in all, are
  each within 5% of OpenRocket's apogee and largest speed. Five apogees, three of the cluster's
  and two of the three-stage example's, are compared with OpenRocket's flight with no parachute,
  since its parachute opened before apogee
  ([M1.9c](decisions-and-roadmap.md#m1-9c), a two-stage and a cluster design against OpenRocket;
  [M4.5g2](decisions-and-roadmap.md#m4-5g2), several separations;
  [results](format/ork.md#staged-clustered-and-air-start-flights)).
  - A `.ork` file's ignition settings and every powered separation are read, from the tail
    forward ([M4.5g2](decisions-and-roadmap.md#m4-5g2), several separations). A powered separation
    has a time known before the flight, and then a motor ahead of it is still burning or yet to
    light and none behind it is. One at its own stage's first burnout may drop the stage's own
    other motors still burning, if they lit strictly before the split, as OpenRocket does
    ([M4.5n](decisions-and-roadmap.md#m4-5n), a stage's first burnout). The ignitions come with
    the rocket. Your program passes the separation to the flight, with a recovery device on each
    part, since HPR Sim refuses the flight without them; `hpr sim` does the same for you
    ([Separation](cli.md#separation)). The example [`ork_two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs) does both. A lone separation with nothing ahead of it left to burn, such as a payload's, is
    read too; `hpr sim` flies it when the payload's own device opens at the split
    ([M4.5g3](decisions-and-roadmap.md#m4-5g3), a payload's split). One beside another
    separation, or at or after apogee beside another, isn't flown
    ([which configurations fly](format/ork.md#which-configurations-the-rocket-flies)).
  - A [cluster](glossary.md#cluster), several motors burning side by side, flies, whether they
    share one mount of several tubes or each has its own: HPR Sim adds up their thrust and mass, and a
    motor off the rocket's center line adds a turning moment. A motor that fails to light can be
    set, and the rocket then turns as a hand calculation says ([Clusters](physics/design.md#clusters)).
    A `.ork` file's cluster flies with a motor in every tube.
  - A rocket can [separate](glossary.md#separation) into parts for recovery once the aft part's
    motors have burnt out.

### Effects left out

- **Some effects are left out of a flight, or approximated:**
  - [tip-off](glossary.md#tip-off) (the rocket pitching as it leaves the rail), thrust
    misalignment (a motor pushing slightly off the rocket's axis), and turbulence (the model
    exists, but a flight doesn't use it), none of which a milestone plans yet
    ([issue #39](https://github.com/nrdptel/fusionspace-eridanus/issues/39) tracks turbulence);
  - the shock load when a parachute opens, and the canopy's overshoot of its steady drag
    ([Recovery](physics/recovery.md));
  - the [internal momentum](glossary.md#internal-momentum) of the burning propellant, which is
    counted twice, as RocketPy counts it: a thrust curve measured on a test stand already includes
    its effect, and the equations of motion add it again. On Valetudo, the rocket that
    [Getting started](getting-started.md) flies, it adds 21 N to the push at liftoff and changes
    the burnout speed by at most 0.05 m/s (0.2 ft/s); see
    [Rigid-body flight](physics/flight.md#equations-of-motion).

### Outputs and ways to use it

- **Output files.** [Exporting a flight](exporting-a-flight.md) writes a recording as CSV, JSON or
  Parquet ([M1.10c2](decisions-and-roadmap.md#m1-10c2)), and the flight path with its landings as
  GeoJSON or KML for a map. A flight's peaks, its stability
  margin from the rail exit to apogee (the weakest direction's, for a rocket with a fin set of
  one or two fins; [#329](https://github.com/nrdptel/fusionspace-eridanus/issues/329)), the optimum ejection delay and its landing's latitude and
  longitude are in [Flight metrics](physics/metrics.md), and its fins' flutter speed and margin in
  [Fin flutter](physics/flutter.md).
- **Where an export comes from.** A CSV's header gives each column's unit in brackets
  (`time [s]`). `hpr sim`'s JSON recording, and the small file `hpr sim` and `hpr mc` write beside
  a CSV, carry the motor catalog's as-of date and how far to trust the numbers (since
  [M0.9c5](decisions-and-roadmap.md#m0-9c5), provenance on the exports), and so do GeoJSON, KML,
  Parquet and the `--plot` figure (since [M0.9c8](decisions-and-roadmap.md#m0-9c8), the date and
  the note on `hpr sim`'s other exports). A catalog motor `hpr convert` writes says the catalog's
  date, and a weather profile `hpr weather --output` saves says whether it is measured, a forecast
  or a reanalysis, and how far to trust it (since [M0.9c14](decisions-and-roadmap.md#m0-9c14), provenance on the other files).
- **It reads what you type, and says what to do when it can't.** Numbers are read as written by
  hand: `--elevation 1,280` is 1280 m (4199 ft), and a comma it can't read as a thousands
  separator, such as `3,9`, is asked about. A refused option is named with its value and unit and
  an example, and a file that isn't there with the closest name in its folder
  ([typing numbers](cli.md#typing-numbers-and-what-a-refusal-says);
  [M0.9c6](decisions-and-roadmap.md#m0-9c6), errors and typed numbers). Every refusal ends with
  a `help:` line that says what to do, and a fetch that waits on the network says on the terminal
  what it waits for ([M0.9c15](decisions-and-roadmap.md#m0-9c15), next steps and waits).
- **The command line flies powered separations only.** `hpr sim` flies a `.ork` file or
  an HPR design file, exports its recording ([The command line](cli.md#hpr-sim)), and with `--plot`
  draws its altitude, speed and acceleration against time, events marked and listed in a table,
  as an SVG in the FusionSpace chart style, its panels' heights chosen so their slopes run near
  45° ([banked](cli.md#plotting-the-flight)) and a title block at its end,
  with the numbers it draws in a CSV beside it
  ([Plotting the flight](cli.md#plotting-the-flight); [M4.5e](decisions-and-roadmap.md#m4-5e),
  plots; [M0.6c](decisions-and-roadmap.md#m0-6c), their style;
  [M0.9c12](decisions-and-roadmap.md#m0-9c12), balloons, banking, title block and data).
- **`hpr sim` reads by name.** It leads with the static margin and the center of gravity and
  center of pressure it came from, apogee, rail exit speed, ejection delay and the descent speed
  under open parachutes, each height and speed in SI with feet or feet per second in parentheses
  ([units](cli.md#units)), and ends with how far to trust it, from 55 logged flights
  ([M0.9c3](decisions-and-roadmap.md#m0-9c3)), and calls configurations and parts by name,
  such as `[C6-5]` for a configuration the file leaves unnamed
  ([M4.5d](decisions-and-roadmap.md#m4-5d), readable output). It flies a
  `.ork` file's parachutes and streamers, and its powered separations, one or several
  ([M4.5g1](decisions-and-roadmap.md#m4-5g1), powered separation in `hpr sim`;
  [M4.5g2](decisions-and-roadmap.md#m4-5g2), several separations), and a payload dropped with
  nothing left to burn, when its own parachute opens at the split
  ([M4.5g3](decisions-and-roadmap.md#m4-5g3), a payload's split). A rocket's `.json` carries no
  parachute. A Rust program flies separations and parachutes:
  [Getting started](getting-started.md) flies a first rocket, and [The builder](the-builder.md)
  builds and flies one of your own in a few calls. A Python program flies the parachutes of a
  rocket it builds part by part, or a design file's with `recovery=True`, but no staging
  ([Python](python.md)).
- **`.ork` files are read, but most don't fly with their own motors.** A `.ork` file is read
  ([`.ork` design files](format/ork.md)), but few of its motor configurations fly as written, as
  HPR Sim has few motors' curves. Its parachutes and streamers fly as OpenRocket flies them
  ([Recovery](physics/recovery.md#from-an-openrocket-file)). `hpr sim --motor` flies one
  with a motor you give ([The command line](cli.md#hpr-sim)).
- **Monte Carlo runs give the apogee's spread and landing ellipses, from Rust, the command
  line or Python.** A [Monte Carlo](glossary.md#monte-carlo) run flies a rocket many times with its mass,
  drag, motor, wind and rail scattered, seeded and reproducible
  ([Monte Carlo dispersion](monte-carlo.md)), and draws the ellipse its landings fall in
  ([Landing ellipses](monte-carlo.md#landing-ellipses)). [`hpr mc`](cli.md#hpr-mc) runs one on any
  design `hpr sim` flies and exports every flight as CSV; for a design without staging,
  `hpr.MonteCarlo` returns the same flights to Python as NumPy arrays
  ([Python](python.md#monte-carlo)), but not the ellipses.
  Morris screening and Sobol' indices rank which inputs matter most
  ([Sensitivity analysis](sensitivity.md)). CMA-ES finds the values of a design's numbers that
  hit a target, such as the ballast and body length for a 3,048 m (10,000 ft) apogee with a chosen
  margin. CMA-ES also chooses a motor and a nose cone within a competition's limits, and NSGA-II
  weighs apogee against stability ([Optimization](optimization.md)). Their answers are only as good
  as HPR Sim's flight models. Competition rule files are not built yet.
  No app yet: it is on the [roadmap][roadmap].
- **The flight-log analyzer reads one logger so far.** `hpr analyze` reads a PerfectFlite
  altimeter's `.pf2` log on its own, with no design file and no simulation, and prints liftoff,
  apogee, the top speed, landing and the descent, each saying where it came from, or withheld with
  the reason the log can't support it ([Reading a flight log](reading-a-flight-log.md)). Like
  `hpr weather` and `hpr motors show`, it ends with how far to trust its figures (since
  [M0.9c4](decisions-and-roadmap.md#m0-9c4), units and trust notes on every readout). The other
  loggers come with [M7.1](decisions-and-roadmap.md#m7-1): Altus Metrum (AltOS), Featherweight
  (Raven, Blue Raven and the GPS tracker), Missile Works RRC3, Eggtimer, Entacore AIM,
  Mercury/AltimeterCloud, CATS, and plain CSV with column mapping. The rest of the readings, such
  as burnout and each leg of the descent, come with [M7.2](decisions-and-roadmap.md#m7-2), and
  comparing a flight with a simulation of it with [M7.3](decisions-and-roadmap.md#m7-3).

<a id="how-far-to-trust-it"></a>

## Accuracy so far

[Accuracy](accuracy.md) gathers every result so far, gaps included. The accuracy work aims first
at the flights most rocketeers make, the core band: up to Mach 2.5. It assumes angles of attack of
15° or less. The wider envelope runs to Mach 3.5, at any angle
([the operating envelope](VALIDATION.md#operating-envelope)). Whole flights are checked mostly
below about Mach 1.15 so far. Faster flights still fly, and say so: a flight beyond the validated
range, at high angle of attack, outside the core band or beyond the envelope carries a flag
([M1.14a1, the envelope flags](decisions-and-roadmap.md#m1-14a1)), and a flight that meets a known
error in HPR Sim's drag, its stability or the flight's path warns by its issue number
([the lists](VALIDATION.md#operating-envelope); [M1.14a2, the drag issue
warnings](decisions-and-roadmap.md#m1-14a2); [M1.14a3, the stability
ones](decisions-and-roadmap.md#m1-14a3); [M10.1d2, four more and a flight unstable under
power](decisions-and-roadmap.md#m10-1d2); [M10.1d6, the flight-path
warnings](decisions-and-roadmap.md#m10-1d6)). A flight whose static margin falls below zero while a
motor burns says its apogee is not a prediction. In brief:

- **Whole flights match RocketPy's in height, speed and time when both codes fly the same drag.**
  Six of RocketPy's example rockets agree within 3% on apogee, speeds, burnout and flight time
  ([M2.1b2](decisions-and-roadmap.md#m2-1b2), the whole-flight comparison). So does where they
  go, except for rockets that leave the rail slowly in a wind. There HPR Sim's
  [body lift](glossary.md#body-lift), which RocketPy leaves out, its later release from the rail
  and, for one rocket, its simpler fin model put the apogee's drift −4.333% to −38.158% from
  RocketPy's, and the landing's −21.686% to +36.678% ([report][report];
  [Accuracy](accuracy.md#whole-flights-against-rocketpy)). With HPR Sim's own drag, against RocketPy
  flying the drag its examples ship, HPR Sim's heights differ from RocketPy's by −7.280% to +10.302%
  ([report][report]). The larger gaps are where the two drags differ most: HPR Sim's is well below
  the example's for two rockets, and above it at high speed for Prometheus 2022, which flies
  through Mach 1 ([Accuracy](accuracy.md#whole-flights-with-each-codes-own-drag)).
- **Against seven real flights, HPR Sim's apogees miss by 6.04% on average,** outside the 5% target,
  and by −8.90% to +10.40% one by one. HPR Sim's height is read the way each log's barometric
  altimeter reads the air (four of the seven assumed barometric); drift and speed are not
  compared yet
  ([real flights](accuracy.md#real-flights),
  [report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md)).
- **Against 55 more real flights, from a private collection, HPR Sim and OpenRocket both over-predict
  the logged apogee,** HPR Sim by 9.83% on average and OpenRocket by 9.00%, while agreeing with each
  other within 5% on 53 of them; neither meets the 5% target. The flights' owners allow only
  aggregate numbers
  ([private collection](accuracy.md#real-flights-of-the-private-collection),
  [report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md)).
- **The descent under a parachute matches RocketPy's.** The comparison flies the descents of five
  of RocketPy's [example rockets](glossary.md#example-rockets) in both codes:
  - Each starts from the same state near apogee, with the first parachute opening at once.
  - Both codes get the same [drag areas](glossary.md#drag-area) and wind.
  - RocketPy's parachutes can add random noise, which would make each run differ; it is switched
    off.
  - HPR Sim uses RocketPy's formula for gravity, and RocketPy's way of interpolating the wind, that
    is, of working out the wind between the heights it is given.

  Six numbers are compared for each rocket: the descent time, the mean descent rate, the descent
  rate at landing, and the [drift](glossary.md#drift) in total, to the east and to the north. All
  30 agree with RocketPy's within 3%; the largest difference is +2.865%. That shows the two codes
  agree on the descent physics, not that either matches a real flight. The committed
  [validation report][report] has every number, and [Recovery](physics/recovery.md) explains the
  comparison.
- **Each model is tested on its own**: against exact answers, and where its source prints tables
  or worked examples, against those; several parts also against RocketPy. Each test states its
  [tolerance](glossary.md#tolerance). The largest known gaps:
  - The drag was checked at Mach 0.3 against other programs' curves (below), and from Mach 0.6 to
    4.63 against NASA's Arcas Robin wind tunnel. There it reads high at most speeds, most of all
    with fins past Mach 1: 2 of 44 measurements are within the 10% target set before measuring
    ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). The normal force
    and center of pressure were checked at Mach 0 against Barrowman's worked examples and from
    Mach 0.6 to 4.63 against the same wind tunnel, where they miss between Mach 0.8 and 1.2, and
    past Mach 3, the current comparison and its measured range are reported in
    [Aerodynamics](physics/aero.md#normal-force-through-mach-1).
  - The drag was compared with drag curves that come with RocketPy's example rockets, labelled as
    [RASAero II](glossary.md#rasaero-ii)'s (another rocket aerodynamics program). The curves don't
    record the fins' edges or the surface finish, so HPR Sim's copies of the designs follow a declared
    guess. HPR Sim is within 10% in four of the seven cases.
  - HPR Sim's drag is 18% low for Cavour, another of RocketPy's examples, while its motor burns
    ([power-on drag](glossary.md#power-on-and-power-off-drag)); the cause is not known yet.
  - HPR Sim's drag is 47% to 50% below Valetudo's example curve. Valetudo is the rocket that
    [Getting started](getting-started.md) flies. Its references disagree with each other, though:
    at Mach 0.3 the example curve gives a [drag coefficient](glossary.md#drag-coefficient) of
    1.05, 1.44 times the 0.728 in an [OpenRocket](glossary.md#openrocket) file of the same rocket.
    Given that OpenRocket file's own surface finish and launch lugs, HPR Sim gives 0.714, 1.9% under
    the file's 0.728 ([Aerodynamics](physics/aero.md#drag-verification)).
  - The Recruiter is a six-fin model rocket that J. S. Barrowman, whose
    [method](glossary.md#barrowmans-method) HPR Sim follows for the normal force, works through in
    his 1970 report Centuri TIR-33. HPR Sim's [normal-force slope](glossary.md#normal-force-slope)
    for it, how fast the sideways force grows with angle of attack, is 2.87% above his printed
    value, and 3.42% on the fins alone. Most of that comes from a different rule for six fins
    ([Aerodynamics](physics/aero.md#verification)).
  - [Tumbling](glossary.md#tumble-recovery) drag is −10 to +19% off its source's own drop tests.
    A separated body's parachute can open at a higher speed than it would for real, because the
    body falls with no drag until then ([Recovery](physics/recovery.md)).
- **You can check it yourself.** [Getting started](getting-started.md#checking-it-yourself) says
  how: run a program that shows how much the drag moves the apogee, trace any number with
  [Checking a claim](checking-a-claim.md), or compare HPR Sim's apogee by hand with your own
  altimeter's or another simulator's.

## Reading these pages

New here? [Getting started](getting-started.md) builds HPR Sim and flies a first rocket, and
[How a flight is simulated](how-a-flight-is-simulated.md) follows a flight from the pad to the
ground, linking the page for each model on the way.

Have an OpenRocket design? Three how-to guides take it through `hpr` on the command line:
[Fly your .ork](fly-your-ork.md), [Pick a motor](pick-a-motor.md) and
[Check stability for a certification flight](stability-for-certification.md).

Each model page opens with *In short*: what it models, its sources, how well it is validated and
what it leaves out. Below that, it names the code that implements the model, the sources it follows
and the tests that pin it.

Sources are cited by a short key in square brackets, such as **[N09]** for Niskanen's 2009 thesis
on model rocket simulation, with the full reference near the top of the page.

Equations are written in plain text, so they read the same here, on GitHub and in the code's
documentation. For example, once its drag balances its weight, a rocket under a parachute falls
at the steady speed `v_e = √(2 m g / (ρ C_D S))`, where `m` is its mass, `g` gravity, `ρ` the air's density and
`C_D S` the parachute's [drag area](glossary.md#drag-area)
([Recovery](physics/recovery.md#the-descent)).

Terms are defined in the [Glossary](glossary.md), and
[Checking a claim](checking-a-claim.md) shows how to trace any number to its source, its test and
its validation.

The code itself is documented in [the API reference](api.md), which Rust's documentation tool
generates from the source. It lists every public type and function, and each crate's front page
links back to the pages here that explain its models.

Three kinds of label link to the project's records on GitHub, which
[Decisions and the roadmap](decisions-and-roadmap.md) introduces:

- A **milestone**, such as [M1.8](decisions-and-roadmap.md#m1-8), is a step of the [roadmap][roadmap], the ordered plan
  of work.
- A **decision record**, such as [ADR-011][adr-011], explains a significant choice and the
  alternatives that were considered. All of them are in the [decision log][decisions].
- A **Loft lesson**, such as [Loft lesson L15](decisions-and-roadmap.md#l15), is a mistake found in Loft, the project
  that came before HPR Sim. A test here guards against it, or will once its milestone ships. They
  are listed in [Lessons from Loft][lessons].

Every page's source is a Markdown file in the repository's
[`docs/` folder](https://github.com/nrdptel/fusionspace-eridanus/tree/main/docs). The pencil icon at the top of
a page opens its source in GitHub's editor, where you can propose a fix.

A page prints in the light theme's colors, whichever theme you read it in, without the table of
contents or the buttons, with each link's address in parentheses after it and the title block,
with its version and date, at the end. The printer icon at the top opens every page as one, to
print the whole site.

## Title block

Every page of this site ends with a title block, the box in the corner of an engineering drawing
that says what a document is, who issued it, when, and which sources it rests on; its fields
follow ISO 7200, the standard for drawings' title blocks. This page's is at its foot, below the
page arrows. Its entries are written once, at the end of this section in the page's source, and
describe the whole site and this version of HPR Sim; each page adds its own title, as "Page",
after the site's. FusionSpace, the family of tools HPR Sim belongs to, closes its pages this way;
`FS-ACHERNAR · SW · TOOL 001` is HPR Sim's number in that family's register: Achernar, the
simulator's internal name, then software tool 1.

<div class="title-block">

| Field | Entry |
|---|---|
| Owner | FusionSpace |
| Title | FusionSpace HPR · Sim: a flight simulator for hobby and high-power rockets, and its documentation |
| Designation | `FS-ACHERNAR · SW · TOOL 001` |
| Version | 0.1.0, not yet released |
| Date of issue | <span id="issue-date">the date of the commit the site is built from</span> |
| Status | IN PREPARATION: no release yet; how far to trust each result is in [Accuracy so far](#accuracy-so-far) |
| Units | SI (meters, kilograms, seconds) first. Every command's text adds US units in parentheses: feet after heights and distances, feet per second after speeds, miles per hour after the wind, °F after the air's temperature, g after an acceleration and inches after stations along the rocket and a motor's size; `hpr motors show` adds ounces or pounds. The plot gives a second scale in feet, feet per second and g. This site's pages, unlike the program's output, keep a motor's nominal size in millimeters alone, as motors are named (`a 54 mm motor`); every other height, distance, size and speed on them gives its US units in brackets, checked as conversions ([house style](writing.md)). JSON and exported files stay SI ([units](cli.md#units)) |
| Data | Thrust curves: ThrustCurve.org's public-domain files, catalog captured 2026-09-17. Atmosphere: U.S. Standard Atmosphere 1976, or a sounding or forecast you supply or fetch ([weather](weather.md)). Magnetic field: WMM2025. Accuracy: the committed [validation report][report]. Every source and its terms: [third-party notices][notices]. |
| Fonts | Archivo and Cascadia Mono, SIL Open Font License 1.1, served from this site; no page asks another server for anything |

</div>

[adr-011]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0011-rigid-body-flight-equations-of-motion.md
[decisions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md
[lessons]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/loft-lessons.md
[notices]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/THIRD-PARTY-NOTICES.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md
