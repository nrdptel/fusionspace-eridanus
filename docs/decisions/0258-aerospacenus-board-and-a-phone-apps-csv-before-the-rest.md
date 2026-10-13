# ADR-258: M7.1c4 in two: AerospaceNU's flight computer board, a phone app's `launch` CSV and an Eggtimer's RTF first, the rest to M7.1c5 (2026-10-13)

- **Status:** accepted; narrows M7.1c4 to a first increment and moves the rest, with M7.1c4's old entry and *done when* (the parent's), to M7.1c5; follows [ADR-256](0256-three-club-and-team-layouts-before-the-rest.md)
- **Summary:** After M7.1c3, 23 families of sample logs, 425 files, still had no reader. M7.1c4 now reads the group ADR-256 left open over unit questions, 54 files: AerospaceNU's flight computer board's CSV (20), its ground station's text (4), a phone app's `launch` CSV (27), and the 3 Eggtimer downloads saved as RTF ([#518](https://github.com/nrdptel/fusionspace-eridanus/issues/518)). It also makes one rule for every reader: a value that a unit conversion turns infinite is refused ([#520](https://github.com/nrdptel/fusionspace-eridanus/issues/520)). M7.1c5 keeps the rest and the parent's *done when*.

**Context.** `cargo xtask logs` on main after M7.1c3 (#521) counts 23 families no reader reads.
ADR-256 held back AerospaceNU's board and the phone app's files over open unit calls. A survey of
their files since settles most of them from the files:

- **The board's pressure is in atmospheres.** Every pressure in the 20 board files is a whole
  number of pascals divided by 101,325 (the largest residual 5 × 10⁻¹⁰ atm over 248,000
  samples); divided by 100,000 almost none is. At the four pads well above sea level, the pad's
  pressure altitude read in atmospheres lies within 26 m of the GPS altitude, and in bars 82 to
  113 m from it. Near sea level the two readings are 110 m apart and the weather moves either
  by as much, which is where ADR-256's "the files disagreeing" came from.
- **One board's accelerometer reads 1.5 g at rest:** the FCBV2 in both of its files, whose
  scaled columns use another part's scale factor. The board's own estimate beside them reads 1 g.
- **State codes come in three numberings,** none published.
- **The ground station's text** is the board's radio downlink, one Python-literal packet a line
  under a `RUN START` line, in the board's units.
- **The phone app's files come in two generations.** In the 8 newer ones (67 or 69 columns) a
  pressure column labelled `hPa` and a barometric altitude settle the altitude's unit, and the
  GPS's gain against it the GPS's. The 19 older ones (35 columns) have no barometer and nothing
  else in the file that sets the GPS altitude's unit, and their acceleration reads 2.0 at rest.

The rest is binary downloads, AltOS's raw files, the RDAS's counts, spreadsheets, containers,
summaries and teams' one-off layouts (the planned order below).

**Decision.**

1. **Two increments.** M7.1c4 reads `aerospacenu-fcb-csv`, `aerospacenu-gs-txt` (the ground
   station's files, told by their `RUN START` line), `phone-launch-csv` and `eggtimer-rtf` (the
   app's and the RTF files moved by id from `csv` and the not-a-log list). Its *done when*: every
   sample of those families in refs imports (`cargo xtask logs --check` fails on one not read);
   a unit a header doesn't state, or that a file's values contradict, is decided from the file
   or a source by two independent checks, the file refused or the column kept as written when
   they disagree, in a note the survey counts; and a value a reader's unit conversion makes
   infinite is refused. M7.1c5 keeps M7.1c4's old entry and *done when*, the parent's, whole,
   in this order: raw downloads checked against their makers' exports (ARTS2 raw and `.odf`,
   RTx, `.fipa`, RRC3 `.rff`), AltOS's raw files and the RDAS's, then containers, summaries,
   the other teams' layouts and the column mapper.
2. **A unit needs two checks that agree.** Following M7.1c3's review, no unit rests on one
   check. The board's pressure: its size on the pad, near 1, leaves atmospheres and bars of
   the units a barometer is written in, and its step, exactly 1/101,325 of the value, leaves
   atmospheres, an arithmetic mark of the conversion that no flight or weather can mimic. The
   pad's pressure altitude against the GPS's isn't one of the two: the weather moves it as far
   as atmospheres and bars are apart. A length rests on a gain against a column of known unit
   and a second, independent test. A column no pair of checks settles is kept as written, never
   guessed: the older phone files' GPS altitude and acceleration among them.
3. **One rule for infinite values.** A cell is already refused when it isn't a finite number as
   written. A finite cell that a conversion makes infinite, such as `1.7e308` g, is refused the
   same way, by every reader, so that a channel holds SI values or `NaN` gaps and nothing else.
4. **An RTF file is unwrapped, not parsed.** The three RTF files are an Eggtimer's terminal text
   inside an RTF document. The reader takes the document's plain text (control words, groups and
   escaped characters) and reads it as the Eggtimer text reader does; anything else in an RTF
   document is refused by name.

**Consequences.** `LogFormat` grows `AerospaceNuFcbCsv`, `AerospaceNuGroundStationText`,
`PhoneLaunchCsv` and `EggtimerRtf`. The readings withhold on these records, as on every logger's
but PerfectFlite's `.pf2`, until M7.2. The docs that pointed to M7.1c4 for the other loggers'
formats point to M7.1c5.
