# ADR-149: M0.5e to g, reviews while CI runs, a faster gate and a split CI (2026-10-04)

- **Status:** accepted; amended 2026-11-03 (below: a time budget for CI and the gate, the flight censuses each night)
- **Summary:** Reviews start once the PR is open and CI runs, with a shared bar for blocking findings and scoped re-reviews; the gate runs tests in parallel and adds the private-data checks when physics changes; CI's slowest job is split. Amended 2026-11-03: a pull request's CI within 20 minutes and a passing full gate within 30; the flight censuses (the synthetic flights, two logs and the fit's intervals) run whole each night on main, a failure filing a P-critical issue, and a change to research notes alone skips the jobs that read none.

[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-222]: 0222-six-platforms-tested-on-every-pull-request.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-312]: 0312-two-logs-on-one-timeline.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[m0-16]: ../decisions-and-roadmap.md#m0-16
[ci]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/.github/workflows/ci.yml
[pr1146]: https://github.com/nrdptel/fusionspace-eridanus/pull/1146

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

**Amended 2026-11-03: a time budget for CI and the gate, the flight censuses each night.**

**The amendment's context.** Three checks fly thousands of synthetic flights again and hold them to their
committed reports: the synthetic flights' census ([ADR-280][adr-280]), the clock offset between two
logs ([ADR-312][adr-312]) and the fit's 90% intervals ([ADR-316][adr-316]). Each arrived with a
milestone, and each ran whole on every pull request: the first two on all five platforms, the fit
in eight Linux jobs. Measured from GitHub's run and job records:

| Passing runs | 2026-10-20 | 2026-10-27 | 2026-11-01 | 2026-11-02 | 2026-11-03, to 20:50 UTC |
|---|---:|---:|---:|---:|---:|
| Pull request runs, median minutes | 14 (27 runs) | 15 (15) | 32 (15) | 37 (16) | 35 (27) |
| Runs on main, median minutes | 15 (19) | 14 (9) | 33 (6) | 83 (11) | 46 (20) |

- **Where the time went.** Runner time per run rose from 238 to 270 minutes (six runs on
  2026-10-27) to 492 to 537 (six on 2026-11-03), and a pull request's jobs from 42 to 51. In one run of each day,
  `examples` went from 37 to 107 runner minutes, `validate` from 81 to 113, and the fit's nine
  jobs, new, took 115. On Windows the synthetic flights took 18.9 and 17.3 minutes in two halves
  and two logs 7.0; the fit's parts took 8.6 to 16.5 minutes each.
- **Waiting for runners.** On 2026-11-03 a job waited up to 22 minutes for a runner in the six
  runs above, and up to 38 in the day's runs to 19:00 UTC.
- **What gained nothing.** 14 of the 27 passing pull request runs on 2026-11-03 were the research
  notes', which change only `docs/research/` and which none of the three checks reads, and ran
  them whole all the same.
- **What was next.** M7.3c3 ([#1146][pr1146]) adds the Bayesian option's draws to the
  fit's report: 3 h 1 min on a 10-core Mac, against 16 minutes before, 32 CI jobs instead of 8,
  and about 2 h 20 min for a full gate.

Issues #494, #882 and #1018 each measured one check's growth, and each split or trimmed that check.
The rule that every census runs whole on every pull request stayed, so the next one added time
again.

**The amendment.**

1. **The budget.** A pull request's CI finishes within 20 minutes, counted as its longest job's
   run, a wait for a runner apart. A passing full `scripts/gate.sh` finishes within 30 minutes on
   the Mac. A change that would break either brings, in the same pull request, the split that
   keeps it within.
2. **The censuses each night.** The synthetic flights, two logs and the fit's parts with their
   check run whole on main each night at 08:17 UTC, on the platforms they ran on before, and on a
   run started by hand with `full`. A pull request and a push to main leave them out. CI's `scope`
   job decides ([`ci.yml`][ci]); a test in xtask fails if a set of synthetic flights isn't on one
   line of a job on all five platforms, or if that job runs on any condition but the nightly one.
3. **A nightly failure comes first.** A nightly run that fails, or in which a census didn't run,
   files one P-critical issue, or comments on the one still open, and the work that broke the
   census is fixed before other work ([ADR-144][adr-144] §2). A run started by hand with `full`
   files none: whoever started it reads its result.
4. **Before a merge.** `scripts/gate.sh` runs the three whole on the Mac, as before, on the
   platform that writes the reports. What a pull request loses is the run of the censuses on the
   other four platforms, and the optimized build's run on every platform, the Mac included: a
   change that moves a census only there reaches main and is found that night.
5. **Research notes.** A pull request or push that changes only files under `docs/research/`
   (renamed from or to, too) runs no step of `validate`, `examples` and `python`, and skips `types`,
   `wasm-check` and `phone`; none of them reads a note. Each still reports its check by name, as
   branch protection needs. `test`, `site` and the quick Linux checks, which read the notes, run.
6. **The rest, M0.16, before M7.3c3 merges.** Each census gets a fixed slice that a pull request
   and the gate check exactly against the committed report, the whole staying nightly, and a check
   that measures the budget. Then M7.3c3's fit runs the same way, and its gate fits the budget.

**Alternatives to it.**

- **Split the censuses into more jobs, still on every pull request.** It shortens the slowest job
  but adds a build to each new job, and the fit's 32 jobs in #1146 already waited up to 32 minutes
  for a runner on 2026-11-03. More jobs mean more waiting.
- **Run them on fewer platforms on pull requests.** Windows is the slowest platform, and the one
  most worth keeping, so this saves the least where the time is.
- **Run a census only when a change touches its code.** The analyzer's milestones change that code
  in most of their pull requests (M7.3 is the fit), so their runs would keep it.

**Its consequences.**

- A pull request's CI loses about 270 runner minutes of the 500 a run took on 2026-11-03: the
  synthetic flights' and two logs' steps on five platforms and the fit's nine jobs. Its slowest
  jobs become the site's parts and the Windows tests: 14 to 16 minutes in one run on 2026-11-03,
  the census steps aside.
- The gate is unchanged until M0.16, so #1146 waits for it.
- [ADR-222][adr-222]'s six platforms still build, test and validate on every pull request. Only the
  censuses move to the night.
