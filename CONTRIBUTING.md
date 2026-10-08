# Contributing to FusionSpace HPR

FusionSpace HPR is open source under `MIT OR Apache-2.0`. These are the rules every change follows. They override
the roadmap and any deadline. The docs site, [hpr.fusionspace.co](https://hpr.fusionspace.co/), explains the code;
[`docs/ROADMAP.md`](docs/ROADMAP.md) holds the open work and [`docs/DECISIONS.md`](docs/DECISIONS.md) the design
decisions.

## The rules

### 1. Correct physics first

A number the simulator prints is a claim. Every model cites its source (a paper, report or standard) in its doc
comment and in `docs/physics/`, and tests pin every model. A model whose correctness is in doubt says so in its docs
instead of shipping silently.

### 2. Never weaken a check

Tolerances aren't loosened, tests aren't deleted or ignored, and reference data isn't edited to make a comparison
pass. A milestone's *done when* isn't rewritten to match what was built. If a target can't be met, a decision record
explains why, with the measurement, and the gap stays visible in the validation report.

### 3. Clean room

The project never reads, copies or ports GPL or AGPL source code, OpenRocket's Java source included. Allowed:

- published specifications, papers, file samples and file-format documentation;
- running GPL tools, such as the OpenRocket jar, as external oracles;
- reading and porting MIT, Apache or BSD code, with attribution in `THIRD-PARTY-NOTICES.md`;
- the maintainer's own MIT projects, `fusionspace-loft` and `fusionspace-debrief`.

`cargo deny` rejects copyleft dependencies. AltOS, AltosUI and orhelper are GPL: they're used only through
documented file formats and published docs.

### 4. Private data stays private

Some validation uses design files and flight logs that belong to other people. They live only in a gitignored
`refs/` folder and are never committed, quoted at length, or described in issues, pull requests or reports. Results
computed from them are published as counts, error statistics and anonymised case ids. A private design's name,
sizes or masses, or a ratio that back-computes them, are never published. Third-party data with an unclear license is
fetched and cached, never committed, and its status is recorded in `THIRD-PARTY-NOTICES.md`.

### 5. Offline first

Every core capability works with no network on macOS, Windows and Linux. Network features live in
`fusionspace-hpr-net`, behind a cargo feature, with an on-disk cache and a clear offline fallback. The core crates do
no I/O and compile to `wasm32-unknown-unknown`.

### 6. Scope until 1.0

Release 1.0 covers commercial (COTS) solid motors. Hybrids, liquids and research motors come after it, one at a
time, with extension points left for them now.

### 7. Releases are the maintainer's

Publishing packages, creating releases and tags, and changing repository settings are the maintainer's to do.
Changes reach `main` through pull requests that pass every CI check on all three operating systems.

## How a change ships

- Branch from `main`, keep commits small, and write messages in the imperative mood.
- Run `scripts/gate.sh`: it runs CI's own commands locally.
- Open a pull request that says what changed, how it was verified (with numbers) and what's left.
- Physics, numerics and validation changes are reviewed against their sources before they merge.

## Engineering standards

- Rust stable, pinned in `rust-toolchain.toml`, edition 2024.
- `f64` in the physics; SI units inside, converted only at the edges, with units in names or docs (`mass_kg`).
  Frames and signs are as `docs/physics/frames.md` defines them.
- Library code has no `unwrap`, `expect` or `panic!` except to state an invariant, with a comment saying why. Errors
  use `thiserror` in libraries. Public data types derive `serde`. Every public item is documented, and physics items
  give the equation and the citation.
- The same inputs and seed give bit-identical results on one platform.
- Unit tests sit next to the code; `proptest` covers invariants, `insta` snapshots parsers and exporters, analytic
  solutions test the integrator, `criterion` benches the hot paths, and validation cases live under `validation/`.
- Dependencies are mature crates; each new one gets a line of justification in its pull request.
