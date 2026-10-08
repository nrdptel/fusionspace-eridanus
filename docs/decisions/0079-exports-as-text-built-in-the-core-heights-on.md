# ADR-079: Exports as text built in the core, heights on each format's own datum (2026-09-26)

- **Status:** accepted
- **Summary:** Exports as text built in the core, heights on each format's own datum

**Context.** M1.10c asks for CSV, JSON, KML and GeoJSON files, Parquet behind a feature, with
GeoJSON checked by schema and KML by parsing. The core crates do no I/O and build for wasm32
([rule 5](../../CONTRIBUTING.md#5-offline-first)), and a number the simulator prints is a claim, so
an export must not round it. Parquet needs the `arrow`/`parquet` stack, a large dependency whose
choice and feature wiring is a piece of work of its own.

**Decision.**

1. **Split.** M1.10c1 ships the four text formats; M1.10c2 adds Parquet behind a cargo feature.
2. **Text built in `hpr-sim`, no files.** `hpr_sim::export::{csv, json, track, geojson, kml}`
   return `String`s; a caller (the example, later the CLI) writes them. `hpr-sim` gains
   `serde_json` as a normal dependency, which already builds for wasm32.
3. **Exact numbers, or none.** Numbers are written in Rust's shortest round-trip form (`{:?}` in
   CSV and KML, `ryu` in JSON), so each reads back to the recorded `f64`; tests assert equality,
   not a tolerance. A value that isn't finite is refused with `SimError::Domain`, since CSV's
   `NaN` and JSON's `null` are read differently by different tools.
4. **Each format's own height datum.** GeoJSON positions are `[longitude, latitude, h]` with `h`
   above the WGS 84 ellipsoid (RFC 7946, section 4). KML's `absolute` altitude is above sea level
   (OGC 07-147r2), so KML writes `H = h − N` with the site's geoid undulation `N`, taken as
   constant over the flight. A `TrackPoint` carries both.
5. **Checked by the published schema and a strict parser.** The GeoJSON `FeatureCollection`
   schema from `geojson/schema` (MIT, commit 268ba0a) is bundled under
   `crates/hpr-sim/tests/data/`, and the `jsonschema` crate (MIT, tests only, no default
   features, so it fetches nothing) validates against it; a test shows it rejects a broken
   position. KML is parsed by `roxmltree` (already a dependency) and checked for the KML 2.2
   namespace, `altitudeMode` and every coordinate.
6. **Two observers on one flight.** `(A, B)` implements `Observer`, so the metrics and a recorder
   watch the same flight rather than flying it twice.

**Consequences.** A path crossing the antimeridian is not cut as RFC 7946 asks; it is documented.
Landings are ground-clamped points without a height. The recorder must keep the time and the CG
position for a map; without them `track` refuses. Nothing here changes a simulated number.
