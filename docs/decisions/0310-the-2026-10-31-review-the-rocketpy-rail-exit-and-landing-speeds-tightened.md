# ADR-310: The 2026-10-31 review: the RocketPy rail-exit and landing speeds tightened (2026-10-31)

- **Status:** accepted; tightens [M2.1][m2-1]'s gates on the RocketPy comparisons flown on the same inputs, as [ADR-289][adr-289], [ADR-294][adr-294] and [ADR-303][adr-303] did: the rail-exit speed of the nine flights from 0.2% to 0.05%, and the landing speed of those flights and the five descents from 0.2% to 0.1%. Three predicted-mode targets go from 3% to 0.1%, 0.1% and 0.5%; they stay targets ([ADR-023][adr-023]).
- **Summary:** In the RocketPy comparisons flown on the same inputs, the rail-exit speeds agree within 0.0144% and the landing speeds within 0.0301%, while RocketPy's own solver moves them by at most 3.13e-5 and 8.6e-7 of their values. The bounds go from 0.2% to 0.05% and 0.1%, 3.5 and 3.3 times the largest difference. All nine rail-exit speeds read low, the cautious side, for two causes measured here, and a test stops one reading high. These bound agreement between two codes, not accuracy against a flight.

[m2-1]: ../decisions-and-roadmap.md#m2-1
[adr-023]: 0023-predicted-mode-each-codes-own-drag-reported.md
[adr-289]: 0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[adr-294]: 0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[adr-303]: 0303-the-2026-10-28-review-the-rocketpy-peaks-tightened.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[aero]: ../physics/aero.md

**Context.** A planning review looks for a target already met with room to spare, since raising a
met bar is the cheapest way to make a check notice a regression sooner. ADR-289 set four speeds to
0.2%. In [the report][report], two of them sit far inside it:

| Metric | Rows | Reading low, high | Largest difference | Its case | Solver's own movement, largest |
|---|---:|---:|---:|---|---:|
| Rail-exit speed, same drag | 9 | 9, 0 | −0.01435% | Prometheus 2022 | 3.13e-5 of the value (NDRT 2020) |
| Landing speed, same drag and descents | 14 | 7, 7 | −0.03007% | Prometheus 2022's descent | 8.6e-7 of the value (Calisto's descent) |
| Rail-exit speed, predicted | 6 | 3, 3 | +0.0295% | Valetudo | 7.5e-5 of the value (NDRT 2020) |
| Landing speed, predicted | 6 | 3, 3 | +0.0216% | NDRT 2020 | 2.9e-9 of the value (NDRT 2020) |
| Rail-exit time, predicted | 6 | 6, 0 | −0.143% | Juno III | 5.9e-5 of the value (NDRT 2020) |

The solver's movement is each case's `solver.relative_change_from_loose`, between `rtol` 1e-6 and
1e-8. No row has a floor: each case file gives these metrics a relative bound alone.

**Which side flatters.** A rail-exit speed that reads high is the flattering side. It tells a flyer
the fins have more air over them, and the rocket turns less into the wind, than will happen. So all
nine same-drag rows reading low is the cautious direction, by at most 3.7 mm/s (0.15 in/s) on
26 m/s (85 ft/s). The one predicted row furthest out, Valetudo's, reads high by 5.9 mm/s (0.23 in/s)
on 20 m/s (66 ft/s), because HPR Sim's own drag there is about half of the example's table. A
landing speed that reads low is the flattering side, since it promises a softer landing, and the
largest difference, −0.03007%, is on that side: 2.2 mm/s (0.087 in/s) on 7.3 m/s (24 ft/s). Those
rows split evenly, 7 low and 7 high, so they show no bias, and their row-by-row cause isn't traced.

**Why nine rail-exit speeds read low.** Two terms that HPR Sim has and RocketPy's rail equation
(`udot_rail1`) leaves out account for all of it. Probes took them out in turn, in `dynamics.rs` (the
factor set to one at every angle, its mirror past 90° included), and were reverted. None is committed:

| Flight | Report | Without the drag factor | Without the mass terms too |
|---|---:|---:|---:|
| Calisto, wind | −0.00969% | −0.00074% | +0.00001% |
| NDRT 2020, wind | −0.00934% | −0.00054% | −0.00004% |
| Prometheus 2022, wind | −0.01435% | −0.00039% | −0.00012% |
| Juno III, wind | −0.00450% | −0.00202% | 0.00000% |
| Bella Lui, wind | −0.01286% | −0.00014% | −0.00001% |
| Valetudo, calm | −0.00164% | −0.00164% | −0.00001% |
| Juno III, calm | −0.00211% | −0.00211% | −0.00009% |
| Calisto, calm | −0.00075% | −0.00075% | 0.00000% |
| Bella Lui, calm | −0.00008% | −0.00008% | +0.00005% |

The drag factor is the one that turns $C_{D0}$ into the axial coefficient at an angle of attack
(Niskanen 2009 §3.4.7, [the aerodynamics page][aero]). In wind the air meets a slow rocket on the
rail at a steep angle, so HPR Sim's drag over most of the rail is higher (above about 40° the factor falls below one), and RocketPy's has no such factor. The
mass terms are the propellant's moving center of mass, which each flight's case file describes. With
both out, the differences are at most 1.2e-6 of the value, with both signs. That is below RocketPy's
largest solver movement on the metric, though Prometheus 2022's and Juno III's calm flight stay above
their own flight's movement. The rail-exit times, all low as well, fall to at most 9.8e-6 of the
value. So neither is an error found in HPR Sim, and this record changes no model. Whether Niskanen's factor holds at the
very start of the rail, where the angle is largest, isn't checked here. It moves the rail-exit
speed by at most 0.014%.

**Decision.**

1. The rail-exit speed of each of the nine same-drag flights is held to 0.05% relative, with no
   floor: 3.48 times the largest difference and 16 times the solver's movement.
2. The landing speed of those nine flights and the five descents is held to 0.1% relative, with no
   floor: 3.33 times the largest difference and about 1,160 times the solver's movement.
3. A same-drag rail-exit speed may not read high by more than 0.005%, RocketPy's largest solver
   movement on the metric rounded up. The two-sided bound alone would let a row swing from −0.014%
   to +0.05% unseen. Landing speeds get no such limit, since they carry no bias to hold.
4. In predicted mode, targets of 0.1% on the rail-exit and landing speeds and 0.5% on the rail-exit
   time: 3.40, 4.62 and 3.50 times the largest differences. They stay targets: a miss doesn't fail
   the run. It does fail the test that lists the predicted misses, so it has to be written down.
   Every other relative predicted metric keeps 3%; the traces keep their absolute targets.
5. The burnout and largest speeds keep 0.2%. The largest difference is +0.056% (Juno III's burnout),
   under four times inside 0.2% and under two times inside 0.1%.
6. Test `rail_exit_and_landing_speeds_hold_adr_310s_bounds` holds each case file to these bounds,
   checks the limit in point 3, and counts the rows (9, 14 and 6, 6, 6). A file widened back, given
   a floor or marked not scored, or a new case added at the old bound, fails.

Five mutation probes failed as they should:

- Prometheus 2022's rail-exit speed set back to 0.2% fails the test, and so does a 0.5 m/s floor on
  Valetudo's descent.
- The measured rail-exit speed scaled by 0.9988 fails `cargo xtask validate --check`: nine rows fall
  outside 0.05% while still inside the old 0.2%, and the report no longer reproduces.
- The measured rail-exit speed scaled by 1.0003, inside 0.05% on every row, fails the test's limit
  on reading high.
- Valetudo's predicted rail-exit target set to 0.02%, inside its measured +0.0295%, fails
  `the_metrics_that_are_not_scored_are_these_and_no_others`.

**What could go wrong.** The rail-exit bound is now about 3.5 times a term from one drag model. A
change to the factor $f(\alpha)$ that made it several times stronger at steep angles would fail
Prometheus 2022's row for a modelled reason. A predicted row can miss in the same way, since its
rail-exit speed carries each code's own drag, and neither drag is the truth. ADR-294 §4 already says
what to do then: measure the cause, write it in the case file, and set that row's bound in this test
with the reason. Taking the terms out moves every row toward zero, so a change that only removes a
difference from RocketPy won't trip the limit on reading high.

**Consequences.** The comparisons notice a regression in the rail-exit speed at a quarter of the old
size and in the landing speed at half. [`docs/VALIDATION.md`](../VALIDATION.md) lists the tighter
gates. No measured value moved; the census accepts the 41 redefined rows.
