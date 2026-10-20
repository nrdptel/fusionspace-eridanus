# Why AFATL's tangent-ogive drag reads above NACA's at Mach 2 to 3

**In short: the AFATL-TR-77-8 set is the biased one, high by a nearly constant +0.011 to +0.026 on the base area,
and theory sides with NACA.** Two independent theories agree within 2%: this note's own second-order shock-expansion
calculation (NACA TN 3527) and DATCOM's ogive chart. For a fineness-3 tangent ogive they give 0.100 to 0.101 at
Mach 2 and 0.091 to 0.092 at Mach 3. AFATL's entry-1 runs read 0.121 and 0.102; TN 3716 reads 0.107 to 0.108. The
excess is the same size on 2-, 3- and 4-caliber noses, so it is an axial-force bias, not nose aerodynamics. AFATL's
second balance reads +0.006 to +0.016 above its first on six repeated bodies; the first balance's own excess,
about +0.02 at Mach 2 and +0.011 at Mach 3, has no traced cause. Friction is not it. How sure: the sign is well
supported; the cause of the first-balance offset is not settled. It moves [#616][i616]'s row: HPR Sim's
fineness-3 nose is +3% to +4% at Mach 2 and −6% to −7% at Mach 3 against theory, not −14% to −22%.

**What this covers.** The follow-up the [ogive wave drag note](ogive-wave-drag.md) left open: which Mach 2–3
reference is biased, which way and by how much, and whether DATCOM or Van Dyke's TN 2744 sides with either. It
builds on that note's readings and does not repeat them. **How far to trust it.** Table values are copied from
the reports; appendix rows are read off scanned printouts; chart values are read by pixels. **Inference** marks this
note's own arithmetic; scripts are in `refs/research/ogive-drag-reference-spread/`.

## 1. Theory sides with the NACA data

**Second-order shock-expansion (Inference).** This note implemented Syvertson and Dennis's method, eqs. (4) to (10)
of TN 3527 (pp. 7–9). It uses a 200-element tangent body and a Taylor–Maccoll cone at the vertex (the parent note's
`tm.py`); doubling the element count changes no fourth digit. Checks: at Mach 3 on fineness 3 it gives p/p∞ 2.63 at
the vertex, 1.02 at x/l 0.8 and 0.80 at the shoulder. TN 3527's Fig. 3(c) (p. 41) plots about 2.65, 1.0 and 0.8,
and finds characteristics solutions "almost indiscernible" from the method there (p. 15). The method's stated range
is Mach/fineness 0.4 to 2 (p. 1), which covers every case below. It fails where the vertex flow is subsonic
(fineness 2 at Mach 1.5).

**DATCOM 4.2.3.1, Method 2.** USAF DATCOM's ogive-nose chart, Fig. 4.2.3.1-61 (p. 4.2.3.1-61), plots
$C_D (f^2 + 1/4) K_N$ against $\beta/f$. It is read here by pixels at 300 dpi (±0.005 on the ordinate), with $K_N$
1.008 to 1.03 from Fig. 4.2.3.1-63. The chart rests on test data and slender-body and Newtonian theory, substantiated
in Morris (1961) (p. 4.2.3.1-11). DATCOM, reporting Morris's survey, says second-order shock-expansion "gives the best
over-all agreement with data" of ten theories (p. 4.2.3.1-8). Missile DATCOM offers the same method, Van Dyke's
hybrid theory or modified Newtonian theory for supersonic body wave drag (1998 manual, Table 27, p. 97).

| Fineness | Mach | Shock-expansion (Inference) | DATCOM chart | Measured, pressure only | Source of measured |
|--:|--:|--:|--:|--:|--|
| 3 | 1.5 | 0.107 | 0.108 | 0.103–0.136 (mean 0.119) | AFATL, both entries |
| 3 | 1.98 | 0.100 | 0.101 | 0.107–0.108 ± 0.005 | TN 3716 integrated |
| 2.93 | 1.99 | 0.104 | 0.105 | 0.103 | Report 1386 less laminar |
| 3 | 2.0 | 0.100 | 0.101 | 0.121 (entry 1) | AFATL |
| 3 | 2.73 | 0.093 | 0.094 | 0.109 | Report 1306 less laminar |
| 2.93 | 2.86 | 0.096 | | ≤ 0.090–0.094 | Report 1386 less laminar |
| 3 | 3.0 | 0.091 | 0.092 | 0.102 (entry 1) | AFATL |
| 2.93 | 3.06 | 0.095 | | ≤ 0.088–0.092 | Report 1386 less laminar |

At Mach 2 the theories sit 0.006 to 0.008 under TN 3716 (its ±0.005 nearly covers that), 0.020 to 0.021 under AFATL.
Shock-expansion runs 0.000 to 0.020 under TN 3716's eight nose pressures (Table I(a), p. 20), mean −0.008.
At Mach 3 the theories are not one number. Report 1386's characteristics line (from TN 2250) reads about 0.084
at Mach 3.0 and 0.083 at 3.06 for fineness 2.93, about 12% under shock-expansion (its Fig. 11(b), p. 12, re-read
by pixels). The parent note's similarity reading for fineness 3 spans 0.083 to 0.090. The theory band at Mach 3,
fineness 3, is 0.083 to 0.092. AFATL's 0.102 to 0.105 sits above all of it; Report 1386 sits inside or under it.

**Van Dyke's TN 2744** tabulates no fineness-3 drag at Mach 2 to 3: its worked cases are a 6-caliber ogive and a
3-caliber ogive at Mach 1.5 (pp. 31–32). The theory needs the body's slope under the Mach line's (TN 3527, p. 2):
a fineness-3 vertex slope of 0.342 passes at Mach 2 (Mach line 0.577) and is marginal at Mach 3 (0.354). It sides
with NACA only through TN 3716, which finds it predicts the Mach 1.98 pressures adequately (p. 1).

## 2. AFATL's excess is one offset, the same on every nose

Table 3 (pp. 10–12) less Table 4 (p. 16), entry-1 parts only (Table 2, p. 8, gives each cell's part number):

| Mach | Fineness | AFATL entry 1 (n) | Shock-expansion | DATCOM | Excess over shock-expansion |
|--:|--:|--:|--:|--:|--:|
| 2 | 2 | 0.225 (4) | 0.198 | 0.201 | +0.026 |
| 2 | 3 | 0.121 (3) | 0.100 | 0.101 | +0.021 |
| 2 | 4 | 0.082 (3) | 0.061 | 0.061 | +0.021 |
| 3 | 2 | 0.196 (3) | 0.183 | 0.179 | +0.013 |
| 3 | 3 | 0.102 (3) | 0.091 | 0.092 | +0.011 |
| 3 | 4 | 0.068 (3) | 0.055 | 0.056 | +0.012 |

**Inference.** A nose-flow effect would scale with the nose's drag, which falls by a factor of 3 from fineness 2 to
4. The excess changes by at most 0.0055 (0.007 against DATCOM at Mach 3). So it is an additive bias in the forebody
axial force at each Mach number: balance, base-pressure term, or a friction error common to all bodies. The report's
stated 95% uncertainty in that coefficient, bias included, is ±0.014 to ±0.016 at Mach 2 and ±0.010 to ±0.014 at
Mach 3 (pp. 6–7). The excess is 1.3 to 1.9 times it at Mach 2 and 0.8 to 1.3 times it at Mach 3.

## 3. The two balances disagree by +0.006 to +0.016

AFATL ran two entries on two different balances, parts 1–247 and 248–338 (p. 4; uncertainty tables pp. 6–7).
Six bodies were run in both at the same Mach number and Reynolds number. Appendix B's zero-angle rows (printed
pp. 119–230), entry 2 less entry 1, in forebody axial force: +0.006 to +0.008 (N14 M11, Mach 2), +0.010 (N26 M9,
Mach 2), +0.014 (N26 M5, Mach 2), +0.015 (N22 M5, Mach 3), +0.016 (N26 M9, Mach 3), +0.007 (N26 M5, Mach 3, parts
129 and 312). All six have the same sign.

- **Table 3 mixes the entries.** 5 of its 24 sharp-nose cells at Mach 2 and 3 come from entry 2 (Mach 2 on the
  11-caliber body for N18 and N22; Mach 3 on the 5-caliber body for all three). Each is the highest in its row:
  0.133, 0.084, 0.208, 0.113, 0.075. Dropping them lowers the fineness-3 means by 0.003 (0.124 to 0.121, 0.105 to
  0.102): the part of the gap the balance split explains in the published means.
- **Where the shift sits.** At Mach 2 it is in total axial force (+0.011 to +0.019); the base term moves 0.001 to
  0.012. At Mach 3 two pairs repeat their base term within 0.004 and shift total axial force (+0.006, +0.011),
  but on N26 M9 the base term read 0.045 in entry 1 and 0.018 in entry 2.
- **Point scatter.** On N18 M5 at Mach 2 the five points from −1° to +1° read 0.156 to 0.164 (part 101, p. 141).

## 4. Friction is not the cause

- **Table 4 is a fair laminar estimate.** This note's own laminar friction (**Inference**: Blasius local friction,
  Eckert reference temperature, Mangler's axisymmetric transform, the tunnel's 1.8 million per foot and 120 °F
  supply) reads 7% to 10% under Table 4 on the N18 bodies. If Table 4 is high, AFATL's pressure drag should
  rise by 0.003 to 0.005, widening the gap.
- **No growth with length.** Extra friction from transition would grow with wetted area. Within entry 1, derived
  nose drag changes by −0.001 to +0.006 from the shortest to the longest body run (six series). The shortest
  bodies at Mach 2 still sit +0.019 to +0.029 above shock-expansion.
- **A Reynolds sweep.** Entry 1 ran N22 on the 5-caliber body at Mach 3.01 at four Reynolds numbers (Table 2 note,
  p. 8; printed pp. 175–176). At 1.1, 1.3, 1.8 and 3.7 million per foot it read 0.116, 0.111, 0.099 and 0.086.
  Less this note's laminar friction (0.046, 0.042, 0.036, 0.025) that leaves 0.070, 0.069, 0.063 and 0.061: no
  sign of transition at the higher Reynolds numbers. The residual is +0.006 to +0.015 over shock-expansion's
  0.055 at the same Mach number.
- **Table 4's odd cell.** Its 0.0475 (Mach 2, N18, 11-caliber body) breaks the row's +0.0041 step. A misprint for
  0.0495 would move that one cell by 0.002 and the fineness-3 mean by 0.0005. That is too small to matter.

So friction leaves AFATL's excess where it is. The rest (about +0.02 at Mach 2 and +0.011 at Mach 3 in entry 1)
is unbounded by cause: an axial balance zero or calibration error, or a base-cavity pressure that the three base
transducers (p. 4) do not represent. The report gives no way to split them.

## 5. The NACA side: Report 1306 against Report 1386

Report 1386's footnote 6 (p. 13) finds its fineness-2.93 ogive's foredrag about 10% under Report 1306's fineness-3
ogive. The two reports agree on the cone and the hypersonic optimum shape, and reruns found no cause. Against theory:

- **Report 1306 is the high one.** At Mach 2.73 its pressure drag after laminar friction, 0.109, is +17% over
  shock-expansion's 0.093 and +16% over DATCOM's 0.094. Its stated foredrag uncertainty is ±0.001 at the low Mach
  numbers (p. 8), so the excess is not noise. The report states no boundary-layer state. **Inference:** fully
  turbulent friction would remove 0.012 more (0.020 against 0.0075: flat-plate laws with a compressibility factor,
  at 2.1 million). That leaves 0.096, +3% over theory, so transition on its ogive alone could explain most of the
  excess.
- **Report 1386 is close.** At Mach 1.99 it reads 0.103 against 0.104 to 0.105. At 2.86 and 3.06 its bounds are 2%
  to 7% under shock-expansion. The report reads its Mach 3 ogive as partly turbulent (p. 13), judged against its
  0.083 to 0.084 characteristics line; against shock-expansion's 0.095 no turbulence is needed. The 12% spread
  between the two theories decides that.

## 6. What it means for HPR Sim's nose drag

HPR Sim's tangent ogive is the cone formula times 1.0 (the parent note, §7):

| Mach | Fineness | HPR Sim | vs shock-expansion | vs DATCOM | vs AFATL entry 1 |
|--:|--:|--:|--:|--:|--:|
| 2 | 2 | 0.194 | −2% | −4% | −14% |
| 2 | 3 | 0.104 | +4% | +3% | −14% |
| 2 | 4 | 0.068 | +12% | +12% | −17% |
| 3 | 2 | 0.166 | −9% | −7% | −15% |
| 3 | 3 | 0.086 | −6% | −7% | −16% |
| 3 | 4 | 0.054 | −2% | −3% | −20% |

At fineness 3, against the whole Mach 3 theory band (0.083 to 0.092), HPR Sim is −7% to +3%. Against TN 3716 at
Mach 1.98 it is −2% to −3% (the parent note). The −14% to −22% row against AFATL reflects AFATL's offset, not
HPR Sim. Signs for apogee: where HPR Sim's nose drag is low (fineness 2 and 3 at Mach 3) apogee reads slightly
high; where it is high (fineness 4 at Mach 2) apogee reads low, the flattering side for a waiver. The size in
apogee depends on the rocket and is not bounded here.

## What the project can use

All sources are US Government works or cleared for public release. NACA reports and TN 2744 come from NTRS. AFATL
carries Distribution Statement A. USAF DATCOM (AFWAL-TR-83-3048) is marked approved for public release on its DTIC
cover sheet, as is the Missile DATCOM feasibility study. The project may cache them under `refs/` and cite their
numbers; nothing here is committed data.

- **[#616][i616]'s row (a suggestion).** Score against TN 3716 and Report 1386 as measured references, and against
  shock-expansion and DATCOM as a theory line. Carry AFATL as entry-1 values with this offset stated: +0.021 to
  +0.026 at Mach 2 and +0.011 to +0.013 at Mach 3 over shock-expansion (+0.010 to +0.017 at Mach 3 over DATCOM). Or
  keep AFATL only at Mach 1.5 (mean +0.011 to +0.012 over theory, inside its ±0.020). Expected row at fineness 3:
  +3% to +4% at Mach 2, −6% to −7% at Mach 3.
- **A model change (a suggestion).** HPR Sim already runs the second-order shock-expansion method for
  body normal force (`crates/hpr-aero/src/shock_expansion.rs`, [the aero page][aero]). Integrating its zero-angle
  element pressures would give the nose's wave drag for any profile. Expected change (Inference, this note's
  script): fineness 3 at Mach 1.5 falls from 0.130 to 0.107 (−18%); Mach 2 from 0.104 to 0.100 (−4%); Mach 3
  rises from 0.086 to 0.091 (+6%). Fineness 4 at Mach 2 falls 11%; fineness 2 at Mach 3 rises 10%. It needs a
  fallback where the vertex flow is subsonic, and its own validation row.

## Open questions

- Which AFATL balance was right, and what causes the entry-1 offset. AEDC's VKF tunnel A calibration reports and
  the balance calibrations (AFATL refs. 4–5) were not read.
- The 12% theory spread at Mach 3: TN 2250/2399's similarity solutions against shock-expansion and DATCOM. A
  rotational characteristics solution for fineness 3 at Mach 3 would settle it; none was computed here.
- Report 1306's boundary-layer state on its ogive; any sting or cavity effect in either NACA report. NAVWEPS 1488
  vol. 3 was not read.

## Sources

Read 2026-10-19 unless marked. Copies in the gitignored `refs/papers/`; readings in `refs/research/<this slug>/`.

1. Butler, Sears and Pallas, AFATL-TR-77-8 (1977), DTIC ADB031340: pp. 4, 6–8, 10–16; Appendix B, printed
   pp. 119–230. <https://archive.org/download/DTIC_ADB031340/DTIC_ADB031340.pdf>
2. Syvertson and Dennis, NACA TN 3527 (1956): pp. 1–2, 7–9, 15, Fig. 3 p. 41.
   <https://ntrs.nasa.gov/citations/19930084226>
3. Finck, *USAF Stability and Control DATCOM*, AFWAL-TR-83-3048 (1978 revision), DTIC ADB072483: pp. 4.2.3.1-8,
   4.2.3.1-11, Figs. 4.2.3.1-61, 4.2.3.1-63. <https://archive.org/details/DTIC_ADB072483>
4. Van Dyke, NACA TN 2744 (1952): pp. 31–32. <https://ntrs.nasa.gov/citations/19930083480>
5. Perkins and Jorgensen, NACA TN 3716 (1956): p. 1, Table I(a) p. 20. <https://ntrs.nasa.gov/citations/19930084497>
6. Perkins, Jorgensen and Sommer, NACA Report 1386 (1958): pp. 5, 12–13, footnote 6.
   <https://ntrs.nasa.gov/citations/19930091022>
7. Eggers, Resnikoff and Dennis, NACA Report 1306 (1957): pp. 6–8. <https://ntrs.nasa.gov/citations/19930092299>
8. Blake, *Missile DATCOM User's Manual, 1997 Fortran 90 Revision*, AFRL-VA-WP-TR-1998-3009: Table 27, p. 97.
9. Vukelich, *Development Feasibility of Missile Datcom*, ADA127378: p. 35. <https://archive.org/details/DTIC_ADA127378>
10. Ehret, Rossow and Stevens, NACA TN 2250 (1950), and Rossow, NACA TN 2399 (1951), as read in the parent note
    (2026-10-16). <https://ntrs.nasa.gov/citations/19930082890>, <https://ntrs.nasa.gov/citations/19930090560>

[i616]: https://github.com/nrdptel/fusionspace-eridanus/issues/616
[aero]: ../physics/aero.md#bodies-faster-than-sound
