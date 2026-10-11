# ADR-236: Supersonic flights from published comparisons, after Release 0.3 (2026-10-11)

- **Status:** accepted; adds M2.3c7 after [M10.3, Release 0.3][m10-3], from the 2026-10-08 review's ideas
- **Summary:** no public real flight FusionSpace HPR is scored against passes Mach 1, and the private collection's report doesn't split its errors by speed. RASAero II's author publishes 36 flights with measured apogees, and says the designs of most ship with RASAero II. M2.3c7 flies those designs, scores them by Mach band beside RASAero II's own published errors, and splits the collection's errors at Mach 1.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[m3-5]: ../decisions-and-roadmap.md#m3-5
[m10-3]: ../decisions-and-roadmap.md#m10-3
[rogers]: https://www.rasaero.com/comparisons-alt.htm
[rfdb]: https://doi.org/10.5281/zenodo.20531977
[glass]: https://www.rocketryforum.com/threads/adding-over-glassing-in-or-design-to-revise-performance-sim.199194/

**Context.** A planning review on 2026-10-11 read the committed reports. None of the 7 public real
flights in the real-flights report passes Mach 1 (hpr's largest is Mach 0.99). The private
collection has faster flights: OpenRocket 24.12 flies 19 of the 79 collection flights it was run
on past Mach 1, the fastest to Mach 1.64. But the fixture report gives one error over all its
flights, with no split by speed, and the collection's designs and logs can't be published
([ADR-151][adr-151]). So nothing a reader can reproduce measures hpr above Mach 1, and nothing at
all does above about Mach 1.6, while [ADR-143][adr-143]'s core band runs to Mach 2.5 and its
extended band to 3.5. Users feel the gap: a [2026 forum thread][glass] on a supersonic design sends
its asker to RASAero II for a second opinion past OpenRocket's supersonic warnings.

Two public sources list such flights:
- [RASAero II's comparison page][rogers], by its author, gives 36 flights: rocket, motor, diameter,
  how the apogee was measured, the measured apogee, and RASAero II's prediction and error. It gives
  no other dimensions; it says the `.CDX1` design files of most of these rockets ship with RASAero
  II. Its notes say some of its predictions are reconstructions, rerun after the flight with the
  launch angle adjusted. The page states no license. The design files stay in the gitignored
  `refs/`, with only results published, as [ADR-151][adr-151] does for private designs.
- The [Rocket Flight Database v1.2][rfdb] (CC-BY-4.0 for its tabulation) takes 25 of those 36 and
  adds 3, 28 in all, 15 of them past Mach 1 by its own predicted peak Mach. It holds no geometry,
  and it admits a flight only when one tool, a fork of OpenRocket, lands within ±10% of the
  measured apogee. That rule selects on a simulator's error, so it can't serve as the selection.

**Decision.** Add M2.3c7, queued right after Release 0.3 (M10.3). The real-flights report takes
every flight on RASAero II's comparison page that flies a COTS motor and whose design ships with
RASAero II, read by hpr's own `.CDX1` importer. Eligibility comes from the page alone, never from
any tool's output: a flight the importer can't read or hpr can't fly is counted by cause, not
dropped. Errors are split at Mach 1, 2.5 and 3.5, by the page's measured Mach where it states one
and by hpr's otherwise; flights past Mach 3.5 are outside the envelope and counted, not scored. Each
flight's error sits beside RASAero II's published error on it, its reconstructions flagged, since a
reconstruction is not a prediction. The report states the selection: the page is the author's own
showcase, and nothing says which flights it leaves out. If fewer than 5 are scored past Mach 1, an
ADR records the count and why, and the report shows the gap. The fixture report also splits the
collection's errors at Mach 1, as aggregates.

**Consequences.**
- The scoreboard gets its first reproducible number above Mach 1, where today the cell says "not
  measured", and the collection's supersonic flights get their own row.
- Where hpr's own Mach decides a flight's band, a drag error near a cut can move a flight across
  it; the report gives how many flights sit within 0.1 of each cut.
- It builds the import half of `.CDX1` support. [M3.5][m3-5] keeps its own done-when, on the
  corpus's files, and the export. `THIRD-PARTY-NOTICES.md` gets a row for the page and the database.
- It adds no user-facing surface: it is a report section, so the limit of three surfaces in
  progress is not touched.
- Research-motor flights wait for 1.0 (COTS solids only before it).
- No done-when is loosened and no release moves: queued after Release 0.3, it adds nothing to
  [ADR-209][adr-209] §7's list for that release.
