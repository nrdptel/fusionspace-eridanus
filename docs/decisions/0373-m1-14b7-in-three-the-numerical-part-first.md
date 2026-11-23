# ADR-373: M1.14b7 in three, the numerical part first (2026-11-23)

- **Status:** accepted; splits [ADR-351][adr-351] decision 2's line for M1.14b7 into three increments, its done-when kept whole across them
- **Summary:** A validation uncertainty for each real flight, after ASME V&V 20, is three increments. M1.14b16 writes the combination (the root sum of squares of three parts, from a source that was read), pins it on a case worked by hand, and gives each public flight's comparison error beside its numerical part, from the flight flown again with the stepper's tolerances divided by 32. M1.14b17 adds the data part, the log's own apogee uncertainty. M1.14b18 adds the input part, the band, the count within it and the accuracy page.

[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[note]: ../research/validation-uncertainty-vv20.md
[report]: ../../validation/reports/real-flights.md#how-much-of-each-error-the-integrator-could-be

**Context.** ADR-351 gave M1.14b7 one done-when: each public real flight's row gives its apogee's
comparison error beside a validation uncertainty, the root sum of squares of three stated parts
(numerical, input, data), combined as ASME V&V 20 gives it in a source that was read; the report
counts the flights whose error lies within twice their validation uncertainty; a test pins the
combination on a case worked by hand; and the accuracy page explains the three parts and what an
error outside the band means. The three parts are three pieces of work of different sizes:

- **Numerical.** Each flight flown once more on finer steps. Hours.
- **Data.** Each log's apogee uncertainty, from its altimeter's published accuracy and from how
  a pressure reading becomes a height. That conversion is shared with the simulation: HPR Sim
  reads its own height through the same barometer, so part of the data's error also sits in the
  simulation's, and V&V 20 treats such shared errors otherwise than by a root sum of squares. A
  session's research and a check.
- **Input.** Each flight's spread over its stated input uncertainties (mass, motor impulse,
  wind, launch angle), each cited. Many flights per public flight, and a page.

The standard is paywalled. Its method was read from open texts, mainly Eça, Dowding and Roache
(2020), and the [research note][note] records what was read and what wasn't.

**Decision.**

1. **M1.14b16, the combination and the numerical part.**
   - **The combination:** `u_val = √(u_num² + u_input² + u_D²)`, each part a standard uncertainty,
     for independent errors (Eça, Dowding and Roache 2020, SAND2020-3788C, p. 4, eq. 10).
   - **The numerical part:** each flight is flown twice more, as the report flies it and with the
     Dormand–Prince stepper's relative and absolute tolerances divided by 32. An adaptive
     stepper's step can't be halved directly. Its step follows the tolerance to the power 1/5
     (Hairer, Nørsett and Wanner, vol. I, §II.4), so these steps are about half as long, and ADR-351's
     "halving the time step" is read that way. The refinement ratio `r` is measured, not assumed:
     the second run's accepted steps over the first's, counting the steps that start before the
     apogee, since later ones can't move it.
   - **The index is the coarser run's.** `E` takes the report's own flight, the coarser run.
     Richardson's estimate puts the finer run's error at `|ε|/(r⁴ − 1)` (the [note][note]), so the
     coarser run's is `|ε|·r⁴/(r⁴ − 1)`. Roache's factor 3 for two runs makes the index
     `3·|ε|·r⁴/(r⁴ − 1)`. Order 4 is assumed, the dense output's formal order: on a model problem
     the project measured 4.8 to 5 (`docs/physics/integration.md`). The flights' own order isn't
     measured, and a lower one would give a larger index. The index is a band of about 95%, divided by 2 for a standard
     uncertainty, the divisor NIST's summary gives for a Gaussian (NISTIR 8298, p. 17). `r⁴` is
     two products, so every platform computes the same figure.
   - **The report:** a section of the real-flights report gives, for each of the 12 public
     flights, the comparison error `E = S − D`, both runs' apogees and steps, `r` and `u_num`.
     A check holds each row: the first run is the report's own flight (the same apogee to the
     bit), `E` is its row's, and `u_num` is its two runs'.
2. **M1.14b17, the data part.** Each public flight's log apogee gets a standard uncertainty from
   its altimeter's published accuracy and its reading's conversion to a height. The share both
   sides take from the same barometer is handled as V&V 20 handles a shared error, from a source
   that was read, or measured to cancel and left out with the measurement written down.
3. **M1.14b18, the input part and the band.** Each flight's input part comes from its spread over
   its stated input uncertainties, each cited. Each row then gives `u_val` and whether `|E|`
   lies within `2·u_val`, and the report counts those that do. The accuracy page explains the
   three parts and what an error outside the band means. It says, as the paper does, that
   meeting the inequality is not the aim, since a larger uncertainty always meets it.

The three done-whens below hold ADR-351's whole between them, each clause in one place.

**Why not the alternatives.**

- *One increment.* It is more than a session. The input part alone is a dispersion of 12 flights.
- *Take the bare difference of the two runs as `u_num`.* It isn't V&V 20's: the standard goes
  through Roache's index (the [note][note], "In short").
- *Assume `r = 2`.* An adaptive stepper's steps only roughly follow its tolerance. On the 12
  flights the measured ratio runs from 1.26 to 1.99, and a ratio under 2 makes the index larger.
- *The finer run's index, `3·|ε|/(r⁴ − 1)`.* It bounds the finer run's error, but `E` is the
  coarser run's. Taken for the coarser run it would be `r⁴` times too small: 2.5 to 16 times
  on these flights, the flattering side.

**What was measured.** On the 12 public flights the second run took 1.78 to 1.99 times the
steps to apogee on 11, and 1.26 times on one (Andromeda). `u_num` runs from 0.0025 mm (Juno III)
to 5.7093 mm (Andromeda). Every flight's `|E|` is at least 2,700 times its own `u_num`. A finer
step can't account for any of these errors; this says nothing yet of the models, the inputs or
the logs.

**What this does not decide.**

- Whether two runs are in the asymptotic range. The safety factor 3 is Roache's for two runs
  without a measured order; three runs would measure one. With differences this far under the
  errors, a third run can't change a conclusion.
- Why Andromeda's steps grow less. Its climb takes 2,792 steps, where the others take 286 to
  671, so something other than the tolerance sets many of them; this wasn't looked into. Its
  difference between the runs is the largest of the 12, and its `r` of 1.26 makes its index
  larger still.
- The divisor: 1.1 to 1.15 instead of 2 makes `u_num` up to 1.82 times larger. Neither was
  checked against the standard.

**Done when.**

- **M1.14b16:** a function gives V&V 20's combination as the root sum of squares of the three
  parts, citing a source that was read, and a test pins it on a case worked by hand (1, 20 and
  10 m give √501 m); each of the 12 public flights in the real-flights report gives its
  comparison error beside its numerical part, from a second run with its tolerances divided by
  32, Roache's two-run index on the measured refinement ratio, over a stated divisor; a check
  holds each row to the report's own flight; the research note is committed.
- **M1.14b17:** each public flight's row gives its log apogee's standard uncertainty, each
  component cited; the share the simulation and the log take from the same barometer is
  combined as a cited source that was read gives it, or shown by a measurement to cancel; a test
  pins one flight's data part worked by hand; a flight whose altimeter has no published accuracy
  is listed by name, with no band.
- **M1.14b18:** each public flight's row gives its input part from its dispersion over stated
  input uncertainties, each one cited, and its validation uncertainty; the report counts the
  flights whose error lies within twice it; the accuracy page explains the three parts, what an
  error outside the band means, and that meeting the band is not the aim.

**Consequences.** The real-flights report flies 24 more flights. The
[report's new section][report] is checked with the rest of the report. The roadmap's M1.14b7
becomes three entries. Its title now reads "per real flight" for "for each real flight", to fit
the roadmap's size budget; the meaning and ADR-351's done-when are unchanged.
