# ADR-277: A track's own speed, held by its vertical speed before apogee (2026-10-16)

- **Status:** accepted; amends [ADR-274](0274-the-rest-of-the-importers-planned-teensy-logger-first.md)'s amendment for M7.1c22, item 5, which had recorded a track's own speed as not met
- **Summary:** M7.1c22's *done when* names a track's own speed among the physical checks that hold a mapped table's units. A track's ground speed holds none: the 4 tracks' medians spread 21.34 times. Its vertical speed does. In the 5 s before apogee a coasting rocket slows at g, plus a little drag, so a parabola fitted to a GPS track's fixes there must slow at 0.6 to 5/3 g. An altitude in feet stated as meters reads 3.28 times too high, and a clock 10 times off moves it 100 times. The club's GPS track slows at 0.9925 g, so that part of the *done when* is met.

**Context.** ADR-274's amendment for M7.1c22 held each mapped column's unit to a physical check.
For a track's own speed, it measured the ground speed from fix to fix. That holds no unit:

- the 4 tracks' medians spread 21.34 times, and their fastest steps 16.10 times, both more than
  the 10 times a clock misread by a factor of 10 moves them;
- a track drifts with the wind, which on a calm day gives it no floor.

So the amendment recorded that part as not met. But the *done when* names a track's own speed,
not its ground speed, and the speed's vertical part has a physical value near apogee that
doesn't depend on the flight.

**Decision.** A new survey check, *track speed* (`xtask/src/logs/held.rs`), holds a track's GPS
altitude and its clock.

- *The physics.* Coasting, a rocket's vertical deceleration is g plus drag's vertical share,
  ½ρC_DA/m · v · v_z, which falls to 0 at apogee as v_z does.
  - A 54 mm rocket of 1.5 kg, with C_D 0.5 at ρ = 1.1 kg/m³, coasting straight up, climbs at
    50.8 m/s 5 s before apogee. Its drag adds 0.11 g there.
  - Carrying 100 m/s across at apogee, it adds 0.33 g.
  - A parabola fitted to its fixes once a second reads 1.031 g straight up and 1.135 g with the
    speed across (the test `a_coast_with_drag_reads_inside_the_band`).
- *The check.* It takes the track's highest fix as apogee and fits a least-squares parabola to
  the fixes in the 5 s before it, the highest included. It needs at least 4 fixes, and counts
  once an altitude repeated over consecutive rows. Its deceleration, −2c, over g must lie in the
  altitude checks' band, 0.6 to 5/3.
- *What it refuses.* An altitude in feet stated as meters reads 3.28 times too high, and one in
  meters stated as feet 0.3048 times. A clock 10 times off moves the ratio 100 times, so the
  check holds the clock as well as the altitude.
- *Every track that can take it does.* The survey fails a track held by a latitude-and-longitude
  check whose GPS altitude has the 4 fixes but takes no track speed check. The report prints the
  fixes of each GPS altitude that takes none.

The ground speed stays measured in the report, with the finding that it holds no unit.

**Consequences.**

- The 20 held tables' checks number 62, all passing.
- The club's GPS track, in 2 files with the same fixes, slows at 0.9925 g.
- The radio telemetry's receiver lost its fix on the way up, leaving 1 fix in the window, the
  highest itself. Its GPS altitude stays held by another logger's.
- The other 2 tracks have no altitude.
- M7.1c22's *done when* is met with its text unchanged. ADR-274's item 5, the flight-log format
  page and the roadmap's done entry say so.
- The check holds one distinct track. It holds a unit within a wide band; it doesn't measure
  drag, which a receiver's noise can hide.
