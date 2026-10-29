# Hobby main openings logged at 500 Hz

**In short: six public flights log a main opening at 500 Hz, all on Featherweight Blue Ravens, and their peaks span
0.49 to 2.65 times the simulator's `Instant` load (median 1.26) on a 50 ms mean; on a 200 ms mean, 0.16 to 1.05
(median 0.68).** Instant sits below the 50 ms peak on four of six flights, so the parent note's 2.3 to 3.5 times
over-read ([canopy-opening-loads](canopy-opening-loads.md) §4) isn't a general bound. Only the Blue Raven records
the whole descent at 500 Hz; a Raven 3 or 4 logs 400 Hz on its long axis, the rest 1 to 100 samples/s (§1). Of 15
public 500 Hz files from 12 or 13 flights, 6 flights (7 files) show the opening with both speeds readable (§2). One
states its canopy and mass and flew a 91 Hz load cell of unstated place (§5); none has a cell at the swivel at
500 Hz. **How far to trust it:** one section's accelerometer isn't the canopy's load. Its share of the harness force
is unknown, and tumbling adds 0.2 to 28 g per 0.1 m of offset at the peaks (§3, §5). Each factor is computed here
(**Inference**) from logs whose teams state no license.

This builds on [canopy-opening-loads](canopy-opening-loads.md) (the factor, the UAH load cells) and on
[accel-bias-after-pad-zero](accel-bias-after-pad-zero.md), which found the 500 Hz files.

## 1. Which loggers record fast enough in descent

| logger | acceleration rate in descent | source |
|---|---|---|
| Featherweight Blue Raven | 500 Hz, 3 axes, ±32 g; high-range sensor above 32 g | [1] PDF pp. 18, 23, 24; [2] |
| Featherweight Raven (2014 manual) | 400 Hz long axis, 200 Hz lateral, for 480 s | [3] PDF pp. 4, 12 |
| Featherweight Raven4 | 400 Hz long axis, 200 Hz lateral; 8 min high rate | [4] |
| Altus Metrum (TeleMega, EasyMega) | 100 samples/s ascent, 10 descent (100 ms measured) | [5] PDF p. 83; [11] |
| Entacore AIM XTRA | 100 Hz in flight, 10 Hz on the pad, in one file | [9], measured |
| CATS Vega | 100 Hz | [cats-vega-height](cats-vega-height.md) |
| Eggtimer Proton | descent 1 to 10 samples/s, default 2 | [6] PDF p. 32 |
| PerfectFlite StratoLoggerCF; Missile Works RRC3 | 20 samples/s, baro only | [7] PDF p. 49; [8] |

- **Blue Raven files.** Each flight gives a 50 Hz low-rate CSV (baro, channel voltages, flags) and a 500 Hz
  high-rate CSV (accel, gyro) aligned by a 1 ms sync code [1] PDF pp. 18, 23, 24. Measured: two files stop at 192,000
  rows (384 s), one runs to 327,342, so a long descent can outlast the 500 Hz record.
- **Its own summary fields** hold the baro speed just before the main fires, the mean over 5 s from 6 s after, and
  the largest 500 Hz acceleration in each 1 s firing [1] PDF p. 33; that peak mixes charge and opening.
- **The "Main fired" flag lags.** It turns true 1.5 s after the channel turns on [1] PDF p. 15. Measured on the
  NUSTARS flight: the main channel's voltage drops at 64.53 s, the 500 Hz record jumps 181 g at 64.532 s, the flag
  reads true at 66.04 s. This note times each firing by the voltage drop or the 500 Hz record's charge jump,
  whichever comes first: on NUSTARS Midland the jump (153.64 s) leads the first low voltage by 80 ms.

## 2. The public 500 Hz files, counted

All 15 Blue Raven high-rate files that [accel-bias-after-pad-zero](accel-bias-after-pad-zero.md) found (L1 there)
were re-read, plus NUSTARS's own copy of its 2025 flight (both files byte-identical, SHA-256 checked).

| flight (repository) | files | opening usable? | why not |
|---|---|---|---|
| NUSTARS, 2025-08-16 (NUSTARS/launch-vehicle) | 1 | yes | |
| NUSTARS, Midland 2026 (same) | 1 | yes | |
| Snowbird, FAR 2026 (VoicefulEmu, legomyleg) | 2 | yes, both units | |
| Volta, IREC 2025 (Volta-Rocketry/CUBESAT-2026) | 1 | yes | |
| Pico flight, 2025-06-11 (gregoryw3/Pico-FreeRTOS) | 1 | yes | |
| ARD, Level 3 (lehangajanayake/ARD) | 1 | yes | |
| NUSTARS FT3 2026, payload and rocket | 2 | no | payload already at 7 m/s; rocket's main never fires |
| Procyon, 2026-04-04 (prs2026/Procyon) | 1 | no | two flights in one file; 500 Hz record ends first |
| Procyon, 2026-06-07 (same) | 1 | no | never slows: its summary gives 80.4 ft/s under the main |
| Procyon, 2026-01-18 (same) | 1 | no | 500 Hz file disagrees with its summary: 3 to 4 g against 112.9 g |
| WVUER (rex-mcall/Shred-O-Vision, MIT) | 1 | no | no main charge in the record; attitude failed |
| Imperial, two files (icl-rocketry/art-data) | 2 | no | no opening seen; summary: 50.6 ft/s before, 43.9 after |

Six flights qualify, of 12 (13 if Imperial's two files are two flights). Only Shred-O-Vision carries a license
(MIT); the rest have none, so they are cited and cached under `refs/`, never copied.

## 3. Method (Inference)

- **A factor without C_D or mass.** In steady descent under the main the accelerometer reads 1 g, so the canopy
  system's drag at w equals the weight. `Instant` puts that drag area at the line-stretch speed v₁, a load of
  (v₁/w)² g. The flight's factor is k = (a_peak/g)·(w/v₁)², the parent note's F_peak/(q C_D S) with the C_D S the
  flight itself shows. ρ is taken equal at both speeds; they are 125 to 300 m apart, so k reads 1 to 3% low.
- **Speeds.** v₁ is the least-squares baro slope over the 5 s before the firing, stopping 0.1 s short of it: the
  charge's pressure pulse moves the baro by up to 200 ft in that last 0.1 s. The range takes 3 s and 10 s. w is
  the slope from 6 to 16 s after the firing; the range adds 16 to 26 s. Rows below 30 ft AGL are dropped.
- **Peaks.** The three axes are averaged over a centered window, then the magnitude taken: 10 ms, 50 ms or 200 ms.
  A 50 ms mean keeps a fill of 0.2 s or longer and smooths the airframe's ringing; 200 ms keeps only the broad
  part. Phases: charge, from 0.15 s before to 0.1 s after the voltage drop; opening, 0.1 to 4 s after.
- **Snatch against inflation.** Four flights show two humps 1 to 2 s apart, two show one. One section's
  accelerometer can't tell line stretch from filling, and no flight has video or a swivel cell; both are listed.
- **Bias, with sign.** The drogue's drag, about 1 g at v₁, stays in the peak, and its area stays in the w that
  sets C_D S; the simulator's `Instant` uses the main's own C_D S. Taking the drogue out of both gives
  k′ = (a_peak/g − 1)/(Instant − 1), a change of (k − 1)/(Instant − 1): −0.06 (ARD) to +0.21 (Midland).
- **Rotation.** It adds ω²r and α·r at the sensor; the gyros give 0.2 to 28 g of ω²r per 0.1 m of offset at the
  peaks, which makes the measured k read high by an unknown amount; no offset is stated. NUSTARS 2025 and ARD read
  1.24 and 1.18 g in steady descent under the main against 0.99 to 1.01 g on the rest, likely the same bias.

## 4. Per-flight results (Inference, computed here 2026-10-29)

| flight | fired at | v₁ (range) | w (range) | Instant | charge, 50 ms |
|---|---|---|---|---|---|
| NUSTARS 2025 | 162 m | 21.1 (20.4–21.9) | 5.94 (5.54–5.94) | 12.6 g | 17.5 g |
| Snowbird, unit P | 457 m | 28.0 (26.1–34.9) | 3.49 (3.49–3.67) | 64.2 g | 2.3 g |
| Snowbird, unit S | 449 m | 28.0 (26.1–34.8) | 3.49 (3.49–3.62) | 64.2 g | 1.8 g |
| Volta 2025 | 463 m | 26.2 (21.8–26.2) | 7.34 (7.34–7.88) | 12.7 g | 7.8 g |
| NUSTARS Midland | 305 m | 17.5 (16.7–17.5) | 5.90 (4.47–5.90) | 8.8 g | 21.4 g |
| Pico 2025 | 243 m | 25.6 (25.5–26.2) | 9.37 (8.79–9.37) | 7.5 g | 15.2 g |
| ARD Level 3 | 250 m | 20.3 (20.1–20.8) | 6.59 (6.59–9.08) | 9.5 g | 8.6 g |

| flight | humps after the firing, 50 ms mean | k, 50 ms (range) | k, 200 ms |
|---|---|---|---|
| NUSTARS 2025 | 15 g at +0.6 s; 18 g at +1.7 s | 1.42 (1.14–1.52) | 0.75 |
| Snowbird, unit P | 33 g at +0.8 s | 0.51 (0.15–0.65) | 0.16 |
| Snowbird, unit S | 30 g at +0.8 s | 0.47 (0.30–0.57) | 0.15 |
| Volta 2025 | 14 g at +3.1 s | 1.10 (1.10–1.83) | 0.62 |
| NUSTARS Midland | 11 g at +0.9 s; 23 g at +2.1 s | 2.65 (1.52–2.93) | 1.05 |
| Pico 2025 | 10 g at +0.6 s; 11 g at +2.1 s | 1.49 (1.25–1.50) | 0.74 |
| ARD Level 3 | 5 g at +1.9 s; 5 g at +3.8 s | 0.49 (0.47–0.94) | 0.36 |

Speeds in m/s, heights AGL; k uses the largest 50 ms hump. Snowbird P's lower bound takes its summary's 172.2 ft/s
before the main [10] with the log's w; with the summary's 14.9 ft/s after it as well, k = 0.25. Its baro jumps up
to 122 ft between samples, 9 times by over 100 ft in the last 18 s.

- **The spread.** Over six flights (Snowbird once), k at 50 ms is 0.49 to 2.65, median 1.26, above 1 on four. At
  200 ms it is 0.16 to 1.05, median 0.68, above 1 on one. The parent note's UAH load cells gave 0.43 and 0.29 and
  NDRT's 100 Hz IMU about 0.52 [canopy-opening-loads §3]. On a 50 ms mean four of these six read above 0.52 and
  all six above 0.43, though UAH's figures are on a 5 ms mean; on 10 ms these read higher still (NUSTARS 2025, 2.8).
- **The two Snowbird units agree** within 10% (33.0 and 29.9 g at +0.80 s); 2 g at the firing suggests no main
  charge in their bay (**Inference**).
- **Charge pulses are as large as the opening.** At 50 ms they run 0.04 to 2.4 times Instant; at 10 ms, up to 82 g
  (NUSTARS 2025; 181 g in one 2 ms sample). On NUSTARS 2025, Midland, Pico and ARD the charge out-pulls the first
  hump, and on Pico and ARD the largest one too, as the parent note's builder trace found [canopy-opening-loads §3].
- **Rebound.** NUSTARS 2025's largest hump comes after its logged inertial speed overshoots the descent rate (to
  about 2 m/s) and falls back: likely a harness recoil, not inflation (**Inference**; the gyros pass their ±2,000
  deg/s range there [1] PDF p. 23, so the inertial speed is uncertain). Its first hump alone gives k = 1.16.

## 5. The one flight with canopy and mass stated

NUSTARS, 2025-08-16: apogee 1,238 m (4,061 ft) AGL, main fired at 162 m; files in [11], no license, cited only.

- **Stated inputs** (`opening-sim.py` [11]): a 4 ft (1.219 m) main with C_D 2.2, burnout mass 0.3511 slug
  (5.124 kg), drogue descent 74 ft/s, main at 550 ft. Unnamed in the script, the flight fits it: the log gives
  69 to 72 ft/s and 531 ft (**Inference**). C_D S = 2.2·π·(0.6096 m)² = 2.568 m².
- **Check against the log.** ρ at the main: the logged 0.9589 atm (97,160 Pa) with 313 K (the bay's 104 °F) or
  285.9 K (standard air at that pressure) gives 1.081 to 1.184 kg/m³. The stated inputs then predict w = 6.02 to
  5.75 m/s; the log gives 5.54 to 5.94. The ranges overlap: C_D 2.2 fits the descent as flown.
- **The factor from the stated inputs.** q₁C_D S at 21.1 m/s is 618 to 677 N (12.3 to 13.5 g on 5.124 kg). The
  first hump, 14.7 g at 50 ms (739 N), gives k = 1.09 to 1.20; the rebound, 17.9 g (900 N), 1.33 to 1.46. These
  match §4's ratio method, which needs neither input.
- **Rotation is the caveat here.** The gyros read 1,994 deg/s at the first hump and 3,017 deg/s at the rebound:
  12.3 and 28.3 g per 0.1 m of offset (a sample either way: 13.8 g, and 19 to 31 g). Single axes pass the
  ±2,000 deg/s range 22 times, so ω may read low. The peaks can't be told from tumbling here.
- **The load cell.** An HX711 amplifier read in a loop with a 10 ms pause (`flight_loadcell.ino` [11]): 8,865
  samples over 97.98 s, 90.5 Hz, 11 ms median spacing (counted). Its peak is 49.6 lbf (221 N) 1.40 s after the
  apogee charge; at the main, 22.8 lbf at +0.41 s, 22.2 at +0.80 s, 18.9 at +1.93 s. It averages 2.12 lbf under the
  drogue and 0.51 lbf under the main against an 11.3 lb design weight, so it carries neither canopy's whole load
  (**Inference**). Without its place in the harness, it gives no factor.

## 6. Closest other candidates and what each lacks

- **Rocketry Forum, 2023-02-01, post 4** [9]: an AIM XTRA's three axes as a CSV (23,800 rows, counted), on a rocket
  of "about 20 lb" under a 66 in homemade toroidal main with a C_D near 1.9. It logs 100 Hz in flight, and no
  baro, so no v₁ or w. Measured: −28.7 g on the long axis at 156.84 s (taken here as the charge), then 15 g at
  +0.33 s and 13 g at +1.5 s. The poster says ejection and harness shocks exceed the
  opening's, and that the sensor wasn't vertical, so the file misses part of the load.
- **Same thread, posts 5 and 6** [9]: two plots of one rocket, 114 ft/s under the drogue, 24 ft/s under a toroidal
  main in a deployment bag; no data, size or mass.
- **Blue Raven summaries** [10]: 7 cached public ones list main speeds and the firing's peak, which includes the
  charge (§1), so they bound nothing.

## What the project can use

- **Data.** The six flights' files and NUSTARS's script and load cell are cited and cached under `refs/` with the
  scripts that made §4; no license, so never committed. Shred-O-Vision (MIT) may be quoted.
- **For M1.14f.** Measured section peaks run 0.49 to 2.65 times `Instant` at 50 ms, not a 2.3 to 3.5 times
  over-read. The parent's §4 over-read should be scoped to its two load-cell flights, as its
  trust line already is. Testing `FillingTime` on these six flights with the measured v₁ and w would add a reference.
- **For M6.6 and M6.7.** A deployment-load output that prints Instant as an upper bound may flatter on four of six
  flights at 50 ms (rotation may explain part). Suggested: print Instant as a typical inflation load with a measured
  section range of about 0.5 to 2.7 times it, and the charge pulse and snatch as separate, unbounded items.
- **A sample that would settle it:** a Blue Raven with a load cell at the swivel logged at 500 Hz or more, the
  sensor's offset from the section's center of mass stated, and video of the opening.

## Open questions

- Which hump is the canopy filling? Video or a swivel cell on any of these six rockets would decide it.
- How far does tumbling inflate the 50 ms peaks? It needs the sensor's position in each section; none is stated.
- Unread: the Spaceport America Cup and Student Launch reports of these teams, which may state canopy and mass for
  the other five flights.

## Sources (all read 2026-10-29)

1. Featherweight, *Blue Raven User's Manual* (2026-05-12), PDF pp. 15, 18, 23, 24, 33.
   https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2026may12.pdf
2. Featherweight, Blue Raven product page. https://www.featherweightaltimeters.com/blue-raven-altimeter.html
3. Featherweight, *Raven User's Manual* (2014-05-20), PDF pp. 4, 12.
   https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/raven_users_manual_2014may20.pdf
4. Featherweight, Raven4 page, recorded-data list. https://www.featherweightaltimeters.com/raven-altimeter.html
5. Packard, Garbee et al., *The Altus Metrum System*, v1.9.22.2 (2025), App. F, PDF p. 83, CC ShareAlike 3.0.
   https://altusmetrum.org/AltOS/doc/altusmetrum.pdf
6. Eggtimer Rocketry, *Proton User's Guide* 2.01C, PDF p. 32.
   https://eggtimerrocketry.com/wp-content/uploads/2024/02/Eggtimer-Proton-2_01C.pdf
7. PerfectFlite, *StratoLoggerCF manual*, PDF p. 49. http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf
8. Missile Works RRC3 manual, as read in [altimeter-deploy-logic](altimeter-deploy-logic.md) (R3, p. 24).
9. Rocketry Forum, "Looking for acceleration/time data of parachute deployments on big rockets", posts 4 to 6
   (2023-02-01), attachment `AccelVsTime.csv`. https://www.rocketryforum.com/threads/
   looking-for-acceleration-time-data-of-parachute-deployments-on-big-rockets.177697/ Forum terms; cited only.
10. Blue Raven summaries (no license), five repositories, cached for [the gaps note](public-logs-evidence-gaps.md).
11. NUSTARS/launch-vehicle@d7bad45ba6 (no license): `analysis/ParachuteOpeningForcesResearch/` (logs, `opening-sim.py`,
    `flight_loadcell.ino`); Midland files as cached at @3a89e4be5a. https://github.com/NUSTARS/launch-vehicle
12. 500 Hz logs (repository@commit, no license unless marked): VoicefulEmu/DataAnalysisSnowbirdTest@cb6b8157a1,
    legomyleg/BYU_Telemetry_HUD@c66c04f4b3, Volta-Rocketry/CUBESAT-2026@eb7e79694c,
    gregoryw3/Pico-FreeRTOS@905dfe6c1f, lehangajanayake/ARD@a27d2518fb, prs2026/Procyon@8c8b2b9f08,
    icl-rocketry/art-data@bee1c863eb, rex-mcall/Shred-O-Vision@619af886f4 (MIT).
