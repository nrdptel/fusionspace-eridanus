# ADR-236: Supersonic flights from published comparisons, after Release 0.3 (2026-10-11)

- **Status:** accepted; adds M2.3c7 after [M10.3, Release 0.3][m10-3], from the 2026-10-08 review's ideas
- **Summary:** no real flight FusionSpace HPR is scored against passes Mach 1, so its accuracy above Mach 1 is not measured. RASAero II's author publishes 36 flights with measured apogees, and says the designs of most ship with RASAero II. M2.3c7 flies those designs and scores them beside RASAero II's own published errors.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[m3-5]: ../decisions-and-roadmap.md#m3-5
[m10-3]: ../decisions-and-roadmap.md#m10-3
[rogers]: https://www.rasaero.com/comparisons-alt.htm
[rfdb]: https://doi.org/10.5281/zenodo.20531977

**Context.** A planning review on 2026-10-11 read the committed reports. None of the 7 public real
flights in the real-flights report passes Mach 1, and the fastest top speeds stated in the private
collection are about Mach 1. [ADR-143][adr-143]'s core band runs to Mach 2.5, so most of that band
has no real flight behind it. Users feel the same gap: a 2026 forum thread on a supersonic design
sends its asker to RASAero II for a second opinion past OpenRocket's supersonic warnings.

Two public sources list such flights:
- [RASAero II's comparison page][rogers], by its author, gives 36 flights: rocket, motor, diameter,
  how the apogee was measured, the measured apogee, and RASAero II's prediction and error. It gives
  no dimensions; it says the `.CDX1` design files of most of these rockets ship with RASAero II.
  The page states no license; measured apogees are facts, and the design files stay in the
  gitignored `refs/` with only results published, as [ADR-151][adr-151] does for private designs.
- The [Rocket Flight Database v1.2][rfdb] (CC-BY-4.0 for its tabulation) takes 25 of those 36 and
  adds 3, 28 in all, 15 of them past Mach 1 by its own predicted peak Mach. It holds no geometry,
  and it admits a flight only when one tool, a fork of OpenRocket, lands within ±10% of the
  measured apogee. That rule selects on a simulator's error, so it can't serve as the selection.

**Decision.** Add M2.3c7, queued right after Release 0.3 (M10.3). The real-flights report scores
every flight on RASAero II's comparison page that flies a COTS motor, whose design ships with
RASAero II, and that hpr flies past Mach 1 and below Mach 3.5, read by hpr's own `.CDX1` importer.
Each flight's error sits beside RASAero II's published error on the same flight. The report states
the selection: the page is the author's own showcase, and nothing says which flights it leaves out.
If fewer than 5 flights qualify, an ADR records the count and why, and the report shows the gap.

**Consequences.**
- The scoreboard gets its first measured number above Mach 1, where today the cell says "not
  measured". The selection is on the motor, the design file and hpr's own peak Mach, never on any
  tool's error.
- It builds the import half of `.CDX1` support. [M3.5][m3-5] keeps its own done-when, on the
  corpus's files, and the export.
- It adds no user-facing surface: it is a report section, so the limit of three surfaces in
  progress is not touched.
- Research-motor flights wait for 1.0 (COTS solids only before it); flights above Mach 3.5 are
  past the envelope and are counted, not scored.
- No done-when is loosened and no release moves: queued after Release 0.3, it adds nothing to
  [ADR-209][adr-209] §7's list for that release.
