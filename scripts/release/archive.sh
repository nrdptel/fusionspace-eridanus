#!/usr/bin/env bash
# Builds the `hpr` command line in release mode and packs it, with the README and the license files
# (scripts/release/licenses.sh), as fusionspace-hpr-<version>-<target> (ADR-199 §4): a .zip on
# Windows, a .tar.gz elsewhere. Prints the archive's path. The release workflow runs it on each
# operating system.
#
# On Linux the program is linked for the glibc floor in scripts/release/linux.sh (2.17), through
# cargo-zigbuild and Zig, which uv fetches at their pinned versions; elsewhere cargo builds it.
#
# Usage: scripts/release/archive.sh <output folder>

set -euo pipefail
out="${1:?usage: archive.sh <output folder>}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

# `cargo pkgid` ends in `#<version>` or `#<name>@<version>`.
version="$(cargo pkgid --manifest-path crates/hpr-cli/Cargo.toml | sed 's/.*[#@]//')"
target="$(rustc -vV | sed -n 's/^host: //p')"
name="fusionspace-hpr-$version-$target"
exe=""
case "$target" in *windows*) exe=".exe" ;; esac

case "$target" in
  *-linux-gnu)
    # shellcheck source=scripts/release/linux.sh
    . scripts/release/linux.sh
    uvx --from "cargo-zigbuild==$CARGO_ZIGBUILD" --with "ziglang==$ZIGLANG" \
      cargo-zigbuild zigbuild --release --locked --package fusionspace-hpr-cli \
      --target "$target.$GLIBC_FLOOR" >&2
    built="target/$target/release/hpr"
    ;;
  *)
    cargo build --release --locked --package fusionspace-hpr-cli
    built="target/release/hpr$exe"
    ;;
esac
mkdir -p "$out"
out="$(cd "$out" && pwd)"
rm -rf "${out:?}/$name"
mkdir "$out/$name"
cp "$built" README.md "$out/$name/"
scripts/release/licenses.sh crates/hpr-cli/Cargo.toml "$out/$name" >&2
cd "$out"
if [ -n "$exe" ]; then
  rm -f "$name.zip"
  7z a -tzip "$name.zip" "$name" >&2
  echo "$out/$name.zip"
else
  rm -f "$name.tar.gz"
  tar -czf "$name.tar.gz" "$name"
  echo "$out/$name.tar.gz"
fi
