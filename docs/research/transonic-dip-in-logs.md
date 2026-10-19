# The transonic baro error measured in flight logs

This note measures how far a barometric altimeter misreads between Mach 0.8 and 1.3 on real flights, against an
independent altitude: the logger's own accelerometer, integrated, or a GPS. It sizes the error's sign, its size,
the Mach where it changes and how long it lasts, for M7.4's "Mach baro error" fault, M7.2's flag threshold and
M6.8's lockout check. It builds on [altimeter Mach error](altimeter-mach-error.md), which sizes the same error
from tunnel and 1950s free-flight data only. Logs were read and measured on 2026-10-19 with plain Python under the
ignored `refs/`; no logger's code was opened. Mach is the independent speed over the 1976 standard atmosphere's
speed of sound; a day 10 K warmer than standard lowers each Mach by 1.7%. **Inference** marks this note's analysis.

**In short.** Six public log sets on five flights size the error through Mach 0.9 against an independent track (a
sixth flight's GPS alignment leaves it open), and ten private flights add counts. The error shows on all of them,
with both signs, and is mostly smaller than the tunnel's.
- **Two kinds of error.** A steady one grows with speed squared and reads low on most flights (Cp +0.005 to
  +0.15 rising), high on a few (Cp −0.02 to −0.13 rising); single points reach −0.21 and +0.28. On top of it comes
  a step near Mach 1: 20 to 490 m, mostly within 0.04 to 0.2 s, some over 0.45 s, at Mach 0.88 to 1.30, most
  often 0.93 to 1.07 (**Inference**, §1 to §4).
- **Sizes on the public logs.** A forum TeleMega read +310 to +400 m high at Mach 0.9 to 1.0 and +961 m at Mach
  1.18. AerospaceNU's 2022-06-11 flight read within 30 m except for a 30 to 50 m dip, then a 90 to 95 m jump in
  0.04 to 0.06 s, at Mach 0.94 to 0.97. Hedy read 256 to 303 m low, then jumped 206 m at Mach 1.00 to 1.02 and
  fell 226 m at Mach 0.99 coming back down (§2).
- **Private collection (aggregates only).** Of 45 candidate logs, 12 on 10 flights pass this note's rules. 8 of
  10 flights read low. The steady error at Mach 0.9 has a median of about 80 m (14 to 882 m), the largest 0.1 s
  step a median of about 130 m (19 to 488 m). Time above Mach 0.9 is 0.4 to 12.6 s, median 1.3 s (§4).
- **A low reading in climb can't be told from vent lag (Inference).** Both grow as V²; only a high reading or a
  fast step is surely the port's aerodynamics (§1).
- **False descents come while slowing.** Four public flights show the baro height falling while the rocket still
  climbs, 11 to 276 m, as it slows through Mach 1.14 to 0.93; each ended before the falling Mach 0.9 crossing (§5).
- **No log documents its port's position**, so the tunnel's port-by-calibers table can't be checked yet (§6).

## 1. How each log was measured (Inference)

- **The independent height.** The axial acceleration less its pad reading is integrated twice from the pad, times
  the cosine of tilt where the log gives it. Blue Raven logs give their own inertial vertical and total speed. A
  scale k is fitted where the inertial speed is under 60 m/s and the height above half of apogee, and the baro
  height is compared with (1 + k) times the inertial height. k absorbs the day's air temperature and the
  accelerometer's scale; logs with |k| over 12% were dropped. The departure D is baro minus scaled inertial height.
  Mach comes from the unscaled inertial speed, so it carries the accelerometer's share of k as well as the day's
  temperature (1.7% per 10 K): up to 12% in Mach under these rules, most logs much less.
- **A pressure coefficient from D.** Hydrostatics gives D = −Cp·V²/(2g): the tunnel note's −0.7·M²·Cp·H,
  since 0.7·M²·H equals V²/(2g), with no pressure or temperature left in it. A port reading low has Cp > 0.
- **Vent lag looks the same.** The tunnel note's orifice lag reads low by u²/(2g), with the vent speed u
  proportional to the climb rate. In a steady, near-vertical climb that also grows as V², so a low steady reading
  is port pressure, vent lag or both. A high reading on ascent can only be aerodynamic.
- **Steps.** The step is the largest change in D within 0.1 s above Mach 0.85. An integrated track drifts well
  under a meter in 0.1 s, so the step is robust where D's absolute level is not (±2 to 5% of height).

## 2. The public logs

| Flight | Logger, rate | Top Mach | D at Mach 0.9 to 1.0 (Cp) | Step near Mach 1 | Above 0.9 |
|---|---|---|---|---|---|
| Forum A, 2018, J270W [L1] | TeleMega, 100 Hz | 1.20 | +314 to +404 m (−0.07) | none, a smooth rise | 2.0 s |
| AerospaceNU, 2022-06-11 [L2] | FCB, EasyMini | 1.01 to 1.04 | within ±30 m | −30 to −48, then +94 m | 4 s |
| Hedy, EuRoC 2025 [L3] | CATS, 100 Hz | 1.09 | −256 to −303 m (+0.055) | −58, +206 m; down −226 m | 4.5 s |
| WVU, IREC 2026 [L4] | Blue Raven, 50 Hz | 1.50 | −110 to −151 m (+0.025) | −49 m at Mach 1.38 | ≥ 3.5 s |
| Piccard body, 2021 [L5] | TeleMega, CATS | 1.06 | −670 to −758 m (+0.14) | down +230, then −230 m | 6.8 s |
| Piccard nose, 2021 [L5] | TeleMetrum, 100 Hz | 1.09 | −194 to −276 m (+0.045) | none over 16 m | similar |
| UTRGV, 2026-03-29 [L6] | team board, GPS | 0.99 | −76 to +15 m, by alignment | not sized | 1.4 s |

- **Forum A** (a LOC kit on a J270W; the cached TeleMega export [D1] holds a call sign and pad position, never
  quoted). The baro reads high from launch: Cp −0.064 to −0.072 rising through Mach 0.3 to 1.0, −0.10 at Mach 1.1
  to 1.2, −0.08 to −0.11 slowing back. D is +68 m at Mach 0.3 to 0.5, +314 m at Mach 0.9 rising, a peak of +961 m
  at Mach 1.18 (2.20 s), and +384 m at Mach 0.9 falling (3.26 s). The GPS apogee is 2.2% above the inertial height
  at the baro's apogee time, 0.7% above the inertial peak. **Inference:** a port in suction, as at the tunnel's
  shoulder (Cp −0.08 at Mach 0.9 there). The 276 m dip the [anomalous flights](anomalous-public-flights.md) note
  reported at 2.4 to 3.1 s is this high reading unwinding.
- **AerospaceNU 2022-06-11.** The FCB's IMU height ran 12 to 16% above its GPS, so its acceleration was scaled
  by 0.86 to 0.88 to match the GPS at 10.7 s and at apogee (**Inference**). Rising, D holds near +28 m until Mach
  0.94 to 0.97, falls 32 m in 0.15 s, jumps 88 to 94 m in 0.06 s, then settles 22 m above where it began. The
  EasyMini, on its own clock 0.85 s apart, shows the same: a plateau 48 m below its trend, then a 90 to 95 m jump
  in 0.04 to 0.06 s (its baro rises 111 m in 0.06 s at 4.85 s, 19 m of it climb at 321 m/s). Slowing through Mach
  0.96 to 0.93, D drops 40 to 50 m, overshoots 60 m and returns. **Inference:** the tunnel's aft-port sequence (a
  dip as the shock forms ahead, a jump as it passes aft) at about a quarter of its size.
- **Hedy** (TU Wien Space Team; a liquid-fueled rocket, outside the project's motor scope, but a port's reading
  depends on the airframe). The notebook gives a 132.8 mm body, 3,707 mm long, with a 561 mm LV-Haack nose, and an
  84° rail [L3]. Times count from its first sample above 3 g (file time + 0.106 s). Cp is +0.054 to +0.059 at Mach
  0.85 to 0.99. Rising, D sinks 58 m over Mach 0.96 to 1.00, jumps 206 m in 0.21 s (Mach 1.00 to 1.02), then
  settles at Cp +0.043 by Mach 1.05, 38 m above its level before the dip. Slowing, D climbs 104 m from Mach 1.04
  to 1.00, then falls 226 m in 0.11 s at Mach 0.99.
- **WVU** (Shred-O-Vision's example flight, which broke up at about 6.0 to 6.5 s). The device's inertial speed
  and two integrations of its high-rate file agree within 3% (495 against 503 and 508 m/s). Cp stays +0.020 to
  +0.030 from Mach 0.3 to 1.5 with no step at Mach 1; bumps of +16 m (Mach 1.13) and −49 m (Mach 1.38). With no
  apogee, k is unknown; a 7% temperature scale would be about −30 m of D. Its README gives 1,602 ft/s, Mach 1.39
  at 6.28 s [L4]; **Inference:** the lower Mach is a warmer speed of sound than the 1976 table's.
- **Piccard** (ARIS, EuRoC 2021). Three loggers on one flight; the TeleMega's GPS apogee is 6,188 m, near the
  6,200 m the CATS README states [L5]. The TeleMega and the CATS board's three baros agree within about 45 m. The
  TeleMega's baro falls 93 m below its early reading from 1.2 to 3.6 s while the rocket climbs from 67 to 598 m.
  Against GPS the main body reads 458 m (12.6%) low at Mach 0.76 in coast, and 3.8% low at the GPS apogee's time
  (3.6% at its own peak, 4 s later). Slowing through Mach 1.00 to 0.97, D rises 230 m in 0.45 s, then falls back.
  The nose-cone TeleMetrum reads smoothly, Cp +0.04 to +0.05 rising, no 0.1 s change over 16 m. **Inference:** the
  main body's Cp of +0.13 to +0.15 rising (more slowing) exceeds every body station in the tunnel (+0.03 at most);
  a port ahead of a fin (in free flight +0.075 to +0.12 of impact pressure, about +0.10 to +0.15 in Cp at Mach 1)
  or vent lag could give it, and this log can't tell them apart.
- **UTRGV** (Lonestar launch). Its Blue Raven export repeats a 3.92 s block (196 rows) about 16 times and peaks
  at 868 ft against a 9,543 ft summary, so it was not used. The team's board logs pressure at about 47 Hz; its main
  IMU fails (top speed 157 m/s). Against the 10 Hz Featherweight GPS, D hangs on the clock alignment (25 to 40 m
  per 0.1 s): the best coast fit (RMS 1.1 to 1.35 m) puts the baro 50 to 76 m low at Mach 0.72 to 0.99 rising; a
  looser one (RMS 18 m), within 15 m rising and 44 to 55 m high slowing. Neither is used. The auxiliary IMU, at
  330 m/s against the GPS's 334 m/s, is an independent track not yet tried.

## 3. Logs that cross Mach 0.9 but can't size it

- **UND, IREC 2024** [L7]: control was lost under thrust, and the primary log ends at 1.88 s, at Mach 0.81.
- **Forum B, 2014** [L8], TeleMetrum telemetry: 10 Hz packets, none from 0 to 0.57 s, 3.47 to 5.59 s or 8.2 to
  23.9 s. Decoded by the published packet layout [D2], the raw accelerometer gives 18 counts per g, too coarse to
  integrate. The board's speed (300 m/s at 0.97 s, 559 m/s at 3.47 s) comes from a filter using the baro.
- **Piccard's CATS IMU** starts with the rocket moving (1.15 to 1.17 g, rising): no rest reading. Less 1 g, its
  three IMUs give apogees of 6,364 to 6,411 m, 3 to 4% above the GPS: usable, but not used here.

## 4. The private collection, in aggregate

The private catalog lists 45 logs from loggers with an independent speed (Blue Raven, Altus Metrum with an
accelerometer, Featherweight GPS) whose catalog peak speed is at least 250 m/s, counted on 2026-10-19. Rules: a
top Mach of at least 0.9 by the inertial track, at least 20 samples a second, an apogee fit with |k| ≤ 12%.
- **Kept:** 12 logs on 10 flights. Two flights carry two loggers; each flight counts once below.
- **Dropped:** 14 GPS-only logs (not yet paired with a baro), 7 below Mach 0.9, 4 telemetry logs under 20 Hz or
  too short, 7 with |k| over 12% or no apogee fit, 1 that did not parse.
- **Sign.** 8 of 10 read low (Cp +0.005 to +0.084); 2 read high (Cp −0.023 and −0.125, reaching −0.21 at Mach 0.92).
- **Steady size at the Mach 0.9 up-crossing:** |D| 14, 26, 32, 61, 71, 89, 257, 281, 417 and 882 m.
- **Steps:** 6 fall (the reading drops), 4 rise; |step| 19 to 488 m, median about 130 m; 7 of 10 at Mach 0.94 to
  1.07, the rest at 0.88, 1.15 and 1.30. Two Blue Ravens on one flight agree: Cp +0.010 and +0.011, steps of
  −138 and −174 m. One flight reads far off while slowing (Cp up to +0.28), with its −488 m step at Mach 0.99.
- **Time above Mach 0.9:** 0.4 to 12.6 s, median 1.3 s.

## 5. False descents, the case for a lockout

In four public logs the baro height falls while the inertial height still rises, all while slowing:
- Forum A: 1,507 m to 1,231 m from 2.37 to 3.10 s, Mach 1.14 to 0.93, 276 m.
- Hedy: 1,808 m to 1,620 m from 9.14 to 9.25 s, Mach 0.995 to 0.985, 188 m.
- Piccard's main body (TeleMega): about 1,450 m to 1,404 m from 8.61 to 9.04 s, Mach 0.99 to 0.97, 46 to 48 m.
- AerospaceNU's FCB: 11 and 21 m from 8.78 to 8.94 s, 28 m (35 on its second baro) at 9.09 to 9.12 s, Mach 0.96
  to 0.93.

Hedy also dips 18 m at Mach 1.02 to 1.04 while still speeding up. **Inference:** a baro-only apogee test would
take each as apogee unless locked out. They lasted 0.1 to 0.7 s; each ended before the falling Mach 0.9 crossing,
by 0.15 s on Forum A (crossing at 3.25 s), 0.5 s on AerospaceNU (9.6 s), 1.0 s on Hedy (10.25 s) and 1.5 s on
Piccard (10.5 s). Forum A's ended at Mach 0.93: little margin, given Mach's uncertainty (§1).

## 6. Against the tunnel sizes (Inference)

- **Steady error.** The tunnel note's ogive-cylinder gives Cp −0.02 at Mach 0.9 and −0.08 at Mach 1.0 one caliber
  behind the shoulder (+110 to +480 m), −0.08 to −0.20 at the shoulder, and at most +0.03 on the body. The logs
  span Cp −0.13 to +0.15 while rising. High readers sit in the shoulder's range; low ones mostly at +0.005 to
  +0.06, beyond the tunnel's +0.03 only where vent lag or a fin may add.
- **Step.** The tunnel's −410 m step (a Cp change of about 0.07) is near the largest measured, 488 m. Half of
  the private steps (5 of 10) are a quarter to a half of it. Hedy's and Piccard's largest swings come while
  slowing; AerospaceNU's while speeding up (+95 m rising, −53 m slowing).
- **Mach.** The largest fast changes fall at Mach 0.93 to 1.07 on 7 of 10 private flights and on the 3 public
  flights with a step: the tunnel's Mach 0.98 to 1.02, widened by the day's temperature (1.7% per 10 K), the
  accelerometer's scale (§1) and local flow.

## What the project can use

- **Data and terms.** AerospaceNU [L2] and Piccard [L5] are GPL-3.0 data: cached under `refs/`, cited, and
  used for statistics only, as the earlier notes do. Hedy [L3] (in RocketPy, MIT), WVU [L4] (MIT) and UTRGV [L6]
  (MIT) may be cached and cited; whether each team agreed to the license of the repository holding its data is
  not recorded. Forum A and B [L1, L8] carry no license, and their files sit on Dropbox and Google Drive outside
  the forum: cite and cache only. Private flights: aggregates only.
- **M7.4's "Mach baro error".** Suggest a steady term D = −Cp·V²/(2g) with Cp from −0.13 to +0.15 rising, wider
  slowing (low on 8 of 10), plus a step at Mach 0.93 to 1.07 of 20 to 490 m (median 130 m) over 0.04 to 0.45 s:
  a dip then a jump while speeding up, the reverse while slowing. *Expected effect:* fault sizes drawn from
  flights, both signs, where the tunnel table alone gives one sign per port station.
- **M7.2's flag.** The lowest step came at Mach 0.88, so 0.9 would miss one of the 13 seen. Steps were only
  sought above Mach 0.85, and a step's Mach carries the accelerometer's scale (§1), so the logs can't place a
  threshold below 0.9; 0.85 is the cautious reading. The steady error has no threshold: +68 m at Mach 0.3 to 0.5
  on Forum A. A boost reading could carry ±0.15·V²/(2g) rather than a Mach cut (**Inference**).
- **M6.8's lockout.** Check the lockout against the simulated falling Mach 0.9 crossing: every false descent
  seen ended before it, one only 0.15 s before. Altus Metrum already de-weights the baro near Mach [D1] p. 67.
- **A reader check.** The Forum B decode shows the AltOS telemetry layout [D2] suffices to read `.telem` sensor
  packets, if the project's log reader ever takes them.

## Open questions

- Where were the ports? A flight with a documented port station, bay volume and vent area would tie a Cp to a
  station and split port pressure from vent lag; the Hedy and WVU teams' reports may give stations.
- The 14 private GPS-only logs could be paired with their flights' baro logs for more cases.
- UTRGV's GPS alignment: its auxiliary IMU could settle the sign. AerospaceNU's 12 to 16% IMU excess: scale or
  clock? Piccard's CATS IMU, with 1 g taken off, could be a third track on that flight.
- Spaceport America Cup reports that plot baro beside GPS were not mined here.

## Sources

Read 2026-10-19. Logs cached under `refs/research/transonic-dip-in-logs/dl/` or the earlier notes' caches.
- [L1] The Rocketry Forum, "Failed Level 2 attempt but many things learned", and its linked TeleMega CSV (no
  license; site terms <https://www.rocketryforum.com/help/terms/>):
  <https://www.rocketryforum.com/threads/failed-level-2-attempt-but-many-things-learned.146905/>
- [L2] AerospaceNU, `rocket-flight-data@e951b1f`, `flight-data/2022-06-11 Beanboozler/` (FCB and EasyMini CSVs),
  GPL-3.0: <https://github.com/AerospaceNU/rocket-flight-data>
- [L3] RocketPy@9bd6ad3a (MIT), Hedy's `cats_tust/` data and notebook: <https://github.com/RocketPy-Team/RocketPy>
- [L4] rex-mcall/Shred-O-Vision@619af886f4 (MIT), README, LR and HR CSVs: <https://github.com/rex-mcall/Shred-O-Vision>
- [L5] catsystems/euroc21-team-data@ef54912 (GPL-3.0), `Aris-Piccard/`: TeleMega and TeleMetrum CSVs, CATS
  baro and IMU CSVs, READMEs: <https://github.com/catsystems/euroc21-team-data>
- [L6] Lmx154/RL-RocketPy-Notebook@80c9cd6 (MIT): Blue Raven and GPS CSVs and summaries, the team board's
  `FLGT0023_TRIMMED.CSV`, alignment notes: <https://github.com/Lmx154/RL-RocketPy-Notebook>
- [L7] UND-ARC/IREC-ARC-2024@94a991b (MIT), `IREC_Primary_Flight_Data.csv`: <https://github.com/UND-ARC/IREC-ARC-2024>
- [L8] The Rocketry Forum, "I managed to lose a rocket with GPS telemetry", and its linked `.telem` (no license):
  <https://www.rocketryforum.com/threads/i-managed-to-lose-a-rocket-with-gps-telemetry.68671/>
- [D1] Altus Metrum manual v1.9.22.2 (CC BY-SA 3.0), pp. 37, 67: <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [D2] *AltOS Telemetry* (CC BY-SA 3.0), pp. 4 to 5 and 10: <https://altusmetrum.org/AltOS/doc/telemetry.pdf>
- [T1] The tunnel and free-flight sizes and vent rules: [altimeter Mach error](altimeter-mach-error.md) §1 to §5.
