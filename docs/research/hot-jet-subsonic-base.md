# Base pressure behind a hot rocket jet below Mach 1

**In short: one hot solid-motor test near HPR boost was found, at Mach 0.9 only; on its body the hot jet's base
pressure matched a cold air jet's of the same thrust coefficient within 0.03 of ambient in four of five firings.** In
the AEDC 4T firings for NASA's plume program (FA 22), aluminized propellant at 5,400 to 6,020 °R left 15° nozzles at
exit Mach 2.4 to 3.2, exit pressure 1.1 to 3.5 times ambient, on a 1.5 in body with an exit about 0.47 of the base.
The near-matched firings gave C_pb −0.17 to −0.27, a base drag of 0.13 to 0.21 on the base area. Both the exit-annulus
and whole-base rules land within about 0.1 of that; HPR Sim's case rule, about 0.02 at minimum diameter, does not. The
numbered copy of MIL-HDBK-762 Fig. 5-150 is Brazzel's RD-TM-70-7 Fig. 2, a cold-air correlation; at Mach 0.9 and
thrust coefficients 4.5 to 7 it reads 0.10 to 0.25 of ambient deeper than three measured sets, so it is not a safe
stand-in. Below Mach 0.9 no hot test was reached. There the cold correlation alone puts the jet's suction at −0.3% to
+5.1% of thrust above the whole-base rule, which would lower apogee; hot data don't bound it. Plot readings carry
their read-off error; "Inference" marks this note's arithmetic. Sources read 2026-10-28.

It follows [power-on base drag](power-on-base-drag.md) (its §3 and §4 rules, its first open question) for #633 and
Cavour's power-on miss ([aero](../physics/aero.md), "Power-on"). C_pb = (p_b − p∞)/q∞ throughout; negative is
suction, and base drag on an area is −C_pb times that area. Thrust coefficient C_T is thrust over q∞ times the base
area (MIL-HDBK-762 eq. 5-28 [2] p. 5-37 uses the reference area, the base for a cylinder; Inference). HPR boost
spans C_T 13 to 50 at Mach 0.3, 1.5 to 5.5 at 0.9 (parent §1); as 1/M², 2.4 to 9.2 at 0.7, 4.7 to 18 at 0.5 (Inference).

## 1. Fig. 5-150 with a numbered axis: Brazzel's cold-air correlation

- **The same figure.** MIL-HDBK-762 Fig. 5-150 cites Ref. 106, Brazzel's RD-TM-70-7, with exit Mach 2.7 and a 20°
  exit half angle [2] p. 5-218. Both cached handbook copies hold the same scan, and its thrust axis carries no
  numbers [2] p. 5-218; Fig. 5-85 has the same gap [2] p. 5-159. RD-TM-70-7 Fig. 2 has the axis: C_T on a log
  scale from 0.01 to 100 [1] p. 8.
- **What it is.** A correlation of cold-air test data: cylindrical afterbody, nozzle flush with the base, design
  exit Mach 2.7, 20° cone, nozzles of several diameters [1] p. 2. It names no data set and no diameter ratio; the
  handbook's copy, Fig. 5-85, is captioned d_j/d_b 0.8 and credits Refs. 45 and 106 [2] p. 5-159. Its
  text says it agrees closely with flight tests of moderate-impulse solid rockets, with no numbers [1] p. 2.
- **Its own reading.** At Mach 1.0, C_T 0.01 to 2 follows the Mach curve, 2 to 15 a line common to all Mach
  numbers, and above 15 a branch per Mach number [1] p. 3. The handbook adds that exit Mach shifts p_b/p∞ by about
  5% per 0.1, lower exit Mach giving higher pressure [2] p. 5-54.
- **How it was read.** A 300 dpi render of the PDF, rotated, with the grid fitted to its heavy lines (the scan is
  skewed about 1.2°); curves read to ±0.01 in p_b/p∞ and ±3% in C_T. In C_pb that is ±0.02 at Mach 0.9, ±0.03 at
  0.7, ±0.06 at 0.5 and ±0.16 at 0.3. Readings and script: `refs/research/hot-jet-subsonic-base/scripts/`.

| Mach | Low C_T (0.3) | HPR C_T | p_b/p∞ across it | C_pb across it |
|--:|--:|--:|--:|--:|
| 0.9 | 0.88 (−0.21) | 1.5–5.5 | 0.80, 0.72 at 3.2, 0.65 at 4.4, 0.70 | −0.35 to −0.62 |
| 0.7 | 0.94 (−0.18) | 2.4–9.2 | 0.91, 0.86 at 6.2, 0.84 at 8.3 | −0.27 to −0.47 |
| 0.5 | 0.98 (−0.09) | 4.7–18 | 0.95, 0.91 at 9 to 10, 0.98 | −0.11 to −0.54 |
| 0.3 | 0.99 (−0.2 to 0) | 13–50 | 0.94, 0.96 at 15, 0.98 at 30, 0.99 | −0.11 to −0.95 |

- **Shape.** Every subsonic curve falls to the common line (Mach 0.9 at C_T 4.4 and 0.65, Mach 0.7 near 8.6 and
  0.84, Mach 0.3 and 0.5 near 10 and 0.91), then climbs back toward ambient or above it. At low Mach a small
  pressure change is a large C_pb, since q∞ is small: the same suction as a share of p∞ grows as 1/M² in C_pb.
- **Its low-thrust end** reads deeper than Cubbage's jet-off values (−0.12 at Mach 0.6, −0.145 at 0.9; parent
  §2.1), so even the starting levels of the curves are not jet-off values.

## 2. Hot solid-propellant firings at Mach 0.9 (AEDC FA 22)

- **The test.** NASA-MSFC's plume program fired 20 solid-propellant motors in the AEDC 4T tunnel at Mach 0.5, 0.9
  and 1.2: 15° conical nozzles of area ratio 4 and 8, propellants with 2% and 16% aluminum, chamber pressure 300 to
  1,350 psia [3] p. 2-10. The hardware matched the strut-held ogive-cylinder of the earlier MA 11F test [3]
  pp. 2-8, 2-10. Base pressure was taken at the outer tap away from the strut [3] p. 2-10.
- **Geometry.** The single-nozzle body is 1.500 in across, 9.0 diameters long [4] p. 47, Fig. 23(a); its exit plane
  is aft of the base by an amount not given [4] PDF p. 49. Table 2-1's exit radii, 0.345 to 0.356 in [3] p. 2-14,
  give d_e/d_b 0.46 to 0.47 and an annulus 78% of the base (Inference, taking FA 22's body to be this one).
- **What was kept.** Table 2-6 holds nine firings, five at Mach 0.9 and four at 1.2 (counted); the Mach 0.5 runs are
  not tabulated [3] PDF pp. 71–74. Chamber pressure may read low on some runs, from propellant fouling its tubing [3]
  p. 2-10. Table 2-2's tolerances [3] p. 2-15 print no decimal point; read as 0.020 psi static and 0.025 psi base,
  they come to ±0.004 to ±0.015 of p∞ per run (Inference).
- **The Mach 0.9 points** [3] Table 2-6 (exit state as Lockheed computed it; C_T, C_pb and drag are Inference):

| Run | Al | p_e/p∞ | Exit Mach | C_T | p_b/p∞ | C_pb | Drag on base area |
|:--|--:|--:|--:|--:|--:|--:|--:|
| 4/6 | 2% | 1.06 | 3.20 | 5.4 | 0.847 | −0.271 | 0.210 |
| 13/5 | 16% | 1.55 | 2.92 | 6.6 | 0.857 | −0.251 | 0.196 |
| 14/7 | 16% | 1.61 | 2.39 | 4.5 | 0.904 | −0.170 | 0.134 |
| 5/5 | 2% | 2.44 | 3.18 | 12.5 | 0.918 | −0.146 | 0.114 |
| 15/6 | 16% | 3.51 | 2.93 | 16.1 | 0.980 | −0.035 | 0.027 |

- **Chamber temperature** was 5,400 to 6,020 °R [3] Table 2-6, hotter than the propane-oxygen rocket behind the
  supersonic +0.08 shift (core near 4,200 °R; parent §2.2).
- **Sign.** The near-matched jets (p_e/p∞ 1.06 to 1.61, the HPR range) give the deepest suction; underexpanded ones
  (2.4 to 3.5) lift base pressure toward ambient. The lower exit Mach of run 14/7 reads 0.05 higher than 13/5, the
  handbook's sign but far less than its 5% per 0.1 (Inference).
- **Mach 1.2** [3] Table 2-6: p_b/p∞ 0.626 to 0.782, C_pb −0.22 to −0.37, at C_T 2.6 to 4.2 (Inference). The three
  near-matched runs give 0.25 to 0.29 on the base area, above eq. 3.94's 0.208 and Cubbage's jet-off 0.22 at Mach
  1.1: the transonic gap of parent §4, now with a hot jet (Inference).

## 3. Hot against cold on the same body

- **The cold runs.** MA 10F flew air and CF4 through nozzles of the same exit size on the same body class [4]
  Fig. 23(a), exit-to-base ratio held fixed, at Mach 0.9, 1.2, 1.46 and 3.48 [3] pp. 2-5 to 2-6. Table 2-3 lists
  7 Mach 0.9 runs for air nozzle 2 (area ratio 3.6, 25° wall) and 25 for air nozzle 4X (6.5, 35° wall) (counted)
  [3] pp. 2-17 to 2-21.
- **Lockheed's verdict.** On its momentum-based similarity parameter, hot and cold Mach 0.9 data fall in one band,
  the hot points slightly left, which it blames on the low chamber readings [3] p. 4-5, Fig. 4-5 p. 4-16.
- **At matched C_T** (Inference, `scripts/compare.py`): run 13/5 is +0.011 in p_b/p∞ above cold run 356 (C_T 6.5);
  5/5 is −0.02 below cold runs 354 and 357; 15/6 is −0.005 off cold runs interpolated to C_T 16; 4/6 is −0.006 off
  nozzle 4X interpolated to C_T 5.4. In C_pb that is −0.05 to +0.02. Run 14/7 sits +0.08 above 4X at C_T 4.5, but
  its exit Mach is 2.4 against 4X's 3.8.
- **Bias.** If the hot chamber pressures read low, their true C_T is higher and the matching cold value higher too,
  which moves the hot points further below cold (Inference). Wall angle cuts the other way: the cold nozzles' 25°
  to 35° walls raise base pressure against the hot 15° (parent §2.3, Gillespie). Neither is sized.
- **So,** at Mach 0.9 the hot shift is −0.05 to +0.02 in C_pb, not the +0.08 seen at Mach 1.91 (parent §2.2).

## 4. The cold correlation against measured sets at Mach 0.9 (Inference)

Each line is measured p_b/p∞ minus Brazzel's Fig. 2 at the same C_T (positive: the data sit higher, less suction).

- **Cubbage, sonic jets** at total-pressure ratio 2, d_j/d_b 0.25 to 0.75, C_T 0.17 to 1.5: +0.016 to +0.028.
- **Cubbage, matched Mach 2 jet,** d_j/d_b 0.75, C_T 5.6: +0.13.
- **MA 10F cold** at C_T 4.5 to 11: +0.06 to +0.16; at C_T 11.7 to 22: −0.02 to −0.15. Nozzle 4X's overexpanded
  runs at C_T 2.1 to 2.3 agree within 0.011.
- **FA 22 hot** at C_T 4.5 to 6.6: +0.11 to +0.25; at 12.5 and 16.1: −0.07 and −0.14. At Mach 1.2 about +0.10 to
  +0.18, off Fig. 2's common line (not in the cached readings file).
- **Reading.** Fig. 2 agrees at low C_T but dips deepest at C_T 4 to 7, then climbs fastest. At C_T 5 to 7 it gives
  0.68 to 0.76 (C_pb −0.56 to −0.42), the measured sets 0.84 to 0.89 (−0.28 to −0.19). Geometry (exit size,
  nozzle overhang, wall angle, sting or strut) matters as much as the jet. The correlation is a bound, not a value.

## 5. Does the whole-base rule hold? (Inference)

The FA 22 body has no jet-off run, so the power-off value is borrowed: Cubbage's sting-free 0.145 at Mach 0.9, or
eq. 3.94's 0.225, which HPR Sim uses and which reads high power-off ([cylinder base drag](cylinder-base-drag.md)).
Signed error is rule minus measured, on the base area, for the near-matched runs 4/6, 13/5 and 14/7 (0.13 to 0.21).

| Rule at d_e/d_b 0.47 | Power-off 0.145 | Power-off 0.225 |
|:--|--:|--:|
| Whole base | −0.065 to +0.011 | +0.015 to +0.091 |
| Exit annulus (78% of base) | −0.097 to −0.021 | −0.034 to +0.042 |
| HPR Sim's case rule, minimum diameter (about 10%) | −0.196 to −0.120 | −0.188 to −0.112 |

- **Mach 0.9:** both the whole-base and exit-annulus rules sit within about 0.1 of the hot data; the case rule reads
  low by 0.11 to 0.20. The hot data can't pick between the first two at this exit size.
- **As a share of thrust,** extra base force beyond the whole-base rule is (−C_pb(1 − r²) − C_b,off)/C_T, r the
  exit-to-base ratio. For run 13/5 it is −0.4% against eq. 3.94: the whole base is about right at Mach 0.9.
- **Elsewhere** only Fig. 2 speaks, drawn for d_j/d_b 0.8 in the handbook and applied here at r = 0.4. It gives +3.9%
  to +6.7% of thrust at Mach 0.9 (C_T 1.5 to 5.5), +1.9% to +2.8% at 0.7, −0.3% to +3.0% at 0.5, and −0.1% to +5.1% at
  0.3 (+2.6% at C_T 15). It reads 0.1 to 0.25 too deep at Mach 0.9 against the hot data, so these are upper ends.
  Reading error alone at Mach 0.3, C_T 15, is ±0.9% of thrust.
- **Sign.** If Fig. 2 holds, extra suction under power makes the true apogee lower than any of the rules predicts:
  a flattering simulation for an altitude goal, the safe side for a waiver ceiling (parent §6).

## 6. Other leads

- **Space Shuttle flights.** Base pressures were much higher on every element than ground tests predicted [5] p. 1,
  over STS-1 to 5 [5] p. 4; SRB plume-on flight bands sit well above plume-off model values, Mach 0.8 to 1.25 [5]
  Fig. 7, p. 12: a hot, full-scale, multibody sign, not a single-body size.
- **Set aside:** MA 11F's firings, no valid transonic data [4] summary, [3] p. 2-9; FA 23's triple body (Mach 0.5
  to 1.3, 28 firings), unknown nozzle states [3] pp. 2-12, 2-13.
- **Craven 1960** is a cold supersonic jet at d_j/d_b 0.1875 in subsonic flow (catalog abstract [6]); the PDF could not
  be fetched on 2026-10-28 (403, then 404) and its archived copy is an HTML page, so only the abstract was read.
- **Moore and Hymer's patent** extends Brazzel's method below Mach 1.5 with data down to Mach 0.9, and holds its jet
  temperature term at no less than 0.7 below Mach 1.2 (eq. 108b) [7]. It states no subsonic error.

## What the project can use

- **Terms.** RD-TM-70-7 is approved for public release, distribution unlimited (its DTIC cover page) [1]. The three
  NASA reports are on NTRS as public, government use permitted [3, 4, 5]. All four are cached in
  `refs/research/hot-jet-subsonic-base/` with the Fig. 2 readings and the hot-cold script; no table or figure copied.
- **For #633:** the FA 22 rows give a hot-jet test for the rule at Mach 0.9 and 1.2 (d_e/d_b 0.47). A test pinning
  base drag under power to 0.13 to 0.21 on the base area at Mach 0.9 (near-matched) would fail the case rule.
- **For Cavour:** RASAero's power-on table equals power-off, the whole-base rule; the Mach 0.9 hot data side with a
  base drag near the whole base, so the −8.3% no-relief miss is the better-founded one (Inference).

## What this suggests

1. Choose rule (a) or (b) of #633 on the cold-jet grounds already in the parent note; the hot data at Mach 0.9 agree
   with either within about 0.1, and cut the case rule's 0.11 to 0.20 low reading out.
2. Keep the subsonic hot-jet effect at Mach 0.9 at the cold value: the hot shift is −0.05 to +0.02 in C_pb there.
3. Don't take Fig. 2 as the model: at Mach 0.9 it is 0.1 to 0.25 of ambient off measured bodies.
4. Record the low-subsonic jet suction (Mach 0.3 to 0.7) as an open, one-sided gap: up to about 5% of thrust more
   base drag than the whole-base rule in Fig. 2, which would lower apogee.

## Open questions

- No hot test below Mach 0.9 was read. FA 22 fired at Mach 0.5; its data report (Baker, AEDC-DR-75-79) wasn't found.
- What data Fig. 2's subsonic curves rest on: RD-TM-70-7 names none, and the handbook credits a 1982 report (Ref. 45,
  Henderson RD-CR-82-25), not found, for subsonic and transonic data [2] p. 5-54. Brazzel and Henderson's AGARD CP-10
  paper was not reached: DTIC served a maintenance page, then 403, on 2026-10-28.
- FA 22's body had no jet-off run and its nozzle exit sits aft of the base, so the whole-base and annulus rules
  can't be told apart from it. Particle momentum in the two-phase jets may be missing from the computed C_T
  (unbounded).
- The exit-Mach effect: the handbook's 5% per 0.1 against FA 22's much smaller shift is one pair of runs. Lockheed
  flags one 16% Al point of FA 22 as hit by tubing contamination [3] Fig. 4-5, p. 4-16, without naming the run; if
  it is 14/7, that run's +0.08 against cold and the exit-Mach pair are both in doubt.

## Sources

NASA and Army reports: public release; read 2026-10-28.

1. C. E. Brazzel, *Estimated Power-On Base Drag for a Rocket-Assisted Projectile*, US Army Missile Command
   RD-TM-70-7, 1970 (DTIC AD0874580), pp. 2, 3, 8. Archived copy of the DTIC PDF:
   <http://web.archive.org/web/20250202084050/https://apps.dtic.mil/sti/tr/pdf/AD0874580.pdf>
2. MIL-HDBK-762(MI), 1990, pp. 5-37, 5-53, 5-54, 5-159, 5-218.
   <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets> (read 2026-10-17 and 10-28)
3. P. R. Sulyma, M. M. Penny, *Development of Base Pressure Similarity Parameters for Application to Space Shuttle
   Launch Vehicle Power-On Aerodynamic Testing*, LMSC-HREC TR D568366 (NASA CR-150826), 1978.
   <https://ntrs.nasa.gov/citations/19780025191>
4. L. M. Hair, *Space Shuttle Plume Simulation Effect on Aerodynamics*, REMTECH RTR 016-5 (NASA CR-150848), 1978,
   p. 47. <https://ntrs.nasa.gov/citations/19790002932>
5. A. L. Frost, C. C. Dill, NASA TP 2569, 1986, pp. 1, 4, 12. <https://ntrs.nasa.gov/citations/19860012036>
6. A. H. Craven, College of Aeronautics Report 129, 1960, catalog abstract.
   <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4803>
7. F. G. Moore, T. C. Hymer, US 6,721,682 B1, 2004. <https://patents.google.com/patent/US6721682B1/en>
8. J. M. Cubbage, Jr., NACA RM L56C21, 1956, Figs. 10(b), 11(c), 13, as read in the parent note.
   <https://ntrs.nasa.gov/citations/19930089150>
