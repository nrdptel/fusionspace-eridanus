# ADR-259: M7.1c5 in two: the Raven's `.FIPa` and the RRC3's `.rff` raw downloads first, each checked against its maker's export; the rest to M7.1c6 (2026-10-13)

- **Status:** accepted; narrows M7.1c5 to a first increment and moves the rest, with M7.1c5's old entry and *done when* (the parent's), to M7.1c6; follows [ADR-258](0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md)
- **Summary:** After M7.1c4, 23 families of sample logs, 376 files, were left to M7.1c5, 374 of them read by no reader. M7.1c5 now reads the two raw downloads whose makers' exports of the same flights sit beside them, 63 files: the Featherweight Raven's `.FIPa` (57) and Missile Works' RRC3 `.rff` (6). Each raw file with an export beside it must match that export in every sample of every channel the two share, and the survey fails on one that doesn't: all 55 such `.FIPa` files and the one such `.rff` match, 6,019,645 raw samples, with no value off by any step. The rest, ARTS2's raw data and `.odf`, the RTx's files and the families ADR-258 listed, moves to M7.1c6 with the parent's *done when*.

**Context.** `cargo xtask logs` on main after M7.1c4 (#527) leaves 23 families to M7.1c5, 376
files, 374 of which no reader reads. ADR-258 put the raw downloads first, in the order ARTS2 raw and `.odf`, RTx, `.fipa`,
`.rff`. A survey of the five showed that two are ready and the others are not:

- **The Raven's `.FIPa`** is the altimeter's memory as the Featherweight Interface Program (FIP)
  saves it: 11 text lines, then frames of a `:`, 256 bytes and a checksum. All 57 sample files
  decode with every checksum good, every settings block equal to its header's `#FlightState`, and
  `#DONE` after the last frame. Five firmware versions write them (3.0 in 50, 2.2 to 2.5 in 7).
  Of the 57, 55 have the FIP's CSV export of the same flight in their folder.
- **The RRC3's `.rff`** is a .NET `BinaryFormatter` stream (MS-NRBF) holding a list of 16-bit
  words, which Debrief, the project owner's earlier analyzer, already reads. All 6 decode, with
  one candidate list each and the once-a-second words counting the same seconds as the 20 Hz
  readings. One has mDACS's export beside it, in UTF-16 with a UTF-8 byte-order mark written a
  second time inside, which the importer didn't recognize.
- **ARTS2's raw data** (3 files) holds sensor counts whose calibration is in its `.odf` files (3),
  a layout no document describes; **the RTx's files** (5) have no export of the same flight. Both
  need more decoding work than one increment holds.

**Decision.**

1. **Two increments.** M7.1c5 reads `raven-fipa` and `rrc3-rff`. Its *done when*: every sample of
   those families in refs imports (`cargo xtask logs --check` fails on one not read); every one
   with its maker's export beside it matches that export in every sample of every channel the two
   share, within half a step of the raw format's resolution, counted by the survey, and `--check`
   fails on one that doesn't; a word the file holds no calibration for is kept as written; and a
   frame whose checksum fails, or a clock check that fails, is refused by name. M7.1c6 keeps
   M7.1c5's old entry and *done when*, the parent's, whole, with ARTS2's raw data and `.odf` and
   the RTx's files added to it.
2. **A raw file is checked against its maker's export.** The export is the maker's own reading of
   the same bytes, an independent answer for every sample. The survey pairs a raw file with each
   export in its folder that the export's reader reads, and counts a match when every sample of
   every channel the two share agrees: the value within half a step of the raw format's
   resolution, the time within what the export prints (six significant figures in the FIP's
   newer exports; mDACS's clock, 0.05 s summed in single precision, which runs a hundredth of a
   second ahead of `n / 20` from 156.65 s and reproduces all 3,541 of its export's times). An
   export with one clock for every channel holds each channel's last sample at every tick, and is
   compared that way: every raw sample must show at a row of its own time, none skipped. Only the
   accelerometer's velocity may stop before the raw file's last sample, as the FIP stops it at
   apogee; the survey counts those samples apart (140,865 in the 55 files), and every other
   channel must run to the end. The pairing is by folder, not by content, so that a
   decoding error shows as a difference rather than as a missing pair; the committed report pins
   how many pairs there are.
3. **The arithmetic is the maker's, found from the exports.** The FIP works the accelerometer's
   gain in double precision, `trunc((X / 1000) × counts)`: exact integer arithmetic differs from
   its export by a hundredth of a g on 11 of the files, and the survey's check caught it (a probe
   with integer arithmetic leaves 44 of 55 matching; one reading unused memory as data leaves
   none). Every other channel is
   a fixed scale of its word.
4. **What the raw file records, nothing more.** Neither file holds an altitude: the FIP's and
   mDACS's altitudes are their own conversions of the pressure. So both records have an empty
   primary track, as the GPS logs and the Blue Raven's high-rate file do, and working an altitude
   out of the pressure is M7.2's reading. Words the file holds no calibration for are kept as
   written: the RRC3's once-a-second pairs (its temperature and battery voltage in counts) and its
   single words, and the Raven's slow channel the FIP doesn't export and the two words after each
   block's samples.
   A Raven whose header says one axis (one file) keeps its lateral bits as counts, since it
   calibrates no lateral axis.
5. **Binary formats are told by their bytes.** The importer recognizes them before it decodes a
   file as text, and a refusal names the byte (`LogError::Binary`).

**Consequences.** `LogFormat` grows `RavenFipa` and `Rrc3Rff`; `LogError` grows `Binary`. The
importer's text decoding drops a UTF-8 byte-order mark written inside a UTF-16 file. The survey's
report gains a table of the raw files against their exports. Two `.FIPa` files have no export
beside them, and five `.rff` files have none at all: those decode by the same rules, unchecked by
an export. The one `.rff` with an export checks its pressure only: its pairs and single words
have no calibration to compare, and the exports' altitudes, flags and derived velocities are
their makers' conversions, not compared. The readings withhold on these records, as on every logger's but PerfectFlite's `.pf2`,
until M7.2. The docs that pointed to M7.1c5 for the other loggers' formats point to M7.1c6.
