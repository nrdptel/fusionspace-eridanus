# ADR-270: M7.1c15 in two: EuRoC exports and mappings a publisher states first; the rest to M7.1c16 (2026-10-15)

- **Status:** accepted; narrows M7.1c15 to a first increment and moves the rest, with M7.1c15's old entry and *done when* (the parent's), to M7.1c16; follows [ADR-269](0269-excel-workbooks-before-the-rest.md)
- **Summary:** After M7.1c14, 68 sample logs were left unread, in 19 layouts, the largest 7 files. Nine of them need little new code: three exports of known loggers saved by a European locale's spreadsheet, and six tables whose publisher, RocketPy, states each column's meaning and unit. M7.1c15 now reads them, adds a check that keeps a filtered altitude written in the wrong scale out of the record, and leaves the rest to M7.1c16.

**Context.** `cargo xtask logs` on main after M7.1c14 (#552) leaves 68 files unread. Sorted by
layout, they fall into 19 groups: the largest are 12 headerless numeric tables from several
teams, 7 text files of one course logger's tagged sensor lines and 7 tables from RocketPy's
repository; then 4 files named after AltOS but not in its layouts, 4 of a team's radio packets,
3 EuRoC exports and smaller groups, among them the Entacore AIM `.xtra` and flash files, MARSA's
downloads and an AltiMAX `.ALS` with its `.xls`, none with a published layout. Two groups need
almost no new code:

- *EuRoC exports* (European Rocketry Challenge flights, from Debrief's fixtures): two Eggtimer CSV
  downloads saved with semicolons between the cells, which the Eggtimer reader refuses as it
  splits on commas, and an RRC3 mDACS export in a European locale's form (semicolons, decimal
  commas), whose lines all end in a semicolon, so its header holds an empty name the RRC3 reader
  doesn't recognize.
- *RocketPy's tables*: seven team logs RocketPy (MIT) keeps in its repository, whose headers name
  the clock `t`, `ts` or `time` with no unit, so the table reader of [ADR-268](0268-tables-read-by-their-headers-before-the-rest.md) can't
  take them. For six, RocketPy's example notebook for the flight (`docs/examples/`, commit
  9bd6ad3) reads the file with `FlightDataImporter`, a `columns_map` naming what each column is
  and `units=None`, which the importer's docstring defines as every column in SI. That is the
  publisher's own statement of the mapping ADR-268's `generic::read_with` takes. The seventh, a
  GNSS file, is loaded by no notebook.

Reading the EuRoC files showed two things the readers must not take on trust. In one Eggtimer
export a spreadsheet took the decimal points out: `12.34` is written `1234`, so the filtered
altitude and both speeds are 100 times their values while `Alt`, whole feet, is intact. The
Eggtimer reader's speed check catches the raw `Veloc` (it matches the change in `Alt` on 1.8% of
the rows), but `FVeloc` is checked against `Filt_Alt`, which is scaled the same way, so both would
pass and be read 100 times too large. And the RRC3 export's rows were sorted by their time's text,
`10,05` before `2`: put in order by value, they step 0.05 s from the first with none missing
or repeated.

**Decision.**

1. **Two increments.** M7.1c15 reads the EuRoC exports and the tables a publisher states a
   mapping for; M7.1c16 keeps M7.1c15's old entry and *done when*, the parent's, whole.
2. **Semicolons in an Eggtimer download.** A header that holds a semicolon is split on
   semicolons. The decimal mark is settled once a file, as ADR-268 settles it for tables: a comma
   only if no number is written with a point and one is written with a comma before other than
   three digits; a cell in the other mark is no number and is refused, naming its line, so a
   thousands separator is never read as a decimal mark. The shared rule moves to
   `table::decimal_comma_shown` and `table::number_in_mark`. A terminal download stays
   comma-separated, as the terminal prints it.
3. **Filtered altitudes checked against `Alt`.** A filtered altitude is read in feet only if the
   least-squares factor through zero that takes `Alt` to it lies from 0.1 to 2; otherwise it is
   kept as written, with a note, and so are the filtered speeds when it is the file's first. A
   first-order lag `τ` at a speed `v` over a climb or descent of height `H` moves the factor by
   about `1.5 v τ / H` (a ramp lagged by `v τ`; `Σ A / Σ A²` over a ramp from 0 to `H` is
   `1.5 / H`): down on the way up, by more the less of the climb the file holds, up on the way
   down by little, the descent being slow. A decimal place lost writes a value 10 times over.
   Measured over the 44 Eggtimer files with a filtered altitude, the factor lies from 0.900 to
   1.000; the export with its decimal points taken out gives 99.93.
4. **An RRC3 export's closing semicolon.** Under a semicolon-separated header that ends with a
   semicolon, a row's last cell, empty, is dropped. The reader keeps refusing a time that goes
   back: it doesn't reorder a file. The text-sorted export is filed as damaged
   (`rrc3-csv-damaged`), recognized and refused on the line where its time goes back.
5. **Mappings a publisher states.** `validation/log-mappings.json` holds, by file id, the
   `generic::Mapping` the file's publisher states and where it states it; committed, as the tables
   it maps are public. `cargo xtask logs` reads a file of the `mapped-table-csv` family by its
   mapping through `generic::read_with`, which holds every unit to ADR-268's values check, and
   refuses it if no mapping is stated, if a reader takes the file on its own, or if its header
   doesn't name the mapping's columns in order. A column the publisher doesn't map is kept as
   written. RocketPy's altitude is above ground level (`Flight.altitude`), its `vz` and `az` the
   vertical velocity and acceleration (`Flight.vz`, `Flight.az`). A mapping for a private file
   would need a home outside the public repository; none is needed yet.

**Consequences.** The survey reads 1,002 of its 1,122 files: both Eggtimer exports, as Eggtimer
CSV, and the six RocketPy tables, by their stated mappings, with no column's values contradicting
its unit; 59 files are left to M7.1c16. In the export whose decimal points were taken out, `Alt`
reads in feet and `Filt_Alt`, `FVeloc` and `Veloc` are kept as written, so its record has no
vertical speed. The RRC3 export is refused, though a flyer's file sorted this way holds every
sample: reading it would mean the reader reorders rows, which no other reader does and which
would hide a clock that truly steps back. A stated mapping is only as right as its publisher's
statement; the values check catches a unit far off (a pressure over 110 kPa, a length beyond
150 km), not feet read as meters.

**Done when (M7.1c15):** the two semicolon-separated Eggtimer exports in refs read as Eggtimer
CSV, in either decimal mark settled once a file (tests), and the RRC3 export whose lines end in a
semicolon is recognized and refused where its time goes back (filed as damaged); a filtered
altitude whose least-squares factor on `Alt` lies outside 0.1 to 2 is kept as written with its
filtered speeds (tests at both edges, a mutation probe), so the export whose decimal points were
taken out reads its `Alt` in feet and keeps `Filt_Alt`, `FVeloc` and `Veloc` as written; the 6
tables whose publisher, RocketPy, states their column mapping read by it
(`validation/log-mappings.json`, each citing where), a mapping refused on a header it doesn't name
or a file a reader takes (tests); `cargo xtask logs --check` fails on a file of these that doesn't
read; the guide's flight-log page says what is read and how.
