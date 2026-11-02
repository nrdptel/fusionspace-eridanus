# ADR-316: M7.3b: the least-squares fit, what it solves for and how its intervals are held (2026-11-01)

- **Status:** accepted; carries out [ADR-314][adr-314]'s M7.3b under [ADR-257][adr-257] §1
- **Summary:** Levenberg-Marquardt fits drag, impulse, a constant wind and mass to a log, with the
  clock, height offset and drift, and a barometer's gain and port, as further unknowns. Each
  parameter's 90% interval passes on sets A and B. Those two sets were seen while the model's choices
  were made. A fit takes about a second. The wind is held only on logs with a track.

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
5. **The wind's counts stand on 1,500 track flights,** of which three on each set state no track,
   leaving 1,497. Its errors have long tails, so a narrowed interval moves few flights. On 1,010, the narrowed probe passed on set B: 71 below, 875 inside, 63
   above of 1,009. At 1,500 it fails, and every other band is tighter too, the wind's own test
   included. The done-when asks for at least 1,000. This adds power and doesn't touch a threshold, but
   it was done because the probe passed.
6. **Set B's truth is the same fit on noise-free samples.** Set B flies another model, so it has no
   true scale. The first truth, a fit reading every bin, left set B's errors larger than set A's for a
   reason in the definition. It's now the same fit, on the same bins, at the same noise, with mass held
   as flown.
7. **How fast a fit must be.** About 0.8 CPU-seconds a fit in the optimized profile and about 70 flights
   flown, measured on set A in development; set B's count, about 115, holds its truth's fit too. Writing
   the report takes 11 minutes on 10 cores. It runs in one Linux job on CI, its figures held loosely,
   and whole at the gate, on the platform that wrote the report, with calibration.

## Evidence

[The report][report] holds the counts. Every parameter passes on both sets, every probe fails, drag,
impulse and wind are identified on set A, and pure noise leaves all five not identified.

## What this leaves

- **Set A and B aren't untouched.** Decisions 1, 3, 4 and 6 followed their counts. Only the shares are
  fitted apart, on the calibration set. No set was run again on a new seed. A real flight, or a fresh
  set, is the only honest measure of this ([evidence row G38](../evidence.md#g38)).
- **The errors are mixed.** A flight whose heights fit worse has larger errors than its standard
  errors say, so the shares average over two kinds of flight. The report counts them apart; a pair of
  shares by residual is open as an issue, not done here. On set B, some designs fall well short of 90%:
  Prometheus holds 69 of 98 on drag and 112 of 161 on the north wind.
- **Accelerometers are unused,** and the fit goes no further than apogee.
- **No surface.** It's a library function until a later increment.
