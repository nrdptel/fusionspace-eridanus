# ADR-280: M7.2 in seven: the synthetic flights first, then every remaining increment (2026-10-17)

- **Status:** accepted; narrows M7.2 ([the milestone][m7-2]) to a first increment and plans every remaining increment, M7.2b to M7.2g, each with its own *done when*; M7.2's own *done when* is unchanged
- **Summary:** M7.2 asks for readings with stated 90% intervals that pass [ADR-257][adr-257]'s test on synthetic flights, a smoother that fuses three sensors, two recordings read side by side and ghost files. None of it fits one session, and almost none exists: the readings run on PerfectFlite logs only, and nothing smooths, fuses, aligns or writes a trajectory. This record plans all of it at once, in seven increments, so that the sets every later check runs on come first. M7.2a builds those flights: two test sets of 1,000 and a tuning set of 200, on seeds fixed before any interval exists, each flight one of the 10 public RocketPy designs, its truth from HPR Sim and its log from a logger profile whose noise comes from the makers' datasheets and measured public logs.

[m7-2]: ../decisions-and-roadmap.md#m7-2
[m7-3]: ../decisions-and-roadmap.md#m7-3
[m7-4]: ../decisions-and-roadmap.md#m7-4
[adr-046]: 0046-debrief-folded-in-and-flight-log-analysis-that.md
[adr-251]: 0251-flight-log-importers-in-two-increments.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[noise]: ../research/altimeter-sensor-noise.md
[smoothing]: ../research/altimeter-smoothing-methods.md
[mach]: ../research/altimeter-mach-error.md
[gnss]: ../research/gnss-in-boost.md
[heights]: ../research/altimeter-height-formulas.md
[sigmas]: ../research/dispersion-sigmas.md
[debrief]: ../research/debrief-flight-readings.md
[hprcode]: https://www.nar.org/safety-information/high-power-rocket-safety-code/

**Context.** What M7.2 needs and what exists, from a survey of the code on 2026-10-17:

| M7.2 asks for | Exists |
|---|---|
| readings: apogee, top speed and acceleration, burnout, descent rates, flight time, with provenance | liftoff, apogee, top speed, landing and one mean descent rate, on PerfectFlite logs only ([ADR-251][adr-251] §6); no burnout, no acceleration, no "clipped" |
| a smoother fusing baro, accelerometer and GNSS | a running median; no filter of any kind |
| two recordings side by side, stages on one timeline | nothing |
| ghost files: JSON and CZML/glTF | nothing |
| 90% intervals passing ADR-257's test on 1,000 flights | no interval, no synthetic flight with noise, no coverage helper |

[ADR-251][adr-251] withholds the readings on every other logger "until M7.2 validates them". A
logger's own figures can't do that. A scratch run of the log survey with that withholding lifted
read 993 sample logs. Outside PerfectFlite's, a logger states its own apogee on 10 logs of
samples, and on 45 summaries with no samples. And 407 of the other 797 logs have no primary
track at all: a GPS track, or a pressure kept in a channel, which the readings don't yet take.
Every check of a reading therefore needs a truth, and only a simulated flight gives one.

[ADR-257][adr-257] §1 already says what those flights are: at least 1,000, each drawing its
design, motor, wind, launch angle and sensor noise, two sets, seeds fixed before any interval is
tuned. The research lane has since written the noise model ([sensor noise][noise] §6), the
smoother's design and the probe that shows white noise flatters an interval ([smoothing
methods][smoothing] §5–6), the transonic port error ([Mach error][mach] §1–2) and GNSS lag in
boost ([GNSS in boost][gnss]). So the flights can be built first, and every later increment
checked on them.

**Decision.**

1. **Seven increments, all planned here.** Each is one kind of work with its own check. M7.2a
   keeps M7.2's entry; M7.2b to M7.2g follow it in the roadmap, and M7.2's *done when* stays the
   parent's. The plan isn't narrowed again: an increment that can't be met as planned is recorded
   in an amendment to this record, with its measurement, and the gap stays visible.

   | Increment | *Done when* |
   |---|---|
   | M7.2a The synthetic flights | `hpr_forensics::synthetic` draws a tuning set of 200 and two test sets of 1,000 on seeds fixed in its code, each flight drawing its design (the 10 public RocketPy designs), motor, drag, wind, launch angle, launch-day air and logger independently from the ranges §2 states; the second test set differs from HPR Sim's model in three named ways (drag against Mach number, wind shear, thrust curve at equal impulse). Each flight gives its truth and the log its logger would write, by §3's noise terms, each cited in its doc comment and on a physics page. Tests: a flight is bit-identical when drawn again; each noise term holds its stated parameters on noise-only draws, and a mutation of each fails. A committed census, held to the digits the platforms share, lists each set by design and logger, with the ranges of apogee, top Mach number, wind and launch angle, and every draw that didn't fly, with its reason. |
   | M7.2b Readings on every logger | Every family that records an altitude, a pressure or a GNSS track takes the readings, and ADR-251's withholding is retired; on the tuning set, logged at each family's rate and step, each reading's error stays within the bound it states, or that family's reading is withheld by name. #646's two mistakes (an altitude above sea level, a millisecond clock) are noted with sourced bounds, tested both ways. Each reading names its method and provenance (measured, derived or clipped). `cargo xtask logs` reports the readings each family gives and withholds, by reason, with a corpus case for every reason (an invented sample where the corpus has none). `hpr analyze` reads every family with no design file, and Python's `FlightLog` gives the readings. |
   | M7.2c The rest of the readings | Top acceleration from the accelerometer, marked clipped at the part's stated range; burnout, measured from the accelerometer or derived from the speed's peak; the descent rate under each canopy, the main found back from landing as [Debrief][debrief] finds it; flight time. Each names its method and provenance, has a withheld reason with a corpus case, and its error on the tuning set is reported. |
   | M7.2d The smoother | A linear RTS smoother over height, speed, the accelerometer's bias and the barometer's correlated error, the accelerometer as its input ([smoothing methods][smoothing], "What this suggests" 1–2); GNSS fused at zero weight until it agrees with the barometer's speed ([GNSS in boost][gnss]); the barometer de-weighted above Mach 0.9 and after each charge, with a χ² gate per sensor and every rejected sample listed; liftoff from the accelerometer where there is one. On the test sets the smoothed height and speed hold the truth within their stated error, with coverage reported per state; a mutation that trusts GNSS from liftoff fails; each log gets a health card. |
   | M7.2e Every reading's 90% interval | Every reading not withheld states a 90% interval. On both test sets, ADR-257 §1's test passes for each reading: per tail a 99.9% Clopper–Pearson band holding 5%, the holding count 90%, counts per design reported; narrowed to 0.8, widened to 1.25 and shifted down one standard error, each fails. The apogee reading's mean error is reported with its sign (§2). The intervals reach the library, `hpr analyze` and Python. |
   | M7.2f Two recordings and staged flights | Two logs of one flight are put on one timeline by a shared liftoff and read side by side, never averaged, their spread per reading reported; on a set of two-logger flights drawn on new seeds, the alignment recovers the true clock offset within its stated error; per-stage logs of a staged synthetic flight sit on one timeline; RocketPy's two-logger flight (a TeleMetrum and a TeleMega) reads side by side from `hpr analyze`. |
   | M7.2g Ghost files | A flight's logs give a ghost file: time-synced trajectories in a frame at the pad with the pad's geodetic position, as JSON (a schema under `schema/`), CZML and glTF, each read back by an outside reader; every real flight log in RocketPy's repository with an altitude or a GNSS track gives one (a test where `refs/` holds them, an invented log in CI). |

2. **M7.2a's flights.** Every range below is stated before any interval exists; a range changed
   after a failure needs an ADR, as ADR-257 §1 asks of a seed.
   - **Sets.** A tuning set of 200, alternating the two test sets' draws, for anything fitted or
     tuned (ADR-257 §1: "any factor that scales the intervals is fitted on other seeds"); test
     set A, 1,000 flights of HPR Sim's model; test set B, 1,000 flights differing from it as below.
     Each set has its own seed; flight *k* draws from streams keyed by its index, so a set's first
     flights don't change if it grows.
   - **Design.** One of the 10 public RocketPy designs under `validation/designs/`, each equally
     likely. The others there can't fly (no motor) or are invented. Each flies with its RocketPy
     example's own parachutes, drag areas, triggers and lags as `rocket_mass.py` records them;
     Cavour's example has none, so it lands without one, the case of a failed recovery.
   - **Motor.** The design's motor with its impulse scaled by a factor in 0.94–1.06 and its burn
     time by one in 0.90–1.10: twice the 3% impulse σ and about the 10.7% burn-time σ
     [dispersion sigmas][sigmas] §2 gives.
   - **Drag.** The zero-lift drag scaled by a factor in 0.8–1.2, twice the low end of the 10–16%
     σ in [dispersion sigmas][sigmas].
   - **Wind.** A speed of 0–8.9 m/s (0–20 mph, the most the [high-power safety code][hprcode] allows) from a
     direction anywhere around the compass, the same at every height in set A.
   - **Launch angle.** 0–10° from vertical, toward any azimuth, inside the [safety code][hprcode]'s 20°.
   - **The site and its air.** An elevation of 0–1,500 m, sea-level fields to the high deserts;
     the 1976 standard atmosphere offset by −15 to +15 K. The loggers convert pressure by the
     standard atmosphere ([height formulas][heights]), so a day off standard is a real error in
     their heights, which the readings must carry.
   - **Set B's three differences.** The zero-lift drag times 1 + *a*·exp(−((*M* − 1)/0.2)²) +
     *b*·*M*, with *a* in −0.2–0.2 and *b* in −0.05–0.05, a transonic rise and a slope shaped
     differently; the wind a power law of height with exponent 0.14–0.20 and the drawn speed at
     18.3 m (NASA/TM-2008-215633 §2.2.5.2, as `PowerLawWind` cites); the thrust tilted by a factor
     1 + τ·(2*t*/*t*_b − 1), τ in −0.3–0.3, then rescaled to the same impulse.
   - **The truth** is HPR Sim's flight to the ground: the center of mass's height above the ground
     (a reading's truth taken from where it stood at liftoff),
     its vertical speed, the specific force along the rocket's axis, its position and Mach number,
     every 0.01 s, and its events. Under a canopy the rocket is taken to hang along the vertical.
3. **M7.2a's loggers.** A flight draws one of six profiles, each like a family the readers read,
   at its maker's logged rate and step; its noise parameters are drawn per flight within the
   ranges [sensor noise][noise] §6 gives.

   | Profile | Rate | Pressure sensor | Logged as | Accelerometer | GNSS |
   |---|---|---|---|---|---|
   | PerfectFlite | 20 Hz | the measured field model | altitude, whole feet | none | none |
   | RRC3 | 20 Hz | MS5607 at 2.3 Pa | pressure, 10 Pa steps | none | none |
   | Eggtimer | 20 Hz | Bosch BMP class, 0.2–6.6 Pa | altitude, whole feet | none | none |
   | AltOS | 100 Hz up, 10 Hz down | MS5607, 2.4–13 Pa | pressure, 1 Pa steps | high-g (ADXL375 class) | 1, 5 or 10 Hz |
   | 500 Hz IMU | 500 Hz | Bosch BMP class | pressure, 2 × 10⁻⁵ atm steps | low-g, clipping at ±32 g | none |
   | GNSS tracker | 1, 5 or 10 Hz | none | GNSS position and altitude | none | yes |

   - **Pressure:** the truth's pressure plus a per-flight offset, a gain on the change from the
     pad, a drift, fast and slow correlated noise, each part's white noise and the port's
     transonic error, rounded to the logged step. The fast noise is an AR(1) process, σ = 0.5 m
     at 50 Hz with φ = 0.5, on every barometer; the slow one a first-order Gauss–Markov term of
     σ 0.24–0.29 m and τ 0.65–1.26 s, measured on a BMP085 at rest ([sensor noise][noise] §3),
     which a reading averaged over a second keeps. White noise alone flatters a 90% interval
     ([smoothing methods][smoothing] §6). The pad's zero takes the offset out of the heights, so
     the error left after it is the gain and the drift: an MS5607's ±1.0 mbar after the zero,
     over the 400 mbar its datasheet states it on, is read as a gain of ±0.25% (an inference),
     PerfectFlite's stated ±0.1% of the height as a gain of 0.1%, and a Bosch-class part's
     relative accuracy as ±0.08% ([sensor noise][noise] §1–2). The port sits 1–5 calibers behind the nose's shoulder and reads Cp·q off true by the
     ogive-cylinder's measured Cp against Mach number ([Mach error][mach] §1–2). Each charge
     punches a pulse of the shape on the public Pnut log, scaled by 0–2.
   - **Acceleration:** the truth's axial specific force times 1 + *s*, plus a per-flight bias
     and white noise, clipped at the part's range and rounded to its step. The noise is the
     part's density over its bandwidth, half the rate it is sampled at while climbing, kept
     after apogee when a logger writes fewer samples.
   - **GNSS:** the truth's position through a second-order lag of natural frequency 0.25–1.0
     rad/s (about 0.5 rad/s, near the note's simulated filter's 0.47–0.69 rad/s), 3 m of noise,
     and a dropout of 0–10 s from liftoff and from burnout, the fix flagged valid throughout ([GNSS in boost][gnss] §3–4 and *What this suggests*).
   - **Clock:** the log starts 2–10 s before liftoff and ends 2–10 s after landing.
   - **Left out, said on the physics page:** the bay's vent lag (under 3 cm at apogee for any
     maker's rule, [Mach error][mach] §5), a temperature term (no source gives a bay's change in
     flight), clock drift, and the logger's own speed column, which a logger filters its own way.
4. **Where the code lives.** The flights are in `hpr-forensics`, the one crate that may join a
   log and the simulator ([ADR-046][adr-046]); it serves [M7.3][m7-3]'s fits next, whose
   synthetic flights ADR-257 §1 also asks for. A synthetic log is an `hpr_flightdata` record
   tagged with the format of the family its profile mimics, its logger named as synthetic, so
   that a reading checked on it is checked on that family's rate, step and record. The crate is
   published, and a published crate can't read files outside its own folder, so the ten design
   files are handed to it by whoever flies the flights: `cargo xtask synthetic` reads them from
   `validation/designs/`.

**The roadmap's wording.** M7.2's bullets in the roadmap give way to the increments that carry
them: the readings with their provenance (measured, derived or clipped) to M7.2b and M7.2c; the
smoother, its liftoff detection and time alignment to M7.2d; two recordings never averaged, and
per-stage logs, to M7.2f; the ghost product to M7.2g. Phase 5's opening now names
`hpr-forensics` as the synthetic flights' home beside M7.3 and M7.4, and keeps ADR-046's rule
that the readings need no design file or simulator.

**Consequences.** Every later increment is checked on flights fixed now, before anything is
tuned, so no check can be fitted to the flights it is judged on. The readings reach every logger
only once the tuning set shows they hold at that logger's rate and step. The transonic port error,
which [M7.4][m7-4]'s fault models will name, is in the noise from the start, so no reading can lean
on a barometer through Mach 1 unseen.
