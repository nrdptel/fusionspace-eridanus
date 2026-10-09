# Ideas backlog

Every idea from Neer's 2026-10-03 review of the project, where he accepted all of each research
pass, plus his own, custom ejection charge sizing. Ideas live here, not in the roadmap
([ADR-144, that review's decisions][adr-144]). This page holds the rules and the scope
questions; the ideas themselves are in the theme notes below, one line each.

## Tiers

- **NEXT**: feeds a queued milestone: [M4.5, flying a `.ork` as saved][m4-5] or
  [M1.14, accuracy inside the envelope][m1-14]. Only these are in the roadmap already.
- **SOON**: the phase after the queue; most attach to an existing milestone, such as
  [M6.3, competition rules][m6-3], [M6.4, airbrakes][m6-4] or the flight-log milestones
  [M7.1][m7-1] to [M7.4][m7-4].
- **LATER**: the UI phase, community features and moonshots.
- **MAINTAINER'S CALL**: money, outreach, accounts or a change of scope. Listed below, for the
  maintainer to decide.
- **PROMOTED**: moved into the roadmap, with the milestone named, such as
  [M8.3, CAD interop][m8-3] from Neer's own idea of the same day.

## Promotion rule

An idea moves into the roadmap only with a named user, the workflow it serves and a *done when*.
At most 3 user-facing surfaces are in progress at once. Safety-relevant outputs follow the
guardrails in [ADR-144][adr-144]: estimates with ranges, never a go/no-go; the range safety
officer (RSO) and the safety code decide; charge sizing always says "ground test first"; each
safety number has a test pinning which way it errs.

## Theme notes

- [Pad, range and recovery hardware](ideas-pad-and-recovery.md)
- [Avionics](ideas-avionics.md)
- [Design, build, physics and trust](ideas-design-and-physics.md)
- [Flight data](ideas-flight-data.md)
- [Competitions, certification and club operations](ideas-competitions-and-clubs.md)
- [Developer ecosystem, UI and moonshots](ideas-ecosystem-and-ui.md)

## The suite's new lines (Neer, 2026-10-04)

PROMOTED to the roadmap's Phase 9, after release 1.0 ([ADR-162, the suite and its
releases][adr-162]): a motor designer, a recovery designer for custom parachutes, and avionics.
Their research: [propulsion](suite-propulsion.md), [parachutes](suite-parachutes.md),
[avionics](suite-avionics.md).

## Promoted by the second pass (2026-10-04)

PROMOTED by [ADR-163, the second pass][adr-163]: Student Launch and IREC submission packs, a
declared apogee with its bound and a ballast solver (M6.6); ejection-charge sizing (M6.7); Monte
Carlo from the CLI and Python (M4.6); pad day on a phone (M9.4).

## In scope, low priority (Neer's scope call, 2026-10-03)

- WITHDRAWN by [ADR-193, the control scope][adr-193] (Neer, 2026-10-07: no pitch or yaw
  control): thrust vector control (TVC) and active fins with a controller in the loop.
- LATER: GPS-steered gliding parachutes (with TVC, forum threads of about 520 and 311 replies).
- LATER: rocket-boosted gliders.

## Neer's 2026-10-06 ideas

Placed by [ADR-191, the 2026-10-06 ideas][adr-191]; research in [the web tools](suite-web-tools.md),
[the repo layout](repo-layout.md), [watches](suite-watches.md) and
[the hardware line](suite-hardware-line.md).

- PROMOTED: the four web tools rebuilt in hpr (M9.6, after mobile); printed-part warnings
  (M8.3g); more tracker and flight-computer boards (M13.5).
- LATER: smartwatch companions, the phone as hub; researched again after M9.4.
- LATER: camera shroud kits; a camera shroud's drag as a protuberance, checked against
  OpenRocket's pods.
- LATER: drone and plane payloads released in flight, with a glide model after release; last of
  the hardware, since IREC bans them and drone rules don't fit a release at apogee.
- PROMOTED by [ADR-193][adr-193] (Neer, 2026-10-07): roll control on tail-fin tabs, then
  canards, roll only (M6.5, queued before M13.3); drag-only airbrake and roll-only tab firmware and
  hardware (M13.6; roll canards later), held for a ruling ([roll and drag control](roll-and-drag-control.md)).

## Neer's 2026-10-07 field tools

Placed by [ADR-194, the field tools][adr-194]; research in [field tools](suite-field-tools.md).

- PROMOTED: a launch checklist and guide built from the design, a check of altimeter settings
  against the simulated flight, and a ground-test log (M6.8, release 0.4); an equipment register
  and a cross-vendor frequency board (M9.7, after mobile); a ground-test bench (M13.7).
- LATER: the ground station reads other vendors' documented telemetry, such as Altus Metrum's
  configuration packet, to check settings over the air and list busy channels.
- LATER: a wireless firing box, after the wired one.
- LATER: camera recording reminders and a camera run-time budget against the pad hold.

## Neer's 2026-10-07 guides

Placed by [ADR-195, the guides][adr-195]; research in [guides](guides.md).

- PROMOTED: guides to all of high-power rocketry on the docs site, figures made from hpr's own
  output (M0.7a, M0.7b, M0.7d); live figures run by `hpr-wasm` in the browser (M0.7c, after 0.5).
- LATER: a classroom mode built on the guides (already listed under the ecosystem ideas).
- LATER: a guide's figure opens the same rocket in the app, with its inputs.
- LATER: NASA Glenn's public-domain figures, redrawn in the product system with credit.

## Neer's 2026-10-07 product guides

Placed by [ADR-196, the product guides][adr-196]; research in [product guides](product-guides.md).

- PROMOTED: a landing page and an illustrated tour for each product, every picture generated
  (M0.8a for the simulator; the release checklist from 0.2; each later line's first milestone).
- LATER: a versioned copy of the site for each release.
- LATER: animated CLI recordings (VHS, MIT), still frames tested first.
- LATER: runnable notebooks for the Python package, opened from the guide.

## Neer's 2026-10-07 parts catalog

Placed by [ADR-197, the parts catalog][adr-197]; research in [parts catalog](parts-catalog.md).

- PROMOTED: a catalog of hpr's own beside OpenRocket's, every value sourced and dated; search, a
  parts list to buy from, parachutes with their Cd's area, motor hardware, more makers (M5.6a to
  M5.6d); a part picker in the editor (M9.1).
- LATER: masses flyers measure, sent in with their scale's resolution.
- LATER: a "vendor confirmed" flag for data a maker sends.
- LATER: live prices through `hpr-net`, cached and dated.
- LATER: regional vendors (Klima, Wizard Rockets); parts that fit `.orc` offered upstream.

## The 2026-10-08 planning review

Each idea names its user, workflow, the scoreboard number it moves (ADR-209 §13) and a *done when*.

- PROMOTED ([M2.3c3][m2-3c3], ADR-212): OpenRocket's newest release as the bar. User: a flyer
  choosing a simulator. Workflow: reads the accuracy page against the OpenRocket they run. Moves:
  the apogee error against OpenRocket, by version. *Done when:* as M2.3c3.
- SOON: apogees measured above Mach 1, from published comparisons (Rogers' RASAero II reports; the
  CC-BY Rocket Flight Database v1.2, doi:10.5281/zenodo.19976138, 28 flights). User: a flyer
  planning a Mach 1.5 flight. Workflow: reads the supersonic row before a waiver. Moves: apogee
  error above Mach 1, not measured today. *Done when:* the real-flights report scores at least 5
  COTS-motor flights the reports give geometry for (or an ADR shows fewer exist), rebuilt from them,
  beside RASAero II's published error on the same flights, with the set's admission rule stated
  (OpenRocket Plus within ±10%, per its README); research motors wait for 1.0.
- SOON: a landing spread checked against tracked landings. User: an L2 flyer at a small field.
  Workflow: sizes the recovery area from `hpr mc`'s 90% region. Moves: the share of GPS-logged
  landings inside the 50% and 90% regions, not measured today. *Done when:* the fixture report gives
  both shares with Clopper–Pearson bounds over every flight with a tracked landing, failed runs
  counted.
- SOON: weighed against estimated mass at import. User: a flyer whose apogee is 20% off. Workflow:
  reads which masses are weighed (overrides, and what each covers) and which estimated, with each
  one's effect on the apogee. Moves: the collection's apogee error split by the share of liftoff
  mass weighed. *Done when:* `hpr sim` prints that share and each override's scope; the fixture
  report gives the error above and below the median share, fixed before the errors are read, as
  aggregates.

## MAINTAINER'S CALL (money, outreach)

- Bids to be the official simulator of US competitions.
- A paid team workspace (Excalidraw+ style).
- University validation partners.
- Disclosed buy links to motor.fusionspace.co, never affecting rankings.
- Verified kit badges, with kit makers.
- ThrustCurve: a data-quality give-back, and asking its maintainer about bundling common motors.
- A fiscal host or sponsor tiers (each funder's eligibility rules to be checked first).
- DROPPED by Neer (2026-10-05, the logs can't be had): asking SparkyVT for logs.
  Its flight computer's README claims 250+ flights to Mach 2.3, but no logs are published.
  Supersonic evidence comes from `hpr-sim-fixtures` instead (ADR-151).
- A migration layer for users of orhelper (a GPL-licensed Python bridge to OpenRocket): a
  licensing call.
- PROMOTED to [M10.1, release 0.1][adr-162]: prebuilt binaries through a release. The project
  prepares the workflow; publishing a release is Neer's.
- Asking parts vendors for their data, and a "vendor confirmed" flag ([ADR-197][adr-197]).
- crates.io and PyPI names, and `hpr-io`'s license field: deferred by Neer for now.

[adr-197]: ../decisions/0197-the-2026-10-07-parts-catalog.md
[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-163]: ../decisions/0163-the-second-pass-products-and-priorities.md
[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
[adr-144]: ../decisions/0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[m1-14]: ../decisions-and-roadmap.md#m1-14
[m4-5]: ../decisions-and-roadmap.md#m4-5
[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m7-1]: ../decisions-and-roadmap.md#m7-1
[m7-4]: ../decisions-and-roadmap.md#m7-4
[m8-3]: ../decisions-and-roadmap.md#m8-3
[adr-193]: ../decisions/0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md
[adr-194]: ../decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-195]: ../decisions/0195-the-2026-10-07-guides-rocketry-explained.md
[adr-196]: ../decisions/0196-the-2026-10-07-product-guides.md
[m2-3c3]: ../decisions-and-roadmap.md#m2-3c3
