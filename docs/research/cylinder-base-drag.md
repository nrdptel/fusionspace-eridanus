# Base drag of a blunt cylinder from Mach 0.3 to 1.2

**In short: from Mach 0.7 to 1.0 the shared base-drag rule reads high against every sting-free measurement found, by
0.03 to 0.10 on the base area; from Mach 1.05 to 1.25 it is within ±0.025 of them.** HPR Sim and OpenRocket both use
Niskanen's eq. 3.94 after Fleeman, 0.12 + 0.13 M² below Mach 1 and 0.25/M above. Three measured sets carry no sting: a
finless body flown free (Hart 1952), the same body with fins (Hart and Peck), and a tunnel cylinder held from upstream
with a stated turbulent boundary layer, δ/d = 0.20 (Cubbage 1956). All three stay nearly flat from Mach 0.6 to 0.9, at
0.12–0.16, then rise steeply to a peak of 0.215–0.240 at Mach 1.02–1.12, while the rule climbs steadily to its peak at
Mach 1.0: high by 18–46% of the finless value at Mach 0.7–0.9, and 57–82% of the finned one. Fins lowered the subsonic
value by 0.030–0.039, about Hart's error bound. Below Mach 0.6 no sting-free measurement was found. Above Mach 1.2 the
measured plateau, 0.20–0.22, sits above Love's correlation, so a join to Love belongs near Mach 1.3–1.5. Too much base
drag lowers the simulated apogee of flat-based rockets that fly through Mach 0.8–1.0; the change is not sized here.
Every curve value was read off a plot; "Inference" marks this note's own arithmetic.

## 1. The rule and what each program applies it to

- **The formula.** Eq. 3.94 gives the base drag on the base's own area, citing Fleeman's *Tactical Missile Design*
  [B1] p. 50. It has no input for forebody length, boundary layer, fins near the base or a cavity in it. It is
  continuous at 0.25 at Mach 1, with a corner there.
- **HPR Sim** (`base_drag_coefficient` in `crates/hpr-aero/src/drag.rs`) applies it to the aft base less the thrusting
  motors' cross-section, as Niskanen proposes [B1] p. 51.
- **OpenRocket** documents the same formula and the same subtraction [B1] pp. 50–51. The project's probe of its
  release found it keeps the whole base while a motor burns (`docs/physics/aero.md`, "Base drag under power").
  Coasting, the two agree; this note is about the coasting, power-off value.
- **Values (Inference):** 0.132 at Mach 0.3 and 0.153 at 0.5; the rest are the *rule* column of §2's table.

## 2. What was measured

Each set below is a flat base behind a cylinder, power off, with its coefficient on the base area. "Read" values come
off the source's plot; the read-off error is given with each.

| Set | Source | Body and support | Boundary layer | Fins | Pressure measured |
|:--|:--|:--|:--|:--|:--|
| HA | [B2] Fig. 8, p. 16 | l/d 11, free flight | partly laminar | none | rim and center, each on its area |
| HC | [B2] Fig. 8, p. 16 | HA's body, free flight | as HA | 4, t/d 0.018 | rim and center, each on its area |
| HB | [B2] Fig. 6, p. 15 | HA on a 0.6 d sting | as HA | none | rim only |
| PE | [B3] Fig. 12, p. 18 | l/d 11.1, free flight | not stated | 4, square TE | edge, mean of configs A, B, C |
| CU | [B4] Fig. 11(a), p. 36 | 2 in tube, no sting | turbulent, δ/d 0.20 | none | one rim orifice |

- **HA, a finless body flown free.** Hart's fuselage is an ogive (3.5 calibers) on a cylinder, 5.00 in across, flown
  coasting after a booster at Reynolds numbers of 15–45 × 10⁶ on length [B2] pp. 2–3, Table I p. 9. The booster pushed
  through a thrust tube that withdrew at separation, so the base was hollow; Hart measured the rim and the inside of
  the body [B2] pp. 3–4. **Inference:** the opening is about 0.6 d, as the tube was made to resemble the 3.00 in sting
  [B2] p. 4, Fig. 1(e) p. 12. Hart judges the subsonic flow partly laminar from the low fore drag [B2] p. 6; Love
  calls the layer turbulent ahead of the base [B5] p. 7. Hart's error bound is ±0.030 in the base-pressure coefficient
  at Mach 0.8 and ±0.013 at 1.25 [B2] p. 5.
- **HC, the same body with fins.** Four fins, 45° leading-edge sweep, root 8.38 in, tip 1.38 in, 0.091 in thick with
  rounded leading edges and a square section [B2] p. 3. **Inference:** their trailing edges end 0.24 in (0.05 d) ahead
  of the base, Fig. 1(c) p. 11. Hart takes these data from Peck's flights [B2] p. 3.
- **PE, Peck's finned bodies.** Wood, polished lacquer, four 0.09 in duralumin fins with square trailing edges, a
  closed flat base on configuration A [B3] p. 2. The base pressure oscillated strongly between Mach 0.7 and 1.2. Two
  identical models differed by up to 0.06 at subsonic speed [B3] p. 3. PE is the solid curve labeled as the edge
  average; the short-dash curve above it is configuration A's center orifice, about 0.03 higher subsonic.
- **CU, Cubbage's tunnel cylinder.** The model sat on a 2 in tube cantilevered from the tunnel entrance, so no sting
  trails it [B4] p. 4. The boundary layer was fully turbulent, a 1/7-power profile, about 20% of the base diameter
  thick; Cubbage calls that thicker than an aircraft's [B4] p. 7. The base is an annulus around a nozzle exit 0.65 of
  its diameter, read here with no jet flow [B4] Fig. 11(a), p. 36, and one orifice 0.055 in from the edge [B4] p. 5.
  Wall interference was not corrected [B4] p. 7; at Mach 1.0 the base pressure "may be excessively low" [B4] p. 6, so
  the drag may read high there.

Base drag on base area by Mach number (positive is drag; the rule's error is rule minus measured):

| Mach | rule | HA | rule − HA | HC | rule − HC | CU | rule − CU | PE edge | HA rim | HB rim (sting) |
|-----:|-----:|---:|----------:|---:|----------:|---:|----------:|--------:|-------:|---------------:|
| 0.6 | 0.167 |  |  |  |  | 0.117 | +0.050 |  |  |  |
| 0.7 | 0.184 | 0.156 | +0.028 | 0.117 | +0.067 | 0.120 | +0.064 | 0.12 | 0.177 |  |
| 0.8 | 0.203 | 0.155 | +0.048 | 0.117 | +0.086 | 0.127 | +0.076 | 0.12 | 0.172 | 0.097 |
| 0.9 | 0.225 | 0.154 | +0.071 | 0.124 | +0.101 | 0.141 | +0.084 | 0.125 | 0.172 | 0.104 |
| 0.95 | 0.237 | 0.170 | +0.067 | 0.144 | +0.093 |  |  |  | 0.186 | 0.108 |
| 1.0 | 0.250 | 0.221 | +0.029 |  |  | 0.173 | +0.077 |  | 0.235 | 0.141 |
| 1.05 | 0.238 | 0.240 | −0.002 | 0.214 | +0.024 |  |  | 0.225 | 0.258 | 0.19 |
| 1.1 | 0.227 | 0.225 | +0.002 | 0.207 | +0.020 | 0.222 | +0.005 | 0.216 | 0.240 | 0.226 |
| 1.2 | 0.208 | 0.222 | −0.014 |  |  | 0.209 | −0.001 |  | 0.235 | 0.234 |
| 1.25 | 0.200 | 0.222 | −0.022 |  |  | 0.199 | +0.001 |  | 0.235 | 0.234 |

How these were read: HA and HC by pixel measurement against the plot's grid, ±0.004; the rim columns ±0.006; CU from a
300 dpi scan by eye against the grid, ±0.006; PE by eye, ±0.01, left blank on its steep rise. On the steep rise (Mach
0.95–1.05) Hart's Mach error, ±0.01 [B2] p. 4, moves a value by up to ±0.01. HA rim stands at 0.258 at Mach 1.04.

- **Where the peak sits.** Measured peaks lie at Mach 1.02–1.05 in flight (HA 0.240, PE about 0.235, HC about 0.215)
  and near 1.12 in the tunnel (CU, 0.224). The rule's peak is 0.250 at exactly 1.0. Hoerner's compilation shows the
  same shape: a modest subsonic rise, a peak near Mach 1, then a roughly level "transonic" phase [B6] p. 16-4, Fig. 2.
- **Below Mach 0.6.** No sting-free flat-base cylinder measurement was found. Hoerner's Fig. 2 holds three subsonic
  levels for plain cylindrical bodies: about 0.056 (a Johns Hopkins flight with fins, Mach 0.5–0.6), 0.156 (Hart's
  body, drawn from about Mach 0.58) and 0.23 (a smooth projectile), read ±0.01, plus Peck's point near 0.11 at Mach
  0.9 [B6] p. 16-4. MIL-HDBK-762's Fig. 5-140 holds points from 0.061 to 0.129 at Mach 0.3 and 0.080 to 0.125 at 0.5,
  and 0.157–0.164 for its 1 atm points at Mach 0.7–0.9, read ±0.003 [B7] p. 5-209. The handbook calls values below
  Mach 1.2 "not very well defined" and blames hard-to-measure small pressures [B7] p. 5-51. Neither source says how
  those models were held. **Inference:** the rule's 0.132 at Mach 0.3 tops the handbook's points (0.129, within the
  read error); its 0.153 at 0.5 matches Hart's level.
- **Sting effect (HB against HA).** A rear sting of 0.6 d took about 40% off the subsonic suction and nothing above
  Mach 1.15 [B2] pp. 1, 5. Love calls this transonic interference large and Hart's the only sting-free data without
  fins then available [B5] pp. 6–7. Hoerner's sting data, Mach 1.5–4.5, show the largest effect at the low end of that
  range [B6] p. 16-10, Fig. 10. The project's Arcas Robin tunnel data (Mach 0.6–4.63) are forebody only, their bases
  on a sting, so no project reference measures base drag past Mach 0.3 ([aero](../physics/aero.md), #68).

## 3. Boundary-layer thickness

- **Sign: a thicker layer means less base drag.** Hoerner's subsonic correlation is C_DB = 0.029/√C_fB, C_fB being the
  forebody's friction drag on the base area [B6] p. 3-19, eq. 34, Fig. 37. Doubling the forebody's friction takes 29%
  off the base drag. A low-speed test on a blunt-based body (Re 5.5 × 10⁵) found thicker layers reduced base suction
  [B9] (abstract only; values not read).
- **Size in this Mach range.** CU, with δ/d = 0.20, sits 0.013–0.036 below the thinner-layered HA from Mach 0.7 to 0.9
  (Inference, the table); at Mach 1.1 they agree within 0.003. The two also differ in support, cavity and tunnel, so
  this bounds the effect only loosely.
- **Supersonic, for the join.** At Mach 1.5–1.8 with forced turbulence, Hoerner's Fig. 4 falls from about 0.19 at l/d
  4 to 0.15 at 8 and 0.10 at 20 (read ±0.01) [B6] p. 16-5. In Love's tripped cone-cylinder tests Reynolds number had a
  small effect; he notes artificial transition gives base pressures 5–10% higher than natural [B8] p. 15. Cubbage
  cites a Mach 2.0 test where δ/d from 0.05 to 0.18 did not change the base pressure much [B4] p. 7.
- **Hoerner's correlations don't fit near Mach 1. Inference:** HA's forebody has S_wet/S_B = 39 (Table I); at a
  flat-plate turbulent C_f of 0.0024–0.0028 at its Reynolds numbers (Prandtl–Schlichting, 0.455/(log Re)^2.58), eq. 34
  gives 0.088–0.095 against 0.156 measured at Mach 0.7, though Hoerner expects only a modest subsonic rise with Mach
  [B6] p. 16-4. CU's layer, taken as a 1/7-power profile (C_fB = 8θ/d, θ = 7δ/72), gives C_fB 0.16 and 0.074 against
  0.117–0.141 measured.
- **HPR rockets. Inference:** the textbook flat-plate turbulent layer, δ = 0.37 x Re_x^(−1/5), gives δ/d near 0.22 for
  l/d 15 at Re_L 10⁷, close to CU's 0.20. HA's would be about 0.14 if fully turbulent. Paint thickens it.

## 4. Fins near the base

- **Subsonic: lower, by 0.030–0.039 (19–25%) at Mach 0.7–0.9; transonic: by 0.018 (8%) at 1.1** (HC against HA, the
  table). Hart attributes it to slow air from the fin–body junctions bleeding into the base region [B2] p. 5.
- **Not firmly established.** Hart's error bound at Mach 0.8 is ±0.030 [B2] p. 5. Peck's two identical finned models
  differ by up to 0.06 [B3] p. 3. The 0.030–0.039 is about Hart's bound on each value and inside Peck's spread. PE's
  edge average (0.12 subsonic) agrees with HC, but Hart took HC from Peck's flights, so it is no check.
- **Other sizes.** Hoerner puts fin effects at about ±10% on straight bodies, larger on boattailed ones; on the RM-10
  missile's fins (supersonic) suction rose from 0.09 to 0.13 [B6] p. 16-11. Supersonic sizes are in
  [junction-drag](junction-drag.md) §5. The rule has no fin term.

## 5. The rule against the measurements

- **Mach 0.3–0.6: no sting-free test.** The rule tops most compiled points, by up to 0.07 at Mach 0.3, and matches
  Hart's level at 0.5 (§2). With supports unknown and a sting worth 40%, neither sign is established.
- **Mach 0.7–1.0: the rule reads high.** Against the finless flight (HA): +0.028 at 0.7, +0.048 at 0.8, +0.071 at 0.9,
  +0.067 at 0.95, +0.029 at 1.0 (+13% to +46%). Against the thick-layer tunnel (CU): +0.050 to +0.084 from 0.6 to 0.9.
  Against the finned flight (HC): +0.067 to +0.101. The cause is shape, not level: the rule climbs with M² from 0.6
  while the data stay flat to 0.9 and then jump.
- **Mach 1.05–1.25: within ±0.025.** HA: −0.022 to +0.002; CU: −0.001 to +0.005; HC: +0.020 to +0.024.
- **Past Mach 1.2 the rule turns low.** At Mach 1.28 HA is 0.219 against the rule's 0.195 (−11%). Issue #68's reading
  of Love's curve, 0.184 at Mach 1.1, is lower still than HA, CU and PE there. **Inference:** the measured plateau
  holds near 0.20–0.22 through Mach 1.3, so a join to Love's correlation belongs at about Mach 1.3–1.5, not 1.2.
- **On C_D. Inference:** where the base is the reference area, the rule adds 0.05–0.10 to C_D at Mach 0.8–0.95 over
  the sting-free sets. The apogee change depends on time spent there; it is not sized here.

## What the project can use

- **Terms.** The NACA reports (Hart, Peck, Cubbage, Love, Cahn) and MIL-HDBK-762 (Distribution A) are US government
  works, cached under `refs/papers/` (gitignored); values may go into a fixture with figure and page. Hoerner is
  copyrighted: cite, don't copy. Mariotti and Buresti: abstract only. Niskanen's thesis is CC BY-NC-ND 1.0 Finland per
  `THIRD-PARTY-NOTICES.md` (the PDF states none); methods only.
- **Reference to add.** A fixture of the read-off curves HA, HC, CU and PE (Mach 0.6–1.28), each with figure, page,
  read-off error, body, support and boundary layer, and a test that prints the rule's signed error at each Mach.
- **Candidate rule (suggestion).** Hold 0.13 ± 0.02 from rest to Mach 0.9 (the rule's own low-speed value and the
  CU–HA band), rise to 0.23 ± 0.01 at Mach 1.05, hold 0.20–0.22 to Mach 1.3, then fade to Love's correlation by Mach
  1.5 (#68). Expected effect (Inference): about −0.04 at Mach 0.6, −0.095 at 0.9, −0.01 at 1.05 and +0.02 to +0.03 at
  1.3 on the base area, so flat-based rockets that fly through Mach 0.8–1.0 reach higher. That runs against the
  collection's high apogee bias ([fin-edge-drag](fin-edge-drag.md) §7): ship it with the bias re-measured.
- **Bounded task for #68 (core band).** Build the fixture above. *Done when* the base-drag rule (old or replaced) is
  within ±0.03 of HA at every tabled Mach from 0.7 to 1.25 and within ±0.03 of CU at 0.7–0.9, or an ADR records the
  measured gap. Today it fails: +0.071 at Mach 0.9 against HA.
- **[M1.14c](../decisions-and-roadmap.md#m1-14c), the transonic accuracy increment.** The fixture gives it its first
  sting-free base reference between Mach 0.6 and 1.3.

## Open questions

- No sting-free flat-base cylinder was found between Mach 0.3 and 0.6, nor any with a stated δ/d below Mach 0.6.
- Hart's open base and Cubbage's nozzle opening are cavities; how much each raises the base pressure is unsized here.
  HPR rockets have motor-tube cavities too, so a closed-base reference would separate the two.
- Brazzel's report behind MIL-HDBK-762's Fig. 5-140 (models, supports), and its 0.186–0.29 scatter at Mach 1.0 (#68).
- A DTIC survey with a "Moore base drag method" (ADA127378) and DATCOM 4.2.3 could not be reached (DTIC under
  maintenance on 2026-10-17). AGARD base-flow reviews were not searched.
- Cahn's sting tests (NACA TR 1353, cached) use boattailed afterbodies at Mach 0.8–1.1 and were not reduced.
- Fin trailing edges flush with the base, as most HPR fins sit, were not tested in any set here.

## Sources

All read 2026-10-17. Printed page numbers.

1. **B1.** S. Niskanen, *Development of an Open Source model rocket simulation software*, MSc thesis, TKK, 2009.
   <https://openrocket.sourceforge.net/thesis.pdf>
2. **B2.** R. G. Hart, *Effects of Stabilizing Fins and a Rear-Support Sting on the Base Pressures of a Body of
   Revolution in Free Flight at Mach Numbers from 0.7 to 1.3*, NACA RM L52E06, 1952.
   <https://ntrs.nasa.gov/citations/19930087048>
3. **B3.** R. F. Peck, *Flight Measurements of Base Pressure on Bodies of Revolution With and Without Simulated Rocket
   Chambers*, NACA TN 3372, 1955 (supersedes RM L50I28a). <https://ntrs.nasa.gov/citations/19930084618>
4. **B4.** J. M. Cubbage, Jr., *Jet Effects on Base and Afterbody Pressures of a Cylindrical Afterbody at Transonic
   Speeds*, NACA RM L56C21, 1956. <https://ntrs.nasa.gov/citations/19930089150>
5. **B5.** E. S. Love, *A Summary of Information on Support Interference at Transonic and Supersonic Speeds*, NACA RM
   L53K12, 1954. <https://ntrs.nasa.gov/citations/19930088012>
6. **B6.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
7. **B7.** MIL-HDBK-762(MI), *Design of Aerodynamically Stabilized Free Rockets*, 1990.
   <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets>
8. **B8.** E. S. Love, *Base Pressure at Supersonic Speeds on Two-Dimensional Airfoils and on Bodies of Revolution
   With and Without Fins Having Turbulent Boundary Layers*, NACA TN 3819, 1957.
   <https://ntrs.nasa.gov/citations/19930084517>
9. **B9.** A. Mariotti and G. Buresti, *Experimental investigation on the influence of boundary layer thickness on the
   base pressure and near-wake flow features of an axisymmetric blunt-based body*, Exp. Fluids 54, 1612, 2013
   (abstract only). <https://doi.org/10.1007/s00348-013-1612-5>
10. **B10.** M. S. Cahn, *An Experimental Investigation of Sting-Support Effects on Drag and a Comparison with Jet
    Effects at Transonic Speeds*, NACA TR 1353, 1958. <https://ntrs.nasa.gov/citations/19930091015>
11. **B11.** Issue #68 (Fleeman against Love). <https://github.com/nrdptel/fusionspace-eridanus/issues/68>
