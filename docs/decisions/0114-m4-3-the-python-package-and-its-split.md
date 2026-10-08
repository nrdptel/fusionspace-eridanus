# ADR-114: M4.3: the Python package, and its split (2026-09-29)

- **Status:** accepted
- **Summary:** M4.3 split a to c; the Python package wraps the builder, SI names, flies on construction, one abi3 wheel per OS

**Context.** M4.3 asks for Python bindings (PyO3 abi3 wheels built by maturin) with NumPy outputs,
a RocketPy-like API, Python callbacks for custom models, a pytest suite and wheels built in CI on
three operating systems; it is done when pytest passes in CI and a notebook-style example
reproduces a RocketPy example flight within M2.1's 3%. That is more than one pull request. The one
RocketPy example the validation suite flies with the same drag (Calisto) needs a drag table the
builder takes only as a model (`FlightBuilder::drag_model`), and callbacks need a Python function
behind the `DragModel` and `Wind` traits.

**Decision.**

1. **Split.** M4.3a: the package over the builder, its pytest suite, and wheels built and tested
   in CI on Linux, macOS and Windows. M4.3b: a drag table on a flight, and RocketPy's Calisto
   reproduced from Python by a notebook-style example within M2.1's 3%. M4.3c: drag and wind models
   written as Python functions. Each has its own *done when* in `ROADMAP.md`; the parent's two
   bullets are b's and a's.
2. **What it wraps.** The `hpr` builder (ADR-103), not the crates below it: `Environment`,
   `Motor`, `Rocket` and `Flight`, as the Rust builder has them, with `Rocket.from_file` reading a
   design as `hpr sim` does (`.hpr`, `.hprz`, `.ork`, a rocket's JSON): the same configuration
   chosen, the same refusals of one that stages under power or doesn't fly as written, and its
   notes kept in `Rocket.notes`. No physics is added, so the
   bindings' tests hold a Python flight to the Rust example's printed output rather than to a
   reference of their own.
3. **"RocketPy-like."** RocketPy's shape, not its names: the same four objects, a `Flight` that
   flies as soon as it is made, time series as arrays, parachutes by `cd_s`. Names keep the Rust
   API's units (`apogee_m`, `wind_from_deg`), as the project's rules ask of every field, since a
   Python user reads the same guide. Choices among several things are strings, read through the Rust
   types' own `serde` names where they have them, so the two can't drift.
4. **Records as dictionaries.** A flight's summary, landing, events, mass properties and margin
   cross as the objects `json.loads` makes of the Rust types' JSON: one code path, every field.
   The recording crosses as read-only NumPy arrays, one per column, copied from the recorder.
5. **Build.** PyO3 0.29 with `abi3-py310`: one wheel per operating system for CPython 3.10 and
   later (3.9 is past its end of life). NumPy through the `numpy` crate (BSD-2-Clause). No
   `extension-module` feature: maturin links the module itself, and the feature would break
   linking under `cargo test --all-features`. The crate's Rust test binary is off (`test =
   false`); its tests are pytest's, run on the built wheel by `scripts/python-tests.sh` in CI's
   `python` job on 3.10 and 3.13, and in the gate's optional `python` step. With no test binary,
   `cargo test` doesn't compile the crate; clippy and rustdoc check it on Linux, and the `python`
   job compiles it on each OS with warnings as errors.
6. **Not published.** The distribution name is `hpr-sim`; PyPI is Neer's call
   ([rule 7](../../CONTRIBUTING.md#7-releases-are-the-maintainers)). CI builds and tests each OS's
   wheel but keeps none: a wheel handed out, even as a CI artifact, is a binary distribution, and
   needs the licence texts of what it links first.

**Consequences.** A builder change shows up in Python only when a binding exposes it. The
guide's [Python page](../python.md) runs in the tests, block by block, against its printed output.
A design read from a file flies without its stored parachutes and separations, as the builder's
`Rocket::from_design` does; `Rocket.notes` says what isn't flown. Type stubs (`.pyi`) are not
written yet, so editors see the classes' docstrings but not their signatures' types. Before a
wheel is handed out anywhere, it needs the licence texts of what it links (rust-numpy's
BSD-2-Clause asks for its notice in binary distributions).
