# ADR-300: The smoother and its check on the synthetic flights (2026-10-27)

- **Status:** accepted; re-lands [M7.2d][m7-2d] after [ADR-299][adr-299] withdrew it; records its smoother and how it is held to the synthetic flights' truth, where it departs from [ADR-280][adr-280]'s plan for it, and moves the health card's `hpr analyze` and Python surfaces to [M7.2e][m7-2e], whose *done when* now names the card beside the intervals
- **Summary:** `hpr_flightdata::smoother` fuses a log's barometer, accelerometer and GNSS receiver in a Kalman filter run forward and a Rauch-Tung-Striebel pass back, and gives the height and vertical speed at every sample with a standard error. `cargo xtask synthetic` holds it to the truth of the three synthetic sets at burnout, the top speed, apogee and a drawn moment, and fails unless every cell holds on every set and logger, the pass back solves every step, and trusting the GNSS receiver from liftoff fails on both test sets. Its 90% intervals hold on all three sets: between 94.3% and 99.4% of the truths lie inside each set's cells.

[m7-2d]: ../decisions-and-roadmap.md#m7-2d
[m7-2e]: ../decisions-and-roadmap.md#m7-2e
[adr-299]: 0299-withdraw-the-smoother-to-land-it-in-order.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[smoothing]: ../research/altimeter-smoothing-methods.md
[mach]: ../research/altimeter-mach-error.md
[gnss]: ../research/gnss-in-boost.md
[noise]: ../research/altimeter-sensor-noise.md
[report]: ../../validation/reports/synthetic-flights.md
[hprcode]: https://www.nar.org/safety-information/high-power-rocket-safety-code/

**Context.** [ADR-280][adr-280] planned M7.2d as "a linear RTS smoother over height, speed, the
accelerometer's bias and the barometer's correlated error, the accelerometer as its input", with
GNSS fused at zero weight until it agrees with the barometer's speed, the barometer de-weighted
above Mach 0.9 and after each charge, a χ² gate per sensor, and liftoff from the accelerometer.
M7.2d is done when that smoother "holds the test sets' truth in its stated error, coverage per
state reported; trusting GNSS from liftoff fails; a health card."

Built and run on the synthetic sets (tuning 200 flights, A and B 1,000 each), the plan as written
fell short in five places:

- **An input accelerometer** makes the accelerometer's errors the model's own and leaves a log
  with no accelerometer, or a clipped one, with no model at all.
- **Learning the day's air** from the barometer and accelerometer alone failed. With the air's
  share of the height as a state, the smoothed heights fell to 0.24–0.3 of the truth on the
  tuning set: the barometer can't tell the air from a trajectory scaled to fit it.
- **Learning the climb's port coefficient** failed too. On the barometer-only loggers its
  estimate came out biased by +0.010 while claiming a standard deviation of 0.001; with an
  accelerometer it claimed 2.4 times too little spread.
- **A linear barometer row** can't carry the port's error, `Cp·q/(ρg)`, which grows with the
  speed squared.
- **Without events,** the charges' pulses go unmarked. The synthetic logs name no events, as many
  real ones don't, so the apogee charge's pressure pulse went straight into the fit.

**Decision.**

1. **Twelve states,** on the barometer's own scale: the height, speed and acceleration (a
   white-jerk model that the accelerometer measures, as `a + g` along its axis); the
   accelerometer's bias, scale and a slowly varying model error for the axis's tilt; the
   barometer's slow correlated noise, its drift and the drift's rate; the day's air; the GNSS
   receiver's offset; and the port's coefficient in the fall. The update is Joseph's form, valid
   for any gain.
2. **The day's air** is measured only through the GNSS receiver's geometric heights. Without a
   receiver it is held at its prior, as a Schmidt consider state, and its spread is carried
   into each output.
3. **The climb's port coefficient** is held at its prior, −0.01 ± 0.02. It enters by a consider
   analysis: the log is smoothed again with the coefficient a standard deviation off, and each
   standard error is widened by the difference, `σ ← hypot(σ, x_off − x)`. The second run
   replays the first's choices: each step's phase, the transonic de-weighting, the GNSS trust
   and each sample's fate at the gate. Without the replay, the two runs' differences were
   branch differences. On an IMU-500 tuning flight the second run kept a transonic span closed
   where the first opened it; a second before burnout its speed differed by 115 m/s, and that
   spread widened the interval. The pass back zeroes the held states' rows of its gain, as the
   filter zeroes theirs. The covariance it carries back is the textbook one, derived for the
   optimal gain, so for those rows it is HPR Sim's approximation; no source for a consider
   smoother's was found, and the coverage on the synthetic sets is its only check.
4. **The port term is evaluated at the barometer's own rate,** a least-squares slope over a quarter
   of a second either side, so the barometer's row stays linear in the states. Evaluated at the
   filter's own speed, a speed gone wrong fed the squared term and the filter ran away. On the
   tuning set's barometer-only flights this produced standard errors up to 10⁸ m, and on one
   flight an apogee 140 km off. Six flights over the tuning set and A had steps the pass back
   couldn't solve. After the change, none in any set has.
5. **The fall's port coefficient** is learned from apogee on: a falling rocket's attitude isn't
   the climb's.
6. **Burnout** is taken once the climb surely slows: its acceleration below −g/2, or its speed
   past its peak, each by two standard errors, or the barometer's rate a tenth below its peak
   outside a transonic span. **Apogee** comes when the speed plus two standard errors and the
   barometer's rate are both at or below zero. On the speed alone, a barometer-only tuning flight
   took apogee 21 s early, while still climbing at about 260 m/s. From apogee on, the
   accelerometer is set aside and its states held.
7. **The barometer is de-weighted** from Mach 0.8, estimated high, rather than ADR-280's 0.9. That
   puts a margin below the Mach number where [the Mach-error note][mach] has the port's
   coefficient start to move. A span opens when the filter's speed and the barometer's rate
   both say transonic, and closes after the rate has said subsonic for a second. A charge's span
   is the event's 0.5 s for each charge the log names, plus one around the barometer's own
   apogee in every log: from where its running median first comes within 5 m of its top to
   0.5 s after it last is. 5 m is about a second of a coast either side of apogee. Set A's
   apogee-speed cell failed without that span: 65 of 820 truths lay above the interval, and the
   misses rose with the pulse's size. With it, 18 lay above and 1 below; after the later rules,
   the report prints 27 above and 5 below, and the cell holds.
8. **The landing** is where the barometer's running median settles within 5 m for 3 s. From it
   the speed and acceleration are widened by the impact, and the acceleration is given the
   powered jerk. With only the descent's jerk, a ballistic impact at 180 m/s left a tuning
   flight's filter rejecting 507 samples in the second after it.
9. **Each sensor is gated** at 5σ, and a sensor rejected for a second is taken back, widened by
   its gap. **GNSS** is fused at zero weight from liftoff until its speed has agreed with the
   smoother's for 2 s, then with its variance inflated for 5 s ([GNSS in boost][gnss] §5). Its
   fixes after the barometer's last sample are dropped: there the track would be the motion
   model's alone.
10. **The barometer's noise in flight.** Past the barometer's own top and the charge expected
    there, each sample's noise also holds the log's own scatter: the robust spread of the heights
    about a least-squares quadratic through their running medians over three seconds either side,
    less the pad's noise and the fast and slow correlated noise the pad's was taken less of. The
    scatter beyond those is fused as an error correlated over 2 s. Under a canopy, a swinging
    rocket and the gusts on it scatter a barometer's readings by 0.3–7.5 m about two-second fits
    on public pairs' descents ([the sensor-noise note][noise] §3), far more than a pad shows. A
    pendulum on 3–15 m of cord and lines has a time constant `√(L/g)` of 0.55–1.24 s; 2 s is taken
    because the quadratic takes part of a correlated error's variance out. In the tests, descents
    swinging by 3–6 m correlated over up to a second keep 91.5–100% of their heights inside the 90%
    intervals, and one correlated over 3 s keeps 74%: a barometer alone can't tell a slow swing
    from motion. Swings correlated over 1–1.24 s, the top of the pendulum's range, aren't
    tested. Before the top the scatter is left out: a six-second quadratic reads liftoff's
    and burnout's bends as noise.

    This was found on a public forum log, a barometer's track with a GNSS receiver's, cached under
    the ignored `refs/research/` as its license is unclear, so these figures can't be rerun from
    the repository. Its pad was supplied as a constant reading, as a logger reading whole feet at
    rest gives. Weighed by the pad's noise alone, 960 of its 2,704 barometer samples failed the
    gate, the filter reset 40 times, and its apogee came out 1,630 m, standard error 58 m, against
    the barometer's highest reading, 1,407 m. With the scatter, 8 samples are rejected, all at the
    ejection's onset, and the apogee is 1,408 m, standard error 43.5 m.

    Two more guards came from review. The rates the port's term and the transonic span read are
    the running medians' slopes, and the air that thins the barometer's noise is taken at the
    running median's height: one garbled sample, of 2 km or 10⁷ m, had bent its neighbors' rates
    and put its own noise in no air, and the filter lost the track. An estimate that leaves the
    finite numbers anywhere withholds the log (`Reason::Diverged`) rather than return it.
11. **The check.** Per set, state and moment, each side's misses of the 90% interval are counted.
    A cell holds when the lower edge of the two-sided 99.9% Clopper–Pearson band on each side's
    miss share is at most 5%, so the flights don't show it missing more often than it says it
    does. The report gives each cell overall and per logger, and each cell's median and largest
    standard error overall (per logger in its JSON), so an interval that holds only by being wide
    shows. `cargo xtask synthetic`
    fails if any cell fails on any set, overall or for a logger, or if any step goes unsmoothed,
    or if a flight's smoothing panics; a moment the smoothed track doesn't reach misses on both
    sides. A log the smoother withholds for any reason but having no barometer, and a flight
    that isn't flown, fail it too, since either would leave every cell and take its misses with
    it (added in the re-land's review; none occurs on the three sets). It also fails if trusting
    GNSS from liftoff holds on A or B. The tuning set's 200 flights are too few to require that;
    its result is reported. The health cards' sums are held to three significant figures, as the
    census's values are ([ADR-280][adr-280]); a sum under 1,000 keeps every digit. A log ends a
    drawn time after the simulated landing. CI's Linux runners logged one tuning flight a sample
    shorter than macOS and Windows did: the set's barometer samples came to 4,859,243 against
    4,859,244, its accelerometer's to 4,444,351 against 4,444,352. For a sample to cross the
    log's end, the landing must differ by platform by far more than its last bits; by how much
    hasn't been measured. So the report no longer shows a change of a few samples in millions.
    Two exposures remain. The cells' counts of truths stay exact, and a drawn moment moves with
    the landing time. And the tuning set's 4,446 rejected barometer samples, held as 4,450, lie two
    above the count held as 4,440. If either differs by platform, the fix is to find the landing's
    spread, never to loosen a cell.
12. **The health card** is `Smoothed::health`, written out by `Health::card`. It holds each
    sensor's samples, fused, rejected and not weighed, its resets, its share inside the gate, its
    mean NIS and largest test ratio; then the de-weighted spans, when GNSS was first trusted, the
    barometer's height less the accelerometer's at apogee, and the steps the pass back kept
    forward. The report sums the cards per set and prints one per logger. Its surfaces in
    `hpr analyze` and Python come with M7.2e, whose *done when* now names the card beside the
    intervals.
13. **What was seen before the rules were set.** The counts above from before a rule was taken
    up, and the before-values under Consequences, were measured while it was built and can't be
    rerun from the repository; the report holds the after-values. The rules were found on the
    tuning set. A and B
    were run as each change landed, and three changes were found by tracing flights that failed
    there: the apogee rule, the port term at the rate and the landing's jerk. The flight's own
    noise came from the real log in item 10, and A and B were run after it. Its first form,
    which held the slow correlated noise twice, failed B's per-logger cell for the GNSS logger's
    apogee height (21 of 186 truths below), once each logger's cells were gated; taking the slow
    noise out, as the pad's estimate does, set it right, though only just: 19 of 186 lie below
    now, and the band's lower edge is 4.3%, under the rule's 5%. For the apogee
    charge span, A's failing apogee-speed cell came first, and B's was seen at 62 of 845. So A
    and B are no longer untouched for those cells. Each rule is physical, not fitted: of
    the new constants, the span's 5 m comes from a coast's shape, and the swing's 2 s from a
    pendulum's, with the scatter's window long against it; the rest reuse the smoother's own.
    M7.2e's test of every reading starts from the same sets.

**Consequences.**

- On B, the median standard errors are 59 m of height and 11 m/s of speed at burnout, 84 m and
  1.2 m/s at apogee, and 18.5 m and 1.0 m/s at a drawn moment. The day's air, held at its prior
  without a receiver, and the port's spread are expected to be their largest terms.
- The widest intervals are on barometer-only loggers, up to about 410 m at burnout and 260 m/s
  at a drawn moment.
  They hold the truth, but say little there.
- Trusting the receiver from liftoff fails on A and B at apogee and at a drawn moment, where the
  lagging fix pulls the height. With B's 186 smoothed logs that have a GNSS track trusted from
  liftoff, and its other flights as before, 146 of its 845 truths at apogee lie below the interval.
- `cargo xtask synthetic` takes about four minutes on an 8-core Mac, up from 101 s, as each flight
  is smoothed twice and each GNSS log twice more.
- The flight's own noise widens B's barometer weights past apogee where its loggers scatter more
  than their pads: barometer samples rejected fell from 29,500 to about 13,400 of 20.6 million, resets
  from 90 to 35, and every cell's coverage stayed within the check.
- The smoother's page, [The smoother](../physics/smoother.md), put the site's search index 23 KB
  past mdBook's 10 MB limit. The evidence register's page leaves the index, as earlier pages that
  restate others have (`book.toml`): its rows restate the gaps the guide's pages state, and each
  of those links its row. The index is then 9.87 MB. The lasting fix is still
  [#414](https://github.com/nrdptel/fusionspace-eridanus/issues/414).
- Real logs are untested against a truth: the one public log of item 10 shows the smoother stays
  on its barometer, not that its intervals hold. A port that reads the flow differently from the
  synthetic loggers', or a charge away from apogee that the log doesn't name, can leave the truth
  outside more often than the interval says. The health card shows where. A log that starts at
  launch, as that one's logger writes them, has no pad to zero on and is withheld.
