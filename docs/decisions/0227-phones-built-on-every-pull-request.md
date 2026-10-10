# ADR-227: Phones built on every pull request (2026-10-10)

- **Status:** accepted
- **Summary:** CI builds the workspace for iPhone (`aarch64-apple-ios`), its simulator on Apple silicon (`aarch64-apple-ios-sim`) and 64-bit Android (`aarch64-linux-android`, Android 7.0 and later) on every pull request, so the mobile apps (M9.4) start from a core that already builds there. Build only: the tests run on phones when the apps do, and no phone archives or wheels are released.

**Context.** The mobile apps come after release 1.0 ([ADR-162][adr-162]): an offline web app
first, then iPhone and Android apps through Tauri 2, which links the native Rust core
(`ARCHITECTURE.md`). Until then nothing checks that the core builds for a phone. The wasm check
already keeps the pure core free of I/O, so what a phone build can break is the rest: `hpr-net`'s
TLS library `ring` compiles C and assembly with the target's own compiler, and any dependency
added before M9.4 could do the same, or only build on desktop operating systems. Found at M9.4,
such a dependency would be years old and load-bearing.

Rust supports all three targets in its second tier: the standard library is built and shipped
for them, and they're built (not tested) on every Rust change. GitHub's macOS runners carry
Xcode with the iPhone SDKs, and its Ubuntu runners carry the Android NDK
(`ANDROID_NDK_HOME`).

**Decision.**

1. **A `phone` job builds every crate with every feature** for the three targets, except `xtask`
   (a tool for this repository) and `hpr-py` (the Python package isn't built for phones), through
   `scripts/phone-build.sh`, which a Mac with Xcode or a machine with the NDK runs as is. It
   builds, not only checks, so linking the command-line program is covered too, and the C ABI
   once `hpr-ffi` builds a C library (M4.4).
2. **Android from API level 24 (Android 7.0)**, Tauri 2's default `minSdkVersion`
   ([config reference](https://v2.tauri.app/reference/config/)), revisited if M9.0 picks another
   framework. Only 64-bit ARM: 32-bit Android phones and x86-64 emulators wait for the apps.
3. **No tests on a phone yet.** iPhones use Apple's math library, which the macOS rows already
   test ([ADR-222][adr-222]). Android's (bionic) is a new one that can round differently in the
   last bit, so the tests run on an Android emulator when the apps exist (M9.4), unless one math
   library on every platform (#435) makes that moot first.

**Consequences.**

- Two more jobs on every pull request: one macOS, one Linux, each a build without tests.
- A dependency that doesn't build for a phone fails CI when it is added, while it is still easy
  to swap.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-222]: 0222-six-platforms-tested-on-every-pull-request.md
