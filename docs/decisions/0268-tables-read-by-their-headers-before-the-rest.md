# ADR-268: M7.1c13 in two: tables read by their headers first; the rest to M7.1c14 (2026-10-14)

- **Status:** accepted; narrows M7.1c13 to a first increment and moves the rest, with M7.1c13's old entry and *done when* (the parent's), to M7.1c14; follows [ADR-267](0267-missile-works-rtx-before-the-rest.md)
- **Summary:** After M7.1c12, 117 sample logs were left unread, most of them tables from teams' own flight computers and scripts, each in its own words. Thirty-one of them, among them a Blue Raven file a person edited, name their units in their headers; M7.1c13 now reads such a table by what its header states, holds each unit to the column's name and values, and takes a caller's column mapping for the rest; spreadsheets and the other layouts go to M7.1c14.

**Context.** `cargo xtask logs` on main after M7.1c12 (#549) leaves 117 files unread: 60 filed as
generic CSV, 22 as text, 24 spreadsheets, and smaller families (4 binary files, 2 Entacore AIM
XTRA files, 2 RTF telemetry streams, 2 XML-like files, a ground station's conversion of a
SillyGoose flight). Most of the CSV files are tables no maker documents: a team's flight
computer, a data-logging script or a spreadsheet export writes a header row and one row a sample,
each in its own words. In 30 of them, and in a Blue Raven low-rate file a person edited in a
spreadsheet, counted as damaged since M7.1c7, the header names a clock with a unit of time, such
as `Time (s)`, `time_ms`, `time__us` or `Flight_Time_(s)`, and most state the units of other
columns in the same way: in brackets at the name's end, `Alt AGL (ft)`, `Pressure
[hPa]`, or in an underscored ending, `baro__kpa`, `acc_y__m_s2`,
`pitch_rate_deg_per_s`. The rest state no unit, or have no header row.

[ADR-258](0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md) asks two checks of a
unit no header states, and keeps a column as written when they disagree. A unit a header states
is the file's own statement: one check. The second can't come from a maker here, as there is
none; it comes from the column's name, which may say another quantity (`vz_filtered_ms`, a speed's
name and a time's unit), and from its values, which may contradict the unit: in two tables a
column headed as a pressure in pascals reads more on the pad than any sea-level pressure on
record; in three, a column headed as a current in amperes reads past 100 A, milliamperes by its
size.

**Decision.**

1. **Two increments.** M7.1c13 reads a table by its header (`LogFormat::GenericTable`,
   `hpr_flightdata::generic`) and by a caller's column mapping (`generic::Mapping`,
   `generic::read_with`). M7.1c14 keeps M7.1c13's old entry and *done when*, the parent's,
   whole: the spreadsheets, Entacore AIM XTRA, CATS, EuRoC and Juno, the RTF telemetry, the
   tables whose headers state too little, which the mapping reads once a person states it.
2. **Recognition, last.** The reader is tried after every other layout, so it takes no file
   another reader knows. Its header is the first line within the first 64 that isn't blank, a
   `#` comment or a `sep=` line, has at least two cells and no number among them, and names a
   clock: a time word with a unit of time. The next row must hold a number there.
3. **A unit rests on the header and one more check.** A column is read in its unit only where the
   name states one the reader knows and the name's words don't say another quantity, an
   estimate's error or a difference (`Alt error (m)`, a pitot tube's `Diff press (Pa)`); a length
   or a time only under a name that says an altitude or a time, since `z (m)` may be a height or a
   distance; and a unit that may stand for one the reader doesn't know, `g` or `mg` for grams or
   gauss, or a single letter, only under a name that says what it measures. Every other column is
   kept as written, with a note of its own. The file's decimal mark is settled once, a comma only
   where one stands before other than three digits and no number has a point, and a cell in the
   other mark is a gap, so a thousands separator is never read as a decimal mark. A column read in a unit is
   kept as written instead if any value, converted, lies outside what the quantity can be on a
   flight logger (pressure 0 to 110 kPa, temperature −100 °C to 150 °C, length 150 km, speed
   3,000 m/s, acceleration 1,000 g, rate of turn 50,000 °/s, voltage and current 100,
   latitude 90°, longitude 180°). The clock is refused if its median step, converted, lies
   outside 0.1 ms to 60 s, or if it never steps forward, as in a file of one row. No unit is
   guessed from values alone.
4. **No primary track.** Which column is a barometric altitude above the pad isn't stated, so the
   record's track is empty and every column is a channel; an altitude whose sensor or zero the
   name doesn't say is the new `Quantity::Altitude`. Readings from these tables wait for M7.2.
5. **A column of words** is an event at each change of its word if its name says a state, phase,
   status, event or mode, and is otherwise left out with a note.
6. **The survey** files the 31 tables as `table-csv`, by id, and reports each table's columns by
   decision (read in a stated unit, kept by the header, kept by the values, text) as counts, so
   the public report names no column a private file writes; and, where one file writes one
   quantity under one name in two units, the largest difference between them.

**Consequences.** All 31 read: 224 columns in a stated unit, 364 kept as written by their headers,
5 by their values, 11 of text; 6 files give events. The one file that writes its altitude and
speed in both feet and meters agrees with itself to 4.5 × 10⁻¹³ m. The survey reads 978 of its
1,122 files and labels 89 M7.1c14, 87 of them unread. Three one-row files of a team computer's
saved state, not logs, are refused by the clock's check. A table's numbers are only as right as
its header's units: a header that states the wrong unit, inside the bounds, is read in it.

**Done when (M7.1c13):** every file of the `table-csv` family in refs reads (`cargo xtask logs
--check` fails on one that doesn't); a column is read in a unit only where its name states one
the reader knows and its words don't say another quantity, a length or a time only under an
altitude's or a time's name, and is kept as written otherwise, with a note (tests); a column
whose values contradict its unit is kept as written, and a clock whose median step contradicts
its unit, or that never steps forward, is refused (tests); a caller's mapping reads a file with
no header row, and a mapping whose unit doesn't measure its quantity is refused (tests); the
survey's report counts each table's columns by decision, naming none; and the docs say what is
read, what is left out and how far to trust it.
