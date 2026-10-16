# How far a barometric altimeter misreads through Mach 1 and through its vents

This note asks what published sources measure for the static-pressure error on a rocket's body tube near Mach 1
(its sign and size by port position), how far a vented bay lags the outside air against vent area and bay volume,
and whether the makers' vent rules agree. It turns both into an altitude error at Mach 0.9–1.2 and at apogee. It
builds on [dual-altimeter-spread](dual-altimeter-spread.md) §4 (a sealed model-rocket bay; the makers' rules for
one bay) and [apogee bias](apogee-bias.md) §3 (lag lowers a peak by g·τ²/2). Sources are NACA and NASA reports,
makers' manuals and two hobby articles. **Inference** marks this note's arithmetic (1976 atmosphere). Read 2026-10-16.

**In short.** Near Mach 1 the port's error is hundreds of meters, either sign; the vent's lag is about a meter in a
well-vented bay and can reach a hundred in a small one. Neither reaches a vented bay's apogee.
- **A port behind the nose reads high through Mach 1.** On a 3-caliber ogive-cylinder in a transonic tunnel, the
  pressure coefficient one caliber behind the shoulder is about −0.02 at Mach 0.9 and −0.08 at Mach 1.0 [N1]:
  +110 m and +480 m of false altitude at sea level, about 7% less at 3 km. A free-flight fuselage vent read
  +470 m just past Mach 1 [N2] (§1–2).
- **Behind the Mach-1 shock the sign flips.** At Mach 1.0 a shock stands 2.9 calibers behind the shoulder; a port
  behind it reads 120–180 m low, and as the shock sweeps over a port indicated altitude drops about 410 m within
  about 0.04 in Mach [N1], [N3]. **Inference:** that is the makers' "Mach dip"; PerfectFlite's placement rule,
  4–5 calibers back, puts the port there (§1–3).
- **No maker sizes the error.** Five of seven hold off or de-weight the barometer through Mach 1; none ties it to
  the vehicle; no hobby log sizes the dip (§3).
- **A vent lags in two regimes (Inference).** In fast climb the hole carries orifice flow: the bay reads low by
  u²/(2g), growing with the square of climb rate, not a first-order lag. Near apogee the flow is viscous and the
  lag is a first-order τ that grows as 1/p [N4] (§4).
- **The makers' vent rules differ by up to 14.5 times in area in a 38 mm bay, 1.3 times in a 98 mm one.** In the
  38 mm bay the volume-squared rules lag 49–135 m through Mach 1; any rule lags 0.4–1.1 m in the 98 mm bay. All
  of them move apogee by under 3 cm (§5).
- **Only a near-sealed bay moves apogee.** One model-rocket test lagged about 2.5 s and read about 30 m low [P1];
  a 2.5 s first-order lag predicts 30.6 m (**Inference**). No source measures a high-power bay (§5).

## 1. The measured transonic error on a body of revolution

**Wind tunnel, ogive-cylinder [N1].** A 4.096 in cylinder, 3-caliber ogive nose, 50.0 in long, at zero incidence
in Langley's 8-foot transonic pressure tunnel, 34 orifices, turbulent boundary layer [N1] pp. 3–5. At Mach 1.0 a
shock stands 2.9 calibers behind the 12.3 in shoulder; at 1.1 and 1.2 the nose shock reflects off the tunnel wall
onto the model, so stations behind about 30 and 48 in are artifacts [N1] p. 6. Cp by calibers behind the
shoulder, read from [N1] Fig. 5 (p. 27) by a pixel trace to about ±0.01 (**Inference**):

| Calibers aft | Mach 0.7 | 0.9 | 1.0 | 1.1 | 1.2 |
|---|---|---|---|---|---|
| 0 (shoulder) | −0.07 | −0.08 | −0.20 | −0.14 | −0.11 |
| 1 | −0.02 | −0.02 | −0.08 | −0.07 | −0.04 |
| 2 | −0.01 | −0.01 | −0.06 | −0.05 | −0.01 |
| 3 | −0.01 | −0.01 | +0.03 | −0.02 | −0.01 |
| 4 | −0.01 | −0.01 | +0.02 | tunnel | −0.02 |
| 6 | 0.00 | −0.01 | 0.00 | tunnel | −0.01 |

- **Free flight, a fuselage vent [N2].** One orifice on a 7 in rocket-propelled model's cylindrical fuselage,
  0.28 of its length behind the nose, read about −0.001 of impact pressure (total minus static) to Mach 0.99,
  then −0.060 at Mach 1.0 and −0.045 near 1.2 [N2] Fig. 38, p. 660: Cp −0.077 and −0.063 (**Inference**).
- **Free fall through Mach 1 [N3].** On a fineness-12 body the flow changes almost wholly between Mach 0.98 and
  1.02 [N3] p. 6; a shock sweeps aft over three orifices between Mach 0.997 and 1.005 [N3] p. 9. Zero-error
  stations move with Mach; a vent's error is "comparatively irregular" [N2] p. 659.
- **Ahead of a fin the error is positive below Mach 1 [N2] p. 647:** booms ahead of a tail fin peak at +0.075 to
  +0.12 of impact pressure near Mach 0.95–1.16, then one reads −0.035 to −0.055 past Mach 1 [N2] Fig. 35, p. 659,
  sizes Gracey calls open to question [N2] p. 658.

## 2. From pressure coefficient to altitude error (Inference)

The pressure error is Δp = Cp·q, with q = 0.7·p·M². Hydrostatics gives Δh ≈ −Δp/(ρg) = −0.7·M²·Cp·H for scale
height H = R·T/g. Pressure cancels: the error depends on Mach, Cp and air temperature only. At sea level (101,325 Pa,
288.15 K, H = 8,435 m), Mach 1.0, Cp −0.06 (2 calibers aft): q = 70,928 Pa, Δp = −4,256 Pa, Δh = 4,256 / 12.01 N/m³
= +354 m (+360 m inverting the standard atmosphere exactly); at 3 km (268.65 K, H = 7,864 m), +330 m.

Altitude error at sea level / 3 km, from the unrounded trace (+ reads high; ±0.01 in Cp is ±50 m at Mach 0.9 to
±85 m at 1.2, so §1's rounded Cp reproduces each cell only to that):

| Port position | Mach 0.9 | 1.0 | 1.1 | 1.2 |
|---|---|---|---|---|
| 1 caliber aft | +110 / +100 m | +480 / +450 m | +480 / +440 m | +340 / +310 m |
| 2 calibers aft | +70 / +60 m | +360 / +340 m | +360 / +340 m | +120 / +110 m |
| 4 calibers aft | +30 / +30 m | −140 / −130 m | (tunnel) | +180 / +170 m |
| Fuselage vent [N2] | ≈0 | +470 / +440 m (1.01) | not read | +530 / +490 m (1.18) |

- **The step.** A port aft of the Mach-1 shock sees pressure jump as the shock passes: the −0.04 to +0.03 step is
  about −410 m at sea level, −390 m at 3 km, within about 0.04 in Mach [N3], which a rocket at 10 g gains in
  about 0.14 s. Coasting back through Mach 1 replays it in reverse.
- **At apogee it is gone:** Cp 0.05 at Mach 0.1 is about 3 m. It reaches a baro apogee only through filters,
  detection timing and lag (§4).

## 3. What the makers say about Mach

- **PerfectFlite:** a "mach dip" is an apparent altitude drop from higher pressure past Mach 1 [PF1] p. 4;
  "MachLock" waits for speed to stay below an unpublished threshold for an unpublished time [PF1] p. 40. The
  Alt15K refuses an apogee unless pressure changes smoothly [PF3] p. 12.
- **Missile Works:** the RRC3 drops anomalous velocity changes, most pronounced from sonic to subsonic [MW1]
  p. 24; the RRC2+ and RRC2L have built-in immunity [MW2] p. 1.
- **Featherweight:** baro "pressure increasing" can be briefly true going to and from supersonic flight, so the
  Raven also checks accelerometer velocity [F1] p. 8; the Blue Raven's default Vel1 < 400 ft/s (122 m/s) is
  labeled a transonic inhibit [F2] pp. 11–14.
- **Eggtimer:** above about 800 ft/s (244 m/s) shock pressure buildup can read as descent; deployment arms only
  after baro velocity stays under 100 ft/s for 1 s [E1] p. 30, [E2] p. 37, [E3] p. 31.
- **Altus Metrum:** keep the apogee lockout at 0, the Kalman filter is not fooled [A1] p. 45; baro is de-weighted
  near and above Mach 1, and was "wildly incorrect above mach due to poor static port design" in the worst
  flights seen [A1] p. 73. **Entacore** has no Mach rule [EN] p. 21; nor do **Jolly Logic's** guides [J1].
- **Their sign, pressure rising, is the tunnel's aft-port case:** PerfectFlite's ports sit at least 4–5 calibers
  from the shoulder [PF1] p. 11, behind [N1]'s Mach-1 shock (**Inference**). PerfectFlite, Featherweight,
  Eggtimer and Altus Metrum state that sign.
- **Hobby flight evidence.** Forums give rules of thumb [RF1], [RF2]; of 412 Spaceport America Cup reports, at
  least a dozen say transonic flight corrupts the barometer, and none sizes it.

## 4. How a vented bay lags

- **Viscous lag is first order [N4].** For a tube, Δp = λ·dp/dt with λ = 128·μ·L·C/(π·d⁴·p): length L, bore d,
  volume C, viscosity μ, pressure p [N4] eqs. (10.2)–(10.3), pp. 165–166; capillary tests scale as L·μ/d⁴ [N5]
  p. 1. Gracey's 100 in³ on 50 ft of 3/16 in tubing at 30,000 ft lags 1.0 s [N4] pp. 285–286. λ is 1.37 times
  its sea-level value at 3 km, 3.1 times at 10 km (**Inference**: 1/p, μ by Sutherland).
- **A rocket bay is an orifice, not a tube.** The vent is a hole through a 1–4 mm wall; Stroick sizes it with the
  compressible orifice equation, discharge coefficient Cd 0.62 for sharp edges [ST] Eq. 5, pp. 8–9.
- **Orifice-regime lag (Inference, derived here).** The bay's mass balance is (V/(n·R·T))·dp_b/dt = ṁ, with n = 1
  isothermal or 1.4 adiabatic; outside pressure falls at ρ·g·w for climb rate w [N4] pp. 284–286. In a steady climb
  the jet speed is u = w·Λ/(n·H) with Λ = V/(Cd·A), and the bay reads low by **Δh = u²/(2g)**: low going up,
  high coming down, independent of pressure. A transonic spike on top is smoothed over 2·Δh/w.
- **At apogee only the viscous τ is left;** it delays a parabolic peak by τ and lowers it by g·τ²/2.

## 5. The makers' vent rules, applied (Inference)

Three invented bays, inside diameter × free length with no sled: 38 × 150 mm (170 cm³), 54 × 200 mm (458 cm³),
98 × 300 mm (2,263 cm³). Total vent area each rule gives, mm² (single-hole equivalent, in):

| Rule | 38 × 150 | 54 × 200 | 98 × 300 |
|---|---|---|---|
| PerfectFlite: d = 0.0016·D²·L in, 1 hole or 4 of d/2 [PF1] p. 11 | 0.227 (0.021) | 1.64 (0.057) | 40.1 (0.281) |
| Missile Works: d = V/400 under 100 in³, ≥ 3 holes [MW1] pp. 4–5 | 0.341 (0.026) | 2.47 (0.070) | 43.7 (0.294) |
| Eggtimer: 1/4 in per 100 in³, 3 holes [E1] p. 7 | 3.29 (0.080) | 8.85 (0.132) | 43.7 (0.294) |
| Stroick / Entacore at Mach 1 [ST] Eq. 8, [EN] pp. 15–16 | 2.58 (0.071) | 6.94 (0.117) | 34.3 (0.260) |
| Largest / smallest | 14.5× | 5.4× | 1.3× |

- **They agree on placement:** 3–4 equal holes on the cylinder, clear of the nose joint, rail buttons and
  transitions, 2–5 calibers back [PF1] p. 11, [MW1] p. 5, [E1] p. 7, [P1] pp. 7–8. Featherweight gives no
  size but warns small vents on a big bay lag and may not catch up [F2] p. 7.
- **They disagree on size.** PerfectFlite's rule, and Missile Works' below 100 in³, grow area with volume
  squared; Eggtimer's and Missile Works' above 100 in³ grow it with volume and are the same rule. PerfectFlite's
  vents less than Eggtimer's below 151 in³ (2.47 L). Stroick's sizes for about 1 m of steady lag at the design
  speed; Eggtimer's equals it at Mach 1.28. He finds no stated derivation for four of five rules [ST] pp. 2–5.
- **Lag in an invented flight** (vertical, peak 365 m/s or Mach 1.07, apogee 3,320 m; orifice plus viscous flow
  through a 2 mm wall, Cd 0.62; isothermal / adiabatic bay):

| Bay and vents | Lag at Mach 0.9–1.07 | Apogee change | Peak delay |
|---|---|---|---|
| 98 mm, any maker's rule | 0.7–1.1 m / 0.4–0.6 m | 0.0 cm | under 1 ms |
| 54 mm, PerfectFlite | 20 m / 11 m | 0.0 cm | 2 ms |
| 38 mm, Missile Works | 81 m / 49 m | −2.1 cm | 65 ms |
| 38 mm, PerfectFlite | 135 m / 80 m | −1.3 cm | 52 ms |
| 98 mm, one 1/16 in hole | 231 m / 139 m | −0.05 cm | 10 ms |
| 98 mm, one 1/32 in hole | 679 m / 593 m | −157 m / −15 m | 3.1 s / 1.8 s |
| Leak, first-order τ = 2.5 s | 561 m | −31.8 m | 2.55 s |

- **Sensitivity and filtering.** Cd 0.8 instead of 0.62 cuts the boost lag by 30–40%; the sealed row hangs on n,
  which no source gives. Maker-sized vents in a 98 mm bay pass a transonic spike almost unfiltered (2·Δh/w of
  2–7 ms); the 38 mm PerfectFlite vent smooths it over about 0.4–0.8 s (n = 1.4 to 1).
- **The one measurement [P1].** In a 2.5 in BT-50 compartment on C6 flights, a glue-and-tape-sealed bay lagged
  about 2.5 s and peaked almost 30 m low; a leak-only bay tracked one with three 1/32 in holes, and nine times the
  area changed only the noise [P1] pp. 3–5.

## What this suggests

1. **M7.4's "Mach baro error":** Δh = −0.7·M²·Cp(calibers aft, M)·R·T/g from §1's table, with port position as
   an input: +100 to +480 m for a forward port, a −140 to −410 m step for one aft of the Mach-1 shock (§2).
2. **M7.4's "baro-port artifacts":** Λ = V/(Cd·A) for boost (Δh = (w·Λ/(n·H))²/(2g)) and a viscous τ scaling as
   1/p for apogee; a late and low peak (τ, g·τ²/2) marks a leak-only bay (§4–5).
3. **M7.2's readings:** flag baro samples above about Mach 0.9, up and back down; don't withhold an apogee
   reading for vent size alone (under 3 cm for any maker's rule), only for a near-sealed bay.
4. **M6.8's lockout check:** compare a user's Mach lockout with the simulated time above Mach 0.9, and warn when
   a port 3–5 calibers back meets a flight through Mach 1.
5. **The vent-hole idea:** OpenRocket #2442 (open since 2024-01-19) asks for a vent part with diameter, placement
   and count [OR]; with V and A, the simulator could show each maker's rule and its lag, most telling in small bays.

## Open questions

- **Ports near a boattail, on a nose cone or ahead of fins,** other noses, and angle of attack are unsized.
- **What a real flight shows.** A public baro-plus-accelerometer log through Mach 1 would size the dip and step;
  AerospaceNU's 2022-06-11 flight (about Mach 0.95–1.02, [AerospaceNU's flights](aerospacenu-flights.md)) is one.
- **The bay's thermal behavior** (n between 1 and 1.4) and Cd of holes drilled in fiberglass and cardboard move
  the lag tables by tens of percent, the near-sealed case by ten times. No high-power bay's lag was measured.

## Sources

NTRS reports are US government works, cached under the ignored `refs/`. Makers' manuals and the two hobby articles
are cited only; the Altus Metrum manual is Creative Commons ShareAlike 3.0. Manual pages are PDF pages. Read 2026-10-16.

- [N1] K. R. Czarnecki, W. J. Monta, NASA TN D-3516 (1966).
  <https://ntrs.nasa.gov/api/citations/19660024157/downloads/19660024157.pdf>
- [N2] W. Gracey, NACA Report 1364 (1958). <https://ntrs.nasa.gov/api/citations/19930092348/downloads/19930092348.pdf>
- [N3] Thompson, NACA RM L9J27 (1950). <https://ntrs.nasa.gov/api/citations/19930086141/downloads/19930086141.pdf>
- [N4] W. Gracey, *Measurement of Aircraft Speed and Altitude*, NASA RP-1046 (1980), ch. X and app. B.
  <https://ntrs.nasa.gov/api/citations/19800015804/downloads/19800015804.pdf>
- [N5] Sinclair, Robins, NACA TN 2793. <https://ntrs.nasa.gov/api/citations/19930083485/downloads/19930083485.pdf>
- [PF1] PerfectFlite, StratoLoggerCF manual; [PF3] Alt15K manual. <http://www.perfectflite.com/Downloads/>
- [MW1] Missile Works, RRC3 manual rev. 1.71 (2024); [MW2] RRC2+ and RRC2L manuals rev. 1.03.
  <https://www.missileworks.com/downloads/>
- [F1] [F2] Featherweight Raven (2014) and Blue Raven (2026-05-12) manuals. <https://www.featherweightaltimeters.com/>
- [E1] [E2] [E3] Eggtimer Quantum 1.09G, Proton 2.01C and TRS 2.07 guides. <https://eggtimerrocketry.com/>
- [A1] Altus Metrum manual v1.9.22.2 (2025-10-13). <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [EN] Entacore AIM XTRA/BASE manual 4.x.x v7 (2025). [J1] Jolly Logic AltimeterTwo guide 64G. <https://jollylogic.com/>
- [P1] B. Cawley Jr., Peak of Flight 543 (2021). <https://www.apogeerockets.com/education/downloads/Newsletter543.pdf>
- [ST] G. Stroick, "HPR Research – Static Port Holes from Nescience to Science".
  <https://offwegorocketry.com/userfiles/file/Documents/Static%20Port%20Holes.pdf>
- [OR] OpenRocket issue #2442, "Vent hole component". <https://github.com/openrocket/openrocket/issues/2442>
- [RF1] [RF2] Rocketry Forum: vent-hole placement (2017); "10,000 ft." (2006), p. 2. <https://www.rocketryforum.com/>
