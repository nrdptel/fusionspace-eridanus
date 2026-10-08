# ADR-149: M0.5e to g, reviews while CI runs, a faster gate and a split CI (2026-10-04)

- **Status:** accepted
- **Summary:** Reviews start once the PR is open and CI runs, with a shared bar for blocking findings and scoped re-reviews; the gate runs tests in parallel and adds the private-data checks when physics changes; CI's slowest job is split

**Context.** ADR-144 §1e and §1g ask that reviews run while CI runs, and that the gate run the
`refs/`-dependent checks when a physics crate changes. A measurement on 2026-10-04, over about 20
recent pull requests, gives the baseline these changes are judged against:

- Reviews ran after the gate and CI rather than alongside CI.
- The gate's test step was 82% of the gate. CI's critical path was the Windows test job: 24
  minutes in the last green run, 10 of them `examples --check` and 10 the test binaries run one
  after another. 27 of the last 50 CI runs outlasted `ci-wait.sh`'s 1,080 s bound.

**Decision.**

1. **Reviews after the PR, while CI runs** (§1e). The gate passes, the PR opens, then the reviews
   the diff needs start together: a physics review and a validation review on every physics,
   numerics or validation diff, a code review on non-trivial code, and a docs review only when
   user-facing pages change. Each review is given the *done when* and the ADRs not to re-argue.
2. **One bar for every review.** BLOCKING is a bug, a wrong or unsupported number, a check that
   can pass wrongly, an unmet *done when* or a hard-rule breach; the rest is ADVISORY. Accepted
   ADRs are frozen. A re-review checks only the fix's diff against the earlier blocking items.
   Porter et al. (1997) found that a second inspection did not significantly raise defect
   detection.
3. **Advisories don't loop, and loops end.** An advisory is fixed only when it is a quick wording
   change; otherwise it becomes an issue or is dropped. A fix that keeps failing review is
   redesigned or recorded.
4. **A faster gate, and the `refs/` checks in it** (§1g). The test step runs `cargo nextest run`
   (each test a process, in parallel), then `cargo test --doc`, because nextest skips doctests.
   Single runs on 2026-10-04: 133 s plus 13 s, against 296 to 309 s for `cargo test` in three
   earlier gates (the window's median test step was 230 s), with the same tests passing:
   1,806 under nextest plus 44 doctests, the 1,846 `cargo test` counted before four xtask tests
   were added; this branch adds one more, 1,807. On this branch the gate's test step took 150 to
   163 s and the full gate 435 s, against a window median of 280 s that had grown to 524 s by the
   window's end as the examples step grew (median 230 s for the test step). These are single
   runs; M0.5i measures the effect over later work. Without nextest the gate falls back to
   `cargo test`. A `refs` step runs `ork-flights --check`, `ork-flights --library --check` and
   `real-flights --check`. It joins the default set when `refs/` exists and the branch changes
   `hpr-aero`, `hpr-sim`, `hpr-motor`, `hpr-design`, `hpr-atmos` or `hpr-io` against
   `origin/main`, or that names `test` (a fix's gate), and says when it skips them. The three
   took 4, 5 and 2 s on a warm build (single runs on main, 2026-10-04). CI is unchanged on this
   point: those checks stay Mac-only.
5. **CI's critical path split.** `examples --check` leaves the test job for an `examples` job per
   OS, and the test job runs nextest, then the doctests. The deploy job waits for `examples` too.
   No check is dropped or loosened. `ci-wait.sh` now waits up to 1,680 s a call.

**How this is measured.** No saving is forecast as a number. At the next planning review, the first
20 pull requests after M0.5 are compared with the baseline: CI's critical-path median (from 19
to 25 minutes; the first run after the split took 13.2) and `ci-wait.sh` exit-3 repeats (from 9 in
about 20 pull requests).

**Consequences.** Reviews and CI overlap, so a clean PR's wall time drops by up to the shorter of
the two. A blocking finding still costs a second CI run. nextest runs each test in its own
process, so a test relying on state left by another in the same binary would now fail; all 1,807
pass. The first CI run with the new layout took 13.3 minutes, its slowest job
`test (windows-latest)` 13.2 (one run, on this PR). Until CI's new numbers are in, the one
13.2-minute run is a sample of one.
