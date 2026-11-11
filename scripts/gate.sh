#!/usr/bin/env bash
# The local gate every change passes before it is pushed. It prints one line per step, and for a
# failing step only the lines that say why. Every step runs even after a failure, so one run
# reports everything, with three exceptions. A failing fmt stops the run, since `cargo fmt --all`
# changes the code every later step would check (six full runs of 5.5 to 8 min each, 38 min in
# all, failed on fmt alone over two weeks of work to 2026-10-08). A failing pages step stops it
# too (see that step). And once an early step fails (clippy, test, doc, wasm, docs, deny,
# evidence or site), the slow steps after it (validate, examples, refs, ego, python, and after a
# code step the site) are skipped: the run already fails, a merge needs a full run that passes,
# and the gate runs again after the fix. From 2026-10-25 to 11-01, 16.6 of the gate's
# 94.8 hours were slow steps run after a code step had already failed, in 82 runs. Deny, evidence
# and site ran after validate until 2026-11-02. From 10-25 to 11-02, site failed 30 of the 149
# full runs that reached it and validate 1, and validate, examples, refs and python ran 10.1 hours
# in full runs that site, evidence or deny failed. Full logs stay in target/gate/<step>.log for
# when the summary is not enough.
#
# Usage:
#   scripts/gate.sh                 # every step
#   scripts/gate.sh clippy test     # just these steps
#   GATE_TAIL=80 scripts/gate.sh    # show more of each failure (default 40 lines)
#
# Steps: fmt clippy test doc wasm deny evidence site validate examples, and pages, refs, ego and
# python when they apply.
# The test step runs the tests with cargo-nextest, each test its own process, in parallel, then the
# doctests with `cargo test --doc` (nextest doesn't run doctests); together they run what
# `cargo test` runs in about half the time (146 s against 296 to 309 s, single runs on 2026-10-04,
# ADR-149). Without
# nextest installed it falls back to `cargo test`.
# Step refs (M0.5g, ADR-144 §1g): the checks that need the private reference data under refs/, which
# CI can't run: `ork-flights --check`, `ork-flights --library --check`, `real-flights --check`,
# `public-predictions --check` (M2.3d3, ADR-334: the predicted flights reproduce, on the converted
# EuRoC 2022 weather file `validation/oracles/netcdf/euroc_2022.py` writes under refs/; M2.3d5,
# ADR-337: the two Macie XL flights too, from the pinned checkout under refs/public-flights/),
# `roll-drev --check` (#763: the Basic Finner's roll damping against DREV-TM-9703's cached range fits),
# `ork-cli --check` (M4.5j; it reads the motor cache `ork-cli --fetch` fills),
# `fixture-flights --check` (M2.3c1, ADR-184; it reads the private collection's manifest and records)
# `ork-export-flights --check` (M3.2b; ADR-237: its records went stale unseen before it joined)
# `logs --check` (M7.1a, ADR-251: every flight-log sample read, its report reproduced)
# and `evidence --samples` (M2.10, ADR-278: each real sample the evidence register lists is there,
# its SHA-256 matches and its test reads it without skipping), and the motor catalog's recount
# (M5.7c, ADR-304: `analyze_stats.js --recount` works out which files pass from the raw
# ThrustCurve.org snapshot with ThrustCurve.org's own code, and fails unless the index agrees).
# Step validate runs `motor-catalog --check` as CI does, and with the raw snapshot under refs/ it
# also holds the committed records and files to it. It runs that in the dev profile, where CI runs it
# optimized. Of the flight censuses (ADR-149, amended), which run whole each night, it runs the
# synthetic flights' and two logs' slices (M0.16a1: every 21st flight of each set, held line for line
# to the committed `.slice.json` and to the census's rules for one flight), as CI does on every pull
# request, and the fit's slice (M0.16a2: every 21st fit of each run, the calibration set's among
# them, held line for line to the committed `.slice.json`, and those of sets A and B to the
# report's rules for one fit; the whole report and its shares only `fit --check` holds exactly),
# in the optimized build that writes the fit's report, as CI does in every word on every pull
# request. The dev profile writes the other reports, so this checks that what's committed is what
# each writes, byte for byte.
# The default run leaves validate out when the branch changes only prose that none of its five
# checks reads: pages under docs/, the site's theme/ and book.toml, and Markdown files at the top,
# but not README.md, docs/accuracy.md and docs/images/ (the census writes their tables and badges),
# docs/ROADMAP.md or docs/decisions/ (the scoreboard and its features table read them), nor a
# deleted or renamed file (the features table checks its pages exist). Read on 2026-11-01, the five
# checks' code reads no other prose, so such a change leaves what they compare as it was; CI runs
# validate and motor-catalog on every pull request but a change to research notes alone, and the
# censuses each night. 33 of the 96 pull requests merged from 2026-10-25 to 11-02 (UTC) changed
# only such prose. A run that leaves validate out this way still counts as a full
# run. Name it to run it anyway (`scripts/gate.sh validate`), or set GATE_VALIDATE=1.
# It joins any run that includes `test` (the default set, or a fix's `scripts/gate.sh clippy test`)
# when refs/ is present and the branch changes a physics crate (hpr-aero, hpr-sim, hpr-motor,
# hpr-design, hpr-atmos, hpr-io), the log readers (hpr-flightdata), the command line (hpr-cli,
# which `ork-cli --check` runs), the evidence register
# (validation/evidence.toml, whose samples `evidence --samples` checks) or the xtask files that
# split and guard the private collection's halves (`fixture-flights --check`) against origin/main, and
# says so when it can't. Name it to run it
# anyway (`scripts/gate.sh refs`), or set GATE_REFS=0 to leave it out.
# Step evidence (M2.10, ADR-278): `xtask evidence`, the evidence register held to the guide's pages,
# its schema and its report, and each row short of its wanted evidence to an open `needs-sample`
# issue, which it reads with `gh` (CI's `evidence` job runs the same).
# Step pages: the site's page checks alone (`xtask site --no-build`, about 80 s, single runs on
# 2026-10-13). It joins any run that includes `site` when the branch changes the pages (docs/, a
# Markdown file, or xtask's site checks) against origin/main, and runs right after fmt. A failing
# pages step stops the run as fmt does: the site step would fail the same way after the code
# steps had run (the site step failed in 21 of its 54 runs from 2026-10-12 to 10-13, about 9
# of them on page checks this step runs). Name it to run it alone (`scripts/gate.sh pages`), or set
# GATE_PAGES=0 to leave it out.
# Step site: `xtask site --locked --cache`. It lays out and measures again only the pages whose
# built HTML changed since its last passing run, and reuses that run's results for the rest; every
# page, with the canaries, when there is no usable cache or the checker, Chrome or any built file
# other than a page, the search index and api/ changed (the cache is in target/site-check-cache).
# CI's site job runs without --cache and checks every page. On 2026-11-02 (UTC) the whole step
# took 801 s with no cache, 51 s with no page changed, and 92 to 96 s with one page edited (2 of
# the 81 pages checked again).
# The default run leaves site out when the branch changes only the words of pages: Markdown files
# under docs/ or at the top, but not docs/SUMMARY.md (the sidebar), nor a deleted or renamed file
# (links to it), nor anything else (the site's theme/ and book.toml, xtask's site checks, the
# rustdoc under api/ and the reports the pages show). CI's site jobs build the site, check its
# links and check every page's layout across its grid of widths on every pull request, and branch
# protection requires them, so no clipping check is lost: only the local run of the same checks
# goes. From 2026-10-25 to 11-05 the site step took 52 of the gate's hours and failed 58 times; a
# study classed all 58 as self-made or flaky and found no layout defect CI missed (ADR-324). The
# page checks (step pages) still run on such a branch. A run that leaves site out this way still counts as a
# full run. Name it to run it anyway (`scripts/gate.sh site`), or set GATE_SITE=1.
# Step ego (M6.2d2, ADR-152): EGO's 20 runs on Hartmann's six-variable function, each held within
# 1% of the minimum in 250 evaluations (`cargo bench -p fusionspace-hpr-analysis --bench ego_hartmann6`, a
# release build; about 3 minutes on 10 cores, hours in the debug build CI tests with, so CI doesn't
# run it). It joins any run that includes `test` when the branch changes EGO or what it calls
# (crates/hpr-analysis/src/optimize/{ego,cmaes,normal,benchmark}.rs, or the bench) against
# origin/main. Name it to run it anyway (`scripts/gate.sh ego`), or set GATE_EGO=0 to leave it out.
# Extra step, not in the default set: docs (only xtask's tests, which guard ROADMAP, the lessons
# and the pages; about 40 s against 154 s for all tests), for a change to prose alone.
# Extra step, not in the default set: types (CI's `types` job: the design format's generated
# TypeScript and Python types checked with tsc and mypy, fetched by npx and uvx; about 2 s once
# fetched), for a change to the schema or the generator in xtask/src/format_types.rs.
# Extra step: python (CI's `python` job: the Python package's wheel built by maturin and its pytest
# suite run on CPython 3.10 and 3.13, fetched by uv; about 20 s once fetched, more for a fresh
# release build). It joins any run with `test` or `docs` when the branch changes what the suite
# reads: a crate (crates/, the wrapped library's messages included), Cargo.lock, the toolchain,
# anything under validation/ (cases and reports: test_calisto.py holds each Calisto metric to its
# case's bound; designs, .ork fixtures and logs the tests load), scripts/python-tests.sh, or a page
# under docs/ whose Python examples or shown outputs test_docs.py runs (docs/python.md and every
# page with a `<!-- python: example -->` block). Before it joined, CI's python job failed in 7
# of the 71 failed CI runs from 2026-10-14 to 10-28, the gate never having run it (#919).
# GATE_PYTHON=0 leaves it out.
# The commands are CI's own (.github/workflows/ci.yml), `--locked` included, so a Cargo.lock that
# needs updating fails here rather than on every CI job. `validate` is `--check`: every case,
# compared with the committed report. Regenerating the report is a separate, deliberate step
# (`cargo xtask validate`, debug build). CI's test job also sets RUSTFLAGS=-D warnings; clippy
# with -D warnings covers that here without rebuilding everything under a second set of flags.
#
# It took 435 s once on a warm build with nextest (2026-10-04), a few seconds more with refs.

set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

ALL="fmt clippy test doc wasm deny evidence site validate examples"
PHYSICS_CRATES="hpr-aero hpr-sim hpr-motor hpr-design hpr-atmos hpr-io hpr-flightdata hpr-cli"

# Whether this branch changes a physics crate or the evidence register, committed or not, against
# origin/main. Exit 2 when there is no origin/main to compare with.
touches_physics() {
  local base changed crate
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached; } 2>/dev/null)
  for crate in $PHYSICS_CRATES; do
    # A here-string, not a pipe: under pipefail, grep -q's early exit fails the pipe on a match.
    grep -q "^crates/$crate/" <<<"$changed" && return 0
  done
  # A sample registered or edited is what `evidence --samples` checks (M2.10, ADR-278).
  grep -qx "validation/evidence.toml" <<<"$changed" && return 0
  # What `ghost --check` reads the real logs with: CI holds only the invented flights.
  grep -qE "^(validation/oracles/ghost/|validation/fixtures/logs/rocketpy-mappings/|xtask/src/ghost.rs)" <<<"$changed" && return 0
  # What `fixture-flights --check` holds the held-out half and its guard with (M2.7a, ADR-332),
  # and new flights' predictions (M2.7b, ADR-333), and what it reads, writes and seals: CI has no
  # private collection to run it on.
  grep -qE "^xtask/src/(fixture_flights|collection_boost|collection_errors|predict_first).rs$" <<<"$changed" && return 0
  grep -qE "^validation/reports/(fixture-flights\.(md|json)|collection-boost\.(md|json)|predictions\.sha256|m2\.7a-flights\.sha256|held-out(-fits)?\.sha256)$" <<<"$changed" && return 0
  return 1
}

# Whether this branch changes what step validate reads (see the header): anything but the prose it
# doesn't read, or a deleted or renamed file. Exit 1 when every change is such prose, 2 when there
# is no origin/main to compare with; a branch with no change at all counts as a change.
VALIDATE_PROSE='^(docs/|theme/|book\.toml$|[^/]+\.md$)'
VALIDATE_READS='^(README\.md$|docs/accuracy\.md$|docs/images/|docs/ROADMAP\.md$|docs/decisions/)'
touches_validate() {
  local base changed status
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached
    git ls-files --others --exclude-standard; } 2>/dev/null)
  [ -n "$changed" ] || return 0
  status=$( { git diff --name-status --no-renames "$base"
    git diff --name-status --no-renames --cached; } 2>/dev/null)
  grep -q '^D' <<<"$status" && return 0
  grep -vqE "$VALIDATE_PROSE" <<<"$changed" && return 0
  grep -qE "$VALIDATE_READS" <<<"$changed" && return 0
  return 1
}

# Whether this branch changes more than the words of pages, so the default run keeps its site
# step (see the header): exit 0 to keep it, 1 when only page prose changed, 2 when there is no
# origin/main to compare with. No change at all keeps it (a run on main checks main).
SITE_PROSE='^(docs/.+|[^/]+)\.md$'
touches_layout() {
  local base changed status
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached
    git ls-files --others --exclude-standard; } 2>/dev/null)
  [ -n "$changed" ] || return 0
  status=$( { git diff --name-status --no-renames "$base"
    git diff --name-status --no-renames --cached; } 2>/dev/null)
  grep -q '^D' <<<"$status" && return 0
  grep -vqE "$SITE_PROSE" <<<"$changed" && return 0
  grep -qx 'docs/SUMMARY.md' <<<"$changed" && return 0
  return 1
}

# Whether this branch changes the site's pages, as touches_physics tells for the physics crates.
touches_pages() {
  local base changed
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  # New pages count before they are added, unlike the physics check's files.
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached
    git ls-files --others --exclude-standard; } 2>/dev/null)
  grep -qE '^docs/|\.md$|^xtask/src/site' <<<"$changed" && return 0
  return 1
}

# Whether this branch changes what the Python suite reads, as touches_physics tells for the
# physics crates.
touches_python() {
  local base changed
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached
    git ls-files --others --exclude-standard; } 2>/dev/null)
  grep -qE '^crates/|^Cargo\.lock$|^rust-toolchain\.toml$|^validation/|^scripts/python-tests\.sh$|^docs/python\.md$' \
    <<<"$changed" && return 0
  # A page under docs/ with a marked Python example, which test_docs.py runs.
  local page
  while IFS= read -r page; do
    [ -f "$page" ] && grep -qF '<!-- python: example -->' "$page" && return 0
  done < <(grep -E '^docs/.*\.md$' <<<"$changed")
  return 1
}

# Whether this branch changes EGO or what it calls, as touches_physics tells for the physics crates.
# Path prefixes: the optimizer's module and its parent (bounds, scaling), the bench, the seeded
# random streams every run draws from, and the crate's manifest.
EGO_FILES="crates/hpr-analysis/src/optimize crates/hpr-analysis/benches/ego_hartmann6.rs
crates/hpr-core/src/random.rs crates/hpr-analysis/Cargo.toml"
touches_ego() {
  local base changed file
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached; } 2>/dev/null)
  for file in $EGO_FILES; do
    grep -q "^$file" <<<"$changed" && return 0
  done
  return 1
}

# The refs/ checks join any run that tests, the default one or one naming `test` (a fix's gate,
# `scripts/gate.sh clippy test`), when a physics crate or the evidence register changed. When they
# should and can't, say why.
STEPS="${*:-$ALL}"
REFS_WHY=""
case " $STEPS " in
  *" refs "*) REFS_WHY="named" ;;
  *" test "*)
    touches_physics; physics=$?
    if [ "$physics" -eq 0 ]; then
      if [ "${GATE_REFS:-1}" = 0 ]; then
        echo "SKIP refs: a physics crate or the evidence register changed, but GATE_REFS=0"
      elif [ ! -d refs ]; then
        echo "SKIP refs: a physics crate or the evidence register changed, but there is no refs/ here (CI can't run it either)"
      else
        STEPS="$STEPS refs"; REFS_WHY="physics"
      fi
    elif [ "$physics" -eq 2 ]; then
      echo "SKIP refs: no origin/main to tell whether a physics crate changed (git fetch origin)"
    fi ;;
esac
# The page checks join any run that builds the site, when the pages changed, right after fmt.
case " $STEPS " in
  *" pages "*) ;;
  *" site "*)
    touches_pages; pages=$?
    if [ "$pages" -eq 0 ]; then
      if [ "${GATE_PAGES:-1}" = 0 ]; then
        echo "SKIP pages: the pages changed, but GATE_PAGES=0"
      else
        case " $STEPS " in
          *" fmt "*) STEPS=" $STEPS "; STEPS="${STEPS/ fmt / fmt pages }"; STEPS="${STEPS# }"; STEPS="${STEPS% }" ;;
          *) STEPS="pages $STEPS" ;;
        esac
      fi
    elif [ "$pages" -eq 2 ]; then
      echo "SKIP pages: no origin/main to tell whether the pages changed (git fetch origin)"
    fi ;;
esac
# EGO's measurement joins any run that tests, when EGO changed.
case " $STEPS " in
  *" ego "*) ;;
  *" test "*)
    touches_ego; ego=$?
    if [ "$ego" -eq 0 ]; then
      if [ "${GATE_EGO:-1}" = 0 ]; then
        echo "SKIP ego: EGO changed, but GATE_EGO=0"
      else
        STEPS="$STEPS ego"
      fi
    elif [ "$ego" -eq 2 ]; then
      echo "SKIP ego: no origin/main to tell whether EGO changed (git fetch origin)"
    fi ;;
esac
# The Python suite joins any run that tests or checks the docs, when what it reads changed.
case " $STEPS " in
  *" python "*) ;;
  *" test "*|*" docs "*)
    touches_python; python=$?
    if [ "$python" -eq 0 ]; then
      if [ "${GATE_PYTHON:-1}" = 0 ]; then
        echo "SKIP python: what the Python suite reads changed, but GATE_PYTHON=0"
      elif ! command -v uv >/dev/null 2>&1; then
        echo "SKIP python: what the Python suite reads changed, but uv isn't installed (CI runs it)"
      else
        STEPS="$STEPS python"
      fi
    elif [ "$python" -eq 2 ]; then
      echo "SKIP python: no origin/main to tell whether what it reads changed (git fetch origin)"
    fi ;;
esac
# Step validate leaves the default run when nothing it reads changed (see the header).
VALIDATE_LEFT=""
if [ -z "${*:-}" ] && [ "${GATE_VALIDATE:-0}" != 1 ]; then
  touches_validate; reads=$?
  if [ "$reads" -eq 1 ]; then
    STEPS=" $STEPS "; STEPS="${STEPS/ validate / }"; STEPS="${STEPS# }"; STEPS="${STEPS% }"
    VALIDATE_LEFT=1
    echo "SKIP validate: the branch changes only prose its checks don't read (CI still runs them; GATE_VALIDATE=1 runs them here)"
  elif [ "$reads" -eq 2 ]; then
    echo "validate stays in: no origin/main to tell whether what it reads changed (git fetch origin)"
  fi
fi
# Step site leaves the default run when the branch changes only the words of pages (see the
# header); the page checks, added above, stay, and site stays in when they don't run.
SITE_LEFT=""
if [ -z "${*:-}" ] && [ "${GATE_SITE:-0}" != 1 ]; then
  touches_layout; layout=$?
  if [ "$layout" -eq 1 ] && [[ " $STEPS " != *" pages "* ]]; then
    echo "site stays in: the branch changes only the words of pages, but the page checks aren't running (GATE_PAGES=0), and site holds them too"
  elif [ "$layout" -eq 1 ]; then
    STEPS=" $STEPS "; STEPS="${STEPS/ site / }"; STEPS="${STEPS# }"; STEPS="${STEPS% }"
    SITE_LEFT=1
    echo "SKIP site: the branch changes only the words of pages (CI checks the built site at every width; GATE_SITE=1 runs it here)"
  elif [ "$layout" -eq 2 ]; then
    echo "site stays in: no origin/main to tell whether only page prose changed (git fetch origin)"
  fi
fi
TAIL="${GATE_TAIL:-40}"
LOGDIR="target/gate"
XTASK="cargo run --locked --quiet --package xtask --"

cmd_for() {
  case "$1" in
    fmt) echo "cargo fmt --all --check" ;;
    clippy) echo "cargo clippy --workspace --all-targets --all-features --locked -- -D warnings" ;;
    # nextest, then the doctests it doesn't run; cargo test if nextest isn't installed.
    test)
      if cargo nextest --version >/dev/null 2>&1; then
        echo "cargo nextest run --workspace --all-features --locked --no-fail-fast && cargo test --workspace --all-features --locked --doc"
      else
        echo "cargo test --workspace --all-features --locked"
      fi ;;
    doc) echo "env RUSTDOCFLAGS=-D\\ warnings cargo doc --workspace --no-deps --all-features --locked" ;;
    wasm) echo "$XTASK wasm-check --locked" ;;
    validate) echo "$XTASK validate --check && $XTASK synthetic --slice && $XTASK two-logs --slice && cargo run --locked --quiet --package xtask --profile report-check -- fit --slice && $XTASK motor-catalog --check && $XTASK scoreboard --check --against origin/main" ;;
    deny) echo "cargo deny check" ;;
    site) echo "$XTASK site --locked --cache" ;;
    pages) echo "$XTASK site --no-build" ;;
    examples) echo "$XTASK examples --check --locked" ;;
    docs) echo "cargo test --locked -p xtask" ;;
    evidence) echo "$XTASK evidence" ;;
    types) echo "$XTASK format --typecheck" ;;
    python) echo "scripts/python-tests.sh" ;;
    ego) echo "cargo bench --locked -p fusionspace-hpr-analysis --bench ego_hartmann6" ;;
    refs) echo "$XTASK ork-flights --check && $XTASK roll-drev --check && $XTASK ork-flights --library --check && $XTASK real-flights --check && $XTASK public-predictions --check && $XTASK ghost --check && $XTASK ork-cli --check && $XTASK fixture-flights --check && $XTASK ork-export-flights --check && $XTASK logs --check && $XTASK evidence --samples && node validation/oracles/thrustcurve/analyze_stats.js --recount" ;;
    *) return 1 ;;
  esac
}

for step in $STEPS; do
  cmd_for "$step" >/dev/null || { echo "unknown step: $step (known: $ALL pages refs ego docs types python)" >&2; exit 64; }
done
mkdir -p "$LOGDIR"

# The lines that explain a failure, then the log's last lines. Failures come first, errors next and
# warnings last, so a wall of warnings cannot push a failing test out of the summary. A panic and a
# compiler message keep the lines after them, where an assertion's left and right values and the
# source excerpt are.
why() {
  local log="$1" hits
  hits=$( {
    { grep -nE '^test .* FAILED|^\s+FAIL \[|test result: FAILED|^Diff in |mismatch|not reproduced|does not reproduce' "$log"
      grep -nE -A5 'panicked at' "$log"; } | grep -v '^--$' | sort -n -u
    grep -nE -A5 '^error(\[|:)' "$log" | grep -v '^--$'
    grep -nE -A5 '^warning(\[|:)' "$log" | grep -v '^--$'
  } | awk '!seen[$0]++' | head -n "$TAIL")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits" | sed 's/^/    /'
    echo "    (up to $TAIL lines of $log: failures, then errors, then warnings; its last lines follow)"
  fi
  tail -n 12 "$log" | sed 's/^/    | /'
}

failed=""
times=""
# The steps whose failure skips the slow steps after it (see the header): the code steps, whose
# failure skips the site too, and the quick checks.
CODE_STEPS=" clippy test doc wasm docs "
EARLY_STEPS="$CODE_STEPS deny evidence site "
SLOW_STEPS=" validate examples refs ego python "
code_failed=""
early_failed=""
skipped=""
# The tree as the run found it, for the history line below: HEAD, and whether anything differed
# from it, untracked files included.
head=$(git rev-parse HEAD 2>/dev/null)
dirty=0; [ -z "$(git status --porcelain 2>/dev/null)" ] || dirty=1
start_all=$(date +%s)
for step in $STEPS; do
  why_skip=""
  [ -n "$early_failed" ] && [[ "$SLOW_STEPS" == *" $step "* ]] && why_skip=$early_failed
  [ -n "$code_failed" ] && [ "$step" = site ] && why_skip=$code_failed
  if [ -n "$why_skip" ]; then
    echo "SKIP $step: $why_skip failed; fix it, then run the gate again"
    skipped="$skipped $step"
    continue
  fi
  cmd=$(cmd_for "$step")
  log="$LOGDIR/$step.log"
  t0=$(date +%s)
  eval "$cmd" > "$log" 2>&1
  rc=$?
  dt=$(( $(date +%s) - t0 ))
  times="$times $step=${dt}$([ "$rc" -eq 0 ] || echo :FAIL)"
  if [ "$rc" -eq 0 ]; then
    extra=""
    if [ "$step" = test ]; then
      nextest=$(grep -oE '[0-9]+ tests run: [0-9]+ passed' "$log" | tail -1)
      extra=": ${nextest:+$nextest, }$(grep -cE '^test result: ok' "$log") suites ok"
    fi
    if [ "$step" = refs ]; then
      [ "$REFS_WHY" = physics ] && extra=": a physics crate or the evidence register changed, so the refs/ checks ran"
      [ "$REFS_WHY" = named ] && extra=": the refs/ checks"
    fi
    echo "PASS $step (${dt}s)$extra"
  else
    echo "FAIL $step (${dt}s, exit $rc): $cmd"
    why "$log"
    failed="$failed $step"
    [[ "$CODE_STEPS" == *" $step "* ]] && [ -z "$code_failed" ] && code_failed=$step
    [[ "$EARLY_STEPS" == *" $step "* ]] && [ -z "$early_failed" ] && early_failed=$step
    if [ "$step" = fmt ] && [ "$STEPS" != fmt ]; then
      echo "    run \`cargo fmt --all\`, then the gate again: the other steps would check code it changes"
      break
    fi
    if [ "$step" = pages ] && [ "$STEPS" != pages ]; then
      echo "    fix the pages, then the gate again: the site step would fail the same way"
      break
    fi
  fi
done
total=$(( $(date +%s) - start_all ))
# One line a run in the git directory, where `cargo clean` can't reach it, so gate times and failed
# runs can be compared across weeks: UTC time, HEAD, whether the tree had changes, whether every
# default step ran, the result, the seconds, and each step's seconds (`:FAIL` marks a failed one),
# then `validate-left` and `site-left` when the prose rules left validate or site out, so a check
# of the history can tell such a full run from one that ran them.
history="${HPR_GATE_HISTORY:-}"
if [ -z "$history" ] && gitdir=$(git rev-parse --git-common-dir 2>/dev/null); then
  history="$gitdir/gate-history.tsv"
fi
[ -n "$VALIDATE_LEFT" ] && times="$times validate-left"
[ -n "$SITE_LEFT" ] && times="$times site-left"
full=1
for s in $ALL; do
  case " $STEPS " in
    *" $s "*) ;;
    *) { [ "$s" = validate ] && [ -n "$VALIDATE_LEFT" ]; } || { [ "$s" = site ] && [ -n "$SITE_LEFT" ]; } || full=0 ;;
  esac
done
if [ -n "$history" ]; then
  { printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$head" "$dirty" "$full" \
      "$([ -n "$failed" ] && echo FAIL || echo PASS)" "$total" "${times# }" >> "$history"; } 2>/dev/null || true
fi
if [ -n "$failed" ]; then
  echo "GATE FAILED in ${total}s:$failed${skipped:+ (skipped:$skipped)}"
  exit 1
fi
echo "GATE PASSED in ${total}s ($STEPS)"
