# ADR-314: M7.3 in five: residuals first, the simulation read as the log is (2026-11-01)

- **Status:** accepted; splits [M7.3][m7-3] in five under [ADR-257][adr-257], whose *done when* is unchanged and met when M7.3a to M7.3e are
- **Summary:** M7.3 is more than one PR, so it goes in five parts: residuals, the least-squares fit with its coverage test, the Bayesian option under the same test, the report with apogee predicted from the boost, and the post-flight plot. Residuals come first because the rest stand on them. They rest on one rule: the simulated flight is written out as a noise-free log and read by the same reader as the real log, its height as the real log's sensor would read it, and both clocks are lined up by what that reader calls liftoff.

[m7-3]: ../decisions-and-roadmap.md#m7-3
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-308]: 0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md
[adr-312]: 0312-two-logs-on-one-timeline.md

**Context.** M7.3 asks for five things that build on one another, and each is a PR of its own size.
[ADR-257][adr-257] fixes their tests, so this record only orders them and fixes how a log meets a
simulated number. That meeting needs a rule, because a logged and a simulated number aren't the same
quantity. A barometer reads the standard atmosphere's altitude of the pressure around it, so on a warm day
it reads low. A reading such as apogee depends on the median window it was taken through. And a log's
clock has no ignition on it. Comparing the simulator's own apogee with a log's reading would mix all
three into the residual.

**Decision.**

1. **The simulation is read as the log is.** `hpr_forensics::residuals` writes the simulated flight as a
   noise-free log at the real log's median sample interval, with its height as that log's sensor would read
   it and an axial accelerometer if that log has one. The same reader takes the same readings from it.
   - The height stands for a barometer's reading unless the log's track is a GNSS receiver's or the
     accelerometer's integral, as the reader names its source. A caller that knows the sensor from elsewhere
     can say so: `cargo xtask real-flights` uses the altimeter each public flight's record states.
   - The speed's residual is the smoother's speed less the rate at which that sensor's reading would change,
     for the same reason.
   - The clocks are lined up by the [shared liftoff][adr-312], the reader's liftoff on each. The bound on
     their offset is reported, and it is wide: 1.31 to 3.21 s on the 7 public flights. It is what the reader
     can promise of liftoff, not a measured misalignment.
2. **A residual is logged less simulated**, sample by sample, by phase (boost, coast or ascent, descent).
   Heights are the log's own samples, unsmoothed, as ghost files take them, so an ejection charge's spike or
   a garbled tail shows. Speeds are the smoother's, with its standard error.
3. **A reading keeps its provenance.** Each row holds the log's reading as `hpr analyze` gives it, its
   bound, 90% interval, source and provenance. The simulated cell holds no source, provenance or
   interval: it wasn't measured, and its bound on noise-free samples says nothing of the simulation's error.
4. **The increments**, each with its own *done when* in the roadmap:
   - **M7.3a Residuals.** `hpr compare`, and a committed report on the 7 public flights.
   - **M7.3b The least-squares fit.** Levenberg–Marquardt on drag scale, impulse scale, a constant wind vector
     and mass within its weighed uncertainty, each with a 90% interval passing ADR-257 §1 on sets A and B, with
     its three probes, its identification rule and its pure-noise probe. This is the first run of a fit over
     thousands of flights, so it also sets how fast one fit must be.
   - **M7.3c The Bayesian option.** The same parameters under the same test, truths not drawn from its prior.
   - **M7.3d From the boost.** Apogee predicted from the log cut at burnout, beside the forward simulation's,
     on the public flights and the collection; the report and scoreboard row ADR-257 §3 asks for.
   - **M7.3e The post-flight comparison.** One command draws predicted and logged altitude and velocity,
     liftoff to landing, as designed and under launch-day conditions (ADR-257 §4).
5. **The public flights' parachutes come from their notebooks.** `real-flights` flies only to apogee, and a
   design's `.json` holds no recovery, so the report's table takes each example's canopies from its notebook.
   Cavour's notebook adds none and says so, so only its ascent is compared. Lince's notebook splits a payload off
   under a canopy of its own; the report flies the main body alone.

**Consequences.** `hpr compare` leaves the planned list. A comparison's report and command share one library
function, so M7.3b's fits and M7.3e's plot get the same clocks and heights. `Barometer` moves from `hpr-validate`
to `hpr-atmos`, where the forensics crate can reach it; no number moves. The report's flights show what this
increment doesn't fix. Juno III's log has a garbled sample (a residual of 7.8 km at 29.85 s after liftoff).
Lince's log swings by 500 m or more in 0.01 s as its recovery fires, and the smoother's speed reads +10,211 m/s
there. Genesis's smoothed speed lies +705 m/s from the simulated at 8.24 s after liftoff, and Prometheus's boost
leaves an RMS of 86 m/s. They are shown, not filtered: the heights are raw samples, and the speeds are what the
smoother gives on them. None of it has been traced to a cause here.
