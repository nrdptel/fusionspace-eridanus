# Supersonic wave drag of thin and blunt-edged fins

**In short: Ackeret's linear theory fits measured sharp-edged fins within −15% to +9% once friction is added. On the
Arcas Robin it removes 108% to 167% of HPR Sim's excess fin drag.** Faster than sound, HPR Sim gives every fin a blunt
leading edge: the Arcas Robin's 4.4%-thick double-wedge fins carry 0.24 to 0.28 of pressure drag; theory gives 0.010 to
0.041. The fins-on rows would go from +39.4%…+154.0% to −2.0%…+8.7% (short model, Mach 1.5 to 4.63), but only by
cancellation: the fin increment would read 35% to 52% *low*, the fins-off body reads 13.6% to 24.1% high, and 0.020 to
0.080 stays unexplained. Square trailing edges read *low*: 0.25/M against 1.6 to 2.0 times that measured on thin blunt
edges, Mach 1.5 to 2.5; on Calisto at Mach 2 that alone moves −14.9% to −13.2% against RASAero II. The round edge
matches modified Newtonian theory within 1%, but no thin plate with one was found tested. Trust the theory for sharp,
thin, symmetric sections with attached shocks; blunt-edge sizes rest on two-dimensional tests read by eye. Drag that
reads high reads apogee low. "Inference" marks this note's own arithmetic.

## 1. What HPR Sim does faster than sound

- **The rule.** `fin_pressure_drag_coefficient` (`crates/hpr-aero/src/drag.rs`) gives the fins' frontal area N·t·s a
  leading-edge term times cos² Λ (Λ, Γ_L in the code, is the leading-edge sweep) plus a trailing-edge term, after
  Niskanen [10] eq. 3.89–3.94. Below Mach 1 the rule and its errors are in [fin-edge-drag](fin-edge-drag.md).
- **Square:** leading edge 0.85 C_p,stag (stagnation pressure rise over dynamic pressure, q_stag/q in the code),
  trailing edge the body's base drag 0.25/M. **Rounded:** leading edge 1.214 − 0.502/M² + 0.1095/M⁴, trailing edge
  0.125/M. **Airfoil:** the rounded leading edge, no trailing edge.
- **Nothing depends on thickness over chord.** A 2% sharp fin and a 10% round one pay the same per unit of frontal area.
- **The rounded term is a modified-Newtonian cylinder. Inference:** (2/3) C_p,stag, with the Rayleigh pitot pressure [9]
  eq. 100, gives 1.021, 1.105 and 1.147 at Mach 1.5, 2 and 2.5. The rule gives 1.013, 1.095 and 1.136: within 1%.

## 2. Linear theory for sharp sections

**Words first.** In thin-airfoil theory each surface element's pressure coefficient is twice its slope over β = √(M² −
1). Summed over the section, drag grows with the square of thickness over chord and falls with β.

- **The formula** [1] Fig. 5-129, p. 5-201; [2] eqs. 30–34, p. 17-13: C_D,w = K (t/c)²/β on the fin's planform area. K
  is 4 for a symmetric double wedge (ridge at mid-chord), 16/3 for a biconvex section and 1 for a single wedge (base
  drag extra). A double wedge with its ridge at m·c has K = 1/(m(1 − m)).
- **Beveled flat plates ("hexagonal"),** beveled over m·c at both ends: K = 2/m ([1] Fig. 5-129). For a fixed bevel
  length a the drag area per fin is (2/β)(t²/a)·s, whatever the chord (Inference).
- **Second order cancels for symmetric sections.** Busemann's term lifts front and rear faces' pressure alike; Hoerner
  puts fore and aft each about 15% off linear (with the boundary layer), opposite ways [2] p. 17-14. **Inference,** by
  exact oblique-shock and Prandtl–Meyer relations [9]: on the Arcas fins (wedge 2.5° streamwise) the total is within
  0.3% of linear theory up to Mach 3 and 0.7% at 4.63. A single front bevel keeps the excess: +6% to +7% at 2.5° and
  +16% to +19% at 6.7°, Mach 1.5 to 2.5.
- **Sweep.** An infinite swept panel sees only the normal Mach number M cos Λ: β becomes √(M² − sec² Λ) on the
  streamwise t/c (Inference). For a front bevel on a 45° leading edge that is ×2.2 at Mach 1.5 and ×1.22 at Mach 2.
  Below M = sec Λ the edge is subsonic and the drag changes "significantly" ([3] p. 123); the charts drop well below 2-D
  ([1] Fig. 5-128 D, F).
- **Low aspect ratio.** The handbook's finite-span charts plot C_D,w/(AR (t/c)²) against AR·β, for several AR·tan Λ_½
  and taper ratios ([1] Fig. 5-128 (A)–(H), pp. 5-193 to 5-200). The Arcas pair has AR 1.52, AR·tan Λ_½ 0.41 on the
  drawn 15° ridge (0.44 on the geometric mid-chord), taper 0.64. Read by eye from panels (G) and (H), it sits about 5%
  above two-dimensional theory at Mach 1.5 and 3% to 4% at Mach 2.96 (Inference).
- **Where it stops.** A wedge's shock detaches above 3.9° at Mach 1.2, 6.7° at 1.3, 9.4° at 1.4, 12.1° at 1.5 and 23.0°
  at 2.0 (Inference, [9]). Across a leading edge the normal Mach number and the normal angle count. A 3 mm (0.12 in) fin
  beveled over 12.7 mm (0.50 in) behind a 39° edge has a detached shock below about Mach 1.75, where theory bounds
  nothing.

## 3. Measured sharp sections against theory

**NACA TN 3548** [4] flew rocket models at Wallops with 6%-thick rectangular wings, aspect ratio 2.7, chord 243 mm (9.58
in). Wing-alone drag came from winged less wingless models, ±0.002 [4] p. 4, at 3–10 × 10⁶ chord Reynolds numbers.
Values are read from Fig. 10 (p. 19) by pixel at 300 dpi, ±0.002. Theory below is Ackeret plus turbulent friction on
both faces: the Prandtl–Schlichting flat-plate formula 0.455/(log₁₀ R)^2.58 times Hoerner's compressibility ratio (1 +
0.15 M²)^−0.58 [2] p. 17-4, eq. 15 (Inference: 0.0051–0.0054).

| Section | Mach | measured | theory + friction | difference | report's own theory |
|---|--:|--:|--:|--:|--:|
| diamond | 1.4 | 0.019 | 0.0201 | +6% | 0.020 |
| diamond | 1.5 | 0.018 | 0.0181 | +1% | n/a |
| diamond | 1.6 | 0.0195 | 0.0166 | −15% | 0.017 |
| circular arc | 1.4 | 0.023 | 0.0250 | +9% | 0.026 |
| circular arc | 1.5 | 0.024 | 0.0224 | −7% | n/a |
| circular arc | 1.6 | 0.0225 | 0.0204 | −9% | 0.021 |
| slab (40% bevel, blunt base) | 1.4 | 0.039 | 0.0362 | −7% | n/a |
| slab | 1.5 | 0.037 | 0.0331 | −11% | n/a |

- **The slab is an HPR-style fin:** a flat plate, beveled at the front, cut square at the back. Its theory row adds the
  single-wedge term (bevel only) and the measured base pressure (−0.36 at Mach 1.4, −0.33 at 1.5 [4] Fig. 6, p. 15)
  times t/c. The rows include wing-body-tail interference, which the report did not separate [4] p. 2.
- **The diamond had the least drag above Mach 1.1, the circular arc below it** [4] p. 6; theory's K is 4 against 5.3.
- **Two-dimensional sections at Mach 1.5** [2] Fig. 15, p. 17-14: a 14%-thick round-nosed section tested at a reduced
  coefficient β C_D/(t/c), friction subtracted, of 7.7, against theory's 4.0 for a double wedge and 5.3 for a biconvex.
  For the whole 9% NACA 65-009 section, pressure plotting gave K about 7.5 at Mach 1.65 and 11 at 2.4 [2] p. 17-15.

## 4. Blunt edges faster than sound

- **Square leading edge.** Measured two-dimensional blunt edges fit C_D/C_p,stag = 0.94 − 0.046/M² on the edge's frontal
  area, down to Mach 2 [2] p. 18-14, eq. 42. A blunt cylinder's face gives 0.90 [2] p. 16-13. HPR Sim's 0.85 is 7% to 9%
  below the edge value from Mach 1.5 to 2.5 (Inference; at 1.5 the fit is extrapolated).
- **Sweep on a blunt edge.** Newtonian theory scales the edge pressure by cos² Λ: a 60° half-cylinder edge at Mach 5
  measured 0.27 of q against 0.25 [2] p. 18-16. Stagnation pressure also falls with the normal Mach number: on a 39°
  edge at Mach 1.5 (normal Mach 1.17) it is 0.90 of the freestream value (Inference), which HPR Sim uses, so it reads
  ≤11% high near Mach 1.5.
- **Round leading edge.** Measured half-circle edges sit about 0.02 of C_p,stag above Newtonian's 0.67 from about Mach 3
  up [2] p. 18-15 (Fig. 24). No test of a thin flat plate with a round edge at Mach 1.2 to 2.5 was found. This note
  bounds it between Newtonian at the normal Mach number and 3% above it at the freestream one.
- **Round-nosed airfoils carry far less.** HPR Sim gives an "airfoil" fin the whole round-edge term on its thickness.
  Scaled with §3's whole-section K values to a 3 mm fin of 80 mm mean chord (t/c 0.0375), a 65-series section's wave
  drag is a third of the rule's on a 38.7° edge, cos² Λ included (Inference: ×3.0 at Mach 1.65, ×3.6 at 2.4; unswept
  ×4.9, ×6.0).
- **Square trailing edge: about twice the rule.** The slab's measured base pressure is −0.47 at Mach 1.2, −0.36 at 1.4
  and −0.30 at 1.6 [4] Fig. 6, p. 15. Chapman, Wimbrow and Kester's blunt-trailing-edge wings, as compiled in [3] Fig.
  79 (p. 127), give 0.37, 0.30, 0.22, 0.16 and 0.12 at Mach 1.4, 1.6, 2.0, 2.5 and 3.0 (read by pixel, ±0.005). HPR
  Sim's 0.25/M is 0.17, 0.13 and 0.10 at Mach 1.5, 2 and 2.5: 0.50 to 0.61 of the measured value. A body of revolution's
  base suction is "much less" than the slab's [4] p. 5: the rule borrows the wrong base (Inference).

## 5. The Arcas Robin: what linear theory closes

The Arcas fins are 4.4% thick at root and tip (0.150 in over 3.38 in, 0.096 in over 2.165 in), a double wedge with its
ridge swept 15° (2.72° normal to it) [6] Fig. 1(b), printed p. 8. Four fins have 5.87 times the reference area in
planform. Linear theory gives 0.0462/β on the reference area: 0.0413 at Mach 1.5 and 0.0102 at 4.63, against the 0.24 to
0.28 HPR Sim charges the committed design's "airfoil" section (round leading edge, no trailing edge). Measured
increments and HPR Sim's terms come from the committed fixture [5] (fins on less fins off, short model). Inference:

| Mach | measured increment | HPR Sim, pressure + friction | linear + friction | left over | fins on, now → linear |
|--:|--:|--:|--:|--:|--:|
| 1.5 | 0.153 | 0.241 + 0.032 | 0.073 | 0.080 | +39.4% → −1.1% |
| 1.8 | 0.112 | 0.255 + 0.030 | 0.061 | 0.051 | +50.5% → −2.0% |
| 2.3 | 0.085 | 0.267 + 0.027 | 0.049 | 0.036 | +76.7% → +3.1% |
| 2.96 | 0.061 | 0.276 + 0.023 | 0.040 | 0.021 | +105.0% → +6.8% |
| 3.96 | 0.050 | 0.281 + 0.019 | 0.031 | 0.020 | +139.7% → +8.7% |
| 4.63 | 0.046 | 0.283 + 0.016 | 0.027 | 0.020 | +154.0% → +6.4% |

- **The long model agrees:** fins on +1.0% to +6.5% from Mach 1.8 to 4.63, with 0.015 to 0.068 left over.
- **Sweep and second order barely move it.** Sweep on the 15° ridge adds 3% at Mach 1.5 and under 2% from 1.8. The exact
  shock-expansion value differs from linear by under 0.7%. Finite span adds about 5% (§2). None closes 0.08.
- **Over-closing.** Linear theory removes 167% of HPR Sim's excess fin drag at Mach 1.5, 117% at 2.3, 108% at 4.63,
  raising simulated apogees of sharp-finned rockets.
- **Base-lip flow, documented.** The report finds fins-off axial force "low" at Mach 1.50 and 1.80 from the model's
  reflexed base lip, an effect the fins hide by thickening the boundary layer [6] p. 6. Part of the two largest
  leftovers (0.080, 0.051) is lip flow, not fin drag; its size isn't given.
- **What else it could be, unbounded here:** the fins' pressure field on the 15° boattail behind them, where the aft
  1.54 in of each root sits; edge radii the drawings don't give; the No. 60 grit strips 0.5 in behind each fin's leading
  edge [6] p. 4. Issue [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70) expected interference.

## 6. Worked example: Calisto's fins

Calisto (RocketPy's example rocket, a public design [8]) has four fins: 120 mm (4.72 in) root, 40 mm (1.57 in) tip, 100
mm (3.94 in) span, leading edge swept 38.7°, trailing edge unswept. HPR Sim's guess is square, 3 mm (0.12 in) thick. The
fins' frontal area is 0.0947 of the reference area (127 mm, 5.00 in, diameter). Fin pressure drag on the reference area,
by section (Inference; "measured" applies §4's two-dimensional values; "bounds" runs from no trailing-edge drag to the
measured square-edge base pressure):

| Mach | square, rule | square, measured | rounded, rule | rounded, bounds | beveled 12.7 mm, linear | airfoil, rule |
|--:|--:|--:|--:|--:|--:|--:|
| 1.5 | 0.091 | 0.105–0.113 | 0.066 | 0.053–0.093 | 0.040–0.049 (detached) | 0.059 |
| 2.0 | 0.093 | 0.105–0.110 | 0.069 | 0.060–0.087 | 0.026–0.028 | 0.063 |
| 2.5 | 0.094 | 0.104–0.108 | 0.070 | 0.064–0.084 | 0.020 | 0.066 |

- **Square edges read low.** HPR Sim's C_D0 is 0.542 at Mach 1.5 and 0.435 at 2.0 (`rocketpy-drag-curves.json`). The
  measured square edge adds 0.014 to 0.022 at Mach 1.5 and 0.012 to 0.017 at 2: against RASAero II, −9.5% becomes −7.2%
  to −5.9%, and −14.9% becomes −12.9% to −11.7%, a fifth of the Mach 2 gap. RASAero II's fin section isn't recorded.
- **Beveled fins cut fin pressure drag by 70% to 79%** against the square rule at Mach 2 to 2.5. At Mach 1.5 the swept
  bevel's shock is detached (8.6° normal angle against 3.2° allowed), so the linear row is not a bound there.

## What the project can use

- **Cache:** TN 3548 [4] (NTRS, public use) in gitignored `refs/research/fin-wave-drag/`; [1], [3], [6], [7] (US
  government works, public release) in `refs/papers/`. Hoerner [2] is copyright L. A. Hoerner: cite only.
- **Suggestion 1, sharp sections ([#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70), core band).** Add
  double-wedge, hexagonal (bevel length), single-wedge and biconvex sections with K (t/c)²/β on planform area, swept by
  √(M² − sec² Λ), from Mach 1.4 where the shock is attached. Expected: the Arcas fins-on rows within 10% (−2.0% to
  +8.7%), but the fin increment 35% to 52% low. Record that miss; don't score the rows as explained.
- **Suggestion 2, square trailing edge.** Replace 0.25/M on the fin's trailing edge by the measured two-dimensional base
  pressure ([3] Fig. 79 from Chapman, Wimbrow and Kester; [4] Fig. 6). Expected: +0.009 on Calisto's C_D0 at Mach 2, its
  −14.9% to −13.2%; lower apogees for square-finned supersonic flights.
- **Suggestion 3, airfoil sections.** Scale the round-nose term with the nose radius, or use K about 7.5 to 11 from the
  65-009 data, not the full cylinder on the thickness. Expected: about a third of today's term.
- **For [M1.14d, Mach 1.2 to 2.5](../decisions-and-roadmap.md#m1-14d):** suggestions 1 and 2 move opposite ways on
  apogee. Pin each to its own measurement (§3 rows, Arcas increment) before the Calisto comparison.
- **The handbook check.** MIL-HDBK-762's sample single-wedge fins, 0.022 to 0.009 from Mach 1.6 to 3.2 [5], are left out
  of that comparison; at 1.6 that is 0.002 of wave drag and 0.020 of blunt trailing edge, so suggestions 1 and 2
  together could put them back in.

## Open questions

- What makes the 0.020 to 0.080 left over in the Arcas fin increment, and how much is base-lip flow? NASA's
  wingless-missile tests with four fin planforms (TM X-2774, [ironbark](ironbark.md) §4) could separate interference
  from section drag.
- No supersonic test of a thin flat plate with a round leading edge, or of a round trailing edge, was found.
- The swept, detached-shock bevel (most beveled HPR fins below about Mach 2) has no theory or test here.
- Chapman, Wimbrow and Kester's NACA Report 1109 was read only through [3] Fig. 79; its base pressure varies with
  trailing-edge thickness over boundary-layer thickness, which a hobby fin's root may not match.

## Sources

All read 2026-10-16. Printed page numbers unless "PDF".

1. MIL-HDBK-762(MI), *Design of Aerodynamically Stabilized Free Rockets*, 1990: Figs. 5-128, 5-129, pp. 5-193 to 5-201.
   <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets>
2. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965: pp. 16-13, 17-4, 17-13 to 17-15, 18-14 to 18-16.
   <https://archive.org/details/FluidDynamicDragHoerner1965>
3. L. A. Mason et al., *Aerodynamic Design Manual for Tactical Weapons*, NSWC TR 81-156, 1981: pp. 123, 127.
   <https://archive.org/download/DTIC_ADA109180/DTIC_ADA109180.pdf>
4. J. D. Morrow, E. Katz, NACA TN 3548, 1955: pp. 1–6, 15, 19. <https://ntrs.nasa.gov/citations/19930084307>
5. The project's fixtures `drag-vs-mach.json`, `rocketpy-drag-curves.json` and `mil-hdbk-762-sample-drag.json`;
   the [aero page](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel).
6. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967: pp. 4, 6, 8. <https://ntrs.nasa.gov/citations/19670020031>
7. J. C. Ferris, NASA TN D-4013, 1967: Fig. 1(b), p. 10. <https://ntrs.nasa.gov/citations/19670020050>
8. RocketPy's Calisto as committed: `validation/designs/rocketpy-calisto-tests-motor-at-minus-1.373.json`.
9. Ames Research Staff, *Equations, Tables, and Charts for Compressible Flow*, NACA Report 1135, 1953: eq. 100
   (Rayleigh pitot), eq. 171 (Prandtl–Meyer), oblique-shock relations. <https://ntrs.nasa.gov/citations/19930091059>
10. S. Niskanen, *Development of an Open Source model rocket simulation software*, MSc thesis, 2009, pp. 49–50.
    <https://openrocket.sourceforge.net/thesis.pdf>
