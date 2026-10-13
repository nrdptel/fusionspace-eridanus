# ADR-252: M7.1b in two: the RRC3's, Eggtimer's and Blue Raven's CSV exports first, each unit a header leaves out decided from the file or stated as assumed (2026-10-12)

- **Status:** accepted; narrows M7.1b to three families and moves the rest, with M7.1b's old *done when* (the parent's), to a new M7.1c; follows [ADR-251](0251-flight-log-importers-in-two-increments.md)
- **Summary:** M7.1b listed every layout M7.1a left, about 30 families, too many for one increment. It now reads the three CSV exports with the most sample files among them, the RRC3's, Eggtimer's and the Blue Raven's (173 distinct files), into the record M7.1a built; M7.1c reads the rest. A unit a header doesn't name is decided from the file, such as an RRC3's altitude from its pressure, or taken from the maker's manual, and either way stated in a note the survey counts; a Blue Raven's high-rate file, which holds no altitude, reads with an empty primary track.

**Context.** After M7.1a, `cargo xtask logs` counted about 30 families of sample logs no reader
knew, from 1 file to 140. M7.1b's entry named them all, with the parent's *done when*: every sample
file in refs imports, snapshot-tested. Three CSV families hold 173 of the distinct files and have
Debrief readers to follow (MIT, the maintainer's own): Missile Works' mDACS export of an RRC3's
flight (50), Eggtimer's download (43) and the Featherweight Blue Raven app's export (80). The rest
are binary dumps (AltOS `.eeprom`, the RRC3's `.rff`), spreadsheets, GPS trackers, KML and NMEA,
summaries, and teams' own CSVs, each a reader of its own or the generic CSV reader with column
mapping.

None of the three states every unit. An RRC3 export's header names none unless mDACS writes them
in brackets, and mDACS writes the altitude, speed and temperature each in the unit its own
preferences set (*mDACS User Manual*, rev. 1.71, p5). An Eggtimer's header names none, and its
time is milliseconds on the Classic and TRS and seconds on the Quantum and Proton under the same
`T`. The Blue Raven app's low-rate file names most units, but its velocities, inertial positions
and high-rate sensors are bare. Debrief guessed some of these: it reads an RRC3's speed in its
altitude's unit, which reads the 8 sample exports in miles per hour 32% low, and its note on an
Eggtimer set to meters has the direction backwards (`docs/research/debrief-log-formats.md`).

**Decision.**

1. **Two increments.** M7.1b reads the RRC3's, Eggtimer's and the Blue Raven's CSV exports. Its
   *done when*: every `rrc3-csv`, `eggtimer-csv` and `blueraven-csv` sample in refs imports, as
   M7.1a's families do (`cargo xtask logs --check` fails on one not read); a unit the header
   doesn't name is decided from the file's own evidence and the evidence stated, or the
   assumption stated in a note. M7.1c reads every other layout, with M7.1b's old entry and
   *done when*, the parent's.
2. **A unit is decided from the file where the file can.** An RRC3's altitude unit is the one,
   feet or meters, whose climb is within 25% of the climb its pressure implies in the standard
   atmosphere (USSA 1976): on the 50 samples, feet misses by at most 0.05% and meters by at least
   227%. Its speed unit is the one of feet per second, miles per hour, meters per second and
   kilometers per hour that two measures put within 3%: the speed's running integral against the
   altitude a fitted delay earlier, by a least-squares slope and by the ratio of their areas, so
   that a speed lagging the altitude, as a filtered one does, isn't read as a smaller unit (ft/s
   and km/h are 9.7% apart). With no such unit, or a record that stops before the rocket is
   halfway down, the column is kept as written. On the 48 samples measured, the unit each is in
   came within 0.26% and the runner-up at least 8.8% off; 2 stop early. An Eggtimer's clock is in
   milliseconds when the median step is above 5, as the manuals' 1 to 33 samples a second put a
   step in seconds below 1 and in milliseconds above 30; an RRC3's is in seconds where its step is
   the RRC3's 20 samples a second.
3. **Otherwise from the maker, or assumed, and said.** An Eggtimer's heights are feet: the
   Classic and TRS manuals state it, the Quantum and Proton manuals give every height in feet,
   and none offers meters; its speeds are feet per second where they match the altitude's change
   on at least 90% of rows, else kept as written.
   The Blue Raven's bare columns take the units the manual gives for its USB download of the same
   figures (feet per second, feet, degrees per second, g), which the app's summary files and the
   pad's 1 g bear out. An RRC3's temperature without brackets is assumed in the unit the RRC3's
   settings pair with the altitude's (°F with feet, °C with meters). The Blue Raven's `Current`,
   whose unit no source states, is kept as written.
4. **Every decision is a note the survey counts.** A note on a decided unit reads
   `` `<column>` read in <unit>, <from what>: <evidence> ``, or
   `` `<column>` kept as written, <why>: … ``, the part before the colon the same for every file
   decided alike. `cargo xtask logs` counts those parts by family into its report
   (`unit_decisions`), so a page quoting a count quotes the tool.
5. **The record grows two quantities**: a rate of turn, in degrees per second, for the Blue
   Raven's gyroscopes, and a time of day on the logger's own clock, in seconds after its
   midnight, for its `Time` column, which states no time zone and in 9 sample files runs
   backwards or stalls, so it is never a clock.
6. **A file with no altitude still reads.** The Blue Raven's high-rate file holds only its
   inertial sensors. It reads with an empty primary track and every column a channel, with a note
   pointing to the low-rate file, rather than being refused: the parent's *done when* asks that
   every sample import, and the readings withhold on any logger but a PerfectFlite before they
   read the track.
7. **The survey is optimized.** The readers, SHA-256 and serde_json's float writer build at
   opt-level 3 in the dev profile: `cargo xtask logs` took 167 s unoptimized with the Blue Raven's
   20 MB files and takes 25 s, its report the same byte for byte either way.

**Consequences.** M7.1b closes with 173 more files read, 494 in all, and the report's new table
of unit decisions. `LogFormat` grows three formats, `Quantity` two kinds; JSON consumers of a
`FlightLog` see them. The importer reads UTF-16 by its byte-order mark, as Debrief's notes say some
mDACS exports are, though none of the samples is. M7.1c keeps the binary formats, spreadsheets,
GPS logs, the companion summary files and the generic CSV reader, and the list of formats needing
samples.
