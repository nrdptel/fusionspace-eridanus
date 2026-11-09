# ADR-332: M2.7 in two, the halves and the fits' guard, then predictions first (2026-11-09)

- **Status:** accepted; splits [M2.7][m2-7] into M2.7a and M2.7b, each with its own *done when*, M2.7's *done when* whole across the two, and so plans every remaining increment of M2.7; carries out M2.7a under [ADR-209][adr-209] §4 and [ADR-322][adr-322] §2.
- **Summary:** The private collection's report gives its apogee errors over both halves, as aggregates: of the 60 flights compared, 37 are in the calibration half and 23 held out. A fit across the collection's flights takes its flights through one guard that reads each flight's half from its id again and refuses a held-out one, so `cargo xtask fixture-flights --check`, in the gate's `refs` step, fails when a held-out flight feeds one. New flights predicted before their logs are compared, with only their files' hashes committed, are M2.7b.

[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md
[adr-329]: 0329-m7-3d6-the-collections-errors-by-halves.md
[m2-7]: ../decisions-and-roadmap.md#m2-7

**Context.** M2.7's *done when* has four clauses. Surveyed on 2026-11-09:

- **The rule, fixed first.** Met by #1328 (M7.3d4, [ADR-326][adr-326]): `held_out` in
  `xtask/src/collection_boost.rs` is ADR-322 §2's encoding, and the SHA-256 of the held-out ids,
  `validation/reports/held-out.sha256`, reached main on 2026-11-07, before #1359 (M7.3d6,
  [ADR-329][adr-329]) computed the first half's number on 2026-11-08.
- **The fixture report's halves.** `validation/reports/fixture-flights.{json,md}` gives its errors
  over every flight compared, without the early deployments, and without the flights the catalog
  proposes below tier A. It has no half.
- **A refs check on fits across flights.** The one fit across the collection's flights today is the
  calibration half's fit aggregates in `validation/reports/collection-boost.json`: the drag, impulse
  and mass scales that M7.5 will choose its correction from. They are taken over the outcomes whose
  `held_out` flag is false. Nothing would stop a change to that filter, or a new fit, from taking a
  held-out flight, and no check fails if one does.
- **New flights predicted first.** The collection is pinned at one commit, and no flight has been
  added to its manifest since the rule. A prediction made before a flight's log is compared needs a
  command that predicts from the design, motor and day without reading the log, a place in the
  private collection for its file, a committed list of the files' hashes, and a refusal in
  `fixture-flights` to compare a new flight whose prediction's hash isn't on that list. Pull
  requests here are squash-merged, so, as ADR-326 found for the seal, only a pull request of its own
  puts a hash on main before the comparison that follows it.

**Decision.**

1. **Two increments, in this order.** M2.7 is met when both are.
   - **M2.7a The halves and the fits' guard.** *Done when:* M2.7's first three clauses: a flight is
     held out when SHA-256 of its id and the seed `ADR-209` falls in the lowest third, fixed before
     any held-out error is computed; the fixture report gives both halves as aggregates; a refs
     check fails when a held-out flight feeds a fit across flights.
   - **M2.7b New flights predicted first.** *Done when:* M2.7's last clause: new flights'
     predictions sit in the private collection before their logs are compared, only their files'
     hashes committed. A flight the manifest gains after M2.7a is compared by `fixture-flights` only
     when the private collection holds its prediction, written without reading its log, whose
     file's SHA-256 is in a committed list that reached main in an earlier pull request; otherwise
     it is refused by name. With no new flight yet, this is shown on an invented one.
2. **A fit across flights** is any value estimated from more than one of the collection's flights
   that feeds a model, a correction, a tolerance or a choice of model. One flight's own fit, such as
   its drag scale from its boost, is not. Each takes its flights through
   `collection_boost::Calibration`, which can be built only by `Calibration::of`. That reads each
   flight's half from its manifest id again, whatever the caller's own filter said, and refuses the
   whole set when any flight is held out, counting them and naming none ([ADR-151][adr-151]). The
   calibration half's fit aggregates take one. M7.5's correction must take one too. The held-out
   half's fit aggregates are not a fit across flights: they are sealed, their file's hash committed,
   and published only after M7.5 chooses on the calibration half (ADR-322 §2).
3. **The report's halves.** `fixture-flights` gives two more sets, the calibration half and the
   held-out half of the flights compared, with the same statistics as the other three, and counts
   the held-out flights compared. Its check holds each half's count to that count. Measured:
   - The halves: 37 flights in the calibration half and 23 held out (38%, against a third
     expected).
   - The calibration half: HPR Sim's mean absolute error 9.43% (signed mean +4.24%), against
     OpenRocket 24.12's 8.95% (+3.64%).
   - The held-out half: 16.32% (+13.42%), against 14.98% (+11.68%).

   The held-out half is held out, not blind: every flight's error was published before the halves
   were sealed.

**What doesn't change.** M2.7's *done when* is whole across the two increments, and no clause is
narrowed. M2.7b adds how its clause is shown; it doesn't replace it. ADR-322's encoding and the
sealed digest stand.

**Consequences.** M7.5 must choose its correction on the calibration half alone, through the
guard. That half reads lower than the whole collection (+4.24% against +7.76%). The halves differ
because of where the hash put each flight, not because of a choice: the split was sealed before
either half's errors were computed. So a correction chosen on the calibration half will be smaller
than one chosen on every flight, and the held-out half is where it will be judged. M2.7b comes
before M2.3d's predictions (ADR-328), which commit their public predictions whole rather than a
hash.
