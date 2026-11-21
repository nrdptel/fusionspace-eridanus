# ADR-365: A smoothed track past Earth's escape speed is withheld as diverged (2026-11-21)

- **Status:** accepted; adds a refusal to the smoother of [ADR-300][adr-300] and a count to its check's mutation; changes no rule of the filter, and no interval of a track that is still given
- **Summary:** The nightly census failed on every platform ([#1590][issue]): one synthetic flight's track, smoothed with its GNSS receiver trusted from liftoff, ran away, and the report held its standard error, 2,480,000 m, to three figures that no two builds agree on. The smoother now withholds as diverged any track whose vertical speed passes 11,186 m/s, Earth's escape speed. The mutation's diverged logs are counted and leave its cells, and it still fails on the rest. The same bound withholds 3 of the 1,058 real sample records, whose runaway tracks `hpr analyze` printed until now.

[adr-300]: 0300-the-smoother-and-its-check.md
[adr-362]: 0362-base-drag-behind-a-boattail-below-mach-0-8-held-to-the-hermes-a-3a.md
[issue]: https://github.com/nrdptel/fusionspace-eridanus/issues/1590
[bound-issue]: https://github.com/nrdptel/fusionspace-eridanus/issues/1073
[report]: ../../validation/reports/synthetic-flights.md#trusting-the-gnss-receiver-from-liftoff
[survey]: ../../validation/reports/log-import.md#the-smoother
[page]: ../physics/smoother.md#a-track-that-runs-away

**Context.** The smoother's check ([ADR-300][adr-300]) smooths each synthetic flight's log and
holds the 90% intervals to the flight's truth. It also smooths each log with a GNSS track a
second time, the receiver trusted from liftoff: a mutation that must fail. The report holds
the mutation's cells too, each with its largest standard error, and the nightly run flies
every flight again on five platforms and compares to three figures.

On 2026-11-21 that run failed on all of them. Two cells of set A's mutation differed, the
drawn moment's height and speed:

| build | largest standard error, height | speed |
|---|---:|---:|
| committed (macOS, the dev profile) | 2,480,000 m | 10,100,000 m/s |
| nightly, macOS and Windows | 2,460,000 m | 10,000,000 m/s |
| nightly, Linux, x86-64 and ARM | 2,300,000 m | 9,380,000 m/s |

Before [ADR-362][adr-362]'s drag change moved the synthetic flights, the same two cells held
247 m and 827 m/s. Its pull request's checks fly a slice of the flights, not the whole census,
so the night's run was the first to fly these cells on another build.

One log makes both numbers: set A's flight 282, trusted from liftoff. Measured on this
change's build (macOS, the nightly's profile), its smoothed speed reaches 3.7 × 10¹⁰ m/s and
its height 1.9 × 10¹⁰ m, on a flight that tops out near 3 km, and the pass back
couldn't solve 219 of its steps. The filter's own forward estimate holds those speeds before
the pass back touches it, so the filter runs away, not the pass back.

The cause is the gate's take-back with two sensors on one quantity. A sensor rejected for a
second is taken back, and the height, speed and acceleration are widened by its gap
([ADR-300][adr-300], decision 9). That reads the gap as motion the filter missed. Trusted from
liftoff, the receiver is taken back as the barometer is. When the two disagree past their
gates, each take-back moves the speed and acceleration toward one sensor, the other is
rejected for the next second and taken back across a wider gap, and the gap grows at each
turn. As shipped, the receiver is never taken back: two rejections lose it its trust.

A probe of every synthetic log on the same build sized it:

| rule | logs | top smoothed speed, any log | logs past 1,500 m/s |
|---|---:|---:|---:|
| as shipped: tuning, A and B, whole and cut at liftoff | 3,672 | 664 m/s | 0 |
| trusted from liftoff, tuning | 171 | 9,110 m/s | 7 |
| trusted from liftoff, A | 820 | 3.7 × 10¹⁰ m/s | 24 |
| trusted from liftoff, B | 845 | 219,000 m/s | 22 |

The mutation's logs count every smoothed flight of the set; only those with a GNSS track are
smoothed again (33, 185 and 186). So the mutation runs away to some degree on about one log
in eight of those it touches, and no synthetic log does as shipped. The mutation's top speeds
have no gap in them: they run from a few thousand m/s up.

**Decision.**

1. **A track whose vertical speed passes 11,186 m/s anywhere is withheld,**
   `Reason::Diverged`, as a track that leaves the finite numbers already is. The bound is
   Earth's escape speed at its surface, `√(2 GM/R)`: 11.186 km/s in NASA's *Earth Fact Sheet*
   (D. R. Williams, NASA Space Science Data Coordinated Archive), with `GM` =
   398,600 km³/s² and the mean radius 6,371.0 km; a test holds the constant to that formula.
   It is 17 times the fastest shipped synthetic track and nine times Mach 3.5 at sea level,
   the top of HPR Sim's envelope. No rocket that comes back flew it. `hpr analyze` prints
   `withheld` and the reason on its `smoother` row.
2. **It bounds a runaway. It doesn't certify a track.** A track that is wrong at 5,000 m/s is
   still given. The intervals, the health card and the reading's bound do that work, as
   before.
3. **The mutation's diverged logs are counted and leave its cells.** The report prints how
   many of the logs smoothed again were withheld: 0 of the tuning set's 33, 11 of A's 185 and
   6 of B's 186. Leaving them out can only make the mutation look better, which is the hard
   side for the claim that the check catches it: the check still demands that the mutation
   fail on A and on B, and it does. At apogee 143 of A's 809 truths and 144 of B's 839 lie
   below the height's interval ([the report][report]).
4. **The filter's rules don't change.** Three other fixes were weighed and left:
   - *Compare the mutation's largest standard error to fewer figures, or not at all.* That
     edits a check to pass it, and leaves the smoother handing out a track 10¹⁰ m high.
   - *Don't take the receiver back under the mutation.* The mutation exists to show what
     trusting the receiver from liftoff does under the smoother's own rules
     ([ADR-300][adr-300], decision 9). Softening it changes what the check is shown to catch.
   - *Cap what a take-back widens.* That is a rule of the shipped filter. No source gives the
     cap, and every interval would need its refit and its check again. The traced cause on
     real logs should come first (decision 5).
5. **Three real logs ran away as shipped, and are now withheld.** The survey of the sample
   logs smooths every record ([the survey][survey]). With the bound, 3 of its 1,058 records
   are withheld as diverged: one `altos-csv` log and two `held-table` logs. The smoothed count
   falls from 549 to 546, and the smoothed apogees past the reading's bound from 32 to 30, so
   the card had marked two of the three and not the third. The shipped rule takes no receiver
   back, so these ran away by another road. It isn't traced. That work belongs to the open
   issue on smoothed apogees past the reading's bound ([#1073][bound-issue]), which now
   carries these three.

**Which way it errs.** A refusal is the safe side: the reader loses a track and keeps the
readings, which don't come from the smoother. The bound can't refuse a real flight inside the
envelope. It can let a wrong track through, which decision 2 says.

**What isn't settled.**

- The mutation's top speeds have no gap, so a log near the bound could be withheld on one
  build and given on another. On this build the nearest are 1.3% below it (set A, 11,040 m/s)
  and 6.8% above (set B). The bound is a physical constant and wasn't moved to clear them.
  If one crosses, the census fails on a count of diverged logs and says so. The runaway that
  failed the night's run had grown by eight orders of magnitude; a log near the bound has
  grown by one or two, and both of this Mac's builds, the dev profile and the nightly's,
  count the same logs.
- The real logs' road to a runaway ([#1073][bound-issue]).

**Consequences.**

- `validation/reports/synthetic-flights.*` are regenerated. Only the mutation's cells and its
  new count move. The check's own cells, the logs cut at liftoff and every interval are
  byte for byte what they were. Two flights of the slice every pull request flies are among
  the mutation's diverged, so each platform's checks now fly the refusal.
- `validation/reports/log-import.*` are regenerated: three records move from smoothed to
  withheld.
- [The smoother's page][page] explains the bound, with the invented flight its test flies.
  The `analyze` schemas carry the reason's new wording.
