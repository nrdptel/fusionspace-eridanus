# Apogee predicted from the boost, on the logs of real flights

Written by `cargo xtask real-flights` ([M7.3d2][m7-3d], decisions [ADR-257][adr-257] §3 and [ADR-321][adr-321]). How the prediction works is on the [fit's page][site].

- **What is flown:** the 7 flights of the [real-flights report][real], each in its ERA5 air, from its example's thrust file, rail and site.
- **The prediction:** `hpr_forensics::boost` cuts the log just after burnout, as it finds burnout from the samples up to the cut, fits the log up to the cut, and flies the fit on to apogee. A barometer's log whose fitted climb speed reaches 227.7 m/s (747 ft/s) gets no cut. Its apogee is read as the forward simulation's is, from the ERA5 pressure at its center of mass, less the pad's, and set against the same log apogee.
- **The fit:** least squares, told the design, rail, site and ERA5 air. Its wind is constant with height; what's stated of it is ERA5's, averaged evenly in height from the pad to the forward simulation's apogee. Each parameter is held to what's stated of it (below).
- **The forward simulation:** the [real-flights report][real]'s, in ERA5's air and wind, nothing fitted.
- **The interval:** on each predicted flight's absolute error from the prediction less the forward simulation's. It is the exact sign-flip interval (Hartigan, 1969), whose coverage is at least 95% for differences drawn symmetric about their center. With 5 flights or fewer, no pattern of signs is rare enough at that level, and it is unbounded.
- **Files:** the logs, thrust files and weather files are read from the pinned RocketPy checkout and never committed; each one's SHA-256 is in the JSON.

[m7-3d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3d
[adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-321]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
[site]: https://hpr.fusionspace.co/physics/flight-fit.html#apogee-from-the-boost
[real]: real-flights.md

## Summary

- Flights: 7; with a prediction: 2 (ndrt-2020, lince).
- The prediction's mean absolute apogee error: 3.03%; with its sign: -0.85%.
- The forward simulation's on the same flights: 6.89%; with its sign: +6.89%.
- The fit's model at what was stated, nothing fitted, on the same flights: 5.07%; with its sign: +5.07%. Against the forward simulation's, this is what the model's constant wind moves; against the prediction's, what the fit to the boost moves.
- The prediction's absolute error less the forward simulation's, on average: -3.86 percentage points; its exact sign-flip interval: unbounded, as on 2 flights no pattern of signs is rare enough at 95% (it takes 6).
- Cut less than the thrust file's burn after liftoff, so in the boost or at its end, not after it: bella-lui, prometheus-2022, juno-iii, lince. On these logs a real barometer's fitted speed dips by more than the cut's fall for up to half a second during the boost, which the finder reads as burnout ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)). A prediction from such a cut is in the counts above as the library gives it.
- The forward simulation on all 7 flights: mean absolute apogee error 6.06%, with its sign -0.22%.

## The prediction

Times are on the log's clock. The burn is the thrust file's, as flown. The stated apogee is the fit's model, nothing fitted, in its constant wind: set beside the forward simulation's, it shows what that wind alone moves.

| flight | liftoff (s) | cut (s) | top speed (m/s) | burn (s) | outcome | log apogee (m) | predicted (m) | error | forward (m) | forward error | stated (m) | stated error | forward's largest Mach |
|---|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | 0.21 | 2.46 | 91.5 | 2.43 | cut, but too little before the cut to fit | 459.0 | none | none | 462.9 | +0.86% | 462.9 | +0.85% | 0.264 |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | 0.49 | 4.84 | 161.9 | 3.43 | predicted | 1320.4 | 1269.2 | -3.88% | 1457.6 | +10.40% | 1407.1 | +6.57% | 0.536 |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | 0.23 | 2.02 | 61.9 | 4.88 | cut, but too little before the cut to fit | 3895.8 | none | none | 3549.3 | -8.90% | 3639.8 | -6.57% | 0.959 |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | 0.55 | 1.35 | 131.3 | 5.69 | cut, but too little before the cut to fit | 3151.5 | none | none | 2923.4 | -7.24% | 2862.1 | -9.18% | 0.733 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | 0.20 | none | none | 3.60 | no cut: climbing at 307.0 m/s at 3.20 s, too fast | 2785.0 | none | none | 2946.0 | +5.78% | 2926.6 | +5.08% | 0.933 |
| Genesis, EuRoC 2023 (L995) | 0.25 | none | none | 3.60 | no cut: climbing at 229.0 m/s at 3.12 s, too fast | 2916.7 | none | none | 2746.0 | -5.85% | 2744.7 | -5.90% | 0.854 |
| Lince, EuRoC 2023 (M1101) | 0.18 | 2.58 | 224.1 | 4.67 | predicted | 3587.7 | 3666.2 | +2.19% | 3709.2 | +3.39% | 3715.6 | +3.56% | 0.993 |

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

Each log fitted whole, from a second before liftoff to its apogee, and up to its cut where it has a prediction. A 90% interval is the standard error times the shares fitted on the synthetic calibration set; the burn time's scale, which the fit isn't asked for, has none, and nor has the wind on a log with no track, whose shares were fitted with the synthetic flights' own stated wind. A scale is the factor that best fits one flight, not the rocket's drag or motor at every speed; on a real flight the model's own errors go into it.

### Bella Lui, EPFL Rocket Team, 2020 (K828FJ)

Stated wind: 0.67 m/s toward the east, 1.09 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 2.06 m, misfit share 0.0132, 51 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.016 | 0.103 | 0.870 to 1.146 |
| impulse scale | 0.996 | 0.021 | 0.965 to 1.028 |
| burn time scale | 0.986 | 0.046 | none |
| wind toward the east (m/s) | 0.821 | 3.839 | none |
| wind toward the north (m/s) | 1.308 | 3.798 | none |
| structure mass scale | 1.000 | 0.011 | 0.983 to 1.018 |

### NDRT 2020, Notre Dame Rocketry Team (L1395)

Stated wind: 10.46 m/s toward the east, 6.25 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 3.25 m, misfit share 0.0071, 92 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.709 | 0.072 | 0.606 to 0.801 |
| impulse scale | 0.950 | 0.024 | 0.914 to 0.986 |
| burn time scale | 1.062 | 0.035 | none |
| wind toward the east (m/s) | 13.814 | 3.583 | none |
| wind toward the north (m/s) | 8.300 | 3.769 | none |
| structure mass scale | 1.007 | 0.011 | 0.989 to 1.025 |

Fitted up to the cut: fitted, residual RMS 0.88 m, misfit share 0.0060, 57 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.017 | 0.109 | 0.862 to 1.155 |
| impulse scale | 0.963 | 0.027 | 0.924 to 1.004 |
| burn time scale | 1.059 | 0.041 | none |
| wind toward the east (m/s) | 12.653 | 3.609 | none |
| wind toward the north (m/s) | 7.606 | 3.776 | none |
| structure mass scale | 1.003 | 0.011 | 0.985 to 1.021 |

### Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520)

Stated wind: 9.77 m/s toward the east, 2.11 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 22.40 m, misfit share 0.0168, 405 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 0.916 | 0.074 | 0.757 to 1.010 |
| impulse scale | 1.019 | 0.025 | 0.972 to 1.057 |
| burn time scale | 1.073 | 0.054 | none |
| wind toward the east (m/s) | 11.737 | 3.870 | 6.745 to 17.000 |
| wind toward the north (m/s) | 0.598 | 3.113 | -3.698 to 5.268 |
| structure mass scale | 0.987 | 0.011 | 0.969 to 1.006 |

### Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor)

Stated wind: 13.04 m/s toward the east, 7.06 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

The whole log has nothing to fit: its apogee is withheld: the track climbs 42.1 m into its top at 26.20 s in 1.00 s. That is more than the 17.5 m a coast slowing at twice gravity climbs. The track falls 17.1 m from it by 26.50 s, more than the 16.8 m a body at rest there can fall. Each is past twice its noise and step. Its top is a spike, such as a charge's, so which is the apogee can't be told.

Fitted whole log: not fitted, residual RMS none m, misfit share none, 0 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.000 | 0.109 | none |
| impulse scale | 1.000 | 0.033 | none |
| burn time scale | 1.000 | 0.055 | none |
| wind toward the east (m/s) | 13.043 | 3.875 | none |
| wind toward the north (m/s) | 7.060 | 3.875 | none |
| structure mass scale | 1.000 | 0.011 | none |

### Cavour, Politecnico di Torino, EuRoC 2023 (L995)

Stated wind: 8.89 m/s toward the east, 6.64 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 10.72 m, misfit share 0.0122, 74 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.078 | 0.024 | 1.026 to 1.108 |
| impulse scale | 0.972 | 0.012 | 0.949 to 0.990 |
| burn time scale | 1.242 | 0.037 | none |
| wind toward the east (m/s) | 10.880 | 0.665 | 10.023 to 11.784 |
| wind toward the north (m/s) | 3.872 | 0.551 | 3.112 to 4.699 |
| structure mass scale | 1.003 | 0.010 | 0.985 to 1.021 |

### Genesis, EuRoC 2023 (L995)

Stated wind: 1.22 m/s toward the east, 3.65 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 4.06 m, misfit share 0.0041, 113 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.025 | 0.059 | 0.940 to 1.100 |
| impulse scale | 1.052 | 0.016 | 1.028 to 1.077 |
| burn time scale | 1.247 | 0.036 | none |
| wind toward the east (m/s) | 1.930 | 3.867 | none |
| wind toward the north (m/s) | 1.762 | 3.825 | none |
| structure mass scale | 0.994 | 0.010 | 0.976 to 1.011 |

### Lince, EuRoC 2023 (M1101)

Stated wind: 1.71 m/s toward the east, 2.87 m/s toward the north. The fit's model in ERA5's own wind lands +0.000 m from the forward simulation's apogee.

Fitted whole log: fitted, residual RMS 11.85 m, misfit share 0.0101, 128 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.327 | 0.059 | 1.243 to 1.402 |
| impulse scale | 1.114 | 0.018 | 1.088 to 1.142 |
| burn time scale | 0.939 | 0.040 | none |
| wind toward the east (m/s) | 1.629 | 3.854 | none |
| wind toward the north (m/s) | 3.006 | 3.754 | none |
| structure mass scale | 0.981 | 0.010 | 0.964 to 0.998 |

Fitted up to the cut: fitted, residual RMS 4.76 m, misfit share 0.0718, 49 flights flown.

| parameter | value | standard error | 90% interval |
|---|---:|---:|---:|
| drag scale | 1.001 | 0.109 | 0.845 to 1.140 |
| impulse scale | 0.986 | 0.032 | 0.940 to 1.034 |
| burn time scale | 1.049 | 0.045 | none |
| wind toward the east (m/s) | 1.647 | 3.874 | none |
| wind toward the north (m/s) | 3.096 | 3.868 | none |
| structure mass scale | 1.002 | 0.011 | 0.983 to 1.020 |

