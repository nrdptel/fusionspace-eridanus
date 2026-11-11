# Ideas: accuracy against real flights

Part of the [ideas backlog](ideas.md). Ideas whose measure is one of the
[scoreboard][score]'s rows against logged flights, kept apart from the
[flight data](ideas-flight-data.md) note, which holds the log reader's and the analyzer's. Each has
its tier, its user, its workflow, the row it moves and a *done when*.

## From the 2026-11-08 planning review ([ADR-328][adr-328])

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

[score]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md
[adr-328]: ../decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[astraea]: https://github.com/akhan157/astraea/blob/main/docs/accuracy.md
[recsys]: https://github.com/akhan157/RecoverySys/releases
[i1233]: https://github.com/nrdptel/fusionspace-eridanus/issues/1233
