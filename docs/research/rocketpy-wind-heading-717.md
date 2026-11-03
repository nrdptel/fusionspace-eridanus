# RocketPy's wind and heading convention (issue #717), sized

This note checks whether RocketPy 1.13.0 takes the wind or the launch heading the wrong way, as its issue #717
suggests, and sizes what such an inversion would do to the flights the project compares. It follows
[RocketPy defects that move an apogee](rocketpy-apogee-defects.md) §(b), which listed #717 as unsized. Read and
flown on 2026-11-03. Every number below was measured by this project unless it is credited to a source.

**In short.**
- **Nothing is inverted in 1.13.0.** `wind_u` and `wind_v` are the air's velocity toward east and north, the heading
  is clockwise from north, and every equation of motion takes the airflow as wind minus rocket velocity (code read
  [2]). The flight in #717 turned west because a stable rocket turns into a wind blowing from the west: weathercocking,
  not a sign error (**Inference**, from a reproduction, §2). A real log agrees: Juno III's GNSS track drifts the way
  RocketPy's wind says it should, and the reversed wind points the drift the opposite way (§4).
- **The project's oracle and HPR Sim read the convention the same way,** and the signed east and north drifts of the
  four windy descents agree within 2.9% [9]. No comparison cell inherits a mismatch.
- **If the convention were wrong, it would matter.** Reversing the wind moves the apogees of the seven public flights'
  RocketPy notebooks by −10.15% to +1.41% (mean absolute 2.24%), and Juno III's by −10.15%. Mirroring the heading
  moves them by −11.75% to +0.29%. The notebooks' landing drifts over 100 m move by up to 95%. So the convention is
  load-bearing; it is just not broken.
- **What is real in #717's case is a different, known defect.** In #717's 30 m/s crosswind, RocketPy as released
  turns 21% further into the wind than with PR #1188 (nozzle term) and PR #1196 (#1186's center-of-mass sign)
  applied, and its apogee is 5.6% lower (§2). The oracle already flies both ([ADR-026][adr-026]). This moves
  M2.9's RocketPy cells (#717 can leave the unsized list) and the whole-flight comparisons in wind.

## 1. What RocketPy 1.13.0 defines

All from the installed 1.13.0 package (`rocketpy-1.13.0.dist-info`) and the pinned checkout `9bd6ad3` [2], MIT.

- **Wind components.** The `custom_atmosphere` docstring calls `wind_u` the wind speed "heading East" and `wind_v`
  "heading North" (`environment/environment.py:1519-1537`). The class attributes say the same: `wind_velocity_x` is
  the east component, `wind_velocity_y` the north one (`:139-144`). `custom_atmosphere` stores them unchanged
  (`:1584-1585`).
- **Derived angles.** `calculate_wind_heading(u, v)` is `atan2(u, v)` in degrees, mod 360: the direction the wind
  blows *toward*, clockwise from north (`environment/tools.py:40-69`; its doctest gives 90° for u = 1, v = 0).
  `convert_wind_heading_to_direction` subtracts 180°: the direction it blows *from* (`:72-87`). Both are labeled
  that way in the attributes (`environment.py:133-138`). For the Calisto wind (5, 2) m/s they give 68.2° and 248.2°.
- **Reanalysis files.** The ECMWF mapping reads the file's `u` and `v` straight into `wind_u` and `wind_v`
  (`environment/weather_model_mapping.py:99`, `:112`; interpolation at `environment.py:2018-2041`). ECMWF defines `u`
  as the speed of air moving toward the east [6], so the two agree.
- **Rail heading.** `Flight(heading=...)` is degrees from north, 90° pointing east (`simulation/flight.py:526-528`;
  user guide `docs/user/flight.rst:100`). The initial attitude takes precession −heading and nutation
  inclination − 90° (`flight.py:1557-1560`). Flown calm on the oracle's Calisto, heading 0° puts apogee 449 m
  north (4 m west) and 90° puts it 445 m east [10].
- **Where the equations use the wind.** Only the two velocity components enter a trajectory. The rail phase, the
  solid-propulsion and default 6-DOF equations all form the free stream as wind − velocity (`flight.py:1830-1834`,
  `:2012-2015`, `:2536-2540`). The parachute phase forms velocity − wind and applies drag against it
  (`:2764-2775`): the same airflow, with both signs flipped. The heading and direction functions are for display
  and output only (**Inference**, from a search of `flight.py` for both names).
- **Two output quirks, neither in a trajectory.** `lateral_surface_wind` is `−u cos h + v sin h`: positive means
  blowing toward the rocket's left, a sign its docstring does not state (`flight.py:1762-1775`). And a calm wind
  reports a heading of 0° and a direction of 180°, since `atan2(0, 0)` is 0.

## 2. Issue #717 reproduced

The issue [1] (open, labeled Bug, Environment and Flight) flies a rocket of 30 kg without its motor at heading
11.6°, inclination 80°, with `wind_u` = 30 m/s and `wind_v` = 0, and reports that it flew west. Its attached script
[1] comments `wind_u` as positive from west to east, which is the code's convention. A maintainer replied the same
day that something is wrong with the `wind_u` definition, present since v1.0 or earlier, and a month later
suggested the `Flight.u_dot` methods. No later comment, linked PR or size exists (timeline read through
2025-03-03, the last event).

The issue's rocket needs its own drag files, so this note flew the oracle's Calisto (rail exit 28.2 m/s) with the
same rail angles and wind, on Calisto's 5.2 m rail instead of the script's 7 m [10]:

| flight (Calisto, heading 11.6°, inclination 80°) | apogee, m AGL | apogee east, north, m | landing east, m |
|---|---|---|---|
| calm air, corrected equations | 2,544.7 | +173, +862 | +196 |
| `wind_u` = +30 m/s, as released | 2,080.5 | −1,311, +707 | +4,816 |
| `wind_u` = +30 m/s, corrected (PRs #1188, #1196) | 2,205.0 | −1,081, +742 | +5,260 |
| `wind_u` = −30 m/s, corrected | 2,097.1 | +1,320, +703 | −4,830 |

- The rocket turns into the wind and the parachute carries it downwind: west to apogee, then far east to land. That
  is the expected sign for a statically stable rocket. A 30 m/s crosswind on a 28 m/s rail exit is an angle of
  attack of about 48° at rail exit, so a turn of this size is expected too (**Inference**).
- Reversing the wind mirrors the result east to west, as it should.
- What the oracle's corrections remove is the released equations' extra 231 m west at apogee (21% more than
  corrected) and 124.4 m (5.6%) less apogee: PR #1188's nozzle term and PR #1196's fix of #1186's center-of-mass
  sign, together (this probe does not separate them). In all six flights flown here, the released equations move
  the rocket further *into* the wind (**Inference**: the maintainer's second comment points at the same equations,
  but the issue never names #1186).
- A second report of drift "the wrong way", #930 (2026-02-26, parachute drift against a tailwind), was closed the
  same day by its reporter, who found the error in their own code [5].

## 3. Does the oracle or HPR Sim inherit it?

- **Same inputs, same meaning.** `flight.py` and `recovery.py` declare `wind_u` and `wind_v` per case and pass them to
  `custom_atmosphere` [8]. HPR Sim reads them back as the east and north velocity and converts them to its
  from-direction with `atan2(−east, −north)` (`crates/hpr-validate/src/rocketpy.rs:271-297`, `run.rs:715-721`), the
  convention of [Wind](../physics/wind.md). The rail heading becomes HPR Sim's azimuth unchanged (`run.rs:856`), and
  `frames::tests::launch_angles_match_the_rocketpy_oracle` pins the launch attitude to 1e-12 rad
  ([Frames](../physics/frames.md)). HPR Sim reads ERA5's `u` and `v` as east and north too
  (`crates/hpr-io/src/era5/mod.rs:697-713`).
- **What pins the sign.** The four windy descents score signed east and north drifts. In the committed report they
  agree within 2.9%, with every sign the same: Calisto's east part 1,286.99 m against RocketPy's 1,286.03 m,
  Prometheus 2022's −612.91 against −612.39 m, NDRT 2020's north part −50.83 against −49.42 m [9].
- **What cannot see a mirror.** The whole-flight cells score apogee and landing drift as distances, not directions
  (`run.rs:1458-1478`). Calisto flies at heading 0°, where an east-west mirror changes its drifts by only −1.1% and
  +0.5%, and NDRT 2020 flies a vertical rail, where any flip changes them by under 1% (§4). Both are under the 3% gate.
  So those cells alone would not catch a mirrored convention; the signed descent cells and the attitude test do
  (**Inference**).

## 4. Size: the windy flights with the wind flipped

Three variants, each against the flight as given: **reversed** (both components negated, as if "toward" were read
as "from"), **east flipped** (`wind_u` negated), and **heading mirrored** (heading becomes 360° − heading). An east
flip and a heading mirror are the same flight reflected across the north axis; they agree to within 0.4% of apogee here.

**The oracle's five windy whole flights,** same-drag, corrected equations, the declared winds [8], [10]. The flights
as given reproduce the committed fixture's apogees and drifts to the printed digit.

| case (heading, wind east, north) | apogee, m | reversed | east flipped | landing drift, m | reversed | east flipped |
|---|---|---|---|---|---|---|
| Calisto (0°; 5, 2 m/s) | 2,612.2 | −0.90% | +0.03% | 1,261.5 | −26.6% | +0.5% |
| NDRT 2020 (181°, vertical; shear) | 1,216.4 | +0.00% | +0.00% | 347.5 | +0.7% | +0.7% |
| Prometheus 2022 (75°; −4, 7) | 3,679.2 | +2.77% | +5.38% | 1,382.6 | +24.1% | +43.3% |
| Juno III (105°; 8, −3) | 2,614.8 | −9.03% | −8.26% | 478.5 | +39.6% | +37.7% |
| Bella Lui (45°; 3, −2) | 530.3 | −0.18% | −0.58% | 67.2 | +13.4% | +37.4% |

Apogee drift moves by −39.7% (Prometheus 2022, east flipped) to +247.9% (Juno III, reversed). As released, the
reversed wind moves the same five apogees by −10.56% to +3.48%.

**The seven public flights' RocketPy notebooks,** each run as its example does (released equations, its own drag,
rocket and ERA5 file), with only the wind or the heading changed [7]. Parachute noise was seeded alike in every run.

| notebook (heading, inclination) | apogee, m AGL | reversed | mirrored | landing drift, m: given, reversed, mirrored |
|---|---|---|---|---|
| Bella Lui (45°, 89°) | 460.6 | −0.40% | −0.17% | 5, 55, 36 |
| NDRT 2020 (181°, 90°) | 1,293.3 | +0.02% | 0.00% | 458, 460, 457 |
| Prometheus 2022 (75°, 80°) | 4,191.2 | −1.34% | −1.46% | 3,279, 153, 806 |
| Juno III (105°, 85°) | 2,997.9 | −10.15% | −11.75% | 546, 770, 769 |
| Cavour (133°, 84°) | 2,817.9 | +1.41% | −0.28% | 1,264, 579, 887 |
| Genesis (133°, 84°) | 3,044.6 | +1.01% | −0.37% | 628, 914, 391 |
| Lince (133°, 84°) | 3,284.8 | +1.34% | +0.29% | did not land within the run |

- **Sign and size.** A reversed wind changes the mean absolute apogee by 2.24% over the seven, a mirrored heading by
  2.05%. Juno III dominates: it leaves the rail at 15.7 m/s in a 10.8 m/s ground wind on its quarter, so the wind
  sets how far it turns. The fast rails of Cavour, Genesis and Lince (42 to 49 m/s) keep them within 1.5%.
- **Against HPR Sim's real-flight score** (mean absolute apogee error 6.06%, [Accuracy](../accuracy.md)), a
  convention error of this kind would be about a third of the mean, and on Juno III as large as the largest misses
  (−8.90% to +10.40%) (**Inference**: these are RocketPy's flights, so they size the sensitivity, not HPR Sim's).

**Which convention the logs support.** Juno III's GNSS log [4] puts its highest fix 1,027 m from the pad toward
185°, and its fixes then move 1,948 m toward 53° under the drogue. RocketPy with the wind as given puts apogee
1,335 m toward 226° and drifts 1,657 m toward 63°. Reversed, it puts apogee 2,075 m toward 75° and drifts toward
240°, the opposite of the log. Mirrored, apogee is toward 247°. Genesis's log [4] stays at the pad's position
until at least 60 s; its last fix is 409 m toward 86°, against 628 m toward 100° as given and 914 m toward 156°
reversed. Both logs favor the code's convention (**Inference**: this note computed the bearings from the logs'
latitudes and longitudes).

## What this suggests

- **M2.9's RocketPy cells:** move #717 from "unsized" to "examined: no inversion in 1.13.0's trajectory code; the
  reported turn is weathercocking, overstated by the released equations as in #1186". Expected effect: one fewer
  unsized defect, and the cells stop implying that the windy references might point the wrong way.
- **[rocketpy-apogee-defects](rocketpy-apogee-defects.md) §(b):** the #717 bullet could point here, with its
  "unbounded" replaced by "would move the seven notebooks' apogees by up to 11.8% if it were real; it is not".
- **The whole-flight comparisons in wind:** report the landing point's signed east and north offsets beside the
  distances. The fixture already records them (`impact.east_m`, `impact.north_m`). Expected effect: a mirrored
  heading or wind could no longer pass Calisto's and NDRT 2020's distance-only cells (§3).
- **Terms.** RocketPy is MIT, so its code is read and run, and the scratch scripts and results are cached under
  `refs/research/rocketpy-wind-heading-717/`. Issue text is its authors' and is restated, not copied. The Juno III and
  Genesis logs come with RocketPy's data but state no license (the Juno III README says only that the team shared
  the data with RocketPy); the ERA5 files are Copernicus downloads, under the Copernicus license. They are cited and
  read in place, and only derived bearings and distances are published.

## Open questions

- No RocketPy maintainer has said what they meant by "something wrong with the `wind_u` definition". If it is a
  defect outside the trajectory (a plot or a printed angle), nothing here would find it.
- The Juno III log disagrees with RocketPy as given by 41° at apogee, and the cause is unbounded: the rail heading
  as flown, ERA5's wind error ([ERA5 wind direction](era5-wind-direction-us.md)), and the released
  equations' over-turn all bear on it.
- HPR Sim's own real-flight apogees were not reflown with the wind reversed (no cargo runs here), so the sensitivity
  above is RocketPy's, not HPR Sim's. Since HPR Sim turns into the wind less than released RocketPy does, its change
  should be smaller (**Inference**, unsized).
- Lince's notebook run ended before landing, so its drift sensitivity is unknown.

## Sources

1. RocketPy issue #717, its comments and timeline (REST API), and its attached script `Wind_cross_flight.txt`:
   https://github.com/RocketPy-Team/RocketPy/issues/717
2. RocketPy 1.13.0 source, commit `9bd6ad3`: `rocketpy/environment/environment.py`, `environment/tools.py`,
   `environment/weather_model_mapping.py`, `simulation/flight.py`, and the user guide `docs/user/flight.rst`,
   `docs/user/rocket/rocket_axes.rst`: https://github.com/RocketPy-Team/RocketPy/tree/v1.13.0 (MIT)
3. RocketPy issue #1186 and PR #1196 (center-of-mass sign), and PR #1188 (nozzle term, merged 2026-09-09), as in
   [rocketpy-apogee-defects](rocketpy-apogee-defects.md): https://github.com/RocketPy-Team/RocketPy/issues/1186
4. Juno III's `data/rockets/juno3/cots_GNSS.csv` (444 rows counted) and `README.txt`, and Genesis's
   `data/rockets/genesis/gnss_faraday.csv` (246 rows with a fix), at `9bd6ad3`:
   https://github.com/RocketPy-Team/RocketPy/tree/v1.13.0/data/rockets (no license stated; cited only)
5. RocketPy issue #930, "Parachute wind direction": https://github.com/RocketPy-Team/RocketPy/issues/930
6. ECMWF parameter database, parameter 131, U component of wind: https://codes.ecmwf.int/grib/param-db/131
7. RocketPy example notebooks at `9bd6ad3`, `docs/examples/{bella_lui,ndrt_2020,prometheus_2022,juno3,cavour,genesis,
   lince}_flight_sim.ipynb`: https://github.com/RocketPy-Team/RocketPy/tree/v1.13.0/docs/examples
8. The project's oracle scripts `validation/oracles/rocketpy/flight.py`, `recovery.py`, `corrections.py`, and the
   fixture `validation/fixtures/flight/rocketpy-whole-flight.json`.
9. The project's validation report `validation/reports/latest.json` (descent and whole-flight drift comparisons).
10. This note's runs: `refs/research/rocketpy-wind-heading-717/scripts/` (`size_717.py`, `probe_717.py`,
    `notebooks_717.py`) and their outputs, RocketPy 1.13.0 in the oracle environment; not committed.

[adr-026]: ../decisions/0026-the-path-in-wind-rocketpys-corrected-equations.md
