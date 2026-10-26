# ADR-289: The 2026-10-24 review: the RocketPy speed and apogee gates tightened (2026-10-24)

- **Status:** accepted; tightens [M2.1][m2-1]'s gates on the RocketPy comparisons flown on the same inputs: each burnout, largest, rail-exit and landing speed from 3% to 0.2%, and the apogee of the six flights with no written model difference from 3% to 0.5%; places four ideas; no done-when is loosened and no release moves
- **Summary:** In the RocketPy comparisons flown on the same inputs, the burnout, largest, rail-exit and landing speeds agree within 0.056% and the apogees of the six flights with no written model difference within 0.113%, while RocketPy's own solver moves them by at most 0.0032% and 0.00064%; a 3% gate was 53 times the largest speed difference, so they become 0.2% and 0.5%, and the planning review places four new ideas.

[m2-1]: ../decisions-and-roadmap.md#m2-1
[m0-14]: ../decisions-and-roadmap.md#m0-14
[m0-9]: ../decisions-and-roadmap.md#m0-9f
[m7-2]: ../decisions-and-roadmap.md#m7-2
[adr-021]: 0021-whole-flights-against-rocketpy-what-is-compared.md
[adr-026]: 0026-the-path-in-wind-rocketpys-corrected-equations.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-286]: 0286-the-2026-10-20-review-the-export-bar-tightened.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[census]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/flight/rocketpy-whole-flight.json
[descents]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/recovery/rocketpy-descent.json
[validation]: ../VALIDATION.md
[ideas]: ../research/ideas.md

**Context.** A planning review looks for a target already met with room to spare, since raising
a bar is how a review keeps the project from settling ([ADR-209][adr-209]); the last one
tightened the `.ork` export's bar ([ADR-286][adr-286]). The RocketPy comparisons on the same
inputs ([M2.1][m2-1], [ADR-021][adr-021]) fly RocketPy's example rockets in both codes with the
same declared drag, motor, rail, site and wind, so a difference is in the equations of motion,
the environment or the recovery, not the aerodynamics. Every scored metric of the nine whole
flights and five descents is a gate at 3%, the milestone's bar, and the harness fails the run
past it.

The speeds sit far inside it. In the committed [report][report], over the 41 speed rows (burnout
speed, largest speed and rail-exit speed of the nine flights, and the landing speed of the nine
flights and five descents), the largest differences are +0.056% (Juno III's burnout speed),
+0.052% (its largest speed), 0.031% (Prometheus 2022's landing under its parachute) and 0.015%
(rail exit), each rounded up. The apogees of the six flights whose case files give no cause for
one (Calisto in wind and calm air, Valetudo, NDRT 2020, and Juno III and Bella Lui in calm air)
differ by +0.038% to +0.112%. The other three apogees have a measured cause in their case files,
differences HPR Sim keeps on purpose ([ADR-026][adr-026]): Juno III in wind (+0.641%; RocketPy
flown with hpr's rail release, body lift and thin-plate fins reaches 2,631.0 m against hpr's
2,631.57), Prometheus 2022 (+1.208%; rail release and body lift, 3,723.8 m against 3,723.6), and
Bella Lui in wind (+0.342%, 532.12 m against 530.31), whose case file now records that RocketPy flown
with hpr's release at the last button reaches 531.3 m and with hpr's body lift added 532.0 m.

RocketPy's own numerical error is far smaller. Its [reference][fixture] records each metric
flown again at `rtol = 1e-6` instead of `1e-8`: the speeds move by at most 3.2e-5 of their value
(NDRT 2020's rail exit, 3.13e-5, rounded up) and the apogee by at most 6.4e-6 (Calisto in calm
air, 6.35e-6); the [descents'][descents] landing speeds by less than 1e-6. A 3% gate is 53 times
the largest speed difference, so a fault that moved a burnout speed by 2.9% would pass it. The
[census][census] holds each row near where it was, but `cargo xtask census --accept` can
re-baseline any row, and the gate is the only bound a re-accepted change still has to meet.

**Decision.**

1. **Speeds: 0.2%.** Each burnout, largest, rail-exit and landing speed in the same-drag flights
   and the descents is held to 0.2% relative, with no absolute floor: 3.5 times the largest
   difference measured (0.056%) and 62 times RocketPy's largest solver movement on one of them
   (3.2e-5). The mean descent rate keeps 3% (its largest difference, NDRT 2020's, is 0.70%), and
   so does the largest Mach number: it differs by up to 0.235% (Prometheus 2022), and by 0.105%
   and 0.123% in calm air (Calisto, Juno III), while the largest speeds agree within 0.052%, so
   the two codes' speed of sound at the peak, or how they read Mach, differs. That offset is not
   traced and is left until it is.
2. **Apogees with no written cause: 0.5%.** The six apogees above are held to 0.5%: 4.4 times
   the largest (0.112%) and about 780 times the solver's movement. The three with a measured
   cause keep 3%. Every apogee difference is positive today (HPR Sim higher in all nine); the
   bound is two-sided, so a change that flips the sign is held as tightly.
3. **How it is held.** The 14 case files under `validation/cases/` carry the bounds and their
   headers say so; `same_drag_speeds_and_cause_free_apogees_hold_adr_289s_bounds` in
   `crates/hpr-validate/src/tests.rs` fails when any of the 41 speeds or six apogees is not held
   to exactly its bound with no floor, or when the counts change (a new same-drag case added at
   the old bound, a metric renamed or dropped); setting one descent's landing speed back to 3%,
   adding a floor, or marking an apogee not scored each fails it. The report and the census were
   regenerated (`cargo xtask validate`, then `cargo xtask census --accept` with this record as
   the reason): 47 rows changed scale, no measured value moved. [The validation
   page][validation] states the new gates.
4. **What a failure means.** A change that moves one of these speeds past 0.2% or one of the six
   apogees past 0.5% fails the harness. Its cause is measured against RocketPy on the same inputs,
   as the three written causes were: a fault is fixed; a real model difference that HPR Sim keeps
   on purpose (as the rail's variable-mass terms are kept) is measured, written in the case file
   and in a decision record, and only then moves that one metric to the 3% tier. A gate is never
   widened to pass.
5. **Ideas placed.** Four ideas join [the backlog][ideas], each with its user, workflow, the
   scoreboard number it moves and a *done when*: designs that fly as saved, counted against
   OpenRocket (`flies.as-saved`); time to apogee and burnout speed scored against the logs for
   both tools (`accuracy.time-to-apogee`, `accuracy.burnout-speed`); the collection's error split
   by Mach band, impulse class and diameter (`accuracy.collection` and `bias.collection` by cell);
   and one flight's numbers compared across the CI platforms (`determinism.platforms`). None is
   promoted: the queue's head holds two user-facing surfaces in progress ([M0.14][m0-14] with
   [M0.9f][m0-9], and [M7.2][m7-2]), and the queue keeps its order.

**Consequences.** The same-drag comparison now notices a regression in the equations of motion,
the rail, the motor's thrust and mass or the parachutes at a fifteenth of the size it could
before in a speed and a sixth in an apogee, which is what it exists to catch. The margins are
measurements on RocketPy's six example rockets, and agreement between two codes, not accuracy
against flights: a rocket added to the set gets its own measured differences and, where one has
a cause, the 3% tier for that metric with the cause written. HPR Sim's own step-size error at
these settings is not measured here; a change of its default method or step is a change these
gates now see. The predicted-mode comparisons, where each code flies its own drag, keep their
3% targets. No done-when is loosened, no release moves, the queue keeps its order, and no
user-facing surface is added.
