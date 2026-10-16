# Wind-tunnel measurements of whole hobby rockets at low speed

HPR Sim's measured references for a whole rocket's drag and normal force are NASA's Arcas Robin tunnel tests, from Mach
0.6 up ([aero](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)). Below that it has Barrowman's worked
examples and code-to-code comparisons. Issue #172 finds HPR Sim's stability margin reading higher than OpenRocket's on
two designs, and no measurement says which is right. This note asks which published low-speed tunnel tests of a whole
rocket (nose, body, fins) give drag, normal-force slope (C_Nα) and center of pressure (CP), with the geometry stated
well enough to fly in HPR Sim. It counts them and gives each one's geometry, Reynolds number and terms of use.

**In short: none of the sources reached gives a tunnel C_Nα or CP for a modern hobby or high-power rocket, and one hobby
report gives usable drag.** NAR's R&D archive is members-only and was not read, so confidence is high for the open
literature and medium overall. Hobby literature from Gregorek to the Apogee newsletters uses flight tests, CFD or
theory. The one whole-model tunnel report, DeMar's 1995 NAR R&D entry, measured drag only. Its reference area doesn't
match its own models. Five government or academic reports are usable for stability below Mach 0.6. The best is a 1955
Cranfield note: a 60 mm rocket with four flat-plate fins in three spans and none, tabulated at Mach 0.12. Its own theory
put the CP 0.34–0.70 calibers *ahead* of the measured one. The best-dimensioned is a 1984 Japanese four-fin rocket at
Mach 0.5, plotted only. None is the shape of #172's designs, so none settles that issue alone, but rebuilding two of
them would give HPR Sim its first measured subsonic stability reference. "Inference" marks this note's own arithmetic.

## 1. What the hobby literature holds

Reached: Estes and Centuri reports, six open NAR R&D reports, eight Apogee *Peak of Flight* issues and nine
university handouts and papers on model rockets. Two hold a usable-looking tunnel measurement of a whole rocket, and
both are drag only.

- **DeMar 1995, NARAM-37 [A1].** Eighteen models, 16 with a C_D, most on a 0.75 in (BT-20) body, in a teaching
  tunnel's 16 in test section at 80, 100 and 120 km/h. Three readings were averaged (PDF p. 5), and the mount's
  tare was measured and subtracted (p. 7).
  - **Conditions.** At 120 km/h the speed is 33.3 m/s, Mach 0.096. DeMar states Re about 800,000 (p. 4).
    **Inference:** Re ≈ 5.4 × 10⁵ on the 10 in length and 4.0 × 10⁴ on the diameter, at ν = 1.56 × 10⁻⁵ m²/s.
    Both are 10–100 times below an HPR flight at Mach 0.3–0.8.
  - **Geometry.** Drawn in multiples of 0.75 in (pp. 14–15): a 2:1 elliptical nose, a 10 in overall length and swept
    fins. The variants cover finish, a 1/8 × 2 in lug, five noses, a boattail and BT-5 and BT-50 scales. Fin count,
    fin thickness and the mount are **not stated**.
  - **Results at 120 km/h (pp. 7–9).** Standard model C_D 0.68. Unfinished 0.77, polished 0.61, with a lug 0.88
    (+29%), 3:1 cone nose 0.80, boattail 0.52. Across the three speeds the standard model reads 0.50, 0.75 and 0.68.
  - **The reference area is too small (Inference).** Back-solving his formula C_D = 1969 F/(V² A) (p. 5) from his
    tabulated forces gives A = 1.94–2.03 cm² for every BT-20 model. A BT-20 is 0.736 in across (DeMar: about 0.75 in),
    so its frontal area is 2.74–2.85 cm², and DeMar says A includes the fins. His BT-5 and BT-50 rows use areas that
    match their tubes. On the BT-20's frontal area the standard model is C_D ≈ 0.48–0.50, not 0.68. Taken at face value,
    his BT-20 numbers make a simulator's drag look 27–30% low.
  - **Two slips (Inference).** The BT-50 model at 120 km/h is 0.60 on its stated area, not the printed 0.73. The
    report's claim that drag doesn't change with model size rests on the 0.73. One egg-lofter row also reads 0.42
    on its stated area, against the printed 0.63.
  - **Use.** It is a check on the *increments* (lug, finish, nose shape, boattail) at very low Reynolds number. It is
    not an absolute reference, since the standard model's speed scatter alone is about ±20%. The flight check it planned
    "DID NOT HAPPEN" (p. 11). © 1996 J. S. DeMar, no license: cite only.
- **Fischer et al. 2020, a 1 L water rocket [A2].** Drag at 9.6–20 m/s in a university tunnel. C_D = 0.6 ± 0.2, Re_L ≈
  2.2–4.7 × 10⁵ (**Inference**). Under arXiv's non-exclusive license: cite only. The geometry is partial: an 8 cm bottle
  0.35 m long with a half-tennis-ball nose and three 7.5 cm-root fins (PDF pp. 21–22). The error bar is too wide to tell
  drag models apart.
- **Tested, but no whole-rocket reference.** Two 2013 NARAM reports from the US Air Force Academy tunnel [A3, A4]:
  - Tube fins: not repeatable. The finless body's force rose 3.24 times where speed squared predicts 1.53.
  - Nose cones: on a body tube with no fins, and with no tare or air density.
  - A 2022 *Peak of Flight* [A5] shows a lower section only, as a plotted trend.
- **No tunnel data at all.**
  - Gregorek's TR-11 [A6] flew twelve Alphas instead, and TIR-33 [A7] is theory.
  - Barrowman's NARAM-8 [A8] measured CP by flight: shifting a test rocket's CG, it got 16.5 in against 16.7 in
    calculated (PDF pp. 43–45). That is a flight neutral point, not a tunnel value.
  - McAtee's NARAM-40 report [A9] reuses the Arcas data HPR Sim already has.
  - Apogee's launch-lug study [A10] is CFD.
  - NASA Glenn says it tested rocket models to 50 mph for RocketModeler III [A11], but published neither the data nor
    the geometry.

## 2. Government and academic tests below Mach 0.6

Screened: 17 NASA, NACA, Cranfield, JAXA, journal and student sources, plus Zenodo and figshare. Searches turned up no
open dataset of a finned rocket's coefficients. Five are usable for C_Nα and CP; a sixth only with outside geometry.

| Source | Shape | Mach | Re (length) | Measured | Geometry |
|---|---|---|---|---|---|
| Cranfield N29, 1955 [B1] | 60 mm, 4 plate fins, 3 spans + none | 0.12 | 1.4 × 10⁶ | C_L, C_D, C_m vs α | nose drawn |
| NAL TR-814, 1984 [B2] | 1/10 TT-500A, cone nose, 4 fins | 0.50–0.95 | 7.9–11 × 10⁶ | C_N at 4°, 8°; CP at 8° | full |
| TN D-1786, 1963 [B3] | Trailblazer II, 3 fin sets | ≈ 0.2 | 2.9–3.8 × 10⁷ | C_N, C_A, C_m to 28.5° | 3 stages |
| TN D-2163, 1964 [B4] | two stages, odd nose | 0.22 | Re_d 4.0 × 10⁵ | C_N, C_A, C_m | not hobby |
| RM L53J06, 1954 [B5] | 4 delta fins, boattail | 0.50–0.88 | 1.8 × 10⁷ | C_N, C_A, C_m | full |
| Sonda III, 2011 [B6] | two stages, 1:20 | 0.3–1.0 | not stated | C_D0, C_L, C_m | none |

All but B1 are plotted only. Reynolds numbers are on body length unless marked. B5's is stated; the others are
**Inference** from the stated speed or per-foot value and the model's length.

- **B1, Cranfield Technical Note 29 [B1].** The best for stability: the only tabulated set, with fin span varied.
  - **Model.** A full-size 2.35 in (59.7 mm) rocket, 19.81 in long, on a pivot strut at its CG, in an open-jet
    tunnel at 132 ft/s (PDF pp. 2, 5–6).
  - **Fins.** Four 0.048 in plates, square-edged, root 3.25 in with a 45° leading edge. Spans 1.7, 1.2 and 0.83 in,
    and none (Fig. 2).
  - **Data.** Table I (PDF p. 12) gives C_L, C_D and C_m at 11 angles, −13° to +17° in 3° steps, at both fin rolls.
    Stated accuracy: C_L ±0.03, C_D ±0.02, C_m ±0.002 (p. 7). Strut drag was removed; no wall correction.
  - **The geometry gap.** The nose and body are drawn at 1/4 scale, not dimensioned. A rebuild must scale the drawing.
  - **Results (p. 8).** Measured lift slopes 8.37, 6.60, 4.87 and 2.01 per radian, against the report's slender-body and
    lifting-surface theory of 8.09, 6.28, 4.41 and 2.0. CP aft of the pivot measured 3.67, 2.40, 0.50 and −7.89 in,
    against theory's 2.87, 1.44, −1.15 and −8.95.
  - **What the sign says.** On every configuration the theory placed the CP 0.8–1.65 in (0.34–0.70 calibers) ahead
    of the measured one (**Inference** from that table). So a slender-body estimate errs toward a *smaller* margin,
    the safe side.
  - **Terms.** No license on the repository record: cite only. The fin-on and fin-off drag is already used in
    [junction-drag](junction-drag.md).
- **B2, NAL TR-814 [B2], in Japanese.** The best-dimensioned four-fin rocket.
  - **Model.** A 1/10-scale TT-500A: 50.3 mm body, 1,050 mm long, 30° cone nose. Configuration 4 has the tail fins
    only: four trapezoids, root 70 mm, tip 43 mm, span 68.6 mm (Fig. 1, PDF p. 4).
  - **Tunnel.** The NAL 2 m transonic tunnel with an internal six-component balance. Mach 0.50 is at 7.5 × 10⁶ per
    meter (Table 2, PDF p. 5).
  - **Data.** Normal force at 4° and 8° (Fig. 5) and CP at 8° (Fig. 8), plotted against Mach, with no tables. The
    pages read state no wall corrections or uncertainty.
  - **Reading.** By eye at Mach 0.5 and 8°, C_N ≈ 2.45 ± 0.1 at 45° roll and about 2.7 at 90° (**Inference**).
    At 45°, C_N/α is about 17.5 per radian; that is a secant at 8°, not the slope at zero.
  - **Overlap.** Its range overlaps the Arcas fixture from Mach 0.6 up, so it would tie the new low-speed point to the
    existing one. JAXA states no license: cite only.
- **B3, NASA TN D-1786 [B3].** A Langley low-turbulence tunnel at Mach 0.2 with a grit trip and wall and blockage
  corrections. C_N ±0.1 (PDF pp. 5–6).
  - **Fin sets.** Three, given as full-scale panel areas: 12 ft² at aspect ratio 1.5, 10 ft² at 1.5, and 10 ft² at
    0.985. The text states their aerodynamic centers, 39, 38.10 and 36.95 in from the nose (PDF p. 6). The fins are
    modified double wedges with an 18°24′ leading-edge sweep.
  - **Use.** Strap-on motors and a wider payload make it no hobby shape. Its value is the *change* in CP with fin
    area and aspect ratio. NASA work: public use.
- **B4 and B5** are coarse (B4: C_N ±0.2) or only partly hobby-like (B5: delta fins, a boattail, tested in Freon-12).
  Both are NASA/NACA public use.
- **B6** gives no model dimensions, so it can't be flown without another source.
- **Not reached.** These may hold better data:
  - RAE TN Aero 2811 (1962): a 0.7-scale 6 in rocket at Mach 0.4–1.24. DTIC was down on 2026-10-16.
  - The Basic Finner's low-subsonic static data, also on DTIC.
  - The ILR-33 tunnel paper (2017), behind a bot check.
- **Too far off to use.** A 2018 Modified Basic Finner test starts at Mach 0.6 [B7], so it adds nothing below the
  Arcas data. A 2023 drag test of a 27 mm model ran at Re_d of only 5–11 × 10³ [B8].

## 3. The count

| Use | Sources | Best |
|---|---|---|
| C_Nα and CP below Mach 0.6, whole finned rocket | 5 (B1–B5), plus B6 with outside geometry | B1 |
| Drag of a whole finned rocket below Mach 0.3 | 4 (A1, B1, B3, B4); A2 and B8 too coarse | B1 shape, B3 (±0.01) |
| C_Nα or CP of a modern hobby or HPR rocket | 0 | none |
| Public-use terms | 3 (B3, B4, B5) | NASA and NACA |

Every hobby source is copyrighted with no open license (A2 is under arXiv's), and so are B1 and B2: cite them, cache
them privately, never commit their tables. A fixture would hold the project's own readings with page and figure, as the
Arcas fixture does.

## What this suggests

1. **Build B1 and B2 in HPR Sim and compare C_Nα and CP** at their Mach and Reynolds number. For B1, scale the nose
   from Fig. 1 and state the reading error. For B2, read Figs. 5 and 8 with the project's pixel method.
   - **Effect.** The first measured subsonic stability reference. B1 alone has four fin spans, and its theory's CP
     error had a known sign.
   - **On #172.** If HPR Sim's CP sits aft of B1's measured CP, the high margin is a model trait, not an OpenRocket
     quirk (**Inference**).
2. **Add B3's three CP values as a fin-sensitivity check.** They are public-use and stated in the text, so no plot
   reading is needed. Expected effect: a pinned sign for how CP moves with fin area at Mach 0.2.
3. **Use DeMar only for increments, rescaled to the BT-20's area,** and say so where it is cited. Effect: a low-Re
   sanity check on lug, finish and nose increments. It can't be a validation case, since the fin count is unknown.
4. **Retry DTIC** for RAE TN Aero 2811 and the Basic Finner's subsonic data. Either could beat B2 at Mach 0.4–0.6.

## Open questions

- **Hagerty, "Lift vs Drag of 2-Stage Rocket in Wind Tunnel" (NARAM-52, 2010).** Does it give normal force against
  angle on a dimensioned whole rocket? It is the hobby report most likely to, but the NAR archive's reports need a
  member login (HTTP 403 on 2026-10-16). About 18 other titles with "drag" in the name are unread, several on recovery.
- ***Topics in Advanced Model Rocketry*** (MIT Press 1973, on MIT DSpace): does it hold tunnel data on model rockets?
  Scripted downloads were refused.
- **Why DeMar used 2.0 cm² for a BT-20** is unknown.
- **B1's nose profile** comes only from a drawing. How far a 1–2 mm reading error moves body lift and CP is not
  bounded here.
- **B2's wall corrections and uncertainty** were not found in the pages read.
- **No measurement at hobby Reynolds numbers with fillets, airfoiled fins or a painted finish.** B1's plate fins and
  B2's bevelled fins are the nearest.

## Sources

All read 2026-10-16. Hobby sources are copyrighted with no open license unless noted.

- **A1.** J. S. DeMar, "Model Rocket Drag Analysis using a Computerized Wind Tunnel", NAR R&D, NARAM-37, 1995.
  <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
- **A2.** L. Fischer et al., "On the approximation of D.I.Y. water rocket dynamics including air drag",
  arXiv:2001.08828v2, 2020. <https://arxiv.org/abs/2001.08828>
- **A3.** Allison Van Milligan, "Drag on Tube Fins", NARAM-55, 2013.
  <https://www.apogeerockets.com/downloads/Tube_Fin_Drag.pdf>
- **A4.** Ashley Van Milligan, "Drag of Nose Cones", NARAM-55, 2013.
  <https://www.apogeerockets.com/downloads/Drag_of_Nose_Cones.pdf>
- **A5.** *Peak of Flight* 574, 2022. <https://www.apogeerockets.com/education/downloads/Newsletter574.pdf>
- **A6.** G. M. Gregorek, Estes TR-11, 1970. <https://www.plans.rocketshoppe.com/pubs/Estes/estTR-11/TR-11.pdf>
- **A7.** Centuri TIR-33, "Calculating the Center of Pressure of a Model Rocket". Cite only, never redistribute.
- **A8.** J. and J. Barrowman, "Stability of a Model Rocket in Flight", NARAM-8 R&D, 1966.
- **A9.** T. McAtee, NARAM-40, 1998. <https://www.apogeerockets.com/downloads/PDFs/mcatee.pdf>
- **A10.** T. Van Milligan, launch-lug drag, NARCON 2017.
  <https://www.apogeerockets.com/downloads/PDFs/launch-lug-drag.pdf>
- **A11.** NASA Glenn, "Drag Measurement".
  <https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/guide-to-rockets/drag-measurement-2>
- **B1.** K. D. Harris, G. G. Appleby, College of Aeronautics Technical Note 29, Cranfield, 1955.
  <https://dspace.lib.cranfield.ac.uk/handle/1826/11132>
- **B2.** M. Shirouzu et al., NAL TR-814, 1984. <https://jaxa.repo.nii.ac.jp/records/45072>
- **B3.** C. A. Brown Jr., L. E. Putnam, NASA TN D-1786, 1963. <https://ntrs.nasa.gov/citations/19630005431>
- **B4.** J. T. Suttles, NASA TN D-2163, 1964. <https://ntrs.nasa.gov/citations/19650000798>
- **B5.** D. L. Burrows, E. E. Newman, NACA RM L53J06, 1954. <https://ntrs.nasa.gov/citations/19930093747>
- **B6.** J. B. P. Falcão Filho et al., J. Aerosp. Technol. Manag. 3(3):311–324, 2011.
  <https://www.redalyc.org/pdf/3094/309426261009.pdf>
- **B7.** H. Belaidouni et al., Tehnički vjesnik 25(2):445–452, 2018. <https://hrcak.srce.hr/199142>
- **B8.** L. M. C. dos Santos, bachelor's thesis, UFRN, 2023 (CC BY-ND 3.0 BR).
  <https://repositorio.ufrn.br/items/1d00de7f-44d8-42a0-b787-2f14731c3433/full>
- NAR R&D reports, chronological index. <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
