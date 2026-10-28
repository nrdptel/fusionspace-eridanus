# Ideas: flight data

Part of the [ideas backlog](ideas.md) (2026-10-03). One line each, with its tier. All are SOON,
with the flight-log milestones [M7.1][m7-1] to [M7.4][m7-4] (reading logs, readings and
reconstruction, a flight against its simulation, fault diagnosis).

- SOON: a log-health card, in the style of PX4's Flight Review (PX4 is an open-source drone
  autopilot; Flight Review is its web log checker).
- SOON: spectrograms (motor chuffing, flutter, roll resonance).
- SOON: measured stability from the gyro's pitch oscillation.
- SOON: system identification of the normal-force slope, pitch and roll damping, with error bars.
- SOON: drag fitted separately for the burn and the coast.
- SOON: drag from apogee-only altimeters ([arXiv 2512.22248](https://arxiv.org/abs/2512.22248)).
- SOON: wind estimated during the ascent
  ([MDPI *Atmosphere* 12(4):470](https://www.mdpi.com/2073-4433/12/4/470)) and from the GPS
  descent.
- SOON: the thrust curve recovered from the inertial measurement unit (IMU) against the certified
  curve, and the apogee spread across a motor's curve files.
- SOON: sensor calibration across a season.
- SOON: video-to-log sync by sound, and a telemetry overlay.
- SOON: event-aligned delta traces between flights.
- SOON: a multi-flight workspace, in the style of MoTeC (motorsport data-analysis software).
- SOON: record-ready KML (the map format Google Earth reads).
- SOON: AltosUI CSV and Blue Raven text import (documented formats only; AltOS is GPL).
- SOON: motor failure counts from MESS, NAR's Malfunctioning Engine Statistical Survey (cache
  only; the license is unclear).
- SOON: incident report pre-fill.
- SOON: a season flight recorder with NAR's (the National Association of Rocketry's) outcome codes.
- SOON: a "flight package" archive format.

From the 2026-10-10 planning review:

- LATER: the flight against its own log, by phase. User: an L2 flyer whose apogee came in low.
  Workflow: `hpr compare` splits the miss into thrust, coast-drag and recovery parts, by M2.3c6's
  method. Moves: the collection's MAE, through the part it names. *Done when:* each collection
  flight with a log M7.1 reads gets the three parts, aggregated in the fixture report.
- LATER: GNSS altitude distrusted during boost. User: a flyer reading back a log with GNSS and
  barometric altitude. Why: in October 2026 a flyer reported a receiver's altitude 440 m behind
  the rocket in boost under a valid 3-D fix ([a forum thread][gnss-thread]). Workflow: the
  readings ([M7.2][m7-2]) distrust GNSS until its height, not only its vertical speed, agrees
  with the barometer's over a window: on a real track a one-point speed test passed at apogee
  with the height 1,392 m off ([the Eggfinder note][eggfinder]).
  Moves: no scoreboard cell today (every compared apogee is barometric); it would let logs whose
  only apogee is GNSS join `accuracy.collection`. *Done when:* a synthetic log with that lag reads
  the barometric apogee within its stated error and a mutation trusting GNSS in boost fails a test.
- SOON: one altitude formula for every logged apogee ([#701][i-701]). User: a flyer comparing two
  altimeters on one flight, or a log with a simulation. Workflow: `hpr analyze` derives height from
  logged pressure by the standard atmosphere from the pad, beside the logger's column, each family's
  conversion named sourced or assumed. Moves: `accuracy.public`, `accuracy.collection`,
  `bias.public` and `bias.collection`, their reference apogees on one formula. *Done when:* #701's,
  and the real-flight and fixture reports give those rows both ways.

From the 2026-10-20 planning review ([ADR-286][adr-286]):

- SOON: a canopy's drag area fitted from the flyer's own log. User: an L2 flyer whose main lands
  faster or slower than the maker's table. Workflow: `hpr analyze` on a barometric log, canopy and
  descent mass named, prints C_D·S with a 90% interval from each canopy's descent rate ([M7.2][m7-2])
  beside the maker's figure. Moves: a new row, `accuracy.canopy-cd` (one maker's canopies read 17%
  to 36% under their table on 29 balloon descents, [the parachute note][hpcd]). *Done when:* synthetic descents of known
  C_D·S are recovered inside the interval at its stated rate, and a public log gets its C_D·S.
- SOON: the stability margin through the flight, with each change's cause. User: a flyer whose
  rocket passes Mach 1 and whose tool shows 2.6 calibers growing to 4.8 with no reason given ([a
  forum thread][stab-thread], 2026-10). Workflow: `hpr sim` prints the margin at rail exit, top Mach,
  least and greatest, and splits each change into the CG's move and the CP's by part (nose, body
  lift, fins' lift slope). Moves: `features.count`. *Done when:* on a public supersonic design the
  parts sum to the change to rounding, and a mutation dropping one part fails the test.
- SOON: a past day's ERA5 column with no account. User: a flyer comparing an old flight's log with
  its simulation in that day's air. Workflow: `hpr weather era5` with a site and a UTC hour, no
  file, asks NSF NCAR's GDEX copy of ERA5 for the pressure levels (CC BY 4.0, no login, about 2 KB),
  caches them with the credit, and `hpr sim` flies them offline ([the note][slth]). Moves: a new
  task under `steps.command-line`, "fly a past launch in its day's air". *Done when:* a recorded answer
  replays with the ERA5 file's levels to rounding, and offline the cache answers or it says so.

From the 2026-10-24 planning review ([ADR-289][adr-289]):

- SOON: a second observable for each logged flight. User: a university team's simulation lead who
  must know whether a high apogee came from too much impulse or too little drag. Workflow: the
  real-flight and fixture reports score time to apogee (listed today, unscored, for hpr only) and
  burnout speed (logs with an accelerometer) for both tools; a high apogee with a high burnout
  speed points at thrust or mass, one with the log's burnout speed at coast drag. Moves: new rows
  `accuracy.time-to-apogee` and `accuracy.burnout-speed`, signed and mean absolute, with counts.
  *Done when:* both reports give the rows for both tools, and in a test synthetic flights with 10%
  more impulse and with 10% less drag are told apart by their burnout-speed error.
- SOON: where the miss lives. User: an L1 flyer asking how far to trust a prediction for a rocket
  like theirs. Workflow: the fixture report splits the apogee error by top Mach band, impulse class
  and body diameter; a cell of 8 or more flights from 5 or more designs shows its signed and mean
  absolute error with a 90% bootstrap interval, any other only its count; `hpr sim` names a
  design's cell. Moves: `accuracy.collection` and `bias.collection` by cell, naming which carry
  the +8.73% bias. *Done when:* the report gives every cell or its count, a test pins both size
  rules, and the page names each cell whose interval excludes zero, or says none does.

## From the research notes of 2026-10-28

- LATER ([M7.4, fault diagnosis](../decisions-and-roadmap.md#m7-4)): a one- or two-sample pressure
  dip at a firing that both altimeters in a bay see is most likely gas past an imperfect seal; the
  analyzer names it as a build note, rejects it before any apogee
  ([the ejection-dip note](ejection-dip-cause.md)).
- LATER (M7.4): a SiRF-based GPS receiver (A2235-H) can hold its height through the climb and peak
  low or high, so its altitude isn't an apogee; a launch detector reading its speed should expect
  a second or two of zero after liftoff ([the SiRF note](sirf-vertical-stall.md)).

## From the 2026-10-26 planning review

- SOON: how often each tool comes in low. User: a flyer filing for a waiver, and the RSO who checks
  it; a prediction below the log is the worst error ([ADR-144][adr-144]). Workflow: the scoreboard
  counts, from the reports, the logged flights each tool predicts more than 5% below the log (an
  early deployment only lowers a log, so it never explains one). Moves: new rows
  `undershoot.collection` and `undershoot.public`; today hpr 10 of 53 and OpenRocket 11 of 53 (4 and
  6 more than 10% low), hpr 3 of 7 public flights, read off [the fixture report][fixture-report]'s
  histograms. *Done when:* both rows read fields the reports write, a test pins that −5% exactly is
  not counted and −5.01% is, and the scoreboard's check fails when hpr's count rises.
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

[m7-1]: ../decisions-and-roadmap.md#m7-1
[m7-4]: ../decisions-and-roadmap.md#m7-4
[m7-2]: ../decisions-and-roadmap.md#m7-2
[gnss-thread]: https://www.rocketryforum.com/threads/mitigating-gnss-issues-in-boost.199286/
[eggfinder]: forum-eggfinder-gpx-vs-baro.md
[i-701]: https://github.com/nrdptel/fusionspace-eridanus/issues/701
[adr-286]: ../decisions/0286-the-2026-10-20-review-the-export-bar-tightened.md
[hpcd]: hobby-parachute-cd.md
[stab-thread]: https://www.rocketryforum.com/threads/stability-concerns-regarding-my-mach-1-l2.199218/
[slth]: sl-true-height.md
[adr-289]: ../decisions/0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[adr-144]: ../decisions/0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
