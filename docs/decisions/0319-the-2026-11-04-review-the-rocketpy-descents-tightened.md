# ADR-319: The 2026-11-04 review: the RocketPy descents tightened (2026-11-04)

- **Status:** accepted; tightens [M1.7a][m1-7a]'s gate on the RocketPy descents under a parachute, held at the milestone's 3% since [ADR-303][adr-303] kept them there, to 0.3% for the descent time, the mean descent rate and the three drifts of Calisto, Prometheus 2022 and Juno III, and for Valetudo's descent time and rate. NDRT 2020's descent and Valetudo's drifts keep 3%.
- **Summary:** In the RocketPy descents flown from the same state under the same devices, the 17 numbers held here agree within 0.0857%, while RocketPy's own solver moves them by at most 3.41e-6 of their values. The bound goes from 3% to 0.3%, 3.5 times the largest difference. A drag area 1% too large now fails each of the four descents held to 0.3%; at 3% they passed. RocketPy counts the air a parachute drags along and HPR Sim doesn't: a small difference on these four, a large one on NDRT 2020, whose descent time differs by 0.705%. NDRT 2020's descent and Valetudo's drifts, made by the Earth's rotation alone, keep 3%. These bound agreement between two codes, not accuracy against a flight.

[m1-7a]: ../decisions-and-roadmap.md#m1-7a
[adr-303]: 0303-the-2026-10-28-review-the-rocketpy-peaks-tightened.md
[adr-310]: 0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-318]: 0318-the-2026-11-02-review-the-cause-free-apogees-tightened.md
[canopy]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/canopy-opening-loads.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/recovery/rocketpy-descent.json

**Context.** A planning review looks for a target already met with room to spare, since raising a
met bar is the cheapest way to make a check notice a regression sooner. Five of RocketPy's example
rockets are flown down in both codes from the same state near apogee, with the same drag areas,
triggers and wind. Their landing speed has been held to 0.1% since [ADR-310][adr-310]; every other
descent number is held to the milestone's 3%. ADR-303 kept them all there because NDRT 2020's
descent time differs by 0.705%. Behind it is one model difference that acts on every descent:
RocketPy adds the air a canopy drags along, and HPR Sim has no such term. Its size goes with that
air's mass over the rocket's: 15.9 kg for NDRT 2020's main against a 20.8 kg rocket, far less on the
others. Run without it, RocketPy agrees with HPR Sim within 0.36% on all five: NDRT 2020's descent
time differs by −0.116% and its north drift by −0.360%, and Calisto's and Prometheus 2022's largest
gaps fall from +0.08% to −0.027% ([the canopy opening loads note][canopy]). On the four descents held
here the term is small: their 17 numbers agree 35 times inside 3%. In [the report][report]:

| Descent | Descent time | Mean descent rate | Drift | East drift | North drift | Solver movement, largest |
|---|---:|---:|---:|---:|---:|---:|
| Calisto | +0.077% | −0.077% | +0.075% | +0.074% | +0.080% | 3.41e-6 |
| Prometheus 2022 | +0.083% | −0.083% | +0.083% | +0.0857% | +0.082% | 2.97e-6 |
| Juno III | −0.018% | +0.018% | −0.018% | −0.018% | −0.018% | 1.60e-6 |
| Valetudo | −0.019% | +0.019% | −0.889% | −0.889% | −1.766% | 2.11e-3 (north drift) |
| NDRT 2020 | +0.705% | −0.700% | +0.276% | +0.214% | +2.865% | 1.26e-6 |

The solver's movement is each case's `solver.relative_change_from_loose` in
[the RocketPy fixture][fixture], between `rtol` 1e-6 and 1e-8. Valetudo descends in still air, so
its drift is the Earth's rotation alone: 19 cm (7.5 in) east and 20 µm north, the last moved by
RocketPy's own solver by 0.2%. Its −0.889% gap is the added air again, which RocketPy's drag also has
to push sideways. No case has a floor.

**Which side flatters.** Predicting a slower descent than RocketPy, or a shorter drift, flatters:
the real descent is faster, or the recovery area larger, than stated. The bounds here are two-sided,
so a row reading either way fails sooner than before. Of the numbers held here, Calisto's and
Prometheus 2022's rates read slow and Juno III's drifts short, each on the flattering side by under
0.09%; Valetudo's kept drifts read 0.889% short. This record changes no model.

**Decision.**

1. The descent time, mean descent rate, drift, east drift and north drift of Calisto, Prometheus
   2022 and Juno III, and the descent time and mean descent rate of Valetudo, are held to 0.3%
   relative, with no floor: 3.5 times the largest difference (+0.0857%, Prometheus 2022's east
   drift) and 880 times RocketPy's largest solver movement on these numbers (3.41e-6, Calisto).
2. NDRT 2020's five keep 3%, for a written model difference: its descent time (+0.705%) and north
   drift (+2.865%) follow RocketPy's added mass, as [ADR-318][adr-318] keeps 3% for the flights with a
   written cause. Valetudo's three drifts keep 3%: the Earth's rotation alone, the north one about
   20 µm and moved 0.2% by RocketPy's own solver, their −0.889% gap the same added air.
3. Test `descents_hold_adr_319s_bounds` (`crates/hpr-validate/src/tests.rs`) holds each descent
   case file to these bounds, failing on a case file widened back, given a floor or marked not
   scored, or a new descent added at the old bound. The unit test
   `descent_matches_rocketpy_examples` (`crates/hpr-sim/src/recovery.rs`) holds its own rows to
   0.3% on the same four descents: the descent time, the landing speed (looser there than the case
   files' 0.1%) and the drifts, Valetudo's drift excepted. It has no mean-rate row, and it compares
   east and north drifts only past 10 m. NDRT 2020's rows, Valetudo's drift, and the trigger heights
   and descent rates under the drogue, which the case files don't carry, keep 3%.

**Measured.** A probe that makes each device's drag area 1% larger moves the four descents' times by
+0.45% to +0.58% (Juno III +0.452%, Valetudo +0.453%, Prometheus 2022 +0.574%, Calisto +0.580%) and
their drifts alike. The unit test fails on all four. At the old 3% they passed, and the old test
failed only on NDRT 2020's north drift, which sits 0.135 points inside its bound. `cargo xtask
validate` reproduces the committed report with the new bounds; the census
(`validation/reports/census.md`, the accepted accuracy rows) was regenerated with this record named.

**Considered and not taken.**

- **0.25%:** 2.9 times the largest difference, under the 3 times this project's reviews have asked
  of a tightening ([ADR-303][adr-303], [ADR-318][adr-318]).
- **NDRT 2020's descent time to 2%:** 2.8 times inside, and a model difference sets it; an added-mass
  term would close it first.
- **The trigger heights (within 0.17%, RocketPy's trigger sampling) and the descent rates under the
  drogue:** the case files don't carry them, and the trigger heights sit 1.8 times inside 0.3%.

**Consequences.**

- A change that moves one of the four descents' times, rates or drifts outside 0.3% now fails CI on
  every platform instead of passing: as little as 0.21 points on the tightest row (Prometheus 2022's
  east drift, 0.3 − 0.0857).
- CI reproduces the committed report to 2e-6 m or 1e-7 of the value, whichever is larger, on macOS,
  Linux and Windows, well inside the new bound; the RocketPy references are committed, so
  RocketPy's own run-to-run noise doesn't enter CI.
- What could go wrong: a later change that is right may move these descents past 0.3%. An
  added-mass term for the parachute, which RocketPy carries and HPR Sim doesn't, would move them
  toward RocketPy, by the note's probe; another model change might not. Then the change brings a new
  record with the measurement, as this one does, not a quiet widening.
