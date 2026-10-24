# Student Launch flights with a weighed mass and a published altimeter log

**In short: no public flight has it all.** NASA's Student Launch handbooks since at least 2024 make teams submit raw
altimeter data as .csv or .xlsx in the FRR and the post-launch assessment review (PLAR). NASA does not publish those
files, and almost no team does either. Across 11 teams' archives and 12 GitHub searches, no flight pairs a weighed
mass, motor, wind and apogee with a log as data under terms the project may use. The nearest is NC State's
2025-03-08 vehicle demonstration flight (VDF): 42.9 lb fully ballasted, L1520T, 13 mph, and an MIT log from its
air-brake computer showing the brakes stayed shut. But that 42.9 lb is the FRR's as-built table with the full ballast
added (39.68 − 0.46 + 3.7 = 42.92 lb), not a stated scale reading, and the log is not the scoring altimeter's.
**14 flights by 6 teams** state a weighed or part-by-part mass with its value, the motor, the wind and the apogee, but
show the log only as a plotted figure; 13 of them had no motor anomaly. Logs as data exist for 14 NC State,
10 Notre Dame and 3 USF flights, mostly under MIT, but none has a weighed mass beside it, and most flew deployed air
brakes. How sure: high on what each source states (every number was re-read in a separate pass); medium on the
counts, which come from keyword scans and hand tallies of cached reports, not a proof that no other flight exists.

This builds on [as-built-mass-growth](as-built-mass-growth.md) §4, where five of six flights flown at a weighed or
as-built mass still came in 3.6% to 18.4% under their own simulations (the sixth flew 4.4% over), and on
[student-launch-apogees](student-launch-apogees.md), whose report list and file names this note reuses. The question
is whether flights with mass removed as a cause could size the drag share of the private collection's +8.73% mean
bias ([accuracy](../accuracy.md#real-flights)).

## How the flights were read

- **Mass:** **W** stated as weighed or measured; **P** every part weighed and summed; **T** an as-built or
  flysheet total, method unstated; **S** a dry mass on a team site or in log metadata, method unstated. Only W and P
  with a printed value count as "mass removed as a cause".
- **Usable** means such a mass, the motor, the wind, a log as a data file, no drag device opened during ascent, and
  terms that allow caching. Page numbers are PDF page indices. Team reports state no license, so they are cited only.
- Logs were read for column names and their state, servo and encoder columns with streaming tools. Code in the
  team repositories was not opened; three of NC State's repositories may derive from GPL code and were skipped.

## 1. What NASA asks teams to submit

- **2026 handbook:** a successful VDF needs "Raw altimeter data shall be submitted in .csv or .xlsx format" (req.
  2.6.8.3, p. 12). The FRR outline asks for the VDF's launch conditions, altimeter profile and raw data, and for
  lift-off and landing masses (p. 34); the PLAR outline for motor, altitude, profile and raw data (p. 41). The FRR
  addendum outline asks for the altimeter's flight profile and plots, not raw files. Nothing requires the mass to be
  weighed [H26].
- **2024 handbook:** the same raw-data lines in the FRR (p. 34) and PLAR (p. 41) outlines [H24]. 2022 asked only that
  the flights be "flown and flight data reported in the FRR and/or FRR Addendum" (p. 23) [H22]; 2020 asked for
  altimeter output as proof of flight (p. 11–12) [H20]. The 2023 and 2025 handbooks weren't reached, so the raw-CSV
  rule began in 2023 or 2024.
- **Publication:** no NASA page or archive of team reports or raw data turned up. NC State writes that its 2025 files
  "have been submitted with the FRR package" [N25F p. 95] but did not post them. **Inference:** the CSVs exist at
  most teams since 2024; a request to each team is the only route to them.

## 2. The nearest flight: NC State, 2025-03-08

- **Report** [N25F p. 94–95, 183]: "Fully ballasted and flown on an AeroTech L1520T motor, the launch vehicle weighed
  42.9 lbs". Wind 13 mph (10–20), 70 °F, 99,900 Pa. Apogee 3,571 ft on the primary StratoLogger CF and 3,635 ft on
  the EasyMini, against a 4,088 ft prediction without air brakes; the team blames weathercocking, high winds and an
  unstable pad.
- **The mass is arithmetic, most likely.** The as-built table sums 39.68 lb wet "Wet + Ballast" with the projected
  0.46 lb of forward ballast (p. 30); the VDF carried 3.7 lb (p. 94). 39.68 − 0.46 + 3.7 = 42.92 lb. The report never
  says a scale was used, and the table doesn't say how its section masses were found, so this is class T.
- **Log** [NA, `pelicanator_launch_2.csv`, MIT]: the air-brake computer's IMU and pressure altitude, peaking at about
  1,088–1,089 m (3,570–3,573 ft) above the pad; the metadata gives 1,090.16 m. The commanded extension stays at the
  closed values (115 and 120) in every state, and the measured encoder reads 0 in all 85,844 rows. The same encoder
  reached −8 when the brakes opened on 2025-02-22 (`pelicanator_launch_1.csv`), so 0 means shut.
- **Other gaps:** no design file is public for this rocket, only the FRR's drawings and tables; the log metadata
  gives 24.05 km/h (14.9 mph) wind where the FRR gives 13 mph; the wind was gusty and the rocket weathercocked.

## 3. Weighed mass, motor, wind and apogee, log only as a figure (14 flights)

| flight | mass | motor | wind | apogee (ft) | note | source |
|---|---|---|---|---|---|---|
| NC State 2017 VDF | P 52.6 lb | L2200G | 11 mph, gusts 15 | 5,091 / 5,233 | weakest P | N17f 12–13, 45, 51 |
| NC State 2018 VDF | W 44.3 lb | L2200G | 10.4 mph | 5,984 | | N18f 89, 93–94 |
| NC State 2021 VDF | P | L1520T | 7 mph | 2,995 | | N21F 100, 104–105 |
| NC State 2022 VDF | P | L850W | 15 mph | 3,294 | 0.5 lb ballast | N22F 47, 83, 86 |
| NC State 2024 re-flight | W 52.5 lb | L1940X | 7 mph | 3,812 | | N24Ad 6, 13–14 |
| NC State 2024 launch week | W 48.8 lb | L1940X | 2.5 mph | 4,038 | motor liner failure | N24PL 6–7 |
| Georgia Tech 2018 VDF | P | L1390G | 7.8 mph, gusts 33 | 4,734 | | G18f 67, 81, 84 |
| Purdue 2020 VDF | P | L1115 | 14 mph | 4,588 mean | motor mass guessed | U20f 35–36, 45, 48 |
| Notre Dame 2018 VDF | P | L1115 | 2 mph | 5,765 | | D18f 102, 106 |
| Notre Dame 2021 VDF | P | L1395 | 8 mph | 5,472 | | D21F 11–14, 83–85 |
| Notre Dame 2022 VDF | P | L2200G | 13 mph | 5,815 | | D22F 26, 69 |
| USF 2018 flight 1, 01-27 | P | L1420R | 14 mph, gusts 20 | 5,578 mean | | F18f 25, 65–67 |
| Toledo 2026 subscale | W | H130W | 4 mph | 1,225 | | T26C 49–50 |
| Toledo 2026 VDF | W 40.9 lb | L1256WS | 5 mph | 5,327 | | T26F 98–99, 105 |

- **Counts** (by hand from the table): NC State 6, Notre Dame 3, Toledo 2, Georgia Tech 1, Purdue 1, USF 1; 13
  without a motor anomaly. NC State 2017's "actual component weights" (p. 13) is the weakest P: it doesn't say each
  part was put on a scale.
- **Left out:** NC State 2020 says its RockSim file was updated "once the full-scale vehicle was assembled and
  measured" (p. 37), but prints no measured figure; the 47.5 lb on p. 12 is the design's. The 2025-03-08 flight (§2)
  is class T. Other team-seasons scanned gave masses of type T or none near a flight.
- **Printed log:** NC State's 2024 FRR prints its VDF's RRC3 log (20 Hz, 999 rows, p. 271–291) and Eggtimer Quasar log
  (439 rows, p. 292–300) as text [N24F]. That is the only scoring-altimeter log in data form beside a mass, and the
  mass is an as-built table (T), so it is not usable here.
- **Design files:** none of the 14 has one public; each rocket would be rebuilt from drawings and tables, with fin
  shapes, nose shapes and finish read from figures.

## 4. Logs as data without a weighed mass

- **NC State AirbrakesV2** [NA] (MIT, `launch_data/`): 11 CSVs and 11 metadata entries (date, Cd, dry mass,
  temperature, pressure, wind, rod angle, apogee, a note; the motor field is blank in every entry). One CSV,
  `legacy_launch_2.csv`, holds only standby, so 10 have flight data. The 2025-03-08 VDF and the 2026 subscale 2 show
  the brakes shut on a working encoder. The 2024-09-28 test and the 2025 subscale 1 were commanded open and the team
  says the brakes never moved, but those logs have no encoder. On the 2025-01-25 test the encoder first moved (3,
  then 4 counts) 7.6 s before apogee, while the team's note says the brakes opened in free fall.
- **NC State launch catalog** [NL] (no terms stated): six 2025-26 flights, each with motor, "Dry Mass (kg)", wind,
  rail angle, apogee, EasyMini and Fluctus CSVs, an air-brake log and an OpenRocket file; four of them add to the
  AirbrakesV2 set, for 14 NC State flights. The altimeter logs agree with the page's apogees within 1.32% (the largest
  gap, Government Work 2: page 2,376.6 ft against 2,408 ft on the EasyMini). Five of the six flights commanded the
  brakes open in coast. The sixth, the 2026 subscale 2 (J825R, 22.7 km/h, 13° rod), is the closest near-miss: its
  dry mass is 5.15 kg on the page and 4.77 kg in the metadata, method unstated, and the team says the motor
  underperformed ("undershot our apogee because of the motor").
- **Notre Dame** [ND1, ND2] (`EamonTracey/apogee` and `apogee2`, MIT): the apogee-control computers' logs, 30–40 Hz
  BMP390 barometer and IMU with the commanded servo angle. 2023-24: 4 flights, brakes commanded open in coast on all.
  2024-25: 2 subscale logs (no servo column), 4 full-scale flights and one log with only ground rows. Only Huntsville
  2025-05-04 kept 0° through coast; its log peaks at about 1,479 m (4,850 ft) above the pad. The repositories state
  no motor per flight, no launch-day wind and only a model mass (the 2024-25 vehicle file: 1.01243 slug, 14.78 kg,
  apparently without motor). The team's 2024-25 reports are not online.
- **USF SOAR** [USF] (MIT): three 2024-25 flights. The 2024-12-21 subscale RRC3 log (20 Hz, peak 2,184 ft), a rough
  Huntsville telemetry log and a 2025-06-06 air-brake flight; no mass, motor or wind. **UND** (MIT) logs and `.ork`
  files are covered by [und-flight-configs](und-flight-configs.md); no report states a flown mass.
- **Elsewhere:** FAMU-FSU 2024 posts flight decks, not logs. Two rounds of six GitHub searches returned 91 and 261
  repositories; 6 more hold flight logs but have no license, no team or date, or are GPL (not opened).

## 5. Terms

- **MIT, cache and use with attribution:** AirbrakesV2 and FlightMetrics ("Copyright (c) 2025 NCSU High-Powered
  Rocketry Club"), `apogee` and `apogee2` (Eamon Tracey, 2023 and 2024), `usfsoar/NSL_25-26_EECS` (USF SOAR, 2025).
  Following ADR-082's practice, read the logs from `refs/` and commit statistics and SHA-256 only.
- **No terms stated, cite only:** every team report and web page read, NC State's launch-catalog CSVs and
  OpenRocket files (cached outside the `.ork` survey's reach, as `.ork.bin`), FAMU-FSU's decks. NASA's handbooks are
  US-government works.

## What this suggests

1. **No public Student Launch set can isolate the drag share of the bias yet.** No flight meets every test, and the
   nearest has a computed mass, gusty wind and weathercocking, which can't be told apart from a drag error.
2. **The 13 clean flights in §3 are an apogee-only check with design error stated**, not a drag measurement: they
   fix the mass but need every rocket rebuilt from drawings. If they were flown, they would test the bias sign at a
   known mass, as [as-built-mass-growth](as-built-mass-growth.md) §4's six flights did with the teams' own tools.
3. **NC State's 2025-26 catalog is the nearest thing to a public validation set**: OpenRocket file, motor, wind, rail
   angle and three logs per flight. It can only serve a model of commanded air brakes, or the one flight without
   them, and its terms allow citing only. An air-brake case would test drag-device modeling, not the mass question.
4. **Notre Dame's and NC State's MIT logs could feed a flight-derived Cd** where the brakes stayed shut (the Huntsville
   2025 flight, the 2025-03-08 VDF), as in [flight-derived-cd](flight-derived-cd.md), if mass and motor are taken
   from the reports. For Notre Dame the reports are missing.
5. **[public-flights](public-flights.md) understated `apogee2`:** it said two subscale flights with air brakes. The
   repository holds 7 logs: 2 subscale with no servo column, 4 full-scale flights and one log with no flight. Fixed
   there (#777), with `apogee`'s 4 flights.

## Open questions

- Whether any NC State mass comes from a scale: the 2025 VDF's 42.9 lb matches arithmetic, and two dry masses
  (5.15 and 4.77 kg) for one 2026 flight suggest at least one is not a weighing.
- The start year of the raw-CSV rule (the 2023 and 2025 handbooks weren't reached).
- Notre Dame's 2024-25 reports, which would give mass, motor and wind for its 7 logs; Huntsville 2025-05-04's wind
  could come from a public airport record.
- Where the brakes were on flights with no encoder (2024-09-28, both 2024-11-16 flights) or one that reads 0
  even when commanded open (the four 2026 full-scale flights).
- The Toledo 2024 re-flight's mass method: the earlier note calls it weighed; this scan found no statement.

## Sources (read 2026-10-18)

- **H20, H22, H24, H26** NASA Student Launch handbooks, as cached by as-built-mass-growth; 2026:
  <https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf>; 2024:
  <https://www.nasa.gov/wp-content/uploads/2023/08/2024-sli-handbook-final-copy1.pdf>.
- **N** NC State report archive, <https://ncsurocketry.org/report-archive/>: FRRs N17f, N18f, N20f, N21F, N22F, N24F,
  N25F; addendum N24Ad; PLAR N24PL (file names as in student-launch-apogees).
- **NL** NC State launch catalog, <https://ncsurocketry.org/launches/>, and its CSVs under
  `wp-content/uploads/2026/04/`.
- **NA** <https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2> at afdcecb44e97, `launch_data/`.
- **ND1** <https://github.com/EamonTracey/apogee> at 8d704c2454a3; **ND2** <https://github.com/EamonTracey/apogee2> at
  4bbc068b5b95, `data/launches/`, `data/vehicles/fullscale25.json`.
- **USF** <https://github.com/usfsoar/NSL_25-26_EECS> at 78e7344ec3f7, `24-25_Code/Launch_Data/`.
- **G18f** Georgia Tech 2018 FRR, <http://usli.gatech.edu/documentation>. **U20f** Purdue 2020 FRR,
  <https://purdueseds.space/wp-content/uploads/psp-sl/casper/frr/Purdue%20University%20-%202020%20FRR%20Report.pdf>.
- **D18f, D21F, D22F** Notre Dame FRRs, <https://seniordesign.ee.nd.edu/>. **F18f** USF 2018 FRR (Wayback copy, as in
  student-launch-apogees). **T26C, T26F** Toledo 2026 CDR and FRR,
  <https://www.utrocketry.club/projects/nasa-usli-2026>.
- **FAMU-FSU 2024**, <https://eng-web1.eng.famu.fsu.edu/me/senior_design/2024/team502/documents/launches/>.
