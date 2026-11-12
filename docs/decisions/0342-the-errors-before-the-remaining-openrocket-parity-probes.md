# ADR-342: The errors before the remaining OpenRocket-parity probes (2026-11-11)

- **Status:** accepted; carries out [M2.3d2][m2-3d2] under [ADR-328][adr-328] §1b, ahead of what is left of [M2.3d4][m2-3d4], where [ADR-334][adr-334] and [ADR-336][adr-336] had put it last. It finds [M2.3d1][m2-3d1] met and so [M2.3d][m2-3d] met, moves M2.3d4 with its open increments to a [queue][queue] line of its own, seventh, and amends M2.3d4's and [M2.3d9][m2-3d9]'s *done when* for the two lower-stage probes, which stay a pinned gap ([ADR-340][adr-340] decision 4).
- **Summary:** The five public flights whose predictions were merged first are now compared with their logs. `cargo xtask real-flights` flies all 12 public flights and prints each of the five merged predictions beside its log's apogee and its error: Astra −6.53%, Andromeda +4.14%, Erebus 11 −9.58%, and the two Macie XL flights −16.20% and −29.16%. Over the 12 the mean absolute apogee error is 9.00% and the mean with its sign is −4.91%; over the seven compared before, they were 6.06% and −0.22%. No prediction was revised. The work left on how an OpenRocket file's stage overrides are read moves behind the accuracy work: it checks HPR Sim against OpenRocket on probe designs, and the logs are what decide accuracy.
- **Scoreboard:** `accuracy.public.hpr` and `bias.public.hpr` are worse in release 0.2.

[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[m2-3d1]: ../decisions-and-roadmap.md#m2-3d1
[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d6]: ../decisions-and-roadmap.md#m2-3d6
[m2-3d8]: ../decisions-and-roadmap.md#m2-3d8
[m2-3d9]: ../decisions-and-roadmap.md#m2-3d9
[m2-3c9]: ../decisions-and-roadmap.md#m2-3c9
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m0-17]: ../decisions-and-roadmap.md#m0-17
[queue]: ../ROADMAP.md#queue
[real]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[predictions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/public-predictions.md
[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[i478]: https://github.com/nrdptel/fusionspace-eridanus/issues/478
[i1266]: https://github.com/nrdptel/fusionspace-eridanus/issues/1266
[i1424]: https://github.com/nrdptel/fusionspace-eridanus/issues/1424
[p1405]: https://github.com/nrdptel/fusionspace-eridanus/pull/1405
[p1414]: https://github.com/nrdptel/fusionspace-eridanus/pull/1414
[adr-240]: 0240-the-scoreboard.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-334]: 0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[adr-336]: 0336-m2-3d4-in-three-the-inertia-and-the-refusals-last.md
[adr-337]: 0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md
[adr-340]: 0340-a-stage-s-overrides-leave-its-parts-in-place-as-openrocket-weighs-them.md
[adr-341]: 0341-m2-3d8-in-five-a-parallel-stage-s-added-mass-first.md

**Context.** [ADR-328][adr-328] promoted twelve public flights in two increments: the five new
flights' predictions merged first, then their errors. The predictions merged in two pull
requests, the three EuRoC 2022 flights' in [#1405][p1405] and the two Macie XL flights' in
[#1414][p1414]. The Macie XL design carries a stage override that adds to its parts, which HPR
Sim could not fly, so [ADR-334][adr-334] put a milestone for reading such a stage, M2.3d4, inside
the predictions' increment, and the errors after all of it. Its first part, [a stage's added
mass][m2-3d6] ([ADR-335][adr-335]), is what the Macie XL predictions needed
([ADR-337][adr-337]).

M2.3d4 then grew. Probe designs showed more stage overrides that HPR Sim reads differently from
OpenRocket or refuses, and [ADR-336][adr-336] and [ADR-341][adr-341] split the work into more
increments: a parallel stage, parts that weigh nothing, flights that part a stage, and the
inertia on a lower stage and with a motor aboard. Ten of the 12 flights' designs are built from
RocketPy's examples, not read from an OpenRocket file. Each increment measures HPR Sim against OpenRocket 24.12 on a design made for the
purpose.

The maintainer asked whether that work is done because OpenRocket does it or because it is
right. Both hold, in different places. OpenRocket defines what an override in its own file
format means, so the reader follows it there. Whether HPR Sim's flights are right is decided by
logs. With every prediction merged, the errors against the logs were waiting behind work on
OpenRocket parity that no logged flight needs.

**Decision.**

1. **The errors go next.** M2.3d2 runs ahead of [M2.3d8][m2-3d8], its four open increments and
   M2.3d9. The order [ADR-328][adr-328] §2 asked for, each prediction merged before its error is
   computed, is kept: all five were on `main` before this record's pull request opened.
2. **M2.3d1 is met, and M2.3d4 leaves it.** M2.3d1's *done when* is that a pull request of its
   own merges the five flights' inputs, sources and predicted apogees, computing no error; the
   two pull requests above did. M2.3d4 keeps its open increments, each with the *done when* it
   has, and becomes a queue line of its own, seventh: after [M2.12][m2-12], descent rates
   against the logs, and before [M2.3c9][m2-3c9], every flight OpenRocket flies, which is the
   work its refusals bear on. With M2.3d1 and M2.3d2 met, M2.3d is met and leaves the queue;
   [M0.17][m0-17] is first.
3. **The two lower-stage probes stay a gap.** On two probe designs with a stage's override on
   the lower of two stages, HPR Sim's inertia across the axis is 4.95% and 3.33% over
   OpenRocket's. [ADR-340][adr-340] decision 4 measured why: read as the lower stage's own
   inertia about its center, OpenRocket's figure is below what the stage's tube alone has there,
   so no stage that keeps its parts in place gives it. HPR Sim does not reproduce a figure that no
   stage keeping its parts in place has. M2.3d9's *done when* loses that clause and keeps the other. M2.3d9 is now: *on the
   motor's airframe, under an added mass beside a center that covers the parts, the inertia
   across the axis with the motor aboard, at launch and at burnout, is held to OpenRocket's
   within 3 parts in a million, the motor's own term taken from OpenRocket's record.* M2.3d4's
   *done when* holds a stage's inertia to OpenRocket's on probe designs but for the inertia
   across the axis on those two. The two stay pinned at 4.95% and 3.33% over by the test that
   holds them now (`STAGE_WHOLE_BELOW`), so a change of 0.01 points or more in either direction fails, and the pages on
   the `.ork` format and on design overrides say the difference stands.
4. **How the errors are computed.**
   - `cargo xtask real-flights` flies the seven flights compared before, then the five
     predicted first, and writes one report. Each of the five is held to its merged prediction
     to the digit its page printed, 0.1 m. A prediction that differs fails the command and the
     committed report's check, unless the code's list of revisions holds an entry for the
     flight with the number flown now and a reason. The report then prints both numbers and
     both errors, and its summary adds the means over the predictions as first merged. The list
     is empty: no prediction was revised.
   - Each log's apogee is read by the rule merged with its prediction, now also written as
     code, and a test holds the code to the words. The three EuRoC 2022 logs give the highest
     reading of a named column, the log read whole: a row that isn't a number fails the read. The
     two Macie XL logs give the apogee in the PerfectFlite header, never the highest sample. The
     log must be the file whose SHA-256 was committed with the prediction.
   - The error is the prediction less the log's apogee, over the log's apogee. The prediction
     is read as the log's altimeter would read it, as it was merged.
   - `accuracy.public` and `bias.public` are the means over all 12. Beside them the report
     gives the means over the seven compared before, over the five predicted first, over the
     flights tagged as having every input traced, and over the flights flown in ERA5 weather.
   - The seven flights compared before carry no tag: they were compared before tags existed.
     Tagging them now, with their errors known, would let the tags choose the mean. Counting
     them untagged would count flights the five's own standard might tag *not traced*:
     Prometheus flies in another year's weather, and Juno III's thrust file is reshaped. So
     they are in neither tagged group, and the traced means are over the two flights tagged
     traced with their predictions, Astra and Erebus 11, as [ADR-328][adr-328] §1b asks.
   - Two kinds of number are shown beside a flight and never counted: Andromeda on its catalog curve,
     as ADR-328 asked, and for each Macie XL flight the apogee of the OpenRocket simulation
     stored in its design file.
   - No explanation is claimed for a predicted flight outside the 5% target. The report's
     two diagnostics, each example's own drag and Juno III's thrust file as recorded, are flown
     among the seven only, and the five have no trace comparison. What each of the five's records leaves
     untraced was written down with its prediction.
5. **The scoreboard's two public cells are accepted worse** ([ADR-240][adr-240]):
   `accuracy.public.hpr` goes from 6.06% over 7 flights to 9.00% over 12, and `bias.public.hpr`
   from −0.22% to −4.91%. No model changed, and the seven flights' numbers are the ones they
   had. The cells are worse because they now count five flights HPR Sim misses by more.

**The numbers.** Each apogee is as the log's altimeter reads it.

| Flight | Log's apogee | Merged prediction | Error | Every input traced |
|---|---:|---:|---:|---|
| Astra, EuRoC 2022 | 3,249.5 m (10,661 ft) | 3,037.1 m (9,964 ft) | −6.53% | yes |
| Andromeda, EuRoC 2022 | 3,438.5 m (11,281 ft) | 3,581.0 m (11,749 ft) | +4.14% | no |
| Erebus 11, EuRoC 2022 | 3,001.8 m (9,848 ft) | 2,714.1 m (8,904 ft) | −9.58% | yes |
| Macie XL, flight 1 | 842.8 m (2,765 ft) | 706.2 m (2,317 ft) | −16.20% | no |
| Macie XL, flight 2 | 977.8 m (3,208 ft) | 692.6 m (2,272 ft) | −29.16% | no |

| Over | Flights | Mean absolute error | Mean error with its sign |
|---|---:|---:|---:|
| all | 12 | 9.00% | −4.91% |
| the seven compared before | 7 | 6.06% | −0.22% |
| the five predicted first | 5 | 13.12% | −11.47% |
| tagged as having every input traced | 2 | 8.06% | −8.06% |
| flown in ERA5 weather | 10 | 6.27% | −1.35% |

Shown and never counted: Andromeda on the catalog Cesaroni M2020 curve reads 3,050.3 m
(10,008 ft), −11.29%. The OpenRocket simulations stored in the Macie XL design file read 704.6 m
(2,312 ft) and 691.6 m (2,269 ft), −16.40% and −29.26%.

**Consequences.**

- The public numbers rest on 12 flights and are worse. HPR Sim reads low on 7 of the 12 and on
  4 of the 5 it predicted first. [The scoreboard][score] and the site's pages that quote the
  public mean now give the 12; dated research notes keep their own counts.
- The Macie XL flights miss by the most, and the OpenRocket simulations stored in their design
  file miss by about as much. This record does not say why. Both flights are tagged *not
  traced*: no record gives the day's air, the rail or a weighed mass, and the reader warns that
  each motor's curve file disagrees with ThrustCurve.org's record of the motor. The traced means leave the two
  out, and the means over all 12 keep them ([ADR-328][adr-328] §3).
- Issues [#478][i478] and [#1266][i1266], which asked for these five flights to be scored,
  close when this merges.
- A predicted flight has no trace comparison and no diagnostic flight. An issue asks for both
  on the three EuRoC 2022 flights.
- Designs with the stage overrides M2.3d4 still has open are refused or held back as before,
  each with its reason, until the milestone's turn. [M2.3d9][m2-3d9] is smaller by one clause;
  [#1424][i1424] says so.
- A later change that moves one of the five predictions must add a revision with its reason,
  in the pull request that moves it.

**How it was checked.** `cargo xtask real-flights` flew the 12 and wrote [the report][real];
`cargo xtask real-flights --check` and `cargo xtask public-predictions --check` reproduce the
committed files, and [the predictions' JSON][predictions] is byte for byte what it was. The
errors above were worked a second way, outside the tool, from the committed predictions and the
five logs. Tests in `hpr-validate` hold the committed report to the code's merged numbers, to
the committed predictions' log digests and tags, and to its own rows; they fail a report that
drops its worst flight or moves a prediction by 1 m either way with no revision, and they pin
the revision rule on invented flights: a changed number with no entry, an entry with no reason,
an entry stating another number, an entry that changes nothing and two entries for one flight
each fail.
