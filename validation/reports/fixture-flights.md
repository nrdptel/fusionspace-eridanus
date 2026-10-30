# Real flights of the private collection: logged apogees (M2.3c1, rechecked in M2.3c5, ADR-184, ADR-238)

Written by `cargo xtask fixture-flights`; do not edit. The flights are the tier-A flights of a private collection of designs paired with logs of the same flight (ADR-151). Its license allows only aggregate statistics, so no flight is named or numbered here. Tier A is the collection's first grading of its best-documented flights. Each flight's day, logged apogee, early deployment and reason to be left out were rechecked against the collection's catalog, a second reading of each flight's sources (ADR-238, which sets out what the recheck moved). The catalog proposes some flights below tier A: one of the design, motor, log, day or site inferred (B), or the design not shown to be the rocket flown (C).

## Flights

| | flights |
|---|---:|
| tier A | 89 |
| from a `.ork` | 83 |
| flown by hpr | 68 |
| flown by OpenRocket 24.12 | 79 |
| flown by both | 68 |
| left out of the errors (below) | 8 |
| compared with the log | 60 |
| of those, with an early deployment | 19 |
| of those, deployment not known | 0 |
| of those, proposed below tier A by the catalog | 11 |
| of the 83, launch hour not known (local noon flown) | 26 |
| of the 83, rail not known (6 ft flown) | 2 |

## Not flown

| by | cause | flights | issue |
|---|---|---:|---|
| OpenRocket | OpenRocket loads no motor into that configuration | 1 | #362 |
| hpr | a freeform fin outline read as crossing itself | 3 | #358 |
| hpr | a motor whose digest OpenRocket 24.12's database lacks | 2 | #362 |
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

60 barometric, 0 fused (barometer and accelerometer, read as barometric) and 0 satellite (GPS) apogees. Each barometric or fused reading is turned into the height climbed in the day's ERA5 air: the height above the pad that reads it, on an altimeter that converts pressure to the 1976 standard atmosphere's altitude. That height over the reading: mean 1.0366, least 0.9601, most 1.0777.

## Apogee errors

Each simulator's apogee less the logged one, over the logged one, per cent. hpr flies the whole ERA5 profile; OpenRocket the standard atmosphere through the pad's temperature and pressure, in the same wind every 50 m; "hpr in OpenRocket's air" is hpr there too; "hpr against OpenRocket" is hpr's apogee less OpenRocket's, over OpenRocket's. Nothing deploys in either. The mean absolute error averages the errors without their signs; "within 5%" and "within 10%" count the errors strictly smaller either way. The histogram's bins are bounded at -30, -25, -20, -15, -10, -5, +0, +5, +10, +15, +20, +25, +30%, each closed below.

### every flight compared (60 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +7.76% | +6.69% | 12.07% | 15.43% | 14 | 32 | 0 0 0 1 4 6 4 10 12 10 2 5 4 2 |
| OpenRocket | +6.72% | +6.08% | 11.26% | 14.25% | 13 | 31 | 0 0 0 1 6 6 3 10 12 8 5 6 1 2 |
| hpr in OpenRocket's air | +7.90% | +6.50% | 12.17% | 15.53% | 14 | 31 | 0 0 0 1 4 6 4 10 11 9 4 6 3 2 |
| hpr against OpenRocket | +0.93% | +0.62% | 1.64% | 2.34% | 59 | 60 | 0 0 0 0 0 0 16 43 1 0 0 0 0 0 |

### without the early deployments (41 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +3.34% | +4.72% | 9.43% | 11.11% | 10 | 26 | 0 0 0 1 4 6 3 7 10 7 0 1 2 0 |
| OpenRocket | +2.41% | +4.26% | 8.81% | 10.22% | 9 | 25 | 0 0 0 1 6 5 3 6 11 6 1 2 0 0 |
| hpr in OpenRocket's air | +3.49% | +4.86% | 9.51% | 11.18% | 10 | 25 | 0 0 0 1 4 6 3 7 9 8 0 2 1 0 |
| hpr against OpenRocket | +0.88% | +0.64% | 1.66% | 2.41% | 40 | 41 | 0 0 0 0 0 0 12 28 1 0 0 0 0 0 |

### without the flights the catalog proposes below tier A (49 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +7.01% | +6.28% | 11.58% | 14.15% | 11 | 27 | 0 0 0 0 4 6 4 7 10 7 2 4 4 1 |
| OpenRocket | +6.09% | +5.84% | 10.84% | 13.08% | 10 | 26 | 0 0 0 0 6 6 2 8 10 5 5 5 1 1 |
| hpr in OpenRocket's air | +7.14% | +6.49% | 11.68% | 14.24% | 11 | 27 | 0 0 0 0 4 6 4 7 10 5 4 5 3 1 |
| hpr against OpenRocket | +0.83% | +0.55% | 1.64% | 2.40% | 48 | 49 | 0 0 0 0 0 0 15 33 1 0 0 0 0 0 |

## Sources

Flight data published by university rocketry teams, rocketry courses and hobbyists on GitHub, team websites and The Rocketry Forum.

Generated using Copernicus Climate Change Service information 2026. Contains modified Copernicus Climate Change Service information 2026 (the Earth Data Hub subsets). Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2020), *The ERA5 global reanalysis*, Q. J. R. Meteorol. Soc. 146, 1999–2049, <https://doi.org/10.1002/qj.3803>; ERA5 hourly data on single levels (<https://doi.org/10.24381/cds.adbb2d47>) and on pressure levels (<https://doi.org/10.24381/cds.bd0915c6>), Copernicus Climate Data Store.
