# ADR-308: M7.2e in four: the intervals by reading, then the surfaces (2026-10-29)

- **Status:** accepted; splits [M7.2e][m7-2e] in four and records how a reading's 90% interval is made and tested; its *done when* is unchanged and met when e1 to e4 are; §3's withheld-reading check amended by M7.2e2 for the top speed past its transonic mark, §3's floors and §5's one look by M7.2e3, and #982's zero by M7.2e4 (Consequences)
- **Summary:** each reading's 90% interval runs from the reading less one share of its own bound to the reading less another. The two shares are the 5% and 95% points of the readings' errors as shares of their bounds, fitted on a calibration set of 2,000 synthetic flights on a seed of its own. `cargo xtask synthetic` fits them again, holds them to the library's, and runs [ADR-257][adr-257]'s test with its three probes on sets A and B. M7.2e1 does this for the apogee's height and time; e2 and e3 add the other readings; e4 takes the intervals and the smoother's health card to `hpr analyze` and Python.

[m7-2e]: ../decisions-and-roadmap.md#m7-2e
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-300]: 0300-the-smoother-and-its-check.md
[report]: ../../validation/reports/synthetic-flights.md#the-readings-90-intervals

**Context.** M7.2e is done when "ADR-257 §1–2's test passes, probes failing, on both test sets
for every reading given; intervals in the library, `hpr analyze` and Python, with the smoother's
health card ([ADR-300][adr-300])." On 2026-10-29 none of that existed:

- the readings (liftoff, the apogee's height and time, the top speed, landing, the flight time,
  the mean descent rate, the top acceleration, burnout, each canopy's rate and the main's
  opening) stated bounds held on the tuning set, and no interval;
- nothing ran the readings on sets A and B, and nothing held a count to ADR-257's two-sided
  test or ran its probes;
- `hpr analyze` and Python print the readings and bounds, but not the smoother or its card, and
  the smoother withholds a log that starts at launch ([#982][i982]).

That is too much for one session. The smoother's own intervals don't serve: they hold the truth
94.3% to 99.4% of the time in [ADR-300][adr-300]'s set cells, wider than 90% (two logger cells
fall just under, at 89.2% and 89.7%), and lopsided (A's
height at burnout: 18 truths below, 1 above). ADR-300's check only asks that they not miss more
than they say; ADR-257's test fails them. They also cover only height and speed, not a time, a
rate or an acceleration.

[i982]: https://github.com/nrdptel/fusionspace-eridanus/issues/982
[i1070]: https://github.com/nrdptel/fusionspace-eridanus/issues/1070
[i1071]: https://github.com/nrdptel/fusionspace-eridanus/issues/1071
[i1073]: https://github.com/nrdptel/fusionspace-eridanus/issues/1073
[i1074]: https://github.com/nrdptel/fusionspace-eridanus/issues/1074
[i1076]: https://github.com/nrdptel/fusionspace-eridanus/issues/1076

**Decision.**

1. **One rule for every reading.** A reading's bound already scales with what drives its error:
   the apogee height's bound is mostly the day's air, 6.92% of the height. The interval runs from
   `reading − high · bound` to `reading − low · bound`, where `low` and `high` are the 5% and 95%
   points of `(reading − truth)/bound` over a calibration set. The point from each end is the
   `⌊0.05 (n + 1)⌋`-th of the `n` sorted shares, the split-conformal rank (Vovk, Gammerman and
   Shafer, 2005; Lei et al., *JASA* 2018). On flights drawn like the calibration set's, each tail
   then misses about 5% on average. The shares are written into `hpr_flightdata::readings::interval` to
   three significant figures, since a log read on its own has no calibration set at hand.
   `cargo xtask synthetic` fits them again and fails unless they match. Each tail gets its own
   share, so an error that leans one way gets an interval that leans with it.
2. **A calibration set of its own.** ADR-257 §1: "any factor that scales the intervals is fitted
   on other seeds." The tuning set's 171 barometer flights are too few. A share fitted on `n`
   readings has a tail miss rate spread by about `√(0.05 · 0.95/n)`, 1.7% at 171. Against the
   test's own 0.7% at 1,000, a correct interval would often fail. `Set::Calibration` draws 2,000
   flights as the tuning set's are, alternating set A's model and set B's, on seed `2026102900`,
   fixed on 2026-10-29 before any reading's interval existed. The 2,000 flights give 1,638 apogees, and at 1,638 the fit's spread is 0.54%.
   The census and the smoother's check don't fly it. Changing its seed after a failure needs a
   decision record, as for the other sets.
3. **The test.** Each flight's own log is read. A reading that states an interval is counted
   below, inside or above it; a truth that isn't a number misses on both sides. Each count's
   two-sided 99.9% Clopper–Pearson band must hold 5% for each tail and 90% for the inside count.
   An empty count holds nothing. The test counts the readings given, not the flights flown:
   set A's 180 and set B's 155 logs from the GNSS-only tracker withhold the apogee, so A compares
   820 readings and B 845. At 820 a tail passes with 22 to 63 misses. The probes narrow every
   interval to 0.8 of its width about its middle, widen it to 1.25, and shift it down by one
   stated standard error (its width over `2 × 1.645`). Each must fail. A floor (a reading the
   truth may lie any amount above) states no interval in e1 and is counted apart. Counts are
   reported per design, logger and band of the day's air, and the apogee height's mean error
   with its sign (§2). The calibration set alternates set A's model and set B's, as the tuning
   set does, so B is held apart by its seed, not by its generator. The command also fails if a
   reading is withheld from a log with a barometer (but see Consequences, M7.2e2: the top speed
   past its transonic mark), or read with no interval, on any of the
   three sets: either would leave the counts with its misses.
4. **The split.** Each increment's *done when* fails on main today:
   - **M7.2e1 The test, on the apogee.** `cargo xtask synthetic` fits each interval's shares on a
     calibration set of its own seed and fails unless they equal the library's. On A and B, the
     apogee's height and time pass ADR-257 §1's test, its three probes failing. Counts per design
     and the height's signed mean error (§2) are reported, and the intervals are in the library.
   - **M7.2e2 The height track's other readings.** Liftoff, the top speed, landing, the flight
     time and the mean descent rate state intervals in the library that pass e1's test on A and
     B, probes failing.
   - **M7.2e3 The accelerometer's readings and the canopies'.** The top acceleration, burnout,
     each canopy's rate and the main's opening state intervals that pass e1's test on A and B,
     probes failing. A floor states a lower end, held to the test's lower tail with its probes.
   - **M7.2e4 The intervals in `hpr analyze` and Python.** Every interval is in `hpr analyze`'s
     text and JSON and in Python, with the smoother's health card ([ADR-300][adr-300] item 12).
     A log that starts at launch is smoothed on an assumed pad, and its card says so ([#982][i982]).
5. **What was seen before the shares were set.** The shares were fitted once, on the calibration
   set, with the library's set to zero. Sets A and B had been read only with those zero shares,
   where every truth misses. A and B were then run once with the fitted shares, and nothing was
   changed after.

**Consequences.**

- On the calibration set's 1,638 apogees, the height's shares are −0.63 and 0.637 and the
  time's −0.337 and 0.225. The time's errors reach further early than late. On the [report][report]'s
  A and B:
  - the height's interval leaves 55 and 45 of A's 820 truths below and above it, and 38 and 47
    of B's 845;
  - the time's leaves 36 and 32, and 41 and 50;
  - every probe fails: narrowed, 228 to 264 truths miss the height; widened, none do; shifted
    down, 283 lie above.
- The apogee height's mean error is +1 m on A and +0 m on B (under half a meter, not below zero),
  so the reading is not low on average: the flattering side ([ADR-257][adr-257] §2) shows no low
  bias. The report gives the mean to the meter, the figures of the largest error (237 m), since
  its later digits differ from platform to platform, and with its sign.
- The interval holds 90% over the flights the synthetic sets draw, pooled over their loggers,
  designs and days. The misses aren't pooled. The height's all fall on days more than 10 K from
  the standard atmosphere: none of the 551 and 526 days within it on A and B missed. Of the
  warmer days, 45 of 137 on A and 47 of 156 on B read low enough to leave the truth above the
  interval, the flattering side. The time's interval misses late on the AltOS logs, whose rate
  drops at apogee: 27 of 185 on A and 30 of 186 on B. Those groups alone fail the test. Found in
  review, both are now in the report (*By logger and by the day's air*) and on the guide's page,
  which sends a waiver to the bound. The fixes are new work: #1055 conditions the height on a
  given day's air, and #1056 treats a rate that drops at the top. No real flight has checked an
  interval: the evidence register's [G34](../evidence.md#g34) records the gap.
- **M7.2e2 (2026-10-29).** Liftoff, the top speed, landing, the flight time and the mean descent
  rate took the same rule, their shares fitted once on the calibration set with the library's at
  zero, as §5 did. Liftoff's are 0.203 and 0.482, both above zero, as it is read late; the
  flight time's are −0.381 and −0.133, as it reads short. On A and B every one passes the test
  and every probe fails it ([report][report]). Two things were decided:
  - The top speed is withheld at or above 227.7 m/s, where the barometer's port may read the
    transonic flow, on logs with a barometer too. §3's check would fail that, so the one reading
    withheld by its own rule is the top speed for that reason (`withheld_by_rule`); any other
    withheld reading still fails the command. The rule looks at the reading, not the truth, so
    the test holds the top speeds a user is given: 577 on the calibration set, 287 on A and 286
    on B. At 287 a tail passes with 4 to 28 misses, a looser test than at 820; the three probes
    still fail it. Past the mark the rule lets through only the readings that came out low: of
    the two Calisto designs' 18 top speeds on A and B, 10 left the truth above the interval, the
    flattering side (#1061).
  - The flattering side ([ADR-257][adr-257] §2) is a reading too low for the top vertical speed
    (a fin's margin) and the mean descent rate (a descent gentler than it was) as well as the
    height; the times have none here.
    The report gives each reading's mean error with its sign. The top speed's is +1.2 and
    +1.3 m/s; the descent rate's −0.01 m/s on both sets, against errors up to 6.81 m/s. Both
    miss above their intervals on warm days as the height does (#1061), so the guide sends a
    waiver and a fin's margin to the reading plus its bound, and a landing to the main's rate
    and its bound. The times' leans by logger are #1062.
- **M7.2e3 (2026-10-30).** The top acceleration, burnout, each canopy's rate and the main's
  opening took the same rule, their shares fitted once on the calibration set with the library's
  at zero. Liftoff's lean is the main opening's too (0.138 and 0.566: it is read late). On A and B
  every one passes the test and every probe fails it ([report][report]). Decided:
  - **A floor states a lower end, tested on the lower tail.** It is `reading − high · bound`, where
    `high` is the 95% point of the floor's own shares. Only the band on the truths below must hold
    5%. Its standard error is `(high − median) · bound / 1.645`, and the probes raise it by one and
    lower it by two: lowered by one, too few truths move for a band of a few hundred floors to
    fail (of 210, 2 below still holds 5%). The two were set before A and B were read. Burnout's
    shares are −0.0339 and −0.195; 12 of 210 and 6 of 211 truths lie below.
  - **A clipped top is fitted where the part's scale decides the clip; the first rule failed.** No
    flight's own accelerometer clips, so each flight's log is written again clipping at 8 g. The
    first fit, on tops as flown, gave a 95% share of −0.785 and a lower end above the reading. On A
    it held (8 of 252 below). On B it failed: 31 of 285, its band 5.71% to 18.2%. The cause is not
    chance. How far past a clip a top goes is the design's, and no log says: 21 of B's 22 Bella Lui
    floors, whose tops only just pass 8 g under B's model, lay below it, where A has none. So a
    lower end above the reading claims something the log doesn't hold.
    A second rule scaled each top to 0.95 to 1.05 of the clip (0.223 and −0.111; 15 of 224 and 19 of
    240). Review showed that window unsourced, its misses all from tops under the clip, its factors
    the same flight by flight on every set, and its coast read wrong after scaling. A third took the
    window from the part's scale error alone, `1/(1 + s)` to `1/(1 − s)` (0.212 and −0.134; 7 of 235
    and 10 of 258). Review showed that it left out the reader's lowest floors, the likeliest to miss.
    The rule that stands is every top the reader can call clipped and the part may not have clipped:
    from `(1 − (0.25 g + 0.05 g)/8 g)/(1 + s)`, the reader's threshold with the part's bias and gain
    at their limits, to `1/(1 − s)`, past which every such part clips. `s` is ±3% (the MPU-6000's data
    sheet) and ±10% (the ADXL375's). Each flight's top is scaled to a place of its own between,
    which differs by set, and only the force above 1 g is scaled, so the coast stays as flown. Its
    shares, on 348 floors, are 0.259 and −0.124; 3 of 181 and 3 of 187 lie below, all on the AltOS
    logs, and no logger's or design's count alone fails. That is under its 5%, the safe side, though
    the test's band holds it. A top truly past the clip lies above the reading: none of 252 and none
    of 285 lie below the lower end, a check close to certain by construction, whose band must reach
    down to 5%, and which fails on fewer than 149 tops or none. **What this pass is worth:** A and B
    were read under all four rules, so the new places on them are unseen draws but the flights aren't;
    only this floor's rule changed after a look at A and B. The result moves with the window (probes
    not in the report): at 0.90 to 1.10 of the clip the 95% share is 0.236, and at 1.00 to 1.10, with no
    top under the clip, −0.0496 (none of 295 and 314 below, and the test fails). A clipped reading
    supports a statement about the part's error only.
  - **A lower end only on a clipped top.** A floor whose burn the log ends in isn't clipped and its
    errors are a whole top's: cutting a channel in the burn left 17% to 28% of truths below a lower
    end fitted on clipped tops (review probes, not committed). It states none, as an apogee floor
    doesn't, since no set reads one to test ([#1071][i1071]).
  - **The AltOS part clips at 8 g here too**, though it has no such setting, to reach the count: a
    test needs 149 floors before no truth below can fail it, and the IMU's alone give 107 and 108. The
    report says so, and shows each logger's and each design's count, in bold where that group alone
    fails. Tops read whole on these logs are out of the floors' counts and printed beside them.
  - **A burnout's lower end lies after the reading** (−0.0339 of the bound, about 0.045 s), the
    same kind of claim. It stays, being small beside the bound and passing on A and B, but it leans
    by design: 9 of 61 Bella Lui truths on A lie below it, 4 of 63 on B, and neither group alone fails
    ([#1070][i1070]).
  - **Withheld by rule, now three:** the top speed past its mark, a burnout taken from it, a top
    acceleration on a log with no accelerometer. A drogue or an opening on a flight that opened one
    canopy is none to read, counted apart; a main named on such a flight has no truth and misses
    both ways; a flight with two canopies read as one would fail the command. None did.
  - **Flattering side:** a low top acceleration flatters the load, and a low canopy rate the descent,
    as the mean rate does. The guide's "reading plus its bound" for a load excludes a floor, which
    has no upper figure. The canopies' rates miss on warm days as the height does. The top
    acceleration leans by logger: outside the interval 34 of 185 and 38 of 186 on the AltOS logs, 22
    and 20 of them above it; none of 171 and 182 on the IMU's ([#1070][i1070]). The pooled test holds and
    the guide sends a load to the reading plus its bound.
- **M7.2e4 (2026-10-30).** `hpr analyze`'s text prints every interval after its bound, rounded
  outward, and a floor's lower end, rounded down; its JSON and Python already carried them. The JSON
  adds `smoother`, and Python `FlightLog.smoothed()`, with [ADR-300][adr-300]'s card. A count over each
  fixture log that reads with no file beside it holds the text's intervals to the JSON's. Decided:
  - **A log with too little of the pad is smoothed on an assumed pad.** The smoother had withheld
    it, and that was most of the logs it could read: of the 1,058 sample records, 379 smooth this way
    and 175 on their own pad (`validation/reports/log-import.md`, *The smoother*).
  - **The height stands on the readings' zero, not the first sample** as [#982][i982] proposed. The
    zero is the logger's, set on the pad, where a first sample may be up the climb. It also keeps the
    smoothed apogee on the same scale as the reading's. A zero that is the pad's only by a word adds
    `PAD_ZERO_M/√3` to the height's standard error, through the drift state. A logger's own zero adds
    nothing, as in the readings' bounds, though weather on the pad can move one, on the low side too;
    the cut logs below keep a true zero, so nothing tests that.
  - **The noise is the flight's own scatter,** the median past apogee at the pad's air, less the
    correlated noise. It errs noisy, since a swing is in it.
  - **The first speed's prior is about zero, not the barometer's rate.** Taken from the rate, a
    public log's garbled second sample set the track off at 29 km/s: a running median at the log's
    edge has too few samples to set a sample aside.
  - **The first second is settled.** Review found that priors this wide take a sample 10 m off as the
    truth: 10 m on the first sample of a 588 m flight read an apogee of 655 m and a speed of
    1,164 m/s. Over the first second, at least ten samples, each sample fused is the median of the
    five about it, which leaves a climb as it was. The first two aren't weighed. Sweeping errors of
    10 m to 1,000 m over the first three samples now moves the apogee by under a standard error. Two
    samples together 1,000 m high still move the readings' own pad ([#1076][i1076]).
  - **An accelerometer with too little of the pad is set aside,** and the card says so: 135 of the
    1,058 records. Zeroing one without a pad is new work ([#1074][i1074]).
  - **`cargo xtask synthetic` holds it.** Each flight's log is cut at the truth's liftoff, every sensor,
    and smoothed again. Any cell, overall or for a logger, that fails [ADR-300][adr-300]'s rule fails
    the command, as does a log withheld that the whole log wasn't, a step left unsmoothed or a log not
    on an assumed pad. All hold on the three sets. ADR-300's rule is one-sided and a logger's cell
    holds few flights, so a thin cell can hold at 87.9% (the tuning set's AltOS burnout height).
  - **The card says when the smoothed apogee falls outside the reading's bound,** a floor's only from
    below: 12 of the 379 on an assumed pad, 20 of the 175 on their own, and 0 to 2 of the synthetic
    sets' cut logs, so a log's own pad doesn't prevent it ([#1073][i1073]).
  - **A log of more than 2,000,000 samples is withheld** (`Reason::TooLong`), before any work: a file
    could otherwise ask for memory without limit. A first limit of 500,000 withheld the synthetic
    sets' long 500 Hz logs, and the command's check caught it. Review's burst of 900,000 samples 1 ns
    apart ran for minutes in the scatter's window, so the scatter and landing no longer walk a window
    for each sample, and a running median's window is capped. Under the limit, a 39 MB file of 1.95
    million steps took 27 s and 3.4 GB, with no way to skip it yet. The smoother also no longer
    withholds for a short pad, so its own `ShortPad` is gone; the readings keep theirs, and a track
    of pressures with no pad still gives no liftoff to smooth from.
- `cargo xtask synthetic` reads 4,000 more flights, and from M7.2e3 two more logs of each one with
  an accelerometer: about 7.5 minutes in all on an 8-core Mac, from 5.5.
