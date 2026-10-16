# Do TMT and AeroTech curves of one motor differ by the test-site pressure term?

This note asks whether AeroTech's own thrust data and the certification data for the same motor differ by the
pressure thrust $(p_{\mathrm{site}} - p_{\mathrm{maker}}) A_e$ that their test heights predict. AeroTech's factory,
taken as its stand, is in Cedar City, UT (town center 1,781 m, 81.7 kPa in the ISA). TMT fired in North Carolina
in 2017–2024 (99.6 kPa) and near Denver from 2025 (83.6–85.0 kPa); its earlier sites are unknown ([TMT test
sites](tmt-test-sites.md) §1–4). Exits are the drawn ones of [nozzle exits](nozzle-exits.md) §1. It is
for whoever sets the sign of a thrust-at-altitude term and for [apogee bias](apogee-bias.md) §1. **How far to trust
it:** the counts and gaps come from cached data and the scripts named below; every fit and reading of a gap is
**Inference**. Sources read 2026-10-15.

**In short.** The public data can't measure the term: a null result, with bounds.
- **Only 5 AeroTech motors carry both a certification and a maker curve on ThrustCurve, 2 of them TRA** (§1).
  One TRA pair is a copy (H130W, identical points); the other is a redesigned grain (M1780NT, −5.0% impulse). The
  3 NAR pairs differ by +0.4% to +11.3% of impulse, against a predicted +0.7% to +6.4%, with exits unknown.
- **ThrustCurve's 53 "mfr" files are mostly transcriptions,** not maker data files: 17 say they were fit to an
  AeroTech instruction-sheet graph, 27 state no source. 17 of 53 sit within 0.25% of the certified impulse, as if
  scaled to it (§2).
- **AeroTech's own figures often copy the certification.** 60 of 149 store listings and 42 of 132 instruction-sheet
  rows match ThrustCurve's impulse within 0.25%; 96 of 138 store average thrusts equal the designation number (§3).
- **North Carolina era (2017–2024): too few figures to fit** (§4). 5 store figures run the predicted sign (N4000W-PS
  +1.96% against +1.74%; others +1.3% to +11.1% against +1.0% to +5.0%); a J99N maker file runs −1.66% against +1.88%.
- **Before 2017, the gaps don't scale with the exit** (§5). Over 113 maker figures (78 motors), the gap is a flat
  +2.30% ± 0.74% with a pressure slope of −1.3 ± 7.7 kPa (1 SE). An uncorrected Cedar City maker against a
  near-sea-level TMT site would give +17.9 kPa, 2.5 SE away. The gaps' SD, 4.17%, is 3.6 times the median term.

## 1. The motors with both curves

ThrustCurve lists 307 AeroTech motors (222 TRA, 84 NAR, 1 unknown) and returns 599 data files for them: 215
certification, 53 maker and 331 user [S1]. A file's source is what its uploader picked: a certification
organization's page, the maker's page, or self-made [S2]. 47 motors have a maker file; 5 also have a certification
file. Gaps below are maker minus certification, over the certification value. Each curve is reduced the same way,
as ThrustCurve defines it ([ThrustCurve data](thrustcurve-data.md) §2): impulse over the whole curve, burn time
between the 5%-of-peak crossings, average thrust = impulse / burn time. Script: `scripts/pairs.py` (private cache).

| Motor | Cert, era | Maker file (as labeled) | Impulse, N·s mfr / cert | Avg thrust, N | Burn, s | ΔI |
|---|---|---|---:|---:|---:|---:|
| D9W | NAR | no source stated | 20.9 / 18.8 | 9.3 / 10.5 | 2.25 / 1.79 | +11.34% |
| G53FJ | NAR | user fit to AeroTech sheet | 92.1 / 91.0 | 54.4 / 53.1 | 1.69 / 1.71 | +1.27% |
| G76G | NAR | user fit to AeroTech sheet | 115.0 / 114.5 | 74.1 / 73.0 | 1.55 / 1.57 | +0.42% |
| H130W | TRA, maker file 2016 | same points as cert | 210.2 / 210.2 | 135.0 / 135.0 | 1.56 / 1.56 | 0.00% |
| M1780NT | TRA, cert file 2010 | "1 in core revised" | 5,341.1 / 5,623.0 | 1,790.4 / 1,782.2 | 2.98 / 3.16 | −5.01% |

ThrustCurve's metadata gives 18.76, 90.9, 118, 210 and 5,783 N·s [S1]; for the G76G and M1780NT it sits 3.1% and
2.8% above the certification curve, so file and catalog are not always the same test.

- **Predicted, NAR pairs (Inference).** NAR fires within 500 ft of sea level or corrects ([TMT test
  sites](tmt-test-sites.md) §5), 17.8–19.6 kPa above Cedar City. The 24 and 29 mm nozzles (01250, 01350) have no
  published exit. With RCS's molded 24 mm nozzle (0.190 in) or its single-use inserts (0.237–0.253 in; [nozzle
  exits](nozzle-exits.md) R2) for the D9W, and the 29 mm 01400–01500's 0.250–0.438 in for the G53FJ and G76G, the
  maker would read +3.3% to +6.4%, +1.1% to +3.6% and +0.7% to +2.5% high.
  Measured: +11.3% (with a 25% longer burn: a different curve shape), +1.3%, +0.4%. Same sign; no size.
- **H130W:** both files hold the certification curve; no information. **M1780NT:** the maker file says the core
  was revised, and its instruction sheet (2009) lists 7/8 in cores and 5,688 N·s [S7]. Its certification (2010)
  predates North Carolina; a grain change swamps the 1.7% its 1.875 in exit gives a sea-level site against Cedar City.

## 2. What ThrustCurve's "mfr" files are

Of the 53 maker files [S1, comment lines], 17 say a user fit them to an AeroTech instruction-sheet graph (most by
one contributor, 2006–2011); 7 cite "AeroTech curves", "pre-cert data" or an AeroTech thrust curve; 1 (L1940X) cites
the TMT document; 1 (N3120X-PS) an AeroTech release of 2025-11-12; 27 state no source. Contributor and date
come from the files' comment lines [S1] and six file pages [S3]. Licenses: 20 public domain, 7 "free", 26 unmarked.

- **Many are scaled to the certified impulse.** 17 of 53 lie within 0.25% of ThrustCurve's impulse and 6 within
  0.05% (L2200G +0.00%, from a hand fit to a graph). **Inference:** a hand fit lands that close only when scaled
  to a stated total, so impulse from these files says little about the maker's own firing.
- **Average thrust is noisier than impulse.** Against ThrustCurve's metadata, the 53 files' gaps have an SD of
  2.40% in impulse, 5.87% in average thrust and 6.01% in burn time [`pairs.py`]: a coarse transcription moves the
  5% cutoffs. So impulse is compared below; the pressure term predicts the same share for both.

## 3. AeroTech's own figures: store and instruction sheets

AeroTech's store lists 703 products; 149 that state a total impulse match a ThrustCurve motor [S6]. 214 cached
instruction sheets give 146 "typical" impulse rows for 137 designations [S7]. Scripts: `scripts/atstore.py`,
`scripts/sheets.py`, `scripts/compare.py` (whose matching the counts below follow).

- **Copies.** 60 of 149 store figures and 42 of 132 sheet rows (both matched to a TC motor) sit within 0.25% of
  ThrustCurve's impulse. The 2023 Master Motor Matrix prints the N3300R's certified 14,041 N·s ([motor
  data](motor-data.md) M15); its store page says 13,410.
- **Nominal values.** 96 of the 138 store average thrusts equal the designation's number to 1 N, 93 exactly
  (L1256WS: 1,256 against a certified 1,257.07 N). Of §5's 113 non-copy figures, 78 are round to 10 N·s. Some
  store impulses are case capacities (M2500T: 10,240 N·s, the 98/10240 case; certified 9,671).
- **Reading (Inference).** Where AeroTech's figure is not a copy, it may be its own pre-certification firing, an
  older certification, or a design target. Nothing read says which, nor whether AeroTech corrects to sea level.

## 4. North Carolina and Denver eras

TMT's 2021 announcement lists 15 AeroTech certifications of June 2018 to September 2020 [S5]; with ThrustCurve's
letters of 2018–2024, 37 AeroTech-made designations were fired in North Carolina, and 4 on 2025-10-06 [S4]. North
Carolina sits 17.9 kPa above Cedar City, Windsor–Denver 1.9–3.3 kPa ([TMT test sites](tmt-test-sites.md) §2–3).
Predicted share = 17.9 kPa × $A_e$ / certified average thrust.

| Motor (nozzle) | Maker figure | Maker, N·s | Cert, N·s | Gap | Predicted |
|---|---|---:|---:|---:|---:|
| N4000W-PS (01880-4, 2.737 in) | store | 18,540 | 18,183.8 | +1.96% | +1.74% |
| N4000W-PS | maker file, no source | 18,278.5 | 18,183.8 | +0.52% | +1.74% |
| J99N (0.438 in) | maker file | 929.5 | 945.2 | −1.66% | +1.88% |
| H169WS (01500-4, 0.438 in) | store | 236 | 232.95 | +1.31% | +1.03% |
| J435WS (01550-4, 0.688 in) | store | 774 | 742.77 | +4.20% | +0.97% |
| G12ST (01400-1M, 0.250 in assumed) | store | 160 | 143.97 | +11.13% | +5.00% |
| H14ST (01400-6M, 0.250 in assumed) | store | 265 | 240.12 | +10.36% | +4.28% |

Nozzle parts from the reload drawings [S8]; certified values from ThrustCurve [S1] and the N4000W-PS letter (two
firings, burn times 4.747 and 4.563 s) [S4]. The store's other 10 North Carolina figures copy the certification.
The end burners' machined nozzles (-1M, -6M) have no drawn exit; the molded 01400's 0.250 in is assumed.

- **Sign loose, size loose (Inference).** The five store gaps are positive, as an uncorrected TMT site lower than
  Cedar City predicts; two match in size, three run 2.2–4.3 times it, two of those (160, 265 N·s) nominal. The maker
  files split: N4000W-PS +0.52%, J99N −1.66% (a G75M file, +3.57%, has no drawn exit). Their scatter (SD 4.66% for
  the store five) can't separate a 1–2% term from zero.
- **The G11ST/G12ST of 2018.** The member who posted their files wrote that TMT's data made them "technically a G10
  and G11", and that the pressure difference to AeroTech's factory could explain "this" [S9, post 19]; the
  word may refer to the G11-against-G12 difference rather than to the TMT-to-AeroTech gap. The
  G12ST's 11.31 N sits 0.69 N below its name; the 0.250 in exit predicts 0.57 N (**Inference**).
- **Denver era: nothing to compare.** The G73C, F115SN and J550ST store figures copy the letters. The N3120X-PS
  maker file integrates to +0.39% of its letter's 10,329 N·s [S4]; with an 01770-size exit (not drawn yet) the
  prediction is +0.11% to +0.19%.

## 5. Before 2017: a fit across motors

For TRA motors not in §4 the site is unknown ([TMT test sites](tmt-test-sites.md) §7). If neither side corrects,
each gap is $k\,A_e/\bar F$ plus scatter, with $k = p_{\mathrm{site}} - p_{\mathrm{maker}}$. The fit takes every
non-copy store or sheet figure with a drawn exit: 113 figures, 78 motors; their predicted share at 17.9 kPa spans
0.32–5.67% (median 1.16%, SD 0.92 points). Script: `scripts/fit2.py`.

| Subset | n | Mean gap | k through zero, kPa | Offset | k with offset, kPa | Rank corr. |
|---|---:|---:|---:|---:|---:|---:|
| Store and sheet, TRA before 2017 | 113 | +2.19% | +18.9 ± 4.2 | +2.30% ± 0.74 | −1.3 ± 7.7 | −0.03 |
| Same, not round to 10 N·s | 35 | +2.09% | +16.8 ± 8.4 | +3.83% ± 1.96 | −19.6 ± 20.3 | −0.30 |
| Same, ST motors left out | 109 | +2.24% | +19.4 ± 4.4 | +2.32% ± 0.76 | −0.9 ± 7.9 | −0.02 |
| Maker files, TRA before 2017 | 19 | −0.42% | −2.5 ± 7.2 | −1.21% ± 1.44 | +10.2 ± 16.8 | +0.19 |

Errors are 1 SE. 80 of the 113 gaps are positive; their SD is 4.17%. Impulse is compared, not average thrust (§2–3).

- **The gap is an offset, not a pressure term (Inference).** Forced through zero, the fit returns $k$ ≈ +19 kPa,
  which reads as a site near sea level. That is the +2.2% mean leaking into the slope. With an offset free, the
  slope is −1.3 ± 7.7 kPa and the rank correlation −0.03: bigger exits relative to thrust don't show bigger gaps.
- **What site pressure fits.** If AeroTech's figures are uncorrected Cedar City firings, $k$ within ±2 SE puts the
  pre-2017 sites at 65–96 kPa (about 470–3,600 m); if AeroTech corrects to sea level, at 85–115 kPa (sea level to
  about 1,500 m). An uncorrected maker against an uncorrected or corrected near-sea-level TMT ($k$ = +17.9 to +19.6
  kPa) sits 2.5–2.7 SE from the fit. That is the only claim the data come near making, and it holds only if the
  maker's figures are its own measurements (§3).
- **The offset itself** (+2.3% ± 0.74%) fits TMT's own 2001–2004 retests, which fell a median 2.7% on 22 AeroTech
  motors ([HPR lot impulse](hpr-lot-impulse.md) §2), or targets printed above the measured mean. It can't be told
  apart from a uniform correction either.

## 6. What swamps the term

| Cause | Size | Source |
|---|---|---|
| Pressure term, NC against Cedar City | +0.3% to +5.7% (median 1.2%) | §5, drawn exits |
| Gap scatter, maker figure against cert | SD 4.2% (113 figures) | §5 |
| TMT's retests of one motor | median −2.7%, SD 4.4 points (22 pairs) | [HPR lot impulse](hpr-lot-impulse.md) §2 |
| Grain change (M1780NT core) | −5.0% | §1 |
| Transcription and scaling | 17 of 53 maker files scaled; thrust SD 5.9% | §2 |
| Copies (store, sheet, maker file) | 60/149, 42/132, H130W and L1940X | §1–3 |
| Old nozzles (pre-2003 L1500T: 1.4 in, now 1.75 in) | exit area off ≤ 1.56× | [Nozzle exits](nozzle-exits.md) §2 |
| Each stand's temperature (TMT's procedures omit NFPA's 20 °C) | unsized | [TMT test sites](tmt-test-sites.md) §5 |

## What this suggests

- **Keep the per-era test pressures of [TMT test sites](tmt-test-sites.md) §6–7.** This note neither confirms nor
  overturns them: for the 196 curves of unknown site (most pre-2017) the sign stays unbounded. Expected effect:
  none on any number.
- **Don't use AeroTech's store or sheet figures, or ThrustCurve's "mfr" files, as an independent second test.**
  Over a third copy the certification; the other store and sheet figures carry a flat +2.3% offset of unknown cause
  (pre-2017 TRA maker files average −0.42%). A motor importer that compares them should report a gap, not average.
- **Apogee bias §1 can cite this note:** maker-against-certification data show no detectable test-site term, if
  the maker's figures are its own firings (slope −1.3 ± 7.7 kPa, offset +2.3%). Its pressure-thrust share stays.
- **Data use.** ThrustCurve files carry their uploader's license (PD, free or none); AeroTech's store shows a ©
  notice, RCS's parts site "all rights reserved" (motor-data M17). Cite facts, cache under `refs/`, commit no file.
- **A decisive test needs paired firings:** one lot fired by AeroTech and by TMT with the site pressure recorded,
  or TMT's raw files before any reduction. Neither is public.

## Open questions

- Whether AeroTech corrects its firings to sea level, and where its stand is (only its Cedar City address is known).
- What the instruction-sheet "typical" curves are: an AeroTech firing, an older certification, or a target.
- Where TMT fired before 2017; and the exits of the 24 and 29 mm nozzles (01250, 01350) and the machined 01400-xM.

## Sources

All read 2026-10-15 unless marked; cached under `refs/research/tmt-vs-maker-curves/` (gitignored), never committed.
Scripts there: `fetch.py`, `pairs.py`, `atstore.py`, `sheets.py`, `compare.py`, `fit2.py`.

1. **S1** ThrustCurve API: `https://www.thrustcurve.org/api/v1/search.json?manufacturer=AeroTech&maxResults=2000`
   (307 motors) and `download.json` with `data: "both"` for all 307 (599 files, comment lines read).
2. **S2** ThrustCurve, Contribute Data, "Data Source": <https://www.thrustcurve.org/info/contribute.html>.
3. **S3** ThrustCurve file pages, `https://www.thrustcurve.org/simfiles/<id>/`: 6450e4dcdf10040002d18677
   (N4000W-PS), 691a315a2d19df0002c3ed70 (N3120X-PS), 60ac84458dc4640004c24eb6 (L1940X),
   5f4294d20002e90000000789 and 5f4294d20002e9000000071d (M1780NT), 5f4294d20002e90000000856 (H130W).
4. **S4** TMT letters, `https://www.thrustcurve.org/motors/cert/<id>/`: 645072c42cdedd00028df9f4 (N4000W-PS,
   2023-04-30), 6919f5ee2d19df0002c3a316 (N3120X-PS, 2025-10-16); others as [motor data](motor-data.md) S2.
5. **S5** Tripoli TMT announcements, archived 2021-10-18 ([TMT test sites](tmt-test-sites.md) T18).
6. **S6** AeroTech store, `https://aerotech-rocketry.com/products.json?limit=250&page=1` to `3` (703 products).
7. **S7** RCS/AeroTech instruction sheets, 214 ([motor data](motor-data.md) M3, read 2026-10-11); M1780NT-P p. 2.
8. **S8** RCS assembly drawings' nozzle lines ([nozzle exits](nozzle-exits.md) R5, `bom.json`).
9. **S9** The Rocketry Forum, "3 New Aerotech motor certifications and motor files", post 19 (2018-08-27),
   `https://www.rocketryforum.com/threads/3-new-aerotech-motor-certifications-and-motor-files.147651/`.
