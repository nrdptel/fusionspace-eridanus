#!/usr/bin/env bash
# Builds the `hpr` command line in release mode and packs it, with the README and the license files
# (scripts/release/licenses.sh), as fusionspace-hpr-<version>-<target> (ADR-199 §4): a .zip on
# Windows, a .tar.gz elsewhere. Prints the archive's path. The release workflow runs it on each
# operating system.
#
# On Linux the program is linked for the glibc floor in scripts/release/linux.sh (2.17), through
# cargo-zigbuild and Zig, which uv fetches at their pinned versions; elsewhere cargo builds it.
# On Windows the C runtime is linked in (`+crt-static`), so the program runs without the Visual C++
# Redistributable; scripts/release/smoke-cli.sh checks its imports.
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
  *-windows-msvc)
    # Release 0.1.0's hpr.exe imported VCRUNTIME140.dll, which a clean Windows may lack. Set as
    # `target.<triple>.rustflags` for the Windows target alone, so other systems' builds don't
    # change; with no --target, cargo also applies it to this build's build scripts and
    # proc-macros, which run on the same machine. RUSTFLAGS or CARGO_ENCODED_RUSTFLAGS in the
    # environment, even empty, makes cargo ignore it, so either one stops the build instead.
    if [ -n "${RUSTFLAGS+x}${CARGO_ENCODED_RUSTFLAGS+x}" ]; then
      echo "archive.sh: RUSTFLAGS would drop the static C runtime on $target; unset it" >&2
      exit 1
    fi
    triple="$(printf '%s' "$target" | tr 'a-z-' 'A-Z_')"
    export "CARGO_TARGET_${triple}_RUSTFLAGS=-C target-feature=+crt-static"
    ;;
esac

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
