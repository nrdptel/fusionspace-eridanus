# The fit's 90% intervals: least squares and Bayesian

On sets A and B of the synthetic flights, each parameter's 90% interval is held to three counts, each with a two-sided 99.9% Clopper–Pearson band: the two tails' bands must hold 5% and the inside's 90%. The least-squares fit's intervals take shares fitted on the calibration set; the Bayesian option's are its posterior's 5% and 95% points as they come, on the same flights.

Written by `cargo xtask fit`. How to read it: [the fit's page](../../docs/physics/flight-fit.md).

## Set A: 1000 flights counted

Of its first 1216, 216 had nothing to fit and aren't counted, 216 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 2 reached the fit's step limit. The largest residual of any of them is 0.0227 of the spread of the heights read; past 0.1 a fit says nothing.

### Least squares

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 32 | 907 | 61 | passes | fails, fails, fails | -0.00155 | 0.117 / 0.36 | yes |
| impulse scale | 34 | 913 | 53 | passes | fails, fails, fails | 0.000262 | 0.0406 / 0.108 | yes |
| wind toward the east, m/s | 82 | 1345 | 70 | passes | fails, fails, fails | -0.00477 | 0.376 / 12.7 | yes |
| wind toward the north, m/s | 58 | 1380 | 59 | passes | fails, fails, fails | 0.00907 | 0.41 / 12.7 | yes |
| mass on the weighed mass | 39 | 910 | 51 | passes | fails, fails, fails | -0.000389 | 0.0349 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 2 / 77 / 13 | 1 / 88 / 3 | 10 / 125 / 11 | 7 / 132 / 7 | 5 / 83 / 4 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 / 88 / 4 | 3 / 88 / 5 | 5 / 142 / 7 | 4 / 148 / 2 | 3 / 86 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 3 / 97 / 2 | 4 / 91 / 7 | 9 / 131 / 1 | 4 / 132 / 5 | 2 / 96 / 4 |
| rocketpy-cavour | 6 / 94 / 0 | 6 / 92 / 2 | 18 / 144 / 9 | 6 / 148 / 17 | 8 / 87 / 5 |
| rocketpy-genesis | 4 / 97 / 6 | 5 / 93 / 9 | 9 / 143 / 2 | 5 / 146 / 3 | 4 / 93 / 10 |
| rocketpy-juno-iii | 4 / 80 / 9 | 1 / 91 / 1 | 5 / 134 / 9 | 6 / 140 / 2 | 4 / 85 / 4 |
| rocketpy-lince | 2 / 84 / 6 | 3 / 80 / 9 | 8 / 128 / 14 | 12 / 128 / 10 | 5 / 80 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 1 / 118 / 4 | 6 / 110 / 7 | 2 / 142 / 2 | 2 / 143 / 1 | 3 / 117 / 3 |
| rocketpy-prometheus-2022-generic-motor | 5 / 81 / 10 | 4 / 86 / 6 | 11 / 132 / 14 | 8 / 141 / 8 | 2 / 88 / 6 |
| rocketpy-valetudo | 1 / 91 / 7 | 1 / 94 / 4 | 5 / 124 / 1 | 4 / 122 / 4 | 3 / 95 / 1 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 15 / 604 / 22 | 10 / 600 / 31 | 49 / 1096 / 32 | 35 / 1110 / 32 | 20 / 596 / 25 |
| heights fit to 1 m or worse | 17 / 303 / 39 | 24 / 313 / 22 | 33 / 249 / 38 | 23 / 270 / 27 | 19 / 314 / 26 |

By the shares' class, each held to the test (below / inside / above):

| Class | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| misfit under 0.0634% of the heights' spread | 2 / 89 / 4 | 0 / 91 / 4 | 2 / 70 / 2 | 1 / 72 / 1 | 1 / 89 / 5 |
| misfit at 0.0634% of the heights' spread or more | 30 / 818 / 57 | 34 / 822 / 49 | 80 / 1275 / 68 | 57 / 1308 / 58 | 38 / 821 / 46 |

### Bayesian

The posterior's 5% and 95% points, as they come. Of the 2276 flights counted here or for the wind with a posterior drawn, 542 fell short of 100 effective draws; their effective draws run from 1, with a 10% point of 49.4 and a median of 154. Of the 1138000 draws made, the pilots' included, 170 failed; 0 flights with something to fit have no posterior.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 30 | 885 | 85 | fails | fails, fails, fails | -0.0103 | 0.0916 / 0.36 | yes |
| impulse scale | 70 | 877 | 53 | passes | fails, fails, fails | 0.0012 | 0.039 / 0.108 | yes |
| wind toward the east, m/s | 60 | 1382 | 55 | passes | fails, fails, fails | -0.00432 | 0.383 / 12.7 | yes |
| wind toward the north, m/s | 59 | 1382 | 56 | passes | fails, fails, fails | 0.011 | 0.395 / 12.7 | yes |
| mass on the weighed mass | 53 | 885 | 62 | passes | fails, fails, fails | -0.000118 | 0.034 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 5 / 79 / 8 | 8 / 80 / 4 | 5 / 129 / 12 | 7 / 134 / 5 | 8 / 77 / 7 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 80 / 11 | 5 / 85 / 6 | 4 / 146 / 4 | 7 / 142 / 5 | 3 / 86 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 1 / 96 / 5 | 4 / 93 / 5 | 7 / 130 / 4 | 4 / 130 / 7 | 2 / 94 / 6 |
| rocketpy-cavour | 6 / 89 / 5 | 9 / 88 / 3 | 8 / 158 / 5 | 9 / 154 / 8 | 12 / 82 / 6 |
| rocketpy-genesis | 3 / 93 / 11 | 5 / 91 / 11 | 6 / 144 / 4 | 4 / 143 / 7 | 5 / 92 / 10 |
| rocketpy-juno-iii | 2 / 79 / 12 | 8 / 83 / 2 | 1 / 145 / 2 | 6 / 139 / 3 | 7 / 81 / 5 |
| rocketpy-lince | 1 / 81 / 10 | 6 / 76 / 10 | 10 / 136 / 4 | 8 / 136 / 6 | 7 / 78 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 117 / 6 | 10 / 110 / 3 | 7 / 133 / 6 | 6 / 136 / 4 | 4 / 114 / 5 |
| rocketpy-prometheus-2022-generic-motor | 5 / 78 / 13 | 7 / 82 / 7 | 9 / 136 / 12 | 6 / 145 / 6 | 3 / 85 / 8 |
| rocketpy-valetudo | 2 / 93 / 4 | 8 / 89 / 2 | 3 / 125 / 2 | 2 / 123 / 5 | 2 / 96 / 1 |

## Set B: 1000 flights counted

Of its first 1193, 193 had nothing to fit and aren't counted, 193 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 6 reached the fit's step limit. The largest residual of any of them is 0.0254 of the spread of the heights read; past 0.1 a fit says nothing.

### Least squares

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 74 | 895 | 31 | passes | fails, fails, fails | 0.00728 | 0.127 / 0.36 | yes |
| impulse scale | 51 | 912 | 37 | passes | fails, fails, fails | 0.00128 | 0.0414 / 0.108 | yes |
| wind toward the east, m/s | 74 | 1350 | 73 | passes | fails, fails, fails | -0.0251 | 0.436 / 12.7 | yes |
| wind toward the north, m/s | 66 | 1370 | 61 | passes | fails, fails, fails | -0.0055 | 0.496 / 12.7 | yes |
| mass on the weighed mass | 43 | 900 | 57 | passes | fails, fails, fails | -0.000133 | 0.0349 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 0 / 97 / 0 | 0 / 96 / 1 | 6 / 131 / 7 | 3 / 134 / 7 | 5 / 89 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 / 97 / 0 | 7 / 90 / 4 | 8 / 159 / 5 | 4 / 165 / 3 | 7 / 86 / 8 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 11 / 102 / 4 | 2 / 109 / 6 | 5 / 147 / 4 | 2 / 152 / 2 | 4 / 101 / 12 |
| rocketpy-cavour | 8 / 82 / 3 | 5 / 87 / 1 | 10 / 114 / 10 | 14 / 109 / 11 | 2 / 85 / 6 |
| rocketpy-genesis | 14 / 89 / 4 | 4 / 99 / 4 | 5 / 125 / 3 | 5 / 127 / 1 | 0 / 103 / 4 |
| rocketpy-juno-iii | 8 / 91 / 4 | 3 / 96 / 4 | 11 / 137 / 8 | 6 / 145 / 5 | 5 / 92 / 6 |
| rocketpy-lince | 9 / 82 / 4 | 11 / 79 / 5 | 9 / 125 / 14 | 14 / 124 / 10 | 6 / 84 / 5 |
| rocketpy-ndrt-2020-nose-to-tail | 3 / 79 / 1 | 1 / 81 / 1 | 2 / 132 / 0 | 1 / 133 / 0 | 3 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 16 / 70 / 10 | 18 / 74 / 4 | 15 / 130 / 16 | 15 / 125 / 21 | 7 / 87 / 2 |
| rocketpy-valetudo | 1 / 106 / 1 | 0 / 101 / 7 | 3 / 150 / 6 | 2 / 156 / 1 | 4 / 97 / 7 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 20 / 537 / 4 | 11 / 531 / 19 | 28 / 921 / 32 | 25 / 936 / 20 | 20 / 506 / 35 |
| heights fit to 1 m or worse | 54 / 358 / 27 | 40 / 381 / 18 | 46 / 429 / 41 | 41 / 434 / 41 | 23 / 394 / 22 |

By the shares' class, each held to the test (below / inside / above):

| Class | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| misfit under 0.0634% of the heights' spread | 1 / 24 / 2 | 0 / 26 / 1 | 1 / 10 / 0 | 0 / 11 / 0 | 0 / 26 / 1 |
| misfit at 0.0634% of the heights' spread or more | 73 / 871 / 29 | 51 / 886 / 36 | 73 / 1340 / 73 | 66 / 1359 / 61 | 43 / 874 / 56 |

### Bayesian

The posterior's 5% and 95% points, as they come. Of the 2296 flights counted here or for the wind with a posterior drawn, 722 fell short of 100 effective draws; their effective draws run from 1, with a 10% point of 24.6 and a median of 139. Of the 1148000 draws made, the pilots' included, 281 failed; 0 flights with something to fit have no posterior.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 78 | 867 | 55 | fails | fails, passes, fails | -0.0011 | 0.0978 / 0.36 | yes |
| impulse scale | 80 | 884 | 36 | fails | fails, fails, fails | 0.00212 | 0.04 / 0.108 | yes |
| wind toward the east, m/s | 67 | 1344 | 86 | passes | fails, fails, fails | -0.027 | 0.463 / 12.7 | yes |
| wind toward the north, m/s | 85 | 1321 | 91 | passes | fails, fails, fails | -0.0245 | 0.475 / 12.7 | yes |
| mass on the weighed mass | 61 | 870 | 69 | passes | fails, fails, fails | -0.0000781 | 0.034 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 0 / 97 / 0 | 7 / 90 / 0 | 4 / 135 / 5 | 4 / 137 / 3 | 7 / 86 / 4 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 3 / 92 / 6 | 11 / 84 / 6 | 2 / 167 / 3 | 5 / 162 / 5 | 9 / 82 / 10 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 9 / 102 / 6 | 4 / 109 / 4 | 1 / 147 / 8 | 4 / 147 / 5 | 7 / 99 / 11 |
| rocketpy-cavour | 9 / 82 / 2 | 8 / 83 / 2 | 9 / 111 / 14 | 9 / 112 / 13 | 4 / 80 / 9 |
| rocketpy-genesis | 13 / 87 / 7 | 4 / 99 / 4 | 9 / 118 / 6 | 8 / 115 / 10 | 4 / 100 / 3 |
| rocketpy-juno-iii | 10 / 85 / 8 | 6 / 89 / 8 | 4 / 149 / 3 | 6 / 143 / 7 | 7 / 83 / 13 |
| rocketpy-lince | 12 / 73 / 10 | 14 / 76 / 5 | 13 / 119 / 16 | 18 / 113 / 17 | 8 / 80 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 80 / 3 | 1 / 81 / 1 | 4 / 127 / 3 | 2 / 130 / 2 | 3 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 21 / 62 / 13 | 20 / 73 / 3 | 18 / 121 / 22 | 26 / 109 / 26 | 7 / 88 / 1 |
| rocketpy-valetudo | 1 / 107 / 0 | 5 / 100 / 3 | 3 / 150 / 6 | 3 / 153 / 3 | 5 / 96 / 7 |

## Pure noise in place of the log

The smallest residual is that of the fits that came to one, a share of the spread of the heights read; past 0.1 a log says nothing.

| Height noise | Track noise | Reaches | Flights | With nothing to fit | Smallest residual | Parameter | Median width / stated, least squares | Identified | Median width / stated, Bayesian | Identified |
|---:|---:|---|---:|---:|---:|---|---:|---|---:|---|
| ±5 m | none | the reader | 200 | 200 | none | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | none | the reader | 200 | 200 | none | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.995 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.995 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.995 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.995 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | 0.995 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±5 m | none | the fit | 200 | 200 | 1.01 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | none | the fit | 200 | 200 | 1.01 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | none | the fit | 200 | 200 | 1.01 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 1.01 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 1.01 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±50 m | none | the fit | 200 | 200 | 0.967 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±50 m | none | the fit | 200 | 200 | 0.967 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±50 m | none | the fit | 200 | 200 | 0.967 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.967 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.967 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.974 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.974 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.974 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.974 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.974 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.985 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.985 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.985 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.985 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.985 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.974 | drag scale | 0.36 / 0.36 | no | 0.36 / 0.36 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.974 | impulse scale | 0.108 / 0.108 | no | 0.108 / 0.108 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.974 | wind toward the east, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.974 | wind toward the north, m/s | 12.7 / 12.7 | no | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.974 | mass on the weighed mass | 0.036 / 0.036 | no | 0.036 / 0.036 | no |
