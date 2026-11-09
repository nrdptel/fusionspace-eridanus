# ADR-330: M7.3d7, the from-boost scoreboard row (2026-11-08)

- **Status:** accepted; carries out [M7.3d7][m7-3] under [ADR-326][adr-326] §1 and [ADR-257][adr-257] §3, and with it closes M7.3d3 and M7.3d.
- **Summary:** The scoreboard gains `accuracy.from-boost`: the mean absolute error of the apogee predicted from the log cut after burnout. HPR Sim's cell reads the private collection's calibration-half flights the method can predict, 3.22% over 12, from `collection-boost.json`, and the check holds it as it holds HPR Sim's other cells. A note under the table says which flights those are and gives the forward simulation's 9.35% on them and the held-out half's numbers. OpenRocket 24.12's and RocketPy 1.13.0's cells say *can't*, each linking the page the features record cites for its log capability: OpenRocket's documentation describes importing altimeter data only as a planned feature, and RocketPy's Flight Comparator sets a log beside a simulated flight but its page describes no fit to it.

[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-240]: 0240-the-scoreboard.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-325]: 0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md
[adr-329]: 0329-m7-3d6-the-collections-errors-by-halves.md
[m7-3]: ../ROADMAP.md#phase-5-flight-data-and-forensics

**Context.** [ADR-257][adr-257] §3 asked for a new [scoreboard][adr-240] row, `accuracy.from-boost`,
with each other tool's cell read from its documentation: a number if the tool can do it, otherwise
*can't*. M7.3d's other clauses are on main: M7.3d1's prediction, which reads nothing past its cut,
the public flights' report ([ADR-325][adr-325]), the halves sealed ([ADR-326][adr-326]) and the
collection's errors by halves ([ADR-329][adr-329]). Four sets of flights could fill HPR Sim's cell:
the 2 public flights predicted, or the 12 of the calibration half, the 9 held out, or both halves
pooled. The collection's halves come with every predicted flight, or only those without a
deployment before apogee. The cell is one number, and the check holds it: the check fails when the
number prints worse ([ADR-240][adr-240]).

**Decision.**

1. **HPR Sim's cell is the calibration half's.** It reads
   `/predictions/halves/<calibration>/every/prediction_mean_absolute_percent` in
   `validation/reports/collection-boost.json`, found by the half's name, over every flight the half
   predicts. The scoreboard refuses to build when the errors count other than the half's predicted
   flights. Why this half:
   - **Not the public flights.** Two are predicted; their report says that is too few to tell, at
     95%, whether the prediction beats the forward simulation.
   - **Not the held-out half, nor both halves pooled.** The check fails a change that prints the
     cell worse, unless an accepted ADR names it. Held on held-out flights, it would let them veto
     changes to the method, a choice of model drawing on them, which [ADR-209][adr-209] §4 and
     [ADR-322][adr-322] §2 rule out.
   - **Every flight, not only those without a deployment before apogee.** The other measured
     accuracy rows count every compared flight. The report's second set stays on its page.
2. **A note says which flights.** The method predicts only a barometer's log cut before its climb
   reaches the analyzer's transonic speed, 227.7 m/s (747 ft/s), with enough of the log up to the
   cut to fit, among the flights the forward report compares and flies in the day's ERA5 air: 12 of
   the calibration half. The label says "the calibration-half flights it can predict", and a note
   under the accuracy table, its numbers read from the same report, says so. It counts the
   predicted flights cut less than the motor's burn after liftoff (0) and those whose burn or
   liftoff isn't known (2 of 12), gives the forward simulation's mean absolute error on the same 12
   (9.35%), and the held-out half's prediction and forward errors (3.56% and 16.86% on 9; held out
   from choices made after the seal, not blind), shown and not held. The accuracy section's words
   say why the row isn't comparable with the forward rows above it: the prediction reads the
   flight's own log up to its cut, and covers only the flights it can predict.
3. **The other tools can't, by their documentation.** Each cell names its reason and links the page
   that the features record (`validation/features/simulator.toml`) cites for the tool's `log`
   capability, which says whether it reads a flight log: OpenRocket no (its planned features),
   RocketPy yes (its Flight Comparator). The scoreboard refuses to build when either cell's answer
   changes, which sends the reason back to be read again; the test pins the linked pages. These
   pages, OpenRocket's 24.12 release notes and the pages below were read again on 2026-11-08:
   - **OpenRocket 24.12:** its guide lists importing and plotting altimeter data among planned
     features, and the 24.12 release notes add no log import. Its guide's advice after a first
     flight is to compare the simulation with the flight's data, adjust the drag or surface finish,
     rerun and compare until satisfied
     ([overrides](https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html)).
   - **RocketPy 1.13.0:** its Flight Comparator, added in 1.12.0 by its changelog, takes logged data,
     aligns it with a simulated flight and gives their difference (RMS and mean absolute). In the
     pages read, its documentation names no class or function that fits the rocket's drag, motor
     impulse or mass to a flight log, or predicts apogee from part of one. The only function on its
     utilities reference page that turns a measurement into a model parameter is a parachute's drag
     area from a drop test's landing speed.

   A *can't* cell is shown, not held, as the other tools' cells are. A tool that adds the method
   gets a number once a run or its documentation shows one. The legend's *can't* now covers a cell
   by the documentation it cites.

**What it shows.** The prediction's 3.22% is not comparable with the forward rows above it: it reads
the flight's own log up to the cut, which the forward simulation doesn't, and covers only the
flights it can predict. On the same 12 flights the forward simulation's mean absolute error is
9.35%, with the paired bootstrap interval of the difference on the collection's page. Part of that
difference may come from the wind: the forward simulation flies ERA5's wind profile, the prediction
a wind constant with height that the fit may move, held toward ERA5's climb-averaged wind by a
stated spread (±3.88 m/s on each component). On the 2 public flights the fit's model with nothing
fitted, in that constant wind, errs 3.71% against the forward simulation's 5.63%; the collection's
report gives no such figure. The scoreboard's opening now counts 54 cells, 15 measured.

**What doesn't change.** M7.3d's *done when* stands whole and is met with this row, read as
follows. Its "on the same flights in the same air and wind" is [ADR-257][adr-257] §3's "the forward
simulation's on the same flights, flown in the same launch-day air and wind", which this ADR reads
of the forward simulation: it flies each flight's own launch day, ERA5's air and wind, not a
standard day. The prediction flies the same ERA5 air, but its wind is a constant with height that
the fit may move, held toward ERA5's climb-averaged wind by a stated spread (±3.88 m/s on each
component), as M7.3d2 and M7.3d6 did
([ADR-325][adr-325] §2, [ADR-329][adr-329] §1 and §4). Read as asking both to fly one wind
profile, the clause would not be met; [#1366](https://github.com/nrdptel/fusionspace-eridanus/issues/1366)
sizes what that wind alone moves on the collection. The halves, the seal, the reports and their
checks are as ADR-326 and ADR-329 left them.

**Consequences.** M7.3d3 and M7.3d close. M7.3 stays open on M7.3c (the Bayesian option) and
M7.3e (the post-flight comparison command). A change that makes the printed calibration-half error
higher, or counts fewer flights, now needs an accepted ADR whose `- **Scoreboard:**` line names
`accuracy.from-boost.hpr` and the release it ships in ([ADR-240][adr-240]).
