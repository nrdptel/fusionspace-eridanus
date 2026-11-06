# Vent lag or port pressure? The public logs that can tell read the port

This note finds public flights giving an avionics bay's size and vents beside a baro log and an independent boost
track, and applies [altimeter-port-lag](altimeter-port-lag.md) ([P]) §3's sign rule: vent lag can only lower a
baro's slope-read top speed; a port's error can raise it. Are the logs' low climb readings (Cp +0.005 to +0.15,
[T1]) lag or port, for #872 and M2.3c2? Logs read 2026-11-05; **Inference** marks this note's own analysis.

**In short.** Two public sources document a vented bay beside a baro and an accelerometer: three airframes, eight
flights. Every flight outside the read's noise puts the baro top above the independent top, the port's side; none
reads below. On this thin sample, the low climb readings that can be split are port, not lag.
- **Who qualifies (§1).** NC State's full-scale air-brake rocket, three flights [N1] [N3]; Harvey Mudd's DOFPro
  Vulcanite and Mini DX3, five usable flights [F1] to [F6]. Near-misses: WE Rocketry's Hyperion II (bay size, no vents);
  shake_n_bake and Prometheus give no bay. The low readers in [T1] publish no bay or vents.
- **Low in the climb, high at the top (§3, §5, §6).** Six flights on four airframes read low going up (Cp +0.026 to
  +0.064) and their tops +2.1 to +28.5% above the same fit on the independent track. A pure lag of that Cp would read
  1.6 to 5.9% below, a pure port 3.5 to 15% above (**Inference**, synthetic flights).
- **The vents are small (§2).** NC State's sensor reads through four air-brake slots, not its two 1/4 in vents: C about
  10⁻⁷, so that rocket tests where the pressure is taken. The other vents give C 10⁻⁵ to 10⁻³, or 0.001 to 0.024 with
  cross flow [V]: a top 0 to 1.8% low, against port effects of 2 to 165%.
- **High in both (§4).** The five DOFPro flights read high going up (Cp −0.006 to −0.10) and at the top (+7 to +165%).
  The Mini DX3's ±16 g accelerometer clips, but its high-g axis and the height closure keep its top; its three flights'
  sizes are suspect. A steady Cp of −0.007 lifts the read top about 1%, not 8%.
- **What it moves.** #872: the port term carries the logs' Cp, both signs; a vent term stays apart, sized from the bay.
  M2.3c2: climb traces carry 36 to 284 m of port error, either way. Three documented airframes are a sample, not a
  bound, and a Cp under about 0.02 can't be split this way (§6, §7).

## 1. Which flights qualify, and the near-misses

| Source | Flights | Bay size | Vents | Baro; independent | Terms |
|---|---|---|---|---|---|
| NC State AirbrakesV2 [N1] [N3] | pel. 1, 2, 4 (2025) | 15.94 in, ID 5.82 in | 2 × 1/4 in; slots | CX5 | MIT |
| Same [N1] | shake_n_bake; 5 subscale | not given | not given | CX5 | MIT |
| DOFPro [F1] to [F6] | 5 usable of 7 (2021) | pages, `.ork` | 3 holes each | MS5, 55 Hz; ±16 g, high-g | CC, unnamed |
| WE Rocketry [W1] [W2] | Hyperion II (2025) | 33 cm; 13.244 cm across | not found | TeleMega, Blue Raven | CC BY 4.0 |
| WE Rocketry [W3] [W4] | Prometheus (2022) | not given | not given | TeleMetrum, TeleMega | none; one log CC BY 4.0 |

- **Geometry, no log.** NC State's 2022 payload bay (8 × 5.94 in, four 0.2256 in ports) [N5] §4.4.3.5, p. 73; 17 NC
  State reports (2019 to 2026, baro-only); Peak of Flight 543 [PF]; DOFPro's dual-deploy Vulcanite (GPS logs only) [F7].
  **Log, no geometry.** Hedy, WVU, Piccard, Forum A [T1]; NC State's `purple_launch`; Aerospace Team Graz (no license)
  [G1]; DOFPro's 38 and 54 mm rockets [F7]. **Excluded after reading.** Vulcanite J270W (Mach 1.1; baro 249 m off at the
  top); Mini DX3 G138T (shredded at Max-Q [F1]); Hyperion I (integrated apogee 9% high).

## 2. Each bay's C, and which opening the sensor sees (Inference)

C = (V/(Cd·A·n·H))², a steady Cp of C [P] §1, at Cd 0.6, n = 1, H 8,271 to 8,434 m. With cross flow, C_eff runs
from max(0.4·S, C at Cd 0.88) to 1.32·S + C at Cd 0.62, S = V/(n·H·A) [V] §5. Last column: a pure lag by §6's law,
on a flight like the bay's own (`verify/v_lagrows.py`).

| Bay (opening) | V | Area | C | C_eff | Top, lag |
|---|---|---|---|---|---|
| NC State aft: the CX5 (4 slots, 0.35 × 2.94 in) | 3.5 L, half? | 2,655 mm² | 7×10⁻⁸ | 0.0001 to 0.0002 | nil |
| NC State forward, not the CX5 (2 × 1/4 in) | 6.95 L | 63 mm² | 4.7×10⁻⁴ | 0.005 to 0.018 | 0 to −0.8% |
| Vulcanite (3 × 1.59–2.18 mm) | 489–867 cm³ | 5.9–11.2 mm² | 7.7×10⁻⁵ to 8.7×10⁻⁴ | 0.002 to 0.024 | 0 to −1.8% |
| Mini DX3 (3 × 1.61–1.78 mm) | 139–278 cm³ | 6.1–7.4 mm² | 1.4×10⁻⁵ to 8.3×10⁻⁵ | 0.001 to 0.007 | 0 to −0.6% |

- **The CX5 sees the slots; lag there is nil.** Sled bulkheads split NC State's bay: forward, the recovery altimeters
  and two 1/4 in holes "for pressure readings"; aft, the air brakes, whose fins swing out through switchband slots [N3]
  §3.3.5, p. 9, the IMU sled last [N3] §5.3, p. 70; the CX5 reads its own pressure [N1] [D1]. If the partition leaks
  (not stated), the holes' 1/42 of the slots' area pulls the bay (1/42)², 1/1,800, of the way to their pressure. The
  whole 6.95 L gives C 3×10⁻⁷; lag would show only with under 20 to 40 mm² (1.5%) of the slots open past the retracted
  fins [N3] Fig. 3.22.
- **DOFPro's bays** run from the nose shoulder to the coupler's bulkhead, vents 1 in above the joint [F2] [F3]; high V
  adds the hollow nose. Assumed: a 3 to 4 in Mini DX3 coupler (not in the `.ork` [F4]) and that the logged rockets
  follow the pages. The archive's 50.6 mm Vulcanite diameter would scale its C by 0.56.

## 3. NC State: low going up, high at the top

Method (**Inference**): a 6.5 K/km lapse from the pad temperature; a 0.3 s median, then the highest centered 0.5 s
slope ([P] §3), up to 1.5 s past the strapdown top, before the brakes act; on the strapdown it cuts 0.68 to 1.40%.

| Flight (date) | Strapdown top, m/s | Baro top, m/s | vs top / fit | Climb Cp | Closure at baro apogee |
|---|---|---|---|---|---|
| pel. 1 (2025-02-22) | 167.6 | 169.3 | +1.0 / +2.1% | +0.026 to +0.028 | −1.9 m, −1.0 m/s |
| pel. 2 (2025-03-08) | 156.8 | 161.3 | +2.8 / +3.9% | +0.030 to +0.036 | −27 m, −15.4 m/s |
| pel. 4 (2025-05-04) | 168.9 | 183.6 | +8.7 / +9.5% | +0.037 to +0.043 | −31 m, −14.4 m/s |
| shake_n_bake (2024-09-28) | 219.6 | 232.2 to 234.4 | +5.8 to 6.8 / +6.6 to 7.6% | +0.042 to +0.046 | −12 m, −3.8 m/s |
| 5 subscale | 94.6 to 133.1 | 92.2 to 128.8 | −3.8 to +0.8 / −2.7 to +1.5% | none | −11 to +17 m |

- **The shape.** At Mach 0.5 to 0.65 the baro trails 36 to 94 m at 2 s (Cp = 2g·Δh/V²), then climbs faster than the
  strapdown (pel. 4: 180 against 164 m/s at 2.5 s): a lag never can; a port with Cp > 0 does [P] §3.
- **Checks.** Pel. 2 and 4's strapdowns end 14 to 15 m/s off, pel. 1's within 1 m/s; a steady drift costs at most 1% at
  the top. shake_n_bake's baro is the CX5's estimate, read two ways of resampling its 25 Hz steps (pel. 4: +9.2 to
  +10.6% so, +8.7 to +9.0% raw). Subscale: a steady 5 to 20 m offset, no signal.

## 4. DOFPro: high going up and at the top

Method (**Inference**, `top_compare.py`, `verify/v_dof.py`; a cell gives both reads where they differ): strapdown
on the gyros, the high-g axis scaled in for a clipped ±16 g one; the 1976 lapse; the same read, which cuts −0.8 to
−3.0% (G78: +2.2% or −2.2% by read, 5,120 or 5,187 of 5,514 pressure rows kept). "Own track" reads the strapdown
height plus D = −Cp·V²/(2g) at the measured Cp.

| Flight (2021) | Accel top, m/s | Baro top, m/s | vs fit / top | Cp near the top | Own track | Apogee ratio |
|---|---|---|---|---|---|---|
| Vulcanite H128W (04-10) | 143.1 | 152.1 | +8.0 / +6.3% | −0.007 to −0.011 | +1.2 to +3.0% | 1.053 |
| Vulcanite G78 (02-05) | 97.6 to 99.2 | 104 to 109 | +7 to 9 / +7 to 10% | −0.021 to −0.030 | +4.7 to +6.4% | 0.935 |
| Mini DX3 G77R (02-05) | 201.7 to 208.3 | 519 to 523 | +158–165 / +151–157% | −0.08 to −0.10 | +111 to +143% | 0.969 |
| Mini DX3 G79W (02-05) | 196.0 | 320.5 | +65 / +64% | −0.062 to −0.071 | +46 to +54% | 1.000 |
| Mini DX3 H128W (04-10) | 279.1 | 587.9 | +114 / +111% | −0.039 to −0.059 | +56 to +91% | 1.057 |

- **The Mini DX3 accelerometer (Inference).** The ±16 g axis sits at its rail 0.06 to 0.95 s after liftoff on G79W (616
  samples), 0.01 to 1.11 s on H128W (1,249), 0.39 to 1.24 s on G77R (950); the high-g axis tracks it at 1.008 to 1.016
  and the table uses it. Clipped, spliced or high-g alone (by offset), the top reads 190 to 211 m/s on G79W, 168 to 302
  on H128W, 172 to 215 on G77R: 108 to 420 m/s under the baro. The heights meet within 13 m by 4 to 6 s (17 m by 7 s on
  G77R, whose first read drifted to an apogee 1.71 times the baro's), where a top 124 to 317 m/s short would leave
  hundreds of meters; the pressure reads 139, 220 and 212 to 216 m high at the top speed.
- **Mostly the steady error.** The measured Cp gives half to nine tenths of the excess, Cp rising with speed perhaps the
  rest. Why the Mini DX3's Cp is −0.03 to −0.10 at its vents, 4.35 calibers aft of the shoulder (the Vulcanite's 3.9
  calibers read −0.006 to −0.030), where the tunnel gives the body at most +0.03 [T1] §6, is open (a leak at the screwed
  nose joint [F3]?). Its three flights count as "above" by sign; their size is suspect.
- **A small Cp does little at the top.** The quasi-steady lift −Cp·V·a/g, 12 m/s (8%) at Cp −0.007, 143 m/s and 12.3 g,
  holds only just before burnout; a 0.5 s window averages in the coast. Read so, −0.007 gives +1.2% on the H128W's own
  track (+1.3% synthetic); +8% needs about −0.02; −0.07 gives +51 to +57% ([P] §3: +12.8 to +99.9%).
- **So the Vulcanite H128W keeps 5 to 7 points unexplained:** a transient (D peaks at 12.4 m 0.1 s after the top, 7.5 m
  by 0.4 s) or the strapdown (16° tilt). Its sign holds: the baro's 151 to 152 m/s exceeds the axial speed, 143 to 147
  m/s by read, which caps the vertical. Standard air throughout; a warm day lifts the tops.

## 5. WE Rocketry: bay size or nothing, same sign

- **Hyperion II, Launch Canada, 2025-08-21** [W1] [W2]: a TeleMega and a Blue Raven in a 33 cm bay, vents not found.
  Accelerometer tops 283.9 and 273.1 m/s, baro 299.9 and 299.6 m/s: +6.6 and +10.6% against the fit, near Mach 0.82 (25
  °C pad assumed; ±5 K moves it ±1.7 points); a 180 m deficit at 3 s, Cp +0.045 to +0.055.
- **Prometheus, Spaceport America Cup, 2022-06-24** [W3] [W4]: accelerometer tops 317.9 and 326.0 m/s, baro 407.1 and
  404.6 m/s, +28.5 and +24.5%, near Mach 0.93, Cp about +0.064. The README blames "a delay on the order of 0.8s" [W3]; a
  lag can't overshoot (**Inference**). Transonic effects [T1] mix in; §6 leaves it out.

## 6. Does the sign split the low readings? (Inference)

Each low reader against two synthetic copies (`verify/v_split.py`): its mean boost (8.1 to 10.1 g), a coast at
1.7 to 3.0 g, its baro rate, and a quasi-steady port or a lagged bay of the same steady Cp. The lag is [V] §5's
linear law for a bay under cross flow (τ = C_eff·w/(2g), w the flight speed; steady Cp = C_eff), so like meets like.

| Flight | Climb Cp | Observed, vs fit | Pure port | Pure lag |
|---|---|---|---|---|
| pel. 1 | +0.026 to +0.028 | +2.1% | +3.5 to +6.1% | −1.6 to −1.8% |
| pel. 2 | +0.030 to +0.036 | +3.9% | +4.1 to +8.2% | −1.8 to −2.5% |
| pel. 4 | +0.037 to +0.043 | +9.5% | +5.3 to +10.2% | −2.6 to −3.4% |
| shake_n_bake | +0.042 to +0.046 | +6.6 to +7.6% | +6.3 to +11.7% | −3.7 to −4.7% |
| Hyperion II, TeleMega / Blue Raven | +0.045 to +0.055 | +6.6 / +10.6% | +10.1 to +14.8% | −3.4 to −5.9% |

- **The split, where it can be made, is port.** Every top is above; half sit inside the port's band; pel. 1, pel. 2 and
  the TeleMega sit 1.4, 0.2 and 3.5 points under it. Two loggers in one bay differ by 4 points, the read's noise: a lag
  share up to about 0.02 of Cp could hide (3.6 to 5.0 points per 0.02), not one carrying the reading.
- **What stays open.** Below a Cp of about 0.02 the reads differ by under 3.6 to 5.0 points, inside the noise. [T1]'s
  low readers (WVU +0.025, Piccard +0.045 and +0.13 to +0.15, Hedy +0.054 to +0.059) and the private set's 8 of 10 [T1]
  §4 await this read, which past Cp 0.05 splits by 10 points or more (Piccard's accelerometer failed a first try).

## 7. What the sample can't say

- **Thin.** Three airframes with documented vents, six with a usable top; one rocket gives three of six low readers.
  None lags visibly (C_eff ≤ 0.024); 38 mm bays at makers' rules (C_eff 0.024 to 0.11 [V] §5) are untested.
  **Unbounded:** NC State's fin gap and seal; pad temperatures; the CX5's filter [D1] p. 2; NC State's Cp spread (§3).

## What this suggests

- **#872.** Let the port term carry the logs' measured Cp on both signs: +0.026 to +0.064 with the top 2 to 28% high,
  −0.006 to −0.10 with it 7 to 165% high. Keep a vent term apart, low side only, from the bay's C_eff [V] (0 to 1.8% of
  the top here). *Expected effect:* each term answers to a measured cause; [P] §4's 0.02·v·a/g held simulated
  quasi-steady ports, and these are measured cases to test it on.
- **M2.3c2's trace RMS.** Traces read 36 to 94 m low at 2 s (NC State), 180 m at 3 s (Hyperion II), 270 to 284 m at 4 s
  (Prometheus), 10 to 220 m high at top speed (DOFPro). Suggest comparing the climb with the logger's accelerometer or
  GNSS, or beside a ±0.10·V²/(2g) band. *Expected effect:* an RMS that favors neither a slow nor a fast run.
- **Terms.** AirbrakesV2 (MIT) and WE Rocketry's post-flight-analysis data (CC BY 4.0, Prometheus's TeleMega too) may be
  cached; airbrakes-SR (no license), DOFPro ([F5]), reports and datasheet: cite only. No rows copied here.

## Open questions

- A low reader in a small or under-vented bay (C_eff 0.03 or more), the only kind whose lag would show at the top;
  the open area of NC State's slots past the retracted fins, and whether its partition seals.
- What sets the Mini DX3 bay's Cp of −0.03 to −0.10; the Vulcanite H128W's 5 to 7 unexplained points, and how tightly
  its axial speed bounds its vertical speed.

## Sources

Read 2026-11-05. Logs, pages and scripts cached under `refs/research/vented-bay-log-sign-test/`.
- [N1] NCSU HPRC, AirbrakesV2@afdcecb4, `launch_data/`, `metadata.json` (MIT):
  <https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2>
- [N3] NC State, 2025 NASA SL FRR, §3.3.5 and Fig. 3.22 (p. 9), §5.3 (p. 70):
  <https://ncsurocketry.org/wp-content/uploads/2025/03/north-carolina-state-university-2025-frr-report.pdf>
- [N5] NC State, 2022 NASA SL CDR, §4.4.3.5, p. 73:
  <https://ncsurocketry.org/wp-content/uploads/2022/01/north-carolina-state-university-2022-cdr-report.pdf>
- [D1] Parker MicroStrain, 3DM-CX5-VRU datasheet, doc. 8400-0115 Rev F, p. 2; the logged unit is a 3DM-CX5-15
  [N1], assumed to share its pressure sensor: <https://metromatics.com.au/wp-content/uploads/2024/04/3dm-cx5-AR.pdf>
- [W1] WE Rocketry, post-flight-analysis@7ba6ee2f, `data/` (code Apache-2.0, data CC BY 4.0), Prometheus's TeleMega
  in `data/2022-06-24_prometheus_sac22/raw/`: <https://github.com/werocketry/post-flight-analysis>
- [W2] WE Rocketry, Hyperion II `DSS.md` (MIT): <https://github.com/werocketry/Hyperion-II>
- [W3] WE Rocketry, airbrakes-SR@99d8a5c0, `flight_sims/Prometheus_flight_data/`, `flight_sims/README.md` (no license):
  <https://github.com/werocketry/airbrakes-SR>; [W4] wiki, `wiki/prometheus.md`: <https://github.com/werocketry/wiki>
- [G1] Aerospace Team Graz, RocketFlightData (no license): <https://github.com/AerospaceTeamGraz/RocketFlightData>
- [F1] DOFPro (Harvey Mudd), Flight Data Archive: <https://dofpro.org/RCK/fltdata/FlightData.html>; [F5] home page
  ("Creative Commons licenses", none named): <https://dofpro.org/>
- [F2] [F3] DOFPro, Standard Vulcanite and Mini DX3, "Holes for Altimeter Vents":
  <https://dofpro.org/RCK/const/standardVulcanite.html>, <https://dofpro.org/RCK/const/DX3.html>
- [F4] [F6] DOFPro `SimModels/` `.ork` files and 2021 Teensy logs (`Vulcanite/`, `First_Flight_DX3/`), objects in
  <https://dofprobucket2.s3.us-east-2.amazonaws.com/> (the root refuses a listing, 403)
- [F7] DOFPro, 38 mm, 54 mm and dual-deploy pages: <https://dofpro.org/RCK/const/38mmClassRocket.html>,
  <https://dofpro.org/RCK/const/54mmClassRocket.html>, <https://dofpro.org/RCK/const/vulcanite.html>
- [PF] B. Cawley Jr., Peak of Flight 543: <https://www.apogeerockets.com/education/downloads/Newsletter543.pdf>
- [P] [Altimeter port lag](altimeter-port-lag.md) §1 to §4; [V] [vent-hole discharge](vent-hole-discharge-crossflow.md)
  §5; [T1] [transonic dip in logs](transonic-dip-in-logs.md) In short, §1, §4, §6.
- [#872](https://github.com/nrdptel/fusionspace-eridanus/issues/872): the readings' top-speed bound and its port term.
