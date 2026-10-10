#!/usr/bin/env bash
# Builds the workspace for phones (ADR-227), so the mobile apps (M9.4) start from a core that
# already builds there. Every crate is built with every feature, so the C code of the TLS library
# under hpr-net is compiled by the phone's own toolchain, except xtask (a tool for this repository)
# and hpr-py (the Python package isn't built for phones).
#
#   scripts/phone-build.sh aarch64-apple-ios aarch64-apple-ios-sim   # a Mac with Xcode
#   scripts/phone-build.sh aarch64-linux-android                     # with the Android NDK
set -euo pipefail

[ $# -gt 0 ] || { echo "usage: $0 <target>..." >&2; exit 2; }

# Android 7.0, Tauri 2's default minimum (ADR-227).
android_api=24

for target in "$@"; do
  case "$target" in
    *-linux-android)
      ndk="${ANDROID_NDK_HOME:-}"
      [ -n "$ndk" ] || { echo "phone-build: set ANDROID_NDK_HOME to the Android NDK" >&2; exit 1; }
      # The NDK ships its compilers for an x86-64 host on Linux and as universal programs on macOS,
      # both under the x86_64 name.
      bin="$ndk/toolchains/llvm/prebuilt/$(uname -s | tr '[:upper:]' '[:lower:]')-x86_64/bin"
      clang="$bin/$target$android_api-clang"
      [ -x "$clang" ] || { echo "phone-build: no $clang in the NDK" >&2; exit 1; }
      upper=$(echo "$target" | tr '[:lower:]-' '[:upper:]_')
      export "CARGO_TARGET_${upper}_LINKER=$clang"
      export "CC_${target//-/_}=$clang"
      export "AR_${target//-/_}=$bin/llvm-ar"
      ;;
    *-apple-ios | *-apple-ios-sim) ;;
    *) echo "phone-build: $target isn't a phone target this script knows" >&2; exit 1 ;;
  esac
  rustup target add "$target"
  cargo build --locked --workspace --exclude xtask --exclude hpr-py --all-features --target "$target"
done
