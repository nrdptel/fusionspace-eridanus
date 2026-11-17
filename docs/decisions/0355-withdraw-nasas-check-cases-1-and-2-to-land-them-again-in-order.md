# ADR-355: Withdraw NASA's check cases 1 and 2 to land them again in order (2026-11-17)

- **Status:** accepted; withdraws [M1.14b3][m1-14b3] (#1523) for one pull request that lands it again
- **Summary:** M1.14b3, the free body flown against NASA's check cases 1 and 2, merged in #1523 before the captures of the pages it changed were looked at, as [ADR-282][adr-282] requires before a merge. A look done right after the merge found nothing cut at an edge. As with [ADR-306][adr-306], #1523 is reverted and one pull request lands it again with every check run before its merge. M1.14b3's roadmap entry is open until then; [ADR-353][adr-353] and [ADR-354][adr-354] stay, withdrawn with it.

[m1-14b3]: ../decisions-and-roadmap.md#m1-14b3
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-306]: 0306-withdraw-the-motor-update-to-land-it-in-order.md
[adr-353]: 0353-nasas-check-cases-1-and-2-both-differences-past-the-codes-spread-traced.md
[adr-354]: 0354-the-search-index-is-held-to-mdbooks-own-bar.md
[i-1521]: https://github.com/nrdptel/fusionspace-eridanus/issues/1521
[i-1522]: https://github.com/nrdptel/fusionspace-eridanus/issues/1522
[i-1524]: https://github.com/nrdptel/fusionspace-eridanus/issues/1524
[i-1525]: https://github.com/nrdptel/fusionspace-eridanus/issues/1525

**Context.** [ADR-282][adr-282] makes a milestone that changes what a user sees done on its own
acceptance pass in its pull request: each changed page captured at its real size and looked at
for clipping, after the site's automated clipping check, before the merge. A change of words
alone is captured at 320 and 1280 px. #1523 changed the words of eight pages: rigid-body
flight, gravity, frames, time integration, the accuracy page, the start page, the validation
plan and the writing guide. Its physics, validation, code and docs reviews ran before the
merge, and their blocking findings were fixed and reviewed again. The look at the captures did
not run until after it. Done then, after the clipping check passed on main, it found nothing
cut at an edge at either width. It did find a figure that breaks at its exponent's hyphen on a
narrow page, in a line #1523 did not touch ([#1525][i-1525]).

**Decision.** Revert #1523, so the tree equals main before it (fdb60fc2) but for this record
and the two it withdraws, and land the same work again in one pull request whose claims pass,
full gate, reviews, clipping check and captures all run before its merge. M1.14b3 reopens
with its *done when* unchanged.

**What doesn't change.** No model, tolerance or reference changes. The report's numbers come
back as they were: the tool regenerates them from the same cached files, and its check holds
them.

**Consequences.** Between the two merges main has no free body, no `cargo xtask check-cases`
and no check-cases report, and the pages say nothing of the comparison. The three references'
pins and their notices rows are gone until then. [ADR-353][adr-353]'s links to the report and
to the page's section point at the commit that held them. The search index is held to its
earlier limit, since [ADR-354][adr-354]'s limit is withdrawn with the rest. The issues filed
from #1523 stay open: [#1521][i-1521] and [#1524][i-1524] speak of code that comes back with
the work, and [#1522][i-1522] of the limit that does.
