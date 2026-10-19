# How far hobby accelerometers' scale factor errs at boost loads

Impulse read from a boost's specific force scales one to one with the accelerometer's scale factor, and freeing that
factor in a fit couples drag and impulse at r = −0.989 ([drag-impulse-identifiability](drag-impulse-identifiability.md)
§4). This note asks what scale error is left at 5–30 g on the parts hobby boards carry, with and without the ±1 g
calibration some makers and users do. It builds on [altimeter-sensor-noise](altimeter-sensor-noise.md) §4 (the
datasheet table) and [accel-bias-after-pad-zero](accel-bias-after-pad-zero.md) (offsets, and the log set reused
here). Sources are datasheets, makers' manuals and public flight logs measured here. **Inference** marks this note's
arithmetic. g = 9.80665 m/s². Read 2026-10-19.

**In short.** A two-point ±1 g calibration leaves about 1% (1σ) at boost loads; without one, the datasheet's
tolerance stays, ±0.5% to ±11% by part, and the H3LIS331DL and BMI088 state none.
- **Datasheets (§2).** Initial sensitivity tolerance: ADXL375 ±10% (mean ± 3σ, not production tested), ADXL377
  ±10.8%, KX134 ±8%, MMA6555 ±5% and ADXL78 ±5% over temperature, BMX160 ±4%, MPU-6000/9250 ±3%, ICM-42688-P ±0.5%.
  The H3LIS331DL is trimmed at ±1 g with no tolerance given. Tempcos of 0.002–0.03%/°C add 0.06–0.9% over 30 K.
- **Nonlinearity is small at 5–30 g if its shape is smooth**: 0.1% or less for a ±200 g part, 0.3–0.9% for the
  H3LIS331DL at ±100 g, up to 0.7% for a ±16 g IMU near its top. The datasheets bound only the worst deviation
  across full scale; spread over a 10 g reading, that bound is 4–20% for high-g parts (**Inference**, §2).
- **Altus Metrum's calibration is rounded to whole counts (§4).** In 37 exports from 14 units, the counts in a 2 g
  span are an integer every time (37, 39, 40 or 41). The applied scale therefore sits up to ±1.25–2.5% from the
  part's own (**Inference**). Two units on one flight whose spans differ by one count read 2.4% apart, consistent
  with that rounding.
- **Measured spread at boost loads (§4).** The high-g part against the same board's IMU at 3–15 g: unit means
  −3.2% to +2.4% (8 units, 25 logs), within one unit sd 0.9%. Two units on one flight, their integrated speeds at
  burnout: −2.4% to +1.4% apart (11 pairs, 7 flights, rms 1.3%), so about 0.9–1.1% per unit (**Inference**).
- **The supersonic Blue Raven gap builds in coast, not boost (§4).** On the one flight with an Altus unit aboard,
  the Blue Raven's boost speed is within 1.5–2.0% of it; the 42 m/s gap at apogee grows through the coast (0.1 g).
  That note's §4 is corrected here.

## 1. What a scale error does, and what a ±1 g calibration removes

- **The model.** A reading (1 + s)·f + b, zeroed on the pad, integrates to (1 + s)·v + b·t. So the speed, and the
  impulse a fit reads from it, carries s whole at burnout; at apogee, where v = 0, s drops out (§2 of the bias note).
- **Two-point calibration.** Readings at +1 g and −1 g fix offset and gain at 1 g. What it cannot remove: its own
  resolution (one count over the 2 g span), the gain's drift between the calibration's temperature and the flight's,
  the curve between 1 g and the boost load, and drift since the calibration (no datasheet read gives aging).
- **Two parts, one stream.** A board that switches from a low-range to a high-range part (Blue Raven at 32 g [F1]
  PDF p. 24) adds the two parts' gain mismatch to any boost that crosses the switch.

## 2. Datasheet terms per part

| Part, range | Sensitivity tolerance | Tempco | Nonlinearity (as stated) | Source |
|---|---|---|---|---|
| ADXL375, ±200 g | 18.4–22.6 LSB/g on 20.5 (±10%, mean ± 3σ) | ±0.02%/°C | ±0.25% of FS | [D1] T. 1 p. 3 |
| ADXL377, ±200 g | 5.8–7.2 mV/g on 6.5 (±10.8%), ratiometric | ±0.02%/°C | ±0.5% of FS to 180 g | [D6] T. 1 p. 3 |
| H3LIS331DL, ±100–400 g | not given; trimmed at ±1 g | ±0.01%/°C | 2% of FS in ±70 g (±100 g set) | [D2] T. 2 p. 4 |
| ADXL78, ±70 g | 25.65–28.35 mV/g over −40 to 105 °C (±5%) | in the band | 0.2% typ, 2% max | [D5] T. 1 p. 3 |
| MMA6555, ±105 g | ±4% at 25 °C; ±5% over −40 to 105 °C | in the band | ±1% of FSR | [D4] §2.4 p. 7 |
| KX134, ±32 or ±64 g | 942–1,106 counts/g at ±32 g (±8%) | 0.01%/°C | 0.6% of FS | [D7] T. 1 p. 5 |
| BMX160, ±16 g | 1,966–2,130 LSB/g at ±16 g (±4%) | ±0.03%/K | ±0.5% of FS (±8 g) | [D8] pp. 8–9 |
| MPU-6000; MPU-9250, ±16 g | ±3% | ±0.02; ±0.026%/°C | 0.5%; ±0.5% | [D9] p. 13; [D10] p. 9 |
| BMI088, ±24 g | not given | 0.002%/K | 0.5% of FS (±3 g) | [D3] T. 4 p. 9 |
| ICM-42688-P, ±16 g | ±0.5% (production tested) | ±0.005%/°C | ±0.1% (±2 g) | [D12] T. 2 p. 12 |

- **Not read.** The LSM6DSO32 sheet (the ±32 g part Blue Raven's range fits) failed to load twice on 2026-10-19,
  and the sensor-noise note dropped its row, so no tolerance for it is stated here. ST's and ADI's sites refuse plain
  downloads.
- **Nonlinearity as a gain error (Inference).** Stated as the largest deviation from a best-fit line over full scale
  (FS). A quadratic curve with that bound, anchored at ±1 g, errs 1.5·NL·a/FS in relative gain at load a; a cubic
  much less. A deviation of NL·FS at any load, spread over a reading a, is NL·FS/a.

**A range per part at 10–30 g (Inference).** Uncalibrated: tolerance, tempco over 30 K and the smooth curve.
Calibrated at ±1 g: tempco over a 30 K gap between calibration and flight, plus the smooth curve; add the
calibration's own resolution (§4). The last column is the any-shape bound, which no datasheet narrows.

| Part (load) | Uncalibrated | Calibrated at ±1 g | Any-shape curve, at 10 / 30 g |
|---|---|---|---:|
| ADXL375 | ±10.7% (σ about 3.5%) | 0.6–0.7% | 5.0 / 1.7% |
| ADXL377 | ±11.5% | 0.6–0.7% | 10 / 3.3% |
| H3LIS331DL (±100 g set) | not bounded | 0.6–1.2% | 20 / 6.7% |
| ADXL78 (max curve) | ±5% plus up to 1.3% | 0.4–1.3%, drift unstated | 14 / 4.7% |
| MMA6555 (FSR as 105.5 g) | ±5% plus up to 0.4% | 0.1–0.4%, drift unstated | 11 / 3.5% |
| KX134 (±64 g) | ±8.7% | 0.4–0.7% | 3.8 / 1.3% |
| MPU-6000/9250, BMX160 (5–15 g) | ±3.8–5.6% | 0.8–1.6% | 1.6 / 0.5% (at 5 / 15 g) |
| ICM-42688-P; BMI088 | ±0.7%; not bounded | 0.15%; 0.06%, curve unstated at 16–24 g | n/a |

- **Assumed:** the MMA6555's FSR is read as 105.5 g (as the 211 g span, its any-shape cells double); the BMX160's
  nonlinearity, stated at ±8 g, is applied at ±16 g (its curve term may double); the ICM-42688-P row leaves out its
  0.09–0.14% curve term.
- **Calibration resolution:** one count over the 2 g span, 2.4–2.7% for 37–41 counts (§4); averaged readings kept
  to a fraction of a count make it negligible.
- **ADC-read analog parts:** at 6.5 mV/g on a 12-bit, 3.3 V converter one count is 0.12 g, so a single-sample
  ±1 g calibration resolves 6% (**Inference**); averaging fixes it.

## 3. Which boards calibrate the scale

| Board | Scale calibration | Source |
|---|---|---|
| Altus TeleMetrum, TeleMega, EasyMega | Two-point, antenna up then down, by the user | [A1] p. 42 |
| Altus IMUs (MPU6000, MPU9250, BMX160, BMI088) | Not said whether the up/down step covers them | [A1] pp. 42, 78 |
| Featherweight Blue Raven, ±32 g parts | Six-position user calibration, advised within a week | [F1] pp. 5, 17, 30 |
| Blue Raven high-range ("400G") part | None documented | [F1] PDF p. 24 |
| Featherweight Raven4 | Four-position (90° steps on the short edge); advised outside 0.8–1.2 g | [F2] PDF p. 10 |
| MARSA54L | "Cal Accel" only teaches which way is up | [M1] p. 13 |

The CATS paper [C1] and the Entacore AIM manual [N1] describe no accelerometer scale calibration, and the Proton
guide links a calibration page it doesn't describe [E1]: such boards carry the datasheet tolerance unless firmware
trims it unseen. Which axes Featherweight's six positions rescale, and whether the high-range part is among them,
the manual doesn't say.

## 4. Measured in public logs

Methods and confounders are in [accel-scale-at-boost-logs](accel-scale-at-boost-logs.md) (the logs part); in brief:
- **Whole-count calibration (Altus Metrum).** In 37 exports from 14 units the 2 g span is 37, 39, 40 or 41 counts,
  an integer to 0.0022. The applied scale is off from the part's own by the rounding: up to ±1.25% if the span is
  rounded once, ±2.5% (sd 1.0%) if each endpoint is (**Inference**). Two units whose spans differ by one count read
  2.4% apart on one flight; two with equal spans, within 0.8% on two flights.
- **High-g part against the same board's IMU, 3–15 g.** 25 logs, 8 units: unit means −3.2% to +2.4%, within one unit
  sd 0.86%; the IMUs' own ±3% is inside the spread. The gains differ by under about 1% between 1 g and the boost.
- **Two units on one flight, axial only (no baro, GPS or tilt).** 11 pairs, 7 flights: −2.4% to +1.4% apart at
  burnout (rms 1.3%), about 0.9–1.1% per unit. Subsonic Blue Ravens read 0.0–1.6% below the TeleMega beside them.
- **Supersonic Blue Raven.** On the one flight with an Altus unit aboard, the Blue Raven reads 1.5% above it at
  burnout and the gap grows through the coast to 42 m/s at apogee, about 0.1 g; two Blue Ravens on another flight
  agree within 0.4%. The bias note's §4, which put the step in boost, now says so.
- **GPS-referenced scale** (baro scaled to GPS in descent): −2.3% to +8.5% over 17 logs, 4.5 points of scatter
  within one unit: too confounded for a prior.

## What the project can use

- **Datasheets** (§2): vendor copyright; cite, cache under `refs/` only. Cached for this topic:
  `ds/icm42688p-tdk.pdf` (a third-party mirror of TDK's DS-000347 Rev 1.6; TDK's own link is a download gate).
  ADXL375, ADXL377 and MMA655x were read in a browser, not cached; the rest are cached by altimeter-sensor-noise.
- **Logs:** the bias note's set and terms (MIT repositories may be cached and cited; unlicensed ones, statistics
  only). No row is quoted; units and pairs carry this note's letters and numbers only.
- **Scripts** [L2]: standard library, re-runnable with `run.sh`; outputs `*.out` and `*.json` beside them.

## What this suggests

1. **M7.3's accelerometer-scale prior:** normal, sd 1.0%, truncated at ±2.5%, for Altus Metrum and subsonic Blue
   Raven boosts (measured 0.9–1.1% per unit; rounding bounds Altus at ±2.5%). For a board with no documented scale
   calibration, the datasheet: ADXL375 sd 3.5%, KX134 uniform ±8%, MPU or BMX160 ±3–4%, ICM-42688-P ±0.7%; for the
   H3LIS331DL or BMI088 alone, ±10% until measured. Expected effect (**Inference**): the identifiability sketch gives
   impulse SD 2.2% with the scale free; if impulse tracks the scale one to one, a 1% prior gives
   1/√(1/2.2² + 1/1²) ≈ 0.9%. The sketch ran no prior case, so a fit should measure it.
2. **ADR-257's synthetic test:** draw the scale per flight from the same prior, and for an AltOS profile quantize it
   the way the export does: draw a true sensitivity S near the measured spans (mean 19.8 counts/g on the 12 units at
   39–41, below the ADXL375's nominal 20.5) and apply round(2·S)/2 per g, giving whole-count spans 2.4–2.7% apart.
   A supersonic Blue Raven profile could add a coast-only error (about −3% on deceleration, or 0.1 g; logs part §4)
   to test whether the fit pins it on drag.
3. **The impulse-scale interval from a log:** carry ±1% (1σ) for a calibrated board's boost; widen to §2's range
   for an uncalibrated one. On supersonic Blue Raven logs, don't take drag from accelerometer coast without a flag.

## Open questions

- Does AltOS round each calibration endpoint or the span (±2.5% against ±1.25%)? A published configuration listing
  `accel_plus_g` and `accel_minus_g` beside a raw log would show without the source code.
- How much of the high-g/IMU spread is the IMU? An Altus log with a user calibration date, or a bench six-position
  test of one board, would split the IMU's ±3% from the high-g rounding.
- What makes supersonic Blue Raven coasts integrate about 0.1 g less deceleration? A subsonic and a supersonic
  flight of one rocket with an Altus unit aboard, or a shaker test, would separate gain from vibration rectification.
- Calibration temperature, and the high-g gain's drift over months, are logged nowhere read; the tempco term
  (0.6% at 30 K) is unmeasured. No public log has radar or optical tracking of boost speed.
- The LSM6DSO32 and BMI088 state no usable scale tolerance in what was read; Featherweight names no parts.

## Sources

All read 2026-10-19; makers' sites as in [altimeter-sensor-noise](altimeter-sensor-noise.md).
- [D1] Analog Devices, ADXL375 Rev. B, Table 1 p. 3 (and its notes 1–2). Read in a browser 2026-10-19.
  <https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL375.pdf>
- [D2] ST, H3LIS331DL DS9012 Rev 5, Table 2 p. 4; §3.1 p. 9; §4.3 p. 10.
  <https://www.st.com/resource/en/datasheet/h3lis331dl.pdf>
- [D3] Bosch Sensortec, BMI088 BST-BMI088-DS000-19 rev 1.9, Table 4 p. 9. <https://www.bosch-sensortec.com/>
- [D4] NXP, MMA655x Rev. 7.0, §2.4 rows 53–57 and 75, PDF p. 7. Read in a browser 2026-10-19.
  <https://www.nxp.com/docs/en/data-sheet/MMA655x.pdf>
- [D5] Analog Devices, ADXL78 Rev. B, Table 1 p. 3 (cached in `refs/papers/`).
  <https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL78.pdf>
- [D6] Analog Devices, ADXL377 Rev. B, Table 1 and notes p. 3. Read in a browser 2026-10-19.
  <https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL377.pdf>
- [D7] Kionix, KX134-1211 Rev. 4.0, Table 1 p. 5. <https://fscdn.rohm.com/kionix/en/datasheet/kx134-1211-e.pdf>
- [D8] Bosch Sensortec, BMX160 BST-BMX160-DS000-11 rev 1.1, pp. 8–9.
  <https://cdn.sparkfun.com/assets/2/2/8/7/d/BMX160_datasheet.pdf>
- [D9] InvenSense, MPU-6000/6050 Rev 3.2, §6.2 p. 13.
  <https://alliance.seas.upenn.edu/~medesign/wiki/uploads/Guides/MPU6050_datasheet.pdf>
- [D10] InvenSense, MPU-9250 Rev 1.0, §3.2 p. 9.
  <https://cdn.sparkfun.com/assets/learn_tutorials/5/5/0/MPU9250REV1.0.pdf>
- [D12] TDK InvenSense, ICM-42688-P DS-000347 Rev 1.6, Table 2 and notes p. 12, from a third-party mirror.
  <https://pcbsync.com/wp-content/uploads/2026/07/ICM-42688-P-Datasheet-TDK.pdf>
- [A1] Altus Metrum, *AltOS: The Altus Metrum System* v1.9.22.2 (2025), CC BY-SA 3.0, printed pp. 42 (§12.6.20),
  78 (Table 12). <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [F1] Featherweight, *Blue Raven User's Guide* (May 12, 2026), PDF pp. 5, 17, 24, 30; [F2] Raven4 manual (2020),
  PDF p. 10. <https://www.featherweightaltimeters.com/>
- [M1] MARSA54L manual v1.4, p. 13; [N1] Entacore AIM XTRA/BASE manual 4.x.x; [E1] Eggtimer Proton guide 2.01C.
- [C1] J. Binz et al., "CATS", ESA PAC Symposium 2024, CC BY 4.0. <https://arxiv.org/abs/2511.04725>
- [L1]–[L3] Logs, scripts and outputs: see [accel-scale-at-boost-logs](accel-scale-at-boost-logs.md).
