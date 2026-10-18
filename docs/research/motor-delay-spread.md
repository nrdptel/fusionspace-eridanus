# How accurate COTS ejection delays are

How far does a commercial motor's ejection delay stray from its label, and what spread should a Monte Carlo fly?
It serves the settings check (M6.8 in the [roadmap](../ROADMAP.md)), the "delay mismatch" fault (M7.4) and the
optimum-delay output. It builds on [early deployment bias](early-deployment-bias.md), whose NAR count it redoes, and
[fault base rates](fault-base-rates.md) §3. Read 2026-10-17: NFPA revision reports, three certifiers' manuals, 167
NAR sheets, 50 Tripoli (TRA) letters, makers' sheets, one bench study; no source code. **Inference** marks this
note's arithmetic. Trust it for model motors at 20 °C (about 400 firings); thinly for high power (about 60 firings,
mostly AeroTech-built, none above J); not for cold composite delays or high sites, where only the sign is known.

**In short.**
- **The rule is wide.** NFPA 1125 lets each firing's delay differ from its label by 1.5 s or 20%, whichever is
  greater, capped at 3 s, for every class [1]; Tripoli's testers treat a miss as a difference, not a failure [6].
- **Model motors run about 3% short, with 0.5 s of scatter per firing.** On 249 NAR motor-and-delay values the delay
  ran a median 0.19 s (2.9%) short, SD 0.61 s [8]. Firings scatter about their own type's mean by 0.50 s at every
  length; type means stray from labels by a further 0.16 s (labels of 4 s or less) to 0.74 s (9 s and up).
- **High-power delays run 10% short undrilled and 0.8 s long drilled.** 36 firings of 10 undrilled high-power motors
  (NAR and TRA) ran about 1.0 to 1.5 s (9% to 11%) early; 21 TRA firings drilled shorter ran +0.83 s late, because
  each labeled second drilled away removed a median 0.65 s of measured delay [8, 9].
- **Temperature and altitude are unsized for composite delays.** A black-powder study gives −0.30% per K (a 0 °C
  motor 6% long) [16]; NAR and Tripoli found composite delays can burn more slowly at high sites, size not given [14].

## 1. What the standards allow

- **NFPA 1125.** §8.1.7(1)(b) (model motors, at least 10 samples) and §8.2.7(1)(b) (high power, "two or more"
  samples) read the same in the A2021 revision: no delay may vary from its label by more than 1.5 s or 20%,
  whichever is greater, but not over 3 s [1, FR-7 p. 5/9, FR-8 p. 7/9]. The 2018 text carried the same sentence
  [2, PI-5 p. 3/11]. It binds each firing, not the mean. **Inference:** the band is ±1.5 s up to a 7.5 s label,
  ±20% from 7.5 to 15 s, ±3 s above 15 s. The A2025 first revisions leave both clauses alone [3, pp. 1–4/4].
  Firings are "at or corrected to sea level and 20°C ± 5°C" [1, FR-7 p. 5/9].
- **NFPA 1127** (the flyer's code) has no delay tolerance in the texts read. Its A2025 FR-3 adds §4.5.3: a
  single-use high-power motor may be altered only as the maker permits, written for adjustable delays such as
  AeroTech's DMS [4, FR-3 p. 4/9; 5, PI-7 p. 3/11]; issued editions, which need an account, were not read.
- **When the clock starts.** No NFPA text reached defines it. CAR times from burn end (thrust first under 5% of peak)
  to the ejection charge, rounded to 0.25 s [7, §12.1.1 to §12.1.2, pp. 19–20]; Cesaroni, Estes and an RCS patent
  say "from burnout" [12, 13, 15]. **Inference:** a log's burnout (acceleration turning negative) falls after the
  5%-of-peak point on a long tail-off, so a delay timed from a log reads short.

## 2. What the certifiers fire

| Firings per delay | NAR manual v1.5 [10] | TMT, 2020 [6] | CAR, 2018 [7] |
|---|---|---|---|
| Model | 3+, 11 in all (§3.2.1, p. 7) | 3+, 10 in all (§6.3.1, p. 6) | 3+, 11 in all (§4.2.1, p. 8) |
| High power to 5,120 N·s | 3 (§3.3.1, p. 7) | 1+, 3 in all (§6.4.1.1, p. 7) | 3 (§4.3.1.1, p. 8) |
| High power above 5,120 N·s | 2 (§3.3.4, p. 7) | 2 in all (p. 7) | 2 (§4.3.3.1, p. 9) |

- **Adjustable delays.** NAR fires 10 or more, cut with the maker's tool: 3 at the longest, 3 at the shortest, 2 at
  each of two or more settings between [10, §10.1.3, p. 20]. TMT fires 3 or more for high power: shortest, longest,
  one between [6, §6.4.1.2, p. 7]. CAR's manual gives no adjustable-delay rule and no delay pass rule.
- **Many records give no delay times:** 57 of 167 NAR sheets (plugged, none or RC), 34 of 50 TRA letters [8, 9].
  **No certifier fires cold or hot:** NAR conditions motors to 20 °C ± 5 °C [10, §8.3.3, p. 16].

## 3. Measured: NAR data sheets

**Method.** A script read 167 cached NAR sheets (the 157 from [apogee bias](apogee-bias.md), 7 retests, 3 more from
ThrustCurve); 110 carry labeled delays and all parsed. Eight repeat another sheet's data (mostly rebrands) and were
dropped. Left: 301 single firings in 78 motor-and-delay groups, plus 171 sheet means: 249 motor-and-delay values.
Six sheets were checked by eye against the PDFs; all matched. Offsets are measured minus label.

| Set | n | Mean | Median | SD | 5th to 95th percentile | Early |
|---|---:|---:|---:|---:|---:|---:|
| Motor-and-delay values | 249 | −0.17 s (−2.8%) | −0.19 s (−2.9%) | 0.61 s | −1.24 to +0.80 s | |
| Single firings | 301 | −0.28 s (−3.4%) | −0.26 s | 0.73 s | −1.48 to +0.89 s | 63% |
| Firings, labels of 4 s or less | 103 | −0.08 s | −0.08 s | 0.52 s | −0.90 to +0.75 s | 54% |
| Firings, labels of 5 to 8 s | 110 | −0.13 s | −0.07 s | 0.67 s | −1.28 to +0.90 s | 59% |
| Firings, labels of 9 s and up | 88 | −0.69 s (−6.4%) | −0.82 s | 0.86 s | −1.90 to +0.87 s | 78% |
| Firings, high power | 24 | −1.03 s (−9.1%) | −0.97 s | 0.79 s | −2.18 to −0.03 s | 96% |
| Values: Estes / AeroTech / Apogee | 39 / 159 / 37 | −0.38 / −0.09 / −0.35 s | | 0.45 / 0.65 / 0.55 s | | |
| Values: black powder / composite | 54 / 195 | −0.34 / −0.12 s | | 0.47 / 0.64 s | | |

- **Scatter is constant in seconds; the bias grows with length.** Pooled within-group SD is 0.44, 0.51 and 0.55 s
  for short, mid and long labels (0.50 s overall, 223 degrees of freedom). Group means scatter about their labels by
  a further 0.16, 0.45 and 0.74 s once that firing noise is removed: about 6.5% of the label above 4 s.
- **Nothing is clearly out of band.** The one value outside sits under a mislabeled column on the G79 sheet (it is
  7 s, not 8 s, and then within band). Five firings use 97% to 99% of the band.
- **Six sheets disagree with themselves** (a table against its firings, a heading against the delays): use firings.
- **This replaces** the 0.13 to 0.14 s of [early deployment bias](early-deployment-bias.md), which counted twice.
- **The high-power side is thin:** 24 firings of 4 motors (three AeroTech, one AMW), 8 means of 4 more, 1996 to 2008.

## 4. Measured: TRA letters

16 of the 50 TRA letters cached from ThrustCurve list each firing's delay: 156 firings, tested 2017 to 2024, all
AeroTech-built (RCS) brands [9]. Counts match each letter's "Number of Motors Tested" but one; two values that the
J520W-13A letter repeats from the J435WS-13A letter were dropped.

| TRA set | Firings (motors) | Mean | Median | SD | Range |
|---|---:|---:|---:|---:|---|
| Model, fixed delays | 124 (11) | −0.22 s (−1.4%) | −0.08 s | 0.77 s | −2.13 to +1.28 s |
| High power, adjustable, full length | 11 (5) | −1.45 s (−10.7%) | −1.47 s | 1.02 s | −2.95 to +0.09 s |
| High power, adjustable, drilled shorter | 21 (5) | +0.83 s (+11.1%) | +0.85 s | 0.52 s | −0.31 to +1.64 s |

- **Drilled delays run late, full-length ones early.** A J435WS-13A labeled 14 s full length averaged 12.57 s;
  drilled to 10 s, 10.53 s; to 6 s, 7.05 s [9]. NAR's G138T sheet agrees: 14 s full read 13.66 s, drilled to 11.66
  and 8.53 s targets read 12.99 and 9.23 s [8]. **Inference:** each labeled second drilled away removed 0.12 to
  0.92 s of measured delay (median 0.65 s; 12 settings, 6 motors).
- **The cause is open.** AeroTech drills 0.024 to 0.032 in per second [11, pp. 1–2; 17, p. 1]; its 10 s and 6 s
  I161W delay modules are 0.594 and 0.500 in [11, p. 1]. No source says why a drilled second removes about 0.65 s.
- **Two TRA firings broke the NFPA band,** both J520W-13A: drilled to 5 s, 6.64 s (+1.64 s against ±1.5 s); full
  13 s, 10.05 s (−2.95 s against ±2.6 s) [9]. The motor was certified.

## 5. The adjusting tools

- **AeroTech.** The RDDT (18 to 38 mm, 54 mm) and DMS UDDT tools cut 4 or 8 s; a washer makes it 2 or 6 s; no cut
  below 4 s [18, 19]. Its 4/2/23 motor matrix, counted for this note, has 362 designation rows: 112 adjustable (96
  of them `-14A`), 103 plugged, 147 fixed at 3 to 14 s [20]. No AeroTech sheet states an accuracy.
- **Cesaroni.** The Pro24, Pro29 and Pro38 tools remove 3, 5, 7 or 9 s; Pro54's has a vernier from 0 to 10 s; the
  floor is 4 s [12, 21, p. 2]. As shipped: Pro24 `-7A` to `-13A`, Pro29 `-9A` to `-15A`, Pro38 `-9A` to `-17A` [21].
  Cesaroni says only that Pro54 delays met industry tolerance in certification, with no figure, and keeps its kits at
  −5 °C to 30 °C "for storage and use" [21, p. 1–2]. No Cesaroni delay firing was found in any certifier's data.
- **Loki** drills 1/16 in per 2 s, at most 3/8 in (12 s), with no accuracy figure [22, pp. 1–2]. **Inference:** a
  hand-drilled depth off by 1/64 in (0.4 mm) moves a delay about 0.5 s at 0.8 mm per second.

## 6. Temperature, pressure and timing

- **Temperature (black powder only).** Nakano et al. fired 58 Estes A8-3 engines held near 2, 24 and 36 °C [16,
  p. 2]. Their fit: delay = −0.00833 T + 5.24 s, T in K, R² = 0.576 [16, p. 3, Fig. 8], about −0.30% per K on the
  fit's 2.76 s at 25 °C. **Inference:** a 0 °C motor runs about 0.17 s (6%) long against 20 °C, and NFPA's ±5 °C
  window is ±1.5% of the delay. The authors blame hot propellant eroding the delay layer, not a faster-burning delay
  [16, pp. 3–4], so the slope may not carry to composite grains. No composite measurement was found.
- **Pressure (sign only).** NAR S&T's certified-motor list says NAR and Tripoli tests found composite delays can burn
  more slowly as the site rises by thousands of feet, so a delay in band at sea level may run long at 5,000 ft [14,
  p. 2]: **later ejection**, size not given. **Inference:** after burnout the case vents, so the grain burns near
  outside pressure; with burn rate ∝ pⁿ and an assumed n of 0.3 to 0.8, a delay runs 6% to 16% long at 5,000 ft
  (ISA 84.3 kPa) and 12% to 35% at 10,000 ft. The exponent is assumed, not sourced.

## 7. A delay distribution a Monte Carlo could fly (Inference)

measured delay = L × (1 + b) + e, e ~ Normal(0, σ), σ = √(0.5² + (0.065 L)²) s, with L the label or drilled
target in seconds, timed from 5%-of-peak burnout, at 20 °C and sea level.

| Class | b | σ at L = 4, 7, 10, 14 s | Rests on | Inside the 90% / 95% intervals |
|---|---:|---|---|---|
| Model, fixed | −3% | 0.56, 0.68, 0.82, 1.04 s | 301 NAR firings, 78 groups | NAR 91 / 96%; TRA (held out) 87 / 97% |
| High power, undrilled | −10% | as above | 36 firings, 10 motors | 94 / 100% (fitted, not held out) |
| High power, drilled | L + 0.8 s | 0.55 s | 23 firings, 6 AeroTech motors | 96 / 96% (fitted, not held out) |

- **Intervals, model class:** 4 s, 90% 3.0 to 4.8 s; 10 s, 90% 8.4 to 11.0 s; 14 s, 90% 11.9 to 15.3 s. With
  a NAR sheet for the motor, center on its average; one firing then has σ near 0.5 s.
- **Tails** are near normal: 11 of 301 firings sit beyond 2 SD of their band (3.7%, against 4.6%). The largest
  misses are −2.61 s (NAR, H268-14) and −2.95 s (TRA, J520W-13A); the latest +1.64 s.
- **Black powder, cold or hot:** multiply by (1 − 0.0030 ΔT), ΔT in K from 20 °C, from one study [16]. **Left
  out:** composite temperature, site altitude, age, Cesaroni's tools, motors above J.

## What this suggests

- **Fly the distribution, not the label.** The flight-metrics optimum-delay output and M6.8's settings check should
  compare the optimum with §7's interval for the class, and say which burnout the delay is timed from.
- **Long and undrilled delays err early, drilled ones late:** M6.8's check by side and M7.4's prior should use the
  class bias, not a symmetric band. An undrilled 14 s high-power delay centers near 12.6 s.
- **Warn on the drill.** Take the drilled setting as input; warn below 4 s and past a reload's longest cut.
- **Size the altitude term from public letters:** TMT has fired near Denver since 2025 ([TMT test
  sites](tmt-test-sites.md)), so its letters against the same motors' sea-level ones could bound it.

## Open questions

- **Altitude.** TMT's 2018 study (*Tripoli Report* 29(3): about 40 motors at 5,500 ft and 11 ft, about 30% out of
  band, per a forum summary [23]) has no open copy; no burn-rate exponent for delay grains near 1 atm was found.
- **NAR R&D reports, not read:** "Delay Time Variability for APCP F Motors" (2013), "Igniter Placement vs Delay
  Times in Composite Motors" (2004), "Age Related Changes in Motor Ejection Delay Time" (1989), "Effect of Extreme
  Cold on BP Motors" (1984), all NARAM reports in NAR's index [24]. They need a member login; not read.
- Composite delays when cold, Cesaroni delays at all, high-power delays above J; flight logs could time delays in
  use (burnout to the charge's pressure spike), which no source tallies.

## Sources

Read 2026-10-17; cached under `refs/`, cited, not copied. NFPA: `https://docinfofiles.nfpa.org/files/AboutTheCodes/`;
`trf:` `https://www.rocketryforum.com/`; `p38:` `https://pro38.com/wp-content/uploads/2026/08/`;
`cf:` `https://d3l66gvjdr7rqw.cloudfront.net/Templates/170652/myimages/`.

1. NFPA 1125 A2021 first revisions: `1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf`.
2. NFPA 1125 A2021 public inputs: `1125/1125_A2021_PYR_AAA_FD_PIResponses.pdf`.
3. NFPA 1125 A2025 first revisions: `1125/1125_A2025_PYR_AAA_FD_FRStatements.pdf`.
4. NFPA 1127 A2025 first revisions: `1127/1127_A2025_PYR_AAA_FD_FRStatements.pdf`.
5. NFPA 1127 A2025 public inputs: `1127/1127_A2025_PYR_AAA_FD_PIResponses.pdf`.
6. Tripoli Motor Testing, Policies and Procedures, rev. 2020-01-06 (Internet Archive, 2020-11-28).
7. CAR Motor Test Manual rev. 3.2 (2018):
   <https://car-acf.org/resources/Documents/car_motor_testing_manual%20r20181121.pdf>
8. NAR S&T motor data sheets, 167 read, via ThrustCurve's links; e.g. <https://nar.org/SandT/pdf/Aerotech/I218.pdf>
   (also `G138T`, `G79`, `H268`); retests at `https://www.nar.org/docs.ashx?id=1481390` and following.
9. TRA certification letters on ThrustCurve, `https://www.thrustcurve.org/motors/cert/<id>/`; e.g. J435WS-13A
   6669a8f0efb6ed0002e49cb2, J520W-13A 6669aa45efb6ed0002e49dc6.
10. NAR S&T Motor Testing Manual v1.5 (2011): <https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf>
11. AeroTech RMS Delay Modification Procedure, 4/13/05, pp. 1–2: `cf:rms_delay_mod_inst_1642721247872.pdf`
12. Cesaroni FAQ, "Delay Adjustment": <https://pro38.com/frequently-asked-questions/>
13. US 7,918,419 B2 (RCS), "Rocket ejection delay apparatus": <https://patents.google.com/patent/US7918419B2>
14. NAR S&T, "Combined CAR/NAR/TRA Certified Rocket Motors List", 2024-07-31, p. 2: <https://www.nar.org/wp-content/
    uploads/2024/04/CombinedMotorsByImpulse.pdf> (Internet Archive, 2024-12-02)
15. Estes D12-5 page: <https://estesrockets.com/products/d12-5-engines>
16. M. Nakano, H. Mashidori, Y. Takahashi, "Effects of Temperature and Aging on Performance of A8-3 Model Rocket
    Engines", TMCIT bulletin 7 (2013): <https://metro-cit.repo.nii.ac.jp/record/153/files/KJ00008928239.pdf>
17. AeroTech HP-G138T-14A instructions, P/N 20036, 8/11/10: `cf:hp_g138t_14a_instructions_1642627436648.pdf`
18. AeroTech RMS-38/360-1320 Plus instructions, 10/22/20, `cf:` file `rms-38_360-1320%20plus%202%20pg%20instr%20` +
    `10-22-20_1656446777628.pdf`
19. AeroTech delay drilling tool pages (18–38 mm RDDT, 54 mm RDDT, DMS UDDT): <https://aerotech-rocketry.com/>
20. AeroTech Master Motor Matrix, 4/2/23:
    <https://cdn.shopify.com/s/files/1/0615/7592/1878/files/AeroTech_Master_Motor_Matrix_4-2-23.pdf>
21. Cesaroni Pro24, Pro29, Pro38, Pro54 instructions v4.00, e.g. `p38:CTI-AERO-MAN-103-v4.00-Pro29-Instructions_EN.pdf`
22. Loki Research 38 mm Blue instructions, rev. C, 2013: <https://www.lokiresearch.com/images/Documents/38mm%20Blue.pdf>
23. "BP motor consistency", posts #31 to #32, 2021-05: `trf:threads/bp-motor-consistency.166006/page-2`
24. NAR R&D report index: <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>
