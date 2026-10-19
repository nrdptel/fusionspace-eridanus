# A finned missile model at Mach 0.59: AEDC-TR-75-125's Reynolds-number study

This is the second part of [dtic-subsonic-finner-data](dtic-subsonic-finner-data.md). It reads the one finned
configuration that AEDC-TR-75-125 (the Martin Generalized Research Model compilation) tested below Mach 0.8
(Table 8 and the Test 2 text): body B3 with tail fins F15 (B3W0F15), at Mach 0.59, in Test 2's Reynolds-number
study. **Inference** marks this note's own readings and arithmetic.

**In short: the data are reachable, tabulated and fully dimensioned. The rocket's measured C_Nα is 6.0–6.4 per rad
and its CP 6.4–6.6 calibers from the nose. HPR Sim's linear fin and nose terms give 4.52 per rad and 5.68 d: 24–29%
low, with the CP 0.7–0.9 caliber ahead.** Separate fin balances show the fin panels match the model within −3% to
+6%. The shortfall is in the body's share, which the model holds at the nose's 2.0 per rad. The test measured
3.5–3.7 per rad there, the extra acting near the fins. Confidence is medium: the scan is worn, so each value read was
cross-checked against a printed ratio column, and the fits rest on seven points from −1.5° to 4.5°.

## 1. The report and where the data are

*A compilation of static stability and fin loads data for slender body missile models with and without tail fins and
wings*, G. R. Gomillion, ARO Inc., AEDC-TR-75-125 (AFATL-TR-76-23), March 1976, five volumes [G1], [G2].

- **Terms.** Both volumes' DTIC covers read "Approved for public release; distribution is unlimited", changed from US
  Government only on USADTC's letter of 10 Dec 1979 (PDF p. 1 of each). A US Air Force report by AEDC's contract
  operator: cache, cite, commit only readings.
- **Where.** Volume I [G1] holds the text, drawings and indexes. Volume II [G2] (Internet Archive item
  DTIC_ADB010054, 615 PDF pages counted with `pdfinfo`) holds Appendix B, Test 2's tables. Volumes III–V
  (DTIC_ADB010055 to 057) hold Tests 6 and 7, all at Mach 0.80 and up.
- **What is below Mach 0.8.** Test 1's points there are body alone (Table 7, PDF p. 528). Test 2's Reynolds study
  (Table 8, PDF p. 529) adds B3W0F0, body alone, parts 5–16, and B3W0F15 in 16 parts (24–50) at Mach 0.59. Those
  run fixed and free transition, roll 0° and 180°, and 0.3 to 3.4 × 10⁶ per foot. None of these parts is plotted.
- **Units.** The R column is the nominal Reynolds number per foot (Nomenclature, PDF p. 536). On the 40 in length that
  is 1.0 to 11 × 10⁶, and 0.1 to 1.1 × 10⁶ on the diameter (Inference).

## 2. The model

Body B3 (Fig. 3, PDF p. 26) is a secant ogive and cylinder, 40.000 in long and 4.000 in (102 mm) in diameter: 10 d.
The nose is 8.000 in (2 d) long, an arc of radius 34.000 in centered 12.190 in aft of the tip and 31.740 in from the
axis. That is ρ/ρ_t = 2.0 in [shapes](../physics/shapes.md)' terms, so the nose meets the body at about 7.1° and has
a 21° tip (Inference).

Fin F15 (Fig. 6, PDF p. 30) is one of 15 planforms, four mounted in a "+", trailing edges in the base plane (PDF
p. 15). Its drawing gives a 1.875 in span, root chord 4.994 in, tip chord 2.497 in, leading-edge sweep 53°6' and an
unswept trailing edge. One panel is 0.04877 ft² (aspect ratio 0.5). The section is a modified double wedge, 0.187 in
thick at the root, with 4° edge half-angles beveled over 1.165 in (leading) and 1.140 in (trailing) and 0.015 in edge
radii. Table 1 (PDF p. 525) gives each fin as measured: the four F15 fins average 1.8741 in span and 4.9936 in and
2.4985 in chords, within 0.01 in of the drawing. HPR Sim can draw all of it but the double-wedge
section; `airfoil` is the nearest.

## 3. Test and data conventions

- **Tunnel.** AEDC 4T, a 4 ft square test section with perforated walls, Test 2 in May 1972 (PDF pp. 13–14).
  Blockage is 0.55% of the section (Inference). The text names no wall correction for Test 2. It corrects flow
  angularity only for Tests 3 and 4 (PDF p. 19).
- **Balances.** A six-component internal balance carries the model. Four three-component balances in the rear body
  carry each fin's normal force, hinge moment and root bending moment (PDF p. 16).
- **Transition.** "Fixed" parts carry 1/8 in No. 60 grit strips along the cylinder, between the fins on the lee side
  (PDF p. 18).
- **Coefficients.** C_N on the body's cross-section S, C_m on S·d about the moment reference point at 50% of the body
  length, 20.000 in (5.0 d) from the tip (PDF p. 16; Fig. 3). C_m is positive nose up (Fig. 10, PDF p. 34). The XCP
  column is C_m/C_N in calibers (PDF p. 537); negative is aft (Inference, from Fig. 10's signs). Fin normal
  force CNF is on one panel's area S_F.
- **Angles.** About −1.6° to 20°, 22 points a part; roll angle 0° puts fins 2 and 4 horizontal.
- **Uncertainty.** At 3.5 × 10⁶ per foot, Table 6 (PDF p. 527) gives ±0.029 in C_N, ±0.105 in C_m and ±0.022 in
  fin 4's CNF. At 0.3 × 10⁶ they grow to ±0.160, ±1.072 and ±0.089.

## 4. The data at the highest Reynolds number (Inference, read from the scan)

Part 50 (B3W0F15, free transition, roll 0°, 3.4 × 10⁶ per foot) is on PDF p. 201 (printed 200), with the fins on
PDF pp. 202–203. The scan's C_N and C_m digits are partly broken. Each value below was read and then checked
against the XCP column, which prints cleanly, since C_m = XCP × C_N. Where they disagreed, the digit that fits both
was kept.

| α, deg | C_N | C_m | XCP, d | CNF2 | CNF4 |
|---:|---:|---:|---:|---:|---:|
| −1.53 | −0.1307 | 0.1975 | −1.5114 | −0.0498 | −0.0413 |
| −0.58 | −0.0237 | 0.0472 | −1.9958 | −0.0124 | −0.0044 |
| 0.42 | 0.0722 | −0.0868 | −1.2027 | 0.0272 | 0.0303 |
| 1.45 | 0.1825 | −0.2419 | −1.3254 | 0.0615 | 0.0765 |
| 2.47 | 0.2932 | −0.4072 | −1.3888 | 0.1043 | 0.1133 |
| 3.50 | 0.4170 | −0.6307 | −1.5126 | 0.1510 | 0.1627 |
| 4.51 | 0.5395 | −0.8613 | −1.5965 | 0.2076 | 0.2123 |

Least-squares lines through these points give (S_F/S = 0.559):

| α range, deg | C_Nα, per rad | C_mα, per rad | CP from nose, d | fins 2 + 4 on S, per rad |
|---|---:|---:|---:|---:|
| −1.53 to 1.45 | 5.97 | −8.37 | 6.40 | 2.45 |
| −1.53 to 3.50 | 6.16 | −9.18 | 6.49 | 2.55 |
| −1.53 to 4.51 | 6.30 | −9.82 | 6.56 | 2.66 |
| 0.42 to 3.50 | 6.39 | −10.04 | 6.57 | 2.65 |

- **Zero offset.** The fits cross α = 0 at C_N 0.02–0.03, about 0.25° of flow angle or a balance zero. Slopes don't
  see it.
- **Repeat.** Part 33 (the same at fixed transition, PDF p. 171) gives XCP −1.318, −1.510 and −1.676 at 1.48°, 3.49°
  and 5.51°; part 50 gives −1.325, −1.513 and −1.662. They agree within 0.015 caliber.
- **Reynolds number.** The point CP near 3.4° lies 1.47–1.54 calibers aft of the reference in parts 24 (0.3 × 10⁶
  per foot), 31 and 49 (1.1 × 10⁶, fixed and free) and 33 and 50 (3.4 × 10⁶): within 0.07 caliber from 1.0 to 11 ×
  10⁶ on length.
- **The fin's own CP.** XCPF2 and XCPF4 put each horizontal fin's load 0.14–0.22 root chords from its hinge line,
  2.746 in aft of the root's leading edge. Taken as forward of the hinge, that is 1.65–2.05 in aft of the leading
  edge, against the linear model's 2.08 in. This note didn't settle the hinge moment's sign from Fig. 10.

## 5. What HPR Sim's linear terms give (Inference)

The equations of [aero](../physics/aero.md#fins), at Mach 0.59 (β = 0.807), with A_ref = 12.57 in²:

- **Nose.** Slope 2.0. Integrating the arc gives 43.05 in³, so X_B = l − V/A(l) = 4.57 in, 1.14 d.
- **One fin.** s = 1.875 in, A_fin = 7.023 in², mid-chord sweep 33.7°: 2π s²/A_ref = 1.758, over
  1 + √(1 + (0.807 × 0.601)²) = 2.112, gives 0.832 per rad (0.811 at Mach 0).
- **Fin set.** In a "+" at roll 0° the sum of sin² is 2. K_T(B) = 1 + 2/(1.875 + 2) = 1.516, so 2.52 per rad.
- **Fin CP.** MAC 3.884 in, its leading edge 1.110 in aft of the root's: X_f = 2.081 in, at 37.09 in, 9.27 d.
- **Total.** C_Nα 4.52 per rad (4.46 at Mach 0), CP (2.0 × 1.14 + 2.52 × 9.27)/4.52 = 5.68 d.

Against the fits in section 4:

- **The fin panels agree.** Measured 2.45–2.66 per rad against 2.52: −3% to +6%.
- **The whole rocket doesn't.** C_Nα 4.52 against 5.97–6.39 is 24–29% low. The CP is 0.72–0.89 caliber ahead, so
  the margin reads *low*, the safe side. That is Cranfield's sign and the Basic Finner's, larger here. C_mα about the
  reference is −3.07 per rad against −8.4 to −10.0.
- **Where the gap sits.** Measured total minus measured fins minus the model's nose leaves 1.5–1.7 per rad. Its
  moment balance puts it at 8.7–8.8 d, beside the fins, taking the measured fin load at the model's fin CP (9.27 d);
  at the fin CP's other readings (1.65 in or 3.5 in aft of the leading edge) it moves to about 8.5 or 8.1 d.
  That fits lift the fins carry onto the body, which the model's K_T(B) doesn't count, but this note didn't size
  that term from a source. Body lift, in HPR Sim's full model ([aero](../physics/aero.md#body-lift)), has zero slope
  at α = 0 and can't close it.

## What the project can use

- **A validation case.** Fly B3W0F15 at Mach 0.59 against parts 33 and 50 at α up to 5°, as the Arcas fixture does,
  with this note's readings, page and table. Report the gap with its sign. Expected effect: the first subsonic check
  that separates the fin term from the body term, since the fin balances are tabulated. Today the linear terms would
  show −24% to −29% in C_Nα and the CP 0.7–0.9 caliber ahead.
- **The Reynolds sweep.** Parts 24–50 span 1.0–11 × 10⁶ on length, a hobby rocket's range and above, at one Mach
  number. At 0.3 × 10⁶ per foot Table 6's ±1.072 in C_m would leave the
  point CP uncertain by about ±3 calibers, yet it agrees within 0.07 caliber, so Table 6 looks conservative there.
- **Terms.** Public release: cache under `refs/`, cite, commit readings only.

## Open questions

- **The body's share.** Is the 1.5–1.7 per rad near the fins carryover lift? HPR Sim leaves the body lift the fins
  induce, K_B(T), out ([aero](../physics/aero.md#fins)); Barrowman 1967, eq. 3-98, has it. Sizing it here would answer.
- **Ref. 2**, Whoric, AEDC-TR-72-143 (1972), analyzes Test 2 and may hold faired curves or wall corrections; not
  reached.
- **The hinge-moment sign** in Fig. 10, which fixes whether the fin CP is 2.0 in or 3.5 in aft of the leading edge.
- **Parts 36–41** (roll 180°, grit to windward) and the lower Reynolds-number parts were read only for XCP.

## Sources

Read 2026-10-19 from the Internet Archive's DTIC mirror; cached under `refs/research/dtic-subsonic-finner-data/`
(`nsrdc-vol1/`, `grm-vol2/`), not committed.

- **[G1]** G. R. Gomillion, AEDC-TR-75-125, Vol. I, March 1976. <https://archive.org/details/DTIC_ADB010053>
- **[G2]** G. R. Gomillion, AEDC-TR-75-125, Vol. II (Appendixes A–E), March 1976.
  <https://archive.org/details/DTIC_ADB010054>
