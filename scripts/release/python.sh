#!/usr/bin/env bash
# Builds the Python package for a release: `wheel`, this operating system's abi3 wheel, or `sdist`,
# the source package. Either carries the license files (scripts/release/licenses.sh) in its
# metadata's licenses folder (PEP 639): they are put beside crates/hpr-py/pyproject.toml, which
# names them in `license-files` for this build only, and both are undone when it ends. Prints the
# file's path.
#
# Needs uv, which fetches the pinned maturin, and cargo-about. On Linux the wheel is linked by Zig
# for the glibc floor in scripts/release/linux.sh (2.17, manylinux2014), and maturin refuses it if
# it needs a newer glibc.
#
# Usage: scripts/release/python.sh wheel|sdist <output folder>

set -euo pipefail
kind="${1:?usage: python.sh wheel|sdist <output folder>}"
out="${2:?usage: python.sh wheel|sdist <output folder>}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

# scripts/python-tests.sh builds CI's test wheels with the same maturin.
MATURIN="$(sed -n 's/^MATURIN="\(.*\)"$/\1/p' scripts/python-tests.sh)"
package="crates/hpr-py"
licenses='["LICENSE-MIT", "LICENSE-APACHE", "THIRD-PARTY-NOTICES.md", "THIRD-PARTY-LICENSES.txt"]'

# The staged files and the added line are this build's only: the checkout gets its own back.
cp "$package/pyproject.toml" "$package/pyproject.toml.checkout"
restore() {
  mv -f "$package/pyproject.toml.checkout" "$package/pyproject.toml"
  rm -f "$package"/LICENSE-MIT "$package"/LICENSE-APACHE "$package"/THIRD-PARTY-NOTICES.md \
    "$package"/THIRD-PARTY-LICENSES.txt
}
trap restore EXIT
scripts/release/licenses.sh "$package/Cargo.toml" "$package" >&2
# The pyproject.toml in the checkout names no license files, so a test build needs none; this one
# names the four just staged, after the line with its license expression.
if ! grep -q '^license-files' "$package/pyproject.toml"; then
  sed -i.orig "s/^license = \"MIT OR Apache-2.0\"\$/&\\
license-files = $licenses/" "$package/pyproject.toml"
  rm -f "$package/pyproject.toml.orig"
fi
grep -qxF "license-files = $licenses" "$package/pyproject.toml" ||
  { echo "python.sh: couldn't name the license files in $package/pyproject.toml" >&2; exit 1; }

mkdir -p "$out"
out="$(cd "$out" && pwd)"
case "$kind" in
  wheel)
    # On Linux, Zig comes from PyPI's ziglang, installed beside maturin: maturin finds it by
    # running `python3 -m ziglang`, and uvx puts its environment's python3 first on the PATH.
    # (The `+` form keeps an empty list safe under bash 3's `set -u`.)
    with=()
    zig=()
    case "$(rustc -vV | sed -n 's/^host: //p')" in
      *-linux-gnu)
        # shellcheck source=scripts/release/linux.sh
        . scripts/release/linux.sh
        with=(--with "ziglang==$ZIGLANG")
        zig=(--zig --compatibility "$MANYLINUX")
        ;;
    esac
    uvx --from "maturin==$MATURIN" ${with[@]+"${with[@]}"} maturin build ${zig[@]+"${zig[@]}"} \
      --release --locked \
      --manifest-path "$package/Cargo.toml" --out "$out" >&2
    pattern="*.whl"
    ;;
  sdist)
    uvx --from "maturin==$MATURIN" maturin sdist --manifest-path "$package/Cargo.toml" \
      --out "$out" >&2
    pattern="*.tar.gz"
    ;;
  *) echo "python.sh: not wheel or sdist: $kind" >&2; exit 1 ;;
esac
shopt -s nullglob
built=("$out"/$pattern)
if [ "${#built[@]}" -ne 1 ]; then
  echo "python.sh: expected one $pattern in $out, found ${#built[@]}" >&2
  exit 1
fi
echo "${built[0]}"
