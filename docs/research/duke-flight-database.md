# The Rocket Flight Database as a reference, flight by flight

**In short: the Rocket Flight Database adds no flight HPR Sim can use that RASAero II's own comparison page doesn't
already give.** Of its 28 flights, 17 fly a COTS solid motor, all from that page: the set that
[RASAero II's comparison flights][rcf] found for [M2.3c7, faster than sound](../decisions-and-roadmap.md#m2-3c7).
Of the 20 it puts at Mach 2.5 or below, 16 are COTS, all below Mach 1.25. Its 4 flights from Mach 1.5 to 2.5, the
band of [M1.14b](../decisions-and-roadmap.md#m1-14b), fly team or unlisted motors. Its admission rule (its
OpenRocket fork within ±10% of the measured apogee) kept out none of the 24 page flights that ship a `.CDX1` design.
The 11 page flights it lacks have no `.CDX1`. The rule did bite on historical flights: 3 in, 10 out and 1 failed
run. Counting the 10 turns the fork's mean error there from −2.97% to +17.2% to +26.9% (**Inference**). So its
measured apogees can serve as references; its selection and its fork's numbers can't. Its 3 historical flights
(Black Brant V, two Nike-Deacons) fly non-COTS sounding-rocket motors past Mach 4.9: outside scope now and later.

**What this covers.** The database's v1.2.0 release and its GitHub history [S1, S2], RASAero II's comparison page
[S3], NACA TN 3739 [S4] and an attempt at DTIC AD0733141 [S5], read 2026-10-17. **How far to trust it.** Every
count comes from the CSV, the page and the installer files cached for the earlier note. The fork's source was never
opened; only its repository metadata and README were read [S6]. The Black Brant row rests on the database alone.
HPR Sim flew nothing for this note. It builds on [ADR-236: supersonic flights from published comparisons][adr-236].

[rcf]: rasaero-comparison-flights.md
[adr-236]: ../decisions/0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[adr-143]: ../decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-162]: ../decisions/0162-the-suite-and-its-releases.md

## 1. What the database is

- **Data only, CC BY 4.0.** The repository holds 6 files: a CSV, a README, `CITATION.cff`, a LICENSE, a guide and
  `.gitignore` (tree listed 2026-10-17); no code, no geometry. The LICENSE is a CC BY 4.0 notice limited to the
  tabulation, the fork's predictions and the curation; GitHub's API calls it "Other" [S2]. The README treats the
  apogees as Rogers' work, "referenced rather than republished" [S1].
- **Two releases.** Zenodo lists versions of 2026-05-02 and 2026-06-03, named by their zips v1.0.0 and v1.2.0; their
  CSVs hold 25 and 28 flights [S1]. `CITATION.cff` dates v1.2.0 to 2026-05-11; its commit is dated 2026-06-02.
  GitHub's v1.0.0 release has no notes [S2].
- **One correction.** On 2026-06-03 MESOS 293K's fork apogee moved from 291,601 ft (−0.6%) to 273,070 ft (−7.0%).
  The commit says the old value had been re-pinned by hand over a fresh run [S2].
- **Not a second source.** In all 25 page rows, the measured and RASAero II apogees equal the page's to the foot
  (**Inference**: compared row by row).

## 2. The 28 flights

COTS means a ThrustCurve entry by designation and maker, as in [the earlier note][rcf]; OOP, out of production.
"Other Mach" is the page's measured value where given, else RASAero II's stored peak speed over the speed of sound
in the pad's air (**Inference**; a lower bound, by more on high flights, since air aloft is colder). `.CDX1` ships
with RASAero II. Data: B baro, O optical, G GPS, A integrated accelerometer, R radar.

| # | Flight | Motor, class | Fork Mach | Other Mach | ≤ 2.5 | Design | Page | Data |
|--:|---|---|--:|--:|---|---|---|---|
| 1, 2 | Thunder & Lightning, Gibb | AT I284W, COTS | 0.54, 0.55 | 0.55, 0.55 | yes | `.CDX1` | yes | B |
| 3 | Cancer Descending | AT M1297W, COTS | 0.56 | 0.56 | yes | `.CDX1` | yes | B |
| 4 | EZI-65 J450ST | AMW J450ST, COTS OOP | 0.60 | 0.60 | yes | `.CDX1` | yes | B |
| 5–7 | Caliber Isp 04, 3 teams | CTI I205, COTS OOP | 0.64–0.66 | 0.64–0.66 | yes | `.CDX1` | yes | B |
| 8 | Byrum | AT J570W, COTS | 0.75 | 0.72 | yes | `.CDX1` | yes | B |
| 9 | Ion Drive | AT K550W, COTS | 0.79 | 0.80 | yes | `.CDX1` | yes | B |
| 10, 12 | Caliber Isp 05, 2 flights | CTI I285, COTS | 0.81, 0.84 | 0.81, 0.85 | yes | `.CDX1` | yes | B |
| 11 | Blister | AMW K1075GG, COTS OOP | 0.83 | 0.83 | yes | `.CDX1` | yes | B |
| 13 | Rabia, short fin can | CTI L730, COTS | 0.86 | 0.87 | yes | `.CDX1` | yes | B |
| 14 | Raven | AT J570W, COTS | 1.07 | 1.07 | yes | `.CDX1` | yes | B |
| 15 | Rabia | AMW L1080BB, COTS OOP | 1.14 | 1.16 | yes | `.CDX1` | yes | B |
| 16 | Torrent | AMW M1850GG, COTS OOP | 1.22 | 1.23 | yes | `.CDX1` | yes | B |
| 17 | Kline-Rogers L500 | Ace L500, not listed | 1.98 | 1.96 | yes | `.CDX1` | yes | O |
| 18 | A-601 | P4935, team | 2.19 | 1.83 | yes | `.CDX1` | yes | G |
| 19 | Full Metal Jacket, BALLS | Kosdon O10000, not listed | 2.31 | 2.25 | yes | `.CDX1` | yes | O |
| 20 | Full Metal Jacket, Black Rock | Kosdon O10000, not listed | 2.46 | 2.41 | yes, near | `.CDX1` | yes | O |
| 21 | Proteus 6 | Loki-EX P9381, team | 2.87 | 2.94 measured | no | `.CDX1` | yes | A |
| 22 | AeroPac 104K, two stages | AT N1048 + M685W, not listed | 3.04 | 2.74 | no | `.CDX1` | yes | G |
| 23 | Don't Debate This | CTI N5800, COTS | 3.04 | 3.05 | no | `.CDX1` | yes | B |
| 24 | Qu8k | Deville-EX Q18000, team | 3.46 | 3.30 | no | `.CDX1` | yes | A |
| 25 | MESOS 293K, two stages | KIP-EX O4374 + M787, team | 4.33 | 4.18 measured | no | on request | yes | G |
| 26 | Black Brant V VB | Bristol 26KS20000, not COTS | 7.22 | not reached | no | fork repo | no | R |
| 27, 28 | Nike-Deacon, 2 flights | Nike M5 + Deacon, not COTS | 4.96, 5.08 | ≈5.3, ≈5.5 | no | TN 3739 | no | R |

- **Counts.** COTS: 17 (rows 1–16 and 23). Fork Mach ≤ 2.5: 20 (rows 1–20), 16 COTS. Past Mach 1 by the fork: 15,
  4 COTS. Data: 17 baro, 3 optical, 3 GPS, 2 accelerometer, 3 radar [S1].
- **The fork's Mach is a prediction [S1].** Against RASAero II's stored value it is within 0.03 on 17 of the 24
  shipped designs (Byrum at 0.030), 0.05 to 0.18 above on the two Full Metal Jackets, Qu8k and Proteus 6, and 0.36
  and 0.30 above on A-601 and AeroPac 104K. Against the page's measured Machs it is 0.07 low on Proteus 6 and 0.15
  high on MESOS [S3] (**Inference**: our arithmetic).
- **Designs.** 24 ship with RASAero II, which grants no rights to them; MESOS's file came from Rogers on request
  [S1]. For the Black Brant, the README points to a build script and dimension audit in the fork's GPL
  repository, not opened [S1, S6]. TN 3739 gives the Nike-Deacon's dimensions (Figure 2) and weights (p. 3) [S4].

## 3. The admission rule: what it let in and kept out

- **The rule came with v1.2.** The v1.0 README states none. v1.2 requires the fork within ±10% of the measured
  apogee and sends a miss back to the fork before any re-try [S1, S2].
- **Page flights: 25 in, 0 out by the rule.** All 24 page flights with a `.CDX1` are in, plus MESOS from Rogers'
  file. The guide imports `.CDX1` files only (Step 3) [S1]. The 11 it lacks are exactly those with no `.CDX1`
  ([the earlier note][rcf] §1), so none is shown to be kept out by the rule (**Inference**):
  - **`.ALX1` only (6):** Laser/LOC-2.1 twice, EZI-65 on the J125 and on the J100, McGarvey, Hubbub.
  - **No file (5):** Violent Agreement, its SS sustainer, ThreeCarbYen, Project R, Proteus 6.5. The guide calls
    ThreeCarbYen a "candidate for a future release" [S1].
- **Historical flights: 3 in, 10 out by the rule, 1 failed run.** The README names three candidates kept out at v1.2
  [S1]: a Nike-Cajun (1 flight, +16.6%), a 1965 Nike-Apache set (9 flights, +24% to +38%) and a Nike-Cajun Hurricane
  (1 flight; the run failed to integrate). That is 3 of 14 historical flights admitted, against 25 of 25 importable
  page flights (**Inference**: counting the nine Nike-Apache flights one by one).

## 4. What the rule does to the error statistics

- **RASAero II: the 11 left out were harder, but for want of a file.** From the page's numbers [S3; **Inference**]:

  | Page flights | Count | Mean signed | Mean absolute | Largest absolute | Within ±10% |
  |---|--:|--:|--:|--:|--:|
  | In the database | 25 | +2.46% | 5.34% | 11.52% | 22 |
  | Left out | 11 | +5.76% | 8.28% | 14.51% | 7 |
  | All | 36 | +3.47% | 6.24% | 14.51% | 29 |

  The 5.34% matches the README [S1]. The gap tracks the files, not the rule. The 6 `.ALX1` rows' predictions match
  those older files' comments ([the earlier note][rcf]; whether rerun is open), mean absolute 9.05%; the 5
  with no file average 7.36%.
- **The fork's column is truncated and worked on.** Its largest absolute error over the 28 is 8.70%. Its 28 of 28
  within ±10% is the rule itself; misses go back to the model, and MESOS was re-pinned once [S1, S2]. So its errors
  are in-sample, not a held-out score (**Inference**).
- **Sizing the cut.** The fork's mean absolute error over the 28 is 4.56% (4.55% in the README); its mean signed
  error on the 3 historical rows is −2.97%. With the 10 kept-out flights back, each Nike-Apache at an end of its
  range (**Inference**), the mean absolute error over 38 becomes 9.5% to 12.8% and the historical mean signed error
  +17.2% to +26.9% over 13 flights: the sign flips. If each end of the range is one flight's error, these tighten to
  9.85% to 12.43% and +18.3% to +25.8%. Within ±10% falls to 28 of 38, or 28 of 39 counting the failed run.
- **The README's headline mixes sets.** It gives the fork 4.55% over 28 and RASAero II 5.34% over 25 [S1]. On the
  same 25 page flights the fork scores 4.75% (**Inference**). It says the fork ran unmodified pre-flight files, yet
  AeroPac 104K's shipped file holds the tuned launch angle ([the earlier note][rcf]).
- **Correlated tools.** On the 25 page flights, the fork's and RASAero II's errors correlate weakly (Pearson 0.16,
  **Inference**). HPR Sim's aerodynamics are Barrowman-family, like OpenRocket's. A set screened by an OpenRocket
  fork could flatter HPR Sim by an amount no source bounds until HPR Sim flies these designs (**Inference**). On
  the page flights the screen removed none, so the bias there is zero by count. On the historical ones it removed
  11 of 14: 10 runs that came out high and 1 that failed.
- **Conclusion.** The measured apogees can serve as references on three conditions: eligibility comes from RASAero
  II's page and installer alone, as ADR-236 does; only measured apogees are used, never the fork's Machs or
  apogees; the database is cited for provenance. Then it adds only its 3 historical flights, all out of scope (§5).

## 5. The three flights not on RASAero II's page

- **Nike-Deacon (DAN), flights 1 and 2 (NACA TN 3739, Heitkotter, July 1956) [S4].** Both were launched at Wallops
  Island at 75° elevation: a Nike booster, an ABL Deacon sustainer (pp. 3–4).
  - **Apogees:** radar beacon peaks of 356,000 ft and 350,000 ft (p. 5). Flight 2's beacon was tracked only in
    patches, and its ascent was reconstructed (pp. 5–6).
  - **Speed:** peaks of 5,150 ft/s at 47,060 ft and 5,289 ft/s at 39,339 ft (p. 5). Over 968 ft/s, the speed of
    sound in the standard stratosphere, that is about Mach 5.3 and 5.5; the fork says 4.96 and 5.08 (**Inference**).
    Figure 11 plots measured coast Mach, from rawinsonde temperatures; its peaks were not read here.
  - **Precision, geometry, terms:** no stated precision (the CSV's "±1k ft" and "±5k ft" are the database's).
    Figure 2 gives dimensions and CG stations, p. 3 weights, Figure 11 coast drag against Mach. NTRS marks it public.
- **Black Brant V VB, AAF-VB-32 (Churchill, 3 March 1971; DTIC AD0733141).** DTIC gave HTTP 403 and then a
  maintenance page on 2026-10-17, so this NRC Canada report and its terms are unchecked [S5]. The 897,638 ft
  apogee, radar track, Bristol motor and Mach 7.22 are the database's [S1].
- **Out of scope, now and later.** All three fly non-COTS sounding-rocket motors, which the COTS-only scope bars
  before 1.0 ([ADR-162][adr-162] §5). All pass Mach 4.9 by any reading; the envelope ends at Mach 3.5
  ([ADR-143][adr-143]). Only a wider envelope could admit them, and then the Nike-Deacons' coast drag might serve
  better than their apogees (**Inference**).

## 6. M1.14b: each item reported or refused

ADR-143 §6b names the database as one M1.14b item; its other items are outside this note.

- **The database as a reference set: refused.** Its rule selects on an OpenRocket fork's error (§3–4).
- **Its 25 page flights: reported through M2.3c7.** Its 17 COTS flights are M2.3c7's own set; the 8 others fly team
  or unlisted motors. The CSV adds no apogee the page lacks.
- **Mach 1.5 to 2.5: none.** Its 4 flights there fly team or unlisted motors (rows 17–20). The best COTS flight in
  that band, Violent Agreement's SS sustainer, has no design ([the earlier note][rcf] §3).
- **Its 3 historical flights: refused,** being non-COTS and past Mach 3.5 (§5).
- **The fork's predictions, as an oracle: refused.** They are screened and tuned (§4), and the fork is GPL v3 by its
  README (GitHub's API says NOASSERTION), with no published release (none listed, 2026-10-17) [S6].
- **The public real-flight set: unchanged at 7.** The [real-flights report](../accuracy.md#real-flights) flies 7
  flights against logged traces in ERA5 weather. The database gives apogees only, with no log or date, so none
  joins that set. M2.3c7's apogee-only rows come from the page and the installer.

## What the project can use

- **Cite, and cache under `refs/`:** the CSV and README, under CC BY 4.0 with attribution to the database and to
  Rogers' page [S1]. Derived counts and statistics may be published; the page's own numbers stay cite-only.
  `THIRD-PARTY-NOTICES.md` has no row for the database yet, though ADR-236 calls for one: "cited for provenance;
  selection not used" would fill it.
- **For M2.3c7's statement of its selection.** The report can say that all 17 flights it scores are in the
  database, and that the database's rule removed none of the 24 page flights with a `.CDX1`. That turns a caveat
  into a count. It changes wording only; no number moves.
- **For M1.14b.** Close the database item with §6's verdicts. Its band, Mach 1.5 to 2.5, gains no COTS flight here.
- **TN 3739, after an envelope change only.** Its coast drag against Mach for a finned sustainer, from Doppler
  radar, could be a supersonic drag reference. That needs no COTS motor, since the drag comes from the coast.

## Open questions

- Which OpenRocket Plus commit made the v1.2 column? The guide promises it in release notes that are empty [S1, S2].
- Did the fork fly MESOS from Rogers' reconstructed file? What are the nine Nike-Apache errors one by one?
- What does AD0733141 give for the Black Brant's apogee and tracking error, and under what terms? DTIC was down.

## Sources

1. **S1.** A. Yu, *Rocket Flight Database* v1.2.0, CC BY 4.0, <https://zenodo.org/records/20531977>
   (`rocket-flight-database-v1.2.0.zip`, 11,955 bytes, sha256 `83bd1d9f…6fae`): `flight_comparison.csv`, README
   ("Inclusion criterion", "Methodology note"), `CITATION.cff`, `docs/reproducibility.md`; version list from
   <https://zenodo.org/api/records/20531977/versions>. Read 2026-10-17.
2. **S2.** <https://github.com/AidanSYu/rocket-flight-database>: tree, release v1.0.0, and commits `7f9eb6ff`
   (v1.0), `edce4fc7` (v1.2) and `cfbfc37d` (MESOS correction), with the v1.0 README and CSV. Read 2026-10-17.
3. **S3.** C. E. Rogers, "RASAero II Comparisons with Altitude Data", <https://www.rasaero.com/comparisons-alt.htm>,
   the table of 36 flights, cached 2026-10-16, read 2026-10-17.
4. **S4.** R. H. Heitkotter, *Flight Investigation of the Performance of a Two-Stage Solid-Propellant Nike-Deacon
   (DAN) Meteorological Sounding Rocket*, NACA TN 3739, 1956, pp. 3–6, Figures 2 and 11,
   <https://ntrs.nasa.gov/citations/19930084525>. Read 2026-10-17.
5. **S5.** NRC Canada, Black Brant AAF-VB-32 operations report, DTIC AD0733141,
   <https://apps.dtic.mil/sti/citations/AD0733141>, tried 2026-10-17: HTTP 403, then a maintenance page.
6. **S6.** <https://github.com/AidanSYu/openrocketsupersonic>: repository metadata (a fork of
   openrocket/openrocket), its releases list (empty) and its README (GPL v3). Read 2026-10-17; no source file opened.
