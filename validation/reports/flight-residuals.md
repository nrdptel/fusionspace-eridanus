# hpr's residuals against the logs of real flights

Written by `cargo xtask real-flights` ([M7.3a][m7-3a], decision [ADR-314][adr-314]). [How to read them][site].

- **What is flown:** the 7 flights the [real-flights report][real] compared first, each flown to the ground in its ERA5 air and wind, on hpr's own drag, under the parachutes its example's notebook gives it.
- **A residual** is the logged value less the simulated one. Positive: the log read higher, or faster upward, than the simulation. A descent's rate is a size, so there a positive residual in the readings means the log fell faster.
- **Heights** are the log's own samples, above its pad, unsmoothed. The simulation's are read as the log's altimeter would read them. **Speeds** are the smoother's, taken from the log with any GNSS set aside, against the rate at which the same altimeter's reading would change.
- **The clocks** are lined up by what one reader calls liftoff on each. Their bound is rounded up.
- **Tests:** a noise-free 20 Hz log of a flight against itself, its ignition on a sample, leaves an RMS under 1 m (3.3 ft). With ignition midway between a 0.2 s log's samples, on the test's flight of 180 m/s (591 ft/s) at burnout, the clocks alone leave an ascent RMS of about 10 m (33 ft). Each flight's simulated apogee, read by the log's reader, stays within 0.5 m (1.6 ft) of the flight's own event.
- A residual is one flight against one simulation. It doesn't say which is wrong.

[m7-3a]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3a
[adr-314]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0314-m7-3-in-five-residuals-first.md
[site]: https://hpr.fusionspace.co/comparing-a-flight.html
[real]: real-flights.md

## Bella Lui, EPFL Rocket Team, 2020 (K828FJ)

Altimeter: barometric, assumed, read as a barometer. 768 log samples; clocks lined up within 2.25 s; the log reads a landing.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 10.39 | 176 | -0.6 | 3.5 | +9.3 | 1.86 |
| descent | 10.39 | 34.32 | 393 | -10.4 | 11.5 | -20.4 | 28.62 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 10.39 | 176 | -0.5 | 1.7 | +7.2 | 1.50 |
| descent | 10.39 | 34.32 | 393 | -0.5 | 2.0 | -5.0 | 26.40 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 458.96 | 431.34 to 486.27 | measured | 464.01 | -5.06 |
| time to apogee (s) | 10.39 | none stated | measured | 10.41 | -0.02 |
| top vertical speed (m/s) | 91.54 | 85.39 to 96.58 | derived | 85.72 | +5.82 |
| top acceleration (m/s²) | withheld |  |  | withheld |  |
| burnout (s) | at least 1.86 | none stated | derived | at least 2.04 |  |
| flight time (s) | 34.32 | 34.66 to 35.28 | measured | 35.22 | -0.90 |
| descent under the drogue (m/s) | withheld |  |  | withheld |  |
| main's opening (m) | withheld |  |  | withheld |  |
| descent under the main (m/s) | 19.16 | 18.19 to 20.11 | derived | 18.65 | +0.51 |

## NDRT 2020, Notre Dame Rocketry Team (L1395)

Altimeter: barometric, read as a barometer. 1819 log samples; clocks lined up within 1.66 s; the log reads a landing.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 16.75 | 335 | -83.7 | 92.6 | -134.9 | 16.70 |
| descent | 16.75 | 84.30 | 1352 | -69.4 | 83.4 | -155.9 | 19.55 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 16.75 | 335 | -8.1 | 11.2 | -42.5 | 4.25 |
| descent | 16.75 | 84.30 | 1352 | +1.3 | 5.1 | -16.3 | 67.20 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 1319.68 | 1256.25 to 1382.41 | measured | 1457.46 | -137.78 |
| time to apogee (s) | 16.75 | none stated | measured | 17.47 | -0.72 |
| top vertical speed (m/s) | 168.48 | 152.09 to 181.90 | derived | 180.06 | -11.58 |
| top acceleration (m/s²) | 105.91 | 70.18 to 137.05 | measured | 77.41 | +28.50 |
| burnout (s) | at least 4.60 | none stated | derived | 3.20 |  |
| flight time (s) | 84.30 | 85.03 to 86.37 | measured | 94.35 | -10.05 |
| descent under the drogue (m/s) | 22.41 | 21.34 to 23.50 | derived | 27.54 | -5.13 |
| main's opening (m) | 39.87 | none stated | derived | 137.62 | -97.75 |
| descent under the main (m/s) | 3.62 | 3.08 to 4.14 | derived | 4.69 | -1.07 |

## Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520)

Altimeter: barometric, read as a barometer. 5712 log samples; clocks lined up within 1.37 s; the log reads a landing; 273 samples after the simulated flight had ended.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| boost | 0.00 | 4.67 | 467 | -126.8 | 165.1 | -309.2 | 4.54 |
| coast | 4.67 | 28.69 | 2402 | +108.6 | 214.2 | +366.1 | 28.64 |
| descent | 28.69 | 223.01 | 2319 | +301.5 | 322.2 | +418.6 | 37.87 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| boost | 0.00 | 4.67 | 467 | -78.9 | 87.8 | -119.0 | 4.14 |
| coast | 4.67 | 28.69 | 2402 | +30.5 | 42.8 | +100.5 | 8.35 |
| descent | 28.69 | 223.01 | 2319 | -0.8 | 6.3 | -21.5 | 157.51 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 3892.09 | 3711.78 to 4070.42 | measured | 3536.60 | +355.49 |
| time to apogee (s) | 28.69 | none stated | measured | 27.21 | +1.48 |
| top vertical speed (m/s) | withheld |  |  | withheld |  |
| top acceleration (m/s²) | 99.40 | 90.12 to 107.48 | measured | 92.99 | +6.41 |
| burnout (s) | 4.67 | none stated | measured | 4.63 | +0.04 |
| flight time (s) | 223.01 | 223.44 to 224.24 | measured | 202.30 | +20.71 |
| descent under the drogue (m/s) | 26.50 | 25.30 to 27.73 | derived | 26.70 | -0.20 |
| main's opening (m) | 411.51 | none stated | derived | 418.85 | -7.34 |
| descent under the main (m/s) | 6.50 | 6.16 to 6.84 | derived | 7.15 | -0.64 |

## Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor)

Altimeter: barometric, read as a barometer. 611 log samples; clocks lined up within 2.00 s; the log ends before a landing, so the residuals stop at its end.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| flight | 0.00 | 29.95 | 600 | +254.1 | 467.1 | +7756.8 | 29.90 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| flight | 0.00 | 29.95 | 600 | -0.1 | 25.2 | -122.9 | 3.10 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | withheld |  |  | 2976.66 |  |
| time to apogee (s) | withheld |  |  | 26.73 |  |
| top vertical speed (m/s) | withheld |  |  | withheld |  |
| top acceleration (m/s²) | withheld |  |  | withheld |  |
| burnout (s) | withheld |  |  | withheld |  |
| flight time (s) | withheld |  |  | 157.80 |  |
| descent under the drogue (m/s) | withheld |  |  | withheld |  |
| main's opening (m) | withheld |  |  | withheld |  |
| descent under the main (m/s) | withheld |  |  | 22.69 |  |

## Cavour, Politecnico di Torino, EuRoC 2023 (L995)

Altimeter: barometric, assumed, read as a barometer. 404 log samples; clocks lined up within 3.21 s; the example gives no parachutes, so only the ascent is compared.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 23.50 | 128 | -96.2 | 117.7 | -164.4 | 23.40 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 23.50 | 128 | -8.4 | 22.9 | +76.3 | 2.60 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 2785.00 | 2641.57 to 2926.85 | measured | 2945.42 | -160.42 |
| time to apogee (s) | 23.50 | none stated | measured | 23.30 | +0.20 |
| top vertical speed (m/s) | withheld |  |  | withheld |  |
| top acceleration (m/s²) | withheld |  |  | withheld |  |
| burnout (s) | withheld |  |  | withheld |  |

## Genesis, EuRoC 2023 (L995)

Altimeter: barometric, assumed, read as a barometer. 17933 log samples; clocks lined up within 1.40 s; the log reads a landing; 157 samples after the simulated flight had ended.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 23.69 | 2369 | +54.8 | 97.1 | +171.8 | 23.68 |
| descent | 23.69 | 175.53 | 15028 | +66.1 | 89.7 | +209.4 | 28.94 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 23.69 | 2369 | +78.1 | 191.9 | +705.2 | 8.24 |
| descent | 23.69 | 175.53 | 15028 | -1.0 | 2.3 | +6.1 | 23.69 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 2915.34 | 2779.95 to 3049.24 | measured | 2745.16 | +170.18 |
| time to apogee (s) | 23.69 | none stated | measured | 23.04 | +0.65 |
| top vertical speed (m/s) | withheld |  |  | withheld |  |
| top acceleration (m/s²) | withheld |  |  | withheld |  |
| burnout (s) | withheld |  |  | withheld |  |
| flight time (s) | 175.53 | 175.76 to 176.18 | measured | 173.72 | +1.81 |
| descent under the drogue (m/s) | 25.68 | 24.49 to 26.88 | derived | 23.73 | +1.95 |
| main's opening (m) | 806.18 | none stated | derived | 797.30 | +8.88 |
| descent under the main (m/s) | 11.53 | 10.98 to 12.06 | derived | 11.60 | -0.07 |

## Lince, EuRoC 2023 (M1101)

Altimeter: barometric, assumed, read as a barometer. 44454 log samples; clocks lined up within 1.32 s; the log reads a landing.

Height:

| phase | from (s) | to (s) | samples | mean (m) | RMS (m) | largest (m) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 25.91 | 2591 | -3.1 | 67.4 | -134.2 | 25.89 |
| descent | 25.91 | 430.91 | 40501 | -525.2 | 550.5 | -902.9 | 27.63 |

Speed:

| phase | from (s) | to (s) | samples | mean (m/s) | RMS (m/s) | largest (m/s) | at (s) |
|---|---:|---:|---:|---:|---:|---:|---:|
| ascent | 0.00 | 25.91 | 2591 | +3.4 | 95.4 | +376.1 | 15.11 |
| descent | 25.91 | 430.91 | 40501 | +16.7 | 354.3 | +10210.8 | 29.09 |

Readings, the simulation read as the log is:

| reading | logged | its 90% interval | provenance | simulated | logged less simulated |
|---|---:|---:|---|---:|---:|
| apogee (m) | 3586.66 | 3420.58 to 3750.92 | measured | 3728.01 | -141.34 |
| time to apogee (s) | 25.91 | none stated | measured | 27.13 | -1.22 |
| top vertical speed (m/s) | withheld |  |  | withheld |  |
| top acceleration (m/s²) | withheld |  |  | withheld |  |
| burnout (s) | withheld |  |  | withheld |  |
| flight time (s) | 430.91 | 431.32 to 432.09 | measured | 551.86 | -120.95 |
| descent under the drogue (m/s) | withheld |  |  | withheld |  |
| main's opening (m) | 3403.33 | none stated | derived | withheld |  |
| descent under the main (m/s) | 8.59 | 8.21 to 8.96 | derived | 7.10 | +1.49 |

## Sources

The ERA5 weather here is read from the files RocketPy's examples read: generated using Copernicus Climate Change Service information 2026, and contains modified Copernicus Climate Change Service information 2026. Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2023), *ERA5 hourly data on pressure levels from 1940 to present*, Copernicus Climate Change Service (C3S) Climate Data Store (CDS), <https://doi.org/10.24381/cds.bd0915c6>, licensed CC BY 4.0 (<https://creativecommons.org/licenses/by/4.0/>).
