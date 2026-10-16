# What the Ironbark paper measured, and what it means for HPR Sim

This note reads Galletly et al. 2026 [1], on the University of Sydney team's Ironbark simulator, and its 2023 version
[2], for [apogee-bias](apogee-bias.md) and the panel-method tier in [ideas](ideas-design-and-physics.md).
**In short: OpenRocket's +7.5% and +8.2% are sourced, and they come from one rocket.** On Bluewren's test flight
the paper puts OpenRocket's apogee 7.49% above the official TeleMega recorder and 8.22% above the Raven4 ([1]
Table 3, p. 9; text, p. 8; apogees in Table 1, p. 7), and 4.49% above the TeleMega at the competition. The paper
names no cause for that miss, and the flights' data are not shared, so Bluewren can't join the reference set.

Ironbark's total drag (PANAIR's panels plus its friction and base terms) came within +4% to +15% of a NASA wind
tunnel with the fins on from Mach 1.5 to 2.96 (fins off, +5% to +19%), where HPR Sim's forebody drag misses by +39%
to +105% (a different quantity, §3). Below Mach 1, though, its normal-force slope is 25% to 48% high, and HPR Sim's
is closer. As evidence this is a peer-reviewed paper by the tool's authors: one rocket, two flights, and Ironbark
apogees re-run after the flights by a later version. Values marked "read" come from the paper's vector figures
(scripts in the gitignored cache), to about ±0.002 in $C_A$ and ±0.1 per radian.

## 1. The flights and each code's apogee error

One rocket, two flights ([1] pp. 6, 8). Bluewren is 3.2 m (10.5 ft) long and 0.143 m (0.469 ft) in diameter. Its test
flight from Tolarno Station, New South Wales (57 m above sea level), used a CTI O3400, with GFS weather. At the 2022
Spaceport America Cup (1,401 m) it flew a CTI N3301, with RAP weather and ballast. Two recorders flew; the TeleMega
was the competition's official one and the paper's benchmark ([1] p. 8). They disagree by 0.68% and 0.49% of apogee
([1] Table 3, p. 9), and the paper treats that as the measurement uncertainty.

Apogees from [1] Tables 1 and 2 (pp. 7–8); the percentages are this note's arithmetic. They match Table 3 (p. 9)
and the text (pp. 7–8) to 0.01 point at Tolarno. At the Cup, Table 3's magnitudes differ from Table 2's apogees by
0.02 to 0.04 point, as if the TeleMega had read 9,121 m, not 9,124 m.

| Source | Tolarno, m | vs TeleMega | vs Raven4 | Cup, m | vs TeleMega |
|---|--:|--:|--:|--:|--:|
| TeleMega (measured) | 9,604 | 0 | +0.68% | 9,124 | 0 |
| Raven4 (measured) | 9,539 | −0.68% | 0 | 9,076 | −0.53% |
| Ironbark, own aero | 9,722 | +1.23% | +1.92% | 9,107 | −0.19% |
| Ironbark, CFD aero | 9,558 | −0.48% | +0.20% | 9,155 | +0.34% |
| RocketPy, CFD drag | 9,145 | −4.78% | −4.13% | 8,716 | −4.47% |
| OpenRocket | 10,324 | +7.50% | +8.23% | 9,531 | +4.46% |
| RockSim | 7,698 | −19.85% | −19.30% | 8,003 | −12.29% |
| RASAero | 7,979 | −16.92% | −16.35% | 8,964 | −1.75% |

- **The prediction made before a flight** is the Cup's: Ironbark with CFD set the ballast for 30,000 ft, and the
  rocket reached 29,933 ft, 0.2% low ([1] p. 8).
- **The other Ironbark rows were re-run later.** They come from the paper's latest version, not the 2022 runs nor
  the 2023 paper ([1] p. 6). In 2023 the same flights gave Ironbark 9,636 m and 9,447 m (CFD) at Tolarno, and
  9,046 m and 9,147 m at the Cup ([2] Tables 2–3, pp. 9, 11). The papers also disagree on which aero matched
  Tolarno better: 2023 says Ironbark's own ([2] p. 10), 2026 says the CFD ([1] p. 8).
- **OpenRocket in 2023:** 10,320 m at Tolarno (+7.45%, +8.19%) and 9,603 m at the Cup (+5.25%; the paper prints
  8.18% and 5.26%) ([2] Table 2, p. 9; Tables 3–4, p. 11). Neither paper gives its version or inputs.
- **Small errors in the paper:** the text cites Table 5 for the R² values (they are Table 4, p. 9) and for the
  apogee-distribution fits (Tables 6 and 7, p. 11).

## 2. Where each code missed, and the causes the authors give

- **RocketPy, low by 4.1% to 4.8%** with the same CFD drag. The authors blame its drag being a function of Mach
  alone, with no angle-of-attack term, and its simplified other coefficients ([1] p. 7).
- **OpenRocket, RockSim and RASAero: no flight-specific cause.** The paper only says semi-empirical tools lose
  accuracy outside subsonic flight ([1] pp. 2–3, 9–10). OpenRocket's mass, motor file and weather are unstated, so
  input causes are unbounded.
- **A clue from peak speed (Inference).** At Tolarno OpenRocket's peak speed was 4.76% high; at the Cup 1.51% low
  ([1] Table 3, p. 9; Tables 1–2 give +4.70% and −1.46%). Its apogee was high on both. So on this rocket its excess
  built up after burnout, which points at too little coast drag. HPR Sim's supersonic pressure drag is about twice
  OpenRocket's, yet on the one supersonic flight compared it read 13.60% above OpenRocket, since it relieves base
  drag under power ([aero](../physics/aero.md),
  [#222](https://github.com/nrdptel/fusionspace-eridanus/issues/222)). Its sign on Bluewren is unknown.
  Bluewren peaked at about 575 m/s, roughly Mach 1.7 (Inference, with a sound speed near 340 m/s).
- **Ironbark's sign flipped between flights**, +1.23% to −0.19%. The authors cite weather, wind profiles, flight
  dynamics and sensitivity to drag and mass, without sizes ([1] p. 8).
- **Weather.** The paper finds the standard atmosphere wrong for the Cup's high, warm site (Fig. 6), with no apogee
  size ([1] p. 4). If OpenRocket used it, that would read low, not high (Inference: colder air is denser). Over 8,250
  daytime June ERA5 hours since 2000 ([1] p. 10), weather alone spread the apogee by a standard deviation of 262 ft
  (0.85%), skewed low ([1] Table 7, p. 11: variance 68,596 ft², skew −1.97; the root is this note's).
- **Motor.** No motor cause is discussed. Ironbark leaves out ambient pressure's effect on thrust ([1] p. 4), which
  reads apogee low; [apogee-bias](apogee-bias.md) sizes that effect at 0.3–1.5%.
- **Recovery** doesn't touch apogee. Predicted landing speeds were 6.71 to 7.72 m/s on the day's three weather
  models, against 7.9 m/s measured: 2% to 15% low ([1] p. 11).

**Bearing on HPR Sim's real-flight bias.** It matches in sign: OpenRocket reads high, as it does by +7.79% on the
private collection ([report](../../validation/reports/fixture-flights.md)). But it is one supersonic rocket, the
inputs are unstated, and the likely cause (coast drag past Mach 1) says nothing about subsonic flights. It moves no
number in [apogee-bias](apogee-bias.md) beyond sourcing the figures.

## 3. The panel method: what it covers, and how well

**What it covers** ([1] pp. 2–4, 6):

- **PANAIR**, NASA's higher-order panel code for linearized potential flow, run at subsonic and supersonic Mach. It
  can't model viscosity, separation, transonic flow or total-pressure changes ([1] p. 3).
- **No transonic results.** PANAIR isn't run from Mach 0.9 to 1.2. A drag curve built after Raymer is interpolated
  across that band, and the other coefficients linearly ([1] p. 6).
- **Tables over Mach and angle of attack.** One plane is solved, then rotated by the roll angle ([1] p. 3). Bluewren's
  tables cover 0°, 5° and 10°, to Mach 2.0 ([1] Fig. 3, p. 4). In 2023 the code matched a tunnel at small angles and
  drifted at high ones, since PANAIR has no separation ([2] p. 7).
- **Geometry:** read from an OpenRocket file or typed in, including hexagonal fin sections and complex planforms
  ([1] p. 2). It is meshed by a modified pyPanair into NASA's LaWGS wireframe format ([1] p. 3).
- **Viscous terms on drag only.** Skin friction is built up part by part, after Gur, Mason and Schetz's FRICTION:
  Blasius with Eckert's reference temperature when laminar, van Driest II when turbulent. Transition is at maximum
  thickness, and Barrowman's rule handles roughness ([1] pp. 3–4). Base drag follows the Aeroprediction Code, with a
  quadratic boattail term ([1] p. 4). Below Mach 1, PANAIR's own axial force is near zero or negative ([1] Fig. 3,
  p. 4), so subsonic drag is all empirical (Inference).
- **Cost:** 324.7 s a run with its own aero, against 1.72 s for OpenRocket, on an eight-core i9-9900 ([1] Table 5,
  p. 10). CFD took hours a case, and 34 cases ([1] pp. 9–10).

**Against the Arcas wind tunnel** (NASA TN D-4013 and D-4014 [3, 4]; the short model; [1] Figs. 9–10, pp. 6–7). The
same reports are in HPR Sim's fixture ([aero](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). Ironbark
against the tunnel values the paper plots (read):

| Mach | 0.6 | 0.8 | 0.9 | 1.2 | 1.5 | 1.8 | 2.3 | 2.96 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| $C_A$, fins on | +11% | 0% | −27% | +22% | +6% | +4% | +7% | +15% |
| $C_A$, fins off | +19% | +21% | +7% | +14% | +19% | +7% | +5% | +10% |
| $C_{N\alpha}$, fins on, against the paper's tunnel values | +31% | +54% | +61% | +12% | +20% | +5% | +11% | +15% |
| $C_{N\alpha}$, fins on, against HPR Sim's fixture | +25% | +46% | +48% | +10% | +3.5% | +0.1% | +3.1% | +6.0% |
| HPR Sim's $C_{N\alpha}$, against its fixture | −4.6% | +9.5% | +25.0% | +20.2% | +8.8% | +3.9% | +6.2% | +3.7% |

- **Normal force.** Below Mach 1 the panel method's slope is 25% to 48% high. HPR Sim's is −4.6% to +25.0%
  ([aero](../physics/aero.md#normal-force-through-mach-1)). From Mach 1.5 up, they are within 6.0% and 8.8% of the
  fixture. The paper's own tunnel slopes sit 2% to 14% below the fixture's (most at Mach 1.5), which shows how
  loosely the plots pin a slope. The paper calls the subsonic trend "slightly divergent" ([1] p. 6).
- **Drag (Inference).** At Mach 0.6 the paper's tunnel value with the fins on, 0.259, is below the base-corrected
  forebody value in HPR Sim's fixture, 0.2987. So it is likely the raw sting-mounted reading; TN D-4013 says its
  $C_A$ plots are not corrected for base axial force ([3] p. 4). The comparison then mixes Ironbark's free-flight
  base drag with a sting's base, a difference near 0.04 here. It is not like-for-like with HPR Sim's +39.4% to
  +105.0% forebody error from Mach 1.5 to 2.96. Still, the gap differs in kind: HPR Sim takes the thin double-wedge
  fins as blunt-edged.
- **Pitching moment: no verdict.** The paper doesn't state Ironbark's moment center. The reports use 70% of the
  length from Mach 0.6 through 1.2 and 63.37% from 1.5 ([3] p. 2; [4] p. 2), and the paper's tunnel values fit
  each report's own center (Inference: the fixture's center of pressure gives −5.8 per radian at Mach 2.96; the
  plot, −5.6). If Ironbark used 63.37% throughout, that shift alone is 17 to 18 of the 19 to 28 per radian gap from
  Mach 0.6 to 0.8 (Inference: its $C_{N\alpha}$ times the 1.2 calibers between the centers).
- **The 2023 supersonic check** used a NASA wingless-missile model with four fin planforms ([2] p. 6). Ironbark's
  zero-lift drag was 3.3% to 10.6% off at Mach 1.6 (two high, two low). It was 8.5% to 20.8% high at 2.38 and 32.3%
  to 43.2% high at 2.83 ([2] Table 1, p. 7). The authors blamed friction formulas made for subsonic flow.

**What a tier in HPR Sim would need (Inference).** First, a linear-potential panel solver: PANAIR is public domain
(PDAS's legal page [6] says the programs are not copyrighted, though its collection is). Second, a mesher from
HPR Sim's geometry; pyPanair, the paper's pre-processor, is MIT [7]. Core crates do no I/O and must build for
WebAssembly, so the solver would be an offline table builder. Its tables would feed the existing path that lets a
flight take another program's drag, normal force and center of pressure
([aero](../physics/aero.md#the-normal-force-from-rasaero-ii)). HPR Sim's friction and base drag would still be
needed, and a transonic method, which the panel code doesn't give. The Arcas fixture already exists to score it.

## 4. Is any of it usable as a reference?

- **No flight data.** The data statement: "The authors do not have permission to share data." ([1] p. 12).
- **No code.** The paper names no Ironbark repository; the code was "developed internally" ([1] p. 2). A GitHub
  repository search for "ironbark rocket" found none (2026-10-15). It names pyPanair (MIT [7]) and PANAIR (public
  domain [6]). Neither was read here beyond its license and description.
- **The published numbers can't fly in HPR Sim.** Length, diameter, motors and their impulses (21,062 N·s and
  19,318 N·s, [1] p. 8), sites, apogees and peak values are given. Mass, center of gravity, fins and a drag curve are
  not. So Bluewren can be cited as an OpenRocket-versus-flight case, not added to the reference set.
- **The tunnel data are public and already used:** [3, 4], NASA, "public use permitted" on NTRS. The 2023 paper's
  wingless-missile model is NASA TM X-2774 (Trescot, Foster and Babb, 1973, Mach 1.60, 2.36 and 2.86 by its title)
  [5]. It is on NTRS with the same terms and isn't in the project; only its record was read.

## What this suggests

1. **Source apogee-bias's figures.** Its open question on Ironbark can cite [1] Table 1 (p. 7) and Table 3 (p. 9):
   OpenRocket +7.49% (TeleMega) and +8.22% (Raven4) on Bluewren's test flight, and +4.49% at the Cup (as printed).
   Add "one supersonic rocket, inputs unstated." Expected effect: the open question closes; no bias number moves.
2. **The panel-method tier: keep at SOON; neither promote nor drop.** The license question is answered: PANAIR is
   public domain, pyPanair MIT. Not promoted: on the one design where both can be scored, it is worse than HPR Sim
   below Mach 1 in $C_{N\alpha}$ (25% to 48% high), and with its moment center unstated, which way that moves the
   margin is unknown. It has no transonic answer, runs about 190 times slower than OpenRocket, and no user is
   named. Not dropped: past Mach 1.5 its drag tracks the tunnel within +4% to +15% (fins on), where HPR Sim's fins
   miss widely. The step that would decide it: run PANAIR offline on the Arcas design and score it against the
   committed fixture, a reference that doesn't touch the core. Expected effect: a measured answer either way.
3. **Fix the cheaper cause first.** The Arcas supersonic drag gap is mostly the fins' edges. A thin-section
   supersonic fin wave-drag term (linear theory, as PANAIR's) may close most of it without a panel code (Inference;
   it needs its own source). Its sign: lower supersonic drag, higher supersonic apogees. Its size on flights is
   unbounded here.
4. **Weigh TM X-2774 [5]** as a second supersonic reference: four fin planforms of one exposed area ([2] p. 6), the
   variable the Arcas data lack.
5. **Don't add Bluewren** to the real-flight set: its inputs are not published.

## Open questions

- Ironbark's moment reference center in Fig. 10, and whether its Fig. 9 curve includes its base drag.
- OpenRocket's version, mass, motor file and weather for both flights. Whether each recorder's apogee was barometric
  or GPS.
- Why Table 3's Cup column fits a TeleMega apogee of 9,121 m, not 9,124 m.
- The validation on "other vehicles" the paper cites ([1] p. 6, refs. 35–36) is not shown.

## Sources

All read 2026-10-15. Copies in the gitignored `refs/papers/` and `refs/research/ironbark/`, never committed.

1. M. Galletly, W. Giang, S. Mouhaiche, D. Verstraete, "Ironbark: Trajectory and aerodynamic simulator for high-power
   student rockets", Aerospace Science and Technology 173, 111758 (2026), 13 pp.
   <https://doi.org/10.1016/j.ast.2026.111758>. CC BY 4.0 (p. 1): may be cached; restated here, not copied.
2. M. Galletly, W. Giang, S. Mouhaiche, "IronBark: trajectory and aerodynamic simulator for high-power student
   rockets", EUCASS 2023-011, 12 pp. <https://www.eucass.eu/doi/EUCASS2023-011.pdf>. Copyright the first author,
   posted by EUCASS with permission (p. 1): cite only.
3. J. C. Ferris, NASA TN D-4013, 1967. <https://ntrs.nasa.gov/citations/19670020050>. NTRS: public use permitted.
4. C. D. Babb and D. E. Fuller, NASA TN D-4014, 1967. <https://ntrs.nasa.gov/citations/19670020031>. Same terms.
5. C. D. Trescot Jr., G. V. Foster, C. D. Babb, NASA TM X-2774, 1973 (record only).
   <https://ntrs.nasa.gov/citations/19730018268>. NTRS: public use permitted.
6. Public Domain Aeronautical Software, PANAIR page and legal statement. <https://www.pdas.com/panair.html>,
   <https://www.pdas.com/legal.html>.
7. pyPanair, LICENSE.md (MIT) and README. <https://github.com/SaTa999/pyPanair>.
8. The project's [aero page](../physics/aero.md) (the Arcas fixture tables), [apogee-bias](apogee-bias.md) and the
   [fixture report](../../validation/reports/fixture-flights.md).
