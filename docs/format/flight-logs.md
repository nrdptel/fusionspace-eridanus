# Flight logs: the record and the layouts read

See which altimeters' flight logs HPR Sim reads, what it keeps from each, and how that was checked.

An altimeter or flight computer saves its flight as a file: a clock and the readings of its
sensors, such as a barometer's altitude, an accelerometer, a battery's voltage, and, on some, a
GNSS receiver's position. Each maker writes its own layout. HPR Sim reads each layout into one
record, the same for every logger, so that what comes after (the readings, and later the
comparison with a simulation) doesn't depend on which altimeter flew.

**To read a flight**, give the file's bytes to `hpr_flightdata::import::read`
([API reference](../api/hpr_flightdata/import/index.html)): it recognizes the layout from the
file's first line, never its name, and reads it or refuses it naming the layouts it knows.
[`hpr analyze`](../cli.md#hpr-analyze) reads PerfectFlite's `.pf2` so far: the readings it prints
were set and tested on PerfectFlite logs, and reading other loggers' flights waits for
[M7.2](../decisions-and-roadmap.md#m7-2), which validates them.

> **How far to trust it.** Every sample log the project holds of the layouts below is read, from
> three sources ([how it was checked](#how-it-was-checked)). No maker publishes a specification of these layouts, so each reader
> follows what the files show and the notes of Debrief, the project owner's earlier flight-log
> analyzer, with each unit checked where the files allow. Other layouts are refused, not guessed
> at; they come with [M7.1b](../decisions-and-roadmap.md#m7-1b).

Written for [M7.1a](../decisions-and-roadmap.md#m7-1a) (the record, the survey of the sample
logs, and the PerfectFlite, AltOS and Raven readers), as
[ADR-251](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0251-flight-log-importers-in-two-increments.md) decided.

## The layouts read

| Layout | Loggers | Recognized by its first line | Reader |
|---|---|---|---|
| PerfectFlite `.pf2` | Pnut, StratoLogger, StratoLoggerCF | names PerfectFlite | [`.pf2` files](pf2.md) |
| PerfectFlite CSV export | the same, through PerfectFlite's software | `Time,Altitude,` and more of the export's own column names | [below](#perfectflites-exports) |
| PerfectFlite text export | the same | the same names, separated by tabs | [below](#perfectflites-exports) |
| AltOS CSV | Altus Metrum's TeleMetrum, TeleMega, EasyMega, EasyMini, through AltosUI | `#version,serial,flight,` (or `version,` after `#` comments) | `hpr_flightdata::altos` |
| Raven FIP CSV | Featherweight Raven, through the Featherweight Interface Program | holds the Raven's `Axial Accel (Gs)` channel | `hpr_flightdata::raven` |

The other layouts in the sample logs, among them the Blue Raven's, Eggtimer's, the RRC3's, AltOS's
binary `.eeprom` and spreadsheets, are refused today as *unrecognized*.

## The record

Every reader fills the same record, `FlightLog`, in SI units:

- **What the file says about itself:** the logger's name, serial number, firmware and flight
  number where the file states them, and figures the logger worked out itself, such as its apogee,
  kept beside HPR Sim's readings and never in place of them.
- **The primary track:** the barometric altitude above the pad on the logger's clock, with the
  vertical speed, temperature and battery voltage where they share that clock. The
  [readings](../physics/log-readings.md) are taken from it.
- **Every column, as a channel:** each data column of the file, in the file's order, with the
  name the file gives it, what it measures, and its own clock. Nothing the logger wrote is dropped:
  a column HPR Sim can't name a quantity for is kept as written and marked *unrecognized*.
- **Events:** what the logger says happened and when, such as AltOS's change from `boost` to
  `coast`.
- **Notes:** what the reader noticed and worked around.

A channel's unit follows from what it measures:

| Quantity | Unit | Examples |
|---|---|---|
| altitude above the pad, above sea level, from GNSS, or integrated from the accelerometer | m | `height`, `[Altitude (Baro-Ft-AGL)]` |
| pressure | Pa | `pressure`, `Baro (Atm)` |
| speed | m/s | `speed`, `Velocity (Accel-Ft/Sec)` |
| acceleration | m/s² | `acceleration`, `Axial Accel (Gs)` |
| temperature | K | `temperature`, `Temperature (F)` |
| voltage, current | V, A | `battery_voltage`, `Current Draw (A)` |
| latitude, longitude, an angle | degrees | `latitude`, `pad_az` |
| distance | m | `pad_dist` |
| a number with no unit | none | a count of satellites, a signal strength, a state code |

A foot is 0.3048 m (1.000 ft), a g 9.806 65 m/s² and an atmosphere 101 325 Pa, each exactly. A cell the file
leaves empty, or fills with something that isn't a number, is a gap, `NaN` in the record and `null`
in its JSON.

### A worked example

PerfectFlite's CSV export of a flight starts like this (an invented flight):

```text
Time,Altitude,Velocity,Temperature (F),Voltage,Acceleration (calculated)
0.00,0,0,71.5,9.1,-3
0.50,0,0,71.5,9.1,-3
1.00,0,88,71.4,9.1,2
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
unit, and two checks on the sample logs bear out feet: each CSV export with its flight's `.pf2`
beside it peaks at that file's height, and every export's climb to apogee takes about as long as
a fall from that many feet would, where a fall from that many meters would take 1.8 times longer
than the climb did. An export doesn't name the altimeter's model, so its logger reads
`PerfectFlite`.

A header with any other name, or these names in another case, isn't read as PerfectFlite's: the
RRC3's `Time,Altitude,Pressure,…` and a bare `time,altitude` say nothing of their units.

## How it was checked

`cargo xtask logs` reads every sample log the project holds: the private collection's catalog of
flight logs, the public fixtures and samples of Debrief, and AerospaceNU's published flight logs,
each distinct file once. It writes what came of each to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md):
per layout, how many files were read and how many refused and why, and per file an anonymized id
with a digest of the record read from it. The collection is other people's data, Debrief's fixtures come from
flights shared on terms nobody recorded, and AerospaceNU's are published under the GPL, so no file
and no value read from one is committed; the digest pins each reading instead, as a snapshot would, and the check
fails if a reader's output changes or a file of these layouts stops reading. Invented files in the
crate's tests are read in CI and pinned whole by snapshots.
