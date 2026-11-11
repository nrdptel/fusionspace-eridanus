# ADR-336: M2.3d4 in two, the inertia beside a covering override last (2026-11-10)

- **Status:** accepted; splits [M2.3d4][m2-3d4] into [M2.3d6][m2-3d6], which [ADR-335][adr-335] carried out, and [M2.3d7][m2-3d7], each with its own *done when*, M2.3d4's *done when* whole across the two, and so plans every remaining increment of M2.3d4. This record puts [M2.3d5][m2-3d5] before M2.3d7, where [ADR-334][adr-334] had put all of M2.3d4 before it, and [M2.3d2][m2-3d2] stays last, as ADR-334 put it.
- **Summary:** M2.3d4's *done when* holds a stage's inertia to OpenRocket's, and the merged work doesn't meet that on eight probe designs. A stage with one override that covers its parts beside one that doesn't keeps HPR Sim's own inertia rule there, from 11.04% under OpenRocket's to 400.5% over. The milestone had been recorded as done with that gap written beside it. This record reopens it and splits it in two. M2.3d6 is what merged: a stage's added mass. M2.3d7 is the inertia on those eight probes, and it carries M2.3d4's *done when* whole. M2.3d4 is met when M2.3d7 is.

[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d5]: ../decisions-and-roadmap.md#m2-3d5
[m2-3d6]: ../decisions-and-roadmap.md#m2-3d6
[m2-3d7]: ../decisions-and-roadmap.md#m2-3d7
[adr-061]: 0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
[adr-334]: 0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[i1407]: https://github.com/nrdptel/fusionspace-eridanus/issues/1407
[pr1408]: https://github.com/nrdptel/fusionspace-eridanus/pull/1408
[left-out]: ../physics/design.md#a-stages-added-mass-leaves-out
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
[ADR-335][adr-335] decision 4 gives the reason and the measurements, and
[the design page][left-out] states them.

The roadmap's archive then showed M2.3d4 checked, with the gap in its result line. A checked
milestone whose *done when* has a clause that isn't met reads as done. The gap needs its own
entry in the roadmap, with a *done when* that fails today.

**Decision.**

1. **Two increments.** M2.3d4 keeps its *done when* as written, and is met when both are.
   - **M2.3d6, a stage's added mass.** *Done when:* an `.ork` stage whose mass override doesn't
     cover its parts is read as an added mass and flies, held back no longer by the reader, but
     for what ADR-335 decision 5 refuses; on probe designs its mass and center are held to OpenRocket 24.12's, and its inertia too where
     neither of the stage's overrides covers the parts. [ADR-335][adr-335] carried this out, and
     it is done.
   - **M2.3d7, the inertia beside an override that covers the parts.** *Done when:* M2.3d4's
     *done when*, whole: what is left of it is the inertia of a stage with one override that
     covers its parts beside one that doesn't, held to OpenRocket 24.12's on the eight probe
     designs where it runs from 11.04% under to 400.5% over today ([#1407][i1407]).
2. **The eight probes, and what they measure today.** Four are on the one-stage rocket, two have
   the override on the lower of two stages, and two are on the airframe with a motor aboard, one
   of them with one pod. The table gives HPR Sim's inertia against OpenRocket's, as a test pins it.

   | The stage | Inertia about the axis | Inertia across the axis |
   |---|---:|---:|
   | a center that covers the parts beside an added mass, one stage | within 3 parts in a million | 11.04% under |
   | the same on the lower of two stages (the rocket's inertias) | within 3 parts in a million | 1.5% over |
   | a mass that covers the parts beside a center that doesn't, one stage | 400.5% over | 345.3% over |
   | the same on the lower of two stages (the rocket's inertias) | 255.8% over | 18.5% over |

   On the airframe with a motor aboard, under a mass beside a center that covers the parts,
   OpenRocket adds 0.00070 kg·m² across the axis where HPR Sim adds none, and 0.00035 kg·m² on
   the one with one pod: 2.4% and 1.2% of OpenRocket's inertia across the axis there. M2.3d7 is
   met when each of these is within the bound the other 19 probes hold, 3 parts in a million.
3. **M2.3d7 has to change a standing rule, and that is its work.** The departures are
   [ADR-061][adr-061]'s rule, not the added mass, where the probes show it: on the one-stage
   probe, each tool's inertia under a center that covers the parts beside an added mass is the
   one it gives under that center alone, and under a mass that covers the parts OpenRocket
   weighs the stage as it does both covering. The lower-stage and airframe probes have no
   center alone to compare with. So meeting M2.3d7 means
   amending ADR-061's rule for a stage's overrides in a record of its own, with the same
   probes. [ADR-335][adr-335] decision 4 argues that OpenRocket's rule under a mass that covers
   the parts ignores the mass. M2.3d7 has to answer that argument, since the *done when* stands
   as ADR-334 wrote it. This record doesn't choose the new rule.
4. **The order.** M2.3d5, the Macie XL predictions, goes next. The Macie XL stage's two
   overrides each have their flag false ([ADR-334][adr-334]), so it needs M2.3d6 alone. M2.3d7
   follows, and M2.3d2, the errors, stays last, as ADR-334 put it. M2.3d1 is met when M2.3d5 and
   M2.3d4 both are.

**Consequences.**

- The roadmap shows M2.3d4 open again, under M2.3d1, with M2.3d7 below it. The archive shows
  M2.3d6 done, with the result line M2.3d4 carried.
- Nothing in the code or the pages changes with this record. The pages already say where the
  inertia is not OpenRocket's.
- [#1407][i1407] is M2.3d7's issue, and closes with it.

**How it was checked.** The tests behind M2.3d6 are named in [ADR-335][adr-335]:
`a_stage_override_not_covering_its_parts_adds_to_them` holds the 19 probes,
`a_stage_override_covering_its_parts_beside_one_that_does_not`,
`a_motor_aboard_does_not_move_a_stages_added_mass` and
`a_stages_added_mass_sits_among_its_pods_and_level_with_their_center` hold the mass and the
center of the other 17, and `sim_flies_a_ork_stage_whose_override_adds_to_its_parts` flies one
through `hpr sim` with no warning. The same tests pin the eight departures above, so M2.3d7's
*done when* fails today.
