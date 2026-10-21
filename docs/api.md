# The API reference

Find which crate of FusionSpace HPR holds what, and reach each crate's rustdoc reference for every
public type, function and constant.

**The API reference documents FusionSpace HPR's code: every public type, function and constant,
generated from the source by rustdoc, Rust's documentation tool.** Use it when you write a program
with the `hpr` library, as [Getting started](getting-started.md) does. The library is pre-alpha,
and none of its interface is stable yet: any of it can change. The simplest way in is the `hpr`
crate's builder ([The builder](the-builder.md)).

**Where to read it.** On the site, the crate names below open the reference. On GitHub they lead
nowhere, because the reference is built rather than stored in the repository. Build it on your own
machine instead:

```sh
cargo doc --workspace --no-deps --open
```

That opens one crate's front page; the others are in the list of crates on the left. `cargo xtask
site` builds this whole site, with the reference beside the guide.

> **How far to trust it.** The reference is generated, not written by hand. It's built by rustdoc
> from the same source that CI, the project's automated checks, compiles on every change. So it
> describes the code as it is, for each of the 17 crates this page links. CI checks every link in
> it but a few in the vector types' documentation, which rustdoc copies from glam, the vector
> library the code uses. The reference says what the code does, not how well its numbers match
> the world. Physics items give their equation and cite their source, and the model pages here
> say how well each model is validated. Each crate's front page links back to them.

## The crates

The code is split into [crates](glossary.md#crate), Rust's packages, so that a program takes only
what it needs. The split also lets the models, which don't read or write files or use the network,
build for the web. Eleven crates hold most of the code today. The names below are the ones a
program writes in its code (`use hpr_core::…`). A published crate's package, the name crates.io
and `Cargo.toml` know it by, adds FusionSpace's prefix
([the public name](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md)).
So `hpr` is the package `fusionspace-hpr`, `hpr_core` is `fusionspace-hpr-core`, and so on.
`hpr_validate`, `hpr_py`, `hpr_ffi` and `hpr_wasm` aren't published.

| crate | what it holds | limits and open work | the guide's pages |
|---|---|---|---|
| [`hpr`](api/hpr/index.html) | One crate to depend on: the builder for environments, motors, rockets and flights, with the other crates re-exported by name ([more below](#the-builder-crate)) | One motor per built rocket | [The builder](the-builder.md), [Models of your own](custom-models.md) |
| [`hpr_core`](api/hpr_core/index.html) | Vectors and quaternions (a compact way to store a rotation), frames, the Earth's shape, distance and bearing between places, gravity and magnetic field, interpolation tables and numerical integration of functions | | [Frames](physics/frames.md), [Geodesy](physics/geodesy.md), [Gravity](physics/gravity.md), [The magnetic field](physics/magnetic.md), [Interpolation tables](physics/interpolation.md), [Adaptive quadrature](physics/quadrature.md) |
| [`hpr_atmos`](api/hpr_atmos/index.html) | The standard atmosphere, humidity, soundings, wind profiles and turbulence | A flight doesn't use the turbulence yet | [Atmosphere](physics/atmosphere.md), [Wind](physics/wind.md), [Turbulence](physics/turbulence.md) |
| [`hpr_motor`](api/hpr_motor/index.html) | Solid motors: thrust curves, mass and inertia through the burn, `.eng` and `.rse` files, and the [32 bundled curves](physics/motor.md#the-bundled-motors) | Hybrid and liquid motors, which are out of scope | [Solid motors](physics/motor.md), [`.eng` files](format/eng.md), [`.rse` files](format/rse.md) |
| [`hpr_design`](api/hpr_design/index.html) | The rocket: its tree of parts, their shapes and materials, mass properties and design checks | | [The design tree](physics/design.md), [Shapes](physics/shapes.md), [Mass properties](physics/mass.md) |
| [`hpr_format`](api/hpr_format/index.html) | The HPR design format: a design as one JSON document (`.hpr`) with its JSON Schema, read from and written to `.ork`, in a zip container with other files (`.hprz`), older versions migrated | None: [its TypeScript and Python types](decisions-and-roadmap.md#m3-3c) are done, outside it | [The HPR design format](format/hpr.md) |
| [`hpr_aero`](api/hpr_aero/index.html) | Aerodynamics: normal force, center of pressure and drag to Mach 5, tables from other programs for the drag and the normal force, and drag models of your own ([Models of your own](custom-models.md)) | Faster than sound, some shapes' methods are checked on one case or not at all, and some keep a theory that reads low past Mach 3 ([below](#aerodynamics-faster-than-sound)) | [Aerodynamics](physics/aero.md) |
| [`hpr_sim`](api/hpr_sim/index.html) | The flight: the launch rail, the equations of motion, time integration, events and recovery | How staging, air starts, clusters and parts that leave the airframe are checked ([below](#staging-clusters-and-parts-that-leave)) | [How a flight is simulated](how-a-flight-is-simulated.md), [Rigid-body flight](physics/flight.md), [Time integration](physics/integration.md), [Recovery](physics/recovery.md), [Staging](physics/staging.md) |
| [`hpr_flightdata`](api/hpr_flightdata/index.html) | Reading a flight log on its own, with no design and no simulation, and the readings taken from it ([below](#flight-logs)) | Formats no sample holds ([listed as needing samples](format/flight-logs.md#formats-that-need-samples), [evidence row G9](evidence.md#g9)); readings from a GPS track; the top acceleration from a board's three axes ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781)); the Mach number; a [90% interval on each reading](decisions-and-roadmap.md#m7-2e) | [Reading a flight log](reading-a-flight-log.md), [Flight-log readings](physics/log-readings.md), [`.pf2` files](format/pf2.md), [Flight logs](format/flight-logs.md) |
| [`hpr_validate`](api/hpr_validate/index.html) | The validation harness: cases, [whole flights against RocketPy](decisions-and-roadmap.md#m2-1b2) among them, reference data, metrics and reports | None | [Accuracy](accuracy.md), [Checking a claim](checking-a-claim.md) |
| [`hpr_py`](api/hpr_py/index.html) | The `fusionspace-hpr` Python package (`fusionspace.hpr`): the builder's environment, motor, rocket and flight from Python ([more below](#the-python-package)) | |

Six more crates are partly built or not yet begun, and each front page says what it will hold.
`hpr_ffi` and `hpr_wasm` hold no code yet; the other four already hold some:

| crate | what it holds or will hold | done and planned |
|---|---|---|
| [`hpr_io`](api/hpr_io/index.html) | Import and export of OpenRocket, RockSim and RASAero designs, export to RocketPy, OpenRocket's parts catalog, and weather and elevation files ([below](#design-and-data-files)) | Done: reading [a `.ork`](decisions-and-roadmap.md#m3-1a) and [its parts](decisions-and-roadmap.md#m3-1b), [writing one](decisions-and-roadmap.md#m3-2a), [ERA5 files](decisions-and-roadmap.md#m2-3a) and [the parts catalog](decisions-and-roadmap.md#m5-5a); [RockSim](decisions-and-roadmap.md#m3-4), [RASAero](decisions-and-roadmap.md#m3-5) and [RocketPy](decisions-and-roadmap.md#m3-6) files next |
| [`hpr_net`](api/hpr_net/index.html) | Optional online data, cached for offline use: weather, soundings, elevation and motor data ([below](#online-data)) | Done: [the cache](decisions-and-roadmap.md#m5-1), [Open-Meteo's weather](decisions-and-roadmap.md#m5-2a), [soundings](decisions-and-roadmap.md#m5-2b), [GFS and RAP](decisions-and-roadmap.md#m5-2c), [`hpr weather`](decisions-and-roadmap.md#m5-2d1), whole GRIB2 files in [complex packing](decisions-and-roadmap.md#m5-2d2) and [JPEG 2000](decisions-and-roadmap.md#m5-2d3), [elevation](decisions-and-roadmap.md#m5-3b) and [motor stock](decisions-and-roadmap.md#m5-4a) |
| [`hpr_analysis`](api/hpr_analysis/index.html) | Monte Carlo dispersion, sensitivity analysis and optimization ([below](#analysis)) | [Monte Carlo](decisions-and-roadmap.md#m6-1) done; [optimization](decisions-and-roadmap.md#m6-2) done, [EGO on six variables](decisions-and-roadmap.md#m6-2d2) too, but for [robust designs](decisions-and-roadmap.md#m6-2e); [competition rules](decisions-and-roadmap.md#m6-3) not yet |
| [`hpr_forensics`](api/hpr_forensics/index.html) | A flown flight against a simulation of it: what will differ, what that says about drag, mass, impulse and wind, and what went wrong | [Residuals and fits to a log](decisions-and-roadmap.md#m7-3) and [diagnosis](decisions-and-roadmap.md#m7-4) |
| [`hpr_ffi`](api/hpr_ffi/index.html) | A C interface, for other languages | [use from C and in the browser](decisions-and-roadmap.md#m4-4) |
| [`hpr_wasm`](api/hpr_wasm/index.html) | WebAssembly bindings, for the browser | [use from C and in the browser](decisions-and-roadmap.md#m4-4) |

The command-line tool, the package `fusionspace-hpr-cli`, is a program rather than a library, so it has no reference here.
[The command line](cli.md) documents its commands, output and exit codes.

### The builder crate

`hpr` holds the builder and re-exports the other crates by name. A rocket's parts can be made
from catalog parts ([parts from a catalog](the-builder.md#parts-from-a-catalog)). A drag model
of your own can fly in the simulator's place.
[`hpr::ork::separation`](api/hpr/ork/fn.separation.html) turns a `.ork` file's staging into a
flight's separation ([example](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs)).
The reference holds a guide too, [`hpr::guide`](api/hpr/guide/index.html).

### Aerodynamics faster than sound

Faster than sound, what `hpr_aero` flies depends on the body's shape:

- A pointed nose, its cylinder and a boattail behind them take the
  [shock-expansion method](physics/aero.md#the-body-faster-than-sound-in-a-flight). The
  boattail's share is unvalidated ([evidence row G29](evidence.md#g29)).
- A blunt or vertical tip flies the method behind a [Newtonian cap](physics/aero.md#blunt-tips),
  checked on a sphere-cone only.
- A conical flare flush with the part ahead of it flies the method too and ends the run. It's
  checked against [one measured flare](physics/aero.md#what-a-marched-flare-is-worth).
- Any other widening shape, or any step, behind the nose keeps slender-body theory, which reads
  low past Mach 3.

### Staging, clusters and parts that leave

- Staging and [air starts](glossary.md#air-start) fly. Tests check them, and so do OpenRocket's
  two-stage, cluster and air-start examples. Each flight is within 5% of OpenRocket's in apogee and
  largest speed ([a two-stage and a cluster design against OpenRocket](decisions-and-roadmap.md#m1-9c)).
  Three cluster apogees are compared with OpenRocket's flight with no parachute, since its
  parachute opened before apogee.
- A [cluster](glossary.md#cluster) flies, one mount of several tubes or one mount per motor, and
  so does a motor out. Tests check them against a hand calculation.
- A nose cone, a section or a payload can leave the airframe, optionally pushed by its charge. It
  lands on its own, under its own parachute or tumbling ([ejection](glossary.md#ejection),
  [the airframe parting](decisions-and-roadmap.md#m1-11a),
  [a charge's push](decisions-and-roadmap.md#m1-11b)). This is checked against exact answers only.

### Flight logs

`hpr_flightdata` reads a flight log on its own, with no design and no simulation, into one record.
It keeps every column but those a table pasted over a FIP sheet covers, and reads the files of a
zip archive one by one. Depend on this crate directly rather than on `hpr`, which pulls in the
simulator. It reads:

- PerfectFlite's `.pf2` and exports, and PerfectFlite's older `.pfa`.
- AltOS's CSV, its radio telemetry (`.telem`) and raw downloads (`.eeprom`), and its radio
  telemetry decoded to a table. It also reads AltOS's CSV re-saved by a spreadsheet, its formulas
  kept as text, or cut to its columns.
- The Raven's FIP CSV and the Blue Raven's, the Raven's `.FIPa` raw download, and a Raven FIP
  export pasted below no header, under a header you state.
- A Blue Raven's USB download, each line held to its CRC.
- RRC3's CSV and the RRC3's `.rff` raw download.
- Eggtimer's CSV, an Eggtimer TRS's terminal text, and an Eggtimer's download saved as RTF.
- AltimeterCloud's CSV.
- Entacore AIM's CSV, its `.xtra` files and raw memory dumps.
- AerospaceNU's SillyGoose logs, and AerospaceNU's flight computer board's CSV and its ground
  station's text.
- Silicdyne Fluctus's CSV.
- The ARTS2's interpreted data, its `.odf` and its raw CSV.
- A phone app's `launch` CSV.
- The Featherweight app's flight summaries (the logger's figures, no samples).
- The R-DAS's raw text (its counts kept as written).
- An AltiMAX altimeter's `.xls` export and its own `.ALS` file.
- A MARSA altimeter's download.
- A course's Teensy data logger, and one team's radio telemetry packets, `255:…:254`.
- GPS tracks from Featherweight's GPS CSV, KML, NMEA, Missile Works' RTx (its `.rtx` file and
  exports) and GPX (checked on invented files only, [evidence row G1](evidence.md#g1)).
- Tables from loggers no other reader knows, by the units their headers state.
- Binary records, from a raw file or a console capture, by a layout you state.
- Tables whose headers state too little, by a column mapping you state (its clock turning over,
  its rows written as lists or behind a date and time, its columns left out).

From any logger's barometric altitude or pressure it reads liftoff, apogee, the top speed,
landing, the flight time and the descent under each canopy. From an accelerometer along the
rocket's axis it reads the top acceleration and burnout. Each reading comes within a stated
bound, or is withheld with the reason. A top acceleration that clipped, or whose thrust the
accelerometer shows no end to, is a [floor](glossary.md#floor). `Clip` says why it clipped: at the
part's range, or held flat at its top.

### The Python package

`hpr_py` builds the builder's environment, motor, rocket and flight from Python. It flies
[a drag table and RocketPy's example flight](decisions-and-roadmap.md#m4-3b) and takes
[a drag and a wind written in Python](decisions-and-roadmap.md#m4-3c). It reads designs from
files and returns a flight's recording as NumPy arrays. Maturin builds it into one wheel per
operating system, and it's on PyPI as `fusionspace-hpr`.

### Design and data files

`hpr_io` reads and writes design files, and reads weather and elevation files:

- OpenRocket's parts catalog: its 16 `.orc` files are bundled, every part found by maker and part
  number ([the format page](format/orc.md)).
- A `.ork` file's container, design document and components are read into a design
  ([the format page](format/ork.md)). So are its motors, when each lights, and its recovery
  settings.
- A design is written back out as a `.ork` with `hpr_io::ork::export`
  ([writing a `.ork`](format/ork.md#writing-a-ork-back-out)).
- An ERA5 weather file (netCDF classic) gives the atmosphere over a launch site at launch time
  ([ERA5 weather files](format/era5.md)).
- A GeoTIFF elevation file gives a site's height
  ([A launch site's elevation](elevation.md#from-an-elevation-file-of-your-own)).

Each powered separation in a `.ork` comes out as a
[`Staging`](api/hpr_io/ork/staging/struct.Staging.html), from the tail forward
(`MotorConfiguration::stagings`). A powered separation is one whose time is known before the
flight. At that time a motor ahead of it is still burning or yet to light, and none behind it
is. Any other separation that could come before apogee is refused, a sustainer already burnt out
at the split included. The configuration's rocket doesn't carry the `Staging`. These functions in
`hpr` carry it, and the recovery settings, into the flight:

- [`hpr::ork::separation`](api/hpr/ork/fn.separation.html) turns one into the flight's
  separation, and [`hpr::ork::separations`](api/hpr/ork/fn.separations.html) a configuration's
  list. You pass the list to the flight yourself, with a recovery device on each part, since the
  simulator refuses the flight without them ([example](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs)).
- [`hpr::ork::recovery`](api/hpr/ork/fn.recovery.html) turns a configuration's parachutes and
  streamers into the flight's recovery devices, as OpenRocket flies them
  ([Recovery](physics/recovery.md#from-an-openrocket-file)).
- [`hpr::ork::separated_recovery`](api/hpr/ork/fn.separated_recovery.html) puts each on its part
  when the configuration separates, as `hpr sim` flies it.

### Online data

`hpr_net`'s cache, offline mode and HTTP work today; HTTPS is checked by hand but not in CI
([Online data and the cache](online-data.md)). It has five sources:

- Open-Meteo's weather ([Launch-day weather](weather.md)).
- Weather-balloon soundings ([Weather-balloon soundings](soundings.md)).
- NOAA's GFS and RAP forecasts ([NOAA forecasts: GFS and RAP](nomads.md)).
- Open-Meteo's ground elevation ([A launch site's elevation](elevation.md)).
- motor.fusionspace.co's motor stock and prices ([Motor stock and prices](motor-stock.md)).

### Analysis

`hpr_analysis` holds:

- Monte Carlo dispersion (flying many copies of a flight with randomly scattered inputs), the
  spread of its results, and landing ellipses ([Monte Carlo dispersion](monte-carlo.md)).
- Sensitivity analysis, Morris screening and Sobol' indices
  ([Sensitivity analysis](sensitivity.md)).
- Optimization ([Optimization](optimization.md)): CMA-ES over numbers and choices such as a
  motor, within limits such as a minimum margin; NSGA-II for two goals at once; and
  [EGO](optimization.md#few-evaluations-ego) (efficient global optimization) for models too slow
  for more than tens of evaluations. On a rocket, its answers are only as good as the simulator's
  flight models.
- Planned: [competition rule files](decisions-and-roadmap.md#m6-3).

## Using it from your own program

Release 0.1.0 is on crates.io, Rust's public package registry. `cargo add fusionspace-hpr` adds
the front door, which your code calls `hpr` and which re-exports the crates under it. Its features
`net`, `parquet` and `parallel` add the online data sources, Parquet export and Monte Carlo on
worker threads ([Install a release](getting-started.md#install-a-release)). Until 1.0, a
new minor version, such as 0.2, may change the API.

To follow `main` instead of a release, a program outside this repository can depend on a crate
straight from GitHub. Pin it to a commit, since anything can change between commits. Take its hash from
[the history of `main`](https://github.com/nrdptel/fusionspace-eridanus/commits/main), and change it only on
purpose:

```toml
[dependencies]
fusionspace-hpr = { git = "https://github.com/nrdptel/fusionspace-eridanus", rev = "<commit>" }
```

Cargo finds the crate in the repository by its package name, `fusionspace-hpr`; your code still
says `use hpr::…`.

## Reading it

- **Names.** Code names a crate with underscores (`use hpr_sim::Simulation;`), and `Cargo.toml`
  and `cargo -p` name its package with hyphens and the `fusionspace-` prefix
  (`fusionspace-hpr-sim`). They're the same crate.
- **Search.** The search box at the top of every reference page, or the **S** key, finds a type or
  function in any crate.
- **Source.** Each item's **Source** link shows the code it documents.
- **Links to the guide** on the crates' front pages go to the published site,
  <https://hpr.fusionspace.co/>; the same pages are in the repository's
  [`docs/` folder](https://github.com/nrdptel/fusionspace-eridanus/tree/main/docs).

