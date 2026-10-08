# ADR-125: Site data split, and WMM2025 in `hpr-core` (2026-09-30)

- **Status:** accepted
- **Summary:** M5.3 split a to c; WMM2025 in `hpr_core::magnetic`, finite at the poles, refused outside 2025 to 2030 and −1 to 850 km; NOAA's 112 test values, NCEI's file's `X` to its measured 7.18e-4 nT residue, shown to lie in the file's `X′`

**Context.** M5.3 asks for elevation (Open-Meteo, cached, a user's GeoTIFF optional), geodetic
helpers and magnetic declination from WMM2025, done when the WMM matches NOAA's test values and
elevation lookups are cached and work offline. That is three pieces of different kinds: a pure
model, a network source, and a file reader. NCEI publishes WMM2025 (released 2024-12-17) as a
coefficient file, `WMM2025.COF`, with two sets of test values: the report's Table 6
(`WMM2025_TEST_VALUES.txt`, 12 points, 0.1 nT and 0.01°, with grid variation) and 100 points to
1e-6 nT (`WMM2025_TestValues.txt`, in `WMM2025COF.zip`). The report also prints one point's
intermediate values to ten decimals (Table 3b) and the field over each pole (section 1.4). All
are works of the United States government.

**Decision.**

1. **M5.3 splits into a to c.** a: WMM2025; b: elevation from Open-Meteo, cached and offline after
   the first fetch; c: geodetic helpers (distance and bearing between two places) and a user's
   elevation file. a goes first: it needs no network and no new dependency.
2. **WMM2025 in `hpr_core::magnetic`.** The Earth's field is an Earth model beside gravity and
   geodesy, used by sims (a rail heading read off a compass) and by flight logs (magnetometers),
   so it sits in the pure core and builds for wasm32. The 90 coefficient rows are a `const` table;
   the coefficient file and both test-value files are committed under
   `crates/hpr-core/data/wmm2025/`, and a test holds the table to the file row by row.
3. **The report's equations, finite at the poles.** Equations 7 to 20, with each Schmidt function
   written as `P̆ₙᵐ = cₙₘ cosᵐφ′ qₙᵐ(sin φ′)`, `qₙᵐ = dᵐPₙ/dμᵐ` from DLMF 14.10.3's recurrence. The
   derivative and `P̆ₙᵐ / cos φ′` then need no division by `cos φ′`, so a pole needs no special
   case and the field there is the limit along the given meridian (report, section 1.4). Equation
   15 prints `ġ cos mλ − ḣ sin mλ` for `Ż′`; the potential gives `+`, which the test values
   confirm.
4. **Refused outside the model.** Times outside 2025.0 to 2030.0 and heights outside −1 km to
   850 km above the ellipsoid return `CoreError::Domain`. A flight log from before 2025 needs
   WMM2020, which is not bundled. `decimal_year` turns a date into the start of its day.
5. **What is held, and to what.** Table 6: every column of every row to half its last printed
   digit, grid variation included (`None` exactly where the table prints `NaN`). Table 3b: every
   printed value to half its last digit, plus eight units in the last place. The poles: to
   0.05 nT. NCEI's 100 points (rows counted from 0): `Y`, `D`, `I` and the rates of `Y`, `Z`, `D`
   and `I` to half the last digit, after an allowance of 64 units in the last place of the row's
   total field for both sides' rounding (7e-10 nT at 50,000 nT). `X` differs at 97 points, by up to
   7.18e-4 nT (row 35: 2026.5, 12 km, 33° N, 145° W), at most 2.11e-8 of the total field; `H`
   and `F` follow it, and `Z` moves by up to 2.2e-6 nT with it. The difference lies in the file's
   geocentric `X′`: one residue there, taken from `X`, brings the file's `Z` to 4.9e-7 nT of this
   code's, and this code's `X′` is the potential's derivative taken by differences to 1.3e-7 nT at
   all 100 points (a test), and the report's ten-decimal `X′` (Table 3b) to 5e-11 nT. pygeomag
   1.1.0 (MIT, a port of NOAA's `geomag` program), run once by hand on rows 3 and 35, gives this
   code's `X` to 7e-7 nT. The rate of `X` differs at 24 points, by up to 9.5e-7 nT a year, a
   second residue whose cause is not known; this code's `Ẋ′` passes the same derivative test, and
   the file's rates of `H` and `F` follow from its own `X` and rates to 1e-6. Those columns are
   held to their measured worst (7.2e-4 nT; 2.2e-6 nT; 1.5e-6 nT a year), 140 times below the
   0.1 nT the report allows single precision. The cause in NCEI's program is not known (an
   uncommitted check in review suggested the residue grows with `|X′|` and stays within about
   one single-precision step of `X′`).
6. **Compass zones by the report's thresholds.** `MagneticField::compass_zone` reports the
   blackout (horizontal intensity under 2,000 nT) and caution (under 6,000 nT) zones of section
   1.8, so a program can warn where a declination cannot be trusted.

**Consequences.** M5.3a is met, with NCEI's `X` column held to its measured residue rather than
its printing. Nothing in a flight uses the field yet: a heading given as magnetic is converted by
the caller with `MagneticField::true_from_magnetic_rad`. The model runs through 2030.0; WMM2030
will be a new table and a new `MagneticModel` constant. The height is above the
ellipsoid, as `Geodetic` holds it; a site's height above sea level differs by the geoid, which the
report puts at about 1 nT of field.
