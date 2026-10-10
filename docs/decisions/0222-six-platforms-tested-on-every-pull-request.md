# ADR-222: Six platforms, each tested on every pull request (2026-10-09)

- **Status:** accepted
- **Summary:** CI's per-platform jobs (the tests, the examples, the Python package and the validation cases) and the release's archives and wheels now run on six platforms: Linux, macOS and Windows, each on x86-64 and on 64-bit ARM. Each runs natively on its own CPU, and a script fails the job if the toolchain would build for another one. Intel Macs are tested on GitHub's last Intel image until it retires (fall 2027), then under Rosetta. Windows on ARM tests the Python package from 3.11, its oldest CPython. Static Linux builds, 32-bit ARM, package managers and math that rounds the same everywhere are follow-ups.

**Context.** Release 0.1 shipped archives and wheels for three platforms: Linux on x86-64, Macs
with Apple silicon and Windows on x86-64. Anyone else built from source with Rust. The maintainer
asked for as wide a reach as the project can hold. Three facts make the other three common
platforms cheap now:

- GitHub's hosted runners for Linux on ARM (`ubuntu-24.04-arm`) and Windows on ARM
  (`windows-11-arm`) have been generally available, and free for public repositories, since
  2025-08-07 ([changelog](https://github.blog/changelog/2025-08-07-arm64-hosted-runners-for-public-repositories-are-now-generally-available/)).
  Intel Macs run on `macos-15-intel`, which GitHub says is its last Intel image, retired in fall
  2027 ([changelog](https://github.blog/changelog/2025-09-19-github-actions-macos-13-runner-image-is-closing-down/)).
- Rust ranks `aarch64-unknown-linux-gnu` and, since 1.91, `aarch64-pc-windows-msvc` in its top
  tier; `x86_64-apple-darwin` dropped to the second tier in 1.90
  ([RFC 3841](https://rust-lang.github.io/rfcs/3841-demote-x86_64-apple-darwin.html)), and macOS 26
  is the last release for Intel Macs.
- The code has no architecture-specific paths: no `target_arch`, no SIMD, byte orders spelled out.
  The only native code in a shipped program is `ring`, the TLS library under `hpr-net`, which
  builds on Windows on ARM with clang, which the runner image carries.

What can differ is arithmetic. Rust's `+`, `*` and `/` round the same on every platform, but
`sin`, `exp`, `powf` and `ln` call each platform's math library, and the standard library's docs
say their precision "varies by platform". Apple's, glibc's and Microsoft's libraries already
disagree in the last bit, and earlier milestones fixed several checks that only failed off macOS.
Windows on ARM and Linux on ARM bring two more math libraries; only running every check on them
shows whether a comparison was relying on one.

**Decision.**

1. **Six platforms, every check that runs per platform.** `test` (both halves), `examples`,
   `python` and `validate` run on `ubuntu-latest`, `ubuntu-24.04-arm`, `macos-latest` (Apple
   silicon), `macos-15-intel`, `windows-latest` and `windows-11-arm`. The release workflow builds
   and smoke-tests an archive and an abi3 wheel on each; the Linux ones on ARM are linked for
   glibc 2.17 and smoke-tested in the `manylinux2014_aarch64` image, as the x86-64 ones are in
   `manylinux2014_x86_64`. The checks run once, on Linux x86-64, as before: they don't depend on
   the platform.
2. **Native builds only.** Every platform's programs are built and run on that platform's own CPU.
   An ARM runner whose toolchain was x86-64 would build x86-64 programs that pass under emulation
   while testing nothing new, so `scripts/native-toolchain.sh` fails the job unless the toolchain
   targets the runner's own CPU.
3. **Windows on ARM tests Python from 3.11.** CPython's Windows on ARM builds start at 3.11. The
   wheel keeps the `cp310-abi3` tag, which is harmless there because no 3.10 exists to install it
   on.
4. **Intel Macs until the tools end.** When GitHub retires `macos-15-intel`, the Intel build moves
   to an Apple-silicon runner (`--target x86_64-apple-darwin`) and its tests run under Rosetta,
   which every Apple-silicon runner image installs. When Rosetta narrows (Apple says macOS 28) a
   new record decides whether Intel Macs stay.
5. **Not now; each is an issue.** Static Linux builds (musl, #433), which also cover Alpine and
   other distributions without glibc; 32-bit ARM (older Raspberry Pi systems, #436); package
   managers (`cargo binstall` metadata needs only files, while Homebrew, Scoop, winget and
   conda-forge need the maintainer's accounts, #434); and one math library on every platform (#435),
   so each platform's results match bit for bit and a check can't pass on one by luck.

**Consequences.**

- More checks gate every merge. GitHub runs at most 5 macOS jobs at once on a free plan, and the
  Intel image doubles each run's macOS jobs, so the slowest runs wait for a macOS runner. The pull
  request that made this change measured each run's time.
- A check that fails only on a new platform is a defect to find, not a tolerance to widen
  ([CONTRIBUTING.md](../../CONTRIBUTING.md), rule 2): the platforms' math libraries differ, so
  the fix is usually a comparison that relied on one, as in the earlier cases.
- Release 0.2 is the first with the new archives and wheels; release 0.1's files stay on three
  platforms.
