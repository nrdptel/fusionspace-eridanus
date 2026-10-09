# Roadmap

Open work only. What is done is in [the archive](roadmap-done.md), moved there with its text
unchanged by `cargo xtask records`.

**Rules:**

- **The queue sets the order.** The current milestone is the first entry of the queue below that
  is open and not blocked; when the queue runs out, the first such entry in file order. An entry
  whose open increments are all blocked counts as blocked. Open `P-critical` issues come before
  any milestone (ADR-144 §4).
- A milestone is done only when every *done when* bullet is demonstrated by a command's output and
  CI is green on all three operating systems. Then check its box, run `cargo xtask records`,
  which moves it to the archive, and take it off the queue.
- A milestone too big to ship at once is split in place into `a`, `b`, `c`... increments, each with
  its own *done when*.
- Blocked milestones are marked `[blocked]` with a pointer to the reason.
- New milestones may be added (at the right position, with a *done when*). Existing *done when*
  bullets may be tightened but never loosened without an ADR. Milestones are never removed,
  renumbered or moved later in the order without an ADR, and an archived entry is never edited.
- A `Loft lessons:` line lists ids from `docs/research/loft-lessons.md`. A milestone that owns a
  lesson (listed first on its row) ships its named tests; `cargo test -p xtask` checks this once
  the milestone is checked off.
- Budgets, checked by `cargo test -p xtask`: this file 43,500 bytes, a line 120 characters, an
  entry 40 lines.

## Queue

The order of work after the `P-critical` issues (ADR-144 §2, amended by ADR-162 and ADR-163). Each
line is `N. M<id> title`, with an id from this file; `cargo test -p xtask` fails on any other list
line here, or on an id that is missing or done.

1. M0.9c2 One name, one design: units and trust
2. M0.9c3 One name, one design: provenance, errors and the plot
3. M0.9d One name, one design: the site
4. M0.9e One name, one design: the words
5. M0.7a Rocketry explained: the format and four guides
6. M0.8a Product guides: the simulator
7. M7.1 Flight log importers
8. M7.2 Readings, reconstruction and ghost data
9. M7.3 A flight against its simulation
10. M10.2 Release 0.2: the flight analyzer
11. M2.3c2 Logged traces
12. M1.14 Accuracy inside the envelope
13. M7.4 Fault diagnosis
14. M10.3 Release 0.3: accuracy and diagnosis
15. M6.3 Challenge specs and presets
16. M6.4 Airbrakes
17. M6.2e Robust mode
18. M6.6 Submission packs
19. M6.7 Ejection charges
20. M3.5 RASAero `.CDX1` import/export
21. M3.6 RocketPy interop
22. M6.8 Field kit: checklists, the settings check and the ground-test log
23. M0.7b Rocketry explained: recovery, wind and motors
24. M10.4 Release 0.4: the competition kit
25. M4.4 C ABI and WASM
26. M9.0 UI architecture ADR plus a spike
27. M9.2 3D flight replay with a ghost
28. M10.5 Release 0.5: the app preview
29. M0.7c Rocketry explained: live figures
30. M5.6a A catalog of hpr's own: format, search and a parts list
31. M8.2 Edit model for UIs
32. M9.1 Desktop app shell
33. M5.6b Parachutes and recovery hardware
34. M8.1 Design assistant
35. M5.6c Motor hardware and rail buttons
36. M5.6d More makers and electronics
37. M3.4 RockSim `.rkt` import/export
38. M9.3 Web PWA
39. M10.6 Release 1.0: the app
40. M0.7d Rocketry explained: the rest of the hobby
41. M11.1 A motor of your own
42. M11.2 Experimental solids
43. M12.1 Parachute gores
44. M12.2 Opening loads
45. M9.4 Mobile
46. M9.6 The four web tools, rebuilt
47. M6.5 Roll control: tail-fin tabs and canards
48. M13.3 Flight computer logic
49. M13.1 Ground station
50. M13.2 GPS tracker logic
51. M9.7 Field equipment and frequencies

## Phase 0: Foundations

- [ ] **M0.7 Rocketry explained** (ADR-195; Neer, 2026-10-07; VISION V49, V50). Guides to
  high-power rocketry on the docs site, illustrated, then live ([research](research/guides.md)).
  Each opens with its scope, reader and how far to trust it; words before equations; a worked
  example a CI-run program prints; one idea per figure, in the drawing language, made from hpr's
  output by a command CI reruns; uncertainty as a band, its flattering side marked. No verdicts;
  copyrighted and share-alike sources only cited; exam questions never answered.
  - [ ] **M0.7a The format and four guides:** stability (the center of pressure moving with Mach
    and angle of attack), fin flutter, drag through Mach 1, how far to trust a simulation.
    *Done when:* each meets the format and ADR-195 §7, read cold in a docs review and by Neer;
    every number in a guide's prose sits in a block `xtask cli --check` regenerates or is read
    from the committed report by a test, a mutation probe each; each flattering-side mark is tied
    to its model's direction test; an `xtask` check fails when a figure's text, marks, strokes or
    markers (transforms resolved, a conservative glyph width) leave its viewBox, and a headless
    page check fails when `scrollWidth > clientWidth` from 320 to 1,920 px in 40 px steps, each
    with a test that moves one label out and fails; a third fails on text over text, with a test
    that overlaps two labels; all three also run on the site's other pages and `--plot` (ADR-198).
    - [ ] **M0.7a2 The format and the trust guide** (how far to trust a simulation). *Done when:*
      the format is in `docs/writing.md`; the guide meets it, ADR-195 §7 and M0.7a's terms on
      numbers and marks, read cold in a docs review, and passes M0.7a1's checks.
    - [ ] **M0.7a3 Stability.** *Done when:* as M0.7a2.
    - [ ] **M0.7a4 Fin flutter.** *Done when:* as M0.7a2.
    - [ ] **M0.7a5 Drag through Mach 1.** *Done when:* as M0.7a2; Neer has read all four guides.
  - [ ] **M0.7b Recovery, wind and motors** (recovery as one timeline, weathercocking and rail
    exit, thrust curves and delays), after M6.8. *Done when:* as M0.7a; charges only as M6.7's
    ranges, "ground test first" ahead of them.
  - [ ] **M0.7c Live figures,** after release 0.5, on the app's plotting code. *Done when:* inputs
    in the text redraw each figure through `hpr-wasm`, offline, with the CLI's numbers (a test
    each); outside a model's range it greys out with the reason; scripts off keeps the static
    figure; M0.7a's two clipping checks run on the drawn figures at each input's least, default
    and largest value; Neer has used each.
  - [ ] **M0.7d The rest of the hobby,** after 1.0: certification, safety codes and waivers,
    construction, wiring, staging and air starts, linking out more than writing. *Done when:* as
    M0.7a; rule pages carry a source date, a stale-after date CI fails past, and "not legal
    advice".

- [ ] **M0.8 Product guides** (ADR-196; Neer, 2026-10-07; VISION V51). Each product gets a
  landing page and an illustrated tour ending on a public real flight, in M0.7's format and checks
  ([research](research/product-guides.md)). No picture is taken by hand: CLI output as terminal
  figures from `xtask cli --check`'s runs, plots from hpr, app screens from end-to-end tests at
  fixed device sizes, labels in captions; phone and watch captures also fail on crossing the
  device's safe area. From 0.2 on, each release's checklist needs its guide (Phase 8).
  - [ ] **M0.8a The simulator's guide** (library, CLI, Python), after M0.7a. *Done when:* its
    landing page and tour meet all of M0.7a's bullets and ADR-195 §7, read cold in a docs
    review and by Neer; the tour ends on a flight from `validation/reports/real-flights.md`,
    named by case id, its measured numbers read from that report by a test and shown beside
    the report's spread over every flight, no measured trace
    drawn without a license entry, ERA5 drawings with Copernicus's credit; the tour has CLI
    terminal figures and Python output, each regenerated by `xtask cli --check` or its docs
    test and failing when stale (a test each).
- [ ] **M0.9 One name, one design** (ADR-205; Neer, 2026-10-08). Every surface says FusionSpace
  HPR as ADR-199 §1 has it, and follows the pinned design system rule by rule, each checked.
  - [ ] **M0.9c The CLI, exports and plot to the design** (ADR-208): #377 to #382, in three steps,
    each fix held by a named test that fails on the defect its issue reproduces, and the audit's
    rows naming the tests; a rule declined has an ADR.
    - [ ] **M0.9c2 Units and trust** (#378, #379). *Done when:* #378 and #379 are closed, with
      every item they list, among them: feet in brackets wherever `hpr sim` and `hpr mc` print a
      height or speed, and on the plot's axes; a trust note with the committed report's spread on
      their text output and the plot.
    - [ ] **M0.9c3 Provenance, errors and the plot** (#380 to #382). *Done when:* #380 to #382
      are closed, with every item they list, among them: the motor catalog's as-of date in the
      text and the exports; errors that name the flag and the unit typed; the plot on the type
      and space scale.
  - [ ] **M0.9d The site to the design** (ADR-208). *Done when:* #383 to #385 are closed: no
    mdBook default the system doesn't draw (colors, radii, shadows, motion, icons, metas, print),
    a title block and sheets on every page, notes in the system's shape, and every committed SVG
    in token colors, each failing `xtask site`, `xtask figures` or a site test when broken.
  - [ ] **M0.9e The words to the design** (ADR-208). *Done when:* #386 and #387 are closed:
    `xtask spelling` fails on the British forms the audit found, a site check fails on a
    hyphen-minus for minus and on a number column not right-aligned, and the READMEs follow
    `writing.md`'s order with the version; the audit's rows name the checks.

## Phase 1: Physics core (the heart), with validation interleaved

- **M1.8** is done; its entry: [archive](roadmap-done.md#phase-1-physics-core-the-heart-with-validation-interleaved).
  - **M1.8e** is done; its entry: [archive](roadmap-done.md#phase-1-physics-core-the-heart-with-validation-interleaved).
    - [ ] [blocked] **M1.8e16 The blunt tip's handover, past 24°** (the rest of the old e13,
      ADR-044; the next free number, so the flare and the step keep theirs). Deferred above Mach 4
      with #108 (ADR-143); the vertical tip's error goes to M1.14d below Mach 2.5, M1.14h above.
      *Done when:* the vertical-tip switch is gone or measured again, fixtures and the guide moving
      together; and, ahead of that, issue #108 closed: a rule for the loading through a crossing
      whose answer settles as the nose is cut finer, on a body that crosses (the committed nose
      under a 30° cap at Mach 4.63), and that leaves TN 3527's printed ogives where they are.
- [ ] **M2.3 Real flights.** Split a to c (ADR-081); its bullets met by M2.3b, open for
  M2.3c (ADR-083).
  - Cases from the RocketPy flight data with their ERA5 environments, which needs a weather-file
    reader: a netCDF reader or a documented conversion.
  - Also the corpus flights that have logs.
  - Loft lessons: L83.
  *Done when:* at least 6 real flights are in the report, with apogee error and altitude-trace
  RMS; mean absolute apogee error is reported against the 5% target; each outlier has an
  explanation.
  - [ ] **M2.3c Corpus flights with logs.** The private designs that have a flight log,
    in their day's weather; *done when* each is in a report as anonymised statistics beside
    M2.3b's. Blocked until 2026-10-04 (ADR-083) for want of a pair; `hpr-sim-fixtures` has 89 in
    its tier A, with their days' weather (ADR-151). Split in two (ADR-163):
    - [ ] **M2.3c2 Logged traces.** *Done when:* each M2.3c1 flight whose log M7.1 reads has its
      altitude-trace RMS from liftoff to apogee, aligned at liftoff, in that report.
- [ ] **M1.14 Accuracy inside the envelope** (ADR-143 §6). The core band (Mach 0–2.5) first, then
  the extended band in M1.14h; angle of attack ≤ 15°, high angles M1.14e's. *Done when:* real
  flights meet the 5% mean apogee target, hpr is at least as accurate as OpenRocket on the same
  real flights, and M1.8's Cd bullet is met or its gap re-measured in an ADR.
  - [ ] **M1.14b References, Mach 1.5–2.5.** *Done when:* each in ADR-143 §6b reported or refused.
  - [ ] **M1.14c Transonic, Mach 0.8–1.2.** *Done when:* the stop rule ends it, gaps re-measured.
  - [ ] **M1.14d Supersonic, Mach 1.2–2.5.** First the switches that fall back to slender-body
    theory and overstate stability (#87, #120, #121, the vertical tip), then drag (#222) and M1.8e's
    body-alone misses. *Done when:* as M1.14c, with M1.8's Cd bullet met or re-measured.
  - [ ] **M1.14e Large angle of attack.** *Done when:* its cost measured; past 1%, a model built.
  - [ ] **M1.14f Audit's physics gaps.** *Done when:* each cited and tested, or measured negligible.
  - [ ] **M1.14g Figures.** *Done when:* accuracy and aero pages show CI-checked plots with errors.
  - [ ] **M1.14h Extended band, Mach 2.5–3.5,** after the core-band increments (ADR-143 §6h).
    *Done when:* each fixed or re-measured in an ADR: the four switches' errors at Mach 3 (CP aft,
    flattering); #108's Mach 2.90–2.95 step (sign not yet measured); M1.8e's Mach 2.96 body-alone
    row (+16.9% in the body-alone C_Nα; conservative on the finned rocket, CP forward).
## Phase 2: Library surfaces and interop

- [ ] **M5.6 A catalog of hpr's own** (ADR-197; Neer, 2026-10-07; VISION V52). Commercial parts
  beyond OpenRocket's catalog, which stays the base ([research](research/parts-catalog.md)).
  Every value carries its source, the date read and whether it was stated, measured or derived.
  Facts only: no vendor text or photos.
  - [ ] **M5.6a Format, search and a parts list,** after release 0.5. *Done when:* the reader
    refuses a value with no source or date (a test each); `hpr parts search` and `show` (and
    Python) find parts in both catalogs by maker, by number and by a description's words (a test
    each); a design's parts list names each catalog part by maker, number and quantity, with a
    link where the catalog has one, and lists the rest as custom with sizes; an imported `.ork`'s
    list names the parts its `<preset>` tags name.
  - [ ] **M5.6b Parachutes and recovery hardware,** before M8.1. *Done when:* each Cd is stored with
    its reference area, and one with none doesn't size a chute; a range's ends are compared on one
    area (Knacke's projected-to-nominal ratio, its type or a named nearest); descent and landing
    energy take the low C_D·S end, drift and opening load the high, a test and a mutation probe
    each; five or more parachutes from each of three makers.
  - [ ] **M5.6c Motor hardware and rail buttons.** *Done when:* a catalog case, retainer and buttons
    move mass and CG to hand-computed values; a catalog case or closure beside a motor file is
    refused by name unless that motor's mass is marked as excluding hardware (a test each path).
  - [ ] **M5.6d More makers and electronics.** *Done when:* a mass range states what it spans; a
    family's spread takes the part's mass from its geometry; a unit's tolerance draws uniformly in
    Monte Carlo, CG and inertia with it; the margin and descent rate report its worst end, a test
    pinning the direction.

## Phase 3: Uncertainty, optimization, challenges

- [ ] **M6.2 Optimization engine.**
  - Continuous and discrete design variables, including motor choice and catalog parts.
  - Constraints and single- or multi-objective goals.
  - Algorithms: CMA-ES, NSGA-II, Bayesian/EGO; robust (MC-in-the-loop) mode.
  *Done when:* benchmark functions converge to known optima within tolerance; a "hit 3,048 m"
  design problem is solved with the result validated by re-simulation. Split a to e (ADR-138).
  - [ ] **M6.2e Robust mode.** *Done when:* a Monte Carlo statistic is optimized with common random
    numbers, the winner checked by a fresh Monte Carlo run.
- [ ] **M6.3 Challenge specs and presets.**
  - A TOML/JSON challenge format: target apogee and scoring, impulse limits, stability
    min/max, rail-exit velocity, mass/length/diameter limits, budget (including live motor
    prices), in-stock-only, drift and landing limits, descent-rate and landing kinetic-energy
    limits, payload.
  - Presets (each cited, dated, and flagged "verify against current rules"): Spaceport America
    Cup/IREC-style, NASA Student Launch-style, TARC-style, and a generic target apogee.

  *Done when:* each preset is solved end to end from both the CLI and Python, with a Pareto report
  and a check that the winners satisfy every constraint by re-simulation.
- [ ] **M6.4 Airbrakes.** Added by Neer on 2026-09-18 (VISION V16).
  - Deployable drag surfaces: added drag as a function of deployment and Mach, from a table or a
    cited semi-empirical estimate; deployment rate limits.
  - A controller interface: a user's controller, sampled at a fixed rate, reads simulated sensors
    (seeded noise) and commands deployment. A reference apogee-targeting controller ships.
  - Drag only (ADR-193): every flap follows the one deployment command, with no pitch or yaw
    channel; a single flap out is a fault, not a command. The brakes stay retracted until burnout
    and retract on an abort, a power loss or a tilt past 30° (IREC §7.3, §7.4).

  *Done when:*
  - RocketPy's air-brakes example, flown with the same drag table and controller, matches RocketPy
    within 3% in apogee and in the deployment history.
  - A challenge spec (M6.3) can target an apogee using airbrakes.
  - The design check reports the stability margin with the brakes retracted and fully deployed,
    from rail exit to apogee, and warns when either falls below zero or below a minimum the user or
    a challenge spec sets (IREC's stable-when-inert rule; ADR-192); a test flags a design stable
    only with the brakes out.
  - With one flap stuck out, the check reports the trim angle of attack along the flight, from
    the flap's drag acting off the axis; a test and a mutation probe pin that it errs high.
  - Tests show the brakes stay retracted through boost and retract on each abort condition.
- [ ] **M6.5 Roll control: tail-fin tabs and canards.** Added by Neer on 2026-09-18 (VISION V16);
  re-scoped on 2026-10-07 (V45, ADR-193; [roll and drag control](research/roll-and-drag-control.md)).
  - Roll only: M6.4's interface gains one roll command, which a fixed mixer turns into equal and
    opposite deflections; no pitch or yaw channel. A stuck surface is a fault, not a command.
  - Tabs first: a hinged trailing-edge tab on a tail fin turns the fin like a cant of τ·δ over the
    span it covers, τ from Glauert's thin-airfoil flap effectiveness below Mach 0.7 and linear
    theory (τ = c_f/c) above Mach 1.25, with M1.8's roll damping. Mach 0.7–1.25, where tabs can
    reverse (RM L8E25) and buzz (RM L53I29), is warned as unvalidated. A tab whose rotation
    stiffness is unknown or below c·ω/V = 2.0 (NASA/TP-2006-212490) is warned of buzz.
  - Then canards: fixed canards with their downwash on the aft fins (NACA Report 1307); when
    deflected for roll, a warning where the tail's exposed span exceeds 0.75 of the canards'.
  - Design checks: stable with every surface neutral and with any one stuck at its limit (the trim
    angle reported); a stuck surface's steady roll rate against the pitch natural frequency (roll
    lock-in); neutral through boost and on an abort, a power loss or a tilt past 30°.
  - Steering to a target point, and pitch or yaw control, are out of scope (ADR-193).

  *Done when:*
  - The subsonic tab effectiveness reproduces Glauert's closed form at c_f/c = 0.1, 0.2 and 0.3
    to 1e-12, and the error against NASA TP-1353's tail-fin ailerons (Mach 1.6–2.86, Figure 18
    digitized by pixel) is published with its sign.
  - The canard–tail interference term's error against a public canard–tail wind-tunnel case (TP-2157
    or TM X-3070, picked in the milestone's ADR before running) is published with its sign, and a
    test fails with the term removed; the canard warning is silent at a span ratio of 0.75 and
    fires at 0.76.
  - A roll-rate hold flown on tabs follows the linearized closed-loop response to a step roll
    disturbance within 2% of the step at every sample.
  - Each check above has a test. The margin check (neutral and stuck), the stuck-surface trim, the
    lock-in check, the canard warning and both edges of the buzz band each have a test and a
    mutation probe pinning that they err toward warning.
- [ ] **M6.6 Submission packs** (ADR-163; VISION V29). *Done when:* from the CLI and Python, a
  public example design yields Student Launch's numbers (each section's landing kinetic energy,
  descent time, drift at 0, 5, 10, 15 and 20 mph) and IREC's prediction table (rail-exit speed,
  thrust to weight, least static margin, largest acceleration and speed, flutter speed, apogee),
  each matched by an independent hand calculation; each number checked against a limit comes
  from its unflattering side, pinned by a test and a mutation probe: kinetic energy from low C_D
  and high mass, descent time and drift from high C_D and the apogee bound's top, drift from
  apogee over the pad; flutter speed from a cited model inside its range, with top speed from the
  low-drag end; a declared apogee carries a two-sided 95%/95% Wilks bound, n printed, failed runs
  counted, the measured model error beside it; a ballast solver re-checks the margin and refuses
  a target it can't reach.
- [ ] **M6.7 Ejection charges** (ADR-163; Neer, 2026-10-03; VISION V25). *Done when:* the black
  powder mass for a bay's gross volume and a target gauge pressure, or the shear pins' load at the
  high end of their strength, follows a cited sizing with an efficiency factor from published
  ground tests; it errs high against that measured basis (a test and a mutation probe), prints the
  bulkhead load at the sized pressure, and says "ground test first". Fetter's 2025 measurements
  join that basis (ADR-194), each rule with a test and a mutation probe: a packed bay is sized to
  separate, never from the empty-bay pressure, and his TR-1 and VTS-1 fixtures as packed get at
  least 1.4 times (a margin) the charges that first separated them cleanly (1.0 g, 1.6 g); the bulkhead
  load is his model scaled to his largest shot (36.6 psi, 23.75 predicted, 0.5 g in 150.8 in³);
  at the deployment's highest simulated altitude, a packed charge grows by the packed peak's fall
  (3.58 to 0.9 psi), linear in ambient pressure between his 14.7 and 3.3 psia shots, the
  bulkhead load never falls, and above 20,000 ft no black-powder size is printed, sealed
  canisters or CO₂ named instead.
- [ ] **M6.8 Field kit: checklists, the settings check and the ground-test log** (ADR-194; Neer,
  2026-10-07; VISION V46–V48). Library, CLI and print
  ([field tools](research/suite-field-tools.md)).
  - A launch checklist and guide built from the design: steps chosen by recovery type,
    altimeters, tracker, staging and charge type, grouped in Student Launch's ten procedure
    areas, safety steps marked; rule-ordered steps cite their rule, and moving one warns.
  - The settings check: main altitudes, apogee delays, Mach lockouts, backup offsets and the motor
    delay, typed in or read from a documented source, against the Monte Carlo flight: ranges and
    the side each errs, no verdict.
  - The ground-test log: Fetter's columns and five-step outcome scale, in a schema'd file, keyed
    by bay and packing. M6.7 shows its spread, and never suggests a charge at or below a logged
    failure, nor under 1.4 times the smallest separation logged (Fetter's 40% margin).

  *Done when:*
  - On public examples (dual deploy with two altimeters and a tracker; two-stage), the checklist
    keeps Tripoli USC 7-8, 13-8 and 13-9's order (a test each, mutation-probed by a swap); a
    motor-ejection design gets no altimeter steps; each cited rule shows its source date, a
    stale-after date and "not legal advice" (a test each).
  - Each edge in [the note's settings table](research/suite-field-tools.md#the-settings-check) is
    checked against the Monte Carlo flight's one-sided 95%/95% Wilks bound on that side (at
    least 59 runs; runs and failed runs printed), and the backup rule against its rule, each
    firing just past its edge and silent just inside (a test and a mutation probe each).
  - A ground-test log round-trips through its schema; a property test and a mutation probe pin
    the log's two floors.
  - The docs site has a launch-day guide built from the same steps, and the example's printed
    checklist is snapshot-tested.
## Phase 4: More formats and embeddings

- [ ] **M3.4 RockSim `.rkt` import/export** (clean room, from the RockSim XML doc and samples).
  - Loft lessons: L69, L70, L71.
  *Done when:* the corpus `.rkt` files import, and the exports reopen in our importer with
  semantic equality.
- [ ] **M3.5 RASAero `.CDX1` import/export** (from samples only). Fix the Loft `<Location>` bug
  class.
  - Loft lessons: L72, L73, L74.
  *Done when:* the corpus `.CDX1` files import with overall length within 0.5% of the stated
  values.
- [ ] **M3.6 RocketPy interop.** Export a runnable RocketPy script and `.rpy`; import `.rpy` where
  feasible.

  *Done when:* exported corpus designs run in RocketPy (oracle venv) and agree with hpr within the
  M2.1 tolerances.
- [ ] **M4.4 C ABI and WASM.** `hpr-ffi` with a cbindgen header and a C example built in CI;
  `hpr-wasm` package with generated TS types and a Node test.

  *Done when:* the C example and the Node test run in CI and reproduce a reference flight.
## Phase 5: Flight data and forensics

M7.1 and M7.2 must work with no design file and no simulator, so that analyzing a flight stands on
its own (ADR-046): `hpr-flightdata` may not depend on `hpr-sim`, and `cargo xtask wasm-check`
enforces it. M7.3 and M7.4 compare a flight with a simulation of it and live in `hpr-forensics`.
The formats, reading methods and log corpus come from Debrief (`refs/fusionspace-debrief`), written
up in `docs/research/`.
- [ ] **M7.1 Flight log importers.**
  - AltOS (TeleMetrum/TeleMega/EasyMega, CSV and eeprom), Missile Works RRC3 and RFF, Eggtimer,
    Featherweight Raven/Blue Raven and Featherweight GPS, PerfectFlite, Entacore AIM,
    AltimeterCloud, spreadsheet exports, CATS.
  - The EuRoC/Juno CSV layouts; generic CSV with column mapping; unit detection.
  - One canonical flight record every importer maps into, carrying every sample the logger wrote.

  *Done when:* every sample file in refs imports, snapshot-tested, and the crate builds with no
  dependency on `hpr-sim` (`cargo xtask wasm-check`). Formats without samples are listed in the
  docs as needing samples.
- [ ] **M7.2 Readings, reconstruction and ghost data.**
  - The readings a flight gives (apogee, maximum velocity and acceleration, burnout, descent
    rates per phase, flight time), each carrying its provenance: measured, derived, or clipped
    when the sensor saturated. A reading the log cannot support is withheld with a reason.
  - RTS/Kalman smoothing fusing baro, accel and GNSS; liftoff detection and time alignment.
  - Two recordings of one flight read side by side, never averaged into one number; per-stage logs
    assembled onto one timeline.
  - A "ghost" data product: time-synced trajectories in a common frame, as JSON and CZML/glTF.

  *Done when:*
  - On synthetic data (a simulated flight plus a noise model), the smoother recovers the truth
    within the stated error, and real RocketPy flights produce ghost files.
  - Every reading names its method and its provenance, and the corpus has a case for each
    withheld reading.
  - A flight is read end to end with no design file present, from the library and from
    `hpr analyze` (M4.2's command).
- [ ] **M7.3 A flight against its simulation.** Sim-versus-real residuals, then fitting Cd scale,
  mass, motor impulse scale and wind to a log (Levenberg–Marquardt plus a Bayesian option with
  uncertainty). The first milestone in `hpr-forensics`.

  *Done when:* residuals are reported for a real flight against its simulation, synthetic-truth
  recovery is within tolerance, and real-flight fits are reported. A reading keeps the provenance
  M7.2 gave it wherever it meets a simulated number.
- [ ] **M7.4 Fault diagnosis.**
  - A hypothesis library with simulate-able fault models: motor under/over-performance, CATO or
    early burnout, high drag or damage, weathercocking, marginal stability/coning,
    early/late/no drogue, main failure, separation → ballistic, baro-port or ejection pressure
    artifacts, Mach baro error, fin flutter or fin loss, delay mismatch, airstart failure.
  - Rank the hypotheses by evidence; explain in plain language; suggest the data that would
    disambiguate.

  *Done when:*
  - A seeded synthetic benchmark with at least 200 faulty flights reaches top-1 accuracy ≥80%
    and top-3 ≥95%, reported.
  - At least 2 real anomalous flights are analyzed (if the data exists; otherwise note it).
## Phase 6: Design experience (library level)

- [ ] **M8.1 Design assistant.**
  - Templates: minimum diameter, 3FNC, dual deploy L1/L2/L3.
  - Automatic checks (stability window, motor fit, rail buttons, flutter margin, recovery sizing).
  - Auto-size parachutes to a target descent rate, at the range's fast end (ADR-197).
  - Suggestions with reasons.
  - Loft lessons: L97.
  *Done when:* every template simulates and passes its own checks, and each check has
  positive/negative tests.
- [ ] **M8.2 Edit model for UIs.** A command/undo model over the design tree, stable ids, change
  events, and cheap incremental re-simulation hooks.

  *Done when:* property tests show undo/redo is a round trip, and re-simulation after an edit is
  measured.
- [ ] **M8.3 CAD interop** (ADR-144 §9; VISION V33). Files only: FreeCAD and its Rocket Workbench
  are copyleft and never read. *Done when:* a part exported to FreeCAD and re-imported keeps its
  mass properties within a stated tolerance, and an imported fin or nose flies the same aero as its
  native equivalent.
  - [ ] **M8.3a Mesh export** (STL, 3MF, OBJ). *Done when:* any part, watertight, in millimeters.
  - [ ] **M8.3b Drawings** (SVG, DXF, PDF). *Done when:* fins, rings, nose profiles, tube cut lists.
  - [ ] **M8.3c FreeCAD script.** *Done when:* it rebuilds the design as a parametric feature tree.
  - [ ] **M8.3d STEP export.** *Done when:* B-rep solids through a permissively licensed kernel.
  - [ ] **M8.3e Mesh import.** *Done when:* exact mass properties; aero recognized, or flagged.
  - [ ] **M8.3f STEP import.** *Done when:* a permissive B-rep reader, or Neer's licensing call.
  - [ ] **M8.3g Printed parts** (ADR-191; Neer, 2026-10-06). *Done when:* a part exported for
    printing names its material's heat-deflection temperature at ISO 75's 1.80 MPa load from a
    cited datasheet; a fin printed upright, so that bending pulls its layers apart, warns; a
    printed fin's flutter speed takes a measured or cited shear modulus, says it is unvalidated,
    and has a test and a mutation probe pinning that it errs low.
## Phase 7: UI, 3D, web, mobile (after release 0.4, ADR-162)

- [ ] **M9.0 UI architecture ADR plus a spike.** Compare the leading option (web UI plus WASM core,
  PWA, Tauri v2 for desktop and mobile) against all-Rust. Build a throwaway 3D trajectory spike in
  each and measure bundle size, frame rate on a phone-class device profile, and development effort.
  `desktop.md`'s default, Tauri 2, is replaced only for a measured reason (ADR-164 §5). The ADR
  also names which part of the mobile app would own a watch link (ADR-191).
  - Loft lessons: P15 (read the UI notes in Loft's maintainer notes before the spike).
  *Done when:* the ADR is merged with measurements.
- [ ] **M9.1 Desktop app shell.** Design editor (2D profile plus 3D model), simulation runner,
  plots, file import/export. Inspired by the workflows of OpenRocket, RASAero and RockSim, but
  modern; a part picker over the catalog and a parts list to buy from (ADR-197).
- [ ] **M9.2 3D flight replay with a ghost.** Real vs simulated flight on terrain, wind
  visualization, time scrubber, chase/ground/onboard cameras.
- [ ] **M9.3 Web PWA.** Fully client-side, offline, installable; the same UI.
- [ ] **M9.4 Mobile.** Offline PWA first, then Tauri mobile iOS/Android; a touch-first "pad day"
  mode: the day's forecast, the best launch hour, drift on the field and a flight card (ADR-163).
  Its done-when includes its guide (ADR-196): a capture fails when an element's box leaves the
  device's safe area, from each platform's published insets and corner radii, with a test that
  moves a label into a corner and fails; a later watch milestone takes the same check.

(The M9.1–M9.4 *done when* criteria are written in M9.0; M9.2's include release 0.5's shell
that opens a design or a log, flies and plots it, ADR-163; each includes the product system's
checklists for its platform, ADR-164.)
- [ ] **M9.5 Accounts and cloud saves.** Added by Neer on 2026-09-18 (VISION V20).
  - Optional accounts that save designs and flights and sync them across devices. Everything still
    works signed out and offline; accounts only add sync and sharing.
  - A saved flight stores its inputs and seed and is re-flown on open, so storage stays small.
  - Neer will pay for hosting if the project takes off. Choosing and signing up for a provider is
    his call, put to him when this milestone starts.

  *Done when:*
  - An ADR is merged comparing hosted and self-hostable options, with monthly costs at 100, 1,000
    and 10,000 users.
  - Sign-in, save, sync and delete-my-data are tested end to end, and edits made offline merge on
    reconnect without loss (a property test).

- [ ] **M9.6 The four web tools, rebuilt** (ADR-191; Neer, 2026-10-06; VISION V42). Charge, Window
  and Muster move into the library and the app; Motor Finder's pages become the app's motor picker,
  and its scraper stays a hosted service ([research](research/suite-web-tools.md)). No old site is
  turned off before its replacement ships; turning one off is Neer's.

  *Done when:*
  - every workflow in the research note's table runs in the app offline, live stock and weather
    falling back to their caches; those marked *service* run through the hosted Motor Finder;
  - density altitude, METAR ceilings, NWS alerts and a calm-window search are library functions,
    each pinned by tests against a cited formula or a recorded response; drift is pad day's
    simulated landing;
  - Muster's compatibility graph is a sourced data module whose validator fails on each kind of
    bad edge (a test each), and a design check names the reloads that fit a case;
  - Charge's backup file imports with its logs intact, and each old site's URLs have a tested map
    into the app;
  - Neer has used each screen.
- [ ] **M9.7 Field equipment and frequencies** (ADR-194; Neer, 2026-10-07; VISION V47). A register
  of altimeters, trackers, radios, cameras, payloads and batteries, a settings snapshot with each
  flight, a pad-hold battery budget and a club frequency board across vendors; offline, shared
  between phones by file or QR code.

  *Done when:*
  - the board flags two devices on the same or overlapping channels within and across vendors'
    documented channel tables (each table sourced), and is silent on adjacent clear channels (a
    test each, and a mutation probe on the overlap rule); its license notes carry a source date,
    a stale-after date and "not legal advice";
  - the battery budget uses each device's pad-state draw and errs short (a test and a mutation
    probe);
  - a flight's settings snapshot feeds M6.8's settings check and its M7.1 record;
  - Neer has used it at a launch.

## Phase 8: Releases (ADR-162)

Each release is met when ADR-162 §2's checklist holds at a commit on `main` and its release steps
are written in [`docs/releasing.md`](releasing.md); publishing and tags stay Neer's. From 0.2
on, the checklist also needs the release's product guide, written by the release milestone in
M0.8a's form and read by Neer (ADR-196): 0.2 the analyzer's; 0.3 updates the simulator's and the
analyzer's; 0.4 the competition and field kits'; 0.5 and 1.0 the app's.

- [ ] **M10.2 Release 0.2: the flight analyzer.** *Done when:* the checklist holds after M7.1,
  M7.2 and M7.3; each archive's smoke test reads a log with no design file; and a fit or residual
  of a flight that meets a condition on ADR-163 §4's list carries its warning.
- [ ] **M10.3 Release 0.3: accuracy and diagnosis.** *Done when:* the checklist holds after M2.3c2,
  M7.4 and M1.14b to M1.14g, each met or ended by ADR-143's stop rule with its gaps in an ADR.
- [ ] **M10.4 Release 0.4: the competition kit.** *Done when:* the checklist holds after M6.3,
  M6.4, M6.2e, M6.6, M6.7, M6.8, M3.5 and M3.6.
- [ ] **M10.5 Release 0.5: the app preview.** *Done when:* the checklist holds after M4.4, M9.0
  and M9.2; the app opens a design or a log, flies, plots and replays it in 3D with the ghost, on
  the desktop and the web; and Neer has used it (ADR-144's UI rule).
- [ ] **M10.6 Release 1.0: the app.** *Done when:* the checklist holds after M10.5, M5.6a, M8.2,
  M9.1, M5.6b, M8.1, M5.6c, M5.6d, M3.4 and M9.3, and Neer has used it.
## Phase 9: The suite's new lines, one at a time after 1.0 (ADR-162 §4, ADR-163 §5)

Neer's, 2026-10-04. Research: [propulsion](research/suite-propulsion.md),
[parachutes](research/suite-parachutes.md), [avionics](research/suite-avionics.md),
[the hardware line](research/suite-hardware-line.md). ADR-162 §6's
guardrails bind each line, and each ships its product guide with its first milestone (ADR-196).

Motor designer (VISION V36, V37):

- [ ] **M11.1 A motor of your own.** *Done when:* a user's `.eng` or `.rse` flies labelled
  uncertified, and a motor's mass can sit in parts at their own positions (tested on a
  draining tank's center of gravity against its closed form).
- [ ] **M11.2 Experimental solids.** Grain regression (BATES first), r = a Pⁿ, Kn, nozzle
  losses; the design flies on its nominal ballistics. *Done when:* a constant-Kn case matches its
  closed form to 1e-9; BATES designs match openMotor run headless within bounds written before
  the run; the maximum expected operating pressure (high burn rate, hot grain, start-up peak)
  errs high and the case burst margin low, each with a test and a mutation probe; the error
  against Nakka's static tests is reported.
- [ ] **M11.3 Certified hybrids.** *Done when:* a Contrail hybrid flies with oxidizer and grain
  apart, and agrees with RocketPy's hybrid motor on the same inputs within bounds written before
  the run.
- [ ] **M11.4 Nitrous blowdown.** *Done when:* an equilibrium tank, its injector's C_d A fitted
  as AIAA 2013-4045 fits it, reproduces that paper's equilibrium-model error on its ground test by
  the paper's integrated measure, within a bound written before the run.
- [ ] **M11.5 Hybrid and liquid design,** after Neer's export-control call. *Done when:* a CEA
  port reproduces RP-1311's sample problems to their printed digits.

Recovery designer (VISION V38):

- [ ] **M12.1 Parachute gores.** Flat, hemispherical and semi-ellipsoidal canopies to 1:1 SVG and DXF.
  *Done when:* each gore construction is named with its source; a gore set's sewn area equals a
  separately computed area of the surface it builds to 1e-6; in the MIT DXF generator's
  construction (overlap zero), hemispherical outlines match it within 0.5 mm; descent bounds
  bracket two vendors' tables, reference areas pinned.
- [ ] **M12.2 Opening loads.** Knacke's method, from the high end of C_D S. *Done when:* the load
  errs high against a measured opening load from a public report (a test and a mutation probe),
  the snatch force at line stretch is modeled or named a validity limit, and it prints "drop test
  first".

Avionics (VISION V39):

- [ ] **M13.1 Ground station.** Its packets include a compact event and position message that a
  watch can decode (ADR-191). *Done when:* a packet codec round-trips bit-exact under
  proptest, fuzzing finds no panic, and a simulated flight's packets replay as an M7.1 record.
- [ ] **M13.2 GPS tracker logic** (`no_std`). *Done when:* it builds for an embedded target,
  survives a dropout from the receiver's modeled speed and acceleration limits, and its radio
  schedule meets FCC §15.247's channel, dwell and power rules.
- [ ] **M13.3 Flight computer logic** (`no_std`, no heap). *Done when:* a fault-injection Monte
  Carlo over Mach 0–2.5, its run count fixed in an ADR before it runs, counts premature and
  missed deployments and failed runs; each rate's 95% Clopper–Pearson upper bound meets a target
  written in that ADR; each lockout's test is mutation-probed.
- [ ] **M13.4 Avionics hardware,** with Neer. *Done when:* Neer has built and bench-tested a board, ERC
  and DRC pass in CI, and it has flown beside a commercial primary.
- [ ] **M13.5 More boards** (ADR-191; Neer, 2026-10-06). Trackers and flight computers in several
  sizes and radios on one firmware ([hardware line](research/suite-hardware-line.md)). *Done when:*
  a second board runs the M13.2 or M13.3 firmware with only its board-support module changed,
  and Neer has built and bench-tested it.
- [ ] **M13.6 Control hardware** (ADR-193; held tier, ADR-192): drag-only airbrakes and roll-only
  tabs (roll canards later, if M6.5 shows they keep their roll), each function on one actuator
  through a common linkage where the design allows. Firmware and hardware are not written for
  the project, and never kept on GitHub or in a third-party cloud service, until a ruling or a
  lawyer's answer puts them outside the Munitions List. *Done when:* that answer is recorded; Neer
  has bench-tested neutral on power loss and on each abort condition; and a flight on 5 kN·s or
  less lands inside the 95% Monte Carlo band of M6.4 or M6.5's simulation for apogee and the roll
  rate's history.
- [ ] **M13.7 Ground-test bench** (ADR-194; public tier: it flies nothing and moves no control
  surface). A pressure logger for a sealed chamber and a wired firing box under the product
  system's ground-test rules, both writing M6.8's ground-test log. *Done when:* Neer has built and
  bench-tested both; the logger reads within its transducer's rated accuracy of a reference gauge
  across its range, with a rise time of 2 ms or less, its sample rate and port damping recorded;
  seven 0.5 g FFFFg shots (by volume, as Fetter's) in an empty 150.8 in³ chamber give a mean
  peak within 28.5 ± 4.7 psi (95%, the difference of two 7-shot means), or the gap is recorded
  in an ADR with the measurement; with a fresh battery and shorted leads, the continuity test
  stays below the named e-match's rated test current (0.04 A for MJG's Firewire); and each
  interlock blocks firing in a test.
