# ADR-285: The rest of the site in nine increments, each held to the voice limits as it lands (2026-10-20)

- **Status:** accepted; splits [M0.14c][m0-14c] into nine increments, M0.14c1 to M0.14c9, the last with M0.14c's own *done when*; amends [ADR-284][adr-284] (its consequence that only the stock-phrase list can fail a build) and the writing guide's "Measured, not gated"; raises the roadmap's budget ([ADR-147][adr-147] §6) from 51,000 to 52,500 bytes
- **Summary:** The 71 site pages M0.14c covers hold 413,377 words, too many for one rewrite. They go in nine increments by the site's sections. From the first, `cargo xtask voice` fails when a page M0.14 has rewritten goes over a voice limit or misses the guide's sentence targets, so a page that passes stays passing while the rest are done. The last increment holds every site page, a new one included.

[m0-14c]: ../decisions-and-roadmap.md#m0-14c
[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-284]: 0284-the-maintainers-own-voice.md

**Context.** M0.14c is done when "each site page outside ADR-284 §5's list passes `xtask voice`
in CI, within its limits and `writing.md`'s sentence targets, with an acceptance pass." The site
is the pages its table of contents lists, plus its 404 page. Leaving out the two landing pages
M0.14b rewrote and the records page, which §5 leaves as it is, that's 71 pages. On 2026-10-20,
`validation/reports/voice.md` gives them 413,377 words. 70 of them are over at least one limit; the
71st, the `.pf2` format's page, already passes. The pattern most often over is sentences over 25 words; many pages
average 20 to 25 words a sentence, with a quarter to two fifths of their sentences over 25. The
largest pages are the aerodynamics model (58,615 words), the flight-log formats (43,721) and the
`.ork` format (36,742).

M0.14b rewrote about 11,000 words in its session, with a claims pass over every changed sentence,
a cold read and an acceptance pass. At that care, 413,377 words is several sessions' work. Nothing
holds the limits yet: only a stock phrase fails a build, so a rewritten page can drift back while
the next ones are being done.

**Decision.**

1. **The hold.** `cargo xtask voice`, with no path, and the site build's voice check fail when a
   held page has a rate over its limit, averages more than 20 words a sentence, or has 10% or more
   of its sentences over 25 words. The comparison is exact, not on the report's rounded figures.
   Held are the pages M0.14b rewrote (`README.md`, `docs/start-here.md`, `docs/the-simulator.md`)
   and each increment's pages, added in its PR. M0.14c9 replaces the list with every site page,
   read from the table of contents less the records page §5 leaves as it is, so a new page is held
   from its first commit. The report marks
   the held pages. This is the "passes `xtask voice` in CI" of M0.14c's *done when*: a report
   can't pass or fail, and a check that holds a page as soon as it's done keeps the earlier work
   from slipping. For the held pages it amends ADR-284's consequence that only the stock-phrase
   list can fail a build, and the writing guide's "Measured, not gated" says so.
2. **The increments,** by the site's sections, each about 33,000 to 59,000 words:

   | Increment | Pages | Words | What's in it |
   |---|---:|---:|---|
   | M0.14c1 The first guides | 16 | 32,886 | the 404 page; Getting started, Fly your `.ork`, Pick a motor, Check stability for a certification flight, Recording a trajectory, Exporting a flight, The builder, Python, Models of your own; the tour and the comparison; the four pages of Rocketry explained |
   | M0.14c2 Studies, data and logs | 13 | 46,230 | Monte Carlo, Sensitivity, Optimization, Online data, Launch-day weather, Soundings, NOAA forecasts, Elevation, Motor stock, Reading a flight log, Reading your own logger, Your own rocket, How a flight is simulated |
   | M0.14c3 The command line and the evidence | 5 | 51,221 | The command line, Accuracy, Checking a claim, What backs each claim, the validation plan |
   | M0.14c4 Reference and the environment | 14 | 41,898 | the API reference, How a release is built, the glossary, Writing these pages, Plan; the environment's seven model pages; the two numerical tools |
   | M0.14c5 The rocket and its flight | 9 | 41,148 | shapes, the design tree, mass, motors; rigid-body flight, time integration, moving mass, released mass, fin flutter |
   | M0.14c6 Aerodynamics | 1 | 58,615 | the aerodynamics model |
   | M0.14c7 Recovery, staging and readings | 5 | 45,567 | recovery, staging, flight metrics, flight-log readings, synthetic flights |
   | M0.14c8 The file formats | 7 | 52,091 | `.hpr`, `.ork`, `.orc`, `.eng`, `.rse`, ERA5, `.pf2` |
   | M0.14c9 Flight logs, and every page held | 1 | 43,721 | the flight-log formats; the hold moves to every site page |

3. **How each increment works,** as ADR-284 §7 and M0.14b did it:
   - its acceptance tasks ([ADR-282][adr-282]) are written into this record in its first
     commit, before any rewriting (M0.14c1's are below);
   - a claims pass reads each changed sentence against the old one: every number, link target,
     caveat and scope stays, and a fact that leaves a page is on a page it links;
   - its PR shows before-and-after excerpts of every page it changed;
   - a reviewer who didn't write the pages does the tasks end to end on the final commit, with
     captures at 320, 768, 1,280 and 1,920 px in each theme looked at, corners and edges on
     purpose, and the site's layout checks passing at every width;
   - the cold read checks each page against each rule of the voice section, and every finding
     is fixed in the PR or filed.

   Reference pages stay plain and impersonal (ADR-284 §2). A model page whose wording changes is
   read by the physics review and the validation audit as well as the docs review, since a
   split sentence can move a claim's scope.
4. **Text a tool writes** into a page is rewritten at its source: the evidence register's wording
   in `validation/evidence.toml`, the census's in its generator. A command's printed output in an
   example is the program's own words, which [M0.14d][m0-14d] rewrites.
5. **The model pages' *In short* box.** The site holds each model page's box to four items, each
   opening with its label in bold (the writing guide's "Measured, not gated"). On a short page the
   four labels alone go over the bold-label limit: 4 in 488 words is 0.82 per 100. The box is the
   one place the guide asks a reader to scan, which the voice section's paragraphs-and-lists rule
   allows, so the bold-label count leaves out those four labels. It's built in M0.14c4, the first
   increment to hold a model page.
6. **Not in M0.14c:** the repository's pages that aren't on the site (the vision, the
   architecture, the performance notes, the changelog, the contributing guide and the GitHub
   templates). Their counts stay in the report, unheld. The architecture page changes only through
   a decision record, and the vision records the maintainer's asks.
7. **The roadmap's budget** rises from 51,000 to 52,500 bytes, for the nine entries.

**M0.14c1's tasks,** each done end to end on the PR's final commit by a reviewer who didn't write
the pages, the way a flyer would:

1. From Getting started alone, install `hpr` and fly its first example with the commands the page
   gives; the output is the page's.
2. From Fly your `.ork`, fly a design the page names and say what the output's parts mean, as the
   page explains them.
3. From Pick a motor, find motors for a design by the page's steps. From Check stability for a
   certification flight, get the margin it describes and say which way that number can err.
4. From Recording a trajectory and Exporting a flight, write each file the pages describe and open
   it as they say.
5. From The builder, Python and Models of your own, run one example from each as the words say
   (the docs' code already runs in CI; this checks that the words around it lead there).
6. From the tour, say what HPR Sim gives for a flight; from the comparison, say where it stands
   against the other simulators, with the numbers the page gives.
7. From each of Rocketry explained's four pages, say in a sentence what it explains and how far
   its numbers are trusted.
8. An address that doesn't exist on the docs site shows the 404 page, and it leads back to the
   landing page.

**Consequences.**

- M0.14c takes nine sessions or more instead of one, and M0.9f waits behind it, as the queue has
  it.
- A rewritten page that drifts over a limit fails the build. The fix is to rewrite the sentence,
  not to drop the page from the list; M0.14c9 removes the list.
- Each increment's pages have to pass the site's link, anchor, name and evidence checks after the
  rewrite, as on any other change.

[m0-14d]: ../decisions-and-roadmap.md#m0-14d
