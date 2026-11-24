# hpr against the logs of real flights

Written by `cargo xtask real-flights` ([M2.3b][m2-3b], decision [ADR-082][adr-082]; the flights predicted first: [M2.3d2][m2-3d2], decisions [ADR-328][adr-328] and [ADR-342][adr-342]). How it is done, and what it means, is on the [documentation site][site].

- **What is flown:** 7 of the rockets RocketPy's documentation flies against their teams' altitude logs. hpr flies each with its own aerodynamics, on the example's own thrust file, from the example's rail and site, in the ERA5 weather the example reads. The log is the reference. They were compared as soon as they were flown.
- **Predicted first:** 5 more public flights, each predicted in a pull request merged before its log was compared ([the predictions][predictions], which give their inputs). Their rows are in [their own section](#predicted-before-they-were-compared); the summary counts all 12.
- **Heights:** a barometric altimeter reads the standard atmosphere's altitude of the pressure it measures, less the pad's. hpr's height is read the same way, from the pressure of the air it flew in at its center of mass, less the start's. Each log of the flights compared first is read up to its apogee, stopping before the recovery's pressure transients; each predicted flight's is read by the rule merged with its prediction.
- **Trace RMS:** over the ascent, both clocks aligned where the trace first reaches 30 m, until the first of the two apogees. Only the flights compared first have one.
- **Files:** the logs, thrust files and weather files are read from the pinned checkouts and never committed; each one's SHA-256 is in the JSON, the predicted flights' inputs' in the predictions' JSON.

[m2-3b]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3b
[adr-082]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-082-real-flights-read-from-refs-compared-over-the-ascent-with-checked-explanations-2026-09-26
[m2-3d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d2
[adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-342]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0342-the-errors-before-the-remaining-openrocket-parity-probes.md
[predictions]: public-predictions.md
[site]: https://hpr.fusionspace.co/accuracy.html#real-flights

## Summary

- Flights: 12: 7 compared as soon as they were flown, 5 predicted first.
- Mean absolute apogee error: 8.83% against a target of 5%: outside target.
- Mean apogee error with its sign: -4.61%.
- Over the 7 compared first: mean absolute apogee error 5.97%, with its sign +0.07%.
- Over the 5 predicted first: mean absolute apogee error 12.82%, with its sign -11.17%.
- Over the 2 tagged, with their predictions, as having every input traced: mean absolute apogee error 7.30%, with its sign -7.30%. The flights compared first were compared before tags existed and carry none, so they are not counted here.
- Over the 10 flown in ERA5 weather: mean absolute apogee error 6.05%, with its sign -0.99%.
- On the predictions as first merged, each revised one put back: mean absolute apogee error 8.95%, with its sign -4.74%.
- Mean absolute apogee error were every log a height, not a barometric reading: 8.00%.
- Mean absolute apogee error with only the logs known to be barometric read so, the assumed ones as heights: 8.96%.
- Largest trace RMS, over the 7 compared first: 6.80% of its log's apogee.
- Outside the target: ndrt-2020, prometheus-2022, juno-iii, cavour, genesis, astra, erebus-11, macie-xl-1, macie-xl-2.
- On each example's own drag, the diagnostic flight, over the 7 compared first: mean absolute apogee error 3.33%.

## Flights compared first

| flight | altimeter | log apogee (m) | hpr apogee (m) | apogee error | hpr apogee as a height (m) | log time to apogee (s) | hpr time to apogee (s) | trace RMS (m) | trace RMS (% of apogee) | rows | hpr's largest Mach |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | barometric, assumed | 459.0 | 464.1 | +1.11% | 464.8 | 9.46 | 9.50 | 3.7 | 0.81% | 161 | 0.265 |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | barometric | 1320.4 | 1457.5 | +10.39% | 1432.7 | 15.76 | 16.71 | 89.8 | 6.80% | 316 | 0.536 |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | barometric | 3895.8 | 3536.6 | -9.22% | 3796.4 | 28.44 | 26.50 | 196.2 | 5.04% | 2870 | 0.959 |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | barometric | 3151.5 | 2976.7 | -5.55% | 3231.4 | 23.62 | 25.76 | 207.4 | 6.58% | 473 | 0.732 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | barometric, assumed | 2785.0 | 2945.5 | +5.76% | 3051.6 | 22.38 | 22.68 | 124.2 | 4.46% | 120 | 0.933 |
| Genesis, EuRoC 2023 (L995) | barometric, assumed | 2916.7 | 2745.2 | -5.88% | 2874.5 | 23.33 | 22.44 | 95.6 | 3.28% | 2244 | 0.854 |
| Lince, EuRoC 2023 (M1101) | barometric, assumed | 3587.7 | 3728.0 | +3.91% | 3894.0 | 25.25 | 26.49 | 68.9 | 1.92% | 2526 | 0.994 |

## Predicted before they were compared

Each of these flights' predictions was merged before its log was read as numbers ([the predictions][predictions]). The merged prediction is the number that pull request printed; hpr's apogee today is what `cargo xtask real-flights` flies now, and the error counted is today's. A merged prediction is never edited: one that changed is revised beside it, with the reason, and both errors are given. The log's apogee is read by the rule merged with the prediction. No explanation is claimed for a miss here: the two diagnostics below are not flown for these flights, and what each record leaves untraced was written down with the prediction.

| flight | altimeter | log apogee (m) | merged prediction (m) | hpr apogee today (m) | apogee error | merged prediction's error | every input traced | hpr apogee as a height (m) | log rows | hpr's largest Mach | impulse flown (N s) |
|---|---|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) | barometric, assumed | 3249.5 | 3037.1 | 3037.1 | -6.54% | -6.54% | yes | 3155.3 | 64 | 0.911 | 4280.2 |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | barometric, assumed | 3438.5 | 3581.0 | 3580.6 | +4.13% | +4.14% | no: not traced | 3729.1 | 1139 | 1.019 | 9866.7 |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) | barometric, assumed | 3001.8 | 2714.1 | 2759.6 | -8.07% | -9.58% | yes | 2856.0 | 746 | 0.877 | 2418.2 |
| Macie XL, flight 1 (AeroTech I284W) | barometric | 842.8 | 706.2 | 706.2 | -16.20% | -16.21% | no: not traced | 706.3 | 2952 | 0.354 | 599.5 |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | barometric | 977.8 | 692.6 | 692.6 | -29.16% | -29.17% | no: not traced | 692.7 | 2312 | 0.362 | 600.8 |

Shown beside them and never counted in a mean:

| flight | what | apogee (m) | error against the log |
|---|---|---:|---:|
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | on the catalog's curve, Cesaroni 8429M2020-P | 3050.0 | -11.30% |
| Macie XL, flight 1 (AeroTech I284W) | OpenRocket 24.12's simulation stored in the design file | 704.6 | -16.40% |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | OpenRocket 24.12's simulation stored in the design file | 691.6 | -29.26% |

- **Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350)** (`astra`): the design built from RocketPy's example, `rocketpy-astra`, flown in the ERA5 weather of 2022-10-13T15:00Z, on 4280.2 N s of impulse. The log is `refs/rocketpy/data/rockets/astra/flight_data.csv`, 64 rows. Its apogee is read as the highest reading of the log's `alt` column, in meters, the log read whole. Revised, because M1.14b15 (ADR-363, decision 1) made the tip-off phase the default. Astra has two rail buttons, so between the forward button's leaving the rail and the aft one's it turns about the aft button, and it leaves the rail turning (ADR-357, ADR-358). Nothing in its design or inputs changed. Every input is traced to its record.
- **Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve)** (`andromeda`): the design built from RocketPy's example, `rocketpy-andromeda`, flown in the ERA5 weather of 2022-10-14T13:00Z, on 9866.7 N s of impulse, +17.1% against the 8428.7 N s of the catalog's curve shown above. The log is `refs/rocketpy/data/rockets/andromeda/flight_data.csv`, 1139 rows. Its apogee is read as the highest reading of the log's `alt(m)` column, the log read whole. Revised, because M1.14b15 (ADR-363, decision 1) made the tip-off phase the default. Andromeda has two rail buttons, so between the forward button's leaving the rail and the aft one's it turns about the aft button, and it leaves the rail turning (ADR-357, ADR-358). Nothing in its design or inputs changed. Not traced: the flight carried the team's air brakes, and no record says when or how far they opened: none is flown here, so any opening leaves this prediction high; the thrust file holds more impulse than the Cesaroni M2020 reload it is named for is certified at, so the notebook does not settle which motor the logged flight burned.
- **Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K)** (`erebus-11`): the design built from RocketPy's example, `rocketpy-erebus-11`, flown in the ERA5 weather of 2022-10-15T15:00Z, on 2418.2 N s of impulse. The log is `refs/rocketpy/data/rockets/erebus11/flight_data_filtered.csv`, 746 rows. Its apogee is read as the highest reading of the log's `alt` column, in meters, the log read whole. Revised, because Two changes to HPR Sim. M1.14i3 (ADR-362) changed its base drag below Mach 0.8: behind a boattail the base is d_B/d of a flat base's value (NASA TR R-100, 1961, p. 11), set on the Hermes A-3A's measured base drag (NACA RM SL55F15), where the old rule read 0.050 to 0.064 high on its largest cross-section and the new one 0.025 to 0.035. Erebus 11 has a boattail, and its apogee rose 46.0 m to 2760.1 m. Then M1.14b15 (ADR-363, decision 1) made the tip-off phase the default: Erebus 11 has two rail buttons and leaves the rail turning (ADR-357, ADR-358), which took 0.5 m off. Nothing in its design or inputs changed. Every input is traced to its record.
- **Macie XL, flight 1 (AeroTech I284W)** (`macie-xl-1`): the design file `refs/public-flights/rocketry-files/macie/macie-xl.ork` as saved, as its stored simulation `maciexl-1-or` flies it, flown in the 1976 US Standard Atmosphere, for the stored simulation's `isa`, on 599.5 N s of impulse. The log is `refs/public-flights/rocketry-files/macie/flights/MRC Summer 26/maciexl-1-sl.pf2`, 2952 rows. Its apogee is read as the apogee the altimeter worked out, in the log's header (its `Apogee:` line, in feet above the pad, 0.3048 m to the foot), which leaves out the ejection spike among the samples; never the highest sample. Its highest sample is 1052.8 m, +210.0 m from that apogee: shown, not compared. Not traced: no record gives the day, its air and wind, or the rail, and of the site only each log's header gives the pad's height above sea level (12 ft for flight 1, 37 ft for flight 2): the stored simulation's are flown, its sea level among them, and all nine simulations in the file carry the same ones; no record says the rocket was weighed as flown: the file's parts and overrides are flown; the reader warns that the motor's curve file disagrees with ThrustCurve.org's record of the motor (the figures are in the flight's `reader_warnings` in the JSON); the file the design's digest names is flown; the motor flies the bundled catalog's masses for it, 313.6 g of propellant and 555.52 g loaded, as every catalog motor does, where the curve file the design's digest names, and OpenRocket's own record of that digest, give 310.016 g and 555.072 g (the table's `mass_difference`).
- **Macie XL, flight 2 (Cesaroni 601I350-16A)** (`macie-xl-2`): the design file `refs/public-flights/rocketry-files/macie/macie-xl.ork` as saved, as its stored simulation `maciexl-2-or` flies it, flown in the 1976 US Standard Atmosphere, for the stored simulation's `isa`, on 600.8 N s of impulse. The log is `refs/public-flights/rocketry-files/macie/flights/MRC Summer 26/maciexl-2-sl.pf2`, 2312 rows. Its apogee is read as the apogee the altimeter worked out, in the log's header (its `Apogee:` line, in feet above the pad, 0.3048 m to the foot); never the highest sample. Its highest sample is 978.7 m, +0.9 m from that apogee: shown, not compared. Not traced: no record gives the day, its air and wind, or the rail, and of the site only each log's header gives the pad's height above sea level (12 ft for flight 1, 37 ft for flight 2): the stored simulation's are flown, its sea level among them, and all nine simulations in the file carry the same ones; no record says the rocket was weighed as flown: the file's parts and overrides are flown; the reader warns that the motor's curve file disagrees with ThrustCurve.org's record of the motor (the figures are in the flight's `reader_warnings` in the JSON); the file the design's digest names is flown.

## On each example's own drag

A diagnostic, not a prediction, flown for the flights compared first: the same flight with hpr's zero-lift drag replaced by the drag the example's notebook specifies (a team's estimate: a table, a curve from RASAero II or CFD, or a constant), on the example's radius. hpr's normal force is its own in both. Where the error falls within the target, the miss is consistent with hpr's drag; where it doesn't, it is elsewhere. The teams' drags are estimates too, and one may have been tuned to its flight.

| flight | example's drag | apogee (m) | apogee error | trace RMS (m) |
|---|---|---:|---:|---:|
| Bella Lui, EPFL Rocket Team, 2020 (K828FJ) | bella_lui_flight_sim.ipynb:94-95, :453-484 (power off and on, times 1) | 460.1 | +0.24% | 2.1 |
| NDRT 2020, Notre Dame Rocketry Team (L1395) | ndrt_2020_flight_sim.ipynb:91 and :316-317 (the drag coefficient, power off and on) | 1322.5 | +0.16% | 31.7 |
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | prometheus_2022_flight_sim.ipynb:224-261 (`prometheus_cd_at_ma` from Mach 0.15, where it starts to change, and power on 1.02 times it) | 3899.3 | +0.09% | 95.9 |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | juno3_flight_sim.ipynb:244-251, :356-359 (`drag_curve.csv`, scaled to 0.38 at Mach 0.6 "from CFD analysis") | 2891.7 | -8.24% | 243.2 |
| Cavour, Politecnico di Torino, EuRoC 2023 (L995) | cavour_flight_sim.ipynb:250-251 | 2724.6 | -2.17% | 27.1 |
| Genesis, EuRoC 2023 (L995) | genesis_flight_sim.ipynb:241-242 | 2913.6 | -0.10% | 32.7 |
| Lince, EuRoC 2023 (M1101) | lince_flight_sim.ipynb:240-241 | 3146.2 | -12.31% | 281.9 |

## On the thrust file as recorded

A second diagnostic, where an example reshapes its thrust file to a burn time and a total impulse: the same flight on the file as recorded (negative points read as zero), on hpr's own drag. Where the error falls within the target and the example's drag doesn't bring it there, the miss is consistent with the impulse the example sets.

| flight | impulse as recorded (N s) | impulse reshaped (N s) | apogee (m) | apogee error |
|---|---:|---:|---:|---:|
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | 9251.7 | 8802.6 | 3246.6 | +3.02% |

## Against the logs' own pressure and satellite heights

Where a log keeps the pressure its height was read from, the height column's largest gap from the standard atmosphere's reading of that pressure shows how the altimeter reads. Where the flight logged a satellite (GNSS) altitude, its apogee above the pad is a geometric height: the log's barometric apogee over it is what the day's air did to the reading, beside what hpr's conversion does to its own apogee.

| flight | height against its pressure (largest gap, m) | satellite apogee (m) | log apogee over it | hpr's reading over its height |
|---|---:|---:|---:|---:|
| Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520) | 0.195 | 4133.0 | 0.943 | 0.932 |
| Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor) | 1.321 | 3369.3 | 0.935 | 0.921 |

## Each flight's inputs

The flights compared first; a predicted flight's inputs are in [the predictions][predictions].

- **Bella Lui, EPFL Rocket Team, 2020 (K828FJ)** (`rocketpy-bella-lui`): weather at 2020-02-22T13:00Z; inputs from RocketPy 1.13.0's bella_lui_flight_sim.ipynb:109-111 (rail), :139-153 (site, date), :761-765 (log); altimeter: the team's own avionics, filtered by a method the example doesn't record: barometric assumed, as hobby altimeters are; total impulse flown 2071.0 N s.
- **NDRT 2020, Notre Dame Rocketry Team (L1395)** (`rocketpy-ndrt-2020-nose-to-tail`): weather at 2020-02-23T16:00Z; inputs from RocketPy 1.13.0's ndrt_2020_flight_sim.ipynb:115-117 (rail), :148-153 (site, date), :679-683 (log); altimeter: a Featherweight Raven (RocketPy's tests/acceptance/test_ndrt_2020_rocket.py:190), a barometric altimeter, in feet above the pad; total impulse flown 4894.9 N s. Outside the target (drag): consistent with hpr's drag. On the example's own drag, a constant 0.44 the notebook gives no source for, the apogee is within the target. hpr's own drag is lower, as the predicted-mode comparison with RocketPy flying the same constant found (+10.3% in apogee), and the design's fin edges and finish are placeholders, since the example records none.
- **Prometheus, Western Engineering, Spaceport America Cup 2022 (M1520)** (`rocketpy-prometheus-2022-generic-motor`): weather at 2023-06-24T15:00Z; inputs from RocketPy 1.13.0's prometheus_2022_flight_sim.ipynb:65-70 (site, date), :401-406 (rail), :508-526 (log); altimeter: an Altus Metrum TeleMetrum: its height column is the standard atmosphere's altitude of its pressure column less the first row's (the gap is below); total impulse flown 7579.1 N s. Note: flown in the weather of 24 June 2023, a year after the flight, as RocketPy's example flies it: RocketPy has no ERA5 file of the day. The log is read to 29.58 s: a pressure transient then drops the reading 600 m and returns it 8 m above the highest reading before. Outside the target (drag): consistent with hpr's drag. On the example's own drag, the team's table from Mach 0.15, the apogee is within the target. The reading is built on weather a year off the flight's day (the note). Against the satellite heights, hpr's conversion reads 1 to 2 points below the altimeter here and on Juno III, which flew in its own day's weather, so the wrong day's share of the miss can't be told apart.
- **Juno III, Projeto Jupiter, Spaceport America Cup 2023 (the team's motor)** (`rocketpy-juno-iii`): weather at 2023-06-23T23:00Z; inputs from RocketPy 1.13.0's juno3_flight_sim.ipynb:54-67 (site, date), :448-456 (rail), :543-553 (log); altimeter: a Missile Works RRC3 (juno3/README.txt:21): its height column is the standard atmosphere's altitude of its pressure column less the first row's (the gap is below); total impulse flown 8802.6 N s, 2.62 N s of it from reading the file's negative thrust as zero. Note: the log is read to 24.60 s, as it levels off: a pressure transient there dips the reading by 94 m, then lifts it 62 m above the level within 0.3 s, to the 3213.4 m the team reports as its apogee, and the record's last two rows are corrupt. At the cut the log's own velocity column still reads 17.5 m/s up, so its apogee may be 10 m to 20 m low. The thrust file's last five points are negative (-6.8 to -47.3 N); hpr, which refuses a negative thrust, reads them as zero, and RocketPy's flight holds its thrust at zero too. Outside the target (thrust): consistent with the motor's impulse. The notebook reshapes the team's own motor curve (`mandioca_thrust_curve.csv`) to 5.8 s and 8800 N s, 4.9% less than the file; on the file as recorded the apogee is within the target, and on the team's drag it is not.
- **Cavour, Politecnico di Torino, EuRoC 2023 (L995)** (`rocketpy-cavour`): weather at 2023-10-13T12:00Z; inputs from RocketPy 1.13.0's cavour_flight_sim.ipynb:125-132 (site, date), :314-315 (rail), :379-389 (log); altimeter: the CATS Vega EuRoC 2023 required, its radio telemetry: the Kalman filter's height from a barometer and an accelerometer, sent in whole meters cut toward zero (as matched on its sister flights' logs), with no pressure; barometric assumed; total impulse flown 3618.0 N s. Note: the log is read to 23.6 s, the first row the team's ground-station log tags Drogue: past it, the radio telemetry's filtered height rises 8 m in 0.2 s, to 2789 m, faster than the rocket can climb and about 2 s before the drogue channel's continuity is first lost (25.6 s); the coast's peak before it is 2785 m. Outside the target (drag): consistent with hpr's drag. On the example's own curves, labeled RASAero II, the apogee is within the target. hpr's drag is below them: at Mach 0.3, 8.3% below power off and 18.3% below power on (the aerodynamics page's comparison; the power-on gap's cause is open), and the design's fin edges are placeholders, since the example records none.
- **Genesis, EuRoC 2023 (L995)** (`rocketpy-genesis`): weather at 2023-10-12T13:00Z; inputs from RocketPy 1.13.0's genesis_flight_sim.ipynb:124-131 (site, date), :326-327 (rail), :391-406 (log); altimeter: a filtered estimate (`filtered_altitude_AGL`), probably the CATS Vega's barometer and accelerometer Kalman filter: barometric assumed; total impulse flown 3618.0 N s. Outside the target (drag): consistent with hpr's drag. On the example's own curves the apogee is within the target; the design's fin edges and finish are placeholders, since the example records none.
- **Lince, EuRoC 2023 (M1101)** (`rocketpy-lince`): weather at 2023-10-12T10:00Z; inputs from RocketPy 1.13.0's lince_flight_sim.ipynb:123-129 (site, date), :432-437 (rail), :609-619 (log); altimeter: a filtered estimate (`filtered_altitude_AGL`, as Genesis's) from a computer the example doesn't name: barometric assumed; total impulse flown 5199.6 N s. Note: the log is read to 26.80 s: past it, as the recovery fires, the filtered height swings by hundreds of meters, up to 3668.5 m; its highest reading before is the 3587 m the team reports as its apogee.

## How much of each error the integrator could be

After ASME V&V 20, a standard for putting an uncertainty on a comparison of a simulation with a measurement ([M1.14b16][m1-14b16], decision [ADR-373][adr-373]):

- **E**, the comparison error, is hpr's apogee less the log's.
- E is the model's own error plus three others: from the integrator, from the inputs and from the log. Each of the three has a standard uncertainty, and while they are independent the validation uncertainty u_val is their root sum of squares.
- **u_num**, the integrator's part, is what this section gives. Each flight is flown twice more: as the report flies it, and with the adaptive stepper's tolerances divided by 32. The stepper's step goes as its tolerance to the power 1/5, so the second run takes about twice the steps; r is the measured ratio, the second run's steps to apogee over the first's. E is the first run's, the coarser, so its index is Roache's grid convergence index for the coarser of two runs, 3·|difference|·r⁴/(r⁴ − 1). That is a band of about 95%, about two standard deviations, so u_num is half of it.
- The data part, the log's own apogee uncertainty, is the next section's ([M1.14b17][m1-14b17]). The input part and the band, E ± 2·u_val, are the two sections after it ([M1.14b18][m1-14b18]).

How far to trust it: u_num measures what a finer step changes, not the models, the inputs or the logs. The order 4 is assumed, the dense output's formal order; the flights' own order isn't measured, and a lower one would make the index larger.

| flight | E (m) | apogee (m) | steps (report's run) | apogee, tolerances ÷ 32 (m) | steps (finer run) | r | u_num (mm) |
|---|---:|---:|---:|---:|---:|---:|---:|
| bella-lui | 5.1 | 464.060607 | 331 | 464.059454 | 635 | 1.92 | 1.8674 |
| ndrt-2020 | 137.1 | 1457.501793 | 286 | 1457.500944 | 558 | 1.95 | 1.3677 |
| prometheus-2022 | -359.2 | 3536.629640 | 488 | 3536.630044 | 968 | 1.98 | 0.6482 |
| juno-iii | -174.8 | 2976.686420 | 659 | 2976.686418 | 1172 | 1.78 | 0.0025 |
| cavour | 160.5 | 2945.476584 | 671 | 2945.476628 | 1322 | 1.97 | 0.0708 |
| genesis | -171.5 | 2745.186999 | 452 | 2745.186560 | 898 | 1.99 | 0.7035 |
| lince | 140.3 | 3728.031681 | 580 | 3728.031362 | 1139 | 1.96 | 0.5127 |
| astra | -212.4 | 3037.091872 | 550 | 3037.092662 | 1087 | 1.98 | 1.2670 |
| andromeda | 142.0 | 3580.592861 | 2792 | 3580.590588 | 3505 | 1.26 | 5.7093 |
| erebus-11 | -242.1 | 2759.618432 | 651 | 2759.619911 | 1224 | 1.88 | 2.4123 |
| macie-xl-1 | -136.5 | 706.244763 | 350 | 706.245056 | 687 | 1.96 | 0.4720 |
| macie-xl-2 | -285.2 | 692.637902 | 348 | 692.638274 | 676 | 1.94 | 0.5993 |

The largest u_num is 5.7093 mm (andromeda). Every flight's |E| is at least 2,700 times its own u_num; the fewest times is on bella-lui.

## How much of each error the log could be

This section puts a correction and a standard uncertainty, the data part u_D, on each log's own apogee, so that the band, below, can say how much of E the log could explain ([M1.14b17][m1-14b17], decision [ADR-375][adr-375]). It uses published figures for a kind of altimeter or its pressure sensor; they are not calibrations of the units that flew.

- **What is uncertain.** The report takes both apogees as barometric readings: the 1976 standard atmosphere's altitude of the pressure at the apogee, less that of the pressure on the pad. u_D is the uncertainty of the log's reading as that quantity. The flights' table, above, says which logs name their altimeter and which are taken as barometric; Genesis's and Lince's logs are read here as a CATS Vega's, from their files ([the note][vega-height]).
- **Its components** are the pressure sensor's published error, its noise, the step the log keeps, the gap between the altimeter's own height formula and the 1976 standard's, and what the log's filter or this report's cut does where that is measured. Each is a standard uncertainty in meters, taken as independent of the others, and u_D is their root sum of squares.
- **Corrections.** Where something is known to make a log read low or high, the log's apogee is corrected by that effect's mean, and only its spread stays in u_D. The table gives each correction in brackets beside its component, their sum, and E against the corrected log: E less the correction. A correction is signed: it is what is added to the log's apogee. The report's own errors, above, stay on the logs as logged.
- **The rules** are the *Guide to the expression of uncertainty in measurement*'s (JCGM 100:2008). Bounds of ±a give a/√3 (4.3.7). A recognized effect of known sign is corrected, not left in the uncertainty (3.2.4): for a log's error equally likely anywhere from lo to hi, the correction is −(lo + hi)/2, and (hi − lo)/√12 stays (F.2.4.5, with 4.3.7). A value kept in steps of δ gives δ/√12 (F.2.2.1). A maker's "typical" figure has no rule in the guide; it is taken as one standard deviation, the larger of the two readings it allows.
- **Pressure to height.** A pascal is 1/(ρ·g₀) of height, the standard atmosphere's hydrostatic equation, with ρ its density there. The table gives it twice: first at the apogee (the site's elevation plus the log's apogee), then on the pad.
- **The zero on the pad.** A reading is the standard altitude of the pressure at the apogee less that of the pressure on the pad. An error the same in pascals at both would cancel if a pascal were the same height at both, but a pascal is more height in the thinner air at the apogee. So the sensor's error on the pad leaves its size times the difference between a pascal's height at the apogee and on the pad: the "pad offset" component. The "sensor" component is how far the error may have changed between the zero and the apogee.

Left out, for want of a published figure: the lag of a vented bay and its ports, a bay outside 0 to 50 °C, the sensor's drift with age and with its supply voltage, the pad sample's own noise and step, how far noise lifts the highest of many samples on the two logs with a noise component, and whether a PerfectFlite takes its pad as sea level ([the note][height-formulas]).

Which way the choices lean: a larger u_D makes a wider band later, which an error fits inside more easily. Three choices lean it large. A "typical" figure is taken as one standard deviation, the larger of the two readings it allows. The Raven's one figure is used both for its error on the pad and for that error's change since the zero: larger than using it once, though not the largest reading, since as independent bounds on the two pressures it would give more. Prometheus's noise is the sensor's at its noisiest setting, as the altimeter's isn't published. One leans small: the sensor sheet's band after an autozero (the sensor zeroed against one known pressure) is taken for the change since the altimeter's own zero on the pad, which the sheet doesn't describe; its band with no autozero is twice as wide ([ADR-375][adr-375] leaves this open). Against all of that, two altimeters flown on one rocket differ by 0.37% at the median over 59 published flights, about 0.39% a unit ([the note][two-altimeters]). The sentence under the table counts the flights whose u_D is a smaller share of the apogee than that.

| flight | altimeter | log apogee (m) | a pascal's height (m): at the apogee, on the pad | components, with corrections in brackets (m) | correction (m) | u_D (m) | u_D, share of the apogee | E corrected (m) | \|E corrected\| over u_D |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|
| ndrt-2020 | Featherweight Raven | 1320.4 | 0.0966, 0.0849 | sensor 5.58; pad offset 0.68; step 0.11; formula 0.04 | +0.00 | 5.62 | 0.43% | 137.1 | 24.4 |
| prometheus-2022 | Altus Metrum TeleMetrum (MS5607: the log steps its pressure by 1 Pa) | 3895.8 | 0.1430, 0.0954 | sensor 8.26; pad offset 5.49; noise 1.86; step 0.04; formula 0.00 (+0.19) | +0.19 | 10.09 | 0.26% | -359.4 | 35.6 |
| juno-iii | Missile Works RRC3 (MS5607) | 3151.5 | 0.1331, 0.0962 | sensor 7.68; pad offset 4.26; noise 0.48; step 0.38; formula 0.00 (+1.32); cut 2.89 (+15.00) | +16.32 | 9.27 | 0.29% | -191.1 | 20.6 |
| cavour | CATS Vega (MS5607), its radio telemetry | 2785.0 | 0.1114, 0.0845 | sensor 6.43; pad offset 3.11; step 0.29 (+0.50); filter 0.75; formula 0.01 (+0.21) | +0.71 | 7.19 | 0.26% | 159.8 | 22.2 |
| genesis | CATS Vega (MS5607), the 9-sample median of its barometric height | 2916.7 | 0.1130, 0.0845 | sensor 6.53; pad offset 3.29; peak 0.66 (-2.05); formula 0.00 (+0.19) | -1.86 | 7.34 | 0.25% | -169.6 | 23.1 |
| lince | CATS Vega (MS5607), the 9-sample median of its barometric height | 3587.7 | 0.1212, 0.0845 | sensor 7.00; pad offset 4.23; peak 0.66 (-2.05); formula 0.00 (+0.23) | -1.82 | 8.20 | 0.23% | 142.1 | 17.3 |
| macie-xl-1 | PerfectFlite StratoLoggerCF | 842.8 | none, none | precision 1.15; calibration 0.42; step 0.09 | +0.00 | 1.23 | 0.15% | -136.5 | 111.4 |
| macie-xl-2 | PerfectFlite StratoLoggerCF | 977.8 | none, none | precision 1.28; calibration 0.49; step 0.09 | +0.00 | 1.38 | 0.14% | -285.2 | 207.3 |

8 of the 12 flights have a u_D; 4 have none. The largest u_D is 10.09 m (prometheus-2022). The largest correction is +16.32 m (juno-iii). The corrections move E away from zero on prometheus-2022, juno-iii and lince and toward it on cavour and genesis. On 7 of the 8 flights with a u_D, it is under 0.39% of the log's apogee, the two-altimeter figure. Every flight with a u_D above zero has a corrected |E| at least 17 times its own u_D; the fewest times is on lince. So the log's part can't account for any of these errors either.

Where each component's figure is stated:

- **sensor** (`ndrt-2020`): Featherweight Altimeters, Raven4 user's manual (2020-08-15), p. 12: "less than 1 mbar (0.1% full-scale) pressure error over most conditions", with no confidence level; the log doesn't say which Raven wrote it.
- **pad offset** (`ndrt-2020`): the same sentence: the manual gives one figure and no part of it that a zero removes, so it is taken for the error on the pad as for its change from the pad to the apogee.
- **step** (`ndrt-2020`): the same manual, p. 12: baro resolution 0.00004 atm.
- **formula** (`ndrt-2020`): docs/research/altimeter-height-formulas.md: Raven height columns are the standard atmosphere's altitude of their own logged pressure to 0.22 ft, either way.
- **sensor** (`prometheus-2022`, `juno-iii`, `cavour`, `genesis`, `lince`): TE Connectivity, MS5607-02BA03 datasheet (06/2017), p. 4: total error band with autozero at one pressure point, 0 to 50 °C, 300 to 1100 mbar, −1.0 to +1.0 mbar (its minimum and maximum columns).
- **pad offset** (`prometheus-2022`, `juno-iii`, `cavour`, `genesis`, `lince`): the same sheet, p. 4: total error band with no autozero, 0 to 50 °C, 300 to 1100 mbar, −2.0 to +2.0 mbar, for the sensor's error on the pad, which the zero leaves in a difference of two standard altitudes.
- **noise** (`prometheus-2022`): the same sheet, p. 4: resolution RMS 0.130 mbar at an oversampling ratio of 256, the largest it lists; the altimeter's ratio isn't published.
- **step** (`prometheus-2022`): the same sheet, p. 8: the compensated pressure is given in steps of 0.01 mbar.
- **formula** (`prometheus-2022`, `juno-iii`): this report: the log's height column less the 1976 standard's reading of its own pressure column, at the log's highest row.
- **noise** (`juno-iii`): the same sheet, p. 4: resolution RMS 0.036 mbar at an oversampling ratio of 2048, the RRC3's (Missile Works, RRC3 user manual v1.71, p. 24).
- **step** (`juno-iii`): docs/research/altimeter-sensor-noise.md, §3: the public RRC3 log keeps its pressure in steps of 10 Pa.
- **cut** (`juno-iii`): this report's note on the flight: the log is read to 24.60 s, where its own velocity column still reads 17.5 m/s up, so its apogee may be 10 m to 20 m low.
- **step** (`cavour`): the telemetry sends the height in whole meters cut toward zero (docs/research/cats-vega-height.md), so it reads up to 1 m low.
- **filter** (`cavour`): docs/research/cats-vega-filtered-column.md, "For Cavour": the telemetry is the Kalman filter's height, within about 1.3 m of the flight's own barometric apogee.
- **formula** (`cavour`): docs/research/cats-vega-height.md, "Against HPR Sim's reading": the Vega's formula reads 0.19 m and 0.23 m low on the two logs it is measured on; Cavour's telemetry keeps no pressure to measure it on, so it is taken as anywhere between the two.
- **peak** (`genesis`, `lince`): docs/research/cats-vega-filtered-column.md, "Noise lifts the maxima": on four Vega logs the 9-sample median's highest value sits 0.9 m to 3.2 m above a parabola through the coast, as the highest of a noisy series does.
- **formula** (`genesis`, `lince`): docs/research/cats-vega-height.md, "Against HPR Sim's reading": near the peak the Vega's formula reads 0.19 m (Genesis) and 0.23 m (Lince) below the 1976 standard's pressure altitude less the pad's.
- **precision** (`macie-xl-1`, `macie-xl-2`): PerfectFlite, StratoLoggerCF user's manual (PDF of 2014-12-02), specifications, PDF p. 49: "Measurement precision: +/- (0.1% reading + 1 foot) typical".
- **calibration** (`macie-xl-1`, `macie-xl-2`): the same page: "Calibration accuracy: +/- 0.05% typical".
- **step** (`macie-xl-1`, `macie-xl-2`): the log's header gives its apogee in whole feet.

No published accuracy, so no u_D and no band:

- `bella-lui` (the team's own avionics): no document read names the sensor that made the logged height, or how it was filtered.
- `astra` (not named): RocketPy's notebook calls it a COTS altimeter and names no make or sensor.
- `andromeda` (not named): RocketPy's notebook calls it a COTS altimeter and names no make or sensor.
- `erebus-11` (not named): RocketPy's notebook calls it a COTS altimeter and names no make or sensor.

### The barometer's conversion, in both readings

The standard atmosphere isn't the day's air, so a barometric reading isn't the height climbed. That difference, the conversion, is not in u_D, because it is in both readings: hpr's apogee is read through the same standard atmosphere, from the pressure of the air it is flown in. The table measures it on each flight:

- **In hpr's reading:** hpr's apogee as the altimeter would read it, less its height.
- **In the log's:** the log's apogee, less the height at which the same altimeter, in the same air, would read it.
- **E as heights:** hpr's height less the log's, E with neither side read through the barometer. The conversion grows with the height, so two apogees E apart don't carry quite the same one, and E as heights differs from E by the difference. That difference is E itself counted in the day's air instead of the standard's: a share of E, not an error of the log or of hpr, and in meters it can be larger than u_D. As a share of the apogee, which is how this report scores a flight, the two are nearly the same number.

| flight | conversion in hpr's reading (m) | in the log's (m) | E (m) | E, share of the log's apogee | E as heights (m) | E as heights, share of the log's height |
|---|---:|---:|---:|---:|---:|---:|
| bella-lui | -0.70 | -0.69 | 5.1 | +1.11% | 5.1 | +1.11% |
| ndrt-2020 | 24.82 | 23.54 | 137.1 | +10.39% | 135.9 | +10.48% |
| prometheus-2022 | -259.78 | -282.36 | -359.2 | -9.22% | -381.8 | -9.14% |
| juno-iii | -254.70 | -266.21 | -174.8 | -5.55% | -186.3 | -5.45% |
| cavour | -106.08 | -101.69 | 160.5 | +5.76% | 164.9 | +5.71% |
| genesis | -129.27 | -136.42 | -171.5 | -5.88% | -178.7 | -5.85% |
| lince | -165.94 | -159.54 | 140.3 | +3.91% | 146.7 | +3.92% |
| astra | -118.24 | -127.62 | -212.4 | -6.54% | -221.7 | -6.57% |
| andromeda | -148.55 | -142.23 | 142.0 | +4.13% | 148.4 | +4.14% |
| erebus-11 | -96.37 | -105.51 | -242.1 | -8.07% | -251.3 | -8.09% |
| macie-xl-1 | -0.08 | -0.11 | -136.5 | -16.20% | -136.6 | -16.20% |
| macie-xl-2 | -0.08 | -0.15 | -285.2 | -29.16% | -285.2 | -29.17% |

The conversion is as large as 282.4 m in a reading (prometheus-2022). E as heights differs from E by at most 22.6 m (prometheus-2022), and as a share of the apogee by at most 0.10 points (juno-iii). In meters that difference is larger than u_D on 2 of the 8 flights with one.

What isn't in both readings is the air's own error. The log's altimeter read the day's real air; hpr's reads the air the flight is flown in, a reanalysis or the standard atmosphere. Where they differ, hpr's reading moves and the log's doesn't: by about 0.35% of the reading for each kelvin of the air column ([the note][height-formulas]). That is an input's uncertainty: the air is one of the five inputs of the next section.

## How much of each error the inputs could be

The third part, u_input, is how far a flight's apogee moves over what isn't known of its inputs ([M1.14b18][m1-14b18], decision [ADR-376][adr-376]). hpr flies each flight on its example's stated mass, thrust curve, rail and weather file. None of those is exact, and E carries their errors with the model's own.

- **Five inputs are moved:** the mass without motors, each motor's total impulse, the wind, the rail's angle and the temperature of the air. The drag is not one of them: it is the model this report tests. The impulse's factor scales a motor's thrust and its propellant's mass together, as more or less of the same propellant would.
- **Each has a standard uncertainty,** listed under the tables with its source. Most are figures for a kind of input, not for this flight's own: no team here published a measured uncertainty for its rocket, and the ones that hobby and student sources print are nearly all assumed ([the survey][sigmas]).
- **Alone.** Each input is moved one standard uncertainty down and one up, the rest held. Half the difference of the two apogees is that input's part: positive where more of the input flies higher. The wind and the rail have two parts each, one an axis. The root sum of squares of the seven is the local estimate of u_input, one of the two ways V&V 20 gives ([the note][vv20]).
- **Together.** Each flight is flown 200 times more with all seven numbers drawn at once from normal distributions: the standard's other way. The draws are 2 Latin hypercubes of 100 flights, which cut each input's distribution into 100 equal shares and fly every share once. u_input is the standard deviation of the 200 apogees. The table also gives each hypercube's own spread: how far the two stand apart is how well the sample knows its answer.
- **The air is drawn once.** A column ΔT warmer keeps its pressure on the pad and its layers grow thicker. The flight is flown in that air, and the altimeter that reads hpr's apogee reads the same air: the reading falls by about 0.35% a kelvin while the height climbed rises a little in the thinner air.
- **A check.** Flown with nothing moved, each flight gives the report's own apogee to the last bit, or the report is refused.

How far to trust it: u_input is as good as the five figures put in, and a larger u_input makes a wider band, which an error fits inside more easily.

- **Smaller readings taken.** The teams' masses are taken as weighed; a mass that is a design figure would spread like the design file's row, about twenty times wider. The impulse takes the floor of its range: 2.5% is the figure only if all of the scatter between two tests of a motor is the test stand's. The range runs to 3.56% if each test fired one motor, and to 5.03% if each figure is the mean of three. Its factor carries the propellant's mass with the thrust; more thrust from the same propellant would move the apogee more. ERA5's wind takes the daytime figure; this project's own comparison of ERA5 with flights' GPS descents stands further apart than that, the canopy's own motion included ([the report][descent-wind]). ERA5's air takes the lower end of 0.9 to 1.35 K. The band's section says what the impulse's two higher figures do to the count.
- **Larger readings taken.** Air and wind that aren't the day's get bounds wide enough for any day, on Prometheus too, whose weather file is of the same date a year later and is likely nearer its day than that. The rail takes 1° across its heading as well as along it, where the sources give 1° to 5° of heading, a smaller turn on a rail near the vertical. An error in the wind or the air is taken as the same at every height, which moves an apogee more than one that changes with height.
- **Borrowed.** Three flights' thrust curves are their teams' own or of no stated origin, and take the commercial motors' figure for want of one.
- **Left out,** each of which would add to u_input: the burn time at a fixed impulse, the center of mass, a thrust or fin misalignment, the rail's length and friction, the pad's pressure and the site's elevation. The inputs are drawn independently.
- **Not corrected.** Four known effects have a sign and are left in E. hpr flies ERA5's air dry, which reads 0.08% to 0.65% low ([the note][era5-air], §4). Andromeda's and Juno III's thrust files stand off their motors' catalog or recorded impulse, as the sections above say. A rocket as built is heavier than its design file, by a median 7.4% of the dry mass ([the note][as-built]), so a flight on a design file's mass is flown light and reads high from that cause. The 10.75% such a mass takes here was measured about that median, and the draws are centered on the file's value: the median is left in E. The band's section says which call rests on it.

Each number alone, half the difference of its two apogees (m), and the root sum of squares of the seven:

| flight | mass | impulse | wind east | wind north | rail up | rail across | air | root sum of squares |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bella-lui | -4.4 | +22.8 | -0.9 | -1.8 | -0.4 | -0.2 | -1.3 | 23.4 |
| ndrt-2020 | -7.6 | +52.0 | -7.9 | -11.9 | +5.9 | -3.8 | -3.3 | 55.0 |
| prometheus-2022 | -10.7 | +98.8 | +82.0 | +23.3 | +29.6 | +0.5 | -67.9 | 150.4 |
| juno-iii | -19.7 | +114.5 | -11.6 | -25.0 | -3.3 | -21.1 | -7.7 | 121.5 |
| cavour | -7.3 | +71.2 | +10.2 | -18.3 | +15.0 | -4.5 | -5.5 | 76.4 |
| genesis | -8.9 | +73.6 | +9.0 | -14.9 | +13.6 | -3.5 | -5.1 | 77.7 |
| lince | -8.9 | +85.5 | +20.2 | -23.8 | +22.2 | -2.4 | -6.2 | 94.4 |
| astra | -9.7 | +80.0 | +7.1 | -5.4 | +7.5 | +0.6 | -5.6 | 81.6 |
| andromeda | -7.3 | +77.0 | +13.6 | -12.7 | +12.4 | -0.0 | -5.6 | 80.7 |
| erebus-11 | -6.2 | +63.0 | +3.2 | -8.9 | +7.7 | -3.1 | -4.5 | 64.7 |
| macie-xl-1 | -91.0 | +26.9 | +7.8 | -0.0 | +1.3 | +0.0 | -16.1 | 96.6 |
| macie-xl-2 | -89.2 | +26.6 | +9.0 | -0.0 | +1.4 | +0.0 | -15.9 | 94.9 |

All seven drawn together, 200 flights:

| flight | each hypercube's spread (m) | u_input (m) | u_input, share of the apogee | mean of the 200 less the report's apogee (m) |
|---|---:|---:|---:|---:|
| bella-lui | 22.9, 24.7 | 23.8 | 5.12% | -5.2 |
| ndrt-2020 | 53.4, 55.5 | 54.3 | 3.73% | -5.6 |
| prometheus-2022 | 139.2, 141.6 | 140.0 | 3.96% | -26.3 |
| juno-iii | 120.7, 123.7 | 121.9 | 4.10% | -9.4 |
| cavour | 72.9, 72.8 | 72.7 | 2.47% | -6.7 |
| genesis | 74.1, 74.6 | 74.1 | 2.70% | -5.8 |
| lince | 89.3, 88.7 | 88.8 | 2.38% | -10.0 |
| astra | 78.4, 79.7 | 78.9 | 2.60% | -8.4 |
| andromeda | 77.1, 77.1 | 76.9 | 2.15% | -11.7 |
| erebus-11 | 62.6, 63.2 | 62.7 | 2.27% | -6.3 |
| macie-xl-1 | 92.9, 98.2 | 95.3 | 13.50% | -12.2 |
| macie-xl-2 | 91.6, 96.5 | 93.9 | 13.55% | -14.5 |

The largest u_input is 140.0 m (prometheus-2022); as a share of the apogee the largest is 13.55% (macie-xl-2). On 10 of the 12 flights the impulse's part is the largest of the seven. The seven parts' root sum of squares stands at most 7.5% from u_input (prometheus-2022), and a hypercube's own spread at most 3.9% from it (bella-lui). The samples' mean lies below the report's apogee on 12 of the 12 flights, by at most 26.4 m (prometheus-2022). There the seven inputs' own bends, each the mean of an input's two flights alone less the report's apogee, sum to -33.2 m. The band is about E on the report's own flight, not about the mean of the samples.

Where each input's standard uncertainty is stated:

- **mass, 0.50%** (`bella-lui`, `ndrt-2020`, `prometheus-2022`, `juno-iii`, `cavour`, `genesis`, `lince`, `astra`, `andromeda`, `erebus-11`): the team's stated mass, taken as weighed. Source: [docs/research/dispersion-sigmas.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dispersion-sigmas.md), §3: 0.5% about a weighed mass is the note's own inference, and one student team's review disperses its rocket's mass by that; hobby sources assume 0% to 3.9% about a file's value, and no notebook here says its mass was weighed.
- **mass, 10.75%** (`macie-xl-1`, `macie-xl-2`): a design file's mass, with no record that the rocket was weighed as flown. Source: [docs/research/as-built-mass-growth.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/as-built-mass-growth.md): over 24 rockets with both a design-file mass and a scale mass, the scale's stands −1.4% to +13.1% from the file's at the quartiles, a standard deviation of 10.75% for a normal spread (the range over 1.349).
- **impulse, 2.50%** (`bella-lui`, `ndrt-2020`, `prometheus-2022`, `cavour`, `genesis`, `lince`, `astra`, `macie-xl-1`, `macie-xl-2`): a commercial motor on its maker's or certifier's curve. Source: [docs/research/hpr-lot-impulse.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/hpr-lot-impulse.md), §2: 21 AeroTech H to L motors that Tripoli Motor Testing tested a second time in 2001 to 2004 differ by a standard deviation of 3.56 points between their two tests (4.41 with a renamed 22nd motor). Where each test fired one motor, a flown motor against its file's curve is such a difference less one test's measuring error: from 3.56/√2 = 2.5%, if all of the scatter is the measuring, to 3.56%, if all of it is from motor to motor. The tests' records don't print how many they fired: where each figure is the mean of three, the least Tripoli Motor Testing's rules ask of such a retest today (§1), the top is 3.56·√2 = 5.03%. The floor, the same 2.5% for any number fired, is taken. The note calls the pairs no clean comparison of lots: some of the motors changed between their tests.
- **impulse, 2.50%** (`juno-iii`, `andromeda`, `erebus-11`): a curve of the team's own or of no stated origin: no spread was found for one, so the commercial motors' is borrowed. Source: [docs/research/hpr-lot-impulse.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/hpr-lot-impulse.md), §2, as for a commercial motor.
- **wind, 2.00 m/s on each axis** (`bella-lui`, `ndrt-2020`, `juno-iii`, `cavour`, `genesis`, `lince`, `astra`, `andromeda`, `erebus-11`): ERA5's wind at the flight's hour. Source: [docs/research/era5-wind-direction-us.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/era5-wind-direction-us.md), §6: about 2 m/s on each axis by day (1.8 to 2.4), inferred from forecast models held to soundings and lidars; above 3.2 m/s on the worst nights.
- **wind, 4.47 m/s on each axis** (`prometheus-2022`, `macie-xl-1`, `macie-xl-2`): not the day's wind. Source: the high-power safety code's limit of 20 mph (8.94 m/s) for a launch (NFPA 1127, as [docs/VALIDATION.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/VALIDATION.md) cites it), read here as a bound on a wind from any direction: spread evenly over that disc, 8.94/2 on each axis.
- **rail, 1.00° on each axis** (`bella-lui`, `ndrt-2020`, `prometheus-2022`, `juno-iii`, `cavour`, `genesis`, `lince`, `astra`, `andromeda`, `erebus-11`, `macie-xl-1`, `macie-xl-2`): the rail's stated angle. Source: [docs/research/dispersion-sigmas.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dispersion-sigmas.md), §5: a tilt of 1° in five of the seven hobby sources that disperse a launch angle, RocketPy's notebooks among them; each assumes it and none measured it. The same 1° is taken on the axis across the heading, where the sources give 1° to 5° of heading, a smaller turn on a rail near the vertical: the larger reading.
- **air, 0.90 K** (`bella-lui`, `ndrt-2020`, `juno-iii`, `cavour`, `genesis`, `lince`, `astra`, `andromeda`, `erebus-11`): ERA5's air at the flight's hour. Source: [docs/research/era5-surface-temperature.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/era5-surface-temperature.md), §1: ERA5's mean temperature from the pad to 1 to 3 km stands 0.9 to 1.35 K from soundings it never took in (Huang et al. 2021, at Tazhong, a desert site; this project's average of their levels), the lower end taken; at stations it did take in, a level stands 0.72 K off at 700 hPa and 0.85 K at 850 hPa, and 1.85 K at 10 m (Taszarek et al. 2021, p. 3218).
- **air, 8.66 K** (`prometheus-2022`, `macie-xl-1`, `macie-xl-2`): not the day's air. Source: [docs/physics/synthetic-flights.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/physics/synthetic-flights.md), "What a flight draws": a day's air taken as within 15 K of the 1976 standard atmosphere's, this project's own range with no outside source; as bounds, 15/√3 = 8.66 K.

## The band: each error against its validation uncertainty

The validation uncertainty u_val is the root sum of squares of the three parts, √(u_num² + u_input² + u_D²), and the band is 2 times it either side of zero ([M1.14b18][m1-14b18], decision [ADR-376][adr-376]). The error it is held against is E against the corrected log, from the section on the logs.

- **Inside the band,** the integrator, the inputs and the log could between them account for the whole error. The model's own error is then known only to be smaller than |E| + 2·u_val, and its sign isn't known. That is not a pass: it says this comparison can't see the model's error through its inputs.
- **Outside it,** the three parts can't account for the error. What is left has the sign of E and a size between |E| − 2·u_val and |E| + 2·u_val. It is the model's error together with whatever the section above leaves out of u_input or leaves uncorrected in E, and it is only as firm as the inputs' figures: the sentence under the table says which calls change at the impulse's two higher figures.
- **Meeting the band is not the aim.** A larger uncertainty always meets it, so a count of flights inside rewards knowing less. What the band is for is the size and sign of the model's error where it can be told (Eça, Dowding and Roache 2020, p. 5; [the note][vv20]).
- **The factor 2** takes the three parts' errors as roughly normal, for a band of about 95%.
- **A flight with no u_D has no u_val** and no band: its log's part isn't known, and without it the root sum of squares would be too small.

| flight | E corrected (m) | u_num (m) | u_input (m) | u_D (m) | u_val (m) | 2·u_val (m) | \|E corrected\| over u_val | within 2·u_val |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| bella-lui | none | 0.0019 | 23.8 | none | none | none | none | no band |
| ndrt-2020 | +137.1 | 0.0014 | 54.3 | 5.62 | 54.6 | 109.2 | 2.51 | no |
| prometheus-2022 | -359.4 | 0.0006 | 140.0 | 10.09 | 140.4 | 280.8 | 2.56 | no |
| juno-iii | -191.1 | 0.0000 | 121.9 | 9.27 | 122.3 | 244.5 | 1.56 | yes |
| cavour | +159.8 | 0.0001 | 72.7 | 7.19 | 73.0 | 146.0 | 2.19 | no |
| genesis | -169.6 | 0.0007 | 74.1 | 7.34 | 74.5 | 149.0 | 2.28 | no |
| lince | +142.1 | 0.0005 | 88.8 | 8.20 | 89.2 | 178.3 | 1.59 | yes |
| astra | none | 0.0013 | 78.9 | none | none | none | none | no band |
| andromeda | none | 0.0057 | 76.9 | none | none | none | none | no band |
| erebus-11 | none | 0.0024 | 62.7 | none | none | none | none | no band |
| macie-xl-1 | -136.5 | 0.0005 | 95.3 | 1.23 | 95.3 | 190.7 | 1.43 | yes |
| macie-xl-2 | -285.2 | 0.0006 | 93.9 | 1.38 | 93.9 | 187.8 | 3.04 | no |

Of the 8 flights with a band, the corrected |E| is within 2·u_val on 3 (juno-iii, lince, macie-xl-1) and outside it on 5: hpr high on 2 (ndrt-2020, cavour), hpr low on 3 (prometheus-2022, genesis, macie-xl-2). No u_D and so no band on 4 of the 12 flights. The count moves with the inputs' figures. With the impulse's standard uncertainty at 3.56%, the top of its range if each certification test fired one motor, the seven parts put 6 of the 8 within (ndrt-2020, juno-iii, cavour, genesis, lince, macie-xl-1). At 5.03%, the top if each test's figure is the mean of three motors, they put 7 of the 8 within (ndrt-2020, prometheus-2022, juno-iii, cavour, genesis, lince, macie-xl-1). Outside at all three figures: 1 (macie-xl-2). The other way: with a rocket's median growth as built, 7.4% of the mass without motors, taken out of each flight on a design file's mass, by the mass's flight alone, the errors would be -195.9 m (macie-xl-1) and -343.3 m (macie-xl-2), and at the figures used 2 of the 8 would lie within (juno-iii, lince).

[m1-14b16]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b16
[m1-14b17]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b17
[m1-14b18]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b18
[adr-373]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0373-m1-14b7-in-three-the-numerical-part-first.md
[adr-375]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0375-m1-14b17-the-logs-part-of-a-real-flights-validation-uncertainty.md
[adr-376]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0376-m1-14b18-the-inputs-part-and-the-band.md
[vv20]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/validation-uncertainty-vv20.md
[descent-wind]: descent-wind.md
[sigmas]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dispersion-sigmas.md
[era5-air]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/era5-surface-temperature.md
[as-built]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/as-built-mass-growth.md
[two-altimeters]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dual-altimeter-spread.md
[height-formulas]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-height-formulas.md
[vega-height]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/cats-vega-height.md

## Sources

The ERA5 weather here is read from the files RocketPy's examples read: generated using Copernicus Climate Change Service information 2026, and contains modified Copernicus Climate Change Service information 2026. Neither the European Commission nor ECMWF is responsible for any use that may be made of the Copernicus information or data it contains. ERA5: Hersbach, H. et al. (2023), *ERA5 hourly data on pressure levels from 1940 to present*, Copernicus Climate Change Service (C3S) Climate Data Store (CDS), <https://doi.org/10.24381/cds.bd0915c6>, licensed CC BY 4.0 (<https://creativecommons.org/licenses/by/4.0/>). EuRoC 2022's file is read as its classic-format (netCDF-3) conversion, which holds the geopotential, temperature and wind of RocketPy's file bit for bit (`validation/oracles/netcdf/euroc_2022.py`).
