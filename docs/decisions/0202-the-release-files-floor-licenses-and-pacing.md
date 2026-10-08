# ADR-202: The release files: glibc floor, license copies and a paced publish (2026-10-08)

- **Status:** accepted
- **Summary:** #372: the Linux archive and wheel are built with Zig for glibc 2.17 (manylinux2014), checked by `objdump` and smoke-tested inside a pinned manylinux2014 image; each publishable crate holds byte-identical copies of both licenses, tested; crates go up one at a time through `scripts/release/publish-crates.sh`, paced to crates.io's limit on new names and resumable. Amends ADR-187 §4.

**Context.** [ADR-187](0187-the-release-workflow.md) built and smoke-tested every release file,
and [issue #372](https://github.com/nrdptel/hpr-sim/issues/372) found three things to settle
before the first publish. The Linux files were built on `ubuntu-latest` and needed glibc 2.35,
which rules out RHEL 9, Debian 11 and Ubuntu 20.04. No published crate carried a license text,
though Apache-2.0 §4(a) asks that recipients get one. And `cargo publish --workspace` uploads 14
new names in one go, while crates.io allows a burst of 5 new names and then one every 10 minutes
([crates.io/docs/rate-limits](https://crates.io/docs/rate-limits), read 2026-10-08; the same in
`rust-lang/crates.io`'s `src/rate_limiter.rs`). A run that stops partway leaves a partial publish
that a rerun can't resume.

**Decision.**

1. **glibc 2.17.** The CLI is built by `cargo-zigbuild` for `x86_64-unknown-linux-gnu.2.17`, and
   the wheel by `maturin build --zig --compatibility manylinux2014`. Both run on the ordinary
   runner, so the license staging and packaging scripts stay as they were; a container build would
   have needed them inside the image. Zig, `cargo-zigbuild` and `maturin` are pinned in
   `scripts/release/linux.sh`. CI checks that every program and library needs at most
   `GLIBC_2.17` (`check-glibc.sh`, by `objdump -T`) and that the wheel's tag is `manylinux_2_17`.
   It then reruns the smoke tests inside `quay.io/pypa/manylinux2014_x86_64`, pinned by digest,
   after asserting that the image reports glibc 2.17. macOS and Windows build as before.
2. **License copies.** Each of the 14 publishable crates holds `LICENSE-MIT` and `LICENSE-APACHE`.
   Cargo packs no file from outside a crate's folder, and a symlink breaks on a Windows checkout,
   so these are copies. The xtask test `every_published_crate_carries_the_two_licenses` takes the
   crates from `cargo metadata` and holds the copies byte-identical to the root's.
3. **A paced, resumable publish.** `scripts/release/publish-crates.sh` publishes the crates one at
   a time in dependency order, and an xtask test checks that order against `cargo metadata`'s
   graph, dev-dependencies included. It skips a version the sparse index already lists. After the
   fifth new name it waits 10.5 minutes before each further one. On a 429 it waits until the time
   crates.io names, then retries. So a failed job reruns where it stopped. The `release`
   environment's approval stays. The `crates` job runs it with `--dry-run`, which checks its index
   lookups and prints the plan.

**Consequences.** The first publish of 14 new names takes about 95 minutes. The maintainer's release
steps say so, and say to rerun failed jobs after a stop. The CHANGELOG's two #372 known gaps go. The
floor is stated in `docs/releasing.md`, *Getting started* and *Python*, and an xtask test holds
those pages to `linux.sh`'s floor. When M10.1d9 renames the crates, the script's list moves with
them, and the order test fails until it does. The manylinux2014 smoke test needs Docker, so it runs
only in CI.
