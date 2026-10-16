# How far flight-measured drag of hobby rockets sits from simulator estimates

Flyers often say that a C_D taken from a flight log comes out above OpenRocket's or RockSim's. This note collects
every public case found with both numbers for one rocket: a flight-derived C_D and a code's C_D over the same
speeds. Is there a shared factor, how big, and does it depend on Mach or on build? **In short: the answer depends
on the method, and no shared factor shows.** Fourteen fits to the coast's deceleration put flight C_D at a median
1.02 times the code's (quartiles 0.89–1.11, range 0.84–1.16, 8 of 14 above 1). Twelve fits that tune C_D until the
simulated apogee matches give a median 1.27 (0.58–1.92): that one number also absorbs mass, motor, boost, wind and
launch-angle errors. Where one team ran both methods on two flights, they differ by 0.23 and 0.37. Every OpenRocket
and RockSim coast pair is subsonic; two rockets flown to Mach 1.5 and 2.4 read below RASAero II. Finishing one nose
cone moved one rocket's fitted C_D by 20%. **Inference:** through [apogee-bias](apogee-bias.md)'s elasticities, the
coast median puts 0.6–1.1% of apogee on missed drag (3–5% if the masses ran light): a tenth to a half of the
private collection's +8.73% mean. How far to trust this: 26 pairs from six student teams, one paper and one
builder; most don't say whether the mass was weighed, and half the coast fits differentiate altitude for velocity.

## 1. How a flight gives a C_D, and what each method folds in

- **Coast deceleration.** After burnout only drag and gravity act, so C_D = 2m(−a − g sin γ)/(ρv²A), with m the
  coasting mass, a the axial acceleration (negative in coast), γ the flight-path angle, ρ the air density, v the
  speed and A the reference area [S13] eq. 1.3, [S15]. It sees drag and mass, not the motor.
- **Apogee matching.** Here C_D is scaled in the simulator until the simulated apogee equals the logged one. The
  sources name motor scatter and wind [S6] p. 5, [S15] p. 4; mass, rail angle and boost errors land on C_D the same
  way (**Inference**).
- **Boost-phase balance.** Drag comes from impulse minus momentum over the burn. It is worse still: Purdue 2020 got
  1.05 against OpenRocket's maximum 0.49 (ratio 2.14), yet OpenRocket's apogee matched the flight within about 2%
  (4,584 ft against 4,488 and 4,688 ft) [R22] pp. 48–49.

## 2. Coast fits against a hobby code (14 pairs)

Ratio = flight C_D / code C_D. Velocity: b = barometric altitude differentiated, a = accelerometer, i = IMU with
vertical components only. OR = OpenRocket, RS = RockSim, RAS = RASAero II. Pages are the PDF's.

| case [source] | flight | code | ratio | method; build note |
|---|---|---|---|---|
| NC State 2023 subscale [R6] pp. 34, 36 | 0.58 | 0.55 RS | 1.05 | b; camera housings |
| NC State 2024 subscale [R7] p. 49 | 0.52 | 0.45 OR | 1.16 | b; area includes fins |
| Notre Dame 2021 subscale [R12] p. 38 | 0.31 | ≈0.3 OR | 1.03 | b; 0.31–0.54 by window |
| Notre Dame 2022 full scale [R14] p. 73 | ≈0.55 | 0.48 (code unnamed) | 1.15 | b, polynomial fit |
| Notre Dame 2023 subscale [R15] pp. 46–47 | 0.374 | 0.432 OR; 0.376 RS | 0.87; 0.99 | i, burnout to 4 s |
| Notre Dame 2023 full scale [R16] p. 109 | 0.448 | 0.442 OR; 0.434 RS | 1.01; 1.03 | i + b; about 40° weathercock |
| FAMU-FSU 2024 flight 1 [R18] §5.1.17 | ≈0.75 | ≈0.65 OR | 1.15 | b; nose cone unfinished |
| FAMU-FSU 2024 flight 2 [R18] §5.2.16 | ≈0.60 | ≈0.64 OR | 0.94 | b; nose cone finished |
| FAMU-FSU 2024 flight 3 [R19] §2.8 | ≈0.68 | ≈0.63 OR | 1.08 | b; nose "rougher" than the file |
| FSU Panama City 2026 demo 1 [R20] p. 29 | 0.399 | 0.357 OR | 1.12 | a |
| FSU Panama City 2026 demo 2 [R20] p. 28 | 0.347 | 0.367 OR | 0.95 | a; camera mount in the file |
| FSU Panama City 2026 launch [R21] p. 9 | 0.322 | 0.367 OR | 0.88 | a; 100–185 m/s |
| Violent Agreement, I216 [S1] | ≈0.22–0.24 | ≈0.27–0.29 RAS | ≈0.84 | a; Mach 0.4–0.85 |
| Violent Agreement SS, J530 [S2] | ≈0.29–0.32 | ≈0.33–0.35 RAS | ≈0.88 | a; Mach 0.6–0.85 |

- **Statistics** (this note's arithmetic, OpenRocket where a report gives both codes): n = 14, median 1.02,
  quartiles 0.89–1.11, range 0.84–1.16. Hobby codes other than RASAero (n = 12): median 1.04, quartiles 0.94–1.13.
- **The two RASAero rockets** come from one builder, published by the code's author. They are read from plots, about
  ±0.01 in C_D, so their ratios are **Inference**. Their supersonic ratios are 0.91–0.95 (I216, Mach 1.1–1.4) and
  0.85–0.97 (J530, Mach 1.1–2.3) [S1, S2]. The J530's coast C_D matched, yet its simulated apogee ran 8.35% high
  (24,784 ft against 22,874 ft); the page blames early angle-of-attack oscillations [S2].
- **Mass** enters C_D linearly. FAMU-FSU weighed all hardware after flight 1: 4.5 lb over its file [R18] §5.1.16.
  NC State 2023 and Notre Dame 2022 ran measured masses [R6] p. 34, [R14] p. 26. The rest don't say.
- **Reference area.** NC State 2024 used body tube area plus fin area [R7] p. 49. **Inference:** a larger A gives a
  smaller C_D for the same drag, so its 1.16 understates the ratio on OpenRocket's body-area basis.
- **Against CFD, not a hobby code.** One ICAS paper fits pitot-plus-IMU coasts against the author's CFD: 0.2480
  against 0.29 (0.86) and 0.4384 against 0.44 (1.00), with wet masses weighed [S3] p. 8. The CFD left out screws,
  buttons and holes, so it should read low [S3] pp. 26–27, Tables 1, 3. NC State's ten coast fits against its own
  ANSYS Fluent runs (2024–2026) give a median 0.96, range 0.84–1.05 [R7] p. 49, [R9]–[R11], [R33]–[R37]. Several
  of those flights flew with air brakes out.

## 3. Apogee-matched C_D (12 pairs)

| case [source] | fitted | code | ratio | notes |
|---|---|---|---|---|
| NC State 2021 demo [R4] pp. 104–105 | 0.634 | 0.37 RS maximum | 1.71 | weights overridden |
| NC State 2021 second flight [R5] p. 26 | 0.71 | 0.37 RS maximum | 1.92 | |
| NC State 2023 full scale [R8] p. 156 | RS curve × 1.0325 | RS curve | 1.03 | matched in RocketPy |
| Toledo 2024 subscale [R26] p. 32 | 0.692 | 0.653 OR | 1.06 | masses set to measured |
| Toledo 2024 demo [R27] p. 80 | 0.660 | 0.479 OR | 1.38 | weathercocking named |
| Toledo 2024 re-flight [R28] p. 19 | 0.589 | 0.479 OR | 1.23 | |
| Toledo 2025 subscale [R29] p. 55 | 0.879 | 0.562 OR | 1.56 | apogee 12.3% low |
| Toledo 2026 subscale [R31] p. 51 | 0.810 | 0.615 OR | 1.32 | cardboard, printed parts |
| FSU Panama City 2026 demo 1 [R20] p. 29 | 0.318 | 0.357 OR | 0.89 | coast fit, same flight: 0.399 |
| FSU Panama City 2026 demo 2 [R20] p. 28 | 0.211 | 0.367 OR | 0.58 | coast fit, same flight: 0.347 |
| Penn State 2018 full scale [R23] p. 40 | 0.597 | 0.600 OR | 0.995 | masses measured on launch day |
| 66 mm model, 8 flights [S4] Table 1 | 0.659–0.894 | 0.52 OR | 1.69 (median) | apogee only, thrust also fitted |

- **Statistics** (this note's arithmetic): n = 12, median 1.27, quartiles 1.02–1.60, range 0.58–1.92, 9 above 1.
- **The 66 mm model** [S4] (arXiv 2512.22248, CC BY 4.0) infers C_D and a thrust factor from apogee alone. Its prior
  was C_D uniform on 0.3–0.9 and thrust factor on 0.8–1.2 [S4] §4.2. Five of the 8 C_D values sit at 0.876–0.894,
  near the prior's top, and five thrust factors (0.768–0.784) fall below its floor [S4] Table 1. **Inference:** the
  estimates are pinned by the prior, not by the data; the authors call them "effective" parameters [S4] §6.4.
- **Evidence that apogee fits absorb the boost.** Toledo 2026's subscale matched apogee at 0.810, yet the tuned
  simulation's peak speed still ran 12.5% above the flight's [R31] p. 51. NC State 2019 fitted 0.55 to one flight
  and 0.365 to a re-flight from a 10 ft rail, crediting its surer wind, rail-exit speed and early angle: a factor
  of 1.51 between two flights of one rocket [R2] p. 64, [R3] pp. 13–14. NC State scaled its Fluent curve ×0.85 to
  match a 2024 flight, and the next flight fell 3.6% short of the rescaled prediction [R38] p. 14, [R39] pp. 152,
  154: the factor didn't carry over.

## 4. Does it depend on Mach or on build?

- **Mach.** Every OpenRocket and RockSim coast pair is subsonic: quoted speeds put the coasts at about Mach 0.6
  or less. Only the two RASAero rockets pass Mach 1. Their ranges overlap, but typical ratios are about 0.84–0.88
  subsonic and 0.90–0.95 supersonic [S1, S2]. **Inference:** nothing public shows a bias that grows with Mach. On
  these two rockets RASAero reads high subsonic, the opposite of the folklore. A transonic or supersonic coast
  fit against OpenRocket or RockSim with numbers was not found.
- **Finish.** FAMU-FSU's fit fell from 0.75 to 0.60 (−20%) once the nose cone was finished [R18] §5.1.17, §5.2.16.
  Notre Dame 2021 flew unpainted against an OpenRocket "smooth paint" file: coast fit 0.31, OpenRocket's apogee
  1.7% high [R13] pp. 85, 88. On Niskanen's airbrushed, weighed 29 mm model, OpenRocket read 7% high (C6-3) and
  RockSim 8 read 19% high [S7] pp. 83–85, Table 6.1. **Inference:** through §6's elasticities, 7% alone would take
  12–23% more C_D, and a file's finish can sit on either side of the built one
  ([hobby-finish-roughness](hobby-finish-roughness.md)).
- **Material.** Toledo's cardboard vehicles fitted 1.06–1.38 × OpenRocket in 2024 and 1.32 in 2026 [R27] p. 16,
  [R31] p. 50. Its 2026 fiberglass full scale overflew a prediction carried over from the subscale fit, and the team
  names the material [R32] p. 107. That is one team's evidence.
- **Inputs the code is given.** NC State 2017: switching OpenRocket's fin edges from square to rounded moved C_D from
  0.55 to 0.45 (−18%) [R41] p. 53. A 2022 competition paper: the same switch moved a prediction from 3,751 ft to
  4,625 ft, against 4,814–4,820 ft flown [R42] p. 9. **Inference:** one file setting moves C_D about as much as the
  coast fits' interquartile spread (0.89–1.11).
- **Lugs and buttons.** Niskanen's 24 × 5 mm lug cost 2–3% of apogee in flight against 1.3% in OpenRocket [S7] p. 85.
  A 2026 NAR R&D report checking simulated aerodynamics against CFD and flight returned 403 [S11].

## 5. What the measurement adds, with sign and size

- **Velocity from altitude reads C_D high.** In the ICAS paper, C_D from differentiated altitude ran +7.0%, +19.5%,
  +25.1% and +85.1% above the pitot-plus-IMU values on its four flights [S3] Tables 1–4 (this note's arithmetic).
  Seven of the 14 coast pairs above use barometric velocity. NC State 2014's barometric fit fell from 0.97 to 0.73 as
  more samples were averaged ([R50] Table 17, pp. 48–49).
- **Mass.** A 1% mass error is a 1% C_D error [S13]. A fit on a file mass 7.4% light, the as-built median
  ([as-built-mass-growth](as-built-mass-growth.md)), reads C_D about 7% low (**Inference**).
- **Air density.** A constant pad density [S6] p. 15 biases C_D low by about 9% per 1,000 m climbed (**Inference**:
  standard-atmosphere density falls 9.2% from 0 to 1,000 m).
- **Accelerometer offset**, either sign: its share is δa/(D/m), small near burnout and unbounded near apogee where
  drag per mass goes to zero (**Inference** from the coast equation [S13]). Two boards in one rocket agreed to 0.38%
  in coast [S3] p. 6.
- **Angle of attack and wind** read C_D high [S6] p. 5; Notre Dame's curves climb with zenith angle [R16] p. 108.
  **Thrust tail-off** counted as coast reads C_D low: start after burnout [S5] pp. 7–8, [S6] p. 5.

## 6. What it means for apogee

The drag elasticities in [apogee-bias](apogee-bias.md) run −0.31% to −0.57% of apogee per 1% of C_D. **Inference:**
the coast fits' median +2% of C_D makes a simulation read 0.6–1.1% high; their quartiles (−11% to +11%) span about
±3–6% of apogee on one rocket. If they used light file masses, the true ratio is higher: at the as-built median,
about 1.10, or 3–5% of apogee. Against the private collection's +8.73% mean, the coast fits' median is roughly a
tenth, a third to a half if their masses ran light. The apogee-matched median (+27%) would be 8–15% of apogee, but
it is not a drag measurement: it is the whole apogee error expressed as C_D.

## What this suggests

1. **M7.3's C_D-scale prior** centered near 1.0, with about ±15% spread, fits the coast evidence. An apogee-matched
   effective C_D needs a wide, right-skewed prior only if thrust and mass are held at nominal.
2. **M7.5 should fit airframe drag only from the post-burnout coast**, with the weighed mass, accelerometer velocity
   where the log has one, and density from the day's air. Apogee alone can't separate drag from impulse and mass.
3. **Report a coast-fit C_D ratio per flight** (aggregates only) on the private collection. If its median stays near
   1, missed drag is not the bias's main share.
4. **Flag the fin-edge and finish settings** in a design's summary: one file setting moves C_D by up to 18%.

## Open questions

- No transonic or supersonic coast fit against OpenRocket or RockSim was found with numbers.
- NAR R&D reports were not reached (downloads 403): Steele 2006, Stevens 2007, M. Steele 1986, Calvert 2005, Karbon
  2026 [S11]. Paywalled: SERA-3 (AIAA 2018-2382); Fors and Nord, Am. J. Phys. 87, 714 (2019).
- A 2025 AIAA paper on an IREC rocket, said in a search snippet to find OpenRocket's C_D off "by almost 0.3",
  returned 403. One builder's in-flight C_D against RASAero is only in forum images [R48] post 1.
- Most teams don't say whether burnout mass was weighed, and none bounds its velocity noise.
- A 2025 RMIT preprint on a J825R rocket says its coast C_D is "considerably lower" than RASAero's and closer to
  OpenRocket's, but prints no values [S5] p. 9, Fig. 17. It is not counted.

## Sources

Read 2026-10-16; copies in the gitignored `refs/research/flight-derived-cd/`. Terms: S4 CC BY 4.0; the rest
reserved or unstated, cited only. NC State's reports: <https://ncsurocketry.org/report-archive/>.

- **S1, S2.** RASAero II, comparisons with flight data: Violent Agreement (I216) and Violent Agreement SS (J530).
  <http://www.rasaero.com/comp-flight-ViolentAgreement.htm>, <http://www.rasaero.com/comp-flight-ViolentAgreementSS.htm>
- **S3.** Kierulf, ICAS 2022-0414. <https://www.icas.org/icas_archive/ICAS2022/data/papers/ICAS2022_0414_paper.pdf>
- **S4.** Pandey and Pandey, arXiv:2512.22248v1. <https://arxiv.org/abs/2512.22248>
- **S5.** Bawa and Michalski, "An Experimental Framework for In-Flight Determination of Rocket Aerodynamic Drag", RMIT
  preprint, 2025, in <http://ftp.demec.ufpr.br/foguete/bibliografia/>.
- **S6.** Van Milligan, Peak of Flight 651 (2025). <https://www.apogeerockets.com/education/downloads/Newsletter651.pdf>
- **S7.** Niskanen, MSc thesis, TKK, 2009, printed pages. <https://openrocket.sourceforge.net/thesis.pdf>
- **S11.** NAR R&D report index. <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
- **S13.** Harvey Mudd AdvRoc, "CD From Post-Boost Flight Data".
  <https://pages.hmc.edu/spjut/AdvRoc/PDF/CDfromPostBoost.pdf>
- **S15.** Van Milligan, NARAM-54 R&D (2012).
  <https://www.apogeerockets.com/downloads/Cd_determination_using_Coast_Data.pdf>
- **R2–R11, R33–R41, R50.** NC State, by season on the archive: R2 2019 FRR, R3 its addendum; R4 2021 FRR, R5 its
  addendum; R6 2023 CDR; R7 2024 CDR; R8 2023 FRR; R9 2025 CDR; R10 2025 FRR, R11 its addendum; R33 2025 PLAR; R34
  2026 CDR; R35 2026 FRR, R36 its addendum; R37 2026 PLAR; R38 2024 FRR addendum; R39 2024 FRR; R41 2017
  FRR; R50 2014 CDR.
- **R12–R16.** Notre Dame, <https://seniordesign.ee.nd.edu/>: R12 2021 CDR and R13 2021 FRR
  (`2021/Design%20Teams/rocketpay/`), R14 2022 FRR (`2022/Design%20Teams/NDRT/sliFrr.pdf`), R15 2023 CDR and R16 2023
  FRR (`2023/DesignTeams/rocket/NDRT_CDR_2022_2023.pdf`, `NDRT_FRR_2022_2023.pdf`).
- **R18, R19.** FAMU-FSU 2024 FRR and addendum (docx; section numbers, as the files have no page numbers).
  <https://eng-web1.eng.famu.fsu.edu/me/senior_design/2024/team502/documents/NASA/FRR.docx>
- **R20, R21.** FSU Panama City 2026 FRR addendum and PLAR,
  <https://web1.eng.famu.fsu.edu/me/senior_design/2026/team533pc/files/>.
- **R22.** Purdue SEDS 2020 FRR. <https://purdueseds.space/wp-content/uploads/psp-sl/casper/frr/>
- **R23.** Penn State 2018 FRR. <https://sites.psu.edu/psurocket/files/2017/08/Penn-State-2018-FRR-Report-1iwefpe.pdf>
- **R26–R32.** Toledo, 2024–2026 CDR, FRR and addenda, linked from
  <https://www.utrocketry.club/projects/nasa-usli-2024> (and `-2025`, `-2026`).
- **R42.** Bohlman, Lee and Muller, "2022 Midwest High-Power Rocket Competition", Wisconsin Space Grant proceedings.
  <https://dione.carthage.edu/ojs/index.php/wsc/article/download/377/375>
- **R48.** The Rocketry Forum, "Extracting Cd from accelerometer data".
  <https://www.rocketryforum.com/threads/extracting-cd-from-accelerometer-data.124890/>
