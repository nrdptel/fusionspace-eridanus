# ADR-304: Every passing file held to the oracles (2026-10-28)

- **Status:** accepted; carries out [ADR-288][adr-288]'s consequences and [ADR-298][adr-298] §1's third increment, [M5.7c][m5-7c]
- **Summary:** The 1% impulse test and the test against ThrustCurve.org's own statistics code run over all 412 files stored as passing, with the code's fixture holding exactly those. HPR Sim reads two samples at one time as a step, and ThrustCurve.org's code as one averaged point. HPR Sim keeps the step, and the code with its merge turned off matches it to 2.9e-15 on every file; the test names the 6 files the merge moves. A recount from the raw snapshot, with ThrustCurve.org's code, agrees with the index on all 1,700 files it holds. `cargo xtask motor-catalog --check` fails over 1%, worked out again from each file's bytes, and runs in CI.

[m5-7c]: ../decisions-and-roadmap.md#m5-7c
[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-298]: 0298-every-solid-motor-in-four-increments.md
[adr-301]: 0301-the-whole-catalog-in-two-increments.md
[tc-a]: https://github.com/JohnCoker/thrustcurve3/blob/577afa62302f70c6b2ba04e97a39240638cd704b/simulate/analyze/analyze.js
[i1002]: https://github.com/nrdptel/fusionspace-eridanus/issues/1002

**Context.** [M5.7c][m5-7c] is done when the L42 and statistics-code tests run over every file
stored as passing, the oracle fixture covering each, `xtask motor-catalog`'s 1% failure runs in
CI, and a raw-snapshot recount agrees. Until now the two tests and the fixture covered the 32 files
first bundled ([ADR-301][adr-301] §6). M5.7b1's snapshot of 2026-10-28 stores 412 of its 1,700
indexed files as passing, and 239 motors have one.

`validation/oracles/thrustcurve/analyze_stats.js` runs ThrustCurve.org's
[`simulate/analyze/analyze.js`][tc-a] (ISC) unchanged. Before it integrates, that code's
`normalize` sorts the points, drops leading points below 500 µN, and merges points less than
50 µs apart into one at their average thrust. HPR Sim joins the points with straight lines and
reads two samples at one time as a step: the thrust takes the later value from that time on
(`crates/hpr-motor/src/curve.rs`, `docs/format/eng.md`). Run over the 412 passing files:

| | Files |
|---|---|
| The merge folds no points; HPR Sim matches the code to 1e-12 | 404 |
| The merge folds points, but no figure moves past 1e-12 | 2 |
| The merge folds points and moves a figure | 6 |

The six moves, HPR Sim against the merged figures, in the headline figure each moves most: one
peak thrust +6.5% (a file with 1,061.8 N then 2,168.21 N at 0.01 s, averaged to 1,615 N), burn
times +0.61%, +0.50% and +0.19%, and average thrusts +0.28% and +0.023%. With `normalize`'s merge
turned off (`timeEpsilon` 0, the code's other standard parameters unchanged), the code matches
HPR Sim on all 412 files to 2.9e-15, relative.

**Decision.**

1. **HPR Sim keeps the step.** The file lists two samples; the step flies both, and the merge's
   average is a thrust no sample gives. The fixture records, for each passing file, the code's
   standard figures, the number of points the merge folded, and, where it folded any, the
   figures with the merge off (`unmerged`).
2. **The statistics-code test** holds HPR Sim's total impulse, burn time and window, average and
   peak thrust to the code's figures within 1e-12 on every passing file: the standard figures
   where the merge folds nothing, the unmerged ones where it does. It names the six files the
   merge moves, each with the figure it moves most and the gap to the digits written; a seventh,
   or a named file the merge no longer moves, fails. The fixture must hold exactly the files
   stored as passing.
3. **The L42 test** holds every passing file, worked out from its text, within 1% of its motor's
   ThrustCurve.org record in total impulse, average thrust and burn time, and within 1% of the
   code's figures (unmerged where it merges) in those and its peak thrust.
4. **The recount.** `analyze_stats.js --recount` reads the raw snapshot's answers under `refs/`
   (the search and download answers, not the committed records, file list or curve files), works
   out each file's figures with the code, and checks them against its raw record: a file passes
   when total impulse, average thrust and burn time are each within 1% (ADR-298 §4). Its verdict
   uses the merge off, the reading the index's check uses. It fails unless every file the index
   holds has the index's verdict, and prints each file where the standard merge gives the other
   verdict, and each left-out file's verdict. The gate's `refs` step runs it, since the raw
   snapshot isn't committed.
5. **The 1% failure in CI.** `cargo xtask motor-catalog` works out each indexed file's three
   figures again from its bytes and fails unless the file is stored as passing exactly when each
   is within 1% of its committed record, and the motor's stated figures are the record's. CI's
   `validate` job runs `motor-catalog --check` on all five runners.

**Consequences.**

- The recount, run 2026-10-28: 1,004 solid motors and 1,706 files; the code reads all 1,706; of
  the 1,700 the index holds, 412 pass and 239 motors have one, the index's counts. One file,
  Cesaroni's 2765L730-P file, passes with the merge off and would fail with it: its burn time is
  0.68% short of its record as HPR Sim reads it and 1.28% short after the merge.
- Of the 6 files the index leaves out, the code would pass one: a RockSim file whose times run
  backwards, which the code sorts and HPR Sim refuses. It stays out (ADR-288 §1's rule); its
  motor has a RASP file that reads.
- `motor-catalog --check` in CI also flies every motor by name offline (ADR-302), about a minute
  on the maintainer's machine.
- The comparison with OpenRocket and the `.eng`/`.rse` round trip still cover the 32 files first
  bundled; the round trip over every file is [issue #1002][i1002].
