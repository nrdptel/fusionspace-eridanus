# What the Arcas Robin's leftover fin drag is made of

**In short: the base lip makes the measured fin increment read *low*, not high, so it widens the leftover at Mach 1.5
and 1.8; the largest closer is fin friction taken on the rocket's length.** The parent note,
[fin-wave-drag](fin-wave-drag.md) §5, left 0.020 to 0.080 of the Arcas fin increment (fins on less fins off, short
model) unexplained after linear wave drag. Correcting the lip adds 0.021 to 0.036 at Mach 1.5 and 0.013 to 0.025 at
1.8 to the measured increment. Taking the fins' friction on their own mean chord, as DATCOM does, adds 0.010 to 0.020
to the model. Grit strips add 0.0003 to 0.008; the fins' pressure field on the boattail moves −0.008 to +0.013; the
body's boundary layer takes off up to 0.016. What remains is 0.007 to 0.018 from Mach 2.96 up, about the drag of a
leading-edge radius of 0.002 to 0.005 in (0.05 to 0.13 mm) that the drawing doesn't state: plausibly fin drag. At
Mach 1.5 and 1.8, 0.044 to 0.089 remains that no fin-drag source here reaches; it falls fivefold by Mach 2.96, and
this note can't size its cause. Moves: issue [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70)'s
acceptance (the increment, not the fins-on rows, is the test), and the body's fins-off excess, which grows from
+13.6%…+24.1% to +18.5%…+36.4%. Drag that reads high reads apogee low. "Inference" marks this note's own arithmetic.

## 1. The numbers this note starts from

- **Reference area:** the body's cross-section, 2.25 in (57.2 mm) across, 3.976 in² ([1] symbols, p. 3; Fig. 1,
  p. 8). Every coefficient below is on it, as in the committed fixtures [7].
- **Measured:** forebody C_A = C_A − 1.383 C_A,c, the balance's axial force with the chamber's pressure taken over the
  whole 1.470-in base ([aero](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel); [1] p. 4 says the axial
  force is not corrected, and plots C_A,c in Fig. 4). Each reading is ±0.0016 to ±0.0031; the report states ±0.004 [1]
  p. 5. Measured increments: 0.153, 0.112, 0.085, 0.061, 0.050 and 0.046 at Mach 1.5, 1.8, 2.3, 2.96, 3.96, 4.63.
- **Modeled:** linear wave drag 0.0462/β plus HPR Sim's fin friction from `drag-vs-mach.json` (fins on less fins off:
  0.032 at Mach 1.5 to 0.016 at 4.63). Leftover: 0.080, 0.051, 0.036, 0.021, 0.020, 0.020 (recomputed, Inference).

## 2. The base lip: the measured increment is low at Mach 1.5 and 1.8

**What the report says.** Fins off, the short model's chamber axial force reads low at Mach 1.50 and 1.80 against the
other Mach numbers near zero angle; the authors blame the reflexed lip, and say separation over the boattail or the
fins' thicker boundary layer masks it; the long model doesn't show it [1] p. 6.

**Which way it moves the increment.** The report doesn't say whether the raw C_A moves with C_A,c. If the chamber
pressure truly rose, raw C_A would fall with it and only the 0.383 share would read high: 0 to +0.008. But the raw C_A,
long less short, has no step at Mach 1.8 (third reading below), so this note takes raw C_A as unaffected. Then a low
C_A,c means a smaller subtraction, the fins-off *forebody* reads high, and the increment reads low. With fins on the
dip is weaker ("particularly for the fin-off condition"); the fixture shows none clearly. HPR Sim gives the lip
nothing ([the lip decision record](../decisions/0039-a-lip-in-a-boattails-wake-carries-nothing-faster.md)), so the
comparison needs the fins-off forebody with the lip masked. Fig. 4(a) (p. 21), read by eye at 360 dpi: fins-off
circles (Mach 1.50) near 0.005, squares (1.80) near 0.014, others near 0.02 to 0.03, as the fixture has them.

**Size, three readings (Inference, fixture values [7]):**

- *Chamber replaced.* From Mach 2.3 up, fins-on less fins-off C_A,c is −0.0005 to +0.0040. Taking the fins-off value at
  1.5 and 1.8 as the fins-on one less that gives a masked C_A,c of 0.0265 to 0.0310 and 0.0275 to 0.0320, against
  0.0051 and 0.0142 read. Times 1.383 (whole base) or 1 (chamber only), the fins-off forebody falls by **0.021 to
  0.036 at Mach 1.5 and 0.013 to 0.025 at 1.8**.
- *The long model's increment* (no lip effect [1] p. 6): 0.128 at Mach 1.8 against the short's 0.112, so +0.016. From
  Mach 2.3 up they differ by −0.005 to +0.004, so this reading spans 0.012 to 0.021; the long model's anchors (§6) may
  carry part of it, unsized.
- *Long less short, fins off.* Measured: 0.013 at Mach 1.8, against 0.026 to 0.045 at 2.3 to 4.63, where it is 1.00 to
  1.28 times HPR Sim's friction difference. Scaled the same, Mach 1.8 should show 0.043 to 0.055: a short-model excess
  of 0.030 to 0.042. The raw C_A (no chamber term) of long less short runs 0.035, 0.034, 0.024, 0.026, 0.022 without a
  step, so the step lives in the chamber reading.

**So:** the lip-free increment is 0.174 to 0.189 at Mach 1.5 and 0.125 to 0.137 at 1.8 (up to 0.154 by the third
reading). The leftover grows to 0.101 to 0.116 and 0.064 to 0.076. The long model's own leftover at 1.8, 0.068 (parent
§5), agrees. From Mach 2.3 up the lip is masked; the chamber-area choice moves the increment by at most 0.0015 there.

## 3. Fin friction taken on the rocket's length

HPR Sim takes every component's friction Reynolds number on the rocket's length (`crates/hpr-aero/src/drag.rs`,
module comment, after Niskanen's thesis §3.4.1). DATCOM builds a supersonic wing-body's zero-lift drag from its
exposed parts, with no interference term, and takes each exposed panel's friction on that panel's mean aerodynamic
chord [6] pp. 4.3.3.1-17 to -18. [junction-drag](junction-drag.md) §4 found this worth 0.023 at Mach 0.6.

- **Inference:** the tunnel's 9.84 × 10⁶ per m gives 1.02 × 10⁷ on the short model's 1.040 m and 7.0 × 10⁵ on the fins'
  71.6 mm (2.82 in) mean aerodynamic chord. HPR Sim's turbulent formula, 1/(1.5 ln R − 5.6)², is 1.626 times higher at
  the chord's. Fin friction rises by **+0.020, +0.019, +0.017, +0.014, +0.012 and +0.010** (Mach 1.5 to 4.63).
- **The trip doesn't undo it.** The fin grit sits 0.5 in (12.7 mm) behind the leading edge [1] p. 4, so the first 18%
  of the mean chord is laminar. Laminar ahead (1.328/√R) and turbulent behind gives the same incompressible friction as
  turbulent from the edge, within 1%. Compressibility cuts turbulent friction more than laminar, so a rough estimate
  puts the mix 2% (Mach 1.5) to 13% (4.63) higher: +0.001 to +0.004, which §7's remainders leave in (Inference).

## 4. The grit strips

The tunnel fixed transition with 1/16-in (1.6 mm) bands of No. 60 carborundum, 3/4 in behind the nose and 1/2 in
behind each fin's leading edge [1] p. 4. The nose band is on both models and cancels in the increment.

- **Grain height.** Braslow, Hicks and Harris measured No. 60 trips: 12%, 48%, 34% and 4% of grains in the 4–8, 8–12,
  12–16 and 16–20 thousandths-of-an-inch bins, a mean of 0.011 in (0.28 mm) [3] Fig. 4, p. 14 (read by eye at
  400 dpi, ±2 points).
- **The method.** Stallings and Lamb sized trip drag in the same tunnel, Mach 1.5 to 4.63, and their smallest grit,
  0.027 cm, was laid as here: random grains in a 0.159-cm band parallel to the edge [4] pp. 5–6. Their eqs. 4–5 (p. 13;
  appendix pp. 18–20) give ΔC_D from strip length, grain height, sweep and Mach number; k over the displacement
  thickness sets which applies; grains sit four heights apart. Its predictions "agree well" with their data from Mach
  1.5 to 3.95, and read low at 4.63 at the lower Reynolds number [4] p. 14.
- **Inference, applied:** both faces of four fins (assumed), strips parallel to the 30° edge, 49.3 cm in all; k =
  0.028 cm; a compressible laminar displacement thickness at 12.7 mm of 0.0095 cm (Mach 1.5) to 0.037 cm (4.63). Grit
  adds **0.005, 0.005, 0.004, 0.002, 0.002 and 0.001**, and 0.0003 to 0.008 between one face at k = 0.020 cm and both
  faces at 0.035 cm.

## 5. The fins over the boattail

Each root's aft 1.54 in (39 mm) follows the 15° boattail; the ridge sits 1.84 in ahead of the base and the boattail
starts 1.81 in ahead [1] Fig. 1, p. 8. So the fins' rear wedges, where the pressure is below ambient, lie over it.

- **Their pressure field on the boattail (Inference, linear theory).** Each fin face's 2θ/β pressure, carried across
  the body surface as in planar flow (shifted aft by β times the distance from the face), integrated over the boattail
  between neighboring fins: **+0.013, −0.002, −0.008, −0.007, −0.004 and −0.003**. At Mach 1.5 the expansion over the
  boattail dominates (drag); faster, the front wedges' compression lands on it (thrust). Planar spreading ignores the
  body's curvature and the separation the report sees at higher Mach [1] p. 6, so this is a scale, not a bound.
- **The root strip on the boattail.** The design takes the span from the cylinder [7]; the wedge of fin between the
  cylinder line and the boattail adds 0.32 in² per fin, +5.5% of planform: +0.005 at Mach 1.5 to +0.002 at 4.63.
- **The body's boundary layer.** A turbulent layer at the root station is about 0.56 in (14 mm) thick incompressible,
  more at Mach 2 to 4: the inner 26% to 43% of the span, 30% to 48% of the fin area, at 65% to 78% of the free-stream
  dynamic pressure (1/7-power profile). Area times the lost dynamic pressure takes 7% to 17% off fin drag: −0.006
  to −0.016 at Mach 1.5, −0.003 to −0.006 at 4.63. DATCOM assumes unit dynamic pressure at tail panels [6]
  p. 4.5.3.1-10.

## 6. Edges, thickness and anchors

- **Leading-edge radius: unbounded, but sized per unit.** The drawing shows sharp edges and gives no radius [1] Fig. 1.
  A round edge by modified Newtonian theory at the normal Mach number of the 30° edge adds **0.0031 per 0.001 in (0.025
  mm) of radius at Mach 1.5 to 0.0038 at 4.63** (Inference). A blunt trailing edge 0.004 in thick adds about 0.003 at
  Mach 1.5, from the base pressures in the parent note's §4.
- **Fin thickness stands at 0.150 in.** TN D-4013's rear view labels ".04d" with a leader that ends on the fin anchor,
  not the fin ([2] Fig. 1(b), p. 10, read at 500 dpi). TN D-4014 dimensions the section 0.150 in at the root and 0.096
  in at the tip ([1] Fig. 1, p. 8). This answers [junction-drag](junction-drag.md)'s open question: no change.
- **Anchors.** The short model kept its fin anchors in the slots with fins off; the long one had them faired flush
  [2] pp. 3–4. TN D-4014 doesn't say, so this note assumes the same. The anchors then cancel in the short increment and
  sit in the long one; that the two increments agree within ±0.005 from Mach 2.3 (§2) bounds anchor drag by about that.
- **Sweep, finite span, second order:** under 5% of the wave drag, as the parent note found.

## 7. What remains

Inference, on the reference area. "Model" is linear wave drag plus fin-chord friction; the remainder subtracts §4–5's
terms from the lip-free increment. Its range spans the increment's ends and the root-strip and body-layer column's
ends, with grit and the boattail term at their central values; the trip's compressible share (§3) would take off a
further 0.001 to 0.004. "As edge radius" divides the remainder by §6's drag per 0.001 in.

| Mach | lip-free increment | model | grit | over the boattail | root strip, body layer | remainder | as edge radius |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 1.5 | 0.174–0.189 | 0.093 | +0.005 | +0.013 | −0.011…−0.001 | 0.077 (0.064–0.089) | n/a |
| 1.8 | 0.125–0.137 | 0.079 | +0.005 | −0.002 | −0.009…−0.001 | 0.054 (0.044–0.064) | n/a |
| 2.3 | 0.085 | 0.066 | +0.004 | −0.008 | −0.008…−0.001 | 0.027 (0.024–0.031) | 0.008 in |
| 2.96 | 0.061 | 0.054 | +0.002 | −0.007 | −0.006…−0.001 | 0.016 (0.013–0.018) | 0.005 in |
| 3.96 | 0.050 | 0.042 | +0.002 | −0.004 | −0.005…−0.001 | 0.013 (0.011–0.015) | 0.004 in |
| 4.63 | 0.046 | 0.037 | +0.001 | −0.003 | −0.004…−0.001 | 0.013 (0.012–0.015) | 0.003 in |

- **From Mach 2.96 up the remainder is plausibly fin drag.** It is flat at 0.013 to 0.016, as an edge's drag is, and a
  0.003 to 0.005 in radius (0.08 to 0.13 mm) on a machined edge would supply it. Nothing here measures that radius.
- **At Mach 1.5 and 1.8 it is not.** An edge would need 0.017 to 0.025 in. The remainder falls fivefold from Mach 1.5
  to 2.96, twice as fast as wave drag's 1/β. The report says the fins thicken the boundary layer at the base [1]
  p. 6; how that changes the boattail's own drag is unmeasured, so this note lists it as the leading candidate,
  unsized.
- **Increment error, linear wave drag plus chord friction:** −39%, −29%, −23%, −11%, −16%, −20% against the measured
  increment (−47% to −51% and −37% to −42% at Mach 1.5 and 1.8 against the lip-free one), from the parent's −35%
  to −52%.
- **The fins-on rows get worse.** With chord friction they read +2.9%, +2.4%, +8.1%, +12.3%, +14.4% and +11.9%, because
  the fins-off body reads high. Closing the fins exposes the body: the fins-off excess at Mach 1.5 and 1.8 rises to
  +30.0%…+36.4% and +18.5%…+23.4% lip-free (the long model reads +22.3% at 1.8).

## 8. TM X-2774 can't separate these

The lead from [ironbark](ironbark.md) §4: fin sets T9 to T12, equal in exposed area (43.1 cm² a panel) and thickness
(1.016 cm), at Mach 1.60, 2.36 and 2.86 [5] p. 4, Table I (p. 10). No fins-off run, and the fin balances measured
normal force, bending and hinge moments only [5] pp. 4–5: no fin increment, no interference term.

## What this suggests

- **Cache:** [3] to [5] in gitignored `refs/research/arcas-fin-increment-leftover/` (NTRS: public use permitted).
- **Suggestion 1, pin the lip-free increment ([#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70), core
  band).** Score supersonic fin drag on the increment, with Mach 1.5 and 1.8 carried as ranges (0.174–0.189,
  0.125–0.137), not on the fins-on rows, which pass by cancellation. Expected effect: no code change; the Mach 1.5 and
  1.8 misses grow and are recorded, not scored as met.
- **Suggestion 2, fin friction on the fin's chord.** Take each fin set's friction Reynolds number on its mean
  aerodynamic chord, per DATCOM [6]. Expected: Arcas fin drag +0.010 to +0.020 here, +0.023 at Mach 0.6 (junction-drag);
  higher fin drag, lower simulated apogees on every finned rocket. Pin it with the Arcas increments, Mach 0.6 to 4.63.
- **Suggestion 3, a supersonic trip-drag allowance in the comparison only.** Add Stallings and Lamb's eqs. 4–5 to the
  tunnel rows, not to flights: +0.001 to +0.005 on the increment. Expected: a cleaner reference, no flight change.
- **Not suggested:** an edge-radius default fitted to the remainder; the reference would agree by construction.

## Open questions

- What makes 0.04 to 0.09 of drag at Mach 1.5 and 1.8 with fins on? A pressure-tapped finned boattail at Mach 1.5 to 2
  would size the fins' effect on boattail drag; none was found. NACA's RM series is the place to look.
- The fins' leading-edge radius (another report on these models may give it), and how TN D-4014 plugged the slots.
- Whether the raw C_A truly holds still when C_A,c dips (§2); the lip correction's size rests on it.
- Cruciform panel-to-panel interference (one fin's waves reaching the next) is not sized here, nor is either sign known.

## Sources

All read 2026-10-20. Printed page numbers unless "PDF".

1. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967: pp. 2–6; Fig. 1, p. 8; Fig. 4(a), p. 21.
   <https://ntrs.nasa.gov/citations/19670020031> (cached `refs/papers/nasa-tn-d-4014.pdf`).
2. J. C. Ferris, NASA TN D-4013, 1967: pp. 3–6; Fig. 1(b), p. 10. <https://ntrs.nasa.gov/citations/19670020050>
3. A. L. Braslow, R. M. Hicks, R. V. Harris Jr., *Use of Grit-Type Boundary-Layer-Transition Trips on Wind-Tunnel
   Models*, NASA TN D-3579, 1966: Fig. 4, p. 14. <https://ntrs.nasa.gov/citations/19660026829>
4. R. L. Stallings Jr., M. Lamb, *Effects of Roughness Size on the Position of Boundary-Layer Transition and on the
   Aerodynamic Characteristics of a 55° Swept Delta Wing at Supersonic Speeds*, NASA TP-1027, 1977: pp. 5–6, 13–14,
   18–20. <https://ntrs.nasa.gov/citations/19780005073>
5. C. D. Trescot Jr., G. V. Foster, C. D. Babb, NASA TM X-2774, 1973: pp. 4–7, Table I (p. 10).
   <https://ntrs.nasa.gov/citations/19730018268>
6. R. D. Finck, *USAF Stability and Control DATCOM*, 1978 (AFWAL-TR-83-3048): pp. 4.3.3.1-17, 4.3.3.1-18,
   4.5.3.1-10. <https://archive.org/details/DTIC_ADB072483>
   (cached `refs/papers/usaf-datcom-1978-afwal-tr-83-3048.pdf`).
7. The project's fixtures `validation/fixtures/aero/drag-vs-mach.json` and `arcas-robin-wind-tunnel.json`, the
   [aero page](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel), and `crates/hpr-aero/src/drag.rs`.
