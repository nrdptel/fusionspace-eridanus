# ADR-179: The envelope flags, and when an angle of attack counts (2026-10-06)

- **Status:** accepted
- **Summary:** M1.14a splits in two: a1 the four envelope flags of ADR-143 §1, a2 the warnings for ADR-163 §4's issues. The flags come from a flight's summary: three by its top Mach number against 1.1467 (the fastest public reference, pinned to the committed reports by a test), 2.5 and 3.5, and one by a new peak, the largest angle of attack more than 1 s after the rail exit and before apogee or the first deployment, counting only instants where a 15° angle would give a normal force of at least a fifth of the rocket's weight. Each edge is exclusive, and a NaN raises its flags. The library, `hpr sim` (text and JSON) and Python all report them.

**Context.** [ADR-143](0143-the-operating-envelope-and-a-stop-rule-for.md) §1 defines four flags:
beyond the validated range (faster than the fastest public whole flight compared with an
independent reference, read from the committed reports), at high angle of attack (above 15° more
than 1 s after rail clearance), outside the core band (past Mach 2.5) and beyond the envelope
(past Mach 3.5). M1.14a's *done when* adds the warnings for the nine issues on
[ADR-163](0163-the-second-pass-products-and-priorities.md) §4's list, each with its own Mach or
geometry condition: too much for one pull request, and the two halves share no code.

Taken as written, the angle-of-attack flag raises on ordinary flights. Valetudo, one of RocketPy's
examples, off an 84° rail in calm air passes 15° about 0.9 s before apogee, at 16.7 m/s through
the air, faster than its 16.2 m/s rail exit. Every flight whose path turns over does the same: near
apogee the rocket slows and the path turns faster than the weathercock can follow. There the air
pushes on the rocket too weakly to matter, whatever the angle: gravity does the turning.

**Decision.**

1. **M1.14a splits** into M1.14a1, the four flags, and M1.14a2, the issue warnings, each with its
   own *done when*. The parent keeps its own, and its place in the queue.
2. **The angle of attack counts** from more than 1 s after the rail exit (or the start of a flight
   begun in the air) until apogee or the first deployment, the span the least stability margins
   already use. Within it, an instant counts where a 15° angle would give a normal force of at
   least a fifth of the rocket's weight (`HIGH_ANGLE_MIN_FORCE_SHARE`): `q (π d²/4) |C_Nα| · 15° ≥
   0.2 m g`, with the flight margin's normal-force slope. Below that, gravity rather than the air
   turns the rocket, so an error in the aerodynamics at a high angle moves the flight little.
   Measured on the tests' RocketPy rockets at their instants past 15°, the gap is wide. Turn-overs
   near apogee off 84° and 85° rails, and wind layers met that late, give 0.7% to 5.0% of the
   weight (Valetudo off 85° the least, NDRT 2020 off 84° the most); wind layers met at speed
   (30 m/s at 300 m, 40 m/s at 2,000 m) give 62% to 141% (Valetudo at 300 m the least, Juno III at
   2,000 m the most). These come from a probe run once on 2026-10-06, not a committed tool. The
   fifth sits near the middle of that gap on a log scale. It is chosen, not
   measured, like ADR-143's 1 s; M1.14e, which measures high angles, can revise both. The largest
   counted angle is a new summary field, `max_angle_of_attack_rad`, taken at each step's start,
   middle and end: a brief excursion inside one step can be missed, which matters little for a flag
   about sustained angles.
3. **The validated range's Mach number is a constant,** `hpr_sim::envelope::VALIDATED_MACH`, now
   1.1467 (OpenRocket's own top Mach number on its *Dual parachute deployment* example). A core
   crate does no I/O, and an `include_str!` of a file outside the crate wouldn't survive
   publishing it. So a test in `hpr-validate` reads the committed public reports (OpenRocket's
   examples and probes, RocketPy's cases) and fails when the constant isn't their largest
   reference. It also fails on a committed report it doesn't know, and on a known report that
   gains a Mach field it doesn't read, so a new reference can't be missed. The real flights'
   report holds only hpr's top Mach number, not the log's; the private library's never counts
   (hard rule 4).
4. **Each edge is exclusive** (a flight exactly at Mach 2.5 or exactly 15° raises nothing), and
   each flag holds on its own: a flight past Mach 3.5 raises all three Mach flags. A NaN peak
   raises its flags, so a broken number can't hide one.
5. **Where they show:** `FlightSummary::envelope_flags()` and `hpr::Flight::envelope_flags()`,
   each flag with the peak that raised it and a sentence; `hpr sim --json`'s `flags` and a
   `warning: envelope:` line per flag in its text, with a link to the operating envelope; Python's
   `Flight.envelope_flags`, a list of dictionaries. A flag is not a verdict: every flight still
   flies, and the numbers print.

**Alternatives considered.**

- *An airspeed floor at the rail exit's.* The first version; Valetudo's slow rail exit let its
  turn-over through.
- *A floor at a tenth of the largest dynamic pressure so far.* The second; review found it blind
  for most of a fast rocket's coast, where the boost's peak is large and brief. The synthetic
  two-stage rocket climbing into a 40 m/s wind 2,000 m up flies past 15° with a normal force
  larger than its weight and raised nothing. A test now flies that case.
- *The force at the instant's own angle, not at 15°.* The same flags, but small angles would then
  never count, and the summary's largest angle would be empty on ordinary flights.
- *No floor, the window ending at burnout.* It would miss a tumble or a wind layer in the coast,
  which is when most of a high-power flight's height is gained.

**Consequences.**

- Of the public designs `hpr sim` flies offline in their default configuration, 25 `.ork` files
  (OpenRocket 24.12's examples and the repository's fixtures) and 12 validation designs, flown off
  an 85° rail in calm air and in 8 m/s of wind (`hpr sim <design> --offline --inclination 85
  --wind 0` and `--wind 8`, `--json`, on 2026-10-06), none raises the angle-of-attack flag. The
  largest angle counted is 4.9°. The synthetic two-stage rocket, Mach 1.61, raises the
  validated-range flag, and no other design raises any.
- Regenerating `openrocket-flights.json` or `latest.json` with a faster reference fails the test
  until the constant moves with it, as ADR-143 meant the edge to rise.
- The CLI's JSON schema gains `flags` and `summary.max_angle_of_attack_rad`. A summary saved
  before this has no largest angle, so it can't raise the angle flag.
