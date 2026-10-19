# How often fetched H+ curve files disagree with ThrustCurve's records on propellant mass

This note surveys every H-to-O solid motor in ThrustCurve's search API, takes the file `hpr motors fetch` would fly,
and compares its header masses with the record's `propWeightG` and `totalWeightG`. It follows
[AMW retests](amw-retests.md) §3 and feeds [issue #578](https://github.com/nrdptel/fusionspace-eridanus/issues/578)
(a fetched-mass cross-check), [motor data](motor-data.md) and [apogee bias](apogee-bias.md). Read 2026-10-15;
**Inference** marks this note's own readings. Corrected 2026-10-16: mass-expended files fly the right burnout mass.

**In short.** Disagreement is common, mostly harmless in sign, and has no natural gap to set a tolerance.
- **One file in three is off by more than 2%.** Of 685 motors with a file and a record, the file's propellant mass
  is beyond 2% of the record's for 223 (33%), beyond 5% for 136 and beyond 20% for 16. The loaded mass agrees far
  better: 57 of 632 beyond 1%, 9 beyond 5% (§2).
- **The file is mostly high, half of those for one reason.** 168 of the 223 are file-high, 84 of them for this
  one: for 98 files the "propellant" is CAR's loaded mass minus its burnout mass, median 5.0% above the propellant.
  Their loaded mass matches CAR's too, so they fly CAR's measured burnout mass (all 98 within 0.72%); only the
  mass during the burn runs light (§3).
- **The flattering side is smaller but real.** 55 files are low by more than 2%, 17 by more than 10%. Of the seven
  beyond −20%, four give the mass after firing as propellant and three are unexplained (§3).
- **The record holds the propellant, the file the burnout mass.** Against CAR's index, the record matches the
  certified propellant for 174 of 179 matched files, the file for 69. But with the file's loaded mass, the record's
  propellant leaves the 98 mass-expended motors heavy at burnout by a median 6.7%. Two records hold CAR's burnout
  mass instead (§4).
- **No natural gap; the guard sees none of it.** Down to −20.5% the file-low gaps have no step over 2.1 points; six
  more scatter from −24.6% to −52.5%. The 200–5,000 m/s exhaust-velocity guard refuses no taken file (§5). On toy
  rockets a 20% low file costs 0.5–0.8% of apogee.

## 1. Population and the file taken

Script `refs/research/fetched-mass-survey/scripts/survey.py` (download: `fetch.py`), run 2026-10-15.
- **Enumeration.** `search.json?maxResults=10000` returns 1,156 motors (the server's own `matches`); classes H to O
  hold 882. Hybrids (143) are excluded, leaving **739 solid motors**: 680 reloads, 59 single-use; 521 regular and
  218 out of production (OOP), both kept and split where it matters. No motor is in class P.
- **Files.** POST `download.json` with `data=file`, 200 ids a batch, per format: 1,249 files (626 RASP, 623
  RockSim), equal to the sum of the records' `dataFiles`. 53 motors have no file, so **686 have one to take**.
- **Selection,** as `find_with` in `crates/hpr-net/src/on_demand.rs` with the build closure of
  `crates/hpr-cli/src/motor_fetch.rs`: RASP first, stable sort by source (cert, mfr, user, other), the first
  file `Motor::from_eng` builds; RockSim (`from_rse`) only when no RASP file builds. The build is emulated in
  Python: UTF-8, the `.eng` reader's rules, one motor, the curve's checks, propellant below loaded mass, and the
  exhaust-velocity range. RockSim XML is read with ElementTree, not roxmltree: an approximation.
- **Within-rank order.** 17 motors have two RASP files tied at the top rank. Per-motor GETs, the request HPR Sim
  sends, gave the same order as the batches for all 17.
- **Result.** Every motor's first-ranked file builds: no file is skipped, and the guard changes no choice. Taken:
  601 RASP (cert 355, mfr 132, user 114) and 85 RockSim (cert 20, mfr 29, user 36). Their licenses: none 412,
  PD 247, free 27.
- **Record fields.** `propWeightG` is on 738 of 739 records (685 of the 686 taken; AMW SK-98-17500 lacks it);
  `totalWeightG` on 644 (632 taken).

## 2. The gaps

Gap = (file − record)/record, on the 685 taken files with a `propWeightG` (`scripts/analyze.py`, `tolerance.py`).

| Propellant mass | n | ≤0.5% | >1% | >2% | >5% | >10% | >20% | p90 | Max |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| All | 685 | 386 | 281 | 223 | 136 | 49 | 16 | 8.4% | 87.6% |
| File above record | | | 189 | 168 | 103 | 32 | 9 | | |
| File below record | | | 92 | 55 | 33 | 17 | 7 | | |
| Source cert | 375 | | 189 | 145 | 87 | 26 | 9 | 8.4% | 45.6% |
| Source mfr | 161 | | 47 | 38 | 22 | 8 | 2 | 6.0% | 32.1% |
| Source user | 149 | | 45 | 40 | 27 | 15 | 5 | 9.9% | 87.6% |

- **Rounding.** 312 agree exactly; 74 more lie within 0.5%, the size of rounding a header to the gram on a motor
  over 100 g. Gaps of 0.5–1% hold 18 files. Above 1% the gap is not rounding, with one caveat: a header printed
  to 0.1 kg can hide either. 502I120-15A's `0.3` kg is the mass-expended 300 g of §3, not a rounded 263 g.
- **Median |gap|** 0.005%; p95 12.6%. RASP and RockSim differ little: beyond 5%, 121 of 600 and 15 of 85.
- **By maker, beyond 2%:** Cesaroni 129 of 248 (123 file-high, 6 file-low), AeroTech 48 of 205,
  AMW 13 of 87, Loki 9 of 55, Gorilla 8 of 45, KBA 7 of 24, Ellis 9 of 18; two others (Kosdon, Estes) 0 of 3.
- **Production status:** beyond 5%, 111 of 520 regular motors and 25 of 165 OOP.
- **Loaded mass** (n = 632): 57 beyond 1%, 28 beyond 2%, 9 beyond 5%, p90 0.80%. The two beyond 20% are Cesaroni
  273H225-14A (RockSim file 293 g against 193 g) and Loki K690-SF (1,800 g against 2,400 g).

## 3. What the tail is

Patterns found by `scripts/classify.py` (one quantity equal to another within 1 g or 0.5%), `car_check.py` and
`shared.py`. No file holds grams as kilograms, and none swaps the two masses.

- **Mass expended given as propellant (file high).** For 98 files the propellant equals CAR's loaded minus
  burnout mass [S4]: 90 Cesaroni (59 cert RASP) and 8 CAR-certified AMW. They exceed CAR's propellant by
  median **+5.0%** (p10 +1.5%, p90 +10.4%, max +15.3%). Example: 168H410-14A's file gives 84.5 g, CAR 75.9 g
  propellant, 182.5 g loaded, 98.0 g burnout. The excess is likely delay grain, ejection charge and liner, some
  of it leaving after burnout (**Inference**: no source found gives what it is made of or the split). For all 98,
  the file's loaded minus propellant is CAR's measured burnout mass within 0.72% (median 0.0%;
  `verify/burnout.py`), so the burnout mass flown is right. The mass falls too fast during the burn, by up to 15%
  of the propellant, and if part of the excess (a delay grain) burns after burnout, the coast is light by that part.
- **Mass after firing given as propellant (file low).** Cesaroni 4937L395-P −35.2%, 548I242-15A −26.3% and
  517I255-16A −17.3% equal CAR's burnout mass [S4]. AMW N4000BB −19.3% and N2700BB −7.1% are the sheets' mass
  after firing (amw-retests §3). AeroTech L339N (−24.6%) and Ellis J228 (−52.5%) fit the same pattern on the
  file's or record's own loaded mass (file + record propellant = loaded mass), unchecked against a sheet. So do
  11 smaller cases, seven of them file-high: AMW SK-38-640, GG-38-640, BB-38-640, SK-54-1400, SK-75-3500 and
  GG-98-11000, Ellis I134, J110 and K475, Gorilla K700RT and AeroTech L1520T.
- **Mass after firing in the record (file closer to right).** Cesaroni 6026M1670-P (+45.6%) and 7579M1520-P
  (+25.4%): the records' 2,130 and 2,981 g are CAR's burnout masses; CAR's propellant is 2,956.1 and 3,602 g, and
  the files (mass expended) sit 4.9% and 3.7% above it [S4]. 654J316-17A (+36.9%, no CAR row) fits the same
  pattern, and its record implies 2,551 m/s (§4).
- **A sibling reload's masses (either sign).** Nine groups of same-maker files, each with at least one file off
  by more than 2%, share identical header masses; the Cesaroni and Gorilla groups are not sibling-mass cases (the
  Cesaroni pair are mass-expended files). AeroTech Redline files whose header says only "provided by
  ThrustCurve.org" carry the masses of the sister reload, converted from 1997–1999 TMT data: H148R = H73J
  (+21.2%), H210R = H180W (+13.7%), I366R = I284W (+4.5%), M1550R = M1315W (+8.8%). AeroTech's page gives the
  H148R 115 g of propellant [S5], 6% under the record's 122 g. Loki H144-LW, I405-LW and L930-LW carry another
  propellant's masses on the same case (−4.8% each); J175-LW and J525-LW share one header (−9.8%, −5.6%).
- **Unexplained.** Cesaroni 14005N1100-P, 4,517 g against CAR's 6,790 g (−33.5%); AeroTech HP-I500T −20.5%; Loki
  K690-SF −32.1% with the loaded mass −25%; Ellis J148 +87.6% (670 against 357.2 g), side unknown.
- **The rest.** About 104 of the 223 match no pattern: 115 before the 11 after-firing fits above are taken out,
  which were 43 Cesaroni (39 with no CAR row by name), 40 AeroTech (28 cert files) and 32 from five other makers.

## 4. Which side is wrong

- **CAR-certified motors.** 179 taken files match a CAR row by designation (154 Cesaroni, 25 AMW) [S4]. The record
  equals CAR's propellant for 174; the file for 69, the mass expended for 98, the burnout mass for 4.
  So for these makers the record is the better source for the propellant (`car_check.py`), but not for the
  burnout mass: the record's propellant with the file's loaded mass leaves the 98 mass-expended motors heavy at
  burnout by a median 6.7% (0.6% to 45.6%, the top where the record itself is wrong; `verify/burnout.py`).
- **CAR's own rows have slips:** 836J210-16A lists 82.7 g of propellant (ISP 403 s), and 287H340-14A and
  305H226-14A look swapped (**Inference**: 305H226's CAR propellant, 206.7 g, exceeds its loaded minus burnout
  mass, 184 g). Their files and records agree, so they are not in the tail; they are 3 of the 5 records that miss
  CAR's propellant. By a 1 g rule, 305H226's record (175 g) also matches CAR's burnout mass (176 g): the third
  burnout-mass match of §4's four, put down to the swap.
- **Exhaust velocity as an arbiter.** CAR's ISP column for its 196 H-and-up rows has p5 142.5 s, median 194.6 s,
  p95 224.2 s; the highest credible is 233.7 s (6800-M3700-P), 2,292 m/s [S4]. A propellant mass that implies more
  than about 2,450 m/s (250 s) is above every credible CAR row (**Inference**). It flags the file side of
  J228 (4,128 m/s), 14005N1100-P (3,109), N4000BB (2,666) and HP-I500T (2,511), and the record side of
  6026M1670-P (2,829), 7579M1520-P (2,542), 654J316-17A (2,551) and J148 (2,530). M6000ST reads 2,483 m/s only
  with the file curve's 9,606 N·s, 16% over the record's 8,247.1 N·s; on the record's impulse it is 2,132 m/s.
- **It can't stand alone.** Two motors whose file and record agree also exceed 2,450 m/s: AeroTech HP-H135W
  (2,793) and K400C (2,652, a bundled motor). And a modest file-low gap stays inside the band: N2700BB's
  −7.1% file gives 2,373 m/s.

## 5. Tolerance, guard and size

**There is no natural gap.** The file-low gaps beyond −0.5% run −0.5, −0.6, … −2.0, −2.2,
−2.3, −2.4, −2.9 … −9.8, −11.4, −12.0 … −17.3, −19.3, −20.5, then −24.6, −26.3, −32.1,
−33.5, −35.2, −52.5 (106 values). No step down to −20.5% exceeds 2.1 points; the six beyond
it are scattered, with steps up to 17.3 points, too few to set a bound. The two-sided ladder
is denser still, filled by the mass-expended files at +1 to +15%.

| Tolerance on propellant | 1% | 2% | 3% | 5% | 7.5% | 10% | 15% | 20% |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Taken files flagged, either sign (of 685) | 281 | 223 | 197 | 136 | 82 | 49 | 23 | 16 |
| Taken files flagged, file below record | 92 | 55 | 49 | 33 | 24 | 17 | 10 | 7 |

- **Which way each errs.** A file-low propellant leaves the burnout mass heavy and the apogee low: the
  waiver-flattering side. A two-sided check at 2–5% would also fire on most of the 98 mass-expended files, which
  fly the measured burnout mass. A one-sided check at 5% flags 33 motors, both AMW N files among them; at
  10% it misses N2700BB (**Inference**).
- **The guard.** Every taken file's `c` is 1,274 to 4,128 m/s (`analyze.py`), inside 200–5,000 m/s, so the guard
  refuses none, and it can't: a 20% mass error moves `c` by 25%, a unit slip by 1,000 times.
- **Apogee size.** `scripts/toy_sensitivity.py` flies three invented rockets (H128W on 1.2 kg, K456DM on 4 kg,
  M1297W on 12 kg; C_D 0.45) with the propellant scaled at a fixed loaded mass. Apogee moves 0.026–0.042% per 1%
  of propellant gap: −20% costs 0.52–0.84%, +10% adds 0.23–0.41% (not for mass-expended files, whose burnout
  mass is right). AMW retests found 0.9–1.2% for N4000BB.
- **Bundled catalog** (built from headers, [ThrustCurve data](thrustcurve-data.md) §6): of 22 H+ curves,
  21062O3400-P (+3.1%) carries the mass-expended header and so flies CAR's 5,570 g burnout mass; 3300L3200-P
  is +6.6% with no CAR row to tell.

## What this suggests

- **Issue #578's check, one-sided.** Warn when the file's propellant mass is below the record's by more than a stated
  tolerance, naming both masses. No natural gap sets the number; 5% is a choice: it catches both AMW N files,
  flags 33 of 685 motors, and what it lets through costs about 0.2% of apogee or less on the toy rockets.
  Its tests would pin N4000BB-like and N2700BB-like files and one at −4.9%.
- **Not the record's propellant on its own.** Swapping it in for the file's would fix the propellant on 174 of
  179 CAR-matched motors, but leave the 98 mass-expended motors heavy at burnout (median 6.7%), the
  waiver-flattering side. A record-plausibility bound at 2,450 m/s would refuse seven records: the two
  burnout-mass ones, and 654J316-17A, J148, M6000ST, HP-H135W and K400C, whose files agree (**Inference**).
- **Motor data's trust statement:** a fetched file's propellant is within 0.5% of the record for 386 of 685 H+
  motors and beyond 5% for 136; the loaded mass is beyond 1% for 57 of 632.
- **Apogee bias:** on the three toy rockets, the low-apogee share from fetched masses is under 1% of apogee on all
  but about six of the 55 file-low motors (up to about 2% for J228), and near zero on the median motor. On those
  rockets it cannot explain a bias of several percent.

## Open questions

- How much of the mass-expended excess leaves before burnout: delay, charge and liner masses per reload.
- Whether TRA's or NAR's sheets confirm the AeroTech, Loki, KBA and Ellis tail cases; only CAR's index and one
  AeroTech page were read here.
- Why HP-H135W and K400C imply 2,793 and 2,652 m/s in both file and record: an impulse or a propellant mass slip.
- Where the 14005N1100-P file's 4,517 g, the K690-SF file and the J148 record come from.

## Sources

Read 2026-10-15 unless noted. Downloads cached under `refs/research/fetched-mass-survey/data/`, never committed.

1. **S1** ThrustCurve search API, `https://www.thrustcurve.org/api/v1/search.json?maxResults=10000`
   (`search/search-all.json`, 1,156 motors). No license granted for records, and the site reserves
   copyright: cite, cache, don't commit.
2. **S2** ThrustCurve download API, `https://www.thrustcurve.org/api/v1/download.json`, POST by format
   (`download/`, 8 answers, 1,249 files); 17 per-motor GETs (`pergets/`). Each file carries its own license
   field (412 of the taken files none, 247 PD, 27 free): cache only.
3. **S3** HPR Sim source: `crates/hpr-net/src/on_demand.rs` (`find_with`, `rank`), `crates/hpr-cli/src/
   motor_fetch.rs` (`fetch`), `crates/hpr/src/motor.rs` (`from_eng`, `from_rse`), `crates/hpr-motor/src/
   motor.rs` (`from_envelope`, `EXHAUST_VELOCITY_RANGE_M_S`), at the worktree's `main`.
4. **S4** CAR Certified Motor Index, Airtable view `https://airtable.com/appK3MU9MDNJXqpSy/shrDAtESEzXPPAxP3`,
   linked from `https://car-acf.org/en/certifications/motor-certifications`; read 2026-10-11 for
   [high-power lot impulse](hpr-lot-impulse.md) S10, rows copied to `certs/car-rows.json` (256 rows; columns
   propellant, loaded and burnout weight, ISP). No license stated: cite, don't copy.
5. **S5** AeroTech, "H148R-14A RMS-38/240 Reload Kit", `https://aerotech-rocketry.com/products/` +
   `product_5234c3bb-3833-ae4e-3b1a-e7c2ce572d9a` (`makers/at-H148R.html`): propellant 115 g, motor 274 g.
6. **S6** NAR data sheets for AMW N2700BB and N4000BB, as [AMW retests](amw-retests.md) D5, D6.
7. **Scripts** (`refs/research/fetched-mass-survey/scripts/`, run with `python3 -I`): `fetch.py`, `survey.py`
   (writes `data/rows.json`), `analyze.py`, `classify.py`, `car_check.py`, `shared.py`, `tolerance.py`,
   `toy_sensitivity.py`; their outputs are the `data/*-out.txt` files. `verify/` holds the check pass's recounts.
