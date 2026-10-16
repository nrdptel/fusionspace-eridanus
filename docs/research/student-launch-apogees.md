# How far Student Launch teams' simulated apogees miss their measured ones

**In short: a median +3.4% high per flight, below HPR Sim's +9.47% on the private collection, and not separable
from OpenRocket's +6.82%.** This note collects 55 public NASA Student Launch flights by 10 teams, 2014–15 to
2025–26, each with a measured apogee and a simulated one made with no drag fitted to an earlier flight (simulated
above measured is positive). Per flight the median is **+3.4%** (95% interval +1.1% to +7.2%); 39 of 55 read high;
the mean, +7.7%, is pulled up by flights that weathercocked or flew far heavier than planned. OpenRocket's 41 rows:
**+1.7%** (−0.2% to +4.5%). RockSim's 16: **+8.9%** (+1.4% to +21.0%), 10 of them one team's. By team-season, the
median is +5.05% (+0.15% to +9.1%), against the collection's +9.47% and +6.82% medians
([accuracy](../accuracy.md#real-flights-of-the-private-collection)). **How sure:** high on each row (pages cited;
strict rows re-read in a second pass); low on the pooled median: one team gives 22 of the 55 flights, most
predictions were printed after the flight, readings are raw, and teams ballast toward a target.

## How the rows were read

- **Error** = simulated ÷ measured − 1. **Measured:** the team's primary or official altimeter, or its stated mean
  of two. These are raw barometric readings, not converted to true height in the day's air (see §4).
- **A, strict:** printed in a report dated before the flight (a CDR before the vehicle demonstration flight, VDF;
  an FRR before launch week). **B, day-of:** printed after the flight but labeled the pre-flight or launch-day
  simulation, with no drag fit; it may use the day's wind. A weighed mass or CG override is allowed in both.
- **Refit (R):** drag or a correction factor tuned to an earlier flight or a subscale test of the vehicle.
  **Unclear (U):** refit status or timing not stated. **Set apart:** a drag system flown, ballast or motor changed
  after the prediction, a build defect or a motor anomaly. R, U and set-apart rows are in no statistic.
- **One row per flight:** A where a flight has A and B; OpenRocket where a team ran two tools. **Per tool:** every
  tool's row on each flight (63 rows).
- **95% interval on a median:** the order statistics x(k) and x(n−k+1), with the largest k giving coverage
  1 − 2·P(Bin(n, ½) ≤ k − 1) ≥ 95%. None exists for n ≤ 5. Pages are PDF page indices.

## What the handbook asks

- Teams declare a target apogee (at PDR in 2022 and 2024, at CDR in 2026); the award goes to the smallest gap
  between measured and target. Outside 3,500–6,500 ft (2024, 2026) or 4,000–6,000 ft (2022) scores no altitude
  points [H22 p13; H24 p11; H26 p15, p49]. Ballast is capped at 10% of the unballasted pad weight [H26 p16].
- The CDR simulates the subscale flight in its launch-day conditions and estimates the full-scale Cd from it
  [H22 p30; H24 p28]. The FRR updates the model with launch-day conditions and lists the target, the "predicted
  altitude from simulations", the measured altitude and the ballast flown [H26 p34].
- **The handbook never says when the predicted altitude must be computed,** so most FRR values were printed after
  the flight; hence the A/B split. **Inference:** fitting is built into the process.

## 1. The rows (55 flights)

Teams: NC State (N), Georgia Tech (G), Notre Dame (D), Purdue (U), USF SOAR (F), Toledo (T), FSU Panama City (P),
Explorer Post 1010 (X), WashU (W), FAMU-FSU (M). Flight: s subscale, v VDF, r re-flight, p payload demonstration,
w launch week. Tool: OR OpenRocket, RS RockSim, RPy RocketPy, own = team code. Error in %, then category.

- **2014–15** N s −4.9 (OR B) [N15c 31–32]; v −3.1 (OR A; the flysheet's target, matching its OpenRocket plot)
  [N15c 43, 91, N15f 21]
- **2015–16** N s +74.1 (OR B; 5 mph simulated, about 20 mph flown) [N16c 35–36, N16p 6]; F s +1.1 (OR B)
  [F16c 69–70]
- **2016–17** N s +1.7, s +2.3 (OR B) [N17c 32, N17f 39]; v +3.4 (OR A) [N17c 40, N17f 51]; F s +14.8 (OR B)
  [F17c 37–38]; w +13.6 (OR A) [F17f pr. 44, F17p 3]
- **2017–18** N s −4.2 (OR A) [N18d 55, N18c 61]; v −4.2 (OR B, weighed) [N18f 91–93]; G v +14.8 (OR A; gusts
  to 33 mph) [G18cy 1, G18f 84]; D v −0.2 (OR B), −0.3 (RS B) [D18f 44, 105]; F v −0.2 (OR B) [F18f 67–68];
  third flight +0.5 (OR A; ballast not stated) [F18f 54, F18w]
- **2018–19** N s +21.0 (RS B) [N19c 35–37]; v +17.6 (RS A; weathercocked 15°) [N19c 50, N19f 62]; D v −14.3
  (OR A), −13.6 (RS A) [D19c 31, D19f 63]; F w +4.5 (OR B) [F19p 10]
- **2019–20** N s +19.3 (RS B) [N20c 21, N20f 124]; v +38.7 (RS A; unpainted, 3 lb heavy) [N20c 45, N20f 84,
  87]; D s −8.1 (OR and RS B) [D20c 38–39]; v −2.0 (OR A) [D20c 89, D20f 95]; U s +9.7 (OR B), +10.7 (own B;
  an H114 curve for the H148 flown); v +8.1 (OR B), +2.4 (own B; +5.7% and +0.2% against the FRR's two-unit mean,
  4,588 ft) [U20c 48–49, 72–73, U20f 47–48, 84]
- **2020–21** N s +63.0 (RS B) [N21C 28–29]; v +43.3 (RS A, no ballast) [N21C 40, N21F 100]; D s +3.0 (OR B); v
  +1.1 (OR A), +1.4 (RS A) [D21C 35–36, 80, D21F 85]; W s −7.5 (OR B; main altimeter as the text names it, the
  tables swap the two) [W21C 25–27]; v −7.0 (OR A) [W21C 49, W21F 113]
- **2021–22** N s +17.8, v +10.5 (RS B, raw output) [N22C 34, N22F 83, 86]; D s +0.4 (OR B); v −10.6 (OR A),
  −0.1 (RS B) [D22C 30–31, 52–54, D22F 69]; X v +5.6 (OR B) [X22F 19]
- **2022–23** N s +7.2 (RS B) [N23C 34–35]; v +4.3 (RS A) [N23C 48–49, N23F 153]; D s +2.6 (OR B), +3.1 (RS B);
  v +3.0 (OR A), +7.3 (RS A) [D23C 40, 75, D23F 94]
- **2023–24** N s +0.3 (RPy B) [N24C 41]; v −3.7 (RPy A) [N24C 66, N24F 151]; T s +1.3 (OR B) [T24C 28, 32]; v
  +26.2 (OR A, no ballast) [T24C flysheet, T24F 73]; r +9.3 (OR A, weighed) [T24F flysheet, T24Ad 4]; M s +4.2
  (OR B); v +13.8 (OR A) [M24C, M24F §5.1]
- **2024–25** N s +3.7 (RPy B) [N25C 39–40]; v +14.5 (RPy B, air brakes idle) [N25F 95, 183]; T s +13.9 (OR B)
  [T25C 50, 54]
- **2025–26** T s +6.5 (OR B) [T26C 46, 49–50]; P s +5.0, s −3.0, v −2.1, p −7.7 (OR B) [P26C 45–46,
  P26F 96, P26Ad 6–7]

**Refit (six flights, median −0.8%; refitting didn't reliably help):** NC State 2015 launch week −9.7%
[N15f 28, N15p 4], 2021 re-flight +2.8% (Cd 0.634 fitted) [N21Ad 7, 26], 2022 subscale +0.1% (the team's ×0.85
RockSim factor) [N22C 34], 2024 re-flight +3.6% [N24Ad 6, 12, 14]. Notre Dame's 2020 VDF in its own code, −1.6%,
used a Cd from CFD and subscale tests [D20c 89–90]. Toledo's 2026 VDF read −23.4%: it carried Cd 0.800 from its
subscale to full scale, predicted 4,078 ft and reached 5,327 ft; the flight fitted 0.29 [T26F 99, 106–107].
**Unclear:** NC State 2019 re-flight, +29.6%, after a Cd fit to the VDF that the addendum doesn't say it used
[N19a 5, N19f 64]; NC State 2022's "pre-flight" 3,800 ft, +15.4% [N22F 83, 86]; FAMU-FSU 2024's second VDF and
re-flight, −3.6% and +0.7% [M24F §5.2, M24Ad]; FSU Panama City's launch week, −3.0%, run with the flight
computer's own weather after a drag fit [P26PL 7–8].

**Set apart:** drag systems flown (NC State 2016 and 2025–26; Georgia Tech's 2018 subscale, +20.4%, whose system
misfired, flown without its rover system [G18c 69, 72]; Notre Dame 2019 re-flight, 2020 payload flight, 2022
flight 2; Toledo 2025 VDF). Ballast, motor, parachute or payload changed after the prediction (NC State 2017
launch week, the 2018 and 2022 VDFs' CDR values, the 2019 FRR value for the re-flight, +30.7%, the 2021
re-flight; Purdue 2020's CDR value; Notre Dame 2018's; USF 2017 VDF, +55.7%, and 2018 flight 1, +20.0%). Post
1010's 2022 launch flight (a planned weight cut). Motor anomalies (NC State 2024 and Toledo 2024 launch weeks). A
warped nose cone (FAMU-FSU 2024 second subscale, +22.3%). Wind tuned after the flight (Notre Dame 2023). The only
RASAero II pairs are on NC State's 2025 launch week, a slow ignition with air brakes closed: +14.9% and +15.1%
[N25PL 7, 9; N25F 51].

## 2. Statistics

| set | n | median | 95% interval | ranks (coverage) | high |
|---|---:|---:|---:|---|---:|
| one row per flight | 55 | +3.4% | +1.1% to +7.2% | 20, 36 (97.0%) | 39 |
| every tool's row | 63 | +3.0% | +1.1% to +5.6% | 24, 40 (95.7%) | 44 |
| OpenRocket | 41 | +1.7% | −0.2% to +4.5% | 14, 28 (97.2%) | 26 |
| RockSim | 16 | +8.9% | +1.4% to +21.0% | 4, 13 (97.9%) | 13 |
| RocketPy; own code | 4; 2 | +2.0%; +6.5% | none | — | 3; 2 |
| strict (A) only, every tool | 23 | +3.0% | −3.1% to +13.6% | 7, 17 (96.5%) | 15 |
| strict, OpenRocket | 15 | +1.1% | −4.2% to +13.6% | 4, 12 (96.5%) | 9 |
| strict, RockSim | 7 | +7.3% | −13.6% to +43.3% | 1, 7 (98.4%) | 6 |
| per flight, without NC State | 33 | +2.6% | −0.2% to +5.6% | 11, 23 (96.5%) | 22 |
| team-season medians | 30 | +5.05% | +0.15% to +9.1% | 10, 21 (95.7%) | 21 |
| team medians | 10 | +5.7% | −2.55% to +9.3% | 2, 9 (97.9%) | 8 |

- **Means:** +7.7% per flight; +4.0% on OpenRocket's rows; +15.0% on RockSim's.
- **Team medians:** NC State +5.8% (22 flights), Georgia Tech +14.8% (1), Notre Dame +0.1% (10), Purdue +8.9% (2),
  USF +2.8% (6), Toledo +9.3% (5), FSU Panama City −2.5% (4), Post 1010 +5.6% (1), WashU −7.2% (2) and
  FAMU-FSU +9.0% (2).
- **Tool is confounded with team.** Ten of RockSim's 16 rows are NC State's, 2018–19 to 2022–23: a weathercocked
  flight, an unpainted heavy one, and a subscale that reached 61% of its simulation. On Notre Dame's six flights
  run in two tools, OpenRocket and RockSim differ by 0.1 to 10.5 points (median 0.6); the 10.5 sets a CDR value
  beside a launch-day one.
- **Subscale and full scale:** 26 subscale flights +3.4% (+1.1% to +9.7%); 29 full-scale +3.4% (−2.0% to
  +10.5%). By era: 2014–20, 25 flights, +2.3% (−0.2% to +13.6%); 2020–26, 30 flights, +4.0% (+1.1% to +7.2%).

## 3. Against the private collection

**The answer: not as high as HPR Sim's figure, though only narrowly once teams are clusters; not separable from
OpenRocket's.** The collection's medians are +9.47% (HPR Sim) and +6.82% (OpenRocket) on 53 flights, its means
+8.73% and +7.79% ([fixture flights](../../validation/reports/fixture-flights.md)).
- **Per flight:** +1.1% to +7.2% excludes +9.47% and holds +6.82%. OpenRocket's own rows, −0.2% to +4.5%, sit
  below its collection median, but those rows aren't grouped by team.
- **By team-season** (+0.15% to +9.1%) **and by team** (−2.55% to +9.3%): both exclude +9.47% by less than half a
  point and hold +6.82%. The means are close: +7.7% against +8.73% and +7.79%.

**Inference:** the sign is shared and the typical error is smaller. Three differences push Student Launch errors
toward zero: most B rows fly a weighed mass and the day's wind; teams ballast toward a declared target; and many
predictions were printed after the flight. The 23 strict rows alone can't separate +3% from +9%.

## 4. What moves these numbers

- **Raw readings against true height.** The collection compares with the height climbed in the day's ERA5 air.
  There, height over reading averaged 1.0360, from 0.9601 to 1.0777, on 53 flights. These rows compare with raw
  readings. In air warmer than the standard atmosphere the true height is above the reading; in colder air,
  below. **Inference:** most subscale flights and VDFs here flew in winter (19 °F [T24F 73], 36 °F [U20f 84],
  43 °F [N21F 100]), which would raise their errors; a hot launch week (91.8 °F [P26PL 7]) would lower them. The
  net sign is unknown until each day's air is read.
- **Which altimeter:** NC State 2015's VDF read 3,345 ft on its StratoLogger and 2,799 ft on two other units
  [N15f 21–22]; against the CDR's 3,240 ft the error moves from −3.1% to +15.8%. Purdue 2020's two units differ
  by 4.5% [U20f 48].
- **Weathercocking and mass:** the largest high errors are a weathercocked subscale (+74.1%) and mass
  underestimates. NC State's 2021 subscale reached 61% of its simulation, every section but one heavier than
  expected (+63.0% [N21C 29]); its 2020 VDF flew unpainted and 3 lb heavy (+38.7% [N20f 87]). Teams simulate a
  steady average wind. Sign: high. Not sized here. See also [as-built mass growth](as-built-mass-growth.md) §1.
- **Drag, both ways:** fits found more drag than the tool assumed for NC State 2021 (0.634 against 0.37
  [N21F 104]) and Toledo 2024 (0.589 against 0.479 [T24Ad 19]), so the prediction read high. Toledo 2026 and
  FSU Panama City 2026 found less (0.29 against 0.800; 0.211 against 0.367 [P26Ad 28]), so it read low.

## What this suggests

- M7.5 and ADR-209 §6 can cite a public figure beside the collection's: Student Launch first-prediction errors run
  +3.4% median per flight (+1.1% to +7.2%), +5.05% by team-season. Quote it with §3's caveats.
- A team-clustered interval is the honest one for any real-flight set where a few teams supply most flights. The
  per-flight interval here spans 6.1 points; the team-season one spans 8.95.
- Converting these rows' readings to true height in each day's reanalysis air would size §4's raw-reading term.
- No team here published a design file, so no flight joins the public real-flight set (M2.6). UND's repositories
  ([UND flight configurations](und-flight-configs.md)) remain the only public Student Launch designs found.

## Open questions

- **Which B rows predate their flights?** Most can't be dated. The 23 strict rows alone can't separate +3% from +9%.
- **Unread or missing:** NASA's launch-week summaries (per-team apogees?), USF's 2016 and 2019 FRRs, Purdue's 2018
  FRR (HTTP 500), Georgia Tech after 2018. River City Rocketry, Vanderbilt, UAH, Minnesota, NU Stars and Cyclone
  Rocketry gave no reports; Explorer Post 1010's 2015–2021 folders return 404.

## Sources (read 2026-10-16)

No team report states a license; Purdue's site says "All rights reserved". All are cited, not committed; reading
copies stay in the gitignored cache. NASA's handbooks are US-government works.

1. **N** NC State Tacho Lycos, <https://ncsurocketry.org/wp-content/uploads/>. 2015–17: `2019/09/<yyyy>_tacho_lycos_`
   `{cdr,frr,plar}.pdf` (N15c, f, p …). 2018: `2019/09/nc-state-university-2018-{pdr,cdr,frr}-report.pdf`.
   2019: `2019/10/north-carolina-state-university-2019-{cdr-document,frr-addendum}.pdf`, `2019/09/…-2019-frr-`
   `document.pdf`, `2019/08/…-2019-plar-document.pdf`. 2020: `2020/01/…-2020-cdr-report.pdf`,
   `2020/03/…-2020-frr-document.pdf`. 2021–26 (capital letters: N21C, N21F, N21Ad) via the season pages.
2. **G** Georgia Tech, 2018 PDR, CDR, CDR flysheet, FRR: <http://usli.gatech.edu/documentation>.
3. **D** Notre Dame, <https://seniordesign.ee.nd.edu/>: `2018/Design%20Teams/rover/documents/`,
   `2019/Design%20Teams/uav/`, `2020/Design%20Teams/rocket/docs/`, `2021/Design%20Teams/rocketpay/`,
   `2022/Design%20Teams/NDRT/`, `2023/DesignTeams/rocket/` (CDR, FRR, 2019 addendum).
4. **U** Purdue 2020 CDR and FRR, <https://purdueseds.space/wp-content/uploads/psp-sl/casper/>.
5. **F** USF SOAR, archived: <https://web.archive.org/web/2022id_/http://www.usfsoar.com/wp-content/uploads/>
   `2016/01/SOAR_NSL_CDR_Final.pdf`, `2017/01/CDR-Report-2017.pdf`, `2017/03/FRR-Report-2017.pdf` (partial copy),
   `2016/09/PLAR-Report-2017.pdf`, `2018/03/Univ-of-S-Florida-2018-FRR.pdf`, `2019/04/PLAR-Final.pdf`. F18w: team
   post of 2018-03-19, <https://web.archive.org/web/20231128165645/https://www.usfsoar.com/projects/nsl-2017-2018/>.
6. **T** Toledo, <https://www.utrocketry.club/projects/nasa-usli-2024> (and `-2025`, `-2026`).
7. **P** FSU Panama City, <https://web1.eng.famu.fsu.edu/me/senior_design/2026/team533pc/files/>.
8. **M** FAMU-FSU, <https://eng-web1.eng.famu.fsu.edu/me/senior_design/2024/team502/documents/NASA/> (cited by section).
9. **X** Explorer Post 1010, <https://post1010.org/Rocketry/Rocketry2022/SLI2022/>.
10. **W** WashU, <https://sites.wustl.edu/wurocketry/files/2021/>: `02/Washington-University-in-St.-Louis-2021-CDR-`
    `Report.pdf` and `03/…-2021-FRR-Report.pdf`.
11. **H** NASA Student Launch handbooks, <https://www.nasa.gov/wp-content/uploads/>: 2022
    `2016/03/2022_nasa_students_launch_handbook.pdf`; 2024 `2023/08/2024-sli-handbook-final-copy1.pdf`; 2026
    `2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf`.
