# ADR-376: M1.14b18, the inputs' part and the band (2026-11-23)

- **Status:** accepted; fixes the rules [ADR-373][adr-373] decision 3 left open for the input part; completes M1.14b7; answers [#1615](https://github.com/nrdptel/fusionspace-eridanus/issues/1615); changes no comparison error, apogee or mean of the report
- **Summary:** Each public flight gets an input part u_input: the standard deviation of its apogee over 200 flights with five inputs drawn at once (mass, total impulse, wind, rail angle, air temperature), each on a cited uncertainty. u_input runs from 23.8 m (78 ft) to 140.0 m (459 ft), the largest part on every flight. Each of the 8 flights with a u_D gets a validation uncertainty and a band of twice it about its error against the corrected log: 3 lie within, 5 outside. The count moves with the inputs: 6 or 7 within at the tops of the impulse's range, 2 with a rocket's growth as built taken out. The headline errors stay on the logs as logged.

[adr-373]: 0373-m1-14b7-in-three-the-numerical-part-first.md
[adr-375]: 0375-m1-14b17-the-logs-part-of-a-real-flights-validation-uncertainty.md
[note]: ../research/validation-uncertainty-vv20.md
[report]: ../../validation/reports/real-flights.md#how-much-of-each-error-the-inputs-could-be
[band]: ../../validation/reports/real-flights.md#the-band-each-error-against-its-validation-uncertainty
[sigmas]: ../research/dispersion-sigmas.md
[impulse]: ../research/hpr-lot-impulse.md
[wind]: ../research/era5-wind-direction-us.md
[air]: ../research/era5-surface-temperature.md
[as-built]: ../research/as-built-mass-growth.md
[synthetic]: ../physics/synthetic-flights.md
[descent]: ../../validation/reports/descent-wind.md

**Context.** [ADR-373][adr-373] decision 3 gave M1.14b18 the last of the three parts of a real
flight's validation uncertainty: the input part, from the flight's spread over its stated input
uncertainties, and with it the band. [ADR-375][adr-375] fixed three things for it. The band is
about the error against the corrected log (decision 7). The four flights with no u_D get no band
(decision 8). The air is drawn once, through the flight and through HPR Sim's barometric reading
(decision 10). What was open: which inputs, on which figures, sampled how, and how the count is
reported.

**Decision.**

1. **Five inputs are moved, by seven numbers.** The mass without motors (a factor), each motor's
   total impulse (a factor on its thrust and on its propellant's mass together, as more or less
   of the same propellant), the wind (east and north, added at every height), the rail's angle
   (up and across, about the rail's own direction) and the air's temperature (one offset in
   kelvins). The drag is not an input: it is the model the comparison tests.
2. **Each input takes a standard uncertainty for its kind,** not for the flight's own rocket. No
   team here published a measured one.

   | input | figure | flights | source and reading |
   |---|---|---|---|
   | mass, a team's stated | 0.5% | 10 | taken as weighed: the survey's own inference for a weighed mass, and one student team's figure ([the survey][sigmas], §3); hobby sources assume 0% to 3.9% about a file's value |
   | mass, a design file's | 10.75% | Macie XL's 2 | scale against file over 24 rockets: quartiles −1.4% and +13.1%, the range over 1.349 ([the note][as-built]) |
   | impulse | 2.5% | all 12 | the floor of its range: 21 motors tested twice differ by 3.56 points between their tests ([the note][impulse], §2), and a flown motor against its file is such a difference less one test's measuring error, 2.5% to 3.56%, or to 5.03% where a test's figure is the mean of three motors (decision 3). Borrowed for the 3 curves that are a team's own or of no stated origin |
   | wind, ERA5's at the hour | 2.0 m/s an axis | 9 | by day, inferred from forecast models held to soundings and lidars ([the note][wind], §6) |
   | wind, not the day's | 4.47 m/s an axis | Prometheus, Macie XL's 2 | the safety code's launch limit of 20 mph as a bound on a wind from any direction: half of it on each axis |
   | rail | 1° an axis | all 12 | a tilt of 1° is assumed in five of the seven hobby sources that disperse it; none measured it ([the survey][sigmas], §5). The same 1° is taken across the heading |
   | air, ERA5's at the hour | 0.9 K | 9 | ERA5 against soundings it never took in, at one desert site, the lower end of 0.9 to 1.35 K ([the note][air], §1) |
   | air, not the day's | 8.66 K | Prometheus, Macie XL's 2 | within 15 K of the standard atmosphere as bounds, 15/√3 ([the model's page][synthetic]) |

3. **The impulse's figure is a flown motor against its file's curve.** The file is one
   certification test and the motor that flew is another motor of the type. If a test is the
   type's mean plus a motor's own departure and the stand's measuring error, two tests differ by
   `2σ_motor² + 2σ_stand²` in variance and a flown motor differs from its file by
   `2σ_motor² + σ_stand²`. The pairs' 3.56 points then give 3.56/√2 = 2.5% if all of the scatter
   is the stand's and 3.56% if all of it is from motor to motor. The split isn't known.

   That top assumes each test fired one motor, and the tests' records don't print how many they
   fired. Where a test's figure is the mean of n motors, two tests differ by
   `2σ_motor²/n + 2σ_stand²` and a flown motor from its file by `σ_motor²(1 + 1/n) + σ_stand²`.
   The floor is the same 2.5%, and the top is 3.56·√((n + 1)/2): 5.03% at three, the least
   Tripoli Motor Testing's rules ask today of a retest in these classes ([the note][impulse],
   §1). The floor is taken, and the band's count is given at both tops too (decision 9).
4. **Each input alone, then all together.** Alone, each of the seven numbers is moved one
   standard uncertainty down and up, and half the difference of the two apogees is its part. The
   root sum of squares of the seven is the local estimate ([the note][note]: the sensitivity
   method). Together, the flight is flown 200 times with all seven drawn from normal
   distributions (the sampling method). **u_input is the standard deviation of the 200.** The
   local estimate is printed beside it. It says which input the spread comes from, and it
   checks the sample.
5. **The draws are two Latin hypercubes of 100,** on seeds 376 and 377. Each number's
   distribution is cut into 100 shares of equal probability, a flight takes one share of each
   number at the share's median, and the shares are paired across numbers at random. The report
   keeps each hypercube's own mean and spread. u_input is put together from those, so a check
   that flies nothing can hold it to them.
6. **The air is drawn once** ([ADR-375][adr-375], decision 10). A column ΔT warmer keeps each
   level's pressure and the pad's, and its layers are thicker by the ratio of their mean
   temperatures. The flight is flown in that air and its barometric reading is taken in the same
   air. A flight in the standard atmosphere takes the standard offset by ΔT with the pad's
   pressure kept.
7. **Two refusals.** Flown with nothing moved, a flight must give the report's own apogee to the
   last bit. A sample that fails to fly fails the report: none is dropped.
8. **The band.** u_val is the root sum of squares of u_num, u_input and u_D. A flight is within
   its band when the size of its error against the corrected log is at most twice u_val. A flight
   with no u_D has no u_val and no band.
9. **The count is by sign, and it is given both ways.** The report counts the flights within,
   those outside with HPR Sim high and those outside with it low, and those with no band. Beside
   it, it gives the count with the impulse at 3.56% and at 5.03%, the two tops of its range
   (decision 3), and names the flights outside at all three figures. It also gives the count with a rocket's median growth as built,
   7.4% of the mass without motors, taken out of the flights on a design file's mass
   ([the note][as-built]). That median is not taken out of the error itself: the note trusts it
   little as a figure for all rockets.
10. **The headline errors stay on the logs as logged**
   ([#1615](https://github.com/nrdptel/fusionspace-eridanus/issues/1615)). The accuracy page's
   headline says so and links the corrections.

**Why not the alternatives.**

- *Plain random draws.* 200 of them know a standard deviation to about 5%, and every flight
  shares the same draws, so all 12 would err the same way.
- *A point drawn inside each share,* the usual hypercube. It is unbiased, but the two end shares
  have no far edge, so one deep draw there moves a hypercube's spread. The medians give every
  hypercube the
  same 100 deviates, with a standard deviation 0.14% under the distribution's. On a flight that
  answers in a straight line, u_input over the two hypercubes is then 0.39% too small.
- *The local estimate alone.* It leaves out what a straight line leaves out. On Prometheus it
  stands 7.4% over the sampled figure.
- *The impulse at 2.5% as the figure, not the floor.* An earlier draft took one test's scatter
  about the motor's mean. That is not the input: the file is itself one test. A review found it.
- *The impulse at the middle of its range.* A middle needs the split between the stand and the
  motors, which no source gives. The floor is the narrower band, the unflattering side, and
  the count at the other end is printed beside it.
- *Each team's own stated spreads.* RocketPy's Bella Lui notebook gives some: 0.010 kg on the
  mass, 3% on the impulse, 1° on the inclination. They are assumed, not measured, and only some
  flights have any. One table for a kind of input keeps the 12 comparable.
- *The headline on corrected logs.* A correction is worked out for 8 of the 12 logs, so one mean
  would mix two kinds of figure. The corrected errors are in the band's table.

**Which way the choices lean.** A larger u_input makes a wider band, which an error fits inside
more easily. The smaller reading is taken five times. The teams' masses are taken as weighed.
The impulse takes the floor of 2.5% to 3.56%. Its factor carries the propellant's mass with the
thrust, and more thrust from the same propellant would move an apogee more. ERA5's wind takes
its daytime figure, and ERA5's air the lower end of its range. This project's own comparison of
ERA5 with flights' GPS descents stands further apart than 2 m/s an axis, the canopy's own motion
included ([its report][descent]). The larger reading is taken three times. Air and wind that
aren't the day's get bounds for any day; Prometheus's weather file is of the same date a year
later, and is likely nearer its day than that. The rail takes 1° across its heading as well as
along it, where the sources give 1° to 5° of heading, a smaller turn on a rail near the
vertical. And an error in the wind or the air is taken as the same at every height.

**What was measured** ([the input part][report], [the band][band]).

- u_input runs from 23.8 m on Bella Lui to 140.0 m on Prometheus, and from 2.15% of the apogee
  (Andromeda) to 13.55% (Macie XL's second flight, on its design file's mass).
- The impulse's part is the largest of the seven on 10 of the 12 flights. On the two Macie XL
  flights it is the mass's.
- The local estimate stands at most 7.5% from u_input, and one hypercube's spread at most 3.9%.
- The samples' mean lies below the report's apogee on all 12 flights, by at most 26.4 m.
- Of the 8 flights with a band, 3 lie within it (Juno III, Lince, Macie XL's first flight) and
  5 outside: HPR Sim high on NDRT 2020 and Cavour, low on Prometheus, Genesis and Macie XL's
  second flight. The errors are 1.43 to 3.04 times their u_val.
- With the impulse at 3.56%, by the seven parts, 6 of the 8 lie within: NDRT 2020, Cavour and
  Genesis join. Prometheus and Macie XL's second flight are outside at both ends.
- With the growth as built taken out, by the mass's flight alone, Macie XL's errors are −195.9 m
  and −343.3 m. The first is then outside its band, and 2 of the 8 lie within.

**What this does not decide, and where the figures are unsure.**

- Whether any team weighed its rocket as flown. If a stated mass is a design figure, its spread
  is the design file's, about twenty times wider.
- How the scatter between two tests of a motor splits between the stand and the motors, and how
  many motors each of those tests fired, which isn't printed. Four calls rest on the two: NDRT
  2020, Cavour and Genesis are outside their bands at the floor and within at 3.56%, and
  Prometheus is within at 5.03% alone.
- The impulse of a team's own motor. No spread was found for one, and the commercial figure is
  borrowed for Juno III, Andromeda and Erebus 11.
- Whether today's curves carry the retests' drift: the second test of the 22 pairs read a median
  2.65% lower than the first ([the note][impulse], §2), and some of the motors had changed.
- The 15 K range for air that isn't the day's. It is this project's own, with no outside source.
- Four known effects with a sign, left in E and not corrected: ERA5's air flown dry (0.08% to
  0.65% low, [the note][air], §4), Andromeda's and Juno III's thrust files against their catalog
  or recorded impulse, and a design file's mass against the rocket as built (a median 7.4%
  heavier, [the note][as-built]).
- What is left out, each of which would add to u_input: the burn time at a fixed impulse, the
  center of mass, a thrust or fin misalignment, the rail's length and friction, the pad's
  pressure and the site's elevation.
- Whether the inputs are independent. It is assumed.
- Whether the impulse's error is in the thrust alone or in the propellant's mass too. The factor
  moves both.
- The 1979 paper on Latin hypercubes (McKay, Beckman and Conover) was not read. The construction
  is the common one, from the summary of V&V 20 that names it ([the note][note]).
- The smaller points left by the log's part's review
  ([#1616](https://github.com/nrdptel/fusionspace-eridanus/issues/1616)) stay open.

**Done when** (ADR-373's, for M1.14b18), and where each clause is met:

- *Each public flight's row gives its input part from its dispersion over stated input
  uncertainties, each one cited.* The section's two tables, a row in each for each of the 12
  flights, and
  its list of each figure with its flights and source.
- *And its validation uncertainty.* The band's table, for the 8 flights with a u_D. The other 4
  have none, by [ADR-375][adr-375]'s decision 8.
- *The report counts the flights whose error lies within twice it.* The sentence under the
  band's table: 3 of 8, with those outside by sign, and the count at the impulse's other end
  and with the growth as built taken out.
- *The accuracy page explains the three parts, what an error outside the band means, and that
  meeting the band is not the aim.* Its paragraphs on the integrator, the log and the inputs,
  and its list under "The band".

**Consequences.** Each row of the report's validation uncertainty gains its input part, its
validation uncertainty and the band's call. The report's consistency check holds each to its
flight's stated inputs and to its own parts, and the check that flies the flights again holds
every apogee. M1.14b7 is met. A better figure for one input changes one row of decision 2's
table, and the report says which flights it moves.
