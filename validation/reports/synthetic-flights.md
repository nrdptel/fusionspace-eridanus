# Synthetic flights

The census of the synthetic flights (M7.2a, ADR-280): each set's flights by design and by logger, the ranges they span, and each draw that didn't fly. Written by `cargo xtask synthetic`; `cargo xtask synthetic --check` holds it on every platform. Values are to three significant figures, but for the two that the logs written from launch hold to two. What the flights are and how far to trust them: `docs/physics/synthetic-flights.md`.

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

### Logs written from launch

The same flights, each log cut at the truth's liftoff, every sensor, as a logger that writes only from launch would have written it on the same zero. Each is smoothed on an assumed pad, and held as the check above is: it fails on a cell that fails, overall or for a logger, or a log withheld with a barometer. Its standard errors and sums of samples are held to two figures, the whole logs' to three, since CI's Linux runners read them in another third figure's bin, for a cause not yet traced (ADR-308): a drift shows here only past a second figure's edge.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed | | 171 | 820 | 845 |
| Accelerometers set aside | | 71 | 356 | 368 |
| Apogees past the reading's bound | | 0 | 1 | 2 |
| burnout | height | 92.4% (13 / 0) | 96.7% (27 / 0) | 95.6% (37 / 0) |
| burnout | speed | 95.9% (0 / 7) | 98.2% (1 / 14) | 98.2% (2 / 13) |
| top speed | height | 93% (12 / 0) | 96.7% (27 / 0) | 95.9% (35 / 0) |
| top speed | speed | 97.7% (4 / 0) | 98.3% (12 / 2) | 96.7% (27 / 1) |
| apogee | height | 99.4% (1 / 0) | 98.7% (10 / 1) | 98% (12 / 5) |
| apogee | speed | 95.9% (0 / 7) | 96.2% (5 / 26) | 94.9% (8 / 35) |
| a drawn moment | height | 99.4% (1 / 0) | 98.3% (13 / 1) | 98.2% (14 / 1) |
| a drawn moment | speed | 98.8% (2 / 0) | 97% (12 / 13) | 96.9% (13 / 13) |

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

Each flight's own log read by `hpr_flightdata::readings`, and each reading that states a 90% interval compared with the flight's truth (M7.2e1 to M7.2e3, ADR-257 §1–2, ADR-308). An interval runs from the reading less its 95% share of its bound to the reading less its 5% share; the shares are the 5% and 95% points of the readings' errors as shares of their bounds on the calibration set's 2000 flights (362 of them the GNSS-only tracker's, which give no apogee), a seed of their own that the test sets never share. `cargo xtask synthetic` fits them again and fails unless they equal the library's to three significant figures. A reading **holds** when the two-sided 99.9% Clopper–Pearson band on the share of truths below its interval and the band on the share above each hold 5%, and the band on the share inside holds 90%; and every probe must fail that test. The command also fails if a reading is withheld from a log with a barometer, or read with no interval, on any of the three sets: either would leave the counts with its misses. Withheld by their own rules are the top speed at or above 227.7 m/s, where the barometer's port may read the transonic flow over the nose (it looks at the reading, not the truth, so the test holds the top speeds a user is given); a burnout taken from that speed; and a top acceleration on a log with no accelerometer.

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
| top acceleration | 641 | -0.495 | 0.568 |
| burnout | 565 | -0.0653 | 0.0689 |
| drogue rate | 781 | -0.622 | 0.612 |
| main opening | 781 | 0.138 | 0.566 |
| main rate | 1638 | -0.572 | 0.583 |

### The floors' shares

A floor's truth may lie any amount above it, so it states a lower end only: the reading less its 95% share of its bound. Its standard error is the 95% share less the median, times the bound, over 1.645. No flight's own accelerometer clips, so the top acceleration's floors come from each flight's log with its accelerometer clipping at 8 g and its top scaled to anywhere the reader can call it clipped and the part may not have clipped: from (1 − 0.3/8)/(1 + s) to 1/(1 − s) of that clip, s the part's scale error, ±3% on the 500 Hz IMU and ±10% on the AltOS part.

| Reading | Floors fitted | 95% share | Median share |
|---|---:|---:|---:|
| top acceleration | 348 | 0.259 | -0.124 |
| burnout | 437 | -0.0339 | -0.195 |

### The test

The truths below, inside and above each interval, then the shares, each set's flights flown. Floors are tested below; withheld readings, and a drogue or a main's opening on a flight with one canopy, aren't compared.

| Set | Reading | Compared | Below / inside / above | Below | Above | Floors | Withheld | Result |
|---|---|---:|---:|---:|---:|---:|---:|---|
| A | liftoff | 820 | 35 / 729 / 56 | 4.27% | 6.83% | 0 | 180 | holds |
| A | apogee height | 820 | 55 / 720 / 45 | 6.71% | 5.49% | 0 | 180 | holds |
| A | apogee time | 820 | 36 / 752 / 32 | 4.39% | 3.9% | 0 | 180 | holds |
| A | top speed | 287 | 11 / 258 / 18 | 3.83% | 6.27% | 0 | 713 | holds |
| A | landing | 820 | 38 / 746 / 36 | 4.63% | 4.39% | 0 | 180 | holds |
| A | flight time | 820 | 38 / 750 / 32 | 4.63% | 3.9% | 0 | 180 | holds |
| A | descent rate | 820 | 47 / 728 / 45 | 5.73% | 5.49% | 0 | 180 | holds |
| A | top acceleration | 356 | 12 / 322 / 22 | 3.37% | 6.18% | 181 | 644 | holds |
| A | burnout | 309 | 10 / 289 / 10 | 3.24% | 3.24% | 210 | 481 | holds |
| A | drogue rate | 428 | 33 / 379 / 16 | 7.71% | 3.74% | 0 | 180 | holds |
| A | main opening | 428 | 22 / 383 / 23 | 5.14% | 5.37% | 0 | 180 | holds |
| A | main rate | 820 | 46 / 734 / 40 | 5.61% | 4.88% | 0 | 180 | holds |
| B | liftoff | 845 | 52 / 739 / 54 | 6.15% | 6.39% | 0 | 155 | holds |
| B | apogee height | 845 | 38 / 760 / 47 | 4.5% | 5.56% | 0 | 155 | holds |
| B | apogee time | 845 | 41 / 754 / 50 | 4.85% | 5.92% | 0 | 155 | holds |
| B | top speed | 286 | 9 / 263 / 14 | 3.15% | 4.9% | 0 | 714 | holds |
| B | landing | 845 | 23 / 788 / 34 | 2.72% | 4.02% | 0 | 155 | holds |
| B | flight time | 845 | 37 / 763 / 45 | 4.38% | 5.33% | 0 | 155 | holds |
| B | descent rate | 845 | 41 / 754 / 50 | 4.85% | 5.92% | 0 | 155 | holds |
| B | top acceleration | 368 | 18 / 330 / 20 | 4.89% | 5.43% | 187 | 632 | holds |
| B | burnout | 327 | 25 / 285 / 17 | 7.65% | 5.2% | 211 | 462 | holds |
| B | drogue rate | 436 | 24 / 393 / 19 | 5.5% | 4.36% | 0 | 155 | holds |
| B | main opening | 436 | 13 / 394 / 29 | 2.98% | 6.65% | 0 | 155 | holds |
| B | main rate | 845 | 40 / 758 / 47 | 4.73% | 5.56% | 0 | 155 | holds |

### The floors' test

Each floor's lower end against the flight's truth: the band on the share of truths below it must hold 5%, the coverage test's lower tail. Raised by one standard error and lowered by two, it must fail.

| Set | Reading | Compared | Below | Share | Band | Raised one standard error | Lowered two standard errors | Result |
|---|---|---:|---:|---:|---|---:|---:|---|
| A | top acceleration | 181 | 3 | 1.66% | 0.0831% to 7.47% | 52, fails | 0, fails | holds |
| A | burnout | 210 | 12 | 5.71% | 1.81% to 12.9% | 56, fails | 0, fails | holds |
| B | top acceleration | 187 | 3 | 1.6% | 0.0805% to 7.24% | 44, fails | 0, fails | holds |
| B | burnout | 211 | 6 | 2.84% | 0.463% to 8.76% | 47, fails | 0, fails | holds |

### The top acceleration's lower ends past the clip

The same lower ends on each flight's log with its top as flown, past the clip by as much as its design takes it. How far is the design's, and no log says, so the test above is on the tops near the clip; here the band on the share below must reach down to 5%: they miss no more. Read whole are the tops with no lower end, out of each count: those near the clip, then those as flown.

| Set | Compared | Below | Share | Band | Read whole, near the clip | Read whole, as flown |
|---|---:|---:|---:|---|---:|---:|
| A | 252 | 0 | 0% | 0% to 2.97% | 175 | 104 |
| B | 285 | 0 | 0% | 0% to 2.63% | 181 | 83 |

### The floors by logger and by design

Floors compared, and truths below the lower end, in bold where that group alone would fail the floors' test. The test is on each set as a whole. The AltOS part has no 8 g setting: its log is written clipping there to reach the count, and its family states no range, so the reader tells the clip by its flat top.

| Set | Reading | Group | Compared | Below |
|---|---|---|---:|---:|
| A | top acceleration | altos | 74 | 3 |
| A | top acceleration | imu-500hz | 107 | 0 |
| A | top acceleration | rocketpy-bella-lui | 15 | 0 |
| A | top acceleration | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 18 | 1 |
| A | top acceleration | rocketpy-calisto-tests-motor-at-minus-1.373 | 21 | 0 |
| A | top acceleration | rocketpy-cavour | 18 | 1 |
| A | top acceleration | rocketpy-genesis | 21 | 0 |
| A | top acceleration | rocketpy-juno-iii | 19 | 0 |
| A | top acceleration | rocketpy-lince | 19 | 0 |
| A | top acceleration | rocketpy-ndrt-2020-nose-to-tail | 18 | 0 |
| A | top acceleration | rocketpy-prometheus-2022-generic-motor | 16 | 0 |
| A | top acceleration | rocketpy-valetudo | 16 | 1 |
| A | burnout | altos | 45 | 6 |
| A | burnout | eggtimer | 68 | 1 |
| A | burnout | perfectflite | 50 | 3 |
| A | burnout | rrc3 | 47 | 2 |
| A | burnout | rocketpy-bella-lui | 61 | 9 |
| A | burnout | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 | 0 |
| A | burnout | rocketpy-juno-iii | 20 | 0 |
| A | burnout | rocketpy-ndrt-2020-nose-to-tail | 58 | 1 |
| A | burnout | rocketpy-valetudo | 67 | 2 |
| B | top acceleration | altos | 79 | 3 |
| B | top acceleration | imu-500hz | 108 | 0 |
| B | top acceleration | rocketpy-bella-lui | 17 | 1 |
| B | top acceleration | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 24 | 0 |
| B | top acceleration | rocketpy-calisto-tests-motor-at-minus-1.373 | 20 | 0 |
| B | top acceleration | rocketpy-cavour | 27 | 1 |
| B | top acceleration | rocketpy-genesis | 20 | 0 |
| B | top acceleration | rocketpy-juno-iii | 20 | 0 |
| B | top acceleration | rocketpy-lince | 16 | 0 |
| B | top acceleration | rocketpy-ndrt-2020-nose-to-tail | 14 | 1 |
| B | top acceleration | rocketpy-prometheus-2022-generic-motor | 16 | 0 |
| B | top acceleration | rocketpy-valetudo | 13 | 0 |
| B | burnout | altos | 37 | 3 |
| B | burnout | eggtimer | 69 | 2 |
| B | burnout | perfectflite | 42 | 0 |
| B | burnout | rrc3 | 63 | 1 |
| B | burnout | rocketpy-bella-lui | 63 | 4 |
| B | burnout | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 | 0 |
| B | burnout | rocketpy-calisto-tests-motor-at-minus-1.373 | 3 | 0 |
| B | burnout | rocketpy-juno-iii | 28 | 0 |
| B | burnout | rocketpy-ndrt-2020-nose-to-tail | 43 | 1 |
| B | burnout | rocketpy-valetudo | 70 | 1 |

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
| A | top acceleration | 24 / 292 / 40, fails | 1 / 350 / 5, fails | 0 / 258 / 98, fails |
| A | burnout | 23 / 255 / 31, fails | 4 / 305 / 0, fails | 0 / 166 / 143, fails |
| A | drogue rate | 66 / 309 / 53, fails | 0 / 428 / 0, fails | 0 / 285 / 143, fails |
| A | main opening | 70 / 301 / 57, fails | 6 / 420 / 2, fails | 1 / 327 / 100, fails |
| A | main rate | 111 / 607 / 102, fails | 0 / 820 / 0, fails | 0 / 545 / 275, fails |
| B | liftoff | 80 / 657 / 108, fails | 26 / 807 / 12, fails | 10 / 474 / 361, fails |
| B | apogee height | 137 / 581 / 127, fails | 0 / 845 / 0, fails | 0 / 562 / 283, fails |
| B | apogee time | 68 / 691 / 86, fails | 3 / 815 / 27, fails | 1 / 679 / 165, fails |
| B | top speed | 33 / 224 / 29, fails | 3 / 275 / 8, fails | 0 / 204 / 82, fails |
| B | landing | 30 / 687 / 128, fails | 17 / 827 / 1, fails | 4 / 353 / 488, fails |
| B | flight time | 76 / 681 / 88, fails | 12 / 817 / 16, fails | 3 / 650 / 192, fails |
| B | descent rate | 111 / 625 / 109, fails | 0 / 841 / 4, fails | 0 / 580 / 265, fails |
| B | top acceleration | 32 / 302 / 34, fails | 1 / 360 / 7, fails | 0 / 276 / 92, fails |
| B | burnout | 33 / 256 / 38, fails | 13 / 312 / 2, fails | 1 / 179 / 147, fails |
| B | drogue rate | 81 / 295 / 60, fails | 0 / 436 / 0, fails | 0 / 292 / 144, fails |
| B | main opening | 52 / 323 / 61, fails | 3 / 432 / 1, fails | 0 / 333 / 103, fails |
| B | main rate | 125 / 604 / 116, fails | 0 / 842 / 3, fails | 0 / 573 / 272, fails |

### Each reading's error side

Each reading's mean error, the reading less the truth, over the readings compared, with the largest error's size and the share of truths above the interval (ADR-257 §2). For the apogee's height, the top speed, the top acceleration, the mean descent rate and each canopy's, a reading too low is the flattering side: a waiver's height, a fin's margin (the top speed is vertical), the load the airframe took, a descent gentler than it was. The share of truths above the interval is that side's test, and the shifted-down probe its mutation probe; the times have none here. The mean is given at the figures of the largest error, the digits every platform shares, and with its sign: +0 is a mean too small for those figures that isn't below zero.

| Set | Reading | Flattering side | Mean error | Largest error | Truths above |
|---|---|---|---:|---:|---:|
| A | liftoff | none | +0.243 s | 0.471 s | 6.83% |
| A | apogee height | low | +1 m | 237 m | 5.49% |
| A | apogee time | none | +0.006 s | 0.449 s | 3.9% |
| A | top speed | low | +1.2 m/s | 20.4 m/s | 6.27% |
| A | landing | none | -0.38 s | 1.33 s | 4.39% |
| A | flight time | none | -0.62 s | 1.56 s | 3.9% |
| A | descent rate | low | -0.01 m/s | 6.81 m/s | 5.49% |
| A | top acceleration | low | -0 m/s² | 14.3 m/s² | 6.18% |
| A | burnout | none | -0.0147 s | 0.0893 s | 3.24% |
| A | drogue rate | low | +0.02 m/s | 1.51 m/s | 3.74% |
| A | main opening | none | +0.416 s | 0.784 s | 5.37% |
| A | main rate | low | -0 m/s | 5.8 m/s | 4.88% |
| B | liftoff | none | +0.247 s | 0.554 s | 6.39% |
| B | apogee height | low | +0 m | 234 m | 5.56% |
| B | apogee time | none | +0.006 s | 0.526 s | 5.92% |
| B | top speed | low | +1.3 m/s | 26.8 m/s | 4.9% |
| B | landing | none | -0.37 s | 1.23 s | 4.02% |
| B | flight time | none | -0.62 s | 1.5 s | 5.33% |
| B | descent rate | low | -0.01 m/s | 6.34 m/s | 5.92% |
| B | top acceleration | low | +0.4 m/s² | 15.7 m/s² | 5.43% |
| B | burnout | none | -0.015 s | 0.104 s | 5.2% |
| B | drogue rate | low | -0 m/s | 1.5 m/s | 4.36% |
| B | main opening | none | +0.413 s | 0.869 s | 6.65% |
| B | main rate | low | +0.01 m/s | 5.87 m/s | 5.56% |

### By design

Below / inside / above.

Set A:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 2 / 72 / 2 | 1 / 75 / 0 | 3 / 72 / 1 | 1 / 75 / 0 | 0 / 69 / 7 | 0 / 67 / 9 | 0 / 76 / 0 | 0 / 39 / 0 | 0 / 15 / 0 | none | none | 1 / 73 / 2 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 80 / 2 | 8 / 72 / 2 | 5 / 75 / 2 | 0 / 2 / 3 | 0 / 82 / 0 | 3 / 79 / 0 | 9 / 71 / 2 | 4 / 28 / 2 | 0 / 34 / 0 | 9 / 71 / 2 | 0 / 82 / 0 | 7 / 73 / 2 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 75 / 7 | 2 / 76 / 4 | 1 / 77 / 4 | 0 / 0 / 2 | 0 / 81 / 1 | 2 / 80 / 0 | 3 / 76 / 3 | 0 / 34 / 3 | 0 / 37 / 0 | 3 / 77 / 2 | 1 / 81 / 0 | 2 / 79 / 1 |
| rocketpy-cavour | 0 / 64 / 21 | 7 / 72 / 6 | 8 / 71 / 6 | none | 38 / 47 / 0 | 21 / 64 / 0 | 1 / 79 / 5 | 3 / 29 / 1 | 2 / 27 / 4 | none | none | 7 / 71 / 7 |
| rocketpy-genesis | 0 / 77 / 12 | 8 / 76 / 5 | 4 / 84 / 1 | none | 0 / 87 / 2 | 3 / 86 / 0 | 10 / 70 / 9 | 2 / 36 / 1 | 2 / 37 / 0 | 8 / 76 / 5 | 0 / 66 / 23 | 8 / 77 / 4 |
| rocketpy-juno-iii | 22 / 54 / 0 | 3 / 65 / 8 | 2 / 73 / 1 | 2 / 22 / 6 | 0 / 76 / 0 | 0 / 67 / 9 | 4 / 64 / 8 | 1 / 30 / 0 | 0 / 26 / 0 | none | none | 4 / 63 / 9 |
| rocketpy-lince | 0 / 65 / 8 | 9 / 56 / 8 | 4 / 65 / 4 | none | 0 / 73 / 0 | 8 / 65 / 0 | 11 / 51 / 11 | 1 / 29 / 3 | 1 / 27 / 5 | none | none | 10 / 53 / 10 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 88 / 4 | 4 / 83 / 5 | 2 / 87 / 3 | 0 / 91 / 1 | 0 / 69 / 23 | 0 / 91 / 1 | 0 / 92 / 0 | 1 / 26 / 7 | 0 / 33 / 1 | 4 / 86 / 2 | 14 / 78 / 0 | 0 / 92 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 9 / 68 / 6 | 6 / 69 / 8 | none | 0 / 83 / 0 | 1 / 82 / 0 | 9 / 67 / 7 | 0 / 38 / 5 | 5 / 38 / 0 | 9 / 69 / 5 | 7 / 76 / 0 | 2 / 80 / 1 |
| rocketpy-valetudo | 10 / 72 / 0 | 4 / 77 / 1 | 1 / 79 / 2 | 8 / 68 / 6 | 0 / 79 / 3 | 0 / 69 / 13 | 0 / 82 / 0 | 0 / 33 / 0 | 0 / 15 / 0 | none | none | 5 / 73 / 4 |

Set B:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 9 / 77 / 1 | 2 / 84 / 1 | 0 / 84 / 3 | 0 / 84 / 3 | 0 / 80 / 7 | 0 / 70 / 17 | 0 / 87 / 0 | 1 / 40 / 0 | 0 / 24 / 0 | none | none | 2 / 82 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 81 / 10 | 5 / 81 / 5 | 2 / 82 / 7 | 0 / 2 / 3 | 0 / 90 / 1 | 5 / 86 / 0 | 7 / 79 / 5 | 2 / 39 / 5 | 0 / 46 / 0 | 5 / 82 / 4 | 1 / 90 / 0 | 5 / 83 / 3 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 94 / 8 | 3 / 94 / 5 | 6 / 89 / 7 | 0 / 4 / 2 | 0 / 102 / 0 | 1 / 101 / 0 | 7 / 91 / 4 | 2 / 35 / 4 | 0 / 41 / 0 | 3 / 96 / 3 | 1 / 101 / 0 | 2 / 97 / 3 |
| rocketpy-cavour | 0 / 64 / 14 | 2 / 71 / 5 | 2 / 66 / 10 | none | 23 / 55 / 0 | 16 / 62 / 0 | 1 / 71 / 6 | 4 / 38 / 1 | 9 / 28 / 6 | none | none | 5 / 65 / 8 |
| rocketpy-genesis | 0 / 85 / 7 | 5 / 85 / 2 | 4 / 83 / 5 | none | 0 / 92 / 0 | 1 / 91 / 0 | 7 / 81 / 4 | 3 / 33 / 3 | 7 / 32 / 0 | 5 / 86 / 1 | 0 / 63 / 29 | 5 / 85 / 2 |
| rocketpy-juno-iii | 29 / 55 / 0 | 6 / 71 / 7 | 7 / 73 / 4 | 2 / 30 / 5 | 0 / 80 / 4 | 0 / 67 / 17 | 7 / 66 / 11 | 0 / 36 / 2 | 0 / 28 / 0 | none | none | 7 / 65 / 12 |
| rocketpy-lince | 0 / 66 / 11 | 2 / 68 / 7 | 7 / 64 / 6 | none | 0 / 76 / 1 | 13 / 64 / 0 | 6 / 59 / 12 | 1 / 28 / 3 | 4 / 17 / 11 | none | none | 5 / 61 / 11 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 65 / 3 | 5 / 57 / 6 | 2 / 64 / 2 | 0 / 68 / 0 | 0 / 52 / 16 | 0 / 68 / 0 | 0 / 68 / 0 | 4 / 20 / 1 | 0 / 25 / 0 | 5 / 58 / 5 | 3 / 65 / 0 | 0 / 68 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 3 / 72 / 8 | 7 / 74 / 2 | none | 0 / 83 / 0 | 1 / 82 / 0 | 6 / 69 / 8 | 1 / 34 / 1 | 5 / 31 / 0 | 6 / 71 / 6 | 8 / 75 / 0 | 2 / 79 / 2 |
| rocketpy-valetudo | 13 / 70 / 0 | 5 / 77 / 1 | 4 / 75 / 4 | 7 / 75 / 1 | 0 / 78 / 5 | 0 / 72 / 11 | 0 / 83 / 0 | 0 / 27 / 0 | 0 / 13 / 0 | none | none | 7 / 73 / 3 |

### By logger and by the day's air

Below / inside / above, and in bold where that group's counts alone would fail the test. The test is on each set as a whole; these groups show where its misses fall. A barometric height is off by about the day's offset from the standard atmosphere over its temperature, so the misses of the height, the top speed and the rates of descent gather on the coldest and warmest days: a warm day reads low, and its truth lies above the interval.

#### By logger

Set A:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | **0 / 126 / 18** | 10 / 125 / 9 | 10 / 134 / 0 | 1 / 45 / 4 | 11 / 130 / 3 | 9 / 132 / 3 | 8 / 131 / 5 | none | none | 6 / 58 / 2 | 1 / 63 / 2 | 9 / 128 / 7 |
| rrc3 | 1 / 125 / 13 | 8 / 124 / 7 | 1 / 135 / 3 | 2 / 42 / 3 | 6 / 125 / 8 | 3 / 132 / 4 | 8 / 123 / 8 | none | none | 6 / 72 / 3 | 2 / 75 / 4 | 9 / 125 / 5 |
| eggtimer | **3 / 155 / 23** | 15 / 158 / 8 | 18 / 161 / 2 | 3 / 59 / 6 | 10 / 165 / 6 | 12 / 167 / 2 | 8 / 166 / 7 | none | none | 6 / 85 / 3 | 9 / 80 / 5 | 9 / 162 / 10 |
| altos | 17 / 166 / 2 | 11 / 162 / 12 | **2 / 156 / 27** | 3 / 62 / 2 | 11 / 171 / 3 | 13 / 171 / 1 | 13 / 158 / 14 | **12 / 151 / 22** | 0 / 129 / 9 | 8 / 78 / 3 | 2 / 79 / 8 | 10 / 166 / 9 |
| imu-500hz | **14 / 157 / 0** | 11 / 151 / 9 | **5 / 166 / 0** | 2 / 50 / 3 | **0 / 155 / 16** | **1 / 148 / 22** | 10 / 150 / 11 | **0 / 171 / 0** | 10 / 160 / 1 | 7 / 86 / 5 | 8 / 86 / 4 | 9 / 153 / 9 |

Set B:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | 3 / 118 / 11 | 5 / 121 / 6 | 9 / 119 / 4 | 0 / 39 / 3 | 5 / 125 / 2 | 7 / 122 / 3 | 5 / 120 / 7 | none | none | 4 / 69 / 2 | 1 / 71 / 3 | 4 / 120 / 8 |
| rrc3 | 6 / 153 / 17 | 5 / 163 / 8 | 7 / 163 / 6 | 2 / 60 / 1 | 5 / 163 / 8 | 4 / 164 / 8 | 6 / 163 / 7 | none | none | 2 / 83 / 5 | 3 / 79 / 8 | 7 / 163 / 6 |
| eggtimer | **3 / 143 / 23** | 12 / 147 / 10 | 15 / 146 / 8 | 2 / 64 / 3 | 5 / 159 / 5 | 10 / 157 / 2 | 11 / 149 / 9 | none | none | 9 / 71 / 4 | 2 / 79 / 3 | 9 / 152 / 8 |
| altos | **23 / 160 / 3** | 4 / 168 / 14 | **4 / 152 / 30** | 4 / 49 / 4 | 8 / 176 / 2 | 14 / 170 / 2 | 8 / 158 / 20 | **18 / 148 / 20** | 0 / 128 / 17 | 2 / 84 / 5 | 5 / 78 / 8 | 7 / 162 / 17 |
| imu-500hz | **17 / 165 / 0** | 12 / 161 / 9 | 6 / 174 / 2 | 1 / 51 / 3 | **0 / 165 / 17** | **2 / 150 / 30** | 11 / 164 / 7 | **0 / 182 / 0** | **25 / 157 / 0** | 7 / 86 / 3 | 2 / 87 / 7 | 13 / 161 / 8 |

#### By the day's air

Set A:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 3 / 115 / 14 | **55 / 77 / 0** | 10 / 114 / 8 | **10 / 24 / 0** | 7 / 120 / 5 | 10 / 118 / 4 | **47 / 85 / 0** | 1 / 48 / 3 | 2 / 41 / 2 | **33 / 39 / 0** | 2 / 66 / 4 | **46 / 86 / 0** |
| −10 to +10 K | 27 / 489 / 35 | **0 / 551 / 0** | 21 / 510 / 20 | **1 / 194 / 1** | 26 / 500 / 25 | 23 / 505 / 23 | **0 / 551 / 0** | 10 / 217 / 14 | 6 / 193 / 6 | **0 / 287 / 0** | 14 / 260 / 13 | **0 / 551 / 0** |
| warmer than +10 K | 5 / 125 / 7 | **0 / 92 / 45** | 5 / 128 / 4 | **0 / 40 / 17** | 5 / 126 / 6 | 5 / 127 / 5 | **0 / 92 / 45** | 1 / 57 / 5 | 2 / 55 / 2 | **0 / 53 / 16** | 6 / 57 / 6 | **0 / 97 / 40** |

Set B:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 14 / 140 / 9 | **38 / 125 / 0** | 12 / 143 / 8 | 7 / 45 / 0 | 3 / 151 / 9 | 5 / 148 / 10 | **41 / 122 / 0** | 3 / 64 / 5 | 5 / 56 / 2 | **24 / 64 / 0** | 2 / 77 / 9 | **40 / 123 / 0** |
| −10 to +10 K | 30 / 457 / 39 | **0 / 526 / 0** | 23 / 468 / 35 | **2 / 163 / 0** | 16 / 490 / 20 | 24 / 473 / 29 | **0 / 526 / 0** | 12 / 205 / 11 | 15 / 177 / 9 | **0 / 268 / 0** | 9 / 243 / 16 | **0 / 526 / 0** |
| warmer than +10 K | 8 / 142 / 6 | **0 / 109 / 47** | 6 / 143 / 7 | **0 / 55 / 14** | 4 / 147 / 5 | 8 / 142 / 6 | **0 / 106 / 50** | 3 / 61 / 4 | 5 / 52 / 6 | **0 / 61 / 19** | 2 / 74 / 4 | **0 / 109 / 47** |
