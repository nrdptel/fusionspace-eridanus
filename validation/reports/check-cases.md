# NASA's check cases 1 and 2

Written by `cargo xtask check-cases`; don't edit it by hand. What it checks and how far to trust it: [the accuracy page](../../docs/accuracy.md) and [rigid-body flight](../../docs/physics/flight.md). A free body of the report's mass and inertia flies each case on HPR Sim's gravity, Coriolis force, rigid-body equations and default integrator, and is set beside each code's published time history (NASA/TM-2015-218675). The codes' files are not committed; only differences and spreads are.

How to read it:

- **A difference** is HPR Sim's value less the code's. A largest difference is the largest in size over the file's samples of the 30 s, with its sign.
- **The spread** is the largest difference between any two codes at a time every one of them has. One code is left out, and named, when leaving it out cuts the spread by more than half; the spread of the rest is then the measure.
- **The model difference** is a named difference between HPR Sim's model and the codes', sized apart from the flight.
- **Left of the largest difference** is a code's largest difference less the model difference at the same sample. A quantity is traced when that is no more than the spread for every kept code.
- **The largest unexplained part** is that subtraction's largest at any sample, a stricter figure. Against a kept code that itself sets the spread, it is the spread plus HPR Sim's own distance from the other codes, so the table gives its part over the spread, and HPR Sim's largest unexplained part from the kept code nearest it: at the default tolerances (1e-8), and at 1e-12, where the integrator's step error is gone.

## Case 1: the dragless sphere

Volume II, Tables 1 and 25; 6 codes. The model difference: WGS 84's normal gravity, which HPR Sim falls in, less gravitation cut off after J2 with the centrifugal term, which the codes fall in, integrated twice along the fall.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| altitude | 0.6351 mm | SIM 1 | 0.1424 mm | 19.51 mm | -19.50 mm | 0.1364 mm | past the spread; traced to within it |

### Case 1: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| altitude | 0.1369 mm | 0 mm | 0.004434 mm | 0.004429 mm |

### Case 1, altitude: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances |
|---|--:|---|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | left out | -20.00 mm | 30 | -19.50 mm | -0.4986 mm | -0.4986 mm | -0.4986 mm |
| SIM 2 | 301 | kept | -19.37 mm | 30 | -19.50 mm | +0.1364 mm | +0.1369 mm | +0.1369 mm |
| SIM 3 | 301 | kept | -19.51 mm | 30 | -19.50 mm | -0.005492 mm | -0.005501 mm | -0.005496 mm |
| SIM 4 | 301 | kept | -19.51 mm | 30 | -19.50 mm | -0.004434 mm | -0.004434 mm | -0.004429 mm |
| SIM 5 | 3001 | kept | -19.51 mm | 30 | -19.50 mm | -0.004510 mm | -0.004510 mm | -0.004504 mm |
| SIM 6 | 301 | kept | -19.51 mm | 30 | -19.50 mm | -0.005325 mm | -0.005325 mm | -0.005320 mm |

## Case 2: the tumbling brick

Volume II, Tables 4 and 26, with the errata's 30,000 ft; 5 codes. The model difference: the Earth's rate left out of HPR Sim's rotational equations: Euler's equations in the rate relative to the Earth-fixed launch frame, less the same equations in the rate relative to the stars.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| inertial roll rate | 0.003015 deg/s | SIM 6 | 0.0008991 deg/s | 0.01077 deg/s | +0.01077 deg/s | 0.0008818 deg/s | past the spread; traced to within it |
| inertial pitch rate | 0.004742 deg/s | SIM 6 | 0.001242 deg/s | 0.01633 deg/s | -0.01633 deg/s | 0.001005 deg/s | past the spread; traced to within it |
| inertial yaw rate | 0.001152 deg/s | SIM 6 | 0.0003532 deg/s | 0.005500 deg/s | +0.005500 deg/s | 0.0001268 deg/s | past the spread; traced to within it |

### Case 2: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| inertial roll rate | 0.0008993 deg/s | 0.0000001701 deg/s | 0.0000005866 deg/s | 0.000000002126 deg/s |
| inertial pitch rate | 0.001242 deg/s | 0.00000007201 deg/s | 0.0000009229 deg/s | 0.000000003051 deg/s |
| inertial yaw rate | 0.0003533 deg/s | 0.00000007634 deg/s | 0.0000002790 deg/s | 0.000000001089 deg/s |

### Case 2, inertial roll rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | +0.01077 deg/s | 29.3 | +0.01077 deg/s | -0.0000002040 deg/s | +0.0000005866 deg/s | -0.000000002129 deg/s | -0.000000002113 deg/s |
| SIM 2 | 301 | kept | +0.009886 deg/s | 29.3 | +0.01077 deg/s | -0.0008818 deg/s | -0.0008993 deg/s | -0.0008991 deg/s | -0.0008991 deg/s |
| SIM 4 | 301 | kept | +0.01077 deg/s | 29.3 | +0.01077 deg/s | -0.0000002040 deg/s | +0.0000005866 deg/s | -0.000000002126 deg/s | -0.000000002097 deg/s |
| SIM 5 | 3001 | kept | +0.01073 deg/s | 29.26 | +0.01077 deg/s | -0.00003544 deg/s | -0.00003610 deg/s | -0.00003594 deg/s | -0.00003594 deg/s |
| SIM 6 | 301 | left out | +0.007986 deg/s | 29.4 | +0.01074 deg/s | -0.002757 deg/s | +0.003015 deg/s | +0.003015 deg/s | +0.003015 deg/s |

### Case 2, inertial pitch rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | -0.01633 deg/s | 22.9 | -0.01633 deg/s | -0.0000004006 deg/s | +0.0000009229 deg/s | +0.000000003051 deg/s | +0.000000003033 deg/s |
| SIM 2 | 301 | kept | -0.01530 deg/s | 22.8 | -0.01631 deg/s | +0.001005 deg/s | +0.001242 deg/s | +0.001242 deg/s | +0.001242 deg/s |
| SIM 4 | 301 | kept | -0.01633 deg/s | 22.9 | -0.01633 deg/s | -0.0000004006 deg/s | +0.0000009230 deg/s | +0.000000003066 deg/s | +0.000000003047 deg/s |
| SIM 5 | 3001 | kept | -0.01629 deg/s | 22.9 | -0.01633 deg/s | +0.00004107 deg/s | +0.00004975 deg/s | +0.00004968 deg/s | +0.00004968 deg/s |
| SIM 6 | 301 | left out | -0.01177 deg/s | 22.8 | -0.01631 deg/s | +0.004541 deg/s | +0.004742 deg/s | +0.004742 deg/s | +0.004742 deg/s |

### Case 2, inertial yaw rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | +0.005500 deg/s | 21.3 | +0.005500 deg/s | -0.00000002354 deg/s | -0.0000002790 deg/s | -0.000000001089 deg/s | -0.000000001057 deg/s |
| SIM 2 | 301 | kept | +0.005344 deg/s | 28.7 | +0.005471 deg/s | -0.0001268 deg/s | -0.0003533 deg/s | -0.0003532 deg/s | -0.0003532 deg/s |
| SIM 4 | 301 | kept | +0.005500 deg/s | 21.3 | +0.005500 deg/s | -0.00000002359 deg/s | -0.0000002791 deg/s | -0.000000001146 deg/s | -0.000000001118 deg/s |
| SIM 5 | 3001 | kept | +0.005490 deg/s | 21.3 | +0.005500 deg/s | -0.000009732 deg/s | -0.00001420 deg/s | -0.00001412 deg/s | -0.00001412 deg/s |
| SIM 6 | 301 | left out | +0.004861 deg/s | 28.5 | +0.005380 deg/s | -0.0005185 deg/s | -0.001153 deg/s | +0.001152 deg/s | +0.001152 deg/s |
