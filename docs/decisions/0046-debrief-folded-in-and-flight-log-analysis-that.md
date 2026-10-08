# ADR-046: Debrief folded in, and flight-log analysis that stands without the simulator (2026-09-20)

- **Status:** accepted
- **Summary:** Debrief folded in, and flight-log analysis that stands without the simulator

**Context.** Debrief (`nrdptel/fusionspace-debrief`) is the owner's own MIT-licensed browser
flight-log analyzer, now being sunset for the same reason Loft was: the web application became the
product and the library under it did not. What it knows, though, is real and expensive to
rediscover — ten logger families parsed byte by byte, a catalogue of readings with the method and
caveats of each written up against published sources, and a corpus of real logs
(`nrdptel/debrief-fixtures`, private) that those parsers were proved on.

hpr-sim already planned to read flight logs: Phase 5 (M7.1 to M7.4) covers importers,
reconstruction, parameter identification and fault diagnosis, and the vision lists flight
forensics. So the use case is not new. What is new is the owner's requirement that it also be
usable **on its own**: a user who flew a rocket, has a log, has no design file and does not want to
simulate anything should be able to use this project as a flight analyzer and nothing else.

The crate map already had `hpr-flightdata`, and it depended on `hpr-sim`. The crate is still an
empty skeleton, so the dependency had cost nothing yet — but it is exactly the edge that would have
made a standalone analyzer impossible, and it would have been expensive to remove once M7.1 had
filled the crate.

**Decision.**

- **Debrief's use case is folded into hpr-sim**, as prior art for Phase 5 rather than as a new
  direction. The repository is mirrored into the gitignored `refs/fusionspace-debrief` through the
  reference lock, the private corpus into `refs/debrief-fixtures`, and both have rows in
  `THIRD-PARTY-NOTICES.md`. What comes across is the format knowledge, the reading methods and
  their citations, and the corpus. What does not is the web application: the Next.js app, its
  components and its deployment stay behind. A user interface waits for Phase 7, as it already did.
- **`hpr-flightdata` must not depend on `hpr-sim`.** It depends on `hpr-core` and `hpr-atmos`
  instead — an atmosphere is needed because Mach number, dynamic pressure and pressure altitude are
  meaningless without one. The rule is declared in the crate's own manifest, where the dependency
  would be added, and enforced by `cargo xtask wasm-check`:

  ```toml
  [package.metadata.hpr]
  forbids = ["hpr-sim"]
  ```

  The check walks the workspace graph, not just direct dependencies, and fails with the path it
  found (`hpr-flightdata -> hpr-analysis -> hpr-sim`), because the way this rule breaks is a
  forbidden crate arriving through a helper that looked harmless. It follows normal, build and
  target-specific dependencies, and an optional one whose feature is off, since a crate that stands
  on its own must do so in every configuration. **Dev-dependencies are excluded**, because they
  never reach anyone who depends on the crate: a test in `hpr-flightdata` may fly a simulated
  flight and compare it with a parsed one. The mechanism is general, and it reads outwards — a
  crate names what **it** must not reach, not who may not reach it.
- **`hpr-forensics` is added to the crate map**, depending on `hpr-flightdata`, `hpr-sim` and
  `hpr-analysis`. It is the only place a reading taken from a log and a number the simulator
  produced meet.
- **Phase 5 is re-cut along that boundary.** M7.1 (importers) and M7.2 (readings, reconstruction
  and ghost data) are analyzer-only and must work with no design file present; sim-versus-real
  residuals move out of M7.2 and into M7.3, which with M7.4 lives in `hpr-forensics`. No milestone
  ids change.
- **Provenance travels with a reading.** Every reading says whether it was measured by an
  instrument, derived from what was measured, or clipped because the sensor saturated; a reading
  the log cannot support is withheld with a reason rather than printed as a number. Two recordings
  of one flight are read side by side as independent measurements and never averaged into one.
  M7.2's *done when* holds these, and M7.3's says a reading keeps its provenance where it meets a
  simulated number.
- **`hpr analyze <log>` is added to M4.2's command list**, taking no design and running no
  simulation, so the standalone use has a way in that is not "write a program".
- **What was mirrored is written up now, not when M7.1 starts**, in
  `docs/research/debrief-log-formats.md` and `docs/research/debrief-flight-readings.md`. The
  knowledge is perishable once nobody touches those repositories again.
- **The clean room is drawn inside the mirror, not around it.** Debrief's `lib/`, its tests and its
  public fixtures may be ported. Its `COMPETITION.md` may not: two of its rows carry OpenRocket's
  simulation-status vocabulary and behaviour read out of named GPL-3 Java files, with links, which
  [rule 3](../../CONTRIBUTING.md#3-clean-room) does not allow as an input to this project. That
  exclusion is recorded in the formats note and applies to M3.1 as much as to Phase 5.

**Consequences.**

- The layering is enforced from today, while the crate is empty, rather than argued about later.
  The check was confirmed to fail on a direct `hpr-sim` dependency and on one reached through
  `hpr-analysis`, and to pass once both were removed.
- Phase 5's documentation and *done when* clauses roughly double: each analyzer milestone now has
  to show it works with nothing but a log. That is the cost of the promise.
- The accuracy story forks. Sim-versus-reference error and reading-extraction error are different
  claims measured against different references, and the validation report will have to keep them
  apart rather than average them into one number.
- Debrief drew a hard line — a measurement instrument, never a predictor — and merging it into a
  simulator is what could erase it. The layering rule, the provenance fields and the separate
  forensics crate are that line rewritten as structure rather than intent.
- **The `.ork` parser's provenance was checked, not assumed, because the owner did not answer the
  question before the work started.** Its header states it was written from OpenRocket's published
  file-format page, which names the ten `flightdata` attributes, and that the `status` vocabulary
  was deliberately left alone because the page defers it to the GPL-3 reference implementation; the
  code bears that out, carrying `status` verbatim and never branching on it. Two things the header
  gets wrong about itself, found by reading the code beside it: the `<databranch>` series **is**
  read, by exact column name, which a later change added — from the file's own `types=` header, so
  the clean room still holds — and only one of the ten attributes' units is proved rather than
  inferred. Porting it is allowed; repeating its header's claim verbatim is not.
- OpenRocket's own example design is GPL-3, which is why Debrief ships no public `.ork` fixture.
  hpr-sim may not vendor one either, and M3.1 will need its own sample designs.
- The corpus is other people's flight logs. It is fetched, never committed, and only counts, error
  statistics and anonymised case ids are published from it, exactly as the Loft fixtures are. The
  twelve fixtures Debrief ships publicly in its own repository are the ones that may appear in
  examples and doctests.

**Not chosen: start Phase 5 now.** The knowledge is perishable, so it is mirrored and written up
now, but M1.8's aerodynamics is mid-flight and the roadmap order holds. Whether M7.1 should move
ahead of some of Phase 3 and 4 — real flights are what validation ultimately wants to compare
against — is left open rather than decided here.

**Not chosen: port the TypeScript.** Around 3 MB of it exists, and it is the owner's own MIT code,
so it may be ported freely with a note. But a browser analyzer's code is not a Rust library's
code; what is worth carrying is the format knowledge and the methods, which the research notes
record, not the implementation.
