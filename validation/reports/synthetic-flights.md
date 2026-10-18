# Synthetic flights

The census of the synthetic flights (M7.2a, ADR-280): each set's flights by design and by logger, the ranges they span, and each draw that didn't fly. Written by `cargo xtask synthetic`; `cargo xtask synthetic --check` holds it on every platform. Values are to three significant figures. What the flights are and how far to trust them: `docs/physics/synthetic-flights.md`.

## The sets

|  | Tuning | A | B |
|---|---:|---:|---:|
| Seed | 2026101700 | 2026101701 | 2026101702 |
| Drawn | 200 | 1000 | 1000 |
| Flown | 200 | 1000 | 1000 |
| Of set B's model | 100 | 0 | 1000 |
| Not flown | 0 | 0 | 0 |

## Flights flown, by design

| Design | Tuning | A | B |
|---|---:|---:|---:|
| rocketpy-bella-lui | 20 | 96 | 98 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 23 | 104 | 107 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 22 | 97 | 121 |
| rocketpy-cavour | 20 | 92 | 97 |
| rocketpy-genesis | 15 | 109 | 99 |
| rocketpy-juno-iii | 19 | 95 | 97 |
| rocketpy-lince | 28 | 87 | 103 |
| rocketpy-ndrt-2020-nose-to-tail | 20 | 113 | 84 |
| rocketpy-prometheus-2022-generic-motor | 15 | 101 | 98 |
| rocketpy-valetudo | 18 | 106 | 96 |

## Flights flown, by logger

| Logger | Tuning | A | B |
|---|---:|---:|---:|
| perfectflite | 31 | 144 | 132 |
| rrc3 | 30 | 139 | 176 |
| eggtimer | 39 | 181 | 169 |
| altos | 33 | 185 | 186 |
| imu-500hz | 38 | 171 | 182 |
| gnss-tracker | 29 | 180 | 155 |

## What the flights span

Least / median / greatest.

| Quantity | Tuning | A | B |
|---|---:|---:|---:|
| Apogee above the pad, m | 472 / 2820 / 4420 | 411 / 2780 / 4740 | 438 / 2780 / 4810 |
| Top Mach number | 0.28 / 0.751 / 1.12 | 0.269 / 0.737 / 1.18 | 0.269 / 0.746 / 1.15 |
| Top vertical speed, m/s | 90.4 / 246 / 367 | 83.1 / 244 / 375 | 84.5 / 246 / 375 |
| Liftoff to landing, s | 36.2 / 170 / 611 | 32.1 / 160 / 617 | 33.3 / 171 / 603 |
| Wind, m/s | 0.00513 / 4.61 / 8.89 | 0.0288 / 4.51 / 8.94 | 0.00837 / 4.64 / 8.91 |
| Rail from vertical, ° | 0.142 / 5.2 / 9.94 | 0.0011 / 4.84 / 10 | 0.0131 / 5.19 / 9.99 |
| Site elevation, m | 10.9 / 802 / 1500 | 0.169 / 752 / 1500 | 4.19 / 749 / 1500 |
| Air's offset from standard, K | -14.6 / -0.585 / 14.9 | -15 / -0.012 / 15 | -15 / 0.0703 / 15 |

## Draws not flown

Every draw flew.
