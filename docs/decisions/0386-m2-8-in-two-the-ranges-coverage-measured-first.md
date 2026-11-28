# ADR-386: M2.8 in two: the ranges' coverage measured first (2026-11-28)

- **Status:** accepted; splits [M2.8][m2-8] into M2.8a and M2.8b, each with its own *done when*, M2.8's own *done when* whole across the two; carries out [ADR-209][adr-209] §5; changes no tolerance, model or flight
- **Summary:** M2.8a flies each of the private collection's 60 compared flights 1,000 times more as the run `hpr mc` flies, on six input spreads stated from published figures before any count, and reports how often the run's 50% and 90% apogee ranges hold the logged apogee, with two-sided 95% Clopper-Pearson bounds, over every flight and each half. M2.8b puts that spread on `hpr sim`'s readout and plot, with the measured share beside it, and closes #401. The counts were predicted here before the first run: about 20 of 60 at 50% and about 47 of 60 at 90%, both short of their levels, the misses on the side where the run flew higher than the log.

[m2-8]: ../decisions-and-roadmap.md#m2-8
[adr-134]: 0134-monte-carlo-dispersion-independent-normals-one.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-376]: 0376-m1-14b18-the-inputs-part-and-the-band.md
[adr-377]: 0377-no-release-waits-on-users-and-release-0-2-comes-next.md
[collection]: ../../validation/reports/fixture-flights.md
[public]: ../../validation/reports/real-flights.md
[report]: ../../validation/reports/apogee-ranges.md
[sigmas]: ../research/dispersion-sigmas.md
[coast]: ../research/flight-derived-cd.md
[lots]: ../research/hpr-lot-impulse.md
[burn]: ../research/hpr-burn-time-scatter.md
[mass]: ../research/as-built-mass-growth.md
[two-units]: ../research/dual-altimeter-spread.md

**Context.** `hpr mc` flies a design many times with its inputs scattered and prints a range of
apogees. Nothing has checked such a range against real flights: the Monte Carlo guide says so, and
the scoreboard's two cells for it read "not measured". [ADR-209][adr-209] §5 asks for the check:
each logged flight flown as a dispersion, and a report of how often its 50% and 90% ranges hold the
logged apogee. M2.8's *done when* adds the halves, the bounds, a failed run counted as a miss, and
issue #401: that spread beside each flight's apogee and as the plot's band. Release 0.2 needs #401
([ADR-377][adr-377]).

That is two pieces of work. The first is a validation report over the private collection's flights.
The second changes what `hpr sim` prints and draws for every user, and is done on an acceptance
pass over its captures ([ADR-282][adr-282]). The second can't be written honestly before the first
has a number: a band drawn at "90%" claims a share of real flights, and the share isn't known yet.

**Decision.**

1. **M2.8 in two.** M2.8 is met when both increments are.
   - **M2.8a, the ranges' coverage.** *Done when:* `cargo xtask apogee-ranges` flies each flight
     the collection's report compares with its log as a dispersion on spreads stated before any
     count (decision 3); `validation/reports/apogee-ranges.md` gives how often the 50% and 90%
     apogee ranges hold the log, over every flight compared and each half, with two-sided 95%
     Clopper-Pearson bounds, a failed run a miss; `--check` reproduces it; the scoreboard's two
     cells read it; the accuracy page gives the counts. (As first written, before the run, this
     named `cargo xtask fixture-flights` as the command. The runs have their own, for the reason
     under *What was measured*; nothing else in it changed.)
   - **M2.8b, the spread on the readout and the plot.** *Done when:* #401 closed, that spread
     beside each flight's apogee and as the plot's band, each with the share of logs its range
     held (M2.8a's report) beside it; its acceptance pass ([ADR-282][adr-282]).
2. **The run is the product's.** Each flight is flown by `hpr_analysis::montecarlo::MonteCarlo`,
   the run `hpr mc` flies ([ADR-134][adr-134]): every input an independent normal draw about its
   value, one random stream a sample and input. The range M2.8b shows is then the range M2.8a
   counted. The validation uncertainty's draws ([ADR-376][adr-376]) answer another question, how
   far the inputs alone can move a flight whose model is under test, and leave the drag out for
   that reason; they aren't used here.
   - **The flights** are the 60 the [collection's report][collection] compares with their logs,
     each from the inputs that report flew: the design as read, the day's ERA5 air and wind, its
     rail, the separations the climb flies, nothing deployed. Before a run, its nominal flight is
     flown again from those inputs and held, within one part in a million, to the apogee the
     collection's code gives that flight.
   - **1,000 flights a run.** A flight's seed is the first eight bytes of the SHA-256 of its id
     followed by `ADR-386`, so a run depends on its flight's id alone.
   - **A range** at 50% runs from the run's 25% point to its 75% point, at 90% from its 5% point
     to its 95% point, by Hyndman and Fan's definition 7, the quantile `hpr mc` prints. The apogee
     is the height the center of mass climbed, which is what the collection's report compares.
3. **Six spreads, each a stated figure, none fitted.** A flight in the collection is known by its
   design file and its motor's published curve: no record says a rocket was weighed as flown. Each
   standard deviation below is taken from a note merged before this record. Where the figure a
   row cites is itself a range, its wider end is taken, since a range too narrow is the error
   that flatters.

   | input | standard deviation | from |
   |---|---:|---|
   | mass without motors | 10.75% | the quartiles of 24 rockets' scale mass over their file's ([as-built mass][mass]), as [ADR-376][adr-376] reads them |
   | zero-lift drag coefficient | 16.3% | the quartiles, 0.89 to 1.11, of 14 published coast fits over the design code's drag ([flight-derived drag][coast] §2), over 1.349; the wider end of [the spreads note][sigmas]'s 10% to 16% |
   | total impulse | 3.56% | two certification tests of one motor type ([lot impulse][lots] §2): a flown motor stands 2.5% to 3.56% off its file's curve, the wider end |
   | burn time, at the same impulse | 9.2% | one test's about its motor's mean, tests years apart ([burn-time scatter][burn] §3); 4.1% within a lot, the wider taken |
   | rail's angle above the horizon | 1° | five of seven hobby sources ([the spreads note][sigmas] §5), each assumed |
   | rail's heading | 5° | the wider end of 1° to 5° in the same sources |

   The notes hold wider figures still, which are not taken (a correction made in review, before
   the merge: as first written, this decision said the wider end was taken wherever a note gives
   a range, which is true only of the range each row cites). They are an impulse of 5.03% if each
   certification figure is the mean of three motors ([ADR-376][adr-376]); a burn time of 10.7%
   measured in one hobby source and up to 19% assumed in others; a tilt of 1.5° and 2° in two of
   the seven sources; and a heading drawn evenly over a full turn ([the spreads note][sigmas],
   its table and §5). Each would widen a range. The spreads flown are the table's, unchanged.

   Every draw is centered on the file's value. The 24 rockets weigh a median 7.4% over their
   files, and that shift is left in the error, as [ADR-376][adr-376] leaves it: a correction
   across flights is another milestone's, and the last one tried didn't hold up.
4. **Left at the values flown, each of which would widen a range.**
   - **The wind and the air.** The run's wind spread is a share of the speed, which is no measure
     of a reanalysis's error, and it has no air temperature. On the 12 [public flights][public],
     where the validation uncertainty draws both, the wind's two parts together move the apogee by
     0.29% to 2.41%, under 1% on 9 of the 12, and the air's part by under 0.3% on 9.
   - **The center of mass:** no stated spread was found for it.
   - **The altimeter's own scatter:** two units in one rocket stand a median 0.37% apart
     ([two altimeters][two-units]). M2.8b's spread is of the apogee, not of a reading of it.
   A term of 1% added in squares to a spread of 10% widens it by a two-hundredth.
5. **A failed run is a miss.** A sample that fails can't help a range hold: the low edge counts
   each failed sample as above the run's apogees and the high edge as below them, the narrowest
   range the failed samples could have left. A run with no range left, and a run that can't be
   set up, count as misses at both levels. The flights HPR Sim doesn't fly at all aren't runs:
   they have no simulated apogee either, and stay in the collection's report under their issues.
   Counting them as misses would lower a share for a reason that isn't the range's width, and
   could hide a 50% range that is too wide.
6. **The bounds and what they mean.** Each share carries its two-sided 95% Clopper-Pearson
   interval. A level outside the interval is a range these flights show to be wrong. A level
   inside it is one they can't tell from right: 23 flights can't tell 90% from 75%.
7. **Nothing is tuned to the count.** The spreads stay as decision 3 states them whatever the
   report shows. [ADR-209][adr-209] §7's target for release 0.3, both levels inside their bounds,
   stands and isn't claimed here. The scoreboard's two cells read the share over every flight
   compared, with the count beside it.
8. **The prediction, written before the first run.** From the collection report's errors (a mean
   of +8.01% and an RMS of 15.42% over the 60, by its histogram) and a spread near 10% of the
   apogee (the mass's part near 6%, the drag's and the impulse's near 5% each, by elasticities
   near −0.6, −0.35 and +1.5):
   - the 50% range holds about 20 of the 60 (between 14 and 25), so 50% is outside or at the edge
     of its bounds;
   - the 90% range holds about 47 of the 60 (between 43 and 53), short of 90%;
   - most misses have the log below the range, the run having flown higher;
   - the held-out half, whose errors are the larger (+13.54% mean), does worse than the
     calibration half: about 5 of 23 and 15 of 23 against about 15 of 37 and 32 of 37.

**What was measured.** The [report][report] has every count; these are the ones the prediction
named.

| | predicted | measured | 95% bounds |
|---|---|---|---|
| 50% range, the 60 | about 20 (14 to 25) | 18 (30.00%) | 18.84% to 43.21% |
| 90% range, the 60 | about 47 (43 to 53) | 50 (83.33%) | 71.47% to 91.71% |
| 50% range, the held-out 23 | about 5 | 5 (21.74%) | 7.46% to 43.71% |
| 90% range, the held-out 23 | about 15 | 18 (78.26%) | 56.29% to 92.54% |
| 50% range, the calibration 37 | about 15 | 13 (35.14%) | 20.20% to 52.54% |
| 90% range, the calibration 37 | about 32 | 32 (86.49%) | 71.22% to 95.47% |

- **The levels.** 50% is outside its bounds over the 60, as predicted. 90% is inside its bounds
  over the 60 and over each half, which the prediction's "short of 90%" didn't settle either way:
  the share is below 90% in every set but one, and no set can tell it from 90%.
- **The side.** At 50% the log is below the range on 31 flights and above it on 11; at 90%, on 9
  and 1. The logged apogee sits a median 6.76% below its run's median. That is the lean of the
  collection report's errors (a median of +7.60%, the simulated apogee above the log), as
  predicted.
- **The width.** A run's standard deviation is a median 10.37% of its apogee (8.60% to 17.83%),
  against the "near 10%" the prediction assumed.
- **The held-out half** does worse than the calibration half at both levels, as predicted, and
  better at 90% than the 15 predicted.
- **No sample failed** of the 60,000, and every run was set up, so decision 5's rule moved no
  count.

Three things about the run itself:

- **Its own command.** The runs fly 60,000 flights, and the command prints how long they took.
  In the unoptimized build `cargo xtask fixture-flights` runs in, that is about a quarter of an
  hour on the machine that wrote the report; in the workspace's `report-check` profile
  (optimized, its debug assertions on) about a minute and a half. The collection's report can't move to that profile: a
  record it reads is keyed to a hash of the conditions it flew, and those bytes differ between
  the two builds in their last places. So `cargo xtask apogee-ranges` is its own command, written
  and checked on `report-check`, and it sets up the 60 flights from the same inputs by the same
  functions the collection's report uses.
- **Tied to the collection's report twice.** Each run's nominal flight is flown again and held to
  the apogee the same build computes within one part in a million (decision 2). And each set's
  nominal errors are held to the committed collection report's own row: the count, the histogram
  and the flights within 5% and 10% exactly, the mean, median, mean absolute and RMS within
  0.011 points, the report's rounding and the builds' last places together.
- **The bounds are printed outward.** A bound is rounded down at the interval's lower edge and up
  at its upper edge, so a printed interval is never narrower than the exact one. A review before
  the merge asked for it; it moves seven of the table's twelve bounds by 0.01 point and no count,
  share or verdict.

Nothing was changed after the first count: the spreads, the seed rule, the sample count and the
rule for failed samples are as committed before it, in the commit that carries the prediction.

**Consequences.** The Monte Carlo guide and the accuracy page say how often the ranges held,
with the counts. Issue #401 stays open for M2.8b, which may not print a range as "90%" without
the measured share beside it. The two-layer test of the burn-time spread (#1082) stays open: this
record takes the wider layer and tests one set of spreads.
