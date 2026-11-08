# ADR-328: The 2026-11-08 review: twelve public flights, predictions first (2026-11-08)

- **Status:** accepted; promotes [M2.3d][m2-3d] to the queue, third, after [M2.7][m2-7], from the [ideas backlog][ideas] and issues [#478][i478] and [#1266][i1266].
- **Summary:** HPR Sim's accuracy against public real flights rests on 7 flights, all of RocketPy's examples. Five more are public with a design, a COTS motor and an altimeter log: RocketPy's Astra, Andromeda and Erebus 11, which wait only on their netCDF-4 weather, and two MIT-licensed flights of one OpenRocket design. M2.3d flies all 12, writes each new flight's predicted apogee down before its error is computed, and puts `accuracy.public` and `bias.public` over the 12. It changes no model and loosens nothing; it raises how many real flights the public numbers rest on, from 7 to 12.

[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[m2-7]: ../decisions-and-roadmap.md#m2-7
[m7-5]: ../decisions-and-roadmap.md#m7-5
[ideas]: ../research/ideas.md
[i478]: https://github.com/nrdptel/fusionspace-eridanus/issues/478
[i1266]: https://github.com/nrdptel/fusionspace-eridanus/issues/1266
[pf]: ../research/public-flights.md
[pf26]: ../research/public-flights-2026.md
[real]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md

**Context.** A planning review raises one bar on accuracy against real flights. While there are no
users ([ADR-322][adr-322]), that bar is the evidence behind the scoreboard. Its public rows,
`accuracy.public` at 6.06% and `bias.public` at −0.22%, come from [the real-flights report][real]:
7 flights, each one of RocketPy's examples flown in its ERA5 air. Its private rows rest on 60 flights
that no reader can re-run. A mean over 7 flights moves by about a seventh of any one flight's error:
NDRT 2020's +10.40% alone is 1.5 points of the 6.06%.

Two surveys found five more public flights that HPR Sim can fly with what it has
([public flights][pf], [public flights since October][pf26]):

| Flight | Motor | Log | What it waits on |
|---|---|---|---|
| Astra, EuRoC 2022 | Cesaroni L1350 | 64 rows, 1 Hz | its weather, a netCDF-4 file HPR Sim's reader refuses (#478) |
| Andromeda, EuRoC 2022 | Cesaroni M2020; the team's curve, 17% more impulse than the catalog's | 1,139 rows, 20 Hz | the same |
| Erebus 11, EuRoC 2022 | Cesaroni K660 (Inference: its curve integrates within 0.8% of the catalog's) | 746 rows, 20 Hz | the same |
| Macie XL, flight 1 | AeroTech I284W | StratoLogger `.pf2` | an ejection spike in the log; the design's stored run is 20% under the logged apogee (#1266) |
| Macie XL, flight 2 | Cesaroni 601I350 | StratoLogger `.pf2` | the stored run is 41% under (#1266) |

RocketPy's flights are MIT, like their weather file; the Macie XL design and logs are MIT
(`rlhatcher/rocketry-files`). The gaps of 20% and 41% between the logs and the design file's own
stored simulations point to a mass or motor record the file doesn't hold, which is the error a
simulator user meets most. These flights test it in the open.

**Decision.**

1. **M2.3d is promoted, third in the queue.** Its *done when*: `real-flights` flies the 12 public
   flights, today's 7 with Astra, Andromeda and Erebus 11 in their ERA5 air (the netCDF-4 file read
   by HPR Sim, or converted by a committed, tested script) and the two Macie XL flights from their
   design and `.pf2` logs. Each new flight's motor and mass are stated with their source, Andromeda's
   flown on its team's curve and on the catalog's. Each new flight's inputs and predicted apogee are
   committed before its error is computed, so the history shows the order. A later change of input
   keeps both numbers and says why. `accuracy.public` and `bias.public` are taken over all 12, and,
   shown beside them, over the flights whose inputs are all traced to a record. Moves:
   `accuracy.public`, from 7 flights to 12.
2. **Third, after M2.7, before M7.5.** [M7.5][m7-5]'s correction is fitted on the collection's
   calibration half and judged on its held-out half. Flown and committed first, the five new public
   flights are blind to that fit too, and add to what judges it in the open.
3. **No flight is dropped for its error.** A flight whose record can't be traced is flown on what is
   stated and marked; it stays in the 12. Leaving out the flights that miss most would flatter the
   mean, the worst side for an accuracy claim.

**Consequences.** Two P-high issues, #478 and #1266, become a milestone with a *done when*. The
public mean may rise: the Macie XL flights' stored runs miss by 20% and 41%, and nothing here
promises they fly closer. The scoreboard's public rows then rest on 12 flights instead of 7. Each new
flight's prediction is in the history before its error, as [ADR-209][adr-209] §4 asks of new flights.
