# The least-squares fit's 90% intervals

On sets A and B of the synthetic flights, each parameter's 90% interval is held to three counts, each with a two-sided 99.9% Clopper–Pearson band: the two tails' bands must hold 5% and the inside's 90%. Shares fitted on the calibration set.

Written by `cargo xtask fit`. How to read it: [the fit's page](../../docs/physics/flight-fit.md).

## Set A: 1000 flights counted

Of its first 1216, 216 had nothing to fit and aren't counted, 216 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 2 reached the fit's step limit. The largest residual of any of them is 0.0209 of the spread of the heights read; past 0.1 a fit says nothing.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 34 | 905 | 61 | passes | fails, fails, fails | -0.00169 | 0.115 / 0.36 | yes |
| impulse scale | 35 | 908 | 57 | passes | fails, fails, fails | 0.000363 | 0.0399 / 0.108 | yes |
| wind toward the east, m/s | 89 | 1343 | 65 | passes | fails, fails, fails | -0.00705 | 0.345 / 12.7 | yes |
| wind toward the north, m/s | 67 | 1366 | 64 | passes | fails, fails, fails | 0.00437 | 0.374 / 12.7 | yes |
| mass on the weighed mass | 40 | 911 | 49 | passes | fails, fails, fails | -0.000394 | 0.035 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 4 / 76 / 12 | 1 / 87 / 4 | 10 / 125 / 11 | 8 / 131 / 7 | 5 / 84 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 87 / 4 | 3 / 87 / 6 | 7 / 140 / 7 | 5 / 147 / 2 | 3 / 86 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 3 / 96 / 3 | 4 / 91 / 7 | 9 / 131 / 1 | 6 / 129 / 6 | 2 / 96 / 4 |
| rocketpy-cavour | 5 / 95 / 0 | 6 / 91 / 3 | 18 / 146 / 7 | 7 / 147 / 17 | 9 / 86 / 5 |
| rocketpy-genesis | 6 / 95 / 6 | 4 / 93 / 10 | 12 / 140 / 2 | 7 / 141 / 6 | 4 / 94 / 9 |
| rocketpy-juno-iii | 3 / 82 / 8 | 1 / 91 / 1 | 5 / 134 / 9 | 6 / 137 / 5 | 4 / 85 / 4 |
| rocketpy-lince | 1 / 84 / 7 | 4 / 79 / 9 | 9 / 129 / 12 | 13 / 128 / 9 | 5 / 80 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 1 / 118 / 4 | 6 / 110 / 7 | 2 / 142 / 2 | 3 / 143 / 0 | 3 / 117 / 3 |
| rocketpy-prometheus-2022-generic-motor | 4 / 82 / 10 | 5 / 85 / 6 | 12 / 132 / 13 | 10 / 138 / 9 | 2 / 88 / 6 |
| rocketpy-valetudo | 2 / 90 / 7 | 1 / 94 / 4 | 5 / 124 / 1 | 2 / 125 / 3 | 3 / 95 / 1 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 20 / 600 / 23 | 13 / 595 / 35 | 56 / 1097 / 29 | 44 / 1105 / 33 | 24 / 594 / 25 |
| heights fit to 1 m or worse | 14 / 305 / 38 | 22 / 313 / 22 | 33 / 246 / 36 | 23 / 261 / 31 | 16 / 317 / 24 |

## Set B: 1000 flights counted

Of its first 1193, 193 had nothing to fit and aren't counted, 193 of them logs whose only height is a GNSS receiver's, from which the reader takes no apogee. A wind is counted on the 1497 of 1500 flights of the logger with a track, some past these, fitted with a track read. Of the flights counted here or for the wind, 7 reached the fit's step limit. The largest residual of any of them is 0.0251 of the spread of the heights read; past 0.1 a fit says nothing.

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 70 | 901 | 29 | passes | fails, fails, fails | 0.00523 | 0.123 / 0.36 | yes |
| impulse scale | 54 | 910 | 36 | passes | fails, fails, fails | 0.00164 | 0.0408 / 0.108 | yes |
| wind toward the east, m/s | 75 | 1349 | 73 | passes | fails, fails, fails | -0.021 | 0.407 / 12.7 | yes |
| wind toward the north, m/s | 71 | 1362 | 64 | passes | fails, fails, fails | -0.00533 | 0.449 / 12.7 | yes |
| mass on the weighed mass | 42 | 900 | 58 | passes | fails, fails, fails | -0.000145 | 0.035 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 0 / 97 / 0 | 0 / 96 / 1 | 6 / 131 / 7 | 4 / 134 / 6 | 5 / 89 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 96 / 0 | 7 / 90 / 4 | 7 / 160 / 5 | 3 / 166 / 3 | 7 / 86 / 8 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 10 / 102 / 5 | 2 / 108 / 7 | 3 / 149 / 4 | 3 / 151 / 2 | 4 / 102 / 11 |
| rocketpy-cavour | 7 / 84 / 2 | 5 / 87 / 1 | 11 / 112 / 11 | 12 / 111 / 11 | 2 / 85 / 6 |
| rocketpy-genesis | 13 / 92 / 2 | 5 / 98 / 4 | 6 / 120 / 7 | 8 / 122 / 3 | 0 / 103 / 4 |
| rocketpy-juno-iii | 5 / 93 / 5 | 3 / 96 / 4 | 9 / 142 / 5 | 10 / 141 / 5 | 6 / 91 / 6 |
| rocketpy-lince | 8 / 83 / 4 | 14 / 77 / 4 | 11 / 124 / 13 | 14 / 122 / 12 | 6 / 82 / 7 |
| rocketpy-ndrt-2020-nose-to-tail | 3 / 79 / 1 | 1 / 80 / 2 | 2 / 132 / 0 | 1 / 133 / 0 | 3 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 19 / 68 / 9 | 17 / 75 / 4 | 17 / 129 / 15 | 13 / 127 / 21 | 5 / 89 / 2 |
| rocketpy-valetudo | 0 / 107 / 1 | 0 / 103 / 5 | 3 / 150 / 6 | 3 / 155 / 1 | 4 / 97 / 7 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 21 / 525 / 1 | 15 / 517 / 15 | 29 / 918 / 34 | 32 / 925 / 24 | 20 / 494 / 33 |
| heights fit to 1 m or worse | 49 / 376 / 28 | 39 / 393 / 21 | 46 / 431 / 39 | 39 / 437 / 40 | 22 / 406 / 25 |

## Pure noise in place of the log

The smallest residual is that of the fits that came to one, a share of the spread of the heights read; past 0.1 a log says nothing.

| Height noise | Track noise | Reaches | Flights | With nothing to fit | Smallest residual | Parameter | Median width / stated | Identified |
|---:|---:|---|---:|---:|---:|---|---:|---|
| ±5 m | none | the reader | 200 | 200 | none | drag scale | 0.36 / 0.36 | no |
| ±5 m | none | the reader | 200 | 200 | none | impulse scale | 0.108 / 0.108 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±5 m | none | the reader | 200 | 200 | none | mass on the weighed mass | 0.036 / 0.036 | no |
| ±1000 m | none | the reader | 200 | 200 | none | drag scale | 0.36 / 0.36 | no |
| ±1000 m | none | the reader | 200 | 200 | none | impulse scale | 0.108 / 0.108 | no |
| ±1000 m | none | the reader | 200 | 200 | none | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | none | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±1000 m | none | the reader | 200 | 200 | none | mass on the weighed mass | 0.036 / 0.036 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | drag scale | 0.36 / 0.36 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | impulse scale | 0.108 / 0.108 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±5 m | none | the fit | 200 | 200 | 0.958 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | drag scale | 0.36 / 0.36 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | impulse scale | 0.108 / 0.108 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±50 m | none | the fit | 200 | 200 | 0.986 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | drag scale | 0.36 / 0.36 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | impulse scale | 0.108 / 0.108 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±1000 m | none | the fit | 200 | 200 | 0.984 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.981 | drag scale | 0.36 / 0.36 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.981 | impulse scale | 0.108 / 0.108 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.981 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.981 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±5 m | ±5 m | the fit | 200 | 200 | 0.981 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.971 | drag scale | 0.36 / 0.36 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.971 | impulse scale | 0.108 / 0.108 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.971 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.971 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±1000 m | ±5 m | the fit | 200 | 200 | 0.971 | mass on the weighed mass | 0.036 / 0.036 | no |
