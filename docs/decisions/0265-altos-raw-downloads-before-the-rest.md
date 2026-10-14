# ADR-265: M7.1c10 in two: AltOS raw downloads first; the rest to M7.1c11 (2026-10-14)

- **Status:** accepted; narrows M7.1c10 to a first increment and moves the rest, with M7.1c10's old entry and *done when* (the parent's), to M7.1c11; follows [ADR-263](0263-altos-telemetry-before-the-rest.md)
- **Summary:** After M7.1c9, 153 sample logs were left to M7.1c10. Fourteen of them are AltOS's `.eeprom` files, the board's own log as AltosUI downloads it, and nine have AltosUI's export of the same flight beside them. M7.1c10 now reads them, their layout worked out from the files and held to those exports, every unit converted resting on a sensor's data sheet and a second check; the rest goes to M7.1c11.

**Context.** `cargo xtask logs` on main after M7.1c9 (#534) leaves 153 files: AltOS's `.eeprom`
(14), generic CSV and text (93), spreadsheets (24), and smaller families. Altus Metrum publishes no
layout for the `.eeprom` records; its manual gives only their sizes by board (*The Altus Metrum
System*, v1.9.22.2, Appendix F, Table 11, p. 77). The AltOS and AltosUI sources are GPL and were
not read, nor was any port of them. What the files show:

- Two forms. Seven files are JSON: the board's configuration as an object, then the memory in
  hexadecimal. Seven are an older text form: the configuration as the board prints it, then one
  record a line with the checksum left out; one ends mid-line. Every record is a type letter, a
  checksum, a 16-bit tick and the type's fields.
- Seven log formats: 1 (three files, TeleMetrum v1, 8-byte records; two of
  them from firmware that states no log format), 6 and 14 (EasyMini, 16 bytes), 7 (six files,
  TeleMetrum v2 and newer, 16 bytes), 15, 16 and 22 (TeleMega and EasyMega, 32 bytes).
- All 29,437 records of the JSON files sum, with 0x5A, to 0 modulo 256, the seed of the
  TeleDongle's line checksum (*AltOS Telemetry*, §4, p. 10); fewer than 5% of chunks cut at a
  wrong size do. Erased memory, every byte 0xFF, follows the last record.
- The records aren't in time order: the pad's samples come after the flight record, and GPS
  records carry their fix's tick. One file ends with 14 records of another session 332 s on, and
  two hold records written twice, byte for byte, 19 in all.
- The MS5607 boards' pressures and temperatures match the exports exactly by the data sheet's
  compensation with each division by a power of 2 a floor; truncating instead misses by 1 Pa at
  6 samples of one cold flight. The TeleMetrum v1's pressures match within half the export's
  0.01 Pa print step by the MP3H6115A's transfer function read at a full scale of 32,752, the
  CC1111 converter's 2,047 in the top bits of a 16-bit result.
- The exports' accelerations follow from the configuration's +1 g and −1 g calibration, zeroed at
  the flight record's pad reading in six of the seven exports that hold one and at the +1 g
  count in the seventh.

**Decision.**

1. **Two increments.** M7.1c10 reads AltOS's `.eeprom` (`hpr_flightdata::altos_eeprom`,
   `LogFormat::AltosEeprom`). M7.1c11 keeps M7.1c10's old entry and *done when*, the parent's,
   whole.
2. **Framing by the stated log format.** Records are 8, 16 or 32 bytes as the configuration's
   log format says; any other number is refused by number, as misaligned bytes read as a
   plausible flight. A file that states none is format 1 if its product is a TeleMetrum v1, as
   the manual's release notes for 1.4 say AltosUI does (§H.43.2, p. 128), and refused otherwise.
   A JSON record whose checksum fails refuses the file; so does a data record after erased
   memory, a malformed text line but a last one cut short (left out, noted), two flight records
   or none, and a repeated configuration that disagrees with the first. `2147483647` is not set.
3. **The clock.** The tick counts 100 a second (*AltOS Companion Port*, Table 1). Each record
   goes on the turn nearest the latest tick so far, and the records are sorted by tick, stably.
   A record more than 10 s from the latest tick ends the flight; it and the rest are counted.
   Exact repeats are dropped and counted. Time zero is the flight record's tick, boost, as in
   the exports.
4. **Units, each with a second check.** MS5607 pressure and temperature by the data sheet's
   compensation, second order below 20 °C, in integers with floor shifts (*MS5607-02BA03*,
   pp. 8–9); the pad's pressure must lie from 10 to 120 kPa and within 1% of the flight record's,
   or the file is refused. TeleMetrum v1 pressure by the MP3H6115A's transfer function (rev. 5.1,
   p. 5) at the CC1111's full scale of 32,752 (SWRS033H, pp. 143–144), the converter's reference
   taken as the sensor's supply; the pad check as above. Acceleration along the board's axis from
   the file's own calibration, `(ground − count) / ((minus − plus) / 2) × g`, zero at the flight
   record's pad reading. A Mega's pyro, igniter, apogee and main senses in volts by *AltOS
   Telemetry*'s dividers (p. 3). GPS positions in ten-millionths of a degree, altitude in meters
   (Table 15, pp. 6–7); a record without a valid fix is a gap. The rest is kept as written: the
   battery and other senses and the IMU's words, whose factors fit the exports but have no
   source; the TeleMetrum v1's chip temperature, which the CC1111's constants miss by up to
   0.10 K. Not read: the TeleMega v4's IMU, which its one sample never ran, and a Mega's IMU when
   the flight record's IMU fields are all zero. States 2 to 8 take the exports' names.
5. **The record.** No altitude: the primary track is empty, as for the other raw downloads, and
   every field is a channel. The flight record's pad readings, the calibration, the main
   deployment height and the apogee delay are stated figures.
6. **The survey checks each file against what lies beside it.** An AltosUI export in the same
   folder of the same serial number and flight, read by its column names, lined up by tick: at
   each of the file's samples the export's row at that hundredth must agree within half its print
   step in pressure, temperature, acceleration and a Mega's voltages; an export whose
   acceleration differs by one constant, the +1 g zero, is counted apart. The board's `.telem` of
   the same serial number, lined up by whole turns from the two `boost` events: every pressure at
   a tick both hold must be equal. A differing file fails `cargo xtask logs --check`. The report
   prints each file's form, log format, records, checksums, erased records, repeats, records
   after a silence and a cut last line.

**Consequences.** All 14 files read. Against the 9 exports, 38,908 pressures agree (the MS5607
boards' exactly, the TeleMetrum v1's within 0.005 Pa), 31,673 temperatures within 0.05 K, 33,095
accelerations within 0.005 m/s² (one file zeroed at +1 g, counted apart) and 9,968 of a Mega's
voltages within 0.005 V, each half the export's print step; against the 2 telemetry files, all
231 pressures at shared ticks are equal. Four files have neither beside them and are decoded by
the same rules, unchecked. The survey reads 927 of its 1,122 files and labels the families not
yet read M7.1c11, 139 files.
