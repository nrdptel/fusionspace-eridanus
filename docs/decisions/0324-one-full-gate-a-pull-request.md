# ADR-324: One full gate a pull request, and the site step on prose left to CI (2026-11-06)

- **Status:** accepted; amends [ADR-149][adr-149] §1 (the gate passes, then the pull request opens) and the merge checklist's gate rule, and narrows the gate's default run (`scripts/gate.sh`).
- **Summary:** The gate (`scripts/gate.sh`) is the maintainer's local run of most of CI's checks; a *full* gate runs every default step. A pull request passes one full gate before it opens. A later commit, such as a review's fix, passes the steps it touches and CI on its head, unless main is merged in. Main gets its own full gate, with the checks on the private reference data, on each new tip of main before the next piece of work. The gate's default run leaves the site step out when a branch changes only the words of pages: CI builds and checks the site at every width on every pull request, and a merge needs it to pass.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md

**Context.** The gate (`scripts/gate.sh`) runs CI's own checks on the maintainer's Mac, plus the
checks on the private reference data under `refs/` that CI can't run. Its history (one line a
run, kept on the maintainer's Mac and not in the repository, so these figures can't be rechecked
from it) was read for the 42 pull requests merged from 2026-10-30 to 11-05:

- **Full runs.** The gate ran 380 times, 173 of them full runs on a committed tree (every default
  step planned; some stopped at the first step or the page checks). A merged pull request took 3.8
  full runs on average (median 3, at most 15; 12 full runs that couldn't be matched to a pull
  request are left out), and 113 minutes of gate on average, partial runs included. After a full run
  had passed, 21 more came after a later commit: 17 after a fix to code or a doc comment (6.2 h), 4
  after prose or reports alone (0.7 h). Another 7 (4.1 h) followed a merge of main.
- **Two rules that disagreed.** The merge checklist asked for a full pass "before it opens or on its
  final head", and the second half was read strictly; the study ties the 21 above to that
  reading. The same checklist gates a fix by the steps it touches, with CI running the rest, and the
  merge check accepts a full pass on the head or on any commit of the branch before it.
- **What CI covers.** CI runs each gate step but the `refs/` checks and EGO's benchmark on a pull
  request's head: tests, examples, validate and the Python suite on Linux, Windows and macOS, the
  rest on Linux. A change to research notes alone skips validate, examples, the Python suite and
  the WebAssembly check. CI's validate holds the reports' numbers to the digits the platforms share,
  where the gate on the Mac that writes the reports holds them byte for byte. Branch protection
  requires each of these jobs but `evidence`, `site` among them.
- **The site step.** From 2026-10-25 to 11-05 the gate's site step ran 380 times for 52 hours and
  failed 58 times. The study classed all 58 as self-made (house style, the docs' own tests) or
  flaky, from the failure text where it survived (for 158 of the 363 failed steps of every kind); 8
  passed when run again unchanged. None was found to be a layout defect that CI's site check missed.
  CI's site jobs (`site (1/3)` to `site (3/3)`) build the site, check its links and check every
  page's layout across the grid of widths from 320 to 1,920 px.

**Decision.**

1. **One full gate a pull request.** A full `scripts/gate.sh` passes on the pull request's diff
   before it opens; merge main first when main has moved. A later commit passes the gate steps it
   touches: `scripts/gate.sh clippy test` for code (the `refs/` checks join `test` when a physics
   crate changed), with `validate` too when the commit can move a report's numbers, and
   `scripts/gate.sh docs pages` for prose. CI runs the rest on the head. A merge of main after the
   pull request opens moves the branch's fork point, and the merge check then wants a new full
   pass.
2. **Main's own gate.** On each new tip of main, before the next piece of work starts, a full
   gate runs with the `refs/` checks and the Python suite named; a failed step runs once more
   before the failure is recorded. A failure there is
   fixed first, test first, as any defect in merged work. It catches what rule 1 could let through:
   a `refs/` check a late commit didn't touch, a report that moved in its last digits, or two pull
   requests that pass apart and fail together. CI's nightly run adds the censuses.
3. **The site step on prose.** The gate's default run leaves the site step out when every change
   on the branch is a Markdown file under `docs/` or at the top level, except `docs/SUMMARY.md` (the
   sidebar), and no file is deleted or renamed. The theme, `book.toml`, xtask's site checks, the
   rustdoc and the reports keep it in. The page checks (step `pages`) still run. Such a run still
   counts as full and is marked `site-left` in the history; `cargo xtask ci-budget` reads the mark.
   The merge check accepts it only while the branch still changes only the words of pages;
   otherwise it wants a passing `scripts/gate.sh site` on a commit that holds every other change.
   `GATE_SITE=1`, or naming the step, runs it anyway.
4. **Captures of such a pull request**, the screenshots looked at for clipping when pages change
   ([ADR-282][adr-282]: the project tests each UI milestone itself), are taken from the site CI
   built and checked on the head (the `github-pages` artifact of its run, once the site jobs pass),
   or after a local `scripts/gate.sh site`, so they still follow a passing layout check.

**Alternatives.**

- *Keep the full gate on the final head.* It reruns every step after a commit that touched one or
  two. CI runs those steps on that head too, apart from `refs/`, which rule 1 keeps for physics, and
  the reports' last digits, which rule 2 holds on main.
- *Leave the site step out of every run that changes no layout file, code included.* Code changes
  the rustdoc and the reports the pages show, so it was not chosen.
- *Cache each step's result by a hash of what it reads.* With the full reruns gone it would save
  little. Every step's inputs would have to be listed right, and the cache kept in step with them.

**Consequences.** A late commit can merge with a local run of only its touched steps; CI and main's
own gate cover the rest, and a defect main's gate finds is fixed after the merge, not before. A
layout defect, or a link that breaks only in the built site, in a prose change shows up in CI rather
than on the Mac, so it costs a push. How this is measured: the next review compares full runs a
merged pull request (3.8) and gate minutes a merged pull request (113) with this window's figures.
