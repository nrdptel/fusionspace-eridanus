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

[m6-3]: ../decisions-and-roadmap.md#m6-3
[arc27]: https://www.rocketrychallenge.org/wp-content/uploads/2027_AmericanRocketryChallenge_Rules.pdf
[irec27]: https://www.esrarocket.org/s/IREC-2027-Rules-and-Requirments-v10.pdf
[sl27]: https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf
[adr-286]: ../decisions/0286-the-2026-10-20-review-the-export-bar-tightened.md
[nf-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/normal-force-vs-mach.json
