# ADR-316: M7.3b: the least-squares fit, what it solves for and how its intervals are held (2026-11-01)

- **Status:** accepted; carries out [ADR-314][adr-314]'s M7.3b under [ADR-257][adr-257] §1; amended 2026-11-02 (§7: CI fits in eight parts)
- **Summary:** Levenberg-Marquardt fits drag, impulse, a constant wind and mass to a log, with the
  clock, height offset and drift, and a barometer's gain and port, as further unknowns. Each
  parameter's 90% interval passes on sets A and B, counted on logs with something to fit. Those two
  sets were seen while the model's choices were made. Correlated residuals and, on heights alone,
  the wind's unknown loss widen the errors. A fit takes about a second. The wind is held only on
  logs with a track.

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-314]: 0314-m7-3-in-five-residuals-first.md
[report]: ../../validation/reports/fit-intervals.md
[page]: ../physics/flight-fit.md

## Context

ADR-314 asks for a fit whose intervals pass ADR-257 §1 on sets A and B, and for how fast one fit must
be. The [page][page] says what the fit is. This records what it took, in the order it came up, since
each step was made with sets A and B in view.

## Decisions

1. **The nuisances are unknowns.** The first fit, on drag, impulse, burn time, wind and mass, with
   clock and height offsets, had overconfident errors and drag biased by up to 7 standard errors on
   a barometer. Its gain, drift and port's dynamic-pressure error went into the drag. They're now
   unknowns, each with the range [`residuals`](../physics/log-readings.md) already gives it, and a
   barometer's bins at or past Mach 0.8 are left out. The drift is open on every log, the gain and
   port on a barometer's alone.
2. **A stated range is a prior.** Each parameter's range enters as a Gaussian penalty row, of width
   so that its 5% and 95% points are what the sets draw. The synthetic flights' drag is ±20%, impulse
   ±6%, and wind a uniform speed up to 8.94 m/s (29.3 ft/s) along a uniform direction. A mass's range
   is new: a weighed mass within ±2% of the structure's, drawn uniform on a stream of its own, so no
   existing flight's draw moved. ±2% is the maintainer's judgment of a kitchen scale and a loaded
   airframe, not a measurement.
3. **A track's floor is 10 m (33 ft), and shares split by track.** With a floor of 2 m (7 ft), logs
   with a track had wind shares of ±4 to ±6. A receiver's error is correlated from fix to fix, so
   fewer independent samples hold than the bins count. The floor is a judgment read from those
   shares. Since a track's errors are of another kind, a log with a track has its own shares, as the
   log itself says which it is.
4. **The wind isn't held on height alone.** Truth's 95% point on a log of height alone was 6.2 m/s
   (20 ft/s) on set A and 7.8 m/s (26 ft/s) on set B, whose wind shears. No one pair of shares could
   pass on both. So the wind's counts are on flights of the logger with a barometer and a GNSS receiver,
   drawn past each set's first 1,000 as far as it takes, and a height-alone log's wind isn't a stated result. Set A and B each still count
   drag, impulse and mass on their 1,000.
5. **The wind's counts stand on 1,500 track flights,** of which three on each set are left out,
   with nothing to fit or no track read, leaving 1,497. Its errors have long tails, so a narrowed interval moves few flights. On 1,010, the narrowed probe passed on set B: 71 below, 875 inside, 63
   above of 1,009. At 1,500 it fails, and every other band is tighter too, the wind's own test
   included. The done-when asks for at least 1,000. This adds power and doesn't touch a threshold, but
   it was done because the probe passed.
6. **Set B's truth is the same fit on noise-free samples.** Set B flies another model, so it has no
   true scale. The first truth, a fit reading every bin, left set B's errors larger than set A's for a
   reason in the definition. It's now the same fit, on the bins the log's fit read, at the same noise,
   with mass held as flown. It's given the flight's drift at every bin, even where the log holds no
   track, so that a log of height alone carries the wind's loss in its errors on set B as on set A,
   whose truth is the drawn value. Reading the log fit's bins, it inherits that fit's Mach 0.8 cut,
   so an error in where the cut falls is counted on set A alone.
7. **How fast a fit must be.** About 0.8 CPU-seconds a fit in the optimized profile and about 70 flights
   flown, measured on set A in development; set B's count, about 115, holds its truth's fit too.
   Writing the report takes about 16 minutes on 10 cores. It runs in one Linux job on CI and whole at
   the gate, on the platform that wrote the report, with calibration. On Linux, a count may differ
   from the committed report by 5 flights and a figure by 5% or 0.002; the first Linux run, on an
   earlier head of this work, differed by at most 1 flight and 5.01%, the latter on a figure within
   0.002. The pass and fail of every test and probe are taken again on Linux's own counts.
8. **Only logs with something to fit are counted.** A log that says nothing states its range, which
   holds about 90% whatever the fit does. Counted, those logs leaned the drag's, impulse's and mass's
   counts toward passing: a review counted set B's impulse at 50 below, 781 inside and 14 above of
   the 845 fitted, which fails, against a pass with them. Each set now counts its first 1,000
   fitted flights: of set A's first 1,216, 216 had nothing to fit; of set B's first 1,193, 193.
   Every one of them is a log whose only height is a GNSS receiver's, from which the reader takes
   no apogee, as the report counts and requires: every refused log is that logger's, one of the
   six, and no other cause removed a flight. Which logs drop out doesn't depend on how a fit did.
9. **A fit that can't be this flight's says nothing.** A review handed pure noise of ±5 m (16 ft)
   to the fit, which gave the impulse a median width of 0.019, under half its stated 0.108. A fit
   whose heights' RMS residual is over 0.1 of the spread of the heights it read now says nothing. On pure noise the
   least of 1,000 fits stands at 0.958; on every counted synthetic flight the most is 0.0251. The 0.1
   is a judgment with wide room either side. Noise now reaches the fit at ±5, ±50 and ±1,000 m on height
   alone, and at ±5 and ±1,000 m with a track of ±5 m, which is what the wind listens to.
10. **A fit runs until it stops.** At 12 steps a pass, a review found 16 of 157 fits ending on the
    limit, and their answers moved by up to 3.5 standard errors when run on. The limit is 60, and each fit says
    whether it stopped short. Of the flights counted or for the wind, 2 on set A and 7 on set B
    reach it, set B's truths' fits included. The shares were fitted again on the code as it now stands.
11. **A residual that runs on from bin to bin widens the errors.** The standard errors took the bins
    as independent, but a misfit is correlated from bin to bin. The height's and the track's
    variances are each widened by $(1+\rho)/(1-\rho)$, $\rho$ their residuals' lag-one
    autocorrelation, capped at 0.9: the effective number of independent values under a first-order
    autoregression (Wilks, *Statistical Methods in the Atmospheric Sciences*, 3rd ed., eq. 5.12).
    It's an approximation: Wilks gives it for the variance of a mean, the residuals at a fit's
    minimum read $\rho$ low, and the cap is a judgment; the calibrated shares take up the rest.
    Measured in development, not in the report: before it, with the shares fitted again, set A's drag (27 below, 918 inside, 55 above) and
    impulse (28, 922, 50) and set B's drag (81, 878, 41) failed. After it, only set A's drag failed,
    at 23 below.
12. **On heights alone, the wind's unknown loss widens the errors.** A log of height alone can't tell
    the wind, and the wind's loss grows as its speed squared, which the errors' first order at the
    stated wind leaves out. Set B's wind grows with height, by a power of 0.14 to 0.20, so its loss
    goes past what a constant wind of the stated range makes. Measured in development, not in the
    report: on set B's height-alone logs, the
    drag's mean error over its standard error ran from −0.22 with a truth wind under 3 m/s
    (9.8 ft/s) to +1.02 at 9 m/s (30 ft/s) and over (134 flights); on set A, from −0.20 to +0.16. The
    fit now moves its answer, to first order, to a calm and to a strong wind: the stated speed and
    1.645 of a component's standard deviations past it, 11.5 m/s (37.7 ft/s) at 18.3 m (60 ft) for
    the synthetic flights, past the 8.94 m/s (29.3 ft/s) they draw, grown with height by IEC
    61400-1's normal profile exponent of 0.2 (IEC 61400-1:2019, §6.3.2.2). The larger move either way,
    over $\sqrt 3$, joins each standard error in quadrature, as a uniform spread. It costs two flights
    a fit. A first try, the shear alone at the stated wind, moved set A's drag only to 24 below.
    After it, every parameter passes on both sets.
13. **The reviews' smaller corrections.** A barometer's Mach 0.8 cut is taken again from the flight
    each pass finds, not only the stated one. The port's coefficient is stated from 0 to 0.02 as the
    logger draws it, centered on 0.01, not on 0. A log of height alone is flown in a wind of the stated
    range's RMS speed, 5.16 m/s (16.9 ft/s), since a wind of either sign lowers the top. A log whose
    climb would run past 1,000 s says nothing, so a clock that jumps can't ask for bins past memory.
    A track's latitude and longitude must share one clock, and a ground station's position isn't
    read as the rocket's.

## Evidence

[The report][report] holds the counts. Every parameter passes on both sets, every probe fails, drag,
impulse and wind are identified on set A, and pure noise leaves all five not identified: all 1,400
logs of noise say nothing, 400 turned away by the reader and 1,000 by the fit.

## What this leaves

- **Set A and B aren't untouched.** Decisions 1, 3, 4, 5, 6, 8, 11 and 12 followed their counts, and
  decision 9's 0.1 was set after its results were seen. Only
  the shares are fitted apart, on the calibration set. No set was run again on a new seed. A real flight, or a fresh
  set, is the only honest measure of this ([evidence row G38](../evidence.md#g38)).
- **The errors are mixed.** A flight whose heights fit worse has larger errors than its standard
  errors say, so the shares average over two kinds of flight. The report counts them apart; a pair of
  shares by residual is [#1108](https://github.com/nrdptel/fusionspace-eridanus/issues/1108). On set
  B, some designs fall well short of 90%: Prometheus holds 68 of 96 on drag and 127 of 161 on the
  north wind.
- **Set B's drag passes at the edge.** It has 29 above, the fewest that pass at 1,000, and 70 below,
  where 5 more would fail: its fit runs high by 0.005 on average. One flight the other way on another platform fails it.
- **Accelerometers are unused,** and the fit goes no further than apogee.
- **No surface.** It's a library function until a later increment.
