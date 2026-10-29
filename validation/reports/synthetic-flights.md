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

Each flight's own log read by `hpr_flightdata::readings`, and each reading that states a 90% interval compared with the flight's truth (M7.2e1, ADR-257 §1–2, ADR-308). An interval runs from the reading less its 95% share of its bound to the reading less its 5% share; the shares are the 5% and 95% points of the readings' errors as shares of their bounds on the calibration set's 2000 flights, a seed of their own that the test sets never share. `cargo xtask synthetic` fits them again and fails unless they equal the library's to three significant figures. A reading **holds** when the two-sided 99.9% Clopper–Pearson band on the share of truths below its interval and the band on the share above each hold 5%, and the band on the share inside holds 90%; and every probe must fail that test. A floor states no interval and is counted apart.

### The shares

| Reading | Readings fitted | 5% share | 95% share |
|---|---:|---:|---:|
| apogee height | 1638 | -0.63 | 0.637 |
| apogee time | 1638 | -0.337 | 0.225 |

### The test

The truths below, inside and above each interval, then the shares, each set's flights flown. Floors and withheld readings aren't compared.

| Set | Reading | Compared | Below / inside / above | Below | Above | Floors | Withheld | Result |
|---|---|---:|---:|---:|---:|---:|---:|---|
| A | apogee height | 820 | 55 / 720 / 45 | 6.71% | 5.49% | 0 | 180 | holds |
| A | apogee time | 820 | 36 / 752 / 32 | 4.39% | 3.9% | 0 | 180 | holds |
| B | apogee height | 845 | 38 / 760 / 47 | 4.5% | 5.56% | 0 | 155 | holds |
| B | apogee time | 845 | 41 / 754 / 50 | 4.85% | 5.92% | 0 | 155 | holds |

### The probes

Each interval narrowed to 0.8 of its width about its middle, widened to 1.25, and shifted down by one stated standard error (its width over 2 × 1.645): below / inside / above. Each must fail the test.

| Set | Reading | narrowed to 0.8 | widened to 1.25 | shifted down one standard error |
|---|---|---:|---:|---:|
| A | apogee height | 115 / 592 / 113, fails | 0 / 820 / 0, fails | 0 / 537 / 283, fails |
| A | apogee time | 61 / 690 / 69, fails | 5 / 803 / 12, fails | 1 / 680 / 139, fails |
| B | apogee height | 137 / 581 / 127, fails | 0 / 845 / 0, fails | 0 / 562 / 283, fails |
| B | apogee time | 68 / 691 / 86, fails | 3 / 815 / 27, fails | 1 / 679 / 165, fails |

### The apogee's error side

The apogee height's mean error, the reading less the truth, over the readings compared (ADR-257 §2). A reading too low is the flattering side for a waiver: the share of truths above the interval is that side's test, and the shifted-down probe its mutation probe.

| Set | Mean error, m | Truths above |
|---|---:|---:|
| A | 1.07 | 5.49% |
| B | 0.0742 | 5.56% |

### By design

Below / inside / above.

| Design | A, apogee height | A, apogee time | B, apogee height | B, apogee time |
|---|---:|---:|---:|---:|
| rocketpy-bella-lui | 1 / 75 / 0 | 3 / 72 / 1 | 2 / 84 / 1 | 0 / 84 / 3 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 8 / 72 / 2 | 5 / 75 / 2 | 5 / 81 / 5 | 2 / 82 / 7 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 2 / 76 / 4 | 1 / 77 / 4 | 3 / 94 / 5 | 6 / 89 / 7 |
| rocketpy-cavour | 7 / 72 / 6 | 8 / 71 / 6 | 2 / 71 / 5 | 2 / 66 / 10 |
| rocketpy-genesis | 8 / 76 / 5 | 4 / 84 / 1 | 5 / 85 / 2 | 4 / 83 / 5 |
| rocketpy-juno-iii | 3 / 65 / 8 | 2 / 73 / 1 | 6 / 71 / 7 | 7 / 73 / 4 |
| rocketpy-lince | 9 / 56 / 8 | 4 / 65 / 4 | 2 / 68 / 7 | 7 / 64 / 6 |
| rocketpy-ndrt-2020-nose-to-tail | 4 / 83 / 5 | 2 / 87 / 3 | 5 / 57 / 6 | 2 / 64 / 2 |
| rocketpy-prometheus-2022-generic-motor | 9 / 68 / 6 | 6 / 69 / 8 | 3 / 72 / 8 | 7 / 74 / 2 |
| rocketpy-valetudo | 4 / 77 / 1 | 1 / 79 / 2 | 5 / 77 / 1 | 4 / 75 / 4 |
