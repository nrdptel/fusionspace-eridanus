# ADR-341: M2.3d8 in five, a parallel stage's added mass first (2026-11-11)

- **Status:** accepted; splits [M2.3d8][m2-3d8] ([ADR-336][adr-336]) into five increments, each with its own *done when*, M2.3d8's *done when* whole across the five, and so plans every remaining increment of M2.3d8: [M2.3d10][m2-3d10] (carried out here), [M2.3d11][m2-3d11], [M2.3d12][m2-3d12], [M2.3d13][m2-3d13] and [M2.3d14][m2-3d14]. It extends [ADR-340][adr-340]'s rule, a stage's parts left in place under an override that covers them, to a parallel stage. [ADR-171][adr-171]'s refusal of a covering override on the stage a parallel stage hangs on stands; this record measures what that record could not.
- **Summary:** A parallel stage in an OpenRocket file, boosters strapped beside the core, can carry a mass override that doesn't cover its parts. HPR Sim warned of one and held the design back. It now reads it as OpenRocket 24.12 weighs it: one mass added to the set of boosters, at their center or at a stated station. Of 49 new probe designs HPR Sim lays out 27, and on the 25 of those with boosters the parallel stage alone matches OpenRocket's mass, center and both inertias. The probes also show cases this change doesn't fly, so M2.3d8 is split in five, and four increments stay open.

[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d8]: ../decisions-and-roadmap.md#m2-3d8
[m2-3d9]: ../decisions-and-roadmap.md#m2-3d9
[m2-3d10]: ../decisions-and-roadmap.md#m2-3d10
[m2-3d11]: ../decisions-and-roadmap.md#m2-3d11
[m2-3d12]: ../decisions-and-roadmap.md#m2-3d12
[m2-3d13]: ../decisions-and-roadmap.md#m2-3d13
[m2-3d14]: ../decisions-and-roadmap.md#m2-3d14
[adr-171]: 0171-a-parallel-stage-is-a-pod-set-that-separates.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[adr-336]: 0336-m2-3d4-in-three-the-inertia-and-the-refusals-last.md
[adr-340]: 0340-a-stage-s-overrides-leave-its-parts-in-place-as-openrocket-weighs-them.md
[i1423]: https://github.com/nrdptel/fusionspace-eridanus/issues/1423
[i1424]: https://github.com/nrdptel/fusionspace-eridanus/issues/1424
[i1429]: https://github.com/nrdptel/fusionspace-eridanus/issues/1429
[i1430]: https://github.com/nrdptel/fusionspace-eridanus/issues/1430
[i1431]: https://github.com/nrdptel/fusionspace-eridanus/issues/1431
[i1432]: https://github.com/nrdptel/fusionspace-eridanus/issues/1432
[probe]: ../glossary.md#probe-design
[placement]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/added_mass_placement.py
[record]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/openrocket-added-mass-placement.json
[ork-stage]: ../format/ork.md#a-stages-mass-and-center-of-gravity-overrides
[design-added]: ../physics/design.md#a-stages-added-mass
[left-out]: ../physics/design.md#a-stages-added-mass-leaves-out

**Context.** [ADR-335][adr-335] decision 5 lists what HPR Sim still refuses of a stage's added
mass: a parallel stage that carries one, a stage a parallel stage hangs on that carries one, one
with no station on parts that weigh nothing, and a flight that moves a mass, releases one or
parts through such a stage. [ADR-336][adr-336] gave them to M2.3d8, whose *done when* has each
flown as OpenRocket 24.12 weighs it, its mass, center and inertia held to OpenRocket's on
[probe designs][probe].

None of the first three was measured. The earlier probes have no parallel stage. Their one stage
of parts that weigh nothing starts at the rocket's tip, so its center at the tip doesn't say
whether that is the stage's forward end. So the work began with a survey:
[`added_mass_placement.py`][placement] asks OpenRocket 24.12 about 49 designs, and
[its record][record] is committed. Each probe records the whole rocket and each stage active
alone, which is the stage as OpenRocket weighs it once the other is gone.

The survey found four different pieces of work, and one of them changes how a flight composes
its stages. That is more than one session, so this record splits M2.3d8 once, with every
remaining increment written here.

**The measurements.** The airframe is a nose with ballast, a tube and three fins: 0.3616 kg,
centered 0.3477 m aft of the tip. Three boosters hang round its tube, each a nose and a tube,
their forward end 0.45 m aft of the tip. Together they weigh 0.0605 kg, centered 0.6199 m aft,
with an inertia across the axis of 0.000487 kg·m². One booster alone weighs 0.0202 kg, 52 mm off
the axis. The mass override is 2 kg. The center override is 0.1 m on the parallel stage and
0.4 m on the stage it hangs on.

1. **A parallel stage's own overrides weigh as an axial stage's do.** The parallel stage alone,
   with three boosters:

   | The parallel stage's overrides | Mass (kg) | Center (m) | Inertia across the axis (kg·m²) |
   |---|---:|---:|---:|
   | none that takes effect (a center alone, flag `false`) | 0.0605 | 0.6199 | 0.000487 |
   | mass, flag `false` or not written | 2.0605 | 0.6199 | 0.000487 |
   | both, flags `false` or not written | 2.0605 | 0.5521 | 0.000774 |
   | mass flag `false`, center flag `true` | 2.0605 | 0.5500 | 0.000783 |
   | mass, flag `true` | 2.0000 | 0.6199 | 0.000487 |
   | both, mass flag `true`, center flag `true` or `false` | 2.0000 | 0.5500 | 0.000783 |
   | center alone, flag `true` | 0.0605 | 0.5500 | 0.000783 |

   - The 2 kg is added once to the set of boosters, not to each.
   - With no center stated it sits at the boosters' center and adds no inertia.
   - A center that doesn't cover the parts is a station from the parallel stage's forward end:
     0.45 m and 0.1 m put the point 0.55 m aft of the tip. With one booster the point stays
     52 mm off the axis, level with the booster.
   - Under an override that covers the parts, the boosters stay where the design puts them. The
     inertia is theirs about the stage's center:
     $0.000487 + 0.0605 \times 0.0699^2 = 0.000783\,\mathrm{kg\cdot m^2}$.
     That is [ADR-340][adr-340]'s rule for a stage on the axis.
   - The stage the boosters hang on is as it was, under every row.
2. **The stage a parallel stage hangs on is weighed with it.** With the boosters on, that stage's
   "parts" are its own and the boosters'. With no override the rocket weighs 0.4222 kg, centered
   0.3868 m aft.
   - *A mass that doesn't cover the parts, with no center.* The rocket weighs 2.4222 kg and its
     center stays at 0.3868 m: the point is at the center of the stage and its boosters together.
     The stage alone weighs 2.3616 kg at its own parts' 0.3477 m. So the point moves 39 mm when
     the boosters drop, and 14 mm with one booster.
   - *The same mass at a station.* The stage alone is its parts and a point 0.4 m aft of the
     tip, and with three boosters the rocket is that stage and the boosters. With one booster
     the rocket stays 2.7 mm off the axis, as with no override. A point on the axis would pull
     it in to 0.4 mm. So OpenRocket's point is off the axis with the rocket's center while the
     booster hangs there.
   - *A center that covers the parts.* The stated 0.4 m is the rocket's center with the boosters
     on, and the stage's without them.
   - *A mass that covers the parts.* The rocket weighs the stated 2 kg with the boosters on, and
     the stage weighs 2 kg without them.
3. **On parts that weigh nothing, a mass with no center stated sits at the rocket's tip** (six
   of the eight such probes). A lower stage that starts 0.8 m aft of the tip and boosters that
   start 0.45 m aft each weigh their 2 kg at the tip, on the axis, alone as with the other
   stage. On the other two a center that covers the parts is beside the mass, and the 2 kg is
   at that center.
4. **Beside a parallel stage's covering override, OpenRocket's rocket is not its stage's body.**
   OpenRocket sums each booster about the rocket's center from where the design puts it. It
   takes the rocket's center from the stage as it weighs it alone: that stage's whole mass at
   its own center. A body's inertia
   goes with where its mass is, so the stage as OpenRocket weighs it alone, taken as one body,
   can't give that sum. On the probe of an added mass beside a covering center, with three
   boosters, OpenRocket's sum is more than that body's by $2m(x - c)(c - X)$: the boosters'
   mass $m$, their center $x$, the stated center $c$ and the rocket's center $X$. Under a
   mass that covers the boosters, OpenRocket's point at the stated center is the whole stated
   mass, beside the boosters' own. Their mass is counted twice, so the sum is more than a
   body's by more than that term, even where the center doesn't move. The same sum reproduces OpenRocket's figure on all ten probes whose override
   covers the boosters.

**Decision.**

1. **A parallel stage's overrides are read as an axial stage's.** The `.ork` reader uses
   [ADR-335][adr-335] decision 2's table for a `<parallelstage>` too. A mass that doesn't cover
   the parts becomes the parallel stage's added mass, with a station when a center that doesn't
   cover them is beside it. An override that covers the parts is the whole stage's, weighed with
   its parts in place ([ADR-340][adr-340]'s rule). A center alone that doesn't cover the parts
   changes nothing and is kept for an export. No row raises a warning. Before, the reader took
   every override on a parallel stage as covering it, with a warning where the flag said
   otherwise, which held the design back.

   In `hpr-design`, a parallel stage can carry an added mass: one point for the set, at the
   pods' center or at a station from the parallel stage's forward end, as far off the axis as
   the pods' center. The stage it hangs on can carry one at a station, unless the parallel
   stage is one pod off the axis (decision 4).
2. **Five increments.** M2.3d8 keeps its *done when* as written and is met when all five are.
   - **M2.3d10, a parallel stage's added mass.** *Done when:* M2.3d8's *done when*, whole: what
     is left of it here is a `.ork` parallel stage whose mass override doesn't cover its parts,
     and the stage one hangs on with such a mass at a station where the parallel stage's center
     is on the axis. Each is read with no warning and flown, its mass, center and inertia
     within decision 3's bounds of OpenRocket 24.12's on the probe designs. This record carries
     it out.
   - **M2.3d11, the stage a parallel stage hangs on, weighed with it.** *Done when:* M2.3d8's
     *done when*, whole: what is left of it here is that stage with an added mass that has no
     station, alone or beside a center that covers the parts, or with one at a station beside
     one booster off the axis. Each is flown as OpenRocket 24.12 weighs it, with the boosters on
     and once they are gone, its mass, center and inertia within decision 3's bounds on the
     eight probe designs the layout refuses today ([#1429][i1429]).
   - **M2.3d12, an added mass on parts that weigh nothing.** *Done when:* M2.3d8's *done when*,
     whole: what is left of it here is an added mass with no station on a stage whose parts
     weigh nothing, flown as OpenRocket 24.12 weighs it, its mass, center and inertia held to
     OpenRocket's on the eight probe designs the layout refuses today ([#1430][i1430]).
   - **M2.3d13, flights that move, release or part.** *Done when:* M2.3d8's *done when*, whole:
     what is left of it here is a flight with a mass that moves, a mass that is released, or a
     parting through a stage, where that stage carries an added mass. It is flown, the stage
     weighed as OpenRocket 24.12 weighs it until the event, and afterward by a rule its record
     states and its tests work by hand ([#1431][i1431]).
   - **M2.3d14, the rocket's inertia beside a parallel stage's covering override.** *Done
     when:* M2.3d8's *done when*, whole: what is left of it here is the rocket's inertia where
     a parallel stage has an override that doesn't cover its parts beside one that does. That
     is an added mass beside a center that covers them, or a center that doesn't cover them
     beside a mass that does. What the overrides add to each of the rocket's inertias is within
     decision 3's bound of what they add in OpenRocket 24.12, on the four probe designs of
     those two pairings. Today they add less across the axis: 0.63% and 0.22% of OpenRocket's
     inertia less on the first pairing (three boosters, one), and 0.79% and 0.27% less on the
     second. About the axis they add 0.103% less with one booster on the second
     ([#1432][i1432]).
3. **M2.3d10's bounds** are the ones [ADR-335][adr-335]'s probes hold, but for the one on
   what an override adds to an inertia, which here goes with how much it adds.
   - The parallel stage alone: its mass within 5 parts in 10 million of OpenRocket's, its
     center within 0.2 µm along the axis and off it, and each inertia within 3 parts in a
     million.
   - The stage it hangs on, and the whole rocket: the mass and the center within the same
     bounds. The airframe has fins, whose inertia the two tools weigh differently, so its
     inertias are held by what the override adds to them: within 5 parts in 10 million of what
     it adds in OpenRocket, or within 1e-9 kg·m² where it adds less than 0.002 kg·m². ADR-335's
     probes hold a flat 1e-9 kg·m², on additions of 0.00071 kg·m² and less. A parallel stage's
     mass override adds more, and the flat bound fails there: with the mass alone, on three boosters
     and on one, the override adds more than 0.018 kg·m² across the axis, and the two tools
     differ by between 1.3e-9 and 1.6e-9 kg·m², under 8 parts in 100 million of it
     (`a_parallel_stages_override_not_covering_its_parts_adds_to_them` pins both). So the bound
     goes with the addition, at the share the masses are held to, and keeps ADR-335's 1e-9
     kg·m² below it. No probe ADR-335 holds is held to less than before.
   - With one booster, an inertia across the axis is compared about the body axis the booster
     lies along. OpenRocket gives one inertia across the axis, the one about its y axis, and
     the booster hangs off the axis along that axis, so how far off it is adds nothing to
     OpenRocket's figure. About that body axis HPR Sim's has no such term either.
   - Not held: the rocket's inertia where a parallel stage's override covers its parts
     (measurement 4). Each shortfall is pinned. M2.3d14 holds the four probes where an override
     that doesn't cover the parts is beside one that does. On the other six every override
     covers the parts, which puts them outside M2.3d8's *done when*. No increment is planned for
     them, and [#1432][i1432] records them.
4. **What stays refused, and why.**
   - *M2.3d11.* HPR Sim weighs each stage as one body that keeps its mass and center when
     another stage drops. OpenRocket's stage changes when its boosters drop (measurement 2).
     Flying that means composing the stage's mass with what hangs on it at each separation,
     which changes the flight's staging. Until then the layout refuses an added mass with no
     station on that stage, and one at a station beside one booster off the axis.
   - *M2.3d12.* A mass at the rocket's tip reads the stability margin as high as any placement
     can: the flattering side. M2.3d12's record has to say how a flight tells the flier where
     the mass was put, with a test pinning which way the margin errs. Leaving such a stage
     refused instead would take a record that changes this *done when*.
   - *M2.3d13.* No probe of OpenRocket 24.12 in this record covers a mass that moves, a mass
     that is released or a parting through a stage, so no measurement stands behind one. The
     record has to choose where an added mass with no
     station goes when its stage's parts move, leave or part.
   - *M2.3d14.* OpenRocket's sum is not its stage's body's (measurement 4). It is the composition
     [M2.3d9][m2-3d9] builds for a lower stage ([#1424][i1424]), so M2.3d14 follows M2.3d9.
     Under a mass that covers the boosters the sum counts their mass twice. The *done when*
     holds HPR Sim to OpenRocket's figure there, and holding it to anything else would take a
     record that changes it.
5. **The order.** M2.3d10, M2.3d11, M2.3d12 and M2.3d13, then M2.3d9, then M2.3d14.
   [M2.3d2][m2-3d2], the errors, stays last, as ADR-336 put it. [M2.3d4][m2-3d4] is met when
   M2.3d8 and M2.3d9 are.

**How well HPR Sim agrees.** HPR Sim lays out 27 of the 49 probes and refuses 22.

- On the 25 with boosters that it lays out, the parallel stage alone has OpenRocket's mass,
  center and both inertias within decision 3's bounds. That covers every row of measurement 1's
  table with three boosters and with one, the two with no override, the two with no flag
  written (the mass alone, and both overrides), the two with a mass at a station on the stage
  the boosters hang on, and three with a motor aboard.
- On those 25 the whole rocket has OpenRocket's mass and center. Where no override covers the
  parallel stage's parts, the override adds what it adds in OpenRocket to both of the rocket's
  inertias.
- Beside a center that covers the parallel stage's parts, with an added mass, the rocket's
  inertia across the axis gains 0.63% of OpenRocket's inertia less with three boosters and 0.22%
  less with one. With a center that doesn't cover the parts beside a mass that does, it gains
  0.79% and 0.27% less, and about the axis 0.103% less with one booster. Those four probes are
  decision 2's M2.3d14.
- On the six probes where every override on the parallel stage covers its parts (a mass, a
  center, or both), the same shortfall is 0.070% to 4.673%. About the axis the rocket gains
  what OpenRocket's does, but for one booster under a mass that covers it: 0.103% short. With
  every override covering the parts, these six are outside M2.3d8's *done when*. No increment
  is planned for them, and [#1432][i1432] records them.
- Under HPR Sim's default rule, which the reader gave a parallel stage until now, the parallel
  stage's own inertia across the axis is more than 37% off OpenRocket's on each of the ten
  probes whose override covers the parts.
- The 22 refused are 12 with an override on the stage the boosters hang on, 2 with a mass at a
  station there beside one booster, and 8 with no place for the mass to sit. A test pins what
  OpenRocket does on each.

**What doesn't change.** An axial stage's overrides, a part's and a pod set's read and fly as
before, and so does every drag override. A covering override on the stage a parallel stage
hangs on is still refused ([ADR-171][adr-171]); the six probes of it show that it covers the
boosters while they hang there, and [#1429][i1429] holds that. No tolerance moved.

**Consequences.**

- An OpenRocket design whose parallel stage has a mass override that doesn't cover its parts now
  flies, with the mass its author entered on top of the boosters. Its boosters carry that mass
  away when they drop.
- A parallel stage's override that covers its parts is weighed with the boosters in place, so
  its inertia changes: it was the boosters' own, scaled with the mass and moved whole.
- A parallel stage read from a file keeps its rule through an export, and one under the default
  rule is warned of on export, as a stage on the axis is. That is what [#1423][i1423] asked for.
- The roadmap shows M2.3d8 open with M2.3d11 to M2.3d14 below it, and the archive shows M2.3d10
  done. The pages [A stage's override][ork-stage] and [A stage's added mass][design-added] say
  what is read and [what is still left out][left-out].
- The roadmap is at its byte budget, so six open entries there now name their *done when* by
  its record and no longer repeat it: M2.3d8 and M2.3d9 name [ADR-336][adr-336]'s decision 1,
  and M2.3d11 to M2.3d14 name this record's decision 2. No *done when* changes. The words the
  roadmap drops (among them "each", "held to OpenRocket's" and "as OpenRocket 24.12 weighs it")
  stand in those two records, which are the text each increment is held to. The same edit
  drops the lists of finished increments from M2.3d1's and M2.3d4's headings; the archive has
  them.

**How it was checked.** All in `hpr_validate::openrocket` unless named otherwise.

- `a_parallel_stages_override_not_covering_its_parts_adds_to_them` holds the mass alone and
  both overrides, neither covering the parts, with three boosters and with one, and the two
  rows with no flag.
- `a_parallel_stages_override_covering_its_parts_leaves_them_in_place` holds the parallel stage
  alone under each covering row, pins the rocket's shortfalls, and holds that a center alone
  that doesn't cover the parts changes nothing.
  `openrockets_sum_with_a_parallel_stages_covering_center_is_no_bodys` reproduces OpenRocket's
  figure from its own records.
- `an_added_mass_at_a_station_on_the_stage_boosters_hang_on` holds the stationed mass with three
  boosters and pins the refusal with one.
- `the_stage_boosters_hang_on_is_weighed_with_them_in_openrocket` and
  `an_added_mass_on_parts_that_weigh_nothing_sits_at_the_tip_in_openrocket` pin measurements 2
  and 3 beside HPR Sim's refusals, so M2.3d11's and M2.3d12's *done when* fail today.
  M2.3d13's refusals stand in `hpr-sim`, pinned by its tests, and M2.3d14's four shortfalls are
  pinned above.
- `a_motor_aboard_does_not_move_a_parallel_stages_added_mass` holds the three probes with a
  motor, and `every_placement_probe_is_checked` counts all 49.
- In `hpr-design`, `a_parallel_stages_added_mass_sits_among_its_pods` works the layout by hand.
  In the `.ork` writer, `a_parallel_stages_override_not_covering_its_parts_is_read_and_written_back`
  holds the reader's table and the round trip.
- `hpr-sim`'s `strap_on_boosters_take_their_added_mass_with_them` drops boosters with an added
  mass, and the command line's `sim_flies_a_ork_parallel_stage_whose_override_adds_to_its_parts`
  flies two of the probes through `hpr sim`: a mass on the parallel stage, alone and at a
  station. It also flies the same airframe with the recorded override at a station on the stage
  the boosters hang on. The other probes are laid out and weighed, not flown.
