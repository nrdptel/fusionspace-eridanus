# What NAR's June 2026 retests of AMW motors measured

This note asks what NAR S&T's 2026-08-20 notice on six Animal Motor Works (AMW) reloads, K1000SK to N4000BB,
measured against their certification and against the ThrustCurve curves HPR Sim flies. It follows
[high-power lot impulse](hpr-lot-impulse.md) §3 and feeds [motor data](motor-data.md) §5 and
[apogee bias](apogee-bias.md) §1. Read 2026-10-15; **Inference** marks this note's arithmetic and readings.

**In short.** The topic's premise is wrong: the notice publishes **no measured number**, nor does any source found.
- **The notice is one page.** It names six motors, a test date (2026-06-24) and casing rules. It says only that new
  and original production are "within the allowable performance variances specified by NFPA 1125" [N1]. The forum
  thread's attachment is that same page; all 56 posts, read without a login, carry no figure (§1).
- **No listing moved.** NAR's list prints the same six rows in captures of 2025-02-11, 2026-06-08 and 2026-10-15,
  with the same 2002–2006 data sheets (§2). ThrustCurve edited the six records on 2026-08-21, but the one archived
  page (N4000BB, 2024) shows the same impulse; it went from out of production to "regular" [S3].
- **The curves HPR Sim would fetch match the 2002–2006 means** within −1.17% to +0.23% of impulse (§3). Two carry
  the mass after firing as propellant mass (N2700BB −7.1%, N4000BB −19.3%), which the exhaust-velocity guard accepts.
- **What "within the variances" bounds (Inference, §4).** If NAR applied its manual, each retest firing was within
  ±13.1% of the original mean, and each motor's mean stayed within 1% of its letter class: that caps the drop at
  **−5.2%** for L1300BB (waivable), −11.2% for M1350WW, −11.5% for N2700BB and −13.1% for the other three.
  Burn time follows only from those two rules, about −34% to +39%.
- **Bound unchanged (§5).** It does not narrow hpr-lot-impulse's 0 to −4% (−9% not excluded) for current H+ lots:
  one motor of one restarted maker excludes −9%, the rest don't. Per-firing retest impulses would.

## 1. What the notice and the threads say

- **The notice** [N1] is a one-page, unsigned PDF dated 2026-08-20; post 1 of [F1] posts it for NAR S&T. It lists
  K1000SK (Skidmark, 54-2550 hardware), L1300BB (Blue Baboon, 54-2550), M1350WW (White Wolf, 75-6000), M1900BB
  (Blue Baboon, 75-6000), N2700BB (98-11000) and N4000BB (98-17500). 54 and 75 mm reloads may use same-length Loki
  or Gorilla cases with AMW closures; the 98 mm motors need the full AMW set. No impulse, thrust, burn time,
  count of firings, lot, temperature or site appears.
- **TRF thread 198601** (2026-08-20 to 09-01; 56 posts on 2 pages, counted from the cached pages) [F1]. Post 1
  restates the notice; its attachment, 181.9 KB, is the cached PDF (186,256 bytes). The rest is hardware fit
  and history: post 19 says the maker brought new nozzles and closures "for the cert test firings", and post 43
  says the current owner began producing the classic propellants again in 2026. No post quotes a figure.
- **TRF thread 198634** (9 posts) [F2] announces the reloads on sale again; no numbers.
- **TRF thread 197044** (27 posts, one page) [F3]. Post 14 (2026-06-27) names the M2200 Skidmark, post 17 a photo of it;
  post 16 says six AMW M and N motors were fired for recertification; post 27 names M3000ST and Blue Baboon in
  75 and 98 mm, with "more details" after NAR's review. Post 5 of thread 198601 expects the M2200 in a later set.
- **Inference.** The notice is not the full list of what was fired: the M2200SK and M3000ST are named as tested
  but not listed. Why is not public. NAR's manual keeps other test data private but exempts the published summary
  of successful tests, and asks that tests leading to certification have their summary published within 30 days
  [M1, §7.2–7.3, p. 14]. So the manual does not explain the missing numbers; no source found does.
- hpr-lot-impulse §3 says "numbers need a forum login"; the cached pages show that is not so.

## 2. The certified record

NAR's data sheets for the six [D1–D6] are the original certifications; none is a retest sheet (NAR marks those
◊ or ∆ on its list, and none of the 53 AMW rows carries one) [L1]. Average thrust is impulse over burn time where
the sheet doesn't print it (**Inference**).

| Motor | Tested | n | Mean impulse, N·s (σ) | List, N·s | Burn, s | Avg thrust, N | Propellant, g |
|---|---|---:|---:|---:|---:|---:|---:|
| K1000SK | 2004-03-27 | 3 | 2,077.51 (41.64) | 2,120 | 2.18 | 953.0 | 1,297 |
| L1300BB | 2004-03-27 | 3 | 2,672.02 (3.96) | 2,672 | 2.06 | 1,297.1 | 1,314 |
| M1350WW | 2002-11-03 | not given | 5,710.54 (2.23) | 5,725 | 4.23 | 1,349.97 | 2,927.3 |
| M1900BB | 2004-03-27 | 2 | 6,040.87 (50.64) | 6,100 | 3.23 | 1,870.2 | 2,733 |
| N2700BB | 2006-04-14 | 2 | 11,452.32 (63.70) | 11,452 | 4.15 | 2,758.14 | 5,147.1 |
| N4000BB | 2006-03-30 | 2 | 16,460.94 (96.96) | 16,461 | 4.20 | 3,914.24 | 7,565.7 |

- **The list sits above the measured mean on three:** K1000SK +2.05%, M1900BB +0.98%, M1350WW +0.25%. The 2004
  sheets print a "certified" impulse above the mean: 2,120 on 2,077.51 [D1], 6,100 on 6,040.87 [D4], and 2,675
  on 2,672.02 [D2], where the list says 2,672. NFPA 1125's 2021 revision asks that the
  printed impulse be the mean [N2, FR-8, §8.2.7(1)(d), p. 7/9]. K1000SK's burn time scatters σ 0.41 s on 2.18 s.
- **Tests:** 17 °C, 200 ft elevation on the 2004 sheets [D1, D2, D4]; the 2002 and 2006 sheets don't say.
- **Unchanged after the retest.** The list's "Last Updated" moved from February 2, 2025 to 2026 May 06 to 2026
  August 12, but the six rows and their sheet links are identical in the 2025-02-11, 2026-06-08 and 2026-10-15
  captures [L1, L2, L3]. The August 12 update falls after the test and before the notice.
- **Other certifiers.** TMT's archived K, L and M lists (2003–2012) carry those four as NAR certifications with NAR's
  figures, no TMT test [A1]; the cached TMT N/O lists date from 2003, before the N motors. CAR's index has none
  of the six (its 23 "AMW" rows are Cesaroni's Pro lines) [C1].

## 3. ThrustCurve's entries and the curves HPR Sim flies

None of the six is in HPR Sim's bundled catalog; `hpr motors fetch` takes the first RASP file by source rank
(cert, then maker, then user) and builds the motor from that file's header masses (`crates/hpr-net/src/
on_demand.rs`, `crates/hpr-cli/src/motor_fetch.rs`). Each motor has one RASP and one RockSim file [S1, S2]; no
file states a license. Impulse is the trapezoid sum from (0, 0); burn time is between the 5%-of-peak points.

| Motor | File HPR takes | Curve, N·s | vs mean | Peak, N | Burn 5%, s | Avg, N | File propellant | Catalog I / F / t |
|---|---|---:|---:|---:|---:|---:|---|---|
| K1000SK | RASP, user | 2,077.5 | 0.00% | 1,426.1 | 2.10 | 991 | 1,297 g | 2,120 / 1,000 / 2 |
| L1300BB | RASP, user | 2,672.0 | 0.00% | 1,586.7 | 1.98 | 1,348 | 1,314 g | 2,675 / 1,300 / 2 |
| M1350WW | RASP, cert | 5,723.8 | +0.23% | 1,665.7 | 4.15 | 1,380 | 2,927 g | 5,710.54 / 1,349.97 / 4.23 |
| M1900BB | RASP, user | 6,040.9 | 0.00% | 2,324.7 | 3.13 | 1,927 | 2,733 g | 6,100 / 1,900 / 2.85 |
| N2700BB | RASP, user | 11,350.1 | −0.89% | 3,519.9 | 4.08 | 2,779 | 4,783.7 g | 11,452 / 2,700 / 4.16 |
| N4000BB | RASP, user | 16,268.3 | −1.17% | 5,293.2 | 4.12 | 3,945 | 6,102.6 g | 16,461 / 4,000 / 4.2 |

- **Impulse.** Three user files say they were drawn from NAR's 2004 data and match its mean to 0.03 N·s. Against
  the *list*, the fetched curve reads K1000SK −2.00%, M1900BB −0.97%, N2700BB −0.89%, N4000BB −1.17%. So a lot gap
  for these motors counts from the old measured mean the curves carry, not from the list (**Inference**).
- **Peaks.** N2700BB's curve peaks 2.8% under its sheet's 3,622.57 N, N4000BB's 1.1% under 5,354.56 N.
- **Catalog fields** are mostly the name's nominal thrust: M1900BB's 2.85 s is 11.8% short of the sheet's 3.23 s.
  The sheets' burn times are within 0.01 s of each curve's last sample: last-action times (**Inference**).
- **Masses (a defect, not a lot effect).** N2700BB's and N4000BB's files, in both formats, give the sheet's
  *mass after firing* as the propellant mass, the loaded mass right. HPR Sim then burns 4.78 kg instead of
  5.15 kg (−7.1%) and 6.10 kg instead of 7.57 kg (−19.3%): burnout is 0.36 kg (+7.6%) and 1.46 kg (+24.0%)
  heavy. The exhaust velocities, 2,373 and 2,666 m/s against the sheets' 2,225 and 2,176, pass the
  200–5,000 m/s guard ([exhaust-velocity guard](exhaust-velocity-guard.md)); the RockSim files print 242 and 272 s.
- **Their size (Inference).** A vertical point-mass flight with constant drag (C_D 0.45, invented airframes of
  102 and 157 mm, 12–25 kg empty) loses 0.9–1.2% of apogee on N4000BB and 0.3% on N2700BB. Apogee low is the
  waiver-flattering side. Script: `refs/research/amw-retests/scripts/toy_apogee.py`.
- **Smaller slips.** M1350WW's cert file gives a loaded mass of 5,403 g against the sheet's 5,632.2 (−4.1%), and
  its RockSim file 2,967.3 g of propellant against 2,927.3. L1300BB's RASP header says 75 mm for a 54 mm motor.

## 4. What "within the allowable variances" bounds

No number found in NFPA 1125's revision texts says how far a production motor may stray from its certified
values: changes past "manufacturing tolerances" go to the testing body, with no figure [N2, FR-4, §8.2.6,
p. 6/9]. Its test rules (2021 revision, §8.2.7) hold three numbers, besides a delay rule moot for plugged
motors [N2, FR-8, p. 7/9]: the samples' impulse σ at most 6.7% of their mean; the labeled average thrust within
20% of the measured; the printed impulse equal to the measured mean.
NAR's manual (v1.5, 2011) adds the retest rules [M1]:
- **Count:** two per delay up to 5,120 N·s, one above [§3.3.3, §3.3.5, p. 7]. Plugged, the minimum is 2 each for
  K1000SK and L1300BB and 1 each for the four M and N motors: 8 firings, unless more were fired (**Inference**).
- **Impulse:** a single recertification firing more than 13.1% from the original mean forces a full retest
  [§6.2.3, p. 11]. A mean outside the letter class must be relabeled or retested if more than 1% out and not
  explained by test conditions; the chair may waive it [§6.6.1–6.6.3, p. 12].

**Inference:** the bound each motor's pass implies, if NAR applied its manual, against the 2002–2006 means of §2.

| Motor | Each firing, ±13.1%, N·s | Class floor | Drop the class allows | Avg thrust allowed by 20% |
|---|---|---:|---:|---|
| K1000SK | 1,805 to 2,350 | 1,280 | −38.4% | −12.6% to +31.2% |
| L1300BB | 2,322 to 3,022 | 2,560 | **−4.2%** (−5.2% with 1%) | −16.5% to +25.3% |
| M1350WW | 4,962 to 6,459 | 5,120 | −10.3% (−11.2%) | −16.7% to +25.0% |
| M1900BB | 5,250 to 6,832 | 5,120 | −15.2% | −15.3% to +27.0% |
| N2700BB | 9,952 to 12,953 | 10,240 | −10.6% (−11.5%) | −18.4% to +22.4% |
| N4000BB | 14,305 to 18,617 | 10,240 | −37.8% | −14.8% to +27.7% |

- **Net per motor:** impulse within −13.1% to +13.1% of the old mean, with the floor at −5.2% (L1300BB),
  −11.2% (M1350WW) and −11.5% (N2700BB), or −4.2%, −10.3%, −10.6% without the 1% margin. Average thrust within
  −18% to +31%. Burn time, impulse over average thrust, only within about −34% to +39%: too wide to use.
- **"New and original production"** suggests both were fired; neither group's result is given.
- **The window.** The A2025 cycle's First Revision 3 (to NFPA 1125-2023) would end a plugged motor's impulse at
  2% of peak, not at "all action ceased" [N3, §7.8.5, p. 3/4]. On these curves the cut tail is 0.008–0.023%
  of impulse (`window.py`): negligible against the bounds.
- **What the notice cites** is NFPA 1125, not the manual; NFPA alone bounds only the average thrust column. The
  13.1% and class bounds hold only if S&T applied its own manual, which the notice doesn't say.

## 5. Does it narrow the H+ lot bound?

**No.** hpr-lot-impulse puts the high-power lot gap at 0 to −4%, with −9% not excluded.
- **No measured number.** A pass/fail statement sets an outer limit, not a value; the sign is not stated.
- **Too wide.** Five of the six bounds (−10.3% to −13.1%) admit −9%. Only L1300BB's class floor (−4.2%, or −5.2%
  with the 1% margin) excludes it, and only for that motor, on that reading of the manual.
- **Not today's mainstream production.** One maker, restarted in 2026 after years out of production, six motors
  against 20-year-old means. It says nothing about AeroTech, Cesaroni or Loki lots.
- **What it moves, at 0.6–1.3% of apogee per 1% of impulse** ([apogee bias](apogee-bias.md), from toy flights on
  H to L motors, stretched here): a lot at L1300BB's bound reads 2.5–6.7% high; one at −13.1%, 8–17% (**Inference**).
- **The figure that would:** each June 2026 firing's impulse, average thrust and burn time, split new and original
  production, with temperature and elevation: the summary NAR publishes on its small-motor retest sheets.

## What this suggests

- **hpr-lot-impulse §3** can drop "numbers need a forum login" for "the notice has no numbers, though NAR's
  manual §7.3 asks for a published summary", and quote §4's per-motor floors. Its §5 bound stands.
- **Motor data §5's "high power unbounded"** gets one weak bound: six AMW motors within ±13.1% per firing, one
  (L1300BB) no more than about 5% below, and that waivable. It does not change the "likely low, size
  unmeasured" reading.
- **A fetched-file mass check** could compare a RASP header with ThrustCurve's own `propWeightG` and refuse or warn
  past a few percent: N4000BB's 6,102.6 g against the record's 7,565.7 g. The guard's range can't see it.
  Expected effect: removes a 0.3–1.2% low apogee on these two motors (on the invented rockets above).
- **A catalog note** for K1000SK: its list figure is 2.05% above the measured mean the curve integrates to.

## Open questions

- The June 2026 per-firing figures, and whether NAR will post them as ◊ or ∆ sheets as it does for small motors.
- Whether the M2200SK and M3000ST firings passed, and why they are not in the notice.
- How many motors were fired, which were "original production", and their age.
- Whether S&T applied manual v1.5's 13.1% and class rules, or a newer manual, to these retests.
- Why no summary was published, when manual §7.3 asks for one within 30 days of tests leading to certification.
- Whether any NFPA 1125 clause sets a numeric production tolerance; none was found in the revision texts read.

## Sources

Read 2026-10-15. Cached under `refs/research/amw-retests/` (and the notice and thread pages under
`refs/research/hpr-lot-impulse/web/`), never committed. None states a license: cite, don't copy.

1. **N1** NAR S&T, "Animal Motor Works Test Results", 2026-08-20, one page; attachment of post 1 of `trf:` below,
   `https://www.rocketryforum.com/threads/nar-s-t-notification-amw-2026-08-20.198601/`.
2. **N2** NFPA 1125 A2021 First Revisions, `https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/` +
   `1125_A2021_PYR_AAA_FD_FRStatements.pdf`, FR-4 (§8.2.6), p. 6/9; FR-8 (§8.2.7), p. 7/9.
   **N3** The same path, `1125_A2025_PYR_AAA_FD_FRStatements.pdf`, FR-3 (§7.8.5), p. 3/4.
3. **M1** NAR Motor Testing Manual v1.5 (2011), `https://www.nar.org/wp-content/uploads/2014/08/` +
   `ST-MotorTestingManual.pdf`, §3.3 (p. 7), §6.2.3 (p. 11), §6.6 (p. 12), §7.2 (p. 14).
4. **L1** NAR S&T Certified Motor List (Interactive), "Last Updated: 2026 August 12",
   `https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=670148` (53 AMW rows counted).
   **L2, L3** Its Internet Archive captures `20250211191921` and `20260608182553` (CDX: 8 captures, 2024–2026).
5. **D1–D6** NAR data sheets, `https://www.nar.org/docs.ashx?id=` 1481390 (K1000SK), 1481398 (L1300BB), 1481400
   (M1350WW), 1481403 (M1900BB), 1481411 (N2700BB, p. 2 per firing), 1481413 (N4000BB); 1481404 (M2200SK).
6. **S1** ThrustCurve search API, `https://www.thrustcurve.org/api/v1/search.json`, manufacturer AMW (89 motors).
   **S2** Download API, `/api/v1/download.json`, the six motors (12 files); simfile pages, e.g.
   `https://www.thrustcurve.org/simfiles/5f4294d20002e90000000115/`. **S3** `/motors/AMW/BB-98-17500/`, live and
   Internet Archive capture `20241121030623`.
7. **A1** Archived TMT class lists, as [hpr-lot-impulse](hpr-lot-impulse.md) A2 (K, L, M lists 2003–2012).
   **C1** CAR Certified Motor Index, as hpr-lot-impulse S10 (256 rows, read 2026-10-11).
8. **F1–F3** `trf:` is `https://www.rocketryforum.com/threads/`: `nar-s-t-notification-amw-2026-08-20.198601/`
   (pages 1–2), `amw-54-75-98mm-motors-recerted.198634/`, `amw-motor-testing.197044/`.
