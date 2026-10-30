# Canopy C_D0 against descent speed and line length on 2 to 10 ft canopies

**In short: the low-speed rise is measured on hobby-sized canopies: two of Knacke's speed figures come from them.**
Figure 5-16 of Knacke's manual is 1949 free drops of 45 in (3.8 ft) model canopies, not full-scale ones, and Figure
5-18 comes from the same report [1, pp. 5-16 to 5-18]. A solid flat circular model reads C_D0 0.91 to 0.97 at 15
ft/s, 0.84 to 0.85 at 20 ft/s and 0.79 to 0.82 at 25 to 30 ft/s, on the nominal area S_0. Against the simulator's
flat-circular default of 0.775, a hobby main at 15 to 22 ft/s then takes 3.5 to 12% longer to come down than
predicted, so descent time and drift read short by that much (**Inference**, §6). Line length can cancel it. Lines
of 0.6 D_0 cost 12 to 20% of C_D0 on canopies of 3 to 12 ft (§4); with them the same canopy's error runs from −7% to
+5%. Reynolds number adds a few percent at most at hobby sizes, in the other direction (§5). **How far to trust
it:** the 45 in data are one 1949 test series, read from figures; its line length isn't printed. Tunnel tests of
small tethered canopies agree on the direction but not the size. No hemispherical or cross canopy of 2 to 10 ft was
found measured against speed. This note feeds issue #741 and builds on [hobby-parachute-cd](hobby-parachute-cd.md)
§3 (Knacke's Figure 5-24 on 24 to 100 ft canopies) and
[hobby-canopy-glide-and-swing](hobby-canopy-glide-and-swing.md) §1 (why round canopies glide).

## Method

- Every C_D0 below is on S_0, the whole cloth area (D_0 = √(4S_0/π)), unless a line says otherwise. A free-drop
  C_D0 is the vertical one, 2W/(ρv²S_0) from the steady descent rate; a glide shows up in it as a higher value.
- Figure values were read by pixels against the printed ticks or grid (300 dpi renders, centroids of the markers),
  to about ±0.01 in C_D0. The scripts are in the project's private cache, not the repository.
- At one load the descent speed goes as 1/√C_D0, so a canopy whose true C_D0 is C flies a descent time (and, in a
  given wind, a drift) √(C/0.775) times the simulator's. Landing energy goes as 0.775/C.
- Reynolds numbers use D_0 and ν = 1.46×10⁻⁵ m²/s (sea level, 15 °C): a 2 ft canopy at 15 ft/s is 1.9×10⁵, a
  10 ft canopy at 22 ft/s 1.4×10⁶ (computed here).

## 1. Knacke's speed curves on 45 in models (the closest measurement to hobby mains)

- **The tests.** In 1949 the U.S. Air Force dropped 1,600 in² model canopies in the Lakehurst airship hangar
  [1, p. 5-16; report MCREXE-672-12B, ref. 5.1]. That area is D_0 = 45.1 in (1.15 m); Figure 5-16's legend gives
  D_0 3.8 ft. Solid canopies oscillated violently above 30 ft/s; loaded down to 10 ft/s they stopped swinging but
  glided at up to 45°; between 15 and 25 ft/s they mixed the two [1, p. 5-16]. Line length isn't stated.
- **Figure 5-16** (p. 5-16), read here: flat circular 0.91, 0.84, 0.82 and 0.79 at 15, 20, 25 and 30 ft/s;
  30° conical 0.99, 0.93, 0.90, 0.88; quarter-spherical 1.03, 0.96, 0.92, 0.91. The text's check at 20 ft/s
  (conical 9.5% and quarter-spherical 14.5% above flat) matches these readings to about a point.
- **Figure 5-18** (p. 5-18), from the same report (ref. 5.1); it prints no size, so 45 in is **Inference** (the
  report also flew 3.1 ft canopies, Figure 5-20). Two solid flat curves (circular and square share one symbol). The
  lower reads 1.12, 0.97, 0.85, 0.82, 0.81 and 0.81 at 10, 15, 20, 25, 30 and 35 ft/s; the upper 1.21, 1.06, 0.98,
  0.93, 0.90 and 0.89. A flat ribbon canopy and a guide surface, both stable, stay flat at about 0.5 and 0.4. The
  lower curve matches Figure 5-16's flat circular to 0.02 from 20 ft/s up, so it is likely the circular one
  (**Inference**); at 15 ft/s the two figures differ by 0.06, which this note keeps as the spread.
- **Cause.** Knacke puts the rise down to the change from oscillation to glide [1, p. 5-18]. Cockrell, on USAF model
  and full-scale drops at 4.6 to 9.1 m/s (Re 3.6×10⁵ to 7.2×10⁵ in his text; his Figure 2.9c, after Knacke, is
  likely the same series (**Inference**), though its axis reads 1.2×10⁵ to 2.4×10⁵), suggests an unsteady effect
  (pitch stability falling with the load) rather than a Reynolds one. Canopies held rigidly in a tunnel change
  little above Re 10⁵ [2, p. 11].
- **Size of the rise.** From 30 to 15 ft/s the flat circular model gains 15 to 20%, the conical and quarter-spherical
  12 to 13%. Hanging rocket parts below a hobby main may damp or feed the swing; no source tests that.

## 2. A larger model in the same kind of drop: 11.9 ft extended skirt

- Stimler and Ross's drop report AFTR 5867 (1960) logs about 700 drops of 16,000 in² (D_0 11.9 ft) models; Tutt
  re-prints eight drops of the 10% extended skirt, with C_D on S_0 [3, pp. 1, 3 to 5].
- Lines of 0.92 D_0: 1.05, 0.87, 0.82 and 0.91 at 10.7, 16.5, 27.0 and 41.5 ft/s. Lines of 0.60 D_0: 0.85, 0.77,
  0.69 and 0.70 at 11.9, 17.7, 29.2 and 45.2 ft/s [3, Table IV-1, p. 5]. One drop per point; the 41.5 ft/s value
  sits above the 27 ft/s one, so single drops may scatter by about 0.1 (**Inference**).
- From about 28 to 17 ft/s both line lengths gain 6 to 12%. The report's own summary says these canopies glided at
  low speed and oscillated widely at high speed [3, p. 5]. Outside the 2 to 10 ft range, but only by a little; but it is
  the one series with speed and line length varied together.

## 3. Free-floating in a vertical tunnel: 3.5 ft flat circular

- NASA TN D-6423 flew 3.5 ft (1.07 m) flat circular canopies of four gores free in the Langley 20 ft spin tunnel,
  lines 4.7 ft (1.34 D_0), 28 lines when fully inflated [4, pp. 2 to 5]. Fully inflated: about 0.81 (low porosity,
  swinging ±25° to ±30° with wide lateral travel) and 0.78 (high porosity, stable) [4, pp. 6, 8].
- The report gives no speed, only Re ≈ 5.4×10⁵ for the inflated canopies [4, p. 6]: about 7.3 m/s (24 ft/s)
  (**Inference**). That sits 0.01 to 0.04 below Figure 5-16's 0.82 at 25 ft/s on a canopy of about the same size,
  despite longer lines.

## 4. Line length on small canopies

- **1 m models, 1943** (Figure 5-19a, p. 5-19): C_D0 0.62, 0.65, 0.68, 0.70, 0.72 and 0.735 at L_e/D_0 0.6, 0.7,
  0.8, 0.9, 1.0 and about 1.07 (read here). Lines of 0.6 D_0 give 0.85 of lines of 1.0. These models read low
  overall (0.72 at 1.0); their speed isn't given.
- **Figure 5-20** (p. 5-20), change against L_e/D_0 = 1.0, read here: 3.8 ft solid flat (tunnel) 0.86 at about 0.52,
  1.07 at 1.5, 1.09 at 1.9; 6.5 ft solid flat (tunnel, RAE 1944) 0.87 at 0.62, 0.80 at 0.43, 1.09 at 1.6;
  3.1 ft solid hemispherical (free flight, 1949) 0.82 at 0.59 and 1.005 at 1.24; 11 ft extended skirt (free flight)
  0.80 and 0.84 at about 0.6. Knacke: hemispherical and extended-skirt canopies gain little past 1.1 [1, p. 5-20].
- **AFTR 5867** (§2): lines of 0.60 against 0.92 D_0 gave 0.81, 0.88, 0.84 and 0.77 of the drag at nearby speeds;
  the shorter-lined drops flew 1 to 4 ft/s faster, which biases these ratios low.
- **Hanoi tunnel** (§7): flat circular −19% at 0.6 D_0 and +8.5% at 1.5 D_0, against 1.0 D_0, at 8 m/s [5, Fig. 11].
- **Taken together:** lines of 0.6 D_0 give 0.80 to 0.88 of the drag of lines of one D_0 on 3 to 12 ft canopies
  (AFTR's 0.77 sits below, against 0.92 D_0 and at a higher speed); lines of 1.5 D_0 give 1.07 to 1.09 on flat ones.
  That spread is wider than the speed effect.

## 5. Reynolds number: small, and the other way

- Cockrell's survey: above Re about 10⁵ on D_0 parachute drag varies little [2, p. 10, Figs 2.9a and 2.9b].
  In Figure 2.9a (after Jorgensen, tunnel-mounted), a hemispherical canopy reads about 0.81 at 1.0×10⁵, 0.96
  at 2×10⁵ and 0.99 at 4×10⁵ (read here; reference area not shown); a circular disc on the same rig moves
  alike, so part of the dip may be the rig (**Inference**). Figure 2.9b's flat circular rises about 0.61 to
  0.69 from 0.6×10⁶ to 4×10⁶.
- Knacke's text finds no Reynolds effect on parachute drag [1, pp. 5-29, 5-30], yet his Figure 5-29 (no C_D scale)
  shows a fabric hemisphere's drag rising from Re 10⁵ to about 3.5×10⁵.
- Hobby mains sit at 1.9×10⁵ to 1.4×10⁶. Only a 2 ft canopy at 15 ft/s nears the knee: about 5% low in C_D
  (2.5% in descent speed) by Figure 2.9a (**Inference**). Lower Re lowers C_D, against the glide effect. The
  45 in drops (Re 3.6×10⁵ to 7.2×10⁵) already include whatever Re effect acts at that size.

## 6. What it does to descent time and drift

| case at the 0.775 default | C_D0 at 15 ft/s | at 20 to 22 ft/s | descent time and drift against predicted |
|---|---|---|---|
| 45 in flat circular, lines as tested [1] | 0.91 to 0.97 | 0.83 to 0.85 | +8 to +12% at 15; +3.5 to +5% at 20 to 22 |
| same, lines 0.6 D_0 (×0.80 to 0.88) | 0.73 to 0.85 | 0.66 to 0.75 | −3 to +5% at 15; −7 to −2% at 20 to 22 |
| same, lines 1.5 D_0 (×1.07 to 1.09) | 0.97 to 1.06 | 0.89 to 0.93 | +12 to +17% at 15; +7 to +9% at 20 to 22 |
| 3.5 ft flat, lines 1.34 D_0, free in tunnel [4] | none | 0.78 to 0.81 at about 24 | 0 to +2% |

- All rows are **Inference** from the measured C_D0, with the line factors of §4 applied to the 45 in curves as if
  those were flown on lines of about one D_0, which no source confirms.
- Landing energy reads high where descent time reads short (7 to 25% in the first row): the safe side for energy,
  the flattering side for descent time and drift, as issue #741 says. The issue's 5 to 14% (from Figure 5-24 on 24 to
  100 ft canopies) becomes 3.5 to 12% on 45 in canopies. It changes sign with lines shorter than about 0.6 D_0 at
  15 ft/s and about 0.75 D_0 at 20 to 22 ft/s (Figure 5-19a's factors).
- At an altitude field a canopy flies faster than its equivalent airspeed (Knacke's Figure 5-24 axis is EAS). Which
  speed sets the glide isn't shown by any source here, so this note doesn't correct for it.

## 7. Tunnel tests of tethered small canopies: direction only

- **Hanoi, 2021** [5]: 9.1 in (0.23 m; S_0 0.042 m²) flat circular and 10% extended-skirt canopies, 16
  gores, lines 1.0 D_0, tethered to a load cell in an open-return tunnel listed as 1.0 × 0.4 × 0.5 m, at 3
  to 12 m/s [5, pp. 70, 71]. Flat circular, read here from Figure 8: 1.71, 1.28, 1.07, 0.94, 0.89, 0.85 and
  0.83 at 3, 4, 5, 6, 7, 10 and 12 m/s (the text gives 1.72 below 5 m/s and about 0.85 over 10 m/s [5, p.
  72]). The rise is larger than in free drops.
- Its limits (**Inference**): the canopy is below the 2 ft range (Re 4.8×10⁴ to 1.9×10⁵); tethered, it can't
  glide, so the cause differs (the authors blame the skirt spreading at low speed [5, p. 72]); no blockage
  correction is stated, with the drag area 17 to 36% of a 0.4 × 0.5 m section; and at 3 m/s the drag is
  about 0.4 N on a 5 kg cell. The blockage factor of 1.85 the WPI report used [7, p. 64] would cut its
  values by 24 to 40%. Direction holds; size doesn't.
- **Martins and Wehmann, 2024** [6]: a 50 cm, 1:5 semi-ellipsoidal model, tethered at 15, 17 and 25 m/s (25 m/s matches
  the full-size chute's Reynolds number at 5 m/s), C_D on the projected circle. Results 0.59, 1.35 and 0.95
  (5% spill hole) and 0.20, 0.74 and 0.72 (10%); the canopy didn't fully inflate at 15 m/s and touched the walls
  [6, Tables 3 and 4, conclusion]. No speed trend can be read from it.
- **WPI, 2006** [7]: rigid canopy-shaped cups up to about 8.6 in across (10% blockage), corrected for blockage, on
  projected area. The last of its inflation shapes (model 8) gave 1.37, 1.30 and 1.31 at 10, 18 and 25 m/s (Re
  1.8×10⁵ to 4.4×10⁵) [7, Table 4, p. 65]: a shape held still shows no speed trend, as Cockrell says.

## 8. Cross and hemispherical canopies

- **Cross:** Knacke's Figure 5-27 (p. 5-28), read here: a 13.6 ft cross in free flight is about 0.72 at 21 ft/s,
  0.68 at 25, 0.58 at 35, 0.45 at 55 and 0.43 at 60; 2.5 ft tunnel models at 50 to 75 ft/s read 0.54 to 0.68
  (lines 1.28 to 1.5 D_0), a third series 0.69 to 0.83 at 65 ft/s. A 61.3 ft cross in free flight reads 0.69 to
  0.92 at 18 to 29 ft/s, so the rise reaches crosses at full scale. Knacke notes that
  this stable canopy also gains drag at low speed [1, p. 5-27]. At 21 ft/s the cross sits at the middle of its table
  range (0.60 to 0.85); below 21 ft/s on small crosses, no data: **unbounded**.
- **Hemispherical:** no speed series on a 2 to 10 ft hemisphere was found. The 45 in quarter-spherical gained 13%
  from 30 to 15 ft/s (§1); that a hemisphere does the same is **Inference** only.

## What this suggests

- **Issue #741's direction holds on hobby-sized flat canopies, with a smaller size and a condition.** `recovery.md`
  can cite Figures 5-16 and 5-18 as 45 in model drops (C_D0 0.91 to 0.97 at 15 ft/s, 0.84 to 0.85 at 20) and say the
  0.775 default makes descent time and drift short by about 3.5 to 12% at 15 to 22 ft/s, assuming the 1949 models
  flew on lines near one D_0. With lines of 0.6 D_0 the sign is unknown (−7% to +5%).
- **A line-length input would move more than a speed curve.** Lines of 0.6 against 1.5 D_0 span 0.80 to 1.09 of C_D0,
  wider than 15 against 30 ft/s (1.15 to 1.20). A canopy record with L_e/D_0, scaled by Figure 5-20, would
  narrow the band.
- **A speed-dependent flat-circular C_D0** from Figure 5-16's 45 in curve (0.91 at 15 to 0.79 at 30 ft/s) would
  remove most of the 3.5 to 12% for lines near one D_0. Applied by EAS or true speed, it is an open choice.
- **A test pinning the error's sign** (#741's second item) can take the first row of §6: at 15 to 22 ft/s the
  default's descent time is shorter than a 0.83 to 0.97 canopy's.
- **Terms.** The NASA report is a U.S. Government work and Martins and Wehmann is CC BY 4.0: both may be cached and
  quoted with credit. Knacke's manual is approved for public release (its 1992 commercial edition is copyrighted);
  the rest are copyrighted or of unclear terms. All of these are cited only and cached privately.

## Open questions

- The 1949 models' line length and gore count (report MCREXE-672-12B); its numbers would turn §6's rows from
  Inference into measurement.
- Whether a rocket airframe hanging under a hobby main damps or feeds the swing that sets the glide.
- AFTR 5867's flat circular and conical series (about 700 drops of 27 designs); only the extended skirt was read.
- Typical L_e/D_0 on hobby kit and HPR canopies; no maker publishes it consistently.
- A hobby-size hemispherical or cross canopy measured across 10 to 25 ft/s; none found.
- Not reached: Doty's line-length report (Knacke ref. 5.5) and RAE R&M 2335 (ref. 5.6).

## Sources

All read 2026-10-29. Cached privately under the gitignored `refs/` where noted; none is committed.

1. T. W. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), pp. 5-16 to 5-20, 5-27 to 5-30,
   Figures 5-16 to 5-20, 5-27, 5-29; refs. 5.1 to 5.6 on p. 5-131. U.S. Navy report, approved for public release,
   DTIC ADA247666, cached: https://apps.dtic.mil/sti/pdfs/ADA247666.pdf
2. D. J. Cockrell, *The Aerodynamics of Parachutes*, AGARDograph 295 (1987), pp. 10 and 11, Figures 2.9a to 2.9c.
   NATO AGARD; terms unclear: cited only, cached: https://everyspec.com/NATO/NATO-ARMP/AGARD-AG-295_39039/
3. B. A. Tutt, "Fluid Structure Interaction Parachute Benchmark Models in LS-DYNA", AIAA 2013-1384, pp. 2 to 6,
   Table IV-1 (re-printing Stimler and Ross, AFTR 5867, 1960). Copyright Airborne Systems; cited only:
   https://airborne-sys.com/wp-content/uploads/2016/09/Fluid-Structure-Interaction-Parachute-Benchmark.pdf
4. S. H. Scher and I. G. Young, "Drag Coefficients for Partially Inflated Flat Circular Parachutes", NASA TN D-6423
   (1971), pp. 2 to 8, Figure 4. U.S. Government work:
   https://ntrs.nasa.gov/api/citations/19710023919/downloads/19710023919.pdf
5. Vu Dinh Quy, Le Thi Tuyet Nhung and Tran Minh Duy Dat, "Experimental Characterization of Drag Coefficient
   of an UAV Recovery Parachute", *JST: Smart Systems and Devices* 31(2), 67 to 74 (2021),
   doi:10.51316/jst.152.ssad.2021.31.2.9. License not stated: cited only:
   https://storage.googleapis.com/jst-journals/articles/ssad/152/21004_1.3m.pdf
6. A. A. Martins and C. F. Wehmann, "Impacts of the spill hole in the stability of a model rocket parachute",
   *Revista Brasileira de Física Tecnológica Aplicada* 11(2) (2024), doi:10.3895/rbfta.v11n2.18034, CC BY 4.0:
   https://periodicos.utfpr.edu.br/rbfta/article/view/18034
7. B. P. Day, M. N. Field and J. P. Gelito, "An Experimental Investigation of Aerodynamic Drag on a Round Parachute
   Canopy", WPI Major Qualifying Project ME-HJ-0502 (2006), Tables 3 and 4, pp. 63 to 65. Cited only:
   https://digitalcommons.wpi.edu/mqp-all/3855
