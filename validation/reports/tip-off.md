# What RocketPy's tip-off phase moves

Written by `cargo xtask tip-off`; don't edit it by hand. What it measures and how far to trust it: [rigid-body flight](../../docs/physics/flight.md#what-leaving-tip-off-out-costs). It is one code against another: the phase is in RocketPy's development branch, in no release, and no flight checks it. By default HPR Sim frees a rocket with no turning rate where its last guide leaves the rail ([its opt-in tip-off phase against this report](tip-off-model.md)). This report measures the pivot about that guide which that leaves out, with RocketPy's tip-off phase: rocketpy develop at 65b9838f23f62ce6155ac712798ccab01c45ea47 ([pull request #1217](https://github.com/RocketPy-Team/RocketPy/pull/1217)'s merge). Equations: as the pinned commit has them: it carries [pull request #1196](https://github.com/RocketPy-Team/RocketPy/pull/1196)'s correction to the sign of the center-of-mass vector, which the reference's script (validation/oracles/rocketpy/tip_off.py) checks on the commit's source before it flies, and #1188's to the nozzle's jet-damping lever. Each example rocket of `validation/fixtures/design/rocketpy-rocket-mass.json` flies from its example's rail and site on a declared constant drag coefficient, in still air and in a 20 mph (8.94 m/s) wind square across the rail's heading, blowing toward its right, the same at every height.

How to read it:

- **Phase off** is RocketPy as its releases fly: the rocket is free where its upper button leaves the rail. The commit this report flies has no switch for the phase: it flies it in every flight that uses its standard six-degree-of-freedom equations and starts with both buttons on the rail, the lower one nearer the nozzle, so its rail length is the longer. So these flights take the commit's own branch for a rocket without a pair of buttons on the rail, reached by setting the lower button's rail length equal to the upper one's: free flight from where the upper button leaves.
- **Phase on** adds the tip-off phase: from there the rocket slides on along the rail, free to pitch and yaw about its lower button with roll held, until that button leaves too.
- **Guided** is the phase off on a rail longer by the distance between the buttons: the rocket is held straight until the lower button leaves the real rail, as HPR Sim holds it.
- **Rocket** is the design's id. The two Calisto ids are two versions of one rocket, from RocketPy's getting-started notebook and from its tests. Each ends in the motor's position in meters, in the rocket's own coordinates as its source sets it: `minus-1.255` is −1.255 m. Each source measures from its own zero point, and the two zero points are 0.118 m (4.6 in) apart. So in both the nozzle sits 2.533 m (8.31 ft) behind the nose's tip. The fins differ, and the notebook's rail buttons and its center of mass without the motor sit 0.118 m (4.6 in) farther back.
- **Rail** is the rail's length and its angle above the horizontal.
- **Moved** is the phase on less the other flight: the phase-on figure minus the other's. *On less off* takes the phase off from the phase on, and *on less guided* takes the guided flight. A landing point's move is the distance between the two landing points, in meters and as a share of the other flight's apogee. **Farther from the pad** is how much farther from the pad the rocket lands with the phase on, negative when it lands nearer. **Apogee point moved** is the distance across the ground between the two flights' apogee points; the change in height is in **Apogee moved**.
- **Where the lower button leaves:** the rocket's speed with the phase on, the angle between the rocket's axis and the rail's, its part below the rail's line in the rail's vertical plane (nose down), and the turning rate across the rocket's axis (pitch and yaw together).
- **The bars** are [the plan's](../../docs/decisions/0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md): a tip-off model is called for when the phase moves any apogee by more than 1%, or any landing point by more than 1% of the apogee.
- **A declared change** is a flight flown with one of its design's inputs changed, with the change and its reason stated.

## The reading

- **On less off, the plan's rule:** 1 of 26 apogees past 1% and 9 of 26 landing points past 1% of the apogee, on 6 of the designs; with the phase on the rocket lands farther from the pad on 22 of the 26 and nearer on 4. A tip-off model is called for.
  The phase moves the apogee by at most +1.165% (+5.6 m; bella-lui, crosswind), the landing point by at most 4.585% of the apogee (113.7 m; juno-iii, crosswind), and the pitch rate where the lower button leaves by at most -15.18 deg/s (bella-lui, crosswind).
- **On less guided, what HPR Sim's release leaves out:** 0 of 26 apogees past 1% and 6 of 26 landing points past 1% of the apogee, on 3 of the designs; with the phase on the rocket lands farther from the pad on 23 of the 26 and nearer on 3.
  The phase moves the apogee by at most -0.414% (-10.9 m; juno-iii, still), the landing point by at most 3.765% of the apogee (28.5 m; valetudo, crosswind), and the pitch rate where the lower button leaves by at most +2.38 deg/s (juno-iii, crosswind).
- **Since the earlier pin.** A pin is the one commit of RocketPy's source a report flies. This report first flew rocketpy develop at ebe1b775cd9270c93018dc6d7867313d6eed4e5c ([pull request #920](https://github.com/RocketPy-Team/RocketPy/pull/920)'s merge). Its equations: the pinned commit's with [pull request #1196](https://github.com/RocketPy-Team/RocketPy/pull/1196)'s sign correction (corrections.py, ADR-026). RocketPy then rewrote the phase ([pull request #1217](https://github.com/RocketPy-Team/RocketPy/pull/1217)), and this report flies that pull request's merge. The earlier report's data is kept as `validation/fixtures/flight/rocketpy-tip-off-at-pull-920.json`, a byte-for-byte copy of `validation/reports/tip-off.json` as commit `ee9a5af6` of this repository had it. Between the two pins, the same flights are past each bar, and the rocket lands farther from the pad in the same number of them. [The last table](#between-the-two-pins) gives each flight.
  - Where the lower button leaves with the phase on, the turning rate moved by at most 0.000020 deg/s (prometheus-2022-generic-motor, still) and the angle off the rail's axis by at most 0.0000003° (prometheus-2022-generic-motor, still), each as its nose-down and nose-right parts together.
  - What the phase moves from the guided flight changed by at most 0.000034% of the apogee in the apogee (ndrt-2020-nose-to-tail, still) and 0.000032% of the apogee in the landing point (calisto-tests-motor-at-minus-1.373, crosswind).
  - No flight's apogee moved by more than 1.04 mm (prometheus-2022-generic-motor, crosswind, phase off), nor its landing point by more than 0.96 mm (calisto-tests-motor-at-minus-1.373, crosswind, guided).
  - These are differences between two commits, not the measured effect of one change. By its description the rewrite simplified the phase's function (`udot_rail2`), changed what RocketPy reports inside the phase and corrected how the phase ends. It also finds every flight's rail exit a new way, by a root search on the solver's own interpolation of the step. Other changes to RocketPy lie between the two pins as well. Over all the flights without the phase (phase off and guided), an apogee or landing point moved by up to 1.04 mm; over those with it, by up to 0.81 mm. So the moves are not the phase's alone. The last two columns of the last table give the guided flights' moves alone. This report does not test which change made each.
  - Each rocket carried the same in both: its normal force at every Mach number, its dry mass, its motor's total impulse, burn time and propellant mass, and its parachutes.
- **A declared change** (ndrt-2020-nose-to-tail, still):
  - The change: the parachute that opens at apogee opens with no lag.
  - Why: on a vertical rail in still air RocketPy's integration of the tail-first fall through the lag had not finished after 20 s of wall time.
  - The probe (`tip_off.py --lag-probe`, 20 s of wall time) had integrated to 16.0 s. The apogee is at 15.4 s.

## What the phase moves: on less off

| Rocket | Wind | Rail | Between the buttons | Apogee, phase off | Apogee moved | Landing point moved | Of the apogee | Farther from the pad | Apogee point moved |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | 5.20 m at 85.0° | 0.700 m | 2614.3 m | -0.054% | 14.7 m | 0.563% | +14.7 m | 13.1 m |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | 5.20 m at 85.0° | 0.700 m | 2569.7 m | +0.020% | 21.2 m | 0.826% | +18.9 m | 18.8 m |
| calisto-tests-motor-at-minus-1.373 | still | 5.20 m at 85.0° | 0.700 m | 2614.3 m | -0.056% | 15.4 m | 0.589% | +15.4 m | 13.6 m |
| calisto-tests-motor-at-minus-1.373 | crosswind | 5.20 m at 85.0° | 0.700 m | 2575.6 m | +0.001% | 19.9 m | 0.773% | +16.8 m | 17.6 m |
| bella-lui | still | 4.20 m at 89.0° | 0.600 m | 540.1 m | -0.003% | 1.0 m | 0.176% | +1.0 m | 0.7 m |
| bella-lui | crosswind | 4.20 m at 89.0° | 0.600 m | 480.7 m | +1.165% | 20.3 m | 4.233% | -19.7 m | 14.7 m |
| ndrt-2020-nose-to-tail | still | 3.35 m at 90.0° | 0.500 m | 1219.4 m | 0.000% | 0.0 m | 0.000% | 0.0 m | 0.0 m |
| ndrt-2020-nose-to-tail | crosswind | 3.35 m at 90.0° | 0.500 m | 1204.0 m | +0.014% | 2.1 m | 0.173% | +2.1 m | 1.4 m |
| valetudo | still | 5.70 m at 84.7° | 1.154 m | 777.9 m | -0.322% | 26.2 m | 3.369% | +26.2 m | 21.5 m |
| valetudo | crosswind | 5.70 m at 84.7° | 1.154 m | 755.3 m | -0.019% | 30.9 m | 4.095% | +30.9 m | 25.5 m |
| juno-iii | still | 5.20 m at 85.0° | 1.410 m | 2623.0 m | -0.378% | 82.7 m | 3.154% | +82.7 m | 74.5 m |
| juno-iii | crosswind | 5.20 m at 85.0° | 1.410 m | 2480.1 m | +0.321% | 113.7 m | 4.585% | +70.4 m | 100.9 m |
| cavour | still | 12.00 m at 84.0° | 1.090 m | 3163.0 m | -0.025% | 10.8 m | 0.341% | +10.8 m | 6.1 m |
| cavour | crosswind | 12.00 m at 84.0° | 1.090 m | 3113.2 m | +0.099% | 47.6 m | 1.529% | -16.9 m | 27.4 m |
| genesis | still | 12.00 m at 84.0° | 0.885 m | 3265.0 m | -0.031% | 8.7 m | 0.266% | +8.7 m | 7.8 m |
| genesis | crosswind | 12.00 m at 84.0° | 0.885 m | 3216.8 m | +0.067% | 28.4 m | 0.882% | +26.9 m | 24.7 m |
| lince | still | 12.00 m at 84.0° | 1.405 m | 3802.8 m | -0.060% | 17.0 m | 0.448% | +17.0 m | 16.8 m |
| lince | crosswind | 12.00 m at 84.0° | 1.405 m | 3755.5 m | +0.054% | 40.5 m | 1.078% | +39.0 m | 37.8 m |
| astra | still | 12.00 m at 84.0° | 0.400 m | 3447.4 m | -0.003% | 1.6 m | 0.046% | +1.6 m | 0.8 m |
| astra | crosswind | 12.00 m at 84.0° | 0.400 m | 3391.9 m | +0.006% | 4.1 m | 0.121% | -1.1 m | 2.3 m |
| andromeda | still | 12.00 m at 84.0° | 0.700 m | 3054.6 m | -0.013% | 3.1 m | 0.102% | +3.1 m | 2.9 m |
| andromeda | crosswind | 12.00 m at 84.0° | 0.700 m | 2986.2 m | +0.034% | 9.4 m | 0.314% | +9.2 m | 8.4 m |
| erebus-11 | still | 12.00 m at 84.0° | 0.700 m | 2814.4 m | -0.029% | 10.9 m | 0.388% | +10.9 m | 6.1 m |
| erebus-11 | crosswind | 12.00 m at 84.0° | 0.700 m | 2763.1 m | +0.050% | 26.3 m | 0.951% | -5.6 m | 15.1 m |
| prometheus-2022-generic-motor | still | 5.18 m at 80.0° | 0.480 m | 3805.7 m | -0.322% | 50.6 m | 1.329% | +50.6 m | 47.4 m |
| prometheus-2022-generic-motor | crosswind | 5.18 m at 80.0° | 0.480 m | 3711.9 m | -0.171% | 61.6 m | 1.660% | +61.1 m | 57.2 m |

## Where the lower button leaves the rail

| Rocket | Wind | Speed, phase on | Off the rail's axis: off | on | guided | Nose down: off | on | Pitch rate: off | on | guided |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | 31.06 m/s | 0.000° | 0.008° | 0.000° | 0.000° | 0.008° | 0.00 deg/s | 0.72 deg/s | 0.00 deg/s |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | 31.05 m/s | 0.007° | 0.009° | 0.000° | 0.000° | 0.008° | 0.58 deg/s | 0.74 deg/s | 0.00 deg/s |
| calisto-tests-motor-at-minus-1.373 | still | 30.66 m/s | 0.000° | 0.008° | 0.000° | 0.000° | 0.008° | 0.00 deg/s | 0.66 deg/s | 0.00 deg/s |
| calisto-tests-motor-at-minus-1.373 | crosswind | 30.66 m/s | 0.005° | 0.008° | 0.000° | 0.000° | 0.008° | 0.43 deg/s | 0.67 deg/s | 0.00 deg/s |
| bella-lui | still | 18.64 m/s | 0.000° | 0.009° | 0.000° | 0.000° | 0.009° | 0.00 deg/s | 0.54 deg/s | 0.00 deg/s |
| bella-lui | crosswind | 18.63 m/s | 0.267° | 0.010° | 0.000° | 0.000° | 0.009° | 15.78 deg/s | 0.60 deg/s | 0.00 deg/s |
| ndrt-2020-nose-to-tail | still | 19.30 m/s | 0.000° | 0.000° | 0.000° | 0.000° | 0.000° | 0.00 deg/s | 0.00 deg/s | 0.00 deg/s |
| ndrt-2020-nose-to-tail | crosswind | 19.30 m/s | 0.005° | 0.002° | 0.000° | 0.000° | 0.000° | 0.36 deg/s | 0.18 deg/s | 0.00 deg/s |
| valetudo | still | 22.51 m/s | 0.000° | 0.056° | 0.000° | 0.000° | 0.056° | 0.00 deg/s | 2.09 deg/s | 0.00 deg/s |
| valetudo | crosswind | 22.51 m/s | 0.020° | 0.059° | 0.000° | 0.000° | 0.056° | 0.77 deg/s | 2.20 deg/s | 0.00 deg/s |
| juno-iii | still | 21.53 m/s | 0.000° | 0.079° | 0.000° | 0.000° | 0.079° | 0.00 deg/s | 2.26 deg/s | 0.00 deg/s |
| juno-iii | crosswind | 21.53 m/s | 0.054° | 0.083° | 0.000° | 0.000° | 0.079° | 1.54 deg/s | 2.38 deg/s | 0.00 deg/s |
| cavour | still | 51.94 m/s | 0.000° | 0.008° | 0.000° | 0.000° | 0.008° | 0.00 deg/s | 0.79 deg/s | 0.00 deg/s |
| cavour | crosswind | 51.93 m/s | 0.026° | 0.014° | 0.000° | 0.000° | 0.008° | 2.45 deg/s | 1.32 deg/s | 0.00 deg/s |
| genesis | still | 49.43 m/s | 0.000° | 0.005° | 0.000° | 0.000° | 0.005° | 0.00 deg/s | 0.60 deg/s | 0.00 deg/s |
| genesis | crosswind | 49.43 m/s | 0.010° | 0.009° | 0.000° | 0.000° | 0.005° | 1.09 deg/s | 0.96 deg/s | 0.00 deg/s |
| lince | still | 52.50 m/s | 0.000° | 0.016° | 0.000° | 0.000° | 0.016° | 0.00 deg/s | 1.18 deg/s | 0.00 deg/s |
| lince | crosswind | 52.50 m/s | 0.020° | 0.020° | 0.000° | 0.000° | 0.016° | 1.47 deg/s | 1.50 deg/s | 0.00 deg/s |
| astra | still | 49.17 m/s | 0.000° | 0.000° | 0.000° | 0.000° | 0.000° | 0.00 deg/s | 0.11 deg/s | 0.00 deg/s |
| astra | crosswind | 49.17 m/s | 0.003° | 0.001° | 0.000° | 0.000° | 0.000° | 0.62 deg/s | 0.34 deg/s | 0.00 deg/s |
| andromeda | still | 45.91 m/s | 0.000° | 0.004° | 0.000° | 0.000° | 0.004° | 0.00 deg/s | 0.49 deg/s | 0.00 deg/s |
| andromeda | crosswind | 45.90 m/s | 0.016° | 0.007° | 0.000° | 0.000° | 0.004° | 2.09 deg/s | 0.86 deg/s | 0.00 deg/s |
| erebus-11 | still | 44.57 m/s | 0.000° | 0.005° | 0.000° | 0.000° | 0.005° | 0.00 deg/s | 0.62 deg/s | 0.00 deg/s |
| erebus-11 | crosswind | 44.56 m/s | 0.009° | 0.006° | 0.000° | 0.000° | 0.005° | 1.11 deg/s | 0.71 deg/s | 0.00 deg/s |
| prometheus-2022-generic-motor | still | 27.36 m/s | 0.000° | 0.013° | 0.000° | 0.000° | 0.013° | 0.00 deg/s | 1.49 deg/s | 0.00 deg/s |
| prometheus-2022-generic-motor | crosswind | 27.36 m/s | 0.004° | 0.014° | 0.000° | 0.000° | 0.013° | 0.39 deg/s | 1.58 deg/s | 0.00 deg/s |

## What HPR Sim's release leaves out: on less guided

| Rocket | Wind | Apogee, guided | Apogee moved | Landing point moved | Of the apogee | Farther from the pad | Pitch rate moved |
|---|---|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | 2614.4 m | -0.060% | 16.3 m | 0.625% | +16.3 m | +0.72 deg/s |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | 2571.3 m | -0.042% | 16.5 m | 0.641% | +8.3 m | +0.74 deg/s |
| calisto-tests-motor-at-minus-1.373 | still | 2614.5 m | -0.062% | 16.9 m | 0.648% | +16.9 m | +0.66 deg/s |
| calisto-tests-motor-at-minus-1.373 | crosswind | 2576.8 m | -0.047% | 16.7 m | 0.648% | +7.7 m | +0.67 deg/s |
| bella-lui | still | 540.2 m | -0.003% | 1.3 m | 0.238% | +1.3 m | +0.54 deg/s |
| bella-lui | crosswind | 486.2 m | +0.021% | 1.2 m | 0.240% | -0.3 m | +0.60 deg/s |
| ndrt-2020-nose-to-tail | still | 1219.4 m | +0.001% | 0.0 m | 0.000% | 0.0 m | 0.00 deg/s |
| ndrt-2020-nose-to-tail | crosswind | 1204.5 m | -0.022% | 2.9 m | 0.241% | -2.9 m | +0.18 deg/s |
| valetudo | still | 778.1 m | -0.345% | 28.2 m | 3.621% | +28.2 m | +2.09 deg/s |
| valetudo | crosswind | 756.7 m | -0.195% | 28.5 m | 3.765% | +26.7 m | +2.20 deg/s |
| juno-iii | still | 2623.9 m | -0.414% | 91.3 m | 3.481% | +91.3 m | +2.26 deg/s |
| juno-iii | crosswind | 2492.0 m | -0.157% | 92.0 m | 3.692% | +83.8 m | +2.38 deg/s |
| cavour | still | 3163.3 m | -0.032% | 13.7 m | 0.434% | +13.7 m | +0.79 deg/s |
| cavour | crosswind | 3115.8 m | +0.015% | 22.6 m | 0.726% | +1.0 m | +1.32 deg/s |
| genesis | still | 3265.2 m | -0.037% | 10.3 m | 0.317% | +10.3 m | +0.60 deg/s |
| genesis | crosswind | 3218.7 m | +0.010% | 16.7 m | 0.520% | +16.7 m | +0.96 deg/s |
| lince | still | 3803.1 m | -0.068% | 19.3 m | 0.507% | +19.3 m | +1.18 deg/s |
| lince | crosswind | 3758.2 m | -0.019% | 24.2 m | 0.643% | +17.6 m | +1.50 deg/s |
| astra | still | 3447.5 m | -0.006% | 3.0 m | 0.086% | +3.0 m | +0.11 deg/s |
| astra | crosswind | 3392.9 m | -0.024% | 8.3 m | 0.244% | +6.8 m | +0.34 deg/s |
| andromeda | still | 3054.8 m | -0.019% | 4.7 m | 0.153% | +4.7 m | +0.49 deg/s |
| andromeda | crosswind | 2988.8 m | -0.053% | 7.9 m | 0.264% | -3.6 m | +0.86 deg/s |
| erebus-11 | still | 2814.6 m | -0.035% | 13.1 m | 0.464% | +13.1 m | +0.62 deg/s |
| erebus-11 | crosswind | 2764.8 m | -0.013% | 14.3 m | 0.516% | +5.9 m | +0.71 deg/s |
| prometheus-2022-generic-motor | still | 3806.6 m | -0.345% | 54.0 m | 1.417% | +54.0 m | +1.49 deg/s |
| prometheus-2022-generic-motor | crosswind | 3714.8 m | -0.249% | 58.1 m | 1.565% | +57.9 m | +1.58 deg/s |

## Between the two pins

This pin's figure less the earlier pin's, for each flight: the difference between the two commits of RocketPy, whatever in them made it. The first four are the figures HPR Sim's own phase is held to RocketPy's on ([the comparison](tip-off-model.md)), compared the same way: the turning rate and the angle where the lower button leaves with the phase on, each as its nose-down and nose-right parts together; and what the phase moves from the guided flight, in the apogee and, as a vector, in the landing point, each as a share of the guided apogee. The last two are how far the guided flight itself moved. Each figure here is rounded to the nearest; the bounds under [the reading](#the-reading) are rounded up.

| Rocket | Wind | Turning rate | Angle | Apogee move | Landing-point move | Apogee, guided | Landing point, guided |
|---|---|--:|--:|--:|--:|--:|--:|
| calisto-getting-started-motor-at-minus-1.255 | still | 0.000000 deg/s | 0.0000000° | -0.000015% | 0.000002% | +0.09 mm | 0.03 mm |
| calisto-getting-started-motor-at-minus-1.255 | crosswind | 0.000000 deg/s | 0.0000000° | -0.000019% | 0.000026% | +0.56 mm | 0.80 mm |
| calisto-tests-motor-at-minus-1.373 | still | 0.000000 deg/s | 0.0000000° | +0.000023% | 0.000003% | -0.58 mm | 0.08 mm |
| calisto-tests-motor-at-minus-1.373 | crosswind | 0.000001 deg/s | 0.0000000° | -0.000020% | 0.000032% | +0.62 mm | 0.96 mm |
| bella-lui | still | 0.000000 deg/s | 0.0000000° | -0.000001% | 0.000001% | -0.01 mm | 0.00 mm |
| bella-lui | crosswind | 0.000000 deg/s | 0.0000000° | -0.000006% | 0.000000% | -0.03 mm | 0.00 mm |
| ndrt-2020-nose-to-tail | still | 0.000000 deg/s | 0.0000000° | +0.000033% | 0.000000% | -0.08 mm | 0.00 mm |
| ndrt-2020-nose-to-tail | crosswind | 0.000000 deg/s | 0.0000000° | +0.000004% | 0.000009% | -0.01 mm | 0.02 mm |
| valetudo | still | 0.000000 deg/s | 0.0000000° | +0.000004% | 0.000001% | +0.06 mm | 0.01 mm |
| valetudo | crosswind | 0.000000 deg/s | 0.0000000° | -0.000002% | 0.000001% | -0.08 mm | 0.03 mm |
| juno-iii | still | 0.000000 deg/s | 0.0000000° | -0.000007% | 0.000001% | +0.21 mm | 0.03 mm |
| juno-iii | crosswind | 0.000000 deg/s | 0.0000000° | +0.000009% | 0.000002% | -0.29 mm | 0.12 mm |
| cavour | still | 0.000000 deg/s | 0.0000000° | +0.000001% | 0.000000% | +0.16 mm | 0.04 mm |
| cavour | crosswind | 0.000001 deg/s | 0.0000000° | 0.000000% | 0.000004% | -0.06 mm | 0.09 mm |
| genesis | still | 0.000001 deg/s | 0.0000000° | +0.000010% | 0.000002% | -0.65 mm | 0.12 mm |
| genesis | crosswind | 0.000000 deg/s | 0.0000000° | +0.000015% | 0.000012% | -0.05 mm | 0.12 mm |
| lince | still | 0.000001 deg/s | 0.0000000° | +0.000005% | 0.000001% | -0.16 mm | 0.02 mm |
| lince | crosswind | 0.000000 deg/s | 0.0000000° | +0.000015% | 0.000017% | -0.25 mm | 0.26 mm |
| astra | still | 0.000000 deg/s | 0.0000000° | +0.000001% | 0.000002% | +0.09 mm | 0.02 mm |
| astra | crosswind | 0.000000 deg/s | 0.0000000° | +0.000002% | 0.000001% | -0.05 mm | 0.02 mm |
| andromeda | still | 0.000000 deg/s | 0.0000000° | +0.000014% | 0.000001% | +0.07 mm | 0.05 mm |
| andromeda | crosswind | 0.000005 deg/s | 0.0000001° | +0.000003% | 0.000027% | -0.09 mm | 0.86 mm |
| erebus-11 | still | 0.000000 deg/s | 0.0000000° | +0.000002% | 0.000001% | -0.17 mm | 0.04 mm |
| erebus-11 | crosswind | 0.000000 deg/s | 0.0000000° | 0.000000% | 0.000000% | +0.03 mm | 0.01 mm |
| prometheus-2022-generic-motor | still | 0.000019 deg/s | 0.0000003° | -0.000006% | 0.000014% | +0.19 mm | 0.29 mm |
| prometheus-2022-generic-motor | crosswind | 0.000003 deg/s | 0.0000001° | +0.000015% | 0.000016% | -0.45 mm | 0.49 mm |
