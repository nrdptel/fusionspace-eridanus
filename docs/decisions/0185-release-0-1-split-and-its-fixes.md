# ADR-185: Release 0.1 split, and its first fixes (2026-10-06)

- **Status:** accepted
- **Summary:** M10.1 is split in four: a, the flight and design fixes; b, the data, network and Python fixes; c, the release workflow; d, the release notes and ADR-162's checklist. M10.1a refuses a separation or ejection timed before the rail exit instead of firing it late, records one apogee, counts only errors in `SimError::DesignChecks`'s message, makes `DragTable::with_reference_diameter_m` check its diameter (#79's decision), and measures a part in a nose cone or transition against the room where it sits: an error past the fit tolerance of the widest room along it, a new warning when it fits there but runs into the wall where the profile narrows.

**Context.** M10.1's *done when* holds ten issues, a CI release workflow, the publish dry run, the
changelog and the install pages, more than one pull request can ship. Five of the issues are small
fixes in the flight and the design checks; five touch the network cache, the motor catalog, the
docs' accuracy figures, the Python binding and `.hprz` attachment names (#252, closed by a planning
PR's text though its fix never landed, and reopened); the rest is packaging.

#313 asked that an internal part in a nose cone or transition be compared with "the profile's
inner radius (the outer radius less the wall) over the part's own length, with the fit tolerance
of ADR-155", and named its test: a mass at the tip an error, a mass at the shoulder none. Before
it, the room was the profile's largest outer radius anywhere, so a 20 mm part at the tip of a
cone 27 mm wide at its base passed.

**Decision.**

1. **The split.** M10.1a: #46, #242, #231 and #313 fixed, #79 decided. M10.1b: #252, #272, #295,
   #298, #308. M10.1c: the dispatch workflow, archives, wheels, source package, licence texts, the
   `release` environment and `cargo publish --workspace --dry-run`. M10.1d: `CHANGELOG.md`, the
   install pages, and ADR-162 §2's checklist at a commit on `main`. M10.1's own *done when* is
   unchanged: it is met when all four are.
2. **#231: refused, not fired on the rod.** A separation or ejection whose time is known before
   the flight and comes while the rocket is on the pad or the rail is `SimError::Domain` when it
   comes, as an early mass shift or release already is. Firing it there would leave the booster on
   the pad, a flight nobody has asked for and OpenRocket's handling of which is unprobed. Only a
   time known before the flight (a time, a motor delay, a burnout) can come on the rail: an apogee
   or a height comes in free flight.
3. **#242: one apogee.** The apogee watch stays armed past the apogee until a part leaves, and can
   find the same root again one step later; a second firing records nothing. The watch is left
   armed rather than disarmed, so every flight's steps, and so its numbers, are unchanged bit for
   bit. A later, higher peak (a motor lit after the first apogee) gets no event either; the
   summary already took the first apogee, and the stack's apogee devices fire there.
4. **#79, decided: the builder checks.** `DragTable::with_reference_diameter_m` returns a
   `Result`, refusing a diameter that isn't finite and positive as the normal-force table's
   builder does. 0.x allows the break. A table whose field is set directly or read with serde is
   still checked at its first lookup. A `TableReference` for drag tables waits for a reader that
   needs one: RASAero's `.CDX1` (M3.5).
5. **#313: the room where a part sits.** In a nose cone or transition the room is the profile's
   outer radius less its wall (none when filled), over the part's span within the profile, read at
   the span's ends and 31 stations between. Two numbers come from it: the least and the most room
   along the part.
   - A rigid part (a tube or ring) past the fit tolerance of the *most* room is an error,
     `internal_part_wider_than_parent`, as in a tube.
   - One within the tolerance of the least room is `internal_part_tight_in_parent`, as before.
   - One between, fitting where the profile is widest and running into the wall where it
     narrows, is a new warning, `internal_part_wedged_in_parent`, and flies where it is drawn.
   - A packed part takes ADR-170's rule against the *least* room: a warning while its center is
     in it, an error once out.
   - A part wholly past the profile's ends, or touching one only, is measured against the
     largest outer radius as before: `internal_part_past_parent_end` already names it.
6. **A tool for the counts.** `cargo xtask design-checks` counts the findings by kind over the
   default `.ork` survey or any `--dir`, the per-file detail in gitignored `corpus-out/`, so a
   check's effect on real designs can be measured and quoted.

**Why the wedged warning, measured.** The literal rule, an error whenever a part passes the
tolerance of the least room along it, was run first, by a variant of the check that was not
committed, over every `.ork` under `refs/` and `validation/fixtures/`, the private collection
included (633 that open; the jar's examples were not in that run). It added 73 errors. Most
were couplers drawn as inner tubes reaching 30 to 150 mm into a nose cone from its base, with no
shoulder drawn. Each fits the nose's bore at the base, and its forward end runs 1.3 to 3.2 mm into
the wall where the ogive narrows. Each error would refuse an ordinary design's flight in
`hpr sim` over a drawing slip whose effect on the center of gravity is a few millimeters of tube.
Another 16 were rings wholly aft of a transition, touching its end. Split as above, the same
designs give, by `cargo xtask design-checks`:

| set | wider-than-parent errors | tight | wedged | packed |
|---|---|---|---|---|
| default survey (72 designs that open), before | 10 | 13 | 0 | 17 |
| default survey, after | 10 | 16 | 1 | 25 |
| private collection (560 files that open), before | 409 | 144 | 0 | 173 |
| private collection, after | 425 | 158 | 33 | 413 |

The 16 new errors, in 11 files newly refused, are tubes too wide even at the profile's widest
room, past the tolerance. The same tubes in a body tube would be errors too. No design in the
default survey gains an error. A part of no length at a profile's end is inside it, at that end:
at a nose's tip it has no room (found in review, where it had counted as touching the tip).

**The issue's test, read with ADR-170.** #313 predates ADR-170, which made a packed part drawn too
wide a warning while its center is in the room. A centered mass at a cone's tip is therefore a
warning, `packed_part_wider_than_parent`: the room closes to nothing at the tip and the mass's
center is on the axis. The test pins that, an off-axis mass at the tip (an error), a rigid
bulkhead at the tip (an error), the same bulkhead and a mass at the base (no finding), a wedged
tube and a tube too wide even at the base, and two mutation probes: the old largest-radius room
and a rule with no wedged branch each fail it.

**Consequences.** `hpr sim` refuses 11 more files of the private collection until their tubes
are resized, and warns on more. A centered packed mass wholly where a cone has no room, as at its
tip, only warns under ADR-170, though its center of gravity sits forward of where it can: the
flattering side, left visible as #367. The outer radius less the wall overstates the room on a slope by
`t (1/cos θ − 1)`, 1% of the wall at 8°: the error is on the flattering side and small. A part
past the profile's ends is not checked against a shoulder's bore: shoulders are not in the
layout's span, and the past-the-end finding already names the part.
