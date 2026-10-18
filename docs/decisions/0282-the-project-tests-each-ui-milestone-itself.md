# ADR-282: The project tests each UI milestone itself (2026-10-18)

- **Status:** accepted; amends [ADR-144][adr-144]'s UI rule, [ADR-162][adr-162] §2 and §4, [ADR-163][adr-163] §2, [ADR-191][adr-191]'s site rule, [ADR-196][adr-196] §4 and [ADR-249][adr-249]'s *done when*
- **Summary:** No milestone waits on the maintainer's use any more. A milestone that adds or changes something a user sees counts as done once the project's own acceptance pass is in its PR: each user task done end to end on the built product, every screen captured at its real size and shape and checked for clipping, and a cold reader's run through the guide. Turning off the maintainer's sites, publishing and tags stay his.

[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-163]: 0163-the-second-pass-products-and-priorities.md
[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-196]: 0196-the-2026-10-07-product-guides.md
[adr-204]: 0204-the-no-clipping-checks.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-249]: 0249-fusionspace-co-in-its-own-repository.md
[m0-7c]: ../decisions-and-roadmap.md#m0-7c
[m0-13a]: ../decisions-and-roadmap.md#m0-13a
[m9-6]: ../decisions-and-roadmap.md#m9-6
[m9-7]: ../decisions-and-roadmap.md#m9-7
[m10-5]: ../decisions-and-roadmap.md#m10-5
[m10-6]: ../decisions-and-roadmap.md#m10-6

**Context.** [ADR-144][adr-144] made the maintainer's own use the last step of every UI
milestone, and later records copied it: seven *done when*s on the roadmap, the release checklist's
"Neer has read it" for each product guide ([ADR-196][adr-196] §4), and the replaced web tools
([ADR-191][adr-191]). Release 0.1 shipped without the maintainer trying the simulator, and on
2026-10-18 he decided that later releases shouldn't need him either: the project has to test what
it ships properly by itself.

A person's use catches three kinds of problem: a task that can't be done, something that looks
wrong, and something that reads wrong. Each has a check the project can run without him. A task can be done end
to end on the built product. A screen can be captured at its real size and checked against its
safe shape ([ADR-204][adr-204] already does this for pages and figures). And a reader who doesn't
know the code can follow the guide and say where it stops making sense. Outside users still come
in at release 0.5 ([ADR-209][adr-209] §10), which this record leaves as it is.

**Decision.**

1. **The acceptance pass.** A milestone that adds or changes a screen, page, figure, guide or
   command a user meets counts as done once its PR records an acceptance pass:
   - **Tasks.** The user tasks the milestone serves, written from its *done when* and its product
     guide, each done end to end on the built product the way a user would do it: the release
     archive's command, the built docs site in a browser, the app's desktop shell, the web build in
     a browser, the phone and watch builds in their platform simulators. Never through internal
     APIs or test helpers.
   - **Inputs.** For each task, at least one public real file where one exists (public flights,
     sample logs, published designs), and one invented hostile file.
   - **Screens.** Every screen the tasks reach, captured at its real size and shape: web pages from
     320 to 1,920 px in both themes, phone and watch screens on the platform's device shapes with
     their published safe areas. Each capture passes the automated clipping check (boxes against
     the safe shape) and is looked at, corners and edges on purpose.
   - **A cold read.** A reviewer that knows none of the code does the tasks from the guide alone.
     Each place it gets stuck is fixed in the PR or filed as an issue.
   - **The record.** The PR lists each task, the commands or clicks, what came out and where the
     captures are. A capture is committed only when a command CI reruns makes it
     ([ADR-196][adr-196]).
2. **The roadmap's wording.** Where a *done when* said "Neer has used it" or "Neer has used each
   screen", it now says the milestone's acceptance pass is in its PR (this record):
   [M0.7c][m0-7c], [M0.13a][m0-13a], [M9.6][m9-6], [M10.5][m10-5] and [M10.6][m10-6]. For
   [M9.7][m9-7], "at a launch" becomes a pass under launch conditions: no network, two phones in
   their simulators sharing the register by file and by QR code, and a full pad hold on the battery
   budget.
3. **Product guides.** From release 0.2, each release's guide passes the cold read in place of the
   maintainer's read ([ADR-196][adr-196] §4).
4. **What stays the maintainer's.** Turning off any of his existing sites, their redirects and
   domains ([ADR-191][adr-191]), publishing and tags ([ADR-162][adr-162]), and money and accounts.
   M10.5's study with at least five outside flyers stays. He may still try anything; what he finds
   becomes an issue, and no milestone waits for it. The planning review that [ADR-162][adr-162] §4
   placed after his use of 1.0 happens once 1.0 is out.

**Consequences.**

- No milestone is blocked on the maintainer, so the queue no longer stalls on a UI milestone's last
  clause, as M0.7a6, M0.8a4 and M9.8b did.
- Each UI milestone costs one acceptance pass more than its tests. The phone and watch passes need
  the platform simulators, which run only on macOS, so those passes run on the maintainer's Mac and not in CI.
- An automated pass can still miss what a person would feel: a slow screen, an awkward order of steps. The
  cold read covers part of it, and the outside flyers at release 0.5 are the first real users.
