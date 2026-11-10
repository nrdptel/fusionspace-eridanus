# Public flights predicted before they are compared

Written by `cargo xtask public-predictions` ([M2.3d3][m2-3d3], decisions [ADR-328][adr-328] and [ADR-334][adr-334]). What it is for is on the [documentation site][site].

- **What this is:** 3 public real flights that join the seven of the [real-flights report][real], each flown by HPR Sim from its record alone. This page was merged before any of their logs was compared, so no number here could be tuned to its own error.
- **What it holds:** each flight's inputs with their sources, the rule its log's apogee will be read by, and HPR Sim's predicted apogee. It holds no measured apogee and no error: those come in a later pull request, which must print each prediction below beside its error.
- **How it is flown:** as the seven are. The design is built from the example's inputs and flown on the example's own thrust file, from its rail and site, in the ERA5 air of its launch hour, on HPR Sim's own aerodynamics.
- **The predicted apogee** is read as the log's altimeter would read it: the standard atmosphere's altitude of the day's pressure at the apogee, less the pad's. The height climbed is beside it.
- **Traced:** An input is traced when the flight's own record states it: here RocketPy's example notebook, which holds the team's inputs. What no RocketPy example records, the surface finish and the fin edges, is a placeholder in every design built from one and doesn't count against the tag. A flight is tagged not traced when its record leaves out, or contradicts, something that moves the apogee.
- **Files:** the thrust files, logs and weather file are read from the pinned RocketPy checkout and never committed; each one's SHA-256 is in the JSON. A log's bytes are hashed to pin the file its rule will be applied to; they are never read as numbers here.

[m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3
[adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[site]: https://hpr.fusionspace.co/accuracy.html#predicted-before-compared
[real]: real-flights.md

## Predictions

| flight | impulse flown (N s) | liftoff mass (kg) | predicted apogee (m) | as a height (m) | time to apogee (s) | largest Mach | every input traced |
|---|---:|---:|---:|---:|---:|---:|---|
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) | 4280.2 | 12.670 | 3037.1 | 3155.4 | 24.40 | 0.911 | yes |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | 9866.7 | 25.323 | 3581.0 | 3729.5 | 26.30 | 1.019 | no |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) | 2418.2 | 6.996 | 2714.1 | 2808.7 | 22.45 | 0.876 | yes |

## On the catalog curve, never counted

The same design on the bundled certified curve of the motor its record names, where the counted flight is on another curve. It is here so the reader can see what the choice of curve moves; it enters no mean.

| flight | catalog motor | impulse flown (N s) | predicted apogee (m) | as a height (m) |
|---|---|---:|---:|---:|
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | Cesaroni 8429M2020-P | 8428.7 | 3050.3 | 3176.1 |

- **Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve):** the reload the notebook names, on its certified curve as bundled, whole. Shown beside the counted prediction and never counted: every RocketPy flight here is counted on its example's own thrust file.

## Each flight's inputs and its apogee rule

- **Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350)** (`astra`, design `rocketpy-astra`):
  - Design, site and rail: RocketPy 1.13.0's astra_flight_sim.ipynb:123-131 (site, date), :247-280 (rocket, rail buttons, motor position, nose, fins), :316-318 (rail). The site is 39.3897°, -8.2889639° at 160 m; the rail is 12 m long, 84° from the horizontal, heading 133°.
  - Motor: Cesaroni 4263L1350-P (astra_flight_sim.ipynb:155-157: RocketPy's copy of the certified curve, cut at the notebook's burn time). Thrust file `refs/rocketpy/data/motors/cesaroni/Cesaroni_4263L1350-P.eng`, burn time 3.284 s.
  - Mass: 10.6462 kg without the motor, 12.670 kg at liftoff (astra_flight_sim.ipynb:249-251 (mass, inertia and center without the motor), :158-162 (the grain, whose density and size give the propellant), :168-170 (no motor dry mass or inertia)).
  - Weather: ERA5 at 2022-10-13T15:00Z, from the converted file below.
  - Apogee rule: the highest reading of the log's `alt` column, in meters, the log read whole, in `refs/rocketpy/data/rockets/astra/flight_data.csv`. Altimeter: barometric, assumed (the notebook calls it a COTS altimeter's flight (astra_flight_sim.ipynb:383-385), one row a second beside a latitude and a longitude, and records no more: barometric assumed, as hobby altimeters are).
  - Every input traced: yes.
- **Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve)** (`andromeda`, design `rocketpy-andromeda`):
  - Design, site and rail: RocketPy 1.13.0's andromeda_flight_sim.ipynb:123-131 (site, date), :244-277 (rocket, rail buttons, motor position, nose, fins), :388-390 (rail). The site is 39.388692°, -8.287814° at 160 m; the rail is 12 m long, 84° from the horizontal, heading 133°.
  - Motor: the team's thrust file, flown in the notebook under the name of a Cesaroni M2020 (andromeda_flight_sim.ipynb:153 and :162-165: "obtained by the team, after testing", cut at the notebook's burn time; the catalog file is commented out at :163). Thrust file `refs/rocketpy/data/rockets/andromeda/thrust_curve.csv`, burn time 3.76 s.
  - Mass: 20.9767 kg without the motor, 25.323 kg at liftoff (andromeda_flight_sim.ipynb:246-248 (mass, inertia and center without the motor), :166-170 (the grain), :176-178 (no motor dry mass or inertia)).
  - Weather: ERA5 at 2022-10-14T13:00Z, from the converted file below.
  - Apogee rule: the highest reading of the log's `alt(m)` column, the log read whole, in `refs/rocketpy/data/rockets/andromeda/flight_data.csv`. Altimeter: barometric, assumed (the notebook calls it a COTS altimeter (andromeda_flight_sim.ipynb:459-461), logged in feet with a vertical speed, and records no more: barometric assumed, as hobby altimeters are).
  - Every input traced: no. Not traced: the flight carried the team's air brakes, and no record says when or how far they opened: none is flown here, so any opening leaves this prediction high; the thrust file holds more impulse than the Cesaroni M2020 reload it is named for is certified at, so the notebook does not settle which motor the logged flight burned. Note: both tags are the research note's findings (docs/research/andromeda-impulse.md), written down before this flight was flown.
- **Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K)** (`erebus-11`, design `rocketpy-erebus-11`):
  - Design, site and rail: RocketPy 1.13.0's erebus_flight_sim.ipynb:123-131 (site, date), :251-284 (rocket, rail buttons, motor position, nose, fins, tail), :320-322 (rail). The site is 39.3897°, -8.2889639° at 160 m; the rail is 12 m long, 84° from the horizontal, heading 133°.
  - Motor: a Cesaroni Pro54 K-class motor, as the notebook says; the team's thrust file (erebus_flight_sim.ipynb:153 and :162-165, cut at the notebook's burn time). Thrust file `refs/rocketpy/data/rockets/erebus11/thrust_source.csv`, burn time 3.657 s.
  - Mass: 5.82 kg without the motor, 6.996 kg at liftoff (erebus_flight_sim.ipynb:253-255 (mass, inertia and center without the motor), :166-172 (the grain), :163 and :173-174 (no motor dry mass or inertia)).
  - Weather: ERA5 at 2022-10-15T15:00Z, from the converted file below.
  - Apogee rule: the highest reading of the log's `alt` column, in meters, the log read whole, in `refs/rocketpy/data/rockets/erebus11/flight_data_filtered.csv`. Altimeter: barometric, assumed (the notebook calls it a COTS altimeter (erebus_flight_sim.ipynb:390-392), a file named filtered, and records no more: barometric assumed, as hobby altimeters are).
  - Every input traced: yes.

## The weather file

The three flights' ERA5 file is netCDF-4, which HPR Sim does not read. `validation/oracles/netcdf/euroc_2022.py` converts it as the [guide][era5] says, `xarray.open_dataset(source).drop_vars(["number", "expver"], errors="ignore").to_netcdf(out, format="NETCDF3_64BIT")`, and checks the result: the four fields hold the source's values bit for bit, and RocketPy reads the same levels from both files at each flight's site and hour. Its record is `validation/fixtures/weather/euroc-2022-conversion.json`.

| file | SHA-256 |
|---|---|
| `refs/rocketpy/data/weather/euroc_2022_all_windows.nc` (netCDF-4, as RocketPy ships it) | `b4583116c89c9e2b8fb9a88ef61d6f7c2c25329559184f7bde7040694ac0d52d` |
| `refs/converted/rocketpy/euroc_2022_all_windows.nc` (converted, flown) | `3a3b43a749938e07cee8ad1d705dc89dc8a0c1fcac135c76520e17ad8860b003` |

[era5]: https://hpr.fusionspace.co/format/era5.html#converting-a-current-file
