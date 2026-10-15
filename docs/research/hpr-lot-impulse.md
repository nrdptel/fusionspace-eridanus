# Do high-power production lots deliver their certified impulse?

This note asks whether anyone has published a measurement of a high-power motor (H and up) against its certified
or catalog impulse: a certifier's repeat test, a maker's lot data, or an independent static fire. It follows [motor
data](motor-data.md) §5 (NAR's 2023–26 retests of 17 motors, F and smaller, a median 9.4% below their certification;
high power "unbounded") and feeds [apogee bias](apogee-bias.md) §1. Read 2026-10-15; **Inference** marks this note's
arithmetic. Trust: the certifier pairs are all found in the archived TMT paths searched, but old; the independent
firings are few, none from a stand with a documented calibration.

**In short.** No certifier has published a repeat test, with numbers, of an unchanged high-power motor since 2004.
- **Certifiers: one historical set.** TMT's 2001–2004 recertification round, kept in Internet Archive copies of
  tripoli.org, measured 22 AeroTech H–M motors a second time. Impulse moved a median **−2.7%** (range −8.5% to
  +9.5%, SD 4.4 points); 17 of 22 fell (§2). ThrustCurve carries the later figure on 19 of the 22, so these pairs
  size how far one certification test sits from another, not a bias in today's catalog.
- **Since 2004, no certifier repeat with numbers** but two redesigns (§3). NAR's 18 published retests are all F and
  smaller; it re-fired six AMW reloads (K to N) in June 2026 and reported only "within the allowable performance
  variances". CAR's index and Cesaroni's pages show one test per motor, the latest in 2014. No maker publishes lot
  testing.
- **Independent static fires: 11 firings of 6 H–M motor types, from 3 sources** (§4). Median **−6.8%** (−21.9% to
  +1.4%; 10 of 11 low), or **−3.5%** over the 8 whose authors did not doubt their stand. Hobby stands read low by
  construction (friction, misalignment), and none of these documents a calibration, so this is a sign, not a size.
  ThrustCurve's "user" curves are transcriptions, not static tests: none of the 476 H-and-up ones claims a firing.
- **Bound (Inference, §5).** A high-power lot gap of 0 to −4% is consistent with everything found; a gap the size
  of NAR's small-motor −9.4% is neither shown nor ruled out. At 0.6–1.3% of apogee per 1% of impulse, 0 to −4%
  makes a simulator flying the catalog read 0–5% high. "High power unbounded" becomes "likely low, size unmeasured".

## 1. What the rules require of repeat tests

- **NFPA 1125's 2021 revision** of §8 shows no recertification interval or lot sampling. It asks for two or more
  samples at first test, an impulse SD within 6.7% of the mean, and the measured mean as the printed impulse [N1,
  FR-8, p. 7]. A 2019 public input on recalls notes that lists "do not identify manufacturing lots" [N2, PI-6].
- **NAR:** 5-year term; recertification fires 2 motors per delay up to 5,120 N·s, 1 above [M1, §3.3.3, §3.3.5, §6.2,
  pp. 7, 11]. One recert firing more than 13.1% from the original mean forces a full retest [M1, p. 11], so a passed
  recert bounds each firing to ±13.1%, no tighter (**Inference**).
- **CAR:** 3-year term up to L, 5 years for M–O; 2 motors per delay up to L, 1 above [C1, §4.3, §7.2, pp. 9, 13].
- **TMT:** 5-year term, extendable 5 years untested; H–L retests fire at least 3 motors [T1, §6.4.1.6, §10.2.1,
  §10.5.2], bought at retail or supplied by the maker [T1, §11.2]: a retest need not sample production.
- **What's published.** NAR's live list (dated 2026-08-12) marks 18 retest sheets; the largest is 61.38 N·s, an F
  [W2]. Of the 49 TRA letters on ThrustCurve, all 11 dated 1998–2010 print an expiry; none of the 38 dated
  2014–2025 does [S1]. Whether TMT now retests or extends is not published.

## 2. TMT's 2001–2004 round: 22 AeroTech pairs

Archived TMT letters (178 PDF versions) and class lists (172 captures, 1999–2021; 135 with a table, 2002–2012) were
read [A1, A2]. A pair counts when a designation carries two impulses and a new test is shown: two letters, or a list
entry whose expiry moves forward with a new impulse. H70W, whose expiry moved with unchanged numbers, is left out: an
extension and an identical result look alike. Percentages are (later − earlier)/earlier.

| Motor | Earlier, N·s | Later, N·s (TMT test) | Change | Motor | Earlier, N·s | Later, N·s | Change |
|---|---:|---:|---:|---|---:|---:|---:|
| H123W | 235.6 (1999) | 223.2 (2003) | −5.26% | J415W | 1,295.2 | 1,231.7 (2003) | −4.90% |
| H242T | 253.3 (1999) | 231.7 (2003) | −8.53% | J460T | 855.0 | 805.5 (list) | −5.79% |
| H238T | 167.8 (2000) | 165.5 (2004) | −1.37% | J570W | 1,042.5 | 973.1 (2003) | −6.66% |
| I195J | 426.5 (1999) | 426.1 (2004) | −0.09% | J800T | 1,278.0 | 1,294.7 (list) | +1.31% |
| L1120W → M1297W | 4,946.5 (1998) | 5,416.6 (2004) | +9.50% | K250W | 2,555.0 | 2,484.0 (2001) | −2.78% |
| H128W | 166.2 | 172.9 (2003) | +4.03% | K560W | 2,519.5 | 2,417.0 (list) | −4.07% |
| H180W | 217.8 | 217.7 (2003) | −0.05% | K700W | 2,319.5 | 2,261.0 (2001) | −2.52% |
| I161W | 335.8 | 328.7 (2003) | −2.11% | K1100T | 1,599.0 | 1,472.0 (2003) | −7.94% |
| I211W | 434.5 | 441.6 (2003) | +1.63% | L850W | 3,736.4 | 3,646.2 (2003) | −2.41% |
| I357T | 360.4 | 342.0 (2003) | −5.11% | J275W | 840.3 | 774.0 (2001) | −7.89% |
| I435T | 565.0 | 568.9 (2004) | +0.69% | H125W | 322.0 | 299.4 (list) | −7.02% |

Years in brackets are test dates from letters; "list" means only a class list shows the new figure. Earlier values
without a year come from lists with no test date: the 5 dated pairs are 4–6 years apart, the other 17 unknown.

- **Size (Inference):** n = 22, median −2.65%, mean −2.61%, SD 4.41 points, quartiles −5.66% and −0.06%; 17 fell
  (sign test, two-sided p = 0.017), 8 by more than 5%, 6 within ±2%. Without the renamed L1120W: n = 21, median
  −2.78%, SD 3.56. If both tests err alike and independently, one test's scatter is SD/√2: 2.5–3.1% (1σ).
- **Not a clean lot comparison.** Some products changed between tests: I195J kept its impulse but its peak thrust
  halved (452.1 → 216.3 N) and its declared propellant fell 301.0 → 272.1 g; H123W's fell 128.7 → 119 g; L1120W
  became M1297W with 2% less propellant and 9.5% more impulse [A1]. H125W kept its average thrust and burn time, so
  its change may be a correction, not a test [A2]. How many motors each test fired is not printed.
- **Site and stand.** Letterheads moved from Sparks, NV (tests to March 2001) to Glendale, AZ (May 2001 on) [A1]; places
  are unknown ([TMT test sites](tmt-test-sites.md) §4). If the later stand sat about 1,000 m lower, uncorrected, the
  nozzle's pressure term alone reads it 0.5–2.2% low (**Inference**, from that note's §6). The 2004 calibration was
  questioned: checked with about 75 lb against peaks over 400 lb; TMT replied that the day's other motors matched
  their earlier data [F1, posts 1, 9, 11].
- **What the catalog carries.** ThrustCurve gives the later value for 19 of the 22, to within 0.2% (H123W 223.6);
  J800T and K250W show a third value (1,229.11 and 2,353 N·s); L1120W is out of production at 4,947 [S2].

## 3. Other certifier repeats, and what isn't public

| Case | Earlier, N·s | Later, N·s | Change | Kind |
|---|---|---|---:|---|
| AeroTech H220T | NAR 1996, 215.41 [S3] | TMT 2004, 199.7 [A1] | −7.29% | two certifiers |
| AeroTech I65W | TMT 1997, 665.2 [A1] | NAR 2001, 630.50 [S4] | −5.22% | two certifiers, 10% less propellant |
| AeroTech K1050W | TMT 1998, 2,530.3 [S5] | TMT 2008, 2,426.35 [S6] | −4.11% | shorter grain, 7% less propellant |
| AeroTech O5280X → O5500X-PS | TMT 2019, 22,463 [F2, T2] | TMT 2023, 21,384 [S7] | −4.80% | redesigned case |
| KBA M2900R → M3500R | NAR 2007, 5,290 [S8] | NAR 2008, 7,310 [S9] | +38.2% | stand fault [R1] |

- **H220T reads high in the catalog.** ThrustCurve lists 220 N·s, the NAR figure [S2]; TMT measured 199.7 in 2004,
  with 102 g of propellant against 106.4 [A1]. If TMT's is current, the catalog is 10.2% high (**Inference**).
- **AMW, 2026.** NAR S&T fired AMW K1000SK, L1300BB, M1350WW, M1900BB, N2700BB and N4000BB on 2026-06-24 and said
  new and original production were "within the allowable performance variances" of NFPA 1125 [F3]; numbers need a
  forum login. **Inference:** under NAR's rule each firing was within ±13.1% of its original mean, no tighter.
- **CAR and Cesaroni.** CAR's index (256 rows, 2002–2014) has no recertification column [S10]; Cesaroni's 284 reload
  pages give one test date each, 2001–2014 [S11]. Every one up to L was due again by 2017 (**Inference**).
- **Makers.** No maker publishes lot testing; TMT may ask for "internal quality control data" [T1, §10.5.1].

## 4. Independent static fires

- **ThrustCurve's user curves are not tests.** The site defines a user file as one made "from a printed thrust curve
  or other data" [B1]. Of 1,252 H–O curve files (2026-09-17 capture), 476 are user files (379 motors); none claims
  the uploader's own test by its header text. Their impulse sits a median 0.00% from the catalog (p10 and p90
  ±2.0%): digitizing error. Of three raw stand exports, two match a certification within 0.1%; the third (KBA
  M2900R, +3.2%) doesn't say who fired it [B2].
- **Université libre de Bruxelles students, EUCASS 2025** [B3, pp. 5–8, Table 3]: Cesaroni 540I470-15A,
  4 firings, 518.91–526.12 N·s, mean 523.59 against 540.10, **−3.06%** (−3.92% to −2.59%). Its 2412K1085-P,
  3 firings, 2,071.50 / 2,182.30 / 2,247.93 against 2,412: **−10.15%** (−14.12% to −6.80%). The paper describes
  a 10 kN load cell on a sliding plate but no calibration; it blames the K shortfall on age near expiry and notes a
  grain defect in one motor. It calls the I result a validation of its stand, so selection bias applies.
- **Western Michigan University thesis, 2023** [B5, Tables V–VIII]: AeroTech J425R, J1799N, L2200G, one firing
  each, **−21.9%, −20.7%, −20.8%** against ThrustCurve. The load cell's span was set with 1 and 10 kg weights
  against peaks up to 2,727 N; the authors blame calibration or clamp friction (**a stand fault**). The J1799N reads
  −9.6% against the thesis's reference, 1,066 N·s from AeroTech's page [B6], itself 12.25% below ThrustCurve's
  1,214.8 TRA figure [S2].
- **A builder's AeroTech M1850W at FAR, 2024** [B7]: 7,767.6 N·s against 7,658.6, **+1.42%**, in 49 °C air with
  the motor estimated at 65–77 °C and a non-standard igniter; the stand is "likely calibrated well", no method given.
- **Below H, for scale: University of Alabama in Huntsville, 2022** [B10, pp. 5–16]: four AeroTech G138T reloads
  from two retailers on a stand recalibrated daily with several weights: 32.1–32.9 lbf·s against 35.32 (157.1 N·s),
  **−9.12% to −6.85%, mean −7.77%**, like NAR's small-motor retests. The only multi-point-calibrated stand found.

## 5. The gap, sized

| Set | Firings or pairs | Median | Range | Low |
|---|---|---:|---|---:|
| TMT repeat tests, AeroTech H–M, 2001–04 (§2) | 22 pairs | −2.7% | −8.5% to +9.5% | 17 of 22 |
| Cross-certifier and redesign pairs (§3, without KBA) | 4 pairs | −5.0% | −7.3% to −4.1% | 4 of 4 |
| Independent H–M firings, all (§4) | 11 (6 types) | −6.8% | −21.9% to +1.4% | 10 of 11 |
| Independent, without the doubted WMU stand | 8 (3 types) | −3.5% | −14.1% to +1.4% | 7 of 8 |
| G138T, calibrated stand (below H) | 4 | −7.6% | −9.1% to −6.9% | 4 of 4 |

**Inference** from the rows:
- **The median is negative in every set,** but each has a possible cause besides the propellant: test site (§2, sign
  unknown), product changes (§2, §3), stand friction and calibration (§4). No source separates them.
- **No calibrated measurement compares today's production with today's catalog.** The TMT pairs are 20+ years old,
  and the catalog carries their later figure; the recent firings are uncalibrated. 0 to −4% fits the data, and −9%
  is not excluded. One certification test's own scatter is about 2.5–3% (1σ, from the TMT pairs).
- **What it moves.** At 0.6–1.3% of apogee per 1% of impulse ([apogee bias](apogee-bias.md), table), a −2.65% median
  is 1.6–3.4% of apogee, high; 0 to −4% is 0–5%. That is up to half of the +9.8% bias, not all of it.

## What this suggests

- **Motor data §5 can replace "high power unbounded"** with §5's table here; no printed number changes.
- **A Monte Carlo impulse spread** could take 2.5–3% (1σ, one certification test's scatter from the TMT pairs)
  rather than NFPA's 6.7% ceiling, leaving the sign of lot drift a stated unknown.
- **A catalog check** could flag a motor whose catalog impulse exceeds a later certifier figure: H220T (+10.2%), the
  N2700W-PS transposed digit ([motor data](motor-data.md) §5) and NAR's 17 small motors. The J1799N
  maker-page/certification mismatch (−12.25%) is a different case: which figure is current is unknown.
- **A measurement that would settle it:** fire 2–3 current-production reloads of one common H–K motor on a stand
  calibrated across its range with dead weights, at a known pressure, and compare with the catalog.

## Open questions

- The AMW 2026 retest figures (behind a forum login); whether CAR or TMT has retested any motor since 2014.
- Where TMT fired in 1998–2004, and how many motors each 2001–04 test fired.
- Whether the EUCASS stand was calibrated; who fired the KBA M2900R raw file; the M1850W post's load cell.
- Which J1799N impulse AeroTech ships today: its page's 1,066 N·s or the TRA figure on ThrustCurve, 1,214.8.

## Sources

`ia:` is `https://web.archive.org/web/<timestamp>id_/`; `trf:` is `https://www.rocketryforum.com/threads/`; `tc:`
is `https://www.thrustcurve.org/motors/cert/`. Read 2026-10-15 unless marked; cached under `refs/`, never committed;
none grants a license.

1. **N1, N2** NFPA 1125 A2021 First Revisions and Public Input responses, `https://docinfofiles.nfpa.org/files/`
   + `AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf`, `..._PIResponses.pdf`. **M1** NAR Motor Testing
   Manual v1.5 (2011), `https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf`.
2. **C1** CAR Motor Test Manual r20181121, `https://car-acf.org/resources/Documents/` +
   `car_motor_testing_manual%20r20181121.pdf`. **T1** TMT Policies and Procedures, rev. 2020-01-06,
   `ia:20201128164829` + `http://www.tripoli.org/Portals/1/Documents/TMT/Resources/` +
   `TMT%20Policies%20and%20Procedures%20-%2001-06-2020.pdf`. **T2** Tripoli's public TMT notice index, entry of
   2019-12-07, "O5280X" (document 886772).
3. **A1, A2** Archived TMT letters, `ia:` + `http://www.tripoli.org/tmt/new_motors/` and `/motors/new_motors/`
   (e.g. `AT%20H242T%20dec%2008.pdf`), found through the Wayback CDX API; class lists, `ia:` +
   `http://www.tripoli.org/tmt/HMotors.shtml` (and I–M), `/motors/Hmotors.htm`, `/CertifiedHMotors/tabid/171/`.
4. **R1** NAR S&T release R124 (2008-08-29), `ia:20091103214154` + `http://www.nar.org/SandT/releases/R124.html`.
   **W2** NAR S&T Certified Motor Listing, "Last Updated: 2026 August 12", `https://www.nar.org/content.aspx?` +
   `page_id=22&club_id=114127&module_id=670148`.
5. **S1** The 49 TRA letters on ThrustCurve ([motor data](motor-data.md) S2, read 2026-10-11). **S2** ThrustCurve
   search API, `https://www.thrustcurve.org/api/v1/search.json`, snapshot 2026-10-11.
6. **S3–S9** Certification documents: H220T (NAR, 1996) `tc:62d81491ac50e90004732e3b/`; I65W (NAR, 2001)
   `tc:60c63bfdb5bc370004713eb3/`; K1050W (TRA, 1998, 2008) `tc:6796873d6ae3120002d42344/`,
   `tc:679687c86ae3120002d42401/`; O5500X-PS (TRA, 2023) `tc:63bed7ba5efa3f0004cc2e79/`; KBA M2900R and M3500R
   (NAR) `https://www.nar.org/docs.ashx?id=1481464` and `id=1481465`.
7. **S10** CAR Certified Motor Index, linked from `https://car-acf.org/en/certifications/motor-certifications`;
   **S11** Cesaroni reload pages, `https://pro38.com/product-category/pro-54/` and siblings (both read 2026-10-11).
8. **F1–F3** `trf:rol-news-aerotech-l1120w-p-motor-recertified-as-m1297w-p.78176/`, posts 1, 9, 11 (2004);
   `trf:recertification-of-a-re-designed-motor.176952/`, post 1 (2023); `trf:nar-s-t-notification-amw-2026-08-20.`
   + `198601/`, post 1, and `trf:amw-motor-testing.197044/`, posts 16, 27.
9. **B1, B2** ThrustCurve, `https://www.thrustcurve.org/info/contribute.html`; API capture of 2026-09-17 and simfile
   pages, e.g. `https://www.thrustcurve.org/simfiles/5f4294d20002e90000000553/`.
10. **B3** Lecomte, Gelain, Di Matteo, Hendrick, "Experimental Investigation of Solid Rocket Motors for a Student
    Sounding Rocket", EUCASS 2025, DOI 10.13009/EUCASS2025-024.
11. **B5** J. Wine, "High Power Rocket Motor Test Bench", WMU Honors Thesis 3685 (2023),
    `https://scholarworks.wmich.edu/honors_theses/3685`; results pages are scans, read by OCR.
12. **B6** AeroTech, J1799N-P reload page, `https://aerotech-rocketry.com/products/` +
    `product_6c181381-9d6a-db54-918f-06be861f93bc`. **B7** `trf:aerotech-open-thread.125657/page-277`, 2024-07-06.
13. **B10** M. Amara, "Performance Variability in Solid Rocket Motors", UAH Honors Capstone 683 (2022),
    `https://louis.uah.edu/cgi/viewcontent.cgi?article=1682&context=honors-capstones`.
