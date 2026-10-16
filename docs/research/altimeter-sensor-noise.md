# Noise and error of the sensors in hobby altimeters

This note asks which pressure sensors and accelerometers the common hobby altimeters carry, what their datasheets
state for noise, accuracy, range and behavior past full scale, and what public logs measure at rest. It ends with a
noise model per family for M7.2's synthetic smoother test (a simulated flight plus a noise model). It builds on
[dual-altimeter-spread](dual-altimeter-spread.md) §3–4 and does not repeat its flights. Sources are makers' manuals,
datasheets, 24 public logs and a paper. **Inference** marks this note's arithmetic (1976 atmosphere). Read 2026-10-16.

**In short.** Datasheet white noise is small (tenths of a meter, hundredths of a g); a per-flight bias dominates.
- **Three makers name their pressure parts.** Missile Works and Altus Metrum use the TE MS5607 (Altus's first boards
  an MP3H6115); Eggtimer names a Bosch BMP085/180 and, for the TRS, a TE MS5637. Altus Metrum's accelerometers are
  the MMA6555 (102 g), then the ADXL375 (200 g). The other makers name no part (§1).
- **Measured rest noise is 0.16–1.14 m (1σ), above the datasheet's best rows.** Blue Raven 0.43–0.71 m at 50 Hz
  (one settling rest 1.13 m); StratoLoggerCF 0.16–0.25 m at 20 Hz; Altus Metrum 0.52–1.14 m at 100 Hz on short pad
  windows. The MS5607 sheet gives 0.20 m (2.4 Pa) at best. Blue Raven samples correlate (lag-1 +0.40 to +0.54) (§3).
- **Bias outweighs noise by an order of magnitude.** An MS5607 zeroed on the pad holds ±1.0 mbar at 0–50 °C: ±11 m
  at 3,000 m, 28 times its 0.40 m noise at oversampling 2048. The 0.4%-per-unit scatter in dual-altimeter-spread
  (12 m at 3,000 m) is bias-sized. An accelerometer bias b adds ½·b·t²: 0.49 m per mg over 10 s (§2, §6).
- **No datasheet read says what a pressure sensor outputs past its range.** A "clipped" baro reading can rest only on
  a maker's ceiling: 29,500 ft (about 300 hPa, the Bosch parts' edge) or 100,000 ft. Of the accelerometers, only the
  BMX160 and MMA6555 state a saturating output; Jolly Logic flags readings past 64 g with ">" (§5).

## 1. Which sensors the altimeters carry

| Altimeter | Pressure part | Accelerometer, range | Ceiling; baro rate | Source |
|---|---|---|---|---|
| PerfectFlite StratoLoggerCF, Pnut, APRA, FireFly | not published | none | 100,000 ft; 20 sps | [PF1] p. 49 |
| Missile Works RRC3, RRC2+, RRC2L | MS5607 | none | 100,000 ft; RRC3 OSR 2048, 20 Hz | [MW1] pp. 2, 24 |
| Featherweight Raven3 / Raven4 | not published | ±70 or ±250 g / ±105 g | 100,000 ft; 20 Hz | [F1] p. 4, [F2] p. 12 |
| Featherweight Blue Raven / 2 | not published | ±32 g IMU + "400G" / 320 g | not stated; 50 Hz | [F3], [F4] |
| Eggtimer Quark / Apogee, Quantum | not published | none | 29,500 ft / 60,000 ft | [E1], [E3], [E4] p. 2 |
| Eggtimer Classic | Bosch BMP085/180 | none | 29,500 ft | [E2] p. 2, PDF p. 54 |
| Eggtimer Proton | not published | 120 g or 200 g by board | 60,000 ft | [E5] p. 2 |
| Eggtimer TRS | BMP180 (rev A5); MS5637 (guide 2.07) | none | 65,000 ft | [E6] p. 9, [E7] PDF p. 48 |
| Altus Metrum | MS5607 (first: MP3H6115, 10 km) | MMA6555 102 g, then ADXL375 | 30 km; 100 Hz up | [AM] pp. 83–84 |
| Jolly Logic AltimeterOne / Two 64G | not published | none / 64 g, over-range ">" | 29,500 ft | [J1] p. 5, [J2] p. 7 |
| Entacore AIM XTRA | not published | 100 g axial, 16 g 3-axis | 100,000 ft | [EN] p. 6 |
| Silicdyne Fluctus | not published | ±200 g | 20 km | [SI] |

- **Altus Metrum** also lists MPU6000, MPU9250, BMX160 and BMI088 IMUs by board (Table 12), and older MMA2202 (50 g)
  and ADXL78 (70 g) axial parts; EasyMini and TeleMini carry none [AM] p. 84. Above 30 km the baro is dropped (p. 72).
- **Not identified (Inference).** Featherweight's ±32 g IMU and "400G" part fit the LSM6DSO32 and H3LIS331DL by
  range; the AltimeterOne's "19-bit, <0.25 m RMS" fits the BMP180's top mode [S7] pp. 9, 12. MARSA names none [MA].
- **Steps and accuracy.** PerfectFlite 1 ft, ±(0.1% + 1 ft) [PF1] p. 49; Raven 4.05 Pa, under 1 mbar [F2] p. 12;
  Blue Raven 2.03 Pa stored [F3] p. 23, 10.1 Pa in its CSV; MS5607 compensated output 1 Pa [S1] p. 8.

## 2. Pressure sensors: what the datasheets state

σ is RMS noise with the sensor's filter off, lowest to highest oversampling (TE calls it "Resolution RMS"). Accuracy
in hPa (= mbar = 100 Pa). The ceiling is the range's low edge in the 1976 atmosphere (**Inference**).

| Part | σ (Pa) by oversampling | Relative | Absolute | Pa/K | Range, hPa (ceiling) | Source |
|---|---|---|---|---|---|---|
| MS5607 | 13.0, 8.4, 5.4, 3.6, 2.4 | ±0.5–1.0 † | ±1.5–3.5 | none | 300–1100 (9,177 m) ‡ | [S1] pp. 3–4 |
| MS5611 | 6.5, 4.2, 2.7, 1.8, 1.2 | ±0.5–1.0 † | ±1.5–3.5 | none | 450–1100 (6,350 m) | [S2] p. 4 |
| BMP390 | 3.7, 2.7, 2.0, 1.6, 1.2, 0.9 | ±0.03 per 100 hPa | ±0.50 | ±0.62 | 300–1250 (9,177 m) | [S3] T. 2, 8 |
| BMP388 | 6.6, 4.3, 3.2, 2.3, 1.6, 1.2 | ±0.08 per 100 hPa | ±0.50 | ±0.75 | 300–1250 (9,177 m) | [S4] T. 2, 8 |
| BMP280 | 3.3, 2.6, 2.1, 1.6, 1.3 | ±0.12 | ±1.0 | ±1.5 | 300–1100 (9,177 m) | [S5] T. 2, 8 |
| BMP180 | 6, 5, 4, 3 | ±0.12 | −4.0 to +2.0 | none | 300–1100 (9,177 m) | [S7] pp. 6, 12 |
| DPS310 | 2.5, 1, 0.5, 0.4, 0.35, 0.3, 0.2 | ±0.06 per 1 hPa | ±1.0 | 0.5 | 300–1200 (9,177 m) | [S6] T. 3, 7, 16 |
| LPS22HB | 0.75 (filtered, one value) | ±0.1 | ±1 | none | 260–1260 (10,125 m) | [S8] Table 3 |

- † After a zero at one pressure (the pad zero): ±0.5 mbar at 25 °C and 700–1100 mbar, ±1.0 at 0–50 °C, ±2.5 at
  −20–85 °C [S1] p. 4. ‡ Its ADC stays linear over 10–1200 mbar [S1] p. 4; 10 mbar is 31,207 m (**Inference**).
- **Pascals to meters.** dh = −dp/(ρg): 0.083, 0.096, 0.112 and 0.219 m/Pa at 0, 1,500, 3,000 and 9,000 m
  (**Inference**), so σ is 0.02–1.1 m at sea level, 0.04–2.8 m at 9 km. MS5607 conversions take 0.54–8.22 ms [S1].
- **Temperature.** Tempcos are 0.04–0.125 m/K at sea level; the MS5607 states none. No source found measures a
  bay's temperature change in flight. Drift and reflow shift (up to +2 hPa) are offsets the pad zero removes.

## 3. Measured noise at rest in public logs

Each log's pad segment before liftoff, and its rest after landing where present; σ is the sample standard deviation
about a least-squares line; the step is the smallest gap between distinct values. Short pad windows mix in wind,
handling and ignition, so they are upper bounds on sensor noise. Logs are from public repositories [L1]–[L8].

| Device (logs) | Rate | Samples per window | σ altitude (m) | σ pressure (Pa) | Logged step |
|---|---|---|---|---|---|
| Blue Raven (5 flights) | 50 Hz | pad 70–73; rest 213–290 | 0.43–0.62; 0.51–1.13 | 5.5–8.5; 6.6–13 | 0.15–0.18 m |
| StratoLoggerCF (10) | 20 Hz | rest 92–161 | 0.16–0.25 | not logged | 1 ft (0.3048 m) |
| Altus TeleMetrum, TeleMega (5) | 100 Hz | pad 18–25 | 0.52–1.14 | 5.5–12.1 | 1 Pa |
| Altus TeleMega, after landing (1) | 10 Hz | rest 302 | 0.64 | 7.5 | 1 Pa |
| Team-built board, LPS-class (1) | 3 Hz | pad, 1 h (10,781) | 0.77 | 8.1 | about 1 Pa |
| Missile Works RRC3 (1) | 20 Hz | no pad in the log | none | none | 10 Pa (0.95 m) |

- **Not white.** Detrended Blue Raven altitude has lag-1 autocorrelation +0.40 to +0.54 in six windows, the TeleMega
  rest +0.36; two 18-sample Altus pad windows give −0.35 and −0.29. The 1 h record's Allan deviation falls from
  6.6 Pa at τ = 0.33 s to 1.25 Pa near 33 s, then rises to 3.0 Pa at 328 s: white noise below about 10 s, drift
  (weather, heat) above 30 s (**Inference**).
- **One published rest model** has the same shape: a BMP085 at 50 Hz, 10 min static, gives white noise of σ
  0.22–0.27 m plus a first-order Gauss–Markov term of σ 0.24–0.29 m with τ 0.65–1.26 s [P1] Tables 1–2. No
  third-party rest test of the MS5607, MS5611, DPS310 or LPS22 was found; a 3 s hobby test gives BMP390 2.5 Pa [P2].
- **Against the datasheet.** Altus Metrum's 5.5–12 Pa sits between the MS5607's 1024 and 256 oversampling rows; its
  setting isn't in the published docs. The RRC3 reads at oversampling 2048 (3.6 Pa) and averages 50 Hz down to
  20 Hz [MW1] p. 24: about 2.3 Pa if samples are independent (**Inference**).
- **Accelerometers at rest.** Blue Raven high-range axis (500 Hz, about 760 samples): σ 0.007–0.030 g, printed in
  0.01 g steps, mean 0.5–1.0% below 1 g. Altus Metrum logs 0.49 m/s² (TeleMega) and 0.53 m/s² (TeleMetrum) steps.
- **Two public dual-altimeter pairs** agree on apogee within 0.138% (TeleMetrum against TeleMega, 3,904 against
  3,898 m) [L5] and 0.085% (Blue Raven against TeleMega, 2,852 against 2,854 m) [L2]. Descent residuals about 2 s
  quadratic fits, 0.3–7.5 m robust σ, carry swinging and gusts: they bound flight noise, not sensor noise.

## 4. Accelerometers: what the datasheets state

| Part | Range; step | Noise density | Zero-g offset; drift | Scale; tempco | Source |
|---|---|---|---|---|---|
| ADXL375 | ±200 g (min ±180); 49 mg | 5 mg/√Hz | ±0.4 g typ, ±6 max; ±10 mg/°C | ±10%; ±0.02%/°C | [D1] p. 3 ¶ |
| H3LIS331DL | ±100/200/400 g; 49–195 mg | 15 mg/√Hz | ±1 g; ±5 mg/°C | not given; ±0.01%/°C | [D2] p. 4 |
| MMA6555 | ±105 g; 55 mg | ≤1 LSB RMS, 400 Hz | ±3.3 g; ±14 mg cancelled | ±5% | [D4] p. 7 |
| ADXL377 (analog) | ±200 g | 2.7 (x, y), 4.3 (z) mg/√Hz | ±15 g; z ±30 mg/°C | ±11%; ±0.02%/°C | [D6] p. 3 |
| MPU-6000 / MPU-9250 | ±2–16 g; 0.49 mg | 400 / 300 µg/√Hz | ±50–80 mg | ±3% | [D9] p. 13, [D10] p. 9 |
| BMX160 | ±2–16 g; 0.49 mg | 180 µg/√Hz | ±40 mg; ±1 mg/K | ±4% | [D8] p. 8 |
| BMI088 | ±3–24 g; 0.73 mg | 160–190 µg/√Hz | 20 mg; <0.2 mg/K | not given; 0.002%/K | [D3] p. 9 |

- **RMS from density.** σ = density × √(1.57 × bandwidth), single-pole (**Inference**): ADXL375 44 mg at 50 Hz,
  125 mg at 400 Hz; H3LIS331DL 133–376 mg at 50–400 Hz. Prefer a stated RMS: the MPU-9250's 8 mg at 94 Hz [D10] p. 9 is
  2.2 times the formula's 3.6 mg. The ADXL375's limits are mean ± 3σ, not production-tested [D1] p. 3.
- **The pad zero.** Blue Raven's manual shows a 0.93 g on-pad correction of its high-g axis [F3] p. 32; Raven4 asks
  for recalibration outside 0.8–1.2 g [F2] p. 10. No source states the bias left after the zero: thermal drift,
  scale error × 1 g and rail tilt (1.4 mg at 3°, 15 mg at 10°) remain.
- **Gyros.** Angle random walk of 0.3–0.84 °/√h [D3], [D8]–[D10] is 0.02–0.04° in 10 s; a 1° tilt costs 0.015%.

## 5. Ranges, ceilings and saturation

- **Pressure: unspecified past range** in seven datasheets and the MS5611 excerpt: no clamp, flag or wrap. Makers'
  ceilings cluster at 29,500 ft (about 308 hPa, the Bosch parts' 300 hPa edge: Eggtimer Quark and Classic, Jolly
  Logic) and 100,000 ft (about 11 hPa, in the MS5607's linear range: PerfectFlite, Missile Works, Raven4, Altus,
  Entacore) (**Inference**). The ideas list's
  9,100 m for BMP280/388/390 holds to 1%: 300 hPa is 9,177 m, the edge of the specified range, not a known clip.
- **Acceleration.** BMX160 data hold at the maximum code [D8] p. 32; the MMA6555's output window is ±1,920 LSB
  (±105.5 g), its converter clipping at 375–450 g [D4] p. 7; analog parts stop at their output swing (ADXL377 about
  −215 g and +200 g, **Inference** from [D6] p. 3). The BMI088 sheet says its ±24 g range is there "to avoid signal
  clipping" [D3] p. 2. The H3LIS331DL says nothing. Blue Raven switches from its ±32 g
  parts to the high-range part past 32 g [F3] p. 24.

## 6. A noise model per family

Baro: p_meas = Q(p_true + b + k_T·ΔT + n), with Q rounding to the logged step. Accel, per axis:
a_meas = Q(clip((1 + s)·a + b_a + n_a, ±FS_min)). b, s and b_a are drawn once per flight; n per sample.

| Family | n (1σ) | Per-flight bias | Other terms | Basis |
|---|---|---|---|---|
| MS5607 (Missile Works, Altus) | 2.4–13 Pa; RRC3 2.3 Pa | ±1.0 mbar after pad zero | 1 Pa; RRC3 log 10 Pa | [S1] |
| Bosch BMP3xx, DPS310 | 0.2–6.6 Pa by setting | ±0.03–0.08 hPa per 100 hPa | 0.5–0.75 Pa/K | [S3], [S4], [S6] |
| Measured, as a field model | 0.5 m at 50 Hz, AR(1) φ ≈ 0.5 | as above | 1–3 Pa drift, 30–300 s | §3, [P1] |
| High-g digital (ADXL375) | 44 mg at 50 Hz | ±10 mg/°C × ΔT | ±10% scale; clip ±180 g | [D1] |
| High-g (H3LIS331DL) | 133 mg at 50 Hz | ±5 mg/°C × ΔT | clip at the set range | [D2] |
| Low-g IMU axes | 1.6–8 mg | ±20–80 mg raw | clip ±16–32 g | [D3], [D8]–[D10] |

- **What each term does (Inference).** A ±1.0 mbar baro bias is ±8.3, 9.6, 11.2 and 21.9 m at 0, 1,500, 3,000 and
  9,000 m; Bosch's relative spec read as a gain over a 3,000 m climb is ±1.0 m (BMP390), ±2.8 m (BMP388). An
  accelerometer bias adds ½·b·t²: the ADXL375's typical 0.4 g raw offset is 196 m in 10 s, 100 mg of drift 49 m.
  A scale error s adds s·(h + ½·g·t²): 1% on a flight reaching 1,000 m in 10 s is 15 m. White noise adds
  N·t^{3/2}/√3, about 0.9 m for the ADXL375 at 10 s. A clipped boost loses ∫(a − FS) dt of speed and reads low.
- **Correlated noise.** At φ ≈ 0.5, white-noise math understates an average's σ by √((1+φ)/(1−φ)) ≈ 1.7.

## What this suggests

1. **M7.2's synthetic test should draw a per-flight bias, not white noise alone:** ±1.0 mbar for an MS5607-class
   baro, and a residual accelerometer bias no source states (a test range of ±10–50 mg is a guess).
2. **Use correlated baro noise** (white plus AR(1), σ about 0.5 m at 50 Hz, φ about 0.5), rounded to each device's
   logged step. White-noise tests flatter a smoother's 90% intervals by about 1.7 times.
3. **"Clipped" provenance:** clip acceleration at the minimum stated range (ADXL375 ±180 g, MMA6555 ±105.5 g, else
   the maker's range); mark pressure "beyond the maker's ceiling" rather than "clipped".
4. **Treat dual-altimeter-spread's 0.4% per unit as a bias term;** the two public pairs here (0.085%, 0.138%) fit.

## Open questions

- Which oversampling, rate and filter do Altus Metrum, Featherweight, Eggtimer and PerfectFlite use? Their docs
  don't say; Altus Metrum's is in GPL firmware this project doesn't read.
- Which parts do PerfectFlite, Featherweight, Jolly Logic, Entacore, MARSA and Silicdyne fit? A chip-marking photo
  would settle each. How far does bay temperature move in flight? What accelerometer bias survives a pad zero?
- No public log has over 2 s of pad data at 50 Hz or more; no Eggtimer, Jolly Logic or EasyMini log was found.
- ¶ The ADXL375 row (and its ±180 g clip) was read once and could not be re-read against the sheet; the maker's
  product page confirms only ±200 g. An LSM6DSO32 row was dropped for the same reason.

## Sources

Datasheets and manuals are cited and cached privately, not copied; the Altus Metrum manual is CC BY-SA 3.0. Logs
[L3], [L4], [L6] carry no license: statistics only.
- [S1] TE MS5607-02BA03, pp. 1, 3–4, 8. <https://pages.hmc.edu/spjut/AdvRoc/PDF/MS5607-02BA03DataSheet.pdf>
- [S2] TE MS5611-01BA03 06/2017, distributor excerpt; [S6] Infineon DPS310 v1.2, from Infineon's knowledge base.
- [S3]–[S5], [S7] Bosch BMP390 DS002-07, BMP388 DS001-07, BMP280 DS001-26, BMP180 DS000-09;
  [D3] BMI088 rev 1.9; [D8] BMX160 rev 1.1. <https://www.bosch-sensortec.com/>
- [S8] ST LPS22HB rev 5; [D2] H3LIS331DL Rev 5. <https://www.st.com/resource/en/datasheet/>
- [D1] ADXL375 Rev. B; [D6] ADXL377 Rev. 0. <https://www.analog.com/media/en/technical-documentation/data-sheets/>
- [D4] NXP MMA655x Rev. 7.0 <https://www.nxp.com/docs/en/data-sheet/MMA655x.pdf>; [D9] InvenSense MPU-6000
  Rev 3.2; [D10] MPU-9250 Rev 1.0 <https://cdn.sparkfun.com/assets/learn_tutorials/5/5/0/MPU9250REV1.0.pdf>
- [AM] Altus Metrum manual v1.9.22, PDF pp. 72, 83–84. <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [PF1] PerfectFlite StratoLoggerCF manual. <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
- [MW1] RRC3 manual v1.71; [MW2] RRC2+ rev 1.03; [MW3] RRC2L. <https://www.missileworks.com/downloads/>
- [F1] Raven 2014; [F2] Raven4 2020; [F3] Blue Raven 2026 manuals; [F4] Blue Raven 2 page.
  <https://www.featherweightaltimeters.com/>
- [E1]–[E7] Eggtimer Quark, Classic, Apogee, Quantum, Proton guides; TRS manual A5, guide 2.07.
  <https://eggtimerrocketry.com/>; [J1] AltimeterOne v2.0, [J2] AltimeterTwo 64G guides <https://jollylogic.com/>
- [EN] Entacore AIM XTRA/BASE manual 4.x.x; [MA] MARSA54L manual v1.4; [SI] <https://www.silicdyne.net/fluctus/>
- [P1] Sabatini and Genovese, Sensors 13(11):15692 (2013), CC BY. <https://doi.org/10.3390/s131115692>;
  [P2] madflight, "Sensor Boards". <https://madflight.com/Sensor-Boards/>
- Logs on GitHub (repository@commit): [L1] rex-mcall/Shred-O-Vision@619af886f4, MIT; [L2] werocketry/Hyperion-II
  @efda132ff4, MIT; [L3] VoicefulEmu/DataAnalysisSnowbirdTest@cb6b8157a1; [L4] icl-rocketry/art-data@bee1c863eb;
  [L5] RocketPy-Team/RocketPy@9bd6ad3af8, MIT; [L6] ISSUIUC/flight-data@b8c95fa993; [L7] gbertoldo/rRocket-UI
  @a625a96296, MIT; [L8] UND-ARC/IREC-ARC-2024@94a991bfc6, MIT.
