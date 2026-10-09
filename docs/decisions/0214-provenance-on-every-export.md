# ADR-214: Provenance on the exports: the catalog's date, the trust note, units in brackets, ASCII JSON (2026-10-09)

- **Status:** accepted; carries out the product system's `data.md` and `principles.md` §2 and §7 for #380 (M0.9c5), and amends [ADR-174][adr-174] §5 (the CSV's sidecar) and the CSV header [ADR-210][adr-210] §4 kept
- **Summary:** `hpr sim --export`'s JSON recording and the `.meta.json` sidecar beside a CSV, and `hpr mc --export`'s sidecar, carry the bundled motor catalog's as-of date (`catalog_as_of`) and the *How far to trust it* note as `--json` has it (`kind`, `trust`); the JSON recording also names the design and configuration. `hpr sim` and `hpr mc` say "from the bundled catalog, as of 2026-09-17" beside a motor taken from it, and `--json` gives the date as the source's `as_of`. The run's own date stays out, so a run writes the same bytes each time. A CSV header gives each column in words with its unit in brackets, `time [s]`, by one rule from the name the other formats keep (`time_s`). Every JSON document `hpr` writes, and the library's JSON and GeoJSON, is ASCII: other characters, the designation's middle dots among them, are `\u` escapes. The plot's last visible line is the program, its version and its designation. GeoJSON, KML and Parquet carry neither the date nor the note yet (#403).

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-174]: 0174-the-command-line-s-colors-hints-and-title-block.md
[adr-208]: 0208-the-design-audit.md
[adr-210]: 0210-us-units-in-brackets.md
[adr-211]: 0211-how-far-to-trust-a-result.md
[adr-213]: 0213-units-and-trust-notes-on-every-readout.md

**Context.** The design audit ([ADR-208][adr-208]) found four gaps in what the exports and the
plot say about where they come from (#380). The product system ([ADR-164][adr-164]) asks each
export to carry its source, tool, version and date, each dataset its "as of", a CSV header with
its units in brackets (`time [s]`), JSON in ASCII, and a printed drawing's designation and
version where a reader sees them. Before this record the exports named the tool and version,
but no date; the CSV header was the column names (`time_s`); the designation's U+00B7 middle dot
went into every JSON document; and the plot's version was only in its invisible `<metadata>`.
[ADR-213][adr-213] also left the exports without the trust note every command's text now ends
with, so a saved flight lost it.

**Decision.**

1. **The date is the catalog's.** A run's date would make the same run write different bytes,
   which [ADR-174][adr-174] §5 ruled out. The bundled catalog's as-of date (the day its curve
   files were downloaded, `Snapshot::captured`) is fixed per build, so it keeps runs
   byte-identical and is the date that bears on the numbers: a motor's curve can change on
   ThrustCurve.org after it. The sidecars and the JSON recording carry it as `catalog_as_of`
   whatever motor flew; the motor's own `source` in `--json` says whether it came from the
   catalog, and then carries the date as `as_of`. The text says "from the bundled catalog, as of
   2026-09-17".
2. **The trust note goes with the file.** The JSON recording and both sidecars carry `kind` and
   `trust`, the same strings `--json` has, so the note can't drift from the text. The JSON
   recording puts them, with the design, configuration and date, between `tool` and `columns`
   (`hpr_sim::export::json_with`, which refuses a field that would write a key twice).
3. **The CSV header follows `data.md`** (chosen over keeping the names). The CSV is the format a
   spreadsheet reads, and a header like `velocity east [m/s]` reads there without a key. 0.1
   isn't published, so no released file changes. One rule maps each name to its header
   (`hpr_sim::export::csv_header`): the name's last one or two words, read from a table of twelve
   units, go in brackets in ASCII (`m_s` is `m/s`, `m_s2` is `m/s^2`, `pa` is `Pa`); the rest are
   words. A name with no unit stays its words (`mach`, `drag scale`). JSON, Parquet and Python
   keep the names, which `data.md` allows for JSON ("units in the key"); `hpr mc`'s CSV and
   Python's `MonteCarlo.to_csv` use the same rule. Hand-written goldens of every recorder column
   and every Monte Carlo column pin the rule, so a unit read wrongly fails.
4. **JSON is ASCII by escaping.** The designation keeps its middle dots (ADR-198 names it so);
   JSON writes them, and any other character outside ASCII such as a design's file name, as `\u`
   escapes, a character past U+FFFF as a surrogate pair (RFC 8259 §7). Every reader reads back the
   same strings. `hpr_sim::export::ascii` does it on the finished text, which is safe because
   outside a string JSON text is ASCII already.
5. **The plot's last line is its title block's.** Under the trust note, after a blank line, the
   figure prints `FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001`, so a printed plot can be
   traced to the build that drew it. The metadata keeps it too.

**Held by.** `crates/hpr-cli/tests/provenance.rs`:
`every_export_carries_the_catalogs_date_and_how_far_to_trust_it`,
`the_text_dates_a_motor_from_the_bundled_catalog`, `a_csv_header_gives_each_unit_in_brackets`
and `every_json_document_is_ascii`, each failing on the build before this record;
`crates/hpr-sim/src/export.rs`'s `a_csv_header_gives_each_columns_unit_in_brackets`,
`json_text_is_ascii_and_reads_back_the_same` and `json_with_adds_the_callers_fields_after_the_tool`;
`crates/hpr-analysis/src/table.rs`'s `the_csv_reads_back_to_the_runs_bits`; and
`crates/hpr-cli/src/plot/tests.rs`'s
`the_figure_names_its_tool_and_version_where_a_reader_sees_them`.

**Alternatives considered.**

- **Keep `time_s` in the CSV, by an ADR declining the rule.** Rejected: it would keep one name
  per quantity across every format, but the CSV is the spreadsheet's format, the rule is explicit,
  and the names stay where code reads them (JSON, Parquet, Python).
- **The run's date in the sidecar.** Rejected by [ADR-174][adr-174] §5's byte-identical runs; the
  catalog's date is the one that bears on the numbers.
- **A `\u`-escaping `serde_json` formatter.** Not needed: escaping the finished text gives the
  same bytes with one short function the library and the command line share.
- **The note and date in GeoJSON, KML and Parquet now.** Deferred to #403, with the `.ork`
  motor whose curve the catalog supplies, which still prints as the design's: the issue named the
  JSON recording and the sidecars.

**Consequences.** The design audit's rows *principles.md · 2* and *data.md · Files and exports*
are met; *principles.md · 7* waits only on the README (#387); *data.md · Copying and typing
numbers* waits on #381's typed numbers; *principles.md · 3* waits on #386, #401 and #403. A
script that read `hpr sim`'s CSV by the old header names needs the new header, or the JSON or
Parquet recording, which keep them.
