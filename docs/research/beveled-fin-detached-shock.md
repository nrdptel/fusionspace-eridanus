# Transonic drag of a swept, beveled fin's leading edge, Mach 1.1 to 1.75

**In short: from Mach 1.5 to 1.75, a 3 mm fin beveled over 12.7 mm behind a 38.7° edge carries 0.31 to 0.41 of
leading-edge pressure drag on its frontal area: 1.1 to 1.7 times linear theory, 0.37 to 0.50 of HPR Sim's square-edge
rule.** That comes from two-dimensional wedge tests with detached shocks, swept by the normal Mach number. In those
tests the bevel face's pressure sits on a plateau 1.2 to 1.6 times linear theory at Mach 1.21 to 1.37, and 0.81 to 0.90
of transonic theory's peak. Below Mach 1.45 linear theory reads high instead, and below 1.28 the swept edge is subsonic:
sweep theory then gives 0.08 to 0.16. Two effects the two-dimensional numbers leave out pull opposite ways. Finite span
relieves: an unswept, beveled slab in flight drew 0.71 to 0.74 of its two-dimensional parts at Mach 1.1 to 1.2. The fin
root sees the edge unswept: 4 to 6 times sweep theory at Mach 1.1. So this note suggests sweep theory on a measured
transonic-wedge curve, with a band, over linear theory or the square rule. On a beveled Calisto that lowers C_D0 by
0.038 to 0.056 at Mach 1.5 (7.1% to 10.3%), raising apogee. Trust: the wedge tests ran at a chord Reynolds number of
60,000, and no swept bevel test was found. This note follows [fin-wave-drag](fin-wave-drag.md) (§2, §6). "Inference"
marks this note's own arithmetic.

## 1. The fin, and why linear theory stops

- **The section.** A flat plate 3 mm (0.12 in) thick, beveled on both faces over 12.7 mm (0.50 in) at the front and cut
  square at the back. Each face slopes 6.74° streamwise. Normal to a 38.7° leading edge the slope is 8.61° (tan θ_n =
  tan θ / cos Λ) and the normal Mach number is M_n = 0.780 M (Inference). The unrounded sweep, 38.66°, gives 8.60° and
  0.781 M.
- **Where the shock detaches.** Exact oblique-shock relations [R] put detachment for 8.61° at M_n 1.370, so M 1.754 to
  1.756. The edge is subsonic below M = sec Λ = 1.28 (Inference). Every Mach number this note covers is transonic at the
  edge.
- **The two rules it is compared with** ([fin-wave-drag](fin-wave-drag.md) §1, §2), both on the fin's frontal area,
  thickness times span: HPR Sim's square edge, 0.85 C_p,stag cos² Λ; linear theory, (2 tan θ_n / β_n) cos² Λ with β_n =
  √(M_n² − 1). Linear theory grows without bound as M_n falls to 1; the square rule never depends on the bevel.
- **Transonic similarity, in words.** Thin wedges at the same reduced free-stream Mach number (Bryson's term), ξ = (M² −
  1) / [(γ + 1) τ]^(2/3), share one reduced drag C̃ = (γ + 1)^(1/3) C_D / τ^(5/3) [B] p. 16, eqs. 10–12. Here τ = tan θ,
  and C_D is the front part's drag on twice the wedge's length [B] p. 35. On the wedge's own frontal area the face's
  mean pressure coefficient is then C̃ τ^(2/3) / (γ + 1)^(1/3), and linear theory reads C̃ = 2/√ξ (Inference).

## 2. Two-dimensional wedges with detached shocks: a plateau 1.2 to 1.6 times linear

Bryson measured wedges of 4.53°, 7.56°, 10.00° and 26.57° half-angle, each followed by a straight section about 1/4 in
behind the nose, in a 4 by 10 in transonic tunnel by interferometer [B] p. 7. The chord Reynolds number was about 60,000
with laminar boundary layers [B] p. 14. Supersonic Mach numbers carry a ±0.05 standard deviation [B] p. 13. Values below
are read by eye from 300 dpi scans, ±0.002 in C_D and ±0.03 in C̃.

- **Pressure drag against Mach number** [B] Fig. 14, p. 81. The 7.5° wedge reads C_D 0.072 to 0.074 at Mach 1.21 to
  1.27, falling to 0.047 at 1.39. The 10° wedge reads 0.106 to 0.110 from Mach 1.21 to 1.37, then 0.080 at 1.47. The
  chart's shock-expansion curve reads 0.0465 at Mach 1.40 for 7.56°, against an exact 0.0467 [R] (Inference).
- **Against linear theory.** The face's mean pressure coefficient is C_D / tan θ: 0.55 (7.5°) and 0.61 (10°). That is
  1.2 to 1.6 times linear theory from Mach 1.21 to 1.37 (Inference). The excess grows toward attachment, where the drag
  then drops onto exact oblique-shock theory [B] Fig. 14.
- **The plateau in similarity form** [B] Fig. 13, p. 80. From ξ 0.82 to about 1.35 the measured C̃ is 2.62 to 2.71
  (10°), 2.84 to 2.90 (7.5°, ξ 1.00 to 1.32) and 2.89 to 2.91 (4.5°). The 7.5° wedge is down to 2.74 at ξ 1.43. At ξ
  1.66 to 1.77 it reads 2.41 to 2.56 (10°, 7.5°) and 2.14 to 2.25 (4.5°), and 2.0 at ξ 2.0.
- **Transonic theory sits above it** [B] Fig. 8, p. 66, from Guderley and Yoshihara at M = 1, Vincenti and Wagoner
  (detached), Cole (subsonic) and Tsien and Baron (attached). It reads C̃ 0.70 at ξ −0.5, 1.75 at 0, 2.66 at 0.5 and
  about 3.23 at its peak, near ξ 1.0. The measured plateau is 0.81 to 0.90 of that peak (Inference).
- **Subsonic, it agrees.** Measured C̃ at ξ −0.45 to −0.55 is 0.62 to 0.64 (10°, 7.5°; one 4.5° point reads 0.46 at
  −0.57), and 0.94 at −0.37 [B] Fig. 13, against Cole's 0.70 and about 0.94 (Fig. 8). The tests have no point between ξ
  −0.37 and 0.82.
- **Why there is no spike.** Bryson shows that the local Mach number on an unswept two-dimensional body is stationary at
  M = 1 and changes slowly near it [B] p. 1. Linear theory's 2/√ξ crosses the theory curve near ξ 0.55: below it linear
  theory reads high, above it low until the shock attaches (Inference).
- **Shock attachment in similarity form drifts.** For the 7.5° wedge, oblique-shock theory gives attachment at ξ 1.68
  and small-perturbation theory at 1.19 [B] p. 34, footnote. The measured plateau holds to about ξ 1.35 (Fig. 13).

## 3. A finite beveled slab in flight: the two-dimensional parts add up 35% to 41% high

Morrow and Katz flew a rocket model with a rectangular, unswept wing of aspect ratio 2.7, 6% thick, beveled over 40% of
its 9.582 in chord and cut square behind [N] pp. 1, 4. The bevel's faces slope 4.29°, so its shock detaches below Mach
1.213 [R] (Inference). Wing drag is the winged less the wingless model, ±0.002 [N] pp. 4, 5; base drag is the base
pressure at two central slits times thickness over chord [N] p. 5, "uncorrected for tip and root effect" (legend of Fig.
8, page 17). Read by pixel from Fig. 8, on wing planform area; the last four columns are this note's (Inference):

| Mach | wing | base | friction (report) | left for the bevel | 2-D bevel [B] | linear | exact |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 1.1 | 0.045 | 0.034 | 0.0065 | 0.0045 | 0.021–0.023 | 0.020 | detached |
| 1.2 | 0.041 | 0.028 | 0.0065 | 0.0065 | 0.021–0.023 | 0.014 | detached |
| 1.3 | 0.038 | 0.024 | 0.0065 | 0.0075 | not used | 0.011 | 0.013 |
| 1.4 | 0.039 | 0.021 | 0.0065 | 0.0115 | not used | 0.009 | 0.010 |
| 1.5 | 0.037 | 0.020 | 0.0065 | 0.0105 | not used | 0.008 | 0.009 |

- **Detached, the parts overshoot.** The two-dimensional bevel (C̃ 2.6 to 2.9, carried at Mach 1.1, ξ 0.66, just below
  the plateau's first point) plus the measured base and the report's friction (its line reads 0.0062 to 0.0066) sums to
  0.062 to 0.064 at Mach 1.1 and 0.056 to 0.058 at 1.2: 35% to 41% above the measured wing. Attached, at 1.4 and 1.5,
  the same sum with exact bevel drag reads about 4% low (Inference).
- **The report saw it too.** Below about Mach 1.35 its components add to more than the wing's drag; it names possible
  favorable interference on the tail, and base slits that don't read the base's average [N] p. 5.
- **Finite span predicts about that much.** Hoerner's transonic correlation drops wing wave drag below its
  two-dimensional level once A (t/c)^(1/3) falls under about 3 to 4 [H] p. 17-12, Fig. 12. That figure is for
  round-nosed wings at M = 1; using it for a bevel at Mach 1.1 to 1.2 is a transfer. For the slab, 2.7 × 0.06^(1/3) =
  1.06 reads about 0.7 of the plateau, against the measured 0.71 to 0.74 (Inference). The data can't say whether the
  bevel's or the base's drag was relieved.

## 4. Sweep: the normal-Mach rule holds for a long edge, not at the root

- **A long swept edge, yes.** The cross-flow principle, that an infinite yawed panel's pressures follow the normal
  velocity component alone, is independent of compressibility [H] p. 15-20. Nothing in it is linearized, so a detached
  normal-plane shock is still a two-dimensional wedge flow at M_n, its pressures times cos² Λ (Inference). A 60° blunt
  edge at Mach 5 measured 0.27 of q against Newtonian theory's 0.25 ([fin-wave-drag](fin-wave-drag.md) §4).
- **The root, no.** The centerline of a swept wing meets the flow unswept, and its critical Mach number lies between the
  straight wing's and that over cos Λ [H] p. 15-22; only the center part and spots at the tips go sonic early [H] p.
  15-23. A fin's root on the body plays the centerline's part (Inference).
- **Which way the root moves it.** Unswept, the 6.74° bevel sits at ξ 0.49 at Mach 1.1, in the tests' gap (theory times
  0.83 to 0.98), on the plateau at 1.2 (ξ 1.02), and attached from 1.31. Against sweep theory that is 4 to 6 times as
  much drag at Mach 1.1, 3 to 4 at 1.2, about equal at 1.4, and 0.5 to 0.7 times from 1.5 to 1.75 (Inference, §6). Sweep
  only helps below about Mach 1.4. How much of the edge acts as root isn't bounded by any source found.

## 5. MIL-HDBK-762's transonic charts

- **Figs. 5-130 and 5-131** give transonic wave drag of rectangular and delta fins against (M² − 1)/(t/c)^(2/3), for
  several A (t/c)^(1/3) [M] p. 5-202; other planforms lie between them, and none is precise at transonic speeds [M] p.
  5-49. Fig. 5-130 rests on NACA Report 1253's rounded sections [M] p. 5-73; neither covers a bevel or a square base.
- **Fig. 5-132** is a whole double wedge [M] p. 5-203. Its printed ordinate, C_D,w (M/(2t/c))^(2/3), does not reduce to
  linear theory. Read as the usual similarity drag it matches linear theory, 4/√(abscissa), within 3% at abscissa 3 to 7
  (Inference). On its plateau it sits at linear theory at abscissa 1, where Bryson's front half alone is 1.3 to 1.7
  times linear (ξ 0.82 to 1.35). Halving it for a front-only bevel would read low (Inference).


## 6. Worked example: Calisto with a 12.7 mm bevel

Calisto (RocketPy's example rocket, a public design [C]) has four fins: 120 mm root, 40 mm tip, 100 mm span, leading
edge swept 38.7°, trailing edge unswept, 3 mm thick. The design's section is square; this asks what a 12.7 mm bevel
would change. Leading-edge pressure drag on the fin's frontal area, freestream dynamic pressure (Inference; "sweep
theory" applies §2's C̃ at M_n, "unswept" the same tests at M with 6.74°, exact theory once attached):

| Mach | M_n | ξ | sweep theory | linear | unswept | square rule | sweep theory, Calisto ref |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 1.1 | 0.86 | −0.52 | 0.078–0.097 | none | 0.40–0.47 | 0.692 | 0.0073–0.0092 |
| 1.2 | 0.94 | −0.24 | 0.142–0.155 | none | 0.48–0.57 | 0.723 | 0.0135–0.0147 |
| 1.3 | 1.01 | 0.06 | 0.19–0.23 | 1.08 | 0.42–0.48 | 0.750 | 0.018–0.022 |
| 1.4 | 1.09 | 0.38 | 0.27–0.32 | 0.419 | 0.300 | 0.773 | 0.026–0.030 |
| 1.5 | 1.17 | 0.73 | 0.34–0.39 | 0.303 | 0.250 | 0.793 | 0.032–0.037 |
| 1.6 | 1.25 | 1.10 | 0.34–0.41 | 0.247 | 0.220 | 0.810 | 0.032–0.039 |
| 1.75 | 1.37 | 1.70 | 0.31–0.33 | 0.198 | 0.190 | 0.832 | 0.029–0.031 |

- **C̃ used:** 0.60–0.75 (measured and Cole); 1.10–1.20 (Cole, no data at ξ −0.24); 1.5–1.8 and 2.1–2.45 (theory times
  0.85 to 1.0, no data); 2.6–3.0 (plateau carried below its first point, ξ 0.82) and 2.65–3.15 (measured plateau to
  theory); 2.4–2.55 (measured; exact attached at Mach 1.76 is 2.36 to 2.39, sensitive to the sweep this near
  detachment).
- **Other areas.** Per fin on its own planform (8,000 mm²), multiply by 0.0375: 0.0126 to 0.0145 at Mach 1.5. On
  Calisto's reference area (127 mm diameter), multiply by 0.0947, the four fins' frontal share
  ([fin-wave-drag](fin-wave-drag.md) §6).
- **"None"** marks a subsonic edge: linear theory then gives no wave drag to a wedge on a long flat [B] p. 34.
- **The band this note suggests,** on the reference area: from 0.6 times sweep theory (finite span, §3: Calisto's fin
  pair has A (t/c)^(1/3) = 2.5 × 0.0375^(1/3) = 0.84, about 0.6 on [H] Fig. 12) up to the larger of sweep theory and
  unswept (root, §4). That is 0.004–0.045 at Mach 1.1, 0.008–0.054 at 1.2, 0.015–0.030 at 1.4, 0.019–0.037 at 1.5 and
  0.018–0.031 at 1.75 (Inference).
- **Against the square rule.** Its leading-edge term is 0.066 to 0.079 on the reference area. The band lowers HPR Sim's
  C_D0 (`rocketpy-drag-curves.json` [C]) by 0.014 to 0.060 at Mach 1.2 (2.2% to 9.3% of 0.649), 0.038 to 0.056 at 1.5
  (7.1% to 10.3% of 0.542) and 0.048 to 0.061 at 1.75 (9.8% to 12.7% of 0.483). The square trailing edge is the same.
- **Against [fin-wave-drag](fin-wave-drag.md) §6.** Its Mach 1.5 cell, "linear 0.040–0.049 (detached)", holds 0.0287 of
  linear leading edge (Inference). Sweep theory makes that 0.032 to 0.037: a detached excess of +0.003 to +0.008.
- **Two causes this table leaves out.** A flat of height h left at the bevel's nose adds the square rule on h: a 0.3 mm
  flat on 3 mm adds a tenth of 0.075, +0.0075 at Mach 1.5 (Inference). A bevel cut on one face only (13.3° streamwise,
  16.8° normal) stays detached up to about Mach 2.17 (Inference) and needs its own row.

## What this suggests

- **For [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70)'s attached-shock limit:** take the limit at the
  normal angle and Mach number, M = M_det(θ_n) / cos Λ: about 1.755 for this fin, 1.30 for the same bevel unswept. Above
  it, exact or linear-plus-second-order wedge theory on the faces; [fin-wave-drag](fin-wave-drag.md) §2 covers that.
- **Below the limit, a transonic-wedge term.** For a beveled section with bevel length a: θ_n, M_n and ξ as in §1; C̃(ξ)
  from Bryson's curves: Cole and the subsonic points below ξ 0, 1.75 at 0, the measured plateau 2.75 ± 0.15 from ξ 0.8
  to attachment, theory times 0.85 to 1.0 between. Drag on the frontal area is C̃ τ_n^(2/3) / (γ + 1)^(1/3) times cos²
  Λ. Expected on Calisto: the sweep-theory column of §6, 0.42 to 0.50 of the square rule at Mach 1.5 to 1.6.
- **Carry the band, don't pick an edge.** The band's lower edge (finite span) and upper edge (root) are inferences from
  unswept tests; each output across Mach 1.1 to 1.75 should show it. From Mach 1.1 to 1.3 it spans a factor of 4 to 10.
- **For [M1.14d, Mach 1.2 to 2.5](../decisions-and-roadmap.md#m1-14d):** no swept bevel has been measured in this range,
  so a beveled-fin comparison there can't score the bevel term. On Calisto the band alone is 0.018 of C_D0 wide at Mach
  1.5 and 0.046 at 1.2 (Inference). Pin the term on a measured flight or tunnel case before it closes a gap.

## What the project can use

- [B] and [N] are US government works, public use permitted (NTRS); [M] is cleared for public release; all three are
  cached in gitignored `refs/`. Bryson's Figs. 8, 13 and 14 can be digitized by pixel into a committed table with their
  citation. Hoerner [H] is copyright: cite only.

## Open questions

- No test of a swept bevel or swept wedge section at Mach 1.1 to 1.75 was found. Arbic and Gillespie's NACA RM SL53J02
  (a blunt trailing edge on a swept arrow wing, Mach 0.9 to 1.8) is a lead, not read.
- Bryson's tests have no point between ξ −0.37 and 0.82, so on Calisto's fins Mach 1.15 to 1.52, including the 1.2 and
  1.5 rows, rests on theory or a carried plateau (Inference).
- His Reynolds number is 60,000, laminar. He reports viscous effects at nose and shoulder nearly cancel in the drag [B]
  p. 34; a hobby fin at 10⁶ to 10⁷, turbulent, was not tested.
- How far along a fin's edge the root's unswept flow reaches. Vincenti and Wagoner and Cole were read via [B] Fig. 8.

## Sources

All read 2026-10-20. Printed page numbers.

1. **B.** A. E. Bryson Jr., *An Experimental Investigation of Transonic Flow past Two-Dimensional Wedge and Circular-Arc
   Sections Using a Mach-Zehnder Interferometer*, NACA TN 2560, 1951 (Hoerner cites it as NACA Report 1094 [H] p.
   17-11): pp. 1, 7, 13, 14, 16, 34, 35, Figs. 8, 13, 14 (pp. 66, 80, 81).
   <https://ntrs.nasa.gov/citations/19930083267>; PDF <https://authors.library.caltech.edu/79538/1/19930083267.pdf>
2. **N.** J. D. Morrow and E. Katz, NACA TN 3548, 1955: pp. 1, 4, 5, Fig. 8 (p. 17).
   <https://ntrs.nasa.gov/citations/19930084307>
3. **H.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965: pp. 15-20, 15-22, 15-23, 17-11, 17-12 (Fig. 12).
   <https://archive.org/details/FluidDynamicDragHoerner1965>
4. **M.** MIL-HDBK-762(MI), *Design of Aerodynamically Stabilized Free Rockets*, 1990: pp. 5-49, 5-73, Figs. 5-130 to
   5-132 (pp. 5-202, 5-203). <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets>
5. **R.** Ames Research Staff, *Equations, Tables, and Charts for Compressible Flow*, NACA Report 1135, 1953:
   oblique-shock relations, used for this note's exact wedge values. <https://ntrs.nasa.gov/citations/19930091059>
6. **C.** RocketPy's Calisto as committed, `validation/designs/rocketpy-calisto-tests-motor-at-minus-1.373.json`, and
   HPR Sim's C_D0 rows in `validation/fixtures/aero/rocketpy-drag-curves.json` (case `calisto-power-off`).
