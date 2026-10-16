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
