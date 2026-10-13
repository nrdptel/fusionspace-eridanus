# ADR-254: M7.1c in two: GPS tracks first (Featherweight's GPS CSV, KML and NMEA), read with an empty primary track and each point on the clock the file gives it (2026-10-12)

- **Status:** accepted; narrows M7.1c to a first increment, M7.1c1, and moves the rest, with M7.1c's old entry and *done when* (the parent's), to M7.1c2; follows [ADR-253](0253-rrc3-eggtimer-and-blue-raven-csv-first.md)
- **Summary:** M7.1c named 29 families of sample logs, too many for one increment. M7.1c1 reads the three that record a GPS receiver's track: Featherweight's GPS tracker and ground-station CSV, KML, and NMEA 0183, 95 distinct sample files. None has a barometer, so each reads with an empty primary track and every column a channel. A point's time is the one the file gives it, from a column, a sentence, a `when` or the comment a writer puts beside each point; a KML path that states no time keeps its points with their times as gaps. A KML altitude is in meters, as the KML standard sets, unless the file's writer is known to write another unit.

**Context.** After M7.1b, `cargo xtask logs` counted 29 families whose files no reader read,
from 1 file to 140. Three of them hold a GPS receiver's positions rather than an altimeter's: `fwgps-csv` (40
distinct files, with Debrief's ground-station fixture), `kml` (43) and `nmea` (12). They share a
record shape (latitude, longitude, an altitude above sea level, a receiver's own speed, heading
and satellite counts) and none of the questions the barometric readers answered: there is no
pressure to decide an altitude's unit from, and no primary track. They differ from each other
in where a point's time is:

- Featherweight's tracker log writes `UTCTIME` (such as `Jul 14 2031 17:04:31.250 UTC`) and `UNIXTIME`.
  In all 28 tracker files the two differ by a whole number of hours, 4 to 8, the launch site's
  offset from UTC, with `UTCTIME` the one that reads as daylight at the site; in the older
  layout, rows go back in time and repeat. The ground station's log writes `DATE` and `TIME`
  with no zone, which read as the site's local time, and may hold several trackers' rows.
- NMEA 0183 states the time of day in UTC on each `GGA` and `RMC` sentence and the date on
  `RMC`. The 12 samples carry a placeholder date until the receiver has a fix, and 9 of them
  step back about 2 s once, 7 a few minutes after the first fix and 2 before it.
- KML (OGC 07-147r2) holds a path, with times only in Google's `gx:Track`. Several writers put
  each point's time, height or satellite count in an XML comment beside it instead (AltOS, Big
  Red Bee's BeeLine and one more); the rest (fw2kml's paths, Missile Works' RTx, GPS Visualizer)
  give no time at all. One sample is four identical documents one after another; one isn't
  valid UTF-8; two end in a 128 KiB hex dump inside a comment. One, a map of a flight made in a
  map program, its predicted tracks with the pad and landing spot pinned, isn't a logger's file.

**Decision.**

1. **Two increments.** M7.1c1 reads Featherweight's GPS CSV (the tracker's and the ground
   station's), KML and NMEA 0183. Its *done when*: every `fwgps-csv`, `kml` and `nmea` sample in
   refs imports, as M7.1a's and M7.1b's families do (`cargo xtask logs --check` fails on one not
   read), and a unit or clock the file doesn't state is decided from the file or a source and
   said in a note the survey counts. M7.1c2 reads the rest, with M7.1c's old entry and *done
   when*, the parent's. The order planned for it: the binary dumps (AltOS `.eeprom`, the RRC3's
   `.rff`, the Raven's `.fipa`, PerfectFlite's `.pfa`, Entacore's, Missile Works' RTx), the
   companion summary and event files, spreadsheets, then teams' own CSV and text by column
   mapping; it splits again when it is next.
2. **No barometer, an empty primary track.** As for the Blue Raven's high-rate file
   ([ADR-253](0253-rrc3-eggtimer-and-blue-raven-csv-first.md) §6), a GPS log reads with an empty
   primary track and every value a channel, with a note. The readings withhold on such a log.
3. **A point's time is the file's.** The log's zero is the first point the file times. The
   tracker's clock is `UTCTIME`, and `UNIXTIME` stays a channel as written, with a note of its
   offset; the ground station's is `DATE` and `TIME`, in a zone the file doesn't state; NMEA's is
   the sentence's time of day, on the date of the last `RMC` with a valid fix. KML's is the
   `when` of a `gx:Track`, else the time a writer's comment beside the point gives. Channels keep
   every point in the file's order, so a time may go back; a note says how often and how far.
4. **A path with no time keeps its points.** A channel's time may now be a gap, `NaN`, written
   `null` in JSON as a value's gap is, for a point the file gives no time. A KML path that states
   none reads as channels of positions whose times are all gaps, with a note: a path is still
   where the rocket went, and the parent's *done when* asks that every sample import.
5. **Units from the standard, the writer or the data.** KML's altitude is in meters (OGC
   07-147r2, `kml:coordinates`). NMEA's altitude is in the unit its sentence names (always `M` in
   the samples), its speed in knots and its course in degrees true, as the sentence layouts say.
   Featherweight's files name no unit: heights read in feet and speeds in feet per second, as
   the tracker's manual gives them, checked against the file. The horizontal speed against the
   distance between fixes is the check that can tell feet from meters; the vertical speed against
   the change of altitude shows only that the two share a unit. Where the first disagrees, every
   length and speed is kept as written; each decision is a counted note. A writer known to break the standard's unit has its
   column kept as written, with a note.
6. **Not every KML is a flight.** The map moves to a family of
   files that aren't logs, by id, with the reason, as M7.1a's reclassified files do.
7. **Hostile input stays bounded.** XML is parsed by `roxmltree`, already a dependency, which
   refuses a document type declaration and so any entity expansion; per-satellite and
   per-tracker channels and notes are capped, the rest counted.

**Consequences.** `LogFormat` grows Featherweight GPS CSV, KML and NMEA 0183; `Channel::time_s`
may hold gaps, so a consumer of channels skips a `NaN` time as it skips a `NaN` value. The
readings, set on barometric tracks, withhold on GPS logs until M7.2 reads a GPS altitude.
M7.1c2 keeps the binary formats, spreadsheets, the companion files and the generic CSV reader,
and the list of formats needing samples.
