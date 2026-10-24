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
