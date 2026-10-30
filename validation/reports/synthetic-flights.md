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

Each tuning flight logged by its own logger, then again at every rate and step of the grid, from 500 to half a sample a second and from no step to 100 Pa, with its own logger's accelerometer at 500, 100 and 20 Hz and, where its part can be, set to ±8 g, so that some tops clip at the part's range and the rest don't, and cropped as loggers start their logs, and read by `hpr_flightdata::readings` (M7.2b, M7.2c). Each cell gives how many of the flights' readings were given, then the largest share of its bound an error took: 1 is the bound, and `cargo xtask synthetic` fails above it. The JSON names each worst flight, its error and its bound. What is method and what is measured: `docs/physics/log-readings.md`.

### The height track's readings

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
| its accelerometer at 500 Hz | 71, 0.575 | 71, 0.692 | 71, 0.555 | 23, 0.329 | 70, 0.539 | 70, 0.409 | 70, 0.593 |
| its accelerometer at 100 Hz | 71, 0.575 | 71, 0.692 | 71, 0.555 | 23, 0.329 | 70, 0.539 | 70, 0.409 | 70, 0.593 |
| its accelerometer at 20 Hz | 71, 0.575 | 71, 0.692 | 71, 0.555 | 23, 0.329 | 70, 0.539 | 70, 0.409 | 70, 0.593 |
| its accelerometer set to ±8 g | 38, 0.504 | 38, 0.682 | 38, 0.282 | 14, 0.262 | 37, 0.363 | 37, 0.409 | 37, 0.556 |
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

### The accelerometer's readings and the canopies'

| Log | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|
| imu-500hz | 38, 0.282 | 38, 0.0861 | 16, 0.655 | 16, 0.582 | 37, 0.621 |
| 500 Hz, altitude, no step | 0 | 54, 0.0424 | 87, 0.684 | 87, 0.628 | 170, 0.7 |
| 500 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0424 | 87, 0.681 | 87, 0.63 | 171, 0.699 |
| 500 Hz, altitude in 10 m steps | 0 | 54, 0.0566 | 87, 0.645 | 87, 0.57 | 171, 0.646 |
| 500 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0424 | 87, 0.684 | 87, 0.628 | 170, 0.699 |
| 500 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.044 | 87, 0.675 | 87, 0.631 | 171, 0.69 |
| 500 Hz, pressure alone in 100 Pa steps | 0 | 54, 0.0424 | 87, 0.607 | 87, 0.56 | 171, 0.643 |
| 100 Hz, altitude, no step | 0 | 54, 0.0115 | 87, 0.688 | 87, 0.626 | 170, 0.695 |
| 100 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0115 | 87, 0.685 | 87, 0.625 | 171, 0.694 |
| 100 Hz, altitude in 10 m steps | 0 | 54, 0.035 | 87, 0.645 | 87, 0.562 | 171, 0.611 |
| 100 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0115 | 87, 0.687 | 87, 0.639 | 171, 0.694 |
| 100 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.0115 | 87, 0.68 | 87, 0.626 | 171, 0.69 |
| 100 Hz, pressure alone in 100 Pa steps | 0 | 53, 0.125 | 87, 0.606 | 87, 0.568 | 171, 0.643 |
| 20 Hz, altitude, no step | 0 | 54, 0.0677 | 87, 0.692 | 87, 0.619 | 170, 0.696 |
| 20 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0677 | 87, 0.689 | 87, 0.624 | 170, 0.693 |
| 20 Hz, altitude in 10 m steps | 0 | 53, 0.228 | 87, 0.615 | 87, 0.56 | 171, 0.611 |
| 20 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0677 | 87, 0.692 | 87, 0.593 | 170, 0.694 |
| 20 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.0677 | 87, 0.687 | 87, 0.642 | 171, 0.691 |
| 20 Hz, pressure alone in 100 Pa steps | 0 | 53, 0.544 | 87, 0.609 | 87, 0.56 | 171, 0.643 |
| 5 Hz, altitude, no step | 0 | 0 | 87, 0.691 | 87, 0.563 | 170, 0.7 |
| 5 Hz, altitude in 0.3048 m steps | 0 | 0 | 87, 0.687 | 87, 0.563 | 170, 0.698 |
| 5 Hz, altitude in 10 m steps | 0 | 0 | 87, 0.625 | 87, 0.555 | 171, 0.652 |
| 5 Hz, pressure alone in 1 Pa steps | 0 | 0 | 87, 0.69 | 87, 0.563 | 170, 0.699 |
| 5 Hz, pressure alone in 10 Pa steps | 0 | 0 | 87, 0.682 | 87, 0.574 | 171, 0.696 |
| 5 Hz, pressure alone in 100 Pa steps | 0 | 0 | 87, 0.608 | 87, 0.558 | 171, 0.642 |
| 1 Hz, altitude, no step | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, altitude in 0.3048 m steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, altitude in 10 m steps | 0 | 0 | 75, 0.565 | 75, 0.55 | 95, 0.572 |
| 1 Hz, pressure alone in 1 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, pressure alone in 10 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, pressure alone in 100 Pa steps | 0 | 0 | 75, 0.617 | 75, 0.534 | 95, 0.555 |
| 0.5 Hz, altitude, no step | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, altitude in 0.3048 m steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, altitude in 10 m steps | 0 | 0 | 47, 0.53 | 47, 0.586 | 47, 0.528 |
| 0.5 Hz, pressure alone in 1 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, pressure alone in 10 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, pressure alone in 100 Pa steps | 0 | 0 | 0 | 0 | 0 |
| its accelerometer at 500 Hz | 71, 0.576 | 71, 0.0861 | 36, 0.656 | 36, 0.613 | 70, 0.679 |
| its accelerometer at 100 Hz | 71, 0.686 | 70, 0.172 | 36, 0.656 | 36, 0.613 | 70, 0.679 |
| its accelerometer at 20 Hz | 71, 0.715 | 71, 0.262 | 36, 0.656 | 36, 0.613 | 70, 0.679 |
| its accelerometer set to ±8 g | 38, 0.283 | 38, 0.186 | 16, 0.655 | 16, 0.582 | 37, 0.621 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | 0 | 54, 0.0677 | 87, 0.689 | 87, 0.624 | 171, 0.7 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | 0 | 54, 0.0677 | 87, 0.689 | 87, 0.624 | 170, 0.693 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | 0 | 54, 0.0677 | 87, 0.689 | 87, 0.624 | 170, 0.693 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | 0 | 54, 0.0677 | 86, 0.687 | 86, 0.642 | 168, 0.691 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | 0 | 0 | 0 | 0 | 0 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | 0 | 0 | 0 | 0 | 0 |
| altos | 33, 0.686 | 32, 0.0669 | 20, 0.656 | 20, 0.613 | 33, 0.679 |
| gnss-tracker | 0 | 0 | 0 | 0 | 0 |
| eggtimer | 0 | 14, 0 | 23, 0.64 | 23, 0.593 | 39, 0.679 |
| perfectflite | 0 | 10, 0 | 11, 0.689 | 11, 0.519 | 31, 0.693 |
| rrc3 | 0 | 7, 0 | 17, 0.599 | 17, 0.539 | 30, 0.625 |

### Readings withheld

Each reading withheld, by reason; beside them, a drogue or a main's opening that the truth has none of, on a flight that came down under one canopy or none, as none to read.

| Log | Reading | Reason | Flights |
|---|---|---|---:|
| imu-500hz | top speed | `transonic` | 24 |
| imu-500hz | landing | `ends_before_landing` | 1 |
| imu-500hz | flight time | `ends_before_landing` | 1 |
| imu-500hz | descent rate | `ends_before_landing` | 1 |
| imu-500hz | drogue rate | `ends_before_landing` | 1 |
| imu-500hz | drogue rate | none to read | 21 |
| imu-500hz | main opening | `ends_before_landing` | 1 |
| imu-500hz | main opening | none to read | 21 |
| imu-500hz | main rate | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | top speed | `transonic` | 117 |
| 500 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, altitude, no step | burnout | `transonic` | 117 |
| 500 Hz, altitude, no step | drogue rate | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | drogue rate | none to read | 83 |
| 500 Hz, altitude, no step | main opening | `ends_before_landing` | 1 |
| 500 Hz, altitude, no step | main opening | none to read | 83 |
| 500 Hz, altitude, no step | main rate | `ends_before_landing` | 1 |
| 500 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 500 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, altitude in 0.3048 m steps | burnout | `transonic` | 117 |
| 500 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 84 |
| 500 Hz, altitude in 0.3048 m steps | main opening | none to read | 84 |
| 500 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 500 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, altitude in 10 m steps | burnout | `transonic` | 117 |
| 500 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 500 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 500 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | drogue rate | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 500 Hz, pressure alone in 1 Pa steps | main opening | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 500 Hz, pressure alone in 1 Pa steps | main rate | `ends_before_landing` | 1 |
| 500 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, pressure alone in 10 Pa steps | burnout | `transonic` | 117 |
| 500 Hz, pressure alone in 10 Pa steps | drogue rate | none to read | 84 |
| 500 Hz, pressure alone in 10 Pa steps | main opening | none to read | 84 |
| 500 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, pressure alone in 100 Pa steps | burnout | `transonic` | 117 |
| 500 Hz, pressure alone in 100 Pa steps | drogue rate | none to read | 84 |
| 500 Hz, pressure alone in 100 Pa steps | main opening | none to read | 84 |
| 100 Hz, altitude, no step | top speed | `transonic` | 117 |
| 100 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, altitude, no step | burnout | `transonic` | 117 |
| 100 Hz, altitude, no step | drogue rate | `short_leg` | 1 |
| 100 Hz, altitude, no step | drogue rate | none to read | 83 |
| 100 Hz, altitude, no step | main opening | `short_leg` | 1 |
| 100 Hz, altitude, no step | main opening | none to read | 83 |
| 100 Hz, altitude, no step | main rate | `short_leg` | 1 |
| 100 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 100 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, altitude in 0.3048 m steps | burnout | `transonic` | 117 |
| 100 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 84 |
| 100 Hz, altitude in 0.3048 m steps | main opening | none to read | 84 |
| 100 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 100 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, altitude in 10 m steps | burnout | `transonic` | 117 |
| 100 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 100 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 100 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 100 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 84 |
| 100 Hz, pressure alone in 1 Pa steps | main opening | none to read | 84 |
| 100 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, pressure alone in 10 Pa steps | burnout | `transonic` | 117 |
| 100 Hz, pressure alone in 10 Pa steps | drogue rate | none to read | 84 |
| 100 Hz, pressure alone in 10 Pa steps | main opening | none to read | 84 |
| 100 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 118 |
| 100 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, pressure alone in 100 Pa steps | burnout | `transonic` | 118 |
| 100 Hz, pressure alone in 100 Pa steps | drogue rate | none to read | 84 |
| 100 Hz, pressure alone in 100 Pa steps | main opening | none to read | 84 |
| 20 Hz, altitude, no step | top speed | `transonic` | 117 |
| 20 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude, no step | burnout | `transonic` | 117 |
| 20 Hz, altitude, no step | drogue rate | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | drogue rate | none to read | 83 |
| 20 Hz, altitude, no step | main opening | `ends_before_landing` | 1 |
| 20 Hz, altitude, no step | main opening | none to read | 83 |
| 20 Hz, altitude, no step | main rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps | drogue rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps | main opening | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps | main rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 10 m steps | top speed | `transonic` | 118 |
| 20 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 10 m steps | burnout | `transonic` | 118 |
| 20 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 20 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 20 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 20 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 20 Hz, pressure alone in 1 Pa steps | drogue rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 20 Hz, pressure alone in 1 Pa steps | main opening | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 20 Hz, pressure alone in 1 Pa steps | main rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 10 Pa steps | top speed | `transonic` | 117 |
| 20 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 10 Pa steps | burnout | `transonic` | 117 |
| 20 Hz, pressure alone in 10 Pa steps | drogue rate | none to read | 84 |
| 20 Hz, pressure alone in 10 Pa steps | main opening | none to read | 84 |
| 20 Hz, pressure alone in 100 Pa steps | top speed | `transonic` | 118 |
| 20 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 100 Pa steps | burnout | `transonic` | 118 |
| 20 Hz, pressure alone in 100 Pa steps | drogue rate | none to read | 84 |
| 20 Hz, pressure alone in 100 Pa steps | main opening | none to read | 84 |
| 5 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, altitude, no step | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, altitude, no step | drogue rate | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | drogue rate | none to read | 83 |
| 5 Hz, altitude, no step | main opening | `ends_before_landing` | 1 |
| 5 Hz, altitude, no step | main opening | none to read | 83 |
| 5 Hz, altitude, no step | main rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, altitude in 0.3048 m steps | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, altitude in 0.3048 m steps | drogue rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 83 |
| 5 Hz, altitude in 0.3048 m steps | main opening | `ends_before_landing` | 1 |
| 5 Hz, altitude in 0.3048 m steps | main opening | none to read | 83 |
| 5 Hz, altitude in 0.3048 m steps | main rate | `ends_before_landing` | 1 |
| 5 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, altitude in 10 m steps | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 5 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 5 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, pressure alone in 1 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 1 Pa steps | drogue rate | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 5 Hz, pressure alone in 1 Pa steps | main opening | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 5 Hz, pressure alone in 1 Pa steps | main rate | `ends_before_landing` | 1 |
| 5 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, pressure alone in 10 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 10 Pa steps | drogue rate | none to read | 84 |
| 5 Hz, pressure alone in 10 Pa steps | main opening | none to read | 84 |
| 5 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 5 Hz, pressure alone in 100 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 5 Hz, pressure alone in 100 Pa steps | drogue rate | none to read | 84 |
| 5 Hz, pressure alone in 100 Pa steps | main opening | none to read | 84 |
| 1 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | landing | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | flight time | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | descent rate | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude, no step | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | drogue rate | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | drogue rate | `sampled_too_slowly` | 170 |
| 1 Hz, altitude, no step | main opening | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | main opening | `sampled_too_slowly` | 170 |
| 1 Hz, altitude, no step | main rate | `ends_before_landing` | 1 |
| 1 Hz, altitude, no step | main rate | `sampled_too_slowly` | 170 |
| 1 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude in 0.3048 m steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | drogue rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | main opening | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | main rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude in 10 m steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | drogue rate | `sampled_too_slowly` | 76 |
| 1 Hz, altitude in 10 m steps | drogue rate | none to read | 20 |
| 1 Hz, altitude in 10 m steps | main opening | `sampled_too_slowly` | 76 |
| 1 Hz, altitude in 10 m steps | main opening | none to read | 20 |
| 1 Hz, altitude in 10 m steps | main rate | `sampled_too_slowly` | 76 |
| 1 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | drogue rate | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | main opening | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 1 Pa steps | main rate | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | drogue rate | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | main opening | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 10 Pa steps | main rate | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, pressure alone in 100 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, pressure alone in 100 Pa steps | drogue rate | `sampled_too_slowly` | 76 |
| 1 Hz, pressure alone in 100 Pa steps | drogue rate | none to read | 20 |
| 1 Hz, pressure alone in 100 Pa steps | main opening | `sampled_too_slowly` | 76 |
| 1 Hz, pressure alone in 100 Pa steps | main opening | none to read | 20 |
| 1 Hz, pressure alone in 100 Pa steps | main rate | `sampled_too_slowly` | 76 |
| 0.5 Hz, altitude, no step | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude, no step | landing | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | flight time | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | descent rate | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude, no step | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude, no step | drogue rate | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | drogue rate | `sampled_too_slowly` | 157 |
| 0.5 Hz, altitude, no step | main opening | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | main opening | `sampled_too_slowly` | 157 |
| 0.5 Hz, altitude, no step | main rate | `ends_before_landing` | 14 |
| 0.5 Hz, altitude, no step | main rate | `sampled_too_slowly` | 157 |
| 0.5 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | drogue rate | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | drogue rate | `sampled_too_slowly` | 159 |
| 0.5 Hz, altitude in 0.3048 m steps | main opening | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | main opening | `sampled_too_slowly` | 159 |
| 0.5 Hz, altitude in 0.3048 m steps | main rate | `ends_before_landing` | 12 |
| 0.5 Hz, altitude in 0.3048 m steps | main rate | `sampled_too_slowly` | 159 |
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
| 0.5 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude in 10 m steps | burnout | `sampled_too_slowly` | 170 |
| 0.5 Hz, altitude in 10 m steps | burnout | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | main opening | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | main opening | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | main opening | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | main rate | `ends_before_landing` | 1 |
| 0.5 Hz, altitude in 10 m steps | main rate | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | main rate | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | drogue rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | drogue rate | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 1 Pa steps | main opening | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | main opening | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 1 Pa steps | main rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 1 Pa steps | main rate | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | landing | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | flight time | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | descent rate | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | drogue rate | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | drogue rate | `sampled_too_slowly` | 161 |
| 0.5 Hz, pressure alone in 10 Pa steps | main opening | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | main opening | `sampled_too_slowly` | 161 |
| 0.5 Hz, pressure alone in 10 Pa steps | main rate | `ends_before_landing` | 10 |
| 0.5 Hz, pressure alone in 10 Pa steps | main rate | `sampled_too_slowly` | 161 |
| 0.5 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 100 Pa steps | landing | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | flight time | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | descent rate | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 100 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 100 Pa steps | drogue rate | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | drogue rate | `sampled_too_slowly` | 170 |
| 0.5 Hz, pressure alone in 100 Pa steps | main opening | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | main opening | `sampled_too_slowly` | 170 |
| 0.5 Hz, pressure alone in 100 Pa steps | main rate | `ends_before_landing` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | main rate | `sampled_too_slowly` | 170 |
| its accelerometer at 500 Hz | top speed | `transonic` | 48 |
| its accelerometer at 500 Hz | landing | `ends_before_landing` | 1 |
| its accelerometer at 500 Hz | flight time | `ends_before_landing` | 1 |
| its accelerometer at 500 Hz | descent rate | `ends_before_landing` | 1 |
| its accelerometer at 500 Hz | drogue rate | `ends_before_landing` | 1 |
| its accelerometer at 500 Hz | drogue rate | none to read | 34 |
| its accelerometer at 500 Hz | main opening | `ends_before_landing` | 1 |
| its accelerometer at 500 Hz | main opening | none to read | 34 |
| its accelerometer at 500 Hz | main rate | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | top speed | `transonic` | 48 |
| its accelerometer at 100 Hz | landing | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | flight time | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | descent rate | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | burnout | `transonic` | 1 |
| its accelerometer at 100 Hz | drogue rate | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | drogue rate | none to read | 34 |
| its accelerometer at 100 Hz | main opening | `ends_before_landing` | 1 |
| its accelerometer at 100 Hz | main opening | none to read | 34 |
| its accelerometer at 100 Hz | main rate | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | top speed | `transonic` | 48 |
| its accelerometer at 20 Hz | landing | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | flight time | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | descent rate | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | drogue rate | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | drogue rate | none to read | 34 |
| its accelerometer at 20 Hz | main opening | `ends_before_landing` | 1 |
| its accelerometer at 20 Hz | main opening | none to read | 34 |
| its accelerometer at 20 Hz | main rate | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | top speed | `transonic` | 24 |
| its accelerometer set to ±8 g | landing | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | flight time | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | descent rate | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | drogue rate | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | drogue rate | none to read | 21 |
| its accelerometer set to ±8 g | main opening | `ends_before_landing` | 1 |
| its accelerometer set to ±8 g | main opening | none to read | 21 |
| its accelerometer set to ±8 g | main rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | drogue rate | none to read | 84 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | main opening | none to read | 84 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | liftoff | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | flight time | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | flight time | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | drogue rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main opening | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | liftoff | `short_pad` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | landing | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | flight time | `short_pad` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | descent rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | drogue rate | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main opening | `ends_before_landing` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main rate | `ends_before_landing` | 1 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | liftoff | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | apogee height | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | apogee time | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | top speed | `transonic` | 114 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | top speed | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | landing | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | flight time | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | descent rate | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | burnout | `transonic` | 114 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | burnout | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | drogue rate | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | drogue rate | none to read | 82 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | main opening | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | main opening | none to read | 82 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | main rate | `short_pad` | 3 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | liftoff | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | apogee height | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | apogee time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | top speed | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | landing | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | flight time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | descent rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | burnout | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | drogue rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | main opening | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | main rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | liftoff | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | apogee height | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | apogee time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | top speed | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | landing | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | flight time | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | descent rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | burnout | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | drogue rate | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | main opening | `short_pad` | 171 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | main rate | `short_pad` | 171 |
| altos | top speed | `transonic` | 24 |
| altos | burnout | `transonic` | 1 |
| altos | drogue rate | none to read | 13 |
| altos | main opening | none to read | 13 |
| gnss-tracker | liftoff | `receiver_dropout` | 29 |
| gnss-tracker | apogee height | `receiver_lag` | 29 |
| gnss-tracker | apogee time | `receiver_lag` | 29 |
| gnss-tracker | top speed | `receiver_lag` | 29 |
| gnss-tracker | landing | `needs` | 29 |
| gnss-tracker | flight time | `receiver_dropout` | 29 |
| gnss-tracker | descent rate | `needs` | 29 |
| gnss-tracker | top acceleration | `no_accelerometer` | 29 |
| gnss-tracker | burnout | `receiver_lag` | 29 |
| gnss-tracker | drogue rate | `needs` | 29 |
| gnss-tracker | main opening | `needs` | 29 |
| gnss-tracker | main rate | `needs` | 29 |
| eggtimer | top speed | `transonic` | 25 |
| eggtimer | top acceleration | `no_accelerometer` | 39 |
| eggtimer | burnout | `transonic` | 25 |
| eggtimer | drogue rate | none to read | 16 |
| eggtimer | main opening | none to read | 16 |
| perfectflite | top speed | `transonic` | 21 |
| perfectflite | top acceleration | `no_accelerometer` | 31 |
| perfectflite | burnout | `transonic` | 21 |
| perfectflite | drogue rate | none to read | 20 |
| perfectflite | main opening | none to read | 20 |
| rrc3 | top speed | `transonic` | 23 |
| rrc3 | top acceleration | `no_accelerometer` | 30 |
| rrc3 | burnout | `transonic` | 23 |
| rrc3 | drogue rate | none to read | 13 |
| rrc3 | main opening | none to read | 13 |

## Errors on the tuning set

Each reading's errors, the reading less the truth, over every tuning flight and every log above together, beside the mean bound the readings stated. Each error is held to its own reading's bound, which differs from flight to flight and log to log, so the least or the most may pass the mean bound; the mean is for scale, and is given at the decimal place of the larger of the least and the most to three figures, the digits every platform shares. A canopy's rates are compared with the truth over the span the reader chose, its leg's ends, so an opening read early or late shows in the opening's error, not in the rates'. A floor, a burnout from the speed's peak or a clipped top acceleration, is apart: it holds while it is no more than its bound above the truth, and may lie any amount below it.

| Reading | Given | Mean error | Least | Most | Mean bound |
|---|---:|---:|---:|---:|---:|
| liftoff | 7051 | 0.12 s | -1.79 s | 1.21 s | 2.03 s |
| apogee height | 7258 | 0 m | -179 m | 202 m | 200 m |
| apogee time | 7258 | 0.04 s | -1.04 s | 1.2 s | 1.54 s |
| top speed | 1322 | 2.2 m/s | -9.5 m/s | 28.2 m/s | 38.6 m/s |
| landing | 7191 | -0.6 s | -4.4 s | 2.61 s | 3.83 s |
| flight time | 6985 | -0.74 s | -5.06 s | 3.15 s | 5.81 s |
| descent rate | 7191 | 0 m/s | -12.4 m/s | 5.66 m/s | 4.09 m/s |
| top acceleration | 290 | 0.6 m/s² | -13.6 m/s² | 13 m/s² | 16.3 m/s² |
| top acceleration, a floor | 32 | -48 m/s² | -75.9 m/s² | -1.58 m/s² | 12.9 m/s² |
| burnout | 300 | -0.0102 s | -0.0943 s | 0.0476 s | 0.582 s |
| burnout, a floor | 1236 | -0.24 s | -1.33 s | 0.718 s | 1.3 s |
| drogue rate | 2843 | 0.04 m/s | -1.61 m/s | 1.35 m/s | 1.79 m/s |
| main opening | 2843 | 0.72 s | -0.778 s | 5.3 s | 2.04 s |
| main rate | 5428 | 0.04 m/s | -5.47 m/s | 4.83 m/s | 1.71 m/s |

No tuning flight's true top acceleration reaches the range its own part clips at: the nearest, flight 57's on its imu-500hz logger, comes to 0.492 of it. With its accelerometer set to ±8 g, 28 of the 38 flights logged so have a true top that reaches that range, and 10 don't. Tops read as floors: 32; clipped at the part's range, 28; held flat at the top, 4; where the accelerometer holds the thrust only in part, with no start or no end to it, 0.

Clipped at the part's range: its accelerometer set to ±8 g, imu-500hz flights 0, 3, 4, 5, 7, 8, 26, 31, 34, 49, 57, 70, 73, 107, 111, 118, 123, 132, 133, 138, 141, 160, 163, 167, 175, 190, 196, 198.

Held flat at the top: its accelerometer at 20 Hz, altos flights 78, 122, 144, 161.

## The smoother against the truth

Each flight's own log smoothed (`hpr_flightdata::smoother`, M7.2d, ADR-300) and its height above the pad and vertical speed compared with the truth's at burnout, the top vertical speed, apogee and one moment drawn per flight. Each cell gives the share of flights whose truth lies inside the smoother's 90% interval, then how many lay below and above it. A cell **fails** when the lower edge of the 99.9% Clopper–Pearson band on either side's miss share is above 5%. Logs without a barometer are withheld.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed | | 171 | 820 | 845 |
| burnout | height | 96.5% (5 / 1) | 97.7% (18 / 1) | 96.7% (27 / 1) |
| burnout | speed | 96.5% (2 / 4) | 96.6% (16 / 12) | 96.6% (20 / 9) |
| top speed | height | 96.5% (5 / 1) | 97.6% (19 / 1) | 97.3% (22 / 1) |
| top speed | speed | 95.3% (8 / 0) | 96.7% (26 / 1) | 94.3% (47 / 1) |
| apogee | height | 98.8% (2 / 0) | 97.6% (15 / 5) | 97.3% (19 / 4) |
| apogee | speed | 96.5% (0 / 6) | 96.1% (5 / 27) | 96% (2 / 32) |
| a drawn moment | height | 99.4% (1 / 0) | 97.3% (18 / 4) | 97.3% (21 / 2) |
| a drawn moment | speed | 97.7% (3 / 1) | 97.3% (9 / 13) | 97.2% (14 / 10) |

### The standard errors

The median and the largest, of height in m and of speed in m/s.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| burnout | height | 58.8 / 347 | 58.9 / 406 | 59.1 / 407 |
| burnout | speed | 12.1 / 169 | 11.5 / 185 | 11.3 / 239 |
| top speed | height | 62 / 302 | 61.3 / 333 | 62.3 / 363 |
| top speed | speed | 13.4 / 227 | 12.2 / 213 | 10.9 / 214 |
| apogee | height | 86.8 / 142 | 84.5 / 159 | 84.3 / 158 |
| apogee | speed | 1.16 / 2.05 | 1.24 / 2.25 | 1.23 / 2.25 |
| a drawn moment | height | 25 / 157 | 20.6 / 221 | 18.5 / 243 |
| a drawn moment | speed | 0.992 / 90 | 1.04 / 147 | 1.01 / 260 |

### By logger

The share inside the interval, each set's flights by logger.

#### Tuning

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 97% (0 / 1) | 97.4% (1 / 0) | 100% (0 / 0) | 90.3% (3 / 0) | 96.7% (1 / 0) |
| burnout | speed | 93.9% (2 / 0) | 100% (0 / 0) | 100% (0 / 0) | 90.3% (0 / 3) | 96.7% (0 / 1) |
| top speed | height | 97% (0 / 1) | 97.4% (1 / 0) | 100% (0 / 0) | 90.3% (3 / 0) | 96.7% (1 / 0) |
| top speed | speed | 90.9% (3 / 0) | 97.4% (1 / 0) | 92.1% (3 / 0) | 96.8% (1 / 0) | 100% (0 / 0) |
| apogee | height | 93.9% (2 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) |
| apogee | speed | 100% (0 / 0) | 94.9% (0 / 2) | 97.4% (0 / 1) | 96.8% (0 / 1) | 93.3% (0 / 2) |
| a drawn moment | height | 97% (1 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) |
| a drawn moment | speed | 93.9% (2 / 0) | 97.4% (0 / 1) | 100% (0 / 0) | 96.8% (1 / 0) | 100% (0 / 0) |

#### A

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 98.9% (2 / 0) | 96.1% (7 / 0) | 99.4% (0 / 1) | 95.8% (6 / 0) | 97.8% (3 / 0) |
| burnout | speed | 93% (13 / 0) | 96.1% (0 / 7) | 98.2% (3 / 0) | 97.2% (0 / 4) | 99.3% (0 / 1) |
| top speed | height | 99.5% (1 / 0) | 95.6% (8 / 0) | 99.4% (0 / 1) | 95.8% (6 / 0) | 97.1% (4 / 0) |
| top speed | speed | 93% (13 / 0) | 98.9% (1 / 1) | 94.7% (9 / 0) | 99.3% (1 / 0) | 98.6% (2 / 0) |
| apogee | height | 89.7% (15 / 4) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 99.3% (0 / 1) |
| apogee | speed | 99.5% (1 / 0) | 94.5% (2 / 8) | 93.6% (1 / 10) | 95.1% (1 / 6) | 97.8% (0 / 3) |
| a drawn moment | height | 91.4% (14 / 2) | 99.4% (1 / 0) | 98.8% (1 / 1) | 100% (0 / 0) | 97.8% (2 / 1) |
| a drawn moment | speed | 95.7% (2 / 6) | 98.3% (2 / 1) | 97.1% (3 / 2) | 97.9% (2 / 1) | 97.8% (0 / 3) |

#### B

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 99.5% (0 / 1) | 94.1% (10 / 0) | 98.9% (2 / 0) | 96.2% (5 / 0) | 94.3% (10 / 0) |
| burnout | speed | 94.6% (10 / 0) | 97% (0 / 5) | 95.1% (9 / 0) | 98.5% (1 / 1) | 98.3% (0 / 3) |
| top speed | height | 99.5% (0 / 1) | 94.1% (10 / 0) | 100% (0 / 0) | 97.7% (3 / 0) | 94.9% (9 / 0) |
| top speed | speed | 92.5% (14 / 0) | 97% (5 / 0) | 92.3% (14 / 0) | 93.9% (7 / 1) | 96% (7 / 0) |
| apogee | height | 89.2% (19 / 1) | 100% (0 / 0) | 100% (0 / 0) | 98.5% (0 / 2) | 99.4% (0 / 1) |
| apogee | speed | 99.5% (0 / 1) | 93.5% (0 / 11) | 96.2% (0 / 7) | 93.9% (2 / 6) | 96% (0 / 7) |
| a drawn moment | height | 90.3% (18 / 0) | 98.8% (1 / 1) | 100% (0 / 0) | 99.2% (1 / 0) | 98.9% (1 / 1) |
| a drawn moment | speed | 98.4% (1 / 2) | 95.3% (6 / 2) | 97.8% (2 / 2) | 93.9% (5 / 3) | 99.4% (0 / 1) |

### Trusting the GNSS receiver from liftoff

The same flights, each log with a GNSS track smoothed again with its receiver trusted from liftoff rather than once its speed agrees with the smoother's. A receiver lags a climb, so this must fail.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed again | | 33 | 185 | 186 |
| burnout | height | 93.6% (5 / 6) | 94% (21 / 28) | 92.4% (29 / 35) |
| burnout | speed | 89.5% (6 / 12) | 87% (73 / 34) **fails** | 88.9% (59 / 35) |
| top speed | height | 93.6% (5 / 6) | 95% (18 / 23) | 94.1% (25 / 25) |
| top speed | speed | 87.1% (15 / 7) | 87.9% (75 / 24) **fails** | 84.5% (99 / 32) **fails** |
| apogee | height | 87.7% (20 / 1) **fails** | 81% (149 / 7) **fails** | 82% (146 / 6) **fails** |
| apogee | speed | 92.4% (4 / 9) | 93.9% (16 / 34) | 93.3% (14 / 43) |
| a drawn moment | height | 88.9% (15 / 4) | 84.1% (112 / 18) **fails** | 85.1% (112 / 14) **fails** |
| a drawn moment | speed | 96.5% (4 / 2) | 95.2% (20 / 19) | 95.3% (21 / 19) |

### Health

The smoothed flights' health cards, summed.

| | Tuning | A | B |
|---|---:|---:|---:|
| GNSS: logs | 33 | 185 | 186 |
| GNSS: samples rejected / logged / not weighed | 2 / 38300 / 8040 | 18 / 184000 / 37500 | 22 / 175000 / 35000 |
| GNSS: logs with a rejection | 1 | 10 | 12 |
| GNSS: resets | 0 | 0 | 0 |
| accelerometer: logs | 71 | 356 | 368 |
| accelerometer: samples rejected / logged / not weighed | 0 / 4440000 / 3810000 | 0 / 18800000 / 15900000 | 0 / 18700000 / 15600000 |
| accelerometer: logs with a rejection | 0 | 0 | 0 |
| accelerometer: resets | 0 | 0 | 0 |
| barometer: logs | 171 | 820 | 845 |
| barometer: samples rejected / logged / not weighed | 4450 / 4860000 / 0 | 13800 / 20600000 / 0 | 13400 / 20600000 / 0 |
| barometer: logs with a rejection | 63 | 319 | 348 |
| barometer: resets | 9 | 29 | 35 |
| GNSS receivers trusted | 30 | 173 | 177 |
| Transonic spans de-weighted | 107 | 479 | 510 |
| Charge spans de-weighted | 171 | 820 | 845 |
| Steps the pass back kept forward | 0 | 0 | 0 |

### A health card for each logger

Each logger's first smoothed flight in the tuning set; times are on the log's clock.

**perfectflite**, flight 16:

```text
barometer: 10737 samples, 10736 fused, 1 rejected, 0 not weighed, 0 resets; 99.9% inside the gate; mean NIS 0.723; largest test ratio 2.71 at 31.9 s, Mach 0.00
barometer de-weighted: transonic 7.40–14.2 s, charge 32.0–32.5 s
pass back: every step smoothed
```

**rrc3**, flight 21:

```text
barometer: 5593 samples, 5593 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.709; largest test ratio 0.585 at 141 s, Mach 0.01
barometer de-weighted: charge 27.9–29.9 s
pass back: every step smoothed
```

**eggtimer**, flight 9:

```text
barometer: 3044 samples, 3044 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.676; largest test ratio 0.424 at 47.5 s, Mach 0.09
barometer de-weighted: charge 27.4–28.0 s
pass back: every step smoothed
```

**altos**, flight 1:

```text
barometer: 6029 samples, 6029 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.675; largest test ratio 0.456 at 148 s, Mach 0.05
accelerometer: 6029 samples, 3479 fused, 0 rejected, 2550 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.0430; largest test ratio 0.0713 at 10.0 s, Mach 0.00
GNSS: 2833 samples, 2425 fused, 0 rejected, 408 not weighed, 0 resets; 100.0% inside the gate; mean NIS 1.18; largest test ratio 0.865 at 204 s, Mach 0.00
barometer de-weighted: transonic 12.8–13.4 s, charge 33.4–35.9 s
GNSS trusted from 50.5 s
barometer less accelerometer at apogee: -309 m
pass back: every step smoothed
```

**imu-500hz**, flight 0:

```text
barometer: 34602 samples, 34602 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.617; largest test ratio 0.736 at 23.9 s, Mach 0.22
accelerometer: 34602 samples, 15404 fused, 0 rejected, 19198 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.00850; largest test ratio 0.0361 at 6.57 s, Mach 0.00
barometer de-weighted: transonic 8.68–13.5 s, charge 29.5–32.1 s
barometer less accelerometer at apogee: 88.0 m
pass back: every step smoothed
```

## The readings' 90% intervals

Each flight's own log read by `hpr_flightdata::readings`, and each reading that states a 90% interval compared with the flight's truth (M7.2e1 and M7.2e2, ADR-257 §1–2, ADR-308). An interval runs from the reading less its 95% share of its bound to the reading less its 5% share; the shares are the 5% and 95% points of the readings' errors as shares of their bounds on the calibration set's 2000 flights (362 of them the GNSS-only tracker's, which give no apogee), a seed of their own that the test sets never share. `cargo xtask synthetic` fits them again and fails unless they equal the library's to three significant figures. A reading **holds** when the two-sided 99.9% Clopper–Pearson band on the share of truths below its interval and the band on the share above each hold 5%, and the band on the share inside holds 90%; and every probe must fail that test. The command also fails if a reading is withheld from a log with a barometer, or read with no interval, on any of the three sets: either would leave the counts with its misses. The one reading withheld by its own rule is the top speed at or above 227.7 m/s, where the barometer's port may read the transonic flow over the nose; it looks at the reading, not the truth, so the test holds the top speeds a user is given.

### The shares

| Reading | Readings fitted | 5% share | 95% share |
|---|---:|---:|---:|
| liftoff | 1638 | 0.203 | 0.482 |
| apogee height | 1638 | -0.63 | 0.637 |
| apogee time | 1638 | -0.337 | 0.225 |
| top speed | 577 | -0.216 | 0.265 |
| landing | 1638 | -0.332 | 0.157 |
| flight time | 1638 | -0.381 | -0.133 |
| descent rate | 1638 | -0.473 | 0.493 |

### The test

The truths below, inside and above each interval, then the shares, each set's flights flown. Floors and withheld readings aren't compared.

| Set | Reading | Compared | Below / inside / above | Below | Above | Floors | Withheld | Result |
|---|---|---:|---:|---:|---:|---:|---:|---|
| A | liftoff | 820 | 35 / 729 / 56 | 4.27% | 6.83% | 0 | 180 | holds |
| A | apogee height | 820 | 55 / 720 / 45 | 6.71% | 5.49% | 0 | 180 | holds |
| A | apogee time | 820 | 36 / 752 / 32 | 4.39% | 3.9% | 0 | 180 | holds |
| A | top speed | 287 | 11 / 258 / 18 | 3.83% | 6.27% | 0 | 713 | holds |
| A | landing | 820 | 38 / 746 / 36 | 4.63% | 4.39% | 0 | 180 | holds |
| A | flight time | 820 | 38 / 750 / 32 | 4.63% | 3.9% | 0 | 180 | holds |
| A | descent rate | 820 | 47 / 728 / 45 | 5.73% | 5.49% | 0 | 180 | holds |
| B | liftoff | 845 | 52 / 739 / 54 | 6.15% | 6.39% | 0 | 155 | holds |
| B | apogee height | 845 | 38 / 760 / 47 | 4.5% | 5.56% | 0 | 155 | holds |
| B | apogee time | 845 | 41 / 754 / 50 | 4.85% | 5.92% | 0 | 155 | holds |
| B | top speed | 286 | 9 / 263 / 14 | 3.15% | 4.9% | 0 | 714 | holds |
| B | landing | 845 | 23 / 788 / 34 | 2.72% | 4.02% | 0 | 155 | holds |
| B | flight time | 845 | 37 / 763 / 45 | 4.38% | 5.33% | 0 | 155 | holds |
| B | descent rate | 845 | 41 / 754 / 50 | 4.85% | 5.92% | 0 | 155 | holds |

### The probes

Each interval narrowed to 0.8 of its width about its middle, widened to 1.25, and shifted down by one stated standard error (its width over 2 × 1.645): below / inside / above. Each must fail the test.

| Set | Reading | narrowed to 0.8 | widened to 1.25 | shifted down one standard error |
|---|---|---:|---:|---:|
| A | liftoff | 64 / 628 / 128, fails | 9 / 798 / 13, fails | 2 / 472 / 346, fails |
| A | apogee height | 115 / 592 / 113, fails | 0 / 820 / 0, fails | 0 / 537 / 283, fails |
| A | apogee time | 61 / 690 / 69, fails | 5 / 803 / 12, fails | 1 / 680 / 139, fails |
| A | top speed | 24 / 237 / 26, fails | 2 / 279 / 6, fails | 0 / 211 / 76, fails |
| A | landing | 45 / 655 / 120, fails | 22 / 795 / 3, fails | 5 / 327 / 488, fails |
| A | flight time | 87 / 670 / 63, fails | 11 / 802 / 7, fails | 3 / 626 / 191, fails |
| A | descent rate | 96 / 626 / 98, fails | 0 / 816 / 4, fails | 0 / 563 / 257, fails |
| B | liftoff | 80 / 657 / 108, fails | 26 / 807 / 12, fails | 10 / 474 / 361, fails |
| B | apogee height | 137 / 581 / 127, fails | 0 / 845 / 0, fails | 0 / 562 / 283, fails |
| B | apogee time | 68 / 691 / 86, fails | 3 / 815 / 27, fails | 1 / 679 / 165, fails |
| B | top speed | 33 / 224 / 29, fails | 3 / 275 / 8, fails | 0 / 204 / 82, fails |
| B | landing | 30 / 687 / 128, fails | 17 / 827 / 1, fails | 4 / 353 / 488, fails |
| B | flight time | 76 / 681 / 88, fails | 12 / 817 / 16, fails | 3 / 650 / 192, fails |
| B | descent rate | 111 / 625 / 109, fails | 0 / 841 / 4, fails | 0 / 580 / 265, fails |

### Each reading's error side

Each reading's mean error, the reading less the truth, over the readings compared, with the largest error's size and the share of truths above the interval (ADR-257 §2). For the apogee's height, the top speed and the mean descent rate, a reading too low is the flattering side: a waiver's height, a fin's margin, a landing's speed. The share of truths above the interval is that side's test, and the shifted-down probe its mutation probe; the times have no flattering side. The mean is given at the figures of the largest error, the digits every platform shares, and with its sign: +0 is a mean too small for those figures that isn't below zero.

| Set | Reading | Flattering side | Mean error | Largest error | Truths above |
|---|---|---|---:|---:|---:|
| A | liftoff | none | +0.243 s | 0.471 s | 6.83% |
| A | apogee height | low | +1 m | 237 m | 5.49% |
| A | apogee time | none | +0.006 s | 0.449 s | 3.9% |
| A | top speed | low | +1.2 m/s | 20.4 m/s | 6.27% |
| A | landing | none | -0.38 s | 1.33 s | 4.39% |
| A | flight time | none | -0.62 s | 1.56 s | 3.9% |
| A | descent rate | low | -0.01 m/s | 6.81 m/s | 5.49% |
| B | liftoff | none | +0.247 s | 0.554 s | 6.39% |
| B | apogee height | low | +0 m | 234 m | 5.56% |
| B | apogee time | none | +0.006 s | 0.526 s | 5.92% |
| B | top speed | low | +1.3 m/s | 26.8 m/s | 4.9% |
| B | landing | none | -0.37 s | 1.23 s | 4.02% |
| B | flight time | none | -0.62 s | 1.5 s | 5.33% |
| B | descent rate | low | -0.01 m/s | 6.34 m/s | 5.92% |

### By design

Below / inside / above.

Set A:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 2 / 72 / 2 | 1 / 75 / 0 | 3 / 72 / 1 | 1 / 75 / 0 | 0 / 69 / 7 | 0 / 67 / 9 | 0 / 76 / 0 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 80 / 2 | 8 / 72 / 2 | 5 / 75 / 2 | 0 / 2 / 3 | 0 / 82 / 0 | 3 / 79 / 0 | 9 / 71 / 2 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 75 / 7 | 2 / 76 / 4 | 1 / 77 / 4 | 0 / 0 / 2 | 0 / 81 / 1 | 2 / 80 / 0 | 3 / 76 / 3 |
| rocketpy-cavour | 0 / 64 / 21 | 7 / 72 / 6 | 8 / 71 / 6 | none | 38 / 47 / 0 | 21 / 64 / 0 | 1 / 79 / 5 |
| rocketpy-genesis | 0 / 77 / 12 | 8 / 76 / 5 | 4 / 84 / 1 | none | 0 / 87 / 2 | 3 / 86 / 0 | 10 / 70 / 9 |
| rocketpy-juno-iii | 22 / 54 / 0 | 3 / 65 / 8 | 2 / 73 / 1 | 2 / 22 / 6 | 0 / 76 / 0 | 0 / 67 / 9 | 4 / 64 / 8 |
| rocketpy-lince | 0 / 65 / 8 | 9 / 56 / 8 | 4 / 65 / 4 | none | 0 / 73 / 0 | 8 / 65 / 0 | 11 / 51 / 11 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 88 / 4 | 4 / 83 / 5 | 2 / 87 / 3 | 0 / 91 / 1 | 0 / 69 / 23 | 0 / 91 / 1 | 0 / 92 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 9 / 68 / 6 | 6 / 69 / 8 | none | 0 / 83 / 0 | 1 / 82 / 0 | 9 / 67 / 7 |
| rocketpy-valetudo | 10 / 72 / 0 | 4 / 77 / 1 | 1 / 79 / 2 | 8 / 68 / 6 | 0 / 79 / 3 | 0 / 69 / 13 | 0 / 82 / 0 |

Set B:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 9 / 77 / 1 | 2 / 84 / 1 | 0 / 84 / 3 | 0 / 84 / 3 | 0 / 80 / 7 | 0 / 70 / 17 | 0 / 87 / 0 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 81 / 10 | 5 / 81 / 5 | 2 / 82 / 7 | 0 / 2 / 3 | 0 / 90 / 1 | 5 / 86 / 0 | 7 / 79 / 5 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 94 / 8 | 3 / 94 / 5 | 6 / 89 / 7 | 0 / 4 / 2 | 0 / 102 / 0 | 1 / 101 / 0 | 7 / 91 / 4 |
| rocketpy-cavour | 0 / 64 / 14 | 2 / 71 / 5 | 2 / 66 / 10 | none | 23 / 55 / 0 | 16 / 62 / 0 | 1 / 71 / 6 |
| rocketpy-genesis | 0 / 85 / 7 | 5 / 85 / 2 | 4 / 83 / 5 | none | 0 / 92 / 0 | 1 / 91 / 0 | 7 / 81 / 4 |
| rocketpy-juno-iii | 29 / 55 / 0 | 6 / 71 / 7 | 7 / 73 / 4 | 2 / 30 / 5 | 0 / 80 / 4 | 0 / 67 / 17 | 7 / 66 / 11 |
| rocketpy-lince | 0 / 66 / 11 | 2 / 68 / 7 | 7 / 64 / 6 | none | 0 / 76 / 1 | 13 / 64 / 0 | 6 / 59 / 12 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 65 / 3 | 5 / 57 / 6 | 2 / 64 / 2 | 0 / 68 / 0 | 0 / 52 / 16 | 0 / 68 / 0 | 0 / 68 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 3 / 72 / 8 | 7 / 74 / 2 | none | 0 / 83 / 0 | 1 / 82 / 0 | 6 / 69 / 8 |
| rocketpy-valetudo | 13 / 70 / 0 | 5 / 77 / 1 | 4 / 75 / 4 | 7 / 75 / 1 | 0 / 78 / 5 | 0 / 72 / 11 | 0 / 83 / 0 |

### By logger and by the day's air

Below / inside / above, and in bold where that group's counts alone would fail the test. The test is on each set as a whole; these groups show where its misses fall. A barometric height is off by about the day's offset from the standard atmosphere over its temperature, so the misses of the height, the top speed and the descent rate gather on the coldest and warmest days: a warm day reads low, and its truth lies above the interval.

Set A:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | **0 / 126 / 18** | 10 / 125 / 9 | 10 / 134 / 0 | 1 / 45 / 4 | 11 / 130 / 3 | 9 / 132 / 3 | 8 / 131 / 5 |
| rrc3 | 1 / 125 / 13 | 8 / 124 / 7 | 1 / 135 / 3 | 2 / 42 / 3 | 6 / 125 / 8 | 3 / 132 / 4 | 8 / 123 / 8 |
| eggtimer | **3 / 155 / 23** | 15 / 158 / 8 | 18 / 161 / 2 | 3 / 59 / 6 | 10 / 165 / 6 | 12 / 167 / 2 | 8 / 166 / 7 |
| altos | 17 / 166 / 2 | 11 / 162 / 12 | **2 / 156 / 27** | 3 / 62 / 2 | 11 / 171 / 3 | 13 / 171 / 1 | 13 / 158 / 14 |
| imu-500hz | **14 / 157 / 0** | 11 / 151 / 9 | **5 / 166 / 0** | 2 / 50 / 3 | **0 / 155 / 16** | **1 / 148 / 22** | 10 / 150 / 11 |

Set B:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | 3 / 118 / 11 | 5 / 121 / 6 | 9 / 119 / 4 | 0 / 39 / 3 | 5 / 125 / 2 | 7 / 122 / 3 | 5 / 120 / 7 |
| rrc3 | 6 / 153 / 17 | 5 / 163 / 8 | 7 / 163 / 6 | 2 / 60 / 1 | 5 / 163 / 8 | 4 / 164 / 8 | 6 / 163 / 7 |
| eggtimer | **3 / 143 / 23** | 12 / 147 / 10 | 15 / 146 / 8 | 2 / 64 / 3 | 5 / 159 / 5 | 10 / 157 / 2 | 11 / 149 / 9 |
| altos | **23 / 160 / 3** | 4 / 168 / 14 | **4 / 152 / 30** | 4 / 49 / 4 | 8 / 176 / 2 | 14 / 170 / 2 | 8 / 158 / 20 |
| imu-500hz | **17 / 165 / 0** | 12 / 161 / 9 | 6 / 174 / 2 | 1 / 51 / 3 | **0 / 165 / 17** | **2 / 150 / 30** | 11 / 164 / 7 |

Set A:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 3 / 115 / 14 | **55 / 77 / 0** | 10 / 114 / 8 | **10 / 24 / 0** | 7 / 120 / 5 | 10 / 118 / 4 | **47 / 85 / 0** |
| −10 to +10 K | 27 / 489 / 35 | **0 / 551 / 0** | 21 / 510 / 20 | **1 / 194 / 1** | 26 / 500 / 25 | 23 / 505 / 23 | **0 / 551 / 0** |
| warmer than +10 K | 5 / 125 / 7 | **0 / 92 / 45** | 5 / 128 / 4 | **0 / 40 / 17** | 5 / 126 / 6 | 5 / 127 / 5 | **0 / 92 / 45** |

Set B:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 14 / 140 / 9 | **38 / 125 / 0** | 12 / 143 / 8 | 7 / 45 / 0 | 3 / 151 / 9 | 5 / 148 / 10 | **41 / 122 / 0** |
| −10 to +10 K | 30 / 457 / 39 | **0 / 526 / 0** | 23 / 468 / 35 | **2 / 163 / 0** | 16 / 490 / 20 | 24 / 473 / 29 | **0 / 526 / 0** |
| warmer than +10 K | 8 / 142 / 6 | **0 / 109 / 47** | 6 / 143 / 7 | **0 / 55 / 14** | 4 / 147 / 5 | 8 / 142 / 6 | **0 / 106 / 50** |
