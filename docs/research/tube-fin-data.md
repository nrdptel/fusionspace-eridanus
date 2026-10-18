# Measured lift, center of pressure and drag of tube fins

HPR Sim flies each tube of a [tube fin](../physics/aero.md#tube-fins) set as an isolated ring wing: Weissinger's slope,
Fletcher's measured center joined to the leading edge, no interference. Up to Mach 0.5, on OpenRocket's *Tube fin
rocket* and 14 probes, its center of pressure sits 0.42 to 3.0 calibers forward of OpenRocket's (#228, #234), and
nothing measured says which code is nearer. This note asks what measurements exist, at low speed, for ring and annular
tails, clusters of tubes and lattice fins: normal-force slope, center of pressure (CP) and drag, with geometry stated.
It also asks whether the body–tube interference that slender-body theory predicts shows in them. "Inference" marks
this note's own arithmetic.

**In short: no open source measures a hobby tube cluster's slope or CP, so #228 and #234 stay open.** The nearest data
are single rings around a body. In two Army test series those lifted *less* than the ring alone: 0.24 to 0.69 of it at
Mach 0.8 (22 values, mean 0.43), 0.42 to 0.62 at Mach 0.3 to 0.5, with the CP near the leading edge (0.07 c on
average). That is the opposite sign to the rise #234 estimates for tubes *beside* a body, a different geometry. Second,
OpenRocket's extra lift is not interference: the ring formula its documentation describes, on the inner diameter,
reproduces all 75 of its recorded tube-set slopes. It leaves out Weissinger's long-ring term, so it lifts 1.34 to 1.82
times HPR Sim's for tubes 1 to 6 diameters long. Third, HPR Sim's square-edge drag on the tube wall reads high by 2D
data, about 18% of a tube's drag, against the known gap; the missing drag is likely at junctions and channels, which
nothing open measures. The best unread lead, a NAR report on tube-fin stability, needs a member login. Confidence: high
that the open literature has no cluster data (DTIC was down on 2026-10-18); medium on numbers read from plots.

## 1. Isolated rings: two more short ones, no long one

HPR Sim already uses Fletcher's five Clark Y rings (NACA TN 4117, d/c = 1/3 to 3). No other isolated ring longer than
1.5 diameters was found, so a paper tube's CP still rests on HPR Sim's line from d/c = 2/3 to the leading edge. Two
more short rings, each with a body inside, check Weissinger's slope:

- **Parlett, NACA TN 3547 (1955) [R1].** A shrouded-propeller ring with the propeller removed and its motor left in:
  18 in bore, 12.25 in long (d/c = 1.47), flat-plate wall with a round lip, 56 ft/s, coefficients on c·d (pp. 2–3,
  Fig. 1). From Fig. 2 (p. 7): C_L = −0.09, 0.58 and 1.18 at 0°, 10° and 20°, so 3.6 to 3.8 per radian. Weissinger
  gives 3.89 at c/d = 0.68, 1% to 7% above (Inference). Its aerodynamic center, from dC_m/dC_L on three points, is about
  0.3 c, aft of Fletcher's 0.253 c at d/c = 1.5 (Inference, weak).
- **Grunwald and Goodson, NASA TN D-995 (1962) [R2].** A shroud of 10.31 in chord, propeller off, with a nacelle
  inside. C_L ≈ ±0.6 at ±10° (Fig. 8) gives 3.4 per radian on the report's area: 20% below Weissinger on the 16.9 in
  exit diameter, 2% below on the 15.0 in propeller diameter (Inference). For a paper tube the choice doesn't matter.

## 2. Single rings around a body: lift lost, CP forward

**Brazzel, Henderson and Craft, Army Missile Command RD-TR-65-7 (1970) [R3].** A 1.15 in body, 10 calibers long with a
4-caliber ogive. Rings with a half-diamond section and a 4° internal flare sat on four posts at stations near the base,
Mach 0.8 to 4.5 (pp. 1–2; Fig. 2). Table II gives the ring's increment, configuration less body alone, with "all
interference factors" included (p. 3). The Mach 0.8 rows of the Fig. 2 series (T6 and T7, other flare angles, left
out), against Weissinger's slope with Göthert's rule at Mach 0.8 on the ring's mean diameter (Inference):

| ring | leading-edge diameter (cal) | chord (cal) | measured ΔC_Nα, per degree | isolated ring | ratio |
|---|--:|--:|--:|--:|--:|
| T1 | 1.50 | 0.750 | 0.043 to 0.050 | 0.148 | 0.29 to 0.34 |
| T2 | 1.50 | 1.125 | 0.046 to 0.054 | 0.162 | 0.28 to 0.33 |
| T3 | 2.00 | 1.500 | 0.176 to 0.200 | 0.288 | 0.61 to 0.69 |
| T4 | 2.00 | 1.000 | 0.127 to 0.164 | 0.262 | 0.48 to 0.63 |
| T5 | 2.00 | 0.600 | 0.106 to 0.123 | 0.220 | 0.48 to 0.56 |
| T12 | 1.25 | 1.250 | 0.038 to 0.047 | 0.119 | 0.32 to 0.39 |
| T13 | 1.25 | 0.937 | 0.027 to 0.037 | 0.113 | 0.24 to 0.33 |

- Over all 22 values the ring on the body gave 0.24 to 0.69 of the isolated ring's lift, mean 0.43. The rings with a
  0.5-caliber gap to the body (0.48 to 0.69) stand apart from those at 0.125 and 0.25 calibers (0.24 to 0.39). Where the
  body's boundary layer is thick against that gap, the authors write, more chord need not add normal force (p. 4).
- **CP.** The 21 Mach 0.8 values average 0.066 of the chord aft of the leading edge, standard deviation 0.18 (Inference,
  counted from Table II). The report's own curve (Fig. 8, p. 35) starts near 0.09 c.
- **Slender-body reading (Inference).** In cross flow a closed thin ring carries the air inside it, an apparent area of
  2πR²: twice a solid body's, which is Hoerner's long-ring limit. With a coaxial body of radius a inside, only the
  annulus is trapped air, so the ring adds 2π(R² − a²): a factor 1 − (a/R)² on the isolated ring. That is 0.42 to 0.77
  for these rings, and the measured ratios divided by it are still 0.47 to 0.90. Theory and data agree on the sign; the
  data lose more.

**Dahlke and Craft, Army Missile Command RD-TM-72-5 (1972) [R4].** A 3.0 in cone-cylinder with a ring of the same
diameter, a 0.075 in wall (t/d = 0.025, near a paper tube) and a chord of one diameter, held behind the base on struts
2.5 to 4 calibers long. AEDC tunnel 4T, Mach 0.5 to 1.3, one ring also at Mach 0.3 (p. iii; Fig. 5). Read by eye
(±0.003 per degree, on the body's frontal area): forebody alone 0.033 to 0.039 at Mach 0.5 (p. 111); with ring and
struts 0.066 to 0.071 (p. 127). The ring's increment is 0.42 to 0.62 of an isolated ring's (Inference). It flies in the
body's wake, a different cause from Brazzel's.

**What carries over.** These are concentric rings, the lowest Brazzel point Mach 0.8. Hobby tubes sit *beside* the body,
each with an open bore, where its upwash may raise their lift instead (#234: 1.13 to 1.96 times). Nothing measures that.
Inference: if such tubes lose lift as these rings did, HPR Sim's CP is too far aft and its margin flatters the rocket.

## 3. Tube clusters and tube-finned projectiles: nothing measured

Searches of NTRS, Google Scholar, DTIC through archived copies, and the NAR R&D index (2026-10-17, 540 report ids) found
no wind-tunnel, range or flight measurement of the slope or CP of 3 to 8 open tubes around a body. No tube-finned
projectile, mortar or rocket round with a cluster of tubes turned up in a public report. The Army's ringtail work [R3]
and MIL-HDBK-762 [R5] cover single rings; the handbook says a ringtail gives about twice the restoring moment of a
cruciform of equal span and chord, at a high drag cost (§5-3.2, p. 5-14). An OpenRocket developer wrote in 2022 that
only a wind tunnel would show which tube-fin model is right (PR #1413 [R7]).

**OpenRocket's extra lift is its ring formula, not interference.** RocketPy's documentation says its tube-fin model
follows OpenRocket's, and states it [R6]: Ribner's ring airfoil (1947) for each tube, times the number of tubes, with
the CP at a quarter of the tube's length up to Mach 0.5. OpenRocket PR #3235 confirms the quarter length [R8]. On the
area d·L, with d the *inner* diameter and λ = L/d, that slope is π²/(1 + πλ/2). It reproduces every tube-set slope in
`validation/fixtures/ork/openrocket-tube-fin-aero.json`, all 75 rows (the *Tube fin rocket* and 14 probes, five Mach
numbers each), to a relative 4 × 10⁻¹⁶ (Inference, from the record's radii, lengths, counts and reference areas).
Weissinger's slope, which HPR Sim uses on the mean diameter, adds λ·arctan(1.2λ) below the line. On one diameter:

| λ = L/d | 0.5 | 1 | 2 | 3 | 4 | 6 |
|---|--:|--:|--:|--:|--:|--:|
| Ribner's slope over Weissinger's | 1.15 | 1.34 | 1.57 | 1.68 | 1.75 | 1.82 |

So the record's "1.26 to 1.86 times HPR Sim's" is this λ-dependence, plus the inner against mean diameter and
HPR Sim's compressibility factor. For long
tubes Ribner's form tends to 2πd², twice Hoerner's long-ring limit of πd² (Hoerner and Borst, p. 17-10, eq. 23).
Fletcher's thick d/c = 1/3 ring is the only measurement on that question: Hoerner and Borst read its slope as
"practically the same" as the long-ring limit (p. 19-16, Fig. 24). One thick ring leans toward Weissinger's form
without settling it.

**Hobby work.** All unread:

- S. Avecilla, "Stability of Tube Fins", NARAM-39, 1997, NAR R&D (index id 1357957). Apogee's *Peak of Flight* #347
  cites it for tube-fin rockets being "less stable than regular flat fin rockets", with no number [R9, p. 7].
- Four more NAR R&D reports on tubes or ring fins (ids 1357981, 1358056, 1358168, 1358248) and L. Brand's three
  *Sport Rocketry* articles (2008, 2010), behind a member login; a high-school CP study listed in #347 [R9, p. 8].
- *Peak of Flight* #689 (2026-10-15) describes an OpenFOAM study of 11 configurations at 20 m/s and 5° behind a
  "corrected tube-fin model" in RockSim Pro [R10]. It publishes no slope, CP or tube geometry, and it is CFD.
- OpenRocket PR #1312 compared its tube-fin drag with one rocket's flights: 377 ft simulated against 349 and 355 ft on
  a B6, and 689 ft against 694 and 697 ft on a C6 [R11]. That tests drag, not CP.

## 4. Lattice fins below Mach 0.8

The one open subsonic grid-fin test with its geometry stated is Hiroshima and Tatsumi, ICAS 2004 paper 092 [R12]:
square, triangle and hexagon cells of 25 mm², 5 mm chord, 0.2 mm webs, Mach 0.5 to 2.0, fin balances.

- **Slope.** About 0.10 per degree at Mach 0.5 on the body's area for all three (Fig. 7, by eye). The hexagon has 78% of
  the square's lifting-surface area and about the same normal force: lift follows the open cells (pp. 3–4).
- **Choking.** At Mach 0.8 the authors take square and triangle cells to choke above about 5°, hexagon cells at every
  angle (Fig. 8, p. 3).
- **CP.** Hinge moment over normal force at Mach 0.5 puts it 0.26 to 0.34 of the chord from the leading edge
  (Inference; the sign rests on one sketch, Fig. 6), about 0.1 c aft of Fletcher's ring at d/c = 1. A grid cell shares
  walls, so this neither confirms nor moves the tube-fin CP. Washington and Miller's Army tests were not reached.

## 5. Drag of short open tubes

HPR Sim charges friction on both faces of each tube, 2πL(r_o + r_i), and gives the wall's front ring a square fin
edge's pressure drag: 0.85 q_stag/q at the face plus base drag 0.12 + 0.13M² behind (about 0.97 at low speed).

- **Friction.** Fletcher's zero-lift drag [R13, Fig. 12] is 0.011 to 0.035 on d·c, or 0.0018 to 0.0056 per wetted area
  2π·d·c (Inference). All five are at or below Schlichting's turbulent flat-plate friction with a form factor of 1.25,
  at chord Reynolds numbers of 0.7 to 2.1 million. His rings are thick and rounded, so this checks friction only.
- **Edges (Inference).** Hoerner's two-dimensional data put a square leading edge, past reattachment, about 0.4 above a
  rounded one [R14, Fig. 22, p. 3-12]. A thin blunt trailing edge's base drag (eqs. 38–39, p. 3-21) comes to 0.07 to
  0.12 for a 0.33 mm wall 76 mm long, against HPR Sim's 0.12 to 0.20. This repeats [fin-edge-drag](fin-edge-drag.md)
  for flat fins. On the *Tube fin rocket*'s tube with C_f = 0.004, the edges are 34% of one tube's zero-lift drag in
  HPR Sim; Hoerner's values cut that tube's drag by 16% to 18%. That raises the apogee, widening the 6.95% gap above
  OpenRocket rather than closing it.
- **What no source measures.** The drag of thin tubes against the body and each other: junction lines, the channels
  between tubes, a lip in the body's boundary layer. It is likely positive, as for fin–body junctions
  ([junction-drag](junction-drag.md)), and unbounded: the one candidate found that could make HPR Sim's drag read low.
- **Internal choking (Inference).** Friction accelerates the flow in a tube (Fanno flow), which chokes when 4fL/D
  reaches the Fanno limit at the inlet Mach number. With f = 0.004, tubes of L/D = 1, 3 and 6 choke at Mach 0.90,
  0.83 and 0.78; the *Tube fin rocket*'s (L/D = 3.2) at 0.81 to 0.85 for f = 0.003 to 0.005. Angle of attack, the
  channels and lip separation choke earlier, by an unbounded amount. HPR Sim refuses tube fins from Mach 0.8: that
  holds for L/D ≤ 3, and is above the friction-only onset past L/D ≈ 4.6.

## What this suggests

- **Keep the "smaller margin is more cautious" advice as a direction, not a bound.** Every on-body ring measurement
  found shows less lift than the isolated ring and a CP near its leading edge, both moving a rocket's CP forward.
  They are concentric rings, so the sign for tubes beside a body is still unmeasured.
- **Record why OpenRocket's slope is higher.** ADR-102, the aero page and #234 can say the 1.26 to 1.86 band is
  Ribner's short-ring formula on the inner diameter, exact on all 75 rows. It is a code-to-code explanation, not a
  validation of either, though Hoerner's long-ring limit and Fletcher's long ring lean toward Weissinger's form.
- **Don't ship the edge-term fix alone.** It lowers tube drag about 18%, away from the only flight-refined reference;
  pair it with a junction and channel term once one is measured.
- **Check the Mach 0.8 refusal for long tubes.** A friction-only estimate chokes a tube of L/D = 6 at about Mach 0.78.
  A limit that falls with L/D past about 4.6 would be cautious.
- **Get Avecilla 1997 and the other four NAR R&D reports.** Avecilla's is the likeliest measured tube-fin CP found.

## Open questions

- Does lift per tube rise beside a body (slender-body theory, #234) or fall (the concentric data's loss; tube-to-tube
  crowding)? Nothing measured separates them.
- Where is the CP of a thin tube about three diameters long, alone or on a body? No source puts it between HPR Sim's
  0.07 L and OpenRocket's 0.25 L, or outside that range.
- Weissinger's "Ring plus body" (AFOSR TN 60-343, 1960), DTIC AD0452725 (a 1964 bibliography of ring-wing papers) and
  AD0467829 (ringtails 1.25 to 2.50 calibers across on a body) were unreachable on 2026-10-18. Werle (AIAA J., 2014),
  which compiles ring-wing data against Weissinger's slope, is paywalled.

## Sources

US government works are public domain; copyrighted items are cited only, cached under the gitignored `refs/`.

- **R1.** L. P. Parlett, NACA TN 3547, 1955, pp. 2–3, Figs. 1–2. **R2.** K. J. Grunwald, K. W. Goodson, NASA TN D-995,
  1962, Figs. 2, 8. Both on NTRS: <https://ntrs.nasa.gov/citations/19930084303>, `/19620000022`.
- **R3.** C. E. Brazzel, J. H. Henderson, J. C. Craft, *Longitudinal Stability Characteristics of a Series of
  Ringtail-Body Combinations at Mach Numbers of 0.8–4.5*, US Army Missile Command RD-TR-65-7, 1970, DTIC AD0871756,
  approved for public release; read through the Internet Archive's copy. pp. 1–4, Table II (pp. 20–26), Figs. 2, 8.
- **R4.** C. W. Dahlke, J. C. Craft, *Experimental Aerodynamic Investigations of Configuration Shape for Ring Tail
  Missiles at Mach Numbers from 0.5 to 1.3*, US Army Missile Command RD-TM-72-5, 1972, DTIC ADA039676, approved for
  public release. p. iii, Fig. 5, plots pp. 111, 127.
- **R5.** MIL-HDBK-762(MI), *Design of Aerodynamically Stabilized Free Rockets*, 1990, p. 5-14.
- **R6.** RocketPy, "Tube Fin Aerodynamics", documentation.
  <https://docs.rocketpy.org/en/develop/technical/aerodynamics/tube_fins.html>. Its documentation says the model
  follows OpenRocket's (GPL), so only the documentation is read.
- **R7, R8, R11.** OpenRocket PRs #1413 ("Update tube fin stability calculations", 2022), #3235 ("Fix tube-fin CP
  interpolation initialization", 2026) and #1312 ("Rewrite tube fin and launch lug drag calculations", 2022). Text only.
- **R9, R10.** Apogee *Peak of Flight*: A. Van Milligan, "Drag of Tube Fins", #347, 2013, pp. 7–8; T. Van Milligan,
  "CFD for Dummies (Like Me)", #689, 2026, pp. 3, 11–12, 18.
- **R12.** F. Hiroshima, K. Tatsumi, "Grid pattern effects on aerodynamic characteristics of grid fins", ICAS 2004
  paper 092. <https://www.icas.org/icas_archive/ICAS2004/PAPERS/092.PDF>. Tables 1–2, Figs. 4, 6–9.
- **R13.** H. S. Fletcher, NACA TN 4117, 1957. <https://ntrs.nasa.gov/citations/19930084906>. pp. 3–4, Fig. 12.
- **R14.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, pp. 3-12, 3-21; S. F. Hoerner, H. V. Borst, *Fluid-Dynamic Lift*,
  1985, pp. 17-10, 19-16.
