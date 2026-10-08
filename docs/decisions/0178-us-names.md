# ADR-178: US names, and the spelling check reads them (2026-10-05)

- **Status:** accepted
- **Summary:** M0.6e renames the 63 names that held a word on M0.6d's list (`center_of_pressure_m`, `MorrisDesign::analyze`, `VerticalUnit::Meter` and the rest, as they are now called) to their US forms, and `cargo xtask spelling` now reads names as well as writing, so a new one fails CI. Seven serialized types keep each renamed key's old spelling as a serde alias, read and never written, with tests that read the old form back; the design format's keys never held a word on the list. GDAL's own names for a unit are another program's data and stay.

**Context.** M0.6e's *done when*: "no public Rust, Python, JSON or schema name holds a word on
d's list, and design files written with the old keys still read (a test)"
([ADR-164](0164-the-fusionspace-product-system.md) §e). The check M0.6d built
([ADR-176](0176-the-spelling-check.md), [ADR-177](0177-the-spelling-check-in-the-crates.md))
skips every identifier, so it couldn't show the first half, and a grep can't say which matches are
names. Taught to read names, the check finds 421 in its scope on `main`: 63 distinct names, from
public items (`Ellipse::centre_east_m`, `Finding::CentreOutsideRocket`, `sets_centre`) to locals
and test names.

**Decision.**

1. **The check reads names.** In a source file's code (outside its comments and strings) every
   name; a string literal that is one bare key; and in writing, a word shaped like a name (joined
   by `_`, holding a digit or an inner capital), a name in a format string and each word of inline
   code. A name is split into words at `_`, digits and camel case's capitals, so `CENTRE_X`,
   `centreX` and `XMLCentre` all read, and is reported with its US form. Addresses stay out as
   before. All names are read, not only public ones: telling public from private needs a Rust
   parser, and a private name today is a public one after a refactor.
2. **Every name is renamed**, public or not, in the crates, `xtask`, the pages and the aero
   fixtures' keys (`center_of_pressure_calibers`, `blend_arc_center`; only the keys change, not a
   value). `--fix` does not rename names: a serialized one needs its old key kept.
3. **Old keys still read.** Seven serialized types renamed a key: `PlacedComponent` and
   `PlacedStage`'s `center_overridden`, the ellipse's `center_east_m` and `center_north_m`,
   `ShockExpansionSlope::center_of_pressure_m`, the finding kind `center_outside_rocket`, the
   metric point `center_of_dry_mass` and `VerticalUnit::Meter`. Each takes
   `#[serde(alias = "<old>")]`, and six tests (one for both placed types, with an override set so
   the flag is `true`) write the type, swap in the old key, and read back the same value; each
   fails without its alias. The ellipse's test also refuses a document holding both spellings of
   one key, as a duplicate.
4. **The design format had nothing to rename.** No key of the design format, its schema or its
   0.1 migration held a word on the list (the check reads `schema/` and `hpr-format` and found
   none), so a design file written before M0.6e reads unchanged. A test that used `colour` as its
   unknown key now uses `tint`.
5. **Kept, on the check's list as keys** (a new reason beside quotation, title and name): each
   alias above, its doc comment and its test; and GDAL's vertical-unit names (`"metre"`,
   `"metres"`) that GeoTIFF metadata and rasterio write, which the reader must match as written.
   The check counts 38 such matches on every run and fails on an entry that matches nothing.

**Consequences.** The public API reads in US spelling before the first publish: users of
`MorrisDesign::analyse`, `SobolDesign::analyse` or the renamed fields and constants rename their
calls (nothing is published, so no one outside the repository does yet). A JSON document written
before the change still reads. The check's exceptions now count 71 matches, 38 of them keys; the
two that let an example print `Metre` are gone, since it prints `Meter`.

Two things stay outside the check, as before: `validation/`, whose case files and fixture notes
are prose the check has never read, and whose RocketPy oracle script keeps its own result keys
(`wind_response.py`; rewriting an oracle means rerunning it); and the frozen records, which name
some tests by their old names (ADR-103 names one), as they were when written.

**Alternatives.** *Rename only the public names* (rejected: §1). *Keep the old keys only on
design-file types* (rejected: none of the seven is one; they derive serde so that programs built on the library can
save them and read them back, and a document saved before M0.6e would stop reading). *Accept both spellings of
GDAL's unit as ours* (rejected: it is GDAL's string, not a name of ours).
