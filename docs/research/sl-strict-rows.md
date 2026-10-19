# More Student Launch predictions printed before their flights

**In short: 15 more strict rows on 12 flights narrow the strict OpenRocket interval from 17.8 points to 12.6, but it
still holds the collection's +6.82% median.** [Student Launch apogees](student-launch-apogees.md) §2 had 23 strict
rows: predictions printed in a report dated before the flight they predict. This note adds 15, from Purdue and USF
seasons that note didn't read and from five teams it didn't have: Oregon State, WPI, UAH, River City Rocketry and
Northeastern, plus a Vanderbilt subscale. Strict OpenRocket goes from 15 rows, median **+1.1%** (95% interval
−4.2% to +13.6%), to 26 rows, **+3.85%** (−0.3% to +12.3%). Every tool: 23 rows, +3.0%, to 38 rows, **+5.25%**
(+1.1% to +12.3%). Both intervals hold +3% and +6.82%, and +9.47% too. The new rows lean high (OpenRocket median
+7.3%), so the strict median rose; it didn't settle. **How sure:** high on each row (pages cited, re-read in a
separate pass); low on any pooled median: 14 teams, a few rows each, and the flagged rows move it by up to 1.85 points.

## How the rows were read

The parent note's rules, unchanged ([student-launch-apogees](student-launch-apogees.md#how-the-rows-were-read)):
- **Error** = simulated ÷ measured − 1, against the team's primary altimeter or its stated mean, raw (not
  converted to true height; see [sl-true-height](sl-true-height.md)).
- **Strict (A):** printed in a report dated before the flight: a CDR before the vehicle demonstration flight (VDF),
  an FRR before launch week or a later flight. No drag fitted to an earlier flight or subscale of the vehicle; a
  weighed mass is allowed. Unplanned mass growth stays in, as the parent keeps NC State 2020's heavy VDF.
- **Set apart:** a drag system that deployed, ballast or motor changed after the prediction, a build defect or a
  motor anomaly. Air brakes carried but idle stay in, as the parent keeps NC State 2025's VDF.
- **Flagged** (in the statistics, and also shown without): a reason the row may not test the tool, listed per row.
- Pages are PDF page indices. 95% interval on a median: order statistics x(k) and x(n−k+1), the largest k with
  1 − 2·P(Bin(n, ½) ≤ k − 1) ≥ 95%, as in the parent.

## 1. The new strict rows

Flight: s subscale, v VDF, r re-flight, f a flight after the FRR. Tool: OR OpenRocket, RAS RASAero II, RS RockSim,
own = team code. Predicted and measured in ft.

| team, season, flight | tool | predicted [report, date] | measured [source] | error | flag |
|---|---|---|---|---:|---|
| Purdue 2018–19 v, 2019-02-25 | OR | 4,853 [U19c p31, 2019-01-10] | 4,321 [U19fy p3] | +12.3% | |
| same flight | RAS | 5,209 [U19c p37] | 4,321 | +20.6% | |
| Purdue 2018–19 r, 2019-03-17 | OR | 4,853 [U19f p104, 2019-03-03] | 4,568 [U19L p7] | +6.2% | |
| Purdue 2020–21 v, 2021-02-27 | OR | 5,170.7 [U21c p65, 2021-01-04] | 5,187 [U21f p99] | −0.3% | |
| same flight | RAS | 5,240 [U21c p66] | 5,187 | +1.0% | |
| USF 2022–23 f, 2023-03-25 | OR | ≈4,500 [F23f p26, 2023-03-06] | 4,007 [F23a p3] | +12.3% | a |
| Oregon State 2017–18 v | OR | 5,199 [O18cf p1, 2018-01-12] | 4,845 [O18fp p33] | +7.3% | b |
| same flight | RS | 5,552.79 [O18c p68] | 4,845 | +14.6% | b |
| Oregon State 2018–19 v, 2019-02-22 | OR | 5,296 [O19c p70, 2019-01-11] | 5,079 [O19a p8] | +4.3% | |
| WPI 2018–19 v, 2019-03-03 | OR | 4,683 [W19c p38, 2019-01-11] | 4,031 [W19f p33] | +16.2% | e |
| UAH 2018–19 v, 2019-02-09 | OR | 4,806 [A19c p38, 2019-01-11] | 5,017 [A19f p37] | −4.2% | |
| UAH 2020–21 v, 2021-02-27 | OR | 4,022 [A21c p53, 2021-01-04] | 3,072 [A21f p66] | +30.9% | |
| River City 2014–15 launch week | OR | 2,919 [R15f p88–90, 2015-03-16] | 2,282 [R15p p9] | +27.9% | c |
| Northeastern 2014–15 launch week | OR | ≈3,000 [E15f p8–9, 2015-03-15] | 2,909 [E15p p2] | +3.1% | a |
| Vanderbilt 2015–16 s, 2015-11-14 | RS? | 4,900 [V16pdr p15, 2015-11-06] | 4,476 [V16c p105] | +9.5% | d |

Notes, by row:
- **Purdue 2019:** the FRR gives 43.5 lb for the vehicle's current state and its post-flight run [U19f p25, p29],
  against the CDR's 39.31 lb [U19c p24]. The FRR's own table reads 4,285 ft for the VDF (+13.3%) [U19f p30–31].
  The FRR estimates a full-scale Cd of about 0.63 from the subscale [U19f p27] but reprints the CDR's 4,853 ft
  for the re-flight, so that row wasn't refitted; its flysheet's 4,876 ft gives +6.7%. Its RASAero II value fell
  from the CDR's 5,209 ft to 5,078 ft (+11.2%) with no reason given [U19f p112], so that row is unclear, not counted.
- **Purdue 2021 VDF:** the CDR's 5° and 5 mph cell (VDF wind 6 mph [U21f p98]); at 0° the rows read +2.4% and
  +3.9%. The CDR calls its 10° and 10 mph case the most realistic (4,724.9 ft): −8.9%, which would move the strict
  OpenRocket interval's lower end to −2.0%. The air brakes were armed but "did not function" [U21f p97]. It flew
  49.2 lb, no ballast [U21f p98].
- **Oregon State 2019:** about 8 lb (16%) heavier than the CDR's 48.9 lb [O19cf p1; O19ff p1], and the team blamed
  a thrust curve below the motor's rating [O19f p67]: two errors of opposite sign. The air brake's electronics
  were off [O19f p38–39]. The three working altimeters span +4.0% to +4.4%.
- **WPI 2019:** the CDR added the mass its subscale showed was missing, a mass correction, not a drag fit
  [W19c p8, p35]; the vehicle still "turned out to be heavier than expected" [W19f p33]. Two of this note's three
  searches found this row; it is counted once.
- **UAH 2019:** the CDR used OpenRocket's own drag, no fixed Cd [A19c p40]; weighed mass 2.874% off the CDR's
  [A19f p20]. **UAH 2021:** about 19% mass growth (about 45 to 53.6 lb [A21c p13; A21f p66]); drag tabs on
  board, never deployed. The team's own code read +32.3% (4,063 ft), but it runs a fixed Cd of 0.48 of unstated
  origin [A21c p50], tied to the subscale's drag on p53, so that row is unclear, not counted.
- **Northeastern 2015:** the FRR slides give 3,076 ft (+5.7%) [E15fs p6]; ballast flown not stated.
- **Flags.** a: the prediction is the target by construction, since the ballast was sized to hit it. USF's FRR
  value is a plot, read by pixels (±15 ft), equal to its 4,500 ft target; only the backup altimeter recorded
  [F23a p5]. Northeastern's OpenRocket runs sized 181 g of ballast for 3,000 ft [E15f p8–9], after a flight
  the FRR calls "only a dozen feet off" [E15f p11]. b: the flown vehicle was 14 in.
  shorter and 1.9 lb heavier than the CDR's [O18cf p1; O18ff p1], a design change rather than growth; this
  flight's burn read 3.30 s against 4.40 s simulated [O18p p9]. The row takes the top of the CDR's 4,981 to
  5,199 ft wind range for a VDF flown in 17 mph; at 4,981 ft it reads +2.8%. c: flown from the team's own guide
  tower, which its FRR blamed for the VDF's shortfall [R15f p119–120]. d: the PDR names RockSim 9 as its tool
  [V16pdr p79] but not for this number; counted in every tool, not in RockSim's set. e: the FRR's flight section
  doesn't name the motor flown; the CDR's run used the L730, and the FRR lists an L1030 as the backup.

## 2. Statistics

| strict set | n | median | 95% interval | ranks (coverage) | high |
|---|---:|---:|---:|---|---:|
| OpenRocket, before | 15 | +1.1% | −4.2% to +13.6% | 4, 12 (96.5%) | 9 |
| **OpenRocket, now** | 26 | **+3.85%** | **−0.3% to +12.3%** | 8, 19 (97.1%) | 18 |
| OpenRocket, now, without flagged rows | 21 | +3.0% | −3.1% to +12.3% | 6, 16 (97.3%) | 13 |
| new OpenRocket rows alone | 11 | +7.3% | −0.3% to +27.9% | 2, 10 (98.8%) | 9 |
| every tool, before | 23 | +3.0% | −3.1% to +13.6% | 7, 17 (96.5%) | 15 |
| **every tool, now** | 38 | **+5.25%** | **+1.1% to +12.3%** | 13, 26 (96.6%) | 28 |
| every tool, now, without flagged rows | 31 | +3.4% | −0.3% to +12.3% | 10, 22 (97.1%) | 21 |
| one row per flight (OR first), now | 32 | +5.25% | +0.5% to +13.6% | 10, 23 (98.0%) | 23 |
| team-season medians, now | 30 | +4.3% | +0.5% to +13.6% | 10, 21 (95.7%) | 21 |
| team medians, now | 14 | +10.9% | +3.1% to +16.2% | 3, 12 (98.7%) | 12 |
| day-of (B) rows, one per flight (parent) | 35 | +3.7% | +1.1% to +7.2% | 12, 24 (95.9%) | 26 |

- The "before" rows reproduce the parent's §2 exactly (its 15 OpenRocket, 7 RockSim and 1 RocketPy strict rows).
- **Team medians, strict per flight:** NC State +3.85% (8 flights), Notre Dame −2.0% (5), USF +12.3% (3), Toledo
  +17.75% (2), Purdue +6.2% (3), Oregon State +5.8% (2), UAH +13.35% (2), and one flight each from Georgia Tech
  +14.8%, WashU −7.0%, FAMU-FSU +13.8%, WPI +16.2%, River City +27.9%, Northeastern +3.1%, Vanderbilt +9.5%.
- **RASAero II:** these are the first strict RASAero II rows (two, both Purdue VDFs): +20.6% and +1.0%, above
  the same flight's OpenRocket row by 8.3 and 1.3 points. Too few for an interval.

## 3. What it answers

**The question was whether more strict rows would separate a +3% bias from +9%. Not yet.** The strict OpenRocket
interval, −0.3% to +12.3%, holds +3%, +6.82% and +9.47%. It is 5.2 points narrower than before, and its lower end
moved up 3.9 points to just under zero.
- **Every tool, the strict median is now +5.25%**, and its interval (+1.1% to +12.3%) excludes zero. By team it
  is +10.9% (+3.1% to +16.2%). Without the seven flagged rows it falls to +3.4% (−0.3% to +12.3%).
- **Strict rows read higher than the day-of rows** (+5.25% against +3.7% per flight), as the parent expected:
  B rows can use a weighed mass and the day's wind, and were printed after the flight. The two intervals
  overlap, so the gap is a sign, not a measurement.
- **Inference: mass growth drives several of the new high rows.** WPI, UAH 2021, Oregon State 2019 and
  Purdue 2019 flew heavier than their CDR models (by up to about 19%); the CDR is printed before the vehicle is
  finished. Not sized per row, and River City (+27.9%) and USF (+12.3%) have other causes or no mass comparison.
  A strict CDR row tests the tool and the team's mass estimate together. Compare
  [as-built mass growth](as-built-mass-growth.md).
- **Inference:** if the strict OpenRocket median stays near +4% at this spread, excluding +6.82% takes roughly
  200 strict rows (the upper half-width shrinks about as 1/√n, from 8.45 points at n = 26 to under 2.97).
  A search of about 30 teams' archives found 11. The public archives can't settle it.

## What this suggests

- M7.5 and ADR-209 §6 should quote the public Student Launch figure as a range that holds both readings: strict
  OpenRocket +3.85% (−0.3% to +12.3%, n = 26) beside the per-flight +3.4% of all rows. Neither excludes +6.82%.
- Report strict errors split by whether the flown mass matched the prediction's. The mass term is the likely
  reason strict rows read higher, and it isn't the tool's.
- More rows won't come from public archives at the needed rate; a better lever is the mass split above, or
  flights whose mass and design are both known ([sl-weighed-mass-flights](sl-weighed-mass-flights.md)).

## Open questions

- **How much of the strict bias is mass?** Only some rows give both the CDR mass and the weighed flight mass.
  Re-simulating each strict flight at its flown mass needs the design file, which no team published.
- **B rows found here** (about two dozen, −9.1% to +38.6%) were logged in the working parts but not added
  to the parent's 55; doing so needs the parent's per-flight choice rules applied to each.
- **Not reached:** Wayback pattern searches time out on large university domains (umn.edu, vanderbilt.edu,
  cornell.edu, louisville.edu, uah.edu, northwestern.edu); GitHub code search hit its rate limit; Oregon State's
  2018 FRR share link returns 404; WPI has no 2021 FRR; Rose-Hulman's wiki needs a login. Minnesota, Iowa State,
  NU Stars, Harding (2009–10 only), Kennesaw, Tennessee Tech, Akron, Wichita State and Cedarville gave no reports.
- **NASA's launch-week releases** name only the altitude-award winners, by distance from the declared target,
  so they give no paired rows. One check holds: NASA's 2019 release puts Oregon State 48 ft from its 4,500 ft
  target, matching its PLAR's 4,548 ft.

## Sources (read 2026-10-18)

No team report states a license; the two Oregon State site repositories are MIT on GitHub, but the reports in
them carry no license of their own. All are cited, not committed; reading copies stay in the gitignored cache.
Wayback raw form: `https://web.archive.org/web/<timestamp>id_/<url>`.

1. **U** Purdue, <https://purdueseds.space/wp-content/uploads/psp-sl/>: 2019 `walker/{cdr,frr,lrr}/Purdue%20`
   `University%20-%202019%20{CDR,FRR,LRR}%20Report.pdf` (U19c, U19f, U19L) and `walker/frr/…FRR%20Flysheet.pdf`
   (U19fy); 2021 `voss/{cdr,frr}/Purdue%20University%20-%202021%20{CDR%20Report,FRR%20Report}.pdf` (U21c, U21f).
2. **F** USF SOAR 2022–23, <https://web.archive.org/web/2024id_/https://usfsoar.com/wp-content/uploads/2023/06/>
   `SOAR-NSL-{FRR,FRR-Addendum}-2022-23.pdf` (F23f, F23a).
3. **O** Oregon State: <https://github.com/OSU-USLI-18/OSU-USLI-18.github.io> at `16cfe0c68df5`, `deliverables/`
   (CDR O18c, CDR flysheet O18cf, FRR flysheet O18ff, FRR slides O18fp, PLAR O18p);
   <https://github.com/OSU-USLI-19/OSU-USLI-19.github.io> at `5f1ab6dbfa02`, `deliverables/2019/` (CDR O19c,
   flysheets O19cf, O19ff, FRR O19f, addendum O19a).
4. **W** WPI, `aiaa.wpi.edu/static/hprc/goats/Worcester%20Polytechnic%20Institute%20-%202019%20-%20`
   `{CDR,FRR}%20Report.pdf` (W19c, W19f), Wayback 20240705152926 and 20240705105916; the host no longer resolves.
5. **A** UAH Charger Rocket Works, <https://chargerrocketworks.files.wordpress.com/>: `2020/01/uah-2019-`
   `{cdr-report,frr-report}.pdf` (A19c, A19f); `2021/05/university-of-alabama-in-huntsville-2021-{cdr,frr}-`
   `document.pdf` (A21c, A21f).
6. **R** River City Rocketry, <https://www.rivercityrocketry.com/documents/2014-2015/reports/>`{frr,plar}.pdf`.
7. **E** Northeastern AIAA, `https://web.northeastern.edu/aiaa/wp-content/uploads/2014/10/Northeastern_USLI_`
   `2014_2015_{FRR_Report,FRR_Presentation-With_Motor_Change,PLAR_Report}.pdf` (E15f, E15fs, E15p), Wayback 2017.
8. **V** Vanderbilt, `http://www.vanderbilt.edu/USLI/2016/Reports/`: `Vanderbilt%202016%20PDR.pdf` (V16pdr,
   Wayback 20160520153404) and `Vanderbilt%20University%202015-2016%20CDR%20Report.pdf` (V16c, 20160520152808).
9. **NASA** 2019 Student Launch preliminary winners release (2019-04-10),
   <https://www.nasa.gov/news-release/nasa-announces-preliminary-winners-of-2019-student-launch-competition>.
