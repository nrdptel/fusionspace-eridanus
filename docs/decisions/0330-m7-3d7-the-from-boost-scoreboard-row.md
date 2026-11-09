# ADR-330: M7.3d7, the from-boost scoreboard row (2026-11-08)

- **Status:** accepted; carries out [M7.3d7][m7-3] under [ADR-326][adr-326] §1 and [ADR-257][adr-257] §3, and with it closes M7.3d3 and M7.3d.
- **Summary:** The scoreboard gains `accuracy.from-boost`: the mean absolute error of the apogee predicted from the log cut at burnout. HPR Sim's cell reads the private collection's calibration half, 3.22% over 12 flights, from `collection-boost.json`, and the check holds it as it holds HPR Sim's other cells. OpenRocket 24.12's and RocketPy 1.13.0's cells say *can't*, each linking the documentation page that shows it: OpenRocket reads no flight log, and RocketPy's Flight Comparator sets a log beside a simulated flight but fits nothing to it.

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
*can't*. M7.3d's other clauses are on main: the public flights' report ([ADR-325][adr-325]), the
halves sealed ([ADR-326][adr-326]) and the collection's errors by halves ([ADR-329][adr-329]).
Three sets of flights could fill HPR Sim's cell: the 2 public flights predicted, the 12 of the
calibration half, or the 9 held out. The cell is one number, and the check holds it.

**Decision.**

1. **HPR Sim's cell is the calibration half's.** It reads
   `/predictions/halves/<calibration>/every/prediction_mean_absolute_percent` in
   `validation/reports/collection-boost.json`, found by the half's name, over every flight the half
   predicts. The scoreboard refuses to build when the errors count other than the half's predicted
   flights. Why this half:
   - **Not the public flights.** Two are predicted; their report says that is too few to tell, at
     95%, whether the prediction beats the forward simulation.
   - **Not the held-out half, nor both halves pooled.** The check fails any change that makes the
     cell worse. Held on held-out flights, it would let them veto changes to the method, a choice
     of model drawing on them, which [ADR-209][adr-209] §4 and [ADR-322][adr-322] §2 rule out. The
     held-out half's errors stay on the collection's page, shown and not held.
   - **Every flight, not only those without a deployment before apogee.** The other accuracy rows
     count every compared flight. The report's second set stays on its page.
2. **The other tools can't, by their documentation.** Each cell names its reason and links the page,
   taken from the features record (`validation/features/simulator.toml`), which names each tool's
   release and the date its pages were read. The pages were read again for this row on 2026-11-08:
   - **OpenRocket 24.12:** its guide lists importing and plotting altimeter data among planned
     features, and the 24.12 release notes add no log import. Its guide's advice after a first
     flight is to compare by hand and adjust the drag
     ([overrides](https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html)).
   - **RocketPy 1.13.0:** its Flight Comparator, added in 1.12.0 by its changelog, takes logged data,
     aligns it with a simulated flight and gives their difference (RMS and mean absolute). Its
     documentation names no class that fits a parameter to a log or predicts from part of one.

   A *can't* cell is shown, not held, as the other tools' cells are. A tool that adds the method
   gets a number once a run or its documentation shows one.
3. **The page says what the row measures.** The accuracy section's words add one sentence: the
   simulation is fitted to the flight's own log up to just after burnout, then flown on to apogee,
   and the calibration half is the part of the collection a correction may be tuned on. The legend's
   *can't* now covers a cell that links its page.

**What it shows.** The prediction's 3.22% is not comparable with the forward rows' errors: it reads
the flight's own log up to the cut, which the forward simulation doesn't. On the same 12 flights the
forward simulation's mean absolute error is 9.35%, on the collection's page with the paired
bootstrap interval of the difference. The scoreboard's opening now counts 54 cells, 15 measured.

**What doesn't change.** M7.3d's *done when* stands whole and is met with this row. The halves, the
seal, the reports and their checks are as ADR-326 and ADR-329 left them.

**Consequences.** M7.3d3 and M7.3d close. M7.3 stays open on M7.3c (the Bayesian intervals) and
M7.3e (the post-flight comparison command). A change to the from-boost method that raises the
calibration half's error now needs an accepted ADR naming `accuracy.from-boost.hpr`.
