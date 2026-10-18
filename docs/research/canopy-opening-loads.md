# Opening loads and added mass of hobby parachutes

**In short: the simulator's default opening (`Instant`, full drag area at line stretch, no overshoot) reads a hobby
main's peak load 2.3 to 3.5 times above the only measured hobby mains, and RocketPy's added mass accounts for the whole
RocketPy-to-HPR Sim descent gap.** Knacke's opening-load factor C_x (1.6 hemispherical, 1.7 flat, 1.8 conical [1,
Table 5-1]) holds only when the load can't slow. A hobby main sits at a mass ratio near 2, about 20 times past
Knacke's data, where the factor falls well below 1 [2, p. 10]; two load-cell flights of a 72 in main give
F_peak/(q C_D S) of 0.43 and 0.29 [3]. Re-running the project's RocketPy oracle without added mass brings all 30
descent comparisons within 0.36% (NDRT's north drift: +2.865% to −0.360%, §6). Published added-mass coefficients
scatter tenfold; lateral is a fifth to a tenth of axial on round canopies [13, p. 40]. **How far to trust it:**
Knacke's factors, RocketPy's formula and the probe are primary or reproduced; the hobby loads are two flights on one
rig that saw one section's share, plus a plot read by eye; every number marked **Inference** is computed here.

This builds on [suite-parachutes](suite-parachutes.md) (Knacke's force formula and Pflanz),
[hobby-parachute-cd](hobby-parachute-cd.md) (canopy C_D) and [wind-from-descent](wind-from-descent.md) (the descent
lag w/g). The simulator's current opening load is in [recovery](../physics/recovery.md#the-opening-load).

## 1. Knacke's opening-load factor is an infinite-mass number

- **The formula** [1, p. 5-50]: F = (C_D S) q C_x X_1, q at line stretch. C_x is the peak over steady drag in
  wind-tunnel (infinite-mass) tests; X_1 allows for the load slowing while the canopy fills, 1 at infinite mass and
  "as low as 0.02" for lightly loaded final-descent parachutes.
- **C_x by type** [1, Table 5-1, p. 5-3]: flat circular about 1.7; conical and polyconical about 1.8; hemispherical
  about 1.6; extended skirt and annular about 1.4; guide surface 1.2 to 1.4; cross 1.1 to 1.2. There is no toroidal or
  elliptical row; the nearest are annular and hemispherical.
- **Knacke's own limits.** "Do not use C_x at low canopy loading" [1, p. 5-54], and his Pflanz method multiplies by
  C_x only "for high canopy loading conditions" [1, p. 5-59]. His X_1 by canopy loading [1, Fig. 5-48, p. 5-55], read
  by pixel: 0.02 at 0.31 psf, 0.10 at 1, 0.20 at 2, 0.51 at 10, 0.81 at 40 psf.
- **Mass ratio** [1, p. 5-56; 2, eq. 2]: R_m = ρ(C_D S)^{3/2}/m, m the vehicle plus decelerator. C_k = C_x X_1 levels
  off at 1.0 to 1.6 below R_m = 0.001 (near 1.6 for low porosity, 1.1 to 1.2 for porous canopies) and falls to
  fractions of that above R_m = 1, as C_k ~ 1/(n_fill R_m) [2, p. 10]. Knacke's Figure 5-50 spans R_m 1e-4 to 1e-1,
  for ribbon and ringsail canopies only.

## 2. Where hobby recovery sits (Inference, invented examples)

- **A main.** A 10 kg rocket under a 2.4 m elliptical canopy, C_D 1.5 on projected area: C_D S = 6.79 m², R_m = 2.2,
  loading 14.5 Pa (0.30 psf), descent 4.9 m/s; at 25 m/s, q C_D S = 2.6 kN (26.5 g). Figure 5-48 would give
  X_1 ≈ 0.02, below the weight; Knacke gives no deployment speeds for its data, so it isn't applied here. Pflanz with
  n = 2 [1, pp. 5-56 to 5-59] gives X_1 = 0.27 to 0.08 for fill times of 0.3 to 2.0 s horizontally, and 0.32 to 0.12
  falling vertically: gravity raises the peak.
- **A drogue.** A 0.6 m drogue (C_D S = 0.42 m²): R_m = 0.034, 231 Pa (4.8 psf), descent 19 m/s, inside Figure 5-50,
  where C_k is about 0.8 to 1.0 (ribbon data; solid low-porosity canopies level off higher [2, p. 10]).
- **A unit trap.** Two student works in §3 compute R_m with weight in pounds where mass in slugs belongs, so their R_m
  is 32 times too small (UAH: 0.068 for 2.2). That puts a main on ribbon-drogue data; UAH's predictions ran 17% and
  75% over its measured loads [3, Tables 1 and 2, p. 20].

## 3. Measured hobby and student deployments

| source | canopy | mass | speed | sensor | measured | factor |
|---|---|---|---|---|---|---|
| UAH, flight 1 [3] | 72 in toroidal, C_D 2.2 | 7.82 kg | 21.4 m/s | load cell, 10 kHz | 155.7 lbf (9.0 g) | 0.43 |
| UAH, flight 2 [3] | same | same | 23.2 m/s | same | 122.9 lbf (7.1 g) | 0.29 |
| NDRT 2022–23 [4] | commercial main | not read | drogue rate | IMU, 100 Hz | about 8 g | ≈ 0.52 |
| EUCASS 2015 [6, Table 4] | cross, 0.46 m² | tunnel | 50–97 m/s | load cell, 100 kHz | C_x 1.07–1.16 | infinite mass |

The factor is F_peak/(q C_D S), recomputed here; NDRT's is 8.08 g read by eye over its predicted 15.56 g of drag.

- **UAH** [3] (flown April 2026): a commercial toroidal main on 9/16 in tubular nylon; peaks 2.5 and 2.75 s after
  the main charge, on a 5 ms rolling mean; q C_D S = 364.9 and 429.7 lbf. The cell sat at the fin can, one section's
  share; the authors say the loads measured could be lower than the canopy's [3, p. 21]. **Inference:** vertical
  Pflanz (n = 2) matches them with fill times of 0.20 and 0.36 s at C_x = 1, 0.48 and 0.90 s at 1.6.
- **Notre Dame** [4] predicted 14.56 g net of gravity at main deployment with C_x = 1 (printed p. 56). Its 100 Hz plot
  (Figure 106, p. 100) shows the charge spike, then about 260 ft/s² (8 g, read by eye) as the main opens 2.3 s later.
  The true peak may be higher at 100 Hz; a damaged drogue (p. 91) means a faster deployment, so a lower true factor.
- **CSU Long Beach** [5] gave C_x 0.29 from a leaf-blower test; below 1, so (**Inference**) its q was overstated.
- **One builder's traces** [8] (0.46 kg, 18 in main): +86 g from the apogee charge, −8 g of snatch, −93 g at the
  main's shear-pin break, and no worrying peak at opening. Deployment spikes are mostly charge and separation.
- **Hobby texts.** Nakka gives C_x 1.7 flat, 1.6 ellipsoidal, 1.2 cross, and designs to X_1 = 1 though Pflanz gives
  his 3 kg rocket 0.21 [7]. No factor was found from Fruity Chutes [9], Rocketman, Apogee, NAR or Tripoli.

## 4. Bounds, and which way the simulator errs

- **Above.** No source gives a solid-canopy C_x above 1.8; Potvin and Ray's plateau tops out at 1.6 [2, p. 10]. The
  infinite-mass value C_x q C_D S bounds the inflation load in every source read.
- **Not bounded by that.** Snatch can reach or pass the opening peak when the parachute is 3 to 7% of the mass
  decelerated [1, p. 5-52], a band hobby chutes fall in (Nakka's is about 4% [7]). Charge kicks and cord recoil add
  more [8]. No published hobby-sized magnitude bounds them.
- **The simulator's sign (Inference).** `Instant` (C_x = 1, X_1 = 1) over-reads a hobby main by 1/0.43 to 1/0.29, 2.3
  to 3.5 times: safe for hardware, wrong as a typical load. For a drogue it runs from about 40% high (C_k 0.7) to 40%
  low (a solid canopy near 1.6), so it can flatter there. `FillingTime` integrates the slowing, the right family for
  mains; it leaves out the 1.4 to 1.8 overshoot, which matters most near infinite mass, for drogues. Neither has snatch.

## 5. Added-mass coefficients

Sources use four volumes. Here k_p = m_a/(ρ·πD_p³/12), the air in a hemisphere of the inflated (projected) diameter.

| source, method | axial k_p | lateral k_p |
|---|---|---|
| 2:1 oblate spheroid, potential flow [17; 14, p. 49] | 1.12 | 0.31 |
| Thin hemispherical shell with enclosed air, potential flow [14, pp. 97, 176] | 2.1 | 1.3 |
| Ibrahim 1965, spherical cup (secondary [13, p. 40]) | about 2.6 | none |
| Round canopy in a water tank, Cockrell et al. (secondary [13, Table 4.3]; D_p = 0.7 D_0) | about 3.2 | about 0.58 |
| Doherr and Saliaris, simulation values (secondary [13, eq. 4.19, p. 40]) | 0.99 | 0.50 |
| Eaton, tuned to flight data (secondary [15, p. 3]) | 1.2 | 0.6 |
| Dragonfly ringsail, wind-tunnel coning fit [12, p. 10] | 0.36 | 0.13 |
| UD233 guide (Heinrich): enclosed air plus a quarter (reviewed in [16, eqs. 4, 6, pp. 4, 5]) | 1.25 | none |
| RocketPy 1.13.0 (§6) | 1.0 | 1.0 |

- **Lateral is small on round canopies:** one fifth to one tenth of axial, with high uncertainty [13, p. 40]. Yet
  Dragonfly's drop tests matched attitude rates best with lateral equal to axial [12, pp. 13, 14].
- **What moves them.** Yavuz's ship-tank tows: k depends on shape, angle of attack and the acceleration modulus
  δ = aD/V², threefold for a tenfold change in δ [14, p. 96], and at low δ runs several times the potential-flow value
  [14, p. 151]. Porosity lowers it: Guglieri's fit gives 1.068 at zero porosity, 0.910 at 0.1, 0.755 at 0.2 and 0.610
  at 0.3 [10, eq. 10, p. 4].
- **At inflation,** a growing m_a adds (dm_a/dt)V to the force, raising opening loads [11, eq. 2]; NASA's CPAS
  reconstructions matched peak loads with enclosed air only [18, p. 7].

## 6. RocketPy's model, and a probe without it

- **The formula** (RocketPy 1.13.0, MIT [19]; `rocketpy/simulation/flight.py` lines 2754 to 2761): m_a =
  k_a·ρ·(2/3)π·R²·H, ρ at the current height, R = √(C_D·S/(1.4π)) unless given, H = R (`rocketpy/rocket/parachute.py`
  lines 244 to 254). k_a is Guglieri's porosity fit [10, eq. 10], uncited in the code: 1.000 at the default 0.0432.
- **One scalar on every axis, inertia only:** acceleration is D/(m + m_a), the vertical (D_z − m·g)/(m + m_a)
  (flight.py lines 2775 to 2777), so steady descent is unchanged. Coriolis is subtracted after the division, undivided
  (lines 2780 to 2783), so drag must supply (m + m_a) times it. The canopy opens fully at once, after the lag.
- **NDRT 2020's main.** C_D S = 16.05 m² gives R = 1.910 m and 14.60 m³. At 1,000 m, where the descent starts, ρ =
  1.0892 kg/m³ gives 15.906 kg, the "15.9 kg" in [recovery](../physics/recovery.md). At the main's trigger, 167.9 m,
  ISA ρ ≈ 1.1816 gives 17.26 kg: m_a/m = 0.828 on the 20.85 kg rocket.
- **The probe (Inference, one run each, 2026-10-18).** A copy of `validation/oracles/rocketpy/recovery.py` set every
  parachute's radius and height to 1e-6 m, which in RocketPy feeds only m_a; unchanged, the copy reproduced the
  committed fixture to the six digits compared. Without m_a, the largest of six gaps per case:

| case | with RocketPy's m_a | without |
|---|---|---|
| Calisto | +0.080% | −0.027% |
| Valetudo (north drift) | −1.766% | −0.011% |
| NDRT 2020 (north drift; descent time) | +2.865%; +0.705% | −0.360%; −0.116% |
| Prometheus 2022 | +0.086% | −0.027% |
| Juno III | −0.018% | −0.007% |

- **Valetudo's gap is added mass too.** Its still-air drift comes from the Earth's rotation; with Coriolis undivided,
  drag must also push m_a, so the drift grows by 1 + m_a/m = 1.0089 (0.0736 kg on 8.258 kg): the whole −0.889% gap.
- **The opening peak.** NDRT's +83.059% and Prometheus 2022's +19.062% peak-deceleration gaps at the main
  ([accuracy](../accuracy.md)) equal m_a/m where each main opens, 0.830 (1 s after its trigger) and 0.190: opening at
  once with a constant m_a, RocketPy's peak is HPR Sim's divided by 1 + m_a/m.
- **Closed form** (Inference, exact for instant opening): Δt = (m_a/m)(w/g)·ln((1 + v₁/w)/2), v₁ the speed at
  opening, w the descent rate. NDRT: 0.494 s, against the probe's 0.503 s.

## 7. Size on hobby rockets (Inference; invented examples, ρ = 1.225 kg/m³)

With m·g = ½ρC_D S w² and RocketPy's R, m_a/m = 0.952·k_a·R·g/w², and the horizontal time constant τ = (1 + m_a/m)·w/g.

| case | C_D S | w | m_a (RocketPy) | m_a/m | τ without → with |
|---|---|---|---|---|---|
| A: 10 kg, 2.4 m canopy | 6.79 m² | 4.86 m/s | 4.92 kg | 0.49 | 0.50 → 0.74 s |
| B: 1.5 kg, 0.9 m canopy, C_D 0.84 | 0.534 m² | 6.70 m/s | 0.109 kg | 0.072 | 0.68 → 0.73 s |

- **Across sources,** case A's lateral m_a runs from 0.56 kg (Dragonfly's fit) to 5.76 kg (the potential-flow
  shell), m_a/m 0.06 to 0.58. Wind changes are low-δ (a 1 m/s step 0.14, a 0.01 s⁻¹ shear 0.005), where measured
  values run several times higher [14, p. 151]; no source bounds the upper end.
- **Drift and time.** RocketPy's m_a shortens A's descent by 0.27 s and B's by 0.04 s from 25 m/s: 1.4 m and 0.2 m
  less drift in a 5 m/s wind, and 0.04 to 0.43 m for A in 1.5 m/s of shear, all under 1% of a 150 m drift. Leaving
  m_a out errs long. The instant-opening peak is divided by 1 + m_a/m: 0.67 of HPR Sim's for A, 0.93 for B.
- **No hobby-scale measurement** of added mass was found; the nearest are Eaton's tuning [15, p. 3], Dragonfly's
  subscale drops [12] and NASA's CPAS flights [18].

## What this suggests

- **The docs can state the measured cause.** recovery.md and accuracy.md call added mass "the likely cause" of NDRT's
  +2.86% and +0.71%; §6 measures it (−0.360% and −0.116% without it). Valetudo's −0.889% is added mass acting on
  Coriolis. The "15.9 kg" is at 1,000 m (as recovery.md's mass section says); the main opens at 0.830.
- **An opt-in, weightless m_a = k·ρ·(2/3)πR³** would reproduce RocketPy within 0.36% on all five cases. A physical
  model would split axial (k_p about 1) from lateral (0.1 to 0.6) and print the spread.
- **Opening loads for mains** belong to `FillingTime`'s family (Pflanz or Ludtke, X_1 from the mass ratio, not
  Figure 5-48), with C_x 1.4 to 1.8 printed as the infinite-mass bound; snatch needs its own output or a stated gap.
- **A measurement would settle the main's factor:** a logger at 1 kHz or more, load cell at the canopy's swivel.

## Open questions

- Unreachable: Ludtke's NSWC TR 86-142 and AFFDL-TR-78-151 (DTIC refused scripted fetches, then showed a maintenance
  page, 2026-10-18); the constant in C_k ~ 1/(n_fill R_m) (Wolf 1999, Potvin 2007, paywalled); Ibrahim 1965 and 1967,
  Klimas 1977, Lingard 1995, Macha 1993, Eaton, Yavuz and Cockrell 1981.
- No source read bounds lateral added mass at hobby δ, hobby snatch, or the UAH rig's section share; no public hobby
  main is measured at the canopy.
- AGARD's 0.70 and 0.21 for an L/D 0.5 ellipsoid [13, p. 39] disagree with Brennen's 1.12 and 0.31 [17].

## Sources

1. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), DTIC ADA247666. Cited, not reproduced.
2. Potvin and Ray, CPAS opening-shock factor and mass ratio (2017). https://ntrs.nasa.gov/citations/20170003949
3. Mills, UAH honors capstone (2026). https://louis.uah.edu/cgi/viewcontent.cgi?article=2021&context=honors-capstones
4. Notre Dame Rocketry Team, 2022–23 FRR. https://seniordesign.ee.nd.edu/2023/DesignTeams/rocket/NDRT_FRR_2022_2023.pdf
5. Alejos, CSU Long Beach honors thesis (2022). https://scholarworks.calstate.edu/downloads/c821gr23j
6. Marciniak and Kindracki, EUCASS 2015. https://www.eucass.eu/component/docindexer/?task=download&id=3524
7. Nakka, Appendix K, parachute design. https://nakka-rocketry.net/Appendix-K.htm
8. Rocketry Forum, post 28 (March 2026), https://www.rocketryforum.com/threads/
   formula-for-recovery-system-failure-load-vs-rocket-weight.196302/
9. Fruity Chutes help pages (FAQ, certification). https://fruitychutes.com/help_for_parachutes
10. Guglieri, Parachute-payload system flight dynamics, Int. J. Aerospace Eng. 182907 (2012), CC BY.
11. Cruz, Kandis and Witkowski, DGB opening loads, AIAA 2003-2131. https://ntrs.nasa.gov/citations/20030065911
12. Guecha-Ahumada, Cruz et al., Dragonfly parachute apparent mass (2026). https://ntrs.nasa.gov/citations/20260003537
13. Cockrell, *The Aerodynamics of Parachutes*, AGARDograph 295 (1987).
14. Yavuz, Parachutes and like bodies in unsteady motion, PhD thesis, Leicester (1982), figshare 10181513.
15. Ginn, Clark and Braun, Parachute dynamic stability and apparent inertia, AIAA 2014-2390. hdl.handle.net/1853/51987
16. Fraire, Dearman and Morris, Added mass of Orion drogues (2011). https://ntrs.nasa.gov/citations/20110011303
17. Brennen, *Internet Book on Fluid Dynamics*, Values of the added mass, eqs. B5 to B7 (Caltech).
18. Ray, Added mass load components of CPAS main clusters (2017). https://ntrs.nasa.gov/citations/20170004358
19. RocketPy 1.13.0 (MIT), https://github.com/RocketPy-Team/RocketPy; files and lines as in §6.

All read 2026-10-18.
