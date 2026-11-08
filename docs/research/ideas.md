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
the 2026-10-31 review's five ([ADR-310][adr-310], [ADR-311][adr-311]) went there too; the milestone it
promoted, every flight OpenRocket flies, is [M2.3c9][m2-3c9]. The 2026-11-02 review's four
([ADR-318][adr-318]) went to flight data, avionics, clubs, and pad and recovery. The 2026-11-04
review's four ([ADR-319][adr-319]) went to avionics and clubs.

## The 2026-11-08 planning review

Placed with [ADR-328][adr-328], which promotes the first; the other three are SOON:

- PROMOTED ([M2.3d][m2-3d]): twelve public flights, predictions first. User: a flyer reading the
  accuracy page to judge how far to trust an apogee, and anyone running [M2.6][m2-6]'s open
  benchmark. `accuracy.public`, 6.06%, rests on 7 flights, all from RocketPy's examples. Five more
  public flights with a COTS motor and a log can join them ([public flights](public-flights.md),
  [since October](public-flights-2026.md)): RocketPy's Astra, Andromeda and Erebus 11, which wait on
  their netCDF-4 weather ([#478][i478]) and designs built from their examples, and two MIT-licensed
  flights of one OpenRocket design ([#1266][i1266]). Workflow: each new flight's inputs and
  predicted apogee are merged before its error is computed. Moves: `accuracy.public` and
  `bias.public`, from 7 flights to 12. *Done when:* M2.3d's.
- SOON: the flyer's own run as a baseline. User: a flyer who already simulates in OpenRocket, asking
  whether HPR Sim tells them more than the run they made with their own settings. 228 of the private
  collection's 240 distinct design files (by SHA-256) store at least one OpenRocket run with an
  apogee (counted 2026-11-08); the scoreboard compares HPR Sim with OpenRocket 24.12 re-run in a
  standard atmosphere through the pad's temperature and pressure, not with what the flyer saw.
  Workflow: the fixture report reads the stored run of each flown configuration, with the version
  and settings it records, and gives its apogee error beside HPR Sim's and the re-run's. A stored
  run may postdate the flight and be tuned to it, which the file can't show, so the column is shown,
  never held. Moves: `accuracy.collection`, a shown column. *Done when:* the report gives the stored
  runs' mean absolute and signed errors on the flights that have one, how many have none, and a
  paired bootstrap 95% interval on HPR Sim's error less theirs, as aggregates.
- SOON: a public case another open simulator scored. User: anyone comparing simulators on the same
  public flights rather than on each tool's own claims. A new Apache-2.0 simulator scored itself
  blind on three flights of one kit from a college course's flight archive (DOFPro), a Madcow Mini
  DX3 on a G77R, a G79W and an H128W: +7.0%, +6.1% and +10.2%, each a fail by its own test
  ([Astraea's accuracy page][astraea], read 2026-11-08). HPR Sim reads DOFPro logs but has no public
  apogee case for that kit. Workflow: the kit built from its maker's published dimensions and
  masses, flown on the three motors in the archive's stated conditions, each prediction merged
  before its error, as [M2.3d][m2-3d] orders. Moves: a cited comparison beside `accuracy.public`,
  per flight only if the archive's license allows (it names none today). *Done when:* the three
  flights are flown with each input's source and the archive's license recorded (aggregates only if
  it is unclear), and the report gives HPR Sim's errors beside the other tool's, dated.
- SOON: a part's mass held to its material. User: a flyer whose design holds a mass typed ten times
  off, as one recovery tool's catalog did until 2026-11-07, listing a 462 g parachute at 46 g ([its
  release notes][recsys]). A bundled motor's mass is checked against its record ([#1233][i1233]); a
  part's typed or library mass isn't checked against its size. Workflow: at import, each part's mass
  is set against its volume times the density range of its stated material, or of any hobby material
  when none is stated, and a mass outside it is named with the part, the mass and the range. Moves:
  `accuracy.collection`, as a shown diagnosis: the mean absolute apogee error of the flights with a
  flagged part beside the rest. *Done when:* the density ranges are committed with their sources;
  the count of the collection's designs with a flagged part is reported, as an aggregate; a test
  fails when a tenfold slip either way in a body tube's or a parachute's mass goes unflagged; and
  the share of ordinary parts flagged is counted, as a false-alarm rate.

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
[or-3431]: https://github.com/openrocket/openrocket/pull/3431
[adr-212]: ../decisions/0212-measure-against-openrockets-newest-release.md
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[adr-319]: ../decisions/0319-the-2026-11-04-review-the-rocketpy-descents-tightened.md
[adr-328]: ../decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[m2-6]: ../decisions-and-roadmap.md#m2-6
[astraea]: https://github.com/akhan157/astraea/blob/main/docs/accuracy.md
[recsys]: https://github.com/akhan157/RecoverySys/releases
[i478]: https://github.com/nrdptel/fusionspace-eridanus/issues/478
[i1266]: https://github.com/nrdptel/fusionspace-eridanus/issues/1266
[i1233]: https://github.com/nrdptel/fusionspace-eridanus/issues/1233
