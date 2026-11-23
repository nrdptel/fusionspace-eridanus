# Wind from the GPS descent, against ERA5

Written by `cargo xtask descent-wind` (M1.14b6); don't edit it by hand. The wind is reconstructed from each public flight's GPS track of its descent as the ground velocity plus `w/g` times its rate of change, `w` the descent rate (Hock and Franklin 1999; [the method](../../docs/physics/wind.md#wind-from-a-gps-tracked-descent)). It is set beside ERA5's wind at each fix's height above sea level, read at the launch site and hour. A fix is one position the receiver reported.

**5 public flights compared.** The differences below hold ERA5's miss at the field and whatever the canopy did, which nothing here tells apart.

**Settings.** A quadratic in time fitted over 10 s around each fix, at least 5 fixes reaching a quarter of the window on each side. A fix is read where the body falls at 2 to 35 m/s with a vertical acceleration of at most 0.5 m/s². `g` = 9.80665 m/s².

## The flights

Repeats are rows dropped as a repeat of the fix before; read fixes are those read as a steady fall. The lag correction is the mean length of what `w/g` times the ground velocity's rate of change adds to it.

| Flight | Status | Fixes | Repeats | Longest gap (s) | Read | Heights read (m above the pad) | Descent (m/s) | Lag correction (m/s) | Speed less ERA5's, mean (m/s) | Vector difference, RMS (m/s) |
|---|---|---|---|---|---|---|---|---|---|---|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) (`bella-lui`) | not run: its log holds a filtered height and speed only, no position |  |  |  |  |  |  |  |  |  |
| NDRT 2020, Notre Dame Rocketry Team (L1395) (`ndrt-2020`) | not run: its log holds acceleration and a barometric height only, no position |  |  |  |  |  |  |  |  |  |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) (`prometheus-2022`) | not compared: RocketPy's ERA5 file for it is of 24 June 2023, a year after the flight, and no file of its day is pinned | 239 | 6842 | 1.0 | 174 | 28 to 3927 | 5.7 to 32.0 | 1.47 |  |  |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) (`juno-iii`) | compared | 383 | 19 | 3.0 | 102 | 421 to 3210 | 23.7 to 27.2 | 0.61 | +1.53 | 5.71 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) (`cavour`) | not run: no fix falls steadily inside the settings: the median descent rate the fitted windows give is 75 m/s, past the 35 m/s the method is held to | 110 | 294 | 9.8 | 0 |  |  |  |  |  |
| Genesis, EuRoC 2023 (L995) (`genesis`) | compared | 140 | 106 | 79.2 | 56 | 44 to 666 | 9.7 to 12.4 | 0.82 | +1.14 | 2.55 |
| Lince, EuRoC 2023 (M1101) (`lince`) | not run: its log holds a filtered height and acceleration only, no position |  |  |  |  |  |  |  |  |  |
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) (`astra`) | not run: no fix falls steadily inside the settings: the median descent rate the fitted windows give is 98 m/s, past the 35 m/s the method is held to | 45 | 17 | 9.0 | 0 |  |  |  |  |  |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) (`andromeda`) | not run: its log holds a height and a vertical speed only, no position |  |  |  |  |  |  |  |  |  |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) (`erebus-11`) | not run: its log holds a filtered height only, no position |  |  |  |  |  |  |  |  |  |
| Macie XL, flight 1 (AeroTech I284W) (`macie-xl-1`) | not run: its logs hold heights, speeds and accelerations, no position; the positions in the OpenRocket file beside them are a simulation's |  |  |  |  |  |  |  |  |  |
| Macie XL, flight 2 (Cesaroni 601I350-16A) (`macie-xl-2`) | not run: its logs hold heights, speeds and accelerations, no position; the positions in the OpenRocket file beside them are a simulation's |  |  |  |  |  |  |  |  |  |
| Camões, EuRoC 2023 (the team's own motor, not flown by HPR Sim) (`camoes`) | compared | 422 | 78 | 9.8 | 353 | 114 to 2677 | 24.7 to 28.2 | 1.07 | +1.39 | 3.68 |
| Halcyon, EuRoC 2023 (the team's own motor, not flown by HPR Sim) (`halcyon`) | compared | 2012 | 44 | 6.5 | 1951 | 623 to 3336 | 3.9 to 5.4 | 0.38 | -1.46 | 5.07 |
| Hedy, EuRoC 2025 (the team's own motor, not flown by HPR Sim) (`hedy`) | not compared: its ERA5 file is netCDF-4, which HPR Sim reads only through a converted copy checked against the source, as EuRoC 2022's is; no such copy is made for this file yet | 154 | 234 | 68.0 | 94 | 126 to 3310 | 20.8 to 26.4 | 0.56 |  |  |
| Valkyrie, 2025 (not flown by HPR Sim) (`valkyrie`) | not run: its log holds a height only, no position |  |  |  |  |  |  |  |  |  |
| Hyperion-II, 21 August 2025 (not flown by HPR Sim) (`hyperion-ii`) | not run: no fix falls steadily inside the settings: the median descent rate the fitted windows give is 85 m/s, past the 35 m/s the method is held to | 79 | 3339 | 1.0 | 0 |  |  |  |  |  |
| Itzamna, UTRGV, March 2026 (not flown by HPR Sim) (`itzamna`) | compared | 380 | 27 | 3.0 | 96 | 24 to 2731 | 6.7 to 31.3 | 1.92 | +0.61 | 4.43 |
| Testbed 2, RIT Launch Initiative, 16 May 2026 (not flown by HPR Sim) (`rit-testbed-2`) | not compared: no ERA5 file of its day and site is committed | 220 | 29 | 2.0 | 33 | 33 to 1032 | 7.9 to 27.6 | 2.32 |  |  |

The receivers read:

- `prometheus-2022`: the TeleMetrum's GPS (Altus Metrum), 1 fix a second.
- `juno-iii`: the COTS GNSS log (`cots_GNSS.csv`), 1 fix a second in the descent; the team's own telemetry, which the README calls possibly noisy, is not read.
- `cavour`: the CATS Vega's radio telemetry: latitude and longitude sent to four decimals (about 11 m north and 9 m east), height from its barometric filter.
- `genesis`: the GNSS log (`gnss_faraday.csv`): latitude and longitude, 1 to 10 fixes a second; height from the flight log's barometric filter.
- `astra`: the flight log (`flight_data.csv`): latitude and longitude to four decimals about once a second; height from its `alt` column, read as barometric above the pad.
- `camoes`: the CATS flight computer's GNSS log (`gnss_flight.csv`): latitude and longitude; height from its barometric filter (`flight_data.csv`). Its site and hour: camoes_flight_sim.ipynb's environment: 14:00 Portugal time, UTC+1.
- `halcyon`: the CATS flight computer's GNSS log (`gnss_halcyon.csv`): latitude and longitude; height from its barometric filter (`altimeter_halcyon.csv`). Its site and hour: halcyon_flight_sim.ipynb's environment: 14:00 Portugal time, UTC+1.
- `hedy`: the CATS flight computer's GNSS log (`gnssInfo.csv`): latitude and longitude; height from its barometric filter (`filteredDataInfo.csv`). Its site and hour: hedy_flight_sim.ipynb's environment: 16:00 UTC, and the site's elevation as the notebook prints it.
- `hyperion-ii`: the TeleMega's GPS (Altus Metrum), 1 fix a second. Its site and hour: the TeleMega's first GPS fix: 18:11:46 UTC, on the pad.
- `itzamna`: the Featherweight GPS tracker's log (`UTRGV GPS _03-29-2026_08_47_14.csv`): latitude, longitude and the receiver's altitude. Its site and hour: the GPS log's summary: launched 13:47:36 UTC, the pad at 1,362 ft. It dates the launch 30 March; the Blue Raven's summary gives 29 March, 08:47:13 local, and so does ERA5's pressure at the pad's height, 97,139 Pa on the 29th and 96,713 Pa on the 30th against the 97,119 Pa of the team's own flight computer (FLGT0023) on the pad (the ERA5 record, `descent-wind-era5.json`).
- `rit-testbed-2`: the Featherweight GPS tracker's log (`featherweight/data.csv`): latitude, longitude and the receiver's altitude. Its site and hour: the GPS log's summary: launched 18:10:57 UTC, the pad at 893 ft.

## Reconstructed less ERA5, by height band

Over the compared flights. A difference in speed is the reconstructed wind's speed less ERA5's; the vector difference is the length of the one wind less the other. Neighboring fixes share their fit's window, so the fixes of one flight are not independent. 0 read fixes lie outside ERA5's levels and are not compared.

| Band (m above the pad) | Flights | Fixes | Mean speed difference (m/s) | RMS speed difference (m/s) | RMS vector difference (m/s) |
|---|---|---|---|---|---|
| below 500 | 4 | 145 | +1.04 | 2.30 | 2.89 |
| 500 to 1,000 | 5 | 366 | -1.08 | 3.38 | 4.88 |
| 1,000 to 2,000 | 4 | 990 | -1.07 | 3.39 | 4.55 |
| 2,000 to 3,000 | 4 | 853 | -1.08 | 4.03 | 5.47 |
| 3,000 and up | 2 | 204 | +0.70 | 2.29 | 4.71 |
| all | 5 | 2558 | -0.81 | 3.49 | 4.86 |

## Two receivers on one rocket

Each receiver's track is reconstructed alone. The difference of their winds is a floor on the noise between two receivers, not the method's error: an error of the lag model, or a motion both receivers share, cancels in it.

- `prometheus-2022`, with the TeleMega's GPS (Altus Metrum), 1 fix a second: the two highest fixes are 5523 m and 5524 m above sea level and the last fixes 2.4 m apart. 170 fixes are read by both within 0.5 s; their winds differ by 0.51 m/s RMS, 1.35 m/s at most.

## What it leaves out

- **The canopy's own motion.** A canopy that glides moves across the wind by `w tan γ` at a glide angle `γ`; a swing on the shock cord moves the rocket back and forth. Neither is measured for hobby canopies, so neither is bounded: the differences above hold them along with ERA5's miss.
- **ERA5's hour and place.** ERA5 is read at the launch site and hour; the descent is minutes later and away from the site.
- **Drogues and tumbling bodies.** The lag `w/g` assumes the same drag in every direction; a tumbling airframe or a streamer has no measured response.
- **Each fix of the EuRoC logs.** CATS publishes EuRoC logs under GPL-3.0, and three of RocketPy's copies are files of RocketPy-Team's GPL-3.0 fork of its 2023 repository, so the logs of its flight computers give statistics here, not rows.
- **Public logs not searched.** The team logs are those a search of public repositories found with GPS fixes in the descent; a log not published, or not found, is not here.

## Sources

The ERA5 weather here is read from the files RocketPy's examples read: generated using Copernicus Climate Change Service information 2026, and contains modified Copernicus Climate Change Service information 2026. Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2023), *ERA5 hourly data on pressure levels from 1940 to present*, Copernicus Climate Change Service (C3S) Climate Data Store (CDS), <https://doi.org/10.24381/cds.bd0915c6>, licensed CC BY 4.0 (<https://creativecommons.org/licenses/by/4.0/>).
