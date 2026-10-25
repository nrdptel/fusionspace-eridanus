# How the pages read

`cargo xtask voice` writes this report from the pages. A test fails when the two disagree, so run it after editing a page. [The writing guide's voice section](../../docs/writing.md#voice) says what the pattern columns count and why, and sets the limits.

Rates are per 100 words, rounded up to two decimals, so a page over a limit never shows a rate at or under it; a rate over its limit is in bold. Words a sentence and the share of sentences over 25 words are rounded up too, to 0.1 and to 1%, so neither shows a page inside a target it misses. A stock phrase fails the build, and so does a page marked `held` that's over a limit or misses a sentence target. The 94 held pages are every page the site's table of contents lists, with the README and the 404 page, less the records page ([ADR-285](../../docs/decisions/0285-the-rest-of-the-site-in-nine.md)), and each snapshot of an `hpr` command's `--help` ([ADR-292](../../docs/decisions/0292-the-programs-words-in-four.md)).

102 pages, 445918 words, 0 stock phrases. 98 pages are within every limit.

| Pattern | Limit per 100 words | Pages over it | Highest rate |
|---|---:|---:|---|
| Bold-label items | 0.50 | 4 | 1.88, `docs/perf.md` |
| Labels | 0.50 | 3 | 3.62, `docs/VISION.md` |
| Sentences over 25 words | 0.50 | 3 | 1.17, `docs/perf.md` |
| Hedges | 1.00 | 0 | 0.26, `docs/checking-a-claim.md` |
| Stacked hedges | 0.10 | 0 | 0.02, `docs/physics/design.md` |
| Minimizers | 0.10 | 1 | 0.11, `docs/ARCHITECTURE.md` |
| Signposts | 0.10 | 0 | 0.01, `docs/format/flight-logs.md` |
| First person on a reference page | 0.00 | 0 | none |

The register has no limit of its own (first person on a reference page, above, is the one limit on it): "I", "we", "you" and contractions per 100 words, and the guide's sentence targets (at most 20 words a sentence on average, fewer than 10% of sentences over 25 words). A reference page is marked `ref`.

| Page | Words | Words a sentence | Over 25 words | Bold-label items | Labels | Sentences over 25 words | Hedges | Stacked hedges | Minimizers | Signposts | First person on a reference page | I | We | You | Contractions |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `.github/ISSUE_TEMPLATE/an-idea.md` | 28 | 0.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.00 |
| `.github/ISSUE_TEMPLATE/something-wrong.md` | 29 | 0.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.00 |
| `.github/pull_request_template.md` | 8 | 0.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.00 |
| `CHANGELOG.md` | 1810 | 20.9 | 31% | **0.94** | 0.00 | **0.89** | 0.06 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.06 | 0.23 |
| `CONTRIBUTING.md` | 671 | 13.7 | 5% | 0.00 | 0.00 | 0.30 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 1.05 |
| `README.md` (`held`) | 2464 | 13.9 | 2% | 0.17 | 0.17 | 0.09 | 0.05 | 0.00 | 0.00 | 0.00 | | 0.17 | 0.00 | 0.82 | 0.65 |
| `crates/hpr-cli/tests/help/hpr-analyze.help` (`ref`, `held`) | 288 | 13.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.35 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-compare.help` (`ref`, `held`) | 50 | 7.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-completions.help` (`ref`, `held`) | 78 | 7.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.29 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-convert.help` (`ref`, `held`) | 174 | 13.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.58 |
| `crates/hpr-cli/tests/help/hpr-diagnose.help` (`ref`, `held`) | 53 | 7.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-mc.help` (`ref`, `held`) | 572 | 12.8 | 0% | 0.00 | 0.00 | 0.00 | 0.18 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-fetch.help` (`ref`, `held`) | 146 | 10.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-list.help` (`ref`, `held`) | 96 | 8.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-search.help` (`ref`, `held`) | 159 | 9.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-show.help` (`ref`, `held`) | 96 | 9.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors.help` (`ref`, `held`) | 172 | 11.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-optimize.help` (`ref`, `held`) | 50 | 7.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-sim.help` (`ref`, `held`) | 404 | 12.3 | 0% | 0.00 | 0.00 | 0.00 | 0.25 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-era5.help` (`ref`, `held`) | 132 | 8.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-gfs.help` (`ref`, `held`) | 158 | 9.9 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-open-meteo.help` (`ref`, `held`) | 189 | 10.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-rap.help` (`ref`, `held`) | 164 | 10.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-wyoming.help` (`ref`, `held`) | 182 | 10.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather.help` (`ref`, `held`) | 199 | 11.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr.help` (`ref`, `held`) | 287 | 12.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/404.md` (`held`) | 51 | 12.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 1.97 | 0.00 |
| `docs/ARCHITECTURE.md` | 1860 | 11.7 | 7% | **1.13** | **2.42** | 0.38 | 0.06 | 0.00 | **0.11** | 0.00 | | 0.00 | 0.11 | 0.00 | 0.11 |
| `docs/VALIDATION.md` (`held`) | 10233 | 13.9 | 1% | 0.26 | 0.00 | 0.03 | 0.05 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.06 | 0.56 |
| `docs/VISION.md` | 3734 | 15.4 | 15% | **1.10** | **3.62** | **0.59** | 0.19 | 0.00 | 0.09 | 0.00 | | 0.00 | 0.00 | 0.03 | 0.00 |
| `docs/accuracy.md` (`held`) | 15752 | 13.3 | 1% | 0.23 | 0.00 | 0.02 | 0.06 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.49 |
| `docs/api.md` (`held`) | 2514 | 13.2 | 0% | 0.16 | 0.00 | 0.00 | 0.20 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.40 | 0.36 |
| `docs/checking-a-claim.md` (`held`) | 1575 | 14.8 | 0% | 0.26 | 0.07 | 0.00 | 0.26 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.13 | 1.21 |
| `docs/cli.md` (`ref`, `held`) | 17609 | 13.4 | 0% | 0.24 | 0.02 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.22 | 0.61 |
| `docs/custom-models.md` (`held`) | 1842 | 13.7 | 0% | 0.00 | 0.00 | 0.00 | 0.22 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.93 | 1.09 |
| `docs/drag-through-mach-1.md` (`held`) | 1996 | 14.8 | 2% | 0.36 | 0.00 | 0.11 | 0.16 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.06 | 0.81 |
| `docs/elevation.md` (`held`) | 4082 | 14.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.32 | 0.79 |
| `docs/evidence.md` (`held`) | 6551 | 12.9 | 1% | 0.00 | 0.02 | 0.02 | 0.04 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.33 |
| `docs/exporting-a-flight.md` (`held`) | 1937 | 14.6 | 0% | 0.00 | 0.00 | 0.00 | 0.06 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.21 | 0.93 |
| `docs/fly-your-ork.md` (`held`) | 1420 | 12.6 | 0% | 0.00 | 0.00 | 0.00 | 0.15 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.78 | 0.64 |
| `docs/format/eng.md` (`ref`, `held`) | 2410 | 13.6 | 2% | 0.38 | 0.21 | 0.09 | 0.13 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.05 | 0.17 |
| `docs/format/era5.md` (`ref`, `held`) | 2437 | 13.5 | 0% | 0.42 | 0.13 | 0.00 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.42 | 0.25 |
| `docs/format/flight-logs.md` (`ref`, `held`) | 44294 | 13.1 | 0% | 0.21 | 0.21 | 0.00 | 0.08 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.04 | 0.46 |
| `docs/format/hpr.md` (`ref`, `held`) | 5413 | 13.9 | 0% | 0.39 | 0.02 | 0.00 | 0.10 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.13 | 0.80 |
| `docs/format/orc.md` (`ref`, `held`) | 2418 | 13.3 | 0% | 0.38 | 0.13 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.21 | 0.38 |
| `docs/format/ork.md` (`ref`, `held`) | 37818 | 14.0 | 0% | 0.21 | 0.13 | 0.00 | 0.10 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.05 | 0.14 |
| `docs/format/pf2.md` (`ref`, `held`) | 815 | 13.9 | 0% | 0.00 | 0.13 | 0.00 | 0.25 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.23 |
| `docs/format/rse.md` (`ref`, `held`) | 2241 | 13.8 | 0% | 0.27 | 0.09 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.05 |
| `docs/getting-started.md` (`held`) | 4155 | 13.6 | 2% | 0.27 | 0.00 | 0.10 | 0.10 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.87 | 0.73 |
| `docs/glossary.md` (`ref`, `held`) | 13102 | 13.2 | 0% | 0.00 | 0.10 | 0.00 | 0.07 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.06 | 0.13 |
| `docs/how-a-flight-is-simulated.md` (`held`) | 2127 | 14.2 | 0% | 0.33 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.15 | 0.33 |
| `docs/monte-carlo.md` (`held`) | 3572 | 13.8 | 0% | 0.23 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.45 | 0.73 |
| `docs/motor-stock.md` (`held`) | 3539 | 13.7 | 0% | 0.00 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.26 | 1.11 |
| `docs/nomads.md` (`held`) | 4492 | 13.4 | 0% | 0.00 | 0.00 | 0.00 | 0.03 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.25 | 0.85 |
| `docs/online-data.md` (`held`) | 2165 | 14.6 | 0% | 0.10 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.14 | 1.02 |
| `docs/optimization.md` (`held`) | 7642 | 14.2 | 0% | 0.23 | 0.00 | 0.00 | 0.11 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.28 | 0.59 |
| `docs/perf.md` | 5426 | 17.3 | 24% | **1.88** | **0.78** | **1.17** | 0.02 | 0.00 | 0.02 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.17 |
| `docs/physics/aero.md` (`ref`, `held`) | 58706 | 14.4 | 0% | 0.35 | 0.35 | 0.00 | 0.07 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.02 | 0.12 |
| `docs/physics/atmosphere.md` (`ref`, `held`) | 2347 | 12.5 | 0% | 0.13 | 0.30 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.18 |
| `docs/physics/design.md` (`ref`, `held`) | 9149 | 14.2 | 0% | 0.14 | 0.18 | 0.00 | 0.10 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.46 |
| `docs/physics/flight.md` (`ref`, `held`) | 4652 | 13.7 | 0% | 0.26 | 0.33 | 0.00 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.09 |
| `docs/physics/flutter.md` (`ref`, `held`) | 2836 | 13.5 | 0% | 0.36 | 0.08 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.15 | 0.18 |
| `docs/physics/frames.md` (`ref`, `held`) | 1369 | 12.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/geodesy.md` (`ref`, `held`) | 1735 | 12.3 | 1% | 0.24 | 0.00 | 0.06 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.12 | 0.06 |
| `docs/physics/gravity.md` (`ref`, `held`) | 1134 | 11.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/integration.md` (`ref`, `held`) | 2058 | 12.5 | 0% | 0.20 | 0.25 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.25 |
| `docs/physics/interpolation.md` (`ref`, `held`) | 547 | 11.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/log-readings.md` (`ref`, `held`) | 16237 | 14.7 | 0% | 0.33 | 0.13 | 0.00 | 0.17 | 0.00 | 0.01 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.34 |
| `docs/physics/magnetic.md` (`ref`, `held`) | 2229 | 13.9 | 0% | 0.14 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.14 | 0.00 |
| `docs/physics/mass.md` (`ref`, `held`) | 9793 | 12.9 | 1% | 0.21 | 0.04 | 0.02 | 0.07 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.10 | 0.35 |
| `docs/physics/metrics.md` (`ref`, `held`) | 5556 | 14.3 | 0% | 0.04 | 0.22 | 0.00 | 0.04 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.11 |
| `docs/physics/motor.md` (`ref`, `held`) | 5636 | 13.7 | 1% | 0.32 | 0.22 | 0.02 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.06 | 0.18 |
| `docs/physics/moving-mass.md` (`ref`, `held`) | 2491 | 13.5 | 0% | 0.09 | 0.05 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.09 | 0.65 |
| `docs/physics/quadrature.md` (`ref`, `held`) | 494 | 11.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/recovery.md` (`ref`, `held`) | 12598 | 14.0 | 0% | 0.20 | 0.22 | 0.00 | 0.10 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.06 | 0.16 |
| `docs/physics/released-mass.md` (`ref`, `held`) | 2801 | 13.8 | 0% | 0.04 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.15 | 0.43 |
| `docs/physics/shapes.md` (`ref`, `held`) | 2014 | 13.0 | 0% | 0.20 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.20 |
| `docs/physics/staging.md` (`ref`, `held`) | 7205 | 14.3 | 0% | 0.46 | 0.24 | 0.00 | 0.20 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.06 | 0.10 |
| `docs/physics/synthetic-flights.md` (`ref`, `held`) | 3982 | 14.6 | 1% | 0.46 | 0.08 | 0.03 | 0.06 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.21 |
| `docs/physics/turbulence.md` (`ref`, `held`) | 1292 | 11.3 | 0% | 0.31 | 0.00 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.24 |
| `docs/physics/wind.md` (`ref`, `held`) | 2989 | 14.6 | 0% | 0.37 | 0.07 | 0.00 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.31 | 0.27 |
| `docs/pick-a-motor.md` (`held`) | 1166 | 13.3 | 0% | 0.00 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 1.46 | 0.35 |
| `docs/plan.md` (`held`) | 1064 | 10.4 | 0% | 0.00 | 0.19 | 0.00 | 0.10 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.57 |
| `docs/python.md` (`held`) | 4627 | 14.2 | 1% | 0.07 | 0.00 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.46 | 0.78 |
| `docs/reading-a-flight-log.md` (`held`) | 2329 | 13.7 | 0% | 0.48 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.35 | 0.82 |
| `docs/recording-a-trajectory.md` (`held`) | 821 | 14.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.74 |
| `docs/releasing.md` (`held`) | 2587 | 14.2 | 0% | 0.28 | 0.00 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.35 | 0.74 |
| `docs/sensitivity.md` (`held`) | 2301 | 14.7 | 0% | 0.27 | 0.00 | 0.00 | 0.14 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.57 | 0.66 |
| `docs/simulator-compared.md` (`held`) | 2588 | 12.8 | 4% | 0.00 | 0.00 | 0.28 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.04 | 0.00 | 0.16 | 0.70 |
| `docs/simulator-tour.md` (`held`) | 1573 | 13.9 | 0% | 0.45 | 0.00 | 0.00 | 0.07 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.32 | 0.51 |
| `docs/soundings.md` (`held`) | 5024 | 14.3 | 0% | 0.18 | 0.00 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.90 |
| `docs/stability-for-certification.md` (`held`) | 1988 | 14.5 | 1% | 0.00 | 0.00 | 0.06 | 0.26 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.66 | 0.46 |
| `docs/stability-in-flight.md` (`held`) | 1848 | 15.1 | 2% | 0.22 | 0.00 | 0.11 | 0.11 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.00 | 1.03 |
| `docs/start-here.md` (`held`) | 7485 | 15.6 | 4% | 0.22 | 0.05 | 0.22 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.07 | 0.00 | 0.42 | 0.69 |
| `docs/the-builder.md` (`held`) | 4119 | 12.8 | 0% | 0.20 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.05 | 0.00 | 0.47 | 0.59 |
| `docs/the-simulator.md` (`held`) | 797 | 15.1 | 2% | 0.00 | 0.00 | 0.13 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.13 | 0.00 | 0.88 | 1.13 |
| `docs/trusting-a-simulation.md` (`held`) | 1555 | 15.2 | 2% | 0.00 | 0.00 | 0.13 | 0.20 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.07 | 1.03 |
| `docs/weather.md` (`held`) | 1974 | 13.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.31 | 0.87 |
| `docs/when-fins-flutter.md` (`held`) | 1773 | 14.9 | 1% | 0.23 | 0.00 | 0.06 | 0.12 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.17 | 0.68 |
| `docs/writing.md` (`held`) | 9672 | 14.4 | 1% | 0.40 | 0.17 | 0.07 | 0.18 | 0.00 | 0.03 | 0.00 | | 0.03 | 0.02 | 0.07 | 0.96 |
| `docs/your-own-logger.md` (`held`) | 2407 | 14.5 | 0% | 0.00 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.42 | 0.67 |
| `docs/your-own-rocket.md` (`held`) | 4489 | 14.5 | 0% | 0.36 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.38 | 0.67 |
