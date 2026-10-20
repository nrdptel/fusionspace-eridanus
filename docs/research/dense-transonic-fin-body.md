# A dense, tabulated transonic test of a finned rocket

Between Mach 0.8 and 1.2, HPR Sim's normal-force slope (C_Nα) runs up to +27.1% high against NASA's Arcas Robin
tunnel tests, and its center of pressure (CP) up to 2.36 calibers off
([aero](../physics/aero.md#normal-force-through-mach-1)). [M1.14c](../decisions-and-roadmap.md#m1-14c), the open
transonic increment, needs to know where a finned body's transonic peak sits and how tall it is, on a grid fine enough
to place it. [transonic-normal-force.md](transonic-normal-force.md) found tabulated public sets with only three
transonic points (Allen 2001 and 2005: Mach 0.6, 0.9 and 1.2 or 1.18) and dense sets that are only plotted.
[basic-finner-transonic-tables.md](basic-finner-transonic-tables.md) then reached the Basic Finner's tables, but they
are Crown copyright and the tunnel has no point between Mach 1.076 and 1.157.

This note asks whether any public report tabulates C_N and C_m of a cone- or ogive-cylinder with three or four planar
fins at Mach steps of 0.05 or finer through 0.8–1.2. **Inference** marks this note's own readings and arithmetic.

**In short: no public report found meets the rule strictly, but one comes close, and it is better than anything the
project holds.**

- **AEDC-TR-75-125 (the Martin Generalized Research Model compilation, 1976), public release.** Its Test 1 tabulates a
  9.5-caliber tangent-ogive cylinder with four planar fins at **Mach 0.80, 0.85, 0.92, 0.94, 0.96, 0.98, 1.05, 1.10,
  1.20, 1.30**: ten points, but steps of 0.07 twice and 0.10 once within 0.8–1.2, and none at Mach 1.00. Its Test 6
  tabulates the same fins on a 10-caliber body at 0.85, 0.92, 0.97, 1.01, 1.05, 1.10: steps of 0.04–0.05 from 0.92.
- **The peak it shows (Inference, fits over α 0° to 4°).** With a clipped-delta fin, C_Nα rises from 9.75 per rad at
  Mach 0.80 to a plateau of 11.0–11.2 from 0.98 to 1.20 (+13% to +15%), then falls to 10.0 at 1.30. With a square
  fin it rises +21% to a peak at Mach 1.05. The CP moves 0.2–0.3 caliber aft up to Mach 1, then 0.40–0.45 caliber
  forward by 1.30. No dip like the Arcas's at Mach 0.8–0.9 appears, except in one unconfirmed run.
- **NASA's own sets are structurally coarse above Mach 1.03.** In Langley's slotted 8-ft tunnel, wall-reflected
  shocks spoil rocket-length models from just above Mach 1.025 to about 1.1, so every NASA 8-ft report read stops at
  1.03 (TN D-1958) or jumps from 1.03 to 1.1–1.2 (TN D-794, TM X-3070). The named candidates are plotted only
  (TN D-794, TM X-3070), not reached (TN D-1228), or never found as a NASA report (the Triservice database).

Confidence: high that the compilation is tabulated, public and on the grid stated (read from its run indexes and
data pages); medium on the fitted slopes, read by eye from worn scans with no text layer, each slope about ±3%;
medium that nothing better exists, because NTRS searches titles and abstracts only and DTIC was down.

## 1. AEDC-TR-75-125: the best public set found

G. R. Gomillion, ARO Inc., *A Compilation of Static Stability and Fin Loads Data for Slender Body Missile Models With
and Without Tail Fins and Wings*, AEDC-TR-75-125 (AFATL-TR-76-23), March 1976, five volumes [G1]–[G3].
[dtic-subsonic-finner-data-grm.md](dtic-subsonic-finner-data-grm.md) read its Mach 0.59 Reynolds study; this note
reads the transonic tests.

- **Terms.** Every volume's DTIC cover (PDF p. 1) reads "Approved for public release; distribution is unlimited",
  changed by USADTC letter of 10 Dec 1979. Written by AEDC's contract operator, so "public release" is not by itself
  a statement of public domain: cite it and commit readings.
- **Bodies** [G1 Fig. 3, PDF p. 26]. d = 3.750 in. B4: tangent ogive nose 9.375 in (2.5 d), total 35.625 in (9.5 d).
  B2: the same nose, 37.500 in (10 d). Moment reference at half the length (§3.1, PDF p. 16). Fins sit at the base
  (§2.2, PDF p. 14).
- **Fins** [G1 Fig. 6, PDF p. 30; Table 1, PDF p. 525]. Modified double wedges, 4° edge half-angles, 0.015 in edge
  radii. F13: clipped delta, aspect ratio 1.0, taper 0.5, leading-edge sweep 33°41', root 3.747 in (1.0 d), span
  2.812 in (0.75 d). F16: a 2.812 in square. **Inference:** F13 is the nearest to a hobby fin, though its span is
  smaller than most.
- **Coefficients.** C_N on the body's cross-section; C_m on that area times d about the moment reference; XCP = C_m/C_N
  in calibers from it, negative aft [G1 nomenclature, PDF pp. 535–537]. XCP × C_N reproduces C_m on every row used.

### Test 1: AEDC 4T, ten Mach points

- **Conditions** [G1 §3.2, PDF pp. 17–18]. The 4-ft perforated-wall tunnel at 2.3 × 10⁶ per ft (6.8 × 10⁶ on the
  length, **Inference**), α −3° to 32° (tabulated in about 2° steps to 12–14°, then 1°), roll 0°, transition unknown,
  no wall correction described. Stated uncertainty [G1 Table 2, PDF p. 526]: C_N ±0.067, C_m ±0.078, Mach ±0.005 to
  0.95 and ±0.010 above.
- **Grid** [G1 Table 7, PDF p. 528]. B4 with F12, F13 or F16 at the ten Mach points above (F13: parts 22, 24, 30–37;
  F16: parts 42–51). F33, F32 and F11 have nine (no 1.05); six more sets and the bodies alone have seven
  (0.80, 0.85, 0.92, 0.98, 1.10, 1.20, 1.30).
- **Where** [G2, Appendix A]: F13 on PDF pp. 19–28, F16 on pp. 29–38, the body alone (parts 196–202) on pp. 123–129.
- **Sample rows** [G2]: part 22 (F13, Mach 0.80, PDF p. 19), α 3.96°, C_N 0.7434, XCP −2.5162; part 34 (F13, Mach
  1.05, p. 25), α 3.96°, C_N 0.8148, XCP −2.8248; part 48 (F16, Mach 1.05, p. 35), α 3.97°, C_N 0.7356.

| Mach | 0.80 | 0.85 | 0.92 | 0.94 | 0.96 | 0.98 | 1.05 | 1.10 | 1.20 | 1.30 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| B4 + F13, C_Nα per rad | 9.75 | 9.85 | 9.21 | 10.32 | 10.88 | 11.00 | 11.24 | 11.23 | 11.10 | 10.03 |
| B4 + F13, CP d from nose | 7.35 | 7.39 | 7.38 | 7.50 | 7.55 | 7.52 | 7.52 | 7.43 | 7.31 | 7.15 |
| B4 + F16, C_Nα per rad | 8.75 | 8.87 | 8.85 | 9.51 | 9.75 | 9.96 | 10.59 | 10.40 | 10.48 | 9.63 |
| B4 + F16, CP d from nose | 7.17 | 7.21 | 7.30 | 7.38 | 7.47 | 7.49 | 7.41 | 7.34 | 7.22 | 7.04 |
| B4 alone, C_Nα per rad | 2.58 | 2.59 | 2.65 | | | 2.71 | | 2.76 | 2.71 | 2.73 |

**Inference** for every number in the table: least-squares slopes over α ≈ 0° to 4° (body alone −2° to 4°), CP from
the moment-to-force slope over 2° to 6°.

- **How firm.** Repeat points at one α scatter up to 4.5% in C_N (part 24, Mach 0.85, α 1.99°: 0.3520 to 0.3678), so
  each slope carries about ±3%. Fitting 0° to 6° instead raises the F13 slopes 0% to 7%, most at Mach 0.85–0.92.
- **The F13 dip at Mach 0.92 (−6%)** rests on one run (part 30), low at both its points near 2° and 4°, and does not
  appear with F16. Unconfirmed.
- **Offsets.** Fins-on parts read C_N from about −0.03 to +0.065 at α ≈ 0°, positive below Mach 1 and near zero
  or negative from 1.05 up (body alone under 0.01), unexplained in the report. It moves the intercept, not the slope.

### Test 6: NSRDC 7×10 ft, steps of 0.04–0.05 to Mach 1.10

- **Conditions** [G1 §3.7, PDF p. 20]: 1.7 × 10⁶ per ft, transition fixed with No. 90 grit, α −3° to 31° in about
  0.5° steps, slotted floor and ceiling (§2.1.3), a balance on each fin. Stated uncertainty [G1 Table 3]: C_N ±0.100.
- **Grid** [G1 Table 12, PDF p. 533]: B2 with F16, F13 or F12 at 0.85, 0.92, 0.97, 1.01, 1.05, 1.10.
- **Where** [G3, Appendix F]: F13's main-balance sheets on PDF pp. 44, 50, 56, 62, 68, 74. Sample: part 12 (Mach
  1.10, p. 74), α 3.68°, C_N 0.7895, C_m −2.2381.
- **Fits (Inference),** B2 + F13 over α −2.8° to 3.7°: C_Nα 8.95, 9.19, 10.75, 11.09, 11.31, 11.57 per rad (+29% from
  Mach 0.85 to 1.10); CP 7.53 to 7.89 d from the nose.
- **Against Test 1 (Inference).** At Mach 0.85 Test 6 reads 9% below Test 1, at 1.10 3% above. The bodies differ by
  0.5 d of cylinder, which accounts for part of it; the rest is not bounded here. Test 7 (Vols. IV–V, Mach 0.80, 0.90,
  1.00, 1.10 only, 95 and 119 MB) was not downloaded.

## 2. The NASA candidates

| Report | Reached | Shape | Mach points, 0.75–1.25 | Form |
|---|---|---|---|---|
| TN D-794 [N1] | yes (HathiTrust) | multistage Scout, delta fins | 0.80, 0.90, 1.00, 1.03, 1.11 or 1.20 | plots |
| TN D-1228 [N2] | no | 1/10 Blue Scout Jr. | title says 0.40 to 1.03 | |
| Triservice database [N3] | AGARD paper only | 9 cruciform fin sets | Ames 6×6 ft, 0.6–2.0, not listed | |
| TM X-3070 [N4] | yes | canard missile, round nose | 0.80, 0.90, 0.95, 1.00, 1.03, 1.20 | plots |
| RM SL55F15 [N5] | yes | Hermes A-3A, A-3B, 4 delta fins, boattailed | flight, points ~0.02 apart | plots, text |

- **TN D-794** has one table, of reference constants (printed p. 13); every coefficient is plotted. The author warns
  that the 1.11 and 1.20 points "may be subject to error" from reflected shocks (printed p. 6). HathiTrust's copy is
  Google-digitized, under Google's terms of use: cite only.
- **TN D-1228:** HathiTrust's copy under its title holds TN D-1229, a different report; NTRS has no record. Its title
  stops at Mach 1.03 anyway.
- **"TM-2002-211653"** has no NTRS record. Allen, Shaw and Sawyer [N3] describe the tests: α −5° to 45° in **5°
  steps**, so only three points near zero and a loose slope; a tabulated release was "currently underway" in 1988.
- **RM SL55F15**, free flight at Wallops: model B's C_Nα rises from 0.10 at Mach 0.90 to 0.125 at 1.05 (+25%; units
  not printed, per degree by **Inference**); model A's changes "very little" (PDF p. 12). Model A's CP goes from 6.63
  calibers at Mach 0.75 to 6.72 at 1.02, then forward to 6.38 at 1.50 (PDF p. 13). Same sign as the Basic Finner.
- **Why NASA's grids jump.** In the slotted 8-ft tunnel, reflected compressions reach the model just above Mach 1.025
  and the reflection-free length grows from about 4 in at 1.025 to about 36 in at 1.14 [N6, PDF p. 19]. A rocket-length
  model gets clean data only to about 1.03. A dense grid through 1.0–1.2 needs free flight, a ballistic range, or a
  small model in a large tunnel; the compilation's 4-ft tunnel with a 36 in model reports no correction.
- **Also read and set aside:** TM-74058 (0.7, 0.9, 1.1, 1.2), TM X-594 (0.8, 1.0, 1.2, dynamic), TN D-945 (no fins).
  NTRS searches for "transonic missile", "fin-stabilized", "8-foot transonic", "body-tail" and a dozen rocket names
  found nothing tabulated and dense.

## 3. Other near misses

- **Calspan AA-4017-W-11** [C1] (public release): a 10.4 d ogive-cylinder, four rectangular fins, tabulated, but at
  Mach 0.90, 1.00, 1.20, 1.25 only in the band. Its plume-off runs are a third public tabulated set, with a point
  at Mach 1.00.
- **MICOM TR T-79-26** [C2] has wraparound fins; **AEDC-TR-77-97** [C3], AEDC's high-angle-of-attack data base, is
  plotted.
- **AGARD-AR-138** [C4] was reached and holds no finned body.

## What this suggests

- **A transonic fixture from Test 1, B4 + F13 and B4 + F16, Mach 0.80–1.30**, with the body alone as a third row,
  read from the tables, not plots. Expected effect: a measured peak position (flat to 0.92, rising through 0.94–0.98,
  plateau or peak 1.05–1.20, falling by 1.30) and size (+13% to +21% over Mach 0.80) on two hobby-like planforms, for
  M1.14c's reference set. HPR Sim was not run against it here.
- **Add Test 6 as a second tunnel** on nearly the same rocket. Expected effect: a measured tunnel-to-tunnel spread
  (−9% at Mach 0.85, +3% at 1.10 in C_Nα) that a tolerance can cite.
- **Cite RM SL55F15** as a public-domain free-flight check on the peak's sign and Mach; digitizing its Fig. 13 would
  add a plotted, not tabulated, series.
- **Read the remaining Test 1 fin sets** (F12 delta and the seven-point sets) before choosing a fixture's fins.

## Open questions

- **Whether a fixture may hold the compilation's numbers** or only the project's readings: a contractor report marked
  public release. A maintainer's call, like the Allen 2005 files.
- **Wall interference** in the 4-ft perforated tunnel near Mach 1.0–1.1 with a 36 in model is not described in the
  compilation; the original Test 1 report (AEDC-TR-71-74, a limited AD number) was not reached.
- **The Arcas's dip and CP move** remain unmatched: no public set read shows a C_Nα dip at Mach 0.8–0.9 beyond the
  one F13 point, or a CP move over 0.5 caliber.
- **Not reached:** TN D-1228; AFATL-TR-81-87 (Air Force Finner); AGARD-AR-138 Addendum and AR-303 (NATO's site
  answered 403); the Triservice tables (perhaps NEAR-TR-360, AD-A176503). DTIC answered "Under Maintenance" on
  2026-10-20; every DTIC report here is the Internet Archive's copy of DTIC's scan.
- **Reynolds number:** 5 to 7 × 10⁶ on length, about the size of a mid-size hobby rocket at Mach 1, but no fillets.

## Sources (all read 2026-10-20)

- **[G1]** G. R. Gomillion, AEDC-TR-75-125 Vol. I, 1976. <https://archive.org/details/DTIC_ADB010053>
- **[G2]** Same, Vol. II (Tests 1–5). <https://archive.org/details/DTIC_ADB010054>
- **[G3]** Same, Vol. III (Test 6). <https://archive.org/details/DTIC_ADB010055>
- **[N1]** T. C. Kelly, NASA TN D-794, 1961. <https://ntrs.nasa.gov/citations/20040008085> (record only); full text
  <https://hdl.handle.net/2027/uiug.30112106740845>
- **[N2]** T. C. Kelly, R. J. Keynton, NASA TN D-1228, 1962. <https://catalog.hathitrust.org/Record/011446766>
- **[N3]** J. M. Allen, D. S. Shaw, W. C. Sawyer, "Remote Control Missile Model Test", AGARD-CP-451 paper 17, 1989,
  pp. 197–198. <https://archive.org/details/DTIC_ADA218094>
- **[N4]** E. B. Graves, R. H. Fournier, NASA TM X-3070, 1974. <https://ntrs.nasa.gov/citations/19740025319>
- **[N5]** H. H. Jackson, NACA RM SL55F15, 1955. <https://ntrs.nasa.gov/citations/20010057623>
- **[N6]** V. S. Ritchie, A. O. Pearson, NACA RM L51K14, 1952. <https://ntrs.nasa.gov/citations/19930087191>
- **[C1]** R. E. de Kuyper, Calspan AA-4017-W-11, 1974. <https://archive.org/details/DTIC_AD0782208>
- **[C2]** J. A. Humphrey, MICOM TR T-79-26, 1979. <https://archive.org/details/DTIC_ADA066733>
- **[C3]** W. B. Baker Jr., AEDC-TR-77-97, 1978. <https://archive.org/details/DTIC_ADA051797>
- **[C4]** AGARD-AR-138, 1979.
  <http://web.archive.org/web/20230507000401/https://www.sto.nato.int/publications/AGARD/AGARD-AR-138/AGARD-AR-138.pdf>
- Also read: NASA TM-74058, TM X-594, TN D-945 (NTRS 19780012129, 19660024033, 19980231071).
