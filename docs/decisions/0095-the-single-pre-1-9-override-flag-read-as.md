# ADR-095: The single pre-1.9 override flag read as OpenRocket reads it (2026-09-27)

- **Status:** accepted
- **Summary:** The single pre-1.9 override flag read as OpenRocket reads it

**Context.** Before schema 1.9 a `.ork` said once, with `overridesubcomponents`, whether a
part's overrides cover the parts inside it; later files say it per quantity, with
`overridesubcomponentsmass`, `...cg` and `...cd`. hpr read the old flag as setting all three,
unless a per-quantity flag was also written, which then won; and it warned (ADR-052, item 5).
The warning made any design carrying the flag "an airframe not read exactly as written", which is
not flown (ADR-055). Issue #174 names two private designs held that way: `C05` (six elements, one
`true`, on a stage's mass override; schema 1.4) and `C10` (three, all `false`; schema 1.8). M2.2's
bar is 20 designs with the five spreads (ADR-072); after M2.2e5 the two reports held 15. The old
M2.2e6 ("Twenty designs", blocked on #174 and #133) is split by reading: M2.2e6 the old flag,
M2.2e7 fin fillets and an inner tube's automatic radius (the rest of #174), M2.2e8 tube fins whose
radius OpenRocket works out (#133), and M2.2e9 the bar, unchanged.

**Decision.**

1. **Measured, not assumed.** `conventions.py` gained eleven probes: the old flag alone on a
   stage's mass and centre overrides and on a tube's mass override, in schema 1.4, 1.8 and 1.10
   files; `false` as well as `true`; and beside a per-quantity mass, centre or drag flag, in both
   orders. It records the structure and, through OpenRocket's public getters, the three flags it
   read each part with. OpenRocket 24.12 reads the old flag as setting all three, in all three
   schema versions, and where both forms are written **the later one wins**, quantity by
   quantity. The 75 earlier probes' answers are unchanged.
2. **hpr reads it so.** `Values::overrides` takes, for each quantity, whichever of its own flag
   and the old one comes later among the element's children, and raises no warning. This amends
   ADR-052's item 5. The old rule differed only where the old flag comes second, which no file in
   the corpus does. A flag tag written twice on one element is read at its first copy and warned
   of (`Dropped`), because which copy OpenRocket takes is not measured; no file in the corpus does
   that either.
3. **Held to the probes.** `hpr_validate::openrocket` holds hpr to OpenRocket on all eleven: the
   three flags part by part, the mass, no warning, and the centre of mass. The centre is
   OpenRocket's except where a mass override covering the parts inside states no centre. That is
   ADR-061's departure: 3.686 mm, the same as on the per-quantity flag's probe. Making the
   per-quantity flag win again fails the test.

**Consequences.**

- `C05` flies all five configurations: apogee −0.03% to +0.26%, largest speed within 0.61%,
  margin within 0.0113 calibres, mass at launch +0.000%. The two reports hold 16 designs.
  `C05/2`'s margin gap is nearly all its centre of mass (−0.0111 calibres, against at most
  0.0004 on the other four): not traced.
- **M2.2e6's bar is not met for `C10`.** Its three flags are `false`, which the old and new rules
  read alike, so the flag never decided its numbers. It had a second, independent reason not to
  fly: its one configuration separates a stage with no motor ahead of it, which can come before
  apogee, and `ork-flights` does not fly that (#184). With the flag read, its reason for not
  flying changes from "an airframe not read exactly as written" to "stages hpr can't separate as
  written"; the report lists it so. It stays in M2.2e9's pool behind #184.
- The corpus survey's warnings fall from 31 to 21: the 10 were this flag, 9 in `C05` and `C10`
  and 1 in a Loft demo design outside both reports.
- Not measured: an old flag that is neither `true` nor `false` (hpr drops it, out loud, and the
  file is not flown), and which copy of a tag written twice OpenRocket takes.
