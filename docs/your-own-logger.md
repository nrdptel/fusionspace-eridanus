# Reading your own logger

Read a logger FusionSpace HPR doesn't know by a JSON file you write: a column mapping for a table,
a record layout for binary records.

This page is for a flyer whose altimeter or flight computer writes a file no reader here knows:
a board they built that logs to a card, a team's flight computer, or a commercial logger whose
format isn't read yet. FusionSpace HPR · Analyzer, the command line and the Python package read
such a file by the mapping or layout you write, into the same
[record](format/flight-logs.md#the-record) every other log reads into: each column of numbers but
the clock and those you skip a channel on the logger's clock, its values in SI units (degrees for
angles) where your mapping or layout states a unit, and, where a mapping marks one, the barometric
altitude above the pad as the primary track, the one the readings are taken from. Every
example below reads one invented flight's files, its logger's and its GPS tracker's
([evidence row G11](evidence.md#g11)); it flew
nowhere.

> **How far to trust it.** The record is the logger's own samples, converted to SI units (degrees
> for angles) by the units your mapping or layout states: it is as right as those are. The
> reader refuses a unit that doesn't measure its column's quantity, and keeps as written a column
> whose values a unit makes impossible, but a wrong unit whose values stay plausible reads in it.
> The tests read this page's files, and each sample log the project reads by a stated mapping or
> layout where the samples are present (they are other people's, so CI reads this page's only),
> from the command line, from Python and from the Rust library, and hold the three records equal,
> channel by channel. The readings (liftoff, apogee, the top speed, landing) are taken from these
> logs as from any other, each within a bound held on HPR Sim's synthetic flights
> ([M7.2b, readings on every logger](decisions-and-roadmap.md#m7-2b)), but those bounds hold only
> if the units are right. Check a unit against a known height, such as the launch site's elevation
> from a GPS altitude, before trusting it, and go by the logger's own figures for a record.

## The logger

The examples read one made-up flight, as a home-built logger might record it: a clock in
milliseconds, its altitude above the pad in feet, the air pressure in pascals, and an
accelerometer along the rocket's axis in g. The rocket peaks at about 1,345.7 ft, 10.8 s after the
logger starts; its highest sample, 10.6 s in, reads 1,345.2 ft. The logger writes the same
samples three ways, each in
[`validation/fixtures/logs/own-logger/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/fixtures/logs/own-logger),
written by its `make.py`:

- `flight.csv`, a table whose header names each column but states only some units. The altitude
  column, `Alt`, says nothing of its unit.
- `flight.bin`, the same samples as binary records of 16 bytes: the clock as a 32-bit unsigned
  integer, then the altitude, the pressure and the acceleration as 32-bit floats, little-endian.
- `flight-capture.txt`, those records as the logger prints them over its USB serial console:
  base64 lines between two marker lines that name the block, `flight`.

A GPS tracker beside it writes `gps-track.csv`, read in [A GPS tracker's table](#a-gps-trackers-table).

## A table, by a column mapping

A column mapping says how a table's columns are read: the header's line, the separator, the
clock's column, and what each column measures in which unit. You rarely write one from nothing:
`--print-mapping` prints what the header states, for you to edit. The invented table starts with
a comment, so its header is on line 2:

```text
# An invented flight, written as a flyer's own logger would write it
Time (ms),Alt,Pressure (Pa),Accel X (g)
0,0.0,87823,1.00
```

### What the header states

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight.csv --print-mapping`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight.csv --print-mapping
{
  "header_line": 2,
  "separator": ",",
  "decimal_comma": false,
  "clock": 0,
  "columns": [
    {
      "name": "Time (ms)",
      "quantity": null,
      "unit": "ms"
    },
    {
      "name": "Alt",
      "quantity": null,
      "unit": null
    },
    {
      "name": "Pressure (Pa)",
      "quantity": "pressure",
      "unit": "pa"
    },
    {
      "name": "Accel X (g)",
      "quantity": "acceleration",
      "unit": "g"
    }
  ]
}
note: the column "Alt" states no unit the reader knows: kept as written
help: save it to a file and check each column's quantity and unit; set "primary" to the column, from 0, that is the barometric altitude above the pad, mapped to altitude_above_ground; then read the log with --mapping and that file
```

<!-- cli: end -->

The header states the clock's unit, `ms`, and the pressure's and the acceleration's. It states
none for `Alt`, so the reader keeps that column as written, `"quantity": null`, and a note says
why. A column it can't read stays in the record, its numbers unconverted.

### The mapping, edited

Save the printed mapping to a file, `hpr analyze flight.csv --print-mapping > flight.mapping.json`,
and give `Alt` what the header leaves out: its quantity, `altitude_above_ground`, and its unit,
`ft`. Then mark it as the primary track, `"primary": 1`, its column counted from 0: the barometric
altitude above the pad that the readings are taken from. A mapping or layout file may be
UTF-8, or UTF-16 with its byte-order mark, as Windows PowerShell 5.1 saves what `>` sends it. The
file the folder holds:

<!-- quote: validation/fixtures/logs/own-logger/flight.mapping.json -->
```json
{
  "header_line": 2,
  "separator": ",",
  "decimal_comma": false,
  "clock": 0,
  "primary": 1,
  "columns": [
    {
      "name": "Time (ms)",
      "quantity": null,
      "unit": "ms"
    },
    {
      "name": "Alt",
      "quantity": "altitude_above_ground",
      "unit": "ft"
    },
    {
      "name": "Pressure (Pa)",
      "quantity": "pressure",
      "unit": "pa"
    },
    {
      "name": "Accel X (g)",
      "quantity": "acceleration",
      "unit": "g"
    }
  ]
}
```

Each field, as the
[mapping's schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-mapping.schema.json)
lists them:

| field | what it says |
|---|---|
| `header_line` | the header's line, counted from 1; `null` for a file with no header row |
| `separator` | the character between cells: a comma, `;`, a tab, `\|` or a space |
| `decimal_comma` | whether a comma is the decimal mark, as in `1,5`; only beside another separator |
| `clock` | the clock's column, from 0; its unit must be one of time, such as `s`, `ms` or `us` |
| `primary` | the primary track's column, from 0, mapped to `altitude_above_ground`; left out for a log with none |
| `columns` | every column in the file's order: its `name`, the `quantity` it measures (`null` keeps it as written), its `unit`, and `"skip": true` to leave a column out |
| `clock_wrap` | for a counter that runs over, the count it turns over at, in the clock's unit; left out for a clock that doesn't |
| `rows` | how a row is written: `cells`, the default; `list`, as `[1, 2.5]`; or `stamped`, a date and time first |

A quantity sets the unit the record holds the column in: SI, but degrees for angles. The
quantities are `altitude_above_ground`, `altitude_msl`, `gnss_altitude`, `inertial_altitude` and
`altitude` (an altitude of unstated zero), `pressure`, `speed`, `acceleration`, `temperature`,
`voltage`, `current`, `latitude`, `longitude`, `angle`, `angular_rate`, `distance`,
`time_of_day`, `duration` and `number`, a count or a code with no unit; `unrecognized` is the
same as `null`. The units a mapping may state are the ones a header may
([Tables read by their headers](format/flight-logs.md#tables-read-by-their-headers)).

A mapping file that doesn't follow the schema is refused before the log is read, naming the field
by its path in the JSON, and its line and column. A unit written as `3` instead of `"ft"` says
``column mapping: `columns[1].unit`: invalid type: integer `3`, expected a string at line 16 column 15``,
and a misspelled key, such as `primry`, is refused rather than left out. One the log contradicts
is refused naming the column: a primary track on the clock's column, or a unit that doesn't
measure its column's quantity.

### The log, read by it

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight.csv --mapping validation/fixtures/logs/own-logger/flight.mapping.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight.csv --mapping validation/fixtures/logs/own-logger/flight.mapping.json
flight.csv: read by the mapping flight.mapping.json
251 samples, from 0.00 s to 50.00 s
readings from the logger's barometric altitude, sampled every 0.200 s in steps of 0.030 m, after a 0.40 s running median
3 channels: Alt (altitude above ground), Pressure (Pa) (pressure), Accel X (g) (acceleration)

liftoff           1.20 s, within 1.04 s
apogee            409.5 m (1344 ft) above the pad, within 33.0 m (108 ft); at 10.50 s, within 0.99 s, 9.30 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.23 s; 47.40 s after liftoff, within 3.28 s; 38.10 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.0 m/s (7 ft/s)
top acceleration  withheld: HPR Sim doesn't take the top acceleration from an accelerometer yet: roadmap milestone M7.2c does

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping or layout
states, not simulated: as right as those units are. Each reading states the bound its error stayed
within on HPR Sim's synthetic flights, logged at every rate from 500 to 0.5 samples a second and in
steps up to 10 m or 100 Pa, in air within 15 K of standard; a reading the log can't support is
withheld, with the reason. The top speed is derived from the heights, and withheld from Mach 0.8,
where a barometer's port may read the flow over the nose. Those bounds hold only if the units are
right: check a unit against a known height, such as the launch site's elevation from a GPS altitude,
and go by the logger's own figures for a record.
More: https://hpr.fusionspace.co/your-own-logger.html
```

<!-- cli: end -->

The primary track, `Alt` in meters, holds every row; it is a channel too, beside the pressure
and the acceleration, and the clock is each channel's `time_s`. The readings come from it, each
with the bound its error stays within
([Flight-log readings](physics/log-readings.md#what-each-bound-holds)). The invented flight peaks
at 1,345.7 ft between samples, within the apogee's bound of the reading. Two readings are withheld,
each with the reason: the top speed, as a sample every 0.2 s is too few for its 0.5 s fit, and the
top acceleration, which HPR Sim doesn't take from an accelerometer yet
([M7.2c, the rest of the readings](decisions-and-roadmap.md#m7-2c)). `--json` prints the same as
one JSON document. `--record` prints the whole record instead, every sample of every channel in SI units
(degrees for angles),
as one JSON document whose
[schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-record.schema.json)
lists its fields:

```bash
hpr analyze flight.csv --mapping flight.mapping.json --record > flight.json
```

A table on a sheet of an Excel `.xlsx` workbook is read the same way, its sheet named from 0 with
`--sheet`, for `--print-mapping` and `--mapping` alike.

## Binary records, by a record layout

A record layout says how a file of binary records is laid out: each field's name and kind, in the
record's order, and the byte order. A field's name states its unit as a table's header does, such
as `Time (ms)` or `pressure__pa`: the records are written out as a table under those names and
read by what they state. So one field must be a clock in a unit of time. The invented logger's
layout:

<!-- quote: validation/fixtures/logs/own-logger/flight.layout.json -->
```json
{
  "fields": [
    { "name": "Time (ms)", "kind": "u32" },
    { "name": "Alt (ft)", "kind": "f32" },
    { "name": "Pressure (Pa)", "kind": "f32" },
    { "name": "Accel X (g)", "kind": "f32" }
  ]
}
```

A field's `kind` is one of `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64` (unsigned and
signed integers of 8 to 64 bits), `f32` or `f64`; `"byte_order": "big"` reads big-endian records,
and little-endian is the default
([the layout's schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-layout.schema.json)):

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight.bin --layout validation/fixtures/logs/own-logger/flight.layout.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight.bin --layout validation/fixtures/logs/own-logger/flight.layout.json
flight.bin: read by the layout flight.layout.json
no primary track: no barometric altitude above the pad
readings from the pressure, by the 1976 standard atmosphere, sampled every 0.200 s in steps of 0.093 m, after a 0.40 s running median
3 channels: Alt (ft) (altitude), Pressure (Pa) (pressure), Accel X (g) (acceleration)

liftoff           1.20 s, within 1.05 s
apogee            409.5 m (1344 ft) above the pad, within 33.2 m (109 ft); at 10.60 s, within 0.99 s, 9.40 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.27 s; 47.40 s after liftoff, within 3.32 s; 38.00 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.0 m/s (7 ft/s)
top acceleration  withheld: HPR Sim doesn't take the top acceleration from an accelerometer yet: roadmap milestone M7.2c does

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping or layout
states, not simulated: as right as those units are. Each reading states the bound its error stayed
within on HPR Sim's synthetic flights, logged at every rate from 500 to 0.5 samples a second and in
steps up to 10 m or 100 Pa, in air within 15 K of standard; a reading the log can't support is
withheld, with the reason. The top speed is derived from the heights, and withheld from Mach 0.8,
where a barometer's port may read the flow over the nose. Those bounds hold only if the units are
right: check a unit against a known height, such as the launch site's elevation from a GPS altitude,
and go by the logger's own figures for a record.
More: https://hpr.fusionspace.co/your-own-logger.html
note: 251 binary records read by the caller's layout, written out as a table and read by its header
```

<!-- cli: end -->

A name says a unit, not where the zero is, so `Alt (ft)` reads as an altitude of unstated zero,
and the records have no primary track: a layout can't mark one yet. So the readings come from the
pressure, turned into a height by the 1976 standard atmosphere and zeroed on the pad, as the line
`readings from the pressure` says. The serial console's capture
holds the same records, read by the same layout with one more field, the block's name:
`"block": "flight"`. A capture read by a layout that names no block is refused, its blocks listed.

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight-capture.txt --layout validation/fixtures/logs/own-logger/flight-capture.layout.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight-capture.txt --layout validation/fixtures/logs/own-logger/flight-capture.layout.json
flight-capture.txt: read by the layout flight-capture.layout.json
no primary track: no barometric altitude above the pad
readings from the pressure, by the 1976 standard atmosphere, sampled every 0.200 s in steps of 0.093 m, after a 0.40 s running median
3 channels: Alt (ft) (altitude), Pressure (Pa) (pressure), Accel X (g) (acceleration)

liftoff           1.20 s, within 1.05 s
apogee            409.5 m (1344 ft) above the pad, within 33.2 m (109 ft); at 10.60 s, within 0.99 s, 9.40 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.27 s; 47.40 s after liftoff, within 3.32 s; 38.00 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.0 m/s (7 ft/s)
top acceleration  withheld: HPR Sim doesn't take the top acceleration from an accelerometer yet: roadmap milestone M7.2c does

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping or layout
states, not simulated: as right as those units are. Each reading states the bound its error stayed
within on HPR Sim's synthetic flights, logged at every rate from 500 to 0.5 samples a second and in
steps up to 10 m or 100 Pa, in air within 15 K of standard; a reading the log can't support is
withheld, with the reason. The top speed is derived from the heights, and withheld from Mach 0.8,
where a barometer's port may read the flow over the nose. Those bounds hold only if the units are
right: check a unit against a known height, such as the launch site's elevation from a GPS altitude,
and go by the logger's own figures for a record.
More: https://hpr.fusionspace.co/your-own-logger.html
note: read from the block `flight` of a console capture, line 2: 4016 bytes of base64 decoded
note: 251 binary records read by the caller's layout, written out as a table and read by its header
```

<!-- cli: end -->

## From Python

`FlightLog.read` reads a log by its format's own reader, a mapping or a layout, each a dict or the
path of its JSON file, into the record the command line prints with `--record`.
`FlightLog.header_mapping` gives what a table's header states, as a dict to edit. This reads the
invented table by its header's mapping, edited in Python as above, then its records by their
layout:

<!-- python: example -->
```python
import numpy as np
from fusionspace import hpr

folder = "validation/fixtures/logs/own-logger"
mapping = hpr.FlightLog.header_mapping(f"{folder}/flight.csv")
mapping["columns"][1]["quantity"] = "altitude_above_ground"
mapping["columns"][1]["unit"] = "ft"
mapping["primary"] = 1
log = hpr.FlightLog.read(f"{folder}/flight.csv", mapping=mapping)
print(log)
top = np.nanmax(log.altitude_m)  # a gap is NaN
print(f"highest sample {top:.2f} m ({top / 0.3048:.1f} ft) above the pad")
for channel in log.channels:
    print(f"{channel['column']}: {channel['quantity']} in {channel['unit']}")
records = hpr.FlightLog.read(f"{folder}/flight.bin", layout=f"{folder}/flight.layout.json")
print(records)
```

```text
FlightLog(generic_table, 3 channels, 251 samples on the primary track)
highest sample 410.02 m (1345.2 ft) above the pad
Alt: altitude_above_ground in m
Pressure (Pa): pressure in Pa
Accel X (g): acceleration in m/s²
FlightLog(binary_records, 3 channels, 0 samples on the primary track)
```
<!-- python: end -->

The records' log has no samples on its primary track, as a layout marks none. Each channel's
`time_s` and `values` are NumPy arrays, a gap NaN; `log.to_dict()` is the whole
record as `hpr analyze --record` writes it, without its `tool`, a gap `None`. A mapping or a
layout that fails raises `HprError` naming the field or the column, in the command line's words.

`log.readings()` takes the readings, as `hpr analyze` does, and returns them as a dict: under
`liftoff`, `apogee`, `max_speed`, `max_acceleration` and `landing`, each reading with its
`status`, `"read"` or `"withheld"`. A reading read holds its value, its bound (`bound_m`,
`bound_s` or `bound_m_s`), its `source`, its `provenance` and its `method`; one withheld holds its
`reason` and a `detail` in words. Beside them, the track's `source`, `sample_interval_s` and
`step_m`, the median's `median_window_s` and `peak_bound_m`, and `pad_altitude_m` say how they were
taken. Each code is the one the JSON output uses
([Flight-log readings](physics/log-readings.md#when-a-reading-is-withheld)). The table's log, read
above:

<!-- python: example -->
```python
readings = log.readings()
apogee = readings["apogee"]
print(f"apogee {apogee['height_m']:.1f} m above the pad, within {apogee['bound_m']:.1f} m")
print(f"from the {apogee['source']}, {apogee['provenance']}, by {apogee['method']}")
speed = readings["max_speed"]
print(f"top speed {speed['status']}: {speed['reason']}")
```

```text
apogee 409.5 m above the pad, within 33.0 m
from the barometer, measured, by highest_filtered
top speed withheld: sampled_too_slowly
```
<!-- python: end -->

## A GPS tracker's table

A table of GPS fixes, with no barometer, reads by a mapping too. The invented tracker writes its
clock in seconds, its position in degrees and its GPS altitude in meters, once a second. Its
header states the clock's and the altitude's units, not the position's, nor what `Sats` counts,
so its mapping states them:

<!-- quote: validation/fixtures/logs/own-logger/gps-track.mapping.json -->
```json
{
  "header_line": 1,
  "separator": ",",
  "decimal_comma": false,
  "clock": 0,
  "columns": [
    {
      "name": "Time (s)",
      "quantity": null,
      "unit": "s"
    },
    {
      "name": "Lat",
      "quantity": "latitude",
      "unit": "deg"
    },
    {
      "name": "Lon",
      "quantity": "longitude",
      "unit": "deg"
    },
    {
      "name": "GPS Alt (m)",
      "quantity": "gnss_altitude",
      "unit": "m"
    },
    {
      "name": "Sats",
      "quantity": "number",
      "unit": null
    }
  ]
}
```

A GPS altitude is above sea level as the receiver states it, not above the pad, so the mapping
marks no primary track, and the readings are withheld, each by name: a receiver's altitude lags a
climb and overshoots its top, so no bound can be stated for it:

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/gps-track.csv --mapping validation/fixtures/logs/own-logger/gps-track.mapping.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/gps-track.csv --mapping validation/fixtures/logs/own-logger/gps-track.mapping.json
gps-track.csv: read by the mapping gps-track.mapping.json
no primary track: no barometric altitude above the pad
4 channels: Lat (latitude), Lon (longitude), GPS Alt (m) (GNSS altitude), Sats (number)

liftoff           withheld: the log's only height is a GNSS receiver's, whose fix drops out or lags at liftoff
apogee            withheld: the log's only height is a GNSS receiver's, whose navigation filter lags a climb and overshoots its top by more than a barometer's bounds allow
top speed         withheld: the log's only height is a GNSS receiver's, whose navigation filter lags a climb and overshoots its top by more than a barometer's bounds allow
landing           withheld: landing is found against the pad after apogee, and liftoff and apogee were withheld
top acceleration  withheld: the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping or layout
states, not simulated: as right as those units are. Each reading states the bound its error stayed
within on HPR Sim's synthetic flights, logged at every rate from 500 to 0.5 samples a second and in
steps up to 10 m or 100 Pa, in air within 15 K of standard; a reading the log can't support is
withheld, with the reason. The top speed is derived from the heights, and withheld from Mach 0.8,
where a barometer's port may read the flow over the nose. Those bounds hold only if the units are
right: check a unit against a known height, such as the launch site's elevation from a GPS altitude,
and go by the logger's own figures for a record.
More: https://hpr.fusionspace.co/your-own-logger.html
```

<!-- cli: end -->

## What it doesn't do yet

- Take the top acceleration from a log's accelerometer, as from this logger's `Accel X`:
  [M7.2c, the rest of the readings](decisions-and-roadmap.md#m7-2c).
- Mark a primary track in a record layout. A mapping marks one; binary records' altitude reads as
  one of unstated zero.
- Read a table from an older Excel `.xls` workbook by a mapping: `--sheet` reads `.xlsx` only.
- Read an `.xls` export, a zip archive's files, or a workbook's sheet but by a mapping, from the
  command line or Python. Both read every other layout the library tells from the file alone by
  its own reader ([the layouts read](format/flight-logs.md#the-layouts-read)), and another
  logger's table or binary records by a mapping or a layout.
