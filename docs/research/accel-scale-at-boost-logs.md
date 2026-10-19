# Accelerometer scale at boost loads, measured in public logs

Part of [accel-scale-at-boost](accel-scale-at-boost.md), which holds the datasheet terms, the boards' calibrations
and what the project can do with this. Here: four measurements of the scale an accelerometer applies in boost, from
the public log set of [accel-bias-after-pad-zero](accel-bias-after-pad-zero.md) (the bias note). **Inference** marks
this note's arithmetic. g = 9.80665 m/s². Read and measured 2026-10-19.

**In short.** Altus Metrum's calibration applies a 2 g span of a whole number of counts, so its scale is rounded by
up to ±1.25–2.5% (§1). In boost, units differ by −2.4% to +1.4% on one flight, about 1% each (§2–3). The
supersonic Blue Raven gap grows in coast, not boost (§4). A GPS-referenced scale is too confounded to use (§5).

## 1. Measured: Altus Metrum's calibration is rounded to whole counts

**Method.** The AltOS CSV prints axial acceleration to 0.01 m/s² on a grid of q m/s² per count. A scan of q (as the
bias note did) then a least-squares refinement on every value gives q to about 10⁻⁵; 2·g/q is the 2 g span the
export applies [L2] `step.py`. Inputs: 37 exports from 14 units (the bias note's 34 logs plus the 3 it dropped by
rule), from the repositories in [L1].

**Result.** Every log's span is a whole number to within 0.0022 counts: 37 (2 units), 39 (6), 40 (5), 41 (1); no
unit's span changes between its flights (one unit has 7, 2024–2026). The applied sensitivity is 18.5–20.5 counts/g,
inside the ADXL375's 18.4–22.6 [D1]; the two 37-count units (flights of 2019 and 2022) fit the MMA6555's 18.2 [D4].

- **What it implies (Inference).** The part's true span is not an integer, so the applied scale is off by the
  rounding: up to ±0.5 count (±1.25% of 40) if the span is rounded once, ±1 count (±2.5%, sd 1.0%) if each endpoint
  is. The telemetry format sends `accel_plus_g` and `accel_minus_g` as 16-bit integers [A2] Table 6; which rounding
  AltOS does is in its source, which this project doesn't read.
- **Check against pairs.** Two Altus units on one flight with spans of 40 and 39 read 2.4% apart at burnout (P2,
  §3), the lower span reading higher, as rounding of a shared true sensitivity predicts (2.5%). Two units with
  equal spans on two flights read within 0.8% (P6, P7).

## 2. Measured: the high-g part against the same board's IMU, in boost

**Method** [L2] `hg_imu.py`. On Altus logs that carry IMU columns, the IMU axis that reads most on the pad is the
axial one. Both records less their pad readings equal (1 + s)·(f − f_pad), so their ratio over the boost is the
gain ratio, with every offset cancelled at the pad point. Samples where the IMU reads from 3 g to 90% of its flight
maximum (at most 15.5 g on a ±16 g IMU), after a 0.1 s moving average on both; logs with a clipped IMU or under 50
samples left out.

| Unit (span) | Logs | High-g / IMU − 1, mean | Range |
|---|---|---:|---:|
| A (41) | 6 | −3.2% | −4.2 to −3.0% |
| B (39) | 2 | +2.4% | +2.1 to +2.6% |
| C (40) | 7 | −0.9% | −2.5 to +0.7% |
| D (40) | 4 | +0.3% | −0.4 to +0.8% |
| E (40) | 3 | −1.4% | −2.1 to −1.0% |
| F, G, H (39, 39, 40) | 1 each | +0.1, −0.2, +1.3% | n/a |

- 25 logs, 8 units, IMU window means 3.9–11.5 g; unit means sd 1.7%; pooled within-unit sd 0.86% (4 units with 3 or
  more logs). The ratio of whole-boost speeds, where the IMU never clipped (24 logs), agrees within about 1 point.
- The spread mixes both parts: the IMUs carry ±3–4% uncalibrated ([main note](accel-scale-at-boost.md) §2–3).
  The span-41 unit reads lowest, and the span-39 units average +0.8% against −0.2% for span 40: the sign rounding
  predicts (8 units, weak evidence).
- **No curve between 1 g and 15 g shows.** The local slope (offset free) minus the pad-anchored ratio is
  −0.26% (±0.15 formal, ±0.40 from the scatter of 2.0%, n = 25), so the two parts' gains differ by under about 1%
  between 1 g and the boost (**Inference**).

## 3. Measured: two units on one flight, axial only

**Method** [L2] `pairs_axial.py`. Units in one airframe read the same axial specific force, so dv(t) = v₁ − v₂ =
(s₁ − s₂)·U(t) + (b₁ − b₂)·t, with U the axial integral; tilt, baro and GPS drop out. AltOS: the logged axial column;
Blue Raven: the 500 Hz file on its pad gravity direction. Clocks aligned by matching the first 2.5 s (0.01 s steps).
Reported: r = dv/U at burnout + 0.5 s, and s from fitting the line above over the ascent. Same-flight sets as in the
bias note; 3 pairs with one Blue Raven file failed alignment and 2 Blue Ravens without a 500 Hz file were left out.

| Pair | Units | U at burnout (m/s) | r | s |
|---|---|---:|---:|---:|
| P1 | TeleMetrum (37) − TeleMega (40) | 321 | −2.1% | −2.6% |
| P2 | EasyMega (40) − TeleMega (39), supersonic | 591 | −2.4% | −3.1% |
| P4 | TeleMega − Blue Raven | 197 | +1.3% | +0.9% |
| P6 | TeleMega C − D; C − Blue Raven; D − Blue Raven | 243 | −0.5, 0.0, +0.5% | +0.8, +1.1, +0.3% |
| P7 | TeleMega C − D; C − Blue Raven; D − Blue Raven | 250 | −0.3, +1.0, +1.4% | +0.3, +1.6, +1.3% |
| P8 | TeleMega − Blue Raven, supersonic | 666 | −1.5% | −2.0% |
| P9 | Blue Raven − Blue Raven, supersonic | 601 | +0.4% | +0.4% |

- 11 pairs, 7 flights: r −2.4% to +1.4% (rms 1.3%), s −3.1% to +1.6% (rms 1.6%). For two independent units the
  difference has √2 times one unit's sd, so each carries about 0.9–1.1% (**Inference**; P6 and P7 reuse units).
- Subsonic Blue Ravens read 0.0–1.6% below the TeleMega beside them (5 pairs, 3 flights); P8's reads 1.5–2.0% above,
  having gained 78% of its speed above 30 g, near or past the switch to its high-range part (1.4 s [L3]).
- **Confounders (Inference).** Clock alignment ±0.01 s at a 2–6 g burnout tail: ±0.2–0.6 m/s, under 0.25%. An
  offset difference of 40 mg moves r by up to 0.9% at 5.6 s; s removes it under a constant offset, and r and s
  differ by up to 1.2 points (P6). Boards misaligned by 2° lose 0.06%.

## 4. The supersonic Blue Raven gap builds in coast

In accel-bias-after-pad-zero §4, four of five supersonic Blue Raven logs read +21 to +35 m/s above the baro near
apogee, judged to build in boost from baro speeds. On P8, the one such flight with an Altus unit aboard (whose own gap
to the baro is −2.8 m/s), the axial gap (TeleMega − Blue Raven) is −9.5 m/s at burnout (3.0 s), −19.7 m/s at 8 s,
−29.9 m/s at 20 s and −42.2 m/s at apogee (39.3 s) [L2] `profile_p8_axial.out`. It grows through the coast:
about 0.23 g while drag decelerates the rocket at 2.5–5 g, 0.09–0.10 g later (**Inference**, from the slopes). A
coast gain 3.2% low on deceleration plus 23 mg fits it with half the residual of one scale plus an offset
(1.6 against 3.2 m/s rms); n = 1, and the same two-gain fit is ill-conditioned on the subsonic pairs, whose coasts
decelerate too little. The two Blue Ravens of P9 agree within 0.7 m/s at apogee, so the cause is common to the
units, not a calibration. It is not the 32 g switch either: P9 peaked at 24.4–24.5 g.

## 5. GPS-referenced scale: tried, too confounded

[L2] `gps_baro.py` fits the baro height's scale to GPS height in descent (where the receiver tracks well,
[gnss-in-boost](gnss-in-boost.md)) and applies it to the bias note's ascent fit h_acc − h_baro = c + ½·b·τ² + k·h.
On 17 tilt-projected logs passing a check (GPS apogee within 1.5% of baro, rms under 15 m), the implied accelerometer
scale runs −2.3% to +8.5% (median +1.7%). Grouped by span it orders as rounding predicts (median +7.1%, +1.7%,
−0.3% for 39, 40, 41), but one unit's five flights spread −2.3 to +2.2%, and the span-39 median exceeds the ±2.5%
rounding bound. Tilt projection over a 20–40 s ascent and baro lag at apogee are not sized here, so the figure
carries errors of a few percent and is not used for a prior.

## Sources

- [D1] Analog Devices, ADXL375 Rev. B, Table 1 p. 3; [D4] NXP, MMA655x Rev. 7.0, §2.4, PDF p. 7. URLs in
  [accel-scale-at-boost](accel-scale-at-boost.md).
- [A2] K. Packard, B. Garbee, *AltOS Telemetry* v1.9.22.2 (CC BY-SA 3.0), Table 6.
  <https://altusmetrum.org/AltOS/doc/telemetry.html>, read 2026-10-19.
- [L1] Logs, by repository at commit, as listed in [accel-bias-after-pad-zero](accel-bias-after-pad-zero.md) [L1]:
  RocketPy-Team/RocketPy (MIT), werocketry/Hyperion-II (MIT), ISSUIUC/flight-data, NUSTARS/launch-vehicle,
  prs2026/Procyon, saividyud/NASA_25-26_Rocket_Simulations, DexterCarpenter/BEAVS4-Simulation,
  VoicefulEmu/DataAnalysisSnowbirdTest, legomyleg/BYU_Telemetry_HUD (no license: statistics only, no row quoted).
- [L2] This topic's scripts and outputs, `refs/research/accel-scale-at-boost/scripts/` (not committed); `run.sh`
  re-runs them all, standard library only.
- [L3] The bias note's per-log output `br.json` (time and speed gained above 30 g).
