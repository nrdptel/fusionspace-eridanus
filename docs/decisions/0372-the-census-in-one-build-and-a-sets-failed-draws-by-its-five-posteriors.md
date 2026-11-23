# ADR-372: The census in one build, and a set's failed draws by its five worst posteriors (2026-11-22)

- **Status:** accepted; fixes [#1602](https://github.com/nrdptel/fusionspace-eridanus/issues/1602), the nightly run's two failed checks. Amends how the synthetic census is written ([ADR-280][adr-280]) and how the fit's failed draws are held on another platform ([ADR-316][adr-316] §7)
- **Summary:** One of set A's readings sits on an interval's edge: the dev build and Windows on ARM count it on one side, the optimized build on four other platforms on the other. The synthetic census is now flown, written and checked in the optimized `report-check` build, byte for byte on macOS, where it is written, and elsewhere within one flight on a count and 5% or 0.002 on a figure. On a platform other than the one that wrote the fit's report, a set's failed posterior draws may now differ by the failures of its five posteriors that failed most, not by 5 draws: failures cluster in poorly mixed chains.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-369]: 0369-the-fits-slice-on-another-platform-one-fit-may-stop-otherwise.md
[run]: https://github.com/nrdptel/fusionspace-eridanus/actions/runs/45043078586
[validation]: ../VALIDATION.md#in-ci-and-regenerating-the-references

**Context.** Each night CI flies the two flight censuses whole ([ADR-149][adr-149], amended).
The [run on 2026-11-22][run] failed both.

- **The synthetic census.** Set A's drogue descent rate is read on 428 flights. Under the probe
  that narrows each 90% interval to 0.8 of its width, one of those readings sat on the edge of
  its interval. The census on main had been written by `cargo xtask synthetic`, which runs in
  the dev build. That build counted 64 below, 312 inside and 52 above. The nightly run checks
  the census in the optimized `report-check` build. On macOS, both Linux runners and Windows on
  x86-64 that build counted 64, 313 and 51, and their checks failed. On Windows on ARM it
  counted 312, as the dev build had, and its check passed. On one machine and one tree, the dev
  build's check passed and the optimized build's failed with the nightly run's exact
  difference. The two builds also
  print a few of the slice's numbers a last digit apart (one landing share reads −0.124 in one
  and −0.123 in the other). This is not a physics change. An optimizer, or another platform's
  math library, may compute a value a last bit differently; the workspace's manifest already
  says so for other reports. A reading on an interval's edge then flips a count.
- **The fit.** The nightly run's 16 Linux jobs put the fit's report together. Set B's failed
  draws were 298, where the report, written on macOS, says 281. A count may differ by 5 on
  another platform ([ADR-316][adr-316] §7). Part 8 was fitted again on macOS and its fits
  compared with Linux's flight by flight: they differ on one flight, set B's flight 3328, a
  log with a track. Its posterior failed 12 draws on macOS and 31 on Linux, and had 4.7 and 3.6
  effective draws, far under the 100 a posterior needs. Part 1 agreed on every flight. Fitted
  again whole on macOS, the report came back byte for byte, so it wasn't stale.

The rule that failed draws stand 5 apart came from a measurement on 2026-11-03: CI's Linux runner
kept every count within 1 of macOS's then. Its reason, written beside the rule, was that "a failed
draw is one draw's own event, not a flight's 500". Flight 3328 shows that it isn't. A posterior's
draws follow its chain, and its failures fall where the chain wanders. When the last digits of a
math library move a poorly mixed chain's start, the whole chain lands elsewhere, and its failures
with it. On the Linux parts, one posterior failed 31 draws and another 16; most failed none.

**Decision.**

1. **The census has one build: `report-check`.** It is the optimized build with the dev
   profile's debug assertions and overflow checks kept, so a flight that trips one still
   counts as not flown. The census, its page and its slice are flown, written and checked
   there. `cargo xtask synthetic`, run from the dev build, runs itself again in that build with
   the same arguments, so every page that names the command stays true, and so does the gate's
   slice check. CI's census jobs already ran there. A build script hands the code the build's
   optimization level (`XTASK_OPT_LEVEL`).
2. **The census is written again in that build, on macOS.** Its counts are the ones four of the
   nightly run's five platforms made. The slice's numbers take that build's last digits, and
   its words don't change.
3. **Byte for byte on macOS; within one flight elsewhere.** The nightly `census` job holds the
   census byte for byte on macOS, as before. On the other four platforms it runs `synthetic
   --check --other-platform`: every field, length and word as committed, a count within 1 and a
   figure within 5% of the larger or 0.002, the fit's rule for a figure ([ADR-316][adr-316]
   §7). The committed page must still render from the committed JSON. The slice stays held in
   its words there, as before. One flight is what was measured, as in [ADR-369][adr-369]: a
   second flip on one count is new, and the check should say so. Each platform still runs every
   test of the census itself: a reading past its bound, an interval that fails, or a probe that
   passes fails there whatever the committed numbers say.
4. **A set's failed draws may stand off by its five worst posteriors.** On another platform, a
   set's failed draws may differ from the committed report's by the sum of the failed draws of
   the 5 posteriors that failed most, read from the fits being checked, and never by less than
   5. The room is the five flights that every other count gets, measured in what five flights'
   posteriors can fail. On the nightly run's Linux parts the room is 17 draws for set A and 68
   for set B. The 17 draws set B's total moved fit inside 68.
5. **Nothing else moves.** The draws made keep their room of 5 posteriors' 500 draws. Every
   other count keeps 5 flights, and every figure keeps 5% or 0.002. The platform that writes
   the report still holds it byte for byte. The report's numbers and page don't change.

**Why not the alternatives.**

- *Keep every platform byte for byte, and write the census on the platform most agree with.*
  Windows on ARM would then fail each night, and the next reading on an edge could split the
  platforms another way.
- *Count a truth within some margin of an interval's edge as inside.* The two sides differ by
  a whole flight simulated and read again, not by a last bit, so no margin is known to be
  enough. And counting it inside flatters an interval.
- *A larger fixed room for failed draws.* Any fixed number would be a guess. The room above
  comes from the data the check is reading.
- *Leave the failed draws of poorly mixed posteriors uncounted.* Those posteriors are where the
  failures are, so the count would stop saying anything.

**What this does not decide.** Why flight 3328's posterior mixes so poorly, and whether the
sampler should draw more for such a flight. The report already counts the 722 posteriors of
set B with too few effective draws. Nor the two logs' census (`two-logs`), which still passes
in both builds. It keeps writing in the dev build until a check shows the same flip
([#1608](https://github.com/nrdptel/fusionspace-eridanus/issues/1608)).

**Consequences.**

- `cargo xtask synthetic` from the dev build prints that it runs in `report-check` and then
  does so. The first run builds that profile, which the gate's fit slice already builds.
- `synthetic --check --other-platform` prints how many numbers differ.
  `another_platforms_census_stands_one_flight_off_at_most` holds its room: one flight and 5%
  pass; two flights, a figure 6% off, a word, a field or a count written as a figure fail.
  `ci_checks_every_set_on_every_platform` fails if macOS is held within the room or the other
  platforms byte for byte.
- `fit --check --other-platform` prints each set's room beside the differences.
  `another_platform_stands_a_few_flights_off_at_most` holds the room: in a room of 68, a set's
  total 68 off passes and 69 off fails, and a set with no room has the flights' 5.
  `the_failed_draws_room_is_five_posteriors_failures` holds how the room is summed.
- The nightly run's checks, flown again by hand on this change's branch, pass.
