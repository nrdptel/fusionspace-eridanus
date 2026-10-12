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
  readings ([M7.2][m7-2]) distrust GNSS until its vertical speed agrees with the barometer's.
  Moves: no scoreboard cell today (every compared apogee is barometric); it would let logs whose
  only apogee is GNSS join `accuracy.collection`. *Done when:* a synthetic log with that lag reads
  the barometric apogee within its stated error and a mutation trusting GNSS in boost fails a test.

[m7-1]: ../decisions-and-roadmap.md#m7-1
[m7-4]: ../decisions-and-roadmap.md#m7-4
[m7-2]: ../decisions-and-roadmap.md#m7-2
[gnss-thread]: https://www.rocketryforum.com/threads/mitigating-gnss-issues-in-boost.199286/
