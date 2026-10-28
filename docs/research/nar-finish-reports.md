# What NAR's two R&D reports on surface finish measured

[fdm-and-tube-friction](fdm-and-tube-friction.md) found no test of k_s on FDM prints or paper tube, and named two NAR
R&D reports as possible finish series. This note reads both. **In short: one is a flight test of finish, the other is
not about drag at all.** Bock's NARAM-41 report (1999) flew five painted-and-waxed Estes Alphas and five coated with
sand on tape, each once on a B6-4, tracked from two stations. The sanded ones averaged 143.68 m against 175.18 m,
−18.0% (95% interval −12.3% to −23.6%), at a peak speed near 65 m/s and a body Reynolds number of 1.4 × 10⁶. By this
note's flight integration that is a drag rise of about +0.35 to +0.42 in C_D, from about 0.44 to 0.85 (×1.7 to ×2.1).
Sand-grain friction alone, at the fully rough law HPR Sim uses, gives +0.11 to +0.25 for 0.1 to 0.5 mm grains. So the
rise is at least what the model predicts. The NARAM-32 report (Snowden, indexed as Sowden, 1990) measured tube
strength: no drag. How far to trust it: the altitudes are measured, five a side, closure up to 8.8%; every C_D and k_s
is **Inference** from a point-mass flight. Neither report tests FDM, bare paper tube, or any finish in HPR Sim's bands.

## 1. What the index holds

- **The index lists Bock's report twice under one file.** The saved NAR R&D index (2026-10-17 copy) links `id=1357986`
  as "Drag vs Surface Finish on an Alpha" (G. Bock) and "Effects of Drag on Estes Alpha" (K. Bock), both NARAM-41
  [3]. The served file is Krysia Bock's report [1]; Greg Bock appears only in its photo caption (p. 2).
- **No other title names a finish.** The page holds 547 document links, 540 distinct ids (counted 2026-10-28); only
  Bock's title contains "finish". Surface, paint, sand, smooth, rough, wax or drag match 22 other titles; a selection
  follows, with Hampton's boundary-layer title (not opened). DeMar's tunnel series (`1357937`) is read already [10]:

| id | Title (author, meet, year) | Why it might bear on finish |
| --- | --- | --- |
| `1358268` | Effect of Turbulent Flow on Fin Boundary Layer (B. Hampton, NARAM 55, 2013) | boundary-layer state |
| `1358051` | Neat Spray Paint Techniques (H. Johnson, NARAM 44, 2002) | paint method; may be cosmetic |
| `1358102` | Paint Weight on Contest Rockets (R. Anthony, NARAM 47, 2005) | paint mass, likely not drag |
| `1358116` | Experimental Determination of Model Rocket Drag Coefficients (K. Steele, NARAM 48, 2006) | C_D method |
| `1358125` | Estimating and Verifying Drag Coefficients (C. Stevens, NARAM 49, 2007) | C_D method |
| `1358015` | Drag vs Airspeed (R. Chambers, NARAM 43, 2001) | Re dependence |

- **NARAM-32 is a strength test.** The title page reads Bryan Snowden, NAR 43949; the index, B. Sowden, `id=1357877`
  [2] p. 1, [3]. An Instron frame crushed 1.5 in BT-5 and 2 in BT-20 pieces at 0.5 in/min, three per cell, axially and
  radially: plain, peeled and CA-coated, peeled and MonoKoted [2] pp. 1–2. Axial failure on BT-5 was, for example,
  28.5 lb plain and 40.5 lb CA [2] p. 5. No roughness, flow or drag was measured. **It answers the question "no".**

## 2. Bock 1999: the method

- **Rockets.** Twelve Estes Alpha kits, built to the instructions with streamers instead of parachutes and no decals;
  ten flew in the test; one (p. 4) or two (p. 3) picked the motor. The current kit is 31.2 cm long, 25 mm across,
  with balsa fins [4]; this note takes a BT-50 tube, 24.8 mm (**Inference**), and assumes the 1997 kit matched.
- **Motor.** Of B2-9, B4-6, B6-4 and B6-6, the B6-4's charge came closest to apogee on a test rocket [1] p. 5. The B6
  is NAR certified at 4.33 N·s, 0.86 s burn, 12.14 N peak, 5.6 g propellant [5].
- **Mass.** Clay in the nose cones, then extra tracking powder, brought all ten to 58 g with the motor in [1] p. 7.
- **Smooth.** Painted pink, then paste-waxed [1] pp. 5, 12. Whether fins were sealed first is not stated.
- **Rough.** Strips of 3/4 in double-sided poster tape along the body tube and on the nose cone, sand poured on [1]
  pp. 5, 12. Fins were not taped (the follow-up plans propose taping them, p. 12), and the rough set was not painted
  (p. 4): **its fins were likely bare balsa**, a second difference from the smooth set. Earlier coatings (sealer
  dipped in sand, spray adhesive) clumped and were too heavy [1] p. 5.
- **The sand.** Listed as fine-grain white sandbox sand [1] p. 12; **the report gives no grain size**, and the
  close-up photo has no readable scale (p. 7). Wentworth's fine sand is 0.125 to 0.25 mm [6]. One playground sand
  sieved by ASTM C136 has D10 0.185 mm, D30 0.29 mm and D60 0.45 mm, with 4.1% passing 0.15 mm and 78.4% passing 0.6 mm
  [7] pp. 1–2. This note takes 0.1 to 0.5 mm as the grain range (**Inference**).
- **Flights.** 9 March 1997 at George Washington University, 50 °F (10 °C), "some wind", smooth set first [1] pp.
  7–8. Two stations on a 230.77 m baseline read azimuth and elevation (printed in 0.5° steps); the vertical midpoint
  method gave the heights [1] pp. 8–9.

## 3. Bock 1999: the altitudes

The report prints each rocket's angles, altitude and closure [1] p. 9, and altitudes to 0.1 m [1] p. 11. This note
recomputed every altitude from the angles: all ten match p. 11 within 0.2 m. Closure, as printed, is half the two
stations' difference over their mean.

| Rocket | Smooth (m) | Closure | Rough (m) | Closure |
| --- | --: | --: | --: | --: |
| 1 | 176.6 | −2.75% | 136.3 | 2.22% |
| 2 | 185.8 | −5.06% | 144.6 | 8.77% |
| 3 | 173.8 | −0.45% | 141.5 | −0.15% |
| 4 | 164.9 | −1.75% | 143.4 | −1.85% |
| 5 | 174.8 | −2.34% | 152.6 | 1.40% |
| Mean | 175.18 | −2.47% | 143.68 | 2.08% |

- **The gap is firm.** Standard deviations 7.46 m (4.3%) smooth and 5.91 m (4.1%) rough; gap 31.5 m, −18.0% of the
  smooth mean. Welch's t = 7.40 on 7.6 degrees of freedom, p ≈ 1.0 × 10⁻⁴; 95% interval 21.6 to 41.4 m (−12.3% to
  −23.6%). Every smooth rocket beat every rough one: exact rank-test p = 0.0079, two-sided (**Inference**, computed).
- **Tracking.** A 0.5° elevation reading moves one station's height 1.7% to 1.9% at 35° to 50° (arithmetic). All five
  smooth tracks close negative, three of five rough ones positive. If one station carried a bias that changed between
  sets, the mean closures (−2.47%, +2.08%) bound its effect on the gap at about ±4.5 points: a gap of 13.5% to 22.5%
  (**Inference**; which station erred is unknown). The sets flew in sequence, so a wind change is a confound, unbounded.

## 4. From the altitude gap to a drag change

**Inference**, computed: a vertical point-mass flight, 58 g at liftoff, 5.6 g of propellant spent in proportion to
impulse, the NAR-derived B6 curve in the project's motor catalog (4.33 N·s, 0.86 s) [5], air at 10 °C and 100.1 kPa
(the field's height is not given; about 100 m assumed) with a 6.5 K/km lapse, reference area of a 24.8 mm tube, one
constant C_D, no rod friction or wind. The altitude is taken where the trackers saw it, at the ejection puff.

- **The charge fired before apogee.** Ejection comes at 4.86 s after ignition on the labeled 4 s delay; the smooth
  rockets peak near 6.1 s, the rough near 5.2 s. So the smooth set was still climbing about 12 m/s at the puff, the
  rough set about 4 m/s; their apogees were about 182 m and 144 m, a gap of −21%, larger than the measured −18%.
- **Delay spread.** Estes delays run 0.38 s short on average with 0.45 s scatter, and a 10 °C motor about 3% long
  ([motor-delay-spread](motor-delay-spread.md) §3, §6). The table spans 3.62 to 4.5 s.

| Altitude read at | C_D smooth | C_D rough | ΔC_D | Ratio |
| --- | --: | --: | --: | --: |
| apogee (no delay) | 0.501 | 0.853 | +0.352 | 1.70 |
| ejection, 3.62 s delay | 0.385 | 0.806 | +0.421 | 2.09 |
| ejection, 4.00 s | 0.443 | 0.843 | +0.400 | 1.90 |
| ejection, 4.12 s | 0.457 | 0.848 | +0.392 | 1.86 |
| ejection, 4.50 s | 0.487 | 0.852 | +0.365 | 1.75 |

- **Robust in ΔC_D, not in the ratio.** At apogee read, 55 or 61 g instead of 58 g gives ΔC_D 0.354 or 0.350; impulse
  ±10% gives 0.310 to 0.392 (ratio 2.12 to 1.56). The 95% interval of the gap maps to ΔC_D +0.22 to +0.51.
- **Flow.** Peak speed 63.5 to 66.9 m/s, Mach 0.19 to 0.20; on the 0.312 m body Re = 1.4 × 10⁶ at peak, 0.9 × 10⁶ at
  40 m/s (ν = 1.43 × 10⁻⁵ m²/s at 10 °C). Near C_D 0.45, 0.01 of C_D moves apogee about 1.3 m (0.7%).

## 5. Against the sand-grain friction model

HPR Sim and OpenRocket share Niskanen's friction model: C_f = 1/(1.50 ln Re − 5.6)² when smooth (eq. 3.78), the rough
constant C_f = 0.032 (k_s/L)^0.2 (eq. 3.80) above Re_crit = 51 (k_s/L)^−1.039 (eq. 3.79), and friction drag C_f
times the body's wetted area, with the factor 1 + 1/(2 f_B), over the reference area (eq. 3.85) [8] pp. 43–46.

**Inference**, computed at 66 m/s: smooth C_f = 0.00407 at Re 1.44 × 10⁶. Only body and nose were coated: 0.28 to
0.312 m of cylinder is 45 to 50 reference areas, 47 to 52 with the factor. k_s+ uses u_τ = V √(C_f/2), the larger C_f.

| k_s | C_f rough | over smooth | ΔC_D | Rough above | k_s+ at 66 m/s |
| --- | --: | --: | --: | --: | --: |
| 20 µm (HPR Sim's default) | 0.00464 | 1.14 | +0.027 to +0.030 | 53 m/s | 4.4 |
| 100 µm | 0.00640 | 1.57 | +0.11 to +0.12 | 10 m/s | 26 |
| 150 µm | 0.00694 | 1.70 | +0.14 to +0.15 | 6.6 m/s | 41 |
| 250 µm | 0.00769 | 1.89 | +0.17 to +0.19 | 3.9 m/s | 71 |
| 500 µm | 0.00883 | 2.17 | +0.22 to +0.25 | 1.9 m/s | 153 |
| 1 mm | 0.01015 | 2.49 | +0.29 to +0.32 | 0.9 m/s | 328 |

- **Regime.** 0.1 to 0.5 mm grains sit at k_s+ 26 to 153 at peak speed, transitional to fully rough, and half that at
  30 m/s. The fully rough law then overstates their friction at coast speeds, so these rows are upper bounds for k_s
  equal to the grain diameter. Nikuradse's dense sand has k_s ≈ d; sparse spheres reach k/d ≈ 4 near 20% cover [9]
  p. 5-4 (see [hobby-finish-roughness](hobby-finish-roughness.md) §1). Grains on tape: a monolayer, cover unknown.
- **Other differences, same sign.** The grain layer widens the tube: frontal area +1.6% to +8.2% for 0.1 to 0.5 mm,
  more with the tape: at most +0.01 to +0.04 of C_D, if the whole C_D scaled with it (**Inference**). Bare balsa
  fins on the rough set: DeMar's unfinished BT-20 model, kraft and bare balsa together, read +13% over primer and
  paint [10] p. 7, so at most about +0.06 here. Tape ends, strip edges and clumps are discrete steps
  ([discrete-steps-drag](discrete-steps-drag.md), [excrescence-drag](excrescence-drag.md)), unbounded here. Laminar
  runs on the waxed set would lower its friction and widen the gap, also unbounded.
- **Consistent in sign, at the coarse end in size.** Friction at k_s = d gives +0.11 to +0.25; with the other terms
  at their largest, +0.18 to +0.35, against a measured +0.35 to +0.42. Agreement needs coarse grains, sparse cover
  (k_s above d), or the unbounded terms. Nothing here suggests the rough-wall law overstates rough-model drag.
- **No k_s from these data.** Read as all friction, ΔC_D 0.35 to 0.40 would mean k_s ≈ 1.3 to 2.9 mm, and the gap's
  95% interval 0.4 to 6.9 mm, several times the grain. The confounds cannot be split from friction with one pair of
  sets, so this is an order of magnitude, not a k_s (**Inference**).

## 6. Against the prediction, and the +8.73% bias

- **The prediction ran high.** Rogers Aerospace's Alt 4.05e gave 212.05 m for a smooth Alpha on a B6-4 [1] pp. 3, 11:
  the smooth mean sits 17.4% below it (the prediction is +21.0% over the measurement). The report gives neither the
  C_D nor the mass entered. In this note's flight, 212 m needs C_D ≈ 0.25 at 58 g, or about 47 g at C_D 0.5.
- **Part of it is the puff.** At the nominal delay the smooth apogee is about 182 m (§4), so the prediction is about
  16% high against apogee, 11% to 20% across the delay range (**Inference**).
- **What it says about HPR Sim's bias: little.** On the private collection's 53 scored flights HPR Sim reads +8.73%
  high ([apogee-bias](apogee-bias.md)). Bock's gap has the same sign but comes from another code, unknown inputs, a
  25 mm model at 65 m/s, and optical tracking, so it is no evidence for the bias's size or cause (**Inference**).
- **Paint and wax against the default.** A waxed finish (k_s of a few µm, k_s+ ≈ 1) is smooth at Bock's Re; HPR Sim's
  unset 20 µm adds up to +0.03 of C_D, only above 53 m/s: under 2% of apogee, low side. As
  [hobby-finish-roughness](hobby-finish-roughness.md) §5 found, on a painted rocket finish can't be the bias's share.

## What this suggests

- **A coarse-roughness check, not a finish-band check.** Bock's pair can pin a regression test that HPR Sim's
  rough-wall friction, at 0.1 to 0.5 mm sand on an Alpha at Re 1.4 × 10⁶, stays below the top of the measured ΔC_D
  band (+0.22 to +0.51 at 95%). Expected effect: none on today's numbers; it guards the rough end of the law.
- **The finish bands stay unchecked.** Bock's sand (0.1 to 0.5 mm, 5 to 25 times HPR Sim's 20 µm default) is far
  rougher than every paint, composite and paper band in [hobby-finish-roughness](hobby-finish-roughness.md) §3 (only
  the antifouling row's top, 120 µm, reaches 0.1 mm); his smooth case is paint and wax. It overlaps only the coarse
  end of the FDM band (11 to 197 µm), and loose grains are not layer ridges
  ([fdm-and-tube-friction](fdm-and-tube-friction.md) §2). Bare paper tube is not tested, and [#782][i782] (split the
  bias by printed parts) gets no data from either report.
- **A clean repeat would give a k_s.** Sieved sand of one size, identical fins, an altimeter in each rocket, and the
  grain cover photographed with a scale would turn the same pair's ΔC_D into a k_s/d for a monolayer.

## Open questions

- Grain size, cover and tape thickness on Bock's rough rockets: not in the report; a builder's sample would bound them.
- Whether the rough set's fins were bare balsa (p. 4 implies it) and the smooth set's sealed: not stated.
- The Alt 4.05e inputs (C_D, mass, launch-site air) behind the 212.05 m prediction.
- The six NAR titles listed in §1 were not opened; any of them may hold a finish comparison.

## References

All read 2026-10-28. NAR R&D index (public; reports need a member login), members' copyrighted work, cited only:
<https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>. Private copies sit under `refs/papers/`.

1. K. Bock, "It's a Drag: A Demonstration of the Effects of Drag on Estes *Alpha* Model Rockets", NAR R&D, NARAM-41,
   August 1999, 12 pp., index `id=1357986`, member login. <https://www.nar.org/docs.ashx?id=1357986>
2. B. Snowden (indexed as B. Sowden), "Comparison of Different Preparation Methods for Body Tubes", NAR R&D, NARAM-32,
   1990, 5 pp., index `id=1357877`, member login. <https://www.nar.org/docs.ashx?id=1357877>
3. NAR, R&D reports in chronological order (index page above), copy saved 2026-10-17.
4. Estes Industries, Alpha kit page, SKU 001225. <https://b2b.estesrockets.com/products/alpha>
5. ThrustCurve, Estes B6 (NAR certified 1995-03-25) and its RASP file, from NAR data, in the
   project's motor catalog; data license unknown, cite only. <https://www.thrustcurve.org/motors/Estes/B6/>
6. USGS Open-File Report 2006-1195, Wentworth grade scale chart, public domain.
   <https://pubs.usgs.gov/of/2006/1195/htmldocs/images/chart.pdf>
7. Greely Sand & Gravel, playground sand sieve analysis, ASTM C136, lab no. 59351, May 2025, pp. 1–2; cite only.
   `04-PM0754_Greely-Sand-Gravel_Lab-Testing_Playground-Sand_Sieve_59351.pdf`,
   in <https://greelysand.com/wp-content/uploads/2025/05/>
8. S. Niskanen, MSc thesis, 2009, eqs. 3.78–3.81, 3.85, pp. 43–46. <https://openrocket.sourceforge.net/thesis.pdf>
9. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, p. 5-4. <https://archive.org/details/FluidDynamicDragHoerner1965>
10. J. S. DeMar, "Model Rocket Drag Analysis using a Computerized Wind Tunnel", NAR R&D, NARAM-37, 1995, p. 7.
    <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>

[i782]: https://github.com/nrdptel/fusionspace-eridanus/issues/782
