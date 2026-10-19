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

## Readings within their bounds

Each tuning flight logged by its own logger, then again at every rate and step of the grid, from 500 to half a sample a second and from no step to 100 Pa, and read by `hpr_flightdata::readings` (M7.2b). Each cell gives how many of the flights' readings were given, then the largest share of its bound an error took: 1 is the bound, and `cargo xtask synthetic` fails above it. The JSON names each worst flight, its error and its bound. What is method and what is measured: `docs/physics/log-readings.md`.

| Log | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| imu-500hz | 38, 0.504 | 38, 0.682 | 38, 0.282 | 14, 0.262 | 37, 0.363 | 37, 0.409 | 37, 0.556 |
| 500 Hz, altitude, no step | 171, 0.603 | 171, 0.71 | 171, 0.55 | 54, 0.296 | 170, 0.408 | 170, 0.483 | 170, 0.597 |
| 500 Hz, altitude in 0.3048 m steps | 171, 0.577 | 171, 0.708 | 171, 0.511 | 54, 0.296 | 171, 0.421 | 171, 0.481 | 171, 0.59 |
| 500 Hz, altitude in 10 m steps | 171, 0.597 | 171, 0.655 | 171, 0.19 | 54, 0.359 | 171, 0.432 | 171, 0.479 | 171, 0.466 |
| 500 Hz, pressure alone in 1 Pa steps | 171, 0.594 | 171, 0.709 | 171, 0.524 | 54, 0.295 | 170, 0.439 | 170, 0.499 | 170, 0.594 |
| 500 Hz, pressure alone in 10 Pa steps | 171, 0.554 | 171, 0.708 | 171, 0.352 | 54, 0.296 | 171, 0.474 | 171, 0.51 | 171, 0.568 |
| 500 Hz, pressure alone in 100 Pa steps | 171, 0.505 | 171, 0.648 | 171, 0.511 | 54, 0.309 | 171, 0.544 | 171, 0.5 | 171, 0.484 |
| 100 Hz, altitude, no step | 171, 0.597 | 171, 0.71 | 171, 0.479 | 54, 0.328 | 171, 0.498 | 171, 0.445 | 171, 0.588 |
| 100 Hz, altitude in 0.3048 m steps | 171, 0.576 | 171, 0.707 | 171, 0.557 | 54, 0.319 | 171, 0.385 | 171, 0.44 | 171, 0.581 |
| 100 Hz, altitude in 10 m steps | 171, 0.592 | 171, 0.655 | 171, 0.254 | 54, 0.355 | 171, 0.431 | 171, 0.477 | 171, 0.468 |
| 100 Hz, pressure alone in 1 Pa steps | 171, 0.589 | 171, 0.709 | 171, 0.47 | 54, 0.329 | 171, 0.402 | 171, 0.437 | 171, 0.586 |
| 100 Hz, pressure alone in 10 Pa steps | 171, 0.594 | 171, 0.707 | 171, 0.527 | 54, 0.322 | 171, 0.437 | 171, 0.473 | 171, 0.588 |
| 100 Hz, pressure alone in 100 Pa steps | 171, 0.487 | 171, 0.648 | 171, 0.315 | 53, 0.374 | 171, 0.538 | 171, 0.502 | 171, 0.483 |
| 20 Hz, altitude, no step | 171, 0.552 | 171, 0.717 | 171, 0.446 | 54, 0.352 | 170, 0.386 | 170, 0.392 | 170, 0.586 |
| 20 Hz, altitude in 0.3048 m steps | 171, 0.508 | 171, 0.715 | 171, 0.427 | 54, 0.358 | 170, 0.371 | 170, 0.406 | 170, 0.581 |
| 20 Hz, altitude in 10 m steps | 171, 0.56 | 171, 0.655 | 171, 0.338 | 53, 0.495 | 171, 0.425 | 171, 0.461 | 171, 0.466 |
| 20 Hz, pressure alone in 1 Pa steps | 171, 0.542 | 171, 0.717 | 171, 0.438 | 54, 0.352 | 170, 0.373 | 170, 0.422 | 170, 0.609 |
| 20 Hz, pressure alone in 10 Pa steps | 171, 0.492 | 171, 0.709 | 171, 0.33 | 54, 0.371 | 171, 0.398 | 171, 0.393 | 171, 0.577 |
| 20 Hz, pressure alone in 100 Pa steps | 171, 0.48 | 171, 0.647 | 171, 0.562 | 53, 0.544 | 171, 0.497 | 171, 0.466 | 171, 0.456 |
| 5 Hz, altitude, no step | 171, 0.372 | 171, 0.716 | 171, 0.618 | 0 | 170, 0.434 | 170, 0.306 | 170, 0.6 |
| 5 Hz, altitude in 0.3048 m steps | 171, 0.356 | 171, 0.714 | 171, 0.595 | 0 | 170, 0.431 | 170, 0.292 | 170, 0.594 |
| 5 Hz, altitude in 10 m steps | 171, 0.49 | 171, 0.655 | 171, 0.173 | 0 | 171, 0.433 | 171, 0.375 | 171, 0.461 |
| 5 Hz, pressure alone in 1 Pa steps | 171, 0.367 | 171, 0.715 | 171, 0.608 | 0 | 170, 0.433 | 170, 0.303 | 170, 0.598 |
| 5 Hz, pressure alone in 10 Pa steps | 171, 0.369 | 171, 0.708 | 171, 0.409 | 0 | 171, 0.424 | 171, 0.3 | 171, 0.573 |
| 5 Hz, pressure alone in 100 Pa steps | 171, 0.415 | 171, 0.646 | 171, 0.184 | 0 | 171, 0.572 | 171, 0.491 | 171, 0.489 |
| 1 Hz, altitude, no step | 171, 0.314 | 171, 0.692 | 171, 0.424 | 0 | 170, 0.466 | 170, 0.291 | 170, 0.51 |
| 1 Hz, altitude in 0.3048 m steps | 171, 0.301 | 171, 0.691 | 171, 0.415 | 0 | 171, 0.465 | 171, 0.288 | 171, 0.506 |
| 1 Hz, altitude in 10 m steps | 171, 0.208 | 171, 0.653 | 171, 0.297 | 0 | 171, 0.433 | 171, 0.304 | 171, 0.434 |
| 1 Hz, pressure alone in 1 Pa steps | 171, 0.304 | 171, 0.692 | 171, 0.42 | 0 | 171, 0.466 | 171, 0.29 | 171, 0.509 |
| 1 Hz, pressure alone in 10 Pa steps | 171, 0.291 | 171, 0.686 | 171, 0.388 | 0 | 171, 0.463 | 171, 0.283 | 171, 0.499 |
| 1 Hz, pressure alone in 100 Pa steps | 171, 0.212 | 171, 0.634 | 171, 0.22 | 0 | 171, 0.431 | 171, 0.283 | 171, 0.426 |
| 0.5 Hz, altitude, no step | 171, 0.384 | 171, 0.636 | 171, 0.428 | 0 | 157, 0.478 | 157, 0.357 | 157, 0.472 |
| 0.5 Hz, altitude in 0.3048 m steps | 171, 0.382 | 171, 0.635 | 171, 0.425 | 0 | 159, 0.477 | 159, 0.355 | 159, 0.469 |
| 0.5 Hz, altitude in 10 m steps | 170, 0.278 | 170, 0.594 | 170, 0.305 | 0 | 169, 0.451 | 169, 0.315 | 169, 0.377 |
| 0.5 Hz, pressure alone in 1 Pa steps | 171, 0.384 | 171, 0.636 | 171, 0.428 | 0 | 158, 0.478 | 158, 0.356 | 158, 0.472 |
| 0.5 Hz, pressure alone in 10 Pa steps | 171, 0.375 | 171, 0.631 | 171, 0.407 | 0 | 161, 0.476 | 161, 0.351 | 161, 0.426 |
| 0.5 Hz, pressure alone in 100 Pa steps | 171, 0.275 | 171, 0.593 | 171, 0.272 | 0 | 170, 0.451 | 170, 0.312 | 170, 0.389 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | 171, 0.555 | 171, 0.711 | 171, 0.427 | 54, 0.358 | 171, 0.409 | 171, 0.442 | 171, 0.587 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | 135, 0.381 | 171, 0.695 | 171, 0.427 | 54, 0.358 | 170, 0.272 | 134, 0.332 | 170, 0.556 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | 0 | 171, 0.674 | 171, 0.427 | 54, 0.358 | 170, 0.174 | 0 | 170, 0.516 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | 168, 0.555 | 168, 0.709 | 168, 0.33 | 54, 0.371 | 168, 0.429 | 168, 0.462 | 168, 0.58 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| altos | 33, 0.575 | 33, 0.692 | 33, 0.555 | 9, 0.329 | 33, 0.539 | 33, 0.342 | 33, 0.593 |
| gnss-tracker | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| eggtimer | 39, 0.508 | 39, 0.687 | 39, 0.313 | 14, 0.287 | 39, 0.371 | 39, 0.357 | 39, 0.545 |
| perfectflite | 31, 0.493 | 31, 0.715 | 31, 0.276 | 10, 0.306 | 31, 0.328 | 31, 0.406 | 31, 0.581 |
| rrc3 | 30, 0.418 | 30, 0.642 | 30, 0.344 | 7, 0.194 | 30, 0.302 | 30, 0.343 | 30, 0.546 |

### Readings withheld

| Log | Reading | Reason | Flights |
|---|---|---|---:|
| imu-500hz | top speed | `transonic` | 24 |
| imu-500hz | landing | `ends_before_landing` | 1 |
| imu-500hz | flight time | `ends_before_landing` | 1 |
| imu-500hz | descent rate | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | top speed | `transonic` | 117 |
| 500 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 500 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 500 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, altitude, no step | top speed | `transonic` | 117 |
| 100 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 100 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 118 |
| 20 Hz, altitude, no step | top speed | `transonic` | 117 |
| 20 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 10 m steps | top speed | `transonic` | 118 |
| 20 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 20 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 20 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 118 |
| 5 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 1 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude, no step | landing | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | flight time | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | descent rate | `ends_before_landing` | 14 |
| 0.5 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 10 m steps | liftoff | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | apogee height | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | apogee time | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 170 |
| 0.5 Hz, altitude in 10 m steps | top speed | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | landing | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | landing | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | flight time | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | flight time | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | descent rate | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | descent rate | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | landing | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | flight time | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | descent rate | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 100 Pa steps | landing | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | flight time | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | liftoff | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | flight time | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | liftoff | `short_pad` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | flight time | `short_pad` | 170 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | descent rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | liftoff | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | apogee height | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | apogee time | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | top speed | `transonic` | 114 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | top speed | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | landing | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | flight time | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | descent rate | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | liftoff | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | apogee height | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | apogee time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | top speed | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | landing | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | flight time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | descent rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | liftoff | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | apogee height | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | apogee time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | top speed | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | landing | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | flight time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | descent rate | `short_pad` | 171 |
| altos | top speed | `transonic` | 24 |
| gnss-tracker | liftoff | `receiver_dropout` | 29 |
| gnss-tracker | apogee height | `receiver_lag` | 29 |
| gnss-tracker | apogee time | `receiver_lag` | 29 |
| gnss-tracker | top speed | `receiver_lag` | 29 |
| gnss-tracker | landing | `needs` | 29 |
| gnss-tracker | flight time | `needs` | 29 |
| gnss-tracker | descent rate | `needs` | 29 |
| eggtimer | top speed | `transonic` | 25 |
| perfectflite | top speed | `transonic` | 21 |
| rrc3 | top speed | `transonic` | 23 |
