# How far hobby parachutes glide, swing and turn in descent

**In short: a solid flat or elliptical hobby main can glide sideways at roughly 1–3 m/s at hobby descent rates, and
no source measures whether its turning averages that out.** Hobby-sized model canopies (45 in, 1949) glided at up to
45° at 10 ft/s and swung or glided at 15–25 ft/s [1, p. 5-16]; read through their drag-coefficient curves, that is
about 24° at 15 ft/s and 12–19° at 20 ft/s, a horizontal 1.3–2.1 m/s (**Inference**, §1). A full-size
hemisphere, simulated, glides at 27° (3.4 m/s at 6.8 m/s down) [3]. Annular (toroidal) and cross canopies oscillate
under ±6° and 0–3° [1, Table 5-1] and should glide far less (**Inference**). No turn rate of a round, elliptical
or toroidal canopy was found; one forum flight's GPS track bounds a glide that turned through its last 27 s to about
1–1.4 m/s, and says nothing of a glide on a fixed heading (§2). The swing under the main is real and measurable: on
issue #903's flight the barometer swings 6–10 m peak to trough every 2.5–4.5 s for about 20 s after the main opens,
then calms (§3). That is several times what the pendulum's own height change allows; a pressure error at the vent fits
its size, but nothing here proves it. **How far to trust it:** the canopy numbers are primary but mostly full scale or
1949 models of unknown porosity; every hobby size is **Inference**; the log results are measurements on a dozen public
flights, not a sample of hobby recovery. It moves issue #752's caveat, M7.3's wind fit and #903.

Builds on [wind-from-descent](wind-from-descent.md) §4 (Knacke's oscillation table, the pendulum period) and
[hobby-parachute-cd](hobby-parachute-cd.md) §3 (Knacke's C_D against descent rate). Researched 2026-10-28.

## 1. Glide: which canopies fly sideways, and how fast

- **Why a round canopy glides.** A solid flat canopy is statically unstable at zero angle of attack and stable at
  about ±20° [2, p. 17] or ±25° [1, p. 5-34]. With the payload's drag small, it hangs vertical with the relative air
  on a cone of that half-angle around it [2, p. 17]: if the air stays on one side, the canopy glides at that angle.
  From static tunnel coefficients, the trim is 32° for a hemisphere, 25° for an aeroconical canopy and 0° for a 4:1
  cross [3, p. 123]. Simulated at full size (100 kg, 5.1 m [3, p. 179]), the hemisphere settles to a 27° glide at
  6.8 m/s down (3.4 m/s sideways, computed here), the aeroconical 25° (3.1 m/s at 6.5), the cross none [3, pp.
  180–181, 206–207].
- **Lakehurst, 1949: hobby-sized models.** The canopies had 1,600 in² of cloth [1, p. 5-16], a nominal diameter of
  45 in (1.15 m; computed). Above 30 ft/s (9.1 m/s) they fell straight but oscillated violently; at 10 ft/s (3.0 m/s)
  they did not oscillate but glided at up to 45°; at 15–25 ft/s (4.6–7.6 m/s) they swung or glided [1, p. 5-16].
  Their fabric was likely permeable, unlike hobby nylon, and the report is lost [4, p. 4].
- **The glide in the drag curve (Inference).** Two unstable solid flat canopies in the same tests read C_D0 about
  1.20 and 1.12 at 10 ft/s, 1.06 and 0.97 at 15, 0.98 and 0.85 at 20, and 0.88 and 0.81 at 35 ft/s (read by eye)
  [1, Fig. 5-18, p. 5-18]; Knacke puts the rise down to the change from oscillation to glide [1, p. 5-18]. If a
  gliding canopy keeps the coefficient it has at 35 ft/s along its path, the vertical C_D grows as 1/cos²γ. Then γ
  is 31–32° at 10 ft/s, about 24° at 15 and 12–19° at 20 ft/s, and the sideways speed w·tan γ is 1.9, 2.0 and
  1.3–2.1 m/s. The assumption can err either way (oscillation also lowers the 35 ft/s value), so treat it as an
  order of size: 1–2 m/s at hobby main rates of 15–22 ft/s, up to the descent rate itself at 45°.
- **Indoor model drops.** Malewicki's lightly loaded kit and bag-plastic chutes glided in still air with C_D up to
  2.2; heavily loaded ones oscillated and fell vertically, with lower C_D (secondary: Flanagan's summary [4, p. 4]).
- **Size.** Larger canopies oscillate less. A 3 ft flat canopy of 1.1 oz/yd² ripstop swings ±35–40° in a tunnel, a
  28 ft personnel canopy ±30–35°, the 100 ft G-11 10–20°, a 200 ft canopy under 5° [1, p. 5-34]. Hobby mains
  (2–12 ft) sit at the unstable end, often in the same 1.1 oz ripstop (**Inference**).
- **Shape.** Average oscillation [1, Table 5-1, p. 5-3; Table 5-3, p. 5-4]:

| Knacke type | ±° | Hobby family it is closest to (**Inference** except where cited) |
|---|---|---|
| Flat circular | 10–40 | flat sheets, hexagons, Top Flight and Rocketman standard |
| Conical, biconical / triconical | 10–30 / 10–20 | none common |
| Hemispherical; extended skirt | 10–15; 10–15 | elliptical (Fruity Chutes Classic, Rocketman Elliptical) |
| Annular | under 6 | toroidal: Fruity Chutes' Iris, which it calls annular or pull-down apex [5] |
| Cross | 0–3 | cross and X-form chutes |
| Rotating (rotafoil, vortex ring) | 0–2 | Rocketman Rotating Star |

- **Porosity.** Oscillation, drag and opening force all fall as porosity rises [1, p. 5-71]. 3.5 ft ribbon canopies
  in a tunnel swing about 13° at 17% total porosity, about 10° at 20%, about 5° at 25% and 0° at 35% (read by eye)
  [1, Fig. 5-63, p. 5-72]; 35% is needed for zero oscillation at that size [1, p. 5-73]. Cloth of 27.4 ft³/ft²/min
  counts as 1% [1, p. 5-71]. Hobby canopies are near-zero-porosity nylon with a spill hole of a few percent of the
  area (Fruity Chutes' is 3.1% [6, §1]), far below where oscillation stops (**Inference**).
- **Makers say only words.** Fruity Chutes rates "stability", the tendency to stay over the load: cruciform good at
  any speed, flat poor at high speed, panel style good but "can rotate or spin", elliptical medium at high speed and
  good at low, toroidal good at low speed [7, p. 2]. No maker publishes an angle.

## 2. Turning: does the glide average out?

- **No measurement found** of the turn rate of an unmodified round, elliptical or toroidal canopy, hobby or full size.
- **Nothing sets a direction.** An axisymmetric canopy has no preferred glide direction [2, p. 17], so the heading is
  set by gusts and by the payload. Steady glide on one heading and coning (the glide direction sweeping round once
  per cone period) both occur: a flat canopy's natural motion is coning side to side about its descent line [8, p. 6],
  and a stable canopy with a draggy payload cones at its small trim angle [2, p. 13]. Coning cancels within a period
  or two; a held heading doesn't cancel at all (**Inference**).
- **What turns look like when forced.** A paratrooper canopy with slots cut across 11 gores, steered by lines, turns
  180° in 5 s at a glide ratio of 0.75 [1, p. 5-111]: likely an upper mark for how fast a gliding round canopy turns
  (**Inference**). On model rockets the canopy "may fly around in tight circles", or swing like a pendulum, disturbed
  by the payload [4, p. 6].
- **One GPS track (thread 135349, the #903 flight).** Under the main from about T+97 to T+124 s the receiver's 1 Hz
  positions give a ground velocity of 5.9 m/s on average; its east and north components scatter 0.73 and 0.74 m/s about
  straight-line trends, and its heading stays inside a 32° band (computed here [L1], with the GPS clock 104 s behind
  the altimeter's). A glide that rotated through a full turn in that time would add a component of amplitude g, scatter
  g/√2, so g ≤ about 1.0 m/s; over the clock offsets the two logs allow (101–104 s) the scatter is 0.66–0.96
  m/s and g ≤ 0.9–1.4 m/s (**Inference**, counting all the scatter as glide). A glide on a fixed heading is
  indistinguishable from wind.
- **The airframe's heading is not the canopy's.** On public AltOS TeleMega logs under a steady main, the airframe's
  rate about the vertical (gyro along the accelerometer's direction) is large: 87 °/s rms, peak 183 °/s and 0.68 sign
  changes a second on ISSUIUC's 2023 flight at 6.5 m/s, and 107–169 °/s rms on a subscale flight's full-rate log at
  7.5 m/s, depending on the window. It twists back and forth on its cord. On a full-scale flight at 7.0 m/s the
  airframe spins one way instead: gyro and magnetometer agree on −1,163° against −1,168° over 23 s (94 °/s rms;
  computed here). Either way the sensors track the airframe, which turns on its cord or swivel, so no log gives the
  canopy's turn [L2] (**Inference**).

## 3. The swing on the shock cord, and what the barometer sees (#903)

- **The model.** Computed for a full-size canopy and payload, the payload swings much more than the canopy, like a
  pendulum hung from the canopy's center of volume; the period goes as the square root of the line length (4.6, 5.6
  and 6.6 s for 7, 10 and 13 m) and is nearly independent of payload mass; damping coefficients are 0.08–0.23 and
  the motion settles in 15–30 s after a 20° disturbance [3, pp. 179–184]. Two-body stability has its own literature
  (White and Wolf, Wolf, Doherr) [1, p. 5-38], not read here.
- **What a swing does to the vertical speed.** In NASA's two-main Orion cluster tests the vertical velocity
  oscillated at twice the swing frequency [9, p. 3]; one test swung to 24.5° with a 14.0 s period [9, p. 5]. The
  mean descent rate during the swing stayed within 2% of the test's time average (30.16 against 30.76 ft/s; 31.08
  against 30.82 ft/s) [9, pp. 4, 6].
- **Breathing is faster.** 5–7 ft flat canopies on a guide wire, which stopped their coning, breathed at about 2 Hz
  [8, pp. 2, 6, 8].
- **Vortex shedding can drive the swing.** A ringsail drop test swung 40–45° from 70,000 to 57,000 ft, put down
  to shedding at a Strouhal number of about 0.17 locking onto the pendulum [10, PDF pp. 20, 23]. At 5 m/s on a
  1.5 m canopy that shedding is near 0.6 Hz, against 0.2–0.35 Hz for a pendulum (**Inference**).
- **The #903 flight, measured (20 Hz StratoLogger CF, `.pf2` SHA-256 `87e9cc91…`) [L1].** After the main charge at
  T+87.15 s, the height less a straight line peaks at T+95.8, 98.3, 102.8 and 106.5 s (2.5–4.5 s apart), 10 m peak to
  trough at first and 6 m by T+106 s. From T+109 s it is quiet: scatter 0.43 m about a line over T+112–126 s. Rates
  by window over T+92–126 s: 1 s, −14.6 to +6.7 m/s; 4 s least squares, −6.6 to −2.2; 8 s, −4.9 to −3.1.
  The mean rate in the swing (3.98 m/s, T+92–110) matches the quiet part (4.07 m/s, T+110–126).
- **1 s rates under the main on 10 flights** (17 distinct `.pf2` logs of 11 flights from six public collections; one
  flight dropped, its main not open, and one log of a two-altimeter flight failed the window check [L3]). Seven
  scatter about 0.5–1.4 m/s (the edges move with the landing cut; 1 s differences on a 1 s grid, from 5 s after the
  main charge to 2 s before landing), above the 0.3–0.7 m/s that their own 20 Hz noise gives through a 1 s difference.
  Three scatter about 2.0, 2.8 and 3.6 m/s, with 1 s rates from −16.8 to about +6 m/s; on one the swing hasn't faded
  50 s after the charge. Large swings are common, not a one-off (**Inference**).
- **What could make 6–10 m (Inference).** *Pendulum height:* on L = 3–5 m at 30–40°, the body rises at most L(1
  − cos θ) = 0.4–1.2 m and moves vertically at most about 0.7–1.7 m/s: several times too small. *Tilted drag:* a
  30° tilt cuts vertical drag by a quarter and speeds the fall by 15%, about 0.6 m/s. *Vent pressure:* a port reads
  ΔCp·V²/(2g) of height. With 4–7 m/s of airflow across the bay (descent plus swing) and ΔCp from +1 to −3
  (crossflow on a cylinder, potential flow [11, §4]; real flow separates and never recovers to +1 on the lee side [12,
  pp. 3-3 to 3-4]), that is 0.8–7.5 m, and only it covers the 6–7 m peaks. The peaks are sharp and read high (low
  pressure), the shape a side-port suction would give. Rises of 5–7 m in 1 s can't be real motion under a full main.

## What the project can use

- **Cite only:** Knacke [1] (cleared for public release, cached in `refs/papers/`), Cockrell [2] (AGARD, cached),
  Yavuz [3] (thesis, copyright the author, cached), Flanagan [4], maker pages [5, 7], Hoerner [12] (in
  `refs/papers/`), and the AIAA paper [8], the NTRS paper [9] and the Georgia Tech report [10] (cached under
  `refs/research/hobby-canopy-glide-and-swing/`).
- **Logs:** UND's two repositories are MIT; ISSUIUC's, NateOrlowski06's and saividyud's carry no license;
  AerospaceNU's is GPL-3.0 data; the forum thread states no terms. All are cached by earlier topics and cited here
  only through statistics; the scripts are in `refs/research/hobby-canopy-glide-and-swing/scripts/`.
- **M7.3's wind fit (suggestion).** Carry the canopy glide as an error term, not as zero: 1–2 m/s for flat canopies,
  up to about 3 m/s for elliptical ones (the computed hemisphere in §1), under about 0.5 m/s for toroidal and cross
  ones (**Inference** from the oscillation table). Its heading may hold for the whole descent, so averaging may not
  remove it (**Inference**). Effect: under a 75 s main, 75–225 m of drift the fit can't explain, beside the 100–400
  m from ERA5's miss (wind-from-descent §6). Fit over 8 s or two swings, not 1 s.
- **Issue #903 (suggestion).** The walk back from landing stops on this flight's swing. A rough re-run of its rule (2
  s median of 1 s rates) on the other public flights stopped late on none; only a real `hpr analyze` run settles how
  often (**Inference**). Over 8 s least-squares windows the #903 flight stays within −4.9 to −3.1 m/s, under its 5.8
  m/s mark; a 4 s window reaches about 6.6. A window of two swing periods (8 s or more), or starting the test once the
  scatter has fallen, would keep the drogue-to-main change visible. The CI test's invented flight can carry swings of
  6–10 m peak to trough at 2.5–4.5 s, fading over 20 s.
- **The drift test for body lift (suggestion).** Fly a cross or toroidal main, put the magnetometer on the canopy or
  its riser above a swivel (not in the airframe), log GPS at 5 Hz or faster, and measure the wind beside the
  descent (balloon or tower). The canopy's heading then shows whether it glides, and how far.
- **Monte Carlo landing spread (suggestion).** A glide of 0–2 m/s on a random, fixed heading per flight would widen
  the main-descent spread by about its speed times descent time; it needs a measurement before it becomes a model.

## Open questions

- A hobby canopy's turn rate and glide on a fixed heading: needs canopy-mounted heading data (none found).
- Glide against porosity and spill-hole size for zero-porosity nylon; Heinrich and Haak's porosity report (AFFDL-TR-
  71-58, DTIC AD0729858) was unreachable (DTIC under maintenance on 2026-10-28).
- Which vent layouts read the swing: the #903 bay's vents are unknown. A bay with a vent ring or a sealed reference
  would separate vent pressure from real motion (static-pressure-hobby-shapes §4 on rings).
- Not read: NAR R&D reports "Hole-y Chutes" (NARAM-61), "Spinning Parachute Effect on Descent Rate" (NARAM 53) and
  "Effect of Spin on Altimeter Data" (NARAM 50), member downloads; White and Wolf (1968), Doherr and Saliaris (1981).

## Sources

Read 2026-10-28 unless noted. [L1–L3] are this note's computations on public logs, scripts cached, not committed.

1. T. W. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), pp. 5-3, 5-4, 5-16, 5-18, 5-34,
   5-38, 5-71 to 5-73, 5-111; Tables 5-1, 5-3; Figs. 5-18, 5-63. <https://apps.dtic.mil/sti/pdfs/ADA247666.pdf>
   (read from the project's cached copy; DTIC was down on 2026-10-28).
2. D. J. Cockrell, *The Aerodynamics of Parachutes*, AGARDograph 295 (1987), pp. 13, 17.
   <https://apps.dtic.mil/sti/pdfs/ADA184620.pdf>; the copy read came from
   <https://everyspec.com/NATO/NATO-ARMP/AGARD-AG-295_39039/>.
3. T. Yavuz, *Aerodynamics of Parachutes and Like Bodies in Unsteady Motion*, PhD thesis, University of Leicester
   (1982), pp. 123, 179–184, 206–207.
   <https://figshare.le.ac.uk/articles/thesis/Aerodynamics_of_parachutes_and_like_bodies_in_unsteady_motion_/10181513>
4. D. Flanagan, "Drag Coefficients of Model Rocket Parachutes", *Peak of Flight* 668 (2025), pp. 4, 6.
   <https://www.apogeerockets.com/education/downloads/Newsletter668.pdf>
5. Fruity Chutes, "Iris Ultra Parachutes". <https://fruitychutes.com/parachute_recovery_systems/iris_ultra_parachutes>
6. [hobby-parachute-cd](hobby-parachute-cd.md) §1 (Fruity Chutes' spill hole, from its descent calculator).
7. Fruity Chutes, NARCON 2013 handout, p. 2.
   <https://fruitychutes.com/files/help/NARCON-2013-Rocketry-Recovery-Technology.pdf> (cached 2026-10-17).
8. B. Tutt, S. Roland, R. Charles, G. Noetscher, "Finite Mass Simulation Techniques in LS-DYNA", AIAA 2011-2592,
   pp. 2, 6, 8. <https://airborne-sys.com/wp-content/uploads/2016/10/aiaa-2011-2592-finite_mass_simulation_te.pdf>
9. E. S. Ray, R. A. Machín, "Pendulum Motion in Main Parachute Clusters", 23rd AIAA ADS conference (2015), pp. 3–6.
   <https://ntrs.nasa.gov/api/citations/20150003485/downloads/20150003485.pdf>
10. M. Bernatovich, I. Clark, "A Method to Characterize Parachute System Instability Modes", Georgia Tech AE 8900
    report (2013), PDF pp. 20, 23.
    <https://repository.gatech.edu/server/api/core/bitstreams/376ac795-2c41-474f-a085-e625659ccd78/content>
11. [static-pressure-hobby-shapes](static-pressure-hobby-shapes.md) §4 (crossflow ΔCp = sin²α·(1 − 4·sin²φ)).
12. S. F. Hoerner, *Fluid-Dynamic Drag* (1965), pp. 3-3 to 3-4 (cylinder pressure distribution).

- [L1] Rocketry Forum thread 135349 (<https://www.rocketryforum.com/threads/135349/>), the `.pf2` and the
  EggFinder GPX, no terms stated; scripts `pf2b.py`, `win.py`, `gpx.py`.
- [L2] ISSUIUC/flight-data@b8c95fa993 (`20230621/irec_2023_telemega.csv`) and
  saividyud/NASA_25-26_Rocket_Simulations@5ec2189860 (full-rate subscale and full-scale TeleMega logs), no license;
  also read: werocketry/Hyperion-II@efda132ff4, RocketPy-Team/RocketPy@9bd6ad3af8 (MIT), ISSUIUC's 2023-05-07
  EasyMega, NUSTARS/launch-vehicle@3a89e4be5a (no license). Script `altos2.py`.
- [L3] UND-ARC/RocketModels-2023@922c084f and UND-ARC/Nasa-SLI-2024@97bee76e (MIT), ISSUIUC/flight-data@b8c95fa993
  and NateOrlowski06/Rocket-Data@1f63804e36 (no license), AerospaceNU/rocket-flight-data@e951b1f (GPL-3.0 data),
  thread 135349; script `pf2all.py`.
