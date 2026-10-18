# Hagerty's 1975 tunnel tests: a hobby rocket's measured normal force

This note reads Hagerty's tunnel report on an Estes Cineroc/Omega [H]: which values its hand-drawn graphs hold, on what
geometry and Reynolds number, and whether HPR Sim could fly it to compare the normal-force slope (C_Nα), body lift and
drag. It follows [hobby-rocket-tunnel-data](hobby-rocket-tunnel-data.md), which found no measured C_Nα of a hobby
rocket. Every reading is this note's own pixel calibration of the scans; **Inference** marks its own arithmetic.

**In short: the report gives lift and drag against angle of attack, 0° to 23°, for the two-stage rocket with and without
its booster fins, at Mach 0.03–0.09; it gives no pitching moment or center of pressure.** On the rocket with one fin
set, a hand Barrowman slope plus Jorgensen's body lift lands within −1% to +3% of the measured normal force from 5° to
23° (+3% to +9% on the kit's fins). At 1.4° and 2.4° it reads 18–34% high, as an angle-zero offset near 0.3° would make
it. The rocket with two fin sets shows what HPR Sim leaves out: booster fins in line behind the upper fins added only
4–19% (±3–9 points) of their own Barrowman slope near 0°, and a third at 23°. Summing the two sets, as Barrowman's
method and HPR Sim do, gives 1.9–2.6 times the measured C_Nα. With an assumed nose, configuration 1a can be HPR Sim's
first measured subsonic normal force of a whole hobby rocket; drag needs parts the report doesn't dimension. Confidence:
high in the readings (two scans agree within 0.024 in C_L), medium in the comparisons, low for drag.

## 1. What the report holds

- **The document.** A 22-page scan with no text layer: a 1975 AIAA student-conference paper from UC Berkeley,
  republished for NARAM-52 with a summary and an addendum (PDF pp. 2–3). Counted 2026-10-17: PDF pp. 4–11 hold the
  abstract and printed pp. 2–7 and 10; pp. 12–21 hold Figs. 2–9. Printed pp. 8, 9 and 11 (results, conclusions,
  references) and Fig. 1, the configuration drawing, are not in the scan, nor is Eq. 8, which Fig. 8 cites.
- **The rocket.** An Estes Omega two-stager carrying a Cineroc movie camera, 28 in (0.711 m) long, 1.75 in (0.044 m)
  across the camera (printed p. 3, PDF p. 6). Fig. 8 (PDF p. 20) adds a 1 5/8 in body and two cruciform fin sets.
  Upper fins: root 3 in, tip 2 in, span 2 1/4 in. Booster fins: root 3 1/2 in, tip 2 1/2 in, span 2 3/4 in, in line
  with the upper fins (printed p. 4, PDF p. 7).
- **Four configurations, not three.** The abstract says three; the text has four (printed p. 3). 1: both stages, with
  a small tail fairing standing in for the plume. 1a: 1 without the booster fins. 2: the upper stage alone, with the
  fairing. 3: the upper stage with a spent casing and no fairing. Lift and drag against angle exist for 1 and 1a only
  (Figs. 6, 7); drag against Reynolds number for all four (Fig. 9).
- **The tunnel.** UC Berkeley's low-turbulence tunnel: a 0.81 m (2.67 ft) square test section, 3 m long, up to
  45 m/s, contraction ratio 14.1; speed from a Betz manometer read to 0.02 mm of water (printed p. 5, PDF p. 8).
- **Mount and balance.** A two-axis beam balance on the roof, on a universal joint, read lift and drag together "to an
  accuracy of nearly 0.01 grams" at low speed. A shield covers the sting where it leaves the ceiling (printed p. 6,
  PDF p. 9). Figs. 2 and 3 (PDF pp. 12–13) show a vertical sting into the side of the body and the fins at 45° to it,
  so the angle was set in the horizontal plane (**Inference** from the photos).
- **Not given.** Wall or blockage corrections, a tare subtraction, the angle's accuracy or zero, the α sweeps' speed,
  air temperature, repeat counts, and any pitching moment. The only stability evidence is qualitative: loaded with
  motors, 1 and 1a were "judged to have equal restoring moments" (printed p. 4). **Inference:** the frontal area
  blocks 0.24% of the test section and the side-projected area 2.6–3.5% at 23° with the fins; the model is 88% as long
  as the section is wide. This note doesn't bound a correction.
- **Reference area.** C_D is on the maximum cross-section (abstract, PDF p. 4): π(1.75 in)²/4 = 2.405 in², 15.5 cm².
  C_L's area isn't stated. **Inference:** it is the same, because Fig. 8's two predicted slopes, 0.325 and 0.445 per
  degree, equal 2(b² + d_b²)/d_m² per radian (the slope of the report's Eq. 6 on that area) at its stated b/d_m of
  2.91 and 3.45, to three digits.
- **Reynolds number.** Fig. 9's data span 1.67 × 10⁴ to 5.17 × 10⁴ per inch (readings): 4.7 × 10⁵ to 1.45 × 10⁶ on
  the 28 in length, matching the abstract's 5 × 10⁵ to 1.5 × 10⁶, and 2.9 × 10⁴ to 9.0 × 10⁴ on the 1.75 in diameter.
  At ν = 1.50 × 10⁻⁵ m²/s (20 °C, assumed) the speed is 9.8–30.5 m/s, the stated 30–100 ft/s.
- **The α sweeps' speed (Inference).** The abstract caps the crossflow Reynolds number at 10⁴. At 23.4° on a 1.65 in
  mean diameter that holds only below about 9 m/s, the bottom of the range: about 4–5 × 10⁵ on length. Fig. 7's
  configuration-1a drag at 0.4° (0.64) also equals Fig. 9's at the lowest speed (0.636).

## 2. Reading the graphs

Figs. 6 and 7 each come twice: a typed-caption copy (PDF pp. 16, 18) and the hand-drawn "Original Graph" (pp. 17, 19).
This note read all four, and Fig. 9 (p. 21), from 200 dpi renders.

- **Method.** Grid lines found by darkness profiles in a strip through each point, a lattice fitted per strip (which
  absorbs the scans' tilt, up to about 0.6°), and symbol centers from an outline fit, checked by eye. Scales: Fig. 6,
  50.15 px per degree and 49.8 px per 0.25 in C_L; Fig. 7, 50.0 px per degree and 49.8 px per 0.1 in C_D; Fig. 9
  (log–log), 1,505 px per decade of Reynolds number and 1,484 px per decade of C_D.
- **Error.** A symbol's center to ±2 px is ±0.04°, ±0.010 in C_L and ±0.004 in C_D. The two versions agree within
  0.024 in C_L and 0.04° at 17 of Fig. 6's 20 points, and within 0.005 in C_D and 0.08° at all 20 of Fig. 7's.
- **Where the versions differ.** Fig. 6's last configuration-1 point is 22.9°, C_L 10.37 on the original, 22.1°, 9.83
  on the copy; near 1° two symbols swap. The copy's Fig. 7 adds two 1a repeats, 0.05 lower at 1.4° and 3.4°.
- **Angles.** Configuration 1's points sit on whole degrees, 1a's 0.4° past them on both figures, unexplained.

## 3. The normal-force slope

Normal force here is C_N = C_L cos α + C_D sin α, with C_D interpolated from Fig. 7 at each angle; it adds 0.6–0.9 per
radian to the lift slope. Slopes are per radian on 15.5 cm², from the original graphs and then the copies:

| Fit of C_N against α | Configuration 1a | Configuration 1 |
|---|--:|--:|
| line through 0, points below 3.5° (3 points) | 13.6 / 14.6 | 17.5 / 17.1 |
| linear and quadratic, below 7.6° (5 points) | 13.0 / 14.0 | 15.4 / 14.8 |
| offset, linear and quadratic, all 10 points | 16.3 / 16.0 | 18.7 / 19.2 |

The fits' standard errors are 0.4–1.3 per radian, smaller than the spread between fits. The offset fits put zero lift at
0.1–0.4°, so the near-zero slope rests on an angle zero the report doesn't give.

**Barrowman by hand (Inference).** This uses Barrowman's fin equation as HPR Sim's [aero page](../physics/aero.md#fins)
gives it ([B67]), with the interference factor K = 1 + r/(s + r), and his body terms. The body gives 2 for the nose to
1.75 in and 2((1.625/1.75)² − 1) = −0.28 for the shoulder: 1.72 per radian, from areas alone, so the undimensioned
nose and camera don't change it. The upper fins on the report's dimensions give 11.28, times K = 1.265, or 14.27.
Configuration 1a: **16.0 per radian** (0.279 per degree); 16.9 on the kit's fin pattern (section 7). Any mid-chord
sweep from leading edge unswept to trailing edge unswept moves the fin term under 1%. So Barrowman sits −2% to +23%
from the measured slope near 0° (to +30% on the kit's fins), closest on the fit that allows an angle offset. The
report's own slender-body estimate, 0.325 per degree (18.7 per radian), is 14–43% above.

## 4. Body lift: the nonlinear term

The report puts C_L's growth with α² down to the body's crossflow drag, after Allen and Perkins (printed p. 6). HPR Sim
sizes it by Jorgensen ([J77]; [aero page](../physics/aero.md#body-lift)): C_N = η C_dn (A_plan/A_ref) sin²α.

- **Inputs (Inference).** C_dn ≈ 1.13: [J77] Fig. 2 (p. 76) holds 1.2 from about 2 × 10⁴ up to the critical crossflow
  Reynolds number, then falls to about 1.13 at 10⁴, where it ends; the test sits at or below 10⁴. η ≈ 0.73 at
  fineness 16 ([J77] Fig. 4, p. 77); A_plan 44–46 in², less the nose: a factor of 15.0–15.7 (16.0–16.8 at 1.2).
- **Against configuration 1a.** The hand slope (16.0 sin α cos α) plus 15.4 sin²α reads +3.2%, +0.7%, +0.8% and
  −0.7% against the measured C_N at 5.4°, 9.4°, 15.4° and 23.4° (C_N 1.60, 2.96, 5.14 and 8.30); at 16.4 sin²α, +1% to
  +4%. It reads +9% at 11.4°, a point below the report's own faired curve, and +18% to +34% at 1.4° and 2.4°; the 0.4°
  point isn't resolved (two pixels move it 10 points). On the kit's fins (slope 16.9), +3% to +9% from 5° up.
- **Size and sign.** At 23.4° body lift is 2.4–2.6 of the estimate's 8.2–8.4, about 30%. From 5° up the sign of the
  gap flips within the C_dn and reading errors, so this case can't say which way body lift errs.

## 5. Two fin sets in line

The report found the booster fins raised lift only 1.35 times as α → 0, while adding 2.44 times the fin area and 1.43
times the drag (printed p. 7, PDF p. 10). It reasons that the upper fins' downwash nearly cancels α behind them, so
only the part of a booster fin beyond the upper fins' span lifts, which adds 24% more area.

- **Readings (Inference).** The booster fins add 0.8–3.9 per radian to C_Nα, across the fits and versions (each ±0.6–1.8
  from the fits' errors). Alone, Barrowman gives them 20.6: they deliver 4–19% of it. The share grows with angle: at 23°
  the secants are 27.2 per radian (configuration 1) and 20.4 (1a), a 6.8 gap, or 33%. The slope ratio near 0° reads
  1.06–1.29.
- **What HPR Sim does.** Each fin set is its own term; the [aero page](../physics/aero.md#fins) lists interference
  between sets at the same station as not modeled and names no term for a set in another's wake. Summed,
  configuration 1 is 36.6 per radian (38.4 on the kit's fins), against 14.8–19.2 measured.
- **The center of pressure (Inference, not measured).** With Fig. 1 missing, assume the booster fins' trailing edge at
  the base, the upper fins' 5 in ahead of it, the nose's CP 2 in and the shoulder 7 in from the tip. The summed estimate
  puts the CP 22.5 in from the nose; with the booster fins at 4–33% of their Barrowman value it moves 0.8–2.0 calibers
  forward (3.5 in spacing gives the low end). The sign is sure: lift lost at the aftmost fins moves the CP forward, so
  summing in-line sets reads a two-stage margin high, the flattering side; the size rests on the assumed positions.

## 6. Drag

- **At 0° against Reynolds number (Fig. 9).** On 15.5 cm², from 1.67 × 10⁴ to 4.0–5.2 × 10⁴ per inch: configuration
  1, 0.76 to 0.70; 1a, 0.64 to 0.54; 2, 0.43 to 0.39; 3, 0.50 to 0.42. Each falls 8–16% over the range.
- **Two values for configuration 1.** Fig. 7 gives 0.90 near 1°, 0.13 above Fig. 9's 0.76 at the lowest speed; 1a
  agrees (0.64 both). The text's 1.43 drag ratio is Fig. 7's; the abstract's 26% cut matches neither figure exactly
  (Fig. 7: 29%; Fig. 9: 17–18%); printed p. 10's "over 30%" rise fits Fig. 7. The pages read don't say why.
- **Axial force (Inference).** Resolving to body axes, C_A = C_D cos α − C_L sin α holds 0.57–0.64 up to 7.5° on 1a,
  then falls to 0.06 at 23.4°; on configuration 1 it falls from 0.89 to −0.91. Rounded fin edges can carry some
  leading-edge suction, which lowers axial force, but this note found nothing sizing it near −0.9. The report gives no
  way to tell that from an angle, lift or drag error. Treat C_D above about 10° as unverified.
- **Friction regime.** The tunnel kept a flat plate laminar to a Reynolds number of 2 × 10⁷ (printed p. 5). HPR Sim's
  fully turbulent friction ([aero page](../physics/aero.md)) reads high if the model ran partly laminar: size unbounded.

## 7. Can HPR Sim fly the configurations?

For normal force, yes, with an assumed nose; for drag, not without guesses whose effect isn't bounded.

| Item | What the sources give | Status |
|---|---|---|
| Length, diameters | 28 in; 1.75 in camera; 1 5/8 in body [H] | stated |
| Fin plans | Fig. 8 [H]; kit 3.16 × 2.04 × 2.32 in and 3.69 × 2.53–2.56 × 2.81 in (root, tip, span) [E] | 1–5% apart |
| Fin sweep | symmetric trapezoids, leading edge back 0.55–0.60 in [E] | read |
| Fin build | 1/8 in balsa, edges round, trailing edges flush with each stage's end [E steps 16, 17] | the kit's |
| Fin roll | in line between stages [H, E]; 45° to the angle's plane (photos) | stated, inferred |
| Nose, camera, shoulder | sketched in Fig. 8, not dimensioned; no Cineroc drawing reached | missing |
| Stage split, tail fairing, 2 and 3 | in Fig. 1, absent | missing |
| Launch lugs | the kit has two half-lugs on stand-offs [E step 14]; the report doesn't say if fitted | unknown |
| Sting position, finish, CG | not given | missing |

The kit's fins were read from its pattern sheet at 300 dpi [E, sheet g]; its lines include the cut's width, and the gap
to Fig. 8 moves the fin slope 6%. The nose and camera move only A_plan (a few percent) and the unmeasured CP.

## What the project can use

- **Terms.** [H] is a NAR member benefit with no license (PDF p. 22): cite it, cache it privately, and commit neither
  its pages nor full digitized tables; a fixture would hold comparison statistics and a few cited points. The Estes
  instructions [E] are Estes's, scanned by a hobby site: cite only, cached under `refs/research/`.
- **A case for configuration 1a's C_N, 5° to 23°.** Fly it in HPR Sim at Mach 0.03 and a length Reynolds number of
  5 × 10⁵ with an assumed Cineroc nose, and compare C_N angle by angle. Expected: +1% to +15% high. It would be HPR
  Sim's first measured subsonic normal force of a whole hobby rocket, and the first check of body lift at hobby
  fineness and low crossflow speed. It complements Cranfield's case (#611), which has a CP but a 1955 shape.
- **An issue on in-line fin sets.** Summing a booster set in another's wake overstates C_Nα 1.9–2.6 times here, and on
  section 5's assumed fin positions reads a two-stage margin 0.8–2.0 calibers high, the flattering side. A cited
  downwash method for tandem surfaces should come before any model.
- **Drag only as a low-Reynolds-number check**, once the missing geometry is assumed. On #172 the report is silent:
  with no moment measured, it can't say which code's CP is right.

## Open questions

- **The missing pages.** Another copy of the 1975 paper could give printed pp. 8, 9 and 11, Fig. 1 and Eq. 8: the α
  sweeps' speed, the stage split, the fairing, and configurations 2 and 3.
- **The angle zero.** What sets 1a's angles 0.4° off whole degrees, and the estimate 18–34% above the low-angle C_N:
  an offset, balance friction at loads of a few grams, or low-Reynolds-number fins (chord Re ≈ 5 × 10⁴)?
- **The axial force's sign change** above 15° on configuration 1, and **the Cineroc's shape**: no dimensioned drawing
  was reached, and a museum record gives only its box's size.
- **Downwash methods for tandem fins** at hobby spacing, to turn section 5 into a model with a cited source.

## Sources

Read 2026-10-17.

1. **[H]** J. J. Hagerty, "Low Speed Wind Tunnel Tests of a Small Rocket", AIAA Western Region Student Conference,
   Stanford, May 1975; NAR R&D report, NARAM-52, 2010. Members only; cached privately in `refs/papers/`.
   <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
2. **[E]** Estes Industries, Omega K-52P assembly instructions, scans on JimZ's Rocket Plans: sheets b (step 10),
   e (steps 14, 17), f (step 16) and g (fin pattern). <https://www.spacemodeling.org/jimz/k-52.htm>
3. **[B67]** J. S. Barrowman, *The Practical Calculation of the Aerodynamic Characteristics of Slender Finned
   Vehicles*, MS thesis, 1967 (NASA/TM-2001-209983).
   <https://ntrs.nasa.gov/api/citations/20010047838/downloads/20010047838.pdf>
4. **[J77]** L. H. Jorgensen, *Prediction of Static Aerodynamic Characteristics for Slender Bodies Alone and with
   Lifting Surfaces to Very High Angles of Attack*, NASA TR R-474, 1977.
   <https://ntrs.nasa.gov/api/citations/19770026166/downloads/19770026166.pdf>
5. Smithsonian National Air and Space Museum, Cineroc retail box, A19730569000 (box size only).
   <https://airandspace.si.edu/collection-objects/rocket-flying-model-movie-camera-cineroc/nasm_A19730569000>
