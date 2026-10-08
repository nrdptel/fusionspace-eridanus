#!/usr/bin/env bash
# Installs a wheel or source package made by scripts/release/python.sh into a fresh environment on
# the oldest and newest Python it serves, and runs scripts/release/smoke_python.py there: the
# license files, the version, and the Python page's example flight. A source package is compiled
# by pip's build, so this needs Rust as well as uv.
#
# Usage: scripts/release/smoke-python.sh <wheel or .tar.gz>

set -euo pipefail
file="$(cd "$(dirname "${1:?usage: smoke-python.sh <wheel or sdist>}")" && pwd)/$(basename "$1")"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"
version="$(cargo pkgid --manifest-path crates/hpr-py/Cargo.toml | sed 's/.*[#@]//')"

for python in 3.10 3.13; do
  echo "smoke-python: $(basename "$file") on Python $python"
  uv run --isolated --no-project --python "$python" --with "$file" -- \
    python scripts/release/smoke_python.py "$version"
done
echo "smoke-python: $(basename "$file") passed"
