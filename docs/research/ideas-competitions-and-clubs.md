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

[m6-3]: ../decisions-and-roadmap.md#m6-3
[arc27]: https://www.rocketrychallenge.org/wp-content/uploads/2027_AmericanRocketryChallenge_Rules.pdf
