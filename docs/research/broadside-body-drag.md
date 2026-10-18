# Broadside and tumbling body drag: a booster's descent and the airframe under a canopy

Measured drag of a rocket body falling broadside or tumbling, and what it adds under a drogue or main, for issue #179
(a separated booster has no airframe drag), M1.14f's airframe-drag gap and the tumble model's use outside its fit. It
builds on [streamer-and-tumble-drag.md](streamer-and-tumble-drag.md) ("Tumble": OpenRocket's 0.56 and 1.42, their −10
to +19% replay, TN D-540, TR R-474's 1.2), repeating none of it. **Trust:** fixed-attitude coefficients are measured;
every *tumbling* number is an average or an inference, since no free source measures one at hobby Reynolds numbers.
Read 2026-10-18; printed page numbers; **Inference** marks the note's own arithmetic.

## In short

- **The tumble fit is short; hobby boosters are long.** OpenRocket's drop models had length-to-diameter ratios (l/d) of
  2.3 to 2.8; boosters run about 5 to 20, where measured broadside drag per side area is 15 to 30% higher. The 0.56
  body constant is extrapolated in l/d as well as in Reynolds number (§1).
- **Tumbling, fixed and broadside differ in sign, not only size.** Quasi-static averaging puts a tumble at 0.42 to 0.59
  of broadside drag; rotation itself can add drag (a measured autorotating plate reached a mean of 1.6); falling
  cylinders often don't tumble but fall flat (§2). Which one a booster does moves its descent about 4× (§7).
- **Under a main the airframe barely matters; under a drogue it can dominate:** on round numbers 0.3 to 6% of the drag
  area under a main (0.2 to 3% in speed), 3 to 68% under a drogue (1 to 44%), set mostly by attitude. One flight's
  drogue phase logged 95 ft/s against a 159.8 ft/s forecast (§6). OpenRocket, RocketPy and RockSim leave the airframe
  out, so for a given canopy C_D they err one way: **too fast**, drogue time and drift too short.

## 1. Length: the fit set is short

OpenRocket's documentation, §3.5, Table 3.3 (p. 54) [12]: bodies of l/d 2.3 to 2.8 plus fins alone, 22 m drops. TN
D-540 Fig. 5 (p. 13; Wieselsberger's subcritical data at Re 88,000) [11] gives broadside C_D on side area of about 0.71
at l/d 2.5 (between 0.63 at 1 and 0.74 at 3), 0.82 at 10 and 0.92 at 20. **Inference:** with a fixed tumble factor,
0.56 fitted at l/d ≈ 2.5 is 13% low at l/d 10 and 23% low at 20, so a tumbling long tube is predicted 7 to 12% too
fast. Falling broadside instead, the body term is 0.82 to 0.92 (1.5 to 1.6 × 0.56) and a finless long tube falls 17 to
22% slower than predicted. The sign of the error depends on how the body moves, which no free source predicts.

## 2. Tumbling versus fixed

**Range-safety practice.** General Dynamics' Atlas/Centaur data book (GDC-BTD66-034, 1966) [6] takes spent hardware as
unstable (p. 2-1). With normal force C_D90 sin²α and axial force C_D0 cos²α, its "tumbling" drag is the mean over
constant-rate rotation, (2/3π)(C_D0 + C_D90 + C_D180 + C_D270) (App. A, p. A-2). Its curves are estimates from Hoerner,
Welsh's TN 2941 and theory (p. 2-1), not measurements; Fig. 2-3 (p. 2-7, Atlas sustainer, by eye) puts tumbling at
about 0.52 of broadside subsonic and 0.47 at Mach 1. NASA's debris model [7] is quasi-static too: (A_max + A_min)/2 as
the low end of a band up to A_max (p. 6), with no dynamic correction (p. 8).

**Inference (averaging factors).** Uniform rotation in one plane averages sin³α to 4/(3π) = 0.424; an attitude spread
uniformly over the sphere averages it to 3π/16 = 0.589 (and |cos α|³ to 1/4). A quasi-static tumble is so 0.42 to 0.59
of the broadside coefficient on side area, plus an end term of at most 0.07 at l/d ≥ 5. With §1's values and an end C_D
of 1.0 that is 0.38 to 0.50 at l/d 10 and 0.41 to 0.55 at l/d 20, all below OpenRocket's 0.56.

**Rotation can change drag.** Smith (J. Fluid Mech. 50(3), 1971; abstract only) [9]: a plate autorotating about its
span at Re ≈ 2.4e5 had mean drag ≈ 1.6 and mean lift ≈ 2.2; free-falling plates above Re ≈ 4,000 matched. The abstract
gives no reference area, aspect ratio or static value, so the gain's sign is open: 1.6 is above §3's finite fins (1.19
to 1.33) but below a 2-D static plate (about 2.0). The lift makes an autorotating part glide. Watts and McCarter
(NASA/CR-2012-217475) [8], eq. B.1 (p. 13), write a pitching cylinder's local dynamic pressure as ½ρ(V² − 2VqX + q²X²);
integrated along the tube at the broadside instant (**inference**) it multiplies drag by 1 + k²/3 with k = qL/(2V): +8%
at k = 0.5, +33% at k = 1. The source drops q²X² as about 1% for its missiles; at tumble rates it is the whole gain,
and no source gives them.

**Falling cylinders often don't tumble.** Holland et al. (Exp. Fluids 37, 2004; abstract only) [10]: cylinders falling
in water at Re ≈ 1e6 with mass and volume centers together mostly fell broadside; an offset center of mass gave more
upright, complex motion. **Inference:** OpenRocket's finless tube (#4) wanted 0.79 on side area, 0.71 to 0.89 within
its ±0.3 m/s timing: above every quasi-static mean and at or above its fixed broadside value (about 0.70). A flat fall
or a rotation gain fits it; a quasi-static tumble does not.

## 3. Fins broadside

Koenig (NASA Memo 2-27-59A, 1959) [5]: semispan wings, much as a fin sits on a body, at 0 to 180°, Re 1.5e6 to 2.2e6
(p. 1), with a blockage correction the memo calls approximate (p. 4). Fig. 3 (pp. 10 to 13, by eye) gives C_D at 90° on
wing area of 1.19 (delta) to about 1.33 (aspect ratio 6). OpenRocket's 1.42 exceeds every static value (only a rotation
gain might reach it), so it still has no free source; nor does fin-tube interference broadside.

## 4. Reynolds number: the drag crisis and the 36 m/s case

Polhamus (NASA CR-3809, 1984) [4]: the smooth 2-D minimum is about 0.25 in the low supercritical range (p. 6). For
finite cylinders the length factor η rises through the critical range toward 1.0, as end venting into the narrower wake
fades (pp. 12 to 13; Fig. 1.5, p. 99: l/d 10 at η 0.68 at Re 1e5, 1.0 at 4e5). Roughness (p. 9; Fig. 1.3, p. 98): k/D
below 8e-5 changes nothing below the supercritical range; k/D 2.2e-4 bottoms near 0.35, 1.1e-3 near 0.45.
Compressibility raises the critical Re measurably from Mach 0.15 to 0.2 (pp. 10 to 11). TR R-474 Fig. 2 (p. 76) [3]
holds the 2-D value at 1.2 to Re ≈ 2e5, with the drop near 3.5e5 to 4e5.

**Inference (hobby range, ν ≈ 1.46e-5 m²/s at sea level).** A 41 mm body at 10 m/s is at Re 2.8e4, 100 mm at 36 m/s
2.5e5, 160 mm at 40 m/s 4.4e5; only the large, fast corner enters the crisis. There a finite tube sits near η × 0.25 to
0.5 ≈ 0.2 to 0.5 on side area, 0.2 to 0.65 of its subcritical value; painted hobby tubes (k/D near 1e-4 to 1e-3, an
estimate, not a measurement) soften it toward 0.35 to 0.6. The recovery page's 80 mm body at 36 m/s is at Re ≈ 2.0e5:
at the start of the critical range on TR R-474's 2-D curve, not past it.

## 5. Mach 0.5 to 1.3: the first seconds after separation

Welsh (NACA TN 2941, 1953) [2] flew cylinders of l/d 15, 30 and 60 on rockets, Re 1.25e5 to 7.25e5, drag on side area
within ±0.02 at Mach 0.8 (pp. 1 to 3); Gowen and Perkins' TN 2960 Fig. 9 (p. 25) [1] replots it, by eye:

| l/d | Mach 0.5 | Mach 0.7 | Mach 0.85 | peak (at Mach) | Mach 1.3 |
| --- | --- | --- | --- | --- | --- |
| 15 | 0.91 | 0.93 | 1.10 | 1.86 (1.0) | 1.44 |
| 30 | 0.99 | 1.30 | 1.23 | 1.93 (0.97) | ends at 1.68 (Mach 1.13) |
| 60 | 1.24 | 1.52 | 1.54 | 2.13 (0.95) | 1.55 |

TR R-474 Fig. 1 (p. 75) [3] puts the 2-D crossflow value near 2.0 at crossflow Mach 1, and the drag crisis acts only
below crossflow Mach ≈ 0.5 (p. 15). Descent (Mach 0.03 to 0.12) never sees these values.

## 6. The airframe under a canopy

**Drag areas add.** Knacke (NWC TP 6575) [13], §4.5.1 (p. 4-14), writes steady descent as canopy drag plus load drag
equal to weight, then drops the load term "in most cases". **Attitude** follows the rigging: a transfer bridle moves a
recovered vehicle to a horizontal attitude (p. 6-13). **Inference:** a body hung from one end trails end-on; one hung
near its CG lies broadside. A student team's wiki [14] classes two online videos of other rockets, one under a drogue,
as falling mostly horizontal with a tilt; no measured attitude record was found. **Wake loss is small at HPR harness
lengths.** Knacke Fig. 5-21 (p. 5-22, by eye): with canopy-to-body diameter 3, a canopy keeps about 0.87 of its C_D one
body diameter behind, 0.90 at 4 and 0.97 at 20. **Inference:** a drogue 5 to 10 m behind a 98 mm body sits 50 to 100
diameters back, past the figure's edge: a few percent at most.

**What the tools do.** OpenRocket's documentation v13.05 [12], §4.2.5 (p. 69): after deployment "the entire drag
coefficient of the rocket" comes from the devices (hpr matches it, ADR-153). RocketPy (MIT) [15], `u_dot_parachute`:
canopy drag, added mass and gravity, no airframe drag. RockSim, by its maker (Peak of Flight 361, pp. 5 to 8) [16]: the
rocket's area is "ignored"; his workaround is a phantom chute of area L·d at C_D 0.75 (his 4 in × 36 in, 4,773.9 g
example checks: 33.1 m/s against his 74.3 mph). RASAero II's default canopy C_D of 1.33 was fitted to HPR descents
(manual p. 82) [17]; **inference:** it already holds the airframe's share.

**Measured systems.** FSU Panama City's 2026 Student Launch PLAR [18] (Tables 3, 4, 6, pp. 7 to 9): OpenRocket forecast
an average drogue descent of 159.8 ft/s; the flight logged 95 ft/s. **Inference:** (159.8/95)² = 2.83, so 65% of the
drag was missing from the forecast; its budget names a 15 in drogue and lists two 12 in chutes without a role, so the
canopy's share is uncertain. Penn State's 2018 FRR [19] fitted the body's drag to flight data as a 10.5 in chute at C_D
0.5 (0.028 m²; **inference**, taking its 12 in drogue at C_D 1.5 and its 84 in main at the FRR's 2.2: 20% of the drag
area under the drogue, 0.35% under the main; a mostly end-on body).

**Worked example** (invented round numbers; ρ 1.10 kg/m³; a 98 mm × 2.0 m airframe, 8 kg; canopies C_D 1.5 on the
nominal circle). The body coefficients are placeholders: end-on 0.8 on frontal area for two sections, broadside 0.6 and
1.2 on L·d (§1, §4). Each cell: the airframe's share of drag area, then the speed change, (1 − share)^½:

| device (C_D·S) | alone | end-on, 0.012 m² | broadside 0.6, 0.118 m² | broadside 1.2, 0.235 m² |
| --- | --- | --- | --- | --- |
| 12 in drogue (0.109 m²) | 36.1 m/s | 10%, −5% | 52%, −31% | 68%, −44% |
| 18 in drogue (0.246 m²) | 24.1 m/s | 5%, −2% | 32%, −18% | 49%, −29% |
| 24 in drogue (0.438 m²) | 18.1 m/s | 3%, −1% | 21%, −11% | 35%, −19% |
| 72 in main (3.94 m²) | 6.0 m/s | 0.3%, −0.2% | 3%, −1.5% | 6%, −3% |

## 7. A spent booster's descent

**Stable stages trim nose-first unless forced.** Ares I-X's first stage fired four tumble motors 3 s after staging to
keep it from "settling into a nose-first trim attitude" (Tartabini and Starr [20], pp. 2, 5); even so, its pre-flight
Monte Carlo had 4.1% nose-first, 58.6% broadside and 37.3% tail-first at peak dynamic pressure (Table 2, p. 6). NASA TM
X-1414 [21] tumbled a spent Nike booster on purpose, with six small motors (pp. 3, 4). Apogee's 2003 guide (Peak of
Flight 96, pp. 3 to 5) [16] says burnout moves a booster's CG forward, toward stability; its example booster with no
nose was stable at burnout and would come down "ballistically". **Inference:** a finned HPR booster is often stable
after burnout and coasts nose-first until it slows near its own apogee or is upset, as #179 expects.

**Round numbers** (a 0.1 m × 1.0 m booster, 2 kg; placeholders as in §6; **inference**): nose-first with an open front
at C_D ≈ 1.0 on frontal area, 67 m/s; uniform tumble by §2's 4/(3π) average, 35 to 26 m/s; flat at 0.6 to 1.2 on L·d,
24 to 17 m/s: 4× in speed, 15× in drag area.

**Drogueless descents, measured.** Karbon (Peak of Flight 460, pp. 5 to 8) [16]: a 4 in × 79 in, 3-fin rocket of 3.553
kg whose main never opened fell at 18 to 16 m/s, C_D·A 0.223 m² mean (**inference:** 1.03 to 1.13 on L·d for two
tethered sections with fins and harness); a 3 in × 72 in rocket fell at 20 to 18 m/s, about 1.2 on L·d (Fig. 15, by
eye); altimeter density can be about 10% off (p. 8). Builders report 10 to 15.5 m/s for light rockets [22].

## What this suggests

1. **M1.14f:** add the airframe's C_D·A to the devices' (Knacke's balance), attitude an input (end-on by default for an
   end-mounted harness), with a range; never on top of a coefficient fitted to whole-rocket descents.
2. **#179, booster:** coast nose-first on the headless body's axial drag; then bound the tumble between the uniform
   average 4/(3π)(C_D0·A_front + C_D90·L·d) and a flat fall C_D90·L·d, C_D90 from §1 (subcritical) and §5 (Mach 0.5 to
   1.3), and report the band rather than one number.
3. **Tumble constants:** scale the body term by length (§1) or state the l/d limit beside the Reynolds one; keep 1.42
   documented as unsourced (static fins give 1.19 to 1.33).
4. **Validation:** Karbon's, FSU's and Penn State's are the only measured system numbers; drogue phases in the private
   flight collection could add aggregate C_D·A against L·d.

## Open questions

- No free measurement of a tumbling cylinder or finned stage against a fixed one at Re 1e4 to 1e6; the rotation gain's
  size and sign stay unbounded (strip theory +0 to +33%; Smith's plate has no static pair).
- Whether a finned hobby booster tumbles, falls flat or tail-first after it slows; a booster IMU log would settle it.
- Not read: NAR R&D's "Effect of Rocket Orientation on Descent Drag" (2015, members only); DTIC (down); Smith, Holland.
- Whether OpenRocket 24.12 still counts devices alone (docs silent; an oracle run would check); tube roughness.

## Sources

1. Gowen, Perkins, NACA TN 2960 (1953). <https://ntrs.nasa.gov/api/citations/19930084018/downloads/19930084018.pdf>
2. Welsh, NACA TN 2941 (1953). <https://ntrs.nasa.gov/api/citations/19930083676/downloads/19930083676.pdf>
3. Jorgensen, NASA TR R-474 (1977). <https://ntrs.nasa.gov/api/citations/19770026166/downloads/19770026166.pdf>
4. Polhamus, NASA CR-3809 (1984). <https://ntrs.nasa.gov/api/citations/19840023027/downloads/19840023027.pdf>
5. Koenig, NASA Memo 2-27-59A (1959). <https://ntrs.nasa.gov/api/citations/19980228368/downloads/19980228368.pdf>
6. General Dynamics, GDC-BTD66-034. <https://ntrs.nasa.gov/api/citations/19670005228/downloads/19670005228.pdf>
7. Murman, NASA/TM-2010-216406. <https://www.nas.nasa.gov/assets/nas/pdf/staff/Murman_S_nasa_tm_2010_216406.pdf>
8. Watts, McCarter, NASA/CR-2012-217475. <https://ntrs.nasa.gov/api/citations/20130003336/downloads/20130003336.pdf>
9. Smith, J. Fluid Mech. 50(3), 513 to 534 (1971), abstract. <https://doi.org/10.1017/S0022112071002738>
10. Holland et al., Exp. Fluids 37, 690 to 700 (2004), abstract. <https://aquila.usm.edu/fac_pubs/2983>
11. McKinney, NASA TN D-540 (1960). <https://ntrs.nasa.gov/api/citations/20040047039/downloads/20040047039.pdf>
12. Niskanen, OpenRocket Technical Documentation v13.05 (CC BY-SA), §3.5 and §4.2.5; a release asset of
    <https://github.com/openrocket/openrocket> (tag OpenRocket_technical_documentation-v13.05).
13. Knacke, Parachute Recovery Systems Design Manual, NWC TP 6575. <https://apps.dtic.mil/sti/pdfs/ADA247666.pdf>
14. MIT Rocket Team, Hermes (2018). <https://wikis.mit.edu/confluence/display/RocketTeam/Hermes+Descent+Dynamics>
15. RocketPy (MIT), `rocketpy/simulation/flight.py` at 9bd6ad3a. <https://github.com/RocketPy-Team/RocketPy>
16. Apogee Components, Peak of Flight newsletters 96 (2003), 361 (2014) and 460 (2018), each at
    <https://www.apogeerockets.com/education/downloads/Newsletter361.pdf> with its own number.
17. Rogers, Cooper, RASAero II Users Manual (2019). <https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>
18. FSU Panama City, 2026 Student Launch PLAR. <https://web1.eng.famu.fsu.edu/me/senior_design/2026/team533pc/>
19. Penn State, 2018 FRR. <https://sites.psu.edu/psurocket/files/2017/08/Penn-State-2018-FRR-Presentation-1ugzagv.pdf>
20. Tartabini, Starr, Ares I-X separation. <https://ntrs.nasa.gov/api/citations/20110014618/downloads/20110014618.pdf>
21. Preisser et al., NASA TM X-1414 (1967). <https://ntrs.nasa.gov/api/citations/19670022936/downloads/19670022936.pdf>
22. Rocketry Forum, drogueless descent rates (2016 to 2017).
    <https://www.rocketryforum.com/threads/please-post-your-drogueless-descent-rates-here.134452/>

Terms: the NASA, NTRS and NWC reports are public and cached under `refs/`; the other sources are cite only.
