# ADR-383: A branch's own run repairs the jobs' budget (2026-11-27)

- **Status:** accepted; fixes [issue #1658][i1658]; amends [ADR-149][adr-149] §1's check, as [ADR-365][adr-365] decision 6 amended its §3
- **Summary:** When main's last run of a pull request's checks has a job over the 20 minutes, the budget check before a pull request now takes a passing run started by hand on the branch's own head, without `full`, in main's place, and holds its jobs to the same 20 minutes. Without it the pull request that splits a slow job could not open. CI's slowest jobs are split in the same change: the tests in three parts on `windows-latest` and `macos-latest`, the fit's slice in four on `ubuntu-latest`, and the site's checks in four.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-365]: 0365-a-smoothed-track-past-earths-escape-speed-is-withheld-as-diverged.md
[i1658]: https://github.com/nrdptel/fusionspace-eridanus/issues/1658
[i1646]: https://github.com/nrdptel/fusionspace-eridanus/issues/1646

**Context.** `cargo xtask ci-budget` runs before every pull request opens. Its second check reads
main's newest passing run of a pull request's checks (one started by a push, or by hand without
`full`) and fails when any job of it ran past 1,200 s on its runner. After the fit's scan of the
wind ([issue #1646][i1646]) merged, two jobs passed that line on main's push run at e533d0aa and
again on a run started by hand at the same commit:

| job | push run 45965513904 | hand run 45973660626 |
|---|---:|---:|
| `test (windows-latest, 2/2)` | 1,266 s | 1,269 s |
| `fit slice (ubuntu-latest, 2/3)` | 1,214 s | 1,208 s |
| `test (macos-latest, 2/2)` | 1,214 s | under 957 s |

The site's `site (1/3)` took 1,066 s on the first. The Windows half spent about 9 minutes
building and 11 running its tests. So every pull request is refused until main has a faster run,
and main gets one only through a pull request: the split that fixes it is refused too. The
night's check had the same trap and a repair for it (ADR-365, decision 6); the jobs' check had
none.

**Decisions.**

1. **A branch's own run on its head stands in for main's.** When main's jobs fail the check,
   `ci-budget` reads the branch's own passing hand-started runs. The first one on the branch's
   head that ran a pull request's checks (`types` passed and no `census` job ran, as on main) has
   its jobs held to 1,200 s in main's place. Over the budget still fails, naming main's job and
   the branch's.
2. **Only the head's own run, and only on top of main.** The run's commit must hold every commit
   of main as GitHub has it, the rule of ADR-365's repair: a branch behind main never ran what main
   gained, which may be what made a job slow. Unlike the night's repair, a run on an earlier commit
   doesn't count even when only pages changed since, because a page moves the site's jobs.
3. **The split.** Each platform's test parts and fit-slice parts stay 1 to N of one split, so
   every test and every fit of the slice still runs once on each platform; tests in `ci.yml`'s
   readers hold that. The tests run in three parts on `windows-latest` and `macos-latest`, two on
   the other three platforms; the fit's slice in four parts on `ubuntu-latest`; the site's checks
   in four parts, each a quarter of the widths and of the pages' vitals.

**Consequences.**

- The pull request that splits a slow job opens after a hand-started run on its head (`gh
  workflow run ci.yml --ref <branch>`, `full` left off) passes within the budget. Once it merges,
  main's own runs are read again and the repair isn't needed.
- More jobs run: 3 more test parts, 1 more fit-slice part and 1 more site part, each with its
  own build. The issue stays open until main's runs show the longest job under 1,020 s three
  times in a row, its own *done when*.
- `xtask/src/ci_budget.rs`: `repaired_jobs_verdict` and its test,
  `a_branchs_own_run_on_its_head_stands_in_for_main_s_jobs`; the split's test,
  `every_platforms_test_parts_cover_its_tests_once`, beside the fit's
  `every_pull_request_and_the_gate_check_the_fits_slice`.
