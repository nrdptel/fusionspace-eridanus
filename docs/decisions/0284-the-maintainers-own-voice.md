# ADR-284: The maintainer's own voice on every page (2026-10-18)

- **Status:** accepted; adds [M0.14][m0-14] in four increments at the head of the queue, with [M0.9f][m0-9f] second; extends [the writing guide](../writing.md) (its lead rule and "Measured, not gated", §2 and §4) and, through a PR to the design system, its writing rules ([ADR-164][adr-164]); amends [ADR-147][adr-147] §6 (the roadmap's budget)
- **Summary:** The project's pages read as if they came off a template, not from a person. M0.14 rewrites what people read (the README, the docs site, fusionspace.co, the design system's writing rules and README, `hpr`'s own messages and the repositories' About text) in the maintainer's plain voice, with a voice section in the writing guide and a `cargo xtask voice` check. Decision records, the archive and tools due to be replaced stay as they are.

[m0-14]: ../decisions-and-roadmap.md#m0-14
[m0-9f]: ../decisions-and-roadmap.md#m0-9f
[m7-2]: ../decisions-and-roadmap.md#m7-2
[m9-6]: ../decisions-and-roadmap.md#m9-6
[m10-2]: ../decisions-and-roadmap.md#m10-2
[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-217]: 0217-the-2026-10-09-fusionspace-co-rebuilt.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md

**Context.** On 2026-10-18 the maintainer read through the project's pages and found that they
sound generic: correct, but like nobody in particular wrote them. Reading the README, the docs
landing page and a few guides shows the patterns behind that:

- nearly every bullet opens with a bold label and a colon, so pages read like forms;
- clipped, label-first lines ("**Status: RELEASED.** Version 0.1.0, released October 8, 2026;
  work toward 0.2 goes on here.") that no person would say out loud;
- lists of three with every item the same shape and length;
- caveats stacked on caveats until a page sounds defensive;
- openings like "FusionSpace HPR is a ..." where a person would say why they built it.

The writing guide ([ADR-164][adr-164]) rules clarity: the bottom line first, sentence length,
numbers with units. It says nothing about voice, and the readability report it plans was never
built. Every new page is written to the same habits, so the cost of fixing this grows with each
milestone. The maintainer asked for it to be done carefully, across everything people read.

**Decision.**

1. **Whose voice.** The maintainer's, writing plainly to other flyers. The guide is built from his
   own writing, kept privately with his notes and never quoted; the public guide states the rules
   and shows before-and-after pairs taken from the project's own pages. Nothing is added for
   color: no phrasing he doesn't use himself, and no personal facts a page doesn't already state.
2. **Where "I" goes.** First person singular where a person is talking: the README, the landing
   pages, the pages that say why something exists, release notes and the guides. The bottom line
   still comes first ([the writing guide](../writing.md)'s rule 1 and its one-sentence leads): the
   why follows the lead, in the maintainer's words. Reference pages (a model's equations, a file
   format, a command's options) stay plain and impersonal. No corporate "we" for a project one
   person runs.
3. **The first rules,** refined in M0.14a from the research note on plain voice:
   - plain words over stock ones, from a list the check holds;
   - contractions, short sentences, and saying a thing once: no signposts ("In this section"),
     no recaps;
   - paragraphs for thoughts, lists only for real lists, bold labels only where a reader scans;
   - say what a thing is before its label (a milestone or decision number), and use the label
     only where the reader needs to follow it;
   - honesty stays exactly as strict, said once and plainly: what was checked, against what, and
     where it stops.
4. **The check.** `cargo xtask voice` reports, per page and per 100 words, the patterns a program
   can count: stock phrases, bold-label bullets, bare labels, sentences over 25 words and stacked
   hedges. It reads this repository's pages and any path given to it, so fusionspace.co's and the
   design system's pages are scored too. It fails only on its stock-phrase list, each entry with a
   test that fails without it and no hit on the model pages (`docs/physics/`); the rest it
   reports, against limits per 100 words that M0.14a writes into the guide. It is the readability
   report the guide planned, and the stock-phrase list is a second hard check beside the one
   "Measured, not gated" names.
5. **Scope.** Everything people read: this repository's README, the docs site, `hpr`'s help,
   warnings and errors, the Python package's docs, each crate's top-level docs, fusionspace.co, the
   design system's writing rules and README, the maintainer's GitHub profile README (no personal
   facts it doesn't already state), the About text of `fusionspace-eridanus`, `fusionspace-design`,
   `fusionspace-site`, `homebrew-tap` and `scoop-bucket`, and the PR and issue text the project
   writes from M0.14a on. On 2026-10-18 the maintainer authorized the project to edit those About
   texts and his profile README, and to merge its own PRs in the design system's repository on green
   checks, as [ADR-217][adr-217] did for fusionspace.co. Left as they are: decision records, the
   records pages and the done-work archive (history), research notes until they're next edited, the
   four old web tools that [M9.6][m9-6] replaces (Motor Finder, Window, Muster, Charge), the old
   landing page, and the retired Loft and Debrief repositories.
6. **Order.** M0.14 goes first in the queue, then M0.9f, which touches the same pages for number
   and date style while they are fresh. [M7.2][m7-2] resumes after them. The voice
   comes before [M10.2][m10-2] so release 0.2's guide is written in it from the start.
7. **Checked like any UI work.** Each rewriting increment's PR shows before-and-after excerpts of
   every page it changed, and its acceptance pass ([ADR-282][adr-282]) includes a cold reader who
   checks each page against each rule of the voice section and lists what it finds, each fixed in
   the PR or filed.
8. **The roadmap's budget rises** from 49,500 to 51,000 bytes, for M0.14's entry and its four
   increments (as ADR-275 and ADR-276 did for theirs).

**Consequences.**

- Release 0.2's remaining work moves back by about two days.
- Almost every page changes. The site's link, search and clipping checks have to pass on each
  increment, as on any other.
- Voice is partly taste, so only the stock-phrase list can fail a build; the counted patterns
  guide a reviewer, and the cold read covers the rest.
- New pages keep drifting back to the old habits unless the guide and the check are in place
  first, which is why M0.14a comes before the rewrites.
