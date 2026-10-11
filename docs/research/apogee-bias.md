# Where the high apogee bias on real flights comes from

On the 55 flights the private collection's report scored when this note was written, HPR Sim's apogee was
+9.83% above the logs on average (mean absolute 13.96%), OpenRocket's +9.00%, and the two agreed within 5% on 53
([ADR-184](../decisions/0184-logged-apogees-of-the-private-collection.md)); the note's arithmetic keeps those.
Rechecked ([ADR-238](../decisions/0238-the-corpus-rechecked-against-the-catalog.md)), the [report](../../validation/reports/fixture-flights.md) scores 53: +8.73% and +7.79%. This note sizes the
published causes that make a simulated apogee run high, each with its sign. **In short: no single
published cause explains it.** What is bounded of the motor is worth about +0.3%, the barometer a
few meters either way, ERA5's temperature perhaps under 1%; the rail is already flown at its angle.
That leaves drag the Barrowman-family codes miss (fin–body interference alone may be 3–7% of
apogee) and as-built mass, with no published size. Without the 19 flights flagged for early
deployment the bias fell to +5.88% (+3.95% since the recheck). Sizes come mostly from single rockets or small samples:
orders of magnitude, not a budget. "Sim high" means a cause makes the simulated apogee read high.

## What the collection itself says (aggregates only)

- **The early deployments carry most of the bias.** Without the 19 flights where a charge fired or
  the rocket separated before the peak, the other 36 are +5.88% (HPR Sim) and +5.29% (OpenRocket)
  ([report](../../validation/reports/fixture-flights.md)). So the 19 average about +17.3% for HPR
  Sim (this note's arithmetic: (55 × 9.83 − 36 × 5.88)/19). ADR-184's readers put each charge
  under 1.5 s before the peak, where the climb left is a few meters; if so, the groups differ in
  their rockets, not the charges' cost, as [accuracy](../accuracy.md) also says. Which it is is open.
- **The rail is already flown** at each flight's recorded angle and heading (`rail` in
  `xtask/src/fixture_flights.rs`). Of the 74 manifest flights not excluded, 72 have an angle; the 11
  whose altimeter measured the rod's tilt have a median of 1.5° and none over 7°. A vacuum shot loses
  1 − cos²θ, 1.5% at 7° (**Inference**): an unmodeled tilt can't explain several percent here.
- **The weather is already corrected.** Every scored log is barometric, and each reading is turned
  into a height through the day's ERA5 air (factor 1.0361 on average, 0.9601 to 1.0777).
- **The public flights show no bias.** On RocketPy's seven team flights HPR Sim's mean signed error
  is −0.25% ([accuracy](../accuracy.md#real-flights)). **Inference:** the bias lies more in how the
  collection's rockets differ from their files than in physics HPR Sim leaves out everywhere.

## How much apogee a cause buys

| change | apogee | flight | source |
|---|---|---|---|
| 1% of total impulse | 0.6% to 1.3% (vacuum bound 2%) | toy vertical H to L flights | **Inference** |
| 1% of total impulse | about 0.9% | the project's Monte Carlo rocket, 1,113 m | [A11] |
| 1% of dry mass | −0.32% | the project's Monte Carlo rocket | [A11] |
| 1% of drag | −0.51% | same | [A11] |
| 1% of dry mass | −0.90% | Calisto, 3,181 m | [B7], this note's arithmetic |
| 1% of drag area | −0.31% | same | [B7], this note's arithmetic |
| 1% of C_D | −0.57% | Project R, Mach 2.7 | [B9] p. 41, Table 4 |

**Inference** (from the rows): a +9.8% bias needs, on its own, impulse 8–15% short, 11–31% more
mass, or 17–32% more drag. +5.9% on the clean 36 needs about 60% of each.

## 1. The motor: about +0.3% from what is bounded, partly cancelled

- **Simulators fly the certification mean, not the label.** NFPA 1125's 2021 revision text tests
  high-power motors "at or corrected to sea level and 20°C ± 5°C", holds the impulse's σ within 6.7%
  of the mean, and says "Imprinted total impulse shall be the mean measured value" [A1, FR-8, p. 7].
- **The catalog matches the test mean.** Of 138 NAR data sheets parsed, ThrustCurve's impulse sits
  within 0.5% of the static-test mean on 127 (median 0.00%) [A5, A6]. The printed label runs higher,
  median +0.99% and up to +14.7% on small black-powder motors (+0.47% median on 28 high-power ones).
  Nobody flies the label. TRA letters agree: catalog impulse equals the measured one for the K480W,
  M1315W and L1500T, and the N3300R is 0.35% off [A7].
- **Curve files carry slightly more impulse than the catalog.** Over 1,252 high-power files: median
  +0.01%, mean +0.32% (p10 −1.4%, p90 +1.9%) [A5]: sim high, about +0.3% of apogee. Firing scatter
  (σ/mean median 0.63%, largest 2.9% [A6]) averages to about ±0.1% over 55 flights (**Inference**).
- **Grain temperature changes the curve's shape, not its impulse.** On a measured AP/HTPB propellant
  [A9] (Table 2, p. 600), burn time fell 1.687 → 1.556 → 1.422 s from 233 to 288 to 323 K. The
  integrated chamber pressure stayed within 3.1% over all 90 K with no consistent sign. **Inference:**
  a ±15 °C day moves apogee about ±0.4%, either way. Hobby propellants were not measured.
- **Thrust at altitude runs the other way.** Certification thrust is taken near sea-level pressure.
  In flight it gains (p₀ − p)·A_e, which HPR Sim leaves out without an exit size
  ([motor model](../physics/motor.md)). For an L1500T (35.6 mm exit, 1,473 N average [A7]),
  **Inference:** about +1% impulse at a 1,300 m site, 0.6–1.3% of apogee; 0.3–1.5% across motors
  and sites.
- **Unbounded:** production lots against the certification mean (sim high if short; no published
  statistic; one home-tested Estes A10-PT read 20% below its label but about 6% above the catalog
  [A8]) and propellant aging (a lower burn rate, no impulse figure, sign unknown [A10]).

## 2. The vehicle: drag the codes miss, and as-built mass

**Published comparisons nearly all run high** (simulated minus flight):

| code | flights | signed | source |
|---|---|---|---|
| OpenRocket, weighed mass, tower launch | 2 | +16%, +7% | [B1] p. 85 |
| RockSim 8, same rocket | 2 | +24%, +19% | [B1] p. 85 |
| OpenRocket and RockSim, a hybrid, mass overridden | 1 | −16%, −12% | [B1] p. 87 |
| OpenRocket, 66 mm model | 8 | mean +20.5%, +12.2% to +30.7% | [B3] Table 1, ours |
| RockSim, Estes D12 student model | 27 launches | +29% (126 m, 98 m mean) | [B8] p. 30, ours |
| RASAero II, the maker's selection | 36 | mean +3.47%, mean absolute 6.24% (ours) | [B4] |

"Ours" is this note's arithmetic. [B3]'s stated mean error (19.9 m) doesn't match its columns (43.6 m).
[B3] and [B8] blame the motor in part; [B4]'s 36 (25 high) include post-flight runs.

**Fin–body interference drag (left out by both codes; sim high; 3–7% of apogee).** Barrowman leaves
it out and HPR Sim follows ([aero](../physics/aero.md)); Niskanen leaves it out as small [B1] p. 41.
Gregorek puts it at "as much as 10% above the sum of the fin and body tube drag" [B2] (PDF p. 13).
In his worked model it is 0.040 of a C_D of 0.339 (12%, all laminar) and 0.052 of 0.538 (10%, all
turbulent) [B2] (PDF pp. 30–31). **Inference** (× −0.31 to −0.57): 3% to 7% of apogee, if HPR
fins with fillets carry that much.

**Launch lugs and rail buttons (sim high where a code underrates them).** A 1/8 × 2 in lug is 6–9% of
C_D in Gregorek's model [B2] (PDF pp. 29–31) and +29% on DeMar's BT-20 in a tunnel [B5] p. 8.
Niskanen's flight test lost 2–3% of apogee to a lug against 1.3% in OpenRocket, about half [B1] p. 85.
Van Milligan's 10× CFD gives one 1010 rail button 0.931 lbf against 0.180 lbf for a 1/4 × 3 in lug;
a standoff nearly doubles the button [B6] p. 26. **Inference** (area-scaled, C_D 0.45 assumed): two
buttons are about 2% of a 98 mm rocket's drag and 7% of a 54 mm one's; neither code's is measured.

**Surface finish (either sign).** DeMar measured unfinished models +13% and polished ones −10% in C_D
against painted ones at 120 km/h [B5] p. 7, but at 80 km/h the polished one reads above the painted
one. In the roughness-limited friction C_f = 0.032 (R_s/L)^0.2 [B1] eq. 3.80, one step from 20 to
60 µm raises friction 25% (**Inference**, from the exponent). A file whose finish is smoother than
the real paint reads high. A part with no stated finish gets 20 µm in HPR Sim and 60 µm in
OpenRocket ([aero](../physics/aero.md), issue #216), so the codes' near-agreement doesn't test this.
**Boundary layer (sim low):** both codes take fully turbulent flow, the high-drag side; partly laminar
flow changed apogee under 5% in Niskanen's runs [B1] p. 43.

**Coning and angle of attack (sim high; unbounded).** Neither code models fin or thrust misalignment.
Project R's DATCOM predictions ran 27.8% and 15.2% high; a coning model alone (about 3.5° at high
dynamic pressure, more near apogee) cut the first to +11.8% [B9] pp. 41, 45. **Inference:** a mean
angle α adds about C_Nα·α² to C_D: for C_Nα of 8–15 per radian, +0.5–1% of a 0.45 C_D at 1°,
+2–4% at 2°, +5–9% at 3°. No hobby measurement was found.

**As-built mass (sim high; unbounded).** No published survey says how much heavier finished rockets
fly than their files. The OpenRocket wiki tells users to match the weighed rocket, naming glue, paint
and decals [B10]. Niskanen weighed every part and the whole rocket, and his simulation still ran 7%
and 16% high [B1] pp. 83, 85: there it was not mass. **Inference:** a whole-rocket override from a
scale closes this cause for a flight; a file built from parts' catalog masses does not.

## 3. The measurement and the air: a few meters either way, but ERA5's temperature

- **Sensor offsets (either sign).** After zeroing at the pad, an MS5607 is good to ±0.5 mbar at
  25 °C and ±1.0 mbar over 0–50 °C [C1] p. 4. That is ±4–8 m (**Inference**, 8.3 m/hPa near sea
  level). A BMP390's relative accuracy is ±3 Pa, under ±1 m [C2] p. 8. They widen the error, not the mean.
- **Altitude formula (covered).** The StratoLogger and Raven read the standard atmosphere's altitude
  less the pad's [C5] PDF p. 4, [C3] PDF p. 12, as the comparison inverts it.
- **Port lag, filtering, sampling (sim high; under 1 m).** A lag τ lowers a parabola's peak by
  g·τ²/2. **Inference:** a port sized by PerfectFlite's rule [C5] PDF p. 11 lags a few milliseconds,
  and a 1 s centered average costs 0.41 m. Featherweight warns that small vents on a large bay lag
  [C6] PDF p. 7; a bay vented only through leaks might lag seconds (**Inference**), unsized.
- **ERA5's temperature (likely sim high; about +0.35% per kelvin).** Height from a pressure drop scales
  with the layer's mean temperature, 1/288 per kelvin (**Inference**, hypsometric equation). ERA5's
  summer daily range is smaller than observed at 80% of stations, particularly in western North
  America [C8] p. 15, Fig. 9b. If that means a too-cool afternoon (**Inference**; warm nights would
  do it too), the true heights come out too low. No source gives the 0–3 km layer-mean error;
  2 K would be +0.7%.
- **ERA5's wind (likely sim high in the west).** ERA5 underestimates winds above 4 m/s in the western
  US [C8] p. 15, so the simulations weathercock too little there. On two loads of one rocket, apogee
  moved 0.1–2% per mph of wind [C11] pp. 35, 39 (weak). Gusts' extra cost is unbounded.

## What this suggests

1. **Split the early deployments:** do the 19 flagged flights' rockets, motors or readings differ?
2. **The collection's own elasticities:** rerun the 55 with dry mass +5% and with C_D × 1.1.
3. **Split the bias by a whole-rocket mass override** (aggregates only): if it survives, mass is out.
4. **An opt-in fin–body interference probe** after Gregorek's rule [B2] (PDF p. 29, eq. 21): expected
   3–7% lower apogees. Validate it on wind-tunnel or public flight data, never tuned to these logs.
5. **A nozzle exit diameter per catalog motor** (TMT letters) for pressure thrust: right, though it
   moves apogees 0.3–1.5% the other way.

## Open questions

- Production-lot impulse against the certification mean; hobby propellants' temperature sensitivity;
  whether OpenRocket corrects thrust for ambient pressure (not checked: its source is GPL).
- Hoerner's fin-junction drag for thin, filleted HPR fins; a flight-measured HPR coast C_D against
  OpenRocket's; the button drag of either code against a measurement.
- ERA5's 0–3 km layer-mean temperature error on summer afternoons, and its wind 2–5 m above a pad.
- The Ironbark comparison (OpenRocket reportedly +7.5% and +8.2% on one rocket) was not reachable.

## Sources

All read 2026-10-11; copies sit under the gitignored `refs/research/apogee-bias/`. Terms: B1 CC BY-NC-ND,
B3 CC BY 4.0, A8 and C8 arXiv's license; the rest all rights reserved or unstated. Cite, never copy.

- **A1.** NFPA 1125 (A2021) First Revision Report, FR-7, FR-8.
  <https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf>
- **A5.** ThrustCurve.org catalog and curve files, the project's cache of 2026-09-17. <https://www.thrustcurve.org/>
- **A6.** NAR S&T motor data sheets: 157 from ThrustCurve's links via the Internet Archive, 138 parsed.
- **A7.** TRA letters (K480W, L1500T, M1315W, N3300R) on ThrustCurve, e.g.
  <https://www.thrustcurve.org/motors/cert/6178f239caf91100040db736/l1500t-p_tra_cert.pdf>
- **A8.** Courtney and Courtney, arXiv:0903.1555, pp. 1–2. <https://arxiv.org/abs/0903.1555>
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
