# ADR-127: M5.3c1, geodesics through GeographicLib, held to Karney's test set (2026-09-30)

- **Status:** accepted
- **Summary:** M5.3c split c1, c2; geodesics in `hpr_core::geodesic` through `geographiclib-rs`, flattening past 1/150 refused; Karney's 500,000-line test set within his 15 nm on five measures, either azimuth pair on its 21 mirror lines

**Context.** M5.3c asks for distance and bearing between two places on WGS 84, matched against a
published geodesic test set, and a site's height from a user's GeoTIFF, matched against another
reader's. The two share nothing: one is a pure model, the other a file reader. The method of
record is C. F. F. Karney, *Algorithms for geodesics*, J. Geodesy 87 (2013) 43–55
(arXiv:1109.4448v2): an auxiliary sphere with series in the flattening to `O(f⁶)` (§2) and
Newton's method for the inverse (§4, §5), whose round-off "in the direct and inverse methods are
less than 15 nanometers" (§7, page 10), with the truncation below round-off for `f` up to 1/150
(page 9). Karney publishes the set it was tested on, `GeodTest.dat` (doi:10.5281/zenodo.32156,
CC0): 500,000 WGS 84 geodesics in nine kinds (random, nearly antipodal, short, near a pole, and
so on), worked from exact `φ₁`, `α₁`, `s₁₂` with series to `O(f³⁰)` in high precision, each end
to 1e-18°. GeographicLib (MIT) is Karney's own code; georust's `geographiclib-rs` 0.2.7 (MIT,
2026-02 release, `libm` its one dependency without default features) ports it to Rust. Its
series grow inaccurate for large `f` (GeographicLib's table: 10 µm at 0.05, 0.3 m at 0.2; a
review measured the quarter meridian 0.32 m short at 0.2), and the inverse-then-direct round trip
can't see that, both halves sharing the series.

**Decision.**

1. **M5.3c splits.** c1: geodesics, done when distances and bearings match Karney's test set.
   c2: a site's height from a user's GeoTIFF, done when it matches another reader's. c1 first:
   no file format, and the oracle is a published file.
2. **`geographiclib-rs`, not a port.** It is Karney's algorithm line for line, maintained, pure
   Rust and wasm32-clean; a port would be the same code with our bugs. `hpr_core::geodesic` puts
   it behind `Ellipsoid::geodesic_inverse` and `geodesic_direct`, in radians and metres, heights
   ignored. Both return `Result`: an ellipsoid flatter than 1/150 (`GEODESIC_MAX_FLATTENING`,
   Karney's own limit) is refused, and so is a point built past `Geodetic`'s checks; the direct
   also refuses a non-finite azimuth or distance. A longitude outside `[−π, π]` is reduced modulo
   2π before its degrees could overflow. The direct returns latitude and longitude, not a
   `Geodetic`, so a height of zero can't pass as a site by accident; `GeodesicDirect::end` takes
   the height. Both result types are `#[non_exhaustive]`.
3. **What is measured, per kind, over the whole set, every line held to Karney's 15 nm.** The
   inverse's `|s₁₂|` error; where the direct problem from point 1 with the inverse's `α₁` and
   `s₁₂` lands, from point 2; the inverse's azimuth errors times `|m₁₂|` (the reduced length, so
   the sideways miss an azimuth error stands for at the other end), not measured on the
   "between vertices" kind, whose `|m₁₂|` is at most 1e-13 m; the direct's end-point miss in ECEF;
   and the direct's heading at the end, compared as a direction in ECEF (near a pole an azimuth
   turns through large angles as its point moves by nanometres; the heading does not) times `a`.
   Every per-line value is asserted finite.
4. **Mirror lines take either pair.** When `φ₂ = −φ₁` exactly and `α₁ ≠ α₂`, two geodesics of
   the same length join the points, the second with `α₁` and `α₂` swapped (GeographicLib's
   `GeodSolve` manual, *Multiple solutions*); where `α₁ = α₂` (the 50,000 between-vertices
   lines) the geodesic is unique. 21 lines of the set are mirror lines once read as `f64`, all
   nearly antipodal with `m₁₂` under a centimetre, so their azimuths are nearly undetermined; on
   4 the answer is nearer the swapped pair (2 of them were first misread as an ill-conditioned
   excess of 75 nm). The test takes the smaller error of the two pairs on those lines only; with
   it every line is within 15 nm.
5. **Where it runs.** Every 500th line (1,000) and the 21 mirror lines are committed and checked
   in CI; the whole file is pinned in `validation/refs.lock.toml` and checked where fetched. The committed
   table, `validation/reports/geodesics.md` (rewritten by `HPR_WRITE_GEODESICS=1`), is compared
   only on the build that wrote it, a debug build on macOS aarch64: its cells are a few ulps of
   ECEF coordinates and a release build moves three by up to 1.8 nm. Elsewhere only the bound is
   held.

**Consequences.** M5.3c1 is met: the largest errors over the 500,000 lines are 11.18 nm in
distance, 11.26 nm landing, 8.49 nm in the inverse's azimuths times `m₁₂`, 14.02 nm in the
direct's end point and 13.99 nm in its heading times `a`. Multiplying `a` by `1 + 1e-15` fails
CI's sample at 26.08 nm. Nothing in a flight uses geodesics yet, and `hpr` has no command for
them; the landing distance a flight prints is still a flat offset from the pad. The set is
WGS 84 only, so other ellipsoids up to 1/150 rest on Karney's method, not on a measurement here.
