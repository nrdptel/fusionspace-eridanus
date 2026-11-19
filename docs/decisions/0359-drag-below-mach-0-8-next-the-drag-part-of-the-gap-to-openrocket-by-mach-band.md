# ADR-359: Drag below Mach 0.8 next: the drag part of the gap to OpenRocket, by Mach band (2026-11-19)

- **Status:** accepted; adds [M1.14i][m1-14i] to [M1.14][m1-14] as three increments, [M1.14i1][m1-14i1], [M1.14i2][m1-14i2] and [M1.14i3][m1-14i3], each with its own *done when*; puts M1.14i ahead of [M1.14b][m1-14b]'s open increments, whose order among themselves stays [ADR-357][adr-357]'s and [ADR-358][adr-358]'s; no *done when* of M1.14 or of another increment changes
- **Summary:** Split by the Mach number the drag acts at, the drag part of the gap to OpenRocket 24.12 on the private collection's 68 flights is largest below Mach 0.8: a mean absolute 0.87% of the apogee there, 0.50% from Mach 0.8 to 1.2 and 0.23% above Mach 1.2. Flown on OpenRocket's drag below Mach 0.8 alone, HPR Sim's mean absolute error against the 60 logged apogees falls from 12.04% to 11.65%; on its whole drag, to 11.27%, beside OpenRocket's 11.26%. No increment of M1.14 owned drag below Mach 0.8. M1.14i does, and goes next.

[m1-14]: ../decisions-and-roadmap.md#m1-14
[m1-14b]: ../decisions-and-roadmap.md#m1-14b
[m1-14b10]: ../decisions-and-roadmap.md#m1-14b10
[m1-14c]: ../decisions-and-roadmap.md#m1-14c
[m1-14d]: ../decisions-and-roadmap.md#m1-14d
[m1-14f]: ../decisions-and-roadmap.md#m1-14f
[m1-14i]: ../decisions-and-roadmap.md#m1-14i
[m1-14i1]: ../decisions-and-roadmap.md#m1-14i1
[m1-14i2]: ../decisions-and-roadmap.md#m1-14i2
[m1-14i3]: ../decisions-and-roadmap.md#m1-14i3
[m2-3c12]: ../decisions-and-roadmap.md#m2-3c12
[m2-3c13]: ../decisions-and-roadmap.md#m2-3c13
[adr-097]: 0097-a-cause-in-the-drag-sized-by-hpr-flying.md
[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[adr-349]: 0349-m2-3c6-in-four-the-public-nose-probe-first.md
[adr-350]: 0350-m2-3c13-the-nose-s-line-and-why-222-s-flight-is-not-in-it.md
[adr-357]: 0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
[adr-358]: 0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md
[report]: ../../validation/reports/fixture-flights.md
[report-json]: ../../validation/reports/fixture-flights.json
[page]: ../accuracy.md#the-drag-part-by-mach-band
[tests]: ../../xtask/src/drag_bands/tests.rs
[i-18]: https://github.com/nrdptel/fusionspace-eridanus/issues/18
[i-68]: https://github.com/nrdptel/fusionspace-eridanus/issues/68
[i-69]: https://github.com/nrdptel/fusionspace-eridanus/issues/69
[i-73]: https://github.com/nrdptel/fusionspace-eridanus/issues/73
[i-222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
[i-633]: https://github.com/nrdptel/fusionspace-eridanus/issues/633

**Context.** [M1.14][m1-14]'s *done when* asks that HPR Sim be at least as accurate as
OpenRocket on the same real flights. On the private collection's 60 logged flights it is not:
its mean absolute apogee error is 12.04% and OpenRocket 24.12's is 11.26%
([report][report]). [The split by cause][m2-3c12] ([ADR-349][adr-349]) put the gap between the
two codes in the drag: of the 68 flights both fly, the drag part dominates 59, with a mean
absolute value of 1.57% of OpenRocket's apogee. [The nose's line][m2-3c13]
([ADR-350][adr-350]) sized one difference inside it, on the 5 flights that pass Mach 1 with a
tangent-ogive nose. The rest of the drag part was not located.

M1.14's increments divide the drag by Mach number. [M1.14c][m1-14c] is Mach 0.8 to 1.2 and
[M1.14d][m1-14d] is Mach 1.2 to 2.5. None is drag below Mach 0.8. [M1.14b][m1-14b]'s open
increments, next in the order until this record, are tip-off, wind, a validation uncertainty
and a second oracle. None of them acts on drag.

The maintainer's plan of 2026-11-18 called for a measurement before any reordering: size the
drag part by Mach band on the split's flights, and put the step that owns the largest band
next, by a decision record. This is that record. Every figure below is in
[the committed report][report] ([as data][report-json]), which `cargo xtask fixture-flights
--check` reproduces on the machine that holds the collection, and whose section a test holds to
itself in CI.

**Decision.**

1. **The measurement.** On OpenRocket's thrust and mass, HPR Sim flies with OpenRocket's drag
   coefficient in place of its own below Mach 0.8 only, then below Mach 1.2, then at every Mach
   number. A band's part is the apogee before its swap less the apogee after it, over
   OpenRocket's apogee. The three sum to the split's drag part on every flight, and the run
   stops where they don't. All 68 flights split are split by band.

   | band | flights with a part in it | mean | median | mean absolute | mean absolute over those flights | flights it dominates | flights where it is over 1% |
   |---|---:|---:|---:|---:|---:|---:|---:|
   | below Mach 0.8 | 68 | +0.15% | +0.19% | 0.87% | 0.87% | 45 | 16 |
   | Mach 0.8 to 1.2 | 28 | +0.20% | +0.00% | 0.50% | 1.21% | 16 | 9 |
   | above Mach 1.2 | 7 | +0.23% | +0.00% | 0.23% | 2.21% | 7 | 7 |
   | the drag part | 68 | +0.58% | +0.44% | 1.57% | | | |

   A positive part means HPR Sim climbs higher on its own drag in that band than on
   OpenRocket's: its drag there is the lower of the two over that flight.

   Each band is also swapped alone, on HPR Sim's own thrust and mass, and those flights are set
   beside the logs of the 60 flights compared. Each row is the apogee less the logged one, over
   the logged one.

   | HPR Sim's drag | flights moved | mean | median | mean absolute | flights nearer their log | flights farther from it |
   |---|---:|---:|---:|---:|---:|---:|
   | its own | | +7.72% | +6.69% | 12.04% | | |
   | OpenRocket's below Mach 0.8, its own above | 60 | +7.42% | +6.70% | 11.65% | 39 | 21 |
   | OpenRocket's from Mach 0.8 to 1.2, its own elsewhere | 25 | +7.44% | +6.78% | 11.79% | 14 | 11 |
   | OpenRocket's above Mach 1.2, its own below | 7 | +7.44% | +5.99% | 11.86% | 6 | 1 |
   | OpenRocket's at every Mach number | 60 | +6.85% | +6.33% | 11.27% | 33 | 27 |
   | OpenRocket's own flight, for comparison | 60 | +6.72% | +6.08% | 11.26% | 36 | 24 |

2. **Below Mach 0.8 owns the most, by each of three measures.**
   - **The mean absolute part:** 0.87% of the apogee, against 0.50% and 0.23%.
   - **The flights it dominates:** 45 of 68, against 16 and 7.
   - **What its swap takes off HPR Sim's mean absolute error against the logs:** 0.39 points
     (12.04% to 11.65%), against 0.25 (to 11.79%) and 0.18 (to 11.86%).

   The three agree, so no rule is needed to choose among them. Had they disagreed, the third
   would have decided: it is the one measured against the flights, which M1.14's *done when*
   is written on.

   The faster bands are not small where they act. Over the flights with a part in them, the
   mean absolute part is 1.21% from Mach 0.8 to 1.2 and 2.21% above Mach 1.2, against 0.87%
   below Mach 0.8. They rank lower because fewer of the collection's flights reach them: 28
   and 7 of 68. They stay [M1.14c][m1-14c]'s and [M1.14d][m1-14d]'s, in their places.

   The signed means say less than the absolute ones. The drag part's mean of +0.58% is spread
   over the three bands, +0.15%, +0.20% and +0.23%, and below Mach 0.8 the parts are of both
   signs.

3. **M1.14i, drag below Mach 0.8, in three increments.** M1.14i is met when the three are.
   - **M1.14i1, the drag part by Mach band.** *Done when:*
     - the fixture report splits the drag part of every flight it can into three bands of Mach
       number, below 0.8, 0.8 to 1.2 and above 1.2, by swapping OpenRocket's drag coefficient
       for HPR Sim's own in one band after another, on OpenRocket's thrust and mass; the three
       are held to sum to the drag part on every flight split by band, and the run stops on a
       flight where they don't;
     - over the flights split by band it gives each band's mean, median and mean absolute part,
       the flights with a part in it, the flights it dominates and the flights where it is over
       1%, and each band swapped alone on HPR Sim's own thrust and mass;
     - over the flights compared with a log it gives HPR Sim's error against the logs with each
       band swapped alone, beside its own, the whole drag's and OpenRocket's own, each with the
       flights it moves nearer their log and farther;
     - a flight split that HPR Sim fails with a band swapped is counted, and at least half the
       flights split are split by band;
     - no size is given unless every aggregate is over no flights or at least three
       ([ADR-151][adr-151]);
     - tests hold the swap to OpenRocket's table inside its bands and to HPR Sim's own buildup
       outside them, bit for bit, hold the two public designs' bands to sum to their drag parts
       and pin them, and hold each count and size of the section on flights worked by hand;
       `fixture-flights --check` holds the section, and a test that runs in CI holds the
       committed section to itself;
     - the accuracy page says what the bands are and how far to trust them;
     - and this record names the band that owns the most, by measures it states, and puts the
       increment of M1.14 that owns it next.
   - **M1.14i2, the part below Mach 0.8 by term, and a measurement for the largest.** *Done
     when:*
     - the OpenRocket oracle that records the histories also records OpenRocket 24.12's
       friction, pressure and base drag coefficients along each flight, as its own flight data
       gives them, on the collection and on the two public designs;
     - the fixture report splits the part below Mach 0.8 into friction, pressure, base and a
       rest, the rest being OpenRocket's drag coefficient less its three terms: each of HPR
       Sim's terms is swapped for OpenRocket's below Mach 0.8, one after another in that
       order, and the four are held to sum to the band's part on every flight so split;
     - it gives each term's mean, median and mean absolute part, the flights it dominates, and
       HPR Sim's error against the logs with each term swapped alone; a flight not split by
       term is counted under a reason from a fixed list; the floor of three flights holds as
       in M1.14i1;
     - tests on the two public designs hold each swap to replace its own term and nothing
       else, hold the terms to sum, and pin both codes' terms at Mach 0.3 and 0.6;
     - a measured reference of the term with the largest mean absolute part, from a wind
       tunnel or a flight and not from another program, is added to the validation set in that
       pull request: cited, with its stated uncertainty, and with HPR Sim's error on each of
       its rows below Mach 0.8 in the validation report. If the validation set already holds
       every such reference a search finds, a decision record lists the search, and the
       increment then counts as no progress under the stop rule ([ADR-143][adr-143]);
     - the accuracy page and the aerodynamics page say what the terms are, what the reference
       shows and how far to trust both.
   - **M1.14i3, the largest term's model, changed or kept on the measurement.** *Done when:*
     - where HPR Sim's error on M1.14i2's reference is past the reference's stated
       uncertainty, the term's model is changed to a cited source, and its error on the
       reference is smaller after than before, row by row or with each row that grows named;
     - every committed report the change moves is regenerated in that pull request, which
       gives, before and after, the term's part and the band's part of the gap to OpenRocket,
       and HPR Sim's mean and mean absolute error against the collection's logs;
     - the change rests on the reference: no number in it is fitted to the collection's
       flights, whose errors are reported and choose nothing. A correction fitted to them
       would be judged on flights predicted first ([ADR-346][adr-346], decision 6);
     - a test pins which way the changed term errs against the reference, with a mutation
       probe;
     - where the error is within the reference's uncertainty, a decision record gives the
       numbers and the model stays, and the record says what else below Mach 0.8 the term
       split points at.

4. **The order.** M1.14i's open increments come next, ahead of [M1.14b10][m1-14b10] and the
   rest of M1.14b's open increments. Those keep the order [ADR-357][adr-357] and
   [ADR-358][adr-358] gave them, and resume after M1.14i. [M1.14c][m1-14c] and
   [M1.14d][m1-14d] keep their places after M1.14b.

5. **What is not decided: whose drag is right.** A row nearer the logs does not show that
   OpenRocket's drag is the truer one in a band. Both codes climb above the logs on average,
   +7.72% and +6.72%, so more drag from any cause moves the mean toward them, and a wrong mass
   or thrust would read the same way. M1.14i does not make HPR Sim's drag OpenRocket's. Its
   second increment finds which term differs, and its third changes a model only on a
   measurement.

**Consequences.**

- The roadmap gains M1.14i, with M1.14i1 done and M1.14i2 next. M1.14b's tip-off default, wind
  and uncertainty increments wait for M1.14i.
- The fixture report flies six more flights for each flight split.
- **The stop rule.** M1.14i1 is a measurement that orders the work. It neither shrinks a
  measured error against an independent reference nor adds a reference, so under
  [ADR-143][adr-143]'s stop rule it does not count as progress. M1.14i2's *done when* asks for
  a measured reference so that it can.
- The band swap is a tool of the report, in `xtask`. No flight a user runs changes, and every
  other committed report reproduces unchanged.

**What this leaves open.**

- **The parts are code against code,** and the rows against the logs carry the logs' own
  errors. [The accuracy page][page] gives the limits.
- **A band's edge is a step.** Outside the bands swapped the drag is HPR Sim's own, so the
  coefficient steps from one program's to the other's at Mach 0.8 or 1.2. The order matters
  little here: swapped alone, each band's mean absolute part is within 0.01 points of its
  part swapped after the slower bands (0.87%, 0.50% and 0.24% against 0.87%, 0.50% and
  0.23%).
- **OpenRocket's coefficient is a table in Mach number** read along its own flight, power on
  and power off. It leaves out how OpenRocket's drag changes with the Reynolds number and the
  angle of attack away from that path, as [the split][m2-3c12] does.
- **Above Mach 1.2 the 7 flights all have HPR Sim's drag the lower:** the band's mean and
  its mean absolute part are both 0.23%. That is the opposite sign to
  [the nose's line][m2-3c13], where HPR Sim's nose drag is the higher. The two are over
  different flights and Mach numbers; [M1.14d][m1-14d] has both, with
  [#222][i-222].
- **Candidates below Mach 0.8, none yet sized on these flights:** the base under power, where
  HPR Sim takes the burning motor's cross-section off the base and OpenRocket 24.12 keeps the
  whole base ([ADR-097][adr-097], [#633][i-633]); the base-drag rule itself ([#68][i-68]);
  long boattails ([#73][i-73]); laminar friction ([#18][i-18]); and fin-body interference
  ([#69][i-69]). M1.14i2's split by term says which of friction, pressure and base to look in.
  The base under power, laminar friction and interference are also on [M1.14f][m1-14f]'s list
  ([ADR-143][adr-143], decision 6), which keeps them.
- **M1.14i1's *done when* was written with the measurement in hand.** It asks for the
  measurement and the decision, not for a result, and M1.14i2's and M1.14i3's were written
  before either is built.
