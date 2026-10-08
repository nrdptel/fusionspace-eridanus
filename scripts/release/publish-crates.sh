#!/usr/bin/env bash
# Publishes the workspace's crates to crates.io one at a time, in dependency order, paced to
# crates.io's limit on new crate names, and so that a run that stops can be run again (issue #372).
#
# - A crate whose version crates.io's index already lists is skipped, so a second run carries on
#   where the first stopped; so is one that cargo or crates.io says is already there.
# - crates.io lets one account publish a burst of 5 new crate names, then one every 10 minutes
#   (https://crates.io/docs/rate-limits, read 2026-10-08; its source, src/rate_limiter.rs, sets
#   the same). After this run's fifth new name, it waits 10.5 minutes after each new name before
#   the next. New versions of existing crates have a burst of 30, more than the 14 crates, so they
#   go up without waiting.
# - If crates.io still answers that the limit is reached, it waits until the time crates.io names
#   and tries again, up to three times a crate.
#
# `--dry-run` publishes nothing and waits for nothing: it checks the index lookups this script
# relies on against crates.io, then prints what a real run would do and how long it would wait.
# The release workflow runs it on every run; `cargo publish --workspace --dry-run` checks that each
# crate packages and builds.
#
# Needs curl and python3; publishing needs CARGO_REGISTRY_TOKEN.
#
# Usage: scripts/release/publish-crates.sh [--dry-run]

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

dry_run=no
case "${1:-}" in
  "") ;;
  --dry-run) dry_run=yes ;;
  *) echo "usage: publish-crates.sh [--dry-run]" >&2; exit 2 ;;
esac

# Each crate after every workspace crate it depends on, dev-dependencies included, since
# crates.io checks those exist too. `cargo test -p xtask` holds this list to `cargo metadata`:
# every crate that is published, each after its dependencies.
# Package names, the FusionSpace ones (ADR-199 §2); the folders under crates/ keep the short names.
CRATES=(
  fusionspace-hpr-core fusionspace-hpr-atmos fusionspace-hpr-motor fusionspace-hpr-design
  fusionspace-hpr-aero fusionspace-hpr-sim fusionspace-hpr-io fusionspace-hpr-format
  fusionspace-hpr-net fusionspace-hpr-flightdata fusionspace-hpr-analysis
  fusionspace-hpr-forensics fusionspace-hpr fusionspace-hpr-cli
)
INDEX="https://index.crates.io"
# The crates.io account that must own any name already there.
OWNER="nrdptel"
NEW_BURST=5
NEW_INTERVAL_S=630
# A wait longer than this for crates.io's limit means something else is wrong.
MOST_WAIT_S=4500
# The User-Agent every FusionSpace HPR program sends (ADR-199 §4), with the workspace's version.
AGENT="fusionspace-hpr/$(cargo pkgid --manifest-path crates/hpr-cli/Cargo.toml | sed 's/.*[#@]//') (+https://hpr.fusionspace.co)"

# The crate's file in the sparse index, by the layout cargo's registry-index reference gives
# (https://doc.rust-lang.org/cargo/reference/registry-index.html): 1/<name>, 2/<name>,
# 3/<first letter>/<name>, or <first two letters>/<next two>/<name>, in lower case.
index_path() {
  local name
  name="$(printf '%s' "$1" | tr '[:upper:]' '[:lower:]')"
  case "${#name}" in
    1) echo "1/$name" ;;
    2) echo "2/$name" ;;
    3) echo "3/${name:0:1}/$name" ;;
    *) echo "${name:0:2}/${name:2:2}/$name" ;;
  esac
}

# `published`, `new-version` (the name is on crates.io, this version isn't) or `new-name`.
status() {
  local name="$1" version="$2" body code
  body="$(mktemp)"
  code="$(curl -sS --retry 3 -A "$AGENT" -o "$body" -w '%{http_code}' \
    "$INDEX/$(index_path "$name")")"
  case "$code" in
    200)
      python3 -c '
import json, sys
version = sys.argv[1]
lines = [json.loads(line) for line in open(sys.argv[2], encoding="utf-8") if line.strip()]
print("published" if any(entry["vers"] == version for entry in lines) else "new-version")
' "$version" "$body" || {
        echo "publish-crates: the index's answer for $name doesn't read" >&2
        rm -f "$body"
        return 1
      }
      ;;
    404) echo new-name ;;
    *)
      echo "publish-crates: the index answered $code for $name" >&2
      rm -f "$body"
      return 1
      ;;
  esac
  rm -f "$body"
}

# The Unix time in crates.io's "Please try again after <HTTP date>" in a log, or nothing.
retry_after() {
  python3 -c '
import email.utils, re, sys
found = re.search(r"try again after (.+? GMT)", open(sys.argv[1], encoding="utf-8").read())
if found:
    print(int(email.utils.parsedate_to_datetime(found.group(1)).timestamp()))
' "$1"
}

utc() {
  python3 -c '
import sys, time
print(time.strftime("%H:%M:%S UTC", time.gmtime(int(sys.argv[1]))))
' "$1"
}

# Whether crates.io lists an account among a crate's owners: 0 if it does, 1 if it doesn't, 2 if
# the lookup failed (its answer's code on stderr).
owns() {
  local name="$1" owner="$2" body code result=0
  body="$(mktemp)"
  code="$(curl -sS --retry 3 -A "$AGENT" -o "$body" -w '%{http_code}' \
    "https://crates.io/api/v1/crates/$name/owners")" || code=000
  if [ "$code" != 200 ]; then
    echo "publish-crates: the owners lookup for $name answered $code" >&2
    result=2
  else
    python3 -c '
import json, sys
users = json.load(open(sys.argv[1], encoding="utf-8")).get("users", [])
sys.exit(0 if any(user.get("login") == sys.argv[2] for user in users) else 1)
' "$body" "$owner" || result=$?
    # A body that doesn't read as JSON is a failed lookup, not someone else's crate.
    [ "$result" -le 1 ] || result=2
  fi
  rm -f "$body"
  return "$result"
}

# Stops the run unless `$OWNER` owns a crate already on crates.io: a name someone else holds is
# neither ours to skip nor ours to publish a version of.
owned() {
  local name="$1" result=0
  owns "$name" "$OWNER" || result=$?
  case "$result" in
    0) ;;
    1)
      echo "publish-crates: $name is on crates.io, but not owned by $OWNER; stopping" >&2
      exit 1
      ;;
    *)
      echo "publish-crates: couldn't check who owns $name; stopping (re-run the job)" >&2
      exit 1
      ;;
  esac
}

# By package name, which needn't be its folder's.
version_of() {
  cargo pkgid "$1" | sed 's/.*[#@]//'
}

wait_until() {
  local at="$1" why="$2" now
  now="$(date +%s)"
  if [ "$at" -gt "$now" ]; then
    echo "publish-crates: waiting $(((at - now + 59) / 60)) min, until $(utc "$at"): $why"
    sleep "$((at - now))"
  fi
}

# Waits, up to 10 minutes, for the index to list a version just published: cargo waits only a
# minute, and the next crate's build needs this one from the index.
wait_listed() {
  local name="$1" version="$2" tries=0
  until [ "$(status "$name" "$version")" = published ]; do
    tries=$((tries + 1))
    if [ "$tries" -gt 60 ]; then
      echo "publish-crates: the index doesn't list $name $version after 10 minutes; re-run the" \
        "job once it does, and it goes on from there" >&2
      exit 1
    fi
    sleep 10
  done
}

# The lookups this script depends on, against crates.io itself.
self_check() {
  local want got
  for want in "a 1/a" "cc 2/cc" "hpr 3/h/hpr" "serde se/rd/serde" "HPR-Core hp/r-/hpr-core"; do
    got="$(index_path "${want%% *}")"
    [ "$got" = "${want#* }" ] ||
      { echo "publish-crates: index_path ${want%% *} gave $got" >&2; return 1; }
  done
  for want in "serde 1.0.0 published" "serde 0.0.1-no-such-version new-version" \
    "hpr-no-such-crate-issue-372 0.1.0 new-name"; do
    read -r name version expected <<<"$want"
    got="$(status "$name" "$version")"
    [ "$got" = "$expected" ] ||
      { echo "publish-crates: $name $version read as $got, not $expected" >&2; return 1; }
  done
  local result
  for want in "serde dtolnay 0" "serde $OWNER 1"; do
    read -r name owner expected <<<"$want"
    result=0
    owns "$name" "$owner" || result=$?
    [ "$result" = "$expected" ] ||
      { echo "publish-crates: owns $name $owner gave $result, not $expected" >&2; return 1; }
  done
  local log
  log="$(mktemp)"
  echo "error: the remote server responded with an error (status 429 Too Many Requests): You" \
    "have published too many new crates in a short period of time. Please try again after" \
    "Thu, 08 Oct 2026 14:20:00 GMT and see https://crates.io/docs/rate-limits for more" \
    "details." >"$log"
  got="$(retry_after "$log")"
  rm -f "$log"
  [ "$got" = 1791469200 ] ||
    { echo "publish-crates: crates.io's retry time read as '$got'" >&2; return 1; }
  echo "publish-crates: the index and owners lookups and the retry time read as expected"
}

if [ "$dry_run" = yes ]; then
  self_check
fi

new_names=0
last_new=0
waits=0
for name in "${CRATES[@]}"; do
  version="$(version_of "$name")"
  state="$(status "$name" "$version")"
  if [ "$state" != new-name ]; then
    owned "$name"
  fi
  if [ "$state" = published ]; then
    echo "publish-crates: $name $version is on crates.io already; skipped"
    continue
  fi
  if [ "$state" = new-name ]; then
    if [ "$new_names" -ge "$NEW_BURST" ]; then
      waits=$((waits + 1))
      [ "$dry_run" = yes ] ||
        wait_until "$((last_new + NEW_INTERVAL_S))" "crates.io takes one new name every 10 min"
    fi
  fi
  if [ "$dry_run" = yes ]; then
    case "$state" in
      new-name) echo "publish-crates: would publish $name $version, a new name" ;;
      *) echo "publish-crates: would publish $name $version, a new version of a crate of ours" ;;
    esac
  else
    log="$(mktemp)"
    for attempt in 1 2 3; do
      if cargo publish --locked --package "$name" 2>&1 | tee "$log"; then
        wait_listed "$name" "$version"
        break
      fi
      if grep -qE 'already exists on|is already uploaded' "$log"; then
        echo "publish-crates: $name $version was on crates.io already"
        break
      fi
      retry_at="$(retry_after "$log")"
      now="$(date +%s)"
      if [ -n "$retry_at" ] && [ "$attempt" -lt 3 ] &&
        [ "$((retry_at - now))" -le "$MOST_WAIT_S" ]; then
        wait_until "$((retry_at + 15))" "crates.io's rate limit"
        continue
      fi
      echo "publish-crates: $name $version wasn't published. What is published stays; once" \
        "the cause is fixed, run this again and it carries on from $name." >&2
      exit 1
    done
    rm -f "$log"
  fi
  if [ "$state" = new-name ]; then
    new_names=$((new_names + 1))
    last_new="$(date +%s)"
  fi
done

if [ "$dry_run" = yes ]; then
  echo "publish-crates: $new_names new names; a real run waits about" \
    "$(((waits * NEW_INTERVAL_S + 30) / 60)) min between them, besides building each crate"
fi
echo "publish-crates: done"
