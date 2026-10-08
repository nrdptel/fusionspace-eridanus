# ADR-081: ERA5 weather read from netCDF classic in `hpr-io`; M2.3 split a to c (2026-09-26)

- **Status:** accepted
- **Summary:** ERA5 weather read from netCDF classic in `hpr-io`; M2.3 split a to c

**Context.** M2.3 compares hpr with real flights, flown in the weather of their day. RocketPy's
logged flights come with ERA5 reanalysis files: netCDF, some in the classic formats (`CDF\x01`,
`CDF\x02`), some netCDF-4, which is HDF5 underneath. The roadmap asks for a netCDF reader or a
documented conversion. `ARCHITECTURE.md` lists `hdf5` among crates to avoid (abandoned; C library)
and has no weather-file crate. The milestone is larger than one pull request: a reader, then the
flights, then the corpus flights with logs.

**Decision.**

1. **Split M2.3 into a to c.** M2.3a reads ERA5 files; M2.3b flies RocketPy's logged flights in
   their weather and meets the parent's bullets; M2.3c adds the private corpus designs that have
   logs, published as statistics.
2. **netCDF classic, by hand, from Unidata's specification**, in `hpr_io::netcdf`: both classic
   formats, every type, record variables (with the lone-record padding case), and the attribute
   conventions for packed and missing data. It depends on nothing new and builds for wasm32.
   netCDF-4 and CDF-5 are refused with a conversion: xarray's `to_netcdf(...,
   format="NETCDF3_64BIT")` after dropping `number` and `expver`, the Data Store's 64-bit integer
   and string variables that the classic formats can't hold (`nccopy` can't drop them for you).
   Reading HDF5 would mean a large format or a C library, for files a three-line conversion turns
   into ones this reader reads. A header whose variables claim more bytes than the file holds is
   refused before anything is allocated. Each dimension's name is allocated once and shared by
   every axis that names it, and duplicate names are found through ordered sets, so a hostile
   file costs memory a small multiple of its size and time `n log n` in its names (fuzzed by
   proptest).
3. **The Users Guide's conventions over netCDF4-python's.** With no valid bounds, the fill value
   bounds the valid range on its own side (one step away for integers, two units in the last
   place for floats), and a byte with no explicit fill has every value valid (netCDF Users Guide,
   "Attribute Conventions"). netCDF4-python 1.7.4 masks only values equal to a fill, the byte
   default included. The tests compare against that library everywhere else and pin each cell
   where the two differ, in both classic formats. It matters: RocketPy's multi-year netCDF-4 ERA5
   files for EuroC (2001–2021) and Spaceport America (2002–2021) hold −32768 under a −32767 fill
   (102 and 298 geopotentials, 4 and 2 temperatures), which hpr reads as missing and
   netCDF4-python as, for example, 198.66 K among neighbours near 301 K.
4. **ERA5 in `hpr_io::era5`, as a `SoundingProfile`.** `hpr-io` gains `hpr-atmos` (both pure).
   Values are bilinear in latitude and longitude, as RocketPy 1.13 takes them (its MIT
   `bilinear_interpolation`), and linear in time between the two hours around the launch, where
   RocketPy takes the nearest hour. Geopotential height `Z = z/g₀` becomes geometric height by
   WMO-No. 8 at the site's latitude, the relation `SoundingProfile` inverts, so each level's
   geopotential round-trips. ECMWF's Knowledge Base suggests `R·Z/(R − Z)` instead, "neglecting
   horizontal variations" of gravity, and RocketPy does that. Both are approximations. Taking
   the model's surface geopotential as `g₀ h_s` (an assumption; ECMWF doesn't say), WMO's reading
   is off by about `h_s(g₀/γ_s − 1) + h_s²/R` at every height and ECMWF's by
   `h_s²/R − (h − h_s)(g₀/γ_s − 1)`, growing with height above the model's ground. Neither is
   always the smaller: for a ground 1400 m up at 33° N, WMO's is 1.88 m and ECMWF's is smaller up
   to 1.95 km above the ground, −3.07 m at 3 km (with RocketPy's Earth radius). hpr keeps WMO's
   because it is the rule `SoundingProfile` uses for every sounding, so a level's geopotential
   round-trips; a test pins these numbers. The readings differ by `g₀/γ_s(φ) − 1` of the height:
   −0.0158% at 47.21° N and +0.0343% at 41.78° N, pinned by a test. Humidity is not read yet (dry
   air). Beyond the levels the profile is `SoundingProfile`'s: hydrostatic between levels and the
   offset standard atmosphere above them, where RocketPy holds the end level's values.
5. **Fixtures.** `validation/oracles/netcdf/write_cases.py` writes the reader's test files with
   the Unidata C library (through netCDF4-python) and records its reading. `era5.py` cuts small
   extracts of RocketPy's Bella Lui and NDRT 2020 files (the second one from the current Data
   Store, converted by the guide's recipe) and records RocketPy's reading of the full files. The
   extracts are committed under ERA5's CC BY 4.0 licence with Copernicus's attribution and
   disclaimer (`THIRD-PARTY-NOTICES.md`), and every fixture records its generator, tool versions,
   date, command and input hashes. netCDF4 and xarray join the oracle environment.

**Consequences.** `Era5Profile::read` agrees with RocketPy's levels to 1e-12 on the hour at both
sites, and two downloads of the same analysis four years apart agree to 0.23 m²/s², 0.35 mK and
0.11 mm/s, consistent with each file's rounding (`hpr_io::era5` tests). A user with a current
Data Store file runs three lines of Python first; a native
netCDF-4 reader stays open for later. Humidity and single-level (surface) files are not read yet.
