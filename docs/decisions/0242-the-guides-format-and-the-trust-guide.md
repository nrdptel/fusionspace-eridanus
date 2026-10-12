# ADR-242: The guides' format and the trust guide, every number read from a report (2026-10-11)

- **Status:** accepted; carries out [ADR-195](0195-the-2026-10-07-guides-rocketry-explained.md) §6 and §7, and [ADR-204](0204-the-no-clipping-checks.md) §1's second increment; closes M0.7a2
- **Summary:** The guides' format is a section of the writing guide. The first guide, *Trusting a simulation* (how far to trust one), is a top-level page in a new *Rocketry explained* part of the site. `cargo xtask guides` holds every number in a guide's prose to a committed report: a quote fills its holes from the report's JSON, a page must hold it word for word, and any other number in the prose fails. The tests move every value each quote reads, one at a time, and require each hole to change and the page check to fail: a mutation probe per number, automated. The guide's figure, HPR Sim's apogee misses on 53 logged flights by bin, is drawn from the same report by the same command, the band within 10% shaded and the low side marked as the one that flatters a ceiling, tied to a new test of a miss's sign.

**Context.** M0.7a2's *done when*: "the format is in `docs/writing.md`; the guide meets it,
ADR-195 §7 and M0.7a's terms on numbers and marks, read cold in a docs review, and passes M0.7a1's
checks." M0.7a's terms: every number in a guide's prose sits in a block `xtask cli --check`
regenerates or is read from the committed report by a test, a mutation probe each; each
flattering-side mark is tied to its model's direction test.

The site already traces numbers on the accuracy page (ADR-017): each must appear, written the same
way, somewhere in a file the same paragraph links to. That is too loose for a guide: a count such
as 53 appears many times in a report, so a moved number can still find a match. No test held a
figure to a report, and no committed figure drew a spread.

**Decision.**

1. **Where the guides live.** A *Rocketry explained* part in `SUMMARY.md`, after *Accuracy*. Its
   pages sit at the top of `docs/`, like the other hobbyist pages, so the site's checks treat
   them as they treat those. The first is `trusting-a-simulation.md`.
2. **The format** is a subsection of the writing guide's *Four kinds of page* (the guide is at six
   sheets, its limit): ADR-195 §6's six points and §7's rules, which parts the build checks, and
   the three lists a new guide joins.
3. **Every number in a guide's prose is a quote's** (`xtask/src/guides.rs`). A quote is a template
   with holes, filled from the committed reports (`validation/reports/fixture-flights.json`,
   `real-flights.json`) through JSON pointers. The check reads the page as a reader reads it
   (paragraphs, items, table cells, headings and image descriptions; not code, not HTML
   comments), fails a quote the page doesn't hold word for word, and then fails every number left
   once the quotes are taken out, single digits included. A decision record's label (`ADR-211`)
   is a name, not a number. Numbers inside a block `cargo xtask cli` regenerates are code, and
   follow the program. So every number in the guide is either regenerated or read from a report
   by a test, and one that is neither can't be written.
4. **A mutation probe for every number, run as a test.** The reader records each value a quote
   reads. The test moves each one in turn (a count to none or none to one, so a histogram's bin
   can empty or fill; a number by a tenth; text to other text) and requires every hole of the
   quote to change under one of the moves, and the page check to fail on each text that changed.
   A quote whose hole moves with nothing it reads fails the test. A worked miss is also
   recomputed from the heights as the page writes them, so a reader's arithmetic agrees with the
   report's miss at the page's precision.
5. **The figure.** `docs/images/trust-apogee-misses.svg` draws HPR Sim's error histogram over the
   53 flights, from the report's bins (5% wide, open past ±30%), with the band within 10% either
   way shaded (ADR-195 §6's band; the report's count sits in the key) and zero as a chain line.
   Under the axis, the low side is marked in the caution color as the side that flatters a
   ceiling, with its count, and the high side as the side that costs height. It is drawn by
   `cargo xtask guides`, held to the figure and drawing checks, and a test fails a committed
   figure the report no longer draws. The band is the spread of other rockets' misses, labeled as
   such, not a confidence interval for one rocket, which ADR-211 §5 refused to draw.
6. **The mark is tied to a direction test.** `fixture_flights`' new
   `a_low_simulated_apogee_is_a_negative_miss` pins that a simulated apogee below the logged one
   is a negative miss, in a bin below zero's bound; the guide's
   `the_flattering_mark_is_the_low_side` pins that the caution bracket ends at zero's bound and
   counts exactly those bins. The guide names the first test.
7. **What the guide says.** Two kinds of error (inputs and models) and why Monte Carlo shows only
   the first; the miss defined and worked on a public flight; the 53 flights, OpenRocket on them,
   and the two simulators' agreement with each other (52 of 53 within 5%) read as a shared miss,
   not a check; the 7 public flights; which side flatters for which plan, worked in meters on a
   public flight that read low; the Monte Carlo spread as a regenerated worked example; what the
   comparisons leave out. It gives no margin to plan with: that is the flier's and the RSO's call.

**Alternatives considered.**

- **Trace the guide's numbers as the accuracy page's are.** Rejected: a match anywhere in a linked
  file lets a moved count pass.
- **Hand-written probes recorded in the ADR,** as earlier milestones did. Rejected: they prove the
  check once; the automated probe proves it for every quote on every run, and for each new one.
- **Numbers in prose quoted from the regenerated block's output.** Rejected for now: matching a
  prose number to a line of program output needs its own anchors; the worked example's numbers
  stay in the block, and the prose reads it by its columns' names.
- **A guide subfolder (`docs/explained/`).** Rejected: the site's checks sort pages by folder
  (model pages, formats), and a new folder would need its own rules for no gain.

**Consequences.**

- A regenerated report that moves a number fails `cargo test -p xtask` until the guide follows,
  and a new figure regenerates with `cargo xtask guides`.
- Each later guide (M0.7a3 to M0.7a5) adds its quotes and figures to `guides()`; a guide whose
  numbers come from a source other than these two reports adds that report to the reader first.
- The private corpus's report can't be regenerated in CI, so the guide's 53-flight numbers move
  only when that report is rerun and committed (M2.3c-series milestones).
