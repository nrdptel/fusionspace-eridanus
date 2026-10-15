# ADR-271: M7.1c16 in two: binary records by a stated layout first; the rest to M7.1c17 (2026-10-15)

- **Status:** accepted; narrows M7.1c16 to a first increment and moves the rest, with M7.1c16's old entry and *done when* (the parent's), to M7.1c17; follows [ADR-270](0270-euroc-exports-and-stated-mappings-before-the-rest.md)
- **Summary:** After M7.1c15, 59 sample logs were left unread. Five of them are a team's binary records with the table the team decoded from them beside each: three captures of a flight computer's serial shell dumping its flash as base64, and two raw files of records end to end. M7.1c16 reads files like these by a record layout the caller states, the names from that table and the types found from the bytes and that table. All 931,875 fields of the 45,915 records written match the teams' tables. The rest goes to M7.1c17.

**Context.** `cargo xtask logs` on main after M7.1c15 (#553) leaves 59 files unread. A survey of
them, file by file (not printed by the report, which counts by the catalog's families), sorts
them into six groups by what reads them and what can check the reading:

| Group | Files | The check a reading can be held to |
|---|---:|---|
| binary records with the team's decoded table beside them | 5 | that table, record by record |
| one course's Teensy logger, tab-tagged sensor lines | 8 | physical: its `MSL` column against the ISA of its pressure; GPS beside |
| one team's radio packets, `255:…:254`, in four forms | 4 | across the forms, and a workbook beside that reads |
| altimeters' other text: Blue Raven's raw lines, its summary as a table, MARSA, AltOS variants | 8 | a CRC on each Blue Raven line; files of the same flight that read |
| proprietary files: AltiMAX `.ALS` and `.xls`, headerless Raven FIP workbooks, Entacore AIM `.xtra` | 7 | an export beside some; none for the `.xtra` |
| tables a mapping settles, and files to refile as not logs (housekeeping, coordinates with no clock, a filter's output) | 27 | physical checks, or none needed |

Only the first group can be checked exactly, on every record. Three of its files are a terminal program's capture of a flight computer's USB shell: the commands
typed, then each dump as base64 lines between marker lines the firmware prints,
`----++++//[[( data start )]]\\++++----` and its `end`, 64 characters (48 bytes) a line. The
`data` block decodes to 100-byte records, a `u32` clock in milliseconds then 24 `f32`; a `params`
block holds the flight's saved state, which the team's tables file apart. The other two are raw
files with no header or marks: 12-byte records (`u32`, 2 `f32`) and 92-byte records (`u32`,
22 `f32`). Beside each file is a CSV the team decoded from the same records, whose header names
the fields; the record sizes, the types and the row counts were found from the bytes and those
tables. No team publishes its firmware's record layout under a license this project can read, and
neither file kind has marks a reader could recognize a layout by.

One capture's `data` block holds 13,655 records, of which the last 12,222 are wholly `0xFF`: flash
the logger never wrote, which the dump read past. The team's table writes them as rows of
`4294967295` and `nan`. A first survey reported all three captures as matching on every field; it
had counted those rows as matching without noticing that they aren't samples.

**Decision.**

1. **Two increments.** M7.1c16 reads binary records by a layout the caller states; M7.1c17 keeps
   M7.1c16's old entry and *done when*, the parent's, whole. Its groups above are a plan, not a
   split: each later increment names its own.
2. **A layout the caller states.** `records::RecordLayout` lists the fields in order, each a name
   and a type: an integer of 8 to 64 bits, signed or not, or an IEEE 754 binary32 or binary64,
   little-endian unless stated big. The records are written out as a table, the names its header,
   and read by the table reader of [ADR-268](0268-tables-read-by-their-headers-before-the-rest.md),
   so that a field is read in a unit only where its name states one and its values bear it out, and
   kept as written otherwise. A binary32 is widened to binary64 exactly and written as the shortest
   decimal that reads back to it, so the table's number is the field's value. A float that isn't
   finite is a gap, counted in a note. A layout of fewer than 2 fields or more than 512, a name
   empty, over 256 characters, repeated, a number, or holding a separator (a comma, a semicolon or
   `|`), a quotation mark, a control character or spaces at its ends, a first name opening with `#`
   or `sep=`, or names stating no clock with a unit of time, is refused (`LogError::Layout`); so is
   a table past 256 MiB of text. `records::table` gives the text for a caller who would rather read
   it by a column mapping.
3. **Erased flash.** A run of records each wholly `0xFF` at the file's end is memory never written:
   dropped, and a note counts it. So are bytes past the last whole record if all are `0xFF`, as a
   dump in 48-byte lines leaves them. Other bytes past the last record are refused at the first
   (`LogError::Binary`), as a layout that doesn't fit the file. An erased record with a written one
   after it isn't dropped, as dropping it would hide the logger's fault: its unsigned integers
   are at their largest, its signed ones −1 and its floats gaps, so a float clock has no time
   there and the table's reader refuses the record, named by its number and offset; with an
   integer clock the record is read, and a column its values contradict is kept as written.
4. **A console capture.** `capture::blocks` recognizes a capture by a start marker on a line of its
   own within its first 1,000 lines and decodes each block's lines as one strict RFC 4648 base64
   text: a character outside the standard alphabet, padding before the end or over two, bits the
   padding should leave zero that aren't (§3.5), a length not a multiple of 4, a block that never
   ends, a marker inside a block, an end with no block open,
   or two blocks of one name are refused at their line. `capture::read` reads a named block by the
   caller's layout. `import::read` recognizes a capture and refuses it, naming its first blocks, as
   it has no layout to read it by.
5. **Layouts for private files.** The samples are private, so `validation/log-layouts.json` holds
   for each file by id only the block, the types as runs of `[kind, count]` and which table beside
   it names the fields (`data.csv`, or the file's own name then `_parsed.csv`). The survey reads the
   names from that table's header at each run. Only the survey applies these layouts.
6. **The check.** For each file the survey decodes every record's fields itself from the file's
   bytes (a capture's block as the reader decodes its base64), apart from the reader's records,
   and compares them with the team's table row by row, field by field, a NaN equal only to
   a NaN; the table must have a row for every whole record, each of as many cells as the layout
   has fields. It then reads the team's table by its
   header, cut to the records the reader keeps, and the two records must hold the same channels and
   events. A file that fails either check is as unread as a refusal. As the teams decoded the same
   bytes their own way, a wrong base64 decoding would fail the first check too.

**Consequences.** The survey reads 1,006 of its 1,122 files. The 5 files hold 58,137 whole
records, 12,222 of them erased at one capture's end. The 45,915 records written hold 931,875
fields, the samples read, and every field of every record, 1,237,425 with the erased ones, matches
the teams' tables, none differing. Each record read holds the same channels as its team's table. The new families are
`console-capture` (3) and `record-dump` (2); 54 files are left to M7.1c17. The team's own table of
the capture with erased flash is read by its header with all 13,655 rows (the report's table
checks), its 12,222 erased ones among them; the binary reading drops them. A user with such a file states the layout once,
from the team's code or table. A file whose layout no one states stays unread, by design.
