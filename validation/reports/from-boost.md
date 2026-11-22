# Apogee predicted from the boost, on the logs of real flights

This report asks whether apogee predicted from a log cut after burnout comes closer to the logged apogee than the forward simulation, the simulator's flight of the design with nothing fitted. It covers the 7 public logged flights. 2 get a prediction, 2 of them in the case the method is meant for, so it can't yet say at 95% which comes closer. An error is an apogee less the log's, as a share of the log's: + is too high.

Written by `cargo xtask real-flights` ([M7.3d2][m7-3d], decisions [ADR-257][adr-257] §3 and [ADR-325][adr-325]). How the prediction works is on the [fit's page][site].

- **What is flown:** the 7 flights the [real-flights report][real] compared first, each in its ERA5 air, from its example's thrust file, rail and site.
- **The prediction:** the library finds the cut from the samples up to it (the first sample where the climb speed's fall has held 0.75 s and the speed is the lowest of those 0.75 s), cuts the log there, fits it, and flies the fit on to apogee (`hpr_forensics::boost`). A barometer's log whose fitted climb speed reaches 227.7 m/s (747 ft/s) before the cut gets none. Its apogee is read as the forward simulation's is: the standard atmosphere's altitude of the ERA5 pressure at the center of mass, less the pad's.
- **The fit:** least squares, told the design, rail, site and ERA5 air. Its wind is constant with height; what's stated of it is ERA5's, averaged evenly in height from the pad to the forward simulation's apogee. Each parameter is held to what's stated of it (below): the synthetic flights' spreads, not a measure of a real flight's model error.
- **The case the method is meant for:** a cut after the thrust file's burn, on a log whose climb, read whole, stays under 227.7 m/s. A cut less than the burn after liftoff counts as inside it; as liftoff lags ignition, that also takes in a cut up to that lag after the burn's end. The burn is the file's, not scaled by any fit.
- **The interval:** on the difference, flight by flight, between the prediction's absolute error and the forward simulation's. It is the exact sign-flip interval (Hartigan, 1969), whose coverage is at least 95% for independent differences, each drawn symmetric about their center. With 5 flights or fewer, no pattern of signs is rare enough at that level, and it is unbounded.
- **Files:** the logs, thrust files and weather files are read from the pinned RocketPy checkout and never committed; each one's SHA-256 is in the JSON.

[m7-3d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3d
[adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-325]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[site]: https://hpr.fusionspace.co/physics/flight-fit.html#apogee-from-the-boost
[real]: real-flights.md

## Summary

- Every flight with a prediction, as the library gives it, the method's case and the rest together, 2 (bella-lui, ndrt-2020): the prediction's mean absolute apogee error is 2.37% (-1.20% with its sign); the forward simulation's on the same flights 5.75% (+5.75% with its sign); the fit's model at what was stated, nothing fitted, 3.84% (+3.84% with its sign). The prediction's absolute error less the forward simulation's is -3.37 percentage points on average; its exact sign-flip interval: unbounded, as on 2 flights no pattern of signs is rare enough at 95% (it takes 6 flights).
- In the case the method is meant for, 2 (bella-lui, ndrt-2020): the prediction's mean absolute apogee error is 2.37% (-1.20% with its sign); the forward simulation's on the same flights 5.75% (+5.75% with its sign); the fit's model at what was stated, nothing fitted, 3.84% (+3.84% with its sign). The prediction's absolute error less the forward simulation's is -3.37 percentage points on average; its exact sign-flip interval: unbounded, as on 2 flights no pattern of signs is rare enough at 95% (it takes 6 flights).
- Set beside the forward simulation's, the stated model's error shows what the fit's constant wind alone moves; beside the prediction's, what the fit to the boost moves.
- Cut less than the thrust file's burn after liftoff, 0 of the 2 cut: none. The cut waits for the climb speed's fall to hold 0.75 s, so that a dip in a barometer's reading during the boost isn't read as burnout ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)).
- Climbing at or past 227.7 m/s, the log read whole: prometheus-2022, cavour, genesis, lince. Top speed withheld for another reason, such as a spike at the top, so the log can't be set against that speed: juno-iii.
- The forward simulation on all 7 flights: mean absolute apogee error 5.97% (+0.07% with its sign).

## The prediction

The stated apogee is the fit's model, nothing fitted, in its constant wind.

| flight | outcome | log apogee (m) | predicted (m) | error | forward (m) | forward error | stated (m) | stated error |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | predicted | 459.0 | 464.4 | +1.17% | 464.1 | +1.11% | 464.0 | +1.10% |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | predicted | 1320.4 | 1273.2 | -3.57% | 1457.5 | +10.39% | 1407.3 | +6.58% |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | no cut: climbing at 229.9 m/s at 4.73 s, too fast | 3895.8 | none | none | 3536.6 | -9.22% | 3625.4 | -6.94% |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | no cut: climbing at 233.7 m/s at 2.80 s, too fast | 3151.5 | none | none | 2976.7 | -5.55% | 2944.1 | -6.58% |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | no cut: climbing at 307.0 m/s at 3.20 s, too fast | 2785.0 | none | none | 2945.5 | +5.76% | 2930.1 | +5.21% |
| Genesis, EuRoC 2023 (L995) | no cut: climbing at 229.0 m/s at 3.12 s, too fast | 2916.7 | none | none | 2745.2 | -5.88% | 2744.1 | -5.92% |
| Lince, EuRoC 2023 (M1101) | no cut: climbing at 227.9 m/s at 3.17 s, too fast | 3587.7 | none | none | 3728.0 | +3.91% | 3735.4 | +4.11% |

## The cut

Times are on the log's clock; the burn is the thrust file's, as flown.

| flight | liftoff (s) | cut (s) | top speed before the cut (m/s) | burn (s) | cut in the burn | climbs past 227.7 m/s | forward's largest Mach |
|---|---:|---:|---:|---:|---|---|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | 0.21 | 3.96 | 91.5 | 2.43 | no | no | 0.265 |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | 0.49 | 7.39 | 168.5 | 3.43 | no | no | 0.536 |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | 0.23 | none | none | 4.88 | none | yes | 0.959 |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | 0.55 | none | none | 5.69 | none | not read | 0.732 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | 0.20 | none | none | 3.60 | none | yes | 0.933 |
| Genesis, EuRoC 2023 (L995) | 0.25 | none | none | 3.60 | none | yes | 0.854 |
| Lince, EuRoC 2023 (M1101) | 0.18 | none | none | 4.67 | none | yes | 0.994 |

## What is stated before a log

Each parameter is held to a normal distribution about its middle, as on the synthetic flights. The wind's middle is each flight's own (below).

| parameter | middle | standard deviation |
|---|---:|---:|
| drag scale | 1.000 | 0.1094 |
| impulse scale | 1.000 | 0.0328 |
| burn time scale | 1.000 | 0.0547 |
| wind toward the east (m/s) | the flight's | 3.8752 |
| wind toward the north (m/s) | the flight's | 3.8752 |
| structure mass scale | 1.000 | 0.0109 |

## The fits

Each log is fitted whole where the analyzer reads its apogee, from a second before liftoff to that apogee, and up to its cut where it has a prediction. A whole-log fit's 90% interval is the standard error times the shares fitted on the synthetic calibration set's whole-log fits. The burn time's scale, which the fit isn't asked for, has none. Nor has the wind on a log with no track, whose shares were fitted with the synthetic flights' own stated wind, nor any fit up to the cut, which no shares were fitted on. A scale is the factor that best fits one flight, not the rocket's drag or motor at every speed; on a real flight the model's own errors go into it.

### Bella Lui, EPFL Rocket Team, 2020 (K828FJ)

Stated wind: 0.67 m/s toward the east, 1.09 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 2.06 m, misfit share 0.0132, 51 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.020 | 0.103 | 0.868 to 1.157 |
| impulse scale | 0.995 | 0.021 | 0.965 to 1.028 |
| burn time scale | 0.985 | 0.046 | none |
| wind toward the east (m/s) | 0.842 | 3.840 | none |
| wind toward the north (m/s) | 1.339 | 3.798 | none |
| structure mass scale | 1.000 | 0.011 | 0.983 to 1.018 |

Fitted up to the cut: residual RMS 3.04 m, misfit share 0.0380, 37 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.999 | 0.109 | none |
| impulse scale | 1.000 | 0.021 | none |
| burn time scale | 0.993 | 0.048 | none |
| wind toward the east (m/s) | 0.660 | 3.855 | none |
| wind toward the north (m/s) | 1.082 | 3.829 | none |
| structure mass scale | 1.000 | 0.011 | none |

### NDRT 2020, Notre Dame Rocketry Team (L1395)

Stated wind: 10.46 m/s toward the east, 6.25 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 3.25 m, misfit share 0.0071, 87 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.710 | 0.073 | 0.603 to 0.807 |
| impulse scale | 0.949 | 0.024 | 0.914 to 0.987 |
| burn time scale | 1.062 | 0.035 | none |
| wind toward the east (m/s) | 13.804 | 3.587 | none |
| wind toward the north (m/s) | 8.289 | 3.770 | none |
| structure mass scale | 1.007 | 0.011 | 0.989 to 1.024 |

Fitted up to the cut: residual RMS 3.37 m, misfit share 0.0131, 61 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.947 | 0.102 | none |
| impulse scale | 0.954 | 0.026 | none |
| burn time scale | 1.061 | 0.035 | none |
| wind toward the east (m/s) | 12.619 | 3.610 | none |
| wind toward the north (m/s) | 7.589 | 3.776 | none |
| structure mass scale | 1.005 | 0.011 | none |

### Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520)

Stated wind: 9.76 m/s toward the east, 2.11 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 24.49 m, misfit share 0.0185, 197 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.836 | 0.073 | 0.684 to 0.929 |
| impulse scale | 1.016 | 0.026 | 0.968 to 1.055 |
| burn time scale | 0.925 | 0.054 | none |
| wind toward the east (m/s) | 11.206 | 3.874 | 5.976 to 16.551 |
| wind toward the north (m/s) | 0.344 | 3.379 | -4.623 to 5.413 |
| structure mass scale | 1.010 | 0.011 | 0.991 to 1.028 |

### Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor)

Stated wind: 13.05 m/s toward the east, 7.08 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

The whole log has nothing to fit: its apogee is withheld: the track climbs 42.1 m into its top at 26.20 s in 1.00 s. That is more than the 17.5 m a coast slowing at twice gravity climbs. The track falls 17.1 m from it by 26.50 s, more than the 16.8 m a body at rest there can fall. Each is past twice its noise and step. Its top is a spike, such as a charge's, so which is the apogee can't be told.

Fitted whole log: not fitted.

### Cavour, Politecnico di Torino, EuRoC 2023 (L995)

Stated wind: 8.89 m/s toward the east, 6.64 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 10.70 m, misfit share 0.0122, 74 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.075 | 0.024 | 1.026 to 1.106 |
| impulse scale | 0.972 | 0.012 | 0.949 to 0.991 |
| burn time scale | 1.243 | 0.037 | none |
| wind toward the east (m/s) | 12.207 | 0.837 | 11.077 to 13.362 |
| wind toward the north (m/s) | 4.135 | 0.623 | 3.219 to 5.070 |
| structure mass scale | 1.003 | 0.010 | 0.985 to 1.020 |

### Genesis, EuRoC 2023 (L995)

Stated wind: 1.22 m/s toward the east, 3.65 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 4.06 m, misfit share 0.0041, 104 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.024 | 0.058 | 0.939 to 1.101 |
| impulse scale | 1.052 | 0.016 | 1.028 to 1.078 |
| burn time scale | 1.247 | 0.035 | none |
| wind toward the east (m/s) | 1.937 | 3.866 | none |
| wind toward the north (m/s) | 1.768 | 3.820 | none |
| structure mass scale | 0.994 | 0.010 | 0.976 to 1.011 |

### Lince, EuRoC 2023 (M1101)

Stated wind: 1.71 m/s toward the east, 2.87 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: residual RMS 11.86 m, misfit share 0.0101, 139 simulations flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.344 | 0.058 | 1.259 to 1.422 |
| impulse scale | 1.116 | 0.018 | 1.090 to 1.144 |
| burn time scale | 0.935 | 0.040 | none |
| wind toward the east (m/s) | 1.597 | 3.851 | none |
| wind toward the north (m/s) | 3.082 | 3.753 | none |
| structure mass scale | 0.981 | 0.010 | 0.963 to 0.998 |

