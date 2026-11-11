# ADR-336: M2.3d4 in three, the inertia and the refusals last (2026-11-10)

- **Status:** accepted; splits [M2.3d4][m2-3d4] into four increments, each with its own *done when*, M2.3d4's *done when* whole across the four, and so plans every remaining increment of M2.3d4: [M2.3d6][m2-3d6] (done, [ADR-335][adr-335]), [M2.3d7][m2-3d7] (done, [ADR-340][adr-340]), [M2.3d8][m2-3d8] and [M2.3d9][m2-3d9]. Amended in place on 2026-11-11: M2.3d7's two lower-stage probes moved to M2.3d9, which also takes the motor-aboard sum, a figure no increment held; the title and file name keep "in three" so that links to it hold. This record puts [M2.3d5][m2-3d5] before M2.3d7 and M2.3d8, where [ADR-334][adr-334] had put all of M2.3d4 before it, and [M2.3d2][m2-3d2] stays last, as ADR-334 put it.
- **Summary:** A stage in an OpenRocket file can carry a mass override and a center of gravity override, each covering the stage's parts or not. M2.3d4's *done when* holds such a stage's inertia to OpenRocket's and has it held back no longer. The merged work fell short twice. Where one override covers the parts and the other doesn't, HPR Sim kept its own inertia rule: from 11.04% under OpenRocket's to 400.5% over on eight probe designs. A few such stages are still refused. This record reopens the milestone: M2.3d6 is what merged, M2.3d7 the inertia, M2.3d8 the refusals, and, after ADR-340, M2.3d9 the inertia on a lower stage and with a motor. M2.3d4 is met when the last three are.

[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d5]: ../decisions-and-roadmap.md#m2-3d5
[m2-3d6]: ../decisions-and-roadmap.md#m2-3d6
[m2-3d7]: ../decisions-and-roadmap.md#m2-3d7
[m2-3d8]: ../decisions-and-roadmap.md#m2-3d8
[m2-3d9]: ../decisions-and-roadmap.md#m2-3d9
[adr-340]: 0340-a-stage-s-overrides-leave-its-parts-in-place-as-openrocket-weighs-them.md
[i1424]: https://github.com/nrdptel/fusionspace-eridanus/issues/1424
[adr-061]: 0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
[adr-334]: 0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[i1407]: https://github.com/nrdptel/fusionspace-eridanus/issues/1407
[pr1408]: https://github.com/nrdptel/fusionspace-eridanus/pull/1408
[left-out]: ../physics/design.md#a-stages-added-mass-leaves-out
[ork-stage]: ../format/ork.md#a-stages-mass-and-center-of-gravity-overrides
[probe]: ../glossary.md#probe-design

**Context.** [ADR-334][adr-334] wrote M2.3d4's *done when*: an `.ork` stage whose mass or center
override doesn't cover its parts is flown as OpenRocket 24.12 weighs it, its mass, center and
inertia held to OpenRocket's on [probe designs][probe], and is no longer held back.

[The pull request that carried it out][pr1408] met part of that. On 19 probes of a nose and tube
or of two stages, where neither of the stage's overrides covers the parts, the mass, the center
and the inertia are held to OpenRocket's. On the 17 other probes HPR Sim lays out (7 on those
rockets, 10 on an airframe with a motor aboard), the mass and the center are held. On eight of
those 17 the inertia is not: a stage with one override that covers its parts beside one that
doesn't keeps the rule [ADR-061][adr-061] gave every override that covers the parts. A ninth, a
center that covers the parts alone, keeps it too, outside M2.3d4's *done when*.
[ADR-335][adr-335] decision 4 gives the reason, its *How well HPR Sim agrees* section the
measurements, and [the design page][left-out] states them.

The pull request also left some such stages refused. [ADR-335][adr-335] decision 5 lists them:
a parallel stage, a stage one hangs on, an added mass with no station on parts that weigh
nothing, and a flight that moves or releases a mass or parts through such a stage. M2.3d4's
*done when* has no such stage held back.

The roadmap's archive then showed M2.3d4 checked, with the inertia gap in its result line. A
checked milestone whose *done when* has a clause that isn't met reads as done. Each gap needs its
own entry in the roadmap, with a *done when* that fails today.

**Decision.**

1. **Four increments** (three until the amendment of 2026-11-11, below). M2.3d4 keeps its
   *done when* as written, and is met when all four are.
   - **M2.3d6, a stage's added mass.** *Done when:* an `.ork` stage whose mass override doesn't
     cover its parts is read as an added mass and flies: the reader no longer holds it back, and
     only what ADR-335 decision 5 refuses is still held; on probe designs its mass and center
     are held to OpenRocket 24.12's, and its inertia too where neither of the stage's overrides
     covers the parts. [ADR-335][adr-335] carried this out, and it is done.
   - **M2.3d7, the inertia beside an override that covers the parts.** *Done when:* M2.3d4's
     *done when*, whole: what is left of it here is the inertia of a stage with one override
     that covers its parts beside one that doesn't, held to OpenRocket 24.12's, within the
     bounds ADR-336 decision 2 gives, on the eight probe designs where it runs from 11.04%
     under to 400.5% over today ([#1407][i1407]). Amended 2026-11-11: on the six of them that
     aren't on the lower of two stages; those two are M2.3d9's.
   - **M2.3d8, the stages an added mass is still refused on.** *Done when:* M2.3d4's
     *done when*, whole: what is left of it here is each `.ork` stage ADR-335 decision 5 still
     refuses or holds back, flown as OpenRocket 24.12 weighs it, its mass, center and inertia
     held to OpenRocket's on probe designs.
   - **M2.3d9, the inertia on a lower stage and with a motor aboard** (added 2026-11-11). *Done when:* M2.3d4's
     *done when*, whole. What is left of it here is twofold. On the two probe designs with the
     override on the lower of two stages, both inertias are held to OpenRocket 24.12's within
     3 parts in a million, as M2.3d7's *done when* held them before the amendment; on
     2026-11-11 the inertia across the axis is 4.95% and 3.33% over. And on the motor's
     airframe, under an added mass beside a center that covers the parts, the inertia across
     the axis with the motor aboard, at launch and at burnout, is held to OpenRocket's within
     3 parts in a million, the motor's own term taken from OpenRocket's record; on 2026-11-11
     it is 1.6% and 1.1% under ([#1424][i1424]).
2. **The eight probes, what they measured on 2026-11-10, and M2.3d7's bounds.** Four are on the one-stage
   rocket, two have the override on the lower of two stages, and two are on the airframe with a
   motor aboard, one of them with one pod. The table gives HPR Sim's inertia against
   OpenRocket's, as a test pinned it. Its first and third rows are two probes each, and its second
   and fourth are one each.

   | The stage | Inertia about the axis | Inertia across the axis |
   |---|---:|---:|
   | a center that covers the parts beside an added mass, one stage | within 3 parts in a million | 11.04% under |
   | the same on the lower of two stages (measured on the whole rocket) | within 3 parts in a million | 1.5% over |
   | a mass that covers the parts beside a center that doesn't, one stage | 400.5% over | 345.3% over |
   | the same on the lower of two stages (measured on the whole rocket) | 255.8% over | 18.5% over |

   On the airframe with a motor aboard, under an added mass beside a center that covers the
   parts, OpenRocket adds 0.00070 kg·m² across the axis where HPR Sim adds none, and
   0.00035 kg·m² on the one with one pod. That is 2.4% and 1.2% of OpenRocket's inertia across
   the axis on each of those probes (its structure with the override, without the motor).

   M2.3d7's bounds are the ones the probes beside these hold today. M2.3d7 holds each inertia
   on the six probes of the table within 3 parts in a million of OpenRocket's, as the 19 do
   today. On the two airframe probes it holds the inertia the override adds within 1e-9 kg·m²
   of what it adds in OpenRocket, as that airframe's other probes do today. Since the
   amendment, M2.3d7 holds the four one-stage probes of the table, and M2.3d9 the two
   lower-stage ones, each to the same 3 parts in a million.
3. **M2.3d7 has to change a standing rule, and that is its work.** The departures are
   [ADR-061][adr-061]'s rule, not the added mass, where the probes show it: on the one-stage
   probe, each tool's inertia under a center that covers the parts beside an added mass is the
   one it gives under that center alone, and under a mass that covers the parts beside a center
   that doesn't, OpenRocket weighs the stage as it does both covering. The lower-stage and
   airframe probes have no center alone to compare with. So meeting M2.3d7 means amending
   ADR-061's rule for a stage's overrides in a record of its own, with the same probes.
   [ADR-335][adr-335] decision 4 argues that OpenRocket's rule under a mass that covers the
   parts ignores the mass. M2.3d7 has to answer that argument, since the *done when* stands as
   ADR-334 wrote it. This record doesn't choose the new rule.
4. **The order.** M2.3d5, the Macie XL predictions, goes next. The Macie XL stage's two
   overrides each have their flag false ([ADR-334][adr-334]'s Context), so it needs M2.3d6 alone. M2.3d7
   and M2.3d8 follow, then M2.3d9 (amended 2026-11-11), and M2.3d2, the errors, stays last, as ADR-334 put it. M2.3d1 is met when
   M2.3d5 and M2.3d4 both are.

**Amended 2026-11-11.** [ADR-340][adr-340] chose the rule decision 3 left open: a `.ork`
stage's covering override leaves its parts in place. That meets M2.3d7 on six of its eight probes. On the two with the
override on the lower of two stages, ADR-340's measurement (decision 4 draws the conclusion)
shows OpenRocket's figure is below
what any stage that keeps its parts in place can have: OpenRocket sums each part about the
rocket's center, and it counts the override's mass as a point at the stage's center. Those two
are a different piece of work, so they become M2.3d9, with M2.3d7's *done when* on them
unchanged, and M2.3d7 is met on the other six. HPR Sim's rule, worked by hand from OpenRocket's records, leaves the
motor's airframe, under an added mass beside a center that covers the parts, 1.6% and 1.1% under OpenRocket's inertia across the axis with the motor aboard, a figure no increment
held, so M2.3d9 holds it too. Meeting M2.3d9 means composing such a stage's inertia at
the rocket level, as OpenRocket does (each part about the rocket's center, not the stage as one
body), in a record that answers ADR-340 decision 4. This
record is amended in place, not narrowed by a new one, since it plans every increment of
M2.3d4; its title and file name keep "in three" so that links to it hold.

**Consequences.**

- The roadmap shows M2.3d4 open again, under M2.3d1, with M2.3d7 and M2.3d8 below it. The
  archive shows M2.3d6 done, with a result line for the part of M2.3d4's result that was met.
  From 2026-11-11 the roadmap shows M2.3d8 and M2.3d9 below it; M2.3d7 merged in #1422.
- Nothing in the code or the guide's pages changes with this record. The pages already say where
  the inertia is not OpenRocket's and what is refused. The amendment of 2026-11-11 adds to
  each of two pages that closing the lower-stage gap, and the motor-aboard gap, is open work, M2.3d9.
- [#1407][i1407] is M2.3d7's issue, and closes with it.

**How it was checked.** Five of the tests behind M2.3d6 are named on
[the `.ork` page][ork-stage]; the sixth, the refusal's, is in `hpr_validate::openrocket`.
`a_stage_override_not_covering_its_parts_adds_to_them` holds the 19 probes.
`a_stage_override_covering_its_parts_beside_one_that_does_not`,
`a_motor_aboard_does_not_move_a_stages_added_mass` and
`a_stages_added_mass_sits_among_its_pods_and_level_with_their_center` hold the mass and the
center of the other 17. `sim_flies_a_ork_stage_whose_override_adds_to_its_parts` flies one
through `hpr sim` with no warning, and `a_stages_added_mass_on_parts_that_weigh_nothing_is_refused`
holds one of ADR-335 decision 5's refusals. The same tests pin the eight departures above (six
of the seven the second test names, and the two added inertias), so M2.3d7's *done when* failed
on 2026-11-10, when this record was written. Decision 5's refusals stand in the code, so M2.3d8's does too. As of 2026-11-11,
`STAGE_WHOLE_BELOW` pins the lower-stage probes at 4.95% and 3.33% over, and
`openrockets_inertia_with_a_motor_aboard_is_the_same_sum` pins the motor-aboard sum at -0.016
and -0.011, so M2.3d9's fails too.
