# Published one-sigma inputs for rocket Monte Carlo, and which were measured

**In short: nearly every published input sigma is assumed; the widest published mass spread is about a third of what
the project measured for a design file, and hobby drag spreads run from a quarter of its figure to above it.** This
note gathers the one-sigma (1σ) spreads that hobby tools, student teams and sounding-rocket ranges put on drag, dry
mass, impulse, burn time, launch angle, rail, wind and air, and asks which came from data. It feeds
[M2.8](../ROADMAP.md)'s dispersion runs, whose 50% and 90% apogee ranges are scored against logs, and issue #401 (no
spread on one flight's readout), extending the priors in [drag–impulse
identifiability](drag-impulse-identifiability.md) §5. Read 2026-10-17. **Inference** marks this note's arithmetic,
including each 1σ taken from a stated 2σ or 3σ (normal assumed). Few coverage checks exist.

- **Measured inputs are rare.** Of 9 hobby and academic sources here that print input sigmas, one takes a value
  from repeat firings (MAPLEAF's motor spread, from a NAR test sheet [3, 4]); wind is the exception, drawn from a
  measured forecast error or a weather ensemble in five [1, 5, 6, 11, 13]. Four of five professional tables are
  labelled estimates or "assumed" [14–16, 18]; the fifth gives no basis [17]. The professional figures from data are
  SP-8039's typical motor scatter (total impulse 0.3%, action time 2.0%) [19] and angles after wind weighting [20].
- **The values cluster loosely.** Hobby: launch tilt 1° (5 of 7 sources), drag 3.8–20%, dry mass 0–3.9%, impulse
  0.9–2.5% (one example 15%), burn time 1.5–19%. Professional 1σ: impulse 0.67–2.5%, mass 0.33% (weighed), drag
  1–6.7%, launcher elevation 0.03–0.33°, wind left after wind weighting 0.67–1 m/s.
- **The project's own evidence is wider for a design file.** Finished rockets weigh a median 7.4% over their file,
  spread near σ 11%; coast-fit drag scatters near σ 16%; impulse 2.5–3.1% per certification test, with lots
  inferred 0 to −4% low. No published dispersion models a mean shift for mass or impulse.
- **Coverage is barely tested** (§8): one hobby apogee check; professional impact sigmas cover, apogee means miss.

## 1. Inputs side by side

1σ values. M = measured from tests, weighings or flight data; A = assumed or copied. Source numbers in brackets.

| Input | Hobby and academic | Professional | Project's evidence |
|---|---|---|---|
| Drag scale | 3.8% [5], 7% [10], 10% [8], 20% [1, 2]; A | 1% [18], 3.3% [15], 5% [14], 6.7% [16, 17]; A | σ ≈ 16% |
| Dry mass | 0–3.9%, zero mean [5–10]; A | 0.33% [16, 17], 5 lb [14], 1.7 lb [15]; A | +7.4%, σ ≈ 11% |
| Total impulse | 2.33% M [3]; 0.9–2.5% A [5, 6, 10] | 0.67–2.5% A [14–18]; 0.3% M [19] | 2.5–3.1% |
| Burn time | 10.7% M [3]; 1.5–19% A [5, 6, 9, 10] | 2.0% M [19] | not measured |
| Launch tilt | 1° in 5 of 7 [2, 5, 6, 8, 9]; 1.5°, 2°; A | 0.03–0.33° setting [14–18]; A | — |
| Azimuth | 1–5°, or uniform [2, 5, 7, 8, 10]; A | 0.25–1.33° [14, 16, 17]; A | — |
| Wind | ensembles, forecast error, M [1, 5, 6, 11, 13]; offsets A [7, 8] | 0.67–1 m/s left [14–17]; A | — |
| Temperature, air | 2.2–6 K; pressure ~3%; density 2%; A [7, 8, 10] | density 1% [18]; A | ERA5 ±0.5 K |
| Measurement | not added by any source | — | ~0.4% of apogee per logger |

## 2. Motor: impulse and burn time

- **Measured scatter is tight in impulse, loose in time.** SP-8039's Table III gives typical full-scale 1σ, estimated
  from demonstration firings: total impulse 0.3%, action time 2.0%, average thrust 2.1% [19] p. 53; no motor or count
  is named. A NAR sheet for a small black-powder motor gives impulse σ 2.33% and burn time σ 10.7% over its batch [4]
  p. 1, and MAPLEAF uses exactly those [3]. **Inference:** impulse is propellant mass times specific impulse, both
  held tight, while burn time follows burn rate; scatter time several times more than impulse.
- **Assumed values.** Hobby: 2.5% (Valetudo, a team-built motor) [5], 2% ("typical lot-to-lot", unsourced) [10], 0.88%
  [6], 1% thrust scale [2, 9], 5% thrust scale [7, 8]. Professional: 2.5% (Nike-Apache) [14], 0.67% (Black Brant VC)
  [15], 1% (VSB-30, SHEFEX II) [16, 17], 2% (Army free rockets) [18].
- **No source models a mean shift.** RocketPy's class example centers impulse 7.9% and mass 6.9% above its rocket's,
  impulse σ 15% of its mean [6]: a setup slip, not a bias model (**Inference**). The NAR sheet's batch mean sits 11.5%
  under its certified impulse [4] (**Inference**), like the small-motor retests in [motor data](motor-data.md) §5.
- **Against the project.** TMT's repeat certifications put one test's scatter at 2.5–3.1%; H-and-up lots are inferred
  0 to −4% low, −9% not ruled out ([lot impulse](hpr-lot-impulse.md)). **Inference:** hobby 0.9–2.5% is a little
  narrow; SP-8039's 0.3% and Black Brant VC's 0.67% are far too narrow for COTS hobby motors.

## 3. Dry mass

- **Hobby sources disperse mass around the file's value, 0–3.9%:** 0.012% [5], 0.5% then 0% in the FRR [7], 1.5% [10],
  2% with inertia [8], 3.2–3.5% [6], 3.9% [9]. Box 2011 and the Cambridge simulator leave mass fixed [1, 2].
  RocketPy's sensitivity guide finds mass carries about 71% of apogee variance at its 3.5% [6].
- **Professional vehicles are weighed:** 0.33% [16, 17], 5 lb on Nike-Apache [14], 1.7 lb on Black Brant VC [15].
  Nike-Apache's handbook says apogee holds "within ±5 miles" if payload weight is known to 5 lb [14] p. 54.
- **Against the project.** Finished rockets weigh a median +7.4% more than their file, interquartile range −1.4% to
  +13.1%, a σ near 11% (**Inference**, IQR ÷ 1.349) ([as-built mass growth](as-built-mass-growth.md)). **Inference:**
  0.5% around a weighed mass is fair; 1–4% around a file mass is too narrow.

## 4. Drag

- **Hobby:** 20% on axial force in Box 2011 ("expert guess") [1] p. 27 and the Cambridge simulator [2] Table 4; 10% in
  OpenRocket's new Monte Carlo (merged 2026-09-17, called "practical starting values, not as validated uncertainty
  data") [8]; 7% in a 2026 Student Launch template [10]; 3.8% on Valetudo (σ 0.033 on a 0.865 scale), whose comment
  says drag "usually" carries 5–10% [5]. Several leave drag fixed [6, 7, 9].
- **Professional:** 3σ 10% on Black Brant VC (1σ 3.3%) [15], 2σ 10% on Nike-Apache (5%) [14], 3σ 20% on VSB-30 and
  SHEFEX II (6.7%) [16, 17]; the Army budget's ballistic coefficient 1% [18]. Drag adds little to impact spread (0.73
  of 38 statute miles) [15].
- **Against the project.** Coast fits against hobby codes give quartiles 0.89–1.11 of the code's C_D, about σ 16%
  (**Inference**) ([flight-derived C_D](flight-derived-cd.md) §2). The 10–20% hobby defaults match it; the 1–7%
  values, hobby or professional, describe a vehicle whose drag was already fitted to flights.

## 5. Launch angle and rail

- **Tilt from vertical, σ 1°,** in RocketPy's notebooks [5, 6], Cambridge [2], an SA Cup team [9] and OpenRocket [8];
  1.5° [10] and 2° [7] elsewhere. RocketPy's class example keeps Valetudo's angle sigmas for a different flight [6]: a
  copied default. Azimuth runs 1–5° or uniform over a full turn [2, 5, 7, 8, 10].
- **Professional launchers are set, not aimed by eye:** elevation 0.03–0.33° and azimuth 0.25–1.33° [14–18].
- **Rail:** 0.5 mm on rail length in one source [5]; no source disperses rail-exit speed with a number. Black Brant VC
  dropped tower tip-off "since none has been experienced on the first five flights" [15] p. 3, the one input removed
  on evidence. Nike-Apache carried 2σ tip-off 5 °/s, its largest term in range and cross-range together [14].

## 6. Wind and air

- **Wind is the one hobby input often drawn from data.** Box 2011 samples five years of forecast-minus-measured wind
  profiles [1] §4.2.2; Valetudo, RocketPy's example and a GTXR flight pick weather-ensemble members [5, 6, 13]; an
  IREC team measured forecast drift error on 2,900 sonde tracks (250 m at 3 min at best) [11] p. 2. Others add: 1 m/s
  and 10° [8]; 2–5 mph and 2–20° [7].
- **Professional winds are measured on the day and compensated** by wind weighting; the input is what is left: 1σ 0.76
  m/s [14, 15], 0.67 m/s [16], 1 m/s [17]. The Army's indirect-fire budget uses 1.19 m/s during burn and 2.47 m/s
  ballistic wind, not called residuals [18]. On 12 Trailblazer flights the flight-path angle at 83,000 ft still missed
  by 0.88° RMS in elevation and 3.85° RMS in azimuth after compensation [20] Table VII (**Inference** from the table).
- **Air.** Hobby: temperature 2.2 K [7] and 6 K [10], pressure 3.4 kPa [7] or 900 Pa [10], density 2% [8]; the Army
  handbook uses density 1% [18] p. 7-28. All assumed. The scored logs fly ERA5's air, which misses by about ±0.5 K
  ([ERA5 surface temperature](era5-surface-temperature.md)); a forecast-style 2–6 K is too wide for them.

## 7. Recovery and others

Parachute C_D or CdS: 10% [1, 2, 10], 15% [5, 8], 1% main and 7% drogue [6]. Deployment delay 0.1–0.5 s [5, 6, 8,
10]. Fin cant 0.5° [1]. Professional thrust misalignment 0.03–0.07°, fin misalignment 0.003–0.07° [14–17]; on
Black Brant VC, wind and thrust misalignment make 37.6 of its 38 statute-mile radius [15] p. 3. No source measured
a misalignment distribution. Every source draws inputs independently; none models a correlation [6, 8]. FAA
guidance for unguided launches asks each σ's derivation to be stated [23] p. 11.

## 8. Coverage: did the flight land inside the predicted spread?

| Flight | Check | Result | Source |
|---|---|---|---|
| Valetudo, 20,000 runs | apogee | 860 m flown at the 62nd percentile (SD 5.0%): inside 50% | [5] (**Inference**) |
| Valetudo | landing | Mahalanobis 2.49: outside the 90% region | [5] (**Inference**) |
| Box's L-motor flight, 500 runs | landing | inside the 1σ ellipse; point apogee −1.5% | [1] p. 28 |
| NC State, four flights | landing | two within 2σ, one within 3σ, one 26 ft from the mean | [7] |
| Black Brant III, 13 flights | impact | miss never above 11 km against a 48 km 3σ radius | [21] p. 3 |
| Black Brant IV, VA, VB, 11 flights | apogee | "within ±5% of theory"; no band stated | [21] p. 3 |
| VSB-30, 10 flights | impact | all inside 3σ, "most" inside 2σ | [16] p. 329 |
| VSB-30, 10 flights | apogee | mean −2.55%, SD 3.50%, 7 of 10 low (**Inference** from Table 3) | [16] p. 329 |
| Nike-Apache, 16 flights | apogee | re-flown with real payloads: −9.8 to +11.4 km, 12 of 16 high | [14] p. 105 |
| IRVE-II, 1 flight | trajectory | altitude within 2σ; dynamic pressure outside 2σ, inside 3σ | [22] pp. 1, 7 |

- No source reports a coverage rate over many flights. Six landings can't test calibration: a 2-D normal holds 39%
  inside 1σ and 86% inside 2σ (**Inference**).
- **The tool can move the spread sixfold.** NC State's 2,000-run dispersion with one input table gave an apogee SD of
  344 ft in OpenRocket and 56.7 ft in RocketPy [7] FRR p. 38; its CDR's run had them the other way (11.3, 68.4 ft).
- **An indirect check (Inference).** RocketPy's docs list 16 point predictions against measured apogees, errors SD
  5.6% [12]. Bands from Valetudo's 5.0% SD (±3.4% for 50%, ±8.2% for 90%) would hold 5 or 6 (one on the edge) and 14.
- **Professional ranges are built for impact safety.** Their impact circles cover with room; their apogee means
  miss by a few percent, low on VSB-30 and high on Nike-Apache. **Inference:** a spread doesn't fix a biased center;
  M2.8's report needs the median miss beside each coverage rate.

## What this suggests

1. **M2.8: key priors to what the log's file knows** (**Inference**, expected to widen the 90% band and move its
   center): mass σ 0.5% if weighed, else the as-built −1.4% to +13.1% spread (σ about 11%; that note calls it
   two-sided, not a correction); drag σ 10–16%; impulse σ 3% around a scale of 0.97–1.00 for H and up; burn time
   scattered apart from impulse; tilt 1°; ERA5 air ±0.5 K.
2. **M2.8: add the logger's own scatter, about 0.4% per unit, to the predicted range** before scoring; no
   published dispersion does, and a log is a measurement.
3. **#401: don't adopt professional sigmas.** Weighed vehicles, factory motors and wind weighting make them
   two to ten times narrower than a hobby file supports; bands from them would under-cover.
4. **A reproducible check:** RocketPaper's 20,000-run Valetudo outputs [5] let a test recompute §8's two figures.

## What the project can use

MIT, reusable with attribution: [3, 5, 6, 10]. US Government works or public use, cached under refs/: [14, 15,
18–20, 22, 23]. CC BY 4.0: [13, 16]. Cite only: [1, 2, 4, 7–9, 11, 17, 21].

## Open questions

- RocketPy's journal paper [24] is closed access; its Monte Carlo table may be the Valetudo notebook [5].
- No measured spread for rail-exit speed, misalignment, grain temperature or a hobby motor's burn time at H and up.
- Not reached: NASA TN D-2142, DTIC AD0680177, Sandia SC-DR-69-291 (field wind weighting); FAA AC 450.117-1.
- How often a hobby apogee falls inside a published-sigma band over many flights is unmeasured (M2.8's count).

## Sources (all read 2026-10-17)

1. Box, Bishop, Hunt, J. Aerosp. Eng. 24(1) 31–45, 2011, draft: https://eprints.soton.ac.uk/73938/ (Table 7 p. 27).
2. Eerland, Box, Fangohr, Sóbester, Southampton eprint 404214: https://eprints.soton.ac.uk/404214/ (Table 4 p. 5).
3. MAPLEAF (MIT), `MAPLEAF/Examples/Simulations/Jake1_MonteCarlo.mapleaf`: https://github.com/henrystoldt/MAPLEAF
4. NAR Standards and Testing, Estes B4 sheet (1995, rev. 8/96):
   http://web.archive.org/web/20241121023719/http://nar.org/SandT/pdf/Estes/B4.pdf
5. RocketPaper (MIT), commit 23d9271db533, Valetudo notebook and outputs: https://github.com/RocketPy-Team/RocketPaper
6. RocketPy (MIT), commit 9bd6ad3af8f9, Monte Carlo notebooks and docs: https://github.com/RocketPy-Team/RocketPy
7. NC State Rocketry, NASA Student Launch 2023 FRR p. 53; 2026 CDR p. 44, FRR pp. 38, 77–78, addendum, PLAR:
   https://ncsurocketry.org/
8. OpenRocket PR #3260 description and dispersion docs: https://github.com/openrocket/openrocket/pull/3260,
   https://openrocket.readthedocs.io/en/latest/dev_guide/landing_dispersion_analysis.html
9. ITA, Team 73 Project Technical Report, Spaceport America Cup 2018, Table 1 p. 5:
   https://www.soundingrocket.org/uploads/9/0/6/4/9064598/73_project_report.pdf
10. nasa-sli-sim (MIT, 2026), `config/uncertainty.yaml`: https://github.com/DrJayBee51/nasa-sli-sim
11. RIT Team 144, 2025 IREC podium abstracts pp. 1–2, ESRA:
    https://www.soundingrocket.org/uploads/9/0/6/4/9064598/irec2025_podium_abstracts__1_.pdf
12. RocketPy docs, `docs/examples/index.rst` (16 apogee pairs), commit as [6].
13. Garud et al., "Material Girl Launch Report", arXiv:2411.00807, p. 6 (CC BY 4.0).
14. Crough, Galloway, NASA X-721-66-569, 1966, pp. 54, 59, 105: https://ntrs.nasa.gov/citations/19670015760
15. Nolan, Celmer, NASA TM-X-70474, 1973, Table 3 p. 3: https://ntrs.nasa.gov/citations/19730024047
16. Garcia et al., "VSB-30 sounding rocket: history of flight performance", JATM 3(3) 325–330, 2011, Tables 3–4
    p. 329, doi:10.5028/jatm.2011.03032211
17. Scheuerpflug, Kallenbach, Cremaschi, "Sounding Rocket Dispersion Reduction by Second Stage Pointing Control",
    J. Spacecraft Rockets 49(6), 2012, Table 1 p. 1160, doi:10.2514/1.A32193
18. US Army Materiel Command, AMCP 706-280, Design of Aerodynamically Stabilized Free Rockets, 1968, Table 7-1
    p. 7-28: https://everyspec.com/ARMY/ARMY-General/AMCP_706-280_JUL1968_50859/
19. NASA SP-8039, 1971, Table III p. 53: https://ntrs.nasa.gov/citations/19720011135
20. Henning, Lundstrom, Keating, A Wind-Compensation Method … Twelve Trailblazer Rocket Vehicles, NASA TN D-2053,
    1964, Table VII p. 26: https://ntrs.nasa.gov/citations/19640008163
21. Sevier, "Fin Structural Design for High Performance Sounding Rockets", AIAA 1967-1342, p. 3:
    https://wikis.mit.edu/confluence/download/attachments/129020679/FinDesign%20Paper.pdf
22. O'Keefe, Bose, IRVE-II Post-Flight Trajectory Reconstruction, AIAA 2010: https://ntrs.nasa.gov/citations/20100031103
23. FAA AST, Supplemental Application Guidance for Unguided Suborbital Launch Vehicles, 2001, Attachment 1:
    https://www.faa.gov/about/office_org/headquarters_offices/ast/media/2whi96c2.doc.pdf
24. Ceotto et al., "RocketPy: Six degree-of-freedom rocket trajectory simulator", J. Aerosp. Eng. 34(6), 2021,
    doi:10.1061/(ASCE)AS.1943-5525.0001331 (not reached).
