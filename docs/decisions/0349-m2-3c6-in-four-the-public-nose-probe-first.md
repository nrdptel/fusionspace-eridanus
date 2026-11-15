# ADR-349: M2.3c6 in four: the public nose probe first (2026-11-14)

- **Status:** accepted; splits [M2.3c6][m2-3c6] into [M2.3c10][m2-3c10], [M2.3c11][m2-3c11], [M2.3c12][m2-3c12] and [M2.3c13][m2-3c13], each with its own *done when*, M2.3c6's *done when* whole across the four, and so plans every remaining increment of M2.3c6; under [ADR-226][adr-226] and [ADR-273][adr-273], neither changed
- **Summary:** The gap to OpenRocket by cause needs four things the tree lacks: a public probe of the tangent-ogive nose's pressure drag, OpenRocket's thrust, mass and drag along its own flights with a way for HPR Sim to fly them, the split in the fixture report, and the nose's own line. Each becomes an increment. The probe ships first: on an invented fineness-3 tangent ogive, OpenRocket 24.12 gives the nose a pressure drag coefficient of 0.00098, 0.00073 and 0.00047 at Mach 1.3, 1.5 and 2, where HPR Sim and the equivalent cone give 0.1557, 0.1303 and 0.1042.

[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[m2-3c10]: ../decisions-and-roadmap.md#m2-3c10
[m2-3c11]: ../decisions-and-roadmap.md#m2-3c11
[m2-3c12]: ../decisions-and-roadmap.md#m2-3c12
[m2-3c13]: ../decisions-and-roadmap.md#m2-3c13
[m2-3c3]: ../decisions-and-roadmap.md#m2-3c3
[m1-14]: ../decisions-and-roadmap.md#m1-14
[adr-067]: 0067-curves-come-from-openrockets-own-database-by.md
[adr-097]: 0097-a-cause-in-the-drag-sized-by-hpr-flying.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-226]: 0226-the-2026-10-10-review-the-gap-to-openrocket-by-cause.md
[adr-273]: 0273-the-2026-10-15-review-openrockets-supersonic-ogive-drag-as-a-cause.md
[i-222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
[i-616]: https://github.com/nrdptel/fusionspace-eridanus/issues/616
[or-3391]: https://github.com/openrocket/openrocket/issues/3391
[or-3431]: https://github.com/openrocket/openrocket/pull/3431
[forward]: ../../validation/reports/fixture-flights.md
[probe]: ../../validation/fixtures/ork/openrocket-nose-pressure-drag.json
[aero]: ../physics/aero.md#a-tangent-ogive-past-mach-1-beside-openrocket-2412

**Context.** M2.3c6's *done when* asks for three things. The fixture report splits the HPR Sim
less OpenRocket apogee into thrust, mass and drag parts, each input swapped for OpenRocket's in
that order, the parts tested to sum to the whole ([ADR-226][adr-226]). The drag part splits off
the pressure drag of tangent-ogive noses above Mach 1, with #222's flight among those swapped.
And a public fineness-3 probe sets the two codes' nose pressure drag beside the equivalent
cone's, a test pinning HPR Sim's ([ADR-273][adr-273]).

A survey of the tree on `main` at commit 62204939 found what each needs.

- **The whole.** [The fixture report][forward] already gives HPR Sim's apogee less OpenRocket
  24.12's on the 60 flights compared with a log: a mean of +0.90% and a mean absolute value of
  1.63%. Both fly 68 flights.
- **Drag.** HPR Sim can fly another tool's drag coefficient against Mach number, and
  `drag_curves.py` records OpenRocket's along its own flights, power on and power off
  ([ADR-097][adr-097]). It is run on the design library and the jar's examples, not on the
  collection's flights in their day's weather.
- **Thrust.** HPR Sim flies the curve OpenRocket places, found by its digest, with the total
  impulse held to OpenRocket's within 0.1% ([ADR-067][adr-067]). So the thrust part is what the
  two codes make of one curve. A flight takes its thrust from its motors' curves: nothing lets
  it take a sampled thrust against flight time in their place.
- **Mass.** A flight can scale its structure's mass, and move or drop a mass at an event. Nothing
  lets it take a sampled mass and center of mass against flight time in place of its design's.
- **OpenRocket's side.** `fixture_flights.py` records each flight's largest altitude, the time
  to it and the largest Mach number. It records no thrust, mass, center of mass or drag
  coefficient at the flight's times.
- **The nose's column.** `drag_override.py` reads each part's friction, pressure and base drag
  through OpenRocket's public interface, and its record holds a tangent ogive's pressure column
  past Mach 1. No control there says what that column holds for a nose, or on which area.

OpenRocket's [#3391][or-3391] has moved since [ADR-273][adr-273] read it as an unconfirmed
report. A maintainer confirmed it on 2026-11-02 and merged a fix to the development branch the
same day ([pull request #3431][or-3431]): above Mach 1 an ogive takes the cone of its length
and base diameter, scaled by its shape's factor (1 for a tangent ogive), as OpenRocket's
technical documentation describes. By 2026-11-14 no release carried the fix, so 24.12, the
release every report here flies, still has the defect.

**Decision.**

1. **Four increments, in this order.** The probe goes first. It is public and needs nothing
   from the private collection. Its controls show which column of OpenRocket's interface holds
   a nose's pressure drag, and on which area, which the fourth increment's swap relies on. And it pins what 24.12
   gives before [M2.3c3][m2-3c3] flies a newer release beside it. The inputs come second,
   since the split and the nose's line both fly them.
   They are M2.3c10 to M2.3c13: a roadmap id ends in one letter and a number, so M2.3c6's
   increments take the next free numbers after M2.3c9.

2. **The increments.**
   - **M2.3c10 The fineness-3 nose probe.** *Done when:* a public probe, an invented rocket
     with a fineness-3 tangent-ogive nose, records OpenRocket 24.12's pressure drag
     coefficient of the nose at Mach 1.3, 1.5 and 2; a test sets it, on the nose's base area,
     beside HPR Sim's and the equivalent cone's, and pins HPR Sim's at each Mach number; the
     same rocket with a conical nose, the control, shows the column read is the nose's
     pressure drag, by returning the cone's closed form, and on a body that widens behind the
     nose shows the area the column is on; the record cites #3391; the aerodynamics page gives
     the table, what OpenRocket did about #3391, and what stays open.
   - **M2.3c11 OpenRocket's inputs along its flights, flown by HPR Sim.** *Done when:* an
     oracle records, under the gitignored `corpus-out/`, OpenRocket 24.12's thrust, mass,
     center of mass and drag coefficient at its flight's own times for every flight of the
     collection it flies, flown as `fixture_flights.py` flies it; the fixture report counts
     the flights holding all the inputs and each one lacking an input, each under a reason
     from a list the oracle fixes, none of them a catch-all; HPR Sim flies a thrust against
     time, and a mass and center of mass against time, given from outside in place of its
     own, each documented with its equation and with what stays HPR Sim's; each is pinned by
     a test against a closed form and by a test that a flight given the histories of its own
     run, sampled at the times a committed record of OpenRocket's own flight of that design
     holds, comes back within 0.05% of its own apogee, on at least two public designs, one of
     them passing Mach 1; a flight that can't take a history is refused with an error that
     says why.
   - **M2.3c12 The split in the fixture report.** *Done when:* the fixture report gives, as
     aggregates over the flights both simulators fly, the HPR Sim less OpenRocket apogee
     split into thrust, mass and drag parts, each input swapped for OpenRocket's in that
     order, the rest named as other, with the air on its own line inside other, as a fourth
     swap after drag; over the flights split, each part's mean, its mean absolute value and
     the count of flights it dominates, a part dominating a flight when its absolute value
     is the largest among thrust, mass, drag and other; the drag part in the drag-first
     order beside it; each flight not split counted under one of three reasons: OpenRocket's
     record lacks an input, HPR Sim refuses a history or fails the swapped flight, or the
     flight fails M2.3c11's own-history test at OpenRocket's sample times; the whole's mean
     and mean absolute value on the flights split, beside the same on every flight both
     fly; at least half the flights both fly split; a test holds the parts, the air's
     among them, to sum to the whole on every flight split; `fixture-flights --check`
     holds the section; the page that explains the report says what each part is and how
     far to trust it.
   - **M2.3c13 The tangent-ogive nose's line.** *Done when:* the drag part splits off the
     pressure drag of tangent-ogive noses above Mach 1, by swapping in OpenRocket's
     per-component nose pressure drag as its public interface reports it, the column
     M2.3c10's controls identify: its share of the gap, with its sign, over the flights
     whose HPR Sim flight passes Mach 1 with a tangent-ogive nose, each such flight counted as
     swapped or skipped with its reason, #222's flight among those swapped or a decision
     record saying why not; a test holds the line's flights to that rule; [#222][i-222] says
     what the line found.

3. **What the later increments take as settled.**
   - **The order.** Thrust, then mass and center of mass, then drag, then the air, each added
     to the ones before. A part is the apogee before its swap less the apogee after it. Other
     is what thrust, mass and drag leave: the air's line, and the last flight's apogee less
     OpenRocket's. So the parts sum to the whole by construction ([ADR-226][adr-226]).
   - **The air is a swap too.** OpenRocket flies the standard atmosphere through the pad's
     temperature and pressure, HPR Sim the day's whole profile. The report's existing flight
     of HPR Sim in OpenRocket's air uses HPR Sim's own inputs, so it is not a part of this
     split. The air's line is the apogee of the flight with thrust, mass and drag swapped,
     less the apogee of the same flight in OpenRocket's air.
   - **The mass history** replaces the mass and the center of mass. The inertia stays HPR
     Sim's, so its difference lands in other. M2.3c11's page also says which sets each of
     these, the history or HPR Sim: the time origin, the datum the center of mass is
     measured from, the thrust's correction for the air's pressure, and the mass flow behind
     jet damping. A history is interpolated so that those rates stay finite.
   - **The own-history test** samples a flight's histories at OpenRocket's times, since the
     error it bounds is the resampling's: a history arrives at OpenRocket's own time steps,
     and a short burn rebuilt from those samples can gain or lose impulse. The 0.05% is
     about a thirtieth of the 1.63% mean absolute gap the parts divide. M2.3c12 holds every
     flight it splits to the same test.
   - **The floor.** If fewer than half the flights both fly are split, M2.3c12 stays open. A
     decision record then gives, from the report, the flights lost under each of the three
     reasons and the measured cause of the largest, before anything is narrowed.
   - **The nose's line** is taken first inside the drag step: on thrust and mass, the nose's
     pressure drag is swapped, then the rest of the drag, so the line and the rest sum to the
     drag part. OpenRocket's columns are per instance: on the probe, the rocket's pressure
     drag is the nose's plus three times the fins' column. A nose that repeats, on pods, would
     need its count: M2.3c13 shows that on a probe before it swaps one. M2.3c13's oracle records the nose's column against Mach number
     for each design it swaps; M2.3c11's records the rocket's drag coefficient only.
   - **Only aggregates leave this machine** ([ADR-151][adr-151]), and ADR-151 governs.
     M2.3c13's set may be small, and an aggregate over one or two flights is a flight's own
     number. If the line can't be published for that reason, M2.3c13 stays open, with a
     decision record that gives the count and says so, and M2.3c6's clause stays shown as
     unmet.

**What doesn't change.** M2.3c6's *done when* is whole across the four. The split, its order,
other and the sum test are M2.3c12's, on the inputs M2.3c11 records and flies. The nose's line,
its counted flights and #222's flight are M2.3c13's. The probe, its citation and the pinned
test are M2.3c10's. No clause is narrowed. The increments add to it: the conical controls,
the air's line, the own-history test with its bound, the closed lists of reasons, the floor
on the flights split with no way round it, and each increment's own documentation and checks.

**What M2.3c10 found.** [The probe][probe] is a 150 mm nose on a 50 mm tube, the shape of
#3391's report, with three fins on the tube. Pressure drag coefficients of the nose,
on its base area, at zero angle of attack:

| Mach | equivalent cone | HPR Sim, tangent ogive | OpenRocket 24.12, tangent ogive | OpenRocket 24.12, cone |
|---:|---:|---:|---:|---:|
| 1.3 | 0.15571 | 0.15571 | 0.00098 | 0.15571 |
| 1.5 | 0.13028 | 0.13028 | 0.00073 | 0.13028 |
| 2 | 0.10421 | 0.10421 | 0.00047 | 0.10421 |

OpenRocket's cone returns the equivalent cone's closed form: within 1e-12 at Mach 1.5 and 2,
and 4.8e-6 below it at Mach 1.3. So the column is the nose's own pressure drag. A second
control puts that cone on a body that widens to 80 mm behind it, where the reference area is
2.56 times the nose's base: OpenRocket's number is then 1/2.56 of the first control's, so the
column is on the rocket's reference area. Its tangent ogive is under 1% of the equivalent
cone's at each Mach number. HPR Sim's tangent ogive is the equivalent cone's, as the
documented method gives it. [The aerodynamics page][aero] carries the table.

**Consequences.**
- [M1.14][m1-14] still waits on the split: nothing here says yet which input carries the gap.
- The probe is code against code. Whether the equivalent cone's value matches a measured
  tangent ogive stays open in [#222][i-222] and [#616][i-616].
- When OpenRocket releases the fix, [M2.3c3][m2-3c3] flies both releases on the flights. The
  probe can be run on the new jar to show the nose's column moved. That run is not in
  M2.3c3's *done when*.
- M2.3c11 adds a way to fly outside histories to the simulator's library. It is for sizing a
  difference between two codes, as the drag table is ([ADR-097][adr-097]), not a model.
