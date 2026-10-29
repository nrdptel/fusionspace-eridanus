# How an open base or a motor-tube cavity changes base pressure, Mach 0.3 to 1.3

**In short: a cavity in a blunt base lowers its base drag by 0 to 0.02 on the base area from Mach 0.3 to 1.3, about
the scatter of each measured set.** A cavity as wide as the base gains 0.004–0.012 in base pressure at Mach 0.3–0.9,
and the gain peaks at about 0.15 d deep, not deeper (Compton 1968, a tunnel at flight Reynolds numbers). A shallow
full-width cavity flown at Mach 0.70–0.93 changed base drag by 0.01 at most, which Powers (1988) calls not
significant. Low-speed tunnels read up to 7 times larger and two-dimensional bases about 15 times, so neither sizes a
rocket's correction. The two sting-free open-base sets behind [cylinder-base-drag](cylinder-base-drag.md) read *low*
for a closed base: Hart's hollow body by 0.015–0.019, and Cubbage's jet-off nozzles by an amount bounded only to
±0.02. The one flown closed-against-open pair, Peck's dummy rocket chamber, nets 0.000 to −0.011, inside its 0.06
model-to-model scatter. Closing Hart's base cuts the base rule's high error at Mach 0.9 from +0.071 to about +0.053;
it still fails a ±0.03 bound, so [#68](https://github.com/nrdptel/fusionspace-eridanus/issues/68) stands. No test
was found that varies cavity diameter at a fixed depth. Every curve value was read off a plot; "Inference" marks
this note's own arithmetic.

## 1. Terms and the HPR cases

- **Base pressure coefficient** C_pb = (p_base − p_∞)/q, negative behind a blunt base. **Base drag** on the base's own
  area is C_D,base = −C_pb when one pressure acts on the whole base. A positive ΔC_pb (less suction) lowers it.
- **A cavity** is a recess in the base: depth L, diameter d_c, on a body of diameter d. Here *shallow* is L < 0.5 d
  and *deep* is several d.
- **HPR shapes.** A coasting rocket's open end is usually a spent motor: a nozzle exit 0.21–0.71 of the case
  ([nozzle exits](nozzle-exits.md)) in front of an empty case, a small opening into a deep chamber. A recessed
  retainer or a motor tube ending short of the base is shallow and partial. An airframe running past its aft
  centering ring is shallow and nearly full width.

## 2. A full-width cavity on a cylinder, Mach 0.3–0.9

- **Compton 1968 [S1].** A 6.00 in cone-cylinder nacelle on a swept strut in the Langley 16-ft transonic tunnel,
  transition fixed near the nose, Reynolds number 8–16 × 10⁶ on length [S1] p. 1. The turbulent layer is 1.52 cm,
  δ/d about 0.1 [S1] p. 7. The cylindrical afterbody (configurations 15–18, [S1] Fig. 5 p. 20) carried a flat
  base, concave recesses 0.125 d and 0.25 d deep (cavity radius 7.29 cm in a base of radius 7.62 cm, d_c/d 0.96),
  and an open bore of radius 7.37 cm (d_c/d 0.97) running deep into the model.
- **Against depth, Mach 0.8.** [S1] Fig. 16 p. 40, read off ±0.002: ΔC_pb ≈ +0.008 at 0.125–0.15 d, +0.005 at
  0.25 d, +0.004 for the open bore. The gain peaks shallow and the deep bore keeps half of it.
- **Against Mach.** [S1] Fig. 15(b) p. 39: the flat base's C_D,base is about 0.13 at Mach 0.3, 0.14 at 0.7–0.8 and
  0.155 at 0.9; the recessed and open bases sit 0.005–0.012 below it. Repeat points on the flat base differ by up
  to about 0.01 at Mach 0.4 and 0.6, so the gain is about one repeat-scatter wide.
- **Boattails.** On 3°–10° boattails the gain is 0.01–0.03 and needs deeper cavities [S1] pp. 1, 9. Past some depth
  there is "little or no effect" [S1] pp. 1, 9, 11. Rims were not measured [S1] p. 5.
- **Powers 1988 [S2], flown.** An 8 in body of revolution, fineness 10.9, flown as the cap of an F-111's vertical
  fin. Solid-wall and vented-wall extensions 2.8 in deep (0.35 d) at Mach 0.70, 0.80 and 0.93 changed C_D,base by
  0 to −0.01 [S2] p. 4, Fig. 9 p. 12, read by eye ±0.005; Powers calls 0.01 not significant. The airplane's angle of
  attack was 3.9°–6.6° [S2] p. 4. This is the HPR shape of an airframe overhanging its aft ring.

## 3. Low-speed and two-dimensional tests read larger

- **Low-speed axisymmetric.** Goodyer (1966), as Compton reports it: a thin-walled tube of body diameter about 1 d
  long, ΔC_pb ≈ +0.02 [S1] p. 9. Mair (1978), incompressible as Powers reports it [S2] p. 2, for Powers's geometry:
  −0.03 (solid wall) and −0.07 (vented) in base drag [S2] p. 4. Morel (1979), abstract only, speed not stated
  [S3]: six depths 0.1–0.9 d, all shallow cavities cut drag, the largest −0.027 (vented, 0.2 d); vented cavities
  past 0.5 d gave "a very large drag increase". Morel puts axisymmetric gains an order of magnitude below 2D ones,
  at much shallower depth [S3].
- **A square-back bluff body.** Evrard et al. 2016 [S4], an Ahmed body at Re 4 × 10⁵, depth 0–0.35 of the base
  height: base suction −18%, drag −9% at best [S4] abstract. Its introduction claims 10–20% for 2D and axisymmetric
  bodies, saturating past 0.25 of the height [S4] p. 2, which disagrees with §2.
- **Two-dimensional.** Nash et al. 1963 [S5], a blunt trailing edge in a slotted tunnel with a cavity 1 h deep
  ([S5] §2.2). [S5] Fig. 47 p. 60, read off: plain C_pb −0.61 to −0.74 at Mach 0.2–0.8, the cavity −0.47 to −0.57,
  a gain of +0.14 to +0.17 (22–25%); about 20% subsonic and none supersonic, from a weaker vortex street [S5] p. 23.
- **Reading (Inference).** Low-speed axisymmetric gains, 0.02–0.07, are 1.7–7 times §2's ≤ 0.012. A 2D base's suction
  is 4–5 times an axisymmetric one, and its gain about 15 times. Compton argues axisymmetric bases shed less
  regularly [S1] p. 11. Only §2's data should size a rocket's correction.

## 4. Peck 1955: the one closed-against-open pair, flown

- **Bodies [S6].** A 5.0 in wooden ogive-cylinder, l/d 11.1, four 0.09 in fins with square trailing edges [S6] p. 2.
  A has a closed flat base. B has a dummy rocket chamber with its nozzle exit flush with the base. C has the same
  chamber with its exit 1.2 in aft, in a 3.5 in sleeve (C's exit 3.0 in). D, boattailed, is left out here.
- **B's cavity, read off [S6] Fig. 1 pp. 7–8.** A convergent-divergent nozzle: exit 2.9 in (0.58 d, 0.336 of the
  base area), throat 1.5 in (0.30 d); the chamber's front bulkhead lies 13.5 in (2.7 d) ahead of the base.
- **Orifices.** A's center orifice is on the axis of the base. On B and C it is "on the inside of the front
  bulkhead" of the chamber, the cavity floor; the edge orifice sits about 3/8 in in from the rim [S6] p. 2.
- **Scatter.** Records oscillated strongly at Mach 0.7–1.2 and the curves are faired through them [S6] p. 3. Two
  identical models differ by up to 0.06 in Δp/q [S6] pp. 3–4.

Suction, −Δp/q (positive is drag). Edges from [S6] Fig. 11 p. 17, centers from Fig. 12 p. 18, pixel-read ±0.004:

| Mach | A edge | B edge | C edge | A center | B floor | C floor |
|-----:|-------:|-------:|-------:|---------:|--------:|--------:|
| 0.80 | 0.107 | 0.127 | 0.125 | 0.145 | 0.105 | 0.074 |
| 0.88 | 0.113 | 0.131 | 0.128 | 0.144 | 0.105 | 0.074 |
| 1.08 | 0.217 | 0.224 | 0.217 | 0.234 | 0.198 | 0.183 |
| 1.12 | 0.212 | 0.212 | 0.212 | 0.223 | 0.191 | 0.176 |

The peaks sit at Mach 1.01–1.04: A edge 0.223, B edge 0.243, A center 0.243, B floor 0.203.

- **The floor (Inference, the table).** B's floor reads 0.040 less suction than A's closed center below Mach 1 and at
  the peak, 0.032–0.036 at Mach 1.08–1.12. C's reads 0.070 less below Mach 1, 0.051 at 1.08 and 0.047 at 1.12. Peck: the
  chamber turns the center from more suction than the edge into less [S6] pp. 1, 4–5.
- **The edge.** B's edge reads more suction than A's: 0.020 below Mach 1 and at the peak, 0.007 at 1.08, 0 at 1.12.
  Peck calls the edge changes "not consistent" and smaller than the center's [S6] p. 4.
- **Base drag (Inference).** Put the center value on the 2.9 in circle and the edge value on the rest, for A and B
  alike: A 0.120 and B 0.120 at Mach 0.8, 0.123 and 0.122 at 0.88, both 0.230 at their peaks, 0.223 and 0.215 at
  1.08, 0.216 and 0.205 at 1.12. B − A is 0.000 to −0.011. With A on its edge orifice alone, the practice Peck set
  out to check [S6] p. 1, B − A is +0.013 at Mach 0.8 and −0.007 at 1.12: no settled sign. C − A is −0.012 to
  −0.017, but C also changes the outer shape.
- **Assumed:** the floor pressure holds through the chamber, so the cavity's net force is that pressure times the
  exit area. Nothing was measured on the divergent wall.

## 5. The two sting-free open-base sets

- **Hart 1952, set HA [S7].** A steel thrust tube ran from the booster through the balsa shell to the nose and
  withdrew at separation, leaving the body hollow [S7] pp. 3–4. The tube was dressed to resemble a 3.00 in sting
  [S7] p. 4, Fig. 1(e) p. 12; the hole's size is not stated. **Inference:** about 0.6 d, up to 7.5 d deep (the
  37.6 in cylinder, Table I). One orifice is on the rim, one inside the hollow body [S7] p. 4.
- **Readings.** [S7] Fig. 7 p. 16, finless body, pixel-read ±0.004:

| Mach | 0.7 | 0.8 | 0.9 | 1.04 (peak) | 1.17–1.27 |
|:--|--:|--:|--:|--:|--:|
| rim C_pb | −0.176 | −0.174 | −0.171 | −0.258 | −0.235 to −0.237 |
| inside C_pb | −0.121 | −0.122 | −0.127 | −0.209 | −0.197 to −0.198 |
| inside − rim | +0.055 | +0.052 | +0.044 | +0.049 | +0.038 |

- **HA is already area-weighted.** Hart's Fig. 8 puts "rim- and center-base pressures … over the entire respective
  areas" [S7] p. 16. **Inference:** fitting HA (cylinder-base-drag §2) as (1 − s)·rim + s·inside gives the inside's
  share s = 0.37 at Mach 0.8, 0.37 at 1.05, 0.38 at 1.2; a 3.00 in hole in a 5.00 in base gives 0.36. The finned
  body C reads −0.122 to −0.125 rim (−0.134 at 0.9) and −0.105 to −0.111 inside at Mach 0.7–0.9.
- **HA closed (Inference).** The rim alone sits 0.015–0.019 above HA: 0.019 at Mach 0.8, 0.018 at 1.05, 0.015 at
  1.2. That is how much closing the hole would raise HA if a closed base read like its rim. Peck's closed center
  reads more suction than its edge (0.031–0.038 below Mach 1, 0.017 at 1.08, §4), so the rise could be a little larger.
  Hart's error bound is the same size: ±0.030 at Mach 0.8, ±0.013 at 1.25 [S7] p. 5.
- **Cubbage 1956, set CU [S8].** A 2 in tube cantilevered from the tunnel entrance, no sting [S8] p. 4. Sonic nozzle
  exits are 0.25, 0.45, 0.65 and 0.75 of the base, supersonic ones 0.75 and 0.85 [S8] Fig. 3(a) p. 18, pp. 4–5;
  each opens back into the tube that ducts the jet air [S8] p. 4. **Inference:** jet off, each is a cavity many
  diameters deep. One base orifice sits 0.055 in from the outer edge [S8] p. 5; nothing was measured inside the
  nozzle and there is no closed base.
- **CU against exit size.** Jet-off annulus C_pb for exits of 0.65, 0.75 and 0.85, from [S8] Figs. 7(a) p. 27,
  7(b) p. 28 and 11(d) p. 39, pixel-read ±0.005: −0.117 to −0.122 at Mach 0.6, −0.140 to −0.143 at 0.9, −0.217 to
  −0.221 at 1.1, −0.198 to −0.200 at 1.25. The annulus moves 0.005 or less with exit size, about the read error.
  Fig. 7(c) shows one jet-off point a Mach for the 0.25 and 0.45 nozzles that matches 0.65; the legend doesn't say
  whether it was measured on them. At Mach 1.0 the base pressure "may be excessively low" from wall-reflected
  expansions [S8] p. 6.
- **CU closed (Inference).** CU spreads its rim value over the whole base. A cavity floor reading less suction than
  the rim (§4, HA) would put CU's true value below its rim; a closed center reading more suction (§4) would put a
  closed base above it. Bounded to about ±0.02, sign open.

## 6. What it does to the base rule (Inference)

- **#68's error against a closed HA.** The rule reads 0.225 at Mach 0.9 against HA 0.154 and HA rim 0.172
  (cylinder-base-drag §2): +0.071 for the open base, +0.053 rim-only. At Mach 0.8, +0.048 and +0.029 to +0.031
  (this note's rim read or that note's). Past Mach 1.05 the rule's small low error against HA grows by the same
  0.015–0.019: −0.035 rim-only at Mach 1.25. The ±0.03 done-when proposed in cylinder-base-drag still fails at
  Mach 0.9–0.95 and 1.25, and is marginal at 0.8.
- **Sizes by HPR shape.** An airframe past its aft ring, full width and shallow: 0–0.012 [S1, S2]. A shallow
  recessed retainer or motor-tube lip (d_c/d 0.3–0.8): no test; assumed no larger than the full-width 0.012. A
  spent motor, a small opening into a deep case: the opening's area share times 0.03–0.05 at the floor, partly
  offset at the rim by Peck's 0.00–0.02, so 0 to 0.02.
- **Sign.** Every axisymmetric source agrees that a cavity lowers base drag [S1–S3, S6, S7], except Morel's vented
  cavities deeper than 0.5 d [S3]. Near Mach 1 the size is inside every set's scatter, so no cavity term is sized.

## What this suggests

- **#68's fixture.** Tag HA and CU "open base". Carry a closed-base allowance of 0 to +0.02 on HA (Mach 0.7–1.27,
  §5) and ±0.02 on CU, and add Peck A's area-weighted closed values (§4) as a finned closed-base check with its 0.06
  scatter. The rule's verdict holds with either reading.
- **No cavity term in the base rule.** Measured effects on axisymmetric bodies at flight Reynolds numbers are below
  0.02 and below the scatter; a term would rest on low-speed or 2D data that read up to 15 times larger.
- **Users' designs.** A motor-tube cavity need not enter the design input for base drag; a note in the aero page can
  cite §6's 0–0.02.

## Open questions

- No subsonic or transonic test varies cavity diameter at a fixed depth, or depth at a fixed small opening; none maps
  pressure across a cavity floor or a closed base.
- Whether Cubbage's 0.25 and 0.45 nozzles ran jet off.
- Not reached: BRL-MR-3662 (DTIC ADA196206; flat-cavity against dome base on a shell in a transonic range; DTIC
  under maintenance on 2026-10-29); Viswanath 1988 (J. Aircraft 25(3), paywalled, no abstract); Morel's full text;
  Kruiswyk and Dutton 1990 and Molezzi and Dutton 1995 (2D); an AIP 2019 paper on cylinder cavities at Mach 0.6–1.5
  (doi 10.1063/1.5120195), paywalled, its listings disagreeing on the authors.
- Mair 1978 and Goodyer 1966 are known only through [S1] and [S2].
- Evrard et al.'s introduction (p. 2) was read once in a browser; a second pass could not reach the text.
- Hoerner's *Fluid-Dynamic Drag* ch. 3: a text search for "cavit", "hollow" and "recess" found no base-cavity data.
- Fins ending at a cavity base, angle of attack, and Reynolds numbers between 4 × 10⁵ and 10⁷ are unbounded.

## Sources

All read 2026-10-29. Printed page numbers. NACA and NASA reports are US government works, cached under `refs/papers/`
(gitignored); values may go into a fixture with figure and page. Nash et al. is UK Crown copyright: cite, don't
reproduce. Morel and Evrard et al.: cite only.

1. **S1.** W. B. Compton III, *Effect on Base Drag of Recessing the Bases of Conical Afterbodies at Subsonic and
   Transonic Speeds*, NASA TN D-4821, 1968. <https://ntrs.nasa.gov/citations/19680027582>
2. **S2.** S. G. Powers, *Flight Tests of External Modifications Used to Reduce Blunt Base Drag*, NASA TM-100433
   (AIAA 88-2553), 1988. <https://ntrs.nasa.gov/citations/19880010895>
3. **S3.** T. Morel, *Effect of base cavities on the aerodynamic drag of an axisymmetric cylinder*, Aeronautical
   Quarterly 30(2), 400–412, 1979 (abstract only). <https://doi.org/10.1017/S0001925900008611>
4. **S4.** Evrard, Cadot, Herbert, Ricot, Vigneron and Délery, *Fluid force and symmetry breaking modes of a 3D
   bluff body with a base cavity*, J. Fluids Struct. 61, 99–114, 2016, doi 10.1016/j.jfluidstructs.2015.12.001.
   <https://hal.science/hal-01249010>
5. **S5.** Nash, Quincey and Callinan, *Experiments on Two-Dimensional Base Flow at Subsonic and Transonic Speeds*,
   ARC R&M 3427, 1963 (published 1966). <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4011>
6. **S6.** R. F. Peck, *Flight Measurements of Base Pressure on Bodies of Revolution With and Without Simulated
   Rocket Chambers*, NACA TN 3372, 1955. <https://ntrs.nasa.gov/citations/19930084618>
7. **S7.** R. G. Hart, *Effects of Stabilizing Fins and a Rear-Support Sting on the Base Pressures of a Body of
   Revolution in Free Flight at Mach Numbers from 0.7 to 1.3*, NACA RM L52E06, 1952.
   <https://ntrs.nasa.gov/citations/19930087048>
8. **S8.** J. M. Cubbage, Jr., *Jet Effects on Base and Afterbody Pressures of a Cylindrical Afterbody at Transonic
   Speeds*, NACA RM L56C21, 1956. <https://ntrs.nasa.gov/citations/19930089150>
