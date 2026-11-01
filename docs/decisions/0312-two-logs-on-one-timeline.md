# ADR-312: Two logs on one timeline: the offset's error, the new sets, a booster's truth (2026-10-31)

- **Status:** accepted; carries out [ADR-280][adr-280]'s M7.2f
- **Summary:** two logs of one flight, or one logger to a stage, go on one timeline by a shared
  liftoff and read side by side, never averaged. The offset's stated error is the sum of the two
  liftoffs' bounds, with a 90% interval fitted and tested as a reading's is. Four sets on seeds of
  their own check it, in a report of its own. HPR Sim's observers now see a separated body's steps,
  so a booster has a truth.

[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[report]: ../../validation/reports/two-logs.md
[debrief]: ../research/debrief-flight-readings.md#two-recordings-of-one-flight

**Context.** ADR-280 asks that two logs of one flight sit on one timeline by a shared liftoff, read
side by side and never averaged, their spread reported; that the alignment recover the true clock
offset within its stated error on two-logger flights drawn on new seeds; that per-stage logs of a
staged synthetic flight sit on one timeline; and that RocketPy's two-logger flight read side by side
from `hpr analyze`. It leaves open what the stated error is. [Debrief][debrief] took each record's
liftoff and offset by its negation, and deleted three validity rules with measurements that showed
them useless.

**Decision.**

1. **The method is Debrief's**: each log's liftoff reading is the timeline's zero. There is no
   sweep over lags and no check that the logs are of one flight, as Debrief's measurements found
   none that had power. The result names the method. The command's trust note says the flight is
   the caller's word.
2. **The offset's stated error is the sum of the two liftoffs' bounds**, the most two errors can
   add to. Each bound is held on the tuning set already. It proves loose: the largest error was
   0.13 of it. So the offset also states a 90% interval, by shares of that sum fitted on a set of
   its own and tested as ADR-257 §1 asks, since M7.2's parent asks one of every reading. The shares
   are −0.0526 and 0.0515.
3. **Four sets, on seeds fixed here, none in the census.** Two-logger calibration (2,000 flights,
   seed 2026103100), A and B (1,000 each, 2026103101 and 2026103102), staged (200, 2026103103).
   Each flight's second logger and both clocks draw on streams of their own, each clock moved by up
   to 300 s. A log with no barometer has no liftoff, so its flight gives no offset, counted by
   reason. `cargo xtask two-logs --check` holds [the report][report] on all five platforms.
4. **A staged flight's truth needs the booster's fall.** `FlightResult::bodies` holds a separated
   body's events and not its path, so `hpr_sim::Observer` gains `body_step`, seeing every accepted
   step of a body with its dense output, as `step` does a flight's, and doing nothing by default. It
   changes no flight. The booster tumbles from the split, because HPR Sim needs a device
   acting there on a booster dropped under power. The sustainer takes a canopy.
5. **Stages report no spread.** A booster's apogee and a sustainer's are different heights.
6. **A floor joins no spread**, not being the value. A log that ends on the climb makes floors of
   its smoothed apogee and its top speed too, in this comparison, each without an interval. One log
   read alone doesn't do so yet ([#1089][i1089]).

**Consequences.** Reading a pair needs no design file and no simulation, as ADR-046 asks. A GPS
track can't join a timeline yet ([#1086][i1086]), nor can a log read by a mapping, nor can Python
read a pair yet ([#1085][i1085]). The staged path rests on simulated flights alone
([#1084][i1084]). One real pair checks the offset against another clock, weakly: its two logs both
start at the boost, so an offset of zero misses by little more, and the reference has no stated
error of its own. Its evidence row therefore takes the weakest kind behind it, calibration.

[i1084]: https://github.com/nrdptel/fusionspace-eridanus/issues/1084
[i1085]: https://github.com/nrdptel/fusionspace-eridanus/issues/1085
[i1086]: https://github.com/nrdptel/fusionspace-eridanus/issues/1086
[i1089]: https://github.com/nrdptel/fusionspace-eridanus/issues/1089
