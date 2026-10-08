# ADR-163: The second pass: six products, the analyzer before the accuracy campaign, and what each release must hold (2026-10-04)

- **Status:** accepted
- **Summary:** amends ADR-162 §1 to §4 after a review of the whole project, its 95 open issues, the work left, its users and its competitors. The six products are defined by who uses them; the competition kit is a product and avionics starts as a test bench for any flight computer. Release 0.1 adds Monte Carlo from the CLI and Python, an apogee check on the logged flights, and named issue fixes. 0.2 becomes the flight analyzer (logs, a flight against its simulation, drag fitted from logs), ahead of the accuracy campaign it feeds; 0.3 is accuracy and fault diagnosis; 0.4 the competition kit, with RASAero and RocketPy files and ejection-charge sizing; 0.5 an app preview whose center is the 3D ghost replay; 1.0 the app with the editor. After 1.0: the motor designer, the recovery designer, pad day on a phone, then avionics

**Context.** A 2026-10-04 review, the same evening as [ADR-162][adr-162], covered the whole
project, not just the latest ideas, to put the priorities in the right order and settle what the
products will be: the built state and the open roadmap; every open issue; the vision and the ideas
backlog; what rocketeers ask for; the competitors; and the work left. What it found:

- **The work left.** Past milestones took 1 to 2 PRs when small, 3 to 6 when medium, 8 to 12 when
  large, and 20 to 26 for open-ended accuracy research. The whole road to 1.0 was estimated at
  about 107 increments of work (75 to 160). The order inside it changes when each product ships,
  not how much work there is. Order still matters, because
  each release puts its products in people's hands and gives Neer something to use.
- **What users ask for**, from rocketryforum.com threads, OpenRocket's and RocketPy's trackers,
  and the 2025 to 2026 competition rules:
  1. accurate transonic and supersonic drag and centre of pressure in one tool; today people
     design in OpenRocket and take drag from RASAero;
  2. a flight log against its simulation, and drag fitted from the log. NASA Student Launch
     requires teams to "estimate the drag coefficient ... utilizing launch data" and re-simulate;
  3. Monte Carlo landing dispersion. OpenRocket merged its own on 2026-09-17, unreleased
     ([PR #3260](https://github.com/openrocket/openrocket/pull/3260)).
- **Competition deliverables are concrete.** Student Launch asks for:
  - each section's landing kinetic energy;
  - the descent time;
  - drift at 0, 5, 10, 15 and 20 mph;
  - a declared apogee;
  - a second, independent calculation
  ([2026 handbook](https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf)).

  IREC asks for rail-exit speed, thrust to weight, the least static margin, the largest
  acceleration and speed, the fin flutter speed and the predicted apogee, and it scores apogee
  error ([rules](https://www.soundingrocket.org/uploads/9/0/6/4/9064598/irec_rules_and_requirements_document_v_1.6_.pdf)).
  EuRoC asks for a RocketPy model. Supersonic teams use RASAero.
- **Competitors.**
  - OpenRocket 24.12 (July 2025) is GPL and releases every one to two years.
  - RASAero II is closed and unchanged since 2019, Windows only.
  - RocketPy 1.13 (July 2026) is MIT, but needs drag curves from elsewhere.
  - A browser port of OpenRocket opened its beta in August 2026. Forum members judged it on its
    divergence from OpenRocket, on designs its import corrupted silently, and on real-flight
    data.
  - Every altimeter maker has its own app. The one tool that reads several brands is
    Windows-only and centred on Featherweight. No tool reads every brand, compares a flight with
    a simulation and fits drag to it.
- **What hpr can already do that nobody can reach.** Monte Carlo and optimization work only from
  Rust. `hpr mc`, `hpr optimize`, `hpr compare` and `hpr diagnose` are "not available yet"
  stubs (`crates/hpr-cli/src/lib.rs`), and the Python package has neither.
- **What 0.1 would expose.** No issue is `P-critical`. These would mislead or break a first
  user:
  - a `.ork` part's drag override read but never applied ([#165][i165]);
  - the bundled catalog's statistics under unstated terms ([#295][i295]);
  - `.hprz` names colliding on macOS ([#252][i252]);
  - offline weather lookups missing their cache ([#272][i272]);
  - stale accuracy figures ([#298][i298]);
  - a `.ork` read in Python flying no parachute ([#308][i308]);
  - a separation before rail exit firing late ([#231][i231]);
  - apogee recorded twice ([#242][i242]);
  - warnings counted as errors ([#46][i46]);
  - a drag table checked only when used, an API change best made before the first publish
    ([#79][i79]);
  - a fit check that flatters the margin ([#313][i313]).
- **Real-flight evidence stops below Mach 1.** All 7 public real flights are subsonic, and the
  supersonic references disagree on sign. The 89 tier-A flights of Neer's collection each carry
  a logged apogee ([ADR-151][adr-151]). Reading their full traces needs the M7.1 importers.
- **Neer's stated priorities** set four weights:
  - fidelity is worth extra time, to a certain extent;
  - the 3D ghost replay is a feature he especially wants;
  - the analyzer must work on its own, as just a flight analyzer;
  - he came from university competitions.

  ADR-162 put the 10-04 product lines ahead of forensics, interop and pad day, all from the
  original brief. It put the ghost replay after 1.0, and left ejection-charge sizing (asked for
  on 10-03) and pad day with no milestone.

**Decision.**

1. **Six products,** each with a user, a workflow and its first release:

   | product | for | workflow | first release |
   |---|---|---|---|
   | **hpr**, the simulator: library, CLI, Python, later C and WASM | people who script, teams, tool builders | a `.ork` or `.hpr` design, flown, its spread, plots | 0.1 |
   | **The flight analyzer** | anyone with an altimeter log, design file or not | a log, its readings with their provenance, then against its simulation, drag fitted, faults ranked | 0.2, diagnosis in 0.3 |
   | **The competition kit** | university, Student Launch, IREC, TARC and ARC teams | a rules preset and a design, optimized, checked by re-simulation, a submission pack | 0.4 |
   | **The app**: desktop and web from one codebase, then phones | hobbyists who want a window, not a terminal | open a design or a log, fly it, replay it in 3D with the ghost; then edit; then pad day | 0.5 preview, 1.0 |
   | **The designers**: motor, then recovery | research flyers, IREC teams who build their own motors, people who sew their own chutes | a grain or a canopy, its numbers as ranges, flown in the simulator | after 1.0 |
   | **Avionics**: a test bench, then a ground station, a tracker and a flight computer | people who build flight computers | firmware tested against simulated and real flights, then hpr's own boards | after 1.0 |

   The "buy what flies" motor finder is part of the competition kit (the optimizer over motor
   stock). Pad day is part of the app. Ejection charges belong to the competition kit's release,
   because teams need them first; the recovery designer reuses them. ADR-162's shared formats
   and single workspace stand.
2. **The release train,** amending ADR-162 §2:
   - **0.1, the simulator** (M10.1): M4.5 with #165 fixed; M1.14a; M4.6, Monte Carlo from the
     CLI and Python; M2.3c1, the logged flights' apogees against hpr and OpenRocket; and the
     issues above fixed or, for #79, decided.
   - **0.2, the flight analyzer** (M10.2): M7.1, M7.2 and M7.3.
   - **0.3, accuracy and diagnosis** (M10.3): M2.3c2 (the logged flights' traces), M1.14b to
     M1.14g under ADR-162 §2's terms, and M7.4.
   - **0.4, the competition kit** (M10.4):
     - M6.3, M6.4 and M6.2e;
     - M6.6, the submission packs;
     - M6.7, ejection charges;
     - M3.5, RASAero files;
     - M3.6, RocketPy files.
   - **0.5, the app preview** (M10.5): M4.4, M9.0 and M9.2. It opens a design or a log, flies it,
     plots it and replays it in 3D with the ghost, on the desktop and the web. M9.0 writes M9.2's
     *done when* to include that shell. Neer uses it before it counts.
   - **1.0, the app** (M10.6): M8.2, M9.1 (the editor), M8.1 (templates and checks), M3.4
     (RockSim files) and M9.3 (the web app), and Neer has used it.
3. **Why the analyzer comes before the accuracy campaign.** Fidelity stays the reason for the
   project ([ADR-143][adr-143]); this order serves it.
   - **The analyzer is the instrument the accuracy work lacks.** The campaign's references stop
     below Mach 1. Its stop rule needs independent references, and the next ones are logged flights:
     the 89 traces of tier A, of which 5 of the 42 that state a top speed reach about Mach 1 or
     more, and supersonic logs such as the 250 that asking SparkyVT may bring (the maintainer's to
     ask). These need M7.1's readers and M7.3's fitted drag.
   - **The analyzer is the clearest gap among competitors.** It also stands alone.
   - **The rule that a known wrong number blocks a new surface still holds.** The analyzer's
     readings (M7.1, M7.2) use no aerodynamics, so the rule doesn't touch them. M7.3's fits do
     use hpr's aerodynamics. A known drag error on a flight above Mach 0.8 would go into the
     fitted drag, so M10.2 requires each such fit or residual to carry its issue's warning.
     The competition kit and the app come after 0.3. By then M1.14 has fixed what it could, and
     has written the rest down as gaps that keep their warnings. That is the rule's intent, not
     its letter: "until fixed" can outlast a stop rule, and this ADR accepts warned gaps in
     their place.
4. **0.1's warning list** under ADR-162 §2's flattering-side rule, with the rule still the
   condition:
   - #87, #120, #121, #222 and #172 (ADR-162);
   - drag that reads high: #67 (transonic cones and ogives), #70 (sharp fins, supersonic) and
     #72 (steep boattails, supersonic). Drag that reads high makes three numbers flatter:
     - apogee reads too low against a waiver ceiling;
     - top speed reads too low, so the flutter margin and the largest dynamic pressure look
       better than they are;
     - drift reads too small.

     Each warning names speed as well as apogee. For a contest's target apogee the error has no
     safe side;
   - #68 (base drag): high from Mach 0.8 to 1.1, so apogee reads low there; low above Mach 1.2,
     so apogee reads high there. Its warning states the direction for each range;
   - #313 is not on this list: M10.1 requires it fixed.
5. **After 1.0**, amending ADR-162 §4. It stays provisional until the planning review after Neer has
   used 1.0, and the queue holds it:
   1. the motor designer's first part (M11.1, M11.2);
   2. the recovery designer (M12.1, M12.2);
   3. pad day on a phone (M9.4), the app's offline field mode: forecast, best hour, drift on
      the field, flight card;
   4. avionics as a test bench: M13.3's logic run against any flight computer's recorded
      sensors, then M13.1, M13.2;
   5. then canards (M6.5), CAD interop (M8.3), hybrids and liquids (M11.3 to M11.5), avionics
      hardware (M13.4) and accounts (M9.5).
6. **Budget.** `ROADMAP.md`'s budget rises from 24,000 bytes, set by [ADR-147][adr-147] §6, to
   28,000. The queue now reaches past 1.0, and one release, four milestones and two split
   increments are added. No other budget changes.
7. **M2.3c's split.**
   - **M2.3c1** covers the tier-A flights from a `.ork`. A flight it can't fly is refused with
     its cause in an open issue, so a refusal is a tracked defect, not a pass.
   - **The six tier-A designs from other formats** wait for M3.4 to M3.6's importers. M2.3c, the
     parent, stays open until they are in.
   - **M2.3c2's traces** need M7.1 only.
   - **Privacy:** both report aggregates only, with ADR-151's attribution of the weather data.

**Alternatives considered.**

- **Accuracy campaign before the analyzer (ADR-162's order).** Rejected:
  - it would run M1.14's 20 to 35 increments of work without the real-flight evidence above Mach 1
    that its stop rule needs;
  - it would hold back the product with the clearest gap.
- **The app editor first, the ghost replay after 1.0 (ADR-162).** Rejected. A viewer is
  narrower and cheaper than an editor. It is the feature Neer singled out, and it lets him use
  an app weeks earlier.
- **The competition kit before the accuracy release.** Rejected. Teams fly supersonic, and the
  open flattering-side drag and stability issues sit in that band. A competition tool is a new
  surface those numbers would mislead.
- **The motor designer before the app.** Rejected. Scope rule 6 holds until 1.0, and Neer's
  brief puts the app first.

**Consequences.**

- `ROADMAP.md`:
  - a new queue;
  - M2.3c split into c1 and c2;
  - new M4.6, M6.6, M6.7 and M10.6;
  - M10.2 to M10.5 retitled;
  - M4.5 names #165.
- [ADR-162][adr-162] is marked amended.
- `VISION.md`'s rows V10, V11, V21, V24, V25, V29 and V35 name their milestones.
- `hpr mc` and `hpr optimize` point at M4.6 and M6.3, which now bring them.
- The project's scope rule names M10.6.
- Start here and the records page follow.
- The issues above are linked from M10.1 and M1.14a.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-162]: 0162-the-suite-and-its-releases.md
[i165]: https://github.com/nrdptel/hpr-sim/issues/165
[i295]: https://github.com/nrdptel/hpr-sim/issues/295
[i252]: https://github.com/nrdptel/hpr-sim/issues/252
[i272]: https://github.com/nrdptel/hpr-sim/issues/272
[i298]: https://github.com/nrdptel/hpr-sim/issues/298
[i308]: https://github.com/nrdptel/hpr-sim/issues/308
[i231]: https://github.com/nrdptel/hpr-sim/issues/231
[i242]: https://github.com/nrdptel/hpr-sim/issues/242
[i46]: https://github.com/nrdptel/hpr-sim/issues/46
[i79]: https://github.com/nrdptel/hpr-sim/issues/79
[i313]: https://github.com/nrdptel/hpr-sim/issues/313
