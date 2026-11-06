# ADR-323: The transonic margin warning, and shorter known-issue lines (2026-11-06)

- **Status:** accepted; adds [issue #1198][i1198]'s stability warning to the [known-issue warnings][adr-181], adds a row to [ADR-188][adr-188]'s flattering-side survey, and changes `hpr sim`'s and `hpr mc`'s text output: a known issue's line no longer ends with its address, which one `help:` line gives after them.
- **Summary:** From Mach 0.8 the fins' straight-line join puts the center of pressure aft of a wind tunnel's, by 0.6794 and 1.343 calibers on two Arcas Robin models at Mach 0.9, so the flight margin reads high, and nothing warned. Every flight with a margin past Mach 0.8 now warns (#1198). To keep the guides' example output inside the search index's 10 MB limit, a known issue's text line leaves its address to one `help:` line, and the always-on warnings are shorter; #18's message now says its 3.6% is of the friction, not the drag. #414 comes before any further prose.

[i1198]: https://github.com/nrdptel/fusionspace-eridanus/issues/1198
[i414]: https://github.com/nrdptel/fusionspace-eridanus/issues/414
[adr-181]: 0181-the-stability-issue-warnings.md
[adr-188]: 0188-release-notes-and-the-flattering-side-survey.md

**Context.** Issue #1198 measured HPR Sim's center of pressure against the wind tunnel
(`validation/fixtures/aero/normal-force-vs-mach.json`, recomputed by `hpr_aero`'s
`normal_force_against_mach` test). At Mach 0.9 it sits aft on both Arcas Robin models, by 1.34
calibers (short) and 0.68 (long), each more than its reading's uncertainty (0.055 and 0.071). An
aft center of pressure makes the flight margin, taken at the flight's Mach number, read high: the
flattering side. The static margin, taken at Mach 0, is clear of the join. At Mach 0.8 the fins are
still on Prandtl-Glauert; the long model's +0.067 there is inside its uncertainty, but the short
model's +0.912 is not, and #1290 tracks it below the join. Near Mach 1
the error turns both ways: the short model reads 0.41 calibers forward at Mach 1.0. The long
model's +2.36 at Mach 1.0 rests on a faired curve that its plotted points contradict
(`docs/research/arcas-fin-share-cp-station.md` §4). No public fin method closes the Mach 0.9 gap
(`docs/research/transonic-fin-lift-methods.md`), so the warning is needed whatever replaces the
join; the model's correction belongs to #619, #841 and #860.

**Decision.**

1. `KnownIssue::TransonicCenterOfPressure` (#1198, stability) warns on every flight with a static
   margin whose top Mach number passes 0.8 (`TRANSONIC_CP_MACH`, the join's start), as #172
   warns on every flight with a margin; this narrows the issue's "every flight" to those. It
   says the *flight* margin may read high. Its message quotes `TRANSONIC_CP_AFT_CAL`, 0.67 to 1.35
   calibers, the two models' Mach 0.9 errors rounded outward: the one speed on the join where both
   read aft beyond their uncertainty. A flight on a normal-force table of its own meets none,
   as with the other stability warnings.
2. A test reads the fixture's Mach 0.9 rows. It pins that HPR Sim's center of pressure
   sits aft of the measured at Mach 0.9 on both models, by more than the reading's uncertainty,
   and that the quoted range holds every error within 0.01 caliber. Swapping the two centers of
   pressure in one row and negating its stored error fails it, and so does inverting the Mach
   condition.
3. **The index.** The guides show real `hpr sim` output, and the warning adds 9 lines to indexed
   pages. Built, the index stood 9,123 bytes past mdBook's warning, which the site
   check fails on. Every page's search hits are kept; repeated text was cut:
   - The text output ends a known issue's line at its message. One `help: issues:
     https://github.com/nrdptel/fusionspace-eridanus/issues/<number>` line follows them. The
     line already names its number, and the JSON keeps `url` and the whole message, as Python's
     warnings do.
   - The drag warnings' shared consequence is one sentence: "the apogee, top speed and drift read
     low, and the flutter margin and largest dynamic pressure look better than they are". #172
     drops "for a reason not yet found" for "cause unknown".
   - #18 said "HPR Sim's drag reads high by 3.6%". `LAMINAR_FRICTION_PERCENT` is the share of
     the *friction* a laminar run takes off, about 2% of Calisto's zero-lift drag at Mach 0.3
     (ADR-188), so the message overstated the drag's error. It now says a laminar run "lowers
     it [the friction], by 3.6%".

   With the docs' own lines for #1198, the index passes with almost nothing to spare.

**Consequences.** A flyer past Mach 0.8 is told the flight margin may read high, with the measured size.
A terminal line no longer holds a clickable address; its number and the one help line give it.
The index is full again: #414 (a chunked index, measured by download size) comes before any
milestone that adds site prose.
