# Ghost files

A ghost file is one flight's path, from its logs, on one clock in a frame at the pad. `hpr analyze
--ghost` writes it.

It holds the path for a viewer to replay: as JSON for your own program, as CZML for
[Cesium](https://cesium.com/platform/cesiumjs/) (a free 3D globe), or as glTF for any 3D tool. This page covers what each holds, where each number comes from
and what the file assumes when a log can't say. It is for anyone who wants to see a flight in 3D or
load its path into a program. [The app's replay](../decisions-and-roadmap.md#m9-2) will read it too.
It is called a ghost for the simulated flight that will fly beside the real one. Today it holds logs
only ([#1091](https://github.com/nrdptel/fusionspace-eridanus/issues/1091)).

> **How far to trust it.** A ghost file lays out what a log holds and adds no measurement, so it is
> as good as the log. RocketPy's repository holds 22 real flight logs with an altitude or a GNSS
> track. Each gives a ghost file, and outside readers read each back
> ([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/ghost-files.md),
> [evidence row G37](../evidence.md#g37)). Nobody has yet looked at one in Cesium's globe or a
> modeling tool. For an apogee or a speed, go to [the readings](../reading-a-flight-log.md).

## Writing one

```bash
hpr analyze flight.csv \
  --ghost flight.czml

hpr analyze telemetrum.csv \
  telemega.csv \
  --ghost flight.json \
  --ghost flight.gltf

hpr analyze flight.csv \
  --ghost flight.czml \
  --pad 39.39,-8.29,120
```

The file's extension picks its format: `.json`, `.czml` or `.gltf`. Give `--ghost` again for another
format. `--pad` states the
pad's latitude and longitude in degrees, then its height in meters above the WGS 84 ellipsoid if you
know it. It overrides what the log's fixes say. Each option of [`hpr
analyze`](../cli.md#hpr-analyze) that reads a log, such as `--mapping`, works with it.

## What a track holds

Each log gives one track, a list of moments, each with these numbers.

| Number | Where it comes from |
|---|---|
| Time, s | Counted from the log's [liftoff](../reading-a-flight-log.md), so that two logs of one flight share a clock. A log with no liftoff stands alone on its own clock, marked `"synced": false`. It can't join other logs in one file. |
| Up, m | The log's own altitude above the pad. A column above sea level, or with no stated zero, is taken less its own reading on the pad: the median of its last 20 samples before liftoff, or of its first 20 with no liftoff. A log with no altitude takes its GNSS altitude less the receiver's on the pad. With none of these, every height is `null`. |
| East and north, m | The log's GNSS fixes (a fix is one position from the receiver) in the frame at the pad, in a straight line between fixes. Both are `null` before the first fix and after the last. A log with no fixes has none. |

Why the log's altitude and not [the smoother's](../physics/smoother.md): on a barometer alone the
smoother can run far from the log through a sudden swing. On RocketPy's Lince flight, as its
recovery fires, it runs to 10,205 m (33,481 ft), 6,618 m (21,713 ft) past the flight card
([#1073](https://github.com/nrdptel/fusionspace-eridanus/issues/1073)). `hpr analyze`'s health card
says to go by the reading there, and a file for a viewer has no card. So a barometer's smaller
spikes stay in the height, and a receiver's lag on the climb stays in east and north.

### The pad

The pad is the frame's origin: a flat east, north and up grid there, on the
[WGS 84](../glossary.md#wgs-84) ellipsoid ([the frame's equations](../physics/geodesy.md)). It comes from the first of these a flight has:

- `--pad`;
- the median of the last 20 fixes before liftoff;
- the first fix, and how long after liftoff that was, where the log can tell.

A pad from the first fix may be off by what the rocket has already flown. With no fix at all, nothing
says where the flight was, and the file says that (below).

The pad's height comes from the same place, and never from a fix in flight. So fixes before liftoff
with no height, or a first fix after liftoff, leave it unknown. A receiver's height stands in for one
above the ellipsoid. Most receivers state sea level, which is off from the ellipsoid by the local
geoid, within about 110 m (361 ft). Up isn't affected: it is measured from the receiver's own reading.

### What is left out

Each track counts what it leaves out, in `fixes`, `heights_too_fast`, `heights_below_pad` and
`moments_left_out`. CZML and glTF say how many rows went. A fix goes when any of these holds:

- it is empty, out of range, or within 0.01° of 0°N 0°E, which is a receiver's way of saying no fix;
- its clock didn't go forward;
- it lies more than 200 km (124 mi) from the track's median, as a longitude with its sign flipped does;
- reaching it from the last fix kept would take over 1,500 m/s (4,921 ft/s), plus 200 m (656 ft) for a receiver's scatter.

A height goes on the same speed, plus 50 m (164 ft). The first kept is the first that agrees with the
middle of the log's first 9 rows, so one bad first row can't shut out the flight. That catches a logger's garbage rows, such as the last
two of Juno III's altimeter file, at 10,700 m (35,105 ft) and −2,490 m (−8,169 ft). It also took 57 of the 44,454 rows of
Lince's height, in the swing as its recovery fires.

Three more go, each counted:

- A height over 500 m (1,640 ft) under the pad. A receiver with no lock can write a height near sea
  level: one of Juno III's files does that for 3,400 rows, 1.4 km (0.87 mi) under its pad, close enough
  in time for the speed test to pass.
- A moment with no height, or a height of exactly 0 from a receiver, in a track that has heights. Each
  track has a height at every moment or at none, and a viewer's line crosses the gap.
- A clock past 317 years, or over 116 days from the log's opening.

## The three formats

JSON follows [`schema/cli/ghost.schema.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/ghost.schema.json).
Every number is the track's own, with `null` where a log can't say. Use this one for a program.

CZML is [Cesium's scene format](https://github.com/AnalyticalGraphicsInc/czml-writer/wiki/CZML-Guide).
Each track is an entity with an Earth-fixed position, a point and a path.

glTF is [Khronos's 3D scene format](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html), one
file with its numbers inside. Each track is a line along its whole path, with a marker 10 m (33 ft) across
moving along it. In meters, +x is east, +y is up and −z is north.

These last two have no `null`, and a drawing needs a number at every moment. So they draw something
where a log can't say, and each track's own text says what it drew:

| Where the log can't say | CZML and glTF draw |
|---|---|
| No fix yet, or no longer | The nearest fix. A log with no fixes is a straight vertical line over the pad. |
| No height in the whole track | 0 |
| No place for the pad | CZML: 0°N 0°E. glTF has no globe, so it needs none. |
| No height for the pad | CZML: the path starts on the ellipsoid, which can be far under or over the ground. glTF needs none. |

Each also has a limit of its own:

- CZML needs a date, and most logs hold none, so liftoff, or a log's first moment if it has none, is
  always noon on 1 January 2000. The time on its clock is seconds from then, not the day of the flight.
- glTF's times can't be negative, so its clock starts at the earliest moment. The file's
  `extras.first_moment_s` says which that was.
- glTF stores 32-bit numbers, which stand about 0.06 ms apart at 1,000 s and 2 mm (0.079 in) apart at 30 km (19 mi). Two
  moments it can't tell apart become one.
- East, north and up are on the flat plane at the pad, which rises off the curved ground as you go.
  So a height is drawn about 31 m (102 ft) too high at 20 km (12 mi) downrange.

## A worked example

[RocketPy's repository](https://github.com/RocketPy-Team/RocketPy/tree/master/data/rockets/prometheus)
holds two boards' logs of its Prometheus flight, shortened here. Reading them both:

```bash
hpr analyze \
  5115-TeleMetrum.csv \
  6583-TeleMega.csv \
  --ghost prometheus.json
```

The TeleMetrum's liftoff, at 0.23 s on its clock, is 0 in the file. It holds 5,710 moments. Its height
peaks at 3,904 m (12,808 ft), the apogee RocketPy's notebook states from the same column. It kept 5,712 of its
7,081 fixes, which is normal: the other 1,369 rows repeat the time of the row before them. The fixes put the rocket at
most 1,170 m (3,839 ft) from the pad, and the fixes before liftoff put the pad at 1,390 m (4,560 ft) above sea level. The
TeleMega's track sits on the same clock and in the same frame, and peaks at 3,898 m (12,789 ft).

## How it's checked

`cargo xtask ghost` writes each file of each of the 22 logs, then reads them back with open-source
readers this project didn't write. CI runs the same on two invented flights, as `refs/` isn't there.

- Ajv, a JSON Schema checker, holds the JSON to the schema.
- Cesium's own `CzmlDataSource` loads the CZML. Cesium's own transform takes each position to the
  pad's frame, where it comes within 1 cm (0.39 in) of the JSON's at every moment, and the file's count of
  them matches.
- Khronos's glTF-Validator finds no error in the glTF. glTF-Transform's animation comes within 1 cm (0.39 in)
  and 1 ms of the JSON's at every moment, none missing, and its path's line holds the same places.

So the formats agree with each other. What ties them to the log is pinned apart from that:

- A foot read as a meter would miss by a factor of 3.28, so each height is held to an apogee from
  RocketPy's notebooks that wasn't read off that column. Of the 16 logs with a barometer's or a
  filtered height, 13 peak within 2% of a flight card or another board's record, Valkyrie's log at
  1.9% under being the widest. The other 3 have no such figure, so they are held to RocketPy's
  simulated apogee, the widest 3.3% under. The two with only a receiver's altitude, on Juno III, peak
  4.9% and 5.9% over its altimeter's card.
- East and north match the ellipsoid's radii of curvature, by a route that doesn't use the rotation
  under test, to 5 mm (0.20 in).
- The bytes of a small CZML and glTF file are worked out by hand from each format's spec.

## What it leaves out

- A simulated flight beside the real one ([#1091](https://github.com/nrdptel/fusionspace-eridanus/issues/1091)).
- A GNSS log and an altimeter log of one flight joined up
  ([#1086](https://github.com/nrdptel/fusionspace-eridanus/issues/1086)). Each of RocketPy's split
  files gives its own ghost, as a GPS track has no liftoff.
- Two logs of the same file name, whose tracks carry that name,
  and a pad by the 180° meridian ([#1093](https://github.com/nrdptel/fusionspace-eridanus/issues/1093)).
- Attitude: nothing here turns the marker.
- The Python package: `hpr analyze` only so far
  ([#1085](https://github.com/nrdptel/fusionspace-eridanus/issues/1085)).
