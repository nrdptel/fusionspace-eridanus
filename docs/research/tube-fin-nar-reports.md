# What the NAR R&D tube-fin reports measured

HPR Sim's tube-fin center of pressure (CP) sits 0.42 to 3.0 calibers forward of OpenRocket's up to Mach 0.5, and its
tube-fin drag probably reads low ([#228](https://github.com/nrdptel/fusionspace-eridanus/issues/228),
[#234](https://github.com/nrdptel/fusionspace-eridanus/issues/234)). [Tube-fin data](tube-fin-data.md) named five NAR
R&D reports, behind a member sign-in, as the last lead; NAR's index holds a sixth, a tunnel test of tube-fin drag.
This note reads all six: what each measured, and whether any case rebuilds in both codes to say whose CP or drag is
nearer. "Inference" marks this note's own arithmetic; the scripts are kept privately with the scans.

**In short: one report measures a tube-fin CP, two measure tube-fin drag, and none settles the CP.**

- **CP: a tie.** Avecilla (1997) found the CP of four small models by moving a pivot in a blower jet. Each code,
  applied by hand to his models, misses the measured shift from flat fins to tubes by the same RMS, 0.28 against
  0.29 in. One model (three tubes) leans to OpenRocket, but only under two unstated details of the geometry.
- **Drag: both codes low, HPR Sim lower.** Staab (2012) flew a six-tube rocket close to OpenRocket's *Tube fin rocket*
  23 times on C6-5 motors. Its apogees imply a whole-rocket C_D near 2.5 (1.9 to 2.9 with every listed error at once),
  against HPR Sim's 1.654 and OpenRocket's 1.78 on the *Tube fin rocket*: #228's direction, larger. VanMilligan's
  tunnel test (2013) is too noisy to confirm or refute it.
- **Ring lift on a body: likely lower than isolated.** Rangitsch (2002) computed one ring at the tail by CFD, checked
  against nothing. Its CPs are unusable, but its normal forces, on an assumed density, put the ring at 0.73 and 0.84
  of HPR Sim's isolated-ring slope, near the slender-body factor 1 − (a/R)².

Reynolds (1999) and Calvert (2008) carry no usable number. Confidence: high on what each report did; medium on Staab's
C_D (one rocket, five baseline flights); low on the CP lean and the CFD's forces.

## 1. Avecilla, "Stability of Tube Fins" (NARAM-39, 1997): a measured CP

**Method** (p. 1). A blower feeds a 4.5 in straightening section; the exit air runs 10 to 20 ft/s. Each model hangs on
low-friction caliper pivots in the jet, is pushed to 15° and released, and is judged stable, neutral or unstable.
The pivot moves along the body until the model turns neutral: the CP is bracketed by pivot stations, read to 0.1 in.

**Geometry** (pp. 1 to 3). Four BT-20 models (0.736 in across, by the maker's catalog) share one 2.8 in ogive nose.
A has three flat clipped-delta fins (root 1.5, tip 0.75, span 1.5 in); B six tubes 1.5 in long; C three tubes 1.5 in
long; D six tubes 0.75 in long (p. 2 table). **Not stated:** body length, where the tubes sit along the body, the flat
fins' sweep, mass and CG. The photo puts A near 10 in long (Inference, ±15%).

**Measured** (p. 2 table and chart; inches from the nose tip; runs in model order, three times):

| run | A (flat) | B (6 × 1.5 in) | C (3 × 1.5 in) | D (6 × 0.75 in) |
|---|--:|--:|--:|--:|
| 1st / 2nd / 3rd | 8.9 / 8.7 / 8.1 | 8.3 / 8.5 / 8.1 | 8.3 / 8.1 / 7.7 | 8.7 / 8.7 / 8.3 |
| average as printed | 8.5 | 8.3 | 8 | 8.5 |

The author states ±0.2 in, but runs spread up to 0.8 in, and every model reads lowest on run 3: a drift of the rig
that a within-run difference cancels (Inference). Tubes minus flat fins: B −0.27 in, C −0.53 in (95% interval
−0.82 to −0.25, t at 2 df), D 0.00 in (−0.50 to +0.50). He notes that B's long tubes fouled the caliper, so its CP
may sit 0.2 to 0.4 in further aft (p. 3). He ranks stability A, D, B, C, and blames tube area out of the stream and
interference between nearby surfaces (p. 3).

**Rebuilt by hand in both codes (Inference).** Barrowman's ogive (slope 2 at 1.305 in) and flat fins swept 0.75 in
put A's CP at its measured 8.5 in with the fin root at 8.53 in and the tail at 10.03 in, which matches the photo.
Tubes flush with that tail, Mach 0:

- HPR Sim ([aero](../physics/aero.md#tube-fins)): Weissinger on the 0.723 in mean diameter, Fletcher's CP line.
  One 1.5 in tube: 3.74 per radian on the body's area, CP 0.155 in aft of its leading edge.
- OpenRocket ([tube-fin data](tube-fin-data.md) section 3): the Ribner formula on the 0.710 in bore, CP at a quarter
  length: 5.72 per radian at 0.375 in, 1.53 times HPR Sim's.

| model | measured shift (in) | HPR Sim | OpenRocket |
|---|--:|--:|--:|
| B | −0.27 | −0.42 | −0.02 |
| C | −0.53 | −0.93 | −0.39 |
| D | 0.00 | +0.22 | +0.42 |
| RMS miss | | 0.28 | 0.29 |

Moving A's measured CP anywhere from 8.3 to 8.7 in changes the shifts by under 0.01 in. The tube station and the flat
fins' sweep matter. HPR Sim's −0.93 on C falls 0.11 in outside C's interval and OpenRocket's falls inside, but only
with the tubes flush with, ahead of, or up to about 0.13 in aft of the flat fins' tail, and those fins swept 0.75 in.
Move the tubes 0.15 in aft, or halve the sweep, and HPR Sim fits as well or better (RMS 0.26 against 0.42 to 0.43).
On C, two thirds of the 0.54 in gap between codes is the slope, one third the CP rule. Both codes order B and C as
measured but put D aft of A, where the measurement gives 0.00 ± 0.50 in and the author ranks A first.

**How far to trust it.** A direct bracket, but three runs a model on one rig. The Reynolds number on a 1.5 in tube is
7,700 to 15,500, 16 to 33 times below a 100 m/s flight (Inference): thick laminar layers should cut a small tube's lift
and move the CP forward, by an unbounded amount. Body lift at the 15° release moves the predicted shifts by −0.16 to
+0.21 in. On C, HPR Sim's CP is 0.40 in (0.54 calibers) forward of the measurement, the cautious side.

## 2. Staab, "Adjustable Drag Tube Fin Rocket" (NARAM-54, 2012): a tube-fin rocket's drag

**Method** (pp. 4 to 6). Flights on Estes C6-5 motors from a 4 ft rod, apogee from a Perfect Flite pNut barometric
altimeter, run order randomized. Drag inserts (a 2 in BT-50 sleeve holding a wood blade) go into some tubes' aft
ends; ballast keeps each set at one gross mass. 23 flights in two sets (p. 5).

**Geometry** (pp. 3, 4, 12). BT-50 body (0.976 in), 3 in nose, 20.75 in long by its RockSim printout. Six 3 in BT-51
tubes around a central BT-51 tube, their aft ends 0.5 in ahead of the motor mount's. Wall, finish and reference area
not stated. OpenRocket's *Tube fin rocket* has the same count, length and near size: six 3.0 in tubes 24.79 mm across
on a BT-50 body, but a 4.72 in nose and an 18 in body. Staab's shorter nose and body would lower the codes' C_D a
little, widening the gap (Inference).

**Measured** (p. 5). Set 1, 113 ± 1 g gross, clear, 78 to 82 °F: no inserts 430.3 ft (three flights, SD 4.16), two
inserts 337.0, four 313.3. Set 2, a BT-51 oversleeve on the lower body, 125 ± 1 g, overcast, 84 to 86 °F: no inserts
390.5 ft (two flights), down to 326.0 ft with four thin blades. The author's RockSim 9 matched at C_D 2.0 to 2.2 (p. 6).

**The C_D the apogees imply (Inference).** A vertical one-dimensional flight on NAR's C6 curve (8.82 N·s, 10.8 g
propellant), the stated gross mass at liftoff (p. 5), air density 1.13 kg/m³, area 4.827 cm², fitted to each mean:

| source | whole-rocket C_D | tubes' share |
|---|--:|--:|
| HPR Sim, *Tube fin rocket*, Mach 0.2 ([aero](../physics/aero.md#tube-fins)) | 1.654 | 1.05 |
| OpenRocket 24.12, same rocket, Mach 0.1 to 0.2 | 1.78 | 1.18 |
| Staab's rocket, set 1 / set 2, peak Mach 0.13 to 0.15 | 2.52 / 2.54 | about 1.9 (1.3 to 2.3) |

- The share is the fit less HPR Sim's 0.60 for nose, body and lug; the two codes agree on those within 0.005.
- Each cause alone: impulse ±1σ moves the fit 2.37 to 2.68 (set 1), density 1.10 to 1.16 kg/m³ 2.59 to 2.46, mass
  ±1 g 2.57 to 2.48. If the pNut assumes the standard atmosphere's temperature (not checked), the true apogee is 4%
  to 6% higher and the fit falls to 2.2. All at once: 1.98 to 2.79 (set 1), 1.91 to 2.87 (set 2).
- The two days agree within 0.02; read as mass without the motor they disagree (1.18 and 0.88).
- At 1.654 the same flight model reaches 506 ft against 430 (+17.5%) and 451 against 391 (+15.4%); at 1.78, +14.3%
  and +12.7%. Weathercocking (lowers the fit) and the motor lot are unbounded.

**Drag inside a tube (Inference, set 2).** Two bare sleeves add 0.33 ± 0.08 (1σ) to C_D: their front face, 0.053 in²
per tube, costs about 2.3 on its own area, against the 1.0 or so HPR Sim charges a free square edge. A 1/32 in blade
costs about 6.5 on its face (0.08 of it friction). A rim inside a tube may hold some missing drag, beside the
junctions and channels [tube-fin data](tube-fin-data.md) section 5 names.

**Stability.** None measured; RockSim's 2.27-caliber margin (p. 12) is a third code's reading, not data.

## 3. VanMilligan, "Drag on Tube Fins" (NARAM-55, 2013): a tunnel test

**Method** (pp. 6 to 13). A youth entry run in the Air Force Academy's 1 ft by 1 ft low-speed tunnel at about 39 mph
(17.6 m/s), drag from a balance calibrated with weights, 20 readings averaged per point. **Geometry** (pp. 2, 9, 18).
Three tubes on a 24 mm body (a PNC-24A nose); BT-20 and BT-50 tubes cut from 6 in down to 0.5 in, BT-60 and BT-70
tubes 2 in long, three 2 by 2 in flat fins, a finless base and the stand alone. Body length not stated.

**Measured** (p. 14). The finless base read 6.958 g at 39.3 mph but 2.150 g at 31.8 mph, 3.2 times more for 1.53
times the dynamic pressure; the author calls the first too high (p. 17), and five tube increments come out negative.

**What it implies (Inference, density 0.92 kg/m³ assumed for the lab's elevation).** For three 3 in BT-50 tubes, the
nearest case to the *Tube fin rocket*, the added C_D on the body's area is 0.35 against the author's base, or 0.88
against the slower base scaled to 39 mph. HPR Sim's share for three such tubes is about 0.53, half its six-tube 1.05.
The same two bases give the flat fins 0.09 and 0.62, either side of the 0.2 or so friction alone suggests. The
bracket holds both codes and Staab's figure: the test can't separate them.

## 4. Rangitsch, "Simulation of Ring Finned Model Rocket Stability Using CFD" (NARAM-44, 2002)

**Method** (pp. 3 and 4). CFX 4.4 on just over 500,000 cells, periodic sides, 13.5 m/s, 1.25° to 10°; no turbulence
model, grid study, density or check against a measurement stated. **Geometry.** A BT-20 body, 0.254 m of tube and a
2:1 elliptical nose; one BT-55 or BT-60 ring, 0.0254 or 0.0127 m long, flush with the tail, no supports.

**The CP numbers fail.** In both tabled cases (the 0.0254 m rings), at every angle, the ring's lift acts 0.198 to
0.199 m from the reference (Tables 1 and 2), which Figs. 8 to 10 place 0.068 m (2.7 chords) ahead of the ring, where
no lifting ring acts (Fletcher's: 0.14 to 0.36 chord aft of the edge). The report puts the reference at the nose tip
(p. 4), which its ring torques contradict; with the forces at sensible stations the rocket's CP moves about 0.10 m
aft (Inference).

**The normal forces may be usable (Inference).** With air at 1.205 kg/m³ (assumed; each 1% of density is 1% here), the
long ring's slope on the body's area is 8.79 per radian (BT-55) and 13.65 (BT-60): 0.73 and 0.84 of Weissinger's
isolated ring, 0.58 and 0.70 of OpenRocket's ring formula, and within 1% and 6% of Weissinger times 1 − (a/R)². That
is lift lost to the body, as in the Army data, but less. If it holds, HPR Sim's CP for these rockets would move 0.24
to 0.60 calibers forward: an isolated-ring model flatters the margin, OpenRocket's more. Neither code flies this
layout: HPR Sim's tubes sit beside the body, not around it, and OpenRocket has no ring-tail component. The two codes'
ring CPs differ by only 0.01 chord here: #228's CP gap is a long-tube matter, which this report doesn't test.

## 5. The two reports with no usable number

- **Reynolds (NARAM-41, 1999)** dropped 1 oz Alpha-based models 17 ft through photogates 12.5 in apart (p. 2), with no
  geometry. 37 of 60 gate speeds (13.5 to 122.6 m/s) are impossible in a 5 m drop, and 9 of 12 plausible pairs gain
  more than gravity adds: noise 10 to 30 times the drag signal (Inference).
- **Calvert (NARAM-50, 2008)** flew a 34 in backslider glider 22 times, 15 of them timed glides; 14 flew one BT-55
  coupler tube in place of one of three fins (pp. 2 to 5). No CP, margin or drag; the tube's length is unstated.

## What this suggests

- **Fly Staab's rocket in both codes** as #228's first flight reference: six 3 in tubes on BT-50, 20.75 in, 3 in nose,
  113 g and 125 g gross on a C6-5, against 430.3 and 390.5 ft. The hand fit expects HPR Sim 15% to 18% high and
  OpenRocket 13% to 14% high. One rocket and one motor: a sample, not a bound. State the pNut's temperature handling,
  the site's elevation and the tube wall.
- **Don't cut the tube-wall edge term** that tube-fin data section 5 questions: this rocket wants more tube drag, about
  0.9 more of C_D (0.3 to 1.2), not less. A rim inside a tube costing twice a free edge is a lead to test.
- **Keep "the smaller margin is more cautious", as a direction, not a bound.** On Avecilla's geometry as rebuilt, each
  measured CP sits between the codes or forward of both; with the tubes 0.30 in further forward, or B's caliper
  correction applied, a measurement lands aft of OpenRocket's. Rangitsch's CFD, if its forces hold, puts the truth
  forward of both. HPR Sim's own CP is aft of the measurement on Avecilla's D (0.22 in, inside the noise).
- **A body-interference factor** of 1 − (a/R)² on a ring around a body has two sources in sign, the Army
  ring tails and this CFD, for #234; neither covers tubes beside a body.

## Open questions

- Does the pNut correct for temperature (about 0.3 in Staab's C_D)? His tube wall, site elevation? His cited blog
  C_D of 1.69 (rocketdungeon.blogspot.com, 2008) asks for a sign-in; the Wayback Machine has no copy (2026-11-03).
- Avecilla's body length, tube station and fin sweep: no write-up beyond the report was found. How much does a
  Reynolds number near 10⁴ cut a short tube's lift? Rangitsch's density and moment reference? VanMilligan's stand?

## Sources

All six reports are NAR members' copyrighted work behind a sign-in: cited and paraphrased, numbers restated, nothing
copied; the scans stay in the private cache. Years and NARAM numbers for Avecilla, Reynolds and Calvert are from the
index (7). Read 2026-11-03.

1. S. Avecilla, "Stability of Tube Fins", NARAM-39 R&D, 1997, <https://www.nar.org/docs.ashx?id=1357957>, pp. 1 to 3.
2. G. Reynolds, "Drag Analyses Tubular Fin Drop Test", NARAM-41 R&D, 1999, <https://www.nar.org/docs.ashx?id=1357981>.
3. M. Rangitsch, "Simulation of Ring Finned Model Rocket Stability Using Computational Fluid Dynamics", NARAM-44 R&D,
   2002, <https://www.nar.org/docs.ashx?id=1358056>, pp. 3 to 5, Tables 1 and 2, Figs. 6 to 10.
4. J. G. Calvert, NARAM-50 R&D, 2008, <https://www.nar.org/docs.ashx?id=1358168>, pp. 2 to 8, Tables 2 to 4.
5. J. Staab, "Adjustable Drag Tube Fin Rocket", NARAM-54 R&D, 2012, <https://www.nar.org/docs.ashx?id=1358248>.
6. A. VanMilligan, "Drag on Tube Fins", NARAM-55 R&D, 2013, <https://www.nar.org/docs.ashx?id=1358266>, pp. 2 to 17.
7. NAR, R&D Reports index, <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>.
8. Estes C6 curve from NAR data (2000), via the sunset MIT project fusionspace-loft; NAR's C6 sheet (1995).
9. [Tube fins](../physics/aero.md#tube-fins), [tube-fin data](tube-fin-data.md); OpenRocket 24.12's *Tube fin rocket*
   and its drag in `validation/fixtures/ork/openrocket-drag-curves.json`.
