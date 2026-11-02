# Published recovery ground tests and whether the bay separated

Which published hobby and student ground tests report the bay, the charge, the shear pins and whether the
sections separated? Do those outcomes agree with the charge [M6.7](../ROADMAP.md) will suggest for the same bay,
and does that charge err toward a clean separation? It serves M6.7's check on which way its size errs and M6.8's
ground-test log. It builds on [suite field tools](suite-field-tools.md#ejection-testing) (Fetter's 2025 tests)
and [shear pin strength](shear-pin-strength.md) (the pin forces used here).
Read 2026-11-02: 13 student-competition reports from 11 teams, Fetter's paper, one builder's page, a vendor
newsletter and 10 Rocketry Forum threads (each post one person's word; no video watched). Outcomes are as
reported; **Inference** marks this note's own reading. M6.7 isn't built: "the rule" is its done-when, by hand.

**In short.**
- **The done-when's own form errs toward separation on every failure with a known charge.** Gas-law sizing at
  the pins' load, times Fetter's packed factor (3.33 to 3.47) and the 1.4 margin, is 4.67 to 4.86 times the
  pins' load. It clears Fetter's packed failures and partial by 1.87 or more. A 6 in student bay's failure sits
  at 3.54 if one charge fired (the narrative's reading): cleared by 1.32 to 1.37, missed without the margin.
  Its log says main and backup fired; then it sits above the rule and can't test the sign.
- **The 15 psi calculator is not enough in every packed bay.** One team's 15 psi main failed packed and 20 psi
  worked; another's 15 psi (on net volume) worked as for launch. Gas-law pressure overlaps across outcomes:
  clean shots 8.1 to 24.5 psi, failures 7.7 to 14.8 psi.
- **A charge per cubic inch from Fetter's fixtures doesn't carry to other bays.** His two fixtures need 0.0306
  and 0.0151 g/in³, 2.0 times apart. Scaled from the small one with the margin, a 6 in student bay gets 3.4 to
  7.0 times the charge that separated it cleanly, about 2,400 lbf on its bulkhead. From the larger one, his small
  fixture gets 0.69 g, and it failed at 0.7 g.
- **Clean charges spread 5 to 9 times, whatever they are divided by.** Over 30 clean or chosen charges in
  2.2 to 6.1 in bays, the max-to-min ratio is 8.8 per in³, 5.0 per in² of bulkhead and 8.6 per inch of length.
- **Public logs seldom bracket the least charge.** Of 38 tests outside Fetter's paper with a bay size, 2 give
  both a failed charge and the one that then worked. Column logs: Fetter's, Rice's and McGill's tables.

## The rule, worked out from M6.7's done-when

The usual calculator is the ideal-gas law: m = 0.000516 × P × V, in grams, with P the gauge pressure (psi) and V
the gross bay volume (in³). That comes from R = 22.16 ft·lbf/(lbm·°R), T = 3,307 °R and 454 g/lb. The pins'
balance pressure is P_pin = n × F / (π/4 × D²), for n pins of force F on a bulkhead of inner diameter D. The
pin forces are 31.0 / 49.8 / 75.0 lbf for 2-56 / 4-40 / 6-32 nylon (Fetter's Table 6-1), with a measured high
end of about 35 / 56 / 116 lbf ([shear pin strength](shear-pin-strength.md)).

M6.7's done-when sizes a charge "for a bay's gross volume and a target gauge pressure, or the shear pins' load
at the high end of their strength", "with an efficiency factor from published ground tests". With the pins'
pressure as the target, the factor is Fetter's first clean packed charge over the gas-law charge at P_pin:

- **TR-1** (2.15 × 9 in, 32.67 in³, two 2-56 pins): P_pin = 2 × 31.0 / 3.63 = 17.1 psi. The gas law gives
  0.288 g; 1.0 g separated cleanly, a factor of **3.47**. His failures there (0.3 to 0.7 g) sat at 1.04 to 2.43.
- **VTS-1** (3 × 15 in, 106.03 in³, two 2-56 pins, two parachutes): P_pin = 8.77 psi, 0.480 g by the gas law;
  1.6 g put both parachutes out, a factor of **3.33**. 1.2 g put out one, at 2.50.
- **With the 1.4 margin**, the rule's size is 4.67 to 4.86 times the gas-law charge at P_pin. That is the "force
  basis" below. On high-end pins the factor falls, the grams stay for 2-56 and 4-40, and 6-32 grows 37%
  (high end 1.55 times nominal against 1.13).
  **Inference:** one reading of the done-when, the one using its pins; a fixed pressure by volume is the "per
  cubic inch" basis below.

## Student-competition tests with full data

21 tests give inner diameter, length or volume, charge, pins and outcome. They come from 5 teams: Rice, Buffalo
and BYU at the 2018 Spaceport America Cup [1-3]; NC State's 2020, 2025 and 2026 Student Launch FRRs [4-6]; and
Toledo's 2025 FRR [7]. 18 separated cleanly, 2 failed on the ground and 1 failed in flight. "ld/pin" is the
gas-law load on the bulkhead over the pins' nominal strength; the rule sits at 4.67 to 4.86. Buffalo and NC
State (2020, 2025) say the bays were packed as for flight. Rice's 4.0 in is the outside diameter; its sizing
names 2-56 pins (p. 25), its checklist 4-40, which would cut its ld/pin by 38%.

| team [ref] | ID × L, in (V in³) | pins | charge g: outcome | gas-law psi | ld/pin |
|---|---|---|---|---|---|
| Rice [1, pp. 34-36] | 4.0 × 8 (100.5) | 2 × 2-56 | 0.6 pins held; 0.8 clean | 11.6; 15.4 | 2.34; 3.13 |
| Rice [1] | 4.0 × 19 (238.8) | 2 × 2-56 | 1.0 clean | 8.1 | 1.65 |
| Buffalo [2, p. 13] | 6.148 × 14.69 (436.1) | 4 × 2-56 | 3.33 main stayed in; 4.33 clean | 14.8; 19.2 | 3.54; 4.61 |
| Buffalo [2] | 6.148 × 17.9 (531.4) | 4 × 2-56 | 4.06 clean (drogue) | 14.8 | 3.54 |
| BYU [3, p. 24] | 6.0 × 8.3, 15.0 | 4 × 20 lbf | 1.5, 2.7 slow; 1.7, 3.0 fast | 12.3 to 14.0 | 4.36 to 4.94 |
| NC State 2020 [4] | 6.0 × 7, 21 | 4 × 4-40 | 2.5, 6.1 "excessive"; 2.3, 5.5 clean | 18.0 to 24.5 | 2.55 to 3.47 |
| NC State 2025-26 [5, 6] | 6.0 (351 to 393) | 4 × 4-40 | 2.8, 3.1, 3.5, 3.8 clean | 15.3 to 19.3 | 2.17 to 2.75 |
| Toledo [7, p. 116] | 6.00 (376 tested; 1,131) | 4 × M3 | 3.0, 7.0 clean, pins broke | 15.5; 12.0 | 1.64; 1.27 |
| Toledo, flight [7, p. 116] | 6.00 (755 flown) | 4 × M3 | 3.0 no separation, nor the 3.6 backup | 7.7 | 0.82 |

- BYU gives no bay lengths; they are back-solved from its own formula, so its rows aren't independent [3].
  Its pins are its own unsourced 20 lbf.
- NC State 2025's volume is net of the parachute [5, p. 30], below gross. The FRR rates a
  4-40 pin "for 2.5 psi", a force written as a pressure; their own pull tests read 37 to 45 lbf [5, p. 156].
- Toledo's volume table equals π × D² × L, four times the volume [7, p. 15]; 1,131 in³ is its main bay
  divided by 4 (**Inference**). Its 3.0 g ground test had two harness holes taped that were open in flight,
  half the flown volume [7, p. 116]; the 7.0 g test is on pp. 193-194. M3 nylon is 66.6 lbf by Fetter's rule
  (**Inference**; not in the pin table).
- Buffalo's narrative tells of a 15 psi charge that failed and a 20 psi one that worked [2, p. 8]; its log says
  the first test fired "main and backup charges" [2, p. 13]. With the 5.33 g backup added, the failure sits at
  9.21 times the pins' load, still under 1.4 times either Fetter density.

Six more teams (Iowa State, Oregon State, RIT, McGill, UIC, Stanford; 2018 reports [8-13]) give charges and
outcomes without a bay length. Among them: McGill's 2.0 g failed in a full chamber and separated in an
empty one [11]; UIC's 3.25 g separated "most of the time" with some partial separations, 3.5 g every time [12];
Stanford's 3 to 4 g never broke its #6 pins in a 4 in rocket [13]. Oregon State climbed from 8 g in 2 g steps to
14 g and 12 g [9].

## Builders and forums

Fetter's paper [14] holds 28 shots in three fixtures, more than M6.7 cites. The same VTS-1 tube and pins
separated at 0.15 g empty and needed 1.6 g with two parachutes, more than ten times [14, Table 9-6, p. 47]. A
bare phenolic tube split at the charge canister at 4.0 g with its pins whole [14, pp. 66-67]. A log that records
only "separated or not" would file that as a failure to separate.

17 rows outside student reports give diameter, length and charge (one length back-solved; a "4 in" tube taken
as 3.9 in inside, **Inference**): Knowles's page [15], 5 bays charged until "reasonably forceful", and 12 forum
rows from 9 posters [16-24, 26]. Only 9 also state pins, and 3 of those bays
are friction fit. They hold one failure (a 3.9 × 14 in bay, three 4-40 pins, 1.2 g, gas out the vent; ld/pin
1.11 [16, #2]) and one partial (pins sheared, parachute stuck, 1.5 g [21, #100]). The rest are working charges
chosen above the least that works, so they bound nothing below them.

## How the outcomes sit against each basis

| basis | clean (min / median / max) | failed | rule's size |
|---|---|---|---|
| gas-law ld/pin, student (18 clean, 2 failed) | 1.27 / 2.98 / 4.94 | 2.34, 3.54 | 4.67 to 4.86 |
| gas-law ld/pin, Fetter packed (6 clean, 4 failed) | 3.33 / 4.24 / 6.25 | 1.04 to 2.43 | 4.67 to 4.86 |
| gas-law ld/pin, builders with pins (5 clean, 1 failed) | 1.49 / 1.95 / 7.50 | 1.11 | 4.67 to 4.86 |
| charge per in³ over TR-1's, student | 0.14 / 0.26 / 0.41 | 0.20, 0.25 | 1.4 |
| charge per in³ over VTS-1's, student | 0.28 / 0.53 / 0.84 | 0.40, 0.51 | 1.4 |

- **Force basis, failure side:** every failure with a known charge sits below the rule; Buffalo's, as one
  charge, is closest at 1.32 to 1.37 times under. Toledo's flight failure (0.82) points to a leak, not the pins.
- **Force basis, clean side:** the rule gives 0.95 to 3.8 times the student teams' clean charges and 1.9 to 3.3
  times the builders' where pins are known. Two clean shots sit above it: BYU's 4.84 and 4.94, on BYU's own pin
  ratings. One builder's single-pin 54 mm bay flew 1.2 g; the rule gives 0.78 g [17, #30]. That is a working
  charge, not a seen failure.
- **Per cubic inch:** no failure outside Fetter's reached 1.0 times either density. 1.4 times TR-1's is 1.4 to
  12.3 times every working charge outside Fetter. 1.4 times VTS-1's falls below two small bays' working
  charges (0.69 and 0.83 times them) and on TR-1's 0.7 g failure.
- **Pinless bays:** the force basis gives no charge for a friction fit. Three pinless working bays used 1.0 to
  2.25 g in 3.9 in tubes (one pinless by **Inference**), 6.8 to 14.0 psi by the gas law [18, #3; 19, #2].
- **Overpressure:** NC State called 2.5 g in 198 in³ and 6.1 g in 594 in³ "excessive" [4]. Both sat below the
  rule's size. On TR-1 the rule gives 1.4 g, above the 1.3 g Fetter called very energetic. No source bounds
  when a bulkhead or tube breaks except his phenolic tube at 4.0 g.

**What scales (Inference).** Over the 30 clean or chosen charges with dimensions (13 student, BYU and Toledo's
inferred volume left out; 17 others), the max-to-min ratio and coefficient of variation are 8.8 and 0.61 per
in³, 5.0 and 0.45 per in² of bulkhead, 8.6 and 0.52 per inch of length. Area tracks best across diameters;
within the 11 6 in student bays volume does (1.65 and 0.17, against 2.65 and 0.32 by area). Fetter's two fixtures
need almost the same charge per inch, 0.111 and 0.107 g/in.

## Outcome words and the columns logs keep

- **Fetter** [14]: no, yes, partial, clean, energetic, very energetic, "1 of 2 chutes", "chutes ejected,
  piston remains", airframe split. "Energetic" means the nose reaches the cord's end fast and snaps back
  (p. 39). Columns: charge, parachutes, tube fill (empty, front, back or back quarter, full), piston, outcome.
- **Student teams:** clean, slow, fast, "excessive", pins held, all pins broke, flew 30 ft, main stayed in.
- **Forums:** barely separated, sheared but chute not out, separated cleanly but weak, full extension,
  thrown 40 ft, loud but good. Damage: coupler rivets driven 2 in up [24, #1]; a nose cone torn with its pins
  intact [22, #3].
- **Advice splits:** Peak of Flight 587 urges the least charge that separates [25, p. 5]; Fetter adds 40%.
- **What made rows unusable here:** a missing length, net or as-tested volume without saying which, pins
  without size, an unstated packing, a hole sealed only for the test, and failed shots left out (Oregon
  State gives its start, step and finals, not each shot).

## What this suggests

- **M6.7's sign test:** take the 21 student rows and Fetter's packed shots as a table. The rule must give more
  than every failed charge, with the 1.4 margin intact. The closest is Buffalo's, 1.32 times as one charge. A
  mutation dropping the margin should fail on that row. Expected effect: it pins the sign on bays up to 5.6
  times VTS-1's volume, not only on Fetter's two fixtures.
- **M6.7, two gaps the done-when leaves open:** how Fetter's factor carries to another bay (by pins, volume or
  area) and what a pinless bay gets. On these data, by pins errs toward separation and per in³ from VTS-1 does
  not. A pinless floor near the 14 psi the builders' working bays reached is an option, not a finding.
- **M6.8's log:** inner diameter and pressurized length, how volume was measured, pins (count, size, rating
  source), fill and protector, every sealed hole, vent, powder grade, holder, and every shot's charge. Use
  Fetter's outcomes plus "pins sheared, nothing out" and "airframe damage", which the forums report and a
  separated-or-not field loses. Log the failed shot below the clean one: it brackets the least charge.

## Open questions

- **Thin evidence:** partial separations with dimensions are Fetter's VTS-1 shot and one forum bay; two bays
  outside Fetter bracket the least charge (Rice 4.0 in, Buffalo 6.1 in), none larger; all are at sea level.
- **Not reached or searched:** University of Hawaii's current pages (timed out), University of Florida (404),
  Sport Rocketry, rec.models.rockets, and Spaceport America Cup reports outside ESRA's 2018 page.

## Sources

All read 2026-11-02. ESRA reports cite PDF pages; NASA Student Launch reports printed pages. None states a
license for the whole report: cite only. Copies are cached in the gitignored research cache.
ESRA's 2018 reports sit at `https://www.soundingrocket.org/uploads/9/0/6/4/9064598/<n>_project_report.pdf`
(team 111's file is `111_projectreport.pdf`); NC State's FRRs at `https://ncsurocketry.org/wp-content/uploads/`
(`2020/03/north-carolina-state-university-2020-frr-document.pdf`, `2025/03/...-2025-frr-report.pdf`,
`2026/03/...-2026-frr-report.pdf`); Rocketry Forum threads at `https://www.rocketryforum.com/threads/<id>/`,
post numbers after #.

1. Rice Eclipse, team 92, 2018 SA Cup, pp. 4, 25, 34-36. 2. University at Buffalo SEDS, team 83, 2018, pp. 8, 13, 19.
3. BYU Rocketry, team 84, 2018, pp. 9-10, 24.
4. NC State, 2020 FRR, §3.5.7 p. 36, §7.1.9 pp. 135-136, pins p. 181.
5. NC State, 2025 FRR, Table 3.4 pp. 30-31, pin pulls p. 156. 6. NC State, 2026 FRR, pp. 7, 9, 30.
7. University of Toledo Rocketry Club, NASA SL FRR, 17 Mar 2025, pp. 14-16, 116, 193-194; linked from
   https://www.utrocketry.club/projects/nasa-usli-2025
8. Iowa State, team 20, 2018, pp. 8, 25-26. 9. Oregon State, team 21, 2018, pp. 23-24.
10. RIT Launch Initiative, team 78, 2018, p. 34 (http://launch.rit.edu/uploads/1/1/4/2/114234541/78_project_report.pdf).
11. McGill, team 93, 2018, Table 2 p. 13. 12. UIC, team 65, 2018, p. 13. 13. Stanford SSI, team 111, pp. 48-50.
14. R. Fetter, "Using Black Powder for Parachute Deployment", vNARCON 2025, Rev 1.2; Tables 6-1, 9-1 to 9-6,
    §15. http://www.speedmotionrockets.com/Using%20Black%20Powder%20for%20Parachute%20Deployment%20-%20Rev1_2.pdf
15. V. Knowles, "Ejection Charge Sizing", table "Actual Charge Sizes". https://vernk.com/EjectionChargeSizing.htm
16. "ground test fail, need help" (2013), id 51470. 17. "Dual Deploy / Shear pin questions" (2022), id 175468.
18. "How much BP / how many shear pins" (2015), id 121885. 19. A first dual-deploy build thread (2014), id 67192.
20. "Shear pin and BP sizing" (2015), id 124658. 21. A build thread (2013), id 51497, page 4.
22. "updated nose cones" (2023), id 182326. 23. A 4 in fiberglass Level 2 build thread (2016), id 133697, page 2.
24. "Dual Deploy Ground Test Issue" (2022), id 172686.
25. T. Van Milligan, "How to Ground Test a Dual Deployment Altimeter", Peak of Flight 587 (2022), p. 5.
    https://www.apogeerockets.com/education/downloads/Newsletter587.pdf
26. "shear pins" (2006), id 93890.
