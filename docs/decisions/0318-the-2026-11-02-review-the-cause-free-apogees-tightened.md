# ADR-318: The 2026-11-02 review: the cause-free apogees tightened (2026-11-02)

- **Status:** accepted; tightens [M2.1][m2-1]'s gate on the apogee of the six RocketPy comparisons flown on the same inputs with no written model difference, as [ADR-289][adr-289] did, from 0.5% to 0.35%, and adds a one-sided stop: none of the nine same-drag apogees may read low by more than 0.001%.
- **Summary:** In the RocketPy comparisons flown on the same inputs, the six apogees with no written model difference agree within 0.1123%, while RocketPy's own solver moves an apogee by at most 6.35e-6 of its value. The bound goes from 0.5% to 0.35%, 3.1 times the largest difference. All nine same-drag apogees read high, the cautious side for a waiver, and a test now stops one reading low. These bound agreement between two codes, not accuracy against a flight.

[m2-1]: ../decisions-and-roadmap.md#m2-1
[adr-289]: 0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[adr-294]: 0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[adr-303]: 0303-the-2026-10-28-review-the-rocketpy-peaks-tightened.md
[adr-310]: 0310-the-2026-10-31-review-the-rocketpy-rail-exit-and-landing-speeds-tightened.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/flight/rocketpy-whole-flight.json

**Context.** A planning review looks for a target already met with room to spare, since raising a
met bar is the cheapest way to make a check notice a regression sooner. ADR-289 held the apogee of
six flights to 0.5%. In [the report][report] they sit well inside it, and every same-drag apogee
reads high:

| Flight | Apogee, HPR Sim less RocketPy | Bound before | RocketPy's solver movement |
|---|---:|---:|---:|
| Valetudo, calm | +0.1123% | 0.5% | 4.17e-6 of the value |
| Juno III, calm | +0.0850% | 0.5% | 1.78e-7 |
| NDRT 2020, wind | +0.0640% | 0.5% | 2.44e-6 |
| Calisto, wind | +0.0523% | 0.5% | 4.09e-6 |
| Calisto, calm | +0.0450% | 0.5% | 6.35e-6 |
| Bella Lui, calm | +0.0383% | 0.5% | 1.75e-6 |
| Prometheus 2022, wind | +1.2084% | 3% (a written cause) | 1.78e-6 |
| Juno III, wind | +0.6409% | 3% (a written cause) | 6.40e-7 |
| Bella Lui, wind | +0.3416% | 3% (a written cause) | 2.07e-6 |

The solver's movement is each case's `solver.relative_change_from_loose` in
[the RocketPy fixture][fixture], between `rtol` 1e-6 and 1e-8. No row has a floor: each case file
gives the apogee a relative bound alone. The three flights in wind with a written cause keep 3% for
the model differences their case files measure (the rail release and body lift; Juno III's
thin-plate fins too).

**Which side flatters.** An apogee that reads low is the flattering side for a waiver: it tells a
flyer the rocket stays under a ceiling it may pass ([ADR-144][adr-144] puts "apogee too low for a
waiver" in the worst class). All nine read high, from +0.0383% to +1.2084%, so the comparison errs
toward the cautious side today. On the cause-free six the offset is at most 0.11%, 0.87 m (2.9 ft) on
Valetudo's 778 m (2,552 ft). Its cause isn't traced here. Of the two terms that put the rail-exit
speeds low ([ADR-310][adr-310]), the drag factor acts in wind, but the propellant's moving mass acts
on the rail in calm air too, so this record rules neither out. It changes no model.

**Decision.**

1. The apogee of each of the six flights with no written model difference (Calisto in wind and
   calm, Valetudo, NDRT 2020, Juno III and Bella Lui calm) is held to 0.35% relative, with no floor:
   3.12 times the largest difference (+0.1123%, Valetudo) and about 550 times RocketPy's largest
   solver movement on the apogee in these flights.
2. None of the nine same-drag apogees may read low by more than 0.001%, RocketPy's largest solver
   movement on the metric (0.000635%) rounded up. The two-sided bound alone would let a row swing
   from +0.11% to −0.35% unseen.
3. The three flights with a written cause keep 3%. Their largest difference is +1.21%, 2.48 times
   inside (rounded down, as [ADR-294][adr-294] rounds headroom), under the 3 times this project's
   reviews have asked of a tightening ([ADR-303][adr-303]); keeping a bound tightens nothing.
4. Test `same_drag_speeds_and_cause_free_apogees_hold_adr_289s_bounds` holds each case file to these
   bounds, failing on a case file widened back, given a floor or marked not scored, or a new case
   added at the old bound; test `no_same_drag_apogee_reads_low_adr_318` runs the committed cases and
   fails on an apogee reading low past 0.001%.

**Considered and not taken.**

- **0.25%:** 2.22 times the largest difference, rounded down. ADR-294 took 2.5 times at the least
  and ADR-303 turned down margins under 3 times: at 0.25%, a rise of 0.14 points on Valetudo
  (0.25 − 0.1123) would fail a run, too little room for the codes' own drift.
- **The apogee time (1% to 0.5%) and the burnout height (0.2% to 0.1%) of the same six:** 2.09 and
  2.06 times inside after the change, rounded down, under the same 3 times.
- **The largest Mach number (1% to 0.5%):** 8 of 9 read low, the flattering side for flutter, but
  the speed-of-sound offset behind it is untraced ([ADR-289][adr-289]); trace it first.

**Consequences.**

- A change that moves a cause-free apogee up by more than about a quarter of a per cent (0.24 points
  on Valetudo, 0.35 − 0.1123), or any same-drag apogee low past the solver's noise, now fails CI on
  every platform instead of passing.
- CI reproduces the committed report to 2e-6, or 1e-7 of the value, on macOS, Linux and Windows,
  well inside the new bound; the RocketPy references are committed, so RocketPy's own run-to-run
  noise doesn't enter CI.
- What could go wrong: a later model change that is right (for example, a closer rail release) may
  move an apogee low. Then the one-sided stop asks for a new record with the measurement, as this
  one is, not a quiet widening.
