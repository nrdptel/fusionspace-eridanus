# ADR-272: M7.1c17 in two: the Blue Raven's USB download first; the rest to M7.1c18 (2026-10-15)

- **Status:** accepted; narrows M7.1c17 to a first increment and moves the rest, with M7.1c17's old entry and *done when* (the parent's), to M7.1c18; follows [ADR-271](0271-binary-records-by-a-stated-layout-before-the-rest.md)
- **Summary:** After M7.1c16, 54 sample logs were left unread. Two of them are a Blue Raven's recording as its USB port downloads it to a terminal, one line a sample, each closed by a CRC, in a layout the maker's manual describes. M7.1c17 reads these, keeping only the records whose CRC checks, each held to its stated length, and reads each field by the manual where the record holds what the manual lists. All 354,284 values of the 19,636 records read agree with the survey's own decoding. A third file is filed as a flyer's copy of a summary, not a log. The rest goes to M7.1c18.

**Context.** ADR-271 sorted the 54 files left after M7.1c16 into five groups: a course's Teensy
logger (8), one team's radio packets (4), altimeters' other text (8), proprietary files (7), and
tables a mapping settles or files to refile (27). In the altimeters' group, three files belong to
the Blue Raven. Two are a team's terminal captures of one USB download, its high-rate and
low-rate sets: lines such as `@ LOG_HIR  139 <date> <time> <sync> … CRC: XXXX`. The third,
filed by a flyer beside a Blue Raven's summary CSV, is that summary's 48 rows written as a table of
cells between `|`, by the same flyer's program that wrote the events file beside it, which the
survey already files as a reading, not a log; compared by hand, it holds the CSV's cells.

The *Blue Raven User's Guide* (Featherweight, updated May 12 2026, Appendix A, p22–24) describes
the download: each line opens with its mnemonic, "the length of the string, and the time and date
of the flight", then the sync code and the data, and the low-rate line closes with a "16b Buypass
CRC code for line of text". It lists each field and its scale: gyro rates in deg/s × 100,
accelerations in g × 100, the quaternion × 30,000, pressure in atm × 50,000, voltages in mV,
velocities in ft/s, positions in ft, tilt in degrees × 10. It doesn't say which text the CRC
covers. In the two files, CRC-16/BUYPASS (polynomial `0x8005`, starting at 0, no reflection, no
final XOR) over the text from `@` to the last character before `CRC:`, the spaces between left out,
checks on every piece that closes with one: 17,930 of 17,930 high-rate lines, and 1,706 low-rate
records, of 1,861 pieces of which 155 close with no CRC. The stated length is the line's length with its CR and LF in every one.

The capture lost some records. On some of the low-rate file's lines, records run together, each
but the last having lost its `CRC: XXXX` and its line end; with fragments, 155 pieces that open
`@ LOG_` close with no CRC, and none closes with one that differs.

The files' firmware writes other counts than the manual lists. A high-rate record holds 12 values
after the sync code where the manual lists 10, `V:` holds 5 where it lists 6, and `FER:` 8 where it
lists 5. The maker's forum names two 500 Hz fields beyond the manual's, the output current and the
aux voltage, but not their order (Featherweight, *Blue Raven development thread*,
rocketryforum.com, 2023-03-31). The manual's `Bo:` gives the baro sensor's temperature with no
unit.

**Decision.**

1. **Two increments.** M7.1c17 reads the Blue Raven's USB download; M7.1c18 keeps M7.1c17's old
   entry and *done when*, the parent's, whole. ADR-271's other groups are its plan.
2. **A record is what its CRC and length check.** `blueraven_usb` cuts every line before each
   `@ LOG_`, and keeps a piece only if it closes with a CRC that checks; a piece with no CRC, or
   whose CRC differs, is dropped and a note counts each kind, as is a line holding no record. A
   record whose CRC checks is the logger's own, so anything else wrong in it refuses the file at
   its line: a stated length not its own, another mnemonic, a date or time that can't be one, a
   value that isn't a whole number (hexadecimal in `FER:`), more than 256 values, or a layout
   unlike the first record's. A file in which no record checks isn't one. A file is recognized by
   a line among its first 1,000 that opens `@ LOG_HIR` or `@ LOG_LOW` and whose CRC checks.
3. **The manual's fields where the record holds them.** A group holding at least the manual's
   count is read by the manual in its order, and the values past it are kept as written; a group
   holding fewer is kept as written whole, as nothing says which is missing; a group the manual
   doesn't list is kept as written. A value kept as written is named by its group and place, such
   as `V: 3`, and a note says so. The baro temperature is kept as written. The registers are kept
   as numbers, with no events taken from their bits, which the manual doesn't name.
4. **The clock** is the logger's date and time of day, in milliseconds, counted from the first
   record's; a time that doesn't increase refuses the file. A note gives the first record's date
   as written, as the manual says the date "starts with the build time/date" unless the phone sets
   it: the files' date isn't the one the team's file names give the flight.
5. **The summary's copy is not a log**: filed `blueraven-summary-copy-txt`, as its events file is.
6. **The check.** For each download, the survey works out each piece's CRC and length itself and
   decodes each checked record by its own copy of the manual's table and rule; the reader must keep
   exactly the records that check, and each of their values, the clock's included, must equal the
   reader's. Three physical checks hold parts of the reading to the files, the other fields resting
on the manual's scales alone: each high-rate quaternion's norm
   must be within 2 of 30,000, as a unit quaternion × 30,000 cut to whole numbers is; the low-rate
   pressure on the pad, the median of the first 20 records, must be within 2 % of the TeleMega's in
   the same rocket; and the first register's lowest bit, the one the app's files call `Liftoff`,
   must first set within 0.1 s after the high-rate file's first acceleration above 3 g. A file that
   fails any is as unread as a refusal.
7. **A correction.** The Blue Raven CSV reader's docs said the app writes the quaternion with its
   magnitude fourth, as the manual says of the download; in every high-rate file among the sample
   logs `Quat_1` is the largest on the first row. The docs now say the app writes it first. The
   column is a number kept as written, so no value changes.

**Consequences.** The survey reads 1,008 of its 1,122 files. The 2 downloads hold 19,636 records,
17,930 high-rate and 1,706 low-rate, every one whose CRC checks; 354,284 values compared with the
survey's own decoding, none differing. The quaternion's norm runs from 29,998.25 to 30,000.00; the
pad pressure is 0.9977 of the TeleMega's; the liftoff bit sets 0.024 s after the high-rate file's
first acceleration above 3 g.

The download records no flight. Its acceleration stays above 3 g for at most 0.056 s at a time, and
its greatest mean over 1 s is 1.68 g, where the TeleMega in the same rocket records a boost. Its
pressure moves by 0.16 % of its pad's over the whole file, while the TeleMega's falls to 0.773 of
its pad's. So the liftoff bit was set by a short spike, not a launch, and none of the download's
channels, the gyroscopes' and accelerometers' included, describes the TeleMega's flight. The files
don't say why. The liftoff check holds only that the reader finds the bit at the spike the
high-rate file records; it is no check of a launch. The reading keeps the values as written, and
the report prints these measures. 51 files are left to M7.1c18.
