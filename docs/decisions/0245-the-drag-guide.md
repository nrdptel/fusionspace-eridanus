# ADR-245: The drag guide: the guide rocket's drag through the speed of sound, the tunnel's high reading marked, and the maintainer's read apart (2026-10-12)

- **Status:** accepted; carries out [ADR-195](0195-the-2026-10-07-guides-rocketry-explained.md) §6 and §7 for its drag guide, on [ADR-242](0242-the-guides-format-and-the-trust-guide.md)'s format; closes M0.7a5 and opens M0.7a6
- **Summary:** The fourth guide, *Drag through the speed of sound*, explains the drag's three parts, the rise near Mach 1, what the surface finish adds, how HPR Sim's forebody drag compares with NASA's Arcas Robin wind tunnel, and which error flatters. Its numbers come from a new report, `guide-rocket-drag.json`, which `cargo xtask guides` writes from the design and fails when stale, and from the committed tunnel comparison. The figure draws the tunnel's forebody drag against HPR Sim's with every speed where HPR Sim's reads high marked in the caution color: an apogee that reads low, tied to a new `hpr_sim` test of that direction. Because the tunnel's rocket isn't a hobby rocket, the guide also quotes the logged flights' mean apogee miss, which leans the other way. M0.7a5's *done when* held a second clause, the maintainer reading all four guides, which only he can meet; it moves, word for word, to a new increment, M0.7a6, marked blocked on him.

**Context.** M0.7a5's *done when* was "as M0.7a2; Neer has read all four guides". The first half
is the guide; the second is the parent M0.7a's "read ... by Neer", which no session can meet. The
guides' rocket tops out near Mach 0.84, so it shows the start of the drag rise, not the rise
itself; HPR Sim's drag through Mach 1 is checked against one measurement, the Arcas Robin's
forebody axial force (NASA TN D-4013 and D-4014, `drag-vs-mach.json`, ADR-028), where HPR Sim reads
high at every speed and 2 of 44 readings are within the 10% target. No test pinned which way a
drag error moves a flight: `hpr_sim::issues` says it in its module comment only.

**Decision.**

1. **A report the guide owns.** `validation/reports/guide-rocket-drag.json`: the guides' rocket's
   zero-lift drag coefficient, coasting, in the standard atmosphere's sea-level air, at thirteen
   Mach numbers from 0.1 to 2 with its friction, pressure, base and parasitic terms and the nose's
   and the fins' pressure drag apart; its peak from Mach 0.5 to 2 in hundredths; and at Mach 0.3
   the drag under the design's finish and three of Barrowman's (polished, HPR Sim's default
   aircraft paint, badly sprayed paint), given to every outside part. No flight is flown, so the
   tolerance is ADR-243's absolute 1e-9. Sea-level air, not the launch site's, keeps the report
   independent of a flight; the page says so.
2. **The figure is the measurement.** As ADR-243 asks, the figure draws the tunnel, not the
   guide rocket's model alone: the short Arcas Robin model with its fins on, each reading with its
   uncertainty, HPR Sim's forebody drag as a line, Mach 1 ruled, and each speed's gap marked in
   the caution color where HPR Sim's reads high. The guide rocket's own curve is in the prose.
3. **The flattering side is drag that reads high.** It makes the apogee, the top speed and the max
   q read low: the side that flatters an apogee ceiling and the fins, the class the project's
   guardrails count worst. `hpr_sim`'s new `a_higher_drag_reads_a_lower_apogee` flies two rockets, one past Mach 1
   and one below it, with their drag scaled by 0.9, 1 and 1.1, and pins that all three numbers
   fall as the drag rises. The guide's `the_flattering_marks_are_the_speeds_that_read_high` pins
   that the caution marks are the positive errors and that a positive error is HPR Sim's drag
   above the tunnel's.
4. **Both directions, with their evidence.** The tunnel says HPR Sim's drag reads high on a
   sounding rocket with thin, sharp fins and a boattail; the logged hobby flights' mean apogee
   miss reads high, which points the other way. The guide quotes both and states neither as a
   rule for a reader's rocket.
5. **Outward rounding for spreads.** A least error rounds down and a largest up, to a tenth of a
   percent, so a spread is never stated narrower than the comparison holds; single readings round
   to nearest.
6. **The maintainer's read is its own increment.** M0.7a5's *done when* becomes "as M0.7a2". Its
   second clause, "Neer has read all four guides", moves unchanged to M0.7a6, marked blocked, and
   the parent M0.7a stays open until it is met. Nothing is narrowed: the read is still required
   before M0.7a counts as done.

**Alternatives considered.**

- **Fly the guide rocket with its drag scaled and quote the apogee it loses.** Useful, but neither
  `hpr sim` nor the library's facade takes a drag scale, and flown numbers carry platforms' last
  bits; the direction test pins the sign instead, and the guide gives no size it can't read.
- **Draw the guide rocket's own drag curve.** Rejected for the figure: it is a model's output with
  nothing measured to set it against, and ADR-243 asks for the measurement. The curve's numbers
  are in the prose.
- **Keep the maintainer's read in M0.7a5.** Rejected: the guide would be finished and merged with
  its increment still open, and the next session would have nothing it could do on it.

**Consequences.**

- A change to `hpr_aero`'s drag, the finishes, the design or the tunnel comparison that moves a
  number fails `cargo test -p xtask` until `cargo xtask guides` rewrites the report and the guide
  follows.
- M0.7a's four guides are written; the parent waits on the maintainer's read (M0.7a6), and the
  queue moves on to M0.8a meanwhile.
