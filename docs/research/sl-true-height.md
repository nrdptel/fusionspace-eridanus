# Student Launch apogee errors against true height in the day's air

**In short: converting the 55 Student Launch readings to the height climbed in each day's air raises the
per-flight median error from +3.40% to +3.76% and the team-season median from +5.05% to +6.40%.** The gap to
HPR Sim's +9.47% on the private collection closes by 0.36 point per flight and 1.35 points by team-season; against
OpenRocket's +6.82% the team-season median now sits 0.42 point below. Most of these flights flew in cold air, where a
standard-atmosphere altimeter over-reads the climb: the height over the reading averages 0.990 (0.923 to 1.039) on
the 51 dated flights, and 48 of them flew between November and March. The collection's averages 1.036 [S8].
**How sure:** a factor moves a median 0.4% between ERA5's profile and its surface temperature with a standard
lapse; the medians move by up to about 1 point with the launch hour, the sim's own air and the profile source
(§4). One flight couldn't be converted; three use the report's stated temperature. This feeds the
public Student Launch bias figure beside the collection's ([M7.5](../ROADMAP.md), the bias every simulator shares;
[ADR-209](../decisions/0209-the-2026-10-08-best-on-the-market-measured.md) §6's premise).

## 1. Method

- **Rows.** The 55 flights of [Student Launch apogees](student-launch-apogees.md) §1, error = simulated ÷
  measured − 1, with the same A/B categories, tools and team-seasons. Rerun with every factor set to 1, the script
  below reproduces that note's §2 table to the digit (every row it prints; it
  omits the RocketPy and own-code row).
- **True height, as the collection defines it.** A barometric altimeter reads the 1976 standard atmosphere's
  pressure altitude of the pressure it sees, less the pad's; the height climbed is the height in the day's air
  that reads the same ([ADR-184](../decisions/0184-logged-apogees-of-the-private-collection.md) §5,
  `Barometer` in `crates/hpr-validate/src/real_flight.rs`). Every hobby family whose documents name a model reads
  this way ([altimeter formulas](altimeter-height-formulas.md) §1–3). The factor is $f$ = true height ÷ reading,
  and the error against true height is $(1 + e)/f - 1$.
- **A: ERA5 pressure levels (the main result).** For each dated flight: temperature and geopotential on the
  650–1,000 hPa levels at the four 0.25° grid points around the site, for the two hours around launch [S4].
  As in `hpr_io::era5` and `hpr_atmos::profile`: bilinear in latitude and longitude, linear in time, temperature
  linear in geopotential height between levels with ln p through both levels, the offset standard atmosphere
  below the lowest, geopotential to geometric height by WMO-No. 8 eqs. 12.15–12.16 (a normal-gravity formula
  other than `hpr_atmos::profile`'s; with the project's the factors agree to four decimals). The pad is the site's
  Copernicus DEM elevation [S6]; the height is bisected to the reading. A check on a column 20 K off standard
  returns the closed form (T₀ + ΔT)/T₀ exactly in geopotential height.
- **B: ERA5 2 m temperature with the standard lapse** (Open-Meteo's, downscaled to the site [S5]), and
  **C: the report's stated launch temperature** with the standard lapse. For a column with a 6.5 K/km lapse,
  $f = T_s/T_{\mathrm{std}}$, with $T_s$ the pad temperature and $T_{\mathrm{std}}$ the standard temperature at
  the pad's pressure altitude (**Inference**, from the standard's troposphere formula; it is the reciprocal of altimeter
  formulas §3's T₀/(T₀ + ΔT), at any pad).
- **Launch hour.** The report's time where it gives one (14 flights; two from the team's own launch log [S3]),
  else local noon, as the collection does (ADR-184 §4).
- **Sites.** Bayboro NC at the team's stated coordinates [N18c p58], Three Oaks MI [D18f p102], Milford IN
  [T25C p53], Dalzell SC and Bragg Farms AL from the team's launch log [S3]; Palm Bay FL and Cedarville OH from
  the reports' street addresses; Samson AL, Plant City FL, Pence IN, South Charleston OH, Elsberry MO and
  Culpeper VA at town centroids [S7] (fields within a few kilometers, inside one ERA5 grid cell; **Inference**).
- **The simulation's own air (S, D, U).** The collection flies HPR Sim in the ERA5 profile and OpenRocket "the
  standard atmosphere through the pad's temperature and pressure" [S8]. A team that entered the day's
  temperature (D, 12 rows) matches the latter; one that says it used standard conditions (S, 6) or doesn't say
  (U, 37) flew thinner air on a cold day, which raises its apogee. §4 moves those sims into the day's air by
  $e_D(\rho_{\mathrm{day}}/\rho_{\mathrm{std}} - 1)$, with $\rho$ the layer-mean density over the climb and $e_D$,
  apogee's elasticity to drag, −0.31 to −0.57 ([apogee bias](apogee-bias.md), its elasticity table).
- **Reproducible.** The table was computed from these inputs by `compute.py` in the project's gitignored research
  cache (`refs/research/sl-true-height/scripts/`, with `flights.csv`, `sites.csv` and the fetch scripts).

## 2. Each flight

The per-flight table (date, site, ERA5 and stated temperatures, the three factors, the sim's air and both
errors) is in [the flights part](sl-true-height-flights.md), with the report pages read for each row. In brief:

- **Cold flights dominate.** Factor A runs from 0.923 (Toledo's VDF, 19 °F at Milford IN, 2024-02-17) to 1.039
  (USF's subscale, Plant City, 2016-12-17); 31 of 51 dated flights are below 1. Every dated Notre Dame, Purdue
  and WashU flight is (0.942 to 0.991); every converted USF flight is above (1.002 to 1.039).
- **The two launch weeks in the set** read 1.002 (Huntsville, 2017-04-08) and 1.023 (2019-04-06). The hot
  launch week the earlier note names (91.8 °F) is an Unclear row, in no statistic.

## 3. The statistics, raw and against true height

Method A, with the three undated flights by C and F16s held at its raw error (its factor at 0.98 or 1.06
leaves the per-flight and team-season medians unchanged; at 0.98 OpenRocket's moves to +2.80%). Intervals
use the earlier note's order-statistic rule; ranks are unchanged.

| set | n | raw median | true median | true 95% interval | read high, raw → true |
|---|--:|--:|--:|--:|--:|
| one row per flight | 55 | +3.40% | +3.76% | +1.79% to +8.76% | 39 → 41 |
| every tool's row | 63 | +3.00% | +3.96% | +2.10% to +8.02% | 44 → 48 |
| OpenRocket | 41 | +1.70% | +2.10% | +0.69% to +6.38% | 26 → 29 |
| RockSim | 16 | +8.90% | +11.39% | +5.46% to +22.46% | 13 → 15 |
| strict (A), every tool | 23 | +3.00% | +3.34% | −2.01% to +10.92% | 15 → 16 |
| strict, OpenRocket | 15 | +1.10% | +2.09% | −3.11% to +10.92% | 9 → 10 |
| per flight, without NC State | 33 | +2.60% | +3.69% | +1.10% to +7.94% | 22 → 24 |
| **team-season medians** | 30 | +5.05% | **+6.40%** | +1.06% to +10.92% | 21 → 22 |
| team medians | 10 | +5.68% | +6.40% | −0.92% to +11.40% | 8 → 8 |
| subscale | 26 | +3.35% | +3.86% | +1.10% to +10.50% | 21 → 20 |
| full scale | 29 | +3.40% | +3.69% | +0.07% to +11.14% | 18 → 21 |
| 2014–20 | 25 | +2.30% | +2.09% | +0.69% to +10.92% | 16 → 18 |
| 2020–26 | 30 | +3.95% | +5.70% | +2.80% to +8.76% | 23 → 23 |

- **Means:** per flight +7.69% → +8.76%; OpenRocket's rows +4.03% → +5.19%; RockSim's +15.03% → +17.69%.
- **Each flight's error moved** by a median +1.1 points (−4.3 to +10.6); 33 of 54 rose. The largest rises are
  cold days: Toledo's 19 °F VDF (+26.2% → +36.8%), NC State's 2021 VDF (+43.3% → +49.8%).
- **Team medians:** NC State +5.75% → +7.11%, Notre Dame +0.10% → +3.37%, Toledo +9.3% → +13.7%, Purdue +8.9% →
  +11.4%, WashU −7.25% → −5.18%, FSU Panama City −2.55% → −0.92%; the warm-air teams fell: USF +2.8% → +1.6%,
  Georgia Tech +14.8% → +10.92%, FAMU-FSU +9.0% → +6.84%; Post 1010 +5.6% → +5.95%.
- **Against the collection:** per flight, the gap to +9.47% goes from 6.07 to 5.71 points and the interval still
  excludes it, by 0.71 point; by team-season, from 4.42 to 3.07 points, and the interval now holds +9.47% and
  +6.82%. The means, +8.76% against +8.73% and +7.79%, now match HPR Sim's.

## 4. How far the conversion can move

| choice | per flight | team-season |
|---|--:|--:|
| A, ERA5 levels (main) | +3.76% | +6.40% |
| B, ERA5 2 m temperature, standard lapse | +4.30% | +6.15% |
| C, stated temperature where given (39 flights), else A | +3.92% | +5.87% |
| untimed flights (37) at 10:00 or 15:00 local, not noon | +4.71%; +3.11% | +6.69%; +5.80% |
| S sims moved into the day's air, $e_D$ −0.31 to −0.57 | +3.76% | +6.23% to +5.95% |
| S and U sims moved into the day's air, same $e_D$ | +3.76% | +6.07% to +5.66% |
| PerfectFlite rows read with the pad as sea level | +3.76% | +6.40% |

- **Profile source: small per flight.** B's factor differs from A's by a median 0.40% (largest 2.4%), with no
  net sign (mean +0.02%). C differs by a median 0.62% on 39 flights, mean +0.17%; its largest, +4.3% on FSU
  Panama City's second subscale, uses a flight computer's average reading, as does that team's first subscale;
  Toledo 2026's "measured temperature" reads like one too (57.3 °F, 76.1 °F and 62 °F against ERA5's 52 °F,
  58 °F and 46 °F). **Inference:** board temperatures run warm; ERA5 is the better input.
- **Launch hour: the largest term per flight, an upper bound.** The row shifts each untimed factor by the change
  in 2 m temperature from noon; the 1–2 km layer warms less than the surface through the day, so the true spread is
  narrower (**Inference**). Sign: an earlier launch raises the errors.
- **The sim's air: lowers the team-season median by up to 0.74 point.** Layer-mean density ran 0.973 to 1.087
  times the standard's. Moving all 43 S and U sims (41 dated) assumes none of the U teams entered the day's
  temperature, so it is the far end; the six S rows alone move it 0.17 to 0.45 point.
- **Altimeter form: at most 0.51% per flight, either way.** PerfectFlite doesn't say whether it differences
  two standard altitudes or treats the pad as sea level ([altimeter formulas](altimeter-height-formulas.md) §1);
  the latter reads off by $T_0/T_{\mathrm{std}}$ at the pad: −0.51% to +0.25% on the 11 PerfectFlite rows (a
  high-pressure day puts the pad's pressure altitude below sea level). Families not named are assumed standard,
  pad-zeroed.
- **ERA5's own error:** about 0.35% of height per kelvin of layer-mean temperature; ERA5's low-level layer mean
  is within about ±0.5 K away from the surface (that note's **Inference**,
  [ERA5 at launch sites](era5-surface-temperature.md)).
- **Net:** per flight +3.1% to +4.7%, team-season +5.7% to +6.7%. Every choice closes the team-season gap to
  +9.47% (by 0.6 to 1.6 points); per flight the change runs from −0.3 to +1.3 points.

## 5. Flights not converted, or converted another way

- **F16s** (USF 2015–16 subscale): no date or temperature in the CDR; held at its raw +1.1%.
- **By stated temperature (C), no date:** WashU's subscale (56 °F), FAMU-FSU's first subscale (64–68 °F, taken
  as 66 °F) and Toledo 2025's (50 °F, its 30.13 inHg read as an altimeter setting).
- **Assumed:** the home field for NC State's 2015–16, second 2016–17 and 2023–24 subscales (site not stated);
  Plant City for USF 2018's third flight (date about 2018-03-17 from a team post the archive no longer shows);
  the Cedarville address for Toledo's 2024 re-flight ("WSR in Springfield"); the first of two flight days for
  Notre Dame 2021–22's subscale (37 °F; the other, 36 °F). Purdue's subscale time is taken as stated, central.
  FSU Panama City's first subscale is dated 12/07/2025 as its CDR's text says (the photo caption says
  December 14). N25S uses the earlier of the launch log's two subscale times that day (13:54; the other, 16:55 EST).

## What this suggests

- **Quote the true-height figures beside the collection's:** Student Launch first predictions run +3.76% per
  flight (+1.79% to +8.76%) and +6.40% by team-season (+1.06% to +10.92%), against +9.47% and +6.82%. The air
  explains about 1.35 of the 4.42 points between the team-season median and HPR Sim's figure (**Inference**).
- **The rest is still the population:** weighed masses, the day's wind, ballast toward a declared target and
  predictions printed after the flight ([Student Launch apogees](student-launch-apogees.md) §3) all pull toward
  zero. [M7.5](../ROADMAP.md) could test a correction fitted on the collection against these 54 converted rows.
- **ERA5 point subsets need no account.** NSF NCAR's GDEX serves ERA5 pressure levels by OPeNDAP under CC BY 4.0,
  about 2 KB per flight and variable [S4]. A suggestion for the `hpr-net` ERA5 path: fetching a site's column
  instead of a whole file would let public-flight conversions run without a Copernicus login.

## Open questions

- Launch hours for the 37 untimed flights, and whether the 37 U rows' teams entered the day's temperature.
- F16s's date; the 2018 USF post's original capture.
- The project's air is dry; ERA5's level heights carry moisture, its extrapolation below 1,000 hPa doesn't.

## Sources

Read 2026-10-18. Team reports state no license: cited only, read from the gitignored cache. ERA5 and Open-Meteo
data are CC BY 4.0 and may be cached under `refs/` with attribution; this note contains modified Copernicus
Climate Change Service information. Cached: `refs/research/sl-true-height/downloads/` (era5, openmeteo, elev, geo).

1. **S1** Team reports, keyed and linked as in [Student Launch apogees](student-launch-apogees.md), its Sources.
2. **S2** [Student Launch apogees](student-launch-apogees.md) §1–2: the 55 rows and the raw statistics.
3. **S3** NC State AirbrakesV2, `launch_data/metadata.json` (MIT, commit afdcecb44e97), launch times and
   coordinates. <https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2>
4. **S4** ERA5 pressure levels, NSF NCAR GDEX dataset d633000, DOI 10.5065/BH6N-5N20, CC BY 4.0, via
   <https://tds.gdex.ucar.edu/thredds/dodsC/files/g/d633000/e5.oper.an.pl/>. Hersbach et al., Q. J. R. Meteorol.
   Soc. 146 (2020) 1999–2049.
5. **S5** Open-Meteo Historical Weather API, `models=era5`, CC BY 4.0. <https://archive-api.open-meteo.com/v1/archive>
6. **S6** Open-Meteo Elevation API (Copernicus DEM, 90 m). <https://api.open-meteo.com/v1/elevation>
7. **S7** Nominatim, OpenStreetMap data (ODbL). <https://nominatim.openstreetmap.org/>
8. **S8** [Fixture flights](../../validation/reports/fixture-flights.md): the collection's medians, factor and air.
9. **S9** *U.S. Standard Atmosphere, 1976* (troposphere constants), and the project's `Barometer`,
   `hpr_atmos::profile` and `hpr_io::era5` doc comments for the interpolation rules.
