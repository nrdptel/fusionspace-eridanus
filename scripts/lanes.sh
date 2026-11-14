#!/usr/bin/env bash
# Run each argument as a shell command, all at once, and wait for them all.
#
#   scripts/lanes.sh "<command>" "<command>" ...
#
# Each command's output goes to a file of its own under $LANES_DIR (default target/gate/lanes),
# `<n>.log`, and is printed whole once every command has ended, in the order the commands were
# given, under a line `== lane <n> (PASS|FAIL exit <rc>, <s>s): <command>`. So a log reads the
# same from run to run, however the commands interleave. Exits 1 when any command failed, and
# ends with a line `error: lane <n> failed (exit <rc>, <its log>): <command>` for each; 0
# otherwise. A command that is killed counts as failed.
#
# For `scripts/gate.sh`'s refs step, whose checks each read their own files and write none:
# side by side they take as long as the longest, not their sum.
set -u
[ "$#" -gt 0 ] || { echo "usage: scripts/lanes.sh \"<command>\" ..." >&2; exit 64; }
dir="${LANES_DIR:-target/gate/lanes}"
rm -rf "$dir"; mkdir -p "$dir" || exit 1
pids=()
n=0
for cmd in "$@"; do
  n=$((n + 1))
  (
    t0=$(date +%s)
    bash -c "$cmd" > "$dir/$n.log" 2>&1
    rc=$?
    echo "$rc $(( $(date +%s) - t0 ))" > "$dir/$n.rc"
  ) &
  pids+=("$!")
done
for pid in "${pids[@]}"; do wait "$pid"; done
failed=0
summary=""
n=0
for cmd in "$@"; do
  n=$((n + 1))
  # No result file: the lane's own shell was killed before it could write one.
  if [ -f "$dir/$n.rc" ]; then read -r rc dt < "$dir/$n.rc"; else rc=137; dt="?"; fi
  if [ "$rc" -eq 0 ]; then
    verdict="PASS"
  else
    verdict="FAIL exit $rc"
    failed=$((failed + 1))
    summary="${summary}error: lane $n failed (exit $rc, $dir/$n.log): $cmd"$'\n'
  fi
  echo "== lane $n ($verdict, ${dt}s): $cmd"
  cat "$dir/$n.log" 2>/dev/null
done
# The failed lanes again, last, where a reader of the log's tail finds them.
if [ "$failed" -gt 0 ]; then
  printf '%s' "$summary"
  exit 1
fi
