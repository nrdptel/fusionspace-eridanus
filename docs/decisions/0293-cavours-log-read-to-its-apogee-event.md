# ADR-293: Cavour's log read to its apogee event (2026-10-25)

- **Status:** accepted; corrects the logged apogee of one of the seven public real flights ([issue #818][i818]); the simulation is unchanged
- **Summary:** Cavour's real-flight record reads its log only up to 23.6 s, the first row its ground station tags as the drogue event. Past it, most likely the ejection charge's pressure pulse pushes the radio telemetry's filtered height 4 m (13 ft) above the coast's peak. So the logged apogee falls to 2,785 m (9,137 ft), Cavour's error rises from +5.63% to +5.78%, and the mean absolute apogee error over the seven flights rises from 6.04% to 6.06%.
- **Scoreboard:** `accuracy.public.hpr` worse in release 0.2, by the reference's correction, not a model's change.

[i818]: https://github.com/nrdptel/fusionspace-eridanus/issues/818
[note]: ../research/cats-vega-height.md#4-cavour-and-genesis
[filtered]: ../research/cats-vega-filtered-column.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md

**Context.** Cavour (Politecnico di Torino, EuRoC 2023) is one of the seven public flights whose
logs score HPR Sim's apogees ([accuracy](../accuracy.md#real-flights)). Its log, RocketPy's
`polito/altimeter_cavour.csv`, is a CATS Vega flight computer's radio telemetry. It holds the
computer's Kalman-filtered height in whole meters and no pressure. The heights are most likely cut
toward zero, as they match that way on two sister flights' logs ([note][note], §3). The record
read the whole log, so its apogee was the log's highest row, 2,789 m (9,150 ft).

That row comes after apogee ([note][note], §4 and §5). The coast reaches 2,785 m at 23.1 to
23.2 s, with the logged speed falling from 1 to 0 m/s. The row at 23.6 s reads 2,781 m and is the
first one the ground station's log tags Drogue. The next two rows read 2,787 m and 2,789 m, then
the height falls. After apogee the Vega's filter stops using its accelerometer, and the rise is
read as the charge's pressure pulse moving the filtered height: the log carries no pressure to
show it directly. Lince's log shows the same rise after its event, and its record was already cut
just after the event, at 26.80 s.

On four sister flights with the same computer, the filtered peak up to the apogee event sits
within 0.6 m (2 ft) of a noise-free barometric apogee ([filtered column][filtered], §3 and §4).
So Cavour's coasting peak is within about 1.3 m (4 ft) of its own barometric apogee.

**Decision.** Cavour's record reads its log to 23.6 s (`until_s`), the drogue event's row. Its
note says why, and its altimeter text says what the log holds. A test reads the log's rows around
apogee through the record and pins the cut: the last row read is 23.6 s, and the highest is
2,785 m at 23.1 s.

**Consequences.**

- The [real-flight report][report] changes. Cavour's logged apogee goes from 2,789 m to
  2,785 m, its error from +5.63% to +5.78%, and its error on the team's own drag from −2.29% to
  −2.15%. The mean absolute apogee error goes from 6.04% to 6.06%, and the signed mean from
  −0.25% to −0.22%.
- The scoreboard's public accuracy cell is worse by 0.02 points. This is the guard of
  [ADR-240](0240-the-scoreboard.md) working as meant: the number moves because the reference
  was read wrongly, and this record names the cell.
- Pages that quote the mean, and `hpr`'s own note on how far to trust it, now quote 6.06%.
