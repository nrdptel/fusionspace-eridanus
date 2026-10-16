# How far two altimeters on one rocket disagree in published flights

This note asks how far two barometric altimeters on one rocket disagree on apogee in published flights, and whether
the gap has a sign by family, port size or bay venting. It builds on [AerospaceNU's flights](aerospacenu-flights.md)
(7 two-logger flights, 0.3–5.7% apart read from pressure) and [apogee bias](apogee-bias.md) §3, which sizes the
measurement from datasheets: a few meters either way. Flights come from Student Launch reports, one club's launch
reports and one builder's table; mechanisms from manuals, datasheets and two newsletter experiments. Spread is
(highest − lowest) / lowest apogee on one flight. **Inference** marks this note's arithmetic. Read 2026-10-15.

**In short.** Two commercial altimeters on one rocket usually agree to a few tenths of a percent, but about one
flight in five differs by over 2%, and no source found measures what causes those.
- **59 published flights, median spread 0.37% (4.0 m).** Quartiles 0.21% and 1.72%; 11 flights over 2%. The two
  largest, 18.8% and 15.7%, are club flights; without them the largest is 6.56% (56.7 m) (§1).
- **The source matters more than the sensor.** Student Launch and youth-team reports have a median of 0.27%, one
  builder's table 0.35%, one club's quick reports 1.52%. Two of one model can disagree: two StratoLogger CFs in one
  bay read 2.79% (43 m) apart, two RRC2s in one rocket 15.7% (§1).
- **No family sign that survives its sample.** An EasyMini read 0.5–3.7% above a Missile Works RRC2 on 6 of 6
  flights, but all six are one builder's rockets. Raven against Eggtimer Classic: 9 higher, 8 lower, 1 equal (§2).
- **Two causes carry a sign:** how a unit turns pressure into height, and a sealed bay. In one model-rocket test,
  two of three altimeters in a sealed compartment peaked about 30 m (19%) low; a makers' port lags milliseconds (§4).
- **For HPR Sim.** A pair shows only the error its two units don't share; the standard atmosphere's warm-day error
  is shared, and the scored logs already replace it with ERA5's air. What is left is about 0.4% of apogee per unit,
  either sign (**Inference**): it widens the error, not the mean bias (§5).
- **AerospaceNU's pairs are wide.** Three published pairs exceed its +5.7%; its median, about 2.0%, sits near the
  published 81st percentile. Read from pressure through one formula, conversion can't explain it; where the boards
  sit and how they sample pressure could (§4, §5).

## 1. The published flights

Every pair was read from its source document, not a summary; the spread is per flight.

| Source | Flights | Median | Largest | Over 2% |
|---|--:|--:|--:|--:|
| Student Launch and youth-team reports [L1–L10] | 14 | 0.27% | 2.79% | 1 |
| Champlain Region club launch reports, 2020–2026 [L14] | 26 | 1.52% | 18.8% | 10 |
| One builder's table, Xi series [N1] | 18 | 0.35% | 1.19% | 0 |
| Forum report, StratoLogger CF and RRC3 [L15] | 1 | 0.50% | 0.50% | 0 |
| Leaving out the 18.8% and 15.7% flights | 57 | 0.37% (3.7 m) | 6.56% (56.7 m) | 9 |
| All | 59 | 0.37% (4.0 m) | 18.8% (165 m) | 11 |

- **Student Launch reports agree closely, with one exception.** NC State's three 2026 flights put a Silicdyne Fluctus
  and an EasyMini 0.16%, 0.16% and 0.26% apart, 1.2–3.7 m [L3] p. 79, [L4] p. 13, [L5] p. 2. Explorer Post 1010's
  two RRC3s in one sealed electronics bay differ by −3 to +11 ft on 5 flights, 0.09–1.01% [L6], [L7] PDF pp. 9, 12;
  three of its youth-contest flights to about 200 m agree to 0.16–1.16% [L9], [L10]. But NC State's 2017 full-scale
  flight had two StratoLogger CFs in one bay with four 1/4 in vents: 5,091 and 5,233 ft, 2.79% (43.3 m) apart, with
  no cause given [L1] pp. 27, 51–52.
- **The club's reports are wider.** Its yearly pages give two or three apogees for 26 flights, 2020–2026, at
  956–13,851 ft [L14]; many don't say which unit gave which, so they count toward the spread but not the sign.
- **Two identical units can split widely.** One 4 in rocket flew two RRC2s to 2,834 and 3,020 ft (6.56%) and, later
  in 2022, to 4,148 and 4,801 ft (15.7%, 199 m) [L14], with no cause given. The same rocket's two RRC2+ units read
  2,753 and 2,754 ft in 2018 (**Inference**: the same rocket, by its kit name and size). A gap that size is a fault
  or an installation event, not sensor scatter (**Inference**).
- **The 18.8% flight may not be a sensor gap.** An RRC3 and a MARSA read 3,431 and 2,888 ft (2024) [L14]. A MARSA
  logs barometric, integrated and Kalman altitudes: 3,554, 4,155 (16.9% higher) and 3,530 ft on a 2020 flight
  [L14]. **Inference:** if the 3,431 was the MARSA's integrated altitude, that explains the gap; the page doesn't
  say which unit gave which.
- **One builder's table is the cleanest sample.** Richard Nakka's Xi rockets flew a Featherweight Raven and an
  Eggtimer Classic on 18 flights to 421–1,347 m, and a BREO-N from Xi-6 on (13 flights), at launch temperatures of
  −17 to +30 °C [N1]. His page gives an average per-flight spread of 0.5%; read from its figure, the median is 0.35%
  and the largest 1.19% (10.7 m), and agreement doesn't change with temperature [N1] Figs. 1–3.
- **Ground checks agree too:** four contest altimeters within 2–7 ft at 500–1,200 ft in a vacuum chamber
  (0.28–0.66%) [P2] p. 8, Fig. 7; two at 792 and 796 ft (0.5%) in NC State's [L11] p. 133.
- **Left out:** homebuilt loggers (0.7–2.0% from a commercial unit [L12] pp. 2–3, [L13] p. 13), a low-power forum
  comparison under 80 m, a pair given only as "within 7 feet" [L16], and a 2023 paper's pair seen only as a snippet.

## 2. Is there a sign by family?

**No published family offset survives its sample.** The pairs that look signed come from one builder each.
- **EasyMini above Missile Works RRC2/RRC2L, 6 of 6:** +3.7, +0.5, +1.9, +0.9, +1.9 and +2.1% (+19 to +139 ft)
  [L14]. It is also above a StratoLogger CF once (+1.8% [L2] p. 78) and an Altimeter3 three times (+0.1 to +1.0%),
  but 0.16–0.26% below the Fluctus on all three NC State flights. All six RRC2 flights are one club member's
  rockets, so a family offset can't be told from that builder's bays (**Inference**).
- **Raven against Eggtimer Classic, none:** the Eggtimer higher on 9 flights, lower on 8, equal on 1; mean +0.16%,
  median +0.01%. The BREO-N was above the Raven on 10 of 13, mean +0.26% [N1] Fig. 2.
- **RRC3 against Eggtimer's Proton, Quantum and Quasar:** −0.13, +0.20 and +0.07%, all small [L14].
- **MARSA, both signs:** +4.4% and −1.7% against an RRC3, −2.2% against a PerfectFlite, all one member's [L14].
- **Same model:** two RRC3s 0.09–1.01% (5 flights), two RRC2s 1.32%, 6.56% and 15.7%, an RRC2 and an RRC2L 0.07%,
  two StratoLogger CFs 2.79% [L6], [L14], [L1].

## 3. How each family turns pressure into a reported apogee

Every maker that states an onboard formula uses the standard atmosphere; four manuals don't say.
- **Standard atmosphere above the pad:** PerfectFlite, which tracks the pad and finds apogee from a smoothed speed
  [PF1] PDF pp. 4, 21; Missile Works' RRC3, an MS5607 read through a running-average pad and a filtered speed [MW]
  pp. 2, 23–24. Featherweight's Raven states the standard atmosphere [F1] p. 12.
- **Above the recorded pad, formula not stated:** Altus Metrum, an MS5607 through a Kalman filter [A1] pp. 45, 63, 84.
- **Not stated:** Eggtimer [E1], Jolly Logic [J1] and Entacore, which also uses a Kalman filter [EN] p. 7.
- **Two units with the same formula share its error.** Featherweight warns that warm air can make standard-atmosphere
  altitudes 10% or more low [F1] p. 12. On 43 flights Nakka's GPS read 3.5% below his Raven on average, from −14%
  to +6% with launch temperature, while his altimeters still agreed to about 0.5% [N1] Figs. 1–3.
- **A different formula gives a signed gap (Inference, computed in the standard atmosphere).** A unit that treats the
  pad as sea level reads high by 0.68% at a 300 m pad and 2.78% at 1,200 m. A unit corrected by the measured pad
  temperature differs from a standard one by about +0.35% per kelvin above standard: +7.7% on a 35 °C day at a
  300 m pad. No onboard conversion read does either; Missile Works' PC software offers the second after the flight,
  from measured temperatures [MW] pp. 23–24, as Nakka's correction does [N1].

## 4. Ports, bays and the smaller terms

- **A sealed bay can read low; a vented one doesn't care much.** On a model rocket, two of three altimeters in a
  sealed compartment lagged about 2.5 s and peaked almost 30 m lower, about 19% of a 153 m flight; the third, a
  Firefly, read 153.9 m against 153.0 m vented. A leak-vented compartment matched one with three 1/32 in holes, and
  nine times the vent area changed only the noise [P1] pp. 3–5. No source found measures a high-power bay.
- **The makers' port rules differ by about 1.8 times in area.** For a 3.0 in × 12 in bay: PerfectFlite one 0.170 in
  hole [PF2] p. 9; Missile Works 0.212 in, at least three [MW] p. 5; Eggtimer one 1/4 in hole per 100 in³ [E1] p. 7.
  Featherweight gives no rule but warns that small holes on a large bay lag and "may not fully catch up" [F2] p. 7.
  **Inference:** any of these lags milliseconds, worth well under 1 m ([apogee bias](apogee-bias.md) §3).
- **Sensors, between two units (Inference, from the datasheet).** An MS5607 holds ±0.5 mbar after the pad zero at
  25 °C and 700–1,100 mbar, ±1.0 mbar over 0–50 °C [S1] p. 4. Two at ±0.5 mbar can differ by up to ±8.6 m at
  300 m (2.9%), ±9.6 m at 1,500 m (0.6%), ±11 m at 3,000 m (0.4%). PerfectFlite states ±(0.1% + 1 ft) [PF2] p. 23.
- **The rule that approves contest altimeters allows 2%.** NAR's draft approval rule asks for 1% of altitude or 2 m,
  whichever is greater [N2] rule 14.2, so two approved units may differ by 2% (**Inference**).
- **Mach, peak picking, rounding.** Makers hold off or screen apogee detection through transonic flight ([PF1] p. 4,
  [MW] p. 24, [E1] p. 30); Altus Metrum also de-weights pressure there [A1] p. 73. No source sizes what is left at
  apogee. A raw peak reads high by about one noise width, under 1 m; whole-foot readouts add ±0.15 m (**Inference**).

## 5. What it means for the measurement term

- **The datasheet's few meters holds for the median pair, not the tail.** The published median is 4.0 m; a quarter
  of pairs differ by over 1.72%, 11 of 59 by over 2%, and the two largest by 165 m and 199 m.
- **A pair bounds only the error its units don't share.** The standard atmosphere's temperature error is shared and
  is already replaced by ERA5's air in the scored logs. **Inference:** if two units err alike, independently and
  normally, a median gap of 0.37% means about 0.39% per unit (median |a − b| = 0.954 σ); the tail is heavier.
- **That scatter has either sign.** With no family offset that survives its sample, it widens the mean absolute
  error and leaves [apogee bias](apogee-bias.md)'s mean where it was (+9.83% on the first 55 flights, +8.73% on 53).

## What this suggests

1. **Use 0.4% per unit, either sign, as the measured scatter in [apogee bias](apogee-bias.md) §3** next to the
   datasheet bound, and say one pair in five differs by over 2% (**Inference**, from §5). Issue #507 asks for this
   term; this note gives the published side of it.
2. **Read a unit's pressure, not its height column.** [AerospaceNU's flights](aerospacenu-flights.md) found two
   height columns 21% and 5.8% above their own pressure, and a MARSA's integrated altitude ran 16.9% high (§1).
3. **When a flight carries two altimeters, record their gap,** and treat one over about 2% as a flag to explain
   before scoring the flight: the published tail holds faults, not scatter.

## Open questions

- **What causes the tail over 2%?** No source found measures it. A bench test of two families in one vented bay
  and in two bays, through the same pressure profile, would separate the sensor, the conversion and the bay.
- **Is the EasyMini–RRC2 offset real?** Six flights from one builder can't tell; a second builder's pairs could.
- **Which formula do Eggtimer, Jolly Logic, Entacore and AltOS use, and which sensor do five families use?** The
  manuals don't say; AltOS's source is GPL and was not read.
- **How large is the error a Mach lockout leaves at apogee,** and how much does a leak-vented high-power bay lag?

## Sources

Pages and PDFs with no stated license are cited only, cached under the ignored `refs/`; the Altus Metrum manual is
Creative Commons ShareAlike 3.0. Club rocket and flier names are left out. Manuals: PDF pages; reports: printed pages.

- [L1] NC State Tacho Lycos, 2017 FRR, pp. 27, 51–52.
  <https://ncsurocketry.org/wp-content/uploads/2019/09/2017_tacho_lycos_frr.pdf>
- [L2] NC State, 2025 FRR, p. 78.
  <https://ncsurocketry.org/wp-content/uploads/2025/03/north-carolina-state-university-2025-frr-report.pdf>
- [L3] NC State, 2026 FRR, pp. 79–80.
  <https://ncsurocketry.org/wp-content/uploads/2026/03/north-carolina-state-university-2026-frr-report.pdf>
- [L4] NC State, 2026 FRR addendum, p. 13.
  <https://ncsurocketry.org/wp-content/uploads/2026/04/North_Carolina_State_University___FRR_Addendum.pdf>
- [L5] NC State, 2026 PLAR, p. 2.
  <https://ncsurocketry.org/wp-content/uploads/2026/05/North-Carolina-State-University-2026-PLAR-Report.pdf>
- [L6] Explorer Post 1010, 2022 altimeter data sheets (VDF, PDF1–3, Demo).
  <https://post1010.org/Rocketry/Rocketry2022/SLI2022/Data/>
- [L7] Explorer Post 1010, 2022 FRR, PDF pp. 9, 12. <https://post1010.org/Rocketry/Rocketry2022/SLI2022/FRR/>
- [L8] Washington University in St. Louis, 2021 CDR, p. 24.
  <https://sites.wustl.edu/wurocketry/files/2021/02/Washington-University-in-St.-Louis-2021-CDR-Report.pdf>
- [L9] Explorer Post 1010, contest flight list. <https://post1010.org/Rocketry/Rocketry2020/Data/listAll.xlsx>
- [L10] Explorer Post 1010, altimeter inventory. <https://post1010.org/Rocketry/Rocketry2019/altimeters.xls>
- [L11] NC State, 2020 FRR, p. 133.
  <https://ncsurocketry.org/wp-content/uploads/2020/03/north-carolina-state-university-2020-frr-document.pdf>
- [L12] NC State, 2024 PLAR, pp. 2–3.
  <https://ncsurocketry.org/wp-content/uploads/2024/08/north-carolina-state-university-2024-plar-report.pdf>
- [L13] NC State, 2025 PLAR, p. 13.
  <https://ncsurocketry.org/wp-content/uploads/2025/06/north-carolina-state-university-2025-plar-report.pdf>
- [L14] Champlain Region Model Rocket Club, launch reports, a page a year: <https://crmrc.org/2020_launch_reports.htm>
- [L15] Rocketry Forum, "POTROCS Launch Reports", post 16.
  <https://www.rocketryforum.com/threads/potrocs-launch-reports.153499/>
- [L16] Rocketry Forum, "Altimeters vs. GPS?" (2024). <https://www.rocketryforum.com/threads/altimeters-vs-gps.185979/>
- [N1] R. Nakka, "Altimeter Correction to Account for Launch Site Temperature" (updated 2026-08-26), Figs. 1–3.
  <https://www.nakka-rocketry.net/apogee.html>
- [N2] NAR, 2017 draft rule book, rule 14.2 (the current one was unreachable), web.archive.org 20230608011746 of
  <https://www.nar.org/wp-content/uploads/2016/08/2017-Draft-Rule-Book-Final.html>).
- [P1] B. Cawley Jr., "Altimeter Venting", Peak of Flight 543 (2021), pp. 3–5, 10.
  <https://www.apogeerockets.com/education/downloads/Newsletter543.pdf>
- [P2] H. Larson, vacuum chamber, Peak of Flight 628 (2024), p. 8.
  <https://www.apogeerockets.com/education/downloads/Newsletter628.pdf>
- [PF1] PerfectFlite, StratoLoggerCF manual, pp. 4, 21.
  <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
- [PF2] PerfectFlite, Pnut manual, pp. 9, 23. <http://www.perfectflite.com/Downloads/Pnut%20manual.pdf>
- [MW] Missile Works, RRC3+/RRC3 manual rev. 1.71 (2024), pp. 2, 5, 23–24. <https://www.missileworks.com/downloads/>
- [F1] Featherweight, Raven manual (2014-05-20), p. 12.
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/raven_users_manual_2014may20.pdf>
- [F2] Featherweight, Blue Raven manual (2026-05-12), pp. 7, 11–12.
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2026may12.pdf>
- [A1] Altus Metrum manual v1.9.22.2 (2025), pp. 45, 63, 73, 84. <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [E1] Eggtimer Quantum guide 1.09G, pp. 7, 30.
  <https://eggtimerrocketry.com/wp-content/uploads/2024/02/Eggtimer-Quantum-1_09G.pdf>
- [J1] Jolly Logic, AltimeterTwo user guide 64G.
  <https://jollylogic.com/wp-content/uploads/2022/09/JollyLogicAltimeterTwo-Gen64G-SmallWeb.pdf>
- [EN] Entacore, AIM XTRA/BASE manual 4.x.x (2025), p. 7.
  <https://downloads.entacore.com/aimxtra/AIM%20XTRA%20&%20BASE%20Manual%204.x.x%20v7.pdf>
- [S1] TE, MS5607-02BA03 datasheet, p. 4. <https://pages.hmc.edu/spjut/AdvRoc/PDF/MS5607-02BA03DataSheet.pdf>
