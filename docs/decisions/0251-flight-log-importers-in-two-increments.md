# ADR-251: Flight log importers in two increments: the record every reader fills, a survey of the sample logs pinned by digest, and the PerfectFlite, AltOS and Raven readers first (2026-10-12)

- **Status:** accepted; splits M7.1 into M7.1a and M7.1b, the parent's *done when* unchanged and M7.1b's; follows [ADR-046](0046-debrief-folded-in-and-flight-log-analysis-that.md) (no simulator in `hpr-flightdata`) and [ADR-151](0151-hpr-sim-fixtures-joins-the-reference-library-as.md) (aggregates only)
- **Summary:** The sample logs under `refs/` number about 1,000 distinct files in some 20 layouts, too many for one increment. M7.1a builds the flight record every reader fills (every column a channel on its own clock, the logger's events), a survey that reads every sample and writes a committed report of counts and anonymized ids, and readers for the PerfectFlite text exports, AltOS CSV and the Raven's FIP CSV; M7.1b reads the rest. A private or unlicensed log is pinned by a digest of what was read from it, never by a snapshot of its values.

**Context.** M7.1's *done when*: "every sample file in refs imports, snapshot-tested, and the crate
builds with no dependency on `hpr-sim` (`cargo xtask wasm-check`). Formats without samples are
listed in the docs as needing samples." The private collection's catalog lists 1,445 log files, of
which 1,039 are distinct and outside its unpinned scratch folder; Debrief's 12 public fixtures and
AerospaceNU's 46 logs add more. They come in about 20 layouts, from PerfectFlite's `.pf2` (173
distinct files, all of which HPR Sim already reads) to binary AltOS and Missile Works memory
dumps. Debrief's TypeScript readers (MIT, the maintainer's own) cover most of them and port.

Two constraints shape how a reader is pinned. The collection is private (rule 4), so neither a
log nor a snapshot of what was read from it may be committed. Debrief's public fixtures are
trimmed from flights shared on public pages and team repositories whose licenses aren't stated,
and AerospaceNU's are GPL-3.0 data, so they too stay under `refs/`, read but uncommitted.

**Decision.**

1. **Two increments.** M7.1a: the record, the survey, and the readers for PerfectFlite's CSV and
   text exports (the `.pf2` reader exists), AltOS CSV and the Featherweight Raven's FIP CSV, the
   three families with both public fixtures and many files in the collection, one of which writes
   each channel on its own clock. M7.1b: every other layout, generic CSV with column mapping, unit
   detection and the list of formats needing samples; its *done when* is the parent's.
2. **The record.** `FlightLog` keeps its primary barometric track (`time_s`, `altitude_m` and the
   rest), which the readings use, and gains `channels`, every data column the file holds in file
   order, each converted to SI (angles in degrees, as GNSS writes them) and on its own clock, and
   `events`, what the logger says happened when (AltOS's flight states). A column HPR Sim can't
   name a quantity for is kept as written, marked as such. The primary track's columns appear
   among the channels too, so "every column kept" is a count a test can check.
3. **One entry point.** `hpr_flightdata::import::read` takes a file's bytes, recognizes the layout
   from its content, and reads it, or refuses it naming the layouts it knows. Recognition is by
   header, never by file name.
4. **The survey.** `cargo xtask logs` reads every log sample under `refs/`: the collection's
   catalog rows at its pinned commit, Debrief's fixtures and samples, and AerospaceNU's logs,
   distinct by content hash. The family of each file comes from the catalog's label, or from the
   survey's own header table for files the catalog doesn't list, not from HPR Sim's recognition,
   so a reader that fails to recognize its own family shows as a failure, not as another family.
   A file the catalog files under a family whose layout it no longer has moves to its layout's
   family only through a table in the survey, by id, with the reason printed in the report; the
   first is a spreadsheet's re-save of an AltOS export, its header edited to carry units, its
   `height` column deleted and three columns holding formulas, which is M7.1b's generic CSV.
   The committed report, `validation/reports/log-import.json`, holds per family the counts read
   and refused with each reason, and per file an anonymized id (the first 16 hex digits of the
   file's SHA-256) with the first 16 of the SHA-256 of the record read from it. The refs step runs
   `cargo xtask logs --check`, which fails on a stale report and on any M7.1a family file not read.
5. **Snapshots.** Invented sample files under `crates/hpr-flightdata/tests/` are read in CI and
   pinned by `insta` snapshots of the whole record. The record digests in the report pin every
   real sample the same way without publishing its values.
6. **`hpr analyze` stays with the `.pf2`.** The readings were set and tested on PerfectFlite logs;
   reading other loggers' flights waits for M7.2, which validates them.

**Consequences.** M7.1a closes with three new layouts read, about 330 distinct files, and the rest
counted in the report as M7.1b's. The record grows two fields, which every JSON consumer of a
`FlightLog` sees. A file's hash in the report reveals nothing of its contents, but someone holding
the file could confirm it is in the collection; that is accepted.
