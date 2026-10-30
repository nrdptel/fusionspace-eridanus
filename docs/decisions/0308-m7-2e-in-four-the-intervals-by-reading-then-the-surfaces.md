# ADR-308: M7.2e in four: the intervals by reading, then the surfaces (2026-10-29)

- **Status:** accepted; splits [M7.2e][m7-2e] in four and records how a reading's 90% interval is made and tested; its *done when* is unchanged and met when e1 to e4 are; §3's withheld-reading check amended by M7.2e2 for the top speed past its transonic mark (Consequences)
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
    still fail it. Near the mark the rule lets through only the readings that came out low: of
    the two Calisto designs' 18 top speeds on A and B, 10 left the truth above the interval, the
    flattering side (#1061).
  - The flattering side ([ADR-257][adr-257] §2) is a reading too low for the top vertical speed
    (a fin's margin) and the mean descent rate (a descent gentler than it was) as well as the
    height; the times have none here.
    The report gives each reading's mean error with its sign. The top speed's is +1.2 and
    +1.3 m/s; the descent rate's −0.01 m/s on both sets, against errors up to 6.81 m/s. Both
    miss above their intervals on warm days as the height does (#1061), so the guide sends a
    margin and a landing's speed to the bound. The times' leans by logger are #1062.
- `cargo xtask synthetic` reads 4,000 more flights, about 100 s on an 8-core Mac (5.5 minutes in
  all, from about 4).
