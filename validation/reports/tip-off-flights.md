# What the tip-off phase moves on the real flights

Written by `cargo xtask tip-off-flights` ([M1.14b14][m1-14b14], decision [ADR-363][adr-363]); do not edit. HPR Sim's tip-off phase lets the rocket pivot about its last guide once the guide ahead of it has left the rail, instead of guiding it to that last guide ([the phase][flight]; held to RocketPy's in [the comparison](tip-off-model.md)). It is off by default. This report flies every real flight HPR Sim compares twice, guided as today and with the phase, before the phase becomes the default.

- **What is flown:** each flight as its own report flies it, the phase the only change: the seven flights [the real-flight report](real-flights.md) compared as soon as they were flown, the five it compares after their predictions were merged ([the predictions](public-predictions.md)), and the private collection's logged flights ([its report](fixture-flights.md#the-tip-off-phase-m114b14)), given in aggregate only.
- **Two guides:** a design with fewer than two guides (rail buttons or launch lugs) at different stations has no stretch on its last guide alone, so the phase moves nothing; such a flight is listed as not moved.
- **The errors** are each set's own report's: for the public flights, HPR Sim's apogee as the log's altimeter would read it, less the log's apogee, over the log's apogee; for the collection, the apogee as a height climbed in the day's air, against the logged apogee read the same way.
- **The move** is the apogee with the phase less the apogee guided; its share is of the apogee guided.
- **Landing points:** none of these flights has a landing point compared (the reports compare apogees; landing points against tracked landings are [M2.13][m2-13]), so no landing move is given.

[m1-14b14]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b14
[adr-363]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[flight]: https://hpr.fusionspace.co/physics/flight.html#what-leaving-tip-off-out-costs
[m2-13]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-13

## The errors, guided and with the phase

| set | flights | with two guides | mean absolute error, guided | with the phase | mean error, guided | with the phase | higher with the phase |
|---|---:|---:|---:|---:|---:|---:|---|
| compared first | 7 | 7 | 6.065% | 5.974% | +0.026% | +0.074% | no |
| predicted first | 5 | 3 | 12.818% | 12.819% | -11.161% | -11.167% | yes |
| the private collection | 60 | 23 | 12.095% | 12.098% | +8.007% | +8.015% | yes |

The mean absolute apogee error is higher with the phase over: predicted first, the private collection. ADR-363 asks for a decision record that measures the rise and its cause before the phase becomes the default.

## Where the change comes from

For each set: which way the phase moved the apogees of the flights with two guides, in the wind each report flies and in still air (the same air with its wind taken out, so that what moves is gravity's turn about the last guide alone), and whether the move took the apogee toward the log or away from it. The change in a set's mean absolute error is the sum of two parts: the rises in absolute error of the flights moved away, and the falls of those moved toward, each summed and divided by the set's flights.

| set | with two guides | raised | lowered | raised in still air | lowered in still air | moved away | moved toward | away's part | toward's part |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| compared first | 7 | 2 | 5 | 1 | 6 | 3 | 4 | +0.051 | -0.142 |
| predicted first | 3 | 0 | 3 | 0 | 3 | 2 | 1 | +0.004 | -0.002 |
| the private collection | 23 | 19 | 4 | 8 | 15 | 14 | 9 | +0.014 | -0.011 |

The parts are percentage points of the mean absolute error. The collection's part of fewer than five flights is held back, so as not to give a few flights' own figures.

## Each public flight

| flight | set | two guides | log apogee (m) | apogee guided (m) | with the phase (m) | move (m) | move | move in still air (m) | error guided | with the phase |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | compared first | yes | 459.0 | 464.03 | 464.06 | +0.03 | +0.006% | -0.02 | +1.103% | +1.109% |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | compared first | yes | 1320.4 | 1457.64 | 1457.50 | -0.14 | -0.010% | +0.00 | +10.397% | +10.387% |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | compared first | yes | 3895.8 | 3549.29 | 3536.63 | -12.66 | -0.357% | -13.70 | -8.895% | -9.220% |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | compared first | yes | 3151.5 | 2950.19 | 2976.69 | +26.49 | +0.898% | -16.72 | -6.388% | -5.547% |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | compared first | yes | 2785.0 | 2945.97 | 2945.48 | -0.50 | -0.017% | -0.99 | +5.780% | +5.762% |
| Genesis, EuRoC 2023 (L995) | compared first | yes | 2916.7 | 2745.96 | 2745.19 | -0.77 | -0.028% | -1.04 | -5.854% | -5.880% |
| Lince, EuRoC 2023 (M1101) | compared first | yes | 3587.7 | 3732.51 | 3728.03 | -4.48 | -0.120% | -4.11 | +4.036% | +3.911% |
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) | predicted first | yes | 3249.5 | 3037.11 | 3037.09 | -0.02 | -0.001% | -0.18 | -6.535% | -6.535% |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | predicted first | yes | 3438.5 | 3580.96 | 3580.59 | -0.37 | -0.010% | -0.60 | +4.142% | +4.131% |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) | predicted first | yes | 3001.8 | 2760.13 | 2759.62 | -0.51 | -0.018% | -0.56 | -8.050% | -8.066% |
| Macie XL, flight 1 (AeroTech I284W) | predicted first | no | 842.8 | 706.24 | not moved | | | | -16.200% | -16.200% |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | predicted first | no | 977.8 | 692.64 | not moved | | | | -29.164% | -29.164% |

## The private collection

Of the 60 flights compared with their logs, 23 fly a design with two or more guides at different stations: 20 of the 42 designs. No flight is named or numbered here and no apogee of one is given (decision records ADR-151 and ADR-184).

The flights with two guides by the phase's apogee move in the day's wind, in bins of 0.1% of the apogee guided, a bin holding fewer than five merged with its smaller neighbor:

| move | flights |
|---|---:|
| -0.4% to +0.4% | 23 |

The flights with two guides by the phase's apogee move in still air (the day's air with its wind taken out, so gravity's part alone), in bins of 0.1% of the apogee guided, a bin holding fewer than five merged with its smaller neighbor:

| move | flights |
|---|---:|
| -0.4% to +0.0% | 15 |
| +0.0% to +0.1% | 8 |

