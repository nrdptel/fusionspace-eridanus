# Step, gap, hole and head drag from Mach 0.8 to 1.2

[Excrescence drag](excrescence-drag.md) and [discrete steps](discrete-steps-drag.md) sized the small features of an
airframe at 150–300 m/s and estimated their drag faster than sound from flights at Mach 1.25–2. **In short: measured
transonic data put these features at about 1.2–1.3 times their 300 m/s drag at Mach 1.0 and 1.6–1.9 times at Mach 1.1
(range 1.2–2.2), which confirms the earlier estimate of 3–4% of C_D (98 mm) and 6–9% (54 mm) at Mach 1.1.** Forward
steps rise 1.4–1.6 times by Mach 1.1, rearward steps 1.6–3.4 times; grooves and ports barely change; heads are
unmeasured between Mach 0.8 and 1.4. The rocket's own drag rises 1.36–1.51 times (flat base) or 1.7–1.9 (boattail), so
at Mach 1.1 the features are 2.1–2.8% of C_D on the invented 98 mm L2 rocket and 4.7–6.3% on the 54 mm one with a flat
base, against 2.0% and 4.4% subsonic: apogee a further 0–0.5% and 0.1–1.1% lower, up to 5% and 12% of the +8.73% bias.
RAE's wall-variable data reach down to HPR heights (h+ 57–320). How far to trust it: the transonic Mach shape was
measured at one height per step, near h+ 1,200 for forward steps, and the rearward factor's sources disagree by 2 times.
"Inference" marks this note's own readings and arithmetic.

## 1. What the RAE balance tests measured, and where they stop

- **Gaudet and Winter [T1]** mounted steps, ridges, grooves, holes, cylinders and fairings on strain-gauge balances in
  the walls of the RAE 8 ft × 8 ft tunnel, from Mach 0.2 to 2.8 but not transonic [T1] summary, p. 3. The fits are
  given at Mach 0.2, 0.8, 1.4, 2.2 and 2.8 [T1] p. 9; forward steps were also run at 1.7 and 2.4, rearward at 1.7 and
  2.0 [T1] p. 7, fig. 6. Nothing lies between Mach 0.8 and 1.4.
- **The wall-variable law (measured).** For steps small against δ, C_D/c_f on h times the span is linear in log h+,
  and the step tops sat in the log region in every test [T1] p. 6. Forward steps rise with Mach to about 1.4, then
  hold; rearward steps peak at about 1.4 and fall [T1] pp. 6–7. u_τ and ν in h+ are taken at the wall [T1] p. 6.
- **The fits.** C_D/c_f = C log10 h+ + D with, for forward steps, C 60 and D −70 at Mach 0.8, C 80 and D −65 at 1.4;
  for rearward steps, C 20 and D −13 at 0.8, C 48 and D −18 at 1.4 [T1] p. 9. At h+ 1,200 these give 115 and 181,
  where the AGARDograph plots RAE's points at 113 and 180, so the log is base 10 (**Inference**, [T2] fig. 4.24). The
  report's own constants confirm the line discrete-steps-drag read from an Airbus slide (59.6 and −69; 77.8 and −58).
- **Grooves across the flow.** On plan area, C_D/c_f ≈ 2 log10(u_τ l/ν) − 2 for 10 < u_τ l/ν < 1,000, at all five
  fit Mach numbers, widths 1–3 times the depth [T1] p. 9: no Mach effect.
- **Short cylinders** drag about as shallow holes do, both estimated from forward and rearward step faces with a sweep
  factor [T1] pp. 10, 13; this note treats screw and rivet heads that way.
- **The range of h+ (Inference, read ±10%).** Forward steps: from h+ about 55 subsonic and 13 supersonic to about
  1,500 [T2] p. 78 fig. 4.23, [T1] fig. 6. The lowest supersonic points are at Mach 2.2–2.4; the Mach 1.4 points start
  near h+ 80, and using the low ones at 1.4 rests on [T1] p. 6's statement that the parameter does not change with
  Mach above 1.4. Rearward steps: about h+ 80–1,200 at Mach 0.8 and 55–1,000 at 1.4 [T2] p. 83 fig. 4.28.

## 2. Pallister's transonic tunnel tests, as the AGARDograph reports them

- **The gap filled, narrowly.** Pallister at the Aircraft Research Association repeated RAE's method from Mach 0.6 to
  1.4, at a narrow range of h+ [T2] p. 78, crediting ARA Report 37 (1974) [T2] p. 104. That report was not reached.
- **Forward steps (measured).** Corrected to h+ = 1,200, Pallister's points read (±3) 92 at Mach 0.5 (a point below
  the stated range), 104 at 0.8, 110 at 0.9, 114 at 0.95, 124 at 1.0, 144 at 1.05, 160 at 1.1, 175 at 1.2, 191 at 1.3
  and 201 at 1.4 [T2] p. 79 fig. 4.24 (**Inference**, read). Most of the rise lies between Mach 1.0 and 1.2; his 1.4
  point is 12% above RAE's.
- **The interpolation.** The AGARDograph sets C_D/c_f = L(0.8) + PC1 × (L(1.4) − L(0.8)), L the RAE fits [T2] p. 80
  eq. 4(27). Read from fig. 4.25 (±0.02), PC1 is 0.05 at Mach 0.9, 0.09 at 0.95, 0.20 at 1.0, 0.40 at 1.05, 0.57 at
  1.1, 0.72 at 1.2 and 0.87 at 1.3 (**Inference**).
- **Rearward steps (measured).** Three heights, 1.27, 2.54 and 3.81 mm (h/δ 0.01–0.03), at nearly one unit Reynolds
  number, gave three Mach shapes [T2] pp. 83–84 fig. 4.29. Each peaks near Mach 1.15–1.2, at about 1.9, 2.7 and 3.0
  times its Mach 0.8 value (read ±5 in C_D/c_f, **Inference**). Pallister discredited his 1.27 mm results [T2] p. 82,
  and the AGARDograph, noting that he discounts "the smaller steps", takes the 3.81 mm shape [T2] p. 83. Its factor
  PC2, read from fig. 4.30 (±0.02): 0.08 at Mach 0.9, 0.11 at 0.95, 0.40 at 1.0, 1.08 at 1.05, 1.21 at 1.1, 1.27 at
  1.15, 1.25 at 1.2 and 1.14 at 1.3 (**Inference**). It overshoots the Mach 1.4 value by 27%.
- **Holes.** C_D/c_f = A (u_τ d/ν)^B, A set by depth over diameter and Mach, B by Mach [T2] p. 73 eq. 4(22). B reads
  about 0.3 to Mach 0.8 and 0.35 at 1.4 (fig. 4.18, ±0.03); A reads about 0.6 at Mach 0.8 and 0.3–0.4 at 1.4 for
  depths 0.1–0.35 d (p. 74 fig. 4.19): a 4 mm port (u_τ d/ν about 2,700) at Mach 1.4 is 0.8–1.0 times Mach 0.8.
- **Heads: nothing transonic.** The AGARDograph's round-head rivet data (h/d 0.2) are at Mach 0.2 and 0.8 only [T2]
  p. 62 fig. 4.6a. ESDU 76008's public summary says heads of h/d 0.2 were tested at Mach 0.8, 1.4 and 2.2 only [T5].
- **Antennas.** A tall cylinder's C_D, Reynolds number taken out, dips near Mach 0.86 and peaks near 1.0 at 1.6–1.8
  times its Mach 0.8 value [T2] p. 68 fig. 4.13 (**Inference**, read ±0.05).

## 3. A body of revolution through Mach 1 (NASA tunnel)

- **The model.** A 50 in (1.27 m) ogive cylinder, 4.096 in (104 mm) across, 3-caliber nose, fineness ratio 12.2,
  with roughness built into the cylinder, in the Langley 8-foot transonic pressure tunnel at Mach 0.70, 0.90, 1.00,
  1.10 and 1.20, 2.6–19.7 × 10^6 per m, tripped turbulent near the tip [T3] pp. 1, 4–6. Its boundary layer grows from
  about 5 to 19 mm along the cylinder [T3] fig. 4 (p. 22): an HPR airframe's layer (excrescence-drag §1).
- **Steps with grooves.** Nine cycles, 2.0 in apart, each a 0.021 in (0.53 mm) step with a groove at its joint [T3]
  table I (p. 13). Fig. 3(a) (p. 17) draws the joint's axial gap as 0.021 in, beside a 0.069 in (1.75 mm) radial
  dimension; the drawing is ambiguous. The report does not say which way the steps face. h/δ is 0.03–0.1
  (**Inference**).
- **Readings (fig. 10(a), p. 40, ±0.00003, on wetted area; Inference).** At 1,000 psf stagnation pressure: 0.00041
  at Mach 0.7 and 0.9, 0.00072 at 1.0, 0.00080 at 1.1 and 1.2, 0.00071 at 1.6. At 3,000 psf: 0.00051, 0.00072,
  0.00091, 0.00131 and 0.00120 from Mach 0.7 to 1.2; 0.00101 at 1.6 and 0.00082 at 2.0. The peak is at Mach 1.1, 1.95
  and 2.6 times the Mach 0.7 value; 1.95 and 1.8 times the Mach 0.9 value.
- **Per step on its own frontal area (Inference).** Wetted area 592 in², nine step faces π d h = 2.43 in², ratio
  0.0041. The report's theory line, 2/(γM²) times that ratio [T3] fig. 10(a), reads 0.0041 at Mach 1.2 on the plot,
  which checks it. So each step, its groove included, scores 0.10–0.12 at Mach 0.7, 0.10–0.18 at 0.9, 0.18–0.22 at
  1.0, 0.20–0.32 at 1.1, 0.20–0.29 at 1.2 and 0.17–0.25 at 1.6, the lower value at 1,000 psf.
- **Against RAE (Inference, h+ 350, c_f 0.0023).** The forward fit gives 0.19 at Mach 0.8 and 0.32 at 1.4, the
  rearward fit 0.09 and 0.24; this body gives 0.12 at 0.7 and 0.25 at 1.6, between the two and the same order. The
  AGARDograph plots the same models' earlier supersonic tests (NACA TN 4299) with the forward steps [T2] pp. 78–79, and
  RAE discusses them there too, expecting step-to-step interference to lower them [T1] p. 7. Read as forward steps,
  their rise to Mach 1.1 (1.8–2.0 times Mach 0.9) is above PC1's 1.5–1.8.
- **What else the reports say.** The increments rose with Reynolds number, at a slowing rate, and cluster on a
  subsonic and a transonic-supersonic curve [T3] p. 8; they were mainly pressure drag [T3] p. 9. Reflected nose shocks
  at Mach 1.1–1.2 differ from free flight, not enough to change the trends [T3] pp. 6–7. Pressure drag rose earlier
  and higher at the forward stations [T4] p. 1, put down to their smaller δ/h [T4] p. 10.

## 4. Does the wall-variable law hold at HPR heights?

- **HPR steps in wall units (Inference;** discrete-steps-drag's c_f, 1 m from the tip, sea level**).** On free-stream
  ν = 1.5 × 10^−5 m²/s, each 0.1 mm of height is h+ 66 at 300 m/s, 74 at Mach 1.0 and 81 at Mach 1.1. On wall ν, as
  RAE defines it (adiabatic wall, recovery 0.89, μ ∝ T^0.7), these are about 0.86 and 0.79 times as large: 0.1 mm is
  h+ about 57, and 0.1–0.5 mm steps sit at h+ 57–320. At 3 km, h+ is about 20% lower (ν 1.86 × 10^−5 m²/s, sound
  speed 329 m/s). The table's factors change by 1% or less between the two definitions.
- **Subsonic and supersonic: inside the data, at its lower edge.** Forward steps of 0.1–1 mm lie inside RAE's range,
  0.1 mm at or just under the subsonic edge (h+ 55); rearward steps of 0.1–0.2 mm (h+ 57–130) at or below the
  rearward data (from h+ 80).
- **The transonic shape: assumed, and the sources disagree on its h+ trend.** Pallister's steps sit at h+ about 1,200
  (forward) and one Reynolds number each (rearward) [T2] pp. 78, 83, 4–21 times HPR's h+. The RAE fits alone give a
  larger factor at lower h+: the Mach 1.4 line is steeper, and L(1.4)/L(0.8) for forward steps is 1.58 at h+ 1,200, 1.68
  at 350 and 1.90 at 100, 2.13 at 57 (**Inference**). Below h+ 55 that rests on the Mach 0.8 line extrapolated past its
  data. Every measured trend runs the other way: the NASA body rises 1.95 times at h+ about 130 (1,000 psf) and 2.6
  times at 350–380 (3,000 psf; §3, **Inference**, h+ from unit Reynolds number in proportion to stagnation pressure,
  about 6.6 and 20 × 10^6 per m); Pallister's smaller rearward steps rose less (§2); TN D-3519 found the larger rise
  where δ/h was smaller (§3). So neither direction is settled; the table applies PC1 and PC2 to the fits at HPR h+,
  carrying only the fits' trend.
- **A check on Hopko's flights (Inference).** The forward factor at Mach 1.2 (1.6–1.8) times discrete-steps-drag's
  inner-over-outer ratio at 300 m/s (1.45–1.8) is 2.3–3.2 times Hoerner's low-speed law; Hopko's laps read 2.7 times
  at Mach 1.25 (excrescence-drag §2). The 2–3 times that excrescence-drag assumed was on this base, and it holds.

## 5. Transonic factors on the invented rockets (Inference throughout)

Rockets and rows as in excrescence-drag §4. Each factor is C_D/c_f over its 300 m/s (Mach 0.88) value, taken on the
Mach 0.8 lines. c_f falls 6.5–7% from Mach 0.7 to 1.2 ([T3] fig. 6, p. 24): totals carry 0.95 (Mach 1.0), 0.93 (1.1).

| feature | Mach 0.9 | Mach 1.0 | Mach 1.1 | Mach 1.2 |
|---|---|---|---|---|
| forward step, 0.2 mm (PC1) | 1.05 | 1.22 | 1.58 | 1.76 |
| forward step, 0.5 mm (PC1) | 1.04 | 1.18 | 1.47 | 1.60 |
| rearward step, 0.2–0.5 mm (PC2, 3.81 mm shape) | 1.14 | 1.74–1.79 | 3.25–3.40 | 3.38–3.55 |
| rearward step, Pallister's 1.27 mm shape (discounted) | not read | 1.4 | 1.6 | 1.8 |
| NASA steps with grooves, facing unstated, over Mach 0.9 | 1.0 | 1.3–1.8 | 1.8–2.0 | 1.7–2.0 |
| joint gaps (grooves) | 1.0 | 1.0 | 1.0 | 1.0 |
| static ports | 1.0 | 0.9–1.0 | 0.9–1.0 | 0.8–1.0 |
| screw, rivet and rail-button heads, two faces | 1.1 | 1.3–1.4 | 1.5–2.2 (1.0–2.9) | 1.6–2.3 (1.0–2.9) |

- **Gaps and ports** are measured only at Mach 0.8 and 1.4. **Heads** are not measured there at all: the head row
  splits each head 0.62 forward face, 0.38 rearward (Hoerner's 0.30 and 0.18, excrescence-drag §2), after [T1] p. 13,
  the low end on the 1.27 mm shape, the high end on PC2; Hopko's 1.3–2.9 times Hood's rule at Mach 1.2–2.0
  (excrescence-drag §3) is the outer range. The proud band is split the same way.
- **Clean airframe, as % of the subsonic C_D.** 98 mm: 2.0% becomes about 2.5% (2.2–3.8) at Mach 1.0 and 3.2–3.8%
  (2.4–4.4) at Mach 1.1. 54 mm: 4.4% becomes about 5.5% (4.7–8.6) and 7.1–8.5% (5.3–9.7). The typical span is the
  head factor's 1.5–2.2; the low end of the range takes the 1.27 mm rearward shape and no rise in the heads, the high
  end heads at 2.9. With discrete-steps-drag's inner-law totals (2.4–2.6% and 5.1–5.5%), Mach 1.1 gives about
  3.8–4.2% and 8.2–8.8% with heads at 1.5.
- **As a share of the rocket's own C_D,** which is 1.36–1.51 times its subsonic value near Mach 1.04–1.12 with a flat
  base and 1.7–1.9 times with a boattail ([transonic drag rise](transonic-drag-rise.md)): at Mach 1.1, 2.1–2.8%
  (1.6–3.2) on 98 mm and 4.7–6.3% (3.5–7.1) on 54 mm with a flat base; 1.7–2.2% and 3.7–5.0% with a boattail, at or
  below the subsonic share. The features rise about as fast as the rocket.
- **What it changes in excrescence-drag §4.** Its "faster than sound" totals (3–4% and 6–9%) hold at Mach 1.1; its
  heads, held near their share there, rise 1.5–2.2 times here on this note's two-face assumption.
- **In apogee,** at −0.31% to −0.57% of apogee per 1% of drag ([apogee bias](apogee-bias.md) sensitivity table). If
  the Mach 1.1 share held over the whole flight, an upper bound with a flat base: 98 mm, −0.7% to −1.6% of apogee
  against −0.6% to −1.1% subsonic; 54 mm, −1.5% to −3.6% against −1.4% to −2.5%. The transonic increment is at most
  0 to −0.5% (98 mm) and −0.1% to −1.1% (54 mm): 0–5% and 1–12% of the +8.73% bias. At the high ends of the ranges,
  −0.4% to −0.7% and −0.8% to −1.5%, up to 8% and 18% of the bias. With a boattail it is within ±0.3% of apogee. The
  sign of the features' own drag stays one-way: the codes fly high.

## What this suggests

- **A Mach factor for an optional joints term:** C_D/c_f from the RAE fits at Mach 0.8 and 1.4, joined by PC1 for
  forward faces and a rearward factor between the PC2 and 1.27 mm shapes, heads as two faces, with grooves and ports
  held at their Mach 0.8 value. Expected effect: the term's ΔC_D rises 1.2–1.3 times at Mach 1.0 and 1.6–1.9 times at
  1.1, so its share of the rocket's C_D stays within about 1.05–1.4 times the subsonic one with a flat base; apogee
  up to about 0.5% lower on 98 mm than a subsonic-only term.
- **Keep the bias budget's transonic share** near the subsonic 2.0–4.4%, now on measured factors: 1.05–1.4 times it
  with a flat base, about 1 with a boattail, bounded at 1.6 times.
- **A validation case:** fly TN D-5004's smooth and stepped ogive cylinders through such a term, Mach 0.7 to 1.2, and
  compare ΔC_D; it would test the factor that drives the range, once the steps' facing is settled.

## Open questions

- **Pallister, ARA Report 37 (1974),** the transonic data with their h+ and step heights: not found online.
- **Gaudet and Johnson, RAE TR 70190 and TR 71181:** DTIC returned 403 on 2026-10-28; neither is among the 34 distinct
  hits for Gaudet or excrescence in the Internet Archive's DTIC mirror that day. Gaudet 1987 stays abstract only.
- **Heads between Mach 0.8 and 1.4** are unmeasured in every source read; their factor (1.0–2.9) is unbounded by data.
- **Which rearward shape applies to 0.2–0.5 mm steps,** which way h+ moves the transonic factor (§4), and which way
  TN D-5004's steps face: the sources span 1.6–3.4 at Mach 1.1, the largest range here.
- **Free flight:** a rocket's shoulders and boattail set local Mach and gradients the tunnels did not. Not bounded.

## Sources

All read 2026-10-28; printed page numbers; cached in the gitignored `refs/`. NASA reports: public use per NTRS. T1:
Crown copyright (cover year under a stamp; issued September 1973), approved for public release by DTIC (DSTL, 2008),
"may be subject to privately-owned rights". T2: Copyright © AGARD 1981, All Rights Reserved, approved for public
release. Both read from the Internet Archive's DTIC mirror; cited only, no figure or table copied. ESDU: proprietary,
public summaries only.

- **T1.** Gaudet, L., Winter, K. G., *Measurements of the drag of some characteristic aircraft excrescences immersed
  in turbulent boundary layers*, RAE TM Aero 1538, 1973 (amended from an AGARD drag meeting paper, Izmir, 1973),
  summary, pp. 3, 6, 7, 9, 10, 13, fig. 6. DTIC AD0918999, read from <https://archive.org/details/DTIC_AD0918999>;
  DTIC's own <https://apps.dtic.mil/sti/pdfs/AD0918999.pdf> returned 403.
- **T2.** Young, A. D., Paterson, J. H., *Aircraft Excrescence Drag*, AGARDograph 264, 1981, pp. 62, 68, 73, 74,
  78–80, 82–84, 104, 105 (figs. 4.6a, 4.13, 4.18, 4.19, 4.23–4.25, 4.28–4.30; refs. 4.15, 4.18, 4.26). DTIC
  ADA106030, read from <https://archive.org/details/DTIC_ADA106030>.
- **T3.** Czarnecki, K. R., Monta, W. J., *Roughness drag due to two-dimensional fabrication-type surface roughness
  on an ogive cylinder from force tests at transonic speeds*, NASA TN D-5004, 1969, pp. 1, 4–9, 13, 17, 22, 24, 40.
  <https://ntrs.nasa.gov/citations/19690007128>
- **T4.** Czarnecki, K. R., Monta, W. J., *Pressure drags due to two-dimensional fabrication-type surface roughness
  on an ogive cylinder at transonic speeds*, NASA TN D-3519, 1966, pp. 1, 10.
  <https://ntrs.nasa.gov/citations/19660024159>
- **T5.** ESDU 76008, Toolbox public summary (rivet heads; Mach numbers tested).
  <https://beta.esdu.com/toolbox/ui/tba_76008>
- **T6.** ESDU 75031, Toolbox public summary (steps and ridges, Mach up to 3).
  <https://beta.esdu.com/toolbox/ui/tba_75031>
- **Not reached:** Pallister, K. C., ARA Report 37, 1974 (cited through T2); Gaudet and Johnson, RAE TR 70190 and
  TR 71181; Gaudet, L., Aeronaut. J. 91(904):170–182, 1987. <https://doi.org/10.1017/S000192400002114X>
