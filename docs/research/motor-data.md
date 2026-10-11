# What motor makers and certifiers publish beyond the thrust curve

This note counts what the makers of COTS solid motors and the three certifiers (NAR, Tripoli's TMT, CAR) publish
per motor besides the thrust curve: nozzle throat and exit, grains, case and hardware masses, delays, and measured
impulse against the nominal. It backs the thrust-at-altitude idea ([ideas](ideas.md)), motor mass properties
during the burn (`hpr-motor/src/grains.rs`), and the impulse part of [apogee-bias](apogee-bias.md) §1. It builds on
[thrustcurve-data](thrustcurve-data.md). Read 2026-10-11. Counts are censuses or text searches over PDF text.

**In short.** A nozzle exit diameter is published for almost no motor. Certifiers give one for 2 of ThrustCurve's
1,004 solid motors, both on 1990s letters. AeroTech's drawings give one on 15 of 376, all 98 mm. AeroTech, through
its parent company RCS, publishes far more than anyone else: grain count and size on 374 drawings, a throat on about
302, and hardware masses for 34 case sizes. A geometric bound, the bore of the nozzle cap or the aft closure's
diameter, cuts the area of today's "exit ≤ case" bound to 0.17–0.58 of it. Certified impulse matches ThrustCurve's
within 0.5% on 47 of 49 TRA letters; one catalog entry carries a single firing, +3.05%. No source grants a license.
Cite the facts; never commit the documents.

## 1. What each source gives per motor

| Source (documents) | Nozzle | Grains | Masses | Impulse |
|---|---|---|---|---|
| AeroTech RMS drawings (325) [M1] | throat ~254, exit 14 | N, OD × length 324; core 27 | none | none |
| AeroTech DMS drawings (51) [M2] | throat 48, exit 1 | N, OD × length 50; core 32 | none | none |
| AeroTech instructions (214) [M3] | who owns it | none | hardware, 34 sizes | none |
| AeroTech matrix (362 motors) [M15] | none | none | propellant, loaded | certified |
| Cesaroni pages (284) [C7] | none | none | loaded, propellant, burnout | certified |
| Loki table, 30 sheets [L1, L2] | throat by nozzle number, 33 | typical per case | loaded, propellant | nominal |
| AMW 2003–07, Gorilla [A2, G1] | graphite, hardware | AMW: count, length | Gorilla: propellant, total | nominal |
| NAR sheets (141) [S2] | none | none | loaded, propellant, burnout | mean, σ |
| TRA letters (49) [S2] | throat 5, exit 2 (to 2001) | none | loaded; burnout 2018+ | mean, ± |
| CAR index (256 rows) [S7] | none | none | loaded, propellant, burnout | 5% window |

How counted: all 325 + 51 drawings and 214 sheets downloaded and searched by a text rule on the bill-of-materials
lines; every one of 284 Cesaroni pages fetched; all 1,004 ThrustCurve certification pages crawled 3 s apart, as its
robots.txt asks, giving **190 documents for 184 motors** (141 NAR, 49 TRA, 0 CAR) [S2]. By group, motors with a
document: NAR-certified 135/202, TRA 49/466, CAR 0/293 (no Cesaroni motor has one); high power in production 49/521.
nar.org and tripoli.org refuse scripted fetches, so both were read through Internet Archive snapshots [S3, S5].

## 2. Nozzle exits

- **Certifiers: 2 of 1,004 motors (0.2%).** TMT letters of 1998–2001 carried throat and "exit cone" rows; TRA
  dropped them by 2006, NAR sheets never had them, CAR's index has no such column [S2, S7]. Five letters give a
  throat; two of those (K1050W, M1315W) list seven Medusa orifices, not a throat and an exit. The usable exits:
  L1500T 1.4 in (throat 0.734 in) [S10] and I195J 0.416 in (throat 0.182 in, area ratio 5.22) [S11]. The I195J
  in today's catalog peaks at 216 N against the letter's 452 N, so the 1999 nozzle may not be today's.
- **AeroTech drawings: 15 of 376.** Thirteen RMS-98 reloads and the M1340W DMS at 1.75 in; the N2220DM at 2.74 in
  [M1, M2]. Cesaroni, Loki, AMW and Gorilla publish no exit. One RCS store page gives a whole nozzle: the 01500 part,
  0.180 in throat, 0.438 in exit, 1.000 in OD, 8 g [M16]. **Inference:** drilling widens the throat, not the exit, so
  its area ratio falls from 5.9 molded to 3.7 when drilled to 0.228 in.
- **The L1500T's two exits disagree.** Its drawing gives 1.75 in [M5]; its 1998 letter 1.4 in [S10]; both give a
  0.734 in throat; the drawing's 2007 revision renamed the nozzle part. [Apogee-bias](apogee-bias.md) used 1.4 in
  (+0.99% of average thrust at 1,300 m); the drawing gives +1.55% (**Inference**, ISA, Δp = 14.7 kPa).
- **A tighter bound from parts that sit over the exit (Inference).** 260 of 325 RMS drawings (none at 98 mm)
  list the plastic cap that slips over the nozzle's aft end, by bore [M1]; the exit must fit inside it. Against
  the case cross-section that bound's area is 0.28 (24 mm), 0.20–0.52 (29 mm, three caps), 0.17–0.29 (38 mm),
  0.50 (54 mm) and 0.58 (75 mm). The 01500 nozzle passes: 0.438 in exit under a 0.625 in cap. For Cesaroni the aft
  closure or nozzle boss on its dimension drawings caps the exit: Pro29 1.00 in, Pro54 1.25 in, Pro75 XL 2.200 in,
  Pro98 2.05 in (XL 2.70 in), Pro150 4.250 in [C6]; Pro24 and Pro38 show none.
- **Worked example, AeroTech N3300R-P (98 mm RMS) (Inference).** Drawing: throat 1.0 in, exit 1.75 in, area
  ratio 3.06, six grains 3.37 × 6.00 in [M4]. At a 1,300 m field (Δp = 14.7 kPa) the drawn exit adds
  14.7 kPa × 15.5 cm² = 22.8 N, 0.72% of the 3,168 N average thrust; the case bound gives 111.6 N (3.5%), 4.9 times.

## 3. Who owns the nozzle

| Product line | Nozzle | Source |
|---|---|---|
| AeroTech RMS, 18–98 mm | single-use, in the reload kit; discarded with the liner | [M8, p. 1; M11] |
| AeroTech DMS, LMS, single-use | part of the disposable motor | [M7; M13] |
| Kosdon by AeroTech (KBA) | reusable graphite, hardware; throats 0.221–0.928 in on its 6 sheets | [M19] |
| Cesaroni Pro24–Pro54 | in the preassembled reload (rear closure and nozzle) | [C2, p. 3] |
| Cesaroni Pro75, Pro98 | in the kit; the nozzle holder is hardware | [C3, p. 3; C4] |
| Loki 29–76 mm | reusable graphite, hardware; engraved with the throat in 64ths of an inch | [L1, L3] |
| Loki 54/2800, 54/4000; 98 mm | single-use, in the kit (graphite insert; 98 mm uses Cesaroni's) | [L4, L5] |
| AMW, Gorilla | reusable graphite, hardware | [A3, G2] |

In single-use lines (AeroTech RMS, every Cesaroni line) the nozzle goes with the reload's designation; in graphite
lines with the case, though Loki's nozzle number still sets each reload's throat.

## 4. Grains and masses

- **Grains: makers only.** No certifier publishes grain count or size (0 of 190 documents; none in CAR's index).
  AeroTech drawings give N and OD × length on 374 (about 240 motors), the core on 59 [M1, M2]. Loki gives a
  typical BATES set per case: three 3.25 in grains, 5/8 in cores, for its 12.875 in 54 mm case [L1]. Cesaroni's
  "nG" names are length classes: a Pro75-4G may hold four half-length grains [C3, p. 12].
- **Hardware mass: AeroTech only.** Instruction sheets give 34 sizes, e.g. RMS-29/180 84.7 g [M10, p. 2], RMS-54/852
  278 g [M9, p. 2], RMS-98/15360 3,204 g [M8, p. 2]; some sizes have two values (38/360: 147.4 and 166 g), likely
  two generations. Nobody splits case, closures and nozzle.
- **Loaded and burnout mass include the hardware for reloads**, on all three certifiers (NAR's AMW K475 sheet,
  TRA's N4000W-PS letter, CAR's index) [S8, S16, S7].
- **Mass lost beyond the propellant**, (loaded − burnout − propellant)/propellant: median +1.2% on NAR sheets
  (high power, n = 45, p10 −1.4%, p90 +7.4%), +4.7% in CAR's index (above 160 N·s, n = 223), +6.6% on TRA
  2018–24 letters (n = 19, −10% to +96%) [S8, S7, S2]. Cesaroni's pages: median 26.9 g over 259, 7 negative [C7].
- **Propellant mass is the maker's declared number.** 26 of 35 recent TRA letters mark it "provided by
  manufacturer" [S2]; NAR weighs the motor before and after, not the propellant [S17, §8.3.7]. The four 2025 TRA
  letters lose exactly zero beyond it. **Inference:** their burnout mass was computed, not weighed.
- **Sources disagree on one motor's propellant.** N3300R: 7,338 g on the instruction sheet, 7,512 g in the matrix,
  the TRA letter and ThrustCurve [M8, M15, S23]. ThrustCurve's propellant mass is CAR's burnout mass on 4 Cesaroni
  motors (41F36-11A: 69 g against 29.5 g) [S1, S7].

## 5. Measured impulse against the nominal

- **TRA, a census of 49 letters:** ThrustCurve's impulse is within 0.5% of the letter's mean on 47 [S1, S2], up from
  the 4 letters [apogee-bias](apogee-bias.md) read. One exception is a superseded F41W letter. The other is real:
  the 2024 letter for "M2700W-PS" fired two motors, printed as 10,637 and 9,969.7 N·s, and certifies 10,322 ("N2717").
  Its lbf·s column converts to 10,673 and 9,969.7, mean 10,322, so 10,637 looks like a transposed digit.
  ThrustCurve lists N2700W-PS at 10,637 N·s, the first firing as printed: **+3.05%, sim high** [S16].
- **Spreads:** 24 TRA letters give "mean ± x" with x undefined, median 2.4% of the mean; one letter's lbf·s and
  N·s spreads disagree (J1265T) [S2]. NAR spreads are σ, median σ/mean 0.63% ([apogee-bias](apogee-bias.md)).
- **Cesaroni and CAR:** a Cesaroni name's leading number is within 0.5% of CAR's measured impulse on 239 of 246 rows
  [S7], and within 1% of the reload page's value on all 237 that parsed [C7]: the pages repeat the certification.
  CAR integrates between the 5% points [S18, §12.1.3]; the tail outside is a median 0.13% of impulse
  ([thrustcurve-data](thrustcurve-data.md) §2), so CAR figures read low by about that (**Inference**).
- **Makers publish no per-lot data.** The matrix's N3300R is 14,041 N·s, as ThrustCurve; its letter (tested 2009)
  measured 14,091 (+0.36%) [M15, S23]. Production lots against the certification mean stay unbounded.

## 6. Delays

Measured delays run slightly early. NAR, 210 delays on 86 sheets: median −0.06 s, relative p10 −16%, p90 +11% [S8].
TRA, 121 firings in 13 letters: median −0.09 s, p10 −16%, p90 +14% [S2]. Composite delays can burn more slowly at
sites thousands of feet up [S3, p. 2]. AeroTech's tool removes 4 or 8 s, 2 s less with its washer [M14, p. 1];
Cesaroni's Pro54 tool up to 10 s [C2, p. 2]; Loki offers five delay lengths per reload [L2].

## 7. Where the test was fired

NFPA 1125 tests "at or corrected to sea level and 20°C ± 5°C" [S19, FR-8, p. 7]. NAR's manual keeps sites within
500 ft of sea level [S17, §1.5]; 19 of 157 cached NAR sheets state 20–500 ft and 17–23 °C [S8]. One breaks it:
the Estes L2350 (2021) was fired at 5,276 ft and "corrected to sea level", method unstated [S20]. **No TRA letter
(0 of 49) and nothing CAR publishes states the site or its conditions.** An uncorrected curve from 1,500 m would
read high at sea level by (p₀ − p)·A_e: +1.13% of average thrust for the L1500T's 1.4 in exit, +0.58% for the
I195J, both against the letters' measured averages (**Inference**, ISA). At NAR's 500 ft limit it is at most +0.12%.

## 8. Terms

RCS reserves all rights and grants no license [M17]; Cesaroni allows personal, non-commercial download only [C9];
Loki asks that nothing be reproduced without permission [L2]. 136 of 141 NAR sheets carry an NAR copyright line;
TRA letters carry none, and Tripoli says its data is public through thrustcurve.org [S5]. CAR's index is a public
view with no license [S7]. ThrustCurve's robots.txt asks for 3 s between fetches. **None grants
redistribution:** facts restated with a citation are fine; a committed table would be re-entered by hand with
sources, and the PDFs stay under `refs/`.

## What this suggests

- **Thrust at altitude:** replace "exit ≤ case" with the cap-bore bound for AeroTech RMS and the aft-boss bound for
  Cesaroni, and a drawn exit where one exists. The bound's term shrinks about 2–6 times (**Inference**, the area
  ratios in §2). Keep both L1500T exits visible until one is confirmed.
- **Grains (`grains.rs`):** N, OD and length for about 240 AeroTech motors and Loki's typical cases; the core and
  the density still come from mass. Cesaroni has no published geometry.
- **Mass:** AeroTech hardware masses split loaded mass into case and reload; burnout mass is measured, propellant
  mass declared. A check that the catalog impulse equals a certifier's mean, not one firing, would flag the
  N2700W-PS entry.

## Open questions

- Which L1500T exit is current, 1.75 in or 1.4 in; whether RCS's other nozzle pages (not listable) give exits.
- Where TMT fires and whether it corrects to sea level (unbounded for TRA curves); what TRA's "±" means; how NAR
  corrected the L2350.
- Whether Cesaroni's 2001–2014 page masses describe current reloads; AMW (site down), Ellis Mountain (refused the
  connection), Estes and Quest were not surveyed for drawings.

## Sources

All read 2026-10-11; cached under `refs/research/motor-data/` except M16 (live), S8 (`refs/research/apogee-bias/`)
and S17, S19 (`refs/papers/`). Terms in §8. RCS files are at
`https://d3l66gvjdr7rqw.cloudfront.net/Templates/170652/myimages/` (shown as `cf:`); ThrustCurve documents at
`https://www.thrustcurve.org/motors/cert/` (shown as `tc:`).

- **M1–M3** RCS (<https://www.rocketmotorparts.com/>) RMS drawings `page/motor-assembly-drawings`, DMS designs
  `index.aspx?pageid=7452840`, instructions `page/instructions`
- **M4, M5** N3300R-P drawing, rev B 2009-11-02, `cf:98-15360_n3300r-p_assy_1648897918405.pdf`; L1500T-P
  drawing (nozzle note 4/11/07), `cf:98-5120_l1500t-p_assy_1648849442531.pdf`; **M7** `cf:k400c-14a_rcs_assembly.pdf`
- **M8–M14** instruction sheets listed on M3: N3300R-P (P/N 20093), RMS-54/852-1706, RMS-29/180-240,
  RMS-98 2560–20480, 29–54 mm DMS (P/N 21049), RMS-38/360-1320 Plus
- **M15** AeroTech Master Motor Matrix, 4/2/23:
  <https://cdn.shopify.com/s/files/1/0615/7592/1878/files/AeroTech_Master_Motor_Matrix_4-2-23.pdf>
- **M16, M17** RCS nozzle 01500 <https://www.rocketmotorparts.com/details/p1577809_7762949.aspx>; terms of use §11
  <https://www.rocketmotorparts.com/storepage1577807.aspx>
- **M19** Kosdon-by-AeroTech sheets on M3, e.g. `cf:75-7600_kba_ac_in_k08151n_1642721053837.pdf`
- **C2–C5** Cesaroni Pro54, Pro75, Pro98 and Pro24 instructions, on <https://pro38.com/resources/>
- **C6, C7, C9** Cesaroni dimension drawings (e.g. `pro38.com/wp-content/uploads/2024/11/Pro54_dimensions.pdf`),
  reload pages (<https://pro38.com/product-category/pro-54/> and siblings), <https://pro38.com/legal-conditions/>
- **L1** Loki Tech Info: <https://www.lokiresearch.com/page/Tech_Info>; **L2** Documents:
  <https://www.lokiresearch.com/page/Documents>; **L3, L4** 54 mm Red and Red 2800 sheets on L2; **L5** Loki store
- **A2, A3** AMW PDFs, 2003–2007, via `web.archive.org`; **G1, G2** <https://gorillarocketmotors.com/reloads/>,
  `/hardware/`
- **S1** ThrustCurve search API, 1,004 solid motors: <https://www.thrustcurve.org/api/v1/search.json>
- **S2** ThrustCurve certification pages (`/motors/<maker>/<designation>/cert.html`) and their 190 documents
- **S3** NAR S&T, Combined CAR/NAR/TRA Certified Rocket Motors List, 2024-07-31, via the Internet Archive:
  <https://www.nar.org/wp-content/uploads/2024/04/CombinedMotorsByImpulse.pdf>
- **S5, S7** Tripoli Motor Testing <https://www.tripoli.org/TMT> (snapshot 2026-02-15); CAR Certified Motor Index,
  linked from <https://car-acf.org/en/certifications/motor-certifications>
- **S8** 157 NAR data sheets cached by [apogee-bias](apogee-bias.md) (e.g. <http://nar.org/SandT/pdf/Animal/K365.pdf>)
- **S10, S11** TMT letters, L1500T 1998-12-31 `tc:6178f239caf91100040db736/l1500t-p_tra_cert.pdf`; I195J
  1999-09-26 (scan) `tc:65eb8c68aa7e85000263f5f9/IMG_6176.pdf`
- **S16** TMT letters 2022–25, e.g. M2700W-PS (2024) `tc:6623cfaef873440002ac6a68/`, N4000W-PS
  `tc:645072c42cdedd00028df9f4/`
- **S17, S18** NAR Motor Testing Manual v1.5 (2011),
  <https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf>; CAR Motor Test Manual r20181121,
  <https://car-acf.org/resources/Documents/car_motor_testing_manual%20r20181121.pdf>
- **S19** NFPA 1125 (A2021) First Revision statements, FR-8 (as [apogee-bias](apogee-bias.md) A1)
- **S20, S23** NAR sheet, Estes L2350, 2021-04-17, `tc:61ec5e53a1cd0a000491cffe/Estes%20L2350.pdf`; TMT letter,
  N3300R-P, 2010-01-03, `tc:6239d39422d46000044d09df/n3300r_p_tra_cert_1642718898263.pdf`
