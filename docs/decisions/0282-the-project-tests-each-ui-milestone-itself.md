# ADR-282: The project tests each UI milestone itself (2026-10-18)

- **Status:** accepted; amends [ADR-144][adr-144]'s UI rule, [ADR-162][adr-162] §2 and §4, [ADR-163][adr-163] §2, [ADR-191][adr-191]'s site rule, and [ADR-196][adr-196] §4
- **Summary:** No milestone waits on the maintainer's use any more. A milestone that adds or changes something a user sees counts as done once its PR records the project's own acceptance pass: user tasks written before the work and done end to end on the built product by a reviewer that didn't build it, every screen captured at its real size and shape and checked for clipping, and a cold reader's run through the guide. The app and the field kit also post a short recording for the maintainer to look at, which informs but never blocks. Turning off his sites, publishing, tags and finding M10.5's outside flyers stay his.

[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-163]: 0163-the-second-pass-products-and-priorities.md
[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-196]: 0196-the-2026-10-07-product-guides.md
[adr-204]: 0204-the-no-clipping-checks.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[m0-7c]: ../decisions-and-roadmap.md#m0-7c
[m0-13a]: ../decisions-and-roadmap.md#m0-13a
[m9-6]: ../decisions-and-roadmap.md#m9-6
[m9-7]: ../decisions-and-roadmap.md#m9-7
[m10-5]: ../decisions-and-roadmap.md#m10-5
[m10-6]: ../decisions-and-roadmap.md#m10-6

**Context.** [ADR-144][adr-144] made the maintainer's own use the last step of every UI
milestone, and later records copied it: six *done when*s on the roadmap, the release checklist's
"Neer has read it" for each product guide ([ADR-196][adr-196] §4), and the replaced web tools
([ADR-191][adr-191]). Release 0.1 shipped without the maintainer trying the simulator, and on
2026-10-18 he decided that later releases shouldn't need him either: the project has to test what
it ships properly by itself.

A person's use catches three kinds of problem: a task that can't be done, something that looks
wrong, and something that reads wrong. Each has a check the project can run without him. A task
can be done end to end on the built product. A screen can be captured at its real size and checked
against its safe shape ([ADR-204][adr-204] already does this for pages and figures). And a reader
who doesn't know the code can follow the guide and say where it stops making sense.

Two limits stay. A pass judged by whoever built the work checks only what that builder thought to
check, so the tasks have to be fixed before the work and run by someone else. And no check catches
taste: on 2026-10-12 the maintainer found the docs site's wide windows felt empty, with nothing
clipped or broken (he later kept the layout, but only a person could raise it). That matters
most where a person handles the product by hand, the app and the field kit, so those get a short
look by him that informs but never blocks.

**Decision.**

1. **The acceptance pass.** A milestone that adds or changes a screen, page, figure, guide or
   command a user meets counts as done once its PR records an acceptance pass. That includes the
   ones whose *done when* doesn't name it, such as M0.7b, M0.7d, M0.13b, M6.8 and M9.1 to M9.4.
   Every task completes end to end on the PR's final commit:
   - **Tasks, written first.** The user tasks the milestone serves, written from its *done when*
     and its product guide before the building starts: in the milestone's ADR or its first commit.
     Later commits may add tasks but not drop or weaken one. Each task is done end to end on the
     built product the way a user would do it: the release archive's command, the built docs site
     in a browser, the app's desktop shell, the web build in a browser, the phone and watch builds
     in their platform simulators. Never through internal APIs or test helpers.
   - **Inputs.** For each task that reads a file, at least one public real file where one exists
     (public flights, sample logs, published designs), and one invented hostile file.
   - **Screens.** Every screen the tasks reach, captured at its real size and shape: web pages at
     the site check's widths in every theme the surface has, phone and watch screens on the
     platform's device shapes with their published safe areas. Each capture passes the automated
     clipping check (boxes against the safe shape) and is looked at, corners and edges on purpose.
   - **Someone else runs it.** A reviewer that did not build the milestone runs the tasks and
     judges the captures. What blocks a task is fixed in the PR; only defects off every task's path
     may be filed as issues instead.
   - **A cold read.** A reviewer that knows none of the code does the tasks from the guide alone.
     A place it gets stuck on a task's path is fixed in the PR; others may be filed.
   - **The record.** The PR lists who ran the pass (not the builder) and on which commit; each
     task with its commands or clicks, what came out, and pass or fail; where the captures are;
     and each finding from the run, the look at the captures and the cold read, with its fix
     commit or issue number, or "no findings" written out. A capture is committed only when a
     command CI reruns makes it ([ADR-196][adr-196]).
2. **The roadmap's wording.** Where a *done when* said "Neer has used it" or "Neer has used each
   screen", it now says the milestone's acceptance pass is in its PR (this record):
   [M0.7c][m0-7c], [M0.13a][m0-13a], [M9.6][m9-6], [M10.5][m10-5] and [M10.6][m10-6]. For
   [M9.7][m9-7], "at a launch" becomes a pass under launch conditions: no network, two phones in
   emulators sharing the register by file and by QR code read through an emulated camera, and the
   battery budget run to the end of its longest stated pad hold.
3. **Product guides.** From release 0.2, each release's guide passes the cold read in place of the
   maintainer's read ([ADR-196][adr-196] §4).
4. **A look for the app and the field kit.** For [M9.7][m9-7], [M10.5][m10-5] and [M10.6][m10-6],
   the pass also records the tasks on screen, about ten minutes, and posts the recording and the
   captures on the PR for the maintainer to look at. It is not a test and not a gate: the
   milestone merges on its pass, and what he says becomes issues. Publishing a release is his
   already, so for 0.5 and 1.0 the request to publish links the recording, and he sees the app
   before anything goes out.
5. **What stays the maintainer's.** Turning off any of his existing sites, their redirects and
   domains ([ADR-191][adr-191]), publishing and tags ([ADR-162][adr-162]), and money and accounts.
   He may still try anything; what he finds becomes an issue, and no milestone waits for it. The
   planning review that [ADR-162][adr-162] §4 placed after his use of 1.0 happens once 1.0 is out.
6. **The outside flyers.** M10.5's study with at least five flyers outside the project
   ([ADR-209][adr-209] §10) stays. The project does no outreach on its own, so finding them is
   the maintainer's call, as all outreach is ([ideas](../research/ideas.md)). He is asked before
   M10.5 starts. If he says no, an ADR records what replaces the study and
   the gap that leaves; M10.5's *done when* doesn't change on its own.

**Consequences.**

- No milestone is blocked on the maintainer, so the queue no longer stalls on a UI milestone's last
  clause, as M0.7a6, M0.8a4 and M9.8b did.
- Each UI milestone costs one acceptance pass more than its tests, and a second reviewer's time.
  The iPhone and watch passes need Apple's simulators, which run only on macOS, so those passes
  run on the maintainer's Mac and not in CI.
- An automated pass can still miss what a person would feel: a slow screen, an awkward order of
  steps. The cold read covers part of it, the maintainer's look part of the rest, and the outside
  flyers at release 0.5 are the first real users.
