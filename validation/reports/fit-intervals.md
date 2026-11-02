# The least-squares fit's 90% intervals

On sets A and B of the synthetic flights, each parameter's 90% interval is held to three counts, each with a two-sided 99.9% Clopper–Pearson band: the two tails' bands must hold 5% and the inside's 90%. Shares fitted on the calibration set.

Written by `cargo xtask fit`. How to read it: [the fit's page](../../docs/physics/flight-fit.md).

## Set A: 1000 flights, 180 with nothing to fit

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 45 | 897 | 58 | passes | fails, fails, fails | -0.00345 | 0.112 / 0.36 | yes |
| impulse scale | 35 | 907 | 58 | passes | fails, fails, fails | -0.00236 | 0.0425 / 0.108 | yes |
| wind toward the east, m/s | 77 | 1351 | 69 | passes | fails, fails, fails | -0.00752 | 0.432 / 12.7 | yes |
| wind toward the north, m/s | 85 | 1336 | 76 | passes | fails, fails, fails | 0.00431 | 0.446 / 12.7 | yes |
| mass on the weighed mass | 36 | 912 | 52 | passes | fails, fails, fails | -0.000223 | 0.0353 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 1 / 90 / 5 | 4 / 82 / 10 | 7 / 131 / 8 | 10 / 129 / 7 | 4 / 89 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 5 / 96 / 3 | 3 / 94 / 7 | 5 / 143 / 6 | 5 / 146 / 3 | 2 / 93 / 9 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 3 / 90 / 4 | 4 / 87 / 6 | 7 / 132 / 2 | 6 / 129 / 6 | 3 / 88 / 6 |
| rocketpy-cavour | 8 / 81 / 3 | 5 / 84 / 3 | 12 / 151 / 8 | 5 / 146 / 20 | 5 / 82 / 5 |
| rocketpy-genesis | 9 / 91 / 9 | 5 / 97 / 7 | 11 / 137 / 6 | 13 / 132 / 9 | 4 / 96 / 9 |
| rocketpy-juno-iii | 5 / 81 / 9 | 3 / 87 / 5 | 7 / 133 / 8 | 7 / 136 / 5 | 4 / 89 / 2 |
| rocketpy-lince | 6 / 76 / 5 | 3 / 80 / 4 | 7 / 129 / 14 | 13 / 126 / 11 | 6 / 77 / 4 |
| rocketpy-ndrt-2020-nose-to-tail | 2 / 106 / 5 | 1 / 108 / 4 | 2 / 143 / 1 | 4 / 141 / 1 | 3 / 107 / 3 |
| rocketpy-prometheus-2022-generic-motor | 5 / 87 / 9 | 5 / 87 / 9 | 14 / 128 / 15 | 19 / 128 / 10 | 2 / 92 / 7 |
| rocketpy-valetudo | 1 / 99 / 6 | 2 / 101 / 3 | 5 / 124 / 1 | 3 / 123 / 4 | 3 / 99 / 4 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 5 / 497 / 11 | 6 / 478 / 29 | 37 / 1083 / 19 | 43 / 1066 / 30 | 16 / 479 / 18 |
| heights fit to 1 m or worse | 34 / 237 / 36 | 17 / 267 / 23 | 40 / 268 / 50 | 42 / 270 / 46 | 14 / 270 / 23 |

## Set B: 1000 flights, 155 with nothing to fit

| Parameter | Below | Inside | Above | Test | Narrowed, widened, shifted | Mean error | Median width / stated | Identified |
|---|---:|---:|---:|---|---|---:|---:|---|
| drag scale | 71 | 893 | 36 | passes | fails, fails, fails | -0.000974 | 0.111 / 0.36 | yes |
| impulse scale | 58 | 913 | 29 | passes | fails, fails, fails | -0.000626 | 0.0429 / 0.108 | yes |
| wind toward the east, m/s | 81 | 1347 | 69 | passes | fails, fails, fails | -0.00902 | 0.475 / 12.7 | yes |
| wind toward the north, m/s | 78 | 1352 | 67 | passes | fails, fails, fails | -0.00913 | 0.508 / 12.7 | yes |
| mass on the weighed mass | 43 | 906 | 51 | passes | fails, fails, fails | 0.000435 | 0.0353 / 0.036 | no |

By design (below / inside / above):

| Design | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 1 / 95 / 2 | 0 / 98 / 0 | 4 / 136 / 4 | 3 / 138 / 3 | 8 / 87 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 8 / 98 / 1 | 8 / 97 / 2 | 6 / 163 / 3 | 3 / 165 / 4 | 7 / 92 / 8 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 8 / 104 / 9 | 3 / 116 / 2 | 5 / 147 / 4 | 2 / 152 / 2 | 3 / 109 / 9 |
| rocketpy-cavour | 3 / 89 / 5 | 6 / 84 / 7 | 15 / 109 / 10 | 13 / 109 / 12 | 3 / 87 / 7 |
| rocketpy-genesis | 3 / 95 / 1 | 5 / 92 / 2 | 6 / 125 / 2 | 6 / 125 / 2 | 0 / 95 / 4 |
| rocketpy-juno-iii | 11 / 82 / 4 | 5 / 90 / 2 | 9 / 140 / 7 | 8 / 142 / 6 | 4 / 88 / 5 |
| rocketpy-lince | 5 / 93 / 5 | 9 / 90 / 4 | 12 / 123 / 13 | 17 / 123 / 8 | 6 / 93 / 4 |
| rocketpy-ndrt-2020-nose-to-tail | 6 / 76 / 2 | 4 / 79 / 1 | 1 / 132 / 1 | 0 / 133 / 1 | 4 / 76 / 4 |
| rocketpy-prometheus-2022-generic-motor | 23 / 69 / 6 | 17 / 74 / 7 | 20 / 120 / 21 | 22 / 112 / 27 | 4 / 92 / 2 |
| rocketpy-valetudo | 3 / 92 / 1 | 1 / 93 / 2 | 3 / 152 / 4 | 4 / 153 / 2 | 4 / 87 / 5 |

By how well the flight's heights fit (below / inside / above):

| Residual | drag scale | impulse scale | wind toward the east, m/s | wind toward the north, m/s | mass on the weighed mass |
|---|---:|---:|---:|---:|---:|
| heights fit within 1 m | 7 / 452 / 4 | 13 / 448 / 2 | 26 / 913 / 22 | 30 / 912 / 19 | 14 / 422 / 27 |
| heights fit to 1 m or worse | 52 / 309 / 21 | 37 / 333 / 12 | 55 / 434 / 47 | 48 / 440 / 48 | 22 / 342 / 18 |

## Pure noise in place of the log

| Noise | Reaches | Flights | With nothing to fit | Parameter | Median width / stated | Identified |
|---:|---|---:|---:|---|---:|---|
| ±5 m | the reader | 200 | 200 | drag scale | 0.36 / 0.36 | no |
| ±5 m | the reader | 200 | 200 | impulse scale | 0.108 / 0.108 | no |
| ±5 m | the reader | 200 | 200 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±5 m | the reader | 200 | 200 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±5 m | the reader | 200 | 200 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±1000 m | the reader | 200 | 200 | drag scale | 0.36 / 0.36 | no |
| ±1000 m | the reader | 200 | 200 | impulse scale | 0.108 / 0.108 | no |
| ±1000 m | the reader | 200 | 200 | wind toward the east, m/s | 12.7 / 12.7 | no |
| ±1000 m | the reader | 200 | 200 | wind toward the north, m/s | 12.7 / 12.7 | no |
| ±1000 m | the reader | 200 | 200 | mass on the weighed mass | 0.036 / 0.036 | no |
| ±1000 m | the fit | 200 | 0 | drag scale | 0.399 / 0.36 | no |
| ±1000 m | the fit | 200 | 0 | impulse scale | 0.111 / 0.108 | no |
| ±1000 m | the fit | 200 | 0 | wind toward the east, m/s | 10.8 / 12.7 | no |
| ±1000 m | the fit | 200 | 0 | wind toward the north, m/s | 10.6 / 12.7 | no |
| ±1000 m | the fit | 200 | 0 | mass on the weighed mass | 0.0368 / 0.036 | no |
