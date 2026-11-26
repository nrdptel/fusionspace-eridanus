# ADR-381: M7.3f, the fit on the command line and in Python (2026-11-26)

- **Status:** accepted; carries out [M7.3f][m7-3f] ([ADR-377][adr-377]); changes no number the fit's reports hold
- **Summary:** `hpr fit` and Python's `FlightLog.fit` run one library function, `hpr::fit::fit`, so both give the same numbers. It fits a design's simulation to its log by least squares and gives each parameter with its 90% interval or why it has none, what the fit was told, and each reading's provenance. The fitted flight's known-issue warnings come with it, and the drag scale's interval carries [ADR-368][adr-368]'s caveat. A log of height alone is told at least the wind its intervals were sized in, a value outside the tested range is marked, a track's wind carries its own caveat, a flight that separates is refused, and no threshold calls a fit good or bad.

[m7-3f]: ../decisions-and-roadmap.md#m7-3f
[m7-3g]: ../decisions-and-roadmap.md#m7-3g
[adr-163]: 0163-the-second-pass-products-and-priorities.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-278]: 0278-an-evidence-register-for-every-claim.md
[adr-368]: 0368-the-fits-drag-interval-fails-set-bs-test-on-the-default-flight-recorded.md
[adr-377]: 0377-no-release-waits-on-users-and-release-0-2-comes-next.md
[page]: ../fitting-a-flight.md
[fit-page]: ../physics/flight-fit.md
[i1104]: https://github.com/nrdptel/fusionspace-eridanus/issues/1104
[i1108]: https://github.com/nrdptel/fusionspace-eridanus/issues/1108
[i1646]: https://github.com/nrdptel/fusionspace-eridanus/issues/1646

**Context.** The least-squares fit ([its page][fit-page]) lived in `hpr_forensics`, reached only by
the validation reports' own code. ADR-377 moved it to the command line and Python for release 0.2,
ahead of the work that makes its failing interval pass ([M7.3g][m7-3g]). So it ships with a known
gap: on the set of synthetic flights flown with a model unlike the fit's own, 80 of 1,000 true drag
scales lie below their intervals, where the test passes at 74 or fewer (ADR-368). A fitted drag
that reads high predicts a flight that reads low, the side that flatters a field's ceiling. The
milestone's *done when* asks that the caveat and the warnings of [ADR-163][adr-163] §4's issues
travel with every fit.

**Decisions.**

1. **One function, two surfaces.** `hpr::fit::fit(launch, log, height_read_as)` takes the launch as
   a `FlightBuilder` and the log as the reader's record. `hpr fit` and `FlightLog.fit` both call
   it. A test holds the Python dict's numbers to those of `hpr fit --json` with `==`, on a
   committed log and on a public flight.
2. **The stated ranges are the calibration's, and nobody can change them.** The intervals are the
   fit's standard errors times shares sized on synthetic flights that were flown with these stated
   ranges ([ADR-257][adr-257] §1). An interval from other ranges would be one nothing has tested.
   So the command has no option for them. The one stated value that follows the launch is the
   wind's mean: the wind given, averaged over the stated climb in 100 levels. On a log of height
   alone that mean has a floor. The synthetic flights' fits of such logs were told a wind of 20 mph
   over √3, 5.16 m/s, toward the east ([the method's page][fit-page]). No test covers the
   intervals of a fit told less wind than that. So on such a log a wind slower than 5.16 m/s is
   raised to it along its own direction, calm air to that toward the east, and a stronger wind
   stands. The table says so. A
   log with a track is told the wind given, as its synthetic fits were.
3. **Each row says what the fit was told, and a value outside the tested range is marked.** The
   fit will bend a scale far from its stated value to pass a log. A committed test,
   `fit_marks_a_value_outside_the_range_its_intervals_were_tested_over`, fits a 54 mm design to
   the Level 1 guide rocket's log: it passes to under 2 m RMS with a drag scale above 1.4 and an
   impulse scale below 0.8. A threshold that called such a fit bad would be a verdict, which the
   guardrails rule out. A validity limit isn't one. The intervals were tested on synthetic flights
   whose true values were drawn from fixed ranges (`Asked::drawn` and `BURN_TIME_SCALE_DRAWN`), and
   no test covers them outside those. So each fitted value outside its range prints with a `†` and
   a line that names the range, and the document holds `tested_range` and `outside_tested_range`.
   For the wind the range is the fastest wind drawn, 20 mph. Each row also prints the stated mean
   and deviation beside the fitted value, and the page says how to read the two together.
4. **A parameter with no tested interval states none.** The burn time's scale is moved by the fit
   but its interval was never tested. Its interval prints "none", and a line under the table says
   "The burn time scale states no interval: not asked of the fit." On a log of height alone the
   wind's prints "none" too, with "The wind states no interval: a log of height alone can't tell
   the wind." Both still print their fitted values, since they are part of the flight that was
   fitted. Every printed interval and bound is rounded outward, so none claims less than it holds.
5. **"Each reading's provenance" means what the fit took from the log.** Those are the liftoff and
   the apogee, which set the window fitted (from a second before the liftoff to the apogee), the
   height track and the horizontal track if the log has one. Each prints its source and whether it was measured or derived, in `hpr analyze`'s
   words. The other readings of a log, such as its top speed, aren't inputs to the fit and aren't
   printed.
6. **The warnings are the fitted flight's own.** After the fit, the flight at the fitted parameters
   is flown whole, recovery included, and its `issue_warnings` are the fit's. That is every known
   issue the flight meets, a superset of ADR-163 §4's nine (#87, #120, #121, #222, #172, #67, #70,
   #72 and #68). A library test builds four shapes that between them raise all nine and pins each
   fit's list. It also fits a log flown with half the drag: the fitted flight passes Mach 0.6 and
   carries an issue the flight as stated doesn't. The two apogees the fit reports, as stated and
   as fitted, are the climb's as the fit flies it, with nothing open.
7. **The caveat is read from the list the check enforces.** `KNOWN_GAPS` moved from `xtask` into
   `hpr_forensics::synthetic::fits`, and the caveat on an interval is built from it. `cargo xtask
   fit` reads the same list. When M7.3g takes set B's drag scale off the list, the caveat stops
   printing and the tests that pin it fail until someone removes them on purpose. The caveat prints
   on standard output under the intervals, and a `*` marks the interval. It says which way the
   interval errs and links [issue #1108][i1108]. A test holds its counts to the committed report.
8. **The fit flies the climb with nothing open.** A recovery device that opens before apogee would
   change the climb the fit differentiates through. A flight whose parachute really did open early
   has that drag put into its drag scale, and a note says so.
9. **What can't be fitted is refused with its reason.** That is a log with no apogee or no samples
   to fit, a flight that separates in the air, a launch the simulator refuses, and a log no flight
   of the design passes through. A staged design flown as one stack doesn't separate, and is
   fitted. The last uses the library's existing cut: a residual above a tenth
   of the heights' spread.
10. **A new trust kind, `fitted`.** The note says a scale is one flight's best factor, that the
    simulation's own errors go into the scales, that the intervals were tested on synthetic flights
    only, and that the RSO decides. It links [the new page][page].

11. **The page's examples are held exactly on one platform and by their words on the others.**
    The fit's search stops on a tolerance, and its last digits differ from one platform to
    another. The platforms' math libraries are the likely cause, which I haven't isolated.
    On this change's first run in CI, Windows printed a burn time scale of 1.036 where macOS
    printed 1.037, and Linux a wind of 0.100 m/s where macOS printed 0.101. The outputs on
    [the page][page] are written on macOS on Apple silicon and held character for character there,
    which CI's macOS runners are. On Linux and Windows the checks put a `#` for each number and one
    space for each run of spaces before they compare, and the command-line check leaves out the
    `warning:` lines, since a known issue's warning turns on at a Mach number and the example's
    fitted flight tops out near Mach 0.8. The fit's own reports are already held this way
    (`cargo xtask fit --other-platform`). The tests that hold the Python dict to the command's
    document still compare with `==` on every platform, since both are built and run on one
    machine.

12. **The wind of a log with a track carries a caveat of its own.** A review found, and a
    committed test reproduces
    (`fit_can_settle_far_from_the_wind_a_track_was_flown_in`), that the fitted wind can settle
    far from the flight's, with a narrow interval about it. The 54 mm synthetic design's own
    flight in a 9 m/s wind, fitted from calm air, comes out as a wind the other way. Its interval
    is under 2 m/s wide and misses the true wind, and the heights' residual lies within 0.5 m of
    the residual of the fit told the true wind. Told the true wind, each component of the fitted
    wind stays within 1 m/s of it. The cause isn't isolated, and no test says how near the wind told has to be. The
    wind's interval still passes its test on the synthetic flights, so that test doesn't see it.
    The search is the one the intervals were sized with, and changing it here would leave them
    untested. So every fit of a log with a track prints a caveat that says what can happen and to
    give the day's wind, [the page][page] explains it, and [issue #1646][i1646] holds the fix,
    labeled to come first.

**Evidence.** A fit of a log of the design's own simulated flight, flown at a drag scale of 1.15,
returns an interval that holds 1.15. On NDRT's 2020 public flight the command fits a drag scale
within 0.1 of the 0.71 that `validation/reports/from-boost.json` holds for that flight, and the
test asserts it. The two aren't one fit. Both fit the whole climb. The report's flies ERA5's air
and is told ERA5's wind, and the command's flies standard air and is told the floor's wind, so the
agreement is of the lean, not of the inputs. Thirteen mutation probes each fail the tests that pin them. Seven came before review: a
fit with no caveat, a caveat with its lean swapped, a fit that drops its warnings, a wind given but
not told to the fit, a staged flight not refused, an apogee's provenance changed, and the two
reasons for no interval swapped. Six came with the review's fixes: no floor on the wind of a log of
height alone, no mark on a value outside its tested range, intervals rounded to nearest in the
library and in the command, the warnings of the flight as stated, and the table's line on that
wind without its "at least".

**What this leaves open.** No real flight has checked the intervals against something measured
apart from the log. The evidence register's row on the fit ([ADR-278][adr-278]) already says so,
and [issue #1104][i1104] asks for such a flight. The public-flight tests need a checkout that CI
doesn't have, so in CI the command runs on the committed invented log and the local gate runs the
public flight. The Bayesian option and the prediction from the boost stay in the library.
