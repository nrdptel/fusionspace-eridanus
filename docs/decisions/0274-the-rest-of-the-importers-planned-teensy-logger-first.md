# ADR-274: M7.1c18 in six: a course's Teensy logger first, then the five increments left (2026-10-15)

- **Status:** accepted; narrows M7.1c18 to a first increment and plans every remaining increment, M7.1c19 to M7.1c23, each with its own *done when*, so M7.1's last files are planned once; follows [ADR-272](0272-the-blue-ravens-usb-download-before-the-rest.md)
- **Summary:** After M7.1c17, 51 sample logs were left unread, in five groups. This record plans all of them at once, in six increments, the easiest to check first. M7.1c18 reads the 8 files of a university course's Teensy data logger, one sensor's reading a line, in two layouts. Where a file states no unit, the unit is held to the physics the files must keep: on every one of 305,340 `Alt` lines, the barometer's altitude follows its pressure by the BMP180's formula to within 5.4 mm (0.21 in); the accelerometer reads 0.983 to 0.989 g on the pad; and the time to apogee is 0.90 to 1.14 times √(2h/g). The other 43 files go to M7.1c19 to M7.1c23.

**Context.** ADR-271 sorted the files left after M7.1c15 into groups. After M7.1c16 and M7.1c17,
51 remain, which `cargo xtask logs` lists by id. Their groups, from a survey of each file:

| Group | Files | What could check a reading |
|---|---:|---|
| a course's Teensy data logger, one sensor's reading a line | 8 | physics: a barometer's altitude against its pressure, 1 g at rest, the time to apogee |
| one team's radio packets, `255:…:254`, in four forms: the onboard log, the packets a receiver heard, and two exports wrapped in RTF | 4 | the onboard log, packet by packet |
| headerless Raven FIP workbooks, and an AltiMAX `.xls` written as a bare BIFF8 stream | 3 | the `.FIPa` and FIP CSV of the same flights; the `.xls`'s heights against its pressures |
| altimeters' other text: MARSA's files, an AltOS telemetry CSV, an AltOS-like table and an AltOS CSV a spreadsheet saved with formula cells | 5 | other loggers' records of the same flights |
| tables a column mapping settles (20), and files that aren't logs (7) | 27 | physics, by the quantity each column measures |
| AltiMAX's `.ALS` and an Entacore AIM's `.xtra` files and raw flash dump | 4 | the `.xls` beside the `.ALS`; the `.xtra` and the dump beside each other |

A course's data logger, built around a Teensy board, writes the 8 files of the first group. The
course publishes its flights' files on its *Flight Data* page. That page lists the files as
"Teensy Data Logger" and "New Teensy Data Logger", read by the course's own scripts. No document
describes either layout, and the scripts carry no license this project can read under, so they
weren't read.

Seven files are the older layout: a tag, the clock and the values, separated by tabs, seven tags in
a cycle of about 23 ms (`A`, `G`, `M`, `Alt`, `HG` and two analog inputs). No line states a unit. One
file is the newer layout: four lines state each sensor's units, `ICM data in us, milli-g, deg/s,
milli-T, deg C`, and then lines of name–value pairs follow, `MS5, t, <µs>, T, <°C>, P, <Pa>`. It
is a 100 s cut from a longer recording, so its clock starts at 1,330 s.

The older layout's `Alt` lines carry the pressure and an altitude, `MSL`. Computed in 64-bit
floats, the altitude follows the pressure by the Bosch BMP180 data sheet's formula, 44,330 × (1 −
(P / 101,325)^(1/5.255)) (BST-BMP180-DS000, §3.6), to within 5.4 mm on every line. The standard
atmosphere's own form, 44,330.77 × (1 − (P / 101,325)^0.190263), is 0.09 to 0.21 m off. On 21,540
of the 305,340 lines the gap is more than the 5 mm half step the value is written to, and it is
greatest at the same pressure in every file, so it is the logger's own rounding of the formula,
whose exact form no document gives. The check allows 44,330 × 2⁻²³ m, 5.3 mm, for it:
the 44,330 m scale times a power near 0.9 good to about 2⁻²³.

The high-g accelerometer `HG` steps by 0.4788 m/s² if read in m/s², but it doesn't agree with `A`.
In the boost, while `A` reads from 3 g to 90 % of its greatest, `HG` is 1.37 to 2.39 times `A`, by
file. In the newer layout, the high-g `LIS`, read in its stated g, is 0.996 times the `ICM`
accelerometer over the boost. The newer layout states its magnetometer in milli-T. Earth's field
is 0.025 to 0.065 mT, and the values are of the order of a thousand times that.

**Decision.**

1. **Six increments, all planned here.** Each is one reader or one kind of decision, with its own
   check. M7.1c18 keeps M7.1c18's entry for the course's logger. The others, M7.1c19 to M7.1c23,
   follow in the roadmap with their own *done when*s, and M7.1's *done when* stays the parent's.
   The plan isn't narrowed again: a group that can't be read as planned is recorded in an
   amendment to this record, with its measurement, and the gap stays visible in the report.

   | Increment | Files | *Done when* |
   |---|---:|---|
   | M7.1c18 A course's Teensy data logger | 8 | all 8 read; every value equals the survey's own decoding; every `Alt` line's `MSL` is the BMP180's formula of its `P` to within 0.0103 m; each accelerometer read is within 3 % of 1 g on the pad and after landing; the time to apogee is within a factor of 2 of √(2h/g) |
   | M7.1c19 Radio packets, `255:…:254` | 4 | all four forms read; every packet of the received and RTF forms that the onboard log also holds matches it field for field; packets the onboard log doesn't hold are counted by kind (GPS, repeated, broken) |
   | M7.1c20 Workbooks with a reading beside | 3 | the two headerless Raven FIP sheets match the `.FIPa` or FIP CSV of the same flight sample by sample; the AltiMAX `.xls` reads from its bare BIFF8 stream, its columns by their stated units, and each height follows its pressure by one barometric formula whose constants the report states |
   | M7.1c21 Altimeters' other text | 5 | MARSA's two files read, their apogee within 5 % of another logger's in the same flight; the AltOS telemetry CSV reads by its header; the AltOS-like table reads by AltOS's state numbers; the CSV with formula cells reads, each formula kept as text |
   | M7.1c22 Tables by mapping, and files that aren't logs | 27 | the 20 logs read, each column's unit held to a physical check (a pressure's standard atmosphere, 1 g at rest, a track's own speed); the 7 others filed as not logs, each with its reason in the report |
   | M7.1c23 The last binaries, and formats with no sample | 4 | the `.ALS` decodes to the `.xls` beside it, value for value; the `.xtra` files and the raw dump decode against each other, or an amendment here records what the bytes show and the gap stays in the report; the guide lists every format the roadmap names that has no sample, such as CATS's and Juno's, as needing samples |

2. **The course's logger, two layouts** (`hpr_flightdata::teensy`). Each line is a tag, a clock in
   whole microseconds, and values: by place, named `A 1`, or by name, `Alt P` and `ICM xa`. Each tag
   is a channel set on its own clock, read as seconds from the logger's zero. The record has no
   barometric track. A line that isn't so, a tag's line laid out unlike its first, or a clock that
   doesn't increase within a tag refuses the file at its line. A clock past 2³² µs is refused, as
   the logger's count runs over there. A last line the file ends in with no line end, which can't
   be read, is dropped with a note. The bounds are 32 tags, 64 values on a line, and names of 32
   bytes.
3. **Units the older layout doesn't state are held to physics.**
   - `A` is read in m/s², as written.
   - `Alt`'s `P` is read in Pa, and `MSL` as a pressure altitude in m.
   - The rest is kept as written: the gyroscope's, whose unit no check settles; the
     magnetometer's, a quantity HPR Sim doesn't hold; `HG`, whose scale disagrees with `A`'s; and
     the analog inputs.
4. **The newer layout's units are its own.** Each units line's first unit is the clock's and must
   be `us`. The other units go to the fields' groups, a name less a leading `x`, `y` or `z`, in the
   order the tag first names them; a single unit goes to every field. A tag whose units don't fit
   so, or which has no units line (the file's `VOL`), is kept as written, and a note says so. So
   is `milli-T`.
5. **The check.** The survey decodes each file itself and compares every value with the reader's.
   Four physical checks hold the units:
   - **`MSL` against the BMP180's formula** of `P`, to within 0.005 m + 44,330 × 2⁻²³ m: the
     half step and an allowance for the logger's rounding of the formula.
   - **The accelerometer at rest:** its magnitude on the pad (the 5 s ending 1 s before liftoff)
     and over the last 2 s (after landing) is within 3 % of 1 g. A unit other than the one read
     would be 3 or more times off.
   - **The time to apogee:** from liftoff (the first magnitude above 3 g in the 30 s before
     apogee) to apogee, within a factor of 2 of √(2h/g). A clock misread by a power of ten would
     be 10 or more times off.
   - **The pad pressure,** for the named layout, is between 50 and 110 kPa.

   Each check also needs the reader to have read the values it holds in the quantities it takes
   them in (acceleration, pressure, altitude above sea level), so a unit the reader assigned
   wrongly fails it. A file that fails any check is as unread as a refusal.

**Consequences.** The survey reads 1,016 of its 1,122 files. The 8 files' 6,188,639 values equal
the survey's own decoding. Their worst `MSL` gap is 0.0054 m. Their pad magnitudes run from 0.983
to 0.989 g, and over their last 2 s, after landing, from 0.997 to 1.008 g. Their times to apogee are
0.90 to 1.14 times √(2h/g). The newer layout's pad pressure is in the band; the report says so
without the value, which would give the site's elevation. The pad window is the one the rocket
stands in: over the first 2 s of the recordings, minutes before launch, while the logger was
handled, three files read 1.034 to 1.048 g. 43 files are left, to M7.1c19 to M7.1c23.

**Amendment for M7.1c19, the radio packets (2026-10-15).** The group's four files are one flight's:
the flight computer's own log of its packets, the text a ground receiver wrote of what it heard,
and two exports of the log, cut to the flight. The plan stands; these are the reader's decisions.

1. **One reader, two formats** (`hpr_flightdata::packets`). `RadioPacketText` reads the onboard
   log and the receiver's text by one rule; `RadioPacketRtf` reads the exports. A packet is `255`,
   a path letter, a two-digit stage, a two-letter kind, a whole-number clock, its values and `254`.
   The path is `T` for the flight computer and `N` for a second transmitter that relays its packets
   and sends the GPS packets (kind `GG`, stage `88`); it isn't kept, as a packet's
   fields don't depend on the path it came by. Each kind is a channel set on its own clock, its stage a channel too; a `Stage` line, its
   word a number's from `One` to `Nine`, is an event at the next packet's clock. A kind whose first
   packet is on stage `88` runs on the relaying transmitter's own clock, ahead of the flight
   computer's by an offset nothing states: its times are gaps and
   its clock is kept as written, in `GG clock`. The survey fails a received or exported packet the
   onboard log doesn't hold unless it is a GPS packet.
2. **Broken and repeated packets are dropped and counted, not refused.** The receiver's text holds
   packets the radio cut or ran together, and many packets twice. One rule for both text forms
   keeps the reader simple; the survey shows the onboard log has no broken line, so nothing is
   lost there. A packet whose kind has another count of values than its first is broken.
3. **The exports are read line by line, not unwrapped.** A spreadsheet split the RTF document's
   lines at the colons and saved them as CSV, so the markup no longer nests: the opening group's
   line ends in commas, and only some lines keep the `\` that broke them. The reader drops each
   line's trailing commas, the control words before a packet, a closing `\` and the document's
   last `}`; a line of markup alone is dropped. ADR-258's unwrapping of the Eggtimer's RTF reads a
   whole document and would join these lines.
4. **Units held to physics.** The clock is read in milliseconds and `PP 1` in hectopascals, held
   by the survey to the onboard log: the pad's pressure between 500 and 1,100 hPa, and the time from
   liftoff to apogee within a factor of 2 of √(2h/g). The rest is kept as written. `PP 3` climbs
   with the flight but, where it reads above 200, reads 1.11 to 1.24 times the standard atmosphere's height in meters, so no
   unit is read for it.

**Consequences of the amendment.** The survey reads 1,020 of its 1,122 files. It decodes the four
files itself, and the 1,315,788 values it compares equal the reader's. Of the receiver's 42,796
whole packets, 38,475 are the onboard log's packets of the same stage, kind and clock, every one
field for field (29,914 relayed, 8,561 direct); the other 4,321 are GPS packets, which the onboard
log doesn't record; 8,368 are repeats and 10,495 lines are broken. The exports' 4,939 and 2,469
whole packets all match the onboard log's; one export has 1 broken line. The onboard log has no
broken line. The time from liftoff to apogee is 1.055 times √(2h/g), and the pad's pressure is in
its band. 39 files are left, to M7.1c20 to M7.1c23.

**Amendment for M7.1c20, the workbooks with a reading beside (2026-10-16).** The group's three
files are two workbooks, each a Raven FIP export pasted below no header with the `.FIPa` of the
same flight beside it, and an AltiMAX altimeter's export to `.xls`. The plan stands; these are what
the files showed, measured, and the reader's decisions.

1. **The FIP sheets are read under a stated header** (`hpr_flightdata::raven::read_rows`, the
   header in `validation/log-headers.json`): the one Debrief's public sample of a Raven flight
   carries, 16 channels, each with its clock and status column. Both sheets' rows hold 48 cells
   in its pattern until a channel stops. A sheet writes every row as wide as its widest, so a
   row may end in empty cells; a first row that is itself a header is refused, as `raven::read`
   reads such a file whole.
2. **One sheet has a table pasted over two of its columns.** Columns 34 and 35 hold
   a clock and a value pasted over the export's cells, where the header puts `[Altitude
   (Accel-Ft)]`; from the row where the first channel stops, where the export's rows shift one
   cell left, they cover the cells of whichever channel stands there. `raven::read_rows_around` reads such a sheet by one rule: a channel whose sample, in any
   row, has a cell in a pasted column is left out whole, with a note; every other channel is
   read; and a row whose pasted cells hide whether a channel has a sample there (its time and
   value cells, and the cell after them, pasted over or empty) is refused. The rule leaves out three of the
   five channels the FIP works out and reads the other 13, among them the 11 the `.FIPa` holds,
   which always sit in a row's first 33 cells.
3. **The `.xls` stream isn't BIFF8's.** The plan called it a bare BIFF8 stream, from its BOF's
   record id, `0x0809`, which BIFF5 to BIFF8 share. Its BOF's body is 6 bytes with a version of
   0, where Excel writes 8 bytes and `0x0500` for BIFF5 and 16 bytes and `0x0600` for BIFF8 (Rentz,
   *OpenOffice.org's Documentation of the Microsoft Excel File Format*, §5.8). Rentz reads such a
   tool-written BOF's version of 0 as BIFF5 (§5.8.2); the cells settle it, as they are BIFF2's
   records: 50,668 `INTEGER`, 202,672 `NUMBER` and 25,345
   `LABEL`. xlrd 2.0.2, run as an outside reader, reads the file as BIFF2. `hpr_flightdata::xls`
   reads BIFF2's cell records under any BOF and refuses the later versions' cell records by
   name. The file is read from its bare stream, as planned; its format is named for what it
   holds.
4. **Its pressures aren't in their stated unit.** The two pressure columns say `[hpa]` and hold
   pascals: read in hectopascals, the pad's pressure would be a hundred times any launch site's,
   and the heights would miss the standard atmosphere's height of their pressures by more than a
   kilometer. The reader takes each pressure column in the unit that puts its first value, on the
   pad, between 500 and 1,100 hPa, which only one of the two can, and says so in a note. Every
   other column is read in its stated unit; `TIME` states none and is read in seconds, held to
   its 5 ms step. The plan's *by its stated units* holds for every column but these two.
5. **The checks** (`cargo xtask logs`, the report's sections for each):
   - each sheet against its `.FIPa`, sample by sample, as the raw downloads are checked against
     the FIP's own exports, every `.FIPa` channel of a known quantity matched by name;
   - four of the five derived channels the `.FIPa` doesn't hold, where a sheet keeps them, which
     back the stated names (`[Altitude (Accel-Ft)]` is held to nothing):
     `[Altitude (Baro-Ft-ASL)]` is the standard atmosphere's height of the sheet's own pressure,
     44,330.77 × (1 − (P / 101,325)^0.190263) m, less one offset, within the printed half steps;
     `[Altitude (Baro-Ft-AGL)]` is that less one constant, and reads within 3 m of 0 on the pad,
     which a swap of the two names would fail; `[Velocity (Accel-MPH)]` is `[Velocity
     (Accel-Ft/Sec)]` in m/s;
   - the `.xls`'s cells against the survey's own decoding of its records, and its values against
     the text read by the stated units, but the pressures in pascals;
   - each of its heights, filtered and raw, against the same formula of its pressure in pascals,
     less one offset for the file, within 44,330.77 × 2⁻²³ m, 5.3 mm, the allowance the Teensy
     logger's check takes for a software's own working of a formula no document gives;
   - its pad pressure between 50 and 110 kPa; `ACCEL`'s magnitude before liftoff within 15 % of
     1 g, where a column in g or ft/s², read as m/s², would be 9.8 or 3.3 times off; `SPEED`'s
     change over the boost within 10 % of `ACCEL`; and the time from liftoff to apogee within a
     factor of 2 of √(2h/g).

**Consequences of the amendment.** The survey reads 1,023 of its 1,122 files. The two sheets
match their `.FIPa` in 40,841 and 48,290 samples, none differing: the worst time is 0.00005 s, the
FIP's printing, and the worst value 0 steps. Their ASL is the formula's height of their pressure
to within 0.004 m and 0.020 m, their AGL is ASL less one constant to within 0.0016 m, and it reads
0.20 m and 0.72 m on the pad. A test on invented rows with the two altitudes' names swapped fails
the pad check, with the ASL and AGL checks still holding. The `.xls`'s 278,685 cells and 253,340 values equal
the survey's decoding, and xlrd agrees on every cell. Its heights follow the formula to within
0.0045 m over 50,668 pairs. `ACCEL` reads −0.940 g before liftoff: its axis points down the
rocket, where one pointing up would read +1 g at rest. `SPEED` is `ACCEL` summed over the clock,
its change 0.999 times `ACCEL` over the boost, so it isn't the rocket's speed. The time to apogee is 0.945 times √(2h/g). 36 files are left, to
M7.1c21 to M7.1c23.

**Amendment for M7.1c21, the altimeters' other text (2026-10-16).** The group's five files are two
MARSA downloads, a table of AltOS's radio telemetry, a table cut from an AltOS export, and an
AltOS export a spreadsheet re-saved with formulas in its cells. The plan stands; these are what the
files showed, measured, and the readers' decisions.

1. **MARSA's file is described nowhere,** so `hpr_flightdata::marsa` takes its layout from the two
   files: settings as `<NAME> value` lines, a section's heading as `<NAME>` alone, then rows of
   three unnamed numbers, read as seconds, feet and g. The manuals give the altitude in feet and
   the software's plot of "Baro altitude and acceleration" (*MARSA4 User Manual*, 2009, p. 18).
   One file is the other layout re-saved by a spreadsheet, its settings' lines ending in empty
   cells and its numbers rewritten; both read the same. The catalog files them as XML, for their
   `<…>` lines; they are moved by id. Each setting is a figure as written, as no document gives
   their units. A file opens with a setting and names `<MARSA VERSION>` in its first 200 lines.
2. **The other logger in the same flight** is the AltOS board whose export sits in the MARSA's
   flight folder: the catalog gives the flights no shared id. For one file the flight card names
   both as one stage's loggers; for the other the card gives the MARSA no stage, and the folder's
   only other log is labelled as the other stage's, so the board is the MARSA's stage's by
   elimination. The survey names each pair by id. The MARSA's pad reads three counts below its
   own zero, −87 ft, in both files, and the reader keeps the altitude as written, a height above
   the logger's zero; the plan's apogee is the highest altitude so read, which is the harder
   reading to pass, as measured from the pad the ratios lie nearer 1.
3. **The telemetry table is read by its header,** `serial,tick,ptype,…`, each column by its name
   and the fields of *AltOS Telemetry*'s `0x09` packet (§2.3, Table 6). The document sends the
   voltages as converter counts; the table holds volts, 4.1 for the battery, a single LiPo
   cell's (*The Altus Metrum System*, §3.2), so they are read as volts. A row whose `crc` is
   `FALSE` is left out and counted; a tick no later than the one before refuses the file, as a
   table of seconds can't tell the tick's 655.36 s turn from a restart.
4. **The table cut from an export is read by AltOS's states.** Its header is AltOS's names alone,
   with no `version,serial,flight`; of its first 100 rows, one must give a `state` and a
   `state_name` that AltosUI's exports pair, 2 `pad` to 8 `landed`, and none may contradict them,
   and a later row that does refuses the file. In a full export the same contradiction is only
   counted in a note, as Debrief's sample export names state 1 `pad`: refusing it would
   unread a file the reader has read since M7.1a.
5. **Formulas are kept as text in notes, not in a new field.** The record has no channel of text,
   and adding one would touch every reader's record; a note keeps each column's formulas run by
   run, a reference to the cell's own row written `{row}`, so each formula the note gives (up to
   8 runs a column, 200 characters a formula and 64 columns) can be rebuilt from it, and the
   survey rebuilds each one. A column of formulas alone isn't a channel; a column
   with some reads them as gaps. A unit after a column's name reads the column as AltOS's where it
   is the unit the reader reads it in, and keeps it as written otherwise. The file has no
   `height`, which the reader required until now; such a record has no primary track, only
   its channels, with a note.
6. **The checks** (`cargo xtask logs`, the report's sections for each): each reader's
   record against the survey's own decoding, every channel's values and times and the primary
   track's; MARSA's apogee within 5 % of the AltOS board's, and
   its barometric climb over the first 2 s after liftoff within 0.6 to 5/3 of the acceleration
   integrated twice, the band the `.eeprom` reader's climb check takes, and its acceleration
   before liftoff within 0.5 g of 0, nearer 0 than 1 g; the telemetry's height change over its
   speed and its speed change over its acceleration, from boost to coast, each within the same
   band, which a value left in the packet's sixteenths, a height in feet or a tick in its counts
   would miss, its battery between 2.5 and 5 V and its ground pressure between 50 and 110 kPa;
   the cut table's `altitude` less `height` one offset within 0.01 m, its `altitude`'s rise over
   the standard atmosphere's rise of its `pressure` within the same band, and its events AltOS's
   names; and each formula rebuilt from the notes against its cell. All three ratio checks share
   one band, so that a unit is held to one rule. The MARSA's step of a count, from which the
   reader's altitude resolution is taken, and the telemetry's resolution are reported as
   measures: neither can fail for a wrong unit.

**Consequences of the amendment.** The survey reads 1,028 of its 1,122 files. The MARSA's apogee is
0.981 and 0.986 times the AltOS board's (read in meters, 3.2 times); its climb is 0.915 and 1.035
times the acceleration in g, where m/s² would give 9.0 and 10.1; its acceleration before liftoff
is −0.12 g; and all 5,712 comparisons with the survey's decoding agree. The telemetry table's 325
rows are all type 9 with good CRCs, its 12,025 comparisons agree, its height change is 1.019 times
its speed and its speed change 0.929 times its acceleration, and its battery reads 4.09 to 4.10 V.
The cut table's 1,005 rows all give AltOS's state pairs, its 15,948 comparisons agree, its
`altitude` less `height` lies within 0.005 m of one offset, and its altitude rises 1.000 times its
pressure's height. Debrief's sample export, read since M7.1a, gains a note counting its state 1
named `pad`; its values are unchanged. The re-saved export's 80,748
formulas all rebuild from the notes, and its 4,198,896 comparisons agree. 33 files are left, to
M7.1c22 and M7.1c23.

**Amendment for M7.1c22, the tables a mapping settles and the files that aren't logs
(2026-10-16).** The group's 27 files are 20 logs whose headers state too little or that have
none, and 7 files that aren't logs. The plan stands; these are what the files
showed, measured, and the decisions.

1. **Four things a mapping can now say** (`hpr_flightdata::generic::Mapping`), each found in a
   file the old mapping couldn't read:
   - a clock that turns over: one team's board counts microseconds in 32 bits, which turn over
     every 71.6 minutes, four times in each of its six files, and prints the count 9 × 2³² high in
     the upper half of each turn. The reader takes each value modulo the turn and adds a turn at
     each step back of more than half of one;
   - rows written as lists, `[…, True, True]`, as the same team's ground station saved its radio
     telemetry;
   - stamped rows, `YYYY-MM-DD HH:MM:SS.ffffff: reading`, a reading or a tuple `(x, y, z)`, as a
     payload logged its barometer and accelerometer. A line with no stamp, the payload's
     `===DETECTION MOUVEMENT===`, is an event at the next row's time;
   - a column left out: the cells about the flight that a flyer typed beside an altimeter's table
     pasted into a workbook.
2. **The mappings name a private table's columns only by their own labels or its header**
   (`validation/log-held-mappings.json`). A column whose header names it takes the name from the
   file at each run, as the layouts of M7.1c16 do; a file with no header gets the survey's own
   labels, such as `pressure 1`. Sixteen of the tables are in the private collection; the one
   name the file gives from a private header is the last of a header that runs onto its first
   row, a sensor's name. The four public tables' columns are named as their publisher names them.
3. **Each unit the mapping states, and the clock's, is held to a physical check**, one rule for
   each kind of reading, with the bands earlier increments set and two new ones, the overlap of
   a clock beside another and the clock rate of a recording with no flight (`cargo xtask logs`,
   the report's section for this increment). A file that fails any check fails the survey,
   listed among the unread. Liftoff is the last reading before apogee within 5 % of the climb.
   - The clock: the time from liftoff to apogee within a factor of 2 of √(2h/g), as M7.1c18's;
     for a file with no height, its span of times overlapping another file's of the same logger
     by at least half of the longer, or, for a recording on the pad with no flight in it, its
     median step within 10 % of the same logger's in a file with a flight. A cut of a file is
     held by its rows, each a row of the whole file.
   - A pressure: its reading on the pad within 50 to 110 kPa.
   - An altitude: its rise over the standard atmosphere's height of the file's own pressure, over
     another logger's in the same flight, or over the apogee the flyer states, each within 0.6 to
     5/3, M7.1c21's band, which no unit off by 3.28 (feet for meters) or more reaches. A vertical
     speed: the altitude's change over it from liftoff to apogee, in the same band.
   - An accelerometer: its median magnitude over the recording within 15 % of 1 g, M7.1c20's band.
     A recording spends most of its time on the pad or under a parachute, so the median is the
     magnitude at rest.
   - A track's latitude and longitude, in degrees, each on its own: the median size of its
     readings other than 0 (a receiver writes 0 before its first fix) past ±π/2 for a latitude
     and ±π for a longitude, which no place has in radians.
   Apogee is the last of the highest heights of the readings' running median over 5, and the
   pad the lowest of them over the 300 s before apogee:
   more than twice the 122 s a rocket at Mach 3.5, the simulator's limit, about 1.2 km/s, takes
   to coast to apogee against gravity alone. Over a whole recording, one board's barometer read
   lower heights hours before launch than on the pad, and taking the lowest as the pad put
   liftoff hours early, a time to apogee 316 times √(2h/g) (measured while the checks were
   built; not in the report); and a workbook's table starts at liftoff, so its first 5 s are no
   pad.
4. **What is kept as written.** No check here tells °C from °F at a launch's temperatures: 24 °C
   read as °F would be −4 °C, a launch's temperature too. So every temperature is kept as written,
   and so is every rate of turn, as no check holds one, and every other column no check holds,
   such as a battery's voltage, humidity, a magnetometer and an orientation's quaternion. The
   team's board's two accelerometer axes read 0 at rest, with gravity taken out, and the axial
   one, in a probe while the checks were built, read with the sign opposite to the other
   loggers' in the rocket, so they are kept too. So are words, packed fields and counts.
5. **A track's own speed, which the plan named, holds no unit.** Between fixes, on the pad, a
   track's speed has a median of 0 m/s, whatever the unit. Under the parachute it is the wind's,
   a few meters a second, and near 0 on a calm day. One bad fix reaches 235 to 1,200 m/s
   (measured while the checks were built; not in the report). No band admits every flight's
   drift and refuses a clock misread by a power of ten or degrees read as radians, which are 57
   times smaller. So the track's units are held by its place; its clock by the time to apogee of
   its own altitude or by the clock of a file of the same logger; and its altitude, where it has
   one, by another logger's.
6. **The seven files that aren't logs**, each filed with its reason in the report: a flight
   computer's housekeeping (processor, memory, disk and network counters); a battery pack's (its
   cells' voltages, temperatures and current); two GPS tracks' coordinates copied from their map
   files, with no clock, the tracker's own file of the same fixes beside them; and three tables
   a program worked out from a log: a Kalman filter's state, whose altitude reaches 169 km; an
   inertial unit's recording turned into the rocket's frame with gravity taken out; and a flyer's
   rebuild of an altimeter's log, whose rows the flyer marks as made up, the altimeter's own file
   beside it already read. The last two are judgement calls: each is a time series of a flight,
   but not what a logger wrote.

**Consequences of the amendment.** The survey reads 1,048 of its 1,122 files. The 20 tables give
3,976,354 rows. Their mappings read 64 columns in a unit, keep 102 as written and leave 7 out, and
each of their 60 checks passes:
- the 12 times to apogee lie 0.8627 to 1.7105 times √(2h/g);
- the 6 rises over the standard atmosphere's lie 0.9559 to 1.0005 times, the 3 over another
  logger's 0.9008 to 0.9552 times (two of them the same fixes with and without a header), and
  the 1 over the flyer's stated apogee 1.0025 times;
- the vertical speed's 1 check gives 0.9709;
- the 10 accelerometers read 0.9600 to 1.0531 g;
- the 6 clocks of a file beside overlap it by 0.998 to 1.000, and the recording with no flight
  steps at 1.000 times its logger's rate;
- the 13 pad pressures lie in their band, the 6 tracks lie past what radians can, and the 1
  cut of a file is the whole file's rows, value for value.

A test for each check passes an invented flight read in its right unit and fails it read in a
wrong one: seconds as milliseconds, hectopascals as pascals, meters as feet, m/s² as g and g as
m/s², m/s as ft/s, and degrees as radians. Six files are labelled for M7.1c23, four of them
unread.
