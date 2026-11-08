# Ideas: competitions, certification and club operations

Part of the [ideas backlog](ideas.md) (2026-10-03). One line each, with its tier. Regulatory and
rules templates carry a source date, a stale-after date and "not legal advice".

## Competitions and certification (SOON, with [M6.3, competition rules][m6-3])

- SOON: report packs. NASA Student Launch: landing kinetic energy per section with a mass
  reconciliation, drift at 0, 5, 10, 15 and 20 mph, descent time, both ballast cases, and a second
  method. The Spaceport America Cup's IREC (Intercollegiate Rocket Engineering Competition): the
  Appendix A table and the expected score. A Level 3 certification packet. An FAA waiver packet,
  Class 3 impact areas included.
- SOON: a ballast-to-target solver.
- SOON: a declared apogee with its uncertainty, for IREC's Barrowman award (named for the author
  of the Barrowman stability method, and given for flight prediction; check the current rules).
- SOON: a payload environment report: peak g by phase, spin, the pressure and temperature
  timeline, vibration as gRMS (root-mean-square acceleration in g) scaled by Reynolds number
  ([doi 10.2514/1.A36444](https://doi.org/10.2514/1.A36444)).
- SOON: landing-survival g for eggs and payloads.
- SOON: an American Rocketry Challenge two-target tuner (the 2027 rules: a 37 to 40 s duration
  and 800 ft, or 244 m, for qualifying), with a day-of grid of ballast and launch angle.
- SOON: contest optimizer presets for the National Association of Rocketry (NAR).
- SOON: the low-g window.
- SOON: thermal sensitivity of the duration.
- SOON: requirements as code.
- SOON: RocketPy export raised in priority: the European Rocketry Challenge (EuRoC) requires a
  RocketPy file (its rules, §8.5).
- SOON: Student Launch's checks in one command. User: a Student Launch team's safety lead before a
  design review. Workflow: one `hpr` command runs the M6.3 preset on a design and prints every
  limit's reading against it, at both ballasts and four winds. Moves: a new steps row for that
  task, `steps.student-launch`, with each tool's count or *can't* by its docs. *Done when:* the row
  is in `validation/steps/`, HPR Sim's file is run by a test, and the scoreboard shows it.
- SOON: the American Rocketry Challenge's expected score. User: an ARC team tuning a design to the
  2027 rules (at most 650 g and 80 N·s, two eggs, an 800 ft qualifying goal, 37 to 40 s;
  [rules][arc27] 4.3 to 4.6). Workflow: `hpr mc` with an ARC preset prints the score's 50% and 90%
  ranges, as rules 4.5 and 4.6 score it. Moves: a new steps row, `steps.arc-score`, each tool's by
  its docs. *Done when:* tests pin the score at both edges of the duration window and either side
  of the altitude goal, the row is in `validation/steps/`, and the scoreboard shows it.

- SOON: a second-method report for Student Launch, whose reviews ask for data from a different
  calculation method. User: a team's simulation lead. Workflow: `hpr crosscheck design.ork` sets HPR
  Sim's apogee, rail-exit margin and speed, landing energies and descent time beside the file's
  stored OpenRocket results, each against its requirement in the [2027 handbook][sl27], each gap
  split by cause. Moves: a new steps row, `steps.sl-crosscheck`. *Done when:* the five numbers print
  for both tools on every public Student Launch design (count stated), tests pin them, and the page
  cites the handbook (source and stale-after dates) and names any HPR Sim feature its rule 1.1.1
  bars (none today).

## Club operations (LATER)

- LATER: a tablet check for the range safety officer (RSO) and launch control officer (LCO):
  distance tables by impulse, thrust-to-weight, wind, the site's edge and the waiver ceiling.
- LATER: a pre-flight failure-risk checklist from base rates.
- LATER: a flight-line layout planner.
- LATER: an FAA day-of kit (air traffic control notification, window log).
- LATER: a weather-hold board.
- LATER: heads-up flags for the LCO.
- LATER: an air-start approval packet.
- LATER: a recovery queue and lost-and-found.
- LATER: the safety codes' hardware rules as design checks.
- LATER: a heat-soak note.
- LATER: a spectator photo card.
- LATER: an onboard camera planner.

From the 2026-10-09 planning review:

- SOON: airbrakes failed as each rulebook says. User: a EuRoC or IREC team's airbrake lead.
  Workflow: `hpr mc` flies the controller with the brakes failed in each rule's state (EuRoC 2026:
  fully extended). Moves: apogee spread under each failure, not measured today. *Done when:* the
  run reports each state's 90% apogee range with Wilks bounds, each state cited to its rulebook
  with a source date and a stale-after date.

From the 2026-10-15 planning review:

- SOON: the 2027 IREC altitude score. User: an IREC team choosing a motor and a ballast for its
  target apogee. Workflow: `hpr mc` with an IREC preset prints the altitude score's 50% and 90%
  ranges by the [2027 rules][irec27] (§6.6.1, p. 28: 350 points times one less the miss over 30%
  of the target, given only within 30%; zero beyond is our reading), and Table 4-1's predicted
  flight data (p. 21): seven predicted values (liftoff thrust to weight, rail exit speed, least
  static margin, peak acceleration, peak speed, flutter speed, apogee) beside the declared target.
  Moves: a new steps row, `steps.irec-score`, each tool's by its docs. *Done when:* tests pin the
  score at the target (350), 10% short (233.3) and either side of the 30% edge; the preset cites
  the rules' date and a stale-after date; each of the seven values comes from a tested output;
  the scoreboard shows the row.

## From the 2026-10-26 planning review

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

## Neer's 2026-10-07 field tools

Placed by [ADR-194, the field tools][adr-194]; research in [field tools](suite-field-tools.md).

- PROMOTED: a launch checklist and guide built from the design, a check of altimeter settings
  against the simulated flight, and a ground-test log (M6.8, release 0.4); an equipment register
  and a cross-vendor frequency board (M9.7, after mobile); a ground-test bench (M13.7).
- LATER: the ground station reads other vendors' documented telemetry, such as Altus Metrum's
  configuration packet, to check settings over the air and list busy channels.
- LATER: a wireless firing box, after the wired one.
- LATER: camera recording reminders and a camera run-time budget against the pad hold.

## From the 2026-11-02 planning review ([ADR-318][adr-318])

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
  than 1%, aggregates only; while no build is published, it waits; [M2.3c3][m2-3c3] reuses the
  harness on the release. New beside [ADR-212][adr-212], which already holds HPR Sim to the newest
  release published before M2.7: a build before it ships, shown as a forecast, never held.

## From the 2026-11-04 planning review ([ADR-319][adr-319])

- SOON: fault odds from the field. User: a flyer reading the ranked causes of a flight that went
  wrong, and anyone judging [M7.4][m7-4]'s benchmark. Its 200 faulty flights are drawn with no
  stated mix, while clubs' launch reports put a motor fault on about 1.1% to 2.7% of H to O flights
  and a recovery fault on 3% to 9%, a missing main or chute the largest mode
  ([high-power outcome tallies](high-power-outcome-tallies.md), [MASA's recovery
  modes](masa-recovery-modes.md)). A ranking that is right on a uniform mix can still put the
  rare causes first on real flights. Workflow: the benchmark is run twice, on a uniform mix and on
  one drawn from a committed table of field rates, and the ranker may take the table as its prior.
  Moves: the proposed `diagnosis.top-1` and `diagnosis.top-3`, M7.4's 80% and 95%, on both mixes.
  *Done when:* the table is committed with Clopper–Pearson bounds, each rate marked as a floor
  where its source undercounts; the report gives top-1 and top-3 on both mixes; and a test pins
  that the prior never ranks a nominal flight first where the evidence alone ranks a recovery fault
  first, the flattering side.

[m6-3]: ../decisions-and-roadmap.md#m6-3
[arc27]: https://www.rocketrychallenge.org/wp-content/uploads/2027_AmericanRocketryChallenge_Rules.pdf
[irec27]: https://www.esrarocket.org/s/IREC-2027-Rules-and-Requirments-v10.pdf
[sl27]: https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf
[adr-286]: ../decisions/0286-the-2026-10-20-review-the-export-bar-tightened.md
[nf-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/normal-force-vs-mach.json
[adr-194]: ../decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-212]: ../decisions/0212-measure-against-openrockets-newest-release.md
[adr-318]: ../decisions/0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[m2-3c3]: ../decisions-and-roadmap.md#m2-3c3
[or-3431]: https://github.com/openrocket/openrocket/pull/3431
[adr-319]: ../decisions/0319-the-2026-11-04-review-the-rocketpy-descents-tightened.md
[m7-4]: ../decisions-and-roadmap.md#m7-4
