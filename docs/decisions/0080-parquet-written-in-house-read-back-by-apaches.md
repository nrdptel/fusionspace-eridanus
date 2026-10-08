# ADR-080: Parquet written in-house, read back by Apache's library (2026-09-26)

- **Status:** accepted
- **Summary:** Parquet written in-house, read back by Apache's library

**Context.** M1.10c2 asks for a recording as a Parquet file, behind a cargo feature, which an
independent reader reads back the same. `ARCHITECTURE.md` suggested the `arrow`/`parquet` crates.
The writer lives in `hpr-sim`, a pure crate that builds for wasm32
([rule 5](../../CONTRIBUTING.md#5-offline-first)). The `parquet` crate without default features
still adds some ten runtime crates to it (`ahash`, `chrono`, `half`, `num-bigint`, `twox-hash` and
others), and a file read back by the library that wrote it doesn't show that someone else's reader
agrees.

**Decision.**

1. **Written by hand, from the specification.** `hpr_sim::export::parquet` builds the bytes from
   the Apache Parquet format (`parquet-format` at commit `bf09939`: `README.md` and
   `parquet.thrift`) and the Thrift compact protocol for the footer. It writes the simplest file a
   reader must accept: one `REQUIRED` `DOUBLE` column per recorded column, one row group (none for
   an empty recording), version 1 data pages of at most 1024 values (the 8 KiB page the
   specification recommends), `PLAIN` encoding, no compression, statistics, dictionary or index.
2. **Behind a `parquet` feature that adds no dependency.** `hpr-sim` has the feature and the `hpr`
   facade forwards it. It keeps a binary writer out of builds that don't ask for one, and gives
   compression codecs a place to go later without touching the default build.
3. **Apache's library is the independent reader.** The `parquet` crate (Apache arrow-rs, 60.0.0,
   no default features) is a test-only dependency, renamed `parquet-reader` so that it doesn't
   clash with the feature's name. Tests read every file back through it and compare every value
   bit for bit: a flight's recording, one of every channel (30 columns, over 2048 rows, so at least
   three pages per column, with the chunks' offsets and sizes checked to tile the file), an empty
   recording and extreme values. The fields that reader ignores (uncompressed sizes, the row
   group's offset) are pinned by a two-row file compared byte for byte with bytes worked out by
   hand from the specification, and the compact-protocol encoder by the protocol's own examples.
   Once, when this was written, pyarrow 25.0.1 and DuckDB 1.5.5 read the example's file and a
   30-column, 3005-row file equal to their CSV. `cargo xtask wasm-check` builds the feature for
   wasm32.
4. **The text exports' rules.** A value that isn't finite is refused with `SimError::Domain`, and a
   recorder with no columns with `SimError::Unsupported`. `Recorder::new` now refuses a channel
   listed twice (`SimError::Unsupported`): two columns of one name make pyarrow and Polars refuse
   the file, and made the CSV header ambiguous.

**Consequences.** A file is 8 bytes per value, plus about 20 bytes per page and a footer; without
statistics a query tool can't skip pages by value. The `export_flight` example needs the feature
(`required-features`), so its command gains `--features parquet`. The `parquet` crate stays out of
every build but the tests. `ARCHITECTURE.md` now names this writer instead of the
`arrow`/`parquet` crates.
