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
# Steps: fmt clippy test doc wasm validate deny site examples, and refs when it applies (below).
# The test step runs the tests with cargo-nextest, each test its own process, in parallel, then the
# doctests with `cargo test --doc` (nextest doesn't run doctests); together they run what
# `cargo test` runs in about half the time (146 s against 296 to 309 s, single runs on 2026-10-04,
# ADR-149). Without
# nextest installed it falls back to `cargo test`.
# Step refs (M0.5g, ADR-144 §1g): the checks that need the private reference data under refs/, which
# CI can't run: `ork-flights --check`, `ork-flights --library --check`, `real-flights --check`,
# `ork-cli --check` (M4.5j; it reads the motor cache `ork-cli --fetch` fills) and
# `fixture-flights --check` (M2.3c1, ADR-184; it reads the private collection's manifest and records).
# It joins any run that includes `test` (the default set, or a fix's `scripts/gate.sh clippy test`)
# when refs/ is present and the branch changes a physics crate (hpr-aero, hpr-sim, hpr-motor,
# hpr-design, hpr-atmos, hpr-io) against origin/main, and says so when it can't. Name it to run it
# anyway (`scripts/gate.sh refs`), or set GATE_REFS=0 to leave it out.
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
# Extra step, not in the default set: python (CI's `python` job: the Python package's wheel built
# by maturin and its pytest suite run on CPython 3.10 and 3.13, fetched by uv; about 20 s once
# fetched), for a change to crates/hpr-py or what it wraps.
# The commands are CI's own (.github/workflows/ci.yml), `--locked` included, so a Cargo.lock that
# needs updating fails here rather than on every CI job. `validate` is `--check`: every case,
# compared with the committed report. Regenerating the report is a separate, deliberate step
# (`cargo xtask validate`, debug build). CI's test job also sets RUSTFLAGS=-D warnings; clippy
# with -D warnings covers that here without rebuilding everything under a second set of flags.
#
# It took 435 s once on a warm build with nextest (2026-10-04), a few seconds more with refs.

set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1

ALL="fmt clippy test doc wasm validate deny site examples"
PHYSICS_CRATES="hpr-aero hpr-sim hpr-motor hpr-design hpr-atmos hpr-io"

# Whether this branch changes a physics crate, committed or not, against origin/main. Exit 2 when
# there is no origin/main to compare with.
touches_physics() {
  local base changed crate
  base=$(git merge-base HEAD origin/main 2>/dev/null) || return 2
  changed=$( { git diff --name-only "$base"; git diff --name-only; git diff --name-only --cached; } 2>/dev/null)
  for crate in $PHYSICS_CRATES; do
    # A here-string, not a pipe: under pipefail, grep -q's early exit fails the pipe on a match.
    grep -q "^crates/$crate/" <<<"$changed" && return 0
  done
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
# `scripts/gate.sh clippy test`), when a physics crate changed. When they should and can't, say why.
STEPS="${*:-$ALL}"
REFS_WHY=""
case " $STEPS " in
  *" refs "*) REFS_WHY="named" ;;
  *" test "*)
    touches_physics; physics=$?
    if [ "$physics" -eq 0 ]; then
      if [ "${GATE_REFS:-1}" = 0 ]; then
        echo "SKIP refs: a physics crate changed, but GATE_REFS=0"
      elif [ ! -d refs ]; then
        echo "SKIP refs: a physics crate changed, but there is no refs/ here (CI can't run it either)"
      else
        STEPS="$STEPS refs"; REFS_WHY="physics"
      fi
    elif [ "$physics" -eq 2 ]; then
      echo "SKIP refs: no origin/main to tell whether a physics crate changed (git fetch origin)"
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
    validate) echo "$XTASK validate --check" ;;
    deny) echo "cargo deny check" ;;
    site) echo "$XTASK site --locked" ;;
    examples) echo "$XTASK examples --check --locked" ;;
    docs) echo "cargo test --locked -p xtask" ;;
    types) echo "$XTASK format --typecheck" ;;
    python) echo "scripts/python-tests.sh" ;;
    ego) echo "cargo bench --locked -p fusionspace-hpr-analysis --bench ego_hartmann6" ;;
    refs) echo "$XTASK ork-flights --check && $XTASK ork-flights --library --check && $XTASK real-flights --check && $XTASK ork-cli --check && $XTASK fixture-flights --check" ;;
    *) return 1 ;;
  esac
}

for step in $STEPS; do
  cmd_for "$step" >/dev/null || { echo "unknown step: $step (known: $ALL refs ego docs types python)" >&2; exit 64; }
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
start_all=$(date +%s)
for step in $STEPS; do
  cmd=$(cmd_for "$step")
  log="$LOGDIR/$step.log"
  t0=$(date +%s)
  eval "$cmd" > "$log" 2>&1
  rc=$?
  dt=$(( $(date +%s) - t0 ))
  if [ "$rc" -eq 0 ]; then
    extra=""
    if [ "$step" = test ]; then
      nextest=$(grep -oE '[0-9]+ tests run: [0-9]+ passed' "$log" | tail -1)
      extra=": ${nextest:+$nextest, }$(grep -cE '^test result: ok' "$log") suites ok"
    fi
    if [ "$step" = refs ]; then
      [ "$REFS_WHY" = physics ] && extra=": a physics crate changed, so the refs/ checks ran"
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
  fi
done
total=$(( $(date +%s) - start_all ))
if [ -n "$failed" ]; then
  echo "GATE FAILED in ${total}s:$failed"
  exit 1
fi
echo "GATE PASSED in ${total}s ($STEPS)"
