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

[Pad, range and recovery hardware](ideas-pad-and-recovery.md); [avionics](ideas-avionics.md);
[design, build, physics and trust](ideas-design-and-physics.md); [flight data](ideas-flight-data.md);
[competitions, certification and club operations](ideas-competitions-and-clubs.md); [developer
ecosystem, UI and moonshots](ideas-ecosystem-and-ui.md).

## The suite's new lines and the second pass (2026-10-04)

PROMOTED to the roadmap's Phase 9, after release 1.0 ([ADR-162, the suite and its
releases][adr-162]): a motor designer, a recovery designer for custom parachutes, and avionics.
Their research: [propulsion](suite-propulsion.md), [parachutes](suite-parachutes.md),
[avionics](suite-avionics.md). PROMOTED by [ADR-163, the second pass][adr-163]: Student Launch and
IREC submission packs, a declared apogee with its bound and a ballast solver (M6.6); ejection-charge
sizing (M6.7); Monte Carlo from the CLI and Python (M4.6); pad day on a phone (M9.4).

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

## Neer's 2026-10-07 guides and product guides

Placed by [ADR-195, the guides][adr-195] and [ADR-196, the product guides][adr-196]; the ideas
are in [the ecosystem note](ideas-ecosystem-and-ui.md#neers-2026-10-07-guides-and-product-guides).

## Neer's 2026-10-07 parts catalog

Placed by [ADR-197, the parts catalog][adr-197]; research in [parts catalog](parts-catalog.md).

- PROMOTED: a catalog of hpr's own beside OpenRocket's, every value sourced and dated; search, a
  parts list to buy from, parachutes with their Cd's area, motor hardware, more makers (M5.6a to
  M5.6d); a part picker in the editor (M9.1).
- LATER: masses flyers measure, sent in with their scale's resolution.
- LATER: a "vendor confirmed" flag for data a maker sends.
- LATER: live prices through `hpr-net`, cached and dated.
- LATER: regional vendors (Klima, Wizard Rockets); parts that fit `.orc` offered upstream.

## Earlier planning reviews (2026-10-08 to 2026-10-24)

PROMOTED: OpenRocket's newest release as the bar ([M2.3c3][m2-3c3], ADR-212), the gap to it by
cause (M2.3c6, [ADR-226][adr-226]) with OpenRocket's supersonic ogive drag as its own cause
([M2.3c6][m2-3c6], [ADR-273][adr-273]), apogees past Mach 1 ([M2.3c7][m2-3c7], [ADR-236][adr-236]),
Student Launch's limits by requirement number ([M6.3][m6-3], [ADR-246][adr-246]), the
post-flight comparison ([M7.2][m7-2], [M7.3][m7-3], [ADR-257][adr-257]), fin flutter held to
measurement ([M2.11][m2-11], [ADR-279][adr-279]). The rest moved to the [theme notes](#theme-notes);
the 2026-10-20 review's four ([ADR-286][adr-286]) and the 2026-10-24 review's four ([ADR-289][adr-289])
to flight data and the ecosystem.

## The 2026-10-26 planning review

Placed with [ADR-294][adr-294], which tightens the RocketPy comparisons' times, burnout heights
and traces; no idea is promoted.

- SOON: how often each tool comes in low. User: a flyer filing for a waiver, and the RSO who checks
  it; a prediction below the log is the worst error ([ADR-144][adr-144]). Workflow: the scoreboard
  counts, from the reports, the logged flights each tool predicts more than 5% below the log (an
  early deployment only lowers a log, so it never explains one). Moves: new rows
  `undershoot.collection` and `undershoot.public`; today hpr 10 of 53 and OpenRocket 11 of 53 (4 and
  6 more than 10% low), hpr 3 of 7 public flights, read off [the fixture report][fixture-report]'s
  histograms. *Done when:* both rows read fields the reports write, a test pins that −5% exactly is
  not counted and −5.01% is, and the scoreboard's check fails when hpr's count rises.
- SOON: center of pressure against the tunnel, for every tool. User: an RSO or a Level 3 mentor
  checking a stability margin from any of the three tools. Workflow: each wind-tunnel model the
  aerodynamics checks score is written as a `.ork` by HPR Sim's writer ([ADR-286][adr-286]);
  OpenRocket 24.12's jar, run as an outside oracle, and RocketPy 1.13.0 give its center of pressure
  at each reading's Mach number; the scoreboard shows each tool's mean absolute error in calibers
  and the readings it puts aft of the measured one, which overstates the margin. Moves: new row
  `accuracy.cp-tunnel`, three cells; today hpr's only: 0.58 calibers over the Arcas Robin's 16
  readings below Mach 2.5, 9 of them aft ([the fixture][nf-fixture]). *Done when:* the row gives all
  three tools on at least 16 core-band readings from at least two models, each cited to its table
  or figure; a reading a tool can't model is listed with its reason; a test fails when a tool's aft
  count is missing.
- SOON: motors at a field with no signal, counted. User: a flyer at a remote launch whose first
  motor sold out. Workflow: the scoreboard counts the motors each tool flies offline from a fresh
  install (HPR Sim's bundle, [M5.7][m5-7]; OpenRocket 24.12's jar; RocketPy's documentation) and,
  of those, the curves whose total impulse misses the published figure by more than 1% with no
  warning ([ADR-005][adr-005]'s check). Moves: new rows `motors.offline` and `motors.unchecked`;
  today hpr bundles 32 curves ([the bundle][motor-page]) against 1,452 entries recorded from
  OpenRocket's database ([its defects note](openrocket-apogee-defects.md)). *Done when:* hpr's
  counts come from the bundle's committed manifest and OpenRocket's from a committed record naming
  the jar's hash; hpr's offline count is at least OpenRocket's; a check fails when hpr's falls.
- SOON: a flight with a date and no hour. User: a flyer flying again a forum post's flight that
  gives the day but not the time. Workflow: `hpr weather era5` given a date with no time writes a
  profile for each hour from 08:00 to 18:00 local mean solar time, and `hpr sim` prints the
  apogee's range over them with the hours; the fixture report does the same for each compared
  flight with no known hour (26 of the 83 fly local noon today). Moves: `accuracy.collection`
  gains a stated hour term: the median and largest spread over the day, in % of the logged
  apogee, beside the mean absolute error on flights with a known hour. *Done when:* the report
  gives those numbers and the count of compared flights with no hour; a test on a synthetic day
  with a known temperature swing gets the range derived by hand; a date with no time is refused,
  with the reason, when the file doesn't cover those hours.

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
[adr-226]: ../decisions/0226-the-2026-10-10-review-the-gap-to-openrocket-by-cause.md
[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[adr-236]: ../decisions/0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[adr-246]: ../decisions/0246-the-2026-10-12-review-student-launch-by-requirement-and-oracle-defects.md
[adr-257]: ../decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[m7-2]: ../decisions-and-roadmap.md#m7-2
[m7-3]: ../decisions-and-roadmap.md#m7-3
[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[adr-273]: ../decisions/0273-the-2026-10-15-review-openrockets-supersonic-ogive-drag-as-a-cause.md
[m2-11]: ../decisions-and-roadmap.md#m2-11
[adr-279]: ../decisions/0279-the-2026-10-17-review-fin-flutter-held-to-measurement.md
[adr-286]: ../decisions/0286-the-2026-10-20-review-the-export-bar-tightened.md
[adr-289]: ../decisions/0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[adr-005]: ../decisions/0005-solid-motors-statistics-consumption-grains-file.md
[adr-294]: ../decisions/0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[m5-7]: ../decisions-and-roadmap.md#m5-7
[nf-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/normal-force-vs-mach.json
[motor-page]: ../physics/motor.md#the-bundled-motors
