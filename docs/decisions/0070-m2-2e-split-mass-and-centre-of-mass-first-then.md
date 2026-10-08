# ADR-070: M2.2e split: mass and centre of mass first, then the corpus (2026-09-25)

- **Status:** accepted
- **Summary:** M2.2e split: mass and centre of mass first, then the corpus

**Context.** M2.2e carries M2.2's *done when*: at least 20 designs in a report with an error
spread over apogee, largest speed, stability margin, mass and centre of mass (CG); a written
hypothesis for every apogee more than 5% off; private designs only as anonymised ids. After
M2.2d2 (ADR-069) the flight report covers 21 configurations of five public designs, and holds
each flight's mass and CG beside OpenRocket's, but gives no spread of them. OpenRocket has not yet
flown the private corpus (`refs/loft-fixtures`), and hpr has flown none of it. That is more than
one pull request.

**Decision.** Split M2.2e into four increments, each with its own *done when*; the parent's is
kept unchanged and is met only when the last is.

1. **M2.2e1, mass and CG in the flight report.** The report summarises hpr's mass at launch and
   at the rod-clearance step, in per cent of OpenRocket's, and its CG at that step, hpr's less
   OpenRocket's distance from the nose in OpenRocket's calibres, over every flight not aborted. A
   CG difference in calibres moves the margin by as much the other way, so the two read together.
2. **M2.2e2, OpenRocket's flights of the corpus.** `flights.py` flies every configuration of
   `refs/loft-fixtures` OpenRocket can, into the gitignored `corpus-out/`; only counts are
   committed.
3. **M2.2e3, hpr's flights of the corpus.** A report of anonymised ids beside the public one,
   together holding at least 20 designs with the five spreads.
4. **M2.2e4, the causes.** A written hypothesis for every apogee more than 5% off, with the
   evidence of its size where there is any.

**Consequences.** M2.2e1 adds no physics: its numbers are ones M2.2d2 already recorded, and a test
pins the arithmetic. On the 21 public flights, launch mass is within 0.21% of OpenRocket's and the
CG within 0.016 calibres. On the 12 flights with no named cause, mass is within 0.025% at launch
and 0.03% at rod clearance, and on the largest miss (−4.34%) hpr is the lighter: mass does not
explain those misses. The CG gap above 0.003 calibres is all on *Dual parachute deployment*,
whose launch mass matches by an override covering its parts.
