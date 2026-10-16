# Where the high apogee bias on real flights comes from

Re-checked 2026-10-16: 37 claims corrected.

On the 53 flights the private collection's [report](../../validation/reports/fixture-flights.md) scores (rechecked in
[ADR-238](../decisions/0238-the-corpus-rechecked-against-the-catalog.md)), HPR Sim's apogee is +8.73% above the logs on
average (mean absolute 12.95%), OpenRocket's +7.79% (first written at 55 flights, +9.83% and +9.00%:
[ADR-184](../decisions/0184-logged-apogees-of-the-private-collection.md)). This note sizes the published causes that
make a simulated apogee run high, each with its sign. **In short: no single published cause explains it.** Of the motor,
curve files add about +0.3%; production lots are likely short, 0–5% of apogee, unmeasured for today's motors; thrust
gained at altitude, which HPR Sim leaves out, pushes the other way, 0.4–3.8% at a 1,300 m field against a sea-level
curve. Two altimeters on one rocket agree to a median 0.37%; ERA5's temperature is worth perhaps under 1%; both either
way. The rail is already flown at its angle. That leaves as-built mass (a public median of +7.4% of dry mass, 2.4–6.7%
of apogee for a file with no weighed override) and drag the codes miss: rail buttons 0.6–4%; fin–body interference, once
put at 3–7%, measures small or favorable on thin fins. The 19 early-deployment flights read high for which rockets they
are, not their charges ([early-deployment-bias](early-deployment-bias.md)). Sizes come mostly from single rockets or
small samples: orders of magnitude, not a budget. "Sim high": a cause makes the simulation read high.

## What the collection itself says (aggregates only)

- **The early deployments are a source split, not a charge cost.** The 19 flights where a charge fired or the rocket
  separated before the peak read +17.29% (HPR Sim) and +16.03% (OpenRocket); the other 34 read +3.95% and +3.19%
  ([report](../../validation/reports/fixture-flights.md)). The charges cost under 0.3% of apogee on 18 and under about
  1% on the one fired 2.6 s before its fitted peak; 15 of the 19 come from 2 of 16 sources
  ([early-deployment-bias](early-deployment-bias.md)). Each source counted once, the 53 read +4.40% and +3.70%.
- **The rail is already flown** at each flight's recorded angle and heading (`rail` in `xtask/src/fixture_flights.rs`).
  Of the 72 manifest flights not excluded, 70 have an angle; the 10 whose altimeter measured the rod's tilt have a
  median of 1.4°, none over 7°. A vacuum shot loses 1 − cos²θ, 1.5% at 7° (**Inference**): not several percent.
- **The weather is already corrected.** Every scored log is barometric, and each reading is turned into a height through
  the day's ERA5 air (factor 1.0360 on average, 0.9601 to 1.0777).
- **The public flights show no bias.** On RocketPy's seven team flights HPR Sim's mean signed error is −0.25%
  ([accuracy](../accuracy.md#real-flights)). **Inference:** the bias lies more in how the collection's rockets differ
  from their files than in physics HPR Sim leaves out everywhere.

## How much apogee a cause buys

| for 1% more | project's Monte Carlo rocket, 1,113 m [A11] | Calisto, 3,181 m [B7] | Project R, Mach 2.7 [B9] |
|---|---|---|---|
| total impulse | about 0.9% | 1.46% | 2.3% |
| dry mass | −0.32% | −0.90% | |
| drag | −0.51% | −0.31% (drag area) | −0.57% (C_D) |

Calisto's are this note's arithmetic from [B7]'s regression; Project R's from [B9] p. 41, Table 4. Toy vertical H to L
flights with drag give 0.6–1.3% per 1% of impulse (**Inference**). The impulse figure grows with the flight: an instant
burn in vacuum gives 2, a real burn against gravity more, drag less. **Inference** (from the rows): +8.73% needs, alone,
impulse 4–15% short, 10–27% more mass, or 15–28% more drag; the 34 flights' +3.95%, about 45% of each.

## 1. The motor: three terms, two of them opposed

- **Simulators fly the certification mean, not the label.** NFPA 1125's 2021 revision text tests high-power motors "at
  or corrected to sea level and 20°C ± 5°C", holds the impulse's σ within 6.7% of the mean, and says "Imprinted total
  impulse shall be the mean measured value" [A1, FR-8, p. 7].
- **The catalog matches the test mean.** Of 140 NAR data sheets parsed, ThrustCurve's impulse sits within 0.5% of the
  static-test mean on 129 (median 0.00%) [A5, A6]. The printed label runs higher, median +1.01% and up to +14.7% on
  small black-powder motors (+0.47% median on 28 high-power ones). TRA letters agree: catalog impulse equals the
  measured one for the K480W, M1315W and L1500T, and the N3300R is 0.36% off [A7].
- **Curve files carry slightly more impulse than the catalog.** Over 1,252 high-power files: median +0.01%, mean +0.32%
  (p10 −1.4%, p90 +1.9%) [A5]: sim high, about +0.3% of apogee. Firing scatter (σ/mean median 0.63%, largest 2.9%, on
  the 28 high-power sheets [A6]) averages to about ±0.1% over 53 flights (**Inference**).
- **Production lots: likely short, size unmeasured (sim high).** NAR's 2023–26 retests of 17 motors, F and smaller, ran
  a median 9.4% below their certification ([motor data](motor-data.md) §5). For high power, TMT's 2001–04 repeat tests
  of 22 AeroTech H–M motors moved a median −2.7% (−8.5% to +9.5%), eleven independent firings −6.8%. 0 to −4% fits all
  of it, −9% is not ruled out; at the toy's 0.6–1.3%, 0–5% of apogee ([HPR lot impulse](hpr-lot-impulse.md) §2, §4–5).
- **Grain temperature changes the curve's shape, not its impulse.** On an AP/HTPB propellant [A9] (Table 2, p. 600),
  burn time fell 1.687 → 1.556 → 1.422 s from 233 to 288 to 323 K; the integrated chamber pressure stayed within 3.1%,
  no consistent sign. **Inference:** a ±15 °C day moves apogee about ±0.4%, either way. Hobby propellants: unmeasured.
- **Thrust at altitude runs the other way (sim low).** In flight a motor gains (p_test − p)·A_e over its test stand,
  which HPR Sim leaves out without an exit size ([motor model](../physics/motor.md)). TMT fired in North Carolina
  (99.6 kPa) in 2017–24 and near Denver (85.0 kPa) from 2025; earlier sites weren't found, and no public TMT text says
  it corrects to sea level ([TMT test sites](tmt-test-sites.md)). Today's L1500T nozzle has a 1.75 in (44.5 mm) exit
  ([nozzle exits](nozzle-exits.md) §2; the 1998 letter's 1.4 in [A7] is likely the older nozzle). **Inference:** against
  sea level, 22.8 N at a 1,300 m site, +1.55% of its 1,473 N average [A7], 0.9–2.0% of apogee. Over 16 AeroTech reloads
  the drawn exits add 0.63–2.92% of thrust there, +0.4% to +3.8% of apogee (nozzle exits §3), less near Denver.

## 2. The vehicle: drag the codes miss, and as-built mass

**Published comparisons nearly all run high** (simulated minus flight):

| code | flights | signed | source |
|---|---|---|---|
| OpenRocket; RockSim 8 (weighed mass, tower launch) | 2 | +16%, +7%; +24%, +19% | [B1] p. 85 |
| OpenRocket and RockSim, a hybrid, mass overridden | 1 | −16%, −12% | [B1] p. 87 |
| OpenRocket, 66 mm model | 8 | mean +20.5%, +12.2% to +30.7% | [B3] Table 1, ours |
| RockSim, Estes D12 student model | 27 launches | +29% (126 m, 98 m mean) | [B8] p. 30, ours |
| RASAero II, the maker's selection | 36 | mean +3.47%, mean absolute 6.24% (ours) | [B4] |
| OpenRocket, one supersonic rocket (Bluewren) | 2 | +7.49% (+8.22% on a second recorder), +4.49% | [B11] |

"Ours" is this note's arithmetic. [B3]'s stated mean error (19.9 m) doesn't match its columns (43.6 m). [B3], [B8] blame
the motor in part; [B4]'s 36 (25 high) include post-flight runs. [B11]: Table 3, p. 9, the 8.22% in its text, p. 8; one
supersonic rocket, inputs unstated, no cause named ([Ironbark](ironbark.md) §1).

**Fin–body interference drag (left out by both codes; small on thin fins, sign unsettled).** Barrowman, HPR Sim
([aero](../physics/aero.md)) and Niskanen [B1] p. 41 leave it out. Gregorek puts it at "as much as 10% above the sum of
the fin and body tube drag" [B2] (PDF p. 13; 10–12% in his worked model, PDF pp. 30–31), 3–7% of apogee through the
table, but cites no source. Hoerner's correlation extrapolates to −0.5% to −5.7% of C_D on fins 1–4% thick (raising
apogee); NASA's Arcas models leave about 6% of forebody drag at Mach 0.6 ([junction drag](junction-drag.md) §1–4). No
source measured fillets.

**Launch lugs and rail buttons (sim high where a code underrates them).** A 1/8 × 2 in lug is 6–9% of C_D in Gregorek's
model [B2] (PDF pp. 29–31), +29% on DeMar's BT-20 in a tunnel [B5] p. 8, +16.3% (1/8 × 1 in) on Van Milligan's 10× CFD
model [B6] pp. 3–4. Niskanen's flight test lost 2–3% of apogee to a lug, OpenRocket 1.3% [B1] p. 85. In the CFD one 1010
rail button draws 0.931 lbf against 0.180 lbf for a 1/4 × 3 in lug, a standoff nearly doubles it, and installed, the lug
added 30 times its isolated drag [B6] p. 26; the author calls the results unvalidated [B6] p. 34. **Inference**
(area-scaled, C_D 0.45): two buttons are about 2% of a 98 mm rocket's drag, 7% of a 54 mm one's, 0.6–4% of apogee.

**Surface finish (likely sim low with the defaults).** DeMar measured unfinished models +13% and polished ones −10% in
C_D against painted ones at 120 km/h; at 80 km/h the polished one read above the painted one [B5] p. 7. In C_f = 0.032
(R_s/L)^0.2 [B1] eq. 3.80, 20 to 60 µm raises friction 25% (**Inference**). An unstated finish gets 20 µm in HPR Sim,
60 µm in OpenRocket ([aero](../physics/aero.md), #216), but painted and sanded finishes' measured roughness converts to
about 0.1–14 µm, so finish more likely hides part of the bias ([hobby finish roughness](hobby-finish-roughness.md)).
**Boundary layer (sim low):** both codes take fully turbulent flow; partly laminar flow changed apogee under 5% in
Niskanen's runs [B1] p. 43.

**Coning and angle of attack (sim high; unbounded).** Neither code models fin or thrust misalignment. Project R's DATCOM
predictions ran 27.8% and 15.2% high; a coning model alone (about 3.5° at high dynamic pressure, more near apogee) cut
the first to +11.8% [B9] pp. 41, 45. **Inference:** a mean angle α adds about C_Nα·α² to C_D: for C_Nα of 8–15 per
radian, +0.5–1% of a 0.45 C_D at 1°, +2–4% at 2°, +5–9% at 3°. No hobby measurement was found.

**As-built mass (sim high).** In 24 public pairs of a file mass and a scale mass, finished rockets came out a median
+7.4% heavier (interquartile range −1.4% to +13.1%): 2.4–6.7% of apogee through the table, for a file with no weighed
override ([as-built mass growth](as-built-mass-growth.md)). The OpenRocket wiki says to match the weighed rocket [B10].
But Niskanen weighed every part and the whole rocket and still ran 7% and 16% high [B1] pp. 83, 85, and five of six
Student Launch runs with weighed mass read 3.6–18.4% high (as-built mass growth §4): one cause among several.

## 3. The measurement and the air: tenths of a percent, but ERA5's temperature

- **Sensor offsets (either sign).** After zeroing at the pad, an MS5607 is good to ±0.5 mbar at 25 °C and ±1.0 mbar over
  0–50 °C [C1] p. 4, ±4–8 m (**Inference**); a BMP390 ±3 Pa relative, under ±1 m [C2] p. 7. On 59 published flights two
  altimeters differed by a median 0.37% (4.0 m), 11 by over 2% ([dual-altimeter spread](dual-altimeter-spread.md)).
- **Altitude formula (covered).** StratoLogger, Raven: ISA altitude less the pad's [C5] PDF p. 4, [C3] PDF p. 12.
- **Port lag, filtering, sampling (sim high).** A lag τ lowers a parabola's peak by g·τ²/2. **Inference:** a port sized
  by PerfectFlite's rule [C5] PDF p. 11 lags milliseconds; a 1 s centered average costs 0.41 m. Featherweight warns that
  small vents on a large bay lag [C6] PDF p. 7; in one model-rocket test a sealed bay lagged about 2.5 s and read about
  19% low, a leak-vented one didn't (dual-altimeter spread §4). No HPR bay was measured.
- **ERA5's temperature (either sign; about 0.35% per kelvin).** Height scales with the layer's mean temperature, 1/288
  per kelvin (**Inference**, hypsometric equation). ERA5's summer daily range is too small at 80% of stations,
  particularly in western North America and southern Europe; its spring-mean 2 m temperature runs warm over most of the
  central and eastern US, cool along the Rockies and Sierra Nevada [C8] p. 15, Figs. 9b, 10c. No 0–3 km figure; 2 K is
  0.7%.
- **ERA5's wind (sign by region).** ERA5 underestimates winds above 4 m/s in the western US (too little weathercocking,
  sim high) and overestimates wind over much of the eastern US, often by over 2 m/s in the northeast (sim low) [C8]
  p. 15. Around cyclones its 10 m wind is right on average (−0.7%), 10–17% low in the strongest [C13] pp. 737–738, 740.
  On two loads of one rocket, apogee moved 0.1–2% per mph [C11] PDF pp. 3, 7 (weak). Gusts' cost is unbounded.

## What this suggests

1. **Report a source-weighted mean** beside the flight mean ([early-deployment-bias](early-deployment-bias.md)).
2. **The collection's own elasticities:** rerun the 53 with dry mass +5%, C_D × 1.1 and impulse −3%.
3. **Split the bias by a whole-rocket mass override** (aggregates only): if it survives, mass is not the cause.
4. **No fin–body interference term by default** on thin fins ([junction drag](junction-drag.md), its suggestions).
5. **A nozzle exit per motor** for pressure thrust: RCS drawings give one for 213 of 215 AeroTech reloads at 38–98 mm
   ([nozzle exits](nozzle-exits.md) §1), certifiers for 2 of 1,004 motors ([motor data](motor-data.md) §2). Apogees
   would rise up to 0.4–3.8%, by how much depending on where the curve was fired ([TMT test sites](tmt-test-sites.md)).

## Open questions

- Today's high-power lots against the certification mean; hobby propellants' temperature sensitivity; aging (a lower
  burn rate, impulse unknown [A10]); whether OpenRocket corrects for ambient pressure (unchecked: its source is GPL).
- Fin–body interference on filleted, painted HPR fins at flight Reynolds numbers; a flight-measured HPR coast C_D
  against OpenRocket's; the button drag of either code against a measurement.
- ERA5's 0–3 km layer-mean temperature error on summer afternoons, and its wind 2–5 m above a pad.
- The collection's own impulse elasticity: the toy's 0.6–1.3%, which later notes reuse, is low at 3 km (1.46%).

## Sources

Read 2026-10-11, re-read 2026-10-16; copies in the gitignored `refs/research/` (`apogee-bias/`, `blocked-sources/`,
`ironbark/`). Terms: B1 CC BY-NC-ND; B3, B11 CC BY 4.0; C13 CC BY-NC 4.0; C8 arXiv's; the rest reserved or unstated.

- **A1.** NFPA 1125 (A2021) First Revision Report, FR-7, FR-8.
  <https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf>
- **A5.** ThrustCurve.org catalog and curve files, the project's cache of 2026-09-17. <https://www.thrustcurve.org/>
- **A6.** NAR S&T motor data sheets: all 159 of ThrustCurve's links via the Internet Archive, 140 parsed.
- **A7.** TRA letters (K480W, L1500T, M1315W, N3300R) on ThrustCurve, e.g.
  <https://www.thrustcurve.org/motors/cert/6178f239caf91100040db736/l1500t-p_tra_cert.pdf>
- **A9.** Florczak et al., Cent. Eur. J. Energ. Mater. 11(4), 589–601 (2014).
  <https://wydawnictwa.ipo.lukasiewicz.gov.pl/wp-content/uploads/2021/04/Florczaki-inni.pdf>
- **A10.** IIT Madras seminar abstract on propellant aging, 2022-01-21. <https://www.iitm.ac.in/hi/node/10746>
- **A11.** The project's [sensitivity](../sensitivity.md) and [Monte Carlo](../monte-carlo.md) pages.
- **B1.** Niskanen, OpenRocket MSc thesis, TKK, 2009, printed pages. <https://openrocket.sourceforge.net/thesis.pdf>
- **B2.** Gregorek, Estes TR-11, 1970, PDF pages. <https://www.plans.rocketshoppe.com/pubs/Estes/estTR-11/TR-11.pdf>
- **B3.** Pandey and Pandey, arXiv:2512.22248v1. <https://arxiv.org/abs/2512.22248>
- **B4.** RASAero II comparisons with altitude data. <https://www.rasaero.com/comparisons-alt.htm>
- **B5.** DeMar, NAR R&D, 1995. <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
- **B6.** Van Milligan, NARCON 2017. <https://www.apogeerockets.com/downloads/PDFs/launch-lug-drag.pdf>
- **B7.** RocketPy, Sensitivity Analysis. <https://docs.rocketpy.org/en/latest/user/sensitivity.html>
- **B8.** Sarper and Landman, ASEE 2016, paper 15726. <https://peer.asee.org/26910.pdf>
- **B9.** Rogers, Project R postflight, 1997. <https://www.rasaero.com/dloads/OuR%20Project%20R%20Rocket.pdf>
- **B10.** OpenRocket wiki. <http://wiki.openrocket.info/Overrides_and_Surface_Finish>
- **B11.** Galletly et al., Aerosp. Sci. Technol. 173, 111758 (2026). <https://doi.org/10.1016/j.ast.2026.111758>
- **C1.** MS5607-02BA03 datasheet. <https://pages.hmc.edu/spjut/AdvRoc/PDF/MS5607-02BA03DataSheet.pdf>
- **C2.** BMP390, rev. 1.7.
  <https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp390-ds002.pdf>
- **C3.** Raven manual, 2014.
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/raven_users_manual_2014may20.pdf>
- **C5.** StratoLoggerCF manual. <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
- **C6.** Blue Raven guide, 2025.
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2025_sept_10.pdf>
- **C8.** Jin et al., WeatherReal, arXiv:2409.09371v1. <https://arxiv.org/abs/2409.09371>
- **C11.** Caballes et al., SISE 2021. <https://www.ieworldconference.org/content/SISE2021/Papers/Caballes2.pdf>
- **C13.** Chen, Collet and Di Luca, Int. J. Climatol. 44(3), 729–747 (2024). <https://doi.org/10.1002/joc.8339>
