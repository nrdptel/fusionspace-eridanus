# Vision

## What was asked

Neer, the project's owner, set the brief on 2026-09-16 and added to it as the work went on. Each
entry below says what was asked, dated. The requirements table further down turns them into
checkable items.

### The brief (2026-09-16)

- The best hobby rocket simulator, at maximum quality, on the best stack available, with a Rust
  base.
- Build it like RocketPy first: the simulation itself matters most, so no UI or visual polish yet.
- Open and unconstrained, like RocketPy: usable as part of other projects and pluggable into
  anything.
- Designing a rocket.
- Import files from other simulators, and export to their formats.
- A design format of the project's own, better than the open ones that exist today, perhaps built
  on one of them.
- Validation matters a great deal: pull down whatever free simulators and data exist to validate
  against.
- Online features (weather, location, parts names and stock), while the whole tool still works
  fully offline on macOS, Windows and Linux.
- Live solid motor stock from Neer's own API, https://motor.fusionspace.co/api.
- University competition challenges, with constraints of various kinds on the rocket to optimize
  for.
- Take in flight data and compare it with the simulation; when a flight went wrong, infer what
  the cause could be.
- Eventually, a UI inspired by what already works (OpenRocket, RASAero, RockSim): more modern
  visuals, more 3D, whole-flight 3D simulation, and the simulated flight shown as a "ghost" (as in
  Mario Kart) beside the real flight in one interactive 3D view.
- After that, a web version that runs entirely client side and installs as a PWA; then a mobile
  UI usable offline, through the PWA or through iOS and Android apps.
- Different from, and more modern than, current solutions.
- For now, COTS solid rocket motors only.
- Open source, like Neer's other FusionSpace projects (https://fusionspace.co/). Loft, his
  similar earlier project, is being shut down because it is not what he wants; its repository
  (`fusionspace-loft`) and his private resources repository (`loft-fixtures`) may be used.

### Additions

- **2026-09-17, documentation.** A heavy emphasis on documentation that is easy to reach and easy
  to read.
- **2026-09-18, more of the rocket.** Airbrakes, and later canards, though the fluid modeling may
  be hard. Payloads whose weight distribution changes during flight. Ejected payloads, nose cones
  and body tubes, each landed under its own parachute. Accounts that save designs and simulation
  runs, which may not be free; Neer is willing to pay for them if the project takes off.
  Multistage, transitions, clusters and side pods.
- **2026-09-20, flight logs.** Debrief, Neer's flight log project, is being sunset too: like Loft,
  it went in the wrong direction and was too web based. Its use case, a universal flight log
  analyzer, becomes part of this project, and a user can use the product as just a flight
  analyzer.
- **2026-10-03, fidelity.** Taking longer to develop is worth it when it improves performance: one
  reason for the project is a better performing, higher fidelity product, within reason. There
  will be no outside feedback until much later in the project, and the work should not wait for
  it. On 2026-10-03 the ideas backlog was adopted, with ejection-charge sizing added. The ideas
  are in `docs/research/ideas.md`, each tiered; the
  roadmap takes only the ones that feed queued work.
- **2026-10-03, CAD interop** (now M8.3). Export the design or its parts to STL, STEP and
  drawings, with FreeCAD parameters and feature trees. Import any of those formats as a solid
  custom part, such as fins, a nose cone, a camera canister or a payload bump.
- **2026-10-03, a scope call.** Thrust vector control, steered parachutes and rocket gliders can
  be in, but are not a priority.
- **2026-10-04, the suite** ([ADR-162, the suite and its releases](decisions/0162-the-suite-and-its-releases.md),
  which sets the suite, the release train and their order). Eventually, support for hybrids,
  liquids and other propulsion, and developing them here, especially experimental solid motors;
  custom parachutes, as people make their own; custom flight computers and GPS trackers. These
  become a suite of products developed together, working together and compatible with one
  another. Prioritizing what is done when matters most, and a proper release should not wait for
  everything.
- **2026-10-04, evening, the design system** ([ADR-164, the FusionSpace product system](decisions/0164-the-fusionspace-product-system.md)).
  Take in and reference Neer's design system, `fusionspace-design`, folding it in carefully.
- **2026-10-06, more ideas** ([ADR-191, the 2026-10-06 ideas](decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md)).
  - Smartwatch companions after the mobile work, with fewer uses; researched when the time comes.
  - Neer's four web-first products, Motor Finder, Charge, Window and Muster (FusionSpace's
    `FS · SW · TOOL 001` to `004`; ADR-164), sunset and rebuilt from the ground up here: more
    platform agnostic and more connected to the other hpr tools, bearing in mind that hpr uses
    Motor Finder's API.
  - As the products grow, splitting into smaller workspaces (separate directories, repositories
    or sub-repositories), after researching it.
  - Hardware: printing the designs made here, several flight computer models, several tracking
    modules, camera, drone and plane payloads, canards and airbrakes.
  - Each is researched first, developed correctly and prioritized with the rest.
- **2026-10-06, the follow-up** ([ADR-192, the follow-up](decisions/0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md)),
  on three questions ADR-191 left open. Charge's minimum-strength pins need no more work: nobody
  uses it and it will be sunset. Breaking release 0.1's reader of Motor Finder's API is fine: no old
  product, and no dependency on one, holds the new one back. On export control, research the rules
  as far as possible, how they have been applied and how others handle them; keeping some material
  in a separate private repository is fine, publishing as much as the law allows. There is no rush
  for release 0.1: the release strategy stands, but the aim is the best product, not the first one
  out the door.
- **2026-10-07, the control scope** ([ADR-193, the control scope](decisions/0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md);
  research in [roll and drag control](research/roll-and-drag-control.md)), building on the
  export research. The scope follows the line the competition rules (IREC) draw: drag or roll
  stabilization is OK, guidance toward a target never is, and the rocket must be stable with the
  controls switched off. Airbrakes designed to strictly add drag, with no active pitch or yaw
  control; canards for roll control only; and servo-driven fin tabs for roll control. This is
  added to the project.
- **2026-10-07, field tools** ([ADR-194, the field tools](decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md);
  research in [field tools](research/suite-field-tools.md)). Software and hardware for the launch
  field and for testing: a launch checklist and guide; managing radios, altimeters, trackers,
  payloads and cameras; ejection testing. Existing tools are surveyed first, and the gaps are
  researched and added.
- **2026-10-07, guides** ([ADR-195, the guides](decisions/0195-the-2026-10-07-guides-rocketry-explained.md);
  research in [guides](research/guides.md)). In-depth guides, with good visuals and possibly
  interaction, to all things high-power rocketry.
- **2026-10-07, product guides** ([ADR-196, the product guides](decisions/0196-the-2026-10-07-product-guides.md);
  research in [product guides](research/product-guides.md)). In-depth guides, with good visuals,
  to the project's products as well.
- **2026-10-07, a parts catalog** ([ADR-197, the parts catalog](decisions/0197-the-2026-10-07-parts-catalog.md);
  research in [parts catalog](research/parts-catalog.md)). Like OpenRocket's catalog of materials
  and commercial products (body tubes, nose cones, parachutes, motor hardware), so anyone can make
  a design from commercial parts in about 5 minutes, without researching each part, and then buy
  it. OpenRocket's catalog may be used if that is allowed; over time, research a higher fidelity,
  more informative and larger one.
- **2026-10-07, the project's name** ([ADR-198, project Eridanus](decisions/0198-the-2026-10-07-project-eridanus.md)).
  Take in the updates to `fusionspace-design` and plan when to integrate them. First, name the
  whole project after a constellation with many stars, its products after the stars, and rename
  the repository `fusionspace-<constellation>`. Neer named the project Eridanus.
- **2026-10-07, the public name** ([ADR-199, FusionSpace HPR](decisions/0199-the-2026-10-07-fusionspace-hpr.md)).
  On 2026-10-07 Neer decided FusionSpace should appear in the product names, in more places than
  not and across the project's other products. Neer named the public brand FusionSpace HPR.
- **2026-10-08, one name and one design** ([ADR-205, one name and one design](decisions/0205-the-2026-10-08-one-name-one-design.md)).
  hpr.fusionspace.co was still headlined by hpr-sim. A global overhaul so that the naming and the
  design standards line up with `fusionspace-design` everywhere, with no holes; the docs cover
  only the simulator today, and that will change.
- **2026-10-08, the best on the market** ([ADR-209, the best on the market, measured](decisions/0209-the-2026-10-08-best-on-the-market-measured.md)).
  Every product aims to beat the best comparable tool on accuracy, fidelity, resolution and ease of use,
  and to do useful things no other tool does; each claim is measured, and each release's guide compares.

**Operating envelope** (ADR-143): the bands are set by Mach alone. Accuracy work goes first to the
core band, Mach 0–2.5. hpr flies the extended band, Mach 2.5–3.5, with its accuracy checked less,
and faster flights still fly; the envelope is Mach 0–3.5 at any angle of attack. The angle is a
separate condition: accuracy work assumes 15° or less, and a flight above 15° more than 1 s after
the rail is at high angle of attack, at any Mach. M1.14a will flag any flight beyond the validated
range (faster than the fastest public whole-flight reference, OpenRocket's example at Mach 1.147
today; private flights never set it), at high angle of attack, outside the core band or beyond the
envelope. The envelope orders the work; it is not a ceiling. An accuracy increment shrinks a
measured error against an independent reference, or adds a reference that will. Either counts as
progress; two increments in a row that do neither end the milestone, gaps written down.

Decisions Neer confirmed at kickoff:

- **Name:** keep `hpr-sim` for now. On 2026-10-07 the project became Eridanus, the repository
  `fusionspace-eridanus` and the simulator the star Achernar (ADR-198); the public name is
  FusionSpace HPR, on every product and package, with the command `hpr` (ADR-199).
- **Repository:** public GitHub repo from day one, with CI on macOS, Windows and Linux.

## Requirements (the brief, turned into checkable items)

| id | requirement | first milestone |
|---|---|---|
| V1 | Rust core; a library-first, RocketPy-class 6-DOF simulator; accuracy is the top priority | M1.x |
| V2 | Open and unconstrained: usable from Rust, a CLI, Python, C and WASM. Every model can be swapped through traits | M4.x |
| V3 | Design a rocket: parametric component model, parts catalog, design checks | M1.4, M5.5, M8.x |
| V4 | Import and export OpenRocket `.ork`, RockSim `.rkt`, RASAero `.CDX1`, RocketPy, and motor files `.eng`/`.rse` | M1.3, M3.x |
| V5 | Our own open design format that improves on the existing ones and builds on `.ork` | M3.3 |
| V6 | Validation against free simulators (RocketPy, OpenRocket) and real flight data, published openly | M2.x |
| V7 | Online weather, site/elevation, parts, and live motor stock from motor.fusionspace.co, all optional | M5.x |
| V8 | Works fully offline on macOS, Windows and Linux | every milestone |
| V9 | Competition challenges: constraint specs, optimizer, live stock and prices | M6.x |
| V10 | Import flight data, compare it with the simulation, and infer what went wrong | M7.3 (release 0.2), M7.4 (0.3) |
| V21 | A universal flight log analyzer that a user can use on its own: read any logger's file and get the flight's readings, with no design file and no simulation (ADR-046) | M7.1, M7.2 (release 0.2, ADR-163) |
| V11 | Later: a modern UI inspired by OpenRocket, RASAero and RockSim, with 3D whole-flight simulation and the sim shown as a "ghost" beside the real flight | M9.2 (release 0.5's app preview, ADR-163), M9.1 (1.0) |
| V12 | Later: a client-side web version (installable PWA), then mobile (offline PWA and/or iOS/Android apps) | M9.x |
| V13 | COTS solid motors only, until release 1.0 ([ADR-162, the suite and its releases](decisions/0162-the-suite-and-its-releases.md)) | scope rule |
| V14 | Open source, in the style of the other FusionSpace projects | M0.1 |
| V15 | Documentation that is easy to reach and easy to read: one searchable site linked from the README, plain language first, every term defined, every claim traceable to its source, test and validation, so a person can check the work without reading the code | M0.4, then every milestone |
| V16 | Airbrakes, then canards: active drag and roll control, as competitions fly them | M6.4, M6.5 |
| V17 | Ejected nose cones, body sections and payloads, each landed under its own recovery | M1.11 |
| V18 | Payload mass that moves or is released during flight | M1.12 |
| V19 | Multistage, transitions, clusters and side pods | M1.4 (transitions, done), M1.9, M1.13 |
| V20 | Optional accounts that save designs and flights; paid hosting is acceptable if the project takes off, and the rest stays free and offline | M9.5 |
| V22 | Fidelity first, within reason: accuracy inside the operating envelope (ADR-143) is worth extra time | M1.14 |
| V23 | Fly a `.ork` as saved, with its recovery, and motors fetched on demand | M4.5 |
| V24 | Pad and range tools: flight cards, delay and waiver odds, best launch hour, landing spread against the field (`docs/research/ideas-pad-and-recovery.md`) | M9.4's pad day, after 1.0 (ADR-163) |
| V25 | Recovery hardware: chute sizing, shock cord and shear pins, custom ejection charge sizing, always "ground test first" (`docs/research/ideas-pad-and-recovery.md`) | M6.7 (release 0.4), M12.1, M12.2 |
| V26 | Avionics: simulated sensor streams, controller-in-the-loop, altimeter settings and what the altimeter will read (`docs/research/ideas-avionics.md`) | later, with M6.4 |
| V27 | Design and build: kits, fabrication packs, as-built mode, flutter as a spread (`docs/research/ideas-design-and-physics.md`) | later, M8.x |
| V28 | Flight data: log health, system identification of drag and stability, wind from the log (`docs/research/ideas-flight-data.md`) | M7.x |
| V29 | Competitions and certification: report packs, ballast solvers, declared apogee with uncertainty (`docs/research/ideas-competitions-and-clubs.md`) | M6.3, M6.6 (release 0.4) |
| V30 | Club operations: RSO and LCO checks, flight-line planning, day-of kits (`docs/research/ideas-competitions-and-clubs.md`) | later |
| V31 | Developer ecosystem: MCP server, notebooks, plugins, schemas, interchange exports (`docs/research/ideas-ecosystem-and-ui.md`) | later |
| V32 | UI and automated assistant features, with every number grounded in the engine: assistant features never write numbers or make safety calls (`docs/research/ideas-ecosystem-and-ui.md`) | M9.x |
| V33 | CAD interop: export a part or the design to STL, STEP, drawings and a parametric FreeCAD tree; import any of those as a solid custom part (fins, nose cone, camera canister, payload bump) | M8.3 |
| V34 | A suite of products built together and compatible by construction: the simulator, the flight analyzer, the app, a motor designer, a recovery designer and avionics, joined by the hpr format and the canonical flight record (ADR-162) | ADR-162 |
| V35 | Releases at product boundaries, not after everything: 0.1 the simulator, 0.2 the flight analyzer, 0.3 accuracy and diagnosis, 0.4 the competition kit, 0.5 the app preview with the 3D ghost, 1.0 the app (ADR-162, ADR-163) | M10.1 to M10.6 |
| V36 | Fly other propulsion: a user's own thrust curve, certified hybrids, then liquids, each motor's mass where it really sits (tanks drain) | M11.1, M11.3, M11.4 |
| V37 | Design motors: experimental solids first (grain, burn rate, nozzle; never a propellant formulation), then hybrids and liquids; every design says "static test first" | M11.2, M11.5 |
| V38 | Design custom parachutes: canopy shape, 1:1 gore patterns, descent rate and opening load as ranges; "drop test first" | M12.1, M12.2 |
| V39 | Custom avionics: a ground station, a GPS tracker and a flight computer, their firmware checked against simulated and real flights, hardware built and bench-tested by Neer | M13.1 to M13.4 |
| V40 | Designed to the FusionSpace product system ([fusionspace-design](https://github.com/nrdptel/fusionspace-design), Rev A): drawn, not decorated; the working shown; how far to trust it on every result; quiet color with meaning; numbered `FS · SW · TOOL 005` ([ADR-164, the product system decision](decisions/0164-the-fusionspace-product-system.md)) | M0.6 |
| V41 | Smartwatch companions after the mobile apps: the phone as hub, glances and alerts on the wrist; researched when the time comes ([watches](research/suite-watches.md)) | after M9.4 (ADR-191) |
| V42 | Neer's four web tools (Motor Finder, Charge, Window, Muster) rebuilt in hpr, platform agnostic and joined to the rest; each old site retired only after its replacement ships; none of the old tools, Motor Finder's API included, constrains hpr's design (ADR-192; [web tools](research/suite-web-tools.md)) | M6.7, M9.6 |
| V43 | Split the project into workspaces or repositories as it grows, each split by a measured trigger ([repo layout](research/repo-layout.md)) | ADR-191 |
| V44 | A hardware line: printable designs, several flight computers and tracking modules, camera, drone and plane payloads, airbrake and canard hardware, ordered by what the law allows ([hardware line](research/suite-hardware-line.md)) | M8.3g, M13.5, ADR-191 |
| V45 | Active control only as drag and roll: airbrakes that strictly add drag, canards and servo-driven fin tabs that only roll, never pitch or yaw; stable with the controls switched off ([roll and drag control](research/roll-and-drag-control.md)) | M6.4, M6.5, M13.6, ADR-193 |
| V46 | A launch checklist and guide built from the rocket's own configuration, each rule-ordered step citing its rule ([field tools](research/suite-field-tools.md)) | M6.8 (release 0.4), M9.4, ADR-194 |
| V47 | Field equipment: a register of radios, altimeters, trackers, payloads, cameras and batteries; each altimeter's settings checked against the simulated flight; one frequency board across vendors | M6.8, M9.7, ADR-194 |
| V48 | Ejection testing: a ground-test log that charge sizing reads, and a ground-test bench (a pressure logger and a wired firing box), always "ground test first" | M6.7, M6.8, M13.7, ADR-194 |
| V49 | In-depth guides to all of high-power rocketry on the docs site, each figure made from hpr's own output, uncertainty drawn with its flattering side marked, nothing copied from copyrighted sources ([guides](research/guides.md)) | M0.7a, M0.7b, M0.7d, ADR-195 |
| V50 | Live guide figures: the reader changes a number in the text and hpr itself, in the browser, redraws the figure, offline | M0.7c (after release 0.5), ADR-195 |
| V51 | An in-depth, illustrated guide for each product: a landing page and a tour ending on a public real flight, every picture made by a command CI reruns and checked for clipping, phone and watch captures against the safe area ([product guides](research/product-guides.md)) | M0.8a, the release checklist from 0.2, ADR-196 |
| V52 | A design from purchasable parts in minutes: search the catalog, build from it, and get a parts list to buy from; a catalog of hpr's own beyond OpenRocket's, every value with its source and date, parachutes with their drag and its range, motor hardware, rail buttons and recovery hardware ([parts catalog](research/parts-catalog.md)) | M5.6a to M5.6d, M9.1, ADR-197 |

## North stars

1. **Trustworthy numbers.** Every output comes from a cited model, is pinned by tests, and is
   measured against independent references. Where the model is weak, the docs say so.
2. **A simulator you can build on.** A clean core with no I/O and stable, documented extension
   points. Other people's tools should want to embed it.
3. **Fast enough to explore.** Thousands of flights per second, so Monte Carlo, optimization and
   live "what-if" sliders feel instant.
4. **Honest about uncertainty.** A flight is a distribution, not a single number. Dispersion and
   sensitivity analysis are first-class features.
5. **Offline at the pad.** Nothing essential depends on a network.
6. **Readable by people.** The documentation is how a person understands and checks this code. A
   newcomer finds any answer in two clicks and follows it without reading the source.
7. **The best on the market, measured.** Each product beats the best comparable tool on
   accuracy, fidelity, resolution and ease of use, and does something useful none of them does;
   every such claim stands on a committed comparison (ADR-209).

## Beyond the brief: ideas to consider (keep, drop, or queue each through the roadmap)

- **Flight forensics.** Compare a flight log with its simulation, rank fault hypotheses by
  likelihood, and say which extra data would tell them apart. *Queued:* M7.3 and M7.4, with the
  analyzer it stands on in M7.1 and M7.2 (V21).
- **A drag model that learns.** Each logged flight updates the rocket's drag and mass estimates by
  Bayesian updating, so predictions get sharper with every flight.
- **Buy what flies.** "Which motors in stock this week get my design to 10,000 ft ±2%, under $150?"
  combines the optimizer, live stock, and the challenge rules.
- **Pad-day mode.** Take today's winds aloft, then show a drift ellipse on the field map, a
  suggested delay, and apogee with an error band. Estimates only, never a go/no-go verdict.
- **Sensitivity tornado.** Show which parameters actually move apogee and landing for *this*
  rocket.
- **A public accuracy census.** An auto-generated table of simulator-vs-reference errors across
  every validation case.
- **Git-friendly designs.** A canonical, diffable format with stable component ids. A design can
  also be shared as a compressed URL on the web.
- **Ghost replay data product.** Time-synced sim and real trajectories in a common frame, exported
  now and ready for the future 3D viewer.

## Not now

- Hybrid, liquid and research (EX) motors, and designing motors, parachutes or avionics: after
  release 1.0, one line at a time, in [ADR-162](decisions/0162-the-suite-and-its-releases.md)'s
  order (motor designer, recovery designer, avionics, then hybrids and liquids).
- Active control beyond drag-only airbrakes (M6.4) and roll-only tabs and canards (M6.5): no
  pitch or yaw control, simulated or built. Neer's 2026-10-07 decision withdraws the thrust vector
  control and active fins of his 2026-10-03 scope call (ADR-193). That call's GPS-steered gliding
  parachutes and rocket-boosted gliders stay **in scope, at low priority**: they wait in the
  ideas backlog (tier LATER) until the queued work is done, and none is promoted ahead of it.
- Any GUI work before release 0.4's milestones are done (ADR-162 §3; see `ROADMAP.md`).
- Firmware or hardware that moves a control surface: held off GitHub and out of third-party cloud
  services until a ruling or a lawyer puts it outside the Munitions List
  ([the export control note](research/export-control.md), ADR-192). Simulated drag-only
  airbrakes and roll-only tabs and canards go ahead (ADR-193).
