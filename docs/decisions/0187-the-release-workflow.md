# ADR-187: The release workflow, its license texts, and `hpr validate` left to xtask (2026-10-06)

- **Status:** accepted; on 2026-10-08 Neer approved `(MIT OR Apache-2.0) AND Apache-2.0` for `fusionspace-hpr-io` and the Python package, which bundle OpenRocket's Apache-2.0 parts catalog
- **Summary:** M10.1c: a `Release` workflow builds and smoke-tests the command line's archives for three operating systems, the Python package's abi3 wheels and its source package, each carrying both licenses, THIRD-PARTY-NOTICES.md and its dependencies' license texts written by cargo-about; it publishes only from a dispatch that asks to, after approval in the `release` environment. Every library crate and `hpr-cli` become publishable. `hpr validate` leaves the command line, because a published crate can't depend on the unpublished harness; `cargo xtask validate --check` makes the same check.

**Context.** [ADR-162](0162-the-suite-and-its-releases.md) §2's checklist asks that every release
artifact build and pass a smoke test on all three operating systems from a CI dispatch, that
publishing wait on a `release` environment only Neer can approve, that `cargo publish --dry-run`
pass for each published crate, and that each archive and wheel carry both licenses,
`THIRD-PARTY-NOTICES.md` and its dependencies' license texts, which [ADR-114](0114-m4-3-the-python-package-and-its-split.md)
requires before any wheel is handed out. Release 0.1 ships the library crates, the command line
and the Python package, not FFI, WASM or the validation harness. Until now every crate said
`publish = false`, CI discarded the wheels it built, and nothing wrote license texts.

Two facts shaped the decision. First, `hpr-cli` depended on `hpr-validate` for `hpr validate`
([ADR-107](0107-hpr-validate-shares-the-projects-own-validation.md)). crates.io refuses a crate
whose dependency, optional or not, isn't published, and M10.1c keeps `hpr-validate` unpublished:
`cargo package` stopped there ("no matching package named `hpr-validate` found"). The 13 library
crates packaged and built from their packaged files at the first try. Second, the project's
local tooling refuses every `cargo publish` command, the dry run included, so the dry run
itself runs in CI; locally, `cargo package` makes the same package-and-build check.

**Decision.**

1. **What is published.** `hpr`, `hpr-aero`, `hpr-analysis`, `hpr-atmos`, `hpr-core`,
   `hpr-design`, `hpr-flightdata`, `hpr-forensics`, `hpr-format`, `hpr-io`, `hpr-motor`,
   `hpr-net`, `hpr-sim` and `hpr-cli`. The workspace no longer sets `publish`; `hpr-py`,
   `hpr-ffi`, `hpr-wasm`, `hpr-validate` and `xtask` each say `publish = false`. The Python package
   is published as wheels and a source package, not as a crate. `cargo test -p xtask` holds the
   list, refuses a published crate that depends on an unpublished one, and asks each published
   crate for a description and the workspace's license and repository.
2. **`hpr validate` leaves the command line.** The check it made is `hpr_validate::committed::check`,
   which `cargo xtask validate --check` calls too, so nothing is lost but the spelling: both needed
   a copy of the repository, and `hpr validate` needed an `hpr` built from that copy's commit. The
   docs now point to `cargo xtask validate --check`. Its JSON schema, its output types and its
   tests go with it; `hpr-validate` keeps the check and its own tests. Keeping the command behind
   a feature doesn't help, since crates.io resolves optional dependencies too; a second,
   unpublished `hpr` binary would make two programs of one name.
3. **One workflow, `.github/workflows/release.yml`.** A dispatch, or a pull request that changes
   the workflow or `scripts/release/`, runs five jobs:
   - `cli` on Linux, macOS and Windows: a clean release build of `hpr`, packed as
     `hpr-<version>-<target>` (`.tar.gz`, or `.zip` on Windows) with the README and the license
     files, then unpacked into an empty folder by `scripts/release/smoke-cli.sh`, which checks the
     files, `hpr --version`, the bundled catalog and an `hpr sim` flight against the apogee the
     command-line page prints (846.1 m, within a meter);
   - `wheel` on the three systems and `sdist` on Linux: `scripts/release/python.sh` builds them with
     CI's pinned maturin; `smoke-python.sh` installs each into fresh environments on CPython 3.10
     and 3.13 (the source package through pip's build, so compiled there) and checks the license
     files, the version and the Python page's example flight (1118.3 m, within a meter);
   - `crates`: `cargo publish --workspace --dry-run --locked`.
   The archives, wheels and source package are kept as the run's artifacts. A smoke test shows a
   file installs and runs as built; the test suite, which CI runs on every change, is what checks
   the numbers.
4. **Publishing waits for Neer.** Three jobs, `publish-crates`, `publish-pypi` and
   `github-release`, run only from a dispatch with `publish` ticked, after the five above pass, and
   each names `environment: release`, so each waits for the environment's reviewer. Crates go up by
   `cargo publish --workspace` with an environment secret, the Python files by PyPI's trusted
   publishing, and the archives into a *draft* GitHub release, whose publishing makes the tag. The
   environment's reviewer, the secret and PyPI's trusted publisher are settings only Neer can make,
   listed as the maintainer's steps. Only the maintainer dispatches with `publish` ticked
   ([rule 7](../../CONTRIBUTING.md#7-releases-are-the-maintainers)).
5. **License texts by cargo-about.** `scripts/release/licenses.sh` copies `LICENSE-MIT`,
   `LICENSE-APACHE` and `THIRD-PARTY-NOTICES.md`, and has cargo-about 0.9.2 (Embark Studios,
   MIT OR Apache-2.0, the makers of cargo-deny) write `THIRD-PARTY-LICENSES.txt`: each license's
   text, from the crates' own license files, with the crates that use it. It reads the crates
   offline from `Cargo.lock` (the text came out identical with and without the network), leaves
   out dev-dependencies, keeps build scripts and derive macros, and covers the four platforms
   the workflow builds on. Its accepted list is `deny.toml`'s allow list, held equal by a test, so
   a license CI admits is one the texts can be written for. For the `hpr` binary the file lists
   114 crates (100 from outside hpr-sim) under 7 licenses; for the Python package, 90 (77) under 5
   (Linux, macOS and Windows dependencies together, 2026-10-06).
6. **License files in a wheel.** PEP 639's `license-files` puts them in the wheel's
   `.dist-info/licenses/` and in the source package. The checkout's `pyproject.toml` doesn't name
   them, because a build that can't find a named file fails and a test build shouldn't need
   cargo-about; `scripts/release/python.sh` stages the files beside it and adds the line for that
   build only, fails if the line didn't land, and puts the checkout's file back when it ends.

**Consequences.** M10.1c's crates are publishable, but nothing is published until Neer sets up
the environment and dispatches. `hpr validate` is gone from the command line from this commit on;
`cargo xtask validate --check` is the way to re-run the validation check. The `crates.io` names
`hpr`, `hpr-cli`, `hpr-core` and the rest, and PyPI's `hpr-sim`, were all free on 2026-10-06; until
the first publish, anyone may take them. The license texts are only as good as the crates' own
license files: a crate that ships none gets the license's standard text, without its copyright
line. The workflow's own run on its pull request, and a dispatch from `main` after it merges, are
its first tests; the publishing jobs have never run.
