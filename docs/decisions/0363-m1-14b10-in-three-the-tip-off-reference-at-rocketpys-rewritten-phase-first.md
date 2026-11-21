# ADR-363: M1.14b10 in three: the tip-off reference at RocketPy's rewritten phase first (2026-11-20)

- **Status:** accepted; splits [M1.14b10][m1-14b10] ([ADR-357][adr-357], decision 4) into [M1.14b13][m1-14b13], [M1.14b14][m1-14b14] and [M1.14b15][m1-14b15], each with its own *done when*; the *done when* changed: M1.14b10 keeps its own words whole across the three and gains M1.14b13, which ADR-357 did not ask for
- **Summary:** RocketPy rewrote its tip-off phase after HPR Sim's comparison was pinned. Re-pinned at the rewrite, the reference moves by at most 0.000019 deg/s in a turning rate and about 1 mm (0.04 in) in any apogee or landing point, the same flights are past the plan's bars, and HPR Sim's phase is inside every bar it was held to. Flying the phase by default reaches the command line, the Python package, the comparisons with other codes and most committed flight reports, so it is split: the reference first, then what the phase moves on each real flight, then the default.

[m1-14b9]: ../decisions-and-roadmap.md#m1-14b9
[m1-14b10]: ../decisions-and-roadmap.md#m1-14b10
[m1-14b13]: ../decisions-and-roadmap.md#m1-14b13
[m1-14b14]: ../decisions-and-roadmap.md#m1-14b14
[m1-14b15]: ../decisions-and-roadmap.md#m1-14b15
[m2-13]: ../decisions-and-roadmap.md#m2-13
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-357]: 0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
[adr-358]: 0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md
[report]: ../../validation/reports/tip-off.md
[report-moved]: ../../validation/reports/tip-off.md#what-rocketpys-rewrite-of-the-phase-moved
[model-report]: ../../validation/reports/tip-off-model.md
[flight]: ../physics/flight.md#what-leaving-tip-off-out-costs
[pr-920]: https://github.com/RocketPy-Team/RocketPy/pull/920
[pr-1196]: https://github.com/RocketPy-Team/RocketPy/pull/1196
[pr-1217]: https://github.com/RocketPy-Team/RocketPy/pull/1217
[i1550]: https://github.com/nrdptel/fusionspace-eridanus/issues/1550

**Context.** [M1.14b10][m1-14b10]'s *done when* ([ADR-357][adr-357], decision 4) is: a flight of
a design with two or more guides flies the tip-off phase unless told not to; each committed
report it moves is regenerated in the same pull request; a committed table gives, for each real
flight HPR Sim compares, what the phase moved in the apogee and, where a landing point is
compared, in the landing point, with the real flights' mean absolute apogee error before and
after; a comparison with a code that has no such phase flies with it off and says so; a test
pins which way a landing point errs without the phase, with a mutation probe; and if the real
flights' mean absolute apogee error rises with the phase on, an ADR measures the rise and its
cause before the default changes.

Two things stand between today's code and that.

- **The reference moved.** HPR Sim's phase is held to RocketPy's at the merge of RocketPy's
  [pull request #920][pr-920]. On 2026-11-17 RocketPy merged [pull request #1217][pr-1217],
  which rewrites the phase, corrects how it ends and flies it in every flight whose two
  buttons start on the rail. A default should not rest on a reference its own authors have
  since corrected ([issue #1550][i1550]).
- **The default is wide.** A survey of the code before this record:
  - Today only the library offers the phase (`Simulation::with_tip_off`). The command line
    (`hpr sim`, `hpr mc`, `hpr compare`) and the Python package (`Flight`, `MonteCarlo`) carry
    their flight options field by field and have no such option.
  - 15 of the 19 public designs under `validation/designs/` carry two guides: the 13 built
    from RocketPy's examples and the 2 synthetic ones. The 20 OpenRocket files under
    `validation/fixtures/ork/` carry none. How many of the private collection's designs carry
    two guides at different stations has not been counted.
  - Flights that leave a rail feed the validation report's RocketPy cases, the real-flight
    reports, the public predictions, the collection's flights, the synthetic sets and the fit
    made on them, the normal-force override's fixture and the examples' committed outputs.
  - Four comparisons fly beside a code with no such phase and would have to opt out: the
    RocketPy whole-flight cases, the OpenRocket flight records, the collection's split of
    HPR Sim's gap to OpenRocket, and the Python package's Calisto example.
  - A prediction merged before its flight was read ([ADR-328][adr-328]) needs a revision entry
    when a default moves it.

That is more than one pull request can carry with its checks, and its first step does not
depend on the rest.

**Decision.**

1. **Three increments.** M1.14b10 keeps ADR-357's *done when* and is met when all three are.
   - **M1.14b13, the tip-off reference at RocketPy's rewritten phase.** *Done when:*
     - `validation/refs.lock.toml` pins `rocketpy-tip-off` at the merge of RocketPy's pull
       request #1217, and the oracle flies the reference's 26 flights from it three ways: the
       phase as that commit flies it, without the phase, and guided to the lower button. The
       oracle stops on any other checkout, and unless the commit's equations carry
       [pull request #1196][pr-1196]'s correction;
     - the reference's report is regenerated from that pin and held by its check. The report
       as the earlier pin gave it is committed unchanged, named by the commit that last held
       it and held by its SHA-256. For each of the 26 flights the new report gives how far
       the rewrite moved the four figures [M1.14b9][m1-14b9]'s bars are on, compared as the
       bars compare two codes, and how far each flight's own apogee and landing point moved.
       It says whether the same flights are past the plan's bars, and whether each rocket
       carried the same;
     - HPR Sim's phase is flown again against the new reference with ADR-357's bars
       unchanged. On RocketPy's normal force every figure of all 26 flights is inside its
       bar, and on HPR Sim's own every figure of the 13 still-air flights is; the check fails
       otherwise. M1.14b13 is not met while one of those is past a bar: a record then says
       which side the miss errs on, with its size and cause, and no bar is widened;
     - this record says whether the two defects pull request #1217 names as left open reach
       a figure the comparison reads;
     - and the flight page says which commit the reference flies and what the rewrite moved.
   - **M1.14b14, what the phase moves on each real flight.** *Done when:*
     - a committed table, held by a check, lists each real flight HPR Sim compares: the
       flights of the real-flight report, of the public predictions, and the private
       collection's logged flights by their anonymised ids. For each it says whether the
       design has two or more guides at different stations. For each that has, it gives the
       apogee flown guided as today and with the phase, and the move between them. Where a
       landing point is compared it gives the landing point's move as a vector. A flight
       with fewer than two guides is listed as not moved;
     - for each of those three sets the table gives the mean absolute apogee error guided
       and with the phase, on the flights and by the error that set's own report uses, with
       the signed mean beside it;
     - a test pins which way a landing point errs without the phase, with a mutation probe
       that fails it;
     - if any set's mean absolute apogee error is higher with the phase, an ADR measures the
       rise and its cause before M1.14b15 changes the default;
     - and the phase stays opt-in: every other committed report reproduces unchanged.
   - **M1.14b15, the phase by default.** *Done when:* M1.14b10's *done when*, whole. What is
     left of it after the two above is the default itself: every flight of a design with two
     or more guides flies the phase unless told not to, from the library, the command line
     and the Python package alike; each report that moves is regenerated in that pull
     request; each comparison with a code that has no such phase flies with it off and says
     so; and M1.14b14's table is regenerated on the default. M1.14b10 is not met while the
     phase is opt-in.
2. **M1.14b13's measurement** ([the report][report], [each flight][report-moved]).
   - **The pin** is `65b9838f23f62ce6155ac712798ccab01c45ea47`, the merge of pull request #1217.
     It carries pull request #1196, so the oracle flies the commit's equations as they are
     and no longer applies that correction itself. The commit has no switch for the phase. A
     flight without it takes the commit's own branch for a rocket with no pair of buttons on
     the rail, which frees the rocket where its upper button leaves, as RocketPy's releases
     do.
   - **What the rewrite moved.** Where the lower button leaves with the phase on, the
     turning rate moved by at most 0.000019 deg/s and the angle off the rail's axis by at
     most 0.0000003°. What the phase moves from the guided flight changed by at most
     0.000033% of the apogee in the apogee and 0.000032% of the apogee in the landing point.
     No flight's apogee moved by more than 1.04 mm, nor its landing point by more than
     0.96 mm. The same flights are past the plan's bars, and each rocket carried the same
     normal force, dry mass, motor and parachutes.
   - **HPR Sim's phase against the new reference** ([the comparison][model-report]). On
     RocketPy's normal force all 26 flights are inside every bar. The largest differences
     are 0.003 deg/s in the turning rate against a bar of 0.238 deg/s, 0.0001° in the angle
     against 0.0083°, 0.002% of the apogee in the apogee move against 0.2%, and 0.029% of
     the apogee in the landing-point move against 1%. At that rounding they are the figures
     [ADR-358][adr-358] recorded on the earlier pin. On HPR Sim's own normal force the 13
     still-air flights are inside every bar and the 13 crosswind flights are not, as before.
   - **Why so little moved.** The rewrite's corrections are to how the phase ends: the time
     the rail exit is found at on a long solver step, a parachute opening inside the phase,
     and a rail too short for the phase to end. None of the 26 flights met one of those on
     the earlier pin. That reading is this record's, from the size of the moves; the report
     measures the moves and does not test each cause.
3. **The two defects pull request #1217 leaves open do not reach the comparison.** Its
   description names a wrong lever arm and a missing weight in the rail-button forces while
   both buttons are on the rail. RocketPy computes those forces after the flight, from the
   solved trajectory (`Flight.rail_button1_normal_force` and its three companions). They are
   outputs: while both buttons are on the rail the rocket moves along the rail alone, and the
   equations of that stretch do not read them. The comparison reads the state where the last
   button leaves, the apogee and the landing point, and none of the forces.
4. **The order.** M1.14b13 ships with this record. M1.14b14 is next, then M1.14b15, all ahead
   of the increments ADR-357's decision 6 put after tip-off. M1.14b14 comes before the default
   so that what the default does to the real flights' error is known, and written down, before
   any report moves.

**Consequences.**

- The roadmap shows M1.14b10 open, M1.14b13 done, and M1.14b14 and M1.14b15 open.
- The reference's report loses its second run, the earlier commit's equations without pull
  request #1196: the new commit carries that correction, so there is no such run to fly.
  ADR-357's reading of that run stands on the kept report, and a test holds it there.
- The kept report is 556 kB of this repository's own history, committed a second time under
  `validation/fixtures/flight/`.

**What this leaves open.**

- **The comparison is still one code against another.** RocketPy's phase is in its
  development branch, in no release, and no real flight checks either code's.
- **The pin is a development commit.** When a RocketPy release carries the phase, the
  reference should move to it, by the same steps as here.
- **M1.14b14's landing column may be empty.** No real flight's landing point is compared
  today; [landing points against the tracked landings][m2-13] is a later milestone. The table
  then says so.
- **The private collection's designs with two guides are not counted yet.** M1.14b14's table
  counts them.
