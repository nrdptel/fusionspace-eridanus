# ADR-358: M1.14b9 in two: the tip-off phase on RocketPy's normal force first (2026-11-18)

- **Status:** accepted; splits [M1.14b9][m1-14b9] ([ADR-357][adr-357], decision 4) into [M1.14b11][m1-14b11] and [M1.14b12][m1-14b12], each with its own *done when*, M1.14b9's *done when* whole across the two; M1.14b12 waits for [M1.14e][m1-14e], and [M1.14b10][m1-14b10] comes next
- **Summary:** HPR Sim's tip-off phase agrees with RocketPy's on all 26 flights when both fly RocketPy's normal force, and on the 13 still-air flights on HPR Sim's own. On its own normal force the 13 crosswind flights are past the bars, because the two codes' normal forces differ by 1.23 to 2.71 times at 10° to 26° angle of attack. M1.14b9 stays open on those 13.

[m1-14b9]: ../decisions-and-roadmap.md#m1-14b9
[m1-14b10]: ../decisions-and-roadmap.md#m1-14b10
[m1-14b11]: ../decisions-and-roadmap.md#m1-14b11
[m1-14b12]: ../decisions-and-roadmap.md#m1-14b12
[m1-14e]: ../decisions-and-roadmap.md#m1-14e
[adr-357]: 0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
[report]: ../../validation/reports/tip-off-model.md
[report-json]: ../../validation/reports/tip-off-model.json
[reference]: ../../validation/reports/tip-off.md
[flight]: ../physics/flight.md#tip-off
[i1538]: https://github.com/nrdptel/fusionspace-eridanus/issues/1538
[cost]: ../physics/flight.md#what-leaving-tip-off-out-costs
[body-lift]: ../physics/aero.md#body-lift
[pr-920]: https://github.com/RocketPy-Team/RocketPy/pull/920

**Context.** [ADR-357][adr-357] wrote [M1.14b9][m1-14b9]'s *done when*: HPR Sim gains a tip-off
phase, and on RocketPy's thirteen example rockets, in still air and in a 20 mph (8.94 m/s)
crosswind, its figures are within set bars of RocketPy's. The bars are on the turning rate and
the angle off the rail where the last guide leaves, and on what the phase moves in the apogee
and the landing point. The *done when* ends: M1.14b9 is not met while any figure is past its
bar; an ADR then gives the miss's cause with its sign and size, the report keeps it visible, and
no bar is widened.

The phase is built ([the flight page][flight]). On HPR Sim's own aerodynamics it is inside every
bar on the 13 still-air flights and past a bar on all 13 crosswind flights. This record gives
the cause, with its sign and size, and splits the increment so the roadmap shows what is held
and what is not. Every figure below is in [the committed report][report]
([as data][report-json]), which `cargo xtask tip-off-model --check` holds in the gate's
`validate` step and in CI.

**Decision.**

1. **Two increments.** M1.14b9 keeps its *done when* as ADR-357 wrote it, and is met when both
   are.
   - **M1.14b11, the tip-off phase, held to RocketPy's on RocketPy's normal force.** *Done
     when:* M1.14b9's *done when* but for which flights its bars hold:
     - the phase, its equation, source and worked example, and the pendulum test within 1e-9,
       as M1.14b9's first clause has them;
     - HPR Sim flies each of the reference's thirteen designs from the reference's rail and
       site, on its drag coefficient, in still air and in its crosswind, guided as today and
       with the phase, twice over: on HPR Sim's own normal force, and on RocketPy's. RocketPy's
       is read from RocketPy with each flight, as its slope against Mach number and its center
       of pressure, and checked there against RocketPy's own totals;
     - a committed report, held by a check that runs in CI, sets beside RocketPy's for each of
       the 26 and each normal force the figures M1.14b9's second clause lists, each with its
       bar, and marks every figure past one;
     - on RocketPy's normal force, every figure of all 26 flights is inside M1.14b9's bars,
       unchanged; on HPR Sim's own, every figure of the 13 still-air flights is; the check
       fails if either stops being so;
     - for each crosswind flight the report gives the angle of attack where the last guide
       leaves, both codes' normal-force coefficients there and each one's moment about the last
       guide; this record gives the cause of the crosswind misses with its sign and size;
     - and the phase is off unless a flight asks for it: every other committed report
       reproduces unchanged.
   - **M1.14b12, the crosswind tip-off on HPR Sim's own normal force.** *Done when:* M1.14b9's
     *done when*, whole: what is left of it here is its bars on the 13 crosswind flights flown
     on HPR Sim's own normal force. Today all 13 are past the turning rate's bar, by up to
     4.010 deg/s against a bar of 0.100 deg/s; 10 are past the angle's, by up to 0.0671°
     against 0.0050°; 3 are past the apogee move's, by up to 0.379% of the apogee against
     0.2%; and 4 are past the landing-point move's, by up to 4.082% against 1%.
2. **The measurement.**
   - **On RocketPy's normal force, 26 of 26 flights are inside every bar.** The largest
     differences are 0.003 deg/s in the turning rate against a bar of 0.238 deg/s, 0.0001° in
     the angle against 0.0083°, 0.002% of the apogee in the apogee move against 0.2%, and
     0.029% of the apogee in the landing-point move against 1%.
   - **On HPR Sim's own normal force, 13 of 26 are:** all 13 in still air, none of the 13 in
     the crosswind. In still air the air meets the rocket nearly head-on, and the normal force
     hardly acts.
3. **The cause of the crosswind misses is the normal force, not the phase.**
   - **Where.** As the last guide leaves in the crosswind, the rocket moves at 18.6 to
     52.4 m/s and the air meets it at 9.7° to 25.6°.
   - **Size.** There HPR Sim's own normal-force coefficient is 1.23 to 2.71 times RocketPy's.
     RocketPy's is its slope at the Mach number times the angle of attack, at any angle. HPR
     Sim's adds [body lift][body-lift], which grows with the angle, to the nose's and the
     fins'. How much of the difference is body lift is not split out here. The force's moment
     about the last guide differs more, since the center of pressure differs too: for Bella
     Lui +2.44 m against +0.16 m for each unit of dynamic pressure and reference area.
   - **Sign.** On all 13, HPR Sim on its own normal force leaves the last guide turning
     farther nose-right, downwind, than on RocketPy's: by 0.136 to 4.011 deg/s.
   - **What separates the cause from the phase.** The same 13 flights, in the same code, with
     the same phase, differ in the normal force alone, and that takes the turning rate's
     difference from RocketPy's from as much as 4.010 deg/s to at most 0.003 deg/s.
4. **No bar is widened, and no reference changes.** M1.14b9's bars are ADR-357's, and
   M1.14b11's are the same numbers. M1.14b11 holds them on fewer flights of HPR Sim's own
   aerodynamics than M1.14b9 asks for, which is why M1.14b9 stays open and M1.14b12 carries the
   rest. The report keeps the 13 crosswind flights on HPR Sim's own normal force, every figure
   past a bar in bold.
5. **Which normal force is right is not decided here.** No reference in the validation set
   gives a finned body's normal force at 10° to 26° and Mach 0.06 to 0.16
   ([issue #1538][i1538]). RocketPy's line is a small-angle slope carried to any angle. HPR
   Sim's body lift is Jorgensen's method, checked on one body alone from 5.5° to 21.7° at
   Mach 1.5 to 4.63, and below Mach 1 only through that body's slope near zero angle
   ([the aerodynamics page][body-lift]). That question is the large-angle increment's
   ([M1.14e][m1-14e]), so M1.14b12 waits for it.
6. **The order.** [M1.14b10][m1-14b10], the phase in every flight with two guides, comes next,
   as ADR-357 decision 6 put it, ahead of M1.14b12. The normal force in question already acts
   on every HPR Sim flight in a wind from the moment the rocket is free, with the phase or
   without it. The phase adds the stretch on the last guide alone, 0.071 s for Juno III in
   RocketPy's still-air flight ([the flight page's worked example][cost]). M1.14b10's
   own *done when* holds the default to the real flights' apogee error.

**Consequences.**

- The roadmap shows M1.14b9 open, M1.14b11 done and M1.14b12 open. A reader who sees the phase
  in the library also sees that its crosswind figures on HPR Sim's own aerodynamics are not held
  to RocketPy's.
- The phase stays opt-in (`Simulation::with_tip_off`) until M1.14b10. Every committed report
  flies without it and reproduces unchanged.
- The oracle's report ([RocketPy's side][reference]) now records what each flight carried: the
  normal force, the dry mass, the reference radius, the motor and the parachutes. HPR Sim's
  flights refuse a design whose dry mass, reference area or motor differs from what RocketPy
  flew.

**What this leaves open.**

- **The comparison is code to code.** RocketPy's phase is in its development branch
  ([its pull request #920][pr-920]), in no release, and no real flight checks either code's.
- **M1.14b12 may not be met by a change to HPR Sim.** If [M1.14e][m1-14e] finds HPR Sim's normal
  force at these angles nearer a measured one than RocketPy's line, bars against RocketPy's
  crosswind figures would hold HPR Sim to a force that reference rejects. A record would then
  have to say so, with that reference. This record does not pre-judge it.
- **The pivot is on the rocket's axis,** at the middle of the last guide. A real button sits a
  body radius off the axis and has play in its slot; a lug slides on a rod. Neither code models
  that.
