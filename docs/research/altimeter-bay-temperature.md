# How far an altimeter's own temperature moves between pad and apogee

This note asks how far a barometric altimeter's sensor temperature moves from the pad to apogee in hobby flights,
how fast, and what that does to the pressure reading and the apogee height, with its sign. It builds on
[altimeter-sensor-noise](altimeter-sensor-noise.md) §2 (the datasheets' tempcos; "no source found measures a bay's
temperature change in flight") and [altimeter-mach-error](altimeter-mach-error.md) §4–5 (how a vented bay lags).
Sources are 96 public flight logs that record the sensor's temperature, the makers' datasheets and handling notes,
one radiosonde paper and textbook heat transfer. **Inference** marks this note's arithmetic (1976 atmosphere, pad at
sea level unless stated). Read 2026-10-19.

**In short.** By apogee the sensor still reads its pad temperature: the temperature term is a few tenths of a meter
below 4 km, under the noise, and far under the ±8–11 m per-flight band bias of altimeter-sensor-noise §6.
- **The sensor barely moves on the way up.** In 86 logs from commercial altimeters (55 flights, apogees 259 m to
  18.9 km, median 2.3 km), the logged temperature at apogee is within ±0.6 K of the pad's in 76; 10–90% −0.30 to
  +0.60 K. The outside air cools 7–37 K over the same climbs (10–90%), so the sensor sees about 0 ± 3% of it (§2).
- **Above 4 km, flights tend to warm.** 11 of 13 read warmer at apogee: eight by +0.7 to +2.6 K, one fast 6.0 km
  flight by +6.7 K after +10.5 K in its first 8 s; the cause is unbounded (§2).
- **The cooling arrives in the descent.** Sealed bays cool a median 0.71 K/min after apogee and land a median 1.0 K
  under the pad (10–90% −6.2 to +0.1 K); a sensor opened to the air at deployment falls 4–34 K within a minute (§3).
- **Pads are hot.** Pad readings run 3.7–55.8 °C (median 28.5 °C); team-built boards with long pad records warm
  +2.3 to +13.1 K before launch, most still rising at liftoff (§4).
- **No maker states a sign; the error is either sign (Inference).** At the stated tempcos (0.5–1.5 Pa/K Bosch and
  Infineon, about 2 Pa/K and at most 4 Pa/K read from the MS5607's bands), 0.6 K is at most 0.27 m at a 3 km apogee;
  6.7 K at 6 km is at most 4.1 m (0.07%). One typical MS5607 curve gives both signs by where the die sits (§5–7).
- **Two fixed-sign terms are unsized:** a temperature reading that trails the MS5607's die (+221 Pa per K of lag at
  sea level) and a skipped second-order step below 20 °C. Neither is known to occur in a hobby altimeter (§6).

## 1. The logs and which temperature they carry

Searched on GitHub 2026-10-19 by each family's CSV header, plus the eight repositories of altimeter-sensor-noise;
`scripts/bay_temp.py` and `scripts/summarise.py` (standard library) compute every number in §2–4.

| Family (logs used) | Column | Sensor it comes from |
|---|---|---|
| Altus Metrum TeleMetrum, TeleMega, EasyMega (60) | `temperature`, 0.1 °C | the MS5607's die (**Inference**, below) |
| Featherweight Blue Raven (18) | `Temperature_(F)`, 0.1 °F | "temperature from baro sensor" [F3] pp. 22–23 |
| Missile Works RRC3 (5) | `Temperature`, 1 Hz | the MS5607 [MW1] pp. 2, 23 |
| PerfectFlite StratoLogger (2), CATS Vega (1) | `temperature`, baro `T` | part not stated |
| Team-built boards (10 sensors, 8 flights) | named columns | LPS-series baro, humidity chip, others |

- **Coverage.** 44 of the 86 commercial logs come from one university team's repository; the result of §2 holds in
  both halves (within ±0.6 K at apogee in 42 of its 44, 34 of the other 42). 78 have no license: statistics only.
- **Left out:** 12 Altus Metrum logs under 150 m apogee and 8 broken or pad-only files. Pad records are short: 62 of
  the 86 commercial logs start at liftoff (most Altus Metrum exports, all RRC3), the rest keep a few seconds.
- **Altus Metrum's source (Inference):** the MS5607 is the only pressure part on these boards [AM] p. 84, and
  telemetry sends `temp` beside `pres` [AT] pp. 3–5. Steps: 0.1 K Altus Metrum, 0.056 K Blue Raven.

## 2. Pad to apogee: the sensor holds the pad's temperature

ΔT is the median over apogee ± 0.5 s less the pad value: the median over the 5 s before liftoff, or the first sample
for the 62 logs that start at liftoff. The air's change is the standard lapse rate, 6.5 K/km, over the same climb.

| Apogee | Logs | ΔT at apogee, median | 10–90% | Range | ΔT ÷ air's change, 10–90% |
|---|---|---|---|---|---|
| under 2 km | 40 | 0.00 K | −0.10 to +0.20 | −0.50 to +0.40 | −0.027 to +0.020 |
| 2–4 km | 33 | 0.00 K | −0.39 to +0.10 | −0.56 to +0.50 | −0.007 to +0.022 |
| over 4 km | 13 | +1.28 K | −0.10 to +2.48 | −1.25 to +6.73 | −0.058 to +0.004 |
| all commercial | 86 | 0.00 K | −0.30 to +0.60 | −1.25 to +6.73 | −0.026 to +0.021 |
| team-built boards | 10 | −0.29 K | −1.43 to +0.32 | −1.65 to +0.45 | −0.016 to +0.077 |

- **Rate.** During the climb, median 0.000 K per 1,000 m (10–90% −0.135 to +0.172 K/km), against −6.5 K/km outside.
- **High and fast flights warm.** Ten logs read at least +0.5 K at apogee; nine flew over 5 km. The largest, a 6.0 km
  flight near 650–700 m/s in its first 4 s (**Inference** from its heights), rose +10.5 K in 8 s (1.3 K/s), read
  +6.7 K at apogee and landed 7.2 K under its pad value. The only clearly colder apogee is −1.25 K (5.2 km).
- **Two altimeters on one flight** agree at apogee within 0.0–0.4 K (28 groups); their pads differ by up to 7.4 K.
- **Inference: the altimeter's temperature is not the air's.** At apogee it reads the pad, so an outside-air
  correction to the height formula cannot use it.

## 3. After apogee: a slow sealed bay, a fast open sensor

- **Sealed bays** (67 logs with a landing): ΔT at landing median −1.0 K, 10–90% −6.2 to +0.1 K, worst −13.5 K on landing
  789 s after liftoff from 8.6 km. Cooling from apogee to landing: median −0.71 K/min (10–90% −1.56 to +0.08).
- **The lag outlasts the descent.** In 36 of 67 the coldest reading falls in the last 10% of the descent, while the
  air around it is warming again. A first-order fit relaxing to standard-lapse air gives τ median 1,259 s (10–90%
  240 s to the fit's 10⁵ s limit). It assumes the bay starts at air temperature, so τ is an order of magnitude
  (**Inference**).
- **Fast falls.** Four commercial logs fall 4–34 K within 60 s of apogee (two in one payload section, the CATS Vega,
  one primary altimeter), τ 50–180 s, and a team-built air-brake board 7 K. **Inference:** these sensors met outside
  air at deployment; the logs don't show how. Still warming at the log's end, two bound how far the pad bay sat above
  the air: at most 18 and 23 K.

## 4. The pad

- **Readings** span 3.7–55.8 °C (median 28.5, 10–90% 16.5–40.8 °C); 12 logs start at 40 °C or more. Two altimeters on
  one flight differ by 0.6–3.0 K on the pad (14 pairs); payload and main bays of one rocket by 3.5–7.4 K.
- **Soak.** Only team-built boards keep long pad records: seven sensors on five boards warm +2.3 to +13.1 K over
  9–91 min, six of them still rising at liftoff (0.06–0.96 K/min over the last 10 min). One warms quickly enough
  (+13.1 K in 13 min) to be power-on self-heating. No commercial log shows a soak trend.
- **Published:** no paper or team report logs a bay's temperature in flight. A builder estimates 10–40 °F above the
  outside by airframe colour [TRF]; §3's 18–23 K is the same size. Altus Metrum asks for opaque bays (sunlight) [AM].

## 5. What the datasheets state about temperature

| Part | Stated after compensation | Conditions | Sign | Source |
|---|---|---|---|---|
| BMP390 | ±0.62 Pa/K (a mean) | 900 hPa, 25–40 °C | ± | [S3] Table 2, p. 7 |
| BMP388 | ±0.75 Pa/K; ±1.0 below 0 °C | 700–1100 hPa, 0–55 °C | ± | [S4] Table 2, p. 7 |
| BMP280 | ±1.5 Pa/K | 900 hPa, 25–40 °C | ± | [S5] Table 2 |
| DPS310 | 0.5 Pa/K, a change from 25 °C | 1000 hPa, 25–65 °C | none | [S6] pp. 4, 9 |
| MS5607 | none; ±0.5 / ±1.0 / ±2.5 mbar after a one-point zero | 25 / 0–50 / −20–85 °C | ± | [S1] p. 4 |
| MS5611 | none; ±0.5 / ±1.0 / ±2.5 / ±5.0 mbar | 25 / 10–50 / −20–85 / −40–85 °C | ± | [S2] p. 4 |

- **The MS5607's bands as a slope (Inference).** From ±0.5 mbar at 25 °C to ±1.0 over 0–50 °C adds 0.5 mbar over
  25 K: 2 Pa/K. The whole ±1.0 mbar over 25 K, 4 Pa/K, is a ceiling. The wider band also widens pressure to 300 mbar,
  so part of the 0.5 mbar is not temperature.
- **Transients.** No maker sizes one. Bosch says that above 3.0 °C/s a correction note applies, from a representative
  [B3] p. 10, [B2] p. 14 (its public link withdrawn in 2019 [B3] p. 19); its internal temperature is "typically above
  ambient" [S3] p. 8; light on the port hole affects the reading [B3] p. 10. TE, Infineon and ST say nothing.
- **A measurement in flight.** On 36 twin flights, two radiosonde models' pressures drift apart to 0.5–0.6 hPa over
  the first 5 km [K17] p. 2489 as the air cools about 32 K. The paper blames no cause; if all of it were temperature,
  about 2 Pa/K, an upper bound (**Inference**).

## 6. The MS5607's compensation: two fixed-sign terms

- **The second-order step.** TE's compensation adds a correction below 20 °C (more below −15 °C) [S1] p. 9. With TE's
  example coefficients [S1] p. 8 it lowers the output by 17, 68 and 277 Pa at a die of 15, 10 and 0 °C at sea level
  (**Inference**). Firmware that skipped it would read a die cooling from 15 to 10 °C as 4.2 m low on a 300 m flight
  (1.4%); at §2's measured 0.6 K, 0.4–0.7 m. Whether hobby firmware applies it is not public; Altus Metrum's is GPL
  and not read here. Pads read above 20 °C in most logs (§4), where the step does nothing.
- **The lever on the temperature reading.** The first-order formula [S1] p. 8 removes the bridge's own drift, so an
  error in the temperature reading moves pressure by ∂P/∂T = +221 Pa/K at 1013 hPa, +152 at 700 and +63 at 300 hPa
  with TE's example coefficients (**Inference**). A reading that trails the die by τ while the die changes at R moves
  pressure by 221·R·τ Pa: a reading warmer than the die gives pressure high, altitude low. A PX4 MS5611 whose
  temperature froze while the board warmed drifted about 950 m over 20 minutes [PX], the sign this predicts. No
  source gives τ. Averaged over the climb, R is under 0.01 K/s in 57 of 86 logs; the fastest, 1.3 K/s over 8 s (§2).
  At τ = 0.1 s that is about 0.2 Pa, and 29 Pa at most (**Inference**).

## 7. Physics of the bay, and the apogee error (Inference unless cited)

- **Why the bay holds its heat.** On the way up air only leaves a vented bay; keeping it isothermal takes Q = V·Δp:
  14 J for a 54 × 200 mm bay to 3 km. Spread over walls, sled and board, with a board time constant C/(hA) of
  80–8,000 s (C 10–80 J/K, h 2–25 W/m²K [H], A 0.005 m²), the die should move 0.1–2 K by apogee; §2 sits low in it.
- **Radio self-heating:** about 0.15 W at +15 dBm [TI], at most 0.015 K/s on the board; steady on the pad.
- **The apogee error** is −k·ΔT·dh/dp, with dh/dp 0.0857, 0.1122 and 0.154 m/Pa at 300 m, 3 km and 6 km.

| Case | ΔT | k, Pa/K | Error at apogee |
|---|---|---|---|
| Below 4 km, 10–90% of logs | ±0.6 K | 0.5–4 | ±0.03–0.27 m at 3 km; ≤ ±0.21 m (0.07%) at 300 m |
| Over 4 km, typical warm | +0.7 to +2.6 K | 0.5–4 | ±0.05–1.6 m at 6 km (≤ 0.03%) |
| Largest logged | +6.7 K | 0.5–4 | ±0.5–4.1 m at 6 km (≤ 0.07%) |
| Stress case | 20 K | 4 | ±6.9 m at 300 m (2.3%), ±9.0 m at 3 km |

- **Sign.** Every stated tempco is ± or bare. One typical MS5607 curve [S1] p. 19, read by pixels, moves +16 Pa for a
  die cooling 5 K from 25 °C and −20 Pa from 10 °C: altitude low in one, high in the other. Use a symmetric bound.

## What this suggests

1. **M7.2's synthetic logs can keep the temperature term out, now on a measurement.** Below 4 km the term is under
   0.3 m at apogee, below the rest noise (0.16–1.14 m) and the ±8–11 m band bias. A stress option would draw
   k_T·ΔT with k_T ±0.5–4 Pa/K and ΔT ±0.6 K below 4 km, 0 to +2.6 K above (+6.7 K rare); the physics page's reason
   ("no source gives a bay's change") can cite §2 instead.
2. **apogee-bias's measurement term:** at 4 Pa/K the temperature part is under 0.1% of apogee in all 86 logs
   (largest 0.07%); the skipped second-order step and the trailing reading are fixed-sign terms of unknown occurrence.
3. **Don't take the altimeter's temperature as the air's** in any height correction or forensic fit: it reads the
   pad (§2). Outside-air temperature has to come from a weather source.
4. **A temperature-change flag in readings:** a log whose sensor moves over 3 K before apogee, or over 1 K/s, is
   outside this sample and worth a note beside its apogee.

## Open questions

- What heats the sensor on high, fast flights (§2): aerodynamic heating, the motor, or the board in thin air.
- Whether hobby firmware applies the MS5607's second-order step, how often it samples temperature, and how far the
  temperature reading trails the bridge (§6). No source gives either.
- A commercial altimeter's pad soak (no commercial log keeps over 5 s of pad); no log has outside air beside a bay's.
- Bosch's fast-change note and ST's AN5227 and AN4672 were unreachable; no bench test of a hobby baro under a
  temperature ramp against a reference was found. Which part the CATS and PerfectFlite boards log from.

## Sources

Datasheets and handling notes carry no reuse license: cited, cached privately. Logs without a license: statistics
only, no rows quoted.
- [S1] TE MS5607-02BA03 (03/2024), pp. 2, 4, 8–9, 19.
  <https://pages.hmc.edu/spjut/AdvRoc/PDF/MS5607-02BA03DataSheet.pdf>
- [S2] TE MS5611-01BA03 (06/2017), pp. 4, 8–9, 19.
  <https://www.amsys-sensor.com/downloads/data/MS5611-01BA03-AMSYS-datasheet.pdf>
- [S3] Bosch BMP390 DS002-07, pp. 7–8; [S4] BMP388 DS001-07, p. 7; [S5] BMP280 DS001-26, Table 2.
  <https://www.bosch-sensortec.com/>; [S6] Infineon DPS310 v1.2, pp. 4, 9.
- [B2] Bosch BST-BMP28X-HS000-07, p. 14; [B3] BST-BMP3xy-HS000-05, pp. 10, 19, under
  <https://www.bosch-sensortec.com/media/boschsensortec/downloads/handling_soldering_mounting_instructions/>
- [K17] Kawai et al., Atmos. Meas. Tech. 10, 2485–2498 (2017), p. 2489, CC BY 3.0.
  <https://doi.org/10.5194/amt-10-2485-2017>
- [H] Incropera et al., Fundamentals of Heat and Mass Transfer, 6th ed., typical h, as tabulated at
  <https://pages.mtu.edu/~fmorriso/cm3120/typical_values_heat_transfer_coefficient.pdf>
- [TI] TI CC1120 datasheet SWRS112H, §4.4. <https://www.ti.com/document-viewer/CC1120/datasheet/specifications>
- [AM] Altus Metrum manual v1.9.22, PDF pp. 75, 84, CC BY-SA 3.0. <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [AT] AltOS telemetry document, pp. 3–5, CC BY-SA 3.0. <https://altusmetrum.org/AltOS/doc/telemetry.pdf>
- [F3] Featherweight Blue Raven user's guide (2026-05-12), pp. 22–23. <https://www.featherweightaltimeters.com/>
- [MW1] Missile Works RRC3 manual v1.71, pp. 2, 23. <https://www.missileworks.com/downloads/>
- [PX] PX4 flight stack, issue #16353 in its main repository under <https://github.com/PX4> (2020-12-08).
- [TRF] Rocketry Forum, "K.I.S.S. flight computer development", p. 2, post #44 (2024-03-21).
  <https://www.rocketryforum.com/threads/k-i-s-s-flight-computer-development.180386/page-2>
- Logs (repository@commit, license): MIT: RocketPy-Team/RocketPy@9bd6ad3af8, werocketry/Hyperion-II@efda132ff4,
  rex-mcall/Shred-O-Vision@619af886f4, UND-ARC/IREC-ARC-2024@94a991bfc6, UCC-Rocket-and-Space-Exploration/CosmoSoft
  @1ced3f21d7, boulaetans/numericalrocketry@136ab67cdc. No license: NUSTARS/launch-vehicle@3a89e4be5a,
  ISSUIUC/flight-data@b8c95fa993, saividyud/NASA_25-26_Rocket_Simulations@5ec2189860, prs2026/Procyon@8c8b2b9f08,
  icl-rocketry/art-data@bee1c863eb, VoicefulEmu/DataAnalysisSnowbirdTest@cb6b8157a1, legomyleg/BYU_Telemetry_HUD
  @c66c04f4b3, Volta-Rocketry/CUBESAT-2026@eb7e79694c, lehangajanayake/ARD@a27d2518fb, gregoryw3/Pico-FreeRTOS
  @905dfe6c1f, NateOrlowski06/Rocket-Data@1f63804e36, AstroJays-Hopkins/Recovery-Software-Two-Stage@ce5fb3454e,
  wwang-8/L1_flight_data@51d0b032c4, Rose-Rocketry/2024-USLI-Vehicle@885c080af2, DexterCarpenter/BEAVS4-Simulation
  @8a7c5b6735, dpfeldman/dpfeldman.github.io@622ccb0cc0.
