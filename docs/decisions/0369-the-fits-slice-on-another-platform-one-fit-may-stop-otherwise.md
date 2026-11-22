# ADR-369: The fit's slice on another platform: one fit may stop otherwise (2026-11-22)

- **Status:** accepted; amends how the fit's slice is held on a platform other than the one that wrote it ([ADR-149][adr-149], amended 2026-11-04): in every word but, for one fit in a comparison, the word that says how the fit stopped; changes nothing where the slice is written
- **Summary:** One fit of the slice, a flight of set B, ends at its step limit on macOS and Windows and converges on both of CI's Linux runners. Every other word of its lines agrees. The slice's check on another platform now lets one fit in a comparison stop otherwise than where the slice was written, names it, and fails on a second. The build that writes the slice still holds it byte for byte. [#1597](https://github.com/nrdptel/fusionspace-eridanus/issues/1597) holds the question of the fit itself.

[adr-149]: 0149-m0-5e-g-reviews-while-ci-runs-a-faster-gate-and.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md
[adr-367]: 0367-the-tip-off-phase-is-the-default-what-moved-and-what-flies-held-straight.md
[issue]: https://github.com/nrdptel/fusionspace-eridanus/issues/1597
[slice]: ../../validation/reports/fit-intervals.slice.json
[validation]: ../VALIDATION.md#in-ci-and-regenerating-the-references

**Context.** The fit's report is checked in full once a night. Each pull request checks a
slice of it instead: every 21st fit of each run, written line by line to a committed file
([the slice][slice]; [how CI checks it][validation]). The build that writes the file holds it
byte for byte. Any other platform holds it in words: a number may move in its last digits
there, and every word must stay ([ADR-149][adr-149], amended 2026-11-04). One of those words
says how the fit stopped: "converged", or "at its step limit".

With the tip-off phase the default ([ADR-367][adr-367]), the regenerated slice was checked on
CI's five platforms. Two comparisons failed, both on one line: set B's flight 7443,
the Prometheus design on a log with a track. The slice, written on macOS, says its fit ended at
its step limit, after 767 simulated flights. Both Linux runners, x86-64 and ARM, made it
converge. macOS and both Windows runners agreed with the slice. Every other word of that
flight's lines agreed on all five: the side of each interval its truths fall on, by least
squares and by the Bayesian option.

A fit that meets its stopping rule on one math library and its step limit on another is moving
along a flat valley of its misfit, where the last digits of a step decide whether the rule is
met in time. The report already allows for this in its totals: the count of fits that reached
the step limit may stand 5 flights off on another platform ([ADR-316][adr-316]). The slice
had no such room, and until now no fit in it had needed any.

**Decision.**

1. **On another platform, one fit in a comparison may stop otherwise.** A comparison is one
   part of the slice on one machine, or the whole slice in the nightly run. Where a flight's
   first line differs from the committed one in the stop's word alone, its numbers aside, the
   check names the flight and holds every other word of its lines as before. A second such
   flight in the same comparison fails it.

2. **Nothing else is set aside.** A truth on another side of its interval, a log read
   otherwise, a posterior flagged otherwise, or any other word still fails, on that flight as
   on any other. The build that writes the slice still holds it byte for byte, the stop
   included.

3. **One, not five.** One is what was measured. The report's room of 5 is for totals over
   several thousand fits; the slice holds 444 flights, and a comparison a part of them. If a
   second fit of one comparison stops otherwise, that is new, and the check should say so.

4. **The fit is not excused.** [#1597][issue] asks which unknowns that fit trades along its
   valley and how far its answer moves between the two stops. If the stopping rule is mended,
   the allowance goes back to none.

**What this does not decide.** Whether the fit's answer for that flight is the same to its
stated error whichever way it stops: that is the issue's. Nor the census slices of the
synthetic flights and the two logs, which keep every word.

**Consequences.**

- `cargo xtask fit --slice --other-platform` and the nightly `fit --check --other-platform`
  print each fit that stops otherwise, and fail past one in a comparison. A test,
  `another_platform_may_stop_one_fit_otherwise_and_no_more`, holds the rule: one is named and
  allowed, a second fails, and a line that differs in any other word is not set aside.
- A platform's report of how a fit stopped is no longer held for that one fit. Its sides, the
  thing the report counts, are.
