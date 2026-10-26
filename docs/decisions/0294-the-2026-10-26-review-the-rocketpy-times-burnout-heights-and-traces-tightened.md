# ADR-294: The 2026-10-26 review: the RocketPy times, burnout heights and traces tightened (2026-10-26)

- **Status:** accepted; tightens [M2.1][m2-1]'s gates on the RocketPy whole flights flown on the same inputs, as [ADR-289][adr-289] did for the speeds and six apogees: each rail-exit time from 3% to 0.5%, and in the six flights with no written model difference the apogee time from 3% to 1%, the burnout height from 3% to 0.2%, the height trace from 3% to 1% of RocketPy's apogee and the speed trace from 3% to 0.5% of its top speed; no done-when is loosened and no release moves
- **Summary:** In the RocketPy whole flights flown on the same inputs, the rail-exit times agree within 0.143%, and in the six flights with no written model difference the apogee times within 0.239%, the burnout heights within 0.049% and the height and speed traces within 0.272% of RocketPy's apogee and 0.153% of its top speed, while RocketPy's own solver moves the point metrics by at most 2.4e-5 of their value; so those 33 gates go from 3% to between 0.2% and 1%, each at least 3.2 times the largest difference measured.

[m2-1]: ../decisions-and-roadmap.md#m2-1
[m2-1d1]: ../decisions-and-roadmap.md#m2-1d1
[adr-021]: 0021-whole-flights-against-rocketpy-what-is-compared.md
[adr-026]: 0026-the-path-in-wind-rocketpys-corrected-equations.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-289]: 0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[census]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/flight/rocketpy-whole-flight.json
[validation]: ../VALIDATION.md
[accuracy]: ../accuracy.md#whole-flights-against-rocketpy

**Context.** A planning review looks for a target already met with room to spare, since raising a
bar is how a review keeps the project from settling ([ADR-209][adr-209]). The RocketPy whole
flights on the same inputs ([M2.1][m2-1], [ADR-021][adr-021]) fly RocketPy's example rockets in
both codes with the same declared drag, motor, rail, site and wind, so a difference is in the
equations of motion, the environment or the recovery, not the aerodynamics. [ADR-289][adr-289]
held their speeds to 0.2% and six apogees to 0.5%, and left every other metric at the milestone's
3%. The 2026-10-26 review measured the rest the same way.

Five more metrics sit far inside 3%. In the committed [report][report], each largest figure
rounded up:

- **Rail-exit time**, all nine flights: the largest difference is −0.143% (Juno III in wind,
  −0.14245%), and −0.137% among the six with no written cause (Juno III in calm air). All nine are
  negative: HPR Sim leaves the rail first.
- **Apogee time**, the six flights with no written cause (Calisto in wind and calm air, Valetudo,
  NDRT 2020, and Juno III and Bella Lui in calm air): +0.020% to +0.239% (Valetudo), all positive.
- **Burnout height**, the same six: −0.004% (NDRT 2020) to +0.049% (Valetudo).
- **Height trace** (`series_height_rms_m`, [M2.1d1][m2-1d1]: the RMS of HPR Sim's height less
  RocketPy's at RocketPy's 120 series times), the same six: 2.1156 m of Valetudo's 777.9 m apogee,
  0.272% of it; NDRT 2020's 2.4253 m, 0.200% of its 1,216.4 m; the other four 0.077% or less.
- **Speed trace** (`series_speed_rms_m_s`), the same six: 0.1717 m/s of Valetudo's 112.6 m/s top
  speed, 0.153% of it; NDRT 2020's 0.069%; the other four 0.029% or less.

RocketPy's own numerical error is far smaller. Its [reference][fixture] records each metric flown
again at `rtol = 1e-6` instead of `1e-8` (each case's `solver.relative_change_from_loose`): the
rail-exit time moves by at most 2.4e-5 of its value (NDRT 2020, 2.385e-5), the apogee time of the
six by at most 2.7e-6 (Calisto in calm air, 2.67e-6) and their burnout height by at most 1.0e-5
(Bella Lui in calm air, 9.99e-6). The traces are not flown again, so their solver movement is not
recorded. A 3% gate is 21 times the largest rail-exit time difference and 62 times the largest
burnout height difference, so a fault that moved the burnout height by 2.9% would pass it. The
[census][census] holds each row near where it was, but `cargo xtask census --accept` can
re-baseline any row, and the gate is the only bound a re-accepted change still has to meet.

**Decision.**

1. **The 33 gates.** Each is held with no absolute floor; each trace's bound is in its own unit,
   rounded down as the case files already round, to 0.1 m and 0.01 m/s:

   | metric | rows | bound before | largest measured | bound now | headroom |
   |---|--:|--:|---|--:|--:|
   | `rail_exit_time_s` | 9 | 3% | −0.143% (Juno III in wind) | 0.5% | 3.5× |
   | `apogee_time_s` | 6 | 3% | +0.239% (Valetudo) | 1% | 4.1× |
   | `burnout_altitude_agl_m` | 6 | 3% | +0.049% (Valetudo) | 0.2% | 4.1× |
   | `series_height_rms_m` | 6 | 3% of the apogee | 0.272% of it (Valetudo, 2.1156 m) | 1% of the apogee (Valetudo: 7.7 m) | 3.6× |
   | `series_speed_rms_m_s` | 6 | 3% of the top speed | 0.153% of it (Valetudo, 0.1717 m/s) | 0.5% of the top speed (Valetudo: 0.56 m/s) | 3.2× |

   Headroom is the bound over the largest difference, rounded down (for the traces, the bound as the
   case file rounds it). The point bounds are 200 times RocketPy's solver movement or more: 0.5% is
   209 times 2.385e-5, 1% is 3,700 times 2.67e-6, and 0.2% is 200 times 9.99e-6. Every one of the 33
   was re-measured from the committed report before the change, and none had less than 2.5 times
   headroom, the least this record accepts. The bounds are two-sided, so a change that flips a sign
   (the rail-exit times are all negative today, the apogee times all positive) is held as tightly.
2. **What keeps 3%, and why.**
   - *The largest acceleration and its time*: RocketPy's own peak moves by up to 0.16%
     (Prometheus 2022) and its time by up to 0.18% (Valetudo) with the solver, and the largest
     scored difference among the six is 1.778% (Bella Lui in calm air, the ignition spike on the
     rail where HPR Sim keeps the variable-mass terms); in wind, Bella Lui's is 1.783%.
   - *The apogee and landing drifts*: up to −1.750% (Juno III in calm air) and −1.811%
     (Valetudo).
   - *The flight time*: among the six, up to +0.643% (NDRT 2020), carrying its descent's 0.70% descent-rate
     difference, which [ADR-289][adr-289] left at 3%.
   - *The largest Mach number*: the untraced speed-of-sound offset [ADR-289][adr-289] describes.
   - *The three flights with a written cause* (Juno III, Bella Lui and Prometheus 2022 in wind,
     [ADR-026][adr-026]): their apogee times reach +0.652%, their burnout heights +0.600% and their
     traces 0.313% to 0.961% of the apogee, from the rail release and body lift their case files
     measure. Their rail-exit times are within 0.006 percentage points of the same rockets' in
     calm air (Juno III: −0.142% against −0.137%), and Prometheus 2022's is −0.033%, so they are
     tightened with the other six.
   - The descents keep [ADR-289][adr-289]'s bounds.
3. **How it is held.** The nine flight case files under `validation/cases/` carry the bounds and
   their headers say so. `same_drag_times_burnout_heights_and_traces_hold_adr_294s_bounds` in
   `crates/hpr-validate/src/tests.rs` fails when any of the 33 is not held to exactly its bound
   with no floor or is marked not scored, when a trace's bound is not its share of the
   reference's own apogee or top speed rounded down, or when the counts change (9 rail-exit
   times, 12 points, 12 traces). Each of eight probes failed it and was reverted: Valetudo's apogee
   time set back to 3%; Prometheus 2022's rail-exit time set back to 3%; Valetudo's height trace
   set back to 23.3 m; a 0.01 s floor on Calisto's rail-exit time; NDRT 2020's burnout height
   removed; Juno III's calm speed trace marked not scored; a copy of Valetudo's case at the old
   bounds added to the lock; and Valetudo's height bound raised to 7.8 m in the case and the test
   together. The report and the census were regenerated (`cargo xtask validate`, then
   `cargo xtask census --accept` with this record as the reason): 33 rows changed scale and
   slack, no measured value moved, and `cargo xtask validate --check` reproduces the report.
   [The validation page][validation] and [Accuracy][accuracy] state the new gates.
4. **What a failure means.** As [ADR-289][adr-289] §4: a change that moves one of these past its
   bound fails the harness. Its cause is measured against RocketPy on the same inputs. A fault is
   fixed; a real model difference that HPR Sim keeps on purpose is measured, written in the case
   file and in a decision record, and only then moves that one metric to the 3% tier. A gate is
   never widened to pass.

**What could go wrong.**

- *The platforms.* CI flies the cases on macOS, Windows and Linux, on x86-64 and ARM
  (`validate --check`). The census's slack is 0.1% of each new bound, and never less than the
  harness's reproduction bound (2e-6 in the metric's unit or 1e-7 of the value): about 1e-5 of
  the value for an apogee time, 2e-6 for a burnout height, 1e-5 of the apogee and 5e-6 of the top
  speed for the traces. 2e-6 of the value is the slack the 41 speeds at 0.2% have held on every
  platform since [ADR-289][adr-289]. The rail-exit times' slack falls to 2.0e-6 to 2.3e-6 s, at or
  just above the reproduction bound, which every platform already meets on every row of the
  report. The RocketPy reference is a committed file, not flown again in CI, so no solver noise of
  RocketPy's enters.
- *Thin margins.* Valetudo's speed trace is the closest, 3.2 times inside, then Juno III's
  rail-exit time (3.5 times) and Valetudo's height trace (3.6 times). The traces include the
  descent, so a recovery change (an opening time, an added mass) is the likeliest thing to move
  them; §4 decides what happens then.
- *The sample.* The margins are measurements on RocketPy's six example rockets, and agreement
  between two codes, not accuracy against flights. A rocket added to the set gets its own measured
  differences and, where one has a cause, the 3% tier for that metric with the cause written.

**Consequences.** The same-drag comparison now notices a regression in the rail-exit time at a sixth
of the size it could before, in the apogee time at a third and in the burnout height at a fifteenth,
and along the height and speed traces at a third and a sixth. HPR Sim's own step-size error at these
settings is not measured here; a change of its default method or step is a change these gates now
see. The predicted-mode comparisons, where each code flies its own drag, keep their 3% targets. No
done-when is loosened, no release moves, the queue keeps its order, and no user-facing surface is
added.