# Measured drag rise of finned rockets through Mach 0.8–1.2

**In short: five public sets give finned bodies' whole drag from Mach 0.8 to 1.2 with their geometry, and on the most
HPR-like one HPR Sim's documented formulas read 19% to 38% high (23% to 42% with the fins' square trailing edges),
peaking at Mach 1.08 where the flight peaks at 1.04–1.05.** The sets are NACA free flights and NASA's Arcas Robin tunnel
models. Flat-based bodies hold their subsonic drag to Mach 0.86–0.96, gain most of their rise between Mach 0.95 and
1.02, peak at Mach 1.04–1.12 at 1.36 to 1.51 times the subsonic value, and fall 3% to 10% by Mach 1.2. A 3:1 cone body
starts at Mach 0.85 and peaks at 1.07–1.08; von Kármán and parabola bodies start at 0.94–0.96 and level off. For #67: on
Stoney's noses, flown on one shared body, HPR Sim's cone-minus-von Kármán nose drag is 1.6 to 1.9 times the measured
difference from Mach 0.8 to 1.2; his 3:1 parabola, within 0.7% of a 3:1 tangent ogive's radius, drags 0.000 to 0.116
from Mach 0.9 to 1.2, near MIL-HDBK-762's ogive curve, where HPR Sim gives the ogive the cone's 0.062 to 0.234. How
sure: high on signs and peak Mach, shared by four flight sets; medium on sizes, read off plots (±0.003 to ±0.01), with
HPR Sim's side worked by hand from its formulas. **Inference** marks this note's own readings and arithmetic.

## 1. What HPR Sim does through Mach 1

- **Noses** ([aero](../physics/aero.md#drag-through-mach-1)). Cones take Niskanen's closed form: eq. 3.87's a·M^b below
  Mach 1, a cubic join from Mach 1 to 1.3. A tangent ogive gets the cone's value (shape factor 1.0). Power, parabolic
  and Haack series take Stoney's measured fineness-3 curves, scaled by eq. B.9.
- **Base:** 0.12 + 0.13 M² to Mach 1, then 0.25/M; it peaks at Mach 1.0. Its slope, 0.26 M, passes 0.1 per unit Mach
  from Mach 0.39 (**Inference**). [Cylinder base drag](cylinder-base-drag.md) finds it 0.03–0.10 high at 0.7–1.0.
- **Fins:** a rounded leading edge is charged (1 − M²)^−0.417 − 1, which reaches 1.00 at Mach 0.9 and falls to 0.82 at
  Mach 1 ([fin-edge drag](fin-edge-drag.md)).
- **Known gaps:** the Arcas Robin forebody, fins on, reads +27.3%, +51.0%, +7.1% and +29.8% at Mach 0.8, 0.9, 1.0 and
  1.2; MIL-HDBK-762's worked rocket +20.1% to +31.9% from Mach 0.9 to 1.1
  ([aero](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). Both are one shape each, or a calculation.

## 2. The measured sets

| Id | Source; test, Re on length | Nose | Body l/d; base | Fins |
|---|---|---|---|---|
| HE | Hart, E [R1]; flight, 15–45 × 10⁶ | ogive, 3.5 cal | 11.0; flat, tail nut | 4, LE 45°, 0.091 in, rounded LE |
| SC, SV, SP | Stoney [R2]; gun, 8–15 × 10⁶ | cone, von Kármán, parabola, 3 cal | 12; boattail, 0.19 A | 3, 2.8% |
| S1–S3 | Stoney [R3]; gun, 6–11.6 × 10⁶ | parabolic arc, 7.13 cal | 8.91, 10.63, 12.13; flat | 3, LE 45°, 2.8% |
| RM10 | Jackson et al. [R4]; flight, 33–60 × 10⁶ | parabolic body | 12.2; cut at 81.25% | 4, 60° sweep, 5% |
| AR | Ferris [R5]; tunnel on a sting, 9.8 × 10⁶ per m | 4.71 cal | 18.2; 15° boattail | 4, double wedge 4.4% |

- **HE.** Body 5.00 in across, nose to station 17.50 in, base at 55.06 in; fins from station 46.44 to 54.82 in, tips at
  9.50 in radius, root 8.38 in, tip 1.38 in, rounded leading edges, polished [R1] p. 3, Fig. 1 pp. 10–12. The nose radii
  run 0 to 0.12 in under a tangent ogive's (Table I, p. 9; **Inference**). Error under 4% in C_D [R1] p. 4.
- **SC, SV, SP.** Helium-gun models: a 3-caliber nose, a 4-caliber cylinder and a 5-caliber cone to 0.438 d, 18 in long,
  1.5 in across; fins 1.8 in chord, 0.05 in thick [R2] Fig. 1, p. 10. Error ±0.008 in C_D [R2] p. 4.
- **S1–S3.** Same fins, tips at 2.5 in radius, exposed area 5.505 frontal areas [R3] p. 3, Fig. 1 p. 14; Re p. 5.
- **RM10.** A fineness-15 parabolic arc cut at 81.25% of its length; fins untapered, 10% circular arc normal to the
  edge, 5% streamwise [R4] pp. 3–4. The base is 0.367 of the frontal area [R4] p. 5. Re at Mach 0.85–1.2 [R4] Fig. 4.
- **AR.** Forebody only, base taken off; the short model, from the project's fixture
  ([aero](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)); Mach points [R5] p. 4. The nose is 4.71 d in
  [R5] Fig. 1(a), p. 9; the project's fitted ogive, from TN D-4014, is 4.17 calibers.

## 3. Drag coefficients by Mach number

All on the body's frontal area, zero lift, with base drag included except AR. "Start" is where the slope first reaches
0.1 per unit Mach; values are this note's readings off 300-dpi renders with the grids calibrated (**Inference**), ±0.003
off the rise and ±0.01 on it, except RM10's printed peak.

| Id | Start | Peak (Mach) | 0.8 | 0.9 | 1.0 | 1.1 | 1.2 | Figure |
|---|--:|---|--:|--:|--:|--:|--:|---|
| HE | 0.86–0.88 | 0.504 (1.04–1.05) | 0.333 | 0.346 | 0.473 | 0.487 | 0.453 | [R1] Fig. 10, p. 18 |
| SC | 0.85 | 0.355 (1.07–1.08) | 0.178 | 0.190 | 0.289 | 0.350 | 0.330 | [R2] Fig. 3(a), p. 12 |
| SV | 0.94 | 0.266, level from 1.1 | 0.156 | 0.155 | 0.205 | 0.265 | 0.266 | [R2] Fig. 3(c), p. 13 |
| SP | 0.96 | 0.310 (1.15–1.22) | 0.164 | 0.164 | 0.215 | 0.300 | 0.309 | [R2] Fig. 3(b), p. 12 |
| S1, 8.91 | 0.92 | 0.360 (1.05) | 0.250 | 0.249 | 0.338 | 0.350 | 0.337 | [R3] Fig. 3, p. 16 |
| S2, 10.63 | 0.92–0.93 | 0.368 (1.08–1.12) | 0.273 | 0.270 | 0.336 | 0.368 | 0.356 | [R3] Fig. 3, p. 16 |
| S3, 12.13 | 0.95–0.96 | 0.380 (1.04–1.05) | 0.283 | 0.277 | 0.345 | 0.373 | 0.361 | [R3] Fig. 3, p. 17 |
| RM10 | 0.90 | 0.26 (1.04), printed | | 0.150 | 0.25 | 0.255 | 0.255 | [R4] Fig. 7, p. 17; p. 6 |
| AR | ≤ 0.8 | ≥ 0.686 (1.0–1.2) | 0.330 | 0.420 | 0.686 | | 0.594 | aero page, from [R5] |

- **HE.** The same body with a nose sting (config. G) reads 0.015 to 0.024 higher to Mach 1.0 and peaks at 0.513 at Mach
  1.04 [R1] Fig. 10; Hart calls this within test accuracy and sizes the sting's friction at 0.009 [R1] p. 6. The finless
  body's fore drag (total less base) is 0.075 below Mach 0.9, under the turbulent estimate's 0.092–0.098, so the flow
  was partly laminar [R1] p. 6, Fig. 12 p. 19; it reaches 0.115, 0.154, 0.149 at Mach 1.0, 1.1, 1.2.
- **SC, SV, SP.** The cone body's subsonic excess over the von Kármán body is laid to its sharp shoulder [R2] p. 5.
  Stoney's nose curves, collected in R-100's Fig. 12(a) and carried by HPR Sim [R6], subtract a "tare" (fins, afterbody,
  base) derived at Mach 1.2 (measured total less calculated friction and cone theory), assumed subsonic, and *faired*
  between, from 0.025 to about 0.065 across Mach 0.96–1.03 [R2] pp. 5–6, Fig. 4 p. 13. Each nose curve there carries
  that fairing, up to ±0.02 (**Inference**: half the tare's rise). Differences between two noses do not.
- **S1–S3:** a long nose; the rise is base and fins. **RM10:** half-scale models peak at 0.25 at Mach 1.05 [R4] p. 6.
- **AR.** Thick double-wedge fins and a steep boattail rise from below Mach 0.8, earlier than any flight set.

Reached but not tabulated: Piland and Putland's 30° and 40° cones on a finned body, Mach 0.7–1.3 [R7], a lead for stubby
cones; Hart and Katz's streamline bodies [R8], a poor scan. Hoerner puts drag divergence of fin-stabilized streamline
missiles near Mach 0.95, rising steeply at Mach 1 [R9] p. 16-25. Of the fits in [flight-derived
C_D](flight-derived-cd.md), Kierulf's are subsonic averages and Bawa and Michalski's oscillates near Mach 1 and dips
implausibly near Mach 1.4 (their pp. 8–9).

## 4. Where the peak sits for HPR proportions

HPR rockets carry a nose of fineness 3 to 5, a body of fineness 10 to 20, thin fins and a flat base. HE and S1–S3 are
that shape, apart from S1–S3's long nose; SC to SP add the nose shapes on a boattailed body.

- **Start.** Mach 0.85–0.88 behind a cone or a 3.5-caliber ogive (SC, HE); 0.92–0.96 behind von Kármán, parabola and
  long noses (SV, SP, S1–S3); 0.90 on the RM-10.
- **Sharpness.** 60% to 91% of the rise falls between Mach 0.95 and 1.02 (HE 62%, SC 60%, SV 72%, S3 91%), steepest at
  1.5 to 2.4 per unit Mach (**Inference**, from the table's readings).
- **Peak.** Mach 1.04–1.05 on the flat-based HE, S1 and S3 and on the RM-10; 1.07–1.08 behind the cone; 1.08–1.12 on S2.
  The von Kármán and parabola bodies have no peak: they level off from Mach 1.1 and 1.15.
- **Height.** 1.36 to 1.51 times the Mach 0.9 value with a flat base (S3 1.37, S2 1.36, S1 1.45, HE 1.46 on its Mach 0.9
  value, 1.51 on 0.8); 1.7 to 1.9 on the boattailed bodies, whose subsonic base drag is small.
- **After it.** Down 3% to 10% by Mach 1.2 on flat bases (HE 10%, S1 6%, S2 3%, S3 5%), 7% behind the cone.

**Inference:** for a flat-based HPR rocket with an ogive or von Kármán nose, expect the rise to start near Mach
0.87–0.95, most of it between 0.95 and 1.02, and a peak 1.4 to 1.5 times the subsonic drag at Mach 1.04–1.08.

## 5. HPR Sim against the flights (Inference)

**Hart's body, term by term.** These are HPR Sim's documented formulas worked by hand at HE's geometry: the nose as a
tangent ogive, all-turbulent smooth friction at the middle of Fig. 5's Reynolds band, the fins as HPR Sim's rounded
section (Hart's are rounded in front, square behind [R1] p. 3, a section HPR Sim can't describe), the tail nut left out
(script in `refs/research/transonic-drag-rise/scripts/hpr_hart.py`).

| Mach | friction | nose | base | fins | HPR Sim | measured | error | nose as Stoney's parabola |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 0.8 | 0.133 | 0.012 | 0.203 | 0.048 | 0.396 | 0.333 | +19% | 0.384, +15% |
| 0.9 | 0.127 | 0.045 | 0.225 | 0.080 | 0.476 | 0.346 | +38% | 0.431, +25% |
| 1.0 | 0.125 | 0.141 | 0.250 | 0.070 | 0.586 | 0.473 | +24% | 0.473, 0% |
| 1.1 | 0.121 | 0.210 | 0.227 | 0.071 | 0.630 | 0.487 | +29% | 0.508, +4% |
| 1.2 | 0.118 | 0.172 | 0.208 | 0.073 | 0.571 | 0.453 | +26% | 0.494, +9% |

HPR Sim's estimate peaks at 0.633 at Mach 1.08, against 0.504 at 1.04–1.05, and has gained 0.12 by Mach 0.95 where the
flight has gained 0.054. Charging Hart's square trailing edge adds 0.013 to 0.016: +23%, +42%, +27%, +32% and +29%.
Below Mach 1 most of the excess is the base, which [cylinder base drag](cylinder-base-drag.md) sizes, and some is
laminar flow on the polished model that HPR Sim, fully turbulent by design, can't have. From Mach 1, it is the nose: the
finless body's fore drag less its turbulent friction leaves 0.03–0.05, 0.07–0.09 and 0.07–0.09 of nose pressure drag at
Mach 1.0, 1.1 and 1.2 (the upper ends keep the subsonic laminar saving, 0.011–0.020), against HPR Sim's 0.141, 0.210 and
0.172. Stoney's parabola scaled to fineness 3.5 by eq. B.9 gives 0.028, 0.088 and 0.095.

**Stoney's noses, paired.** The eight bodies share every part but the nose, so the difference between two of them is the
difference between their noses, tare and all [R2] p. 2; Stoney expects nose and afterbody to act apart only above Mach 1
(p. 5), so subsonic differences carry some interference. HPR Sim's nose terms (3:1 cone or tangent ogive from the closed
form, von Kármán from Stoney's curve in the code) against the measured differences:

| Mach | 0.8 | 0.9 | 1.0 | 1.1 | 1.2 |
|---|--:|--:|--:|--:|--:|
| cone − von Kármán, measured (SC − SV) | 0.022 | 0.035 | 0.084 | 0.085 | 0.064 |
| cone − von Kármán, HPR Sim | 0.035 | 0.068 | 0.139 | 0.165 | 0.124 |
| parabola − von Kármán, measured (SP − SV) | 0.008 | 0.009 | 0.010 | 0.035 | 0.043 |
| tangent ogive − von Kármán, HPR Sim | 0.021 | 0.062 | 0.139 | 0.165 | 0.124 |

HPR Sim's cone difference is 1.6 to 1.9 times the measured one at every Mach number. At rest its corner term, 0.8 sin²
9.46° = 0.0216, matches the measured 0.022; the excess is all transonic. The 3:1 tangent ogive and Stoney's parabola
differ by at most 0.69% of the base radius, at 29% of the length, and by 0.8% in volume (**Inference**, computed here),
yet HPR Sim charges the ogive the cone's curve, 2.6 to 14 times the parabola's measured excess.

**MIL-HDBK-762 agrees with the parabola.** Its transonic ogive curve (Fig. 5-113), in its worked example (Table 5-4 col.
14, p. 5-61), gives 0.007, 0.052, 0.109 and 0.117 at Mach 0.9 to 1.2; Stoney's parabola, in HPR Sim's code, 0.000,
0.037, 0.107 and 0.116. The handbook takes Fig. 5-113 from the Navy's Report 1488 (its ref. 67), not from Stoney, and
sends cone users to Stoney's figures because "there is no apparent, reliable correlation" for cones [R10] p. 5-45, refs.
p. 5-76.

## What the project can use

R1 to R8 are NACA and NASA reports on NTRS, each marked public use permitted (checked 2026-10-17), and R10 is
Distribution A: the project may cache them and commit readings with figure and page. R9 is a book: cite only. Nothing
private was read. R2–R4, R7 and R8 are cached under `refs/research/transonic-drag-rise/` with plot renders and scripts;
R1, R5, R6 and R10 were already in `refs/papers/`.

- **A measured transonic drag row for M1.14c**, the stop rule's "adds a reference": HE's whole-rocket C_D at Mach
  0.8–1.2 (flat base, ogive nose, four thin fins), with SC and SV as a paired nose check. Today's expected reading is
  the §5 table: +19% to +38% (+23% to +42% with square trailing edges), peak 0.03–0.04 Mach late. A design file for HE
  is all the row needs.
- **For #67 (a suggestion).** Give tangent ogives Stoney's measured parabola curve, scaled by eq. B.9, in place of the
  cone times eq. B.8; give cones Stoney's measured cone. Expected: a 3:1 tangent ogive at Mach 1.1 from 0.234 to 0.107;
  HE's estimate from +24%, +29%, +26% to 0%, +4%, +9% at Mach 1.0 to 1.2. The Mach 0.8–0.95 excess then is the base
  rule's, which this change leaves.

## What this suggests

The sign is settled: on every flight set that pairs with HPR Sim's formulas, HPR Sim's transonic drag reads high and
rises early, so a flight through Mach 1 simulates low, which understates the altitude a waiver must cover. The largest
single cause from Mach 1 to 1.2 is the closed-form nose: 0.08 to 0.14 on the reference area for the 3:1 and 3.5:1
tangent ogives, 0.055 to 0.080 for the 3:1 cone; below Mach 1 it is the base rule. The measured peak sits at Mach
1.04–1.08 for HPR-shaped bodies, earlier than HPR Sim's 1.08–1.10. The apogee cost is not sized: at the whole-flight
elasticity of −0.31% to −0.57% per 1% of drag ([apogee bias](apogee-bias.md)), a 25% excess all flight would cost 8% to
14% of apogee. That is a scale, not a bound: the excess runs 19% to 42% from Mach 0.8 to 1.2 and is not measured here
below Mach 0.8.

## Open questions

- How much apogee the transonic excess costs on real L2–L3 flights: a flight-by-flight run with the nose and base
  changes toggled one at a time would size it.
- Stoney's nose curves between Mach 0.96 and 1.03 rest on a faired tare; a second fineness-3 data set with measured base
  pressure through Mach 1 would close the ±0.02.
- No set has a body over fineness 12.2 with a flat base, or a jet; whether fineness 15–20 moves the peak is unbounded.
- Allen's 2001 and 2005 tunnel tables (finned-body axial force at Mach 0.6, 0.9, 1.2 only) were not read for drag.

## Sources, all read 2026-10-17

- **R1.** Hart, NACA RM L52E06 (1952): pp. 2–6, Table I p. 9, Figs. 1, 5, 10, 12.
  <https://ntrs.nasa.gov/citations/19930087048>
- **R2.** Stoney, NACA RM L53K17 (1954): pp. 2–6, Figs. 1, 3, 4 (pp. 10–13).
  <https://ntrs.nasa.gov/citations/19930087953>
- **R3.** Stoney, NACA RM L53I01 (1953): pp. 3–5, Figs. 1, 3 (pp. 14–17). <https://ntrs.nasa.gov/citations/19930087913>
- **R4.** Jackson et al., NACA TN 3320 (1954): pp. 1, 3–6, Figs. 4, 7. <https://ntrs.nasa.gov/citations/19930084086>
- **R5.** Ferris, NASA TN D-4013 (1967): p. 4, Fig. 1(a) p. 9. <https://ntrs.nasa.gov/citations/19670020050>
- **R6.** Stoney, NASA TR R-100 (1961): Fig. 12(a). <https://ntrs.nasa.gov/citations/19630004995>
- **R7.** Piland and Putland, NACA RM L54A27 (1954): pp. 1–3. <https://ntrs.nasa.gov/citations/19930089303>
- **R8.** Hart and Katz, NACA RM L9I30 (1949). <https://ntrs.nasa.gov/citations/20050019251>
- **R9.** Hoerner, *Fluid-Dynamic Drag* (1965), p. 16-25. <https://archive.org/details/FluidDynamicDragHoerner1965>
- **R10.** MIL-HDBK-762 (1990): pp. 5-45, 5-46, Table 5-4 p. 5-61, p. 5-76, Fig. 5-113 p. 5-179.
  <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets>
