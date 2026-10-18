# Open OpenRocket defects that move an apogee by more than 0.5%

**In short: on 2026-10-17 no open OpenRocket issue says, in its own text, that a defect in the code moves apogee by
more than 0.5%.** The defects that bear on HPR Sim's comparisons are signed but unsized. #3391 (opened 2026-10-14)
gives a tangent-ogive nose about 0.0007 of pressure drag at Mach 1.5 in 24.12, where OpenRocket's own documented
method gives about 0.13, so its supersonic drag is low and its apogee high [S2]. Open pull request #3318 says stubby
stored-table noses (fineness below 1.8) carry almost no subsonic pressure drag [S3]. #2489 says OpenRocket turns into
the wind far less than RockSim and RASAero [S4, S5]. Three reports carry a size above 0.5%, but none is an open code
defect and none moves a comparison as it runs: 24.12's motor list files a 316 N·s curve under the 229 N·s
229H255-14A (+38.1% impulse; closed #2348) [S6, S7]; a stage mass override without "override all subcomponents"
turned one forum user's 4,303 ft into 1,438 ft (−66.6%; #2393, #3205), which is how the input works [S8, S9]; and
#1400 quotes OpenRocket's documentation that laminar flow moved apogee "less than 5%", a choice HPR Sim shares
[S10, S11]. How sure: every state, date and number is read from GitHub, the Rocketry Forum and the cited reports on
2026-10-17; every apogee size this note derives is marked **Inference**. It feeds
[M2.9, the other tools' cells](../decisions-and-roadmap.md#m2-9), the causes in
[M2.3c6, the gap to OpenRocket by cause](../decisions-and-roadmap.md#m2-3c6), and how far an "HPR Sim less
OpenRocket" figure can be read as HPR Sim's own error.

**What this covers.** Issue, comment, pull-request and forum text only; no diff, changed file or source file was
opened (OpenRocket is GPL-3.0). It builds on [what changed after 24.12](openrocket-after-24-12.md), the merged
changes; this note covers what is still open. **How far to trust it.** An issue says what its reporter saw; a
defect's size on HPR Sim's comparisons is known only by flying OpenRocket with and without it.

## 1. What was screened

- **Open items.** The GitHub API listed 285 open items: 265 issues and 20 pull requests [S1]. By issue type: 185
  Feature, 27 Bug, 14 Task, 39 untyped; no issue carries a `bug` label, so every title was read.
- **Searches.** 29 aerodynamic terms (drag, apogee, stability, center of pressure, normal force, supersonic, Mach,
  fin, nose cone, boattail, transition, base drag, friction, roughness, pod, ring fin, margin and others) matched 122
  issues; 30 terms on mass, motors, simulation, wind, recovery and staging matched 167. "Time step", "integrator",
  "turbulence", "atmosphere" and "center of gravity" matched none.
- **Read whole.** 44 open threads in the aerodynamic pass and 40 in the other, with comments and timelines; the
  descriptions and reviews of open pull requests #3318, #3173 and #2900; closed issues #2291, #2348, #2639, #2792,
  #3041 and #3342. Of the issues closed as not planned, only #3041 states a present behavior with a size (§5).
- **Leads.** #3391 is §2. #2348 is §3. #2507 (effective rail length) is §4. #2442 asks for a vent-hole part that
  "does not need to affect the drag calculations": no defect.

## 2. The defects that touch a comparison (open, signed, unsized)

### #3391: tangent-ogive pressure drag above Mach 1

- **What it says.** Above Mach 1, 24.12 gives about 0.0007 (the development line exactly 0); the technical
  documentation's ogive method gives about 0.13 at Mach 1.5. The reporter traces it to the subsonic zero of #2999
  also feeding the supersonic curve, and reports a false hump of about 0.075 near Mach 1.1 [S2].
- **State.** Open, unlabeled, no comment, no linked pull request. Versions named: 24.12 and the development line.
- **Sign.** Drag low above Mach 1, so OpenRocket's apogee and top speed high. The hump, if 24.12 has it (the issue
  doesn't say), pulls the other way near Mach 1.1.
- **Size.** The issue sizes a drag coefficient, not an apogee. Measured references put a fineness-3 tangent ogive at
  0.110 to 0.119 near Mach 1.5 ([ogive wave drag](ogive-wave-drag.md)). The documentation's cone formula, which it
  states from about Mach 1.3, gives 0.130 at fineness 3 and 0.065 at fineness 5 [S11, eq. B.4].
- **54 example configurations.** One flight passes Mach 1: *Dual parachute deployment* on a J570W-P, peak Mach
  1.147, HPR Sim's apogee 4.34% below OpenRocket's [S12]. Its nose is a tangent ogive of fineness 5.0, read from
  24.12's example file [S13]. It peaks under power; OpenRocket's power-off drag curve ends at Mach 1.052, total
  0.97 there [S12]. **Inference:** the flight never reaches Mach 1.3, so mostly the hump and the transonic blend
  bear on it; restoring the wave drag would lower OpenRocket's apogee and narrow the −4.34%, removing the hump would
  widen it. Neither is sized; the report gives no time above Mach 1.
- **#222.** On the private collection's supersonic case `C06/1`, HPR Sim less OpenRocket is +13.60%; +1.11% on
  OpenRocket's drag; −10.71% with OpenRocket's base rule alone [S14, S21]. **Inference:** about 11.8 points sit in
  drag other than the base (nose, fins, and the pressure drag OpenRocket gives no part). #3391 is the nose's share;
  no public number splits it from the fins'.
- **The collection.** Its report has no Mach split; ADR-273 already asks M2.3c6 to count supersonic flights.

### #3318 (open pull request): stubby noses get almost no subsonic pressure drag

- **What it says.** Ellipsoid, power, parabolic and Haack noses take their drag from fineness-3 tables, scaled.
  Where the table is zero, a stubby nose keeps about 0 subsonic pressure drag, and an ellipsoid's fit is
  "unrealistically small" at low Mach [S3]. The fix adds a floor below fineness 1.8,
  (1/3) × 0.8/(1 + 4f²) × (1 − (f/1.8)²), tested at 0.123 for a Haack nose at f = 0.5. Conical and ogive noses
  are left out.
- **State.** Open against the development line, labeled "next release" on 2026-09-14; two commenters asked on
  09-12 and 09-14 how the floor will be validated, unanswered on 2026-10-17 [S3]. No version named.
- **Sign.** Drag low on such noses, so OpenRocket's apogee high; the fix lowers it. **Inference:** 24.12 has it.
- **Size.** The text sizes the floor, not an apogee. **Inference** from its formula: 0.102 at f = 0.58.
- **54 example configurations.** *Base drag hack (short-wide)* has an ellipsoid of fineness 0.58 across the whole
  body, 3 configurations, HPR Sim +1.95%, +12.96% and +10.79% from OpenRocket (the last two with named causes)
  [S12, S13]; OpenRocket's power-off drag coefficient at each flight's top speed is 0.218, 0.241 and 0.260 [S12].
  The fineness-0.92 ellipsoid on *Pods--airframes and winglets* is a flipped tail, which #3318 leaves out.
  **Inference, rough:** if 24.12's nose term on the short-wide design is near 0, the floor adds up to 0.10, 38% to
  49% of its drag; two coast-only estimates on assumed burnout masses put OpenRocket's apogee falls at 5% to 10% on
  each flight. HPR Sim's +1.95% would widen unless its own nose term is as small. #3331 (merged 2026-09-21) deletes
  a base-drag-hack example from the development line; the comparison flies 24.12's copy.

### #2489: OpenRocket weathercocks less than RockSim and RASAero

- **What it says.** Filed by a maintainer on 2024-05-31 from a forum comparison in 23.09: in a steady crosswind,
  OpenRocket lost little apogee and drifted little upwind while RockSim and RASAero agreed with each other; by 25 mph
  OpenRocket's rocket went downwind [S4, S5]. The forum's tables are images, so no number is in text.
- **State.** Open. Merged #3220 (fin–body interference) links to it, and its author did not want to close it yet
  [S4]; the fin normal-force slope it adds is 20% to 50% (**Inference**, in
  [what changed after 24.12](openrocket-after-24-12.md)). The documentation calls OpenRocket's normal-force slope
  "notably lower than the experimental values", with an effect on altitude "considered to be small for typical
  stable rockets" [S11, p. 93].
- **Sign.** Too little apogee lost into the wind, so OpenRocket's apogee in wind is high. **Size:** none in text.
  The thread does show how far inertia reaches: at 20 mph, moving 118 g from the CG to two 59 g masses 15 in fore
  and aft took the apogee from 6,760 ft to 6,709 ft (−0.75%) [S5, post #20].
- **Comparisons.** The 54 configurations and #222's flight are calm, so untouched. The collection is flown in each
  day's wind, every 50 m [S15]; OpenRocket's mean error there, +7.79% on 53 flights, has this defect's sign
  (**Inference**, unsized).

## 3. Sized above 0.5%, but not an open code defect

- **#2348, a duplicate H255 curve** (closed 2023-09-23; still in 24.12's motor list). A comment of 2026-10-11 says
  the second 229H255-14A curve is a byte copy of the 315H255-14A file, that ThrustCurve pulled it that day, and that
  24.12's bundled database "still has it" [S6]. ThrustCurve now lists one file, 229.3 N·s over 0.9 s [S7]. The
  project's record of 24.12's database holds two 229H255-14A entries: 228.958 N·s over 0.936 s, and 316.142 N·s
  over 1.252 s carrying the 315H255-14A's digest [S16]. **Size:** +38.1% impulse and +86 g at launch on the wrong
  pick; **Inference:** apogee up by tens of percent. No code fix; the development line's startup database update
  (#2991, merged 2026-03-23) should drop it (**Inference**). **Touches:** none of the 85 designs in the project's
  24.12 motor survey flies either digest [S16]; the collection's flight records name no H255, though 17 rows of its
  design catalog list the 229H255-14A among a file's configurations. Each comparison flies the curve OpenRocket's
  own configuration resolves, so a wrong pick would be shared, not a gap.
- **#2393 and #3205, a mass override without its subcomponents** (open since 2023-11-09 and 2026-08-04). With the
  box unchecked the rocket comes out far too heavy, with no warning [S8, S9]. The forum thread behind #2393 has a
  K530 flight predicted at 1,438 ft against 4,303 ft from an older OpenRocket and 5,200 ft flown (−66.6% against the
  older prediction) [S17]. No fix; #3205 asks for a warning. **Touches:** 8 of 24.12's 17 example files carry a mass
  override and one a subcomponents override [S13]; HPR Sim reads overrides as OpenRocket does (launch mass within
  0.024% mean absolute [S12]), so HPR Sim less OpenRocket is unmoved.
- **#1400, friction taken as fully turbulent** (open since 2022). It quotes the documentation: a laminar layer
  changed apogee "less than 5%" in the designs tested [S10, S11, p. 43]. A bound, not a size; it may exceed 0.5%.
  **Sign:** friction high, apogee low where part of the real layer is laminar. HPR Sim is fully turbulent too
  ([aerodynamics](../physics/aero.md)), so only the cells against logged flights see it.

## 4. Open and unsized, outside the comparisons

- **#2031, automatic streamer drag coefficient** (2023): 0.06 where the reporter, a developer, expects 0.15 to 0.4
  [S18]. **Inference:** descent 1.58 to 2.58 times too fast; apogee moves only if a streamer opens before apogee.
  No example file holds a streamer, and the collection's apogees deploy nothing [S13, S15].
- **#1060**, side force and yaw moment not computed, which matters for asymmetric fin sets at an angle of attack;
  **#2983**, angled fins give no pitching moment in calm air; **#1062**, off-axis cluster thrust "not modeled"
  (15.03; the examples' clusters are symmetric) [S19].
- **#2370**, transition inertia by numerical integration, with no error size; by the forum's −0.75% for a large
  inertia change in a 20 mph wind (§2), **Inference**: far below 0.5% [S19].
- **#3051 and #870**, centering rings ignore clustered tubes. **Inference:** on an invented 98 mm ring with three
  31 mm holes, rings 1% to 3% of launch mass, apogee falls 0.1% to 0.8%. Whether HPR Sim's ring mass agrees on
  cluster designs is unchecked [S19].
- **#2507**, effective rail length from lugs and buttons, "calculated but then ignored"; nothing in calm vertical
  flight (**Inference**) [S19]. **#2892**, ground-hit speed includes the wind's drift (calm descents agree).
- **#3399**, a packed length with an automatic radius saved by getter order (traced to #2176, merged 2023-04-12,
  so **Inference**: 24.12 too); **#3398**, an override saved as the part's mass. Both tools read the saved number.
  **#2427**, a spent booster's stability with an open front, matters after separation only.
- **Missing features** move apogee only on designs that use a workaround: ring fins (#806), nested boattails
  (#2200), rail guides (#1529), beveled fins (#1626; open pull request #3173), pods on fins (#1145), an air-start
  motor's thrust delay (#1496), and recovery requests (#818, #2895, #1864, #2028) [S19].

## 5. Closed, for the record

#3041 (not planned): a typed non-integer delay snaps to an integer. #2639 (side boosters, about 3 km against 2.15 km
in 24.12 beta 1) was fixed by #2667 before 24.12's release. #3342 was explained, not changed [S20].

## 6. What this suggests

- **Terms.** OpenRocket is GPL-3.0; only issue, pull-request and forum text is cited, nothing copied. The motor
  database's curves come from ThrustCurve with unstated terms: cite impulses only.
- **M2.9's OpenRocket cells.** No open defect has an own-report size above 0.5% of apogee, so the list ADR-246 asks
  for holds only unsized rows: #3391 (supersonic flights, OpenRocket high), #3318 (pending fix, OpenRocket high on
  stubby noses), #2489 (wind, OpenRocket high), #1400 (bounded below 5%, shared), #2031 (descent only).
- **M2.3c6.** A fineness-0.58 ellipsoid beside ADR-273's fineness-3 nose probe would show a #3318 merge as its own
  row. 24.12's component drag on *Base drag hack (short-wide)* and *Dual parachute deployment* would turn this
  note's two upper bounds into measurements.
- **A guard.** Warn when a file picks 229H255-14A with the 316 N·s digest; no comparison moves, a user's design does.

## Open questions

- Whether #3391's hump is in 24.12, and how long the J570W-P flight stays above Mach 1; what 24.12 gives the
  fineness-0.58 ellipsoid subsonically (its component drag would say).
- How many collection flights pass Mach 1 on a tangent ogive, or carry a stubby stored-table nose.
- #2489's and #2892's numbers, which exist only in images; whether #1062 and #3399 hold in 24.12 itself.

## Sources

All read 2026-10-17; issue texts are cached in the gitignored `refs/research/openrocket-apogee-defects/`.

1. **S1.** Open issues and pull requests, GitHub API. <https://github.com/openrocket/openrocket/issues>
2. **S2.** #3391. <https://github.com/openrocket/openrocket/issues/3391>
3. **S3.** #3318, description, comments and reviews. <https://github.com/openrocket/openrocket/pull/3318>
4. **S4.** #2489 and its timeline. <https://github.com/openrocket/openrocket/issues/2489>
5. **S5.** Rocketry Forum, "Weathercock simulations all over the place", from 2024-05-30.
   <https://www.rocketryforum.com/threads/weathercock-simulations-all-over-the-place.186584/>
6. **S6.** #2348, comments of 2023-09-23 and 2026-10-11. <https://github.com/openrocket/openrocket/issues/2348>
7. **S7.** ThrustCurve, 229H255-14A. <https://www.thrustcurve.org/motors/Cesaroni/229H255-14A/>
8. **S8.** #2393. <https://github.com/openrocket/openrocket/issues/2393>
9. **S9.** #3205. <https://github.com/openrocket/openrocket/issues/3205>
10. **S10.** #1400. <https://github.com/openrocket/openrocket/issues/1400>
11. **S11.** S. Niskanen, OpenRocket technical documentation, pp. 43 and 93, and eq. B.4.
    <https://openrocket.sourceforge.net/techdoc.pdf>
12. **S12.** [`validation/reports/openrocket-flights.md`](../../validation/reports/openrocket-flights.md) and
    `validation/fixtures/ork/openrocket-drag-curves.json`.
13. **S13.** OpenRocket 24.12 release jar, `datafiles/examples/` (17 files), read as file samples.
    <https://github.com/openrocket/openrocket/releases/tag/release-24.12>
14. **S14.** HPR Sim #222. <https://github.com/nrdptel/fusionspace-eridanus/issues/222>
15. **S15.** [`validation/reports/fixture-flights.md`](../../validation/reports/fixture-flights.md).
16. **S16.** The project's record of 24.12's motor database, written by
    `validation/oracles/openrocket/motor_database.py` (gitignored output; 1,452 entries, 85 designs).
17. **S17.** Rocketry Forum, posts #1, #2 and #4, 2023-11-08.
    <https://www.rocketryforum.com/threads/issues-with-openrocket-simulations-after-upgrade.183055/>
18. **S18.** #2031. <https://github.com/openrocket/openrocket/issues/2031>
19. **S19.** #1060, #2983, #1062, #2370, #3051, #870, #2507, #2892, #3399, #3398, #2427, #806, #2200, #1529,
    #1626, #3173, #1145, #818, #2895, #1864, #2028, #1496. <https://github.com/openrocket/openrocket/issues>
20. **S20.** #3041, #2639 (fixed by #2667), #3342. <https://github.com/openrocket/openrocket/issues/2639>
21. **S21.** [`openrocket-library-flights.md`](../../validation/reports/openrocket-library-flights.md).
