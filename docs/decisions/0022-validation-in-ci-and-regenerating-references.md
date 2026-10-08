# ADR-022: Validation in CI, and regenerating references only by hand (2026-09-18)

- **Status:** accepted
- **Summary:** Validation in CI, and regenerating references only by hand

**Context.** M2.1c asks for a CI job that runs `cargo xtask validate` against the stored references
and is green on three OSes, and for a separate workflow, triggered only by a person, that
regenerates the references and whose output is a diff to review, never a commit. `cargo test`
already reran every case and compared the result with the committed report, but inside a test
that nobody would read as "the validation suite ran", and `cargo xtask validate` itself rewrote
the report rather than checking it. hpr is bit-identical on one platform, not across three
(ADR-015), so the committed report cannot be compared byte for byte on Windows or Linux. The
references come from RocketPy, which needs the oracle environment in the gitignored `refs/` (uv,
RocketPy 1.13.0 and its dependencies); no CI job had one. Loft lesson L76 is why a reference must
never move on its own: Loft's advice to regenerate a reference whenever a check failed let the
reference follow Loft's own drag.

**Decision.**

- **`cargo xtask validate --check`** runs every locked case, writes nothing, and fails unless every
  scored metric passes and the run reproduces the committed `latest.md` and `latest.json`. The
  comparison is `Report::reproduces`, moved from the report test into `hpr-validate` so the test
  and the command share one definition: everything that cannot differ by platform (cases, sources,
  tolerances, verdicts, notes, gaps but for the Mach number the integrator narrowed onto) exactly,
  and hpr's and the reference's value at full precision from the JSON, to 2e-6 or 1e-7 of
  itself, whichever is larger. `latest.md` must be `latest.json`'s rendering exactly, since both
  come from one platform. The old test compared the rendered Markdown instead, where a printed
  percentage's last digit can round the other way on another platform.
  `--check` with `--fast` is refused: a partial run cannot check the whole suite's record.
- **A `validate` job** in `ci.yml` runs it on `ubuntu-latest`, `macos-latest` and
  `windows-latest`, with no oracle and no network; the Pages deploy waits for it.
- **`scripts/regenerate-references.sh`** runs the chain behind the references the harness reads, in
  order: `rocket_mass.py`, `cargo xtask designs`, `recovery.py`, `flight.py`, then
  `cargo xtask validate --check`. Each generator writes to a temporary file, so a failure leaves
  the committed fixture alone. The report is rewritten only when the check fails: the committed
  one holds macOS digits, and a Linux run would otherwise rewrite it on every run for last-digit
  rounding alone. A fixture that moved at all changes the hash the report records, so the report
  is then rewritten. It is written even when a metric fails, so the diff shows what moved, and the
  script then exits non-zero.
- **The *Regenerate references* workflow** (`regenerate-references.yml`) runs the script on
  `macos-latest`, an arm64 Mac like the one the committed references came from, so that an empty
  diff means something, and only on `workflow_dispatch`. Its token has `contents: read` and the checkout keeps no
  credentials, so it cannot push. It uploads `references.diff` and a summary as an artifact, and
  collects them even when the script fails.
- The script covers the harness's references and the design fixture they are built from. The
  other oracles' fixtures (atmosphere, geodesy, shapes, walls, motors, ThrustCurve) feed unit
  tests, not the harness; they keep their own commands.

**Alternatives.**

- Diffing the report `cargo xtask validate` writes with `git diff --exit-code`: exact bytes fail
  on the platforms that round the last digits differently.
- Only the existing test: it runs the same check, but a reader of CI cannot see the suite ran.
- A workflow that opens a PR or commits: that is L76 automated. A person commits the diff, in a PR
  that says why the reference moved.
- Running the oracles in CI on every PR: RocketPy's environment takes minutes to build, and a
  stored reference is the point of ADR-015.

**Consequences.**

- A change that moves a validation number cannot merge without the report that says so, on any
  of the three OSes.
- The workflow could not run before it was on `main` (GitHub only dispatches workflows the default
  branch has). The script ran locally first, on macOS: in 41 s it reproduced every committed fixture
  and the report byte for byte.
- M2.1c2's predicted-mode reference joins the chain when it lands.
