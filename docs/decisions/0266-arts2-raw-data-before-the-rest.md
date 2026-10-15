# ADR-266: M7.1c11 in two: ARTS2's raw data first; the rest to M7.1c12 (2026-10-14)

- **Status:** accepted; narrows M7.1c11 to a first increment and moves the rest, with M7.1c11's old entry and *done when* (the parent's), to M7.1c12; follows [ADR-265](0265-altos-raw-downloads-before-the-rest.md)
- **Summary:** After M7.1c10, 137 sample logs were left to M7.1c11. Six of them are an Ozark Aerospace ARTS2's raw data: three `.odf` files, as its software DataAnalyzer saves a flight, and three raw CSV exports of the same records, each flight with DataAnalyzer's interpreted export and an AltOS board's log beside it. M7.1c11 now reads them, the `.odf` in units from the calibration it carries, held to its sensors' data sheets and to a climb check, and to the files beside it; the rest goes to M7.1c12.

**Context.** `cargo xtask logs` on main after M7.1c10 (#536) leaves 137 files: generic CSV and
text (91), spreadsheets (24), and smaller families, among them ARTS2's `.odf` (3) and raw CSV
(3), which [ADR-259](0259-raw-downloads-against-their-makers-exports-first.md) put first among
the raw downloads left. The *ARTS2 User Manual* (V1.0.0, Ozark Aerospace, 2007) names the
sensors, an Analog Devices ADXL78 set for ±50 g and a Motorola MPXA4100A, read by a 10-bit
converter (Appendix A, p. 16), and says the board's in-flight baselines and the sums of 64
accelerometer readings "at +1g, −1g, and 0" taken when it was calibrated are downloaded with the
flight (pp. 16–17); it describes neither file. What the files show, with no other program's
source read:

- **The `.odf`** is a 13-byte name (`Ozark.odf`, length first), five 32-bit words, a 256-byte
  settings block, the record count, three equal memory sizes, 8-byte records and 124 bytes of the
  software's own settings; each file's length is exactly that. Each record is four 16-bit
  words, in the raw CSV's column order, and the three raw CSVs hold their `.odf`'s records word
  for word, 48,614 in all, 0.02 s apart.
- **The settings block** holds, at offset 10, the barometer's count on the pad; at 12, eight
  times the accelerometer's; and at 20, 22 and 24, three sums of 64 readings, the same in all
  three files, whose middle one lies midway between the others, the reading on the board's side.
  The first record is the two baselines, the accelerometer's cut to a whole count, not a sample.
- **The interpreted export's `Acc`** follows the accelerometer's count less the pad's baseline,
  the export zeroing its first row, at a g a 128th of the end sums' difference: in the survey,
  every record after the first agrees within 0.080 of a count. Its
  `pAlt` is a falling function of the barometer's count one record earlier, in all three files
  with no count meeting two values.
- **One count of the barometer** is 92 Pa, about 7.7 m near sea level and more at a higher
  site, so over a 1 s window one flight climbs less than two counts.
- **An AltOS board flew with each**: the ARTS2's pad pressure lies 0.42 to 1.08 kPa below the
  AltOS board's and its least pressure from 0.48 below to 0.11 kPa above, within the MPX4100A's
  error band, ±1.5 kPa from 20 to 105 kPa at 0 to 85 °C (*MPX4100A*, Rev 9, Freescale, 2009,
  p. 6).
- **Unknown:** where the file keeps the recording rate, which the owner sets (the manual,
  pp. 6–7); all the sample files are at one rate, and the settings block's word at offset 8, 2 in
  each, is the likeliest place. Unknown too: the deployment settings' layout, the
  meaning of the flags byte's bits beyond the barometer's two low bits it repeats, and the
  state codes, which step 0, 1, 2, 3, 5, 6, 7 through the flights in order.

**Decision.**

1. **Two increments.** M7.1c11 reads ARTS2's `.odf` (`LogFormat::Arts2Odf`) and raw CSV
   (`LogFormat::Arts2RawCsv`) in `hpr_flightdata::arts2_raw`. M7.1c12 keeps M7.1c11's old entry
   and *done when*, the parent's, whole, without the ARTS2's raw data.
2. **Layout checks.** An `.odf` is refused whose length isn't its header's, settings' and
   records' and the 124 bytes after; whose three memory sizes disagree or don't hold the records;
   whose record holds a count over 1,023, a flags byte or state over 255, or a flags byte whose
   bits 3 and 2 aren't the barometer count's low two; or whose first record isn't the settings
   block's baselines. The raw CSV is refused for the same record faults, a row of other than five
   cells, and a time not after the row before's.
3. **Units.** The `.odf`'s acceleration from its own calibration: a g is a 128th of the end
   sums' difference in counts, zero at the pad's baseline (gravity on the pad taken out, as the
   export does), positive toward the end sum the pad's reading lies near. It is refused unless the
   ADXL78's data sheet bears it out (*ADXL78*, Rev. B, Analog Devices, 2010, p. 3, AD22280): a g
   from 36.1 to 39.9 mV, the side reading within 150 mV of half the supply and between the ends,
   and the pad nearer an end than the side. Pressure by the MPX4100A's nominal transfer function,
   `V_out = V_S (0.010 59 P − 0.1518)` (Rev 9, p. 5), against the supply as the converter's full
   scale; the pad's within the sensor's rated 20 to 105 kPa (p. 3), else refused. The flags byte
   and the state are kept as written; no event is taken from the state. The raw CSV holds no
   calibration, so its counts are kept as written, its time DataAnalyzer's `MET`.
4. **The clock and the climb check.** The records are 50 a second, the rate DataAnalyzer gives
   all three, and a file whose settings word at offset 8 isn't their 2 is refused, failing closed
   on the rate's likeliest place. Each unit's second check, and the clock's, is the climb check of
   [ADR-265](0265-altos-raw-downloads-before-the-rest.md) over a 2 s window from the first record
   rather than 1 s, for the coarser barometer: the climb by the pressure over the climb by the
   acceleration from rest, from 0.6 to 5/3. It bounds the clock only loosely: the climb by the
   acceleration goes as the square of the time step, so a step `k` times the true one passes for
   `k` from `√(r / (5/3))` to `√(r / 0.6)`, `r` the flight's ratio on its true clock, 0.67 to 1.12
   and 0.83 to 1.38 at the sample files' least and greatest ratios; every record notes that its
   clock is assumed. Unlike the AltOS reader, which keeps a
   failing file's counts as written, this reader refuses it: a count kept as written still
   needs a clock, and nothing else in the file bears one out.
5. **The survey checks each `.odf` against what lies beside it**, in its folder: a raw CSV must
   hold its records word for word, 0.02 s apart; an interpreted export must agree in `Acc` within
   half a count on every record after the first, read zero in `Acc` and `pAlt` at the first row
   and at any row after the records, and order its `pAlt` as the pressure one record earlier,
   with no pressure meeting two values, zero at the pad's pressure and above, rising as the
   pressure falls below it, and no `pAlt` missing. A differing file fails `cargo xtask logs --check`. The report prints each file's
   climb ratio and, reported only, `pAlt`'s largest difference from the standard atmosphere's
   pressure altitude, and the pad's and least pressures less an AltOS board's beside it, as
   differences only.

**Consequences.** All 6 files read. The 3 raw CSVs hold their `.odf`'s 48,614 records word for
word; against the 3 interpreted exports, 48,611 accelerations agree within 0.080 of a count and
the exports' `pAlt` has no gap, no clash and no inversion. The climb
ratios are 0.754, 1.032 and 1.145. Two exports' `pAlt` lies within 3.3 m of the standard
atmosphere's pressure altitude from the `.odf`'s pressure; the third's departs by up to 151 m by
another formula, which the survey reports and doesn't check. The survey reads 933 of its 1,122
files and labels 133 files M7.1c12, 131 of them unread.

**Done when (M7.1c11):** every ARTS2 `.odf` and raw CSV in refs reads (`cargo xtask logs
--check` fails on one that doesn't); an `.odf` with a layout fault, a first record that isn't its
baselines, a calibration or pad pressure its sensors' data sheets don't bear out, or a climb
check that fails or can't be measured is refused, each by its own message (tests); the `.odf`'s
acceleration and pressure are read in SI from its calibration and the sensor's transfer
function, and the raw CSV's counts kept as written (tests); each `.odf` with a raw CSV beside it
holds the same records word for word, and with an interpreted export beside it agrees in `Acc`
within half a count on every record after the first and orders `pAlt` as its pressure one
record earlier (the survey's report; a differing file fails `--check`); and the docs say what
was learned from the files, what is left out and how far to trust it.
