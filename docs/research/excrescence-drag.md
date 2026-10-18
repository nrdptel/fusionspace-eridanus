# How much drag the small things on a high-power airframe add

[Apogee-bias](apogee-bias.md) §2 sized rail buttons and launch lugs among the causes of HPR Sim's +8.73% apogee bias on
real flights. This note sizes the rest of the small features: joint gaps, tube-to-tube steps, switch bands, vent and
static-port holes, screw, rivet and shear-pin heads, the screw head on a rail button, camera shrouds and antennas. **In
short: on an invented, clean 98 mm L2 dual-deploy rocket they add about 2% of C_D (0.6–6%), on a 54 mm one about 4%
(1.4–11%).** That is about 0.6–2.5% of apogee, 7–29% of the bias, always on the side that makes the codes fly high:
neither HPR Sim nor OpenRocket has a term for any of them. Steps are the largest clean-airframe term: a switch band
standing proud, or one tube a few tenths of a millimeter larger than the next. Gaps and ports are negligible. One
external camera shroud outweighs everything else: 1–5% of C_D if faired, 5–14% if blunt (98 mm), about three times that
on 54 mm; a whip antenna sticking straight out is 4–6%. Faster than sound, the steps' drag on their own area is about 1
to 3 times the low-speed value. How far to trust it: the coefficients are measured, mostly on flat tunnel walls in the
1930s and 1940s, and applied to a rocket through a flat-plate boundary layer; the one rocket-body flight test (Hopko
1952, Mach 1.25–2) agrees in size. No hobby rocket measurement of any of them was found; the sizes are invented.
"Inference" marks this note's own arithmetic. Spiral seams, paint edges and decals are left to the `discrete-steps-drag`
topic.

## 1. How drag inside the boundary layer scales

- **The layer shields small parts (correlation).** A part of height h in a turbulent boundary layer of thickness δ meets
  an average dynamic pressure q_eff/q ≈ 0.75 (h/δ)^(1/3) (eq. 9). Its drag on its frontal area is then C_D = 0.75
  (h/δ)^(1/3) C_D', with C_D' the "independent" coefficient, for h < δ (eq. 10) [E1] p. 5-6. So drag grows as h^(4/3),
  not h. Hood's rivet tests take q at the head's top from a 1/7-power profile, (h/δ)^(2/7) [E2] p. 16.
- **The layer's thickness (correlation).** On a turbulent flat plate δ/x = 0.154 Re_x^(−1/7) [E1] p. 2-3 eq. 9, about
  1.6% of x for Re_x of 10^6 to 10^7 [E1] p. 5-6 eq. 11. Nearly all the data are from flat walls; near the rear of a
  body a protuberance may drag about twice as much [E1] pp. 5-11 to 5-12.
- **δ on the invented rockets (Inference;** ν = 1.5 × 10^−5 m²/s, x from the nose tip**).** Speed barely matters, since
  δ ∝ V^(−1/7). Features 0.1–3 mm tall sit at h/δ of about 0.01 to 0.4.

| x from the tip | 0.27 m | 0.45 m | 0.75 m | 1.0 m | 1.2 m | 1.6 m |
|---|---|---|---|---|---|---|
| δ at 150 m/s | 5.0 mm | 7.8 mm | 12.0 mm | 15.4 mm | 18.0 mm | 23.0 mm |
| δ at 300 m/s | 4.5 mm | 7.0 mm | 10.9 mm | 14.0 mm | 16.3 mm | 20.9 mm |

## 2. Steps, joint gaps and holes (measured)

- **Steps at low speed.** For sheet-metal joints, C_D' on the step height is 0.40 for a lap facing the flow, 0.22 for
  one facing away, 0.70 for a strap proud on both edges and 0.01 for a flush butt joint. Beveling or rounding cuts a
  forward lap to 1/4 or 1/10 [E1] p. 5-7 fig. 10 (Wieghardt's tests). A backward step as a base, C_D = 0.18 (h/δ)^(1/3)
  on free-stream q [E1] p. 3-21 eq. 36, matches the 0.22–0.24.
- **In free-stream terms (Inference).** On h times the circumference: forward step 0.30 (h/δ)^(1/3), backward 0.18
  (h/δ)^(1/3), proud band 0.525 (h/δ)^(1/3). A 0.2 mm step under a 14 mm layer scores 0.073 facing forward and 0.044
  facing back.
- **Steps faster than sound, on a rocket body (free flight).** Hopko flew 56 in (1.42 m) bodies of 5 in (127 mm)
  diameter to Mach 2.1, with four 0.064 in (1.63 mm) laps [E3] pp. 1–2. Read from fig. 7 [E3] p. 12 (±0.005), forward
  laps added about 0.090, 0.065 and 0.051 to C_D at Mach 1.25, 1.55 and 2.0, against a smooth body's 0.36, 0.31 and
  0.264: 19% of C_D at Mach 2. On each lap's own area that is 0.44, 0.32 and 0.25, 2.7, 2.0 and 1.5 times the low-speed
  law (Inference). Hoerner's own reduction of these laps, C_D' 0.75 forward and 0.45 backward [E1] p. 20-14, is more
  than twice this note's. He reads the flights as about 3 times the incompressible values, falling from 2 to 1 between
  Mach 1.2 and 2.1, partly wave drag [E1] p. 17-7.
- **Backward steps in a tunnel.** On a 4.1 in ogive-cylinder, 0.020 in (0.51 mm) rearward steps gave 0.16, 0.115 and
  0.053 on their frontal area at Mach 1.61, 2.01 and 2.87 and 3 × 10^6 per foot [E4] figs. 2, 9 (pp. 10, 15): 2.6, 1.8
  and 0.85 times the low-speed law (Inference). Higher Mach number relieves this drag; higher unit Reynolds number
  raises it [E4] p. 8. A forward step's C_D on the mean q over its height holds near 0.72 for tall steps at Mach 2.2;
  the correlation breaks down near the sonic line, and at HPR steps' height over displacement thickness of 0.06–0.1 the
  points read 0.6–0.75, at most about 15% lower [E5] fig. 6 (p. 12).
- **Coupler gaps.** A coupler joint is a deep transverse gap: width e of 0.2–1 mm along the flow over a wall 1.5–3 mm
  deep. Deep gaps score 0.014 on their plan area e·b with q taken at height e [E1] p. 5-10 fig. 20. A hollow gap with
  its upstream edge bent down 10° and its downstream edge up 5° drags 4.1 times the flat-edged one [E1] p. 5-11 fig. 22,
  and rounding an upstream edge into a hole raises the hole's drag about 4 times [E1] p. 5-10 fig. 19: a sanded or
  chamfered tube end is probably the bad case (Inference). On their own areas, 1.27 mm square grooves drag about 1/37,
  1/26 and 1/13 of the 0.51 mm rearward steps at Mach 1.61, 2.01 and 2.87 [E4] figs. 7–9 (Inference).
- **Vent and static-port holes.** About C_D 0.01 on the opening area: 0.017 for d = 0.5δ, 0.008 for depths of 0.5d and
  d, 0.04 with the upstream edge rounded into the hole [E1] p. 5-10 fig. 19. No source found measures a hole venting a
  large, leaky volume, nor holes faster than sound.

## 3. Heads, fairings and antennas (measured)

- **Screw and rivet heads, subsonic.** On plan area πd²/4 and q_eff: hex bolt head 0.80; cylindrical (pan-like) head
  0.42 below the critical Reynolds number and 0.13 above it; round head 0.32 and 0.13; countersunk 0.02 and 0.01; flush
  rivet 0.002 [E1] p. 5-8 fig. 14. A round head's coefficient falls from about 0.5 to 0.09 as its Reynolds number, on
  its diameter and the speed w in the layer, passes about 2 × 10^4, the drop's midpoint near 8 × 10^4 [E1] p. 5-9 fig.
  16. A 7 mm head on a rocket sees about 4 × 10^4 at 150 m/s and 1 × 10^5 at 300 m/s, so pan and dome heads may drag up
  to 3 times less near peak speed (Inference).
- **Hood's brazier heads.** On a NACA 23012 wing in a turbulent layer, C_D 0.32 on frontal area and q at the head's top;
  1.3 in a 0.1 mm laminar layer [E2] pp. 15–16. Rows of 3/32 in heads raised the wing's drag by up to 27%, much of it by
  moving transition forward [E2] p. 9. An HPR layer is mostly turbulent by the nose shoulder, so tripping adds little
  there, but it is not bounded here.
- **Heads faster than sound.** Hopko's second model carried 124 brazier rivets with heads 3/64 in (1.19 mm) high, in
  four rows of 32, 32, 32 and 28 [E3] p. 2, fig. 1. They added the least of the irregularities tested [E3] p. 1: about
  0.022, 0.021 and 0.015 to C_D at Mach 1.2, 1.5 and 2.0 (fig. 7, read to ±0.003), or 1.4–2.2 mm² of drag area per head,
  1.3–2.9 times Hood's subsonic rule (Inference). That brackets a stagnation-pressure scaling, ×1.22 at Mach 0.9 and
  ×1.66 at Mach 2 [E1] pp. 15-2, 16-3. Hoerner's reduction of the same flights [E1] p. 20-14 gives about 5–7 times more
  per head; it appears to count 32 heads where the report shows 124 (Inference).
- **Fairings on a wall.** A streamlined half-body scores C_D' ≈ 0.07 up to h/δ ≈ 1 [E1] p. 8-4 fig. 9; blisters reach a
  minimum of about 0.03 near length over height 10, rise to about 0.065 at 4 on a plane wall and pass 0.10 at 7 on
  fuselages [E1] p. 8-4 fig. 10. Blunt blocks score 0.74–1.20, a bracket 0.40 [E1] p. 5-8 fig. 13. A small body near a
  larger one's nose can trip its laminar layer and cost it more than the small body's own drag [E1] p. 8-7.
- **Antenna whips.** A 2 mm wire across the flow is a circular cylinder at a Reynolds number near 2.7 × 10^4, C_D
  1.17–1.25 [E1] p. 3-3 fig. 2, on the 63 mm outside the layer and at 0.75 q on the 17 mm inside it (Inference). A wire
  lying along the body drags about twice its own skin friction [E1] p. 5-9.

## 4. A typical L2 dual-deploy airframe (invented, Inference throughout)

Two invented rockets: 98 mm (3.9 in) across, 2.0 m long, reference area 7,543 mm²; and 54 mm (2.1 in), 1.5 m, 2,290 mm².
Each has a nose shoulder joint, two body joints at a switch band's edges, four static ports, 6–8 screw, rivet or
shear-pin heads, and two rail buttons. Typical sizes, with the range tried: joint gaps 0.5 mm (0.2–1.0); tube mismatch
0.2 mm (0.1–0.5) at each joint; band proud by 0.5 mm (0.1–1.0); ports 4 × 4 mm (1 × 2 mm to 4 × 6 mm); heads 1–3 mm
proud. Each share is ΔC_D over a whole-rocket C_D of 0.5 (0.45–0.6) on the body's cross-section, at 300 m/s; 150 m/s is
within 4%.

- **Worked line.** One forward mismatch at the 98 mm band, 300 m/s: 0.30 × (0.2/14.0)^(1/3) = 0.073; times 0.2 mm ×
  307.9 mm of circumference = 61.6 mm² gives 4.5 mm² of drag area; over 7,543 mm², ΔC_D = 0.00060, 0.12% of C_D.
- **A dome head 7 mm across, 2 mm proud,** at a nose joint (δ = 7.5 mm, 200 m/s), by Hood's rule: 0.32 × (2/7.5)^(2/7) ×
  (2/3 × 7 × 2) = 0.32 × 0.685 × 9.33 = 2.05 mm². Hoerner's round-head curve [E1] p. 5-9 fig. 17 gives 1.1–1.9 mm² on
  its low-Reynolds curve, 0.35–0.5 on its high-Reynolds one. A pan head by fig. 14 gives up to 8.4 mm² below the
  critical Reynolds number, 2.0–2.6 above it; a countersunk 0.3.

| feature | 98 mm: % of C_D, typical (range) | 54 mm: % of C_D, typical (range) |
|---|---|---|
| 3 joint gaps | 0.05 (0.01–0.12) | 0.09 (0.03–0.23) |
| 3 tube mismatches, facing forward | 0.39 (0.15–1.3) | 0.78 (0.31–2.7) |
| switch band, proud | 0.71 (0.08–1.8) | 1.4 (0.16–3.5) |
| 4 static ports | 0.01 (0.00–0.05) | 0.04 (0.00–0.17) |
| 6–8 screw, rivet and shear-pin heads | 0.5 (0.2–2.0) | 0.8 (0.35–2.3) |
| 2 rail-button screw heads (§5) | 0.4 (0.15–0.7) | 1.3 (0.5–2.3) |
| **clean airframe, total** | **2.0 (0.6–6.0)** | **4.4 (1.4–11.2)** |
| one camera shroud, 25 × 40 × 80 mm, faired | 1.1–4.7 | 3.5–15.5 |
| the same shroud, blunt front or back | 5.3–14 | 17–47 |
| a 2 × 80 mm whip antenna, straight out | 3.9–5.6 | 13–18 |

- **Mismatches facing back** instead score 0.23% (98 mm) and 0.47% (54 mm). A flush band with tight, matched joints
  leaves only the gaps and ports: under 0.15%.
- **The shroud** has a frontal area of about 800 mm² and C_D 0.06–0.2 if faired at both ends (fig. 9's correlation,
  0.058, and fig. 10's plane wall, about 0.07, at length over height 3.2; 0.2 is this note's cap, since fig. 10's
  fuselage points climb off the chart there), 0.3–0.6 if it has a flat lens window or a cut-off back (fig. 13's blocks
  in the layer). Its transonic rise is not bounded: stores under a swept wing read C_D 0.65 at Mach 1.6 and about 1.0 at
  Mach 1.1, interference included [E1] p. 20-13.
- **Faster than sound,** with the steps at 2–3 times near Mach 1.2–1.6, the clean total rises to about 3–4% (98 mm) and
  6–9% (54 mm). Heads follow stagnation pressure, as does the rest of the rocket's pressure drag, so their share holds
  near the subsonic one.
- **In apogee,** at the −0.31% to −0.57% of apogee per 1% of drag in apogee-bias's sensitivity table: the clean 98 mm
  airframe's 2.0% is −0.6% to −1.1% of apogee, 7–13% of the bias; the 54 mm one's 4.4% is −1.4% to −2.5%, 16–29%. Rail
  buttons themselves (apogee-bias §2, 0.6–4% of apogee) come on top. A shroud or whip adds −0.3% to −8% more on 98 mm.

## 5. What HPR Sim and OpenRocket model

- **No gap, joint, hole, head, shroud or antenna term in either.** OpenRocket's parasitic drag covers launch guides only
  [E6] p. 51; Gregorek's TR-11 has no joint, gap or hole term [E7]. HPR Sim's drag buildup leaves out interference and
  has no such term (`crates/hpr-aero/src/drag.rs`, module docs). Both therefore fly high by the totals above.
- **A step drawn as a larger tube is overcharged.** A step up in radius takes 0.8 sin² φ on its annulus, Niskanen's
  nose-joint rule [E6] pp. 47–48 eq. 3.86, 0.8 for a square step; a step down counts as a boattail of no length and
  takes the full base drag [E6] p. 49 eq. 3.88. Both rules are for features far taller than the layer. A 0.5 mm proud
  band drawn as its own tube costs about ΔC_D 0.021 on the 98 mm rocket, 4.2% of C_D, about 6 times §2's measured
  0.0035; a 0.2 mm step up gets 0.8 against a measured 0.073, 11 times (Inference). Drawing the band overshoots more
  than leaving it out undershoots.
- **The rail button's screw head flies as nothing.** HPR Sim weighs a `.ork` button's screw head but gives it no drag
  (`rail_buttons` in drag.rs; [mass](../physics/mass.md)). An invented 1010-class button (11 mm across, 7 mm waist)
  flies 0.85 × (11 × 4 + 7 × 5) = 67 mm² of drag area in HPR Sim. An 8 mm pan or dome head 2–2.5 mm proud has 13–17 mm²
  of side profile, and at C_D 0.3 (Hood) to 0.8 (Hoerner's bolt and pin, fig. 13) with q_eff 0.88 it adds 3.6–12 mm²,
  +5% to +18% of the button (Inference). Two heads are 0.15–0.7% of a 98 mm rocket's C_D, 0.5–2.3% of a 54 mm one's. So
  the simplification in [known risks](known-risks.md) errs on the low-drag, high-apogee side, by under 0.5% of apogee on
  98 mm and up to about 1.3% on 54 mm. The button model's own sign is open: HPR Sim gives the button no boundary-layer
  shielding, which Hoerner's q_eff would put at 0.61 for 9 mm under 17 mm [E1] p. 5-6, against apogee-bias §2's CFD,
  which puts installed buttons higher.

## What this suggests

- **An optional airframe-joints term:** per joint, C_D' × 0.75 (h/δ)^(1/3) on the step height times the circumference, δ
  from the flat-plate law at the joint's station, with heights the user states. Expected effect: +1–2% of C_D on a clean
  rocket, −0.3% to −1.5% of apogee: small against the bias, but one-signed.
- **For drawn steps shorter than the layer,** the boundary-layer step law instead of the nose-joint rule. Expected
  effect: a drawn proud band's drag falls about 6 times, raising apogee by a few percent on files that draw one.
- **Fly the rail button's screw head** as a dome at C_D 0.3–0.8 on its side profile: +0.3–0.4% of C_D on 98 mm, +1–1.5%
  on 54 mm.
- **A shroud or antenna part,** C_D 0.06–0.6 on frontal area by shape: the only items here that move apogee by percents
  on their own.
- **A hobby measurement:** one airframe tested with and without its band, heads and shroud would replace §4's sizes.

## Open questions

- **Transonic step, gap, hole and head drag (Mach 0.8–1.2)** is not bounded. Gaudet and Johnson (RAE TR 70190, TR
  71181), Gaudet and Winter (RAE TM Aero 1538) [E8], AGARDograph 264 [E9] and Gaudet 1987 [E10] cover it; DTIC returned
  403 and then a maintenance page on 2026-10-18. ESDU 75031 (steps and ridges) and 76008 (rivet rows) are proprietary,
  titles only.
- **How large are real hobby joint mismatches, band steps and heads?** No survey was found; every size in §4 is
  invented.
- **A body of revolution's layer** differs from a plate's, thicker on a boattail [E1] p. 8-5; how that moves a feature
  near the tail is not bounded.
- **How OpenRocket treats two touching tubes of different diameter** isn't stated in the thesis.

## Sources

All read 2026-10-18. NACA and NASA reports (public use per NTRS) are cached in the gitignored `refs/`; the rest are all
rights reserved or unstated: cited, not copied.


- **E1.** Hoerner, S. F., *Fluid-Dynamic Drag*, author-published, 1965; printed pages 2-3, 3-3, 3-21, 5-6 to 5-12, 8-4
  to 8-7, 15-2, 16-3, 17-7, 20-13, 20-14. Copyrighted; cite only.
- **E2.** Hood, M. J., NACA TN 695, 1939. <https://ntrs.nasa.gov/citations/19930081552>
- **E3.** Hopko, R. N., NACA RM L52F09, 1952. <https://ntrs.nasa.gov/citations/19930090438>
- **E4.** Czarnecki, K. R., Sevier, J. R., Carmel, M. M., NACA TN 4299, 1958.
  <https://ntrs.nasa.gov/citations/19930085196>
- **E5.** Czarnecki, K. R., NASA TN D-3589, 1966; also in NASA SP-124. <https://ntrs.nasa.gov/citations/19660028009>
- **E6.** Niskanen, S., OpenRocket MSc thesis, TKK, 2009 (not the code). <https://openrocket.sourceforge.net/thesis.pdf>
- **E7.** Gregorek, G. M., *Aerodynamic Drag of Model Rockets*, Estes TR-11, 1970; local copy in `refs/papers/`.
- **E8.** Gaudet, L., Winter, K. G., RAE TM Aero 1538, 1973; not reached. <https://apps.dtic.mil/sti/pdfs/AD0918999.pdf>
- **E9.** Young, A. D., Paterson, J. H., AGARDograph 264, 1981; not reached.
  <https://apps.dtic.mil/sti/pdfs/ADA106030.pdf>
- **E10.** Gaudet, L., Aeronaut. J. 91(904):170–182, 1987; abstract only. <https://doi.org/10.1017/S000192400002114X>
