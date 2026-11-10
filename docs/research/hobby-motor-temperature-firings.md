# Hobby composite motors fired at two grain temperatures

Has anyone outside the military and sounding-rocket world fired one composite motor type at two or more stated
grain temperatures, and what does it give per 10 K for burn time, peak thrust and impulse? This note checks the
coefficients that [propellant temperature](propellant-temperature.md) borrows (π_K 0.15–0.32 %/K, impulse +0.1 to
+0.8% per 10 K), the grain-temperature share in [HPR burn-time scatter](hpr-burn-time-scatter.md) §4, the burn-time
row of [dispersion sigmas](dispersion-sigmas.md), and the composite-delay gap in
[NAR delay reports](nar-delay-reports.md). Read 2026-11-04: forum threads and their attachments, a maker-posted
sheet, a sounding-rocket paper, NAR's report index and data sheets, ThrustCurve's files, student and club reports;
no source code. **Inference** marks this note's arithmetic. Trust it as one hobby-class set and one hobby-adjacent
sheet, both read from charts by pixel or by eye; nothing here is a hobby maker's published coefficient.

**In short.**
- **Only one hobby set was found, and it has no peak thrust.** A 2008 science-fair test fired 71 F-class composite
  motors (24 at −18 °C, 23 at room temperature, 24 at 49 °C). Per 10 K warmer: burn time −2.2%, mean thrust +2.4%,
  impulse +0.2% (+0.1 to +0.3 by leg), delay −1.5% (**Inference**, §2). As π_K that is 0.22 %/K, mid-range of
  the borrowed 0.15–0.32.
- **A maker-posted HTPB/AP sheet agrees.** The RDS-700, a 711 N·s extraction motor with curves at −54, 21 and 74 °C,
  gives burn time −2.1% per 10 K on the cold leg and −2.3% on the warm leg, peak thrust +2.4% and +3.4%
  (**Inference**, read by pixel, §3). Overall, 0.22 %/K again.
- **The slope is not the same on both sides of room temperature.** The F set, the RDS-700's peak and Florczak's peak
  pressure move faster on the warm side; Corrêa's three propellants move faster on the cold side, by 1.07–1.53
  times. One π_K for −20 to +45 °C is good to about ±25% of the change, about 2.3% of burn time at −20 °C
  (**Inference**, §4).
- **Corrêa gives no cause for its impulse rise.** Its conclusion says only that specific impulse rose with
  temperature more than chamber pressure justifies, and asks for a dedicated study [3, p. 11]. On the RDS-700's own
  numbers the nozzle term is +0.13 to +0.15% per 10 K (**Inference**, §5).
- **The records hold no second temperature.** 20 NAR sheets print a test temperature (17, 22 or 23 °C), and none of
  their 19 designations was tested at two. 1,706 cached ThrustCurve files carry no temperature. NAR's one
  air-temperature report flew black-powder motors on two days and stated no grain temperature (§1, §6).

## 1. What was searched, and what held nothing

| Where | What was looked for | Found |
|---|---|---|
| RocketryForum | threads on cold or hot firings with numbers | one science-fair set; one maker-posted sheet [1, 2] |
| NAR R&D index, 540 ids | titles on temperature and composites | air temperature, cold motors: both BP [4, 10] |
| NAR data sheets, 355 cached texts | a "Test Temp" field; a designation at two values | 20 fields; none at two [5] |
| ThrustCurve files, 1,706 | temperature in RASP or RockSim comments | none [6] |
| Student, club, school reports | one motor type at two conditioning temperatures | none [7, 8] |
| Japanese hobby-motor papers | hobby composite motors against temperature | black-powder A8-3 only [11] |

- **NAR's air-temperature report.** "The Effect of Air Temperature on Model Rocket Maximum Altitude" (Nowak,
  NARAM-50, 2008) is the only temperature title not about black powder or sensors by name [4]. Read 2026-11-04: it
  flew Estes B6-4 black-powder motors, so it is in §6 [10]. The cold-motor report (Gaff, NARAM-26) is black
  powder too ([NAR delay reports](nar-delay-reports.md) §4).
- **Data-sheet fields.** The project's three caches of NAR sheets (157, 68 and 130 texts, overlapping; counted
  2026-11-04) hold 20 sheets with a test temperature: 17 AMW sheets at 17 °C (tested 2004), two F26FJ at 22 °C and one
  AeroTech I285R at 23 °C [5]. No designation repeats at a second temperature.
- **ThrustCurve.** In the 2026-10-28 capture (1,706 files, 924 motors) no comment names a temperature [6].
- **Empty leads.** A balloon team heated a Cesaroni 54 mm motor in flight, no firing data [7]; a TARC poster
  regressed altitude on air, not grain, temperature [8]; an open-source motor tool has no temperature input, and
  its tracker's request cites a coefficient of unstated propellant, its author testing KNSB, not APCP [9].

## 2. The science-fair F set (2008)

A forum member described his son's 2008 science-fair test [1, posts 8, 13]. The motors were F-class composites, maker
and designation not stated. They were stored a week in a freezer at 0 °F (−18 °C) or a hot box at 120 °F (49 °C)
and held there during testing with dry ice or a heating pad. A control group fired at an unstated room temperature.
Post 8 recalls about 40 motors; post 13, from the original files, counts 23 controls and 24 each hot and cold.

Read by pixel from post 13's three full-size bar charts (gridlines calibrate each axis; about ±0.01 s, ±0.1 N·s):

| Group | Burn time | Impulse | Mean thrust (I/t) | Delay |
|---|---:|---:|---:|---:|
| Cold, −18 °C, 24 | 2.44 s | 77.7 N·s | 31.8 N | 11.65 s |
| Control, room, 23 | 2.26 s | 78.7 N·s | 34.8 N | 11.14 s |
| Hot, 49 °C, 24 | 2.11 s | 78.8 N·s | 37.4 N | 10.53 s |

- **Per 10 K over the full 67 K (Inference):** burn time −2.2%, mean thrust +2.4%, impulse +0.2%, delay −1.5%.
  The impulse chart is titled "Total Thrust" in N, but post 13's caption calls it total impulse.
- **By leg (Inference):** with the control at 20 °C, burn time −2.0% (cold leg) and −2.4% (warm leg); at 25 °C,
  −1.8% and −2.9%. The warm leg is steeper if the room was above 17 °C. Impulse rose +0.3% per 10 K on the cold
  leg; the warm leg's +0.07% is within reading error.
- **Is the impulse step real (Inference)?** No spreads were posted. At NAR's in-lot impulse CV of 0.5–1.1%
  ([HPR burn-time scatter](hpr-burn-time-scatter.md) §2), the 1.3% cold deficit is 4–8 standard errors; if the
  motors spanned lots with TMT's 4%, about 1.1. The post does not say whether one lot was used.
- **Delay.** The post does not say whether delay was timed from ignition or burnout, nor its nominal value. Either
  way −1.5% per 10 K sits inside the −4.2% to +2.0% bound that Staab's flights allow for composite delays
  ([NAR delay reports](nar-delay-reports.md)). Post 8 adds that cold motors' delays ran longer.
- **No peak thrust or curves** were posted; post 8's recalled 10% burn-rate change roughly fits the 14.6% span.

## 3. The RDS-700 sheet

AeroTech's forum account posted Industrial Solid Propulsion's RDS-700 sheet as an example of performance against
temperature [2]. The motor extracts a parachute from a remotely piloted aircraft. It is not a hobby reload, but it
is an 82%-solids HTPB/AP center-perforated grain of hobby size: 335 g of propellant, 711 N·s, 1.78 s. The sheet
prints ambient values only and overlays three curves at −65, +70 and +165 °F (−54, 21 and 74 °C).

- **Read by pixel (Inference):** curve ends at about 2.08, 1.78 and 1.58 s, the middle one matching the printed
  1.78 s; peak about 86, 102 (printed 103) and 123 lbf. Per 10 K: burn time −2.1% on the cold leg (75 K), −2.3% on
  the warm leg (53 K); peak +2.4%, +3.4%. Ends taken at 10% of peak give −2.05% and −2.5%.
- **Reading error.** ±0.05 s on the end of burn moves the cold-leg slope ±0.3 points and the warm-leg ±0.6, so the
  burn-time asymmetry is not established; ±2 lbf leaves the peak asymmetry standing.
- **Impulse by temperature** is not printed, and the overlapping curves can't be integrated to 1% by eye. Peak
  rises faster than burn time falls, by 0.3–1.2 points per 10 K, but peak is not mean thrust.

## 4. One slope or two

Per 10 K warmer, by leg below and above room temperature (**Inference** from each source's printed values):

| Set | Quantity | Cold leg | Warm leg | Warm ÷ cold |
|---|---|---:|---:|---:|
| F science fair [1], control 20–25 °C | burn time | −1.8 to −2.0% | −2.4 to −2.9% | 1.2–1.6 |
| RDS-700 [2] | burn time; peak thrust | −2.1%; +2.4% | −2.3%; +3.4% | 1.1–1.2; 1.5 |
| Florczak 2014, 11 mm throat [12, Table 2, p. 600] | peak pressure | +0.24% | +2.1% | about 9 |
| Corrêa 2006, PC1, PC2, PC3 [3, Table 3, p. 10] | π_K | 0.19, 0.32, 0.26 %/K | 0.15, 0.21, 0.25 %/K | 0.65–0.94 |

- Corrêa splits at 296 K (289 K for PC3) and reads it as sensitivity falling when warmer [3, p. 11]; Florczak's
  split is [propellant temperature](propellant-temperature.md) §2's.
- **Inference:** burn-time and π_K leg slopes differ by a factor 0.65–1.6, with no consistent sign (one throat's
  peak pressure, about 9). An exp(π_K ΔT) stretch fitted at the mean slope errs about ±25% of the change at the
  ends of a −20 to +45 °C span: about 2.3% of burn time at −20 °C, 1.4% at +45 °C.

## 5. Why impulse might rise with temperature

Corrêa's 27 firings gave specific impulse +0.31 to +0.80% per 10 K
([propellant temperature](propellant-temperature.md) §3). The paper names no mechanism. It notes the rise exceeds
what chamber pressure explains and calls the effect a possible influence of temperature on specific impulse, to be
confirmed by a dedicated test [3, p. 11]. It computes specific impulse as thrust integral over propellant mass
[3, eq. 17, p. 4]. Candidate causes, with their sizes where known (**Inference**):

- **c\* from sensible heat:** +0.05 to +0.14% per 10 K (SP-8076, in the parent note). Warm higher.
- **Nozzle expansion at higher pressure:** d ln C_F/d ln p_c = ε p_a/(p_c C_F). The RDS-700 prints ε 3.58, mean
  pressure 3,861 kPa, throat 0.710 cm² and mean thrust 400 N, so C_F is 1.46 and the derivative 0.064 [2]. With
  pressure +2.1 to +2.3% per 10 K (its burn-time slopes, at fixed K), impulse gains +0.13 to +0.15%. Warm higher.
- **Unbounded here:** less unburned sliver or slag in a warm motor; a shorter ignition and tail-off share of the
  integral; heat lost to the case. Sign plausibly warm higher for each; no source sizes them.
- The F set's +0.2% (+0.1 to +0.3) per 10 K fits c\* plus nozzle alone; Corrêa's median +0.53% needs a further
  0.2–0.4.

## 6. Black powder, for contrast

Black powder is out of scope; its cold and aging tests are in [NAR delay reports](nar-delay-reports.md).

- **E12 flights:** a flier reports burns 130 ms longer at 40 °C than at 20 °C, the opposite sign to composite,
  inferred from flights rather than a stand [1, post 12].
- **Nowak (NARAM-50, 2008)** flew one streamer model on Estes B6-4 motors, five flights a day, on 2007-11-25 at
  38 °F and 2007-12-08 at 31 °F air; grain temperature is not stated [10, pp. 6, 10–12]. Mean altimeter apogee
  515.6 and 489.2 ft, printed SDs 10.7 and 22.4 (Table 1, p. 12).
- **Inference:** −5.1% for 3.9 K colder air; with n − 1 SDs (11.9, 25.0; the printed ones divide by n) a Welch
  test gives t 2.1 on 5.7 df, two-sided p about 0.08. Colder air alone is about −0.4% (the parent's −0.9 to −1.2%
  per 10 K); the rest is day-to-day scatter or an unmeasured grain effect. It sizes nothing for composites.

## What this suggests

- **π_K:** keep 0.15–0.32 %/K and center the correction idea's coefficient at 0.22 %/K. The only hobby-class firing
  set and the hobby-size HTPB/AP sheet both give 0.22 %/K overall. Expected effect: burn time −2.2% and thrust +2.2%
  per 10 K, against −2.5% at the parent's 0.25.
- **Impulse factor:** keep +0.1 to +0.8% per 10 K, warm higher. The hobby F set sits at +0.2%, the low end; a
  center near +0.3% would sit between it and Corrêa's median.
- **M2.8's burn-time σ:** a grain spread of 5–10 K is 1.1–2.2% of burn time at 0.22 %/K, inside the parent's
  0.8–3%. Stated as from one F set and one extraction motor.
- **Composite delays:** −1.5% per 10 K (−1.0 to −2.4 by leg), cold longer, as the first sized value for
  [NAR delay reports](nar-delay-reports.md)' open item. From one unnamed F motor with an unstated timing reference.
- **Tests could pin the signs:** warm grain gives shorter burn, higher peak, impulse not lower, shorter delay.
- **Data terms.** Forum posts, their charts and NAR reports state no license: cited, not copied, cached under
  `refs/research/` (this topic's and hpr-burn-time-scatter's folders). Corrêa's paper (ABCM proceedings, no
  license stated) is in `refs/papers/`. NAR sheets are NAR's: cited by id.

## Open questions

- A hobby reload fired hot and cold with curves, peak thrust and spreads; the F set's raw files exist but are
  unpublished, and its maker, lot and control temperature are unknown.
- Whether hobby propellants are steeper on the warm side; the RDS-700's impulse at each temperature.
- Corrêa's 2005 ITA dissertation, which may discuss the impulse rise; not found online.

## Sources

All read 2026-11-04. `trf:` is `https://www.rocketryforum.com/threads/`, `cdn:` is `https://cdn.rocketryforum.com/`.

1. RocketryForum, "Propellant Temperature Effect on Motor Performance?",
   `trf:propellant-temperature-effect-on-motor-performance.190157/`, posts 8, 12, 13 (2024-12-23 and 24); charts
   `cdn:a/602102-6814c9192ba4fd674b123b29cf128ef5/Burn-times.png`,
   `cdn:a/602103-86baa11aa600c497202f25659e93be90/total-thrust.png` (impulse),
   `cdn:a/602104-4adf8c2924acdb513b6587fa66d242c1/Dealy-times.png` (delay).
2. Same thread, post 9 (AeroTech's account): Industrial Solid Propulsion, RDS-700 sheet, catalog p. 24,
   `cdn:a/602064-9e445f9bdfa9390df230874917c63a70/IMG_9509.jpeg`.
3. Corrêa, Shimote, Rapozo, Lacava, ENCIT 2006, paper CIT06-1015, pp. 4, 10–11, Tables 2–3.
   <https://abcm.org.br/app/webroot/anais/encit/2006/arquivos/Combustion%20and%20Fuels/CIT06-1015.pdf>
4. NAR R&D report index (cached 2026-10-17; 540 ids, [NAR R&D reports](nar-rd-reports.md)); report links
   `https://www.nar.org/docs.ashx?id=1358134` (member login).
   <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
5. NAR certification data sheets, `https://www.nar.org/docs.ashx?id=<id>`, as cached for
   [apogee bias](apogee-bias.md) and [HPR burn-time scatter](hpr-burn-time-scatter.md).
6. ThrustCurve API capture of 2026-10-28, `https://www.thrustcurve.org/api/v1/download.json`
   ([ThrustCurve data](thrustcurve-data.md)).
7. L. Haines, "LOHAN motor heater test", *The Register*, 2014-04-23.
   <https://www.theregister.com/2014/04/23/lohan_motor_heater_test/>
8. Homestead High School Aerospace Club, JMP Discovery Summit 2019 abstract 2019-US-EPO-194.
   <https://community.jmp.com/t5/Abstracts/Optimization-of-Rocket-Model-Parameters-for-Team-America/ev-p/849395>
9. openMotor issue 208, "Ambient Temperature Dependence" (2023-01-18).
   <https://github.com/reilleya/openMotor/issues/208>
10. M. Nowak, "The Effect of Air Temperature on Model Rocket Maximum Altitude", NAR R&D report, NARAM-50 (2008),
    pp. 6, 10–13. `https://www.nar.org/docs.ashx?id=1358134` (member login).
11. M. Nakano, H. Mashidori, Y. Takahashi, "Effects of Temperature and Aging on Performance of A8-3 Model Rocket
    Engines", TMCIT bulletin 7 (2013) ([NAR delay reports](nar-delay-reports.md) source 6).
12. Florczak, Białek, Cholewiak, Cent. Eur. J. Energ. Mater. 11(4), 589–601 (2014), Table 2, p. 600
    ([propellant temperature](propellant-temperature.md) source 4).
