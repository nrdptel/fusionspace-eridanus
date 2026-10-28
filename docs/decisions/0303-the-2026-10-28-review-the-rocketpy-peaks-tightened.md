# ADR-303: The 2026-10-28 review: the RocketPy peaks tightened (2026-10-28)

- **Status:** accepted; tightens [M2.1][m2-1]'s gates on the RocketPy whole flights flown on the same inputs, as [ADR-289][adr-289] and [ADR-294][adr-294] did: the largest Mach number of all nine flights from 3% to 1%, and the largest acceleration under power, with the whole flight's where it is scored, from 3% to 1% in the seven flights that aren't Bella Lui's.
- **Summary:** In the RocketPy whole flights flown on the same inputs, the largest Mach numbers agree within 0.235% and the largest accelerations under power, outside Bella Lui's two flights, within 0.248%, while RocketPy's own solver moves them by at most 9.1e-6 and 2.7e-4 of their values. Each bound goes from 3% to 1%, 4.3 and 4.0 times the largest difference. Bella Lui keeps 3% on its accelerations for the cause its case file measures. A test pins every bound.

[m2-1]: ../decisions-and-roadmap.md#m2-1
[adr-289]: 0289-the-2026-10-24-review-the-rocketpy-speed-and-apogee-gates-tightened.md
[adr-294]: 0294-the-2026-10-26-review-the-rocketpy-times-burnout-heights-and-traces-tightened.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md

**Context.** A planning review looks for a target already met with room to spare, since raising a
met bar is the cheapest way to make a check notice a regression sooner. ADR-294 left the largest
acceleration, the largest Mach number, the drifts and the flight time at the milestone's 3%. In
[the report][report], on the nine same-drag flights:

| Metric | Flights | Largest difference | Its flight | Solver's own movement, largest |
|---|---:|---:|---|---:|
| Largest Mach number | 9 | −0.235% | Prometheus 2022 | 9.1e-6 of the value |
| Largest acceleration under power | 7 | +0.248% | Valetudo | 2.7e-4 of the value |
| Largest acceleration, whole flight (where scored) | 5 | +0.248% | Valetudo | 2.3e-4 of the value |

Both codes fly RocketPy's standard atmosphere, as each case file declares, and the top speeds
already agree within 0.2% (ADR-289's bound; 0.051% measured), so a Mach gap is a speed-of-sound
gap: 1% still allows about 6 K between two copies of one atmosphere. Both codes fly the same thrust
curve, masses and drag, so away from the rail's ignition spike the peak thrust over mass should
match as closely as the burnout speed does. HPR Sim reads the largest Mach number lower in 8 of the 9 flights, by up to 0.235%: a
two-sided 1% bound limits that offset rather than hiding it, and its cause is left to trace
([#1019](https://github.com/nrdptel/fusionspace-eridanus/issues/1019)).

Bella Lui's two flights differ by +1.778% (calm) and +1.783% (wind) in their accelerations: the
rail's variable-mass terms at its ignition spike, which its case file measures. They keep 3%.

**Decision.**

1. The largest Mach number of each of the nine same-drag flights is held to 1% relative, with no
   floor.
2. The largest acceleration under power of the seven flights other than Bella Lui's, and the whole
   flight's largest acceleration where it is scored (Calisto in wind and calm, Juno III in wind and
   calm, Valetudo), are held to 1% relative, with no floor. NDRT 2020's and Prometheus 2022's
   whole-flight peaks stay not scored: they are the main openings, where RocketPy has added mass
   and HPR Sim has none.
3. Test `same_drag_peaks_hold_adr_303s_bounds` holds each case file to these bounds and counts
   them (9, 7 and 5): a file widened back, given a floor or marked not scored, or a new same-drag
   case added at the old bound, fails. Two mutation probes fail as they should: Valetudo's powered
   peak set back to 3% fails the test, and the measured Mach number scaled by 0.989 fails
   `cargo xtask validate --check` (any change to the report would; the scaling also puts
   Prometheus 2022's row at about −1.3%, outside 1% and inside the old 3%).
4. Kept at 3%, with reasons. The drifts in wind: the four descents' total and east drifts are
   within 0.276% (NDRT 2020's), but NDRT 2020's north drift is +2.865%, the whole flights' scored
   landing drifts reach −1.811% (Valetudo), and NDRT 2020's descent time differs by 0.705% with no
   traced cause, which a drift in a steady wind follows. The descent time (0.705%) and mean descent
   rate (−0.700%), 2.8 times inside 2%, and the flight time (0.830%) are under three times inside
   any tighter round bound.

**What could go wrong.** On Calisto, RocketPy's two acceleration peaks are 0.95% apart (0.94% in calm air), and the
rail's variable-mass terms make HPR Sim's ignition spike its largest, 1.05% above RocketPy's at the
same moment, so the two codes' largest accelerations come from different moments that happen to
agree within 0.11%. A change to the motor curve that made the spike the larger by more than 1%
would fail the bound for a modelled reason, and a drop of a few percent in HPR Sim's later peak
would go unseen while the spike stays the largest. ADR-294 §4 already says what to
do then: measure the cause, write it in the case file, and move that row back to 3% in this test,
as Bella Lui's are.

**Consequences.** The same-drag comparison notices a regression in the largest Mach number or the
powered peak acceleration at a third of the old size, except a regression in Calisto's later peak,
as above. [`docs/VALIDATION.md`](../VALIDATION.md) lists the tighter gates. No measured value moved;
the census accepts the 21 redefined rows.
