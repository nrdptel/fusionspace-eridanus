# ADR-246: Student Launch by requirement number, and the other tools' known defects (2026-10-12)

- **Status:** accepted; tightens [M6.3][m6-3] and [M2.9][m2-9]; no release moves
- **Summary:** M6.3's NASA Student Launch preset takes its nine numeric limits from the 2027 handbook, each by requirement number, with a second calculation method for descent time and landing energy beside the simulation, and each check's error direction pinned. M2.9's cells for the other tools name the release they came from and each open defect that moves that tool's apogee by more than 0.5%, flown with the public fix too.

[m6-3]: ../decisions-and-roadmap.md#m6-3
[m2-9]: ../decisions-and-roadmap.md#m2-9
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-240]: 0240-the-scoreboard.md
[sl27]: https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf
[rp1195]: https://github.com/RocketPy-Team/RocketPy/pull/1195
[rp1186]: https://github.com/RocketPy-Team/RocketPy/issues/1186

**Context.** A planning review on 2026-10-12 read the competitions' current rules and the other
tools' trackers.

- **Student Launch.** NASA's [2027 Student Launch handbook][sl27] (issued August 2026) sets nine
  numeric limits a design must meet:
  - apogee between 4,000 and 6,000 ft above ground, with no altitude points below 3,500 or above
    6,500 ft (2.1);
  - a static margin of at least 2.0 on the pad (2.11) and thrust to weight of at least 5.0 (2.12);
  - at least 52 ft/s at rail exit (2.14), never past Mach 1 (2.20.6), and ballast at most 10% of
    the unballasted weight (2.20.7);
  - at most 75 ft-lbf of kinetic energy per independent section at landing (3.2), a recovery area
    of 2,500 ft radius from the pads (3.10), and a descent of at most 100 s from apogee to
    touchdown (3.11).

  Its design reviews ask for the predicted apogee in 5, 10, 15 and 20 mph wind, each section's
  landing energy and descent time, and "data from a different calculation method to verify that
  original results are accurate". [M6.3][m6-3] promises a "Student Launch-style" preset, cited and
  dated, but its done-when asks only that each preset be solved end to end. Nothing makes the
  preset hold the handbook's own numbers, nor pins which way each check errs. A check that reads
  the descent short or the landing energy low tells a team it passes when it may not: the
  flattering side, the worst class of error the project's rules for safety outputs name.
- **The other tools' known defects.** [M2.9][m2-9] fills the scoreboard's RocketPy cells, among
  others, from RocketPy's release ([ADR-240][adr-240]). As of 2026-10-12 that release is 1.13.0,
  and RocketPy's own tracker holds open defects that move its numbers. Two examples:
  - [PR #1195][rp1195], approved but not merged, puts the angular rates back into its
    solid-motor equations of motion, which today zero them and so drop aerodynamic damping. On its
    Calisto example the fix moves apogee from 4,729 to 4,643 m (−1.8%).
  - [Issue #1186][rp1186] reports a sign error in a center-of-mass lever arm.

  A cell read from the release alone would hold a known error larger than some of the gaps
  [ADR-209][adr-209] asks HPR Sim to close, and would not say so.

**Decision.**
1. **M6.3's done-when gains the handbook by number.** The Student Launch preset holds the nine
   limits above, each named by its requirement number, with the handbook's date, a stale-after date
   of 2027-08-31 (the next handbook is expected in August) and "not legal advice". Its report gives
   the apogee in 5, 10, 15 and 20 mph wind. For each section, it also gives the descent time and
   the landing kinetic energy two ways: by the simulation, and by the closed-form terminal-descent
   estimate. The difference between the two is printed.

   Each limit check gets a test pinning which way it errs and a mutation probe that shows the
   flattering side fails the test. The flattering side is low for descent time, landing energy,
   drift radius and peak Mach, and high for margin, thrust to weight and rail-exit speed.
2. **M2.9's done-when gains the known defects.** Each cell M2.9 fills names the tool's release it
   was read from. It also names every defect open in that tool's tracker on the reading date whose
   own report says it moves apogee by more than 0.5%, with that size. Where the fix is public and
   the tool's license allows running it, the cell's flights are flown with the fix applied too, and
   both numbers are shown. The scoreboard compares HPR Sim with the release.

**Consequences.**
- A Student Launch team gets the handbook's own checks, a second method its reviews ask for, and
  checks that err toward failing. M6.3 grows by one preset's worth of tests. It stays after Release
  0.3, so no release moves.
- M6.3 is user-facing but not in progress, so the limit of three surfaces in progress is not
  touched.
- The handbook changes every year, so the stale-after date makes the preset fail its own
  freshness check rather than quietly apply last year's numbers.
- M2.9's RocketPy cells cost one more flight set per open defect with a public fix. RocketPy is
  MIT-licensed, so running a patched copy is allowed.
- No done-when is loosened: both milestones keep every earlier clause.
