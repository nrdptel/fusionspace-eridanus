# How much drag a fin's edges make below Mach 1

**In short: HPR Sim's fin-edge rule reads high, not low, for most fins below Mach 1.** HPR Sim and OpenRocket share one
rule (Niskanen 2009 eq. 3.89–3.93). It gives a square leading edge a blunt cylinder's face drag, 0.87 at Mach 0.3 on the
edge's frontal area; two-dimensional tests put it near 0.4. It gives a square trailing edge a rocket base's 0.13 at Mach
0.3 and 0.20 at 0.8; thin blunt edges measure 0.10–0.19 at low speed, but 0.29–0.39 at Mach 0.8. On an invented 98 mm
three-fin rocket (fins swept 45°), square edges read high by 4.3–8.3% of the whole C_D at Mach 0.3 and 1.3–5.1% at 0.8,
where the trailing edge's shortfall partly cancels. Rounded and airfoil leading edges are charged 0.53 at Mach 0.8, a
separated cylinder's forebody; if the edge stays subcritical the true term is near zero, and the rule is high by up to
7.2% of C_D. On the Arcas tunnel models that term is 2–3 times, at Mach 0.8, the drag it should explain. Rounded edges
at low speed land within ±1.2% of C_D. So the edges push the simulated apogee *down*, against the collection's +8.73%
bias: correcting them would widen it by roughly 1–4%. The sign is sure for square edges at low speed; the sizes rest on
figures read by eye and on the invented rocket. "Inference" marks this note's own arithmetic.

## 1. The rule, and where it came from

- **What HPR Sim computes.** `fin_pressure_drag_coefficient` in `crates/hpr-aero/src/drag.rs` follows Niskanen
  [E1] pp. 49–50 term for term. The coefficient is taken on the fins' frontal area N·t·s (count, thickness, span).
  The leading edge's share is multiplied by cos² Γ_L, Γ_L being the leading-edge sweep. OpenRocket's technical
  documentation v13.05 carries the same section unchanged [E2] pp. 49–50.
- **Square edge:** leading edge = a blunt circular cylinder's face, 0.85 q_stag/q ([E1] eq. B.2, p. 107); trailing
  edge = the body's base drag, 0.12 + 0.13 M² ([E1] eq. 3.94, p. 50).
- **Rounded edge:** leading edge = eq. 3.89, (1 − M²)^−0.417 − 1 below Mach 0.9; trailing edge = half the base drag.
- **Airfoil:** eq. 3.89 at the leading edge and no trailing-edge drag. Niskanen's airfoil has a rounded nose
  [E1] p. 49; HPR Sim gives the same term to a sharp-nosed fin filed as an airfoil, as on the Arcas models (§5).
- **Barrowman's original differs.** His eq. 4-17 is a circular cylinder's total drag less its base drag, read off a
  cylinder chart: the forebody of a cylinder whose rear has separated [E3] p. 50. His trailing edge was Hoerner's
  two-dimensional base drag, 0.135 divided by the cube root of the forebody friction on the base area, made larger
  with Mach number [E3] pp. 50, 53. Niskanen kept the first and replaced the second by the rocket-base formula. That
  dropped the edge's dependence on thickness against chord.

| Mach | 0.85 q_stag/q (square LE) | eq. 3.89 (round LE) | base drag (square TE) |
|-----:|--------------------------:|--------------------:|----------------------:|
| 0.3 | 0.869 | 0.040 | 0.132 |
| 0.8 | 0.995 | 0.531 | 0.203 |

(Inference: the formulas evaluated; all on the edge's frontal area.)

## 2. Square leading edge: the rule is about twice the measured value

- **Two-dimensional sections.** Hoerner's figure of "rectangular" sections between tunnel walls shows two curves
  against chord over thickness c/t [E4] p. 3-12, Fig. 22. With a blunt leading edge the drag on the thickness falls
  to about 0.9 once c/t passes about 4. With a rounded nose it falls to about 0.5, mostly base drag. The figure marks
  the difference at about 0.4. The separated flow reattaches behind the face once c/t is about 2.5 [E4] p. 3-13.
- **A modern check of the total.** In four drag tests a 5:1 rectangular cylinder has C_D 0.94–1.05 on its depth, at
  Reynolds numbers 2 × 10⁴ to 2 × 10⁶ [E8] pp. 17–18: Hoerner's square curve, but not split into front and base.
- **Why the rule is high. Inference:** 0.85 is the face of a three-dimensional blunt cylinder (Hoerner gives 0.8 for
  a flat-faced cylinder's forebody, [E4] p. 3-12, Fig. 20). On a long flat face the suction in the bubble behind each
  corner pays back part of the face pressure. The two-dimensional value, 0.4 (0.35–0.45 allowing for reading), is
  0.40–0.52 of the rule's 0.87 at Mach 0.3.
- **Mach number.** No subsonic measurement of a square-nosed thin plate against Mach was found. This note scales the
  0.35–0.45 by the rule's own stagnation ratio, ×1.14 from Mach 0.3 to 0.8: an assumption, not a source.

## 3. Square trailing edge: about right at low speed, about half the measured value near Mach 0.8

- **Low speed.** Hoerner correlates two-dimensional base drag with the forebody friction referred to the base area,
  C_fB = C_f S_wet/S_B, which is 2 C_f c/t for a flat fin [E4] p. 3-19, eq. 33. Airfoils with cut-off or thickened
  trailing edges fit C_DB = 0.135/∛C_fB [E4] p. 3-21, eq. 39 (data in Fig. 39, p. 3-20). A sheet-metal step on
  a wall, with no vortex street, gives 0.1/∛C_fB (eq. 38). The cut-off sections of Fig. 40 (NACA 0012, 0018, 2415
  at 7 × 10⁵ to 6 × 10⁶) move from a 0.10 curve toward the 0.14 one as h/c grows; only the smooth 0012 sits on or
  above 0.14 [E4] p. 3-21.
- **On an HPR fin. Inference:** a 3.2 mm fin with a 156 mm mean chord, turbulent at 1.1 × 10⁶, has C_fB 0.43. Eq. 39
  gives 0.18; Fig. 40's points near h/c 1.3–2% imply about 0.10–0.19 on the base, read by eye over each series' uncut
  drag. The rule's 0.13 sits inside that range. A vortex street is expected: the edge is about as thick as the 3–4 mm
  boundary layer; Hoerner finds none only below about half the layer's thickness [E4] p. 3-21.
- **Mach 0.6 and 0.8.** Nash's compilation of base pressure against trailing-edge thickness over chord h/c holds
  most of the thin-section data [E5] p. 22, Fig. 4 (thickened 3% and 4% sections, zero incidence). Read by eye at
  h/c ≈ 2%, base drag is 0.25–0.38 at Mach 0.6 and 0.29–0.39 at 0.8; at 10% it is 0.66 and 0.74. Below Mach 0.85 the
  curves for different h/c "are well spread out" [E5] p. 7: thickness against chord matters, which the rule ignores.
  The rule's 0.17 and 0.20 are 0.44–0.70 of these.
- **The Mach trend agrees.** Eggers' 12%-thick single wedge, a blunt trailing edge at 0.5–1 × 10⁶, went from c_d
  0.089 at Mach 0.3 to 0.133 at 0.8 [E7] pp. 3, 14, Fig. 4. Less a friction near 0.008, that is ×1.5 on the base
  drag; the rule's 0.12 + 0.13 M² rises ×1.54 (Inference). The level is what the rule gets wrong for thin fins.
- **Thick edges, for scale.** Base drag of sections with thick blunt edges stays near 0.5–0.7 from Mach 0.5 to 0.85
  and peaks near 0.8 at Mach 1 [E4] p. 16-11, Fig. 11. Nash quotes a base-pressure coefficient of −0.625 at Mach 0.4
  for h/c = 0.10 [E5] p. 20.

## 4. Rounded edges

- **Leading edge, low speed: near zero.** Hoerner's rounded sections settle at base drag plus friction, with nothing
  left for the nose [E4] p. 3-12, Fig. 22. The rule's 0.040 at Mach 0.3 is as small; either way it barely counts.
- **Leading edge, Mach 0.8: the rule is a separated cylinder's.** Eq. 3.89 charges 0.53. An attached round nose that
  stays subcritical has none: the normal NACA 0012, round-nosed, held c_d near 0.0075 to Mach 0.75 [E7] p. 14, Fig. 4.
  A 2%-thick plate with a semicircular nose has a curvature jump where nose meets flat, and a local supersonic pocket
  there at Mach 0.8 is possible. No test of such a plate at Mach 0.6–0.9 was found, so the true term lies between
  about 0 and something unbounded; this note sizes the subcritical case.
- **Trailing edge: no direct test found.** A rounded edge still separates, with the separation points free to move
  [E5] p. 4. The nearest test is Eggers' reversed NACA 0012, a small round trailing edge: it added 0.011–0.014 to c_d
  over the normal section from Mach 0.3 to 0.7, flat with Mach [E7] p. 14, Fig. 4. Inference: 0.09–0.12 on the
  section's thickness. Estes TR-11's illustrative 1/8 in fins give rounded sections about half a square one's
  pressure drag, from a tunnel test it describes as imagined and does not source [E9] PDF pp. 21–22. This note takes
  0.05–0.12 at Mach 0.3 (half the square's, up to the reversed 0012's) and 0.10–0.20 at 0.8 for the rounded edge.

## 5. A measured check: the Arcas fins at Mach 0.6 and 0.8

The Arcas tunnel models carried double-wedge fins, sharp at the nose [E10]. HPR Sim files them as "airfoil", so they
take eq. 3.89. From the committed fixture (`validation/fixtures/aero/drag-vs-mach.json`, short model, fins on less
fins off), HPR Sim's fins carry 0.036 friction and 0.049 edge drag at Mach 0.6, and 0.035 and 0.126 at Mach 0.8.
Measured fin increments are 0.077 and 0.099 ([junction-drag](junction-drag.md) §4). Inference:

- What friction leaves is 0.041 at Mach 0.6 and 0.064 at 0.8 against HPR Sim's own friction, or 0.018 and 0.042 against
  friction on the fin's chord. The edge term is 1.2–2.7 times that at Mach 0.6 and 2.0–3.0 times at 0.8.
- Something real does grow with Mach on these fins (wave drag over the wedges, the boattail under them), so the term
  is not simply zero. But at Mach 0.8 it is the main reason HPR Sim's fin increment reads 63% high.

## 6. Sized on an invented three-fin rocket

**The rocket (invented):** 98 mm body, 4:1 tangent ogive (392 mm), 1,200 mm of tube, 1,592 mm long; three 3.2 mm fins,
200 mm root, 100 mm tip, 100 mm span, leading edge swept 100 mm (Γ_L = 45°). Mean chord 155.6 mm, t/c̄ 0.021; fin
frontal area over reference area 0.127. Sea level, 20 µm roughness, coasting with the whole base. **Inference:** the
buildup is Niskanen's eqs. 3.78–3.85 and 3.94 by hand, without nose pressure, lugs or buttons. Its C_D is 0.449 at
Mach 0.3 and 0.524 at 0.8 with square fins (0.388 and 0.482 rounded, 0.380 and 0.469 airfoil). The square-edge term
is 16.0% and 17.0% of it; rounded, 2.8% and 9.7%; airfoil, 0.7% and 7.2%.

"Measured" below combines §2–4: square leading edge 0.35–0.45 (×1.14 at Mach 0.8) times cos² Γ_L; square trailing edge
0.10–0.19 and 0.29–0.39; rounded trailing edge as in §4; round or sharp noses 0 (subcritical). Apogee uses
[apogee-bias](apogee-bias.md)'s −0.31% to −0.57% per 1% of C_D, as if the whole flight sat at that Mach number.

| Edges | Mach | rule, on frontal | measured, on frontal | rule minus measured, C_D | % of C_D | apogee from the rule |
|-------|-----:|-----------------:|---------------------:|-------------------------:|---------:|---------------------:|
| square | 0.3 | 0.566 | 0.275–0.415 | +0.019 to +0.037 | +4.3 to +8.3 | −1.3% to −4.7% |
| square | 0.8 | 0.701 | 0.490–0.647 | +0.007 to +0.027 | +1.3 to +5.1 | −0.4% to −2.9% |
| rounded | 0.3 | 0.086 | 0.050–0.120 | −0.004 to +0.005 | −1.1 to +1.2 | +0.6% to −0.7% |
| rounded | 0.8 | 0.367 | 0.100–0.200 | +0.021 to +0.034 | +4.4 to +7.1 | −1.4% to −4.0% |
| airfoil | 0.3 | 0.020 | about 0 | +0.003 | +0.7 | −0.2% to −0.4% |
| airfoil | 0.8 | 0.266 | about 0 | +0.034 | +7.2 | −2.2% to −4.1% |

- **Sweep moves the square case.** Unswept, the rule's square edges are 1.001 against 0.45–0.64 at Mach 0.3: high by
  9.1–13.9% of C_D (Inference). The leading-edge error scales with cos² Γ_L; the trailing-edge error doesn't.
- **The switch from square to rounded.** NC State's OpenRocket C_D fell from 0.55 to 0.45 when only the fin edges
  changed ([flight-derived-cd](flight-derived-cd.md) §4). On this rocket the rule's switch is 0.061 of C_D at Mach
  0.3; the measured values give 0.020–0.046 (Inference). The setting is real, but the rule overstates it.

## 7. What this does to the collection's bias

- **The collection's fins (aggregates only).** The 210 distinct OpenRocket files (by SHA-256, 2026-10-16) in the
  private collection's flights, imported and leads folders hold 279 fin sets by their cross-section field: 134 square,
  76 rounded and 69 airfoil. These are all its design files, not only its 53 scored flights.
- **Sign. Inference:** about half the fin sets are square, where the rule reads high at every subsonic speed; the
  airfoil and rounded ones read high from about Mach 0.6. Correcting the edges would *raise* HPR Sim's apogees,
  roughly 1–4% on a subsonic flight with square fins and less with rounded ones: the +8.73% bias widens.
- **So it moves [junction-drag](junction-drag.md)'s conclusion further.** Neither the junction nor the edges supply
  drag the simulators miss. As-built mass and rail buttons ([apogee-bias](apogee-bias.md)) move up.

## What the project can use

- **Cache, under the gitignored `refs/research/fin-edge-drag/`:** Eggers' RM A7C10 (NTRS: public use permitted). For
  reading only: Nash's R&M 3323, 3427 and 3468 (HMSO Crown copyright, 1963 and 1966; no license at Cranfield), the
  BARC preprint (none stated), Hoerner (1992 printing, copyright L. A. Hoerner) and Estes TR-11: cite only. Niskanen's
  thesis is Creative Commons no-derivatives and the technical documentation share-alike, by the latter's own words
  (versions in `THIRD-PARTY-NOTICES.md`); Barrowman and TN D-4013 are US government works.
- **Suggestion 1, trailing edge (core band, subsonic).** Use Barrowman's own term, 0.135/∛C_fB with the fin's
  C_fB = 2 C_f c̄/t, for square edges, and half for rounded, times a Mach factor fitted to Nash Fig. 4 (thin sections)
  and Eggers' wedge. Expected at Mach 0.3 on the invented rocket: +0.05 on frontal area, +1.4% of C_D with
  square fins; +0.02, +0.8% with rounded ones. Its larger effect is near Mach 0.8 and on thick, short-chord fins.
- **Suggestion 2, square leading edge.** Replace 0.85 q_stag/q below Mach 0.8 with a two-dimensional value near
  0.4 q_stag/q, citing Hoerner Fig. 22, flagged as an extrapolation from c/t below about 8. Expected: −0.06 on C_D
  unswept, −0.03 at 45° sweep, Mach 0.3.
- **Suggestion 3, round and sharp noses.** Keep eq. 3.89 only above a critical Mach number for the edge, or pin it to
  the Arcas fin increment first, as [junction-drag](junction-drag.md) proposes. Any change must not widen that gap.
- **Do 1 and 2 together.** At Mach 0.3 and 45° sweep, 2 lowers square fins' C_D about 6.5% and 1 raises it 1.4%;
  together about 5% down, and the collection's bias up. Near Mach 0.8 the two cancel more.

## Open questions

- No subsonic test of a thin square-nosed plate against Mach number, nor of a thin plate with a semicircular nose at
  Mach 0.6–0.9, was found. Mach 0.8 for those edges rests on an assumed scaling (square) or on subcritical flow (round).
- Hoerner's Fig. 22 data are old, low-Reynolds tunnel tests between walls; their Reynolds numbers are not given.
- A fin's trailing edge flush with the body's base shares the base's dead air; the root runs in the body's thick
  boundary layer, which raises C_fB and lowers base drag. Neither is sized here.
- Nash, Quincey and Callinan's R&M 3427 (cached, not reduced) measured an isolated blunt edge at subsonic and
  transonic speeds; Tanner's base-drag papers and NACA TN 3548 and TN 2074 were not read.
- How the 279 fin sets split among the 53 scored flights, and their mean thickness over chord.

## Sources

All read 2026-10-16. Printed page numbers unless "PDF".

1. **E1.** S. Niskanen, *Development of an Open Source model rocket simulation software*, MSc thesis, TKK, 2009.
   <https://openrocket.sourceforge.net/thesis.pdf>
2. **E2.** S. Niskanen, *OpenRocket technical documentation*, v13.05, 2013.
   <https://openrocket.sourceforge.net/techdoc.pdf>
3. **E3.** J. S. Barrowman, *The Practical Calculation of the Aerodynamic Characteristics of Slender Finned Vehicles*,
   MS thesis, 1967 (NASA/TM-2001-209983). <https://ntrs.nasa.gov/api/citations/20010047838/downloads/20010047838.pdf>
4. **E4.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
5. **E5.** J. F. Nash, *A Review of Research on Two-Dimensional Base Flow*, ARC R&M 3323, 1962 (published 1963).
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/3903>
6. **E6.** J. F. Nash, V. G. Quincey and J. Callinan, *Experiments on Two-Dimensional Base Flow at Subsonic and
   Transonic Speeds*, ARC R&M 3427, January 1963 (published 1966; cached, not read).
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4011>
7. **E7.** A. J. Eggers, Jr., *Aerodynamic characteristics at subcritical and supercritical Mach numbers of two airfoil
   sections having sharp leading edges and extreme rearward positions of maximum thickness*, NACA RM A7C10, 1947.
   <https://ntrs.nasa.gov/citations/19930085638>
8. **E8.** L. Bruno, M. V. Salvetti and F. Ricciardelli, *Benchmark on the Aerodynamics of a Rectangular 5:1
   Cylinder: an overview after the first four years of activity*, J. Wind Eng. Ind. Aerodyn., 2014, preprint.
   <https://www.aniv-iawe.org/new_site/wp-content/uploads/2020/05/BARC_overview_JWEIA_2014_preprint.pdf>
9. **E9.** G. M. Gregorek, *Aerodynamic Drag of Model Rockets*, Estes TR-11, 1970; PDF pages.
   <https://www.plans.rocketshoppe.com/pubs/Estes/estTR-11/TR-11.pdf>
10. **E10.** J. C. Ferris, NASA TN D-4013, 1967, as reduced in the project's fixture and in
    [junction-drag](junction-drag.md). <https://ntrs.nasa.gov/citations/19670020050>
