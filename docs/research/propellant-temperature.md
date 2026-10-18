# How grain temperature shifts a COTS motor's curve

What does a motor's grain temperature do to its thrust curve and to apogee? It serves the "propellant temperature
correction" idea ([ideas](ideas-design-and-physics.md)), M2.8's dispersion runs ([roadmap](../ROADMAP.md))
and the cold-day flights in [UND flight configs](und-flight-configs.md); it qualifies [apogee bias](apogee-bias.md)
§1's "shape, not impulse" bullet. Read 2026-10-18: NASA monographs, three test-motor papers (39 firings counted
in two), a patent, a thesis, makers' sheets, certifiers' rules; no source code. **Inference** marks this note's
arithmetic. Trust the coefficients for military and sounding-rocket AP/HTPB/Al. **No COTS hobby propellant has a
published temperature sensitivity**: every hobby number is borrowed, every apogee size from a generic model.

**In short.**
- **Warm grain burns shorter and harder.** Five measured AP/HTPB/Al propellants have a pressure sensitivity π_K of
  0.15–0.32 %/K. In 39 test-motor firings at −40 to +60 °C (two papers), each 10 K warmer cut burn time 1.2–2.2% and
  raised mean pressure 1.6–3.6%; peak pressure +1.0 to +2.1% in 12 of them, mostly above 15 °C (§1–2).
- **Impulse is not quite constant; warm is higher.** c* gains about 0.1% per 10 K; one set of 27 firings delivered
  0.31–0.80% more impulse per 10 K, more than its authors could explain. **Inference:** 0.1–0.8% per 10 K (§3).
- **A certified curve is a 15–25 °C grain** (CAR's to 27 °C): NFPA 1125 asks "at or corrected to … 20°C ± 5°C"; makers
  give storage or use limits (Cesaroni −5 to 30 °C) and nothing on performance (§4).
- **Apogee (Inference).** At fixed impulse the shape alone moves apogee −0.2 to +0.4% per 10 K (the sign flips with
  speed), but 1–2% for a slow, long burn. The impulse term adds 0.06–1.3%, warm higher. A cold morning (grain 20 K
  below its curve) loses 0.1–2.6% to impulse, ±1% to shape; the day's colder air 0.5–1.7% per 10 K (§6).
- **A motor lags the air by up to hours.** A 54 mm motor takes about 50 min (breezy) to 3.5 h (still air) to close
  90% of a step change (**Inference**, §5). Grain temperature does not explain UND's cold-day miss (§6).

## 1. Published sensitivities

σ_p = (∂ln r/∂T)_p is burn rate's sensitivity at fixed pressure; π_K = (∂ln p_c/∂T)_K is chamber pressure's at a
fixed burning-to-throat area ratio K [1, p. 39; 2, pp. 11–12]. The test reports use π_K = σ_p/(1 − n), n the
burn-rate exponent [4, p. 590, eq. 3; 5, p. 84, eq. 9].

| Propellant (source) | n | σ_p, %/K | π_K, %/K | Range, K |
|---|---|---|---|---|
| AP/HTPB/Al 70/15, iron catalyst [4] | 0.11–0.14 (Inf.) | 0.11 | 0.15–0.20 | 233–323 |
| AP/HTPB/Al 70/16, Catocene 0.2% [5] | 0.13–0.19 | 0.16–0.22 (Inf.) | 0.20, 0.25 | 233–323 |
| VSB-30 PC1, second stage [6] | 0.22–0.29 | 0.11–0.15 (Inf.) | 0.150–0.193 | 233–333 |
| VSB-30 PC2, roll motors [6] | 0.23–0.32 | 0.14–0.24 (Inf.) | 0.208–0.318 | 231–333 |
| VSB-30 PC3, first-stage booster [6] | 0.38–0.43 | 0.14–0.16 (Inf.) | 0.246–0.262 | 233–333 |

- Read at [4] p. 599 (σ_p 0.0011 K⁻¹; n from its eq. 10), p. 600 (π_K 0.0015–0.0020 K⁻¹ by K); [5] pp. 86–87 (π_K
  about 0.0025 K⁻¹ for 233–291 K, 0.002 K⁻¹ for 291–323 K); [6] p. 9 (burn laws), p. 10 (Table 3). "Inf." is (1 −
  n)·π_K with each n.
- SP-8064: low n usually goes with low σ_p; n runs 0.2–0.8 [1, pp. 36, 39–40]. The monographs give no typical σ_p.
- **Not found:** any value for AP/PBAN, AP/CTPB or a hobby formulation (white, blue, black, red, strontium,
  magnesium). For contrast, a KNSB thesis fits σ_p 0.33 and π_K 0.48 %/K [12, eqs. 6.22–6.23, p. 120]; its conclusion
  (p. 130) prints 0.0052 and 0.0038 K⁻¹, and only p. 120's pair satisfies π_K = σ_p/(1 − n) at its n = 0.305
  (**Inference**).

## 2. From sensitivity to the curve, and what firings show

SP-8039 puts temperature into the burn-rate law as a = a_ref·exp(σ_p ΔT) and the pressure as
p = p_ref·exp(π_K ΔT) [3, p. 61, eqs. 53–54]. At fixed K burn rate follows pressure, so per 10 K warmer burn time
scales by exp(−10 π_K) and pressure (so roughly thrust) by exp(+10 π_K) (**Inference**): at π_K = 0.15, 0.25 and
0.32 %/K, burn time −1.5%, −2.5%, −3.1%, thrust +1.5%, +2.5%, +3.3%.

- **Florczak et al. 2014** [4]: one tubular AP/HTPB/Al charge, 4 throats × 3 temperatures (12 firings), soaked 6 h
  at 233, 288 or 323 K (p. 596; Table 2, p. 600). **Inference** from Table 2, 233 → 323 K, per 10 K: burn time
  −1.8 to −2.1%, peak pressure +1.0 to +2.1%, mean pressure +1.6 to +2.1%. The cold half moves less: on the 11 mm
  throat peak pressure rose 1.3% from 233 to 288 K, 7.8% from 288 to 323 K. Pressure integral: −0.3 to 0.0%.
- **Corrêa et al. 2006** [6]: three VSB-30 propellants, 3 throats × 3 temperatures each (27 firings), conditioned
  to −40, ambient and +60 °C (pp. 4–6). **Inference**, coldest → hottest per 10 K: burn time −1.2 to −2.2% (median
  −1.6%), mean pressure +1.8 to +3.6% (median +2.8%). Ignition delay grew in the cold, 0.58 → 0.71 s on PC1's
  largest throat (p. 10, Table 2). Reading Table 1 as one throat per block of three rows is this note's.
- **A Navy patent** [11] says a composite motor's curve shape "changes little with motor conditioning
  temperature" and fits hot and cold curves by stretching one baseline to the interpolated impulse and action time.
  Its two worked examples (0 and 120 °F) give action time −1.8% and −2.8% per 10 K (**Inference**); it doesn't say
  whether they are real motors. RocketPy's stochastic motor (MIT) uses the same stretch, with no temperature input
  and no tie between impulse and burn time [25].
- **Hobby APCP at two stated temperatures: none found** on RocketryForum, ThrustCurve, theses or NAR's R&D index;
  two NAR reports on air temperature and on cold black-powder motors need a member login [24].

## 3. Impulse: a little, warm higher

- **c\*.** SP-8076: c* varies 0.5–0.75% per 100 degrees of conditioning temperature, from the grain's sensible heat
  [2, p. 60]: 0.09–0.14% per 10 K if °F, 0.05–0.08% if kelvin (**Inference**). [6]'s measured c* moved −0.10 to
  +0.35% per 10 K, median +0.14%.
- **Delivered impulse.** In [6] (Table 2, p. 10) specific impulse rose at every throat, +3.2 to +8.5% from 231–233
  K to 333 K, or +0.31 to +0.80% per 10 K (**Inference**, median +0.53%); more than pressure explains (p. 11).
- **Part of the gap is the nozzle (Inference).** At a fixed nozzle, sea-level C_F grows with chamber pressure
  through the (p_e − p_a)ε/p_c term ([SP-8039 definitions](nasa-sp-8039-motor-definitions.md)):
  d ln C_F/d ln p_c = ε p_a/(p_c C_F). With AeroTech's ε of 2.3–7.5 ([nozzle exits](nozzle-exits.md) §3), an assumed
  p_c of 3–7 MPa and C_F of 1.3–1.6, that is +0.03 to +0.7% of impulse per 10 K.
- **What this changes.** Apogee bias's "shape, not impulse" holds for c* and [4]'s pressure integral, not for the
  thrust integral: 0.1–0.8% per 10 K, warm higher, from test motors at 3.3–9.2 MPa (the patent's examples: +0.2%,
  +0.5% [11]). A c* rise lifts the pressure integral too [2, p. 12]: in [6] by 0.3–1.8% per 10 K (**Inference**).

## 4. What makers and certifiers say

- **Makers give limits, not effects.** Cesaroni's six current instructions (Pro24 to Pro98) say to keep the reload
  between −5 °C and 30 °C "for storage and use" [13, p. 1 of each]; its safety data sheet's −40 to +40 °C is storage
  stability [14, §10.2, p. 7]. AeroTech's DMS sheet says store at 45–100 °F (7–38 °C) [15, p. 1]; its RMS-38 Plus
  and HP-G138T sheets say only store dry, and its FAQ has no temperature entry [16]. Loki's four sheets and FAQ give
  none [17]. No maker states a change in burn time, thrust or delay.
- **Builders' reports.** Cold composites are "sometimes a little hard to light" [18, thread 1, post 23]; one flyer
  reports about 250 launches near −10 °C without a CATO [18, thread 2, post 5]. A balloon-launch team found an APCP
  motor "will not burn if it's frozen" after 30 min in dry ice, while one cooled to about −50 °C fired [19].
- **The curve's basis.** NFPA 1125 asks model and high-power certification "static testing, conducted at or corrected
  to sea level and 20°C ± 5°C" [20, §8.1.7(1), p. 5; §8.2.7(1), p. 7]; no hot or cold firing is listed. NAR conditions
  motors to 20 ± 5 °C [21, §8.3.3, p. 16], CAR stores them at 60–80 °F [22, §11.2.2, p. 17], and TMT names no
  temperature ([TMT test sites](tmt-test-sites.md) §5). Of 157 cached NAR sheets 18 have a test temperature field,
  reading 17, 22 or 23 °C [23]; no TRA letter or ThrustCurve field ([data](thrustcurve-data.md) §1) gives one.
  **Inference:** NFPA's ±5 K alone is ±0.6–1.1% of burn time at the measured rates.

## 5. How long a motor takes to reach the air's temperature

- SP-8064 has test grains soak until embedded thermocouples show no gradient [1, p. 72]; [4] soaked charges 6 h.
- **Model (Inference):** radial conduction through a cored grain (bore a third of its radius), 1 mm liner, 2 mm
  aluminum case, a step change in air. Diffusivity 2.2 × 10⁻⁷ m²/s (measured 2.1–2.3 [7, p. 13]); ρ 1,730 kg/m³ (from
  [5]'s Table 2 masses, p. 85); c_p about 1.28 kJ/(kg·K) from AP, binder [8, p. 1] and aluminum [9]; h of 5–25
  W/(m²·K) (free convection in gases spans 2–25 [10]).
- Minutes for the grain's mean to close 90% of the step, at h = 5, 10, 25 W/(m²·K), then conduction alone: 29 mm 105,
  53, 22, 2; 54 mm 205, 107, 48, 10; 98 mm 395, 214, 107, 38. The air film, not the propellant, sets the time: a 54 mm
  motor from a 20 °C car averages 5, 11 or 14 °C after 30 min on a 0 °C pad at h = 25, 10 or 5.
- SP-8039 models a web gradient as firing temperature against web burned [3, p. 61]. **Inference:** a core burner
  warmed or cooled from outside burns its inner web at the old temperature first, so its curve tilts.

## 6. What it does to apogee, with its sign

No published hobby or sounding-rocket study of apogee per kelvin of grain temperature was found. **Model
(Inference):** a 1-D point mass, RK4 at 2 ms, ISA offset at fixed sea-level pressure, drag rising past Mach 0.8.
Warm grain: burn time ÷(1 + k), thrust ×(1 + k), impulse fixed, k = 1.5–3% per 10 K (§2). Seven invented rockets;
the altimeter's read error and thrust at altitude are left out.

| Rocket (invented) | Apogee | Peak speed | Grain +10 K, shape only | Air −10 K | Air +10 K |
|---|---|---|---|---|---|
| G, 1.6 s burn | 677 m | 132 m/s | +0.05 to +0.10% | −1.18% | +1.13% |
| H, 3.5 s, mean T/W 4 | 646 m | 109 m/s | +0.21 to +0.40% | −0.93% | +0.89% |
| H, 7.0 s, mean T/W 2 | 401 m | 62 m/s | +1.1 to +2.2% | −0.46% | +0.43% |
| H, 0.8 s | 1,097 m | 178 m/s | +0.02 to +0.04% | −1.15% | +1.11% |
| K, 2.0 s, 98 mm | 1,190 m | 164 m/s | +0.09 to +0.17% | −0.86% | +0.82% |
| M, 4.0 s, 157 mm | 2,993 m | 301 m/s | −0.05 to −0.10% | −1.71% | +1.65% |
| M, 3.5 s, 80 mm | 8,535 m | 754 m/s | −0.09 to −0.19% | −1.60% | +1.58% |

- **Shape term.** A shorter, harder burn cuts gravity loss (helps slow rockets) and raises peak speed and drag
  (hurts fast ones), so warm grain raises a subsonic apogee and lowers a transonic or supersonic one. It stays under
  0.4% per 10 K unless mean thrust is near twice liftoff weight, where it reaches 1–2%.
- **Impulse term.** 0.1–0.8% per 10 K (§3) times an apogee elasticity of 0.6–1.3 ([apogee bias](apogee-bias.md))
  to 1.0–1.6 (six of the model's rockets) is +0.06 to +1.3% per 10 K, warm higher: as large as the shape term or
  larger, and of the same sign on slow rockets (**Inference**).
- **Cold morning** (grain and air 0 °C; grain −20 K from its curve, air −15 K from ISA): at fixed impulse −1.5% to
  −2.5% for six rockets, the grain's share −0.9 to +0.4 points, the impulse term a further −0.1 to −2.6. The T/W 2
  rocket loses 3.0–5.5%. **Hot desert** (grain 45 °C, air 35 °C): +1.8% to +3.1% at fixed impulse for six, the
  T/W 2 rocket +3.6% to +6.1%, plus 0.15–3.2% from impulse.
- **UND's cold flight (Inference).** At −20 to −29 K the L1420R's 3.24 s burn [26] stretches to 3.34–3.53 s. The
  profile fit wanted 4.6–5.1 s and 20–28% more impulse ([UND flight configs](und-flight-configs.md), 2023 season):
  grain temperature covers about a fifth of the lengthening at most and goes the wrong way on impulse.

## What this suggests

- **A grain-temperature input as a curve stretch:** burn time × exp(−π_K ΔT), thrust × exp(+π_K ΔT), ΔT from
  20 °C, π_K 0.25 %/K (0.15–0.32), with an impulse factor of 0.1–0.8% per 10 K stated as borrowed from test motors.
  Tests could pin the sign: warm grain, shorter burn, higher impulse.
- **M2.8's dispersion:** a grain spread of 5–10 K about 20 °C is a burn-time σ of 0.8–3% (**Inference**), small
  beside impulse. A soak model (§5) would say how far a motor fresh from a car sits from the air.
- **A notice, not a verdict,** when a grain is outside NFPA's 15–25 °C basis or Cesaroni's −5 to 30 °C.
- **Qualify apogee bias's bullet:** impulse moves too, warm higher. **Cold-day flights:** state air and grain
  temperature with each; grain temperature does not explain the UND cold miss.

## Open questions

- σ_p, n and impulse against temperature for any COTS hobby propellant; a hobby motor fired hot and cold.
- The c*/C_F split of [6]'s impulse rise; cold composite delays; how fast a motor on a pad follows the air or sun.
- Unread: Sutton and Biblarz (no legal copy); DTIC (down); a DRDO 0.27 %/°C (unreachable); a 0.43 %/K (paywalled).

## Sources

All read 2026-10-18. `ntrs:` is `https://ntrs.nasa.gov/citations/`, `ipo:` is
`https://wydawnictwa.ipo.lukasiewicz.gov.pl/wp-content/uploads/2021/04/`, `trf:` is
`https://www.rocketryforum.com/threads/`, `p38:` is `http://pro38.com/wp-content/uploads/`.

1. NASA SP-8064, *Solid Propellant Selection and Characterization* (1971), pp. 36, 39–40, 72. `ntrs:19720006088`
2. NASA SP-8076, *Solid Propellant Grain Design and Internal Ballistics* (1972), pp. 11–12, 60–61. `ntrs:19730007077`
3. NASA SP-8039, *Solid Rocket Motor Performance Analysis and Prediction* (1971), p. 61. <https://ntrs.nasa.gov/>
4. Florczak, Białek, Cholewiak, Cent. Eur. J. Energ. Mater. 11(4), 589–601 (2014). `ipo:Florczaki-inni.pdf`
5. Florczak, High Energy Materials 11(2), 83–88 (2019). `ipo:HEM_0185.pdf`
6. Corrêa, Shimote, Rapozo, Lacava, ENCIT 2006, paper CIT06-1015 (Portuguese).
   <https://abcm.org.br/app/webroot/anais/encit/2006/arquivos/Combustion%20and%20Fuels/CIT06-1015.pdf>
7. Erikson et al., SAND2012-9907C (JANNAF 2012), p. 13. <https://www.osti.gov/servlets/purl/1294000>
8. Martinez-Sanchez, MIT 16.512 lec. 17–18, p. 1. <https://ocw.mit.edu/courses/16-512-rocket-propulsion-fall-2005/>
9. NIST WebBook, aluminum, solid phase at 298 K. <https://webbook.nist.gov/cgi/cbook.cgi?ID=C7429905&Mask=2>
10. Incropera et al., 6th ed., Table 1.1, as reproduced by Michigan Tech.
    <https://pages.mtu.edu/~fmorriso/cm3120/typical_values_heat_transfer_coefficient.pdf>
11. A. F. Garcia (US Navy), US 6,290,505 B1 (filed 1999-04-19). <https://patents.google.com/patent/US6290505B1/en>
12. Olde, MSc thesis (2019). <https://repository.tudelft.nl/file/File_b5f6bc4e-cefc-4b39-a2b6-ffeffd094531>
13. Cesaroni instructions, `p38:2026/08/CTI-AERO-MAN-101-v4.00-Pro24-Instructions_EN.pdf`; `-103` Pro29, `-105`
    Pro38, `-107` Pro54, `-109-v5.00` Pro75, `-111-v6.00` Pro98.
14. Cesaroni SDS ProX 4.05 (2025-01-02), `p38:2025/08/SDS-ProX-ver4-05-EN_FR.pdf`.
15. AeroTech DMS instructions, P/N 21049. <https://www.apogeerockets.com/downloads/PDFs/29-54mm-DMS-Instructions.pdf>
16. AeroTech RMS-38 Plus (10/22/20) and HP-G138T (8/11/10) sheets; FAQ, <https://aerotech-rocketry.com/pages/faq>
17. Loki Research documents and FAQ. <http://www.lokiresearch.com/page/Documents>
18. RocketryForum: "How does cold affect AP motors?" (2015), `trf:how-does-cold-affect-ap-motors.126140/`;
    "Aerotech White Lightning Composite Motors and Cold Temperature Questions" (2025), thread 190526.
19. L. Haines, *The Register*, 2014-04-02. <https://www.theregister.com/2014/04/02/lohan_rehab_tests/>
20. NFPA 1125, A2021 first-revision report (2019-10-25).
    <https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf>
21. NAR S&T Motor Testing Manual v1.5. <https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf>
22. CAR testing manual. <https://car-acf.org/resources/Documents/car_motor_testing_manual%20r20181121.pdf>
23. NAR certification data sheets, via ThrustCurve's links and the Internet Archive ([apogee bias](apogee-bias.md)).
24. NAR R&D report index (titles public). <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
25. RocketPy at `9bd6ad3af8`, `rocketpy/motors/`, `rocketpy/stochastic/`. <https://github.com/RocketPy-Team/RocketPy>
26. ThrustCurve, AeroTech L1420R (4,603 N·s, 3.24 s). <https://www.thrustcurve.org/>
