# ADR-364: The tip-off phase on the real flights: two small rises, measured (2026-11-21)

- **Status:** accepted; carries out [M1.14b14][m1-14b14] under [ADR-363][adr-363] decision 1, whose *done when* asks for this record: with the tip-off phase the mean absolute apogee error rises over two of the three sets of real flights; changes no flight HPR Sim flies by default
- **Summary:** Every real flight HPR Sim compares was flown twice, guided to its last guide as today and with the tip-off phase. Over the seven flights compared first the mean absolute apogee error falls from 6.065% to 5.974%, a fall that rests on Juno III's rise in its wind. Over the five predicted first it rises by 0.0014 percentage points, and over the private collection's 60 by 0.0033. Both rises are under the hundredth of a point the reports print. Each is measured to its flights: one is gravity lowering a flight HPR Sim already flies low, the other the phase raising, on net, a set HPR Sim already flies 8% high. M1.14b15 may make the phase the default.

[m1-14b12]: ../decisions-and-roadmap.md#m1-14b12
[m1-14b14]: ../decisions-and-roadmap.md#m1-14b14
[m1-14b15]: ../decisions-and-roadmap.md#m1-14b15
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-184]: 0184-logged-apogees-of-the-private-collection.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[adr-358]: 0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md
[adr-363]: 0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[report]: ../../validation/reports/tip-off-flights.md
[report-split]: ../../validation/reports/tip-off-flights.md#where-the-change-comes-from
[collection]: ../../validation/reports/fixture-flights.md#the-tip-off-phase-m114b14
[model-report]: ../../validation/reports/tip-off-model.md
[rocketpy-report]: ../../validation/reports/tip-off.md

**Context.** HPR Sim's tip-off phase lets a rocket pivot about its last guide once the guide
ahead of it has left the rail. It is opt-in. Before [M1.14b15][m1-14b15] makes it the default,
[M1.14b14][m1-14b14] measures what it does to the real flights' apogee errors
([ADR-363][adr-363], decision 1). Its *done when* says: if any set's mean absolute apogee error
is higher with the phase, a decision record measures the rise and its cause before the default
changes. Two sets are higher. This is that record.

`cargo xtask tip-off-flights` writes [the report][report]. It flies each public flight four
ways, each as its own report flies it: guided and with the phase, in the report's wind and in
still air. Still air keeps the report's air and takes out only the wind, so what the phase moves
there is gravity's turn about the last guide alone. `cargo xtask fixture-flights` flies the
private collection's flights the same four ways and writes only their aggregate
([its section][collection]; [ADR-151][adr-151], [ADR-184][adr-184]). Each set's error is its own
report's.

**Decision.**

1. **The measurement** ([the report][report]):

   | set | flights | with two guides | mean absolute error, guided | with the phase | change |
   |---|---:|---:|---:|---:|---:|
   | compared first | 7 | 7 | 6.065% | 5.974% | −0.091 points |
   | predicted first | 5 | 3 | 12.818% | 12.819% | +0.0014 points |
   | the private collection | 60 | 23 | 12.095% | 12.098% | +0.0033 points |

   The signed means move from +0.026% to +0.074%, from −11.161% to −11.167%, and from +8.007%
   to +8.015%. Both rises are smaller than the hundredth of a point the real-flight report and
   the collection's report print. No landing point is compared for any of these flights, so
   none moves.

2. **Where each rise comes from** ([the split][report-split]). A set's change in mean absolute
   error is the sum of two parts: the rises in absolute error of the flights the phase moved
   away from their logs, and the falls of those it moved toward them, each summed and divided by
   the set's flights.
   - **Predicted first, +0.0014 points.** Of its five flights, the two Macie XL flights have no
     guide at all, so the phase moves nothing. It lowers the other three, by 0.02 to 0.51 m, and
     lowers them in still air as well. Erebus 11, flown 8.05% low, is moved 0.51 m farther from
     its log: +0.0034 points. Andromeda, flown 4.14% high, is moved 0.37 m toward its log:
     −0.0021 points. Astra's move of 0.02 m adds +0.0001. The rise is gravity's lowering of a
     flight HPR Sim already flies low.
   - **The private collection, +0.0033 points.** 23 of its 60 flights fly a design with two
     guides at different stations (20 of its 42 designs); 11 of the 23 fly from a vertical rail.
     The phase moves 14 of the 23 away from their logs and 9 toward them. The flights moved away
     give +0.0142 points, those moved toward −0.0110. The mean signed error rises from +8.007%
     to +8.015%: on net the phase raises the apogees of a set HPR Sim flies 8.0% high on
     average, so more move away than toward. How many it raises and how many it lowers is held
     back, in the wind and in still air (see the floor under *Consequences*). In the wind no
     flight moves by more than 0.4% of its apogee; in still air none rises by 0.1% or more.
   - **The compared-first flights fall, −0.091 points.** Juno III is the largest move: the phase
     raises its apogee by 26.5 m, toward its log, which HPR Sim reads 6.4% low (−0.120 points).
     Prometheus 2022 is lowered by 12.7 m, away from its log (+0.046 points). Without Juno III's
     part the set's error would rise as well.

3. **The cause: gravity lowers; the wind moves the rest, unchecked.** In still air the phase
   lowers 9 of the 10 public flights with two guides by more than 1 cm. The tenth, NDRT 2020,
   flies from a vertical rail, where gravity has no moment about the last guide, and moves
   1 mm. That is gravity turning a rocket on a tilted rail nose-down about its last guide, as
   RocketPy's phase lowers Juno III's apogee in still air ([its report][rocketpy-report]). In
   the day's wind the phase raises 2 of the 10 public apogees, Juno III and Bella Lui. Juno III
   in still air is lowered by 16.7 m; in its wind it is raised by 26.5 m. Taking out the wind
   changes the guided flight as well, so the two moves are not parts of one sum.
   The wind's part of the turn is the part of HPR Sim's phase not held to RocketPy's on its own
   normal force: in a crosswind its turning rate differs from RocketPy's by up to 4.010 deg/s
   against a bar of 0.100 ([the comparison][model-report]; [M1.14b12][m1-14b12] holds it open).
   On Juno III in that comparison's crosswind the two differ in sign: HPR Sim's phase raises
   the apogee by 4.35 m and RocketPy's lowers it by 3.90 m. Against the flight with the phase
   off, RocketPy's phase does raise Juno III in a crosswind (+0.321%, against −0.378% in still
   air), so a wind can turn the phase's lowering into a rise, but against the guided flight,
   the comparison made here, it lowers it by 0.157%. So the collection's net rise and Juno
   III's rise rest on the phase's least-checked part. They do not show that the phase is
   wrong, and they do not show it right.
4. **M1.14b15 may make the phase the default.** Both rises are measured, smaller than the
   reports' rounding, and traced to their flights. The fall over the seven flights compared
   first is no argument for the default: it leans on Juno III's wind part, the part decision 3
   leaves unchecked. [ADR-358][adr-358]'s decision 6 put the default ahead of M1.14b12, and
   [ADR-363][adr-363] decision 4 kept that order; neither changes here. When M1.14b15
   regenerates this report on the default, it says again whether any set's error is higher.

**Consequences.**

- The tip-off flights' report is committed and held: `cargo xtask tip-off-flights --check`
  joins the gate's refs step beside `real-flights --check`, and a unit test holds the committed
  page and its sums without refs/.
- The collection's report gains its tip-off section, held by `fixture-flights --check`; its
  other figures reproduce unchanged.
- The bins of the collection's moves keep [ADR-363][adr-363]'s floor of five flights and settle
  the cases it left open: the bin with the fewest flights is merged with its smaller neighbor,
  again until every bin holds five or more, and with fewer than five flights in all the count is
  given alone, with no bin. A group of counts (raised, not moved, lowered) is given only where
  each holds no flight or at least five, and the split's two parts only where both the flights
  moved away and those moved toward number five or more; otherwise they are held back. This is
  stricter than the three of [ADR-346][adr-346]'s rule against figures given by subtraction.
  In the day's wind that leaves one bin of all 23 flights and the counts held back; in still
  air, two bins and the counts held back.
- A move under 1 cm counts as not moved, so a vertical rail's still-air flight, where gravity
  has no moment about the last guide, is not counted as raised or lowered by a rounding error.

**What this leaves open.**

- **No real flight checks the phase.** The logs read an apogee, and the phase moves apogees by
  at most 0.9% here, against mean absolute errors of 6% to 13% over the three sets.
- **The wind's part is held to nothing.** Until [M1.14b12][m1-14b12] holds the crosswind turn on
  HPR Sim's own normal force, Juno III's rise and the collection's net rise rest on an
  unchecked part of the phase.
- **Landing points.** None of these flights compares one, so what the phase does to a landing
  point is pinned by two tests on a design, not measured on a flight. Which way leaving the
  phase out errs depends on the rail's tilt and the wind: on a tilted rail in still air the
  guided flight lands nearer the pad, on a vertical rail in a crosswind farther from it.
