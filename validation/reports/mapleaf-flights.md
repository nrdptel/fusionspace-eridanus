# HPR Sim against MAPLEAF on three public designs

Written by `cargo xtask mapleaf-flights` ([M1.14b8][m1-14b8], decision [ADR-378][adr-378]); do not edit. [MAPLEAF][mapleaf] is an open-source (MIT) six-degree-of-freedom rocket flight simulator with its own aerodynamics, written apart from HPR Sim and from RocketPy. This report flies it and HPR Sim on the same 3 public designs and gives how far apart their apogees and rail-exit speeds are. Neither code is the truth: a difference here is a measurement to explain, and nothing in this report is held to a tolerance. It covers flights straight up in calm air to apogee, all subsonic (the largest Mach number is 0.56); it says nothing about wind, a leaning rail, the descent or supersonic flight.

- **The oracle** is MAPLEAF 0.8.16 at commit af970d3e8200832f5e70d537b15ad38dd74fa551, run as an outside program in its own Python environment (`validation/oracles/mapleaf/`); its flights are `validation/fixtures/flight/mapleaf-flights.json`.
- **The designs** are [`valetudo`](../../validation/designs/rocketpy-valetudo.json), [`bella-lui`](../../validation/designs/rocketpy-bella-lui.json), [`ndrt-2020-nose-to-tail`](../../validation/designs/rocketpy-ndrt-2020-nose-to-tail.json): the RocketPy examples with a tangent-ogive nose, the one curved nose MAPLEAF has.
- **The inputs** (`validation/fixtures/flight/mapleaf-inputs.json`) are written from HPR Sim's own reading of each design, so MAPLEAF is handed what HPR Sim flies: the outside shape and its surface roughness, the mass, center of mass and inertia without propellant, the motor's thrust, propellant flow, propellant center and propellant inertia through the burn, the guided travel on the rail, the site, and the height the center of mass stands at on the pad. Parts MAPLEAF has no counterpart for are not handed over, and their mass stays in the total: motor-mount (inner tube), rail-buttons (rail buttons) (`valetudo`), motor-mount (inner tube), rail-buttons (rail buttons) (`bella-lui`), motor-mount (inner tube), rail-buttons (rail buttons) (`ndrt-2020-nose-to-tail`).
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
| `valetudo` | K400C | 9.667 kg | 0.333 | 776.4 m (2547 ft) | 773.7 m (2538 ft) | +2.70 m (+0.35%) | 22.510 m/s (73.85 ft/s) | 22.502 m/s (73.83 ft/s) | +0.0076 m/s (+0.034%) |
| `bella-lui` | 2245K1075-P | 19.051 kg | 0.291 | 554.1 m (1818 ft) | 553.4 m (1816 ft) | +0.71 m (+0.13%) | 18.652 m/s (61.19 ft/s) | 18.650 m/s (61.19 ft/s) | +0.0023 m/s (+0.012%) |
| `ndrt-2020-nose-to-tail` | L2500ST | 23.321 kg | 0.550 | 1403.8 m (4606 ft) | 1402.0 m (4600 ft) | +1.83 m (+0.13%) | 19.330 m/s (63.42 ft/s) | 19.328 m/s (63.41 ft/s) | +0.0014 m/s (+0.007%) |

### With MAPLEAF's gravity matched

The two codes' gravity differs at the pad by up to 0.324% ([the list below](#what-the-two-could-not-share)). To measure what that moves, MAPLEAF flew each same-drag flight once more with its gravity constant scaled so that its gravity at the pad is HPR Sim's. That is a measurement, not MAPLEAF as it ships; HPR Sim's flights are the same-drag ones above.

| design | factor on MAPLEAF's gravity constant | apogee, HPR Sim | apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |
|---|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | 0.996761 | 776.36 m | 776.36 m | -0.0003 m (-0.00004%) | 22.5098 m/s | 22.5102 m/s | -0.00039 m/s (-0.0017%) |
| `bella-lui` | 0.998772 | 554.10 m | 554.10 m | -0.0003 m (-0.00005%) | 18.6522 m/s | 18.6522 m/s | -0.00001 m/s (-0.0001%) |
| `ndrt-2020-nose-to-tail` | 0.998272 | 1403.85 m | 1403.86 m | -0.0186 m (-0.00132%) | 19.3297 m/s | 19.3299 m/s | -0.00018 m/s (-0.0009%) |

With the drag and the gravity both shared, the two codes' apogees are within 0.019 m (0.063 ft, 0.0014%) of each other and their rail-exit speeds within 0.00040 m/s (0.0013 ft/s, 0.0018%). Between its two time steps MAPLEAF's apogee in these flights moves by up to 0.030 m (0.099 ft) and its rail-exit speed by up to 0.00024 m/s (0.0008 ft/s), so a difference smaller than that is not resolved here.

### With the internal-momentum force added

HPR Sim's equations of motion carry three internal-momentum terms beside the thrust, as RocketPy's do, and MAPLEAF's carry none ([the list below](#what-the-two-could-not-share)). With the propellant's center fixed in the airframe, as on these motors, the terms sum to one force along the axis: the rate of change of the propellant flow times the lever, the nozzle exit's distance behind the propellant's center. In a second measurement MAPLEAF flew each gravity-matched flight again with that force added to its thrust. *Moved by* is this flight less the gravity-matched one; *difference* is HPR Sim's same-drag flight less this one; *step* is this flight at MAPLEAF's coarser time step less at its finer.

| design | lever | apogee, MAPLEAF | moved by | difference | step | rail exit, MAPLEAF | moved by | difference | step |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | 0.420 m | 776.34 m | -0.014 m | +0.014 m (+0.0018%) | -0.002 m | 22.5098 m/s | -0.00039 m/s | -0.00000 m/s (-0.0000%) | +0.00000 m/s |
| `bella-lui` | 0.300 m | 554.09 m | -0.007 m | +0.007 m (+0.0012%) | +0.011 m | 18.6522 m/s | -0.00004 m/s | +0.00002 m/s (+0.0001%) | +0.00017 m/s |
| `ndrt-2020-nose-to-tail` | 0.350 m | 1403.76 m | -0.104 m | +0.085 m (+0.0061%) | +0.064 m | 19.3310 m/s | +0.00103 m/s | -0.00121 m/s (-0.0062%) | -0.00108 m/s |

The added force moves MAPLEAF's apogee by up to 0.104 m (0.342 ft) and its rail-exit speed by up to 0.00103 m/s (0.0034 ft/s). It changes at once where the thrust curve bends, and between MAPLEAF's two time steps these flights move by up to 0.065 m (0.214 ft) of apogee and 0.00109 m/s (0.0036 ft/s) of rail-exit speed. A change the force makes that is no larger than that is not resolved here: this table sizes what the terms move, and does not settle it.

## Own drag

| design | motor | mass at ignition | largest Mach number | apogee, HPR Sim | apogee, MAPLEAF | difference | rail exit, HPR Sim | rail exit, MAPLEAF | difference |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `valetudo` | K400C | 9.667 kg | 0.333 | 778.4 m (2554 ft) | 780.2 m (2560 ft) | -1.75 m (-0.22%) | 22.511 m/s (73.85 ft/s) | 22.504 m/s (73.83 ft/s) | +0.0064 m/s (+0.028%) |
| `bella-lui` | 2245K1075-P | 19.051 kg | 0.292 | 555.8 m (1823 ft) | 550.3 m (1805 ft) | +5.50 m (+1.00%) | 18.653 m/s (61.20 ft/s) | 18.651 m/s (61.19 ft/s) | +0.0024 m/s (+0.013%) |
| `ndrt-2020-nose-to-tail` | L2500ST | 23.321 kg | 0.551 | 1407.2 m (4617 ft) | 1307.5 m (4290 ft) | +99.71 m (+7.63%) | 19.330 m/s (63.42 ft/s) | 19.328 m/s (63.41 ft/s) | +0.0015 m/s (+0.008%) |

### The drag each flew

Each code's aerodynamic force along the axis just after burnout, as a coefficient on HPR Sim's reference area, whole, by part and by term. The base drag is counted with the last body part in both codes. The two are read at slightly different Mach numbers, each at its own flight's.

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
| `bella-lui` | `tail-1` | 0.0986 | 0.1165 |
| `ndrt-2020-nose-to-tail` | `nose` | 0.0238 | 0.0331 |
| `ndrt-2020-nose-to-tail` | `body-tube-1` | 0.0334 | 0.0471 |
| `ndrt-2020-nose-to-tail` | `tail-1` | 0.0181 | 0.0241 |
| `ndrt-2020-nose-to-tail` | `body-tube-2` | 0.1821 | 0.2574 |
| `ndrt-2020-nose-to-tail` | `fins-1` | 0.0766 | 0.0905 |
| `ndrt-2020-nose-to-tail` | `rail-buttons` | 0.0040 | none |

Of the 12 parts both codes have, MAPLEAF's coefficient is the higher on 12.

Each part's coefficient by term. Skin friction is the drag of the air sliding along a surface, and pressure drag the push on a part's front and its narrowing sides. [Base drag](https://hpr.fusionspace.co/glossary.html#base-drag) is the suction behind the blunt aft end, which a [boattail](https://hpr.fusionspace.co/glossary.html#boattail) narrows. The other two terms are the thickness drag MAPLEAF adds for its fins, and the rail buttons' drag, which only HPR Sim has. From here to the end of this section a difference is MAPLEAF's value less HPR Sim's. A term a code does not have for a part reads `none` and counts as zero in the difference.

| design | part | term | HPR Sim | MAPLEAF | MAPLEAF less HPR Sim |
|---|---|---|---:|---:|---:|
| `valetudo` | `nose` | friction | 0.0248 | 0.0352 | +0.0104 |
| `valetudo` | `nose` | pressure | 0.0000 | 0.0000 | 0.0000 |
| `valetudo` | `body-tube-1` | friction | 0.2543 | 0.2595 | +0.0053 |
| `valetudo` | `body-tube-1` | base | 0.1341 | 0.1342 | +0.0001 |
| `valetudo` | `fins-1` | friction | 0.0105 | 0.0219 | +0.0114 |
| `valetudo` | `fins-1` | pressure | 0.1108 | 0.1108 | 0.0000 |
| `valetudo` | `fins-1` | thickness | none | 0.0019 | +0.0019 |
| `valetudo` | `rail-buttons` | rail buttons | 0.0243 | none | -0.0243 |
| `bella-lui` | `nose` | friction | 0.0130 | 0.0210 | +0.0080 |
| `bella-lui` | `nose` | pressure | 0.0010 | 0.0010 | 0.0000 |
| `bella-lui` | `body-tube-1` | friction | 0.1884 | 0.1925 | +0.0041 |
| `bella-lui` | `fins-1` | friction | 0.0391 | 0.0649 | +0.0257 |
| `bella-lui` | `fins-1` | pressure | 0.0634 | 0.0634 | 0.0000 |
| `bella-lui` | `fins-1` | thickness | none | 0.0013 | +0.0013 |
| `bella-lui` | `rail-buttons` | rail buttons | 0.0065 | none | -0.0065 |
| `bella-lui` | `tail-1` | friction | 0.0037 | 0.0083 | +0.0046 |
| `bella-lui` | `tail-1` | pressure | 0.0102 | 0.0102 | 0.0000 |
| `bella-lui` | `tail-1` | base | 0.0848 | 0.0980 | +0.0132 |
| `ndrt-2020-nose-to-tail` | `nose` | friction | 0.0232 | 0.0325 | +0.0093 |
| `ndrt-2020-nose-to-tail` | `nose` | pressure | 0.0006 | 0.0006 | 0.0000 |
| `ndrt-2020-nose-to-tail` | `body-tube-1` | friction | 0.0334 | 0.0471 | +0.0137 |
| `ndrt-2020-nose-to-tail` | `tail-1` | friction | 0.0063 | 0.0124 | +0.0060 |
| `ndrt-2020-nose-to-tail` | `tail-1` | pressure | 0.0118 | 0.0118 | 0.0000 |
| `ndrt-2020-nose-to-tail` | `body-tube-2` | friction | 0.0893 | 0.0980 | +0.0087 |
| `ndrt-2020-nose-to-tail` | `body-tube-2` | base | 0.0928 | 0.1593 | +0.0666 |
| `ndrt-2020-nose-to-tail` | `fins-1` | friction | 0.0137 | 0.0265 | +0.0129 |
| `ndrt-2020-nose-to-tail` | `fins-1` | pressure | 0.0629 | 0.0630 | 0.0000 |
| `ndrt-2020-nose-to-tail` | `fins-1` | thickness | none | 0.0010 | +0.0010 |
| `ndrt-2020-nose-to-tail` | `rail-buttons` | rail buttons | 0.0040 | none | -0.0040 |

The terms summed over each design's parts. A term's share is its difference divided by the whole difference in the coefficient, so a design's shares add to 100%. A term that works against the whole difference has a negative share, and the other shares then add to more than 100%. On `valetudo` the whole difference is small beside its terms, so its shares are large. A share is taken before rounding, so a difference that reads 0.0000 can still have one.

| design | term | HPR Sim | MAPLEAF | MAPLEAF less HPR Sim | share of the difference |
|---|---|---:|---:|---:|---:|
| `valetudo` | friction | 0.2895 | 0.3166 | +0.0271 | +555.9% |
| `valetudo` | pressure | 0.1108 | 0.1108 | 0.0000 | +0.7% |
| `valetudo` | base | 0.1341 | 0.1342 | +0.0001 | +2.4% |
| `valetudo` | thickness | none | 0.0019 | +0.0019 | +39.4% |
| `valetudo` | rail buttons | 0.0243 | none | -0.0243 | -498.4% |
| `valetudo` | whole | 0.5587 | 0.5636 | +0.0049 | +100.0% |
| `bella-lui` | friction | 0.2443 | 0.2867 | +0.0424 | +84.2% |
| `bella-lui` | pressure | 0.0745 | 0.0745 | 0.0000 | 0.0% |
| `bella-lui` | base | 0.0848 | 0.0980 | +0.0132 | +26.2% |
| `bella-lui` | thickness | none | 0.0013 | +0.0013 | +2.5% |
| `bella-lui` | rail buttons | 0.0065 | none | -0.0065 | -12.9% |
| `bella-lui` | whole | 0.4101 | 0.4605 | +0.0504 | +100.0% |
| `ndrt-2020-nose-to-tail` | friction | 0.1659 | 0.2165 | +0.0506 | +44.3% |
| `ndrt-2020-nose-to-tail` | pressure | 0.0753 | 0.0754 | +0.0001 | +0.1% |
| `ndrt-2020-nose-to-tail` | base | 0.0928 | 0.1593 | +0.0666 | +58.3% |
| `ndrt-2020-nose-to-tail` | thickness | none | 0.0010 | +0.0010 | +0.9% |
| `ndrt-2020-nose-to-tail` | rail buttons | 0.0040 | none | -0.0040 | -3.5% |
| `ndrt-2020-nose-to-tail` | whole | 0.3380 | 0.4522 | +0.1142 | +100.0% |

The diameter of the base each code charges its base drag on, and MAPLEAF's base area over HPR Sim's.

| design | base diameter, HPR Sim | base diameter, MAPLEAF | base area, MAPLEAF's over HPR Sim's |
|---|---:|---:|---:|
| `valetudo` | 0.0809 m | 0.0809 m | 1.000 |
| `bella-lui` | 0.1350 m | 0.1350 m | 1.000 |
| `ndrt-2020-nose-to-tail` | 0.1550 m | 0.2030 m | 1.715 |

The length each code takes a part's friction coefficient on. HPR Sim takes the rocket's whole length for every part. MAPLEAF takes the part's own length, and the mean aerodynamic chord for the fins.

| design | part | friction length, HPR Sim | friction length, MAPLEAF |
|---|---|---:|---:|
| `valetudo` | `nose` | 2.158 m | 0.274 m |
| `valetudo` | `body-tube-1` | 2.158 m | 1.884 m |
| `valetudo` | `fins-1` | 2.158 m | 0.042 m |
| `bella-lui` | `nose` | 2.678 m | 0.242 m |
| `bella-lui` | `body-tube-1` | 2.678 m | 2.386 m |
| `bella-lui` | `fins-1` | 2.678 m | 0.212 m |
| `bella-lui` | `tail-1` | 2.678 m | 0.050 m |
| `ndrt-2020-nose-to-tail` | `nose` | 3.391 m | 0.610 m |
| `ndrt-2020-nose-to-tail` | `body-tube-1` | 3.391 m | 0.590 m |
| `ndrt-2020-nose-to-tail` | `tail-1` | 3.391 m | 0.127 m |
| `ndrt-2020-nose-to-tail` | `body-tube-2` | 3.391 m | 2.064 m |
| `ndrt-2020-nose-to-tail` | `fins-1` | 3.391 m | 0.118 m |

The terms that carry each design's difference, the largest share first, each with its difference and its share of the whole:

- `valetudo`: MAPLEAF's coefficient is 0.0049 the higher: friction 0.0271 (555.9%), thickness 0.0019 (39.4%), base 0.0001 (2.4%), pressure 0.0000 (0.7%), rail buttons -0.0243 (-498.4%).
- `bella-lui`: MAPLEAF's coefficient is 0.0504 the higher: friction 0.0424 (84.2%), base 0.0132 (26.2%), thickness 0.0013 (2.5%), pressure 0.0000 (0.0%), rail buttons -0.0065 (-12.9%).
- `ndrt-2020-nose-to-tail`: MAPLEAF's coefficient is 0.1142 the higher: base 0.0666 (58.3%), friction 0.0506 (44.3%), thickness 0.0010 (0.9%), pressure 0.0001 (0.1%), rail buttons -0.0040 (-3.5%).

#### What each term is in the two codes

Each row names the formula a code uses and the source its own code cites for it, or says that the code has no such term. MAPLEAF's files are those of its `MAPLEAF` package at the pinned commit. `M` is the Mach number, `R` the [Reynolds number](https://hpr.fusionspace.co/glossary.html#reynolds-number) on the length `L`, and `R_s` the surface's roughness height, set by its finish.

| term | HPR Sim | MAPLEAF |
|---|---|---|
| Skin friction coefficient | Niskanen's turbulent coefficient `1/(1.5 ln R - 5.6)^2`, or past the critical Reynolds number `51 (R_s/L)^-1.039` the rough wall's `0.032 (R_s/L)^0.2` (Niskanen 2009, eq. 3.78 to 3.81). `R` and `L` are on the rocket's whole length for every part, the fins too (`skin_friction_coefficient` in `crates/hpr-aero/src/drag.rs`). | The same formulas (`getSkinFrictionCoefficient` in `Rocket/AeroFunctions.py`, which cites Barrowman 1967 eq. 4-7 and Niskanen eq. 3.81). `R` and `L` are on each part's own length: the nose's, each tube's, the transition's, and the fins' [mean aerodynamic chord](https://hpr.fusionspace.co/glossary.html#mean-aerodynamic-chord-mac). |
| Its factor for compressibility at these Mach numbers | `1 - 0.1 M^2` on a smooth or a rough wall (Niskanen eq. 3.82). | `1 - 0.12 M^2` on a surface given any roughness and `1 - 0.10 M^2` on one given none. Its comment cites Barrowman's eq. 4-9 and 4-10; Barrowman's smooth wall, eq. 4-9, reads `1 - 0.09 M^2`. |
| Friction of a body part | The coefficient times `1 + 1/(2 f_B)`, `f_B` the rocket's [fineness ratio](https://hpr.fusionspace.co/glossary.html#fineness-ratio) (Niskanen eq. 3.85), on the area of the surface's projection along the axis, `2π ∫ r dx`. | The same factor, citing Barrowman eq. 4-16, on the same area for a tube and an ogive nose. A narrowing part or a cone takes its own slanted surface, which is the larger. |
| Friction of the fins | The coefficient times `1 + 2t/c`, `t` the thickness and `c` the mean aerodynamic chord, on the fins' faces (Niskanen eq. 3.85). | The same, citing Niskanen eq. 3.85 (`_getPreComputedFinAeroData` in `Rocket/Fins.py`). |
| Pressure drag | A nose by Niskanen's eq. 3.86 and 3.87, a narrowing part by the boattail rule (eq. 3.88), a fin's blunt edges by eq. 3.89 to 3.93. | A nose and a narrowing part by the same rules (it cites Niskanen eq. 3.87 and 3.88). A fin's blunt leading edge takes a blunt body's coefficient, citing Hoerner and Niskanen's eq. B.1 and B.2, turned for the edge's sweep by Barrowman eq. 4-22. Its blunt trailing edge cites Niskanen's section 3.4.4. |
| Base drag | `0.12 + 0.13 M^2` below Mach 1 (Niskanen eq. 3.94) on the area of the aft base, less a burning motor's. Up to Mach 0.8 a base right behind a boattail keeps `d_B/d_1` of that, its own diameter over the boattail's fore diameter ([ADR-362, the base behind a boattail held to a measured flight](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0362-base-drag-behind-a-boattail-below-mach-0-8-held-to-the-hermes-a-3a.md)). | The same coefficient (`getBaseDragCoefficient`, which cites no source) on the bottom area of a boattail, and only after the motor's shut-off time. A rocket that ends in a tube is given a boattail of no length for it, as wide as the rocket's largest body tube. The code's own comment says the bottom tube's diameter is still to be used there (`_ensureBaseDragIsAccountedFor` in `Rocket/rocket.py`). |
| Thickness drag of the fins | None. Niskanen takes a fin's thickness into account in the friction's `1 + 2t/c` alone (p. 49). | Barrowman's subsonic thickness drag for each fin (its constant cites Barrowman eq. 4-36), added to the friction with its `1 + 2t/c`. The code marks that section as not working quite right. |
| Rail buttons | Niskanen's rule for a rail pin (p. 52). | No such part. |
| Interference between the fins and the body | None: neglected, as Niskanen does (p. 41). | None in the drag: its fin and body factors act on the normal force. |

In these three flights neither code has a drag term outside these rows. Each part's terms above sum to the part's coefficient, to one part in a million of the whole for HPR Sim and one in a billion for MAPLEAF, so no interference drag is hidden in a part. A MAPLEAF nose's pressure drag is read as what its friction leaves of the nose's force.

#### What the split says

Base drag and skin friction carry `ndrt-2020-nose-to-tail`'s difference, the largest of the three. The two codes give a shared formula different inputs: an area for base drag and a length for friction. These tables compare two codes at one instant of each flight; the measurements come in the next section.

- **Base drag is the largest term of the largest difference, and it comes from an area, not from a formula.** The two codes use one coefficient. On `ndrt-2020-nose-to-tail` MAPLEAF charges it on a base as wide as the rocket's largest tube. The rocket ends in a narrower one, so MAPLEAF's base area is the larger by the ratio in the table of base diameters. On `valetudo`, which ends in its only tube, the two base terms differ by 0.0001. On `bella-lui`, which ends in a boattail, the base diameters agree, and HPR Sim's base term is `d_B/d_1` of MAPLEAF's: its relief behind a boattail.
- **Friction is higher in MAPLEAF on every part of every design, and every one of its friction lengths is the shorter.** A shorter length gives a higher coefficient on either branch. The rough wall's coefficient rises with `R_s/L`, and the turbulent one falls as the Reynolds number rises. The tables don't size the length's effect apart from two smaller differences in the rows above. MAPLEAF's compressibility factor on a rough wall lowers its friction, and its area on a narrowing part raises it.
- **The sources read for this report take a body's friction on one length.** Niskanen writes of the Reynolds number that "In most cases, the length used is the length of the rocket" (p. 20), and his one coefficient serves the body and the fins (eq. 3.85). MIL-HDBK-762's sample calculation (Table 5-4) takes the body's Reynolds number on the body's length. For the fins the sources differ: the handbook takes the fin's chord, as MAPLEAF does, and Niskanen the rocket's length, as HPR Sim does. [Issue #484](https://github.com/nrdptel/fusionspace-eridanus/issues/484) is open to measure the fins' choice on the Arcas wind-tunnel model.
- **Pressure drag carries almost none of it.** The two codes follow the same rules. In the table by part and term, no part's pressure drag differs by more than 0.0001.
- **Two terms belong to one code only.** MAPLEAF's thickness drag is under 0.002 on each design. On `valetudo` HPR Sim's rail buttons take back most of MAPLEAF's higher friction, which is why the whole difference there is small and the shares are large.

#### Against a measurement

Three committed measurements bear on where the two codes differ, and they don't all point one way. MAPLEAF was run on none of them as measured, so each item says only what the committed numbers allow.

- **NDRT 2020's logged flight.** NDRT 2020 is among the [real flights](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md), which HPR Sim flies on the launch day's weather and rail. On its own drag, HPR Sim's apogee is 10.39% above NDRT 2020's logged 1320.4 m. On a constant coefficient of 0.44, which the RocketPy example gives the rocket with no source, it is 0.16% above. MAPLEAF's coefficient just after burnout is 0.4522 here and HPR Sim's 0.3380. So on NDRT 2020's whole drag the log points to MAPLEAF as the nearer.
- **What that log doesn't say.** It doesn't say which term is short, or that drag alone is. A log holds the whole flight, with its mass, thrust and weather, and the design's fin edges and finish are placeholders for what the example doesn't record. MAPLEAF's own flight of the design here leaves a vertical rail in calm air, so its apogee is not the launch day's.
- **Bella Lui's logged flight.** HPR Sim's apogee is 1.11% above the logged 459.0 m, and the two codes' apogees here are 1.00% apart. That flight's inputs alone leave its apogee uncertain by 5.12%, so the log doesn't separate the two codes.
- **The base behind a boattail, which is `bella-lui`'s base difference.** The Hermes A-3A's base drag was measured in flight from Mach 0.65 to 0.79, above these flights' Mach numbers (`validation/fixtures/aero/base-drag-under-power.json`, power off). The flat-base coefficient on the base's own area, which is MAPLEAF's rule, reads 0.050 to 0.064 above the measured, 0.0574 in the mean. HPR Sim's rule with its relief reads 0.025 to 0.035 above, 0.0303 in the mean. Both read high, and HPR Sim's is the nearer.
- **The friction of a body.** The Arcas wind-tunnel model's forebody axial force leaves out the base (`validation/fixtures/aero/drag-vs-mach.json`). At Mach 0.6 with the fins off, HPR Sim's friction and pressure drag together read 13.8% above the shorter model's and 22.8% above the longer's. Most of that excess is the boattail's pressure drag, which both codes take by one rule ([why it misses](https://hpr.fusionspace.co/physics/aero.html#drag-against-the-arcas-robin-wind-tunnel)), so the measurement doesn't isolate the friction. By the formulas, a shorter friction length raises the friction and leaves the pressure drag as it is. A body part's own length, MAPLEAF's choice, would then read further above that measurement than the rocket's whole length does.

No committed measurement covers `ndrt-2020-nose-to-tail`'s largest term, the base behind an aft tube narrower than the rocket's largest. The Hermes reference holds HPR Sim's base rule the nearer behind a boattail, and the Arcas one reads high in HPR Sim before any higher friction. The one logged flight holds MAPLEAF's whole drag the nearer on NDRT 2020. This report does not settle which term leaves HPR Sim's drag short on that flight.

## What the two could not share

Each row is an input or a model the two codes differ in, in the flights named, with what was measured of it. *Not measured apart* means its effect is inside the differences above and has no number of its own.

| input | flights | HPR Sim | MAPLEAF | measured |
|---|---|---|---|---|
| Drag | own drag | its own drag buildup, part by part | its own, from the same outside shape and surface roughness, with its defaults kept (`Rocket.Aero.addZeroLengthBoatTailsToAccountForBaseDrag true`, `Rocket.Aero.fullyTurbulentBL true`) | The axial coefficient just after burnout, HPR Sim's less MAPLEAF's: -0.9% (`valetudo`), -11.0% (`bella-lui`), -25.2% (`ndrt-2020-nose-to-tail`). By part below. |
| Drag of the rail buttons | own drag | in its drag buildup | no such part | Of HPR Sim's axial coefficient just after burnout: 4.3% (`valetudo`), 1.6% (`bella-lui`), 1.2% (`ndrt-2020-nose-to-tail`). |
| Normal force and moments | every flight | its own | its own with its own drag; none with the same drag, where no shape is flown | Not measured apart. |
| Gravity | every flight | WGS 84 normal gravity at the site's latitude and the rocket's height, the centrifugal term in it | its `Flat` Earth model, the default: GM/(R + z)² with R = 6371 km, the same at every latitude. Its round and WGS 84 Earth models, which turn with the Earth and fly in an inertial frame, were not flown | At ignition, HPR Sim's against MAPLEAF's: 9.7864 against 9.8182 m/s² (-0.32%) (`valetudo`), 9.8069 against 9.8190 m/s² (-0.12%) (`bella-lui`), 9.8026 against 9.8196 m/s² (-0.17%) (`ndrt-2020-nose-to-tail`). With MAPLEAF's gravity matched to HPR Sim's at the pad, the same-drag apogee differences go from +0.35% (`valetudo`), +0.13% (`bella-lui`), +0.13% (`ndrt-2020-nose-to-tail`) to -0.0000% (`valetudo`), -0.0001% (`bella-lui`), -0.0013% (`ndrt-2020-nose-to-tail`) ([the table](#with-mapleafs-gravity-matched)). |
| The Earth's turning | every flight | the Coriolis force on the moving rocket | none in its `Flat` model | Not measured apart. |
| The air | every flight | the 1976 standard atmosphere, worked out at each height | its table of the U.S. Standard Atmosphere | The center of mass starts at the same height in both. The air density at ignition then differs by at most 2.6e-9 of itself. |
| The rail's lean | every flight | straight up, where the examples' rails lean 5.3° (`valetudo`), 1.0° (`bella-lui`), 0.0° (`ndrt-2020-nose-to-tail`) from the vertical | straight up too | On a rail 5.3° from the vertical MAPLEAF's rocket travels 10.60° from it with its body 5.30° from it, an angle of attack of 5.30° while still guided (`valetudo`, [below](#mapleaf-on-a-leaning-rail)). |
| Leaving the rail | every flight | guided until the airframe has traveled the guided length; no rail friction and no tip-off phase, neither of which MAPLEAF has | guided until its center of mass has traveled that length; the center moves forward in the airframe as the propellant burns, so the airframe has then traveled less | MAPLEAF frees the rocket with its airframe short of the guided length by 15.5 mm (`valetudo`), 6.6 mm (`bella-lui`), 16.2 mm (`ndrt-2020-nose-to-tail`). Both rail-exit speeds here are read where the airframe has traveled all of it. |
| Internal-momentum terms beside the thrust | every flight | −m r″ − 2ṁ r′ + m̈ (n − r) beside the thrust, as RocketPy has them: m is the rocket's mass, r its center of mass and n the nozzle exit. With the propellant's center c fixed in the airframe, as on these motors, they sum to m̈ (n − c) | none: its motor's force is the thrust curve alone | Added to MAPLEAF's thrust in its gravity-matched flight, that force moves MAPLEAF's apogee by -0.014 m (`valetudo`), -0.007 m (`bella-lui`), -0.104 m (`ndrt-2020-nose-to-tail`) and its rail-exit speed by -0.00039 m/s (`valetudo`), -0.00004 m/s (`bella-lui`), +0.00103 m/s (`ndrt-2020-nose-to-tail`). Between MAPLEAF's two time steps that flight's apogee moves by up to 0.065 m ([the table](#with-the-internal-momentum-force-added)). |
| Jet damping | every flight | the exhaust's damping of pitch and yaw | only as a `FractionalJetDamping` part, which these flights do not add | It acts on a rocket that turns, and these flights go straight up in calm air. Not measured apart. |
| Where the propellant's center of mass is | every flight | where its motor model puts it at each moment | on a straight line in the burnt fraction, between the two centers its motor file gives | HPR Sim's center leaves that line by at most 4.5e-16 m on these motors. |
| Mass and inertia between the motor file's rows | every flight | its motor model at every moment | straight lines between the rows HPR Sim's model was sampled into, 10 to each interval of the thrust curve | MAPLEAF's logged mass, center of mass and inertia at ignition and at burnout are the inputs' to one part in a million, checked on every flight in the tables. Between the rows, not measured. |
| Integration | every flight | Dormand–Prince 5(4) with an adaptive step, relative and absolute tolerance 1e-8 | classical fourth-order Runge–Kutta at a fixed 1 ms | At 2 ms MAPLEAF's apogee moves by at most 0.059 m with its own drag and 0.030 m with the same drag, and its rail-exit speed by at most 0.0003 m/s in either. The matched flights' steps are with their tables. |

## MAPLEAF on a leaning rail

Every flight here leaves a vertical rail (90° from the horizontal), though the examples' rails lean. Flown on `valetudo`'s rail as its example has it, 5.3° from the vertical, MAPLEAF at this commit moves the rocket up a line 10.60° from the vertical while its body points 5.30° from it: halfway up the rail (2.81 m of travel) the rocket is still guided and already flies at an angle of attack of 5.30°. A rail holds a rocket's travel and its axis on one line, so that is not the launch the input asks for, and the lean is left out of both codes' flights instead of compared.

## Running it again

```sh
cargo xtask mapleaf-flights --inputs   # HPR Sim's reading of the designs
UV_PROJECT_ENVIRONMENT=$PWD/refs/venv-mapleaf uv sync --locked --project validation/oracles/mapleaf
refs/venv-mapleaf/bin/python -I validation/oracles/mapleaf/flights.py   # MAPLEAF's flights
cargo xtask mapleaf-flights            # this report
```

`cargo xtask mapleaf-flights --check` flies HPR Sim again and fails unless the committed inputs, report and page reproduce; it refuses MAPLEAF flights that were not flown from the committed inputs (their SHA-256, `8578109cc75f89543719614c6955d5a2d620fc3178f5055a2235b20758e5b7c4`). MAPLEAF itself runs only where its environment is installed.
