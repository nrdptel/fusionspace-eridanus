# ADR-331: Withdraw the from-boost row to land it again (2026-11-09)

- **Status:** accepted; withdraws #1365 and #1368 from main so the same work lands again in one pull request reviewed at its last commit. Nothing in the prediction or its reports changes; the scoreboard loses the row's cell.
- **Summary:** #1365, the scoreboard's `accuracy.from-boost` row, merged with seven commits that came after its last review; #1368 then tightened the row's test. From here, a pull request merges only once each review it needs has read its last commit, so both are withdrawn and land again together with their reviews first. While withdrawn, the scoreboard has no `accuracy.from-boost` row.
- **Scoreboard:** `accuracy.from-boost.hpr` worse in release 0.2: the row is gone from the scoreboard while it is withdrawn, so the cell that read 3.22% over 12 flights is no longer measured.

**Context.** #1365 added the scoreboard row `accuracy.from-boost` (M7.3d7). Seven commits after its
last review changed the row's note (which counts it states and how it words them), its decision
record and the pages that quote them, and they merged with the rest.

**Decision.** Revert #1368 and #1365 together, then land the same work again in one pull request
whose reviews read its last commit before it merges. The scoreboard has no `accuracy.from-boost`
row until the work lands again, so this record names its cell. M7.3d7 reopens with its *done when*
unchanged.

**What doesn't change.** The prediction, its reports, the halves and their seal are untouched. The
revert removes the scoreboard row, its test and the pages' mentions of it, and reopens M7.3d,
M7.3d3 and M7.3d7; ADR-330 stays, marked withdrawn.

**Consequences.** None for the method or its numbers.
