# ADR-267: M7.1c12 in two: Missile Works' RTx first; the rest to M7.1c13 (2026-10-14)

- **Status:** accepted; narrows M7.1c12 to a first increment and moves the rest, with M7.1c12's old entry and *done when* (the parent's), to M7.1c13; follows [ADR-266](0266-arts2-raw-data-before-the-rest.md)
- **Summary:** After M7.1c11, 131 sample logs were left unread. Fourteen are a Missile Works RTx GPS tracker's: five `.rtx` files, as mDACS saves a download, and nine CSV and text exports, eight of them beside an `.rtx` with the KML export of the same fixes. M7.1c12 now reads them, the `.rtx`'s words decoded from the files and held to every export beside them; the rest goes to M7.1c13.

**Context.** `cargo xtask logs` on main after M7.1c11 (#545) leaves 131 files unread: generic CSV
and text (91), spreadsheets (24), and smaller families, among them the RTx's `.rtx` (5), which
[ADR-259](0259-raw-downloads-against-their-makers-exports-first.md) left among the raw downloads,
and nine files filed as generic CSV or text that are its exports. The *RTx/GPS User Manual*
(firmware 1.31, Missile Works, 2016-11-26) says the tracker, built around a u-blox 7 receiver,
keeps its fixes at 1 Hz, "~68 minutes per flight" (p2), sends them "at 1-Hz intervals" (p14), and
states its GPS altitude "as Mean Sea Level" (p13); the *mDACS User Manual* (revision 1.71, May
2024) says mDACS saves a download "with the .rtx file extension" and exports it to KML, CSV or
text (p16). Neither describes the files. What the files show, with no other program's source
read:

- **The `.rtx`** is an MS-NRBF stream (*[MS-NRBF]*, revision 13.0, 2024-11-19) of one
  `List<Int32>`: the same 186 bytes in all five files up to the list's `_size`, a list of 65,536
  slots, and nothing after the end record. `_size` counts the slots in use, 16 a record; the rest
  are 0. 65,536 slots are 4,096 records, 68.3 minutes at 1 Hz, the manual's flight.
- **Each slot is a 16-bit word times 387,420,489 (3¹⁸), modulo 10⁹.** The slot holding the time
  steps by exactly 387,420,489 modulo 10⁹ from one record to the next a second later; all 264,336
  slots in use lie from 0 to 10⁹ − 1 and read back, times 513,180,409 (the inverse), as values
  below 65,536, which a slot drawn at random from that range would do about once in 15,000; and the values equal the
  exports' figures on every row.
- **The words** are the time as `HHMMSS` in two halves; the latitude's `DDMM`, then its minutes'
  five decimals after a leading 1 in two halves; the longitude's the same; the altitude, m; a
  velocity; VDOP; the hemispheres' letters, `N` and `W` in the two bytes of one word; the
  navigation status and the satellite count in the two bytes of another; the speed and course
  over ground; and a word no export writes, from 814 to 2,272.
- **The exports** hold every record but the last, in order: the CSV and text files their times
  and numbers exactly, their positions as `DDMM.mmmmm` and a letter or, in 5 of the 9, in degrees
  cut toward zero at the sixth decimal; their altitude the meters × 3.280 84 rounded down
  to a foot. The KML writes the meters as written and the degrees cut the same way.
- **Units.** The altitude's meters rest on the manual and on the KML's meters and the CSV's feet,
  which mDACS writes from the same word; and, reported only, in four flights the climb from the pad to the highest fix is 0.98 to 1.15 times the highest
  altitude of the PerfectFlite altimeter flown beside the tracker (feet would give about 3.3).
  The fifth's list is full and holds a climb of 0.003 times the altimeter's: its memory filled
  before the flight. The course lies 1.7° to 6.9° from the bearing between fixes, the median
  difference either way in each file. The speed over ground is,
  by file, 3.5 to 4.0 times the speed between fixes in meters a second, as km/h would be, but
  feet a second would give 3.3 and no document says which; the velocity reads far more than
  the climb rate while the rocket climbs and about the fall rate as it falls.
- **The radio link's marks:** in three files a pair of records repeats the pair before it a
  second back, and in one, one record's high time word reads 65,282 where every other reads 2.

**Decision.**

1. **Two increments.** M7.1c12 reads the `.rtx` (`LogFormat::RtxRaw`) and the CSV and text
   exports (`LogFormat::RtxCsv`, `LogFormat::RtxText`) in `hpr_flightdata::rtx`. M7.1c13 keeps
   M7.1c12's old entry and *done when*, the parent's, whole, without the RTx's files.
2. **Layout checks.** An `.rtx` is refused, at the first byte that differs, unless it is the
   list's shape above: the header, class record and members exactly, `_size` a whole number of
   records within the list's slots, an `Int32` array whose slots end in `MessageEnd` and the
   file; and refused at its slot for a slot in use that isn't a word under the mapping, or a slot
   past `_size` that isn't 0. An export is refused, by line, for a row of other than ten cells or
   a cell that isn't its field's.
3. **What is read.** The time of day, UTC, the log's zero the first record's, each time on the
   day that puts it nearest the one before (more than 12 hours behind, the next day's; ahead,
   the day before's); positions in degrees; the altitude in meters
   (the `.rtx`) or whole feet read as 0.3048 m (the exports), as `GnssAltitude`; the course in
   degrees; VDOP, the navigation status and the satellite count as numbers; the velocity, the
   speed over ground and the sixteenth word kept as written. A time that isn't a time of day,
   a coordinate out of range, and a position of (0, 0) are gaps, counted in a note. Records keep
   the file's order.
4. **The survey** decodes each `.rtx` from its bytes, apart from the reader, and holds it to the
   reader's record field for field, and to each CSV, text and KML export beside it: every record
   but the last, the positions equal or cut toward zero at the sixth decimal, the feet rounded
   down. A differing file fails `cargo xtask logs --check`. It reports, without checking, the
   speed and course against the positions and the climb against a barometric altimeter beside
   the tracker, as ratios and differences only.
5. **The mapping** keeps no one's work from view: the slots hold the owner's own fixes, and the
   project reads them, as it reads every logger's files, so its users can see their flights.

**Consequences.** All 14 files read: the 5 `.rtx` files agree with their 8 CSV and text exports
and 5 KML files on every record but the last, positions to under 10⁻⁶°. In four the CSV or text export compares every field; the fifth has
only a KML, which holds no times, so there the positions and altitudes are compared. The survey reads
947 of its 1,122 files and labels 119 files M7.1c13, 117 of them unread. The velocity and the
speed over ground stay in counts until a document or a better check gives their units.

**Done when (M7.1c12):** every RTx `.rtx`, CSV and text export in refs reads (`cargo xtask logs
--check` fails on one that doesn't); an `.rtx` that isn't the list's shape, whose `_size` isn't a
whole number of records within its slots, a slot in use of which isn't a word under the mapping,
or a slot past `_size` of which isn't 0, is refused at its byte, each by its own message, and an
export's row of other than ten cells or a cell that isn't its field's by its line (tests); the
time of day, the positions in degrees, the altitude in meters and the course in degrees are read,
and the velocity and speed over ground kept as written (tests); each `.rtx` with a CSV, text or
KML export beside it agrees with it on every record but the last, field for field, a position to
its written digit (the survey's report; a differing file fails `--check`); and the docs say what
was learned from the files, what is left out and how far to trust it.
