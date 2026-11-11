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

## From the 2026-10-28 planning review

- SOON: the motor file's share of the collection's bias. User: a flyer filing for a waiver whose
  prediction rests on a curve file nobody checked. Workflow: the fixture report flies each logged
  flight of the collection again on its motor's bundled file that passes [ADR-005][adr-005]'s 1%
  check, where it has one (239 of 924 motors do), and prints each flight's apogee change and the
  mean. Moves: `bias.collection` (+7.76% for HPR Sim, +6.72% for OpenRocket) and
  `accuracy.collection` (12.07%, 11.26%), split by the files' share. *Done when:* the report gives
  the count of flights re-flown and the mean signed change, aggregates only; a test pins one public
  flight's swap; the scoreboard notes both figures beside the rows.
- SOON: landings inside the predicted ellipse. User: a flyer at a field whose recovery area ends at
  a road or a waiver boundary. Workflow: the fixture report compares each GPS-fixed landing in the
  collection with `hpr mc`'s 90% landing ellipse for that flight's weather, with and without a
  canopy glide term ([the canopy glide note](hobby-canopy-glide-and-swing.md)). Moves: new row
  `landing.inside-90`, nearer 90% better; OpenRocket 24.12 has no landing ellipse (can't), RocketPy
  has one. *Done when:* the row gives HPR Sim's share over every GPS-fixed landing it can fly, with
  the count, and the glide term's change; a synthetic flight in a known wind gets its ellipse by
  hand.

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
  6 more than 10% low), read off [the fixture report][fixture-report]'s histograms, and hpr 3 of
  7 public flights ([the real-flights report][real-report]). *Done when:* both rows read fields
  the reports write, a test pins that −5% exactly is not counted and −5.01% is, and the
  scoreboard's check fails when hpr's count rises.
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

## From the 2026-10-31 planning review ([ADR-310][adr-310], [ADR-311][adr-311])

- SOON: a second flight on the first flight's drag. User: a Level 3 flyer planning a second flight
  of a rocket with one good log. Workflow: fit the drag on the first log ([M7.3][m7-3]), then
  `hpr sim` the next flight with that fit, testing whether per-flight drag shrinks
  `bias.collection` (+7.76%). Moves: new row `accuracy.second-flight`. *Done when:* the row gives the count of designs with two
  logged flights and the error with and without the fit; a synthetic pair pins the fit; the row
  says so plainly where the count is too small to mean anything.
- SOON: the intervals' width, held. User: a flyer quoting a log's apogee with its interval. A noisier
  reader, once refitted, passes the coverage test ([ADR-308][adr-308]) on wider intervals. Workflow:
  the synthetic flights' report records each reading's mean width. Moves: new rows `readings.width`,
  lower better, and `readings.inside-90`, nearer 90% better. *Done when:* the report prints each reading's mean width on sets
  A and B, `synthetic --check` fails when one grows more than 10% over its committed figure, and a
  probe adding noise to one reading shows that the width check fails while coverage passes.
- PROMOTED ([M2.12][m2-12]): descent rates against the log. User: a flyer sizing a main or a drift budget. Workflow: the
  fixture report compares each tool's descent rate under the main with the log's. Moves: new row
  `accuracy.descent`, lower better; too gentle is the flattering side. *Done when:* signed and absolute errors with their counts for both simulators, aggregates only,
  and a test pinning which way too gentle reads.

## From the 2026-11-02 planning review ([ADR-318][adr-318])

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
[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[adr-005]: ../decisions/0005-solid-motors-statistics-consumption-grains-file.md
[adr-310]: ../decisions/0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-311]: ../decisions/0311-the-2026-10-31-review-every-flight-openrocket-flies.md
[m7-3]: ../decisions-and-roadmap.md#m7-3
[adr-308]: ../decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md
[adr-257]: ../decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-318]: ../decisions/0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[hfs]: https://github.com/aneeshkaravadi/hpr-flight-sim
[m2-12]: ../decisions-and-roadmap.md#m2-12
