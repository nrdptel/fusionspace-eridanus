# What the 2027 competition rules ask a simulation for

Which numbers do the 2027 rules of the Spaceport America Cup's IREC (Intercollegiate Rocket Engineering Competition),
the European Rocketry Challenge (EuRoC), the American Rocketry Challenge (ARC, formerly TARC) and NASA Student Launch
ask a simulation to produce or meet, by section, and what changed from 2026? It serves [M6.3](../ROADMAP.md)'s cited,
dated presets, M6.6's submission packs, [#417](https://github.com/nrdptel/fusionspace-eridanus/issues/417) (airbrake
rules) and the IREC score idea in [competition ideas](ideas-competitions-and-clubs.md). It builds on
[ADR-246](../decisions/0246-the-2026-10-12-review-student-launch-by-requirement-and-oracle-defects.md) (Student
Launch's 14 limits) and [roll and drag control](roll-and-drag-control.md). Read 2026-10-18 from the organizers' own
documents; **Inference** marks this note's own reading. Trust the numbers as printed on that date: every rule set
changes yearly, and two of the four are not final for 2027.

**In short.**
- **IREC's 2027 rules change no simulation number; its 2027 guide isn't out.** The Rules and Requirements v1.0
  (2026-10-12) keep the targets, the 350-point altitude score and the predicted-flight table. Every limit (rail exit
  25 m/s, margin 7.5% to 18% of length) is in the 2026 guide, still current. ESRA has announced that the 2027 guide
  lets control run in the burn and drops the 30° tilt shutoff and the 50% flutter margin.
- **Student Launch 2027 keeps all 14 of ADR-246's limits** (each checked), but descent rose from 90 to 100 s and the
  2026 drift asks (0 to 20 mph, apogee over the pad) are gone. No altitude-score formula is printed.
- **ARC 2027 moves the target** to 800 ft and 37 to 40 s (were 750 ft, 36 to 39 s), with two eggs. It scores the
  altimeter's reading, about 5% below true height at 30 °C: 40 points at 800 ft (**Inference**, handbook formula).
- **EuRoC has no 2027 documents;** 2026's issue 1.1 changed no flight number from 2025. It alone requires a
  simulation file (RocketPy) and airbrakes that fail fully extended.

## 1. IREC 2027 (ESRA)

The Rules and Requirements (R&R) set categories and scoring; the Design, Test & Evaluation Guide (DTEG) sets every
limit. Pages are printed numbers.

- **Targets** (R&R Table 3-1, p. 10): COTS, SRAD solid and SRAD hybrid or liquid each fly to 10,000 or 30,000 ft
  AGL; two-stage to 30,000 or 45,000 ft. 2026: §2.0, p. 8, the same nine categories.
- **Altitude score** (§6.6.1, p. 28): 350 × [1 − |target − actual| ÷ (0.3 × target)] within ±30% of the target:
  350 on target, 233.3 at 10% off; each 1% of the target missed costs 11.7 points. Outside ±30% neither year
  states a value; zero is an **Inference**. 2026 (§2.8.1.4, p. 30) writes the same formula another way.
- **Points** (Table 6-1, p. 27): flight 500 of 1,000 (altitude 350, recovery 150); Barrowman award (Table 6-2).
- **Predicted flight data** (§4.3.10.1, Table 4-1, p. 21): liftoff thrust to weight, rail-departure speed, least
  static margin, peak acceleration, peak speed, fin-flutter speed, target apogee and predicted apogee, plus a
  flight-profile graph and the drogue and main deployment altitudes and descent rates. 2026: §2.6.2.9.1, pp. 18 to 19.
- **New in 2027:** §4.3.5.6 (p. 19) has each major analysis state its assumptions, input uncertainties, margins and
  sensitivity; the 30-point rubric row (Table B-1, p. 35) existed in 2026 (p. 39). No rule asks for Monte Carlo.
- **Payload:** at least 4.4 lb in §3.1.1 (p. 10) but 5.0 lb in §3.4.1.1 (p. 12), passing at 95% of 5.0 lb
  (§3.4.1.5). 2026 said 4.4 lb (§2.3.1). The 2027 text contradicts itself.

From the 2026 DTEG V1.1 (effective 2026-02-23), in force until a 2027 edition ships (DTEG pages, footers):

- **Rail exit** at least 25 m/s (82 ft/s) at the effective rail length, the travel until one button is left on the
  rail (§5.3.1 to 5.3.2, p. 12); rails are 17 ft single-stage and 21 ft two-stage (§5.3.1; solids, §11.1). Elevation
  typically 84° ± 2° single-stage, 87° ± 2° multistage (§10.1.1, p. 30).
- **Stability, as a share of rocket length** (not calibers): at least 7.5% subsonic and 10% supersonic from launch
  to first deployment (§10.2.1); at most 18% at launch and 25% in flight (§10.3.1, p. 30). Evidence is a no-wind
  simulation of the margin in calibers plus the initial margin in percent (§10.2.3). Fin span at least 0.8 calibers.
- **Recovery:** main open no higher than 1,500 ft (457 m) AGL (§6.4.1); drogue 20 to 40 m/s when the main deploys
  (§6.3.3); landing under 11 m/s (§6.4.3), both computed at an 890 m site elevation (p. 18). No drift distance:
  hardware leaving the safe area is lost, and the safety officer may reject a design likely to leave it (§6.2).
- **Active control** (§7, p. 24): only pitch or roll stability augmentation or braking, never guidance (§7.1).
  Neutral (no moment, "aerodynamic surfaces trimmed or retracted", §7.3.2) on an abort, a power loss or past 30°
  from the launch elevation (§7.3); dormant until burnout, max Q, or 6,000 m AGL (30K, 45K) or 2,000 m (10K) (§7.4).
- **Flutter** speed at least 1.5 times peak speed (§8.2.2, p. 26). **Not limited:** thrust to weight (only reported),
  landing kinetic energy, drift distance, dispersion.

**Announced for 2027** (ESRA news, 2026-09-05): control may operate during the motor burn (the max-Q and altitude
gates' fate is unsaid); the 30° shutoff goes (written "30%"); fail-neutral stays; the 50% flutter margin goes, a
flutter analysis stays; stability rules stay. V1.1 sits under "Work-In-Progress For 2027", undated; PDR 2026-12-14.

## 2. NASA Student Launch 2027 (handbook issued 2026-08-05; printed pages)

ADR-246's 14 College/University limits all hold at its numbers and requirement numbers (pp. 11 to 14), among them
apogee 4,000 to 6,000 ft (2.1), margin 2.0 on the pad (2.11), thrust to weight 5.0 (2.12), 52 ft/s at rail exit
(2.14), 75 ft·lbf per section (3.2) and descent at most 100 s (3.11). What ADR-246 doesn't list:

- **Launcher:** 12 ft 1515 rails only, canted 5 to 10° by the day's wind (1.9, p. 6). The printed NAR high-power
  code (p. 44): launcher within 20° of vertical, no launch above 20 mph wind.
- **Altitude score:** set by the target declared at CDR (2.3, p. 11); no formula, only zero points outside 3,500 to
  6,500 ft (2.1). The Altitude Award goes to the closest team (p. 47); the scoring rubric (p. 2) isn't printed.
- **Other points:** up to 15 for vehicle performance (2.6.10, p. 8) and a mass score of 0 above 40 lb, 3 for 30 to
  40 lb and 5 below 30 lb, weighed flight-ready without propellant (2.21, p. 13).
- **Wind and drift:** only the FRR slides' apogee in 5, 10, 15 and 20 mph wind (p. 35). The word "drift" does not
  appear. The 2,500 ft radius from the pads (3.10) and the code's 20 mph limit still bound drift (**Inference**).
- **Second method:** PDR, CDR and FRR ask for "data from a different calculation method" (pp. 22, 27, 31), none
  named. No airbrake rule; flight reports state "air brake system status" if fitted (pp. 32, 36).
- **Undefined:** which thrust 2.12 uses (FRR slides: "launch thrust-to-weight", p. 35); vertical or total landing
  speed for 3.2; how 3.11 applies when sections land apart.
- **High School/Middle School** (pp. 56 to 59, renumbered): apogee 3,500 to 5,500 ft, no award below 3,000 or above
  6,000 ft (2.1, p. 56); at most 2,560 N·s (2.11, p. 57); other limits as above. Not scored (p. 2).

**Changed from 2026** (2026 handbook, 2025-08-07): descent 90 → 100 s in both divisions; the bonuses for under 80 s
and under 65 ft·lbf are gone; the performance and mass points are new. The drift asks are gone: 2026 wanted each
section's drift at 0, 5, 10, 15 and 20 mph "with the assumption that apogee is reached directly above the launch
pad" (2026 pp. 20, 25, 29) and a 15 mph drift at launch readiness (p. 36). The 8 ft 1010 rails and secondary
motors are gone; metallic shot ballast is newly barred (2.20.7.3). The other 13 limits are unchanged.

## 3. American Rocketry Challenge 2027 (rules rev. 27, PDF of 2026-07-17, by rule and sheet)

- **Vehicle** (4.3, sheet 5): liftoff mass at most 650 g; length at least 650 mm; one stage, F or lower from the
  ARC list, at most 80 N·s; two tube diameters, one at least 66 mm and 300 mm long, the others at least 9 mm
  different. **Payload** (4.4): two raw eggs of 55 to 63 g, both uncracked.
- **Score per flight** (4.5, 4.6, sheet 6): |goal − altimeter reading| in feet, plus 4 points per second of
  duration (liftoff to first touchdown) outside 37 to 40 s. Lower is better.
- **Goals** (4.6): 800 ft to qualify, ranked on the best two of up to three flights (4.7); at Finals two different
  goals within 775 to 825 ft, and at least 24 teams fly twice, ranked on the sum (4.12).
- **Rails and control** (4.10, 4.11, sheet 7): at least 6 ft of guidance; autonomous control allowed, no radio.
- **No stability, rail-exit or descent-rate rule;** the NAR model rocket code applies (4.1): launcher within 30° of
  vertical, wind at most 20 mph (handbook p. 32). The handbook's advice, not rules: about two calibers of margin,
  40 ft/s off the rail, thrust five times liftoff weight or 20 N (p. 17).
- **Altimeter reading** (handbook p. 43): altimeters assume 15 °C at the ground, true height = reading ×
  (273.15 + T)/288.15, no post-flight correction, Finals field 600 ft above sea level. **Inference:** a predicted
  score needs the reported altitude: at 30 °C the reading is 288.15/303.15 = 0.951 of true height, 40 ft low at 800.

**Changed from 2026** (rules rev. 26, 2025-06-16): goal 750 → 800 ft; Finals goals 725 to 775 → 775 to 825 ft;
window 36 to 39 → 37 to 40 s; eggs 1 → 2; one diameter of at least 47 mm, nose to fins (3.3) → the stepped rule.
Mass, length, motor class, impulse, guidance and scoring are unchanged; sections 3.x became 4.x.

## 4. EuRoC (Portugal Space; 2026 issue 1.1, 2026-04-30; printed page headers)

No 2027 document is out, and euroc.pt gives no date. Issues came out 2020-06-19, 2021-03-03, 2022-02-04, 2023-02-20,
2024-03-25, 2025-03-07, 2026-04-30: February to May 2027 is likely (**Inference**). Rules (CR) and requirements (SRD):

- **Targets** (CR §4.1, p. 11): 3,000 or 9,000 m AGL, at least 1 kg of payload; at most 40,960 N·s (§4.3, p. 12).
- **Score:** flight performance 350 of 1,000 points (CR §11.1, p. 33). No formula: the award weighs "meters away
  from the target apogee" and the state after recovery, first flight only (§11.4.4, p. 36); §11.4's own example
  says 300. The official apogee is the mandatory CATS Vega's (SRD RQT-0430, p. 31).
- **Launch:** 84° ± 1° elevation, down to 70° if officials require (RQT-1000, p. 62); EuRoC's rail is 12 m (App. I,
  p. 93). **Rail exit** "higher than 30 m/s", shown by simulation, when the last guide forward of the center of
  gravity leaves the rail (RQT-1100, p. 62).
- **Stability:** at least 1.5 calibers for the whole flight after the rail, counting the CG shift and the CP shift
  from Mach 0.5 up (RQT-1030, p. 63); its note warns against "significantly greater than 2" (e.g., over 6).
- **Recovery:** drogue 23 to 46 m/s; main at or below 450 m AGL, descent below 9 m/s (RQT-0360 to 0380, p. 29).
  No landing-energy, drift-distance, thrust-to-weight, Monte Carlo or flutter rule in the public text.
- **Active control** (SRD §7): stable with actuators removed or locked (RQT-0660, p. 45); fail-safe, with airbrakes
  "attempted fully extended in a fault or abort scenario" (RQT-0670 note, p. 46); dormant in boost to 1,500 m AGL,
  500 m with limited flight history, the rail's end with adequate history (RQT-0680, p. 47); App. F1 (p. 87).
- **Simulation** (CR §8.5, pp. 25 to 26): a RocketPy file with the concept and the technical report, all inputs
  included; other software only if its inputs let the organizers reproduce the flight "on RocketPy software".

**Changed:** 2025 to 2026 changed no flight number (change records and a word-level comparison). 2024 to 2025 made
fail-safe a "shall" and restated team rails as 70 to 85° (2024 barred above nominal). 2021 to 2024 dropped a
linear apogee formula (2021 §10.9) for today's unwritten one and raised flight performance from 300 to 350 points.

## 5. Side by side (2027 where out; else the latest)

| Number | IREC (DTEG 2026) | Student Launch 2027 | ARC 2027 | EuRoC 2026 |
|---|---|---|---|---|
| Target apogee | 10k, 30k, 45k ft | declared, 4,000 to 6,000 ft | 800 ft; 775 to 825 | 3,000 or 9,000 m |
| Score formula | linear, zero at ±30% | not printed | ft off + 4/s off | not printed |
| Rail exit | ≥ 25 m/s | ≥ 52 ft/s (15.8 m/s) | none (advice 40 ft/s) | > 30 m/s |
| Static margin | 7.5/10% to 18/25% of length | ≥ 2.0 cal on the pad | none (advice 2 cal) | ≥ 1.5 cal, all flight |
| Thrust to weight | report only | ≥ 5.0 | none (advice 5) | none |
| Descent | 20 to 40 m/s; < 11 m/s | ≤ 100 s; ≤ 75 ft·lbf | all under chute | 23 to 46; < 9 m/s |
| Main deploy | ≤ 1,500 ft AGL | ≥ 500 ft | none | ≤ 450 m AGL |
| Drift | none numeric | 2,500 ft radius | none | none public |
| Airbrake fault | neutral or retracted | no rule | no rule | fully extended |
| Simulation file | none | second method | none | RocketPy |

Margins come in three units, percent of length (IREC), calibers on the pad (Student Launch) and calibers through
the flight (EuRoC): a preset must say which, and when it is measured.

## What this suggests

- **M6.3's presets can cite these sections now, with stale-after dates:** IREC's R&R v1.0 plus the 2026 DTEG
  until the 2027 DTEG ships (by the PDR, 2026-12-14, is an **Inference**); Student Launch 2027 to 2027-08-31 (ADR-246);
  ARC rev. 27 to the 2028 rules (June or July 2027 by past dates); EuRoC 2026 issue 1.1 until 2027's (February to May).
- **M6.6's Student Launch numbers name 2026's drift asks** (0 to 20 mph, apogee over the pad), which 2027 dropped;
  a drift check against the 2,500 ft radius (3.10) still has a basis, but the cited source should change.
- **#417 can say where EuRoC's rule lives:** the SRD's RQT-0670 note, not the Competition Rules. Its IREC defaults
  (retract until burnout, retract past 30°) are announced to go in 2027; fail-neutral stays.
- **An ARC preset needs an altimeter model:** the score uses the reading, about 5% below true height at 30 °C.
- **The IREC score idea holds as written** (§6.6.1, p. 28); 2027's §4.3.5.6 asks for sensitivity and margins,
  which a Monte Carlo report would answer, though no rule requires one.

## Open questions

- The 2027 DTEG's text and date, and whether brakes still retract on a fault (2026 §7.3.2 counts retracted as
  neutral); the post promises only fail-neutral. Tripoli's code update, expected in early 2027, may move more.
- Student Launch's scoring rubric is unpublished; its FAQ, which overrides the handbook (p. 10), touches no flight
  number as read.
- Whether EuRoC 2027 publishes a flight-score formula; the 2021 one, as printed, doesn't reach zero at its own
  ±67% band (**Inference**); 2022 and 2023 rules were not read.
- Which IREC payload minimum (4.4 or 5.0 lb) ESRA means. ARC 2027's title block carries a 2025 revision date.

## Sources

Read 2026-10-18; cited only. NASA's are US government works; ESRA's rules and DTEG say "Creative Commons 4.0" (no
variant); AIA's rules are all rights reserved; EuRoC's bar copying without permission. ESRA below is
https://www.esrarocket.org/s/, AIA https://www.rocketrychallenge.org/wp-content/uploads/, NASA
https://www.nasa.gov/wp-content/uploads/ and EUROC https://euroc.pt/wp-content/uploads/.

1. ESRA, IREC 2027 Rules and Requirements v1.0 rev. A, 2026-10-12: ESRA `IREC-2027-Rules-and-Requirments-v10.pdf`
2. ESRA, IREC Rules & Requirements 2026 v1.0, 2025-10-05: ESRA `IREC-Rules-and-Requirements-Document-2026-v10.pdf`
3. ESRA, IREC Design, Test & Evaluation Guide 2026 V1.1, 2026-02-23: ESRA `2026-IREC-DTEG-V11.pdf`
4. ESRA, 2027 IREC Integrated Master Schedule rev. P1, 2026-09-21: ESRA `2027-IREC-IMS-REV-P1.pdf`
5. ESRA, "DTEG Updates", 2026-09-05, https://www.esrarocket.org/news/dteg-updates, and its documents-and-forms page
6. NASA, Student Launch 2027 Handbook, 2026-08-05: NASA `2026/08/g-739882-2027-sli-handbook-508-aug5.pdf`
7. NASA, Student Launch 2026 Handbook, 2025-08-07: NASA `2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf`
8. AIA, American Rocketry Challenge 2027 Rules rev. 27 (PDF 2026-07-17): AIA `2027_AmericanRocketryChallenge_Rules.pdf`
9. AIA, 2027 Team Handbook v27.0, 2026-07-15: AIA `ARC2027_Handbook.pdf`; 2026 Rules: AIA `ARC-Rules-2026-FINAL.pdf`
10. Portugal Space, EuRoC Competition Rules (CR) and System Requirements Document (SRD) issue 1.1, 2026-04-30:
    EUROC `2026/05/PTS_EDU_EuRoC_ST_001905_CR_v1.1.pdf` and `2026/05/PTS_EDU_EuRoC_ST_001928_SRD_v1.1.pdf`
11. EuRoC CR and SRD issue 1.0, 2025-03-07: EUROC `2025/03/PTS_EDU_EuRoC_ST_001905_CR_v01-1.pdf` and the SRD's twin
12. EuRoC RR and DTEG v05, 2024-03-25: https://2024.euroc.pt/wp-content/uploads/2024/04/; Rules & Requirements v02,
    2021-03-03: EUROC `2021/03/EuRoC_CompetitionRules_Requirements.pdf`; https://euroc.pt/documents/ and /faq/
