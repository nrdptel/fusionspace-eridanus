# What the tip-off phase moves on the real flights

Written by `cargo xtask tip-off-flights` ([M1.14b14][m1-14b14], decision [ADR-363][adr-363]; the result in [ADR-364][adr-364]); do not edit. HPR Sim's tip-off phase lets the rocket pivot about its last guide once the guide ahead of it has left the rail ([the phase][flight]; compared with RocketPy's in [the comparison](tip-off-model.md), held in still air and not yet in a crosswind on HPR Sim's own normal force). It is off by default. This report flies every real flight HPR Sim compares with the phase and without it, before the phase becomes the default. It compares HPR Sim with itself: it does not check the phase against a measurement.

- **What is flown:** each flight as its own report flies it, the phase the only change: the seven flights [the real-flight report](real-flights.md) compared as soon as they were flown (*compared first*), the five it compares after their predictions were merged (*predicted first*; [the predictions](public-predictions.md)), and the private collection's logged flights ([its report](fixture-flights.md#the-tip-off-phase-m114b14)), given in aggregate only.
- **Guided** is the flight as HPR Sim flies it by default: held straight to its last guide, then freed with no turning rate.
- **Two guides:** a design with fewer than two guides (rail buttons or launch lugs) at different stations has no stretch on its last guide alone, so the phase moves nothing; such a flight is listed as not moved.
- **Still air:** each flight with two guides flies again, guided and with the phase, in its report's air with the wind taken out. What the phase moves there is gravity's turn about the last guide alone. Taking out the wind changes the guided flight too, so the move in still air and the move in the wind are not two parts of one sum.
- **The errors** are each set's own report's: for the public flights, HPR Sim's apogee as the log's altimeter would read it, less the log's apogee, over the log's apogee; for the collection, the apogee as a height climbed in the day's air, against the logged apogee read the same way.
- **The move** is the apogee with the phase less the apogee guided; its share is of the apogee guided.
- **Landing points:** none of these flights has a landing point compared (the reports compare apogees; landing points against tracked landings are [M2.13][m2-13]), so no landing move is given.

[m1-14b14]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b14
[adr-363]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[adr-364]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0364-the-tip-off-phase-on-the-real-flights-two-small-rises-measured.md
[adr-151]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-184]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0184-logged-apogees-of-the-private-collection.md
[flight]: https://hpr.fusionspace.co/physics/flight.html#what-leaving-tip-off-out-costs
[m2-13]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-13

## The errors, guided and with the phase

| set | flights | with two guides | mean absolute error, guided | mean absolute error, with the phase | change (points) | mean signed error, guided | mean signed error, with the phase |
|---|---:|---:|---:|---:|---:|---:|---:|
| compared first | 7 | 7 | 6.065% | 5.974% | -0.0909 | +0.026% | +0.074% |
| predicted first | 5 | 3 | 12.818% | 12.819% | +0.0014 | -11.161% | -11.167% |
| the private collection | 60 | 23 | 12.095% | 12.098% | +0.0033 | +8.007% | +8.015% |

With the phase the mean absolute apogee error is higher over predicted first by 0.0014 points, the private collection by 0.0033 points, and lower over compared first by 0.0909 points. [ADR-364][adr-364] measures the rises and their cause, as [ADR-363][adr-363] asks before the phase becomes the default.

## Where the change comes from

For each set: which way the phase moved the apogees of the flights with two guides, in the wind each report flies and in still air, and whether the move took the apogee toward the log or away from it. A move under 1 cm (0.4 in) either way counts as not moved: on a vertical rail in still air gravity has no moment about the last guide. The change in a set's mean absolute error is the sum of two parts: the rises in absolute error of the flights moved away, and the falls of those moved toward, each summed and divided by the set's flights. The parts are rounded on their own, so their sum may differ from the change in its last digit.

| set | with two guides | in the wind: raised, not moved, lowered | in still air: raised, not moved, lowered | moved away | moved toward | away's part (points) | toward's part (points) |
|---|---:|---|---|---:|---:|---:|---:|
| compared first | 7 | 2, 0, 5 | 0, 1, 6 | 3 | 4 | +0.0511 | -0.1420 |
| predicted first | 3 | 0, 0, 3 | 0, 0, 3 | 2 | 1 | +0.0035 | -0.0021 |
| the private collection | 23 | held back | held back | 14 | 9 | +0.0142 | -0.0110 |

The parts are percentage points of the mean absolute error. For the private collection a group of counts is given only where each count is none or at least five. The counts moved away and toward and the two parts are given only where the flights moved away, those moved toward and those whose error did not change each number none or at least five; otherwise they are held back together, so that no figure gives a few flights' own moves.

## Each public flight

| flight | set | two guides | log apogee (m) | apogee guided (m) | with the phase (m) | move (m) | move | move in still air (m) | error guided | error with the phase |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | compared first | yes | 459.0 | 464.03 | 464.06 | +0.03 | +0.006% | -0.019 | +1.103% | +1.109% |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | compared first | yes | 1320.4 | 1457.64 | 1457.50 | -0.14 | -0.010% | +0.001 | +10.397% | +10.387% |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | compared first | yes | 3895.8 | 3549.29 | 3536.63 | -12.66 | -0.357% | -13.703 | -8.895% | -9.220% |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | compared first | yes | 3151.5 | 2950.19 | 2976.69 | +26.49 | +0.898% | -16.720 | -6.388% | -5.547% |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | compared first | yes | 2785.0 | 2945.97 | 2945.48 | -0.50 | -0.017% | -0.987 | +5.780% | +5.762% |
| Genesis, EuRoC 2023 (L995) | compared first | yes | 2916.7 | 2745.96 | 2745.19 | -0.77 | -0.028% | -1.043 | -5.854% | -5.880% |
| Lince, EuRoC 2023 (M1101) | compared first | yes | 3587.7 | 3732.51 | 3728.03 | -4.48 | -0.120% | -4.108 | +4.036% | +3.911% |
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) | predicted first | yes | 3249.5 | 3037.11 | 3037.09 | -0.02 | -0.001% | -0.181 | -6.535% | -6.535% |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | predicted first | yes | 3438.5 | 3580.96 | 3580.59 | -0.37 | -0.010% | -0.600 | +4.142% | +4.131% |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) | predicted first | yes | 3001.8 | 2760.13 | 2759.62 | -0.51 | -0.018% | -0.556 | -8.050% | -8.066% |
| Macie XL, flight 1 (AeroTech I284W) | predicted first | no | 842.8 | 706.24 | not moved | | | | -16.200% | -16.200% |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | predicted first | no | 977.8 | 692.64 | not moved | | | | -29.164% | -29.164% |

## The private collection

Of the 60 flights compared with their logs, 23 fly a design with two or more guides at different stations: 20 of the 42 designs. 11 of those flights fly from a vertical rail. No flight is named or numbered here and no apogee of one is given: the collection's terms allow aggregates only ([ADR-151][adr-151], [ADR-184][adr-184]).

The phase's apogee moves in the day's wind, for the flights with two guides, in bins of 0.1% of the apogee guided, each from its lower edge up to its upper one, a bin holding fewer than five merged with its smaller neighbor:

| move | flights |
|---|---:|
| -0.4% to +0.4% | 23 |

One bin holds all 23 flights.

The phase's apogee moves in still air, for the flights with two guides, in bins of 0.1% of the apogee guided, each from its lower edge up to its upper one, a bin holding fewer than five merged with its smaller neighbor:

| move | flights |
|---|---:|
| -0.4% to 0.0% | 15 |
| 0.0% to +0.1% | 8 |

