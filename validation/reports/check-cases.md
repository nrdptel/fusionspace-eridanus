# NASA's check cases 1 to 3 and 6 to 10

Written by `cargo xtask check-cases`; don't edit it by hand. What it checks and how far to trust it: [the accuracy page](../../docs/accuracy.md) and [rigid-body flight](../../docs/physics/flight.md). A free body of the report's mass and inertia flies each case on HPR Sim's gravity, Coriolis force, air, wind, rigid-body equations and default integrator, under the case's damping or drag applied by the test, and is set beside each code's published time history (NASA/TM-2015-218675). The codes' files are not committed; only differences and spreads are.

How to read it:

- **A difference** is HPR Sim's value less the code's. A largest difference is the largest in size over the file's samples of the 30 s, with its sign. A position's is the distance between HPR Sim's and the code's, each in Earth-fixed axes, the code's from the latitude, longitude and altitude in its file.
- **The spread** is the largest difference between any two codes at a time every one of them has. One code is left out, and named, when leaving it out cuts the spread by more than half; the spread of the rest is then the measure.
- **The model difference** is a named difference between HPR Sim's model and the codes', sized apart from the flight. For a position it is a distance too, and the codes' side of it flies each code's own air density, so it differs from code to code; the table gives the largest.
- **Left of the largest difference** is a code's largest difference less the model difference at the same sample. A quantity is traced when that is no more than the spread for every kept code.
- **The largest unexplained part** is that subtraction's largest at any sample, a stricter figure. Against a kept code that itself sets the spread, it is the spread plus HPR Sim's own distance from the other codes, so the table gives its part over the spread, and HPR Sim's largest unexplained part from the kept code nearest it: at the default tolerances (1e-8), and at 1e-12, where the integrator's step error is negligible.

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

## Case 3: the tumbling brick with damping

Volume II, Tables 4, 5 and 27, with the errata's 30,000 ft; 5 codes. The model difference: the Earth's rate left out of HPR Sim's rotational equations: Euler's equations with the damping moment in the rate relative to the Earth-fixed launch frame, less the same equations in the rate relative to the stars, both damped by the inertial rate along the same fall.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| inertial roll rate | 0.03192 deg/s | SIM 1 | 0.003609 deg/s | 0.004873 deg/s | -0.003755 deg/s | 0.003176 deg/s | past the spread; traced to within it |
| inertial pitch rate | 0.07435 deg/s | SIM 1 | 0.003792 deg/s | 0.005633 deg/s | -0.005609 deg/s | 0.001123 deg/s | past the spread; traced to within it |
| inertial yaw rate | 0.01674 deg/s | SIM 1 | 0.003503 deg/s | 0.005102 deg/s | +0.004433 deg/s | 0.0007793 deg/s | past the spread; traced to within it |

### Case 3: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| inertial roll rate | 0.003287 deg/s | 0 deg/s | 0.00006587 deg/s | 0.00006581 deg/s |
| inertial pitch rate | 0.003791 deg/s | 0 deg/s | 0.00006268 deg/s | 0.00006222 deg/s |
| inertial yaw rate | 0.002631 deg/s | 0 deg/s | 0.0002691 deg/s | 0.0002690 deg/s |

### Case 3, inertial roll rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | left out | -0.03430 deg/s | 4.6 | -0.003526 deg/s | -0.03078 deg/s | -0.03080 deg/s | -0.03080 deg/s | -0.03083 deg/s |
| SIM 2 | 301 | kept | -0.004873 deg/s | 4.2 | -0.003702 deg/s | -0.001171 deg/s | -0.001235 deg/s | -0.001235 deg/s | -0.001261 deg/s |
| SIM 4 | 301 | kept | -0.003792 deg/s | 3.9 | -0.003754 deg/s | -0.00003815 deg/s | -0.00006587 deg/s | -0.00006581 deg/s | -0.00009115 deg/s |
| SIM 5 | 3001 | kept | -0.004078 deg/s | 3.42 | -0.003670 deg/s | -0.0004074 deg/s | +0.003287 deg/s | +0.003287 deg/s | +0.003281 deg/s |
| SIM 6 | 301 | kept | +0.003575 deg/s | 9.7 | +0.0003986 deg/s | +0.003176 deg/s | +0.003267 deg/s | +0.003267 deg/s | +0.003261 deg/s |

### Case 3, inertial pitch rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | left out | +0.06979 deg/s | 3.7 | -0.004405 deg/s | +0.07419 deg/s | +0.07431 deg/s | +0.07431 deg/s | +0.07434 deg/s |
| SIM 2 | 301 | kept | -0.004253 deg/s | 2 | -0.005376 deg/s | +0.001123 deg/s | +0.001855 deg/s | +0.001854 deg/s | +0.001884 deg/s |
| SIM 4 | 301 | kept | -0.005633 deg/s | 2.5 | -0.005609 deg/s | -0.00002375 deg/s | -0.00006268 deg/s | -0.00006222 deg/s | -0.00005211 deg/s |
| SIM 5 | 3001 | kept | -0.005134 deg/s | 2.24 | -0.005547 deg/s | +0.0004132 deg/s | -0.003791 deg/s | -0.003790 deg/s | -0.003790 deg/s |
| SIM 6 | 301 | kept | -0.004757 deg/s | 1.7 | -0.005008 deg/s | +0.0002508 deg/s | -0.003791 deg/s | -0.003790 deg/s | -0.003790 deg/s |

### Case 3, inertial yaw rate: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | The codes' model, integrated here, less the code |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | left out | +0.01878 deg/s | 5.3 | +0.002580 deg/s | +0.01620 deg/s | +0.01624 deg/s | +0.01624 deg/s | +0.01628 deg/s |
| SIM 2 | 301 | kept | +0.005102 deg/s | 3 | +0.004323 deg/s | +0.0007793 deg/s | +0.002631 deg/s | +0.002631 deg/s | +0.002682 deg/s |
| SIM 4 | 301 | kept | +0.004484 deg/s | 2.6 | +0.004432 deg/s | +0.00005147 deg/s | +0.0002691 deg/s | +0.0002690 deg/s | +0.0003215 deg/s |
| SIM 5 | 3001 | kept | +0.004399 deg/s | 2.53 | +0.004432 deg/s | -0.00003331 deg/s | -0.001297 deg/s | -0.001297 deg/s | -0.001297 deg/s |
| SIM 6 | 301 | kept | +0.004206 deg/s | 2.3 | +0.004382 deg/s | -0.0001758 deg/s | -0.001297 deg/s | -0.001297 deg/s | -0.001297 deg/s |

### Case 3: the damping law in each code's file

Each code's printed moments against the test's law on its own printed density, speed and rates, over the first 8 s: the largest relative difference about roll, pitch and yaw, on the inertial rate and on the rate relative to the Earth.

| Code | On the inertial rate | On the rate relative to the Earth |
|---|---|---|
| SIM 1 | 1.348e-4, 1.015e-2, 3.508e-6 | 8.824e-3, 4.066e-2, 1.009e-4 |
| SIM 2 | 2.264e-1, 1.333e-1, 1.103e-1 | 2.350e-1, 1.899e-1, 1.103e-1 |
| SIM 4 | 3.000e-5, 8.115e-11, 3.000e-5 | 9.162e-3, 4.967e-2, 1.283e-4 |
| SIM 5 | 9.200e-3, 7.917e-2, 9.835e-5 | 9.992e-16, 1.110e-15, 8.882e-16 |
| SIM 6 | 9.308e-3, 4.533e-2, 9.834e-5 | 1.159e-10, 1.141e-10, 1.141e-10 |

## Case 6: the sphere with drag

Volume II, Tables 1, 2 and 30; 6 codes. The model difference: WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less gravitation cut off after J2 with the centrifugal term and the code's own air density as its file gives it, which the code flies in: a point mass flown in each, with the same drag and wind, one position less the other.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| position | 273.1 mm | SIM 3 | 85.03 mm | 102.6 mm | 180.2 mm | 77.57 mm | past the spread; traced to within it |

### Case 6: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| position | 77.57 mm | 0 mm | 0.007677 mm | 0.003182 mm |

### Case 6, position: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | Its air density less the 1976 atmosphere's, largest, relative | Left with the gravity alone |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | 102.6 mm | 30 | 180.2 mm | 77.57 mm | 77.57 mm | 77.57 mm | +1.351e-3 | 85.04 mm |
| SIM 2 | 301 | kept | 58.46 mm | 30 | 58.58 mm | 2.342 mm | 8.196 mm | 8.196 mm | +3.496e-4 | 40.92 mm |
| SIM 3 | 301 | left out | 170.5 mm | 30 | 17.59 mm | 188.0 mm | 188.0 mm | 188.0 mm | -2.836e-3 | 188.0 mm |
| SIM 4 | 301 | kept | 17.57 mm | 30 | 17.53 mm | 0.03922 mm | 0.03922 mm | 0.03994 mm | -4.031e-7 | 0.01406 mm |
| SIM 5 | 3001 | kept | 17.86 mm | 30 | 17.87 mm | 0.007677 mm | 0.007677 mm | 0.006950 mm | +2.429e-6 | 0.3114 mm |
| SIM 6 | 301 | kept | 20.82 mm | 30 | 20.81 mm | 0.002454 mm | 0.01062 mm | 0.003182 mm | +2.071e-5 | 3.264 mm |

### Case 6: each code's drag on its own density

The largest relative difference between a code's printed drag force and ½ρV²SC_D on its own printed density and speed, in still air.

| Code | Largest relative difference |
|---|--:|
| SIM 1 | 1.092e-11 |
| SIM 2 | 1.903e-1 |
| SIM 3 | 3.661e-7 |
| SIM 4 | 2.081e-7 |
| SIM 5 | 1.554e-15 |
| SIM 6 | 1.110e-15 |

## Case 7: the sphere in a steady wind

Volume II, Tables 1, 2 and 31; 6 codes. The model difference: WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less gravitation cut off after J2 with the centrifugal term and the code's own air density as its file gives it, which the code flies in: a point mass flown in each, with the same drag and wind, one position less the other.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| position | 273.5 mm | SIM 3 | 85.24 mm | 102.8 mm | 180.4 mm | 77.63 mm | past the spread; traced to within it |

### Case 7: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| position | 77.63 mm | 0 mm | 0.01569 mm | 0.01491 mm |

### Case 7, position: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | Its air density less the 1976 atmosphere's, largest, relative | Left with the gravity alone |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | 102.8 mm | 30 | 180.4 mm | 77.63 mm | 77.63 mm | 77.63 mm | +1.351e-3 | 85.25 mm |
| SIM 2 | 301 | kept | 58.47 mm | 30 | 58.65 mm | 2.895 mm | 7.260 mm | 7.260 mm | +3.497e-4 | 40.92 mm |
| SIM 3 | 301 | left out | 170.7 mm | 30 | 17.59 mm | 188.3 mm | 188.3 mm | 188.3 mm | -2.836e-3 | 188.3 mm |
| SIM 4 | 301 | kept | 17.56 mm | 30 | 17.53 mm | 0.03127 mm | 0.03127 mm | 0.03205 mm | -4.031e-7 | 0.006026 mm |
| SIM 5 | 3001 | kept | 17.86 mm | 30 | 17.87 mm | 0.01569 mm | 0.01569 mm | 0.01491 mm | +2.429e-6 | 0.3039 mm |
| SIM 6 | 301 | kept | 20.81 mm | 30 | 20.82 mm | 0.01624 mm | 0.01642 mm | 0.01598 mm | +2.071e-5 | 3.262 mm |

## Case 8: the sphere in a wind that varies with height

Volume II, Tables 1, 2 and 32; 6 codes. The model difference: WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less gravitation cut off after J2 with the centrifugal term and the code's own air density as its file gives it, which the code flies in: a point mass flown in each, with the same drag and wind, one position less the other.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| position | 275.5 mm | SIM 3 | 87.74 mm | 105.0 mm | 182.3 mm | 77.68 mm | past the spread; traced to within it |

### Case 8: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| position | 77.68 mm | 0 mm | 0.004812 mm | 0.004751 mm |

### Case 8, position: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | Its air density less the 1976 atmosphere's, largest, relative | Left with the gravity alone |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | 105.0 mm | 30 | 182.3 mm | 77.68 mm | 77.68 mm | 77.68 mm | +1.351e-3 | 87.71 mm |
| SIM 2 | 301 | kept | 61.64 mm | 30 | 59.16 mm | 9.562 mm | 9.562 mm | 9.562 mm | +3.500e-4 | 44.78 mm |
| SIM 3 | 301 | left out | 170.5 mm | 30 | 17.59 mm | 188.0 mm | 188.0 mm | 188.0 mm | -2.835e-3 | 187.9 mm |
| SIM 4 | 301 | kept | 17.53 mm | 30 | 17.53 mm | 0.004812 mm | 0.004812 mm | 0.004751 mm | -4.031e-7 | 0.02960 mm |
| SIM 5 | 3001 | kept | 17.83 mm | 30 | 17.88 mm | 0.05101 mm | 0.05101 mm | 0.05095 mm | +2.428e-6 | 0.2719 mm |
| SIM 6 | 301 | kept | 20.88 mm | 30 | 20.85 mm | 0.4320 mm | 0.4320 mm | 0.4319 mm | +2.071e-5 | 3.422 mm |

## Case 9: the sphere shot east

Volume II, Tables 1, 2 and 33; 6 codes. The model difference: WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less gravitation cut off after J2 with the centrifugal term and the code's own air density as its file gives it, which the code flies in: a point mass flown in each, with the same drag and wind, one position less the other.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| position | 1989 mm | none | 1989 mm | 1980 mm | 1253 mm | 1991 mm | within the spread |

### Case 9: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| position | 1991 mm | 2.181 mm | 0.03830 mm | 0.02862 mm |

### Case 9, position: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | Its air density less the 1976 atmosphere's, largest, relative | Left with the gravity alone |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | 1859 mm | 30 | 1253 mm | 628.5 mm | 628.5 mm | 628.5 mm | +9.996e-4 | 1870 mm |
| SIM 2 | 301 | kept | 672.4 mm | 30 | 672.5 mm | 1.693 mm | 2.270 mm | 2.273 mm | +9.107e-4 | 682.8 mm |
| SIM 3 | 301 | kept | 1980 mm | 30 | 16.87 mm | 1991 mm | 1991 mm | 1991 mm | -5.668e-4 | 1991 mm |
| SIM 4 | 301 | kept | 15.60 mm | 30 | 15.81 mm | 0.4177 mm | 0.4177 mm | 0.4207 mm | +8.649e-7 | 1.788 mm |
| SIM 5 | 3001 | kept | 13.68 mm | 30 | 13.69 mm | 0.02551 mm | 0.03830 mm | 0.02862 mm | +3.884e-6 | 7.032 mm |
| SIM 6 | 301 | kept | 131.2 mm | 30 | 14.36 mm | 130.5 mm | 130.5 mm | 130.5 mm | +9.126e-6 | 142.3 mm |

### Case 9: each code's drag on its own density

The largest relative difference between a code's printed drag force and ½ρV²SC_D on its own printed density and speed, in still air.

| Code | Largest relative difference |
|---|--:|
| SIM 1 | 5.500e-12 |
| SIM 2 | 1.259e-3 |
| SIM 3 | 3.661e-7 |
| SIM 4 | 2.081e-7 |
| SIM 5 | 1.332e-15 |
| SIM 6 | 5.329e-15 |

## Case 10: the sphere shot north

Volume II, Tables 1, 2 and 34; 6 codes. The model difference: WGS 84's normal gravity and the 1976 atmosphere, which HPR Sim flies in, less gravitation cut off after J2 with the centrifugal term and the code's own air density as its file gives it, which the code flies in: a point mass flown in each, with the same drag and wind, one position less the other.

| Quantity | Spread of all codes | Code left out | Spread of the rest | Largest difference from a kept code | Model difference | Most left of a largest difference | Reading |
|---|--:|---|--:|--:|--:|--:|---|
| position | 46510 mm | SIM 2 | 1986 mm | 1977 mm | 1251 mm | 1988 mm | within the spread |

### Case 10: the stricter figure, at every sample

| Quantity | Largest unexplained part from a kept code | Its part over the spread | From the nearest kept code | The same at tight tolerances |
|---|--:|--:|--:|--:|
| position | 1988 mm | 2.180 mm | 0.04217 mm | 0.03268 mm |

### Case 10, position: each code

| Code | Samples | Kept | Largest difference | At, s | Model difference there | Left of it | Largest unexplained part | The same at tight tolerances | Its air density less the 1976 atmosphere's, largest, relative | Left with the gravity alone |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| SIM 1 | 301 | kept | 1858 mm | 30 | 1251 mm | 628.6 mm | 628.6 mm | 628.6 mm | +9.995e-4 | 1868 mm |
| SIM 2 | 301 | left out | 46510 mm | 30 | 672.2 mm | 45990 mm | 45990 mm | 45990 mm | +9.106e-4 | 46510 mm |
| SIM 3 | 301 | kept | 1977 mm | 30 | 16.87 mm | 1988 mm | 1988 mm | 1988 mm | -5.751e-4 | 1988 mm |
| SIM 4 | 301 | kept | 15.60 mm | 30 | 15.81 mm | 0.4151 mm | 0.4154 mm | 0.4176 mm | +8.649e-7 | 1.786 mm |
| SIM 5 | 3001 | kept | 13.68 mm | 30 | 13.69 mm | 0.02633 mm | 0.04217 mm | 0.03268 mm | +3.884e-6 | 7.035 mm |
| SIM 6 | 301 | kept | 337.9 mm | 0.3 | 0.001594 mm | 337.9 mm | 337.9 mm | 337.9 mm | +9.108e-6 | 337.9 mm |

### Case 10: each code's drag on its own density

The largest relative difference between a code's printed drag force and ½ρV²SC_D on its own printed density and speed, in still air.

| Code | Largest relative difference |
|---|--:|
| SIM 1 | 6.177e-12 |
| SIM 2 | 1.259e-3 |
| SIM 3 | 3.661e-7 |
| SIM 4 | 2.081e-7 |
| SIM 5 | 1.332e-15 |
| SIM 6 | 3.109e-15 |
