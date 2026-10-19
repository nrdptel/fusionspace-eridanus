# What accelerometer bias survives an altimeter's pad zero

This note asks how large a constant accelerometer offset is left once a hobby altimeter has zeroed itself on the
pad, for M7.2's synthetic test (which draws ±50 mg, the top of a guessed ±10–50 mg) and for the interval on an
accelerometer-derived speed reading. It builds on [altimeter-sensor-noise](altimeter-sensor-noise.md) §4, which
found no maker stating the residual, and on [altimeter-bay-temperature](altimeter-bay-temperature.md), whose public
log set it reuses. Sources are makers' manuals, datasheets, a CATS paper, forum posts by a maker, and 49 public
flight logs measured here. **Inference** marks this note's arithmetic. g = 9.80665 m/s²; 1 mg is 0.0098 m/s².
Read 2026-10-19.

**In short.** Measured over a flight, the offset is about 10–20 mg, and it varies from flight to flight.
- **No maker states the residual, and the zeros differ in kind.** Blue Raven re-zeros along the measured gravity
  direction on the pad; Raven4 re-zeros its axial offset while it waits; Altus Metrum keeps a factory or user
  two-point (+1 g, −1 g) calibration and stopped re-zeroing on the pad in AltOS 1.3.1; CATS Vega carries the
  offset as a Kalman state the barometer corrects. Eggtimer Proton, MARSA54L and Entacore AIM document none (§1).
- **Datasheet terms leave 1–8 mg typical, 5–131 mg worst, after an on-pad zero** (§2, **Inference**): thermal
  drift over the bay's usual ±0.6 K, rail tilt and the zero's own noise. Without a pad zero, an ADXL375's 10 mg/°C
  over a bench-to-pad gap gives about 60 mg.
- **Measured in flight: Altus Metrum −39 to +48 mg (sd 22, n = 34 logs); subsonic Blue Raven −9 to +24 mg (sd 10,
  n = 10)** (§3). The figure is the gap between accelerometer-integrated and baro speed at apogee, divided by the
  time to apogee, so a scale error drops out. A 20 mg offset held 20 s is 3.9 m/s and 39 m at apogee.
- **It moves from flight to flight.** Within one Altus unit it scatters with sd 20 mg (24 flights, 5 units), of
  which the export's ±25 mg rounding is about 14 mg; units sharing a flight spread a median 24 mg (9 flights) (§3).
- **Four of five supersonic Blue Raven logs read +21 to +35 m/s fast near apogee** (about 4% of peak speed). Read
  against the baro it seemed to build in boost; against a TeleMega on one flight it grows through the coast
  ([accel-scale-at-boost-logs](accel-scale-at-boost-logs.md) §4). The cause is unknown (§4).
- **Quiet readings don't predict the flight offset.** Altus pad means (10–27 rows) do not correlate with it
  (r = −0.07); every Blue Raven high-rate pad reads 5–8 mg short of 1 g; landing rests are too noisy (§3).

## 1. How each maker zeroes the accelerometer

| Family | Zero, and when | Source |
|---|---|---|
| Blue Raven | Offset along measured gravity (3-axis magnitude against 1 g), updated to liftoff | [F1] pp. 17, 32 |
| Blue Raven | Six-position user calibration of the low-range axes, within days of flight | [F1] pp. 5, 30 |
| Raven3, Raven4 | Axial offset, updated continuously on the pad | [F2] p. 4 |
| Raven3, Raven4 | Four-position user calibration when the pad reads outside 0.8–1.2 g | [F2] p. 10 |
| TeleMetrum, TeleMega, EasyMega | Two-point (+1 g, −1 g), factory or user (since 1.8.2) | [A1] pp. 42, 109 |
| Same | No pad re-zero of the main axis since 1.3.1; a pad average `ground_accel` is logged | [A1] p. 130; [A2] |
| CATS Vega | Offset as a Kalman state, corrected by baro height after a 10 s still phase | [C1] pp. 2–3 |
| Proton, MARSA54L, AIM | None documented (the Proton warms up 3 min) | [E1] p. 20; [M1] p. 13; [N1] p. 22 |

- **Blue Raven's example.** The manual's flight summary shows a pad reading of 0.987 g in magnitude and a
  low-range z correction of 0.0116 g; the high-g z axis is corrected by 0.933 g [F1] p. 32. On the high-g part the
  pad zero removes an offset of order 1 g (H3LIS331DL ±1 g typical [D2] p. 4).
- **Altus.** The 1.3.1 notes say the main axis "uses the factory calibration values instead of re-calibrating on
  the pad each time", to stop false boost detection when the board is handled [A1] p. 130. The 1.9.6 notes name
  "small errors in accelerometer calibration leading to large accumulated error in speed" [A1] p. 97. The exported
  CSV's acceleration column reads 0 on the pad and sits on a grid of about 51 mg, so the export subtracts a pad
  reading rounded to a count, ±25 mg (**Inference**, from the logs in §3). Whether the flight software does the
  same is not documented.
- **Raven.** The maker writes on a forum that the pad's gravity reading is assumed to act through the flight
  [T2], and that some Raven3 units' converter nonlinearity shows at many times the calibrated range [T1].

## 2. What a zero leaves, from the datasheets (Inference)

Terms: offset tempco times the bay's temperature change in the ascent (±0.6 K pad to apogee in 76 of 86 public logs;
worst a +10.5 K rise in the first 8 s of a 6 km flight [B1]); rail tilt g·(1 − cos θ) for a single-axis zero (1.4
mg at 3°, 15.2 mg at 10°); and the zero's noise, density/√(2T) over T = 1 s (no maker states its window). Typical:
0.6 K, 3°, root-sum-square. Worst: 10.5 K, 10°, plain sum. Turn-on repeatability and shock shift are stated by no
datasheet read; a per-flight zero removes the first.

| Family (part) | Zero | Total typical / worst (mg) | ½·b·t² at 10 s | at 20 s |
|---|---|---|---|---|
| Blue Raven low range (LSM6DSO32 class†) | Magnitude, per flight | 0.6 / 4.7 | 0.3 / 2.3 m | 1.1 / 9.1 m |
| Raven4 (part unnamed; 5–10 mg/°C assumed) | Axial, per flight | 7.9 / 131 | 3.9 / 64 m | 16 / 257 m |
| Altus, `ground_accel` subtracted (ADXL375, 10 mg/°C) | Axial, per flight | 7.1 / 124 | 3.5 / 61 m | 14 / 243 m |
| Altus, stored calibration only | Bench, one-time | 60 / 435+ | 30 / 213+ m | 118 / 853+ m |
| CATS Vega (LSM6DSO32†) | Still phase, then a state | 1.4 / 16 | 0.7 / 8 m in boost | baro corrects |

- A magnitude zero (Blue Raven) has no cos θ term: a lateral offset b⊥ leaks in only as b⊥·sin θ·cos θ
  (0.5 mg for 10 mg at 3°, 3.4 mg for 20 mg at 10°).
- † LSM6DSO32 values (±0.1 mg/°C, 0.22 mg/√Hz) were read once on 2026-10-16 and could not be re-read: assumed.
- The Altus no-pad row assumes a 20–25 °C bench against the logs' 28.5 °C median pad; 330 mg is the 55.8 °C pad.
- **Scale is not offset.** A reading (1 + s)·f + b zeroed at rest gives (1 + s)·a, so Δv = s·v and Δh = s·h: the
  speed error is zero at apogee and s·v_max at the top speed (1% of 250 m/s is 2.5 m/s). Nonlinearity
  (H3LIS331DL 2% over ±70 g [D2] p. 4) and cross-axis terms are bounded only as fractions of full scale.

## 3. Measured in public logs

**Data.** The public log set of [altimeter-bay-temperature](altimeter-bay-temperature.md) plus the 500 Hz high-rate
files beside 13 Blue Raven logs, from 14 repositories [L1]. Of 76 AltOS CSV exports, 42 carry an accelerometer on a
count grid (the other 34 are baro-only units whose acceleration column is a filter output); 5 could not be read or
lack a deployment state and 3 were dropped by rule (41° tilt at the speed peak, a 20–33 Hz telemetry record, an
event 2.6 s before apogee): 34 logs, 30 flights, 14 units, 100 Hz. Of 17 Blue Raven logs, 2 were dropped (a failed
attitude solution, 33° tilt): 15 logs from 13 flights, 5 of them supersonic.

**Method.** The offset b_v is (v_acc − v_baro)/t* at t* = the baro apogee or 0.2 s before the first deployment,
whichever is first. v_acc is the AltOS column projected with the logged tilt where it exists (else taken as vertical),
or the Blue Raven's own Velocity_Up checked against an integration of its 500 Hz file. The gap v_acc − v_baro is the
Theil–Sen slope of h_acc − (1 + k)·h_baro over the 3 s before t*, with baro height from the 1976 atmosphere and k
from a least-squares fit of h_acc − h_baro = c + ½·b·τ² + k·h_baro over boost and coast (below 100 m/s). Since
true speed is zero at apogee, an accelerometer scale error drops out (§2). Scripts are re-runnable, standard library
only, with the sha256 of all 108 inputs [L2].

| Group | n logs | b_v range (mg) | median | sd | \|b_v\| median, 90th pct | Speed gap at t* (m/s) |
|---|---|---|---|---|---|---|
| Altus Metrum, all | 34 | −38.6 to +47.9 | +9.3 | 22.1 | 17.9, 39.0 | −13.0 to +8.4 |
| Altus Metrum, subsonic | 29 | −38.6 to +47.9 | +12.8 | 20.4 | n/a | −9.1 to +7.1 |
| Blue Raven, subsonic | 10 | −9.0 to +23.7 | −2.4 | 9.9 | 6.8, 10.5 | −2.1 to +5.2 |
| Blue Raven, supersonic | 5 | −18.2 to +88.0 | +71.2 | 44.5 | n/a | −8.7 to +34.6 |

- **Repeatability.** Altus units with three or more logs give a pooled within-unit sd of 20.2 mg (24 flights, 5
  units; unit means +7.6 to +23.3 mg). One unit read +1 to +46 mg over five flights in 2024–25 and −35 and −12 mg
  in 2026. The export's ±25 mg rounding alone is sd 14.4 mg, so about 14 mg is the units' own flight-to-flight
  drift (**Inference**). Two to four units on one flight (Altus and Blue Raven mixed, one a supersonic step flight)
  spread 5.0–95.4 mg (median 23.9, 9 flights); for two independent units the median spread is about 0.95σ, so
  σ ≈ 20–25 mg (**Inference**).
- **The fitted k follows the season** on Altus logs (June–August median +0.10, n = 10; December–April −0.01,
  n = 16), so it mostly measures the baro's temperature scale, not the accelerometer. Removing k moves b_v by −12 to
  +6 mg. A fit for b without k is biased high (median +42 mg Altus, +57 mg Blue Raven) and should not be used.
- **Confounders in b_v:** baro lag δ gives −1,000·δ/t* mg (−5 mg for 0.1 s at 20 s); leaving tilt out adds a
  median +5.4 mg (−5.0 to +19.3, 27 logs); gravity falling with height, −0.1 to −1.7 mg; the export's zero
  rounding, ±25 mg uniform. The two speed estimates differ by a median 0.4 m/s (Altus) and 1.8 m/s (Blue Raven). So
  the sds are upper bounds on the units' spread (**Inference**).
- **Quiet figures.** Altus pad means are −87 to +127 mg over 10–27 rows (median −18), mostly rail noise and
  rounding, uncorrelated with b_v (r = −0.07, n = 34). Blue Raven high-rate pads read −7.8 to −5.4 mg against
  1 g on all 15 files; local gravity spans only ±2.7 mg, so the shared −6 mg looks like the file's processing,
  not the units (**Inference**). After landing the rocket lies on its side, so only |a| compares with 1 g: Blue Raven
  −7.4 to +13.8 mg (n = 11); the Altus rest check has a median sd of 190 mg, too noisy for a 20 mg offset, and
  bays cool a median 1.0 K by landing [B1], ±10 mg on an ADXL375.

## 4. The supersonic Blue Raven step

On four of five supersonic Blue Raven logs (3 flights, 3 units; one unit flew twice) the scale-corrected speed gap
is already +21 to +34 m/s by mid-coast and changes by about 11 m/s at most to apogee: 3.7–4.6% of peak speed
(**Inference**). The 500 Hz files give the same (b_v +42 to +104 mg), so it is in the acceleration record, not only
the device's filter. Two Blue Ravens on one of these flights agree within 15 mg, and a TeleMega on another reads
−2.8 m/s, which clears the baro. Three of the four
peak at 22–25 g, below the 32 g switch to the high-range part [F1] p. 24, and subsonic flights to 61 g show no
step. The fifth log (peak about 1,080 m/s) shows none. Against the TeleMega, the gap grows through the coast
([accel-scale-at-boost-logs](accel-scale-at-boost-logs.md) §4): a coast gain or vibration rectification would fit.

## 5. Published gaps between an accelerometer and a reference

| Source | Instrument | Reference | Gap (accel − reference) |
|---|---|---|---|
| RASAero comparisons, A-601 [R1] | ARTS II accel | GPS, 42,771 ft | +2,153 ft (+5.0%) |
| RASAero comparisons, MESOS [R1] | Kate instrumentation | GPS, 293,488 ft | +783 ft (+0.27%) |
| Builder's five flights, 2014 [T2] | Raven | Same unit's baro | +219 to +548 ft (+8.9 to +18.2%) |
| Maker, 2019 [T1] | Raven4, one flight | 10 Hz GPS | Accel apogee within 1% |

All signs found are accelerometer-high, the sign rail tilt and path length give. The thread states neither units nor
times; read as feet over 9–13 s to apogee, the builder's 9–18% would need 81–318 mg as pure offset, far above
§3, so tilt or nonlinearity is the likelier cause (**Inference**). No source splits a gap into offset, scale and
tilt, and none states a residual in mg after a pad zero.

## What this suggests

1. **M7.2's synthetic test:** replace the uniform ±50 mg draw (`ACCEL_BIAS_G` in the synthetic logger) with a normal
   draw of sd 20 mg for the AltOS profile (the export's ±25 mg zero rounding is inside it) and sd 10 mg for a Blue
   Raven profile. Both are measured upper bounds; ±50 mg stays a fair worst case (it covers every subsonic log here).
2. **The accelerometer-derived top speed:** an offset of 20 mg costs 0.6 m/s at a 3 s burnout, so the offset barely
   moves it; scale and nonlinearity (s·v_max) do. On supersonic Blue Raven files the interval should carry the
   measured +4% until its cause is known.
3. **Apogee and coast speed from the accelerometer alone:** carry an offset of 20 mg (0.2 m/s², 1σ): 2 m/s per 10 s
   of flight, 3.9 m/s and 39 m at a 20 s apogee.
4. **A pad or landing mean is not the bias:** fit it against the baro over the flight, with a scale term (§3).

## Open questions

- What causes the supersonic Blue Raven step? Subsonic and supersonic high-rate logs from the same rockets, or a
  bench spin or shaker test, would separate a boost gain from vibration rectification.
- Does AltOS subtract `ground_accel` in flight? Converting a log's pad acceleration with its stored `accel_plus_g`
  and `accel_minus_g` [A2] would show, without reading source code.
- Does an Altus unit's offset drift with temperature or age? One unit moved about 50 mg in a year.
- Why do Blue Raven high-rate pads read 6 mg short of 1 g on every unit? The manual doesn't say.
- No pad zero's averaging window, and no turn-on repeatability or shock shift for the ADXL375 or H3LIS331DL, is
  stated. The ADXL375 and LSM6DSO32 sheets could not be re-read on 2026-10-19 (time-outs); their values are as
  read on 2026-10-16 for altimeter-sensor-noise.
- Student-built boards with both sensors in the public set were not measured; neither were Eggtimer or CATS logs.

## Sources

Manuals, datasheets and forum posts are cited and cached privately; the Altus Metrum documents are CC BY-SA 3.0 and
the CATS paper CC BY 4.0. Logs from repositories without a license are used for statistics only; no row is quoted.
- [A1] Altus Metrum, *AltOS: The Altus Metrum System*, v1.9.22.2 (2025), printed pp. 7, 37, 42, 97, 109, 130.
  <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [A2] K. Packard, B. Garbee, *AltOS Telemetry*, v1.9.22.2, Tables 6, 9.
  <https://altusmetrum.org/AltOS/doc/telemetry.html>
- [F1] Featherweight, *Blue Raven User's Guide* (May 2026), PDF pp. 5, 17, 23, 24, 30, 32; [F2] Raven4 manual
  (2020), PDF pp. 4, 10. <https://www.featherweightaltimeters.com/>
- [C1] J. Binz, N. Stojoski, L. Jost, "CATS: Empowering the next generation of rocket scientists through educational
  flight computers", ESA PAC Symposium 2024, pp. 2–3. <https://arxiv.org/abs/2511.04725>
- [E1] Eggtimer Proton User's Guide 2.01C, pp. 20, 22; [M1] MARSA54L manual v1.4; [N1] Entacore AIM XTRA/BASE manual.
- [D2] ST H3LIS331DL datasheet DS9012 Rev 5, Table 2 p. 4. ADXL375 Rev. B p. 3 values as in
  [altimeter-sensor-noise](altimeter-sensor-noise.md) §4; LSM6DSO32 Rev 1 as read once on 2026-10-16 (†).
- [B1] [altimeter-bay-temperature](altimeter-bay-temperature.md), In short and §2–4.
- [R1] RASAero, "RASAero II Comparisons with Altitude Data", Notes 1, 8. <https://www.rasaero.com/comparisons-alt.htm>
- [T1] Rocketry Forum, posts by Featherweight's maker, 2017-05-06 (thread 140400) and 2019-04-26 (thread 146560).
  <https://www.rocketryforum.com/threads/featherweight-raven3-accel-data.140400/>
- [T2] Rocketry Forum, "Accelerometer vs. Barometer Altitudes", 2014, posts #1 and #10 (#10 by the maker).
  <https://www.rocketryforum.com/threads/accelerometer-vs-barometer-altitudes.66981/>
- [L1] Logs (repository@commit; MIT where marked, else no license): RocketPy-Team/RocketPy@9bd6ad3af8 (MIT),
  werocketry/Hyperion-II@efda132ff4 (MIT), rex-mcall/Shred-O-Vision@619af886f4 (MIT), ISSUIUC/flight-data@b8c95fa993,
  NUSTARS/launch-vehicle@3a89e4be5a, prs2026/Procyon@8c8b2b9f08, saividyud/NASA_25-26_Rocket_Simulations@5ec2189860,
  DexterCarpenter/BEAVS4-Simulation@8a7c5b6735, VoicefulEmu/DataAnalysisSnowbirdTest@cb6b8157a1,
  legomyleg/BYU_Telemetry_HUD@c66c04f4b3, icl-rocketry/art-data@bee1c863eb, Volta-Rocketry/CUBESAT-2026@eb7e79694c,
  lehangajanayake/ARD@a27d2518fb, gregoryw3/Pico-FreeRTOS@905dfe6c1f.
- [L2] This topic's scripts and outputs, kept with the cached logs (not committed: most logs carry no license).
