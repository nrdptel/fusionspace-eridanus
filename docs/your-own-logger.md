# Reading your own logger

Read a logger FusionSpace HPR doesn't know by a JSON file you write: a column mapping for a table,
a record layout for binary records.

This page is for a flyer whose altimeter or flight computer writes a file no reader here knows.
Theirs can be a board they built that logs to a card, a team's flight computer, or a commercial
logger whose format isn't read yet.

FusionSpace HPR · Analyzer, the command line and the Python package read such a file by the
mapping or layout you write. The file reads into the same
[record](format/flight-logs.md#the-record) as every other log. Each column of numbers but
the clock and those you skip becomes a channel on the logger's clock. A channel's values are in SI
units (degrees for angles) where your mapping or layout states a unit. A mapping can mark the
barometric altitude above the pad as the primary track, the one the readings are taken from.

The examples read invented files. All but the FIP export and the workbook `two-flights.xlsx`
are one flight's, its logger's and its GPS tracker's
([the evidence register's row on these files](evidence.md#g11)).

> **How far to trust it.** The record is the logger's own samples, converted by the units your
> mapping or layout states. It's as right as those units are.
>
> The reader refuses a unit that doesn't measure its column's quantity. It keeps as written a
> column whose values a unit makes impossible. But a column with a wrong unit whose values stay
> plausible is read in that unit.
>
> The tests read this page's files from the command line, Python and the Rust library. They do
> the same for each sample log the project reads by a stated mapping or layout, where present.
> The samples are other people's, so CI reads this page's only. The tests hold the three records
> equal, channel by channel.
>
> The readings (liftoff, apogee, the top speed, landing) are taken from these logs as from any
> other. Each reading carries a bound held on HPR Sim's synthetic flights
> ([readings on every logger](decisions-and-roadmap.md#m7-2b)). But those bounds hold only if the
> units are right. Check a unit against a known height, such as the launch site's elevation from a
> GPS altitude, before trusting it. For a record, go by the logger's own figures.

## The logger

The logger's examples read one made-up flight, as a home-built logger might record it. The
logger records a clock in milliseconds and its altitude above the pad in feet. It records the
air pressure in pascals, and an accelerometer along the rocket's axis in g. The rocket peaks at
about 1,345.7 ft, 10.8 s after the logger starts; its highest sample, 10.6 s in, reads
1,345.2 ft. The logger writes the same samples three ways, each in
[`validation/fixtures/logs/own-logger/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/fixtures/logs/own-logger),
written by its `make.py`:

- `flight.csv`, a table whose header names each column but states only some units.
- `flight.bin`, the same samples as binary records of 16 bytes. Each holds the clock as a 32-bit
  unsigned integer, then the altitude, the pressure and the acceleration as 32-bit floats,
  little-endian.
- `flight-capture.txt`, those records as the logger prints them over its USB serial console:
  base64 lines between two marker lines that name the block, `flight`.

A GPS tracker beside it writes `gps-track.csv`, read in [A GPS tracker's table](#a-gps-trackers-table).

## A table, by a column mapping

A column mapping says how a table's columns are read. It gives the header's line, the separator,
the clock's column, and what each column measures in which unit. You rarely write one from
nothing: `--print-mapping` prints what the header states, for you to edit. The invented table
starts with a comment, so its header is on line 2:

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
help: save it to a file and check each column's quantity and unit. Set "primary" to the column, from 0, that is the barometric altitude above the pad, mapped to altitude_above_ground. Then read the log with --mapping and that file
```

<!-- cli: end -->

The header states the clock's unit, `ms`, and the pressure's and the acceleration's. It states
none for `Alt`, so the reader keeps that column as written, `"quantity": null`, and a note says
why. A column it can't read stays in the record, its numbers unconverted.

### The mapping, edited

Save the printed mapping to a file, `hpr analyze flight.csv --print-mapping > flight.mapping.json`,
and give `Alt` what the header leaves out: its quantity, `altitude_above_ground`, and its unit,
`ft`. Then mark it as the primary track, `"primary": 1`, its column counted from 0. A mapping or
layout file may be UTF-8, or UTF-16 with its byte-order mark, as Windows PowerShell 5.1 saves what
`>` sends it. The file the folder holds:

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
quantities for an altitude are `altitude_above_ground`, `altitude_msl`, `gnss_altitude`,
`inertial_altitude` and `altitude` (an altitude of unstated zero). The others are `pressure`,
`speed`, `acceleration`, `temperature`, `voltage`, `current`, `latitude`, `longitude`, `angle`,
`angular_rate`, `distance`, `time_of_day`, `duration` and `number`. A `number` is a count or a
code with no unit.

`unrecognized` is the same as `null`. The units a mapping can state are the ones a header can
([Tables read by their headers](format/flight-logs.md#tables-read-by-their-headers)).

A mapping file that doesn't follow the schema is refused before the log is read. The refusal names
the field by its path in the JSON, and its line and column. A unit written as `3` instead of
`"ft"` says
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

liftoff           1.20 s, within 1.05 s
apogee            409.5 m (1344 ft) above the pad, within 33.1 m (109 ft); at 10.50 s, within 0.99 s, 9.30 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.24 s; 38.10 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.1 m/s (7 ft/s)
flight time       47.40 s, within 3.28 s
drogue descent    19.3 m/s (63 ft/s), within 1.9 m/s (7 ft/s), from apogee at 10.50 s to the main's opening at 24.00 s, 409.5 m (1344 ft) to 148.7 m (488 ft) up
main descent      6.0 m/s (20 ft/s), within 0.7 m/s (3 ft/s), from the main's opening at 24.00 s, within 1.36 s, to landing at 48.60 s, 148.7 m (488 ft) to 1.0 m (3 ft) up
top acceleration  withheld: the log's accelerometers (`Accel X (g)`) don't say which lies along the rocket's axis
burnout           withheld: as for the top acceleration; and burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping, layout or
FIP header states, not simulated. They are as right as those units are. Each reading states the
bound its error stayed within on HPR Sim's synthetic flights. They cover rates from 500 to 0.5
samples a second, steps up to 10 m or 100 Pa, and air within 15 K of standard. A reading the log
can't support is withheld, with the reason. The top speed is derived from the heights, and withheld
from Mach 0.8, where a barometer's port may read the flow over the nose. Those bounds hold only if
the units are right. Check a unit against a known height, such as the launch site's elevation from a
GPS altitude. For a record, go by the logger's own figures.
More: https://hpr.fusionspace.co/your-own-logger.html
```

<!-- cli: end -->

The primary track, `Alt` in meters, holds every row. It's a channel too, beside the pressure and the
acceleration, and the clock is each channel's `time_s`. Each reading comes with the bound its error
stays within ([Flight-log readings](physics/log-readings.md#what-each-bound-holds)). The invented
flight peaks at 1,345.7 ft between samples, within the apogee's bound of the reading. Three readings
are withheld, each with the reason:

- the top speed, as a sample every 0.2 s is too few for its 0.5 s fit;
- the top acceleration, as the accelerometer `Accel X (g)` doesn't say it lies along the rocket's
  axis;
- and burnout, which then needs the top speed.

A column is read as the rocket's axis only where its logger's reader names it so or its name holds
the word `axial` ([the accelerometer](physics/log-readings.md#the-accelerometer)). A column's name
can be the header's or the mapping's. `--json` prints the same as one JSON document. `--record`
prints the whole record instead, every sample of every channel, as one JSON document. The record's
[schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-record.schema.json)
lists its fields:

```bash
hpr analyze flight.csv --mapping flight.mapping.json --record > flight.json
```

A table on a workbook's sheet is read the same way. The workbook can be an `.xlsx`, or an `.xls`
written as one bare worksheet. Its sheet is named from 0 with `--sheet`, for `--print-mapping` and
`--mapping` alike; a workbook of one sheet needs none. A sheet one of the
library's readers takes needs no mapping: `--sheet` alone reads it by that reader
([A file of several records](cli.md#a-file-of-several-records)).

### A FIP export below no header

A Featherweight Raven's FIP export, the CSV its Featherweight Interface Program writes, reads by
its own reader, but only with its header line. When it's pasted into a workbook below no header,
nothing names its columns. So you state that line in a JSON file, a FIP header, and give it with
`--fip-header`
([its schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-fip-header.schema.json)).
It has two keys:

- `header`, the header line the program writes for an export of the same channels: each
  channel's `Time@` clock, its value and its `bILBA` status cell.
- `pasted`, the columns, counted from 1, that another table pasted over the export covers. A
  channel with a cell in them, in any row, is left out whole, with a note. Leave it out when no
  table is pasted over.

A key the schema doesn't name is refused, naming it. The invented workbook
`validation/fixtures/logs/several/fip-below-no-header.xlsx` holds an export of three channels,
its header line gone. Its flight was made to peak at 273.5 m (897 ft) above the pad. The
workbook's FIP header states that line:

<!-- quote: validation/fixtures/logs/several/fip-below-no-header.header.json -->
```json
{
  "header": "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,Time@Baro (Atm),Baro (Atm),bILBA,Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)],bILBA"
}
```

<!-- cli: example `hpr analyze validation/fixtures/logs/several/fip-below-no-header.xlsx --fip-header validation/fixtures/logs/several/fip-below-no-header.header.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/several/fip-below-no-header.xlsx --fip-header validation/fixtures/logs/several/fip-below-no-header.header.json
fip-below-no-header.xlsx: Featherweight Raven
201 samples, from 0.00 s to 40.00 s
readings from the logger's barometric altitude, sampled every 0.200 s in steps of 0.337 m, after a 0.40 s running median
6 channels: Axial Accel (Gs) (acceleration), bILBA (number), Baro (Atm) (pressure), bILBA (number), [Altitude (Baro-Ft-AGL)] (altitude above ground), bILBA (number)

liftoff           1.40 s, within 1.10 s
apogee            273.0 m (896 ft) above the pad, within 23.9 m (79 ft); at 9.90 s, within 1.03 s, 8.50 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           37.40 s, within 1.46 s; 27.50 s from apogee, at 9.9 m/s (32 ft/s) on average, within 2.1 m/s (7 ft/s)
flight time       36.00 s, within 2.55 s
drogue descent    withheld: the descent shows no drogue's leg, which at the ground's density falls faster than 16.0 m/s up to the main's opening. That mark is 1.6 times the rate near the ground, 10.0 m/s, or 3 m/s more if that is more. The leg also falls on median faster than 1.4 times that rate. The rocket came down under one canopy, or under a drogue no faster than the mark, which reads as one. The main's descent is the whole descent's, from apogee
main descent      9.9 m/s (32 ft/s), within 1.0 m/s (4 ft/s), from apogee at 9.90 s to landing at 37.40 s, 273.0 m (896 ft) to 0.7 m (2 ft) up
top acceleration  withheld: the accelerometer `Axial Accel (Gs)`: no pad is read to tell by; no coast to apogee is read to tell by; its thrust says neither: as written, it never reads past 2 g either way up from liftoff's earliest time (the log's start where liftoff is withheld) to apogee: none of them says which way up it reads
burnout           withheld: as for the top acceleration; and burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping, layout or
FIP header states, not simulated. They are as right as those units are. Each reading states the
bound its error stayed within on HPR Sim's synthetic flights. They cover rates from 500 to 0.5
samples a second, steps up to 10 m or 100 Pa, and air within 15 K of standard. A reading the log
can't support is withheld, with the reason. The top speed is derived from the heights, and withheld
from Mach 0.8, where a barometer's port may read the flow over the nose. Those bounds hold only if
the units are right. Check a unit against a known height, such as the launch site's elevation from a
GPS altitude. For a record, go by the logger's own figures.
More: https://hpr.fusionspace.co/your-own-logger.html
note: read under a header line the caller stated, the rows' own header missing
```

<!-- cli: end -->

`--sheet N` picks the workbook's sheet, counted from 0; with none, the workbook must have only one.
The rows saved as text read too. A log is read by one of `--mapping`, `--layout` and
`--fip-header`.

The readings are marked as read by your own description, as a mapping's are. They're only as
right as the header you state.
[A FIP export pasted below no header](format/flight-logs.md#a-fip-export-pasted-below-no-header)
says how the rows are read.

## Binary records, by a record layout

A record layout says how a file of binary records is laid out. It gives each field's name and
kind, in the record's order, and the byte order.

A field's name states its unit as a table's header does, such as `Time (ms)` or `pressure__pa`.
The records are written out as a table under those names and read by what they state. So one
field must be a clock in a unit of time. The invented logger's layout:

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

A field's `kind` is one of `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `f32` or `f64`.
The first eight are unsigned and signed integers of 8 to 64 bits. `"byte_order": "big"` reads
big-endian records, and little-endian is the default
([the layout's schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze-layout.schema.json)):

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight.bin --layout validation/fixtures/logs/own-logger/flight.layout.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight.bin --layout validation/fixtures/logs/own-logger/flight.layout.json
flight.bin: read by the layout flight.layout.json
no primary track: each channel on its own clock
readings from the pressure, by the 1976 standard atmosphere, sampled every 0.200 s in steps of 0.093 m, after a 0.40 s running median
3 channels: Alt (ft) (altitude), Pressure (Pa) (pressure), Accel X (g) (acceleration)

liftoff           1.20 s, within 1.06 s
apogee            409.5 m (1344 ft) above the pad, within 33.2 m (109 ft); at 10.60 s, within 1.00 s, 9.40 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.27 s; 38.00 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.1 m/s (7 ft/s)
flight time       47.40 s, within 3.33 s
drogue descent    19.5 m/s (64 ft/s), within 2.0 m/s (7 ft/s), from apogee at 10.60 s to the main's opening at 24.00 s, 409.5 m (1344 ft) to 148.7 m (488 ft) up
main descent      6.0 m/s (20 ft/s), within 0.7 m/s (3 ft/s), from the main's opening at 24.00 s, within 1.36 s, to landing at 48.60 s, 148.7 m (488 ft) to 1.1 m (4 ft) up
top acceleration  withheld: the log's accelerometers (`Accel X (g)`) don't say which lies along the rocket's axis
burnout           withheld: as for the top acceleration; and burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping, layout or
FIP header states, not simulated. They are as right as those units are. Each reading states the
bound its error stayed within on HPR Sim's synthetic flights. They cover rates from 500 to 0.5
samples a second, steps up to 10 m or 100 Pa, and air within 15 K of standard. A reading the log
can't support is withheld, with the reason. The top speed is derived from the heights, and withheld
from Mach 0.8, where a barometer's port may read the flow over the nose. Those bounds hold only if
the units are right. Check a unit against a known height, such as the launch site's elevation from a
GPS altitude. For a record, go by the logger's own figures.
More: https://hpr.fusionspace.co/your-own-logger.html
note: 251 binary records read by the caller's layout, written out as a table and read by its header
```

<!-- cli: end -->

A name says a unit, not where the zero is, so `Alt (ft)` reads as an altitude of unstated zero.
The records have no primary track: a layout can't mark one yet. So the readings come from the
pressure, turned into a height by the 1976 standard atmosphere and zeroed on the pad.

The serial console's capture holds the same records, read by the same layout with one more field,
the block's name: `"block": "flight"`. A capture read by a layout that names no block is refused,
its blocks listed.

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/flight-capture.txt --layout validation/fixtures/logs/own-logger/flight-capture.layout.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/flight-capture.txt --layout validation/fixtures/logs/own-logger/flight-capture.layout.json
flight-capture.txt: read by the layout flight-capture.layout.json
no primary track: each channel on its own clock
readings from the pressure, by the 1976 standard atmosphere, sampled every 0.200 s in steps of 0.093 m, after a 0.40 s running median
3 channels: Alt (ft) (altitude), Pressure (Pa) (pressure), Accel X (g) (acceleration)

liftoff           1.20 s, within 1.06 s
apogee            409.5 m (1344 ft) above the pad, within 33.2 m (109 ft); at 10.60 s, within 1.00 s, 9.40 s after liftoff
top speed         withheld: the track's samples come every 0.200 s, too far apart for a 0.5 s fit of 5 samples, which needs 0.125 s
landing           48.60 s, within 2.27 s; 38.00 s from apogee, at 10.7 m/s (35 ft/s) on average, within 2.1 m/s (7 ft/s)
flight time       47.40 s, within 3.33 s
drogue descent    19.5 m/s (64 ft/s), within 2.0 m/s (7 ft/s), from apogee at 10.60 s to the main's opening at 24.00 s, 409.5 m (1344 ft) to 148.7 m (488 ft) up
main descent      6.0 m/s (20 ft/s), within 0.7 m/s (3 ft/s), from the main's opening at 24.00 s, within 1.36 s, to landing at 48.60 s, 148.7 m (488 ft) to 1.1 m (4 ft) up
top acceleration  withheld: the log's accelerometers (`Accel X (g)`) don't say which lies along the rocket's axis
burnout           withheld: as for the top acceleration; and burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping, layout or
FIP header states, not simulated. They are as right as those units are. Each reading states the
bound its error stayed within on HPR Sim's synthetic flights. They cover rates from 500 to 0.5
samples a second, steps up to 10 m or 100 Pa, and air within 15 K of standard. A reading the log
can't support is withheld, with the reason. The top speed is derived from the heights, and withheld
from Mach 0.8, where a barometer's port may read the flow over the nose. Those bounds hold only if
the units are right. Check a unit against a known height, such as the launch site's elevation from a
GPS altitude. For a record, go by the logger's own figures.
More: https://hpr.fusionspace.co/your-own-logger.html
note: read from the block `flight` of a console capture, line 2: 4016 bytes of base64 decoded
note: 251 binary records read by the caller's layout, written out as a table and read by its header
```

<!-- cli: end -->

## From Python

`FlightLog.read` reads a log into the record `hpr analyze --record` prints, by its format's own
reader, a mapping, a layout or a FIP header. The mapping, layout or FIP header is a dict or the
path of its JSON file. `FlightLog.header_mapping` gives what a table's header states, as a dict to
edit. This reads the invented table by its header's mapping, edited in Python as above, then its
records by their layout:

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
`time_s` and `values` are NumPy arrays, a gap NaN. `log.to_dict()` is the whole record as
`hpr analyze --record` writes it, without its `tool`, a gap `None`. A mapping or a layout that
fails raises `HprError` naming the field or the column, in the command line's words.

`log.readings()` returns the readings `hpr analyze` takes, as a dict. Its keys
are `liftoff`, `apogee`, `max_speed`, `max_acceleration`, `burnout`, `landing`, `flight_time`,
`drogue_descent` and `main_descent`. Each reading has its `status`, `"read"` or `"withheld"`.

A reading read holds its value and its bound. The bound is `bound_m` for a height, `bound_s` for a
time, `bound_m_s` for a speed or a descent rate, and `bound_m_s2` for the top acceleration. The
reading also holds its `source`, `provenance` and `method`. The apogee, the top
acceleration and burnout also say whether each is a [floor](glossary.md#floor), `is_floor`. The
top acceleration says why it clipped, `clip`: `"range"`, `"flat"` or `None`.

A reading withheld holds its `reason` and a `detail` in words. Beside the readings, these say how
they were taken:

- the track's `source`, `sample_interval_s` and `step_m`;
- the median's `median_window_s` and `peak_bound_m`;
- `pad_altitude_m`;
- and the `accelerometer` read, if any.

Each code is the one the JSON output uses
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

`fip_header=` reads a FIP export below no header, as the command line's `--fip-header` does.
`sheet=` picks a workbook's sheet, from 0, for a mapping, a FIP header or the sheet's own reader,
and `recording=` a raw dump's recording.

The same combinations are refused as on the command line. A log is read by one of `mapping=`,
`layout=` and `fip_header=`. `recording=` is refused with any of them or with `sheet=`, and
`sheet=` with `layout=`. This reads the invented FIP export above, then the second flight of the
workbook in [A file of several records](cli.md#a-file-of-several-records):

<!-- python: example -->
```python
folder = "validation/fixtures/logs/several"
fip = hpr.FlightLog.read(
    f"{folder}/fip-below-no-header.xlsx",
    fip_header=f"{folder}/fip-below-no-header.header.json",
)
print(fip)
print(fip.notes[0])
second = hpr.FlightLog.read(f"{folder}/two-flights.xlsx", sheet=2)
print(second.notes[0])
```

```text
FlightLog(raven_fip_csv, 6 channels, 201 samples on the primary track)
read under a header line the caller stated, the rows' own header missing
read from the sheet "Second" of an Excel workbook, its cells as stored
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

A GPS altitude is above sea level as the receiver states it, not above the pad. So the mapping
marks no primary track, and the readings are withheld, each by name. A receiver's altitude lags a
climb and overshoots its top, so no bound can be stated for it:

<!-- cli: example `hpr analyze validation/fixtures/logs/own-logger/gps-track.csv --mapping validation/fixtures/logs/own-logger/gps-track.mapping.json`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr analyze validation/fixtures/logs/own-logger/gps-track.csv --mapping validation/fixtures/logs/own-logger/gps-track.mapping.json
gps-track.csv: read by the mapping gps-track.mapping.json
no primary track: each channel on its own clock
4 channels: Lat (latitude), Lon (longitude), GPS Alt (m) (GNSS altitude), Sats (number)

liftoff           withheld: the log's only height is a GNSS receiver's, whose fix drops out or lags at liftoff
apogee            withheld: the log's only height is a GNSS receiver's, whose navigation filter lags a climb and overshoots its top by more than a barometer's bounds allow
top speed         withheld: the log's only height is a GNSS receiver's, whose navigation filter lags a climb and overshoots its top by more than a barometer's bounds allow
landing           withheld: landing is found against the pad after apogee, and liftoff and apogee were withheld
flight time       withheld: the flight time needs liftoff, which is withheld above
drogue descent    withheld, as landing is
main descent      withheld, as landing is
top acceleration  withheld: the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g
burnout           withheld: burnout from the speed's peak needs the top speed, which is withheld above

How far to trust it. The logger's own samples, read by HPR Sim in the units your mapping, layout or
FIP header states, not simulated. They are as right as those units are. Each reading states the
bound its error stayed within on HPR Sim's synthetic flights. They cover rates from 500 to 0.5
samples a second, steps up to 10 m or 100 Pa, and air within 15 K of standard. A reading the log
can't support is withheld, with the reason. The top speed is derived from the heights, and withheld
from Mach 0.8, where a barometer's port may read the flow over the nose. Those bounds hold only if
the units are right. Check a unit against a known height, such as the launch site's elevation from a
GPS altitude. For a record, go by the logger's own figures.
More: https://hpr.fusionspace.co/your-own-logger.html
```

<!-- cli: end -->

## What it doesn't do yet

- Take the top acceleration from an accelerometer whose column doesn't say it lies along the
  rocket's axis, as this logger's `Accel X` doesn't. It doesn't take one from a board's three axes
  either ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781)). A mapping has no
  field that marks a column as the axis yet.
- Mark a primary track in a record layout. Binary records' altitude reads as one of unstated zero.
- Read Excel's own `.xls`, a compound file of several streams, by a reader or a mapping. Only an
  `.xls` written as one bare worksheet, as an AltiMAX altimeter's export is, reads. Save any other
  as `.xlsx` or CSV.
- Read a zip archive's files, from the command line or Python: unpack it and give the log inside.
  Both read every other layout the library tells from the file alone by its own reader
  ([the layouts read](format/flight-logs.md#the-layouts-read)). They read a workbook's sheet or a
  raw dump's recording as chosen, and another logger's table or binary records by a mapping or a
  layout.
