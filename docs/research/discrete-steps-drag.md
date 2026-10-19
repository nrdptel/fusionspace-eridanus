# How much drag spiral seams, paint edges, decals and fin slots add

[Excrescence drag](excrescence-drag.md) sized joint gaps, tube mismatches, switch bands, ports and heads, and left
spiral seams, paint edges and decals to this note; [hobby finish roughness](hobby-finish-roughness.md) found that a seam
is a discrete step, not sand grain. **In short: on the same invented 98 mm and 54 mm L2 rockets at 300 m/s, an unfilled
spiral-wound tube's seam, a two-tone paint band, vinyl decals and open fin-slot gaps add about 0.3% of C_D (98 mm) and
0.8% (54 mm).** The range runs from near zero (filled or filament-wound tube, waterslide decals) to 2.8% and 7.2% if the
seam is a groove 0.25 mm deep and 1 mm wide, and to 16% on 54 mm from the seam alone if a ply edge stands proud along
it. The sign is one-way: more drag, so the codes fly high, by 0.1–0.5% of apogee typically (1–5% of the +8.73% bias).
The seam is the only feature here that can matter: a helix wound 53–76° from the axis lays 1.3–4.0 m of nearly
transverse seam across each meter of tube, whatever the diameter. Its depth and width have never been measured.
Paint (25–150 µm) and vinyl (93 µm) edges cost 0.01–0.37% of C_D per full circumferential edge pair; waterslide
decals (13–14 µm) sit in the viscous sublayer. A second law also changes the existing note: tunnel data scaled in
wall units put forward steps of 0.2–1 mm at 1.4–1.8 times Hoerner's law at 300 m/s, lifting excrescence-drag's clean
total from about 2.0% to about 2.5% of C_D (98 mm). How far to trust it: the step and groove laws are measured on flat
tunnel walls; every hobby size is invented; the seam's size is a guess. "Inference" marks this note's own arithmetic.

## 1. Two laws for a step much smaller than the layer

- **Hoerner's outer law (correlation).** A forward step scores 0.30 (h/δ)^(1/3) on h times the span and free-stream q, a
  backward step 0.18 (h/δ)^(1/3) ([excrescence drag](excrescence-drag.md) §2, from [1] pp. 3-21, 5-6, 5-7). The forward
  value rests on Wieghardt's 1943 tests [1] p. 5-7 ref. 12a; the backward one on Hoerner's base drag, p. 3-21.
- **The inner law (measured).** When h is a few percent of δ, the drag depends only on wall variables: D/(ρ u_τ² h) =
  C_D/c_f = F(shape, h+), with h+ = h u_τ/ν, C_D on h times the span and free-stream q, and c_f the clean local skin
  friction [3] pp. 14–15. F follows a logarithm, C_D/c_f = A log10 h+ + B, with A and B set by the shape [3] p. 15.
- **Forward steps.** RAE force-balance tests of 1.27, 2.54 and 3.81 mm steps up to 5% of δ, Mach 0.2 to 2.4, plotted
  in this form with Wieghardt's points, are reproduced in [2] slide 6. Read from that figure (±5 in C_D/c_f,
  **Inference**): subsonic, C_D/c_f ≈ 59.6 log10 h+ − 69 at Mach 0.8 and − 78 at Mach 0.2, over h+ 53–1,800;
  supersonic, one line, ≈ 77.8 log10 h+ − 58, over h+ 13–1,300. Wieghardt's points fall below the Mach 0.2 line.
- **Backward steps.** A least-squares fit over Wieghardt's, Gaudet and Johnson's and his own data gives C_D/c_f =
  15 log10 h+ − 8, within ±10–15%, in zero and in adverse or favorable pressure gradients [3] p. 101; small steps
  covered h+ 150–900, large ones up to 3,500 [3] abstract.
- **Wall units on the rockets (Inference).** Local c_f = 0.036 Re_x^(−1/6) [1] p. 2-5 eq. 28 and u_τ = V (c_f/2)^(1/2),
  ν = 1.5 × 10^−5 m²/s, x from the nose tip. Each 0.1 mm of height is h+ 34–39 at 150 m/s and 64–74 at 300 m/s, for x
  from 1.6 m to 0.27 m. Hobby steps of 0.05–1 mm span h+ 17–740, inside or just below the measured range.
- **Inner against outer (Inference, x = 1 m).** At 300 m/s the forward-step law gives 1.14–1.50 times Hoerner's at
  0.1 mm, 1.45–1.73 at 0.2 mm, 1.59–1.80 at 0.5 mm and 1.57–1.74 at 1 mm (Mach 0.2 to 0.8 lines); at 150 m/s, 0.61–1.02,
  1.11–1.43, 1.42–1.66 and 1.49–1.69. Backward steps agree within 0–22%. The inner law grows with speed, since h+ ∝ V;
  Hoerner's barely moves (δ ∝ V^(−1/7)). Faster than sound, F is 1.85–2.07 times the Mach 0.8 line at 0.2 mm, before
  c_f's fall with Mach: in line with Hopko's 1.5–2.7 times in excrescence-drag §2.
- **What it changes there (Inference, 300 m/s).** On the 98 mm rocket, the three forward mismatches (0.39% of C_D)
  become 0.57–0.67% and the proud band (0.71%) 0.91–0.99%; the clean total 2.0% becomes about 2.4–2.6%. On 54 mm, 4.4%
  becomes about 5.1–5.5%. At 150 m/s the rise is about half that.

## 2. Spiral seams: a helix of grooves or steps

- **The winding angle.** A butt-wound ply of width W on diameter D lies at α from the axis with W = πD cos α
  (**Inference**: all nine textile cores of [6] Table 1, 75–143 mm inside diameter with 4 and 5 in plies at 57.4–73.6°,
  fit it within 0.2° on the mm column). Earlier high-speed cores stayed under 74°; the patent's own ran 73.2–80° [6].
  Hobby makers state no ply width; 3–5 in plies on 98 mm give 66–76°, 2–4 in on 54 mm give 53–73° (**Inference**, ply
  widths assumed).
- **How much seam crosses the flow (Inference).** Along any line down the body the seam recurs every pitch p = πD/tan α,
  so each meter of tube carries tan α meters of seam projected across the flow: 1.3 at 53°, 2.1 at 65°, 4.0 at 76°. That
  is independent of D, so its share of C_D goes as tube length over D²: about three times on 54 mm as on 98 mm.
- **The angle barely helps.** Pressure drag on a strip at β to the flow goes roughly as sin²β; across the flow it is
  about 60 times that along it [1] p. 5-9. Per projected span that is sin β, 0.80–0.97 at 53–76° (**Inference**).
  ESDU's public summary of item 75028 says grooves inclined more than 60° to the flow take the data for grooves normal
  to it [5]. So a spiral seam acts as a transverse seam within 20%; the helix angle matters through the length, not the
  sweep.
- **A butt seam is a groove.** Deep transverse grooves (depth over width above 0.7) score 0.014 on width times span at
  q_eff taken at the width; shallow ones (width over depth above 7) 0.29 on depth times span at q_eff at the depth [1]
  p. 5-10 figs. 20, 21. A groove twice as wide as deep, across the flow, scores 0.01 against 1.3 for a square strip [1]
  p. 5-9 fig. 18. This note interpolates between the two laws in log of depth over width (**Inference**).
- **Sizes (invented).** An outer ply is 0.1–0.25 mm thick ([hobby finish roughness](hobby-finish-roughness.md) §3), so a
  seam groove is at most that deep. Assumed: 0.1 mm deep (0.05–0.25), 0.3 mm wide (0.1–1.0), α 65° (53–76°), sin β 0.9
  (0.8–1.0), over the tube from the nose shoulder to the tail. No hobby seam measurement was found.
- **Results (Inference, 300 m/s).** 98 mm: 0.14% of C_D (0.01–1.5); 54 mm: 0.40% (0.03–4.3). At 150 m/s within 4%.
- **A proud ply edge is a step.** If the outer ply overlaps or an edge lifts, each crossing is a forward or a backward
  step of ply height, all facing one way. At 0.1 mm: 0.45–0.68% of C_D facing forward, 0.27–0.33% facing back (98 mm);
  1.3–1.9% and 0.78–0.92% (54 mm), the spread being the two laws. Bound, 0.25 mm proud at 76°: 5.6% and 15.7%; low,
  0.05 mm at 53°: 0.06% and 0.17% (**Inference**).
- **Removing it.** Filling the spirals, a filament-wound glass tube, or glass cloth over kraft leaves no seam
  (**Inference**). DeMar's unfinished kraft-and-balsa model read C_D 0.77 at 120 km/h against 0.61 filled, polished
  and waxed [9] p. 7, but that mixes seams, grain and paint.

## 3. Paint and decal edges

- **Heights.** Aerosol enamel goes on at 1.0–2.0 mils (25–50 µm) dry per coat [8], so a masked stripe of 1–3 coats
  stands 25–150 µm. Cut vinyl is a 0.07 mm film on 0.023 mm of adhesive, 93 µm in all [7]. One waterslide decal paper
  is listed at 13–14 µm [10]. Printed vinyl under a laminate is thicker; no datasheet for it was read.
- **In wall units (Inference, x = 1 m).** 75 µm is h+ 26 at 150 m/s and 50 at 300 m/s; 93 µm, 33 and 61; 150 µm, 53 and
  99; 14 µm, 5 and 9. So paint and vinyl edges sit near the smallest subsonic forward steps measured (h+ 53), 75 µm and
  below under them; the fitted lines reach zero near h+ 14–20. A 14 µm decal edge lies in the sublayer or low buffer
  layer: nil.
- **Per edge (Inference, 300 m/s).** One full circumferential front-and-back edge pair costs, on 98 mm: 0.05–0.07% of
  C_D at 75 µm, 0.13–0.19% at 150 µm, 0.07–0.09% for vinyl; on 54 mm: 0.10–0.13%, 0.26–0.37% and 0.14–0.18%. Ranges
  span the inner and outer laws. At 150 m/s the inner law gives 0.56–0.80 as much (25 µm: 0.5–0.6).
- **Edges along the flow** count little (sin²β above), so only the decal's edge span across the flow matters.
- **A clear coat that rounds an edge** helps: beveling a lap at 60° cuts its drag to about 1/4, rounding to 1/10 [1]
  p. 5-7 fig. 10 (0.40, 0.11, 0.04). Whether a sprayed clear coat rounds a 93 µm edge that much is not measured.

## 4. Fin-tab slots and root gaps

- **An open slot beside a through-the-wall fin** is a groove along the flow, the length of the root and 0.5–1 mm wide.
  Grooves along the flow score 0.009 with a blunt downstream end and 0.001 with ramped ends, on wetted area [1] p. 5-9
  fig. 18. This note takes the opening's plan area; counting the walls would raise it (**Inference**).
- **Size (Inference).** Four fins with 200 mm roots on 98 mm, three with 150 mm roots on 54 mm, a gap each side:
  0.02–0.38% and 0.04–0.71% of C_D for 0.5–1 mm gaps. A fillet closes the gap: zero, the usual case. A slot venting
  into a fin can or motor tube may pass air; no source bounds that ([excrescence drag](excrescence-drag.md) §2, holes).

## 5. Many small steps: when they stop adding

- **Spacing over height decides.** Square elements show skimming flow at spacing s/h about 2 or less, wake
  interference near 4, and isolated elements from about 12 (Liu and Kline, as cited in [4] p. 6). A ridge's downstream
  bubble is 8.5 h long in zero gradient [4] p. 96. Two ridges interact most when spaced one bubble apart, and
  negligibly beyond about 24 h (Bradshaw and Wong, as cited in [4] p. 113).
- **On a rocket (Inference).** Seam crossings recur every 50–140 mm against heights under 0.25 mm: s/h above 200,
  fully isolated, so they add. Vinyl letters 2–5 mm apart at 93 µm reach s/h 21–54: isolated or nearly, so adding them
  overstates the tightest lettering slightly. FDM layer lines, 0.1–0.3 mm apart, are skimming or wake-interference
  roughness: they belong in k_s ([hobby finish roughness](hobby-finish-roughness.md) §3), not here.
- **Each step also thickens the layer** a little for the next; that second-order effect is not bounded here.

## 6. On the invented rockets

Rockets as in [excrescence drag](excrescence-drag.md) §4: 98 mm, 2.0 m, 7,543 mm² reference; 54 mm, 1.5 m, 2,290 mm².
Tube from the nose shoulder (0.45 m, 0.27 m) to the tail; edges at mid-body (1.0 m, 0.75 m); whole-rocket C_D 0.5;
300 m/s. Typical and range, % of C_D (**Inference** throughout).

| feature | 98 mm | 54 mm |
|---|---|---|
| spiral seam, butt groove (filled or wound glass: 0) | 0.14 (0.01–1.5) | 0.40 (0.03–4.3) |
| two-tone paint, 1 band (0–3), 75 µm (25–150) | 0.06 (0–0.58) | 0.12 (0–1.12) |
| vinyl decals, 0.3 m of edge each way (0.1–1.0) | 0.08 (0.02–0.31) | 0.28 (0.08–1.07) |
| waterslide decals | under 0.01 | under 0.01 |
| fin-slot gaps (filleted: 0) | 0 (0–0.38) | 0 (0–0.71) |
| **total** | **0.3 (0.03–2.8)** | **0.8 (0.11–7.2)** |
| seam with a proud ply edge instead of a groove | 0.27–0.68 (0.06–5.6) | 0.78–1.9 (0.17–15.7) |

- **In apogee,** at −0.31% to −0.57% of apogee per 1% of drag ([apogee bias](apogee-bias.md), sensitivity table): −0.09%
  to −0.17% (98 mm) and −0.25% to −0.46% (54 mm), 1.1–2.0% and 2.8–5.2% of the +8.73% bias. The groove bound gives
  −0.9% to −1.6% and −2.2% to −4.1%; a proud seam on 54 mm, up to −4.9% to −8.9%, the whole bias.
- **With excrescence-drag's revised clean airframe** (§1), a kraft-tube rocket's small features total about 2.7–2.9%
  of C_D (98 mm) and 5.9–6.3% (54 mm): −0.8% to −1.7% and −1.8% to −3.6% of apogee, one-signed.
- **Neither HPR Sim nor OpenRocket has a term for any of these** ([excrescence drag](excrescence-drag.md) §5); a finish
  setting is sand-grain k_s, which §5 above keeps separate.

## What the project can use

- **Laws, cite only.** Hoerner [1] is copyrighted. The inner-law forward-step lines are this note's reading of a figure
  reproduced in a 2017 symposium deck [2] (no license stated; cached under `refs/research/discrete-steps-drag/`, not
  committed). Higazy's and Abd Rabbo's theses [3][4] are marked all rights reserved; cached locally, cite only.
- **ESDU 75028** is proprietary; only its public summary was read [5] (75031's summary, read late, tentatively extends
  the method to inclined steps and ridges). The patent [6] is a public record. Maker datasheets [7][8] and the listing
  [10] are cited, not copied.

## What this suggests

- **Use the inner law for steps under the layer.** For a stated step height: forward C_D/c_f ≈ 59.6 log10 h+ − 69 to
  − 78 (subsonic), backward 15 log10 h+ − 8, clamped at zero, with c_f and u_τ from the local flat-plate layer. Expected
  effect: excrescence-drag's step rows rise 1.1–1.8 times at 150–300 m/s; apogee about 0.1–0.3% lower on 98 mm.
- **A "spiral-wound, unfilled" tube option:** a groove term of tan α meters of span per meter of tube, depth and width
  stated. Expected effect: −0.04% (98 mm) to −0.23% (54 mm) of apogee typical, up to about −2.5% at the bound.
- **Skip paint and decal edges** unless a user asks: each edge pair is 0.02–0.2% of apogee.
- **The cheapest measurement with the largest payoff:** a feeler-gauge or profilometer survey of seam depth and width on
  a few kraft, phenolic and glassine-wrapped hobby tubes, and their ply widths. It would collapse the 0.01–4.3% range.

## Open questions

- **Seam depth, width and edge state** on real hobby tubes, and the ply widths makers wind: no source found.
- **The originals.** Gaudet and Johnson, RAE TR 70190 (1970), and Gaudet and Winter, AGARD CP-124 paper 4 (1973) ([3] p.
  191), hold the groove and ridge lines too ([3] p. 14, crediting the former). The deck credits its figure to the latter
  under the former's title [2]. DTIC showed a maintenance page on 2026-10-18, as it had for AGARDograph 264
  ([excrescence drag](excrescence-drag.md), open questions).
- **Below h+ 50,** where paint edges of 75 µm and less sit, no subsonic forward-step drag was found measured.
- **Transonic edges and seams (Mach 0.8–1.2),** a vented fin slot, and printed laminated vinyl's thickness: not bounded.

## Sources

All read 2026-10-18; page numbers are the documents' printed ones.

1. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, pp. 2-5 (eq. 28), 5-6, 5-7 (fig. 10), 5-9 (fig. 18, oblique flow), 5-10
   (figs. 20, 21). Copyrighted; cite only. <https://archive.org/details/FluidDynamicDragHoerner1965>
2. A. Shires, B. Hinchliffe, K. Hackett, D. Philpott, "Fuel-burn penalties caused by irregularities in aircraft
   surfaces (ASTEROID project)", Airbus DiPaRT symposium, 2017-11-22, slide 6.
   <https://pdfs.semanticscholar.org/1eec/38b9aebad88c54855b294bb9cc0c04d0d457.pdf>
3. M. G. A. F. I. Higazy, *Influence of isolated backward-facing steps on boundary layers*, PhD thesis, University of
   Leicester, 1986: abstract, pp. 14–15 (eqs. 2.2, 2.3; p. 14, shared form), 101 (eq. 5.8), 191 (refs. 7, 8).
   <https://hdl.handle.net/2381/34778>
4. M. F. Abd Rabbo, *Aerodynamic drag of ridge arrays in adverse pressure gradients*, PhD thesis, University of
   Leicester, 1976, pp. 6, 96, 113. <https://hdl.handle.net/2381/34734>
5. ESDU 75028, public summary (grooves; inclined grooves). <https://beta.esdu.com/toolbox/ui/tba_75028>
6. US 5,472,154, "High spiral angle winding cores", Table 1 and text. <https://patents.google.com/patent/US5472154>
7. ORAFOL, ORACAL 651 datasheet, technical data: film 0.07 mm, adhesive 0.023 mm.
   <https://www.orafol.com/product/film/full/463.datasheet?locale=en>
8. Rust-Oleum, Painter's Touch 2X Gloss technical data, form GDH-1268, rev. 013019, p. 3 (dry film per coat).
   <https://zoro.com/static/cms/enhanced_pdf/RUST%20OLEUM%20Corp_PT2XxxGlossxxTDS.pdf>
9. J. S. DeMar, *Model rocket drag analysis*, NAR R&D report, 1995, p. 7.
   <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
10. Green Stuff World waterslide decals, inkjet white, retailer listing (specifications: 13–14 µm).
    <https://www.frontlinehobbies.com.au/green-stuff-world-waterslide-decals-inkjet-white>
