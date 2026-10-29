# How far an H-and-up COTS motor's burn time varies between firings

What spread of burn time and average thrust, apart from impulse, do repeat firings of one high-power motor show, and
how much can grain temperature explain? It answers [dispersion sigmas](dispersion-sigmas.md)' open question and
serves M2.8's burn-time prior ([roadmap](../ROADMAP.md)), building on [HPR lot impulse](hpr-lot-impulse.md) §2,
[TMT vs maker curves](tmt-vs-maker-curves.md), [AMW retests](amw-retests.md) and
[propellant temperature](propellant-temperature.md). Read 2026-10-29: 84 NAR data sheets, 178 archived TMT
letters, ThrustCurve's files, two student test series, certifiers' manuals, two forum threads; no source code.
**Inference** marks this note's arithmetic. Trust: NAR's figures are measured on one lot in one session; TMT's span
years and two test sites, so they mix scatter with drift.

**In short.**
- **One lot, one session: burn time scatters 2% for a typical motor, 4% averaged over motors.** Over 84 NAR sheets
  for H-and-up solids (about 253 firings, 1996–2023), the median sheet's burn-time CV is 1.97% (quartiles
  1.14–3.42%); pooled, weighted by firings, it is 4.1% (95% 3.0–5.3%). Impulse on the same sheets: median 0.51%,
  pooled 1.1%. Burn time is the wider of the two on 74 of the 83 sheets with a nonzero burn-time σ (§2).
- **Tests years apart: burn time about 9%.** Re-reading 14 AeroTech H–L motors' repeat TMT tests from their own
  curves (16 test-to-test steps, propellant mass within 3.5%), one test's 1σ is burn time 9.2%, average thrust
  8.5%, impulse 3.8% (burn time 95% about 6.8–13.9%). Later tests ran shorter in 12 of 16 steps (median −4.4%). If
  the product changed (propellant mass moved over 3.5%), burn-time σ is 19.8%, impulse still near 4% (§3).
- **Burn time moves at nearly fixed impulse.** Within a session impulse scatters a quarter as much and the two
  correlate weakly (pooled −0.44, median −0.18 on sheets of three or more firings); across years r = 0.11 (16
  steps, 95% roughly −0.4 to +0.55). Average thrust, impulse over burn time, takes the change (§2, §3).
- **Temperature explains little of it.** Certification holds grains at 20 ± 5 °C, ±0.6–1.1% of burn time. A
  maker-posted HTPB/AP motor gives 2.0–2.4% per 10 K, at and above propellant-temperature's 1.2–2.2%; a 10 K
  test-to-test σ would explain 2–7% of the TMT variance, and the seasons run the wrong way (§4).
- **Hobby Monte Carlo burn-time σ of 1.5–19%:** 2–4% matches one lot, about 9% a catalog test against a motor
  bought years later; 19% matches only the worst single NAR sheet or a product change.

## 1. What certifiers print, and what burn time means on each

- **NAR** sheets print the certification firings' mean and σ of impulse, peak thrust, burn time and, on later
  sheets, average thrust [N2]; its manual asks σ of burn time, delay, impulse and average thrust [N4, §9.2.2,
  p. 20] at 20 ± 5 °C [N4, §8.3.3].
- **σ is printed two ways.** Where a sheet lists its firings, σ can be recomputed: 20 sheets divide by n (AMW N2700's
  two impulses, 11,516.02 and 11,388.62 N·s, give its printed 63.70, half their difference) [N2, id 1481411, p. 2];
  13 divide by n − 1 (AeroTech I49N's three firings give its printed 6.722) [N2, id 1481293, p. 2]. The other 51
  print σ alone. This note converts all known population σ to sample σ (×1.41 at n = 2, ×1.22 at n = 3).
- **NFPA 1125 caps impulse scatter only:** two or more samples at 20 ± 5 °C, impulse σ at most 6.7%, average
  thrust within 20%; no burn-time limit [N5, FR-8, §8.2.7, p. 7/9].
- **Burn time's definition moved.** NAR's manual reduces firings by NFPA 1125's protocols with ThrustCurveTool "or
  comparable software" [N4, §9.1, p. 19; §8.5.3, p. 18], whose burn time runs between the 5%-of-peak crossings
  [N6, pp. 8–9]; CAR uses the same window [N7, §12.1.1, p. 19]. AMW sheet burn times sit within 0.01 s of each
  curve's last sample ([AMW retests](amw-retests.md)), so older S&T sheets look like ignition to last action
  (**Inference**). NFPA 1125's burn-time clause itself was not reached.
- **TMT letters** print test date, burn time, impulse, peak and average thrust, propellant mass and a plotted curve;
  1999–2000 letters add "Burntime Evaluation (%): 5" [T1]. None read states how many motors a test fired. On 48
  H-and-up solid letters, the 5%-of-peak burn time of the letter's own curve sits a median +0.5% from the print,
  29 within ±3%, but 5 beyond ±10% (−13.3% to +29.5%) [T1] (**Inference**). Below, every TMT test is re-read
  from its curve, never from the print.
- **CAR publishes no σ:** 256 index rows of means only [N8]. Its one joint sheet with NAR, Cesaroni J210 (10
  firings, 2002), gives burn time 1.8%, impulse 0.85%, average thrust 1.6% [N2, id 1481432, pp. 1–2].

## 2. One lot, one session: 84 NAR sheets

**What was read.** NAR's list (updated 2026-08-12) links 68 H-and-up sheets [N1]; probing sheet ids found 15 it no
longer links (13 Kosdon, Cesaroni J210, AeroTech I218), the Internet Archive two (AeroTech I65, O6000W) [N2, N3].
Kept 84: AMW 50, AeroTech 13, Kosdon 13, Kosdon/AeroTech 7, Cesaroni 1 (no hybrids). Loki, Cesaroni Pro (bar J210)
and current AeroTech reloads were certified by TMT or CAR. 51 sheets give n (2 to 10); the other 33 are taken at
NAR's minimum, 3 up to 5,120 N·s and 2 above [N4, §3.3, p. 7]; the 51 alone pool burn time at 4.0%.

| Quantity, CV (%) | Sheets | Min | Q1 | Median | Q3 | Max | Pooled (95%, bootstrap over sheets) |
|---|---:|---:|---:|---:|---:|---:|---|
| Burn time | 84 | 0.00 | 1.14 | 1.97 | 3.42 | 18.81 | 4.12 (2.96–5.32) |
| Total impulse | 84 | 0.02 | 0.28 | 0.51 | 1.03 | 4.42 | 1.13 (0.86–1.39) |
| Average thrust | 33 | 0.10 | 1.33 | 2.14 | 3.79 | 10.99 | 3.70 (2.53–5.02) |
| Peak thrust | 84 | 0.02 | 1.62 | 3.55 | 5.64 | 19.10 | not pooled |

- **Pooled** is the root mean square weighted by n − 1. Bartlett's test rejects one shared σ (219 on 82 degrees of
  freedom, 0.1% point 127), so the bootstrap interval over sheets is the one to quote (**Inference**).
- **Burn time against impulse.** The per-sheet ratio has median 4.1 (quartiles 2.0–8.0); the two still rank
  together (Spearman ρ 0.45). Average thrust is printed as I/t on 98 of 99 firings, so its ρ 0.92 with burn time
  and within-sheet r −0.95 are mostly arithmetic. Pooling by sheet, not by firing, gives burn time 4.35%.
- **The tail.** Above 10%: AMW K1000 18.8% (2.18 s, σ 0.41 s) [N2, id 1481390], Kosdon H70 12.7%, AMW M2200 11.6%
  [N2, id 1481404], Kosdon K777 10.5%; next, AeroTech M650W 9.3% (9.73 and 8.53 s) [N2, id 1481304, p. 2].
  Without K1000 and M2200 the pool is 3.5%; without K1000 and H70, 3.3%.
- **One session scatters more.** 16 sheets come from one test day, 2004-03-27, at 17 °C; with K555 (also 17 °C) they
  pool 6.0% in burn time against 3.6% for the other 67, while their impulse pools 1.0% [N2].
- **Propellant (small counts, Inference).** Pooled burn-time CV: Redline 1.4% (7 sheets), Green Gorilla 1.5% (9),
  White Wolf 1.5% (11), Red Rhino 1.6% (8), Blue Baboon 4.0% (10), White Lightning 5.3% (7), Skidmark 7.6% (10;
  3.8% without its two 2004 sheets). No trend with class (H–I 3.7%, J–K 4.6%, L–M 4.1%, N–O 2.9%) or burn length
  (ρ 0.08); eras differ up to twofold (1996–99 5.4%, mostly Kosdon; 2000–03 2.4%; 2006–09 3.4%) with no trend.
- **A sheet's own σ is weak.** From the chi-square law, the 95% interval on a true σ runs 0.45–31.9 times a σ from 2
  firings, 0.52–6.3 times from 3, 0.68–1.9 times from 9 (**Inference**): the distribution over sheets is the
  result. One lot, one stand, one day at 20 ± 5 °C: a floor for what a flier sees, not a ceiling.

## 3. Tests years apart: TMT's repeat certifications on one definition

Earlier tests are ThrustCurve's public-domain "converted from TMT test stand data <year>" files [S1, S2] or the
1999–2000 letters' curves; later tests are the 2001–2004 letters' curves, read from the PDFs' vector paths and kept
only when the plot reproduces the printed impulse within 5% (J275W 2001 is kept at +5.04%; K458W 2001's curve
failed, so its printed impulse is used) [T1]. All are reduced one way: 5%-of-peak burn time, whole-curve impulse,
average thrust as their ratio. Steps are later over earlier; all motors are AeroTech.

| Motor | Tests | Impulse | Burn time | Avg thrust |
|---|---|---:|---:|---:|
| H242T | 1997 → 1999 → 2003 | +6.2%, −10.0% | −12.4%, −17.7% | +21.3%, +9.4% |
| H238T | 1997 → 2000 → 2004 | −3.5%, −3.3% | −15.0%, −10.4% | +13.5%, +8.0% |
| I284W | 1997 → 1999 | +4.0% | +22.0% | −14.8% |
| I435T | 1999 → 2004 | −1.0% | −4.3% | +3.5% |
| H128W, H180W | 1997 → 2003 | −2.3%, −10.8% | +9.6%, +9.0% | −10.9%, −18.2% |
| J275W, K250W | 1998 → 2001 | −3.5%, −2.3% | −9.1%, −4.4% | +6.1%, +2.2% |
| K700W, K458W | 1998 → 2001 | −0.7%, −6.7% | −7.6%, −0.3% | +7.5%, −6.5% |
| J415W, J570W | 1998 → 2003 | −3.9%, −3.9% | −3.2%, +3.1% | −0.8%, −6.8% |
| K1100T, L850W | 1998 → 2003 | −6.5%, −1.4% | −1.9%, −8.4% | −4.7%, +7.6% |

- **Size (Inference).** Pooled within-motor 1σ of one test: impulse 3.8%, burn time 9.2%, average thrust 8.5%;
  median absolute step 3.7%, 8.7%, 7.6%. Burn-time steps span −17.7% to +22.0%. With 16 degrees of freedom the
  burn-time σ's 95% chi-square interval is about 6.8–13.9%.
- **Reduction noise is small beside it.** Seven 1999 tests exist as both a ThrustCurve file and a letter curve: the
  file's burn time reads a median 1.5% shorter (−0.9% to +5.8%); taking out 2% per test leaves about 9% (**Inference**).
- **A drift, not only scatter.** Burn time fell in 12 of 16 steps (median −4.4%), impulse in 14 (median −3.4%). The
  letterhead moves from Sparks, NV to Glendale, AZ between tests ([TMT test sites](tmt-test-sites.md) §1); a February
  2000 to February 2004 pair (H238T) still fell 10.4%. Site, stand, product and lot cannot be separated, so 9% is
  an upper bound for one stand and a fair size for "catalog test against a motor bought later".
- **Product changes are larger.** Five motors whose propellant mass moved more than 3.5% (I195J, H123W, I357T,
  I161W, I435T 1996 → 1999) pool burn-time σ 19.8% and impulse σ 4.3%. Five Ellis Mountain motors, 1998 → 2003,
  all moved one way (impulse −5.9% to −14.3%, burn time +5.1% to +20.5%): a formulation change (**Inference**).
- **ThrustCurve holds no other repeat firings.** Its 67 H-and-up "converted from TMT" files are the only
  independent firings; 4 of the 99 motors with two or more distinct curves hold two TMT years. Of 7 other motors
  with two certification curves, 6 are re-digitizations, one sheet read twice or wrong uploads, and J575FJ (0.6%
  apart, no source) is unexplained; user and maker files are transcriptions [S1].

## 4. Same-lot test stands, a certifier's word, and temperature

| Source | Motor, firings | Burn time CV | Impulse CV | Avg thrust CV |
|---|---|---:|---:|---:|
| ULB, EUCASS 2025 [B1, Table 3, p. 8] | Cesaroni 540I470, 4 | 2.65% | 0.64% | 3.19% |
| same | Cesaroni 2412K1085, 3 (one grain defect) | 1.45% | 4.11% | 5.03% |
| UAH 2022 [B2, Table 5, PDF p. 18] | AeroTech G138T, 4 (below H) | 3.9% | 0.90% | 4.5% |

- CVs are SD over mean (**Inference**). EUCASS does not define its burn time, and its motors were near their
  expiry date. UAH's four motors came from two vendors, two each [B2, PDF p. 13]. Both sets fit NAR's 2–4%.
- **A certifier's member, 2007:** a NAR S&T member wrote that impulse over a lot of 10 test motors varies "on the
  order of 1% or less", while burn time and thrust vary "on average" under about ±5% and up to ±20% [F1, post 16].
- **Maker-side temperature data.** AeroTech's forum account posted, as an example of performance against
  temperature, Industrial Solid Propulsion's RDS-700 sheet: a 711 N·s HTPB/AP extraction motor, not a hobby reload,
  at −65, +70 and +165 °F [F2, post 9]. Read by eye (±0.05 s): burn time about 2.05, 1.78 (printed) and 1.55 s,
  +2.0% per 10 K colder, −2.4% hotter (**Inference**). A 2008 science-fair set of F-class composite motors (about 24
  each at −18, room and 49 °C) gives about −2.2% per 10 K, impulse 1.5% lower cold [F2, posts 8, 13] (**Inference**,
  bars read by pixel). No Cesaroni statement was found.
- **Share of the spread (Inference).** The certification band ±5 K is ±0.6–1.1% of burn time at 1.2–2.2% per 10 K
  ([propellant temperature](propellant-temperature.md) §4), part of NAR's median 2% but not its tail. TMT's 2020
  procedures name no test temperature [T3, §4.13] but defer to NFPA 1125's 20 ± 5 °C, so a 10 K σ between tests
  (1.2–2.4%, 2–7% of the TMT variance) is generous. Seasons hint the same way: retests in July–August 2003 moved a
  median −1.9% (7 steps), those in February or May −7.6% (7), the reverse of a warm-grain effect; the earlier
  tests' months are unknown, so this is weak evidence.

## What this suggests

- **Two layers for M2.8's burn-time prior (Inference).** A flier's motor differs from its catalog curve within a lot
  (NAR: 2% median, 4% RMS) and between lots and years; TMT's repeat tests bound the sum near 9%. Test coverage at
  σ = 4% and 9%, impulse 1% and 4%, and report which holds the logs' 50% and 90% ranges; NAR alone is too narrow.
- **The shape fits.** hpr's Monte Carlo stretches the curve in time and divides thrust by the same factor at
  fixed impulse, impulse drawn on its own ([`montecarlo.rs`](../../crates/hpr-analysis/src/montecarlo.rs)); burn
  time and impulse correlate weakly (§2, §3), so drawing them apart loses little.
- **Heavy tail:** 4 of 84 sheets exceed 10% against a median of 2%, and Bartlett's test rejects one σ for all
  motors; a per-motor σ or a Student-t would carry it, worth testing only if 90% coverage falls short.
- **Grain temperature as its own input** (about 2% per 10 K from 20 °C), not counted twice inside σ.
- **Burn times from curves, not prints:** on 5 of 48 TMT letters the curve's burn time is 12–30% off the print.
- **dispersion-sigmas:** its §1 table's "not measured" cell for a hobby H-and-up burn time can cite this note.

## Open questions

- The σ convention of the 51 NAR sheets without firing rows: all dividing by n would make burn time 4.9%, impulse 1.3%.
- Lot-to-lot spread at one stand: NAR posts no H-and-up recertification, and TMT's later tests mix site and product
  with lot; how many motors each TMT test fired; why later tests burned shorter; why 2004-03-27 scatters more.
- No temperature sensitivity of a hobby propellant; the RDS-700 crossings are read by eye. NFPA 1125's
  burn-time clause (7.8.3) was not reached in any edition.
- Loki, current Cesaroni Pro and current AeroTech reloads have no posted multi-firing record.

## Sources

Read 2026-10-29 unless marked; `ia:` is `https://web.archive.org/web/<timestamp>id_/`, `trf:` is
`https://www.rocketryforum.com/threads/`. ThrustCurve's TMT files are flagged public domain; the rest state no
license or are the authors' copyright: cited, not copied.

- **N1** NAR S&T Certified Motor List (Interactive), `https://www.nar.org/content.aspx?page_id=22&club_id=114127&`
  `module_id=670148`, "Last Updated: 2026 August 12".
- **N2** NAR motor data sheets, `https://www.nar.org/docs.ashx?id=<id>`, ids as cited (p. 1 summary, p. 2 firings).
- **N3** Internet Archive copies of `http://www.nar.org/SandT/pdf/Aerotech/I65.pdf` (`ia:` 20030319190307) and
  `.../AeroTech/AeroTech_O6000W_2023-11-15_TestReport.pdf` (`ia:` 20240401165808).
- **N4** NAR Motor Testing Manual v1.5 (2011), `https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual`
  `.pdf`. **N8** CAR Certified Motor Index, `https://car-acf.org/en/certifications/motor-certifications` (10-11).
- **N5** NFPA 1125 (2021 revision) First Revision statements, `https://docinfofiles.nfpa.org/files/AboutTheCodes/`
  `1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf`, FR-8, p. 7/9.
- **N6** ThrustGear, "Thrust Curve Tool" manual, `https://www.thrustgear.com/ThrustCurveTool.pdf`. **N7**
  CAR Motor Test Manual r20181121, `https://car-acf.org/resources/Documents/car_motor_testing_manual%20r20181121.pdf`.
- **T1** Archived TMT certification letters (178 PDFs), `ia:` + `http://www.tripoli.org/tmt/new_motors/` and
  `/motors/new_motors/` (read 2026-10-15 and 2026-10-29).
- **T3** TMT Policies and Procedures, rev. 2020-01-06, §4.13, `ia:20201128164829` + `http://www.tripoli.org/`
  `Portals/1/Documents/TMT/Resources/TMT%20Policies%20and%20Procedures%20-%2001-06-2020.pdf`.
- **S1, S2** ThrustCurve API, `https://www.thrustcurve.org/api/v1/download.json`: capture of 2026-09-17 (S1);
  the 29 multi-file motors, 2026-10-29 (S2).
- **B1** Lecomte et al., "Experimental Investigation of Solid Rocket Motors for a Student Sounding Rocket",
  EUCASS 2025, DOI 10.13009/EUCASS2025-024.
- **B2** M. Amara, "Performance Variability in Solid Rocket Motors", UAH Honors Capstone (2022),
  `https://louis.uah.edu/cgi/viewcontent.cgi?article=1682&context=honors-capstones`.
- **F1** `trf:motor-consistency.109733/`, post 16 (2007-10-07). **F2** `trf:propellant-temperature-effect-on-`
  `motor-performance.190157/`, posts 8, 9, 13 (2024-12-23 and 24).
