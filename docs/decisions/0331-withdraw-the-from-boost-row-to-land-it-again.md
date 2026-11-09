# ADR-331: Withdraw the from-boost row to land it again (2026-11-09)

- **Status:** accepted; withdraws #1365 and #1368 from main so the same work lands again in one pull request reviewed at its final head. Nothing in the method or its numbers changes.
- **Summary:** #1365, the scoreboard's `accuracy.from-boost` row, merged with seven late commits that its reviews had not read. Those commits were reviewed after the merge with no blocking finding, and #1368 fixed what the reviews found. The project's rule is that a pull request merges only once every review has read its final head, so both are withdrawn here and land again together with their reviews first. While withdrawn, the row is absent.
- **Scoreboard:** `accuracy.from-boost.hpr` worse in release 0.2: the cell goes from 3.22% to not measured while the row is withdrawn, and returns when the work lands again.

**Context.** #1365 added the scoreboard row `accuracy.from-boost` (M7.3d7). Seven commits after its
reviews, wording changes to the row's note and its decision record, merged before any review read
them. The project merges a pull request only once each review it needs has read the final head.
Reviews after the merge found nothing blocking and #1368 fixed their findings, but a review after a
merge is not the check the rule asks for.

**Decision.** Revert #1368 and #1365 together, then land the same work again in one pull request
whose reviews read its final head before it merges. The scoreboard holds the row's cell, so this
record names it: the cell reads *not measured* until the work lands again. The roadmap's M7.3d7
reopens with its *done when* unchanged.

**What doesn't change.** The prediction, its reports, the halves and their seal are untouched; the
revert removes only the scoreboard row, its tests, its decision record and the pages' mentions of it.

**Consequences.** The merge check now refuses a merge while any review the diff needs started
before the branch's last commit.
