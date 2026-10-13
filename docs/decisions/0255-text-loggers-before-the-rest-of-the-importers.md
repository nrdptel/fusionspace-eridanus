# ADR-255: M7.1c2 in two: four text loggers first (AltimeterCloud, Entacore AIM, PerfectFlite's `.pfa`, an Eggtimer TRS's terminal text), the rest to M7.1c3 (2026-10-13)

- **Status:** accepted; narrows M7.1c2 to a first increment and moves the rest, with M7.1c2's old entry and *done when* (the parent's), to M7.1c3; follows [ADR-254](0254-gps-tracks-first-among-the-other-importers.md)
- **Summary:** After M7.1c1, 26 families of sample logs, 503 files, still had no reader: binary downloads, spreadsheets, companion summaries, teams' own CSV and text, and four text layouts a single maker writes. M7.1c2 now reads those four, 54 files: AltimeterCloud's CSV and Entacore AIM's, each with the generic `csv` family's files in its layout moved to it by id, PerfectFlite's `.pfa` and an Eggtimer TRS's terminal text. M7.1c3 keeps the rest and the parent's *done when*, in a planned order: club and app text layouts, raw downloads checked against their makers' exports, AltOS's raw files and the RDAS's, then containers, summaries and the column mapper.

**Context.** `cargo xtask logs` on main after M7.1c1 (#517) counts 26 families no reader reads,
from 1 file to 140. A survey of their files sorted them by what reading them takes:

- **One maker's text layout, each in a handful of files:** AltimeterCloud's CSV of a Rocketry
  Ltd Nano V1's or Mercury's flight (1 file in its family, about 10 more filed as generic `csv`), Entacore
  AIM's CSV with a time column before each channel group (1, and about 15 filed as `csv`),
  PerfectFlite's `.pfa`, an Alt15K/WD's flight as one altitude a line with no sample rate in the
  file (25), and an Eggtimer TRS's terminal download, its settings and summary above the table its
  CSV download writes (2). Debrief, the project owner's earlier analyzer (MIT), reads the first
  two.
- **Club and app layouts:** AerospaceNU's flight computer and SillyGoose logs (35), an app's
  70-column `launch…csv` (27), ARTS2 and RTx exports, a semicolon-separated logger.
- **Raw downloads:** the Raven's `.fipa` (57, 55 with a FIP CSV of the same flight), the RRC3's
  `.rff` and Missile Works' RTx (.NET binary serialization), ARTS2's `.odf`; each but the `.rff`
  has its maker's own export of the same flight to check a decoder against.
- **AltOS's raw files and the RDAS's:** AltOS's `.eeprom` and telemetry (16), whose provenance
  needs care (AltOS is GPL), and the RDAS's raw counts (54), which carry no calibration.
- **Containers, summaries, the column mapper:** spreadsheets (37), zip archives (8), the Blue
  Raven's and Featherweight's summary files (48), a flyer's event notes (12), Debrief's six
  invented flights (two of them in its column mapper's own layout), and teams' one-off CSV and
  text.

**Decision.**

1. **Two increments.** M7.1c2 reads the four single-maker text layouts. Its *done when*: every
   `altimetercloud-csv`, `entacore-aim-csv`, `pfa` and `eggtimer-txt` sample in refs imports,
   the generic `csv` family's files in AltimeterCloud's or the AIM's layout moved to those
   families by id, as M7.1a's reclassified files are (`cargo xtask logs --check` fails on one not
   read); a unit, clock or sample rate a file doesn't state is decided from the file or a source,
   in a note the survey counts. M7.1c3 keeps M7.1c2's old entry and *done when*, the parent's,
   whole; it splits again when it is next, in the order above.
2. **Debrief's invented flights go with the column mapper.** Four of the six hold an Eggtimer's or
   a Featherweight tracker's columns under a preamble, but two are in the layout Debrief made to
   show its column mapper (`Elapsed,Height,Rate`), which only that mapper reads; the family stays
   whole, in M7.1c3.
3. **A file filed as generic CSV moves by id.** The catalog files a CSV by its extension when it
   doesn't know the logger. A file in AltimeterCloud's or the AIM's layout moves to that family
   in the survey's table of reclassified files, by its id, with the reason, so `--check` holds
   it to that reader; one the reader doesn't recognize fails the check.

**Consequences.** `LogFormat` grows `PerfectFlitePfa`, `EggtimerText`, `AltimeterCloudCsv` and
`EntacoreAimCsv`. The readings withhold on these records, as on every logger's but
PerfectFlite's `.pf2`, until M7.2. The docs that point to the other loggers' formats point to
M7.1c3.
