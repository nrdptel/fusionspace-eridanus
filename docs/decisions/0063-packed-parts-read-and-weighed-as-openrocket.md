# ADR-063: Packed parts read and weighed as OpenRocket packs them (2026-09-21)

- **Status:** accepted
- **Summary:** Packed parts read and weighed as OpenRocket packs them

**Context.** M2.2b3 (ADR-062 §1) held four things: clusters, fillets, packed parts OpenRocket
sizes itself, and the parts kept unread. ADR-062 §7 had measured two packed-part gaps on its
probes: a parachute that writes no packed size (hpr's centre of mass 0.611 mm off, roll −0.167%,
pitch +0.744% of the probe's), and a mass override on a parachute that weighs nothing (roll
−0.805%, pitch −0.128%). Clusters need M1.9's mounts and fillets a shape of their own; neither
fits in the same pull request.

**Decision.**

1. **M2.2b3 splits.** b3 takes the packed parts. A new b4 takes clusters, fillets and the parts
   kept unread, and the stored results (Loft lesson L87) become b5. Every item of the old *done
   when* is in b3's or b4's.
2. **Measured on nine more probes** in `validation/oracles/openrocket/conventions.py`, each a
   tube and one part, fixed ids: a streamer, a shock cord and a mass component written with no
   packed size; a parachute written with only a packed length, and one with only a packed radius;
   a parachute with no packed size in a tube twice as wide, and in one whose bore is 8 mm in
   radius; a mass component of no mass and a shock cord of no length, each under a 30 g override.
   The earlier probes' records are unchanged by the rerun, bit for bit.
3. **An unwritten packed size is OpenRocket's.** A packed part that writes no `packedlength` is
   25 mm long; one that writes no `packedradius` is 12.5 mm in radius, a fixed number: it is the
   same in bores 48 and 98 mm in radius, and OpenRocket does not shrink it to fit a bore 8 mm in
   radius. hpr does not either (only an automatic radius is fitted to the bore). Each holds on
   its own: the half-written parachutes take the written half and the default for the other. It
   holds for all four kinds. The numbers are inferred from OpenRocket's output, not taken from
   its source. `hpr_io::ork` reads them with no warning, as ADR-061 reads what else a file leaves
   unsaid. Before, hpr read a length of zero silently and a radius of zero with a warning.
4. **A mass override on a weightless packed part is spread over its packing.** hpr gives the
   override `m` the packing's solid cylinder: `m r²/2` in roll, `m (3r² + l²)/12` in pitch, its
   centre halfway along. Any other part that weighs nothing still becomes a point mass under an
   override, which ADR-061 measured as both programs' rule. `hpr_design::tree` does this in both
   places an override applies, the part's own and one covering the parts inside. A streamer
   takes the rule by the same packing; no probe weighs a weightless one under an override.
5. **One cause retired.** `cargo xtask ork` no longer counts "packed parts hpr weighs as point
   masses" as a cause for a roll gap: hpr no longer makes such a point mass.

**Consequences.** `hpr_validate::openrocket::tests` holds all 42 probes of one tube and one part:
the two that were pinned gaps and the nine new ones are OpenRocket's to 1e-12 in mass, centre,
roll and pitch. On 2026-09-21 (`cargo xtask ork`, 74 files OpenRocket opens, before and after):

| | before | after |
|---|---|---|
| mass within 1% | 61 | 61 |
| centre of mass within 0.1% of length | 54 | 55 |
| pitch inertia within 0.1%, median | 34, 0.110% | 37, 0.086% |
| roll inertia with OpenRocket's fin rule, within 0.1% and 1% | 49, 55 | 53, 57 |
| files outside 1% in that roll, by content | 19 (15) | 17 (13) |
| roll inertia, hpr's own fins, within 1%, median | 29, 2.112% | 28, 2.354% |

The last row moves the wrong way on purpose: in one file the point mass's missing roll inertia had
offset part of hpr's fin departure (ADR-062 §3). The 17 still outside each have a cause: a
covering override (7 by content) or a reduced design (6). The Loft demo whose streamer writes no
packed radius no longer warns of it. A flight changes only where a `.ork` packed part writes no
size, or a weightless one carries an override. Two readings are hpr's and unprobed: a packed size
written but unreadable is read as zero, with a warning, as every unreadable number is; and every
probe places its part from the top, so the 25 mm length's effect on a part placed from the middle,
bottom or after another is derived, not measured.
