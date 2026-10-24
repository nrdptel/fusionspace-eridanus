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

**M0.14c2's tasks,** done the same way, on the PR's final commit:

1. From Monte Carlo dispersion, run its 200-flight example as the page says, and say what the
   apogee's range means and which inputs the run scatters.
2. From Sensitivity analysis, run its example and say which input moves the apogee most. From
   Optimization, run one of its examples and say what the optimizer was asked for and what it
   found.
3. From Online data and the cache, say where the cache lives, how long a copy stays fresh, and
   what a run with no network and an empty cache does.
4. From Launch-day weather, Weather-balloon soundings and NOAA forecasts, run each page's example
   and say, for each source, what the simulator asks it for and what the page says it leaves out.
5. From A launch site's elevation, get a site's height by the page's steps, say what the height
   means, and read the page's file of one's own by its example.
6. From Motor stock and prices, run its example and say where the listing comes from, how fresh
   it is and what the simulator refuses.
7. From Reading a flight log and Reading your own logger, read a log with `hpr analyze` as each
   page says, and read the invented logger's table by its edited mapping; the output is the
   pages'.
8. From Your own rocket, run its example and say what the rocket is and what it printed. From How
   a flight is simulated, say in a few sentences what the simulator does from the pad to the
   ground and how far its numbers are trusted, with the page's numbers.

**M0.14c3's tasks,** done the same way, on the PR's final commit:

1. From The command line's *Running it* and *Flying a design*, fly the page's example design as
   it says, and say what each part of the output means, the accuracy note included.
2. From its `hpr mc` section, run its example and say what the spread means, which inputs it
   scatters, and how to write every flight to a file.
3. From its `hpr motors` and `hpr convert` sections, show a motor's figures and convert a design
   as the page says, and say what a conversion keeps and what it drops.
4. From its `hpr analyze`, `hpr weather` and *Scripting and shells* sections, read a log, ask for
   the weather with no network as the page says, get JSON output, and say what exit code a
   refusal gives.
5. From Accuracy, say how far the simulator's apogee is from real flights and from RocketPy's,
   with the page's numbers, and which of the four kinds of evidence backs each.
6. From Accuracy's *Model by model* and *Known gaps*, pick a model, say what it was compared with
   and how well it did, and name one gap with its issue.
7. From Checking a claim, follow one of its two examples end to end: the source, the test that
   pins the number, and the comparison, by the page's steps.
8. From What backs each claim, find a surface's rows, say what kind of evidence backs each, and
   how to send a sample. From the validation plan, say what proves a model right, which programs
   serve as oracles, and where the real flight data comes from.

**M0.14c4's tasks,** done the same way, on the PR's final commit:

1. From The API reference, build the reference as the page says and open it. Say which crate to
   start from to fly a design from your own program, which crate holds the atmosphere, and how
   far the page says to trust the reference.
2. From How a release is built, say what a release holds and how each file is checked. Run
   *Checking a file yourself*'s smoke test on release 0.1's download for this machine; it ends
   with `passed`.
3. From the glossary, look up angle of attack, the launch frame, ECEF, normal gravity, magnetic
   declination and Dryden turbulence. Say what each means, and follow each entry's link to the
   page it leads to.
4. From Writing these pages, run `cargo xtask voice` on one page as *The check* says and read its
   counts against *The limits*. Say which rule two of its before-and-after pairs show, and what
   the bold-label count leaves out on a model page.
5. From Plan, say what each release holds and in what order, how the page rates two of its goals,
   and what only the maintainer can do.
6. From Frames and sign conventions, say which way each axis of the launch frame and the body
   frame points, and what a positive angle of attack and sideslip mean. From Geodesy, say how a
   place's latitude, longitude and height become ECEF coordinates, and how closely the distance
   and bearing match their reference.
7. From Gravity and Earth rotation, say how gravity is checked against its source and which
   gravity the flight engine uses in a like-for-like case with RocketPy. From The magnetic field
   and declination, run its example and get the page's output, and say how far a compass can be
   trusted.
8. From Atmosphere, say what a barometric altimeter reads. From Wind, run its example and get the
   page's output, and say which height each wind model takes. From Turbulence, say what it leaves
   out for rockets. From Interpolation tables, say what a table does outside its range; from
   Adaptive quadrature, how closely its tests match their references. For each of the nine model
   pages, say from its *In short* how well it is validated.

**M0.14c5's tasks,** done the same way, on the PR's final commit:

1. From The design tree, configurations and checks, say where a part's position is measured
   from, what an automatic radius is taken from, and what an override replaces. Name one design
   the checks refuse and what the refusal says, and follow one of the page's worked examples (a
   cluster, the pods or the parallel stage) to its numbers.
2. From Nose cones, transitions and solids of revolution, name the shapes it draws and say how a
   wall's thickness is measured. Say what measuring it another way changes, with the page's
   figures, and how closely the shapes match their closed forms.
3. From Mass properties of components, say how a structure's mass and center of mass compare with
   OpenRocket's on the compared designs, with the page's counts, and the cause it gives for the
   design outside them. Say what a `.ork` leaves unsaid about a part's mass and how an override
   fills it.
4. From Solid motors, run its example program as the page says, and get the page's output. Say
   how the motor's mass and center of mass move as it burns, what its thrust does at altitude, and
   how closely the bundled curves' statistics match ThrustCurve.org's.
5. From Rigid-body flight, say what the state holds, what happens in each phase from the rail to
   the ground, and what ends a flight. From its *Whole flights against RocketPy*, say how closely
   the two codes agree on the example rockets, with the page's numbers, and where the path
   doesn't.
6. From Time integration and events, say which two methods step a flight and what each is for,
   how a step lands on burnout, how apogee is found, the defaults it runs with, and what it can't
   detect.
7. From Moving mass and Released mass, describe a shift and a release as each page says, follow
   each worked example to its numbers, and say how far each model is validated and what it leaves
   out.
8. From Fin flutter, follow one of Martin's worked examples and the rocket's fins to their
   numbers. Say which of the two readings decides, why the speed ratio alone isn't a safe line,
   which way each reading can err, and where a fin's shear modulus comes from.

**M0.14c6's tasks,** done the same way, on the PR's final commit:

1. From Aerodynamics' *In short*, say what the model covers and over what Mach range, which
   sources it rests on, how well its drag is validated faster than sound, and what it leaves out.
2. From *Code and conventions*, find the crate and function that compute one part's normal force,
   and the source the page gives for it. From *Your rocket's center of pressure*, get a rocket's
   CP and $C_{N\alpha}$ by the page's steps, and say where the CP is measured from.
3. From *Bodies of revolution*, say how a body's lift is found below and above Mach 1, and how a
   boattail or a flare is handled in a flight faster than sound. Say what a blunt tip's cap is
   worth, with the page's numbers.
4. From *Fins* and *Fins through Mach 1*, say how a fin set's normal force and CP are found, and
   what changes through Mach 1. From *Roll: forcing and damping*, say what makes a rocket roll and
   what slows it.
5. From *Pods* and *Tube fins*, say how each is modeled and what each leaves out. From *The normal
   force from RASAero II*, say what the export holds, how a flight uses it, and how it was checked.
6. From *Drag*, name the parts of a rocket's drag and say what a part's stated drag coefficient
   replaces. From *Drag through Mach 1* and *Boattails faster than sound*, say how the drag moves
   through Mach 1, and how far a boattail's drag is from its measurements, with the page's numbers.
7. From *Validity and open questions*, say where the model stops and name one open question with
   its issue.
8. From *Verification*, say how the normal force through Mach 1 compares with its target, and how
   the drag compares with the Arcas Robin wind tunnel, with RASAero II through Mach 2 and with
   MIL-HDBK-762's sample calculation, with the page's numbers. Say how the roll compares with the
   Arcas Robin and the Basic Finner.

**Consequences.**

- M0.14c takes nine sessions or more instead of one, and M0.9f waits behind it, as the queue has
  it.
- A rewritten page that drifts over a limit fails the build. The fix is to rewrite the sentence,
  not to drop the page from the list; M0.14c9 removes the list.
- Each increment's pages have to pass the site's link, anchor, name and evidence checks after the
  rewrite, as on any other change.

[m0-14d]: ../decisions-and-roadmap.md#m0-14d
