# Flight logs: the record and the layouts read

See which altimeters' flight logs HPR Sim reads, what it keeps from each, and how that was checked.

An altimeter or flight computer saves its flight as a file: a clock and the readings of its
sensors, such as a barometer's altitude, an accelerometer, a battery's voltage, and, on some, a
GNSS receiver's position. Each maker writes its own layout. HPR Sim reads each layout into one
record, the same for every logger, so that what comes after (the readings, and later the
comparison with a simulation) doesn't depend on which altimeter flew.

> **How far to trust it.** Every sample log the project holds of the layouts below is read but
> one AltOS export whose header line is damaged, which is refused
> ([how it was checked](#how-it-was-checked)). No maker publishes a specification of these
> layouts, so each reader follows what the files show, the makers' manuals and the notes of
> Debrief, the project owner's earlier flight-log analyzer, with each unit checked where the
> files allow. Where a header names no unit, the reader decides it from the file and says how in
> a note ([units the file decides](#units-the-file-decides)); the few units no source states are
> named [below](#units-no-source-states). Other layouts are refused, not guessed at; they come
> with [M7.1c](../decisions-and-roadmap.md#m7-1c), the other loggers' formats.

**To read a flight** from these layouts today, use the Rust library: give the file's bytes to
`hpr_flightdata::import::read`
([API reference](../api/hpr_flightdata/import/index.html)). It recognizes the layout from the
file's first line, never its name, and reads it or refuses it, naming the layouts it knows:

```rust,ignore
let bytes = std::fs::read("flight.csv")?;
let log = hpr_flightdata::import::read(&bytes)?;
for channel in &log.channels {
    println!("{} ({:?}): {} samples", channel.column, channel.quantity, channel.values.len());
}
```

[`hpr analyze`](../cli.md#hpr-analyze), the command line, and Python read PerfectFlite's `.pf2`
only so far. The readings `hpr analyze` prints (liftoff, apogee and the rest) were set and tested
on PerfectFlite logs, so the library withholds them on another logger's record, with that reason,
until [M7.2](../decisions-and-roadmap.md#m7-2) validates them there.

Written for [M7.1a](../decisions-and-roadmap.md#m7-1a) (the record, the survey of the sample
logs, and the PerfectFlite, AltOS and Raven readers), as
[ADR-251](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0251-flight-log-importers-in-two-increments.md)
decided, and [M7.1b](../decisions-and-roadmap.md#m7-1b) (the Blue Raven, RRC3 and Eggtimer
readers), as
[ADR-252](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0252-rrc3-eggtimer-and-blue-raven-csv-first.md)
decided.

## The layouts read

| Layout | Loggers | Recognized by its first line | Reader |
|---|---|---|---|
| PerfectFlite `.pf2` | Pnut, StratoLogger, StratoLoggerCF | names PerfectFlite | [`.pf2` files](pf2.md) |
| PerfectFlite CSV export | the same, through PerfectFlite's software | `Time,Altitude,` and more of the export's own column names | [below](#perfectflites-exports) |
| PerfectFlite text export | the same | the same names, separated by tabs | [below](#perfectflites-exports) |
| AltOS CSV | Altus Metrum's TeleMetrum, TeleMega, EasyMega, EasyMini, through AltosUI (Altus Metrum's ground software) | `#version,serial,flight,`, or `#` comment lines and then that header or `version,serial,flight,` | `hpr_flightdata::altos` |
| Raven FIP CSV | Featherweight Raven, through the Featherweight Interface Program | opens with a time column and holds the Raven's `Axial Accel (Gs)` channel | `hpr_flightdata::raven` |
| Blue Raven CSV | Featherweight Blue Raven, through its phone app: a low-rate file (barometer, inertial figures, flags) and a high-rate file (gyroscopes, accelerometers, attitude) | `Year,Month,Day,Time,Flight_Time_(s),Sync,` | `hpr_flightdata::blueraven` |
| RRC3 CSV | Missile Works RRC3, through mDACS, Missile Works' program that downloads its flights | `Time`, `Altitude`, `Pressure`, `Velocity`, then any of `Temperature`, `Events`, `Voltages`, perhaps with units in brackets | `hpr_flightdata::rrc3` |
| Eggtimer CSV | Eggtimer Classic, TRS, Quantum and Proton, through Eggtimer's download | `T,Alt,` and more of Eggtimer's column names | `hpr_flightdata::eggtimer` |

The Blue Raven app writes each flight as a low-rate file, logged 50 times a second, and a
high-rate file, 500 times a second (*Blue Raven User's Guide*, 2026-05-12, p18); each is read on its own, as a record of its own, not joined
to the other. The high-rate file holds no altitude, so its record's
[primary track](#the-record) is empty and its channels hold everything; the low-rate file of the
same flight holds the altitude. The app's summary and events files beside them aren't read yet. The other
layouts in the sample logs, among them PerfectFlite's older `.pfa`, AltOS's binary `.eeprom`, the
RRC3's binary `.rff`, the Raven's `.fipa`, Featherweight's GPS tracker and spreadsheets, are
refused today, most as *unrecognized*; the [survey's report](#how-it-was-checked) counts them by
layout.

## The record

Every reader fills the same record, `FlightLog`, in SI units:

- **What the file says about itself:** the logger's name, serial number, firmware and flight
  number where the file states them, and figures the logger worked out itself, such as its apogee,
  kept beside HPR Sim's readings and never in place of them.
- **The primary track:** the barometric altitude above the pad (AGL, above ground level) on the
  logger's clock, with the barometer's vertical speed, the temperature and the battery voltage
  where they share that clock. The [readings](../physics/log-readings.md) are taken from it.
- **Every column, as a channel:** each data column of the file, in the file's order, with the
  name the file gives it, what it measures, and its own clock. Nothing the logger wrote is dropped:
  a column HPR Sim can't name a quantity for is kept as written and marked *unrecognized*.
- **Events:** what the logger says happened and when, such as AltOS's change from `boost` to
  `coast`.
- **Notes:** what the reader noticed and worked around.

A channel's unit follows from what it measures:

| Quantity | Unit | Examples |
|---|---|---|
| altitude above the pad (AGL), above sea level (ASL or MSL), from GNSS (satellite navigation, such as GPS), or integrated from the accelerometer | m | `height`, `[Altitude (Baro-Ft-AGL)]` |
| pressure | Pa | `pressure`, `Baro (Atm)` |
| speed | m/s | `speed`, `Velocity (Accel-Ft/Sec)` |
| acceleration | m/s² | `acceleration`, `Axial Accel (Gs)` |
| temperature | K | `temperature`, `Temperature (F)` |
| voltage, current | V, A | `battery_voltage`, `Current Draw (A)` |
| latitude, longitude, an angle | degrees | `latitude`, `pad_az` (the bearing from the pad) |
| a rate of turn | degrees per second | `Gyro_X` |
| a time of day on the logger's clock | s after its midnight, in no stated time zone | a Blue Raven's `Time` |
| distance | m | `pad_dist` (the distance from the pad) |
| a number with no unit | none | a count of satellites, a signal strength, a state code |

A foot is 0.3048 m (1.000 ft), a g 9.806 65 m/s² and an atmosphere 101 325 Pa, each exactly. A
cell the file leaves empty, or fills with something that isn't a number, is a gap: `NaN` ("not a
number") in the record, and `null` in its JSON, which reads back as a gap.

### Units no source states

Most units come from the file's own column names or from the maker's manual. A few don't:

- AltOS's `pad_dist` and `pad_range` are read as meters, as its manual says its exports are
  metric, without naming these columns.
- AltOS's gyroscope and magnetometer columns (`gyro_*`, `mag_*`) are kept as written, unrecognized,
  as no document states their units.
- The Raven's status columns, `bILBA`, are kept as numbers, the bits read as binary: no published
  source says what each bit means, so the record takes no events from them.
- The Blue Raven's `Current` is kept as written, unrecognized: the app's file states no unit, and
  its manual gives milliamperes only for another download; the values fit amperes better, but
  that is a size, not a source.
- An RRC3's `Temperature` and `Voltages`, without units in brackets, are read in degrees
  Fahrenheit, or Celsius when the altitude is in meters, and in volts, assumed: the RRC3's own
  unit settings pair feet with degrees Fahrenheit and meters with degrees Celsius (*RRC3+/RRC3
  User Manual*, rev. 1.71, p12), but mDACS writes each in its own preferred unit (*mDACS User
  Manual*, rev. 1.71, p5), which the file doesn't name. Each record says so in a note.

### Units the file decides

Three layouts leave some units out of their headers. For some, the reader works the unit out from
the file as it reads it; for the rest, it takes the unit the maker's manual gives, and checks it
against the file where the file can show a disagreement. Either way it writes a note giving the decision and this file's evidence, and
the [survey's report](#how-it-was-checked) counts the decisions over the sample logs:

- **An RRC3's altitude, speed and clock, worked out from the file.** Missile Works' mDACS (the
  program that downloads an RRC3's flights to a computer) writes the altitude, speed and
  temperature each in the unit chosen in its preferences, and the header names none of them
  unless the units are in brackets. The pressure settles the altitude: the climb the pressure
  implies in the standard atmosphere is set against the altitude column's climb read in feet and
  in meters, 3.28 times apart, and the unit within 25% of it is taken. The speed is set against
  the altitude's rate of climb, allowing for the speed to lag behind it: of feet per second,
  miles per hour, meters per second and kilometers per hour, the one two measures agree on within
  3% is taken. Where they don't, or the record stops before the rocket is halfway down, the column
  is kept as written, with a note. The time is in seconds where its step is the RRC3's 20 samples
  a second. Of the 50 RRC3 exports among the sample logs, 46 are read in feet from the pressure
  and 4 state feet; 36 have speeds in feet per second and 8 in miles per hour, 4 state feet per
  second, and 2, whose records stop early, keep their speeds as written.
- **An Eggtimer's clock, worked out from the file.** The Classic and TRS write their time in
  milliseconds, the Quantum and Proton in seconds, under the same `T`. The reader takes
  milliseconds when the median step is above 5: the manuals' rates, 1 to 33 samples a second, put
  a step between 0.03 and 1 in seconds and between 30 and 1,000 in milliseconds. Of the 43
  Eggtimer files, 3 are read in milliseconds and 40 in seconds.
- **An Eggtimer's heights and speeds, from the manuals.** The Classic and TRS manuals state feet
  and feet per second. The Quantum and Proton manuals state no unit for the download, but give
  every height in feet, and no Eggtimer manual offers meters; the heights are read in feet for
  every model. A speed is read in feet per second only where it matches the altitude's change
  over the step on at least 90% of rows (on the sample logs, 95.9% or more); otherwise it is kept
  as written.
- **A Blue Raven's speeds, inertial positions, rates of turn and accelerations, from the
  manual.** The manual gives their units for the logger's USB download: feet per second, feet,
  degrees per second and g (*Blue Raven User's Guide*, 2026-05-12, p23). The app's own files
  agree: its summary of a flight states the top speed in feet per second and the inertial
  altitude in feet, and its accelerometers read 1 g at rest on the pad. A file that disagreed
  would still be read in these units; its note would show the disagreement.

A unit read in the wrong one of feet and meters would put every height 3.28 times too high or too
low, so the RRC3's is never assumed where its pressure can settle it, and the Eggtimer's rests on
the manuals.

### A worked example

PerfectFlite's CSV export of a flight starts like this: the first rows of an invented flight,
`perfectflite-export.csv`, which the crate's tests read and check these numbers against:

```text
Time,Altitude,Velocity,Temperature (F),Voltage,Acceleration (calculated)
0.00,0,0,71.5,9.1,-3
0.50,0,0,71.5,9.1,-3
1.00,0,88,71.4,9.1,1
```

It reads into a record whose primary track has the times 0, 0.5 and 1 s and the altitudes 0, 0 and
0 m (0 ft), and five channels on that clock: `Altitude`; `Velocity`, whose 88 at 1 s reads as
26.82 m/s (88.0 ft/s); `Temperature (F)`, whose 71.5 reads as 295.1 K (71.5 °F); `Voltage`; and
`Acceleration (calculated)`, which the export doesn't explain and is kept as written,
unrecognized.

## PerfectFlite's exports

PerfectFlite's software exports a flight as CSV, `Time,Altitude,Velocity,Temperature (F),Voltage`,
sometimes with `Acceleration (calculated)` after them, or as tab-separated text, `Time`,
`Altitude`, `Temp.`, `Voltage`, whose temperatures carry their unit letter, such as `58.7F`. The
units are a `.pf2`'s: seconds, feet and feet per second. The header doesn't state the altitude's
unit, and the sample logs bear out feet. Of the 16 exports among them, 13 peak at exactly the
height of the `.pf2` saved beside them, which states feet. Read in feet, those 13 climb in 0.94 to
1.05 times as long as a fall in vacuum from their peak would take, and the other 3 in 0.93 to
1.07 times, alike; read in meters, the 3 would climb in 0.51 to 0.59 times
([the survey's report](#how-it-was-checked) prints these). An export doesn't name the
altimeter's model, so its logger reads `PerfectFlite`.

A header with any other name, or these names in another case, isn't read as PerfectFlite's: the
RRC3's `Time,Altitude,Pressure,…` is the RRC3's, and a bare `time,altitude` says nothing of its
units.

## How it was checked

`cargo xtask logs`, a command of the project's own build tool, reads every sample log the project
holds: the private collection's catalog of flight logs, the public fixtures and samples of
Debrief, and AerospaceNU's published flight logs, each distinct file once. It leaves out a cache
of downloads whose flights' logs the collection already catalogs, and RocketPy's processed flight
tables, which no logger wrote. It writes what came of each to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md):
per layout, how many files were read and how many refused and why; how many files' readers
decided each unit their headers leave out, and how; and per file an anonymized id (the start of a
hash of the file, which says nothing of its contents) with a digest, a hash, of the record read
from it.

A file the collection's catalog files under a layout it doesn't have is moved, by id, with the
reason printed: one is a spreadsheet's re-save of an AltOS export, its header edited, which is
generic CSV; another is an AltOS export whose header line is damaged, two column names run
together, which no reader can line up with its rows and which is refused.

The collection is other people's data, Debrief's fixtures come from flights shared on terms nobody
recorded, and AerospaceNU's are published under the GPL, so no file and no value read from one is
committed. The digest pins each reading instead, as a snapshot (a stored copy of a test's output)
would: the check fails if a reader's output changes or a file of these layouts stops reading.
Invented files in the crate's tests are read in CI and pinned whole by snapshots.
