# ADR-263: M7.1c9 in two: AltOS telemetry first; the rest to M7.1c10 (2026-10-14)

- **Status:** accepted; narrows M7.1c9 to a first increment and moves the rest, with M7.1c9's old entry and *done when* (the parent's), to M7.1c10; follows [ADR-262](0262-the-r-das-raw-text-before-the-rest.md)
- **Summary:** After M7.1c8, 155 sample logs were left to M7.1c9. Two of them are AltOS radio telemetry, the `.telem` files AltosUI saves of the packets a TeleDongle hears, and Altus Metrum publishes their layout. M7.1c9 now reads them from that document, with every unit it converts held to a second check, and the rest goes to M7.1c10. The document's pressure unit is contradicted by the files, which hold pascals, not tenths.

**Context.** `cargo xtask logs` on main after M7.1c8 (#532) leaves 155 files: AltOS's `.eeprom`
(14), generic CSV and text (91), spreadsheets (24), and smaller families. Most need a decoder
written from the files alone, or the column mapper. One family has a published specification:
*AltOS Telemetry* (Bdale Garbee and Keith Packard, v1.9.22.2, 2025, CC BY-SA 3.0) lays out the
TeleDongle's serial line (§4, p. 10) and sixteen 32-byte packet types (§2–3). The two sample files
are two flights of one university team's rocket, 18,870 and 12,131 lines, each `TELEM` and a
packet in hexadecimal; the older has the AltosUI export of the same flight's on-board log beside
it. Measured on them:

- The checksum is 0x5A plus every byte after the length byte through the link-quality byte. The
  document's table puts its byte positions one place early, and its formula as written fails its
  own example; the reading above passes the example and all 31,001 lines.
- 2,561 and 498 packets fail the radio's CRC (the link-quality byte's top bit clear), their bytes
  noise. The rest name one serial number each, and five packet types: configuration, GPS
  position, GPS satellites, and the TeleMetrum v2's sensor and calibration packets.
- The newer file's ticks fall back twice, in every packet type at once: the board was switched
  off and on again on the pad, and only the last power-on flies. GPS packets carry the earlier
  tick of their fix, behind the sensor packet sent before them.
- The document gives `pres` as "Pa * 10". Read so, both pads would be more than 15 km up; read
  as pascals they are at the height above sea level the export states, and the export's
  pressures, in pascals, equal the telemetry's at all 123 ticks where it holds a fresh sample.
  The document is wrong about this field.
- The document numbers no flight state. The AltosUI exports among the sample logs write the
  number beside the name, and in every real one 2 is `pad`, 3 `boost`, 4 `fast`, 5 `coast`, 6
  `drogue`, 7 `main`, 8 `landed` (Debrief's edited sample shifts them by one).

**Decision.**

1. **Two increments.** M7.1c9 reads AltOS telemetry (`hpr_flightdata::altos_telem`,
   `LogFormat::AltosTelemetry`). M7.1c10 keeps M7.1c9's old entry and *done when*, the
   parent's, whole.
2. **A line that isn't a packet refuses the file; a packet that failed the radio's CRC is
   counted.** The TeleDongle computes the checksum over its own line, so a mismatch, a length
   other than 34 or a line that isn't `TELEM` and hexadecimal is damage to the file. A failed CRC
   is the radio's, expected, and its bytes are left out. Two serial numbers in one file are
   refused, as two flights.
3. **One power-on a record.** A packet whose tick falls behind the last of its own type starts a
   power-on; the record keeps the one whose sensor packets leave the pad (a state from `boost` to
   `main`; `landed` alone, as after a silence, doesn't count), or the last if none does, counts
   the others' packets in a note, and refuses a file where two leave the pad. Each tick goes on
   the turn nearest the latest so far, so GPS packets fall behind, not a turn ahead. The clock is
   the board's tick in seconds from the first packet's turn, not from boost. A silence of 327.68
   to 655.36 s reads as a power-on, a longer one as a turn shorter, and a restart whose tick lands
   ahead of the last as a silence.
4. **Only the five types the samples hold are decoded.** The eleven others the document lays
   out are counted and not read: its pressure unit is wrong for the one type the files check, so
   an unchecked table is not trusted.
5. **Every unit converted has a second check; the rest is kept as written.** Pressure in pascals
   where the pad's pressure, the median of the pad-state packets, lies from 10 to 120 kPa read
   so, otherwise kept as written. Temperature in hundredths of a °C, checked against the export
   (within its 0.05 K print step). The board's own `acceleration` and `speed`, in sixteenths, and
   `height`, in meters above the pad, checked against each other (the median ratio of the climb
   to the speed after burnout, and of the speed's change to the acceleration in the climb, each
   from 0.8 to 1.25; a file with too few fast packets for a ratio fails apart) and the highest
   height against the export's. GPS latitude and longitude in
   ten-millionths of a degree and altitude in meters, checked against the export's fixes (0.01°,
   100 m); a fix without the valid bit is a gap. The converters' counts (`accel`, `v_batt`, the
   continuity senses, the calibration's), the dilutions, the GPS date, ground speed, climb rate
   and course are kept as written. The RSSI is a signed byte, in dBm as `RSSI / 2 − 74`.
6. **The survey checks telemetry against the export beside it.** Lined up by whole turns from
   the two `boost` events, every telemetry pressure at a tick the export holds must equal the
   export's, but where the export's rows there repeat the pressure of the row before (15 of the
   138 sensor packets at its ticks), counted apart; a differing file, or one whose estimates fall
   out of the band, fails `cargo xtask logs --check`. The report prints each file's packets
   heard, those failing the CRC and its power-ons.

**Consequences.** Both sample files read; the export check finds 123 pressures equal and 15 at
repeated rows, the temperatures within 0.050 K, the positions within 0.000 06° and the GPS
altitudes within 3 m over 39 values; the board's ratios lie from 0.94 to 1.02, the heights' at
0.996. The survey
labels the families not yet read M7.1c10, 153 files. The rest of the AltOS family, `.eeprom`, needs
its records decoded from the files and the export beside them, as the document doesn't lay them out.
