#!/usr/bin/env bash
# Checks the Homebrew formula scripts/release/package_managers.py wrote, as Homebrew takes it:
# `brew style` and `brew audit` on the formula as the release publishes it, then `brew install`
# and `brew test` of a copy whose URLs name this run's own archives (file://), since the release
# the formula points at doesn't exist yet. The sha256 lines stay as written, so the install also
# checks them against the archives. The release workflow runs it on macOS and Linux (issue #434).
#
# The throwaway tap and the install are removed on the way out.
#
# Usage: scripts/release/smoke-homebrew.sh <formula> <archives folder>

set -euo pipefail
usage="usage: smoke-homebrew.sh <formula> <archives folder>"
formula="${1:?$usage}"
archives="$(cd "${2:?$usage}" && pwd)"
export HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_ENV_HINTS=1

tap="hprcheck/local"
name="$tap/fusionspace-hpr"
brew tap-new --no-git "$tap" >/dev/null
trap 'brew uninstall --formula "$name" >/dev/null 2>&1 || true; brew untap "$tap" >/dev/null 2>&1 || true' EXIT
dir="$(brew --repository "$tap")/Formula"

cp "$formula" "$dir/fusionspace-hpr.rb"
brew style "$name"
brew audit --strict --formula "$name"

release="https://github.com/nrdptel/fusionspace-eridanus/releases/download/v[^/]+/"
sed -E "s#$release#file://$archives/#" "$formula" > "$dir/fusionspace-hpr.rb"
if grep -q "https://github.com/" "$dir/fusionspace-hpr.rb"; then
  echo "smoke-homebrew: a URL in the formula isn't a release download" >&2
  exit 1
fi
# Naming the formula in full is what tells Homebrew to trust it (Homebrew's tap trust).
brew install --formula "$name"
brew test "$name"
"$(brew --prefix)/bin/hpr" --version
