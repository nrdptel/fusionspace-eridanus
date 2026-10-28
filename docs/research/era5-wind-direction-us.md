# ERA5's wind vector and direction error near the ground over US land

**In short: no published study scores ERA5's hourly wind vector or direction by height against a US instrument
it did not assimilate; what exists brackets a per-component error of about 1.4–2.4 m/s in the lowest 3 km, a
boundary-layer speed 3–16% slow over the Plains and the West, and a direction error set mostly by how light the
wind is.** At soundings, which ERA5 assimilates [7, Table 15], its 0–1 and 0–3 km shear magnitude misses by 2.6
m/s RMS [4, Table 3]. The ECMWF model's 12-hour background, a field not yet drawn to the sonde, misses US sondes
by an RMS vector of about 2.6–3.4 m/s at 1000–700 hPa [9, Fig. 3.5]. Against lidars and towers ERA5 did not use,
its 100 m speed runs 1.0–1.5 m/s slow on the Oklahoma plains at every hour [3, Fig. 9b]. Direction error is about
σ/U radians [6, Eq. 9]; for σ = 1.5–2.5 m/s that is a mean absolute 14–26° at 5 m/s and 39–56° at 2 m/s
(**Inference**, §4), so at a light-wind launch ERA5 says little about direction. **How far to trust it:** every
source is primary, but the vector numbers come from soundings, from the operational model rather than ERA5, or
from one Oklahoma site; ERA5's own hourly vector error away from soundings, and between its hours, are measured
by no source read. It moves M7.3's wind prior and the 100–400 m drift uncertainty.

Builds on [wind-from-descent](wind-from-descent.md) §5 (Taszarek's speed RMSEs, tower speed scores, ERA5's
ensemble spread). Researched 2026-10-28.

## 1. What ERA5 assimilates over the US, and so which checks are independent

- **Upper air: almost everything.** ERA5 takes radiosonde and PILOT profiles, the American wind profiler
  network's radar winds and aircraft winds (AIREP, AMDAR, ACARS, TAMDAR, Mode-S) [7, Table 15]. AMDAR and the
  NOAA profilers are not independent checks, and "far from a sonde" is not "far from upper-air data": aircraft
  climbing out of hub airports are assimilated too (**Inference**).
- **Land surface: no wind.** SYNOP and METAR stations give ERA5 only pressure, temperature and humidity; its 10 m
  wind comes from ships and buoys [7, Table 15]. US station scores of 10 m wind are therefore independent.
- **Assigned observation errors, not ERA5's.** The model cycle gives profiler and PILOT u and v an RMS error of
  1.80 m/s at 1000 and 850 hPa and 1.90 m/s at 700 hPa, AMDAR and ACARS 2.46–2.56 m/s [8, Table 2.6, p. 42].
- **ARM's Southern Great Plains (SGP) site, Oklahoma.** None of its lidars were assimilated; the central
  facility's four-a-day sondes were, so one study dropped the central lidar for four lidars on a square about 60
  km on a side around it [3, p. 15]. Two other studies use the central lidar, 250 m from the sonde launch [1, p.
  2082], so their SGP scores sit beside an assimilated sounding.

## 2. How big the vector miss is

| Comparison | What it measures | Size (m/s) | Source |
|---|---|---|---|
| ERA5 at soundings, 1980–2018 | Speed RMSE, 0–1 / 1–3 km mean wind | 1.93 / 1.42 | [4, Table 3] |
| Same | Shear magnitude RMSE, 0–1 / 0–3 km | 2.62 / 2.60 | [4, Table 3] |
| IFS 12 h background, US LMS6 sondes, 2015–16 | RMS vector O−B, 1000–700 hPa | about 2.6–3.4 | [9, Fig. 3.5] |
| IFS day-ahead vs SGP lidar, 2012–20 | Mean absolute vector error | <3 day; >4 at times by night | [1, p. 2085] |
| 915 MHz profiler vs GPS balloons | RMS vector difference | 2.0–3.5 | [5, p. 14] |

- **At the sonde: a floor.** Taszarek et al. score ERA5 at the hour and grid point of 3.7 million soundings from
  232 North American and European stations [4, Table 3, p. 3221]. North America's are 97% at 00 and 12 UTC
  (756,218 and 791,577 of 1,596,795) [4, Table 1, p. 3216]. Shear is the size of the vector difference between
  10 m and the named height, so its 2.6 m/s RMSE carries some direction error that the speed scores (1.4–1.9 m/s)
  leave out. MERRA-2 misses the same shears by 3.02–3.18 m/s [4, Table 3]. **Inference:** ERA5 assimilated these
  soundings [7, Table 15], so the scores flatter it.
- **Away from the sonde: the background.** Before it is drawn to a sonde, the operational model's 12-hour
  forecast misses 20–50° N sondes by an RMS vector of about 2.6–3.6 m/s at 1000–700 hPa across sonde types; for
  the US LMS6 sonde about 2.6, 3.2, 3.3 and 3.4 m/s at 1000, 925, 850 and 700 hPa (read from the figure) [9, Fig.
  3.5, p. 16]. That model ran at about 16 km until March 2016 and about 9 km after, finer than ERA5's 31 km.
  **Inference:** removing a GPS sonde's error (well under 1 m/s) leaves about 1.8–2.4 m/s per component. A field
  50–150 km and some hours from a sonde should sit between the at-sonde score and this.
- **Withholding the local sondes changed little in steady flow** over the Barbados trades: differences "almost
  nonexistent below 2 km" [10, p. 13056]. US convective or frontal weather is untested.
- **The closest US hourly vector score is the operational model, not ERA5.** At SGP, the IFS day-ahead forecast's
  mean absolute vector error is under 3 m/s in the daytime boundary layer and can exceed 4 m/s at night and above
  the boundary layer; the low-level jet core runs about 2 m/s slow [1, pp. 2085, 2088]. **Inference:** for an
  unbiased isotropic error, σ per component is the mean absolute vector error over 1.2533: below 2.4 m/s by day,
  above 3.2 m/s on the worst nights. An analysis should miss by less than a 12–35 h forecast, a coarser grid by
  more; no source sizes either.
- **Two good instruments disagree by as much.** A 915 MHz profiler on the US Western Range against 85 GPS
  balloons, released at the profiler or 3.7 km away and within 7.5 minutes, differs by 1.5–3.0 m/s per component
  and 2.0–3.5 m/s as a vector [5, pp. 2–5, 14]. The authors treat this as an upper bound on the profiler's own
  error. **Inference:** any point check of ERA5 includes a term of this size, and so does a launch.

## 3. Speed bias against instruments ERA5 did not use

Daily-mean 100 m speed, ERA5 minus observed, where ERA5 assimilated none of the winds [3, p. 5; Fig. 10, p. 17]:

| Data set (years) | Bias (m/s) | r | Normalized bias |
|---|---|---|---|
| ARM SGP, four lidars (2016–21) | −1.29 | 0.98 | −16% |
| Iowa Atmospheric Observatory tower (2016–21) | −1.23 | 0.95 | −15% |
| WFIP1 northern Plains towers / sodars (2011–12) | −1.14 / −1.39 | 0.98 / 0.92 | −14% / −16% |
| WFIP2 Columbia basin, away from the Gorge (2015–17) | −0.73 | 0.96 | −14% |
| WFIP1 Texas towers / sodars (2011–12) | −0.25 / −0.54 | 0.97 / 0.97 | −3.3% / −7.2% |
| New York Mesonet lidars (2017–21) | −0.095 | 0.97 | −1.8% |
| Indiana lidar (2016–19) | +0.22 | 0.96 | +3.9% |

- Across six networks the mean speed bias is −12% [3, p. 19]. Wind-farm wakes lower some SGP lidars' stable-night
  speeds by 4–6%, so ERA5's true low bias there is if anything larger [3, p. 19]. These are daily means of
  speed: they bound the bias, not the hourly vector error.
- **By hour it barely moves.** At SGP (read from the figure) ERA5 runs about −1.5 m/s at 05 UTC, the jet's peak,
  and about −1.05 m/s at 17–18 UTC; the authors call the difference "nearly constant across the diurnal cycle"
  [3, Fig. 9b, pp. 15–16]. One SGP lidar in 2018, at 65–195 m, agrees: about −1.5 m/s, negative in all months and
  hours, with the night jet's daily amplitude well captured [2, pp. 493, 498].

## 4. Direction

- **Direction error is vector error over speed.** For component errors σ_u = σ_v = σ, the direction error is
  σ/U radians [6, Eq. 9, p. 1233]. A Monte Carlo (true wind U along x, independent Gaussian errors σ on each
  component, 200,000 draws) gives the mean absolute / RMS direction error in degrees (**Inference**; re-run
  twice independently, agreeing within 1°):

| σ (m/s) | U = 2 m/s | 3 | 5 | 8 | 12 |
|---|---|---|---|---|---|
| 1.5 | 39 / 53 | 26 / 35 | 14 / 18 | 9 / 11 | 6 / 7 |
| 2.0 | 49 / 64 | 35 / 47 | 20 / 26 | 12 / 15 | 8 / 10 |
| 2.5 | 56 / 72 | 43 / 57 | 26 / 35 | 15 / 19 | 10 / 12 |

  A uniformly random direction averages 90°. The model ignores bias and any change of σ with speed.
- **The instrument check agrees.** The profiler-balloon RMS direction difference is 10–35°, and 10–20° when the
  wind exceeds 4 m/s [5, pp. 14, 24–25]; σ = 1.5 m/s at 5 m/s gives 18°. Large differences cluster below 4 m/s
  [5, Fig. D1, p. 24]. The SGP lidar study leaves out winds under 2 m/s because "the wind direction uncertainty
  increases rapidly" [1, p. 2084], so the light-wind case a hobby launch cares about is not scored there.
- **10 m over the US.** WeatherReal maps ERA5's direction RMSE against ISD stations' 2023 reports (stations
  reporting every 1–24 h), ERA5 interpolated to each station: about 40–50° over the central Plains and upper
  Midwest, 55–85° over the Southeast and Appalachians, 70–90° or more over much of the West (read from the map)
  [13, Fig. 8f, p. 14]. Its speed RMSE is about 1.4–1.7 m/s in the East and 2.0–2.8 m/s over the western Plains
  and Rockies [13, Fig. 8e]. It does not say how calms (coded direction 0 [13, p. 3]) or the 360° wrap are
  handled, which can inflate a direction RMSE (**Inference**). With σ = 1.5–2.4 m/s and 10 m winds of 2–5 m/s,
  the table gives 18–70° RMS, the same range.
- **Bin by the model's value.** HRRR against 733 ASOS stations (April 2019): speed bias +0.85 m/s when the
  observed speed is under 2.5 m/s and −1.38 m/s above 5 m/s, yet centered on zero when binned by forecast speed
  [14, p. 2267]. Fovell and Gallagher score no direction. **Inference:** an error table built for a prior should
  be binned by ERA5's speed, the value the simulator has, or it inherits this selection effect.

## 5. Between ERA5's hours

- **Windows and look-ahead.** ERA5's 4D-Var windows run 09–21 and 21–09 UTC, and its accuracy "is not uniform
  throughout each window" [7]. Its 00 and 12 UTC states see 9 hours of later observations, 06 and 18 UTC only 3
  [11, p. 6]. With 97% of North American soundings at 00 and 12 UTC (§2), the at-sonde scores are for the
  best-observed hours.
- **A seam at 09–10 and 21–22 UTC.** Hourly analyses show a near-surface wind mismatch where windows meet, mostly
  over low-latitude oceans, but "it can also be seen over Europe and the USA"; it cannot be corrected, and ERA5's
  forecast winds agree better across it [7, known issue 8]. In France the 10 UTC deficit "may be larger than 0.5
  m s−1 on average" and is more visible in convective weather [12, §4.1, p. 69]. **Inference:** in the US the
  21–22 UTC seam falls at 15–17 local time in the Central zone, in the afternoon launch window; its size there is
  unmeasured.
- **Interpolating between hours:** no source measures it. The mean SGP cycle moves at most about 0.4 m/s an hour
  [3, Fig. 9b] (**Inference**: small for the mean, unbounded on a day with a jet onset, outflow or front).

## 6. What it means for drift (Inference)

- **A per-component prior.** The bracket is 1.4–1.9 m/s (at-sonde speed RMSE, a floor), about 1.8–2.4 m/s
  (background minus instrument error) and above 3.2 m/s for the operational forecast at SGP on the worst nights.
  The 1.8–2 m/s prior in wind-from-descent's *What this suggests*, a speed error, becomes about 2 m/s on each
  axis. At night or under a jet it errs narrow: σ 3 instead of 2 widens a landing ellipse 1.5 times.
- **Drift.** With the error constant down the column, a main from 1,000 m at 5 m/s (200 s) drifts with a 1σ of
  2 × 200 = 400 m on each axis: the earlier 100–400 m (speed only) becomes a disk, smaller if errors decorrelate.
- **Bias first.** Over the Plains and the West ERA5's 100 m wind is 3–16% slow, 12% on average (§3); uncorrected,
  predicted drift in the lowest few hundred meters is short by about that fraction there, not in the Northeast.

## What this suggests

- **M7.3's wind prior.** Treat ERA5's wind error as an isotropic per-component σ, about 2 m/s by day (bracket
  1.4–2.4), larger at night, rather than a speed error along the ERA5 wind. Expected: Monte Carlo landing
  ellipses widen across the wind and become nearly round disks in light wind (2–3 m/s: 26–56° mean absolute).
- **A regional speed-bias correction** for ERA5's boundary-layer wind (3–16% on the Plains and in the West, near
  zero in the Northeast [3]), stated as a choice with its source, until the descent-fitted wind replaces it.
- **Measure it with CUON.** The Copernicus upper-air network (CC BY) stores ERA5's first-guess and analysis
  departures for sondes in ERA5's input archive, and an offline first-guess departure interpolated from ERA5's
  fields [15]. US sondes by height and hour (00/12 against 06/18) would give a measured σ (assimilation flags aside).
- **Measure 10 m direction by speed class** from ISD/ASOS hourly winds (independent of ERA5 [7]), binned by
  ERA5's speed (§4), with calms handled explicitly.
- **Prefer ERA5's forecast winds** at hours next to the 09–10 and 21–22 UTC seam [7, known issue 8; 12].

## Open questions

- ERA5's hourly vector and direction error by height at US instruments it did not assimilate: the SGP lidars,
  NOAA or ARM 915 MHz profilers, NREL M2, BAO, WFIP2 or campaign sondes (PECAN, VORTEX-SE). None found.
- ERA5's error at 06/18 UTC and at local afternoon hours against 00/12 UTC, and the US size of the 21–22 UTC seam;
  its Great Plains low-level jet (core speed, height, veer) hour by hour against profilers.
- Not reached: Hersbach et al. 2020 (the ERA5 paper; the publisher refused the download and the browser was not
  connected), Ramon et al. 2019 (77 towers), Jiménez and Dudhia 2013 (direction error by speed), Zhang et al.
  2019 (AMDAR against sondes). The assimilation list here comes from ECMWF's ERA5 documentation [7].

## Sources (all read 2026-10-28)

1. P. Pentikäinen, E. J. O'Connor, P. Ortiz-Amezcua, "Evaluating wind profiles in a numerical weather prediction
   model with Doppler lidar," Geosci. Model Dev. 16:2077–2094, 2023, doi:10.5194/gmd-16-2077-2023.
2. V. Pronk et al., "Can reanalysis products outperform mesoscale numerical weather prediction models in modeling
   the wind resource in simple terrain?," Wind Energ. Sci. 7:487–504, 2022, doi:10.5194/wes-7-487-2022.
3. J. M. Wilczak et al., "Evaluation and bias correction of the ERA5 reanalysis over the United States for wind
   and solar energy applications," Energies 17:1667, 2024, doi:10.3390/en17071667.
4. M. Taszarek et al., "Comparison of convective parameters derived from ERA5 and MERRA-2 with rawinsonde data
   over Europe and North America," J. Climate 34:3211–3237, 2021, doi:10.1175/JCLI-D-20-0484.1.
5. B. J. Barbré, M. Fielder, "Performance assessment of a Western Range 915-MHz Doppler radar wind profiler,"
   NASA MSFC, NTRS 20210017065, 2021, <https://ntrs.nasa.gov/citations/20210017065>.
6. R. K. Newsom et al., "Validating precision estimates in horizontal wind measurements from a Doppler lidar,"
   Atmos. Meas. Tech. 10:1229–1240, 2017, doi:10.5194/amt-10-1229-2017.
7. ECMWF, "ERA5: data documentation," Copernicus Knowledge Base: Table 15, "Accuracy and uncertainty", known
   issue 8, <https://confluence.ecmwf.int/display/CKB/ERA5%3A+data+documentation>.
8. ECMWF, IFS Documentation Cy41r2, Part I: Observations, 2016, Table 2.6, p. 42.
9. B. Ingleby, "An assessment of different radiosonde types 2015/2016," ECMWF Tech. Memo. 807, 2017,
   doi:10.21957/0nje0wpsa, Fig. 3.5, p. 16.
10. A. C. M. Savazzi et al., "The representation of the trade winds in ECMWF forecasts and reanalyses during
    EUREC4A," Atmos. Chem. Phys. 22:13049–13066, 2022, doi:10.5194/acp-22-13049-2022.
11. I. Price et al., "GenCast: diffusion-based ensemble forecasting for medium-range weather," arXiv:2312.15796.
12. B. Jourdier, "Evaluation of ERA5, MERRA-2, COSMO-REA6, NEWA and AROME to simulate wind power production
    over France," Adv. Sci. Res. 17:63–77, 2020, doi:10.5194/asr-17-63-2020.
13. W. Jin et al., "WeatherReal: a benchmark based on in-situ observations for evaluating weather models,"
    arXiv:2409.09371, 2024.
14. R. G. Fovell, A. Gallagher, "Boundary layer and surface verification of the High-Resolution Rapid Refresh,
    version 3," Wea. Forecasting 35:2255–2278, 2020, doi:10.1175/WAF-D-20-0101.1.
15. Copernicus Climate Change Service, In situ Comprehensive Upper-Air Observation Network (CUON), Climate Data
    Store, doi:10.24381/ba504f7a.
