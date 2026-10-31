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

## Earlier planning reviews (2026-10-08 to 2026-10-28)

PROMOTED: OpenRocket's newest release as the bar ([M2.3c3][m2-3c3], ADR-212), the gap to it by
cause (M2.3c6, [ADR-226][adr-226]) with OpenRocket's supersonic ogive drag as its own cause
([M2.3c6][m2-3c6], [ADR-273][adr-273]), apogees past Mach 1 ([M2.3c7][m2-3c7], [ADR-236][adr-236]),
Student Launch's limits by requirement number ([M6.3][m6-3], [ADR-246][adr-246]), the
post-flight comparison ([M7.2][m7-2], [M7.3][m7-3], [ADR-257][adr-257]), fin flutter held to
measurement ([M2.11][m2-11], [ADR-279][adr-279]). The rest moved to the [theme notes](#theme-notes);
the 2026-10-20 review's four ([ADR-286][adr-286]) and the 2026-10-24 review's four ([ADR-289][adr-289])
to flight data and the ecosystem, and the 2026-10-26 review's four ([ADR-294][adr-294]) to flight
data, clubs and the ecosystem. The 2026-10-28 review's four ([ADR-303][adr-303]) went to flight data and the ecosystem.

## The 2026-10-31 planning review

Placed with [ADR-310][adr-310], which tightens the RocketPy comparisons' rail-exit and landing
speeds, and [ADR-311][adr-311], which promotes one idea of this review: every flight OpenRocket
flies, as the scoreboard row `coverage.collection` ([M2.3c9][m2-3c9]). The 2026-10-28 review's four
moved to the theme notes.

- SOON: the gap's tail. User: a team's simulation lead asking how far to trust HPR Sim against
  OpenRocket. HPR Sim holds more flights within 5% and within 10% of the log (14 vs 13, 32 vs 31) and
  still loses on the mean, so a few flights make the gap. Workflow: `cargo xtask fixture-flights`
  prints the mean absolute gap with the worst 1, 3 and 5 of its contributors removed. Moves:
  `accuracy.collection` (12.07% vs 11.26%), by naming how much of it is a tail. *Done when:* the
  report prints those three aggregates and no flight's identity, a test pins them on invented
  flights, and [M2.3c6][m2-3c6]'s split names the same contributors.
- SOON: a second flight on the first flight's drag. User: a Level 3 flyer planning a second flight
  of a rocket with one good log. Workflow: fit the drag on the first log ([M7.3][m7-3]), then
  `hpr sim` the next flight with that fit. Per-flight drag with its interval, rather than one fleet
  factor, is what `bias.collection` (+7.76%) would need to shrink. Moves: new row
  `accuracy.second-flight`, lower better. *Done when:* the row gives the count of designs with two
  logged flights and the error with and without the fit; a synthetic pair pins the fit; the row
  says so plainly where the count is too small to mean anything.
- SOON: the intervals' width, held. User: a flyer quoting a log's apogee with its interval. The
  coverage test ([ADR-308][adr-308]) has a gap: a noisier reader, once refitted, widens its intervals
  and still passes. Workflow: the synthetic flights' report records each reading's mean interval
  width. Moves: a new row, `readings.width`, lower better; `readings.inside-90` beside it from the
  same report, nearer 90% better. *Done when:* the report prints each reading's mean width on sets
  A and B, `synthetic --check` fails when one grows more than 10% over its committed figure, and a
  probe adding noise to one reading shows that the width check fails while coverage passes.
- SOON: descent rates against the log. User: a flyer sizing a main or a drift budget. Workflow: the
  fixture report compares each tool's descent rate under the main with the rate read from that
  flight's log. Moves: new row `accuracy.descent`, lower better; too gentle is the flattering side.
  *Done when:* signed and absolute errors with their counts for both simulators, aggregates only,
  and a test pinning which way too gentle reads.
- SOON: what moved my number. User: a flyer whose saved prediction changes after an upgrade; a
  merged change to OpenRocket's damping reports one three-stage booster's apogee moving 20%
  ([OpenRocket pull request 3410](https://github.com/openrocket/openrocket/pull/3410)). Workflow:
  `hpr sim design.ork --against last-time.json` prints each number's change, both versions and the
  release notes' physics entries between them. Moves: `features.count`, as a new row: can a tool
  say what changed a saved number across versions. *Done when:* a test flies one design against a saved run from
  an edited copy, the change prints with its sign, and a saved run from an unknown version says so
  and doesn't guess.

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
- DROPPED by Neer (2026-10-28, [ADR-305][adr-305]): a migration layer for users of orhelper
  (a GPL-licensed Python bridge to OpenRocket). The `.ork` importer already serves those users.
- PROMOTED to [M10.1, release 0.1][adr-162]: prebuilt binaries through a release. The project
  prepares the workflow; publishing a release is Neer's.
- A "vendor confirmed" flag ([ADR-197][adr-197]). Asking parts vendors for their data was
  dropped by Neer (2026-10-28, [ADR-305][adr-305]): their published dimensions and
  masses are used as facts.
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
[adr-005]: ../decisions/0005-solid-motors-statistics-consumption-grains-file.md
[adr-294]: ../decisions/0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[adr-303]: ../decisions/0303-the-2026-10-28-review-the-rocketpy-peaks-tightened.md
[adr-308]: ../decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md
[adr-310]: ../decisions/0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-311]: ../decisions/0311-the-2026-10-31-review-every-flight-openrocket-flies.md
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[m2-9]: ../decisions-and-roadmap.md#m2-9
[adr-305]: ../decisions/0305-the-2026-10-28-licensing-calls.md
