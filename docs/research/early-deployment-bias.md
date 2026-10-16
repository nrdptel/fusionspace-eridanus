# Why the early-deployment flights read so high

**In short: the charges cost almost nothing; the 19 flights come from different rockets.** On the private collection's
53 scored flights ([report](../../validation/reports/fixture-flights.md)), the 19 flagged for a charge or separation
before the peak read +17.29% (HPR Sim) and +16.03% (OpenRocket) above their logs, against +3.95% and +3.19% for the
other 34. A fit to each flight's coast just before its event puts the uncut peak within −0.25% to +0.26% of the logged
apogee; adding that climb back moves the 19's mean by 0.01 point. Instead, 15 of the 19 come from 2 of the collection's
16 sources, and the flag mostly marks altimeters that fire at their own apogee detection, a fraction of a second early.
This note builds on [apogee-bias](apogee-bias.md), which asked the question, and
[ADR-184](../decisions/0184-logged-apogees-of-the-private-collection.md), which flags the flights. Everything about the
collection here is an aggregate (ADR-151). How sure: high that 18 of the 19 charges cost under 0.3% of their apogee
(the one 2.6 s early, under about 1%); medium on the sources, since one source still shows a gap of its own (below).

## What an early charge can cost (the bound)

- **No more than the climb left when it fires.** A rocket coasting up at t seconds before apogee has about g·t²/2 left
  to climb: 4.9 m at 1 s, 11.0 m at 1.5 s, 19.6 m at 2 s, 44 m at 3 s. Integrating a coasting 98 mm rocket back from
  apogee with drag gives within 2% of that this near the top (**Inference**, computed).
- **Extra drag takes only part of it.** OpenRocket's technical documentation fits a tumbling stage with C_D 0.56 on the
  body's side area and 1.42 on the fins' [1] pp. 54–55. On a 98 mm, 2 m airframe the side term alone is 29 times the
  nose-first drag area at C_D 0.5 (**Inference**). Even so, tumbling loses about 1.0 m of the 11.0 m left at 1.5 s,
  and tumbling plus a 0.61 m drogue opening at once (C_D 1.5) 3.6 m, for 6 kg in air of 1.10 kg/m³; a 3 kg rocket
  loses 1.8 and 5.0 m (**Inference**: a point-mass integration, fins and line-out left out).
- **What the gap would need.** A flight at +3.95% reads +17.29% only if its log loses 1 − 1.0395/1.1729 = 11.4% of its
  apogee: 114 m of a 1,000 m flight, a charge about 4.8 s early even if all the climb left were lost (**Inference**).
  ADR-184's readers timed every charge under 1.5 s from the peak.
- **Charge gas makes the reading go low, not high.** Gas reaching the av-bay "temporarily raise[s] its pressure" [3]
  p. 8; PerfectFlite, MissileWorks and Entacore ask for sealed bays for the same reason [4] pp. 7–8, [5] p. 4, [6] PDF
  p. 10. A spike can only hide a peak inside it, and ADR-184 then takes the reading from before the charge.
- **Air past a port on a tumbling piece goes either way,** by up to v²/2g: about ±11 m at 15 m/s (**Inference**).
  Several small ports make the reading depend less on crosswind or orientation [3] p. 7, [4] p. 9, [5] p. 4.

## What the 19 logs show (aggregates of the private collection)

- **The split reproduces.** The per-flight rows the report's run of 2026-10-15 printed give the report's four group
  means to 0.01 point, so OpenRocket's and HPR Sim's errors below are the report's.
- **The cost, measured.** On each of the 19 traces, a quadratic fit to the 1.5 s before the event puts the uncut peak
  −0.25% to +0.26% from the logged apogee (median +0.01%, mean −0.01%; −2.9 to +3.1 m). Fits over 1.0 s and 2.5 s
  agree (means −0.02% and −0.03%). The fits slow at 7.8 to 10.3 m/s², near g, as a coast near the top should.
- **With the climb added back** the 19 read +16.04% (OpenRocket) and +17.30% (HPR Sim); the 53 move by 0.01 point
  or less.
- **The charge's own spike** reads up to 1.04% above the logged apogee (median 0.23%). Taking the reading before the
  charge, as ADR-184 does, raises the 19's mean by about 0.42 point over the files' maximum; it is the right reading.
- **Every event is real.** The 0.6 s after each event leaves the pre-event fit by 3 to 4,350 times the fit's scatter
  (18 of 19 by 8 times or more); the seven logs with an accelerometer show a jolt at the event.

## What fired, and when

- **16 of 19 are an altimeter firing at its own apogee detection,** 0.17 to 0.72 s before the fitted peak while still
  climbing 1.7 to 5.7 m/s (medians 0.35 s and 3.2 m/s). Two are likely motor-delay charges; one is unknown.
- **That timing is ordinary.** PerfectFlite's StratoLogger finds apogee "typically within +/- 0.2 seconds" [4] p. 2;
  Altus Metrum's within "a few sensor samples" [2] p. 66; Eggtimer fires 1 s after the highest altitude [8] p. 31;
  Silicdyne's Fluctus may react up to 1,000 ms late [7] p. 25. **Inference:** which flights get the flag depends on
  which altimeter and settings a flier uses, so it travels with the source.
- **Only one fired 1.5 s or more early:** a likely motor-delay charge 2.6 s before its fitted peak (1.3 s before the
  raw one) at 24.5 m/s. Its logged peak sits 0.11% below the ballistic extrapolation, but its trace is disturbed
  after the charge, so its cost is bounded only at under about 1% (**Inference**).
- **None fired in transonic flight.** Every event climbs at under 25 m/s; the two fastest early flights (OpenRocket peak
  Mach 1.38 and 1.50) fired 0.4 to 0.5 s before their peaks. A Mach false apogee, where the shock's pressure rise reads
  as descent [4] p. 2, [9] PDF p. 8, [2] p. 39, would fire near burnout, many seconds early (**Inference**).
- **Motor delays scatter, and selection runs the wrong way.** NFPA 1125 lets a delay differ from its label by 1.5 s or
  20%, whichever is greater, up to 3 s [10] p. 7. On about a hundred NAR data sheets (some 250 labeled delays,
  counted twice), the median ran 0.13 to 0.14 s (2% to 2.5%) short [11]. **Inference:** every cause that makes a
  simulation read high shortens the real coast, so a delay picked from that simulation fires late. Early motor charges
  should mark rockets with a *lower* bias, not a higher one.

## Which rockets the 19 are

| | 19 early | 34 other |
|---|---:|---:|
| OpenRocket error, mean / median | +16.03% / +15.46% | +3.19% / +5.73% |
| HPR Sim error, mean / median | +17.29% / +16.15% | +3.95% / +5.44% |
| sources (teams or fliers) | 4 | 14 |
| distinct rockets | 13 | 23 |
| flights with OpenRocket peak Mach 0.8 or more | 6 | 17 |
| mean time to apogee (OpenRocket) | 19.2 s | 20.6 s |

- **Two sources carry 15 of the 19.** One source has all 10 of its flights flagged, at +17.6% (OpenRocket); on 4 of
  those 10, the source's notes put the flights 11% to 21% below the flier's own pre-flight simulation. The rockets
  overshoot in their builder's own tool too: that miss is the rockets' or the files', not the charges' (**Inference**).
- **Each source counted once,** the early group is +11.25% (4 sources) and the rest +2.57% (14 sources); all 53, +3.70%
  (OpenRocket) and +4.40% (HPR Sim). The 12 sources with no early flight range from −12.3% to +21.5%.
- **Without that one source** the 43 flights read +5.51% (OpenRocket) and +6.62% (HPR Sim). The 9 early flights left
  average +14.28% but have a median of +3.41%: four of five from one source read high; the fifth and four from two
  others sit near zero.
- **Every stratum the counts allow keeps a gap:** below Mach 0.8, +17.70% (13 early) against +5.46% (17); Mach 0.8 to
  1.2, +12.34% (4) against −0.03% (12); class J or below, +19.13% (8) against +5.97% (20). Class K is 8 early to 1,
  all 8 from one source, so motor class and source can't be told apart.
- **One source still has a gap of its own.** The only source with both kinds in numbers reads +25.3% on 5 early flights
  and +9.7% on 9 others (one-sided permutation p = 0.022). Two of its early flights and one other are copies of one
  design file flown on one day, at +33.6%, +49.7% and +24.5% for one simulated apogee: the builds differ from the
  file. Without that trio its gap is +14.4% (3) against +7.8% (8). Three flights can't settle it.

## The flag's edge is soft

- **Three unflagged flights fire inside the flagged range.** The same fit on six unflagged flights whose notes put a
  charge at or near the peak finds three 0.19 to 0.30 s early at 1.9 to 2.8 m/s, two at 0.13 to 0.15 s, and one at the
  peak. Their lost climb is −0.05% to +0.04%. The other 28 unflagged flights were judged from their notes, not traces.
- **Moving the three leaves the picture:** 22 early at +14.71% (OpenRocket; HPR Sim +15.79%) against 31 at +2.88%
  (+3.73%). The "without early deployments" row of the report moves with the flag's edge, by up to 0.3 point here.

## What this suggests

1. **Read the "without early deployments" row as a source split, not a cost.** The charges explain under 0.3% of 18
   flights and under about 1% of the last; HPR Sim's 4.8-point drop in that row comes from which sources flag.
   [accuracy](../accuracy.md) already says the sets differ in their rockets; this note gives the size.
2. **Report a mean that counts each source once** beside the flight mean: +3.70% (OpenRocket) and +4.40% (HPR Sim)
   against +7.79% and +8.73%. Today a source with ten flights weighs ten times one with a single flight.
3. **Set the early flag by a stated rule from each trace** (time and climb rate at the event), not from notes. With a
   rule such as "fired over 1 s or 10 m/s before the peak", only one of the 19 would stay flagged.
4. **Look at the high sources' builds, not their charges:** as-built mass against the file's, and fin finish, for the
   two sources behind 15 of the 19 (aggregates only). This answers [apogee-bias](apogee-bias.md)'s suggestion 1 and
   narrows its suggestion 3.
5. **Drop early deployment as a candidate cause** in [apogee-bias](apogee-bias.md)'s budget: the bound and the traces
   both put it under 0.3% (one flight, under about 1%).

## Open questions

- Why the one source with both kinds shows a gap of its own (p = 0.022 on 14 flights, two sibling builds among five).
- How often motor delays fire before apogee in flight, measured separation speeds of the charged section, and drogue
  opening times at 5–20 m/s; none was found published.
- No published flight gives both the simulated and the flown apogee for an ejection before apogee, nor compares a
  simulation's error with and without one (searched 2026-10-15).
- Whether a tilted flight's accelerometer apogee logic fires early more often [2] p. 67, [9] PDF p. 8, and whether
  those flights also read high from wind the simulation misses; the 19 were not split by flight angle.

## Method

The collection's part was computed with plain Python from the report run's printed per-flight rows, the hand-read
manifest and OpenRocket 24.12's per-flight record, all under the gitignored `refs/` and `corpus/`; scripts and outputs
stay there. Each early log's altitude trace (PerfectFlite `.pf2`, Featherweight `.FIPa` decoded with the collection's
own tool, Altus Metrum CSV) was fitted over the 1.0, 1.5 and 2.5 s before its event. No simulation was rerun.

## Sources (read 2026-10-15)

1. S. Niskanen, *OpenRocket technical documentation*, 2013-05-10, §3.5.
   <https://openrocket.sourceforge.net/techdoc.pdf>
2. K. Packard, B. Garbee et al., *The Altus Metrum System*, v1.9.22.2 (2025), CC BY-SA 3.0.
   <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
3. Featherweight Altimeters, *Blue Raven User's Guide*, 2026-05-12.
   <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2026may12.pdf>
4. PerfectFlite, *StratoLoggerCF manual*. <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
5. MissileWorks, *RRC3+/RRC3 User Manual*, v1.71. <https://www.missileworks.com/rrc3>
6. Entacore, *AIM USB 4 Manual*, 2021-04-01.
   <https://dept.aem.umn.edu/~./faculty/flaten/Rocketry_Remote_Lessons_Fall_2021/AIM_USB_4_Manual.pdf>
7. Silicdyne, *Fluctus documentation*, v1.7b (2025). <https://silicdyne.net/resources/docs/fluctus_fulldoc_v1_7b_EN.pdf>
8. Eggtimer Rocketry, *Quantum User's Guide*, 1.09G. <https://eggtimerrocketry.com>
9. Featherweight Altimeters, *Raven manual*, 2014-05-20.
   <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/raven_users_manual_2014may20.pdf>
10. NFPA 1125 (A2021) First Revision Report, FR-8 (§8.2.7(1)(b)), p. 7.
    <https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf>
11. NAR Standards and Testing motor data sheets (cached for [apogee-bias](apogee-bias.md)); NAR *Motor Testing
    Manual* §9.2, §10.1.3, pp. 19–20. <https://www.nar.org/wp-content/uploads/2014/08/ST-MotorTestingManual.pdf>

Terms: the Altus Metrum manual is CC BY-SA 3.0 and OpenRocket's technical documentation share-alike; both are cited,
not copied. The makers' manuals, NFPA's report and NAR's sheets are copyrighted: cached under `refs/`, cited only.
