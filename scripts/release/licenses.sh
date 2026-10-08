#!/usr/bin/env bash
# Puts the license files a release artifact carries (ADR-114, ADR-187) into a folder: both of
# hpr-sim's licenses, THIRD-PARTY-NOTICES.md, and THIRD-PARTY-LICENSES.txt, the license texts of
# every crate compiled into the given package, written by cargo-about from the crates' own files.
#
# Needs cargo-about (scripts/release/about.toml says which version) and the crates fetched
# (`cargo fetch --locked`): it reads them offline, so the same lock gives the same text.
#
# Usage: scripts/release/licenses.sh <package's Cargo.toml> <folder>

set -euo pipefail
manifest="${1:?usage: licenses.sh <package Cargo.toml> <folder>}"
folder="${2:?usage: licenses.sh <package Cargo.toml> <folder>}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

mkdir -p "$folder"
cp "$root/LICENSE-MIT" "$root/LICENSE-APACHE" "$root/THIRD-PARTY-NOTICES.md" "$folder/"
cargo about generate --offline --locked --fail \
  --config "$root/scripts/release/about.toml" --manifest-path "$manifest" \
  --output-file "$folder/THIRD-PARTY-LICENSES.txt" "$root/scripts/release/licenses.hbs"
