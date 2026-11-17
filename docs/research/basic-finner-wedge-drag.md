# The Basic Finner's wedge fins: face wave drag and trailing-edge base drag

**In short: the wedge faces add little drag; the blunt trailing edges add 1.3 to 3 times HPR Sim's rule.** Exact
oblique-shock theory gives the four wedges' faces 0.012–0.043 of C_X (on π d²/4) from Mach 3.15 down to 1.29; linear
theory reads 5–8% low. At Mach 1.077 the shock is detached: transonic theory gives 0.110, linear theory 26% less.
Turbulent two-dimensional tests put the trailing edges' base drag at 0.10–0.12 (Mach 3.15) to 0.37–0.49 (1.29) on the
base area; HPR Sim's 0.25/M is 21–68% low, most at low Mach. The bases add 0.04–0.20 to C_X (0.24–0.29 at 1.077), above
the 0.04–0.08 "rough bound" in [finner-reference-data](finner-reference-data.md) at Mach 1.832 and below. The wave drag
is closed-form theory, sure to a few percent; base values are read off figures (±0.01; ±0.05 at 1.077). How a fin base
flush with the body's base behaves is unbounded. "Inference" marks this note's own reasoning. Readings not in
`basic-finner-wedge.json` (neighbouring points, checks, §4–5's other figures) were read by eye and are not committed.

## 1. Geometry and conversions

Each face is inclined at δ = atan(0.04) = 2.291° (0.03998 rad). Both faces of a fin give c_d = 2 C_p tan δ = 0.08 C_p
on the chord, and four fins of planform d × d give C_X = (16/π) c_d = 5.093 c_d. The four 0.08 d × 1 d bases total
0.32 d², 0.4074 of the reference area: C_DB = −C_pb on the base area, C_X = 0.4074 C_DB.

## 2. Face wave drag, shock attached (Mach 1.293 to 3.147)

**The exact faces carry 5–9% more pressure than linear theory.** θ is the weak root of NACA 1135 eq. 139a [S1] p. 621
(PDF p. 10); C_p = 4(M² sin²θ − 1)/[(γ+1)M²] is eq. 145, second order the first two terms of eq. 152 [S1] p. 622 (PDF
p. 11), and linear C_p = 2δ/√(M² − 1). All computed with γ = 1.4 (Inference: evaluated, not read off a chart).

| Mach | θ | C_p exact | C_p linear | C_p 2nd order | c_d exact | c_d linear | linear/exact | C_X, 4 fins |
|-----:|--:|----------:|-----------:|--------------:|----------:|-----------:|-------------:|------------:|
| 1.293 | 54.44° | 0.1060 | 0.0976 | 0.1047 | 0.00848 | 0.00780 | 0.921 | 0.043 |
| 1.832 | 35.11° | 0.0547 | 0.0521 | 0.0546 | 0.00438 | 0.00417 | 0.952 | 0.022 |
| 2.375 | 26.63° | 0.0393 | 0.0371 | 0.0393 | 0.00315 | 0.00297 | 0.944 | 0.016 |
| 2.718 | 23.23° | 0.0338 | 0.0316 | 0.0337 | 0.00270 | 0.00253 | 0.937 | 0.014 |
| 3.147 | 20.12° | 0.0289 | 0.0268 | 0.0288 | 0.00231 | 0.00214 | 0.928 | 0.012 |

**The shock detaches below Mach 1.134:** eq. 168 (greatest deflection) solved for δ = 2.291°; eq. 170, the thin-wedge
form, gives 1.139, and eq. 167 puts sonic flow behind the shock at 1.141 [S1] p. 624 (PDF p. 13). Chart 2 was not read.

## 3. Mach 1.077: detached bow wave

**Transonic small-disturbance theory puts the face drag 35% above linear theory, as an upper bound.** Vincenti and
Wagoner's double wedge has a front wedge of slope t/c ending in a sharp convex corner; its results apply "within minor
limitations" to any such wedge [S2] p. 3 (PDF p. 6). The fin is that front wedge, t/c = 0.04, half their chord.
**Inference:** the flow is sonic at the corner, so what lies behind it (rear wedge or base) cannot reach the face.

- **Similarity parameter** ξ₀ = (M² − 1)/[(γ+1) t/c]^(2/3) [S2] p. 3: **0.763** at Mach 1.077 (cases span 0–1.058).
- **Front-wedge drag,** generalized as c̃_d = (γ+1)^(1/3) c_d/(t/c)^(5/3) on their chord [S2] p. 2, eq. 10a p. 8.
  Fig. 7 by pixels: **c̃_df = 3.09 ± 0.05** [S2] p. 9 (PDF p. 12); its points at ξ₀ = 0.70, 0.92 read 3.03, 3.19. On
  the fin's chord: **c_d = 0.0216** (0.0212–0.0219), face C_p 0.270, four fins **C_X = 0.110** (0.108–0.112).
- **Linear theory** gives c̃_df = 2/√ξ₀ = 2.29 (half of eq. 14, [S2] p. 10): c_d = 0.0160, C_p = 0.200, C_X = 0.081.
  It **under-estimates by 26%**; the report calls it "inherently incapable" of mixed flow [S2] p. 10.
- **Bounds.** The report calls its results "an approximate upper bound" for a finite wing, since finite span lowers
  transonic drag [S2] p. 11 (PDF p. 14). The lower side is unbounded.

## 4. Trailing-edge base pressure, turbulent boundary layer

**Two-dimensional tests agree within ±0.03 from Mach 1.5 up; below 1.3 they scatter and rise steeply.** Sources:
Hoerner's Fig. 11, measured blunt trailing edges [S3] p. 16-11 (PDF p. 334); Love's Fig. 5, data compilations and his
estimates [S4] p. 34 (PDF p. 36). Both read by pixels, checked on their vacuum curves −2/(γM²) to 0.004 and 0.008.

| Mach | vacuum | Hoerner Fig. 11 | Love Fig. 5 | used here | 0.25/M | rule low by | base C_X, 2-D | rule C_X |
|-----:|-------:|----------------:|------------:|----------:|-------:|------------:|--------------:|---------:|
| 1.077 | 1.232 | 0.59–0.72 | 0.43–0.49 (estimates) | 0.59–0.72 | 0.232 | 61–68% | 0.24–0.29 | 0.095 |
| 1.293 | 0.854 | 0.40–0.49 | 0.37–0.41 | 0.37–0.49 | 0.193 | 48–61% | 0.15–0.20 | 0.079 |
| 1.832 | 0.426 | 0.26–0.27 | 0.24–0.27 | 0.24–0.27 | 0.136 | 43–49% | 0.10–0.11 | 0.056 |
| 2.375 | 0.253 | 0.16–0.18 | 0.16–0.18 | 0.16–0.18 | 0.105 | 34–42% | 0.065–0.073 | 0.043 |
| 2.718 | 0.193 | 0.15 | 0.13–0.14 | 0.13–0.15 | 0.092 | 29–39% | 0.053–0.061 | 0.037 |
| 3.147 | 0.144 | 0.11–0.12 | 0.10–0.12 | 0.10–0.12 | 0.079 | 21–34% | 0.041–0.049 | 0.032 |

- **Direct check.** Tripped blunt wings, Fig. 8 of [S5] p. 8 (PDF p. 11), at this fin's c/[h Re^(1/5)] = 0.68–0.84 (c/h
  = 12.5, chord Reynolds number 0.7–2.1 × 10⁶): p_b/p∞ = 0.50, 0.36, 0.20, so C_DB = 0.32 at Mach 1.5, 0.23 at 2.0, 0.12
  at 3.1 (Inference: × 2/(γM²)). Hoerner's example, 0.65 of vacuum at C_fB = 0.12, is at Mach 1.5 (0.39) [S3] pp. 16-12
  to 16-13; at Mach 2 it gives 0.23 too (Inference); this fin's C_fB is 0.10–0.13 (Inference: 2 × 0.074 Re^(−1/5) ×
  c/h).
- **Robust to shape.** With turbulent flow, Reynolds number, profile, boattail angle and angle of attack mattered
  little; Mach number dominated [S5] p. 1 (PDF p. 4). So the wedge's 2.3° flare should not matter (Inference).
- **Mach 1.077 is the weak point.** Love's data start near Mach 1.16. Hoerner's peak near 0.8 at Mach 1.0 and fall
  steeply: ±0.01 in Mach moves C_DB by ±0.05. Below Mach 1.3 tests exceed Korst's theory [S3] p. 16-12.
- **Why the rule reads low. Inference:** 0.25/M (Niskanen 2009 eq. 3.94, after Fleeman; `base_drag_coefficient`) is
  a body-of-revolution rule. Love expects axisymmetric base pressures "considerably higher" than two-dimensional ones
  at low and moderate supersonic speeds, about equal at high speeds [S4] p. 11 (PDF p. 13): the table's sign and trend.

## 5. Fins flush with the body's base

**No source read measures the fin's own base pressure there; it is unbounded.** They measure the body's base instead.

- **Hoerner:** fins move a straight body's base-pressure coefficient by about ±10%; their pressure field matters little
  when they adjoin or sit ahead of the base [S3] p. 16-11.
- **Love:** four 10% circular-arc fins (sharp edges) flush with the base lowered the body's base pressure, more so for
  thicker fins and less at higher Mach [S4] pp. 19–22 (PDF pp. 21–24). He asks for more fin tests [S4] p. 24.
- **Hart, flight, Mach 0.7–1.3:** rectangular-section fins, 0.091 in thick on a 5 in body, ending 0.24 in ahead of the
  base, *raised* its base pressure: base drag about 0.235 bare, 0.21–0.22 finned at Mach 1.05–1.12 (Fig. 8, by eye).
  He credits junction air and puts the fins' low pressures "just behind the blunt trailing edge" [S6] pp. 3, 5, 16.
- **Spanwise:** blunt wings' base pressure departs from midspan both ways near tip and junction [S5] p. 7, Fig. 7 p. 8.

## What the project can use

- **Terms.** S1, S2, S4–S6 are NACA works, public domain in the US, cached in `refs/papers/`; Hoerner: cite only.
- **The wedge in the comparison (M1.14b2, done).** 5.093 c_d (§2–3) plus 0.4074 C_DB (§4) replace the drawn fins'
  edge terms; the readings are in `validation/fixtures/aero/basic-finner-wedge.json`, with Mach 1.077's base band
  widened to 0.43–0.72 to hold Love's estimate too. The aerodynamics page gives the result.
- **The rule itself.** If 0.25/M is as low on other thin blunt fin edges, square-finned rockets get too little drag
  above Mach 1 and apogees read high; not sized here. [finner-reference-data](finner-reference-data.md)'s C_pb of
  −0.1 to −0.2 holds only above about Mach 2 (outside at 1.832, inside at 2.375).

## Open questions

- The fin's base pressure beside the body's base wake and at aspect ratio 1; finite-span relief at Mach 1.077.
- Base pressure at Mach 1.0–1.15 rests on one steep figure; Nash's R&M 3427 ([fin-edge-drag](fin-edge-drag.md)) may
  help. Chapman's TR 1051 is cached, unread; Love's TN 3601 and Goin's RM L52D21 were not read.

## Sources (all read 2026-11-16; printed page numbers unless "PDF")

1. **S1.** Ames Research Staff, *Equations, Tables, and Charts for Compressible Flow*, NACA Report 1135, 1953.
   <https://ntrs.nasa.gov/citations/19930091059>
2. **S2.** W. G. Vincenti, C. B. Wagoner, *Transonic Flow Past a Wedge Profile with Detached Bow Wave*, NACA Report
   1095, 1952. <https://ntrs.nasa.gov/citations/19930091088>
3. **S3.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, ch. XVI. <https://archive.org/details/FluidDynamicDragHoerner1965>
4. **S4.** E. S. Love, *Base Pressure at Supersonic Speeds on Two-Dimensional Airfoils and on Bodies of Revolution with
   and without Fins Having Turbulent Boundary Layers*, NACA TN 3819, 1957. <https://ntrs.nasa.gov/citations/19930084517>
5. **S5.** D. R. Chapman, W. R. Wimbrow, R. H. Kester, *Experimental Investigation of Base Pressure on Blunt-Trailing-
   Edge Wings at Supersonic Velocities*, NACA Report 1109, 1952. <https://ntrs.nasa.gov/citations/19930090968>
6. **S6.** R. G. Hart, *Effects of Stabilizing Fins and a Rear-Support Sting on the Base Pressures of a Body of
   Revolution in Free Flight at Mach Numbers from 0.7 to 1.3*, NACA RM L52E06, 1952.
   <https://ntrs.nasa.gov/citations/19930087048>
