# What OpenRocket changed after 24.12 that moves an apogee

Re-checked 2026-11-05: 9 claims corrected.

**In short: no OpenRocket release after 24.12 (2025-07-27) has shipped as of 2026-11-05, but release notes for a 26.11
beta merged on 2026-11-04, and the development line (`unstable`, versioned "26.xx") has merged 286 pull requests since
24.12; 19 (§8's list) could move a number HPR Sim compares, and §7's motor-data changes might.** The largest for a
single-stage apogee is #3431 (2026-11-02): ogive noses get a cone's supersonic pressure drag, 0 to 0.130 at Mach 1.5 on
a 3:1 tangent ogive, and a Mach 2.5 test rocket's apogee falls 5.5% [S9]; #3410's damping fix drops a separated
booster's 20% [S18]. The largest for stability is fin–body interference: subsonic fin normal-force slope rises by the
factor 1 + τ (20% to 50% for fins spanning 2 to 0.5 calibers), which moves OpenRocket's center of pressure aft and its
margin up; a follow-up puts it 46 to 100 mm aft of the previous development build, mostly supersonic [S5, S6]. Thrust at
altitude (#3327) and powered base drag (#3244) are **off by default**: both need a nozzle exit diameter, which is 0
unless a user enters it [S3, S4]; **Inference:** no 24.12 file carries one, since the field arrived in August 2026. The
only current builds are Linux edge snaps of 2026-11-03 [S2]. How sure: read from GitHub and the snap store on 2026-10-16
and 2026-11-05; derived sizes are marked **Inference**. It moves [M2.3c3, OpenRocket's newest
release](../decisions-and-roadmap.md#m2-3c3) (nothing tagged to fly yet), the collection's gap of 0.81 points (12.07%
against 11.26% mean absolute apogee error, `validation/reports/fixture-flights.md`), and the causes of [M2.3c6, the gap
by cause](../decisions-and-roadmap.md#m2-3c6).

**What this covers.** Pull-request, review, issue, release and user-guide text only; no diff or source file was opened.
**How far to trust it.** A pull request's description says what its author meant to change; whether the merged code does
that is checked only by flying the jar.

## 1. Release status on 2026-11-05

- **Releases.** GitHub lists 44 releases; the newest is 24.12, published 2025-07-27, with 24.12.RC.01 (2025-03-25)
  before it, and no tag after `release-24.12` [S1]. The downloads page offers v24.12 only; the release notes' newest
  entry is 24.12 [S13]. **#3438**, "Release notes for 26.11 beta release", merged 2026-11-04 at 19:22 UTC. No milestone
  names a 25.x or 26.x version [S1].
- **Snapshots.** The snap store's edge channel carries `26.xx-g67c5ec1` (the merge commit of #3434), built 2026-11-03:
  amd64 revision 1797 (206 MB), arm64 1799, and armhf, ppc64el and s390x; stable and beta carry 24.12; license GPL-3.0,
  Linux only [S2]. The Maven snapshot repository's core library `26.xx.SNAPSHOT` is still dated 2026-04-14, before
  #3190, #3220 and every change after them [S2]. The build guide calls development builds "26.xx-SNAPSHOT" [S12]. A
  forum thread of 2026-10-03, titled for a "26XX" snapshot, names no download [S14].
- **Merges.** 262 pull requests merged into `unstable` from 2025-07-28 to 2026-10-16 (recounted on 11-05), and 24 more
  by 11-05, the first on 10-25, counted from the GitHub API by merge date; four of the 24 touch physics (§4, §7) [S1].
  None of the pull requests below was reverted since. #3318, a subsonic drag floor for stored-table noses, is open,
  hoped for after the beta.

## 2. Off by default: thrust at altitude and powered base drag

- **#3244, powered base drag** (merged 2026-08-25). While a motor fires, the base area its nozzle exit covers is taken
  out of the base-drag area, per wake; coasting is unchanged, citing NACA RM L50I18 and Fleeman [S3]. The description
  first used the case diameter; review settled on an entered nozzle exit diameter, default 0, with no correction at 0
  [S3, comment of 2026-08-25]. Stated size: none. Band: all Mach, powered phase. **Sign:** apogee up when set.
- **#3327, thrust at altitude** (merged 2026-09-23). Thrust rises by A_e (p_ref − p), with p_ref = 101,325 Pa, using
  #3244's diameter; 0 gives no correction [S4]. Stated size: a two-stage N5800 design with a 2.5 in (64 mm) exit rose
  from 79,204 ft to 85,910 ft in OpenRocket (+8.47%) and from 79,031 ft to 85,336 ft in RASAero (+7.98%), base drag and
  thrust together [S4]. **Inference:** at a field 1,400 m up (85.6 kPa) a 25 mm exit adds 7.7 N at the pad.
- **Moves:** nothing in HPR Sim's comparisons as they run, since 24.12 files carry no exit diameter. If M2.3c3 sets one,
  OpenRocket's apogees rise and the collection's gap narrows (**Inference**); the size is unbounded until each motor's
  exit is known (the thrust-at-altitude idea in [design and physics ideas](ideas-design-and-physics.md)).

## 3. Fin–body interference: normal force and center of pressure

- **#3220** (merged 2026-08-10, reverted by #3232 an hour later, restored by #3233 on 2026-08-11). Adds the lift the
  fins induce on the body: the subsonic interference factor goes from 1 + τ to (1 + τ)², τ = r/(s + r), through Mach
  0.9, faded to the old factor by Mach 1.5, citing NACA Report 1307 eqs. 14 and 21 [S5]. The description says this moves
  the CP aft. #3233's text says a test's apogee time fell from 3.62 s to 3.35 s [S5].
- **#3263** (merged 2026-09-09) splits the fin and body loads and puts each at its own CP (NACA 1307 charts 15 and 16,
  planar-pressure equations supersonic), for trapezoidal fins with r/s ≤ 0.6 on a constant tube; other fins keep
  #3220's factor. Subsonic CNa is unchanged and the CP moves a few millimeters from #3220's; against the previous
  development build the CP moves aft about 51 mm (an ARC payload rocket, Mach 2), 46 mm (dual-parachute example) and
  about 100 mm (the simulation-extension examples, Mach 3) [S6].
- **Size, Inference.** The fin set's CNa grows by 1 + τ: +50% at a span of 0.5 caliber, +33% at 1, +20% at 2. On an
  invented 15-caliber rocket (ogive nose 4 calibers; four fins at the tail, root chord 1 to 2 calibers, tip a third to
  half of it, Barrowman, body lift left out) the CP moves aft 1.12 to 1.20, 0.45 to 0.53 and 0.12 to 0.18 calibers for
  spans of 0.5, 1 and 2 calibers, the spread set by the chords. HPR Sim uses Barrowman's 1 + τ and leaves the body term
  out (`docs/physics/aero.md`, "Not modeled").
- **Moves:** every margin comparison: the census's margin at rod clearance (within 0.016 calibers on 41 of 53 today) and
  the private designs' margins would show OpenRocket's margin up by tenths of a caliber, subsonic (**Inference**, from
  the invented rocket). In wind, a stiffer rocket weathercocks more, so OpenRocket's collection apogees fall and the gap
  widens (sign **Inference**; size unbounded until flown).
- **Related fixes.** #3236 (2026-08-12): the transonic fin CNa used the queried Mach in an endpoint slope and read high
  in the transonic region; its author calls the difference "hardly noticeable" past Mach 0.9 [S7]. #3262 (2026-08-27;
  #3279 closed unmerged for the same issue, #3196): holds a low-aspect-ratio fin's supersonic CP at the quarter chord
  below A·√(M² − 1) = 0.84 instead of crossing a pole at 0.5 [S7]. #3235 (2026-08-11): tube fins' CP sat at the
  leading edge from Mach 0.5 to 2.0 because an interpolation was never filled [S7]. **Inference:** #3235 is why HPR
  Sim's tube-fin probes find OpenRocket's tube center at the leading edge from Mach 0.6 ([accuracy](../accuracy.md));
  the newer jar should change those 28 probe rows.

## 4. Drag

- **#2999, tangent-ogive nose** (merged 2026-01-05, from #2998). A tangent ogive's pressure drag is set to 0. In 24.12 a
  joint angle taken over the last 1% of the nose gave sin φ ≈ 0.0016 and about 0.002 at Mach 0 on a fineness-3 nose
  [S8]. **Size, Inference:** −0.002 is 0.33% to 0.5% of a drag coefficient of 0.4 to 0.6; at −0.31% to −0.57% of
  apogee per 1% of drag ([apogee bias](apogee-bias.md)), OpenRocket's subsonic apogee rises 0.1% to 0.3% on tangent
  ogives.
- **#3431, ogive wave drag** (merged 2026-11-02, fixing #3391). Above Mach 1 the joint angle also fed the conical
  method: about 0.0007 in 24.12, exactly 0 after #2999, plus a fineness-independent bump of 0.074 at Mach 1.1 from the
  Mach 1 to 1.3 cubic. Ogives now take an equal-length cone's half-angle. A 3:1 tangent ogive at Mach 1.5 goes from 0 to
  0.130 (RASAero II's fit: 0.106). A Mach 2.5 test rocket (5:1 nose, L1115) loses 5.5% of apogee, 1.9% with a 0.5 secant
  ogive; the examples' one supersonic flight loses 2.2%, subsonic ones at most 0.15% [S9]. **Inference:** the bump came
  from the cubic, so 24.12 has it too. **Moves:** OpenRocket's supersonic apogees fall against 24.12; the collection's
  gap moves little (mostly subsonic); ADR-273's probe should record both jars.
- **#3237, body friction** (merged 2026-08-12). The body's friction factor 1 + 1/(2f_B) took f_B as length over the
  largest *radius*, doubling it; a fineness-10 body got 1.025 instead of 1.05 [S10]. **Size, Inference:** body friction
  +2.4% at fineness 10, +1.6% at 15, +1.2% at 20. With body friction 30% to 50% of drag, apogee falls 0.1% to 0.7%. HPR
  Sim already uses length over the largest diameter (`docs/physics/aero.md`), so the census and private-design gaps
  should shrink while the collection's gap widens, OpenRocket nearer the logs (**Inference**). Band: subsonic mainly.
- **#3078, the drag report** (merged 2026-03-28): component analysis reports fin pressure and base drag apart [S10]. No
  physics change; **Inference:** M2.3c6's swap reads that per-component interface, so its labels change.
- **Lookup tables** (#2889, merged 2025-11-30; editing fixes #3338 to #3341, 2026-09-22). A simulation can take drag
  (Mach, angle of attack, C_D) and stability (Mach, angle of attack, C_N, C_m, CP) CSV tables, clamped at their edges,
  the angle-of-attack column optional; the component forces are then zero [S11, S15]. A tool for M2.3c6 (§8).
- **Finish defaults.** None in the 286 titles, searches for "roughness" and "surface finish", or the guide [S11].

## 5. Air

- **#2834, humidity** (merged 2026-01-02; #3238 on 2026-08-13 made a humidity-only change take effect). Relative
  humidity enters the gas constant; it is a simulation option. The description proposes it fall 6% per km aloft, review
  asked to hold it constant, and a 2026-01-02 commit in it did so [S16]. **Size, Inference:** 50% humidity lowers
  density 0.44% at 20 °C and 0.79% at 30 °C, so apogee rises 0.14% to 0.45%. HPR Sim already models humid air.
- **#2997, geopotential altitude** (merged 2026-01-05, from #2731). The ISA model now converts geometric to geopotential
  altitude; the issue puts the error at about 20 m at 11 km and 162 m at 32 km [S16]. **Inference:** the fix raises
  density at a given height (0.03% at 4.4 km, 0.19% at 10 km), so apogee falls, under 0.1% for flights below 10 km.

## 6. Tumble and abort rules

- **#3190** (merged 2026-08-03, from #3183): tumbling is now declared from an angle of attack filtered over the rocket's
  own pitch period, not one step past the stall angle with CG behind CP. On an Estes Alpha III with a 1 m guide, a 6 m/s
  steady crosswind went from an abort at 1.1 m to a 132.0 m flight; gust aborts at 0 m/s fell from 2 of 12 to 0 of 12;
  flights that already worked were unchanged [S17].
- **#3382** (merged 2026-10-07, from #3375): the tumble-under-thrust and deploy-under-thrust checks use the current
  step's thrust; a finless booster no longer aborts just after burnout on a stale 0.6 N [S17].
- **Moves, Inference:** the census's one aborted flight (*Pods--powered with recovery deployment*, "Stage began to
  tumble under thrust" at 1.81 s, after separation) fits #3375's pattern, so the newer jar may fly it to apogee and add
  a compared flight. #3189 (turbulence seeding, 2026-07-29) changes nothing at the scripts' turbulence 0.

## 7. Mass, motors and the rest

- **#3319** (2026-09-16): with "override mass of all subcomponents", inertia kept the geometric mass, so roll and pitch
  inertia were off by the override factor; "apogee was largely unaffected" [S18]. **Inference:** a candidate cause for
  the 13 of 71 designs whose pitch inertia is over 1% from OpenRocket's ([accuracy](../accuracy.md)).
- **#3217** (2026-08-12): each motor mount now uses its own ignition time in the motor mass; no size [S18]. Staged and
  air-start mass histories may move. **#3204** (2026-08-12): a pod motor's ejection charge no longer deploys the
  sustainer's parachute, and a device deploys once [S18]; the census's pod examples may move.
- **Damping.** #3197 (2026-07-31) fixed the jet-damping term's sign, but no flight used the term until #3434
  (2026-11-02), which adds it to the dynamics: over 54 example flights apogee rises by a median of 0.008%, at most 0.07%
  (0.45 m); #3433 (11-02) moves only plotted outputs [S18]. #3410 (2026-10-30) takes pitch and yaw damping about the CG:
  example apogees move at most 0.12%, a separated unstable booster's falls 20%, and one example with recovery off hits
  the ground at 68 m/s, not 14 [S18]. **Inference:** staged boosters' landing and apogee rows move; single-stage apogees
  hardly. #2911 fixes an optional listener.
- **Motor data.** ThrustCurve's v1 API and an SQLite store (#2984, #2985), database updates at startup (#2991, #3199,
  #3255) and curves embedded in `.ork` files, preferred over approximate matches (#2972, #3066, #3337) [S18].
  **Inference:** the newer jar can resolve a different curve for a 24.12 file's motor; that would move the thrust part
  of every comparison, size unknown.
- **No apogee change by default:** RK6 (#2839, #3013) and gravity (#2923) are options; #3331 deletes the *Base drag
  hack* example the census flies from 24.12.

## 8. What this suggests

- **Terms.** OpenRocket is GPL-3.0; only its issue, pull-request, release and guide text is cited; nothing is copied.
- **M2.3c3 has no release to fly yet, but a 26.11 beta is near.** Until it is tagged, the edge snap (amd64 revision 1797
  or later) holds every physics change above; it would be pinned by revision and hash, unpacked and run as a jar, Linux
  first. Once the beta's jar is published, it is the build to fly. A suggestion, not a plan: fly it with each
  comparison's motor digests checked in both jars, turbulence 0, updates off and the exit diameter left 0, then again
  with it set, so §2 has its own row.
- **M2.3c6.** The lookup tables let OpenRocket fly HPR Sim's drag and CP, isolating aerodynamics; expected effect: the
  drag and stability parts of the 0.81-point gap measured inside one code. #3078's split report and #2999's zero belong
  in the nose line ADR-273 asks for.
- **ADR-212's list.** It holds #3220, #3263, #3244 and #3327; this note adds #2999, #3431, #3237, #3236, #3262, #3235,
  #2834, #2997, #3190, #3382, #3410, #3434, #3319, #3217 and #3204, and finds #3244 and #3327 off by default.
## Open questions

- The net sign on the collection: fin interference, body friction, supersonic ogive drag, geopotential height and
  booster damping lower OpenRocket's apogees, the subsonic ogive and humidity raise them; only flying both jars on the
  same flights gives the sum.
- Which value the humidity option defaults to (its height profile and #3197's effect are answered in §5 and §7).
- Which pull requests the 26.11 beta's release notes list; the notes file was not read, and no tag exists yet.

## Sources

All read 2026-10-16 and again 2026-11-05; copies in the gitignored `refs/research/openrocket-after-24-12/` and
`refs/research/verify-openrocket-after-24-12/`.

1. **S1.** OpenRocket releases and merged pull requests, GitHub API. <https://github.com/openrocket/openrocket/releases>
2. **S2.** Snap store channel map <https://snapcraft.io/openrocket>; Maven snapshot metadata
   <https://central.sonatype.com/repository/maven-snapshots/info/openrocket/core/maven-metadata.xml>.
3. **S3.** #3244 and review. <https://github.com/openrocket/openrocket/pull/3244>
4. **S4.** #3327 and review. <https://github.com/openrocket/openrocket/pull/3327>
5. **S5.** #3220, #3232, #3233. <https://github.com/openrocket/openrocket/pull/3220>
6. **S6.** #3263. <https://github.com/openrocket/openrocket/pull/3263>
7. **S7.** #3236, #3262, #3235, #3279, #3196. <https://github.com/openrocket/openrocket/pull/3262>
8. **S8.** #2998 and #2999. <https://github.com/openrocket/openrocket/issues/2998>
9. **S9.** #3391 and #3431. <https://github.com/openrocket/openrocket/pull/3431>
10. **S10.** #3237 and #3078. <https://github.com/openrocket/openrocket/pull/3237>
11. **S11.** User guide: Thrust Curves; Advanced Flight Simulation (lookup tables); Overrides and Surface Finish.
    <https://openrocket.readthedocs.io/en/latest/user_guide/thrust_curves.html>
12. **S12.** Building and Releasing. <https://openrocket.readthedocs.io/en/latest/dev_guide/building_releasing.html>
13. **S13.** Downloads and release notes. <https://openrocket.info/downloads.html>, `release_notes.html`
14. **S14.** Rocketry Forum thread, 2026-10-03.
    <https://www.rocketryforum.com/threads/omg-openrocket-26xx-sapshot-drag-csv-and-stability-csv.199161/>
15. **S15.** #2889. <https://github.com/openrocket/openrocket/pull/2889>
16. **S16.** #2834, #3238, #2731, #2997. <https://github.com/openrocket/openrocket/pull/2834>
17. **S17.** #3183, #3190, #3375, #3382. <https://github.com/openrocket/openrocket/pull/3190>
18. **S18.** #3319, #3217, #3204, #3197, #3410, #3433, #3434, #2911, #2984, #2985, #2991, #3199, #3255, #2972, #3066,
    #3337. <https://github.com/openrocket/openrocket/pull/3319>
