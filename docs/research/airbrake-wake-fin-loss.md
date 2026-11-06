# Airbrake wake over the fins: how much fin normal force a deployed brake removes

What this covers: how much normal force (and so static margin) a rocket's fins lose when a deployed drag brake sits
ahead of them or beside them, against the brake's height, its distance ahead of the fins, its position around the body
and Mach number. It is for issue #867, M6.4's deployed-brake stability margin, a safety output whose flattering side is
leaving the loss out. How far to trust it: **no tunnel or flight test was found that measures fin or tail normal-force
slope behind a brake on a body.** What is measured is the wake itself (velocity behind plates and one body brake) and
whole-aircraft stability with fuselage brakes out, the tail out of line with the brake or ahead of it. The rocket
numbers are two student CFD studies: one sizes a large loss, the other gives no net figure. Builds on
[airbrake-drag.md](airbrake-drag.md) §6 and [airbrake-installed-drag.md](airbrake-installed-drag.md). Researched
2026-11-05.

**In short.** In the one rocket study with numbers (WVU, steady CFD at 1.43° incidence), four brakes **between the
fins**, level with 40% of the fin root chord and 45° off the fins, cut the static margin from 3.37 to 0.25 calibers at
Mach 0.2, and left it below the brakeless rocket's at every Mach to 1.2. Read through a two-part lift model this is a
loss of about **52% of the fins' (and aft body's) normal-force slope at Mach 0.2, falling to 17% at Mach 0.8**
(*inference*); Mach 0.2 is the slowest case run and the loss was still growing as speed fell. Measured wakes say why: 2
body diameters (about 5 brake heights) behind a pair of brakes on a body, the wake core kept 0.49 or less of free-stream
dynamic pressure; behind fences on a wall, flow reattaches at 10 to 12.5 heights and pressure recovers by about 35.
**Rule of thumb** (*inference*): expect a large loss when any fin lies beside the brake or within about 2 body diameters
(5 brake heights) behind it, a residual loss out to about 30 brake heights, and treat "negligible" as unbounded: WVU's
folding brakes about 13 brake heights (6 D) ahead of the fins still cost up to 1.14 calibers at Mach 0.2 to 0.8 and 2.81
at Mach 1.0 half open. The two numbers that matter most for #867 are the 52% loss at Mach 0.2 and that 6 D station's
−1.14 to −2.81 calibers.

## Conventions

- **h**: the brake's radial reach out of the skin; **D**: body diameter, one caliber.
- **q_w/q**: dynamic pressure in the wake over free stream, taken here as (u/U)², which ignores the wake's static
  pressure (*inference* wherever used). A fin's normal force scales with the q it sees, so q_w/q is the fraction kept.
- **Static margin**: center of pressure (CP) minus center of gravity (CG), over D.

## 1. The rocket study: Farha, WVU MS thesis (2025), CFD only

- **Setup.** Ansys Fluent steady RANS (k-ω SST) on a half-model with a symmetry plane: a generic 4 m rocket, D = 157
  mm, a 75 cm von Kármán nose and four trapezoidal fins (root 250 mm, tip 150 mm, span 125 mm) whose trailing edge
  ends at the base (p. 10). Each brake set has four brakes 45° off the fins. Three sets (EB extruding, FB folding, RB
  rotating) sit 280 cm from the nose tip; the fourth (AB) sits "between the fin set" at 385 cm (p. 13). Mach 0.2 to
  1.2 by 0.2, half and full deployment (p. 24); CG at 2.62 m (p. 25). Angle of attack comes from a crosswind of 2.5% of
  the axial speed, 1.43° (p. 21). Margin is (X_CP − X_CG)/D (eq. 3.9, p. 24). Each CP is one steady solve at a
  small angle with convergence watched on drag; its noise floor is not stated.
- **Settled: between, not ahead.** The fin root's leading edge is at 375 cm (4 m less 250 mm), so the AB brakes sit
  10 cm (0.64 D) behind it, with 15 cm (about 1 D) of fin behind them (*inference* from pp. 10 and 13). Issue #867's
  "between the fins" is right; "just ahead" in the queue entry is not.
- **Table 3.3 (p. 26), margin in calibers.** Base 3.37, 3.44, 3.56, 3.76, 4.63, 4.04 at Mach 0.2 to 1.2. AB100: 0.25,
  1.16, 2.34, 3.12, 3.69, 3.70, below the base at every Mach. AB50 at Mach 0.2: 2.46. The forward sets sit about 6 D
  (95 cm) ahead of the fin root: the extruding brake reaches 39.25 mm (24 h, p. 14), the 75 mm folding brake turned
  80° about 74 mm (about 13 h; about 20 h half open, *inference*). EB100 3.36 to 4.19, FB100 2.89 to 4.36, RB100 3.32
  to 4.45; FB50 2.49 at Mach 0.2, **1.82 at Mach 1.0** (−2.81 against the base) and 2.23 at 1.2.
- **The author's reading (pp. 38 to 39).** A low-pressure wake envelops the lower half of the fins and moves the CP
  forward; it narrows as speed rises. The author calls the AB set a flawed contingency design.
- **Implied fin loss (*inference*).** Take the nose lift slope as 2 at 37.5 cm (half the nose length) and the fins'
  at 382 cm (their quarter mean chord, from the p. 10 geometry), and solve each margin for the fins' share. The base
  rocket gives a fin slope of 8.2 to 12.5 per radian. AB100 then removes **52%, 43%, 28%, 17%, 27% and 10%** of it at
  Mach 0.2 to 1.2; AB50 removes 22% at Mach 0.2. Moving the nose point 20 to 60 cm changes these by under 0.01, the fin
  point ±7 cm by ±0.02 (±0.03 at Mach 1.0). The OpenRocket CP the thesis quotes (301 cm at Mach 0.3, p. 12) implies a
  fin slope near 6.5, so the CFD's extra aft lift (aft body, base) is lumped into the "fin" share here. The model
  gives the AB brakes no normal force; any they carry acts behind the CP and would make the true loss larger.
- **Forward sets mix two causes.** A brake between the CG (262 cm) and the CP (315 cm) moves the CP toward itself
  through its own normal force, with or without a wake. FB50 lost more than FB100 at every Mach (2.49 against 2.89 at
  0.2; 1.82 against 3.58 at 1.0), as a half-open flap that lifts would (*inference*); the table cannot separate the
  two. The FB50 Mach 1.0 case met the convergence criteria (p. 25) and is not discussed in the pages read.

## 2. The other rocket CFD: Monash test rocket (2019)

- Three flaps between three fins, hinged forward, on a 65 mm body. A CFD run at 126 m/s (about Mach 0.37) and 10°
  angle of attack gave **larger** forces on both faces of almost every fin with the flaps out, put down to faster flow
  off the flap tips (up to 180 m/s) striking the fins; the authors conclude the flaps have little effect on stability
  (pp. 22 to 23). The force table is an image and gives no net normal force or CP, so it neither confirms nor
  contradicts WVU's sign.

## 3. Measured wakes behind plates and brakes

- **A pair of brakes on a body (ARC C.P. 251, Fail, Owen and Eyre 1956).** A 7 in body with a fin and no tailplane,
  two sheet-metal cascade brakes 3.54 in square on opposite sides at 50°, 0.5 in off the skin (fig. 18). Hot-wire
  traverse 15 in behind the hinges, which the report says is roughly the full-size aircraft's tail plane station
  (p. 11); the body is 5.16 in across there (fig. 18). The innermost contour drawn is **u/U = 0.6, so q_w/q ≤ 0.36**
  in a small core; the 0.9 contour spans about one to one and a half brake widths (fig. 21). The hot wire read up to
  about 0.1 U below the pitot-static tubes behind a plate (p. 6), so the core's q lies from below 0.36 to about 0.49
  (*inference*). The plane is 2.1 maximum diameters, 4.2 brake widths and about 4.7 brake heights (3.2 in reach, gap
  included) downstream (*inference*). Cascades split the wake into two cores (p. 12); a solid plate's wake is likely
  deeper (*inference*). Low speed only.
- **A free square plate (same report).** A 25 in² square plate at 90° in a 4 ft × 3 ft tunnel at 140 ft/s: the
  reversed-flow bubble ends near 15 in (3 widths) behind it, where the centerline velocity is about 0.3 U, so
  q_w/q ≈ 0.09 (p. 8, figs. 4, 5). Behind a circular plate of the same area, static pressure has begun to recover by
  12 in (2.1 diameters) and is back to free stream by 18 in (3.2 diameters) (p. 8).
- **Two-dimensional fences on a wall.** Behind ribs at h/δ = 0.2 to 1.37 the mean flow reattaches about 10 h behind
  the front face (Arie and others, as cited by Suzuki, Kiya and Arie 1978, p. 1334); at h/δ = 0.125 to 0.375 reversed
  flow appears intermittently out to 30 h (p. 1334); a 1/7-power profile fits only beyond about 100 h (p. 1338). Plate
  (1963) puts reattachment at 12.5 h behind a fence, citing Nagabhushanaiah (p. 16), and the pressure gradient back
  to zero by 35 h (p. 17). A fence spans the whole width, so these are upper bounds for a brake of finite width.
- **Mirror image (*inference*).** A brake on a wall and its image form a free plate twice as tall, so the plate's
  3-width bubble becomes about 3 to 6 brake heights for brakes about as wide as tall; very wide ones tend to 10 h.

## 4. Measured aircraft stability with fuselage brakes out

All but the X-5 are wind-tunnel tests of whole models; the X-5 half-model flew on an F-51D's wing in dives (the
wing-flow method). None isolates the tail's normal-force slope, and none states the wake's position at the tail.

| Source | Brakes | Mach | Measured effect on stability |
|---|---|---|---|
| RM L50L11a (X-5) | 0.67 in fuselage side flap near the nose, 60° | 1.24 | dC_m/dC_L about 0.40 with and without |
| RM L8B06 (D-558-1) | two side flaps, 60° (fig. 2) | to 0.96 | "not appreciably changed", one tail setting |
| RM L55C17 (swept fighter) | perforated side flaps behind the wing | 0.15 | tail's part changed "fairly small" |
| RM SL54H24 (XF-103) | 45° brakes at the tail end, behind the tail | 0.4 to 1.15 | less stable above 1 |
| RM L56A25a (swept fighter) | perforated brakes near the wing | 0.8 to 1.05 | small; unstable at ≥ 0.98, CL ≈ 0 |
| TN D-4773 (swing-wing fighter) | under-fuselage flaps | 1.6 to 2.5 | directional: ungapped up, gapped down |

- **X-5** (PDF pp. 2, 4, 6): the flap sat on the fuselage side ahead of the wing; the neutral point stayed at 0.66 c̄.
  **XF-103** (fig. 4): the tail sits ahead of the brakes, so it is not a tail-in-wake case at all.
- **L55C17** (pp. 6, 9, fig. 3). Tanks and brakes together halved the directional stability (C_nβ 0.0022 to 0.0010
  at α = 0.3°), but the report traces this to their direct effect on the wing and fuselage, not to the tail; sealing
  the holes or moving the 3.8 in wide brakes 14.7 in (about 4 widths) rearward changed little (p. 9).
- **L56A25a** (pp. 4 to 6) computed trim assuming the stabilizer's effectiveness is unchanged with brakes out: an
  assumption, not a measurement. **XS-1** (RM L8A12): its tail in the brakes' wake cut the drag increment 7% to 13%
  ([airbrake-installed-drag.md](airbrake-installed-drag.md)); its brake runs were at zero lift only, with no slope.
- **Design practice (ARC R&M 2614, Davies and Kirk, 1951, p. 6),** on wing brake flaps. Tail buffet was avoided by
  keeping the brakes' wake clear of the tail plane, but wake widths could not be predicted and a tunnel test with
  tufts was needed. Chordwise venting with about 27% open area narrowed a wake by up to 20% at the same drag.
- **Reading (*inference*).** The aircraft cases are evidence that a tail out of line with a fuselage brake, or ahead
  of it, loses little at Mach 0.15 to 1.24, with exceptions near Mach 1; nothing about fins beside or just behind a
  brake, where hobby rockets put them. Above Mach 1.6 the sign depends on the brake (TN D-4773, pp. 1, 8).

## 5. What simulators do now

- **RocketPy** (MIT; `rocket/aero_surface/air_brakes.py`, current docs read 2026-11-05) sets the brakes' CP to zero
  and their lift slope to zero, and changes nothing on the fins: no wake loss and no brake normal force.
- **A conservative hand rule.** Maryland's 2022 team checked its flaps above the fin can by moving the CP to the flaps
  themselves, a stated worst case, and still found 2.17 calibers (2022 report, p. 23). It can bound brakes ahead of
  the fins, but not brakes between them: on WVU's rocket it would put the CP at 385 cm, 7.8 calibers behind the CG,
  where the CFD found 0.25 (*inference*).

## 6. A rule of thumb, with its range (*inference* throughout)

| Fin root leading edge, from the brake | q_w/q in the wake core | Loss of the fins' normal-force slope |
|---|---|---|
| beside the brake, or under 1 D behind | 0.1 to 0.5 (bubble, near wake) | 17% to 52% (WVU, Mach 0.8 to 0.2) |
| 1 to 2 D, or 3 to 5 h, behind | 0.09 (plate) to 0.36 to 0.49 (body) | not measured; 51% to 91% on area in it |
| 5 to 35 h behind | filling; 2D reverse flow to 30 h | unbounded; WVU's 13 h station lost up to 2.81 cal |
| beyond 35 h, or fin outside the brake's sector | near 1 | not shown to vanish (below) |

- **Far is not yet negligible:** see the 6 D station above; part of its loss is the brakes' own normal force.
- **Around the body.** The C.P. 251 wake was one to one and a half brake widths across two diameters downstream and
  WVU's narrowed with speed; 45° between a brake and a fin did not stop a loss at any Mach WVU ran.
- **Mach.** Below Mach 0.8 the loss fell with speed (WVU) and grew toward the slowest case run, Mach 0.2; brakes stay
  out below that on the way to apogee, so Mach 0.2's 52% is an edge of the data, not a bound. Near Mach 1 one CFD case
  grew and two aircraft tests turned less stable; above 1.6 the sign varied.

## What this suggests

- **Warn first, then model.** A warning can stand on this note today, in two tiers. Brakes whose station lies within
  the fin root chord, or within about 2 D (or 5 brake heights, whichever is longer) ahead of its leading edge: quote
  WVU's margin of 3.37 to 0.25 calibers at Mach 0.2 (3.76 to 3.12 at 0.8), worse below Mach 0.2. Any deployed brake
  ahead of the fins: quote WVU's 6 D station, −1.14 calibers at Mach 0.2 to 0.8 and −2.81 at 1.0. Expected effect: a
  margin read too high with brakes out is flagged wherever the one study saw a loss.
- **A bounding model.** For fins beside brakes or within 2 D behind them, keep at most 0.48 of the fins' normal-force
  slope (WVU, Mach 0.2) at every Mach, or 0.36 on the fin area in the wake (C.P. 251, *inference*), whichever is
  lower. On the WVU rocket this matches its worst row by construction; its use on other rockets is untested.
- **Include the brake's own normal force,** or bound it: brakes ahead of the CP lower the margin with no wake.
- **Pin which way it errs:** a test fires the warning on both WVU geometries, between the fins and 6 D ahead.

## Open questions

- No measurement of a fin's or tail's normal-force slope behind a body brake was found, at any Mach. A whole-rocket
  tunnel test with brakes at three axial stations and two angles around the body would settle it.
- WVU's CP noise floor (one steady solve at 1.43°, convergence watched on drag), its mesh study for fin forces, and
  its FB50 Mach 1.0 case; Monash's net normal force, which its pages don't give.
- Not read: Hoerner's *Fluid-Dynamic Lift*; Sakamoto and Arie (1983) on finite-width wall plates; Good and Joubert
  (1968) and AIAA 2025-98650 (paywalled); ESDU's fence items; XF-103's and L55C17's tail figures (read in words).
- Fin buffet in a brake's unsteady wake: C.P. 251 measured regular shedding behind plates, but no fin load; unbounded.

## Sources (all read 2026-11-05)

NACA and NASA reports are US government works (cached). ARC (Crown copyright), JSME (© JSME), the thesis, Plate's
report and the team reports: cited only.

1. M. L. Farha, "Influence of Actuated Air Brakes on the Apogee of Sub-Orbital Rockets", MS thesis, West Virginia
   University (2025), pp. 10 to 14, 21, 24 to 26, 38 to 39, table 3.3; read in a browser, not cached:
   https://researchrepository.wvu.edu/etd/12808/
2. S. Tandon, T. Mackellar, N. Tryndoch et al., "Design 1 Airbrakes Team, Submission 2", Monash University (2019),
   pp. 8 to 23: https://siddhant-tandon.com/files/Design1Report.pdf
3. R. Fail, T. B. Owen, R. C. W. Eyre, ARC C.P. 251 (1956), pp. 6 to 8, 11 to 12, figs. 4, 5, 18, 21:
   https://reports.aerade.cranfield.ac.uk/handle/1826.2/264
4. Y. Suzuki, M. Kiya, M. Arie, "Turbulent boundary layers disturbed by two-dimensional bluff bodies", Bulletin of
   the JSME 21 (159), pp. 1332 to 1340 (1978), pp. 1334, 1338:
   https://www.jstage.jst.go.jp/article/jsme1958/21/159/21_159_1332/_article/-char/en
5. E. J. Plate, Colorado State University CER63EJP66 (1963), pp. 16 to 17; cached by
   [airbrake-blade-partial.md](airbrake-blade-partial.md): https://hdl.handle.net/10217/198002
6. R. M. Kennedy, NACA RM L50L11a (1951), PDF pp. 2, 4, 6, fig. 4: https://ntrs.nasa.gov/citations/19930086470
7. J. B. Wright, NACA RM L8B06 (1948), pp. 4 to 5, 9, fig. 2: https://ntrs.nasa.gov/citations/19930085315
8. W. C. Sleeman, W. J. Alford, NACA RM L55C17 (1955), pp. 1, 6, 9, fig. 3:
   https://ntrs.nasa.gov/citations/19930088637
9. A. A. Luoma, NACA RM SL54H24 (1954), pp. 1, 11 to 12, fig. 4: https://ntrs.nasa.gov/citations/20090023318
10. D. D. Arabian, NACA RM L56A25a (1956), pp. 4 to 6: https://ntrs.nasa.gov/citations/19930089201
11. C. S. Richardson, NASA TN D-4773 (1968), pp. 1, 6 to 8: https://ntrs.nasa.gov/citations/19680024197
12. H. Davies, F. N. Kirk, ARC R&M 2614 (1951), p. 6: https://reports.aerade.cranfield.ac.uk/handle/1826.2/3152
13. E. Bregin, A. Bean, H. Daniel et al., Terrapin Rocket Team, 2022 Spaceport America Cup report, p. 23:
    https://drum.lib.umd.edu/items/d6631eb4-dddd-4c62-95bf-eb313fe2770b
14. RocketPy (MIT), `air_brakes.py`, docs:
    https://docs.rocketpy.org/en/latest/_modules/rocketpy/rocket/aero_surface/air_brakes.html
