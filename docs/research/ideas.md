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

## Earlier planning reviews (2026-10-08 to 2026-11-11)

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
promoted, twelve public flights with predictions first, is [M2.3d][m2-3d]. The 2026-11-11 review's
four ([ADR-338][adr-338]) went there too; the milestone it promoted, descent rates against the
logs, is [M2.12][m2-12].

## The 2026-11-14 planning review

Placed with [ADR-347][adr-347], which promotes the first; the other five are SOON, two of them
in [accuracy against real flights](ideas-accuracy.md):

- PROMOTED ([M2.13][m2-13]): landing points against the tracked landings, predictions first.
  User: a flyer judging whether the rocket comes down inside the field. `hpr sim` prints the
  landing's distance from the pad, held today to another simulator's and to no flight. 41 of the
  private collection's 83 flights are catalogued with a position log (counted 2026-11-14 from the
  collection's catalog; no track read yet). Workflow: each track's pad and landing points and
  each flight's predicted landing point are merged before any miss. Moves: new
  rows `accuracy.landing` and `bias.landing`, nearer the pad than tracked the flattering side.
  *Done when:* [M2.13][m2-13]'s.
- SOON: the landing on the forecast a flyer had. User: a flyer at the pad, who flies a forecast,
  not the reanalysis [M2.13][m2-13] scores. The command line's page says of a forecast's winds
  that "nothing has compared them with measured ones". Workflow: for each tracked flight whose
  launch hour is known, a forecast issued before the launch is read from a public archive that
  keeps past runs, cached and flown; the fixture report gives the landing miss on it beside the
  miss on the reanalysis. Moves: `accuracy.landing`, a second figure on a forecast. *Done when:*
  the archive and its lead time are named from its own documents; the report gives both misses
  over the flights it covers and counts those it doesn't; and a test pins one cached profile by
  its hash.
- SOON: speed at the rail's end, against accelerometer logs. User: a flyer or a range safety
  officer checking that the rocket leaves the rail fast enough to fly straight. `hpr sim` prints
  that speed, and no report sets it against a flight; a speed read too high is the flattering side.
  48 of the collection's 83 flights have a log with an acceleration channel, 43 of them sampled at
  100 Hz or faster (counted 2026-11-14 from the collection's catalog).
  Workflow: the analyzer integrates the logged acceleration from first motion over the rail length
  the flight's record states, with an interval from the sensor's offset and the first-motion time;
  each flight's predicted speed at that length is merged first, as [ADR-333][adr-333] holds the
  apogees'. Moves:
  new rows `accuracy.rail-exit` and `bias.rail-exit`. *Done when:* the reading is checked on
  synthetic boosts with a known answer; the hashes are merged before any error; the report gives
  the mean absolute and signed errors with their count; and a test pins which sign is too fast.
- SOON: the delay's spread held to logged delays. User: a flyer choosing an ejection delay from a
  scattered run. The spread fitted to bench firings is too narrow for flights: its 90% interval
  held 21 or 22 of 30 logged delays, 70% to 73% ([the delay-spread note][delay-spread], section 7;
  [#1474][i1474] asks the Monte Carlo page to say so). Workflow: `hpr mc` gets a named delay spread fitted
  to logged delays, and the report scores its intervals on logged flights the fit didn't read.
  Moves: a new row `uncertainty.delay-90`, nearer 90% better, beside the apogee ranges of
  [M2.8][m2-8]. *Done when:* the report gives the share of logged delays inside the 50% and the
  90% intervals with Clopper–Pearson bounds, on flights held out from the fit by their ids' hash,
  and the Monte Carlo page states both.

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
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[adr-319]: ../decisions/0319-the-2026-11-04-review-the-rocketpy-descents-tightened.md
[adr-328]: ../decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[adr-338]: ../decisions/0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-333]: ../decisions/0333-m2-7b-new-flights-predicted-first.md
[m2-12]: ../decisions-and-roadmap.md#m2-12
[adr-347]: ../decisions/0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[m2-13]: ../decisions-and-roadmap.md#m2-13
[m2-8]: ../decisions-and-roadmap.md#m2-8
[delay-spread]: motor-delay-spread.md
[i1474]: https://github.com/nrdptel/fusionspace-eridanus/issues/1474
