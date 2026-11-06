# ADR-322: The 2026-11-05 progress review: accuracy before reader-facing work (2026-11-05)

- **Status:** accepted; reorders the [queue][queue] (moves [M7.3][m7-3]'s other increments, [M10.2][m10-2], [M0.13][m0-13], [M0.9g][m0-9], [M2.3c3][m2-3], [M2.6][m2-6] and M7.4 later, amending [ADR-163][adr-163]'s and [ADR-226][adr-226]'s order and [ADR-314][adr-314]'s order of M7.3's parts), and tightens [M7.3d][m7-3]'s *done when* to give its numbers for both of [M2.7][m2-7]'s halves, fixes the halves' encoding and seals the held-out fits until M7.5 publishes them.
- **Summary:** In the four weeks to 2026-11-05 the measured accuracy did not move: FusionSpace HPR's mean absolute apogee error on the private collection's 60 logged flights is 12.07% against OpenRocket 24.12's 11.26%, a gap of 0.8 to 0.9 points since 10-08, and no aerodynamics fix has closed since 10-15. There are no outside users yet. The work goes to what is hard to add later, correct numbers and accuracy against real flights, and holds what matters only once someone reads it: the 0.2 release, the sites' calendar banner, the figures' digit check. Next: M7.3d, its fits split by the held-out halves, then M2.7, M7.5, M2.3c6 and M1.14.

[queue]: ../ROADMAP.md#queue
[m7-3]: ../decisions-and-roadmap.md#m7-3
[m10-2]: ../decisions-and-roadmap.md#m10-2
[m0-13]: ../decisions-and-roadmap.md#m0-13
[m0-9]: ../decisions-and-roadmap.md#m0-9
[m2-3]: ../decisions-and-roadmap.md#m2-3
[m2-6]: ../decisions-and-roadmap.md#m2-6
[m2-7]: ../decisions-and-roadmap.md#m2-7
[adr-163]: 0163-the-second-pass-products-and-priorities.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-226]: 0226-the-2026-10-10-review-the-gap-to-openrocket-by-cause.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-314]: 0314-m7-3-in-five-residuals-first.md

**Context.** The maintainer asked whether the project is on course. A review measured the four
weeks since the repository restarted on 10-08:

- **Accuracy.** The [scoreboard](../../validation/reports/scoreboard.md)'s collection rows have not
  moved except by adding flights (53 to 60). Open `P-high` issues on the simulator's and the
  analyzer's numbers (labelled `A-aero`, `A-flight` or `A-motor`) number 47. About 18 are known
  model limitations, most from the aerodynamics work before 10-08, for example transonic cone and
  ogive drag at 2 to 3 times the measured values (#67) and supersonic pressure drag about twice
  OpenRocket's on one design (#222). The queue's first item to change a model, M1.14, stood twelfth.
- **The analyzer.** Its readings (M7.1, M7.2) are done on `main`. The least-squares fit (M7.3b)
  passes its interval test on synthetic flights, but no real flight has checked it. The Bayesian
  option, a second kind of interval from the same fit, was split three times in four days (ADRs
  317, 320, 321) and passes 8 of its 10 counts.
- **Users.** Release 0.1 (10-08) is the only release. No issue, pull request or comment comes from
  outside the project.

ADR-163 put the analyzer first because logged flights are the only independent references above
Mach 1, and fitting them needs M7.1 and M7.3. That reason stands. What it needs from M7.3 is a fit
run on real logs, which M7.3d gives. The Bayesian pass and the post-flight plot don't feed it.

**Decision.**

1. **The queue's head becomes:** M7.3d From the boost; M2.7 Held-out flights; M7.5 The bias every
   simulator shares; M2.3c6 The gap to OpenRocket, by cause; M1.14 Accuracy inside the envelope;
   M2.3c2 Logged traces; M2.3c9 Every flight OpenRocket flies; M2.8 Honest uncertainty. Then M7.3
   (its other increments, unchanged), M10.2, M0.13, M0.9g, M2.3c3, M2.6, M7.4, M10.3, and the rest
   of the queue in its old order.
2. **M7.3d's numbers are split by the held-out halves, and the held-out fits are sealed.** ADR-209
   §4 states the rule in words; this fixes its encoding, before any code computes it. A flight's id
   is its `flight_id` in the private collection's manifest, which is stable as the collection grows
   (a report's case numbers are not). Its digest is SHA-256 of that id's UTF-8 bytes followed
   directly by the seven bytes `ADR-209`, with no separator. The flight is held out when the
   digest's first eight bytes, read as a big-endian unsigned integer, are below 2^64 / 3 (at most
   6,148,914,691,236,517,205). Before any half's number is reported, a commit holds the SHA-256 of
   the held-out ids, sorted, one per line with a newline after each; the ids stay private (ADR-151).
   M7.3d's report gives the collection's prediction errors and intervals for both halves, and the
   calibration half's fit aggregates. The held-out half's fit aggregates, the drag and impulse
   scales that M7.5 would correct, stay in the private collection with only their file's hash
   committed. M7.5 chooses its correction on the calibration half alone, then publishes them, so
   held-out flights never feed a choice of model.
3. **M7.5 goes before M2.3c6.** Both simulators read high, HPR by +7.76% and OpenRocket by +6.72%
   on the same flights, so a cause they share is the larger prize. M7.5's own *done when* covers
   the case where no correction helps: an ADR records it and nothing ships. M2.3c6 then follows.
   It runs against OpenRocket 24.12, as its *done when* already says. ADR-226 queued it after
   M2.3c3 without a stated reason; ADR-212 asks for the newest release beside 24.12. M2.3c3 waits on
   a release after 24.12, and the gap study's swaps can be rerun on that release when it lands. M1.14
   now sits before M2.3c3 though its *done when* also names the newest release: while none after
   24.12 exists that clause compares with 24.12, and when one lands, M2.3c3 goes before M1.14's last
   open increment.
4. **Held, not dropped:** the Bayesian pass (M7.3c's open increments) and the post-flight plot
   (M7.3e), each with its *done when* whole; release 0.2 until there is someone to give it to or
   a result to show; the calendar banner (M0.13); the digit and wording check (M0.9g); the open
   benchmark (M2.6); fault diagnosis (M7.4).

**What doesn't change.** No *done when* is loosened: M7.3d's is made stricter, and every held
item keeps its text. Open `P-critical` issues still come before all of it. The `P-high` issues on
the simulator's numbers stay open, and those that skew a measured number come first in the issue
cycles. Releases still need their checklist (ADR-162).

**Consequences.** The next scoreboard changes should come from M7.5 or M1.14. If M7.3d shows that
neither drag nor impulse explains the shared bias, M7.5 ends by its ADR and M2.3c6 and M1.14
carry the accuracy work. Users who arrive before 0.2 is released get 0.1, whose known gaps are in
its [changelog](../../CHANGELOG.md) entry and the open issues.
