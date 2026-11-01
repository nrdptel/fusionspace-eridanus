# Ghost files

See how to read and use the files `hpr analyze --ghost` writes: one flight's logs, on one clock, in a
frame at the pad.

A ghost file holds a flight's path for a viewer to replay: as JSON for your own program, as CZML for
Cesium, or as glTF for any 3D tool. This page covers what each holds, where each number comes from
and what the file assumes when a log can't say. It is for anyone who wants to see a flight in 3D or
load its path into a program. [The app's replay](../decisions-and-roadmap.md#m9-2) will read it too.
Today the file holds logs only. Drawing a simulated flight beside them is
[#1091](https://github.com/nrdptel/fusionspace-eridanus/issues/1091).

> **How far to trust it.** A ghost file lays out what a log holds and adds no measurement, so it is
> as good as the log. RocketPy's repository holds 22 real flight logs with an altitude or a GNSS
> track. Each gives a ghost file, and outside readers read each back
> ([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/ghost-files.md),
> [evidence row G37](../evidence.md#g37)). Nobody has yet looked at one in Cesium's globe or a
> modeling tool. For an apogee or a speed, go to [the readings](../reading-a-flight-log.md).

## Writing one

```bash
hpr analyze flight.csv --ghost flight.czml
hpr analyze telemetrum.csv telemega.csv --ghost flight.json --ghost flight.gltf
hpr analyze flight.csv --ghost flight.czml --pad 39.39,-8.29,120
```

The file's extension picks its format: `.json`, `.czml` or `.gltf`. Give `--ghost` again for another
format. The readings print as usual. `--pad` states the pad's latitude and longitude in degrees, then
its height in meters if you know it. It overrides what the log's fixes say. Each option of [`hpr
analyze`](../cli.md#hpr-analyze) that reads a log, such as `--mapping`, works with it.

## What a track holds

Each log gives one track, a list of moments, each with these numbers.

| Number | Where it comes from |
|---|---|
| Time, s | Counted from the log's [liftoff](../reading-a-flight-log.md), so that two logs of one flight share a clock. A log with no liftoff stands alone on its own clock, marked `"synced": false`. It can't join other logs in one file. |
| Up, m | The log's own altitude; for a log with none, its GNSS altitude less the pad's. Neither gives `null`. |
| East and north, m | The log's GNSS fixes in the frame at the pad, in a straight line between fixes. Both are `null` before the first fix and after the last. A log with no fixes has none. |

Why the log's altitude and not [the smoother's](../physics/smoother.md): on a barometer alone the
smoother can run far from the log through a sudden swing. On RocketPy's Lince flight, as its
recovery fires, it runs to 10,205 m, 6,618 m past the reading
([#1073](https://github.com/nrdptel/fusionspace-eridanus/issues/1073)). `hpr analyze`'s health card
says to go by the reading there, and a file for a viewer has no card. So a barometer's smaller
spikes stay in the height, and a receiver's lag on the climb stays in east and north.

## The pad

The pad is the frame's origin, on the WGS 84 ellipsoid ([the frame's
equations](../physics/geodesy.md)). It comes from the first of these a flight has:

- `--pad`;
- the median of the last 20 fixes before liftoff;
- the first fix, and how long after liftoff that was, where the log can tell.

A pad from the first fix may be off by what the rocket has already flown. With no fix at all, nothing
says where the flight was, and the file says that (below).

## What is left out

Each track counts what it leaves out, in its `fixes` and `heights_too_fast`. A fix goes when any of
these holds:

- it is empty, out of range, or within 0.01° of 0°N 0°E, which is a receiver's way of saying no fix;
- its clock didn't go forward;
- it lies more than 200 km from the track's median, as a longitude with its sign flipped does;
- reaching it from the last fix kept would take over 1,500 m/s, plus 200 m for a receiver's scatter.
  The envelope's top speed, Mach 3.5, is about 1,190 m/s at sea level.

A height goes on the same speed, plus 50 m. That catches a logger's garbage rows, such as the last
two of Juno III's altimeter file, at 10,700 m and −2,490 m. It also took 57 of the 44,454 rows of
Lince's height, in the swing as its recovery fires.

## The three formats

JSON follows [`schema/cli/ghost.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/ghost.schema.json).
Every number is the track's own, with `null` where a log can't say. Use this one for a program.

CZML is [Cesium's scene format](https://github.com/AnalyticalGraphicsInc/czml-writer/wiki/CZML-Guide).
Each track is an entity with an Earth-fixed position, a point and a path.

glTF is [Khronos's 3D scene format](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html), one
file with its numbers inside. Each track is a line along its whole path, with a marker 10 m across
moving along it. In meters, +x is east, +y is up and −z is north.

These last two have no `null`, and a drawing needs a number at every moment. So they draw something
where a log can't say, and each track's own text says what it drew:

| Where the log can't say | CZML and glTF draw |
|---|---|
| No fix yet, or no longer | The nearest fix. A log with no fixes is a straight vertical line over the pad. |
| No height | 0 |
| No place for the pad | CZML: 0°N 0°E. glTF has no globe, so it needs none. |

Each also has a limit of its own:

- CZML needs a date, and most logs hold none, so liftoff is always noon on 1 January 2000. The time
  on its clock is seconds from liftoff, not the day of the flight.
- glTF's times can't be negative, so its clock starts at the earliest moment. The file's
  `extras.first_moment_from_liftoff_s` says which that was.
- glTF stores 32-bit numbers, which stand about 0.06 ms apart at 1,000 s and 2 mm apart at 30 km. Two
  moments it can't tell apart become one.
- East, north and up are on the flat plane at the pad, which rises off the curved ground as you go.
  So a height is drawn about 31 m too high at 20 km downrange.

## A worked example

RocketPy's Prometheus flight has two boards. Reading them both:

```bash
hpr analyze 5115-TeleMetrum.csv 6583-TeleMega.csv --ghost prometheus.json
```

The TeleMetrum's liftoff, at 0.23 s on its clock, is 0 in the file. It holds 5,710 moments. Its height
peaks at 3,904 m, the apogee RocketPy's notebook states from the same column. It kept 5,712 of its
7,081 fixes: the other 1,369 rows repeat the time of the row before them. The fixes put the rocket at
most 1,170 m from the pad, and the fixes before liftoff put the pad at 1,390 m above sea level. The
TeleMega's track sits on the same clock and in the same frame, and peaks at 3,898 m.

## How it's checked

`cargo xtask ghost` writes each file of each of the 22 logs, then reads them back with readers this
project didn't write. CI runs the same on an invented flight, as `refs/` isn't there.

- Ajv holds the JSON to the schema.
- Cesium's own `CzmlDataSource` loads the CZML. Cesium's own transform takes each position to the
  pad's frame, where it comes within 1 cm of the JSON's. A long track is checked at about 2,000
  moments, a shorter one at each.
- Khronos's glTF-Validator finds no error in the glTF. glTF-Transform's animation comes within 1 cm
  and 1 ms of the JSON's at every moment.

So the formats agree with each other. What ties them to the log is pinned apart from that:

- Each of the 16 logs with a barometer's or a filtered height peaks within 2% of the apogee
  RocketPy's notebooks state, a check that each stated unit is right. Valkyrie's log, at 1.9% under,
  is the widest. A foot read as a meter would miss by a factor of 3.28. The two with only a
  receiver's altitude, on Juno III, peak 4.9% and 5.8% over its altimeter's card.
- East and north match the ellipsoid's radii of curvature, by a route that doesn't use the rotation
  under test, to 5 mm.
- The bytes of a small CZML and glTF file are worked out by hand from each format's spec.

## What it leaves out

- A simulated flight beside the real one ([#1091](https://github.com/nrdptel/fusionspace-eridanus/issues/1091)).
- A GNSS log and an altimeter log of one flight joined up
  ([#1086](https://github.com/nrdptel/fusionspace-eridanus/issues/1086)). Each of RocketPy's split
  files gives its own ghost, as a GPS track has no liftoff.
- Attitude: nothing here turns the marker.
- The Python package: `hpr analyze` only so far
  ([#1085](https://github.com/nrdptel/fusionspace-eridanus/issues/1085)).
