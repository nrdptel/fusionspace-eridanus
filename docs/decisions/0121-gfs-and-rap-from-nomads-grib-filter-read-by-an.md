# ADR-121: GFS and RAP from NOMADS' grib filter, read by an in-house GRIB2 decoder (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2c: GFS and RAP from NOMADS' grib filter as a sounding, bilinear at the site, RAP's winds turned from its grid; an in-house GRIB2 decoder (3.0, 3.30 tangent, 4.0, 5.0) in `hpr-io` in place of the `grib` crate, checked value for value against ecCodes

**Context.** M5.2c's *done when*: a recorded GRIB2 cut decodes to the values an outside decoder
(ecCodes, run only) prints, and its levels become a profile. `ARCHITECTURE.md` named the `grib`
crate (0.18.6, MIT OR Apache-2.0, maintained) for GRIB2. It decodes to `f32`, so it cannot give
back ecCodes' doubles; its default features bind C libraries (`openjpeg-sys`, `libaec-sys`,
`proj`). NOMADS' grib filter (`filter_gfs_0p25.pl`, `filter_rap.pl`) cuts variables, levels and
a latitude/longitude box from a run's file. Both recorded cuts are simple packing (template 5.0)
on the model's own grid: GFS's 0.25° latitude/longitude (3.0), RAP's 13 km Lambert conformal grid
130 (3.30, tangent at 25° N, `LoV` 265°, winds along the grid).

**Decision.**

1. **An in-house decoder, `hpr_io::grib2`**, for what the filter serves: grids 3.0 and 3.30 (a
   northern tangent cone on a sphere; a secant cone or `LaD` off the tangent latitude is refused,
   since it would need the scale factor there and no NCEP grid in use has one), product 4.0,
   packing 5.0, and a bitmap given, absent or reused. Anything else is refused by template number.
   Values are `Y = (R + X · 2^E) / 10^D` in `f64` with exact powers (WMO-No. 306 Vol. I.2,
   Regulation 92.9.4); signed integers are sign and magnitude (92.1.5). `hpr-io` holds it because
   it is pure and M5.2d reads users' files offline; `hpr-net` now depends on `hpr-io`. It replaces
   the `grib` crate in `ARCHITECTURE.md`; no third-party crate is added.
2. **Bounded against hostile files.** `parse` keeps each field's bitmap and packed values
   borrowed, so it costs memory per field, not per grid point; a bitmap carries its running count
   of set bits every 64 bytes, shared by the fields that reuse it, so neither checking a field nor
   reading a value recounts it (the code review found a file of reused bitmaps took 31 s); a grid is refused above 2²⁴ points
   (a 0-bit field has no data to bound it); every section's length, the bitmap's length and marked
   count, and the packed bits are checked against the grid and the packing before a value is read;
   scale factors that make an infinite value are refused; so are latitudes off the Earth and a
   given radius outside 6,000 to 7,000 km. `hpr_net::nomads` keys fields in a hash map, so neither
   the duplicate check nor the lookups are quadratic, refuses a cut of more than 1,000 fields (a real
   one has 147 or 192); `fetch` refuses a cut whose grid steps and projection aren't the model's (GFS's 0.25°
   steps; RAP's 13,545 m cells, tangent at 25° N about 265° E; where the grid starts is not checked), which a self-consistent but wrong grid
   would otherwise pass: the bilinear weights and the points' positions come from the same grid.
3. **Lambert grids** use Snyder's spherical Lambert conformal conic (USGS PP 1395, 1987,
   eqs. 14-1, 14-2, 14-4, 15-1 and 15-2; inverse 14-9 to 14-11 and 15-5) with `n = sin φ₁`, eq.
   15-3's one-parallel case. **Grid-relative winds** are
   turned at the site by `θ = n (λ − λ₀)`, the bearing of the grid's `+y` axis: `u_E = u cos θ +
   v sin θ`, `v_N = −u sin θ + v cos θ`. The sign was first written backwards in the draft; the test
   measures the grid's `+x` bearing from ecCodes' own positions of two grid points and matches
   `90° + θ` to 2.2e-6°. Components are interpolated first and turned once, at the site (`θ`
   changes about 0.06° across a 13 km cell).
4. **`hpr_net::nomads`**, mirroring `open_meteo`: a `NomadsRequest` (model, cycle, forecast hour,
   site, optional endpoint) builds the filter URL for `HGT`, `PRES`, `RH`, `TMP`, `UGRD`, `VGRD` at
   the ground, 2 m, 10 m and each pressure level (GFS 1000 to 10 hPa, 28 levels; RAP all 37, 1000 to
   100 hPa) in a box 0.3° each way, written to 0.01°; `NomadsProfile::parse` decodes it, refuses a
   cut whose fields differ in grid, run or forecast time or give a variable twice, and interpolates
   bilinearly in grid indices between the four points around the site; `fetch` goes through
   `Client::fetch_checked`, whose check also refuses a cut of another run or hour, so it is never
   cached, as is one whose grid is not the model's (point 2).
   GFS runs every 6 hours, hourly to hour 120 and every third hour to 384; RAP every hour, to hour 21,
   and to 51 from its 03, 09, 15 and 21 UTC runs; other cycles and hours are refused. A run's file doesn't change once written, so a copy stays fresh 30 days.
5. **The ground and the levels** follow ADR-119: the ground at the model's terrain height with the
   surface pressure, 2 m temperature and humidity and 10 m wind; a level is dropped when its pressure
   is not below the ground's or its height not above it, or when a variable is absent or has no value
   at a grid point with weight. Heights are GRIB2's geopotential metres (code table 4.2), converted
   with WMO-No. 8 eq. 12.16 at the site's latitude; the terrain height, also in gpm, is converted the
   same way (1.9 m at 1,400 m and 33° N; if a model's terrain is a geometric height, the ground
   sits that far high, ADR-081's caveat). Checked as in ADR-119: from 500 hPa up, the recorded layers match
   the hypsometric thickness to −0.002% (GFS) and −0.08% (RAP) on average; read as geometric heights
   they would be 0.63% and 0.51% too thin. Humidity over 100% is kept and clamped in `sounding()`
   (ADR-004).
   Whether NCEP reports humidity over ice at cold levels is not settled; the density effect is under
   0.1% (ADR-119 §7).
6. **Checked** by `tests/nomads.rs` against ecCodes 2.49.0 (Apache-2.0, run only, not ported), whose
   reading `validation/oracles/grib2/eccodes_dump.py` writes to `tests/fixtures/nomads-eccodes.json`:
   every field's identity, all 6,123 values within 2.2e-16 relative (one rounding: ecCodes multiplies
   by an inexact `10^−D`) and every grid point within 5.7e-14° on macOS (the test's bound is
   1e-12°: RAP's points go through `tan`, `powf` and `atan`, whose last digits vary between maths
   libraries). The profile gives back ecCodes' values
   interpolated to the site at every level kept: 22 GFS levels and 31 RAP levels, 6 of each
   underground at the 1,400 m site. The ground test is made at the site only, so a kept level can
   take weight from a grid point where it is underground (RAP's 850 hPa here, 13%, about 0.02 K).
   ecCodes is pinned in the oracle environment (`eccodes` 2.48.0 with `eccodeslib` 2.49.0.30).
7. **Fixtures**: the two cuts recorded unchanged on 2026-09-30 at Spaceport America, GFS's 00 UTC run
   at hour 18 and RAP's 12 UTC run at hour 6, both for 18 UTC. NCEP's forecasts are U.S. government
   works (17 U.S.C. § 105).

**Consequences.** M5.2c is met. NCEP's whole files use complex packing (5.2, 5.3) or JPEG 2000
(5.40), which the decoder refuses; M5.2d, which reads users' GFS files, adds them or reconsiders the
`grib` crate's pure-Rust features. There is no interpolation in time: the user picks the run and the
hour. How good either forecast is at a launch is unmeasured.
