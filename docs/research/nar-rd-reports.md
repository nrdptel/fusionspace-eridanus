# Which of NAR's 540 R&D reports hold numbers HPR Sim can use

NAR's R&D index lists 540 reports, journals and proceedings entered at NARAM and NARCON since 1963. This note sorts
all 540 titles by subsystem, ranks those likely to hold measured numbers HPR Sim can compare against, and gives a
fetch list ([Hagerty's NARAM-52 tunnel report](hagerty-tunnel-lift.md) is the worked example). **In short: about two
dozen unread reports are worth opening, and none is likely to add a measured flutter onset.** The likeliest are drag
coefficients and simulation-against-flight checks (K. Steele 2006 and 2009, Stevens 2005 and 2007, Karbon 2026), a
tube-fin stability test (Avecilla 1997), streamer tests (Flanigan, Estrada), small-canopy descent rates (Kidwell,
K. Steele) and altimeter speed tests (Rangitsch, Kidwell). Eight cached reports have never been read into a note.
How far to trust it: **every rank is inferred from a title**; no unread report was opened (they need a member
sign-in, not available for this note; a plain download returns HTTP 403). Two public youth reports, read, show a
title is a weak guide to whether a report holds a usable number.

## 1. What the index holds

- **540 entries.** The cached index (2026-10-17) holds 547 `docs.ashx` links to 540 ids, NARAM-5 (1963) to virtual
  NARCON-2026 [1]: 18 from NARAM-19 and before, 22 from NARAM-20 to 28, 77, 162 and 164 from the 30s, 40s and 50s,
  54 from NARAM-60 to 63, 13 from three NARCONs, 27 "Other Reports" (7 MIT Rocket Society journals and 10 MITCON
  proceedings among them, contents unlisted) and 3 books. It isn't complete: A. Stenberg's "Alternatives for Sealing
  Balsawood" (NARAM-53) is in NAR's 2015 bibliography [2] p. 28, not in the index.
- **By subsystem**, one per entry, from titles by a keyword pass and 55 hand assignments:

| Subsystem | Entries | Subsystem | Entries |
| --- | ---: | --- | ---: |
| Recovery (chutes, streamers, helicopters) | 112 | Drag | 22 |
| Altimeters and flight data | 59 | Staging | 18 |
| Boost and rocket gliders | 56 | Simulation accuracy | 17 |
| Motors and thrust | 42 | Flutter and structures | 17 |
| Other (launchers, contests, scale, education) | 176 | Stability | 15 |
| | | Wind tunnel (rocket aero) | 6 |

- **Two index titles look like copy errors.** Id `1358230` (A. Stenberg, NARAM-52) repeats the title of `1358219`
  (J. Rains, same meet), and `1358276` (E. Kristal, NARAM-55) repeats `1358275`'s. The 2015 bibliography lists
  those authors at those meets as "Streamer Optimization Testing with High Altitude Drop" (p. 28) and "Video
  Analysis of Motor Staging" (p. 27) [2]. **Inference:** the bibliography's titles are the real ones; opening the
  files settles it.

## 2. Already read or cached

Read into notes: Hagerty ([hagerty-tunnel-lift](hagerty-tunnel-lift.md)); Bock and Sowden (Snowden on its title
page, [nar-finish-reports](nar-finish-reports.md)); public copies of DeMar, McAtee, the Van Milligans' nose, tube-fin
and lug reports and Barrowman's NARAM-8 CP paper ([hobby-rocket-tunnel-data](hobby-rocket-tunnel-data.md)); Reynolds,
Nau and Coffee ([tumbling-vs-fixed-drag](tumbling-vs-fixed-drag.md)); Kidwell's streamer drops
([streamer-and-tumble-drag](streamer-and-tumble-drag.md)); T. Van Milligan's C_D
([flight-derived-cd](flight-derived-cd.md)).

**Cached under `refs/papers/` and never read into a note:** Staab, APCP F delay variability (`1358263`, NARAM-55);
Pod Bay Doors, igniter placement and delay (`1358077`, NARAM-46); Weaver, delay changes with age (`1357862`, NARAM-31);
Gaff, extreme cold on BP motors (`1357846`, NARAM-26); Gormley, booster separation in delayed staging (`1357908`) and
boosted dart recovery (`1357941`); Cody Steele, drag separation (`1358096`, NARAM-47); Flanigan, Evaluation of
Filters for Competition Altimeters (`1705658`, vNARCON-2026). [Motor delay spread](motor-delay-spread.md) still lists
the four delay and cold reports as not read. Rangitsch's backslide report (`1358120`, NARAM-49), also cached, was read
for this note: CFD of a broadside body and six flights, five timed, with no measured apogee, so no measured
coefficient (Table 1 and p. 4) [3].

## 3. Aerodynamics: drag, stability, simulation accuracy

Tier A: very likely measured numbers that move a named item. Tier B: probably measured, a weaker link. All from titles.

| Tier | id | Report (author, NARAM, year) | What it would move |
| --- | --- | --- | --- |
| A | `1358116` | Experimental Determination of Model Rocket Drag Coefficients (K. Steele, 48, 2006) | drag bias |
| A | `1358125` | Estimating and Verifying Drag Coefficients (C. Stevens, 49, 2007) | drag bias |
| A | `1358183` | Simulation Accuracy Compared to Actual Flights (K. Steele, 51, 2009) | apogee bias, small rockets |
| A | `1705661` | (Karbon, vNARCON-2026; title below) | drag; CP if given |
| A | `1357957` | Stability of Tube Fins (S. Avecilla, 39, 1997) | #228, #234 (tube-fin CP) |
| A | `1358099` | Launch Lug Drag (C. Stevens, 47, 2005) | the lug term in apogee-bias |
| B | `1358137` | Dual Finned Rockets (J. Smith, 50, 2008) | [tandem-fin downwash](tandem-fin-downwash.md) |
| B | `1358043` | Empirical Stability of Spool Rockets (E. Strother, 44, 2002) | #611 (measured C_Nα, CP) |
| B | `1358164` | Triform vs Cruciform Fins (J. Dunbar, 50, 2008) | #611 |
| B | `1357849` | Evaluation of Drag Coeff & Altitude Prediction Methods (M. Steele, 28, 1986) | drag bias |
| B | `1358101` | Optimizing Rocket Shape to Maximize Altitude (A. Calvert, 47, 2005) | drag bias |
| B | `1358268` | Effect of Turbulent Flow on Fin Boundary Layer (B. Hampton, 55, 2013) | finish, laminar share |

- **Why these.** Each title names an experiment, a verification or a flight comparison; Karbon's is "Assessing the
  Accuracy of Aerodynamics Simulation Through CFD and Flight Testing". [Tube-fin data](tube-fin-data.md) named
  Avecilla the likeliest measured tube-fin CP; [flight-derived C_D](flight-derived-cd.md) lists Steele and Calvert.
- **Further B rows:** drag against superroc length, fineness and airspeed (`1358104`, `1357959`, `1358015`); fin
  and nose shape (`1358308`, `1358304`, `1358364`); altitude prediction against flight (`1358185`, `1358297`,
  `1358384`); precision-event variability (`1358147`).
- **No unread title promises a directly measured CP or C_Nα** (McAtee's and Hagerty's tunnel reports did; both are
  read). The stability rows above are the only unread leads for #172 and #611.

## 4. Flutter and structures

- **No title promises a measured flutter onset.** Only four titles name flutter or aeroelasticity: Flanigan's Fin
  Flutter Nomographs (`1358139`, 2008), his two super-roc aeroelastic analyses (`1358213`, `1358246`) and Micci's
  assumed-mode flutter of glider wings (`1357813`, 1975). All read as analysis [1]. So the index is unlikely to add
  any of the three measured fins [M2.11](../ROADMAP.md) needs. Opening `1358139` would confirm it holds no tests.
- **Hobby material inputs, maybe.** Fin Material Strength (A. Stenberg, `1358140`, 2008) and Analysis of Balsa
  Material Data (B. Guzek, `1358198`, 2010) might give balsa or fin-stock moduli, an input to the flutter estimate,
  not evidence for it. Tube buckling and stiffness reports (`1358215`, `1358419`) have no model in HPR Sim to feed.

## 5. Recovery, motors, altimeters, staging

| Tier | id | Report (author, NARAM, year) | What it would move |
| --- | --- | --- | --- |
| A | `1358319` | Flight and Wind Tunnel Testing of Streamers (C. Flanigan, 58) | streamer C_D, pleats risk |
| A | `1358330` | Streamer Pleats (R. Estrada, 58) | streamer pleats risk |
| A | `1358230` | Streamer Optimization Testing with High Altitude Drop, per [2] (A. Stenberg, 52) | streamer C_D |
| A | `1358053` | Performance of Plastic and Mylar Chutes (C. Kidwell, 44, 2002) | #741, M5.6b canopy C_D |
| A | `1358130` | Parachute Descent Rates (K. Steele, 49, 2007) | #741, M5.6b |
| A | `1358260` | Velocity Effects on Altimeter Performance (M. Rangitsch, 54, 2012) | baro error with speed |
| A | `1358252` | Dynamic Pressure & Altimeters (C. Kidwell, 54) | baro port placement |
| A | `1358001` | Effect of Altitude on Engine Performance (E. Langford, 42, 2000) | thrust at altitude |
| A | `1358097` | Failure Rates in Rocketry (T. Cochran, 47, 2005) | fault base rates |
| A | `1358113` | Ejection Charge Variability (Cody Steele, 48, 2006) | ejection charge spread |

- **Why these.** Streamer pleats are an open known risk (+58% on Kidwell's drops, [known risks](known-risks.md));
  Estrada's title names pleats, and Flanigan's tests streamers in a tunnel and in flight. Kidwell wrote the only
  streamer drops HPR Sim uses. Langford's report sits under engine static testing in the bibliography [2] p. 13, so
  it may hold firings.
- **Tier B:** ejection charge strength (`1358017`, `1358024`); BP impulse against mass (`1358204`, `1358190`,
  `1358336`); Barber's BP properties against pressure and temperature (`1357825`); cycling and D12 failures
  (`1357888`, `1357818`, `1357838`); deployment failures (`1357991`, `1358006`); reefing, vents, descent stability,
  drop tests and shroud lines (`1358470`, `1358370`, `1358005`, `1357965`, `1358225`, `1357983`, `1357990`);
  altimeter venting, spread and temperature (#507, #871). Helicopter recovery (33) ranks C: no rotor model.

## 6. What two public copies show

Two NARAM youth parachute reports by A. Van Milligan have public copies on Apogee's site [4], so they could be read.

- **Number of parachutes (NARAM-52, `1358200`).** 200 in² of trash-bag hexagons as one, two or three canopies, each
  system 29.5 g, dropped 7.55 m off a roof and timed by stopwatch [4a]. Table 21 prints a one-canopy average of 8.54
  ft/s, but its ten rows (6.52 to 9.95) average 7.69. **Inference:** C_D 0.68 (ρ 1.2) to 0.82 (ρ 1.0) on the total
  area. Timed from apogee in flight, the same canopies give 0.31 to 0.38 for one (Table 14), 0.36 to 0.43 for two and
  0.35 to 0.42 for three (Tables 15 and 16).
- **Spinning parachutes (NARAM-53, `1358245`).** 18 in canopies, 27.8 g, same roof: a plain canopy fell at 4.65 ft/s
  [4b] Chart 3. **Inference:** C_D 1.38 to 1.65 if 18 in is a flat circle's diameter (the area isn't stated), about
  twice the year before's corrected drop value by the same method.
- **For the ranking.** A 7.55 m fall from rest doesn't reach terminal speed: with drag as speed squared, the mean
  speed reads about 5% low at 2.3 m/s and 2% low at 1.4 m/s, so C_D reads 11% and 4% high (this note's calculation),
  too little to explain a factor of two. Short-drop youth reports rank C for C_D; Kidwell, Flanigan, K. Steele rank A.

## 7. A side find: impulse at altitude

Searching for public copies turned up an Air Force Academy static test [5]: ten firings per motor at about 77 kPa
(the abstract's 22.80 inHg is 77.2; the methods say 77.9) and 101 kPa. Mean impulse rose from 2.006 ± 0.022 to
2.232 ± 0.029 N·s for an Estes A10-PT (+11.3%) and from 17.315 ± 0.087 to 18.308 ± 0.103 N·s for an Estes D11-P
(+5.7%) (pp. 3–4); a long-burn Quest D5-0P changed by 0.021 ± 0.182 N·s, no gain (pp. 4–5). The authors predicted
the gain as Δp times the nozzle throat area times the burn time: 0.167 and 0.754 N·s. The measured gains are 35% and
32% larger (p. 4; the authors write 20–25%). The appendix tables (pp. 6–8) agree with p. 4 on the makers; the
abstract swaps them. [Apogee bias](apogee-bias.md) sizes the "thrust at altitude" term from (p_test − p) times the
exit area, with no measurement; this is one, at small scale.

## Fetch list

With the NAR sign-in, open `https://www.nar.org/docs.ashx?id=<id>` and save under `refs/papers/` as named: §3's rows
on the left, §5's and two checks on the right, then §4's `1358140` and `1358198`. Cache and cite, never commit.

| id | Save as | id | Save as |
| --- | --- | --- | --- |
| `1358116` | `steele-2006-naram48-drag-coefficients.pdf` | `1358319` | `flanigan-naram58-streamer-tunnel-flight.pdf` |
| `1358125` | `stevens-2007-naram49-verifying-cd.pdf` | `1358330` | `estrada-naram58-streamer-pleats.pdf` |
| `1358183` | `steele-2009-naram51-simulation-accuracy.pdf` | `1358230` | `stenberg-2010-naram52-streamer-drop.pdf` |
| `1705661` | `karbon-2026-vnarcon-aero-sim.pdf` | `1358053` | `kidwell-2002-naram44-plastic-mylar-chutes.pdf` |
| `1357957` | `avecilla-1997-naram39-tube-fin-stability.pdf` | `1358130` | `steele-2007-naram49-parachute-descent.pdf` |
| `1358099` | `stevens-2005-naram47-launch-lug-drag.pdf` | `1358260` | `rangitsch-2012-naram54-altimeter-velocity.pdf` |
| `1358137` | `smith-2008-naram50-dual-finned.pdf` | `1358252` | `kidwell-naram54-dynamic-pressure-alt.pdf` |
| `1358043` | `strother-2002-naram44-spool-stability.pdf` | `1358001` | `langford-2000-naram42-altitude-engine.pdf` |
| `1358164` | `dunbar-2008-naram50-triform-cruciform.pdf` | `1358097` | `cochran-2005-naram47-failure-rates.pdf` |
| `1357849` | `steele-1986-naram28-drag-prediction.pdf` | `1358113` | `steele-2006-naram48-ejection-charge.pdf` |
| `1358101` | `calvert-2005-naram47-shape-altitude.pdf` | `1358276` | `kristal-2013-naram55-staging-video.pdf` |
| `1358268` | `hampton-2013-naram55-fin-bl.pdf` | `1358139` | `flanigan-2008-naram50-flutter-nomographs.pdf` |

## What this suggests

- Run the fetch list, then read the tier A aero reports first: K. Steele 2006 and 2009, Stevens 2007 and Avecilla
  are the likeliest measured references for the drag bias and the tube-fin CP (#228).
- Read the eight cached reports (no sign-in needed) into [motor delay spread](motor-delay-spread.md) and others.
- Look for M2.11's fluttering fins outside NAR's index: no title in it promises one.
- Put the Academy's measured impulse gain beside apogee-bias's formula for the thrust-at-altitude term.

## Open questions

- Whether each tier A report holds what its title suggests: none was opened (§6 shows why that matters).
- What the MIT Rocket Society journals and MITCON proceedings hold. The bibliography [2] places a D12 failure study
  in a 1980 MIT journal (p. 12; **Inference:** the April 1980 issue, `1358405`, the index's only 1980 issue) and a
  trajectory paper "with experimental verification" in the 1975 proceedings (`1358396`, p. 5).
- Why the Academy's small motors gained about a third more than its throat-area formula; whether the exit area
  in its place explains the gap was not checked.

## Sources

1. NAR, R&D Reports index, <https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>;
   copy of 2026-10-17, read 2026-11-03. Titles public; the reports are members' copyrighted work behind a sign-in.
2. T. Barber, *Sport Rocketry Technical Bibliography 1958–2014*, NAR, April 2015, Wayback copy of
   <https://www.nar.org/wp-content/uploads/2014/05/Rocketry-Technical-Bibliography-Apr-2015.pdf>, read 2026-11-03,
   pp. 5, 12, 13, 27, 28. NAR copyright: cited, cached privately.
3. M. Rangitsch, "Aerodynamics of Backslide Recovery Rockets", NAR R&D report, NARAM-49, 2007, docs.ashx id
   1358120, read 2026-11-03, Table 1 and p. 4. Members' copyrighted work: cited, cached privately.
4. Apogee Components, "R and D Projects From The N.A.R.", <https://www.apogeerockets.com/Tech/RD_Projects_from_the_NAR>,
   read 2026-11-03. No license stated: cited only.
   a. A. Van Milligan, "Comparison of Number of Parachutes Versus Descent Rate", NARAM-52, 2010,
      <https://www.apogeerockets.com/downloads/PDFs/Cluster_Parachutes.pdf>, Tables 14 to 16 and 21.
   b. A. Van Milligan, "Spinning Parachute Effect on Descent Rate", NARAM-53, 2011,
      <https://www.apogeerockets.com/downloads/Spinning%20Parachutes.pdf>, Chart 3.
5. K. Albanese, M. Meyers, K. Prusko, M. Courtney, "Altitude Dependence of Rocket Motor Performance", US Air Force
   Academy, 2012, DTIC AD1025239, <https://apps.dtic.mil/sti/pdfs/AD1025239.pdf> (read from a Wayback copy,
   2026-11-03), pp. 1, 3–8. Marked "Distribution A. Approved for public release. Distribution unlimited."
