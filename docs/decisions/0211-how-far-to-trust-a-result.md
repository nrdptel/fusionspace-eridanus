# ADR-211: How far to trust a result: one note, from the largest comparison with real flights (2026-10-08)

- **Status:** accepted; carries out [ADR-164, the product system][adr-164] and the design audit's rows on trust ([ADR-208][adr-208]) for M0.9c3 (#379)
- **Summary:** `hpr sim` and `hpr mc` say under the design's name that their figures are simulated, and end their result, on standard output, with a three-part note, *How far to trust it*: what kind of figure it is, what HPR Sim's apogee was checked against with numbers, and what to rely on instead. The numbers are the committed report's on 55 logged flights of fliers' own designs, the largest and least flattering comparison with real flights: the simulated apogee averaged 9.8% above the altimeter's, within 10% on 27, low on 14 by at most 20%. A test holds each to the report. The plot ends with the same note; `--json` carries it as `trust`, beside `kind: "simulated"`. The note replaces the `help:` line that pointed at the Accuracy page. Every trust note on the site takes one shape, a quote opening **How far to trust it.**, and the site check refuses any other.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-184]: 0184-logged-apogees-of-the-private-collection.md
[adr-208]: 0208-the-design-audit.md

**Context.** The product system asks every result someone might fly on to say what kind of number
it is, its spread, and what it was checked against, in one fixed note: `principles.md` §3,
`writing.md` (*How far to trust it*), `data.md` (*Readouts*, *Charts*). The design audit
([ADR-208][adr-208]) found `hpr sim` printing a lone apogee to 0.1 m, unmarked, with one `help:`
line pointing at the Accuracy page, and the site's notes in five shapes (#379).

**Decision.**

1. **Which comparison.** The note quotes `validation/reports/fixture-flights.json`'s first set:
   HPR Sim's apogee against 55 logged flights of fliers' own designs, flown as drawn
   ([ADR-184][adr-184]). It is the largest comparison with real flights the project has, and the
   least flattering: the seven public flights of RocketPy's examples miss by 6.04% on average with
   no bias, while the 55 average 9.8% high. Quoting the smaller, kinder set would flatter the
   model. The report's histogram gives the low side, the side that matters for a waiver: 14
   flights read low, the lowest in the bin from −20% to −15%, so "by at most 20%".
2. **The note's words** (`crates/hpr-cli/src/trust.rs`), in the three parts and order of
   `writing.md`: "Simulated from the design file, not measured." Then the numbers. Then "Plan a
   waiver or a field's ceiling with room above this apogee; the altimeter's reading is the one to
   log, and the RSO decides." No verdict; the room to leave is the flier's call. `hpr mc` adds
   that its spread comes from the inputs' scatter alone, not from the model's error, which is the
   misreading its table invites.
3. **Where it goes.** On standard output, as the result's last lines, as `cli.md` asks ("the last
   lines say what to trust"), so a flight saved with `>` keeps it; wrapped at 100 columns, its
   link on a line of its own. The `help: see the Accuracy page` line it replaces is gone. Under
   the design's name, `hpr sim` prints "a simulated flight, not a measurement", and `hpr mc` counts
   "simulated flights": the label that `data.md`'s readouts carry, said once for every figure
   below it rather than on each row.
4. **JSON.** `SimFlight` and `McRun` gain `kind` (`"simulated"`, an enum open to `"measured"`
   for `hpr analyze` later) and `trust`, the note's text. Both are new fields; nothing is renamed.
5. **The plot** ends with the same note under its event table. It draws no spread band: the
   report's spread is over 55 different rockets, a histogram with an open top bin, not a band for
   this flight, and drawing it as one would read as a confidence interval it isn't. The chart's
   other unmet rules (banking, a data table) stay with #382.
6. **The site.** Every trust note is a quote whose first words are exactly **How far to trust
   it.**, followed by the three parts. Bare paragraphs, variant labels and the "not validated"
   openers become that shape; headings that began "How far to trust" are renamed for what their
   section covers. A site check refuses a bold "How far to trust" label anywhere else, such a
   heading, or a quote opening with a bold "not validated" sentence.
7. **Held by tests.** `trust::tests::the_notes_numbers_are_the_committed_reports` reads the
   committed report and fails when a figure moves; `the_bins_are_the_reports` holds the
   histogram's bounds to the report's own statement of them; `each_note_has_its_three_parts_in_order`
   holds the order.

**Left out.** The library's example programs keep their one line, "Not yet validated: see the
Accuracy page", since they show the library's use rather than a result to fly on. The headline
apogee keeps its tenth of a meter: precision that matches knowledge is `data.md`'s *Numbers*,
with #386.

**Consequences.** A regenerated collection report that moves the mean, the count within 10%, the
low count or the lowest bin fails a test until the note is updated with it. The note's numbers
are for the apogee; the other figures carry no measured spread of their own yet.
