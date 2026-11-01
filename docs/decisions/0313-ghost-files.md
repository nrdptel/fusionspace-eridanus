# ADR-313: Ghost files: the log's own height, three formats from one track, outside readers in Node (2026-11-01)

- **Status:** accepted; carries out [ADR-280][adr-280]'s M7.2g
- **Summary:** `hpr analyze --ghost` writes a flight's logs as time-synced trajectories in a frame at
  the pad, as JSON, CZML and glTF. The height is the log's own, not the smoother's. Fixes and heights
  no flight could reach are left out and counted. The formats that can't hold a gap draw one and say
  so. CesiumJS, Khronos's glTF-Validator, glTF-Transform and Ajv read each file back, on RocketPy's
  22 real logs in the gate's `refs` step, and on an invented flight in CI.

[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[report]: ../../validation/reports/ghost-files.md
[page]: ../format/ghost-files.md
[i1073]: https://github.com/nrdptel/fusionspace-eridanus/issues/1073

**Context.** ADR-280 asks that a flight's logs give a ghost file: time-synced trajectories in a frame
at the pad with the pad's geodetic position, as JSON with a schema, CZML and glTF, each read back by
an outside reader; and that every real flight log in RocketPy's repository with an altitude or a GNSS
track gives one, by a test where `refs/` holds them and an invented log in CI. Nothing wrote a
trajectory. Five of those logs read by their format's own reader. The other 17 each need a mapping:
most state no units, and none has a format's reader of its own.

**Decision.**

1. **A flag, not a command.** `hpr analyze --ghost FILE`, its extension picking the format, reuses
   how `hpr analyze` reads a log and its pairs, and adds no surface. `--pad` states the pad.
2. **The height is the log's, above the pad.** A column above sea level or of no stated zero is taken
   less its own reading on the pad, and a receiver's heights less the receiver's own, so a stated pad
   height of another datum can't move them. The first draft used the smoother's, and on RocketPy's Lince flight, as its
   recovery fires, it peaked at 10,205 m against the log's 3,669 m and the flight card's 3,587 m; on Genesis, 3,360 m
   against 2,917 m. The health card already says "go by the reading" there
   ([#1073][i1073]); a file for a viewer has no card. A reading's bound belongs to `hpr analyze`.
3. **Screens are counted, not silent.** Each fix left out has one reason, in `fixes`: no place, the
   clock not going forward, past 200 km of the track's median, or a jump over 1,500 m/s plus 200 m. A
   height goes on the same speed plus 50 m (`heights_too_fast`). The speed is above the envelope's
   top; the reach and the scatters are judgment, sized on these 22 logs, where they take Juno III's
   flipped longitudes and garbage last rows and leave every other track's height peak in place.
   Lince's height loses 57 of 44,454 rows in the swing as its recovery fires, its peak going from
   3,669 m to 3,629 m. The card says 3,587 m, so what is left of the swing is still over it: a
   ghost's top isn't an apogee. A review found what those screens let through on Juno III's telemetry,
   so three more stand. The first kept row must agree with the middle of the first 9, or one bad first
   row keeps itself and shuts out the flight. A height over 500 m under the pad goes
   (`heights_below_pad`), since a receiver with no lock wrote heights 1.4 km under its pad for 3,400
   rows, close enough in time for the speed to pass; the margin is judgment, and those rows leave
   nothing below 28 m under the pad on any of the 22. A moment with no height goes, so a track has a
   height at every moment or at none; that file held 2,965 of them, which were drawn at the pad.
4. **One track, drawn two ways.** JSON holds `null` where a log can't say. CZML and glTF can't, so
   they hold the nearest fix, draw a track of no height at 0, and an unknown pad at 0°N 0°E or on the
   ellipsoid, and each track's text says so. CZML's date is a fixed noon on 1 January 2000, as most
   logs hold no date: liftoff, or the first moment with none, since a date far from that epoch
   crosses leap seconds that seconds from it don't. A clock past 317 years, or 116 days from the
   log's opening, is garbage, left out and counted, so no date overflows.
5. **The frame is the one the simulator's geodesy already has** (`hpr_core::frames::LaunchFrame`).
6. **A log with no liftoff can't join others in one file.** It would need another way onto their
   clock (#1086). Alone, it is written on its own clock, `synced: false`.
7. **The readers are in Node, once, on Linux.** CesiumJS's data source is the reader a Cesium user
   has, so no CZML reader is more outside than it. They are pinned by a lockfile under `validation/oracles/ghost`, and nothing there ships. The gate's `refs`
   step reads RocketPy's logs back, as CI has no `refs/`; CI's `types` job reads the invented flight
   back. Cesium's scene code names the browser's `Image` on load, so the script stubs it.

**Consequences.** All 22 logs give all three files, all read back at every moment ([report][report]).
Each stated height's unit is checked against a figure not read off that column. Of the 16 with a
barometer's or a filtered height, 13 peak within 2% of a recorded apogee in RocketPy's notebooks, and 3
within 3.3% of its simulated one, having no other; the two with only a receiver's altitude are within 6%. Left
out, each with its issue, on [the page][page]: a simulated track (#1091), joining a GNSS log to its
altimeter log on the clock they share (#1086), and Python (#1085). Nobody has yet looked at one in a
viewer: every check is of the numbers. M9.2's replay will be that look.
