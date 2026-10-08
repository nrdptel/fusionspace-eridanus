#!/usr/bin/env bash
# Runs a Linux release file's smoke test on the oldest glibc it is built for: inside the pinned
# manylinux2014 image of scripts/release/linux.sh, a CentOS 7 with glibc 2.17. An archive goes
# through smoke-cli.sh; a wheel is installed by pip into fresh environments on CPython 3.10 and
# 3.13, with NumPy from wheels only, and checked by smoke_python.py. The checkout is mounted
# read-only, for the example design and the scripts. The release workflow runs it after the same
# smoke test on its own runner.
#
# Needs Docker, and cargo for the version.
#
# Usage: scripts/release/smoke-manylinux.sh <Linux archive (.tar.gz) or wheel (.whl)>

set -euo pipefail

# Inside the image: /io is the checkout, /dist holds the file.
if [ "${1:-}" = "--inside" ]; then
  file="${2:?}"
  # shellcheck source=scripts/release/linux.sh
  . /io/scripts/release/linux.sh
  said="$(getconf GNU_LIBC_VERSION)"
  echo "smoke-manylinux: the image has $said"
  if [ "$said" != "glibc $GLIBC_FLOOR" ]; then
    echo "smoke-manylinux: the image's glibc isn't the floor, $GLIBC_FLOOR" >&2
    exit 1
  fi
  # An image may install an interpreter on first use; `ensure` does, and is quiet if it's there.
  python_at() {
    [ -x "/opt/python/$1/bin/python" ] || manylinux-interpreters ensure "$1" >&2
    echo "/opt/python/$1/bin/python"
  }
  case "$file" in
    *.tar.gz)
      python="$(python_at cp312-cp312)"
      PATH="$(dirname "$python"):$PATH" bash /io/scripts/release/smoke-cli.sh "$file"
      ;;
    *.whl)
      for python in cp310-cp310 cp313-cp313; do
        echo "smoke-manylinux: $(basename "$file") on /opt/python/$python"
        interpreter="$(python_at "$python")"
        venv="$(mktemp -d)/venv"
        # The image's Pythons are built without ensurepip, so the environment is made bare and
        # filled by the interpreter's own pip, as the image's own tools are.
        "$interpreter" -m venv --without-pip "$venv"
        # Wheels only, so pip takes the newest NumPy built for this glibc instead of compiling one.
        "$interpreter" -m pip --python "$venv/bin/python" install --quiet \
          --disable-pip-version-check --only-binary=:all: "$file"
        (cd "$(dirname "$venv")" &&
          "$venv/bin/python" /io/scripts/release/smoke_python.py "$HPR_RELEASE_VERSION")
      done
      echo "smoke-manylinux: $(basename "$file") passed"
      ;;
    *) echo "smoke-manylinux: not a .tar.gz or .whl: $file" >&2; exit 1 ;;
  esac
  exit 0
fi

file="${1:?usage: smoke-manylinux.sh <archive or wheel>}"
file="$(cd "$(dirname "$file")" && pwd)/$(basename "$file")"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=scripts/release/linux.sh
. "$root/scripts/release/linux.sh"
case "$file" in
  *.whl) package=crates/hpr-py ;;
  *) package=crates/hpr-cli ;;
esac
version="$(cargo pkgid --manifest-path "$root/$package/Cargo.toml" | sed 's/.*[#@]//')"
docker run --rm --platform linux/amd64 \
  -v "$root:/io:ro" -v "$(dirname "$file"):/dist:ro" -w /io \
  -e HPR_RELEASE_VERSION="$version" \
  "$MANYLINUX_IMAGE" \
  bash /io/scripts/release/smoke-manylinux.sh --inside "/dist/$(basename "$file")"
