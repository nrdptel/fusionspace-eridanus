# What roughness hobby rocket finishes have, as equivalent sand grain

HPR Sim gives a part with no finish 20 µm, OpenRocket 60 µm (a `.ork` outside part now reads 60 µm, [#216][i216]); in
the friction model both use, that step adds about 25% ([apogee-bias](apogee-bias.md) §2). This note asks what hobby
finishes really measure. **In short: painted, gel-coated and sanded surfaces are far smoother than either default.**
Published values put automotive-type paint at Ra 0.04 to 2 µm and sanded fiberglass at 0.6 to 2.3 µm: k_s about 0.1 to
14 µm at the usual 2 to 6 Ra. Only bare FDM prints (Ra 5 to 33 µm across 0.1 to 0.3 mm layers), perhaps bare kraft and
wood, and coarse marine coatings reach the defaults' size. 20 µm is Hoerner's 1965 grain size for aircraft production
paint; 60 µm has no published source. A 1 m rocket at 100 to 300 m/s is hydraulically smooth below k_s of about 7 to 19
µm, yet the shared model applies its fully rough constant above 57 m/s for 20 µm. **Inference:** a truer finish lowers
both codes' friction and raises their apogee, so finish cannot explain the collection's high bias; it more likely hides
part of it. How far to trust this: Ra is measured, but no k_s on any hobby finish; the conversion spans a factor of 3;
the apogee sign is arithmetic only.

## 1. What the defaults and named finishes are

**Hoerner's table is grain size, not sand grain.** Its grain heights or diameters were measured under a 10× to 20×
microscope with oblique light, a method he calls "somewhat arbitrary" [1] p. 5-3. His Table A gives mirror 0, glass
0.1, polished 0.5, sheet metal 2, optimum spray paint 5, planed wood 15, mass-production aircraft paint 20, bare steel
50, cement 50, asphalt coating 100, dip-galvanized 150, incorrectly sprayed paint 200, cast iron 250, raw wood 500 and
concrete 1,000 µm. On p. 5-4 he converts grain diameter d to sand roughness k through shape and spacing: spheres reach
k/d = 4 near 20% coverage, and for the flat cones and segments he sees in paint he suggests k/d ≈ 1 "or a somewhat
lower value" [1].

**Barrowman and Niskanen use the grain sizes as k_s directly.** Barrowman's Table 4-1 copies the 15 rows as roughness
heights R_s and feeds them to the critical Reynolds number 51 (R_s/l)^−1.039 and the rough-wall friction 0.032
(R_s/l)^0.2 (eqs. 4-7, 4-8) [2]. Niskanen keeps 10 rows (Table 3.2, p. 44) with the same equations (3.79, 3.80)
[3][4]. The friction equation is Hoerner's eq. 4, p. 5-3 [1]. So both codes carry k/d ≈ 1 silently. That is Hoerner's
own estimate for paint, but nothing measured on hobby finishes stands behind it (**Inference**).

**OpenRocket's names.** The user guide lists Rough, Unfinished, Regular paint, Smooth paint and Polished with no
values [5]. The program uses 500, 150, 60, 20 and 2 µm (#216 reads 60 µm from its API); a 2013 forum post by its
author lists them without a source [6]. Against Hoerner: 500 is raw wood, 150 dip-galvanized metal, 20 aircraft
production paint, 2 sheet metal. 60 µm has no row; Niskanen says only that regular paint "corresponds to an average
surface roughness of 60 µm" for a test rocket whose roughness was not measured [3] p. 83, [4] p. 85 (**Inference:** a
round number).

## 2. From a measured roughness to k_s

No single factor turns Ra (mean absolute height) into k_s. Rq is the root-mean-square height, Rz the mean
peak-to-valley over five lengths, Rt the largest, Sk the skewness (peaks positive).

| Source | Surfaces | k_s from statistics | Spread |
| --- | --- | --- | --- |
| Adams et al. 2012 [7] Table 1 | model: one layer of equal spheres | 5.86 Ra; 3.10 Rq; 0.98 Rz | |
| same, Table 4 | 2.05 cm copper pipe, measured | 1.78 µm = 8.7 Ra, 0.94 Rz | ± 0.08 µm |
| [8] Figs. 7–9 | Sk > 0 / ≈ 0 / < 0 | 2.48 Rq (1+Sk)^2.24 / 2.11 Rq / 2.73 Rq (2+Sk)^−0.45 | R² ≥ 0.95 |
| [8] Table 1 | 7 printed random surfaces, Rq 45–114 µm | k_s/Ra 1.81–6.23; k_s/Rq 1.45–4.86 | |
| Flack et al. 2016 [9] eq. 3.2 | grit-blasted | 2.91 Rq (2+Sk)^−0.284 | 45 cases: mean error 48%, max 121% |
| Schultz 2004, via [11] | five antifouling coatings | k = 0.17 Ra in a Colebrook-type function | see below |

Schultz's paint fit uses ΔU+ = (1/κ) ln(1 + k+) [11] eq. 25; in the fully rough limit it equals the sand-grain
function when k_s ≈ 4.2 k, so k_s ≈ 0.7 Ra for those coatings (**Inference**). The spread comes from peaks against
pits and from slope: a peaked surface had 2.2 times the k_s of a level one with the same Rq [8] Table 1, wavy ones add
little [8] p. 12, and on polished airfoils Ra/k_s varied about 400% with slope [10] (abstract). Paint is low in
slope, so it likely sits low (**Inference**). §3 uses 2 to 6 Ra, with 0.7 Ra a lower bound for paint.

## 3. What hobby finishes measure

Values are in µm, mostly stylus readings; stated cutoffs run 0.25 to 2.5 mm, and several sources state none, so
spreads of 2 to 10 times between sources are expected (**Inference**). k_s is this note's **Inference** at 2–6 Ra.

| Finish | Measured | k_s (µm) | Src |
| --- | --- | --: | --- |
| Clear coat over basecoat; polyester | Ra 0.04–0.09; 0.05–0.15 | 0.1–0.9 | [14] (1988 book quoted) |
| Factory automotive paint on steel | Ra 1.26 mean (1.06–1.74); Rz 2.6–4.4 | 2.1–10 | [14] |
| Body-shop refinish spray, 3 shops | Ra 1.37–1.65 mean (1.15–2.05) | 2.3–12 | [14] |
| Wind-tunnel model, painted | Ra 0.7 (0.3 bare) | 1.4–4.2 | [15] abstract |
| Marine paint, unsanded as sprayed | Rq 3.4–5.0, Rt 39–50 | 4.9–24 (1.45–4.86 Rq) | [13], after [12] |
| Same paint, polished | Rq 0.27–0.30 | 0.4–1.5 | [13] |
| Antifouling coatings, as cured | Ra 12–20, Rt 66–129 | 8–14 (Schultz's fit) to 120 | [11] Table 1 |
| Gel-coat barrier coat, panel | Ra 1.44 | 2.9–8.6 | [16] Table 3 |
| Glass-fiber laminate, degreased | Ra 0.56, Rz 3.12 | 1.1–3.4 | [17] Table 3 |
| Same, P60 / P240 / P800 sanded | Ra 2.30 / 1.55 / 0.62 | 4.6–14 / 3.1–9.3 / 1.2–3.7 | [17] Table 3 |
| Kraft paper; linerboard | Ra 3.75–4.27; 4.08–4.15 | 7.5–26 (a floor) | [18] Tables 1, 3 |
| Four woods, planed and sanded | Ra 4.74–6.78 | 9.5–41 | [19] abstract |
| FDM side wall, 0.1 mm layers | Ra 5.5–15.7 | 11–94 | [20][21] |
| FDM side wall, 0.2 mm layers | Ra 9.2–24.1 | 18–145 | [20][21][24] |
| FDM side wall, 0.3 mm layers | Ra 13.1–32.8 | 26–197 | [20][21] |
| FDM, wet sanded to P4000; acetone vapor 10 min | Ra 0.72; 0.2 | 1.4–4.3; 0.4–1.2 | [22]; [24] |

- **Paint.** No measurement of aerosol-can paint was found; hand-sprayed automotive paint [14] is the closest.
  Unsanded spray raised plate friction by up to 7.3% over the same film polished, and 600-grit sanding left a
  measurable rise [12] abstract. A primer-surfacer on a NACA wing section added 30% of its smooth profile drag at the
  top test speeds (10 to 14% at cruise) until wet-sanded with No. 400 paper [23] pp. 2, 4. Orange peel is waviness
  (about 1 µm at 0.1 to 30 mm wavelengths [25]), outside the cutoff; likely small, but no source bounds it.
- **Composites.** Carbon fiber cured against a polished tool plots below Ra 0.3 µm [26]; on release films a student
  project read 1.2 to 5.6 µm, 21 µm with a liquid release agent [27] abstract. None found for wound tubes or peel ply.
- **Paper and wood.** [18]'s stylus tip is 1.0 mm across, 250 times the usual 2 µm tip radius, so kraft's Ra is a floor.
  No spiral seam height was found; a seam is a discrete step at most one ply deep (0.1 to 0.25 mm by [18]'s
  thicknesses, **Inference**), not sand-grain roughness. None was found for phenolic or balsa.
- **FDM.** Ra scales with layer height h: Ra/h is 0.04 to 0.16 across five studies (**Inference**, arithmetic), and
  Rz/Ra about 4.4 [22][24]. Along the layers the same walls read Ra 1.1 to 8.6 [20]. A vertically printed tube shows
  the flow the across-layer profile (**Inference**).
- **Grit.** On a painted plate, 600 to 60 grit gave Rq 0.67 to 1.63 µm [13]; on glass-fiber laminate P800 to P60
  gave Ra 0.62 to 2.30 [17]. Grit labels mix US and FEPA scales above about 220, so the finest rows are uncertain.

## 4. Smooth, transitional or fully rough on a rocket

A rough wall is hydraulically smooth below about k_s+ = 5 and fully rough above about 70 ([28], after Nikuradse),
where k_s+ = k_s u_τ/ν and u_τ = V √(C_f/2). Printed random surfaces left the smooth curve at k_s+ 4 to 6 and were
fully rough at 40 to 45; a peaked one needed 10 to 70 [8] pp. 10, 12. Hoerner's "permissible" roughness, V k/ν about
90 to 120, marks the same edge [1] p. 5-2.

**Inference**, computed: a 1 m flat plate at sea level (ν = 1.46 × 10⁻⁵ m²/s), C_f from Niskanen's eq. 3.78 [3].

| V | Re | u_τ | k_s+ at 5 / 20 / 60 / 150 µm | k_s at k_s+ = 5 | k_s at k_s+ = 70 |
| --- | --: | --: | --- | --: | --: |
| 100 m/s | 6.8 × 10⁶ | 3.9 m/s | 1.3 / 5.4 / 16 / 40 | 19 µm | 260 µm |
| 200 m/s | 1.4 × 10⁷ | 7.4 m/s | 2.5 / 10 / 31 / 76 | 10 µm | 138 µm |
| 300 m/s | 2.1 × 10⁷ | 10.8 m/s | 3.7 / 15 / 44 / 111 | 7 µm | 95 µm |

So every automotive paint, gel-coat and sanded-glass row of §3 is smooth or barely transitional in the core band's
subsonic part, and 20 µm is transitional (k_s+ 5 to 15), never fully rough below Mach 1. Yet eq. 3.79 puts a 1 m body
on the rough constant above 57 m/s for 20 µm and 18 m/s for 60 µm; the smooth curve already crossed it at 36 and 12
m/s, so C_f jumps up 8% to 9% there [3] (**Inference**, computed). At 100, 200 and 300 m/s, 20 µm then reads +19%,
+33% and +42% over smooth. Lowering the smooth wall law by the fully rough sand-grain ΔU+, an upper bound for a
Nikuradse-type surface in transition, gives at most +6%, +20% and +28% (**Inference**, rough in size, firm in sign).

## 5. What it means for #216, the finish band and the bias

- **The default.** Neither 20 nor 60 µm is a measured hobby finish (§1); on a painted rocket both read friction high.
  #216 (read a file as OpenRocket does) stays right as interoperability, but nothing measured backs 60 µm as physics.
- **The finish band.** Seniwan and Mohd Saiah's "drag increases of 17–75%" [28] is a review citing a CFD study of an
  X-33-type launch vehicle with no k_s values given; the review itself measures nothing (abstract; Table 2, p. 71).
  Its proposed levels (0.5, 60, 150 µm, p. 73) are Hoerner's and OpenRocket's; no hobby finish factor follows.
- **Model-rocket data.** At 120 km/h DeMar's BT-20 tunnel models gave C_D 0.77 unfinished (kraft, bare balsa), 0.68
  primer and Krylon, 0.61 polished: +13% and −10% [29] p. 7; at 80 km/h polished read above painted, and the gaps are
  2 to 3 counts of the 0.5 g resolution (p. 5).
  Gregorek estimates (not tests) 51 µm wood grain at C_D 25% above smooth at 91 m/s (300 ft/s) [30] p. 35.
- **The bias.** On the collection, HPR Sim's apogee runs +8.73% above the logs ([apogee-bias](apogee-bias.md)). On
  #216's six probes, stating 60 µm instead of leaving 20 µm (about +25% friction) cut HPR Sim's gap to OpenRocket
  from +6.0–7.3% to +0.4–0.8%, about 5 to 7 points of apogee. **Inference:** if painted hobby rockets are near
  hydraulically smooth, true friction is 16% to 30% below the 20 µm model's at 100 to 300 m/s, so a truer finish
  would raise the codes' apogee by roughly 3% to 8% on such a rocket. For painted rockets, finish then cannot be the
  friction share of the bias; it more likely masks other missing drag (or mass) of that size. Laminar runs push the
  same way; discrete roughness (seams, fillets, decals, buttons), outside sand-grain k_s, the other. Neither is sized.

## What this suggests

- **A regime check before the rough constant** (an idea, not a fix): compute k_s+ and blend smooth to rough through a
  cited transitional roughness function (Colebrook or Nikuradse type) instead of switching at eq. 3.79. Expected
  effect: friction about 12 to 14 points lower at 100 to 300 m/s for 20 µm on a 1 m body; apogee up. It would depart
  from Niskanen and OpenRocket, so it needs an oracle comparison and a decision record.
- **Measure the bias at smooth finishes.** Rerun the collection's real-flight comparison with every outside part at
  5 µm (optimum spray paint) and 0.5 µm; how the +8.73% moves sizes the drag the defaults may be hiding.
- **A measured-Ra input.** Map Ra to k_s = 2 to 6 Ra (0.7 Ra low for paint) or Rz to about 1.0 Rz [7], as a band;
  bare FDM could take Ra ≈ 0.04 to 0.16 times its layer height.

## Open questions

- No k_s was found measured on any hobby finish (rattle-can paint, kraft or phenolic tubes, FDM); a towed-plate or
  pipe test of one would replace the 2–6 Ra conversion with a number.
- Flack and Schultz 2010 (public copy down on 2026-10-16) and Schultz 2004 were read only through citing papers.
- Whether the CFD study behind "17–75%" exists as cited: one title search found it only in [28]'s reference list.
- How much discrete steps (spiral seams, fin fillets, paint edges) add on a hobby rocket: no measurement was found.
- The +3% to +8% apogee estimate in §5 rests on six related probes' sensitivity; a collection run would replace it.

## Sources

All read 2026-10-16. Cite only unless a license is named; copies under `refs/papers/` where cached.

1. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, pp. 5-2 to 5-5. <https://archive.org/details/FluidDynamicDragHoerner1965>
2. J. S. Barrowman, MS thesis, 1967 (NASA/TM-2001-209983), Table 4-1, eqs. 4-7, 4-8.
   <https://ntrs.nasa.gov/api/citations/20010047838/downloads/20010047838.pdf>
3. S. Niskanen, MSc thesis, 2009, Table 3.2, eqs. 3.78–3.80, p. 83. <https://openrocket.sourceforge.net/thesis.pdf>
4. OpenRocket technical documentation v13.05, Table 3.2 p. 44, p. 85. <https://openrocket.sourceforge.net/techdoc.pdf>
5. OpenRocket user guide. <https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html>
6. OpenRocket's author, 2013-08-22. <https://www.rocketryforum.com/threads/open-rocket-finishes.57558/post-585716>
7. T. Adams, C. Grant, H. Watson, *Int. J. Mech. Eng. Mechatronics* 1(1), 66–71, 2012, CC BY 2.0.
   <https://web.archive.org/web/2020/https://ijmem.avestia.com/2012/PDF/008.pdf>
8. Flack, Schultz, Barros, *Flow Turbul. Combust.* 104, 317–329, 2020. <https://oist.repo.nii.ac.jp/records/1571>
9. M. Aghaei Jouybari et al., data-driven prediction of k_s, Table 3, eq. 3.2. <https://arxiv.org/abs/2002.01515>
10. M. N. Goodhand et al., GT2015-43329, abstract. <https://eprints.hud.ac.uk/id/eprint/27000/>
11. Karabulut et al., *J. Marine Sci. Appl.* 19(1), 2020, §4, eq. 25 <https://html.rhhz.net/jmsa/html/20200104.htm>;
    Demirel et al. 2014, *Ocean Eng.* 89, Table 1 after Schultz 2004 <https://strathprints.strath.ac.uk/51626/>.
12. M. P. Schultz, *J. Fluids Eng.* 124(2), 492, 2002, abstract (paywalled).
13. Klaka 2022, Tables 1–2 (after [12]). <https://www.onemetre.net/Race/Surface%20finish/>
14. Guma, Gana 2022, *LAUTECH J. Eng. Technol.* 16(2), 7–19. <https://laujet.com/index.php/laujet/article/view/500>
15. Gebbie 2005, AFIT MS thesis, abstract. <https://scholar.afit.edu/etd/3669>
16. Ravenna et al. 2022, A. Yücel Odabaşı Colloquium, Table 3. <https://strathprints.strath.ac.uk/84014/>
17. Nazari et al. 2024, arXiv:2406.15292, Table 3, CC BY-SA 4.0. <https://arxiv.org/abs/2406.15292>
18. Park et al. 2021, *BioResources* 16(3), 5667–5678, Tables 1, 3. <https://bioresources.cnr.ncsu.edu/?p=41023>
19. Sofuoğlu, Kurtoğlu 2015, *Drvna industrija* 66(4), 265–272, abstract. <https://hrcak.srce.hr/en/150794>
20. Alsoufi, Elsayed 2017, *Am. J. Mech. Eng.* 5(5), CC BY 4.0. <http://pubs.sciepub.com/ajme/5/5/4/index.html>
21. Djurović et al. 2025, ICPES 2025, 371–374. <https://scidar.kg.ac.rs/handle/123456789/22692>
22. Kočiško et al. 2025, *Materials* 18(20), 4672, CC BY 4.0. <https://pmc.ncbi.nlm.nih.gov/articles/PMC12566055/>
23. Hood 1938, NACA SR-78, effect of surface irregularities on wing drag. <https://ntrs.nasa.gov/citations/20090015316>
24. Kordikova et al. 2024, *Trudy BGTU* ser. 2, 2(283), 176–183 <https://elib.belstu.by/bitstream/123456789/67095/1/>;
    Pérez et al. 2018, *Materials* 11(8), 1382, Table 6, CC BY 4.0 (0.15 and 0.25 mm layers).
25. Kigle-Böckler 2006, "Controlling appearance", *PCI Magazine*. <https://pcimag.com/articles/94460>
26. Herring, Fox 2011, *Compos. B* 42, 1035–1043, Figs. 1–3.
27. Louis, Dang, Schlocker 2022, Cal Poly senior project, abstract. <https://digitalcommons.calpoly.edu/matesp/255>
28. W. M. H. Seniwan, H. R. Mohd Saiah, *J. Transport System Eng.* 13(1), 68–74, 2026.
    <https://jtse.utm.my/index.php/jtse/article/download/263/187/1280>
29. J. S. DeMar, NAR R&D report, 1995, pp. 4–7. <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
30. G. M. Gregorek, Estes TR-11, 1970, p. 35. <https://www.plans.rocketshoppe.com/pubs/Estes/estTR-11/TR-11.pdf>

[i216]: https://github.com/nrdptel/fusionspace-eridanus/issues/216
