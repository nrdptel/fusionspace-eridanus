# Is the Arcas Robin short model's Mach 0.9 reading biased? No: the row is in the report, and the gap stands

NASA TN D-4013's short Arcas Robin model puts the center of pressure (CP) 1.34 calibers ahead of HPR Sim's at Mach 0.9,
the gap that sizes [#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198) (an open CP warning). In the
committed readings its fins-on less fins-off normal-force slope falls 18% from Mach 0.6 to 0.9 (9.51 to 7.77 per rad)
while the long model's rises 6%, and its fins-off slope is 2.44 per rad, against 1.41 at 0.8 and 1.58 at 1.0. This note
asks whether a misreading, page skew, the walls of Langley's 8-ft Transonic Pressure Tunnel (TPT) or the model explain
that row, and how much gap a corrected row leaves. **Inference** marks this note's own readings, fits and arithmetic. It
builds on [transonic-body-normal-force](transonic-body-normal-force.md),
[transonic-fin-lift-methods](transonic-fin-lift-methods.md) and [dense-transonic-fin-body](dense-transonic-fin-body.md),
and moves that issue and the Arcas CP row of [accuracy](../accuracy.md).

**In short: the short model's Mach 0.9 row is what the report plotted, the walls can't move it, and a corrected row
leaves the whole +1.34-caliber gap (best estimate +1.35, range +1.31 to +1.37).**

- **The reading holds.** An independent re-read of the 300-ppi scans returns every committed short-model slope within
  0.3 per rad. The fin share still falls 19% (9.62 to 7.81 per rad, about 2.4 standard errors); the fins-off slope reads
  2.49 ± 0.10. Page skew moves a slope 0.06 per rad at most and cancels in the fin share;
  [#97](https://github.com/nrdptel/fusionspace-eridanus/issues/97) (a page-skew check) covers TN D-4014, not this one.
- **The walls are more than ten times too small.** Blockage is 0.055% to 0.071%; wall corrections move C_Nα under about
  1.5% and the CP under about 0.04 caliber, act the same on both models and are larger on the long one.
- **The models differ with fins off.** The short model kept its protruding fin anchors; the long model's slots were
  faired flush ([1] pp. 3–4). The fins-off pitching moment puts the extra Mach 0.9 lift at about 88% of length, near the
  anchors. If they inflate the baseline, the fin-share dip halves (−9%, not −19%); the CP gap doesn't move.
- **The gap rests on Fig. 11, not on the slopes.** Fig. 11 re-reads as 71.56% (committed 71.6%); the per-angle points
  over −2° to 2° give 71.8%, a gap of +1.31. Only a straight line in Mach from 0.8 to 0.95 (+0.80) or a −3° to 4° window
  (+0.77) shrinks it, and the plotted data support neither.

Confidence: high that the row is no reading or wall artifact (two channels agree, wall terms bounded); medium on its
cause (anchors or an aft-body effect can't be told apart); ±0.13 caliber on the per-angle CP.

## 1. The reading

- **Point by point (Inference;** native scans of Figs. 7(b), 7(c), 9(b), 11 [1], grid fitted locally, symbols matched to
  the legend; 50 points: 5 Mach × 5 angles × fins on and off). Away from α ≈ 0 (40 points) the re-read differs from the
  committed values by −0.006 mean, 0.015 rms, 0.041 at most. Five differ by over 0.03: four are α ≈ 0 clusters of
  overlapping symbols, one the Mach 0.9 fins-off point at −2.07° (−0.045 against −0.004).

| per rad, committed / re-read | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 |
|---|--:|--:|--:|--:|--:|
| fins on | 11.05 / 11.19 | 9.92 / 10.11 | 10.22 / 10.31 | 11.88 / 11.83 | 15.79 / 15.59 |
| fins off | 1.53 / 1.57 | 1.41 / 1.44 | 2.44 / 2.49 | 2.88 / 2.80 | 1.58 / 1.67 |
| on less off | 9.51 / 9.62 | 8.51 / 8.67 | 7.77 / 7.81 | 9.01 / 9.03 | 14.21 / 13.92 |
| on less off, ± from fit scatter | 0.44 | 0.53 | 0.60 | 0.40 | 0.40 |

- **The fall survives (Inference):** −1.81 per rad (−19%) from Mach 0.6 to 0.9, about 2.4 standard errors. It is −24% on
  the −2° to 2° points alone and −18% on the −3°, 0° and 4° points, the long model's angles.
- **The Mach 0.9 fins-on scatter is curvature:** slopes between successive points are 12.1, 8.4, 9.1 and 13.9 per rad
  (0.6 alike, less deep; 0.8 rises steadily: 8.5, 9.1, 9.6, 12.7). The fins-off points lie within 0.010 rms of a line.
- **Page skew.** [#97](https://github.com/nrdptel/fusionspace-eridanus/issues/97) reports an unverified 2.6% to 5.4%
  slope bias on one page of TN D-4014 [2] (long model, Mach 1.8 up; open 2026-11-05). Here a 0.002 lean across −4° to 4°
  (0.008 in C_N) is 0.06 per rad: under 1% of a fins-on slope, 2% of a fins-off one; it cancels in the share.
- **The long-model control** (Fig. 9(b)) re-reads at 8.40, 8.50, 9.71 and 14.32 per rad at Mach 0.6, 0.8, 0.9 and 1.0
  (committed 8.98, 8.74, 9.49, 14.61): a 16% rise to 0.9, not 6%.

## 2. What the gap rests on

- **Not the slopes.** The committed tunnel CP is read off Fig. 11's faired curve (p. 60), labeled −2° ≤ α ≤ 2°, not
  computed from the C_N points; the 18% fall and the gap rest on different plots. The gap is (0.82146 m − x_cp × 1.04013
  m) / 0.05715 m, positive when HPR Sim's CP is aft (Inference).
- **Fig. 11 re-read (Inference):** 70.86% of length at Mach 0.8, 71.56% at 0.9, about 76.3% at 0.95 and about 83.6% at
  1.0. The committed 70.9, 71.6, 76.45 and 83.55 hold within ±0.3% (±0.6% at 0.95 and 1.0).
- **From the per-angle points (Inference):** x_cp = 70% − (C_mα / C_Nα) / 18.2, on the re-read fins-on symbols.

| Mach | −2° to 2° | 0° to 2° | −2° to 0° | −3° to 4° | Fig. 11 (committed) |
|---|--:|--:|--:|--:|--:|
| 0.6 | 75.3% | 74.4% | 76.3% | 76.9% | 74.7% |
| 0.8 | 71.0% | 71.7% | 70.2% | 74.4% | 70.9% |
| 0.9 | 71.8% | 71.5% | 72.1% | 74.7% | 71.6% |
| 0.95 | 76.7% | 74.3% | 79.1% | 77.2% | 76.45% |
| 1.0 | 82.4% | 82.2% | 82.6% | 81.5% | 83.55% |

- **Curve against points.** Over −2° to 2° they agree within 0.12 caliber at 0.6, 0.8, 0.9 and 0.95; at 1.0 the points
  sit 0.20 forward. Mach 0.9 is corrected by [arcas-fin-share-cp-station](arcas-fin-share-cp-station.md) §1.
- **Uncertainty at 0.9.** At ±0.03 a point, C_mα over −2° to 2° carries ±0.6 per rad, ±0.07 caliber of CP; the stated
  balance accuracy (C_N ±0.03, C_m ±0.05 [1] p. 4) adds about ±0.11 if independent between angles.
- **The window matters.** The report calls the short model "almost neutral" from −2° to 2° at Mach 0.60, 0.80 and 0.90
  [1] p. 6. Zero lift falls about 0.5° below the corrected α = 0 (fins-on C_N +0.06 to +0.09 there); the two
  half-windows (0° to 2°, −2° to 0°) span 71.5% to 72.1%.
- **The long model takes the same step.** Its Fig. 12 (p. 61) CP goes from 75.6% to 76.2% from Mach 0.8 to 0.9, +0.14
  caliber aft; the short model's moves +0.13 aft. The gap grows from +0.91 at 0.8 to +1.34 at 0.9 because HPR Sim's CP
  moves 0.56 caliber aft (75.9% to 79.0% of length).

## 3. The tunnel walls

- **The tunnel** ([1] p. 3; [3] pp. 3–4, 11; [4] p. 7): square, 85.2 to 85.5 in a side with corner fillets, 48.7 to 50.3
  ft²; solid side walls, four slots each in floor and ceiling. TN D-4013 states no wall, blockage or lift-interference
  correction; α is corrected for flow angularity and sting bending, and Mach is good to ±0.003 [1] p. 4.
- **Blockage (Inference).** The 2.25 in body's 3.976 in² is 0.055% of about 7,243 in², 0.069% to 0.071% with fins. TN
  D-7331 found significant drag errors near Mach 1 from 0.03% up [4] pp. 1–2. A solid-wall, one-dimensional bound on the
  Mach error, ΔM = M (1 + 0.2 M²) ε / (1 − M²), is +0.0030 to +0.0039 at Mach 0.9; slots relieve it. At the Arcas's own
  rise from 0.90 to 0.95 (+16% fins on, +18% off), that is at most 1.3% to 1.4% of C_Nα, against the fins-off row's 73%.
  Through Fig. 11's local slope it is about 0.04 caliber of CP, on the side that grows the gap.
- **Lift interference (Inference).** Davis and Moore's upwash factor for a square tunnel with slotted floor and ceiling
  ([5] Fig. 8(a), p. 52), at four slots a wall and 4% to 10% open [3] p. 11, is δ ≈ −0.04 to −0.06 for a small span
  (closed walls +0.137). With model over tunnel area 5.49 × 10⁻⁴ the slope error is −0.02% to −0.03%, under 1% even with
  closed walls and a 1/β³ factor. A nose-to-fin upwash difference that size moves the CP under 0.03 caliber.
- **Flow along the section.** At Mach 0.9 the centerline Mach is flat from about station 0 to 160 in ([3] Fig. 5(b), p.
  49, 1989 data). The 1972 flap schedule "was not optimum near Mach 0.9" [3] p. 1; the 1966 one wasn't found.
- **No reflected wave at Mach 0.9.** In Langley's older 8-ft Transonic Tunnel, a body blocking 0.14% matched
  interference-free pressures through Mach 1.00; reflected compressions had no clear effect up to 1.025, showed in
  schlieren pictures from 1.040 and passed behind a 33.5 in model's base at 1.120 ([6] PDF p. 14).
- **Verdict (Inference).** One 11.5 in sting serves both models [1] p. 4, so their fins share a tunnel station. No wall
  term reaches 1.5% of C_Nα or 0.04 caliber of CP at Mach 0.9, and none acts on the short model and not the long one.
  The blockage term would grow the gap; a wall-corrected row leaves at least 1.31 of the 1.34 calibers.
- **The Mach 1.00 neighbor is the row the walls could touch.** A solid-wall tunnel would choke at Mach 0.971 to 0.975 at
  this blockage (Inference). Above that, Langley saw "transonic creep" (an early drag rise); separately, drag divergence
  came about 0.013 later in Mach as blockage grew from 0.02% to 0.10% [4] p. 14. A 1.00 row read as 0.99 leaves 0.90's
  2.44 just as odd.
- **Same tunnel, other models.** Allen's two missile sets, 36 in and 42.5 in long at about twice the Arcas's blockage
  (Inference), rise from Mach 0.6 to 0.9 (no 0.8 point): whole-model or fin-pair C_Nα +7.3% to +10.9%, body alone +3.5%
  to +5.4% ([transonic-normal-force](transonic-normal-force.md) §2). No dip at 0.9.

**A correction for [dense-transonic-fin-body](dense-transonic-fin-body.md)** (and for
[transonic-fin-lift-methods](transonic-fin-lift-methods.md), which inherits it). Its reflection-free lengths, about 4 in
at Mach 1.025 to 36 in at 1.14, cited to RM L51K14 [7], belong to the 8-ft Transonic Tunnel, not the TPT. NACA Report
1389, on the 8-ft Transonic Tunnel, supersedes RM L51K14 ([6] PDF p. 5), describes a twelve-sided slotted section about
8 ft across (PDF pp. 5–6) and gives the same 4 in to 36 in (PDF p. 15). The TPT's calibration calls that tunnel its
"predecessor" [3] p. 3. No TPT reflection survey was found. A Mach-line estimate scaled by the older tunnel's measured
clearance puts the clean edge near Mach 1.17 (short) and 1.24 (long): the long model's 1.20 row may carry a reflection
(Inference, rough).

## 4. The fin anchors, and what else could explain it

- **The fins-off pitching moment moves with the normal force (Inference,** Fig. 7(c)). C_mα about the 70% center: 26.20
  ± 0.28, 26.12 ± 0.23, 22.65 ± 0.19, 20.53 ± 0.83 and 26.53 ± 1.02 per rad at Mach 0.6, 0.8, 0.9, 0.95, 1.0, within 0.3
  of [transonic-body-normal-force](transonic-body-normal-force.md) §1. The extra fins-off lift is real.
- **Where it acts (Inference).** From Mach 0.8 to 0.9, ΔC_Nα = +1.05 ± 0.31 and ΔC_mα = −3.47 ± 0.30 per rad: the added
  force sits 3.3 ± 1.0 calibers aft of the 70% center, at 88% of length (83% to 94%). The fin roots' leading edges are
  at 91.1% and the boattail starts at 95.6% (the project's fixture geometry).
- **The anchors.** The two models might be taken to share fins, boattail and anchors; the report says otherwise. With
  fins off, the anchors stayed in the short model's slots, proud of the skin, while the long model's slots were faired
  flush ([1] pp. 3–4); the report blames them for the short model's higher corrected axial force from Mach 0.975 up (pp.
  5–6). They sit at that station, but so would an aft-body transonic effect; the figures can't separate the two. Their
  own force is unbounded here: Fig. 1(b) sizes the anchor (0.12d, 0.13d) and a raised root fairing (0.04d, about 0.09
  in), but not what stands proud with the fins off.
- **Sign and size on the fin share (Inference).** If the anchors inflate the fins-off baseline, putting the mean of the
  0.8 and 1.0 fins-off slopes (1.55 per rad) at Mach 0.9 makes the fin share fall 9% from 0.6, not 19%: half the dip
  that [transonic-fin-lift-methods](transonic-fin-lift-methods.md) found no fin method to explain. It leaves the CP gap
  alone: that compares whole rockets with fins on, each anchor holding its fin as designed.
- **Also weighed.** The report says nothing on data quality at Mach 0.9 or 0.95; both models ran at 9.8 × 10⁶ per m with
  grit near the nose and fins [1] p. 4. No other 1960s TPT short-and-long pair was found.

## 5. How much gap a corrected row leaves

HPR Sim's CP held at its −2° to 2° value, 78.98% of length (Inference throughout).

| tunnel CP at Mach 0.9 | x_cp | gap, caliber |
|---|--:|--:|
| committed (Fig. 11) | 71.6% | +1.34 |
| Fig. 11 re-read | 71.56% | +1.35 |
| per-angle points, −2° to 2° | 71.77% | +1.31 (±0.07 reading, ±0.11 balance) |
| per-angle points, 0° to 2° | 71.46% | +1.37 |
| Mach 0.8 row plus the long model's 0.8 to 0.9 step | 71.69% | +1.33 |
| any row above, wall-corrected | ±0.2% | ±0.04 |
| straight line in Mach, 0.8 (70.9%) to 0.95 (76.45%) | 74.60% | +0.80 |
| per-angle points, −3° to 4° (HPR Sim not refitted) | 74.74% | +0.77 |

- **Best estimate +1.35 caliber (+1.31 to +1.37).** Every row the figures support is on the flattering side: HPR Sim's
  CP aft of the tunnel's, its stability margin too large.
- **The two rows that shrink it lack support.** The straight line skips the minimum Fig. 11 and the points both put at
  Mach 0.85 to 0.9; the −3° to 4° window leaves Fig. 11's stated −2° to 2° and a flight's small angles.

## What this suggests

1. **Keep [#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198) and the Arcas Robin CP row of
   [accuracy](../accuracy.md) as they are,** and don't drop or reweight the Mach 0.9 row as a reading error. The
   committed 71.6% is the plotted value; a corrected row moves the gap −0.03 to +0.03 caliber.
2. **Flag the short model's fins-off rows as anchors-on** in `arcas-robin-wind-tunnel.json`, and score the fin share on
   the long model, whose fins-off body is clean. Expected effect: the dip a fin method must explain shrinks from about
   19% to about 9%; the whole-rocket CP rows don't change.
3. **Correct dense-transonic-fin-body.md and transonic-fin-lift-methods.md** to put the reflection lengths on the 8-ft
   Transonic Tunnel (Report 1389, superseding RM L51K14), and treat the TPT's own reflection band as unmeasured.
4. **Re-read the long model's Fig. 9(b) row before a fix leans on it:** its rise to 0.9 reads 16%, not 6%.

## Open questions

- **The anchors' protrusion and force at Mach 0.9.** Fig. 1(b)'s 0.04d root fairing is a first bound.
- **The TPT's flap schedule and centerline survey before 1972,** and any TPT reflection survey.
- **The models' tunnel station** (not printed), and why the short model's CP alone has a sharp minimum near 0.9.

## Sources

US government works on NTRS, public use. All but [7] cached under `refs/` (gitignored) and read 2026-11-05.

1. J. C. Ferris, *Static Stability Investigation of a Single-Stage Sounding Rocket at Mach Numbers From 0.60 to 1.20*,
   NASA TN D-4013, 1967. <https://ntrs.nasa.gov/citations/19670020050>. Printed pp. 3–6; Figs. 7(b), 7(c), 9(b), 11, 12
   (printed pp. 22–25, 41, 60, 61; the PDF page is the printed one plus 2).
2. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967. <https://ntrs.nasa.gov/citations/19670020031>. Not re-read here.
3. C. W. Brooks Jr., C. D. Harris, P. G. Reagon, *The NASA Langley 8-Foot Transonic Pressure Tunnel Calibration*, NASA
   TP-3437, 1994. <https://ntrs.nasa.gov/citations/19950004327>. Pp. 1, 3–4, 11; Fig. 5(b), p. 49.
4. L. M. Couch, C. W. Brooks Jr., *Effect of Blockage Ratio on Drag and Pressure Distributions for Bodies of Revolution
   at Transonic Speeds*, NASA TN D-7331, 1973. <https://ntrs.nasa.gov/citations/19740001903>. Pp. 1–2, 7, 14.
5. D. D. Davis Jr., D. Moore, *Analytical Study of Blockage- and Lift-Interference Corrections for Slotted Tunnels
   Obtained by the Substitution of an Equivalent Homogeneous Boundary for the Discrete Slots*, NACA RM L53E07b, 1953.
   <https://ntrs.nasa.gov/citations/19930093719>. Fig. 8(a), p. 52.
6. R. H. Wright, V. S. Ritchie, A. O. Pearson, *Characteristics of the Langley 8-Foot Transonic Tunnel With Slotted Test
   Section*, NACA Report 1389, 1958. <https://ntrs.nasa.gov/citations/19930092368>. PDF pp. 5, 6, 14, 15, 19.
7. V. S. Ritchie, A. O. Pearson, NACA RM L51K14, 1952; superseded by [6], not read.
   <https://ntrs.nasa.gov/citations/19930087191>
