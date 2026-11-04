# Installed airbrake drag: brakes on bodies against Vick's wall charts, below Mach 1

What this covers: how the drag of a speed brake mounted on a body of revolution compares with the same brake on a flat
tunnel wall (NACA RM L57J23a, Vick 1958), below Mach 1, and how that ratio moves with brake height over the boundary
layer, body diameter, nearby fins or wings and the base. It is for M6.4's installed drag range and for which way a
braked apogee errs. How far to trust it: **one clean body-of-revolution tunnel test was found**, at Mach 0.12, with
plates standing off the skin; the rest are aircraft fuselage brakes near wings and two hobby rockets (one flight, one
whole-rocket tunnel test), each one team's data with no stated uncertainty. Chart values are read by eye, about ±0.03
in C_D. Builds on [airbrake-drag.md](airbrake-drag.md), which covers Vick's charts, the D-558 brake, the rocket CFD and
the ARIS flight. Researched 2026-11-04.

## Conventions

- **On brake area**: added drag over q times the brakes' total projected frontal area; **on body area**: over q times
  the body's cross-section. "Ratio" is installed over Vick's wall value at the same deflection α, aspect ratio A (width
  over chord) and Mach.
- δ₉₅ is Vick's boundary-layer thickness; δ₉₉ ≈ 1.4 δ₉₅. Where a source gives no δ, it is estimated
  here with the turbulent flat-plate δ₉₉ ≈ 0.37 x Re_x^(−0.2) from the nose tip, which ignores the nose's
  pressure gradient and the thinning from transverse curvature (*inference* throughout).
- Vick's peaks used here: 90° 1.27 (A = 0.5) and 1.10 (A = 1) at Mach 0.2, 1.52 and 1.38 at Mach 0.9; 60° 1.12 (A =
  0.5) and 0.97 (A = 1) at Mach 0.2; 30° about 0.75 (A = 0.5) and 0.64 (A = 1). His sting-mounted plate read 1.04 at
  Mach 0.25.

**In short.** On the one clean body (ARC C.P. 251), plates far from the base read 1.26 to 1.37 on projected area at
90°, about 1.15 to 1.3 times Vick, and 1.10 to 1.19 times a free plate in the same tunnel, against 1.03 to 1.06 for
Vick's wall brake over his own sting plate. So a body at H/δ₉₅ ≈ 5 to 11 did not lower brake drag; if anything it
raised it (*inference*). Brake size over body diameter from 0.6 to 0.9 made no difference beyond ±3% from 50° to 90°.
Aircraft fuselage brakes read 0.47 to 1.17 of Vick, lowest in a wing's wake. Hobby rockets with known brake geometry
read 0.51 to 0.95 on brake area, 19% to 56% under the free plate's 1.17 and up to about 37% under Vick at high angles;
they sit at H/δ₉₉ ≈ 1 to 3, often between or just ahead of fins, where no body test exists. C.P. 251 does not
explain the low hobby values, but it tested only gapped plates well above the boundary layer (*inference*).

## 1. The clean body: ARC C.P. 251 (Fail, Owen and Eyre 1956)

- **Setup** (pp. 4 to 9, fig. 7). RAE 4 ft × 3 ft tunnel at 140 ft/s (Mach about 0.125). A 4.50 in body with an
  elliptic nose, a parallel section and a bluff, boat-tail or faired tail. One square flat plate of side c = 2/3 or 1/2
  of the diameter, hinged on a line tangent to the body with a gap of 0.2c (or 0.4c) between hinge and plate, at θ =
  30° to 130° to the axis. Coefficients on plate area.
- **Isolated plates, same tunnel** (pp. 7 to 8, fig. 2). A square plate at 90° read 1.15, and stalls near 36°.
- **On the body, more than about 2 chords from the base** (tables I to IV, gap 0.2c), on plate area: 0.56 to 0.68 at
  30°, 0.84 to 0.96 at 50°, 1.13 to 1.27 at 70°, 1.26 to 1.37 at 90°. On projected area (over sin θ): 1.13 to 1.36,
  1.10 to 1.25, 1.20 to 1.35, 1.26 to 1.37 (*inference*). A 7 in body with a 5 in plate peaked at 1.28 at 90° (p. 12;
  table V).
- **Against Vick** (*inference*). The hinges sit 21 to 40 in from the nose, so δ₉₅ ≈ 0.32 to 0.53 in, the plate's
  tip stands at H/δ₉₅ ≈ 5 to 11 and its near edge about 1 to 1.5 δ₉₅ off the skin. Vick's A = 1, Mach 0.2
  90° curve reads 1.10 at H/δ 6, about 1.03 at 8.5, and stops near 8.7 (fig. 12a): a ratio of about 1.15 to 1.3. At
  50° the ratio is 1.25 to 1.42; at 30°, 1.8 to 2.1.
- **Why small angles read so high** (*inference*). With the gap, air passes between plate and body, so the plate behaves
  as an inclined free plate (about 0.6 on plate area at 30°, near 1.17 sin θ), while Vick's flaps are sealed at the
  hinge. A flap hinged at the skin, or a blade pushed out through a slot, is Vick's case. C.P. 251 bounds the body
  effect only for brakes that stand clear of the skin, and only well above the boundary layer.
- **Brake size over body diameter.** Plate side over diameter 1/2 and 2/3 (tip height over diameter about 0.6 and 0.8
  with the gap) agree within ±3% from 50° to 90°; at 30° the smaller plate reads about 9% lower. The 7 in body
  (about 0.88) sits inside the same band at 90°.
- **Gap.** 0.4c instead of 0.2c lowers the 90° value by 1% to 3.5% and raises the 30° value by 9% to 15% (table II).
- **Near the base** (tables I, II, IV; p. 10). With the hinge about one chord or less from a bluff base, drag rose 16%
  to 48% at 90° and 70°, up to 62% at 50°; with a boat tail, 12% to 24% at 90°, 21% to 38% at 70°; with a faired
  tail, nothing. At 1.5 chords the rise was 0% to 5%. Beyond about 3 chords there was no effect. The rise vanishing with
  a faired tail points to lower base pressure (*inference*).
- **Under a wing.** The same plate under a Javelin model, just ahead of the wing's trailing edge, read 1.67 at 90°,
  which the authors put down to wing interference (p. 11): 1.45 times the free plate (*inference*).
- **Body pressures, ARC C.P. 288** (Owen 1956, same rig, pp. 1 to 5). Relative to the clean body: about +0.1 q (30°) to
  +0.6 q (90°) ahead of the brake and −1.0 q to −0.6 q behind it; at 90° the separated flow reattaches about 3
  chords (2 diameters) behind the hinge. On a parallel tube these act sideways and add no drag (*inference*), so any
  drag the body takes back from a brake must sit on a taper, the base or the fins. Shedding Strouhal number falls from
  0.25 at 50° to 0.13 at 90°, close to a free plate's (p. 5): a lead for fin buffet.

## 2. Aircraft fuselage brakes

All are aircraft brakes, most near a wing, with δ unknown; no report says whether the hinge is sealed. Ratios assume
the A and Mach given, and use Vick at Mach 0.5 for XS-1 and D-558 (*inference*).

| Source | Brake | On projected area | Ratio to Vick |
|---|---|---|---|
| NACA RM L8A12 (XS-1), Mach 0.4 to 0.9 | two side flaps at 65°, aft of wing | 1.10 to 1.19 (tail on) | 0.89 to 0.96 |
| same, horizontal tail off | | 1.19 to 1.34 | 0.96 to 1.08 |
| NACA RM L8B06 (D-558), to Mach 0.96 | two side flaps at 60° | 0.83 to 1.13 | 0.70 to 0.96 |
| NACA RM A56E22, Mach 0.8 to 0.9 | one flap under the fuselage, 60° | 1.23 to 1.55 | 0.93 to 1.17 |
| same | two side flaps in the wing's wake | 0.62 to 0.78 | 0.47 to 0.59 |
| NACA RM L56A25a, Mach 0.8 to 0.94 | two perforated side brakes, 56°, behind wing | 0.92 to 1.09 | 0.75 to 0.89 |
| NACA TN 1939, table I, Mach < 0.3 | four fuselage brakes, 60° to 90° | 0.63 to 1.04 | 0.57 to 0.95 |
| Vick's own overlay (fig. 12) | five fuselage points, Mach 0.2 to 0.9 | 1.02 to 1.27 | 0.87 to 1.00 |

- **XS-1** (fig. 9, by eye): ΔC_D 0.037 to 0.040 on wing area tail on, 0.040 to 0.045 tail off, rising about 8% to 12%
  from Mach 0.4 to 0.8, which the report calls essentially the same. The flap's true size is taken from the side view as
  a flap swung out 65° (*inference*; Vick's own reduction of it is 1.08 at Mach 0.5). The horizontal tail in the
  brakes' wake cut the increment by 7% to 13%; tufts showed no separation on the fuselage behind the brakes (p. 8).
- **A56E22** (pp. 11 to 12, fig. 21a): the single under flap, half the frontal area, gave increments within 10% of the
  two side flaps; the report puts the side flaps' loss down to the wing wake's lower dynamic pressure.
- **L56A25a** (pp. 4 to 5): about 0.03 on wing area against the authors' flat-plate estimate of 0.045; the brakes raised
  pressure on the wing's aft part, cutting wing drag. Flat to Mach 0.94, then down about a third by 1.0.
- **TN 1939** (table I, p. 20) gives no brake geometry; A = 1 is assumed. Fig. 8: a side brake rises 22% from Mach 0.3
  to 0.875, and a dive-recovery flap reaches "161 percent of its low-speed value" at Mach 0.8 (p. 15).
- **NASA TM X-188**: an under-fuselage brake gave essentially the same increment at four stations along the fuselage
  (pp. 5 to 6). **NACA RM L56I05**: brakes out gave the lowest base pressures; its increments exclude that base drag (p.
  5), so its size is not known here.

## 3. Hobby rockets with brake geometry

- **Monash test rocket, one flight, 2019** [9]. A 65 mm body, AeroTech H135W, three curved aluminum flaps 80 mm long
  between the three fins, hinged at their forward end, opened about 30° (pp. 8 to 19). Projected area 50.2 cm², 1.5
  body areas (*inference*: 41.8 mm chord width × 40 mm out, three flaps). A fit made here (*inference*) of the report's
  acceleration and velocity plots (figs. 4.2.4, 4.2.5, digitized; burnout mass from ThrustCurve) gives 0.77 to 0.95 on
  body area, **0.51 to 0.63 on brake area** (0.58 to 0.72 if the flaps opened only 26°), at Mach 0.24 to 0.32. The
  report's own two-point forces (p. 27) give 0.77 on brake area at an assumed air density (*inference*), but take 98 m/s
  at 3 s where its velocity plot reads about 109 m/s. Its CFD forces give 0.64 (whole rocket, *inference*), and the
  report calls the CFD high (p. 21). H/δ₉₉ ≈ 2.5 (H/δ₉₅ ≈ 3.7). Against Vick at 30°, A ≈ 0.5 (0.75 to
  0.78): 0.65 to 0.84 by the fit. The open slots in the fin can are part of the measured increment.
- **University of Maryland "Karkinos", 2023** [10, 11]. A 157 mm rocket, four flaps each 45° of arc and 5 in long,
  hinged forward, directly above the fin can. The whole rocket went on a sting in the 2.5 m Glenn L. Martin tunnel,
  flaps 0° to 80°, about 20 to 80 m/s (pp. 59, 61; fig. 84). One constant fits the drag: C_D = 0.95 on the team's 4 ×
  10 in² × sin δ, R² = 0.996 (p. 59); on the 11.8 in² projected flap area rebuilt from the 2022 geometry, **0.80**
  (*inference*). Against Vick (*inference*), 0.95 matches him near 45° and reads 0.75 of him at 80°; 0.80 matches him
  near 35° and reads 0.63 at 80°. A single constant fitted over all points is dominated by the high-angle, high-speed
  points, so it does not test Vick's sin² α law, and the per-angle points are not tabulated. H/δ₉₉ ≈ 0.9 to 3.3
  in the tunnel. A braked flight to about 43° (2,922 m, against 3,164 m with brakes disabled on a separate
  student-built motor) is reproduced by their model with 0.95 (p. 60, fig. 85).
- **ARIS, 2019** ([airbrake-drag.md](airbrake-drag.md)): three blades at 90°, 0.62 to 0.92, H/δ₉₅ ≈ 1.5.
- **Body area only.** Utah State 2008 [12]: a 1:3 model's wake survey gave 0.41 brakes in and 1.6 out, and a flight 0.32
  and 1.55: increments of 1.19 and 1.23 on body area, with no brake size stated. A University of New Hampshire capstone
  (2026) [13] tested printed models at Re ≈ 5 × 10⁴: 1.04 in and 2.17 out; the retracted value, twice a normal
  body's, suggests the mount dominates (*inference*).
- **A bias in braked logs.** Maryland found a barometer inside the brake module reads high as the flaps open, modeled as
  0.052 q sin δ (2023 report, pp. 51 to 55). A drag fit from such a barometer carries it.
- **Stability.** No record of buffet or a measured margin change with brakes out was found; Monash's CFD at 10° shows
  larger fin forces with flaps out (pp. 22 to 23).

## 4. What the ratio depends on

- **H/δ on a body**: no source varies it. The only clean body sits at H/δ₉₅ ≈ 5 to 11; hobby brakes sit at
  H/δ₉₉ ≈ 1 to 3. Nothing is measured on a body below H/δ₉₅ ≈ 2, where Vick's curves fall.
- **Brake height over body diameter**: no trend at 0.6 to 0.9 (C.P. 251); XS-1 (about 0.4) and A56E22 (about 0.5), clear
  of a wing wake, sit at 0.89 to 1.17 of Vick. Hobby brakes span about 0.2 (ARIS) to 0.6 (Monash; Maryland at 43°), so
  only the smallest lie below every test (*inference*).
- **Lifting surfaces**: a brake in a wing's wake read 0.47 to 0.89 of Vick; a tail in the brake's wake lost 7% to 13% of
  the increment (XS-1), the same sign as the WVU CFD's 8% to 15% in airbrake-drag.md. On a rocket only the fins lift; no
  tunnel test has brakes between them, and both new hobby sets have fins at or just behind the brakes.
- **Base**: within about one brake chord of a bluff base, +16% to +62%; none with a faired tail (C.P. 251).
- **Mach**: installed brakes rise 0% (L56A25a to 0.94) to 12% (XS-1) and 22% (a TN 1939 side brake) by about Mach 0.9,
  and one TN 1939 dive-recovery flap 61% by 0.8; the D-558 falls about 10%. Vick's walls rise 16% to 44%. The
  installed-to-wall ratio has no single trend with Mach (*inference*).

## What this suggests

- **Carry the range, and widen it down.** Measured installed values on brake area for hobby rockets now run 0.51 to 0.95
  (Monash flight, Maryland tunnel, ARIS flight), against airbrake-drag.md's 0.62 to 1.18. A brake model following Vick
  or the free plate reads high for them, predicting a braked apogee too low: the flattering side for a waiver or range
  ceiling. For a ceiling check the low end of the band is the conservative side; three data sets are too few to fix a
  default (*inference*).
- **The body alone did not lower drag where it was tested.** On a clean body well above the boundary layer, gapped
  plates read at or above the wall (1.15 to 1.3 at 90°). The low hobby values come with H/δ below about 3, fins at or
  behind the brakes, open slots and fits of one flight; any of them can explain it, and nothing here separates them.
- **Which way it errs depends on where the brake sits.** Ahead of a blunt base or clear of fins at large H/δ, Vick can
  read low (the conservative side); between or just ahead of fins at small H/δ, it has read high (flattering) in the
  cases measured. A model should say which case it assumes.
- **Mach.** No single factor covers installed brakes (0% to 61% by Mach 0.8 to 0.9; Vick's walls 16% to 44%).
- **Flight fits.** A braked flight's drag fit should take its velocity from the integrated log rather than read points,
  and keep the barometer near the brakes out of the fit, or correct it.

## Open questions

- No sealed flap or slot-fed blade on a body of revolution below H/δ₉₅ ≈ 3 was found, and none between fins. A
  whole-rocket tunnel test of the Maryland kind, tabulated per angle, would settle the angle law and the level.
- Maryland's per-angle points, its true 2023 flap area (10 in² stated, 11.8 in² from the 2022 geometry), and Monash's
  flap angle in flight (26° to 30°) are not known.
- The base-pressure drop with brakes out (NACA RM L56I05) is known in sign only; so is fin buffet behind brakes.
- No missile or bomb speed-brake tests below Mach 1 were found; ARC R&M 2614 (p. 4) gives body flaps in words only.
- Not reached: Notre Dame's 2019 Student Launch flight readiness review (server refused connections); the AerospaceNU
  braked-flight log has no brake geometry.

## Sources (all read 2026-11-04)

NACA and NASA reports are US government works. ARC papers are Crown copyright (1956 and 1942, likely lapsed) and team
reports state no license: cited only.

1. R. Fail, T. B. Owen, R. C. W. Eyre, ARC C.P. 251 (1956), pp. 4 to 11, tables I to V, figs. 2, 7, 18:
   https://reports.aerade.cranfield.ac.uk/handle/1826.2/264
2. T. B. Owen, ARC C.P. 288 (1956), pp. 1 to 5: https://reports.aerade.cranfield.ac.uk/handle/1826.2/301
3. A. T. Mattson, D. L. Loving, NACA RM L8A12 (1948), pp. 3 to 8, figs. 1, 9:
   https://ntrs.nasa.gov/citations/19930085331
4. J. L. Summers, S. L. Treon, L. A. Graham, NACA RM A56E22 (1956), pp. 11 to 12, table I, figs. 11, 21(a):
   https://ntrs.nasa.gov/citations/19930089412
5. D. D. Arabian, NACA RM L56A25a (1956), pp. 1 to 5, figs. 1, 2, 6(a): https://ntrs.nasa.gov/citations/19930089201
6. J. D. Stephenson, NACA TN 1939 (1949), pp. 14 to 15, table I, fig. 8: https://ntrs.nasa.gov/citations/19930082614
7. R. T. Taylor, NASA TM X-188 (1959), pp. 5 to 6: https://ntrs.nasa.gov/citations/19980235623; F. E. West, C. C.
   Critzos, NACA RM L56I05 (1956), pp. 4 to 5: https://ntrs.nasa.gov/citations/19930089490
8. A. R. Vick, NACA RM L57J23a (1958), figs. 10, 12: https://ntrs.nasa.gov/citations/19930090297; H. Davies, F. N. Kirk,
   ARC R&M 2614, p. 4: https://reports.aerade.cranfield.ac.uk/handle/1826.2/3152
9. S. Tandon, T. Mackellar, N. Tryndoch et al., "Design 1 Airbrakes Team, Submission 2", Monash University (2019), pp. 8
   to 29, figs. 4.2.4, 4.2.5: https://siddhant-tandon.com/files/Design1Report.pdf
10. H. Daniel, A. Bean, E. Bregin et al., Terrapin Rocket Team, 2023 Spaceport America Cup technical report, pp. 44 to
    61, table 16, figs. 84, 85: https://drum.lib.umd.edu/items/5555aa80-3f2d-480d-b7f9-b18af4293571
11. E. Bregin et al., Terrapin Rocket Team, 2022 Spaceport America Cup technical report, pp. 22 to 23:
    https://drum.lib.umd.edu/items/d6631eb4-dddd-4c62-95bf-eb313fe2770b
12. S. Eilers, T. Whitmore, Utah Space Grant Consortium conference (2008), pp. 5 to 7, figs. 6 to 8:
    https://digitalcommons.usu.edu/spacegrant/2008/Session3/1
13. C. J. MacLean, M. Wharton, N. Revis, C. Guido, UNH Honors Theses and Capstones 949 (2026), pp. 26 to 30:
    https://scholars.unh.edu/honors/949/
14. ThrustCurve, AeroTech HP-H135W entry (82 g propellant): https://www.thrustcurve.org
