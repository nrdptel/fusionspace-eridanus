# A double-wedge fin's lift slope and center near Mach 1, against a 65-series section

**In short: in the paired tests found, a sharp-edged section (double wedge or biconvex) has 0.67 to 0.74 of a NACA
65-series twin's lift slope at its worst, Mach 0.92 to 0.95, and a center up to 0.19 of the mean aerodynamic chord (MAC)
further forward; by Mach 1.0 the ratio is 0.84 to 1.05.** The pairs are a wing-flow test of delta wings (NACA RM L51F01)
and a free-flight test of rocket fins (RM L50B10; roll damping, read as a stand-in for lift slope, falls 31% to 36% from
Mach 0.85 to 0.92 and rises 59% to 72% by 1.0). On the Arcas Robin (NASA TN D-4013,
[#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198)), that size covers the tunnel's Mach 0.9 fin-share
dip, 0.83 of DATCOM's slope, though the pairs' floors fall nearer Mach 0.95. It does not cover the Mach 1.0 jump, 1.33
to 1.47 times the methods' slope, where no pair exceeds 1.05. Confidence: medium on the slope loss's sign, lower on the
forward center (wing-flow only), low on carrying sizes to the Arcas, whose transonic aspect ratio, 0.54, is below every
pair's (1.0 to 1.8). It builds on [transonic-fin-lift-methods](transonic-fin-lift-methods.md) and moves #1198 and the
0.5-caliber CP target in [accuracy](../accuracy.md). **Inference** marks chart readings, mappings and arithmetic.

## 1. What the question needs, and the Arcas numbers

- **Wanted:** a lift slope C_Lα and a chordwise center for a finite wing of aspect ratio A 1 to 3 with a double-wedge or
  hexagonal section, 3% to 6% thick, Mach 0.8 to 1.2, ideally beside a NACA 63A or 65A twin of the same planform. No
  test meets all of that. The nearest are below; each section says what it misses.
- **The Arcas fins** ([transonic-fin-lift-methods](transonic-fin-lift-methods.md) §1): A 1.517, taper 0.64, leading
  edge 30°, 4.4% double wedge; transonic parameter Ā = A (t/c)^(1/3) = 0.536.
- **The tunnel's fin share** (fins on less fins off, on the panels' planform): 2.41, 2.15, 1.97, 3.59 per rad at Mach
  0.6, 0.8, 0.9, 1.0 (same note §7). Its center over −3° to 4°: 0.20, 0.27, 0.33 caliber aft of the root leading edge
  at Mach 0.8, 0.9, 1.0, against HPR Sim's 0.56 to 0.65 ([arcas-fin-share-cp-station](arcas-fin-share-cp-station.md)
  §2). The methods (DATCOM, slender wing, NACA TN 3671) rest on 63A- and 65A-section tests or none; 2.2 to 2.6 per rad.
- **Comparing thicknesses (Inference).** Transonic similarity matches flows at equal ξ = (M² − 1)/(t/c)^(2/3) and
  equal Ā. A Mach number below 1 on a 6% wing maps to a higher one on a 4.4% fin: 0.92 maps to 0.935. The rule is
  inviscid; the sharp-edged dips below look viscous, so the mapping is a guide, not a law.

## 2. Paired: two delta wings each way, wing-flow method (RM L51F01)

Hall and McKay flew four semispan wing-fuselage models over an F-51D's wing [1]: triangular wings of A 2.31 (leading
edge 60°) in NACA 65-009 and 9% biconvex, and of A 4.00 (45°) in NACA 65-006 and 6% double wedge, thickest at mid-chord
(printed pp. 1, 3; PDF = printed + 2). Mach 0.75 to 1.075, about 1.5 million on the A 2.31 MAC, coefficients on the
wing area carried to the centerline, moments about 50% MAC (pp. 1, 4). Ā is 1.04 and 1.57 (Inference).

Readings at C_L = 0 (Inference, by eye off Fig. 7(a), p. 17, and Fig. 11, p. 23; ±0.002 per deg, ±1.5% MAC). Ratio is
sharp-edged over 65-series; Δac is the sharp-edged center less the 65-series one, + aft.

| Mach | A 2.31: C_Lα, per deg | ratio | ac, % MAC | Δac | A 4.00: C_Lα, per deg | ratio | ac, % MAC | Δac |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 0.75 | 0.047 / 0.042 | 0.90 | 36 / 30 | −6 | 0.069 / 0.066 | 0.96 | 30 / 28.5 | −1.5 |
| 0.90 | 0.054 / 0.040 | 0.74 | 40 / 25 | −15 | 0.079 / 0.074 | 0.94 | 33.5 / 34.5 | +1 |
| 0.95 | 0.054 / 0.041 | 0.75 | 44 / 28 | −16 | 0.099 / 0.073 | 0.74 | 45.5 / 37 | −9 |
| 1.00 | 0.054 / 0.052 | 0.96 | 44.5 / 44 | −0.5 | 0.079 / 0.066 | 0.84 | 42 / 34 | −8 |
| 1.05 | 0.051 / 0.050 | 0.98 | 45.5 / 44.5 | −1 | 0.076 / 0.068 | 0.89 | 41.5 / 35.5 | −6 |

- **The extremes.** The biconvex wing bottoms at Mach 0.92 to 0.93: 0.039 per deg (0.72 of its twin) and 23% MAC (−18 to
  −19). It then rises 36% to 0.053 at Mach 1.01. The 65-006 delta peaks at 0.099 near Mach 0.95, the double wedge at
  0.077 near 0.93; the double wedge's center swings forward from 38.5% to 34% MAC by Mach 0.98.
- **The authors' reading.** The 65-series wings gave higher slopes "throughout the Mach number range" at zero lift and
  more rearward centers where the two differ (pp. 5, 7); at C_L = 0.3 section hardly mattered below Mach 0.9 (p. 5). A
  caution (p. 6): a bump test of a 65A006 delta of the same planform gave slopes "very close to" this double wedge's,
  lower than its 65-006 twin's, so facility scatter at A 4 is the size of the section effect.
- **Mapped to 4.4% (Inference):** the A 2.31 dip at Mach 0.92 maps to 0.951, the A 4.00 one at 0.95 to 0.960.

## 3. Paired: fins on rocket models, free flight (RM L50B10)

Dietz and Edmondson flew rocket models with three unswept rectangular fins of A 4.5 on a 6.5 in body, one in NACA
65-006 and two in a 6% modified double wedge, from Mach 0.85 to 1.45 at 3 to 6.7 million on an 8 in chord [2] (printed
pp. 1–3, Fig. 1 p. 8; PDF = printed + 1). They give the damping in roll, −C_lp. **Inference:** it follows the fins'
lift slope weighted to the tips, but only at small local angles (pb/2V), where RM L9A12 (§4) finds the double wedge
weakest; so its ratios likely overstate the loss in a −3° to 4° slope, by an amount not bounded here. Ā = 1.76.

| Mach | 0.875 | 0.90 | 0.92 | 0.95 | 1.00 | 1.05 | 1.20 |
|---|--:|--:|--:|--:|--:|--:|--:|
| 65-006, −C_lp | 0.35 | 0.36 | 0.37 | 0.39 | 0.41 | 0.41 | 0.43 |
| double wedge, −C_lp | 0.36 | 0.31 | 0.25 to 0.27 | 0.36 | 0.43 | 0.45 | 0.51 |
| ratio | 1.03 | 0.86 | 0.67 to 0.72 | 0.92 | 1.05 | 1.10 | 1.18 |

- **Readings (Inference):** Fig. 4(a), 4(b), printed p. 11, ±0.01; the 65-006 starts at Mach 0.875, the double wedge
  reads 0.39 at 0.85.
- **The report:** a "sudden decrease" in damping for the double wedge at transonic speeds, none for the 65-006
  (p. 1); both double-wedge models show the dip in roll rate between Mach 0.85 and 0.95 (p. 5).
- **Shape (Inference):** −31% to −36% from Mach 0.85 to 0.92, then +59% to +72% to 1.0, at flight Reynolds
  numbers. Mapped to 4.4%, the floor falls at Mach 0.935.
- **Not the wing's stiffness (Inference).** The double-wedge fins were 29% to 32% less stiff in torsion (465 and 485
  against 682 in·lb per deg, Fig. 1, p. 8). A twist loss grows with dynamic pressure; this dip recovers by Mach 0.95.

## 4. Unpaired double wedges: Reynolds number, small-angle lift, a low-aspect-ratio wing

- **RM L9A12 [3],** rectangular, A 4, 6% double wedge, wing-flow method, Mach 0.66 to 1.12 at 0.36 to 0.81 million
  (printed p. 1; Fig. 4, pp. 10, 12; PDF = printed + 1). It reports slopes rising and the center moving aft as the
  Reynolds number rises (p. 3), and a small-lift center going from 25% to 30% of chord, forward to 20% at Mach 0.92,
  then back to 30% at 0.94 and above (p. 1).
- **Its slopes (Inference,** Fig. 13(a), p. 25, ±0.002 per deg): medium and high dives 0.098 to 0.108 near Mach 0.86
  to 0.87 (the low dive peaks at 0.125 near 0.88), a dip to 0.088 to 0.094 at 0.92, then 0.104 to 0.112 by 1.0. At
  Mach 0.85 to 0.88 the low dive (0.56 to 0.81 million) reads 18% to 30% above the high one (0.36 to 0.52 million).
- **Small-angle lift (Inference,** Figs. 13(a), 13(b), pp. 25–26, medium dive). The slope over C_L 0 to 0.2 is 12% to
  29% below that over 0.3 to 0.5, from Mach 0.7 to 0.9: 0.085 against 0.097, 0.100 against 0.140. The report finds the
  lift curve steepening between 2° and 3°, abruptly from Mach 0.90 to 0.94 (p. 3). At Mach 0.92 the small-lift center
  sits near 20% of chord and the larger-lift one 35% to 40%.
- **RM L50G13a [4],** A 2.5, taper 0.625, unswept mid-chord, 6% modified double wedge, Langley transonic bump, about
  1 million (printed p. 1). Fig. 15 (p. 25; PDF = printed + 1) shows C_Lα 0.053 per deg at Mach 0.6 and 0.054 to 0.059
  at 0.9 over three root fittings, 1% to 11% below the report's subsonic estimate (Inference); the authors say the
  slopes "compare favorably" with theory (p. 6). Ā = 0.98: the nearest double wedge to the Arcas below Mach 1.1 (TN
  3522's is lower, 0.86, but only above 1.16), at two Mach numbers; its moments weren't read.

## 5. The conventional section at the Arcas's Ā, and the band above Mach 1.1

- **NACA TN 3501 [5],** 63A rectangles on the Ames 16-ft bump, 1.25 to 2.05 million (printed p. 1; PDF = printed + 2).
  At A 1.5 (4% and 2%, Ā 0.51 and 0.41) the slope rises smoothly with no dip: 0.040 to 0.043 per deg at Mach 0.8,
  0.045 to 0.048 at 1.0, +10% to +15%, peaking at 0.049 to 0.052 near 1.05 to 1.08 (Inference, Fig. 14, p. 106). The
  center stays 0.12 to 0.23 MAC aft of the leading edge, most forward near Mach 0.95 (Fig. 16, p. 109). Its A 4 63A006
  wing (the short-dash curve) rises 38% to 40% from Mach 0.7 to a 0.104 peak near 0.93.
- **NACA TN 3522 [6],** pressure plots on double-wedge rectangles of A 2 and 4, 7.87% thick, Mach 1.166 to 1.377, 0.54
  million (printed pp. 1, 5; PDF = printed + 1). Near shock attachment the two-dimensional section's lift-slope peak is
  "almost completely gone" at A 2 (p. 20). Mapped to 4.4% (Inference): Mach 1.12 to 1.16; no section bump expected.

## 6. Worked by hand: the section on the Arcas (Inference)

Method of [transonic-fin-lift-methods](transonic-fin-lift-methods.md) §6: body 1.90 per rad, fins 3.95 times their
planform slope on the body's area, the body's center 0.1 m ahead of the nose tip (its DATCOM middle rows, −0.47 and
−0.63 caliber, come back within 0.01). From DATCOM's middle (2.37 per rad at 0.33 root chord, Mach 0.9; 2.56 at 0.37,
Mach 1.0), scale the slope by the pairs' ratio and move the center by Δac, a band spanning L51F01's readings from Mach
0.9 (−0.15, +0.01) to its floors (−0.18 to −0.19). L50B10's rows borrow it (no moments); theirs are roll ratios (§3).

| Mach | section ratio | Δac, MAC | fin slope, per rad | rocket C_Nα | CP against the join | against the tunnel |
|--:|--:|--:|--:|--:|--:|--:|
| 0.9 | 0.94 (L51F01, A 4) | 0 to −0.20 | 2.23 | 10.71 | −0.64 to −0.84 | +0.70 to +0.50 |
| 0.9 | 0.86 (L50B10 at 0.90) | 0 to −0.20 | 2.04 | 9.96 | −0.89 to −1.09 | +0.45 to +0.25 |
| 0.9 | 0.67 (L50B10 floor) | 0 to −0.20 | 1.59 | 8.18 | −1.67 to −1.86 | −0.33 to −0.52 |
| 1.0 | 0.84 (L51F01, A 4) | 0 to −0.08 | 2.15 | 10.38 | −1.09 to −1.18 | −1.50 to −1.59 |
| 1.0 | 1.05 (L50B10) | 0 to −0.08 | 2.68 | 12.50 | −0.51 to −0.59 | −0.92 to −1.00 |

- **Mach 0.9.** The tunnel reads 10.22 per rad and a CP 1.34 caliber ahead of the join; the 0.86 row comes within
  0.25 to 0.45 caliber, and the band 0.67 to 0.94 brackets zero. The share's 1.97 is 0.83 of DATCOM's middle (0.81 to
  0.86 over its band). The share's center, 0.18 root chord, is 0.18 MAC ahead of DATCOM's: inside the 0 to 0.20 band.
- **Mach 1.0.** Every pair moves the CP further from the tunnel than the join's −0.41. The share's 3.59 is 1.38 to 1.43
  times DATCOM's slope and 1.33 to 1.47 times TN 3671's 2.45 to 2.7; the pairs' ratios there run 0.84 to 1.05.
- **Sign on the rocket.** A lower fin slope moves the CP forward and the margin down, the conservative side. Taking
  the section into the model there would move HPR Sim's CP 0.6 to 1.9 caliber forward, toward the tunnel.

## 7. What the section explains, and what it leaves

- **The Mach 0.9 dip: in sign and size, yes; in timing, not clearly (Inference).** Two wing-flow tests and one
  free-flight test show a slope dip between Mach 0.9 and 0.95, the wing-flow ones a forward center too; the bump test
  sits 1% to 11% low at its one point, Mach 0.9. The pairs' loss is 6% to 26% at 0.9, up to 33% at their floors; the 63A
  wing at the Arcas's Ā shows none. But the floors map to Mach 0.935 to 0.96 on a 4.4% fin, while the tunnel's fins-on
  slope is higher at 0.95 than at 0.9 (8.97 against 7.74 per rad,
  [arcas-fin-share-cp-station](arcas-fin-share-cp-station.md) §2): the Arcas dips earlier.
- **The Mach 1.0 jump: its first part, not its height.** From their floors the sharp-edged wings recover 11% to 72%
  (none for L51F01's A 4); the share rises 82%. It ends 33% to 47% above the 63A methods, where at Mach 1.0 no
  sharp-edged section exceeds its twin by over 5%.
- **The ±2° flattening: sign, yes.** On the short model the share's force falls 6% to 11% and its moment 17% to 19%
  within ±2° ([arcas-fin-share-cp-station](arcas-fin-share-cp-station.md) §3). RM L9A12's double wedge has the lower
  slope at small lift, and near Mach 0.91 to 0.94 the more forward center (at 0.8 within 1% of chord). Its split of lift
  ranges differs, so the sizes can't be compared.
- **The scale problem.** Every pair has Ā 1.0 to 1.8, the Arcas 0.54. Slender-wing theory has no section term; TN 3522
  finds the nonlinear part shrinking with aspect ratio. At Ā 0.54 the effect is likely smaller, by an unbounded amount.

## What the project can use

- **NACA RM L51F01, RM L50B10, RM L9A12, RM L50G13a, TN 3501, TN 3522** [1–6]: US government works on NTRS, public use;
  cached under `refs/research/double-wedge-transonic-fin-lift/` (gitignored), with the index used to find them [8].
  Readings with figure and page may be committed; the §2 and §3 tables could be a paired section fixture.

## What this suggests

1. **Record in #1198** that the Mach 0.9 row is consistent with a double-wedge loss of the paired tests' size, and the
   Mach 1.0 row is not. Expected effect: the 0.9 miss (+1.34 caliber, flattering) gets a named, sized candidate: a
   ratio 0.68 to 0.87 brings it within 0.5 caliber at any center shift from 0 to 0.20 MAC forward, and 0.63 to 0.94
   with a suitable one (Inference). The timing caveat (§7) stays with it. The 1.0 row keeps its open cause.
2. **Don't ship a section factor yet.** Fitting 0.83 on the one rocket it should explain would score itself. A factor
   needs a double-wedge test at Ā near 0.5, which none found here is; RM L51G03 [7] (Ā 0.98) is the nearest unread.
3. **Score the share by window.** The pairs' small-lift losses argue for keeping the ±2° rows apart from −3° to 4°.

## Open questions

- **A low-Ā twin.** No double-wedge or hexagonal wing near A 1.5 with a 63A twin was found. RM L50A18 [9] (65A004,
  same Langley bump as [4]) is the nearest conventional wing, but A 2, taper 0.78, unswept quarter chord: not a twin.
- **RM L51G03 [7]:** lift at α −4°, 0°, 4° on the [4] wing with a body, 14 Mach numbers from 0.60 to 1.18 (counted
  in Fig. 7's legend, printed p. 14, PDF p. 16; p. 5); its stacked axes weren't read.
- **Reynolds number.** The Arcas tunnel ran near 0.70 million on MAC. RM L9A12's double wedge reads 18% to 30% more
  slope at 0.56 to 0.81 million than at 0.36 to 0.52. Flight may dip less; RM L50B10 still dips 31% to 36% at 3 million.
- **The A 3, 5% hexagonal free-flight model** of RM L9K25a [10] shows a 27% MAC center travel (p. 1), but for the
  whole airplane, tail included; its wing's share isn't separable.

## Sources

All read 2026-11-05. NACA works on NTRS: US government works, public use, cached under
`refs/research/double-wedge-transonic-fin-lift/` (gitignored).

1. A. W. Hall, J. M. McKay, NACA RM L51F01, 1951. Pp. 1–7; Table I p. 9; Figs. 7, 11 (pp. 17, 23).
   <https://ntrs.nasa.gov/citations/19930086839>
2. A. E. Dietz, J. L. Edmondson, NACA RM L50B10, 1950. Pp. 1–6; Figs. 1, 4 (pp. 8, 11).
   <https://ntrs.nasa.gov/citations/19930086285>
3. L. J. Lina, NACA RM L9A12, 1949. Pp. 1–5; Figs. 4, 13 (pp. 10, 12, 25, 26).
   <https://ntrs.nasa.gov/citations/19930085867>
4. T. R. Turner, J. E. Fikes, NACA RM L50G13a, 1950. Pp. 1–6; Fig. 15 (p. 25).
   <https://ntrs.nasa.gov/citations/19930086281>
5. W. H. Nelson, J. B. McDevitt, NACA TN 3501, 1955. Pp. 1–5; Figs. 14, 16 (pp. 106, 109). Cached under
   `refs/research/transonic-fin-lift-methods/src/`. <https://ntrs.nasa.gov/citations/19930084143>
6. W. G. Vincenti, NACA TN 3522, 1955. Pp. 1, 5, 17–20. <https://ntrs.nasa.gov/citations/19930084206>
7. R. D. Vogler, V. E. Lockwood, T. R. Turner, NACA RM L51G03, 1951. Pp. 1–6; Figs. 1, 7 (pp. 8, 14).
   <https://ntrs.nasa.gov/citations/19930086757>
8. NACA reports once designated restricted or nonpublic, 1944–1959 (FOIA release, 2014-04-21, governmentattic.org),
   searched for "wedge" and "hexagonal". <https://www.governmentattic.org/11docs/NACAcontrolledRpts_1944-1959.pdf>
9. E. C. Polhamus, G. S. Campbell, NACA RM L50A18, 1950. P. 1. <https://ntrs.nasa.gov/citations/19930086162>
10. C. L. Gillis, R. F. Peck, A. J. Vitale, NACA RM L9K25a, 1950. P. 1. <https://ntrs.nasa.gov/citations/19930086055>
