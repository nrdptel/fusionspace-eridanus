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
flights](ideas-accuracy.md) and
[its continuation](ideas-accuracy-2.md).

## The suite's new lines and the second pass (2026-10-04)

PROMOTED to the roadmap's Phase 9, after release 1.0 ([ADR-162, the suite and its releases][adr-162]): a
motor designer, a recovery designer for custom parachutes, and avionics. Their research:
[propulsion](suite-propulsion.md), [parachutes](suite-parachutes.md), [avionics](suite-avionics.md).
PROMOTED by [ADR-163, the second pass][adr-163]: Student Launch and IREC submission packs, a declared
apogee with its bound and a ballast solver (M6.6); ejection-charge sizing (M6.7); Monte Carlo from the
CLI and Python (M4.6); pad day on a phone (M9.4).

## Neer's 2026-10-06 ideas and 2026-10-07 field tools, guides and parts catalog

Placed by [ADR-191, the 2026-10-06 ideas][adr-191], [ADR-194, the field tools][adr-194], [ADR-195, the
guides][adr-195] and [ADR-196, the product guides][adr-196]; the ideas are in
[avionics](ideas-avionics.md#neers-2026-10-06-ideas), [competitions and
clubs](ideas-competitions-and-clubs.md#neers-2026-10-07-field-tools) and [the ecosystem
note](ideas-ecosystem-and-ui.md#neers-2026-10-07-guides-and-product-guides).
The parts catalog's ideas were placed by [ADR-197, the parts catalog][adr-197]; research in [parts catalog](parts-catalog.md).

- PROMOTED: a catalog of hpr's own beside OpenRocket's, every value sourced and dated; search, a
  parts list to buy from, parachutes with their Cd's area, motor hardware, more makers (M5.6a to
  M5.6d); a part picker in the editor (M9.1).
- LATER: masses flyers measure, sent in with their scale's resolution.
- LATER: a "vendor confirmed" flag for data a maker sends.
- LATER: live prices through `hpr-net`, cached and dated.
- LATER: regional vendors (Klima, Wizard Rockets); parts that fit `.orc` offered upstream.

## Earlier planning reviews (2026-10-08 to 2026-11-19)

PROMOTED: OpenRocket's newest release as the bar ([M2.3c3][m2-3c3], ADR-212), the gap to it by cause
(M2.3c6, [ADR-226][adr-226]) with OpenRocket's supersonic ogive drag as its own cause ([M2.3c6][m2-3c6],
[ADR-273][adr-273]), apogees past Mach 1 ([M2.3c7][m2-3c7], [ADR-236][adr-236]), Student Launch's limits
by requirement number ([M6.3][m6-3], [ADR-246][adr-246]), the post-flight comparison ([M7.2][m7-2],
[M7.3][m7-3], [ADR-257][adr-257]), fin flutter held to measurement ([M2.11][m2-11], [ADR-279][adr-279]).
The rest moved to the [theme notes](#theme-notes); the 2026-10-20 review's four ([ADR-286][adr-286]) and
the 2026-10-24 review's four ([ADR-289][adr-289]) to flight data and the ecosystem, and the 2026-10-26
review's four ([ADR-294][adr-294]) to flight data, clubs and the ecosystem. The 2026-10-28 review's four
([ADR-303][adr-303]) went to flight data and the ecosystem, and the 2026-10-31 review's five
([ADR-310][adr-310], [ADR-311][adr-311]) went there too; the milestone it promoted, every flight
OpenRocket flies, is [M2.3c9][m2-3c9]. The 2026-11-02 review's four ([ADR-318][adr-318]) went to flight
data, avionics, clubs, and pad and recovery. The 2026-11-04 review's four ([ADR-319][adr-319]) went to
avionics and clubs. The 2026-11-08 review's three ([ADR-328][adr-328]) went to [accuracy against real
flights](ideas-accuracy.md); the milestone it promoted, twelve public flights with predictions first, is
[M2.3d][m2-3d]. The 2026-11-11 review's four ([ADR-338][adr-338]) went there too; the milestone it
promoted, descent rates against the logs, is [M2.12][m2-12]. The 2026-11-14 review's four
([ADR-347][adr-347]) went there too; the milestone it promoted, landing points against the tracked
landings, is [M2.13][m2-13]. The 2026-11-16 review's three ([ADR-352][adr-352]) went there too; its
milestone, the speed off the rail against the accelerometer logs, is [M2.14][m2-14]. The 2026-11-19
review's three ([ADR-360][adr-360]) went to [accuracy against real flights,
continued](ideas-accuracy-2.md); the milestone it promoted, drag in the coast against the logs, is
[M2.15][m2-15].

## The 2026-11-22 planning review

Placed with [ADR-370][adr-370], which promotes the first; the other three are SOON.

- PROMOTED ([M2.16][m2-16]): the next flight predicted from the last one's log. User: a flyer with a
  logged flight of a rocket, planning its next on another motor. Workflow: the fit's drag scale on the
  earlier flight's log is carried to the later flight; the fixture report sets it against the plain
  prediction and a by-hand ratio for both programs, every prediction sealed first. Moves: new rows
  `accuracy.second-flight` and `bias.second-flight`. *Done when:* [M2.16][m2-16]'s.
- SOON: the apogee in the morning's forecast air. User: a flyer setting a waiver ceiling or a contest
  target on launch morning, who has a forecast, not the reanalysis the collection's rows fly in. 50 of
  the collection's 74 flights not marked excluded have a launch time that isn't assumed (counted
  2026-11-22). Workflow: the fixture report flies each of them again in a GFS forecast issued before
  the launch hour, read from a public archive of past runs whose terms allow it (NOMADS, which
  `hpr-net` reads, keeps only recent ones), beside the same flight in the day's ERA5 air. Moves:
  `accuracy.collection` and `bias.collection`, as shown figures in forecast air beside the held ones.
  *Done when:* the rule fixes the forecast's lead time before any flight is flown; the report gives
  both programs' mean absolute and signed errors in both airs on the same flights, with each flight
  lacking an archived forecast counted by cause; and a test refuses a forecast issued after the launch
  hour.
- SOON: the Monte Carlo's drag scatter, measured. User: a flyer reading `hpr mc`'s 90% apogee range
  for a waiver. The drag's scatter is today a guessed spread around HPR Sim's own drag ([trusting a
  simulation](../trusting-a-simulation.md)). Workflow: from [M2.16a][m2-16a]'s fits, the report gives
  the fitted drag scale's spread across rockets (the drag error a first flight carries) and between
  flights of one rocket (what a carried fit leaves), each less the share a fit's own noise adds, read
  from the synthetic sets' fitted scales around their truths; [M2.8][m2-8]'s run takes the first as
  its drag scatter. A fitted scale also holds some of what the run scatters apart (mass, impulse,
  wind), so the spread is set beside those scatters, not added blind. Moves: `uncertainty.range-50`
  and `uncertainty.range-90`, HPR Sim's cells. *Done when:* both spreads are given with their count of
  rockets and a bootstrap 95% interval; the scatter is merged before any coverage is computed; and the
  report gives how many logged apogees fall in the 50% and 90% ranges, with Clopper–Pearson intervals,
  with the measured scatter and with today's, each rocket scored on a spread made without it.
- SOON: a rocket's log book, its next flight flown from its last. User: the flyer of [M2.16][m2-16],
  once its rows land. Workflow: `hpr` keeps a local file of one rocket's flights, each with its design
  file and log; asked for the next flight, it fits the latest log by [M2.16a][m2-16a]'s rule and flies
  the new plan with that drag scale, printing beside the apogee the committed `accuracy.second-flight`
  figure and the count of rockets it rests on. Moves: `accuracy.second-flight`, from a report's figure
  to a command a flyer runs. *Done when:* a test flies a synthetic pair through the command and
  matches the report's carried-fit apogee to the meter; the command refuses a log the fit refuses,
  with its cause; and its page gives the measured error with its count and date.

## MAINTAINER'S CALL (money, outreach)

- Bids to be the official simulator of US competitions.
- A paid team workspace (Excalidraw+ style).
- University validation partners.
- Disclosed buy links to motor.fusionspace.co, never affecting rankings.
- Verified kit badges, with kit makers.
- ThrustCurve: a data-quality give-back, and asking its maintainer about bundling common motors.
- A fiscal host or sponsor tiers (each funder's eligibility rules to be checked first).
- DROPPED by Neer (2026-10-05, the logs can't be had): asking SparkyVT for logs. Its flight computer's
  README claims 250+ flights to Mach 2.3, but no logs are published. Supersonic evidence comes from
  `hpr-sim-fixtures` instead (ADR-151).
- DROPPED by Neer (2026-10-28, [ADR-305][adr-305]): a migration layer for users of orhelper (a
  GPL-licensed Python bridge to OpenRocket). The `.ork` importer already serves those users.
- PROMOTED to [M10.1, release 0.1][adr-162]: prebuilt binaries through a release. The project prepares
  the workflow; publishing a release is Neer's.
- A "vendor confirmed" flag ([ADR-197][adr-197]). Asking parts vendors for their data was dropped by
  Neer (2026-10-28, [ADR-305][adr-305]): their published dimensions and masses are used as facts.
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
[m2-12]: ../decisions-and-roadmap.md#m2-12
[adr-347]: ../decisions/0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[m2-13]: ../decisions-and-roadmap.md#m2-13
[adr-352]: ../decisions/0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[m2-14]: ../decisions-and-roadmap.md#m2-14
[adr-360]: ../decisions/0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md
[m2-15]: ../decisions-and-roadmap.md#m2-15
[adr-370]: ../decisions/0370-the-2026-11-22-review-the-next-flight-predicted-from-the-last-ones-log.md
[m2-16]: ../decisions-and-roadmap.md#m2-16
[m2-16a]: ../decisions-and-roadmap.md#m2-16a
[m2-8]: ../decisions-and-roadmap.md#m2-8
