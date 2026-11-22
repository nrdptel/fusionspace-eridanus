# ADR-371: M1.14b6, wind from the GPS descent: the method's settings and what is compared (2026-11-22)

- **Status:** accepted; carries out [ADR-351][adr-351] decision 2's line for M1.14b6
- **Summary:** The wind is read from a GPS-tracked descent as the ground velocity plus `w/g` times its rate of change, from a quadratic fitted over 10 s around each fix, and only where the body falls steadily at 2 to 35 m/s (7 to 115 ft/s). Of the seven public flights, two are compared with ERA5, one is reconstructed without its day's ERA5 file, one fell too fast for any canopy and three logged no position. With fewer than three compared, the validation plan calls it a method, not yet a comparison. Prometheus's two receivers give the method's own noise on a real descent: 0.65 m/s (1.5 mph) RMS.

[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[note]: ../research/wind-from-descent.md
[report]: ../../validation/reports/descent-wind.md
[page]: ../physics/wind.md#wind-from-a-gps-tracked-descent

**Context.** ADR-351 asked for a documented reconstruction, the GPS ground velocity plus `w/g`
times its rate of change (Hock and Franklin 1999), pinned on synthetic descents at 5 and 25 m/s
through a wind that changes by 2 m/s per 100 m, within 0.3 m/s RMS at both, with the ground
velocity alone failing that line at 25 m/s; then run on every public flight whose log holds a
GPS track of the descent and set beside ERA5 by height band. The [research note][note] gives the
method and its bounds but no settings: how the velocity and its rate are taken from fixes, which
fixes are a steady fall, and what a log's repeated or frozen fixes mean.

The public flights are the seven the real-flights report flies, RocketPy's example rockets with a
flight log. Four logs hold a position: Prometheus (two Altus Metrum receivers, 1 fix a second,
descent at about 28 m/s under a drogue and 7 m/s under a main), Juno III (a COTS GNSS log, 1 fix a
second, about 25 m/s under its drogue alone), Genesis (latitude and longitude only, 1 to 10 fixes
a second, with a barometric height in the flight log) and Cavour (latitude and longitude sent to
four decimals, about 11 m, beside a barometric height). Prometheus's ERA5 file is of the year after
its flight, as RocketPy's example flies it; Cavour fell at a median 75 m/s after apogee.

**Decision.**

1. **The fit.** At each fix, a least-squares quadratic in time over the fixes within 10 s
   centered on it: the slope is the velocity, twice the curvature the acceleration. A fit is
   kept only with at least five fixes reaching a quarter of the window on each side, so no
   velocity is extrapolated past a track's end or across a gap. 10 s spans about two and a half
   periods of a 4 s swing, the note's reading (§4) of GRUAN's filter for a radiosonde's swing.
   It differentiates positions rather than reading the receiver's Doppler velocity, as Hock and
   Franklin do, because none of the four logs keeps a velocity vector.
2. **A steady fall.** A fix is read where the fit gives a descent of 2 to 35 m/s and a vertical
   acceleration of at most 0.5 m/s². In a steady fall the descent rate changes only with the
   air's density, `w²/(2H)`, 0.04 m/s² at 25 m/s; an opening or a landing changes it by several
   m/s². 35 m/s is past the note's fastest drogue (30 m/s); the tests pin the method at 5, 25
   and 33 m/s. A faster fall, Cavour's, is not a canopy's, and the lag model is not shown to hold
   for a tumbling body: the flight is listed as not run, with its median rate.
3. **A new fix is the receiver's.** A receiver repeats its last fix between updates and after it
   loses lock. A row is a new fix only when the receiver's own values change: latitude and
   longitude, and its altitude where the track reads it; in AltOS's CSV, its GPS clock with the
   lock flag set. A barometric height that changes under a frozen position is not a new fix.
   Without this rule, Genesis's receiver, frozen for about 45 s under its drogue, read as a
   motionless rocket and then an 80 m/s jump.
4. **ERA5 as HPR Sim reads it.** Each compared flight's ERA5 wind is the one the real-flights
   report flies it in: its file and hour, read as a sounding, interpolated by components in
   height. Heights outside ERA5's levels are not compared. Heights above the pad come from the
   receiver's first fix where the track reads the receiver's altitude, and from the barometric
   log where it doesn't, the site's elevation added for ERA5.
5. **Prometheus: reconstructed, not compared,** since no ERA5 file of its day is pinned. Its two
   receivers are reconstructed apart and their winds compared at fixes within 0.5 s: both rode
   one rocket, so the canopy moved both alike, and their difference is the receivers' and the
   method's own noise.
6. **Bands** of 0 to 500, 500 to 1,000, 1,000 to 2,000, 2,000 to 3,000 and 3,000 to 5,000 m above
   the pad, pooled over the compared flights, each with its counts of flights and samples, the
   mean and RMS of the difference in speed and the RMS of the vector difference.
7. **Held.** `cargo xtask descent-wind --check` reproduces the report in the gate's `refs` step.
   In CI, a test recomputes its bands from its committed samples and its page from its data,
   checks it lists every public flight in order, and, while fewer than three flights are
   compared, that the validation plan's row says the reconstruction is a method, not yet a
   comparison.

**Results** ([the report][report]). Juno III (102 fixes under its drogue) and Genesis (56 under
its main) are compared: over 158 fixes the reconstructed speed is 1.39 m/s above ERA5's on average
and 3.96 m/s from it RMS, by band from 4.86 m/s below (500 to 1,000 m) to 6.19 m/s above (3,000 m
up). Prometheus's receivers agree to 0.65 m/s RMS over 169 fixes, 2.25 m/s at most.

**Consequences.**

- The reconstruction is a method. Two flights can't separate ERA5's miss at a launch field from
  a canopy's glide, which nothing bounds for hobby canopies ([the wind page][page]).
- A third public flight with a GPS-tracked descent and its own day's ERA5 file, or an ERA5 file
  of Prometheus's day, would turn it into a comparison by ADR-351's line; the test then stops
  requiring the plan's wording.
- M7.3's wind fit can take this reading of the descent as its data and ERA5 as its prior.
