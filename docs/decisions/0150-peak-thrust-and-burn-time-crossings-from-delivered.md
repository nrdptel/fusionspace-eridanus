# ADR-150: Peak thrust and the burn-time crossings come from delivered samples (2026-10-04)

- **Status:** accepted
- **Summary:** A thrust-curve sample strictly inside a run of three or more equal times is never delivered, so it no longer sets the peak thrust or a 5%-of-peak burn-time crossing; no surveyed ThrustCurve.org file changes

**Context.** [ADR-005](0005-solid-motors-statistics-consumption-grains-file.md) makes equal
consecutive times in a thrust curve a step: the thrust takes the later sample's value from that
time on. A sample strictly inside a run of three or more equal times (`t_{i−1} = t_i = t_{i+1}`)
is then never the thrust at any time, nor its limit from either side. `ThrustCurve::peak_thrust_n`
still took the largest of all samples, and the NFPA 1125 burn window compared all samples with 5%
of that peak (issue #10). With times `0, 1, 1, 1, 2` s and thrusts `10, 10, 1000, 10, 0` N the
curve holds 15 N·s and never exceeds 10 N, but the skipped 1000 N sample made the threshold 50 N
and `ThrustCurve::new` refused the curve for having no burn time. A milder skipped spike inflates
the peak and shortens the burn window, so the average thrust comes out high.

ADR-005 left this alone because hpr matched ThrustCurve.org's own statistics code
(`simulate/analyze/analyze.js`, ISC) on the bundle. That code averages points less than 50 µs
apart instead of keeping steps, so it says nothing about a sample hpr's own curve skips.

**Decision.**

1. A sample is **delivered** when it starts or ends a segment of non-zero width
   (`t_{i−1} < t_i` or `t_i < t_{i+1}`). This matches `ThrustCurve::thrust_n` exactly: at a time
   `t` inside the curve it interpolates on the segment that starts at the last sample at or
   before `t`, which has non-zero width, so a sample that starts a segment is returned at its own
   instant and one that ends a segment is the limit as time rises to its instant. The thrust's
   least upper bound over time is therefore the largest delivered sample.
2. `peak_thrust_n` is the largest delivered sample, and `burn_window_s` finds the first and last
   delivered samples at or above 5% of that peak. The segment before the first, and after the
   last, either has non-zero width, so the crossing lies on it, or is a step, where the crossing is
   the step's instant.
3. Total impulse is unchanged: zero-width segments add nothing to the trapezoid sum.
4. The rejection test that refused a spike with a 1% trickle after it encoded the defect: that
   curve delivers a 1 N trickle and now builds, with a 1 N peak. A curve whose only thrust is a
   skipped sample is still refused, for having no impulse.

**Evidence.**

- Tests in `hpr_motor::curve`: the issue's example builds with a 10 N peak and a 0 to 1.95 s
  window; a skipped 400 N sample on a 100 N triangle leaves the triangle's peak and window; skipped
  samples at ignition and at burnout do not move the crossings. A property test on curves with
  steps holds the peak to the largest value `thrust_n` takes on a dense grid, at every sample time
  and just before it (within 1e-12 above and 1e-9 below), and another holds every time the
  evaluator is above the threshold inside the window. Reverting the peak to all samples fails four
  tests, reverting the crossings to all samples fails two, and dropping the segment-end half of the
  definition fails eight.
- The 32 bundled curves still match `analyze.js` to 1.8e-15
  (`catalog::tests::every_bundled_curve_matches_thrustcurve_statistics_code`); none has a run of
  three equal times.
- The surveyed ThrustCurve.org solid-motor files (1712, of which 1710 read; cached under the
  gitignored `refs/samples/thrustcurve/`) were run through hpr before and after the change, and
  through the pinned `analyze.js` on the API's samples. Three files hold a run of three equal times;
  in none does the skipped sample set the peak, and where one was the first sample above the
  threshold it sits on a step, so the crossing instant is the same. A sample is also skipped when
  it opens a step at the curve's start or closes one at its end; a reviewer's recount, also not
  committed, found a positive one at the start in 1 file and at the end in 3, and none moves a
  statistic. So no file changes in total
  impulse, peak thrust, burn window, burn time or average thrust. The 17 files that differ from
  `analyze.js` by more than 1e-9 relative are the same 17 before and after, with the same largest
  differences: 1.13% in average thrust, 0.61% in burn time, 0.59% in total impulse and 21.7% in
  peak thrust (all from its merging of close points). This survey run used scratch scripts that
  are not committed, so the repo can't reproduce its counts; the committed check is the 32 bundled
  curves' test above.

**Consequences.**

- A curve file with a skipped spike, valid under the step rule, now builds and reports the thrust
  the curve actually delivers.
- `.rse` reading compares a file's `peakThrust` with the delivered peak; on the survey no warning
  changes, as no peak does.
