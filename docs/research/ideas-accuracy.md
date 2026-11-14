# Ideas: accuracy against real flights

Part of the [ideas backlog](ideas.md). Ideas whose measure is one of the
[scoreboard][score]'s rows against logged flights, kept apart from the
[flight data](ideas-flight-data.md) note, which holds the log reader's and the analyzer's. Each has
its tier, its user, its workflow, the row it moves and a *done when*.

## From the 2026-11-08 planning review ([ADR-328][adr-328])

The review promoted a fourth idea to [M2.3d, twelve public flights][m2-3d]. The three below are
SOON.

- SOON: the flyer's own run as a baseline. User: a flyer who already simulates in OpenRocket, asking
  whether HPR Sim tells them more than the run they made with their own settings. 237 of the private
  collection's 240 distinct design files (by SHA-256) store at least one OpenRocket run with an
  apogee (counted 2026-11-08); the scoreboard compares HPR Sim with OpenRocket 24.12 re-run in a
  standard atmosphere through the pad's temperature and pressure, not with what the flyer saw.
  Workflow: the fixture report reads the stored run of each flown configuration, with the version
  and settings it records, and gives its apogee error beside HPR Sim's and the re-run's. A stored
  run may postdate the flight and be tuned to it, which the file can't show, so the column is shown,
  never held. Moves: `accuracy.collection`, a shown column. *Done when:* the report gives the stored
  runs' mean absolute and signed errors on the flights that have one, how many have none, and a
  paired bootstrap 95% interval on HPR Sim's error less theirs, as aggregates.
- SOON: a public case another open simulator scored. User: anyone comparing simulators on the same
  public flights rather than on each tool's own claims. A new Apache-2.0 simulator scored itself
  blind on three flights of one kit from a college course's flight archive (DOFPro), a Madcow Mini
  DX3 on a G77R, a G79W and an H128W: +7.0%, +6.1% and +10.2%, each a fail by its own test
  ([Astraea's accuracy page][astraea], read 2026-11-08). HPR Sim reads DOFPro logs but has no public
  apogee case for that kit. Workflow: the kit built from its maker's published dimensions and
  masses, flown on the three motors in the archive's stated conditions, each prediction merged
  before its error, as [M2.3d][m2-3d] orders. Moves: a cited comparison beside `accuracy.public`,
  per flight only if the archive's license allows (it names "Creative Commons", no variant). *Done
  when:* the three flights are flown with each input's source and the archive's license recorded
  (aggregates only if it is unclear), and the report gives HPR Sim's errors beside the other tool's,
  dated.
- SOON: a part's mass held to its material. User: a flyer whose design holds a mass typed ten times
  off, as one recovery tool's catalog did until a 2026-11-07 test release, listing a 462 g parachute
  at 46 g ([its release notes][recsys]). A bundled motor's mass is checked against its record
  ([#1233][i1233]); a part's typed or library mass isn't checked against its size. Workflow: at
  import, each part's mass is set against its volume times the density range of its stated material,
  or of any hobby material when none is stated, and a mass outside it is named with the part, the
  mass and the range. Moves: `accuracy.collection`, as a shown diagnosis: the mean absolute apogee
  error of the flights with a flagged part beside the rest. *Done when:* the density ranges are
  committed with their sources; the count of the collection's designs with a flagged part is
  reported, as an aggregate; a test fails when a tenfold slip either way in a body tube's or a
  parachute's mass goes unflagged; and the share of ordinary parts flagged is counted, as a
  false-alarm rate.

## From the 2026-11-11 planning review ([ADR-338][adr-338])

The review promoted a fifth idea, descent rates against the logs, to [M2.12][m2-12]. The four
below are SOON.

- SOON: RocketPy's own apogee table beside HPR Sim's. User: anyone comparing simulators on the same
  public flights. An open RocketPy pull request commits a table of apogee errors on 11 of its
  example rockets, each flown on the drag its example supplies: median absolute error 6.36%, 90th
  percentile 8.42% ([its table][rp1208], committed 2026-10-24, read 2026-11-11; unmerged). Six of the
  11 are among HPR Sim's 7 public flights and three more join with [M2.3d][m2-3d]. Workflow: the
  real-flights report shows that table's error for each flight both fly, beside HPR Sim's on its own
  aerodynamics and on the example's drag, cited and dated. Moves: `accuracy.public`, a cited figure
  beside its RocketPy cell until [M2.9][m2-9] measures it. *Done when:* the report gives the cited
  error per shared flight and its mean, with the table's commit, and a test fails when the pinned
  table's hash changes.
- SOON: a third tool on the supersonic list, and the finish it assumes. User: a flyer past Mach 1
  asking which simulator to trust. Pocket Rocket Flight Lab scores itself on 23 measured apogees
  from RASAero II's example files: its mean absolute error is 9.9%, +9.2% with its sign, and 13.0%
  on the 9 flights above Mach 1.2; it gives RASAero II's own as 5.4%. It names finish as the largest unknown: at regular paint, 60 µm,
  its signed error falls to +0.5% ([its validation page][prfl], read 2026-11-11). Workflow: where
  [M2.3c7][m2-3c7] scores those flights, the report cites that tool's errors beside RASAero II's and
  flies each flight at the file's finish and at two named finishes. Moves: `accuracy.supersonic`,
  with its spread across finishes. *Done when:* the report gives the three tools' errors on the
  flights all score, and HPR Sim's at each finish, each finish's roughness sourced.
- SOON: the published apogee beside the log's. User: a reader checking an error against the figure
  a team published. Sources quote one flight's measured apogee differently: NDRT 2020's is 1,320.4 m
  as HPR Sim reads its log and 1,316.7 m in RocketPy's table. Workflow: the real-flights report
  lists, per flight, the apogee its team or its example states beside the log's as read, and the
  mean absolute error against each. Moves: `accuracy.public`, shown against a second reference.
  *Done when:* each flight has both apogees with their sources, or says none is published, and the
  report gives the error both ways.
- SOON: the collection's other predictions, sealed before anyone looks. User: a reader asking
  whether a new accuracy row was tuned. No committed report compares the collection's simulated
  times to apogee or top speeds with their logs, so each can still be fixed before its error is
  seen. The fit from the boost has read the calibration half's climbs, so burnout speed is left out. Workflow: one
  pull request merges the hash of each flight's predicted time to apogee and top speed, as
  [ADR-333][adr-333] holds the apogees', and the fixture report refuses to compare an output whose
  hash didn't merge first. Moves: the proposed `accuracy.time-to-apogee` row, whose first figure
  would then be one fixed before its error was seen. *Done when:* the hashes are merged, and a
  test shows the report refuses a flight whose predicted time to apogee wasn't merged first.

## From the 2026-11-14 planning review ([ADR-347][adr-347])

- SOON: a published apogee scored as a true height and as its altimeter read it. User: a flyer
  past Mach 1 comparing simulators on published apogees. Another simulator's write-up says 17 of
  the 23 apogees that RASAero II's example files publish were read by barometric altimeters, which
  read about 9% low on a 38 °C (100 °F) day; scored as the altimeter reads, its released model's
  mean absolute error falls from 9.9% to 8.3%, its test model's from 8.3% to 5.8%, and RASAero
  II's from 5.4% to 5.2% ([its aerodynamics page][prfl-aero], read 2026-11-14). Against a log,
  HPR Sim reads its own height as a barometric altimeter would, but a published apogee has no
  log. Workflow: where [M2.3c7][m2-3c7] scores a published apogee, the
  report gives the error both ways, with the day's temperature and its source, and says which
  reading the publisher states, or that it states none. Moves: `accuracy.supersonic`, as a range
  across the two readings. *Done when:* each published apogee has its reading's kind and its
  temperature sourced or marked unknown, the report gives both errors, and a test pins the
  conversion on a day with a hand-worked answer.
- SOON: the collection's error against the day's temperature. User: a flyer on a cold morning
  asking how far to trust the apogee. A solid motor burns slower when cold. The research note
  infers that a grain 20 K colder than its test curve costs 0.1% to 2.6% of apogee through its
  impulse ([propellant temperature][prop-temp]). One public sensitivity study, simulations of one
  public flight, gives grain temperature sampled from −45 to +60 °C (−49 to +140 °F) a
  total-order index of 0.53 for the apogee ([its page][srm], read 2026-11-14). All but one of
  the collection's 83 flights have their day's air on file. Workflow: the fixture report
  regresses each flight's signed apogee error on the air temperature at the pad, on the
  calibration half, and re-scores the held-out half with that slope. Moves:
  `bias.collection`, as a shown diagnosis. *Done when:* the report gives the slope with its 95%
  interval and the held-out half's error with and without it, as aggregates, and says so when the
  interval holds zero.

[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-328]: ../decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[astraea]: https://github.com/akhan157/astraea/blob/main/docs/accuracy.md
[recsys]: https://github.com/akhan157/RecoverySys/releases
[i1233]: https://github.com/nrdptel/fusionspace-eridanus/issues/1233
[adr-338]: ../decisions/0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md
[adr-333]: ../decisions/0333-m2-7b-new-flights-predicted-first.md
[m2-12]: ../decisions-and-roadmap.md#m2-12
[m2-9]: ../decisions-and-roadmap.md#m2-9
[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[rp1208]: https://github.com/RocketPy-Team/RocketPy/pull/1208
[prfl]: https://github.com/dustintlee-blahblah/PocketRocketFlightLab/blob/main/VALIDATION.md
[adr-347]: ../decisions/0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[prfl-aero]: https://github.com/dustintlee-blahblah/PocketRocketFlightLab/blob/main/AERODYNAMICS.md
[prop-temp]: propellant-temperature.md
[srm]: https://github.com/adamcw169/SRM-motor-to-flight
