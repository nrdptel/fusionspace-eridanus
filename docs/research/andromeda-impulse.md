# Andromeda's thrust curve: not the M2020 it flew

This note asks why RocketPy's Andromeda example flies a team thrust curve 17% above the Cesaroni
8429M2020 that the rocket flew, and which curve HPR Sim should use if Andromeda joins the public
real-flight set ([public flights](public-flights.md#usable-now-mit-design-and-measured-flight-cots-motor)
lists it with this caveat). It is for whoever adds that flight. The curve's arithmetic, the catalog
figures and the team's own posts are sourced; which test the curve came from is **Inference**,
strong against the M2020 reading, moderate for the alternative. All sources read on 2026-10-15.

**In short.** The team curve cannot be a fair test of the M2020 reload, and the flight itself
says the reload delivered about its catalog impulse.
- **The file is clean.** It integrates to 9,866.7 N·s; no unit factor, zero offset or sampling
  slip explains the 17% (finding 1).
- **The reload can't deliver it.** 9,867 N·s from the reload's 4.35 kg of propellant is a specific
  impulse of 231.5 s; ThrustCurve's 15 Cesaroni Imax motors span 180.8 to 204.8 s (finding 2).
- **The flight agrees with the catalog.** Flown in RocketPy, the catalog curve reaches each of six
  heights from 100 to 1,000 m within 0.05 s of the log; the team curve runs up to 0.54 s ahead
  (finding 4).
- **A likely source.** The team built its own motor, Sirius, which they rate at 9,750 N·s, 1.2%
  from the curve's 9,867 N·s. Sirius flew Andromeda's second EuRoC 2022 flight; the logged one was
  the first, on the M2020 (finding 3).
- **What it moves.** Andromeda should fly the catalog M2020 curve. That lowers RocketPy's apogee
  for it from 3,618 m to 3,060 m against 3,439 m logged: from 5.2% high to 11.0% low with the
  team's drag file. The airbrakes it flew with are not modeled, and their effect has no bound yet.

## 1. The team curve: what the file holds

**The file is internally sound; its impulse is real arithmetic, not a slip.** RocketPy's
`data/rockets/andromeda/thrust_curve.csv` (commit `9bd6ad3`, added in PR #754 on 2024-12-15 with
no provenance beyond the notebook) has 2,224 rows of time (s) and thrust (N) from 0 to 3.760 s
[1, 2]. Counted and integrated here with `refs/research/andromeda-impulse/curve.py`:

| quantity | team curve | catalog 8429M2020-P [3, 4] |
|---|---:|---:|
| total impulse | 9,866.7 N·s (trapezoid) | 8,429.4 N·s |
| peak thrust | 3,347.7 N at 1.106 s | 2,680.4 N (maker); 2,649.4 N in the cert file |
| burn | 3.760 s, file end | 4.17 s (maker); cert file ends at 4.301 s |
| average thrust | 2,624 N over 3.760 s | 2,021.9 N |

- **Sampling.** 2,035 steps are 1 ms and 188 are 8 to 10 ms: bursts of 11 to 13 samples every
  19 to 21 ms. Mean thrust times the 3.760 s span gives 9,876 N·s, 0.1% from the trapezoid, so the
  uneven steps can't account for the 17%.
- **Units.** The ratio is 1.171. No unit factor is near it (lbf to N is 4.448, kgf to N 9.807).
- **Zero offset.** Removing the gap with a constant would take 382 N off every sample. The first
  sample is 1.8 N and thrust rises from there, so the zero at ignition is not off by that much.
- **Time base.** Stretching a slow clock would lengthen the burn, but the team curve is already
  0.4 s shorter than the catalog's. A time-base slip moves the impulse the wrong way.
- **Truncated tail.** The last nonzero sample is 133.6 N at 3.752 s, then a zero at 3.760 s. The
  catalog curve still holds 149 N·s after 3.76 s, so a cut tail makes the gap larger, not smaller.
- **Shape.** Over 0.5 to 3.0 s the team curve runs 1.13 to 1.29 times the catalog curve (1 ms
  grid), at a similar regressive shape. A load-cell gain about 1.2 times too high would look like this; so
  would a different, hotter motor. The file alone can't separate the two.

## 2. The M2020 reload cannot deliver 9,867 N·s

**Sign: the team curve overstates the reload by about 17%; size: at least 13%.** Total impulse
over propellant weight is the delivered specific impulse, I_sp = I / (m_p · g₀).
- The notebook gives the motor 4.346 kg of propellant (its grain, 0.893 m long) [2]. Cesaroni and
  ThrustCurve give 4,349 g for the reload [3, 4]. With the team curve that is 231.5 s; with the
  catalog impulse, 197.6 s, which is what Cesaroni lists.
- ThrustCurve's search (2026-10-15, 296 Cesaroni entries with impulse and propellant weight) has
  15 whose propellant is Imax (one listed as Imax/Dual Thrust): 180.8 to 204.8 s, median 194.5 s
  [5]. Even at 204.8 s, 9,867 N·s needs 4.91 kg of propellant, 13% more than the reload holds.
- **Inference:** a static test of a stock M2020 might sit a few percent off the cert curve, not
  17%. The CAR cert data are dated 2010 (test date 04/18/2010 on the maker's page) [3, 4].
- **Not another Cesaroni reload.** Of the 24 Cesaroni 75 mm M motors on ThrustCurve, the nearest
  in impulse is the 9977M2245-P (Pro75-6GXL, Imax): 9,976.7 N·s, 1.1% from the team curve, and a
  listed 3,357 N peak [5]. Its cert curve peaks at only 3,046 N, 9% below the team curve's, is flat
  near 3,000 N from 1.4 to 2.7 s and ends at 4.371 s; the team curve peaks at 1.1 s and falls [6].
  It also needs the longer 6GXL case. **Inference:** a coincidence in impulse, not the motor.

## 3. What the team says: two flights, two motors

**Andromeda flew twice at EuRoC 2022; the logged flight was the commercial one.**
- ASAT (Aristotle Space & Aeronautics Team, Aristotle University of Thessaloniki) reports two
  launches, to 3,446 m and 4,076 m, on a COTS motor and on its own motor, Sirius [7]. Faulty
  chemicals kept Sirius off the first flight; it flew the second, within three days [7]. Both
  flights carried the team's airbrakes, "an automatic control system" aimed at a target altitude.
- RocketPy's notebook dates its flight to 2022-10-14, names the M2020, and quotes 3,443 m from
  the flight card [2]. The log's own peak is 3,438.5 m (11,281 ft) [1]. So the log is the first,
  COTS flight (**Inference**, from the apogees). The notebook says only that the curve was
  "obtained by the team, after testing".
- Sirius is an M-class potassium nitrate and sorbitol motor (65:35). The team rates it at
  "It of 9750Ns" [8]; the curve's 9,867 N·s is 1.2% above that. A 2022 post tabulates 7.5 kg of
  propellant, 132 s specific impulse, 3,750 N peak and a 3 s burn [9]. Those give 9,709 N·s, 1.6%
  below the curve; the same table's "3721 Ns" disagrees with its own row (**Inference:** a typo).
- The team static-tests its own motors on a load-cell stand read by an STM32 microcontroller [8];
  the burst-and-gap sampling of finding 1 fits such a logger. **Inference.**
- Searched and not found: a EuRoC 2022 technical report, a team static-test record, or a RocketPy
  issue on the curve's origin. HeroX's 2022 Spaceport America Cup page for ASAT links no
  documents [10]. Andromeda's June 2022 Spaceport flight (3,461 m, 10K COTS) names no motor [11].

**Inference, moderate confidence:** the curve is a Sirius static test. For: its impulse matches
the team's Sirius rating to 1.2% and the M2020 is ruled out by finding 2. Against: its peak is 11%
below Sirius's tabulated 3,750 N, its burn 25% longer than 3 s, and finding 5 can't reproduce the
4,076 m Sirius flight with it.

## 4. The flight log matches the catalog impulse

**The boost climb fits 8,429 N·s and not 9,867 N·s; the apogee fits neither with this drag file.**
`refs/research/andromeda-impulse/sim.py` flies the notebook's inputs in RocketPy 1.13.0 (same
weather file, 84° rail, team drag curve) with each motor, stopping at apogee [1, 2]. Times are
from the 10 m crossing, in seconds; the log is a COTS altimeter at 20 Hz on ascent [1, 2].

| curve | 100 m | 200 m | 400 m | 600 m | 800 m | 1,000 m | 3,000 m | apogee AGL |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| log | 0.97 | 1.55 | 2.41 | 3.22 | 3.91 | 4.60 | 16.48 | 3,439 m |
| team, 9,867 N·s | 0.90 | 1.44 | 2.21 | 2.84 | 3.43 | 4.06 | 14.28 | 3,618 m |
| catalog, 8,429 N·s | 0.99 | 1.59 | 2.46 | 3.17 | 3.88 | 4.64 | 20.34 | 3,060 m |
| team scaled to 8,429 N·s | 0.98 | 1.56 | 2.41 | 3.11 | 3.82 | 4.58 | 20.57 | 3,049 m |

- **Boost.** The catalog curve stays within 0.05 s of the log at every height to 1,000 m. The
  team curve's lead grows from 0.07 s to 0.54 s, as an impulse excess would; a fixed altimeter
  lag would give a constant offset instead. **Inference.**
- **Impulse bound.** Scaling the catalog curve by 1.05 moves the 1,000 m time to 4.47 s
  (`scan.py`). Interpolating to the log's 4.60 s gives about 8,520 N·s; the 600 m time (catalog
  ×0.97 reaches it at 3.234 s, ×1.00 at 3.174 s, log 3.218 s) gives about 8,240 N·s. So the climb
  supports about 8,240 to 8,520 N·s, −2.2% to +1.1% of the catalog.
- **Mass alternative.** The team curve matches the climb to 1,000 m only with about 4.6 kg more
  liftoff mass, 18% above the notebook's 25.32 kg (`massscan.py`). The notebook's rocket mass is
  unsourced, so this can't be ruled out from the log, only from finding 2.
- **Apogee sensitivity.** Each 1% of impulse moves RocketPy's apogee by about 1.1% (scaling by
  1.10 gives 3,396 m). The notebook's saved 3,614.95 m is the team-curve row; this run's 3,618 m
  differs by 3 m. Stopping at apogee is not the cause: the notebook's own setup, parachutes and
  all, also gives 3,617.6 m in 1.13.0. A RocketPy change since the output was saved may be; unchecked.

## 5. What else moves Andromeda's apogee

- **Airbrakes: sign known, size unbounded.** Both flights used the team's airbrakes [7]; no
  model includes them and the log has no deployment channel [1]. Whether and how far they opened
  is not public. Any deployment makes the real apogee lower than a no-brake simulation, so it
  would widen the catalog curve's 11.0% shortfall.
- **The drag file.** `drag_coefficient.csv` is a time series of Mach and drag coefficient, up to
  Mach 0.852 and back down; RocketPy sorts it, giving about 0.44 to 0.58 above Mach 0.1 [1].
  On the catalog curve the rocket peaks at 288 m/s, inside that range; the team curve reaches 345 m/s, beyond it. On the
  catalog curve the simulated coast reaches 3,000 m 3.9 s after the log: drag or mass too high.
  HPR Sim builds its own aerodynamics from the notebook's geometry, so this one is RocketPy's.
- **The Sirius flight.** Flying the team curve on a Sirius-like motor (7.5 kg propellant, 12 kg
  motor [9]) gives 3,018 to 3,246 m (`sirius.py`, two guesses at the hardware mass) against 4,076 m
  reported. That flight's mass, brakes and log are not public; it is not a validation case.

## What this suggests

- **Fly Andromeda on the catalog M2020.** This reverses the first suggestion of
  [public flights](public-flights.md) (fly the team curve, the 17% stated) and answers its open
  question on the gap. ThrustCurve flags both CAR cert files for the 8429M2020-P public domain, so
  they may be cached or bundled [5]; RocketPy's copy of the same curve integrates to 8,428.7 N·s
  [1]. Expected effect: Andromeda's apogee drops about 15% against
  the team curve's (RocketPy: 3,618 m to 3,060 m, with 3,439 m logged).
- **List it apart until the airbrakes are bounded.** Its error mixes an unmodeled active brake of
  unknown size, sign known. Report it with the real flights but out of any held-out mean: the
  brakes push every no-brake simulation high against the log by an amount no one has measured.
- **Keep the team curve cited, not flown.** RocketPy's data files are MIT and cached under
  `refs/rocketpy` [1]. ASAT's posts are "All rights reserved"; cite them only.
- **Record the correction upstream.** A RocketPy issue saying the logged flight is the COTS one
  and the curve likely Sirius's would help other users. That is a post outside this repository,
  so it is a suggestion for the maintainer, not an action here.

## Open questions

- Which test made the curve: ASAT could confirm it is a Sirius static fire, and give its date.
- Andromeda's real liftoff mass on the M2020 flight, and whether the notebook's 20.98 kg includes
  the 2.5 to 2.7 kg of motor hardware (loaded 7,031.8 g, burnout 2,527.3 g [3]).
- When the airbrakes opened and how far: without that, Andromeda's apogee error has no bound.
- Why RocketPy's examples page gives 3,415 m while the card says 3,443 m and ASAT 3,446 m.

## Sources

1. RocketPy (MIT), commit `9bd6ad3`: `data/rockets/andromeda/thrust_curve.csv`,
   `drag_coefficient.csv`, `flight_data.csv`; `data/motors/cesaroni/Cesaroni_8429M2020-P.eng`,
   `Cesaroni_9977M2245-P.eng`. https://github.com/RocketPy-Team/RocketPy (read 2026-10-15).
2. RocketPy, `docs/examples/andromeda_flight_sim.ipynb` at `9bd6ad3`, the motor and flight-card
   cells and their saved output; PR #754, https://github.com/RocketPy-Team/RocketPy/pull/754
   (read 2026-10-15).
3. Cesaroni Technology, 8429M2020-P product page, https://pro38.com/products/p75-6g/8429m2020-p/
   (read 2026-10-15; maker's terms, cite only).
4. ThrustCurve.org, Cesaroni 8429M2020-P, https://www.thrustcurve.org/motors/Cesaroni/8429M2020-P/
   (read 2026-10-15): CAR cert 2010-06-09, two cert data files.
5. ThrustCurve.org API, `search.json` (Cesaroni, 75 mm, M: 24 motors; all Cesaroni: 296) and
   `download.json`, https://www.thrustcurve.org/api/v1/ (read 2026-10-15). Cert files for the
   M2020 and M2245 are flagged `PD`; cached as JSON under `refs/research/andromeda-impulse/`.
6. ThrustCurve.org, Cesaroni 9977M2245-P cert data via [5] (read 2026-10-15).
7. ASAT, "ASAT launches twice and takes 7th place in the European Rocketry Challenge 2022",
   2022-11-28, https://asat.gr/asat-launches-twice-and-takes-7th-place-in-the-european-rocketry-challenge-2022-euroc/
   (read 2026-10-15; all rights reserved, cite only).
8. ASAT, "Propulsion" (undated), https://asat.gr/propulsion/ (read 2026-10-15; cite only).
9. ASAT, "Sirius Vs I Class: Rocketry's Department most recent R&D Projects", 2022-09-28,
   https://asat.gr/sirius-vs-i-class-rocketrys-department-most-recent-rd-projects/ (performance
   table; read 2026-10-15; cite only).
10. HeroX, Spaceport America Cup 2022, team 14379 (ASAT),
    https://www.herox.com/SpaceportAmericaCup2022/team/14379 (read 2026-10-15).
11. ASAT, "ASAT's Rocketry Department among the top international rocketry teams", 2022-08-12,
    https://asat.gr/asats-rocketry-department-among-the-top-international-rocketry-teams/
    (read 2026-10-15; cite only).
12. Xometry, "Asat Launches A High-Power Rocket", 2023-12-12,
    https://xometry.pro/en-tr/stories/asat-high-power-rocket/ (read 2026-10-15): repeats the two
    apogees of [7].
