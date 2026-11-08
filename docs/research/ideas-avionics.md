# Ideas: avionics

Part of the [ideas backlog](ideas.md) (2026-10-03). One line each, with its tier. Several feed
[M6.4, airbrakes][m6-4], and the flight-log milestones [M7.1][m7-1] to [M7.4][m7-4].

- SOON: simulated sensor streams (barometer, accelerometer, gyro, GPS) for firmware and
  hardware-in-the-loop (HIL) tests.
- LATER: a two-way software-in-the-loop bridge to flight firmware.
- SOON (with [M6.4][m6-4]): a controller-in-the-loop Python API and a scoring harness; about 30
  student teams have built their own airbrake simulations.
- SOON: fault-injection Monte Carlo on deployment logic.
- SOON: altimeter settings export, with each device's real setting steps, and a Mach-delay
  advisor (how long an altimeter should ignore its barometer near Mach 1, where the pressure it
  reads jumps).
- SOON (with [M7.1 to M7.4][m7-1]): re-run deployment logic on a real flight log.
- SOON: sensor range checks: a barometer's ceiling (about 9,100 m, 30,000 ft, for the BMP280,
  BMP388 and BMP390) and accelerometer clipping.
- SOON: air-start tilt-lockout odds.
- SOON: staging and air-start timing, with the motor's pressure-up delay.
- SOON: predict what the altimeter will read (barometric against true altitude); contests score the
  altimeter, not the truth.
- LATER: an apogee-predictor benchmark.
- LATER: an onboard apogee predictor exported as C.
- SOON (with [M6.4][m6-4]): reference airbrake controllers: model-predictive control (MPC),
  proportional–integral–derivative (PID) and the linear-quadratic regulator (LQR).
- LATER: a pad-hold battery budget.
- LATER: an avionics data budget (sample rates against storage).
- LATER: a tracker antenna pointing table.
- SOON: the tilt-abort window. User: a flyer whose altimeter fires its drogue when the tilt passes
  90° within 2 s of liftoff, on by default in one maker's two lines. Why: in October 2026 flyers
  reported such early deployments on fast flights ([a forum thread][tilt-thread]). Workflow:
  `hpr mc` prints the tilt at the window's end (median, 90th percentile, largest) and how many runs
  cross the threshold inside it, with two-sided 95% Clopper–Pearson bounds, failed runs counted;
  window and threshold are inputs. Moves: honest uncertainty, and the features the scoreboard
  counts. *Done when:* `hpr mc` prints both; crossings are found on the continuous solution, not
  at output samples; a test pins that a run touching the threshold counts, with a mutation probe.
- SOON: an altimeter's events checked against its flight. User: a flyer whose charge fired at the
  wrong time, such as an apogee charge long after a tilt lockout tripped near liftoff
  ([a forum thread][tilt-thread], October 2026). Workflow: `hpr analyze` lists each event the
  logger recorded (launch, tilt lockout, burnout, apogee, deployments) beside the time the log's
  own motion puts it, and flags each disagreement. Moves: a new row, the share of logged events
  checked. *Done when:* a synthetic log with a lockout near liftoff is flagged and a clean one
  isn't (a missed flag, the flattering side, has its own test); the reports count events checked
  and flagged on the public logs and, as aggregates, the collection's.
- SOON: a static-port placement check through Mach 1. User: an L2 or L3 flyer drilling an avionics
  bay's ports on a rocket past Mach 0.95. Workflow: the design check lists each port's zone (behind
  the nose's shoulder, ahead of a fin root, boattail or transition, a single hole) with the measured
  range of false altitude, no verdict ([the static-pressure note][sphs]). Moves: `features.count`.
  *Done when:* each zone cites its source's table, and each flag fires just inside its edge and not
  just outside (a test and a mutation probe each).

## Neer's 2026-10-06 ideas

Placed by [ADR-191, the 2026-10-06 ideas][adr-191]; research in [the web tools](suite-web-tools.md),
[the repo layout](repo-layout.md), [watches](suite-watches.md) and
[the hardware line](suite-hardware-line.md).

- PROMOTED: the four web tools rebuilt in hpr (M9.6, after mobile); printed-part warnings
  (M8.3g); more tracker and flight-computer boards (M13.5).
- LATER: smartwatch companions, the phone as hub; researched again after M9.4.
- LATER: camera shroud kits; a camera shroud's drag as a protuberance, checked against
  OpenRocket's pods.
- LATER: drone and plane payloads released in flight, with a glide model after release; last of
  the hardware, since IREC bans them and drone rules don't fit a release at apogee.
- PROMOTED by [ADR-193][adr-193] (Neer, 2026-10-07): roll control on tail-fin tabs, then
  canards, roll only (M6.5, queued before M13.3); drag-only airbrake and roll-only tab firmware and
  hardware (M13.6; roll canards later), held for a ruling ([roll and drag control](roll-and-drag-control.md)).

## From the 2026-11-02 planning review ([ADR-318][adr-318])

- SOON: the intervals by sample rate. User: a flyer whose logger records at 10 or 15 Hz, quoting a
  log's apogee or top speed with its 90% interval. A 2026 preprint, down-sampling one logged
  flight, finds the apogee from its accelerometer ranging over 66 m at 10 Hz against 12 m at 50 Hz,
  while the barometer's stays within 0.3 m ([Varigonda][varigonda]).
  Workflow: the synthetic flights' report counts each reading's 90% coverage
  ([ADR-308][adr-308]) in three bands of the logger's rate: under 20 Hz, 20 to 100 Hz, and over
  100 Hz, each band drawn to hold at least 200 readings. Moves: the proposed
  `readings.inside-90`, by band. *Done when:* each band passes [ADR-257][adr-257]'s three counts
  on sets A and B, or the reading is withheld below the rate where it fails; a probe that fits the
  bound on the fastest band alone fails the slowest.

## From the 2026-11-04 planning review ([ADR-319][adr-319])

- SOON: the altimeter's own delay at deployment. User: a dual-deploy flyer setting an apogee delay
  on a Raven, a Blue Raven or an Eggtimer. Logs show each family fires at its own lag after the
  peak: PerfectFlite about 0.25 s early, RRC3 and AltOS on it, Raven 0.9 s late, Blue Raven 0.5 s
  late with a tail to 4 s, Eggtimer 0.9 s to nose-over before its typed delay
  ([apogee fire latency](apogee-fire-latency-in-logs.md)). A model that adds the typed delay alone
  reads the speed at deployment low by about 9 m/s for a Raven or an Eggtimer, and up to 40 m/s if
  Blue Raven's tail is lag rather than a wait typed into it (g·t, drag left out, an upper bound): the
  flattering side for a charge and a harness. Workflow: [M6.8][m6-8]'s settings check and
  M7.4's late-drogue model time the drogue at apogee plus the typed delay plus the family's
  measured lag, drawn from its spread. Moves: the proposed `recovery.deploy-speed`, the speed at
  drogue deployment, predicted less logged. *Done when:* the check prints that speed's range for the
  altimeter named; on the public and the collection's logs that time their charge, its 90% range
  passes [ADR-257][adr-257]'s three counts (aggregates only); and a test fails when the lag is
  dropped.
- SOON: readings that survive a sensor fault. User: a flyer whose log has a dropout, an
  accelerometer pinned at its range or a pressure spike at ejection, quoting the log's apogee or
  top speed with its interval. One new open-source ground station tests flight computers by
  injecting such faults ([Aviary][aviary]); no flight-log tool reports how its readings hold up
  under them. Workflow: the synthetic flights' report injects four faults, one at a time: a 0.5 s
  dropout in the climb, the accelerometer clipped at 16 g, an ejection spike in the pressure, and
  the barometer's error near Mach 1, and counts each reading's 90% coverage under each. Moves: the
  proposed `readings.inside-90`, by fault. *Done when:* each fault's set, of at least 200 readings,
  passes [ADR-257][adr-257]'s three counts, or the reading says it is withheld under that fault; and
  a probe that ignores the clipping fails the clipped set.
- SOON: the mass a log implies. User: a flyer whose simulated apogee missed the logged one by 10%,
  asking why. Forum threads often name a wrong lift-off mass as a cause of such a gap, and
  the collection's error is where HPR Sim trails OpenRocket (12.07% against 11.26%). Workflow: `hpr
  compare` prints the lift-off mass the boost implies, from [M7.3][m7-3]'s fit, beside the design's,
  with its 90% interval, and the apogee simulated at that mass. Moves: `accuracy.collection`, as a
  shown diagnosis, never held: the share of the collection's error a mass outside its interval
  explains. *Done when:* on synthetic flights with a known 5% mass error the implied mass passes
  ADR-257's three counts; the report gives, as aggregates, how many of the collection's flights
  imply a mass outside its interval and the mean absolute apogee error with and without them.

[m6-4]: ../decisions-and-roadmap.md#m6-4
[m7-1]: ../decisions-and-roadmap.md#m7-1
[m7-4]: ../decisions-and-roadmap.md#m7-4
[tilt-thread]: https://www.rocketryforum.com/threads/emergency-deployment-option-for-unstable-flights.194466/
[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-193]: ../decisions/0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md
[sphs]: static-pressure-hobby-shapes.md
[adr-257]: ../decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-308]: ../decisions/0308-m7-2e-in-four-the-intervals-by-reading-then-the-surfaces.md
[adr-318]: ../decisions/0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[varigonda]: https://doi.org/10.31224/8645
[adr-319]: ../decisions/0319-the-2026-11-04-review-the-rocketpy-descents-tightened.md
[aviary]: https://github.com/Birdbrain-Aviation/aviary
[m6-8]: ../decisions-and-roadmap.md#m6-8
[m7-3]: ../decisions-and-roadmap.md#m7-3
