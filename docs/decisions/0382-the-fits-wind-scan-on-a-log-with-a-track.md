# ADR-382: The fit's wind scan on a log with a track (2026-11-26)

- **Status:** accepted; fixes [issue #1646][i1646]; amends [ADR-316][adr-316]'s search and supersedes [ADR-381][adr-381] §12's caveat
- **Summary:** On a log with a track the least-squares fit's misfit can hold a second low in the wind, far from the flight's, and a search from calm air settled there with a narrow interval. After the first pass the fit now scans the wind on a polar grid, searches again from the scan's lows, carries every answer to the second pass and keeps the lowest misfit there, of the answers that stay within the scan's 12 m/s (27 mph) reach. The caveat every fit of a track printed goes. An answer left past that reach that fits the log better is said with the fit, not dropped. The fit's report is regenerated on the new search.

[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-381]: 0381-m7-3f-the-fit-on-the-command-line-and-in-python.md
[i1646]: https://github.com/nrdptel/fusionspace-eridanus/issues/1646
[fit-page]: ../physics/flight-fit.md#a-second-answer-in-the-wind
[report]: ../../validation/reports/fit-intervals.md
[i1652]: https://github.com/nrdptel/fusionspace-eridanus/issues/1652

**Context.** A review of `hpr fit` found that on the 54 mm synthetic design's own flight in a
9 m/s wind toward the east, fitted from calm air, the fitted wind came out at 1.41 m/s toward
the west, with a 90% interval under 2 m/s wide ([issue #1646][i1646]). The design's drift at
apogee isn't monotonic in the wind: a stable rocket turns into the wind under thrust and flies
upwind, and the air then carries it back. ADR-381 shipped the gap with a caveat on every fit of a
track.

**What was measured.** Flown from two starts on that log, with the rest of the fit unchanged, the
search from calm air ends at `χ²` 82.1 (heights 7.1, track 71.3, stated ranges 3.8), and the
search from 4.5 m/s or 9 m/s ends at 15.6 (7.3, 0.1, 8.2), at 8.68 m/s. So the wrong answer is a
second low of `χ²`, not a search that stopped early. Its track misfit, 7.7 m RMS, lies under the
track's 10 m floor, which is why its residuals didn't stand out. On the 6 m/s flight the two lows
are 17.6 and 12.7 at the second pass's noise; at the first pass's, twice the floors, the wind's
stated range made the wrong low the lower one.

**Decisions.**

1. **A scan, then searches from its lows.** After the first pass's search, on a log with a track,
   the fit flies 97 constant winds with its other unknowns held where that search left them: calm
   air and every 1.5 m/s to 12 m/s in 12 directions. A grid point whose `χ²` is no higher than its
   neighbors' is a low. Lows within 1.5 m/s of the first answer's wind are passed over, and the
   search starts again from the three lowest of the rest.
2. **Compared at the last pass.** Every answer goes on to the second pass, which re-weighs the
   track by the chosen answer's residual noise, and the lowest `χ²` there stands, the first of
   equals. Comparing at the first pass picked the wrong low on the 6 m/s flight.
3. **Only inside the scan's reach.** An answer searched from a scan low counts only while its
   wind is within 12 m/s, the scan's outer ring; the search from the stated wind counts wherever
   it ends, as before. The first regeneration, without this rule, failed: on its slice two
   restarts had run out past the disk to lows the scan never saw. Set A's flight 1305, flown in
   6.5 m/s, fitted 14.0 m/s, and set B's flight 2585's noise-free fit, its truth, moved from
   6.3 m/s to 14.8 m/s. The calibration set's track shares for the wind widened with such fits, and
   set A's wind toward the north then held 1,385 of 1,497 truths, more than its band allows; a
   calibration flight's noise-free truth moved the height-alone drag share from −1.33 to −1.32.
   With the rule, the calibration set gives the wind's shares and the height-alone drag share as
   before, and the track's drag and impulse shares move by 0.01 at their upper ends (2.09 to 2.10,
   1.84 to 1.85). 12 m/s lies past the 20 mph (8.9 m/s) the NAR's high-power safety code allows,
   the fastest wind the synthetic flights draw.
4. **Not the scan's lowest point alone.** The scan holds the other unknowns where a wrong wind
   left them, so its lowest point needn't be the flight's wind: on the 9 m/s flight it was the
   wrong low again (83.5 at −1.5 m/s, against 147 at 9 m/s). Several lows are searched from.
5. **Rejected:** a finer grid or a full profile over the wind (each point a search of its own,
   tens of times the cost); widening the wind's interval where a second low lies within reach
   (the issue's third idea), which would keep a wrong answer and only hide it.
6. **The caveat goes,** from the command's text and JSON (`wind_caveat`), the facade and Python.
   The reproducing test is turned around: each of the issue's flights, fitted from calm air or
   from a wrong wind of 3 m/s, states an interval that holds its wind
   (`fit_finds_the_wind_a_track_was_flown_in_from_calm_air`).
7. **An answer left past the reach is said.** A physics review flew the 54 mm design in 12.5 m/s
   toward the east and fitted it from calm air. The search from a low of the scan ended just past
   12 m/s, at a `χ²` of about 15. Rule 3 dropped it for the calm-air answer, more than 4 m/s the
   other way at a `χ²` of about 128, and nothing in the output said so. The fit can't take such an
   answer on its `χ²` alone: rule 3's two had the lower `χ²` too, and were wrong. So the fit
   reports it (`Fit::past_scan`). The rule: a search from a scan low ends past the reach, its
   `χ²` at the last pass is lower than the standing answer's, and its wind lies more than
   1.5 m/s from that answer's. Nearer, it is the same low found twice. `hpr fit` prints it as a
   caveat with the options that fit again from it; `--json` and Python give it as
   `wind_past_scan`. It is tested both ways. On the flights in 12.5 m/s and 13 m/s the answer
   left lies within 0.5 m/s of the flight's wind
   (`fit_says_the_second_answer_it_leaves_past_the_wind_scans_reach`). On the report's slice, 2
   of the 195 fits of a track say one, set A's flight 1305 among them, and both are wrong
   (`the_slice_says_a_second_answer_twice_and_both_are_wrong`). Those two fits each gain a line
   in the slice; no other line of the report changes. **Rejected:** taking the answer left where
   its `χ²` is far lower (about 15 against 128 on the flight in 12.5 m/s; 99.7 against 128 and
   221 against 230 on the slice's two), since a cut between them would rest on three flights;
   and a wider scan, under which rule 3's lows near 14 m/s would count. **Not counted:** how
   often it is said over the whole sets ([issue #1652][i1652]).

**Held by tests, and not.** The figures under *What was measured*, and those of decisions 3 and
4, came from runs made while the scan was built. No committed test makes them again. Tests hold
the outcomes: each of the issue's flights states an interval that holds its wind, the report's
counts stand as [the report][report] gives them, and decision 7's two tests hold what the fit
says of an answer past the reach.

**Consequences.** A fit of a log with a track flies 97 more flights, and each search from a low
more. The fit's report changes on every set's track flights and is regenerated; its numbers are in
[the report][report] and [the method's page][fit-page]. The scan can still miss a low between its
points. A flight's own wind past 12 m/s is taken only from a search from a wind given near it;
where a search from the scan ended there with a better fit, the fit says so (decision 7).
