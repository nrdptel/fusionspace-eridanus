# ADR-381: M7.3f, the fit on the command line and in Python (2026-11-26)

- **Status:** accepted; carries out [M7.3f][m7-3f] ([ADR-377][adr-377]); changes no number the fit's reports hold
- **Summary:** `hpr fit` and Python's `FlightLog.fit` run one library function, `hpr::fit::fit`, so both give the same numbers. It fits a design's simulation to its log by least squares and gives each parameter with its 90% interval or why it has none, what the fit was told before the log, and each reading's source and provenance. The fitted flight's known-issue warnings come with it, and the drag scale's interval prints with [ADR-368][adr-368]'s caveat, read from the same list `cargo xtask fit` enforces. The stated ranges are fixed, staged flights are refused, and no threshold calls a fit good or bad.

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
   it. A test holds the Python dict to `hpr fit --json`, number for number, on a committed log and
   on a public flight.
2. **The stated ranges are the calibration's, and nobody can change them.** The intervals are the
   fit's standard errors times shares sized on synthetic flights that were flown with these stated
   ranges ([ADR-257][adr-257] §1). An interval from other ranges would be one nothing has tested.
   So the command has no option for them. The one stated value that follows the launch is the
   wind's mean: the wind given, averaged over the stated climb in 100 levels.
3. **Each row says what the fit was told.** The fit will bend a scale far from its stated value to
   pass a log. In a test, a two-stage design fitted a smaller rocket's log to 1.7 m RMS with an
   impulse scale of 0.663, ten stated deviations low. A threshold on that would be a verdict, which
   the guardrails rule out. Each row prints the stated mean and deviation beside the fitted value,
   and the page says how to read the two together.
4. **A parameter with no tested interval states none.** The burn time's scale is moved by the fit
   but its interval was never tested, so it prints "none: not asked of the fit". On a log of height
   alone the wind's interval prints "none: a log of height alone can't tell the wind". Both still
   print their fitted values, since they are part of the flight that was fitted.
5. **"Each reading's provenance" means what the fit took from the log.** Those are the liftoff and
   the apogee, which bound the samples fitted, the height track and the horizontal track if the log
   has one. Each prints its source and whether it was measured or derived, in `hpr analyze`'s
   words. The other readings of a log, such as its top speed, aren't inputs to the fit and aren't
   printed.
6. **The warnings are the fitted flight's own.** After the fit, the flight at the fitted parameters
   is flown whole, recovery included, and its `issue_warnings` are the fit's. That is every known
   issue the flight meets, a superset of ADR-163 §4's nine (#87, #120, #121, #222, #172, #67, #70,
   #72 and #68). A library test builds four shapes that between them raise all nine and pins each
   fit's list.
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
   to fit, a flight that separates into stages, a launch the simulator refuses, and a log no flight
   of the design passes through. The last uses the library's existing cut: a residual above a tenth
   of the heights' spread.
10. **A new trust kind, `fitted`.** The note says a scale is one flight's best factor, that the
    simulation's own errors go into the scales, that the intervals were tested on synthetic flights
    only, and that the RSO decides. It links [the new page][page].

**Evidence.** A fit of a log of the design's own simulated flight, flown at a drag scale of 1.15,
returns an interval that holds 1.15. On NDRT's 2020 public flight the command fits a drag scale
within 0.1 of the 0.71 that `validation/reports/from-boost.json` holds for the same flight, and the
test asserts it. Three mutation probes each fail the tests that pin them: a fit with no caveat, a
caveat with its lean swapped, and a fit that drops its warnings.

**What this leaves open.** No real flight has checked the intervals against something measured
apart from the log. The evidence register's row on the fit ([ADR-278][adr-278]) already says so,
and [issue #1104][i1104] asks for such a flight. The public-flight tests need a checkout that CI
doesn't have, so in CI the command runs on the committed invented log and the local gate runs the
public flight. The Bayesian option and the prediction from the boost stay in the library.
