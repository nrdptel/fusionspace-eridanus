# Real flights of the private collection: logged apogees (M2.3c1, rechecked in M2.3c5, ADR-184, ADR-238)

Written by `cargo xtask fixture-flights`; do not edit. The flights are the tier-A flights of a private collection of designs paired with logs of the same flight (ADR-151). Its license allows only aggregate statistics, so no flight is named or numbered here. Each flight's day, logged apogee, early deployment and reason to be left out were rechecked against the collection's catalog of its sources (ADR-238), which also puts some flights below tier A: one fact of the flight inferred, or the design not shown to be the rocket flown.

## Flights

| | flights |
|---|---:|
| tier A | 89 |
| from a `.ork` | 83 |
| flown by hpr | 61 |
| flown by OpenRocket 24.12 | 79 |
| flown by both | 61 |
| left out of the errors (below) | 8 |
| compared with the log | 53 |
| of those, with an early deployment | 19 |
| of those, deployment not known | 0 |
| of those, below tier A in the catalog | 9 |
| of the 83, launch hour not known (local noon flown) | 26 |
| of the 83, rail not known (6 ft flown) | 2 |

## Not flown

| by | cause | flights | issue |
|---|---|---:|---|
| OpenRocket | OpenRocket loads no motor into that configuration | 1 | #362 |
| hpr | a fin tab ending past the root chord | 6 | #357 |
| hpr | a freeform fin outline read as crossing itself | 3 | #358 |
| hpr | a motor whose digest OpenRocket 24.12's database lacks | 3 | #362 |
| hpr | a part of negative length (rail buttons spaced forward) | 1 | #359 |
| hpr | an airframe not read as written: a part inside an off-axis inner tube | 2 | #181 |
| hpr | an airframe not read as written: an unread surface finish | 3 | #360 |
| hpr | an airframe not read as written: mass and CG overrides that differ on the parts inside | 1 | #361 |
| the collection | the collection has no ERA5 file of the flight's day | 1 | #364 |
| the collection | the design holds no configuration with the flown motor | 2 | #363 |

## Left out of the errors

| reason | flights |
|---|---:|
| a design that is not the flown rocket | 1 |
| active control in the climb | 6 |
| another reason | 1 |

## The logs

53 barometric, 0 fused (barometer and accelerometer, read as barometric) and 0 satellite (GPS) apogees. Each barometric or fused reading is turned into the height climbed in the day's ERA5 air: the height above the pad that reads it, on an altimeter that converts pressure to the 1976 standard atmosphere's altitude. That height over the reading: mean 1.0360, least 0.9601, most 1.0777.

## Apogee errors

Each simulator's apogee less the logged one, over the logged one, per cent. hpr flies the whole ERA5 profile; OpenRocket the standard atmosphere through the pad's temperature and pressure, in the same wind every 50 m; "hpr in OpenRocket's air" is hpr there too; "hpr against OpenRocket" is hpr's apogee less OpenRocket's, over OpenRocket's. Nothing deploys in either. The mean absolute error averages the errors without their signs; "within 5%" and "within 10%" count the errors strictly smaller either way. The histogram's bins are bounded at -30, -25, -20, -15, -10, -5, +0, +5, +10, +15, +20, +25, +30%, each closed below.

### every flight compared (53 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +8.73% | +9.47% | 12.95% | 16.25% | 9 | 26 | 0 0 0 1 3 6 3 6 11 10 2 5 4 2 |
| OpenRocket | +7.79% | +6.82% | 12.02% | 14.95% | 9 | 25 | 0 0 0 1 5 5 2 7 11 8 5 6 1 2 |
| hpr in OpenRocket's air | +8.89% | +9.67% | 13.06% | 16.35% | 9 | 25 | 0 0 0 1 3 6 3 6 10 9 4 6 3 2 |
| hpr against OpenRocket | +0.81% | +0.41% | 1.62% | 2.34% | 52 | 53 | 0 0 0 0 0 0 16 36 1 0 0 0 0 0 |

### without the early deployments (34 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +3.95% | +5.44% | 10.26% | 11.83% | 5 | 20 | 0 0 0 1 3 6 2 3 9 7 0 1 2 0 |
| OpenRocket | +3.19% | +5.73% | 9.49% | 10.77% | 5 | 19 | 0 0 0 1 5 4 2 3 10 6 1 2 0 0 |
| hpr in OpenRocket's air | +4.11% | +5.87% | 10.36% | 11.92% | 5 | 19 | 0 0 0 1 3 6 2 3 8 8 0 2 1 0 |
| hpr against OpenRocket | +0.69% | +0.30% | 1.63% | 2.42% | 33 | 34 | 0 0 0 0 0 0 12 21 1 0 0 0 0 0 |

### only the flights the catalog keeps in tier A (44 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +7.84% | +7.60% | 12.13% | 14.71% | 8 | 23 | 0 0 0 0 3 6 3 5 9 7 2 4 4 1 |
| OpenRocket | +6.99% | +6.34% | 11.28% | 13.53% | 8 | 22 | 0 0 0 0 5 5 2 6 9 5 5 5 1 1 |
| hpr in OpenRocket's air | +7.97% | +7.32% | 12.24% | 14.81% | 8 | 23 | 0 0 0 0 3 6 3 5 9 5 4 5 3 1 |
| hpr against OpenRocket | +0.75% | +0.27% | 1.65% | 2.45% | 43 | 44 | 0 0 0 0 0 0 15 28 1 0 0 0 0 0 |

## Sources

Flight data published by university rocketry teams, rocketry courses and hobbyists on GitHub, team websites and The Rocketry Forum.

Generated using Copernicus Climate Change Service information 2026. Contains modified Copernicus Climate Change Service information 2026 (the Earth Data Hub subsets). Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2020), *The ERA5 global reanalysis*, Q. J. R. Meteorol. Soc. 146, 1999–2049, <https://doi.org/10.1002/qj.3803>; ERA5 hourly data on single levels (<https://doi.org/10.24381/cds.adbb2d47>) and on pressure levels (<https://doi.org/10.24381/cds.bd0915c6>), Copernicus Climate Data Store.
