# Base drag of a thin square trailing edge against boundary-layer thickness, Mach 0.3 to 0.9

**In short: a parallel-sided fin whose boundary layer is about as thick as the fin carries a square trailing-edge
base drag near 0.30 at Mach 0.3 and 0.35 at Mach 0.8, on the edge's area: 2.3 and 1.7 times the rule's 0.132 and
0.203.** This follows [fin-edge-drag](fin-edge-drag.md) §3 (0.10–0.19 at low speed, 0.29–0.39 near Mach 0.8, h/c
near 2%). That gap is mostly not Mach number: Hoerner's low-speed line is cut-off airfoils', and four parallel-sided
tests sit 1.6–2.2 times above it; Nash's transonic points are thickened, partly laminar sections. Base drag falls
about as the cube root of momentum thickness and rises only ×1.05–1.37 from Mach 0.3 to 0.8; the rule's is ×1.54.
So [#614](https://github.com/nrdptel/fusionspace-eridanus/issues/614)'s second run (0.135/∛C_fB times the rule's
rise) reads low by about 0.11 at Mach 0.3 and 0.05 at 0.8. The level rests on one modern test at the fin's own
boundary layer and three thinner-layer tests; figures are read by eye. "Inference" marks this note's arithmetic.

## 1. Terms, and the fin this note sizes

- **Base drag** is −C_pb, the base pressure coefficient's magnitude, on the base area (span times thickness h).
- **C_fB** is Hoerner's forebody friction over base area, C_f S_wet/S_B [H] p. 3-19, eq. 33: 2 C_f c/h for a fin
  wet on both sides. A flat plate's momentum thickness per side is θ = C_f c/2, so C_fB = 4θ/h (Inference).
- **Hoerner's two lines:** 0.135/∛C_fB for airfoils with cut-off or thickened edges (eq. 39), and 0.1/∛C_fB for
  sheet-metal steps, which shed no vortex street (eq. 38) [H] p. 3-21.
- **The fin (invented, fin-edge-drag §6):** 3.2 mm thick, 156 mm mean chord, turbulent. Inference, flat-plate
  formulas, sea level: at Mach 0.3, 1.1 × 10⁶ on chord, δ/h 1.1, θ/h 0.11, C_fB 0.45; at 0.8, δ/h 0.92, θ/h
  0.089, C_fB 0.37. C_fB 0.43 is used, as in fin-edge-drag; on thickness, 22,000 (Mach 0.3) to 60,000 (0.8).

## 2. Thin boundary layer, parallel sides: about 0.6, rising slowly with Mach number

- **The test.** Nash, Quincey and Callinan's section had a 6 in chord, a wedge forebody and a 2 in parallel
  afterbody, h/c 10%, in a slotted transonic tunnel with 3.5% blockage, uncorrected [N1] p. 4. Its chord Reynolds
  number was 0.8 × 10⁶ at Mach 0.2, and its layers turbulent at the edge from Mach 0.4 [N1] p. 6.
- **Its layer was thin:** δ/h about 0.12 one inch ahead of the edge at Mach 0.4 and 0.8, 0.145 at the edge at 0.8
  [N1] p. 35, Fig. 10. Profiles follow a 1/7 power law (Fig. 9), so θ/h is 0.012–0.014 (Inference).
- **Base pressure on the centerline** (hole 6) [N1] p. 36, Fig. 13: −0.61 at Mach 0.2, −0.625 at 0.4, −0.655 at
  0.6, −0.74 to −0.75 at 0.8, a dip to −0.71 near 0.875, then −0.90 near 0.95 [N1] p. 8. The text gives −0.6
  at low speed and about −0.7 at 0.8 [N1] p. 7. The centerline is the peak suction; an edge hole read −0.60 to
  −0.70. Doubling the Reynolds number at Mach 0.8 moved C_pb by under 1% [N1] p. 8.
- **A step for scale.** A step on the same model, with no vortex street, held about −0.2, and −0.23 at Mach 0.8
  against the section's −0.75 [N1] p. 13; an airfoil's can be three times a step's [N2] p. 4. Inference: on the
  step's height h_s (half the base), C_fB = 2θ/h_s and the step's 0.20 is 0.075/∛C_fB, near Hoerner's step's 0.1.
- **Rounding is not a safe half:** a semicircular edge here read −1.10 at Mach 0.925 [N1] pp. 22–23.

## 3. Thicker boundary layer: less base drag, by the cube root

- **The sign.** A thicker layer raises the base pressure: Hoerner's "insulating" layer [H] p. 3-19. A review of
  later tests (Bearman; Petrusma and Gai; Rowe et al.) finds the same, with base pressure flat once h/θ exceeds
  about 25 (laminar) to 35 (turbulent) [K] p. 2. Nash (h/θ near 77), Doddipatla and Morrow (33–34) sit on or near it.
- **Parallel-sided tests**, faired nose and square edge (θ from δ by the 1/7 law; C_fB and K are Inference):

| Test | c/h | δ/h | θ/h | C_fB | −C_pb | K = −C_pb ∛C_fB |
|------|----:|----:|----:|-----:|------:|----------------:|
| Nash [N1], Mach 0.2–0.4 | 10 | 0.12–0.145 | 0.012–0.014 | 0.047–0.056 | 0.61–0.625 | 0.22–0.24 |
| Doddipatla [D], low speed, sand strip | 12.5 | about 0.3 | about 0.029 | 0.12 | 0.61 | 0.30 |
| Morrow and Katz [N5], Mach 0.6, flight | 17 | not measured | about 0.03 | 0.12 | 0.55 | 0.27 |
| Zhao [Z], low speed, tripped | 56 | 1.2–1.75 | 0.10–0.17 | 0.39–0.68 | 0.315 | 0.23–0.28 |

- **Doddipatla:** −0.61 at 24,000 and 46,000 on thickness, 4.2% blockage [D] pp. 23, 107–108, Table 4.2. δ/h
  0.3 is Naghib-Lahouti et al.'s (2014) on the same model, Reynolds number not given, as Zhao cites it [Z] pp. 7, 38.
- **Morrow and Katz:** a 6% rectangular slab, 40% chord bevel, 9.582 in chord, 2.8 × 10⁶ at Mach 0.625 [N5]
  pp. 1, 4. θ/h assumes a turbulent layer (Inference); laminar, as Hoerner assumed (§5), K would be 0.16.
- **Zhao:** about −0.315 with no control, at 35,000 [Z] p. 49, Fig. 4.12, on a 56h plate with a layer near 1.75h
  [Z] p. 19. On a 54h plate at that Reynolds number he measured θ/h 0.098 and δ/h 1.2 [Z] p. 37, Table 3.1. Its
  blockage is 2–3% (Inference, from the 1.2 m by 0.8 m section [Z] p. 14); no correction is described. It is
  the fin's twin (c/h 56 against 49, θ/h 0.10 against 0.11), and it is one test.
- **A cube root fits the set (Inference).** From Nash's θ/h 0.013 to Zhao's 0.10, K stays 0.22–0.24. The four
  tests give K 0.22–0.30, 1.6–2.2 times Hoerner's 0.135. Since three sit near the flat region, this is consistent
  with Hoerner's cube root rather than a test of it; Zhao's level is what the fin needs.
- **Hoerner's own figure agrees at the thin end.** By eye, his bluff and thick-edged sections at C_fB below about
  0.1 lie near 0.35–0.70, above the 0.135 curve; near C_fB 0.4 cut-off foils spread over about 0.13–0.22, and
  sheet-metal joints near 0.12 [H] p. 3-20, Fig. 39. He finds the two shapes alike [H] p. 3-21; these don't.
- **Why cut-off foils read lower (Inference).** A cut-off airfoil's afterbody converges toward the base. Slight
  positive boattailing raises the base pressure [N3] p. 6, and quickly even for moderate boattails [N2] p. 10.

## 4. Mach number: ×1.05–1.37 from 0.3 to 0.8, against the rule's ×1.54

Each set's base drag over its own value at Mach 0.3 (or 0.6, value in brackets, where it starts there), by eye:

| Test | Geometry | 0.6 | 0.8 | 0.9 | Source |
|------|----------|----:|----:|----:|--------|
| Nash, h/c 10% | parallel afterbody | 1.06 | 1.18–1.21 | 1.15 | [N1] p. 36, Fig. 13 |
| Morrow and Katz, h/c 6%, flight | slab | 1 (0.55) | 1.05 | 1.05 | [N5] p. 15, Fig. 6 |
| Cleary and Stevens, h/c 1.2% | thickened 4% arc | 1 (0.165) | 1.07 | 1.19 (rough 0.97) | [N4] p. 35, Fig. 14 |
| Same, h/c 2.4%, smooth / rough | thickened 4% arc | 1 (0.31 / 0.41) | 1.13 / 1.17 | 1.8 / 1.4 | [N4] p. 35 |
| Same, h/c 4%, smooth / rough | thickened 4% arc | 1 (0.56) | 1.29 / 1.16 | 1.41 / 1.25 | [N4] p. 35 |
| Rule, 0.12 + 0.13 M² | rocket base | 1.27 | 1.54 | 1.71 | Inference |

- **The sets.** Cleary and Stevens tested aspect-ratio-4 wings on a transonic bump at 1.7–2.2 × 10⁶, base
  pressure at 25% semispan [N4] pp. 1, 4, 35. Morrow and Katz's faired curve runs −0.55 at Mach 0.6, −0.575 at
  0.8 and −0.58 at 0.9, peaking at −0.83 near 0.98; its slits read the central base only [N5] pp. 5, 15. Nash
  summarized subsonic data as rising "slightly with Mach number" up to the critical Mach number [N2] pp. 4–5.
- **Combined (Inference).** From Mach 0.3 to 0.6, ×1.00–1.06; from 0.6 to 0.8, ×1.05–1.29. So ×1.05–1.37 from 0.3
  to 0.8, ×1.05–1.2 on the two parallel-sided sets. To 0.9, ×0.97–1.9: the thickened sections start their
  transonic rise near 0.85–0.9 [N4] p. 35, and thin sections climb steeply toward Mach 1 [N3] p. 7, p. 23, Fig. 5.
- **The rule's rise is too steep below Mach 0.85:** its ×1.54 tops every set at 0.8. Eggers' wedge rose ×1.5
  (fin-edge-drag §3), but a diverging wedge is not a parallel fin. In flight the fin's C_fB falls from 0.45 to
  0.37 by Mach 0.8, adding ×1.07 by the cube-root law (Inference); the tunnel ratios hold their own.

## 5. Why 0.10–0.19 and 0.25–0.39 differ

- **Mostly not Mach number.** The two ranges differ about ×2; Mach 0.3 to 0.8 supplies ×1.05–1.37 of it (§4).
- **Configuration lowers the low-speed set.** Hoerner's points near h/c 1.3–2% are cut-off NACA sections
  [H] p. 3-21, Fig. 40, with converging afterbodies; a parallel plate at the same C_fB reads 1.6–2.2 times more.
- **A thin laminar layer raises the transonic set.** Cleary's smooth wings were largely laminar [N4] p. 6, and
  Hoerner fits the Mach 0.7–0.9 thin-foil data only with mostly laminar friction [H] pp. 3-21, 3-22. Inference: a
  laminar θ/h near 0.02 gives Cleary's 2.4% section C_fB 0.08, and eq. 39 then 0.31, as measured at Mach 0.6.
- **Roughness is not a clean test.** It lowered the 2.4% section's base pressure (0.41 against 0.31 at Mach 0.6)
  and raised the 4% one's [N4] p. 35, pp. 8–9: tripping changes the layer's state as well as its thickness.
- **And h/c near 2% is steep ground.** In Nash's Fig. 5 at Mach 0.6–0.8, Cleary's 1.2% section reads about 0.14
  and Holder, Pearcey and Nash's 2.5% one 0.41–0.43 [N3] p. 23. 0.25–0.39 interpolates across that; his Fig. 4
  has no parallel-sided section near 2% (only at 6% and 10%) [N3] p. 22.
- **So the gap is settled in part.** A parallel fin with θ/h near 0.1 belongs to neither set: about 0.30 at low
  speed, between them, and about 0.35 at Mach 0.8, inside Nash's band.

## 6. The fin's trailing edge, Mach 0.3 to 0.9

Inference: K = 0.23 (0.22–0.30) over ∛0.43 = 0.754 at low speed, times §4's factors (central ×1.06, 1.15, 1.15).

| Mach | evidence, central | evidence, range | rule 0.12 + 0.13 M² | #614 run 2 | Hoerner eq. 39 alone |
|-----:|------------------:|----------------:|--------------------:|-----------:|---------------------:|
| 0.3 | 0.30 | 0.29–0.40 | 0.132 | 0.196 | 0.179 |
| 0.6 | 0.32 | 0.29–0.42 | 0.167 | 0.249 | 0.179 |
| 0.8 | 0.35 | 0.31–0.54 | 0.203 | 0.303 | 0.179 |
| 0.9 | 0.35 | 0.28–0.76 | 0.225 | 0.336 | 0.179 |

- **The fin's Mach factor (Inference):** about 1 + 0.3 M², or 1.08, 1.16 and 1.21 at Mach 0.6, 0.8 and 0.9 over
  0.3. The rule's is 1.27, 1.54 and 1.71; #614 takes it over zero (1.10, 1.39, 1.69, 1.88 from Mach 0.3 to 0.9).
- **Mach 0.9 is open above.** A thin fin may go locally sonic first at its leading-edge corners
  ([thin-edge-mach-dependence](thin-edge-mach-dependence.md) §3), and thin sections' base drag climbs steeply
  from about 0.9 [N3] p. 23. The upper value 0.76 is a bump test's, not a bound.
- **If the fin behaved like a cut-off foil,** the level would be Hoerner's 0.18; §3's tests say it does not.

## 7. What it does to #614's second run, and to the invented rocket

The invented rocket's fin frontal area over reference area is 0.127; its C_D is 0.449 at Mach 0.3 and 0.524 at 0.8
with square fins (fin-edge-drag §6). Apogee moves −0.31% to −0.57% per 1% of C_D ([apogee-bias](apogee-bias.md)).
All Inference:

| Mach | evidence | C_D added over the rule | C_D added over #614 run 2 |
|-----:|----------|------------------------:|--------------------------:|
| 0.3 | central | +0.022 (+4.9%) | +0.014 (+3.1%) |
| 0.3 | range | +0.020 to +0.034 (+4.5 to +7.5%) | +0.012 to +0.026 (+2.7 to +5.7%) |
| 0.8 | central | +0.019 (+3.6%) | +0.006 (+1.1%) |
| 0.8 | range | +0.013 to +0.043 (+2.5 to +8.3%) | 0.000 to +0.031 (+0.1 to +5.9%) |

- **Sign.** Both read the trailing edge low, so simulated apogee reads high: by 1.4–4.3% at Mach 0.3 from the
  rule's edge alone. Run 2 is close only near Mach 0.8–0.9, where the rule's steep rise offsets Hoerner's level.
- **This turns fin-edge-drag's trailing-edge sign.** With 0.29–0.40, its square-fin row (both edges, rule minus
  measured) at Mach 0.3 becomes −1.7% to +2.9% of C_D with its leading edge, or −5.9% to 0.0% with
  thin-edge-mach-dependence's; at Mach 0.8, −2.4% to +4.7% or −6.4% to +3.0%. It no longer reads clearly high.

## What this suggests

- **A trailing edge for parallel-sided square fins (core band, below Mach 0.85):** C_DB = 0.23 (1 + 0.3 M²)/
  (1.027 ∛C_fB), C_fB = 2 C_f c̄/t, for θ/h of about 0.01–0.17. On the fin: 0.30, 0.33, 0.35 and 0.37 at Mach
  0.3, 0.6, 0.8 and 0.9. Expected effect on the invented rocket: +0.022 in C_D at Mach 0.3 over the rule.
- **For #614's second run,** use K = 0.23 with 1 + 0.3 M² in place of 0.135 with the rule's rise, and bracket it
  with K = 0.30 and Hoerner's 0.135. The spread between them is the trailing edge's doubt.
- **Rounded trailing edges:** keep "half of square" flagged as unsupported (§2).
- **Ship it with the leading edge** (fin-edge-drag §2, thin-edge-mach-dependence §2), which moves the other way:
  the net square-fin change at Mach 0.3 is within about ±2% of C_D at the two notes' central values.

## What the project can use

- **US government works, cached** under `refs/research/blunt-te-boundary-layer/dl/` (NTRS: public use permitted):
  Cleary and Stevens RM A51J11, Morrow and Katz TN 3548, Dugan RM A52E01.
- **Cite and read only:** Nash's R&M 3323, 3427 and 3468 (Crown copyright, cached under fin-edge-drag); Hoerner
  (copyright L. A. Hoerner); Zhao's and Doddipatla's theses (copyright their authors; university repositories);
  the Kellaris et al. preprint (arXiv non-exclusive license). No data table is copied; values are readings.

## Open questions

- Zhao's plate is the only parallel-sided test found at θ/h near 0.1. Rowe, Fry and Motallebi (AIAA J. 39(4),
  2001) and Durgesh et al. (AIAA J., 2013) measured base pressure against layer thickness; both are paywalled,
  and Tanner's base-drag papers (Aeronautical Quarterly 1972; Prog. Aerospace Sci. 1975) were seen as abstracts.
- No compressible test of a parallel plate with a thick turbulent layer was found; this note assumes the layer's
  effect and Mach number's multiply. Motallebi and Norbury (J. Fluid Mech. 110, 1981), compressible, is paywalled.
- Dugan's 0°, 2.02° and 6.88° boattails at Mach 0.6–0.925 [N6] p. 28, Fig. 8: scanned lines can't be told apart.
- A real fin's paint, roughness and partly laminar layer change θ/h; its root runs in the body's boundary layer.

## Sources

All read 2026-10-19. Printed page numbers.

1. **N1.** J. F. Nash, V. G. Quincey and J. Callinan, *Experiments on Two-Dimensional Base Flow at Subsonic and
   Transonic Speeds*, ARC R&M 3427, 1963. <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4011>
2. **N2.** J. F. Nash, *A Discussion of Two-Dimensional Turbulent Base Flows*, ARC R&M 3468, 1965.
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4047>
3. **N3.** J. F. Nash, *A Review of Research on Two-Dimensional Base Flow*, ARC R&M 3323, 1962.
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/3903>
4. **N4.** J. W. Cleary and G. L. Stevens, *The Effects at Transonic Speeds of Thickening the Trailing Edge of a
   Wing with a 4-Percent-Thick Circular-Arc Airfoil*, NACA RM A51J11. <https://ntrs.nasa.gov/citations/19930087063>
5. **N5.** J. D. Morrow and E. Katz, *Flight Investigation at Mach Numbers from 0.6 to 1.7 to Determine Drag and
   Base Pressures on a Blunt-Trailing-Edge Airfoil …*, NACA TN 3548, 1955. <https://ntrs.nasa.gov/citations/19930084307>
6. **N6.** D. W. Dugan, *Effects of Three Types of Blunt Trailing Edges on the Aerodynamic Characteristics of a
   Plane Tapered Wing of Aspect Ratio 3.1 …*, NACA RM A52E01, 1952. <https://ntrs.nasa.gov/citations/19930087008>
7. **H.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
8. **Z.** W. Zhao, *Effect of Boundary Layer Thickness and Passive Vortex Generators on the Wake of a Blunt Trailing
   Edge Profiled Body*, MASc thesis, University of Toronto, 2017.
   <https://utoronto.scholaris.ca/bitstreams/16134374-65b9-4a59-a24e-0e007031b023/download>
9. **D.** L. S. Doddipatla, *Wake Dynamics and Passive Flow Control of a Blunt Trailing Edge Profiled Body*, PhD
   thesis, University of Western Ontario, 2010. <https://ir.lib.uwo.ca/etd/71>
10. **K.** K. Kellaris, G. Papadakis, M. A. Mendez and M. Manolesos, *On the low drag regime of flatback airfoils*,
    arXiv:2410.10359, 2024. <https://arxiv.org/abs/2410.10359>
