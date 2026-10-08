# ADR-184: Logged apogees of the private collection (2026-10-06)

- **Status:** accepted
- **Summary:** M2.3c1 flies the 83 tier-A flights of `hpr-sim-fixtures` whose design is a `.ork`, in hpr and in OpenRocket 24.12, each in its day's ERA5 weather with nothing deployed, and compares each apogee with the flight's logged one, turned into the height climbed in that day's air. A hand-read manifest under the gitignored `corpus/` holds what each flight's free-text row says; only aggregates are published, with the collection's credit and the Copernicus attribution.

**Context.** M2.3c1's *done when*: "each tier-A flight from a `.ork` is flown by hpr and by
OpenRocket in its day's weather, or refused with its cause in an open issue, the counts published
as aggregates with ADR-151's attribution; each logged apogee is converted to height above the pad
with the day's temperature profile, early deployments flagged or left out; each simulator's error
is in the report as anonymised statistics." The collection ([ADR-151](0151-hpr-sim-fixtures-joins-the-reference-library-as.md))
holds 89 tier-A flights, 83 of them with a `.ork`, each with its log, motor, date, site and the
ERA5 pressure-level file of its day. Four things stood in the way: its rows are free text (an
apogee's cell names its altimeters, their readings in feet or meters, and sometimes a GPS height
beside them); every ERA5 file is netCDF-4, which
`hpr_io::netcdf` refuses by design ([ADR-081](0081-era5-weather-read-from-netcdf-classic-in-hpr-io.md));
`cargo xtask ork-flights` and its OpenRocket oracle fly calm standard air only; and
`hpr_validate::real_flight::Barometer` turned a height into a reading, not back.

**Decision.**

1. **A manifest, read by hand.** Each flight's row, notes, evidence, log and design were read
   into one JSON entry under the gitignored `corpus/hpr-sim-fixtures/manifest.json`: the `.ork`
   as flown and its configuration, the launch time in UTC, the site, the rail, the ERA5 file, the
   logged apogee in meters with what read it (barometric, fused or GPS) and which altimeter, an
   early deployment, and why a flight's log is no fair test, if it isn't, in one of five classes
   (a motor anomaly, active control in the climb, a log that lost its apogee, a design that is not
   the flown rocket, another reason). The reading was split over seven parallel readers and
   checked by `cargo xtask fixture-flights`, which refuses a manifest that doesn't hold the 83
   flights once each, with numbers and a class from the list. It stays on this machine, as the
   collection's license requires; whether to keep a copy in the private collection is the
   maintainer's call.
2. **ERA5 by the guide's conversion.** `validation/oracles/netcdf/fixture_era5.py` converts each
   tier-A flight's pressure-level file with the conversion the weather guide gives, word for word,
   into `corpus-out/`, and indexes each source's and output's SHA-256. `hpr_io::netcdf` still
   reads only the classic formats.
3. **Each simulator in the day's weather as it can take it.** hpr flies the whole ERA5 profile of
   the site and hour (`Era5Profile`, its temperature, pressure and wind at every level).
   OpenRocket 24.12's atmosphere is the standard one through a launch temperature and pressure, so
   it gets the profile's at the pad; its multi-level wind (`WindModelType.MULTI_LEVEL`, heights
   above ground) gets hpr's wind every 50 m from the pad to 25 km, without turbulence. A probe in
   the oracle holds the multi-level wind to the average-wind model whose convention
   `conditions.py` measured: the simple example lands 48.43 m west in a 5 m/s wind from the east
   both ways. OpenRocket's geodesy is set to the WGS 84 ellipsoid hpr flies, as the designs'
   stored simulations differ. hpr is flown a second time in OpenRocket's air (`Ussa76::anchored` at the pad, the
   same wind), which says how much of the gap between the two is the atmosphere.
4. **The same flight in both.** Both fly the configuration hpr chooses: the manifest's, by id, or
   else the first holding a motor of the flown designation (matched on the class letter and
   thrust, without a total-impulse prefix, a delay or a note in brackets); an id the design lacks
   stops the run. Both fly OpenRocket's database curve
   for the digest the `.ork` records (ADR-067). Nothing deploys in either (every recovery device
   set to `NEVER` in OpenRocket), so each apogee is the climb's; hpr's is its center of mass's
   height above where it started, as OpenRocket's altitude is 0 at launch. A launch hour not known
   is flown at local noon, and a rail not known as 6 ft and upright; the report counts both.
5. **A logged reading turned into height.** A barometric altimeter reads the standard
   atmosphere's altitude of the pressure it measures, less the pad's
   ([ADR-082](0082-real-flights-read-from-refs-compared-over-the.md) §4). `Barometer::height_m`
   inverts that in the day's ERA5 air, bisecting to the resolution of `f64`; a test holds it to the
   closed form on a day 20 K warmer than the standard. A fused (barometer and accelerometer)
   apogee is read as barometric, as M2.3b read its fused logs; a GPS apogee is taken as it is.
6. **Early deployments flagged, both ways.** The errors are given over every compared flight and
   again without those whose log or notes show a deployment before apogee; a flight whose log is
   no fair test (§1) is left out and counted by class.
7. **Aggregates only.** `validation/reports/fixture-flights.{json,md}` hold counts, each
   simulator's mean, median, mean absolute and RMS error, the counts within 5% and 10%, and a
   histogram in 5% bins: no flight named or numbered, no apogee, no per-flight row. The page
   credits the sources collectively and carries the Copernicus attribution and the ERA5 citations.
   CI holds the report to itself; `scripts/gate.sh refs` flies it again with `--check`, which
   refuses an OpenRocket record unless its conditions hash equals that of the conditions rebuilt
   from the manifest and ERA5 files, and each script it records hashes as it did.
8. **Every flight not flown names an issue.** A cause of a flight either simulator doesn't fly is
   counted with the open issue that tracks it, and the run stops on a cause with none.

9. **The records page leaves the site's search.** This milestone's pages took mdBook's search
   index past its 10 MB warning (10,021,814 bytes), which fails the site check. Rather than give
   up the `####`-level search the physics pages rely on, `book.toml` leaves the generated records
   page, `decisions-and-roadmap.md` (195 KB of tables linking elsewhere), out of the index: 9,658,065
   bytes after. It is still reached from every page's links.

**Consequences.** Measured on 2026-10-06 (`validation/reports/fixture-flights.md`):

- Of the 83 flights, hpr flies 61 and OpenRocket 79; 61 are flown by both; 6 of those are left
  out (an airbrake or canard acted on the climb) and 55 compared. Every flight not flown is under
  a cause with an open issue: in hpr's `.ork` reading, a fin tab tens of microns past its root (6,
  #357), a freeform outline with a repeated point (3, #358), rail buttons spaced forward (1, #359),
  unread surface finishes (3, #360), override flags read as one (1, #361), a part in an off-axis
  inner tube (2, #181); motors whose digest OpenRocket's database lacks (3 in hpr, 1 in
  OpenRocket, #362); and in the collection, a design without the flown motor (2, #363) and a day
  with no ERA5 file (1, #364). The two flights under #363 come from one source, and each design
  holds the motor the other flew: the collection may have paired them crosswise.
- Over the 55: hpr +9.83% mean, 13.96% mean absolute; OpenRocket +9.00%, 13.08%; hpr in
  OpenRocket's air +9.99%, 14.07%; hpr against OpenRocket +0.69%, 1.72%, 53 of 55 within 5%.
  Without the 19 early deployments (each under 1.5 s and a few meters before the peak, by the
  manifest's notes): hpr +5.88%, OpenRocket +5.29% over 36. Both codes over-predict alike; the gap
  to the logs is not a difference between them.
- The day's air turns the barometric readings into heights 3.61% higher on average (0.9601 to
  1.0777 times). The makers that document their method all read the standard atmosphere without a
  temperature correction: PerfectFlite (SL100 and CF manuals, p. 2), Featherweight's Raven
  (Raven4 manual, p. 12), MissileWorks (RRC3 manual rev. 1.71, p. 23) and Silicdyne (Fluctus
  documentation v1.7b, p. 23). The RRC3's troposphere-only formula reads about 0.04% low below
  11 km and the Fluctus's simplified model about 0.13% high, both small beside the conversion. Blue Raven,
  Altus Metrum, Eggtimer and Jolly Logic document no method, and are read the same way.
- 24 of the 83 logged apogees differ from the collection's `log_max_altitude` by more than 0.5%:
  a course export whose altitude column stops before apogee (the raw log decoded with the
  collection's own tool instead), or a highest value inside an ejection charge's pressure spike
  (the reading before the charge taken instead). Each has its reason in the manifest. All the
  readers used the `.ork`'s rod angles as degrees, as OpenRocket writes them.

**Left open.** M2.3c2 adds each log's altitude trace. The six tier-A flights from other formats
wait for M3.4 to M3.6. The manifest's reading is a person's (here, a reader's) judgement of free
text, checked for form but not cross-checked flight by flight by a second reader.
