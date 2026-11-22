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
| Apogee above the pad, m | 472 / 2850 / 4420 | 411 / 2800 / 4740 | 440 / 2800 / 4810 |
| Top Mach number | 0.28 / 0.752 / 1.12 | 0.269 / 0.738 / 1.18 | 0.27 / 0.747 / 1.15 |
| Top vertical speed, m/s | 90.3 / 246 / 367 | 82.9 / 244 / 375 | 84.6 / 246 / 375 |
| Liftoff to landing, s | 36.2 / 170 / 611 | 32.1 / 161 / 620 | 33.3 / 171 / 606 |
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
| imu-500hz | 38, 0.504 | 38, 0.683 | 38, 0.267 | 14, 0.263 | 38, 0.382 | 38, 0.404 | 38, 0.553 |
| 500 Hz, altitude, no step | 171, 0.603 | 171, 0.711 | 171, 0.387 | 54, 0.296 | 170, 0.402 | 170, 0.481 | 170, 0.582 |
| 500 Hz, altitude in 0.3048 m steps | 171, 0.577 | 171, 0.709 | 171, 0.486 | 54, 0.296 | 171, 0.404 | 171, 0.474 | 171, 0.581 |
| 500 Hz, altitude in 10 m steps | 171, 0.599 | 171, 0.651 | 171, 0.127 | 54, 0.372 | 171, 0.431 | 171, 0.491 | 171, 0.471 |
| 500 Hz, pressure alone in 1 Pa steps | 171, 0.594 | 171, 0.711 | 171, 0.393 | 54, 0.296 | 171, 0.409 | 171, 0.486 | 171, 0.592 |
| 500 Hz, pressure alone in 10 Pa steps | 171, 0.554 | 171, 0.707 | 171, 0.618 | 54, 0.297 | 171, 0.476 | 171, 0.511 | 171, 0.573 |
| 500 Hz, pressure alone in 100 Pa steps | 171, 0.506 | 171, 0.653 | 171, 0.283 | 54, 0.327 | 171, 0.549 | 171, 0.505 | 171, 0.477 |
| 100 Hz, altitude, no step | 171, 0.597 | 171, 0.711 | 171, 0.595 | 54, 0.326 | 171, 0.451 | 171, 0.447 | 171, 0.587 |
| 100 Hz, altitude in 0.3048 m steps | 171, 0.576 | 171, 0.709 | 171, 0.567 | 54, 0.335 | 171, 0.43 | 171, 0.435 | 171, 0.578 |
| 100 Hz, altitude in 10 m steps | 171, 0.592 | 171, 0.651 | 171, 0.317 | 54, 0.354 | 171, 0.433 | 171, 0.493 | 171, 0.467 |
| 100 Hz, pressure alone in 1 Pa steps | 171, 0.589 | 171, 0.71 | 171, 0.583 | 54, 0.328 | 171, 0.444 | 171, 0.439 | 171, 0.583 |
| 100 Hz, pressure alone in 10 Pa steps | 171, 0.594 | 171, 0.707 | 171, 0.419 | 54, 0.335 | 171, 0.407 | 171, 0.462 | 171, 0.582 |
| 100 Hz, pressure alone in 100 Pa steps | 171, 0.487 | 171, 0.653 | 171, 0.588 | 53, 0.379 | 171, 0.538 | 171, 0.501 | 171, 0.474 |
| 20 Hz, altitude, no step | 171, 0.552 | 171, 0.717 | 171, 0.335 | 54, 0.353 | 170, 0.386 | 170, 0.376 | 170, 0.581 |
| 20 Hz, altitude in 0.3048 m steps | 171, 0.508 | 171, 0.715 | 171, 0.32 | 54, 0.354 | 171, 0.371 | 171, 0.379 | 171, 0.574 |
| 20 Hz, altitude in 10 m steps | 171, 0.56 | 171, 0.676 | 171, 0.482 | 52, 0.496 | 171, 0.425 | 171, 0.449 | 171, 0.471 |
| 20 Hz, pressure alone in 1 Pa steps | 171, 0.542 | 171, 0.716 | 171, 0.329 | 54, 0.355 | 171, 0.372 | 171, 0.37 | 171, 0.579 |
| 20 Hz, pressure alone in 10 Pa steps | 171, 0.492 | 171, 0.708 | 171, 0.653 | 54, 0.328 | 171, 0.406 | 171, 0.408 | 171, 0.575 |
| 20 Hz, pressure alone in 100 Pa steps | 171, 0.48 | 171, 0.653 | 171, 0.562 | 53, 0.547 | 171, 0.496 | 171, 0.469 | 171, 0.475 |
| 5 Hz, altitude, no step | 171, 0.372 | 171, 0.714 | 171, 0.538 | 0 | 170, 0.408 | 170, 0.304 | 170, 0.577 |
| 5 Hz, altitude in 0.3048 m steps | 171, 0.356 | 171, 0.712 | 171, 0.518 | 0 | 170, 0.405 | 170, 0.29 | 170, 0.571 |
| 5 Hz, altitude in 10 m steps | 171, 0.49 | 171, 0.675 | 171, 0.187 | 0 | 171, 0.408 | 171, 0.394 | 171, 0.464 |
| 5 Hz, pressure alone in 1 Pa steps | 171, 0.367 | 171, 0.713 | 171, 0.53 | 0 | 170, 0.407 | 170, 0.302 | 170, 0.575 |
| 5 Hz, pressure alone in 10 Pa steps | 171, 0.369 | 171, 0.707 | 171, 0.689 | 0 | 171, 0.399 | 171, 0.289 | 171, 0.563 |
| 5 Hz, pressure alone in 100 Pa steps | 171, 0.415 | 171, 0.652 | 171, 0.216 | 0 | 171, 0.572 | 171, 0.491 | 171, 0.492 |
| 1 Hz, altitude, no step | 171, 0.314 | 171, 0.7 | 171, 0.366 | 0 | 171, 0.47 | 171, 0.296 | 171, 0.516 |
| 1 Hz, altitude in 0.3048 m steps | 171, 0.301 | 171, 0.698 | 171, 0.358 | 0 | 171, 0.469 | 171, 0.294 | 171, 0.515 |
| 1 Hz, altitude in 10 m steps | 171, 0.208 | 171, 0.645 | 171, 0.248 | 0 | 171, 0.44 | 171, 0.304 | 171, 0.447 |
| 1 Hz, pressure alone in 1 Pa steps | 171, 0.304 | 171, 0.699 | 171, 0.362 | 0 | 171, 0.469 | 171, 0.295 | 171, 0.513 |
| 1 Hz, pressure alone in 10 Pa steps | 171, 0.291 | 171, 0.691 | 171, 0.334 | 0 | 171, 0.467 | 171, 0.288 | 171, 0.501 |
| 1 Hz, pressure alone in 100 Pa steps | 171, 0.212 | 171, 0.644 | 171, 0.258 | 0 | 171, 0.442 | 171, 0.296 | 171, 0.454 |
| 0.5 Hz, altitude, no step | 171, 0.385 | 171, 0.636 | 171, 0.422 | 0 | 152, 0.382 | 152, 0.347 | 152, 0.449 |
| 0.5 Hz, altitude in 0.3048 m steps | 171, 0.382 | 171, 0.635 | 171, 0.415 | 0 | 153, 0.378 | 153, 0.344 | 153, 0.447 |
| 0.5 Hz, altitude in 10 m steps | 170, 0.278 | 170, 0.589 | 170, 0.312 | 0 | 168, 0.334 | 168, 0.245 | 168, 0.382 |
| 0.5 Hz, pressure alone in 1 Pa steps | 171, 0.385 | 171, 0.636 | 171, 0.421 | 0 | 152, 0.382 | 152, 0.347 | 152, 0.449 |
| 0.5 Hz, pressure alone in 10 Pa steps | 171, 0.375 | 171, 0.632 | 171, 0.399 | 0 | 158, 0.352 | 158, 0.307 | 158, 0.435 |
| 0.5 Hz, pressure alone in 100 Pa steps | 170, 0.275 | 170, 0.589 | 170, 0.31 | 0 | 167, 0.335 | 167, 0.246 | 167, 0.382 |
| its accelerometer at 500 Hz | 71, 0.575 | 71, 0.689 | 71, 0.571 | 23, 0.328 | 71, 0.538 | 71, 0.404 | 71, 0.582 |
| its accelerometer at 100 Hz | 71, 0.575 | 71, 0.689 | 71, 0.571 | 23, 0.328 | 71, 0.538 | 71, 0.404 | 71, 0.582 |
| its accelerometer at 20 Hz | 71, 0.575 | 71, 0.689 | 71, 0.571 | 23, 0.328 | 71, 0.538 | 71, 0.404 | 71, 0.582 |
| its accelerometer set to ±8 g | 38, 0.504 | 38, 0.683 | 38, 0.267 | 14, 0.263 | 38, 0.382 | 38, 0.404 | 38, 0.553 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | 171, 0.555 | 171, 0.711 | 171, 0.32 | 54, 0.354 | 171, 0.414 | 171, 0.431 | 171, 0.575 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | 135, 0.381 | 171, 0.693 | 171, 0.32 | 54, 0.354 | 171, 0.245 | 135, 0.28 | 171, 0.549 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | 0 | 171, 0.672 | 171, 0.32 | 54, 0.354 | 171, 0.172 | 0 | 171, 0.51 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | 168, 0.555 | 168, 0.707 | 168, 0.653 | 54, 0.328 | 168, 0.436 | 168, 0.435 | 168, 0.578 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| altos | 33, 0.575 | 33, 0.689 | 33, 0.571 | 9, 0.328 | 33, 0.538 | 33, 0.337 | 33, 0.582 |
| gnss-tracker | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| eggtimer | 39, 0.508 | 39, 0.686 | 39, 0.294 | 14, 0.296 | 39, 0.371 | 39, 0.37 | 39, 0.542 |
| perfectflite | 31, 0.493 | 31, 0.715 | 31, 0.298 | 10, 0.314 | 31, 0.351 | 31, 0.362 | 31, 0.574 |
| rrc3 | 30, 0.418 | 30, 0.631 | 30, 0.416 | 7, 0.193 | 30, 0.364 | 30, 0.368 | 30, 0.506 |

### The accelerometer's readings and the canopies'

| Log | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|
| imu-500hz | 38, 0.282 | 38, 0.0861 | 16, 0.661 | 16, 0.588 | 37, 0.627 |
| 500 Hz, altitude, no step | 0 | 54, 0.0424 | 87, 0.685 | 87, 0.621 | 170, 0.696 |
| 500 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0471 | 88, 0.682 | 87, 0.622 | 170, 0.695 |
| 500 Hz, altitude in 10 m steps | 0 | 54, 0.0566 | 87, 0.615 | 87, 0.57 | 171, 0.687 |
| 500 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0424 | 87, 0.685 | 87, 0.621 | 170, 0.697 |
| 500 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.044 | 87, 0.673 | 87, 0.628 | 171, 0.69 |
| 500 Hz, pressure alone in 100 Pa steps | 0 | 54, 0.0989 | 87, 0.628 | 87, 0.56 | 171, 0.638 |
| 100 Hz, altitude, no step | 0 | 54, 0.0115 | 87, 0.686 | 87, 0.627 | 170, 0.693 |
| 100 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0115 | 87, 0.684 | 87, 0.626 | 170, 0.689 |
| 100 Hz, altitude in 10 m steps | 0 | 54, 0.035 | 87, 0.614 | 87, 0.563 | 171, 0.648 |
| 100 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0115 | 87, 0.685 | 87, 0.639 | 170, 0.689 |
| 100 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.0115 | 87, 0.679 | 87, 0.626 | 171, 0.685 |
| 100 Hz, pressure alone in 100 Pa steps | 0 | 53, 0.125 | 87, 0.626 | 87, 0.569 | 171, 0.638 |
| 20 Hz, altitude, no step | 0 | 54, 0.0677 | 87, 0.694 | 87, 0.627 | 170, 0.692 |
| 20 Hz, altitude in 0.3048 m steps | 0 | 54, 0.0677 | 88, 0.691 | 87, 0.631 | 170, 0.69 |
| 20 Hz, altitude in 10 m steps | 0 | 52, 0.366 | 87, 0.617 | 87, 0.566 | 171, 0.648 |
| 20 Hz, pressure alone in 1 Pa steps | 0 | 54, 0.0677 | 88, 0.693 | 87, 0.627 | 170, 0.691 |
| 20 Hz, pressure alone in 10 Pa steps | 0 | 54, 0.0677 | 87, 0.686 | 87, 0.636 | 171, 0.688 |
| 20 Hz, pressure alone in 100 Pa steps | 0 | 53, 0.544 | 87, 0.625 | 87, 0.565 | 171, 0.638 |
| 5 Hz, altitude, no step | 0 | 0 | 87, 0.701 | 87, 0.565 | 170, 0.694 |
| 5 Hz, altitude in 0.3048 m steps | 0 | 0 | 87, 0.697 | 87, 0.565 | 170, 0.692 |
| 5 Hz, altitude in 10 m steps | 0 | 0 | 87, 0.621 | 87, 0.556 | 171, 0.647 |
| 5 Hz, pressure alone in 1 Pa steps | 0 | 0 | 87, 0.699 | 87, 0.565 | 170, 0.693 |
| 5 Hz, pressure alone in 10 Pa steps | 0 | 0 | 87, 0.694 | 87, 0.833 | 171, 0.691 |
| 5 Hz, pressure alone in 100 Pa steps | 0 | 0 | 87, 0.621 | 87, 0.562 | 171, 0.637 |
| 1 Hz, altitude, no step | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, altitude in 0.3048 m steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, altitude in 10 m steps | 0 | 0 | 76, 0.593 | 76, 0.618 | 96, 0.582 |
| 1 Hz, pressure alone in 1 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, pressure alone in 10 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 1 Hz, pressure alone in 100 Pa steps | 0 | 0 | 75, 0.605 | 75, 0.617 | 95, 0.586 |
| 0.5 Hz, altitude, no step | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, altitude in 0.3048 m steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, altitude in 10 m steps | 0 | 0 | 46, 0.5 | 46, 0.586 | 46, 0.441 |
| 0.5 Hz, pressure alone in 1 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, pressure alone in 10 Pa steps | 0 | 0 | 0 | 0 | 0 |
| 0.5 Hz, pressure alone in 100 Pa steps | 0 | 0 | 0 | 0 | 0 |
| its accelerometer at 500 Hz | 71, 0.58 | 71, 0.0861 | 36, 0.661 | 36, 0.621 | 70, 0.675 |
| its accelerometer at 100 Hz | 71, 0.69 | 69, 0.172 | 36, 0.661 | 36, 0.621 | 70, 0.675 |
| its accelerometer at 20 Hz | 71, 0.737 | 70, 0.262 | 36, 0.661 | 36, 0.621 | 70, 0.675 |
| its accelerometer set to ±8 g | 38, 0.283 | 38, 0.186 | 16, 0.661 | 16, 0.588 | 37, 0.627 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | 0 | 54, 0.0677 | 87, 0.691 | 87, 0.633 | 171, 0.691 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | 0 | 54, 0.0677 | 88, 0.691 | 87, 0.631 | 170, 0.69 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | 0 | 54, 0.0677 | 88, 0.691 | 87, 0.631 | 170, 0.69 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.3 s before liftoff | 0 | 54, 0.0677 | 86, 0.686 | 86, 0.636 | 168, 0.688 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.1 s after liftoff | 0 | 0 | 0 | 0 | 0 |
| 20 Hz, pressure alone in 10 Pa steps, from 0.5 s after liftoff, its zero first | 0 | 0 | 0 | 0 | 0 |
| altos | 33, 0.69 | 31, 0.0682 | 20, 0.649 | 20, 0.621 | 33, 0.675 |
| gnss-tracker | 0 | 0 | 0 | 0 | 0 |
| eggtimer | 0 | 14, 0 | 23, 0.639 | 23, 0.585 | 39, 0.678 |
| perfectflite | 0 | 10, 0 | 11, 0.691 | 11, 0.52 | 31, 0.69 |
| rrc3 | 0 | 7, 0 | 17, 0.598 | 17, 0.574 | 30, 0.616 |

### Readings withheld

Each reading withheld, by reason; beside them, a drogue or a main's opening that the truth has none of, on a flight that came down under one canopy or none, as none to read.

| Log | Reading | Reason | Flights |
|---|---|---|---:|
| imu-500hz | top speed | `transonic` | 24 |
| imu-500hz | drogue rate | `short_leg` | 1 |
| imu-500hz | drogue rate | none to read | 21 |
| imu-500hz | main opening | `short_leg` | 1 |
| imu-500hz | main opening | none to read | 21 |
| imu-500hz | main rate | `short_leg` | 1 |
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
| 500 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 83 |
| 500 Hz, altitude in 0.3048 m steps | main opening | `short_leg` | 1 |
| 500 Hz, altitude in 0.3048 m steps | main opening | none to read | 83 |
| 500 Hz, altitude in 0.3048 m steps | main rate | `short_leg` | 1 |
| 500 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 500 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, altitude in 10 m steps | burnout | `transonic` | 117 |
| 500 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 500 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 500 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 500 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 500 Hz, pressure alone in 1 Pa steps | drogue rate | `short_leg` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 500 Hz, pressure alone in 1 Pa steps | main opening | `short_leg` | 1 |
| 500 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 500 Hz, pressure alone in 1 Pa steps | main rate | `short_leg` | 1 |
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
| 100 Hz, altitude in 0.3048 m steps | drogue rate | `short_leg` | 1 |
| 100 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 83 |
| 100 Hz, altitude in 0.3048 m steps | main opening | `short_leg` | 1 |
| 100 Hz, altitude in 0.3048 m steps | main opening | none to read | 83 |
| 100 Hz, altitude in 0.3048 m steps | main rate | `short_leg` | 1 |
| 100 Hz, altitude in 10 m steps | top speed | `transonic` | 117 |
| 100 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, altitude in 10 m steps | burnout | `transonic` | 117 |
| 100 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 100 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 100 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 100 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 100 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 100 Hz, pressure alone in 1 Pa steps | drogue rate | `short_leg` | 1 |
| 100 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 100 Hz, pressure alone in 1 Pa steps | main opening | `short_leg` | 1 |
| 100 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 100 Hz, pressure alone in 1 Pa steps | main rate | `short_leg` | 1 |
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
| 20 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps | main opening | `short_leg` | 1 |
| 20 Hz, altitude in 0.3048 m steps | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps | main rate | `short_leg` | 1 |
| 20 Hz, altitude in 10 m steps | top speed | `transonic` | 119 |
| 20 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 10 m steps | burnout | `transonic` | 119 |
| 20 Hz, altitude in 10 m steps | drogue rate | none to read | 84 |
| 20 Hz, altitude in 10 m steps | main opening | none to read | 84 |
| 20 Hz, pressure alone in 1 Pa steps | top speed | `transonic` | 117 |
| 20 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, pressure alone in 1 Pa steps | burnout | `transonic` | 117 |
| 20 Hz, pressure alone in 1 Pa steps | drogue rate | none to read | 83 |
| 20 Hz, pressure alone in 1 Pa steps | main opening | `short_leg` | 1 |
| 20 Hz, pressure alone in 1 Pa steps | main opening | none to read | 83 |
| 20 Hz, pressure alone in 1 Pa steps | main rate | `short_leg` | 1 |
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
| 1 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude, no step | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | drogue rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | main opening | `sampled_too_slowly` | 171 |
| 1 Hz, altitude, no step | main rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude in 0.3048 m steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | drogue rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | main opening | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 0.3048 m steps | main rate | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 1 Hz, altitude in 10 m steps | burnout | `sampled_too_slowly` | 171 |
| 1 Hz, altitude in 10 m steps | drogue rate | `sampled_too_slowly` | 75 |
| 1 Hz, altitude in 10 m steps | drogue rate | none to read | 20 |
| 1 Hz, altitude in 10 m steps | main opening | `sampled_too_slowly` | 75 |
| 1 Hz, altitude in 10 m steps | main opening | none to read | 20 |
| 1 Hz, altitude in 10 m steps | main rate | `sampled_too_slowly` | 75 |
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
| 0.5 Hz, altitude, no step | landing | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | flight time | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | descent rate | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude, no step | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude, no step | drogue rate | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | drogue rate | `sampled_too_slowly` | 152 |
| 0.5 Hz, altitude, no step | main opening | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | main opening | `sampled_too_slowly` | 152 |
| 0.5 Hz, altitude, no step | main rate | `ends_before_landing` | 19 |
| 0.5 Hz, altitude, no step | main rate | `sampled_too_slowly` | 152 |
| 0.5 Hz, altitude in 0.3048 m steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | landing | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | flight time | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | descent rate | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, altitude in 0.3048 m steps | drogue rate | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | drogue rate | `sampled_too_slowly` | 153 |
| 0.5 Hz, altitude in 0.3048 m steps | main opening | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | main opening | `sampled_too_slowly` | 153 |
| 0.5 Hz, altitude in 0.3048 m steps | main rate | `ends_before_landing` | 18 |
| 0.5 Hz, altitude in 0.3048 m steps | main rate | `sampled_too_slowly` | 153 |
| 0.5 Hz, altitude in 10 m steps | liftoff | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | apogee height | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | apogee time | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | top speed | `sampled_too_slowly` | 170 |
| 0.5 Hz, altitude in 10 m steps | top speed | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | landing | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | landing | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | flight time | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | flight time | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | descent rate | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | descent rate | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, altitude in 10 m steps | burnout | `sampled_too_slowly` | 170 |
| 0.5 Hz, altitude in 10 m steps | burnout | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | drogue rate | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | main opening | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | main opening | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | main opening | `step_too_coarse` | 1 |
| 0.5 Hz, altitude in 10 m steps | main rate | `ends_before_landing` | 2 |
| 0.5 Hz, altitude in 10 m steps | main rate | `sampled_too_slowly` | 122 |
| 0.5 Hz, altitude in 10 m steps | main rate | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 1 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | landing | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | flight time | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | descent rate | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 1 Pa steps | drogue rate | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | drogue rate | `sampled_too_slowly` | 152 |
| 0.5 Hz, pressure alone in 1 Pa steps | main opening | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | main opening | `sampled_too_slowly` | 152 |
| 0.5 Hz, pressure alone in 1 Pa steps | main rate | `ends_before_landing` | 19 |
| 0.5 Hz, pressure alone in 1 Pa steps | main rate | `sampled_too_slowly` | 152 |
| 0.5 Hz, pressure alone in 10 Pa steps | top speed | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | landing | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | flight time | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | descent rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | burnout | `sampled_too_slowly` | 171 |
| 0.5 Hz, pressure alone in 10 Pa steps | drogue rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | drogue rate | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 10 Pa steps | main opening | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | main opening | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 10 Pa steps | main rate | `ends_before_landing` | 13 |
| 0.5 Hz, pressure alone in 10 Pa steps | main rate | `sampled_too_slowly` | 158 |
| 0.5 Hz, pressure alone in 100 Pa steps | liftoff | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | apogee height | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | apogee time | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | top speed | `sampled_too_slowly` | 170 |
| 0.5 Hz, pressure alone in 100 Pa steps | top speed | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | landing | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | landing | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | flight time | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | flight time | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | descent rate | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | descent rate | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | top acceleration | `no_accelerometer` | 171 |
| 0.5 Hz, pressure alone in 100 Pa steps | burnout | `sampled_too_slowly` | 170 |
| 0.5 Hz, pressure alone in 100 Pa steps | burnout | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | drogue rate | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | drogue rate | `sampled_too_slowly` | 167 |
| 0.5 Hz, pressure alone in 100 Pa steps | drogue rate | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | main opening | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | main opening | `sampled_too_slowly` | 167 |
| 0.5 Hz, pressure alone in 100 Pa steps | main opening | `step_too_coarse` | 1 |
| 0.5 Hz, pressure alone in 100 Pa steps | main rate | `ends_before_landing` | 3 |
| 0.5 Hz, pressure alone in 100 Pa steps | main rate | `sampled_too_slowly` | 167 |
| 0.5 Hz, pressure alone in 100 Pa steps | main rate | `step_too_coarse` | 1 |
| its accelerometer at 500 Hz | top speed | `transonic` | 48 |
| its accelerometer at 500 Hz | drogue rate | `short_leg` | 1 |
| its accelerometer at 500 Hz | drogue rate | none to read | 34 |
| its accelerometer at 500 Hz | main opening | `short_leg` | 1 |
| its accelerometer at 500 Hz | main opening | none to read | 34 |
| its accelerometer at 500 Hz | main rate | `short_leg` | 1 |
| its accelerometer at 100 Hz | top speed | `transonic` | 48 |
| its accelerometer at 100 Hz | burnout | `transonic` | 2 |
| its accelerometer at 100 Hz | drogue rate | `short_leg` | 1 |
| its accelerometer at 100 Hz | drogue rate | none to read | 34 |
| its accelerometer at 100 Hz | main opening | `short_leg` | 1 |
| its accelerometer at 100 Hz | main opening | none to read | 34 |
| its accelerometer at 100 Hz | main rate | `short_leg` | 1 |
| its accelerometer at 20 Hz | top speed | `transonic` | 48 |
| its accelerometer at 20 Hz | burnout | `transonic` | 1 |
| its accelerometer at 20 Hz | drogue rate | `short_leg` | 1 |
| its accelerometer at 20 Hz | drogue rate | none to read | 34 |
| its accelerometer at 20 Hz | main opening | `short_leg` | 1 |
| its accelerometer at 20 Hz | main opening | none to read | 34 |
| its accelerometer at 20 Hz | main rate | `short_leg` | 1 |
| its accelerometer set to ±8 g | top speed | `transonic` | 24 |
| its accelerometer set to ±8 g | drogue rate | `short_leg` | 1 |
| its accelerometer set to ±8 g | drogue rate | none to read | 21 |
| its accelerometer set to ±8 g | main opening | `short_leg` | 1 |
| its accelerometer set to ±8 g | main opening | none to read | 21 |
| its accelerometer set to ±8 g | main rate | `short_leg` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | drogue rate | none to read | 84 |
| 20 Hz, altitude in 0.3048 m steps, from 0.3 s before liftoff | main opening | none to read | 84 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | liftoff | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | flight time | `short_pad` | 36 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main opening | `short_leg` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.1 s after liftoff | main rate | `short_leg` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | liftoff | `short_pad` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | top speed | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | flight time | `short_pad` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | top acceleration | `no_accelerometer` | 171 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | burnout | `transonic` | 117 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | drogue rate | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main opening | `short_leg` | 1 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main opening | none to read | 83 |
| 20 Hz, altitude in 0.3048 m steps, from 0.5 s after liftoff, its zero first | main rate | `short_leg` | 1 |
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
| altos | burnout | `transonic` | 2 |
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
| liftoff | 7050 | 0.12 s | -1.79 s | 1.21 s | 2.03 s |
| apogee height | 7257 | 0 m | -178 m | 198 m | 201 m |
| apogee time | 7257 | 0.04 s | -1.06 s | 1.18 s | 1.54 s |
| top speed | 1321 | 2.2 m/s | -9.49 m/s | 28.2 m/s | 38.5 m/s |
| landing | 7178 | -0.6 s | -4.4 s | 2.21 s | 3.83 s |
| flight time | 6971 | -0.74 s | -5.06 s | 3.36 s | 5.8 s |
| descent rate | 7178 | 0 m/s | -12.4 m/s | 5.62 m/s | 4.05 m/s |
| top acceleration | 290 | 0.6 m/s² | -13.7 m/s² | 13 m/s² | 16.3 m/s² |
| top acceleration, a floor | 32 | -48.1 m/s² | -75.9 m/s² | -1.59 m/s² | 12.9 m/s² |
| burnout | 296 | -0.0103 s | -0.0943 s | 0.0476 s | 0.551 s |
| burnout, a floor | 1236 | -0.24 s | -1.33 s | 0.718 s | 1.3 s |
| drogue rate | 2848 | 0.04 m/s | -1.6 m/s | 1.36 m/s | 1.79 m/s |
| main opening | 2843 | 0.73 s | -0.00484 s | 5.3 s | 2.03 s |
| main rate | 5425 | 0.04 m/s | -5.77 m/s | 5.06 m/s | 1.71 m/s |

No tuning flight's true top acceleration reaches the range its own part clips at: the nearest, flight 57's on its imu-500hz logger, comes to 0.492 of it. With its accelerometer set to ±8 g, 28 of the 38 flights logged so have a true top that reaches that range, and 10 don't. Tops read as floors: 32; clipped at the part's range, 28; held flat at the top, 4; where the accelerometer holds the thrust only in part, with no start or no end to it, 0.

Clipped at the part's range: its accelerometer set to ±8 g, imu-500hz flights 0, 3, 4, 5, 7, 8, 26, 31, 34, 49, 57, 70, 73, 107, 111, 118, 123, 132, 133, 138, 141, 160, 163, 167, 175, 190, 196, 198.

Held flat at the top: its accelerometer at 20 Hz, altos flights 78, 122, 144, 161.

## The smoother against the truth

Each flight's own log smoothed (`hpr_flightdata::smoother`, M7.2d, ADR-300) and its height above the pad and vertical speed compared with the truth's at burnout, the top vertical speed, apogee and one moment drawn per flight. Each cell gives the share of flights whose truth lies inside the smoother's 90% interval, then how many lay below and above it. A cell **fails** when the lower edge of the 99.9% Clopper–Pearson band on either side's miss share is above 5%. Logs without a barometer are withheld.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed | | 171 | 820 | 845 |
| burnout | height | 96.5% (5 / 1) | 97.6% (18 / 2) | 96.8% (26 / 1) |
| burnout | speed | 96.5% (3 / 3) | 96.2% (17 / 14) | 96.8% (17 / 10) |
| top speed | height | 96.5% (5 / 1) | 97.4% (19 / 2) | 97.4% (21 / 1) |
| top speed | speed | 95.9% (7 / 0) | 96.3% (28 / 2) | 94.1% (48 / 2) |
| apogee | height | 98.8% (2 / 0) | 97.7% (15 / 4) | 97.3% (20 / 3) |
| apogee | speed | 94.2% (1 / 9) | 96% (8 / 25) | 96% (5 / 29) |
| a drawn moment | height | 98.2% (2 / 1) | 97.1% (20 / 4) | 97.2% (22 / 2) |
| a drawn moment | speed | 97.7% (2 / 2) | 96.3% (14 / 16) | 97.5% (11 / 10) |

### The standard errors

The median and the largest, of height in m and of speed in m/s.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| burnout | height | 58.9 / 346 | 59 / 413 | 59.3 / 403 |
| burnout | speed | 11.7 / 170 | 11.4 / 185 | 11.2 / 237 |
| top speed | height | 62 / 300 | 61.4 / 336 | 62 / 358 |
| top speed | speed | 13.4 / 228 | 12 / 215 | 10.9 / 213 |
| apogee | height | 86.4 / 143 | 85.3 / 159 | 85.1 / 158 |
| apogee | speed | 1.16 / 2.06 | 1.23 / 2.28 | 1.24 / 2.59 |
| a drawn moment | height | 24.5 / 158 | 20.7 / 232 | 18.5 / 249 |
| a drawn moment | speed | 0.993 / 90.3 | 1.05 / 140 | 1.02 / 260 |

### By logger

The share inside the interval, each set's flights by logger.

#### Tuning

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 97% (0 / 1) | 97.4% (1 / 0) | 100% (0 / 0) | 90.3% (3 / 0) | 96.7% (1 / 0) |
| burnout | speed | 90.9% (3 / 0) | 100% (0 / 0) | 100% (0 / 0) | 90.3% (0 / 3) | 100% (0 / 0) |
| top speed | height | 97% (0 / 1) | 97.4% (1 / 0) | 100% (0 / 0) | 90.3% (3 / 0) | 96.7% (1 / 0) |
| top speed | speed | 93.9% (2 / 0) | 97.4% (1 / 0) | 92.1% (3 / 0) | 96.8% (1 / 0) | 100% (0 / 0) |
| apogee | height | 93.9% (2 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) |
| apogee | speed | 100% (0 / 0) | 97.4% (0 / 1) | 100% (0 / 0) | 90.3% (1 / 2) | 80% (0 / 6) |
| a drawn moment | height | 90.9% (2 / 1) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) |
| a drawn moment | speed | 93.9% (2 / 0) | 100% (0 / 0) | 97.4% (0 / 1) | 96.8% (0 / 1) | 100% (0 / 0) |

#### A

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 98.4% (2 / 1) | 96.1% (7 / 0) | 99.4% (0 / 1) | 95.8% (6 / 0) | 97.8% (3 / 0) |
| burnout | speed | 92.4% (14 / 0) | 96.7% (0 / 6) | 98.2% (3 / 0) | 95.8% (0 / 6) | 98.6% (0 / 2) |
| top speed | height | 98.9% (1 / 1) | 95.6% (8 / 0) | 99.4% (0 / 1) | 95.8% (6 / 0) | 97.1% (4 / 0) |
| top speed | speed | 92.4% (14 / 0) | 98.3% (2 / 1) | 94.7% (9 / 0) | 97.9% (2 / 1) | 99.3% (1 / 0) |
| apogee | height | 90.3% (15 / 3) | 100% (0 / 0) | 100% (0 / 0) | 100% (0 / 0) | 99.3% (0 / 1) |
| apogee | speed | 99.5% (1 / 0) | 94.5% (4 / 6) | 91.8% (1 / 13) | 97.9% (1 / 2) | 96.4% (1 / 4) |
| a drawn moment | height | 90.8% (15 / 2) | 99.4% (1 / 0) | 98.8% (1 / 1) | 100% (0 / 0) | 97.1% (3 / 1) |
| a drawn moment | speed | 93.5% (5 / 7) | 98.3% (2 / 1) | 95.3% (5 / 3) | 97.9% (2 / 1) | 97.1% (0 / 4) |

#### B

| Moment | State | altos | eggtimer | imu-500hz | perfectflite | rrc3 |
|---|---|---:|---:|---:|---:|---:|
| burnout | height | 99.5% (0 / 1) | 94.1% (10 / 0) | 98.9% (2 / 0) | 96.2% (5 / 0) | 94.9% (9 / 0) |
| burnout | speed | 95.7% (8 / 0) | 97% (0 / 5) | 95.6% (8 / 0) | 97.7% (1 / 2) | 98.3% (0 / 3) |
| top speed | height | 99.5% (0 / 1) | 94.7% (9 / 0) | 100% (0 / 0) | 97.7% (3 / 0) | 94.9% (9 / 0) |
| top speed | speed | 91.4% (16 / 0) | 97% (5 / 0) | 92.3% (14 / 0) | 93.9% (6 / 2) | 96% (7 / 0) |
| apogee | height | 88.7% (20 / 1) | 100% (0 / 0) | 100% (0 / 0) | 98.5% (0 / 2) | 100% (0 / 0) |
| apogee | speed | 100% (0 / 0) | 92.9% (3 / 9) | 96.7% (0 / 6) | 91.7% (2 / 9) | 97.2% (0 / 5) |
| a drawn moment | height | 90.3% (18 / 0) | 98.8% (1 / 1) | 100% (0 / 0) | 99.2% (1 / 0) | 98.3% (2 / 1) |
| a drawn moment | speed | 96.8% (2 / 4) | 96.4% (5 / 1) | 98.4% (1 / 2) | 96.2% (3 / 2) | 99.4% (0 / 1) |

### Trusting the GNSS receiver from liftoff

The same flights, each log with a GNSS track smoothed again with its receiver trusted from liftoff rather than once its speed agrees with the smoother's. A receiver lags a climb, so this must fail. On some flights the receiver and the barometer then disagree past their gates, both are taken back, and the estimate runs away: a track whose speed passes 11,186 m/s, Earth's escape speed, is withheld as diverged and leaves the cells, which then count fewer flights than the check's.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed again | | 33 | 185 | 186 |
| Of them, withheld as diverged | | 1 | 8 | 6 |
| burnout | height | 93.5% (5 / 6) | 94.6% (20 / 24) | 92.6% (28 / 34) |
| burnout | speed | 90.6% (6 / 10) | 87.3% (72 / 31) **fails** | 89.3% (57 / 33) |
| top speed | height | 93.5% (5 / 6) | 95.2% (18 / 21) | 94.3% (24 / 24) |
| top speed | speed | 87.1% (16 / 6) | 88.3% (74 / 21) **fails** | 85.1% (98 / 27) **fails** |
| apogee | height | 87.1% (21 / 1) **fails** | 81.8% (144 / 4) **fails** | 82.4% (144 / 4) **fails** |
| apogee | speed | 90.6% (5 / 11) | 94.2% (18 / 29) | 93.7% (15 / 38) |
| a drawn moment | height | 89.4% (14 / 4) | 84.5% (112 / 14) **fails** | 85.5% (109 / 13) **fails** |
| a drawn moment | speed | 96.5% (3 / 3) | 94.8% (20 / 22) | 95.6% (19 / 18) |

### Logs written from launch

The same flights, each log cut at the truth's liftoff, every sensor, as a logger that writes only from launch would have written it on the same zero. Each is smoothed on an assumed pad, and held as the check above is: it fails on a cell that fails, overall or for a logger, or a log withheld with a barometer. Its standard errors and sums of samples are held to two figures, the whole logs' to three, since CI's Linux runners read them in another third figure's bin, for a cause not yet traced (ADR-308): a drift shows here only past a second figure's edge.

| Moment | State | Tuning | A | B |
|---|---|---:|---:|---:|
| Flights smoothed | | 171 | 820 | 845 |
| Accelerometers set aside | | 71 | 356 | 368 |
| Apogees past the reading's bound | | 0 | 1 | 1 |
| burnout | height | 92.4% (13 / 0) | 96.7% (27 / 0) | 95.6% (37 / 0) |
| burnout | speed | 96.5% (0 / 6) | 97.7% (2 / 17) | 98.1% (2 / 14) |
| top speed | height | 92.4% (13 / 0) | 96.8% (26 / 0) | 96% (34 / 0) |
| top speed | speed | 97.7% (4 / 0) | 97.9% (14 / 3) | 96.6% (27 / 2) |
| apogee | height | 99.4% (1 / 0) | 98.7% (10 / 1) | 98.2% (11 / 4) |
| apogee | speed | 94.2% (1 / 9) | 96.5% (8 / 21) | 94.7% (9 / 36) |
| a drawn moment | height | 99.4% (1 / 0) | 98.2% (14 / 1) | 98.1% (15 / 1) |
| a drawn moment | speed | 97.1% (2 / 3) | 96.7% (10 / 17) | 97.4% (9 / 13) |

### Health

The smoothed flights' health cards, summed.

| | Tuning | A | B |
|---|---:|---:|---:|
| GNSS: logs | 33 | 185 | 186 |
| GNSS: samples rejected / logged / not weighed | 2 / 38400 / 7930 | 17 / 185000 / 38200 | 21 / 175000 / 34700 |
| GNSS: logs with a rejection | 1 | 9 | 12 |
| GNSS: resets | 0 | 0 | 0 |
| accelerometer: logs | 71 | 356 | 368 |
| accelerometer: samples rejected / logged / not weighed | 0 / 4460000 / 3830000 | 0 / 18900000 / 16000000 | 0 / 18700000 / 15700000 |
| accelerometer: logs with a rejection | 0 | 0 | 0 |
| accelerometer: resets | 0 | 0 | 0 |
| barometer: logs | 171 | 820 | 845 |
| barometer: samples rejected / logged / not weighed | 4430 / 4880000 / 0 | 13800 / 20700000 / 0 | 13500 / 20700000 / 0 |
| barometer: logs with a rejection | 62 | 321 | 341 |
| barometer: resets | 9 | 27 | 37 |
| GNSS receivers trusted | 31 | 173 | 177 |
| Transonic spans de-weighted | 107 | 478 | 510 |
| Charge spans de-weighted | 171 | 820 | 845 |
| Steps the pass back kept forward | 0 | 0 | 0 |

### A health card for each logger

Each logger's first smoothed flight in the tuning set; times are on the log's clock.

**perfectflite**, flight 16:

```text
barometer: 10755 samples, 10754 fused, 1 rejected, 0 not weighed, 0 resets; 99.9% inside the gate; mean NIS 0.709; largest test ratio 2.39 at 31.9 s, Mach 0.00
barometer de-weighted: transonic 7.40–14.2 s, charge 32.0–32.6 s
pass back: every step smoothed
```

**rrc3**, flight 21:

```text
barometer: 5625 samples, 5625 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.714; largest test ratio 0.646 at 141 s, Mach 0.01
barometer de-weighted: charge 28.1–30.1 s
pass back: every step smoothed
```

**eggtimer**, flight 9:

```text
barometer: 3069 samples, 3069 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.697; largest test ratio 0.510 at 47.5 s, Mach 0.10
barometer de-weighted: charge 27.6–28.1 s
pass back: every step smoothed
```

**altos**, flight 1:

```text
barometer: 6064 samples, 6064 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.679; largest test ratio 0.455 at 150 s, Mach 0.05
accelerometer: 6064 samples, 3496 fused, 0 rejected, 2568 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.0428; largest test ratio 0.0713 at 10.0 s, Mach 0.00
GNSS: 2852 samples, 2487 fused, 0 rejected, 365 not weighed, 0 resets; 100.0% inside the gate; mean NIS 1.19; largest test ratio 0.750 at 204 s, Mach 0.02
barometer de-weighted: transonic 12.8–13.4 s, charge 33.7–36.1 s
GNSS trusted from 46.2 s
barometer less accelerometer at apogee: -314 m
pass back: every step smoothed
```

**imu-500hz**, flight 0:

```text
barometer: 34606 samples, 34606 fused, 0 rejected, 0 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.614; largest test ratio 0.730 at 23.9 s, Mach 0.22
accelerometer: 34606 samples, 15406 fused, 0 rejected, 19200 not weighed, 0 resets; 100.0% inside the gate; mean NIS 0.00850; largest test ratio 0.0361 at 6.57 s, Mach 0.00
barometer de-weighted: transonic 8.68–13.5 s, charge 29.5–32.1 s
barometer less accelerometer at apogee: 88.8 m
pass back: every step smoothed
```

## The readings' 90% intervals

Each flight's own log read by `hpr_flightdata::readings`, and each reading that states a 90% interval compared with the flight's truth (M7.2e1 to M7.2e3, ADR-257 §1–2, ADR-308). An interval runs from the reading less its 95% share of its bound to the reading less its 5% share; the shares are the 5% and 95% points of the readings' errors as shares of their bounds on the calibration set's 2000 flights (362 of them the GNSS-only tracker's, which give no apogee), a seed of their own that the test sets never share. `cargo xtask synthetic` fits them again and fails unless they equal the library's to three significant figures. A reading **holds** when the two-sided 99.9% Clopper–Pearson band on the share of truths below its interval and the band on the share above each hold 5%, and the band on the share inside holds 90%; and every probe must fail that test. The command also fails if a reading is withheld from a log with a barometer, or read with no interval, on any of the three sets: either would leave the counts with its misses. Withheld by their own rules are the top speed at or above 227.7 m/s, where the barometer's port may read the transonic flow over the nose (it looks at the reading, not the truth, so the test holds the top speeds a user is given); a burnout taken from that speed; and a top acceleration on a log with no accelerometer.

### The shares

| Reading | Readings fitted | 5% share | 95% share |
|---|---:|---:|---:|
| liftoff | 1638 | 0.203 | 0.482 |
| apogee height | 1638 | -0.63 | 0.637 |
| apogee time | 1638 | -0.339 | 0.223 |
| top speed | 579 | -0.217 | 0.265 |
| landing | 1638 | -0.335 | 0.125 |
| flight time | 1638 | -0.384 | -0.135 |
| descent rate | 1638 | -0.476 | 0.494 |
| top acceleration | 641 | -0.495 | 0.568 |
| burnout | 565 | -0.065 | 0.0686 |
| drogue rate | 781 | -0.622 | 0.615 |
| main opening | 781 | 0.141 | 0.561 |
| main rate | 1638 | -0.574 | 0.588 |

### The floors' shares

A floor's truth may lie any amount above it, so it states a lower end only: the reading less its 95% share of its bound. Its standard error is the 95% share less the median, times the bound, over 1.645. No flight's own accelerometer clips, so the top acceleration's floors come from each flight's log with its accelerometer clipping at 8 g and its top scaled to anywhere the reader can call it clipped and the part may not have clipped: from (1 − 0.3/8)/(1 + s) to 1/(1 − s) of that clip, s the part's scale error, ±3% on the 500 Hz IMU and ±10% on the AltOS part.

| Reading | Floors fitted | 95% share | Median share |
|---|---:|---:|---:|
| top acceleration | 348 | 0.259 | -0.124 |
| burnout | 438 | -0.0335 | -0.193 |

### The test

The truths below, inside and above each interval, then the shares, each set's flights flown. Floors are tested below; withheld readings, and a drogue or a main's opening on a flight with one canopy, aren't compared.

| Set | Reading | Compared | Below / inside / above | Below | Above | Floors | Withheld | Result |
|---|---|---:|---:|---:|---:|---:|---:|---|
| A | liftoff | 820 | 35 / 729 / 56 | 4.27% | 6.83% | 0 | 180 | holds |
| A | apogee height | 820 | 56 / 719 / 45 | 6.83% | 5.49% | 0 | 180 | holds |
| A | apogee time | 820 | 39 / 743 / 38 | 4.76% | 4.63% | 0 | 180 | holds |
| A | top speed | 286 | 11 / 260 / 15 | 3.85% | 5.24% | 0 | 714 | holds |
| A | landing | 820 | 48 / 736 / 36 | 5.85% | 4.39% | 0 | 180 | holds |
| A | flight time | 820 | 51 / 736 / 33 | 6.22% | 4.02% | 0 | 180 | holds |
| A | descent rate | 820 | 46 / 730 / 44 | 5.61% | 5.37% | 0 | 180 | holds |
| A | top acceleration | 356 | 12 / 322 / 22 | 3.37% | 6.18% | 181 | 644 | holds |
| A | burnout | 307 | 10 / 286 / 11 | 3.26% | 3.58% | 211 | 482 | holds |
| A | drogue rate | 428 | 31 / 380 / 17 | 7.24% | 3.97% | 0 | 180 | holds |
| A | main opening | 428 | 24 / 375 / 29 | 5.61% | 6.78% | 0 | 180 | holds |
| A | main rate | 820 | 43 / 738 / 39 | 5.24% | 4.76% | 0 | 180 | holds |
| B | liftoff | 845 | 52 / 739 / 54 | 6.15% | 6.39% | 0 | 155 | holds |
| B | apogee height | 845 | 39 / 758 / 48 | 4.62% | 5.68% | 0 | 155 | holds |
| B | apogee time | 845 | 43 / 758 / 44 | 5.09% | 5.21% | 0 | 155 | holds |
| B | top speed | 288 | 8 / 263 / 17 | 2.78% | 5.9% | 0 | 712 | holds |
| B | landing | 845 | 35 / 780 / 30 | 4.14% | 3.55% | 0 | 155 | holds |
| B | flight time | 845 | 34 / 774 / 37 | 4.02% | 4.38% | 0 | 155 | holds |
| B | descent rate | 845 | 39 / 758 / 48 | 4.62% | 5.68% | 0 | 155 | holds |
| B | top acceleration | 368 | 18 / 330 / 20 | 4.89% | 5.43% | 187 | 632 | holds |
| B | burnout | 325 | 25 / 281 / 19 | 7.69% | 5.85% | 215 | 460 | holds |
| B | drogue rate | 436 | 23 / 393 / 20 | 5.28% | 4.59% | 0 | 155 | holds |
| B | main opening | 436 | 16 / 397 / 23 | 3.67% | 5.28% | 0 | 155 | holds |
| B | main rate | 845 | 42 / 756 / 47 | 4.97% | 5.56% | 0 | 155 | holds |

### The floors' test

Each floor's lower end against the flight's truth: the band on the share of truths below it must hold 5%, the coverage test's lower tail. Raised by one standard error and lowered by two, it must fail.

| Set | Reading | Compared | Below | Share | Band | Raised one standard error | Lowered two standard errors | Result |
|---|---|---:|---:|---:|---|---:|---:|---|
| A | top acceleration | 181 | 3 | 1.66% | 0.0831% to 7.47% | 52, fails | 0, fails | holds |
| A | burnout | 211 | 12 | 5.69% | 1.8% to 12.9% | 54, fails | 0, fails | holds |
| B | top acceleration | 187 | 3 | 1.6% | 0.0805% to 7.24% | 44, fails | 0, fails | holds |
| B | burnout | 215 | 7 | 3.26% | 0.634% to 9.31% | 45, fails | 1, fails | holds |

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
| A | burnout | altos | 47 | 6 |
| A | burnout | eggtimer | 67 | 1 |
| A | burnout | perfectflite | 50 | 3 |
| A | burnout | rrc3 | 47 | 2 |
| A | burnout | rocketpy-bella-lui | 61 | 9 |
| A | burnout | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 3 | 0 |
| A | burnout | rocketpy-juno-iii | 22 | 0 |
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
| B | burnout | altos | 39 | 2 |
| B | burnout | eggtimer | 69 | 2 |
| B | burnout | perfectflite | 42 | 1 |
| B | burnout | rrc3 | 65 | 2 |
| B | burnout | rocketpy-bella-lui | 63 | 6 |
| B | burnout | rocketpy-calisto-getting-started-motor-at-minus-1.255 | 4 | 0 |
| B | burnout | rocketpy-calisto-tests-motor-at-minus-1.373 | 3 | 0 |
| B | burnout | rocketpy-juno-iii | 32 | 0 |
| B | burnout | rocketpy-ndrt-2020-nose-to-tail | 43 | 1 |
| B | burnout | rocketpy-valetudo | 70 | 0 |

### The probes

Each interval narrowed to 0.8 of its width about its middle, widened to 1.25, and shifted down by one stated standard error (its width over 2 × 1.645): below / inside / above. Each must fail the test.

| Set | Reading | narrowed to 0.8 | widened to 1.25 | shifted down one standard error |
|---|---|---:|---:|---:|
| A | liftoff | 64 / 628 / 128, fails | 9 / 798 / 13, fails | 2 / 472 / 346, fails |
| A | apogee height | 116 / 592 / 112, fails | 0 / 820 / 0, fails | 0 / 537 / 283, fails |
| A | apogee time | 71 / 677 / 72, fails | 5 / 802 / 13, fails | 3 / 678 / 139, fails |
| A | top speed | 23 / 235 / 28, fails | 3 / 277 / 6, fails | 0 / 213 / 73, fails |
| A | landing | 57 / 658 / 105, fails | 33 / 783 / 4, fails | 16 / 398 / 406, fails |
| A | flight time | 103 / 662 / 55, fails | 13 / 800 / 7, fails | 3 / 642 / 175, fails |
| A | descent rate | 94 / 634 / 92, fails | 1 / 816 / 3, fails | 0 / 563 / 257, fails |
| A | top acceleration | 23 / 292 / 41, fails | 1 / 350 / 5, fails | 0 / 258 / 98, fails |
| A | burnout | 22 / 257 / 28, fails | 4 / 303 / 0, fails | 1 / 148 / 158, fails |
| A | drogue rate | 64 / 313 / 51, fails | 0 / 428 / 0, fails | 0 / 286 / 142, fails |
| A | main opening | 68 / 300 / 60, fails | 7 / 415 / 6, fails | 1 / 332 / 95, fails |
| A | main rate | 108 / 608 / 104, fails | 0 / 820 / 0, fails | 0 / 545 / 275, fails |
| B | liftoff | 80 / 657 / 108, fails | 26 / 807 / 12, fails | 10 / 474 / 361, fails |
| B | apogee height | 136 / 580 / 129, fails | 0 / 845 / 0, fails | 0 / 561 / 284, fails |
| B | apogee time | 75 / 699 / 71, fails | 5 / 823 / 17, fails | 0 / 691 / 154, fails |
| B | top speed | 33 / 226 / 29, fails | 3 / 278 / 7, fails | 0 / 207 / 81, fails |
| B | landing | 43 / 697 / 105, fails | 25 / 817 / 3, fails | 8 / 397 / 440, fails |
| B | flight time | 76 / 691 / 78, fails | 9 / 821 / 15, fails | 1 / 657 / 187, fails |
| B | descent rate | 114 / 625 / 106, fails | 0 / 841 / 4, fails | 0 / 580 / 265, fails |
| B | top acceleration | 31 / 303 / 34, fails | 1 / 360 / 7, fails | 0 / 275 / 93, fails |
| B | burnout | 33 / 249 / 43, fails | 13 / 309 / 3, fails | 1 / 160 / 164, fails |
| B | drogue rate | 79 / 298 / 59, fails | 0 / 436 / 0, fails | 0 / 292 / 144, fails |
| B | main opening | 56 / 314 / 66, fails | 6 / 428 / 2, fails | 2 / 334 / 100, fails |
| B | main rate | 123 / 608 / 114, fails | 0 / 843 / 2, fails | 0 / 567 / 278, fails |

### Each reading's error side

Each reading's mean error, the reading less the truth, over the readings compared, with the largest error's size and the share of truths above the interval (ADR-257 §2). For the apogee's height, the top speed, the top acceleration, the mean descent rate and each canopy's, a reading too low is the flattering side: a waiver's height, a fin's margin (the top speed is vertical), the load the airframe took, a descent gentler than it was. The share of truths above the interval is that side's test, and the shifted-down probe its mutation probe; the times have none here. The mean is given at the figures of the largest error, the digits every platform shares, and with its sign: +0 is a mean too small for those figures that isn't below zero.

| Set | Reading | Flattering side | Mean error | Largest error | Truths above |
|---|---|---|---:|---:|---:|
| A | liftoff | none | +0.243 s | 0.471 s | 6.83% |
| A | apogee height | low | +1 m | 237 m | 5.49% |
| A | apogee time | none | +0.006 s | 0.481 s | 4.63% |
| A | top speed | low | +1.3 m/s | 20.4 m/s | 5.24% |
| A | landing | none | -0.37 s | 1.33 s | 4.39% |
| A | flight time | none | -0.62 s | 1.56 s | 4.02% |
| A | descent rate | low | -0.01 m/s | 6.72 m/s | 5.37% |
| A | top acceleration | low | -0 m/s² | 14.3 m/s² | 6.18% |
| A | burnout | none | -0.0154 s | 0.0893 s | 3.58% |
| A | drogue rate | low | +0.02 m/s | 1.5 m/s | 3.97% |
| A | main opening | none | +0.42 s | 0.767 s | 6.78% |
| A | main rate | low | -0 m/s | 5.76 m/s | 4.76% |
| B | liftoff | none | +0.247 s | 0.554 s | 6.39% |
| B | apogee height | low | +0 m | 235 m | 5.68% |
| B | apogee time | none | +0.006 s | 0.636 s | 5.21% |
| B | top speed | low | +1.3 m/s | 26.8 m/s | 5.9% |
| B | landing | none | -0.37 s | 1.17 s | 3.55% |
| B | flight time | none | -0.62 s | 1.5 s | 4.38% |
| B | descent rate | low | -0.01 m/s | 6.61 m/s | 5.68% |
| B | top acceleration | low | +0.4 m/s² | 15.7 m/s² | 5.43% |
| B | burnout | none | -0.016 s | 0.114 s | 5.85% |
| B | drogue rate | low | -0 m/s | 1.5 m/s | 4.59% |
| B | main opening | none | +0.413 s | 0.921 s | 5.28% |
| B | main rate | low | +0.01 m/s | 5.86 m/s | 5.56% |

### By design

Below / inside / above.

Set A:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 2 / 72 / 2 | 2 / 74 / 0 | 2 / 72 / 2 | 1 / 75 / 0 | 0 / 69 / 7 | 0 / 67 / 9 | 0 / 76 / 0 | 0 / 39 / 0 | 0 / 15 / 0 | none | none | 2 / 74 / 0 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 80 / 2 | 8 / 72 / 2 | 5 / 70 / 7 | 0 / 3 / 1 | 0 / 82 / 0 | 3 / 79 / 0 | 9 / 71 / 2 | 4 / 28 / 2 | 0 / 34 / 0 | 8 / 72 / 2 | 0 / 82 / 0 | 5 / 75 / 2 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 75 / 7 | 2 / 76 / 4 | 3 / 76 / 3 | 0 / 0 / 2 | 0 / 81 / 1 | 4 / 78 / 0 | 2 / 77 / 3 | 0 / 34 / 3 | 0 / 37 / 0 | 2 / 77 / 3 | 1 / 81 / 0 | 2 / 78 / 2 |
| rocketpy-cavour | 0 / 64 / 21 | 7 / 72 / 6 | 10 / 67 / 8 | none | 48 / 37 / 0 | 27 / 58 / 0 | 1 / 80 / 4 | 3 / 29 / 1 | 2 / 27 / 4 | none | none | 7 / 71 / 7 |
| rocketpy-genesis | 0 / 77 / 12 | 8 / 76 / 5 | 4 / 83 / 2 | none | 0 / 87 / 2 | 1 / 88 / 0 | 10 / 70 / 9 | 2 / 36 / 1 | 2 / 37 / 0 | 8 / 76 / 5 | 0 / 60 / 29 | 8 / 77 / 4 |
| rocketpy-juno-iii | 22 / 54 / 0 | 3 / 65 / 8 | 3 / 70 / 3 | 2 / 23 / 5 | 0 / 74 / 2 | 0 / 66 / 10 | 4 / 64 / 8 | 1 / 30 / 0 | 0 / 24 / 0 | none | none | 4 / 63 / 9 |
| rocketpy-lince | 0 / 65 / 8 | 9 / 56 / 8 | 5 / 66 / 2 | none | 0 / 73 / 0 | 14 / 59 / 0 | 11 / 51 / 11 | 1 / 29 / 3 | 1 / 26 / 6 | none | none | 10 / 53 / 10 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 88 / 4 | 4 / 83 / 5 | 3 / 85 / 4 | 0 / 91 / 1 | 0 / 72 / 20 | 0 / 90 / 2 | 0 / 92 / 0 | 1 / 26 / 7 | 0 / 33 / 1 | 4 / 86 / 2 | 12 / 80 / 0 | 0 / 92 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 9 / 68 / 6 | 3 / 74 / 6 | none | 0 / 83 / 0 | 2 / 81 / 0 | 9 / 67 / 7 | 0 / 38 / 5 | 5 / 38 / 0 | 9 / 69 / 5 | 11 / 72 / 0 | 1 / 80 / 2 |
| rocketpy-valetudo | 10 / 72 / 0 | 4 / 77 / 1 | 1 / 80 / 1 | 8 / 68 / 6 | 0 / 78 / 4 | 0 / 70 / 12 | 0 / 82 / 0 | 0 / 33 / 0 | 0 / 15 / 0 | none | none | 4 / 75 / 3 |

Set B:

| Design | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| rocketpy-bella-lui | 9 / 77 / 1 | 3 / 83 / 1 | 2 / 85 / 0 | 0 / 82 / 5 | 0 / 79 / 8 | 0 / 76 / 11 | 0 / 87 / 0 | 1 / 40 / 0 | 0 / 24 / 0 | none | none | 4 / 81 / 2 |
| rocketpy-calisto-getting-started-motor-at-minus-1.255 | 0 / 81 / 10 | 5 / 81 / 5 | 5 / 81 / 5 | 0 / 2 / 3 | 0 / 91 / 0 | 3 / 88 / 0 | 5 / 82 / 4 | 2 / 39 / 5 | 0 / 45 / 1 | 5 / 82 / 4 | 0 / 90 / 1 | 5 / 83 / 3 |
| rocketpy-calisto-tests-motor-at-minus-1.373 | 0 / 94 / 8 | 3 / 94 / 5 | 8 / 90 / 4 | 0 / 4 / 2 | 0 / 102 / 0 | 1 / 101 / 0 | 6 / 93 / 3 | 2 / 35 / 4 | 0 / 41 / 0 | 3 / 95 / 4 | 2 / 100 / 0 | 2 / 96 / 4 |
| rocketpy-cavour | 0 / 64 / 14 | 2 / 71 / 5 | 1 / 69 / 8 | none | 35 / 43 / 0 | 17 / 61 / 0 | 1 / 72 / 5 | 4 / 38 / 1 | 9 / 29 / 5 | none | none | 4 / 67 / 7 |
| rocketpy-genesis | 0 / 85 / 7 | 5 / 85 / 2 | 4 / 82 / 6 | none | 0 / 91 / 1 | 0 / 92 / 0 | 8 / 80 / 4 | 3 / 33 / 3 | 7 / 32 / 0 | 5 / 86 / 1 | 0 / 70 / 22 | 5 / 84 / 3 |
| rocketpy-juno-iii | 29 / 55 / 0 | 6 / 71 / 7 | 6 / 72 / 6 | 2 / 31 / 6 | 0 / 81 / 3 | 0 / 69 / 15 | 7 / 66 / 11 | 0 / 36 / 2 | 0 / 26 / 0 | none | none | 8 / 64 / 12 |
| rocketpy-lince | 0 / 66 / 11 | 2 / 68 / 7 | 4 / 68 / 5 | none | 0 / 76 / 1 | 12 / 65 / 0 | 6 / 58 / 13 | 1 / 28 / 3 | 4 / 15 / 13 | none | none | 5 / 61 / 11 |
| rocketpy-ndrt-2020-nose-to-tail | 0 / 65 / 3 | 5 / 57 / 6 | 3 / 63 / 2 | 0 / 68 / 0 | 0 / 54 / 14 | 0 / 68 / 0 | 0 / 68 / 0 | 4 / 20 / 1 | 0 / 25 / 0 | 5 / 58 / 5 | 4 / 64 / 0 | 0 / 68 / 0 |
| rocketpy-prometheus-2022-generic-motor | 1 / 82 / 0 | 3 / 72 / 8 | 6 / 73 / 4 | none | 0 / 83 / 0 | 1 / 82 / 0 | 6 / 69 / 8 | 1 / 34 / 1 | 5 / 31 / 0 | 5 / 72 / 6 | 10 / 73 / 0 | 1 / 80 / 2 |
| rocketpy-valetudo | 13 / 70 / 0 | 5 / 76 / 2 | 4 / 75 / 4 | 6 / 76 / 1 | 0 / 80 / 3 | 0 / 72 / 11 | 0 / 83 / 0 | 0 / 27 / 0 | 0 / 13 / 0 | none | none | 8 / 72 / 3 |

### By logger and by the day's air

Below / inside / above, and in bold where that group's counts alone would fail the test. The test is on each set as a whole; these groups show where its misses fall. A barometric height is off by about the day's offset from the standard atmosphere over its temperature, so the misses of the height, the top speed and the rates of descent gather on the coldest and warmest days: a warm day reads low, and its truth lies above the interval.

#### By logger

Set A:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | **0 / 126 / 18** | 11 / 124 / 9 | 7 / 136 / 1 | 1 / 45 / 4 | 13 / 127 / 4 | 11 / 130 / 3 | 8 / 131 / 5 | none | none | 6 / 58 / 2 | 1 / 62 / 3 | 10 / 128 / 6 |
| rrc3 | 1 / 125 / 13 | 8 / 124 / 7 | 6 / 129 / 4 | 2 / 43 / 2 | 9 / 123 / 7 | 4 / 132 / 3 | 8 / 123 / 8 | none | none | 6 / 72 / 3 | 2 / 74 / 5 | 6 / 128 / 5 |
| eggtimer | **3 / 155 / 23** | 15 / 158 / 8 | 20 / 157 / 4 | 3 / 60 / 4 | 10 / 166 / 5 | 13 / 166 / 2 | 8 / 166 / 7 | none | none | 6 / 85 / 3 | 5 / 85 / 4 | 9 / 162 / 10 |
| altos | 17 / 166 / 2 | 11 / 162 / 12 | **2 / 155 / 28** | 3 / 62 / 2 | 16 / 167 / 2 | 19 / 165 / 1 | 12 / 159 / 14 | **12 / 151 / 22** | 0 / 126 / 10 | 7 / 79 / 3 | 6 / 72 / 11 | 10 / 165 / 10 |
| imu-500hz | **14 / 157 / 0** | 11 / 151 / 9 | **4 / 166 / 1** | 2 / 50 / 3 | **0 / 153 / 18** | **4 / 143 / 24** | 10 / 151 / 10 | **0 / 171 / 0** | 10 / 160 / 1 | 6 / 86 / 6 | 10 / 82 / 6 | 8 / 155 / 8 |

Set B:

| Logger | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| perfectflite | 3 / 118 / 11 | 5 / 121 / 6 | 9 / 117 / 6 | 0 / 38 / 4 | 8 / 122 / 2 | 8 / 123 / 1 | 4 / 122 / 6 | none | none | 4 / 69 / 2 | 3 / 69 / 3 | 4 / 121 / 7 |
| rrc3 | 6 / 153 / 17 | 5 / 163 / 8 | 7 / 166 / 3 | 2 / 61 / 2 | 8 / 159 / 9 | 3 / 167 / 6 | 6 / 162 / 8 | none | none | 2 / 82 / 6 | 0 / 88 / 2 | 6 / 162 / 8 |
| eggtimer | **3 / 143 / 23** | 13 / 145 / 11 | **24 / 140 / 5** | 2 / 63 / 4 | 7 / 159 / 3 | 9 / 159 / 1 | 10 / 150 / 9 | none | none | 9 / 71 / 4 | 4 / 76 / 4 | 11 / 151 / 7 |
| altos | **23 / 160 / 3** | 4 / 168 / 14 | **2 / 155 / 29** | 3 / 50 / 4 | 12 / 172 / 2 | 12 / 172 / 2 | 8 / 160 / 18 | **18 / 148 / 20** | **0 / 125 / 18** | 1 / 85 / 5 | 5 / 79 / 7 | 9 / 160 / 17 |
| imu-500hz | **17 / 165 / 0** | 12 / 161 / 9 | **1 / 180 / 1** | 1 / 51 / 3 | **0 / 168 / 14** | **2 / 153 / 27** | 11 / 164 / 7 | **0 / 182 / 0** | **25 / 156 / 1** | 7 / 86 / 3 | 4 / 85 / 7 | 12 / 162 / 8 |

#### By the day's air

Set A:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 3 / 115 / 14 | **56 / 76 / 0** | 7 / 116 / 9 | **10 / 24 / 0** | 9 / 118 / 5 | 10 / 117 / 5 | **46 / 86 / 0** | 1 / 48 / 3 | 3 / 39 / 3 | **31 / 41 / 0** | 1 / 65 / 6 | **43 / 89 / 0** |
| −10 to +10 K | 27 / 489 / 35 | **0 / 551 / 0** | 24 / 502 / 25 | **1 / 194 / 0** | 32 / 497 / 22 | 34 / 495 / 22 | **0 / 551 / 0** | 10 / 217 / 14 | 5 / 193 / 6 | **0 / 287 / 0** | 16 / 255 / 16 | **0 / 551 / 0** |
| warmer than +10 K | 5 / 125 / 7 | **0 / 92 / 45** | 8 / 125 / 4 | **0 / 42 / 15** | 7 / 121 / 9 | 7 / 124 / 6 | **0 / 93 / 44** | 1 / 57 / 5 | 2 / 54 / 2 | **0 / 52 / 17** | 7 / 55 / 7 | **0 / 98 / 39** |

Set B:

| The day's air | liftoff | apogee height | apogee time | top speed | landing | flight time | descent rate | top acceleration | burnout | drogue rate | main opening | main rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| colder than −10 K | 14 / 140 / 9 | **39 / 124 / 0** | 12 / 144 / 7 | 6 / 46 / 0 | 8 / 148 / 7 | 5 / 149 / 9 | **39 / 124 / 0** | 3 / 65 / 4 | 5 / 56 / 2 | **23 / 65 / 0** | 4 / 82 / 2 | **42 / 121 / 0** |
| −10 to +10 K | 30 / 457 / 39 | **0 / 526 / 0** | 24 / 469 / 33 | **2 / 165 / 0** | 21 / 487 / 18 | 20 / 485 / 21 | **0 / 526 / 0** | 12 / 205 / 11 | 15 / 174 / 11 | **0 / 268 / 0** | 8 / 243 / 17 | **0 / 526 / 0** |
| warmer than +10 K | 8 / 142 / 6 | **0 / 108 / 48** | 7 / 145 / 4 | **0 / 52 / 17** | 6 / 145 / 5 | 9 / 140 / 7 | **0 / 108 / 48** | 3 / 60 / 5 | 5 / 51 / 6 | **0 / 60 / 20** | 4 / 72 / 4 | **0 / 109 / 47** |
