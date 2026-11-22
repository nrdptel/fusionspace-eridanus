# HPR Sim's tip-off phase against RocketPy's

Written by `cargo xtask tip-off-model`; don't edit it by hand. What it measures and how far to trust it: [rigid-body flight](../../docs/physics/flight.md#tip-off). It is one code against another, on 26 flights of thirteen designs; RocketPy's phase is in its development branch, in no release, and no real flight checks either. RocketPy's side is [its own report](tip-off.md) (rocketpy develop at 65b9838f23f62ce6155ac712798ccab01c45ea47 (pull request #1217's merge); as the pinned commit has them: it carries pull request #1196's correction to the sign of the center-of-mass vector, which the reference's script (validation/oracles/rocketpy/tip_off.py) checks on the commit's source before it flies, and #1188's to the nozzle's jet-damping lever).

How to read it:

- **Guided** is HPR Sim with its tip-off phase off: held straight until its last guide leaves the rail. **Tip-off** adds the phase: once the guide ahead of the last one is off the rail, the rocket slides on, free to pitch and yaw about its last guide with roll held. **Moved** is the flight with the phase less the guided one, in each code against its own guided flight.
- **On RocketPy's normal force,** HPR Sim flies the normal force RocketPy flew, which the reference records for each rocket: a slope against Mach number times the angle of attack, at a center of pressure against Mach number. Both codes already fly one declared drag coefficient. So the phase meets one drag and one normal force in both codes; damping in pitch and yaw stays each code's own. All of these are held to the bars.
- **On its own normal force,** HPR Sim flies its own aerodynamics. In the crosswind the rocket meets the air at 9.7° to 25.6° as it leaves the rail, where HPR Sim's normal force and RocketPy's are far apart, and the turn about the last guide follows the force. The still-air flights are held to the bars. The crosswind flights are past them and are not held: they are open work ([the decision record](../../docs/decisions/0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md)). Every figure past a bar is in bold.
- **Where the last guide leaves:** the rate at which the nose's direction moves and the angle of the rocket's axis off the rail's, each as two parts: nose down, below the rail's line in the rail's vertical plane, and nose right, to the right of the rail's heading, which is downwind in the crosswind. **Apart** is the length of the difference of the two codes' pairs, beside its bar.
- **A landing point's move** is east and north. **Farther from the pad** is how much farther from the pad the rocket lands with the phase, negative when nearer.
- **The bars:** the turning rate within 10% of RocketPy's or 0.1 deg/s, whichever is larger; the angle within 10% or 0.005°; the apogee moves within 0.2% of RocketPy's guided apogee; the landing-point moves, as vectors, within 1% of it.

## The reading

- **On RocketPy's normal force:** 26 of 26 flights inside every bar (13 of 13 in still air, 13 of 13 in the crosswind). The turning rate: 0 past its bar; the largest difference is 0.003 deg/s against a bar of 0.238 deg/s (juno-iii, crosswind). The angle off the rail: 0 past its bar; the largest difference is 0.0001° against a bar of 0.0083° (juno-iii, crosswind). The apogee move: 0 past its bar; the largest difference is 0.002% of the apogee against a bar of 0.200% of the apogee (juno-iii, crosswind). The landing-point move: 0 past its bar; the largest difference is 0.029% of the apogee against a bar of 1.000% of the apogee (valetudo, crosswind).
- **On HPR Sim's own normal force:** 13 of 26 flights inside every bar (13 of 13 in still air, 0 of 13 in the crosswind). The turning rate: 13 past its bar; the largest difference is 4.010 deg/s against a bar of 0.100 deg/s (bella-lui, crosswind). The angle off the rail: 10 past its bar; the largest difference is 0.0671° against a bar of 0.0050° (bella-lui, crosswind). The apogee move: 3 past its bar; the largest difference is 0.379% of the apogee against a bar of 0.200% of the apogee (bella-lui, crosswind). The landing-point move: 4 past its bar; the largest difference is 4.082% of the apogee against a bar of 1.000% of the apogee (valetudo, crosswind).
- **Which way leaving the phase out errs,** counting a move of under 0.1 m toward or away from the pad as none: with the phase HPR Sim lands farther from the pad on 22, nearer on 3 and the same distance on 1 on RocketPy's normal force, and farther from the pad on 20, nearer on 5 and the same distance on 1 on its own; RocketPy lands farther from the pad on 22, nearer on 3 and the same distance on 1.
- **Held:** every figure on RocketPy's normal force is inside its bar; every still-air figure on HPR Sim's own normal force is inside its bar. **Not held:** the crosswind flights on HPR Sim's own normal force, 0 of 13 inside every bar.

## On RocketPy's normal force

| Rocket | Wind | Turning rate, HPR Sim (down, right) | RocketPy | Apart, deg/s | Angle off the rail, HPR Sim (down, right) | RocketPy | Apart, ° |
|---|---|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | +0.717, -0.003 | +0.717, -0.003 | 0.000 of 0.100 | +0.0084, 0.0000 | +0.0084, 0.0000 | 0.0000 of 0.0050 |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | +0.717, +0.174 | +0.717, +0.174 | 0.001 of 0.100 | +0.0084, +0.0020 | +0.0084, +0.0020 | 0.0000 of 0.0050 |
| calisto-tests-motor-at-minus-1.373 | still | +0.659, -0.003 | +0.659, -0.003 | 0.000 of 0.100 | +0.0078, 0.0000 | +0.0078, 0.0000 | 0.0000 of 0.0050 |
| calisto-tests-motor-at-minus-1.373 | crosswind | +0.659, +0.129 | +0.659, +0.129 | 0.001 of 0.100 | +0.0078, +0.0015 | +0.0078, +0.0015 | 0.0000 of 0.0050 |
| bella-lui | still | +0.543, -0.004 | +0.543, -0.004 | 0.000 of 0.100 | +0.0091, -0.0001 | +0.0091, -0.0001 | 0.0000 of 0.0050 |
| bella-lui | crosswind | +0.542, +0.263 | +0.543, +0.264 | 0.001 of 0.100 | +0.0091, +0.0044 | +0.0091, +0.0044 | 0.0000 of 0.0050 |
| ndrt-2020-nose-to-tail | still | 0.000, 0.000 | 0.000, 0.000 | 0.000 of 0.100 | 0.0000, 0.0000 | 0.0000, 0.0000 | 0.0000 of 0.0050 |
| ndrt-2020-nose-to-tail | crosswind | 0.000, -0.180 | 0.000, -0.180 | 0.000 of 0.100 | 0.0000, -0.0025 | 0.0000, -0.0025 | 0.0000 of 0.0050 |
| valetudo | still | +2.089, -0.004 | +2.090, -0.004 | 0.001 of 0.209 | +0.0561, -0.0001 | +0.0561, -0.0001 | 0.0001 of 0.0056 |
| valetudo | crosswind | +2.089, +0.690 | +2.090, +0.691 | 0.002 of 0.220 | +0.0561, +0.0182 | +0.0561, +0.0183 | 0.0001 of 0.0059 |
| juno-iii | still | +2.254, +0.002 | +2.255, +0.002 | 0.002 of 0.226 | +0.0788, +0.0001 | +0.0789, +0.0001 | 0.0001 of 0.0079 |
| juno-iii | crosswind | +2.253, +0.766 | +2.255, +0.767 | 0.003 of 0.238 | +0.0788, +0.0262 | +0.0789, +0.0263 | 0.0001 of 0.0083 |
| cavour | still | +0.786, +0.003 | +0.786, +0.003 | 0.000 of 0.100 | +0.0084, 0.0000 | +0.0084, 0.0000 | 0.0000 of 0.0050 |
| cavour | crosswind | +0.786, +1.064 | +0.786, +1.065 | 0.001 of 0.132 | +0.0084, +0.0113 | +0.0084, +0.0113 | 0.0000 of 0.0050 |
| genesis | still | +0.603, +0.002 | +0.604, +0.002 | 0.000 of 0.100 | +0.0055, 0.0000 | +0.0055, 0.0000 | 0.0000 of 0.0050 |
| genesis | crosswind | +0.603, +0.746 | +0.604, +0.747 | 0.001 of 0.100 | +0.0055, +0.0068 | +0.0055, +0.0068 | 0.0000 of 0.0050 |
| lince | still | +1.183, +0.005 | +1.182, +0.005 | 0.001 of 0.118 | +0.0163, +0.0001 | +0.0162, +0.0001 | 0.0000 of 0.0050 |
| lince | crosswind | +1.183, +0.916 | +1.182, +0.917 | 0.001 of 0.150 | +0.0163, +0.0125 | +0.0162, +0.0125 | 0.0000 of 0.0050 |
| astra | still | +0.114, 0.000 | +0.114, 0.000 | 0.000 of 0.100 | +0.0005, 0.0000 | +0.0005, 0.0000 | 0.0000 of 0.0050 |
| astra | crosswind | +0.114, -0.325 | +0.114, -0.325 | 0.000 of 0.100 | +0.0005, -0.0013 | +0.0005, -0.0013 | 0.0000 of 0.0050 |
| andromeda | still | +0.488, +0.002 | +0.488, +0.002 | 0.000 of 0.100 | +0.0038, 0.0000 | +0.0038, 0.0000 | 0.0000 of 0.0050 |
| andromeda | crosswind | +0.488, -0.704 | +0.488, -0.704 | 0.000 of 0.100 | +0.0038, -0.0054 | +0.0038, -0.0054 | 0.0000 of 0.0050 |
| erebus-11 | still | +0.624, +0.002 | +0.624, +0.002 | 0.000 of 0.100 | +0.0050, 0.0000 | +0.0050, 0.0000 | 0.0000 of 0.0050 |
| erebus-11 | crosswind | +0.624, +0.337 | +0.624, +0.338 | 0.000 of 0.100 | +0.0050, +0.0027 | +0.0050, +0.0027 | 0.0000 of 0.0050 |
| prometheus-2022-generic-motor | still | +1.489, 0.000 | +1.489, 0.000 | 0.001 of 0.149 | +0.0134, 0.0000 | +0.0134, 0.0000 | 0.0000 of 0.0050 |
| prometheus-2022-generic-motor | crosswind | +1.488, +0.540 | +1.490, +0.541 | 0.001 of 0.158 | +0.0134, +0.0048 | +0.0134, +0.0048 | 0.0000 of 0.0050 |

| Rocket | Wind | Apogee moved, HPR Sim | RocketPy | Apart, % of the apogee | Landing point moved, HPR Sim (east, north) | RocketPy | Apart, % of the apogee | Farther from the pad, HPR Sim | RocketPy |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | -1.57 m | -1.58 m | 0.000 of 0.200 | 0.0, +16.3 m | 0.0, +16.3 m | 0.000 of 1.000 | +16.3 m | +16.3 m |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | -1.06 m | -1.07 m | 0.000 of 0.200 | +3.6, +15.9 m | +3.5, +16.1 m | 0.008 of 1.000 | +8.3 m | +8.3 m |
| calisto-tests-motor-at-minus-1.373 | still | -1.61 m | -1.62 m | 0.000 of 0.200 | 0.0, +16.8 m | 0.0, +16.9 m | 0.005 of 1.000 | +16.8 m | +16.9 m |
| calisto-tests-motor-at-minus-1.373 | crosswind | -1.21 m | -1.22 m | 0.000 of 0.200 | +2.9, +16.4 m | +3.0, +16.4 m | 0.002 of 1.000 | +7.6 m | +7.7 m |
| bella-lui | still | -0.02 m | -0.02 m | 0.000 of 0.200 | +0.9, +0.9 m | +0.9, +0.9 m | 0.005 of 1.000 | +1.3 m | +1.3 m |
| bella-lui | crosswind | +0.10 m | +0.10 m | 0.001 of 0.200 | +1.1, +0.4 m | +1.1, +0.4 m | 0.009 of 1.000 | -0.2 m | -0.3 m |
| ndrt-2020-nose-to-tail | still | 0.00 m | +0.01 m | 0.001 of 0.200 | 0.0, 0.0 m | 0.0, 0.0 m | 0.000 of 1.000 | 0.0 m | 0.0 m |
| ndrt-2020-nose-to-tail | crosswind | -0.27 m | -0.27 m | 0.000 of 0.200 | +3.0, -0.1 m | +2.9, -0.1 m | 0.009 of 1.000 | -3.0 m | -2.9 m |
| valetudo | still | -2.68 m | -2.68 m | 0.001 of 0.200 | +22.3, +16.9 m | +22.5, +17.0 m | 0.020 of 1.000 | +28.0 m | +28.2 m |
| valetudo | crosswind | -1.49 m | -1.48 m | 0.001 of 0.200 | +26.7, +9.4 m | +26.9, +9.5 m | 0.029 of 1.000 | +26.5 m | +26.7 m |
| juno-iii | still | -10.88 m | -10.86 m | 0.001 of 0.200 | +88.3, -23.8 m | +88.2, -23.8 m | 0.005 of 1.000 | +91.5 m | +91.3 m |
| juno-iii | crosswind | -3.95 m | -3.90 m | 0.002 of 0.200 | +76.3, -50.7 m | +76.5, -51.1 m | 0.018 of 1.000 | +83.7 m | +83.8 m |
| cavour | still | -1.03 m | -1.03 m | 0.000 of 0.200 | +10.0, -9.4 m | +10.0, -9.4 m | 0.001 of 1.000 | +13.8 m | +13.7 m |
| cavour | crosswind | +0.44 m | +0.46 m | 0.001 of 0.200 | -2.2, -22.3 m | -2.2, -22.5 m | 0.005 of 1.000 | +1.1 m | +1.0 m |
| genesis | still | -1.20 m | -1.20 m | 0.000 of 0.200 | +7.6, -7.1 m | +7.5, -7.1 m | 0.002 of 1.000 | +10.4 m | +10.3 m |
| genesis | crosswind | +0.31 m | +0.32 m | 0.001 of 0.200 | -1.0, -16.6 m | -1.1, -16.7 m | 0.005 of 1.000 | +16.6 m | +16.7 m |
| lince | still | -2.57 m | -2.58 m | 0.000 of 0.200 | +14.1, -13.3 m | +14.0, -13.2 m | 0.002 of 1.000 | +19.3 m | +19.3 m |
| lince | crosswind | -0.72 m | -0.70 m | 0.000 of 0.200 | +4.5, -23.7 m | +4.3, -23.8 m | 0.004 of 1.000 | +17.5 m | +17.6 m |
| astra | still | -0.20 m | -0.20 m | 0.000 of 0.200 | +2.2, -2.0 m | +2.2, -2.0 m | 0.000 of 1.000 | +3.0 m | +3.0 m |
| astra | crosswind | -0.82 m | -0.83 m | 0.000 of 0.200 | +7.2, +4.0 m | +7.2, +4.0 m | 0.001 of 1.000 | +6.8 m | +6.8 m |
| andromeda | still | -0.58 m | -0.58 m | 0.000 of 0.200 | +3.4, -3.2 m | +3.4, -3.2 m | 0.002 of 1.000 | +4.6 m | +4.7 m |
| andromeda | crosswind | -1.58 m | -1.58 m | 0.000 of 0.200 | +7.6, +2.1 m | +7.6, +2.1 m | 0.002 of 1.000 | -3.6 m | -3.6 m |
| erebus-11 | still | -0.98 m | -0.98 m | 0.000 of 0.200 | +9.5, -9.0 m | +9.5, -9.0 m | 0.001 of 1.000 | +13.1 m | +13.1 m |
| erebus-11 | crosswind | -0.37 m | -0.36 m | 0.000 of 0.200 | +4.5, -13.4 m | +4.5, -13.5 m | 0.004 of 1.000 | +5.9 m | +5.9 m |
| prometheus-2022-generic-motor | still | -13.09 m | -13.12 m | 0.001 of 0.200 | +52.0, +13.9 m | +52.1, +13.9 m | 0.002 of 1.000 | +53.9 m | +54.0 m |
| prometheus-2022-generic-motor | crosswind | -9.21 m | -9.25 m | 0.001 of 0.200 | +57.1, -5.5 m | +57.9, -5.7 m | 0.021 of 1.000 | +57.1 m | +57.9 m |

## On HPR Sim's own normal force

| Rocket | Wind | Turning rate, HPR Sim (down, right) | RocketPy | Apart, deg/s | Angle off the rail, HPR Sim (down, right) | RocketPy | Apart, ° |
|---|---|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | +0.717, -0.003 | +0.717, -0.003 | 0.000 of 0.100 | +0.0084, 0.0000 | +0.0084, 0.0000 | 0.0000 of 0.0050 |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | +0.717, +0.846 | +0.717, +0.174 | **0.671** of 0.100 | +0.0084, +0.0099 | +0.0084, +0.0020 | **0.0078** of 0.0050 |
| calisto-tests-motor-at-minus-1.373 | still | +0.659, -0.003 | +0.659, -0.003 | 0.000 of 0.100 | +0.0078, 0.0000 | +0.0078, 0.0000 | 0.0000 of 0.0050 |
| calisto-tests-motor-at-minus-1.373 | crosswind | +0.659, +0.647 | +0.659, +0.129 | **0.518** of 0.100 | +0.0078, +0.0077 | +0.0078, +0.0015 | **0.0061** of 0.0050 |
| bella-lui | still | +0.543, -0.004 | +0.543, -0.004 | 0.000 of 0.100 | +0.0091, -0.0001 | +0.0091, -0.0001 | 0.0000 of 0.0050 |
| bella-lui | crosswind | +0.541, +4.274 | +0.543, +0.264 | **4.010** of 0.100 | +0.0091, +0.0715 | +0.0091, +0.0044 | **0.0671** of 0.0050 |
| ndrt-2020-nose-to-tail | still | 0.000, 0.000 | 0.000, 0.000 | 0.000 of 0.100 | 0.0000, 0.0000 | 0.0000, 0.0000 | 0.0000 of 0.0050 |
| ndrt-2020-nose-to-tail | crosswind | 0.000, -0.044 | 0.000, -0.180 | **0.137** of 0.100 | 0.0000, -0.0006 | 0.0000, -0.0025 | 0.0019 of 0.0050 |
| valetudo | still | +2.089, -0.004 | +2.090, -0.004 | 0.001 of 0.209 | +0.0561, -0.0001 | +0.0561, -0.0001 | 0.0001 of 0.0056 |
| valetudo | crosswind | +2.087, +2.617 | +2.090, +0.691 | **1.927** of 0.220 | +0.0560, +0.0700 | +0.0561, +0.0183 | **0.0517** of 0.0059 |
| juno-iii | still | +2.254, +0.002 | +2.255, +0.002 | 0.002 of 0.226 | +0.0788, +0.0001 | +0.0789, +0.0001 | 0.0001 of 0.0079 |
| juno-iii | crosswind | +2.251, +1.995 | +2.255, +0.767 | **1.227** of 0.238 | +0.0787, +0.0692 | +0.0789, +0.0263 | **0.0429** of 0.0083 |
| cavour | still | +0.786, +0.003 | +0.786, +0.003 | 0.000 of 0.100 | +0.0084, 0.0000 | +0.0084, 0.0000 | 0.0000 of 0.0050 |
| cavour | crosswind | +0.785, +2.206 | +0.786, +1.065 | **1.141** of 0.132 | +0.0084, +0.0236 | +0.0084, +0.0113 | **0.0122** of 0.0050 |
| genesis | still | +0.603, +0.002 | +0.604, +0.002 | 0.000 of 0.100 | +0.0055, 0.0000 | +0.0055, 0.0000 | 0.0000 of 0.0050 |
| genesis | crosswind | +0.603, +1.524 | +0.604, +0.747 | **0.777** of 0.100 | +0.0055, +0.0138 | +0.0055, +0.0068 | **0.0071** of 0.0050 |
| lince | still | +1.183, +0.005 | +1.182, +0.005 | 0.001 of 0.118 | +0.0163, +0.0001 | +0.0162, +0.0001 | 0.0000 of 0.0050 |
| lince | crosswind | +1.182, +1.801 | +1.182, +0.917 | **0.885** of 0.150 | +0.0163, +0.0246 | +0.0162, +0.0125 | **0.0122** of 0.0050 |
| astra | still | +0.114, 0.000 | +0.114, 0.000 | 0.000 of 0.100 | +0.0005, 0.0000 | +0.0005, 0.0000 | 0.0000 of 0.0050 |
| astra | crosswind | +0.114, -0.153 | +0.114, -0.325 | **0.172** of 0.100 | +0.0005, -0.0006 | +0.0005, -0.0013 | 0.0007 of 0.0050 |
| andromeda | still | +0.488, +0.002 | +0.488, +0.002 | 0.000 of 0.100 | +0.0038, 0.0000 | +0.0038, 0.0000 | 0.0000 of 0.0050 |
| andromeda | crosswind | +0.488, -0.462 | +0.488, -0.704 | **0.242** of 0.100 | +0.0038, -0.0036 | +0.0038, -0.0054 | 0.0019 of 0.0050 |
| erebus-11 | still | +0.624, +0.002 | +0.624, +0.002 | 0.000 of 0.100 | +0.0050, 0.0000 | +0.0050, 0.0000 | 0.0000 of 0.0050 |
| erebus-11 | crosswind | +0.624, +1.342 | +0.624, +0.338 | **1.005** of 0.100 | +0.0050, +0.0107 | +0.0050, +0.0027 | **0.0080** of 0.0050 |
| prometheus-2022-generic-motor | still | +1.489, 0.000 | +1.489, 0.000 | 0.001 of 0.149 | +0.0134, 0.0000 | +0.0134, 0.0000 | 0.0000 of 0.0050 |
| prometheus-2022-generic-motor | crosswind | +1.488, +1.188 | +1.490, +0.541 | **0.647** of 0.158 | +0.0134, +0.0106 | +0.0134, +0.0048 | **0.0058** of 0.0050 |

| Rocket | Wind | Apogee moved, HPR Sim | RocketPy | Apart, % of the apogee | Landing point moved, HPR Sim (east, north) | RocketPy | Apart, % of the apogee | Farther from the pad, HPR Sim | RocketPy |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | -1.58 m | -1.58 m | 0.000 of 0.200 | 0.0, +16.4 m | 0.0, +16.3 m | 0.003 of 1.000 | +16.4 m | +16.3 m |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | +0.62 m | -1.07 m | 0.066 of 0.200 | +20.9, +17.2 m | +3.5, +16.1 m | 0.677 of 1.000 | +25.0 m | +8.3 m |
| calisto-tests-motor-at-minus-1.373 | still | -1.61 m | -1.62 m | 0.000 of 0.200 | 0.0, +16.9 m | 0.0, +16.9 m | 0.002 of 1.000 | +16.9 m | +16.9 m |
| calisto-tests-motor-at-minus-1.373 | crosswind | +0.17 m | -1.22 m | 0.054 of 0.200 | +17.0, +17.2 m | +3.0, +16.4 m | 0.544 of 1.000 | +21.2 m | +7.7 m |
| bella-lui | still | -0.02 m | -0.02 m | 0.000 of 0.200 | +0.9, +0.9 m | +0.9, +0.9 m | 0.012 of 1.000 | +1.2 m | +1.3 m |
| bella-lui | crosswind | +1.95 m | +0.10 m | **0.379** of 0.200 | +7.0, -5.2 m | +1.1, +0.4 m | **1.678** of 1.000 | -7.8 m | -0.3 m |
| ndrt-2020-nose-to-tail | still | 0.00 m | +0.01 m | 0.001 of 0.200 | 0.0, 0.0 m | 0.0, 0.0 m | 0.000 of 1.000 | 0.0 m | 0.0 m |
| ndrt-2020-nose-to-tail | crosswind | -0.07 m | -0.27 m | 0.017 of 0.200 | +0.8, 0.0 m | +2.9, -0.1 m | 0.178 of 1.000 | -0.8 m | -2.9 m |
| valetudo | still | -2.66 m | -2.68 m | 0.003 of 0.200 | +22.8, +17.3 m | +22.5, +17.0 m | 0.060 of 1.000 | +28.6 m | +28.2 m |
| valetudo | crosswind | +0.75 m | -1.48 m | **0.295** of 0.200 | +47.9, -13.1 m | +26.9, +9.5 m | **4.082** of 1.000 | +49.4 m | +26.7 m |
| juno-iii | still | -11.33 m | -10.86 m | 0.018 of 0.200 | +92.1, -24.8 m | +88.2, -23.8 m | 0.154 of 1.000 | +95.4 m | +91.3 m |
| juno-iii | crosswind | +4.35 m | -3.90 m | **0.331** of 0.200 | +72.7, -109.7 m | +76.5, -51.1 m | **2.357** of 1.000 | +111.7 m | +83.8 m |
| cavour | still | -1.03 m | -1.03 m | 0.000 of 0.200 | +10.1, -9.6 m | +10.0, -9.4 m | 0.006 of 1.000 | +13.9 m | +13.7 m |
| cavour | crosswind | +2.06 m | +0.46 m | 0.051 of 0.200 | -17.5, -41.0 m | -2.2, -22.5 m | 0.771 of 1.000 | -8.8 m | +1.0 m |
| genesis | still | -1.21 m | -1.20 m | 0.000 of 0.200 | +7.6, -7.2 m | +7.5, -7.1 m | 0.004 of 1.000 | +10.5 m | +10.3 m |
| genesis | crosswind | +1.90 m | +0.32 m | 0.049 of 0.200 | -12.6, -31.3 m | -1.1, -16.7 m | 0.579 of 1.000 | +32.7 m | +16.7 m |
| lince | still | -2.58 m | -2.58 m | 0.000 of 0.200 | +14.1, -13.3 m | +14.0, -13.2 m | 0.003 of 1.000 | +19.4 m | +19.3 m |
| lince | crosswind | +0.99 m | -0.70 m | 0.045 of 0.200 | -7.2, -37.9 m | +4.3, -23.8 m | 0.485 of 1.000 | +35.7 m | +17.6 m |
| astra | still | -0.21 m | -0.20 m | 0.000 of 0.200 | +2.2, -2.1 m | +2.2, -2.0 m | 0.002 of 1.000 | +3.0 m | +3.0 m |
| astra | crosswind | -0.52 m | -0.83 m | 0.009 of 0.200 | +4.9, +1.1 m | +7.2, +4.0 m | 0.112 of 1.000 | +4.7 m | +6.8 m |
| andromeda | still | -0.58 m | -0.58 m | 0.000 of 0.200 | +3.4, -3.2 m | +3.4, -3.2 m | 0.001 of 1.000 | +4.6 m | +4.7 m |
| andromeda | crosswind | -1.24 m | -1.58 m | 0.012 of 0.200 | +6.2, +0.4 m | +7.6, +2.1 m | 0.075 of 1.000 | -1.6 m | -3.6 m |
| erebus-11 | still | -0.99 m | -0.98 m | 0.000 of 0.200 | +9.8, -9.3 m | +9.5, -9.0 m | 0.017 of 1.000 | +13.5 m | +13.1 m |
| erebus-11 | crosswind | +1.43 m | -0.36 m | 0.065 of 0.200 | -11.0, -32.7 m | +4.5, -13.5 m | 0.889 of 1.000 | -4.6 m | +5.9 m |
| prometheus-2022-generic-motor | still | -13.28 m | -13.12 m | 0.004 of 0.200 | +52.8, +14.1 m | +52.1, +13.9 m | 0.019 of 1.000 | +54.7 m | +54.0 m |
| prometheus-2022-generic-motor | crosswind | -7.48 m | -9.25 m | 0.048 of 0.200 | +75.7, -39.3 m | +57.9, -5.7 m | **1.025** of 1.000 | +84.3 m | +57.9 m |

## The normal force where the last guide leaves, in the crosswind

The air's angle to the rocket's axis as the last guide leaves, the normal-force coefficient each model gives there, and that force's moment about the last guide for each unit of dynamic pressure and reference area (the coefficient times its arm ahead of the guide, negative when the force acts behind it). The moment is what turns the nose: downwind where the force acts ahead of the guide, upwind where it acts behind. The nose-right turning rate follows it.

| Rocket | Speed | Angle of attack | Normal force, HPR Sim's own | RocketPy's | Moment about the guide, HPR Sim's own | RocketPy's | Nose-right rate, on its own | on RocketPy's | RocketPy |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | 31.1 m/s | 16.1° | 3.64 | 2.02 | +1.10 m | +0.24 m | +0.846 deg/s | +0.174 deg/s | +0.174 deg/s |
| calisto-tests-motor-at-minus-1.373 | 30.7 m/s | 16.2° | 3.38 | 1.73 | +0.87 m | +0.18 m | +0.647 deg/s | +0.129 deg/s | +0.129 deg/s |
| bella-lui | 18.6 m/s | 25.6° | 8.75 | 5.27 | +2.44 m | +0.16 m | +4.274 deg/s | +0.263 deg/s | +0.264 deg/s |
| ndrt-2020-nose-to-tail | 19.3 m/s | 24.8° | 5.31 | 2.70 | -0.42 m | -1.18 m | -0.044 deg/s | -0.180 deg/s | -0.180 deg/s |
| valetudo | 22.5 m/s | 21.6° | 6.56 | 2.42 | +5.33 m | +1.46 m | +2.617 deg/s | +0.690 deg/s | +0.691 deg/s |
| juno-iii | 21.5 m/s | 22.5° | 7.43 | 4.38 | +6.33 m | +2.52 m | +1.995 deg/s | +0.766 deg/s | +0.767 deg/s |
| cavour | 51.9 m/s | 9.8° | 2.63 | 1.79 | +2.06 m | +1.00 m | +2.206 deg/s | +1.064 deg/s | +1.065 deg/s |
| genesis | 49.4 m/s | 10.2° | 3.36 | 2.45 | +1.87 m | +0.92 m | +1.524 deg/s | +0.746 deg/s | +0.747 deg/s |
| lince | 52.4 m/s | 9.7° | 1.92 | 1.28 | +1.13 m | +0.58 m | +1.801 deg/s | +0.916 deg/s | +0.917 deg/s |
| astra | 49.2 m/s | 10.3° | 3.56 | 2.89 | -0.24 m | -0.49 m | -0.153 deg/s | -0.325 deg/s | -0.325 deg/s |
| andromeda | 45.9 m/s | 11.0° | 2.82 | 2.17 | -0.40 m | -0.59 m | -0.462 deg/s | -0.704 deg/s | -0.704 deg/s |
| erebus-11 | 44.6 m/s | 11.3° | 2.61 | 1.60 | +0.73 m | +0.19 m | +1.342 deg/s | +0.337 deg/s | +0.338 deg/s |
| prometheus-2022-generic-motor | 27.4 m/s | 18.1° | 4.16 | 2.65 | +2.18 m | +1.00 m | +1.188 deg/s | +0.540 deg/s | +0.541 deg/s |

The turn follows the moment, not the force's size alone. On these 13 flights HPR Sim's own normal-force coefficient is 1.23 to 2.71 times RocketPy's, and its moment about the last guide 0.35 to 15.20 times. Flown by HPR Sim on each, the nose-right turning rate on its own normal force is 0.24 to 16.25 times that on RocketPy's. On 10 the force acts ahead of the guide on both, and on 10 the nose turns downwind, farther on HPR Sim's own. On 3 it acts behind the guide on both, and on 3 the nose turns upwind, less far on HPR Sim's own, whose force acts nearer the guide on 3. In signed terms the rate is farther nose-right on HPR Sim's own in 13 of the 13, by 0.136 to 4.011 deg/s. Which of the two forces is nearer a real rocket's at these angles, no reference here says.
