# What Kidwell's and K. Steele's NAR parachute reports measured

[NAR R&D reports](nar-rd-reports.md) ranked Kidwell's plastic and Mylar drops (NARAM-44, 2002) and K. Steele's
parachute flights (NARAM-49, 2007) as NAR's likeliest small-canopy descent measurements. This note reads both scans
(150 and 300 dpi renders): what each flew, what its rows say, and the drag coefficient (C_D0, on the nominal area
S_0) they imply against HPR Sim's flat-circular default of 0.775 ([recovery](../physics/recovery.md)) and the 45 in
curves in [small-canopy speed and C_D](small-canopy-speed-cd.md). "Inference" marks this note's own arithmetic.

**In short: neither report measures a canopy at hobby-main speeds (15 to 22 ft/s), so neither can size #741.**

- **Kidwell's garage drops can't give a C_D.** Nine 45 cm canopies, 8.5 to 9.2 g each, fell 6.0 m in 8.85 to 10.60 s
  on average (1.9 to 2.2 ft/s). Every printed average reproduces from the rows. They imply a vertical C_D0 of 1.9 to
  2.9 (**Inference**), 2.4 to 3.7 times the default and above all of Knacke's 45 in curves (1.21 at most). Rising
  air of 0.3 to 0.5 m/s or a 39° to 54° glide would explain it. The short fall doesn't: it biases C_D high by under 1%.
- **Steele's flights are the only usable numbers, at 9 to 13 ft/s.** Two clean flights of an 18 in LOC nylon chute
  imply C_D0 0.76 to 1.08 on the 18 in circle at 11.2 to 11.7 ft/s; one clean flight of an 18 in Estes plastic chute
  implies 0.96 to 1.38 at 9.2 to 9.5 ft/s (**Inference**, after taking out the ascent). Only the plastic flight sits
  above 0.775 on every reading; the nylon pair's band reaches below it. Both overlap Knacke's 45 in curve there.
- **The two nylon flights differ by 1.6 times in C_D0,** and the report doesn't fix the mass: at most a weak hint of
  the direction #741 states, not a size (**Inference**).

**How far to trust it:** high on what each report printed (every average recomputed); low on any C_D. Kidwell's
site, release and air are unknown; Steele's mass, chute shapes and timing span aren't stated, and the bands carry
that. This note feeds #741 and leaves [small-canopy speed and C_D](small-canopy-speed-cd.md)'s 3.5 to 12% as it is.

## Method

- **C_D from a descent.** At a steady vertical speed v, C_D0 = 2mg/(ρ v² S_0), with m the whole falling mass,
  canopy included. A canopy that glides at θ from vertical shows a vertical C_D of C_R/cos²θ, C_R being its resultant
  force coefficient; at 45° that doubles it.
- **Area.** S_0 is the whole cloth area, vent included (Knacke's convention, as `recovery.md` uses it). A regular
  hexagon sized across flats has 1.103 times the area of the circle of that size, so its C_D0 is 0.907 times the
  circle-based one; across corners 0.827 times the area and 1.209 times the C_D0. An octagon: 1.055 and 0.948 across
  flats, 0.900 and 1.111 across corners (geometry, as in [hobby parachute C_D](hobby-parachute-cd.md)).
- **Reaching terminal speed.** From rest under drag as v², the fall reaches 99% of terminal speed in about
  1.96 v_t²/g and lags a fall at terminal speed by (v_t/g)·ln 2 in time.
- **The default's error.** At one load, descent time and drift go as √C_D0 and landing energy as 1/C_D0, so a canopy
  of true C_D0 C flies √(C/0.775) times the predicted time, and the predicted energy is C/0.775 times the true one.

## 1. Kidwell, "Performance of Plastic and Mylar Parachutes" (NARAM-44, 2002): the setup

- **Canopies** [1, pp. 1 to 3]. Three each of clear 0.25 mil plastic, opaque 0.31 mil plastic with an anti-stick
  coating, and 0.25 mil aluminized Mylar, cut from one cardboard template. Table 1 gives surface densities of 5.7,
  8.1 and 9.1 g/m². The size is "45 cm diameter"; the Methods (p. 2) call the canopies octagonal and the abstract
  (p. 1) hexagonal. Neither says across flats or corners. No vent or line count is stated.
- **Lines and load** [1, pp. 2, 3]. Lines 68 cm (1.5 D) of Kevlar thread, taped on with polyester tape; a 7.3 g lead
  weight stands in for an empty competition rocket. Table 2 (0.1 g balance): canopies 0.8 to 1.5 g, with lines 1.1
  to 1.9 g, with the weight 8.5 to 9.2 g. The 8.5 to 9.2 g is the falling mass; it includes the canopy.
- **Drops** [1, p. 3]. Outdoors off a parking garage, 6.0 m, on one July evening from 8:30 to 11:00 pm, with no
  wind reported. Seven drops per canopy in a fixed order: the 0.25 mil plastic first, then 0.31 mil, then Mylar.
- **Timing** [1, p. 3]. A stopwatch, read to 0.01 s. The report doesn't say how the canopy was released (open or
  bunched) or where the clock started; it names opening times and thermals as part of its error bars (p. 4). The
  Mylar often came down cone-shaped rather than fully open, and one 0.25 mil canopy didn't open (p. 4).
- **Which way the size goes** (**Inference**). Canopy mass over surface density gives 0.140 to 0.175 m² per canopy,
  0.164 to 0.165 m² per material mean (0.155 to 0.173 m² at the ±0.05 g reading). At 45 cm across flats an octagon
  is 0.168 m², a hexagon 0.175 m², a circle 0.159 m²; across corners 0.143 and 0.131 m², below all three means. So
  the 45 cm is likely across flats, unless Table 1's densities were worked out from these canopies.

## 2. Kidwell: rows, averages and statistics

- **Rows** [1, Table 4, p. 4]. 63 slots, 61 times (one canopy didn't open, one timer failed), counted here.
  Drops run from 5.27 to 14.27 s.
- **Averages reproduce** (computed here). All nine canopy means (8.47 to 10.99 s), deviations (1.73 to 2.45 s) and
  "50% confidence" values (0.48 to 0.65 s, which are 0.674 × SD/√n) match the rows to 0.01 s, as do the material
  means 10.60 s (20 drops), 9.95 s (21) and 8.85 s (20) with ±0.32, ±0.31 and ±0.28 s. Nothing is misprinted.
- **Scatter** (**Inference**). Each material's drops scatter by 1.9 to 2.1 s, about 20% of the mean and ten times a
  hand-timing error of 0.2 s. Round means fall from 11.30 s (drop 2) to 8.03 s (drop 6) over all materials, so the
  air changed over the evening; with a fixed order, time of night and material are confounded.
- **Table 5 tests the scatter, not the means.** Its "probability of significant difference" (1%, 41%, 41%, p. 6)
  is reproduced here as 1 minus the two-sided p of an F-test on the variances (F 1.005 and 1.29). On the means, a
  Welch t-test gives p 0.33 for the two plastics, 0.009 for 0.25 mil plastic against Mylar and 0.086 for 0.31 mil
  against Mylar (computed here, before the order confound).
- **Descent speed.** 6.0 m over the means is 0.57, 0.60 and 0.68 m/s (1.86, 1.98 and 2.22 ft/s); the report rounds
  to about 0.6 m/s (2 ft/s) [1, p. 4]. Reynolds number on 45 cm is about 1.6×10⁴ (ν 1.6×10⁻⁵ m²/s, **Inference**).

## 3. Kidwell: the C_D it implies, and why that isn't the canopy's

- **Implied vertical C_D0** (**Inference**). On the octagon across flats (0.168 m²) with ρ 1.10 to 1.18 kg/m³ (the
  drop site isn't named; a low site on a warm evening): 2.67 to 2.86 for 0.25 mil plastic, 2.43 to 2.61 for 0.31 mil
  and 1.98 to 2.12 for Mylar. On a hexagon across flats each is 4% lower (1.89 at the least). Per drop, the fastest
  (5.27 s) gives 0.70 and the slowest (14.27 s) 5.2, on an 8.9 g load at ρ 1.14.
- **Not a short-fall bias.** At 0.57 m/s a canopy released open reaches 99% of terminal speed in 7 cm and loses
  0.04 s on 10.6 s: C_D reads 0.8% high (**Inference**). A canopy released bunched falls fast until it opens, which
  biases C_D low by an amount the report doesn't allow to bound.
- **What would explain it** (**Inference**, both high-side). At C_D0 0.775 to 1.2 the 0.25 mil plastic canopy would
  fall in 5.6 to 7.0 s in still air, against 10.6 s measured. Rising air of 0.28 to 0.52 m/s, averaged over each
  drop, closes the gap; so does a glide of 48° to 54° from vertical at C_R 1.0 to 1.2 (39° to 54° over all three
  materials). July-warmed concrete makes rising air plausible; Knacke's 45 in models glided at up to 45° loaded
  light ([small-canopy speed and C_D](small-canopy-speed-cd.md) §1). Neither is measured: their share is **unbounded**.
- **Far from a hobby main's loading.** Kidwell's canopies carry about 0.51 N/m² (W/S_0); a hobby main at 15 to
  22 ft/s with C_D0 0.775 carries 9.7 to 21 N/m² (ρ 1.2, **Inference**). One of Van Milligan's two roof series read
  high too (1.38 to 1.65 on his plain 18 in canopy; the other 0.68 to 0.82; [NAR R&D reports](nar-rd-reports.md) §6).
- **What survives is a ratio, weakly.** At equal size, Mylar's C_D0 comes to 0.74 of the 0.25 mil plastic's
  ((8.85/10.60)² × 9.17/8.63 g, **Inference**), with the cone shape, the drop order and the air all mixed in.

## 4. K. Steele, "A Comparison of Parachute Descent Rates" (NARAM-49, 2007): the setup

- **Title and kind.** NAR's index lists "Parachute Descent Rates" for id `1358130` [5]. It is a flight test.
- **Rockets and chutes** [2, pp. 2 to 4, 7]. Two Quest Astra rockets with added payload sections, on Estes C6-5
  motors; each weighs 78 g. An Estes 18 in plastic chute weighs 6 g, a LOC 18 in nylon chute 22 g (Figure 4-2,
  p. 4). Shapes, line lengths and vents aren't given. Estes states only LDPE and 18 in (46 cm) across [8]; LOC's
  current 18 in chute has six lines and a 50 mm vent [7], 1.2% of S_0 (2007's may differ).
- **Field and timing** [2, pp. 4, 7]. Rainbow Valley, Arizona; temperature "103" with no unit (°F, **Inference**);
  "slight breeze". One Perfect Flite altimeter gives each flight's altitude, and a stopwatch its "Duration".
- **Mass** (**Inference**). The report doesn't say whether the 78 g includes the motor or the altimeter. A C6 is 24 g
  loaded and 13 g spent [6]. So the descending mass is 73 to 97 g on the plastic chute and 89 to 113 g on the nylon
  (73 and 89 g if the 78 g was weighed with a loaded motor); an outside altimeter would raise both, **unbounded**.
- **Air** (**Inference**). The terrain at Rainbow Valley's listed coordinates is 340 m [9, 10]. For 300 to 450 m and
  39.4 °C, ρ is 1.07 to 1.09 kg/m³.

## 5. K. Steele: rows, averages and what they imply

- **Rows** [2, Figures 4-2 and 4-3, pp. 4, 5]. Plastic, rocket 1: 583 to 685 ft, 37.31 to 70.55 s; four of five
  note tangled lines or a burned cord, and flight 3 flew the backup. Nylon, rocket 2: 452 to 534 ft; flights 2 and 3
  failed (no test), flight 1 is "wobbly" and tangled, and flights 4 and 5 carry no note.
- **The printed rates are altitude over the whole duration** (Figure 5-1, pp. 5, 6). All eight reproduce within
  0.08 ft/s: five round to the printed value, and plastic flights 1, 2 and 3 (8.55, 15.63, 13.95) print 8.6, 15.7
  and 13.9. The averages 12.5 and 10.4 ft/s reproduce (12.48 and 10.37); they include the tangled flights.
- **The summary contradicts the results.** The summary (p. 2) says the plastic chute had the lower descent rate; the
  results and conclusion (p. 6) say it fell faster, 12.5 against 10.4 ft/s; the tables support the latter.
- **Ascent inside the duration** (**Inference**). If "Duration" runs from launch, it includes the climb: a C6-5
  ejects about 6.9 s after ignition (1.9 s burn [6] plus the 5 s delay). Taking 5 to 7 s off raises the descent
  speeds by 8 to 19% and lowers C_D by 14 to 29%. The fall from apogee to full opening is faster than the canopy's,
  so the table reads C_D a little low, unbounded here. Terminal speed comes within 3.4 m of opening, of 150 m or more.

| case (Inference) | speed, ft/s | C_D0, 18 in circle | hexagon, flats | hexagon, corners |
|---|---|---|---|---|
| nylon, flights 4 and 5 pooled | 11.2 to 11.7 | 0.76 to 1.08 | 0.69 to 0.98 | 0.92 to 1.30 |
| nylon, flight 4 | 12.7 to 13.3 | 0.59 to 0.85 | 0.54 to 0.77 | 0.71 to 1.02 |
| nylon, flight 5 | 10.1 to 10.5 | 0.95 to 1.34 | 0.87 to 1.22 | 1.15 to 1.62 |
| plastic, flight 1 | 9.2 to 9.5 | 0.96 to 1.38 | 0.87 to 1.25 | 1.16 to 1.67 |

- Each range spans 5 to 7 s of ascent, the mass range and ρ 1.07 to 1.09; the circle of 18 in is 0.164 m².
- **Scatter.** Flights 4 and 5 used one rocket and one chute, yet differ by 1.6 times in C_D0 (0.75 against 1.20
  at the midpoints). Air motion over a 39 °C desert is the likeliest cause (**Inference**); no time of day is given.
  Reynolds number on 18 in at 3.5 m/s is about 9×10⁴ (**Inference**), just under Cockrell's 10⁵ knee.

## 6. Against the 0.775 default and the 45 in curves

- **Same speeds, 45 in models** (Knacke, read in [small-canopy speed and C_D](small-canopy-speed-cd.md) §1). Figure
  5-18's lower flat curve reads 1.12 at 10 ft/s and 0.97 at 15; linearly, about 1.08 at 11.5 ft/s, and 1.14 at
  9.3 ft/s if extended below its 10 ft/s end (**Inference**). Its upper curve is 0.09 higher. Neither figure goes
  below 10 ft/s, so Kidwell's 2 ft/s has no curve to compare with.
- **Steele overlaps the curve.** The pooled nylon 0.76 to 1.08 (circle) reaches the lower curve's 1.08 at its
  top; plastic flight 1's 0.96 to 1.38 spans the extended 1.14. Neither can size the rise at low speed.
- **What the default would do at Steele's speeds** (**Inference**). With the nylon pool's 0.76 to 1.08, the 0.775
  default predicts descent time and drift from 1% long to 15% short and landing energy from 2% low to 39% high. On
  a hexagon across flats the low end is 0.69. Flight 4 alone (0.59 to 0.85) straddles 0.775. So the sign holds on
  plastic flight 1 on every reading (0.87 at the least), not on the nylon pair nor on every flight.
- **At hobby-main speeds** (15 to 22 ft/s) neither report has a point. Kidwell's loading is 2 to 5% of a hobby
  main's, Steele's 4.4 to 6.8 N/m² (on the circle) about 20 to 70% (**Inference**). Steele's line lengths are
  unknown; Kidwell's 1.5 D would add 7 to 9% against lines of one D_0 (Figure 5-20, in the same note §4).

## What this suggests

- **#741 stays on Knacke's 45 in curves.** Neither report should set or bound the default at 15 to 22 ft/s.
  `recovery.md` could cite Steele's plastic flight as one flight of an 18 in canopy above 0.775 at 9 ft/s (0.96 to
  1.38), the same direction, beside the nylon pair's 1.6 times spread and band reaching below 0.775.
- **Kidwell as a caution, not a reference.** Its drops show why light roof or garage drops can't validate a C_D:
  every average is right, yet the implied 1.9 to 2.9 needs rising air or a glide. A validation case should not
  replay it for C_D0; its material ratios could at most back a note that Mylar canopies may not open fully.
- **A project drop test, if one is made,** would need a hobby-main loading (10 to 21 N/m²), a fall of tens of
  meters, still indoor air, a release with the canopy open, and the area convention measured. Neither report
  meets more than one of these.
- **Terms.** Both reports are NAR members' downloads with no license; Steele grants publication in NAR
  publications (p. 7) and nothing wider. Cite, don't copy; the scans stay in the private cache.

## Open questions

- Kidwell's drop site, how each canopy was released, and when the clock started and stopped.
- What Steele's "Duration" spans, whether the 78 g includes the motor and altimeter, the chutes' shapes and line
  lengths, and the time of day of each flight.
- A drop or logged flight of a 2 to 4 ft canopy at 15 to 22 ft/s with mass, area convention and line length stated;
  none was found in either report.
- Kidwell's leads, Coleman on line length for 50 cm chutes (NARAM-41, 1999) and Etzel on spill holes (NARAM-39,
  1997), are unread.

## Sources

1. C. Kidwell, "Performance of Plastic and Mylar Parachutes" (index title "Performance of Plastic and Mylar
   Chutes"), NAR R&D report, NARAM-44, August 2002, <https://www.nar.org/docs.ashx?id=1358053> (member sign-in),
   read 2026-11-03 from the cached scan, pp. 1 to 6 and Tables 1 to 5. No license stated: cited, cached privately.
2. K. Steele, "A Comparison of Parachute Descent Rates", NAR R&D report, NARAM-49, July 2007,
   <https://www.nar.org/docs.ashx?id=1358130> (member sign-in), read 2026-11-03 from the cached scan, pp. 1 to 7,
   Figures 4-1 to 5-1. Publication granted to NAR only (p. 7): cited, cached privately.
3. [Small-canopy speed and C_D](small-canopy-speed-cd.md) §1 and §4, the project's readings of T. W. Knacke,
   *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), Figures 5-16, 5-18 and 5-20,
   <https://apps.dtic.mil/sti/pdfs/ADA247666.pdf> (approved for public release).
4. [Hobby parachute C_D](hobby-parachute-cd.md), Method and §2 (area conventions of hexagonal hobby chutes).
5. [NAR R&D reports](nar-rd-reports.md) §5 and §6 (ranking; Van Milligan's roof drops).
6. ThrustCurve, Estes C6 motor page, <https://www.thrustcurve.org/motors/Estes/C6/>, read 2026-11-03 (burn time
   1.9 s, loaded mass 24 g, propellant 11 g). Cited only.
7. LOC/Precision, parachutes product page, <https://locprecision.com/products/locprecision-parachutes/>, read
   2026-11-03 (18 in: 50 mm spill hole, six lines; current product). Cited only.
8. Estes, 18 in Printed Parachute (SKU 002267), Wayback copy of 2025-02-09 of
   <https://estesrockets.com/products/18-inch-printed-parachute>, read 2026-11-03 (LDPE, 18 in (46 cm)). Cited only.
9. Wikipedia, "Rainbow Valley, Arizona", <https://en.wikipedia.org/wiki/Rainbow_Valley,_Arizona>, read 2026-11-03
   (coordinates 33°13′55″N 112°23′14″W; no elevation). CC BY-SA 4.0: cited only.
10. Open-Meteo Elevation API (Copernicus DEM GLO-90), read 2026-11-03 (340 m at the coordinates below), CC BY 4.0:
    <https://api.open-meteo.com/v1/elevation?latitude=33.2319&longitude=-112.3872>.
