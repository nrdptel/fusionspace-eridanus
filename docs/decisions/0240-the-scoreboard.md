# ADR-240: The scoreboard: its cells, its ratchet, and the cells no milestone measured (2026-10-11)

- **Status:** accepted; carries out [ADR-209](0209-the-2026-10-08-best-on-the-market-measured.md) §3; closes M0.10; adds M2.9
- **Summary:** `cargo xtask scoreboard` writes `validation/reports/scoreboard.{json,md}`: HPR Sim against OpenRocket 24.12 and RocketPy 1.13.0 on accuracy, bias, honest uncertainty, speed, round trips, features and steps to a first answer, each number read by key from a committed report and kept as printed, each unmeasured cell naming the open milestone that measures it. A check in the `xtask` tests fails when one of HPR Sim's numbers prints worse than the committed scoreboard, or a measured cell is no longer measured, unless an accepted ADR names that cell and the next release in a `- **Scoreboard:**` line. A new milestone, M2.9, measures the other tools' cells that no milestone did, and every tool's speed.

**Context.** M0.10's *done when*: "`cargo xtask scoreboard` writes
`validation/reports/scoreboard.md`, HPR Sim against OpenRocket and RocketPy on each axis ADR-209 §3
names, each number read from a committed report and each unmeasured cell "not measured" with the
milestone that fills it; a test fails when a number is worse, as ADR-209 §3 defines per axis (a
measured cell turned "not measured" included), than the committed one unless an accepted ADR names
that metric and that release; a test that worsens one fails."

What the committed reports measure against the two tools on 2026-10-11:

- HPR Sim's apogees against seven public logged flights (`real-flights.json`); neither other tool
  flies them in any report.
- HPR Sim's and OpenRocket 24.12's apogees against 53 logged flights of the private collection, as
  aggregates (`fixture-flights.json`). RocketPy can't read their `.ork` designs until
  [M3.6](../decisions-and-roadmap.md#m3-6) exports them.
- OpenRocket 24.12 flying HPR Sim's `.ork` exports beside the originals
  (`openrocket-export-flights.json`).
- RocketPy agreeing with HPR Sim on the same drag (`latest.json`). That is a check of HPR Sim's
  flight mechanics, not of either tool against the world, so it is no scoreboard cell; the
  [census](../accuracy.md#the-census) holds it.

No report measures uncertainty, speed, features or steps for any tool.

**Decisions.**

1. **The rows.** One table per axis, a column per tool, the release read from the reports
   (OpenRocket's from `openrocket-flights.json`, checked to match the export report and the
   fixture report's page; RocketPy's from the harness's sources).
   - Accuracy, as mean absolute apogee error, and bias, as mean apogee error: on the public
     flights, the private collection's ("every flight compared"), the held-out flights
     ([M2.7](../decisions-and-roadmap.md#m2-7)) and, for accuracy, published flights past Mach 1 (M2.3c7).
   - Honest uncertainty: the share of logged apogees inside the 50% and the 90% range (M2.8).
   - Speed: one flight, pad to landing, on one reference machine.
   - Round trips, which measure HPR Sim reading and writing each tool's file, so HPR Sim's own
     column says "n/a": configurations whose apogee moves by more than 0.5% or that fly only one
     way; designs that don't make the trip (HPR Sim can't read them, or the tool opens the
     original but not the export, or refuses the two differently); the largest apogee change.
   - Features, counted from the simulator guide's comparison table (M0.8a); steps to a first
     answer (M0.11).
2. **Worse, per axis (ADR-209 §3).** Higher for an error, a lossy round trip or steps; lower for
   features; farther from zero for a bias; farther from its level for a range's coverage. Speed is
   never held. Values are compared as the page prints them: percentages to 0.01, the largest
   round-trip change to 0.000001%, counts whole. A regeneration that moves a value inside its last
   printed digit changes nothing a reader sees, and since the committed mark moves only when the
   printed value does, a value can't creep more than half a printed step past it unseen.
3. **What is held.** HPR Sim's numbers: its own column and every round-trip cell. The other tools'
   numbers are shown, not held, since their changing says nothing about HPR Sim. A measured cell in
   any column but speed's that is no longer measured, or whose row is gone, counts as worse.
4. **Accepting a worse number.** An ADR whose status line starts "accepted" accepts a cell for a
   release in a line of the form `- **Scoreboard:** <cell ids in backticks> … release <x.y>`, a
   cell id being its row's and its tool's (`accuracy.collection.hpr`). The release is the one the
   work in progress ships in: the workspace version's next minor (0.2 while the version is 0.1.0).
   The writer refuses to write a worse scoreboard without one, and the check fails on it.
5. **The committed scoreboard is the mark**, as the [census](../accuracy.md#the-census)'s accepted
   file is: the check also fails when the committed files are not what the committed reports
   give, so a regenerated report can't come in without the scoreboard that the gate allowed. A
   hand edit of the mark shows in its diff, as an edit of the census does.
6. **Every unmeasured cell names an open milestone, and a test holds it.** The check fails when a
   cell's milestone has no open entry in the roadmap, so M0.8a, M0.11, M2.3c7, M2.6, M2.7, M2.8,
   M2.9 and M3.6 can't close while their cells still say "not measured".
7. **M2.9, for the cells no milestone measured:** RocketPy's accuracy and bias, OpenRocket's and
   RocketPy's past Mach 1 and for the ranges, and every tool's speed, each read from a committed
   report or shown as a capability the tool lacks, checked in its current docs with the date. It is
   queued after M3.6, whose export lets RocketPy fly the collection's designs. Its cells fill the
   other tools' columns, so it moves none of HPR Sim's numbers, which ADR-209 §13 allows when a
   milestone says so; a comparison with nothing to compare with is no comparison.
8. **Where it runs.** `the_committed_scoreboard_holds` in the `xtask` tests, so CI runs it on
   every platform; the tests also make a report worse on each held rule and check the gate fails,
   and that the same number on the other side of zero, a move inside the printed digit and another
   tool's worse number pass.

**Consequences.**

- The scoreboard shows HPR Sim behind OpenRocket 24.12 on the private collection: 12.95% against
  12.02% mean absolute error, +8.73% against +7.79% bias, a gap of 0.93 points where ADR-209 §7
  read 0.88 before [ADR-238](0238-the-corpus-rechecked-against-the-catalog.md)'s recheck.
- A milestone that measures a cell now wires it into the scoreboard as part of closing.
- The scoreboard covers the simulator only, the one product released so far; the others have
  nothing to compare yet.
