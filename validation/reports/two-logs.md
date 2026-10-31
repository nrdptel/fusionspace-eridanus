# Two logs of one flight: the clock offset

Written by `cargo xtask two-logs`; don't edit it by hand. How the offset is read: [reading a flight log](../../docs/reading-a-flight-log.md). What it checks: each flight carries two simulated loggers, each with a clock of its own, and the offset their two liftoffs give is held to the true one. It says how well a shared liftoff recovers a clock offset on HPR Sim's flights, not on a real airframe.

The offset's 90% interval takes the shares -0.0526 and 0.0515 of its bound, fitted on 1428 offsets of the two-logger calibration set.

| Set | Flights | Offsets given | Within the bound | Largest share of the bound | Median error, s | 90% of errors under, s | Largest error, s | Median bound, s |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| pair-a | 1000 | 703 | 703 | 0.126 | 0.0332 | 0.0838 | 0.205 | 1.57 |
| pair-b | 1000 | 680 | 680 | 0.123 | 0.0311 | 0.0834 | 0.189 | 1.56 |
| staged | 200 | 135 | 135 | 0.0492 | 0.0237 | 0.0464 | 0.0824 | 1.53 |

## The 90% interval

ADR-257's test: per tail a 99.9% Clopper–Pearson band holding 5%, the count inside holding 90%; the probes must fail it.

| Set | Below | Inside | Above | Passes | narrowed to 0.8 | widened to 1.25 | shifted down one standard error |
|---|--:|--:|--:|---|---|---|---|
| pair-a | 43 | 622 | 38 | passes | fails | fails | fails |
| pair-b | 43 | 612 | 25 | passes | fails | fails | fails |

## Offsets withheld

A log with no liftoff sits on no timeline.

| Set | Reason | Flights |
|---|---|--:|
| pair-calibration | ReceiverDropout | 572 |
| pair-a | ReceiverDropout | 297 |
| pair-b | ReceiverDropout | 320 |
| staged | ReceiverDropout | 65 |

## By logger pair

| Set | Loggers | Offsets | Largest share of the bound |
|---|---|--:|--:|
| pair-a | altos + altos | 33 | 0.0625 |
| pair-a | altos + eggtimer | 71 | 0.075 |
| pair-a | altos + imu-500hz | 49 | 0.0865 |
| pair-a | altos + perfectflite | 45 | 0.126 |
| pair-a | altos + rrc3 | 59 | 0.0869 |
| pair-a | eggtimer + eggtimer | 27 | 0.0651 |
| pair-a | eggtimer + imu-500hz | 44 | 0.0643 |
| pair-a | eggtimer + perfectflite | 61 | 0.125 |
| pair-a | eggtimer + rrc3 | 63 | 0.119 |
| pair-a | imu-500hz + imu-500hz | 29 | 0.056 |
| pair-a | imu-500hz + perfectflite | 57 | 0.0954 |
| pair-a | imu-500hz + rrc3 | 59 | 0.077 |
| pair-a | perfectflite + perfectflite | 26 | 0.0622 |
| pair-a | perfectflite + rrc3 | 57 | 0.066 |
| pair-a | rrc3 + rrc3 | 23 | 0.0738 |
| pair-b | altos + altos | 24 | 0.0838 |
| pair-b | altos + eggtimer | 55 | 0.123 |
| pair-b | altos + imu-500hz | 59 | 0.0895 |
| pair-b | altos + perfectflite | 48 | 0.085 |
| pair-b | altos + rrc3 | 52 | 0.0827 |
| pair-b | eggtimer + eggtimer | 35 | 0.0692 |
| pair-b | eggtimer + imu-500hz | 37 | 0.0567 |
| pair-b | eggtimer + perfectflite | 59 | 0.0858 |
| pair-b | eggtimer + rrc3 | 52 | 0.0707 |
| pair-b | imu-500hz + imu-500hz | 28 | 0.0542 |
| pair-b | imu-500hz + perfectflite | 65 | 0.0885 |
| pair-b | imu-500hz + rrc3 | 54 | 0.0768 |
| pair-b | perfectflite + perfectflite | 27 | 0.075 |
| pair-b | perfectflite + rrc3 | 58 | 0.103 |
| pair-b | rrc3 + rrc3 | 27 | 0.0597 |
| staged | altos + altos | 5 | 0.0492 |
| staged | altos + eggtimer | 10 | 0.0464 |
| staged | altos + imu-500hz | 14 | 0.0476 |
| staged | altos + perfectflite | 14 | 0.031 |
| staged | altos + rrc3 | 9 | 0.0285 |
| staged | eggtimer + eggtimer | 9 | 0.0214 |
| staged | eggtimer + imu-500hz | 6 | 0.0274 |
| staged | eggtimer + perfectflite | 13 | 0.0448 |
| staged | eggtimer + rrc3 | 12 | 0.0481 |
| staged | imu-500hz + imu-500hz | 7 | 0.0366 |
| staged | imu-500hz + perfectflite | 12 | 0.0311 |
| staged | imu-500hz + rrc3 | 4 | 0.0223 |
| staged | perfectflite + perfectflite | 5 | 0.03 |
| staged | perfectflite + rrc3 | 6 | 0.017 |
| staged | rrc3 + rrc3 | 9 | 0.0179 |
