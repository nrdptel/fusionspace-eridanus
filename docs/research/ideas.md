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
ecosystem, UI and moonshots](ideas-ecosystem-and-ui.md); [accuracy against real
flights](ideas-accuracy.md).

## The suite's new lines and the second pass (2026-10-04)

PROMOTED to the roadmap's Phase 9, after release 1.0 ([ADR-162, the suite and its
releases][adr-162]): a motor designer, a recovery designer for custom parachutes, and avionics.
Their research: [propulsion](suite-propulsion.md), [parachutes](suite-parachutes.md),
[avionics](suite-avionics.md). PROMOTED by [ADR-163, the second pass][adr-163]: Student Launch and
IREC submission packs, a declared apogee with its bound and a ballast solver (M6.6); ejection-charge
sizing (M6.7); Monte Carlo from the CLI and Python (M4.6); pad day on a phone (M9.4).

## Neer's 2026-10-06 ideas and 2026-10-07 field tools

Placed by [ADR-191, the 2026-10-06 ideas][adr-191] and [ADR-194, the field tools][adr-194]; the
ideas are in [avionics](ideas-avionics.md#neers-2026-10-06-ideas) and
[competitions and clubs](ideas-competitions-and-clubs.md#neers-2026-10-07-field-tools).

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

## Earlier planning reviews (2026-10-08 to 2026-11-08)

PROMOTED: OpenRocket's newest release as the bar ([M2.3c3][m2-3c3], ADR-212), the gap to it by
cause (M2.3c6, [ADR-226][adr-226]) with OpenRocket's supersonic ogive drag as its own cause
([M2.3c6][m2-3c6], [ADR-273][adr-273]), apogees past Mach 1 ([M2.3c7][m2-3c7], [ADR-236][adr-236]),
Student Launch's limits by requirement number ([M6.3][m6-3], [ADR-246][adr-246]), the
post-flight comparison ([M7.2][m7-2], [M7.3][m7-3], [ADR-257][adr-257]), fin flutter held to
measurement ([M2.11][m2-11], [ADR-279][adr-279]). The rest moved to the [theme notes](#theme-notes);
the 2026-10-20 review's four ([ADR-286][adr-286]) and the 2026-10-24 review's four ([ADR-289][adr-289])
to flight data and the ecosystem, and the 2026-10-26 review's four ([ADR-294][adr-294]) to flight
data, clubs and the ecosystem. The 2026-10-28 review's four ([ADR-303][adr-303]) went to flight data and the ecosystem, and
the 2026-10-31 review's five ([ADR-310][adr-310], [ADR-311][adr-311]) went there too; the milestone it
promoted, every flight OpenRocket flies, is [M2.3c9][m2-3c9]. The 2026-11-02 review's four
([ADR-318][adr-318]) went to flight data, avionics, clubs, and pad and recovery. The 2026-11-04
review's four ([ADR-319][adr-319]) went to avionics and clubs. The 2026-11-08 review's three
([ADR-328][adr-328]) went to [accuracy against real flights](ideas-accuracy.md); the milestone it
promoted, twelve public flights with predictions first, is [M2.3d][m2-3d].

## The 2026-11-11 planning review

Placed with [ADR-338][adr-338], which promotes the first; the other four are SOON:

- PROMOTED ([M2.12][m2-12]): descent rates against the logs, predictions first. User: a flyer
  sizing a main, a drift budget or a landing's energy. Every scoreboard row against a log reads its
  apogee; the [flight data](ideas-flight-data.md) note's descent idea is the first on another
  output. Workflow: the comparison's rule and each collection flight's predicted rates are merged
  before any error. Moves: new rows `accuracy.descent` and `bias.descent`, too gentle the
  flattering side. *Done when:* M2.12's.
- SOON: RocketPy's own apogee table beside HPR Sim's. User: anyone comparing simulators on the same
  public flights. An open RocketPy pull request commits a table of apogee errors on 11 of its
  example rockets, each flown on the drag its example supplies: median absolute error 6.36%, 90th
  percentile 8.42% ([its table][rp1208], committed 2026-10-24, read 2026-11-11; unmerged). Six of the
  11 are among HPR Sim's 7 public flights and three more join with [M2.3d][m2-3d]. Workflow: the
  real-flights report shows that table's error for each flight both fly, beside HPR Sim's on its own
  aerodynamics and on the example's drag, cited and dated. Moves: `accuracy.public`, a cited figure
  beside its RocketPy cell until [M2.9][m2-9] measures it. *Done when:* the report gives the cited
  error per shared flight and its mean, with the table's commit, and a test fails when the pinned
  table's hash changes.
- SOON: a third tool on the supersonic list, and the finish it assumes. User: a flyer past Mach 1
  asking which simulator to trust. Pocket Rocket Flight Lab scores itself on 23 measured apogees
  from RASAero II's example files: its mean absolute error is 9.9%, +9.2% with its sign, and 13.0%
  on the 9 flights above Mach 1.2; it gives RASAero II's own as 5.4%. It names finish as the largest unknown: at regular paint, 60 µm,
  its signed error falls to +0.5% ([its validation page][prfl], read 2026-11-11). Workflow: where
  [M2.3c7][m2-3c7] scores those flights, the report cites that tool's errors beside RASAero II's and
  flies each flight at the file's finish and at two named finishes. Moves: `accuracy.supersonic`,
  with its spread across finishes. *Done when:* the report gives the three tools' errors on the
  flights all score, and HPR Sim's at each finish, each finish's roughness sourced.
- SOON: the published apogee beside the log's. User: a reader checking an error against the figure
  a team published. Sources quote one flight's measured apogee differently: NDRT 2020's is 1,320.4 m
  as HPR Sim reads its log and 1,316.7 m in RocketPy's table. Workflow: the real-flights report
  lists, per flight, the apogee its team or its example states beside the log's as read, and the
  mean absolute error against each. Moves: `accuracy.public`, shown against a second reference.
  *Done when:* each flight has both apogees with their sources, or says none is published, and the
  report gives the error both ways.
- SOON: the collection's other predictions, sealed before anyone looks. User: a reader asking
  whether a new accuracy row was tuned. No committed report compares the collection's simulated
  times to apogee or top speeds with their logs, so each can still be fixed before its error is
  seen. The fit from the boost has read the calibration half's climbs, so burnout speed is left out. Workflow: one
  pull request merges the hash of each flight's predicted time to apogee and top speed, as
  [ADR-333][adr-333] holds the apogees', and the fixture report refuses to compare an output whose
  hash didn't merge first. Moves: the proposed `accuracy.time-to-apogee` row, whose first figure
  would then be one fixed before its error was seen. *Done when:* the hashes are merged, and a
  test shows the report refuses a flight whose predicted time to apogee wasn't merged first.

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
[adr-294]: ../decisions/0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[adr-303]: ../decisions/0303-the-2026-10-28-review-the-rocketpy-peaks-tightened.md
[adr-310]: ../decisions/0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-311]: ../decisions/0311-the-2026-10-31-review-every-flight-openrocket-flies.md
[adr-305]: ../decisions/0305-the-2026-10-28-licensing-calls.md
[adr-318]: ../decisions/0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[adr-212]: ../decisions/0212-measure-against-openrockets-newest-release.md
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[adr-319]: ../decisions/0319-the-2026-11-04-review-the-rocketpy-descents-tightened.md
[adr-328]: ../decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[adr-338]: ../decisions/0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-333]: ../decisions/0333-m2-7b-new-flights-predicted-first.md
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-9]: ../decisions-and-roadmap.md#m2-9
[rp1208]: https://github.com/RocketPy-Team/RocketPy/pull/1208
[prfl]: https://github.com/dustintlee-blahblah/PocketRocketFlightLab/blob/main/VALIDATION.md
