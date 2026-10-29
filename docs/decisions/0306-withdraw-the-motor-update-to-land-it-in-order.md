# ADR-306: Withdraw the motor update to land it again in order (2026-10-29)

- **Status:** accepted; withdraws [M5.7d][m5-7d] (#1040) for one re-land PR
- **Summary:** M5.7d, `hpr motors update`, merged in #1040 before the captures of the pages it changed were looked at as [ADR-282][adr-282] requires: the look before the merge took more captures than its limit, before the site's clipping check. A look done right after the merge found no clipping beyond #1043's. As with [ADR-299][adr-299], #1040 is reverted and one PR lands it again with every check run before its merge. M5.7d's roadmap entry goes back to open until then.

[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-299]: 0299-withdraw-the-smoother-to-land-it-in-order.md
[m5-7d]: ../decisions-and-roadmap.md#m5-7d

**Context.** [ADR-282][adr-282] makes a milestone that changes what a user sees done on its own
acceptance pass in its PR: each changed page captured at its real size and looked at for
clipping, after the site's automated clipping check, before the merge. A word-only change is
captured at 320 and 1280 px in every theme, at most 12 captures to a reviewer. #1040 changed
three pages' words (the command guide, the motor page and the start page). Its look before the
merge took more than 12 captures and ran before the clipping check of its head; after the merge,
the check passed on main and 12 captures were looked at, with no clipping beyond what #1043
records site-wide. The blocking findings of #1040's code and docs reviews were fixed before its
merge.

**Decision.** Revert #1040, so the tree equals main before it (193d047e) but for what merged
since, and re-apply it in one PR whose claims pass, full gate, reviews, clipping check
and captures all run before its merge. The re-land carries #1040's content, records the update's
design as ADR-307 (this record takes 306, so the numbers have no gap), points its citations at
ADR-307, and takes the fixes its own reviews find (ADR-307's status names them).

**Consequences.** Between the two merges main has no `hpr motors update`, and M5.7d is open
again. The issues #1042 and #1043, filed from #1040's review, stay open; their follow-ups wait
for the re-land.
