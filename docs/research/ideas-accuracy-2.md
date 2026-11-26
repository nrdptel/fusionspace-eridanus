# Ideas: accuracy against real flights, continued

Part of the [ideas backlog](ideas.md); it carries on [the first accuracy note](ideas-accuracy.md),
which reached its length. Ideas whose measure is one of the [scoreboard][score]'s rows against
logged flights. Each has its tier, its user, its workflow, the row it moves and a *done when*.

## From the 2026-11-19 planning review ([ADR-360][adr-360])

The review promoted a fourth idea to [M2.15, drag in the coast against the logs][m2-15]. The three
below are SOON.

- SOON: the impulse a motor delivered, read from its boost. User: a flyer whose rocket fell short on
  a motor they trust. The apogee fit put the collection's bias on the thrust, an impulse scale of
  0.94 ([ADR-348][adr-348]), with no reading of the thrust itself. Workflow: over the burn the
  thrust is the mass times the accelerometer's [specific force](../glossary.md#specific-force) plus
  the axial drag; the analyzer sums it, with the plan's mass less the propellant the motor file has
  burned, and the drag from [M2.15][m2-15]'s coast reading at the same Mach numbers, its base under power
  from a cited source; the fixture report sets that impulse against the motor file's. Moves:
  `bias.collection`, as a diagnosis of its thrust part. *Done when:* the reading holds a synthetic
  boost's impulse in its 90% interval, the sensor's scale and the plan's mass in it, on at least 90%
  of a set of 200; the report gives delivered over rated impulse, median and 90% range with counts,
  by maker (five flights or more), all together and on the weighed-mass flights.
- SOON: the coast's drag from a barometer alone. User: a flyer whose altimeter logs pressure but no
  acceleration; 35 of the collection's 83 flights list none ([ADR-352][adr-352] counts 48 with one).
  Workflow: the analyzer reads the drag over the mass along the path from the barometer's height
  over the coast below Mach 0.8, by the vertical equation, the height's second derivative plus
  gravity over the cosine of the path's angle from vertical (heights from the day's air, the angle
  in the interval). Moves: `accuracy.coast-drag`, over more flights, once turned from drag along the
  path to axial force. *Done when:* the reading passes [M2.15a][m2-15a]'s three lines on synthetic
  coasts; on the flights with both channels, the share whose barometer reading lies inside the
  accelerometer's interval is given with its count and the rows show each kind apart.
- SOON: the collection's errors with each source counted once. User: a reader asking whether the
  +7.72% bias (2026-11-16; +8.01% since ADR-362) is the model's or a few teams' and flyers' habits. In an earlier count of 53 flights,
  15 of the 19 whose charge fired or that split before the peak came from 2 of 16 sources ([apogee
  bias](apogee-bias.md)). Workflow: beside each flight-weighted mean, the fixture report gives the
  mean over sources, each source's flights averaged first, with a 95% interval from a bootstrap that
  resamples sources, for both programs. Moves: `accuracy.collection` and `bias.collection`, as shown
  figures beside the held ones. *Done when:* a test pins the weighting on a synthetic set where one
  source of ten flights carries the flight-weighted mean; the report gives both means and their
  intervals for both programs; the accuracy page says which one the held cell reads.

## From the 2026-11-22 planning review ([ADR-370][adr-370])

That review promoted the next flight predicted from the last one's log to [M2.16][m2-16]. The three
below are SOON.

- SOON: the apogee in the morning's forecast air. User: a flyer setting a waiver ceiling or a contest
  target on launch morning, who has a forecast, not the reanalysis the collection's rows fly in. 48 of
  the collection's 72 flights not marked excluded have a launch time that isn't assumed (counted
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

[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-360]: ../decisions/0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md
[adr-348]: ../decisions/0348-m7-5b-the-correction-fails-on-the-public-flights.md
[adr-352]: ../decisions/0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
[adr-370]: ../decisions/0370-the-2026-11-22-review-the-next-flight-predicted-from-the-last-ones-log.md
[m2-16]: ../decisions-and-roadmap.md#m2-16
[m2-16a]: ../decisions-and-roadmap.md#m2-16a
[m2-8]: ../decisions-and-roadmap.md#m2-8
