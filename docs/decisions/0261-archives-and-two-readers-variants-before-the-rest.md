# ADR-261: M7.1c7 in two: zip archives and two readers' variants first; the rest to M7.1c8 (2026-10-13)

- **Status:** accepted; narrows M7.1c7 to a first increment and moves the rest, with M7.1c7's old entry and *done when* (the parent's), to M7.1c8; follows [ADR-260](0260-featherweight-summaries-before-the-rest.md)
- **Summary:** After M7.1c6, 233 sample logs in 16 families were left to M7.1c7, 231 of them read by no reader. M7.1c7 now unpacks the 8 zip archives, in memory and within a bound, and reads each file inside as a file of its own: 19 files no other source holds, 11 of them a Blue Raven's files and summaries. It reads PerfectFlite's text export written with no header row (4 files) and a CSV export that names its temperature `Temp` (1), and a Blue Raven high-rate file with no date (1). It files 16 more files as not logs, and one hand-edited Blue Raven file as damaged. The rest goes to M7.1c8.

**Context.** `cargo xtask logs` on main after M7.1c6 (#529) leaves 16 families to M7.1c7, 233
files, 231 of which no reader reads. A survey of them finds a few small groups that the readers
already written nearly read, and one container:

- **Zip archives** (8 files): a flyer's Blue Raven files with the app's summary, or a folder of
  what a flight left behind. Of their 29 files, 10 are copies of files already among the
  samples; the other 19 are 8 Blue Raven high- and low-rate files, 3 summaries, a GPS
  tracker's KML, and 7 files that aren't logs. The largest unpacks to 32 MB.
- **PerfectFlite's text export with no header row** (4 files, three filed as text and one as
  CSV): rows of four tab-separated cells, the third a temperature followed by its unit's letter
  (invented: `0.05`, `3`, `71.5F`, `9.2`), the text export's `Time`, `Altitude`, `Temp.`,
  `Voltage` without the header. One CSV export names its temperature `Temp`.
- **Blue Raven files without the date** (2 files): a high-rate file whose header opens
  `Flight_Time_(s),Sync`, the app's names otherwise; and its low-rate partner, which a person
  edited in a spreadsheet: a column with no name, two columns named in words the app doesn't
  write.
- **Not logs** (16 files, 7 of them inside archives): a flyer's own program's events read from a
  Blue Raven file and its description of a rocket, a team's description of a rocket, a team flight
  computer's saved state (one row), an OpenRocket simulation's export, a wind profile, and a Blue
  Raven's settings as CSV or a spreadsheet.
- Two CSV files beside one archive's Blue Raven files are named after other altimeters'
  formats and use the Blue Raven's column names, but their altitudes differ from the Blue
  Raven's low-rate file's at the same times by up to several hundred feet. Which logger wrote
  them isn't known; they are left to M7.1c8's column mapping, not read as a Blue Raven's.

The rest, 209 files, needs decoding work or the column mapper: AltOS eeprom and telemetry, the
RDAS, ARTS2 raw and `.odf`, RTx, spreadsheets, the generic CSVs and text.

**Decision.**

1. **Two increments.** M7.1c7 unpacks the archives and reads the two readers' variants. Its
   *done when*: every zip archive in refs unpacks and every file moved to a read family reads
   (`cargo xtask logs --check` fails on one that doesn't); an archive is held to 256 MiB counted
   as it decompresses, and two files of one name, links, encryption, other methods and damaged
   files are refused (tests on hostile archives); an archived file no source holds has a family
   by id, and the survey stops on one without; the new exports' altitude unit is checked as the
   others' are and reported; and the files that aren't logs are filed by family. M7.1c8 keeps
   M7.1c7's old entry and *done when*, the parent's, whole.
2. **An archive isn't a log.** `hpr_flightdata::archive::unpack` returns an archive's files,
   each a name and its bytes; each is then read by `import::read` on its own, so a folder of a
   flight's files reads the same zipped or not. `import::read` refuses an archive with a new
   error, `LogError::Archive`, that says to unpack it, rather than calling it unrecognized; a
   spreadsheet's `.xlsx`, itself a zip, gets the same refusal until spreadsheets are read. An
   archive inside an archive is one of its files and isn't opened.
3. **Bounds as the design container's.** At most 256 MiB comes out of one archive
   (`MAX_UNPACKED_BYTES`, as `.hprz`'s), counted as bytes decompress, never from the sizes the
   archive states; a central directory with more records than the zip reader keeps (two files
   of one name) is refused, as are links, a folder's entry holding bytes, encrypted files,
   methods other than stored and deflated, and a file whose CRC-32 fails, each by a test. Names are kept as written and never used as paths:
   nothing is written to disk.
4. **The survey unpacks after every source.** An archive's file that a source holds byte for byte
   is that source's; the others are samples of the source `archives`, each family from a table
   by id (`ARCHIVED`), decided as the catalog's are, never by HPR Sim's recognition. A file
   missing from the table, or an entry not met, stops the survey.
5. **The headerless export is recognized by its temperatures' unit letter.** Four
   tab-separated numbers, the third ending in `F` or `C`: no other layout HPR Sim reads writes a
   unit's letter after a number in such a row, and among the sample logs only PerfectFlite's
   text exports do. Its columns are taken as the text export's, with a note in the record. Its
   altitude is checked as every export's is: of the 21 exports now read, 16 peak at exactly the
   height of a `.pf2` beside them, 3 of the 4 headerless ones among them (one with its
   temperatures in °C), so the headerless layout writes feet. Of the other 5, read in feet the
   climb takes 0.63 to 1.07 times as long as a fall in vacuum from the peak, and read in meters
   0.35 to 0.59 times. The one at 0.63 is the fourth headerless file, with no `.pf2` beside it and
   an altitude ragged near its peak: its climb tells feet from meters no better than the other
   (0.35), so its unit rests on the layout's convention alone, and the docs say so.
6. **A Blue Raven file may open `Flight_Time_(s),Sync`.** The clock is found at the start of
   the header rather than fifth, and the record notes that the file holds no date. The
   hand-edited low-rate file is filed as damaged: the reader refuses a column with no name, and
   guessing what an edited column holds isn't reading the app's file.

**Consequences.** `hpr-flightdata` gains the `archive` module and depends on `zip` (deflate only,
already in the workspace for `.ork` and `.hprz`); `LogError` gains `Archive`. The survey's report
gains the `archives` source; the families not yet read are labelled M7.1c8. Spreadsheets inside
archives wait with the others for M7.1c8. The docs that pointed to M7.1c7 for the other loggers'
formats point to M7.1c8.
