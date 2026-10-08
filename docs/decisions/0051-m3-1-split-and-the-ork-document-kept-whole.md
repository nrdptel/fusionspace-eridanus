# ADR-051: M3.1 split, and the `.ork` document kept whole rather than interpreted (2026-09-20)

- **Status:** accepted
- **Summary:** M3.1 split, and the `.ork` document kept whole rather than interpreted

**Context.** M3.1 asks for a whole OpenRocket importer: three containers, schema 1.0 to 1.11,
components, materials, finishes, motor configurations, recovery, stages, stored simulation results,
unknown content preserved for a lossless round trip, warnings instead of failures, and a cross-check
against RocketSerializer. Loft owns eleven lessons in it (L56 to L66) and Loft's importer is 2,793
lines of TypeScript without its tests. That is several pull requests' work, and the roadmap's rule
is to split in place.

There is no XSD for `.ork` and no published grammar — only prose documentation and sample files —
and OpenRocket's Java source is GPL, so it cannot be read. Every OpenRocket release has added tags.
The reference library holds 78 `.ork` files spanning schema 1.4 to 1.11.

**Decision.**

1. **Split M3.1 into M3.1a to M3.1d**, one increment level as usual: the container and the
   document (a), the component tree (b), motors, recovery, stages and stored results (c), then the
   corpus and the cross-check (d). The parent's four *done when* bullets are unchanged and are
   M3.1d's; each increment carries its own.
2. **The document is read whole and interpreted by nobody.** `hpr_io::ork` reads the XML into a
   plain tree of elements and text — every attribute in order, every leaf's text as written — and
   later increments walk that tree. With no schema to check a file against, keeping the whole
   document is the only way to be sure nothing was quietly dropped, and it is what makes
   `extensions.x-openrocket` (M3.1c) and export (M3.2) possible at all.
3. **Reading, writing and reading again returns the same document.** The writer is canonical
   rather than byte-faithful: it lays the file out afresh, and escapes the characters an XML
   parser would otherwise change (a carriage return in text; a tab, newline or return in an
   attribute). Blank text between child elements is dropped on the way in, so laying it out again
   is free. Text an element holds beside child elements is kept, because OpenRocket writes it —
   a simulation's `<warning>` prints its message that way in 48 elements of 19 corpus files. The
   property is tested over generated trees and over all 76 corpus files that open.
4. **Nesting is counted before the text is parsed**, and a document deeper than 64 elements is
   refused. On a debug test build with a 2 MiB stack `roxmltree` read 120 levels of nesting and
   died on 130 — a crash where L56 asks for an error, and a figure that moves with the stack a
   platform gives a thread, which is the argument for not relying on it. Of the 76 corpus files
   that open, 53 nest 11 deep and the deepest reaches 17 (OpenRocket's parallel-booster example,
   where a nested stage or inner tube costs two levels and an appearance three). The count comes
   from a scan that skips comments, CDATA and processing instructions and tracks quotes, so it can
   only overstate the depth a parser will find, which a property test holds it to.
5. **Two dependencies**, both with justification in `THIRD-PARTY-NOTICES.md`: `zip` 8.6 (MIT,
   read-only, only the deflate method OpenRocket writes — the default features would pull bzip2,
   lzma, zstd and AES) and `flate2` 1.1 (MIT OR Apache-2.0, pure-Rust backend). Both build for
   `wasm32-unknown-unknown`, which `hpr-io` must, and `cargo deny` accepts both.
6. **`cargo xtask ork` is the corpus survey.** It reads every `.ork` under `refs/` and the example
   designs inside the pinned OpenRocket jar (a zip; reading the designs it ships is not reading its
   source), round-trips each one, writes the per-file detail to a gitignored `corpus-out/`, and
   prints counts only. Two files that are not well-formed XML are named in a committed list with
   the reason, and the survey fails if either ever reads or fails differently — an exclusion that
   polices itself rather than one that hides a regression.

7. **Two limits, both because the alternative is an abort rather than an error.** A read
   decompresses at most 256 MiB out of one archive (deflate expands about a thousandfold, so a
   1 MB `.ork` can ask for 2 GB of memory; the largest corpus design unpacks to 2,052,024 bytes), and
   an entry that would pass it is left out with a warning. And a run of text split by a comment, a
   processing instruction or a CDATA section is joined back as it is read: the writer drops the
   comment that made the split, so without joining, reading what was written would not give the
   same document. XML namespaces are the one thing the tree does not keep, and a document that
   declares any says so in a warning.

**Consequences.** M3.1a ships a reader that cannot yet produce a rocket, and the guide says so in
its second paragraph. The 78-file corpus gives every later increment a regression net it can run in
one command. `Document`'s `version` and `creator` fields are a reading of attributes that also stay
in the tree, so a document built by hand can contradict itself; the round-trip guarantee is stated
for documents that came from `parse`. The canonical writer means a `.ork` re-written by hpr will
not be byte-identical to the one it was read from, which M3.2 will have to live with — matching
OpenRocket's own layout was never achievable without reading its source.
