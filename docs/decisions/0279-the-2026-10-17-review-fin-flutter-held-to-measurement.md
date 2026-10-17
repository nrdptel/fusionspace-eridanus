# ADR-279: The 2026-10-17 review: fin flutter held to measurement, and intervals by design (2026-10-17)

- **Status:** accepted; adds milestone [M2.11][m2-11]; tightens [ADR-209][adr-209] §7 and [M10.3][m10-3]; no release moves
- **Summary:** HPR Sim's fin flutter speed, a safety output, is checked only against Martin's own worked examples, and nothing shows which way it errs on a real fin (the evidence register's row G22). A published wind-tunnel test found five composite fins fluttering at 19.6–21.5 m/s (64–71 ft/s) where the NACA formula gives 15.3 m/s (50 ft/s). M2.11 scores the flutter speed against measured flutter onsets from at least two published tests and against an independent BSD-licensed calculator, states the largest error on the flattering side, and adds a flutter row to the scoreboard. It is queued after release 0.3, before the competition presets that print a flutter speed. Separately, release 0.3's paired bootstrap interval is also reported resampled by design, since resampling single flights understates the spread when a few designs supply most of them.

[m2-11]: ../decisions-and-roadmap.md#m2-11
[m10-3]: ../decisions-and-roadmap.md#m10-3
[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-6]: ../decisions-and-roadmap.md#m6-6
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-236]: 0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[g22]: ../evidence.md#g22
[i-675]: https://github.com/nrdptel/fusionspace-eridanus/issues/675
[flutter]: ../physics/flutter.md
[dinulovic]: https://doi.org/10.5937/fme2203576D
[bennett]: https://github.com/jkb-git/Fin-Flutter-Velocity-Calculator
[rp-1200]: https://github.com/RocketPy-Team/RocketPy/issues/1200
[sl-note]: ../research/student-launch-apogees.md

**Context.** The guardrails of [ADR-144][adr-144] rank a flattering error in a safety output as the
worst class: a flutter speed that reads too high tells a flyer a fin is safe when it is not. HPR
Sim gives fin flutter by D. J. Martin's criterion in NACA TN 4197 (1958), which it reproduces with
both of Martin's worked examples ([the flutter page][flutter]). That checks the arithmetic, not the
answer. Martin's line was drawn through his own missile and wind-tunnel wings; nothing here holds
the criterion against a fin that someone else measured, and the evidence register says so in row
[G22][g22], open as [#675][i-675].

A search on 2026-10-17 found the kind of measurement that row wants. Dinulović and others
("Flutter analysis of tapered composite fins", [FME Transactions 50, 2022][dinulovic], pp. 576–585, Tables 3 and 4) clamped five quasi-isotropic e-glass fins
(root chord 180 mm, tip 90 mm, span 262 mm, 0.65 mm thick: 7.1, 3.5, 10.3 and 0.026 in) in a wind tunnel and raised the speed
until each fluttered: 19.6–21.5 m/s (64–71 ft/s) over ten runs. The NACA formula gave 15.3 m/s
(50 ft/s) for the same fin,
22–29% low: the safe side, on that fin. One fin and one test are not a bound, and a fin clamped at
its root is stiffer there than one glued to a tube. The same search found an independent flutter
calculator with its examples, published under the BSD 2-Clause license ([Bennett's][bennett]), and
a defect in another tool: RocketPy's flutter speed reads √2 high ([RocketPy #1200][rp-1200]), the
flattering side. Flyers ask for flutter checks on the forums, and IREC's 2027 prediction table and
Student Launch's reports ask for a flutter speed, which [M6.3][m6-3] and [M6.6][m6-6] will print.

Release 0.3's target ([ADR-209][adr-209] §7) reports a paired bootstrap interval of the difference
between HPR Sim's and OpenRocket's mean absolute error on held-out flights. Resampling flights one
at a time treats them as independent. They are not when a few designs or owners fly most of them: on
the public Student Launch apogee predictions, the interval resampled by flight spans 6.1 points and
the one resampled by team and season 8.95 ([the note][sl-note]).

**Decision.**

1. **M2.11, fin flutter held to measurement,** joins the roadmap in Phase 1 and the queue after
   M10.3, release 0.3, so no release waits for it and the presets that print a flutter speed come
   after it. Its *done when*: HPR Sim's flutter speed is scored against at least three measured
   flutter onsets (wind tunnel or flight, none from the wings Martin's line was drawn through) from
   at least two independent published tests, each with its fin's geometry, material and shear
   modulus stated and committed with its table and page; `validation/reports/latest.md` gives each
   case's error with its sign; the flutter page states the largest error on the flattering side
   (the model above a measured onset), or that the cases show none, and a test pins each case, with
   a mutation probe that raises the speed failing one; the same fins and the calculator's own
   examples are compared with Bennett's calculator, each within 1% or the difference put down to a
   named modeling choice that a test pins; row G22's kind and what it leaves unchecked are
   updated from these; and the scoreboard gains a flutter row, HPR Sim's largest flattering-side
   error over the cases, with each other tool's cell read from its current docs with the date, or
   named as a capability it lacks. Fewer than three onsets only by an ADR that records each search,
   as [ADR-236][adr-236] allows for supersonic flights.
2. **The interval by design.** Where ADR-209 §7 and M10.3 report a paired bootstrap interval, it is
   reported twice: resampled by flight, and resampled by design, each design's held-out flights
   drawn together (and by owner, where the collection records one, as counts only). The pass rule
   doesn't change: the target is still HPR Sim's mean absolute error below OpenRocket's, and both
   intervals are reported beside it.

The flutter idea is recorded as promoted in `docs/research/ideas.md`. It adds no user-facing
surface: the flutter output exists; this checks it.

**Consequences.** A safety number that today rests on its source's arithmetic gets a measured
error and a stated side, and a fin case where the criterion reads high would be found before a
competition preset prints it. If the cases show the criterion reading high, the guide says by how
much, and the safety outputs built on it ([M6.3][m6-3], [M6.6][m6-6]) take that side into account.
The scoreboard shows flutter beside the other tools, where RocketPy's known defect is on the
flattering side. Release 0.3's comparison becomes harder to over-read: when a few designs dominate
the held-out set, the wider interval says so. No *done when* is loosened, no release waits for new
work, and the queue's other items keep their order.
