#!/usr/bin/env bash
# Unpacks a command-line archive made by scripts/release/archive.sh into an empty folder and checks
# it as a user would get it: the license files are there, `hpr --version` names FusionSpace HPR and
# the workspace's version, `hpr motors show` reads the bundled catalog, and `hpr sim` flies a
# committed design to the apogee docs/cli.md prints for it, within a meter.
#
# The version is read with cargo, or from HPR_RELEASE_VERSION where there is no cargo, as in the
# old-glibc image scripts/release/smoke-manylinux.sh runs it in.
#
# Usage: scripts/release/smoke-cli.sh <archive>

set -euo pipefail
archive="$(cd "$(dirname "${1:?usage: smoke-cli.sh <archive>}")" && pwd)/$(basename "$1")"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"
version="${HPR_RELEASE_VERSION:-}"
[ -n "$version" ] ||
  version="$(cargo pkgid --manifest-path crates/hpr-cli/Cargo.toml | sed 's/.*[#@]//')"

unpacked="$(mktemp -d)"
case "$archive" in
  *.zip) 7z x -o"$unpacked" "$archive" >/dev/null ;;
  *.tar.gz) tar -xzf "$archive" -C "$unpacked" ;;
  *) echo "smoke-cli: not a .zip or .tar.gz: $archive" >&2; exit 1 ;;
esac
dir="$unpacked/$(basename "$archive" | sed -e 's/\.zip$//' -e 's/\.tar\.gz$//')"
hpr="$dir/hpr"
[ -f "$hpr.exe" ] && hpr="$hpr.exe"

for file in "$hpr" README.md LICENSE-MIT LICENSE-APACHE THIRD-PARTY-NOTICES.md \
  THIRD-PARTY-LICENSES.txt; do
  case "$file" in /*|?:*) path="$file" ;; *) path="$dir/$file" ;; esac
  if [ ! -s "$path" ]; then
    echo "smoke-cli: the archive has no $file" >&2
    exit 1
  fi
done
# The texts name the crates the binary links, clap among them.
grep -q '^- clap ' "$dir/THIRD-PARTY-LICENSES.txt" ||
  { echo "smoke-cli: THIRD-PARTY-LICENSES.txt doesn't list clap" >&2; exit 1; }

# Git Bash on Windows has no /dev/stderr, so the line is kept and echoed rather than teed.
said="$("$hpr" --version)"
echo "$said"
case "$said" in
  "FusionSpace HPR $version "*) ;;
  *) echo "smoke-cli: hpr --version doesn't say FusionSpace HPR $version" >&2; exit 1 ;;
esac
"$hpr" motors show H54 >/dev/null
"$hpr" sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54 --json >"$unpacked/sim.json"
"$(command -v python3 || command -v python)" - "$unpacked/sim.json" <<'PY'
import json, sys
apogee = json.load(open(sys.argv[1]))["summary"]["apogee"]["height_above_ground_m"]
print(f"smoke-cli: pods-none.ork on an H54 reaches {apogee:.1f} m")
# docs/cli.md's example prints 846.1 m, from a build of the same commit (`cargo xtask cli`).
if abs(apogee - 846.1) > 1.0:
    sys.exit(f"smoke-cli: apogee {apogee} m, not 846.1 m within 1 m")
PY
echo "smoke-cli: $(basename "$archive") passed"
