#!/usr/bin/env bash
# Checks that a Linux release file asks for no newer glibc than the floor in
# scripts/release/linux.sh (2.17): the highest `GLIBC_` symbol version in the dynamic symbol table
# of each program and library in it, read with objdump, must be at most the floor. A wheel's file
# name must also carry the floor's manylinux tag, which is what lets pip on that glibc pick it.
# The release workflow runs it on the Linux archive and wheel; smoke-manylinux.sh then runs them
# on that glibc.
#
# Usage: scripts/release/check-glibc.sh <fusionspace-hpr-<version>-<target>.tar.gz | <wheel>.whl>

set -euo pipefail
file="${1:?usage: check-glibc.sh <archive or wheel>}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=scripts/release/linux.sh
. "$root/scripts/release/linux.sh"

unpacked="$(mktemp -d)"
trap 'rm -rf "$unpacked"' EXIT
case "$file" in
  *.whl)
    # The platform tags are the name's last field, before `.whl`, joined by dots.
    platform="$(basename "$file" .whl)"
    platform="${platform##*-}"
    case ".$platform." in
      *".${MANYLINUX_TAG}_"*) ;;
      *)
        echo "check-glibc: $(basename "$file") isn't tagged $MANYLINUX_TAG (it says $platform)" >&2
        exit 1
        ;;
    esac
    python3 -m zipfile -e "$file" "$unpacked"
    ;;
  *.tar.gz) tar -xzf "$file" -C "$unpacked" ;;
  *) echo "check-glibc: not a .whl or .tar.gz: $file" >&2; exit 1 ;;
esac

checked=0
while IFS= read -r -d '' elf; do
  # An ELF file starts with 0x7f 'E' 'L' 'F'.
  [ "$(head -c 4 "$elf" | od -An -c | tr -d ' ')" = '177ELF' ] || continue
  versions="$(objdump -T "$elf" | { grep -o 'GLIBC_[0-9][0-9.]*' || true; } | sed 's/^GLIBC_//' | sort -u)"
  if [ -z "$versions" ]; then
    echo "check-glibc: objdump found no glibc symbol versions in ${elf#"$unpacked"/}" >&2
    exit 1
  fi
  # The highest version, compared part by part as numbers (2.17 is above 2.2.5).
  highest="$(printf '%s\n' "$versions" | awk -F. '
    { key = sprintf("%05d%05d%05d", $1, $2, $3) }
    key > best { best = key; version = $0 }
    END { print version }')"
  above="$(printf '%s\n%s\n' "$highest" "$GLIBC_FLOOR" | awk -F. '
    NR == 1 { a = sprintf("%05d%05d%05d", $1, $2, $3) }
    NR == 2 { b = sprintf("%05d%05d%05d", $1, $2, $3) }
    END { print (a > b) ? "yes" : "no" }')"
  if [ "$above" = yes ]; then
    echo "check-glibc: ${elf#"$unpacked"/} needs glibc $highest, above the floor $GLIBC_FLOOR" >&2
    exit 1
  fi
  echo "check-glibc: ${elf#"$unpacked"/} needs glibc $highest at most (floor $GLIBC_FLOOR)"
  checked=$((checked + 1))
done < <(find "$unpacked" -type f -print0)
if [ "$checked" -eq 0 ]; then
  echo "check-glibc: $(basename "$file") holds no program or library to check" >&2
  exit 1
fi
echo "check-glibc: $(basename "$file") passed"
