# ADR-201: The unknown signs (2026-10-08)

- **Status:** accepted
- **Summary:** M10.1d6: of the seven issues of unknown sign, #104, #360 and #361 print no wrong number (the first is never printed, the other two are refused), and #8, #106, #213 and #219 err either way by case, so each warns, as a new `flight` kind: a near-calm wind level that turns, a body part's damping station inside the supersonic join, a single pod off the axis, and a center of gravity more than 1e-9 m off the axis.

**Context.** [ADR-188](0188-release-notes-and-the-flattering-side-survey.md) listed seven open
`env-core` issues whose sign wasn't known, and §2 asked that each get a sign recorded from
evidence or a warning; [ADR-190](0190-the-friction-and-freeform-fin-warnings.md) gave them to
M10.1d6. An issue counts as flattering when its evidence puts the error on ADR-144 §6's side for
some flight, so a sign that turns with the case is flattering for some flight and needs a warning.
Each issue was read against the code that carries it and probed on public or invented designs.
The probes were scratch runs, not committed, so this record leans on the closed-form arguments,
and the warnings quote no probe's figures.

**Decision.**

1. **Three print no wrong number; they need no warning.**
   - [#104](https://github.com/nrdptel/hpr-sim/issues/104), a deep boattail driving the body's
     own normal force through zero so the body's center of pressure runs off the rocket: that
     body-alone quotient is printed nowhere. Every center of pressure the CLI, Python and the
     `hpr` crate print is the whole rocket's, from `metrics::margin`, which divides the summed
     moment by the summed slope and refuses the quotient past its condition limit
     (`MARGIN_CONDITION_LIMIT`, √10). The flight applies each part's moment about the nose tip as
     computed. The slope and moment are right, as the issue says, so it is not an accuracy error.
   - [#360](https://github.com/nrdptel/hpr-sim/issues/360), a `.ork` finish the reader has no
     roughness for: the reader marks the airframe as not read as written, so no configuration
     flies (`hpr sim` on a file with `finishpolished` says so and stops). A refusal is not a silent
     number. Reading the finishes stays the issue's work.
   - [#361](https://github.com/nrdptel/hpr-sim/issues/361), a mass override covering the parts
     inside with a center-of-gravity override that doesn't: the reader drops the disagreement as a
     `Dropped` warning, so the design is refused the same way. With the mass flag set, hpr's
     reading already matches OpenRocket's, 0.0 m apart in the committed oracle test
     `override_precedence_matches_oracle`; with only the center-of-gravity flag set, the contents'
     place against the stated one decides the sign, so that case stays refused.
2. **Four err either way and warn,** each at any speed but #106, and whatever the drag, as they
   concern the flight's path, not its drag:
   - [#219](https://github.com/nrdptel/hpr-sim/issues/219): a center of gravity more than
     `OFF_AXIS_CG_LIMIT_M`, 1e-9 m, off the axis (a NaN counts), sampled on every vehicle the
     flight flies (the stack, then each sustainer after a powered split) at its start, at each
     motor's ignition and burnout and at its end, and on the stack fully burnt. With one motor
     burning at a time the center of gravity moves in a line between those samples, so the
     largest offset is among them; two off-axis motors burning together could peak between. hpr flies the
     moment the thrust makes about it; OpenRocket's flights don't, and no reference sizes it.
     Turning the offset half a turn reverses the moment, so to first order it reverses the change
     to the apogee and the drift wherever the flight isn't vertical: either can read low.
     Symmetric designs sit about 1e-18 m off; the 54 mm test rocket's rail buttons put it about
     5.8e-5 m off, so it warns. A lug or a rail button off the axis is common, so most designs
     will print this warning.
   - [#213](https://github.com/nrdptel/hpr-sim/issues/213): a pod set of one pod, offset from
     the axis (not 0; a NaN counts), holding a part. Its drag and normal force act on the axis, so
     their moments are left out. Two or more pods stand evenly round the axis and their offsets
     add to zero, so theirs cancel. The same half-turn argument makes the sign turn with the pod's
     side against the wind and the rail's tilt. Two single-pod sets facing each other still warn,
     an over-warning on the cautious side.
   - [#106](https://github.com/nrdptel/hpr-sim/issues/106): inside the shock-expansion method's
     join with slender-body theory, a body part's station, where its pitch and yaw damping is
     sampled, sits away from its own center of pressure (4.71 calibres at Mach 1.3 on the 54 mm
     test rocket, pinned). The margins don't read the station, so they aren't touched; the damping
     is, and with it the apogee and the drift, by a sign that has no closed form. It warns past
     the join's start (exclusive), to the join's end, or at every speed past its start where a
     lip's shape weight is under 1 or a part keeps slender-body theory's station. Each vehicle
     (the stack, each sustainer after a powered split) is judged on its own join and its own top
     Mach, and the message names each span that is met.
   - [#8](https://github.com/nrdptel/hpr-sim/issues/8): a `LayeredWind` under `SpeedDirection`
     interpolation with two neighbouring levels, neither exactly calm (that case already takes the
     other's direction), one at or below `NEAR_CALM_WIND_M_S`, 1.5 m/s, with different directions,
     the span between them reaching inside the flight's heights. 1.5 m/s is the top of Beaufort
     force 1, light air, whose direction "smoke drift" shows "but not … wind vanes" (MeteoSwiss's
     Beaufort table, 2023; the Hong Kong Observatory's land criteria): such a level's direction
     means little. The swing turns a wind of unchanged speed, so where the other levels share one
     direction the wind's sum over the path can only shorten, and the drift likely does; where they
     turn it can lengthen. The `Wind` trait
     gains `direction_turns`, empty by default, so a table and the Monte Carlo's dispersed table
     report their spans (the dispersed table judges and quotes the table's unscaled speeds, as
     reported); `hpr sim` flies a constant wind and Python's wind is a function, so only
     library flights meet it.
3. **A third kind, `flight`.** These four bear on the path, not the drag or the margin, so they
   print as `warning: flight:` lines, `kind` `"flight"` in the JSON and in Python. `IssueKind` is
   matched exhaustively, so a later kind can't print as another. An
   `IssueWarning` gains an optional `detail` (the offset, the join's span, the wind's spans), left
   out of the JSON when empty, so older records still read.

**Consequences.** `CHANGELOG.md` lists the four as warned and the three as printing no wrong
number; its "sign isn't known" line goes. Nearly every `.ork` design with a lug or rail button now
prints #219, and the CLI's and Python's warning lists in the tests gain it. The warnings give a
direction, not a size; sizing each needs a reference that models it (RocketPy's eccentricity
inputs may serve #219 and #213). The committed OpenRocket flight report has the public *Parallel
booster staging* example drifting 3.9 and 4.4 times OpenRocket's on its two flights; that cause
isn't traced, and is noted on #219. `hpr mc` warns only of its nominal flight, so a drawn wind
table's #8 isn't warned. Release 0.1's checklist, M10.1d4, no longer waits on d6.
