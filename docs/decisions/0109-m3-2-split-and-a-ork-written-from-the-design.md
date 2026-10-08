# ADR-109: M3.2 split, and a `.ork` written from the design (2026-09-29)

- **Status:** accepted
- **Summary:** M3.2 split, and a `.ork` written from the design

**Context.** M3.2 is done when a corpus design read by hpr and written back out loads in
OpenRocket 24.12, and OpenRocket flies the written file within 0.5% of the original's apogee. The
reader keeps the document whole (ADR-051) and, beside the design, whatever hpr does not model in
`x-openrocket` (ADR-058). `ARCHITECTURE.md` says every exporter maps out of the `hpr-design` model.
Loft's exporter lost plugged delays, per-configuration ignition, conditions, freeform fin points,
cluster scale and rotation, and every digit past the sixth (L67, L68).

**Decision.**

1. **Split in two.** M3.2a writes the file and proves, in Rust, that it reads back as the same
   design; M3.2b runs the OpenRocket oracle on the written files, which is the milestone's own
   two bullets.
2. **Written from the design, not the kept document.** `hpr_io::ork::export` builds the document
   from the `Design` alone: each writer writes exactly the tags its reader asks for, and the
   splice (`export/kept.rs`) puts back every kept part, section, tag and attribute. A design built
   in code, or edited after reading, is written the same way. `cargo xtask ork` holds it: every
   corpus design is written, read back and compared, and written again from what was read.
3. **Tags and sections count among their own name.** A kept tag's path now says it was the k-th
   `<appearance>` of its element, not the k-th tag of any name (a change to M3.1's paths); one
   that names a configuration counts among those naming the same one,
   `@deploymentconfiguration(b)[0]`, since the writer lists configurations in the design's order,
   not the file's. OpenRocket reads no meaning into tag order, with one exception below, and
   counting by name lets the splice put a tag back without knowing where it stood; a part still
   counts among all parts, since their order is where they stack.
4. **A value the reader drops is kept.** A reader that asks for a tag and then drops what it says
   (a rail button's screw height, a drag override, a ring of three, a finish word hpr doesn't
   know) marks it unasked (`reads::forget`), so `x-openrocket` keeps it, and the kept copy is
   written in place of whatever the design would say. Reading the written file raises the same
   warning, and OpenRocket reads the original value. The price: after an edit to the design, a
   kept tag's text still wins on export.
5. **Numbers exactly.** Each number is the shortest decimal that parses back to the same `f64`
   (L68). An angle is the shortest number of degrees whose conversion gives back the design's
   radians; the ogive parameter and rail-button offsets are searched for the same way.
6. **What OpenRocket 24.12 insists on,** found by loading written files in it: an id that is not
   a UUID makes it refuse the whole file, so such an id is left out (the reader invents the same id
   for a part with none, `bodytube-3`, and the tag of a part hpr reads as one kind, such as an
   engine block, is taken from that id or from what is kept), and an id that is not one the
   reader invents is warned of; an inner tube's and a packed part's roll angle is read only as
   `radialdirection`, so it is written so, with `angleoffset` beside it only where the file's
   disagreeing older tag is kept; a lug's and a rail button's `radiusoffset` is not read, so it is
   not written; a kept `<preset>` goes first, after the part's name and id, because OpenRocket
   applies a catalogue preset where it reads it, over the sizes read before. Schema 1.10, a zip
   with `rocket.ork` first and the original's attachments after, dated zip's zero date so the
   bytes never depend on the clock.

**Measured** (`cargo xtask ork`, 2026-09-29): 73 designs written; 73 read back the same; 73
written again byte for byte; no export warnings. Loaded in OpenRocket 24.12 once, by hand, every
written file opened whose original opens (71 of 73; the other two fail as originals do); M3.2b
commits that check.

**Still lost** (none in the corpus changes a design read back): a tag the file leaves out, where
the reader states its assumption (a radius with nothing to take, a missing wall or density) and
the written file states it, so its warning is gone on reading again; a second motor in one mount
for one configuration; a configuration with no id or declared twice; simulation rows and events
the reader left out; a fin tab dropped on a flat pod tube; the text of a second copy of a tag.

**Not chosen: writing the kept document back with the design's values patched in.** It would
round-trip read files trivially but could not write a design with no document behind it, and
every value would need a map back to the tag it came from.

**Not chosen: comparing designs with the kept tags' paths ignored.** It would hide a tag put back
in the wrong element.
