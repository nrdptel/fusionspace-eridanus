# ADR-377: No release waits on users, and release 0.2 comes next (2026-11-24)

- **Status:** accepted; lifts [ADR-322][adr-322] §4's holds on release 0.2, the post-flight comparison and the two reader-facing checks; replaces release 0.5's test by five flyers outside the project ([ADR-209][adr-209] §10, [ADR-282][adr-282] §6) with an emulated study; adds an emulated flyers' pass to [ADR-162][adr-162] §2's release checklist from 0.2 on; moves [M7.3][m7-3]'s Bayesian option from release 0.2 to 0.3 (amending [ADR-163][adr-163] §2); adds [M7.3f][m7-3f], the fit on the command line and in Python, and M7.3g, the least-squares intervals passing again ([#1108][i1108]); and reorders the [queue][queue], moving [M2.8][m2-8] ahead of release 0.2 and putting M0.9g before M0.13 where [ADR-296][adr-296] §2 had it after; and raises the roadmap's budget from 53,500 to 55,500 bytes.
- **Summary:** Through release 1.0, nothing waits on outside users, their feedback, the maintainer's own use, or anyone designing or flying a rocket. Where a release needs a person to try it, emulated flyers stand in: written personas from public sources, played by reviewers who didn't build the work, with planted defects measuring what they catch. They show whether a careful newcomer can finish the tasks, not whether flyers want the tool. Release 0.2, the flight analyzer, comes next, after the MAPLEAF comparison (M1.14b8): the post-flight plot, the fit on the command line and in Python with its intervals passing again, a spread on each flight's apogee, and the figures' digits and wording check (M0.9g). The Bayesian interval moves to 0.3 unchanged.

[queue]: ../ROADMAP.md#queue
[m0-9]: ../decisions-and-roadmap.md#m0-9
[m0-13]: ../decisions-and-roadmap.md#m0-13
[m1-14]: ../decisions-and-roadmap.md#m1-14
[m2-8]: ../decisions-and-roadmap.md#m2-8
[m7-3]: ../decisions-and-roadmap.md#m7-3
[m7-3f]: ../decisions-and-roadmap.md#m7-3f
[m10-2]: ../decisions-and-roadmap.md#m10-2
[m10-3]: ../decisions-and-roadmap.md#m10-3
[m10-5]: ../decisions-and-roadmap.md#m10-5
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-163]: 0163-the-second-pass-products-and-priorities.md
[adr-196]: 0196-the-2026-10-07-product-guides.md
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-275]: 0275-the-2026-10-14-awareness-and-care.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-296]: 0296-what-the-words-leave-digits-and-the-command-lines-words.md
[adr-321]: 0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
[i401]: https://github.com/nrdptel/fusionspace-eridanus/issues/401
[i966]: https://github.com/nrdptel/fusionspace-eridanus/issues/966
[i973]: https://github.com/nrdptel/fusionspace-eridanus/issues/973
[i1108]: https://github.com/nrdptel/fusionspace-eridanus/issues/1108
[i1444]: https://github.com/nrdptel/fusionspace-eridanus/issues/1444
[fit-report]: ../../validation/reports/fit-intervals.md
[n5]: https://www.nngroup.com/articles/why-you-only-need-to-test-with-5-users/
[faulkner]: https://doi.org/10.3758/BF03195514
[schmettow]: https://doi.org/10.1145/2133806.2133824

**Context.** On 2026-11-24 the maintainer set the rule this record writes down. This early,
development shouldn't depend on anyone using the product. The project should be able to reach
release 1.0 with nobody using it, by emulating its users. His feedback is there when it's asked
for, but nothing may wait on anyone else, or on him designing and flying a new rocket. Users will
come, and no release is held back until they do.

Two places in the plan waited on people:

- [ADR-322][adr-322] §4 held release 0.2 "until there is someone to give it to or a result to
  show". It also held the sites' calendar banner ([M0.13][m0-13]) and the figures' digit and
  wording check ([M0.9g][m0-9]), because they matter only once someone reads them.
- Release 0.5's *done when* asks for "at least five flyers outside the project"
  ([ADR-209][adr-209] §10). [ADR-282][adr-282] §6 kept that study, and said that if the
  maintainer declined to find them, an ADR would record what replaces it and the gap that leaves.
  This is that record.

A gate on users can't open on its own here. The project does no outreach (ADR-282 §6). Release
0.1.0, out since 2026-10-08, is still the only release, and no issue or pull request has come
from outside the project. Nothing else before 1.0 waits on a person: new flights come from
published records and the collection. The lines that need hardware built and tested, avionics
and motor design, come after 1.0.

Release 0.2 stood like this on 2026-11-24:

- Done: logs read with no design file (M7.1, M7.2), residuals (M7.3a), the least-squares fit
  (M7.3b) and the prediction from the boost (M7.3d).
- The least-squares fit's 90% interval on set B's drag scale has failed its test since the
  tip-off phase became the default: 80 of 1,000 truths fall below it, where a tail passes with 29
  to 74 ([ADR-368][adr-368], [#1108][i1108]).
- Of the Bayesian option's ten intervals (five parameters on two sets), 7 pass their test
  ([the report][fit-report]). It was
  split three times in four days, the last by [ADR-321][adr-321], and has been held since
  2026-11-05.
- No command draws a log beside its simulation: `hpr compare` prints residuals, and the
  post-flight comparison (M7.3e) isn't built.
- The fit has no command and no Python function. It lives in the Rust library alone, so a user of
  the command line or Python can't fit drag, which [ADR-163][adr-163] §1 lists as part of the
  analyzer.
- Of the release checklist: there is no analyzer guide, and no survey of the tools it competes
  with ([ADR-209][adr-209] §11); no smoke test reads a log; `hpr compare` doesn't carry the
  simulation's warnings.
- Two of the design system's "Before release" items fail, and every release must pass them for
  what it ships ([ADR-164][adr-164] §5). One is the writing rules ([#966][i966], [#973][i973]),
  which M0.9g closes. The other is the FusionSpace test: its *trust stated* answer waits on a
  spread beside one flight's apogee ([#401][i401]), which M2.8's *done when* includes, and its
  *drawn* answer on the README's top ([#1444][i1444]).

**Decision.**

1. **Nothing before 1.0 waits on people.** No milestone, release or check through 1.0 waits on
   outside users or their feedback, on the maintainer's own use, or on anyone designing, building,
   flying or ground-testing a rocket. Real flights come from published records and the
   collection, and a person's trial comes from emulated flyers (§2). What stays the maintainer's
   doesn't change: publishing and tags ([ADR-162][adr-162] §2), and turning off his sites, money and
   accounts ([ADR-282][adr-282] §5). His feedback is asked for when it would help, and nothing waits
   on it. The hardware lines after 1.0 are outside this rule; the planning review after 1.0 takes
   them up.

2. **Emulated flyers.** An emulated flyer is a written persona played by a reviewer that didn't
   build the work.
   - The persona comes from a cited public source: a forum question, a club's or a contest's
     rules, a public design or log. It states what the flyer knows, what they own and what they
     want done.
   - The flyer starts from the release's files and its guide alone, on a clean machine, with a
     budget of time and attempts for each task. It hasn't read the code.
   - The tasks, and what counts as success, are written before the work, from the release's
     *done when* and guide.
   - Each task records success, time, steps, a running account of what the flyer tried and why,
     and every problem. Each problem is fixed or filed before the release.
   - A reviewer who plays no flyer plants defects in a copy of the release: a wrong step in the
     guide, a misleading error, a mislabeled unit, a file missing from an archive, a confusing
     default. The flyers aren't told. How many they catch measures the method.
   - The flyers are played by more than one reviewer. One reviewer playing several personas meets
     much the same problems each time.

3. **Each release from 0.2 on has an emulated flyers' pass.** [ADR-162][adr-162] §2's checklist
   gains it: three flyers with three personas, and two or three planted defects. A pass that
   catches none of them is void and runs again.

4. **Release 0.5's study is emulated.** In place of "at least five flyers outside the project", at
   least twelve emulated flyers do its tasks: six personas, each played by two different reviewers,
   the tasks in a shuffled order. The study goes on past twelve while either of the last two flyers
   meets a problem no earlier one did. Eight to ten defects are planted, across the kinds in §2. The
   report gives, for each task, the successes out of the flyers, the median and range of the times,
   and the problems by how many flyers met them. It gives the planted defects caught with a 95%
   Clopper–Pearson interval, pooled by kind with the earlier releases' passes.

   Twelve, not five. Nielsen's curve gives five testers about 85% of a product's problems when each
   meets a problem with a chance of about 31% ([Nielsen, 2000][n5]); the curve assumes every tester
   meets every problem independently, with that one chance. Random sets of five out of 60 real users
   found anywhere from 55% to 99% ([Faulkner, 2003][faulkner]). Counting the problems no session had
   met yet, and letting the chance differ by problem, raised one study's sessions needed for 80%
   from 11 to 56 ([Schmettow, 2012][schmettow]). This record assumes emulated flyers are further
   from independent than people are, so a count of them proves less; the planted defects' catch is what measures the
   method. A handful of planted defects bounds the catch rate only loosely, so the passes are
   pooled.

5. **What an emulated pass claims, and the gap it leaves.** It claims that a careful newcomer could
   finish the tasks with the files and the guide, at the catch rate its planted defects measured.
   It doesn't claim that flyers want the tool, trust its numbers, fit it into their own workflow
   and hardware, use it on a launch field, or keep using it after weeks. This record assumes
   emulated flyers keep going where people give up, report fewer problems, and agree with each
   other more than people do, so it never counts them as people. Each release's notes and guide
   say so where they say how far to trust them. Five real flyers at 0.5 would have shown some of
   what an emulated study can't, and that loss is the gap. It stays open until people outside the
   project use a release; their reports then become issues, and nothing waits on them.

6. **Release 0.2's contents.** Release 0.2 is the flight analyzer: logs read with no design file
   (M7.1, M7.2), residuals (M7.3a), the least-squares fit (M7.3b) with its 90% intervals passing
   [ADR-257][adr-257] §1's test again (M7.3g, new, for [#1108][i1108]), the prediction from the
   boost (M7.3d), the post-flight comparison (M7.3e), the fit on the command line and in Python
   (M7.3f, new), and a spread beside each flight's apogee (M2.8). The Bayesian option (M7.3c) moves
   to 0.3 with its *done when* whole, and 0.2 prints no Bayesian interval.
   - The Bayesian option is a second kind of interval from the same fit. The analyzer's tasks
     (read a log, compare it with its simulation, fit the drag with an interval) work with the
     least-squares one. A release shouldn't wait on a second method its first tasks don't need.
     [Release 0.3][m10-3]'s *done when* now names all of M7.3, so it still needs the Bayesian one.
   - The fit gets a command because ADR-163 §2 gives the analyzer's users fitted drag, and they
     reach the analyzer through `hpr` and Python, not the Rust library. M7.3f's *done when*:
     `hpr` and the Python package fit a design to its log, and print each fitted parameter with
     its least-squares 90% interval and each reading's provenance. A fit of a flight that meets a
     condition on ADR-163 §4's list carries that issue's warning. An interval that fails ADR-257
     §1's test carries [ADR-368][adr-368]'s caveat. A test runs both on a public flight, and one
     pins each warning and the caveat.
   - #1108 gets its own increment, M7.3g, as the issue's sizing on 2026-11-24 asked: a fit-interval
     increment, not an issue fix. Its *done when* is the outcome 0.2 needs: every least-squares row
     on sets A and B of the report reads *passes*, enforced by `cargo xtask fit`. If the shares are
     split by the fit's residual, as #1108 proposes, each class passes too.

7. **The queue.** M1.14b8, the next increment, finishes first. Then come M7.3e, M7.3f, M7.3g,
   M2.8, M0.9g, [M10.2][m10-2] and M0.13, and then [M1.14][m1-14] resumes. Everything after keeps its
   order; M7.3, which now holds only its Bayesian option, keeps its place before release 0.3's
   other prerequisites. M10.2's *done when* names M7.3g, so [#1108][i1108] can't be passed over.
   - M2.8 moves up from after M2.3c9, since release 0.2 can't pass the FusionSpace test
     without #401. Release 0.3 needs M2.8 anyway, so this changes when it comes, not what it is.
   - M0.9g goes before the release because it closes the writing rules' item (#966, #973), and
     because it checks the words and figures 0.2 ships, M7.3e's, M7.3f's and M2.8's among them.
     That puts it ahead of M0.13, where [ADR-296][adr-296] §2 had it after; both stay ahead of
     M2.3c2, as the maintainer asked on 2026-10-18.
   - M0.13 was held only because no one reads the sites yet. [ADR-275][adr-275] queued it after
     release 0.2, and it goes right after M10.2.

8. **The roadmap's budget rises from 53,500 to 55,500 bytes.** M7.3f, M7.3g, the queue's new lines
   and the release clauses, their counts written out, take the roadmap to 54,885 bytes. The cap
   rises by 2,000 bytes, as [ADR-321][adr-321]'s rose by 1,000, rather than cut other entries'
   text to make room.

**What is loosened, and what isn't.** Two *done when*s are loosened, each by this record: M10.2 no
longer needs the Bayesian option (§6), and M10.5's study by people becomes an emulated one (§4), its
loss written down (§5). No other *done when* is loosened. M7.3's stays whole, and 0.3 now asks for
all of it; M7.3c's increments keep their text; M10.5's other clauses stay. The release checklist
only grows. Open `P-critical` issues still come first. The order [ADR-322][adr-322] set among the
accuracy items stands, since it rests on accuracy, not on readers; they now wait behind release
0.2's path. The dated reviews that cite the lack of users when queueing accuracy work keep their
words.

**Consequences.**

- Release 0.2's path is M1.14b8, then M7.3e, M7.3f, M7.3g, M2.8 and M0.9g. Then comes M10.2's
  own work: the analyzer's survey, and its guide with a *Compared with* section and its cold
  read ([ADR-196][adr-196], [ADR-282][adr-282]), a smoke test that
  reads a log, warnings on fits and residuals, the README's top (#1444), the flattering-side
  issues listed, the changelog and the install pages, the version number, the emulated flyers'
  pass, and the dispatch's smoke tests. Then the maintainer publishes it.
- The accuracy work (M1.14) waits for that path, then resumes.
- Release 0.3 now also needs M7.3c.
- If M7.3g's fix can't pass its test, its record says so with the measurement, and release 0.2
  waits on that decision, not on users.
- Each release costs one emulated pass more. The method's catch rate builds up across releases,
  pooled by kind.
- [The plan](../plan.md), [Start here](../start-here.md), [the vision](../VISION.md) and the
  roadmap's Phase 8 say what changed.
