# How well a hobby rocket's CG is known, measured and from its design file

The per-kind digits table ([figure-digits-by-kind][digits] §4) has no sourced uncertainty for the CG, so a
stability margin's digits rest on the CP alone. This note sizes both halves for M0.9g's check ([#966][i966]): how
far a balance-point CG scatters, and how far a design file's CG sits from the built rocket's. **Inference** marks
this note's own arithmetic. Read 2026-11-05.

**In short.** A CG balanced in the flight state is good to a few millimeters, but no source repeats a hobby
measurement, so the method rows are physics with stated assumptions; one team's two balance points for one rocket
differ by 30 mm. A design file's CG is the larger error: over 13 public pairs it sits a median 0.23 caliber from the
balance point, from 0.35 caliber forward of the file to 0.96 caliber aft of it. The 11 Student Launch pairs are
centered (median −0.03 caliber, SD 0.21); the 2 hobby kits that came out heavy balance 0.88 and 0.96 caliber aft of
their files, the side that flatters a margin. A margin's error in calibers equals the CG's. From a file CG, with the
CP's own 0.15 to 0.9 caliber, a margin is known to 0.34 to 0.95 caliber (tenths); from a balanced CG, the CP decides.

## 1. What instruments achieve (the floor)

- **CG error = moment error ÷ weight** [4] p. 2; fixturing is "the biggest error in most measurements" [1] p. 7.
  Gas-bearing tables are rated to 25 µm [2]; a rocket-motor CG machine passed acceptance under 0.13 mm,
  repeatability ±0.05 mm, with leveling and fixture tilt the named traps [3] pp. 8 to 9, 20.
- **Load-cell platforms** reach about 1 mm [5] §3.1 (a maker's 75 lb model resolves 0.5 lb-in, 1.3 mm on a 10 lb
  rocket by **Inference**, but lacks a fixed zero [4] p. 2), and a double suspension of a 9.2 kg
  block put 8 of 15 trials under 1 mm and all 15 under 2 mm [6] Table 2.

## 2. What hobby methods achieve

Hobby guides say how to balance, not how well. Apogee balances on a finger or a ruler's edge with a new motor
installed [7] p. 2; a builder's site uses a string for models and a wedge for heavier rockets [8]. Neither states
a precision. The rows below combine each method's terms in quadrature (GUM rectangular limits u = a/√3, resolution
u = 0.29δ [9] §4.3.7, §F.2.2.1) on an invented 98 mm, 2.4 kg rocket with supports 1.0 m apart. Every row is
**Inference**; the finger's contact width and the eye's tilt limit are assumptions, the least certain inputs.

| Method | Terms | u(CG) | caliber at 98 mm | at 25 mm |
|---|---|---|---|---|
| Two CG-gauge modules | ±3 g each [10]: 0.51 mm; stations ±1 mm: 0.41 mm; 1° tilt: 0.86 mm | 1.1 mm | 0.011 | 0.04 |
| Knife edge or wedge | edge ≈1 mm wide; mark read ±1 mm | ≈0.7 mm | 0.007 | 0.03 |
| String loop | tilt judged to ≈1°, CG a radius below the loop | ≈0.9 mm | 0.009 | 0.01 |
| Finger or dowel | contact patch ≈15 mm: 15/(2√3) | 4.3 mm | 0.044 | 0.17 |

- **Lean.** A rocket lying on its skin carries its CG one body radius above the contact line, so a tilt θ moves a
  reaction reading by about a·θ: 0.86 mm per degree at 98 mm (**Inference**; the mechanism is [3] pp. 8 to 9).
- **Light models.** On Apogee's 46.6 g example [7] p. 5, ±3 g modules give 7.9 mm: edges suit models (**Inference**).
- **Observed spread.** Notre Dame 2018 reports two balance points for one rocket, 79.5 in and 78.3 in (30 mm,
  0.16 caliber at 7.74 in), never reconciled [B7] pp. 93, 103. One builder read one mark as 29.0 cm straight back
  and 29.5 cm along the nose curve, a 5 mm bias of method, not a repeat [B12] Part 4.
  Notre Dame 2019 set itself a pass band of ±2 in, about 0.26 caliber [B15 p. 114].

## 3. What a builder handles inconsistently

These state terms outweigh any method's scatter. Δx = m·s/M for a part of mass m moved s in a rocket of mass M
(**Inference**, statics; masses sourced, the rocket invented: 2.2 kg empty, CG 0.95 m from the tip).

| Term | Mass | Move | ΔCG | caliber at 98 mm |
|---|---|---|---|---|
| Motor in or out, 29 mm H128W | 206 g total [11] | 0.55 m aft of CG | 47 mm aft | 0.48 |
| Motor in or out, 38 mm J350W | 665 g total [12] | 0.48 m aft of CG | 111 mm aft | 1.1 |
| Loaded or burned out, H128W | 94 g propellant [11] | 0.55 m aft of CG | 21 mm | 0.22 |
| Parachute packed fore or aft | 193 g, packs 3.2 in long [13] | ±110 mm in a 300 mm bay | ±8.8 mm | ±0.09 |
| Motor axial play; ejection charge | 206 g; ≈1.1 g from W = PV/RT [14] | 5 mm; 1 m | under 0.5 mm | under 0.005 |

- **Motor in or out is a state, not a scatter.** Apogee says to balance with the motor in, the aftmost CG of the
  flight [7] p. 2. A file CG with a motor and a balance point without one differ by up to a caliber, so every
  comparison below pins the state, and a quoted CG should name it.
- **Packing** is the largest repeat-to-repeat term: a rectangular ±8.8 mm is u = 5.1 mm, 0.05 caliber
  (**Inference**); Notre Dame 2018 blames packing for its 1 in file-to-balance gap [B7] p. 103. Cords add to it.

## 4. The design file against the balance point (13 pairs)

A pair is one rocket with a file CG (OpenRocket, RockSim or RocketPy, before any update from the scale), a CG the
source says came from balancing the built rocket, and an outer diameter d. Sign: **measured minus file; positive
means the built rocket balances aft of its file.** Pages are PDF page indices ([as-built-mass-growth][asbuilt]).

| Rocket | d (in) | File CG (in) | Measured (in) | Δ (mm) | Δ (caliber) | Source |
|---|---|---|---|---|---|---|
| NC State 2016 subscale | 4.0 | 45.0 | 44.9 | −2.5 | −0.025 | [B1] pp. 35 to 36 |
| NC State 2017 subscale | 5.15 | 58.30 | 59.5 | +30.5 | +0.233 | [B2] pp. 31 to 32 |
| NC State 2018 subscale | 3.9 | 38.6 | 39.8 | +30.5 | +0.308 | [B3] CDR pp. 52 to 53 |
| NC State 2025 subscale | 4.02 | 34.65 | 33.25 | −35.6 | −0.348 | [B4] pp. 38, 40 |
| NC State 2026 subscale | 3.98 | 46.72 | 46.6 | −3.0 | −0.030 | [B5] pp. 33, 36 |
| NC State 2026 full scale | 6.12 | 64.63 | 63.25 | −35.1 | −0.225 | [B5] p. 53; [B6] p. 146 |
| Notre Dame 2018 | 7.74 | 79.28 | 78.3 | −24.9 | −0.127 | [B7] pp. 21, 103 |
| Notre Dame 2021 | 6.17 | 83.1 | 82.54 | −14.2 | −0.091 | [B8] CDR p. 81, FRR pp. 64 to 65 |
| Notre Dame 2022 | 6.17 | 82.2 | 82.152 | −1.2 | −0.008 | [B9] CDR p. 57, FRR pp. 43 to 44 |
| Toledo 2026 | 5.525 | 63.08 | 63 | −2.0 | −0.014 | [B10] CDR p. 61, FRR p. 105 |
| Toledo 2025 | 6.17 | 82.425 | 84.44 | +51.2 | +0.327 | [B11] CDR p. 75, FRR pp. 17, 67 |
| Quest Sport Loader (kit) | 1.93 | 24.7 cm | 29.0 cm | +43 | +0.88 | [B12], no motor |
| Launch Pad AGM-88 (kit) | 2.60 | file | 2.5 in aft | +63.5 | +0.96 | [B13] #1; [B14] |

- **All 13 (Inference):** median −0.014 caliber, quartiles −0.09 and +0.31 (inclusive method), range −0.35 to
  +0.96, mean +0.14, SD 0.40, median |Δ| 0.23 caliber; median |Δ| in mm 30; 5 aft, 8 forward.
- **Student Launch, 11 (Inference):** median −0.025, quartiles −0.11 and +0.11, range −0.35 to +0.33, mean 0.000,
  SD 0.21, median |Δ| 0.13. Five of the 6 full-scale rockets balance forward of their files.
- **Caveats.** Two schools give 9 of the 11 Student Launch pairs, and teams ballast on purpose (Notre Dame 2021
  36.7 oz, 2022 41.1 oz [B8], [B9]), so their balance points are partly chosen and understate a free build's error.
  NC State 2026's payload flew at 10 lb where earlier simulations held 7.54 lb [B6] pp. 46, 88. NC State 2017's
  59.5 in "was located" and Toledo 2025's 84.44 in sits under "As-Built", neither said to be balanced; without
  them the 9 Student Launch pairs give median −0.03, SD 0.18, median |Δ| 0.09 (**Inference**). Notre Dame 2018's
  p. 93 pairs 80 in with 79.5 in (−0.065); Toledo 2026 balanced to the whole inch; both kit files are builders'.
- **Set apart:** a file-entry error at least 1.5 calibers off [B16]; a scratch build 46 mm off with no diameter
  [B17]; two pairs that don't say whether the file held a motor [B18], [B19]; a 4 in redesign [B20].

## 5. A file updated after the build, and the program's own share

- **An as-built update closes most of the gap.** Three FRR-stage files sit within 0.03 caliber of the balance
  point: NC State 2018 full scale +0.027, "after updating all the component weights" [B3] FRR pp. 11, 94; Notre
  Dame 2019 −0.013 [B15] pp. 13, 114; Notre Dame 2021 within 0.05 in [B8] FRR p. 230 (the last two don't say the
  file held weighed parts). NC State 2026's full scale is +0.075 or −0.139 by which CG is taken [B6] pp. 47, 146.
- **The program moves a file's CG too.** That NC State 2026 report gives three CGs for the as-built rocket, 62.79,
  64.103 and 65.511 in, 0.44 caliber apart; its CDR has OpenRocket 64.629, RocketPy 63.425 and RASAero II 64.505 in,
  0.20 caliber apart [B5] p. 53. Penn State 2018 lists 68.53 and 69.86 in at 5.61 in, 0.24 caliber [B21] slide 5.
  (**Inference**: on a 6 in rocket the program alone is worth 0.2 to 0.4 caliber, so a margin should say whose CG.)

## 6. Sign, mass growth and the flattering side

- **Heavy builds balance aft.** All 3 hobby pairs heavier than their files (+5.5%, +6.8%, +39.6%) balance 43 to
  64 mm aft [B12], [B13], [B17]. For the AGM-88, 3.8 oz that moves the CG 2.5 in sits about 8.8 in aft of the file
  CG, near the fins: fillets, paint and repairs (**Inference**, Δx = a·Δm/(m + Δm)).
- **Size from mass growth (Inference, a model).** The median +7.4% growth of [as-built-mass-growth][asbuilt],
  placed 3 to 4 calibers aft of the CG, moves it 0.074/1.074 × (3 to 4) ≈ +0.21 to +0.28 caliber aft.
- **Which way it errs.** A CG aft of the file makes the true margin smaller than the file's, the flattering side:
  5 of 13 pairs err that way, by up to 0.96 caliber, the two hobby kits the largest.

## 7. What place this gives a margin

Margin = (x_CP − x_CG)/d, so u(margin)² = (u_CP² + u_CG²)/d² for independent terms, d's own error negligible
(**Inference**, GUM first-order propagation [9] §5.1.2). The CP's model error is 0.15 to 0.9 caliber ([digits] §4).

| u_CP (caliber) | CG balanced, 0.01 | packing, 0.05 | as-built file, 0.1 | file only, 0.3 | wrong state, 0.5 |
|---|---|---|---|---|---|
| 0.15 | 0.15 | 0.16 | 0.18 | 0.34 | 0.52 |
| 0.29 | 0.29 | 0.29 | 0.31 | 0.42 | 0.58 |
| 0.9 | 0.90 | 0.90 | 0.91 | 0.95 | 1.03 |

- The ±0.3 caliber file-only figure covers the Student Launch quartiles (not the range, −0.35 to +0.33), between
  their SD (0.21) and the 13-pair SD (0.40); taken as u it is a judgment, not a fitted σ (**Inference**).
- **The CG row.** Balanced in the flight state, a CG supports the millimeter (scales, edge, string) or 5 mm (finger,
  packing): 0.01 to 0.05 caliber at 98 mm, 0.1 for a finger below about 40 mm. From a file it is known to about 0.1
  caliber after an as-built update and 0.3 without: tenths, two figures in inches on a 6 in rocket (**Inference**).
- **The margin row.** With a balanced CG the margin's u is the CP's, so the digits note's rule still decides by the
  CP (hundredths for 0.15 to 0.29, tenths above). With a file CG, u is 0.34 to 0.95: tenths in every case. Apogee's
  example prints a CG of 11.6387 in [7] p. 5, to 2.5 µm, three places past any method here (**Inference**).

## What this suggests

1. Fill the digits table's CG row with "balanced in flight state ≈1 to 5 mm; from a file ≈0.1 caliber after an
   as-built update, ≈0.3 without; place 1 mm, or 0.1 caliber"; a margin from a file CG gets 0.1 caliber.
2. A margin HPR Sim prints from a design file could say that it uses the file's CG, and that a heavy build tends
   to balance aft of it, lowering the margin; a margin from a user's balance point could say which state it is in.
3. The ±0.3 caliber file-CG spread could be a margin dispersion term, skewed aft for hobby builds, once fitted.

## Open questions

1. No source repeats a hobby CG measurement (one rocket, one builder, n ≥ 5); a ten-repeat test of the finger,
   edge, string and two-scale methods would replace every assumed row in §2.
2. 9 of 11 Student Launch pairs come from two schools, and ballast hides their build error. WPI 2020, Northeastern
   2017, Hawaii 2013 and COBRA failed to download; no Spaceport America Cup report with a balance point turned up.
3. No pair sets a maker's own kit file against a stock build with both masses and both CGs, which would separate
   glue and paint from file mistakes. Burnout CGs are balanced only by NC State ([B4] p. 38, [B5] p. 33, [B6] p. 146).
4. SAWE papers 1884, 3494 and 3710 (two-support error analysis) are paywalled; no guide read states a precision.

## Sources (read 2026-11-05; PDFs cached under the gitignored refs/, cite only unless marked)

1 to 4 sit under `https://raptor-scientific.com/content/uploads/`; Rocketry Forum threads (B13, B16 to B19) open at
`https://www.rocketryforum.com/threads/<id>/`, cited by post number. Team reports state no license.

1. Boynton, R., Seven Secrets of Accurate Mass Properties Measurement, SAWE 1992: `2020/10/7Secrets_accurate_MP.pdf`.
2. Raptor Scientific, KSR330 Series CG and MOI datasheet: `2023/04/KSR330-Series-GC-MOI.pdf`.
3. Boynton, R., Weight and three-axis CG of a rocket motor, SAWE 3238: `2020/12/Measuring_W_and_3CG_motor.pdf`.
4. WCG brochure, p. 2: `2020/09/Brochure-WCG-Series-Weight-Center-of-Gravity-Measurement-Instruments.pdf`.
5. Wang, H. et al., Aerospace 13(1) 87, 2026, CC BY. <https://www.mdpi.com/2226-4310/13/1/87>
6. Modenini, Curzi, Tortora, Int. J. Aerospace Eng. 2018, 3582508, CC BY. <https://cris.unibo.it/retrieve/>
   `e1dcb335-0733-7715-e053-1705fe0a6cc9/Tortora.pdf`
7. Apogee Components, Peak of Flight 462, 2018. <https://www.apogeerockets.com/education/downloads/Newsletter462.pdf>
8. Nakka, R., Experimental Rocketry, Appendix D. <https://www.nakka-rocketry.net/Appendix-D.htm>
9. JCGM 100:2008 (GUM). <https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf>
10. SkyRC, CG Gauge SK-500039 manual v1.0, specifications. <https://skyrc.com/files/230?open=1>
11. ThrustCurve.org, AeroTech H128W. <https://www.thrustcurve.org/motors/AeroTech/H128W/>; 12. the same, `J350W/`.
13. Fruity Chutes, Iris Ultra 60" Compact.
    <https://shop.fruitychutes.com/products/iris-ultra-60-compact-parachute-19lbs-20fps-12lbs-15fps>
14. Altimeter Cloud, black powder charge calculator. <https://www.altimetercloud.com/tools/bpcharge_calculator/>

- **B1 to B6, B20** NC State, `https://ncsurocketry.org/wp-content/uploads/` + B1, B2 `2019/09/2016_tacho_lycos_cdr.pdf`
  and `…/2017_…`; B3 `2019/09/nc-state-university-2018-<cdr|frr>-report.pdf`; B4 to B6
  `2025/01/`, `2026/01/`, `2026/03/` + `north-carolina-state-university-<year>-<cdr|frr>-report.pdf`;
  B20 `2021/02/`, `2021/03/` + `…-2021-<…>-document.pdf`.
- **B7, B8, B9, B15** Notre Dame, `https://seniordesign.ee.nd.edu/<year>/Design%20Teams/` + B7
  `rover/documents/Univ-of-Notre-Dame-2018-FRR-Report.pdf`, B8 `rocketpay/cdr.pdf`, `frr.pdf`, B9
  `NDRT/sliCdr.pdf`, `sliFrr.pdf`, B15 `uav/NDRT_FRR_2019.pdf`.
- **B10, B11** Toledo, `https://drive.google.com/file/d/<id>/view`: 2026 CDR `1AXwQjvlWpaoPAtEbah3mWO7fZOqbjb0Z`,
  FRR `1ezggOFl7_czPKkV9FN4CmcrxDhjLwqZP`; 2025 CDR `14o9WFFiH2COyXBUwnC2rLYzfTxnnn-PH`,
  FRR `1yGkgLkR1u9jN4LAGxWTx-l3dWkDJfc-b`.
- **B12** Rocket N00b, 2016. <https://rocketn00b.blogspot.com/2016/03/repairing-and-enhancing-quest-magnum.html>
- **B13** 35852, 2012; **B16** 135471, 2016; **B17** 155421, 2019; **B18** 139875, 2017; **B19** 176444, 2022.
- **B14** RocketReviews, The Launch Pad HARM AGM-88A. <https://www.rocketreviews.com/product-230702223127.html>
- **B21** Penn State 2018 FRR slides.
  <https://sites.psu.edu/psurocket/files/2017/08/Penn-State-2018-FRR-Presentation-1ugzagv.pdf>

[digits]: figure-digits-by-kind.md
[asbuilt]: as-built-mass-growth.md
[i966]: https://github.com/nrdptel/fusionspace-eridanus/issues/966
