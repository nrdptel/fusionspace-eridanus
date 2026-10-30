# ADR-309: A fin tab weighed as its file states it (2026-10-30)

- **Status:** accepted; closes [#357][i357], which proposed a micron tolerance and a trim instead
- **Summary:** a fin tab may start ahead of its root's leading edge or end past its trailing edge.
  It is weighed as stated, a slab over the length the design gives it, as OpenRocket 24.12 weighs
  one, so long as it meets the root over some length. The tab check refused seven flights of the
  private collection; HPR Sim now flies them, and flies 68 of its 83 flights.

[i357]: https://github.com/nrdptel/fusionspace-eridanus/issues/357
[report]: ../../validation/reports/fixture-flights.md

**Context.** `FinSet::validate` refused a tab unless it lay along the root chord, within a relative
1e-12 of its end. Seven flights of the private collection ([report][report]) were not flown for
that reason alone. #357 read the cause as rounding: OpenRocket 15.03 writes a tab's position and
length rounded, so their sum can pass the root's end by tens of microns. It proposed a micron
tolerance and a trim to the root.

A scan of the collection's `.ork` files found two kinds of overhang, not one:

- **Rounding.** Tabs whose position and length were rounded pass the root by less than 0.03 mm.
- **By design.** Other tabs pass the root's trailing edge by more than 0.8 mm, up to several
  millimeters, and one starts tens of millimeters ahead of its root's leading edge. A builder may
  run a through-the-wall tab past the root to lock it into a centering ring.

Of the seven refused flights, four were rounding and three by design. A micron tolerance would
fly the four and leave the three refused under the same words.

**What OpenRocket does.** OpenRocket 24.12, run as the oracle on every design the scan found,
refuses none of these tabs. On loading, it trims two of the rounding ones, freeform fin sets whose
tab is measured from the front, to end at the root; it keeps every other tab as the file states
it, the millimeter overhangs and the tab ahead of its root included. It weighs each fin set with
its tab as kept.

**Decision.**

1. A tab is weighed as stated: the slab `offset ≤ x ≤ offset + length` below the root, whatever
   the root's length. Its offset may be negative.
2. A tab must meet its fin's root over some length: `offset < root` and `offset + length > 0`.
   One that meets it nowhere, or only at an end, holds to nothing and is refused with "a fin tab
   must meet its fin's root chord". Its depth is checked as before.
3. The `.ork` reader reads a tab past its root without a warning. A warning there would mark the
   airframe "not read exactly as written", and HPR Sim does not fly such a design; a tab weighed as
   its file states it is read exactly as written.
4. HPR Sim does not trim the rounding overhangs as OpenRocket does for two of them. The trimmed
   slab is under 0.03 mm long, too little to show in a mass or an apogee.

**Consequences.**

- The seven flights fly: HPR Sim flies 68 of the collection's 83, and compares 60 with their logs,
  each in the same agreement band with OpenRocket as most of the rest ([report][report]).
- The collection's numbers in the docs move with the report: every page that quotes them was
  updated in the same change.
- `xtask fixture-flights` no longer names the tab as a cause; its issue is closed.

**Tests.** `hpr-design`'s `a_tab_must_meet_the_root_and_fit_the_body` holds the bound both ways at
each end of the root (a tab touching an end, or a nanometer past it, is refused; a nanometer onto
the root is weighed), and `a_tab_past_the_root_is_weighed_as_its_whole_slab` holds the slab's
mass and center of mass. `hpr-io`'s `a_fin_tab_past_its_root_is_read_as_stated` reads tabs past
either end, by rounding and by design, with no warning.
