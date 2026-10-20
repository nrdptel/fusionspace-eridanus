# Start here

What FusionSpace HPR and its flight simulator, FusionSpace HPR · Sim, do today, and how to read
this site.

**FusionSpace HPR is a suite of open-source tools I'm building for hobby and high-power rockets.
Its first product, FusionSpace HPR · Sim (HPR Sim for short), is a flight simulator, and it isn't
ready to rely on yet.** A flight analyzer, a competition kit and an app are planned
([Decisions and the roadmap](decisions-and-roadmap.md)). This site explains what the simulator
models, where each model comes from, and how well each one has been checked. I wrote it for a
rocketeer who knows some physics and some code, but not this project.

> **How far to trust it.** Every number HPR Sim produces is an estimate from a model, not a
> measurement, and never a go/no-go verdict. Against 53 logged flights of fliers' own designs, its
> apogee missed the logged one by 12.95% on average, either way. That's the mean absolute miss,
> outside the 5% target ([the register's row on these misses](evidence.md#g24)). With signs kept, it
> averaged 8.73% above the logged one. It was within 10% on 26 and read low on 13, by at
> most 20%
> ([report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md)),
> and [Accuracy so far](#accuracy-so-far) gives the rest. Your motor's printed data, your
> altimeter and your range safety officer have the final word.

## What the simulator is

HPR Sim (called hpr-sim before release 0.1) simulates a rocket's flight from the launch rail to
landing. It's a [6-DOF](glossary.md#6-dof-six-degrees-of-freedom) simulator: it follows all six
degrees of freedom, the rocket's position along three axes and its rotation about three.
Meanwhile the motor burns, and the rocket's mass, [center of gravity](glossary.md#center-of-gravity-cg)
and inertia (how hard it is to turn) change.

I made it a Rust library first, meant to be built into other programs, as
[RocketPy](glossary.md#rocketpy) is. RocketPy is an open-source rocket flight simulator, written in
Python and used as a Python library. HPR Sim has a Python package too ([Python](python.md)), and a
graphical app is planned. It reads OpenRocket `.ork` design files today.

The command-line tool, `hpr`, flies a `.ork` or a rocket's JSON. It looks up and converts motor
files, searches vendors' motor stock and prices, and re-runs the validation. It also reads an
altimeter's flight log and fetches a launch day's weather ([The command line](cli.md)).
[HPR Sim](the-simulator.md) is the simulator's guide. [A tour of HPR Sim](simulator-tour.md) shows
it working on a small rocket and a public flight, and
[Compared with other simulators](simulator-compared.md) sets it beside OpenRocket, RocketPy and
RASAero II.

I built it to be checked, too. Every model cites a published source, and tests pin every model.
The simulator is compared with RocketPy, [OpenRocket](https://openrocket.info/) and the logs of
real flights, and the results are committed to the repository.

For now it covers only commercial off-the-shelf solid rocket motors
([COTS motors](glossary.md#cots-motor)), the kind you buy from a manufacturer. That stays true
until the first app.

Release 0.1.0 came out on 2026-10-08. Install its files as
[Install a release](getting-started.md#install-a-release) says, or build it from source.
[How a release is built](releasing.md) says what a release holds and how each of its files is
checked. [The changelog](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md)
says what 0.1.0 does, how far to trust it, and its known gaps. Among them are the open issues that
make a number look safer than it is.

Releases come at product boundaries, and I don't promise dates
([Phase 8, releases](decisions-and-roadmap.md#m10-1)). [Plan](plan.md) says what each one holds
and how likely each goal is:

- 0.1, the simulator: the library, the command line and the Python package, with Monte Carlo
  dispersion from the command line and Python. It waited on
  [the logged flights' apogees](decisions-and-roadmap.md#m2-3c1) and named fixes
  ([release 0.1](decisions-and-roadmap.md#m10-1)). It shipped with its known limits listed. Among
  them is drag that reads high above Mach 0.8, so the apogee there reads low.
- 0.2, the flight analyzer: it reads the common altimeters' logs on their own, with no design
  file. Then it compares a log with its simulation and fits the rocket's drag from it.
- 0.3, accuracy and diagnosis: the simulator measured against logged flights and improved, and a
  flight's likely faults ranked from its log.
- 0.4, the competition kit: rules presets for Student Launch, IREC and others, and an optimizer.
  It adds the numbers a submission asks for, ejection-charge sizing, and RASAero and RocketPy
  files.
- 0.5, an app preview, on the desktop and in a web browser. Open a design or a log, fly it and
  replay it in 3D, with the simulated flight drawn as a "ghost" beside the real one.
- 1.0, the app, with its design editor.

After 1.0, new product lines come one at a time, in an order that may still change. First comes
designing your own solid motors, then parachutes. Then a pad-day mode on your phone: the day's
forecast, the best launch hour, drift on the field and a flight card. Then flight computers and
trackers. Hybrids and liquids come last
([the second-pass plan](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0163-the-second-pass-products-and-priorities.md)).

## What works today

These parts are built and tested. Each page gives its sources, and most say what they leave out.

The pages, the program's messages and the library's names use US spelling for the words
[the style guide](writing.md) checks. Some saved forms held a British-spelled key: a landing
ellipse, a design-check finding, a laid-out rocket and a few more. They still read the old key but
write the new one, so an older build may not read what this one saves. Design files never held one
and are unchanged ([US names](decisions-and-roadmap.md#m0-6e)).

### Earth

Gravity follows [WGS 84](glossary.md#wgs-84), the model of the Earth's shape and gravity that GPS
uses, and varies with latitude and height. The Earth's rotation is in the flight too. HPR Sim works
with launch-site coordinates and gives the distance and bearing between two places. A site's
elevation is looked up online or read from a GeoTIFF file. The detail is in
[Frames](physics/frames.md), [Geodesy](physics/geodesy.md), [Gravity](physics/gravity.md) and
[A launch site's elevation](elevation.md).

### Air

The air is the 1976 US [Standard Atmosphere](glossary.md#standard-atmosphere) up to 86 km
(282,152 ft), with temperature offsets and humidity. Or it's a [sounding](glossary.md#sounding), a
measured or forecast profile of pressure, temperature and wind against height. A sounding can be a
real day's weather from an [ERA5](glossary.md#era5) file, an Open-Meteo forecast, a weather
balloon, or NOAA's GFS and RAP forecasts. The detail is in [Atmosphere](physics/atmosphere.md),
[ERA5 weather files](format/era5.md), [Launch-day weather](weather.md),
[Weather-balloon soundings](soundings.md) and [NOAA forecasts: GFS and RAP](nomads.md).

### Wind

Wind profiles can be constant, layered, power-law or logarithmic.
[Turbulence](glossary.md#turbulence-dryden), random gusts, comes from a random-number generator
started from a [seed](glossary.md#seed), a number you choose. The same seed gives exactly the same
gusts every time on the same platform (operating system and processor). The detail is in
[Wind](physics/wind.md) and [Turbulence](physics/turbulence.md).

### Motors

HPR Sim reads `.eng` and `.rse` [thrust curves](glossary.md#thrust-curve), and follows a motor's
thrust, mass, center of gravity and inertia through the burn.
It bundles [32 motors](physics/motor.md#the-bundled-motors). One the catalog lacks is fetched from
ThrustCurve.org by name and cached, so it flies offline after that
([motors on demand](decisions-and-roadmap.md#m4-5b)). Stock and prices come from
motor.fusionspace.co. The detail is in [Solid motors](physics/motor.md),
[`.eng` files](format/eng.md), [`.rse` files](format/rse.md) and
[Motor stock and prices](motor-stock.md).

### The rocket

A rocket is built from nose cones, body tubes, transitions (tapered sections between tubes of
different diameters), fins and other parts, each with its material. HPR Sim works out the whole
rocket's mass properties and checks the design. Everyday fits warn, and only what can't be built is
refused ([design checks that match reality](decisions-and-roadmap.md#m4-5c)). A packed part drawn
wider than its bore warns. So does a part that fits a cone's wide end but meets its wall where
it narrows. The detail is in [Design tree](physics/design.md), [Shapes](physics/shapes.md) and
[Mass properties](physics/mass.md).

### Aerodynamics

HPR Sim finds the [center of pressure](glossary.md#center-of-pressure-cp), where the aerodynamic
force acts. Its distance behind the center of gravity is the
[stability margin](glossary.md#stability-margin). HPR Sim also finds drag and the
[normal force](glossary.md#normal-force). That's the sideways force when the rocket flies at an
angle to the airflow, its [angle of attack](glossary.md#angle-of-attack). For small angles of attack, both
run from [Mach](glossary.md#mach-number) 0 to 5. Both are checked against a wind tunnel from Mach
0.6 to 4.63, and the drag reads high at most speeds. A part's own drag coefficient, stated in a
`.ork` file, flies as OpenRocket flies it
([A part's stated drag coefficient](physics/aero.md#a-parts-stated-drag-coefficient)). OpenRocket's
*Base drag hack* example relies on that. It flies within 4.52% of OpenRocket's apogee on two of its
three motors. On the third it's 7.80% high, which misses the 5% bar. The detail is in
[Aerodynamics](physics/aero.md).

### Flight

A flight runs from the launch rail through powered flight and the coast to apogee. It takes an
[adaptive time step](glossary.md#adaptive-time-step) and marks [events](glossary.md#event) such as
burnout and apogee. The detail is in [Rigid-body flight](physics/flight.md) and
[Time integration](physics/integration.md).

### Recovery

Parachutes, [streamers](glossary.md#streamer) and [tumbling](glossary.md#tumble-recovery) bring a
rocket down, and HPR Sim gives the [drift](glossary.md#drift) they carry it downwind. A rocket can
[separate](glossary.md#separation) into bodies that each descend on their own. A `.ork` file's
parachutes and streamers fly as OpenRocket flies them, in `hpr sim` too. HPR Sim's landing speed is
within 0.02% of OpenRocket's on 50 of its 53 example flights, and within 0.12% on all 53
([Recovery](physics/recovery.md#from-an-openrocket-file)). `hpr sim` also flies a `.ork`'s powered
separations, one or several, though a dropped booster's landing is rough
([powered separation in `hpr sim`](decisions-and-roadmap.md#m4-5g1)). Staged flights and a
payload's drop are [below](#staging-two-stage-rockets-clusters-and-air-starts).

### Design files

HPR Sim opens an OpenRocket `.ork` file, zipped, gzipped or plain XML, and reads the whole design.
That's the stages, the parts on and inside them, the motor configurations, when parachutes open,
and the simulations OpenRocket stored. The airframe's shape is cross-checked against a second
reader and OpenRocket itself, and so are its mass and center of gravity
([Mass properties](physics/mass.md#checked-against-openrocket)). Pods, parallel stages and fins on
a nose cone are read, weighed and flown ([Pods](physics/aero.md#pods)). On OpenRocket's examples
with them, two checks are weaker than the rest. One flight is compared only up to where OpenRocket
aborts it, and one design's stability margin reads 0.07 calibers high, the flattering side. A part
HPR Sim can't shape honestly is left out with a warning.

`hpr sim` flies 136 of the 170 motor configurations in the reference library and OpenRocket's
examples as saved, 54 of OpenRocket's 56 among them. It flies 9 more with
`--accept-design-errors`, and the other 25 are refused, mostly for a motor with no curve
([How many configurations `hpr sim` flies](format/ork.md#how-many-configurations-hpr-sim-flies)).
A design [writes back out as a `.ork`](format/ork.md#writing-a-ork-back-out) that reads back as the same design. It can also be kept as a
`.hpr` file in [the HPR design format](format/hpr.md), which TypeScript and Python programs read
with generated types. OpenRocket's parts catalog is built in: 16 `.orc` files with 3,449 makers'
parts. The builder makes a rocket of its parts
([Parts from a catalog](the-builder.md#parts-from-a-catalog), [`.orc` parts catalogs](format/orc.md)).
The detail is in [`.ork` design files](format/ork.md).

### Flight logs

`hpr analyze`, Python and the library read a flight log on its own, with no design file
([Reading a flight log](reading-a-flight-log.md)). From any logger's barometric altitude or
pressure, they take liftoff, apogee, the top speed, landing, the flight time and the descent under
each canopy. An accelerometer along the rocket's axis adds the top acceleration and burnout.
Without one, burnout is a [floor](glossary.md#floor), the top speed's time less its bound. Each
reading is within a bound held on the synthetic flights' tuning set, or withheld with the reason
the log can't support it ([Flight-log readings](physics/log-readings.md)). The synthetic flights
are 2,200 simulated flights, each with its logger's log and its known truth
([Synthetic flights](physics/synthetic-flights.md)).

The library reads each format into one record, every column kept on its own clock. The formats
come from PerfectFlite ([`.pf2` files](format/pf2.md)), AltOS, Featherweight's Raven and Blue Raven, Missile Works' RRC3 and RTx,
and Eggtimer. Others come from AltimeterCloud, Entacore AIM, AerospaceNU, Silicdyne, the ARTS2,
the R-DAS, AltiMAX and MARSA. A phone app's log, GPS tracks, a course's Teensy logger and one
team's flight computer are read too. So are zip archives and Excel workbooks. A table no reader knows is read by the
units its header states, or by a column mapping you write. Binary records are read by a layout
you write ([Reading your own logger](your-own-logger.md)). [Flight logs](format/flight-logs.md)
covers each layout and how it was checked, and
[the notes below](#outputs-and-ways-to-use-it) give some readers' checks.

### Monte Carlo

A Monte Carlo run flies one rocket many times, with its mass, drag, motor, wind, rail and recovery
delays scattered. It's seeded, so a run repeats exactly. It gives the spread of the apogee, the
landing or any number a flight reports, with failed flights counted. It runs from Rust or
[`hpr mc`](cli.md#hpr-mc), staged flights included, and exports every flight as CSV. For a design
without staging, Python gets the flights as NumPy arrays. They're the same numbers to the bit,
with both built in release mode ([Monte Carlo dispersion](monte-carlo.md)).

### Optimization

Three optimizers search for the design that makes a result best.
[CMA-ES](glossary.md#cma-es) tunes a design's numbers, such as a ballast mass or a body length. It
also picks from choices such as a motor or a catalog nose cone, within limits such as a minimum
stability margin. [NSGA-II](glossary.md#nsga-ii) weighs two goals against each other, such as
apogee against stability. [EGO](optimization.md#few-evaluations-ego), efficient global
optimization, is for models so slow that only tens of evaluations can be afforded. Each is held to
test functions whose answers are known, and CMA-ES and NSGA-II also to outside implementations. On
a rocket, an answer is only as good as HPR Sim's flight models. They run from Rust only, and
competition rule files aren't built yet ([Optimization](optimization.md)).

### Python

The `hpr` package builds the rocket, motor, site and flight from Python, returns the recording as
NumPy arrays, and reads a design from a file. It runs Monte Carlo too, its flights as NumPy arrays.
Install it with `pip install fusionspace-hpr` from PyPI, or build it from source. It has no
staging. A design read from a file flies its stored parachutes with `recovery=True`, but most `.ork`
configurations are refused, as they don't fly as written ([Python](python.md)).

### Evidence

[What backs each claim](evidence.md) is a register of the gaps these pages state in what backs a
file reader, a model or a check. Each row gives the kind of evidence behind a claim, the kind that
would settle it, and the open issue waiting for a real sample. The build fails a gap stated in one
of its listed phrases with no row, though a gap worded otherwise passes
([the evidence register](decisions-and-roadmap.md#m2-10)).

## What doesn't work yet

These are the gaps you're most likely to meet. Each model page lists what its own model leaves
out, and [What backs each claim](evidence.md) lists the gaps the pages state, with the issue that
waits for each one's evidence.

### Speed and angle of attack

Drag near and past Mach 1 is lightly checked. Since
[the drag buildup through Mach 1](decisions-and-roadmap.md#m1-8b1) was built, a flight on HPR
Sim's own drag flies from Mach 0 to 5. One that reaches Mach 5, the top of the models, stops with
an error.

Near and above the speed of sound, the
[transonic and supersonic](glossary.md#transonic-and-supersonic) speeds, the drag is Niskanen's
2009 method. It's semi-empirical: formulas fitted to measurements. Against NASA's wind-tunnel tests
of the Arcas Robin sounding rocket, from Mach 0.6 to 4.63, it reads high at most speeds. It's
highest with fins past Mach 1
([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). So for a rocket that
goes past Mach 1, expect HPR Sim's apogee to come out low rather than high. Its base drag, on the
flat aft end, has been compared with two references, and neither is a direct measurement. One is
MIL-HDBK-762's worked example, a handbook calculation from Mach 0.5 to 3.2. The other is Love's
correlation of measured bases. The base drag reads high from Mach 0.8 to 1.2 against the first,
and low from Mach 1.5 against the second
([issue 68, the base drag's error](https://github.com/nrdptel/fusionspace-eridanus/issues/68);
[the handbook's calculation](physics/aero.md#drag-against-mil-hdbk-762s-sample-calculation)).

It's also been compared with [RASAero II](glossary.md#rasaero-ii)'s drag near Mach 1, and that
comparison isn't within the 10% target everywhere. [Accuracy](accuracy.md) has the cases and
errors. A drag table from another tool can replace HPR Sim's own drag
([Aerodynamics](physics/aero.md#drag)).

The normal force still has gaps near Mach 1 and on some supersonic bodies. The current body and
Arcas Robin comparisons, with their measured error ranges, are in
[Aerodynamics](physics/aero.md) and [Accuracy](accuracy.md).

HPR Sim's models are for small angles of attack: its accuracy work assumes 15° or less
([the operating envelope](VALIDATION.md#operating-envelope)). Nothing models [stall](glossary.md#stall), the loss of lift
at a large angle of attack, yet a flight uses the same models at every angle. So results near
rail exit in a strong crosswind, and near apogee, are the least trustworthy.

### Staging, two-stage rockets, clusters and air starts

Staging is new, and it's checked against OpenRocket on OpenRocket's own examples. Each motor
lights at its own time, an [air start](glossary.md#air-start) included. A
[sustainer](glossary.md#sustainer) flies on after dropping its [booster](glossary.md#booster),
which lands on its own ([Staging](physics/staging.md)). Tests check the bookkeeping. OpenRocket's
two-stage, three-stage, cluster and air-start examples make 15 flights in all. Each is within 5%
of OpenRocket's apogee and largest speed. In OpenRocket's flights of five of them, three of the
cluster's and two of the three-stage example's, the parachute opened before apogee. Those five
apogees are compared with OpenRocket's flight with no parachute
([a two-stage and a cluster design against OpenRocket](decisions-and-roadmap.md#m1-9c);
[several separations](decisions-and-roadmap.md#m4-5g2);
[results](format/ork.md#staged-clustered-and-air-start-flights)).

A `.ork` file's ignition settings and every powered separation are read, from the tail forward
([several separations](decisions-and-roadmap.md#m4-5g2)). A powered separation has a time known
before the flight. At that time a motor ahead of it is still burning or yet to light, and none
behind it is. A separation at its own stage's first burnout may drop that stage's other motors
while they still burn, if they lit strictly before the split. OpenRocket does the same
([a stage's first burnout](decisions-and-roadmap.md#m4-5n)).

The ignitions come with the rocket. Your program passes the separation to the flight, with a
recovery device on each part, since HPR Sim refuses the flight without them. `hpr sim` does the
same for you ([Separation](cli.md#separation)). The example
[`ork_two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs)
does both. A lone separation with nothing ahead of it left to burn, such as a payload's, is read
too. `hpr sim` flies it when the payload's own device opens at the split
([a payload's split](decisions-and-roadmap.md#m4-5g3)). One beside another separation, or at or
after apogee beside another, isn't flown
([which configurations fly](format/ork.md#which-configurations-the-rocket-flies)).

A [cluster](glossary.md#cluster), several motors burning side by side, flies, whether they share
one mount of several tubes or each has its own. HPR Sim adds up their thrust and mass, and a motor
off the rocket's center line adds a turning moment. You can set a motor that fails to light, and
the rocket then turns as a hand calculation says ([Clusters](physics/design.md#clusters)). A
`.ork` file's cluster flies with a motor in every tube.

A rocket can [separate](glossary.md#separation) into parts for recovery once the aft part's
motors have burnt out.

### Effects left out

These are left out of a flight, or approximated:

- [tip-off](glossary.md#tip-off), the rocket pitching as it leaves the rail;
- thrust misalignment, a motor pushing slightly off the rocket's axis;
- turbulence: the model exists, but a flight doesn't use it. No milestone plans these first three
  yet, though turbulence has
  [an open issue of its own, #39](https://github.com/nrdptel/fusionspace-eridanus/issues/39);
- the shock load when a parachute opens, and the canopy's overshoot of its steady drag
  ([Recovery](physics/recovery.md));
- the [internal momentum](glossary.md#internal-momentum) of the burning propellant, counted twice,
  as RocketPy counts it. A thrust curve measured on a test stand already includes its effect, and
  the equations of motion add it again. On Valetudo, the rocket that
  [Getting started](getting-started.md) flies, it adds 21 N to the push at liftoff. It changes the
  burnout speed by at most 0.05 m/s (0.2 ft/s)
  ([Rigid-body flight](physics/flight.md#equations-of-motion)).

### Outputs and ways to use it

- **Output files.** [Exporting a flight](exporting-a-flight.md) writes a recording as CSV, JSON or
  Parquet ([Parquet recordings](decisions-and-roadmap.md#m1-10c2)), and the flight path with its
  landings as GeoJSON or KML for a map. [Flight metrics](physics/metrics.md) gives a flight's
  peaks and its stability margin from the rail exit to apogee. For a rocket with a fin set of one
  or two fins, that's the weakest direction's margin
  ([issue 329, one- and two-fin sets](https://github.com/nrdptel/fusionspace-eridanus/issues/329)). It also gives the optimum
  ejection delay and the landing's latitude and longitude. [Fin flutter](physics/flutter.md) gives
  the fins' flutter speed and margin.
- **Where an export comes from.** A CSV's header gives each column's unit in brackets
  (`time [s]`). `hpr sim`'s JSON recording carries the motor catalog's as-of date and how far to
  trust the numbers. So does the small file `hpr sim` and `hpr mc` write beside a CSV
  ([provenance on the exports](decisions-and-roadmap.md#m0-9c5)). So do GeoJSON, KML, Parquet and
  the `--plot` figure ([provenance on every export](decisions-and-roadmap.md#m0-9c8)). A catalog
  motor that `hpr convert` writes says the catalog's date. A weather profile that
  `hpr weather --output` saves says whether it's measured, a forecast or a reanalysis, and how far
  to trust it ([provenance on the other files](decisions-and-roadmap.md#m0-9c14)).
- **It reads what you type, and says what to do when it can't.** It reads numbers as you'd write
  them by hand: `--elevation 1,280` is 1280 m (4199 ft). A comma it can't read as a thousands
  separator, such as `3,9`, is asked about. A refused option is named with its value, its unit and
  an example. For a file that isn't there, it names the closest file in the same folder
  ([typing numbers](cli.md#typing-numbers-and-what-a-refusal-says);
  [errors and typed numbers](decisions-and-roadmap.md#m0-9c6)). Every refusal ends with a `help:`
  line that says what to do. A fetch that waits on the network says on the terminal what it's
  waiting for ([next steps and waits](decisions-and-roadmap.md#m0-9c15)).
- **`hpr sim` flies a design and plots its flight.** It flies a `.ork` file or an HPR
  design file and exports its recording ([The command line](cli.md#hpr-sim)). With `--plot` it
  draws the flight's altitude, speed and acceleration against time, as an SVG in the FusionSpace
  chart style. Events are marked and listed in a table, and a title block closes it. The panels'
  heights are chosen so their slopes run near 45° ([banked](cli.md#plotting-the-flight)). The
  numbers it draws go in a CSV beside it
  ([Plotting the flight](cli.md#plotting-the-flight); [plots](decisions-and-roadmap.md#m4-5e);
  [their style](decisions-and-roadmap.md#m0-6c);
  [balloons, banking, title block and data](decisions-and-roadmap.md#m0-9c12)).
- **`hpr sim` reads by name.** Its readout leads with the static margin, and the center of
  gravity and center of pressure it came from. Then come the apogee, the rail exit speed, the
  ejection delay and the descent speed under open parachutes. Each height and speed is in SI, with
  feet or feet per second in parentheses ([units](cli.md#units)). It ends with how far to trust
  it, from 53 logged flights ([how far to trust a result](decisions-and-roadmap.md#m0-9c3)). It
  calls configurations and parts by name, such as `[C6-5]` for a configuration the file leaves
  unnamed ([readable output](decisions-and-roadmap.md#m4-5d)). It flies a `.ork` file's
  parachutes and streamers, and its powered separations, one or several
  ([powered separation in `hpr sim`](decisions-and-roadmap.md#m4-5g1);
  [several separations](decisions-and-roadmap.md#m4-5g2)). It also flies a payload dropped with
  nothing left to burn, when its own parachute opens at the split
  ([a payload's split](decisions-and-roadmap.md#m4-5g3)). A rocket's `.json` carries no
  parachute. A Rust program flies separations and parachutes.
  [Getting started](getting-started.md) flies a first rocket, and [The builder](the-builder.md)
  builds and flies one of your own in a few calls. A Python program flies the parachutes of a
  rocket it builds part by part, or a design file's with `recovery=True`, but no staging
  ([Python](python.md)).
- **A `.ork` file's motors need a fetch the first time.** HPR Sim reads a `.ork` file
  ([`.ork` design files](format/ork.md)), but a file rarely carries its motors' curves, and HPR
  Sim bundles 32 motors. On the file's curves and those 32, 4 of the 170 motor configurations of
  the reference library and OpenRocket's examples fly. `hpr sim` looks for a missing curve on
  ThrustCurve.org and keeps what it finds. After that one fetch, 136 of the 170 fly as saved, 9
  more with `--accept-design-errors`, and 25 are refused
  ([how many fly](format/ork.md#how-many-configurations-hpr-sim-flies)). Its parachutes and streamers fly as OpenRocket
  flies them ([Recovery](physics/recovery.md#from-an-openrocket-file)). `hpr sim --motor` flies
  one with a motor you give ([The command line](cli.md#hpr-sim)).
- **Monte Carlo runs give the apogee's spread and landing ellipses, from Rust, the command line or
  Python.** A [Monte Carlo](glossary.md#monte-carlo) run flies a rocket many times with its mass,
  drag, motor, wind and rail scattered. It's seeded and reproducible
  ([Monte Carlo dispersion](monte-carlo.md)), and it draws the ellipse its landings fall in
  ([Landing ellipses](monte-carlo.md#landing-ellipses)). [`hpr mc`](cli.md#hpr-mc) runs one on
  any design `hpr sim` flies, and exports every flight as CSV. For a design without staging,
  `hpr.MonteCarlo` returns the same flights to Python as NumPy arrays, but not the ellipses
  ([Python](python.md#monte-carlo)). Morris screening and Sobol' indices rank which inputs matter
  most ([Sensitivity analysis](sensitivity.md)). CMA-ES finds the values of a design's numbers
  that hit a target. For example, it finds the ballast and body length for a 3,048 m (10,000 ft)
  apogee with a chosen margin. CMA-ES also chooses a motor and a nose cone within a competition's
  limits, and NSGA-II weighs apogee against stability ([Optimization](optimization.md)). Their
  answers are only as good as HPR Sim's flight models. Competition rule files aren't built yet.
  There's no app yet: it's on the [roadmap][roadmap].
- **The flight-log analyzer gives readings within bounds, not intervals yet.** `hpr analyze` and
  the Python package read a flight log on its own, with no design file and no simulation. They use
  its format's reader, or the column mapping or record layout you write for your own logger
  ([Reading your own logger](your-own-logger.md)). They give liftoff, apogee, the top speed,
  landing, the flight time and the descent under each canopy. From an accelerometer along the
  rocket's axis they also give the top acceleration and burnout. Burnout and each leg of the
  descent came with [the rest of the readings](decisions-and-roadmap.md#m7-2c). Each reading comes
  within a stated bound, or is withheld with the reason the log can't support it
  ([Reading a flight log](reading-a-flight-log.md)). A GPS track gives no reading, a board's three
  axes give no top acceleration, and no reading states a 90% interval yet. Like `hpr weather` and
  `hpr motors show`, it ends with how far to trust its figures
  ([units and trust notes on every readout](decisions-and-roadmap.md#m0-9c4)).

  Here are notes on some of the readers. Most say what the reader was checked against:

  - AltOS's raw downloads, the `.eeprom` files of the board's own log, match AltosUI's exports on
    every value converted to SI, in all 9 samples that have one. One export's acceleration, zeroed
    at +1 g, is counted apart ([AltOS raw downloads](decisions-and-roadmap.md#m7-1c10),
    [the layout](format/flight-logs.md#altoss-eeprom)).
  - The ARTS2's `.odf` is in units from its own calibration, and holds the same records as its raw
    CSV. It agrees with DataAnalyzer's export in all 3 samples: the acceleration within half a
    count, the barometric altitude in order only
    ([ARTS2 raw data](decisions-and-roadmap.md#m7-1c11),
    [the layout](format/flight-logs.md#the-arts2s-raw-data)).
  - The words of Missile Works' RTx GPS tracker's `.rtx` file are decoded from the files. They
    agree with mDACS's CSV and text exports on every fix but the last, which the exports leave out,
    in all 5 samples. Every field agrees in 4, and positions and altitudes in the fifth
    ([Missile Works' RTx](decisions-and-roadmap.md#m7-1c12),
    [the layout](format/flight-logs.md#missile-works-rtx)).
  - A table from a logger no other reader knows is read only as far as its header states its
    units, each unit held to the column's name and values. Of the 31 sample tables, 224 columns
    are read in a stated unit, and the rest kept as written
    ([tables read by their headers](decisions-and-roadmap.md#m7-1c13),
    [the layout](format/flight-logs.md#tables-read-by-their-headers)).
  - GPX 1.0 and 1.1 files: each value of three invented files equals an independent reader's
    ([GPX tracks](decisions-and-roadmap.md#m7-1d1), [the layout](format/flight-logs.md#gpx),
    [the register's row on GPX](evidence.md#g1)).
  - A zip archive of a flight's files is unpacked to read each one
    ([archives and two readers' variants](decisions-and-roadmap.md#m7-1c7)).
  - An Excel workbook's sheets are each read as a file. Every cell of the 16 sample workbooks
    agrees with an independent reader's ([Excel workbooks](decisions-and-roadmap.md#m7-1c14),
    [the layout](format/flight-logs.md#excel-workbooks)).
  - Eggtimer and RRC3 exports saved by a European locale's spreadsheet are read, with decimal
    commas tested on invented files. In the survey, six team tables are read by the column mapping
    RocketPy states for them
    ([mappings a publisher states](decisions-and-roadmap.md#m7-1c15),
    [the mappings](format/flight-logs.md#mappings-a-publisher-states)).
  - A team flight computer's binary records, from a raw file or a console capture's base64, are
    read by a layout the caller states. Every field of the 5 sample files matches the team's own
    decoded table ([binary records by a stated layout](decisions-and-roadmap.md#m7-1c16),
    [the layout](format/flight-logs.md#binary-records-by-a-stated-layout)).
  - A Blue Raven's USB download keeps only the records whose CRC checks, each held to its stated
    length, and reads the fields its manual lists. Every value of the 2 sample files matches the
    survey's own decoding
    ([the Blue Raven's USB download](decisions-and-roadmap.md#m7-1c17),
    [the layout](format/flight-logs.md#the-blue-ravens-usb-download)).
  - Every value of the 8 sample files from a course's Teensy data logger matches the survey's own
    decoding ([a course's Teensy data logger](decisions-and-roadmap.md#m7-1c18),
    [the layout](format/flight-logs.md#a-courses-teensy-data-logger)). Each unit no line states is
    held to the flight's physics: the barometer's altitude to its pressure by the BMP180's
    formula, 1 g at rest, the time to apogee.
  - A team's radio telemetry packets are read as its flight computer logged them, as a receiver
    heard them, and inside an RTF document. Every packet a receiver or an export holds that the
    onboard log also holds matches the log's, field for field
    ([radio packets](decisions-and-roadmap.md#m7-1c19),
    [the layout](format/flight-logs.md#one-teams-radio-telemetry-packets)).
  - A Raven FIP export pasted into a workbook with its header missing is read under a stated
    header ([workbooks with a reading beside](decisions-and-roadmap.md#m7-1c20),
    [the layouts](format/flight-logs.md#a-fip-export-pasted-below-no-header)). The two sample
    sheets match the `.FIPa` of the same flight, sample by sample. Every cell of the sample
    AltiMAX `.xls` export is checked by an outside reader, and each of its heights follows its
    pressure by the standard atmosphere's formula.
  - A MARSA altimeter's download is read in feet and g. Its units are held to an AltOS board's in
    the same flight, the apogee 0.981 and 0.986 times its. AltOS's radio telemetry decoded to a
    table is read by its header, and a table cut from an AltOS export by AltOS's state numbers. An
    AltOS export a spreadsheet re-saved keeps each formula as text
    ([altimeters' other text](decisions-and-roadmap.md#m7-1c21),
    [the layouts](format/flight-logs.md#a-marsa-download)).
  - A table whose header states too little, or that has none, is read by a column mapping. The
    sample tables' mappings each hold every unit to a physical check
    ([tables by mapping](decisions-and-roadmap.md#m7-1c22),
    [the layout](format/flight-logs.md#tables-a-mapping-holds-to-physics)).

  [The layouts read](format/flight-logs.md) covers each format. The first readers came with
  [the record, a survey and three families](decisions-and-roadmap.md#m7-1a) and
  [RRC3, Eggtimer and Blue Raven CSV](decisions-and-roadmap.md#m7-1b).
  [GPS tracks](decisions-and-roadmap.md#m7-1c1), [text loggers](decisions-and-roadmap.md#m7-1c2)
  and [club layouts](decisions-and-roadmap.md#m7-1c3) followed. Every sample log in a layout HPR
  Sim knows is read. The formats with no sample, such as CATS's, are
  [listed as needing samples](format/flight-logs.md#formats-that-need-samples)
  ([the register's row on these formats](evidence.md#g9)). Comparing a flight with a simulation of it is planned
  ([a flight against its simulation](decisions-and-roadmap.md#m7-3)).

<a id="how-far-to-trust-it"></a>

## Accuracy so far

[Accuracy](accuracy.md) gathers every result so far, gaps included. The accuracy work aims first
at the flights most rocketeers make, the core band: up to Mach 2.5. It assumes angles of attack of
15° or less. The wider envelope runs to Mach 3.5, at any angle
([the operating envelope](VALIDATION.md#operating-envelope)). So far, whole flights are checked
mostly below about Mach 1.15.

Faster flights still fly, and they say so. A flight carries a flag when it goes beyond the
validated range, at a high angle of attack, outside the core band or beyond the envelope
([the envelope flags](decisions-and-roadmap.md#m1-14a1)). A flight that meets a known error in
HPR Sim's drag, its stability or the flight's path warns by its issue number
([the lists](VALIDATION.md#operating-envelope)). The warnings came in steps:
[drag](decisions-and-roadmap.md#m1-14a2), [stability](decisions-and-roadmap.md#m1-14a3),
[four more and a flight unstable under power](decisions-and-roadmap.md#m10-1d2), and
[the flight's path](decisions-and-roadmap.md#m10-1d6). A flight whose static margin falls below
zero while a motor burns says its apogee isn't a prediction. The results so far:

- **Whole flights match RocketPy's in height, speed and time when both codes fly the same drag.**
  Six of RocketPy's example rockets agree within 3% on apogee, speeds, burnout and flight time
  ([the whole-flight cases](decisions-and-roadmap.md#m2-1b2)). So does where they go, except for
  rockets that leave the rail slowly in a wind. There, three things move HPR Sim's drift: its
  [body lift](glossary.md#body-lift), which RocketPy leaves out, its later release from the rail,
  and for one rocket its simpler fin model. They put the apogee's drift −4.333% to −38.158% from
  RocketPy's, and the landing's −21.686% to +36.678% ([report][report];
  [Accuracy](accuracy.md#whole-flights-against-rocketpy)). With HPR Sim's own drag, against
  RocketPy flying the drag its examples ship, HPR Sim's heights differ from RocketPy's by −7.280%
  to +10.302% ([report][report]). The larger gaps are where the two drags differ most. HPR Sim's
  drag is well below the example's for two rockets. It's above it at high speed for Prometheus
  2022, which flies through Mach 1
  ([Accuracy](accuracy.md#whole-flights-with-each-codes-own-drag)).
- **Against seven real flights, HPR Sim's apogees miss by 6.04% on average,** outside the 5%
  target, and by −8.90% to +10.40% one by one ([the register's row on these misses](evidence.md#g24)). HPR Sim's
  height is read the way each log's barometric altimeter reads the air. Four of the seven
  altimeters are assumed to be barometric ([the register's row on those altimeters](evidence.md#g21)). Drift and speed
  aren't compared yet
  ([real flights](accuracy.md#real-flights),
  [report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md)).
- **Against 53 more real flights, from a private collection, HPR Sim and OpenRocket both
  over-predict the logged apogee.** HPR Sim does by 8.73% on average and OpenRocket by 7.79%, and
  they agree with each other within 5% on 52 of them. Neither meets the 5% target. Each flight was
  rechecked against the collection's own catalog of its sources
  ([the collection rechecked](decisions-and-roadmap.md#m2-3c5)). The flights' owners allow only
  aggregate numbers
  ([private collection](accuracy.md#real-flights-of-the-private-collection),
  [report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md)).
- **The scoreboard sets HPR Sim beside OpenRocket and RocketPy, axis by axis,** each number read
  from these reports. It opens with how many of its cells are measured. Where HPR Sim and another
  tool both are, it says which is ahead, and each cell nothing measures yet names the milestone
  that will. CI fails when one of HPR Sim's numbers there gets worse, unless a decision record
  accepts it ([the scoreboard's milestone](decisions-and-roadmap.md#m0-10);
  [Accuracy](accuracy.md#the-scoreboard);
  [scoreboard](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md)).
- **The scoreboard counts the steps to a first answer:** opening a design's `.ork` file, flying it
  and getting its landing spread. HPR Sim takes 2 commands or 10 lines of Python, from files a
  test runs. Neither OpenRocket nor RocketPy does all three from the `.ork` file. OpenRocket 24.12
  opens and flies one in 7 clicks but has no Monte Carlo, and RocketPy reads no `.ork` file
  ([steps to a first answer](decisions-and-roadmap.md#m0-11);
  [Accuracy](accuracy.md#the-scoreboard)).
- **The descent under a parachute matches RocketPy's.** The comparison flies the descents of five
  of RocketPy's [example rockets](glossary.md#example-rockets) in both codes:
  - Each starts from the same state near apogee, with the first parachute opening at once.
  - Both codes get the same [drag areas](glossary.md#drag-area) and wind.
  - RocketPy's parachutes can add random noise, which would make each run differ; it's switched
    off.
  - HPR Sim uses RocketPy's formula for gravity. It also uses RocketPy's way of interpolating the
    wind, that is, of working out the wind between the heights it's given.

  Six numbers are compared for each rocket. They're the descent time, the mean descent rate, the
  descent rate at landing, and the [drift](glossary.md#drift) in total, to the east and to the
  north. All 30 agree with RocketPy's within 3%; the largest difference is +2.865%. That shows the
  two codes agree on the descent physics, not that either matches a real flight. The committed
  [validation report][report] has every number, and [Recovery](physics/recovery.md) explains the
  comparison.
- **Each model is tested on its own**: against exact answers, and against its source's tables or
  worked examples where it prints them. Several parts are also tested against RocketPy. Each test
  states its [tolerance](glossary.md#tolerance). The largest known gaps:
  - The drag was checked at Mach 0.3 against other programs' curves (below), and from Mach 0.6 to
    4.63 against NASA's Arcas Robin wind tunnel. There it reads high at most speeds, most of all
    with fins past Mach 1. 2 of 44 measurements are within the 10% target set before measuring
    ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). The normal force
    and center of pressure were checked at Mach 0 against Barrowman's worked examples. From Mach
    0.6 to 4.63, against the same wind tunnel, they miss between Mach 0.8 and 1.2, and past Mach 3.
    [Aerodynamics](physics/aero.md#normal-force-through-mach-1) reports the current comparison and
    its measured range.
  - The drag was compared with drag curves that come with RocketPy's example rockets, labeled as
    [RASAero II](glossary.md#rasaero-ii)'s (another rocket aerodynamics program). The curves don't
    record the fins' edges or the surface finish, so HPR Sim's copies of the designs follow a
    declared guess. HPR Sim is within 10% in four of the seven cases.
  - HPR Sim's drag is 18% low for Cavour, another of RocketPy's examples, while its motor burns
    ([power-on drag](glossary.md#power-on-and-power-off-drag)). The cause isn't known yet.
  - HPR Sim's drag is 47% to 50% below Valetudo's example curve. Valetudo is the rocket that
    [Getting started](getting-started.md) flies. Its references disagree with each other, though.
    At Mach 0.3 the example curve gives a [drag coefficient](glossary.md#drag-coefficient) of
    1.05, 1.44 times the 0.728 in an [OpenRocket](glossary.md#openrocket) file of the same rocket.
    Given that file's own surface finish and launch lugs, HPR Sim gives 0.714, 1.9% under the
    file's 0.728 ([Aerodynamics](physics/aero.md#drag-verification)).
  - The Recruiter is a six-fin model rocket that J. S. Barrowman works through in his 1970 report
    Centuri TIR-33. HPR Sim follows his [method](glossary.md#barrowmans-method) for the normal
    force. For the Recruiter, HPR Sim's [normal-force slope](glossary.md#normal-force-slope), how
    fast the sideways force grows with angle of attack, is 2.87% above his printed value. On the
    fins alone it's 3.42% above. Most of that comes from a different rule for six fins
    ([Aerodynamics](physics/aero.md#verification)).
  - [Tumbling](glossary.md#tumble-recovery) drag is −10 to +19% off its source's own drop tests.
    A separated body falls with no drag until its parachute opens. So the parachute can open at a
    higher speed than it would for real ([Recovery](physics/recovery.md)).
- **You can check it yourself.** [Getting started](getting-started.md#checking-it-yourself) shows
  how. Run a program that shows how much the drag moves the apogee, or trace any number with
  [Checking a claim](checking-a-claim.md). Or compare HPR Sim's apogee by hand with your own
  altimeter's or another simulator's.

## Reading these pages

New here? [Getting started](getting-started.md) builds HPR Sim and flies a first rocket.
[How a flight is simulated](how-a-flight-is-simulated.md) follows a flight from the pad to the
ground, linking the page for each model on the way.

Have an OpenRocket design? Three how-to guides take it through `hpr` on the command line:
[Fly your .ork](fly-your-ork.md), [Pick a motor](pick-a-motor.md) and
[Check stability for a certification flight](stability-for-certification.md).

Want the physics behind a number? *Rocketry explained* holds guides that explain it, each figure
drawn from HPR Sim's own results:

- [Trusting a simulation](trusting-a-simulation.md): how far a simulated apogee has been from
  real flights, and which way it errs.
- [Stability in flight](stability-in-flight.md): how the center of pressure moves with speed and
  angle, against a wind tunnel.
- [When fins flutter](when-fins-flutter.md): what a flutter speed means, and which errors flatter
  it.
- [Drag through the speed of sound](drag-through-mach-1.md): how drag climbs near Mach 1, what the
  finish adds, and which way HPR Sim's drag errs against a wind tunnel.

Each model page opens with *In short*: what it models, its sources, how well it is validated and
what it leaves out. Below that, it names the code that implements the model, the sources it follows
and the tests that pin it.

Sources are cited by a short key in square brackets, such as **[N09]** for Niskanen's 2009 thesis
on model rocket simulation. The full reference is near the top of the page.

Equations are typeset as a textbook sets them, here and on GitHub. The code's documentation writes
them in plain text. For example, once its drag balances its weight, a rocket under a parachute
falls at the steady speed

$$
v_e = \sqrt{\frac{2 m g}{\rho\, C_D S}}
$$

where $m$ is its mass, $g$ gravity, $\rho$ the air's density and
$C_D S$ the parachute's [drag area](glossary.md#drag-area)
([Recovery](physics/recovery.md#the-descent)).

Terms are defined in the [Glossary](glossary.md). [Checking a claim](checking-a-claim.md) shows
how to trace any number to its source, its test and its validation.

The code itself is documented in [the API reference](api.md), which Rust's documentation tool
generates from the source. It lists every public type and function. Each crate's front page links
back to the pages here that explain its models.

Three kinds of label link to the project's records on GitHub, which
[Decisions and the roadmap](decisions-and-roadmap.md) introduces:

- A *milestone*, such as [M1.8](decisions-and-roadmap.md#m1-8), is a step of the
  [roadmap][roadmap], the ordered plan of work.
- A *decision record*, such as [ADR-011][adr-011], explains a significant choice and the
  alternatives that were considered. All of them are in the [decision log][decisions].
- A *Loft lesson*, such as [Loft lesson L15](decisions-and-roadmap.md#l15), is a mistake found in
  Loft, the project that came before HPR Sim. A test here guards against it, or will once its
  milestone ships. They're listed in [Lessons from Loft][lessons].

Every page's source is a Markdown file in the repository's
[`docs/` folder](https://github.com/nrdptel/fusionspace-eridanus/tree/main/docs). The pencil icon
at the top of a page opens its source in GitHub's editor, where you can propose a fix.

Each page opens with HPR Sim's number, `FS-ACHERNAR · SW · TOOL 001`, in a small tag over its
title. The [title block](#title-block) says what it means. Beside the tag, a chip gives the site's
status from the title block. It reads IN PREPARATION while the pages describe work after the last
release, and RELEASED when they describe a release as it shipped. Under the title, one sentence in
larger gray type says what the page covers. The sections are numbered as the sheets of a drawing
set: `SHEET 2 / 5` over a section's name makes it the second of five.

The theme switch, Auto, Light and Dark, sits in the header before the print and edit buttons on a
window 960 px wide or wider. On a narrower one it's at the top of the table of contents, and on a
phone it's behind the menu button. Auto follows your system's light or dark setting. A choice you
make is kept in your browser, for this site only.

A page prints in the light theme's colors, whichever theme you read it in, without the table of
contents or the buttons. Each link's address follows it in parentheses. For another site that's
the whole address, and for a page of this one it's the path from the page, such as
`cli.html#units`. The [title block](glossary.md#title-block), with the version and date, closes the
page. The printer icon at the top opens a cover with a *Print the guide* button, which prints every
chapter one after another. That runs long, since it holds every chapter and its links' addresses.
Printed to a PDF file, the guide reads offline.

## Title block

Every page of this site ends with a title block. It's the box in an engineering drawing's corner
that says what a document is, who issued it, when, and which sources it rests on. Its fields follow
ISO 7200, the standard for drawings' title blocks. This page's is at its foot, below the page
arrows. Its entries are written once, at the end of this section in the page's source. They
describe the whole site and this version of HPR Sim, and each page adds its own title, as "Page",
after the site's. FusionSpace, the family of tools HPR Sim belongs to, closes its pages this way.
`FS-ACHERNAR · SW · TOOL 001` is HPR Sim's number in that family's register: Achernar, the
simulator's internal name, then software tool 1.

<div class="title-block">

| Field | Entry |
|---|---|
| Owner | FusionSpace |
| Title | FusionSpace HPR · Sim: a flight simulator for hobby and high-power rockets, and its documentation |
| Designation | `FS-ACHERNAR · SW · TOOL 001` |
| Version | 0.1.0, released 2026-10-08 ([what it holds](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md)); the site is built from the repository's `main` branch, which may be ahead of it |
| Date of issue | <span id="issue-date">the date of the commit the site is built from</span> |
| Status | IN PREPARATION: these pages describe the next release as the `main` branch holds it, so a page may describe what 0.1.0 lacks; how far to trust each result is in [Accuracy so far](#accuracy-so-far) |
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
