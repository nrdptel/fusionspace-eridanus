# ADR-286: The 2026-10-20 review: the export bar tightened (2026-10-20)

- **Status:** accepted; tightens [M3.2][m3-2]'s bar from [ADR-110][adr-110] (the export's apogee in OpenRocket within 0.5% of the original's) to 0.002%; places four ideas; no done-when is loosened and no release moves
- **Summary:** HPR Sim's `.ork` writer is value-exact and the only differences OpenRocket shows between a design and its export are OpenRocket's own run-to-run spread, at most 0.000509% on the corpus, while a writer that rounded every number to six decimals still passed the 0.5% bar, so the bar is tightened to 0.002% and the planning review places four new ideas.

[m3-2]: ../decisions-and-roadmap.md#m3-2
[adr-110]: 0110-m3-2b-openrocket-flies-the-export-only-counts.md
[adr-109]: 0109-m3-2-split-and-a-ork-written-from-the-design.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-212]: 0212-measure-against-openrockets-newest-release.md
[ork]: ../format/ork.md#checked-in-openrocket
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-export-flights.md
[ideas]: ../research/ideas.md#the-2026-10-20-planning-review
[i-879]: https://github.com/nrdptel/fusionspace-eridanus/issues/879

**Context.** The planning review of 2026-10-20 looked for a target already met with room to
spare, since raising a bar is how a review keeps the project from settling ([ADR-209][adr-209]).
[M3.2][m3-2] has HPR Sim write a design back out as a `.ork`, and [ADR-110][adr-110] checks the
file in OpenRocket 24.12: OpenRocket flies the original and the export, with the same launch
conditions, calm air and seed 1, and each configuration's apogees must agree within 0.5%. The
committed [report][report] (`cargo xtask ork-export-flights`, run from cached records in the
gate's `refs` checks) has all 151 configurations flown both ways within it, 143 identical to the
bit, and the largest difference 0.000191%: 2,600 times inside the bar.

A bar that loose can't see the mistakes it exists for. The writer prints each number in the
shortest decimal that reads back to the same bits, because an earlier tool lost precision by
rounding (`numbers_read_back_to_the_same_bits` pins it, [ADR-109][adr-109]). A probe wrote the
17 public example designs again with every number rounded to six decimals, the old mistake, and
flew them: 45 of the 55 configurations moved, by up to 0.032%, and none by 0.5%. Today's bar
passes that writer.

Where the eight differences come from was traced before tightening:

- **Not the file.** In the six private designs that hold them, every number both files carry is
  the same to the bit, and the parts come in the same order. What differs is form only: current
  tag names for legacy ones, duplicate and zero-valued tags dropped, a few defaults written out,
  and an `automatic` ignition written as the event it resolves to. OpenRocket's own read-back gives
  the same launch mass, and centers of pressure and gravity within one bit.
- **OpenRocket itself.** Flown again with the same jar, scripts and seed, the originals differ
  from their own record in 8 of the 151 configurations, and two more sessions of the export files
  differ from each other in 7: always the same 10 configurations in those six designs, and never
  the 58 others. Forty-three flights of the most sensitive configuration, each in a fresh JVM,
  spread over 0.000509%, with the original's and the export's flights overlapping.
- **Why a last bit moves the apogee at all.** OpenRocket reports the highest of its 0.05 s steps,
  which can sit up to $g\,\Delta t^2/8$ below the true peak: 3.07 mm (0.12 in) at 0.05 s. A
  difference in the last bit that shifts the step grid near apogee moves the reported apogee by
  millimeters, here up to 0.0002% of it.

So no closed form bounds the difference: the sampling part has one, but the step grid's part
depends on OpenRocket's step control, and a last-bit change can in principle flip a discrete
event. The bar is therefore set from the measurement and said to be.

**Decision.**

1. **M3.2's bar becomes 0.002%.** Every export opens where its original does and is refused for
   the same reason where it does not (unchanged), and every configuration flown both ways reaches
   the original's apogee within 0.002% (was 0.5%). That is 3.9 times the largest same-file spread
   measured and 10 times today's largest difference, on the corpus's designs, not a bound for any
   design. The rounding probe above fails it on 20 of its 55 configurations. `TOLERANCE_PERCENT`
   in `xtask/src/ork_export_flights.rs` carries it, with the edge test at exactly 0.002%; the
   report, the scoreboard's `interop.changed` row and [the format page][ork] say 0.002%. M3.2's
   archived entry and ADR-110 keep their words as records of what was decided then.
2. **What a failure means.** A design added to the corpus whose export moves by more than 0.002%
   fails the gate's `refs` checks. The cause is traced: a writer fault is fixed, and an OpenRocket
   spread larger than 0.000509% is measured the same way (fresh sessions of the same file) and
   recorded in a decision record before the bar moves, never widened to pass.
3. **The format page is corrected.** It said a difference between the two flights "comes from
   the file alone" and that the eight differences' cause "has not been traced"; it now gives the
   cause above, and that the count identical to the bit is 142 to 144 of 151 by session.
4. **Ideas placed.** Four ideas join [the backlog][ideas], each with its user, workflow, the
   scoreboard number it moves and a *done when*: OpenRocket's whole-rocket aero tables read and
   flown (after [#879][i-879], which refuses such a file until then), a canopy's drag area fitted
   from the flyer's own log (`accuracy.canopy-cd`), the stability margin through the flight with
   the cause of each change, and a past day's ERA5 column fetched with no account
   (`steps.command-line`). None is promoted: the user-facing surfaces in progress are at their
   limit of three ([ADR-144][adr-144]), and the queue keeps its order.

**Consequences.** The interop check now notices a writer that loses precision, the fault class it
was built against, instead of passing anything short of half a percent. Its margin is a
measurement: OpenRocket's spread on the next release ([ADR-212][adr-212]) or on a new design may
be larger, and then the gate says so and a record is written before anything changes. No
done-when is loosened, no release moves, the queue keeps its order, and no user-facing surface is
added.
