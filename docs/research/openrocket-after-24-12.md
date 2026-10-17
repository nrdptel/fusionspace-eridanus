# What OpenRocket changed after 24.12 that moves an apogee

**In short: no OpenRocket release after 24.12 (2025-07-27) has shipped as of 2026-10-16, but its development line
(`unstable`, versioned "26.xx") has merged 262 pull requests since, and about a dozen move a number HPR Sim
compares.** The largest is fin–body interference: subsonic fin normal-force slope rises by the factor 1 + τ (20% to
50% for fins spanning 2 to 0.5 calibers), which moves OpenRocket's center of pressure aft and its margin up; the
follow-up pull request's sizes, mostly supersonic, are 46 to 100 mm aft of the previous development build [S5, S6].
Two changes the 2026-10-08 review counted as moving apogees, thrust at altitude (#3327) and powered base drag (#3244),
are **off by default**: both need a nozzle exit diameter, which is 0 unless a user enters it [S3, S4]; **Inference:**
no 24.12 file carries one, since the field arrived in August 2026. Smaller drag changes pull both ways: the
tangent-ogive nose loses its residual 0.002 of subsonic pressure drag (apogee up) and body friction is corrected
upward (apogee down) [S8, S10]. No change to the default surface finish was found. The development line's only current
builds are Linux snap packages on the snap store's edge channel, dated 2026-10-07 and 2026-10-08; a Maven core-library
snapshot dates from 2026-04-14 [S2]. How sure: release status, merge dates and stated sizes are read from GitHub and
the snap store on 2026-10-16; every size this note derives is marked **Inference**. It moves [M2.3c3, OpenRocket's
newest release](../decisions-and-roadmap.md#m2-3c3) (nothing to fly yet unless an edge build counts), the collection's
gap of 0.93 points (12.95% against 12.02% mean absolute apogee error, `validation/reports/fixture-flights.md`), and
the causes of [M2.3c6, the gap by cause](../decisions-and-roadmap.md#m2-3c6).

**What this covers.** Pull-request descriptions, review discussion, issue text, release notes and OpenRocket's user
guide only; no diff, changed file or source file was opened. **How far to trust it.** A pull request's description
says what its author meant to change; whether the merged code does that is checked only by flying the jar.

## 1. Release status on 2026-10-16

- **Releases.** GitHub lists 44 releases; the newest is 24.12, published 2025-07-27, with 24.12.RC.01
  (2025-03-25) before it [S1]. OpenRocket's downloads page offers v24.12 only [S13].
- **Snapshots.** The snap store's edge channel carries `26.xx-g44feea3` for amd64 (2026-10-07) and
  `26.xx-g591c5e5` for arm64 (2026-10-08), license GPL-3.0, Linux only [S2]. The Maven snapshot repository holds a
  core library `26.xx.SNAPSHOT` last updated 2026-04-14, before #3190, #3220 and every change after them [S2].
  The build guide calls development builds "26.xx-SNAPSHOT" [S12]. A forum thread of 2026-10-03, titled for a
  "26XX" snapshot, names no download [S14].
- **Merges.** 262 pull requests merged into `unstable` from 2025-07-28 to 2026-10-16, counted from the GitHub API
  by merge date; all 262 titles were screened and 55 pull-request descriptions read. The last merges were on
  2026-10-07 (#3382 at 10:18 UTC, #3296 at 14:31 UTC); none from 2026-10-08 to 2026-10-16 [S1]. The amd64 and
  armhf `g44feea3` builds date from 11:15 and 11:35 UTC on 10-07, after #3382 but before #3296 (a "save as" fix in
  the interface); the `g591c5e5` builds of 10-08 follow every merge [S2]. So every physics change below is in both.
- **#3391** (tangent-ogive pressure drag above Mach 1) is open, with no comment and no linked fix [S9].

## 2. Off by default: thrust at altitude and powered base drag

- **#3244, powered base drag** (merged 2026-08-25). While a motor fires, the base area its nozzle exit covers is
  taken out of the base-drag area, per wake; coasting is unchanged, citing NACA RM L50I18 and Fleeman [S3]. The
  description first used the case diameter; review settled on an entered nozzle exit diameter, default 0, with no
  correction at 0 [S3, comment of 2026-08-25]. A reviewer's M1350 has a 75 mm case and a 47 mm exit. Stated size:
  none in text ("slightly higher" speed). Band: all Mach, powered phase. **Sign:** apogee up when set.
- **#3327, thrust at altitude** (merged 2026-09-23). Thrust rises by A_e (p_ref − p), with p_ref = 101,325 Pa,
  using #3244's diameter; 0 gives no correction [S4]. The user guide says leaving it 0 matters only on very high
  flights [S11]. Stated size: a two-stage N5800 design with a 2.5 in (64 mm) exit rose from 79,204 ft to 85,910 ft
  in OpenRocket (+8.47%) and from 79,031 ft to 85,336 ft in RASAero (+7.98%), base drag and thrust together [S4].
  **Inference:** at a field 1,400 m up (standard pressure 85.6 kPa) a 25 mm exit adds 7.7 N at the pad.
- **Moves:** nothing in HPR Sim's comparisons as they run, since 24.12 files carry no exit diameter. If M2.3c3
  sets one, OpenRocket's apogees rise and the collection's gap narrows (**Inference**); the size is unbounded until
  each motor's exit is known (the thrust-at-altitude idea in [design and physics ideas](ideas-design-and-physics.md)).

## 3. Fin–body interference: normal force and center of pressure

- **#3220** (merged 2026-08-10, reverted by #3232 an hour later, restored by #3233 on 2026-08-11). Adds the lift
  the fins induce on the body: the subsonic interference factor goes from 1 + τ to (1 + τ)², τ = r/(s + r),
  through Mach 0.9, faded to the old factor by Mach 1.5, citing NACA Report 1307 eqs. 14 and 21 [S5]. The
  description says this moves the CP aft. #3233's text says a test's apogee time fell from 3.62 s to 3.35 s [S5].
- **#3263** (merged 2026-09-09) splits the fin and body loads and puts each at its own CP (NACA 1307 charts 15
  and 16, planar-pressure equations supersonic), for trapezoidal fins with r/s ≤ 0.6 on a constant tube; other
  fins keep #3220's factor. Subsonic CNa is unchanged and the CP moves a few millimeters from #3220's; against the
  previous development build the CP moves aft about 51 mm (an ARC payload rocket, Mach 2), 46 mm (dual-parachute
  example) and about 100 mm (the simulation-extension examples, Mach 3) [S6]. Canted-fin roll forcing drops.
- **Size, Inference.** The fin set's CNa grows by 1 + τ: +50% at a span of 0.5 caliber, +33% at 1, +20% at 2. On an
  invented 15-caliber rocket (ogive nose 4 calibers; four fins at the tail, root chord 1 to 2 calibers, tip a third to
  half of it, Barrowman, body lift left out) the CP moves aft 1.12 to 1.20, 0.45 to 0.53 and 0.12 to 0.18 calibers for
  spans of 0.5, 1 and 2 calibers, the spread set by the chords. HPR Sim uses Barrowman's 1 + τ and leaves the body
  term out (`docs/physics/aero.md`, "Not modeled").
- **Moves:** every margin comparison: the census's margin at rod clearance (within 0.016 calibers on 41 of 53 today)
  and the private designs' margins would show OpenRocket's margin up by tenths of a caliber, subsonic (**Inference**,
  from the invented rocket). In wind, a stiffer rocket weathercocks more, so OpenRocket's collection apogees fall and
  the gap widens (sign **Inference**; size unbounded until flown). Band: all, largest subsonic for the margin,
  supersonic for the CP.
- **Related fixes.** #3236 (2026-08-12): the transonic fin CNa used the queried Mach in an endpoint slope and read
  high in the transonic region; its author calls the difference "hardly noticeable" past Mach 0.9 [S7]. #3262
  (2026-08-27; #3279 closed unmerged for the same issue, #3196): holds a low-aspect-ratio fin's supersonic CP at the
  quarter chord below A·√(M² − 1) = 0.84 instead of crossing a pole at 0.5 [S7]. #3235 (2026-08-11): tube fins'
  CP sat at the leading edge from Mach 0.5 to 2.0 because an interpolation was never filled [S7]. **Inference:** #3235
  is why HPR Sim's tube-fin probes find OpenRocket's tube center at the leading edge from Mach 0.6
  ([accuracy](../accuracy.md)); the newer jar should change those 28 probe rows.

## 4. Drag

- **#2999, tangent-ogive nose** (merged 2026-01-05, from #2998). A tangent ogive's pressure drag is set to 0. In 24.12
  a joint angle taken over the last 1% of the nose gave sin φ ≈ 0.0016 and about 0.002 at Mach 0 on a fineness-3
  nose [S8]. #3391 reports the same zero also feeds the supersonic curve: 24.12 gives about 0.0007 above Mach 1, the
  development line exactly 0, where the documented method gives about 0.13 at Mach 1.5 [S9]. #3078 (2026-03-28) notes
  the zero as a visible difference from 24.12 [S10]. **Size, Inference:** −0.002 is 0.33% to 0.5% of a drag
  coefficient of 0.4 to 0.6; at −0.31% to −0.57% of apogee per 1% of drag ([apogee bias](apogee-bias.md)),
  OpenRocket's subsonic apogee rises 0.1% to 0.3% on tangent-ogive designs. Supersonic: 0.0007 to 0 is negligible
  beside #3391's 0.13. **Moves:** the collection's gap narrows slightly; ADR-273's probe should record both jars.
- **#3237, body friction** (merged 2026-08-12). The body's friction factor 1 + 1/(2f_B) took f_B as length over
  the largest *radius*, doubling it; a fineness-10 body got 1.025 instead of 1.05 [S10]. **Size, Inference:** body
  friction +2.4% at fineness 10, +1.6% at 15, +1.2% at 20. Even if all drag were body friction, apogee would fall at
  most 1.4% at fineness 10; with body friction 30% to 50% of drag, 0.1% to 0.7%. HPR Sim already uses length over
  the largest diameter (`docs/physics/aero.md`), so the census and private-design gaps should shrink while the
  collection's gap widens, OpenRocket nearer the logs (**Inference**). Band: subsonic mainly.
- **#3078, the drag report** (merged 2026-03-28): component analysis reports fin pressure and base drag apart
  [S10]. No physics change; **Inference:** M2.3c6's swap reads that per-component interface, so its labels change.
- **Lookup tables** (#2889, merged 2025-11-30; editing fixes #3338 to #3341, 2026-09-22). A simulation can take
  drag (Mach, angle of attack, C_D) and stability (Mach, angle of attack, C_N, C_m, CP) CSV tables, clamped at their
  edges; the component forces are then zero [S11, S15]. No default change; a tool for M2.3c6 (§8).
- **Finish defaults.** No change found: the 262 titles, and searches for "roughness", "surface finish" and
  "default finish" since 2025-03-01, show none, and the user guide's finish page gives no new default [S11].

## 5. Air

- **#2834, humidity** (merged 2026-01-02; #3238 on 2026-08-13 made a humidity-only change take effect). Relative
  humidity enters the gas constant; it is a simulation option. The description proposes it fall 6% per km aloft,
  review asked to hold it constant, and no text says which merged [S16]. **Size, Inference:** 50% humidity lowers
  density 0.44% at 20 °C and 0.79% at 30 °C, so apogee rises 0.14% to 0.45%. HPR Sim already models humid air.
- **#2997, geopotential altitude** (merged 2026-01-05, from #2731). The ISA model now converts geometric to
  geopotential altitude; the issue puts the error at about 20 m at 11 km and 162 m at 32 km [S16]. **Inference:**
  the fix raises density at a given height (0.03% at 4.4 km, 0.19% at 10 km), so apogee falls, under 0.1% for
  flights below 10 km.

## 6. Tumble and abort rules

- **#3190** (merged 2026-08-03, from #3183): tumbling is now declared from an angle of attack filtered over the
  rocket's own pitch period, not one step past the stall angle with CG behind CP. On an Estes Alpha III with a 1 m
  guide, a 6 m/s steady crosswind went from an abort at 1.1 m to a 132.0 m flight; gust aborts at 0 m/s fell from
  2 of 12 to 0 of 12; flights that already worked were unchanged [S17]. #3183 had shown a stable rocket's ~590 m
  flight ending at ~1 m on some turbulence seeds [S17].
- **#3382** (merged 2026-10-07, from #3375): the tumble-under-thrust and deploy-under-thrust checks use the
  current step's thrust; a finless booster no longer aborts just after burnout on a stale 0.6 N [S17].
- **Moves, Inference:** the census's one aborted flight (*Pods--powered with recovery deployment*, "Stage began to
  tumble under thrust" at 1.81 s, after separation) fits #3375's pattern, so the newer jar may fly it to apogee
  and add a compared flight. HPR Sim's scripts set turbulence to 0 in the collection's runs, so #3189 (wind seeded
  from the simulation's seed, 2026-07-29) changes nothing there.

## 7. Mass, motors and the rest

- **#3319** (2026-09-16): with "override mass of all subcomponents", inertia kept the geometric mass, so roll and
  pitch inertia were off by the override factor; "apogee was largely unaffected" [S18]. **Inference:** a candidate
  cause for the 13 of 71 designs whose pitch inertia is over 1% from OpenRocket's ([accuracy](../accuracy.md)).
- **#3217** (2026-08-12): each motor mount now uses its own ignition time in the motor mass; no size [S18].
  Staged and air-start mass histories may move. **#3204** (2026-08-12): a pod motor's ejection charge no longer
  deploys the sustainer's parachute, and a device deploys once [S18]; the census's pod examples may move.
- **#3197** (2026-07-31): the propulsive damping term's sign is fixed, matching RockSim; no size, and the text
  does not say whether flight or only the reported coefficient changes [S18]. #2911 fixes an optional listener.
- **Motor data.** ThrustCurve's v1 API and an SQLite store (#2984, #2985), database updates at startup (#2991,
  #3199, #3255) and curves embedded in `.ork` files, preferred over approximate matches (#2972, #3066, #3337)
  [S18]. **Inference:** the newer jar can resolve a different curve for a 24.12 file's motor; that would move the
  thrust part of every comparison, size unknown.
- **No apogee change by default:** RK6 is an option (#2839, fixed by #3013); gravity is configurable, the default
  as before (#2923); #3331 deletes the *Base drag hack* example, which the census flies from 24.12's copy.

## 8. What this suggests

- **Terms.** OpenRocket is GPL-3.0; only its issue, pull-request, release and guide text is cited; nothing is copied.
- **M2.3c3 has no release to fly.** If a published binary counts, the edge snap (amd64 revision 1770, 253 MB) holds
  every physics change above; it would be pinned by revision and hash, unpacked and run as a jar, Linux first. A
  suggestion, not a plan: fly it with each comparison's motor digests checked in both jars, turbulence 0, updates off
  and the exit diameter left 0, then again with it set, so §2 has its own row.
- **M2.3c6.** The lookup tables let OpenRocket fly HPR Sim's drag and CP, isolating aerodynamics; expected effect:
  the drag and stability parts of the 0.93-point gap measured inside one code. #3078's split report and #2999's
  zero belong in the nose line ADR-273 asks for.
- **ADR-212's list.** It holds #3220, #3263, #3244 and #3327; this note adds #2999, #3237, #3236, #3262, #3235,
  #2834, #2997, #3190, #3382, #3319, #3217 and #3204, and corrects that #3244 and #3327 are off by default.
- **Leads with no change.** #2442 (vent holes) is an open request that asks for no drag effect; #2507 (effective
  rail length from the buttons) is open with no pull request [S19].

## Open questions

- The net sign on the collection: fin interference and body friction lower OpenRocket's apogees, the ogive and
  humidity raise them; only flying both jars on the same flights gives the sum.
- Whether the humidity option defaults to 0, whether it falls with height, and whether #3197 changes flight or only a
  reported coefficient.
- Whether the false hump of about 0.075 near Mach 1.1 in #3391 is in 24.12, the development line, or both.

## Sources

All read 2026-10-16; copies of most in the gitignored `refs/research/openrocket-after-24-12/`.

1. **S1.** OpenRocket releases and merged pull requests, GitHub API. <https://github.com/openrocket/openrocket/releases>
2. **S2.** Snap store channel map <https://snapcraft.io/openrocket>; Maven snapshot metadata
   <https://central.sonatype.com/repository/maven-snapshots/info/openrocket/core/maven-metadata.xml>.
3. **S3.** #3244 and review. <https://github.com/openrocket/openrocket/pull/3244>
4. **S4.** #3327 and review. <https://github.com/openrocket/openrocket/pull/3327>
5. **S5.** #3220, #3232, #3233. <https://github.com/openrocket/openrocket/pull/3220>
6. **S6.** #3263. <https://github.com/openrocket/openrocket/pull/3263>
7. **S7.** #3236, #3262, #3235, #3279, #3196. <https://github.com/openrocket/openrocket/pull/3262>
8. **S8.** #2998 and #2999. <https://github.com/openrocket/openrocket/issues/2998>
9. **S9.** #3391. <https://github.com/openrocket/openrocket/issues/3391>
10. **S10.** #3237 and #3078. <https://github.com/openrocket/openrocket/pull/3237>
11. **S11.** User guide: Thrust Curves; Advanced Flight Simulation (lookup tables); Overrides and Surface Finish.
    <https://openrocket.readthedocs.io/en/latest/user_guide/thrust_curves.html>
12. **S12.** Building and Releasing. <https://openrocket.readthedocs.io/en/latest/dev_guide/building_releasing.html>
13. **S13.** Downloads. <https://openrocket.info/downloads.html>
14. **S14.** Rocketry Forum thread, 2026-10-03.
    <https://www.rocketryforum.com/threads/omg-openrocket-26xx-sapshot-drag-csv-and-stability-csv.199161/>
15. **S15.** #2889. <https://github.com/openrocket/openrocket/pull/2889>
16. **S16.** #2834, #3238, #2731, #2997. <https://github.com/openrocket/openrocket/pull/2834>
17. **S17.** #3183, #3190, #3375, #3382. <https://github.com/openrocket/openrocket/pull/3190>
18. **S18.** #3319, #3217, #3204, #3197, #2911, #2984, #2985, #2991, #3199, #3255, #2972, #3066, #3337.
    <https://github.com/openrocket/openrocket/pull/3319>
19. **S19.** #2442 and #2507. <https://github.com/openrocket/openrocket/issues/2507>
