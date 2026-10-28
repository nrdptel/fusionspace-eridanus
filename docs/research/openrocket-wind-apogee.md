# How much higher OpenRocket's apogee in wind sits than RockSim's and RASAero's

**In short: the forum tables behind OpenRocket #2489 could not be read on 2026-10-28, so no per-wind-speed apogee
difference between the three tools is sourced here; every per-speed figure below is an Inference from a toy model.**
The thread refuses plain fetches (HTTP 403), no archived copy exists, and its images could not be opened [S3].
What was read: the issue (one comment, still open), #3220's description and comments, RASAero II's manual and
OpenRocket's technical documentation [S1, S2, S5, S6]. They say OpenRocket turns into the wind less than RockSim and
RASAero, and why: a weaker restoring moment. **Inference:** less turning into the wind leaves its apogee high. On an
invented rocket, total weathercocking costs 0.17% of apogee at 5 mph, 0.70% at 10 mph and 2.82% at 20 mph; a tool
with none sits that far above one with it, and #3220's stiffer fins add at most 0.07 to 0.27 points at 10 mph. No
re-run after #3220 was found, and #2489 is still open. **How sure:** the direction of the turning gap is sourced; the
sizes are one invented rocket, about three times larger on a slower, lower-thrust flight. **It moves** the wind part of
[M2.3c6's gap by cause](../decisions-and-roadmap.md#m2-3c6) (OpenRocket's +7.79% mean error on 53 flights) and the
[M2.9 other tools' cells](../decisions-and-roadmap.md#m2-9) row for #2489: about 0.2 to 1 point if the collection's
winds run 5 to 12 mph (assumed, not read), unless the tables show more.

**What this covers.** GitHub issue and pull-request text, two manuals and a toy calculation. No OpenRocket source file
was opened. **How far to trust it.** The toy is not validated against any flight; it sizes an order of magnitude.

## 1. What was and was not read

| Source | Read on 2026-10-28 | Gives |
|---|---|---|
| Forum thread (the earlier note's posts #1, #12, #20) | No: HTTP 403, no archive, images unopened | nothing |
| Same thread, search-engine excerpts | Excerpts only, not citable | wording below, marked |
| #2489 body, timeline, its one comment | Yes | version, state, one remark on the tables |
| #3220 description, comments; #3233 | Yes | mechanism, no flight comparison |
| RASAero II manual pp. 4, 78, 80 | Yes (cached copy) | wind and weathercocking inputs |
| OpenRocket technical documentation pp. 62, 67, 93 | Yes (cached copy) | wind model, normal-force note |

- **Not reached, so unbounded.** The images' wind speeds, apogees and upwind distances; the version of RockSim and
  RASAero used; whether the comparison held the rail length and the rail-exit speed equal across the tools.
- **Excerpts (unverified).** A search engine's text of the thread says RockSim (Barrowman CP) and RASAero are alike
  in apogee reduction and upwind distance, OpenRocket's are "very small", and by 25 mph the wind pushes the OpenRocket
  rocket downwind. The earlier note [S4] carries the same 25 mph point. Treat it as a lead to check, not a number.

## 2. What the issue says

- **Version and state.** Filed 2024-05-31 against OpenRocket 23.09, on Linux; labeled bug, "Simulation" added
  2024-07-12, bug label removed 2026-08-04; open on 2026-10-28 [S1].
  Its body has no number; it points to the thread and says it should be investigated to see whether "we have an error".
- **The one comment** (2025-05-20, from a contributor) reads the thread's tabular data and finds that its two negative
  upwind values add to exactly −2,048, which it calls too neat to be chance [S1]. Units, tools and sign convention are
  not stated. **Inference:** if upwind is positive, two cells show downwind drift; no answer followed.
- **Timeline.** The only link after the comment is #3220, cross-referenced 2026-08-09 [S1].
- **Version check.** OpenRocket's latest release is still 24.12 (2025-07-27) [S8]; #3220 is on the development line
  only. Nothing says the comparison was re-run on 24.12. A forum member's 0 to 25 mph weathercock runs in 23.09 are
  mentioned in #2772 [S7]; 24.12's pre-releases capped wind at 10 m/s (22.4 mph), a cap #2771 removed on 2025-04-03,
  before the final release [S7, S8].

## 3. The mechanism the sources give

- **A weaker restoring moment.** #3220's author says OpenRocket left out the lift the fins induce on the body, "one
  likely contributor"; the fin-set normal-force slope $C_{N\alpha}$ is too low and the center of pressure (CP) too far
  forward, so "a weaker restoring moment in a crosswind" follows [S2]. A contributor's remark on 2026-08-10 says the CP
  "definitely moves back", "Probably still somewhat more conservative than RockSim" [S2].
- **OpenRocket's own words.** On the documentation's comparison rocket, its simulated $C_{N\alpha}$ is "notably lower
  than the experimental values"; the rocket "corrects its orientation slightly slower"; the altitude effect is
  "considered to be small for typical stable rockets" (OpenRocket technical documentation, p. 93 [S6]).
- **RASAero II.** With no wind it runs 2 degrees of freedom, with no dynamic stability and no weathercocking; any wind
  speed switches to 3 degrees with weathercocking (manual pp. 4, 80 [S5]). Its launch inputs are one wind speed
  (positive is a headwind), a rail length and a launch angle; no profile or gradient is listed (p. 78 [S5];
  **Inference** from the input list).
- **OpenRocket's wind.** The documented average model is a constant speed and direction at every altitude plus
  turbulence (p. 62 [S6]); the wind is added to the rocket's velocity to get airspeed and angle of attack (p. 67
  [S6]). The 24.12 release adds multi-level winds [S8]. #2732's mismatch between the two models was a units
  slip, 12 ft/s entered for 12 mph, not a model difference [S7].
- **RockSim.** A saved report lists wind speed, "Wind starts at" (a length in feet) and the launch guide's angle and
  length [S9]. Its manual was not read, so how it applies wind near the rail is unknown.
- **Rail and inertia.** Rail-exit speed $v$ sets the first angle of attack, $\arctan(w/v)$ for wind $w$. #3319
  (merged 2026-09-16) fixes pitch inertia with a mass override and says it "threw off weathercocking and drift; apogee
  was largely unaffected" [S10]. The thread's own test, from the earlier note [S4] (not re-read): moving 118 g to
  two 59 g masses 15 in fore and aft at 20 mph took one tool's apogee from 6,760 ft to 6,709 ft (−0.75%).

## 4. The size, by a toy model (Inference)

**The model.** Two invented 2-D scripts in steady wind, uniform with height, over an exponential atmosphere (8.5 km
scale height). Rocket: 10 kg falling to 7.5 kg, 1,200 N for 3 s, 0.1 m diameter, axial drag coefficient 0.55, 3 m
rail, exit speed 25.9 m/s. The pitch script adds $C_{N\alpha}$ = 12 per radian for the whole rocket, the CP 2.5
calibers behind the CG, a 2 m uniform rod's pitch inertia and a damping term; calm apogee 3,136 m (10,289 ft). The
limit script has no pitch dynamics (calm 3,139 m). On the rail the crosswind enters only the drag, as airspeed; it
adds no side force there. Unvalidated, not the thread's rocket. Scripts: `refs/research/openrocket-wind-apogee/`.

Apogee change against the calm flight, percent:

| Wind (mph) | No weathercock | Base | #3220 low | #3220 high | Instant alignment |
|---:|---:|---:|---:|---:|---:|
| 5 | −0.00 | −0.17 | −0.19 | −0.24 | −0.9 |
| 10 | −0.01 | −0.70 | −0.77 | −0.97 | −3.6 |
| 15 | −0.01 | −1.58 | −1.74 | −2.18 | −7.8 |
| 20 | −0.02 | −2.82 | −3.09 | −3.86 | −12.9 |
| 25 | −0.04 | −4.38 | −4.80 | −5.96 | −18.6 |

- **Columns.** *No weathercock* (limit script): the body axis stays vertical. *Base* (pitch script): the model above.
  *#3220 low* and *high*: the whole rocket's $C_{N\alpha}$ times 1.2 with the CP 0.12 caliber aft, and times 1.5
  with it 1.2 calibers aft. #3220 scales only the fin set by $1 + \tau$, and the CP moves come from the earlier note's
  own invented rocket [S4, S2], so *high* is an upper side, not the span. *Instant alignment* (limit script): the body
  follows the relative wind at once, an upper limit no real rocket reaches.
- **Sign and size of the tools' difference.** If OpenRocket behaved as *No weathercock* and RockSim as *Base*,
  OpenRocket minus RockSim would be +0.17, +0.69, +1.57, +2.79 and +4.34% (points of the calm apogee) at 5, 10,
  15, 20 and 25 mph.
  Against *#3220 high* it would be +0.24, +0.96, +2.17, +3.83 and +5.92%. These are limits on this rocket, since
  OpenRocket does weathercock a little. If the excerpts' "very small" loss holds, its row sits nearer *No
  weathercock* and the true figures sit below these.
- **What #3220 changes.** From *Base* to *#3220 low* to *high*, the apogee falls a further 0.02 to 0.07 points at
  5 mph, 0.07 to 0.27 at 10 mph and 0.27 to 1.04 at 20 mph. Raising $C_{N\alpha}$ alone by 30% and 50% adds 0.08
  and 0.12 points at 10 mph; at the high end the CP move is the slightly larger part. The fin-body note's model of
  the fin-set change alone gives less: 0.24% at 8 m/s (18 mph) and 0.52% at 12 m/s (27 mph) [S11].
- **The flight matters more than the rail.** At 1,200 N a 1.1 m rail (15.6 m/s exit) loses 0.82% at 10 mph and
  3.34% at 20 mph, about 1.2 times the 3 m case; a 1.5 m rail (18.3 m/s) 0.79% and 3.20%; a 6 m rail (36.7 m/s)
  0.57% and 2.29%. At 500 N the rocket flies to 946 m and loses 2.07% and 8.27% (15.7 m/s exit), still 1.18% and
  4.73% from an 11.5 m rail at 31.2 m/s. So most of the threefold loss of a slow flight comes from its lower thrust
  and longer slow climb, not its rail exit.
- **Stability margin.** Base loss at 20 mph for margins of 1.0, 1.5, 2.0, 2.5 and 3.0 calibers is 1.80, 2.21, 2.54,
  2.82 and 3.04%; at 10 mph 0.44, 0.55, 0.63, 0.70 and 0.76%.
- **Drift.** Any positive restoring moment in the pitch script turns the rocket upwind (at 25 mph, $C_{N\alpha}$
  times 0.01 still drifts 121 m upwind). The limit script's *No weathercock* rocket drifts downwind, 45 m at 10 mph
  to 113 m at 25 mph. A tool that barely turns into the wind would match the excerpts' downwind drift at 25 mph.

## 5. Does #3220, or anything later, close it?

- **Reported.** #3220 (merged 2026-08-10, reverted by #3232, restored by #3233 on 2026-08-11) says it is "Related to
  #2489, but I don't want to close it yet", and its author says they are not knowledgeable enough to make a reliable
  weathercocking comparison with RASAero or RockSim [S2]. Its plots show OpenRocket's $C_{N\alpha}$ closer to
  RASAero's but still an underestimate, though an edit says the sustainer's curves should sit 2 higher for the nose,
  which narrows that remaining gap [S2]. #3233 reports one test's apogee time falling from 3.62 s to 3.35 s [S2].
- **Later.** #3263 (merged 2026-09-09) splits fin and body loads; its text has no weathercocking or wind
  comparison [S2]. A search of the repository on 2026-10-28 found the word "weathercock" in 5 issues (#2489, #2772,
  #653, #3183, #2732) and 4 pull requests (#3211, #3319, #3220, #3295); none reports a three-tool re-run.
- **Verdict (Inference).** #3220 narrows the cause by an unmeasured amount; nobody reports the gap closed. In the toy
  its effect is at most 0.07 to 0.27 points at 10 mph. Downwind drift at 25 mph, if real, means OpenRocket 23.09
  turned into the wind almost not at all, more than a 20% to 50% weaker fin slope explains. The fin-body note
  reads RASAero II's match to HPR Sim as a sign that RASAero also uses Barrowman's 1 + τ (its own Inference) [S11];
  if so, #3220's change is not what separates OpenRocket from RASAero.

## 6. Relation to the collection's +7.79%

- **Sign (Inference).** OpenRocket's apogee in wind is high; the collection's OpenRocket mean error is +7.79% on 53
  flights, flown in the same wind as HPR Sim's, every 50 m [S4]. Same sign, so the wind part adds to it; it is not
  the whole.
- **Size, bounded (Inference).** On a rocket like the toy's, a tool with no weathercocking sits at most 0.7% above
  the base case at 10 mph and 2.8% at 20 mph; at 5 mph, 0.2%. For the collection to owe 1 point to this, its typical
  wind would need to be near 12 mph, or near 7 mph on slow, low-thrust flights (about 3 times the loss). The
  collection's winds are not read here, so this is an assumption: at 5 to 12 mph the wind part would be 0.2 to 1
  point of the 7.79%.
- **Both tools (Inference).** HPR Sim's +8.73% and OpenRocket's +7.79% are flown in the same wind profile [S4], so
  the wind adds to HPR Sim less OpenRocket mainly through the difference in how strongly each turns into it.

## What this suggests

- **Read the tables first.** Open the thread in a signed-in browser, save the three tables' images under
  `refs/research/openrocket-wind-apogee/`, and transcribe tool, version, wind, apogee and upwind distance. Expected
  effect: replaces §4's limits with the thread's own sizes; the −2,048 remark can be checked at once.
- **M2.3c6.** Add a wind-off run of the 53 flights in OpenRocket 24.12 and HPR Sim: the difference between the wind-on
  and wind-off gaps is the wind part, measured and not assumed. Expected: 0.2 to 1 point.
- **M2.9's #2489 row.** Cite it as "direction known, size unmeasured; toy limit 0.7% at 10 mph" until the tables are
  read.
- **Terms.** Issue and pull-request text and the two manuals are cited, not copied (OpenRocket is GPL; the
  RASAero manual is a vendor download). The forum posts are the authors' own; cite, do not cache.

## Open questions

- The thread's numbers: wind speeds, apogees and drifts per tool, tool versions, and whether the rail length matched.
- Whether RockSim and RASAero apply wind from the first instant on the rail, and whether OpenRocket does.
- Whether a post-#3220 build changes the thread's table; the snap edge build would show it.
- Whether 23.09's downwind drift at 25 mph means it turned into the wind almost not at all, as the toy suggests.

## Sources

All read 2026-10-28 unless noted.

1. **S1.** OpenRocket #2489, its comment and timeline (GitHub API).
   <https://github.com/openrocket/openrocket/issues/2489>
2. **S2.** #3220 description and comments, #3233, #3263. <https://github.com/openrocket/openrocket/pull/3220>
3. **S3.** Rocketry Forum, "Weathercock simulations all over the place" (2024-05-30 on, per [S4]); NOT READ (HTTP 403).
   <https://www.rocketryforum.com/threads/weathercock-simulations-all-over-the-place.186584/>
4. **S4.** [Apogee defects of OpenRocket](openrocket-apogee-defects.md) and
   [what changed after 24.12](openrocket-after-24-12.md); the
   [real-flights report](../../validation/reports/fixture-flights.md).
5. **S5.** C. E. Rogers and D. Cooper, *RASAero II Users Manual*, pp. 4, 78, 80 (cached copy under `refs/papers/`).
   <https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>
6. **S6.** S. Niskanen, *OpenRocket Technical Documentation*, version 13.05, pp. 62, 67, 93 (cached copy).
   <https://openrocket.sourceforge.net/techdoc.pdf>
7. **S7.** OpenRocket #2772 and #2732 with comments. <https://github.com/openrocket/openrocket/issues/2772>
8. **S8.** OpenRocket 24.12 release notes and release list (GitHub API).
   <https://github.com/openrocket/openrocket/releases/tag/release-24.12>
9. **S9.** A saved RockSim simulation report (a file sample). <https://www.qsl.net/ka0ars/rocksim.html>
10. **S10.** OpenRocket #3319. <https://github.com/openrocket/openrocket/pull/3319>
11. **S11.** [Should HPR Sim add the body lift its fins carry over?](fin-body-interference.md), its apogee section.
