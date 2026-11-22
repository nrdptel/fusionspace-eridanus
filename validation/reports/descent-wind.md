# Wind from the GPS descent, against ERA5

Written by `cargo xtask descent-wind` (M1.14b6); don't edit it by hand. The wind is reconstructed from each public flight's GPS track of its descent as the ground velocity plus `w/g` times its rate of change, `w` the descent rate (Hock and Franklin 1999; [the method](../../docs/physics/wind.md#wind-from-a-gps-tracked-descent)), and set beside the ERA5 wind HPR Sim flies the flight in.

**A method, not yet a comparison.** 2 of the public flights are compared with ERA5; the plan asks for at least 3 before the differences below are read as ERA5's miss at a launch field.

**Settings.** A quadratic in time fitted over 10 s around each fix, at least 5 fixes reaching a quarter of the window on each side; a fix is read where the body falls at 2 to 35 m/s with a vertical acceleration of at most 0.5 m/s²; `g` = 9.80665 m/s².

## The flights

| Flight | Status | Fixes | Read | Heights read (m above the pad) | Descent (m/s) | Lag correction, mean (m/s) | Speed less ERA5's, mean (m/s) |
|---|---|---|---|---|---|---|---|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) (`bella-lui`) | not run: its log holds a filtered height and speed only, no position | 0 | 0 |  |  |  |  |
| NDRT 2020, Notre Dame Rocketry Team (L1395) (`ndrt-2020`) | not run: its log holds acceleration and a barometric height only, no position | 0 | 0 |  |  |  |  |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) (`prometheus-2022`) | not compared: RocketPy's ERA5 file for it is of 24 June 2023, a year after the flight, and no file of its day is pinned | 239 | 173 | 28 to 3927 | 5.6 to 32.0 | 1.54 |  |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) (`juno-iii`) | compared | 383 | 102 | 421 to 3210 | 23.7 to 27.2 | 0.61 | +1.53 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) (`cavour`) | not run: no fix falls steadily inside the settings: the median descent rate after apogee is 75 m/s, past the 35 m/s the method is held to, so no canopy held the fall | 110 | 0 |  |  |  |  |
| Genesis, EuRoC 2023 (L995) (`genesis`) | compared | 140 | 56 | 44 to 666 | 9.7 to 12.4 | 0.82 | +1.14 |
| Lince, EuRoC 2023 (M1101) (`lince`) | not run: its log holds a filtered height and acceleration only, no position | 0 | 0 |  |  |  |  |

The receivers read:

- `prometheus-2022`: the TeleMetrum's GPS (Altus Metrum), 1 fix a second.
- `juno-iii`: the COTS GNSS log (`cots_GNSS.csv`), 1 fix a second in the descent; the team's own telemetry, which the README calls possibly noisy, is not read.
- `cavour`: the CATS Vega's radio telemetry: latitude and longitude sent to four decimals (about 11 m north and 9 m east), height from its barometric filter.
- `genesis`: the GNSS log (`gnss_faraday.csv`): latitude and longitude, 1 to 10 fixes a second; height from the flight log's barometric filter.

## Reconstructed less ERA5, by height band

Over the compared flights. A difference in speed is the reconstructed wind's speed less ERA5's; the vector difference is the length of the one wind less the other. The samples of one flight are not independent: each fix's fit shares its window with its neighbors', and one flight's wind and ERA5's miss are each correlated over hundreds of meters.

| Band (m above the pad) | Flights | Samples | Mean speed difference (m/s) | RMS speed difference (m/s) | RMS vector difference (m/s) |
|---|---|---|---|---|---|
| 0 to 500 | 2 | 58 | +0.44 | 2.92 | 3.39 |
| 500 to 1000 | 2 | 21 | -4.86 | 5.75 | 5.90 |
| 1000 to 2000 | 1 | 40 | +2.47 | 2.62 | 2.98 |
| 2000 to 3000 | 1 | 32 | +4.80 | 4.82 | 6.57 |
| 3000 to 5000 | 1 | 7 | +6.19 | 6.20 | 8.97 |
| all | 2 | 158 | +1.39 | 3.96 | 4.83 |

## Two receivers on one rocket

Both rode the same rocket, so the canopy's glide and swing are the same in both: their difference is the receivers' and the method's own noise.

- `prometheus-2022`, with the TeleMega's GPS (Altus Metrum), 1 fix a second: 169 fixes read by both within 0.5 s; their winds differ by 0.65 m/s RMS, 2.25 m/s at most.

## What it leaves out

- **The canopy's own motion.** A canopy that glides moves across the wind by `w tan γ` at a glide angle `γ`; a swing on the shock cord moves the rocket back and forth. Neither is measured for hobby canopies, so neither is bounded: the differences above hold them along with ERA5's miss.
- **ERA5's hour and place.** ERA5 is read at the hour the real-flights report flies the flight in and at the launch site; the descent is minutes later and drifts hundreds of meters.
- **Drogues and tumbling bodies.** The lag `w/g` assumes the same drag in every direction; a tumbling airframe or a streamer has no measured response.
