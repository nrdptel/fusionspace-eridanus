# ADR-132: M5.5a, OpenRocket's `.orc` parts catalogues, held to OpenRocket's reading (2026-10-01)

- **Status:** accepted
- **Summary:** M5.5 split a and b; M5.5a: `hpr_io::orc` reads OpenRocket's `.orc` parts catalogues; the 16 files OpenRocket 24.12 ships bundled unchanged (Apache-2.0); held part by part to OpenRocket's preset loader, run as an oracle; exact unit definitions, the file's makers' names and densities kept, a stated mass kept beside them; unreadable parts left out with a warning, not the file

**Context.** M5.5 asks for OpenRocket's `.orc` component database, looked up by vendor and part
number, with parts usable from the design API; done when "all `.orc` files parse, and a design
built from catalog parts simulates". The database is `openrocket/openrocket-database`
(Apache-2.0), pinned in `refs.lock.toml` at `1512874a` (2025-07-27). OpenRocket 24.12's jar ships
its 16 files byte for byte. The format has no schema; the project's `docs/TechnicalInfo.md` lists
fields and units, and says a shape parameter "cannot be specified" and a material must be defined
in the same file. A survey of the files: 3,449 parts of ten kinds, 402 materials, lengths in `in`,
`mm`, `cm` and `ft`, masses in `oz`, `g` and `kg`; 3 materials named but not defined; 21 part
numbers naming two parts; no shape parameters.

**Decision.**

1. **Split a and b.** M5.5a reads the catalogue (this ADR); M5.5b builds parts from it in the
   builder and flies a rocket of them, each part's built mass held to OpenRocket's for the same
   preset.
2. **The 16 files are bundled unchanged** in `crates/hpr-io/data/openrocket-database/` with the
   project's `LICENSE`, compiled in by `include_str!` (2.1 MB of text; `hpr_io::orc::bundled`
   reads them once), and listed under Bundled in `THIRD-PARTY-NOTICES.md`. Apache-2.0 allows it
   with the licence and the notices kept; the files carry no `NOTICE`. `.gitattributes` keeps
   their bytes. The `refs/` copy stays the pinned source; the oracle checks the jar's copies equal
   the bundled ones.
3. **OpenRocket's own reading is the oracle.** `validation/oracles/openrocket/orc_presets.py`
   runs OpenRocket 24.12's public `OpenRocketComponentLoader.load` on each file (run, never read)
   and records every value of every preset (`ComponentPreset.ORDERED_KEY_LIST`) to
   `crates/hpr-io/tests/fixtures/orc/openrocket-presets.json` (1.5 MB, one part a line), with each
   part's densities as OpenRocket reads the file with every `<Mass>` removed.
   `tests/orc_openrocket.rs` holds hpr's reading to it, part by part in order, every value to
   the bit. Where the two differ the test counts each departure and checks its cause. The script
   also has OpenRocket read 37 probe catalogues, each asking one question (every documented
   unit, values with no units, a part or file it can't read, a material or a list stated twice),
   and records each probe's text with
   OpenRocket's parts or refusal; the same test reads each probe and names what hpr does.
4. **Units are their exact definitions** (NIST Handbook 44, Appendix C), so hpr departs from
   OpenRocket's rounded factors: its ounce is 0.0283495231 kg against the exact 0.028349523125 (185
   masses differ by 8.8e-10 of their value); the fixture's probes show `lb/ft³`, `oz/in²`, `oz/ft²`,
   `lb/ft²` and `oz/ft` rounded the same way, none used by the bundled files. Values with no `Unit`
   are SI, and a density with no `UnitsOfMeasure` is SI, as OpenRocket reads them (probes). `g/m2`
   is read as written, as OpenRocket does, though six ripstop nylons labelled so (five in
   `generic_materials.orc`, unused; Giant Leap's, whose six canopies state their mass) are plainly
   kg/m² (0.067 g/m² is no fabric); a surface density under 1 g/m² warns.
5. **The file's values are kept.** The maker is the file's (OpenRocket shows "LOC Precision" as
   "LOC/Precision" and "Public Missiles" as "Public Missiles, Ltd.": 252 parts). A stated `Mass` is
   kept in `Part::mass_kg` and the material keeps the file's density; OpenRocket instead gives a
   bulk part its stated mass by replacing the density (207 parts). The cause is shown on all 207:
   with every `<Mass>` removed, OpenRocket's density equals hpr's on every part, and it differs
   exactly on the bulk parts stating a mass; on the 54 simple solids among them (7 body tubes, 4
   bulkheads, 34 filled conical noses and 9 transitions, shoulders as solid cylinders) the replaced
   density times the closed-form volume is the stated mass to 1e-15 (largest 5.9e-16). M5.5b makes a
   built part's mass the stated one. A material the file doesn't define has no density (`None`)
   where OpenRocket gives it zero (3 parts). A field stated twice keeps the last, as OpenRocket does
   (3 descriptions).
6. **Strict per part, lenient per file.** Only text that isn't XML, whose root isn't
   `<OpenRocketComponent>`, or nested past `MAX_DEPTH` (16, refused before it is parsed:
   `OrcError::TooDeep`) is refused. A part with a missing, unreadable or negative dimension, a unit
   or shape the format lacks, a material of the wrong kind, a value holding an element, or an
   unknown element is left out with a warning. OpenRocket 24.12 throws on the whole file for a
   missing dimension, an unknown unit or shape (probes); it reads an unreadable number (`ten`) as
   zero, a material of the wrong kind with a density of zero, and `BT<b>-</b>20` as `20` (the text
   after the last element, a quirk not copied). An unknown field is ignored with a warning
   (OpenRocket ignores it silently): the 37 nose cones that state an `InsideDiameter` read that
   way. `in/64` is refused: OpenRocket reads it as inches. As OpenRocket does (probes), a list
   stated twice keeps the last and a material defined twice keeps the first, each with a warning.
   Implausible values read as written with a warning: an inside diameter not under the outside, a
   solid under 1 kg/m³, a fabric under 1 g/m². A density below zero or past `f64` is left out.
   Warnings carry a `WarningKind` and stop at `MAX_WARNINGS` (1,000) with a count of the rest;
   `read` returns a `CatalogFile`. Materials are found through an index by kind and name, so a
   file's reading stays linear in its size.
7. **Lookup.** `Catalog::find(maker, number)` matches the whole part number exactly and the maker
   in any case, both trimmed, and returns every match (21 numbers name two parts, 3 pairs
   identical). Numbers are
   often several in one (`BT-20, 30316`); pieces are ambiguous (`White` names five parts), so
   `Catalog::search` matches text in numbers and descriptions instead.

**Consequences.** M5.5a is met: the 16 bundled files read; every part OpenRocket reads is read, in
its order, 3,449 parts; 17,911 values equal OpenRocket's to the bit, and the departures are exactly
185 ounce masses, 252 makers' names, 207 derived densities and 3 undefined materials, of 18,306
sizes, masses and densities compared (the parachutes' counts of sides and lines are compared too).
The reader's warnings on the bundled files are 55: 37 nose cones' inside diameters, 3 doubled
descriptions, 3 undefined materials, 3 tube-like parts no narrower inside than out, 3 solids lighter
than air (18 paper rings and blocks would weigh a millionth of real ones) and 6 fabrics under
1 g/m². Moving the inch one float step fails the oracle test. Of the 37 probes, hpr reads 23 as
OpenRocket does to the bit, 6 within 2e-9 by its exact units, and leaves out what OpenRocket can't
read (4 it refuses whole, `oz/in` among them, which hpr reads with the cord's density undefined) or
reads oddly (4: `in/64`, `ten`, a material of the wrong kind, an element inside a part number).
Nothing flies a catalogue part yet (M5.5b), and no shape parameter or shoulder wall is chosen for
one: the file gives neither.
