# Measured supersonic pressure drag of fineness 5 to 7 ogive noses

**In short: one wind-tunnel report measures a slender tangent ogive's pressure drag in this range, and by it HPR
Sim's nose wave drag is 38% high (+22% to +60%) at fineness 5.75 and Mach 1.98.** NACA RM A53E01 tabulates surface
pressures on that nose; they integrate to 0.030 ± 0.004 on the base area, against HPR Sim's 0.041. No measurement at
fineness 5 near Mach 1.5 was found. Two theories that land within 7% of the measurement (second-order shock-expansion
and DATCOM's ogive chart) put a fineness-5 tangent ogive at 0.042 to 0.044 at Mach 1.5 and 0.040 to 0.041 at Mach 2.
HPR Sim's eq. B.4 gives 0.065 (+48% to +54%) and 0.050 (+21% to +22%); the excess grows with fineness (+76% to
+90% at fineness 7, Mach 1.5). Hypersonic similarity carries fineness-3 references to fineness 5 only from about Mach
2.5 up, within their scatter there. How sure: the sign is sure; its size at Mach 2 rests on one measurement (±13%)
and two theories; Mach 1.5 is theory. It moves [#616][i616]'s row and the ogive curve of [M1.14d][rm].

**What this covers.** A pressure-only reference for a sharp tangent (or secant) ogive at fineness 5 to 7, Mach 1.3
to 2.5, zero angle, base area: the gap the [ogive wave drag note](ogive-wave-drag.md) and the [reference spread
note](ogive-drag-reference-spread.md) left. **What it is for.** A third fineness for the row; sizing a new nose curve.
**How far to trust it.** Tables are copied and checked against the page images; charts are read by pixels.
**Inference** marks this note's arithmetic (scripts in the gitignored `refs/research/ogive-wave-drag-slender/`).

## 1. NACA RM A53E01: pressures on a fineness-5.75 tangent ogive at Mach 1.98

Perkins and Kuehn (1953) measured pressures on a 33⅓-caliber tangent ogive (fineness 5.75) on a cylinder [S1].

- **Facility and model.** Ames 1- by 3-foot supersonic tunnel No. 1, Mach 1.98, 0.5 million per inch (p. 4).
  Model 2: 1.125 in diameter, 14.2 diameters long, one orifice row (Fig. 1, printed p. 25). It was rotated in 15°
  steps, so each station has 19 readings from 0° to 270°, corrected for the stream's nonuniformity (pp. 4–5).
- **Where.** Table II(a), printed p. 21: zero angle, 12 of 29 stations on the nose (x/d 0.44 to 5.75); II(b), 0.9°.
- **Uncertainty.** ±0.004 in pressure coefficient in the plane of symmetry, ±0.006 elsewhere, mostly a shift of a
  whole station's readings (p. 5). At zero angle every meridian is one, so ±0.004 is used (±0.006 would make HPR Sim
  +15% to +71%). The report blames a wavy distribution on slope errors of 0.12° to 0.25°, and low readings ahead of
  x/d 1 on a nose angle slightly smaller than assumed in its theory (p. 8).
- **Friction.** None: pressure only. Linear, second-order and characteristics theory fit its zero-lift data (p. 7).

**Inference.** Averaging each station's 19 readings and integrating against $\mathrm{d}(r/R)^2$, with a
Taylor–Maccoll tip at the ogive's 9.94° half-angle, gives 0.0297 (`integrate_a53e01.py`); a tip held at the first
station's value changes the fourth digit only. A uniform ±0.004 shift moves it ±0.004; the smaller model tip adds at
most +0.001. Table II(b) gives 0.0284, or 0.0278 if two cells are misprints (0.121 at x/d 2.67, 270°; 0.043 at 1.78,
60°): a repeat within 0.002.

| Fineness 5.75, Mach 1.98 | Pressure drag, base area | HPR Sim against it |
|--|--:|--:|
| RM A53E01, integrated (Inference) | 0.030 ± 0.004 | +38% (+22% to +60%) |
| Second-order shock-expansion, TN 3527 (Inference) | 0.032 | +29% |
| DATCOM Fig. 4.2.3.1-61, read here | 0.031 | +32% |
| Taylor–Maccoll cone of the same length (Inference) | 0.034 | +22% |
| HPR Sim, eq. B.4 times 1.0 | 0.041 | |

The two theories sit 5% to 7% above the measurement, inside its ±13%. So the cone formula, not the ogive
assumption, carries most of HPR Sim's excess: B.4 reads 22% above the exact cone of the same length.

## 2. NACA RM A52E22: fineness 3, 5 and 7 noses at Mach 2.75, with friction

Dennis and Cunningham (1952) tested sharp tangent ogives of fineness 3, 5 and 7, noses alone on a 1 in base,
polished steel, in the Ames 10- by 14-inch tunnel at Mach 2.75, 4.01 and 5.00 [S2] pp. 3–4.

- **Measurement.** Balance drag less a measured base pressure, so the foredrag *includes friction* (p. 4). Maximum
  error ±0.004 below 10° angle of attack, usually under half that (p. 5). No boundary-layer state is stated.
- **Where.** Tables IX, X and XI (printed pp. 21–23). Reynolds number about 8 million per foot at Mach 2.75 (Fig. 2,
  p. 27; its curve begins at Mach 2.8, read there), so 2.0 to 4.8 million on nose length (**Inference**).
- **Friction (Inference).** Laminar: Blasius with Eckert's reference temperature and Mangler's transform, the
  shock-expansion surface state as its edge, a 311 K supply assumed (`a52e22_friction.py`, reused `fric.py`).
  The pressure column runs from the 0° reading to the larger ±1° reading, each less laminar friction.

| Fineness | Foredrag at 0° (−1°, +1°) | Laminar friction | Pressure (Inference) | Shock-expansion | HPR Sim |
|--:|--|--:|--:|--:|--:|
| 3 | 0.108 (0.113, 0.113) | 0.009 | 0.099 to 0.104 | 0.093 | 0.089 |
| 5 | 0.053 (0.056, 0.054) | 0.011 | 0.042 to 0.045 | 0.038 | 0.040 |
| 7 | 0.037 (0.045, 0.046) | 0.012 | 0.025 to 0.034 | 0.021 | 0.025 |

**Inference.** At zero angle all three sit 0.004 to 0.006 above shock-expansion: one additive offset, as the spread
note found in AFATL's data. Turbulent friction (0.020 to 0.040, flat-plate) would give the fineness-7 nose negative
pressure drag, so it was mostly laminar. HPR Sim is −11% to +1% of these only because the set reads high (§4).

## 3. Theory at fineness 5 to 7, labeled as theory

Shock-expansion is this note's own run of TN 3527 (`theory.py`, the spread note's `sose.py`), whose stated range is
Mach/fineness 0.4 to 2 [S3] p. 1. DATCOM's ogive chart is read by pixels (±0.005 on its ordinate, `datcom_read.py`),
with $K_N$ 1.003 to 1.006 from Fig. 4.2.3.1-63 at tip half-angles of 8.2° to 11.4° [S4].

| Fineness | Mach | K = M/f | Shock-expansion | DATCOM | Exact cone | HPR Sim | HPR Sim vs the two |
|--:|--:|--:|--:|--:|--:|--:|--|
| 5 | 1.3 | 0.26 | 0.046 † | 0.043 | 0.055 | 0.081 | +76% to +86% |
| 5 | 1.5 | 0.30 | 0.044 † | 0.042 | 0.049 | 0.065 | +48% to +54% |
| 5 | 2.0 | 0.40 | 0.041 | 0.040 | 0.042 | 0.050 | +21% to +22% |
| 5 | 2.5 | 0.50 | 0.039 | 0.039 | 0.038 | 0.0425 | +9% to +10% |
| 6 | 1.5 | 0.25 | 0.032 † | 0.030 | 0.037 | 0.052 | +62% to +72% |
| 6 | 2.0 | 0.33 | 0.030 † | 0.029 | 0.031 | 0.0385 | +30% to +34% |
| 7 | 1.5 | 0.21 | 0.024 † | 0.022 | 0.028 | 0.0425 | +76% to +90% |
| 7 | 2.0 | 0.29 | 0.022 † | 0.022 | 0.024 | 0.031 | +39% to +44% |

† Below shock-expansion's stated range. DATCOM's chart rests on test data plus slender-body and Newtonian theory
(spread note), and at these points reads 3% to 8% under shock-expansion. RM A51J25 Fig. 2 (p. 8) puts linearized
theory's ogive drag about 4% under characteristics near K = 0.4 and Van Dyke's second-order within about 2% [S5].
So the theories converge as K falls; RM A53E01's K = 0.34 measurement (§1) sits 5% to 7% under the two.

**Why HPR Sim's error grows with fineness (Inference).** Eq. B.4's second term is linear in the half-angle. Exact
cone drag falls faster: from fineness 3 to 7 at Mach 1.5 by a factor of 4.0, B.4 by 3.1. B.4 is 32% over the exact
cone at fineness 5 and 50% at fineness 7 (Mach 1.5); ogive theory sits 11% to 21% under its cone. Too much nose
drag lowers predicted apogee, the flattering side for a waiver; its size in apogee is not bounded here.

## 4. Does hypersonic similarity carry fineness 3 to fineness 5?

Ehret, Rossow and Stevens's rule keeps $C_D f^2$ fixed at fixed $K = M/f$ [S6]. Shock-expansion obeys it: its
fineness-3 value at Mach 1.5, scaled, gives 0.0386 at fineness 5, Mach 2.5, against 0.0387 computed there.

- **Fineness 5 at Mach 1.5 (K = 0.30)** needs fineness 3 at Mach 0.90, below Mach 1. No reference can map there.
- **Fineness 5 at Mach 2 (K = 0.40)** needs fineness 3 at Mach 1.2. Report 1386's Mach 1.24 point maps to 0.035
  ± 0.001 at Mach 2.12, 12% under shock-expansion's 0.040: a miss beyond its scatter. TN 2250's Fig. 4(b) (p. 19)
  shades ogives below about Mach 1.65 to 1.75 at fineness 3 to 6 (by pixels) as "doubtful applicability".
- **Fineness 5 at Mach 2.5 (K = 0.5)** maps from fineness 3 at Mach 1.5: AFATL's mean gives 0.043 (0.037 to 0.049
  over its four bodies), Report 1386's Mach 1.44 point 0.038 (0.036 to 0.039); theory, 0.039 to 0.040 (with Rossow's
  characteristics value, parent note §6). That holds within the scatter, though both sources sit in that region.
- **The rule DATCOM's chart uses** keeps $C_D (f^2 + 1/4) K_N$ fixed at fixed $\sqrt{M^2-1}/f$ where each
  fineness's curve has joined the shared envelope (here within 1% to 2%), and reaches lower Mach. Report 1386's Mach
  1.44 point maps to 0.0385 ± 0.0014 at fineness 5, Mach 2.03 (theory 0.041, −5%). RM A53E01 maps to 0.039 ±
  0.005 at Mach 1.79 (theory 0.041 to 0.042, −5% to −7%).
- **A trap.** Mapping a finer nose to a blunter one multiplies an additive offset by $(f_1/f_2)^2$: RM A52E22's
  fineness 7 at Mach 2.75 (K = 0.39) maps to 0.048 at fineness 5, Mach 1.96, 17% over theory; its +0.004 doubles.

## 5. What else was searched

NTRS's search interface took 12 logged queries on 2026-10-19 (205 distinct records, `ntrs-search.txt`);
archive.org's DTIC mirror, "ogive" in the title (39 records). Read and set aside:

- **NASA TM X-3128** [S7]: a fineness-5 tangent ogive on a 7-diameter cylinder, Mach 1.5 and 2.0, 0.38 million on
  diameter. Its forebody axial force, friction on 12 calibers included, is only plotted (Fig. 6(c), p. 25): 0.11 to
  0.13 at zero angle. Friction alone spans 0.026 to 0.12 (laminar to turbulent, **Inference**): the nose is lost.
- **NACA TN 3349** [S8]: fineness-3 and -5 tangent-ogive pressures, ±0.005, but from Mach 3.00 up, and plotted only.
- **AEDC-TR-77-31 for AFATL, DTIC ADB017853**: the same matrix's force tests, noses of 1 to 4 calibers (§2.2, p. 6).
- **NAVWEPS 1488 vol. 3, Fig. 6-2** (printed p. 239): secant-ogive wave drag at fineness 2 to 3.5, no source given.
- **DATCOM 4.2.3.1's ogive chart** cites refs. 18, 35, 38 to 40 (p. 4.2.3.1-11). As read in a partly illegible
  list: fineness-3 blunted cones (RM A52B13), fineness-3 noses at Mach 0.8 to 1.25 (RM L53K17), three transonic
  tests. None is a slender supersonic ogive. Its ref. 14 (Morris, J. Aero. Sci., 1961) was not reached.
- **Stoney's TR R-100** (its reference list, pp. 19–20): free-flight totals with fins; no nose-only ogive data.
- **RM A51J25** [S5], **RM A53F12** (minimum-drag shapes) and **TN 2250**'s A4V1P comparison (Mach 2.47 and 3.24,
  K 0.71 and 0.93) hold no fineness-5 to -7 ogive measurement at Mach 1.3 to 2.5.
- **Not reached:** BRL ballistic-range reports on ogive heads (none indexed by title), Morris 1961, ESDU items.

## 6. Worked example: fineness 5 at Mach 1.5 and 2

$\tan\varepsilon = 1/10$, so $\sin\varepsilon = 0.0995$. HPR Sim's tangent ogive, eq. B.4 times 1.0:

$$
\begin{aligned}
C_D(1.5) &= 2.1 \sin^2\varepsilon + \frac{0.5 \sin\varepsilon}{\sqrt{1.5^2 - 1}} = 0.0208 + 0.0445 = 0.065, \\
C_D(2.0) &= 0.0208 + \frac{0.0497}{\sqrt{3}} = 0.0208 + 0.0287 = 0.050.
\end{aligned}
$$

Beside them: at Mach 1.5, DATCOM 0.042 and shock-expansion 0.044, so HPR Sim is 0.021 to 0.023 high (+48% to
+54%); at Mach 2, theory 0.040 to 0.041, so 0.009 to 0.010 high (+21% to +22%). Against Report 1386 mapped by
DATCOM's rule to Mach 2.03 (0.0385, §4), B.4's 0.049 is +27%. The nose-drag module documents the same 0.0653.

## What the project can use

- **RM A53E01, RM A52E22, TN 3349, RM A51J25, RM A53F12, TN 2250 and TM X-3128**: NACA and NASA reports on NTRS,
  US Government works; the project may cache them under `refs/` and cite their numbers. **DATCOM** as in the spread
  note. **DTIC ADB017853** (approved for public release) and **NAVWEPS 1488 vol. 3** (DTIC AD0613056, a 1961 Navy
  handbook released through OTS) are cited only. Nothing here is committed data; Table II's integration is
  reproducible from the report alone.

## What this suggests

- **[#616][i616]'s third fineness (a suggestion).** Add RM A53E01 as a measured, pressure-only point: fineness 5.75,
  Mach 1.98, 0.030 ± 0.004. Expected row today: HPR Sim +38% (+22% to +60%). RM A52E22's fineness-5 and -7 values at
  Mach 2.75 could join with friction subtracted and their +0.004 to +0.006 offset stated.
- **A curve for [M1.14d, supersonic accuracy][rm] (a suggestion).** Shock-expansion where Mach/fineness ≥ 0.4,
  DATCOM's chart or linear theory below, in place of B.4 times 1.0. Expected at fineness 5: Mach 1.5 falls from
  0.065 to 0.042–0.044 (−33% to −35%), Mach 2 from 0.050 to 0.040–0.041 (−17% to −19%), Mach 2.5 by 8% to 9%.
  At RM A53E01's point it reads 0.031–0.032, inside the measurement's ±0.004. Map fineness-3 rows to fineness 5 only
  at K ≥ 0.5, or by DATCOM's rule from Mach 1.44 up.

## Open questions

- No measurement at fineness 5 to 7 between Mach 1.3 and 1.8 was found; Mach 1.5 rests on theory outside
  shock-expansion's range. A Van Dyke second-order run at K = 0.3 would firm it up; none was computed here.
- Secant ogives at fineness 5 to 7 (no measurement or chart found); RM A52E22's boundary-layer state and offset;
  TN 3349's plotted fineness-5 pressures; Morris (1961); BRL range data.

## Sources

Read 2026-10-19. Copies in the gitignored `refs/research/ogive-wave-drag-slender/dl/`.

- **S1.** Perkins and Kuehn, NACA RM A53E01 (1953): pp. 1, 4–5, 7–8; Table II(a), (b), printed p. 21; Fig. 1,
  printed p. 25. <https://ntrs.nasa.gov/citations/19930087544>
- **S2.** Dennis and Cunningham, NACA RM A52E22 (1952): pp. 3–5; Tables IX to XI, printed pp. 21–23; Fig. 2, p. 27.
  <https://ntrs.nasa.gov/citations/19930087189>
- **S3.** Syvertson and Dennis, NACA TN 3527 (1956): p. 1. <https://ntrs.nasa.gov/citations/19930084226>
- **S4.** Finck, *USAF Stability and Control DATCOM*, AFWAL-TR-83-3048, DTIC ADB072483: pp. 4.2.3.1-11, -21, -22;
  Figs. 4.2.3.1-61, -63. <https://archive.org/details/DTIC_ADB072483>
- **S5.** Seiff and Sandahl, in NACA RM A51J25 (1951): p. 1, Fig. 2 p. 8. <https://ntrs.nasa.gov/citations/20030067331>
- **S6.** Ehret, Rossow and Stevens, NACA TN 2250 (1950): Fig. 4(b) p. 19. <https://ntrs.nasa.gov/citations/19930082890>
- **S7.** Jorgensen and Nelson, NASA TM X-3128 (1974): pp. 2–3, Fig. 6(c) p. 25.
  <https://ntrs.nasa.gov/citations/19750004825>
- **S8.** Savin, NACA TN 3349 (1955): pp. 21–23. <https://ntrs.nasa.gov/citations/19930084190>
- **Also read.** RM A53F12 <https://ntrs.nasa.gov/citations/19930089148>; RM L53K17
  <https://ntrs.nasa.gov/citations/19930087953>; <https://archive.org/details/DTIC_ADB017853>; NAVWEPS 1488 vol. 3
  <https://archive.org/details/DTIC_AD0613056>; TR R-100 <https://ntrs.nasa.gov/citations/19630004995>.

[i616]: https://github.com/nrdptel/fusionspace-eridanus/issues/616
[rm]: ../ROADMAP.md
