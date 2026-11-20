# The fit's 90% intervals: least squares and Bayesian

On sets A and B of the synthetic flights, each parameter's 90% interval is held to three counts, each with a two-sided 99.9% Clopper–Pearson band: the two tails' bands must hold 5% and the inside's 90%. The least-squares fit's intervals take shares fitted on the calibration set; the Bayesian option's are its posterior's 5% and 95% points as they come, on the same flights.

Written by `cargo xtask fit`. How to read it: [the fit's page](../../docs/physics/flight-fit.md).

## Set A: 1000 flights counted

Of its first 1216, 216 had nothing to fit and aren't counted, 216 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 3 reached the fit's step limit. The largest residual of any of them is 0.0208 of the spread of the heights read; past 0.1 a fit says nothing.

### Least squares

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 34 | 903 | 63 | passes | fails, fails, fails | -0.00173 | 0.115 / 0.36 | yes |
| impulse scale | 34 | 910 | 56 | passes | fails, fails, fails | 0.000364 | 0.04 / 0.108 | yes |
| wind toward the east, m/s | 90 | 1346 | 61 | passes | fails, fails, fails | -0.00706 | 0.348 / 12.7 | yes |
| wind toward the north, m/s | 66 | 1366 | 65 | passes | fails, fails, fails | 0.00584 | 0.372 / 12.7 | yes |
| mass on the weighed mass | 40 | 911 | 49 | passes | fails, fails, fails | -0.000398 | 0.035 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 3 / 76 / 13 | 1 / 88 / 3 | 10 / 126 / 10 | 8 / 131 / 7 | 5 / 84 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 87 / 4 | 3 / 87 / 6 | 8 / 140 / 6 | 5 / 147 / 2 | 3 / 86 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 3 / 96 / 3 | 4 / 91 / 7 | 9 / 131 / 1 | 6 / 129 / 6 | 2 / 96 / 4 |
| rocketpy-cavour | 6 / 94 / 0 | 6 / 91 / 3 | 18 / 146 / 7 | 7 / 147 / 17 | 9 / 86 / 5 |
| rocketpy-genesis | 6 / 95 / 6 | 4 / 93 / 10 | 12 / 140 / 2 | 7 / 141 / 6 | 4 / 94 / 9 |
| rocketpy-juno-iii | 3 / 82 / 8 | 1 / 91 / 1 | 5 / 136 / 7 | 6 / 137 / 5 | 4 / 85 / 4 |
| rocketpy-lince | 1 / 84 / 7 | 3 / 80 / 9 | 9 / 129 / 12 | 12 / 128 / 10 | 5 / 80 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 1 / 118 / 4 | 6 / 110 / 7 | 2 / 142 / 2 | 3 / 143 / 0 | 3 / 117 / 3 |
| rocketpy-prometheus-2022-generic-motor | 4 / 82 / 10 | 5 / 85 / 6 | 12 / 132 / 13 | 10 / 138 / 9 | 2 / 88 / 6 |
| rocketpy-valetudo | 2 / 89 / 8 | 1 / 94 / 4 | 5 / 124 / 1 | 2 / 125 / 3 | 3 / 95 / 1 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 18 / 602 / 26 | 12 / 600 / 34 | 57 / 1096 / 28 | 44 / 1103 / 34 | 21 / 599 / 26 |
| heights fit to 1 m or worse | 16 / 301 / 37 | 22 / 310 / 22 | 33 / 250 / 33 | 22 / 263 / 31 | 19 / 312 / 23 |

### Bayesian

The posterior's 5% and 95% points, as they come. Of the 2276 flights counted here or for the wind with a posterior drawn, 559 fell short of 100 effective draws; their effective draws run from 1, with a 10% point of 46.8 and a median of 153. Of the 1138000 draws made, the pilots' included, 129 failed; 0 flights with something to fit have no posterior.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 30 | 885 | 85 | fails | fails, fails, fails | -0.0106 | 0.0919 / 0.36 | yes |
| impulse scale | 72 | 875 | 53 | passes | fails, fails, fails | 0.00133 | 0.0391 / 0.108 | yes |
| wind toward the east, m/s | 57 | 1381 | 59 | passes | fails, fails, fails | -0.00861 | 0.36 / 12.7 | yes |
| wind toward the north, m/s | 56 | 1382 | 59 | passes | fails, fails, fails | 0.00981 | 0.357 / 12.7 | yes |
| mass on the weighed mass | 56 | 882 | 62 | passes | fails, fails, fails | -0.000173 | 0.034 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 6 / 80 / 6 | 8 / 80 / 4 | 4 / 129 / 13 | 7 / 134 / 5 | 8 / 76 / 8 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 / 79 / 13 | 4 / 85 / 7 | 3 / 146 / 5 | 8 / 142 / 4 | 3 / 86 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 1 / 96 / 5 | 4 / 93 / 5 | 6 / 132 / 3 | 4 / 129 / 8 | 3 / 93 / 6 |
| rocketpy-cavour | 5 / 91 / 4 | 9 / 89 / 2 | 8 / 157 / 6 | 9 / 152 / 10 | 13 / 81 / 6 |
| rocketpy-genesis | 2 / 92 / 13 | 7 / 90 / 10 | 6 / 142 / 6 | 3 / 144 / 7 | 5 / 92 / 10 |
| rocketpy-juno-iii | 3 / 79 / 11 | 10 / 80 / 3 | 3 / 141 / 4 | 3 / 142 / 3 | 7 / 81 / 5 |
| rocketpy-lince | 1 / 81 / 10 | 5 / 79 / 8 | 9 / 138 / 3 | 8 / 138 / 4 | 7 / 79 / 6 |
| rocketpy-ndrt-2020-nose-to-tail | 1 / 116 / 6 | 10 / 109 / 4 | 6 / 134 / 6 | 6 / 136 / 4 | 4 / 114 / 5 |
| rocketpy-prometheus-2022-generic-motor | 5 / 78 / 13 | 8 / 81 / 7 | 7 / 139 / 11 | 5 / 144 / 8 | 3 / 85 / 8 |
| rocketpy-valetudo | 2 / 93 / 4 | 7 / 89 / 3 | 5 / 123 / 2 | 3 / 121 / 6 | 3 / 95 / 1 |

## Set B: 1000 flights counted

Of its first 1193, 193 had nothing to fit and aren't counted, 193 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 8 reached the fit's step limit. The largest residual of any of them is 0.0265 of the spread of the heights read; past 0.1 a fit says nothing.

### Least squares

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 71 | 900 | 29 | passes | fails, fails, fails | 0.00522 | 0.123 / 0.36 | yes |
| impulse scale | 54 | 909 | 37 | passes | fails, fails, fails | 0.00164 | 0.0409 / 0.108 | yes |
| wind toward the east, m/s | 76 | 1352 | 69 | passes | fails, fails, fails | -0.0224 | 0.411 / 12.7 | yes |
| wind toward the north, m/s | 73 | 1360 | 64 | passes | fails, fails, fails | -0.00695 | 0.445 / 12.7 | yes |
| mass on the weighed mass | 41 | 900 | 59 | passes | fails, fails, fails | -0.000138 | 0.035 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 0 / 97 / 0 | 0 / 96 / 1 | 6 / 131 / 7 | 4 / 133 / 7 | 4 / 90 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 96 / 0 | 7 / 90 / 4 | 7 / 160 / 5 | 4 / 165 / 3 | 7 / 86 / 8 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 11 / 101 / 5 | 2 / 107 / 8 | 3 / 149 / 4 | 2 / 152 / 2 | 4 / 101 / 12 |
| rocketpy-cavour | 7 / 85 / 1 | 5 / 87 / 1 | 11 / 114 / 9 | 12 / 111 / 11 | 2 / 85 / 6 |
| rocketpy-genesis | 13 / 92 / 2 | 5 / 98 / 4 | 6 / 120 / 7 | 8 / 122 / 3 | 0 / 103 / 4 |
| rocketpy-juno-iii | 5 / 93 / 5 | 3 / 97 / 3 | 9 / 142 / 5 | 11 / 140 / 5 | 6 / 91 / 6 |
| rocketpy-lince | 8 / 82 / 5 | 14 / 76 / 5 | 12 / 121 / 15 | 15 / 122 / 11 | 6 / 82 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 3 / 79 / 1 | 1 / 80 / 2 | 2 / 132 / 0 | 1 / 133 / 0 | 3 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 19 / 68 / 9 | 17 / 75 / 4 | 17 / 131 / 13 | 13 / 127 / 21 | 5 / 89 / 2 |
| rocketpy-valetudo | 0 / 107 / 1 | 0 / 103 / 5 | 3 / 152 / 4 | 3 / 155 / 1 | 4 / 97 / 7 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 19 / 530 / 1 | 14 / 517 / 19 | 29 / 919 / 32 | 32 / 925 / 23 | 19 / 496 / 35 |
| heights fit to 1 m or worse | 52 / 370 / 28 | 40 / 392 / 18 | 47 / 433 / 37 | 41 / 435 / 41 | 22 / 404 / 24 |

### Bayesian

The posterior's 5% and 95% points, as they come. Of the 2296 flights counted here or for the wind with a posterior drawn, 712 fell short of 100 effective draws; their effective draws run from 1, with a 10% point of 28.6 and a median of 138. Of the 1148000 draws made, the pilots' included, 152 failed; 0 flights with something to fit have no posterior.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 74 | 872 | 54 | passes | fails, passes, fails | -0.00324 | 0.0982 / 0.36 | yes |
| impulse scale | 84 | 883 | 33 | fails | fails, fails, fails | 0.00241 | 0.0402 / 0.108 | yes |
| wind toward the east, m/s | 58 | 1353 | 86 | passes | fails, fails, fails | -0.0362 | 0.427 / 12.7 | yes |
| wind toward the north, m/s | 70 | 1348 | 79 | passes | fails, fails, fails | -0.0302 | 0.443 / 12.7 | yes |
| mass on the weighed mass | 63 | 869 | 68 | passes | fails, fails, fails | -0.000115 | 0.034 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 0 / 97 / 0 | 7 / 90 / 0 | 3 / 137 / 4 | 5 / 136 / 3 | 7 / 86 / 4 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 / 92 / 5 | 11 / 84 / 6 | 2 / 167 / 3 | 4 / 165 / 3 | 9 / 82 / 10 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 9 / 103 / 5 | 5 / 109 / 3 | 0 / 149 / 7 | 3 / 149 / 4 | 7 / 99 / 11 |
| rocketpy-cavour | 10 / 80 / 3 | 9 / 82 / 2 | 10 / 111 / 13 | 6 / 116 / 12 | 5 / 79 / 9 |
| rocketpy-genesis | 10 / 91 / 6 | 6 / 97 / 4 | 6 / 119 / 8 | 7 / 115 / 11 | 5 / 98 / 4 |
| rocketpy-juno-iii | 10 / 82 / 11 | 7 / 89 / 7 | 3 / 147 / 6 | 6 / 144 / 6 | 8 / 84 / 11 |
| rocketpy-lince | 12 / 73 / 10 | 15 / 74 / 6 | 13 / 119 / 16 | 14 / 121 / 13 | 7 / 81 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 80 / 3 | 1 / 81 / 1 | 4 / 128 / 2 | 2 / 130 / 2 | 3 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 19 / 66 / 11 | 20 / 74 / 2 | 15 / 125 / 21 | 21 / 121 / 19 | 7 / 89 / 0 |
| rocketpy-valetudo | 0 / 108 / 0 | 3 / 103 / 2 | 2 / 151 / 6 | 2 / 151 / 6 | 5 / 95 / 8 |

## Pure noise in place of the log

The smallest residual is that of the fits that came to one, a share of the spread of the heights read; past 0.1 a log says nothing.

| Height noise | Track noise | Reaches | Flights | With nothing to fit | Smallest residual | Parameter | Median width / stated, least squares | Identified | Median width / stated, Bayesian | Identified |
|---:|---:|---|---:|---:|---:|---|---:|---|---:|---|
| ±5 m | none | the reader | 200 | 200 | none | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | none | the reader | 200 | 200 | none | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.994 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.994 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.994 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.994 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.994 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.982 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.982 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.982 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.982 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.982 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.957 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.957 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.957 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.957 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.957 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
