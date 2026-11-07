# ADR-325: M7.3d2, apogee predicted from the boost on the public flights (2026-11-07)

- **Status:** accepted; carries out [ADR-321][adr-321]'s M7.3d2 and [ADR-257][adr-257] §3's report on the 7 public logged flights; `hpr_forensics::boost::predict` now says why it gives no prediction.
- **Summary:** The 7 public flights' logs are fitted whole and up to the boost's cut, in each flight's ERA5 air, the fit told the example's design and a wind of ERA5's averaged over the climb. Two flights get a prediction: on them it misses the logged apogee by 3.03% on average (−0.85% with its sign), where the forward simulation misses by 6.89%. Hartigan's exact sign-flip interval on the difference is unbounded on 2 flights, as it is below 6. On real barometers the cut can fall inside the burn: 4 of the 5 cuts do, Lince's prediction among them. That is measured and reported, not fixed here (#1307).

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-321]: 0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
[adr-323]: 0323-the-transonic-margin-warning-and-shorter-issue-lines.md
[i1307]: https://github.com/nrdptel/fusionspace-eridanus/issues/1307
[i414]: https://github.com/nrdptel/fusionspace-eridanus/issues/414

**Context.** M7.3d1 left `hpr_forensics::boost` able to cut a log just after burnout, as found
from the samples up to the cut, fit it, and fly the fit to apogee, tested on synthetic flights
only. M7.3d2's report sets that prediction against the 7 public flights' logs (the
[real-flights report](../../validation/reports/real-flights.md)), beside the forward simulation's
apogee, with an exact sign-flip interval on the difference. The fit needs a model of each real
flight to fly, something stated of each parameter, and an interval. The first run also showed
the cut falling early on real barometers.

**Decision.**

1. **The fit's flight is the forward simulation's.** `cargo xtask real-flights` builds each
   flight as it already does: the example's rocket on its thrust file, its rail and site, ERA5's
   air. The fit's scales act as on the synthetic flights: the impulse and burn time through
   `hpr_analysis::montecarlo::dispersed_motor`, the zero-lift drag and the structure's mass
   through the simulation's own scales. Flown in ERA5's own wind with nothing changed, it lands
   on the forward simulation's apogee: the run fails past 0.5 m, and measured, each of the 7 lands
   on it exactly.
2. **Its wind is constant with height, as the fit's model is** ([ADR-316][adr-316]). What's
   stated of it is ERA5's wind, averaged evenly in height over 100 levels from the pad to the
   forward simulation's apogee. Each other parameter's middle and spread are the synthetic
   flights' (`hpr_forensics::synthetic::fits::stated`): drag scale 1 ± 0.109, impulse scale
   1 ± 0.033, burn time scale 1 ± 0.055, each wind component ± 3.88 m/s, the structure's mass
   1 ± 0.011, as standard deviations. A constant wind moves a flight from the forward
   simulation's apogee by itself, so the report also flies the fit's model at what was stated,
   nothing fitted. On the two predicted flights that alone takes the mean absolute error from
   6.89% to 5.07%. The rest is the fit to the boost.
3. **The intervals.** A fitted parameter's 90% interval is its standard error times the shares
   fitted on the synthetic calibration set, as the fit's page states. The wind's on a log with no
   track gets none: those shares were fitted with the synthetic flights' own stated wind, toward
   the east at the range's RMS speed. On a flight's own stated wind they gave Bella Lui's east
   wind an interval of −11.7 to 2.2 m/s about 0.8 m/s.
4. **The difference's interval** is on each predicted flight's absolute error from the
   prediction less the forward simulation's, in percentage points. It is the exact inversion of
   the sign-flip test of its center: J. A. Hartigan, "Using subsample values as typical values",
   *J. Am. Stat. Assoc.* 64 (1969), pp. 1303–1317, Theorem 1. From the k-th smallest of the
   2ⁿ − 1 subset means to the k-th largest holds the center with probability 1 − 2k/2ⁿ for
   differences drawn symmetric about it. k is the largest with that at least 95%. Below 6
   flights k is 0 and the interval is the whole line. `boost::sign_flip_interval` computes it.
   One test checks that it is the test's inversion, every sign pattern counted. Another checks
   it covers 95.3% within 0.8 points on 20,000 draws of 7. Both fail when it is narrowed by one
   subset mean a side.
5. **The early cuts are measured, not fixed here.** The finder cuts at the first sample where
   the half-second fitted climb speed falls 10% and 10 m/s below its top. On Juno III, Prometheus
   and Lince it cuts 0.80, 1.79 and 2.40 s after liftoff, against thrust files that burn 5.69,
   4.88 and 4.67 s. Bella Lui's is 2.25 s against 2.43 s, at the burn's end at most. Timed with no
   cut taken, a real barometer's fitted speed falls past the threshold for up to 0.48 s and
   recovers (Juno III 0.40 s, Lince 0.48 s). Making the fall hold longer than that doesn't fix
   it: NDRT 2020's coast shows falls of 0.35 to 0.45 s, and between them its fitted speed climbs
   to a new top 4.85 s after liftoff, against a 3.43 s burn. A hold long enough to pass Lince's
   dip would pass over all three, so NDRT's cut would come more than 6.0 s after liftoff. The fix needs its own test on logs with such dips ([#1307][i1307]). Until then the
   report lists each cut that comes less than its thrust file's burn after liftoff. A prediction
   from such a cut is counted as the library gives it, and the report says so.
6. **Why there's no prediction.** `boost::predict` returns `Unpredicted` rather than `None`: no
   cut (and `boost::try_cut`'s reason: no track, a GNSS track, too fast or too sparse a log, no
   liftoff, the transonic limit with the speed and time, or no fall), too little before the cut
   to fit, a fit kept to what was stated (with its misfit share), or no apogee. The prediction
   also carries its apogee as the log's sensor reads it (`apogee_reading_m`), which is what the
   log's apogee is set against. The synthetic cut test now checks that a GNSS log is refused as
   one, and every other uncut log for its speed.

**The result** (`validation/reports/from-boost.md`). NDRT 2020: predicted 1,269.2 m against the
log's 1,320.4 m, −3.88%, where the forward simulation reads +10.40% and the stated model +6.57%.
Lince, from a cut inside its burn: +2.19% against +3.39%. Cavour and Genesis are refused at
307.0 and 229.0 m/s; Bella Lui, Prometheus and Juno III are cut too soon to leave the 12 bins the
fit needs. The whole-log fits come back for 6 of the 7; Juno III's log has no apogee the reader
will take (a spike at its top). The forward simulation on all 7 stays 6.06% (−0.22% with its
sign).

**Consequences.** The report runs with `cargo xtask real-flights` and its `--check`, where
`refs/` holds RocketPy's checkout. CI holds the page to the committed data, the summary to the
rows, and the forward simulation's numbers to the real-flights report's. Two predictions bound
nothing at 95%; M7.3d3's collection is where the comparison can say something, and #1307 may
add or remove public predictions first. The fit's page gains a worked example on NDRT 2020. The
site's search index measured 9,991,414 bytes with it, 4,499 more than main's and 8,586 under
mdBook's warning ([ADR-323][adr-323]: until [#414][i414] lands, prose must fit).
