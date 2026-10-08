# ADR-035: Drop the orhelper dependency; how M2.2 drives OpenRocket is decided when M2.2 starts (2026-09-19)

- **Status:** accepted
- **Summary:** Drop the orhelper dependency; how M2.2 drives OpenRocket is decided when M2.2 starts

**Context.** `orhelper` is a thin Python wrapper that starts a JVM with OpenRocket's jar on the
classpath and gives Python access to it. ADR-002 added it to `validation/oracles/pyproject.toml`
and had `refs doctor` check that it imports. It was never used: nothing in the repository imports
it, and no oracle script exists yet.

Checking the installed wheel rather than trusting the metadata: `orhelper` 0.1.5 ships the stock
**GPLv2** text, and its PyPI metadata carries no license field at all. Its `.py` files have no
licence headers, so whether the grant is "version 2 only" or "version 2 or later" is unstated.
Its own code is 555 lines, 91 of them mirroring OpenRocket's enums. It is pinned to a fork commit
because the PyPI release predates OpenRocket 24.12's `info.openrocket` packages.

Copyleft obligations attach on distribution. This repository is public, so `validation/oracles/*.py`
is distributed; a script that imported orhelper would raise the question of a combined work. The
Rust crates never touch it, run in a different process, and are unaffected either way.
[Rule 3](../../CONTRIBUTING.md#3-clean-room) permits "Running GPL tools (the OpenRocket jar, via
JPype/orhelper) as external oracles", so importing it was allowed; the question was whether to.

**Decision.** Drop the dependency now. Nothing imports it, so removing it costs nothing and stops
GPL code being installed into `refs/venv` for no purpose. `refs doctor` checks only `jpype` for
the OpenRocket oracle, which is what its smoke test already used.

How M2.2 actually drives the jar is deliberately left open, to be decided with the evidence in
hand: JPype directly, or the jar as a subprocess. Note that JPype loads the JVM **into the Python
process**, so driving OpenRocket without orhelper still puts GPL-3.0 classes in that process.
Dropping orhelper removes the GPL-2.0 Python layer, not all contact with copyleft code. Only the
subprocess route isolates properly, and whether the jar has a usable command-line interface is
unverified: launching it opened the GUI.

**Consequences.**

- `uv.lock` loses one package, and the environment no longer installs any GPL-licensed Python.
- Whoever writes M2.2's oracle reimplements what orhelper wrapped, or takes the subprocess route.
  555 lines is the upper bound on the first, most of it thin glue and enum mirroring.
- The project's rules still name orhelper as permitted. That permission is unchanged; this only
  records that the project is not taking it up for now.
- The licence of the fork was never in doubt, but its scope was under-specified. If M2.2 revisits
  orhelper, settle "v2 only" against "v2 or later" with the fork's maintainers first.
