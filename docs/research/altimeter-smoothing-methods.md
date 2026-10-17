# Published smoothing and apogee methods for hobby altimeter data

This note asks what models and noise settings published rocket-altimeter Kalman filters use, how they treat the
transonic baro error and the ejection spike, what accuracy they report against a truth, how makers' documents
describe their onboard filters, and which offline smoother suits a log after the flight. It serves M7.2, whose
smoother must recover a synthetic truth inside 90% intervals that pass ADR-257's coverage test. It builds on
[sensor noise](altimeter-sensor-noise.md), [Mach error](altimeter-mach-error.md) and [CATS
logs](sample-logs-cats-juno.md). Sources are papers, team reports, makers' manuals (documents only), NASA reports,
PX4's docs and a textbook. **Inference** marks this note's reasoning, arithmetic and probe (§6). Read 2026-10-17.

**In short.** The published filters are small and alike, they handle Mach 1 by schedule, and none is checked
against an independent truth on a hobby flight. A linear RTS smoother suits a log, if its model is honest.
- **Two or three vertical states.** Height and speed, plus acceleration or an accelerometer offset; the
  accelerometer is usually an input (§1). Only [R1] gives its filter's Q and R; none bounds what tuning does.
- **Mach 1 by schedule, not statistics.** Lower the baro's weight while the motor burns or the rocket is fast
  (Altus Metrum, CATS), lock it out over a Mach band, or vote (§3). One source gates innovations with χ² [R10].
- **No accuracy against a truth.** Results are simulated (apogee timing 0.269 s over 200 flights [R1]; 1.8 m RMS at
  apogee [R10]); makers claim only timing (±0.2 s [PF1]). No source bounds the ejection spike (§3, §4).
- **The smoother's intervals are honest only if the model is (§6, probe).** AR(1) baro noise treated as white holds
  the truth 70% of the time, not 90%; a 15 m baro error left in for 2 s holds 74% at apogee, missing on the low side.

## 1. Published designs: states, noise and rates

| Source | States | Accel as | Q, R given? | Rates (Hz) |
|---|---|---|---|---|
| [R1] Rice, one flight and a simulation | h, v, a | measurement | yes (below) | 20 |
| [R2] JKUAT, BMP180 and MPU6050 | h, v, a | measurement | grid-searched, no values | 25 (**Inference**) |
| [R3] ICAS 2022, four flights | h, v; EKF h, v, a | input (KF) | R from datasheet, Q arbitrary | about 100 |
| [C2] CATS Vega paper | h, v, accel offset | input | form only, R scheduled | 100 |
| [R9] IREC 2025 abstracts | h, v, a | input (one team) | from datasheets | IMU 200–500, baro 50–80 |
| [R7] USC Aftershock II, UKF | 3-D, with attitude | measurement | form only | IMU 200, baro 10, GPS 1 |
| [R10] a builder's error-state EKF | 25 errors, baro offset | IMU driven | sensor errors | IMU 200, baro 50, GPS 10 |

- **The one design with numbers [R1] pp. 11–13:** constant acceleration, R = diag(35.8229, 0.0012), Q = diag(0, 0,
  1), Δt = 0.05 s, units not stated. **Inference:** 35.8229 ≈ 5.985² and 0.0012 ≈ 0.0346², the noise it fits to its
  flight (p. 19), so R is in m² and (m/s²)²; its table calls 5.985 a variance, its code a standard deviation (p. 20).
- **Accelerometer as input is the common choice** [R3] p. 9, [C2] pp. 2–3. Its cost: the accelerometer's error is left
  out of the filter's error estimate [R3] p. 10. [R3] calls results "relatively insensitive" to Q and R (p. 9),
  with no range; [R2] fits Q by grid search (p. 59). CATS's offset is the only bias state among onboard designs.

## 2. What makers' documents describe

| Maker | Estimator described | Logged rate | Altitude column |
|---|---|---|---|
| Altus Metrum | Kalman: height, speed, acceleration [A1] p. 66 | 100 Hz up, 10 down [A1] p. 77 | baro height |
| CATS Vega | Kalman: height, velocity, accel offset [C1] | 100 Hz sensors [C2] p. 2 | not named |
| Entacore AIM XTRA | "Kalman", no model [EN] p. 3 | set per sensor [EN] p. 20 | baro; Kalman view |
| Featherweight Blue Raven | low-pass baro, 2-of-3 vote [F2] p. 15 | 50 and 500 Hz sets | pressure; inertial |
| Eggtimer Quantum, Proton, TRS | moving average [E3] p. 31 | 20, up to 33 Hz [E1] p. 25 | `Alt` raw, `FAlt` |
| Missile Works RRC3 | average, then 21-element FIR [MW1] p. 24 | 20 Hz | pressure |
| PerfectFlite StratoLoggerCF | "smoothed velocity" [PF1] p. 2 | 20 Hz [PF1] p. 47 | not stated |
| Jolly Logic AltimeterTwo | none described [J1] | not stated | none logged |

- **Altus Metrum** calls apogee at the speed's zero crossing; above 30 km the accelerometer alone updates the state,
  which under-estimates apogee by an unstated amount [A1] pp. 66–67. Since release 1.8 its ground software filters
  "both forward and backwards in time" [A1] p. 111, the only maker's offline smoother found. Its CSV gives baro
  height, its telemetry the Kalman values [A1] p. 37, [A2] pp. 2–4.
- **CATS** feeds acceleration as the input and baro height as the one measurement; R is a scalar set by flight
  state and, in coast, by a function of velocity [C1] "Gain Scheduling", [C2] pp. 2–3; apogee is estimated velocity
  below 0 for 0.3 s [C1]. No value of Q, R or the function is published. **Inference:** a maker's filtered column
  is an observation with its own lag, never the truth; only Eggtimer names raw and filtered columns [E1] p. 38.

## 3. The transonic baro error and the ejection spike

- **De-weight by phase.** Altus Metrum's filter is made to "de-sense" baro near or above Mach [A1] p. 67; CATS
  schedules R through thrust, "more robust against transonic waves" [C2] p. 3, and in coast by velocity [C1]; SOAR
  cut the baro's weight for supersonic flight [R6] pp. 12–13. None reports what the de-weighting buys.
- **Lock out a Mach band and gate.** One builder's simulated design drops baro for 0.7 < M < 1.3 by its own Mach
  estimate and passes every update through a 99.99% χ² gate; with neither, the filter diverged to 2,260 m RMS
  apogee error on its simulated truth [R10] (not reviewed). The MIT notes model the error, only above Mach 1 [R5]
  p. 5; PX4 models static-pressure error as a fraction of dynamic pressure [X1].
- **Makers' rules.** Eggtimer locks out until baro velocity stays under 100 ft/s (30.5 m/s) for 1 s [E3] p. 31; the
  RRC3 discards anomalous velocity changes [MW1] p. 24; PerfectFlite's MachLock threshold is unpublished [PF1] p. 38.
- **Size.** Aftershock II's baro read "abnormally high" on ascent, trusted only below 100,000 ft [R7] pp. 13, 18;
  [Mach error](altimeter-mach-error.md) gives tunnel sizes: hundreds of meters, either sign.
- **Ejection.** No academic filter read models the charge's pressure pulse or the jerk at line stretch. CATS logs an
  acceleration bump 0.35 s before its apogee event, put down to a pyro charge [C2] p. 6. Altus Metrum's ground
  software drops the spike before taking maxima [A1] p. 128; Featherweight's "Apo channel fired" goes true 1.5 s
  after the charge [F2] p. 15; Entacore and Featherweight say to seal the bay [EN] p. 45, [F2] p. 8.
  **Inference:** a charge is a logged event, so a smoother can drop baro for a window after it.

## 4. Accuracy against a truth

| Source | Truth | Quantity | Result |
|---|---|---|---|
| [R1] p. 26 | simulated, 200 randomized flights | apogee-detection time | Kalman 0.269 s; boxcar 1.52 s |
| [R10] | 6-DOF simulation, 200 flights | 3-D position at a 9.1 km apogee | 1.8 m RMS; 99.7% inside ±3σ |
| [R7] pp. 18–19 | 1,024-trial Monte Carlo, reentry fit | apogee | 470,400 ft ± 9,100 ft (1σ) |
| [R11] pp. 11–12 | reference INS with GPS | down position, GPS-aided MEMS INS | 16.8 m (3σ, filter's own) |

- **Inference on [R1]:** a timing error Δt at apogee costs g·Δt²/2 of height; at the mean errors, 0.35 m for the
  Kalman filter, 11.3 m for the boxcar (the mean cost is higher). Its accelerometer bias made speed cross zero
  about 1 s early (p. 25).
- **[R10] alone checks consistency:** mean NEES 2.91, 3.04 and 2.96 for position, velocity and attitude against 3
  expected. On a sounding rocket, GPS lost lock or overshot by several hundred meters for up to 10 s at liftoff and
  burnout [R11] p. 5. No maker reports error against a truth; CATS counts 18 units on 13 rockets [C2] p. 5.

## 5. An offline smoother and when its intervals hold

- **RTS.** After a forward Kalman filter (m_k, P_k): G_k = P_k Aᵀ(A P_k Aᵀ + Q)⁻¹, m_k^s = m_k + G_k(m_{k+1}^s −
  A m_k), P_k^s = P_k + G_k(P_{k+1}^s − P_{k+1}⁻)G_kᵀ, run backward [B1] Thm 12.2, eq. 12.6, pp. 255–256 (after
  [B2]). P_k^s is exact only for a linear-Gaussian model with white noise and the right Q and R [B1] eq. 12.5.
- **NASA practice** prefers the Fraser–Potter forward-backward form to RTS for numerical robustness; only states
  driven by process noise gain from smoothing, so a constant bias gains nothing [N1] p. 113. It edits measurements by
  the squared Mahalanobis innovation against χ², flagged accept, inhibit or force per type [N1] p. 101.
- **Process noise.** For [h, v] with white-noise acceleration of density q_c, Q(Δt) = q_c[[Δt³/3, Δt²/2], [Δt²/2,
  Δt]], exact for any Δt [B1] eq. 4.18, p. 51, so irregular rates and gaps need nothing more; for [h, v, a] a
  white-jerk form [B5] eq. 20, p. 16. **Inference:** with the accelerometer as input [B1] §5.3, q_c is its noise
  density; while the accelerometer is clipped there is no input; carry acceleration as a white-jerk state.

## 6. What breaks a 90% interval: a probe

**Inference throughout.** An invented 1-D flight: 130 m/s² thrust for 2 s, gravity, quadratic drag, apogee 856.8 m
at 11.70 s, 195.9 m/s at burnout; 50 Hz baro, σ 0.5 m; accelerometer white 0.43 m/s² plus a per-flight bias σ 0.3
m/s²; state [h, v, accel bias, (AR(1) baro error)], accelerometer as input; 4,000 flights a case (gated: 400). The
φ = 0.8 row has no bias or bias state. With the model right the apogee interval held 90.2%. Share held:

| Case | Apogee | Burnout speed |
|---|---|---|
| AR(1) baro noise, φ = 0.5, smoothed as white | 70.4% | 81.9% |
| φ = 0.8, smoothed as white | 46.0% | 64.5% |
| AR(1) state added (φ = 0.5) | 89.5% | 92.1% |
| white model with R × (1+φ)/(1−φ) = 3 (φ = 0.5) | 89.1% | 92.2% |
| +15 m baro error on 1–3 s, left in | 74.0% (truth above: 25.6%) | 0.1% |
| same, R inflated to (15 m)² there, or samples dropped | 90.4–90.5% | 91.4–92.9% |
| same as a step, 3σ gate (innovation statistic > 9) | 91.7% | 94.5% |
| same as a ramp to 15 m, 3σ gate (18% of samples leak) | 26.3% (truth below: 71.3%) | 22.8% |

- **What the rows say.** They confirm [sensor noise](altimeter-sensor-noise.md)'s √3 ≈ 1.7 factor on a smoothed
  reading. A bad region left in reads apogee low, the flattering side for a waiver; a leaked ramp reads it high. A
  gate catches a jump but not an error that grows with Mach, so windows should come from physics (Mach, events).
- **Bias and scale.** An unmodeled 0.3 m/s² bias did no harm while baro was trusted; with baro weighted as σ = 100 m
  the apogee interval held 16.4%. White noise alone gives a 4–6 cm σ at apogee: real intervals are set by
  per-flight systematic terms (calibration, temperature, atmosphere model), which a test must draw per flight.

## 7. Heavy tails, tuning and consistency

- **Student-t smoothing.** With 10% of samples gross outliers, a Gaussian smoother's median MSE rose from 0.04 to
  1.3 while Student-t and ℓ1 smoothers stayed at 0.04–0.05 [B8] Table 6.1, p. 15: point error, not coverage.
- **Tuning from the data.** R, φ and a GNSS delay by the innovation likelihood of one forward pass [B1] Thm 16.9,
  p. 332, or by EM with an RTS E-step [B1] Alg. 16.13, p. 336 (after [B3]); a per-flight bias's prior comes from a
  datasheet, not the flight. NEES = eᵀP⁻¹e is χ² with n_x degrees of freedom for a consistent filter, NIS with n_z;
  above the Monte Carlo χ² band means overconfident [B6] §II, citing [B7].
- **A health card's style.** PX4 shows, per sensor, an innovation test ratio against an N-sigma gate (above 1.0
  rejected, under 0.5 in normal flight), pass/fail and fault flags, and reset counts; defaults are a 5 SD baro
  gate and 3.5 m baro noise [X1], [X2], [X3]; ArduPilot shows normalized variances, 0 to 1 [X4].

## What this suggests

1. **M7.2's smoother:** a linear RTS over [h, v, accelerometer bias, AR(1) baro error], accelerometer as input, Q
   built per step from Δt; acceleration as a white-jerk state only over clipped spans (§5). Expected: the probe's
   apogee coverage goes from 70.4% (white model) to 89.5%.
2. **Bad regions from physics:** drop or inflate baro over a smoothed-Mach window (0.7–1.3 a first default to test
   [R10]) and after each recorded charge, a χ² gate per sensor as a second net, each rejected sample listed (§3,
   §6); the same edit for GNSS near liftoff and burnout [R11].
3. **The synthetic test must draw** correlated baro noise and per-flight biases, as [sensor
   noise](altimeter-sensor-noise.md) says; a white-noise generator passes a model that fails on real logs (§6).
4. **Report the published yardsticks** on 200 or more randomized flights: apogee-time and apogee-height error,
   90% coverage per tail, mean NEES ([R1], [R10]). No published real-log figure exists to compare against.
5. **A health card per log** (§7): per sensor the share inside its gate, the largest test ratio with its time and
   Mach, the de-weighted spans, the baro-minus-inertial gap at apogee, and mean NIS against 1.

## Open questions

- No source bounds a filtered apogee's error on a hobby log against GPS, radar or a theodolite. How fast does a real
  transonic baro error rise? It decides whether a gate catches it (§6). A 3-D flight with GNSS was not probed.
- Not read: the RTS paper and Bar-Shalom's book (paywalled; cited through [B1] and [B6]), Zarchan and Musoff past
  their chapter openings [B4], Utah State's 2009 airbrake EKF (fetches refused), Schultz's NAR R&D reports (members
  only). No claim here rests on them. Unpublished: CATS's Q, R and velocity function, Altus Metrum's noise values
  (in GPL firmware, not read), Featherweight's and Eggtimer's filter time constants.

## Sources

Read 2026-10-17. Sources without a license, [R3] and [R11] (authors' copyright), [B1] (personal use) and arXiv
papers [B5], [B6], [B8] are cited and cached privately, not copied; [X4] is CC BY-SA 3.0.
- [R1] Dougal, Kwok, Luckett, *Digital Detection of Rocket Apogee*, Rice/Connexions (2013), CC BY 3.0.
  <https://repository.rice.edu/server/api/core/bitstreams/0fbefe49-d831-4891-b83f-d8c2f05ce2f9/content>
- [R2] Kimani, Osodo, Aoki, JKUAT SRI conference (2021), pp. 57–60.
  <https://sri.jkuat.ac.ke/jkuatsri/index.php/sri/article/download/274/257/1033>
- [R3] Kierulf, ICAS 2022-0414 <https://www.icas.org/icas_archive/ICAS2022/data/papers/ICAS2022_0414_paper.pdf>;
  [R5] Govedic, Miller, "Rocket Kalman filter formulas", MIT Rocket Team (2019)
- [R6] SOAR Atlantis II, IREC 2018 team 43; [R9] ESRA, 2025 IREC podium abstracts <https://www.soundingrocket.org/>;
  [R7] USC RPL, *Aftershock II Apogee Analysis* (2024-11) <https://www.uscrpl.com/>
- [R10] RocketNavigationEKF README, no license <https://github.com/momokazmi1017/RocketNavigationEKF>; [R11] Braun,
  Barf, Markgraf, EUCASS 2019-0241 <https://www.eucass.eu/doi/EUCASS2019-0241.pdf>
- [N1] Carpenter, D'Souza, *Navigation Filter Best Practices* (2025) <https://ntrs.nasa.gov/citations/20250002787>
- [A1] Altus Metrum manual v1.9.22.2, CC BY-SA 3.0 <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>; [A2] AltOS
  telemetry <https://altusmetrum.org/AltOS/doc/telemetry.pdf>
- [C1] CATS manual <https://catsystems.io/manual>; [C2] Binz, Stojoski, Jost, ESA PAC Symposium (2024), CC BY 4.0
  <https://arxiv.org/abs/2511.04725>; [EN] Entacore AIM XTRA manual v7; [J1] Jolly Logic AltimeterTwo guide
- [F2] Featherweight Blue Raven manual (2026); [E1] Eggtimer Quantum 1.09G, [E3] TRS 2.07 guides; [MW1] Missile
  Works RRC3 manual v1.71; [PF1] PerfectFlite StratoLoggerCF manual
- [X1] PX4, "Using PX4's Navigation Filter (EKF2)", [X2] parameter reference (CC BY 4.0), [X3] `EstimatorStatus.msg`
  (field names only) <https://docs.px4.io/main/en/advanced_config/tuning_the_ecl_ekf.html>; [X4] ArduPilot, "EKF
  Failsafe" <https://ardupilot.org/copter/docs/common-ekf-inav-failsafe.html>
- [B1] Särkkä, Svensson, *Bayesian Filtering and Smoothing*, 2nd ed. (2023)
  <https://users.aalto.fi/~ssarkka/pub/bfs_book_2023_online.pdf>; [B2] Rauch, Tung, Striebel, AIAA J. 3(8),
  1445–1450 (1965); [B3] Shumway, Stoffer, J. Time Series Anal. 3(4) (1982); [B4] Zarchan, Musoff (2005)
- [B5] Pulford, arXiv:1503.07828; [B6] Chen et al., "Weak in the NEES?", arXiv:1807.08855; [B7] Bar-Shalom, Li,
  Kirubarajan (2001); [B8] Aravkin, Burke, Pillonetto, arXiv:1303.5588
