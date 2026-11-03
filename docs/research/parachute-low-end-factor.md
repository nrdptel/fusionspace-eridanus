# How far HPR mains fly from the maker's drag area: Student Launch flights

What this covers: measured main-parachute descents in public NASA Student Launch reports, each turned into a drag
area and divided by the drag area the maker's own descent table gives for that canopy. It is for M6.6's landing
energy and descent time, M8.1's auto-sizing and issue #741, which need to know how far a real main sits from its
table. How far to trust it: 25 flights from five teams, 15 from one team; 14 speeds read here from plots; masses are
the teams' own and sometimes disagree between documents (the checking pass, re-reading every number against its
source, replaced four design-table masses with the flights' own). The medians are a lead; the low ends are the
lowest flights seen, not bounds. Builds on [hobby-parachute-cd.md](hobby-parachute-cd.md) (each maker's table drag
area, §1; the few independent tests, §2 and §4). Researched 2026-11-02.

**In short.** The answer depends on the maker. **Fruity Chutes Iris Ultra** mains (17 flights) flew *above* their
table: of the 8 flights with no fault reported and a ratio inside the maker's own log, the median is **1.21** (9%
slower than rated) and the range **0.90 to 1.50**; five more read above 1.5. **SkyAngle CERT-3 and Rocketman High
Performance CD 2.2** mains (7 flights) all flew below their tables: the 5 with no stated fault sit at **0.54 to
0.85**, median 0.74. Deployment faults (part inflation, a fouled harness, twisted lines) gave **0.37 to 0.57**, and
one tangled second canopy 1.63. **No flight of a Classic Elliptical, Rocketman Standard, Top Flight or Spherachutes
main was found**: in these reports the Classic Elliptical flies only as a drogue or on a nose cone.

## Method

- **Table drag area**, from the maker's stated load W at its stated speed v: C_D·S = 2W/(ρ0·v²), ρ0 = 1.225
  kg/m³, which is 0.19543 m² per pound rated at 20 ft/s. Rows used (20 ft/s): Iris Ultra 60 in 19 lb, 72 in 28,
  84 in 39, 96 in 50, 120 in 79 (Standard and Compact rated alike) [1]; Classic Elliptical 12 in 0.5, 15 in 0.8,
  18 in 1.2, 24 in 2.2 [1]; SkyAngle CERT-3 XL 45.2 lb (8.83 m²), XXL 83.1 lb (16.24 m²), Classic 24 in 1.4 lb
  [2]; Rocketman HP CD 2.2 120 in 82.3 lb (16.08 m²), 144 in 118.6 lb (23.18 m²) [3].
- **Measured drag area**, C_D·S = 2·m·g/(ρ·v²): m the mass under the main after burnout (the report's own
  sections, or its liftoff mass less propellant from ThrustCurve [5]); v the steady main descent speed; ρ from the
  report's pressure and temperature, else ISA pressure at the site (Bragg Farms, Toney, AL 790 ft; Bayboro, NC 14 ft
  [11]) with the stated temperature. Ratio = measured / table.
- **Drogues stay on the harness** in every flight. The Iris rows add the drogue's table area to the table side;
  the others subtract it from the measured side, as a range from none to all of it. With drogues of under 5% of
  the main's table area, the two differ by at most 0.03 on these flights (computed here).
- **Plot reads**: page rendered at 200 to 400 dpi, both axes calibrated on two or more labeled gridlines or ticks,
  the trace isolated by color, a straight line fitted over the steady main segment (after the opening, before
  touchdown) in several windows; the window spread is the speed range given. Ratio goes as 1/v², so a ±3% speed
  read is ±6% in the ratio.
- **Never counted**: a team's prediction, or a simulator's "ground hit velocity" restated as a result.

## 1. Iris Ultra mains

Speeds in ft/s, masses in lb, C_D·S in m². "p" = read here from a plot, "t" = the team's stated measured value.
Canopies: I = Iris Ultra, E = Classic Elliptical, A36 = Apogee 36 in flat drogue. All ratios computed here.

| flight [source, pages] | canopies | mass | v | ρ | C_D·S | table | ratio | note |
|---|---|---|---|---|---|---|---|---|
| Penn State 2017 PLAR [6, §4, §5, Fig. 2] | I72, E12 | 37.1 | 24.4 p | 1.197 | 4.99 | 5.57 | 0.90 | a |
| NC State 2018 FRR [7, pp. 49 to 51, 83 to 87] | I120, E18 | 38.75 | 10.9 p | 1.225 | 25.5 | 15.7 | 1.63 | b |
| NC State 2019 FRR [7, pp. 10, 53, 59 to 63] | I84, E24 | 33.8-35.0 | 16.0 p | 1.277 | 10.1 | 8.05 | 1.23-1.27 | c |
| NC State 2019 FRR addendum [7, pp. 11 to 14] | I84, E24 | 35.0-36.3 | 15.3 p | 1.228 | 11.9 | 8.05 | 1.45-1.50 | d |
| NC State 2019 PLAR [7, p. 4] | I84, E24 | 35.0-36.3 | 18.5 p | 1.197 | 8.33 | 8.05 | 1.02-1.05 | c |
| NC State 2020 FRR [7, pp. 32, 84 to 87] | I120, E24 | 43.2 | 11.2 p | 1.245 | 26.5 | 15.9 | 1.67 | e |
| NC State 2021 FRR [7, pp. 34, 90 to 93] | I120, I60, E18 | 45.6 | 18.6 p | 1.290 | 9.79 | 19.4 | 0.50 | f |
| NC State 2022 FRR [7, pp. 43, 68 to 76] | I60, E18 | 21.1 | 33.1 p | 1.266 | 1.46 | 3.95 | 0.37 | g |
| NC State 2023 FRR [7, pp. 140 to 145] | I120, E15 | 38.4 | 13.9 p | 1.230 | 15.5 | 15.6 | 0.99 | h |
| NC State 2024 PLAR [7, pp. 1 to 6] | I96, E15 | 23.9-30.4 | 12.99 t | 1.168 | 11.6-14.8 | 9.93 | 1.17-1.49 | i |
| NC State 2024 FRR addendum [7, pp. 3 to 15] | I96, E15 | 23.9 | 10.62 t | 1.169 | 17.4 | 9.93 | 1.75 | e, i |
| NC State 2025 FRR [7, pp. 77 to 80] | I96, E15 | 38.7 | 16.4 t | 1.183 | 11.7 | 9.93 | 1.17 | j |
| NC State 2025 FRR addendum [7, pp. 6, 7] | I96, E15 | 36.2 | 13.1 t | 1.167 | 17.3 | 9.93 | 1.74 | e |
| NC State 2025 PLAR [7, pp. 7, 8] | I96, E15 | 36.0 | 14.8 t | 1.183 | 13.3 | 9.93 | 1.34 | |
| NC State 2026 FRR [7, pp. 77, 82] | I96, E15 | 36.5 | 15.7 t | 1.218 | 11.6 | 9.93 | 1.17 | |
| NC State 2026 PLAR [7, p. 6] | I96, E15 | 36.65 | 12.4 t | 1.145 | 19.9 | 9.93 | 2.01 | e, k |
| Georgia Tech 2018 FRR [8, pp. 58 to 82] | I96, A36 | 35.4 | 13.2 p | 1.209 | 16.1 | 10.3 | 1.57 | e, l |

- a. Liftoff 692 oz less the L1170's 2.80 kg of propellant [5]; the report says excess mass was found late (the
  ratio would rise). It names a 72 in main in its summary and an 84 in later (0.65 with the 84 in).
- b. Measured 44.3 lb (p. 84) less L2200G propellant. The 60 in Iris released at 700 ft "was never able to fully
  inflate" and tangled in the main's shock cord: off the table side; its partial drag likely explains 1.63. A fault.
- c. Mass is the FRR's 39.2 lb takeoff weight (pp. 10, 53) less the L1150R's 1,902 g of propellant [5]; whether
  it holds the 1.25 lb ballast flown later is unclear, hence the ranges. The section table (28.2 lb, p. 59) gave
  1.02 and 0.85. On the FRR flight the main opened at apogee (shear pins failed); slope over the last 1,100 ft.
- d. As c; the slope changes within the segment (12.8 then 17.6 ft/s); the team gives "near the previous 17 fps".
- e. Above 1.5: beyond the maker's own log, whose highest row (3.22) is 1.46 to 1.54 of the table-implied 2.10 to
  2.21 [12]. A mass, speed or air-motion error is likelier than the canopy.
- f. The payload's 60 in Iris failed to release and came down fouled on the main's harness. A fault.
- g. Two sections; the fin can's 60 in Iris knotted and didn't slow it (not counted). Which section the plotted
  trace records is inferred; likely part inflation. A fault.
- h. Mass from the flight's own budget, 42.48 lb at liftoff (Figure 112, p. 145) less the L1520T's 1,854 g of
  propellant [5]. The design table's 0.935 slug (30.1 lb, p. 43) would give 0.78.
- i. The PLAR's 23.85 lb "at landing" disagrees with its own 48.8 lb field weight (p. 1): less propellant and the
  separately descending nose-cone side, that leaves about 30.4 lb (computed here). The addendum's 73.5 s descent
  with a 110.7 ft/s drogue implies about 16 ft/s under the main, not 10.62.
- j. "Velocity at landing" over the last 50 logged points; the report notes a "temporary deflation of the main
  parachute and slight tangling of the shroud lines" (p. 79). Counted as a fault.
- k. The text gives 12.4 ft/s as the 75 to 95 s average, before the touchdown at about 100 s; Table 2.3's columns
  label 12.4 "predicted" and 16.8 "actual", which the text and the plot contradict (with 16.8, 1.09).
- l. Liftoff 39.73 lb less the L1390G's propellant [5]; the drogue is an Apogee 36 in at 0.75 on its circle.

| Iris set | n | median | range | from plots |
|---|---|---|---|---|
| all (range midpoints) | 17 | 1.33 | 0.37 to 2.01 | 10 |
| no fault (b, f, g, j) and not above 1.5 (e) | 8 | 1.21 | 0.90 to 1.50 | 6 |

The eight, at range midpoints: 0.90, 0.99, 1.03, 1.17, 1.25, 1.33, 1.34, 1.47. In speed, 0.90 is 5% faster than
the table (1/√0.90 = 1.054), with 11% more landing energy (energy goes as 1/ratio); 1.50 is 18% slower, and the
descent 22% longer (√1.50). The median sits near the maker's own Iris log, whose median 2.53 is about 1.15 to 1.20
of the table-implied 2.10 to 2.21 [12]. Below 1 are only NC State 2023 (0.99) and Penn State (0.90, note a).

## 2. SkyAngle CERT-3 and Rocketman HP CD 2.2 mains

Main-only ratio (drogue table area subtracted); range in brackets carries mass, speed, density and drogue.

| flight [source, pages] | main | mass | v | ρ | predicted v | ratio | note |
|---|---|---|---|---|---|---|---|
| JMU 2022 VDF [9, FRR pp. 10, 59, 60] | CERT-3 XL | 41.2 | 25.5 p | 1.201 | 17.8 to 20.7 | 0.54 [0.50, 0.59] | m |
| JMU 2022 re-flight [9, FRR add. pp. 4 to 6] | CERT-3 XXL | 45.4 | 16.5 p | 1.208 | 15.9 | 0.80 [0.67, 0.90] | |
| JMU 2022 competition [9, PLAR pp. 4, 5] | CERT-3 XXL | 41 to 45 | 16.1 p | 1.14 | none | 0.85 [0.63, 1.04] | |
| NC State 2026 re-flight [7, add. pp. 17 to 22] | CERT-3 XL | 37.65 | 25.3 t | 1.077 | 14.9 | 0.57 [0.54, 0.60] | n |
| JMU 2026 re-flight [9, FRR add. pp. 4 to 27] | HP 144 in | 54.3 | 16.79 t | 1.094 | 13.40 | 0.71 [0.68, 0.73] | |
| JMU 2026 competition [9, PLAR pp. 4 to 9] | HP 144 in | 54.1 | 16.0 t | 1.153 | 13.4 | 0.74 [0.72, 0.76] | |
| FSU Panama City 2026 [10, PLAR pp. 5 to 8] | HP 120 in | 48.7 | 27 t | 1.134 | 16.42 | 0.34 [0.33, 0.35] | o |
| NC State 2017 [7, PLAR pp. 5, 6] | 10 ft elliptical | 41.75 | 30.5 p | 1.144 | none | 0.36 [0.31, 0.41] | p |

- m. The team gives 20 to 24 ft/s for the steady main (ratio 0.62 to 0.90); the plot's time axis is a category
  axis, so the plot read is the weaker one. The team blames product data that "did not differentiate between the
  parachute's 2D and open area" (FRR p. 16).
- n. Lines twisted, canopy "not able to fully inflate"; v is the mean of two altimeters (25.2, 25.4). Density
  assumes the Monterey, VA field (2,675 ft).
- o. The report's table is inconsistent: 27 ft/s main, 11.3 ft/s landing, and 548 ft in 15.2 s (36 ft/s). Not
  used for a low end.
- p. Maker not named; Rocketman's 120 in elliptical table gives 9.77 m² [4] and Fruity Chutes' calculator about
  9.8 to 10.1 m² for its 120 in (hobby-parachute-cd.md §1). Cause of 30 ft/s unexplained.

| set | n | median | range |
|---|---|---|---|
| SkyAngle CERT-3 XL and XXL | 4 | 0.68 | 0.54 to 0.85 |
| Rocketman HP CD 2.2 | 3 | 0.71 | 0.34 to 0.74 |
| no stated fault (rows 1, 2, 3, 5, 6) | 5 | 0.74 | 0.54 to 0.85 |

All eight sit below the table. In speed, 0.74 is 16% faster than the table and 0.54 is 36% faster; in landing
energy, 1.35 and 1.85 times. The two clean CERT-3 XL rows (0.54 and 0.57, one with twisted lines) sit below the
XXL rows (0.80, 0.85); with two flights each, that may be the flights rather than the sizes. JMU flew a 144 in
Iris and then a 144 in Rocketman HP, both sold at C_D 2.2; the team found the Rocketman's inflated shape more
trapezoidal and its descent faster than its Iris-based 13.4 ft/s prediction [9, 2026 PLAR p. 8].

Excluded: FSU Panama City's re-flight (main collapsed and reopened low) and first flight (reefing failed) [10];
NC State 2024's 48 in Classic Elliptical on a nose cone whose mass changes mid-descent [7]; UCF 2023 (no
descent mass) [13].

## 3. Causes of spread, with sign and size

- **Mass bookkeeping.** Ratio ∝ m, and the largest cause found. A design or section table in place of the
  flight's own weight moved four rows by 20 to 27% (notes c, h), and one report's two masses differ by 27% (note
  i). Every such slip read low, so a report's section table understates the ratio.
- **Density.** Ratio ∝ 1/ρ. ±10 °C of unstated temperature is ±3.5%; sea-level density at the 790 ft Huntsville
  pad instead of site density would bias a ratio 2 to 7% low.
- **Speed.** Ratio ∝ 1/v². Plot reading ±4 to ±10% in the ratio; window choice up to 0.92 to 1.74 (note d).
- **Air motion.** A barometric log gives speed over the ground. At 4 m/s descent, ±0.5 m/s of rising or sinking
  air moves the ratio by ×1.31 or ×0.79 (computed here, (v/(v − w))²); no report gives it, and it can explain
  either tail.
- **Faults.** Part inflation, fouling and twisted lines (0.37, 0.50, 0.57) are failure modes, not a low C_D;
  a second canopy that tangled but dragged (1.63) errs the other way.

## What this suggests

- **One low-end factor across makers would be wrong.** On these flights, the table drag area is about right or
  conservative for the Iris Ultra (clean flights 0.90 to 1.50) and optimistic for the SkyAngle CERT-3 and
  Rocketman HP (0.54 to 0.85). A catalog entry could carry a measured low-end factor per family, with its n.
- **Landing energy (M6.6) and fast-end sizing (M8.1)**: on this sample, a factor near 0.9 for the Iris table
  and near 0.55 for the CERT-3 and Rocketman HP would cover every flight without a stated fault; landing energy
  1.11 and 1.8 times the table's. Faults (0.34 to 0.57) are a separate case, with their own odds.
- **Descent time and drift** err the other way: a slow descent is the flattering side. The Iris's median 1.21 and
  high 1.50 mean descent 10 to 22% longer than the table gives, consistent in sign with #741's speed effect.
- **A report's mass needs the flight's own weight.** Four of 17 Iris rows first used a design table; a descent
  fit in the analyzer should take the weighed flight mass and say which it used.
- **The Classic Elliptical main has no measurement here.** A flyer-log descent fit (hobby-parachute-cd.md's last
  suggestion) is still the way to get one.

## Open questions

- 15 of the 17 Iris flights are one team's, five of them above 1.5; whether that is mass records, averaging
  windows or air motion can't be told from the reports. The NC State 2020 FRR says the vehicle "came in 3 pounds
  heavier than expected" (p. 87); if that isn't in its mass, the ratio is up to 1.79.
- Not reached: Florida's 2024 FRR (96 in Iris, two flights; dead link), Penn State's 2018 to 2020 reports (site
  gone), JMU's 2025 reports (sign-in), Spaceport America Cup reports, and builder reviews on rocketreviews.com.
- Whether a drogue in the main's wake adds its table drag is not known; subtracting all of it may over-correct.

## Sources

All read 2026-11-02. Team reports and maker pages state no license: cited only, cached privately.

1. Fruity Chutes store product list (titles carry the 20 ft/s ratings): https://shop.fruitychutes.com/products.json
2. b2 Rocketry, SkyAngle descent chart: http://www.b2rocketry.com/PDF%20files/descent%20chart%2010-02.xls.pdf
3. The Rocketman, High Performance CD 2.2 parachutes:
   https://www.the-rocketman.com/products/rocketman-high-performance-cd-2-2-parachutes
4. The Rocketman, Elliptical parachutes: https://www.the-rocketman.com/products/rocketman-elliptical-parachutes-1
5. ThrustCurve.org motor search API (L1150R, L1170FJ, L1390G, L1420R, L1520T, L1940X, L2200G):
   https://www.thrustcurve.org/api/v1/search.json
6. Penn State LionTech, 2017 PLAR: https://sites.psu.edu/psurocket/files/2018/04/ltrl_2017_plar-ruubzr.pdf
7. NC State Tacho Lycos, Student Launch reports (2017 to 2026 FRRs, addenda and PLARs), from the team's archive:
   https://ncsurocketry.org/report-archive/
8. Georgia Tech, 2018 FRR, from https://usli.gatech.edu/
9. JMU Rocketry, 2022 FRR, FRR addendum and PLAR; 2026 FRR addendum and PLAR:
   https://sites.google.com/view/jmu-rocketry/documents
10. FSU Panama City (team 533), 2026 FRR, FRR addendum and PLAR:
    https://web1.eng.famu.fsu.edu/me/senior_design/2026/team533pc/
11. USGS Elevation Point Query Service: https://epqs.nationalmap.gov/v1/json
12. Fruity Chutes, Iris Ultra Cd performance log, as re-counted in hobby-parachute-cd.md §2:
    https://fruitychutes.com/help_for_parachutes/iris-parachute-cd-performance-log
13. UCF Knights Experimental Rocketry, 2023 PLAR, p. 10: https://www.kxrucf.com/
