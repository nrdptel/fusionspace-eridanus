# How the pages read

`cargo xtask voice` writes this report from the pages. A test fails when the two disagree, so run it after editing a page. [The writing guide's voice section](../../docs/writing.md#voice) says what the pattern columns count and why, and sets the limits.

Rates are per 100 words, rounded up to two decimals, so a page over a limit never shows a rate at or under it; a rate over its limit is in bold. Words a sentence and the share of sentences over 25 words are rounded up too, to 0.1 and to 1%, so neither shows a page inside a target it misses. A stock phrase fails the build, and so does a page marked `held` that's over a limit or misses a sentence target. The 131 held pages are every page the site's table of contents lists, with the README and the 404 page, less the records page ([ADR-285](../../docs/decisions/0285-the-rest-of-the-site-in-nine.md)), each snapshot of an `hpr` command's `--help`, the census of what the commands say, a page for each crate's `src/` directory, the Python package's README and its docstrings as `help()` shows them, and each crate's front page, the `//!` docs of its `src/lib.rs` ([ADR-292](../../docs/decisions/0292-the-programs-words-in-four.md)).

139 pages, 521805 words, 0 stock phrases. 135 pages are within every limit.

| Pattern | Limit per 100 words | Pages over it | Highest rate |
|---|---:|---:|---|
| Bold-label items | 0.50 | 4 | 1.88, `docs/perf.md` |
| Labels | 0.50 | 3 | 3.62, `docs/VISION.md` |
| Sentences over 25 words | 0.50 | 3 | 1.16, `docs/perf.md` |
| Hedges | 1.00 | 0 | 0.53, `crates/hpr-sim/src/` |
| Stacked hedges | 0.10 | 0 | 0.03, `crates/hpr-sim/src/` |
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
| `README.md` (`held`) | 2467 | 14.0 | 3% | 0.17 | 0.17 | 0.13 | 0.05 | 0.00 | 0.00 | 0.00 | | 0.17 | 0.00 | 0.82 | 0.65 |
| `crates/hpr-cli/tests/help/hpr-analyze.help` (`ref`, `held`) | 288 | 13.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.35 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-compare.help` (`ref`, `held`) | 50 | 7.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-completions.help` (`ref`, `held`) | 78 | 7.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.29 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-convert.help` (`ref`, `held`) | 174 | 13.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.58 |
| `crates/hpr-cli/tests/help/hpr-diagnose.help` (`ref`, `held`) | 53 | 7.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-mc.help` (`ref`, `held`) | 613 | 13.1 | 0% | 0.00 | 0.00 | 0.00 | 0.17 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-fetch.help` (`ref`, `held`) | 146 | 10.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-list.help` (`ref`, `held`) | 96 | 8.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-search.help` (`ref`, `held`) | 159 | 9.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-show.help` (`ref`, `held`) | 96 | 9.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors-update.help` (`ref`, `held`) | 106 | 10.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-motors.help` (`ref`, `held`) | 190 | 11.9 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-optimize.help` (`ref`, `held`) | 50 | 7.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-sim.help` (`ref`, `held`) | 441 | 12.6 | 0% | 0.00 | 0.00 | 0.00 | 0.23 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-era5.help` (`ref`, `held`) | 132 | 8.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-gfs.help` (`ref`, `held`) | 158 | 9.9 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-open-meteo.help` (`ref`, `held`) | 189 | 10.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-rap.help` (`ref`, `held`) | 164 | 10.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather-wyoming.help` (`ref`, `held`) | 182 | 10.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr-weather.help` (`ref`, `held`) | 199 | 11.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/tests/help/hpr.help` (`ref`, `held`) | 287 | 12.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-py/README.md` (`held`) | 341 | 12.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.30 | 0.30 |
| `crates/hpr-py/tests/docstrings.md` (`ref`, `held`) | 4046 | 13.3 | 1% | 0.00 | 0.00 | 0.08 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.08 | 0.72 |
| `docs/404.md` (`held`) | 51 | 12.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 1.97 | 0.00 |
| `docs/ARCHITECTURE.md` | 1860 | 11.7 | 7% | **1.13** | **2.42** | 0.38 | 0.06 | 0.00 | **0.11** | 0.00 | | 0.00 | 0.11 | 0.00 | 0.11 |
| `docs/VALIDATION.md` (`held`) | 10521 | 15.1 | 4% | 0.25 | 0.05 | 0.18 | 0.05 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.06 | 0.57 |
| `docs/VISION.md` | 3734 | 15.4 | 15% | **1.10** | **3.62** | **0.59** | 0.19 | 0.00 | 0.09 | 0.00 | | 0.00 | 0.00 | 0.03 | 0.00 |
| `docs/accuracy.md` (`held`) | 15834 | 13.5 | 2% | 0.23 | 0.00 | 0.06 | 0.06 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.47 |
| `docs/api.md` (`held`) | 2515 | 13.2 | 0% | 0.16 | 0.00 | 0.00 | 0.20 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.40 | 0.36 |
| `docs/checking-a-claim.md` (`held`) | 1575 | 14.8 | 0% | 0.26 | 0.07 | 0.00 | 0.26 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.13 | 1.21 |
| `docs/cli.md` (`ref`, `held`) | 18282 | 13.9 | 2% | 0.22 | 0.02 | 0.11 | 0.08 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.23 | 0.61 |
| `docs/custom-models.md` (`held`) | 1800 | 13.6 | 0% | 0.00 | 0.00 | 0.00 | 0.23 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.95 | 0.95 |
| `docs/drag-through-mach-1.md` (`held`) | 1957 | 15.0 | 3% | 0.36 | 0.00 | 0.16 | 0.11 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.06 | 0.82 |
| `docs/elevation.md` (`held`) | 4086 | 14.7 | 2% | 0.00 | 0.00 | 0.08 | 0.05 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.30 | 0.86 |
| `docs/evidence.md` (`held`) | 7113 | 13.0 | 2% | 0.00 | 0.00 | 0.09 | 0.03 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.33 |
| `docs/exporting-a-flight.md` (`held`) | 1931 | 14.6 | 0% | 0.00 | 0.00 | 0.00 | 0.06 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.21 | 0.94 |
| `docs/fly-your-ork.md` (`held`) | 1439 | 12.7 | 0% | 0.00 | 0.00 | 0.00 | 0.14 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.84 | 0.56 |
| `docs/format/eng.md` (`ref`, `held`) | 2466 | 13.9 | 3% | 0.37 | 0.21 | 0.17 | 0.13 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.05 | 0.21 |
| `docs/format/era5.md` (`ref`, `held`) | 2442 | 13.6 | 0% | 0.41 | 0.13 | 0.00 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.41 | 0.25 |
| `docs/format/flight-logs.md` (`ref`, `held`) | 44180 | 13.1 | 1% | 0.23 | 0.21 | 0.02 | 0.09 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.04 | 0.46 |
| `docs/format/hpr.md` (`ref`, `held`) | 5402 | 14.0 | 1% | 0.39 | 0.02 | 0.06 | 0.10 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.13 | 0.82 |
| `docs/format/orc.md` (`ref`, `held`) | 2412 | 13.3 | 0% | 0.38 | 0.13 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.21 | 0.38 |
| `docs/format/ork.md` (`ref`, `held`) | 38943 | 14.2 | 1% | 0.21 | 0.13 | 0.03 | 0.10 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.05 | 0.14 |
| `docs/format/pf2.md` (`ref`, `held`) | 815 | 13.9 | 0% | 0.00 | 0.13 | 0.00 | 0.25 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.23 |
| `docs/format/rse.md` (`ref`, `held`) | 2256 | 14.0 | 0% | 0.27 | 0.09 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.05 |
| `docs/getting-started.md` (`held`) | 4150 | 13.6 | 2% | 0.27 | 0.00 | 0.10 | 0.10 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.87 | 0.73 |
| `docs/glossary.md` (`ref`, `held`) | 13151 | 13.2 | 1% | 0.00 | 0.10 | 0.02 | 0.07 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.06 | 0.13 |
| `docs/how-a-flight-is-simulated.md` (`held`) | 2127 | 14.2 | 0% | 0.33 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.15 | 0.33 |
| `docs/monte-carlo.md` (`held`) | 3574 | 13.8 | 0% | 0.23 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.45 | 0.73 |
| `docs/motor-stock.md` (`held`) | 3503 | 14.0 | 2% | 0.00 | 0.00 | 0.09 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.26 | 1.09 |
| `docs/nomads.md` (`held`) | 4483 | 13.6 | 1% | 0.00 | 0.00 | 0.05 | 0.03 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.23 | 0.79 |
| `docs/online-data.md` (`held`) | 2165 | 14.6 | 0% | 0.10 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.14 | 1.02 |
| `docs/optimization.md` (`held`) | 7680 | 14.2 | 0% | 0.23 | 0.00 | 0.00 | 0.11 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.28 | 0.59 |
| `docs/perf.md` | 5441 | 17.3 | 24% | **1.88** | **0.78** | **1.16** | 0.02 | 0.00 | 0.02 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.17 |
| `docs/physics/aero.md` (`ref`, `held`) | 58809 | 14.4 | 1% | 0.36 | 0.34 | 0.03 | 0.07 | 0.01 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.02 | 0.12 |
| `docs/physics/atmosphere.md` (`ref`, `held`) | 2349 | 12.5 | 1% | 0.13 | 0.30 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.18 |
| `docs/physics/design.md` (`ref`, `held`) | 9149 | 14.2 | 0% | 0.14 | 0.18 | 0.00 | 0.10 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.46 |
| `docs/physics/flight.md` (`ref`, `held`) | 4657 | 13.7 | 0% | 0.26 | 0.33 | 0.00 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.09 |
| `docs/physics/flutter.md` (`ref`, `held`) | 2829 | 13.4 | 0% | 0.36 | 0.08 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.15 | 0.18 |
| `docs/physics/frames.md` (`ref`, `held`) | 1369 | 12.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/geodesy.md` (`ref`, `held`) | 1734 | 12.2 | 1% | 0.24 | 0.00 | 0.06 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.12 | 0.06 |
| `docs/physics/gravity.md` (`ref`, `held`) | 1133 | 11.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/integration.md` (`ref`, `held`) | 2062 | 12.5 | 0% | 0.20 | 0.25 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.25 |
| `docs/physics/interpolation.md` (`ref`, `held`) | 547 | 11.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/log-readings.md` (`ref`, `held`) | 19257 | 15.7 | 5% | 0.28 | 0.12 | 0.23 | 0.17 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.34 |
| `docs/physics/magnetic.md` (`ref`, `held`) | 2231 | 13.9 | 1% | 0.14 | 0.00 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.14 | 0.00 |
| `docs/physics/mass.md` (`ref`, `held`) | 9868 | 12.9 | 1% | 0.21 | 0.04 | 0.05 | 0.07 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.10 | 0.35 |
| `docs/physics/metrics.md` (`ref`, `held`) | 5554 | 14.2 | 0% | 0.04 | 0.22 | 0.00 | 0.04 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.02 | 0.11 |
| `docs/physics/motor.md` (`ref`, `held`) | 6828 | 14.7 | 6% | 0.30 | 0.20 | 0.36 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.05 | 0.25 |
| `docs/physics/moving-mass.md` (`ref`, `held`) | 2488 | 13.4 | 0% | 0.09 | 0.05 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.09 | 0.65 |
| `docs/physics/quadrature.md` (`ref`, `held`) | 494 | 11.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `docs/physics/recovery.md` (`ref`, `held`) | 12594 | 14.0 | 0% | 0.20 | 0.22 | 0.00 | 0.10 | 0.00 | 0.01 | 0.01 | 0.00 | 0.00 | 0.00 | 0.06 | 0.16 |
| `docs/physics/released-mass.md` (`ref`, `held`) | 2798 | 13.8 | 0% | 0.04 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.15 | 0.43 |
| `docs/physics/shapes.md` (`ref`, `held`) | 2014 | 13.0 | 0% | 0.20 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.20 |
| `docs/physics/smoother.md` (`ref`, `held`) | 1481 | 15.7 | 6% | 0.28 | 0.21 | 0.34 | 0.07 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.48 |
| `docs/physics/staging.md` (`ref`, `held`) | 7201 | 14.3 | 0% | 0.46 | 0.24 | 0.00 | 0.20 | 0.00 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.06 | 0.10 |
| `docs/physics/synthetic-flights.md` (`ref`, `held`) | 4068 | 14.6 | 1% | 0.45 | 0.08 | 0.03 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.25 |
| `docs/physics/turbulence.md` (`ref`, `held`) | 1292 | 11.3 | 0% | 0.31 | 0.00 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.24 |
| `docs/physics/wind.md` (`ref`, `held`) | 2989 | 14.6 | 0% | 0.37 | 0.07 | 0.00 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.31 | 0.27 |
| `docs/pick-a-motor.md` (`held`) | 1245 | 13.7 | 3% | 0.00 | 0.00 | 0.17 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 1.37 | 0.41 |
| `docs/plan.md` (`held`) | 1064 | 10.4 | 0% | 0.00 | 0.19 | 0.00 | 0.10 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.57 |
| `docs/python.md` (`held`) | 4701 | 14.3 | 1% | 0.07 | 0.00 | 0.05 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.45 | 0.73 |
| `docs/reading-a-flight-log.md` (`held`) | 2441 | 13.7 | 0% | 0.46 | 0.00 | 0.00 | 0.13 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.33 | 0.78 |
| `docs/recording-a-trajectory.md` (`held`) | 821 | 14.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.00 | 0.74 |
| `docs/releasing.md` (`held`) | 2595 | 14.2 | 0% | 0.27 | 0.00 | 0.00 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.35 | 0.74 |
| `docs/sensitivity.md` (`held`) | 2268 | 14.8 | 1% | 0.27 | 0.00 | 0.05 | 0.14 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.58 | 0.67 |
| `docs/simulator-compared.md` (`held`) | 2588 | 12.8 | 4% | 0.00 | 0.00 | 0.28 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.04 | 0.00 | 0.16 | 0.70 |
| `docs/simulator-tour.md` (`held`) | 1573 | 13.9 | 0% | 0.45 | 0.00 | 0.00 | 0.07 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.32 | 0.51 |
| `docs/soundings.md` (`held`) | 5018 | 14.3 | 1% | 0.18 | 0.00 | 0.02 | 0.08 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.02 | 0.88 |
| `docs/stability-for-certification.md` (`held`) | 1990 | 14.5 | 1% | 0.00 | 0.00 | 0.06 | 0.26 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.66 | 0.46 |
| `docs/stability-in-flight.md` (`held`) | 1838 | 15.3 | 3% | 0.22 | 0.00 | 0.17 | 0.11 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.00 | 0.98 |
| `docs/start-here.md` (`held`) | 7876 | 16.0 | 6% | 0.21 | 0.04 | 0.35 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.07 | 0.00 | 0.40 | 0.67 |
| `docs/the-builder.md` (`held`) | 4201 | 12.9 | 1% | 0.20 | 0.00 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.05 | 0.00 | 0.46 | 0.58 |
| `docs/the-simulator.md` (`held`) | 797 | 15.1 | 2% | 0.00 | 0.00 | 0.13 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.13 | 0.00 | 0.88 | 1.13 |
| `docs/trusting-a-simulation.md` (`held`) | 1555 | 15.2 | 2% | 0.00 | 0.00 | 0.13 | 0.20 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.07 | 1.03 |
| `docs/weather.md` (`held`) | 1968 | 13.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.31 | 0.82 |
| `docs/when-fins-flutter.md` (`held`) | 1763 | 14.8 | 1% | 0.23 | 0.00 | 0.06 | 0.12 | 0.00 | 0.00 | 0.00 | | 0.06 | 0.00 | 0.18 | 0.63 |
| `docs/writing.md` (`held`) | 11284 | 14.9 | 4% | 0.47 | 0.19 | 0.24 | 0.16 | 0.00 | 0.02 | 0.00 | | 0.02 | 0.01 | 0.09 | 0.94 |
| `docs/your-own-logger.md` (`held`) | 2407 | 14.6 | 1% | 0.00 | 0.00 | 0.05 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.42 | 0.67 |
| `docs/your-own-rocket.md` (`held`) | 4484 | 14.6 | 0% | 0.36 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | | 0.00 | 0.00 | 0.38 | 0.67 |
| `crates/hpr-cli/src/` (`ref`, `held`) | 8656 | 9.8 | 1% | 0.00 | 0.00 | 0.04 | 0.16 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.17 | 0.50 |
| `crates/hpr/src/` (`ref`, `held`) | 615 | 7.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.98 |
| `crates/hpr-sim/src/` (`ref`, `held`) | 3454 | 12.7 | 1% | 0.00 | 0.00 | 0.03 | 0.53 | 0.03 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.67 |
| `crates/hpr-io/src/` (`ref`, `held`) | 5911 | 9.5 | 0% | 0.00 | 0.00 | 0.00 | 0.17 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.04 | 0.00 | 0.22 |
| `crates/hpr-motor/src/` (`ref`, `held`) | 1234 | 8.2 | 0% | 0.00 | 0.00 | 0.00 | 0.09 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.57 |
| `crates/hpr-design/src/` (`ref`, `held`) | 2533 | 9.9 | 1% | 0.00 | 0.00 | 0.04 | 0.04 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.36 |
| `crates/hpr-aero/src/` (`ref`, `held`) | 1604 | 7.7 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.07 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.44 |
| `crates/hpr-core/src/` (`ref`, `held`) | 241 | 6.1 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-format/src/` (`ref`, `held`) | 538 | 9.8 | 0% | 0.00 | 0.00 | 0.00 | 0.19 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 2.42 |
| `crates/hpr-atmos/src/` (`ref`, `held`) | 244 | 4.3 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-net/src/` (`ref`, `held`) | 967 | 7.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.32 |
| `crates/hpr-flightdata/src/` (`ref`, `held`) | 28626 | 11.7 | 1% | 0.00 | 0.00 | 0.01 | 0.12 | 0.01 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.45 |
| `crates/hpr-analysis/src/` (`ref`, `held`) | 449 | 5.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-forensics/src/` (`ref`, `held`) | 266 | 6.0 | 0% | 0.00 | 0.38 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 2.26 |
| `crates/hpr-py/src/` (`ref`, `held`) | 649 | 10.2 | 4% | 0.00 | 0.00 | 0.31 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.24 |
| `crates/hpr-aero/src/lib.rs` (`ref`, `held`) | 494 | 16.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.41 | 0.21 |
| `crates/hpr-analysis/src/lib.rs` (`ref`, `held`) | 194 | 13.0 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.52 |
| `crates/hpr-atmos/src/lib.rs` (`ref`, `held`) | 119 | 11.9 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-cli/src/lib.rs` (`ref`, `held`) | 204 | 18.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.99 |
| `crates/hpr-core/src/lib.rs` (`ref`, `held`) | 82 | 16.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-design/src/lib.rs` (`ref`, `held`) | 147 | 11.4 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-ffi/src/lib.rs` (`ref`, `held`) | 26 | 6.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-flightdata/src/lib.rs` (`ref`, `held`) | 853 | 12.6 | 2% | 0.00 | 0.00 | 0.12 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.71 |
| `crates/hpr-forensics/src/lib.rs` (`ref`, `held`) | 99 | 14.2 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.02 |
| `crates/hpr-format/src/lib.rs` (`ref`, `held`) | 328 | 15.7 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.92 |
| `crates/hpr-io/src/lib.rs` (`ref`, `held`) | 137 | 10.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.73 |
| `crates/hpr-motor/src/lib.rs` (`ref`, `held`) | 110 | 13.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-net/src/lib.rs` (`ref`, `held`) | 212 | 12.5 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.95 |
| `crates/hpr-py/src/lib.rs` (`ref`, `held`) | 367 | 16.7 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.28 |
| `crates/hpr-sim/src/lib.rs` (`ref`, `held`) | 343 | 15.6 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-validate/src/lib.rs` (`ref`, `held`) | 220 | 13.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr-wasm/src/lib.rs` (`ref`, `held`) | 23 | 5.8 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| `crates/hpr/src/lib.rs` (`ref`, `held`) | 449 | 10.7 | 0% | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.23 |

## What the commands say

Each `src/` page above is a crate's message census ([ADR-292](../../docs/decisions/0292-the-programs-words-in-four.md), the plan for the program's words): its `#[error]` texts and the other string literals in its code that read as prose, less its tests and its markup. A forwarded error passes on another's text. A literal with a placeholder is read in pieces, and the census doesn't see what fills it. `cargo xtask voice --messages` prints every line it read.

| Crate | Files | Literals read | `#[error]` texts | Forwarded | With a placeholder |
|---|---:|---:|---:|---:|---:|
| `crates/hpr-cli/src/` | 25 | 797 | 0 | 0 | 407 |
| `crates/hpr/src/` | 9 | 84 | 30 | 8 | 19 |
| `crates/hpr-sim/src/` | 21 | 230 | 16 | 9 | 50 |
| `crates/hpr-io/src/` | 31 | 580 | 45 | 2 | 333 |
| `crates/hpr-motor/src/` | 15 | 142 | 7 | 0 | 77 |
| `crates/hpr-design/src/` | 13 | 243 | 6 | 3 | 61 |
| `crates/hpr-aero/src/` | 15 | 187 | 6 | 3 | 48 |
| `crates/hpr-core/src/` | 14 | 40 | 12 | 0 | 11 |
| `crates/hpr-format/src/` | 3 | 53 | 7 | 0 | 41 |
| `crates/hpr-atmos/src/` | 8 | 57 | 7 | 0 | 4 |
| `crates/hpr-net/src/` | 14 | 120 | 52 | 10 | 95 |
| `crates/hpr-flightdata/src/` | 53 | 1991 | 10 | 2 | 1313 |
| `crates/hpr-analysis/src/` | 17 | 87 | 10 | 3 | 17 |
| `crates/hpr-forensics/src/` | 10 | 45 | 8 | 1 | 15 |
| `crates/hpr-py/src/` | 6 | 61 | 0 | 0 | 41 |
