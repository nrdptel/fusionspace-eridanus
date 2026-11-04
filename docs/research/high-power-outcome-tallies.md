# Fault rates on high-power flights from published launch reports

How often does a high-power (H to O) flight end in a motor failure, a recovery failure or a structural failure?
Fault diagnosis (M7.4 in the [roadmap](../ROADMAP.md)) ranks faults by evidence and needs these rates as priors.
[How often each fault happens](fault-base-rates.md) found only 375 H to J flights (one club's logs, a motor event on
1.6%, 0.59% to 3.45%) and one 261-motor club year; [recovery modes in MASA's logs](masa-recovery-modes.md) coded those
logs by mode. This note counts two clubs' multi-year launch reports with the same rules, 3,307 H to O motors or
flights in all, and lists the sources that turned out to give denominators without outcomes. Read 2026-11-04; no
source code was opened. **Stated** marks a number a source prints, **Inference** this note's count or arithmetic.
Intervals are Clopper–Pearson 95% (the definition is in [fault-base-rates.md](fault-base-rates.md)). How far to trust
it: every rate is a floor, since a fault counts only if someone wrote it down, and the two clubs differ by more than
their intervals allow, so a single pooled number hides a factor of about 2.4 between clubs.

**In short.**
- **A motor fails on 1.1% to 2.7% of high-power flights, depending on the club.** The Champlain Region club's
  reports, 2008 to 2026: 32 of 1,184 H to O motors, **2.70% (1.86% to 3.79%)** [C1]. The Spaceport Rocketry
  Association's newsletters, 2020 to 2026: 24 of 2,123 H to O flights, **1.13% (0.73% to 1.68%)** [S1]. The two
  differ (Fisher exact p = 0.001, Inference). Pooled with MASA's 6 of 375: 62 of 3,682, 1.68% (1.29% to 2.15%), close
  to the old prior but narrower than the spread between clubs justifies (§3).
- **Certification flights fail least.** At SRA, motor events are 4 of 974 on L1 or L2 attempts (0.41%, 0.11% to
  1.05%) against 16 of 878 on other H to J flights (1.82%, 1.05% to 2.94%); the second is not shown to differ from
  Champlain's H to J rate, 27 of 1,005, 2.69% (p = 0.22, Inference). This fits part of the gap between clubs; it is
  not a measured cause (§3).
- **Recovery fails on 3% to 9% of high-power flights, structure on about 0.5% to 0.8%.** Champlain: recovery 98 of
  1,184, 8.28% (6.77% to 9.99%), mostly a main or chute that never came out (5.24%); shred or fin loss 0.84% (0.41% to
  1.55%). SRA: recovery 3.20% (2.50% to 4.04%), structural 0.47% (0.23% to 0.86%), plus 116 flights (5.46%) noted only
  as "failed"; if all were recovery faults, recovery would reach 8.67%. No LDRS, BALLS or Airfest tally with outcomes
  was found (§4).

## 1. Champlain Region Model Rocket Club, 2008 to 2026 [C1]

A northeastern US club posts one launch-report page per year; 2012 and 2013 list no club launch. 135 club launches
were kept (outreach days, off-site events and 3 launches with no tally dropped). **Denominator:** the motor tally by
class the page prints for 118 launches (Stated); for the other 17, a count of motor designations in the text
(Inference, within about 15% of five stated totals; these 17 add 317 motors and move no rate). Each motor in a
cluster or stage counts, so rates are **per motor**; on the 57 launches that state both, motors run 6% above flights
(1,520 on 1,433, Inference from Stated tallies), so per flight the rates would be about 6% higher. A fault counts
once, in the band of the flight's largest motor.

**Coding** follows [masa-recovery-modes.md](masa-recovery-modes.md): motor event (CATO, nozzle or closure failure,
burn-through, case failure); recovery (no chute or main, separation, lawn dart, core sample, a tangle that caused
damage); structural (shred, fin loss); zipper; unstable boost. A failed staging or partly lit cluster is a lawn dart.
Not coded: a bare spit, a tangle without damage, a hard landing. Three keyword passes, each read by hand, gave 327
club event rows (Inference).

| Mode, per motor (Inference) | Up to G, n 1,982 | H to J, n 1,005 | K to O, n 179 | H to O, n 1,184 |
|---|---|---|---|---|
| Motor event | 15, 0.76% | 27, 2.69% (1.78–3.88%) | 5, 2.79% (0.91–6.40%) | 32, 2.70% (1.86–3.79%) |
| Recovery | 135, 6.81% | 85, 8.46% | 13, 7.26% | 98, 8.28% (6.77–9.99%) |
| No chute or main | 52, 2.62% | 55, 5.47% | 7, 3.91% | 62, 5.24% (4.04–6.66%) |
| Lawn dart or core sample | 49, 2.47% | 20, 1.99% | 3, 1.68% | 23, 1.94% (1.24–2.90%) |
| Separation | 30, 1.51% | 7, 0.70% | 3, 1.68% | 10, 0.84% (0.41–1.55%) |
| Structural (shred, fin loss) | 5, 0.25% | 7, 0.70% | 3, 1.68% | 10, 0.84% (0.41–1.55%) |
| Zipper | 5, 0.25% | 4, 0.40% | 1, 0.56% | 5, 0.42% (0.14–0.98%) |
| Unstable boost | 27, 1.36% | 2, 0.20% | 1, 0.56% | 3, 0.25% (0.05–0.74%) |
| Any of these, once a flight | 166, 8.38% | 116, 11.54% | 22, 12.29% | 138, 11.66% (9.88–13.62%) |

- **Check:** the 2024 subset gives 5 motor events in 261 motors, 1.92%, as [fault-base-rates.md](fault-base-rates.md)
  counted. On the 118 stated-tally launches alone (1,068 H to O motors): motor events 2.81%, recovery 8.43%.
- **Coder accuracy (Inference):** 50 of 50 randomly drawn first-pass events and 20 of 20 later-pass events are real
  faults (lower bounds 92.9% and 83.2%). A fresh draw of 50 uncoded comments found 1 clear miss (since added) and 1
  borderline case, a miss rate of 2% per comment (0.05% to 10.65%); scaled to the pool, tens of missed faults against
  304 coded, not tightly bounded.
- **Trend (Inference):** H to O motor events, 6 of 389 in 2008 to 2018 (1.54%) and 26 of 795 in 2019 to 2026
  (3.27%), p = 0.09. 2022 and 2023 carry 14 of the 26 (6.17% of 227 motors); one 2022 launch reports four motor
  failures (Stated), one 2023 launch has five CATOs in 39 H to O motors. The other later years give 2.11%. Recovery
  is flat between eras (7.71%, 8.55%); a weak sign that H to O faults were reported about as fully in both.
- **Completeness:** before late 2019 the pages describe notable flights and list the rest; from late 2019 most
  flights carry a one-line result. One 2019 launch states only 8 of its 14 flights went as planned (Stated).

## 2. Spaceport Rocketry Association, Florida, 2020 to 2026 [S1]

A Florida Space Coast club posts one PDF newsletter per launch with a table of every flight: flyer, rocket, motor and
a free-text note. The directory listed 74 PDFs on 2026-11-04; 69 were used (3 cancelled launches, one duplicated
table and one misfiled copy dropped), 4,478 flight rows (Inference). **Parse check:** 37 of the used newsletters print
a launch total; they sum to 2,596 flights against 2,566 parsed rows, 1.2% fewer (Inference). Class is the highest
impulse letter among the motor designations, with certification levels ("L1 Cert") stripped first; rows with no
readable designation are left out. Rates are **per flight**.

**Coding.** Every text cell was scanned for fault words, since some recorders wrote the outcome in the motor cell;
each hit was coded by hand, with §1's motor-event rules. Recovery is no chute or main, lawn dart, separation, or a
tangled or shredded chute. Deployment by the motor's ejection charge ("motor eject", 18 rows) is not a fault; 17
ambiguous rows are held out of every rate. An empty note is taken as no fault reported.

| Mode, per flight (Inference) | H to J, n 1,852 | K to O, n 271 | H to O, n 2,123 |
|---|---|---|---|
| Motor event | 20, 1.08% (0.66–1.66%) | 4, 1.48% (0.40–3.74%) | 24, 1.13% (0.73–1.68%) |
| Recovery (see coding) | 58, 3.13% (2.39–4.03%) | 10, 3.69% (1.78–6.68%) | 68, 3.20% (2.50–4.04%) |
| Main at apogee (may be intended) | 16, 0.86% | 2, 0.74% | 18, 0.85% (0.50–1.34%) |
| Structural (shred, "structural failure") | 9, 0.49% | 1, 0.37% | 10, 0.47% (0.23–0.86%) |
| Unstable boost | 11, 0.59% | 0 (0–1.35%) | 11, 0.52% (0.26–0.93%) |
| "Failed", mode not stated | 108, 5.83% | 8, 2.95% | 116, 5.46% (4.54–6.52%) |
| Any noted fault, once a flight | 206, 11.12% (9.73–12.64%) | 23, 8.49% (5.46–12.46%) | 229, 10.79% (9.50–12.18%) |

- **Certification flights.** 974 H to J rows are L1 or L2 attempts (a cell says "cert"), 878 are not. Motor events:
  4 of 974, 0.41% (0.11% to 1.05%), on four different motor types, against 16 of 878, 1.82% (1.05% to 2.94%).
  Recovery is alike (3.29% against 2.96%); unspecified "failed" is higher on cert flights, 6.78% against 4.78%.
- **Runs on one type.** 4 of the 20 H to J motor events are on one J designation at four 2022 launches, three as a
  two-stage booster; without them, 16 of 1,852, 0.86% (0.49% to 1.40%). A run like this suggests failures cluster by
  type or lot; with no per-type denominator it is not measured.
- **COTS only.** 12 H to O rows name a hybrid, research or experimental motor, with 1 motor event; dropping them gives
  23 of 2,111, 1.09% (0.69% to 1.63%), the figure for the project's COTS-only scope.
- **The hidden mix.** If the 116 "failed" notes were all recovery faults, recovery would be 184 of 2,123, 8.67%
  (7.50% to 9.95%); if they split as the moded faults do (21% motor), 48 motor events, 2.26% (1.67% to 2.99%). These
  bracket the hidden part; they are not measurements. The club hosts university and NASA Student Launch teams
  (Stated), so K to O includes student vehicles.

## 3. The two clubs side by side

| H to O (Inference) | Champlain, per motor | SRA, per flight | Fisher exact p |
|---|---|---|---|
| Motor event | 32 / 1,184, 2.70% | 24 / 2,123, 1.13% | 0.001 |
| Motor event, H to J | 27 / 1,005, 2.69% | 20 / 1,852, 1.08% | 0.002 |
| Motor event, H to J, SRA's non-cert flights only | 27 / 1,005, 2.69% | 16 / 878, 1.82% | 0.22 |
| Recovery | 98 / 1,184, 8.28% | 68 / 2,123, 3.20% | below 10⁻⁹ |
| Any noted fault | 138 / 1,184, 11.66% | 229 / 2,123, 10.79% | not tested |

- **Motor events differ between clubs.** MASA's H to J (6 of 375), Champlain's and SRA's H to O rates fail a test of
  one common rate (χ² = 11.4 on 2 degrees of freedom, p = 0.003, Inference); MASA differs from neither club alone (p =
  0.34 and 0.44). The pooled 62 of 3,682, 1.68% (1.29% to 2.15%), treats every flight as one draw from one rate, so
  its interval is too narrow; the spread between clubs, about 1.1% to 2.7%, is the honest range for an unknown club
  (Inference). Champlain counts motors and SRA flights; motors run 6% above flights at Champlain (1,520 on 1,433 where
  both are stated), which widens the gap slightly.
- **Causes that fit, none measured.** The certification share (47% of SRA's H to O flights are L1 or L2 attempts,
  where SRA's motor rate is lowest): without them SRA is not shown to differ from Champlain (p = 0.22), but Champlain's
  own rate includes its cert flights (5 of its 32 events are on cert attempts, the cert denominator unknown), so the
  test is not like for like. Runs on one type (§2) and how fully each recorder writes may also contribute. The
  recovery gap may be mostly wording: SRA's "failed" with no mode is 5.46%, and the any-fault totals come within 0.9
  points (11.66% and 10.79%); that holds only if most "failed" notes are recovery faults.
- **Against MASA's high power.** MASA's H to J any-mode rate was 6.86% (4.53% to 9.89%, 26 of 379) on the same rules
  ([masa-recovery-modes.md](masa-recovery-modes.md)), below Champlain's 11.54%. SRA can't be set beside it on modes,
  since half its faults carry none.
- **Hybrids.** 1 of Champlain's 32 motor events is a hybrid; the pages give no hybrid denominator, so 31 of at most
  1,184, 2.62%, is the COTS bound (Inference).

## 4. Sources that give no rate, and dead ends

- **Denominators without outcomes:** Rocketry Organization of California's launch reports, 2014 to 2022 [S2], give
  flights by class (LDRS 40 in 2022: 652 flights, 381 H to M; NSL 2019: 737 flights, 335 H to M; Inference sums of
  Stated classes) and mention a CATO in 3 of 207 posts only as anecdotes. Tripoli Pittsburgh's 29 report pages,
  2020 to 2026 [S3] (2 read), and the Wright Stuff Rocketeers' "By the Numbers" [S5] give classes, no outcomes.
- **Too small or too coarse:** one Tripoli Houston report, 53 flights with "6 motors malfunctioning on 5 flights"
  (Stated), the two cases described low power [S4]; UKRA and K-Lob flight data, 2001 to 2003, about 450
  motor rows with almost no outcomes [S6]; C'Space student rockets, 5 of 20 ballistic in 2025 (Stated), an
  all-cause measure [S7].
- **Large launches:** no LDRS, BALLS, Airfest, Red Glare, NXRS or Thunda tally with CATO or recovery counts was found
  on the open web or in a signed-in Rocketry Forum search. Electronic flight cards (the open FlightCard app; NSW
  Rocketry's card has outcome boxes) publish no club tallies [S8].
- **Not reached:** magazine archives (Sport Rocketry, Extreme Rocketry, High Power Rocketry), insurance or safety
  committee summaries, and a statistics appendix that a search result places in a print high-power book.

## What this suggests

1. **M7.4's high-power motor prior as a range across clubs**, not one club's rate: about 1.1% to 2.7% per H to O
   flight (SRA 1.13%, Champlain 2.70%, MASA's H to J 1.6%), with 1.6% as a central value, stated as floors.
2. **Split by flight type where the evidence knows it:** a certification flight about 0.4% (SRA, 0.11% to 1.05%),
   other H to J flights 1.8% to 2.7%. A flight a user marks as a cert attempt could get the lower prior.
3. **High-power recovery prior:** a recovery failure on 3% to 9% of H to O flights; no chute or main is the largest
   mode where modes are written (5.2% at Champlain). Structural failures 0.5% to 0.8%.
4. **Allow for failures clustered by motor type and lot** in any odds M7.4 or the dispersion runs report: the
   binomial intervals here assume independent flights, and SRA's run on one J type suggests they are not.

## Open questions

- Which modes sit behind SRA's 116 "failed" notes; whether SRA posted logs before 2020.
- Whether the gap between clubs is certification share, motor mix, recording, or a real difference; a third
  multi-year high-power club with per-flight outcomes would test it. None was found.
- Per-type denominators for the clustered J failures, to tell a lot defect from a type rate.
- Large-launch outcome tallies (LDRS, BALLS, Airfest) and insurance incident counts; none is published openly.

## Sources

All read 2026-11-04. None grants a license; the SRA, Tripoli Pittsburgh and Wright Stuff pages carry copyright
notices (SRA's history page "All Rights Reserved"). All are cached privately and cited, and only aggregates appear
here, since every report names its flyers.
- [C1] Champlain Region Model Rocket Club, launch reports, one page a year, motor tallies by class
  per launch: <https://crmrc.org/2008_launch_reports.htm> to <https://crmrc.org/2026_launch_reports.htm>; the 2024
  page is [L3] in [fault-base-rates.md](fault-base-rates.md).
- [S1] Spaceport Rocketry Association newsletters, 2020-02 to 2026-09, one PDF per launch with a flight table:
  <http://www.spaceportrocketry.org/newsletters/>; history: <http://www.spaceportrocketry.org/History.html>.
- [S2] Rocketry Organization of California: <https://rocstock.org/2022/06/13/ldrs-40-launch-report/>,
  <https://rocstock.org/2019/06/12/nsl-launch-report/>, and the posts listed in <https://rocstock.org/sitemap.xml>.
- [S3] Tripoli Pittsburgh: <https://tripolipgh.org/launchReport051621.html>.
- [S4] Rocketry Forum, "Tripoli Houston 5/10 Flight Report", 2025-05-23:
  <https://www.rocketryforum.com/threads/tripoli-houston-5-10-flight-report.192523/>.
- [S5] Wright Stuff Rocketeers (NAR section 703) reports: <https://sections.nar.org/703/reports/>.
- [S6] UKRA 2003 flight data, Internet Archive:
  <http://web.archive.org/web/20030821175730/http://www.ukra.org.uk:80/news/2003/ukra2003flightdata.txt>.
- [S7] Planète Sciences, C'Space 2025: <https://www.planete-sciences.org/national/2025/07/18/c-space-2025/>.
- [S8] NSW Rocketry flight card:
  <https://nswrocketry.org.au/wp-content/uploads/2024/09/Flight_cards_double_sided.pdf>;
  FlightCard: <https://github.com/broofa/flightcard>.
