# ADR-057: A `.ork` design's stored simulations, read back as written, with their units measured (2026-09-21)

- **Status:** accepted
- **Summary:** A `.ork` design's stored simulations, read back as written, with their units measured

**Context.** M3.1c3 carries "a design's stored results are read back" from M3.1c's *done when*,
with Loft lesson L64: Loft read the wind's direction from `launchroddirection` and dropped it from
the stored conditions. OpenRocket's file-format page shows `<conditions>` and `<flightdata>` with
no units, and its own example writes the rod's direction as `90.0` and the wind's as
`1.5707963267948966`.

**Decision.**

1. **Read, not re-flown.** `hpr_io::ork::StoredSimulation` holds each run's name, status,
   simulator, conditions and results as the file states them; `Design::simulations` lists them,
   even for a document with no design in it. They are OpenRocket's answers, for M2.2 to compare
   against, and they do not affect which configurations hpr flies.
2. **The units are measured, and committed.** `validation/oracles/openrocket/conditions.py` sets
   the conditions through OpenRocket 24.12's public setters, saves, loads and flies them;
   `validation/fixtures/ork/openrocket-conditions.json` holds what it wrote and held, and L64's test
   holds the reader to it. The rod's angle and direction are degrees, and this reader gives them in
   radians; the rod's direction is a compass bearing (a rod tilted toward 90 lands the rocket
   east, while the example's own wind stays put); the wind's direction is radians, the bearing it
   blows from (in a wind from the east, a rocket off a vertical rod lands west); with
   `launchintowind`, OpenRocket writes the wind's bearing, in degrees, as the rod's. All of it is
   OpenRocket 24.12's; a much older file may have meant the rod's direction otherwise.
3. **The results are kept as written.** A time series keeps its column names, its rows and its
   events, and the results keep OpenRocket's stored `<warning>`s. In the eight columns the probe
   compared, rows are SI with angles in radians and latitude in degrees, rounded to three decimal
   places or, for a large value, four significant figures; the summary and the other columns are
   taken to follow. `NaN` becomes `None`, so the design survives JSON and back. A row with the
   wrong number of values, an event with no time or type, and an atmosphere with no model or one
   OpenRocket 24.12 does not write are each warned about.
4. **The wind's speed and direction come from the legacy tags first**, `<windaverage>` and
   `<winddirection>`, and from the average `<wind>` element where those are missing: the file
   specification says OpenRocket still writes the legacy tags for older versions. Where both are
   written and disagree, a warning says so. `<windmodeltype>` says which wind the run flew, since
   OpenRocket writes both.

**Consequences.** On 2026-09-21 the library's 178 stored simulations are read, in 64 documents;
164 carry a summary and 144 a time series (101,955 rows). M3.1c's "a design's stored results are
read back" is met here.
