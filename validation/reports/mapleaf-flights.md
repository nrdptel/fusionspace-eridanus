# HPR Sim against MAPLEAF on three public designs

Written by `cargo xtask mapleaf-flights` ([M1.14b8][m1-14b8], decision [ADR-378][adr-378]); do not edit. [MAPLEAF][mapleaf] is an open-source (MIT) six-degree-of-freedom rocket flight simulator with its own aerodynamics, written apart from HPR Sim and from RocketPy. This report flies it and HPR Sim on the same 3 public designs and gives how far apart their apogees and rail-exit speeds are. Neither code is the truth: a difference here is a measurement to explain, and nothing in this report is held to a tolerance. It covers flights straight up in calm air to apogee, all subsonic (the largest Mach number is 0.56); it says nothing about wind, a leaning rail, the descent or supersonic flight.

- **The oracle** is MAPLEAF 0.8.16 at commit af970d3e8200832f5e70d537b15ad38dd74fa551, run as an outside program in its own Python environment (`validation/oracles/mapleaf/`); its flights are `validation/fixtures/flight/mapleaf-flights.json`.
- **The designs** are [`valetudo`](../../validation/designs/rocketpy-valetudo.json), [`bella-lui`](../../validation/designs/rocketpy-bella-lui.json), [`ndrt-2020-nose-to-tail`](../../validation/designs/rocketpy-ndrt-2020-nose-to-tail.json): the RocketPy examples with a tangent-ogive nose, the one curved nose MAPLEAF has.
- **The inputs** (`validation/fixtures/flight/mapleaf-inputs.json`) are written from HPR Sim's own reading of each design, so MAPLEAF is handed what HPR Sim flies: the outside shape and its surface roughness, the mass, center of mass and inertia without propellant, the motor's thrust, propellant flow, propellant center and propellant inertia through the burn, the guided travel on the rail, and the site. Parts MAPLEAF has no counterpart for are not handed over, and their mass stays in the total: motor-mount (inner tube), rail-buttons (rail buttons) (`valetudo`), motor-mount (inner tube), rail-buttons (rail buttons) (`bella-lui`), motor-mount (inner tube), rail-buttons (rail buttons) (`ndrt-2020-nose-to-tail`).
- **Same drag:** both codes fly one table of drag coefficient against Mach number, motor on or off (HPR Sim's own coasting drag at no angle of attack, in the standard atmosphere's air at the site's elevation, every 0.02 in Mach number to Mach 1, with straight lines between and the first value held down to Mach 0). What is left between them is everything but the drag.
- **Own drag:** each code works its aerodynamics out from the same outside shape.
- **Apogee** is how far the center of mass rises above where it stood at ignition.
- **Rail-exit speed** is the airframe's speed where it has traveled the guided length, which is the rail's length less the distance from the rocket's last guide to its aft end.
- **A difference** is HPR Sim's value less MAPLEAF's, and its percentage is of MAPLEAF's.

[m1-14b8]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b8
[adr-378]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0378-mapleaf-as-a-second-open-oracle-flown-on-three-public-designs.md
[mapleaf]: https://github.com/henrystoldt/MAPLEAF

## Same drag

| design | motor | mass at ignition | largest Mach number | apogee, HPR Sim | apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | K400C | 9.667 kg | 0.333 | 776.4 m (2547 ft) | 773.6 m (2538 ft) | +2.71 m (+0.35%) | 22.510 m/s (73.85 ft/s) | 22.502 m/s (73.83 ft/s) | +0.0076 m/s (+0.034%) |
| `bella-lui` | 2245K1075-P | 19.051 kg | 0.291 | 554.1 m (1818 ft) | 553.4 m (1816 ft) | +0.72 m (+0.13%) | 18.652 m/s (61.19 ft/s) | 18.650 m/s (61.19 ft/s) | +0.0023 m/s (+0.012%) |
| `ndrt-2020-nose-to-tail` | L2500ST | 23.321 kg | 0.550 | 1403.8 m (4606 ft) | 1401.9 m (4600 ft) | +1.90 m (+0.14%) | 19.330 m/s (63.42 ft/s) | 19.328 m/s (63.41 ft/s) | +0.0014 m/s (+0.007%) |

### With MAPLEAF's gravity matched

The two codes' gravity differs at the pad by up to 0.33% ([the list below](#what-the-two-could-not-share)). To measure what that moves, MAPLEAF flew each same-drag flight once more with its gravity constant scaled so that its gravity at the pad is HPR Sim's. That is a measurement, not MAPLEAF as it ships; HPR Sim's flights are the same-drag ones above.

| design | factor on MAPLEAF's gravity constant | apogee, HPR Sim | apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |
|---|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | 0.996761 | 776.36 m | 776.35 m | +0.008 m (+0.0011%) | 22.5098 m/s | 22.5102 m/s | -0.00039 m/s (-0.0017%) |
| `bella-lui` | 0.998772 | 554.10 m | 554.09 m | +0.006 m (+0.0012%) | 18.6522 m/s | 18.6522 m/s | -0.00001 m/s (-0.0001%) |
| `ndrt-2020-nose-to-tail` | 0.998272 | 1403.85 m | 1403.79 m | +0.052 m (+0.0037%) | 19.3297 m/s | 19.3299 m/s | -0.00018 m/s (-0.0009%) |

With the drag and the gravity both shared, the two codes' apogees are within 0.053 m (0.18 ft, 0.0038%) of each other and their rail-exit speeds within 0.00040 m/s (0.0013 ft/s, 0.0018%). MAPLEAF's own apogee moves by up to 0.059 m (0.20 ft) between its two time steps.

## Own drag

| design | motor | mass at ignition | largest Mach number | apogee, HPR Sim | apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | K400C | 9.667 kg | 0.333 | 778.4 m (2554 ft) | 780.2 m (2560 ft) | -1.74 m (-0.22%) | 22.511 m/s (73.85 ft/s) | 22.504 m/s (73.83 ft/s) | +0.0064 m/s (+0.028%) |
| `bella-lui` | 2245K1075-P | 19.051 kg | 0.292 | 555.8 m (1823 ft) | 550.3 m (1805 ft) | +5.50 m (+1.00%) | 18.653 m/s (61.20 ft/s) | 18.651 m/s (61.19 ft/s) | +0.0024 m/s (+0.013%) |
| `ndrt-2020-nose-to-tail` | L2500ST | 23.321 kg | 0.551 | 1407.2 m (4617 ft) | 1307.5 m (4290 ft) | +99.78 m (+7.63%) | 19.330 m/s (63.42 ft/s) | 19.328 m/s (63.41 ft/s) | +0.0015 m/s (+0.008%) |

### The drag each flew

Each code's aerodynamic force along the axis just after burnout, as a coefficient on HPR Sim's reference area, whole and by part. The base drag is counted with the last body part in both codes. The two are read at slightly different Mach numbers, each at its own flight's.

| design | Mach number, HPR Sim | Mach number, MAPLEAF | axial coefficient, HPR Sim | axial coefficient, MAPLEAF | difference |
|---|---:|---:|---:|---:|---:|
| `valetudo` | 0.329 | 0.331 | 0.5587 | 0.5636 | -0.9% |
| `bella-lui` | 0.288 | 0.288 | 0.4101 | 0.4605 | -11.0% |
| `ndrt-2020-nose-to-tail` | 0.549 | 0.550 | 0.3380 | 0.4522 | -25.2% |

| design | part | HPR Sim | MAPLEAF |
|---|---|---:|---:|
| `valetudo` | `nose` | 0.0248 | 0.0352 |
| `valetudo` | `body-tube-1` | 0.3884 | 0.3938 |
| `valetudo` | `fins-1` | 0.1213 | 0.1346 |
| `valetudo` | `rail-buttons` | 0.0243 | none |
| `bella-lui` | `nose` | 0.0140 | 0.0220 |
| `bella-lui` | `body-tube-1` | 0.1884 | 0.1925 |
| `bella-lui` | `fins-1` | 0.1025 | 0.1295 |
| `bella-lui` | `rail-buttons` | 0.0065 | none |
| `bella-lui` | `tail-1` | 0.0986 | 0.1164 |
| `ndrt-2020-nose-to-tail` | `nose` | 0.0238 | 0.0331 |
| `ndrt-2020-nose-to-tail` | `body-tube-1` | 0.0334 | 0.0471 |
| `ndrt-2020-nose-to-tail` | `tail-1` | 0.0181 | 0.0241 |
| `ndrt-2020-nose-to-tail` | `body-tube-2` | 0.1821 | 0.2574 |
| `ndrt-2020-nose-to-tail` | `fins-1` | 0.0766 | 0.0905 |
| `ndrt-2020-nose-to-tail` | `rail-buttons` | 0.0040 | none |

Of the 12 parts both codes have, MAPLEAF's coefficient is the higher on 12.

## What the two could not share

Each row is an input or a model the two codes differ in, in the flights named, with what was measured of it. *Not measured apart* means its effect is inside the differences above and has no number of its own.

| input | flights | HPR Sim | MAPLEAF | measured |
|---|---|---|---|---|
| Drag | own drag | its own drag buildup, part by part | its own, from the same outside shape and surface roughness, with its defaults kept (`Rocket.Aero.addZeroLengthBoatTailsToAccountForBaseDrag true`, `Rocket.Aero.fullyTurbulentBL true`) | The axial coefficient just after burnout, HPR Sim's less MAPLEAF's: -0.9% (`valetudo`), -11.0% (`bella-lui`), -25.2% (`ndrt-2020-nose-to-tail`). By part below. |
| Drag of the rail buttons | own drag | in its drag buildup | no such part | Of HPR Sim's axial coefficient just after burnout: 4.3% (`valetudo`), 1.6% (`bella-lui`), 1.2% (`ndrt-2020-nose-to-tail`). |
| Normal force and moments | every flight | its own | its own with its own drag; none with the same drag, where no shape is flown | Not measured apart. |
| Gravity | every flight | WGS 84 normal gravity at the site's latitude and the rocket's height, the centrifugal term in it | its `Flat` Earth model: GM/(R + z)² with R = 6371 km, the same at every latitude | At ignition, HPR Sim's against MAPLEAF's: 9.7864 against 9.8182 m/s² (-0.32%) (`valetudo`), 9.8069 against 9.8190 m/s² (-0.12%) (`bella-lui`), 9.8026 against 9.8196 m/s² (-0.17%) (`ndrt-2020-nose-to-tail`). With MAPLEAF's gravity matched to HPR Sim's at the pad, the same-drag apogee differences go from +0.35% (`valetudo`), +0.13% (`bella-lui`), +0.14% (`ndrt-2020-nose-to-tail`) to +0.0011% (`valetudo`), +0.0012% (`bella-lui`), +0.0037% (`ndrt-2020-nose-to-tail`) ([the table](#with-mapleafs-gravity-matched)). |
| The Earth's turning | every flight | the Coriolis force on the moving rocket | none in its `Flat` model | Not measured apart. |
| The air | every flight | the 1976 standard atmosphere, worked out at each height | its table of the U.S. Standard Atmosphere | Air density at ignition, HPR Sim's less MAPLEAF's: -0.009% (`valetudo`), -0.011% (`bella-lui`), -0.017% (`ndrt-2020-nose-to-tail`). |
| The rail's lean | every flight | straight up, where the examples' rails lean 5.3° (`valetudo`), 1.0° (`bella-lui`), 0.0° (`ndrt-2020-nose-to-tail`) from the vertical | straight up too | On a rail 5.3° from the vertical MAPLEAF's rocket travels 10.60° from it with its body 5.30° from it, an angle of attack of 5.30° while still guided (`valetudo`, [below](#mapleaf-on-a-leaning-rail)). |
| Leaving the rail | every flight | guided until the airframe has traveled the guided length; no rail friction and no tip-off phase, neither of which MAPLEAF has | guided until its center of mass has traveled that length; the center moves forward in the airframe as the propellant burns, so the airframe has then traveled less | MAPLEAF frees the rocket with its airframe short of the guided length by 15.5 mm (`valetudo`), 6.6 mm (`bella-lui`), 16.2 mm (`ndrt-2020-nose-to-tail`). Both rail-exit speeds here are read where the airframe has traveled all of it. |
| Jet damping | every flight | the exhaust's damping of pitch and yaw | only as a `FractionalJetDamping` part, which these flights do not add | Not measured apart. |
| Where the propellant's center of mass is | every flight | where its motor model puts it at each moment | on a straight line in the burnt fraction, between the two centers its motor file gives | HPR Sim's center leaves that line by at most 4.4e-16 m on these motors. |
| Mass and inertia between the motor file's rows | every flight | its motor model at every moment | straight lines between the rows HPR Sim's model was sampled into, 10 to each interval of the thrust curve | MAPLEAF's logged mass, center of mass and inertia at ignition and at burnout are the inputs' to one part in a million, checked on every run. Between the rows, not measured. |
| Integration | every flight | Dormand–Prince 5(4) with an adaptive step, relative and absolute tolerance 1e-8 | classical fourth-order Runge–Kutta at a fixed 1 ms | At 2 ms MAPLEAF's apogee moves by at most 0.059 m and its rail-exit speed by at most 0.0003 m/s. |

## MAPLEAF on a leaning rail

Every flight here leaves a vertical rail (90° from the horizontal), though the examples' rails lean. Flown on `valetudo`'s rail as its example has it, 5.3° from the vertical, MAPLEAF at this commit moves the rocket up a line 10.60° from the vertical while its body points 5.30° from it: halfway up the rail (2.81 m of travel) the rocket is still guided and already flies at an angle of attack of 5.30°. A rail holds a rocket's travel and its axis on one line, so that is not the launch the input asks for, and the lean is left out of both codes' flights instead of compared.

## Running it again

```sh
cargo xtask mapleaf-flights --inputs   # HPR Sim's reading of the designs
UV_PROJECT_ENVIRONMENT=$PWD/refs/venv-mapleaf uv sync --locked --project validation/oracles/mapleaf
refs/venv-mapleaf/bin/python -I validation/oracles/mapleaf/flights.py   # MAPLEAF's flights
cargo xtask mapleaf-flights            # this report
```

`cargo xtask mapleaf-flights --check` flies HPR Sim again and fails unless the committed inputs, report and page reproduce; it refuses MAPLEAF flights that were not flown from the committed inputs (their SHA-256, `edff83ee026b60393de1831d374340d406afcfbe8aeb07024255d941a5eea388`). MAPLEAF itself runs only where its environment is installed.
