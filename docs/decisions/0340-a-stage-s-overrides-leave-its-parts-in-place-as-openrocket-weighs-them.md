# ADR-340: A stage's overrides leave its parts in place, as OpenRocket weighs them (2026-11-11)

- **Status:** accepted; carries out [M2.3d7][m2-3d7] under [ADR-336][adr-336], on six of its eight probe designs, and says why the other two can't be met by any stage that keeps its parts in place; ADR-336, amended, gives those two to [M2.3d9][m2-3d9]. It amends [ADR-061][adr-061]'s rule for the inertia under a stage's override that covers its parts; a part's overrides keep ADR-061's rule. It answers [ADR-335][adr-335] decision 4.
- **Summary:** A `.ork` stage whose mass or center of gravity override covers its parts now keeps the parts where the design puts them, as OpenRocket 24.12 does. Its inertia is the parts' about the stage's center. Before, HPR Sim moved and rescaled the stage as one body. On six of M2.3d7's eight probes the inertia now matches OpenRocket's within ADR-336's bounds; it ran from 11.04% under to 400.5% over. On the two with the override on a lower stage, OpenRocket's figure is below what the stage's parts alone have about its center: the rocket's inertia across the axis is now 4.95% and 3.33% over (it was 1.5% and 18.5%), pinned. The collection's mean absolute apogee error goes from 12.07% to 12.04%.

[m2-3d7]: ../decisions-and-roadmap.md#m2-3d7
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d9]: ../decisions-and-roadmap.md#m2-3d9
[adr-061]: 0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
[adr-329]: 0329-m7-3d6-the-collections-errors-by-halves.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[adr-336]: 0336-m2-3d4-in-three-the-inertia-and-the-refusals-last.md
[i1407]: https://github.com/nrdptel/fusionspace-eridanus/issues/1407
[i1424]: https://github.com/nrdptel/fusionspace-eridanus/issues/1424
[conventions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/conventions.py
[with-motor]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/stage_added_mass.py
[design-overrides]: ../physics/design.md#overrides
[ork-stage]: ../format/ork.md#a-stages-mass-and-center-of-gravity-overrides
[probe]: ../glossary.md#probe-design

**Context.** M2.3d7's *done when* is the rest of [M2.3d4][m2-3d4]'s: the inertia of a stage with
one override that covers its parts beside one that doesn't, held to OpenRocket 24.12's on eight
[probe designs][probe]. The bounds ([ADR-336][adr-336] decision 2) are 3 parts in a million on
each inertia of six of them, and 1e-9 kg·m² on the inertia the override adds on the two with a
motor aboard. [ADR-336][adr-336] decision 3 said meeting it means amending ADR-061's rule for a
stage's overrides, and answering ADR-335 decision 4's argument for keeping it.

ADR-061's rule treats a stage's parts as one body. A mass override that covers them scales the
body's inertia with the mass, and a center override that covers them moves the body whole.
OpenRocket leaves each part where the design puts it.

**What OpenRocket does, measured.** The probes are [`conventions.py`][conventions]'s and
[`stage_added_mass.py`][with-motor]'s, already in the repository.

- **On one stage**, every pairing of overrides where one covers the parts gives the parts' inertia
  about the stage's center, and nothing more. The one-stage probe's parts weigh 0.3996 kg,
  centered 0.4701 m aft of the tip, with 0.015838 kg·m² across the axis. A center override of
  0.4 m gives 0.015838 + 0.3996 × 0.0701² = 0.017802 kg·m² by hand, OpenRocket's 0.017803 kg·m²
  (the hand sum rounds the distance). The same figure
  comes back with a 2 kg mass override that covers the parts beside it, so that mass adds no
  inertia. It comes back with a 2 kg added mass beside it too: the added mass sits at the
  center, not at the parts' center. About the axis, the parts' inertia is kept, never scaled.
- **On the lower of two stages**, OpenRocket's rocket inertia across the axis is, to 1e-12, a sum
  about the rocket's center $X$. Every part counts at its own place, and the stage's override
  mass $m_o$, 2 kg, counts as a point at the stage's stated center $c$:
  $I = \sum_i \left(I_i + m_i (x_i - X)^2\right) + m_o (c - X)^2$. On one stage $X = c$, and the sum
  is the parts' inertia about $c$ above. Below another stage it is less than that. Read as the
  lower stage's own inertia about $c$, it is 0.003761 and 0.007033 kg·m² on the two probes, where
  the stage's tube alone has 0.013711 kg·m² about $c$. A stage that keeps the tube in place has
  at least that, whatever the rest of its mass does, so no such stage gives OpenRocket's figure:
  - Under a mass override that covers the parts, $m_o$ is the whole stage's mass, so the sum
    counts the lower tube's 0.31 kg twice. It holds 2.71 kg in a rocket that weighs 2.40 kg.
  - Under a center that covers the parts beside an added mass, the masses add up, but they
    don't balance at $c$. The lower stage's own inertia across the axis would be 0.003761 kg·m². Its tube alone has 0.006784 kg·m² about the tube's own
    center, and no body that holds the tube has less.
- **With a motor aboard**, OpenRocket's inertia at launch and at burnout is the same sum, the
  motor's own term added, to 1e-12 on the three motor probes. On their airframe, under an added
  mass beside a center that covers the parts, HPR Sim's rule, the parts' inertia about the
  stated center, is 1.6% under
  OpenRocket's inertia across the axis at launch and 1.1% under at burnout, worked from
  OpenRocket's own records with its motor's term in both. ADR-061's rule, worked by hand from the same
  records and not pinned, was 3.1% and 2.9% under.

**Decision.**

1. **A stage carries a rule for its overrides.** `hpr-design`'s `Stage` gains `override_rule`,
   with two values. `rescaled` is ADR-061's rule and the default. `parts_in_place` keeps the
   parts where the design puts them:
   - the stage's mass and center are the overrides', as under `rescaled`;
   - its inertia about that center $c$ is its parts' about it, by the parallel-axis theorem,
     $I = I_p + m_p \left((\mathbf{d}\cdot\mathbf{d})\,\mathbf{1} - \mathbf{d}\,\mathbf{d}^\mathsf{T}\right)$,
     $\mathbf{d} = \mathbf{x}_p - \mathbf{c}$;
   - the mass a mass override adds or takes away, and an added mass with no station, sit at
     $c$ and add nothing;
   - an added mass at a station is one of the parts, and a stated inertia replaces the tensor.

   The key is optional and written only when it isn't the default, so a design written
   without it reads as before. It joins format 0.2 in place, as `added_mass` did
   ([ADR-335][adr-335]).
2. **The `.ork` reader gives `parts_in_place` to every stage on the axis whose override covers
   its parts.** That is a center alone, a mass alone, both, and each pairing where one covers the
   parts and the other doesn't. Taking the two mixed pairings alone, as M2.3d7 names them, would
   split stages OpenRocket weighs the same. A mass that covers the parts beside a center whose
   flag is `false` weighs exactly as both covering. A design could then fly two inertias on a
   flag OpenRocket ignores: on the one-stage probe, 5.0 times apart about the axis and 4.5 times
   across it. And an added mass of 1 g beside a center would raise the stage's inertia across
   the axis by 12%. A stage with no override, or only an added mass, keeps the default: the rule
   changes nothing there. A parallel stage's override is read as a pod set's, as before.
3. **ADR-335 decision 4's argument, answered.** It argued that holding HPR Sim to OpenRocket under
   a mass that covers the parts "would trade a rule that follows the mass for one that ignores
   it". Three things outweigh that for a `.ork`.
   - Reading a `.ork` as OpenRocket reads it is ADR-061's own principle. The file's author tuned
     the design in OpenRocket and saw OpenRocket's numbers.
   - Neither rule is measured on a flying rocket. A stage's overrides say what it weighs and
     where it balances. They don't say where the mass the parts miss sits.
   - Keeping the parts where the design puts them is the argument ADR-061 made for the center
     under a mass that covers the parts: a builder who weighs a tube with its fins and mount
     inside hasn't moved them. Moving the stage whole moves parts the designer placed.

   The cost is real and stays on the pages. Under `parts_in_place` the mass the parts miss adds
   no inertia. If it is spread along the stage as the parts are, the true inertia is higher by
   the mass ratio under a mass override: 5.0 times about the axis on the one-stage probe, where
   2 kg covers 0.40 kg of parts (4.5 times across it, where the center override also moves the
   parts' center). If it sits farther out, higher still. A design in HPR Sim's own format keeps
   `rescaled` unless it asks for the other.
4. **The two lower-stage probes keep a difference, pinned.** No stage that keeps its parts in place
   gives OpenRocket's figure there (above), so HPR Sim doesn't reproduce it. Its rule is the one OpenRocket's sum
   reduces to when the stage's center is the point the inertia is taken about. The rocket's
   inertia across the axis is 4.95% over OpenRocket's under the center beside an added mass,
   and 3.33% over under a mass that covers the parts, beside a center that doesn't. About the axis
   the two agree. Before, across the axis, they were 1.5% and 18.5% over; about the axis the
   second was 255.8% over.
   `openrockets_lower_stage_sum_is_below_its_parts_in_place` reproduces OpenRocket's sum and
   checks each reason; `openrockets_inertia_with_a_motor_aboard_is_the_same_sum` reproduces the
   sum with a motor aboard and pins the 1.6% and 1.1%.
5. **A Monte Carlo run disperses such a stage as any other.** Its dry-mass scale and center
   shift work by writing the stage's overrides. On a `parts_in_place` stage that would keep the
   parts' inertia under a new mass. So the run states the stage's nominal inertia, scaled,
   on each dispersed `parts_in_place` stage that states none (a `rescaled` stage scales its
   own), and the dispersion's model is the same for every stage.
6. **An export says so.** A `rescaled` stage with a mass or center override and no stated
   inertia is written as before, with a warning: OpenRocket, and HPR Sim reading the file
   back, weigh it with its parts in place.

**How well HPR Sim agrees.**

- **The eight one-stage probes** (M2.3d7's four, a center alone, and the three of a stage's
  overrides covering the parts that ADR-061 pinned): mass within 5 parts in 10 million, center
  within 0.2 µm, both inertias within 3 parts in a million of OpenRocket's. On each, the old
  rule is more than 10% from OpenRocket's inertia across the axis, so the test can tell the two
  apart.
- **M2.3d7's two airframe probes:** the inertia the override adds is within 1e-9 kg·m² of
  OpenRocket's 0.00070 and 0.00035 kg·m², where the old rule added none.
- **The two lower-stage probes:** mass and center within the same bounds, roll within 3 parts in
  a million, pitch 4.95% and 3.33% over, as above.
- **The reports.** On the private collection's 60 logged flights, HPR Sim's mean absolute
  apogee error goes from 12.07% to 12.04% and its mean error from +7.76% to +7.72%; against
  OpenRocket its median apogee difference goes from +0.62% to +0.53%. Six of the 54 public
  `.ork` flights compared with OpenRocket move, and six of the 35 private library flights
  (`openrocket-flights.json` and `openrocket-library-flights.json` in `validation/reports/`). Each
  apogee moves by less than 0.001 percentage points, and the drift at apogee on the public
  flights by less than 0.13 m. The
  held-out half's sealed digest changes with its aggregates, as [ADR-329][adr-329] decision 7
  expects.

**What doesn't change.** A part's overrides, and a pod set's, keep ADR-061's rule and its pinned
departures. So does a design in HPR Sim's own format, whose stages default to `rescaled`. A
stage with no override that covers its parts weighs as before, on all 19 of ADR-335's probes.
No tolerance moved.

**Consequences.** M2.3d7 is met on six of its eight probes. On the other two, this record shows
the target is below what any stage that keeps its parts in place has, and the difference stays
pinned and on the pages. [ADR-336][adr-336], amended in place, gives those two to
[M2.3d9][m2-3d9] ([#1424][i1424]), with M2.3d7's *done when* on them unchanged: meeting it means
composing such a stage's inertia at the rocket level, as OpenRocket does, in a record that
answers decision 4. [#1407][i1407] closes with M2.3d7. [M2.3d4][m2-3d4] is met when M2.3d8, the
refusals, and M2.3d9 are. With a motor aboard, the same sum leaves HPR Sim's inertia across
the axis 1.6% under OpenRocket's at launch and 1.1% under at burnout, pinned; that is outside
M2.3d7's *done when*, which holds the inertia the override adds on the motor's airframe, and the
same composition at the rocket level would close it, so [#1424][i1424] tracks it too. The pages
[Overrides][design-overrides] and [A stage's override][ork-stage] say what the rule is.
