# Ideas: developer ecosystem, UI and moonshots

Part of the [ideas backlog](ideas.md) (2026-10-03). One line each, with its tier. AI features
never write numbers or make safety calls: every number comes from the engine.

## From the 2026-10-28 planning review

- SOON: an export that survives OpenRocket's own save. User: a team that designs in HPR Sim and
  hands the `.ork` to a club member who opens, edits and saves it in OpenRocket, whose 24.12 save
  can fail on a file whose simulations are stale ([OpenRocket issue
  3412](https://github.com/openrocket/openrocket/issues/3412)). Workflow: the export round trip
  ([ADR-286][adr-286]) adds a step: OpenRocket's jar, run as an outside oracle, saves each exported
  design without flying it, and HPR Sim reads the saved file back. Moves: `interop.changed` (0 of
  151 configurations today), `interop.failed` (2 of 75 designs, the two HPR Sim can't read) and
  `interop.largest` (0.000050%), all measured only to OpenRocket's open. *Done when:* the round trip saves all 151 through OpenRocket and counts the
  saves that fail or move an apogee by more than 0.002%, and a test holds one design through the
  save step.
- SOON: HPR Sim's own flight time, held. User: anyone flying thousands of runs (`hpr mc`, the
  optimizer) or waiting on the web app. Workflow: `speed.flight`'s HPR Sim cell reads a committed
  bench record (machine, commit, design, median of at least 6 alternated runs) rather than waiting
  for [M2.9][m2-9]'s other cells. Moves: `speed.flight`, not measured today; [the speed
  page](../perf.md) times a Valetudo flight at 1.65 ms on one thread. *Done when:* the cell shows
  the record's median with its machine, and the scoreboard's check fails when a new record is more
  than 10% slower than the last on the same machine.

## Developer ecosystem

- SOON: an MCP server (Model Context Protocol, the standard way for AI assistants to call tools).
- SOON: notebook widgets.
- SOON: a GitHub Action for design CI.
- LATER: a sandboxed WebAssembly (WASM) plugin API, with Rhai (a small embedded scripting
  language for Rust) for light scripting.
- SOON: the JSON Schema on SchemaStore.
- LATER: an editor extension and language server.
- LATER: MCAP export (an open log format for robotics data, read by ROS 2 and Foxglove).
- LATER: oEmbed embeds.
- LATER: FMU export (a Functional Mock-up Unit, the FMI standard's packaged co-simulation model).
- LATER: CZML (Cesium's format for time-dynamic 3D scenes) and animated glTF.
- SOON: Arrow and Parquet batch output.
- LATER: Grafana overlays.
- LATER: Pyodide and JupyterLite (Python in the browser).
- LATER: a Discord bot.
- LATER: Excel functions.
- SOON: a simulator-neutral conformance suite.
- LATER: an OBS (streaming software) overlay.
- SOON: import on-site wind soundings.
- LATER: the MCP server as an AI design benchmark (RocketBench, [arXiv 2504.19394](https://arxiv.org/abs/2504.19394)).
- SOON: HPR Sim's speed cell before [M2.9][m2-9]'s. User: someone choosing a tool by speed.
  Workflow: `cargo xtask scoreboard` reads the one-flight benchmark ([performance](../perf.md))
  from a committed record. Moves: `speed.flight` for HPR Sim, 14 to 15 of 51 cells. *Done when:*
  the cell names machine, toolchain and commit; `--check` fails on a record older than the code.

## UI and experience (the UI phase)

- NEXT: plots and figures first ([M4.5e, `hpr sim --plot`][m4-5e];
  [M1.14g, figures on the accuracy pages][m1-14g]).
- LATER: grounded number chips: each number links to where the engine computed it.
- LATER: one time cursor across 3D, plots and video (Rerun-style timelines).
- LATER: a scrubbable edit-history sparkline.
- LATER: a configurations grid (Onshape style).
- LATER: auto-director replay clips.
- LATER: a command palette with a typed grammar.
- LATER: plain language to a competition spec to the optimizer.
- LATER: a critique with previewable fixes.
- LATER: ask-the-flight, answers citing time ranges.
- LATER: a local large language model (LLM) option, running on the user's own machine.
- LATER: linked user part components (Figma style).
- LATER: pencil sketching (Shapr3D style).
- LATER: shareable panel layouts.
- LATER: Strava-style flight cards.
- LATER: photoreal launch sites.
- LATER: live force arrows in the replay.
- LATER: stability modes against flight time.
- LATER: shareable flight links.
- LATER: local-first designs: CRDTs (conflict-free replicated data types, which merge edits made
  offline), branches, comments.
- LATER: a classroom mode and a data pack for NGSS (the US Next Generation Science Standards).
- LATER: leaderboards, closest-to-prediction included.
- LATER: accessibility: screen-reader plots and sonification.
- LATER: country regulation profiles (UK, Canada) and translations, French first.
- LATER: "your flight against others on this kit and motor".

## Moonshots (LATER)

- LATER: an open drag commons (opt-in, privacy-safe).
- LATER: a blind flight index: predictions committed before launch, scored after.
- LATER: `hpr-core` as `no_std` on flight computers; the first step is an embedded build check.
- LATER: `.hpr` handed to neutral governance (as glTF was).
- LATER: signed flight passports.
- LATER: vortex-lattice aerodynamics in the browser (WebGPU).
- LATER: pad-side trajectory capture.
- LATER: motor scatter from certification data.
- LATER: shared range winds.
- LATER: sanctioned virtual events.
- LATER: a curriculum partner, in the style of PhET (the University of Colorado's free
  interactive science simulations).

From the 2026-10-20 planning review ([ADR-286][adr-286]):

- SOON: OpenRocket's whole-rocket aero tables, read and flown. User: a flyer whose `.ork` from
  OpenRocket's next release carries drag and stability tables by Mach ([its guide][or-aero]).
  Workflow: `hpr sim` flies them as OpenRocket does, held at their edges, names them and flags the
  flight past their last Mach; until then such a file is refused ([#879][i-879]). Moves:
  `features.count`. *Done when:* a probe `.ork` with both tables flies within 0.5% of that
  release's apogee (pinned as 24.12 is), and tests hold the edge rule both ways.

From the 2026-10-24 planning review ([ADR-289][adr-289]):

- SOON: designs that fly as saved, counted against OpenRocket. User: a flyer opening an old `.ork`
  from a forum or a club's archive. Workflow: `hpr sim` flies it or refuses with one named reason;
  the scoreboard counts, from the committed reports, what each tool flies as saved and the rest by
  reason. Moves: a new row, `flies.as-saved`: today hpr flies 61 of the collection's 83 flights
  whose design is a `.ork`, OpenRocket 79 ([the fixture report][fixture-report]), and hpr 54 of the
  57 configurations of OpenRocket's examples. *Done when:* the row reads both tools' counts from
  the reports, and a check fails when hpr's count falls.
- SOON: the same flight on every platform. User: a university team whose members fly one design on
  macOS, Windows and Linux and compare the numbers in a design review. Workflow: each CI target
  writes the apogee, burnout speed and landing time of the validation cases that have them; a job
  compares the five targets (six once [#438][i-438] lets Intel macOS run) and reports the largest
  relative difference. Moves: a new row, `determinism.platforms`. *Done when:* the report gives
  that number over every target CI runs, and CI fails when it passes 1e-6 of the value.

## Neer's 2026-10-07 guides and product guides

Placed by [ADR-195, the guides][adr-195]; research in [guides](guides.md).

- PROMOTED: guides to all of high-power rocketry on the docs site, figures made from hpr's own
  output (M0.7a, M0.7b, M0.7d); live figures run by `hpr-wasm` in the browser (M0.7c, after 0.5).
- LATER: a classroom mode built on the guides (already listed under the ecosystem ideas).
- LATER: a guide's figure opens the same rocket in the app, with its inputs.
- LATER: NASA Glenn's public-domain figures, redrawn in the product system with credit.

Placed by [ADR-196, the product guides][adr-196]; research in [product guides](product-guides.md).

- PROMOTED: a landing page and an illustrated tour for each product, every picture generated
  (M0.8a for the simulator; the release checklist from 0.2; each later line's first milestone).
- LATER: a versioned copy of the site for each release.
- LATER: animated CLI recordings (VHS, MIT), still frames tested first.
- LATER: runnable notebooks for the Python package, opened from the guide.

## From the 2026-10-26 planning review

- SOON: motors at a field with no signal, counted. User: a flyer at a remote launch whose first
  motor sold out. Workflow: the scoreboard counts the motors each tool flies offline from a fresh
  install (HPR Sim's bundle, [M5.7][m5-7]; OpenRocket 24.12's jar; RocketPy's documentation) and,
  of those, the curves whose total impulse misses the published figure by more than 1% with no
  warning ([ADR-005][adr-005]'s check). Moves: new rows `motors.offline` and `motors.unchecked`;
  hpr bundled 32 curves then and flies 924 offline since M5.7b2 ([the bundle][motor-page])
  against 1,452 entries recorded from OpenRocket's database ([its defects
  note](openrocket-apogee-defects.md)), 528 short. *Done when:* hpr's
  counts come from the bundle's committed manifest and OpenRocket's from a committed record naming
  the jar's hash; hpr's offline count is at least OpenRocket's; a check fails when hpr's falls.

[m1-14g]: ../decisions-and-roadmap.md#m1-14g
[m4-5e]: ../decisions-and-roadmap.md#m4-5e
[m2-9]: ../decisions-and-roadmap.md#m2-9
[adr-286]: ../decisions/0286-the-2026-10-20-review-the-export-bar-tightened.md
[or-aero]: https://openrocket.readthedocs.io/en/latest/user_guide/advanced_flight_simulation.html
[i-879]: https://github.com/nrdptel/fusionspace-eridanus/issues/879
[adr-289]: ../decisions/0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[i-438]: https://github.com/nrdptel/fusionspace-eridanus/issues/438
[adr-195]: ../decisions/0195-the-2026-10-07-guides-rocketry-explained.md
[adr-196]: ../decisions/0196-the-2026-10-07-product-guides.md
[adr-005]: ../decisions/0005-solid-motors-statistics-consumption-grains-file.md
[m5-7]: ../decisions-and-roadmap.md#m5-7
[motor-page]: ../physics/motor.md#the-bundled-motors
