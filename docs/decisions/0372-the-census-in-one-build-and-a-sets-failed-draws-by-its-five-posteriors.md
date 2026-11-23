# ADR-372: The census in one build, and a set's failed draws by its five worst posteriors (2026-11-22)

- **Status:** accepted; fixes [#1602](https://github.com/nrdptel/fusionspace-eridanus/issues/1602), the nightly run's two failed checks. Amends how the synthetic census is written ([ADR-280][adr-280]) and how the fit's failed draws are held on another platform ([ADR-316][adr-316] §7)
- **Summary:** One of set A's readings sits on an interval's edge: the dev build and Windows on ARM count it on one side, the optimized build on four other platforms on the other. The synthetic census is now flown, written and checked in the optimized `report-check` build: byte for byte on macOS, where it is written, and elsewhere with an interval test's counts within one flight and the bands beside them following. On a platform other than the one that wrote the fit's report, a set's failed posterior draws may now differ by the failures of its five posteriors that failed most, not by 5 draws: one posterior can fail many of its draws while most fail none.

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
  x86-64 that build counted 64, 313 and 51, and their checks failed. Six numbers of the JSON
  differed, those two counts and the two bands beside them, and one line of the page; the
  failure stopped those four jobs before sets B and calibration. On Windows on ARM the
  optimized build counted 312, as the dev build had, and it reproduced the whole census. On one
  machine and one tree, the dev build's check passed and the optimized build's failed with the
  nightly run's exact difference. The two builds also print 15 numbers of the slice
  differently, in 3 of its 202 flights: 5 by one in the last digit, and the largest, an
  apogee-speed share, reads 0.017 in one build and −0.029 in the other. Their words are the
  same. This is not a physics change. An optimizer, or another platform's math library, may
  compute a value a last bit differently; the workspace's manifest already says so for other
  reports. Carried through a simulated flight and its log read again, that moves a reading, and
  on an interval's edge it flips a count.
- **The fit.** The nightly run's 16 Linux jobs put the fit's report together. Set B's failed
  draws were 298, where the report, written on macOS, says 281. A count may differ by 5 on
  another platform ([ADR-316][adr-316] §7). Part 8 was fitted again on macOS and its fits
  compared with Linux's flight by flight: their failed draws differ on one flight, set B's
  flight 3328, a log with a track. Its posterior failed 12 draws on macOS and 31 on Linux, and
  had 4.7 and 3.6 effective draws, far under the 100 a posterior needs. Part 1's failed draws
  agreed on every flight.

The rule that failed draws stand 5 apart came from a measurement on 2026-11-03: CI's Linux runner
kept every count within 1 of macOS's then. Its reason, written beside the rule, was that "a failed
draw is one draw's own event, not a flight's 500". Flight 3328 shows that this doesn't carry
across platforms. A posterior's draws are made around its flight's least-squares fit
([ADR-317](0317-m7-3c-in-two-the-sampler-then-the-test.md)), and a draw fails where its flight
can't be flown or its weight isn't finite, so one posterior can fail many draws while most fail
none. Another platform's slightly different fit moves where the draws land, and so which fail:
flight 3328's residual is 14.00 m RMS on macOS and 13.95 m on Linux. On the Linux parts, one
posterior of set B failed 31 draws and another 16; most failed none.

**Decision.**

1. **The census has one build: `report-check`.** It is the optimized build with the dev
   profile's debug assertions and overflow checks kept, so a flight that trips one still
   counts as not flown. The census, its page and its slice are flown, written and checked
   there. `cargo xtask synthetic`, run from any other build, runs itself again in that build
   with the same arguments, so the pages that name the command stay true, and the gate's slice
   check runs there too. CI's census jobs already ran there. A build script hands the code the
   build's optimization level (`XTASK_OPT_LEVEL`); the census's build is optimized with debug
   assertions on.
2. **The census is written again in that build, on macOS.** Its counts are the ones four of the
   nightly run's five platforms made. The slice's 15 numbers above take that build's values,
   and its words don't change.
3. **Byte for byte on macOS; one flight elsewhere.** The nightly `census` job holds the census
   byte for byte on macOS, as before. On the other four platforms it runs `synthetic --check
   --other-platform`:
   - an interval test's counts of truths below, inside and above it may each differ by one
     flight;
   - where they differ, the bands beside them move with them: each is those counts' 99.9%
     Clopper–Pearson band, made from them by the same code that macOS holds exactly, and one
     flight in a small cell moves its band far more than any fixed share would allow;
   - every other number, word, field and length is held exactly, as all five platforms agreed
     on every other number they checked that night.

   The committed page must still render from the committed JSON, and the slice stays held in
   its words there, as before. One flight is what was measured, as in [ADR-369][adr-369]: a
   second flip on one count is new, and the check should say so. Each platform still runs
   every test of the census itself: a reading past its bound, an interval that fails, or a
   probe that passes fails there whatever the committed numbers say.
4. **A set's failed draws may stand off by its five worst posteriors.** On another platform, a
   set's failed draws may differ from the committed report's by the sum of the failed draws of
   the 5 posteriors that failed most, read from the fits being checked, and never by less than
   5. The room is the five flights that every other count gets, measured in what five flights'
   posteriors can fail. On the nightly run's Linux parts the room is 17 draws for set A and 68
   for set B; set A's total moved 1 and set B's 17.
5. **Nothing else moves in the fit.** The draws made keep their room of 5 posteriors' 500
   draws. Every other count keeps 5 flights, and every figure keeps 5% or 0.002. The platform
   that writes the report still holds it byte for byte. The report's numbers and page don't
   change.

**Why not the alternatives.**

- *Keep every platform byte for byte, and write the census on the platform most agree with.*
  Windows on ARM would then fail each night, and the next reading on an edge could split the
  platforms another way.
- *Count a truth within some margin of an interval's edge as inside.* The builds move a
  reading by more than a last bit (the slice's apogee-speed share above moves by 0.046 of its
  bound), so no margin is known to be enough. And counting it inside flatters an interval.
- *Hold the census's figures within 5% or 0.002, as the fit's.* The bands of a small cell move
  far more with one flight (0 of 820 truths against 1 moves a band's upper end by 24%), and the
  other figures matched exactly, so a share would let a real drift through for nothing.
- *A larger fixed room for failed draws.* Any fixed number would be a guess. The room above
  comes from the data the check is reading.
- *Leave out the failed draws of posteriors with too few effective draws.* On the Linux parts
  they hold 198 of the 318 failed draws in set B's fits, so the count would drop most of B's
  failures.

**What this does not decide.**

- Why flight 3328's draws fail so much more on one platform, and whether the sampler should
  draw more for a flight with so few effective draws. The report already counts the 722
  posteriors of set B with too few.
- The room comes from the platform being checked. A rise in failures confined to five
  posteriors passes whatever its size, and a fall bigger than the checked platform's five
  worst fails though it is just as much a move of five posteriors. Storing each set's five
  worst in the report would make the room two-sided; that waits for the next time the fit is
  written again.
- The two logs' census (`two-logs`), which still passes in both builds. It keeps writing in
  the dev build until a check shows the same flip
  ([#1608](https://github.com/nrdptel/fusionspace-eridanus/issues/1608)).

**Consequences.**

- `cargo xtask synthetic` from another build prints the command it runs in `report-check` and
  then runs it. The first run builds that profile, which the gate's fit slice already builds.
  Started that way, a build that still isn't the census's refuses instead of starting itself
  again.
- `synthetic --check --other-platform` prints how many numbers differ.
  `another_platforms_census_stands_one_flight_off_at_most` holds its rule: Windows on ARM's
  case passes, and so does a small cell's band moving far with its counts; two flights, a band
  moved while its counts didn't, the readings given, any other count, figure, seed, word or
  field moved fails. `ci_checks_every_set_on_every_platform` fails if macOS is held as another
  platform or the other platforms byte for byte.
- `fit --check --other-platform` prints the most a set's failed draws moved beside each set's
  room. `another_platform_stands_a_few_flights_off_at_most` holds the room: in a room of 68, a
  set's total 68 off passes and 69 off fails, a set with no room has the flights' 5, and a set's
  other counts keep the flights' 5. `the_failed_draws_room_is_five_posteriors_failures` holds
  how the room is summed.
