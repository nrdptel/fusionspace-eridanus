# ADR-276: Neer's 2026-10-16 plan: a flyer's own logger and GPX tracks in the analyzer (2026-10-16)

- **Status:** accepted; adds M7.1d (M7.1d1 and M7.1d2) to [M7.1](../ROADMAP.md), so both land before release 0.2 ([M10.2](../ROADMAP.md)); amends ADR-147 §6 (the roadmap's budget); amended 2026-10-17 for M7.1d1
- **Summary:** Release 0.2, the flight analyzer, must read a flyer's own altimeter or GPS tracker from the command line and Python, not only from Rust, and must read GPX tracks. M7.1d1 reads GPX 1.0 and 1.1; M7.1d2 lets `hpr analyze` and the Python package read a table by a column mapping and binary records by a record layout, as the library already does.

[adr-268]: 0268-tables-read-by-their-headers-before-the-rest.md
[adr-271]: 0271-binary-records-by-a-stated-layout-before-the-rest.md

**Context.** On 2026-10-16 Neer decided that the analyzer reads custom altimeter files and GPS
trackers, both commercial and home-built, and that whatever the plan didn't already cover be
added. A survey of the code and the roadmap that day found:

- **Custom altimeters: the Rust library only.** A table no maker documents is read by the units
  its header states, or by a column mapping the caller gives (`generic::read_with`,
  [ADR-268][adr-268]); a flight computer's binary records are read by a record layout the caller
  gives (`records::read`, `capture::read`, [ADR-271][adr-271]). `hpr analyze` takes one argument,
  the log, and documents it as a PerfectFlite `.pf2`; the Python package has no log reading at
  all. No milestone or issue planned either surface.
- **GPS trackers.** Featherweight's GPS CSV, Missile Works' RTx, KML and NMEA 0183 are read, and a
  custom table's latitude and longitude columns are read under their names. GPX, the exchange
  format of phone apps, handheld receivers and mapping tools, is not read and not planned. No
  sample log in the project's collection is a GPX file.
- **A mapped table has no primary track.** Nothing states which column is the barometric altitude
  above the pad, so M7.2's readings would have nothing to read from a flyer's own logger.

**Decision.**

1. **M7.1d1 reads GPX tracks.** GPX 1.0 and 1.1 (Topografix's published schemas): each track
   segment's points become a GPS track of latitude, longitude, elevation and time; routes and
   waypoints, which were not flown, are kept apart from it. GPX doesn't state the datum of `ele`,
   so the elevation is a GNSS altitude of unstated datum, with a note. With no real sample, the
   reader is checked on invented files against an outside reader, gpxpy (Apache-2.0), and the
   guide says GPX is read from invented samples only and asks for a real tracker's file.
2. **M7.1d2 brings the library's mappings and layouts to `hpr analyze` and Python.** A mapping
   and a layout are JSON files with a committed schema; `hpr analyze` takes either, and prints a
   table's header reading as a mapping to edit; the Python package reads a log by its own reader,
   a mapping or a layout into the same record. A mapping may mark the column that is the
   barometric altitude above the pad as the primary track, so M7.2's readings reach a flyer's own
   logger. The checks are the library's own: the same files read the same record on all three
   surfaces.
3. **Both go in M7.1, as M7.1d**, after M7.1c's last increment. M10.2's *done when* already
   waits on M7.1, so release 0.2 waits on them with no change to its wording, and nothing queued
   moves.
4. **The roadmap's budget rises** from 48,500 to 49,500 bytes, for the two increments (as
   ADR-275 did for M0.13).

**Consequences.** Release 0.2 moves later by about two increments, a day or two of the run. The
analyzer's two command-line and Python options become part of its public interface, so their
schemas follow the CLI's stability rules from the start. GPX's elevations stay of unstated datum
until a real tracker's file shows what one writes.

**Amendment for M7.1d1, how the GPX reader reads (2026-10-17).** Writing the reader from the two
schemas and GPX 1.0's manual settled five things the plan left open:

1. **The elevation is an altitude of unstated zero**, `Quantity::Altitude`, not a GNSS altitude,
   `Quantity::GnssAltitude`, as §1 had it: the record's GNSS altitude is one above mean sea level
   as the receiver states it, and a GPX file states neither its datum nor its sensor. Its unit,
   meters, GPX 1.1's schema and GPX 1.0's manual do state.
2. **Waypoints and routes are figures the file states**, `Stated::figures`, not channels: a
   waypoint's or route point's latitude, longitude, elevation and time, the time in seconds from
   the log's zero, so a reading that looks for positions among the channels finds the flown
   tracks only.
3. **A track's times go forward.** Within a track, from one segment to the next, a time earlier
   than the one before it refuses the file, as such a track isn't one recording; two tracks may
   run in any order. A time with no zone is read as UTC, as GPX 1.1's schema and GPX 1.0's
   manual say every time is.
4. **Damage refuses the file, by name and line.** Besides the plan's document type and entity, a
   point off the globe and a time that goes back: a value that isn't the schema's kind of number
   (a decimal with no exponent, a whole satellite count), an element read twice in one point, a
   time that isn't a date and time, a GPX 1.0 `course` outside 0° to 360°. A GPX file is written
   by a program, so these mean a damaged file, not a reading to guess at. An empty element, such
   as `<ele/>`, states nothing: it is a gap, counted in a note, as gpxpy reads it too. The
   longitude may be 180, which GPX 1.1 leaves out and GPX 1.0 allows: it is −180's meridian.
5. **More of a point is read**: the satellite count, the three dilutions of precision and GPX
   1.0's course and speed, each a channel where a point states it, each held to gpxpy's reading
   with the rest. gpxpy keeps a time to the microsecond and drops finer digits, so the samples
   write none finer.

**Consequences of the amendment.** Three invented files, GPX 1.0 and 1.1, read to 392 values
and times of their tracks' channels (a gap matching a gap) and 12 figures of their waypoints and
route points, each equal to gpxpy 1.6.2's to the last bit. No reading uses a GPX track until M7.2 reads other loggers.
