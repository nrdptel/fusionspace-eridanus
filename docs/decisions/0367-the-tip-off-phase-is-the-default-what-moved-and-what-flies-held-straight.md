# ADR-367: The tip-off phase is the default: what moved, and what flies held straight (2026-11-22)

- **Status:** accepted; carries out [M1.14b15][m1-14b15] under [ADR-363][adr-363] decision 1; changes the flight HPR Sim flies by default for a design with two or more rail guides at different stations
- **Summary:** A design with two or more rail guides at different stations now leaves the rail by the tip-off phase unless told not to, from the library, the command line and the Python package. Comparisons with OpenRocket and RocketPy 1.13.0, which have no such phase, fly with it off and say so. Every report that moves is regenerated. Of the accepted census's rows, 16 moved, 7 for the worse, each as [ADR-364][adr-364] measured before the default changed. The readings' and the fit's interval shares are refitted on synthetic flights flown on the default; one cell of the fit's test then fails and is recorded ([ADR-368][adr-368]).
- **Scoreboard:** `accuracy.collection.hpr`, `bias.collection.hpr` and `accuracy.from-boost.hpr` worse in release 0.2: on the collection's calibration half the phase moves the mean absolute error from 9.45% to 9.46% and the mean error from +4.56% to +4.58%, and the error of the apogee predicted from the boost from 3.17% to 3.19%. [ADR-364][adr-364] measured the collection's rise to its flights before the default changed; the phase itself rests on its derivation and on RocketPy's rewritten phase in still air ([ADR-357][adr-357], [ADR-363][adr-363]), not on these flights.

[m1-14b12]: ../decisions-and-roadmap.md#m1-14b12
[m1-14b14]: ../decisions-and-roadmap.md#m1-14b14
[m1-14b15]: ../decisions-and-roadmap.md#m1-14b15
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-184]: 0184-logged-apogees-of-the-private-collection.md
[adr-240]: 0240-the-scoreboard.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-357]: 0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
[adr-358]: 0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md
[adr-362]: 0362-base-drag-behind-a-boattail-below-mach-0-8-held-to-the-hermes-a-3a.md
[adr-363]: 0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[adr-364]: 0364-the-tip-off-phase-on-the-real-flights-two-small-rises-measured.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
[fit-report]: ../../validation/reports/fit-intervals.md
[flights-report]: ../../validation/reports/tip-off-flights.md
[collection]: ../../validation/reports/fixture-flights.md
[real]: ../../validation/reports/real-flights.md
[census]: ../../validation/reports/census.md
[synthetic]: ../../validation/reports/synthetic-flights.md
[flight-page]: ../physics/flight.md#tip-off

**Context.** HPR Sim's tip-off phase lets a rocket pivot about its last guide once the guide
ahead of it has left the rail ([the phase][flight-page]). It was opt-in while it was checked:
against its own derivation, against RocketPy's rewritten phase in still air, and on every real
flight HPR Sim compares ([ADR-357][adr-357], [ADR-358][adr-358], [ADR-363][adr-363],
[ADR-364][adr-364]).
[ADR-363][adr-363] decision 1 then set [M1.14b15][m1-14b15]'s *done when*: every flight of a
design with two or more guides flies the phase unless told not to, from the library, the command
line and the Python package alike; each report that moves is regenerated in that pull request;
each comparison with a code that has no such phase flies with it off and says so; and
[M1.14b14][m1-14b14]'s table is regenerated on the default. This record says what that change
moved and what was decided along the way.

**Decision.**

1. **The default.** `Simulation::new` flies the phase for a design with two or more guides at
   different stations. A design with fewer has no stretch on its last guide alone and flies as
   it did. The ways to turn it off:

   | surface | how |
   |---|---|
   | library | `Simulation::without_tip_off`; `FlightBuilder::tip_off(false)`; `FlightInputs::tip_off` |
   | command line | `--no-tip-off` on `hpr sim`, `hpr mc` and `hpr compare`; the JSON output's `launch.tip_off` says which flew |
   | Python | `tip_off=False` on `Flight` and `MonteCarlo` |

   `Simulation::with_tip_off` stays and only undoes the opt-out.

2. **A comparison with a code that has no such phase flies held straight on the rail.**
   OpenRocket and RocketPy 1.13.0 hold a rocket straight until it is off the rail. So the
   flights held to RocketPy 1.13.0 (`cargo xtask validate`), the flights held to OpenRocket's
   (`cargo xtask ork-flights` and the runs built on it), and in the collection's report every
   row that sets HPR Sim beside OpenRocket fly with the phase off: "hpr in OpenRocket's air",
   "hpr against OpenRocket", the split of the gap by cause, the nose's line and the drag by
   Mach band and by term. Those reports reproduce as they were. The collection's report gains
   a row, "hpr held straight on the rail", the flight those rows start from, beside "hpr", the
   default flight held to the logs ([the report][collection], in aggregate only:
   [ADR-151][adr-151], [ADR-184][adr-184]). Each report says which of its flights fly with the
   phase off.

3. **What moved on the real flights** is what [ADR-364][adr-364] measured with the phase on and
   off ([the table, regenerated on the default][flights-report]):

   | set | flights | mean absolute error, held straight | default | mean error, held straight | default |
   |---|---:|---:|---:|---:|---:|
   | public, compared first | 7 | 6.065% | 5.974% | +0.026% | +0.074% |
   | public, predicted first | 5 | 12.818% | 12.819% | −11.161% | −11.167% |
   | the private collection | 60 | 12.095% | 12.098% | +8.007% | +8.015% |

   Over the 12 public flights the mean absolute error is now 8.83%, from 8.88%, and the mean
   error −4.61%, from −4.64% ([report][real]). The collection's prints 12.10%, from 12.09%
   ([report][collection]). No drag, mass or thrust input changed.

4. **The census: 16 rows moved, 7 for the worse, accepted.** All 16 are public flights against
   their logs ([census][census]). Nine come nearer the log: Juno III's apogee the most, from
   −6.39% to −5.55%, then Lince's, Cavour's, Andromeda's and NDRT 2020's apogees and the climbs
   of Juno III, Lince, Cavour and NDRT 2020. Seven go farther: Prometheus 2022's apogee, from
   −8.90% to −9.22%, and its climb, from 4.90% to 5.04%; Genesis's apogee by 0.03 points and its
   climb by 0.01; Erebus 11's apogee by 0.02 points; Bella Lui's apogee and climb by under 0.01
   each. [ADR-364][adr-364] traced each to its flight: gravity lowers a rocket on its last
   guide, and the wind's part of the phase raises two flights. The worse rows are accepted
   because the phase is the more complete model of the rail and was checked on its own
   evidence, not chosen on these flights: Juno III's fall is as little a reason for it as
   Prometheus 2022's rise is one against it.

5. **Three merged predictions are revised** beside their merged numbers ([ADR-328][adr-328]
   §1b): Astra, Andromeda and Erebus 11 each have two rail buttons, and their predicted apogees
   fall by 0.02 to 0.51 m. Erebus 11's entry keeps its earlier revision's reason too
   ([ADR-362][adr-362]). The two Macie XL predictions have no guide and hold to the merged
   digit.

6. **The scales' flights fly the phase.** A public flight flown again with a drag or impulse
   scale (`Variant::corrected`) flies as its report does, with the phase, so that both scales at
   1 reproduce the report's own apogee. Before, it was held straight, and the check that ties
   it to the report failed by the phase's move.

7. **The interval shares are refitted, not held.** The synthetic flights fly the default, so
   the calibration sets' flights changed and `cargo xtask synthetic` and `cargo xtask fit`
   refused until the library's shares were the ones fitted on them: each run fits the shares
   again and fails unless they equal the library's ([calibrated intervals][adr-257]). The
   readings' shares move by at most 0.031 of a bound (landing's 95% point, 0.156 to 0.125); the
   fit's by at most 0.09. The readings' two test sets, which the shares never saw, pass the
   same tests with them ([report][synthetic]). The fit's do but for one cell: on set B the
   least-squares interval on the drag scale now leaves 80 of 1,000 truths below it, where a
   tail passes with 29 to 74 and it read 71. That cell is recorded as failing, not fitted
   away ([ADR-368][adr-368], [the fit's report][fit-report]).

8. **The tour's worked flight follows its rule.** The guide's tour takes the public flight whose
   miss is the median of those compared along their climb. Juno III's move makes that Cavour,
   where it was Genesis, and the page moves with it.

**What this does not decide.** Whether the phase's wind part is right on HPR Sim's own normal
force: that comparison is [M1.14b12][m1-14b12]'s, still open, and the flight page says so.
Nor does it claim the phase makes HPR Sim more accurate: two of the three sets read a little
worse with it.

**Consequences.**

- A user's flight of a design with two guides changes by what the phase moves: on the real
  flights, under 1% of the apogee, and under 0.4% on every flight of the collection with two
  guides. `--no-tip-off` or `tip_off=False` gives the earlier flight.
- A new comparison with another code must say whether that code has such a phase, and fly
  HPR Sim to match.
- The scoreboard's three cells above are worse by 0.01 to 0.02 points in release 0.2
  ([the ratchet][adr-240]).
- The least-squares fit's drag interval fails its test on set B, a known gap with its own
  record ([ADR-368][adr-368]).
