#!/usr/bin/env bash
# Fails unless the Rust toolchain builds for the machine it runs on: on an ARM runner, an x86-64
# toolchain would build x86-64 programs that run under emulation, pass, and test the wrong
# platform (ADR-222). Prints the host triple. CI and the release workflow run it after
# `rustup toolchain install`; GitHub sets RUNNER_ARCH to X64 or ARM64.
#
# Usage: scripts/native-toolchain.sh

set -euo pipefail
host="$(rustc -vV | sed -n 's/^host: //p')"
echo "native-toolchain: rustc's host is $host on a ${RUNNER_ARCH:?RUNNER_ARCH is unset} runner"
case "$RUNNER_ARCH:$host" in
  X64:x86_64-* | ARM64:aarch64-*) ;;
  *)
    echo "native-toolchain: $host doesn't match the runner's $RUNNER_ARCH" >&2
    exit 1
    ;;
esac
