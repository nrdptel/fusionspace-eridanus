# ADR-293: Cavour's log read to its apogee event (2026-10-25)

- **Status:** accepted; corrects the logged apogee of one of the seven public real flights ([issue #818][i818]); the simulation is unchanged
- **Summary:** Cavour's real-flight record reads its log only up to 23.6 s, the first row its ground station tags as the drogue event. After that row the radio telemetry's filtered height jumps 8 m (26 ft) in 0.2 s, faster than the rocket can climb, about 2 s before the drogue charge most likely fires. So the logged apogee is the coast's peak, 2,785 m (9,137 ft), not the jump's 2,789 m (9,150 ft). Cavour's error rises from +5.63% to +5.78%, and the mean absolute apogee error over the seven flights rises from 6.04% to 6.06%.
- **Scoreboard:** `accuracy.public.hpr` worse in release 0.2: the scoreboard's public accuracy cell moves from 6.04% to 6.06% because the reference was corrected, not a model.

[i818]: https://github.com/nrdptel/fusionspace-eridanus/issues/818
[note]: ../research/cats-vega-height.md#4-cavour-and-genesis
[filtered]: ../research/cats-vega-filtered-column.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[glossary]: ../glossary.md#drogue-and-main

**Context.** Cavour (Politecnico di Torino, EuRoC 2023) is one of the seven public flights whose
logs score HPR Sim's apogees ([accuracy](../accuracy.md#real-flights)). Its log, RocketPy's
`polito/altimeter_cavour.csv`, is what the team's flight computer, a CATS Vega, sent down by radio
during the flight. Each row is the computer's own estimate of its height, a Kalman filter blending
its barometer and its accelerometer, in whole meters. The log holds no pressure. On two sister
flights the same telemetry matches the computer's stored heights rounded down to whole meters, so
Cavour's rows are most likely rounded down too ([CATS Vega note][note], §3). The record read the
whole log, so its apogee was the log's highest row, 2,789 m (9,150 ft), which is also the team's
official apogee.

That row comes after apogee. The rows around it ([CATS Vega note][note], §4):

- In the CSV the record reads, the coast reaches 2,785 m at 23.1 and 23.2 s, with the logged
  speed falling from 1 to 0 m/s.
- The row at 23.6 s reads 2,781 m at −5 m/s. It is the first one the ground station's log tags
  with the drogue state ([drogue and main][glossary]).
- The next two rows read 2,787 m and 2,789 m, then the height falls. Rising 8 m in 0.2 s from a
  fall of 5 m/s isn't a motion the coasting rocket can make.
- The ground station's log also records each recovery channel's continuity, whether its igniter
  circuit is closed. Both channels read closed through 25.4 s, and the first reads open from
  25.6 s, 2.0 s after the drogue event. The second does the same 2.0 s after the main event. So
  the drogue charge most likely fires on a 2 s delay, about 1.8 s after the jump: the jump is not
  its pressure pulse. This is an inference: each channel's open spell lasts exactly 2.0 s, and
  the first opens again from 34.7 s, so what the column marks isn't documented. The jump's cause
  is open. One reading is a transient as the computer changes state and its filter stops using
  the accelerometer.

On four sister flights with the same computer, the filtered peak up to the apogee event sits
within 0.6 m (2 ft) of a noise-free barometric apogee, and the computer's height formula and the
1976 standard atmosphere's differ by up to 0.7 m (2 ft) ([filtered column][filtered], its summary
and §3). So Cavour's filtered peak, 2,785 m to 2,786 m before rounding down, is within about
1.3 m (4 ft) of its barometric apogee. The scored 2,785 m is then between 1.3 m (4 ft) above and
2.3 m (8 ft) below it. Against those two ends, Cavour's +5.78% would read +5.83% and +5.69%:
0.05 points one way and 0.09 the other.

**Decision.** Cavour's record reads its log to 23.6 s (`until_s`, the last time read), the
drogue event's row. Its note says why, and its altimeter text says what the log holds. A test
reads the log's rows around apogee through the record and pins the cut: the last row read is
23.6 s, and the highest is 2,785 m at 23.1 s. Cutting at 23.4 s, the last coasting row, would
give the same apogee; the event's row is where the issue set the cut, and the drogue tag marks it
in the team's own log.

Lince, the only other Vega log cut by hand, is cut at 26.80 s, 0.75 s after its event and just
before its filtered height swings by hundreds of meters. Cut at its event instead, its apogee reads 0.12 m
lower and the mean doesn't move at two decimals ([filtered column][filtered], §4), so it stays
as it is.

**Consequences.**

- The [real-flight report][report] changes. Cavour's logged apogee goes from 2,789 m to
  2,785 m, its error from +5.63% to +5.78%, and its error on the team's own drag from −2.29% to
  −2.15%. The mean absolute apogee error goes from 6.04% to 6.06%, and the signed mean from
  −0.25% to −0.22%.
- The scoreboard's public accuracy cell is worse by 0.02 points. This is the guard of
  [ADR-240](0240-the-scoreboard.md) working as meant: the number moves because the reference
  was read wrongly, and this record names the cell.
- Pages that quote the mean, and `hpr`'s own note on how far to trust it, now quote 6.06%.
