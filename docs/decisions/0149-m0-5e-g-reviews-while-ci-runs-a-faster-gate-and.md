# ADR-149: M0.5e to g, reviews while CI runs, a faster gate and a split CI (2026-10-04)

- **Status:** accepted; amended 2026-11-03 (below: a time budget for CI and the gate, the flight censuses each night) and 2026-11-04 (M0.16a in three; the slices of the synthetic flights and two logs)
- **Summary:** Reviews start once the PR is open and CI runs, with a shared bar for blocking findings and scoped re-reviews; the gate runs tests in parallel and adds the private-data checks when physics changes; CI's slowest job is split. Amended 2026-11-03: a pull request's CI within 20 minutes and a passing full gate within 30; the flight censuses (the synthetic flights, two logs and the fit's intervals) run whole each night on main, a failure filing a P-critical issue, and a change to research notes alone skips the jobs that read none. Amended 2026-11-04: pull requests and the gate fly every 21st flight of the synthetic flights and two logs, held line for line to a committed slice.

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
   line of a job on all five platforms, or if that job runs on any condition but a full run's.
3. **A nightly failure comes first.** A nightly run that fails, or in which a census didn't run,
   files one P-critical issue, or comments on the one still open, and the work that broke the
   census is fixed before other work ([ADR-144][adr-144] §4). A run started by hand with `full`
   files none: whoever started it reads its result.
4. **Before a merge.** `scripts/gate.sh` runs the three whole on the Mac, as before, on the
   platform that writes the reports. What a pull request loses is the synthetic flights' and two
   logs' runs on the other four platforms and in the optimized build, and the fit's run on Linux:
   a change that moves a census only there reaches main and is found that night.
5. **Research notes.** A pull request or push that changes only files under `docs/research/`
   (renamed from or to, too) runs no step of `validate`, `examples` and `python`, and skips `types`,
   `wasm-check` and `phone`; none of them reads a note. Each still reports its check by name, as
   branch protection needs. `test` and `site`, which read the notes, run, and so do the quick
   Linux checks.
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
- [ADR-222][adr-222]'s platforms (five now; Intel Macs wait for #438) still build and test on every
  pull request; the examples, the Python package and the validation cases run on each one that
  changes more than research notes.
  Only the censuses move to the night.

**Amended 2026-11-04: M0.16a in three, and the slices of two censuses (M0.16a1).**

**Why three.** M0.16a's *done when* asked for three things at once: a slice of each census, held
exactly where it is written; the gate and pull requests checking the slices; and a pre-PR check of
the budget. The fit's slice can't be held the way the other two are. Its report differs from one
platform to another: `fit --check --other-platform` lets its counts stand a few flights off. So a
record of each flight's fit written on the Mac is exact only there, and the rule it keeps elsewhere
is a design of its own. The budget check fails every pull request until the fit's slice brings the
gate under 30 minutes, so it comes last. M0.16a is split in three, its *done when* whole across
them:

- **M0.16a1, two slices:** "the synthetic flights and two logs each check a fixed part of their
  flights against the committed report (exact where written, else by its rule), failing on one
  changed flight; the gate and pull requests check one part, the whole nightly."
- **M0.16a2, the fit's slice:** "the fit checks a fixed part of its flights against the committed
  report (exact where written, else by its rule), failing on one changed flight; the gate and pull
  requests check one part, the whole nightly."
- **M0.16a3, the budget checked:** "a pre-PR check fails on a full gate over 30 minutes, a job over
  20 in main's last passing CI, or no passing nightly in 26 hours; M7.3c3's fit follows before it
  merges."

M0.16a's *done when* before the split, kept here word for word: "the synthetic flights, two logs
and the fit each check a fixed part of their flights against the committed report (exact where
written, else by its rule), failing on one changed flight; the gate and pull requests check one
part, the whole nightly; a pre-PR check fails on a full gate over 30 minutes, a job over 20 in
main's last passing CI, or no passing nightly in 26 hours; M7.3c3's fit follows before it merges."

**The slices (M0.16a1).**

1. **A fixed part.** A census's slice is every 21st flight of each of its sets, from the first. For
   the synthetic flights that is 202 of 4,200: 10 of the tuning set's 200, 48 each of sets A and B,
   and 96 of the calibration set's 2,000. For two logs it is 202 of 4,200 too: 96 of the
   calibration set, 48 each of sets A and B, and 10 of the 200 staged flights. A flight's design,
   logger and conditions are drawn from streams of its own, so a fixed step samples them as fairly
   as a drawn one would. One draw does follow the index: the tuning and calibration sets give their
   odd flights set B's model. So the step is odd, and their slices hold both models, half and half.
   A first version took every 20th flight and held none of them, which a claims check of this
   change found.
2. **Exact where written.** The command that writes a census writes its slice beside the report,
   as `synthetic-flights.slice.json` and `two-logs.slice.json` in `validation/reports/`. Each flight
   has one line for each thing the census makes of it:
   - the synthetic flights: the flight's census row; each of a tuning flight's logs, with each
     reading's error as a share of its bound; how near its top acceleration came to the
     accelerometer's two ranges; on the tuning set and sets A and B, the smoother's checks (of the
     log, of the log written from launch, and with a GNSS track, of the track trusted from
     liftoff), each error over its standard deviation, with where the truth fell, and its health:
     whether it saw the pad, kept the accelerometer, found the apogee within the reading's bound and
     trusted the track; its transonic and charge spans and unsmoothed steps; and each sensor's
     samples, rejected, unweighed and resets (the flags are words, the counts
     numbers); and on sets A and
     B and the calibration set, what the log says of the readings that state an interval, with
     where each truth fell.
   - two logs: each log's sample count and highest height, and the offset's reading, truth and
     bound, its error as a share of the bound, with where the truth fell.

   Numbers keep the census's three significant figures. A share is kept to three decimals, a fixed
   step, so that an error near zero isn't written to digits that are only its rounding. `--slice` flies the slice's flights again and fails if any line differs, if a flight
   is missing or added, or if the slice takes another step, naming up to 20 such flights and
   counting the rest; flights in another order fail without one named. A line is held byte for
   byte on the build that writes the reports, the Mac's everyday build, and in words everywhere
   else (`--other-platform`): each number, NaN and infinity read as one mark, every word compared.
   Words are what the census counts as outcomes: where each truth fell, what was withheld and why, the
   smoother's flags, the design, the logger and the model.
3. **Else by its rule.** What only a whole set can show stays with the whole run: the census's
   ranges, counts and medians, the shares fitted on a calibration set, the smoother's coverage
   cells, and ADR-257's test with its probes. Each slice flight is held to the rules the census
   holds every flight to. On the synthetic flights, no reading may pass its bound, no log may be
   withheld while it has a barometer, and the smoother may leave no step unsolved. The logs written
   from launch must be smoothed on the pad they assume, and withheld in the same numbers, for the
   same reasons, as the whole logs, set by set. Every flight of sets A and B and of the calibration
   set must fly and leave no reading out but by its rule. On two logs, no offset may pass its bound,
   and one may be withheld only for a receiver's dropout. A rule broken names its flight where the
   rule is the flight's own, and its set where it is a count over the set's slice. `--slice` also
   reads the committed report and checks that it renders the committed page.
4. **Where each runs.** `--slice` runs in `scripts/gate.sh`'s validate step, in place of the whole
   check, byte for byte. CI's `validate` job runs `--slice --other-platform` on all five platforms,
   in the optimized build, on every pull request but one that changes only research notes. The
   whole `--check` runs each night and flies the slice too, holding it in words to the committed
   one, which only the whole run writes. A test reads `ci.yml` and `scripts/gate.sh` and fails if either check is missing,
   runs on one platform, or may fail unseen.
5. **One changed flight fails it.** A test changes one synthetic flight's truth by 0.5%, then its
   log, and checks that its lines change. Another moves one of two logs' clocks by 0.1 s, then
   reads one log 0.5% high. The comparison's own tests change one line of one flight and check that
   the failure names that flight and only it, and that in words a moved number passes and a moved
   word fails. A probe on the committed synthetic slice, not a test: set A's flight 399's "apogee
   height 0.031 inside" written "0.041 inside" fails `--slice` and passes `--slice
   --other-platform`; written "1.041 above" it fails `--slice --other-platform` too.

**Measured.** On the Mac that writes the reports (10 cores, the everyday build, 2026-11-04),
the whole synthetic flights' run took 486 s and two logs' 205 s; their slices took 27 s and 10 s.
The two whole runs wrote the reports byte for byte as committed, beside the two new slices of
327 kB and 49 kB. In the five full gates that ran the validate step and passed from 2026-11-03
21:40 to 11-04 04:59 UTC, it took 2,175 to 3,615 s. With the slices, this change's own full gate
took 1,999 s and its validate step 1,478 s at every 20th flight, and 2,529 s and 1,466 s at every
21st, its site step laying out every page again (one run each, 2026-11-04). The rest of that step,
the validation cases, the fit's whole run, the motor catalog and the scoreboard, isn't timed apart.
The gate stays over its 30 minutes until M0.16a2.

**Held two ways.** The first CI run of the slices held every line byte for byte on every
platform, and failed on Arm Linux: set A's flight 399 and set B's flight 504 differed in the
smoother's lines. The Mac shows the same. Its optimized build writes both reports and two logs' slice byte for
byte as the everyday build does, but the synthetic slice differs in one flight of the 202, flight
399, in three of its lines. The smoother's four shares move in their third decimal (0.031 to
0.033), its six shares with the GNSS track from liftoff by up to 0.004, and its eight shares on
the log written from launch by up to 0.087 (0.357 to 0.270), with that log's unweighed GNSS fixes
252 to 321. Each truth stays on its side, and every word is the same. The flight's simulated
apogee moves 1.2e-6 of itself between the two builds, the spread issue #984 is open to measure.
A tolerance on each number would have to know each flight's gain from its truth to its shares;
the words hold what the census counts without one.

**Alternatives to the slices.**

- **A hash of each flight's numbers.** Exact, and a few bytes a flight, but a failure would say only
  that a flight moved, not what moved or by how much.
- **The slice inside the report.** The report holds tallies; its page renders from it. A file of
  its own keeps the report's form, and the slice's lines readable in a diff.
- **A larger part, every 11th flight.** It doubles the time a pull request and the gate spend, for
  no rule a 21st can't check.

**The slices' consequences.**

- A change that moves only flights outside the slice reaches main and is found that night, as §4
  of the 2026-11-03 amendment says of the other platforms.
- A change meant to move the flights now writes three files a census, where it wrote two.
- Off the Mac, a pull request can't see a number that moves with no word: a share drifting inside
  its bound, or a count. The gate holds those byte for byte before each merge. The night holds the
  census's report to its digits, but a slice flight's own numbers only in words, so such a move
  shows on the Mac's gate alone.
- The fit still runs whole in the gate until M0.16a2 gives it a slice.
