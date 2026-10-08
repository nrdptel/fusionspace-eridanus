# ADR-052: What a `.ork` value means: automatic dimensions, two names for one tag, and overrides (2026-09-20)

- **Status:** accepted; item 5's warning superseded by ADR-095
- **Summary:** What a `.ork` value means: automatic dimensions, two names for one tag, and overrides

**Context.** M3.1b turns the document tree into a design. Before any component can be read, three
things about `.ork` values have to be settled, and Loft got all three wrong: a dimension may say
`auto 0.0125` rather than a number (L58), a tag may be written under two names at once (L62), and a
stated `0` is a value rather than a missing one (L63). There is no schema to settle them from, so
each rests on what the reference corpus shows, counted by `cargo xtask ork`.

**Decision.**

1. **M3.1b splits into M3.1b1 (the values) and M3.1b2 (the components).** The values are what every
   component reader asks for, and they can be settled and tested on their own.
2. **An automatic dimension keeps both halves.** `Dimension::Automatic { cached }` holds what
   OpenRocket last worked out — `auto 0.0125` gives `Some(0.0125)`, a bare `auto` gives `None` —
   and is never confused with `Dimension::Stated`. Saving a design must write `auto` back, which is
   what Loft's dropping of the flag broke. 413 dimensions in the corpus are automatic, across
   `outerradius`, `innerradius`, `radius`, `aftradius`, `foreradius`, `packedradius` and `cd`.
3. **Either name of a renamed tag may be read, newest first — for the two renames that are only
   renames.** Where OpenRocket writes both `axialoffset` and `position` (642 elements) or both
   `instancecount` and `fincount` (109), it agrees with itself on the text *and* on the
   `type`/`method` attribute that says what the number is measured from, every time. So either
   name may be read, the newer wins, and a disagreement — in the number or in the frame — raises
   a warning.

   `angleoffset`/`radialdirection` and `radiusoffset`/`radialposition` are **not** read this way,
   though they look the same. The newer name of each carries a `method` the older never carries:
   of the 26 elements with both `angleoffset` and `radialdirection` the texts agree every time and
   the frames differ every time, and `radiusoffset` (106 elements) and `radialposition` (542) are
   never written together at all. Reading one as the other would move a component in silence, so
   what the older name's frame is belongs to M3.1b2, with a source. The constants for them are not
   in the public API.
4. **A stated zero is a value.** `<overridecd>0.0</overridecd>` (2 in the corpus) is an override to
   no drag at all. The six override tags — three values, three flags — are read independently.
5. **The single pre-1.9 `overridesubcomponents` flag sets all three.** It is what the three
   per-quantity flags replaced; 20 elements of the corpus carry it and **none** of them carries a
   per-quantity flag beside it, so reading it as all three cannot contradict a file. It raises a
   warning, so the inference is never silent. *Amended by ADR-095:* OpenRocket 24.12's reading
   was measured, and hpr now follows it without a warning; the survey now counts 10 such elements.

**Consequences.** The value layer is settled before any component reads it, and its claims are
counts anyone with the corpus can reproduce by running `cargo xtask ork`, which prints each one.
Points 3 and 5 are readings of what a rename meant, not statements from a specification: if a file
ever turns up where the two names disagree, or where the old flag sits beside a new one, the
warning says so rather than the reading being wrong in silence. Two renames are left unread
rather than guessed, which means M3.1b2 cannot place an instanced component until it settles
them. `Dimension` carries no unit, because the tags it reads are metres, radians and
plain numbers alike; the component reader names the unit.
---
