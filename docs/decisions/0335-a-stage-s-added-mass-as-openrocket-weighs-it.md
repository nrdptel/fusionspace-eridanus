# ADR-335: A stage's added mass, as OpenRocket weighs it (2026-11-10)

- **Status:** accepted; carries out [M2.3d4][m2-3d4] under [ADR-334][adr-334]. [ADR-061][adr-061]'s rule for the inertia under an override that covers the parts is unchanged.
- **Summary:** An `.ork` stage whose mass override doesn't cover its parts now flies. OpenRocket 24.12 adds such a mass to what the parts weigh, as a point: at the center of that stage's own parts, or at the stage's center of gravity override when that doesn't cover the parts either. The design holds it as the stage's added mass, and on 19 probe designs HPR Sim's structure matches OpenRocket's: the center within 0.2 µm, the mass and the inertias within 3 parts in a million. A motor aboard doesn't move the added mass.

[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d5]: ../decisions-and-roadmap.md#m2-3d5
[adr-061]: 0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
[adr-095]: 0095-the-single-pre-1-9-override-flag-read-as.md
[adr-334]: 0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[i1225]: https://github.com/nrdptel/fusionspace-eridanus/issues/1225
[i1407]: https://github.com/nrdptel/fusionspace-eridanus/issues/1407
[guide]: https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html
[ork-stage]: ../format/ork.md#the-values-inside-the-tags
[design-added]: ../physics/design.md#a-stages-added-mass
[conventions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/conventions.py
[with-motor]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/stage_added_mass.py
[probe]: ../glossary.md#probe-design

**Context.** A stage's mass and center of gravity overrides each carry a flag that says whether
the override covers the stage's parts. HPR Sim read a stage's override as the whole stage's, which
is what a flag of `true` says. OpenRocket's user guide says a value whose box is unticked "is
added to the calculated value" ([Overrides and Surface Finish][guide]). It says so of the center of
gravity too, which OpenRocket 24.12 itself doesn't do: a center alone that doesn't cover the parts
changes nothing. Since [#1225][i1225] HPR
Sim held such a stage back rather than fly it far too light. The Macie XL design of
[M2.3d5][m2-3d5] has such a stage, so its two predictions wait on this.

The guide doesn't say where the added mass sits, and four probes didn't settle it: they had one
stage that starts at the tip, no motor, and no pairing where one flag is `true` and the other
`false`.

**Decision.**

1. **A stage can carry an added mass.** `hpr-design`'s `Stage` gains `added_mass`: a mass, and
   optionally a station aft of the stage's forward end. It is a point mass added to the stage's
   parts before the stage's overrides apply. With no station it sits at the parts' center of
   mass, so it moves neither the center nor the inertia about it. With a station it sits there on
   the axis, and the stage's center and inertia follow the parallel-axis theorem. The page
   [A stage's added mass][design-added] gives the equations and a worked example.
2. **The `.ork` reader follows OpenRocket 24.12, row by row.** A flag that isn't written reads as
   `false`, beside a written one too, and so does the older single flag saying `false`
   ([ADR-095][adr-095]). The probes measure that in schema 1.10 files, and the older flag in
   schema 1.4; the reader applies it to every schema.

   | The mass override | The center of gravity override | In the design |
   |---|---|---|
   | covers the parts | any, or none | the whole stage's mass, and its center if one is given |
   | doesn't cover them | none | an added mass with no station |
   | doesn't cover them | doesn't cover them | an added mass at the center's station |
   | doesn't cover them | covers the parts | an added mass with no station, under the whole stage's center |
   | none | covers the parts | the whole stage's center |
   | none | doesn't cover them | nothing: OpenRocket ignores it, and the tags are kept for an export |

   No row raises a warning, so the reader no longer holds such a design back; decision 5 lists
   what the layout and a flight still refuse. A flag that says neither `true` nor `false` is still
   warned of where it is read, which holds the design back.
3. **Probes hold every row.** [`conventions.py`][conventions] gains 23 stage [probes][probe]. With
   those it had, each override alone and both together are measured under every pairing of a
   flag that says `true`, says `false` or isn't written. The rest are the older flag, a part with
   an override of its own, a mass of nothing, a station ahead of the tip, parts that weigh
   nothing, and a rocket of two stages with the override on either stage or both. [`stage_added_mass.py`][with-motor] is new:
   four probes with a motor aboard.
4. **The inertia under an override that covers the parts stays hpr's own.** [ADR-061][adr-061]
   keeps hpr's rule there: a center that covers the parts moves the assembly whole, and a mass
   that covers them scales its inertia. Where a stage's added mass sits beside such a center, the
   mass and the center are OpenRocket's and the inertia is hpr's rule, pinned by a test as
   ADR-061's other departures are. Where no override covers the parts, there is no departure.
5. **What is refused, not guessed.**
   - A parallel stage with an added mass, or one hanging on a stage that carries one: where the
     mass sits beside the pods isn't measured. A parallel stage's override in a `.ork` is read as
     a pod set's, as before, and one that doesn't cover its pods is still warned of.
   - An added mass with no station on parts that weigh nothing. OpenRocket reports that stage's
     center at the tip on one probe, which doesn't say whether that is the stage's forward end.
   - A flight with a mass that moves, a mass that is released, or a parting through a stage, where
     that stage carries an added mass. A stage whose mass is overridden is refused the same way.
6. **An export writes it back.** An added mass is written as a mass override whose flag is
   `false`, beside a center override whose flag is `false` when it has a station. A `.ork` has one
   mass tag and one center tag. A design that holds an added mass beside a whole-stage mass, or a
   station beside a whole-stage center, can't be said in one, so the added mass is left out with a
   warning.

**The measurements.** The stage's parts weigh 0.3996 kg, centered 0.4701 m aft of the tip, with an
inertia across the axis of 0.015838 kg·m². The mass override is 2 kg and the center override
0.4 m. OpenRocket 24.12's structure:

| The stage's overrides | Mass (kg) | Center (m) | Inertia across the axis (kg·m²) |
|---|---:|---:|---:|
| none that takes effect (a center alone, flag `false` or not written) | 0.3996 | 0.4701 | 0.015838 |
| mass, flag `false` or not written, or the older flag `false` | 2.3996 | 0.4701 | 0.015838 |
| both, flags `false` or not written, or the older flag `false` | 2.3996 | 0.4117 | 0.017476 |
| mass flag `false` or not written, center flag `true` | 2.3996 | 0.4000 | 0.017803 |
| mass flag `true`, center flag `false` or not written | 2.0000 | 0.4000 | 0.017803 |
| both, flags `true` | 2.0000 | 0.4000 | 0.017803 |

- **The second row** is the parts' center and inertia to every digit: a point at the parts' center.
- **The third row** is a 2 kg point at 0.4 m beside the parts:
  $(0.3996 \times 0.4701 + 2 \times 0.4) / 2.3996 = 0.4117$ m.
- **On a rocket of two stages** the lower stage starts 0.8 m aft of the tip and its one tube is
  centered at 1.05 m. A 2 kg mass override on it that doesn't cover its parts leaves the rocket's
  center at 0.9644 m: the point is at 1.05 m, the lower stage's parts' center, not at the rocket's
  0.7225 m. With a center override of 0.4 m that doesn't cover the parts either, the rocket's
  center is 1.0752 m: the point is at 1.2 m, measured from the stage's forward end.
- **With a motor aboard** (an AeroTech H128W in a different airframe), OpenRocket's rocket at
  launch and at burnout is the motor added to the overridden structure, to 1e-12 in mass and
  center. Had the 2 kg sat at the center of the parts and the motor together, the launch center
  would be more than 5 cm further aft.

**How well HPR Sim agrees.** On the 19 probes where no override covers the parts, its structure is
within the test's bounds of OpenRocket's: 5 parts in 10 million of the mass, 0.2 µm of the center,
and 3 parts in a million of the inertia about the axis and across it. On the seven probes where one
override covers the parts, the mass and the center are within the same bounds, and the inertia
across the axis is hpr's rule: 11.0% under OpenRocket's with a center that covers one stage's
parts and the added mass, 1.5% over on the lower of two stages, and ADR-061's scaled inertia under
a mass that covers them. The added mass brings no difference of its own: on the one-stage probe,
each tool's inertias beside the added mass are the ones it gives under that center alone, to 12
significant digits, so the 11.0% is the same 11.0% a covering center already had. Whether the
added mass should change the rule there is [#1407][i1407].

**What doesn't change.** A stage's one override whose flag says it covers the parts, and two
whose flags both say so, read and fly as before. So do a part's overrides, a pod set's, and every
drag override. No tolerance moved. A stage's mass override that covers the parts beside a center
whose flag is `false` or not written was held back with a warning that the flags disagree. It now
flies, read as both covering the parts, which is exactly how OpenRocket weighs it.

**Consequences.** The Macie XL design can be flown, which [M2.3d5][m2-3d5] needs. Other `.ork`
designs with such a stage fly too, unless decision 5 refuses them, with the mass their author
entered on top of the parts. The
design format has one more optional field, so a design written with an added mass doesn't read in
an older HPR Sim. The gaps left are the refusals in decision 5 and the inertia in decision 4; the
pages [A stage's override][ork-stage] and [A stage's added mass][design-added] list them.
