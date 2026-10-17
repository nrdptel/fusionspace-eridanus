# How often each fault happens

How often does a hobby or high-power flight end in a fault that fault diagnosis (M7.4 in the
[roadmap](../ROADMAP.md)) ranks: a CATO or other motor failure, no drogue or main, a separation, an early or late
deployment, a structural failure, an unstable boost? A ranking by evidence needs these base rates as priors. Each rate
has its count, denominator, selection and 95% Clopper–Pearson interval. It builds on
[public logs of faulty flights](anomalous-public-flights.md) (single cases) and [motor data](motor-data.md) §5 (NAR's
retests). Read 2026-10-17: reports, slide decks, launch logs, score workbooks and MESS downloads; no source code was
opened. **Stated** marks a number the source prints, **Inference** this note's arithmetic or reading. How far to trust
it (Inference, from the spread between sources): a factor of 1.5 for model rockets, 3 for high power; all undercount.

**In short.**
- **Sport flying fails on about 1 flight in 12, mostly in recovery.** NAR's 2005 study of 6,169 club flights, mostly
  A to D, found 8.5% failed unsafely (7.8% to 9.2%), 17.4% of staged or clustered ones; separation (about 2.3% of
  flights), lawn dart (about 2%), unstable boost (1.6%) and no chute (1.3%) lead, a CATO about 0.5% [L1]. A second
  club's 5,092 flights: 4.5% recovery failures (4.0% to 5.1%), 1.06% CATOs (0.80% to 1.38%) [N8].
- **A motor fails dramatically on 0.5% to 1.7% of flights, higher above D.** Counted for this note from one club's
  147 launch logs (13,674 flights, 1999 to 2013): 0.52% for D and smaller, 1.7% for E to G, 1.6% for H to J (375
  flights, 0.59% to 3.45%) [L2]. A partly lit cluster or stage: 2.6% of flights with several motors. MESS's 2,512
  reports have no denominator, so they give the mix of failure modes, not a rate [N3].
- **Student high-power competition loses far more rockets, on a broader measure.** At the Spaceport America Cup, 27%
  to 48% of scored flights (by year; 63 of 193 in 2023 and 2024) were damaged past flying again, hard landings under
  a good chute included [E1]. **No tally bounds** late deployment, tangled chutes, flutter or coning per flight (§5).

**Interval.** For k events in n trials: [B(0.025; k, n−k+1), B(0.975; k+1, n−k)], B the beta quantile; the lower
p solves P(X ≥ k) = 0.025, the upper P(X ≤ k) = 0.025, X binomial; found by bisection (10/100: 4.90% to 17.62%).

## 1. Club flight logs: NAR's 2005 study

NAR's special committee coded the Minnesota Amateur Spacemodeler Association's (MASA's) launch-officer flight cards:
6,169 flights at over 60 launches, late 1998 to mid-2005 [L1, pp. C-2 to C-3]. Only unsafe or potentially unsafe
outcomes count. Drag separation, hard landings, tangled chutes and early or late delays are excluded, and a flight with
several faults counts once, as its most severe [L1, pp. C-4 to C-5]. Only 5.2% of flights were G or above [L1, p. 11],
and the author expects underreporting [L1, p. C-4]. **Bias: low**, by an unbounded amount for dual-deploy high power.

| Outcome (MASA's definitions) | Count | n | Rate | 95% CI |
|---|---|---|---|---|
| Any failure (Stated, p. 11) | 523 | 6,169 | 8.5% | 7.8% to 9.2% |
| Simple rockets (Stated 7.7%, p. 12; n p. C-14) | 433 (Inference) | 5,651 | 7.7% | 7.0% to 8.4% |
| Complex: staged or clustered (Stated 17.4%) | 90 (Inference) | 518 | 17.4% | 14.2% to 20.9% |
| Powered phase; after it (Stated, p. 12) | 143; 380 | 6,169 | 2.3%; 6.2% | 2.0% to 2.7%; 5.6% to 6.8% |
| Lawn dart, simple (Stated, p. C-14) | 70 | 5,651 | 1.2% | 1.0% to 1.6% |
| Lawn dart, complex (Stated, p. C-14) | 52 | 518 | 10.0% | 7.6% to 13.0% |

The other modes are printed only as whole-percent shares of the 523 failures [L1, p. 11]. The counts are those that
round to each share; the interval runs from the lowest count's lower bound to the highest's upper bound (Inference).

| Mode (definition, p. C-5) | Share | Counts | Per flight | 95% CI (outer) |
|---|---|---|---|---|
| Separation: parts fall, one unslowed; stripped chute included | 28% | 144 to 149 | 2.3% to 2.4% | 2.0% to 2.8% |
| Unstable: nose-down in boost; coning only if it crashed | 19% | 97 to 101 | 1.6% | 1.3% to 2.0% |
| No chute: nothing deployed, not ballistic; tangled excluded | 16% | 82 to 86 | 1.3% to 1.4% | 1.1% to 1.7% |
| Motor CATO (kind not recorded consistently, p. 10) | 6% | 29 to 33 | 0.5% | 0.32% to 0.75% |
| Core sample: ballistic with the nose off | 5% | 24 to 28 | 0.4% to 0.5% | 0.2% to 0.7% |
| Shred: comes apart during ascent | 2% | 8 to 13 | 0.1% to 0.2% | 0.1% to 0.4% |
| Motor unrestrained: motor exits at ignition or boost | 1% | 3 to 7 | 0.05% to 0.11% | 0.01% to 0.23% |

"Lawn dart" (no ejection, power prangs, unstaged boosters) is in complex rockets mostly failed staging or partial
cluster ignition [L1, pp. C-5, C-14]. One-motor CATO rates by class, A 0.7%, B 0.0%, C 0.4%, D 0.8%, E 0.4%, F 0.7%,
G 2.1%, H 1.8% [L1, Table 3, p. C-20], have only charted denominators. A second club's 4,546 flights record 15%,
tangled chutes and early ejections included [L1, pp. 11, C-12]. 2005 TARC finals: 11 of 103 rockets had a ballistic
return of some part, 10.7% (5.5% to 18.3%) [L1, pp. 12, C-15].

## 2. Club flight logs counted for this note

**MASA, 1999 to 2013 [L2].** This note parsed MASA's 147 per-launch PDF logs into 13,674 flight rows, equal to each
log's printed total on 145 of 147 and to the study's yearly totals within 7 flights for 1999 to 2002 and 2004. A
**motor event** is a comment naming a CATO, spit motor, nozzle or closure out, blow-by or burn-through; class is from
impulse per motor. The logs carry flyers' names and "All Rights Reserved": cached privately, aggregates only.

| Class | Flights | Motor events | Per flight | 95% CI |
|---|---|---|---|---|
| 1/4A to D | 11,276 | 59 | 0.52% | 0.40% to 0.67% |
| E to G | 2,011 | 34 | 1.69% | 1.17% to 2.35% |
| H to J | 375 | 6 | 1.60% | 0.59% to 3.45% |
| All (12 rows unclassed) | 13,674 | 103 | 0.75% | 0.62% to 0.91% |

- **Checks:** two of the 103 are queried ("blow by?"); without them E to G is 33 (1.64%), H to J 5 (1.33%, 0.43% to
  3.08%). Per motor fired, 0.67%; 1999 to 2005, 0.51%, as the study; 2006 to 2013, 0.96%, mostly from E to G.
- **Partial ignition of clusters and stages** ("only lit 1", a sustainer that did not light): 29 comments over 1,129
  flights (1,111 logged with several motors, 18 logged as one because only one lit), **2.6% (1.73% to 3.67%)**, one a
  late light; a misfire re-tried on the pad is not a logged flight.
- **Ejection-timing comments** are floors, not rates: early 43, late 93, none 60 of 13,674 flights (0.31%, 0.68%,
  0.44%). They mix motor delay faults with delay choice and packing (Inference).

**NOVAAR, 2007 to 2009 [N8, slide 9].** All 5,092 flights on the Northern Virginia club's range ("unsafe" undefined):
any unsafe 374, 7.3% (6.6% to 8.1%); recovery (ballistic return to separated recovery system, not split) 230, 4.5%
(4.0% to 5.1%); motor CATO 54, 1.06% (0.80% to 1.38%); unstable 41, 0.8% (0.6% to 1.1%); shredded 26, 0.5% (0.3% to
0.7%). A 2017 NAR deck's 8.5% of "over
20,000" reports, three quarters in recovery [N9, slide 6], has no exact count and may restate [L1].

**A high-power-heavy club, 2024 [L3].** The Champlain Region club's 2024 reports (9 launches, 261 motors, over half H
or larger) name a CATO or motor failure in 5 notes: **5 of 261 motors, 1.9% (0.62% to 4.4%)**. Other years not counted.

## 3. Bench tests and MESS

**NAR's retests, 2023 to 2026 [N1].** All 18 retest sheets hold 156 firings with 4 CATOs, 2.6% (0.70% to 6.4%): Estes
E12 3 of 8 (8.5% to 76%), AeroTech E16W 1; without E12, 1 of 148, 0.68% (0.02% to 3.7%). They also flag 18 of 125
delay firings out of tolerance (14%, 8.8% to 22%) and 6 firings low in impulse, all on one type. All 18 types are F
or smaller. **Bias: high.** NAR S&T picks engines and lots from MESS reports [N7], may recall a type with 20% of all
MESS reports in 12 months [N2, §6.5, p. 12], and the E12 is MESS's most reported.

**Certification gives no rate.** A new type fires 3 per delay, 2 above 5,120 N·s [N2, §3.2 to §3.3, p. 7]; NFPA 1125
asks 10 or more [N6, FR-7, p. 5]; a CATO can halt testing [N2, §8.3.17, p. 18]. 0 of 10 bounds the rate below 31%.

**MESS [N3].** The joint NAR, Tripoli and CAR survey takes voluntary online reports. The 27 per-maker downloads of
2026-10-17 sum to 2,512 reports (AeroTech's 571 matches its search page), 2005-09 to 2026-10, about 182 a year in
2014 to 2025; 955 accepted, 1,557 pending; 1,444 (57%) Estes, the E12 the most with 385. The form asks for no flights
or sales, so **no rate can be formed**; it gives the mix of modes (shares of reports; 1,201 tick several).

| Class | Reports | Case-type | Burn-through | Ejection or delay only | Reloads |
|---|---|---|---|---|---|
| 1/4A to D | 936 | 68% | 5% | 16% | 2% |
| E to G | 1,116 | 85% | 6% | 8% | 9% |
| H to J | 333 | 71% | 23% | 17% | 69% |
| K to O | 127 | 72% | 36% | 0% | 83% |

Case-type: nozzle out, forward end out, casing split, burning propellant out or burn-through (class from the
designation when the name has none). Dramatic modes are likely over-reported, so the mix leans case-type. Of MESS's 8
notices, the 7 read [N4, N5] are mostly lot defects: failures cluster by lot, not independently (Inference).

## 4. Student high-power competition

**Spaceport America Cup (IREC) score workbooks [E1].** University teams fly to 10,000 or 30,000 ft after design and
flight-readiness reviews, on COTS or student-built motors. One row per team; this note counted rows with a recovery
outcome: in 2023 and 2024 a damage status with all-or-nothing points, in 2019 a 0 or 1 column. "Excessive damage"
means it could not fly again safely once consumables are replaced [E3, p. 36]; that mixes every cause (no or late
deployment, separation, breakup, CATO, a hard landing under a good chute), and this note does not split it.

| Year (team rows) | Scored flights | No recovery credit | Rate | 95% CI | COTS | Student motors |
|---|---|---|---|---|---|---|
| 2024 (122) | 107 | 40 | 37.4% | 28.2% to 47.3% | 29/84, 34.5% | 10/22, 45.5% |
| 2023 (119) | 86, plus 1 "Unknown" | 23 | 26.7% | 17.8% to 37.4% | 17/67, 25.4% | 6/19, 31.6% |
| 2019 (107) | 65 | 31 | 47.7% | 35.1% to 60.5% | 20/41, 48.8% | 11/24, 45.8% |
| 2023 and 2024 | 193 | 63 | 32.6% | 26.1% to 39.7% | | |

In 2019 a row counts as flown if it has an apogee or a recovery credit (Inference). **Bias: low.** ESRA's launch desk
logged 118 launches in 2024 [E2, pp. 1, 4], 11 more than the scored rows: 40 of 118 if they all ended well, 33.9%;
51 of 118 if none did, 43.2% (Inference). Four 2024 statuses name a mode, floors only: 1 shred, 2 CATOs, 1 lawn dart.
A main at or near apogee once cost half the recovery points [E4], but no half score appears in 2023 or 2024.

**European Rocketry Challenge: no official table**, three definitions. 2021: 8 total losses in 15 flights, 53% (27% to
79%) [U1]; 17 of 19 teams not recovered "in good condition", 89% (67% to 99%) [U2]. 2025: 13 of 15 launches without a
"nominal" recovery, 87% (60% to 98%), "nominal" undefined [U3, p. 1].

## 5. Not bounded by any public tally

- **Early or late deployment, main at apogee, tangled chutes, shock cords:** excluded by MASA's study, not split by
  the second club, inside IREC's "excessive damage"; MASA's comments give floors. **Flutter, zippers:** uncounted,
  inside shred (0.1% to 0.4%, NOVAAR 0.5%). **Coning:** excluded unless it crashed [L1, p. C-5].
- **Student Launch, Tripoli, high-power motors:** no counted outcomes, no source with thousands of H to O flights,
  no CATO counts from certification. **Selection:** club logs low (unwritten comments, exclusions) and high (rocket
  faults called CATOs), both unbounded; retests high; certification low (survivors); IREC low (unscored flights).

## What this suggests

1. **M7.4's priors as P(mode | fault).** M7.4 sees a flight already known to be odd, so failure shares are the direct
   prior: of MASA's 523, separation 28%, lawn dart (staging, no ejection) about 23%, unstable 19%, no chute 16%, CATO
   6%, core sample 5%, shred 2% [L1, p. 11]; for high power, motor modes by MESS's H to O mix (§3), stated as such.
2. **Per-flight priors:** motor event 0.5% (D and smaller), 1.7% (E to G), 1.6% (H to J, 0.6% to 3.5%); a partly lit
   cluster or stage 2.6%; any failure 8.5% for sport; a third damaged past flying again in student competition.
3. **The synthetic benchmark's fault mix** (M7.4's 200 flights) could follow item 1, reported beside a uniform mix.
4. **Code MASA's cached 1999 to 2013 logs for recovery modes**: double the study's denominator, rates by class.

## Open questions

- Flights per class behind [L1] Table 3; thousands of H to O flights; motors sold per type (to make MESS rates);
  Student Launch outcome counts; why MASA's E to G events doubled after 2005; NFPA 1122 and 1127 annexes (unread).

## Sources

None states a license except [E3] (Creative Commons 4.0 International, ESRA); all are cached privately and cited.
- [L1] NAR special committee, "Launching Safely in the 21st Century", 2005-10-29, pp. 10 to 12, App. C (T. Cochran):
  <https://assets.zyrosite.com/Yg21M7RM7MSaQpNz/launchsafe-TQxWMfjtrNVQrGeU.pdf>.
- [L2] MASA launch logs, 1998 to 2013, "All Rights Reserved": <https://masa-rocketry.org/launch-statistics>.
- [L3] Champlain Region Model Rocket Club, 2024 launch reports: <https://crmrc.org/2024_launch_reports.htm>.
- [N1] NAR S&T retest sheets, 18, 2023 to 2026 ([motor data](motor-data.md) S24), for example
  <https://www.nar.org/docs.ashx?id=1771858>.
- [N2] NAR Motor Testing Manual v1.5 (2011), pp. 7, 12, 18 (the URL now refuses; a cached copy was read):
  <https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf>.
- [N3] MESS: <https://www.motorcato.org/>, its `/latest` and `/report` pages and `/search.csv?manufacturer=<maker>`.
- [N4] MESS notices 5, 7, 9, 10, 11: <https://www.motorcato.org/notification/5>; [N5] its Cesaroni bulletins.
- [N6] NFPA 1125 (A2021) First Revision text (2019-10-25, not the issued edition), FR-7, p. 5:
  <https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf>.
- [N7] NAR S&T committee page, cached 2026-10-11 for [motor data](motor-data.md) S3 (live now lists motors only):
  <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=670148>.
- [N8] NAR, "Safety in Sport Rocketry: Training for RSO and Safety Check", 2013, slides 6 to 9, 2015-04-21 archive:
  <http://www.nar.org/wp-content/uploads/2014/05/Safety-Training-for-RSO-2013.pdf>.
- [N9] NAR, "Safety in High Power Rocketry", 2017-02 (file credits T. Cochran), slide 6, 2022-04-19 archive:
  <https://www.nar.org/wp-content/uploads/2017/03/NAR-on-HPR-Safety-Feb-2017.pdf>.
- [E1] ESRA score workbooks: 2024 final (rev 4, 2024-07-05), "Overall"; 2023 master (v2), "Teams"; 2019 scoring
  matrix, "Score Input"; from <https://www.soundingrocket.org/2024-sa-cup.html> and its 2023 and 2019 pages.
- [E2] C. Anderson, "IREC 2024 Post-Competition DFMS Analysis", ESRA, 2024-10-25, pp. 1, 4:
  <https://www.soundingrocket.org/uploads/9/0/6/4/9064598/irec_24_dfms_analysis.pdf>.
- [E3] ESRA, SA Cup Design, Test & Evaluation Guide, 2023 v2.2.8, p. 36, "Excessive Damage":
  <https://www.soundingrocket.org/uploads/9/0/6/4/9064598/2023-sa_cup_dteg_v2.2.8_10-8-23.pdf>.
- [E4] ESRA, IREC rules page (undated, a 2014 note), "Recovery": <https://www.soundingrocket.org/rules.html>.
- [U1] University of Bath blog, 2022-03-03 (archived): <https://blogs.bath.ac.uk/engdes-student-insights/?p=1445>.
- [U2] EPFL news, 2021-10-27: <https://actu.epfl.ch/news/epfl-rocket-team-crowned-european-champion-of-ro-3>.
- [U3] G. Kurzmann et al., "Lessons from ISPIDA", SSEA 2026, p. 1: <https://doi.org/10.14459/2026md1857554>.
