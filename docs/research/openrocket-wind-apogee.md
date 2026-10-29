# How much less apogee OpenRocket loses in wind than RockSim and RASAero

Re-checked 2026-10-28: 9 claims corrected.

**In short: on the forum's one test rocket, OpenRocket loses 0.28% of its calm apogee in a 10 mph crosswind, while
RockSim and RASAero lose 0.43% to 0.84%, so OpenRocket sits 0.15 to 0.57 points high; at 20 mph, 1.0 to 1.8 points
[S3].** The thread behind OpenRocket #2489 was read in a signed-in browser and its table transcribed: a 1.6 m, 67 mm,
3.6 kg K-motor rocket, 27 m/s off the rail, about 9,800 ft in a steady wind. The other setups turn farther into the
wind, 1.3 to 1.9 times OpenRocket's 530 ft upwind at 10 mph and 3 to 4 times at 20 mph; above 20 mph OpenRocket's
upwind distance collapses (−1,930 ft at 30 mph). A slow-burn flight in the same thread puts OpenRocket 2.4% above a
RockSim run fitted to GPS at 5.5 mph, a whole-flight difference, not a wind share. **How sure:** one rocket,
OpenRocket 23.09, other versions not given, the other tools assumed right. The earlier toy model (§4) overstated the
largest gap 1.2 to 1.5 times at 10 to 25 mph. #3220 (fin-induced body lift) and the open #3410 (pitch damping about
the nose tip) are named causes, neither re-run.
**It moves** the wind part of [M2.3c6's gap by cause](../decisions-and-roadmap.md#m2-3c6) (OpenRocket's +7.79% mean
error on 53 flights) and the [M2.9 other tools' cells](../decisions-and-roadmap.md#m2-9) row for #2489: about 0.1 to
0.8 points if the collection's winds run 5 to 12 mph (assumed, not read), more on slow flights.

**What this covers.** The forum thread's 23 posts and 7 images, issue and pull-request text, the RASAero II manual,
OpenRocket's technical documentation and a toy model; no OpenRocket source was opened. **How far to trust it.** The
table measures the tools against each other, not against a flight; the toy is not validated against any flight.

## 1. What was read

- **The setup** (post #1, 2024-05-30). One generic HPR rocket in four setups "identical as much as possible", a
  0.01 s step, close in calm apogee and flight time. Static margins: OpenRocket (extended Barrowman) 1.52 calibers,
  RockSim with its own CP 2.72, RockSim with Barrowman 1.51, RASAero II (Rogers modified Barrowman) 2.34. RockSim and
  RASAero are 3 degrees of freedom; the wind is a constant horizontal vector. The post calls OpenRocket's loss and
  upwind distance "both very small"; by 25 mph the wind pushes it downwind. Held equal (posts #7, #19, #22): the
  guidance length, mass and CG, a rail exit of about 90 ft/s (27 m/s). Not given: RockSim's and RASAero's versions;
  the issue names OpenRocket 23.09 [S1].

## 2. The table (post #1)

Apogee loss is against each tool's own calm apogee (calm: OpenRocket 9,802 ft, RockSim 9,728 and 9,729 ft, RASAero
10,005 ft). Upwind is the horizontal distance at apogee, positive into the wind. The printed ratios agree with the
apogees to 0.0005; two separate readings of the image agree in every cell.

| Wind (mph) | OpenRocket loss % (upwind ft) | RockSim, own CP | RockSim, Barrowman | RASAero II |
|---:|---:|---:|---:|---:|
| 5 | 0.08 (327) | 0.43 (544) | 0.38 (416) | 0.16 (375) |
| 10 | 0.28 (530) | 0.84 (1,010) | 0.64 (756) | 0.43 (698) |
| 15 | 0.43 (585) | 1.48 (1,423) | 1.02 (1,048) | 0.87 (1,016) |
| 20 | 0.44 (421) | 2.28 (1,786) | 1.50 (1,296) | 1.46 (1,324) |
| 25 | 0.26 (−118) | 3.21 (2,104) | 2.05 (1,506) | 2.16 (1,587) |
| 30 | 0.89 (−1,930) | 4.21 (2,378) | 2.62 (1,677) | 2.92 (1,801) |

- **OpenRocket's gap,** the other tool's loss less OpenRocket's, in points of calm apogee (positive: OpenRocket high):

| Wind (mph) | vs RockSim, own CP | vs RockSim, Barrowman | vs RASAero II |
|---:|---:|---:|---:|
| 5 | +0.35 | +0.30 | +0.08 |
| 10 | +0.57 | +0.36 | +0.15 |
| 15 | +1.05 | +0.59 | +0.44 |
| 20 | +1.84 | +1.06 | +1.02 |
| 25 | +2.95 | +1.79 | +1.90 |
| 30 | +3.33 | +1.73 | +2.03 |

- **The −2,048 remark.** OpenRocket's two negative upwind cells, −118 ft at 25 mph and −1,930 ft at 30 mph, sum to
  exactly −2,048, the figure #2489's one comment calls too neat to be chance [S1]. Nothing answered it.
- **Absolute apogee.** The gap above is relative. In feet, OpenRocket is above RockSim at every wind but below
  RASAero up to 25 mph, since RASAero's calm apogee is 2.1% higher.
- **Same margin, different turn (Inference).** OpenRocket and RockSim-Barrowman have the same static margin (1.52
  and 1.51 calibers), yet RockSim loses 2.3 times as much at 10 mph. So the gap is not the CP position alone: the
  normal-force slope, the pitch damping or the rail exit differs.
- **A real flight** (post #23, 2024-11-04). A stretched kit on a 15 s H motor, vertical rail, GPS and barometer:
  about 5,500 ft and 3,000 ft upwind, by the author. RockSim was fitted to the GPS apogee and range with a constant
  drag coefficient of 0.64 and a 5.5 mph wind; OpenRocket, given the same drag coefficient and wind, peaked about
  5,600 ft against RockSim's 5,470 ft (+2.4%) and about 2,500 ft upwind against 2,900 ft (chart read by pixels). The
  fit is RockSim's, so this is the tools' whole difference on a slow flight, calm-air differences included (+0.75%
  in calm air on the table's rocket), not a wind share and not OpenRocket against the flight alone.

## 3. The mechanism the sources give

- **A weaker restoring moment.** #3220's author says OpenRocket left out the lift the fins induce on the body, "one
  likely contributor"; the fin-set normal-force slope $C_{N\alpha}$ is too low and the center of pressure (CP) too far
  forward, so "a weaker restoring moment in a crosswind" follows [S2]. A contributor's remark on 2026-08-10 says the CP
  "definitely moves back", "Probably still somewhat more conservative than RockSim" [S2].
- **Pitch damping.** #3410 (open, 2026-10-25) says OpenRocket computed the pitch damping moment about the nose tip,
  not the CG, with a factor of 3 tuned to that arm, and applied it in the wrong plane for half of all roll angles;
  its author puts the example rockets' apogees at most 0.12% apart after the fix [S12]. **Inference:** damping
  that is too strong turns the rocket into the wind more slowly, the table's sign, but the wrong-plane half pushed the
  rotation along; the net sign and size in wind are unknown.
- **High angle at the rail.** At 27 m/s exit the first angle of attack, $\arctan(w/v)$, is 9° at 10 mph, 18° at
  20 mph and 22° at 25 mph. #3183 names a 17.5° stall angle in OpenRocket [S7]. **Inference:** the collapse above
  20 mph tracks this angle; not shown.
- **OpenRocket's own words.** On its comparison rocket, simulated $C_{N\alpha}$ is "notably lower than the
  experimental values" and the rocket "corrects its orientation slightly slower" (technical documentation p. 93 [S6]).
- **RASAero II.** With no wind it runs 2 degrees of freedom, with no dynamic stability and no weathercocking; any wind
  speed switches to 3 degrees with weathercocking (manual pp. 4, 80 [S5]). Its launch inputs are one wind speed
  (positive is a headwind), a rail length and a launch angle (p. 78 [S5]). Its Rogers modified Barrowman option
  adds the body-in-presence-of-fins lift Barrowman leaves out (p. 4 [S5]; its author says the same in post #15 [S3]).
- **OpenRocket's wind.** A constant speed and direction at every altitude plus turbulence (pp. 62, 67 [S6]); 24.12
  adds multi-level winds [S8]. #2732's model mismatch was a units slip, 12 ft/s for 12 mph [S7].
- **Inertia.** #3319 (merged 2026-09-16) fixes pitch inertia under a mass override, which "threw off weathercocking
  and drift" [S10]. Post #20: in OpenRocket at 20 mph, splitting a 118 g mass into two 59 g masses 15 in fore and
  aft took another rocket from 6,760 ft to 6,709 ft (−0.75%) and from 819 ft to 774 ft upwind [S3].

## 4. The toy model, against the table (Inference)

**The model.** Two invented 2-D scripts in steady wind over an exponential atmosphere: 10 kg falling to 7.5 kg,
1,200 N for 3 s, 0.1 m diameter, axial drag coefficient 0.55, 3 m rail, 25.9 m/s exit, calm apogee 3,136 m. The
pitch script adds $C_{N\alpha}$ = 12 per radian, a 2.5-caliber margin, a 2 m rod's pitch inertia and a damping term;
the limit script has no pitch dynamics. Scripts: `refs/research/openrocket-wind-apogee/`.

| Wind (mph) | Toy: no weathercock less base, points | Table: OpenRocket gap, points |
|---:|---:|---:|
| 5 | +0.17 | +0.08 to +0.35 |
| 10 | +0.69 | +0.15 to +0.57 |
| 20 | +2.79 | +1.02 to +1.84 |
| 25 | +4.34 | +1.79 to +2.95 |

- **Where it fails.** The toy's "no weathercock" limit is 1.2 to 1.5 times the largest measured gap (RockSim, own
  CP) at 10 to 25 mph and 1.9 to 4.6 times the others, because
  OpenRocket does turn into the wind: 530 ft at 10 mph, 70% of RockSim-Barrowman's distance. The toy's
  no-weathercock rocket drifts downwind (148 ft at 10 mph); the table's −1,930 ft at 30 mph is four times its limit,
  so the collapse is not simply "no turning". At 5 mph the toy's 0.17 is below the measured 0.30 to 0.35 against
  RockSim, so it was no upper limit there.
- **What still holds.** Margin: base loss at 20 mph for 1.5 and 2.5 calibers is 2.21% and 2.82% (+0.61), against
  +0.78 between RockSim's 1.51 and 2.72 caliber setups. Thrust: at 500 N the toy loses about three times what it
  loses at 1,200 N (2.07% against 0.70% at 10 mph), mostly from the slow climb, not the rail exit.
- **#3220 in the toy.** Raising the whole rocket's $C_{N\alpha}$ 1.2 to 1.5 times, with the CP 0.12 to 1.2 calibers
  aft, adds 0.07 to 0.27 points of loss at 10 mph and 0.27 to 1.04 at 20 mph. Against the measured +0.36 (10 mph)
  and +1.06 (20 mph) gaps to RockSim-Barrowman, that could close a large part, but it is not measured. The fin-body
  note's model of the fin-set change alone gives less: 0.24% at 8 m/s (18 mph), 0.52% at 12 m/s (27 mph) [S11].

## 5. Does #3220, or anything later, close it?

- **Reported.** #3220 (merged 2026-08-10, reverted by #3232, restored by #3233 on 2026-08-11) says it is "Related to
  #2489, but I don't want to close it yet"; its author makes no weathercocking comparison with RASAero or RockSim
  [S2]. Its plots put OpenRocket's $C_{N\alpha}$ closer to RASAero's but still under it (an edit adds 2 for the
  nose, narrowing that) [S2]. #3233 reports one test's apogee time falling from 3.62 s to 3.35 s [S2].
- **Later.** #3263 (merged 2026-09-09) splits fin and body loads, with no wind comparison [S2]. #3410 (open) fixes
  the damping arm, cross-referenced from #2489 [S12, S1]. "Weathercock" appears in 5 issues (#2489, #2772, #653,
  #3183, #2732) and 4 pull requests (#3211, #3319, #3220, #3295); none reports a three-tool re-run.
- **Version check.** OpenRocket's latest release is still 24.12 (2025-07-27) [S8]; #3220 and #3263 are on the
  development line only, and #3410 is unmerged. 24.12's pre-releases capped wind at 10 m/s (22.4 mph), a cap #2771
  removed on 2025-04-03, before the final release [S7, S8].
- **Verdict (Inference).** Nobody reports the gap closed. The thread's RASAero used the Rogers method, which carries
  the body lift #3220 adds, so #3220 moves OpenRocket toward that setup; it does not explain the gap to
  RockSim-Barrowman, which lacks that lift (assumed: Barrowman's method omits it, post #15) and still turns
  farther. Damping (#3410) is the other named candidate.
  The fin-body note infers RASAero uses Barrowman's 1 + τ [S11]; that can hold only with the Rogers option off.

## 6. Relation to the collection's +7.79%

- **Sign.** OpenRocket's relative apogee loss in wind is too small against all three other setups, so in wind its
  apogee errs high; the collection's OpenRocket mean error is +7.79% on 53 flights, flown in the same wind as HPR
  Sim's, every 50 m [S4]. Same sign: the wind part adds to it and is not the whole.
- **Size (Inference).** On the thread's rocket the gap reaches 1 point at about 14.5 mph against RockSim with its own
  CP and at 19 to 20 mph against RockSim-Barrowman and RASAero. If the collection's winds run 5 to 12 mph (an
  assumption: the real-flights report holds no wind summary), the wind part is about 0.1 to 0.8 points; at 15 to
  20 mph, 0.4 to 1.8. Slow, low-thrust flights likely carry more (the toy's threefold thrust effect).
- **HPR Sim against OpenRocket (Inference).** HPR Sim's apogee is 0.81% above OpenRocket's on average [S4]. If HPR Sim
  turns into the wind as RockSim does, wind would put it below OpenRocket, so the wind does not explain that order.

## What this suggests

- **M2.3c6.** Add a wind-off run of the 53 flights in OpenRocket 24.12 and HPR Sim: the difference between the
  wind-on and wind-off gaps is the wind part, measured. Expected: 0.1 to 0.8 points, more if slow flights dominate.
- **M2.9's #2489 row.** Cite it as "OpenRocket 23.09 loses 0.15 to 0.57 points less apogee at 10 mph, 1.0 to 1.8 at
  20 mph, than RockSim and RASAero on one rocket [S3]".
- **A three-tool case for HPR Sim.** Rebuild the thread's published rocket (its sizes and motor are in post #1) and fly
  it in HPR Sim at 0 to 30 mph beside the table: a code-to-code check of HPR Sim's weathercocking, with the other
  tools' versions unknown. Re-fly it in an OpenRocket build with #3220 and #3410 to size each.
- **Terms.** Forum numbers are transcribed and cited, images kept privately; manuals and issue text cited, not copied.

## Open questions

- Why OpenRocket's upwind distance collapses above 20 mph: the rail-exit angle against its stall angle, or another
  limit. Its −1,930 ft at 30 mph is four times the toy's no-turning drift.
- How much of the gap #3220 and #3410 each close; neither has been re-run on the table's rocket.
- Which RockSim and RASAero versions the thread used, and when each applies wind on the rail (RockSim saves a "Wind
  starts at" height [S9]).
- The collection's actual winds aloft, which set how much of the +7.79% the wind carries.

## Sources

All read 2026-10-28 unless noted.

1. **S1.** OpenRocket #2489, its comment and timeline (GitHub API).
   <https://github.com/openrocket/openrocket/issues/2489>
2. **S2.** #3220 description and comments, #3232, #3233, #3263. <https://github.com/openrocket/openrocket/pull/3220>
3. **S3.** Rocketry Forum, "Weathercock simulations all over the place", posts #1 (table, charts, rocket), #7, #15,
   #19, #20, #22, #23 (2024-05-30 to 2024-11-04), read in a signed-in browser; images cached privately.
   <https://www.rocketryforum.com/threads/weathercock-simulations-all-over-the-place.186584/>
4. **S4.** [Apogee defects of OpenRocket](openrocket-apogee-defects.md) and
   [what changed after 24.12](openrocket-after-24-12.md); the
   [real-flights report](../../validation/reports/fixture-flights.md).
5. **S5.** C. E. Rogers and D. Cooper, *RASAero II Users Manual*, pp. 4, 78, 80 (cached copy under `refs/papers/`).
   <https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>
6. **S6.** S. Niskanen, *OpenRocket Technical Documentation*, version 13.05, pp. 62, 67, 93 (cached copy).
   <https://openrocket.sourceforge.net/techdoc.pdf>
7. **S7.** OpenRocket #2772, #2732, #3183 with comments; #2771. <https://github.com/openrocket/openrocket/issues/2772>
8. **S8.** OpenRocket 24.12 release notes and release list (GitHub API).
   <https://github.com/openrocket/openrocket/releases/tag/release-24.12>
9. **S9.** A saved RockSim simulation report (a file sample). <https://www.qsl.net/ka0ars/rocksim.html>
10. **S10.** OpenRocket #3319. <https://github.com/openrocket/openrocket/pull/3319>
11. **S11.** [Should HPR Sim add the body lift its fins carry over?](fin-body-interference.md), its apogee section.
12. **S12.** OpenRocket #3410 (open, 2026-10-25). <https://github.com/openrocket/openrocket/pull/3410>
