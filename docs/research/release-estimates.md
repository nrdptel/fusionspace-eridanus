# Release estimates: how they were made

This note shows how the plan's release estimates were made. Trust them as padded guesses, not
promises.

**Bottom line:** on November 26, 2026, each release's estimate on [the
plan](https://hpr.fusionspace.co/plan.html#when-each-release-may-come) was made from three inputs:
the roadmap's queue, how many increments finished milestones took, and the pace of merges on `main`.
At that pace alone, release 0.2 would come about December 7, 2026 and 1.0 about April 24, 2027. Work
here has run long, so each stretch of the queue is padded: its time doubled before 0.2, tripled
before 0.3 and before 0.4, and quadrupled before 0.5 and before 1.0, the stretches that hold the
app, the newest kind of work. The factors are judgments, largest for the app. The stretches add up,
so a release that slips moves every later one with it. That puts 0.2 by the end of December 2026,
0.3 by the end of May 2027, 0.4 by the end of August 2027, 0.5 by the end of November 2027 and 1.0
by the end of April 2028. 0.3's padded date is May 25, six days before its month ends, so a little
more work makes it June. 1.0's is April 1, the first day of its month, so a little less work, or
the pace unrounded, puts it back in March. They are estimates, not promises: a release comes out
only when its checklist passes.

## The inputs

1. **The queue.** On November 26, 2026, [the roadmap's queue](../ROADMAP.md#queue) put release
   0.2 at place 5, 0.3 at 21, 0.4 at 34, 0.5 at 38 and 1.0 at 49. The queue is worked in
   order, so a release comes after the entries ahead of it, not only those its checklist names.
   M7.3f, at place 1 here, has merged since, and M7.3g, at place 2, ended and left the queue
   ([ADR-385](../decisions/0385-m7-3g2-a-second-rule-passes-every-row-and-fails-a-class.md)), so
   each place on the live queue is two lower.
2. **What a milestone takes.** An entry is split into increments as it is worked, each merged on
   its own. Counted in [the finished roadmap](../roadmap-done.md) as increments with no parts of
   their own, the log importers (M7.1) took 27, though
   [ADR-251](../decisions/0251-flight-log-importers-in-two-increments.md) planned two; readings
   and smoothing (M7.2) took 10; release 0.1 (M10.1) 12; the fit from the boost (M7.3d) 6, as its
   M7.3d3 split into M7.3d4 to M7.3d7; every solid motor offline (M5.7) 5. One ran short: the
   post-flight comparison drawn (M7.3e), sized here at 3 to 6 on November 24, merged as one.
3. **The pace.** From October 28 to November 25, 2026 (UTC), 29 days, 56 merges whose title starts with
   a milestone id landed on `main`: 13.5 a week. This command prints the count:
   `TZ=UTC git log 69bead4f --since=2026-10-28T00:00 --format=%s | grep -cE '^M[0-9]+\.[0-9]+'`.
   It counts a follow-up or a merge landed again too, and misses an increment whose title names
   its milestone later on, so it stands for the increments merged only roughly.

## Each entry's size

Low and high are the fewest and the most increments the entry might take, judged against the
finished milestones above. The middle is halfway between them. An M-number names a milestone in
[the roadmap](../ROADMAP.md).

| place | entry | low | high | middle |
|---|---|---|---|---|
| 1 | M7.3f the fit on the command line and in Python | 2 | 4 | 3 |
| 2 | M7.3g the least-squares intervals pass again | 2 | 6 | 4 |
| 3 | M2.8 honest uncertainty | 3 | 6 | 4.5 |
| 4 | M0.9g the figures' digits and the command line's words | 2 | 4 | 3 |
| 5 | M10.2 release 0.2 | 5 | 12 | 8.5 |
| | **before 0.2** | 14 | 32 | 23 |
| 6 | M0.13 awareness and care on the sites | 2 | 5 | 3.5 |
| 7 | M1.14 the rest: b12, c, d, e, f, g | 19 | 56 | 37.5 |
| 8 | M2.3c2 logged traces | 1 | 3 | 2 |
| 9 to 14 | M2.12 to M2.17, six measurements against the logs | 12 | 30 | 21 |
| 15 | M2.3d4 a stage's override that adds to its parts | 2 | 4 | 3 |
| 16 | M2.3c9 every flight OpenRocket flies | 2 | 5 | 3.5 |
| 17 | M7.3 the Bayesian fit's last two parts | 2 | 4 | 3 |
| 18 | M2.3c3 OpenRocket's newest release | 1 | 3 | 2 |
| 19 | M2.6 an open benchmark | 3 | 6 | 4.5 |
| 20 | M7.4 fault diagnosis | 8 | 20 | 14 |
| 21 | M10.3 release 0.3 | 4 | 10 | 7 |
| | **from 0.2 to 0.3** | 56 | 146 | 101 |
| 22 | M2.11 fin flutter held to measurement | 3 | 6 | 4.5 |
| 23 | M2.3c7 faster than sound: published flights | 2 | 5 | 3.5 |
| 24 | M6.3 challenge specs and presets | 3 | 8 | 5.5 |
| 25 | M6.4 airbrakes | 3 | 8 | 5.5 |
| 26 | M6.2e robust mode | 2 | 4 | 3 |
| 27 | M6.6 submission packs | 3 | 8 | 5.5 |
| 28 | M6.7 ejection charges | 3 | 6 | 4.5 |
| 29 | M3.5 RASAero files | 3 | 6 | 4.5 |
| 30 | M3.6 RocketPy files | 2 | 5 | 3.5 |
| 31 | M2.9 the other tools' cells | 2 | 4 | 3 |
| 32 | M6.8 the field kit | 3 | 6 | 4.5 |
| 33 | M0.7b rocketry explained: recovery, wind and motors | 2 | 4 | 3 |
| 34 | M10.4 release 0.4 | 4 | 10 | 7 |
| | **from 0.3 to 0.4** | 35 | 80 | 57.5 |
| 35 | M4.4 C and WebAssembly bindings | 3 | 6 | 4.5 |
| 36 | M9.0 the user-interface choice | 2 | 5 | 3.5 |
| 37 | M9.2 3D replay with the ghost | 5 | 15 | 10 |
| 38 | M10.5 release 0.5, the app preview | 10 | 25 | 17.5 |
| | **from 0.4 to 0.5** | 20 | 51 | 35.5 |
| 39 | M0.7c rocketry explained: live figures | 2 | 4 | 3 |
| 40 | M5.6a a catalog of HPR Sim's own | 2 | 5 | 3.5 |
| 41 | M8.2 the edit model | 3 | 8 | 5.5 |
| 42 | M9.1 the desktop app | 10 | 25 | 17.5 |
| 43 | M5.6b parachutes and recovery hardware | 2 | 5 | 3.5 |
| 44 | M8.1 the design assistant | 5 | 12 | 8.5 |
| 45 | M5.6c motor hardware and rail buttons | 2 | 5 | 3.5 |
| 46 | M5.6d more makers and electronics | 2 | 5 | 3.5 |
| 47 | M3.4 RockSim files | 3 | 6 | 4.5 |
| 48 | M9.3 the web app | 5 | 15 | 10 |
| 49 | M10.6 release 1.0 | 5 | 12 | 8.5 |
| | **from 0.5 to 1.0** | 41 | 102 | 71.5 |

M1.14's rest is b12 1 to 3, c 5 to 15, d 5 to 15, e 3 to 10, f 3 to 8 and g 2 to 5. Its h, the
Mach 2.5 to 3.5 band, isn't sized: release 0.3 doesn't need it ([M10.3](../ROADMAP.md#phase-8-releases-adr-162)),
and [the plan](https://hpr.fusionspace.co/plan.html#what-10-includes-and-what-comes-after) puts
that band after 1.0. The roadmap still files it under M1.14, at place 7; worked there, before 0.3,
it would move 0.3 and every later release.

## The arithmetic

Each stretch's middle, at 13.5 increments a week, then padded, then added up from November 26,
2026, in whole days, a part of a day dropped. The last column rounds the padded date up to the end
of its month.

| release | weeks at the pace | at the pace alone | padding | padded weeks, summed | padded date | posted |
|---|---|---|---|---|---|---|
| 0.2 | 1.70 | December 7, 2026 | 2 times | 3.41 | December 19, 2026 | end of December 2026 |
| 0.3 | 7.48 | January 29, 2027 | 3 times | 25.85 | May 25, 2027 | end of May 2027 |
| 0.4 | 4.26 | February 28, 2027 | 3 times | 38.63 | August 23, 2027 | end of August 2027 |
| 0.5 | 2.63 | March 18, 2027 | 4 times | 49.15 | November 5, 2027 | end of November 2027 |
| 1.0 | 5.30 | April 24, 2027 | 4 times | 70.33 | April 1, 2028 | end of April 2028 |

## Why padded this much

- **Milestones grow.** The log importers took 27 increments where two were planned.
- **The queue grows.** In November, planning reviews added at least six milestones ahead of 0.3:
  descent rates ([ADR-338](../decisions/0338-the-2026-11-11-review-descent-rates-held-to-the-logs-predictions-first.md)),
  landing points ([ADR-347](../decisions/0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md)),
  speed off the rail ([ADR-352](../decisions/0352-the-2026-11-16-review-speed-off-the-rail-held-to-accelerometer-logs-predictions-first.md)),
  drag in the coast ([ADR-360](../decisions/0360-the-2026-11-19-review-drag-in-the-coast-held-to-accelerometer-logs-predictions-first.md)),
  the next flight from the last one's log
  ([ADR-370](../decisions/0370-the-2026-11-22-review-the-next-flight-predicted-from-the-last-ones-log.md))
  and time to apogee
  ([ADR-380](../decisions/0380-the-2026-11-26-review-time-to-apogee-held-to-the-logs-predictions-first.md)).
  That last one, 2 to 5 increments ahead of 0.3 and so tripled, adds about 0.8 week to every
  release from 0.3 on. Work finished since November 24 took off about 0.9 week, and this estimate
  starts two days later. So 1.0's padded date moved one day, from March 31 to April 1, 2028, and
  the rounding carried its posted month into April. At the pace unrounded, 13.52 a week, the same
  arithmetic gives March 31.
- **Some targets may not be met.** The fit's intervals (M7.3g) didn't pass their test again: M7.3g
  ended with its gap recorded, and release 0.2 goes ahead without it
  ([ADR-385](../decisions/0385-m7-3g2-a-second-rule-passes-every-row-and-fails-a-class.md)).
  Release 0.3 needs HPR Sim's mean apogee error on the held-out flights below OpenRocket's
  ([M10.3](../ROADMAP.md#phase-8-releases-adr-162)). On the private collection, HPR Sim's apogees miss by
  12.10% on average (mean absolute) and OpenRocket's by 11.26% ([the plan](../plan.md#how-feasible-each-goal-is)).
  Either one held back would mean a decision record and more work.
- **The app is a new kind of work:** a user-interface choice first, then an acceptance pass on
  each of its milestones.

## Keeping them current

Each planning review rechecks these estimates by this method, on the queue and the pace of the
day, and moves a slipping one later on the plan page, and in README's status line for the next
release, saying why.
