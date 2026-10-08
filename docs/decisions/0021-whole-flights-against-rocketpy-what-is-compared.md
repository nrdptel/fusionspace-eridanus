# ADR-021: Whole flights against RocketPy: what is compared, and the gaps it may declare (2026-09-18)

- **Status:** accepted; the `M ≥ 1` gap superseded by ADR-028
- **Summary:** Whole flights against RocketPy: what is compared, and the gaps it may declare

**Context.** M2.1b2 scores hpr's whole flights against the same-drag reference M2.1b1 committed,
`validation/fixtures/flight/rocketpy-whole-flight.json`. Its done-when asks for at least five cases
that pass. RocketPy's five examples in that reference include Prometheus 2022, which peaks at Mach
1.014, and hpr refuses `M ≥ 1` until M1.8 (ADR-008, ADR-011). The first run of the other four put
hpr 2.7 to 3.9% high on peak acceleration and 6 to 9% high on apogee, everywhere but NDRT 2020.
At Valetudo's peak, 0.034 s after ignition, there is no drag yet, so the difference had to be
thrust or mass. The reviews then found three definitional differences and a real one: the
landing points, recorded but not compared, were far apart in wind.

**Decision.**

- **The designs fly the thrust curve as RocketPy does.** RocketPy's
  `Motor(reference_pressure=None)` default, which every example keeps, makes its `pressure_thrust`
  zero (`motor.py:1188-1189`). `cargo xtask designs` had written hpr's 101,325 Pa stand-in
  instead, which adds `16 kPa × A_e` of thrust at a 1,400 m site; NDRT, at 206 m, was the one case
  it barely touched. `Nozzle::reference_pressure_pa` is now an `Option`, and the key is required
  (`null` for none), so a design says which it means; the transcription writes `null`. The site's
  example flights move with it: the first flight's apogee goes from 874.0 to 779.0 m.
- **A sixth rocket, not a weaker bar.** Bella Lui, on M2.1's own list of example rockets, is added
  to `flight.py` alone, with its example's site and rail and a declared wind (its example's weather
  is an ERA5 file).
- **The reference records everything the flight needs, and the harness checks hpr against it**
  (L75): the parachutes, RocketPy's `effective_1rl`, and the motor (total impulse, burn-out time,
  propellant mass, reference pressure). A design whose dry mass, reference area or motor differs
  by more than 1e-9, or a case whose drag is not the generator's declaration, is refused.
- **Metrics mean what RocketPy means by them** (L80). Speeds and accelerations are those of the
  centre of dry mass, the point RocketPy's state follows (`v_O + ω × p`,
  `a_O + ω̇ × p + ω × (ω × p)`). Heights are measured from that point's height at launch, because
  RocketPy's starts at the ground (`z_init = elevation`): the main opens and the flight lands at
  RocketPy's heights too. The rail exit is where the rocket has travelled `effective_1rl`, the
  forward button at the top; hpr's own exit is the last guide's (L26). The maxima are over both
  ends of every solver step, so a peak at an event, such as a canopy opening, is read at its
  instant. The drifts of the apogee and the landing point, and the landing speed, are added to
  `flight.py`'s metrics, as M2.1 lists them.
- **The committed report is pinned where the platforms agree.** A whole flight reproduces across
  macOS, Windows and Linux to about 1e-8 of each value, not always to the sixth decimal the report
  prints (NDRT's landing drift is 354.240893 m on macOS and 354.240895 m on Linux; one number of
  75 differed). ADR-015 asks to find out why before loosening. hpr is deterministic on each
  platform, and the likely source is the platforms' maths libraries, whose `sin`, `cos` and `exp`
  differ in their last bit between macOS, glibc and MSVC; an 84 s flight in a sheared wind carries
  that to 6e-9 of the drift. This amends ADR-015's six-decimal rule for whole flights: the test
  that holds the committed report to this run's allows a number two units of its sixth decimal
  or 1e-7 of itself, and nothing else; every word, tolerance and verdict must match. It is still
  a million times tighter than any gate.
- **A known gap is declared, checked and pinned.** A case may say `known_gap = "..."`. The harness
  accepts one kind, hpr's refusal of a real Mach number at or past 1; it checks that the reference
  reaches Mach 1 and fails the run once hpr flies the case
  ([Loft lesson L85](https://github.com/nrdptel/hpr-sim/blob/main/docs/research/loft-lessons.md)).
  The report lists gaps in a section of their own; they count neither as a pass nor as a fail, and
  a test pins the set.
- **What does not agree is reported, not scored, and not called a pass.** In wind, hpr turns into
  the wind less than RocketPy: its apogee moves 67 to 85% as far upwind (Juno III 769 m against
  1,147 m, ending 228 m from the pad against 582 m). Flown once in calm air, the two agree on the
  apogee to 0.18% and on the drifts to 1.3 to 3.7%. The drifts of the four windy cases, and
  Valetudo's still-air landing drift (−3.41%), are open misses whose cause is unknown: they are
  printed with both numbers and pinned, issue #50 tracks them, and **M2.1's landing offset is not
  met** until it closes. A reviewer argued they should count as failures; they don't, because a
  suite that fails on a known, tracked gap can't gate anything else, but the roadmap, the status
  and *Accuracy* say plainly that the metric is not met. So are Calisto's time of peak acceleration (two peaks
  0.9% apart, which hpr's rail terms reorder) and NDRT's whole-flight peak, the main opening, where
  RocketPy has added mass and hpr has none (ADR-012). Every other metric is gated at 3% with no
  floor, as ADR-015 requires.

**Alternatives.**

- Scoring four cases and calling Prometheus the fifth: it compares nothing, so it would not be a
  pass.
- Keeping the stand-in and gating at 10%: that would hide an input difference as a model
  difference.
- Leaving the drifts out of the metrics, as M2.1b1's fixture did: M2.1 names the landing offset,
  and the difference is the largest one found.
- Gating the drifts with a tolerance wide enough to pass: that is the "no single target" excuse of
  L82 with a number on it. They are printed, pinned and tied to an issue instead.
- Flying Prometheus's subsonic part only, or giving it a drag that keeps it subsonic: a different
  flight from the reference's, or a reference tuned to suit hpr.

**Consequences.**

- The heights, speeds, times and accelerations of five flights agree within 3%, the largest
  +1.783% (Bella Lui's 7 ms ignition spike on the rail, where hpr keeps variable-mass terms that
  RocketPy's `udot_rail1` leaves out: +1.2 to 1.3 m/s² with thrust and mass the same to five
  digits) and +1.710% (Juno III's apogee, in the strongest wind). The path in wind is open
  (issue #50): each code's own normal force and damping, hpr's drag growth with the angle of
  attack and the rail release are the candidates.
- No motor gets the sea-level correction by default: catalog and `.eng` motors carry no nozzle,
  and a design's nozzle must say which reference pressure it means.
- When M1.8 lifts the Mach limit, the Prometheus case fails until its gap is removed. Its
  tolerances are already argued in its case file.
