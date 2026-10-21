# The command line

This reference shows what each `hpr` command does and prints, and which exit codes it returns, for
use from a terminal or a script.

`hpr` is HPR Sim's command-line tool. Today it does seven things:

- It flies a design, read from an OpenRocket file or a
  [design file](glossary.md#design-file) in the HPR design format, and prints how the flight went.
- It flies a design many times, its uncertain inputs scattered at random
  ([Monte Carlo](monte-carlo.md)), and prints how far its apogee and landing spread.
- It looks up motors in the catalog built into it or in a motor file of your own. It searches
  vendors' stock and prices from [motor.fusionspace.co](https://motor.fusionspace.co).
- It converts motor files between the two common formats. It converts designs between OpenRocket's
  `.ork` and [the HPR design format](format/hpr.md) (`.hpr`, and `.hprz` with other files beside the
  design).
- It reads a flight log from an altimeter and prints what it says about the flight, with no design
  file and no simulation.
- It fetches a launch day's weather, or reads a weather file, and writes the air and wind over the
  site as a profile.
- It writes shell completion scripts.

Its other commands are registered but not available yet. Each refuses and names the
[milestone](glossary.md#milestone), the step of the roadmap, that brings it
([the table below](#the-commands)).

> **How far to trust it.**
>
> - `hpr sim` prints what the library computes. A
>   [test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-cli/tests/cli.rs)
>   flies the rocket of [the example below](#flying-a-design) through `hpr sim` and through the Rust
>   library. Both give identical numbers, to the last bit. How close those are to a real flight is
>   the [Accuracy](accuracy.md) page's subject.
> - `hpr sim` flies a `.ork` file's parachutes and streamers as OpenRocket flies them. Its landing
>   speed is within 0.02% of OpenRocket's on 50 of its 53 example flights, and within 0.12% on all
>   53 ([Recovery](physics/recovery.md#from-an-openrocket-file)). It flies a
>   [separation](glossary.md#separation) under power, or several in turn as on a three-stage rocket.
>   A dropped stage's own flight is not validated ([evidence row G13](evidence.md#g13)). With no
>   device open within 1 s of apogee, where and how fast its rocket comes down are not predictions
>   ([what it leaves out](#what-hpr-sim-doesnt-fly-yet)).
> - `hpr motors show` works out each figure from the motor's
>   [thrust curve](glossary.md#thrust-curve) with the same code a flight uses. On all 32 bundled
>   curves, that code matches ThrustCurve.org's own statistics code to 1.8e-15, relative
>   ([Solid motors](physics/motor.md#validation)). `hpr motors list` gives the same figures for
>   the bundled motors.
> - `hpr convert` keeps the thrust curve, the size and the masses. Each of the 32 bundled motor
>   files, converted to the other format and back, reads as before in the simulator. That holds bit
>   for bit, except two masses written with 17 digits, each flagged by a warning
>   ([test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-motor/src/convert.rs)).
> - OpenRocket 24.12 opens all 32 converted files and reads 29 as it reads the originals. The other
>   three differ only in the motor's type or a delay ([other programs](#what-a-conversion-keeps)).
>   Whether RockSim opens them is not checked.
> - `hpr` doesn't re-run the validation cases. To run the check the project's automated tests
>   make on every change, clone the repository and run `cargo xtask validate --check`
>   ([what it checks](accuracy.md#the-census)).
> - `hpr weather` runs the library's readers and writes the profiles they build, to the last bit
>   ([how far to trust it](#hpr-weather)). Its online fetch is not tested automatically.
> - `hpr motors search` gives back motor.fusionspace.co's values unchanged
>   ([how far to trust it](#motors-you-can-buy)). Its online fetch is not tested automatically.
> - The tests in
>   [`crates/hpr-cli/tests/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/crates/hpr-cli/tests)
>   run every command as a user would, and check each `--json` document against its
>   [published schema](#json-output).

## Running it

Release 0.1 has downloads for Linux and Windows on x86-64 and for Macs with Apple silicon
([Install a release](getting-started.md#install-a-release)). To run `hpr` from a copy of the
repository instead, set it up as [Getting started](getting-started.md#build-it-and-fly) shows,
then run it through Cargo:

```bash
cargo run -p fusionspace-hpr-cli -- motors list
```

Or install it once, so that `hpr` works from any directory:

```bash
cargo install --path crates/hpr-cli --locked
hpr --help
```

`hpr --help` says what the tool does, then how to call it, and its commands and options. Two or
three examples of real use follow, each with what it does, and last the address of this page. Every
command's `--help` has the same parts. It ends with the address of its own section of this page:
`hpr sim --help` ends with the one on `hpr sim` below.

`-h` is the short version: one line per option, and the examples' command lines without the words
about them. A test parses every example as `hpr` does, so none names an option that isn't there. It
doesn't run them.

`hpr --version` prints its version and its designation, `FS-ACHERNAR · SW · TOOL 001`. Achernar is
the simulator's internal name among FusionSpace's products. The rest is its number as a software
tool
([project Eridanus](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md)).

FusionSpace is the family of programs whose design rules the simulator follows
([the product system](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md)).
It named its products after the stars of project Eridanus in October 2026
([the new designation](decisions-and-roadmap.md#m10-1d7)). Before that, the simulator wrote
`FS · SW · TOOL 005`, and files stamped that way still read.

### The accuracy note on every result

`hpr sim` and `hpr mc` end their text result, and the plot, with a three-part trust note. It says
what kind of figure this is, what the apogee was checked against, with numbers, and what to rely on
instead. The note is on standard output, so `> flight.txt` keeps it. `--json` carries it as `trust`,
with `kind` set to `simulated`. So do the files `--export` writes: a JSON recording, and the sidecar
beside a CSV
([the decision](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0211-how-far-to-trust-a-result.md)).

`hpr weather`, `hpr analyze` and `hpr motors show` end the same way. Each note has the same three
parts and a link to the page that says more. `kind` names what their figures are:

- `forecast` for Open-Meteo, GFS and RAP;
- `measured` for a Wyoming balloon sounding and a flight log;
- `reanalysis` for an ERA5 file;
- `copied` for a motor's figures, taken from its curve file or computed from it.

Where nothing has compared a figure with a measurement of what it stands for, the note says "not yet
checked"
([the decision](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0213-units-and-trust-notes-on-every-readout.md)).
A forecast's winds are one such figure: nothing has compared them with measured ones.

The numbers cover the apogee only, on 53 logged flights of fliers' own designs. Each logged apogee
is the height climbed in the day's air. The simulated apogee averaged 8.7% above the logged one. It
was within 10% on 26 and read low on 13, by at most 20%
([Accuracy: real flights of the private collection](accuracy.md#real-flights-of-the-private-collection)).
Speed, drift and the margin have no measured spread yet.

The note asks for room above the apogee when planning a waiver, the permission to fly above a height
limit. It leaves the call to the RSO, the range safety officer.

### Colors and messages

The result alone goes to standard output. Every message line goes to standard error: a refusal, and
the `warning:`, `note:` and `help:` lines printed beside a result. With `--json`, standard output
gets one JSON document instead, holding the warnings in its own fields. Standard error stays empty
([JSON output](#json-output)).

Each message line starts with a word that says what it is. So it reads the same with or without
color:

| prefix | meaning | color |
|---|---|---|
| `error:` | what went wrong; the command stopped | bold red |
| `warning:` | something to check; the command went on | bold yellow |
| `note:` | something worth knowing about the result | bold |
| `help:` | what to do next, such as the option to add; in a refusal, the last lines | bold blue |

The result itself is plain text, with two marks of emphasis when its stream gets color. Its first
line, which names what was read, and each table's header row are bold. In a `help:` or `note:` line,
what to type is bold blue. That covers a command in backticks, such as `` `hpr motors list` ``, and
an option the line names, such as `--config`.

The colors are your terminal's own red, yellow and blue, so its theme decides how they look.
`hpr` decides for standard output and standard error separately, in this order:

1. `--color always` or `--color never`, if given.
2. `NO_COLOR` set to anything but an empty value: no color.
3. `FORCE_COLOR` set to anything but an empty value: color, even into a file or a pipe.
   `CLICOLOR_FORCE` does the same, set to anything but an empty value or `0`.
4. Otherwise (`--color auto`, the default) a stream gets color only when it is a terminal and
   `TERM` isn't `dumb`.

So `hpr sim rocket.ork > flight.txt` writes the flight alone to the file, in plain text. Its
`warning:`, `note:` and `help:` lines stay on the terminal, in color. `2> messages.txt` keeps those
in a file of their own. `> run.txt 2>&1` keeps the flight and its messages together, in order. A
pipe is the same: `hpr sim rocket.ork | less` pages the flight while the messages print on the
terminal.

The examples on this page show what a terminal shows: both streams, in the order `hpr` writes them.
So a `warning:` line appears among the result's lines, though it went to standard error. `--json`
output never has color, whatever the flag or the variables say. To turn color off everywhere, set
`NO_COLOR=1`. For one run, give `--color never`, which every command takes.

### Typing numbers, and what a refusal says

A number is read the way you would write it by hand. Spaces around it are dropped. A comma between
groups of three digits is a thousands separator: `--elevation 1,280` is 1280 m (4199 ft). A `note:`
line says so (`note: --elevation 1,280 read as 1280 m`) before anything else, in case the comma
meant something else.

That holds for every option that takes a number, negative ones included. `--elevation -1,280` is
−1280 m (−4199 ft). `--longitude -106,970` is read as −106970° and refused, as a longitude is at
most 180° east or west.

Any other comma is refused with a question. `3,9` is 3.9 in much of the world, but it could be a
typing slip. `0,500` is refused too, as no number grouped in thousands starts with 0. The decimal
point is always a period.

The `<M>` in the first line is the option's value, in meters. A usage error like this one exits with
[status 2](#exit-codes):

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --elevation 3,9`, exits 2 -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --elevation 3,9
error: invalid value '3,9' for '--elevation <M>'

help: a comma is read only between groups of three digits, as in 1,280: is 3,9 meant as 3.9? Write the decimal point as a period

help: for more information, try '--help'.
$ echo $?
2
```

<!-- cli: end -->

A refusal has two parts, as a compiler's does. First comes an `error:` line that says what went
wrong, named the way you typed it. That's the option with its value and unit, or the file's path.
Then come `help:` lines that say what to do, the most useful last.

Every refusal has at least one `help:` line. A command that isn't available yet (exit status 3)
names the milestone that brings it and where to follow it on the roadmap. A refusal that only a
fault in the simulator can cause asks you to report it at the issue tracker's address.

A latitude past the pole is refused in the degrees you gave, with the range `--latitude` takes:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --latitude 95`, exits 1 -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --latitude 95
error: --latitude 95: the site's latitude is in degrees, from -90 to 90, north positive and south negative
help: for example `--latitude -33.9`
$ echo $?
1
```

<!-- cli: end -->

A refusal raised by the simulator, rather than by an option, says so. It rounds its number to four
significant digits and names the options that shaped the flight, as they were read. Here an
elevation of a billion meters, chosen only to force the refusal, puts the launch site where there's
next to no air. Nothing slows the rocket, and it passes Mach 5, where the aerodynamics stop:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --elevation 1e9`, exits 1 -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --elevation 1e9
error: the flight stopped: it reached Mach 5.835; the aerodynamics stop at Mach 5 (the normal force's range)
help: the flight came from pods-none.ork with `--motor H54 --elevation 1000000000`: check those values, or fly it without them
$ echo $?
1
```

<!-- cli: end -->

When no launch option was given, the `help:` line points at the design: its motor, masses and
parts.

A file that isn't there is named with the closest name in its folder, one or two letters away in any
case. An example is `help: is it rockets/vega.ork? It is the closest name in that folder`. When the
folder isn't there either, the folder is named instead.

A motor the bundled catalog lacks is named with the catalog's motors whose designation or common
name begins with what you typed. `H5` begins `H54`, the common name of `168H54-10A`. The refusal
also gives [ThrustCurve.org](#motors-from-thrustcurveorg)'s answer: the command that fetches the
motor, or, once the cache holds the answer, the motors that match the name:

<!-- cli: example `hpr motors show H5`, exits 1 -->

```text
$ hpr motors show H5
error: no motor in the bundled catalog is called H5
help: `hpr motors list` lists the catalog's motors, and a .eng or .rse file can be shown by its path
help: with a network connection, `hpr motors fetch H5` fetches it from ThrustCurve.org, and `hpr sim --motor` flies it
help: the catalog has 168H54-10A
$ echo $?
1
```

<!-- cli: end -->

### The commands

This table is written from the tool's own list of commands. So it names only what `hpr` has, and
only the files each command really reads. "Not yet" commands exit with [status 3](#exit-codes).

<!-- cli: commands, written by `cargo xtask cli` from the registered commands; do not edit -->

| command | what it does | reads | prints | status |
|---|---|---|---|---|
| `hpr sim` | Fly a .ork, an .hpr or .hprz design, or a rocket's .json from a rail and print its flight; export its recording | `.ork`, `.hpr` or `.hprz`, a rocket's `.json`, a motor from the bundled catalog, `.eng` or `.rse`, or fetched from ThrustCurve.org | text, JSON, a recording as `.csv`, `.json`, `.parquet`, `.geojson` or `.kml` | available ([how to use it](cli.md#hpr-sim)) |
| `hpr convert` | Convert a motor file between .eng and .rse, or a catalog motor to either; or a design between .ork, .hpr and .hprz | `.eng`, `.rse`, the bundled catalog, a design as `.ork`, `.hpr` or `.hprz` | `.eng` or `.rse`, `.ork`, `.hpr` or `.hprz`, text, JSON | available ([how to use it](cli.md#hpr-convert)) |
| `hpr motors` | Look up motors in the bundled catalog, read a .eng or .rse motor file, fetch a curve from ThrustCurve.org, or search vendors' stock and prices | `.eng`, `.rse`, the bundled catalog, motor.fusionspace.co's stock and prices, fetched or saved, ThrustCurve.org's curves, fetched and cached | text, JSON | available ([how to use it](cli.md#hpr-motors)) |
| `hpr weather` | Fetch a launch day's weather, or read a weather file, as a profile of air and wind | Open-Meteo, a University of Wyoming sounding, GFS or RAP, fetched or saved, a whole GFS file, an ERA5 `.nc` | text, JSON, a profile as `.json` | available ([how to use it](cli.md#hpr-weather)) |
| `hpr mc` | Fly a design many times, each flight's inputs scattered at random, and print how its apogee and landing spread; export every flight | what `hpr sim` reads: a design and its motor | text, JSON, every flight's draw and outcome as `.csv` | available ([how to use it](cli.md#hpr-mc)) |
| `hpr optimize` | Search a design's parameters for a goal | - | - | not yet: [M6.3](decisions-and-roadmap.md#m6-3) |
| `hpr compare` | Compare a flight log with its simulation | - | - | not yet: [M7.3](decisions-and-roadmap.md#m7-3) |
| `hpr analyze` | Read a flight log and print its readings, with no design file | a flight log in any layout HPR Sim reads, by its own reader, a workbook's sheet or a raw dump's recording, as chosen, another logger's table by a column mapping, binary records or a console capture by a record layout, a Raven FIP export below no header, by the header stated | text, JSON | available ([how to use it](cli.md#hpr-analyze)) |
| `hpr diagnose` | Diagnose what went wrong in a flight from its log | - | - | not yet: [M7.4](decisions-and-roadmap.md#m7-4) |
| `hpr completions` | Print a shell completion script for `hpr` | - | a bash, elvish, fish, powershell or zsh script, JSON | available ([how to use it](cli.md#hpr-completions)) |

<!-- cli: end -->

## Flying and scattering

`hpr sim` flies a design once, and `hpr mc` flies it many times with its inputs scattered.

### `hpr sim`

`hpr sim` flies a design from a launch rail to the ground and prints what happened:

- its [stability margin](glossary.md#stability-margin) as it leaves the rail;
- its [apogee](glossary.md#apogee);
- its speed off the rail;
- how its [ejection delay](glossary.md#ejection-delay) suits the climb;
- how fast it comes down;
- its [events](glossary.md#event);
- its top speed;
- where it came down.

It reads an [OpenRocket](glossary.md#openrocket) `.ork` file, or a design in
[the HPR design format](format/hpr.md): `.hpr`, or `.hprz` with its attachments. It also reads a
rocket's JSON (`.json`, the tree [Your own rocket](your-own-rocket.md) describes).

A `.hpr` or `.hprz` flies exactly as the `.ork` it was converted from
([converting a design](#converting-a-design)). It doesn't print the `.ork` reader's warnings,
though: `hpr convert` printed those.

#### Units

The text gives SI first. The US units a flyer in the United States reads follow in brackets
(parentheses): `1065.1 m (3494 ft)`. Every command that prints a height, speed, temperature, wind,
acceleration or size does this. So do the `--plot` figure and their `note:` and `warning:` lines.

A table's heading names both units, `m MSL (ft)`. Each cell gives the US figure in brackets after
the SI one, `1476.4 (4844)`:

| quantity | SI | US, in brackets |
|---|---|---|
| a height or a distance | meters to a tenth | whole feet |
| the rail's length | meters to a tenth | feet to a tenth |
| a speed | m/s to a tenth | whole ft/s |
| the wind | m/s | whole mph |
| a station along the rocket (the CG and CP) | meters to the millimeter | inches to a tenth |
| a motor's diameter and length | millimeters | inches to a hundredth and a tenth |
| an acceleration (`hpr analyze`) | m/s² to a tenth | g to a tenth |
| the air's temperature (`hpr weather`) | °C to a tenth | whole °F |
| the air's pressure (`hpr weather`) | hPa | none: weather pressure is given in hPa |
| a motor's masses | grams to a tenth | ounces to a tenth, pounds to a hundredth from one pound |
| thrust and impulse | N and N·s | none: motors are rated in them |

The plot's right-hand scales are feet, feet per second and g (standard gravity, 9.80665 m/s²). The
SI figure is the one to copy. The US one is rounded from it, to a whole foot for a height. That can
differ by a foot from converting a rounded SI figure.

The site's own prose still gives most heights and speeds in SI alone, until
[issue #400](https://github.com/nrdptel/fusionspace-eridanus/issues/400). The command output it
quotes follows the rule above.

There is no `--units` switch. Both units appear at once, so nothing needs choosing. A switch that
claimed to change units everywhere would also have to reach the exports.

Those stay SI: `--json` and the exported CSV and JSON files name each field with its unit
(`height_above_ground_m`). That's a published format that scripts read and a spreadsheet converts in
one column. The app will bring one units control for all of it
([US units in brackets](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0210-us-units-in-brackets.md),
which carries out §6 of
[the product system](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md)).

#### Flying a design

This flies one of the repository's own test rockets, a small single-stage OpenRocket design. Its
file names an AeroTech H128W, which isn't among the 32 motors built into the simulator. So
`--motor H54` puts the catalog's Cesaroni H54 in its motor mount instead:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54
pods-none (pods-none.ork)
configuration 1 of 1: [H128W-0] with --motor H54
a simulated flight, not a measurement

static margin         2.69 calibers off the rail; least 2.69 calibers, at 0.15 s, before apogee
CG and CP             0.480 m (18.9 in) and 0.641 m (25.2 in) aft of the nose tip, off the rail
apogee                846.1 m (2776 ft) above the site at 11.21 s
rail exit speed       21.3 m/s (70 ft/s)
delay                 apogee 7.71 s after burnout
descent               no recovery device opened, so the fall is not a prediction

motor: 1 × 168H54-10A (from the bundled catalog, as of 2026-09-17) in `Motor mount`, lit at launch
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file has no recovery device, so the rocket falls from apogee on its airframe alone, on aerodynamics that hold only at small angles of attack: its landing time, speed and place, and any peak it sets in the fall, are not a prediction
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.3 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.15 s      1.8 m      (6 ft)    21.3 m/s    (70 ft/s)
burnout               3.50 s    466.0 m   (1529 ft)   136.3 m/s   (447 ft/s)
apogee               11.21 s    846.1 m   (2776 ft)     0.1 m/s     (0 ft/s)
ground hit           29.71 s      0.0 m      (0 ft)    65.9 m/s   (216 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             178.3 m/s (585 ft/s) at 2.31 s
top Mach number       0.525
landing               17.4 m (57 ft) from the pad at 29.71 s, at 65.9 m/s (216 ft/s): with no recovery device opened soon after apogee, not a prediction

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, each logged apogee the height climbed in the day's air, the simulated apogee
averaged 8.7% above the logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan
a waiver or a field's ceiling with room above this apogee; the altimeter's reading is the one to
log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

What each part says:

| lines | what they say |
|---|---|
| the first two | the rocket's name and its file; the [configuration](glossary.md#configuration) flown, by its number in the file and its name ([below](#the-motor-and-the-configuration)). `[H128W-0]` is the file's unnamed configuration, called by the motor it holds, and `with --motor H54` says the H54 flew in its place. The file's other configurations follow, each with why it doesn't fly as read, if it doesn't |
| `a simulated flight, not a measurement` | that every figure below comes from a model of the flight, not from a measurement |
| `static margin` | the [static margin](glossary.md#stability-margin) as the rocket leaves the rail, in [calibers](glossary.md#caliber), at Mach 0, and its least value from there to apogee. Each is the weakest direction's: a rocket with a fin set of one or two fins has a different margin for each direction the air crosses it, and the simulator prints the least; the JSON's `roll_rad` gives that direction ([Flight metrics](physics/metrics.md#stability-margins)) |
| `CG and CP` | the stations the margin off the rail comes from: the [center of gravity](glossary.md#center-of-gravity-cg) and the [center of pressure](glossary.md#center-of-pressure-cp), each in meters and inches aft of the nose tip, as the rocket leaves the rail. The margin is their distance apart over the reference diameter. The JSON gives them as `cg_station_m` and `cp_station_m` |
| `apogee` | the [apogee](glossary.md#apogee): the height of the center of gravity above the site, and when |
| `rail exit speed` | the speed at [rail exit](glossary.md#rail-exit-and-rail-exit-velocity) |
| `delay` | the coast: the time from burnout to apogee. For one motor lit at launch, a delay no longer than that fires at or before apogee. Then the delay the design sets for its motor, if it sets one, and, for one motor lit at launch, how many seconds before or after apogee its charge fires. `--motor` sets no delay without `--delay`, so the example shows the coast alone; OpenRocket's example *A simple model rocket*, with its C6-5, prints `apogee 5.49 s after burnout; the motor's set delay: 5 s; its charge fires 0.49 s before apogee` |
| `descent` | the vertical speed under each set of open parachutes or streamers, when the next one opens and at landing, without the wind's drift. It rests on the drag area the file gives each device ([the landing](#the-landing), where an example shows it). With none open, it says so and gives no speed |
| `motor: ...` | the motor, how many of it fly (a [cluster](glossary.md#cluster)'s tubes and a [pod](glossary.md#pod) set's pods each carry one), and the motor mount it sits in, by name |
| `recovery: ...` | each parachute or streamer the file flies: its name, its event and delay, its drag area, and when it opened, such as ``recovery: `Drogue parachute` at apogee, 0.133 m² of drag area, opened at 9.12 s`` (`recovery` in the JSON, [below](#the-landing)); the example's file has none |
| `launched at ...` | the site, the rail and the wind, [below](#the-launch) |
| `note:` | what the flight leaves out of the design, and which numbers that spoils |
| `warning:` | what the simulator's `.ork` or motor-file reader accepted with a caveat, such as a part it left out, and what the [design's checks](physics/design.md#checks) found unusual but buildable, each in a sentence naming the parts; the same words about several parts of one name are printed once, with a count such as `(×2)` |
| `warning: unstable:` | the rocket is unstable while a motor burns, before apogee or the first deployment, and the `apogee` figure says it is not a prediction ([Flight metrics: unstable under power](physics/metrics.md#unstable-under-power)). Either the static margin falls below zero, or, where the simulator can give no margin, the pitching moment's slope $C_{m\alpha}$ is above zero; a flight prints one of the two. A `help:` line links that page. In the JSON, `flags` lists the first as `unstable_under_power`, its `peak` the least margin under power in calibers, which the summary also has as `min_powered_static_margin_cal`; and the second as `unstable_without_margin`, its `peak` the largest $C_{m\alpha}$ per radian where the margin is undefined under power, which the summary also has as `max_powered_moment_slope_per_rad` |
| `warning: envelope:` | a flag of the [operating envelope](VALIDATION.md#operating-envelope): the flight went faster than any public flight the simulator has been compared with a reference on (currently Mach 1.15; it rises as references are added), past the core band (Mach 2.5) or the envelope (Mach 3.5), or flew above 15° angle of attack while the air was strong enough to matter. Each says when, and a `help:` line links the page. A flag changes no number. In the JSON, `flags` lists them, each with its `flag`, the `peak` that raised it (its `value`, a Mach number or an angle in radians, with its `time_s` and height, as every peak has) and its `message` |
| `warning: drag:`, `warning: stability:`, `warning: flight:` | a known error in the simulator's drag (#18, #67, #68, #70, #72, #73 or #222), in a separated part's drag (#179 or #354, [when they warn](VALIDATION.md#the-separated-parts-warnings)), in the stability margin (#64, #87, #120, #121, #172, #325 or #326) or in the flight's path (#8, #106, #213 or #219, [when they warn](VALIDATION.md#the-flight-path-warnings)) that the flight meets, by its issue number ([the list, with each one's condition](VALIDATION.md#operating-envelope)). Each says which way its numbers lean. A flight on a drag table of your own prints no drag warning except #179 and #354, which concern a separated part and not your table, and keeps the stability and flight warnings; a normal-force table of its own prints no stability warning. In the JSON, `issues` lists them, each with its `issue` number, its `kind` (`drag`, `stability` or `flight`), `url`, the flight's `max_mach` peak, the `parts` that meet it (their ids; none for #8, #18, #68, #121, #172, #179, #219 or #354) and its `message` |
| the events | each [event](glossary.md#event)'s time, the height of the [center of gravity](glossary.md#center-of-gravity-cg) above the launch site, and the speed over the ground; the height at liftoff isn't zero, as the rocket stands on the rail |
| the last figures | the top speed and [Mach number](glossary.md#mach-number), and the landing |

The text names configurations and parts by their names, in backticks, not by the long ids an
OpenRocket file gives them. A part with no name is called by its id. `--json` prints the same flight
as data, with the ids beside the names. The fields are each configuration's `id`, `name` and `label`
(what the text calls it), and each motor's `mount` and `mount_name`.

**A missing motor is fetched, then kept: it flies offline from the cache.** A `.ork` file names its
motor but rarely carries its [thrust curve](glossary.md#thrust-curve), and the simulator's catalog
holds 32 motors. When the file embeds no curve and the catalog lacks the motor, `hpr sim` fetches
it. It finds the motor on [ThrustCurve.org](glossary.md#thrustcurveorg) by the file's manufacturer
and designation, keeps it in the simulator's cache, and flies it. A note names the curve file, who
measured it and its license ([Motors from ThrustCurve.org](#motors-from-thrustcurveorg)).

For the motors of OpenRocket's own example designs, HPR Sim takes the ThrustCurve file that holds
OpenRocket's curve. For any other motor the curve may not be the one OpenRocket flies. No such
flight has been compared with OpenRocket's own curve yet. With `--offline`, or no network, and no
copy in the cache, `hpr sim` refuses and names the command that fetches it (abridged):

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --offline
error: configuration [H128W-0] can't be flown as the file has it: no thrust curve for H128W: ...; AeroTech H128W is not in HPR Sim's cache of ThrustCurve.org, and the run is offline
help: with a network connection, `hpr motors fetch --manufacturer AeroTech H128W` fetches it into the cache
help: give a motor with --motor
```

Run that command once with a network connection, or fly the design once while online, and it
flies offline after. Or give a motor file you have: `hpr sim my-rocket.ork --motor AeroTech_H128W.eng`.

#### The landing

**A `.ork` file's parachutes and streamers fly, as OpenRocket flies them.** So do those of a `.hpr`
or `.hprz` converted from a `.ork`. `hpr sim` flies them through
[`hpr::ork::recovery`](api/hpr/ork/fn.recovery.html), so its numbers are the library's. Each opens
fully at once, the file's delay after its event. The rocket then comes down under the open devices'
drag alone.

The landing speed is within 0.02% of OpenRocket's on 50 of its 53 example flights, and within 0.12%
on all 53 ([Recovery](physics/recovery.md#from-an-openrocket-file)). Those flights are in calm air.
They check how fast and how long the rocket comes down, not where wind drifts it.

When the first device opens within 1 s of apogee, the landing is no longer marked "not a
prediction". Nor is a peak set once a device is open.

A later first opening, such as a main alone at 150 m (492 ft), keeps the mark. It stays on the
landing and on any peak in the fall before the opening. A note says how long the rocket fell first.
Where and how fast the device opens come from that fall, on the airframe alone, as below.

The JSON's `recovery` list gives each device's `name` and `body`, the part that carries it
([below](#separation)). It also gives `opens_at` (`apogee`, `altitude`, `ejection`, `launch` or
`separation`), `height_above_ground_m`, `delay_s`, `drag_area_m2` and `opened_s`.

A device the file sets to `never`, or to an ejection charge a plugged motor doesn't fire, opens
nothing. A note says so. One set to its stage's ejection charge, in a stage where no motor lights,
opens nothing too, as in OpenRocket. The note says "`Main` never opens: it opens at its stage's
ejection charge, and no motor of its stage lights". The other devices fly.

A device the simulator can't fly as written, such as one inside a part it doesn't read, is refused.
The rocket then flies with none, and a note says why.

This flies the repository's dual-deploy test rocket, a drogue at apogee and a main at 150 m
(492 ft). It flies the catalog's H54 in place of its own motor. The `descent` line gives the speed
under the drogue as the main opens, then at landing:

<!-- cli: example `hpr sim validation/fixtures/ork/loft-demo/demo-dual-deploy.ork --motor H54` -->

```text
$ hpr sim validation/fixtures/ork/loft-demo/demo-dual-deploy.ork --motor H54
Loft Demo 54mm — dual deploy (demo-dual-deploy.ork)
configuration 1 of 1: [K550W-P] with --motor H54
a simulated flight, not a measurement

static margin         6.25 calibers off the rail; least 6.25 calibers, at 0.27 s, before apogee
CG and CP             1.005 m (39.6 in) and 1.354 m (53.3 in) aft of the nose tip, off the rail
apogee                324.0 m (1063 ft) above the site at 9.12 s
rail exit speed       11.2 m/s (37 ft/s)
delay                 apogee 5.62 s after burnout
descent               14.1 m/s (46 ft/s) at 150.0 m (492 ft) under `Drogue parachute`; 4.7 m/s (15 ft/s) at landing with `Main parachute` open too

motor: 1 × 168H54-10A (from the bundled catalog, as of 2026-09-17) in `Booster / motor bay`, lit at launch
recovery: `Main parachute` at 150 m (492 ft) on the way down, 1.052 m² of drag area, opened at 22.41 s
recovery: `Drogue parachute` at apogee, 0.133 m² of drag area, opened at 9.12 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 2 recovery devices fly as OpenRocket flies them: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.6 m      (2 ft)     0.0 m/s     (0 ft/s)
rail exit             0.27 s      2.1 m      (7 ft)    11.2 m/s    (37 ft/s)
burnout               3.50 s    163.1 m    (535 ft)    59.6 m/s   (196 ft/s)
apogee                9.12 s    324.0 m   (1063 ft)     0.0 m/s     (0 ft/s)
charge                9.12 s    324.0 m   (1063 ft)     0.0 m/s     (0 ft/s)
deployment            9.12 s    324.0 m   (1063 ft)     0.0 m/s     (0 ft/s)
charge               22.41 s    150.0 m    (492 ft)    14.1 m/s    (46 ft/s)
deployment           22.41 s    150.0 m    (492 ft)    14.1 m/s    (46 ft/s)
ground hit           53.94 s      0.0 m      (0 ft)     4.7 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             65.9 m/s (216 ft/s) at 2.73 s
top Mach number       0.194
landing               0.4 m (1 ft) from the pad at 53.94 s, at 4.7 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, each logged apogee the height climbed in the day's air, the simulated apogee
averaged 8.7% above the logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan
a waiver or a field's ceiling with room above this apogee; the altimeter's reading is the one to
log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

**With no device open soon after apogee, the landing is not a prediction.** The rocket then falls
from apogee on its airframe alone, as the example's does. The simulator's aerodynamics hold only at
small [angles of attack](glossary.md#angle-of-attack), and a falling airframe turns far past them.
So where it lands, and how fast, are artifacts of the model. One test design glides tail-first far
from the pad in calm air (issue [#241](https://github.com/nrdptel/fusionspace-eridanus/issues/241)).

A top speed or Mach number set in the fall is an artifact too. A rocket that falls faster than it
climbed shows its top speed after apogee. `hpr sim` marks it "in the fall: not a prediction"
(`after_apogee` in the JSON). The ascent, up to apogee, is what to read.

#### Separation

**A `.ork` configuration whose [booster](glossary.md#booster) drops away under power flies. So does
one that drops several stages in turn, but a dropped stage's own flight is rough.** `hpr sim` reads
the staging from the file, as [`hpr::ork::separations`](api/hpr/ork/fn.separations.html) does, and
flies it with no `--motor`. A motor of your own can't be told when the stages separate, so with
`--motor` it is refused. Each parachute and streamer rides the part its stage is in
([`hpr::ork::separated_recovery`](api/hpr/ork/fn.separated_recovery.html)).

On both configurations of OpenRocket's *Two stage high power rocket*, the
[sustainer](glossary.md#sustainer)'s landing speed is +0.00% off OpenRocket's. Its flight time is
+0.32% and +0.73% off. On the first, its main opens 1.09 s before OpenRocket's. That's mostly
because its apogee is 1.79% lower and OpenRocket opens 8.9 m (29 ft) below the set height
([Staging](physics/staging.md#against-openrocket)).

After the split, each part flies as a point, with only its devices' drag. So the booster
[tumbles](glossary.md#tumble-recovery) from the split until its first own device opens. A sustainer
with no device tumbles from its apogee.

A real booster flies nose-first for a while. So its peak and landing are likely too low and too
close to the pad (issue [#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179)). A note
says so, and a map export draws no pin for it.

The sustainer, which keeps the nose, is part 0. The booster is part 1. Each device line names its
part (`on part 1`), and the booster's landing is printed as `landing, part 1`, marked rough. A
tumble the simulator adds counts as no recovery device for the landing's caveat. So a sustainer that
only tumbles lands with "not a prediction".

In the JSON, a device's `body` is `null` for the sustainer and the part's index otherwise. `added`
is true for a tumble the simulator added, and a tumble opened at the split has `opens_at`
`separation`. [The powered separation milestone](decisions-and-roadmap.md#m4-5g1) shipped this. The
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0159-a-powered-separation-in-hpr-sim.md)
says why each part needs a device.

**Several separations fly too, one after another, from the tail forward.** At each split the stages
behind it drop away. The rest flies on as a new sustainer, which must still have a motor to burn.
The parts are numbered in the order they drop.

OpenRocket's *Three stage low power rocket* is one of the example designs OpenRocket ships. Its
booster is part 1 and its middle stage part 2, and the file names both `Booster stage`. On its first
configuration, an A8-5 above two B6-0s, the note says the stack comes apart under power 2 times.
Part 1, `Booster stage`, drops away at 0.857 s, and part 2, `Booster stage`, at 1.714 s. The
sustainer, part 0, flies on. The output then prints `landing, part 1` and `landing, part 2`, each marked rough like a booster's.

- **Against OpenRocket, on its own thrust curves.** All three configurations separate at
  OpenRocket's times. Their apogees are within 1.48% and their largest speeds within 1.64% of
  OpenRocket's ([Staging](physics/staging.md#several-separations)).
- **With the curves `hpr sim` fetches.** The file holds no curves, so `hpr sim` fetches them from
  ThrustCurve.org ([Motors from ThrustCurve.org](#motors-from-thrustcurveorg)), taking the files
  that hold OpenRocket's own curves. Its largest speeds are within 1.67% of OpenRocket's. The first
  configuration reads 78.61 m/s (257.9 ft/s) against OpenRocket's 78.37 m/s (257.1 ft/s). With the
  file's parachutes, its apogees are within 1.37% of OpenRocket's own record, which opens them too.
  They're within 2.48% of OpenRocket's flight with nothing deployed.
- **Not validated:** the dropped middle stage's own flight, like the booster's (issue
  [#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179))
  ([evidence row G13](evidence.md#g13)).

[The several separations milestone](decisions-and-roadmap.md#m4-5g2) shipped this. Its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0160-several-powered-separations.md)
(several powered separations) has the numbers.

**Boosters strapped beside the core fly too**, on a rocket of one stage on the axis. They form a
[parallel stage](glossary.md#parallel-stage), which burns with the core and drops at its own
separation. On OpenRocket's *Parallel booster staging*, the boosters' E12s light with the core's
I115W at launch. The note begins "the stack comes apart under power at 2.440 s: part 1,
`Booster Set`, drops away". Boosters with no motor that lights stay on to the landing, as OpenRocket
flies them.

Both configurations' apogees are within 1.23% of OpenRocket's
([Staging: Boosters beside the core](physics/staging.md#boosters-beside-the-core)). The dropped
boosters' own flight is not validated, like a booster's ([evidence row G13](evidence.md#g13)). The
[parallel staging milestone](decisions-and-roadmap.md#m4-5m) shipped this. Its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0171-a-parallel-stage-is-a-pod-set-that-separates.md)
says how a parallel stage is read and flown.

**A booster may drop with motors still burning, when the file separates it at its first burnout.** A
stage with motors in more than one mount burns out when its first motor does, as OpenRocket flies
it. It drops the rest still burning. The dropped part flies with no thrust, so a note names the
motors and the impulse its flight leaves out.

The first configuration of OpenRocket's *Pods--powered with recovery deployment* is
`[C6-7; 2× A3-4, B6-0]`. That lists its motors by stage, sustainer first, the stages separated by
`;`. Its note begins "part 1 drops at 0.860 s with `A3`, `A3` still burning". The impulse it gives,
0.509 N·s, is the one the report's
[*Parts dropped still burning*](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#parts-dropped-still-burning)
section holds.

This sustainer is unstable, with a least margin of −4.21 calibers. OpenRocket's record flies it from
a 0.15 m (5.9 in) rod at 28.61° N with no wind. In those conditions, HPR Sim's sustainer turns over
at 2.04 s. From `hpr sim`'s defaults, a 1.5 m (4.9 ft) rail with no wind, it does not. So the apogee
printed assumes it stays upright, and why the two differ is not traced
([evidence row G17](evidence.md#g17)).

`hpr sim` warns that the rocket is unstable under power and marks that apogee as not a prediction
([Flight metrics: unstable under power](physics/metrics.md#unstable-under-power);
[#335](https://github.com/nrdptel/fusionspace-eridanus/issues/335)). The
[pods milestone](decisions-and-roadmap.md#m4-5n) shipped this
([Staging: A booster dropped still burning](physics/staging.md#a-booster-dropped-still-burning)).

**A payload dropped with nothing left to burn flies when its own parachute opens at the split.**
Some rockets drop a payload section at the booster's ejection charge, after the motor has burned
out, sometimes before apogee. From the split, each part flies as a point with only its own devices'
drag, as OpenRocket flies a descent. A payload with nothing open would coast as if in a vacuum. So
`hpr sim` flies the split only if the part that keeps the nose has its own device open by then.

A device set to `lowerstageseparation` opens at the split, its delay after. Otherwise the
configuration is refused with "the part that keeps the nose, would coast with no drag from the
separation at …". To fly it, set one of the payload's devices to open at the split. In OpenRocket,
that's its deployment event "Lower stage separation" with no delay.

The payload is part 0, and its apogee is the flight's. Its own events, such as its apogee and its
landing, follow the stack's in the events table. The note begins "the stack comes apart at 4.860 s
with nothing left to burn: part 1, `Booster`, drops away". A dropped part that climbs higher gets a
note, "part 1 peaks at … above the flight's apogee". It matters because a waiver's height is the
highest any part reaches.

On OpenRocket's *Deployable payload* and *ARC payload rocket*, all six configurations fly. Two of
the six separate before apogee. The payload's apogee is within 1.1% of OpenRocket's, and every
descent meets the targets. Each deployment is within 0.1 s of OpenRocket's or 2% of its time,
whichever is larger. The landing speed and flight time are within 5%
([Recovery](physics/recovery.md#from-an-openrocket-file)).

Each of the *Deployable payload*'s five configurations flies with two warnings. The payload and the
parachute are 25 mm (0.98 in) across in a 21 mm (0.83 in) bore, so they can't fit as drawn. Their
width sets only their own inertia
([a packed part wider than its bore](physics/design.md#a-packed-part-wider-than-its-bore)).

- **What the refusal avoids.** Coasting from the split with no drag, the *Deployable payload*'s C6-3
  payload would reach 268.8 m (882 ft). That's 4.00% above OpenRocket's 258.5 m (848 ft) for the
  same payload flying on its own airframe.
- **Not validated:** the booster's flight after the split, as above (issue
  [#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179))
  ([evidence row G13](evidence.md#g13)). Nor is a real payload whose parachute opens late, which
  would land later and farther away. OpenRocket's record holds no such flight.
- **The figure and the exports stop at the split.** `--plot` and `--export` follow the stack, which
  ends where it comes apart. After that, each part's flight is in the events and landings only, and
  a note says so.

[The unpowered separation milestone](decisions-and-roadmap.md#m4-5g3) shipped this. Its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md)
(a separation with nothing left to burn) has the numbers.

#### The launch

Every flight starts from a rail, at a site, in the
[standard atmosphere](glossary.md#standard-atmosphere). Without options, the site is at sea level at
0° N, 0° E, and the air is calm. The rail is vertical, 1.5 m (4.9 ft) long, and has no friction.
`hpr sim` doesn't read the launch conditions an OpenRocket file stores with its simulations. Give
them with these options.

**Set `--elevation` for any real field.** The air thins with height, so the same rocket flies higher
from a high site. From 1,400 m (4,593 ft), the rocket above climbs about 8% higher than from sea
level. Latitude changes gravity only a little: at 45° N its apogee moves by less than 0.2%. Set
`--latitude` and `--longitude` too before exporting a map, which is drawn where you say the pad is.

| option | what it sets | default |
|---|---|---|
| `--latitude DEG` | the site's latitude, degrees north (south is negative) | 0 |
| `--longitude DEG` | the site's longitude, degrees east (west is negative) | 0 |
| `--elevation M` | the site's height above sea level, m | 0 |
| `--rail-length M` | the rail's length, from the rocket's aft end to the rail's top, m | 1.5 m (4.9 ft) |
| `--inclination DEG` | the rail's angle above the horizon, degrees: 90 is vertical | 90 |
| `--heading DEG` | the direction the rail leans toward, clockwise from true north, degrees (add the [declination](physics/magnetic.md) to a compass reading) | 0 |
| `--wind M_S` | a wind of this speed at every height, m/s | calm |
| `--wind-from DEG` | where the wind blows from, clockwise from true north, degrees: 270 is a west wind | 0 |

OpenRocket measures its launch rod's angle from the vertical instead, so its 5° is 85 here. This
flies the same rocket from Spaceport America's field, on a rail leaning 5° into a west wind:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --latitude 32.99 --longitude -106.97 --elevation 1400 --rail-length 3 --inclination 85 --heading 270 --wind 5 --wind-from 270`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --latitude 32.99 --longitude -106.97 --elevation 1400 --rail-length 3 --inclination 85 --heading 270 --wind 5 --wind-from 270
pods-none (pods-none.ork)
configuration 1 of 1: [H128W-0] with --motor H54
a simulated flight, not a measurement

static margin         2.71 calibers off the rail; least 2.71 calibers, at 0.21 s, before apogee
CG and CP             0.478 m (18.8 in) and 0.641 m (25.2 in) aft of the nose tip, off the rail
apogee                877.4 m (2879 ft) above the site at 11.48 s
rail exit speed       30.2 m/s (99 ft/s)
delay                 apogee 7.98 s after burnout
descent               no recovery device opened, so the fall is not a prediction

motor: 1 × 168H54-10A (from the bundled catalog, as of 2026-09-17) in `Motor mount`, lit at launch
launched at 32.99000° N, 106.97000° W, 1400 m (4593 ft) above sea level, from a 3 m (9.8 ft) rail 85° above the horizon, leaning toward 270° T, in a 5 m/s (11 mph) wind from 270° T
note: the file has no recovery device, so the rocket falls from apogee on its airframe alone, on aerodynamics that hold only at small angles of attack: its landing time, speed and place, and any peak it sets in the fall, are not a prediction
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.3 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.21 s      3.3 m     (11 ft)    30.2 m/s    (99 ft/s)
burnout               3.50 s    468.6 m   (1537 ft)   144.4 m/s   (474 ft/s)
apogee               11.48 s    877.4 m   (2879 ft)    11.3 m/s    (37 ft/s)
ground hit           28.71 s      0.0 m      (0 ft)    70.3 m/s   (230 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             184.3 m/s (605 ft/s) at 2.35 s
top Mach number       0.556
landing               310.4 m (1018 ft) from the pad at 28.71 s, at 70.3 m/s (230 ft/s): with no recovery device opened soon after apogee, not a prediction

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, each logged apogee the height climbed in the day's air, the simulated apogee
averaged 8.7% above the logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan
a waiver or a field's ceiling with room above this apogee; the altimeter's reading is the one to
log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

In this flight the rocket climbs about 4% higher than in the first example, at sea level, not the
8% the altitude alone gives. The rocket turns into the wind as it climbs, and the rail already leans
that way. So its climb tips away from the vertical. The wind does most of it.

#### The motor and the configuration

- `--config NAME` flies the configuration of that number, name or id. The output's first lines
  list the file's configurations by number and name.
  - A configuration the file leaves unnamed is called by its motors and their
    [delays](glossary.md#ejection-delay) in brackets: `[C6-5]`, or `[C6-P]` for a plugged motor.
    Two motors make `[C6-5; B6-0]`. OpenRocket files usually leave them unnamed (50 of the 56 in
    OpenRocket's own examples).
  - `--config` ignores case and brackets, so `--config c6-5` takes `[C6-5]`, as does its
    number, such as `--config 4`.
  - A name two configurations share is refused, with their numbers. So is a word that is one
    configuration's number and another's id or name, with their names and ids.
  - Without `--config`, `hpr sim` flies the file's default configuration, or its only one. If it
    can't tell which, it refuses and lists them.
- `--motor NAME` flies a motor of the built-in catalog, by its
  [designation](glossary.md#motor-designation) or common name (`H54`, `168H54-10A`).
  `hpr motors list` shows the catalog. A name the catalog lacks is fetched from ThrustCurve.org
  and cached ([Motors from ThrustCurve.org](#motors-from-thrustcurveorg)), as
  `hpr motors fetch NAME` does. `--motor FILE` flies the motor in a `.eng` or `.rse` file
  ([RASP and RockSim files](glossary.md#rasp-and-rocksim-files)). Either way, the motor goes in
  the configuration's motor mount, in place of the file's motor. It lights at launch.
- `--delay S` gives `--motor`'s motor an [ejection delay](glossary.md#ejection-delay) of `S`
  seconds after burnout. `--delay P` makes it a plugged motor. Without it, the motor fires no
  ejection charge, so a parachute set to open on the charge stays shut. With it, the first line
  says `with --motor H125-CT --delay 10`. The `delay` line says how far from apogee the charge
  fires ([Pick a motor](pick-a-motor.md#choose-the-delay)).
- `--offline` takes a fetched motor from the cache alone, never the network. Setting the
  environment variable `HPR_OFFLINE=1` does the same for every command.
- `--mount NAME` says which mount `--motor` goes in, by its name or id. Use it when the design
  has several and no configuration says, or to move the motor to another. A name two mounts share
  is refused, with their ids. `--json` lists each motor's `mount` id too.
- `--accept-design-errors` flies a design whose [checks](physics/design.md#checks) find errors,
  such as a motor wider than its mount. The flight's notes then list the errors: such a rocket
  can't be built as drawn.

`hpr sim` refuses, with the reason, rather than fly something other than the design:

- with `--motor`, a configuration with motors in more than one mount, as `--motor` flies one;
- a rocket whose stages separate in a way `hpr sim` doesn't fly. Powered splits fly, one or
  several ([Separation](#separation)). These don't:
  - a split with nothing left to burn, beside another separation;
  - a lone split with nothing left to burn before apogee, with no device open on the part that
    keeps the nose;
  - a split at or after apogee, beside another separation;
  - a split that comes before the one behind it;
- a motor lit by a stage's separation, in a design that states no separation, such as a rocket's
  `.json`;
- with `--motor`, a `.ork` rocket of more than one stage, or a configuration that switches a
  stage off;
- a `.ork` rocket the simulator couldn't read exactly as written, such as one with a parallel stage
  on a rocket of several stages;
- a design whose checks find errors, unless `--accept-design-errors` is given;
- a hybrid motor in a `.rse` file, which says so: the simulator flies solid motors only. A `.eng`
  file doesn't say what kind of motor it holds, so a hybrid's is flown as a solid. Check the motor.

#### Exporting the recording

`--export FILE` writes the flight's recording: every quantity the simulator tracks, every 0.01 s and
at every event. `--interval` sets the recording interval, down to 0.001 s. The file's extension
picks the format. Repeat `--export` for several files. `hpr sim` won't write over a file it reads.

| extension | what it holds |
|---|---|
| `.csv` | one row per sample, with a header giving each column in words and its unit in brackets, such as `time [s]` |
| `.json` | the same columns and rows, each column's unit in its name (`time_s`), with the design, configuration, the catalog's date and how far to trust it |
| `.parquet` | the same, as Apache Parquet, for data tools such as pandas |
| `.geojson` | the rocket's path over the Earth, for web maps |
| `.kml` | the same path, for Google Earth |

The maps draw the whole path. They mark the landing point only when a recovery device opened.
With none open, the path after apogee and where it ends aren't predictions.

Each file names the program that wrote it, its version and its designation. A CSV file can't
without breaking a spreadsheet's reading of it. So `hpr sim` writes a small file beside it that
does, `flight.meta.json` for `flight.csv`. That file also gives the design and configuration
flown, the number of rows and the bundled motor catalog's as-of date. It holds the trust note
`hpr sim` ends with too (`kind` and `trust`, as `--json` has them).
[Exporting a flight](exporting-a-flight.md#which-program-wrote-a-file) says where each format
keeps it, and what each column and field means.

#### Plotting the flight

`--plot FILE.svg` draws the flight as one picture, to look at rather than to analyze. It shows the
altitude, the speed and the acceleration against time, in three panels over one time axis. Each
[event](glossary.md#event) is marked. The figure is always the same set, so any two flights read
alike. An SVG opens in any web browser.

The numbers behind it come from the same flight as the text. Beside the figure, `--plot` writes
the numbers it draws as a CSV file: `sim-plot.plot.csv` for `sim-plot.svg`
([The figure's data](#the-figures-data)). For every quantity the simulator tracks, use
`--export`. The name must end in `.svg` and its folder must exist.

- **Altitude:** the height of the [center of gravity](glossary.md#center-of-gravity-cg) above the
  launch site, as the events table gives it.
- **Speed:** the center of gravity's speed over the ground, whose peak is the `top speed` line.
  The thin line is its vertical speed, up positive. Under the parachutes, that's the `descent`
  line's rate.
- **Acceleration:** the size of the nose tip's acceleration, and its vertical part (the thin line,
  up positive). The nose tip is the point the simulator's equations of motion track. The `--json`
  summary's peak acceleration is taken there too. For a rocket that isn't turning, every point of
  it accelerates alike. A recovery device opens fully at once ([the landing](#the-landing)), so
  its opening shows as a sharp spike.
- **Not what an accelerometer reads.** The panel shows how the motion changes: zero on the pad,
  and in a coast about −10 m/s² vertically, gravity plus drag. An accelerometer reads about zero
  in that coast, and 9.8 m/s² on the pad.
- **Two scales:** each panel's left scale is SI, its title the quantity and its unit
  (`ALTITUDE · m AGL`). Its right scale is the US unit, at round values of its own. Those are feet
  above the ground, feet per second, and g, one standard gravity (9.80665 m/s²). Both scales
  measure the one quantity the panel draws, so the gridlines are the SI scale's alone.
- **Events:** each instant with events is a numbered balloon (a circle) above the panels, with a
  dotted line through them. The balloons sit in one row, in time order. Where two would overlap,
  as liftoff and rail exit do, the later one steps right. It points at its line with a short
  leader, a thin solid line, as a drawing's balloons do. Near the right edge they step back left
  instead, so each stays whole.
- **Panel heights:** each panel's height is chosen so its own lines' slopes can be compared. The
  eye compares slopes best near 45°. So the height is the one at which the panel's lines run at
  45° on average, each piece of line counted by its length. This is Cleveland's "banking to 45°",
  from "The Shape Parameter of a Two-Variable Graph" by W. S. Cleveland, M. E. McGill and R.
  McGill. It's in the *Journal of the American Statistical Association* 83, 1988.

  Heights stay between 120 and 360 px. A climb and a slow descent can't both run at 45°, so the
  climb draws steeper and the descent flatter. Measured on the drawn figure below, the altitude
  and speed panels reach the 360 px cap, where their lines average 39° and 38°. The acceleration
  panel is 252 px, at 45°. When every panel was 180 px, the altitude and the speed lines each
  averaged 22°.
- **Not a prediction:** the fall from apogee is hatched and labeled when no recovery device
  opens within 1 s of apogee. The hatching runs to the first opening, or to the ground. The
  summary's "not a prediction" marks follow the same rule, for the reason
  [the landing](#the-landing) gives. The example below has no hatching, as its drogue opens at
  apogee.

The figure is drawn in the chart style of the
[FusionSpace product system](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md).
In that style, magenta marks a prediction, and every line here is one. There's no legend box. A
caption under the title says once what the lines mean:

| Line | Means |
|---|---|
| Dashed magenta, thick | A quantity's size, simulated by HPR Sim |
| Dashed magenta, thin | Its vertical part, up positive, so below zero on the way down |
| Chain (long dash, dot) | The ground, in the altitude panel; the other panels' zero lines are solid |
| Dotted | An event, under its numbered balloon |
| Hatched area | A fall that is not a prediction |

The caption's first line is the figure's summary: the apogee, its time and the top speed. They're
the summary's own numbers, with feet and feet per second in parentheses. The SVG carries the same
sentence as its description, which a screen reader reads out. Numbers on the figure are written
for reading, `1,000` and a real minus sign `−20`. Copy numbers from `--export` or `--json`, which
are plain.

In the caption, the notes and the balloons' tooltips, a number and its unit are joined by a
no-break space. So a wrapped line never ends between `100.0` and `m`.

The table under the figure gives each event a row: its balloon's number and time, the altitude
then in meters and feet, and its name. Each parachute or streamer is named. Under the event table
comes the [accuracy note](#the-accuracy-note-on-every-result).

The figure then ends in a title block, as a drawing does. So a plot on paper says what it is and
where it came from. It's a box with a 2 px border, and each field is a label in capitals over its
value:

| Field | Holds |
|---|---|
| OWNER | FusionSpace |
| TITLE | The design's name, cut at 80 characters |
| TYPE | `Simulated flight` |
| DESIGNATION | The program's designation, `FS-ACHERNAR · SW · TOOL 001` |
| PROGRAM | The program and its version, `FusionSpace HPR 0.1.0` |
| UNITS | The figure's units, SI with the US scales in brackets |
| DATA | The figure's CSV file ([below](#the-figures-data)) |
| MOTOR CATALOG | The day the bundled motor catalog was downloaded, as the [exports](exporting-a-flight.md#which-program-wrote-a-file) carry it, whether or not the flight's motor came from the catalog |

The block has no date of issue: the same flight draws the same figure, byte for byte, on any day.

The text is set in Cascadia Mono where it's installed, else another fixed-width font, so the
lines keep their widths. The title is 20 px. All other text, the balloons' numbers included, is
12 px, the smallest size the FusionSpace product system allows on a screen. The caption, balloon
row, event table and title block are spaced in that system's 4 px steps. That's 16 px from line
to line and 24 px from row to row. The time axis is titled at its right end, under the last
tick, as a drawing labels an axis.

The panels sample the flight every `--interval` (0.01 s unless set), at every event, and at the
start and end of each integration step. So a jump such as a parachute's opening is drawn at its
instant and full height. Where a pixel column of the figure covers many samples, it keeps the
column's first, least, greatest and last. A long descent stays a small file, and no sampled peak
is lost.

A peak the summary prints is found between samples, so the plotted one can sit just below it. In
the example, a test holds the plotted top speed and the main's opening peak within 1% of the
summary's
([the plot's design](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0157-hpr-sim-plot.md);
[its style](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0175-the-plot-in-the-product-system.md)).

This draws the dual-deploy rocket of [the landing](#the-landing):

```text
$ hpr sim validation/fixtures/ork/loft-demo/demo-dual-deploy.ork --motor H54 --plot sim-plot.svg
```

![The dual-deploy rocket's flight in three panels against time, 0 to 60 s, every line dashed.
Altitude climbs to 324 m (1,063 ft) at 9.12 s, falls quickly under the drogue to 150 m (492 ft) at
22.41 s, then slowly under the main to the ground at 53.94 s. Speed peaks at 66 m/s (217 ft/s) near
2.7 s; the vertical speed settles at −14 m/s (−46 ft/s) under the drogue and −4.7 m/s (−15 ft/s)
under the main. Acceleration starts near 50 m/s², passes zero at top speed, sits at about −12 to
−10 m/s² vertically in the coast, and spikes to about 77 m/s² as the main opens. Six numbered
balloons: liftoff, rail exit, burnout, apogee with the drogue, the main, and the ground hit; the
table under the figure lists their nine events. A title block ends the figure.](images/sim-plot.svg)

`cargo xtask cli` draws this figure with the command above, so it shows what the current `hpr`
draws. This site shows it at its full 960 px, wider than the text, wherever the window has room.
In a narrower window it shrinks to fit, and a click shows it at full size.

##### The figure's data

Every chart in the FusionSpace product system gives its data as well as its picture. That's for
a reader who can't see the figure or wants the numbers. So `--plot sim-plot.svg` also writes
`sim-plot.plot.csv`, with one row for each point the figure draws. Its header names each column
with its unit in brackets:

```text
time [s],altitude [m AGL],speed [m/s],vertical speed [m/s],acceleration [m/s^2],vertical acceleration [m/s^2]
```

The values are SI only, with no feet, and plain, as `--export`'s CSV writes them. The vertical
speed and acceleration, the panels' thin lines, have columns of their own. The rows aren't evenly
spaced: an event or a step's edge adds a row between the `--interval` ones.

Beside the CSV, `sim-plot.plot.meta.json` names the program that wrote it, the design and
configuration, and the row count. It also gives the catalog's date and the accuracy note, as an
exported CSV's [sidecar](exporting-a-flight.md#which-program-wrote-a-file) does. The `.plot` in
the names keeps them apart from an `--export sim-plot.csv` beside them. If a file the run reads
or writes has either name, the run stops before it flies. The text result ends with a `wrote`
line for each of the three files. `--json` names the two beside the figure under `plot_data`
(`path`, `rows`, `meta`).

#### What `hpr sim` doesn't fly yet

- **A rocket's `.json` has no recovery devices.** It holds no parachute, so the rocket falls from
  apogee on its airframe alone. That fall isn't a prediction, and the output's notes say so. A
  Rust program can fly one ([Getting started](getting-started.md)). The devices of a `.ork`, or of
  a `.hpr` or `.hprz` converted from one, fly ([the landing](#the-landing)). One set to open at
  the lower stage's separation (`lowerstageseparation`) flies only on a split with nothing left to
  burn ([separation](#separation)). Otherwise it's refused.
- **Separation, beyond powered splits.** A `.ork` configuration whose
  [booster](glossary.md#booster) drops away under power flies. So does one that drops several
  stages under power in turn ([separation](#separation)). A design whose only separation can come
  only after apogee flies as one [stack](glossary.md#stage), with a note. So does a design with no
  separation stated. A payload dropped with nothing left to burn flies when its own device opens
  at the split. The rest, and a staged configuration with your own `--motor`, are refused by name
  ([the refusals](#the-motor-and-the-configuration)).
- **Real weather.** One wind at every height, in the standard atmosphere.
  [`hpr weather`](#hpr-weather) writes a launch day's profile, but `hpr sim` doesn't fly it yet
  (issue [#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265)).

### `hpr mc`

`hpr mc` flies a design many times, each flight's uncertain inputs drawn afresh about their
planned values. It prints how far the [apogee](glossary.md#apogee) and the landing spread, with
the ellipses the landings fall in. It reads whatever [`hpr sim`](#hpr-sim) reads, with the same
options for the design, the motor and the launch. It flies each flight as `hpr sim` does, a
staged `.ork`'s separations included. [Monte Carlo dispersion](monte-carlo.md) explains the
method, what each dispersion does and how to choose the numbers.

> **How far to trust it.** The run is the Rust library's: a test flies the same design through
> `MonteCarlo::run` and gets the same flights, bit for bit, on one platform. The spread is only as
> good as the standard deviations you give and the simulator's flight models. Those aren't yet
> validated against repeated real flights ([Accuracy](accuracy.md)).

A run that takes more than a tenth of a second draws its progress on standard error, when that's
a terminal. Each line is drawn over the one before and is gone when the run ends:
`flying 412 of 1000 flights [========            ] 41%, about 6 s left`. The estimate assumes
the flights left take as long as those flown.

`> runs.txt` sends only the result to the file, so the progress still shows on the terminal.
With standard error into a file or a pipe, or with `--json`, it draws nothing. The flights are
the same either way.

#### Scattering a flight

This flies the repository's small test rocket on the catalog's Cesaroni H54 200 times. It flies
in a 4 m/s (8.9 mph) wind from the west, off a rail leaned 5° into it. Each flight's mass, drag,
motor, wind and rail are drawn afresh:

<!-- cli: example `hpr mc validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --runs 200 --seed 2026 --wind 4 --wind-from 270 --inclination 85 --heading 270 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03 --burn-time-sd 0.02 --wind-sd 0.25 --wind-from-sd 15 --inclination-sd 1 --heading-sd 2` -->

```text
$ hpr mc validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --runs 200 --seed 2026 --wind 4 --wind-from 270 --inclination 85 --heading 270 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03 --burn-time-sd 0.02 --wind-sd 0.25 --wind-from-sd 15 --inclination-sd 1 --heading-sd 2
pods-none (pods-none.ork)
configuration 1 of 1: [H128W-0] with --motor H54

200 simulated flights, seed 2026: 0 failed
                        nominal     mean  std dev       5%   median      95%
apogee (m AGL)            813.9    819.4     34.4    765.0    818.8    870.8
apogee (ft AGL)            2670     2688      113     2510     2686     2857
landing distance (m)      286.9    283.2     44.8    208.8    281.8    354.5
landing distance (ft)       941      929      147      685      925     1163

200 of 200 flights landed, centered 281.8 m (925 ft) west and 2.0 m (7 ft) north of the pad
landing ellipse                   semi-major            semi-minor  heading   flights inside
50%                          52.8 m (173 ft)       32.8 m (107 ft)   089° T            48.5%
95%                         109.7 m (360 ft)       68.1 m (223 ft)   089° T            93.5%
95%, the next flight        111.1 m (365 ft)       69.0 m (226 ft)   089° T            93.5%

motor: 1 × 168H54-10A (from the bundled catalog, as of 2026-09-17) in `Motor mount`, lit at launch
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) rail 85° above the horizon, leaning toward 270° T, in a 4 m/s (9 mph) wind from 270° T
scattered, one standard deviation each: --mass-sd 0.02, --drag-sd 0.05, --impulse-sd 0.03, --burn-time-sd 0.02, --wind-sd 0.25, --wind-from-sd 15, --inclination-sd 1, --heading-sd 2
note: the file has no recovery device, so the rocket falls from apogee on its airframe alone, on aerodynamics that hold only at small angles of attack: its landing time, speed and place, and any peak it sets in the fall, are not a prediction
warning: drag, the nominal flight: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability, the nominal flight: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

How far to trust it. Simulated from the design file, not measured; the spread above comes from the
inputs' scatter alone, not from the model's error. Against 53 logged flights of fliers' own designs,
each logged apogee the height climbed in the day's air, the simulated apogee averaged 8.7% above the
logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan a waiver or a field's
ceiling with room above this apogee; the altimeter's reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

The table gives the nominal flight's figure: the flight `hpr sim` flies with the same options.
Then come the run's mean, [standard deviation](glossary.md#standard-deviation), 5th percentile,
median and 95th percentile. Each quantity is in meters, then in feet on the row under it. The
landing's center and the ellipses' axes give feet in parentheses. A failed flight counts as tried
with no value.

[Monte Carlo dispersion](monte-carlo.md#landing-ellipses) explains the landing ellipses. The
50% and 95% ellipses hold that share of a normal scatter with the run's mean and spread. The last
allows for the run's spread being only an estimate, so the next flight lands inside it 95% of the
time. `flights inside` counts the run's own landings inside each, a failed flight counted
outside. This rocket has no parachute, so its landings aren't a prediction, and the note says so.

An ellipse's `heading` is the direction of its long axis, in degrees clockwise from true north.
It's written as a map's bearing is: three digits and a `T` for true, such as `089° T`. True north
isn't the north a compass shows. Magnetic north is more than 10° away from it in much of the
western United States. Every bearing `hpr` prints is from true north, the rail's heading and the
wind's direction too. So is every one it reads ([`--heading`](#the-launch)).

#### The options

Each dispersion is one [standard deviation](glossary.md#standard-deviation), zero by default,
which leaves its input at its nominal value. With none given, every flight is the nominal one,
and a note says so. [Choosing the numbers](monte-carlo.md#choosing-the-numbers) gives values to
start from, with their sources.

| option | one standard deviation of | the library's field (in radians for an angle) |
|---|---|---|
| `--mass-sd FRACTION` | each stage's mass without motors, as a fraction of it (0.02 is 2%) | `dry_mass_sd_fraction` |
| `--cg-sd M` | each stage's center of mass along the axis, m | `cg_sd_m` |
| `--drag-sd FRACTION` | the zero-lift [drag coefficient](glossary.md#drag-coefficient) | `drag_sd_fraction` |
| `--impulse-sd FRACTION` | each motor's [total impulse](glossary.md#total-impulse), its propellant with it | `impulse_sd_fraction` |
| `--burn-time-sd FRACTION` | each motor's burn time, at the same impulse | `burn_time_sd_fraction` |
| `--delay-sd S` | each motor's [ejection delay](glossary.md#ejection-delay), s | `ejection_delay_sd_s` |
| `--wind-sd FRACTION` | the wind's speed at every height | `wind_speed_sd_fraction` |
| `--wind-from-sd DEG` | the wind's direction, about `--wind-from` | `wind_heading_sd_rad` |
| `--inclination-sd DEG` | the rail's angle above the horizon | `rail_elevation_sd_rad` |
| `--heading-sd DEG` | the rail's heading | `rail_azimuth_sd_rad` |
| `--deployment-lag-sd S` | each recovery device's lag after its trigger, s | `deployment_lag_sd_s` |

`--runs N` sets how many flights, 1 to 100,000 (100 by default). `--seed N` picks the random draws
(0 by default). The same seed, design and options give the same flights, bit for bit, on one
platform, whatever the machine's number of cores. Before it flies, `hpr mc` refuses a negative
standard deviation, a run of no flights, and an export that isn't a `.csv`.

#### Exporting every flight

`--export runs.csv` writes one row a flight, in index order: what it drew, then what its flight
came to. Each number is written in the shortest form that reads back to the same bits. So a
spreadsheet or a script reading the file gets the run's own numbers. A number a flight doesn't
have is an empty cell.

Beside the CSV, `runs.meta.json` names the program that wrote it, the design, the configuration
and the seed. It also gives the number of flights, the dispersion, the bundled motor catalog's
as-of date, and the trust note `hpr mc` ends with (`kind` and `trust`). The CSV's header gives
each column in words with its unit in brackets, `apogee [m]` for `apogee_m`, as Python's
`MonteCarlo.to_csv` does. The table below gives the columns by the names Python's `run[...]`
takes:

| columns | what they hold |
|---|---|
| `index`, `outcome`, `failed_at`, `reason` | the flight's number from 0; `flown` or `failed`; for a failed flight, whether its draw made an impossible input (`inputs`) or the flight refused it (`flight`), and the error |
| `drag_scale`, `wind_speed_scale`, `wind_turn_rad`, `rail_elevation_offset_rad`, `rail_azimuth_offset_rad` | the factors and offsets drawn for the drag, the wind and the rail (1 or 0 for an input not scattered) |
| `stage_N_dry_mass_scale`, `stage_N_cg_shift_m` | each stage's, counted from 1 at the nose |
| `motor_N_impulse_scale`, `motor_N_burn_time_scale`, `motor_N_ejection_delay_offset_s` | each motor's in the configuration flown |
| `device_N_deployment_lag_offset_s` | each recovery device's, a tumble's always 0 |
| `termination`, `apogee_m`, `apogee_time_s` | why the flight ended, and its apogee above the site |
| `rail_exit_speed_m_s`, `max_speed_m_s`, `max_mach`, `max_dynamic_pressure_pa`, `max_acceleration_m_s2`, `min_static_margin_cal`, `min_flight_margin_cal`, `max_angle_of_attack_rad` | the flight's peaks, as `hpr sim --json`'s `summary` has them |
| `landing_time_s`, `landing_east_m`, `landing_north_m`, `landing_distance_m`, `landing_latitude_deg`, `landing_longitude_deg`, `ground_hit_speed_m_s` | where and when it landed; after a split with nothing left to burn, where the part that keeps the nose landed |
| `part_N_landing_time_s`, `part_N_landing_east_m`, `part_N_landing_north_m`, `part_N_ground_hit_speed_m_s` | for a staged flight, each dropped part's landing |
| `flags` | the [operating envelope](VALIDATION.md#operating-envelope)'s flags the flight raised, and [`unstable_under_power` or `unstable_without_margin`](physics/metrics.md#unstable-under-power), separated by `;` |

The same table is `hpr_analysis::table::RunTable` in the Rust library.

#### What the output warns of

Failed flights are counted, never dropped. Each reason is printed once, with how many flights
failed so and the first one's index, its row in the export. The
[operating envelope](VALIDATION.md#operating-envelope)'s flags, and the flag for a rocket
[unstable under power](physics/metrics.md#unstable-under-power), are counted over the flights.
The known issues in the simulator's drag, stability and flight path are the nominal flight's, as
`hpr sim` prints them. So are `hpr sim`'s notes on the flight's own numbers, such as a fall from
apogee before a device opened, each beginning "the nominal flight:".

A staged flight can fail where its nominal flight flies, and is counted with its reason:

- A separation timed in seconds isn't scattered. So a flight whose drawn burn is longer still
  burns at it, which the simulator refuses unless the split may drop a burning motor. A
  separation at a burnout follows the drawn burn.
- A part dropped on the way up with nothing left to burn flies as a point, with only its open
  devices' drag. A separation or an ejection charge can drop it. If its device fired by the split
  but waits out a drawn lag, the part would climb through the lag with no drag at all. So the
  flight is refused.

`--json` prints all of it as one document, its schema
[`schema/cli/mc.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/mc.schema.json).

## Motors and conversion

`hpr motors` looks up, reads, fetches and searches motors. `hpr convert` turns motor and design
files from one format into another.

### `hpr motors`

The catalog built into the simulator holds 32 motors, from class B to class O, each with a
public-domain ThrustCurve.org thrust curve.
[Solid motors](physics/motor.md#the-bundled-motors) says how they were chosen. `hpr motors fetch`
fetches any other motor's curve from ThrustCurve.org into the simulator's cache, for `hpr sim` to
fly ([fetching a motor](#motors-from-thrustcurveorg)). Or download its `.eng` or `.rse` file
([RASP and RockSim files](glossary.md#rasp-and-rocksim-files)) and show that.
`hpr motors search` lists the motors vendors have in stock, with their prices
([Motors you can buy](#motors-you-can-buy)).

#### Listing the catalog

`hpr motors list` lists the catalog. Three filters narrow it, and each one given must match:

- `--class` takes an [impulse class](glossary.md#impulse-class), such as `J`.
- `--diameter` takes a casing diameter in millimeters, such as `54`, and matches within 0.5 mm (0.020 in).
- `--manufacturer` takes a maker as the `maker` column spells it (`AeroTech`, `Cesaroni`, `Loki`,
  `Estes`, `Quest`, `AMW`) or its full name (`Cesaroni Technology`), in any case.

A class or diameter the catalog has no motor for lists none, and exits with 0. A class that
doesn't exist, a diameter that isn't a positive number, or a maker the catalog doesn't know is
refused with status 1. A misspelt maker would otherwise look like a maker with no motors.

The figures are the catalog's, each worked out from the motor's public-domain ThrustCurve.org
curve file ([The bundled motors](physics/motor.md#the-bundled-motors)). The size, masses and
delays come from its header, the rest from its curve. In the `delays` column, `P` (or
`1000`) means plugged: the motor has no ejection charge
([ejection delay](glossary.md#ejection-delay)). The delays are as the header writes them, which
can be fewer than the maker sells.

<!-- cli: example `hpr motors list --class J`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors list --class J
3 motors from ThrustCurve.org data files marked public domain, downloaded 2026-09-17; size, masses and delays from each file's header, the rest from its curve.

designation   maker     class  dia mm (in)  len mm (in)  impulse N·s  avg N  burn s  delays
J450DM        AeroTech  J        54 (2.13)   359 (14.1)       1061.6  465.6    2.28  14
1266J760-19A  Cesaroni  J        54 (2.13)   329 (13.0)       1267.3  758.2    1.67  8-10-12-14-16-18-19
J300LR        Loki      J        54 (2.13)   327 (12.9)       1212.8  297.0    4.08  P
```

<!-- cli: end -->

#### A motor's figures

`hpr motors show` takes a motor's name or a file's path. A name can be the full
[designation](glossary.md#motor-designation) (`1266J760-19A`) or the common name (`J760`), in any
case, with or without spaces and hyphens. When several motors share a name, it shows each of them.
An argument ending in `.eng` or `.rse` is read as a file. Anything else is looked up by name.

It works each figure out from the thrust curve, the list of time and thrust points in the motor's
file:

- The [total impulse](glossary.md#total-impulse) is the area under the curve, with its points
  joined by straight lines. It sets the [impulse class](glossary.md#impulse-class).
- The [burn time](glossary.md#burn-time) is measured the [NFPA 1125](glossary.md#nfpa-1125) way.
  It runs from when the thrust first reaches 5% of its peak to when it last falls back to it.
- The [average thrust](glossary.md#average-thrust) is the total impulse divided by that burn time.
- The masses and the casing size are the catalog's, or the file's for a file, with ounces or
  pounds and inches in parentheses.
- The impulse class comes with its range of total impulse, written as NFPA 1125 and 1127 write
  it. Each class runs from just above the one below's top to twice it, such as
  J (640.01–1280 N·s), the lower end exclusive. Classes past O, beyond the standards' table,
  carry on the doubling.
- The designation is read for what it names, as the maker writes it. `H170M` is class H, a
  nominal average thrust of 170 N, and propellant M.
- Where the measured average thrust, rounded to a whole newton, differs from the nominal one, both
  are shown.
- The letters joined to the thrust are the maker's code for the propellant. For a catalog motor,
  the propellant's name is the one ThrustCurve.org gives. A file names no propellant, so for a
  file the code is shown alone. Estes's and Quest's joined letters aren't a propellant, so none
  is read for them. The `T` of `A10T` marks the 13 mm case of a mini motor.
- Letters after a hyphen aren't read as the propellant. The same place holds a delay for one
  maker (AeroTech's `J350W-L`) and a propellant for another (Loki's `H125-CT`). The exception is
  the form Cesaroni's own files write, `131-G84-GR-10A`. Even there, a single delay letter, as in
  `131-G84-P` (plugged), isn't read.

For a catalog motor, the last line names the public-domain curve file the figures come from.
`hpr motors list` gives the same figures, as a
[test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-cli/tests/cli.rs)
checks. ThrustCurve.org's own published figures aren't bundled: it grants no license for them
and reserves copyright ([issue #295](https://github.com/nrdptel/fusionspace-eridanus/issues/295)).
When the 32 were chosen, their total impulse, average thrust and burn time agreed with
ThrustCurve.org's within 1%. The simulator's peak, the curve file's highest point, runs from
16.7% below ThrustCurve.org's (Cesaroni 26E31-15A) to 2.1% above (Loki M1378LR) ([Solid
motors](physics/motor.md#the-bundled-motors)).

<!-- cli: example `hpr motors show J760`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors show J760
1266J760-19A (Cesaroni Technology), from the bundled catalog
  impulse class    J (640.01–1280 N·s)
  total impulse    1267.3 N·s
  average thrust   758.2 N measured; 760 N nominal, as the designation names it
  peak thrust      938.6 N
  burn time        1.67 s, from 0.001 s to 1.672 s (NFPA 1125, 5% of peak)
  propellant       White Thunder, as ThrustCurve.org names it
  propellant mass  576.0 g (1.27 lb) of 1076.8 g (2.37 lb) loaded
  casing           54 mm (2.13 in) across, 329 mm (13.0 in) long
  delays           8 s, 10 s, 12 s, 14 s, 16 s, 18 s, 19 s
  curve file       https://www.thrustcurve.org/simfiles/5f4294d20002e9000000074a/ (public domain)

How far to trust it. Copied from ThrustCurve.org's curve file, downloaded 2026-09-17, not measured
by HPR Sim: the size, masses and delays as the file's header gives them, the impulse, thrusts and
burn time computed from its curve, which can differ from the maker's rated figures. The total
impulse and peak thrust match OpenRocket's reading of the same file on every bundled curve, a check
of the arithmetic, not of the motor; no figure is checked against the maker's or the certifying
bodies' data. The motor's printed data and its maker's instructions come first, and the RSO decides.
More: https://hpr.fusionspace.co/pick-a-motor.html
```

<!-- cli: end -->

A motor file shows every motor in it, as one `.eng` or `.rse` file can hold several. The
repository holds the 32 catalog motors' files to try, such as:

```bash
hpr motors show crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000724.eng
```

Some motor data is ambiguous. A delay of `0` can mean an ejection charge at burnout, or a plugged
motor with no charge. The simulator shows it as "0 (at burnout, or plugged)" and ends the output
with a warning. The Estes F15 in the catalog is one example. The warning's "RASP spec" is the `.eng`
format's description ([RASP and RockSim files](glossary.md#rasp-and-rocksim-files)).

#### Motors you can buy

`hpr motors search` lists the motors that U.S. vendors carry, with who has each one in stock and
at what price. The list comes from [motor.fusionspace.co](https://motor.fusionspace.co), a free
site that reads a dozen vendors' public listings every hour. It covers AeroTech, Cesaroni and Loki
motors of class D and up. [Motor stock and prices](motor-stock.md) says what it publishes and how
the simulator reads it. Five filters narrow the list, and each one given must match:

- `--in-stock` keeps the motors at least one vendor has in stock.
- `--class` takes an [impulse class](glossary.md#impulse-class), such as `L`.
- `--diameter` takes a diameter in millimeters, such as `54`, and matches within 0.5 mm (0.020 in).
- `--manufacturer` takes `AeroTech`, `Cesaroni` or `Loki`, or the full name the site uses
  (`Cesaroni Technology`, `Loki Research`), in any case.
- `--max-price` takes U.S. dollars, such as `150` or `149.99`. It keeps the motors in stock whose
  price for one motor, at the cheapest vendor with it in stock, is at most that. A motor sold in
  a pack counts at the pack's price divided by its size. The site works that out, to the cent
  ([What the site publishes](motor-stock.md#what-the-site-publishes)). `--max-price` never keeps
  a motor with no such price. A motor out of stock has none. Nor does one whose cheapest vendor
  shows no price, or prices it in another currency than U.S. dollars. Every vendor's price was in
  dollars on the recording.

The motors come cheapest first, by that price. Those with no such price, including every motor
out of stock, come last, by maker and designation.

A filter that can't match anything real is refused with status 1, as `hpr motors list` refuses
one. That's a class that doesn't exist, a maker the site doesn't cover, or a price that isn't
dollars and cents. A list with nothing in it exits with 0. When `--max-price` leaves nothing, the
last line names the cheapest motor in stock that the other filters keep, and its price. So you
know what one costs.

The list is fetched and saved as `hpr weather`'s answers are
([Online, offline, and saved answers](#online-offline-and-saved-answers)). The first search
fetches it over HTTPS and keeps a copy in the simulator's cache folder. A search within the hour
reads the copy, and `--offline` reads the copy however old it is. `--from FILE` reads a list
saved earlier, the site's `motors.json` or `in-stock.json`. It touches neither the network nor
the cache.

`--in-stock` and `--max-price` keep only motors in stock, so with either the simulator fetches
the site's list of motors in stock (about 1 MB). Otherwise it fetches the whole list (about
1.6 MB). The two are saved separately. Offline, a search of motors in stock reads the whole
list's copy when it has no copy of the in-stock one. A search of every motor needs the whole
list's copy.

Every list, even an empty one, carries two credit lines under its first two lines. The first is
the site's, "Motor stock data from motor.fusionspace.co", which its data license (CC BY 4.0)
asks for. It comes with the site's caution to check stock and price on the vendor's own page
before relying on them.

The second is ThrustCurve.org's, since the motors' figures (impulse, average thrust, burn time)
are its published values. The site repeats them ([Credit and terms](motor-stock.md#credit-and-terms)).
The JSON output carries both in `attribution`. Keep them wherever you show the list.

This reads the list of motors in stock that the tests use, recorded at 07:07 UTC on 1 October
2026. It keeps the L motors in stock at up to \$300 each:

<!-- cli: example `hpr motors search --in-stock --class L --max-price 300 --from crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors search --in-stock --class L --max-price 300 --from crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json
4 motors (in stock, class L, at most $300.00 each) in motor.fusionspace.co's list built 2026-10-01T07:07:29Z.
Read from motor-finder-in-stock.json
Motor stock data from motor.fusionspace.co, licensed CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/); aggregated from public vendor listings and ThrustCurve.org; provided as is, with no warranty: check stock and price on the vendor's own page before relying on them
Motor data and thrust curves courtesy of ThrustCurve.org, https://www.thrustcurve.org/

designation  maker     class  dia mm (in)  impulse N·s   avg N  burn s  each $  pack  vendor
L1520T       AeroTech  L        75 (2.95)       3715.9  1567.8    2.36  260.99     1  Balsa Machining Service
L850W        AeroTech  L        75 (2.95)       3646.2   850.0    4.42  282.74     1  Sirius Rocketry
3419L645-P   Cesaroni  L        75 (2.95)       3419.8   644.8    5.30  286.36     1  Performance Hobbies
3683L851-P   Cesaroni  L        75 (2.95)       3683.2   849.1    4.34  290.39     1  Animal Motor Works
```

<!-- cli: end -->

`each $` is the price of one motor and `pack` how many come in the pack, both at the vendor named
last. A motor out of stock shows `-` for both and `out of stock` for the vendor. A vendor that
shows no price gives `-` under `each $`. A price in another currency names it, such as
`282.74 CAD`. The full product page is in the JSON output (`cheapest_in_stock.url`). At \$150,
the same recording lists nothing:

<!-- cli: example `hpr motors search --in-stock --class L --max-price 150 --from crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors search --in-stock --class L --max-price 150 --from crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json
0 motors (in stock, class L, at most $150.00 each) in motor.fusionspace.co's list built 2026-10-01T07:07:29Z.
Read from motor-finder-in-stock.json
Motor stock data from motor.fusionspace.co, licensed CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/); aggregated from public vendor listings and ThrustCurve.org; provided as is, with no warranty: check stock and price on the vendor's own page before relying on them
Motor data and thrust curves courtesy of ThrustCurve.org, https://www.thrustcurve.org/
None costs $150.00 or less: of the 20 motors in stock the other filters pass, the cheapest is AeroTech L1520T at $260.99.
```

<!-- cli: end -->

> **How far to trust it.** `hpr motors search` gives back the site's values unchanged. It reads
> the list with the same checks the library makes on every answer from the site
> ([What the simulator refuses](motor-stock.md#what-the-simulator-refuses)). The tests in
> [`crates/hpr-cli/tests/motors_search.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-cli/tests/motors_search.rs)
> run it offline on the recorded lists, from the file and from the cache. They check every motor
> it lists, and their order, against the recording's own JSON, filtered by the test itself, not by
> the simulator's code. With one L motor's price edited to \$149.99 in a copy, the
> `--max-price 150` search lists exactly that motor.
>
> Fetching online isn't tested automatically. Whether a vendor really has a motor at that price is
> the vendor's to say. A list fetched now, or read from a copy under an hour old, can be up to two
> hours behind the vendors' pages. A list read with `--offline` or `--from`, or after a failed
> fetch, is as old as the "built" time on its first line.
> See [how far to trust the stock list](motor-stock.md).

#### Motors from ThrustCurve.org

`hpr motors fetch NAME` finds a motor on [ThrustCurve.org](glossary.md#thrustcurveorg) and keeps
its curve in the simulator's cache ([Where the cache lives](online-data.md#where-the-cache-lives)).
`hpr sim` then flies it with no network. It's for a motor the simulator's 32-motor catalog lacks.
`hpr sim` fetches on its own too, so the command matters most before a trip with no signal. This
is ThrustCurve's data, as its contributors uploaded it. The simulator checks that a file makes a
motor it can fly, not that its curve is right.

- **Before a trip.** For a `.ork`, run the command `hpr sim --offline` prints for it. It names
  the motor's manufacturer as the file writes it (`hpr motors fetch --manufacturer AeroTech
  H128W`). Or fly the design once with a network connection. Either fills the cache the design
  reads offline. `hpr motors fetch H128W`, without `--manufacturer`, fills the cache for
  `hpr sim --motor H128W` instead.
- **Which motor.** `NAME` is a [designation](glossary.md#motor-designation) (`F27R/L`) or common
  name (`F27`), as ThrustCurve.org spells it. `--manufacturer` adds the maker, by name or
  abbreviation. ThrustCurve matches a designation exactly, any case. If no motor has it, the
  simulator asks for the common name. Exactly one motor must answer: `J450` names four makers'
  motors, and the refusal lists them. So give the designation or `--manufacturer`. A hybrid is
  refused, as the simulator flies solid motors only.
- **Which curve.** A motor can have several curve files. The simulator takes the first RASP
  (`.eng`) file that makes a motor it flies, ranked by who measured it. A certification test
  comes first, then the manufacturer, then a user, then a file that names no source. With none,
  it takes a RockSim (`.rse`) file the same way.

  `--file ID` takes the file of that ThrustCurve id first, when the motor has it and it reads.
  The id is the one `hpr motors fetch` prints on its `curve file` line. An offline refusal's
  command names it when the flight asks for a file. The output names the file, who measured it
  and its license, as ThrustCurve states them. It also gives ThrustCurve's credit.
- **OpenRocket's curve for a `.ork` motor.** ThrustCurve can hold several files of one motor with
  different curves. OpenRocket may fly another than the first by that order. The Estes A8's first
  certification file burns 0.536 s, and the one OpenRocket's examples fly burns 0.73 s.

  A `.ork` records which curve OpenRocket flies by a [digest](glossary.md#digest), a fingerprint
  of the curve. So for the motors of OpenRocket's example designs, `hpr sim` keeps a table from
  digest to the ThrustCurve file holding that curve. Its files are matched by comparing the
  curves (`cargo xtask ork-curves`). `hpr sim` takes that file first, and the note on the motor says so.

  A motor outside the table flies the file by the order above. Its note says it may not be the
  curve OpenRocket flies. An offline refusal's command then carries `--file`, so it fetches the
  file the flight takes. Offline, if the cache holds only some of the motor's files, a file it
  lacks is given up for the usual order's. The note then says the curve may not be OpenRocket's.
  The table is as of 2026-10-04. If ThrustCurve's files change, `cargo xtask ork-curves`
  rewrites it.
- **Offline.** `--offline`, or `HPR_OFFLINE=1`, reads the cache alone, and a copy of any age
  flies. With no copy, the command fails with exit status 1. Online, a copy stays fresh for a
  day. After that the simulator asks again, and keeps the old copy if the network fails.

A fetch with a network connection printed this on 2026-10-04, pasted by hand, not run in CI.
Its `help:` line is as the simulator writes it since
[the CLI milestone](decisions-and-roadmap.md#m0-6b), which made each hint a `help:` line. Its
case shows inches too, since [the units milestone](decisions-and-roadmap.md#m0-9c4) gave every
readout US units:

```text
$ hpr motors fetch F27R/L
F27R/L (AeroTech), from ThrustCurve.org: fetched now
  curve file       5f4294d20002e90000000372, RASP (.eng), from a user, no license stated
  case             29 × 83 mm (1.14 × 3.3 in)
help: kept in the cache: `hpr sim --motor F27R/L` flies it, offline too
Motor data and thrust curves courtesy of ThrustCurve.org, https://www.thrustcurve.org/
```

Offline with nothing cached, it says what to run:

<!-- cli: example `hpr motors fetch H128W --offline`, exits 1; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors fetch H128W --offline
error: H128W is not in HPR Sim's cache of ThrustCurve.org, and the run is offline
help: with a network connection, `hpr motors fetch H128W` fetches it into the cache
$ echo $?
1
```

<!-- cli: end -->

The simulator bundles none of these curves. ThrustCurve.org grants no license for its motor
records, and each file carries its contributor's own license.
[The decision record on fetching motors](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0154-motors-fetched-from-thrustcurve-by-name.md)
has the reasoning.

### `hpr convert`

`hpr convert` writes a motor as a RASP `.eng` file or a RockSim `.rse` file, the two formats
ThrustCurve.org offers ([RASP and RockSim files](glossary.md#rasp-and-rocksim-files)). It writes a
design as a `.ork`, `.hpr` or `.hprz` file ([converting a design](#converting-a-design)). Give it the
file to read and the file to write. The extensions pick the formats. It reads a motor file, or a
motor of the bundled catalog by its name:

```bash
hpr convert H170M H170M.eng
```

A catalog motor is written with the size and masses the catalog gives, the ones the simulator
flies. Where its curve file's header says otherwise, a warning says so. Written as `.rse`, the
figures worked out from them are rescaled to match, and a warning says that too. Those are the mass
fraction, the specific impulse, and the mass and center of gravity at each point. The delays are
the curve file's, which can differ from the ones `hpr motors show` lists.

After the line naming the program, the file says where the curve came from. The line is "Copied
from the bundled motor catalog as of" the catalog's date, written YYYY-MM-DD, "the day its curve
files were downloaded". It's a `;` comment in `.eng` and an XML comment in `.rse`. A motor read
from a file gets no such line.

Converted again, a `.eng` file keeps the line as one of its comments. A `.rse` file's reader skips
XML comments, so it isn't carried on. An existing file of the output's name is replaced, but never
the file being read. To rewrite a `.eng` file in the simulator's layout, convert it to a new `.eng`
file name. Here the Estes F15's `.rse` file, from the catalog's curves, becomes a `.eng` file:

<!-- cli: example `hpr convert crates/hpr-motor/data/thrustcurve/curves/5f923edb1bca5800041716ab.rse F15.eng`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr convert crates/hpr-motor/data/thrustcurve/curves/5f923edb1bca5800041716ab.rse F15.eng
read   5f923edb1bca5800041716ab.rse
wrote  F15.eng: F15
warning: line 3: engine "F15": delay 0 in "0,4,6,8" is ambiguous: the RASP spec means an ejection charge with no delay, but files mostly mean plugged
warning: F15: a .eng maker is one word, so "Estes Industries, Inc." is written "Estes_Industries,_Inc."
warning: F15: dropped the comments' blank lines and the spaces that end their lines: a .eng comment is one line of text
warning: F15: dropped Type, auto-calc-mass, auto-calc-cg, avgThrust, peakThrust, throatDia, exitDia, Itot, burn-time, massFrac, Isp, m, cg: a .eng file has no place for them. HPR Sim doesn't use them for a solid motor: it works the mass and center of gravity out from the curve and the masses
```

<!-- cli: end -->

#### What a conversion keeps

Both formats give a motor's name, maker, diameter and length, its loaded and propellant masses, its
delays, and its [thrust curve](glossary.md#thrust-curve). The curve, the size and the masses are
kept. Converting a file and converting the result back gives each of them again, bit for bit, as
the simulator reads the file. A mass of 16 or 17 digits is the exception (below). The formats write
some things differently, and `hpr convert` translates:

| | `.eng` | `.rse` |
|---|---|---|
| masses | kilograms | grams |
| delays | `6-10-14` | `6,10,14` |
| a plugged motor's delay | `P` | `1000` |
| the curve's first point, zero thrust at ignition | left out | written |
| a name or maker of several words | joined by `_` | as it is |

Masses move between kilograms and grams by moving the decimal point, not by multiplying, which
would round. In a computer's arithmetic, `0.0041` kg times 1000 is `4.1000000000000005`, while
moving the point gives `4.1`. A mass written with 16 or 17 digits may still change in its last
digit. A warning says so.

A flight turns a `.rse` file's grams into kilograms by dividing by 1000, and the catalog's by
multiplying by 0.001. So the same motor flown from a `.eng` file and from its converted `.rse` can
differ in a mass's last digit, a part in $10^{16}$.

Some things come back written differently, though they mean the same:

- Delays spelled `p`, `1000` or with spaces come back in the table's spelling. The simulator
  reads them as the same delays.
- A name or maker of several words comes back with `_` between the words, in any `.eng` file
  `hpr convert` writes. A `.eng` header is seven fields split by spaces, and OpenRocket 24.12
  refuses one of eight ([the script](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/motor_files.py) checks it). A warning says so.
- A `.eng` motor whose delays are `-`, which names none, is written to `.rse` without delays, so
  converting it back to `.eng` needs `--delays`.
- Comments lose their blank lines and the spaces ending their lines, and comments after a `.eng`
  file's last motor are dropped. A warning says so.

A `.rse` file also gives figures a `.eng` file has no place for. They're the motor's type, its
total impulse, average and peak thrust, burn time, mass fraction,
[specific impulse](glossary.md#specific-impulse), and nozzle throat and exit diameters. Two flags
say whether RockSim works the mass and center of gravity out itself. The file gives both figures
at each point of the curve. The simulator doesn't use them for a solid motor. It works the mass and
center of gravity out from the curve and the masses, whichever format it reads.

Going to `.eng`, they're dropped, and a warning names them. Going to `.rse`, they're filled in the
way ThrustCurve.org's `.rse` files are. The total impulse comes from adding up the curve, and the
remaining propellant falls in step with the impulse delivered. The center of gravity is at half the
length, both flags are set, and the type is `unspecified`. The rules, and the counts of real files
behind them, are in the [`.rse` format notes](format/rse.md#writer-policy-strict-round-trip-stable).

`hpr convert` refuses to write a hybrid motor as `.eng`, which couldn't mark it as a hybrid (the
simulator models solid motors only). A `.eng` header must give delays. So a `.rse` motor without
them is refused until `--delays` gives them, such as `--delays P` for a plugged motor. `--delays`
fills only the motors that give none.

**Other programs.** OpenRocket 24.12 opens all 32 files `hpr convert` writes from the bundled
curves. A [script](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/motor_files.py) compares what OpenRocket reads from each converted file with what it
reads from the original. It compares the name, the maker as OpenRocket names it, the type, and
the diameter and length. It also compares the launch and burnout masses, the curve's points and
the delays. 29 of the 32 read the same.

- Two are `.rse` files that say the motor is reloadable or single-use. A `.eng` file can't say
  it, so OpenRocket reads the converted files' type as unknown. HPR Sim flies both types alike.
- The third is the Aerotech G69N's `.eng` file, which writes its plugged delay as `1000`.
  OpenRocket reads no delay from that original, but a plugged motor from the converted `.rse`.
  HPR Sim reads both as plugged, so the difference is in OpenRocket's reading of the original, not
  in what `hpr convert` wrote.

The project's automated tests don't run the script, as they have no OpenRocket.

#### Converting a design

Given design files, `hpr convert` takes a design between OpenRocket's `.ork` and
[the HPR design format](format/hpr.md). That format is `.hpr`, one JSON document, or `.hprz`, a zip
archive of the document with other files beside it. The extensions pick the formats, any of the
three to any.
Here one of the repository's public designs becomes a `.hpr`:

<!-- cli: example `hpr convert validation/fixtures/ork/loft-demo/demo-multi-config.ork demo.hpr`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr convert validation/fixtures/ork/loft-demo/demo-multi-config.ork demo.hpr
read   demo-multi-config.ork
wrote  demo.hpr: "Loft Demo 38mm — motor comparison", 2 motor configurations, HPR design format 0.2
```

<!-- cli: end -->

The document holds everything the simulator read from the `.ork`, including what it doesn't model.
So `hpr convert demo.hpr demo.ork` writes the `.ork` the simulator would write from the original,
byte for byte. `hpr sim demo.hpr` flies it to the same flight as the `.ork`. A document of
an older version of the format is migrated as it's read, and the output says from which version.

`hpr sim` may refuse `--motor` for a version 0.1 document, which didn't record whether the rocket
was read exactly as written. Converting the `.ork` again fixes that
([versions](format/hpr.md#versions)).

`--attach` adds a file to a `.hprz`, once for each file: a flight log, a photograph, anything, up
to 256 MiB for the whole container. Each goes at the top of the container, under its file name. A
name the container can't hold, such as `CON.txt`, is refused
([the container's name rules](format/hpr.md#the-container-hprz)). Converting a `.hprz` to a `.hprz`
keeps its attachments, and adds any `--attach` names. Converting one to a `.hpr` or a `.ork` leaves
its attachments out, and a warning names each.

With `--json`, the output
([`convert.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/convert.schema.json))
gives:

- the files read and written, with their formats;
- the rocket's name, and how many motor configurations the design holds;
- the version of the format the input was migrated from, if it was;
- the attachments written into a `.hprz`;
- the warnings: the `.ork` reader's, the `.ork` writer's, and each attachment left out.

## Logs and weather

`hpr analyze` reads an altimeter's flight log, and `hpr weather` writes the air and wind over a launch site.

### `hpr analyze`

`hpr analyze` reads the log an altimeter recorded during a flight. It prints what the log says:
when the rocket lifted off, how high it went, how fast it climbed, and when it landed. It needs
only the log. It takes no design file and runs no simulation. So it answers "what did my rocket
do?" for anyone who flew one, whatever they designed it in.

It reads each layout HPR Sim knows by that layout's own reader. Another logger's file is read by a
JSON file you write for it ([Reading your own logger](your-own-logger.md) works one through). A
table is read by a column mapping, `--mapping`. Binary records, or a serial console's capture of
them, are read by a record layout, `--layout`. The readers cover these
([the layouts read](format/flight-logs.md)):

- PerfectFlite's `.pf2` logs, the Pnut's, the StratoLogger's and the StratoLoggerCF's
  ([the format](format/pf2.md))
- AltOS's CSV
- the Featherweight Raven's FIP CSV
- the Blue Raven's, the RRC3's and Eggtimer's CSV
- an Eggtimer TRS's terminal text
- PerfectFlite's exports and older `.pfa`
- AltimeterCloud's and Entacore AIM's CSV
- AerospaceNU's SillyGoose logs and its flight computer board's CSV and ground station text
- Silicdyne Fluctus's CSV
- the ARTS2's interpreted data
- a phone app's `launch` CSV
- an Eggtimer's download saved as RTF
- the Raven's `.FIPa` and the RRC3's `.rff` raw downloads
- the Featherweight app's flight summaries
- the R-DAS's raw text
- AltOS's radio telemetry and its `.eeprom` raw downloads
- the ARTS2's `.odf` and raw CSV
- Missile Works' RTx tracker's `.rtx` file and exports
- tables whose headers state their units
- the sheets of an Excel workbook
- binary records by a layout the caller states
- a Blue Raven's USB download
- a course's Teensy data logger
- one team's radio telemetry packets
- a Raven FIP export pasted below no header
- an AltiMAX's `.xls` export
- an AltiMAX's `.ALS` file
- Entacore AIM's `.xtra` files and raw dumps
- a MARSA altimeter's download
- AltOS's radio telemetry decoded to a table
- AltOS's CSV re-saved by a spreadsheet or cut to its columns
- tables whose headers state too little, by a column mapping
- GPS tracks

A workbook's sheets and a raw dump's recordings are read one at a time
([A file of several records](#a-file-of-several-records)). A zip archive is refused. Unpack it and
give the log inside. Formats no sample holds are
[listed as needing samples](format/flight-logs.md#formats-that-need-samples)
([evidence row G9](evidence.md#g9)).

`hpr analyze` takes its readings from any log's barometric altitude or pressure, whichever logger
wrote it ([readings on every logger](decisions-and-roadmap.md#m7-2b)). It states a bound on each.
A GPS track gives none: a receiver's height lags and drops out too much to bound
([the height track](physics/log-readings.md#the-height-track)).

Near the speed of sound a barometer reads the air wrongly. So the top speed is withheld as
`transonic` once the climb reaches 227.7 m/s (747 ft/s). That's Mach 0.8 in the coldest air the
bounds allow below 20 km (12 mi).

```bash
hpr analyze flight.pf2
```

Each reading is a value that says where it came from and how far it may be off. One the log can't
support is `withheld`, with the reason. The text output gives the reason as a sentence. The JSON
output adds its code, listed on
[Flight-log readings](physics/log-readings.md#when-a-reading-is-withheld). A log read with
readings withheld still exits with 0; a file the simulator can't read exits with 1
([Exit codes](#exit-codes)).

[The rest of the readings](decisions-and-roadmap.md#m7-2c) are the flight time, the descent under
each canopy with the main's opening, and the top acceleration and burnout. The top
acceleration comes only from an accelerometer along the rocket's axis, printed in m/s² and g.
Working it out from the altitude would turn the altitude's one-foot steps into spikes of many g.
It's what the part reads, the [specific force](glossary.md#specific-force), with gravity's 1 g in,
where `hpr sim`'s `max_acceleration_m_s2` is the rocket's own acceleration. A PerfectFlite has
none, so it's withheld as `no_accelerometer`.

Burnout is then printed as no sooner than the top speed's time less its bound. That's a [floor](glossary.md#floor): the true
burnout comes no earlier and may come later by any amount
([Flight-log readings](physics/log-readings.md#the-accelerometer)).

A top acceleration can be a floor too. Where the part clipped and the top reached the range the
logger's family states, its line says "clipped at the part's range of …". Where it held flat below
that range, or none is stated, the line says "held flat at its top, so it may have been clipped".
Where the accelerometer holds the thrust only in part, with no start or no end to it, the line
says that.

The JSON marks each `is_floor`, and gives the clip's cause in `clip`, `range` or `flat`. `clip` is
`null` for a top that didn't clip and for a thrust with no end.

What the file states about itself, such as the altimeter's own apogee, is printed beside the
simulator's readings, never in their place. Heights are meters above the altimeter's reading on
the pad, with feet in brackets. Times are seconds on the log's clock.

This example log is invented, so every reading can be checked against the flight it was made
from ([Reading a flight log](reading-a-flight-log.md) works through it). Its true apogee is
390.3 m (1280.5 ft), at 10.26 s:

<!-- cli: example `hpr analyze validation/fixtures/logs/synthetic-pnut.pf2`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/synthetic-pnut.pf2
synthetic-pnut.pf2: PerfectFlite Pnut, serial 0, flight 1
984 samples, from 0.00 s to 49.15 s
readings from the logger's barometric altitude, sampled every 0.050 s in steps of 0.305 m, after a 0.30 s running median
4 channels: Altitude (altitude above ground), Velocity (speed), Temperature (F) (temperature), Voltage (voltage)
the logger states: apogee 390.4 m (1281 ft); ground elevation 182.9 m (600 ft) above sea level

liftoff           0.80 s, within 0.81 s
apogee            390.1 m (1280 ft) above the pad, within 31.9 m (105 ft); at 10.28 s, within 0.88 s, 9.47 s after liftoff
                  highest sample 400.5 m (1314 ft) above the pad at 11.35 s, set aside by the median
top speed         78.3 m/s (257 ft/s), within 20.6 m/s (68 ft/s), at 2.20 s, 71.9 m (236 ft) up: derived from the heights
landing           45.80 s, within 2.11 s; 35.52 s from apogee, at 10.9 m/s (36 ft/s) on average, within 2.1 m/s (7 ft/s)
flight time       45.00 s, within 2.91 s
drogue descent    21.7 m/s (71 ft/s), within 2.2 m/s (8 ft/s), from apogee at 10.28 s to the main's opening at 21.40 s, 390.1 m (1280 ft) to 148.4 m (487 ft) up
main descent      6.0 m/s (20 ft/s), within 0.8 m/s (3 ft/s), from the main's opening at 21.40 s, within 1.06 s, to landing at 45.80 s, 148.4 m (487 ft) to 2.1 m (7 ft) up
top acceleration  withheld: the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g
burnout           no sooner than 0.88 s: the top speed's time, 2.20 s (1.40 s after liftoff), less its bound, 1.32 s; derived from the heights, and may have come later by any amount

How far to trust it. Measured by the logger, read by HPR Sim from its log, not simulated. Each
reading states the bound its error stayed within on HPR Sim's synthetic flights, logged at every
rate from 500 to 0.5 samples a second and in steps up to 10 m or 100 Pa, in air within 15 K of
standard; a reading the log can't support is withheld, with the reason. The top speed is derived
from the heights, and withheld from Mach 0.8, where a barometer's port may read the flow over the
nose. For a record or a certification, the altimeter's own reading is the one to log, and the RSO
decides.
More: https://hpr.fusionspace.co/reading-a-flight-log.html
```

<!-- cli: end -->

The highest sample in the file is 10.4 m (34 ft) above the apogee. It's the pressure pulse of the
ejection charge that fired a second after it. The readings are taken from the altitude after a
[running median](glossary.md#running-median), which sets that pulse aside.
[Reading a flight log](reading-a-flight-log.md) explains each reading, and how far to trust it.

#### A log with an accelerometer

A log with an accelerometer along the rocket's axis gives the top acceleration and burnout from
it. This invented log, written by the `make.py` beside it, is of a 2 kg rocket. Its motor pushes
120 N, a tenth more at its crest, from 2 s into the log, and stops at 3.45 s. Its accelerometer
writes, in g, what the part reads, gravity in.

A table of columns doesn't name its logger, so the first line says `unknown`. With no primary
track, the heights come from its altitude column and the accelerometer is read on its own clock:

<!-- cli: example `hpr analyze validation/fixtures/logs/accelerometer/axial.csv`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/accelerometer/axial.csv
axial.csv: unknown
no primary track: each channel on its own clock
readings from the barometric altitude channel, sampled every 0.010 s in steps of 0.010 m, after a 0.30 s running median
2 channels: Baro Alt AGL (m) (altitude above ground), Axial accel (g) (acceleration)

liftoff           2.30 s, within 0.67 s
apogee            272.3 m (893 ft) above the pad, within 22.8 m (75 ft); at 9.91 s, within 0.80 s, 7.61 s after liftoff
top speed         68.9 m/s (226 ft/s), within 20.7 m/s (68 ft/s), at 3.48 s, 55.9 m (183 ft) up: derived from the heights
landing           40.77 s, within 1.82 s; 30.87 s from apogee, at 8.8 m/s (29 ft/s) on average, within 1.8 m/s (6 ft/s)
flight time       38.47 s, within 2.48 s
drogue descent    19.4 m/s (64 ft/s), within 2.4 m/s (8 ft/s), from apogee at 9.91 s to the main's opening at 16.26 s, 272.3 m (893 ft) to 149.0 m (489 ft) up
main descent      6.0 m/s (20 ft/s), within 0.7 m/s (3 ft/s), from the main's opening at 16.26 s, within 0.98 s, to landing at 40.77 s, 149.0 m (489 ft) to 2.0 m (6 ft) up
top acceleration  65.4 m/s² (6.7 g), within 7.1 m/s², at 2.59 s
burnout           3.45 s, within 0.07 s, 1.15 s after liftoff: from the accelerometer

How far to trust it. Measured by the logger, read by HPR Sim from its log, not simulated. Each
reading states the bound its error stayed within on HPR Sim's synthetic flights, logged at every
rate from 500 to 0.5 samples a second and in steps up to 10 m or 100 Pa, in air within 15 K of
standard; a reading the log can't support is withheld, with the reason. The top speed is derived
from the heights, and withheld from Mach 0.8, where a barometer's port may read the flow over the
nose. For a record or a certification, the altimeter's own reading is the one to log, and the RSO
decides.
More: https://hpr.fusionspace.co/reading-a-flight-log.html
```

<!-- cli: end -->

The top acceleration, 6.7 g, is the specific force, gravity's 1 g in. The rocket itself sped up at
about 5.7 g. Burnout, 3.45 s within 0.07 s, is where the thrust's fall ends, the end of the
motor's burn ([Flight-log readings](physics/log-readings.md#the-accelerometer)).

Liftoff is read at 2.30 s, within 0.67 s, later than the motor's start. So burnout's 1.15 s after liftoff is less
than the motor's 1.45 s burn. Its time on the log's clock is the one to compare.

#### A file of several records

Some files hold more than one record. `hpr analyze` reads one of them at a time, by its own
reader. A spreadsheet's workbook holds sheets: an `.xlsx`, or an `.xls` written as one bare
worksheet, as an AltiMAX altimeter's Excel export is. Excel's own `.xls`, a compound file of
several streams, isn't read. An Entacore AIM raw dump can hold several recordings.

With no choice given, `hpr analyze` reads the workbook's one sheet that reads as a log, or the dump's one
recording. Where several sheets read, or none, or the dump holds several recordings, it refuses,
exit code 1, and names the choices:

- `--sheet N` reads the workbook's sheet `N`, counted from 0, as its layout's own reader reads it.
  The JSON output's `log` object gives it as `sheet`.
- `--recording N` reads the dump's recording `N`, counted from 0, given as `recording`. It takes
  none of `--mapping`, `--layout`, `--fip-header` and `--sheet`.

A sheet chosen for a dump, a recording chosen for a workbook, or either for a file of one record
is refused, saying why. A table on a sheet no reader takes is read by a column mapping and
`--sheet` ([below](#a-logger-of-your-own)).

The invented workbook `two-flights.xlsx` has three sheets, "First", "Notes" and "Second", and the
first and last read as logs. With no `--sheet`, `hpr analyze` refuses it: "2 of the
workbook's 3 sheets read as logs, sheets 0 (First), 2 (Second): choose one". `--sheet 2` reads the
second flight, made to peak at 548.0 m (1,798 ft) above the pad, and a note names the sheet:

<!-- cli: example `hpr analyze validation/fixtures/logs/several/two-flights.xlsx --sheet 2`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/several/two-flights.xlsx --sheet 2
two-flights.xlsx: unknown
no primary track: each channel on its own clock
readings from the barometric altitude channel, sampled every 0.200 s in steps of 0.020 m, after a 0.40 s running median
1 channel: Baro Alt AGL (m) (altitude above ground)

liftoff           1.20 s, within 1.05 s
apogee            547.8 m (1797 ft) above the pad, within 43.0 m (141 ft); at 13.10 s, within 0.99 s, 11.90 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           withheld: the log ends at 40.00 s, 280.8 m above the pad, before the altitude comes within 2.0 m of it and stays under 5.0 m for 1 s
flight time       withheld: the flight time needs landing, which is withheld above
drogue descent    withheld, as landing is
main descent      withheld, as landing is
top acceleration  withheld: the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g
burnout           withheld: burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. Measured by the logger, read by HPR Sim from its log, not simulated. Each
reading states the bound its error stayed within on HPR Sim's synthetic flights, logged at every
rate from 500 to 0.5 samples a second and in steps up to 10 m or 100 Pa, in air within 15 K of
standard; a reading the log can't support is withheld, with the reason. The top speed is derived
from the heights, and withheld from Mach 0.8, where a barometer's port may read the flow over the
nose. For a record or a certification, the altimeter's own reading is the one to log, and the RSO
decides.
More: https://hpr.fusionspace.co/reading-a-flight-log.html
note: read from the sheet "Second" of an Excel workbook, its cells as stored
```

<!-- cli: end -->

#### A logger of your own

A logger of your own is read by a file you write for it, each JSON with a schema
([JSON output](#json-output)):

- `--print-mapping` prints what a table's header states, as a column mapping to edit, and reads
  nothing more. It's the same with `--json`, and not stamped with `tool`, so it reads back.
- `--mapping FILE` reads a table by its column mapping. The mapping can mark the column that is
  the barometric altitude above the pad as the primary track. `--sheet N` names the workbook's
  sheet the table is on, `.xlsx` or a bare `.xls`, from 0. A workbook of one sheet needs none.
- `--layout FILE` reads binary records, or the block of a serial console's capture the layout
  names, by a record layout. It takes no `--sheet`.
- `--fip-header FILE` reads a Featherweight Raven's FIP export that lost its header line when it
  was pasted into a workbook. It reads the export under the header line the file states
  ([below](#a-fip-export-below-no-header)). `--sheet N` picks the workbook's sheet, `.xlsx` or
  a bare `.xls`. With none, the workbook must have only one. The rows saved as text read too.
- `--record` prints the record the log reads into, every channel in SI units (degrees for
  angles), as one JSON document, in place of the readings.

A log is read by one of `--mapping`, `--layout` and `--fip-header`, never two. The JSON output's
`log` object names the file given, as `mapping`, `layout` or `fip_header`. A mapping or layout
that doesn't follow its schema is refused, exit code 1, naming the field by its JSON path, such as
`columns[1].unit`. One the log contradicts, such as a primary track on the clock's column,
is refused naming the column. [Reading your own logger](your-own-logger.md) works one invented
logger through each.

<a id="a-fip-export-below-no-header"></a>A FIP header file has two keys. `header` is the header
line the Featherweight Interface Program writes for an export of the same channels. It gives each
channel's `Time@` clock, its value and its `bILBA` status cell.

`pasted` lists the columns, counted from 1, that another table pasted over the export covers.
Leave it out when none are. A key the schema doesn't name is refused, naming it. For an invented
export of three channels, the first's clock and value covered by a pasted table, which leaves that
channel out:

```json
{
  "header": "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,Time@Baro (Atm),Baro (Atm),bILBA,Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)],bILBA",
  "pasted": [1, 2]
}
```

A log read under a stated header is marked as one read by your own description, as a mapping's
is. Its readings are only as right as that header
([A FIP export pasted below no header](format/flight-logs.md#a-fip-export-pasted-below-no-header)).

### `hpr weather`

`hpr weather` writes down the air and the wind over a launch site, level by level from the ground
up. At each height it gives the pressure, temperature, humidity, and the wind's speed and the
[direction it blows from](glossary.md#wind-direction). That list of levels is a *profile* (a
[sounding](glossary.md#sounding), when a weather balloon measured it). It comes from one of five
sources:

| source | what it is | command |
|---|---|---|
| Open-Meteo | a free weather service's forecast, or its archive of past forecasts ([Launch-day weather](weather.md)) | `hpr weather open-meteo --latitude 32.99 --longitude -106.97 --time 2026-10-02T18:00Z` |
| University of Wyoming | a weather balloon's measurements, from the station's latest launch before yours ([Weather-balloon soundings](soundings.md)) | `hpr weather wyoming --station 72364 --time 2025-06-21T15:30Z` |
| GFS | NOAA's global forecast model, on a 0.25° grid, fetched or read from a whole file you download ([NOAA forecasts: GFS and RAP](nomads.md)) | `hpr weather gfs --latitude 32.99 --longitude -106.97 --cycle 2026-09-30T00Z --hour 18` |
| RAP | NOAA's 13 km (8.1 mi) forecast model over the contiguous U.S. and nearby Canada and Mexico ([NOAA forecasts: GFS and RAP](nomads.md)) | `hpr weather rap --latitude 32.99 --longitude -106.97 --cycle 2026-09-30T12Z --hour 6` |
| ERA5 | a [netCDF](glossary.md#netcdf) file you download from Europe's climate data service: the past weather, reconstructed ([ERA5 weather files](format/era5.md)) | `hpr weather era5 era5.nc --latitude 47.21 --longitude 9.00 --time 2020-02-22T13:00Z` |

`hpr sim` doesn't fly a profile yet (issue
[#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265)). A Rust program can. Each source's page
flies Calisto, RocketPy's example rocket, through the weather it fetched.

> **How far to trust it.** `hpr weather` runs each source's reader from the library and adds no
> physics of its own. Each reader is checked against the recorded answers on its source's page.
> The tests in
> [`crates/hpr-cli/tests/weather.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-cli/tests/weather.rs)
> run the four online sources through `hpr weather` offline, from a saved answer and from the
> cache. They run ERA5 from its file.
>
> Each run writes the profile the library builds from the same bytes, to the last bit, and lists
> the same levels left out. Fetching online isn't tested automatically. It was run once by hand, on
> 30 September 2026, for all four online sources. On a network that inspects encrypted traffic,
> every fetch fails
> ([Fetching over HTTP](online-data.md#fetching-over-http)). How good a forecast is at your launch
> is not measured.

#### Asking for the weather

- **Times are UTC**, written `2025-06-21T15:30Z`: the date, a `T`, the hour and minutes, and a
  `Z` for UTC. Seconds are optional, and so are the minutes: `2026-09-30T00Z` is midnight. A
  time without its `Z` is refused, so local time can't be mistaken for UTC.
- **Open-Meteo** reads the forecast at `--time`, between the two hours around it. Add
  `--historical` for a launch already gone: that asks Open-Meteo's archive of past forecasts.
  `--model` names one of Open-Meteo's weather models, by the name its documentation gives. By
  default it picks one ([What the simulator asks for](weather.md#what-the-simulator-asks-for)).
- **Wyoming** takes the [station](glossary.md#station)'s number, such as 72364 for Santa Teresa,
  New Mexico ([finding a station](soundings.md#what-the-simulator-asks-for)), and your launch `--time`. It
  fetches that station's latest sounding before the launch. Soundings are named for 00 and 12 UTC,
  and the balloon goes up about an hour before. The example's 12 UTC sounding left at 11:02.
  `--bufr` asks for the detailed version, a row every second or two of the climb.
- **GFS and RAP** take the [forecast run](glossary.md#forecast-run-cycle) and the hour. `--cycle` is
  when the run started, and `--hour` is how many hours after it the forecast is for. GFS runs every
  6 hours and RAP every hour. [What the simulator asks for](nomads.md#what-the-simulator-asks-for)
  lists the hours each run covers, and how long NOAA keeps them.
- **ERA5** reads a file you downloaded, at your site and time
  ([Getting a file](format/era5.md#getting-a-file)).

#### Online, offline, and saved answers

The first time you ask, `hpr weather` fetches the answer over HTTPS. It keeps a copy in the
simulator's cache folder ([Where the cache lives](online-data.md#where-the-cache-lives);
`HPR_CACHE_DIR` moves it). Ask again and it reads the copy while that is fresh
([How long a copy stays fresh](online-data.md#how-long-a-copy-stays-fresh)). If the network fails,
it falls back to an older copy, and the output says so.

- `--offline` never goes online. It answers from the cache, fresh or not. With no copy it fails,
  with exit status 1, and names what it would have fetched.
- `--from FILE` reads an answer saved earlier and touches neither the network nor the cache. It
  reads Open-Meteo's JSON, the Wyoming archive's CSV, or a [GRIB2](glossary.md#grib2) file for GFS
  and RAP. For GFS and RAP, still give `--latitude` and `--longitude`. The file covers an area, and
  the site picks the point in it. For Open-Meteo and Wyoming, the file says where and when it's
  for. So the options that choose what to fetch are refused beside it: Open-Meteo's `--latitude`,
  `--longitude`, `--historical` and `--model`, and Wyoming's `--station`, `--time` and `--bufr`.
  Open-Meteo's `--time` stays, and must fall within the file's hours. A GRIB2 file is checked as a
  fetched one is. It must be on its model's grid, and when you give `--cycle` and `--hour`, it
  must be that run and hour. Check the first line of the output, which says where and when the
  profile is for.
- `--output FILE` (or `-o`) writes the profile as JSON, described below.

A fetch that takes more than a tenth of a second says so on standard error when that's a
terminal. It shows `waiting for nomads.ncep.noaa.gov, 1 s`, then, once the answer starts to
arrive, `fetching from nomads.ncep.noaa.gov: 48.2 kB so far, 2 s`. Each goes over the one before,
gone when the answer is in. `hpr motors fetch`, `hpr motors search` and `hpr sim` fetching a motor do the
same ([Fetching over HTTP](online-data.md#fetching-over-http)).

The simulator reads the small GRIB2 files that NOAA's download server, NOMADS, cuts out around a
site. It reads whole GFS files you download yourself too, which are packed more tightly
([A whole GFS file](nomads.md#a-whole-gfs-file)). The decoder also reads fields compressed as JPEG
2000 images, which RAP's whole files use. But no whole RAP file has been run through `hpr weather`
yet ([Files in JPEG 2000](nomads.md#files-in-jpeg-2000)).

#### An example

This reads a recorded answer from Open-Meteo's archive, for Spaceport America in New Mexico at 15:30
UTC on 21 June 2025. It writes the profile to `profile.json`. (The page's run writes into a scratch
folder, shown as `<the scratch folder>`; `hpr` prints the path you give it.)

<!-- cli: example `hpr weather open-meteo --time 2025-06-21T15:30Z --from crates/hpr-net/tests/fixtures/replay/open-meteo-historical.json --output profile.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr weather open-meteo --time 2025-06-21T15:30Z --from crates/hpr-net/tests/fixtures/replay/open-meteo-historical.json --output profile.json
Open-Meteo at 32.99655° N, 106.96947° W, for 2025-06-21T15:30:00Z
Read from open-meteo-historical.json
Weather data by Open-Meteo.com (CC BY 4.0)

         height  pressure         temp  humidity       wind  from
     m MSL (ft)       hPa      °C (°F)         %  m/s (mph)   ° T
 1400.0  (4593)     859.5   29.6  (85)        18   3.3  (7)   162
 1482.0  (4862)     850.0   28.2  (83)        16   2.9  (6)   189
 2014.4  (6609)     800.0   23.4  (74)        16   2.8  (6)   225
 3160.6 (10369)     700.0   14.5  (58)        28   6.8 (15)   236
 4439.1 (14564)     600.0    3.3  (38)        60   9.8 (22)   219
 5894.6 (19339)     500.0   -7.0  (19)        70   9.9 (22)   200
 7604.0 (24948)     400.0  -17.1   (1)        34   8.3 (19)   218
 9712.0 (31864)     300.0  -31.5 (-25)         8   4.4 (10)   261
10981.7 (36029)     250.0  -41.2 (-42)         9  10.6 (24)   261
12465.1 (40896)     200.0  -52.2 (-62)        13   9.8 (22)   253
14276.4 (46838)     150.0  -64.8 (-85)        18   9.9 (22)   263
16716.8 (54845)     100.0  -70.8 (-95)        12   6.3 (14)   253
18863.9 (61889)      70.0  -66.2 (-87)         4   7.8 (17)   164
20940.5 (68703)      50.0  -60.0 (-76)         0   5.4 (12)   108
24212.5 (79437)      30.0  -54.0 (-65)         0   8.6 (19)   083

Left out: 5 levels
  below the ground, 5 levels: 1000 hPa, 975 hPa, 950 hPa, 925 hPa, 900 hPa

Profile written to <the scratch folder>/profile.json

How far to trust it. A weather model's forecast, not a measurement. HPR Sim's profile gives back
every level it keeps of Open-Meteo's answer to rounding error, checked on recorded answers; the
forecast itself is not yet checked here against measured winds or a flight's log. On launch day, go
by the latest forecast and the wind measured at the field; the RSO decides.
More: https://hpr.fusionspace.co/weather.html
```

<!-- cli: end -->

The first line says where and when the profile is for:

- for Open-Meteo, its model's [grid point](glossary.md#grid-point) nearest the site
- for GFS and RAP, the site itself, blended from the four grid points around it
- for Wyoming, where the balloon was released

The ground comes first: its height above sea level ([MSL](glossary.md#height-above-sea-level-msl))
and its pressure in hectopascals (hPa; 1013 hPa is sea level's standard). Then come its temperature
and humidity, and the wind 10 m (33 ft) above it. Each pressure level above the ground follows.

The levels the model gives below the ground are listed as left out. Here the ground is at about
1,400 m (4,593 ft), and 1000 to 900 hPa lie beneath it. Where each value comes from, and why levels
are left out, is on the source's page.

#### The profile file

`--output` writes the profile in SI units, the way the library holds it. Its `levels` run from the
lowest up, each with:

| field | unit |
|---|---|
| `height_msl_m` | meters above sea level |
| `temperature_k` | kelvin |
| `pressure_pa` | pascals (100 Pa = 1 hPa) |
| `relative_humidity` | a fraction: 0.18 is 18%; `null` for ERA5, which the simulator reads as dry air |
| `wind_speed_m_s` | meters per second |
| `wind_direction_from_rad` | radians clockwise from true north, where the wind comes from |

The file opens with a `tool` object naming the program that wrote it. Then comes `kind`, which
says what the figures are: `measured` for a balloon sounding, `forecast`, or `reanalysis` for ERA5.
Then comes `trust`, the note the printed result ends with, so a saved profile keeps how far to
trust it. Nothing checks or uses these three keys. The library reads past them, and reads a profile
saved before they were written too.

After them come the `levels` above, then `latitude_rad`, the latitude the profile was measured or
forecast at, in radians. Last is `wind_interpolation`, how the wind is blended between levels
(`speed_direction`). The Rust library reads the file back as a
[`SoundingProfile`](api/hpr_atmos/profile/struct.SoundingProfile.html), with the same checks. The
text output and `--json` give the same levels with the direction in degrees.

#### What the profile leaves out

- **Time between forecast hours**, for GFS and RAP: pick the run and hour nearest your launch.
  Open-Meteo and ERA5 blend the two hours around it.
- **The site's real ground.** A forecast's ground is its model's smoothed terrain, not your pad's
  height. An ERA5 file has no ground at all: its levels start at 1000 hPa, which can lie below a
  high site.
- **Humidity in ERA5.** The simulator doesn't read it yet, so ERA5's profile is dry air.
- **Above the top level**, the library carries the air on as the standard atmosphere and holds
  the top wind. Each source's page gives its top.

## Scripting and shells

What a script or a shell gets from `hpr`: its JSON documents, its exit codes and its completion scripts.

### JSON output

With `--json`, a command prints exactly one [JSON](https://www.json.org) document on standard
output, and nothing on standard error. That holds when the command fails, too. What the text's
`warning:` and `note:` lines say is in the document's own fields instead, such as `hpr sim`'s
`notes`, `warnings`, `flags` and `issues`. Each document has a published [JSON Schema](https://json-schema.org), which
describes its fields and units:

| output | schema |
|---|---|
| `hpr sim` | [`sim.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/sim.schema.json) |
| `hpr motors list` | [`motors-list.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/motors-list.schema.json) |
| `hpr motors show` | [`motors-show.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/motors-show.schema.json) |
| `hpr motors search` | [`motors-search.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/motors-search.schema.json) |
| `hpr convert` | [`convert.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/convert.schema.json) |
| `hpr analyze` | [`analyze.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze.schema.json) |
| `hpr analyze --record` | [`analyze-record.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-record.schema.json) |
| `hpr weather` | [`weather.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/weather.schema.json) |
| `hpr completions` | [`completions.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/completions.schema.json) |
| any failure | [`error.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/error.schema.json) |

Every document opens with the same `tool` object, the program that wrote it:
`{"name": "FusionSpace HPR", "version": "0.1.0", "designation": "FS-ACHERNAR \u00b7 SW \u00b7 TOOL 001"}`.
The text is ASCII, so the designation's middle dot `·` is written as its escape `\u00b7`. Every
JSON reader reads that back as the dot
([Exporting a flight](exporting-a-flight.md#which-program-wrote-a-file)). An error document's
`help` lists what to do about the failure, the lines the text output starts with `help:`. It's
empty when there's nothing to suggest.

The sidecar of a CSV recording ([Exporting the recording](#exporting-the-recording)) has its own
schema, [`sim-export-meta.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/sim-export-meta.schema.json).
Three files `hpr analyze` reads have schemas too, with no `tool`, as a person writes them:

- the column mapping, [`analyze-mapping.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-mapping.schema.json),
  which `--print-mapping` prints
- the record layout,
  [`analyze-layout.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-layout.schema.json)
- the FIP header,
  [`analyze-fip-header.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-fip-header.schema.json)

Units are SI, and each field's name says its unit: `total_impulse_ns` is in newton-seconds and
`diameter_m` in meters. The exceptions say so in their names too:

- `hpr sim` gives angles in degrees (`latitude_deg`) and margins in calibers (`margin_cal`)
- `hpr weather` gives the wind's direction in degrees (`wind_from_deg`) and humidity as a fraction
  (`relative_humidity`, 0 to 1)
- `hpr motors list` keeps the catalog's millimeters (`diameter_mm`)
- `hpr motors search` keeps the site's millimeters and cents (`diameter_mm`, `unit_price_cents`)

Numbers aren't rounded, so a converted value can end in digits such as `0.0036000000000000003`. A
motor's figures carry no more precision than its curve file.

FusionSpace HPR is pre-alpha, so the fields may still change. The schemas are published beside the
code, and a change to a document changes its schema in the same commit.

<!-- cli: example `hpr motors show B4 --json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors show B4 --json
{
  "tool": {
    "name": "FusionSpace HPR",
    "version": "0.1.0",
    "designation": "FS-ACHERNAR \u00b7 SW \u00b7 TOOL 001"
  },
  "kind": "copied",
  "trust": "How far to trust it. Copied from ThrustCurve.org's curve file, downloaded 2026-09-17, not measured by HPR Sim: the size, masses and delays as the file's header gives them, the impulse, thrusts and burn time computed from its curve, which can differ from the maker's rated figures. The total impulse and peak thrust match OpenRocket's reading of the same file on every bundled curve, a check of the arithmetic, not of the motor; no figure is checked against the maker's or the certifying bodies' data. The motor's printed data and its maker's instructions come first, and the RSO decides. More: https://hpr.fusionspace.co/pick-a-motor.html",
  "motors": [
    {
      "name": "B4",
      "manufacturer": "Quest Aerospace",
      "source": {
        "kind": "catalog",
        "curve_url": "https://www.thrustcurve.org/simfiles/5f4294d20002e9000000088e/",
        "format": "eng"
      },
      "diameter_m": 0.018,
      "length_m": 0.08,
      "propellant_mass_kg": 0.0036000000000000003,
      "loaded_mass_kg": 0.0198,
      "total_impulse_ns": 4.892141999999999,
      "impulse_class": "B",
      "average_thrust_n": 4.403753380057823,
      "peak_thrust_n": 7.2,
      "burn_time_s": 1.110902808988764,
      "burn_start_s": 0.01745,
      "burn_end_s": 1.128352808988764,
      "curve_end_s": 1.184,
      "delays": [
        {
          "kind": "seconds",
          "value": 4.0
        },
        {
          "kind": "seconds",
          "value": 6.0
        }
      ]
    }
  ],
  "warnings": []
}
```

<!-- cli: end -->

A failure prints an error document. Its `kind` matches the exit status, and `milestone` is set
only for a command not available yet. Here `hpr sim --offline` refuses the test rocket above
without `--motor`, as neither the file, the bundled catalog nor the cache holds a curve for its
motor:

<!-- cli: example `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --offline --json`, exits 1; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --offline --json
{
  "tool": {
    "name": "FusionSpace HPR",
    "version": "0.1.0",
    "designation": "FS-ACHERNAR \u00b7 SW \u00b7 TOOL 001"
  },
  "error": {
    "kind": "input",
    "message": "configuration [H128W-0] can't be flown as the file has it: no thrust curve for H128W: no embedded curve, and no motor of that manufacturer and designation in the bundled catalog; AeroTech H128W is not in HPR Sim's cache of ThrustCurve.org, and the run is offline",
    "command": "sim",
    "milestone": null,
    "help": [
      "with a network connection, `hpr motors fetch --manufacturer AeroTech H128W` fetches it into the cache",
      "give a motor with --motor"
    ]
  }
}
$ echo $?
1
```

<!-- cli: end -->

### Exit codes

| status | `kind` in JSON | meaning |
|--:|---|---|
| 0 | - | The command did what was asked. |
| 1 | `input` | An input was missing, unreadable or refused, such as a motor the catalog doesn't have. |
| 2 | `usage` | The command line was wrong: an unknown command or option, or a missing argument. |
| 3 | `not_available` | The command is registered, but the milestone that brings it hasn't come yet. |

`hpr --help` and `hpr --version` print text and exit with 0, even with `--json`.

### `hpr completions`

`hpr completions` writes a script that lets your shell complete `hpr`'s commands and options when
you press Tab. Save it where your shell looks for completions:

| shell | command |
|---|---|
| bash | `hpr completions bash > ~/.local/share/bash-completion/completions/hpr` |
| zsh | `hpr completions zsh > ~/.zfunc/_hpr`, with `fpath+=~/.zfunc` before `compinit` in `~/.zshrc` |
| fish | `hpr completions fish > ~/.config/fish/completions/hpr.fish` |
| PowerShell | `hpr completions powershell >> $PROFILE` |
| elvish | `hpr completions elvish >> ~/.config/elvish/rc.elv` |

## What it leaves out

- **Some unpowered separations, a dropped stage's accuracy, and a rocket `.json`'s parachutes.**
  `hpr sim` flies powered separations, one or several in turn. It flies a lone unpowered one
  before apogee only if the part that keeps the nose has a device open by then. A dropped stage
  flies as a point under its devices' drag, so its landing is rough. It flies a `.ork` file's
  parachutes and streamers, but a rocket's `.json` comes down with no recovery
  device ([what it doesn't fly yet](#what-hpr-sim-doesnt-fly-yet)).
- **Readings within bounds, not intervals.** `hpr analyze` gives each reading within a bound or
  withheld, on every layout it reads. It gives no Mach number and takes no top acceleration from a
  board's three axes. It finds no deployment but the main's opening, from the descent's rate. A
  planned milestone adds [an interval on each reading](decisions-and-roadmap.md#m7-2e).
- **Only OpenRocket's design files and the HPR design format.** `hpr convert` and `hpr sim` read
  OpenRocket's `.ork` and the HPR design format's `.hpr` and `.hprz`, not RockSim's `.rkt` or
  RASAero's `.CDX1`
  ([how the formats compare](format/hpr.md#how-it-compares-with-other-design-formats)).
- **No validation check.** The validation cases and their reference results are files in the
  repository, not part of the tool. To re-run them, clone the repository and run
  `cargo xtask validate --check` ([what it checks](accuracy.md#the-census)).
- **RockSim is unchecked.** OpenRocket opens the files `hpr convert` writes
  ([evidence row G25](evidence.md#g25)); whether RockSim does isn't checked.
- **`hpr sim` doesn't fly the weather.** [`hpr weather`](#hpr-weather) writes a profile that only
  a program reads so far (issue [#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265)).
- **Stock from one site.** `hpr motors search` covers only what motor.fusionspace.co does
  ([what it leaves out](motor-stock.md#what-it-leaves-out)). It has no filter by total impulse or
  by reload case.
- **Only 32 motors are built in.** Any other motor needs its `.eng` or `.rse` file, or its curve
  [fetched from ThrustCurve.org](#motors-from-thrustcurveorg) into the cache. `hpr motors search`
  lists motors you can buy, not their curves.
- **Downloads for three systems** ([Running it](#running-it)). On other systems, `hpr` is built
  from source with Rust.
- **The text output is for people.** Its layout may change between versions. Scripts should read
  `--json`, whose schemas are published.
