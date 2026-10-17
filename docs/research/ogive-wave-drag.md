# Measured supersonic pressure drag of tangent-ogive noses

**In short: four NACA and Air Force reports give a tangent ogive's pressure drag past Mach 1.3, and they say HPR
Sim's nose wave drag is 10% to 28% high near Mach 1.5 at fineness about 3, close (−3% to +6%) near Mach 2, and
inside the references' own spread at Mach 3.** The cleanest point is NACA TN 3716: tabulated surface pressures on a
fineness-3 tangent ogive at Mach 1.98, which integrate to 0.107 to 0.108 ± 0.005 on the base area, against HPR Sim's
0.105. At Mach 1.5 the references give 0.110 to 0.119 against HPR Sim's 0.130, mostly because the cone formula HPR
Sim uses (Hoerner's eq. 29, Niskanen's B.4) reads 15% above exact cone theory there. The ~0.0007 that
[#3391][or3391] reports for OpenRocket 24.12 at Mach 1.5 misses nearly the whole value. How sure: every source's
mean shows the Mach 1.5 excess; at Mach 2 to 3 the measurements disagree with each other by 10% to 20%, so HPR Sim's
error there is not settled. It moves the supersonic nose-drag row the 2026-10-15 review's idea asks for,
[#222][i222], M2.3c6's fineness-3 probe and M1.14d.

**What this covers.** Which reports hold a pressure-only reference for a sharp tangent ogive at zero angle of attack
from Mach 1.3 to 3, on the base area; where each value is; how friction was removed; and what HPR Sim's model reads
beside it. **What it is for.** Building the validation row and deciding whether the nose model needs to change.
**How far to trust it.** Report numbers are read from tables where a table exists, else off figures with the read
precision stated. **Inference** marks this note's own arithmetic: friction subtracted by the reports' laminar
method, a pressure table integrated, a Taylor–Maccoll cone solved (`tm.py`; a 10° cone at Mach 2 gives 31.2°).

## 1. Stoney's TR R-100 holds no ogive

Stoney's Fig. 12 (p. 16) plots fineness-3 nose pressure drag for a cone, x^1/2 and x^3/4 noses, three parabolas,
von Kármán, L-V Haack, and, in panel (b), an x^1/4 nose and an ellipsoid [S1]. No ogive appears in it, in Fig. 11
(Mach 1.4 against fineness, p. 16) or in the summary figures 6 to 10 (pp. 12–15). Flight "pressure drag" in R-100
is total drag less flat-plate friction (its Fig. 2, wholly laminar or wholly turbulent assumed), base and fin drag
(pp. 8–9). Panel (b) is credited to Perkins and Jorgensen, NACA RM A52H28 [S1] p. 19, ref. 30. That memo is
superseded by NACA Report 1386 [S2] p. 1, which does test a tangent ogive (§2). Niskanen's thesis says no ogive
Mach-curve data was found, and so assumes a tangent ogive drags as the cone of the same length [S8] p. 109.

## 2. NACA Report 1386: a fineness-2.93 tangent ogive, foredrag and characteristics theory

Perkins, Jorgensen and Sommer (1958) tested model 16, the "L-V ogive": a circular-arc tangent ogive 5.25 in long on
a 1.79 in base, fineness 2.93, sized to the L-V Haack's length and volume [S2] pp. 3–4, Fig. 1(e).

- **Measurement.** Ames 1- by 3-foot tunnels; balance total drag less measured base pressure, so the foredrag
  *includes friction* [S2] pp. 2–4. Reynolds number on model length 1.2 to 4.1 million (table, p. 4). Stated
  uncertainty ±0.004 in foredrag; repeat runs agree within 0.002 [S2] p. 5.
- **Theory beside it.** The ogive's wave drag by characteristics (the report's ref. 11, TN 2250), plus flat-plate
  laminar (Blasius) or turbulent friction at 4 million [S2] pp. 5–6, 12–13.
- **Where.** Fig. 11(b), p. 12, read here off a 600-dpi scan, grid calibrated (symbols ±0.001, lines ±0.002).
- **Boundary layer.** Laminar on the cone at all Mach numbers; at Mach 3 the ogive's was at least partly turbulent,
  and at Mach 3.67 its foredrag exceeds even the turbulent estimate [S2] p. 13. So friction-subtracted values at
  Mach 3 and above are upper bounds on its pressure drag.
- **A known spread.** Its footnote 6 (p. 13) says these ogive foredrags are about 10% lower than Report 1306's
  fineness-3 ogive (§5), unexplained after reruns, while the two agree on the cone.

| Mach | Re, millions | Foredrag, Fig. 11(b) | Blasius friction | Pressure (Inference) | HPR Sim (f 2.93) | Error |
|--:|--:|--:|--:|--:|--:|--:|
| 1.24 | 2.42 | 0.110 | 0.0068 | 0.103 | n/a (cubic) | n/a |
| 1.44 | 3.14 | 0.116 | 0.0060 | 0.110 | 0.141 | +28% |
| 1.99 | 2.01 | 0.110 | 0.0074 | 0.103 | 0.108 | +6% |
| 2.86 | 4.00 | 0.095 to 0.099 | 0.0053 | ≤ 0.090 to 0.094 | 0.091 | ≥ −3% |
| 3.06 | 4.00 | 0.093 to 0.097 | 0.0053 | ≤ 0.088 to 0.092 | 0.089 | ≥ −3% |
| 3.67 | 3.45 | 0.096 | 0.0057 | ≤ 0.090 | 0.083 | ≥ −8% |

Friction is the report's own Blasius rule, $1.328/\sqrt{\mathrm{Re}}$, which it applies at 4 million only; here it
is taken at each run's Reynolds number, times this ogive's wetted-to-base area ratio 7.95 (integrated here). The
Mach 2.86 and 3.06 ranges span two separate readings; turbulent friction there would lower them by up to 0.010
(HPR Sim up to about +10%). The
characteristics wave drag in the same figure reads 0.110 to 0.111 at Mach 1.5, 0.106 at 2.0, 0.085 at 3.0 and 0.078
at 3.6, where HPR Sim is +21% to +22%, +2%, +5% and +7%. Its laminar-foredrag line sits 0.005 above that line, which
identifies it among the overlapping theory lines near Mach 1.5.

## 3. NACA TN 3716: integrated surface pressures at Mach 1.98

Perkins and Jorgensen (1956) measured pressures on a fineness-3 circular-arc ogive tangent to a 7-caliber cylinder at
Mach 1.98, including zero angle of attack [S3] p. 1. Table I(a), p. 20, tabulates one averaged pressure coefficient
at each of 15 stations (x/d 0.22 to 8.06) at 0.39 million per inch; the variation around the body was under ±0.002,
and the stated uncertainty in pressure coefficient is ±0.005 (pp. 5–6). Van Dyke's second-order theory predicts this
zero-lift distribution adequately [S3] p. 1. This is a pressure-only reference: no friction enters it.

**Inference.** Integrating the eight stations on the nose against $\mathrm{d}(r/R)^2$, with the tip from a
Taylor–Maccoll cone at the ogive's 18.9° tip half-angle, gives 0.107 to 0.108 by interpolation, tip value and the
readings of two blurred cells (x/d 0.89, 2.95), ±0.005 from the pressures (`integrate_tn3716.py`). HPR Sim gives
0.105 at Mach 1.98: **−2% to −3%**, inside the uncertainty.

## 4. AFATL-TR-77-8: fineness 2, 3 and 4, friction subtracted by the report

Butler, Sears and Pallas (1977) tested 2-, 3- and 4-caliber tangent ogives (sharp: N14, N18, N22) on 5- to
11-caliber cylinders plus a 1-caliber cylindrical afterbody, in AEDC's VKF tunnel A at Mach 1.5, 2, 3 and 4, 1.8
million per foot, 1.2 in models [S4] pp. 2–5 and its list of symbols. Table 3 (pp. 10–13) tabulates forebody axial
force (total less base) at zero angle of attack; Table 4 (p. 16) tabulates a computed fully laminar friction for
each body. The report subtracts the second from the first to get nose pressure drag; schlieren and drag showed no
transition (p. 15). Stated uncertainty in axial force is ±0.010 to ±0.021 (pp. 6–7). Its Fig. 27 (pp. 59–60)
compares the result with DATCOM as a carpet plot that cannot be read to better than about 0.02.

| Fineness | Mach | Table 3 less Table 4 (Inference), four cylinders | Mean | HPR Sim | HPR Sim error (range) |
|--:|--:|--|--:|--:|--|
| 2 | 1.5 | 0.214, 0.233, 0.233, 0.218 | 0.225 | 0.232 | +3% (−1% to +9%) |
| 2 | 2.0 | 0.227, 0.223, 0.224, 0.226 | 0.225 | 0.194 | −14% |
| 2 | 3.0 | 0.208, 0.195, 0.197, 0.197 | 0.199 | 0.166 | −17% |
| 3 | 1.5 | 0.123, 0.136, 0.103, 0.113 | 0.119 | 0.130 | +10% (−4% to +27%) |
| 3 | 2.0 | 0.119, 0.120, 0.125, 0.133 | 0.124 | 0.104 | −16% |
| 3 | 3.0 | 0.113, 0.101, 0.104, 0.102 | 0.105 | 0.086 | −18% |
| 4 | 1.5 | 0.070, 0.085, 0.069, 0.077 | 0.075 | 0.088 | +17% (+3% to +27%) |
| 4 | 2.0 | 0.081, 0.082, 0.083, 0.084 | 0.083 | 0.068 | −17% |
| 4 | 3.0 | 0.075, 0.065, 0.068, 0.069 | 0.070 | 0.054 | −22% |

The scatter across cylinder lengths (up to 0.033 at Mach 1.5) is the size of the stated uncertainty; Table 4's
0.0475 (Mach 2, 3-caliber, 11-caliber) breaks its row's pattern, perhaps a misprint for 0.0495. At Mach 2
and 3 this data set sits 0.014 to 0.021 above §2 and §3 on the same fineness, which no stated cause explains.
Turbulent friction would lower it, not raise it.

## 5. NACA Report 1306: a fineness-3 tangent ogive from Mach 2.73

Eggers, Resnikoff and Dennis (1957) tested a fineness-3 tangent ogive (arc radius 9.25 diameters) at Mach 2.73 to
6.28; foredrag is balance drag less base, uncertain by ±0.001 at the low Mach numbers [S5] pp. 6–8. Fig. 6 (p. 8),
read here off a 600-dpi scan (±0.002): 0.116 at Mach 2.73, 0.112 at 3.0 (on the curve; no test point there), 0.105
at 3.5. Less Blasius friction at 2.1 million (0.0075, **Inference**) that is 0.109 at Mach 2.73, where HPR Sim's
0.089 is −18%. This is the source Report 1386 found 10% higher than its own (§2).

## 6. Theory, labeled as theory

- **Characteristics, hypersonic similarity.** Ehret, Rossow and Stevens computed ogive-cylinders by characteristics
  at fineness 3 to 12 [S6]. Pressure drag times $\gamma M^2/2$ is a function of $K = M/f$ alone (Fig. 11, p. 27);
  Rossow added the entropy (rotation) term, negligible at $K = 0.5$ and −30% at $K = 2$ [S7] p. 7. His Fig. 7
  (p. 25) reads 0.176 at $K = 0.5$ (computed at Mach 3, fineness 6 and Mach 6, fineness 12) and 0.52 to about 0.57
  at $K = 1$ (Mach 3, fineness 3). So, as the rule maps them: fineness 3 at Mach 1.5 is 0.112 (HPR Sim +17%);
  fineness 3 at Mach 3 is 0.083 to 0.090 (HPR Sim −5% to +4%); fineness 6 at Mach 3 is 0.028 (HPR Sim +4%).
  Mach 1.5 at fineness 3 is below the cases computed, so that value leans on the rule.
- **Taylor–Maccoll cones (Inference).** At fineness 3 the exact cone gives 0.113 at Mach 1.5, 0.096 at 2 and 0.080
  at 3; HPR Sim's cone reads 15%, 9% and 8% above. At fineness 4: 23%, 13%, 8%; at fineness 2: 7–8% throughout.
  Report 1386 found its measured cone foredrag "closely estimated" by exact theory plus laminar friction [S2] p. 13.
- **Why HPR Sim reads high near Mach 1.5.** Its cone is Hoerner's eq. (29), which he offers because low-supersonic
  tests "consistently seem to indicate" more drag than infinite-cone theory [S9] p. 16-20. At fineness 3 two
  measured cones side with exact theory: Report 1386's, and Stoney's flight cone, which the project's aero page
  finds HPR Sim's cone 15% above at Mach 1.5. HPR Sim's tangent ogive equals that cone ([S8] eq. B.8, factor 1.0);
  characteristics put the ogive 6% below the exact cone at Mach 1.5 and 7% above it at Mach 2 (§2, fineness 2.93);
  Hoerner, giving no Mach number, expects a true ogive's wave drag possibly 10% above the cone's [S9] p. 16-21.

## 7. Worked example: fineness 3 at Mach 1.5

$\tan\varepsilon = 1/6$, so $\sin\varepsilon = 0.1644$. HPR Sim's tangent ogive, eq. B.4 times 1.0:

$$
\begin{aligned}
C_D &= 2.1 \sin^2\varepsilon + \frac{0.5 \sin\varepsilon}{\sqrt{M^2 - 1}} \\
&= 0.0568 + 0.0735 \\
&= 0.130.
\end{aligned}
$$

Beside it at fineness 3: 0.112 by similarity (§6), AFATL's 0.119 (0.103 to 0.136), the exact cone's 0.113; HPR Sim
is 0.011 to 0.018 high, +10% to +17%. At Report 1386's fineness 2.93, HPR Sim gives 0.135 against characteristics'
0.110 to 0.111 (+21% to +22%) and 0.141 at Mach 1.44 against the measured 0.110 (+28%). On a minimum-diameter rocket
near Mach 1.5 that excess drag lowers predicted apogee, the flattering side for a waiver; its size in apogee depends
on the rocket and is not bounded here. At Mach 1.3, linear in Mach between Report 1386's 1.24 and 1.44 points gives
0.105, where HPR Sim's 0.161 (fineness 2.93) is +53%.

## What the project can use

All sources but S8 and S9 are US Government works on NTRS or DTIC (AFATL-TR-77-8 carries Distribution Statement A);
the project may cache them under `refs/` and cite their numbers. S8 (CC BY-NC-ND) and S9 (Hoerner's book) are cited
only. [#3391][or3391] is cited as an issue report. Nothing here is committed data.

- **A supersonic nose-drag row**, from tabulated values first: TN 3716 (fineness 3, Mach 1.98, pressure only);
  AFATL Table 3 less Table 4 (fineness 2, 3, 4 at Mach 1.5, 2, 3); Report 1386 Fig. 11(b) less Blasius laminar
  friction (fineness 2.93 at Mach 1.44, 1.99, 2.86, 3.06). That meets the idea's done-when: three or more
  Mach numbers from 1.3 to 3, two or more fineness ratios, each value pressure-only or friction stated. Expected
  row today: +10% to +28% near Mach 1.5 at fineness 2.93 to 4 (+3% at fineness 2); −3% to +6% near Mach 2
  against §2 and §3 but −14% to −17% against AFATL; no lower than −3% at Mach 3 against §2 (a bound) but −17% to
  −22% against AFATL. The row should carry both readings at Mach 2 to 3.
- **[#222][i222] and #3391.** Every reference's mean puts a fineness-3 ogive at 0.110 to 0.119 near Mach 1.5;
  OpenRocket's ~0.0007 is low by nearly all of it, HPR Sim's 0.130 high by 10% to 17%. The nose share of #222's
  gap is mostly OpenRocket's; M2.3c6's probe can set these references beside both at Mach 1.5 and 2.
- **A model change for M1.14d (a suggestion).** A tangent-ogive curve from characteristics (Report 1386's
  fineness-2.93 line, carried to other fineness by the similarity rule of §6) in place of the cone times 1.0.
  Expected: fineness 3 at Mach 1.5 falls from 0.130 to 0.107–0.112 (−14% to −18%); Mach 2 and 3 move under 4%.
  An exact cone alone would not do: at Mach 2 it reads 0.096, 11% to 12% under TN 3716's integrated 0.107 to 0.108.

## Open questions

- DATCOM 4.2.3.1 and its ogive charts were not read; AFATL says agreement with it improves with Mach and fineness
  [S4] p. 15. Van Dyke's TN 2744 tables and NAVWEPS 1488 (Niskanen's ogive source) were not read either.
- Why AFATL reads 0.014 to 0.021 above TN 3716 and Report 1386 at Mach 2 to 3, and why Report 1306 reads 10% above
  Report 1386, stays unbounded: a balance or base-pressure bias is possible in any of them.
- No measured pressure-only ogive at fineness 5 to 7 from Mach 1.3 to 2.5 was found; fineness 6 is theory only.
- R-100 does not say whether its Fig. 12(b) curves (RM A52H28 reports foredrag) still hold friction, about 0.005;
  HPR Sim carries two of them.

## Sources

Read 2026-10-16. Copies in the gitignored `refs/papers/`; reads and scripts in `refs/research/ogive-wave-drag/`.

- **S1.** Stoney, NASA TR R-100 (1961). <https://ntrs.nasa.gov/citations/19630004995>
- **S2.** Perkins, Jorgensen and Sommer, NACA Report 1386 (1958). <https://ntrs.nasa.gov/citations/19930091022>
- **S3.** Perkins and Jorgensen, NACA TN 3716 (1956). <https://ntrs.nasa.gov/citations/19930084497>
- **S4.** Butler, Sears and Pallas, AFATL-TR-77-8 (1977), DTIC ADB031340.
  <https://archive.org/download/DTIC_ADB031340/DTIC_ADB031340.pdf>
- **S5.** Eggers, Resnikoff and Dennis, NACA Report 1306 (1957). <https://ntrs.nasa.gov/citations/19930092299>
- **S6.** Ehret, Rossow and Stevens, NACA TN 2250 (1950). <https://ntrs.nasa.gov/citations/19930082890>
- **S7.** Rossow, NACA TN 2399 (1951). <https://ntrs.nasa.gov/citations/19930090560>
- **S8.** Niskanen, MSc thesis (2009), Appendix B, pp. 106–110; pinned as `niskanen-2009-thesis`, from the
  OpenRocket release tagged `Development_of_an_Open_Source_model_rocket_simulation-thesis-v20090520`.
  <https://github.com/openrocket/openrocket/releases>
- **S9.** Hoerner, *Fluid-Dynamic Drag* (1965), pp. 16-20 to 16-22. Book; cited only.

[or3391]: https://github.com/openrocket/openrocket/issues/3391
[i222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
