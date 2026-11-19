# Real flights of the private collection: logged apogees (M2.3c1, rechecked in M2.3c5, halved in M2.7a, ADR-184, ADR-238, ADR-332)

Written by `cargo xtask fixture-flights`; do not edit. The flights are the tier-A flights of a private collection of designs paired with logs of the same flight (ADR-151). Its license allows only aggregate statistics, so no flight is named or numbered here. Tier A is the collection's first grading of its best-documented flights. Each flight's day, logged apogee, early deployment and reason to be left out were rechecked against the collection's catalog, a second reading of each flight's sources (ADR-238, which sets out what the recheck moved). The catalog proposes some flights below tier A: one of the design, motor, log, day or site inferred (B), or the design not shown to be the rocket flown (C).

## Flights

| | flights |
|---|---:|
| tier A | 89 |
| from a `.ork` | 83 |
| flown by hpr | 68 |
| flown by OpenRocket 24.12 | 79 |
| flown by both | 68 |
| left out of the errors (below) | 8 |
| compared with the log | 60 |
| of those, with an early deployment | 19 |
| of those, deployment not known | 0 |
| of those, proposed below tier A by the catalog | 11 |
| of those, in the held-out half (below) | 23 |
| of the 83, launch hour not known (local noon flown) | 26 |
| of the 83, rail not known (6 ft flown) | 2 |

## Not flown

| by | cause | flights | issue |
|---|---|---:|---|
| OpenRocket | OpenRocket loads no motor into that configuration | 1 | #362 |
| hpr | a freeform fin outline read as crossing itself | 3 | #358 |
| hpr | a motor whose digest OpenRocket 24.12's database lacks | 2 | #362 |
| hpr | a part of negative length (rail buttons spaced forward) | 1 | #359 |
| hpr | an airframe not read as written: a part inside an off-axis inner tube | 2 | #181 |
| hpr | an airframe not read as written: an unread surface finish | 3 | #360 |
| hpr | an airframe not read as written: mass and CG overrides that differ on the parts inside | 1 | #361 |
| the collection | the collection has no ERA5 file of the flight's day | 1 | #364 |
| the collection | the design holds no configuration with the flown motor | 2 | #363 |

## Left out of the errors

| reason | flights |
|---|---:|
| a design that is not the flown rocket | 1 |
| active control in the climb | 6 |
| another reason | 1 |

## The logs

60 barometric, 0 fused (barometer and accelerometer, read as barometric) and 0 satellite (GPS) apogees. Each barometric or fused reading is turned into the height climbed in the day's ERA5 air: the height above the pad that reads it, on an altimeter that converts pressure to the 1976 standard atmosphere's altitude. That height over the reading: mean 1.0366, least 0.9601, most 1.0777.

## Apogee errors

Each simulator's apogee less the logged one, over the logged one, per cent. hpr flies the whole ERA5 profile; OpenRocket the standard atmosphere through the pad's temperature and pressure, in the same wind every 50 m; "hpr in OpenRocket's air" is hpr there too; "hpr against OpenRocket" is hpr's apogee less OpenRocket's, over OpenRocket's. Nothing deploys in either. The mean absolute error averages the errors without their signs; "within 5%" and "within 10%" count the errors strictly smaller either way. The histogram's bins are bounded at -30, -25, -20, -15, -10, -5, +0, +5, +10, +15, +20, +25, +30%, each closed below.

### every flight compared (60 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +7.72% | +6.69% | 12.04% | 15.40% | 14 | 32 | 0 0 0 1 4 6 4 10 12 10 2 5 4 2 |
| OpenRocket | +6.72% | +6.08% | 11.26% | 14.25% | 13 | 31 | 0 0 0 1 6 6 3 10 12 8 5 6 1 2 |
| hpr in OpenRocket's air | +7.87% | +6.50% | 12.14% | 15.50% | 14 | 31 | 0 0 0 1 4 6 4 10 11 9 4 6 3 2 |
| hpr against OpenRocket | +0.90% | +0.53% | 1.63% | 2.34% | 59 | 60 | 0 0 0 0 0 0 17 42 1 0 0 0 0 0 |

### without the early deployments (41 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +3.33% | +4.72% | 9.42% | 11.11% | 10 | 26 | 0 0 0 1 4 6 3 7 10 7 0 1 2 0 |
| OpenRocket | +2.41% | +4.26% | 8.81% | 10.22% | 9 | 25 | 0 0 0 1 6 5 3 6 11 6 1 2 0 0 |
| hpr in OpenRocket's air | +3.48% | +4.86% | 9.51% | 11.18% | 10 | 25 | 0 0 0 1 4 6 3 7 9 8 0 2 1 0 |
| hpr against OpenRocket | +0.87% | +0.60% | 1.66% | 2.41% | 40 | 41 | 0 0 0 0 0 0 12 28 1 0 0 0 0 0 |

### without the flights the catalog proposes below tier A (49 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +6.96% | +6.27% | 11.54% | 14.11% | 11 | 27 | 0 0 0 0 4 6 4 7 10 7 2 4 4 1 |
| OpenRocket | +6.09% | +5.84% | 10.84% | 13.08% | 10 | 26 | 0 0 0 0 6 6 2 8 10 5 5 5 1 1 |
| hpr in OpenRocket's air | +7.09% | +6.48% | 11.64% | 14.20% | 11 | 27 | 0 0 0 0 4 6 4 7 10 5 4 5 3 1 |
| hpr against OpenRocket | +0.79% | +0.41% | 1.63% | 2.40% | 48 | 49 | 0 0 0 0 0 0 16 32 1 0 0 0 0 0 |

## The halves

The two halves of the flights compared (M2.7a, ADR-322, ADR-332). A flight is held out when SHA-256 of its id followed by the seven characters `ADR-209` starts with eight bytes that, read as a big-endian integer, are below 2⁶⁴ / 3; the rest are the calibration half. The held-out ids' digest, [`held-out.sha256`](held-out.sha256), was committed before any half's number. Only the calibration half may feed a fit across flights. Such a fit must take its flights through a guard that stops `cargo xtask fixture-flights` when one is held out; the guard checks only the fits that take it, and review holds a new fit to it. Today the calibration half's fit aggregates in `collection-boost.json` take it, and so do its logs' widened fits there. So does the calibration half's table below, which the scoreboard holds its collection rows by. The held-out half is held out, not blind: the first set above, over every flight compared, was published before the held-out half was sealed.

### the calibration half (37 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +4.22% | +4.21% | 9.42% | 11.56% | 11 | 23 | 0 0 0 1 2 6 2 9 6 7 1 1 2 0 |
| OpenRocket | +3.64% | +3.41% | 8.95% | 11.02% | 11 | 23 | 0 0 0 1 4 4 3 8 8 4 2 2 1 0 |
| hpr in OpenRocket's air | +4.36% | +3.80% | 9.53% | 11.70% | 11 | 23 | 0 0 0 1 2 6 2 9 6 6 2 1 2 0 |
| hpr against OpenRocket | +0.55% | +0.52% | 1.64% | 2.25% | 37 | 37 | 0 0 0 0 0 0 12 25 0 0 0 0 0 0 |

### the held-out half (23 flights)

| | mean | median | mean absolute | RMS | within 5% | within 10% | histogram |
|---|---:|---:|---:|---:|---:|---:|---|
| hpr | +13.36% | +12.69% | 16.26% | 20.10% | 3 | 9 | 0 0 0 0 2 0 2 1 6 3 1 4 2 2 |
| OpenRocket | +11.68% | +11.69% | 14.98% | 18.28% | 2 | 8 | 0 0 0 0 2 2 0 2 4 4 3 4 0 2 |
| hpr in OpenRocket's air | +13.52% | +13.03% | 16.34% | 20.16% | 3 | 8 | 0 0 0 0 2 0 2 1 5 3 2 5 1 2 |
| hpr against OpenRocket | +1.45% | +0.77% | 1.60% | 2.48% | 22 | 23 | 0 0 0 0 0 0 5 17 1 0 0 0 0 0 |

## OpenRocket's inputs along its flights

For M2.3c11 (ADR-349), `validation/oracles/openrocket/histories.py` flies each flight again as OpenRocket's flights above are flown and records, at OpenRocket's own time steps from launch to apogee, its thrust, mass, center of mass and drag coefficient. The record stays on this machine. M2.3c12 flies HPR Sim on those inputs to split the gap between the two simulators by cause. Of the 80 flights OpenRocket was given, 79 hold every input. Each of the rest is counted under the first reason, from the oracle's own list, that applies to it. OpenRocket isn't bit-identical from one run to the next: run again, the flights' apogees moved from those its record above holds by up to 2.3e-6 of themselves, so a flight is taken as the recorded one within 1e-5.

| why a flight lacks an input | flights |
|---|---:|
| OpenRocket didn't fly it to an apogee | 1 |
| the flight flown isn't the one OpenRocket's record above holds: its apogee moved by more than 1e-5 of itself | 0 |
| OpenRocket gave a thrust as not a number | 0 |
| OpenRocket gave a mass as not a number | 0 |
| OpenRocket gave a center of mass as not a number | 0 |
| OpenRocket gave a drag coefficient as not a number above Mach 0 | 0 |

## The gap to OpenRocket, by cause

For M2.3c12 (ADR-349), HPR Sim flies each flight again with OpenRocket's inputs from the section above swapped in for its own, one after another, each added to the ones before: OpenRocket's thrust against time, then its mass and center of mass against time, then its drag coefficient against Mach number, then its air (the standard atmosphere through the pad's temperature and pressure, in place of the day's profile). A part is the apogee before its swap less the apogee after it. Other is what thrust, mass and drag leave: the air's line, and the last flight's apogee less OpenRocket's own. So the parts sum to the whole, HPR Sim's apogee less OpenRocket's, and the run stops if they don't on a flight. Every number is a share of OpenRocket's apogee. This is code against code: it sizes where the two simulators differ and says nothing about which is nearer a flight.

Of the 68 flights both simulators fly, 68 are split. These include the flights left out of the errors above, whose logs are no fair test: the split needs no log. A part dominates a flight when its absolute value is the largest of thrust, mass, drag and other, the earlier in that order if two tie. The last column counts the flights where a part's absolute value is over 1%.

| part | mean | median | mean absolute | flights it dominates | flights where it is over 1% |
|---|---:|---:|---:|---:|---:|
| thrust | +0.00% | +0.00% | 0.00% | 0 | 0 |
| mass | +0.10% | +0.00% | 0.10% | 0 | 1 |
| drag | +0.58% | +0.44% | 1.57% | 59 | 31 |
| other | +0.03% | +0.01% | 0.26% | 9 | 4 |
| other: the air | -0.15% | -0.17% | 0.25% |  |  |
| other: the rest | +0.18% | +0.19% | 0.33% |  |  |
| the whole, on the flights split | +0.71% | +0.50% | 1.61% |  |  |
| the whole, on every flight both fly | +0.71% | +0.50% | 1.61% |  |  |
| drag, swapped first | +0.58% | +0.44% | 1.57% |  |  |

The parts depend on the order. The last row is the drag part with the drag swapped first, on HPR Sim's own thrust and mass, beside the drag row's, which comes after thrust and mass. The drag swap is a table in Mach number, power on and power off, read from OpenRocket's own flight, so it leaves out how OpenRocket's drag depends on the Reynolds number and the angle of attack away from that flight's path. The mass swap leaves HPR Sim's inertia and its center of pressure in place, and no swap replaces its normal force, its center of pressure or its damping of a turn: those are in the rest. So where the two simulators put the center of mass apart, HPR Sim flies OpenRocket's center of mass against its own center of pressure from the mass swap on, which moves its stability margin. What that does to the apogee is in the mass row, and no later swap undoes it: the last flight keeps it, and the rest is measured from that flight to OpenRocket's own. So the rest can take back what the mass row holds. Of the 1 flight where the mass part is over 1%, the rest is over 1% with the other sign on 1, which fits that reading and doesn't prove it. The air's line is taken with the drag table already in place, so it holds the air's density and speed of sound and not what the air does to friction. Each flight split also passed M2.3c11's test: flown on its own thrust and mass sampled at OpenRocket's times, it came back to its own apogee within 0.05%; the largest miss was 0.018%.

A flight both fly that is not split is counted under the first reason that applies:

| why a flight is not split | flights |
|---|---:|
| OpenRocket's record lacks an input for the flight | 0 |
| HPR Sim refuses a history or fails the swapped flight | 0 |
| the flight on its own thrust and mass, sampled at OpenRocket's times, misses its own apogee by more than 0.05% | 0 |

### The tangent-ogive nose's line

For M2.3c13 (ADR-349, ADR-350), the drag part is split once more. From Mach 1.3 up, OpenRocket 24.12 gives a tangent-ogive nose almost no pressure drag, where HPR Sim gives it the drag of the cone of its length and base; between Mach 1 and 1.3 each joins that to its own value at Mach 1 (OpenRocket's issue #3391, fixed on its development line; no release carried the fix by 2026-11-14). On OpenRocket's thrust and mass, HPR Sim flies once more with its own pressure drag of the nose replaced, above Mach 1, by the pressure drag OpenRocket's public interface reports for the nose; everything else in the drag stays HPR Sim's. The nose's line is the apogee before that swap less the apogee after it. The rest of the drag is the apogee after it less the apogee on OpenRocket's whole drag, so the two sum to the drag part, and the run stops if they don't on a flight.

The line's flights are those whose own HPR Sim flight passes Mach 1 with a tangent-ogive nose at the front of the airframe: 5 of the flights both simulators fly. Of those, 5 are swapped.

| over the 5 flights swapped | mean | median | mean absolute |
|---|---:|---:|---:|
| the nose's pressure drag above Mach 1 | -1.28% | -1.33% | 1.28% |
| the rest of the drag | -0.68% | +0.90% | 2.27% |
| the drag part | -1.96% | +0.51% | 2.65% |

Each is per cent of OpenRocket's apogee. The top Mach numbers of these flights, by HPR Sim's own flights, average 1.06, and 0 of the 5 pass Mach 1.3, from which the public probe shows OpenRocket giving the nose almost no pressure drag. The line is measured that far past Mach 1, and says nothing of a faster flight.

A positive line means HPR Sim climbs higher on its own nose drag than on OpenRocket's, a negative one lower. A flight of the line that is not swapped is counted under the first reason that applies:

| why a flight is skipped | flights |
|---|---:|
| the design holds more than one nose cone, and no probe shows how OpenRocket's column counts a repeated nose | 0 |
| the flight is not split, as counted above | 0 |
| OpenRocket's record holds no column for the nose | 0 |
| the design states the nose's drag, so HPR Sim has no pressure drag of its own to swap | 0 |
| HPR Sim fails the flight with the column swapped in, or the flight leaves the Mach numbers the column covers | 0 |

### The drag part by Mach band

For M1.14i1 (ADR-359), the drag part is split by the Mach number the drag acts at. On OpenRocket's thrust and mass, HPR Sim flies with OpenRocket's drag coefficient in place of its own below Mach 0.8 only, then below Mach 1.2 too, then at every Mach number. Outside the bands swapped the drag is HPR Sim's own, so at a band's edge the coefficient steps from one program's to the other's. A band's part is the apogee before its swap less the apogee after it. The three sum to the drag part: the last of these flights is the split's flight on OpenRocket's whole drag, and the run stops on a flight where it isn't. A swap that changes no drag the flight reads leaves the flight the same to the last digit, so a flight has a part in a band when its part there is not zero: the swap changed the flight. Those are the flights that reach the band, or come close enough that a step of the integrator reads the drag inside it. Like the split, the parts are code against code.

Of the 68 flights split, 68 are split by band; on 0 HPR Sim failed a flight with a band swapped.

| band | flights with a part in it | mean | median | mean absolute | mean absolute over those flights | flights it dominates | flights where it is over 1% |
|---|---:|---:|---:|---:|---:|---:|---:|
| below Mach 0.8 | 68 | +0.15% | +0.19% | 0.87% | 0.87% | 45 | 16 |
| Mach 0.8 to 1.2 | 28 | +0.20% | +0.00% | 0.50% | 1.21% | 16 | 9 |
| above Mach 1.2 | 7 | +0.23% | +0.00% | 0.23% | 2.21% | 7 | 7 |
| the drag part | 68 | +0.58% | +0.44% | 1.57% |  |  |  |

Each percentage is a share of OpenRocket's apogee, over the 68 flights split by band, but for the column over the flights with a part in the band. A band dominates a flight when its part's absolute value is the largest of the three, the slower band if two tie. The last column counts the flights where that absolute value is over 1%.

A band's part depends on the speed the slower bands' swaps leave the flight with. So each band is also swapped alone, first: on HPR Sim's own thrust and mass, with its own drag in the other two bands.

| band, swapped alone | flights it changes | mean | median | mean absolute |
|---|---:|---:|---:|---:|
| below Mach 0.8 | 68 | +0.15% | +0.19% | 0.87% |
| Mach 0.8 to 1.2 | 28 | +0.20% | +0.00% | 0.50% |
| above Mach 1.2 | 7 | +0.24% | +0.00% | 0.24% |
| every band: the drag part, swapped first |  | +0.58% | +0.44% | 1.57% |

The flights with a band swapped alone are also set beside the logs. Of the flights compared with a log above, 60 are split by band. Each row is the apogee less the logged one, over the logged one, on those flights: HPR Sim's own flight; HPR Sim on OpenRocket's drag in one band alone; HPR Sim on OpenRocket's whole drag; and OpenRocket's own flight. A row moves a flight when its apogee is not that of HPR Sim's own flight, and the last two columns count the moved flights that come nearer their log than HPR Sim's own flight, and the ones that go farther.

| HPR Sim's drag | flights moved | mean | median | mean absolute | flights nearer their log | flights farther from it |
|---|---:|---:|---:|---:|---:|---:|
| its own |  | +7.72% | +6.69% | 12.04% |  |  |
| OpenRocket's below Mach 0.8, its own above | 60 | +7.42% | +6.70% | 11.65% | 39 | 21 |
| OpenRocket's from Mach 0.8 to 1.2, its own elsewhere | 25 | +7.44% | +6.78% | 11.79% | 14 | 11 |
| OpenRocket's above Mach 1.2, its own below | 7 | +7.44% | +5.99% | 11.86% | 6 | 1 |
| OpenRocket's at every Mach number | 60 | +6.85% | +6.33% | 11.27% | 33 | 27 |
| OpenRocket's own flight, for comparison | 60 | +6.72% | +6.08% | 11.26% | 36 | 24 |

A row nearer the logs doesn't show that OpenRocket's drag is the truer one in that band. Both simulators climb above the logs on average, so more drag from any cause moves the mean toward them, and a wrong mass or thrust would read the same way. The rows aren't a model HPR Sim could fly either: the coefficient is read along OpenRocket's own flight of the same design. They say which band's difference the comparison with the logs feels.

## Sources

Flight data published by university rocketry teams, rocketry courses and hobbyists on GitHub, team websites and The Rocketry Forum.

Generated using Copernicus Climate Change Service information 2026. Contains modified Copernicus Climate Change Service information 2026 (the Earth Data Hub subsets). Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2020), *The ERA5 global reanalysis*, Q. J. R. Meteorol. Soc. 146, 1999–2049, <https://doi.org/10.1002/qj.3803>; ERA5 hourly data on single levels (<https://doi.org/10.24381/cds.adbb2d47>) and on pressure levels (<https://doi.org/10.24381/cds.bd0915c6>), Copernicus Climate Data Store.
