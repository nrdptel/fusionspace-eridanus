#!/usr/bin/env bash
# Run each argument as a shell command, all at once, and wait for them all.
#
#   scripts/lanes.sh "<command>" "<command>" ...
#
# Each command's output goes to a file of its own, `<n>.log`, in a folder made for this run under
# $LANES_DIR (default target/gate/lanes), and is printed whole once every command has ended, in the
# order the commands were given, under a line `== lane <n> (PASS|FAIL exit <rc>, <s>s): <command>`.
# So a log reads the same from run to run, however the commands interleave. Exits 1 when any
# command failed, and ends with a line `error: lane <n> failed (exit <rc>, <its log>): <command>`
# for each; 0 otherwise. A command that is killed counts as failed.
#
# A run never reads another run's results. Its folder is new and named for its process, so a lane
# left over from a run that was stopped can't write a result where this run looks for one. The
# folders of runs that have ended are removed at the start. Stopping a run (Ctrl-C, or a TERM
# signal) stops its lanes, each with everything it started.
#
# A lane is waited for until its own shell ends: a command that puts work in the background and
# returns counts as ended, with whatever that work would have printed or failed on left out.
#
# For `scripts/gate.sh`'s refs step, whose checks read their own files and write only folders no
# other check touches: side by side they take as long as the longest, not their sum.
set -u
[ "$#" -gt 0 ] || { echo "usage: scripts/lanes.sh \"<command>\" ..." >&2; exit 64; }
base="${LANES_DIR:-target/gate/lanes}"
mkdir -p "$base" || exit 1
# The folders of earlier runs, `run.<pid>.<random>`: gone unless that run is still going.
for old in "$base"/run.*; do
  [ -d "$old" ] || continue
  pid=${old##*/run.}; pid=${pid%%.*}
  kill -0 "$pid" 2>/dev/null || rm -rf "$old"
done
dir=$(mktemp -d "$base/run.$$.XXXXXX") || exit 1
# Each lane in a process group of its own, so that stopping the run can stop all a lane started.
set -m
pids=()
stop() {
  trap - INT TERM
  for pid in "${pids[@]}"; do kill -TERM -- "-$pid" 2>/dev/null; done
  echo "error: lanes stopped by a signal; no lane's result counts" >&2
  exit 130
}
trap stop INT TERM
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
# `wait` returns early when a trapped signal arrives; the trap then ends the run.
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
  # Output that ends without a newline would run into the next line, and hide an `error:` there.
  [ -s "$dir/$n.log" ] && [ -n "$(tail -c 1 "$dir/$n.log")" ] && echo
done
# The failed lanes again, last, where a reader of the log's tail finds them.
if [ "$failed" -gt 0 ]; then
  printf '%s' "$summary"
  exit 1
fi
