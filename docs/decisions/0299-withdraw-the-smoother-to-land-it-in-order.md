# ADR-299: Withdraw the smoother to land it again in order (2026-10-27)

- **Status:** accepted; withdraws [M7.2d][m7-2d] (#983, #986) for one re-land PR
- **Summary:** M7.2d, the smoother, merged in #983 before the captures of the pages it changed were looked at, as [ADR-282][adr-282] requires before a merge. The look found the health card's lines cut at their box's edge, which #986 fixed after the merge. Both are reverted, the tree going back to the commit before #983, and one PR lands the two again with every check run before its merge. M7.2d's roadmap entry goes back to open until then.

[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[m7-2d]: ../decisions-and-roadmap.md#m7-2d

**Context.** [ADR-282][adr-282] makes a milestone that changes what a user sees done on its own
acceptance pass in its PR: each changed page captured at its real size and looked at for
clipping before the merge. #983, the smoother's PR, merged first, and its captures were looked
at after the merge. On the smoother page the health card's long lines ran past their box, cut
flush at its edge with no sign in the light scheme that the box scrolls. #986 put the card in a
wrapping block.

**Decision.** Revert #986 and #983 together, so the code equals the commit before #983, and
re-apply both in one PR whose claims pass, page checks, full gate, reviews and captures all run
before its merge. The re-land carries no change beyond the two PRs' content, except that it
records the smoother as ADR-300 (this record takes 299, so the numbers have no gap) and points
its citations, M7.2e's included, at ADR-300. M7.2e's *done when* keeps the health card that #983
added to it, as the re-land needs it.

**Consequences.** For the time between the two merges, main has no smoother, and M7.2e's
*done when* names a health card the code doesn't yet hold; the smoother's synthetic-flight
results stay in #983 and come back with the re-land. The same remedy withdrew M0.14a, the
maintainer's voice, twice (#826, #832); it landed for good in #834.
