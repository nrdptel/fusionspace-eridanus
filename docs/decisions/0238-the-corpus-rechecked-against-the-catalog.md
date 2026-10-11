# ADR-238: The corpus rechecked against the collection's catalog (2026-10-11)

- **Status:** accepted; closes M2.3c5
- **Summary:** Each of the 121 flags the private collection's catalog raised on 68 of the 83 flights HPR Sim compares with their logs was settled against the flight's log and sources. Five changed the manifest: one launch day, one logged apogee, two flights left out and the catalog's tier recorded for every flight. HPR Sim's mean apogee error over the logs moved from +9.83% (55 flights) to +8.73% (53), OpenRocket's from +9.00% to +7.79%. Nearly all of that is one flight left out because it tumbled through its climb. The report adds the errors over the flights the catalog keeps in tier A: +7.84% over 44.

**Context.** M2.3c5's *done when*: "each corpus flight the collection's catalog flags (68 of 83)
is settled against its log and evidence, the manifest fixed and the fixture report regenerated,
saying what moved." [ADR-184](0184-logged-apogees-of-the-private-collection.md) read the 83 flights'
free-text rows into a manifest by hand, split over seven readers, checked for form but not
flight by flight. Later, the collection's own catalog (merged at `7268bcd0`, the pin since
[ADR-237](0237-one-fixture-repository.md)) gave each flight a card from a second reading of
its sources. It compared those cards with the manifest in a table of 121 rows on 68 flights, by
kind: an anomaly the manifest neither flags nor leaves out (50), a design HPR Sim 0.1.0 refuses
in a default run (28), a flight the catalog puts below tier A (15), a logged apogee more than 2%
from the catalog's mechanical log peak (10), a disagreement on early deployment (8), and a launch
day that differs or is doubted (10).

**Decisions.**

1. **One rule for leaving a flight out, as ADR-184 wrote it.** A flight leaves the errors only
   when its log is no fair test of a simulation of the design as drawn: a motor anomaly, active
   control in the climb, a log that lost its apogee, a design that is not the flown rocket, or
   another reason of that weight. A simulator's miss, spin, coning, weathercocking, a gusty wind
   or a recovery fault after apogee stays in: those are the comparison's to show. That rule
   keeps 48 of the 50 anomaly rows; one leaves a flight out and one corrects an apogee (below).
   For the other sensor faults, the manifest's apogee already comes from another altimeter or
   channel.
2. **Two flights left out.**
   - One tumbled through its climb. The flier says so on its launch page and blames warped
     fins. The design as drawn holds over 3 calibres of static margin in HPR Sim, so no simulation
     of it can show the build's tumble. Its error was the set's largest, past +30%. It is under
     "another reason".
   - The other's posted `.ork` is the flier's later rebuild of the rocket, with the change stated.
     HPR Sim and OpenRocket read it 7% heavier than the rocket weighed at the pad. It is under "a
     design that is not the flown rocket".
3. **Two corrections.**
   - One launch day moves a day later, to the event's date. The site shows a date-only entry
     that way, and the collection holds that day's ERA5 file, which both simulators now fly.
   - One logged apogee drops 0.23%, to the primary altimeter's highest reading before
     its drogue charge. The second altimeter, aligned in time, is level from 0.6 s before the
     charge and falls from 0.2 s after it. Meanwhile the primary reads higher, so
     those readings are the charge's pressure transient. The manifest had taken them.
4. **The rest kept, each with its reason.**
   - The five day mismatches are cases where the catalog's card kept the collection's
     spreadsheet date and its readers sided with the manifest.
   - One doubted day stays. The case for the day before rests on a page's entry time alone. The
     flier's stated wind fits ERA5's 10 m wind on the manifest's day (17 against 13 mph). That
     day was fetched from the Copernicus Climate Data Store to check.
   - Four early-deployment rows are a main parachute opened at apogee, not a deployment before
     it, so the climb is unchanged. One is a motor charge the flier thinks fired first, with no
     separation before the peak in either of the flight's two logs. Three are a deployment the
     manifest counts and the catalog judged negligible. ADR-184 §6 counts any deployment logged before the peak while climbing,
     and the report gives the errors without them too.
   - The ten apogee rows are the catalog's mechanical peak taking what the manifest avoided: a
     charge transient, corrupt samples, a short export or a misread column. Decision 3's
     corrected apogee came from an anomaly row instead.
   - The 28 refusal rows are a default run's. The harness flies those it can: design-check
     findings as warnings, motor curves from OpenRocket's database. Each flight it can't fly is
     counted under a cause with an open issue, as before.
5. **The catalog's tier, recorded and reported.** Every manifest entry now states the tier the
   catalog gives it: A, or B with one fact of the flight inferred, or C with the design not shown
   to be the rocket flown. `cargo xtask fixture-flights` refuses an entry without one. The report
   counts the compared flights below tier A and adds a third set of errors, over only the flights
   the catalog keeps in tier A. Of the 15 below tier A, 9 are compared, 2 are left out and 4 are
   not flown.

The per-row decisions stay on this machine with the manifest, under the gitignored `corpus/`, as
the collection's license requires.

**Consequences.** Measured on 2026-10-11 (`validation/reports/fixture-flights.md`), step by step
from the 55 flights compared before:

| step | flights | HPR Sim mean | mean absolute | OpenRocket mean | mean absolute |
|---|---:|---:|---:|---:|---:|
| before (ADR-184) | 55 | +9.83% | 13.96% | +9.00% | 13.08% |
| the day and apogee corrected | 55 | +9.82% | 13.96% | +9.00% | 13.08% |
| the tumbling flight left out | 54 | +8.54% | 12.75% | +7.75% | 11.91% |
| the rebuilt design left out (the report) | 53 | +8.73% | 12.95% | +7.79% | 12.02% |

The table's middle rows are worked from the run's per-flight output on this machine; the last
row is the committed report's. Leaving out the tumbling flight is nearly all of the move, and it
flatters both simulators equally. This record names it so a reader can weigh it. The
corrections move the mean by 0.01 points.

- Without the early deployments, over 34 flights, HPR Sim is +3.95% and OpenRocket +3.19%. Over
  the 44 the catalog keeps in tier A, HPR Sim is +7.84% (mean absolute 12.13%) and OpenRocket
  +6.99% (11.28%). HPR Sim against OpenRocket is +0.81%, 52 of 53 within 5%.
- Both codes still over-predict alike, so the gap to the logs is still not a difference between
  them. The [apogee-bias research note](../research/apogee-bias.md) set out where it may come from.
- `hpr sim`'s note on how far to trust a result
  ([ADR-211](0211-how-far-to-trust-a-result.md)) quotes the report, so it now reads 53 flights,
  8.7%, 26 within 10% and 13 low, by at most 20%.

**Left open.** One launch day stays doubted, as recorded. The causes of the flights the harness
can't fly keep their issues (#357 to #364, #181), and the catalog's flags change none. M2.3c2 adds
each log's altitude trace. The catalog's section on design reading problems is M2.3c6's and the
issues', not this record's.
