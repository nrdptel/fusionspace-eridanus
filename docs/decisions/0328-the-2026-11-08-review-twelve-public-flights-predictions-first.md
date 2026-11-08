# ADR-328: The 2026-11-08 review: twelve public flights, predictions first (2026-11-08)

- **Status:** accepted; promotes [M2.3d][m2-3d] to the queue, third, after [M2.7][m2-7], from the [ideas backlog][ideas] and issues [#478][i478] and [#1266][i1266], in two increments: the predictions merged first, then the errors.
- **Summary:** HPR Sim's accuracy against public real flights rests on 7 flights, all from RocketPy's examples. Five more public flights can join them: RocketPy's Astra, Andromeda and Erebus 11, once their netCDF-4 weather is read and their designs are built from the examples' inputs, and two MIT-licensed flights of one OpenRocket design. The new milestone merges each new flight's inputs and predicted apogee in a pull request of its own before any of their errors is computed, then puts `accuracy.public` and `bias.public` over the 12. It changes no model and loosens nothing; it raises how many real flights the public numbers rest on, from 7 to 12.

[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[m2-7]: ../decisions-and-roadmap.md#m2-7
[m7-5]: ../decisions-and-roadmap.md#m7-5
[ideas]: ../research/ideas.md
[i478]: https://github.com/nrdptel/fusionspace-eridanus/issues/478
[i1266]: https://github.com/nrdptel/fusionspace-eridanus/issues/1266
[pf]: ../research/public-flights.md
[pf26]: ../research/public-flights-2026.md
[real]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[era5]: ../glossary.md#era5
[netcdf]: ../glossary.md#netcdf
[cots]: ../glossary.md#cots-motor
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md

**Context.** This review raises one bar on accuracy against real flights. While there are no users
([ADR-322][adr-322]), that bar is the evidence behind [the scoreboard][score]. Its cells for the
public flights' mean absolute and mean signed apogee error, `accuracy.public` at 6.06% and
`bias.public` at −0.22%, come from [the real-flights report][real]: 7 flights, all from RocketPy's
examples, each flown in [ERA5][era5] weather (Prometheus's a year after its flight). Its private
rows rest on 60 flights that no reader can re-run. A mean over 7 flights leans on each one: NDRT
2020's +10.40% is 1.5 points of the 6.06% (10.40/7), and the other six average 5.34%.

Two surveys found five more public flights with a [COTS motor][cots] and an altimeter log
([public flights][pf], [public flights since October][pf26]). Others in those notes wait on a
confirmed ballast or motor, a weather file or a license, or may have flown air brakes or near
Mach 1.

| Flight | Motor | Log | What it waits on |
|---|---|---|---|
| Astra, EuRoC 2022 | Cesaroni L1350 | 64 rows, 1 Hz | its weather, a [netCDF-4][netcdf] file HPR Sim's reader refuses ([#478][i478]); a design from the example's inputs |
| Andromeda, EuRoC 2022 | Cesaroni M2020; the team's curve, 17% more impulse than the catalog's | 1,139 rows, 20 Hz to just past apogee, 2 Hz after | the same |
| Erebus 11, EuRoC 2022 | Cesaroni K660 (Inference: its curve integrates within 0.8% of the catalog's) | 746 rows, 20 Hz | the same |
| Macie XL, flight 1 | AeroTech I284W | StratoLogger `.pf2` | an ejection spike in the log; the logged apogee is 20% above the design file's own stored simulation ([#1266][i1266]) |
| Macie XL, flight 2 | Cesaroni 601I350 | StratoLogger `.pf2` | the logged apogee is 41% above the stored simulation ([#1266][i1266]) |

RocketPy's code is MIT. Its notebooks record that a named person gave permission to use the flight
data (14 of 17, these three among them), with no terms for anyone else, so, as the real-flights
report already does, the logs and the weather file are read from `refs/` and never committed; only
statistics and SHA-256 hashes are. The Macie XL design and logs are MIT
(`rlhatcher/rocketry-files`). The design file's stored simulations miss its logs by −16.4% and
−29.3%, which may point to a mass or a motor the file doesn't record (Inference: the repository
doesn't say which).

**Decision.**

1. **M2.3d is promoted, third in the queue, in two increments.** Its *done when* is both of:
   - **(a) M2.3d1, the predictions.** A pull request of its own, merged before any of the five
     new flights' errors is computed, commits a predictions file holding, for each of them: its
     public id; its design or inputs, motor and mass, each with its source, and never taken from
     its log; its reference apogee rule: the logger's own reported apogee, or for a log with an
     ejection spike, the reading that rejects the spike, named in the file; whether
     every input is traced to a record; and its predicted apogee. Astra, Andromeda and Erebus 11
     fly in their ERA5 air, the netCDF-4 file read by HPR Sim or converted by a committed, tested
     script, the converted file kept in `refs/` with its SHA-256 committed. Andromeda is counted
     on its team's curve, as the other RocketPy flights fly each example's own thrust file; its
     catalog curve's prediction is shown beside it and never counted.
   - **(b) M2.3d2, the errors.** `real-flights` flies all 12, prints each new flight's merged
     prediction beside its apogee and error, and fails when a merged prediction differs from the
     flown one without a "revised, because" entry that keeps both numbers. `accuracy.public` and
     `bias.public` are taken over all 12 and shown beside them over the flights whose inputs are
     all traced, as tagged in (a), and over the merged predictions where any was revised. Moves:
     `accuracy.public` and `bias.public`, from 7 flights to 12.
2. **Third, after M2.7, before M7.5.** Merged first, the new flights' predictions exist before
   M7.5's correction does. Their logged apogees are already in the research notes, though none has
   been adopted as a reference apogee under the predictions file's rule. Merging the predictions
   first guards against tuning HPR Sim to its own error, not against knowing the target, so each
   input must come from its named record. Pull requests here are squash-merged ([ADR-326][adr-326]),
   so only a merged pull request of its own shows the order on `main`. Any of the five that also sit
   in the private collection follow [ADR-322][adr-322]'s split there, so only those in its held-out
   half are blind to [M7.5][m7-5]'s fit.
3. **No flight is dropped for its error.** A flight whose record can't be traced is flown on what is
   stated and tagged; it stays in the 12. Leaving out the flights that miss most would flatter the
   mean absolute error, the worst side for an accuracy claim.

**Consequences.** Two P-high issues, #478 and #1266, become a milestone with a *done when*. The
public mean may rise: the Macie XL design file's own stored simulations miss by −16.4% and
−29.3%, and nothing here promises HPR Sim flies closer. The scoreboard's public rows then rest on 12 flights
instead of 7. This follows [ADR-209][adr-209] §4's order for new flights; because these flights are
public, the prediction itself is committed rather than kept private behind a hash.
