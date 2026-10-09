# How a release is built

**Nothing has been released yet: when a release comes, one workflow builds every file of it from
one commit, tests each as you would get it, and publishes nothing until Neer Patel, who owns the
project, approves.** This page says what a release holds, which license files come with each file,
how each is checked, and how to check one yourself. The building and checking run when the workflow
is started by hand, and on every pull request that changes it or its scripts (`scripts/release/`).
The publishing steps have never run.

## What a release holds

A release has one version for everything in it, and its title is `FusionSpace HPR <version>`.
Release 0.1 is the simulator: the library, the `hpr` command line and the Python package
([the release plan](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0162-the-suite-and-its-releases.md)).
[The changelog](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md) has each version's
entry: what it holds, what works, how far to trust it, and its known gaps.
[Install a release](getting-started.md#install-a-release) says how to install each file.

| File | What it is | Where it goes |
|---|---|---|
| `fusionspace-hpr-<version>-x86_64-unknown-linux-gnu.tar.gz` | the `hpr` command line for Linux on x86-64 | the GitHub release |
| `fusionspace-hpr-<version>-aarch64-apple-darwin.tar.gz` | the command line for Macs with Apple silicon | the GitHub release |
| `fusionspace-hpr-<version>-x86_64-pc-windows-msvc.zip` | the command line for Windows on x86-64 | the GitHub release |
| `fusionspace_hpr-<version>-cp310-abi3-<platform>.whl` | the Python package, one wheel per operating system, for CPython 3.10 and later; it imports as `fusionspace.hpr` | PyPI, as `fusionspace-hpr` |
| `fusionspace_hpr-<version>.tar.gz` | the Python package's source; installing it compiles it, so it needs Rust | PyPI |
| 14 crates | `fusionspace-hpr`, the library's front door, the 12 `fusionspace-hpr-*` crates under it, and `fusionspace-hpr-cli`, the command line | crates.io |

Each file is built on the CPU of the machine CI builds it on, so there is no archive or wheel yet
for Intel Macs or for Linux on ARM; there, `pip install` builds the Python package from its source
package, which needs [Rust](https://rustup.rs).

**The Linux files run on the GNU C library (glibc) 2.17 or later,** the floor of the
[manylinux2014](https://peps.python.org/pep-0599/) wheel standard and of Rust's own Linux target:
RHEL and CentOS 7, Debian 8, Ubuntu 14.04 and anything newer. A program built the usual way asks
for the build machine's own glibc, so the Linux archive and wheel are linked by
[Zig](https://ziglang.org) instead, which carries the old library's list of functions: [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
builds the command line, and [maturin](https://www.maturin.rs/)'s `--zig` option the wheel, tagged
`manylinux_2_17`. maturin refuses a wheel that calls a newer function. The versions are pinned in
`scripts/release/linux.sh`: cargo-zigbuild 0.23.4 and Zig 0.16.0, both fetched from PyPI by uv.

Four crates and the repository's own tool stay unpublished. `hpr-py` reaches Python users as the
wheels. `hpr-ffi` and `hpr-wasm` wait for their milestone. `hpr-validate`, the validation harness,
needs a copy of the repository, its cases and their references; to re-run its check, clone the
repository and run `cargo xtask validate --check` ([Accuracy](accuracy.md#the-census)).

## The license files

**Each archive, wheel and source package carries four license files:** `LICENSE-MIT` and
`LICENSE-APACHE`, the project's two licenses, either at your option; `THIRD-PARTY-NOTICES.md`, every
outside source the project uses, such as bundled data; and `THIRD-PARTY-LICENSES.txt`, the license
texts of the Rust crates compiled in. An archive has them beside `hpr`. A wheel has them in its
metadata folder, `fusionspace_hpr-<version>.dist-info/licenses/`, where `pip show -f fusionspace-hpr`
lists them.

**Each crate on crates.io carries `LICENSE-MIT` and `LICENSE-APACHE`,** since the Apache license
asks that everyone who gets the code gets its text. cargo packs only the files in a crate's own
folder, so each published crate's folder holds a copy of the two, and `cargo test -p xtask` fails
if a copy differs from the repository's by a byte.

`THIRD-PARTY-LICENSES.txt` is written for each build by
[cargo-about](https://github.com/EmbarkStudios/cargo-about) from the crates' own license files. It
lists each license once, with its text and the crates that use it. Built on 2026-10-06, the
command line's file listed 114 crates under 7 licenses, 100 of them from outside the project; the
Python package's listed 90 under 5, 77 from outside. A crate may appear under several texts, as
`ring` does under 18 ISC notices. The licenses it accepts are the ones the project's dependency check
allows, all permissive, so a crate under any other license stops the build.

## How each file is checked

Each file is unpacked or installed in an empty folder and run, as a user would get it, from a copy
of the repository that provides the example design:

- **An archive** must hold `hpr`, the README and the four license files, with the texts naming
  `clap`, a crate the command line is built on; `hpr --version` must name
  the release's version; `hpr motors show H54` must read the bundled motor catalog; and
  `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54` must reach
  846.1 m (2,776 ft) above the site, within a meter, the apogee [the command line's page](cli.md)
  prints for that example.
- **A wheel or the source package** is installed into fresh environments on Python 3.10 and 3.13.
  It must carry the four license files, the texts naming `pyo3`, the crate that binds Rust to
  Python, and the release's version, and the first example on the
  [Python](python.md) page must reach 1118.3 m (3669 ft), within a meter, as that page prints.
- **The Linux archive and wheel** are then checked against the glibc floor twice.
  `scripts/release/check-glibc.sh` reads each program and library in them with `objdump` and
  fails if one asks for a glibc function newer than 2.17, or if the wheel's name lacks the
  `manylinux_2_17` tag. `scripts/release/smoke-manylinux.sh` runs the same smoke test inside
  pypa's manylinux2014 image, a CentOS 7 with glibc 2.17, installing the wheel with pip on
  CPython 3.10 and 3.13.
- **The crates** must each package, and build from their packaged files alone
  (`cargo publish --workspace --dry-run`), and `scripts/release/publish-crates.sh --dry-run`
  must read crates.io's index as expected and print the order it would publish in.

These smoke tests show a file installs and runs as built. They don't check the physics: the test
suite and the validation cases do that, on every change ([Accuracy](accuracy.md)).

## Publishing

**Neer publishes in two steps:** he starts the workflow with *publish* ticked, and then approves
its `release` [environment](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments),
a GitHub setting whose required reviewer is him. Until he has made that setting, the workflow is
never started with *publish* ticked. The crates then go to crates.io, the wheels and
source package to PyPI, and the archives into a draft GitHub release; publishing the draft makes
the version's tag.

**The first publish of the crates takes about two hours, because crates.io paces new names.**
It lets one account publish 5 new crates at once, then one every 10 minutes
([crates.io's rate limits](https://crates.io/docs/rate-limits), read 2026-10-08). Release 0.1
publishes 14, so 9 of them wait. `scripts/release/publish-crates.sh` publishes the crates one at a
time, each after the crates it depends on, and once the first five new names are out it waits
10.5 minutes before each next one: 9 waits, about 95 minutes, plus the time to build each crate.
The release workflow runs it after you approve the `release` environment, with the repository's
`CARGO_REGISTRY_TOKEN` secret. A name already on crates.io that `nrdptel` doesn't own stops the
run. A later release adds
versions to crates that exist, which crates.io allows 30 at once, so it doesn't wait.

**A publish that stops partway can be run again.** A published version can't be withdrawn, only
[yanked](https://doc.rust-lang.org/cargo/commands/cargo-yank.html) (marked as not for new use), so a second run must not start over. The script asks crates.io's
index which versions are there and skips those; if crates.io still answers that the limit is
reached, it waits until the time crates.io names and tries again.

## Checking a file yourself

From a copy of the repository at the release's commit, with [uv](https://docs.astral.sh/uv/)
installed:

```text
scripts/release/smoke-cli.sh fusionspace-hpr-0.1.0-aarch64-apple-darwin.tar.gz
scripts/release/smoke-python.sh fusionspace_hpr-0.1.0-cp310-abi3-macosx_11_0_arm64.whl
```

Each prints what it checked and ends with `passed`, or stops at the first check that fails. To
build the files yourself, `scripts/release/archive.sh dist` and `scripts/release/python.sh wheel
dist` need cargo-about 0.9.2 (`cargo install cargo-about --locked --version 0.9.2 --features cli`)
and uv, which fetches maturin and, on Linux, cargo-zigbuild and Zig. On Linux,
`scripts/release/check-glibc.sh <file>` checks a file against the glibc floor, and
`scripts/release/smoke-manylinux.sh <file>`, which needs Docker, runs its smoke test on that glibc.
How the workflow was chosen is in its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0187-the-release-workflow.md).
