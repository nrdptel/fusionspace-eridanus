# ADR-002: The reference library: lock file, fetch, verify and doctor (2026-09-17)

- **Status:** accepted; the orhelper dependency and its doctor check superseded by ADR-035, and the Java floor replaced by a range (`max_major`) because OpenRocket 24.12 refuses 21
- **Summary:** The reference library: lock file, fetch, verify and doctor

**Context.** Validation needs other simulators, papers, motor data and private design files on
disk, reproducibly and never committed. M0.2 asks for `cargo xtask refs fetch|verify|doctor`
driven by `validation/refs.lock.toml`.

**Decision.**

- **One lock file, four kinds of item.**
  - `[[git]]`: pinned by full commit id, optionally shallow. Existing checkouts are reused and
    moved to the pin; checkouts with local changes are never touched.
  - `[[file]]`: an immutable download pinned by sha256 (the OpenRocket jar, 11 papers and the
    RockSim `.rse` spec).
  - `[[snapshot]]`: a live API (ThrustCurve, motor.fusionspace.co) pinned by the sha256 and date
    of one capture. APIs move (the motor finder hourly), so a missing snapshot that no longer
    matches fails. The new capture is kept beside it, with a record of its URL, its hash, its
    date and the pin it drifted from. `fetch --adopt-snapshots` moves that kept capture into place
    only if the entry still has the same URL and pin and the bytes are unchanged; otherwise it
    downloads a fresh capture and says why. It then rewrites only that entry's `sha256` and
    `captured` lines, using the capture's real date.
  - `[python]`: a `uv` project in `validation/oracles/` (`pyproject.toml` and `uv.lock`,
    committed), installed into `refs/venv` with `uv sync --locked`. That refuses a lock that is
    stale against `pyproject.toml` and checks every artifact against the hash in `uv.lock`;
    orhelper is pinned by git commit.

  Every destination must be a plain relative path inside the gitignored `refs/`, and
  destinations may not overlap.
- **Private repositories** (`private = true`, today `loft-fixtures`) are fetched with the
  user's git credentials, never prompting. A private repository that isn't checked out and
  doesn't answer `git ls-remote` (no access, as in CI) is skipped with a note. Any other failure,
  such as a bad pin or a checkout that can't move, fails. No partial checkout is left behind. A `sha256sum` manifest in a checkout can be pinned too;
  `verify` then hashes every listed file.
- **Idempotence:** anything already in its pinned state is left alone, so a second `fetch`
  downloads nothing. Downloads go to `<dest>.part` and are renamed into place only after the hash
  matches.
- **Verify re-hashes content**, not timestamps.
  - Files are hashed with SHA-256.
  - Git checkouts are compared, byte for byte (`core.autocrlf=false`), against a fresh temporary
    index read from the pinned commit. That makes git hash every tracked file, whereas a plain
    `git status` trusts cached stat data and can miss a same-size edit; a test shows both.
    Untracked files that the repository doesn't ignore also fail.
  - The environment must pass `uv sync --locked --check`, which compares package versions only.
    Then every installed file is re-hashed against the sha256 its package's `RECORD` lists
    (4,675 files today). Any file or symlinked directory that no `RECORD` lists fails, such as a
    `sitecustomize.py` or a `.pth` file, which would run at startup. The exception is the venv's
    own scaffolding (activation scripts, interpreter links, `pyvenv.cfg`, `_virtualenv.py`),
    which is allowed by name. Its content isn't pinned, because uv generates it, except
    `_virtualenv.pth`, whose one line is checked.
    The check runs Python with `-I`, so `PYTHONPATH` and the current directory can't leak in.
    `fetch` repairs hash failures with `uv sync --reinstall --no-cache`. Files no package owns
    can't be repaired that way, so `fetch` names them and asks for them to be deleted. uv installs by copying
    (`UV_LINK_MODE=copy`), so an edited venv file can't corrupt uv's cache through a hard link.
  - Git commands drop `GIT_DIR`-style variables, so a git hook can't redirect them at this
    repository.
- **External tools instead of crates:** `git`, `curl` and `uv` are run as commands. They exist on
  all three CI operating systems, and an HTTP and TLS stack in `xtask` would add many dependencies
  for no gain. curl is restricted to `https` (and `file` for tests), redirects included, and gives
  up on a transfer that stalls below 1 kB/s for a minute. New
  `xtask` dependencies: `serde`, `toml`, `sha2`, and `tempfile` for tests.
- **Doctor** lists the tools and their versions, a quick reference check (hashes of files,
  commits of checkouts), and whether each oracle is runnable. RocketPy: `rocketpy` imports at the
  locked version. OpenRocket: Java 17+ is found (`JAVA_HOME`, macOS `java_home`, Homebrew's
  keg-only `openjdk` formulae, then `PATH`), the jar verifies, `orhelper` and `jpype` import, and
  a smoke test starts the JVM through JPype and loads the jar's `Main-Class`. The smoke test sets
  `JAVA_HOME` from the runtime's own `java.home` property, which is reliable behind shims. The smoke test uses
  JPype (Apache-2.0) directly, so no project code calls into GPL orhelper. "Runnable" therefore
  means the runtime starts, not that a flight has been simulated; the M2.x oracle scripts do that.
- **Consistency tests:** every lock item needs a row in `THIRD-PARTY-NOTICES.md` with the same
  license and mode (and title, for papers), and every real URL uses `https`.
- **Source choices:** orhelper comes from `openrocket/orhelper` at a pinned commit, because PyPI's
  0.1.3 predates OpenRocket 24.12. The Knacke manual comes from archive.org's mirror of the DTIC
  copy, because DTIC refused automated downloads. It is a contractor report with a restrictive
  title-page notice, so it is labelled "unclear terms" and is never redistributed. RocketPy is a shallow clone of `v1.13.0`
  (about 390 MB, mostly ERA5 weather files that M2.3 needs).

**Consequences.**

- A fresh machine cannot reproduce a snapshot byte for byte once the API has moved; it has to
  adopt a new capture, and results that depend on it must name the capture date. Validation
  results that CI checks must come from committed fixtures, not from `refs/`.
- The `refs` tests need `git` and `curl` on the test machine. They use local `file://` sources
  and never touch the network.
- Java is not fetched: the user installs a JDK, and `doctor` says how.
- M2.2's oracle scripts will need a decision on importing GPL-2.0 orhelper from this repository,
  as opposed to driving the jar through JPype directly. This is recorded as the maintainer's call.

---
