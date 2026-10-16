# How each hobby altimeter turns pressure into height

This note asks, for each common hobby altimeter, what formula and reference its maker's documents give for turning
pressure into height: the standard atmosphere from a ground-zeroed pressure, a temperature-corrected form, the
onboard temperature, a fixed sea-level pressure. Where the documents are silent it says so, and it works out what
each choice does to a reading on a day 20 K off standard. It serves the real-flight comparisons, which read HPR
Sim's apogee "the way each log's altimeter reads the air" ([accuracy](../accuracy.md#real-flights)), and the flight
log importers. It builds on [dual-altimeter spread](dual-altimeter-spread.md) (§3's pad-as-sea-level form; height
columns 21% and 5.8% off their own pressure) and [AerospaceNU's flights](aerospacenu-flights.md). Sources are
makers' manuals, FAQs and a maker's forum post, the 1976 standard atmosphere, and checks of published and private
logs (aggregates only). **Inference** marks this note's reasoning and arithmetic. Read 2026-10-16.

**In short.** Every maker that names a model names the standard atmosphere; none documents an onboard temperature
correction; every family zeroes at the pad.
- **Two makers print or name the formula, and their files bear it out.** Missile Works prints NOAA's
  pressure-altitude formula, h = 145,366.45 ft × (1 − (P/1013.25 mb)^0.190284) [MW1] p. 23; an RRC3's exported
  height is that formula of its own pressure less the pad's, to 0.007 ft, on 49 of 51 flights. Featherweight's
  Raven names the International Standard Atmosphere [FW1] PDF p. 12; Raven and Blue Raven height columns are the ISA
  altitude of their own logged pressure to 0.22 ft and 4.5 ft (§1, §2).
- **Three more name the standard atmosphere without a formula.** PerfectFlite ("US Standard Atmosphere model"),
  Entacore ("Standard Atmospheric table data") and MARSA (its display); Altus Metrum, Jolly Logic, Eggtimer and the
  Blue Raven's guide name no model (§1).
- **No onboard temperature enters any height column checked.** Bay temperatures of 42–118 °F left the Raven and
  Blue Raven columns on the standard curve. The temperature corrections documented are by hand, off the board:
  Missile Works' mDACS "Geopotential" option [MW1] pp. 23–24, and the Raven manual's pointer to NOAA soundings (§1).
- **On a column 20 K warmer than standard, a pad-zeroed standard reading is 6.49% short of the climb; 20 K colder,
  7.46% long**: exactly T0/(T0 + ΔT) of the climb, at any pad height and climb (**Inference**, §3). A height above
  fixed sea-level pressure less a surveyed pad errs −15.6% to +17.9% on a 1,000 m climb from a 1,400 m pad.
- **Of the real-flight comparisons' seven logs, one more reading is now documented.** The NDRT 2020 Raven's formula
  is in its manual and its files; the four assumed logs are a CATS Vega's or teams' own filtered boards, which this
  note's families don't cover (§4).

## 1. What each maker's documents say

Pages are printed pages unless marked "PDF p.". Each row's cells are Sourced unless marked.

| Family | Model named | Reference | Temperature in the height | Raw pressure in the download |
|---|---|---|---|---|
| PerfectFlite | US Standard Atmosphere [PF1] p. 2 | pad, kept updated [PF1] p. 19 | silent; logged | no |
| Missile Works RRC3 | NOAA's formula [MW1] p. 23 | pad "floor", running average | no (PC option) | yes [MW2] p. 10 |
| Featherweight Raven | ISA, three layers [FW1] PDF p. 12 | 101,325 Pa ASL, AGL by offset (§2) | silent; logged | yes |
| Featherweight Blue Raven | silent [FW2] | pad "ASL" from pressure, p. 33 | silent; logged | yes |
| Eggtimer (6 models) | silent [E1]–[E8] | pad "ASL" reading | silent | no |
| Altus Metrum | silent [A1] | pad, averaged `ground_pres` [A2] | silent; logged | yes [A1] p. 37 |
| Jolly Logic | silent [J1]–[J4] | where turned on [J4] | silent | no |
| Entacore AIM XTRA | standard tables [EN1] p. 22 | MSL from a typed or GPS ground | silent; logged | yes [EN1] p. 40 |
| MARSA | standard (display) [M1] p. 16 | stored ground reading [M1] p. 17 | not mentioned | no (Inference) |

- **PerfectFlite** (StratoLogger, StratoLoggerCF, Pnut, FireFly, APRA). Height "above launch point according to the
  US Standard Atmosphere model", from "the pressure differential" [PF1] p. 2; the others say the same. On the pad
  the unit keeps updating its ground pressure, in older units "to compensate for thermal and barometric drift"
  [PF6] p. 2. Telemetry sends the "launch elevation MSL" first, then heights above the pad [PF1] p. 41. Whether
  the height is a difference of two standard altitudes or treats the pad as sea level is silent; they differ by
  +0.68% at a 300 m pad and +2.78% at 1,200 m (dual-altimeter spread §3). **Inference:** sending an MSL figure
  makes the difference likelier. A `.pf2` download holds time, height, speed, temperature and voltage, no
  pressure, so no file can settle the form. The download page has no StratoLoggerX document (42 files, counted).
- **Missile Works** (RRC3, RRC2L). The RRC3 and mDACS use NOAA's "Pressure Altitude" formula above [MW1] p. 23;
  the RRC2L names the same method [MW3] p. 11; the RRC2+ manual names none [MW4]. On the pad the unit updates its
  "floor" altitude by a running average [MW1] p. 23. The RRC3 stores 20 Hz pressure and mDACS computes height from
  it [MW2] p. 2; the CSV export holds time, altitude and pressure [MW2] p. 10. mDACS also offers a "Geopotential"
  method taking the flyer's launch and apogee temperatures and launch humidity [MW2] p. 5, [MW1] pp. 23–24.
  **Inference (computed):** NOAA's constants read 0.04% below ISA's troposphere for the same pressures.
- **Featherweight.** The Raven uses the "International Standard Atmosphere (ISA) model", three layers to
  104,987 ft, in floating point; warm air can make heights "low by 10% or more", and NOAA's balloon soundings give
  a correction [FW1] PDF p. 12. Its FIP exports `Baro (Atm)`, `Temperature (F)` and heights above sea level and the
  pad. The Blue Raven's guide names no formula [FW2]; its pad "ASL" is "based on the barometric pressure" [FW2]
  p. 33; its USB log holds pressure, sensor temperature and an inertial altitude, no barometric height [FW2] p. 23.
  The designer's development thread gives no formula [FW3]. Neither manual has a ground temperature entry.
- **Eggtimer** (Classic, Quantum, Proton, Apogee, TRS, Quasar). No model, formula or sea-level reference in any of
  eight documents [E1]–[E8], and no maker's forum post found. Each reports an "ASL" reading of the site, e.g.
  [E1] p. 36, [E3] p. 43; the Classic averages five baseline readings, then one every 15 s on the pad [E1] p. 19.
  The launch temperature "may appear to be significantly hotter" than the air [E3] p. 43. The Quantum 1.05 fix to
  the "temperature compensation route for the baro sensor" [E7] is, by **Inference**, the sensor's own pressure
  correction, not a height correction. The CSV's `Alt` is "the RAW pressure-derived altitude", with a filtered
  `FAlt` [E2] p. 38; no pressure is saved.
- **Altus Metrum** (TeleMetrum, TeleMega, EasyMega, EasyMini; documents only). Model and formula silent [A1], [A2].
  Main deployment is "above the recorded pad altitude" [A1] p. 39; telemetry carries `ground_pres`, an average of
  ground readings [A2] pp. 2–3. The CSV gives "pressure, altitude and height above pad" [A1] p. 37 (**Inference**:
  `altitude` from fixed sea-level pressure, `height` less the pad's). The flight state fuses barometer and
  accelerometer [A1] p. 66; whether CSV `height` is that state is silent. One TeleMetrum log's height column is the
  standard altitude of its pressure less the first row's ([report](../../validation/reports/real-flights.md)).
- **Jolly Logic** (AltimeterOne, Two, Chute Release). Formula, model and temperature silent [J1]–[J4]; all products
  report height "above where they were turned on, not altitude above sea level" [J4]; the Chute Release measures
  "relative to the launch pad" [J5] p. 9. The AltimeterThree's guide and export columns were not reached.
- **Entacore** (AIM XTRA/BASE; AIM USB 4). The XTRA "uses Standard Atmospheric table data" [EN1] p. 22; the USB 4
  manual is silent [EN2]. MSL is set daily by the PC software's "Calibrate pressure altitude" to a typed or GPS
  ground altitude [EN1] p. 22. The export holds `pressure`, `Pressure MSL`, `Pressure AGL`, `temperature` and
  `GPS MSL` [EN1] p. 40; a Kalman filter fuses GPS, barometer and accelerometers [EN1] p. 3, and which column, if
  any, holds its track is silent.
- **MARSA** (MARSA4, MARSA54L). The MARSA4's diagnostic shows altitude "according to the standard atmospheric
  model" [M1] p. 16; the ground reading is stored as `Baro0` [M1] p. 17. The MARSA54L reports three peaks: `AltB`
  from the barometer, `AltA` from integrated acceleration and `AltK` from a Kalman filter [M2] p. 15. Temperature
  is not mentioned in either manual.

## 2. Checked against the makers' own files

Where a family logs pressure, its height column can be recomputed from it (**Inference**).
- **RRC3.** 91 mDACS exports in the private collection, 51 distinct flights. On 49 flights the altitude column is
  h(P) − h(P_pad) through NOAA's constants to 0.007 ft; ISA's constants would miss by about 1 ft on a 3,000 ft
  flight, so the check tells them apart. The other 2 flights carry a step of about 40 ft in the subtracted pad
  reading, same formula. 22 flights logged temperature swings over 5 °F; none moved a height.
- **Raven.** 37 public FIP files from a college course archive [D1], flights to 13,000 ft: the ASL column is ISA's
  altitude of the logged pressure (101,325 Pa sea level) to 0.22 ft, where NOAA's constants miss by 1.0–6.9 ft
  (median 2.0 ft); AGL is ASL less a constant (spread ≤ 0.10 ft). The private collection's Raven files agree.
- **Blue Raven.** In a published example log [D2] the ASL column is ISA of the logged pressure to 1.8 ft, with the
  bay at 104–108 °F; on 45 app CSVs in the private collection, to 4.5 ft (median per-file maximum 1.4 ft, about the
  logged pressure's rounding). AGL is ASL less a constant.

## 3. What each choice does on a day 20 K off standard (Inference)

The column: sea-level 101,325 Pa, lapse −6.5 K/km′, every level ΔT off standard; dry, hydrostatic. Pressure from
eq. (33a) [U] p. 12, the layer from Table 4 [U] p. 3, geopotential from eq. (18) [U] p. 8. Cells: the misreading
of the true climb, m (% of it).

| Pad, climb | ΔT | a: standard, pad-zeroed | b: 101,325 Pa less surveyed pad | c2: pad T, isothermal | d: 8,434.5 m |
|---|--:|--:|--:|--:|--:|
| 0 m, 1,000 m | −20 K | +74.6 (+7.46%) | +74.6 (+7.46%) | +12.3 (+1.23%) | +87.8 (+8.78%) |
| 0 m, 1,000 m | +20 K | −64.9 (−6.49%) | −64.9 (−6.49%) | +10.7 (+1.07%) | −54.9 (−5.49%) |
| 0 m, 3,000 m | +20 K | −194.8 (−6.49%) | −194.8 (−6.49%) | +99.1 (+3.30%) | −102.1 (−3.40%) |
| 1,400 m, 1,000 m | −20 K | +74.6 (+7.46%) | +179.1 (+17.91%) | +12.8 (+1.28%) | +126.5 (+12.65%) |
| 1,400 m, 1,000 m | +20 K | −64.9 (−6.49%) | −155.8 (−15.58%) | +11.0 (+1.10%) | −25.8 (−2.58%) |
| 1,400 m, 3,000 m | +20 K | −194.9 (−6.50%) | −285.8 (−9.53%) | +102.2 (+3.41%) | −10.9 (−0.36%) |

- **(a) Standard, pad-zeroed: what every documented family does.** Inverting eq. (33a) gives a standard height of
  exactly h·T0/(T0 + ΔT), so the climb reads −ΔT/(T0 + ΔT): 0.32–0.37% per kelvin. **On a hot day a standard
  altimeter under-reads the climb; on a cold one it over-reads.** This matches the project's worked example
  ([atmosphere](../physics/atmosphere.md#pressure-altitude-what-a-barometric-altimeter-reads)).
- **(b) Fixed sea-level pressure less a surveyed pad.** Adds the pad's own misreading and the day's weather:
  8.3 m per hPa at sea level, 9.6 m at 1,400 m, whatever the climb. Not subtracting the pad adds its elevation.
  Altus Metrum's `altitude`, Entacore's `Pressure MSL` and MARSA's display are this kind of number, not heights.
- **(c) Temperature-corrected.** With the measured pad temperature and standard lapse the error is zero in this
  column by construction: it is as good as the temperature given. The pad temperature held for the whole column
  (c2) reads 1.1–1.3% high at 1,000 m, 3.3–4.0% at 3,000 m; a board 15 K warmer than the air (illustrative, no
  source sizes it) 4.9–5.8% high. No maker documents (c) onboard. **(d)** A fixed scale height R*T0/(M0·g0) =
  8,434.5 m reads 1.1% and 3.5% high on a standard day from sea level; no maker documents it.

## 4. What this means for the project's readings

- **Real-flight comparisons.** Of the seven logs ([report](../../validation/reports/real-flights.md)), the RRC3
  (Juno III) and TeleMetrum (Prometheus) are checked against their own pressure; the NDRT 2020 Raven's ISA reading
  is now documented [FW1] and shown on 37 public files, matching the rule (a) the comparison uses. The four
  assumed logs are Bella Lui's own avionics, Cavour's CATS Vega, Genesis's (probably one too) and Lince's unnamed
  computer, all filtered estimates. None is one of these families, so this note cannot settle them.
- **The gaps in AerospaceNU's flights and dual-altimeter spread §1.** GPS's climb 5.4% above the EasyMini's fits
  rule (a) on a column 15.5 K warmer than standard; the club SillyGoose's 5.8% fits +16.7 K or a temperature-using
  formula; the club board's 21% would need +60 K, so no conversion explains it (a drifting filter is likelier,
  unsized). The MARSA's 16.9% is its `AltA`, integrated acceleration [M2] p. 15, not a pressure conversion.
- **Importers.** A height column above the pad (Altus `height`, Entacore `Pressure AGL`, Raven and Blue Raven
  `AGL`, RRC3 `Altitude`; Eggtimer `Alt` by assumption) is rule (a); a sea-level column (Altus `altitude`, Entacore
  `Pressure MSL`, Raven `ASL`) is rule (b) and must have the pad's subtracted before it compares with a climb.
  Filtered (`FAlt`, MARSA `AltK`, a Kalman state) and integrated (`AltA`) columns are not pressure conversions.

## What this suggests

- **Record each log's reading as sourced or assumed by family.** RRC3, Raven, Blue Raven and TeleMetrum are
  sourced by file; PerfectFlite, Entacore and MARSA by document; Eggtimer and Jolly Logic remain assumed standard.
- **Tag each imported height column's provenance:** pressure, standard height above the pad, standard height above
  sea level, filtered, or integrated (§4).
- **Where a log keeps pressure, read the pressure, not the height column.** One documented formula then covers
  RRC3, Raven, Blue Raven, Altus Metrum and Entacore, and the 2 RRC3 flights with a stepped pad reading show why.
- **Carry rule (a)'s 0.32–0.37% per kelvin** as the reading's sensitivity to an uncertain column temperature (§3).

## Open questions

- Whether PerfectFlite and Eggtimer take the difference of two standard altitudes or treat the pad as sea level:
  documents are silent and no file keeps pressure (+0.68% to +2.78% apart at 300–1,200 m pads).
- Whether Altus Metrum's CSV `height` is the filter state or the raw conversion; its pad-averaging window.
- Entacore's calibration (a height shift or a reference pressure); the AltimeterThree's export; a MARSA33L manual.
- The CATS Vega's documented conversion and filter, which would settle two of the four assumed logs.
- How far a board's temperature sits from the air's at the pad; humidity's share (moist air reads lower still).

## Sources

Makers' documents are their copyright with no license stated: cited, cached under the ignored `refs/` only. The
Altus Metrum documents are Creative Commons ShareAlike 3.0. All read 2026-10-16.

- [PF1] [PF6] PerfectFlite StratoLoggerCF (2014) and miniAlt/WD (2002) manuals; the StratoLogger, Pnut, APRA and
  FireFly manuals say the same. <http://www.perfectflite.com/Download.html>
- [MW1] Missile Works RRC3+/RRC3 manual v1.71; [MW2] mDACS manual v1.71; [MW3] RRC2L manual v1.02; [MW4] RRC2+
  manual rev. 1.03. <https://www.missileworks.com/downloads/>
- [FW1] Featherweight Raven manual (2014-05-20); [FW2] Blue Raven user's guide (2026-05-12).
  <https://www.featherweightaltimeters.com/>
- [FW3] Featherweight's designer, Blue Raven development thread, 2023-03-31.
  <https://www.rocketryforum.com/goto/post?id=2440706>
- [E1]–[E8] Eggtimer Rocketry: Classic 1.52 RevD, Quantum 1.09G, Proton 2.01C, Apogee, TRS 2.07, Quasar 1.02S guides,
  Quantum release notes 1.09G, Excel graphing guide. <https://eggtimerrocketry.com/home/support/>
- [A1] Altus Metrum manual v1.9.22.2. <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [A2] *AltOS Telemetry*. <https://altusmetrum.org/AltOS/doc/telemetry.pdf>
- [J1] [J2] Jolly Logic AltimeterOne and AltimeterTwo guides; [J3] AltimeterTwo 64 g guide; [J4] support FAQ;
  [J5] Chute Release guide. <https://jollylogic.com/support/>
- [EN1] Entacore AIM XTRA & BASE manual 4.x.x v7 (2025-02-02).
  <https://downloads.entacore.com/aimxtra/AIM%20XTRA%20&%20BASE%20Manual%204.x.x%20v7.pdf>
- [EN2] Entacore AIM USB 4 manual (2021-04-01), a university course's copy.
  <https://dept.aem.umn.edu/~./faculty/flaten/Rocketry_Remote_Lessons_Fall_2021/AIM_USB_4_Manual.pdf>
- [M1] MARSA4 manual v1.0 (2009), a cached copy (www.marsa4.com refused the connection); [M2] MARSA54L manual
  v1.4 (2014). <http://3.17.74.168/wp-content/uploads/2022/04/MARSA54L-Flight-Computer-User-V1.0.pdf>
- [U] *U.S. Standard Atmosphere, 1976*, NOAA/NASA/USAF, eqs. (18) p. 8, (33a) p. 12, Table 4 p. 3.
- [D1] DOFPro flight data, Harvey Mudd College. <https://dofpro.org/RCK/fltdata/FlightData.html>; Creative Commons
  with no license named: cited, aggregates only.
- [D2] rex-mcall/Shred-O-Vision at 619af886f4, `examples/mothman_avenged/BlRv_wvuer1_LR_06-17-2026_09_25_14.csv`, MIT.
