# ADR-024: The time-series RMS: aligned at ignition, held to 3% of its trace's scale (2026-09-18)

- **Status:** accepted
- **Summary:** The time-series RMS: aligned at ignition, held to 3% of its trace's scale

**Context.** M2.1 lists a "time-series RMS after alignment" among its comparisons, and M2.1a to
M2.1c built none. Each whole-flight fixture already carries a `series`: 120 rows of time since
ignition, the height of the centre of dry mass above the ground, and its speed, on a uniform grid
from ignition to RocketPy's impact. Nothing said what "alignment" means or what bound the RMS
answers to. hpr keeps no trajectory after a run, and the report's other rows are point metrics
held to 3% of their own reference.

**Decision.**

- **Two metrics per whole-flight case,** `series_height_rms_m` and `series_speed_rms_m_s`: the
  root mean square of hpr's value less RocketPy's at the reference's own series times, over every
  time at or before hpr's landing. Their reference value is 0, exact agreement. The harness
  requires every whole-flight case to name both.
- **Alignment is the shared clock.** Both codes start their clocks at ignition with the rocket on
  the rail, so the times align as they are. No time shift is fitted: a fitted shift would absorb a
  real difference in the burn or on the rail, which is what the comparison is for. hpr's values
  come from the dense output of the solver step that holds each time, not from interpolating
  between samples, so the comparison adds no error of its own beyond the step's interpolant.
- **The window is while both fly.** RocketPy's series ends at its impact; hpr's comparison stops
  at hpr's landing, the same event its `flight_time_s` measures. A landing-time difference is
  already gated by that metric, so it is not counted twice as a tail of ground-level zeros.
- **Each RMS is held to 3% of its trace's scale:** the height RMS to 3% of the reference's apogee,
  the speed RMS to 3% of its max speed, rounded down to 0.1. That is M2.1's 3% for a point metric,
  applied to a whole trace, so a trace passes only if it is on average no further off than its
  peak may be. The bound was set before the first measurement. The gate test holds each RMS gate
  no looser than that scale, so no absolute floor can widen it. In predicted mode the same bound is
  a target (ADR-023).

**Consequences.**

- Measured, same-drag: height RMS 1.4 to 39.2 m (0.12% to 1.5% of apogee, Juno III the largest)
  and speed RMS 0.13 to 2.06 m/s; all ten RMS rows pass. Both Prometheus 2022 cases name both
  metrics with their bounds but fly nothing: they stay the checked `M ≥ 1` gap until M1.8, so ten
  of the twelve whole-flight cases report an RMS. Predicted: Valetudo's height RMS and NDRT 2020's
  height and speed RMS are outside target, for the drag that puts their apogees 10% high; each is
  explained in its case file and pinned with the other misses.
- The RMS weights every grid time equally, so a long descent weighs more than a short burn. A
  difference in the burn shows in the point metrics (burnout, max speed and acceleration), which
  remain gated. The speed RMS is loose on the descent, where speeds are 5 to 25 m/s against a bound
  set by the top speed (Calisto falling under its main at twice RocketPy's rate would give about
  5 m/s, inside 7.3); the descent rate is gated by `impact_speed_m_s` instead.
- The window ends where the first code lands, so the tail where the heights differ most is not
  counted; the same-drag `flight_time_s` gate covers it, and in predicted mode it is a target.
- The horizontal path is not in the series, so the drifts stay with issue #50 (M2.1d2).
