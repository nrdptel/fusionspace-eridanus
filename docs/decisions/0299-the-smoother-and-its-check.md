# ADR-299: The smoother and its check on the synthetic flights (2026-10-27)

- **Status:** accepted; records [M7.2d][m7-2d]'s smoother and how it is held to the synthetic flights' truth, where it departs from [ADR-280][adr-280]'s plan for it, and moves the health card's `hpr analyze` and Python surfaces to [M7.2e][m7-2e], whose *done when* already names both
- **Summary:** `hpr_flightdata::smoother` fuses a log's barometer, accelerometer and GNSS receiver in a Kalman filter run forward and a Rauch–Tung–Striebel pass back, and gives the height and vertical speed at every sample with a standard error. `cargo xtask synthetic` holds it to the truth of the three synthetic sets at burnout, the top speed, apogee and a drawn moment, and fails unless every cell holds on both test sets, the pass back solves every step, and trusting the GNSS receiver from liftoff fails on both. Its 90% intervals hold on all three sets: between 95.4% and 100% of the truths lie inside each cell's.

[m7-2d]: ../decisions-and-roadmap.md#m7-2d
[m7-2e]: ../decisions-and-roadmap.md#m7-2e
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
   spread widened the interval.
4. **The port term is evaluated at the barometer's own rate,** a least-squares slope over half a
   second either side, so the barometer's row stays linear in the states. Evaluated at the
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
   misses rose with the pulse's size. With it, 19 do.
8. **The landing** is where the barometer's running median settles within 5 m for 3 s. From it
   the speed and acceleration are widened by the impact, and the acceleration is given the
   powered jerk. With only the descent's jerk, a ballistic impact at 180 m/s left a tuning
   flight's filter rejecting 507 samples in the second after it.
9. **Each sensor is gated** at 5σ, and a sensor rejected for a second is taken back, widened by
   its gap. **GNSS** is fused at zero weight from liftoff until its speed has agreed with the
   smoother's for 2 s, then with its variance inflated for 5 s ([GNSS in boost][gnss] §5). Its
   fixes after the barometer's last sample are dropped: there the track would be the motion
   model's alone.
10. **The barometer's noise in flight.** Each sample's white noise is the larger of the pad's,
    thinned with the air, and the log's own scatter about it: the robust spread of the residuals
    about a least-squares quadratic over a second either side, less the fast noise, which the
    scatter holds whole. Under a canopy, a swinging rocket and the gusts on it scatter a
    barometer's readings by 0.3–7.5 m about two-second fits on public pairs' descents
    ([the sensor-noise note][noise] §3), far more than a pad shows. This was found on a public
    forum log, a barometer's track with a GNSS receiver's, whose pad was supplied as a constant
    reading, as a logger reading whole feet at rest gives. Weighed by the pad's noise alone,
    960 of its 2,704 barometer samples failed the gate, the filter reset 40 times, and its apogee
    came out 1,630 ± 58 m against the barometer's highest reading, 1,407 m. With the scatter, every
    sample is fused, the mean NIS is 1.01, and the apogee is 1,401 ± 43 m.
11. **The check.** Per set, state and moment, each side's misses of the 90% interval are counted.
    A cell holds when the lower edge of the two-sided 99.9% Clopper–Pearson band on each side's
    miss share is at most 5%, so the flights don't show it missing more often than it says it
    does. The report gives each cell overall and per logger, with the median and the largest
    standard error, so an interval that holds only by being wide shows. `cargo xtask synthetic`
    fails if any cell fails on any set, or if any step goes unsmoothed. It also fails if trusting
    GNSS from liftoff holds on A or B. The tuning set's 200 flights are too few to require that;
    its result is reported.
12. **The health card** is `Smoothed::health`, written out by `Health::card`. It holds each
    sensor's samples, fused, rejected and not weighed, its resets, its share inside the gate, its
    mean NIS and largest test ratio; then the de-weighted spans, when GNSS was first trusted, the
    barometer's height less the accelerometer's at apogee, and the steps the pass back kept
    forward. The report sums the cards per set and prints one per logger. Its surfaces in
    `hpr analyze` and Python come with M7.2e, which carries the intervals there too.
13. **What was seen before the rules were set.** The rules were found on the tuning set. A and B
    were run as each change landed, and three changes were found by tracing flights that failed
    there: the apogee rule, the port term at the rate and the landing's jerk. The flight's own
    noise came from the real log in item 10, and A and B were run after it. For the apogee
    charge span, A's failing apogee-speed cell came first, and B's was seen at 62 of 845. So A
    and B are no longer untouched for those cells. Each rule is physical, not fitted: the
    new constant, the span's 5 m, comes from a coast's shape, and the rest reuse the smoother's own.
    M7.2e's test of every reading starts from the same sets.

**Consequences.**

- On B, the median standard errors are 59 m of height and 11 m/s of speed at burnout, 85 m and
  0.76 m/s at apogee, and 19 m and 0.88 m/s at a drawn moment. The day's air, held at its prior
  without a receiver, and the port's spread make up most of them.
- The widest intervals are on barometer-only flights through Mach 1, up to about 410 m and 190 m/s.
  They hold the truth, but say little there.
- Trusting the receiver from liftoff fails on A and B at apogee and at a drawn moment, where the
  lagging fix pulls the height: 151 of 845 truths at apogee lie below B's interval.
- `cargo xtask synthetic` takes 234 s on an 8-core Mac, up from 101 s, as each flight is smoothed
  twice and each GNSS log twice more.
- The flight's own noise widens B's barometer weights where its loggers scatter more than their
  pads: barometer samples rejected fell from 29,500 to 10,822 of 20.6 million, resets from 90 to
  20, and every cell's coverage stayed within the check.
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
