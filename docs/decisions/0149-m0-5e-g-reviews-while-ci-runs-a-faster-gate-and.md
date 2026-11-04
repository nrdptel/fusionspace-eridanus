# ADR-149: M0.5e to g, reviews while CI runs, a faster gate and a split CI (2026-10-04)

- **Status:** accepted; amended 2026-11-03 (below: a time budget for CI and the gate, the flight censuses each night) and 2026-11-04 (M0.16a in three; the slices of the synthetic flights, two logs and the fit)
- **Summary:** Reviews start once the PR is open and CI runs, with a shared bar for blocking findings and scoped re-reviews; the gate runs tests in parallel and adds the private-data checks when physics changes; CI's slowest job is split. Amended 2026-11-03: a pull request's CI within 20 minutes and a passing full gate within 30; the flight censuses (the synthetic flights, two logs and the fit's intervals) run whole each night on main, a failure filing a P-critical issue, and a change to research notes alone skips the jobs that read none. Amended 2026-11-04: pull requests and the gate fly every 21st flight of the synthetic flights and two logs, and every 21st fit of the fit, held line for line to a committed slice.

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
   step, so that an error near zero isn't written to digits that are only its rounding. `--slice`
   flies the slice's flights again and fails if any line differs, if a flight
   is missing or added, or if the slice takes another step, naming up to 20 such flights and
   counting the rest; flights in another order fail without one named. A line is held byte for
   byte on the build that writes the reports, the Mac's everyday build, and in words everywhere
   else (`--other-platform`): each number read as one mark, every word compared, NaN and infinity
   kept as words, since a share that stops being a number is a change, not a build's rounding.
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
   one, which only the whole run writes. So the gate is the one place a slice flight's numbers are
   held byte for byte. A test reads `ci.yml` and `scripts/gate.sh` and fails if either check is missing,
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
327 kB and 49 kB; with the smoother's health added, the synthetic slice is 375 kB, and the slices
take 27 s and 10 s still. In the five full gates that ran the validate step and passed from 2026-11-03
21:40 to 11-04 04:59 UTC, it took 2,175 to 3,615 s. With the slices, this change's own full gate
took 1,999 s and its validate step 1,478 s at every 20th flight, and 2,529 s and 1,466 s at every
21st, its site step laying out every page again (one run each, 2026-11-04). The rest of that step,
the validation cases, the fit's whole run, the motor catalog and the scoreboard, isn't timed apart.
The gate stays over its 30 minutes until M0.16a2.

**Held two ways.** Held byte for byte on every platform, the slices failed CI's optimized build on
macOS and both Linux runners: set A's flight 399 on all three, and set B's flight 504 on Linux, in
the smoother's lines; both Windows runners passed. The Mac reproduces flight
399, not 504. Its optimized build writes both reports and two logs' slice byte for
byte as the everyday build does, but the synthetic slice differs in one flight of the 202, flight
399, in three of its lines. The smoother's four shares move in their third decimal (0.031 to
0.033), its six shares with the GNSS track from liftoff by up to 0.004, and its eight shares on
the log written from launch by up to 0.087 (0.357 to 0.270), with that log's unweighed GNSS fixes
252 to 321. Each truth stays on its side, and every word is the same. The flight's simulated
apogee moves 1.2e-6 of itself between the two builds (one measurement, on one Mac); issue #984 is
open to measure the same kind of spread in the landing time, across platforms.
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
- A word can still flip off the Mac. In the committed slice, 27 of the smoother's shares, in 11
  flights, sit within 0.087 of the 1.645 standard deviations its sides are taken at, though none
  flipped in the builds and platforms compared. A flip fails that platform's pull requests, and
  writing the slice again on the Mac can't fix it; as with the reports, that needs a record, not a
  looser check.
- The fit still runs whole in the gate until M0.16a2 gives it a slice.

**The fit's slice (M0.16a2).**

1. **A fixed part.** The fit's slice is part 1 of 21 of each of its runs (`fit --part 1/21`): the
   1st, 22nd, 43rd fit and on. It holds 444 fits: 123 each of sets A and B, 62 of the first 1,300
   flights and 61 of the wind's flights past them; 10 of each of the seven runs of pure noise; and
   128 of the calibration set, 96 of its 2,000 flights and 32 of its wind's flights past them, half
   of the 96 on each model: 444 of the 9,192 fits a whole run makes, 6,538 for the report and
   2,654 for the shares. The calibration set's two runs join the fit's runs, so the parts CI fits
   each night hold them too. A fit is the same whatever its thread or part fitted before it, which
   a test holds to the bit for one of set A's flights fitted alone and after another, so the
   slice's fits are the whole run's at the same places; `fit --slice` making the whole run's slice
   again byte for byte, and eight parts on Linux agreeing with one job (ADR-316 §7, on the report's
   6,538 fits), bear it out.
2. **Exact where written.** The run that writes the report, `fit` in the optimized build on the
   Mac, writes the slice beside it as `fit-intervals.slice.json`. Each fit has a line for what it
   came to: fitted or with nothing to fit, a track or height alone, converged or at its step
   limit, its residual, its RMS, the flights it flew and its design. Then a line for each of the
   five parameters: its value, standard error and truth, its error over its standard error, and
   where its truth fell against the 90% interval the report counts, or "not counted" where the
   report counts none (a log with nothing to fit, a wind without a track). A fit of the calibration
   set has neither: its errors are what the shares are fitted on, so some sit on an interval's end
   by construction (calibration flight 441's wind toward the east, at the share's −0.369), where a
   side would turn on a platform's last digits and say nothing the report counts. A physics review
   of this change found it. Numbers keep three significant
   figures, shares three decimals, as in the other two slices. `fit --slice` makes the slice's fits
   again and holds them to the committed slice with the same comparison as the other two: byte for
   byte in the gate, which runs the build that writes it, and in words (`--other-platform`)
   elsewhere. `fit --check` makes the slice from the whole run and holds it byte for byte; the
   night's `--check --other-platform --parts` holds it in words.
3. **Else by its rule.** The rules the report holds each fit to hold each of the slice's: on sets
   A and B, a fit among the flights drawn, or one of the wind's, may not stand past the misfit's
   cut, and a log among the flights drawn with nothing to fit must be a GNSS receiver's alone. Each
   names its flight. The flights drawn are the committed report's. The runs of noise and the
   calibration set have no rule for one fit, so their 198 fits in the slice are held by their
   lines alone. What only a whole run can show stays with
   the night: the counts, the tests and their probes, the widths that say what's identified, and
   the shares fitted on the calibration set. Those shares were checked only in the gate, which ran
   `fit --check`; CI's runs left the calibration set out. Now the run that writes the report and
   `fit --check` hold them to the library's to their three figures, and the night holds them
   within the rule the report's figures keep on another platform: 5% of the larger, or 0.002.
   `--slice` fits the slice's 128 calibration fits but no whole calibration set, so it fits no
   shares. It reads the committed report and holds it to its own rules: each test passing and each
   probe failing by its stored bands (worked out from its counts when it was written, not again),
   set A's scales identified, nothing on noise, no fit past the misfit's cut, every log with
   nothing to fit a GNSS receiver's alone, and its stored shares the library's. It also checks that
   the report renders the committed page.
4. **Where each runs.** `fit --slice` runs in the gate's validate step in place of `fit --check`.
   On every pull request but one that changes only research notes, a job of its own, `fit slice`,
   runs `fit --slice --other-platform` on all five platforms. It sits beside `validate`, which took
   14.6 and 15.6 minutes on Windows and macOS on M0.16a1's pull request; its 4.4 to 7.7 minutes of
   fits on CI's runners would take `validate` past the 20 minutes. The whole fit runs each night in its eight parts on Linux. A test reads `ci.yml` and
   `scripts/gate.sh` and fails if either check is missing, runs on fewer platforms or on a
   condition, or may fail unseen, or if the deploy and the night's alert don't wait for it.
5. **One changed fit fails it.** A test moves one made-up fit's value, its truth past its interval,
   its convergence and whether it had something to fit, and checks that its lines change, in a number or a word, and
   that a slice holding it fails naming that fit alone, byte for byte and in words. A probe on the
   committed slice, not a test: set A's fit 819's drag share written 0.392 for 0.391 fails `fit
   --slice` and passes `fit --slice --other-platform`; its "inside" written "above" fails both. Off
   the Mac a calibration fit's lines hold no side, so there only its first line's words can fail:
   fitted or not, track or height alone, converged or not, whether it has a residual, its design. The night holds the shares
   its numbers make.

**Measured.** On the Mac that writes the report (10 cores, the optimized build, 2026-11-04), the
whole run, its 9,192 fits, took 1,329 s, a test build sharing the machine part of the time, and
wrote the report and its page byte for byte as committed, beside a slice of 286 kB; with the
review's change to the calibration lines, it took 1,400 s and left the report and its page so
again, beside a slice of 274 kB. The slice took 79 s in this change's last full gate, whose
validate step took 215 s (1,472 s with the fit whole at M0.16a1) and the whole gate 661 s. In a
`full` CI run on this change's first commit, `fit slice` held the slice in words on all five platforms. Its
fits took 266 to 464 s, the longest on `windows-latest`, and its jobs 8 to 18 minutes with a build
from no cache, as a new job's first run has. The night's parts and `fit intervals` passed on
Linux: 22 numbers of the report stood apart from the committed one, by at most one flight and
3.38%, the shares held within their rule and the slice in every word. The shares come from the fits alone, so the night's check run again on the Mac from those eight
parts gives Linux's shares: at most 0.002 from the library's at three figures, against the rule's
0.002 or 5%; it held the slice, its calibration lines without sides, in every word too. Each part now fits
1,146 to 1,151 flights, not about 820, in 781 to 1,075 s, and its job took 951 to 1,301 s,
against 587 to 999 s in main's last full run before it (run 41753918203, 2026-11-03).

**The fit's slice's consequences.**

- The gate's validate step makes 444 fits, not 9,192. It no longer holds the whole report exactly
  and refits no shares: only `fit --check`, run by hand on the Mac, does both, and the run that
  writes the report holds the fitted shares. A change that moves fits outside the slice is found that night on Linux
  if it moves a count by more than 5 flights or a figure or share past its rule. A smaller move is
  found only when the report is checked or written again on the Mac. A change that moves a line
  of a fit in the slice, a number at its three figures or decimals or a word, fails the gate.
  Issue #1177 is open to hold the whole report exactly in an automatic run again.
- A pull request's CI gains five jobs, each a build and 4 to 8 minutes of fits.
- Two of the night's eight fit parts now pass 20 minutes (1,217 and 1,301 s). The budget is a
  pull request's, but M0.16a3's check
  reads main's last passing run, which may be a night's: that increment splits the parts further
  if it counts them.
