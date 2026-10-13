# ADR-256: M7.1c3 in two: three self-checking layouts first (AerospaceNU's SillyGoose logs, Silicdyne Fluctus's CSV, the ARTS2's interpreted data), the rest to M7.1c4 (2026-10-13)

- **Status:** accepted; narrows M7.1c3 to a first increment and moves the rest, with M7.1c3's old entry and *done when* (the parent's), to M7.1c4; follows [ADR-255](0255-text-loggers-before-the-rest-of-the-importers.md)
- **Summary:** After M7.1c2, 22 families of sample logs, 449 files, still had no reader. ADR-255 ordered what was left: club and app text layouts first. A survey of those found three layouts whose units each file can settle, from its header, its maker's manual or a check on its own values: AerospaceNU's SillyGoose flight computer's logs (14 files), Silicdyne Fluctus's CSV (6, filed as generic CSV) and the ARTS2's interpreted data (4, filed as generic CSV). M7.1c3 now reads those 24. The others in that group need a decision the files can't make yet, and stay in M7.1c4 with the parent's *done when*.

**Context.** `cargo xtask logs` on main after M7.1c2 (#519) counts 22 families no reader reads.
The first group in ADR-255's order, club and app text layouts, held:

- **AerospaceNU's SillyGoose logs (15 files):** a board's own tab-separated log under a header
  whose names carry most units (`pressurePa`, `tempK`, `altitudeM`); one file with a `CONFIG`
  settings line and more columns; one space-separated file whose values contradict its header's
  units; and one ground station's conversion of a flight, 28 columns with no header in another
  order.
- **Silicdyne Fluctus (6, filed as generic CSV):** a `sep=;` line, then a header with each unit in
  brackets, decimal commas, and Silicdyne's published manual for the rest.
- **The ARTS2's export (7 beside 3 `.odf`, filed as generic CSV):** 4 files of interpreted data,
  `# MET Acc Vel iAlt pAlt`, some in English units and some in metric with nothing in the file to
  say which; and 3 of raw sensor counts with no calibration in the file.
- **AerospaceNU's flight computer board's CSV (20) and its ground station's text (4):** pressure
  near 1.0 that is atmospheres or bars, 1.3% apart, the files disagreeing; one board's
  accelerometer scaled wrongly; state codes in two numberings, unpublished.
- **A phone app's 70-column `launch….csv` (27, filed as generic CSV):** no published format; in
  most files the GPS altitude's unit can be settled only against another logger's file.
- **Missile Works' RTx text exports (9):** a `Velocity` column whose scale differs by orders of
  magnitude between files.

**Decision.**

1. **Two increments.** M7.1c3 reads the three layouts that settle their own units. Its *done
   when*: every `aerospacenu-sillygoose-txt`, `fluctus-csv` and `arts2-csv` sample in refs
   imports, the Fluctus and ARTS2 files moved from the generic `csv` family by id (`cargo xtask
   logs --check` fails on one not read); a unit a header doesn't state, or that a file's values
   contradict, is decided from the file or a source, or the column kept as written, in a note the
   survey counts. M7.1c4 keeps M7.1c3's old entry and *done when*, the parent's, whole.
2. **A unit the values refute is not taken from the header.** A SillyGoose file headed `tempK`
   whose temperatures are below zero, or whose accelerometer reads 1 at rest, is read by its
   values: the reader checks each named unit against the file and keeps a column as written,
   with a note, when the check fails. A unit no check or source settles is never guessed.
3. **An ARTS2 file's units come from its own flight.** The manual (*ARTS2 User Manual* V1.0.0,
   Ozark Aerospace, 2007) gives the export's units nowhere, and the files come in both systems.
   The velocity falls at g through apogee, where the drag vanishes, 9.81 a second in m/s and
   32.17 in ft/s; the acceleration's integral and the altitudes' agreement with the velocity
   settle the rest. A file whose velocity or barometric altitude no check settles is refused.
4. **New families by id.** The ARTS2's raw-count files move to `arts2-raw-csv` and the ground
   station's headerless SillyGoose conversion to `aerospacenu-sillygoose-gs-csv`, both M7.1c4's:
   the first with the `.odf` decoder, the second with the column mapper, since nothing in the file
   says which column is which.

**Consequences.** `LogFormat` grows `SillyGooseText`, `FluctusCsv` and `Arts2Csv`. The readings
withhold on these records, as on every logger's but PerfectFlite's `.pf2`, until M7.2. The docs
that pointed to M7.1c3 for the other loggers' formats point to M7.1c4.
