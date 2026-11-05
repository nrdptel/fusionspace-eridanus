# How far a hobby bay's static vents lag the outside pressure on boost

This note sizes a vented avionics bay's lag: how low it makes a barometer read in the climb, and which way it moves
the top speed read from the slope. It applies the makers' vent rules to 38 to 98 mm bays at sea level and 3 km and
100 to 300 m/s, against the readings' top-speed bound (#872, 0.02), building on [M1] §4 to §5 and [T1]. The flights
are invented; **Inference** marks this note's own algebra and simulation (1976 atmosphere).

**In short.** A vent low-passes the climb rate. The sign is settled; the size hangs on the discharge coefficient
and the gas's polytropic index, which no source gives for a rocket bay.
- **The law (Inference, §1).** The bay's own climb rate w_b follows the true one with a time constant
  τ = λ + C·w_b/g, with C = (V/(Cd·A·n·H))² and λ the wall's viscous lag. In a steady climb the trace reads low
  by C·w²/(2g) + λ·w. That is the port term's form, so a vent is a steady Cp of C. This confirms [M1] §4.
- **The makers' rules (Inference, §2).** C runs from 5×10⁻⁵ (Eggtimer, any bay) to 0.021 (PerfectFlite's
  formula, 38 × 150 mm, isothermal), 0.024 at 3 km. Only that bay reaches #872's 0.02, and PerfectFlite's own
  table drills it larger: C 0.0016 to 0.0039. Every rule in a bay of 54 mm or more stays at or under 0.0033: 6 to
  400 times below 0.02, and 45 to 3,000 times below the logs' +0.15.
- **The top speed reads below the true peak (Inference, §3).** A low-passed speed can't exceed its input. Against
  the same fit on the true track, the fit read 3.1 to 8.6% low in the 38 mm PerfectFlite-formula bay and 0.8 to
  3.6% in the 38 mm Missile Works bay (n = 1); at most 0.6% low in 54 mm; within 0.05% in 75 and 98 mm. A
  quasi-steady port error at Cp +0.07 reads high instead: +8 to +16%.
- **The bound's port term can't carry it (Inference, §4).** Lag flattens the rise the term multiplies. Makers' vents
  stayed inside (worst 76%, on room meant for the day's air); bays at C 0.07 to 0.15 read 9 to 23% low, outside on
  12 of 16, and on 6 of 8 at C = 0.15 even with a 0.15 coefficient. Too low a top is the flattering side.
- **The climb trace reads low (Inference, §5).** The RMS from liftoff to apogee is 6.9 to 58 m in the 38 mm
  PerfectFlite-formula bay, under 6.4 m in 54 mm, under 0.25 m in 98 mm, at sea level; apogee drops at most 15 cm.

## 1. The lag law (Inference, from [G1], [H1], [D1])

- **What the reports give.** Gracey writes the steady lag as Δp = λ·dp/dt, with λ = 128·μ·L·C/(π·d⁴·p) for
  laminar tubing of length L and bore d into volume C (V here) [G1] eqs. (10.2) to (10.3), pp. 165 to 166. Huston shows
  that a tube-and-capsule system, a damped oscillator in general, lags a steady rate of change as a first-order
  system with λ = R·C, critically damped or overdamped in most uses [H1] eqs. (11) to (12), p. 21. His
  capacitance is V/(γ·p) for a rigid volume, adiabatic [H1] eq. (18), p. 23. Delio's lumped model has the same
  capacitance, a Hagen-Poiseuille resistance and an inertance ρ·L/A [D1] app. B, pp. 12 to 14.
- **A bay is an orifice.** The vent is a hole through a 1 to 4 mm wall [M1] §4; Stroick sizes it with the
  orifice equation, Cd 0.62 for sharp edges [ST] Eq. 5, p. 8. For small Δp the mass flow is Cd·A·√(2·ρ·Δp).
- **Derivation.** Let h_b be the height the bay's pressure reads, w_b its rate, and Δh = h − h_b. The outflow
  that lowers the bay's pressure at w_b is ρ·V·w_b/(n·H), with H = R·T/g and n = 1 isothermal or 1.4
  adiabatic [H1] eq. (18). Setting it equal to the orifice flow at Δp = ρ·g·Δh gives Δh = C·w_b²/(2g). The
  wall's viscous drop adds λ·w_b, with λ = 128·μ·t·V/(π·d⁴·N·n·p) for N holes of bore d through a wall t.
  Kinematics gives dΔh/dt = w − w_b. Together: dw_b/dt = (w − w_b)/τ, with **τ = λ + C·w_b/g**.
- **What follows.** In a steady climb the trace reads low by **Δh = C·w²/(2g) + λ·w**. [T1] writes the port's
  steady error as D = −Cp·V²/(2g), so on a vertical climb a vent is a steady Cp of C. The equivalent first-order
  lag is Δh/w = C·w/(2g) + λ, and a small change relaxes over C·w/g + λ, which for the orifice part is 2·Δh/w,
  [M1] §4's smoothing time. **[M1] §4's derivation holds.** The τ form takes C and λ as fixed; over a 3 km climb
  C grows 15%, so the simulations integrate Δh = C·w_b²/(2g) + λ·w_b directly (−6.95% against −6.33% at 300 m/s).
- **Checks and limits.** A pressure-form model (mass balance, both drops summed) agrees to 0.1 m/s in the top and
  0.05 m in the lag (38 mm PerfectFlite bay, 300 m/s, n = 1 and 1.4, `check`). The jet runs √C·w, 43 m/s at C 0.02
  and 300 m/s, so the orifice is incompressible. Delio's inertance [D1] eq. (8), p. 5, with a 2.5 mm hole length,
  puts a resonance at 33 to 40 Hz in that bay, damping ratio 10 to 80: overdamped, first order.

## 2. The makers' rules as C and τ (Inference)

The bays are invented, inside diameter × free length, no sled: 38 × 150, 54 × 200, 75 × 250 and 98 × 300 mm
(170, 458, 1,104 and 2,263 cm³). Rules: PerfectFlite's formula, one hole d = 0.0016·D²·L in, or four of d/2,
which it calls best [PF1] p. 11; Missile Works, d = V/400 below 100 in³, 2·√(V/6,397.71) above, at least three
holes [MW1] p. 5; Eggtimer, one 1/4 in hole per 100 in³ (its guide writes "square inches") in three [E1] p. 7;
Stroick, venting a 1 m pressure step in the time the rocket climbs 1 m at its design speed [ST] Eqs. 7 to 8,
pp. 9 to 11 (0.94 m of steady lag at Mach 1 by §1). Featherweight gives no size, only that small vents in a big
bay lag [F1] p. 7. Cd is 0.62, the wall 2 mm; λ for the holes given, n = 1.

| Bay, rule (holes) | Area | Λ | 10⁴·C, n = 1 / 1.4 | 10⁴·C, 3 km | C·w²/(2g), 100 / 200 / 300 m/s | C·w/g, 300 | λ |
|---|---|---|---|---|---|---|---|
| 38, PerfectFlite (4) | 0.227 mm² | 1,211 m | 206 / 105 | 237 | 10.5 / 42.0 / 94.6 m | 0.63 s | 118 ms |
| 38, Missile Works (3) | 0.341 mm² | 804 m | 91 / 46 | 105 | 4.6 / 18.5 / 41.7 m | 0.28 s | 39 ms |
| 54, PerfectFlite (4) | 1.64 mm² | 450 m | 28 / 15 | 33 | 1.5 / 5.8 / 13.0 m | 87 ms | 6.0 ms |
| 54, Missile Works (3) | 2.47 mm² | 299 m | 12.5 / 6.4 | 14 | 0.6 / 2.6 / 5.8 m | 38 ms | 2.0 ms |
| 75, PerfectFlite (4) | 9.55 mm² | 187 m | 4.9 / 2.5 | 5.6 | 0.25 / 1.0 / 2.2 m | 15 ms | 0.4 ms |
| 75, Missile Works (3) | 14.4 mm² | 124 m | 2.2 / 1.1 | 2.5 | 0.11 / 0.44 / 0.99 m | 7 ms | 0.14 ms |
| 98, PerfectFlite (4) | 40.1 mm² | 91 m | 1.2 / 0.6 | 1.3 | 0.06 / 0.24 / 0.53 m | 4 ms | 0.05 ms |
| Any bay, Stroick at Mach 1 (3) | ∝ V | 107 m | 1.6 / 0.8 | 1.8 | 0.08 / 0.33 / 0.73 m | 5 ms | 0.05 to 0.7 ms |
| Any bay, Eggtimer (3) | ∝ V | 84 m | 0.98 / 0.50 | 1.1 | 0.05 / 0.20 / 0.45 m | 3 ms | 0.03 to 0.4 ms |

- **Two families.** Eggtimer, Stroick and Missile Works above 100 in³ (its 98 mm row is Eggtimer's) scale area
  with volume, so Λ and C are the same in any bay; PerfectFlite, and Missile Works below 100 in³, with volume
  squared, so C grows as 1/V² (`table`). At 3 km C is 15% larger, as 1/H² (H 8,434 to 7,864 m); λ 37%, as μ/p.
- **PerfectFlite's table drills small bays larger than its formula.** For a 1.6 × 6 in bay it lists 0.032 in
  (one hole) or 0.020 in (four), against the formula's 0.025 and 0.012 [PF1] p. 11. In the 38 × 150 mm bay those
  give 0.52 and 0.81 mm² and C 0.0039 and 0.0016 (n = 1), 18 and 7 m of steady lag at 300 m/s. The formula rows
  here are the worst case; from 3.0 in up the table and formula agree within 8%.
- **Against #872's 0.02 and the logs.** The 0.02 matches only the 38 mm PerfectFlite-formula bay, isothermal.
  The logs' high readings (Cp −0.02 to −0.13) can't be lag [T1] §1. Their low ones run Cp +0.005 to +0.15; for
  #872's test values, 0.07 and 0.15, a 98 mm bay needs one hole of 1.0 to 1.4 mm, a seventh to a fifth of the
  makers' 7.1 to 7.5 mm; a 54 mm bay, 0.45 to 0.65 mm, a third to a half of PerfectFlite's 1.45 mm (`invert`).
  Only an under-vented or small bay lags that much.
- **The steady lag caps the boost's,** as w_b trails w: 118 m at 300 m/s in the 38 mm PerfectFlite bay, under 94.6 + 35.

## 3. The top speed from a 0.5 s slope (Inference)

- **Why it reads low.** By §1, w_b moves toward w at every instant, so it can never pass the true peak. Its own
  peak comes when it meets the falling w after burnout. With τ held fixed, a boost at a_b and a coast slowing at
  a_c, that peak is low by τ·a_c·ln(1 + a_b/a_c): for the orifice part a share (C·a_c/g)·ln(1 + a_b/a_c).
- **Where quasi-steadiness fails.** The quasi-steady reading w·(1 − C·a/g) is 17.7% low before burnout and
  would jump to 5.0% high after it, for the 38 mm PerfectFlite bay at 300 m/s (a_b 8.57 g, a_c 2.43 g). The
  bay can't jump: Δh, and so w_b, is continuous. The formula gives a 7.56% deficit; the simulation, 6.63% at the
  orifice alone and 8.11% with the 2 mm wall. For C of 0.005 or less it matches the orifice alone to 0.05 to
  0.07 points (`analytic`). Quasi-steadiness holds only where τ is short next to the boost: 54 mm and up.
- **The simulation.** Vertical flights with constant thrust and quadratic drag, peaking at 100, 200 and 300 m/s
  with boosts of 8, 10 and 10 g and drag of 0.5, 1 and 1.5 g at the peak at sea-level density (1.43 g at the
  300 m/s burnout); four more at 3 to 4 g and 20 g. Sampled
  at 20 and 100 Hz, pads at sea level and 3 km: 16 flights, 24 vented bays each. The read follows `readings.rs`: a
  0.3 s running median, then the highest centered 0.5 s least-squares slope. The fit alone cuts the true peak by
  0.8 to 3.1%. Shares below are against that fit; first three columns at 20 Hz, sea level, the 8 and 10 g boosts:

| Bay or port | 100 m/s | 200 m/s | 300 m/s | All 16 flights |
|---|---|---|---|---|
| 38, PerfectFlite, n = 1 / 1.4 | −5.3 / −2.7% | −6.3 / −3.3% | −7.0 / −3.8% | −3.1 to −8.6% / −1.5 to −4.9% |
| 38, Missile Works, n = 1 / 1.4 | −1.5 / −0.6% | −2.2 / −0.9% | −2.7 / −1.2% | −0.8 to −3.6% / −0.3 to −1.6% |
| 54, PerfectFlite, n = 1 | −0.14% | −0.27% | −0.43% | −0.08 to −0.59% |
| 75 and 98, any rule | 0.00 to +0.01% | 0.00% | −0.01 to 0.00% | −0.04 to +0.01% |
| Quasi-steady port, Cp +0.07 | +9.0% | +12.2% | +15.5% | +8.2 to +16.0% |
| Quasi-steady port, Cp −0.07 | +30.9% | +48.5% | +51.0% | +12.8 to +99.9% |

- **Opposite signs.** A port's pressure follows the speed at once, so its slope jumps at burnout and the top sits
  on the side that reads high: after burnout for Cp > 0, before it for Cp < 0. Against an accelerometer's top, a
  baro top above it can only be the port; one below may be lag, the fit's 0.8 to 3.1% cut or a small port (Cp +0.02
  reads 0.6% under the true peak at 100 m/s). Only a low top beyond those splits [T1] §1's open case.
- **Harder boosts and 3 km add to it:** in the 38 mm PerfectFlite-formula bay (n = 1), 3.1% low at 100 m/s and
  3 g, 8.6% at 300 m/s and 20 g; at 3 km and 10 g, 7.3% (200 m/s) and 7.9 to 8.0% (300). Sample rate: under 0.2.

## 4. Against the readings' bound (Inference)

The bound was emulated from its formula, not by running the code ([C1] lines 1957 to 1962): 7.17% of the top
for the day's air and the gain, 0.02·v·a/g for the port, (a₁ + a₂)·T/4 for the fit (T the 0.5 s window) and
5 m/s for noise, with a₁ and a₂ from the fits a window either side of the top.
- **Lag flattens the term built to carry it.** In the 38 mm PerfectFlite bay at 200 m/s (100 Hz, sea level),
  the measured rise is 13.1 m/s² against 62.3 on the true track: a port term of 4.9 m/s, an error of 15.7 m/s.
- **The readings read 20 of the 32 cases in that bay** (the rest top 227.7 m/s and are withheld). All 20 fell
  inside the bound, the worst at 76%: 200 m/s from a 3 km pad, error 17.4 m/s against 23.0. The 6.9% for the
  day's air, 13 m/s there, gave the room. A day 15 K warm also reads low, about 5.6% of the top at 3 km: the
  two together, about 27.6 m/s, exceed the 23.0 m/s bound: the flattering side, a top speed too low.
- **A vent at the logs' C.** One hole in a 98 mm bay sized to C = 0.07 or 0.15 (§2), on four flights at 100
  and 200 m/s, n = 1 and 1.4 (`vent_c_test.py`): the top read 8.7 to 14.9% low at C = 0.07, 15.3 to 23.4% at
  0.15. With 0.02 the bound held 4 of the 8 cases at C = 0.07 and none at 0.15. At 0.15 it held only the 3 g
  pair, within 0.1 m/s, and fell 5 to 14 m/s short on the rest: the measured rise was 2 to 4 m/s².
- **The quasi-steady port inside the 0.02 bound.** All 32 read port cases at Cp −0.07 to +0.15 fell inside it,
  at worst 44% used, because the measured rise grows with the error. A test with D applied as a quasi-steady
  port at Cp 0.07 may pass with 0.02, so #872's mutation check may not fail as written. Vented bays of 54 mm and
  up used at most 10% of the bound, almost all of it the fit's own cut.

## 5. The climb trace and apogee (Inference)

- **Low going up, by C·w_b²/(2g) + λ·w_b.** RMS from liftoff to apogee, sea level, n = 1 / 1.4, at 100, 200 and
  300 m/s: 38 mm PerfectFlite 10.8 / 6.9, 29.9 / 18.1 and 58.3 / 34.2 m; 38 mm Missile Works 4.1 / 2.5, 11.8 /
  6.9 and 23.6 / 13.5 m; 54 mm PerfectFlite 0.9 / 0.6, 3.0 / 1.7 and 6.3 / 3.4 m; 98 mm under 0.25 m
  (`port_lag.py trace`). It peaks near burnout and decays toward apogee. A trace read low makes a simulation that
  climbs too slowly look right in M2.3c2's RMS: the flattering side.
- **Apogee.** Only λ remains near the top, delaying the peak by τ and lowering it by g·τ²/2 [M1] §4. The 38 mm
  PerfectFlite bay with four holes peaks 0.09 to 0.17 s late and 4 to 15 cm low (n = 1.4 to 1). **A correction to
  [M1]** (§5's table, suggestion 3): "under 3 cm for any maker's rule" holds for one hole; PerfectFlite's preferred
  four holes of half the bore have four times the λ.

## 6. What no source here bounds

- **Cd under cross flow: unbounded.** In Rohde's thick-plate tests, cross flow at Mach 0.13 to 0.14, Cd rises
  from under 0.2 to 0.75 to 0.85 as the jet's head grows from 1 to 40 times the cross flow's [R1] Fig. 8, p. 15
  (read by eye), with that ratio, the Mach and the wall-to-bore ratio [R1] p. 25. A climbing bay's ratio, (u/w)²,
  is C (**Inference**): 10⁻⁴ to 0.02, far below any test, on the exit side. C goes as 1/Cd²: ×0.53 at 0.85, ×2.4 at 0.4.
- **n: bounded, 1 to 1.4.** Huston assumes adiabatic and reports unpublished Langley tubing tests closer to it
  [H1] p. 24. No source measures a bay; n = 1 doubles C against n = 1.4, the side a bound should take. Leaks
  (more area) and sleds (less V) shrink the lag, by unknown amounts.

## What the project can use

- **Data.** The NACA and NASA reports are US government works; the manuals and Stroick's paper are cited only.
- **#872's port term.** Suggest a separate vent term on the low side, not a larger coefficient: a share
  ((C·w/g + λ)·a_c/w)·ln(1 + a_b/a_c) at a stated C, erring wide as C grows (40% against a simulated 20% at 0.15).
  C = 0.024 covers every maker's rule, without the temperature share: 6 to 9% of the top, 3.5 to 13% as the boost
  varies. A bay-size input would make it 1.1 to 1.3% (1.8% at 20 g) for 54 mm at 300 m/s, against a simulated 0.6%.
- **#872's test.** Build the case as a lagged bay (§1's law), not a quasi-steady D: the 0.02 bound fails it on
  4 of 8 flights at C = 0.07 and all 8 at 0.15, where the quasi-steady one passes (§4), so a mutation can show.
- **The readings' known risk and M2.3c2.** Name the vent's side (it only lowers a baro top) and its size: under
  6.4 m of trace RMS at 300 m/s for 54 mm bays and up with makers' vents; in 38 mm bays 2.5 to 58 m by Missile
  Works' rule and PerfectFlite's formula (n = 1.4 to 1), under 0.3 m by Eggtimer's.

## Open questions

- Cd of a drilled hole in fiberglass or cardboard under an exit cross flow of 100 to 300 m/s, at head ratios
  of 10⁻⁴ to 10⁻²: no test found. One shop-vacuum step test on a real bay would size C and λ together.
- A log giving its bay's volume and vents, with an accelerometer, would test §3's sign rule.

## Sources

Read 2026-11-05. Printed page numbers for the reports; PDF pages for the manuals.
- [G1] W. Gracey, *Measurement of Aircraft Speed and Altitude*, NASA RP-1046 (1980), ch. X, pp. 165 to 169.
  <https://ntrs.nasa.gov/api/citations/19800015804/downloads/19800015804.pdf>
- [H1] W. B. Huston, *Accuracy of Airspeed Measurements and Flight Calibration Procedures*, NACA TN 1605 (1948).
  <https://ntrs.nasa.gov/api/citations/19930082253/downloads/19930082253.pdf>
- [D1] G. J. Delio, G. V. Schwent, R. S. Cesaro, NACA TN 1988 (1949), eq. (8) and app. B.
  <https://ntrs.nasa.gov/api/citations/19930082734/downloads/19930082734.pdf>
- [R1] J. E. Rohde, H. T. Richards, G. W. Metger, NASA TN D-5467 (1969), Fig. 8 and summary of results.
  <https://ntrs.nasa.gov/api/citations/19690028630/downloads/19690028630.pdf>
- [ST] G. Stroick, "HPR Research – Static Port Holes from Nescience to Science", Eqs. 5, 7 and 8, pp. 8 to 12.
  <https://offwegorocketry.com/userfiles/file/Documents/Static%20Port%20Holes.pdf>
- [PF1] PerfectFlite, StratoLoggerCF manual, p. 11. <http://www.perfectflite.com/Downloads/>
- [MW1] Missile Works, RRC3+ and RRC3 user manual v1.71, p. 5. <https://www.missileworks.com/downloads/>
- [E1] Eggtimer Rocketry, Quantum user's guide 1.09G, p. 7. <https://eggtimerrocketry.com/>
- [F1] Featherweight, Blue Raven user's manual (2026-05-12), p. 7. <https://www.featherweightaltimeters.com/>
- [M1] [Altimeter Mach error](altimeter-mach-error.md) §4 to §5; [T1] [transonic dip in logs](transonic-dip-in-logs.md).
- [C1] `readings.rs`, lines 1957 to 1962; [#872](https://github.com/nrdptel/fusionspace-eridanus/issues/872).
