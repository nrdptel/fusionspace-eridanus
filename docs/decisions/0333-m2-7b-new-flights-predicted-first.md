# ADR-333: M2.7b, new flights predicted first (2026-11-09)

- **Status:** accepted; carries out [M2.7b][m2-7] under [ADR-332][adr-332] §1 and §4, and with it closes M2.7.
- **Summary:** A flight the private collection's manifest gains after M2.7a is compared with its log only when the collection holds its prediction, written from the flight's plan alone, and that file's SHA-256 sits in `validation/reports/predictions.sha256` where the branch left main, so it merged in an earlier pull request; otherwise `fixture-flights` refuses it by id. A committed digest of M2.7a's 83 manifest ids tells a new flight from an old one. Shown on an invented flight; no flight is new yet, so the list is empty. The scoreboard's held-out rows read the fixture report's held-out half, HPR Sim 16.32% and OpenRocket 24.12 14.98% over 23 flights (bias +13.42% and +11.68%), shown, not held.

[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-330]: 0330-m7-3d7-the-from-boost-scoreboard-row.md
[adr-332]: 0332-m2-7-in-two-the-halves-and-the-fits-guard-then-predictions-first.md
[m2-7]: ../decisions-and-roadmap.md#m2-7

**Context.** ADR-332 §1 fixed what M2.7b must show: a new flight's prediction sits in the private
collection before its log is compared, only the file's hash is committed, and that hash reached
main in an earlier pull request. It left how to tell a new flight from an old one, what the
prediction holds, and how to read "earlier" to this record. The manifest's flight ids carry
team and rocket names, so they can't be committed, even one at a time as hashes: a guessed id
could be confirmed against them ([ADR-151][adr-151]).

**Decision.**

1. **A flight's plan.** A manifest entry now reads as a *plan*, the fields a prediction may read
   (its id, design, configuration, motor, launch time, site, rail and ERA5 file), then its log's
   fields. `cargo xtask fixture-flights predict <plan.json>` reads a file holding a plan and
   refuses one holding any other field, a logged apogee among them. It flies the plan as
   `fixture-flights` flies hpr's own row, in the day's ERA5 air with nothing deployed, through the
   same steps. It then writes `refs/hpr-sim-fixtures/predictions/<id>.json`, never over an
   older one. The file holds the plan, the SHA-256 of its design file and of its ERA5 file, the
   commit that flew it, and the apogee. The command prints the file's SHA-256 for the list.
2. **A new flight.** `validation/reports/m2.7a-flights.sha256` holds the SHA-256 of M2.7a's
   manifest ids, sorted, the encoding of the held-out seal. A new flight's entry says
   `added_after_m2_7a`. The entries that don't must be 83, and their ids must hash to that digest,
   or the run stops. So a new flight can't pass as an old one, and an old one marked new would need
   a prediction made before its log was compared, which it can't have had.
3. **Earlier, read from git.** `validation/reports/predictions.sha256` holds only hashes, one a
   line. The check reads it at `git merge-base HEAD origin/main`, where the branch left main, not
   in the working tree. Pull requests here are squash-merged, so, as [ADR-326][adr-326] found for
   the seal, a hash is there only if it merged in a pull request of its own before the branch
   began. A flight marked new is refused, by id and on this machine only, when its prediction is
   missing, when the hash isn't on that list, or when the prediction is of another plan, design
   file or ERA5 file than the flight compared. A test runs the list's reading in a scratch
   repository: a hash added on the branch, or merged on main after the branch began, isn't on it.
4. **Shown on an invented flight.** No flight has joined the manifest since M2.7a. The tests use an
   invented flight's plan and prediction for each refusal and for the flight compared. A
   probe on the real collection added an invented flight, a copy of a compared flight under a new
   id, and was refused by name three times, then removed: with no prediction, with its
   prediction's hash only in the working tree's list, and left unmarked.
5. **The report.** The fixture report counts a flight added after M2.7a among its tier-A `.ork`
   flights and gives the count on its page, which it leaves out while it is zero, so the report
   written before this record reproduces.
6. **The held-out rows** read the fixture report's held-out half: HPR Sim 16.32% (bias +13.42%) and
   OpenRocket 24.12 14.98% (+11.68%) over 23 flights. A row can be shown only. The check then
   holds none of its cells, not even a drop in their count, so held-out flights can't veto a
   change of model ([ADR-330][adr-330]); a drop in the collection's count still fails
   `accuracy.collection`. The accuracy axis no longer says held-out flights are predicted before
   their logs are compared: these 23 were compared before the halves were sealed.

**What doesn't change.** M2.7's *done when* is met whole by M2.7a and this record, no clause
narrowed. The held-out seal, the calibration half's guard and the held rows of ADR-332 stand. A
prediction's apogee is recorded, not reported: #1376 asks for its error beside the compared one
once a new flight is compared. M2.3d commits its public predictions whole ([ADR-328][adr-328]).

**Consequences.** Adding a flight to the collection now takes two pull requests. The first commits
the prediction's hash, the prediction itself going into the collection. The second, begun after
the first merged, compares the flight. A model change between the two moves the compared apogee
but not the prediction, which keeps the commit that flew it.
