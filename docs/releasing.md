# How a release is built

See what a FusionSpace HPR release holds, which license files come with it, how each is checked, and
how to check one yourself.

**Release 0.1.0 came out on 2026-10-08. One workflow builds each release, every file of it from
one commit. It tests each file as you would get it, and publishes nothing until Neer Patel, who
owns the project, approves.** The building and checking run when the workflow is started by hand.
They also run on every pull request that changes the workflow or its scripts (`scripts/release/`).
The publishing steps first ran for 0.1.0, on 2026-10-08.

> **How far to trust it.** This page records how the release workflow builds and checks each file,
> not a result. Each file is run as you would get it. The archive's example flight must reach
> 846.1 m (2,776 ft), and the Python page's first example 1118.3 m (3669 ft), each within a meter.
> Those smoke tests don't check the physics; the test suite and the validation cases do
> ([Accuracy](accuracy.md)). For what a release does and its known gaps, rely on
> [the changelog](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md). Where
> [the workflow](https://github.com/nrdptel/fusionspace-eridanus/blob/main/.github/workflows/release.yml)
> and this page differ, rely on the workflow.

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
| `fusionspace-hpr-<version>-aarch64-unknown-linux-gnu.tar.gz` | the command line for Linux on 64-bit ARM, such as a Raspberry Pi 4 or 5 on a 64-bit system | the GitHub release |
| `fusionspace-hpr-<version>-aarch64-apple-darwin.tar.gz` | the command line for Macs with Apple silicon | the GitHub release |
| `fusionspace-hpr-<version>-x86_64-pc-windows-msvc.zip` | the command line for Windows on x86-64 | the GitHub release |
| `fusionspace-hpr-<version>-aarch64-pc-windows-msvc.zip` | the command line for Windows on ARM | the GitHub release |
| `fusionspace_hpr-<version>-cp310-abi3-<platform>.whl` | the Python package, one wheel per platform, for CPython 3.10 and later (3.11 on Windows on ARM, its oldest); it imports as `fusionspace.hpr` | PyPI, as `fusionspace-hpr`, and the GitHub release |
| `fusionspace_hpr-<version>.tar.gz` | the Python package's source; installing it compiles it, so it needs Rust | PyPI and the GitHub release |
| `fusionspace-hpr.rb` | the Homebrew formula for the Mac and Linux archives, from release 0.2; 0.1's was written afterwards by the same script and put in the tap by hand | the GitHub release, then [the tap](https://github.com/nrdptel/homebrew-tap) |
| `fusionspace-hpr.json` | the Scoop manifest for the Windows archives, from release 0.2; 0.1's was written afterwards by the same script and put in the bucket by hand | the GitHub release, then [the bucket](https://github.com/nrdptel/scoop-bucket) |
| 14 crates | `fusionspace-hpr`, the library's front door, the 12 `fusionspace-hpr-*` crates under it, and `fusionspace-hpr-cli`, the command line | crates.io |

Each file is built and tested on the platform it's for, on that platform's own CPU
([the six platforms](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0222-six-platforms-tested-on-every-pull-request.md)).
Five run now: Linux and Windows, each on x86-64 and on 64-bit ARM, and Macs with Apple silicon. Release
0.1 had the x86-64 Linux, Apple silicon and x86-64 Windows files only. The ARM Linux and ARM
Windows files start with release 0.2. Intel Macs follow
([issue #438](https://github.com/nrdptel/fusionspace-eridanus/issues/438)). Anywhere else,
`pip install` builds the Python package from its source package, which needs
[Rust](https://rustup.rs).

**The Linux files run on the GNU C library (glibc) 2.17 or later.** That's the floor of the
[manylinux2014](https://peps.python.org/pep-0599/) wheel standard and of Rust's own Linux target:
RHEL and CentOS 7, Debian 8, Ubuntu 14.04 and anything newer. A program built the usual way asks
for the build machine's own glibc. So the Linux archive and wheel are linked by
[Zig](https://ziglang.org) instead, which carries the old library's list of functions. The
command line is built by [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild), and the
wheel by [maturin](https://www.maturin.rs/)'s `--zig` option, tagged `manylinux_2_17`. A wheel that
calls a newer function is refused by maturin. The versions are pinned in
`scripts/release/linux.sh`: cargo-zigbuild 0.23.4 and Zig 0.16.0, both fetched from PyPI by uv.

Four crates and the repository's own tool stay unpublished. `hpr-py` reaches Python users as the
wheels. `hpr-ffi` and `hpr-wasm` wait for their milestone. `hpr-validate`, the validation harness,
needs a copy of the repository, its cases and their references. To re-run its check, clone the
repository and run `cargo xtask validate --check` ([Accuracy](accuracy.md#the-census)).

## The license files

**Each archive, wheel and source package carries four license files.** `LICENSE-MIT` and
`LICENSE-APACHE` are the project's two licenses, either at your option. `THIRD-PARTY-NOTICES.md`
lists every outside source the project uses, such as bundled data. `THIRD-PARTY-LICENSES.txt` holds
the license texts of the Rust crates compiled in. An archive has them beside `hpr`. A wheel has them
in its metadata folder, `fusionspace_hpr-<version>.dist-info/licenses/`, where
`pip show -f fusionspace-hpr` lists them.

**Each crate on crates.io carries `LICENSE-MIT` and `LICENSE-APACHE`,** since the Apache license
asks that everyone who gets the code gets its text. Cargo packs only the files in a crate's own
folder, so each published crate's folder holds a copy of the two. `cargo test -p xtask` fails if a
copy differs from the repository's by a byte.

`THIRD-PARTY-LICENSES.txt` is written for each build by
[cargo-about](https://github.com/EmbarkStudios/cargo-about) from the crates' own license files. It
lists each license once, with its text and the crates that use it. Built on 2026-10-06, the
command line's file listed 114 crates under 7 licenses, 100 of them from outside the project. The
Python package's listed 90 under 5, 77 from outside. A crate may appear under several texts, as
`ring` does under 18 ISC notices. The licenses cargo-about accepts are the ones the project's
dependency check allows, all permissive. So a crate under any other license stops the build.

## How each file is checked

Each file is unpacked or installed in an empty folder and run, as a user would get it. A copy of
the repository provides the example design:

- **An archive** must hold `hpr`, the README and the four license files, with the texts naming
  `clap`, a crate the command line is built on. `hpr --version` must name the release's version,
  and `hpr motors show H54` must read the bundled motor catalog.
  `hpr sim validation/fixtures/ork/pod-flights/pods-none.ork --motor H54` must reach 846.1 m
  (2,776 ft) above the site, within a meter. That's the apogee [the command line's page](cli.md)
  prints for that example.
- **A wheel or the source package** is installed into fresh environments on Python 3.10 (3.11 on
  Windows on ARM) and 3.13. It must carry the four license files and the release's version, with
  the texts naming `pyo3`, the crate that binds Rust to Python. The first example on the
  [Python](python.md) page must reach 1118.3 m (3669 ft), within a meter, as that page prints.
- **The Windows archive's `hpr.exe`** must need no Visual C++ runtime, which a clean Windows may
  lack. `scripts/release/check_windows_runtime.py` reads the program's import tables. It fails if
  one names `VCRUNTIME140.dll` or another DLL that only Microsoft's Visual C++ Redistributable
  installs.
  - From release 0.2, `scripts/release/archive.sh` links the C runtime into the Windows program
    (`-C target-feature=+crt-static`, for the Windows targets only).
  - Release 0.1.0's `hpr.exe` imported `VCRUNTIME140.dll`. Run once by hand on the published
    0.1.0 program, the check exits with an error naming that DLL. That was a one-off
    measurement; the repository's tests run the check on small invented programs instead.
  - The wheels keep the C runtime as a DLL, and their imports aren't checked. The Windows
    installer from python.org puts `vcruntime140.dll` beside `python.exe` (CPython's
    `Tools/msi/exe/exe_files.wxs`, and `PC/layout` for its other packages). `python.exe` has
    loaded it before it loads the module.
- **The Linux archive and wheel** are then checked against the glibc floor twice.
  `scripts/release/check-glibc.sh` reads each program and library in them with `objdump`. It fails
  if one asks for a glibc function newer than 2.17, or if the wheel's name lacks the
  `manylinux_2_17` tag. `scripts/release/smoke-manylinux.sh` runs the same smoke test inside pypa's
  manylinux2014 image, a CentOS 7 with glibc 2.17. It installs the wheel with pip on CPython 3.10
  and 3.13.
- **The Homebrew formula** must pass `brew style` and `brew audit --strict`. Then it must install
  with `brew install` and pass its own `brew test` (`hpr --version` names the version). Each check
  runs on a Mac with Apple silicon and on Linux on x86-64 and on 64-bit ARM. The release the
  formula names doesn't exist yet, so the install uses a copy whose addresses point at the run's
  own archives. The hashes stay as written (`scripts/release/smoke-homebrew.sh`).
- **The Scoop manifest** must install with `scoop install` on Windows on x86-64 and on ARM, each
  taking its own archive. It's installed the same way, from a copy whose addresses point at the
  run's own archive, served from the runner. The hash and folder stay as written. The installed
  `hpr --version` must name the version (`scripts/release/smoke-scoop.ps1`).
- **The crates** must each package, and build from their packaged files alone
  (`cargo publish --workspace --dry-run`). `scripts/release/publish-crates.sh --dry-run` must read
  crates.io's index as expected and print the order it would publish in.

These smoke tests show a file installs and runs as built. They don't check the physics: the test
suite and the validation cases do that, on every change ([Accuracy](accuracy.md)).

## Publishing

**Neer publishes in two steps.** He starts the workflow with *publish* ticked, then approves its
`release` [environment](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/manage-environments),
a GitHub setting whose required reviewer is him. The workflow is started with *publish* ticked only
with that setting in place. The crates then go to crates.io, and the wheels and source package to
PyPI. A draft GitHub release gets the archives, the Homebrew formula and the Scoop manifest, and the
wheels and source package too. Publishing the draft makes the version's tag.

**The first publish of the crates takes about two hours, because crates.io paces new names.**
It lets one account publish 5 new crates at once, then one every 10 minutes
([crates.io's rate limits](https://crates.io/docs/rate-limits), read 2026-10-08). Release 0.1
publishes 14, so 9 of them wait. `scripts/release/publish-crates.sh` publishes the crates one at a
time, each after the crates it depends on. Once the first five new names are out, it waits 10.5
minutes before each next one. That's 9 waits, about 95 minutes, plus the time to build each crate.
A name already on crates.io that `nrdptel` doesn't own stops the run. A later release adds versions
to crates that exist. crates.io allows 30 new versions at once, so it doesn't wait.

**Later releases publish without a stored token.** The release workflow uses crates.io's
[trusted publishing](https://crates.io/docs/trusted-publishing). Each crate's settings on crates.io
name this repository, `release.yml` and the `release` environment. After you approve the
environment, the job trades GitHub's identity token for a crates.io token. That token lasts 30
minutes and is revoked when the job ends. It can add versions but can't create a crate. So the
workflow runs the script with `--versions-only`, which stops before publishing anything if a
crate's name is new. A new crate goes up once from your machine (`cargo publish --package <name>`
with an API token). Then it gets the same trusted-publishing entry on crates.io: the crate's
Settings, then Trusted Publishing, with owner `nrdptel`, repository `fusionspace-eridanus`,
workflow `release.yml` and environment `release`.

**A publish that stops partway can be run again.** A published version can't be withdrawn, only
[yanked](https://doc.rust-lang.org/cargo/commands/cargo-yank.html) (marked as not for new use).
So a second run must not start over. The script asks crates.io's index which versions are there
and skips those. If crates.io still answers that the limit is reached, it waits until the time
crates.io names and tries again.

## Homebrew, Scoop and cargo binstall

**Each release brings its own Homebrew formula and Scoop manifest, and the tap and the bucket copy
them from it.** A *formula* is the Ruby file that tells Homebrew where to download a program, its
SHA-256 hash and how to install it. A *tap* is a Git repository of formulas outside Homebrew's own.
Scoop's equivalents are a *manifest*, a JSON file, and a *bucket*, a repository of manifests.

`scripts/release/package_managers.py` writes both files from the archives' names and hashes. Each
has an entry for each of its archives the release has, and none for a system it lacks. So release 0.1's
formula refuses an Intel Mac rather than install a program it can't run. `cargo test -p xtask`
runs the script on invented archives. It compares the formula with the expected one byte for byte,
and the manifest as parsed JSON, written with Unix line ends. The release attaches both files, and
publishing the draft makes them the latest release's.

**The formula and the manifest both state the program's license as
`(MIT OR Apache-2.0) AND Apache-2.0`.** That's the project's two licenses, either at your option,
and Apache-2.0 for OpenRocket's parts catalog, which the program carries
([the release workflow's decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0187-the-release-workflow.md)).
It's an SPDX license expression, the standard way to write which licenses apply, and the one the
Python package states. The formula writes it in Homebrew's form, one term a line,
`license all_of: [{ any_of: ["MIT", "Apache-2.0"] }, "Apache-2.0"]`. That's what `brew style`,
Homebrew's checker for a formula's layout, asks for. Scoop's own form has no parentheses to group
a choice inside a requirement. So the manifest gives the SPDX expression as its license
`identifier`, with a `url` to the README's license section.

**The manifest suggests no Visual C++ runtime,** since from release 0.2 the Windows program needs
none.

**The formula and manifest checks hold up every publishing job, on purpose.** The crates, the
Python package and the draft release all wait for them, so a release goes out on every channel or
on none. The cost is that a new check in Homebrew's `brew audit` or `brew style`, or a change in
Scoop, can stop a release that changed nothing here. If the cause was passing, such as a download
that timed out, rerun the failed job. Otherwise fix the generator or the formula it writes, and
start the workflow again.

The tap, [nrdptel/homebrew-tap](https://github.com/nrdptel/homebrew-tap), and the bucket,
[nrdptel/scoop-bucket](https://github.com/nrdptel/scoop-bucket), each run a workflow once a day.
It downloads the latest release's file and commits it when it changed. A new release reaches
`brew upgrade` and `scoop update` within a day. It reaches them at once if you run that workflow
by hand from its repository's Actions tab (*Run workflow*). Each workflow reads the public release
and pushes with its own repository's token. So neither repository holds a credential for the
other, nor this one for them. Release 0.1 came out before the formula and manifest existed. The
tap and the bucket were given files written by the same script from its archives.

**GitHub turns off a public repository's scheduled workflows after 60 days with no activity.**
Releases further apart than that leave the tap or the bucket on the old version, and GitHub shows
the workflow as disabled. Turn it back on with *Enable workflow* on its Actions page, or with
`gh workflow enable homebrew-update.yml --repo nrdptel/homebrew-tap` (and `scoop-update.yml`,
`nrdptel/scoop-bucket`). Then run it once by hand.

**`cargo binstall fusionspace-hpr-cli` reads where the archives are from the crate's own
manifest** (`[package.metadata.binstall]` in `crates/hpr-cli/Cargo.toml`). Those lines give the release's
tag, the archive's name, and `hpr` in the folder inside it. QuickInstall, a
third-party service that builds and hosts crates' programs, is binstall's fallback, and it's turned
off. So binstall takes only this repository's archives, or compiles. The crate on crates.io carries those lines from release 0.2.
Release 0.1.0's doesn't, so binstall finds no archive for it.

Packages for winget and conda-forge are planned
([issue #434](https://github.com/nrdptel/fusionspace-eridanus/issues/434)).

## Checking a file yourself

From a copy of the repository at the release's commit, with [uv](https://docs.astral.sh/uv/)
installed:

```text
scripts/release/smoke-cli.sh fusionspace-hpr-0.1.0-aarch64-apple-darwin.tar.gz
scripts/release/smoke-python.sh fusionspace_hpr-0.1.0-cp310-abi3-macosx_11_0_arm64.whl
```

Each prints what it checked and ends with `passed`, or stops at the first check that fails. To
build the files yourself, `scripts/release/archive.sh dist` and `scripts/release/python.sh wheel
dist` need cargo-about 0.9.2 (`cargo install cargo-about --locked --version 0.9.2 --features cli`).
They also need uv, which fetches maturin and, on Linux, cargo-zigbuild and Zig. On Linux,
`scripts/release/check-glibc.sh <file>` checks a file against the glibc floor.
`scripts/release/smoke-manylinux.sh <file>`, which needs Docker, runs its smoke test on that glibc.
How the workflow was chosen is in its
[decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0187-the-release-workflow.md).
