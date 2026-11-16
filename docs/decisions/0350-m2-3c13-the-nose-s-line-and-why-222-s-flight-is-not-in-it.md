# ADR-350: M2.3c13's result: the tangent-ogive nose's line, and why #222's flight is not in it (2026-11-15)

- **Status:** accepted; carries out [M2.3c13][m2-3c13] by [ADR-349][adr-349] decisions 2 and 3, its *done when* unchanged; is the decision record that *done when* asks for on [#222][i-222]'s flight; amends one sentence of ADR-349 decision 3, on a repeated nose ([#1484][i-1484]); completes [M2.3c6][m2-3c6].
- **Summary:** Five of the 68 flights both simulators fly pass Mach 1 with a tangent-ogive nose, and all five are swapped. With HPR Sim's pressure drag of the nose replaced above Mach 1 by OpenRocket 24.12's, the nose's line is −1.28% of OpenRocket's apogee on average, inside a drag part of −1.96%. Those flights reach Mach 1.06 on average, so the line is measured just past Mach 1. #222's flight is not among them: it is in the design-library report, which holds no thrust and mass histories, and the line is defined on those. An open issue, #1497, carries its nose-only swap. No flight of the line has a repeated nose, so none was probed.

[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[m2-3c11]: ../decisions-and-roadmap.md#m2-3c11
[m2-3c12]: ../decisions-and-roadmap.md#m2-3c12
[m2-3c13]: ../decisions-and-roadmap.md#m2-3c13
[m1-14]: ../decisions-and-roadmap.md#m1-14
[adr-097]: 0097-a-cause-in-the-drag-sized-by-hpr-flying.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-349]: 0349-m2-3c6-in-four-the-public-nose-probe-first.md
[i-222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
[i-616]: https://github.com/nrdptel/fusionspace-eridanus/issues/616
[i-1484]: https://github.com/nrdptel/fusionspace-eridanus/issues/1484
[i-1497]: https://github.com/nrdptel/fusionspace-eridanus/issues/1497
[forward]: ../../validation/reports/fixture-flights.md
[library]: ../../validation/reports/openrocket-library-flights.md
[aero]: ../physics/aero.md#a-tangent-ogive-past-mach-1-beside-openrocket-2412
[accuracy]: ../accuracy.md#the-tangent-ogive-noses-line

**Context.** [ADR-349][adr-349] split M2.3c6 in four. Its last increment, M2.3c13, splits the
drag part of the gap between HPR Sim's and OpenRocket 24.12's apogees once more: the pressure
drag of tangent-ogive noses above Mach 1. M2.3c10's probe showed that from Mach 1.3 up
OpenRocket 24.12 gives such a nose under 1% of what its documented method gives
([the probe][aero]); between Mach 1 and 1.3 the probe says nothing. Decision 3 fixed how: the
line is taken first inside the drag step, on OpenRocket's thrust and mass, so the line and the
rest sum to the drag part. Only aggregates over three flights or more are published
([ADR-151][adr-151]).

**Decisions.**

1. **The swap is a drag model in the validation harness.** `nose_line::NoseSwap` asks HPR Sim
   for its drag buildup as usual. Above Mach 1 it takes off the pressure drag HPR Sim gives the
   nose and adds OpenRocket's column for it, read against Mach number and moved from
   OpenRocket's reference area to HPR Sim's. At Mach 1 and below, the drag is HPR Sim's own.
   Nothing changes in the simulator's library: a flight takes the model through the hook it
   already has for a caller's own drag model (`Simulation::with_drag_model`). The swap is for
   sizing a difference between two codes, as a drag table is ([ADR-097][adr-097]), not a model
   to fly on.
2. **The oracle** is `validation/oracles/openrocket/nose_columns.py`. For each flight's design
   and configuration it records the pressure drag coefficient OpenRocket's public interface
   reports for the nose cone, at zero angle of attack, from Mach 1.00 to 2.50 every 0.01. It
   is the column M2.3c10's controls identified. A flight either holds a column or lacks it
   under one of five reasons, the list closed. The collection's record stays on this machine;
   the two public designs' record is committed, and a test holds it to the script.
3. **The line's flights** are those both simulators fly whose own HPR Sim flight passes Mach 1
   with a tangent-ogive nose at the front of the airframe. Each is swapped, or skipped under
   the first of five reasons that applies: more than one nose cone, not split by M2.3c12, no
   column in OpenRocket's record, a nose whose drag the design states, or a flight HPR Sim
   fails with the column in. The run that writes the report stops on a flight whose line and
   rest don't sum to its drag part.
4. **#222's flight is not in the line, and [#1497][i-1497] carries it.** #222 was found on
   `C06/1`, a flight of the [design-library report][library]. The line is defined over the
   fixture report's flights, on OpenRocket's thrust and mass against time, which
   [M2.3c11][m2-3c11]'s oracle records for the collection's flights and two public designs,
   not for the library's. The library report
   holds no such histories, so `C06/1` cannot be swapped as decision 3 of ADR-349 defines the
   line. A swap of the nose alone on that flight, on HPR Sim's own thrust and mass, is a
   different measurement in a different report. It is worth having, since that flight is
   where the nose should matter most, and #1497 gives it its own *done when*. It is not
   built here: it changes the library report's probes and its oracle record, which this
   increment's *done when* does not cover.
5. **No repeated nose is probed, and ADR-349's sentence is amended** ([#1484][i-1484]).
   Decision 3 of ADR-349 said M2.3c13 would show on a probe how OpenRocket's column counts a
   nose that repeats, on pods, before it swaps one. It swaps none. The oracle gives no column
   to a configuration with more than one nose cone, and the harness skips a flight whose
   design has one, counted in the report under its own reason: 0 flights. So the count of
   swapped flights with a repeated nose is zero by construction, and the report shows that
   none was skipped for it either. The sentence in ADR-349 now says that.

**What M2.3c13 found** ([report][forward], the tangent-ogive nose's line).

| over the 5 flights swapped | mean | median | mean absolute |
|---|---:|---:|---:|
| the nose's pressure drag above Mach 1 | −1.28% | −1.33% | 1.28% |
| the rest of the drag | −0.68% | +0.90% | 2.27% |
| the drag part | −1.96% | +0.51% | 2.65% |

- **The flights.** 5 of the 68 flights both fly are in the line. All 5 are swapped, and no
  reason for skipping one counts a flight.
- **The sign.** The line is negative: on its own nose drag HPR Sim climbs lower than on
  OpenRocket's. The mean absolute value is the mean's
  size to the report's two decimals, so the line is negative on every flight swapped, to that
  rounding.
- **The size.** The flights' top Mach numbers, by HPR Sim's own flights, average 1.06,
  inside the band from Mach 1 to 1.3 that the probe doesn't cover. The line is 1.28% of
  OpenRocket's apogee on average. It sizes how the two codes' noses differ just past Mach 1,
  and says nothing of a flight that reaches Mach 1.3 and beyond.
- **The rest of the drag** on the same flights is larger flight by flight and of either
  sign: mean −0.68%, median +0.90%, mean absolute 2.27% against the nose's 1.28%. In the
  means the nose is the larger share, −1.28% of −1.96%.
- **On a public design.** A test gives the public `mach-probe` rocket a tangent-ogive nose
  and a column of almost nothing. It holds the nose's line there below −1% and the line and
  the rest to the flight's drag part. With OpenRocket's own column for the design's cone,
  the apogee moves by under a ten-thousandth of itself.

**Consequences.**
- [M2.3c6][m2-3c6]'s *done when* is met across its four increments, and it closes.
- The line is code against code. Which nose drag is nearer a real tangent ogive stays open in
  [#222][i-222] and [#616][i-616].
- For [M1.14][m1-14]: on the 5 flights of the line, the two codes' noses differ by 1.28% of
  OpenRocket's apogee on average, and the rest of the drag by a mean absolute 2.27%. Those
  flights reach Mach 1.06 on average, so they cannot size the line from Mach 1.3 up, where
  the probe's difference is. The library's `C06/1` may ([#1497][i-1497]).
- When OpenRocket releases its fix, the oracle can be run on the new jar to show the column
  moved. The harness holds its records to 24.12, so the report's line stays 24.12's until
  that changes. That run is in no *done when*.

**Where it is explained.** [Accuracy][accuracy] gives the line with the split, and the
[aerodynamics chapter][aero] points to it from the probe.
