# AerospaceNU's flight logs as a reference

Can the public flight logs of AerospaceNU, Northeastern University's aerospace club, add held-out flights for
Release 0.3 ([ADR-209: the release bar](../decisions/0209-the-2026-10-08-best-on-the-market-measured.md))? This
builds on [more public flights](public-flights.md) and [the logger families](debrief-log-formats.md). Read on
2026-10-12: data, READMEs and licenses only; the club's code was never opened.

**In short.** As it stands, the repository adds no held-out flight: none of its 36 flights has a public design file.
- **Logs:** 36 flights on 23 dates, 2022-04-23 to 2026-05-16, with 46 log files from six logger families, two of
  them the club's own boards. 7 flights carry two or more independent loggers.
- **Motors:** 3 flights' attributes name a COTS motor and a 4th's log file names do, all Cesaroni 54 mm L-class
  reloads on ThrustCurve; 32 name none.
- **Date and site:** all 36 give both; for 2 the log's GPS or the folder name contradicts them.
- **Designs:** none. The club's 36 repositories hold 2 `.ork` and 7 RASAero `.CDX1` files, all from an unlicensed
  2019 project, for no rocket in the log set.
- **All four (log, design, COTS motor, date and site): 0 flights.** Three of the four: 4 flights.
- **HPR Sim reads** 2 of the 46 files with `hpr_flightdata::perfectflite`. The validation harness's column reader,
  `hpr_validate::real_flight`, could take the 5 EasyMini files and the 2 FCB files timed in seconds: 8 flights.
- **License:** GPL-3.0, code and data alike; a `refs/` cache with only statistics and digests committed fits it.

What it does offer is 7 flights with two independent loggers aboard. Their barometric peaks differ by 0.2% to
5.9%, which bounds how far one logger can be trusted as "the measured apogee". A board's filtered height is not
such a reading: on 2022-06-11 it overshoots the same board's barometers by 21%.

## The data repository

[AerospaceNU/rocket-flight-data](https://github.com/AerospaceNU/rocket-flight-data) was created on 2026-05-24 and
last pushed on 2026-06-24. It is read here at commit `e951b1f` (2026-06-24): 72 commits, one contributor.
- **Layout.** `flight-data/<date> <rocket>/` is one flight; each logger has a folder with its log and an
  `attributes.csv`. The flight's own gives date (the launch's, says the README), rocket, site, motor and peaks.
- **Counted from the git tree** (323 entries, not truncated): 36 flight folders (one rocket on one day), 46 logger
  folders, 46 log files, one metadata JSON and an 80 MB zip of the club's viewer app (not data, not fetched).
- **A second folder.** `data/` holds 20 files on 8 of the same flights (3 dates); by SHA-256, 16 duplicate files in
  `flight-data/`. The other 4: an EasyMini `.eeprom`, a 43-line third board on 2025-11-15, and two GPS text files.
- **The peaks are the viewer's** (the rest of the repository, not read). Each matched its log's height-column
  maximum when recomputed here; for the FCB that is `pos_z`, the board's filtered estimate, not a raw baro height.

### Six logger families

| logger | files (flights) | file | holds | rate |
|---|---|---|---|---|
| FCB, the club's flight computer | 20 (20) | CSV, 73–87 columns | 1–2 baros, IMU, GPS | 15–16 ms (one 106) |
| FCB ground station | 4 (4) | text of Python-style dicts | radio downlink: altitude, GPS | about 5 Hz |
| SillyGoose, the club's altimeter | 14 (11) | tab-separated text | pressure, IMU, filtered height | 100 Hz in flight |
| Altus Metrum EasyMini | 5 (5) | AltOS CSV | pressure, height, speed | 100 Hz up, 10 down (by phase) |
| PerfectFlite StratoLoggerCF | 2 (2) | `.pf2` (one saved as `.txt`) | baro altitude (ft), speed | 20 Hz |
| raw GPS | 1 (1) | CSV, Unix time | latitude, longitude, altitude | 1 Hz |

Rates are each file's median sample spacing. The club's NULI page reports "over 20 successful FCB-controlled
flights". Traps for a reader:
- **The FCB time column changes meaning.** In 6 files `timestamp_s` holds milliseconds (5 with `timestamp_ms` all
  zero, 1 with no `timestamp_ms`); in 2, `timestamp_s` is seconds; in 12, `timestamp_ms` is the clock.
- **SillyGoose files vary.** One older file is space-separated at 50 Hz; another is comma-separated ground-station
  telemetry, about 5 Hz, with no header row. One is tagged `doctored` (hand-edited, raw offload lost, says the
  README), and one flight's log is split in two by a reboot in flight.
- **The `.eeprom`** names `log_format` 14 (EasyMini v2.0), not among the formats Debrief's reader decodes (1, 10,
  15, 16, 19, 21, 22; [logger families](debrief-log-formats.md)).

### License and provenance

- **GPL-3.0.** Per GitHub's license API and the root `LICENSE` (first commit); no other license file, so data
  and code share it.
- **What it permits.** §2: anyone may "make, run and propagate covered works that you do not convey, without
  conditions": a private `refs/` cache is that; committing copies would convey (§4–5). Statistics and digests are
  facts (**Inference**, not legal advice), as for RocketPy's logs under
  [ADR-082](../decisions/0082-real-flights-read-from-refs-compared-over-the.md).
- **Personal data.** Folder names carry members' names and some logs a callsign; this note uses neutral labels.
- **Other repositories.** 10 of the club's 36 carry a license (6 GPL-3.0, 4 MIT); the two with flight data don't:
  `nuli-airbrakes-software` (another 2025-05-04 FCB copy, a raw FCB CSV) and `dpf-line-cutter`. Cited only.

## The flights

Sites, as `attributes.csv` names them, with the logs' GPS: **VT** is `CRMRC`, a field near St. Albans, Vermont
(source 5), at 44.82° N, 73.16° W; **FAR** is `FAR`, at 35.35° N, 117.81° W; **LDRS** read 34.50° N, 116.96° W,
at Lucerne Dry Lake (**Inference** from the fix; source 6 places an LDRS there); **HSV** is `Huntsville`.
"Peak" is each log's height-column maximum in meters (the FCB's filtered `pos_z`), logger by logger (in brackets,
the stated apogee); `;` separates two rockets flown that day. "Reads": `pf2` is `hpr_flightdata::perfectflite`,
`cols` is `hpr_validate::real_flight`'s column reader.

| date | rocket | site | motor | logs | peak, m | reads |
|---|---|---|---|---|---|---|
| 2022-04-23 | Super Guppy | VT | unknown | EasyMini, FCB | 354 / 342 | cols (EasyMini) |
| 2022-06-11 | Beanboozler | LDRS | L265 (file names) | EasyMini, FCB | 4,506 / 5,429 | cols |
| 2022-08-01 | Carby Two | LDRS? | unknown | FCB | 1,157 | none |
| 2022-09-17 | Beanboozler; TRD | VT | unknown | FCB | 1,288; 658 | none |
| 2022-10-15 | Beanboozler; TRD | VT | unknown | FCB | 2,099; 586 | cols; none |
| 2022-11-19 | Spruce Goose; TRD | VT | unknown | FCB | 491; 622 | none |
| 2023-01-21 | Spruce Goose | VT | unknown | FCB | 346 | none |
| 2023-02-18 | Beanboozler; Spruce Goose | VT | unknown | FCB | 718; 440 | none |
| 2023-03-18 | cert flight A | VT | unknown | ground station, as SillyGoose | 381 | none |
| 2023-04-12 | cert flight A | VT | unknown | SillyGoose (50 Hz) | 195 | none |
| 2023-04-15 | Beanstack (two-stage); OORR | VT | unknown | FCB | 650; 1,029 | none |
| 2023-05-27 | Refried Beans; Starship Enterprise | VT | unknown | ground station | 98; 2,994 | none |
| 2023-07-01 | Branstormz | FAR | unknown | ground station | 742 | none |
| 2023-07-15 | OORR; Starship | FAR | unknown | FCB (Starship 106 ms) | 1,790; 819 | none |
| 2023-07-15 | cert flight A | FAR | L265 | SillyGoose (replayed) | 3,875 | none |
| 2024-02-25 | Teapot | VT | unknown | FCB | 592 | none |
| 2024-05-19 | cert flight B | VT | unknown | ground station | 1,238 | none |
| 2024-11-03 | Wyatt | VT | unknown | FCB | 671 | none |
| 2024-11-17? | Subway | VT | unknown | StratoLoggerCF | 471 (466) | pf2 |
| 2025-02-15 | Wyatt | VT | unknown | FCB | 889 | none |
| 2025-05-04 | MBTA | HSV | L805 | FCB | 1,330 | none |
| 2025-11-15 | Crocket | VT | none given | EasyMini, SillyGoose | 1,920 / 1,916 | cols |
| 2025-11-15 | MBTA, 5 in | VT | L805 | StratoLoggerCF, 2 SillyGoose, GPS | 1,718 (1,709) / 1,694 / 1,680 | pf2 |
| 2025-11-15 | cert flight C | VT | unknown | SillyGoose | 285 | none |
| 2026-02-22 | BlueUp | VT | none given | SillyGoose | 756 | none |
| 2026-02-22 | Ethical Missile | VT | none given | EasyMini, SillyGoose | 762 / 747 | cols |
| 2026-02-22 | Haybales | VT | none given | SillyGoose (2 segments) | 924 | none |
| 2026-02-22 | RocketWorks | VT | none given | 2 SillyGoose (one doctored) | 846 / 840 | none |
| 2026-05-16 | L3 cert flight D | VT | unknown | EasyMini, SillyGoose | 2,430 / 2,573 | cols |

That is 29 VT, 4 FAR, 2 LDRS and 1 HSV. Only the two StratoLoggerCF files state an apogee (1,530 ft and 5,608 ft
AGL); their raw peaks sit 1.0% and 0.5% above it.
- **Loggers disagree.** On the 7 flights with two independent loggers, in table order, the second's peak against
  the first's is −3.4%, +20.5%, −0.2%, −0.9% and −1.7% (MBTA's two boards against the stated apogee), −1.9%,
  −0.8% (one file doctored) and +5.9%. The +20.5% is 2022-06-11's FCB `pos_z`: its barometers read 0.904 and
  0.509 (pad, peak), 4,486 m in the standard atmosphere if bar and 4,497 m if atm, within 0.5% of the EasyMini;
  its GPS rose 4,749 m, as a warm day gives. On 2022-04-23 its barometers give about +4% (`pos_z` −3.4%).
- **Peak speeds that can't be right.** The attributes list 766 m/s (Carby Two), 521 m/s (2023-07-15 Starship) and
  477 m/s (the replayed L265 flight): noise in derived columns, unchecked (**Inference**), and not used here.
- **Mach.** All 36 sit in the core band (Mach 0–2.5), most well subsonic. 2022-06-11's EasyMini reads 328.9 m/s
  at 1,401 m above the pad, Mach 0.97; a barometric speed reads low by the GPS-to-baro ratio (about 1.06 here), so
  Mach 0.95–1.02 (**Inference**). 2022-10-15 Beanboozler's FCB reads 312.9 m/s, about Mach 0.94.
- **Conflicts.** Carby Two's GPS never moves from 39.08° N, 75.88° W, 3,662 km from the 2022-06-11 LDRS fix, and
  its date is a Monday. Subway's folder says 2024-11-17, its attributes 2025-11-17.

## Motors

ThrustCurve's search API gives one match for each designation named, both Cesaroni 54 mm reloads listed as
`regular` and certified by the Canadian Association of Rocketry: **2645L265-P** (Mellow), 2,644.6 N·s over 9.91 s,
and **2833L805-P** (White), 2,833 N·s over 3.52 s.

The attributes name L805 on both MBTA flights and L265 on the 2023-07-15 cert flight; the 2022-06-11 Beanboozler's
L265 comes from its two log file names (**Inference**). None states a delay, a lot or a measured motor mass.

## Sites, dates and weather

- **The launch hour is the gap.** A date and a site fetch the day's ERA5 (`hpr_io::era5` reads it); the hour is
  not given. 12 flights record it: 7 FCB GPS timestamps (UTC), 4 ground-station `run_start` times (zone unstated;
  one matches the rocket's GPS clock) and the 2025-11-15 GPS log (19:56 UTC); a 13th has one in a file name.
- **What the hour is worth (Inference).** A barometric height scales with the air column's mean temperature, about
  0.35% per kelvin near 288 K, so a 5 K swing over the day moves a baro-to-geometric conversion by about 1.7%.

## The designs

There are none for these rockets:
- **The club's repositories.** All 36 trees (6,041 files, none truncated) were searched for `.ork`, `.rkt`, `.CDX1`
  and `.ric`. Only `19SU_Karman_Simulations` matched: 2 OpenRocket and 7 RASAero files from 2019, unlicensed,
  flown before this log set begins.
- **The club's site.** [aerospacenu.com](https://www.aerospacenu.com/nuli) links no design, report or file. Its
  intro course has students model their rockets in simulation software: designs likely exist, unpublished.
- **What else.** MBTA's 5 in diameter (`rocket_diamiter_in`, 2025-11-15) is the only airframe size given. Code
  was not searched: a rocket held only in code stays unread. The private collection lists this repository as a
  lead, with no flight held.

## What HPR Sim reads today, and what's missing

- **PerfectFlite.** `hpr analyze` takes a `.pf2` extension or a first line naming PerfectFlite, so both
  StratoLoggerCF files reach `hpr_flightdata::perfectflite`. A script applying its stated rules found 1,072 and
  2,328 numeric rows, times strictly increasing, heights in feet (**Inference**: the reader was not run).
- **AltOS CSV and seconds-timed CSV.** `hpr_validate::real_flight` reads RocketPy's TeleMetrum AltOS CSV by column
  (time in seconds, a factor only on height); by those rules it would take the 5 EasyMini files and the 2 FCB
  files timed in seconds, given designs (**Inference**: not run). `hpr analyze` doesn't read them.
- **Missing, by flights:** FCB CSV timed in milliseconds, 18; SillyGoose text, 11; ground-station telemetry, 4
  (a radio downlink with dropouts: one file's attributes count 8,150 values blanked); `.eeprom` format 14, 1.
  [M7.1, flight-log importers](../ROADMAP.md) lists AltOS CSV, `.eeprom` (TeleMetrum, TeleMega, EasyMega) and a
  generic CSV with column mapping, which would cover the FCB CSV; SillyGoose and ground-station text are on none.

## What this suggests

1. **Pin and digest.** Record `e951b1f` under `refs/` with the SHA-256 of its 46 logs, and GPL-3.0 as cited-only
   data in `THIRD-PARTY-NOTICES.md`. *Done when* a refs check reproduces the 46 digests. *Effect:* none yet.
2. **A logger-agreement reference for M7.2.** Once M7.1 reads AltOS CSV and SillyGoose text, read the 7 two-logger
   flights' barometric apogees. *Done when* each pair's difference is committed with the logger the readings trust
   and why. *Effect:* a measured bound, 0.2% to 5.9% here, on "the measured apogee".
3. **Ask the club for four designs** (a maintainer's call): MBTA (two L805 flights), the 2022-06-11 Beanboozler and
   the 2023-07-15 L265 flight, under a license that allows caching. *Done when* one `.ork` arrives with its as-flown
   mass. *Effect:* up to 4 more flights, 2 readable now; ADR-209 §4 holds out a flight by its hash's lowest
   third, so about 1.3 held-out on average, the rest calibration flights.
4. **Two StratoLoggerCF fixtures.** Add the two files' digests and readings, read from `refs/`, to `perfectflite`'s
   tests. *Done when* the reader's apogee is within 1% of each file's stated apogee.

## Open questions

- Which design flew on each flight? Did MBTA fly air brakes on 2025-11-15 (they could only lower its apogee)?
- Where and when did Carby Two fly? Is Subway's date 2024 or 2025? Is the replayed 2023-07-15 output measured?
- Did 2022-06-11 pass Mach 1? Which LDRS was it (source 6 names LDRS 41 at Lucerne, its host's blog LDRS 40)?

## Sources

All read on 2026-10-12.
1. AerospaceNU, [rocket-flight-data](https://github.com/AerospaceNU/rocket-flight-data) at `e951b1f`: README,
   `LICENSE` (GPL v3, §2, §4–5), every `attributes.csv` and the 46 logs. Cached under `refs/`, never committed.
2. GitHub REST API (<https://api.github.com/orgs/AerospaceNU/repos>): 36 repositories, licenses, trees, commits.
3. ThrustCurve.org search API (<https://www.thrustcurve.org/api/v1/search.json>), common names L265 and L805.
4. AerospaceNU site: [NULI](https://www.aerospacenu.com/nuli) and [intro](https://www.aerospacenu.com/intro) pages.
5. Northeastern ECE, [Intro to Rocketry Program Lifts Off](https://ece.northeastern.edu/?p=6232) (2023-03-31).
6. Wikipedia, [Rocketry Organization of California](https://en.wikipedia.org/wiki/Rocketry_Organization_of_California),
   and the club's [blog](https://rocstock.org/author/davew6dps/): an LDRS at Lucerne Dry Lake.
7. AerospaceNU's other repositories (<https://github.com/AerospaceNU>): file trees and READMEs only.
