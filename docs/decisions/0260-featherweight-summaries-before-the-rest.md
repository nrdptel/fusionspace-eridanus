# ADR-260: M7.1c6 in two: Featherweight's summaries first, each checked against the file beside it; the rest to M7.1c7 (2026-10-13)

- **Status:** accepted; narrows M7.1c6 to a first increment and moves the rest, with M7.1c6's old entry and *done when* (the parent's), to M7.1c7; follows [ADR-259](0259-raw-downloads-against-their-makers-exports-first.md)
- **Summary:** After M7.1c5, 21 families of sample logs, 312 files, were left to M7.1c6, 310 of them read by no reader. M7.1c6 now reads the Featherweight app's summaries of a Blue Raven's or a GPS tracker's flight, 48 files: every row is kept as a figure, in SI only where the unit it writes is one its name allows, and each summary's stated maximum and pad altitude are checked against the file of samples beside it. 31 files that aren't flight logs are filed as such: Debrief's made-up flights, which say so, teams' checklists, a Blue Raven's settings sheets and a flyer's own program's reading of a log. The rest goes to M7.1c7.

**Context.** `cargo xtask logs` on main after M7.1c5 (#528) leaves 21 families to M7.1c6, 312
files, 310 of which no reader reads. A survey of all 21 shows few are ready:

- **Featherweight's summaries** (48 files, under the catalog's `blueraven-summary-csv` and
  `featherweight-summary-csv`; 36 are a Blue Raven's, 12 a GPS tracker's) are `name,value` rows,
  the unit written after each number (invented: `2500.25 feet`, `500.50 feet/sec`, `12.0 Gs`). The Blue
  Raven's manual describes the same summary as its USB port prints it (*Blue Raven User's Guide*,
  May 12 2026, p31–33), with each field's unit, and those words agree with the app's but for the
  main's descent rate, which the app writes in `feet`. 33 of the 36 Blue Raven summaries have a
  low-rate CSV beside them, 11 of the 12 tracker summaries a tracker's CSV.
- **Not logs** (31 files): Debrief's six sample flights, whose first row says each "is not a
  recording of anything"; four spreadsheets of a team's pre-flight checklist; nine spreadsheets of
  a Blue Raven's deployment settings (`Hardware_Channel:` rows); and twelve text files a flyer's
  own program wrote from a Blue Raven's low-rate file, titled `Blue Raven Events from` the file
  they read.
- **The rest** need more than an increment holds: no published document found describes the
  record layout of AltOS's eeprom files, and AltOS's GPL source may not be read (the
  project is clean-room), so their layout must be worked out from the files against the AltOS CSVs beside them; the
  RDAS's 54 files hold sensor counts and no calibration; ARTS2's raw data and `.odf`, the RTx's
  files, the zip archives, spreadsheets and the one-off CSV and text layouts need decoding work
  or the column mapper.

**Decision.**

1. **Two increments.** M7.1c6 reads the two summary families. Its *done when*: every sample of
   those families in refs imports (`cargo xtask logs --check` fails on one not read); a figure is
   in SI only when the unit it writes is one its name allows, else kept as written in a note the
   survey counts; each summary's stated maximum altitude and pad altitude are compared with the
   file of samples beside it, the agreements counted by the survey, and `--check` fails on one
   that agrees only were its feet meters; and a value a conversion makes infinite is refused.
   M7.1c7 keeps M7.1c6's old entry and *done when*, the parent's, whole.
2. **A summary is a record with figures and no samples.** `Stated`, what a file says about the
   flight, grows `figures`: every row, its name and value as written, its quantity, and its value
   in SI, one number a component. Its apogee and pad elevation fill `Stated::apogee_m` and
   `ground_elevation_msl_m`, as other loggers' stated figures do. The clock, tracks and channels
   are empty, as a GPS log's barometric track is: the samples are in the file beside it.
3. **A unit written beside its value is read as a header's is, only where the name allows it.**
   Each known figure names the quantity it is (`Max velocity`, a speed) and the words the files
   write for it (`feet/sec`, `ft/sec`); a value in one is converted, and one in any other word,
   or with none, is kept as written, never guessed. The main's descent rate in `feet`, and the
   four pad accelerations and offsets, each three numbers with no unit (the manual gives the USB
   form's in g × 10,000; the app's are not scaled so), are kept as written in every file. A
   zero is kept: what a zero means is M7.2's reading.
4. **The lengths are checked against the file beside.** The survey compares each summary's
   maximum altitude with the highest altitude above the pad of each file of samples in its
   folder (a tracker's: its highest GNSS altitude less the summary's pad altitude), and its pad
   altitude with the median of each file's first 20 altitudes above sea level. A figure agrees
   within 10%: feet and meters are 3.28 apart, so any band narrower than a factor of 1.81 tells
   them apart. Measured on the samples: 32 of the 33 Blue Raven summaries with a file beside agree
   in both, the apogee at 0.970 to 1.009 and the pad at 0.977 to 1.003; of the tracker
   summaries, 5 of the 5 stating an apogee agree (0.9995 to 1.0000) and 10 of 11 in the pad
   (0.977 to 1.000). The one of each that agrees in neither agrees with no file beside it, not even
   as meters; the survey doesn't show why.
   No summary agrees only as meters. A folder may hold other flights, so agreeing is counted,
   not required; agreeing only as meters contradicts the written unit and fails the check.
5. **Files that aren't logs are filed as not logs**, as pictures and maps are: the Debrief
   samples and the events files by family, the checklists and settings sheets by id.

**Consequences.** `LogFormat` grows `FeatherweightSummaryCsv`; `Quantity` grows `Duration`;
`Stated` grows `figures` and is no longer `Copy`, and a `Figure` type holds one. The survey's
report gains a table of the summaries against the files beside them. The readings withhold on a
summary's record, which holds no samples, and `hpr analyze` reads only PerfectFlite's `.pf2` so
far. The summaries inside zip archives and spreadsheets wait for M7.1c7's containers. The
docs that pointed to M7.1c6 for the other loggers' formats point to M7.1c7.
