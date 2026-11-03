# Which RASAero version produced the comparison page's predictions

**In short: 33 of the 36 predicted apogees on RASAero II's comparison page were run in the older RASAero (I),
version 1.0.2.0, by 2013. The page was relabeled "RASAero II" in 2023 without changing a number. RASAero II's own
prediction for each shipped design is the "RASAero II Prediction" written in that `.CDX1` file's comments.** It
equals the file's stored `MaxAltitude`, rounded, in all 24 files. The 1.0.2.0 user manual and the rerun Caliber
page quote the same figures. So [M2.3c7, faster than sound](../decisions-and-roadmap.md#m2-3c7) should put the
`.CDX1` figure as RASAero II's published apogee for all 17 eligible flights. On those 17 the switch moves
the mean signed error from +2.49% to +1.92% and the mean absolute error from 5.61% to 5.73%. Per flight it moves
from −4.55 points (Rabia) to +0.50 (Raven) (**Inference**: our arithmetic). Confidence is high on which program
made which number: archived pages, two installers and the manual agree. Which RASAero II release (1.0.0.0,
1.0.1.0 or 1.0.2.0) wrote the `.CDX1` results is not recorded anywhere this note found.

**What this covers.** Wayback Machine captures of the comparison page, the home page, the download page and four
per-flight pages, 2011 to 2026 [S1–S3, S7]. It also covers the data files in both installers: RASAero II 1.0.2.0
and an archived RASAero (I) 1.0.2.0 [S4, S5]. Last, the RASAero II 1.0.2.0 user manual [S6]. It builds on
[RASAero II's comparison flights](rasaero-comparison-flights.md) §2–3, which found the 17 eligible flights and
the two disagreeing columns. Every count below was made from the cached files. Nothing was run in either program.

## 1. The page's column is RASAero (I)'s, relabeled

The page's history, read from its archived copies (39 index entries on 2026-11-02, six content digests) [S1]:

| Date | What the source shows | Source |
|---|---|---|
| 2011-03-09 | RASAero (I) 1.0.2.0 installer saved; its 26 `.ALX1` comments hold the page's 26 values | S5 |
| 2011-06-13 | Proteus 6 page: RASAero predicts 86,799 ft, today's table value | S7 |
| 2013-10-30 | Home page: average error 3.38%, 78.8% within ±10%, the 33-row table's figures | S2 |
| 2015-09-12 | RASAero II 1.0.0.0 released; 1.0.1.0 follows on 2016-08-05, 1.0.2.0 on 2019-05-22 | S3 |
| 2016-05-13 | Page: 33 rows, ".alx1 files", all predictions "run on RASAero Version 1.0.2.0" | S1 |
| to 2021-06-12 | Every capture has the 2016 content digest | S1 |
| by 2023-02-08 | 36 rows, average 3.47%: the home page quotes it (**Inference**: no page capture between) | S2 |
| 2023-03-16 | Page names `.CDX1` files but still says "RASAero" | S1 |
| 2023-09-21 | "RASAero II" all but the column header; numbers unchanged through 2026-10-16 | S1 |

- **The 33 old rows kept their numbers.** All 33 measured and predicted apogees and errors of the 2016 page reappear
  unchanged in 2023. All 36 rows are identical across the captures of 2023-03-16, 2023-09-21, 2025-11-17 and
  2026-08-20 and the copy read on 2026-10-16. The 2016 table's mean signed error recomputes to 3.38%, with 26 of 33
  within ±10%. Those match the 2013 home page, two years before RASAero II existed (**Inference**: the same rows).
- **The 2016 page's version is RASAero (I)'s.** RASAero (I) 1.0.2.0 was released 2011-03-15 [S2, S3]. RASAero II's
  1.0.2.0 came on 2019-05-22, three years after the 2016 capture [S3]. The 2016 page also names `.alx1` files,
  RASAero (I)'s format.
- **The three new rows are RASAero II's.** MESOS, ThreeCarbYen and AeroPac 104K were added between 2021-06-12 and
  2023-02-08. MESOS's 289,789 ft is labeled RASAero II in its own report [S8, p. 14]. AeroPac's equals its `.CDX1`
  and the manual [S4, S6]. All three are multi-stage, and RASAero II's release notes list multi-stage flight as new
  over RASAero [S3]. So ThreeCarbYen's 185,018 ft must be RASAero II's too (**Inference**).
- **The `.ALX1` files are 2011's.** 25 of the 26 `.ALX1` files in RASAero II's installer are byte-identical to those
  in RASAero (I)'s 2011 installer. The 26th, `CalIsp1.ALX1`, differs only in a resaved CG and the program's version
  stamp. Its comment is unchanged. In RASAero II's installer, 24 of the 26 carry the assembly stamp
  "RASAero, Version=1.0.0.0"; Proteus 6 carries 1.0.1.0 and `CalIsp1` 1.0.2.0 [S4, S5].

## 2. Where RASAero II's own figures are published

- **The `.CDX1` comments.** Each of the 24 shipped designs states a prediction and an error in its comments. In 22
  the label is "RASAero II Prediction". In all 24 the figure equals the stored `MaxAltitude` rounded to the foot,
  and the stated error is computed from that rounded figure [S4].
- **The two "RASAero Prediction" files.** Cancer Descending and EZI-65 (J450ST) keep the older label. Their stored
  `MaxAltitude` (6,327.6 and 4,214.1 ft) must still come from a RASAero II run, because no `.ALX1` has a results
  element (**Inference**, from the element lists of all 26) [S4]. Both equal the page's
  figure to within 0.4 ft.
- **The manual.** The RASAero II 1.0.2.0 user manual (PDF created 2019-05-19) works three of the shipped files as
  examples. It quotes Caliber Team 3 at 3,876 ft and −2.22% (printed p. 123) and Full Metal Jacket BALLS 005 at
  38,772 ft and +2.08% (p. 140). It quotes AeroPac 104K at 113,786 ft and +8.72% (p. 154). All three are the
  `.CDX1` figures; the page gives 3,871 and 38,820 for the first two [S6].
- **The per-flight pages.** Three were rerun after 2015 and one only relabeled [S7]:

| Page | Before (capture) | After (capture) | Matches |
|---|---|---|---|
| Caliber (Team 3) | RASAero 3,871 ft, −2.35% (2015-09-16) | RASAero II 3,876 ft, −2.22% (2023-09-21) | `.CDX1` |
| Violent Agreement | RASAero 19,256 ft, +4.11% (2017-10-21) | RASAero II 18,784 ft, +1.56% (2024-04-22) | no file |
| Violent Agreement SS | RASAero 25,602 ft, +11.93% (2021-10-20) | RASAero II 24,784 ft, +8.35% (2023-06-09) | no file |
| Proteus 6 | RASAero 86,799 ft (2011-06-13) | "RASAero II" 86,799 ft (2024-02-25) | `.ALX1` |

  The Proteus 6 `.CDX1` says "RASAero II Prediction = 81499 ft", 6.11% below the page. So the relabel alone does not
  make a figure RASAero II's.

## 3. Which RASAero II release wrote the `.CDX1` results

Not pinned. A `.CDX1` carries `FileVersion` 2 and no program version. The installer's MSI was last saved
2019-05-15. The 1.0.2.0 manual quotes three of the files' results [S4, S6]. The figures could date from any release
since 2015, and the release notes bound how much that matters [S3]. 1.0.1.0 changed power-on base drag only for
very large nozzle exits, such as orbital stages. 1.0.2.0 made these model changes:

- nose wave drag for LV-Haack, parabolic and elliptical noses: none of the 24 files uses one;
- the Rogers Modified Barrowman normal force and center of pressure: switched on in 2 files, A-601 and Qu8k;
- fin supersonic friction with all-turbulent flow, for rounded and square airfoils only: the 4 all-turbulent files
  (AeroPac, Don't Debate This, A-601, Qu8k) all use hexagonal airfoils;
- first motion on the rail when thrust starts below weight: in `rasp.eng` each of the 17 motors' first thrust point
  already exceeds the file's launch weight, closest CTI L730 at 81.4 N against 78.6 N (**Inference**);
- large-nozzle base drag, which the notes say leaves most hobby rockets unchanged.

So for all 17 eligible flights, no listed 1.0.2.0 change applies by the file's own settings (**Inference**). The
size of any unlisted change between releases is unbounded. The Rocket Flight Database reports 50 to 500 ft of
"drift" between RASAero II versions on a few flights. It names Proteus 6, Qu8k, Rabia and Gibb, and treats the
page's figures as RASAero II's [S9, `docs/reproducibility.md` Step 2 and Troubleshooting]. On this note's
reading those gaps are RASAero (I) against RASAero II instead. Proteus 6's is 5,300 ft, and its page figure was
online in 2011 [S7].

## 4. The candidates, flight by flight

Err is simulated minus measured, over measured. Page is the comparison table, run in RASAero (I) 1.0.2.0 (§1).
`.CDX1` is the file's "RASAero II Prediction" and the error its comment states. Recomputing from the unrounded
`MaxAltitude` changes no error by more than 0.012 point (Caliber Team 2). Gap is `.CDX1` minus page, in points
(**Inference**: our arithmetic) [S1, S4].

| Flight | Measured ft | Page ft, err | `.CDX1` ft, err | Gap |
|---|---|---|---|---|
| Don't Debate This (Mach ~3.05) | 56,573 | 62,308, +10.14% | 61,982, +9.56% | −0.58 |
| Torrent (Mach ~1.23) | 12,807 | 13,852, +8.16% | 13,717, +7.11% | −1.05 |
| Rabia (Mach ~1.16) | 12,745 | 12,777, +0.25% | 12,197, −4.30% | −4.55 |
| Raven (Mach ~1.07) | 8,815 | 9,288, +5.37% | 9,332, +5.87% | +0.50 |
| Rabia, short fin can | 10,584 | 10,376, −1.97% | 10,225, −3.39% | −1.42 |
| Blister | 9,026 | 8,347, −7.52% | 8,301, −8.03% | −0.51 |
| Ion Drive | 8,027 | 8,642, +7.66% | 8,642, +7.66% | 0.00 |
| Cancer Descending | 6,188 | 6,328, +2.26% | 6,328, +2.26% | 0.00 |
| Byrum | 5,732 | 5,280, −7.89% | 5,281, −7.87% | +0.02 |
| EZI-65 (J450ST) | 3,965 | 4,214, +6.28% | 4,214, +6.28% | 0.00 |
| Gibb | 3,913 | 4,310, +10.15% | 4,205, +7.46% | −2.69 |
| Thunder & Lightning | 3,577 | 3,989, +11.52% | 3,989, +11.52% | 0.00 |
| Caliber Isp 05, 5,085 ft | 5,085 | 4,842, −4.78% | 4,847, −4.68% | +0.10 |
| Caliber Isp 05, 4,930 ft | 4,930 | 4,831, −2.01% | 4,836, −1.91% | +0.10 |
| Caliber Isp 04, Team 3 | 3,964 | 3,871, −2.35% | 3,876, −2.22% | +0.13 |
| Caliber Isp 04, Team 1 | 3,837 | 3,943, +2.76% | 3,948, +2.89% | +0.13 |
| Caliber Isp 04, Team 2 | 3,710 | 3,871, +4.34% | 3,876, +4.47% | +0.13 |

- **Sizes.** 7 of the 17 move by more than 0.2 point; 4 do not move at all. Over the four past Mach 1, the mean
  signed error falls from +5.98% to +4.56%, and the mean absolute error rises from 5.98% to 6.71%. The Machs are
  the earlier note's, from the stored peak speeds [S10].
- **No per-flight page** covers any of the 17 except Caliber Team 3, and that page agrees with the `.CDX1`.
- **Outside the 17.** The Violent Agreement pair has RASAero II figures only on its per-flight pages. Eight rows have
  no RASAero II figure anywhere: the 6 `.ALX1`-only rows, Project R and Proteus 6.5. If every row that has one used
  its RASAero II figure (24 files, the 2 per-flight pages and MESOS and ThreeCarbYen), 28 rows would give a mean
  signed error of +2.27% and a mean absolute error of 5.36%, with 27 of 28 within ±10%. The page states 3.47% and
  80.6% over 36 (**Inference**: our arithmetic).

## What the project can use

- **Cite only.** rasaero.com grants no license; its comparisons index says "All rights reserved" [S1]. The
  manual is "Copyright © 2019" by its author; the installers carry no license [S4, S5, S6]. The Wayback captures
  are copies of the same pages. They and both installers are cached under `refs/research/` for checking and
  never committed. The project may publish the apogees, errors, dates and file digests above, as the earlier
  note did.
- **The Rocket Flight Database** (CC BY 4.0) may be cached and cited, but its `apogee_rasaero_ft` column is the
  page's. 23 of its 25 RASAero rows come from the 2016 RASAero (I) table [S9].

## What this suggests

1. **A rule M2.3c7 can apply per flight.** Use the `.CDX1` comment's prediction, which equals the rounded stored
   `MaxAltitude`, as RASAero II's published apogee. Without a file, use a per-flight page run after 2015-09-12.
   Otherwise use the page only for rows first published after 2021-06-12. For the 17 eligible flights the first
   rule always applies. Expected effect: the column's mean signed error over the 17 falls 0.57 point. Rabia's
   comparison with HPR Sim changes most.
2. **Show the page's figure beside it, labeled "RASAero 1.0.2.0 (2011), as published".** Readers who check the
   page will otherwise see different numbers. The done-when's "published errors" is met by both, and only the
   `.CDX1` figure is RASAero II's.
3. **The importer can check itself.** Its parsed `MaxAltitude` must round to the comment's figure in 24 of 24 files.
4. **Flag nothing new as a reconstruction.** Of the 17, no comment or page mentions tuning. AeroPac and MESOS stay
   flagged, outside the 17.

## Open questions

- Which RASAero II release produced the `.CDX1` results? Only a run of 1.0.2.0 on the shipped files would test
  whether they reproduce; the older installers were not found in the Wayback Machine's index on 2026-11-02.
- Why do 4 of the 24 RASAero II results equal RASAero (I)'s to the foot, while Gibb, on the same motor and site as
  Thunder & Lightning, moves 2.69 points? The release notes don't say.
- Is a `.CDX1` for either Violent Agreement flight public anywhere? Their RASAero II figures exist only on the
  per-flight pages, so they stay out of the scored set.

## Sources

1. **S1.** C. E. Rogers, "RASAero II Comparisons with Altitude Data", <https://www.rasaero.com/comparisons-alt.htm>,
   read 2026-10-16; Wayback captures 2016-05-13, 2023-03-16, 2023-09-21, 2025-11-17, 2026-03-14 and 2026-08-20, and
   the capture index, read 2026-11-02 (<https://web.archive.org/web/2016*/rasaero.com/comparisons-alt.htm>); and
   the comparisons index, <https://www.rasaero.com/comparisons.htm>, read 2026-10-16.
2. **S2.** RASAero home page, <https://www.rasaero.com/home.htm>, Wayback captures 2013-10-30, 2015-09-24,
   2019-08-22, 2023-02-08 and 2026-08-20, read 2026-11-02.
3. **S3.** "Download the RASAero II Software", <https://www.rasaero.com/dl_software_ii.htm>, release histories of
   RASAero II and RASAero; Wayback captures 2016-05-13, 2017-03-01, 2019-06-14 and 2026-08-20, read 2026-11-02.
4. **S4.** RASAero II installer `RASAero_II_Setup_Version_1.0.2.0.zip` (sha256 `932e6ad7…bef6`), its 24 `.CDX1`
   and 26 `.ALX1` example files, fetched 2026-10-16, re-read 2026-11-02;
   see [the earlier note](rasaero-comparison-flights.md).
5. **S5.** RASAero (I) installer `RASAero_Ver_1.0.2.0.zip`, Wayback capture 2016-10-22 of
   <http://rasaero.com/dloads/RASAero_Ver_1.0.2.0.zip>, sha256 `0be9ab1b…3616`, MSI last saved 2011-03-09;
   unpacked with `msiextract`, nothing run, read 2026-11-02.
6. **S6.** C. E. Rogers and D. Cooper, RASAero II Users Manual, version 1.0.2.0, printed pp. 123, 140 and 154,
   <https://www.rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>, read 2026-11-02.
7. **S7.** Per-flight comparison pages on rasaero.com: `comp-flight-Cal.htm` (captures 2015-09-16, 2023-09-21),
   `comp-flight-Proteus6.htm` (2011-06-13, 2021-10-21, 2024-02-25), `comp-flight-ViolentAgreement.htm`
   (2017-10-21, 2024-04-22), `hidden/comp-flight-ViolentAgreementSS.htm` (2021-10-20) and
   `comp-flight-ViolentAgreementSS.htm` (2023-06-09), read 2026-11-02 through <https://web.archive.org/>.
8. **S8.** C. E. Rogers, "RASAero II Comparison with MESOS 293K Flight Data, Rev B", p. 14,
   <https://www.rasaero.com/dloads/RASAero%20II%20Comparison%20with%20MESOS%20293K%20Flight%20Data%20-%20Rev%20B.pdf>,
   read 2026-11-02 from the copy cached 2026-10-16.
9. **S9.** A. Yu, Rocket Flight Database v1.2.0 (CC BY 4.0), `flight_comparison.csv` and `docs/reproducibility.md`,
   <https://zenodo.org/records/20531977>, read 2026-11-02 from the copy cached 2026-10-16.
10. **S10.** [Which of RASAero II's comparison flights M2.3c7 can fly](rasaero-comparison-flights.md), §2.
