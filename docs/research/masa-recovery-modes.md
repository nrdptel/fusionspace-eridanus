# Recovery and structural failure modes by installed-impulse class in MASA's logs

How often does each failure mode (separation, lawn dart, no chute, unstable boost, core sample, shred) happen per
flight, by motor class? Fault diagnosis (M7.4 in the [roadmap](../ROADMAP.md)) ranks these modes and needs their priors;
[how often each fault happens](fault-base-rates.md) gave the 2005 NAR study's shares. This note codes MASA's logs
(13,670 flights, 1999 to 2013, and the study's two 1998 logs) with the study's rules, per flight in three bands of
installed impulse, checks them against the study's per-class tables, and tests the mix after 2005. Read 2026-11-04.
**Stated** marks a number a source prints, **Inference** this note's count or reading. Intervals are Clopper–Pearson
95%. Every rate is a floor: it counts only what a launch officer wrote.

**In short.**
- **About 1 flight in 13 fails in one of the study's modes in MM to D, 1 in 10 in E to G, 1 in 15 in H to J:** 7.45%
  (6.96% to 7.97%), 10.01% (8.86% to 11.25%) and 6.86% (4.53% to 9.89%, 26 of 379). Separation and lawn dart lead in
  each band. Staged and clustered flights fail 2.1 times as often as simple ones in MM to D (14.4%) and 1.6 times in
  E to G (14.0%).
- **The coder reproduces the study on its own span** (October 1998 to June 2005): 8.11% against 8.48%, lawn darts 1.98%
  in both, and simple-rocket rates within 0.5 points of the study's Table 3 in 62 of 70 class-and-mode cells. Complex
  lawn darts match only with a rule the study implies, which counts a failed staging or partial ignition as a lawn dart;
  none of its 34 flights, read by hand, states the outcome (§3).
- **After 2005 the mix shifts a little; the total doesn't.** Mode shares differ between 1999 to 2005 and 2006 to 2013
  (p = 0.0035) with a small effect (Cramér's V 0.13): unstable boosts fall from 1.45% to 0.96% per flight, motor
  failures rise from 0.45% to 0.79%; any failure, 8.17% to 7.67% (p = 0.28). The band mix explains 10% to 17% of the
  unstable and motor changes (§7).
- **The coder is right on 78 of 80 sampled coded flights** (91.3% to 99.7%) and misses 2 to 6 of 250 sampled uncoded
  comments: uncoded failures could add 6% to 18% to the coded count, up to 39% at the interval's top (§4).

## 1. The study's rules, applied by keyword

**The modes** [L1, p. C-5]. *Unstable*: part of the boost flown nose-down, not mild wobble, horizontal flight or coning
unless they crash. *Lawn dart*: ballistic, nose cone on, no-ejection flights and power prangs included. *Separation*:
parts fall apart, one not slowed, stripped chutes included. *Core sample*: ballistic, nose cone off. *No chute*: no
recovery device out but not ballistic, tangled chutes excluded. *Shred*: comes apart in ascent. *Motor CATO*; *motor
unrestrained*: the motor leaves at ignition or in boost. **The rules.** A flight with several faults counts once, as its
most severe [L1, p. C-5]. Drag separations and hard landings are not unsafe [L1, p. C-4]. Events of unclear kind (a bare
"R.I.P.") were not included; delay complaints and motor-performance notes count only if one of the eight followed
[L1, p. C-5]. Rockets built to lawn dart (stomp rockets, UFOs, birdies) or to break apart are excluded [L1, p. C-5,
notes 11 and 12]. For severity this note takes Table 2's relative-risk order [L1, p. C-20]: lawn dart, core sample,
separation, no chute, shred, unstable, CATO, with motor unrestrained after separation (Inference).

**Motors that leave the rocket.** MASA, unlike a second club, doesn't count spit motors [L1, p. C-12], and its launch
officers almost never recorded motor ejections, though the definition would file them as separations [L1, p. C-5]. So a
bare spit or kicked motor is not coded; a spat nozzle or closure, blow-by or a destroyed casing is a CATO. Of 13,670
flights, 16 comments name a motor leaving, 8 with no other mode (0.06 points as separations), and none at ignition or in
boost, so *unrestrained* is 0, against 1% of the study's failures [L1, p. 11] (Inference).

## 2. The rows

**The parse.** The logs of [the earlier note](fault-base-rates.md) §2, parsed again with three changes (Inference). A
contest block that one 2004 log prints as per-class motor counts (4 rows, 56 flights) is dropped. The two 1998 logs, in
an older stage-by-stage layout, are added: 190 and 72 flights, the study's 262 [L1, Table 1, p. C-3]. Each log's row
count was checked against the total it prints, and 147 of 149 match: one 2003 log leaves out a pad CATO with no motor
burned, and the 2004 log's total counts its 56 contest flights. **The span.** Table 1 lists 2, 8, 7, 9, 11, 13, 9 and 4
launches for 1998 to 2005 [L1, p. C-3]. The cache holds the same number each year, taking 2005's first four (February to
June, 290 flights against 291): 6,251 flights against 6,169, each year within 7 of Table 1 except 2003 (963 against 866,
unexplained). The full years 1999 to 2005 hold 6,410: no 1998 flights, 421 from five later 2005 launches.

**Class by installed impulse.** The study classes each flight by the impulse installed at ignition, from the motor list
[L1, p. C-4]; the logs give what burned. For the 102 rows that list more motors than they burned, installed impulse is
the sum over listed motors of each type's median logged impulse on single-motor rows (a D12 as 17 N·s). Every type was
known. The rule raises 87 rows' class and 28 rows' band (27 from MM to D into E to G, 1 into H to J), and classes 3 pad
CATOs that burned nothing. MicroMaxx (0.3 N·s or less) is its own class, MM, as in the study's tables. No row is K or
larger; 9 have no class. A flight is *complex* when more than one motor burned or is listed: 534 of 6,251 on the span,
against 518 of 6,169 [L1, p. C-7].

## 3. The coder, and the one rule it adds

**The phrases.** Each mode is a set of phrases with negations: separation but not drag separation or a glider pod's
planned release; no chute, no streamer, a chute that didn't open; lawn dart, no ejection, prang, ballistic, ejection on
impact, a rocket left standing nose-down in the ground; unstable but not mildly unstable or queried; core sample; shred,
came apart, a shed fin; CATO, a spat nozzle or closure, blow-by. Rocket names mark the excluded designs. **The staging
rule.** The *study-like* coder counts a complex flight whose only comment is a failed staging or an incomplete cluster
ignition as a lawn dart, unless the comment calls the flight safe or good. The study says failed staging explains most
of the *difference* between complex and simple lawn-dart rates and incomplete cluster ignitions most of the rest
[L1, p. 12; p. C-14], and that such ignitions often end in lawn darts because the rocket never gets high enough for its
delay [L1, p. C-14, note 21]. Rates below use this rule unless marked strict.

**Its flights, read by hand.** The rule codes 34 flights, all after 1998: 13 name a failed staging and 21 say only that
some motors lit. None states one of the eight outcomes; 3 give another (a low flight, a water landing, a horizontal
flight) and 4 carry a question mark. The 7 flights it skips all call the flight safe, good or nice. So the rule rests on
the study's reading, not on these comments (Inference). Table 4 backs it: on the span, complex C, D and E lawn darts are
7.1%, 3.6% and 5.8% strict, 12.5%, 7.9% and 7.9% with the rule, and 14.0%, 8.7% and 9.3% in the study [L1, p. C-21].

## 4. How well the coder reads the comments

**Precision.** A simple random sample of 80 of the 1,081 coded flights (fixed seed; 13 also in earlier samples) was read
against §1's definitions. The comment states the coded mode on 78, 97.5% (91.3% to 99.7%); the other 2 are staging-rule
flights naming only the ignition fault. None was another mode or no failure (Inference). **What it misses.** Of 13,670
flights, 4,388 (32.1%) have no comment and 8,201 have one the coder leaves uncoded. A simple random sample of 250 of the
8,201 (22 read before) held 2 clear misses, both a two-chute rocket with one chute out, and 4 possible: a low flight
that crashed, a breakup, a flight noted as never working, and a partial ignition worded past the staging rule. That is
0.80% (0.10% to 2.86%), about 66 flights (8 to 235) or 6.1% of the coded count (up to 21.7%); with the possible ones
2.40% (0.89% to 5.15%), about 197 (73 to 422) or 18% (up to 39%) (Inference). Rows with no comment count as fine; the
study read the same logs [L1, p. C-4] and shares that blind spot (Inference).

## 5. Rates by class band, 1999 to 2013

| Mode, per flight | MM to D (10,774) | E to G (2,508) | H to J (379) |
|---|---:|---:|---:|
| Separation | 2.41% (2.13% to 2.72%) | 2.31% (1.76% to 2.98%) | 2.64% (1.27% to 4.80%) |
| Lawn dart | 1.83% (1.58% to 2.10%) | 3.35% (2.68% to 4.13%) | 1.58% (0.58% to 3.41%) |
| Unstable | 1.26% (1.06% to 1.49%) | 1.08% (0.71% to 1.56%) | 0 (0 to 0.97%) |
| No chute | 1.04% (0.86% to 1.25%) | 0.96% (0.61% to 1.42%) | 0.53% (0.06% to 1.89%) |
| Motor CATO | 0.41% (0.30% to 0.55%) | 1.48% (1.04% to 2.03%) | 1.06% (0.29% to 2.68%) |
| Core sample | 0.39% (0.28% to 0.53%) | 0.64% (0.37% to 1.03%) | 1.06% (0.29% to 2.68%) |
| Shred | 0.11% (0.06% to 0.19%) | 0.20% (0.06% to 0.46%) | 0 (0 to 0.97%) |
| Any of the eight | 7.45% (6.96% to 7.97%) | 10.01% (8.86% to 11.25%) | 6.86% (4.53% to 9.89%) |
| Strict coder, any | 7.25% | 9.53% | 6.86% |

All Inference: 803, 251 and 26 failures; motor unrestrained 0 in each (up to 0.03%, 0.15%, 0.97%). A CATO here is the
most severe fault only, so these run below the earlier note's motor-event rates. Banded by logged impulse, any failure
is 7.65%, 9.07% and 6.88%; by impulse per motor, 7.72%, 9.00% and 6.93%. H to J has the fewest empty comments (13.5%
against 33.3% in MM to D); its separation share, 38%, runs from 20% to 59%.

| Band | Simple, any | Complex, any | Complex, lawn dart | Complex flights |
|---|---:|---:|---:|---:|
| MM to D | 7.02% (6.53% to 7.54%) | 14.4% (11.8% to 17.5%) | 8.5% (6.4% to 11.0%) | 623 |
| E to G | 8.90% (7.68% to 10.24%) | 14.0% (11.2% to 17.3%) | 7.2% (5.2% to 9.7%) | 541 |

Complex flights fail 2.1 times as often as simple ones in MM to D (Fisher p = 7.5 × 10⁻¹⁰) and 1.6 times in E to G
(p = 0.00065); their lawn darts run 6.0 and 3.2 times the simple rates (1.42%, 2.29%). Strict, complex MM to D fails
10.9% with 5.0% lawn darts, E to G 11.8% and 5.0%. H to J: simple 6.95%, complex 0 of 5 (Inference).

## 6. Against the study's Tables 3 and 4

The study prints each mode's rate per class for simple (Table 3) and complex rockets (Table 4), to 0.1 point and with no
flight counts [L1, pp. C-20, C-21]; Table 4's complex 1/4A column is blank, and Table 6's products place the rest. On
the span, in classes with 50 or more flights here, the any-failure rate lies within 1 point of the study's column sum
(good to 0.35 points) except simple 1/2A (8.2% against 6.9%) and complex C (17.9%, 20.5%), E (13.1%, 14.6%) and F
(30.8%, 25.1%) (Inference). Of the 70 simple cells (10 classes, 7 modes), 52 lie within 0.3 points of the study and 62
within 0.5; the largest gap is 1.7 points (no chute, MM, 65 flights). Of 28 complex cells (C to F), 16 lie within 0.3
and 23 within 1.0; the largest is 4.8 points (lawn darts, complex F, 52 flights). Both count mostly the same flights:
this checks the coder and the class rule, not MASA. **Totals on the span.** The study-like coder finds 8.11%
(7.45% to 8.82%), 507 of 6,251, against 8.48% (523 of 6,169) [L1, p. 11]: 0.37 points or 4.3% below; strict, 7.82%, 0.66
points or 7.7% below. Lawn darts are 1.98% in both (124 here; 70 + 52 there [L1, p. 12]); simple rockets fail 7.29%
against 7.7%, complex 16.9% against 17.4% [L1, p. 12]. The mode shares fit the study's (chi-square 5.7, p = 0.46).

## 7. Before and after 2005

| Mode | Study share | 1999–2005 share | Per flight | 2006–2013 share | Per flight | Change, points | p |
|---|---:|---:|---:|---:|---:|---:|---:|
| Separation | 28% | 32.4% | 2.65% | 28.4% | 2.18% | −0.48 (−1.00 to +0.04) | 0.073 |
| Lawn dart | 23% | 24.2% | 1.98% | 28.7% | 2.20% | +0.22 (−0.26 to +0.70) | 0.37 |
| Unstable | 19% | 17.7% | 1.45% | 12.6% | 0.96% | −0.49 (−0.87 to −0.12) | 0.0092 |
| No chute | 16% | 13.5% | 1.11% | 12.0% | 0.92% | −0.18 (−0.53 to +0.15) | 0.30 |
| CATO and unrestrained | 6% + 1% | 5.5% | 0.45% | 10.2% | 0.79% | +0.33 (+0.07 to +0.60) | 0.017 |
| Core sample | 5% | 4.6% | 0.37% | 6.8% | 0.52% | +0.15 (−0.08 to +0.38) | 0.20 |
| Shred | 2% | 1.9% | 0.16% | 1.3% | 0.10% | −0.06 (−0.20 to +0.07) | 0.34 |
| Any (flights) | 523 of 6,169 | 524 of 6,410 | 8.17% | 557 of 7,260 | 7.67% | −0.50 (−1.42 to +0.40) | 0.28 |

The study's shares are Stated, whole percents of its 523 failures [L1, p. 11]; the rest is Inference, with Fisher's
exact two-sided p and Newcombe's interval for 2006 to 2013 minus 1999 to 2005. **The test.** A chi-square test of
homogeneity on the 2 by 7 table of counts, checked by 20,000 permutations, says the 2006 to 2013 shares differ from
1999 to 2005 (chi-square 19.4, 6 degrees of freedom, p = 0.0035; permutation p = 0.0032) and from the study's
(chi-square 39.1, p = 6.7 × 10⁻⁷). The effect is small: V = 0.134, where chance alone at 1,081 failures gives about
0.075. No mode passes a correction for 7 tests (0.05/7 = 0.007); unstable is closest.

**Causes, with sign and size.** *Class mix:* E to J grew from 17.4% to 24.4% of flights, yet within MM to D the shift is
p = 0.077 at V = 0.119, and within simple MM to D p = 0.035 at V = 0.138, as large as overall. Holding the band mix at
either era's (direct standardization, classed flights), the mix accounts for 10% to 16% of the unstable share's
5.2-point fall, 16% to 17% of the motor share's 4.5-point rise, 0% to 6% of the lawn-dart rise and −6% to 33% of
separation's fall (Inference). Within E to G, motor failures rise from 0.76% to 2.00% per flight
(8 of 1,055 to 29 of 1,453, p = 0.011). *Reporting:* empty comments fell from 34.8% to 29.7%, yet the any-failure rate
fell slightly. *Fewer unstable boosts* (simple MM to D, 1.44% to 0.97%): the logs don't say why, so unbounded.

**Floors.** The study expects underreporting, as some launch officers write more than others [L1, p. C-4, note 9]. A
second club's rate runs about 6% higher overall because it also counts tangled chutes, spit motors and early ejections
[L1, p. C-12]: 15% against MASA's 8.5% [L1, p. 11, note 5]. How far the true rate exceeds the study's is unbounded.
**Excluded outcomes**, flights with no coded mode only: tangled, wadded or partly opened chutes 176 (1.29%), late
ejections 95 (0.69%), early ones 39 (0.29%), a motor leaving the rocket 8 (0.06%), a crash, damage or hard-landing word
44 (0.32%). Flights with one of these and a mode as well: 5, 35, 12, 8 and 18 (Inference, by phrase).

## What this suggests

1. **Per-flight priors by band for M7.4.** §5's table; for H to J, separation 2.6%, lawn dart 1.6%, CATO and core
   sample 1.1% each, no chute 0.5%, unstable and shred 0 of 379. Each is a floor; the miss audit (up to 39% more)
   outweighs the 4.5% gap to the study.
2. **P(mode | fault) for a flight already known to be odd.** For E to J, the 2006 to 2013 E to G mix: lawn dart 30%
   (23% to 38%), separation 24% (18% to 32%), CATO 19% (13% to 26%), no chute 9%, unstable 8%, core sample 7%, shred 2%
   (Inference, 152 failures in 1,453 flights). M7.4's 200-flight benchmark could use it beside a uniform mix.
3. **Complex flights need their own branch.** A staged or clustered MM to D flight lawn darts 8.5% of the time, 6.0
   times a simple one (3.5 times on stated outcomes): a partial ignition should raise that hypothesis. The coder, with
   this audit as its test set, could also read other clubs' public logs.

## Open questions

- Dual-deploy faults are too rare for a rate: 12 comments name dual deployment, and 8 describe an electronics deployment
  fault (a main that didn't come out, failed electronics, no ejection or chute under an altimeter).
- Unwritten failures (no second club or reader); why unstable boosts fell; why 2003 holds 97 more flights than Table 1.

## Sources

The MASA logs are marked "All Rights Reserved": cached privately, aggregates only, never names or rows. The study states
no license: cached privately and cited.
- [L1] NAR special committee, "Launching Safely in the 21st Century", 2005-10-29, pp. 11 to 12 and App. C, pp. C-3 to
  C-5, C-7, C-12, C-14, C-20, C-21; cached copy read 2026-11-04:
  <https://assets.zyrosite.com/Yg21M7RM7MSaQpNz/launchsafe-TQxWMfjtrNVQrGeU.pdf>.
- [L2] MASA launch logs, 1998 to 2013, 149 logs (PDFs and their text), cached 2026-10-17, read 2026-11-04:
  <https://masa-rocketry.org/launch-statistics>.
- [How often each fault happens](fault-base-rates.md), §1 and §2: the parse, the earlier counts and the interval.
