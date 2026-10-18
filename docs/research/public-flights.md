# More public flights with their designs and logs

HPR Sim checks itself against seven public real flights, all from RocketPy's examples
([accuracy: real flights](../accuracy.md#real-flights)), and Release 0.3 needs held-out flights
([ADR-209: the release bar](../decisions/0209-the-2026-10-08-best-on-the-market-measured.md)).
This note asks where more flights pair a design with a measured apogee or log under usable terms:
per source, what it holds, its license, its formats and whether HPR Sim reads them. RASAero II's
comparison list is left out ([M2.3c7](../ROADMAP.md) covers it). All read on 2026-10-11.

**In short.** 14 more COTS flights are usable under the MIT license, all but one subsonic:
- **Three from RocketPy's own examples:** Astra, Andromeda and Erebus 11. They need only EuRoC
  2022's weather file converted from netCDF-4.
- **Eleven from team repositories:** eight are University of North Dakota NASA Student Launch
  flights, seven of them logged as StratoLogger `.pf2` files, which HPR Sim already reads.

13 of the 14 already sit in the project's private flight collection, copied from these sources and
scored there only in aggregate: going public lets each be reported and held out by name, but adds
no flight the project lacks. Larger sets carry no usable license, so they can only be cited:
- **Spaceport America Cup / IREC:** 264 official apogees with a public report, but no design file.
- **A college course archive (DOFPro):** 39 barometric flights on 7 OpenRocket designs, under a
  Creative Commons license it never names.

Two record fixes turned up: two of HPR Sim's seven logs are byte-identical to files in a GPL-3.0
repository, and Erebus 11 flew a COTS Cesaroni motor, not the team motor the records listed.

## Usable now: MIT, design and measured flight, COTS motor

### RocketPy's other measured flights

RocketPy's examples page lists 16 rockets with a simulated and a measured apogee
([RocketPy](https://github.com/RocketPy-Team/RocketPy) v1.13.0, `docs/examples/index.rst`); HPR Sim
flies 7. No example ships a design file: HPR Sim builds each design from the notebook's Python
inputs ([ADR-082: real flights read from refs](../decisions/0082-real-flights-read-from-refs-compared-over-the.md)).
The other 9:

| rocket (year) | measured apogee | log | motor | status |
|---|---|---|---|---|
| Astra (2022) | 3,250 m | 64 rows, 1 Hz | Cesaroni L1350, catalog curve | usable after weather conversion |
| Andromeda (2022) | 3,415 m / 3,443 m | 1,139 rows, 20 Hz | Cesaroni M2020, team curve | same; impulse caveat |
| Erebus 11 (2022) | 3,020 m | 746 rows, 20 Hz | Cesaroni Pro54 K-class | same |
| Valkyrie (2025) | 2,098 m | 12,578 rows | Cesaroni K650 | no weather file (live forecast) |
| Camões (2023) | 3,015 m | 14,346 rows | team curve, unnamed | motor unknown; stand-in air-brake law |
| Halcyon (2023) | 3,450 m | 65,882 rows | hybrid | outside COTS-solid scope |
| Hedy (2025) | 5,232 m | logs | liquid | outside scope |
| Defiance (2024) | 9,308 m | apogee only | hybrid | outside scope |
| Valetudo (2019) | 860 m | apogee only | team-built motor | outside scope |

Rows counted from the CSVs; Andromeda's apogees are the index's and the notebook's. Calisto never flew.

- **Weather.** Astra, Andromeda and Erebus 11 read `euroc_2022_all_windows.nc`, a netCDF-4 file
  (HDF5 signature) that HPR Sim's reader refuses; converted, the three take the set from 7 to 10.
- **Erebus 11 is COTS.** Its notebook names a Cesaroni Pro54 K-class motor. Its thrust file
  integrates (trapezoid, 113 points) to 2,418 N·s over 3.657 s, and the Cesaroni 2437K660 is
  2,437 N·s and 3.69 s on [ThrustCurve](https://www.thrustcurve.org/): 0.8% apart (**Inference**:
  a K660). The [accuracy](../accuracy.md) page and the ADR on real flights now list it as COTS.
- **Andromeda flies a team test curve** of 9,867 N·s in place of the catalog 8429M2020's 8,429 N·s,
  17% more; whether the gap is real or a slip in the test file is open.
- **Peak speed** (**Inference**, from 2 s slopes of the logs' height): Andromeda 283 m/s, Erebus
  303 m/s, Valkyrie 199 m/s, so about Mach 0.6 to 0.9. It's rough: Erebus's 1 s slope reaches
  341 m/s, and Andromeda's log has its own vertical speed peaking at 324 m/s (about Mach 0.95).
- **Terms.** RocketPy is MIT; 14 of its 17 notebooks record a team's permission to RocketPy, none
  terms for anyone else. ADR-082's practice fits: read from `refs/`, commit statistics and SHA-256.

### Team repositories on GitHub

21 repository searches and an `extension:ork` code search found 1,157 rocketry repositories; of the 1,116 file trees
read, 193 hold an `.ork` and 46 a `.rkt` or `.CDX1`. A scan for log and logger file names, then a hand check, left 16
with a design and a flight log: 6 MIT, 10 with no license. It undercounts: zipped `.ork` files aren't searchable, and
no search returned a seventh MIT repository (UTRGV's). A sampled GitLab search found none. Apogee is the altimeter's
stated value (primary / backup); where a StratoLogger states `PWRLOSS` (\*), HPR Sim's reading (`hpr analyze`), whose
raw peak is an ejection-charge spike up to 1.6% higher. Mach is **Inference** from top speed.

| flight | log | apogee | motor | Mach |
|---|---|---|---|---|
| UND Student Launch 2023 subscale, 2022-12-04 | `.pf2` | 854 ft | AeroTech G80 or H55W | ≈0.20 |
| UND SL 2023 full scale, 2023-03-05 | 2 × `.pf2` | 7,185 / 7,193 ft \* | AeroTech L1420R | ≈0.80 |
| UND SL 2023 full scale, 2023-04-02 | 2 × `.pf2` | 5,776 / 5,785 ft | L1420R | ≈0.68 |
| UND SL 2023 launch week | 2 × `.pf2` | 5,146 / 5,157 ft | L1420R | ≈0.56 |
| UND SL 2024 subscale | `.pf2` | 743 ft | AeroTech H268R or H219T | ≈0.18 |
| UND SL 2024 full scale | 2 × `.pf2` | 4,377 / 4,391 ft | L1420R | ≈0.50 |
| UND SL 2024 payload demonstration (**Inference**) | 2 × `.pf2` | 3,892 / 3,898 ft \* | L1420R | ≈0.42 |
| UND SL 2024 launch week | StratoLogger CSV | 4,053 ft (raw peak) | L1420R | ≈0.48 |
| Hyperion-II, 2025-08-21 | Blue Raven + TeleMega CSV | 2,852 / 2,854 m | Cesaroni M2505 | ≈0.80 |
| numericalrocketry model rocket | Altus-style CSV | 115.67 m | Estes C11-5 | ≈0.12 |
| UTRGV Lonestar launch, 2026-03-29 | Blue Raven CSV, GPS | 9,543 ft | AeroTech M2500T | ≈1.0 |

Sources, all MIT by GitHub's license API (the private catalog's "none stated" for UND is wrong):
[UND-ARC/RocketModels-2023](https://github.com/UND-ARC/RocketModels-2023),
[UND-ARC/Nasa-SLI-2024](https://github.com/UND-ARC/Nasa-SLI-2024),
[werocketry/Hyperion-II](https://github.com/werocketry/Hyperion-II),
[boulaetans/numericalrocketry](https://github.com/boulaetans/numericalrocketry),
[Lmx154/RL-RocketPy-Notebook](https://github.com/Lmx154/RL-RocketPy-Notebook) (`data/`).

Caveats, each unbounded until the team's own flight report is read:
- **Which design and mass flew.** UND keeps several `.ork` versions a year. The 2023-03-05 flight's
  7,185 ft is about 12.5% above its matching file's stored 6,386 ft, so ballast or the motor may
  differ. The repositories' flight-versus-simulation workbooks may say; they were not opened.
- **Hyperion-II's air brakes** may have opened. That can only lower apogee: it measured 2,854 m
  (backup) against its design file's stored 3,003 m (−5%).
- **numericalrocketry's** simulator was written from OpenRocket's source, so only its data files
  are of use here, never its code; its processed CSV marks its first 69 rows synthetic.
- **UTRGV's** figures are the Blue Raven's own summary: 1,143 ft/s (348 m/s, about Mach 1.0 at its
  66 °F pad), with the gyro saturated at 0.1 s and 38° tilt at burnout; not checked against the log.

Not counted: UND's 2024 Spaceport America Cup flight (MIT, AeroTech N3300R) peaked at 3,582 m
against a 9,100 m prediction, cause unstated; Notre Dame's `EamonTracey/apogee2` (MIT) has two
subscale flights on a team-built logger, with air brakes. The 10 license-less repositories, among
them Stanford's `rockets-irec` and Ohio State's `irec`, can only be cited.

## Large, cited only: no license or no design file

### Spaceport America Cup / IREC

ESRA (the Experimental Sounding Rocket Association) publishes yearly score sheets (target, actual,
mostly predicted apogee) and some years every team's report ([2024](https://www.soundingrocket.org/2024-sa-cup.html),
[past events](https://www.esrarocket.org/past-irec-events)); reports were matched to rows by team number.

| year | COTS-class flights with an apogee | with a public report |
|---|---|---|
| 2018 | 38 | 12 |
| 2019, 2022, 2023 | 41, 48, 66 | none found |
| 2024, 2025, 2026 | 80, 93, 81 | 78, 93, 81 |

That makes 264 flights with an official apogee and a report.
- **The apogee is measured.** The official log is a COTS barometric altimeter read by a judge ([IREC
  rules](https://www.esrarocket.org/s/IREC-Rules-and-Requirements-Document-2026-v10-f8wf.pdf), 2026 v1.0, §2.5, p. 13).
- **The report is not a design file.** Its Appendix A gives length, diameter, fins, masses, CG,
  CP, motor and predicted apogee (§2.6.2.9.1, pp. 18–19), but no nose shape, mass layout or
  finish: each design would be rebuilt from a table, at low fidelity.
- **The COTS class includes COTS hybrids**, so each flight's motor has to be read from its report.
- **Teams' own predictions run high.** For 2024's 80 COTS flights, (actual − predicted)/predicted
  has a median of −10.3% (quartiles −20% and −0.4%); 63 (79%) flew below. Air-brake teams aim low
  on purpose, so model error and control can't be told apart here.
- **Terms.** The year pages and score sheets carry no license; the rules' Creative Commons 4.0
  license (variant unnamed) doesn't cover the reports.

### DOFPro: a college course's flight archive

The Harvey Mudd College high-power course archive
([DOFPro flight data](https://dofpro.org/RCK/fltdata/FlightData.html)) links 7 OpenRocket designs and 89 data files,
2017 to 2023. 39 barometric flights on 6 of the designs (Raven FIP CSV, Entacore AIM CSV, a Teensy logger's raw
pressure) and 10 GPS-only ones pair with a design, all on COTS C to J motors. HPR Sim flies 3 designs as written and 3
more only past its fit checks, and refuses one whose mass and CG overrides disagree; it reads none of the log formats
yet (M7.1). The site calls its resources Creative Commons without naming a license; its course page says the original
course sites are not CC-licensed. The project's private collection already holds 59 flights from this course, so a
license would make flights public rather than add new ones.

## Searched, nothing usable

- **[American Rocketry Challenge](https://www.rocketrychallenge.org/result/2026/):** scores only, no
  designs. A score adds the altitude miss to four times the duration miss: no altitude to recover.
- **NAR R&D reports:** the [index](https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799)
  is public, but each of its 537 report links leads to the member login (all checked). Cited only.
- **Contest records name no rocket.** NAR's 133 altitude-event
  [records](https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=673904) (2026-06-10), 1/8A to G, up to
  2,071 m, and 17 F–O flights on Tripoli's
  [archived](https://web.archive.org/web/20170117005225/http://www.tripoli.org/Records) 2016–17 pages give altitude and,
  mostly, motor; never airframe or mass, nor launch weather.
- **[RocketReviews](https://www.rocketreviews.com/flight-log-by-date.html) flight logs** give a kit, motors and a
  typed-in altitude; the altimeter is named only in free text, no design file is linked to a log (the site's design-file
  libraries match no flight), no license is shown.
- **Altimeter makers** (PerfectFlite, Featherweight, Eggtimer, Altus Metrum, Missile Works, Jolly
  Logic, Entacore) publish manuals and software, no sample flight logs.
- **Zenodo and Figshare:** 6 Zenodo queries (25 hits each) and 5 Figshare queries (38 hits) found no design paired with
  a measured hobby flight. Zenodo's "[Rocket Flight Database](https://zenodo.org/records/20531977)" (CC BY 4.0) restates
  RASAero II's list. [arXiv 2512.22248](https://arxiv.org/abs/2512.22248) (CC BY 4.0) has 12 low-power apogees, no
  designs; OpenRocket is high on all 8 valid ones, by +12% to +31%.
- **[RocketSerializer](https://github.com/RocketPy-Team/RocketSerializer)'s** (MIT) `.ork` files of Bella Lui, NDRT,
  Valetudo and Prometheus are not the rockets as flown: Bella Lui's holds 8.42 kg against the notebook's 18.227 kg.

## Record fixes found on the way

- **Two flown logs are GPL-3.0 files.** RocketPy-Team's GPL-3.0
  [euroc23-team-data](https://github.com/RocketPy-Team/euroc23-team-data) holds files with the same SHA-256 as the
  Genesis and Lince logs `hpr_validate::real_flight` reads (`genesis/flight_data_faraday.csv`, `lince/main_data.csv`),
  and Camões's and Halcyon's. Only statistics and digests are committed, so nothing committed changes;
  `THIRD-PARTY-NOTICES.md` and the ADR on real flights now name GPL-3.0 for the two flown ones.
- **Erebus 11's motor** is COTS (above): it waits for netCDF-4, not among the team-motor flights.
- **RocketPy's paper** (Ceotto et al. 2021) is in *Journal of Aerospace Engineering* 34(6)
  ([doi](https://doi.org/10.1061/(ASCE)AS.1943-5525.0001331)); it covers Bella Lui, NDRT 2020 and Valetudo only.

## What this suggests

1. Convert EuRoC 2022's weather to classic netCDF and fly Astra, Andromeda and Erebus 11: 7 → 10
   public flights, the cheapest gain. Fly Andromeda on its team curve, with the 17% stated.
2. Pin the MIT team repositories under `refs/`, like RocketPy, and fly the seven `.pf2` UND flights once each flight's
   design, ballast and motor are confirmed from the team's reports. The CSV flights wait for the Blue Raven, TeleMega
   and StratoLogger CSV readers (M7.1). Correct the collection catalog's license field for the UND flights.
3. Two permissions are worth asking for, a maintainer's call: ESRA's license for its reports and
   score sheets, and DOFPro's named Creative Commons license.
4. Record GPL-3.0 for the Genesis and Lince logs, and COTS for Erebus 11 (done). Only UTRGV's flight may
   reach Mach 1; flights past it stay with M2.3c7.

## Open questions

- Which `.ork`, ballast and motor flew on each UND flight? The team's flight reports would say.
- Is Andromeda's 17% impulse gap over the catalog M2020 real? Did UTRGV's flight pass Mach 1?
- Did Hyperion-II's air brakes open on 2025-08-21? Its Blue Raven event columns may show it.
- How many 2024–2026 IREC COTS flights used active apogee control? That needs a read of each report.
- Can ERA5 stand in for Valkyrie's live forecast? Does Tripoli's current records search show airframe size and mass?
