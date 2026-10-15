# ADR-269: M7.1c14 in two: Excel workbooks first; the rest to M7.1c15 (2026-10-15)

- **Status:** accepted; narrows M7.1c14 to a first increment and moves the rest, with M7.1c14's old entry and *done when* (the parent's), to M7.1c15; follows [ADR-268](0268-tables-read-by-their-headers-before-the-rest.md)
- **Summary:** After M7.1c13, 87 sample logs were left unread. The largest group one change can read is the 23 `.xlsx` workbooks: written out sheet by sheet, each cell as the workbook stores it, 16 of them read through the readers that exist; M7.1c14 now reads them, every cell held to an independent reader, and the rest goes to M7.1c15.

**Context.** `cargo xtask logs` on main after M7.1c13 (#551) leaves 87 files unread: 24
spreadsheets (23 `.xlsx`, one `.xls`), 30 tables filed as generic CSV, 22 text files, 4 binary
files, 2 Entacore AIM XTRA archives, 2 RTF telemetry streams, 2 MARSA downloads and a ground
station's conversion of a SillyGoose flight. No group but the spreadsheets can be read by one
change: the 13 CSV tables whose clock names no unit and the 11 with no header row need a column
mapping someone states, which ADR-268 already takes; the next largest single layout is 7 text
files of one logger's tagged sensor lines, and the binary, XTRA and MARSA files have no published
layout. The `.xlsx` files are tables a reader already knows, saved in a spreadsheet: a Blue
Raven's high-rate and low-rate files pasted side by side, a Featherweight tracker's CSV, an
Eggtimer's, tables naming their units in their headers. The `import::read` entry point refuses
them as zip archives.

An `.xlsx` file is a zip archive of XML parts, laid out by ECMA-376 (*Office Open XML File
Formats*, 5th edition, Part 1 §18 SpreadsheetML, Part 2 for the package). The crate already
unpacks archives within a bound (M7.1c7) and parses XML with `roxmltree`. But the largest sample
sheet is 60 MB of XML, and a tree of it would take several times that, so the sheets are read as
a stream.

**Decision.**

1. **Two increments.** M7.1c14 reads `.xlsx` workbooks (`hpr_flightdata::workbook`); M7.1c15
   keeps M7.1c14's old entry and *done when*, the parent's, whole: the older `.xls`, Entacore AIM
   XTRA, CATS, EuRoC and Juno, the RTF telemetry, the tables whose headers state too little.
2. **A sheet is a file.** `workbook::sheets` writes each worksheet's cells as comma-separated
   text, one line a row; `workbook::read` gives each sheet's text to `import::read`, which
   recognizes it as the file it came from. No reader learns about spreadsheets, and one workbook
   can give several logs. `import::read` keeps refusing a workbook, pointing to `workbook::read`.
3. **Cells as stored, never as shown.** A number is written in its stored digits, a string as its
   text (rich-text runs joined, phonetic guides left out), a formula as its saved value, an error
   or a formula with no saved value as a gap counted in a note, a line break in a cell as a space.
   Rows are padded to the sheet's widest; a row with no value is left out.
4. **Dates by the number format, one rule.** A number under a format whose first section holds
   `y`, `d` or a month's `m` is a date, `h`, `s` or a minute's `m` (right after `h` or right
   before `s`) a time; `[h]`, `[m]`, `[s]` an elapsed time. The built-in formats are classified
   by the same rule from their codes in §18.8.30 (ids 14 to 22 and 45 to 47); East Asian built-ins
   (27 to 36, 50 to 58), whose codes the standard leaves to the locale, keep the number. Days count
   from 1899-12-30, or from 1904-01-01 under `date1904`; a date before 1900-03-01, where Excel's
   1900 calendar holds a day that never was, or past 9999-12-31, keeps the number, with a note.
   Seconds are rounded to the millisecond.
5. **Bounds.** The parts come out within the archive's 256 MiB; at most 256 MiB of text is
   written from one workbook, counted as each cell is read, so a shared string repeated in every
   cell can't grow without end; parts read whole (relationships, workbook, styles) are at most
   16 MiB; sheets and shared strings are streamed with `quick-xml` (MIT, no default features,
   no entity but XML's own). A document type declaration, an unknown entity, cells or rows out
   of order or past `XFD1048576`, a value its type can't hold, and two sheets on one part are
   refused.
6. **The survey** files the 16 workbooks that read as `workbook-xlsx`, by id, read when a sheet
   of each reads, and counts each workbook's sheets by outcome; 3 workbooks that are a flyer's
   summary of flights are filed as not logs (`flight-summary-xlsx`). `cargo xtask logs --sheets
   DIR` writes the workbooks and their sheets' text for `validation/oracles/xlsx/cells.py`, which
   reads every cell again with openpyxl 3.1.5 (MIT), run as an outside reader, and compares.

**Consequences.** All 16 workbooks read: of their 28 sheets, 21 read (6 as a Blue Raven's CSV, 2
as its app's summaries, 2 as a Featherweight tracker's CSV, 1 as an Eggtimer's, 10 as tables read
by their headers), 3 hold only charts, and 4 are refused. openpyxl agrees on all 5,743,747 cells
that hold a value (`validation/oracles/xlsx/cells.json`), 147,958 of them dates and times;
moving the 1900 epoch by one day makes 27,103 disagree. The survey reads 994 of its 1,122 files
and labels 70 M7.1c15, 68 unread. Left in M7.1c15: a PerfectFlite export with a person's notes
beside the table, a state estimator's output whose clock names no unit, two tables with no
header, and the `.xls` (a bare BIFF stream). A date-formatted number is only as right as its
format: a seconds clock a person formatted as a time is read as a time of day. The docs site's search
index passed mdBook's 10 MB warning with the workbooks' section (10.02 MB), and every page left
in it is one a reader needs, so a search now lands on a page's `##` section rather than its
`###` one (`book.toml`, 9.06 MB); #414, a search that grows without that limit, stays open.

**Done when (M7.1c14):** every file of the `workbook-xlsx` family in refs reads, a sheet of it
read as a log (`cargo xtask logs --check` fails on one that doesn't); a cell is written as the
workbook stores it (a number's digits, a string's text, a formula's saved value, an error as a
gap) and a number under a date or time format as its date and time, by one rule for built-in and
custom formats and both date systems (tests against hand-derived text); a part out of order, past
`XFD1048576`, declaring a document type or naming an entity, or text past the bound is refused
(tests); every cell of the sample workbooks agrees with openpyxl's reading
(`validation/oracles/xlsx/cells.py`); the survey's report counts each workbook's sheets by
outcome; and the docs say what is read, what is left out and how far to trust it.
