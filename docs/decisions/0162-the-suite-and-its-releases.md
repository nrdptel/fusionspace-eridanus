# ADR-162: The suite and its releases: what ships when, and the order of the new product lines (2026-10-04)

- **Status:** accepted; §1 to §4 amended by [ADR-163, the second pass](0163-the-second-pass-products-and-priorities.md); §2's checklist gains each release's product guide from 0.2 on, by [ADR-196, the product guides](0196-the-2026-10-07-product-guides.md); §1's names: the project is Eridanus, the program keeps `hpr-sim`, by [ADR-198, project Eridanus](0198-the-2026-10-07-project-eridanus.md); §1's product names are FusionSpace HPR's, by [ADR-199, FusionSpace HPR](0199-the-2026-10-07-fusionspace-hpr.md)
- **Summary:** its release order and product list are amended by ADR-163, the second pass. Neer's 2026-10-04 ideas: hpr-sim becomes a suite of six products built in one workspace and joined by shared formats (simulator, flight analyzer, app, motor designer, recovery designer, avionics). Releases come at product boundaries: 0.1 the simulator once M4.5 and M1.14a are met, 0.2 accuracy, 0.3 the flight analyzer, 0.4 competitions, 1.0 the first app. The UI phase no longer waits for every library phase. COTS solids stay the only scope until 1.0. After it, ghost replay and the web app first, then the new lines one at a time: experimental solid motors, then custom parachutes, then avionics, then hybrids and liquids. Each line gets its own guardrails

Each milestone label (such as M4.5) has a row of plain words in
[the table of every milestone](../decisions-and-roadmap.md#every-milestone); terms such as C_D S
(drag area), c* (characteristic velocity) and GNSS (satellite positioning) are defined in the
research notes linked below.

**Context.** On 2026-10-04 Neer set the goal of hybrids, liquids and other propulsion, of designing
motors in hpr (experimental solids especially), custom parachutes, and custom flight computers
and GPS trackers. He sees them as a suite of products developed together and
compatible with one another. He also asked that the work be prioritised, and that a proper
release not wait for everything ([VISION](../VISION.md), 2026-10-04).

Where things stood that day:

- **Nothing has ever been released.** Every crate is at version 0.1.0 with `publish = false`.
  CI builds the Python wheel on all three operating systems, then discards it. There is no
  release workflow, changelog, or licence bundle for binaries. Publishing needs one more thing:
  [ADR-114, wheels and their licence texts][adr-114] asks that a handed-out wheel carry the licence
  texts of what it links.
- **The library is broad already.** It has a 6-DOF engine validated against RocketPy, OpenRocket
  and real flights, `.ork` import and export, the hpr format, the CLI, Python bindings, weather,
  motor stock, Monte Carlo and optimization ([roadmap archive](../roadmap-done.md)).
- **The roadmap kept every user-facing app behind every library phase.** Phase 7 (the UI) ran
  "only after the phases above". That put formats, forensics, the design assistant and CAD
  interop ahead of any app.
- **Peers released early.** RocketPy's first PyPI upload, 0.9.8 on 2021-08-24, was solid motors
  only. It reached 1.0 two years later, adding liquids and hybrids
  ([PyPI](https://pypi.org/pypi/rocketpy/json)). OpenRocket shipped 0.9.0 in 2009 and 1.0 ten
  months later ([release notes](https://openrocket.info/release_notes.html)).
- **Research for the new lines** is in three notes: [propulsion](../research/suite-propulsion.md),
  [parachutes](../research/suite-parachutes.md) and [avionics](../research/suite-avionics.md).
  Each has public sources, a runnable or permissive oracle, and a first increment that can be
  checked. The lines differ in four ways:
  - **validation data:** thin for amateur solids and liquids;
  - **safety:** case burst, opening loads and pyro firing;
  - **law:** radio rules and export control;
  - **hands:** whether hardware has to be built and flown by Neer.

**Decision.**

1. **One suite, six products, one workspace.** The products are:
   - the **simulator** (library, CLI, Python);
   - the **flight analyzer** (M7.1, M7.2: usable alone, [ADR-046][adr-046]);
   - the **app** (M9: desktop, web, mobile);
   - the **motor designer** (M11.1 to M11.5);
   - the **recovery designer** (M12.1, M12.2);
   - **avionics** (M13.1 to M13.4: ground station, GPS tracker, flight computer, hardware).

   They share four things:
   - **The hpr design format** gains motor and canopy documents. A designed motor yields the
     thrust curve and mass model the simulator flies. A designed canopy yields the C_D S and
     opening model it flies.
   - **The canonical flight record from M7.1** is what the tracker and flight computer write. The
     analyzer reads it without conversion.
   - **One `no_std`, no-heap crate** holds the estimator, the atmosphere and the deployment logic.
     The firmware runs it on the board and the simulator runs it in the loop.
   - **One documentation site and one version number.**

   Firmware lives in this workspace. Hardware design files go under `hardware/` when M13.4, the
   hardware, starts; their licence is Neer's call (CERN-OHL-P v2 proposed). The names
   stay `hpr-sim` and `hpr` until Neer picks product names.
2. **A release train at product boundaries.** Each release is a roadmap milestone, M10.1 to M10.5.
   It is met when the release checklist below holds at a commit on `main`, and its release
   steps are written in [`docs/releasing.md`](../releasing.md). Publishing, tagging and setting up accounts
   stay Neer's ([rule 7](../../CONTRIBUTING.md#7-releases-are-the-maintainers)). One version covers
   every crate, the CLI and the Python package. Each 0.x minor may break the API; a patch release
   only carries a `P-critical` fix.
   - **0.1, the simulator** (M10.1): after M4.5 and M1.14a. The library crates, CLI archives for
     the three operating systems, and the Python wheels and source package. Not FFI, WASM or the
     validation harness. M10.1 also builds the release workflow and the licence bundle.
   - **0.2, accuracy** (M10.2): M2.3c (the corpus flights with logs), M6.2e, and M1.14b to
     M1.14g, the core band's increments, each met or ended by [ADR-143][adr-143]'s stop rule with
     its gaps in an ADR. The extended band (M1.14h) is not a condition.
   - **0.3, the flight analyzer** (M10.3): M7.1 and M7.2.
   - **0.4, competitions** (M10.4): M6.3 challenges and M6.4 airbrakes.
   - **1.0, the app** (M10.5): M4.4 (WASM), M8.2 (the edit model), M9.0 and M9.1, and Neer has
     used it (the UI rule, [ADR-144][adr-144]).

   **The release checklist:**
   - every artifact builds and passes a smoke test on all three operating systems, from a CI
     dispatch;
   - publishing jobs wait on a `release` environment that only Neer can approve;
   - `cargo publish --dry-run` passes for each published crate, and every other crate says
     `publish = false`;
   - each archive and wheel carries both licences, `THIRD-PARTY-NOTICES.md` and the generated
     licence texts of its dependencies, which meets ADR-114;
   - `CHANGELOG.md` has the version's entry: what works, how far to trust it, the known gaps;
   - the README and the getting-started page show how to install it;
   - no `P-critical` issue is open, the gate is green, and the validation report matches the
     commit;
   - every open `P-critical` or `env-core` issue whose error lies on a flattering side, as
     [ADR-144][adr-144] §6 defines it (a stability margin or flutter speed too high, an apogee too
     low against a waiver ceiling, a charge too small), is listed by number in the release's PR
     and in the CHANGELOG's known gaps, and a flight that reaches its regime prints a warning
     naming it. For 0.1 that includes the slender-body fallbacks that overstate supersonic
     stability (#87, #120, #121), the supersonic drag gap (#222) and the high margins on two private
     designs (#172); the rule, not this list, is the condition.

   ADR-144 §6's rule that a known wrong number in shipped physics blocks a new user-facing
   surface still holds. A library release adds no surface, so the warning rule above governs it;
   the app (1.0) is a new surface, and that rule binds it.
3. **The queue to 1.0** (amends [ADR-144][adr-144] §2's order, "M6.3 onward in file order"):
   1. M0.5 (blocked on M0.5i);
   2. M4.5;
   3. M1.14a;
   4. M10.1;
   5. M2.3c, the corpus flights with logs, which release 0.2's accuracy claim needs;
   6. the rest of M1.14;
   7. M6.2e;
   8. M10.2;
   9. M7.1 and M7.2;
   10. M10.3;
   11. M6.3 and M6.4;
   12. M10.4;
   13. M4.4, M8.2, M9.0 and M9.1;
   14. M10.5.

   Phase 7's "only after the phases above" is replaced by "after release 0.4". Formats (M3.4 to
   M3.6), forensics (M7.3, M7.4), canards (M6.5), the design assistant (M8.1) and CAD interop
   (M8.3) move behind 1.0. They are not dropped. Each of them can still be queued earlier when
   a release needs it.
4. **After 1.0, one new line at a time.** The queue carries items 1 to 4 below, so the
   file-order fallback doesn't pick a moved milestone first. The order is provisional. It is revisited at the
   planning review after Neer has used 1.0:
   1. **ghost replay and the web app:** M9.2 and M9.3, from his original brief;
   2. **the motor designer's first part (M11.1, M11.2):** experimental solids, the line he marked
      "especially";
   3. **the recovery designer (M12.1, M12.2):** small and safety-relevant, with public sources and an MIT
      oracle;
   4. **avionics in software (M13.1 to M13.3):** ground station, then tracker, then flight
      computer, in rising order of risk;
   5. **then:** mobile (M9.4), the moved library milestones, hybrids and liquids (M11.3 to M11.5),
      avionics hardware (M13.4) and accounts (M9.5).

   Liquids come last because their public validation data is the thinnest and their export-control
   reading is the least certain.

   At most 3 user-facing surfaces are in progress at once ([ADR-144][adr-144]), so a new line
   starts only when an earlier one has shipped its first release.
5. **Scope.** COTS solid motors remain the only propulsion until 1.0
   ([rule 6](../../CONTRIBUTING.md#6-scope-until-10)). Before then, the motor model may gain
   extension points, such as mass held in parts at their own positions, but nothing that flies a
   hybrid or liquid.
6. **Guardrails for the new lines,** on top of [ADR-144][adr-144]'s:
   - **Motor designer:**
     - A propellant is a set of measured properties: burn-rate law, density, c*, ratio of specific
       heats. hpr never gives a formulation, a mixing method or a manufacturing step.
     - A design flies on its nominal ballistics. The maximum expected operating pressure errs
       high and the case burst margin errs low, each pinned by a test and a mutation probe.
     - Every design says "static test first".
   - **Recovery designer:**
     - Descent rate comes from the low end of the C_D range, and drift from the high end.
     - Opening load errs high, from the high end of C_D S.
     - A gore template prints with a scale bar.
     - Every load says "drop test first".
   - **Avionics:**
     - Software is checked against simulated and real logs before any board exists.
     - No firmware is ever called flight-ready.
     - A hardware increment is done only when Neer has built and bench-tested it.
     - No code lifts a GNSS receiver's limits or guides a rocket to a point.
     - Radios default to licence-free bands within their published power limits.
   - **Law:** before M11.5, hybrid and liquid design, and before any hardware is sold, Neer makes the
     export-control and FCC calls ([avionics note](../research/suite-avionics.md),
     [propulsion note](../research/suite-propulsion.md)). Each goes to the maintainer when its
     milestone is next.

**Alternatives considered.**

- **Releasing nothing until the app (1.0).** Rejected. The library is the first product (VISION
  V1, V2). Peers shipped a library first, and an early release starts the CHANGELOG, the version
  policy and the release workflow while they are cheap.
- **Starting the new lines now, in parallel.** Rejected. It breaks the 3-surface cap. It also
  competes with accuracy work that Neer judged worth the time ([ADR-143][adr-143]). And it would
  start safety-relevant tools before the core band's flattering-side errors are warned about.
- **Recovery designer before motor designer.** It is smaller. But Neer singled out experimental
  solids, and openMotor, a free motor designer released since 2019 ([releases](https://github.com/reilleya/openMotor/releases)), shows such a tool has users.
- **A separate repository per product.** Rejected for software: shared crates and one CI keep the
  products compatible by construction. Hardware may still move to its own repository, which
  would be Neer's call when M13.4 starts.

**Consequences.**

- `ROADMAP.md` gains Phase 8 (releases, M10.1 to M10.5) and Phase 9 (the new lines, M11.1 to
  M13.4), and a queue running to 1.0.
- `VISION.md` gains V34 to V39, and its *Not now* list changes.
- `ARCHITECTURE.md` gains the planned crates.
- [Rule 6](../../CONTRIBUTING.md#6-scope-until-10) and the project's guardrails name 1.0 as the
  scope line.
- The [ideas backlog](../research/ideas.md) marks prebuilt binaries as promoted to M10.1.

[adr-046]: 0046-debrief-folded-in-and-flight-log-analysis-that.md
[adr-114]: 0114-m4-3-the-python-package-and-its-split.md
[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
