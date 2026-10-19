# ERA5's lowest 3 km against Santa Teresa's midday soundings

**In short: at Santa Teresa, NM, ERA5's pad-to-3 km mean temperature runs +0.11 K warm at 17 UTC (11:00 MDT), with a
per-sounding spread (1σ) of 0.17 K; over the lowest 1 km it is +0.15 K, 1σ 0.40 K.** That is 149 soundings, May to
September 2026, compared with ERA5 read the way HPR Sim reads it. At 23 UTC the bias is nil (+0.02 K, 1σ 0.21 K over
3 km); before dawn in spring the lowest 200 m read about 1 K warm. In the real-flight comparisons this moves apogee
by about −0.03% (sim low), and one flight by ±0.04–0.07%: a quarter of the per-flight scatter
[ERA5 at launch sites](era5-surface-temperature.md) §3 inferred. **How far to trust it:** it is a lower bound.
ECMWF received every 17 UTC sounding checked, so ERA5 most likely assimilated them, and the bias is within the sonde
maker's 0.2 K accuracy. It is one sonde station, not a launch field 100 km away.

## 1. The soundings, the reanalysis and the method

- **Soundings.** IGRA 2.2's 2026 file for USM00072364 (downloaded 2026-10-19, updated 2026-10-17) holds 306 soundings
  released May to September, counted by release time: 150 at 17 UTC (May 3 onward), 151 at 23–00 UTC and 5 at
  11 UTC [1]. March and April were released at 11 and 23 UTC (one at 17 UTC). The NWS listed the site on the
  GRAW DFM-17 in September 2025 [6] p. 3: temperature accuracy under 0.2 °C, height uncertainty under 8 m [7] p. 2.
- **Reanalysis.** ERA5 temperature and geopotential on the 925 to 600 hPa levels, plus 2 m temperature, surface
  pressure and surface geopotential, at 00, 11, 12, 17, 18 and 23 UTC, March 1 to October 14 (the last hour served
  on 2026-10-19), on the 0.25° points around the station, from the Copernicus Data Store [2][3]. March to July
  are final ERA5 (`expver` 0001), August on ERA5T (0005), the initial release [4]. Open-Meteo's archive serves no
  ERA5 pressure-level values: `temperature_850hPa` with `models=era5` returned nulls with no unit [9].
- **Method.** Each value is bilinear to the station (31.873° N, 106.698° W) and linear in time between the two hours
  around the moment the balloon passed each height (release time plus elapsed time). Temperature is linear in
  geopotential height between levels. Heights are geopotential on both sides, except the sonde's base: its 1,254 m is
  geometric, 1.8 m above the geopotential value (§4 gives the effect).
- **Variant A, as HPR Sim reads a file.** Every level is kept, including those below ERA5's ground, which ECMWF fills
  by extrapolation (`hpr_io::era5` drops none). ERA5's ground here is 1,271.6 m, 17.6 m above the station's 1,254 m,
  and its surface pressure 2.2 hPa below the station's.
- **Variant B.** Levels below ERA5's ground are dropped and its 2 m temperature is placed 2 m above that ground.
- **Layer mean.** The height-weighted mean from the sonde's surface to 0.2, 0.5, 1, 2 and 3 km above it, by trapezoids
  over the sonde's and ERA5's levels together; bias is ERA5 minus sonde. A log-pressure weighting, the one that sets
  hypsometric thickness, moves a sounding's difference by at most 0.072 K (lowest 200 m), a group mean by 0.015 K.
- **Screening.** Of 451 soundings released March 1 to October 13, 443 were compared: 6 fell outside the downloaded
  hours (released 00:01–00:04, 05:00–05:03 or 17:59); 1 had a gap over 1 km between temperature levels below 3 km;
  1 (October 13, 23:07) fell to the script's end-of-data cut.
- **Reproduce.** The scripts and their output stay in the gitignored `refs/research/era5-santa-teresa-midday/`.

## 2. Layer-mean temperature, by hour

ERA5 minus sonde, variant A, mean and 1σ in kelvin (this note's computation):

| release | n | sonde 0–1 km | 0–200 m | 0–500 m | 0–1 km | 0–2 km | 0–3 km |
|---|---:|---:|---:|---:|---:|---:|---:|
| 17 UTC, May–Sep | 149 | 296.3 K | −0.15 ± 0.68 | +0.18 ± 0.57 | +0.15 ± 0.40 | +0.11 ± 0.22 | +0.11 ± 0.17 |
| 23 UTC, May–Sep | 149 | 300.5 K | −0.71 ± 0.78 | −0.21 ± 0.59 | −0.06 ± 0.44 | 0.00 ± 0.29 | +0.02 ± 0.21 |
| 11 UTC, Mar–Apr | 55 | 288.7 K | +0.97 ± 0.98 | +0.18 ± 0.58 | +0.01 ± 0.41 | −0.04 ± 0.26 | −0.04 ± 0.19 |
| 23 UTC, Mar–Apr | 57 | 293.0 K | −0.40 ± 0.69 | −0.04 ± 0.40 | +0.02 ± 0.25 | +0.01 ± 0.20 | +0.02 ± 0.19 |
| 17 UTC, Oct 1–13 | 13 | 290.3 K | −0.41 ± 0.60 | −0.13 ± 0.46 | −0.05 ± 0.26 | +0.07 ± 0.27 | +0.10 ± 0.18 |

- **The 17 UTC bias is warm and real for this pairing.** Its 95% interval is ±0.07 K over 1 km and ±0.03 K over 3 km,
  widened for day-to-day correlation (lag-1 autocorrelation +0.12). By month, the 3 km bias is +0.03 K in May and
  +0.10 to +0.14 K from June to October; the 1 km bias peaks at +0.24 K in July.
- **Above the lowest few hundred meters the spread shrinks.** At 17 UTC the 1σ falls from 0.68 K over 200 m to 0.17 K
  over 3 km. The 0–1 km and 1–3 km errors are uncorrelated (r = −0.02 at 17 UTC, +0.08 at 23 UTC), and so are those
  at 850 and 700 hPa (r = −0.07 at 17 UTC and −0.10 at 23 UTC, n = 148).
- **ERA5T matches final ERA5.** At 17 UTC the 3 km bias is +0.10 ± 0.18 K on 88 final and +0.12 ± 0.16 K on 61
  initial-release soundings.
- **Only 5 summer soundings at 11 UTC** (May 1–2, September 22, 23 and 29): −0.11 K over 1 km, −0.06 K over 3 km.
- **Variant B is warmer by day.** Anchoring the bottom on the 2 m temperature gives +0.32 ± 0.41 K over 1 km at
  17 UTC, +0.17 ± 0.41 K at 23 UTC and −0.39 ± 0.37 K at 11 UTC in spring; over 3 km, +0.16, +0.09 and −0.17 K.

## 3. The pad: where ERA5 reads worst

**At the station's height, ERA5's pressure levels read 1.2–1.8 K cool by day and 3.9 K warm before dawn; its 2 m
temperature does better.** Sonde surface temperature against each, mean ± 1σ in kelvin (this note's computation):

| release | sonde surface | levels at 1,254 m (A) | 2 m temperature (B) |
|---|---:|---:|---:|
| 17 UTC, May–Sep | 29.2 °C | −1.18 ± 0.81 | +0.61 ± 0.78 |
| 23 UTC, May–Sep | 33.7 °C | −1.78 ± 1.07 | +0.05 ± 1.07 |
| 11 UTC, Mar–Apr | 13.1 °C | +3.89 ± 2.64 | +0.07 ± 1.47 |

- **Why (Inference).** The station sits below ERA5's ground, so variant A's value there is ECMWF's extrapolation from
  the air above, which misses the hot daytime surface layer and the cold pre-dawn one [4]. Its error fades within a
  few hundred meters (§2). The 2 m temperature, from the land-surface scheme, is nearly unbiased here, unlike the
  −0.3 to −2.5 K by day that era5-surface-temperature §1 cites for a Rub' al Khali site.
- **What it reaches.** HPR Sim's drag sees variant A's whole profile, and the converted heights use its layer mean,
  so the pad's error enters only through §2's layer means. OpenRocket's column anchors on ERA5's pad temperature
  (era5-surface-temperature, the opening section): if read from levels below ERA5's ground, 1–4 K enters undiluted.
- **The night layer is shallow.** The 11 UTC spring error is +0.97 K over the lowest 200 m, +0.18 K over 500 m and
  +0.01 K over 1 km, against era5-surface-temperature §3's cold-morning case of +3 K over 200 m. Its 95th percentile
  over 200 m is +2.70 K. This is the pre-dawn worst case; by an 8–9 am launch it erodes further (not measured here).

## 4. Pressure and geopotential: a 0.4 hPa offset that cancels

At 17 UTC, May–September, ERA5's pressure at the station's height is 0.45 ± 0.20 hPa below the sonde's surface
reading, and its 850 and 700 hPa surfaces lie 4.8 ± 2.0 m and 4.1 ± 1.9 m below the sonde's (n = 148 and 149). The
850–700 hPa thickness differs by only +0.75 ± 1.36 m. At 23 UTC: −0.39 hPa, −4.8 and −4.9 m, thickness +0.02 m.

- **Reading (Inference).** About 0.4 hPa at 875 hPa and 300 K is 4 m of height, so the level offsets are the surface
  offset carried up. Both are inside the DFM-17's 1 hPa and 8 m uncertainties [7] p. 2. About 40% of it is the base
  height convention (§1): with the sonde's heights lowered 1.8 m to geopotential, the offsets become −0.27 hPa and
  −3.0 and −2.3 m, and the 17 UTC layer biases rise by 0.01–0.02 K (+0.17 K over 1 km, +0.12 K over 3 km).
- **For apogee.** era5-surface-temperature §2 puts ±10 m of such an offset at ±0.011% of converted apogee and
  −0.04 to −0.07% per hPa too high in the simulated drag. If ERA5 holds the offset, its 0.45 hPa too low is +0.02 to
  +0.03% through drag, sim high (**Inference**).

## 5. Is this an independent test? No: a lower bound

- **ECMWF received the 17 UTC soundings.** WMO's data-quality monitor shows ECMWF receiving a complete Santa Teresa
  launch in each of the 29 six-hourly periods at 18 UTC from May 3 to 31, and on July 15: 1 received, 0 expected
  under its OSCAR schedule [5]. Other periods went unchecked: the monitor's API allows 100 downloads a day [5].
- **ERA5 takes its radiosondes from the WMO stream** (its in-situ table lists land TEMP soundings "provided by WMO
  WIS") and analyzes 12-hour windows, 09–21 and 21–09 UTC [4]. A 17:00 release falls inside a window, so ERA5 most
  likely assimilated it (**Inference**; whether its quality control kept each one is not public per sounding).
- **So the spread is an analysis fit.** A sonde ERA5 never saw reads 1.5–2.2 times worse at the same levels
  (era5-surface-temperature §1, an **Inference** from Taklimakan and North American statistics). Applied here, the
  0–3 km 1σ would be 0.26–0.37 K and the 0–1 km 0.6–0.9 K. A pad far from any sonde sits nearer that.
- **The sonde's own error is in both numbers.** The bias (+0.11 K) is inside the DFM-17's under 0.2 °C accuracy;
  its 1σ (0.17 K) exceeds its reproducibility (under 0.2 °C at k = 2, about 0.1 K at 1σ) [7] pp. 2, 4. A daytime
  radiation error in the sonde would look like the 17-versus-23 UTC difference; it is unbounded here.
- **Balloon drift adds almost nothing.** The balloon reaches 3 km above the station in a median 9.0 min, 2.1 km
  downwind (at most 8.3 km at 17 UTC and 11.3 km in spring). Reading ERA5 along the drift track, integrated from
  the sonde's own winds, moves the layer means by 0.01 K or less on average (1σ at most 0.015 K).
- **The grid's own scale is not measured.** ERA5's grid is 31 km [4], and the drift test sees only ERA5's resolved
  gradients. How far a pad 30–130 km from the station differs is unbounded here.

## 6. What it does to apogee

The comparison moves by −(1 − |e_D|)/T̄ per kelvin of ERA5 running warm (era5-surface-temperature, the opening
section), with |e_D| from [apogee-bias](apogee-bias.md)'s table. Paired with this note's layers (**Inference**):

| flight, launched 17 UTC | layer | T̄ | per kelvin | bias → shift | 1σ → per flight |
|---|---|---:|---:|---:|---:|
| 1,113 m, \|e_D\| = 0.51 | 0–1 km | 296.3 K | −0.165% | +0.15 K → −0.025% | 0.40 K → ±0.066% |
| 3,181 m, \|e_D\| = 0.31 | 0–3 km | 288.7 K | −0.239% | +0.11 K → −0.026% | 0.17 K → ±0.041% |
| 1,113 m, at 23 UTC | 0–1 km | 300.5 K | −0.163% | −0.06 K → +0.010% | 0.44 K → ±0.072% |

- **Against era5-surface-temperature §3's hot-desert row:** its −0.5 to +0.2 K bias and −0.05 to +0.11% shift
  hold the measured +0.11 to +0.15 K at the warm end, so the sign is "mostly sim low", not "mostly sim high". Its
  "one thin station's 850 hPa value" row (to −2.6 K) has no counterpart here: 850 hPa, about 275 m above the
  station, reads +0.47 ± 0.57 K at 17 UTC.
- **Against its per-flight 0.9–1.35 K (0.13–0.31%):** measured 0.17–0.40 K, or 0.26–0.88 K with the unassimilated
  factor, so 0.04–0.07% per flight at a sonde and up to about 0.15% at a far pad (**Inference**).
- **Against the collection's +8.73%** ([apogee-bias](apogee-bias.md)): a few hundredths of a percent, against it.

## What the project can use

- **IGRA 2.2** is a NOAA NCEI product with no use restriction stated in its documentation [1]: cache under `refs/`;
  derived numbers may be published.
- **ERA5** is CC BY 4.0 [2][3]: cuts may be cached and derived numbers published with the Copernicus attribution
  (Sources). This note's ERA5 cut is 4 × 3 points, 11 levels, 6 hours a day, March 1 to October 14, 2026.
- **WMO's WDQMS** states no terms on its API page [5]: cite only. Its rows for May sit in `refs/` for checking.
- **RocketPy's Spaceport America file** (`spaceport_america_pressure_levels_2023_hourly.nc`) spans June 19–24, 2023,
  when Santa Teresa released only near 23 and 11 UTC (nominal 00 and 12 UTC); it was not used.

## What this suggests

1. **Update [apogee-bias](apogee-bias.md) §3's ERA5 item**, already asked for in the research carry-over
   [issue #704](https://github.com/nrdptel/fusionspace-eridanus/issues/704): near a sonde, ERA5's temperature moves
   real-flight apogees by about −0.03% at midday and ±0.04–0.07% per flight, not 0.35% per kelvin.
2. **Set era5-surface-temperature §3's hot-desert row** to a measured bias of +0.11 to +0.15 K, sim low by
   0.02–0.03%, with the per-flight 1σ split: 0.17–0.40 K at a sonde, about 0.3–0.9 K far from one.
3. **Weather dispersion for [M2.8, honest uncertainty](../ROADMAP.md):** draw the 0–1 km and 1–3 km layer errors
   independently, not as one column-wide offset: about 0.4 K and 0.2 K at a sonde site, about twice that elsewhere.
   A pad value read from pressure levels below ERA5's ground errs by 1–2 K by day and about 4 K before dawn.
4. **[Issue #703](https://github.com/nrdptel/fusionspace-eridanus/issues/703)** (this measurement) could close on a
   committed report built by this method, with the ERA5 attribution; its done-when also needs M2.8 to cite it.
5. **For OpenRocket's column**, if its pad temperature comes from the pressure levels, anchor it on ERA5's 2 m
   temperature when the pad sits below ERA5's ground: here the pad error falls from −1.2 to −1.8 K to +0.05 to
   +0.61 K by day.

## Open questions

- An unassimilated test: a sounding ERA5 never received (a field campaign, or a WDQMS period marked not received),
  to replace the 1.5–2.2 factor with a measurement.
- ECMWF's receipt for June to September 18 UTC periods (one day checked); ERA5's quality-control flags per sounding.
- The DFM-17's daytime radiation correction: does it explain the 17-to-23 UTC gap (0.09 K over 3 km, 0.21 K over 1 km)?
- How the 0.25° grid misses at a launch field 30–130 km away, such as Spaceport America, by hour.
- The near-ground layer at 8–10 am, between the 11 UTC (05:00 MDT) and 17 UTC releases.

## Sources

Read 2026-10-19 unless noted. Downloads and scripts are in the gitignored `refs/research/era5-santa-teresa-midday/`.
Numbers from ERA5 contain modified Copernicus Climate Change Service information 2026; neither the European
Commission nor ECMWF is responsible for any use made of it.

1. NOAA NCEI, Integrated Global Radiosonde Archive 2.2: `data-y2d/USM00072364-data-beg2026.txt.zip` (dated
   2026-10-17) and `doc/igra2-data-format.txt` (header and data record formats; source codes `ncdc-nws`,
   `ncdc-gts`). <https://www.ncei.noaa.gov/data/integrated-global-radiosonde-archive/>
2. Copernicus Climate Data Store, "ERA5 hourly data on pressure levels from 1940 to present", CC BY 4.0, retrieved
   through its API on 2026-10-19. <https://doi.org/10.24381/cds.bd0915c6>
3. Copernicus Climate Data Store, "ERA5 hourly data on single levels from 1940 to present", CC BY 4.0, retrieved
   2026-10-19. <https://doi.org/10.24381/cds.adbb2d47>
4. ECMWF, "ERA5: data documentation", last modified 2026-09-08, copy of 2026-10-16: data update frequency (ERA5T,
   `expver`), the 4D-Var windows, Table 15 (in-situ data), pressure levels below the surface, the 31 km grid.
   <https://confluence.ecmwf.int/display/CKB/ERA5%3A+data+documentation>
5. WMO WIGOS Data Quality Monitoring System, NWP, TEMP, six-hourly availability, ECMWF, 18 UTC periods of
   2026-05-03 to 2026-05-31 and 2026-07-15; and its API documentation (rate limit; no terms stated).
   <https://wdqms.wmo.int/nwp/land_upper-air>,
   <https://confluence.ecmwf.int/spaces/WIGOSWT/pages/560488542/10.+Application+Programming+Interface+API+documentation>
6. NWS, "Upper Air Sites Ground Equipment and Radiosonde Types" (09-25), p. 3, row 74 (KEPZ); the PDF of that
   name in <https://www.weather.gov/media/upperair/Documents/>
7. GRAW, DFM-17 datasheet DB-DFM-17-EN V01.06, 2021-09-21, p. 2 (temperature, pressure, height), p. 4 (k = 2).
   <https://www.weather.gov/media/upperair/Documents/DB-DFM-17-EN_V01.06.pdf>
8. WMO OSCAR/Surface, station 0-20000-0-72364 (Santa Teresa, NM: 31.8728° N, 106.6981° W, 1,254 m).
   <https://oscar.wmo.int/surface/rest/api/search/station?wigosId=0-20000-0-72364>
9. Open-Meteo Historical Weather API, query of 2026-10-19 for 2026-06-01 at the station with `models=era5`.
   <https://archive-api.open-meteo.com/v1/archive>
