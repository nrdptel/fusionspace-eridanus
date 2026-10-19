# How deep ERA5's warm near-ground layer runs at sunrise over dry land

**In short: over dry inland ground ERA5's dawn warm error sits in the lowest ~100 m, not 200–300 m, and is gone 1–2 h
after sunrise; through the real-flight comparisons it moves apogee by a few hundredths of a percent at any morning
hour.** At 2,672 soundings from six inland Australian stations, 0.2–4.2 h after sunrise, ERA5 read as HPR Sim reads
it is +2.3 K warm at the station in the first hour, crosses zero between 50 and 100 m and is 0.3 K cool at 150–200 m:
+0.74 K over the lowest 100 m, +0.22 K over 200 m, +0.04 K over 1 km. On cold mornings (surface below 8 °C) the
lowest 200 m read +0.47 K (95th percentile +1.81 K), not the +3 K [ERA5 at launch sites](era5-surface-temperature.md)
§3 assumed; by 8 am they read 0.4 K cool (cold mornings −0.1 K). A point-mass model puts 16% of a 1,140 m flight's
drag sensitivity in its lowest 200 m (5% at 3,170 m, 63% at 440 m); for a warm layer of one sign, drag offsets 0.26–0.47
of the height piece. **How far to trust it:** sign and depth are firm (both stations sampled in the first hour agree;
six from 1 h on); the sizes are lower bounds, as ERA5 most
likely assimilated these soundings; no US field is in the sample; the flight shares are computed, subsonic only.

## 1. What published work gives, and what it doesn't

- **The mechanism and sign, with no ERA5 size.** ECMWF's model mixes stable air more than others, which deepens the
  stable layer [3] ¶[2]; runs with less mixing are colder near the ground and warmer aloft [3] ¶[21]–[22], so the
  extra mixing's sign is warm low, cool higher up (**Inference**; at Cabauw the model was still about 1 K too cold
  near the surface at night, ¶[14], [22]). Its stable mixing length was 150 m, against 30–40 m elsewhere [3] ¶[9];
  since November 2013 it is 10% of the boundary-layer height [4] §3.3, pp. 45–46, likely in ERA5's cycle 41r2 [5].
- **The morning, with no ERA5 size.** In GABLS2, a clear CASES-99 day in Kansas in 30 model runs, the morning heat
  flux rose too weakly and the mixed layer grew too slowly [6] abstract; too cool low by mid-morning (**Inference**).
- **The 2 m air.** At a Rub' al Khali site ERA5's median night 2 m bias is +0.2 to +2.9 K, its daily extremes about
  an hour late [7] p. 12, Figs. 1–2; Shiraz and Isfahan at 00 UTC, +3.07 to +5.48 K at 850 hPa [8] Table 4 (parent §1).
- **No evaluation of ERA5's profile between sunrise and noon over dry land was found** besides
  [Santa Teresa](era5-santa-teresa-midday.md)'s (05:00 MDT in spring: +0.97 K over 200 m, +0.01 K over 1 km).
  Taklimakan [9] §3.1–3.2 and the Mongolian Plateau [10] §3.4 give no split by hour.

## 2. The soundings and the method

- **Soundings** [1]: Alice Springs, Giles, Meekatharra, Woomera, Mount Isa and Cobar (167–580 m), dry inland
  stations whose morning release at 22:00–23:47 UTC falls at 06:56–09:28 local solar time: the hours US soundings
  miss. Of 2,749 releases from January 2025 to October 13, 2026, 2,672 are used; 41 had no surface level, 16 no
  significant temperature levels, 2 stopped below 1 km and 18 fell outside the ERA5 download. Median surface relative
  humidity at release is 39–62% by station.
- **Heights** are hypsometric on virtual temperature from the station (−2.9 ± 4.7 m against 8,006 archived 925–700
  hPa heights); temperature is linear between levels, 5–6 of them below 1 km in the median sounding.
- **ERA5** [2]: temperature and geopotential on the twelve levels from 1000 to 700 hPa, plus 2 m temperature, surface
  pressure and orography, at 22, 23 and 00 UTC, bilinear to the station and linear in time. **Variant A** keeps every
  level, those below ERA5's ground included, as `hpr_io::era5` reads a file; ECMWF fills those by extrapolation from
  the lowest model level [5]. **Variant B** keeps the levels above ERA5's ground and anchors the bottom on the 2 m
  temperature. ERA5's ground sits 37 m below to 53 m above the stations'. Sunrise is from NOAA's solar equations at a
  zenith of 90.833° [11].

## 3. ERA5 less the sonde, by hours after sunrise (variant A, K)

| after sunrise | n | sonde surface | at station | 50 m | 100 m | 200 m | 300 m | 500 m | 1 km |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0–1 h (mean 0.61) | 334 | 11.9 °C | +2.25 | +0.56 | −0.13 | −0.35 | −0.11 | +0.01 | +0.05 |
| 0–1 h, surface < 8 °C | 88 | | +4.40 | +1.14 | −0.15 | −0.64 | −0.21 | +0.01 | +0.17 |
| 1–2 h (1.53) | 1,068 | 17.7 °C | −0.60 | −0.45 | −0.43 | −0.22 | −0.01 | +0.14 | −0.05 |
| 2–3 h (2.42) | 927 | 23.1 °C | −1.38 | −0.39 | −0.11 | +0.08 | +0.23 | +0.34 | +0.08 |
| 3–5 h (3.41) | 343 | 26.3 °C | −1.67 | −0.12 | +0.13 | +0.29 | +0.38 | +0.38 | +0.11 |

Layer means, with the range of station means (stations with 15 or more soundings in the bin) in brackets:

| after sunrise | 0–100 m | 0–200 m | 0–500 m | 0–1 km | 1σ, 0–100 m / 0–1 km |
|---|---:|---:|---:|---:|---:|
| 0–1 h | +0.74 (+0.39 to +1.16) | +0.22 (+0.02 to +0.45) | +0.02 | +0.04 | 1.18 / 0.28 |
| 0–1 h, surface < 8 °C | +1.48 | +0.47 (p95 +1.81) | +0.07 | +0.14 | |
| 1–2 h | −0.46 (−0.94 to +0.23) | −0.40 (−0.73 to +0.05) | −0.15 | −0.06 | 1.21 / 0.27 |
| 2–3 h | −0.50 (−0.83 to −0.21) | −0.25 (−0.53 to +0.01) | +0.06 | +0.13 | 0.92 / 0.31 |
| 3–5 h | −0.34 (−0.67 to −0.09) | −0.06 (−0.32 to +0.15) | +0.20 | +0.24 | 0.86 / 0.35 |

- **Depth.** At sunrise the sonde's inversion tops out at a median 190 m, 3.1 K above the surface; ERA5's (variant B)
  at 280 m, 2.2 K. ERA5 is warm below about 100 m, 0.3–0.6 K cool at 150–200 m and 0.1–0.2 K cool at 300 m: a weaker,
  deeper inversion, the overmixing sign of §1. Above 500 m the dawn error stays under 0.25 K in every subset.
- **Erosion.** The warm layer is gone 1–2 h after sunrise, pooled and at each dawn station at a fixed solar time
  (winter's later sunrise): over 0–100 m Alice Springs goes from +1.16 to −0.10 K and Meekatharra from +0.39 to
  −0.69 K between the first and second hour. By 2–3 h the air at 300–700 m reads +0.2 to +0.4 K warm.
- **The cool reading near the ground is partly the reading method (Inference).** At 2–3 h variant A is −1.38 K at the
  station but ERA5's 2 m temperature only +0.03 K: extrapolating below ERA5's ground misses the hot surface layer, as
  at Santa Teresa by day. Variant B's 0–200 m means are −0.44 K at dawn and +0.26 K at 2–3 h.
- **The cool 150–300 m band at dawn is not split by cause (Inference):** levels 25 hPa apart (about 230 m) cannot
  hold a warm nose at 190 m, and overmixing also cools it there.

## 4. Best estimate by launch hour (ERA5 less truth, K; Inference)

On daylight time, sunrise is 05:24–07:37 at four US fields (Santa Teresa, Black Rock, Lucerne, Argonia; March 21 to
October 15) [11]. With sunrise at 6–7 am, 8 am falls 1–2 h after it, 9 am 2–3 h, 10 am 3–4 h and 11 am 4–5 h; an
early summer sunrise moves a launch one row later. Each cell is a mean, then a range drawn from the station means,
the cold-morning subset and Santa Teresa.

| local time | 0–100 m | 0–200 m | 0–500 m | 0–1 km |
|---|---:|---:|---:|---:|
| before dawn (Santa Teresa, 05:00) | not computed | +1.0 (one station) | +0.2 | 0.0 |
| sunrise to +1 h | +0.7 (+0.4 to +1.5) | +0.2 (0 to +0.5) | 0.0 (−0.1 to +0.15) | 0.0 (−0.05 to +0.15) |
| 8 am | −0.5 (−0.9 to +0.7) | −0.4 (−0.7 to +0.2) | −0.15 (−0.25 to 0) | −0.05 (−0.15 to +0.05) |
| 9 am | −0.5 (−0.8 to −0.2) | −0.25 (−0.55 to 0) | +0.05 (−0.2 to +0.2) | +0.1 (−0.1 to +0.2) |
| 10 am | −0.35 (−0.7 to −0.1) | −0.05 (−0.3 to +0.15) | +0.2 (+0.05 to +0.3) | +0.25 (+0.15 to +0.3) |
| 11 am | −0.3 (−0.7 to 0) | −0.1 (−0.3 to +0.15) | +0.2 (+0.05 to +0.3) | +0.2 (+0.15 to +0.3) |

The 8 am upper ends allow for a late spring or fall sunrise, when 8 am falls in the first hour. The 11 am row is
the 3–5 h bin with Santa Teresa's 17 UTC means, as few Australian soundings lie past 4 h. **Per flight the scatter
outweighs every mean:** 1σ about 1 K over 0–100 m, 0.7–0.9 K over 0–200 m and 0.3 K over 1 km.

## 5. How much of a flight's drag loss happens low (computed with a point-mass model)

- **The quantity.** w(h) is apogee's sensitivity to the air density below h above the pad, as a share of that over
  the whole climb (±1% central differences); the whole is [apogee-bias](apogee-bias.md)'s drag elasticity e_D.
- **The model.** A point mass under thrust, drag ½ρv²C_D(M)A and gravity, RK4 at 1 ms (halving the step moves no
  apogee by 0.001 m), no wind or angle of attack. C_D follows RocketPy's Calisto drag curve [13], times one factor per
  rocket fitted to its published apogee; vertical, but planar off an 84.7° rail for (b). The rockets: (a) the guide's
  56.3 mm rocket on an H54, 1,145 m [14]; (b) RocketPy's Calisto on a Cesaroni M1670, 3,181 m [12][13]; (c) the
  guide's rocket on an F52C, 439 m [14]; motor curves from ThrustCurve.org [15]. Rail exit and top speeds match the
  guide: 21.7 and 186 m/s for (a), against 21.7 and 187; 18.6 and 106 for (c), against 18.5 and 106.
- **Check against apogee-bias.** e_D is −0.517 for (a) (there −0.51) and −0.349 for Calisto (−0.31). Calisto's
  3,181 m was flown in forecast wind on a 5,700 N s mean and (c)'s 439 m in 5 m/s; here both fly calm, and the
  fitted C_D factor (1.107, 1.170) absorbs the difference, Calisto's curve integrating to 6,026 N s.

| rocket | apogee | w(100 m) | w(200 m) | w(300 m) | w(500 m) | w(1 km) | height share below 200 m |
|---|---:|---:|---:|---:|---:|---:|---:|
| (a) H54, 54 mm | 1,140 m | 0.058 | 0.156 | 0.276 | 0.537 | 0.971 | 0.175 |
| (b) Calisto, M1670 | 3,173 m | 0.016 | 0.046 | 0.086 | 0.189 | 0.462 | 0.063 |
| (c) F52C, 54 mm | 438 m | 0.307 | 0.632 | 0.867 | 1.000 | 1.000 | 0.457 |

- **The parent's guess,** w(200 m) = 0.15–0.4, holds only at its low end, for a 1 km flight. At the same impulse, a
  burn a third as long raises w(200 m) by about half (0.23 for (a), 0.07 for (b)); one twice as long, down a tenth.
- **When drag wins.** For a warm layer of one sign, the drag piece is |e_D|·w(h) times the air's thinning and the
  height piece the layer's height share times the same: 0.26–0.47 of it for the uniform 200 m layer (at most ~0.8 from
  the bounds). When the error changes sign with height, drag can win (§6's 2–3 h row for (c)).

## 6. The cold-morning term, sized

The comparison's shift is the parent's net = (1 + d)/(1 + ε) − 1, d the simulated apogee's change and ε the converted
log height's (ERA5 too warm makes HPR Sim look low). Percent of apogee, computed with the point-mass model.

| ERA5 error | (a) 1,140 m | (b) 3,173 m | (c) 438 m |
|---|---:|---:|---:|
| the parent's +3 K, uniform over the lowest 200 m, 277 K | −0.10 | −0.05 | −0.27 |
| the same at the cold-sunrise 95th percentile, +1.81 K (scaled) | −0.06 | −0.03 | −0.16 |
| §3's profile, first hour after sunrise | −0.013 | −0.004 | −0.007 |
| §3's profile, first hour, surface < 8 °C | −0.030 | −0.011 | −0.020 |
| §3's profile, 1–2 h (about 8 am) | +0.011 | +0.006 | +0.037 |
| §3's profile, 1–2 h, surface < 8 °C | −0.002 | 0.000 | +0.016 |
| §3's profile, 2–3 h (about 9 am) | −0.010 | −0.003 | −0.008 |
| §3's profile, 3–5 h (10–11 am) | −0.021 | −0.009 | −0.043 |

- **The profile rows** take §3's variant-A values, linear between heights, zero above 1 km, at 280 K, each band's
  mean error weighted by its share of w (`combine.py`); the 1–3 km layer is left out.
- **The parent's cold-morning row** (+3 K over 200 m, −0.17 to +0.03%, "either") becomes −0.03% (sim low) on the mean
  cold sunrise for a 1 km flight, −0.06% at the 95th percentile, and about 0 by 8 am. Its other cold row (+0.1 to
  +0.55 K through the column, −0.02 to −0.14%) holds at 10–11 am, where the 0–1 km mean is +0.2 to +0.25 K.
- **Per flight,** 1σ 0.3 K over 1 km moves a 1 km flight ±0.05%, more than the mean term at any hour (**Inference**).

## What this suggests

1. **Replace era5-surface-temperature §3's "+3 K in the lowest 200 m" row** with the measured cold-sunrise layer:
   +0.47 K over 200 m (95th percentile +1.81 K), −0.03% for a 1 km flight, sim low, and near 0 by 8 am: a few
   hundredths of a percent at most, against the collection's +8.73% bias.
2. **Weather dispersion for [M2.8, honest uncertainty](../ROADMAP.md):** a near-ground layer whose mean depends on the
   hours after sunrise (§4) and whose 1σ is about 1 K over the lowest 100 m and 0.3 K over 1 km, separate from the
   1–3 km layer; draw the 0–100 m and 100–1,000 m errors with opposite-signed means at dawn.
3. **A pad value read from levels below ERA5's ground** errs by +2 to +4 K at sunrise and −1.4 to −1.7 K 2–5 h
   later, while ERA5's 2 m temperature is within about 0.03 K at 2–3 h: the same case as Santa Teresa's suggestion 5
   for OpenRocket's column.
4. **Short flights feel it most.** A 440 m flight has 63% of its drag sensitivity in the lowest 200 m; any near-ground
   correction matters most for mid-power flights, least for 3 km ones.

## Open questions

- An unassimilated test: campaign sondes over dry US land at 7–11 am (MATERHORN at Dugway, ARM SGP's dawn periods).
  ECMWF's receipt of these Australian soundings was not checked (the WMO receipt service refused further requests).
- Is the cool 150–300 m band at dawn level spacing or overmixing? ERA5's model levels at the same releases would tell.
- Basins: Alice Springs, 53 m below ERA5's ground, reads warmest (+3.52 K at the station at dawn).
- w for a supersonic flight such as Project R (e_D −0.57), over a real 5–10 K inversion, with wind.
- The releases fall 1.0–2.8 h after ERA5's 21 UTC assimilation-window change [5]; any seam effect is untested.

## Sources

Read 2026-10-19; copies, scripts, outputs in the gitignored `refs/research/era5-dawn-inversion-depth/`. Terms: [1]
no restriction stated; [2], [7], [9], [10] CC BY 4.0; [3] open access; [6] CC BY-NC 2.0; [12] MIT, its data files
cited and cached only; [15] public domain; the rest cited only. Numbers from ERA5 contain modified Copernicus Climate
Change Service information 2026; neither the European Commission nor ECMWF is responsible for any use made of it.

1. NOAA NCEI, IGRA 2.2, ASM00094326, 94461, 94430, 94659, 94332, 94711: `data-por/`, `data-y2d/` (2026-10-17).
   <https://www.ncei.noaa.gov/data/integrated-global-radiosonde-archive/>
2. Copernicus Climate Data Store, ERA5 hourly data on pressure levels <https://doi.org/10.24381/cds.bd0915c6> and on
   single levels <https://doi.org/10.24381/cds.adbb2d47>; 20.25–32° S, 118–146.25° E, 2025-01-01 to 2026-10-14,
   `expver` 0001 and 0005.
3. Sandu et al., "Why is it so difficult to represent stably stratified conditions in numerical weather prediction
   (NWP) models?", J. Adv. Model. Earth Syst. 5, 117–133 (2013). <https://doi.org/10.1002/jame.20013>
4. ECMWF, IFS Documentation Cy43r1, Part IV: Physical Processes. <https://www.ecmwf.int/en/elibrary/17117>
5. ECMWF, "ERA5: data documentation", copy of 2026-10-16.
   <https://confluence.ecmwf.int/display/CKB/ERA5%3A+data+documentation>
6. Svensson et al., "Evaluation of the diurnal cycle in the atmospheric boundary layer over land as represented by a
   variety of single-column models: the second GABLS experiment", Bound.-Layer Meteorol. 140, 177–206 (2011).
   <https://doi.org/10.1007/s10546-011-9611-7>
7. Nelli et al., Remote Sens. 16, 3593 (2024), a Rub' al Khali campaign. <https://doi.org/10.3390/rs16193593>
8. Pegahfar, J. Earth Space Phys. 51(3), 719–737 (2025). <https://doi.org/10.22059/jesphys.2025.400092.1007713>
9. Ma et al., Taklimakan radiosondes, Remote Sens. 15, 2925 (2023). <https://doi.org/10.3390/rs15112925>
10. Wei et al., Mongolian Plateau, Remote Sens. 17, 393 (2025). <https://doi.org/10.3390/rs17030393>
11. NOAA GML, "General Solar Position Calculations". <https://gml.noaa.gov/grad/solcalc/solareqns.PDF>
12. RocketPy v1.13.0 docs, *First simulation* (masses, radius, rail) and *Sensitivity analysis* (3,181 m):
    <https://docs.rocketpy.org/en/latest/user/first_simulation.html>, `.../user/sensitivity.html`.
13. RocketPy repository, `data/motors/cesaroni/Cesaroni_M1670.eng` and `data/rockets/calisto/powerOffDragCurve.csv`.
    <https://github.com/RocketPy-Team/RocketPy>
14. HPR Sim's guide: `docs/your-own-rocket.md` (56.3 mm, 0.675 kg, 1,144.5 m) and `docs/the-builder.md` (F52C).
15. ThrustCurve.org curves 168H54-10A and F52C, as bundled in `crates/hpr-motor/data/thrustcurve/curves/`.
