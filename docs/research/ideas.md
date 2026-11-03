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

## Earlier planning reviews (2026-10-08 to 2026-10-28)

PROMOTED: OpenRocket's newest release as the bar ([M2.3c3][m2-3c3], ADR-212), the gap to it by
cause (M2.3c6, [ADR-226][adr-226]) with OpenRocket's supersonic ogive drag as its own cause
([M2.3c6][m2-3c6], [ADR-273][adr-273]), apogees past Mach 1 ([M2.3c7][m2-3c7], [ADR-236][adr-236]),
Student Launch's limits by requirement number ([M6.3][m6-3], [ADR-246][adr-246]), the
post-flight comparison ([M7.2][m7-2], [M7.3][m7-3], [ADR-257][adr-257]), fin flutter held to
measurement ([M2.11][m2-11], [ADR-279][adr-279]). The rest moved to the [theme notes](#theme-notes);
the 2026-10-20 review's four ([ADR-286][adr-286]) and the 2026-10-24 review's four ([ADR-289][adr-289])
to flight data and the ecosystem, and the 2026-10-26 review's four ([ADR-294][adr-294]) to flight
data, clubs and the ecosystem. The 2026-10-28 review's four ([ADR-303][adr-303]) went to flight data and the ecosystem, and
the 2026-10-31 review's five ([ADR-310][adr-310], [ADR-311][adr-311]) went there too; its sixth,
every flight OpenRocket flies, was promoted as [M2.3c9][m2-3c9].

## The 2026-11-02 planning review

Placed with [ADR-318][adr-318], which tightens a met target and promotes none of these. Each is SOON:

- SOON: several flights, one drag. User: a team, or a Level 3 flyer, with three or more logs of one
  airframe on different reloads of one motor. A drag fitted to one altimeter log can't tell a weak
  reload from extra drag; one open-source fitter's README puts that at about 6% in drag at one
  standard deviation and suggests fitting over several flights ([hpr-flight-sim][hfs]). Workflow: `hpr compare` given several logs of one
  design fits one drag scale that every flight shares and an impulse scale for each
  ([M7.3][m7-3]'s fit), and prints the drag scale's 90% interval with the count of flights. Moves:
  new row `fit.drag-width`, the drag scale's mean 90% interval width, lower better; through it, the
  proposed `accuracy.second-flight`. *Done when:* on barometer-only synthetic sets of 1, 2 and 4
  flights of one design with a 3% scatter in reload impulse, the shared drag's intervals pass
  [ADR-257][adr-257]'s three counts at each size; their mean width at 4 flights is at most 60% of
  one flight's (1/√4 is 50%); and one flight alone says where drag and impulse aren't told apart.
- SOON: the intervals by sample rate. User: a flyer whose logger records at 10 or 15 Hz, quoting a
  log's apogee or top speed with its 90% interval. A 2026 preprint, down-sampling one logged
  flight, finds the apogee from its accelerometer ranging over 66 m at 10 Hz against 12 m at 50 Hz,
  while the barometer's stays within 0.3 m ([Varigonda][varigonda]).
  Workflow: the synthetic flights' report counts each reading's 90% coverage
  ([ADR-308][adr-308]) in three bands of the logger's rate: under 20 Hz, 20 to 100 Hz, and over
  100 Hz, each band drawn to hold at least 200 readings. Moves: the proposed
  `readings.inside-90`, by band. *Done when:* each band passes [ADR-257][adr-257]'s three counts
  on sets A and B, or the reading is withheld below the rate where it fails; a probe that fits the
  bound on the fastest band alone fails the slowest.
- SOON: OpenRocket's next release, measured before it ships. User: a team's simulation lead who
  will upgrade OpenRocket and wants to know which of their numbers move. Three changes merged to
  OpenRocket's development branch on 2026-11-02 move printed numbers: a tangent ogive's supersonic
  wave drag (a Mach 2.5 test rocket's apogee 5.5% lower, by the pull request's own figures,
  [pull request 3431][or-3431]), jet damping in pitch and yaw, and the propulsive damping's lever
  arm. Workflow: the fixture report flies the public and the collection's flights on a published
  development build or pre-release of OpenRocket, run as an outside program, beside the pinned
  24.12. Moves: the OpenRocket cells of `accuracy.collection` and `bias.collection` (11.26% and
  +6.72%), shown as a forecast, never held. *Done when:* the report names the build and its date
  and gives its mean absolute and mean apogee errors with the count and the flights that moved more
  than 1%, aggregates only, or says that no build is published; [M2.3c3][m2-3c3] reuses the
  harness on the release. New beside [ADR-212][adr-212], which already makes the newest
  release the bar: a build before it ships, shown as a forecast, never held.
- SOON: the wind's error as measured. User: a flyer whose recovery area ends at a road, flying on
  a forecast of light wind. `hpr mc` scatters the wind as a fraction of its speed and a turn of its
  direction, so a 2 m/s forecast scattered by 20% moves by 0.4 m/s, where reanalysis winds over
  the US are inferred to miss measured ones by about 1.8 to 2.4 m/s in each component by day and
  more than 3.2 m/s on the worst nights (a bracket from soundings and profilers, not a direct
  measurement: [the ERA5 wind note](era5-wind-direction-us.md)). Too narrow an ellipse is
  the flattering side. Workflow: `hpr mc --wind-error 2` adds an error in each horizontal
  component, the same at every speed, beside the present scatters. Moves: the proposed
  `landing.inside-90`. *Done when:* a test gets a calm forecast's ellipse from the component error
  alone, derived by hand; a property test shows the option never narrows an ellipse; the page
  gives the measured errors with their source.

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
[adr-308]: ../decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md
[adr-310]: ../decisions/0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-311]: ../decisions/0311-the-2026-10-31-review-every-flight-openrocket-flies.md
[adr-305]: ../decisions/0305-the-2026-10-28-licensing-calls.md
[adr-318]: ../decisions/0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[hfs]: https://github.com/aneeshkaravadi/hpr-flight-sim
[varigonda]: https://doi.org/10.31224/8645
[or-3431]: https://github.com/openrocket/openrocket/pull/3431
[adr-212]: ../decisions/0212-measure-against-openrockets-newest-release.md
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
