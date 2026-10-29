# Booster attitude after burnout: nose-first, tail-low, flat or tumbling

What attitude a spent finned booster takes on its way down, and what decides it, for #179 (which descent band a
separated booster flies) and #666 (evidence G13, a dropped booster's logged flight). Attitude moves its descent about
4×. It builds on [broadside-body-drag.md](broadside-body-drag.md) (§2, §7),
[tumbling-vs-fixed-drag.md](tumbling-vs-fixed-drag.md) and [streamer-and-tumble-drag.md](streamer-and-tumble-drag.md).
**Trust:** no public hobby booster log of attitude was found; hobby evidence is what flyers saw. **Inference** numbers
come from the note's own planar model of invented shapes, checked in sign, not size, on two full-scale stages.
Read 2026-10-29; printed pages.

## In short

- **The test is the CG against the broadside centroid, not the Barrowman margin** (**Inference**, extending
  Jorgensen's body method, TN D-6996 pp. 4, 7, to fins). With the CG ahead of the drag-weighted side-area centroid of
  body plus fins, every attitude pitches back to nose-first; behind it, a second stable trim appears from 90° upward
  as the CG moves aft (151° at 0.80 L on the three-fin example), and the booster can fall tail-low (§2).
- **On the example fins, long boosters need a large margin to stay clear of that trim** (**Inference**): 1.0 to 1.7
  calibers at length-to-diameter (l/d) 10, 3.6 to 4.8 at l/d 20; other fin sizes and a fast fall move these by
  calibers, and the margin's convention for an open front moves them 2 to 4 more (§2).
- **Full-scale stages agree on the sign.** The Shuttle SRB would trim near 90° with its CG at 53% of length; its real
  burnout CG, 5 to 6% further aft, trims "somewhat tail first". Ares I-X's first stage flew tail-first at 151.1° at
  peak dynamic pressure, then fell in a flat spin about 90° (§1).
- **The closer to vertical the separation, the further the booster swings over its own apogee** (**Inference**): on
  the example, peaks of 11 to 21° from an 80° separation, 60 to 104° from 85°, 154 to 158° from 89°. That swing can
  carry it into the tail-low trim's basin; trimmed, the l/d 10 example fell at about 17 m/s against 67 m/s (§3).
- **Hobby reports rank attitudes, log none:** long boosters seen "ballistic", unstable ones tumbling, CG unknown (§1).

## 1. What was seen

**Ares I-X first stage, 2009** (Tartabini and Starr [1]), reconstructed from flight data. Tumble motors fired 3 s
after staging (p. 2) and drove about 12 to 16.5 deg/s (Fig. 13, p. 11, by eye). At peak dynamic pressure, 1,064.8 psf
at Mach 2.51, the stage was tail-first at 151.1° total angle of attack (Table 3, p. 11). A post-flight Monte Carlo
with 2,000 lb of aft slag gives 1.0% nose-first, 46.5% broadside and 53.5% tail-first, axial CG and center of
pressure the main sensitivities (p. 11). Then "the FS entered into a flat spin" about 90° (p. 12), by eye 60° to
130° until the parachute near 180 s (Fig. 14). Finless and forced to tumble, it checks the CG sign only.

**Shuttle SRBs** (Bacchus, Kross and Moog [2]; Lockheed [3]). Spent SRBs reenter "in a random tumbling mode" ([2]
p. 189) "until they are captured in a trim condition" ([3] §5, PDF p. 28). The STS-1 and STS-2 flight band in [3]
Fig. 4-5 (p. 4-7, by eye) spans Mach 0.46 to 1.08: about 140° to 149° near Mach 1.05, 132° to 143° at Mach 1.0, a
minimum of 82° to 90° near Mach 0.7 (the "large pitch down below Mach 1.0", p. 4-2) and 125° to 131° near Mach 0.5.
Predicted trims match flight only from Mach 1.1 (p. 4-7). With the CG "ideally located at approximately 53 percent"
of length, "near centroid of area", it would trim near 90°; the real burnout CG, 5 to 6% further aft, trims "somewhat
tail first", and a 59% limit guards against a tail-first "lock-up" ([2] p. 190). A CG almost 3 ft further aft trims
about 10° more tail-first (p. 194). Finless, Reynolds number near 2e7 (p. 191): the sign transfers, the angles don't.

**Wallops boosters** (McFall, 1964 [4]). "Booster motors were seen descending in a flat spin with a propeller like
rotation after shedding their fins" (p. 2). No count, size or speed. A cylinder has a stable trim at 90° (p. 1).
No public hobby booster logged its attitude; every hobby report below is what a flyer saw.

| Report | Booster | Attitude seen | Numbers |
| --- | --- | --- | --- |
| Forum, 2016 [F1] p. 4 | 54 mm sounding-rocket scale | climbing straight after burnout; descent not seen | none |
| Forum, 2012 [F2] | 24 in gap-staged model booster, C6-0 | "came back ballistic" | none |
| Forum, 2021 [F3] p. 2 | 24 mm booster of a spinning two-stage | "tumbled nicely" | none |
| Apogee PoF 97 [5] PDF pp. 4, 6 | model booster, margin −0.10 caliber | timed drop | 5.108 m/s mean |
| Forum, 2016 [F4] | fin can tethered to the nose | "falls in horizontal orientation and spins" | 10.1 m/s |

The last row's 10.1 m/s is for a 1.16 kg liftoff mass. **Inference:** long boosters were seen ballistic, CG
unreported; boosters built unstable tumble, as Estes designs model boosters to (TR-2 [6] p. 6: the fin-area center
at or up to 1/4 in ahead of the balance point with the spent casing).

## 2. The static test: CG against the broadside centroid

**The body.** Jorgensen's method [7] splits a body's normal force into a potential term, placed at the nose below 90°
and at the base above, and a crossflow term η·C_dn·(A_p/A)·sin²α at the side-area centroid (eqs. 1, 4, 5, pp. 3 to
4). C_dn is about 1.2 for subcritical crossflow up to a crossflow Reynolds number of about 2e5, and 0.15 to 0.30
supercritical (p. 5). It was checked from 0 to 180° only at Mach 2.86 (p. 1); TR R-474 [8] covers Mach 0.6 to 2.0 to
60° only (0 to 180° only at Mach 2.9). Nothing validates it for finned bodies at subsonic Mach past 60°, nor for an
open front.

**The fins.** Koenig's semispan wings at Reynolds number 1.5 to 2.2e6 [9]: C_D about 1.22 (aspect ratio 2, swept)
and 1.19 (delta) at 90° (Fig. 3, p. 10, by eye). **Inference** from C_m (Fig. 5, p. 18, by eye, taking the reference
as the quarter chord, which the text doesn't state): the broadside center sits at 0.51 to 0.54 of the mean chord.
Hoerner and Borst [10] (pp. 4-22, 4-23): past stall the force is about normal to the chord, 2-D C_N 1.8 at 90°.

**The model (Inference).** Body: Jorgensen's terms for a cylinder, with the open front counted as a nose (slope 2 at
x = 0) and crossflow η·1.2 at L/2, η 0.62 to 0.77 by l/d (TN D-540 via
[broadside-body-drag.md](broadside-body-drag.md) §1). Fins: Barrowman slope times sin α·|cos α|, plus
1.2·(2N/π)·S_fin·sin²α, centered at 0.25, 0.50 and 0.75 of the mean chord at 0°, 90° and 180°. On a 0.1 m × 1.0 m
tube, fins of root 0.15 m, tip 0.075 m, span 0.10 m and sweep 0.075 m (margin with the front counted as a nose):

| Fins | CG (fraction of L) | Barrowman margin (cal) | Other trims (u unstable, s stable) |
| --- | ---: | ---: | --- |
| 4 | 0.55, 0.60 | 1.96, 1.46 | none: nose-first from any attitude |
| 4 | 0.65 | 0.96 | u 79°, s 95° |
| 4 | 0.70 | 0.46 | u 29.5°, s 110.5° |
| 4 | 0.75, 0.80 | −0.04, −0.54 | s 127.5°; s 144° |
| 3 | 0.75, 0.80 | −0.47, −0.97 | s 136.5°; s 151° |

Over l/d 5 to 20, fin scale 0.5 to 2, three or four fins, η × 0.6 or 1, fin C_N at 90° of 1.2 or 1.8 and CG 0.30 to
0.89 L, a second stable trim existed exactly when the CG sat aft of the broadside centroid in 5,019 of 5,028
Barrowman-stable cases; all nine exceptions had a margin of 0.01 caliber or less. The margin at which the CG meets the
centroid, on the example fins:

| l/d | 3 fins (cal) | 4 fins (cal) | 4 fins, fin crossflow halved (cal) | 4 fins, body C_dn 0.4 (cal) |
| ---: | ---: | ---: | ---: | ---: |
| 5 | −0.10 | −0.04 | not run | not run |
| 10 | 0.98 | 1.16 | 1.69 | 0.00 |
| 15 | 2.24 | 2.60 | not run | not run |
| 20 | 3.59 | 4.16 | 4.84 | 2.26 |

Half-size fins put the threshold at −0.70 to 0.48 cal, double-size at 0.39 to 0.50 (l/d 10) and 3.43 to 3.71
(l/d 20). A tool giving a headless tube no front lift puts the CP 1.7 to 2.1 cal further aft at l/d 10 (3.5 to 4.4 at
l/d 20) and reads every threshold that much higher. The trim moves about 3.2° per 1% of length of CG shift (three
fins), the same direction as the SRB's (p. 194). A fast fall (C_dn 0.4, above Jorgensen's supercritical band) moves
the centroid aft, so a booster can gain a tail-low trim as it slows. Dahlquist's tunnel data [11] agree in sign only:
the center of pressure moves forward with angle of attack, the lateral-area center is "a rough, conservative
estimate" of it at 90° (PDF p. 1), and the worst case is 13° of safe angle per caliber over four rockets (PDF p. 3).
**Inference:** that angle is the nose-first basin's edge; the model's is 2.5 to 6× wider (64 to 82° per caliber).

## 3. Near the booster's apogee

**A stable body lags a turning path** (MIL-HDBK-762 [12]): the "yaw of repose" (p. 4-14). Its yaw wavelength,
σ = 2π√(2I/(ρ·A·l_s·C_Nα)), is the distance flown per pitch swing (eq. 4-7, p. 4-10). **Inference:** the path turns at
g·cos γ/V and the body's pitch frequency grows as V, so the lag ratio Λ = σ·g·cos γ/(2πV²) grows as 1/V². For three
invented boosters σ is 26 to 66 m; Λ is under 0.1 rad above 20 to 32 m/s and over 0.5 rad below 9 to 14 m/s.

**Apogee airspeed (Inference;** point mass, 2 kg, 0.1 m, blunt front C_D 1.0): separated at 100 m/s and 75°, 80°, 85°
and 88° above the horizon, it reaches apogee at 14.0, 9.5, 4.8 and 1.9 m/s, 6.5 to 6.7 s later: too slow to turn.

**Planar pitch runs (Inference;** §2's statics; fin damping C_mq = −2·C_Nα,fins·(l_f/d)² at every angle; body damping
from [13] eq. B.6 times |sin α|; I = mL²/12, ρ = 1.1 kg/m³; axial C_A 1.0·cos²α nose-first and 0.8 tail-first;
separated at 150 m/s, no wind). Peak angle in the 10 s after apogee, CG 0.58 to 0.66 L: 11 to 21° from an 80°
separation, 60 to 104° from 85°, 154 to 158° from 89°. Outcome on the l/d 10 example with four fins:

- CG at 0.58 to 0.64 L (margin 1.66 to 1.06 cal): nose-first by 25 to 30 s from every separation angle, 70° to 89°.
- CG at 0.68 L (0.66 cal): trimmed at 104° from 83° to 89°, else nose-first; 0.72 L (0.26 cal): 117° from all.
- CG at 0.66 L (0.86 cal): trimmed at 98° only from 85°, nose-first at 87° and 89°: near the threshold it turns on
  the swing.

It falls at about 67 m/s nose-first, 17 m/s trimmed. At l/d 20 (2 m, 4 kg), 4.4 and 5.2 cal ended nose-first, 2.0
cal trimmed at 132° from 80° up, 1.2 cal at 145° from every angle: well past the threshold any separation reaches
the trim, near it only a near-vertical one.

## 4. Flat spin and tumble

**A tumble without a driver dies out (Inference).** Crossflow damping at broadside grows with the cube of length:
M = −(1/3)ρ·V·q_m·C·D·L³·(1 − 3c + 3c²), q_m the pitch rate, C the crossflow drag coefficient and c the CG's fraction
of length from the front ([13] eq. B.6, p. 13). Each turn loses energy and the body settles into a trim's basin.

**A flat spin needs a broadside trim and a yaw torque.** SRB studies proposed opposite-side strakes to force one,
needing tight static-margin control and roll stabilization ([2] p. 190). **Inference:** on a hobby booster, fin cant
or a bent fin would be the torque; without it the booster falls flat without spinning. No flat-spinning finned body
was measured: its gain over broadside drag is [tumbling-vs-fixed-drag.md](tumbling-vs-fixed-drag.md)'s unbounded one.

## 5. What else moves it

The sustainer's wake first cut, then raised, Ares I-X's tumble rate from 5 to 15 s ([1] pp. 10, 11). Asymmetric
vortex side forces start near 20° ([8] p. 57) and shrink with a blunt front (Summary). Both are unbounded for hobby
boosters. Burning out moves a booster's CG forward, toward nose-first (broadside-body-drag.md §7); slag moves it aft,
toward tail-low ([1] p. 11).

## What this suggests

1. **#179:** choose the booster's descent band by the static test: CG ahead of the broadside centroid, the nose-first
   band; behind it, the trimmed tail-low band (about 4× slower on the example), certain well past the threshold and
   likely after a near-vertical separation; near the threshold, report both. Compute the centroid from the design,
   not a margin threshold, which moves with fins and convention. The sign is confirmed on two stages, not the size.
2. **6-DOF:** a full-range C_m(α) (Jorgensen body plus Koenig-like fins) would let the simulator find the trim itself.
   Nothing validates finned bodies past 60° at subsonic Mach, so it needs an ADR and a stated validity limit.
3. **#666 (G13):** a logged booster flight with CG, geometry and separation angle would test it, the first public one.

## Open questions

- No public hobby booster log with attitude or time of leaving nose-first; thresholds unchecked in size (§2).
- The model is planar: one geometry family, static coefficients at all rates, no wind, roll or real fin shielding.
- Not read: NAR R&D reports on boosted darts, backslide recovery, drag separation and delayed staging (members only);
  DTIC's high-angle missile data (ADA101210, down); Titan IIIC −10° to 190° tunnel data (OCR unreadable).

## Sources

NASA, NTRS and MIL-HDBK-762 are public; Apogee, Estes, Dahlquist, Hoerner and the forum posts are cited only.

1. Tartabini, Starr, "Ares I-X Separation and Reentry Trajectory Analyses" (NASA, about 2011).
   <https://ntrs.nasa.gov/api/citations/20110014618/downloads/20110014618.pdf>
2. Bacchus, Kross, Moog, "The Aerodynamic Challenges of SRB Recovery", Space Shuttle Technical Conference (1985),
   pp. 189 to 194. <https://ntrs.nasa.gov/api/citations/19850008599/downloads/19850008599.pdf>
3. Lockheed-Huntsville, LMSC-HEC TR D951500-1, "Filament Wound Data Base Development", Rev. 1 (1985).
   <https://ntrs.nasa.gov/api/citations/19870011583/downloads/19870011583.pdf>
4. McFall, "Flat-Spin Recovery System", NASA Langley L-3933 (1964).
   <https://ntrs.nasa.gov/api/citations/19650025606/downloads/19650025606.pdf>
5. Apogee Components, Peak of Flight 97 (2003). <https://www.apogeerockets.com/education/downloads/Newsletter97.pdf>
6. Estes Industries, Classic Collection 03, TR-2 "Multi-Staging", p. 6.
   <https://www.europerocketry.com/uploads/media/Estes_Classic_Collection_03.pdf>
7. Jorgensen, NASA TN D-6996 (1973). <https://ntrs.nasa.gov/api/citations/19730006261/downloads/19730006261.pdf>
8. Jorgensen, NASA TR R-474 (1977). <https://ntrs.nasa.gov/api/citations/19770026166/downloads/19770026166.pdf>
9. Koenig, NASA Memo 2-27-59A (1959). <https://ntrs.nasa.gov/api/citations/19980228368/downloads/19980228368.pdf>
10. Hoerner, Borst, *Fluid-Dynamic Lift* (1985), pp. 4-22, 4-23.
11. Dahlquist, "Wind Caused Instability" (HPR, March 1998, revised).
    <https://www.spaceportrocketry.org/pdfs/Wind%20Caused%20Instability.pdf>
12. MIL-HDBK-762(MI), *Design of Aerodynamically Stabilized Free Rockets* (1990), pp. 4-10, 4-14.
13. Watts, McCarter, NASA/CR-2012-217475, App. B, p. 13.
    <https://ntrs.nasa.gov/api/citations/20130003336/downloads/20130003336.pdf>

Forum reports (public posts):

- F1. <https://www.rocketryforum.com/threads/full-scale-sparrow-hv-arcus-2-stage-sounding-rocket.132380/>
- F2. <https://www.rocketryforum.com/threads/perfect-24-gap-staging-see-last-post.45863/>
- F3. <https://www.rocketryforum.com/threads/youre-gonna-like-the-turbo-a-spinning-2-stage.165638/>
- F4. <https://www.rocketryforum.com/threads/please-post-your-drogueless-descent-rates-here.134452/>
