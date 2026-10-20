# How a thin fin's leading-edge drag changes from Mach 0.5 to 0.9

**In short: a round nose on a thin plate goes supercritical near Mach 0.5–0.7, so at Mach 0.8 its drag is not zero,
and a square nose's doubt lies in its level, not in its rise with Mach number.** This note follows
[fin-edge-drag](fin-edge-drag.md), which took a round leading edge as 0 at Mach 0.8 and scaled a square one's
low-speed 0.35–0.45 by stagnation pressure. A semicircular nose on a flat plate peaks at C_p −1.54 in potential
flow (critical Mach 0.51), and a test found its flow supersonic from Mach 0.7. That test's momentum loss, read as
drag, puts an unswept round edge near 0.23–0.36 at Mach 0.8, against the rule's 0.53. For a square edge, the
stagnation-pressure scaling is about right; Hoerner's own base-drag rule suggests the low-speed level on a thin fin
is nearer 0.55–0.75 than 0.4, while the one free-flight test found a blunt fin nose's drag negligible. No test of a
thin square-nosed plate at Mach 0.6–0.9 was found, and the one round-nosed plate's thickness is unknown. "Inference"
marks this note's own arithmetic; every size at Mach 0.8 rests on one test read by eye, or on a rule past its data.

## 1. The rule, and what fin-edge-drag assumed

- **The rule.** `fin_pressure_drag_coefficient` in `crates/hpr-aero/src/drag.rs` charges a square leading edge
  0.85 q_stag/q, and a rounded or airfoil one eq. 3.89, (1 − M²)^−0.417 − 1, a separated cylinder's forebody
  [B5] p. 49. Both are taken on the edge's frontal area, times cos² Γ_L, and evaluated at free-stream Mach.
- **Eq. 3.89's values (Inference):** 0.040 at Mach 0.3, 0.127 at 0.5, 0.205 at 0.6, 0.324 at 0.7, 0.531 at 0.8,
  0.707 at 0.85 and 0.999 just below 0.9.
- **fin-edge-drag's Mach 0.8 sizes.** Square: 0.35–0.45 from Hoerner's Fig. 22, times 1.14 for q_stag/q from Mach
  0.3 to 0.8, called "an assumption, not a source". Round: 0, as for an attached nose below its critical Mach
  number. On its invented rocket that made the round-nosed (airfoil) rule high by 7.2% of C_D at Mach 0.8.

## 2. Square leading edge: the Mach factor holds; the level is the doubt

- **The face rises with stagnation pressure.** Hoerner splits sharp-edged bodies into a face part that scales with
  q_stag/q ≈ 1 + M²/4 and a base part held fixed below the critical Mach number: for a two-dimensional normal plate
  C_D = 1.13 + 0.85 (1 + 0.25 M²) [A1] pp. 15-3, 15-4, eqs. 9–10. Lindsey's square cylinders (c/t = 1, Reynolds
  number of order 3–6 × 10⁴, scatter ±2%) read about 2.02 at Mach 0.3, 2.05–2.09 from 0.4 to 0.65 and 2.00–2.05 at
  0.75 [A2] pp. 170, 173, Fig. 9 (by eye); eq. 9 gives 2.00, 2.06 and 2.10, within 3% (Inference). The base carries more
  than half that drag, so the data fit the model but don't isolate the face.
- **The face, measured.** Hoerner takes a flat-faced cylinder's face as 0.90 q_stag/q from pressure tests [A1]
  p. 16-13, eq. 20. A two-dimensional square edge carries 1.5 at Mach 1.4 and 1.4 at 1.8 [A1] p. 16-17, Fig. 22,
  0.85–1.0 of the pitot pressure (Inference). Nothing found has a flat face rising faster than q_stag/q below 0.75.
- **The 0.4 is a net figure (Inference).** Fig. 22 of Hoerner's chapter 3 marks a gap of about 0.4 between blunt
  and rounded rectangular sections [A1] p. 3-12. The BARC 5:1 rectangle (C_D 0.94–1.05 [A3] pp. 17–18) has a front
  face near C_p 1 except at the corners, and a rear face near −0.2 to −0.3 [A3] p. 21, Fig. 5 (by eye): a face
  drag near 0.7–0.8 and a base drag lowered by about 0.2. Hoerner's base rule, C_DB = 0.135/∛C_fB with the face
  counted as forebody drag [A1] pp. 3-19 to 3-21, eqs. 33, 39, reproduces Fig. 22's blunt level of 0.9 at c/t = 6
  with a face of 0.7, and a gap of 0.49 (0.40–0.58 for faces of 0.6–0.8).
- **On a thin fin the base cut nearly vanishes (Inference).** At c/t = 49 (2% thick) friction alone gives
  C_fB 0.43, and the face moves base drag only from 0.18 to 0.13. The net leading-edge term is then 0.55–0.75, at
  c/t = 20 0.51–0.70. fin-edge-drag's 0.35–0.45 fits stubby sections; on a thin fin it may read low by 0.1–0.4.
- **Mach moves the two parts differently (Inference).** Thin-section base drag rises ×1.5 (Eggers' wedge) to
  about ×3 (Nash at Mach 0.8 over Hoerner at low speed) by Mach 0.8 (fin-edge-drag §3). If the cut scales with it,
  the net term on a 2% fin rises ×0.99–1.12 from Mach 0.3 to 0.8, not ×1.14: 0.55–0.84 at 0.8. Against the doubt
  in the level, the factor's error is small.
- **But a flight test found no such term (§6).** The level is open between 0.4 and about 0.75.

## 3. Square edge above Mach 0.65: a sonic corner, effect unbounded

- **When (Inference).** Behind each corner of the BARC rectangle the side pressure sits on a plateau of about
  −0.75 to −1.05 at low speed [A3] p. 21, Fig. 5(a). The critical pressure coefficient is −1.01 at Mach 0.65 and
  −0.78 at 0.70; compressibility deepens the plateau, so a square-nosed plate should carry a supersonic pocket over
  its leading bubble from about Mach 0.65.
- **What it does is not measured.** Hoerner traces drag dips near Mach 0.6–0.8 to such pockets pulling separated
  flow back in; a plate normal to the flow shows none [A1] p. 15-7, and a long canopy that reattached dipped above
  about Mach 0.6 [A1] p. 15-8, Fig. 8. A dip lowers the edge term; a shock on the bubble raises it. Neither its
  sign nor its size is bounded for a thin plate.

## 4. Round leading edge: sonic near Mach 0.5–0.7

- **Potential flow (Inference) [B1].** Source panels on a flat plate of thickness t with the same nose at both
  ends, chord 25–100 t. The code returns a lone circle's peak speed as 2.000 U (exact: 2) and the NACA 0012's
  C_p,min as −0.414. A semicircular nose peaks at C_p −1.54, 75° round the nose, 0.13 t ahead of the junction with
  the flat; chords of 25, 50 and 100 t give −1.552, −1.536 and −1.528, so t/c of 1–4% hardly matters.
- **Critical Mach number:** where the compressible peak reaches sonic, the supervelocity route Hoerner gives [B4]
  p. 15-15, eqs. 20–21. Kármán–Tsien (KT) is used here; Prandtl–Glauert reads higher, Laitone lower.

| Nose (length over half-thickness) | C_p,min, incompressible | M_crit, KT (Laitone–PG) |
|-----------------------------------|------------------------:|------------------------:|
| semicircle on a plate (1:1)       | −1.54                   | 0.51 (0.48–0.53)        |
| 2:1 ellipse on a plate            | −0.76                   | 0.63 (0.61–0.65)        |
| 3:1 ellipse on a plate            | −0.51                   | 0.70 (0.68–0.71)        |
| 4:1 ellipse on a plate            | −0.38                   | 0.74 (0.72–0.75)        |
| NACA 0012 (check)                 | −0.41                   | 0.73 (0.71–0.74)        |

- **Tests.** At low speed (Reynolds number 3,000 on half-thickness) a circular nose on a flat plate peaked at
  C_p −1.0, its flow separated, as Goodhand reports Walraevens and Cumpsty [B2] p. 5; with −1.0, KT gives 0.585
  (Inference). At high speed, Tain and Cumpsty found a circular nose on a flat plate supersonic from Mach 0.7 at
  zero incidence, peaking near Mach 1.7 at 0.95 [B3] (abstract). So onset lies at Mach 0.5–0.7: near 0.6–0.7 with
  a laminar bubble, nearer 0.51 if the nose flow stays attached (Inference). Either way, Mach 0.8 is supercritical.
  The 0012's 0.73 fits Eggers' c_d, flat to Mach 0.75 (fin-edge-drag [E7]).

## 5. Round edge: what it adds once supercritical

- **The data.** Goodhand replots Tain and Cumpsty's momentum thickness θ over nose radius r, measured where the
  bubble has reattached [B2] p. 13, Fig. 2-8. Read by eye (±0.005): 0.097 at Mach 0.5, 0.120 at 0.6, 0.157 at
  0.7, 0.195 at 0.75, 0.214 at 0.8, 0.238 at 0.85, 0.202 at 0.88, 0.166 at 0.9. Up to 0.85 the suction spike
  grows; above it the shock moves aft of the spike after some isentropic recompression, and θ falls [B2] pp. 12–13.
- **As drag (Inference).** Both faces' momentum deficit is drag, so on the frontal area 2r, Δc_d = 2Δ(θ/r). Edge
  speed and density at the station are ignored, an unsized error. The Mach 0.5 level mixes friction with bubble
  loss, so only the rise is used, with the nose term at Mach 0.5 bracketed from 0 to the rule's 0.127.

| Mach | test's rise over Mach 0.5 | rule, eq. 3.89 | rule's rise over Mach 0.5 | test's rise ÷ rule's |
|-----:|--------------------------:|---------------:|--------------------------:|---------------------:|
| 0.6  | 0.05                      | 0.205          | 0.077                     | 0.6                  |
| 0.7  | 0.12                      | 0.324          | 0.197                     | 0.6                  |
| 0.75 | 0.20                      | 0.412          | 0.284                     | 0.7                  |
| 0.8  | 0.23                      | 0.531          | 0.404                     | 0.6                  |
| 0.85 | 0.28                      | 0.707          | 0.579                     | 0.5                  |
| 0.9  | 0.14                      | 0.999          | 0.871                     | 0.2                  |

- **Sign and size.** Up to Mach 0.85 the test rises 0.5–0.7 as fast as the rule; at 0.9, a fifth. At 0.8 an
  unswept round edge is 0.23–0.36, the rule high by 0.17–0.30 on frontal area, not by its whole 0.53.
- **Why so mild.** Hoerner notes that a sharp suction peak recompresses through weak or no shocks; in his examples
  drag divergence came 0.3–0.4 in Mach number above the critical value [B4] pp. 15-18, 15-19, Figs. 24–25. The
  nose spike is such a peak (Inference).
- **Above Mach 1 the rule's level fits.** A two-dimensional semicircular nose carries about 1.0 at Mach 1.4–1.8
  [B4] p. 16-17, Fig. 22; eq. 3.89's supersonic branch gives 0.99 at 1.4 and 1.07 at 1.8 (Inference).
- **The Arcas fins don't test this.** Their sharp double wedges (fin-edge-drag §5) have no nose spike; their fin
  increment can pin only the sharp case, and these sizes should not be fitted to it.

## 6. Sweep, and a free-flight check

- **The rule takes free-stream Mach.** On a swept panel pressures follow the speed normal to the edge, and the
  Prandtl factor belongs to M cos Λ; a 12% wing swept 45° moved drag divergence from 0.70 to almost 0.90 [B4]
  pp. 15-20 to 15-22, eqs. 27, 31, 36, Fig. 30. The rule multiplies by cos² Γ_L but evaluates at M. The
  cross-flow form is cos² Γ_L × c_d(M cos Γ_L) (Inference); at 45° and Mach 0.8 that is Mach 0.57, just above a
  semicircle's 0.51. For a square face the change is small: q_stag/q at 0.57 instead of 0.8.
- **Stoney and Royall's bomb shapes.** Helium-gun models, Mach 0.6–1.1, C_D on body frontal area to ±0.01 [A4]
  pp. 1, 3. Configurations b and c differ only at the fin's leading edge, sharp or blunt; both have sharp trailing
  edges [A4] p. 7, Table II. The authors call the blunt nose's effect "negligible" (summary) and "small" [A4] p. 4;
  by eye b and c lie within about 0.01 of each other below Mach 0.9 [A4] p. 11, Fig. 3(a).
- **What the rules predict there (Inference).** Four fins 0.16 in thick, about 1.3 in exposed span, leading edge
  swept about 46°, on a 2.598 in body [A4] p. 8, Fig. 1(a), p. 10, Fig. 2(a). With cos² sweep and the edge as
  thick as the fin, a square net term of 0.4 adds about 0.034 at Mach 0.8, and 0.6–0.8 adds 0.05–0.07; eq. 3.89
  at free-stream Mach adds 0.04, at normal Mach about 0.013. All but the last exceed what was seen.
- **Caveats.** The blunt nose is drawn, not dimensioned, on a section tapering to a 0.07 in trailing edge marked
  for configuration a (Fig. 1(a), "not to scale"); if the nose is that thin, every prediction above shrinks about
  ×0.44. The fins are about 8% thick and two sit at 2° incidence. It weighs against a large term; it sets no level.

## 7. Sized again on fin-edge-drag's invented rocket

fin-edge-drag §6's rocket: three 3.2 mm fins, leading edge swept 45°, fin frontal over reference area 0.127.
"Measured" takes its trailing edges as they were and the leading edges from §2 and §5 here (Inference).

| Edges | Mach | rule, on frontal | measured, on frontal | rule minus measured, % of C_D | fin-edge-drag had |
|-------|-----:|-----------------:|---------------------:|------------------------------:|------------------:|
| square | 0.3 | 0.566 | 0.375–0.565 | +0.0 to +5.4 | +4.3 to +8.3 |
| square | 0.8 | 0.701 | 0.565–0.810 | −2.6 to +3.3 | +1.3 to +5.1 |
| rounded | 0.8 | 0.367 | 0.115–0.381 | −0.4 to +6.6 | +4.4 to +7.1 |
| airfoil (round nose) | 0.8 | 0.266 | 0.015–0.181 | +2.3 to +6.8 | +7.2 |

- In the round rows, measured runs from the rise at normal Mach (0.57) with no nose term at Mach 0.5, to the rise
  at free-stream Mach plus the rule's 0.127. With the square nose at 0.4, fin-edge-drag's square rows stand.
- **Sign (Inference).** The round-nosed rule still reads high at Mach 0.8, by less. The square-nosed rule may read
  about right on a thin fin: at Mach 0.8 its row straddles zero, and correcting it could lower apogee by up to
  about 1.5% (fin-edge-drag's −0.31% to −0.57% per 1% of C_D). At Mach 0.3 fin-edge-drag's §7 sign stands.

## What this suggests

- **Round and elliptic noses (core band).** Evaluate eq. 3.89 at M cos Γ_L, hold it near 0 below §4's critical
  Mach (0.51 semicircle, 0.63 for 2:1, 0.70 for 3:1), and let it rise at about 0.6 of the rule's rise to Mach
  0.85. Expected on the invented rocket at Mach 0.8: −0.024 to −0.032 in C_D. Keep sharp-nosed fins separate.
- **Square noses.** [#614](https://github.com/nrdptel/fusionspace-eridanus/issues/614)'s fin-edge sensitivity run
  uses 0.4 q_stag/q; a second value near 0.65 would bracket §2's level. Keep the q_stag/q factor.

## What the project can use

- **Cached, for reading and citing only:** Goodhand's thesis (All Rights Reserved, Cambridge repository), Graham's
  R&M 3465 (Crown copyright), Hoerner (copyright L. A. Hoerner), the BARC preprint (no license stated). **US
  government works, cached:** Lindsey's NACA Report 619 and Stoney and Royall's RM L56D16.

## Open questions

- No test of a thin (c/t ≥ 10) square-nosed plate at Mach 0.6–0.9 was found, nor a round-nosed one of known
  thickness; such tests would settle §2's level, §3's sign and §5's size.
- Tain and Cumpsty's paper is paywalled: its Reynolds number, measuring station and plate size are unknown, so §5
  carries a Reynolds-number factor this note can't size; its abstract says a higher Reynolds number thinned the
  layer. A 3.2 mm fin at Mach 0.8 has about 6 × 10⁴ on thickness; Walraevens and Cumpsty were at 6,000.
- That Hoerner's eq. 39 holds for a face far ahead of a long chord, at C_fB near 1, is assumed, not shown.
- Graham's R&M 3465 (round blunting of a biconvex section, Mach 0.7–1.4; its summary finds the drag change small,
  sometimes a reduction) is cached but not reduced; Sheppard
  (Aeronautical Quarterly, 1960) and Nakagawa's transonic square cylinder were seen as abstracts only.

## Sources

All read 2026-10-19. Printed page numbers.

1. **A1, B4.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
2. **A2.** W. F. Lindsey, *Drag of Cylinders of Simple Shapes*, NACA Report 619, 1938.
   <https://ntrs.nasa.gov/citations/19930091694>
3. **A3.** L. Bruno, M. V. Salvetti and F. Ricciardelli, *Benchmark on the Aerodynamics of a Rectangular 5:1
   Cylinder*, J. Wind Eng. Ind. Aerodyn., 2014, preprint.
   <https://www.aniv-iawe.org/new_site/wp-content/uploads/2020/05/BARC_overview_JWEIA_2014_preprint.pdf>
4. **A4.** W. E. Stoney, Jr. and J. F. Royall, *Zero-Lift Drag of a Series of Bomb Shapes at Mach Numbers from 0.60
   to 1.10*, NACA RM L56D16, 1956. <https://ntrs.nasa.gov/citations/19930089367>
5. **A5.** W. J. Graham, ARC R&M 3465, 1965 (published 1967), blunt leading edges at transonic speeds; not reduced.
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4044>
6. **B1.** This note's panel and critical-Mach scripts (Inference), not committed.
7. **B2.** M. N. Goodhand, *Compressor Leading Edges*, PhD dissertation, University of Cambridge, 2010.
   <https://www.repository.cam.ac.uk/handle/1810/247860>
8. **B3.** L. Tain and N. A. Cumpsty, *Compressor blade leading edges in subsonic compressible flow*, Proc. IMechE
   Part C 214(1), 2000, pp. 221–242; abstract only. <https://doi.org/10.1243/0954406001522921>
9. **B5.** S. Niskanen, *Development of an Open Source model rocket simulation software*, MSc thesis, TKK, 2009.
   <https://openrocket.sourceforge.net/thesis.pdf>
10. **B7.** R. E. Walraevens and N. A. Cumpsty, *Leading edge separation bubbles on turbomachine blades*, ASME J.
    Turbomachinery 117, 1995, pp. 115–125; read only as [B2] reports it.
