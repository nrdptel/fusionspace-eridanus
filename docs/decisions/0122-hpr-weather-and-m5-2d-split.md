# ADR-122: `hpr weather`, and M5.2d split (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2d split d1 to d3; M5.2d1: `hpr weather`, one subcommand per source, fetched through `hpr_net`'s cache, `--offline` from the cache alone, `--from` a saved answer held to a fetch's checks; the profile written as `SoundingProfile`'s JSON

**Context.** M5.2d's *done when*: `hpr weather` writes a site's profile from each source, offline
from a fixture. Its title adds the user's ERA5 `.nc` files and whole GFS GRIB2 files, whose
complex packing (templates 5.2, 5.3) and JPEG 2000 (5.40) `hpr_io::grib2` refuses by number
(ADR-121). Each packing is a decoder of its own, to be checked value for value against ecCodes;
with the command, that is more than one pull request.

**Decision.**

1. **Split d1 to d3.** d1: the command, for every source hpr reads today (Open-Meteo, Wyoming,
   the NOMADS cuts, ERA5 `.nc`); its *done when* is the parent's for those. d2: complex packing,
   5.2 and 5.3, from a whole GFS file. d3: JPEG 2000, 5.40. The parent's clause holds for GFS's
   whole files only when d2 and d3 are met.
2. **One subcommand per source**: `open-meteo`, `wyoming`, `gfs`, `rap`, `era5`. Each takes what
   its request type takes, by the same names as `hpr sim`'s site (`--latitude`, `--longitude`).
   Times are UTC written `YYYY-MM-DDTHH[:MM[:SS]]Z`; one without its `Z` is refused, so a local
   time is never read as UTC. Open-Meteo reads at `--time` (`--historical` for the archive, `--model`
   passed through); Wyoming fetches the station's latest sounding before the launch `--time`
   (`WyomingRequest::latest_before`), `--bufr` for the BUFR version; GFS and RAP take `--cycle` and
   `--hour` as `NomadsRequest` does. No source is picked for the user, and nothing depends on the
   clock but the cache's freshness.
3. **Where the answer comes from.** Online, through `hpr_net::Client` over `Http` (rustls) and the
   platform cache folder (`Cache::platform_dir`, `HPR_CACHE_DIR`), so the library's freshness,
   fallback to a stale copy, and checked caching (ADR-119 §9) all apply. `--offline` answers from
   the cache alone and fails with nothing there. `--from FILE` reads a saved answer and touches
   neither the network nor the cache, since a file is not known to be the answer to any URL. It is held
   to the checks `fetch` makes: the parser, the sounding, and for GFS and RAP the model's grid
   and, when `--cycle` and `--hour` are given, the run and hour. The options that choose what to
   fetch and that a saved answer can't be checked against (Open-Meteo's site, `--historical` and
   `--model`; Wyoming's station, time and `--bufr`) are refused beside `--from` rather than
   ignored, so a wrong file can't pass as the place asked for. A time outside the answer's hours
   is named in UTC, as it was asked. `hpr-cli` gets the facade's `net`
   feature; no crate is added to the workspace.
4. **What is written.** `--output` writes `hpr_atmos::SoundingProfile`'s JSON, which the library
   reads back with `SoundingProfile::new`'s checks. Winds between levels are interpolated by speed and
   direction (`WindInterpolation::SpeedDirection`, the type's default) for every source; the
   field is in the file. The `--json` document is the CLI's own type (`weather.schema.json`,
   ADR-105 §5): the source and its credit, where the answer came from, the profile's position and
   time, its levels (wind direction in degrees, as `hpr sim`'s `--wind-from`), and the levels left
   out with the reason. The credit is always printed; ERA5's is the one its licence asks for
   (`docs/format/era5.md`).
5. **`hpr sim` doesn't read the profile yet** (#265). The command stops at the file.
6. **Checked** by `crates/hpr-cli/tests/weather.rs`: the six recordings in
   `crates/hpr-net/tests/fixtures/replay/` (two Open-Meteo, two Wyoming, GFS, RAP) through
   `--from`, and through `--offline` from a cache filled by the library's `fetch` over `Replay`,
   and two ERA5 files, each write the profile the library builds from the same bytes, equal to the
   bit, with the document's levels, position, run and levels left out equal to the library's and
   checked against the schema. Offline with no copy,
   all six are refused; a RAP cut read as GFS, and a cut of another hour, are refused. The live
   fetch was run by hand on 2026-09-30 for all four online sources over HTTPS, then again from the
   cache; CI never goes online. A saved answer is read whole with no size limit, since M5.2d2 reads
   whole GFS files, about 500 MB, through `--from`.

**Consequences.** M5.2d1 is met. A user can fetch, cache and save a launch day's weather from the
command line, but must fly it from a program until #265. NOAA's whole files stay refused until
d2 and d3.
