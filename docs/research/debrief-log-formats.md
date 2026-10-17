# Nine logger families, and how their files are read (M7.1)

Re-checked 2026-10-17: 25 claims corrected.

**What this covers.** The format half of Debrief: which loggers write what, how each file is told
apart, and the traps that only show up on real files, for whoever maintains
[M7.1's log readers](../roadmap-done.md). **How far to trust it:** the figures are Debrief's own,
on a private corpus of 61 logs, unless hpr's readers have since measured more, when hpr's figure
is given. Readings off a parsed flight: [the readings note](debrief-flight-readings.md).

Debrief is `nrdptel/fusionspace-debrief`, the maintainer's own MIT-licensed browser flight-log
analyzer, now sunset (ADR-046), read at commit `9e72db8` (mirror `refs/fusionspace-debrief`, from
`cargo xtask refs fetch`); `:463-469` continues the file named just before it. **Debrief's claims
are leads**: hpr's readers (`crates/hpr-flightdata/`, cited by file name) re-measure them, and
where the two disagree, hpr's reading is the one to use.

## How a file finds its parser

Each parser answers `detect(input) -> 0..1` and the highest score wins, ties going to the earlier
entry in the list (`lib/parsers/index.ts:32-55`). Below 0.6 nothing is claimed and a generic
column-mapping path takes over. Two decisions worth keeping:

- **Detection reads bytes and text both** (`lib/parsers/types.ts:8-23`). A binary flight record
  (an RRC3 `.rff`) cannot be recognized from a decoded string at all, and its text view is
  mojibake. (An AltOS `.eeprom` is ASCII, a JSON head and hex digits, and is detected from its
  text, `altosEeprom.ts:272-280`.)
- **A refusal is a result.** A parser may throw a *guidance* error that reaches the user as a
  sentence (`types.ts:44-56`): this is a design file and not a flight, or a raw card dump with the
  vendor tool that exports it named. Any other failure falls through to column mapping. The
  earlier behavior, telling someone their flight log is not a flight log, sent them after the
  wrong problem (`lib/parsers/rawDownload.ts:1-12`).

Text decoding handles a UTF-8 BOM, UTF-16LE and UTF-16BE by BOM, **and** BOM-less UTF-16 by NUL
density, because one vendor's text export and Excel's "Unicode Text" are both UTF-16: over the
first 4,096 bytes, at least a fifth must be NUL, and more than eight times as many on one side
(odd offsets for LE, even for BE) as on the other (`lib/encoding.ts:27-61`). Delimited text tries
comma, tab, semicolon and pipe, scoring each by column count and how consistent it is over the
first 50 lines; when semicolon wins, cells like `1,5` are decimal commas and are rewritten
(`lib/csv.ts:11-47, 108-118`).

## The families

Nine device families with a parser of their own (Debrief's README says ten and lists eight, without
Mercury), some writing two kinds of file, plus two inputs that are not loggers: a spreadsheet and an `.ork` design.

| family | file | told apart by | units in the file |
|---|---|---|---|
| Altus Metrum AltOS | CSV | header has `state_name`, `height`, `pressure` | SI; one sample file's pressure in hPa |
| Altus Metrum AltOS | `.eeprom` | JSON head with `log_format`, then hex bytes | raw counts |
| Blue Raven | app CSV, serial | `flight_time` + inertial or quaternion column; `LOG_LOW` | ft, °F, atm |
| Featherweight Raven | FIP CSV | contains `bILBA` and `time@` | atm, ft/s, g, °F |
| Featherweight GPS | tracker, base CSV | `unixtime`+`lat`+`lon`+`#sats`; or `tracker lat`+`gs lat` | ft, usually |
| PerfectFlite | `.pf2`, CSV | extension, or a vendor word plus data-shaped rows | feet, ft/s, °F |
| Missile Works RRC3 | mDACS text | header naming pressure, events and voltages | ft or m; 4 speed units |
| Missile Works RRC3 | `.rff` | .NET serialization header | tenths of a millibar |
| Eggtimer | CSV | `t`/`alt` plus `vraw`+`vfilt`, or `veloc` plus event columns | feet |
| Entacore AIM | CSV | `pressure msl` and `pressure agl` | Pa, g, °C |
| Mercury / AltimeterCloud | CSV | `acceleration_total(mg)`, `tilt`, `time(ms)` | mg, ms, centi-°C |
| spreadsheet | `.xlsx` | ZIP magic plus extension | whatever the sheet says |
| OpenRocket | `.ork` | ZIP holding `rocket.ork` | SI (mostly inferred) |

Debrief says the RRC3 text export is UTF-16LE (`lib/encoding.ts:3`; none of hpr's samples is,
ADR-253); it is detected by its header (`missileworksRrc3.ts:22-30`). Its manual (v1.71, p. 12)
gives speed in ft/s, mph, km/h or m/s.

**Units are frequently not in the file.** The RRC3 writes an identical header whether the device
is set to feet or meters, and the two readings differ by 3.28× in whichever direction the guess
is wrong: read a file written in feet as meters and every height reads **3.28× high**, which is
the direction that flatters the flyer; assume feet on a metric file, as Debrief's RRC3 parser does
when pressure can't settle it (`missileworksRrc3.ts:102-104`), and it reads **3.28× low**. Debrief
treats the Eggtimer the same way, but no Eggtimer manual offers a unit other than feet (11
cached manuals and release notes for six products; `crates/hpr-flightdata/src/eggtimer.rs:52-54`).

**Do not inherit Debrief's note on this: it has the direction backwards.** Its Eggtimer parser
declares the column as feet (`eggtimer.ts:102`) and a declared unit is multiplied into meters by
its factor, 0.3048 (`lib/units.ts:22-33`), so a metric file would come out low, while the note it
shows the flyer says a metric-set device's figures read about 3.3× too high (`eggtimer.ts:114`);
the RRC3 parser's note says the same (`missileworksRrc3.ts:122`). Debrief's RRC3 parser
has the fix worth keeping: the ambiguity is **resolved from physics**, testing the stated altitude
against the apogee the pressure drop implies (`missileworksRrc3.ts:49-78`). hpr does the same
(ADR-253): the unit within 25% of the pressure's climb is taken; over 50 sample files feet came
within 0.05% and meters at least 227% off (`rrc3.rs:62-76`). The Eggtimer is read as feet, checked
by a coasting deceleration of 0.7 to 1.3 g (`eggtimer.rs:228-243`). Debrief's speed-unit guess reads
an mph export 32% low (ADR-253).

## The two binary formats, which are the expensive ones

**AltOS `.eeprom`** (`lib/parsers/altosEeprom.ts`) is a JSON header, then the flash as ASCII hex.
Records are fixed size, little-endian: a type byte, a checksum byte, a 16-bit tick at 100 Hz. The
Altus Metrum manual (v1.9.22.2, appendix F, table 11) gives 8 bytes for TeleMetrum v1, 16 for
TeleMetrum v2 and v3, TeleMini v3 and EasyMini, 32 for TeleMega and EasyMega. Debrief reads
`log_format` 1 at 8 bytes and 10, 15, 16, 19, 21, 22 at 32; hpr reads 1 at 8; 6, 7 and 14 at 16; 15, 16
and 22 at 32; refuses 10, 19 and 21; and also reads a serial console's text transcript
(`altos_eeprom.rs:18-31`). Both refuse other numbers rather than guess: a wrong record size makes
a plausible flight out of misaligned bytes (`altosEeprom.ts:291-298`). The MS5607 boards follow
the MS5607 datasheet's compensation, second-order branch below 20 °C included (TE MS5607-02BA03,
p. 9, which adds a third below −15 °C), every division floored; TeleMetrum v1 carries an
MP3H6115A, with its own conversion (`:44-48`). Four traps:

- **Ticks are 16 bits and wrap every 655.36 s.** Each tick is placed on the turn nearest the
  previous one; without that, one flight gained a spurious 655.36 s and reported 975 seconds
  (`:143-156`). File order is not time order, and not only at the wrap: the pre-boost samples are
  written after the flight record and GPS records carry earlier ticks, so hpr sorts by tick and
  ends the flight at a gap over 10 s (`altos_eeprom.rs:47-58`).
- **Erased flash is `0xFF`** and must be skipped, or the clock is dragged to 65535 (`altosEeprom.ts:163-165`).
- **A calibration sentinel of 2147483647** means "this board has none"; a plain finite check lets
  it through and yields pressures in the billions (`:417-419`).
- **The decoded pressures are checked against the pressure the file states for the pad**, within
  2%, which makes an unmeasured format number safe to attempt; on the corpus the two agree to 4 Pa
  (`:455-469`). Format 1 states none (`:20-21`); hpr holds the others to 1% (`altos_eeprom.rs:81-83`).

Accuracy claimed: all 6,820 MS5607 pressures bit-identical to the vendor's own export; TeleMetrum
v1 none bit-identical, worst disagreement 0.0035 Pa over 2,206 samples (`altosEeprom.ts:11-15`). The
cold branch and the MS5611 variant are exercised by no corpus flight; a second reading of the same
datasheet page backs them, which catches a transcription slip but not a misreading.

**Missile Works `.rff`** (`lib/parsers/missileworksRff.ts`) is a .NET `BinaryFormatter`
serialization stream holding a list of 16-bit integers. Four things to carry over:

- The array is found by scanning for the primitive-array record and its matching member reference,
  and **only the list's `size` slots are real**: the rest is spare capacity, and reading it
  appends a run of zero-pressure samples (`:91-95`).
- A word below `0x4000` is a barometer sample in tenths of a millibar; at or above it, an auxiliary
  word, written mostly in pairs once a second. hpr found three that stand alone in each sample file
  (`crates/hpr-flightdata/src/rrc3_rff.rs:14-22`).
- That threshold is physics, not a fitted constant: 0x4000 in those units is 1,638 mbar, half again
  the highest sea-level pressure on record (1,089.1 hPa, WMO; 1,083.8 below 750 m), and above the 1,200 mbar at which
  the RRC3 reports a sensor fault (manual p. 9). It is a **threshold and not a bit test**, because a
  word with bit 15 set and bit 14 clear would pass a bit test and read back as at least 3,277 mbar
  (`missileworksRff.ts:32-39`).
- There are no timestamps, so the clock is the manual's stated 20 Hz (p. 23), and it is tested:
  one auxiliary pair per second against 20 samples per second, within 10% plus 2, or the file is
  refused (`:189-200`). What the pair *means* is deliberately not decoded: the calibration is not
  in the file.

## Traps that generalize

- **A second altitude column is a second instrument, not a duplicate.** AltOS writes `altitude`
  twice; the later one is the GPS receiver's, found by position after `longitude` because the name
  collides (`altusmetrum.ts:169-181`).
- **Acceleration conventions differ and the file rarely says.** AltOS writes acceleration net of
  gravity, which reads about zero on the pad; taken as specific force it is a full g low, including
  in the thrust-to-weight quoted against the 5:1 rail rule. It was proved from the data: one file
  reads −0.98 m/s² in that column while its body-axis column reads 9.78 m/s² on the same sample
  (`altusmetrum.ts:89-94`).
- **Pre-lock GPS positions must be blanked**, or the ground track runs to the equator. Debrief's
  rule is that AltOS writes (0, 0) and holds the last fix before a lock (`altusmetrum.ts:220-222`);
  no sample hpr's AltOS reader has read has such a row (`altos.rs:70-73`). A satellite count of 0
  means the position beside it is held over, and a *tracked* count is not a count in the fix: one
  Featherweight ground-station file reads 16 to 19 tracked on rows whose own fix column says none
  (`featherweightGps.ts:109-115`).
- **An inertial altitude wraps and drifts.** Across four Blue Raven files the inertial channel
  ran to ±32,767 ft (a 16-bit wrap) on two, and integrated away to −151,147 ft on a third. Two
  bounds cut it: a single-sample step near 65,536 ft (above 0.9 of it), and a disagreement with the
  barometer larger than the whole flight. Neither is tuned, and neither may fire when the inertial
  channel is the only altitude there is (`blueraven.ts:216-283`). hpr keeps the channel whole with a
  note instead, and found wrap steps in 10 of 45 low-rate files (`blueraven.rs:126-149`).
- **The barometer reads the rocket descending through Mach 1** as the shock crosses the static
  port: one corpus flight reads 307 ft below its pad, which is exactly where the inertial channel
  is the good one (`blueraven.ts:160-164`).
- **Units can be read off the data when the header is bare.** Blue Raven's high-rate columns state
  no units, so they are settled by magnitude. Over 35 high-rate files hpr found gyros saturating at
  a common 2,292 to 2,294 in 21, just past the manual's ±2,000 deg/s, which is degrees per second
  (radians would be about 365 revolutions per second), and a pad acceleration magnitude of 0.990 to
  1.014 in 34, which is g (`blueraven.rs:71-76`). Debrief's first measurement, on four files, read
  2,291–2,294 and 0.9935–0.9947 (`blueraven.ts:434-480`).
- **Saturation is counted, not assumed from a value.** Four files saturate at four different
  numbers, so what is counted is samples sitting at exactly the channel's own maximum: a saturated
  axis repeats it 13 to 6,729 times, an unsaturated one once or twice, with nothing between
  (`blueraven.ts:494-523`). ("Saturated", not "railed", throughout these notes: in this project a
  rail is the launch rail.)
- **Windows are spans of clock, not counts of samples**: [the readings note](debrief-flight-readings.md).
- **Several time bases in one file** are normal (Raven FIP, Entacore AIM): each channel has its own
  time column, the densest becomes the master clock, and the rest are resampled onto it with ends
  clamped rather than extrapolated (`multiTimebase.ts:14-68`).
- **Re-importing an analyzer's own export is a silent corruption**, not an error: derived columns
  claim the roles the recorded ones should have. Measured across the corpus, 19 of 48 recordings
  shifted their peak acceleration, worst +41.4%, and 16 flipped the velocity's provenance. Debrief's
  own export is therefore recognized and refused (`canonical.ts:23-45`).

## What every parser produces

One canonical record, whose shape [the readings note](debrief-flight-readings.md) describes. What
belongs here is the channel vocabulary: 23 kinds, each with its unit fixed by the kind
(`lib/flight/types.ts:9-91`). Three of those distinctions were paid for: `rollAngle` is not
`rollRate`, since a ±180° column read as a rate reports a plausible 179.99 deg/s; `satellites`
means satellites *in the fix*, with a merely tracked count sent elsewhere; and a high-rate stream's
axis, rate and quaternion kinds exist so the analysis **cannot** read them as flight channels,
because it only ever sees that stream as an envelope. A device's own headline figures ride along as
declared values with their source named, never merged into the measurements (`types.ts:98-145`).

## What may be ported from it, and what may not

[The porting boundary](debrief-porting-boundary.md): which parsers cite which published sources,
why the `.ork` parser is clean room while `COMPETITION.md` is not, and which fixtures hpr may use.

## Open questions

- The 975 s and 4 Pa figures are Debrief's statements about its corpus, not recomputed here.
- What Debrief's "ten logger families" counts: its README lists eight; nine have a parser.
