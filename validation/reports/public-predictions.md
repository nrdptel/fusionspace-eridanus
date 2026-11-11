# Public flights predicted before they are compared

Written by `cargo xtask public-predictions` ([the EuRoC 2022 predictions, M2.3d3][m2-3d3], and [the Macie XL predictions, M2.3d5][m2-3d5]; decisions: [twelve public flights, predictions first, ADR-328][adr-328], [the predictions in two parts, ADR-334][adr-334] and [the Macie XL predictions, ADR-337][adr-337]). What it is for is on the [documentation site][site].

- **What this is:** 5 public real flights that join the seven of the [real-flights report][real], each flown by HPR Sim from its record alone. Each flight's prediction was merged before its log was compared: the three EuRoC 2022 flights' first, then the two Macie XL flights'. Their measured apogees were already public, so the targets were known; what merging first does is make an input changed after seeing HPR Sim's own miss show as a change to a merged number. The code holds each prediction to the number first merged.
- **What it holds:** each flight's inputs with their sources, the rule its log's apogee will be read by, and HPR Sim's predicted apogee. It holds no measured apogee and no error: those come in a later pull request, which must print each prediction below beside its error.
- **How it is flown:** the three EuRoC 2022 flights by the seven's method. The design is built from the example's inputs and flown on the example's own thrust file, from its rail and site, in the ERA5 air of its launch hour, on HPR Sim's own aerodynamics. The two Macie XL flights have no example: their design is an OpenRocket file in a pinned public repository, read whole as OpenRocket 24.12 saved it. Each is flown as the stored simulation the repository names for it: on its motor, from its rod, in its steady wind and standard atmosphere, on HPR Sim's own aerodynamics.
- **The predicted apogee** is read as the log's altimeter would read it: the standard atmosphere's altitude of the flown air's pressure at the apogee, less the pad's. The height climbed is beside it.
- **Traced:** An input is traced when the flight's own record states it: here RocketPy's example notebook, which holds the team's inputs. What no RocketPy example records is a placeholder in the designs built from them, the seven compared flights' too, and doesn't count against the tag: the surface finish, the rail buttons' size, the fins' thickness where no airfoil file gives it, and their edge shape where no airfoil file or published section does. They move the drag, so a traced flight's inputs are stated, not exact. A flight is tagged not traced when its record leaves out, or contradicts, something else that moves the apogee. For a flight read from a design file, the record is that file and its repository's page about the flight. The file states the rocket and names the motor. A stored simulation's launch conditions are what a simulation was run with, not a record of the day, so the site, the air, the wind and the rod count as not traced unless the page states them. So does the mass, unless the page says the rocket was weighed as flown, and the motor's curve where the reader warns that its file and its certifier's record disagree.
- **Files:** the examples' thrust files and the logs are read from the pinned RocketPy checkout (`refs/rocketpy`, which `cargo xtask refs fetch rocketpy` fetches), and the weather file is converted from the checkout's. The Macie XL design file and logs are read from the pinned `rocketry-files` checkout (`refs/public-flights/rocketry-files`, which `cargo xtask refs fetch rocketry-files` fetches). None is committed, and each one's SHA-256 is in the JSON. A log's bytes are hashed to pin the file its rule will be applied to; they are never read as numbers here.

[m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3
[adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[m2-3d5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d5
[adr-337]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md
[site]: https://hpr.fusionspace.co/accuracy.html#predicted-before-compared
[real]: real-flights.md

## Predictions

| flight | impulse flown (N s) | liftoff mass (kg) | predicted apogee (m) | as a height (m) | time to apogee (s) | largest Mach | every input traced |
|---|---:|---:|---:|---:|---:|---:|---|
| Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350) | 4280.2 | 12.670 | 3037.1 | 3155.4 | 24.40 | 0.911 | yes |
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve) | 9866.7 | 25.323 | 3581.0 | 3729.5 | 26.30 | 1.019 | no |
| Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K) | 2418.2 | 6.996 | 2714.1 | 2808.7 | 22.45 | 0.876 | yes |
| Macie XL, flight 1 (AeroTech I284W) | 599.5 | 4.278 | 706.2 | 706.3 | 12.20 | 0.354 | no |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | 600.8 | 4.338 | 692.6 | 692.7 | 12.12 | 0.362 | no |

## On the catalog curve, never counted

The same design, at the same mass, on the bundled certified curve of the motor its record names, where the counted flight is on another curve. It is here so the reader can see what the choice of curve moves; it enters no mean.

| flight | catalog motor | impulse flown (N s) | liftoff mass (kg) | predicted apogee (m) | as a height (m) |
|---|---|---:|---:|---:|---:|
| Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 | Cesaroni 8429M2020-P | 8428.7 | 25.323 | 3050.3 | 3176.1 |

- **Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022:** the reload the notebook names, on its certified curve as bundled, whole. Shown beside the counted prediction and never counted: every RocketPy flight here is counted on its example's own thrust file.

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
  - Apogee rule: the highest reading of the log's `alt(m)` column, the log read whole, in `refs/rocketpy/data/rockets/andromeda/flight_data.csv`. Altimeter: barometric, assumed (the notebook calls it a COTS altimeter (andromeda_flight_sim.ipynb:459-461), logged in feet and in meters with a vertical speed, and records no more: barometric assumed, as hobby altimeters are).
  - Every input traced: no. Not traced: the flight carried the team's air brakes, and no record says when or how far they opened: none is flown here, so any opening leaves this prediction high; the thrust file holds more impulse than the Cesaroni M2020 reload it is named for is certified at, so the notebook does not settle which motor the logged flight burned. Note: both tags are the research note's findings (docs/research/andromeda-impulse.md), written down before this flight was flown.
- **Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K)** (`erebus-11`, design `rocketpy-erebus-11`):
  - Design, site and rail: RocketPy 1.13.0's erebus_flight_sim.ipynb:123-131 (site, date), :251-284 (rocket, rail buttons, motor position, nose, fins, tail), :320-322 (rail). The site is 39.3897°, -8.2889639° at 160 m; the rail is 12 m long, 84° from the horizontal, heading 133°.
  - Motor: a Cesaroni Pro54 K-class motor, as the notebook says; the example's thrust file (erebus_flight_sim.ipynb:153 and :162-165, cut at the notebook's burn time). Thrust file `refs/rocketpy/data/rockets/erebus11/thrust_source.csv`, burn time 3.657 s.
  - Mass: 5.82 kg without the motor, 6.996 kg at liftoff (erebus_flight_sim.ipynb:253-255 (mass, inertia and center without the motor), :166-172 (the grain), :163 and :173-174 (no motor dry mass or inertia)).
  - Weather: ERA5 at 2022-10-15T15:00Z, from the converted file below.
  - Apogee rule: the highest reading of the log's `alt` column, in meters, the log read whole, in `refs/rocketpy/data/rockets/erebus11/flight_data_filtered.csv`. Altimeter: barometric, assumed (the notebook calls it a COTS altimeter (erebus_flight_sim.ipynb:390-392), a file named filtered, and records no more: barometric assumed, as hobby altimeters are).
  - Every input traced: yes.
- **Macie XL, flight 1 (AeroTech I284W)** (`macie-xl-1`):
  - Design: `refs/public-flights/rocketry-files/macie/macie-xl.ork`, SHA-256 `824e4a7b10f64de65aa65bdeaa65262e79879518a7f14eb5f70a6a1bdbf05bdb`, at commit `2ddcaf8858a71f9b4d910877c603ed95fe0a9769` of `https://github.com/rlhatcher/rocketry-files.git` (MIT). From macie/index.md (the two flights; the file names that pair `maciexl-1-or` with flight 1) and macie/macie-xl.ork, read whole as OpenRocket 24.12 saved it.
  - Flown as: the stored simulation `maciexl-1-or`, motor configuration `259029e7-db7c-4917-a55b-4a43cdebef6f` (the stored simulation's launch conditions in macie/macie-xl.ork). The site is 28.61°, -80.6° at 0 m; the rod is 1 m long, 90.0° from the horizontal, heading 90.0°; the wind is a steady 2 m/s from 90.0°; the air is the 1976 US Standard Atmosphere, for the stored simulation's `isa`.
  - Motor: AeroTech I284W (the stored simulation's motor configuration in macie/macie-xl.ork, which names the motor and the digest of its curve; the bundled ThrustCurve.org file that digest names is flown). Curve file `5f4294d20002e9000000003b`, for the digest `d9da87018adebbef9d8f3d8bfbc9c1e6`.
  - Mass: 3.722 kg without the motor, 4.278 kg at liftoff (macie/macie-xl.ork: its parts and the stage's mass and center overrides, which add to the parts as OpenRocket 24.12 weighs them; the motor's masses are the bundled catalog's for the motor, 313.6 g of propellant and 555.52 g loaded).
  - Apogee rule: the apogee the altimeter worked out, in the log's header (its `Apogee:` line, in feet above the pad, 0.3048 m to the foot), which leaves out the ejection spike among the samples; never the highest sample, in `refs/public-flights/rocketry-files/macie/flights/MRC Summer 26/maciexl-1-sl.pf2`. Altimeter: barometric (the log's first line names a PerfectFlite altimeter, `PerfectFlite SLCF`, and a PerfectFlite has a barometer and no accelerometer (docs/format/pf2.md)).
  - Every input traced: no. Not traced: no record gives the day, its air and wind, or the rail, and of the site only each log's header gives the pad's height above sea level (12 ft for flight 1, 37 ft for flight 2): the stored simulation's are flown, its sea level among them, and all nine simulations in the file carry the same ones; no record says the rocket was weighed as flown: the file's parts and overrides are flown; the reader warns that the motor's curve file disagrees with ThrustCurve.org's record of the motor (the figures are in the flight's `reader_warnings` in the JSON); the file the design's digest names is flown; the motor flies the bundled catalog's masses for it, 313.6 g of propellant and 555.52 g loaded, as every catalog motor does, where the curve file the design's digest names, and OpenRocket's own record of that digest, give 310.016 g and 555.072 g (the table's `mass_difference`). Note: the stored simulation's wind carries a turbulence of a tenth of its speed, which is not flown: the wind here is steady. The parachutes are not flown either; the file opens one at apogee and the other lower, on the way down, so neither moves it.
  - OpenRocket's own run of this simulation, stored in the design file, reached 704.6 m. That is another simulator's number, not a measurement; it is shown and never counted.
  - The reader's warnings on this file and motor are in the JSON under this flight's `reader_warnings`: 1 in all.
- **Macie XL, flight 2 (Cesaroni 601I350-16A)** (`macie-xl-2`):
  - Design: `refs/public-flights/rocketry-files/macie/macie-xl.ork`, SHA-256 `824e4a7b10f64de65aa65bdeaa65262e79879518a7f14eb5f70a6a1bdbf05bdb`, at commit `2ddcaf8858a71f9b4d910877c603ed95fe0a9769` of `https://github.com/rlhatcher/rocketry-files.git` (MIT). From macie/index.md (the two flights; the file names that pair `maciexl-2-or` with flight 2) and macie/macie-xl.ork, read whole as OpenRocket 24.12 saved it.
  - Flown as: the stored simulation `maciexl-2-or`, motor configuration `c16d442d-6695-4f95-b4f3-79dfcdc75450` (the stored simulation's launch conditions in macie/macie-xl.ork). The site is 28.61°, -80.6° at 0 m; the rod is 1 m long, 90.0° from the horizontal, heading 90.0°; the wind is a steady 2 m/s from 90.0°; the air is the 1976 US Standard Atmosphere, for the stored simulation's `isa`.
  - Motor: Cesaroni 601I350-16A (the stored simulation's motor configuration in macie/macie-xl.ork, which names the motor and the digest of its curve; the bundled ThrustCurve.org file that digest names is flown). Curve file `5f4294d20002e90000000575`, for the digest `84414906dddfb169c96ef1f7cf8d0ff7`.
  - Mass: 3.722 kg without the motor, 4.338 kg at liftoff (macie/macie-xl.ork: its parts and the stage's mass and center overrides, which add to the parts as OpenRocket 24.12 weighs them; the motor's masses are the bundled catalog's for the motor, which are OpenRocket's for the design's digest).
  - Apogee rule: the apogee the altimeter worked out, in the log's header (its `Apogee:` line, in feet above the pad, 0.3048 m to the foot); never the highest sample, in `refs/public-flights/rocketry-files/macie/flights/MRC Summer 26/maciexl-2-sl.pf2`. Altimeter: barometric (the log's first line names a PerfectFlite altimeter, `PerfectFlite SLCF`, and a PerfectFlite has a barometer and no accelerometer (docs/format/pf2.md)).
  - Every input traced: no. Not traced: no record gives the day, its air and wind, or the rail, and of the site only each log's header gives the pad's height above sea level (12 ft for flight 1, 37 ft for flight 2): the stored simulation's are flown, its sea level among them, and all nine simulations in the file carry the same ones; no record says the rocket was weighed as flown: the file's parts and overrides are flown; the reader warns that the motor's curve file disagrees with ThrustCurve.org's record of the motor (the figures are in the flight's `reader_warnings` in the JSON); the file the design's digest names is flown. Note: the stored simulation's wind carries a turbulence of a tenth of its speed, which is not flown: the wind here is steady. The parachutes are not flown either; the file opens one at apogee and the other lower, on the way down, so neither moves it.
  - OpenRocket's own run of this simulation, stored in the design file, reached 691.6 m. That is another simulator's number, not a measurement; it is shown and never counted.
  - The reader's warnings on this file and motor are in the JSON under this flight's `reader_warnings`: 3 in all.

## The weather file

The three EuRoC 2022 flights' ERA5 file is netCDF-4, which HPR Sim does not read. `validation/oracles/netcdf/euroc_2022.py` converts it as the [guide][era5] says, `xarray.open_dataset(source).drop_vars(["number", "expver"], errors="ignore").to_netcdf(out, format="NETCDF3_64BIT")`, and checks the result: the four fields hold the source's values bit for bit, and RocketPy reads the same levels from both files at each flight's site and hour. Its record is `validation/fixtures/weather/euroc-2022-conversion.json`.

| file | SHA-256 |
|---|---|
| `refs/rocketpy/data/weather/euroc_2022_all_windows.nc` (netCDF-4, as RocketPy ships it) | `b4583116c89c9e2b8fb9a88ef61d6f7c2c25329559184f7bde7040694ac0d52d` |
| `refs/converted/rocketpy/euroc_2022_all_windows.nc` (converted, flown) | `3a3b43a749938e07cee8ad1d705dc89dc8a0c1fcac135c76520e17ad8860b003` |

[era5]: https://hpr.fusionspace.co/format/era5.html#converting-a-current-file
