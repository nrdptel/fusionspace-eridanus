# ADR-362: Base drag behind a boattail below Mach 0.8, held to the Hermes A-3A (2026-11-20)

- **Status:** accepted; carries out [M1.14i3][m1-14i3] under [ADR-359][adr-359] decision 3, and so meets [M1.14i][m1-14i]; changes HPR Sim's base drag below Mach 0.8 behind a boattail; revises Erebus 11's merged prediction under [ADR-328][adr-328] §1b; accepts the census's moved rows ([ADR-084][adr-084]) and two scoreboard cells ([ADR-240][adr-240]); adds evidence row G41
- **Summary:** Below Mach 0.8 HPR Sim gave the base behind a boattail the whole flat-base drag. On the Hermes A-3A's measured free flight it read high by 0.050 to 0.064 power off, at least ten times the report's stated error. The base now keeps $d_B/d_1$ of the flat-base value, the base's diameter over the boattail's fore diameter, from Stoney's rule for conical afterbodies (NASA TR R-100, 1961, p. 11). On the reference the mean absolute error falls from 0.0574 to 0.0303, and no row's error grows. On the private collection's logged flights HPR Sim's mean error goes from +7.72% to +8.01% and its mean absolute error from 12.04% to 12.09%. Those flights choose nothing here; they are reported.
- **Scoreboard:** `accuracy.collection.hpr` and `bias.collection.hpr` worse in release 0.2: the change rests on a measured reference, and on the collection's calibration half it moves the mean absolute error from 9.42% to 9.45% and the mean error from +4.22% to +4.56%.

[m1-14i]: ../decisions-and-roadmap.md#m1-14i
[m1-14i2]: ../decisions-and-roadmap.md#m1-14i2
[m1-14i3]: ../decisions-and-roadmap.md#m1-14i3
[m1-14b10]: ../decisions-and-roadmap.md#m1-14b10
[m2-15]: ../decisions-and-roadmap.md#m2-15
[adr-009]: 0009-subsonic-drag-buildup-surface-finishes-and-drag.md
[adr-084]: 0084-the-accuracy-census-the-reports-numbers-held-to.md
[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-240]: 0240-the-scoreboard.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-346]: 0346-m7-5-in-three-the-rule-before-the-fit.md
[adr-359]: 0359-drag-below-mach-0-8-next-the-drag-part-of-the-gap-to-openrocket-by-mach-band.md
[fixture]: ../../validation/fixtures/aero/base-drag-under-power.json
[tests]: ../../xtask/src/aero_base_power/tests.rs
[report]: ../../validation/reports/fixture-flights.md
[real]: ../../validation/reports/real-flights.md
[note]: ../research/subsonic-boattail-base-drag.md
[cyl]: ../research/cylinder-base-drag.md
[page]: ../accuracy.md#the-base-under-power-against-a-measured-flight
[i-73]: https://github.com/nrdptel/fusionspace-eridanus/issues/73
[i-633]: https://github.com/nrdptel/fusionspace-eridanus/issues/633
[i-1572]: https://github.com/nrdptel/fusionspace-eridanus/issues/1572

**Context.** [ADR-359][adr-359] found the drag gap to OpenRocket 24.12 largest below Mach 0.8,
and [M1.14i2][m1-14i2] split that band by term: the base drag is the largest. It also added a
measured reference of the base term, the Hermes A-3A drag model's base drag in free flight,
power on and power off (Jackson 1955, NACA RM SL55F15, Fig. 20; the faired curves' error
believed less than ±0.005, and ±0.001 more for reading the figure). The model narrows to its
4.8 in base along a long 4.5° cone from 8 in. M1.14i3's *done when* asks for the term's model to
be changed to a cited source where HPR Sim's error is past the reference's uncertainty, with
nothing fitted to the collection's flights. Power off, HPR Sim read high by 0.050 to 0.064 on
the four rows below Mach 0.8, past ±0.006 on each ([fixture][fixture]). The rule gave a
boattailed base the whole flat-base value, $0.12 + 0.13\,M^2$ on the base's area (Niskanen
2009, eq. 3.94).

**Decision.**

1. **The rule.** Below Mach 0.8 a base right behind a boattail keeps $d_B/d_1 = \sqrt{a_b}$ of
   the flat-base value, $a_b$ the base's area over the boattail's fore area. Stoney gives, from
   wind-tunnel data on conical afterbodies, the base drag as the cylinder's times $(r_b/R)^3$ on
   the largest cross-section (NASA TR R-100, 1961, printed p. 11). The flat-base rule on the
   base's area is the cylinder's times $(r_b/R)^2$ on that area, so the change is one more
   factor of $r_b/R$. Hoerner's subsonic base drag has the same cube (*Fluid-Dynamic Drag*,
   1965, p. 3-20, eq. 35), and Missile DATCOM's manual names Stoney's method for a boattail's
   base drag below Mach 1.2 (1998, Table 27). From Mach 0.8 to 1 the factor runs on a straight
   line to the supersonic one, which is unchanged. A steep boattail's separation weight (16° to
   30°) moves it back to 1 as before, so a boattail too short to stay attached drags like the
   step it tends to. The code is `hpr_aero::afterbody::boattail_base_pressure_ratio`.

2. **Why this rule and not Hoerner's whole rule.** Hoerner's eq. 35, which also lowers the base
   for a thicker boundary layer, reads nearer on the Hermes A-3A: high by 0.012 to 0.016. But on
   a flat base it gives 0.046 to 0.065 on the base's area at forebody drags of 0.4 to 0.2, where
   flat bases measured without a sting read 0.12 to 0.16 from Mach 0.6 to 0.9 and HPR Sim's
   rule gives 0.167 to 0.225 ([the note][note], §3; [cylinder base drag][cyl]): Hoerner's reads
   far below the measured flat bases, HPR Sim's above them. The reference is a boattailed base, so the change is held to
   what it measures: the relief behind a boattail. A flat base is unchanged.

3. **What the reference shows after.** Power off, high on every row by 0.025 to 0.035, 3.0 to
   5.1 times the measured; before, by 0.050 to 0.064, 5.0 to 8.4 times. The mean absolute error
   over the four rows falls from 0.0574 to 0.0303 ([fixture][fixture], `hpr_power_off` against
   `flat_base_rule_power_off`). No row's error grows. Still past the stated error: Stoney warns
   the rule doesn't allow for the negative base drags seen at subsonic speeds, and measured
   bases behind cones read lower still ([the note][note]). Under power, HPR Sim's rule as it
   flies takes the motor case off the base; this model's case is wider than its base, so none
   is left, low by 0.020 to 0.027 as before. Given the nozzle exit instead, the rule now comes
   within 0.007 of the measured, from 0.0040 low to 0.0061 high.

4. **The test and the probes.** `the_power_off_rule_reads_high_the_flown_power_on_rule_low`
   ([tests][tests]) pins that the changed term reads high on every row, past the stated error
   and the reading, between Hoerner's and the old rule, with no row's error growing, and the
   means 0.0574 and 0.0303. `the_base_term_is_the_rule_on_the_base_less_the_jet` writes the
   rule out by hand, $d_B/d_1$ included. Two mutation probes, each run in the code and reverted:
   the factor set back to 1, and squared ($a_b$ for $\sqrt{a_b}$). Each fails
   `committed_fixture_is_current` on the committed fixture, and both tests above once the
   fixture is written from the mutated code.

5. **Before and after, as M1.14i3's *done when* asks** ([report][report]; over the 68 flights
   both codes fly, the parts in percent of OpenRocket's apogee):

   | | before | after |
   |---|---:|---:|
   | the base's part, swapped in sequence: mean absolute | 0.95% | 1.20% |
   | the base's part, swapped alone: mean absolute | 0.93% | 1.17% |
   | the base's part, swapped alone only while a motor burns | 0.92% | 0.94% |
   | the part below Mach 0.8: mean, mean absolute | +0.15%, 0.87% | +0.39%, 0.94% |
   | HPR Sim against 60 logs: mean, mean absolute | +7.72%, 12.04% | +8.01%, 12.09% |
   | HPR Sim against OpenRocket: mean, mean absolute | +0.90%, 1.63% | +1.18%, 1.73% |
   | 12 public logged flights: mean, mean absolute ([report][real]) | −4.91%, 9.00% | −4.64%, 8.88% |

   The gap to OpenRocket grows. OpenRocket gives a boattailed base no relief, and ADR-359
   decision 5 keeps HPR Sim's drag from being made OpenRocket's. On the collection's logs the
   mean moves up, away from them: both codes climb above the logs on average, so less drag from
   any cause moves the mean the wrong way, and the reference, not those flights, decides here
   ([ADR-346][adr-346], decision 6). A correction fitted to the collection would be judged on
   flights predicted first. The higher apogees err on the side that matters least for safety: a
   simulated apogee too high is not the side that flatters a flight near a waiver's ceiling.

6. **The other reports it moves**, regenerated in the same pull request:
   - **Public predictions.** Erebus 11, predicted first, flies 46.0 m higher, 2,760.1 m against
     its merged 2,714.1 m: −8.05% from its log against −9.58%. It is the first entry in
     `REVISIONS`, beside the merged number, with this reason (ADR-328 §1b). On the predictions
     as first merged, the 12 flights' mean absolute error is 9.01%. Bella Lui (+0.86% to
     +1.10%) and Lince (+3.39% to +4.04%) move away from their logs; Juno III (−7.24% to
     −6.39%) and Erebus 11 move nearer.
   - **The census.** 52 rows moved, 38 for the worse, accepted with the reason in
     `validation/reports/census.json`: 41 are the whole flights of three boattailed public
     examples against RocketPy 1.13.0 (patched), each code on its own drag curve; 4 are two calm
     flights of one private design against OpenRocket 24.12; 7 are the four public flights above.
   - **The synthetic flights.** HPR Sim flies them, so they move too. The shares that make the
     log readings' 90% intervals and the flight fit's, both fitted on the calibration set, are
     fitted again, and the library holds the new ones at three figures, as `cargo xtask
     synthetic` and `cargo xtask fit` require. Test sets A and B, which the shares never see,
     still pass both commands' tests.
   - **Code against code.** Against the RASAero II curves of RocketPy's examples (ADR-009) at
     Mach 0.3, Calisto's own export comes to +0.2% from +4.4%, and two more examples fall
     outside 10%: Calisto's getting-started fins (−11.5%, from −7.3%) and Juno III (−10.3%,
     from −6.0%). These size a difference between codes, not which is right.

7. **What else below Mach 0.8 the split points at.** The base under power is unmodeled: the jet
   more than doubled the Hermes A-3A's measured base drag, where HPR Sim's rule lowers it. That
   is [#633][i-633] (evidence row G14). A long boattail's own pressure drag is zero in HPR Sim's
   subsonic rule, where Cubbage measures 0.024 to 0.051 ([#73][i-73]); behind such a boattail
   the two errors now pull opposite ways. Neither changes here: neither has a measured reference
   in the validation set yet. [M2.15][m2-15]'s coast drag from the accelerometer logs will read
   the whole coasting drag on the collection's own rockets, predicted first.

**Consequences.**

- Every attached boattail's base drag below Mach 0.8 falls, so its apogee rises. A flat base, a
  step and a separated boattail are as before. No public function's signature changes.
- Evidence row G41 ([#1572][i-1572], `needs-sample`) states that the rule rests on one missile
  model's flight and on tunnel data, not on a hobby rocket's base.
- Under [ADR-143][adr-143]'s stop rule this is progress: a measured error against an independent
  reference shrank. With M1.14i3 done, M1.14i's three increments are met, and M1.14b's open
  increments resume with [M1.14b10][m1-14b10].

**What this leaves open.**

- **One vehicle.** The reference is a missile model with a long 4.5° cone; hobby boattails are
  often shorter and steeper, with fins at the base. TR R-100 gives no range of cone angles.
- **The rule still reads high** behind this cone, by about three times the measured value.
  A larger relief would need a source that covers it, not a fit to this one curve.
- **The jet** ([#633][i-633]) and **long boattails' own drag** ([#73][i-73]), as in decision 7.
