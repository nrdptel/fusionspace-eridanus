#!/usr/bin/env bash
# The local gate every change passes before it is pushed. It prints one line per step, and for a
# failing step only the lines that say why. Every step runs even after a failure, so one run
# reports everything, with one exception: a failing fmt stops the run, since `cargo fmt --all`
# changes the code every later step would check (six full runs of 5.5 to 8 min each, 38 min in
# all, failed on fmt alone over two weeks of work to 2026-10-08). Full logs stay in
# target/gate/<step>.log for when the summary is not enough.
#
# Usage:
#   scripts/gate.sh                 # every step
#   scripts/gate.sh clippy test     # just these steps
#   GATE_TAIL=80 scripts/gate.sh    # show more of each failure (default 40 lines)
#
# Steps: fmt clippy test doc wasm validate deny site examples evidence, and pages, refs, ego and
# python when they apply.
# The test step runs the tests with cargo-nextest, each test its own process, in parallel, then the
# doctests with `cargo test --doc` (nextest doesn't run doctests); together they run what
# `cargo test` runs in about half the time (146 s against 296 to 309 s, single runs on 2026-10-04,
# ADR-149). Without
# nextest installed it falls back to `cargo test`.
# Step refs (M0.5g, ADR-144 §1g): the checks that need the private reference data under refs/, which
# CI can't run: `ork-flights --check`, `ork-flights --library --check`, `real-flights --check`,
# `ork-cli --check` (M4.5j; it reads the motor cache `ork-cli --fetch` fills),
# `fixture-flights --check` (M2.3c1, ADR-184; it reads the private collection's manifest and records)
# `ork-export-flights --check` (M3.2b; ADR-237: its records went stale unseen before it joined)
# `logs --check` (M7.1a, ADR-251: every flight-log sample read, its report reproduced)
# and `evidence --samples` (M2.10, ADR-278: each real sample the evidence register lists is there,
# its SHA-256 matches and its test reads it without skipping), and the motor catalog's recount
# (M5.7c, ADR-304: `analyze_stats.js --recount` works out which files pass from the raw
# ThrustCurve.org snapshot with ThrustCurve.org's own code, and fails unless the index agrees).
# Step validate runs `motor-catalog --check` as CI does, and with the raw snapshot under refs/ it
# also holds the committed records and files to it. It runs that and `synthetic --check` in the dev
# profile, whole, where CI runs them optimized and in halves (issue #1018): the dev profile is the
# one that writes the reports, so this is the check that what's committed is what it writes.
# It joins any run that includes `test` (the default set, or a fix's `scripts/gate.sh clippy test`)
# when refs/ is present and the branch changes a physics crate (hpr-aero, hpr-sim, hpr-motor,
# hpr-design, hpr-atmos, hpr-io), the log readers (hpr-flightdata), the command line (hpr-cli,
# which `ork-cli --check` runs) or the evidence register
# (validation/evidence.toml, whose samples `evidence --samples` checks) against origin/main, and
# says so when it can't. Name it to run it
# anyway (`scripts/gate.sh refs`), or set GATE_REFS=0 to leave it out.
# Step evidence (M2.10, ADR-278): `xtask evidence`, the evidence register held to the guide's pages,
# its schema and its report, and each row short of its wanted evidence to an open `needs-sample`
# issue, which it reads with `gh` (CI's `evidence` job runs the same).
# Step pages: the site's page checks alone (`xtask site --no-build`, about 80 s, single runs on
# 2026-10-13). It joins any run that includes `site` when the branch changes the pages (docs/, a
# Markdown file, or xtask's site checks) against origin/main, and runs right after fmt. A failing
# pages step stops the run as fmt does: the site step, last, would fail the same way after every
# other step had run (the site step failed in 21 of its 54 runs from 2026-10-12 to 10-13, about 9
# of them on page checks this step runs). Name it to run it alone (`scripts/gate.sh pages`), or set
# GATE_PAGES=0 to leave it out.
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

ALL="fmt clippy test doc wasm validate deny site examples evidence"
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
    validate) echo "$XTASK validate --check && $XTASK synthetic --check && $XTASK two-logs --check && $XTASK motor-catalog --check && $XTASK scoreboard --check --against origin/main" ;;
    deny) echo "cargo deny check" ;;
    site) echo "$XTASK site --locked" ;;
    pages) echo "$XTASK site --no-build" ;;
    examples) echo "$XTASK examples --check --locked" ;;
    docs) echo "cargo test --locked -p xtask" ;;
    evidence) echo "$XTASK evidence" ;;
    types) echo "$XTASK format --typecheck" ;;
    python) echo "scripts/python-tests.sh" ;;
    ego) echo "cargo bench --locked -p fusionspace-hpr-analysis --bench ego_hartmann6" ;;
    refs) echo "$XTASK ork-flights --check && $XTASK ork-flights --library --check && $XTASK real-flights --check && $XTASK ghost --check && $XTASK ork-cli --check && $XTASK fixture-flights --check && $XTASK ork-export-flights --check && $XTASK logs --check && $XTASK evidence --samples && node validation/oracles/thrustcurve/analyze_stats.js --recount" ;;
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
# The tree as the run found it, for the history line below: HEAD, and whether anything differed
# from it, untracked files included.
head=$(git rev-parse HEAD 2>/dev/null)
dirty=0; [ -z "$(git status --porcelain 2>/dev/null)" ] || dirty=1
start_all=$(date +%s)
for step in $STEPS; do
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
    if [ "$step" = fmt ] && [ "$STEPS" != fmt ]; then
      echo "    run \`cargo fmt --all\`, then the gate again: the other steps would check code it changes"
      break
    fi
    if [ "$step" = pages ] && [ "$STEPS" != pages ]; then
      echo "    fix the pages, then the gate again: the site step would fail the same way, last"
      break
    fi
  fi
done
total=$(( $(date +%s) - start_all ))
# One line a run in the git directory, where `cargo clean` can't reach it, so gate times and failed
# runs can be compared across weeks: UTC time, HEAD, whether the tree had changes, whether every
# default step ran, the result, the seconds, and each step's seconds (`:FAIL` marks a failed one).
history="${HPR_GATE_HISTORY:-}"
if [ -z "$history" ] && gitdir=$(git rev-parse --git-common-dir 2>/dev/null); then
  history="$gitdir/gate-history.tsv"
fi
full=1
for s in $ALL; do case " $STEPS " in *" $s "*) ;; *) full=0 ;; esac; done
if [ -n "$history" ]; then
  { printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$head" "$dirty" "$full" \
      "$([ -n "$failed" ] && echo FAIL || echo PASS)" "$total" "${times# }" >> "$history"; } 2>/dev/null || true
fi
if [ -n "$failed" ]; then
  echo "GATE FAILED in ${total}s:$failed"
  exit 1
fi
echo "GATE PASSED in ${total}s ($STEPS)"
