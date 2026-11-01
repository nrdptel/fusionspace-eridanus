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
trajectory. Five of those logs read by their format's own reader. The other 17 state no units, so each
needs a mapping.

**Decision.**

1. **A flag, not a command.** `hpr analyze --ghost FILE`, its extension picking the format, reuses
   how `hpr analyze` reads a log and its pairs, and adds no surface. `--pad` states the pad.
2. **The height is the log's.** The first draft used the smoother's, and on RocketPy's Lince flight it
   peaked at 10,205 m against the log's 3,669 m and the flight card's 3,587 m; on Genesis, 3,360 m
   against 2,917 m. The health card already says "go by the reading" there
   ([#1073][i1073]); a file for a viewer has no card. A reading's bound belongs to `hpr analyze`.
3. **Screens are counted, not silent.** Each fix left out has one reason, in `fixes`: no place, the
   clock not going forward, past 200 km of the track's median, or a jump over 1,500 m/s plus 200 m. A
   height goes on the same speed plus 50 m (`heights_too_fast`). The speed is above the envelope's
   top; the reach and the scatters are judgment, sized on these 22 logs, where they take Juno III's
   flipped longitudes and garbage last rows and leave every other track's height peak in place.
   Lince's barometer loses 57 of 44,454 rows at its spike, 3,669 m to 3,629 m.
4. **One track, drawn two ways.** JSON holds `null` where a log can't say. CZML and glTF can't, so
   they hold the nearest fix, draw an unknown height at 0, and an unknown pad at 0°N 0°E, and each
   track's text says so. CZML's date is a fixed noon on 1 January 2000, as most logs hold no date.
5. **The frame is the one the simulator's geodesy already has** (`hpr_core::frames::LaunchFrame`).
6. **A log with no liftoff can't join others in one file.** It would need a clock nothing gives
   (#1086). Alone, it is written on its own clock, `synced: false`.
7. **The readers are in Node, once, on Linux.** CesiumJS's data source is the reader a Cesium user
   has, so no CZML reader is more outside than it. They are pinned by a lockfile under `validation/oracles/ghost`, and nothing there ships. The gate's `refs`
   step reads RocketPy's logs back, as CI has no `refs/`; CI's `types` job reads the invented flight
   back. Cesium's scene code names the browser's `Image` on load, so the script stubs it.

**Consequences.** All 22 logs give all three files, all read back ([report][report]); each log's
height peaks within 2% of what RocketPy's notebooks state, which checks the 17 mappings' units. Left
out, each with its issue, on [the page][page]: a simulated track (#1091), joining a GNSS log to its
altimeter log on the clock they share (#1086), and Python (#1085). Nobody has yet looked at one in a
viewer: every check is of the numbers. M9.2's replay will be that look.
