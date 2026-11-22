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

[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-360]: ../decisions/0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md
[adr-348]: ../decisions/0348-m7-5b-the-correction-fails-on-the-public-flights.md
[adr-352]: ../decisions/0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md
[m2-15]: ../decisions-and-roadmap.md#m2-15
[m2-15a]: ../decisions-and-roadmap.md#m2-15a
