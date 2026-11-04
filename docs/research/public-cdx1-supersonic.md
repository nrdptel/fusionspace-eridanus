# Public `.CDX1` designs flown past Mach 1 on a COTS motor, with a measured apogee

**In short: of the 135 public `.CDX1` files, 40 store a COTS-motor run past Mach 1, and 4 sit beside a flight
with a measured apogee. One, on GPL terms, meets the physical conditions of [M2.3c7, faster than sound][m2-3c7]:
EPFL's Wildhorn, EuRoC 2022.** Its file names a Cesaroni N1560 and stores 9,498 m and Mach 1.47; its AltiMAX log
peaks 9,547 m above the pad [W1, W2]. It is not on RASAero II's list, which the *done when* names. Illinois's
Intrepid III (IREC 2023) and Stanford's Heart of Steel (IREC 2017) are logged, but one file's motor looks wrong, the
other holds seven runs on three motors, and neither has a license. UND's IREC 2025 file is Apache-2.0 and stores
the apogee ESRA recorded as its prediction, but only the apogee was measured. With [RASAero II's four][rcf],
Wildhorn makes five, through a decision record. **Confidence:** high for the counts, stored values, logged apogees
(Wildhorn's two sensors differ by 3.9%) and licenses. Medium for which file matches which flight: no team states
it. **What it moves:** M2.3c7's floor of 5 scored COTS flights past Mach 1.

**What this covers.** The cached files [S1] (from [the `.CDX1` format note][cdx1] §5); each qualifying repository's
logs, reports and READMEs; competition score sheets [W1 to O4]. All read 2026-11-03; design files parsed as data, no
GPL or AGPL code opened, nothing flown. It builds on [IREC 2025's COTS reports][irec25] and [Vayuvega][vv].
**Stated** marks what a source says; **Inference** what this note works out.

[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[rcf]: rasaero-comparison-flights.md
[cdx1]: cdx1-format.md
[irec25]: irec-2025-cots-reports.md
[vv]: vayuvega-supersonic-cots.md

## 1. Forty files store a COTS run past Mach 1; four have a flight beside them

A file's `MaxVelocity` is RASAero II's predicted peak speed in ft/s, not a measurement. This note divides it by
the speed of sound at the file's own launch-site temperature, a = √(1.4 × 287.05 J/(kg·K) × T). The peak comes at
burnout, higher up and usually colder, so this Mach is a floor on the file's own prediction (**Inference**). Worked
example, Wildhorn: 1,668.6 ft/s is 508.6 m/s; at 74 °F (23.3 °C) a = 345.2 m/s, so Mach 1.47.

A run counts as COTS when every motor it names is labeled AeroTech, Cesaroni, Loki or AMW. Labels missing from
ThrustCurve's catalog (CTI P-7700 and P8000, AMW O3800, `M3400LB (LR)`) count as not COTS [O4]; counting the
last would make 41 files. The script read 137 files, 135 distinct by SHA-256, holding 276 stored runs:
- **40 files** have a COTS run past Mach 1; 21 have only subsonic COTS runs; 6 name a COTS motor with no result.
- **38 files** pass Mach 1 only on team, research, mixed or uncataloged motors; 2 fly only team motors below
  Mach 1; 28 have no motor or no run.
- The 40 come from **17 repositories**: 15 with no license, Terrapin's (MIT) and UND's IREC 2025 (Apache-2.0).

| Repository | Files | Top stored Mach | Flight with a measured apogee |
|---|---:|---:|---|
| EPFLRocketTeam/C_AV_SIL (copy in M0WW0M/European-FlightData) | 1 | 1.47 | yes, Wildhorn (§2) |
| ISSUIUC/ISS-STR | 1 | 1.70 | yes, Intrepid III (§3) |
| stanford-ssi/rockets-irec | 5 | 1.58 to 2.35 | yes, Heart of Steel (§4) |
| UND-ARC/IREC_2025 | 5 | 1.47 to 2.21 | apogee only, team 222 (§5) |
| ISSUIUC/ISS_SILSIM | 6 | 2.01 | no: likely Intrepid I or II, both failed (§6) |
| icl-rocketry/art-designs | 3 | 3.22 to 3.60 | no: Sirius flew its uncataloged O3800 booster (§6) |
| abean74525/M1378-Rocket, Kev1n8088/OpenrocketDesigns | 5, 3 | 2.06 to 3.38 | none found |
| Mines-Rocketry-Club/NearHigher | 2 | 2.23 to 3.22 | one flight at about Mach 0.4 (§6) |
| Terrapin-Rocket-Team/Spaceshot (MIT), dssong2/fin-iterations | 2, 1 | 2.40 to 2.61 | none found |
| prs2026/Procyon, mcm001/model_rockets | 1, 1 | 3.00, 1.28 | none matching the file (§6) |
| Berger-Stefan, Iash5225, StrathSEDS, Boreas | 1 each | 1.03 to 6.93 | none: simulation studies or planned |

The cache kept at most 6 files a repository: Stanford's tree has 64, Procyon's 38, UND's 10 [S1, T1, U1].

## 2. EPFL's Wildhorn, EuRoC 2022: a COTS flight past Mach 1 with its design and log

- **The data (Stated).** A GPL-3.0 repository of European teams' flight data holds Wildhorn's AltiMAX log,
  two SRAD logs, telemetry, an OpenRocket file and the RASAero II file [W1]. A note beside them says the RASAero
  model came "closest one to actual apogee". EPFL's own simulator repository, which has no license, holds the same
  folders under names starting `Team12_PFR_`; its `.CDX1` is byte for byte the same [W2].
- **The motor.** The file names `N1560-WH-MB-P (CTI)`, at 59.06 lb. ThrustCurve lists Cesaroni's 16803N1560-P,
  CAR-certified, 16,803 N·s over 10.76 s [W4]. No page found states the flown motor; the log's thrust ends near
  10.6 s, which fits (**Inference**).
- **The apogee.** The AltiMAX export peaks at 9,551.76 m filtered at 41.85 s on its clock, from a pad reading of
  4.63 m: 9,547 m (31,323 ft) above the pad, about 41 s after lift-off. The file stores 31,159.92 ft (9,498 m,
  0.5% low) and 42.14 s. A trade article quotes 9,467 to 9,596 m from the team's flight computers, and Mach 1.6 [W3].
- **Past Mach 1, by the log (Inference).** The log's acceleration crosses zero at 10.575 s and 3,014 m, as drag
  overtakes thrust. If thrust stays below drag after that, as in a tail-off, the speed then was at least
  √(2 × 9.79 m/s² × 6,538 m) = 358 m/s. At 3.2 km above sea level the standard atmosphere's speed of sound is
  327.8 m/s, so Mach 1.09; in air 15 K warmer, 1.06. A 10% error in the transonic baro height moves this by about
  2%. The logger's own speed channel peaks near 529 m/s at 10.5 s.
- **Does the file match the flight?** It is the team's own model, filed with the flight's data. But the folder
  note implies it was chosen after the flight; its site is sea level, the telemetry's pad 173 m. The telemetry's
  GPS peaks 9,924 m above that pad, 3.9% over the AltiMAX, so the file reads 4.3% low against it. EPFL's page
  gives a total weight of 24 kg, the file 26.8 kg [W5]. Flag it as possibly revised after the flight.

## 3. Illinois's Intrepid III, IREC 2023: a logged flight, but the file's motor looks wrong

- **The flight (Stated).** ESRA's 2023 sheet gives team 124 (Illinois) an actual altitude of 27,309 ft in 30k
  COTS [I3]. The team's flight-data repository labels its 2023-06-21 folder Intrepid III's competition launch
  [I1]. Its TeleMega peaks at 8,323.71 m (27,309 ft) and its EasyMega at 8,316.81 m. AltOS's speed, which leans on
  the accelerometer in boost, peaks at 595.9 m/s at 5.73 s: about Mach 1.7 to 1.8 (**Inference**).
- **The file.** `Intrepid_III_ReportFile.CDX1` has one run: a CTI N2540-GR-P at 66.5 lb, 31,416 ft and Mach 1.70
  [I2]. It was committed 2023-10-04, after the flight, into a folder named `Old_Intrepid_III`.
- **The motor doubt (Inference).** At 30.2 kg, the log's lift-off acceleration of 110 m/s² needs about 3,600 N.
  The N2540's catalog curve gives about 2,760 N then, and burns to 7.2 s. The log's acceleration goes negative
  between 5.5 and 5.75 s. A CTI N2900's curve fits both: 3,600 to 4,000 N early, falling from 4.65 to 5.6 s [I6].
- **Air brakes.** The team's controls repository says its controller flew on Intrepid III [I4]. The SRAD log has
  an `extension` channel, but it holds one value to 32 s on its clock, bar a 0.6 s dip, so it can't show whether
  the flaps opened. How much drag they added is unbounded.

## 4. Stanford's Heart of Steel, IREC 2017: a logged flight, three candidate motors

- **The flight.** The StratoLogger file in the 2017-06-22 folder states "Apogee: 30411' AGL" over a 4,562 ft
  ground [T1]. ESRA's 2017 scores rank Heart of Steel in 30k COTS with no altitude [T2]. A university article
  says it came within 1.5% of 30,000 ft at Mach 1.8 [T3]; the club's wiki, that it placed first [T4].
- **Past Mach 1, by the bound (Inference).** [IREC 2025's note][irec25] §3 gives apogee ≤ v·t_b + v²/(2g). The
  file's longest burn is 6.16 s (ThrustCurve's O3400), so 9,269 m needs v ≥ 370 m/s: Mach 1.05 at 35 °C.
- **The file.** `IREC_Current.CDX1`, last changed 2017-06-01, holds seven runs at a 4,596 ft, 95 °F site: five on a
  CTI N2900-CL (58 to 74.8 lb, Mach 1.43 to 1.88, 28,667 to 39,782 ft), one O3400 and one N5800. No source says
  which motor or mass flew.
- **A subscale flight, outside the cache.** A Raven export from an AeroTech J1799N flight (committed 2017-04-19)
  peaks at 7,834 ft by baro and 1,564 ft/s by accelerometer, about Mach 1.4 [T1]. Its two `.CDX1` files date from
  2017-05-26, after it, and store 8,052 ft and 10,548 ft: reconstructions that disagree, and unlicensed.

## 5. UND's IREC 2025 flight: an Apache-2.0 file, an apogee, no proof of Mach 1

- **The match is strong.** `IREC_2025_FKYWKY.CDX1` was last committed 2025-06-11, in competition week [U1].
  Its AeroTech O5500X run at 89.15 lb stores 24,675.76 ft. ESRA's 2025 sheet gives team 222 a predicted apogee of
  24,676 ft and an actual one of 21,377 ft, 13.4% short [U2].
- **Mach.** The run stores 1,704.4 ft/s: Mach 1.47 at the file's 100 °F. The team's report predicts 1.62. The
  model-free bound gives only 0.90 to 0.94, and no log is public, so Mach 1 is not shown ([irec25] §3, §6).
- **The license.** The repository's `LICENSE` grants Apache-2.0; GitHub finds it but can't identify it [U1].

## 6. Searched and set aside

- **Illinois's Intrepid 5800 mk6** (CTI N5800), dated 2022-04-05: likely Intrepid I or II (**Inference**) [I1].
  Intrepid I's motor failed, and Intrepid II lost its fins after Mach 1.3 [I5]. No apogee to score.
- **Imperial's Sirius** flew 2024-07-07; a commit 11 days later edits `Sirius_O3800.CDX1` for post-flight
  analysis [O1]. Its booster is an AMW O3800, not in ThrustCurve's catalog, and only a GPS tracker log is public.
  That log loses its fix near the top, at about 13,800 ft above the pad; every run in the file expects 30,000 ft+.
- **Procyon:** Blue Raven summaries of 47,550, 58,596 and 28,996 ft (2026-01-18, 04-04, 06-07) match no cached
  file; JUMP's two-stage file names L2523, L2342 and M2380 motors, none in ThrustCurve's catalog [O2, O4].
- **Mines's NearHigher:** one 2024-09-02 TeleMetrum flight: 739 m by baro, 771 m by GPS, 133 m/s at most [O3].
- **Terrapin, abean74525, Kev1n8088, dssong2, mcm001:** no log, report or flight statement, there or on the web.

## 7. Licenses and terms

- **GPL-3.0, data only:** M0WW0M/European-FlightData. Cache in `refs/`, never commit; publish derived numbers,
  as the project already does for AerospaceNU's GPL logs (`THIRD-PARTY-NOTICES.md`).
- **Apache-2.0 and MIT:** UND-ARC/IREC_2025 and Terrapin's Spaceshot. Cache; redistribute with license and credit.
- **No license:** the other 15 repositories, ISSUIUC/flight-data and icl-rocketry/art-data. Cite only.
- **ESRA score sheets and the articles** are public pages with no stated license: cite their numbers.

## What this suggests

How many could join M2.3c7, and on what terms (**Inference**):
- **One, Wildhorn, on every physical condition:** COTS motor, the team's own design file, a logged apogee, and
  Mach 1 shown from the log. Cache the GPL copy, flag the file as possibly revised, and fly it at the pad's 173 m.
  With RASAero II's four, that is five past Mach 1. The *done when* names RASAero II's published list, so this
  needs a decision record that admits a team-published design file beside a log.
- **One more, flagged apogee-only:** UND's team 222, with clean terms and a prediction-matched file. It counts
  toward the floor only if Mach 1 is shown, which today it isn't.
- **Two cite-only:** Intrepid III and Heart of Steel. Scoring them needs a license this project can't ask for.
- **Expected effect:** one flight more past Mach 1, at about Mach 1.5. Its two sensors differ by 3.9% at apogee
  and its file's mass by 12% from the team's page, so its error band is wider than a matched file's.

## Open questions

- Was Wildhorn's file revised after the flight? The repository history before 2022-11-10 is not public.
- Which motor did Intrepid III fly, and did its flaps open? Its 2023 report was not found.
- Which of Heart of Steel's seven runs flew? Its Raven log, in a binary format, may show the burn time.
- Why did UND's 2025 flight fall 13.4% short? No log or post-flight text was found.

## Sources

All read 2026-11-03. Copies are in the gitignored `refs/research/public-cdx1-supersonic/`.

1. **S1.** The cached files and their manifest and license list, from [the `.CDX1` format note][cdx1] §5.
2. **W1.** M0WW0M, European-FlightData, GPL-3.0, at `7b8ade67b0`: `EPFL/Wildhorn - 2022/` (AltiMAX `.xls`
   and `.pdf`, `Simulation_Data/README.txt`, the `.CDX1`). <https://github.com/M0WW0M/European-FlightData>
3. **W2.** EPFLRocketTeam, C_AV_SIL, no license, `PythonTest/ERT_Wildhorn_Flight_Data/` (added 2024-07-21).
   <https://github.com/EPFLRocketTeam/C_AV_SIL>
4. **W3.** M. Planchard, "EPFL Rocket Team Breaks Sound Barrier At EuRoc Adventure", SOLIDWORKS blog, 2022-12-05.
   <https://blogs.solidworks.com/products/solidworks/epfl-rocket-team-breaks-sound-barrier-at-euroc-adventure/>
5. **W4.** ThrustCurve.org search API, Cesaroni N1560 (motor id `5f4294d2000231000000041f`).
   <https://www.thrustcurve.org/api/v1/search.json>
6. **I1.** ISSUIUC, flight-data, no license: `README.md`, `20230621/` TeleMega, EasyMega and SRAD files.
   <https://github.com/ISSUIUC/flight-data>
7. **I2.** ISSUIUC, ISS-STR, no license, `RASAero/Old_Intrepid_III/Intrepid_III_ReportFile.CDX1`.
   <https://github.com/ISSUIUC/ISS-STR>
8. **I3.** ESRA, 2023 Spaceport America Cup page and `2023_master_scores_for_posting_v2.xlsx`, team 124.
   <https://www.soundingrocket.org/2023-sa-cup.html>
9. **I4.** ISSUIUC, ISS-Controls, `README.md`, line 8. <https://github.com/ISSUIUC/ISS-Controls>
10. **I5.** University of Illinois Aerospace Engineering, "Third time's the charm for Illinois Space Society
    student rocket", 2022-11-29. <https://aerospace.illinois.edu/news/52397>
11. **I6.** ThrustCurve.org download API, thrust samples for CTI N2540, N2900 and O3400.
    <https://www.thrustcurve.org/api/v1/download.json>
12. **T1.** stanford-ssi, rockets-irec, no license: `Avionics/Flight_Data/SA_2017-6-22/SA_2017-6-22_Strato.pf2`,
    `Trajectory_Sims/RASAero/IREC_Current.CDX1`, `Trajectory_Sims/official_irec_flights/`.
    <https://github.com/stanford-ssi/rockets-irec>
13. **T2.** ESRA, past IREC events and `released_scores_1.xlsx` (2017), team 57.
    <https://www.esrarocket.org/past-irec-events>
14. **T3.** Stanford Space Rendezvous Lab, news item, 2017-09-04.
    <https://slab.stanford.edu/news/news-space-rendezvous-lab-partners-stanford-rocketry>
15. **T4.** Stanford Student Space Initiative wiki, "IREC 2017". <https://ssi-wiki.stanford.edu/IREC_2017>
16. **U1.** UND-ARC, IREC_2025, `LICENSE` (Apache-2.0) and `OpenRocket/IREC_2025_FKYWKY.CDX1`.
    <https://github.com/UND-ARC/IREC_2025>
17. **U2.** ESRA, IREC 2025 final score sheet, team 222.
    <https://www.esrarocket.org/s/irec2025_final_scoresheet_8-2-25.xlsx>
18. **O1.** icl-rocketry, art-designs (commit `9e62a753`, 2024-07-18) and art-data `SIRIUS/`, no licenses.
    <https://github.com/icl-rocketry/art-data>
19. **O2.** prs2026, Procyon, no license, Blue Raven summary files under `rocketpy/`.
    <https://github.com/prs2026/Procyon>
20. **O3.** Mines-Rocketry-Club, NearHigher, no license, `2024-09-02-serial-11518-flight-0002.kml`.
    <https://github.com/Mines-Rocketry-Club/NearHigher>
21. **O4.** ThrustCurve.org search API by common name: no match for O3800, L2523, L2342, M2380 or an AeroTech
    M3400; P8000 and P7700 refused (no P class). <https://www.thrustcurve.org/api/v1/search.json>
22. **W5.** EPFL Space Center, Wildhorn's EuRoC 2022 page (total weight). <https://space.epfl.ch/?p=12809>
