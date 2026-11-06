# ADR-324: One full gate a pull request, and the layout check on prose left to CI (2026-11-06)

- **Status:** accepted; amends [ADR-149][adr-149] §1 (the gate passes, then the pull request opens) and the merge checklist's gate rule, and narrows the gate's default run (`scripts/gate.sh`).
- **Summary:** A pull request passes one full gate on its diff before it opens. A later commit, such as a review's fix, passes the gate steps it touches and CI on its head, not a second full gate. A merge of main still needs one. Main itself gets a full gate, its `refs/` checks included, before each new milestone starts. The gate's default run leaves the site's layout check out when a branch changes only the words of pages: CI checks the built site at every width on every pull request, and a merge waits for it.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md

**Context.** The gate (`scripts/gate.sh`) runs CI's own checks on the maintainer's Mac, plus the
checks on the private reference data under `refs/` that CI can't run. Its history
(`.git/gate-history.tsv`, one line a run) was read for the 42 pull requests merged from
2026-10-30 to 11-05:

- **Full runs.** The gate ran 380 times, 173 of them full (every default step). A merged pull
  request took 3.8 full runs on average (median 3, at most 15) and 113 minutes of gate in all.
  After a full run had passed, 21 more full runs came after a later commit: 17 after a fix to code
  or a doc comment (6.2 h), 4 after prose alone (0.7 h). Another 7 (4.1 h) followed a merge of main.
- **Two rules that disagreed.** The merge checklist asked for a full pass "before it opens or on its
  final head", and the second half was read strictly, so each late commit drew a full run. ADR-149's
  practice for a fix is the steps it touches, with CI running the rest. The merge ledger
  (`merge_ledger.py`) accepts a full pass on the head or on any commit of the branch before it.
- **What CI covers.** CI runs every gate step except the `refs/` checks and EGO's benchmark, on
  every pushed head, on Linux, Windows and macOS, and a merge waits for it to pass.
- **The site step.** From 2026-10-25 to 11-05 the gate's site step ran 380 times for 52 hours and
  failed 58 times: on house style, on the docs' own tests, or on a page timing that passed when run
  again unchanged (8 times). None was a layout defect that CI's site check missed. CI's site jobs
  build the site and check every page's layout at every width (`site (1/3)` to `site (3/3)`).
  Of the 96 pull requests merged from 10-25 to 11-02, 33 changed only prose.

**Decision.**

1. **One full gate a pull request.** A full `scripts/gate.sh` passes on the pull request's diff
   before it opens; merge main first when main has moved. A later commit passes the gate steps it
   touches: `scripts/gate.sh clippy test` for code (the `refs/` checks join `test` when a physics
   crate changed), `scripts/gate.sh docs pages` for prose. CI runs the rest on the head. A merge of
   main after the pull request opens moves the branch's fork point, and the merge ledger then wants
   a new full pass.
2. **Main's own gate.** Before a new milestone starts, a full gate runs on a fresh main, with the
   `refs/` checks and the Python suite named. A failure there is fixed first, test first, as any
   defect in merged work. It catches what rule 1 could let through: a `refs/` check a late commit
   didn't touch, or two pull requests that pass apart and fail together. CI's nightly run adds the
   censuses.
3. **The layout check on prose.** The gate's default run leaves the site step out when every change
   on the branch is a Markdown file under `docs/` or at the top level, except `docs/SUMMARY.md` (the
   sidebar), and no file is deleted or renamed. The theme, `book.toml`, xtask's site checks, the
   rustdoc and the reports keep it in. The page checks (step `pages`) still run. Such a run still
   counts as full and is marked `site-left` in the history; `cargo xtask ci-budget` reads the mark.
   The merge ledger accepts it only while the branch still changes only the words of pages;
   otherwise it wants a passing `scripts/gate.sh site` on a commit that holds every other change.
   `GATE_SITE=1`, or naming the step, runs it anyway.
4. **Captures of such a pull request** ([ADR-282][adr-282]) are taken from the site CI built and
   checked on the head (the `github-pages` artifact of its run, once the site jobs pass), or after a
   local `scripts/gate.sh site`, so they still follow a passing layout check.

**Alternatives.**

- *Keep the full gate on the final head.* It reruns every step after a commit that touched one or
  two, and CI runs the same steps on that head anyway, `refs/` aside, which rule 1 keeps for physics.
- *Leave the site step out of every run that changes no layout file, code included.* Code changes
  the rustdoc and the reports the pages show, so it was not chosen.
- *Cache each step's result by a hash of what it reads.* With the full reruns gone it would save
  little. Every step's inputs would have to be listed right, and the cache kept in step with them.

**Consequences.** A late commit can merge with a local run of only its touched steps; CI on three
platforms and main's own gate cover the rest. A layout defect in a prose change shows up in CI rather
than on the Mac, so it costs a push. How this is measured: the next review compares full runs a
merged pull request (3.8) and gate minutes a merged pull request (113) with this window's figures.
