# How the CATS Vega turns pressure into its logged height

This note asks how the CATS Vega, the flight computer EuRoC 2023 required, turns pressure into the heights in its
logs and radio stream: its formula and constants, the pad zero, temperature, the filter, and what each file carries.
It serves the [real-flight comparisons](../accuracy.md#real-flights), where Cavour's, Genesis's and Lince's logs
are read as "barometric, assumed" ([report](../../validation/reports/real-flights.md)). It builds on
[altimeter-height-formulas](altimeter-height-formulas.md) (rule (a), the standard atmosphere zeroed at the pad),
[sample-logs-cats-juno](sample-logs-cats-juno.md) §1 (which file is which flight) and
[altimeter-smoothing-methods](altimeter-smoothing-methods.md) (the filter's form). Sources are the maker's manual
and paper, EuRoC's 2023 rules, and the published logs, read byte by byte; no CATS source code was opened.
**Inference** marks this note's reasoning and arithmetic. Read 2026-10-19.

**In short.** The Vega's height is rule (a) with its own constants, now checked against the flights' own pressure.
- **The formula is documented, and the files obey it.** The manual gives the troposphere's standard formula with a
  sea-level pressure of 101,250 Pa (not 101,325), exponent 1/5.257, 15 °C and −6.5 K/km, less a pad altitude
  calibrated before liftoff [C1]. No temperature enters it. On three EuRoC 2023 logs, the column RocketPy's tables
  carry is a 9-sample running median of that formula applied to the logged pressure, to 0.002 m on every in-flight
  row (§2). Against the 1976 standard's constants, at one pad pressure, it reads 0.19 m to 0.23 m low (−0.006%).
- **Genesis can be checked against its own pressure; Cavour can't.** Genesis's `main.cfl` holds 17,932 pressure
  samples at 100 Hz; with the fitted pad altitude, the formula reproduces its logged apogee, 2,916.69 m, to
  0.002 m. Cavour's only public record is radio telemetry: the Kalman height in whole meters, no pressure (§3, §4).
- **Cavour's logged apogee is an ejection bump.** Its highest packet, 2,789 m, came 0.2 s to 0.4 s after the apogee
  event, rising 8 m with the velocity turning positive; the coasting peak reads 2,785 m. On three sister flights the
  Kalman peak sits 1.1 m to 3.1 m below the barometric one, so Cavour's barometric apogee is 2,786 m to 2,789 m.
- **What moves.** Of the four assumed kinds, Genesis's and Lince's become barometric by file and Cavour's by the
  maker's document; Bella Lui's stays assumed. The 6.04% mean apogee error moves by at most +0.02 points (§5).

## 1. What the maker documents

The manual [C1] (last updated 2026-09-18) and the maker's paper [C2] describe one pipeline, in sections cited by name.
- **Sensors and rates.** An MS5607 barometer and an LSM6DSO32 IMU [C1] Table 1, read at 100 Hz [C2] p. 2. A
  preprocessing task turns raw pressure into height above ground; a Kalman filter then estimates height and
  velocity [C1] "Software Overview".
- **The formula.** h = ((p/p0)^(1/5.257) − 1) · (T0 + 273.15)/L − h0, with L = −0.0065, T0 = 15 °C, p0 = 101,250 Pa and
  h0 the calibrated altitude above sea level [C1] "Kalman Filter". The paper prints the exponent as 1/5.275 [C2] p. 3,
  eq. 7; the files fit 1/5.257 and miss by 5 m to 6 m on 1/5.275 even at their best window (§2), so the paper's is a
  misprint (**Inference**). The 1976 standard's exponent is 1/5.25588.
- **The pad zero.** During Calibrating and Ready the altitude above sea level is "continuously estimated", and fixed
  when liftoff is detected [C1] "Calibration of Sensors". The window is not documented (§2 bounds its effect).
- **Temperature.** The formula holds T0 at 15 °C. The manual describes no use of the measured temperature in the
  height; the logs record it (§3).
- **The filter.** States: height above ground, vertical velocity and an accelerometer offset; the measured vertical
  acceleration is the input and the formula's height the one measurement [C1] "Kalman Filter", [C2] pp. 2–3. The
  measurement noise R is R_initial on the pad and after apogee, R_max at liftoff, R_max · f(v) while coasting; Q
  is constant [C1] "Gain Scheduling". No value of Q, R or f(v) is published. Apogee is estimated velocity below 0
  for a set time [C1] Table 2. After apogee the filter ignores the IMU [C2] p. 3, so a pressure transient then
  moves the height.
- **Telemetry.** The flight data go to the radio chip at 10 Hz [C2] p. 2; at EuRoC 2023 an average 65.2% of packets
  arrived, 6.5 per second, on 18 computers flown on 13 rockets [C2] p. 5. The ground station's USB stream carries
  link, uptime (0.1 s), state, latitude, longitude, estimated altitude (m), estimated velocity (m/s) and battery
  voltage [C1] "Data Streaming via USB": no pressure.
- **The log.** The `.cfl` file holds "log elements" chosen by a bit mask, `rec_elements`, at a sampling period
  `rec_speed` [C1] Table 12; its layout is not documented. The Configurator plots pressure among other values and
  exports CSV [C1] "Flight Logs".

**EuRoC 2023 required one.** Every single-stage rocket had to fly an operational Vega "for official altitude
logging", and the competition apogee "will be determined from this device" [E2] §3.5 p. 21, on a firmware version
EuRoC mandated [E2] §3.5.5 p. 23. Teams report the apogee of that system; its telemetry apogee counts only if the
rocket is not recovered, or nothing onboard can be read [E1] §9.7.1 pp. 25–26.

## 2. The formula against the logs' own pressure

Three Vega logs of EuRoC 2023 are public with their `.cfl` and a RocketPy table [D1], [R1]: Camões (`20_red`),
Genesis (`11_faraday`) and Lince (`23_star`). All three name firmware `3.0.1-euroc` in their first bytes.

**Reading the bytes (Inference, from the data alone).** Each record is a 4-byte little-endian uptime in ms, a
4-byte one-bit type, and a payload. The types this note uses, by what their values match:

| Type | Payload | What it matches | Genesis count |
|---|---|---|--:|
| 0x20 | two int32: pressure (Pa), temperature (0.01 °C) | the 2025 export's `baro.csv` `P`, `T` [R1] | 17,932 |
| 0x40 | three float32: height, velocity, acceleration | the stats file's peak; the telemetry (§4) | 17,933 |
| 0x100 | two float32: filtered height, acceleration | RocketPy's table, every row (below) | 17,933 |
| 0x200 | uint32 flight state, 3 = Thrusting | the manual's state numbers [C1] | 5 |
| 0x1000 | two float32 and a byte: latitude, longitude, satellites | the site's coordinates | 246 |

IMU (0x10, six int16), events (0x400) and two unidentified types (0x80, 0x2000) complete the file. Records come
every 10 ms, so pressure is logged at 100 Hz. The tables match row for row: Genesis's 17,933 RocketPy rows,
Camões's 14,346 and Lince's 44,454 equal the 0x100 records exactly, on a clock zeroed at the Thrusting record.

**The filtered column is a median of the documented formula.** Fitting each 0x100 height as a running median of
the formula's heights, less one constant, over every row from 1 s after liftoff:

| Constants | Window | Largest miss, Genesis | Camões | Lince |
|---|---|--:|--:|--:|
| manual: 101,250 Pa, 1/5.257 | 9 samples, ending one before the newest | 0.002 m | 0.002 m | 0.002 m |
| 1976 standard: 101,325 Pa, 1/5.25588 | same | 0.114 m | 0.109 m | 0.118 m |
| paper's exponent: 101,250 Pa, 1/5.275 | 9 samples (best of 1 to 11) | 5.9 m | 5.3 m | 6.2 m |

- **The fitted constant is h0.** It is 97.1344 m on Camões and 91.8810 m on Lince, whose stats files give
  `Height_0` as 97.1343 m and 91.8809 m [D1]; Genesis, with no stats file, fits 106.3599 m.
- **The pad zero isn't the last 0.75 s.** h0 sits 0.04 m (Camões), 0.39 m (Genesis) and 0.48 m (Lince) above the
  formula's height of the mean pressure logged in the 0.75 s before liftoff, so the estimate uses an earlier or
  longer window. Its size, under 0.5 m, is 0.02% of a 3 km climb.
- **No temperature enters.** The fit holds while the board read 34.6–39.2 °C (Camões) and 39.3–44.6 °C (Genesis).
  A 2025 Configurator export [R1] (Hedy, a format sample) gives the same 9-sample median, largest miss 0.0023 m,
  across a board temperature of 8.3–44.7 °C, so the conversion outlives the 2023 firmware.
- **Against HPR Sim's reading.** Near each peak, from one pad pressure, the manual's formula reads 0.19 m
  (Genesis, Camões) and 0.23 m (Lince) below the 1976 standard's pressure altitude less the pad's, which is how
  [HPR Sim reads a barometric log](../../crates/hpr-validate/src/real_flight.rs): −0.006% (**Inference**). Less the
  Vega's own fitted h0 instead, the gap is 0.58 m (Genesis) and 0.70 m (Lince).

## 3. What each file carries

| File | Height | Pressure | Rate | Units |
|---|---|---|---|---|
| `.cfl` flight log | Kalman (0x40) and 9-sample median (0x100) | yes, with temperature | 100 Hz | Pa, 0.01 °C, m |
| Configurator CSV export | `height`; `filteredAltitudeAGL` | `baro.csv` `P`, `T` | 100 Hz | Pa, °C, m |
| Stats file (`*_stats.txt`) | the Kalman peak, `Height_0` | no | once | m |
| Ground-station CSV | Kalman, whole meters | no | up to 10 Hz | m, m/s |
| RocketPy tables | the 0x100 median | no | 100 Hz | m |

- **The stats peak is the Kalman's.** Camões's 3,014.8894 m and Lince's 3,608.6294 m equal the highest 0x40
  height, at the stated uptime [D1].
- **Telemetry is the Kalman state, truncated.** Every distinct Genesis packet (1,271 of 1,271) and Camões packet (679 of
  679; two links log most packets twice) carries the 0x40 height and velocity of the record 10 ms before its 0.1 s
  stamp, cut toward zero to whole units (**Inference**, by matching). So a telemetry height reads 0 m to 1 m below the
  state.
- **Exports from older firmware.** The 2021 data's README named the 9-sample median "filtered barometer data"
  ([sample logs](sample-logs-cats-juno.md) §1); the 2023 bytes confirm it is barometric, not the Kalman height.

## 4. Cavour and Genesis

**Genesis: checked.** Its `.cfl` [D1] is the source of RocketPy's table, which HPR Sim reads.
- The logged apogee, 2,916.69 m at 24.18 s after liftoff, is the 9-sample median of the formula's heights, before
  the apogee event at 24.37 s. The flight card's official 2,916 m [R1] is this value cut to whole meters, and the
  README's 2,914 m [D1] the Kalman peak, 2,914.71 m, cut the same way (**Inference**).
- Other readings of the same flight: the Kalman peak 1.98 m (0.07%) lower; the highest single pressure sample
  before the event 2.59 m higher; a transient 1.6 s after the event 6.08 m higher. HPR Sim's −5.85% would read
  −5.79% on the Kalman peak (**Inference**).

**Cavour: documented, not checkable.** `17_polito` holds two ground-station logs (670 and 570 rows, 516 distinct
stamps together) and no `.cfl` or stats file [D1]; the 13 flights in the maker's gallery on 2026-10-16 don't
include it ([sample logs](sample-logs-cats-juno.md) [S7]). The notebook says the flight was ballistic [R1], so the
board was most likely not read after landing, and the telemetry apogee became the official 2,789 m (**Inference**,
[E1] §9.7.1).
- **Its peak is after the apogee event.** In the merged logs, coasting packets reach 2,785 m at 23.1–23.3 s (on the
  table's clock), velocity 1 to 0 m/s, every 0.1 s. The first Drogue packet, at 23.6 s, reads 2,781 m at −5 m/s;
  the next two read 2,787 m at 0 m/s and 2,789 m at +2 m/s; then the height falls [D1]. **Inference:** an ejection
  pressure pulse reaching a filter that no longer uses the IMU, as on Lince, whose transient after the event lifted
  the Kalman height to 3,608.63 m, 24 m above its coasting 3,584.48 m (and the median column to 3,668.5 m).
- **Bounded.** The coasting Kalman peak is 2,785 m to 2,786 m (truncation). On Camões, Genesis and Lince the Kalman
  peak up to the apogee event is 1.11 m, 1.98 m and 3.13 m below the 9-sample median's, so Cavour's barometric
  apogee is about 2,786 m to 2,789 m (**Inference**, from three sister flights on the same firmware).

## 5. What this moves in the real-flight comparisons

- **Altimeter kinds.** Genesis and Lince: barometric, by file (the median of the documented conversion of their own
  pressure, §2). Cavour: barometric by the maker's document, a Kalman height within 3.1 m of the barometric one on
  three sister flights. Bella Lui stays assumed: one of seven, not four.
- **The reading rule holds.** HPR Sim reads its apogee as the standard atmosphere's pressure altitude less the
  pad's; the Vega's constants differ by 0.006%, far below these flights' 3.4% to 5.9% misses.
- **The numbers.** Cut at the Drogue event, Cavour's log apogee is 2,785 m and its error +5.78% (now +5.63%); at
  2,789.1 m, +5.62%. The mean absolute apogee error goes from 6.04% to 6.04–6.06%. With only Bella Lui read as
  a height, the alternative mean "assumed ones as heights" becomes 6.06% (6.08% with Cavour cut), not 6.63%
  (**Inference**, from the report's rows).

## What this suggests

- **Relabel three logs.** Genesis and Lince as `Barometric`, with evidence "the 9-sample median of the CATS
  standard-atmosphere conversion of the Vega's own pressure, to 0.002 m (`main.cfl`)"; Lince's computer named as
  the Vega (`23_star`, row-for-row match). Expected effect: no apogee changes; the assumed count goes from 4 to 1.
- **Cut Cavour at its apogee event** (between 23.4 s and 23.6 s on the table's clock), as Lince is cut. Expected effect:
  its error +5.63% to +5.78%, the mean +0.02 points; the record says the cut removes a post-ejection bump.
- **Cavour's record:** "Kalman height by radio, whole meters truncated; documented barometric measurement; no
  pressure public". An honest band is −4 m to −1 m on the cut 2,785 m (−3 m to 0 m on the uncut 2,789 m).
- **The `cats-vega-filtered-column` topic is mostly settled here:** the 2023 column is the barometric median, not
  the Kalman state, by file. A Configurator export of one `.cfl` would only confirm the record names.
- **A `.cfl` reader looks feasible clean-room** from this note's record table, tested against the RocketPy tables
  and stats files from the cache. Expected effect: three more barometric logs read from pressure, not a column.

## Open questions

- The pad-zero estimator's window, and what f(v), Q and R are (unpublished; under 0.5 m and 3.1 m here).
- Types 0x80 and 0x2000, and whether other firmware versions keep the record lengths.
- Whether the Kalman's 1.1 m to 3.1 m shortfall at apogee is filter lag or the median's noise picking a high
  value (the maximum of a noisy series sits above its mean); a flight with GNSS heights could tell.
- Cavour's own `.cfl`, if the team kept one. Halcyon (`01_astg`) has a `.cfl` and was not decoded here.

## Sources

The manual and EuRoC's documents state no license: cited, cached under the ignored `refs/research/cats-vega-height/`
only. The paper is CC BY 4.0. The EuRoC 2023 data are GPL-3.0 (the repository's license): cached, never committed;
this note publishes only numbers computed from them. All read 2026-10-19 unless marked.
- [C1] CATS User Manual, last updated 2026-09-18: Table 1, "Software Overview", "Calibration of Sensors", "Kalman
  Filter", "Gain Scheduling", "Data Streaming via USB", "Flight Logs", Tables 2 and 12. <https://catsystems.io/manual>
- [C2] Binz, Stojoski, Jost, "CATS: Empowering the next generation of rocket scientists through educational flight
  computers", 26th ESA PAC Symposium (2024), pp. 2, 3, 5; CC BY 4.0. <https://arxiv.org/abs/2511.04725>
- [E1] EuRoC *Rules & Requirements* v4.1 (2023-03-06), PTS_EDU_EuRoC_ST_000454, §9.7.1 pp. 25–26.
  <https://euroc.pt/wp-content/uploads/2023/03/PTS_EDU_EuRoC_ST_000454_RR_v4.1.pdf>
- [E2] EuRoC *Design, Test & Evaluation Guide* v4.1 (2023-03-06), PTS_EDU_EuRoC_ST_000455, §3.5 pp. 21–23.
  <https://euroc.pt/wp-content/uploads/2023/03/PTS_EDU_EuRoC_ST_000455_DTEG_v4.1.pdf>
- [D1] catsystems/euroc23-team-data @ `2dca527a`, GPL-3.0: `11_faraday/` (`main.cfl`, `gs_log_main_1.csv`, `README.md`),
  `17_polito/` (both `gs_log_main_1*.csv`, `README.md`), `20_red/` (also `gs_log_main_1.csv`) and `23_star/`
  (`main.cfl`, `main_stats.txt`). <https://github.com/catsystems/euroc23-team-data>
- [R1] RocketPy (MIT) @ `9bd6ad3a`: `docs/examples/{cavour,genesis}_flight_sim.ipynb` (first cell: team, official
  apogee, permission), `data/rockets/{genesis,camoes,lince,polito}/`, `data/rockets/hedy/cats_tust/` (no permission
  stated: aggregates only). <https://github.com/RocketPy-Team/RocketPy>
- The byte scan and fits are scripts in the cache, `refs/research/cats-vega-height/scripts/`.
