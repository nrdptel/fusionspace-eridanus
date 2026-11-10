# Published methods for a thin fin's lift and center of pressure, Mach 0.8 to 1.2

**In short: three public methods agree that the Arcas Robin's thin, low-aspect-ratio fins gain 0% to 22% in lift
slope from Mach 0.8 to 1.0, where HPR Sim's straight join gains 43%. Put in place of the join, each moves the
rocket's center of pressure (CP) forward: 0.2 to 0.6 caliber at Mach 0.9, 0.4 to 1.0 caliber at 1.0.** That closes
16% to 44% of the +1.34-caliber gap to NASA TN D-4013 at Mach 0.9 (#1198), not the rest. At Mach 1.0 it turns HPR
Sim's −0.41 into −0.84 to −1.38 caliber, a margin read too low. The methods are USAF DATCOM's transonic wing charts,
slender-wing theory with its similarity limits, and a NACA bump test of 56 small wings. None shows what the tunnel
measured, a fin share of lift that falls 18% from Mach 0.6 to 0.9 and then jumps 83% at 1.0, so that comes from
something the wing-alone methods leave out. Confidence: high on direction and on the stated validity limits; medium
on sizes, read by eye off scanned charts. **Inference** marks this note's own arithmetic and readings. It builds on
[transonic-normal-force](transonic-normal-force.md) and [dense-transonic-fin-body](dense-transonic-fin-body.md), and
moves [#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198), the stop rule of
[M1.14c](../decisions-and-roadmap.md#m1-14c) and the 0.5-caliber CP target in [accuracy](../accuracy.md).

## 1. The straight join and the Arcas fins

- **The join.** Below Mach 0.8 each fin takes Diederich's slope with Prandtl–Glauert at its quarter mean aerodynamic
  chord (MAC); from M_s, 1.2 here, supersonic linear theory; between, slope and CP each a straight line in Mach
  (`crates/hpr-aero/src/fins.rs`, `FinAero`; [aero](../physics/aero.md#normal-force-through-mach-1)).
- **The fins** ([1] Fig. 1(b), printed p. 8): four trapezoids, root 3.380 in, tip 2.165 in, span 2.103 in, leading
  edge swept 30°, trailing edge unswept, on a 2.250 in body. Double wedge, 0.150 in thick at the root and 0.096 in at
  the tip, 4.4% at both, as in `wind-tunnel-arcas-robin-short.json`. The tunnel report [6] draws it about 3% smaller.
- **Derived (Inference).** Two panels joined: aspect ratio A = 1.517, taper λ = 0.64, mid-chord sweep 16.1°,
  A tan Λ_LE = 0.876, transonic parameter Ā = A (t/c)^(1/3) = 0.536. MAC 0.833 c_r; quarter MAC 0.375 c_r aft of the
  root leading edge.
- **The join's numbers (Inference,** from `fins.rs`'s formulas; the body's 1.90 per rad, backed out of the aero
  page's totals 10.85, 12.77, 14.69, agrees with its body column, 1.90 to 1.92). On the panels' planform: 2.26, 2.75,
  3.23 per rad at Mach 0.8, 0.9, 1.0, 4.20 at M_s; CP 0.375, 0.405, 0.435 c_r, 0.495 at M_s. A 43% rise to Mach 1.0.

## 2. USAF DATCOM: transonic lift slope (4.1.3.2-B)

DATCOM builds a straight-tapered wing's transonic slope from charts fitted to tunnel data [2]. It finds a force-break
Mach number M_fb, scales the subsonic slope there, sets points at M_fb + 0.07 and M_fb + 0.14, and fairs to Mach 0.6
and 1.4 (Steps 1–8, PDF pp. 513–514). Very thin or very low-aspect-ratio wings ("type B") lack thick wings' dip
(PDF p. 512).

- **Read for the Arcas (Inference, by eye).** M_fb = 1.00: 53a's A = 2 curve sits at 1.0 below about 10% thickness,
  and the sweep correction (53b) leaves it there (PDF p. 553). Ratio at M_fb 1.08, read 1.065 to 1.105 (54a, between
  A = 1 and 2); a/c = −0.11 (54b), so the slope *rises* 11% to Mach 1.07; b/c = 0.08 (54c) (PDF p. 554).
- **The subsonic slope** is Method 1's chart, Fig. 4.1.3.2-49 (PDF p. 503); its closed form gives DATCOM's own sample
  (1.433 against 1.435 per rad). Arcas: 2.18 at Mach 0.6, 2.25 at 0.8, 2.30 at 0.9, 2.35 at 1.0 (Inference).
- **Result (Inference), per rad on planform:** 2.54 at Mach 1.00 (2.51 to 2.60 over the ratio's reading), 2.82 at
  1.07, 2.34 at 1.14. Faired from Mach 0.6: 2.25 to 2.34 at 0.8, 2.30 to 2.44 at 0.9 (the subsonic chart alone, or
  the ratio grown linearly from 0.6; a straight line from 2.18 to 2.54 would give 2.36 and 2.45).
- **How well it fits.** Table 4.1.3.2-B (PDF p. 543) prints chart against test for six 4%-thick wings of TN 3671
  (A 1.0 to 2.67). At M_fb the chart reads −11% to +5%, low on the four with A ≤ 1.71. At Mach 1.07 five rows agree
  within 6%; the A = 1.71 wing reads 3.1 against a measured 2.0, unexplained.
- **Stated range.** "Limited to wings having symmetrical airfoils of conventional thickness distribution", at zero
  angle of attack (PDF p. 513). Fig. 53a has curves for A = 2 to 6 and more, 54a and 54b for A = 1 to 8; thickness 0 to
  12%. **The Arcas is outside on two counts:** A = 1.52 is below 53a's lowest curve, and a double wedge is not a
  conventional section. Its thickness and sweep are inside.

## 3. USAF DATCOM: transonic aerodynamic center (4.1.4.2-B)

Fig. 4.1.4.2-30 plots x_ac/c_r against Ā and A tan Λ_LE at four values of V = (M² − 1)/(t/c)^(2/3), for taper ratios
0 to 1 [2] (PDF pp. 674, 691–694). The rectangles' charts come from McDevitt's correlation [5]; TN 3671 [4] is among
the tapered charts' data (PDF pp. 673, 683), so §5 is not independent of DATCOM.

- **Mach grid (Inference).** At 4.4% thick, V = −2, −1, 0, +1 fall at Mach 0.866, 0.936, 1.000, 1.060; 0.8 is −2.9.
- **Read for the Arcas (Inference, ±0.03 c_r).** Its unswept trailing edge puts it between the λ = 0.5 unswept-edge
  curve and the rectangle. Each one's offset from its own quarter MAC, weighted by λ, added to 0.375 c_r: 0.32 c_r at
  Mach 0.866, 0.34 at 0.936, 0.37 at 1.00, 0.40 at 1.06. So 0.32 to 0.375 at 0.8 (between V = −2 and the subsonic
  quarter MAC), about 0.33 at 0.9 and 0.37 at 1.0, where the join has 0.375, 0.405, 0.435.
- **Check (Inference).** At Ā = 0 the charts give two thirds of the tip's leading-edge station, slender theory's (§4).
- **Stated range: the Arcas is excluded.** Uncambered straight-tapered wings, symmetric sections, low angles; "wings
  with cambered airfoils, double-wedge airfoils, and airfoils with blunt trailing edges show different transonic
  moment and lift characteristics" (PDF p. 673). No chart covers a double wedge.

## 4. Slender-wing theory and transonic similarity

Spreiter's NACA Report 1153 [3] shows that linear theory holds for wings of small aspect ratio through the whole
transonic range, failing only in a Mach band around 1 that "diminishes to zero as the aspect ratio, thickness ratio,
and angle of attack" go to zero (printed p. 1061). In that limit the slope is independent of Mach and of thickness:
C_Lα = (π/2) A for any wing whose trailing edge has no cutout ahead of its widest station (pp. 1067–1068).
McDevitt's Report 1253 [5] puts a number on "small": at sonic speed, slender-body (here slender-wing) theory is
adequate for rectangular, symmetrical wings when A (t/c)^(1/3) is below 1 (summary, printed p. 1355).

- **For the Arcas (Inference).** Ā = 0.536, inside McDevitt's bound; the trailing edge is unswept, so π/2 holds:
  C_Lα = 2.38 per rad at every Mach, the value HPR Sim's subsonic formula reaches at Mach 1. Only stations where the
  span grows carry lift, here the first 0.36 c_r, so the fin alone has its CP at two thirds of that, 0.24 c_r.
- **Stated range.** McDevitt's bound is for rectangles, so the trapezoidal Arcas is outside by planform; the theory
  has no section term. It is the zero-Ā edge of §3's charts (0.32 to 0.37 c_r here): a forward bound on CP.

## 5. A bump test of nearby wings: NACA TN 3671

Emerson tested 56 semispan wings on the Ames 16-ft tunnel's transonic bump, from Mach 0.60 to 1.10 at 1.85 to 2.90
million on MAC [4] (PDF pp. 3–4). They are deltas of A 2 to 4, clipped to λ 0.1 to 0.5, so every clipped wing has
an unswept trailing edge, like the Arcas's. Sections are NACA 63A002 to 63A006 and 63A(1.5)04 (Fig. 1, PDF p. 11).
Pitching moment is taken about the quarter MAC. The bump's boundary layer was about 3/4 in thick, and neither it
nor the fence's leakage was corrected (PDF p. 6).

- **Nearest wings:** 4% thick, λ 0.3 to 0.5, A 1.0 to 2.15 (Table I, PDF p. 9); leading edges swept 45° to 63°,
  derived from the base deltas' A by tan Λ_LE = 4/A (Inference).
- **Lift slope (Inference,** Fig. 17, PDF p. 183, ±0.002 per deg by eye): near A = 1.5 about 0.039 per deg at Mach
  0.70 and 0.040 to 0.043 at 0.98, so 2.2 to 2.5 per rad, up 0% to 10%. DATCOM's Table 4.1.3.2-B prints this report's
  Mach 1.0 values: 1.9 (A 1.08), 2.8 (A 1.67), 2.9 (A 1.71). At A 1.52, interpolation between rows gives 2.42 to
  2.6 and extrapolation from A 1.71 and 2.46 gives 2.7; §6 uses 2.45 to 2.7.
- **Center (Inference,** Fig. 19, PDF p. 185): dC_m/dC_L of −0.01 to −0.05 at Mach 0.70 and 0.98 puts it 0.01 to
  0.05 MAC aft of the quarter MAC (0.38 to 0.42 c_r on the Arcas), so Barrowman's fixed quarter MAC holds to Mach
  0.98. At Mach 1.10 it sits 0.03 to 0.12 MAC aft.
- **Stated range.** A 0.86 to 4.0, λ 0 to 0.5, Λ_LE 45° to 63°, 2% to 6% thick, Mach 0.60 to 1.10, NACA 63A sections.
  The Arcas's A and thickness are inside; **its taper 0.64, sweep 30° and double wedge are outside.**

## 6. Worked by hand: the rocket's CP against the join

Each method's fin slope and CP replace the join's; everything else stays as HPR Sim has it: the body's slope
(1.90 per rad) and moment, Barrowman's interference factor 1.3485 and four fins (Inference). Slopes are per rad: fin on
the two panels' planform, rocket on the body's cross-section. Shifts are calibers of 2.25 in, positive aft. A range
pairs the method's low slope with its forward CP and its high slope with its aft CP. The tunnel reads 9.92, 10.22,
15.79 per rad at Mach 0.8, 0.9, 1.0 ([aero](../physics/aero.md#normal-force-through-mach-1), read from [6]).

| Mach | method | fin slope | fin CP, c_r | rocket C_Nα | CP against the join | CP against the tunnel |
|--:|---|--:|--:|--:|--:|--:|
| 0.8 | straight join | 2.26 | 0.375 | 10.85 | 0 | +0.91 |
| 0.8 | DATCOM | 2.25 to 2.34 | 0.32 to 0.375 | 10.79 to 11.17 | −0.08 to +0.09 | +0.83 to +1.01 |
| 0.8 | slender wing | 2.38 | 0.24 | 11.33 | −0.03 | +0.88 |
| 0.8 | TN 3671 wings | 2.22 to 2.35 | 0.38 to 0.42 | 10.68 to 11.20 | −0.04 to +0.16 | +0.87 to +1.07 |
| 0.9 | straight join | 2.75 | 0.405 | 12.77 | 0 | +1.34 |
| 0.9 | DATCOM | 2.30 to 2.44 | 0.30 to 0.36 | 10.98 to 11.57 | −0.59 to −0.35 | +0.76 to +0.99 |
| 0.9 | slender wing | 2.38 | 0.24 | 11.33 | −0.56 | +0.78 |
| 0.9 | TN 3671 wings | 2.25 to 2.50 | 0.38 to 0.42 | 10.80 to 11.79 | −0.54 to −0.22 | +0.80 to +1.13 |
| 1.0 | straight join | 3.23 | 0.435 | 14.69 | 0 | −0.41 |
| 1.0 | DATCOM | 2.51 to 2.60 | 0.34 to 0.40 | 11.83 to 12.19 | −0.71 to −0.55 | −1.12 to −0.96 |
| 1.0 | slender wing | 2.38 | 0.24 | 11.33 | −0.97 | −1.38 |
| 1.0 | TN 3671 wings | 2.45 to 2.70 | 0.38 to 0.42 | 11.59 to 12.58 | −0.72 to −0.43 | −1.13 to −0.84 |

- **TN 3671 at Mach 1.0** keeps the Mach 0.98 center band; the Mach 1.10 one would move it 0.02 caliber aft.
- **The slope does most of it.** HPR Sim's body alone has its CP about 0.1 m *ahead* of the nose tip, so the
  rocket's CP follows the fins' share of lift. At DATCOM's middle values the slope gives −0.37 of −0.47 caliber at
  Mach 0.9 and −0.54 of −0.63 at 1.0; the fin's own CP about −0.1 (Inference).
- **Direction.** At Mach 0.8 the methods straddle the join. At 0.9 and 1.0 all three move the CP forward. At 0.9 that
  is toward the tunnel but leaves +0.76 to +1.13 caliber, the flattering side, outside the 0.5 target. At 1.0 it moves
  away from the tunnel, on the margin-low side. The rocket slope reads +6% to +15% over the tunnel at 0.9 (join +25%)
  and −20% to −28% at 1.0 (join −6.9%).

## 7. What none of the methods shows

The tunnel's fins-on less fins-off reading, on the same basis and interference factor (Inference, from the aero
page's table): 2.41 per rad at Mach 0.6, 2.15 at 0.8, 1.97 at 0.9, 3.59 at 1.0, a fall of 18% then a rise of 83%. The
methods give 2.2 to 2.6 throughout; DATCOM calls a dip thick or high-aspect-ratio behavior (PDF p. 512). The fins-off
readings ([aero](../physics/aero.md#normal-force-through-mach-1)) scatter over 1.47 per rad with no trend, a quarter
of the jump on the body's basis (6.43 per rad). Candidates for the rest, none bounded here:

- **The double wedge.** DATCOM says it behaves differently and gives no chart (PDF p. 673); Report 1153's double-wedge
  results are linear theory only, two-dimensional (Fig. 11) and triangular wings (Fig. 12), outside the band near
  Mach 1 [3].
- **The roots over the 15° boattail, and the fin anchors** ([6] Fig. 1(b), p. 10; pp. 3–4): outside any method.
- **The slotted-wall tunnel near Mach 1** ([6] p. 3), the 8-ft Transonic Pressure Tunnel, whose reflection band is
  unmeasured, unlike the 8-ft Transonic Tunnel's 4 to 36 in (Report 1389; [Arcas §3](arcas-short-model-mach-0-9.md)).

DATCOM's shape matches the other finned bodies (Inference): for the Arcas a peak at Mach 1.07, 21% to 25% over 0.8;
the Basic Finner at 1.07 to 1.10, 22% (tunnel, 0° roll) to 26% (range), 17% roll-averaged
([transonic-normal-force](transonic-normal-force.md), #619); AEDC-TR-75-125's square fin at 1.05, +21% (#841). The
join peaks at M_s, 1.2 here. On this evidence the Arcas, not the method, is the outlier.

## What the project can use

- **USAF DATCOM, 1978** [2]: a USAF contractor report, "approved for public release" (DTIC cover, PDF p. 1); cached at
  `refs/papers/usaf-datcom-1978-afwal-tr-83-3048.pdf`. The project may commit its own readings of Figs. 4.1.3.2-53
  and 4.1.3.2-54 and 4.1.4.2-30, with figure and page, not the charts.
- **NACA TN 3671** [4] and **Reports 1153** [3] and **1253** [5]: NACA works on NTRS, public use; cached under
  `refs/research/transonic-fin-lift-methods/src/`. TN 3671's lift and moment curves for 56 wings, Mach 0.60 to 1.10,
  could make a wing-alone fixture, where no body can hide a fin error.

## What this suggests

1. **Draw the fins' transonic slope from DATCOM 4.1.3.2-B** (M_fb, Ma, Mb in a committed table), scored first on the
   Basic Finner (#619) and AEDC-TR-75-125 (#841). On the Arcas: CP −0.35 to −0.59 caliber at Mach 0.9 (gap about +0.8
   to +1.0), −0.55 to −0.71 at 1.0 (gap about −1.0). It passes neither row.
2. **Keep #1198's warning whatever the model.** No public fin method closes the Arcas's Mach 0.9 gap, and the part
   left is on the flattering side.
3. **Leave the fin CP near the quarter MAC through Mach 1.** TN 3671 holds it within 0.05 MAC to Mach 0.98; the
   join's aft drift is worth about 0.1 caliber.
4. **Split the Arcas's fin share by model length.** If the long model's fins-on less fins-off reading shows the same
   dip and jump, the cause is at the tail (section, boattail, anchors) or in the tunnel, not the body. Its readings
   are already in `arcas-robin-wind-tunnel.json`.

## Open questions

- **A double wedge's transonic lift on a finite fin.** No public chart or test was found; it may be the Arcas's cause.
- **Interference.** All rows keep Barrowman's 1.3485. Slender wing-body theory (NACA Report 1307) splits lift
  differently; [fin-body-interference](fin-body-interference.md) sizes it subsonic, not here.
- **TN 3671's own curves** (Figs. 14, 16; PDF pp. 144–156, 170–182) hold every Mach point, 0.60 to 1.10; only the
  three-Mach summaries were read.
- **DATCOM's A = 1.71 row** at Mach 1.07, 3.1 against 2.0, is unexplained; with 2.9 and 3.0 measured either side, a
  typo is likely (Inference). TN 3671's Fig. 14 or 16 (the A 1.71 wing) would settle it.
- **Reynolds number.** TN 3671 ran 1.85 to 2.90 million on MAC, the Arcas tunnel about 0.70 million (3.0 million per
  ft [6] p. 1, on a 2.8 in MAC; Inference). No method here has a Reynolds term. TN 3501 [7] was cached, not read.

## Sources (all read 2026-11-05)

1. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967. <https://ntrs.nasa.gov/citations/19670020031>
2. R. D. Finck, *USAF Stability and Control DATCOM*, 1978 revision (AFWAL-TR-83-3048), §4.1.3.2-B, §4.1.4.2-B.
   <https://archive.org/details/DTIC_ADB072483>
3. J. R. Spreiter, NACA Report 1153, 1953. <https://ntrs.nasa.gov/citations/19930090973>
4. H. F. Emerson, NACA TN 3671, 1956 (supersedes RM A53L03). <https://ntrs.nasa.gov/citations/19930084950>
5. J. B. McDevitt, NACA Report 1253, 1955. <https://ntrs.nasa.gov/citations/19930092253>
6. J. C. Ferris, NASA TN D-4013, 1967. <https://ntrs.nasa.gov/citations/19670020050>
7. W. H. Nelson, J. B. McDevitt, NACA TN 3501, 1955 (cached, not read). <https://ntrs.nasa.gov/citations/19930084143>
