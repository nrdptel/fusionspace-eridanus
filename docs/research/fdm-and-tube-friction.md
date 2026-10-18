# What friction FDM prints and paper tubes measure, as equivalent sand grain

[hobby-finish-roughness](hobby-finish-roughness.md) found Ra on hobby finishes but no k_s, and converted at 2 to 6 Ra.
This note looks for flow measurements on the two finishes that conversion leaves widest: bare FDM prints (11 to 197
µm) and bare spiral-wound paper or phenolic tube. **In short: no published pipe, channel, plate or tunnel test
measures k_s on either.** The two printed-pipe tests measured their fittings, not their walls. But a printed tube's
layer lines are regular transverse ridges, and bar data, extrapolated, give k_s ≈ 0.3 to 1.2 layer heights (2 to 29
Ra, 30 to 360 µm for 0.1 to 0.3 mm layers), or near 0.1 h on a gap-width law (**Inference**). Across the flow (an
upright tube) ridges carry about 2 to 3 times the roughness function they do along it (a flat-printed fin). Bare kraft
is converted only, at k_s of about 8 to 26 µm or more; glassine and coated faces about 1 to 5 µm. **For the +8.73% bias
the two differ:** a coarse bare print left at the 20 µm default carries more friction than modeled (the bias's sign);
a bare paper tube no more. How far to trust this: every k_s is a conversion or an extrapolation, 4 to 10 times wide.

## 1. What has been measured: friction on neither surface

**Printed pipes.** Harmon et al. 2026 [1] printed PLA pipes upright (25.4 mm bore, 900 mm long) at 0.1, 0.3 and 0.6
mm layers and ran water at Re ≈ 6,300.
- **Measured:** a depth gauge read ridge depths of about 70, 140 and 210 µm, or 0.70, 0.47 and 0.35 of the layer
  height (arithmetic): coarser layers leave shallower ridges for their height [1] §4.2, §4.6.
- **Measured:** the total drop was 503 to 505 Pa on all three. The paper estimates straight-pipe friction at 35.4 to
  39.4 Pa from smooth correlations and puts the rest on fittings with an assumed ΣK ≈ 15 [1] §4.7. Wall friction is
  then 7.0% to 7.8% of the signal (arithmetic), against a ±5% flowmeter, so its "no measurable change" is not a
  measurement of wall friction (**Inference**).
- The null is expected anyway: at u_τ ≈ 0.017 m/s the ridges sit at k+ ≈ 1.2 to 3.5, hydraulically smooth (arithmetic).

Santos et al. 2019 [2] printed ABS pipes upright (18.5 mm bore) at 0.1, 0.2 and 0.3 mm and fitted ε by Colebrook.
- **Measured:** ε came out 0.95 to 7.42 mm at 0.1 mm layers, which is 9.5 to 74 layer heights [2] Table 3. Several
  0.3 mm values are negative, and the authors suspect losses at fittings near the taps (p. 7). Unusable as k_s.
- **Measured, flat plates:** Ra across the layers 13.4, 10.3 and 6.5 µm, along them 3.5, 1.1 and 1.1 µm, at 0.1, 0.2
  and 0.3 mm [2] Table 1. Across reads 3.9 to 9.1 times along (arithmetic).

**Printed tunnel models.** NASA TP-1998-208396 [3] flew FDM ABS models in 0.254 mm layers at Mach 0.3 to 4.96 against
metal ones. An FDM precursor read axial force "slightly lower", partly a tare artifact (p. 16); the printed models
differed most at Mach 0.3 to 1.2 (p. 23); grit on the metal model, to mimic the printed finish, moved axial force most
at low Mach (p. 36). With no roughness measured, no increment can be read. A PLA airfoil at Re 0.8 to 2.2 × 10⁵ [4]
gave no consistent drag sign with finish (p. 937). No FDM rocket or body-of-revolution drag test was found.

**Paper, glassine, phenolic.** Searches (paper ducts, straws, fiber pipe, phenolic tube, rocket finish tests) found no
flow measurement over a paper or paper-phenolic wall. Near analogs don't transfer: spiral-seam duct (0.088 to 0.116 mm,
seams and joints lumped [5]) and ASHRAE's "phenolic duct", 22 mm foil-faced board whose flanges dominate [6].

**Model rockets.** DeMar's 1995 BT-20 tunnel series [7] stays the one direct test (summarized in
hobby-finish-roughness §5): at 120 km/h, unfinished kraft with bare balsa fins read +13% over primer and paint. At
DeMar's Re (5.7 × 10⁵ by arithmetic, about 8 × 10⁵ as he states, §1.2) even k_s = 26 µm sits at k_s+ ≈ 2.9 (§5
table), hydraulically smooth. So that +13% is not sand-grain friction of the kraft. Balsa grain, open seams, transition
tripped early or scatter (gaps of 2 to 3 counts) carry it (**Inference**).

## 2. Layer ridges as two-dimensional roughness

A wall printed upright repeats its profile at the layer height h. Peak-to-valley k ≈ Rz ≈ 4.4 Ra and Ra/h = 0.04 to
0.16 (hobby-finish-roughness §3) give k/h = 0.18 to 0.70; Harmon's measured 0.35 to 0.70 falls inside. So the frontal
solidity Λ = k/λ is 0.18 to 0.70, and the effective slope ES ≈ 2k/λ is 0.35 to 1.4 (arithmetic).

- **Not the wavy regime.** Below ES ≈ 0.35 the roughness function stops scaling with height [8] p. 101305-11. FDM
  side walls sit at or above that edge, so k_s ∝ k is the starting point (**Inference**).
- **Peak of two-dimensional drag at a pitch of 8 heights (Λ ≈ 0.125).** Square transverse bars there gave k_s/k = 13.6,
  against 3.3 for a 3D mesh (**Measured**, [9] p. 86). Smaller bars in the same group read about 8.8 (arithmetic from
  [8] Table I). Rods tested at pitches of 6 to 24 heights also peaked at 8 [10] (abstract).
- **Denser, k_s/k falls.** For very deep bars (Λ ≥ 1.5), k_s ≈ 0.21 times the gap width whatever the depth, so
  k_s/k = 0.11/Λ [11] pp. 21, 26 (DNS). A log line from the peak (13.6 at Λ = 0.125) to that anchor (0.073 at Λ = 1.5)
  has slope −2.1, close to the Λ⁻² line [11] draws in Fig. 1a after Jiménez 2004 (arithmetic).

| Λ = k/h | k_s/k on Λ⁻² from the peak | k_s / h | k_s / Ra (k = 4.4 Ra) |
| --: | --: | --: | --: |
| 0.18 (shallow, coarse layers) | 6.6 | 1.2 | 29 |
| 0.35 | 1.7 | 0.61 | 7.6 |
| 0.70 (deep, fine layers) | 0.43 | 0.30 | 1.9 |

All **Inference**. Fine layers leave the deeper ridges (Harmon's 0.70 h at 0.1 mm, §1), so this spans about 30 µm
(0.1 mm layers) to 360 µm (0.3 mm), against hobby-finish-roughness's 11 to 197 µm: the old 2 to 6 Ra band fits deep
fine-layer ridges and may read low by up to about 5 times for shallow coarse ones.
- **The low end is open.** [11]'s own runs at Λ = 0.25 and 0.75 (Table 1: k+ 50, ΔU+ 4.1; k+ 300, ΔU+ 5.4) give
  k_s/k ≈ 0.42 and 0.12 on the fully rough law, 8 and 3 times below the Λ⁻² line (arithmetic); being transitional they
  are lower bounds. If k_s ≈ 0.21 W held for printed ridges (W ≈ h/2), k_s ≈ 0.1 h, or 10 to 30 µm (**Inference**).
- **Shape, sign only.** These are sharp-edged bars; FDM beads are rounded crests in V valleys, a negative-skew profile,
  and negative skew lowers k_s on random surfaces (hobby-finish-roughness §2); [10] names skewness and ES as 2D
  roughness's drag parameters. So the true k_s likely sits below the table, by an unbounded amount (**Inference**).
- **Metal printing, for contrast:** upright LPBF tubes' Ra (7.48 to 14.28 µm) did not predict their fitted k_s [12].

## 3. Print direction

- **Measured, roughness:** along-layer Ra is 3.9 to 9.1 times below across-layer on the same plates [2] Table 1.
- **Measured, drag anisotropy (DNS, irregular surfaces at k+ ≈ 66):** at fixed height the roughness function ΔU+ ran
  2.5 to 7.6 with the spanwise and streamwise slopes, and a riblet-like streamwise-stretched surface fell to about 2.4
  [13] p. A1-11. Spanwise-stretched features raised ΔU+ by over 200% against streamwise ones [13] p. A1-3, after [14].
- **Inference:** at one ridge height, an upright-printed tube or nose cone (ridges across the flow) carries roughly 2
  to 3 times the roughness function of a flat-printed fin face with its raster along the chord.
- **No riblet benefit at rocket scale.** Riblets cut drag at spacings s+ ≈ 10 to 20 and add it when larger [15] p. 2;
  an assumed 0.4 mm raster pitch at u_τ = 3.9 to 10.8 m/s gives s+ ≈ 107 to 296 (arithmetic).

## 4. Paper, glassine and phenolic surfaces

| Surface | Measured (µm) | Method | k_s at 2–6 Ra (µm) | Src |
| --- | --- | --- | --- | --- |
| 7 coated boards, 300 g/m² | Ra 0.67–0.80; Rz 5.5–6.2 | confocal, 0.5 × 0.7 mm field | 1.3–4.8 | [16] Table 7 |
| same boards | Parker Print-Surf 0.94–1.70 | air leak, ISO 8791-4 | no conversion | [16] Tables 5, 7 |
| light-coated kraftliner | Print-Surf 1.4–1.7, print side | ISO 8791-4 | no conversion | [17] |
| kraft 80 g/m²; liner 172 g/m² | Ra 3.75–4.27; 4.08–4.15 | stylus, 1.0 mm tip | 7.5–26, a floor | HFR [18] |

The k_s column is **Inference**; HFR is hobby-finish-roughness, whose source 18 that row is.
- **Print-Surf does not convert.** [16] found it correlates only weakly with any optical height parameter (abstract,
  p. 11): it is an air-leak gap under a clamped ring, not a height statistic. The blunt 1.0 mm stylus bridges fiber
  gaps, so bare kraft's Ra is a floor (**Inference**). No fine-tip or optical Ra of brown kraft was found.
- **Glassine.** LOC Precision and Apogee both describe a glassine outer wrap on their round tubes [18][19]. No
  smoothness value for glassine was found; being supercalendered, it likely sits with the coated rows (**Inference**).
- **Phenolic.** Industrial convolute paper-phenolic tube is centerless ground as standard up to 9.000 in OD [20] p. 2,
  with no Ra given; one maker's data sheet gives no surface property [21]. Hobby phenolic is spiral wound, not ground,
  so neither describes it. A builder's side-by-side of 3.9 in tubes rates spiral grooves light (LOC kraft, Blue Tube),
  medium (two paper phenolics), heavy (MagnaFrame) and none (canvas phenolic, wound glass), with no depths [22].
- **Seams.** No groove depth was found for any rocket tube; a paper-tube patent gives core plies of 0.020 and 0.024 in
  [23], so a groove is at most a ply deep: a discrete step, outside sand-grain k_s (**Inference**).

## 5. On a rocket, and the bias

Computed as hobby-finish-roughness §4: a flat plate of the body length, C_f from Niskanen's eq. 3.78,
u_τ = V √(C_f/2), ν = 1.46 × 10⁻⁵ m²/s, hydraulically smooth below k_s+ ≈ 5 and fully rough above about 70.

| Case | Re | k_s+, paper at 1.5 / 7.5 / 26 µm | k_s+, FDM 0.2 mm at 60 / 240 µm |
| --- | --: | --- | --- |
| DeMar tunnel, 0.254 m, 33 m/s | 5.7 × 10⁵ | 0.2 / 0.8 / 2.9 | |
| 1 m, 100 m/s | 6.8 × 10⁶ | 0.4 / 2.0 / 6.9 | 16 / 64 |
| 1 m, 200 m/s | 1.4 × 10⁷ | 0.8 / 3.8 / 13.2 | 30 / 122 |
| 1 m, 300 m/s | 2.1 × 10⁷ | 1.1 / 5.5 / 19 | 44 / 178 |

All **Inference**, computed. Glassine and coated tubes are smooth through the core band's subsonic part; bare kraft is
at most transitional (k_s+ up to about 19), and bare FDM transitional to fully rough, the one finish where it matters.

- **Paper tubes and the bias.** HPR Sim's unset finish (20 µm) sits inside the kraft band and already rides the fully
  rough constant above 57 m/s on a 1 m body (hobby-finish-roughness §4). A truer k_s lowers friction and raises the
  predicted apogee, widening the +8.73% ([apogee-bias](apogee-bias.md)). So bare paper cannot be the friction share
  of the bias unless real kraft k_s is well above 20 µm, which nothing measured shows (sign firm, size open).
- **Printed parts and the bias.** A bare print left at 20 µm sits below the 30 to 360 µm of §2. On the fully rough
  form C_f = 0.032 (k_s/L)^0.2 (Niskanen eq. 3.80), 30 to 360 µm reads +8% to +78% friction over 20 µm (arithmetic);
  the true transitional regime makes that an upper bound. The sign is firm only where the print is fully rough (k_s
  above about 95 µm at 300 m/s, 260 µm at 100 m/s on 1 m), since the model's 20 µm rough constant already reads 19% to
  42% over smooth (hobby-finish-roughness §4). There, unset finish adds to the bias. How many of the collection's
  rockets have bare printed parts was not counted here.

## What this suggests

- **A layer-height input for printed parts** (an idea): k_s ≈ 0.3 to 1.2 h across the layers, lower along them, with
  a direction flag (upright tube or nose against flat fin). Expected effect: friction up on bare printed bodies, toward
  fully rough, against the 2 to 6 Ra band at coarse layers; sign firm at coarse layers, size a factor of 4 to 10.
- **Split the bias by finish.** Count the collection's designs with bare printed outside parts and compare their
  apogee error with the rest; §5 predicts printed ones read higher.
- **The cheapest numbers:** a printed plate or pipe with taps at least 20 diameters from any fitting, or a towed plate
  with ridges across and along; and a fine-tip or optical Ra of a bare kraft tube along and across the spiral, with
  its seam depth. Either replaces an extrapolation here.

## Open questions

- No k_s has been measured on any FDM surface (bare, sanded or vapor-smoothed), nor on kraft, glassine or phenolic.
- k_s/k for dense rounded 2D ridges (Λ 0.2 to 0.7): Abdelaziz et al. 2022 [10] fits k_s from ES and skewness for rods,
  ribs and sine waves; the paper is paywalled and the paper and 2023 thesis copies failed, so it was not read.
- Two NAR R&D reports may hold a second finish series: Bock, "Drag vs Surface Finish on an Alpha" (NARAM-41, 1999), and
  Sowden, "Comparison of Preparation Methods for Body Tubes" (NARAM-32, 1990) [24]. Both need a member login and were
  not read.
- Whether DeMar's +13% comes from the tube, the balsa fins or transition cannot be told from his series.
- Raster pitch and raster angle on printed fin faces, and whether layer ridges near a nose trip transition early (a
  drag increase of unknown size), lack a measured source.

## Sources

All read 2026-10-18. Cite only unless a license is named; copies under `refs/papers/` where cached.

1. G. B. Harmon et al., "A design framework for engineered roughness in additively manufactured thermoplastic pipes",
   *IJIDeM* 20, 3845–3870, 2026, CC BY 4.0. <https://link.springer.com/article/10.1007/s12008-026-02572-y>
2. L. F. A. Santos et al., "Average superficial roughness analysis of manufactured pipes by 3D printing",
   COBEM-2019-1537. <https://repositorio.ufrn.br/handle/123456789/43026>
3. A. M. Springer, NASA/TP-1998-208396, 1998. <https://ntrs.nasa.gov/citations/19980201248>
4. R. Ujjin et al., "3D-printed airfoil wind tunnel aerodynamic testing and simulation", RSU Int. Res. Conf. 2019,
   pp. 931–942. <https://rsucon.rsu.ac.th/files/proceedings/inter2019/IN19-175.pdf>
5. Engineers Edge, duct roughness table after *ASHRAE Handbook Fundamentals* 2021 (secondary).
   <https://www.engineersedge.com/fluid_flow/duct_roughness_factors_table_15789.htm>
6. ASHRAE, "Determining the absolute roughness of phenolic duct", 2019, on S. Idem, A. Paruchuri, RP-1764, 2018.
   <https://www.ashrae.org/news/esociety/determining-the-absolute-roughness-of-phenolic-duct>
7. J. S. DeMar, "Model Rocket Drag Analysis using a Computerized Wind Tunnel", NAR R&D, NARAM-37, 1995, §§1.2, 2.3.
   <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
8. K. A. Flack, M. P. Schultz, *Phys. Fluids* 26, 101305, 2014, doi:10.1063/1.4896280.
9. R. J. Volino, M. P. Schultz, K. A. Flack, *J. Fluid Mech.* 635, 75–101, 2009, doi:10.1017/S0022112009007617.
10. M. Abdelaziz, L. Djenidi, M. H. Ghayesh, R. Chin, *J. Fluid Mech.* 940, A25, 2022 (abstract only).
11. M. MacDonald et al., *J. Fluid Mech.*, 2018, doi:10.1017/jfm.2018.150. <https://arxiv.org/abs/2012.04096>
12. C. R. Hartsfield et al., "Understanding flow characteristics in metal additive manufacturing", *J. Aerosp. Eng.*,
    doi:10.1061/(ASCE)AS.1943-5525.0001325, CC BY 4.0. <https://www.osti.gov/servlets/purl/1982348>
13. T. O. Jelly et al., *J. Fluid Mech.* 951, A1, 2022, CC BY 4.0. <https://eprints.gla.ac.uk/281136/>
14. A. Busse, T. O. Jelly, *Flow Turbul. Combust.* 104, 331–354, 2020 (read through [13]).
15. W. Ran, A. Zare, M. R. Jovanović, riblet design, arXiv:2002.01671, 2020. <https://arxiv.org/abs/2002.01671>
16. S. Rosalen, J. Backhaus, TAPPICon 2021, PC1.1. <https://www.tappi.org/content/Events/21TAPPICon_Virtual/PC1.1.pdf>
17. Billerud, "Pure Decor" liner data sheet, 2026-02.
    <https://www.billerud.se/globalassets/billerud/packaging-materials/tds/tech-data-sheet---pure-decor-2026-02.pdf>
18. LOC Precision, cardboard airframes page. <https://locprecision.com/products/cardboard-airframes>
19. Apogee Components, Vigilangle kit page. <https://www.apogeerockets.com/rocket_kits/skill_level_3_kits/vigilangle>
20. Norplex-Micarta, "Convolute Rolled Tube Products: Phenolic Resins, Paper Substrates", 2023, p. 2.
    <https://www.norplex-micarta.com/wp-content/uploads/2017/07/MC_Tubes_All_-Resins_Paper_Substrates.pdf>
21. The Gund Company, phenolic paper laminated tube data sheet, 05/2023.
    <https://thegundcompany.com/app/uploads/2023/05/Phenolic-Cellulosic-Paper-Laminated-Tubes-by-The-Gund-Co.pdf>
22. J. Coker, body tube comparison (Grooves column). <https://www.jcrocket.com/body-tubes.shtml>
23. US Patent 6,851,643, spirally wound tube. <https://patents.google.com/patent/US6851643>
24. NAR, R&D reports index (reports need a member login).
    <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
